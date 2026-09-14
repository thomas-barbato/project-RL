use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::content::{
    ContentId, RegionLandmarkProfile, RegionSiteEntranceProfile, RegionSiteProfile,
    RegionSiteTerminalProfile, RegionTerrain,
};
use crate::game::GameRng;
use crate::world::{DoorState, GridPos, Map, Terrain};

const LANDMARK_SEED_SALT: u64 = 0x4c41_4e44_4d41_524b;
const SITE_SEED_SALT: u64 = 0x5349_5445_5f4c_4159;
const SITE_TERMINAL_SEED_SALT: u64 = 0x5349_5445_5f54_4552;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionalLandmarkKind {
    SupplyCache,
    ThreatCamp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GeneratedRegionalLandmark {
    pub kind: RegionalLandmarkKind,
    pub position: GridPos,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GeneratedRegionalSite {
    pub camp: GridPos,
    pub cache: GridPos,
    pub entrance: GridPos,
    pub console: Option<GridPos>,
    pub entrance_kind: RegionSiteEntranceKind,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedRegionalSiteTerminal {
    pub site_index: usize,
    pub position: GridPos,
    pub record: ContentId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedRegionalSiteLayout {
    landmarks: Vec<GeneratedRegionalLandmark>,
    terrain: BTreeMap<GridPos, RegionTerrain>,
    interactions: BTreeMap<GridPos, Terrain>,
    sites: Vec<GeneratedRegionalSite>,
}

pub type GeneratedRegionalSiteParts = (
    Vec<GeneratedRegionalLandmark>,
    BTreeMap<GridPos, RegionTerrain>,
    BTreeMap<GridPos, Terrain>,
    Vec<GeneratedRegionalSite>,
);

impl GeneratedRegionalSiteLayout {
    pub fn landmarks(&self) -> &[GeneratedRegionalLandmark] {
        &self.landmarks
    }

    pub fn terrain(&self) -> &BTreeMap<GridPos, RegionTerrain> {
        &self.terrain
    }

    pub fn interactions(&self) -> &BTreeMap<GridPos, Terrain> {
        &self.interactions
    }

    pub fn sites(&self) -> &[GeneratedRegionalSite] {
        &self.sites
    }

    pub fn into_parts(self) -> GeneratedRegionalSiteParts {
        (self.landmarks, self.terrain, self.interactions, self.sites)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionSiteEntranceKind {
    Open,
    ClosedDoor,
    LockedConsole,
}

pub fn generate_regional_landmarks(
    map: &Map,
    passages: &[GridPos],
    reserved: &BTreeSet<GridPos>,
    profile: RegionLandmarkProfile,
    region_seed: u64,
) -> Result<Vec<GeneratedRegionalLandmark>, RegionalLandmarkError> {
    if profile.is_empty() {
        return Ok(Vec::new());
    }
    let mut rng = GameRng::from_seed(region_seed ^ LANDMARK_SEED_SALT);
    let cache_count = inclusive(&mut rng, profile.cache_range());
    let camp_count = inclusive(&mut rng, profile.threat_camp_range());
    let requested = usize::from(cache_count.saturating_add(camp_count));
    let mut candidates: Vec<_> = (1..map.height() as i32 - 1)
        .flat_map(|y| (1..map.width() as i32 - 1).map(move |x| GridPos::new(x, y)))
        .filter(|position| {
            map.is_walkable(*position)
                && !map.is_protected(*position)
                && !passages.contains(position)
                && !reserved.contains(position)
                && passages.iter().all(|passage| {
                    manhattan(*position, *passage) >= u32::from(profile.minimum_passage_distance())
                })
        })
        .collect();
    if candidates.len() < requested {
        return Err(RegionalLandmarkError::InsufficientSpace {
            requested,
            available: candidates.len(),
        });
    }

    let mut landmarks = Vec::with_capacity(requested);
    for kind in
        std::iter::repeat_n(RegionalLandmarkKind::SupplyCache, usize::from(cache_count)).chain(
            std::iter::repeat_n(RegionalLandmarkKind::ThreatCamp, usize::from(camp_count)),
        )
    {
        let index = below(&mut rng, candidates.len() as u64) as usize;
        landmarks.push(GeneratedRegionalLandmark {
            kind,
            position: candidates.swap_remove(index),
        });
    }
    landmarks.sort_by_key(|landmark| (landmark.position.y, landmark.position.x));
    Ok(landmarks)
}

/// Pairs caches and threat camps inside small ruined compounds. A full
/// walkable padding ring is required around every footprint so the new walls
/// cannot form a choke point with pre-existing terrain. The caller still runs
/// the independent map validator after applying the returned overlay.
pub fn generate_regional_sites(
    map: &Map,
    passages: &[GridPos],
    reserved: &BTreeSet<GridPos>,
    landmarks: RegionLandmarkProfile,
    sites: RegionSiteProfile,
    interactive_entrances: bool,
    region_seed: u64,
) -> Result<GeneratedRegionalSiteLayout, RegionalLandmarkError> {
    if sites.is_empty() {
        return Ok(GeneratedRegionalSiteLayout {
            landmarks: generate_regional_landmarks(
                map,
                passages,
                reserved,
                landmarks,
                region_seed,
            )?,
            terrain: BTreeMap::new(),
            interactions: BTreeMap::new(),
            sites: Vec::new(),
        });
    }

    let mut rng = GameRng::from_seed(region_seed ^ SITE_SEED_SALT);
    let cache_count = inclusive(&mut rng, landmarks.cache_range());
    let camp_count = inclusive(&mut rng, landmarks.threat_camp_range());
    let desired_compounds = inclusive(&mut rng, sites.compound_range());
    let compound_count = desired_compounds.min(cache_count).min(camp_count);
    let (width, height) = sites.size();
    let mut unavailable = reserved.clone();
    unavailable.extend(passages.iter().copied());
    let mut generated = Vec::with_capacity(usize::from(cache_count + camp_count));
    let mut terrain = BTreeMap::new();
    let mut interactions = BTreeMap::new();
    let mut generated_sites = Vec::with_capacity(usize::from(compound_count));

    for placed in 0..compound_count {
        let candidates = site_center_candidates(
            map,
            passages,
            &unavailable,
            width,
            height,
            landmarks.minimum_passage_distance(),
        );
        if candidates.is_empty() {
            return Err(RegionalLandmarkError::InsufficientSiteSpace {
                requested: compound_count,
                placed,
            });
        }
        let center = candidates[below(&mut rng, candidates.len() as u64) as usize];
        let cache_distance = i32::from(width / 2).saturating_sub(1).max(1);
        let cache_direction = if rng.next_u64().is_multiple_of(2) {
            -1
        } else {
            1
        };
        let cache = GridPos::new(center.x + cache_distance * cache_direction, center.y);
        generated.push(GeneratedRegionalLandmark {
            kind: RegionalLandmarkKind::ThreatCamp,
            position: center,
        });
        generated.push(GeneratedRegionalLandmark {
            kind: RegionalLandmarkKind::SupplyCache,
            position: cache,
        });
        paint_ruined_compound(&mut terrain, center, width, height);
        let entrance_kind = if interactive_entrances {
            choose_site_entrance(&mut rng, sites.entrances())
        } else {
            RegionSiteEntranceKind::Open
        };
        let (entrance, console) = paint_site_entrance(
            &mut interactions,
            center,
            height,
            entrance_kind,
            interactive_entrances,
        );
        generated_sites.push(GeneratedRegionalSite {
            camp: center,
            cache,
            entrance,
            console,
            entrance_kind,
        });
        unavailable.extend(padded_site_cells(center, width, height));
    }

    let remaining_caches = cache_count - compound_count;
    let remaining_camps = camp_count - compound_count;
    let remaining_total = usize::from(remaining_caches + remaining_camps);
    let mut candidates = standalone_candidates(
        map,
        passages,
        &unavailable,
        landmarks.minimum_passage_distance(),
    );
    if candidates.len() < remaining_total {
        return Err(RegionalLandmarkError::InsufficientSpace {
            requested: remaining_total,
            available: candidates.len(),
        });
    }
    for kind in std::iter::repeat_n(
        RegionalLandmarkKind::SupplyCache,
        usize::from(remaining_caches),
    )
    .chain(std::iter::repeat_n(
        RegionalLandmarkKind::ThreatCamp,
        usize::from(remaining_camps),
    )) {
        let index = below(&mut rng, candidates.len() as u64) as usize;
        let position = candidates.swap_remove(index);
        generated.push(GeneratedRegionalLandmark { kind, position });
    }
    generated.sort_by_key(|landmark| (landmark.position.y, landmark.position.x));
    Ok(GeneratedRegionalSiteLayout {
        landmarks: generated,
        terrain,
        interactions,
        sites: generated_sites,
    })
}

/// Selects terminal-bearing compounds and archive records independently from
/// layout and security. Site indices are sampled without replacement.
pub fn generate_regional_site_terminals(
    sites: &[GeneratedRegionalSite],
    profile: &RegionSiteTerminalProfile,
    region_seed: u64,
) -> Vec<GeneratedRegionalSiteTerminal> {
    if sites.is_empty() {
        return Vec::new();
    }
    let mut rng = GameRng::from_seed(region_seed ^ SITE_TERMINAL_SEED_SALT);
    let (minimum, maximum) = profile.terminal_range();
    let maximum = maximum.min(sites.len() as u16);
    let minimum = minimum.min(maximum);
    let count = inclusive(&mut rng, (minimum, maximum));
    let mut candidates: Vec<_> = (0..sites.len()).collect();
    let records = profile.records();
    let mut generated = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let candidate = below(&mut rng, candidates.len() as u64) as usize;
        let site_index = candidates.swap_remove(candidate);
        let record_index = below(&mut rng, records.len() as u64) as usize;
        let site = sites[site_index];
        generated.push(GeneratedRegionalSiteTerminal {
            site_index,
            // A wall-mounted fixture remains adjacent to navigable interior
            // floor without turning the terminal itself into a walkable tile.
            position: GridPos::new(site.camp.x - 2, site.camp.y - 3),
            record: records[record_index].clone(),
        });
    }
    generated.sort_by_key(|terminal| terminal.site_index);
    generated
}

fn choose_site_entrance(
    rng: &mut GameRng,
    profile: RegionSiteEntranceProfile,
) -> RegionSiteEntranceKind {
    let total = profile.total_weight();
    if total == 0 {
        return RegionSiteEntranceKind::Open;
    }
    let roll = below(rng, u64::from(total)) as u32;
    let open_end = u32::from(profile.open_weight());
    let closed_end = open_end + u32::from(profile.closed_door_weight());
    if roll < open_end {
        RegionSiteEntranceKind::Open
    } else if roll < closed_end {
        RegionSiteEntranceKind::ClosedDoor
    } else {
        RegionSiteEntranceKind::LockedConsole
    }
}

fn paint_site_entrance(
    interactions: &mut BTreeMap<GridPos, Terrain>,
    center: GridPos,
    height: u16,
    kind: RegionSiteEntranceKind,
    interactive: bool,
) -> (GridPos, Option<GridPos>) {
    let door = GridPos::new(center.x, center.y + i32::from(height / 2));
    if !interactive {
        return (door, None);
    }
    match kind {
        RegionSiteEntranceKind::Open => (door, None),
        RegionSiteEntranceKind::ClosedDoor => {
            interactions.insert(door, Terrain::Door(DoorState::Closed));
            (door, None)
        }
        RegionSiteEntranceKind::LockedConsole => {
            let console = GridPos::new(door.x + 2, door.y + 1);
            interactions.insert(door, Terrain::Door(DoorState::Locked));
            interactions.insert(
                console,
                Terrain::ControlPanel {
                    door,
                    activated: false,
                },
            );
            (door, Some(console))
        }
    }
}

fn site_center_candidates(
    map: &Map,
    passages: &[GridPos],
    unavailable: &BTreeSet<GridPos>,
    width: u16,
    height: u16,
    minimum_passage_distance: u16,
) -> Vec<GridPos> {
    let half_width = i32::from(width / 2);
    let half_height = i32::from(height / 2);
    let minimum_x = half_width + 2;
    let maximum_x = map.width() as i32 - half_width - 3;
    let minimum_y = half_height + 2;
    let maximum_y = map.height() as i32 - half_height - 3;
    if minimum_x > maximum_x || minimum_y > maximum_y {
        return Vec::new();
    }
    (minimum_y..=maximum_y)
        .flat_map(|y| (minimum_x..=maximum_x).map(move |x| GridPos::new(x, y)))
        .filter(|center| {
            passages
                .iter()
                .all(|passage| manhattan(*center, *passage) >= u32::from(minimum_passage_distance))
                && padded_site_cells(*center, width, height)
                    .into_iter()
                    .all(|position| {
                        map.is_walkable(position)
                            && !map.is_protected(position)
                            && !unavailable.contains(&position)
                    })
        })
        .collect()
}

fn standalone_candidates(
    map: &Map,
    passages: &[GridPos],
    unavailable: &BTreeSet<GridPos>,
    minimum_passage_distance: u16,
) -> Vec<GridPos> {
    (1..map.height() as i32 - 1)
        .flat_map(|y| (1..map.width() as i32 - 1).map(move |x| GridPos::new(x, y)))
        .filter(|position| {
            map.is_walkable(*position)
                && !map.is_protected(*position)
                && !unavailable.contains(position)
                && passages.iter().all(|passage| {
                    manhattan(*position, *passage) >= u32::from(minimum_passage_distance)
                })
        })
        .collect()
}

fn padded_site_cells(center: GridPos, width: u16, height: u16) -> Vec<GridPos> {
    let half_width = i32::from(width / 2);
    let half_height = i32::from(height / 2);
    ((center.y - half_height - 1)..=(center.y + half_height + 1))
        .flat_map(|y| {
            ((center.x - half_width - 1)..=(center.x + half_width + 1))
                .map(move |x| GridPos::new(x, y))
        })
        .collect()
}

fn paint_ruined_compound(
    terrain: &mut BTreeMap<GridPos, RegionTerrain>,
    center: GridPos,
    width: u16,
    height: u16,
) {
    let half_width = i32::from(width / 2);
    let half_height = i32::from(height / 2);
    for y in center.y - half_height..=center.y + half_height {
        for x in center.x - half_width..=center.x + half_width {
            let boundary = x == center.x - half_width
                || x == center.x + half_width
                || y == center.y - half_height
                || y == center.y + half_height;
            let entrance = x == center.x && y == center.y + half_height;
            terrain.insert(
                GridPos::new(x, y),
                if boundary && !entrance {
                    RegionTerrain::RuinWall
                } else {
                    RegionTerrain::RuinFloor
                },
            );
        }
    }
}

fn inclusive(rng: &mut GameRng, range: (u16, u16)) -> u16 {
    range.0 + below(rng, u64::from(range.1 - range.0) + 1) as u16
}

fn below(rng: &mut GameRng, bound: u64) -> u64 {
    let threshold = bound.wrapping_neg() % bound;
    loop {
        let value = rng.next_u64();
        if value >= threshold {
            return value % bound;
        }
    }
}

fn manhattan(left: GridPos, right: GridPos) -> u32 {
    left.x.abs_diff(right.x) + left.y.abs_diff(right.y)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegionalLandmarkError {
    InsufficientSpace { requested: usize, available: usize },
    InsufficientSiteSpace { requested: u16, placed: u16 },
}

impl Display for RegionalLandmarkError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientSpace {
                requested,
                available,
            } => write!(
                formatter,
                "regional landmarks requested {requested} cells but only {available} are available"
            ),
            Self::InsufficientSiteSpace { requested, placed } => write!(
                formatter,
                "regional sites requested {requested} compounds but only {placed} safe footprints could be placed"
            ),
        }
    }
}

impl Error for RegionalLandmarkError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::generation::{MapValidationRules, validate_interactive_map};

    #[test]
    fn landmarks_are_deterministic_distinct_walkable_and_away_from_passages() {
        let map = Map::from_ascii(
            "############\n#..........#\n#..........#\n#..........#\n#..........#\n#..........#\n#..........#\n############",
        )
        .unwrap();
        let passages = [GridPos::new(1, 1), GridPos::new(10, 6)];
        let reserved = BTreeSet::from([GridPos::new(5, 3)]);
        let profile = RegionLandmarkProfile::new(2, 2, 1, 2, 2).unwrap();
        let first = generate_regional_landmarks(&map, &passages, &reserved, profile, 77).unwrap();
        let second = generate_regional_landmarks(&map, &passages, &reserved, profile, 77).unwrap();

        assert_eq!(first, second);
        assert!((3..=4).contains(&first.len()));
        assert_eq!(
            first
                .iter()
                .map(|landmark| landmark.position)
                .collect::<BTreeSet<_>>()
                .len(),
            first.len()
        );
        assert!(first.iter().all(|landmark| {
            map.is_walkable(landmark.position)
                && !reserved.contains(&landmark.position)
                && passages
                    .iter()
                    .all(|passage| manhattan(landmark.position, *passage) >= 2)
        }));
    }

    #[test]
    fn site_terminals_use_a_bounded_deterministic_stream_independent_of_security() {
        let sites = [
            GeneratedRegionalSite {
                camp: GridPos::new(10, 10),
                cache: GridPos::new(13, 10),
                entrance: GridPos::new(10, 13),
                console: None,
                entrance_kind: RegionSiteEntranceKind::Open,
            },
            GeneratedRegionalSite {
                camp: GridPos::new(30, 20),
                cache: GridPos::new(27, 20),
                entrance: GridPos::new(30, 23),
                console: None,
                entrance_kind: RegionSiteEntranceKind::ClosedDoor,
            },
        ];
        let profile = RegionSiteTerminalProfile::new(
            2,
            2,
            vec![
                "test:field_log".parse().unwrap(),
                "test:maintenance_log".parse().unwrap(),
            ],
        )
        .unwrap();

        let first = generate_regional_site_terminals(&sites, &profile, 91);
        let second = generate_regional_site_terminals(&sites, &profile, 91);

        assert_eq!(first, second);
        assert_eq!(first.len(), 2);
        assert_eq!(
            first
                .iter()
                .map(|terminal| terminal.site_index)
                .collect::<BTreeSet<_>>(),
            BTreeSet::from([0, 1])
        );
        assert!(first.iter().all(|terminal| {
            terminal.position
                == GridPos::new(
                    sites[terminal.site_index].camp.x - 2,
                    sites[terminal.site_index].camp.y - 3,
                )
        }));
    }

    #[test]
    fn sites_pair_landmarks_inside_deterministic_open_ruins() {
        let map = Map::from_ascii(
            "########################################\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n########################################",
        )
        .unwrap();
        let passages = [GridPos::new(1, 15), GridPos::new(38, 15)];
        let landmarks = RegionLandmarkProfile::new(2, 2, 1, 2, 3).unwrap();
        let sites = RegionSiteProfile::new(1, 2, 9, 7).unwrap();
        let first = generate_regional_sites(
            &map,
            &passages,
            &BTreeSet::new(),
            landmarks,
            sites,
            false,
            77,
        )
        .unwrap();
        let second = generate_regional_sites(
            &map,
            &passages,
            &BTreeSet::new(),
            landmarks,
            sites,
            false,
            77,
        )
        .unwrap();

        assert_eq!(first, second);
        let wall_count = first
            .terrain()
            .values()
            .filter(|terrain| **terrain == RegionTerrain::RuinWall)
            .count();
        assert!(wall_count >= 27 && wall_count.is_multiple_of(27));
        assert!(first.landmarks().iter().any(|camp| {
            camp.kind == RegionalLandmarkKind::ThreatCamp
                && first.terrain().get(&camp.position) == Some(&RegionTerrain::RuinFloor)
                && first.landmarks().iter().any(|cache| {
                    cache.kind == RegionalLandmarkKind::SupplyCache
                        && cache.position.y == camp.position.y
                        && cache.position.x.abs_diff(camp.position.x) == 3
                })
                && first
                    .terrain()
                    .get(&GridPos::new(camp.position.x, camp.position.y + 3))
                    == Some(&RegionTerrain::RuinFloor)
        }));
        assert!(first.interactions().is_empty());
    }

    #[test]
    fn site_entrance_variants_use_shared_door_and_console_primitives() {
        let map = Map::from_ascii(
            "########################################\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n#......................................#\n########################################",
        )
        .unwrap();
        let passages = [GridPos::new(1, 15), GridPos::new(38, 15)];
        let landmarks = RegionLandmarkProfile::new(1, 1, 1, 1, 3).unwrap();
        let base_sites = RegionSiteProfile::new(1, 1, 9, 7).unwrap();

        let open = generate_regional_sites(
            &map,
            &passages,
            &BTreeSet::new(),
            landmarks,
            base_sites
                .with_entrances(RegionSiteEntranceProfile::new(1, 0, 0))
                .unwrap(),
            true,
            77,
        )
        .unwrap();
        assert!(open.interactions().is_empty());

        let closed = generate_regional_sites(
            &map,
            &passages,
            &BTreeSet::new(),
            landmarks,
            base_sites
                .with_entrances(RegionSiteEntranceProfile::new(0, 1, 0))
                .unwrap(),
            true,
            77,
        )
        .unwrap();
        assert_eq!(closed.interactions().len(), 1);
        assert!(matches!(
            closed.interactions().values().next(),
            Some(Terrain::Door(DoorState::Closed))
        ));

        let locked = generate_regional_sites(
            &map,
            &passages,
            &BTreeSet::new(),
            landmarks,
            base_sites
                .with_entrances(RegionSiteEntranceProfile::new(0, 0, 1))
                .unwrap(),
            true,
            77,
        )
        .unwrap();
        let (&door, _) = locked
            .interactions()
            .iter()
            .find(|(_, terrain)| matches!(terrain, Terrain::Door(DoorState::Locked)))
            .unwrap();
        let (&console, _) = locked
            .interactions()
            .iter()
            .find(|(_, terrain)| {
                matches!(terrain, Terrain::ControlPanel { door: linked, activated: false } if *linked == door)
            })
            .unwrap();
        assert_eq!(console, GridPos::new(door.x + 2, door.y + 1));
        assert_eq!(locked.terrain().get(&door), Some(&RegionTerrain::RuinFloor));

        let mut interactive_map = map.clone();
        for (position, terrain) in locked.terrain() {
            interactive_map
                .set_terrain(
                    *position,
                    if *terrain == RegionTerrain::RuinWall {
                        Terrain::Wall
                    } else {
                        Terrain::Floor
                    },
                )
                .unwrap();
        }
        for (position, terrain) in locked.interactions() {
            interactive_map.set_terrain(*position, *terrain).unwrap();
        }
        validate_interactive_map(
            &interactive_map,
            passages[0],
            passages[1],
            &locked
                .landmarks()
                .iter()
                .map(|landmark| landmark.position)
                .collect::<Vec<_>>(),
            MapValidationRules::default(),
        )
        .unwrap();
    }
}
