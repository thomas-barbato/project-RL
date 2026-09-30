use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::content::{RegionMapSize, RegionTerrain, RegionTerrainProfile, RegionVerticalDirection};
use crate::game::GameRng;
use crate::world::{Direction, GridPos, Map, MapBuildError, Terrain};

use super::{
    MapValidationError, MapValidationRules, validate_interactive_map, validate_playable_map,
};

const GENERATION_ATTEMPTS: u16 = 32;

/// A deterministic local map whose cardinal passage positions are identical
/// for every biome of the same regional world. That stable geometry lets two
/// already known neighboring regions be linked without pre-generating either.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedRegionalMap {
    map: Map,
    terrain: BTreeMap<GridPos, RegionTerrain>,
    passages: [GridPos; 4],
}

impl GeneratedRegionalMap {
    pub const fn map(&self) -> &Map {
        &self.map
    }

    pub fn terrain(&self) -> &BTreeMap<GridPos, RegionTerrain> {
        &self.terrain
    }

    pub const fn passage(&self, direction: Direction) -> GridPos {
        self.passages[direction_index(direction)]
    }

    /// Applies blocking semantic decoration atomically, then proves that all
    /// four regional exits and every site landmark remain reachable.
    pub fn apply_terrain_overlay(
        &mut self,
        overlay: &BTreeMap<GridPos, RegionTerrain>,
        required_positions: &[GridPos],
    ) -> Result<(), RegionalGenerationError> {
        self.apply_site_overlay(overlay, &BTreeMap::new(), required_positions)
    }

    /// Applies both semantic site terrain and runtime interaction primitives
    /// atomically. Validation reasons about ordinary doors and consoles, so a
    /// locked entrance is accepted only when its control remains reachable.
    pub fn apply_site_overlay(
        &mut self,
        overlay: &BTreeMap<GridPos, RegionTerrain>,
        interactions: &BTreeMap<GridPos, Terrain>,
        required_positions: &[GridPos],
    ) -> Result<(), RegionalGenerationError> {
        let mut candidate = self.clone();
        for (position, terrain) in overlay {
            set_regional_terrain(
                &mut candidate.map,
                &mut candidate.terrain,
                *position,
                *terrain,
            )?;
        }
        for (position, terrain) in interactions {
            candidate
                .map
                .set_terrain(*position, *terrain)
                .map_err(RegionalGenerationError::MapBuild)?;
        }
        let mut required = vec![candidate.passages[1], candidate.passages[3]];
        required.extend_from_slice(required_positions);
        validate_interactive_map(
            &candidate.map,
            candidate.passages[0],
            candidate.passages[2],
            &required,
            MapValidationRules::default(),
        )
        .map_err(RegionalGenerationError::InvalidBlockingDecoration)?;
        *self = candidate;
        Ok(())
    }

    pub fn into_parts(self) -> (Map, BTreeMap<GridPos, RegionTerrain>, [GridPos; 4]) {
        (self.map, self.terrain, self.passages)
    }
}

pub struct RegionalMapGenerator<'a> {
    size: RegionMapSize,
    profile: &'a RegionTerrainProfile,
}

impl<'a> RegionalMapGenerator<'a> {
    pub const fn new(size: RegionMapSize, profile: &'a RegionTerrainProfile) -> Self {
        Self { size, profile }
    }

    pub fn generate(&self, seed: u64) -> Result<GeneratedRegionalMap, RegionalGenerationError> {
        let passages = cardinal_passages(self.size);
        let mut last_validation = None;
        for attempt in 0..GENERATION_ATTEMPTS {
            let mut rng =
                GameRng::from_seed(seed ^ u64::from(attempt).wrapping_mul(0x9E37_79B9_7F4A_7C15));
            let mut map = Map::filled(
                usize::from(self.size.width()),
                usize::from(self.size.height()),
                Terrain::Wall,
            )
            .map_err(RegionalGenerationError::MapBuild)?;
            let mut terrain = BTreeMap::new();
            fill_interior(&mut map, &mut terrain, self.profile.ground())?;
            for _ in 0..self.profile.patch_count() {
                paint_random_patch(&mut map, &mut terrain, self.profile, &mut rng)?;
            }
            carve_passage_routes(
                &mut map,
                &mut terrain,
                passages,
                self.profile.ground(),
                &mut rng,
            )?;
            seal_disconnected_pockets(&mut map, &mut terrain, passages[0])?;
            match validate_playable_map(
                &map,
                passages[0],
                passages[2],
                &[passages[1], passages[3]],
                MapValidationRules::default(),
            ) {
                Ok(()) => {
                    return Ok(GeneratedRegionalMap {
                        map,
                        terrain,
                        passages,
                    });
                }
                Err(error) => last_validation = Some(error),
            }
        }
        Err(RegionalGenerationError::CouldNotCreateConnectedMap {
            attempts: GENERATION_ATTEMPTS,
            last_validation: last_validation.expect("at least one generation attempt"),
        })
    }
}

/// Decoration may surround a tiny walkable pocket even though every passage
/// remains connected. Such inaccessible islands are converted to the first
/// blocking terrain already selected by the biome, so the final map contains
/// no unreachable floor and never advertises a place the player cannot enter.
fn seal_disconnected_pockets(
    map: &mut Map,
    terrain: &mut BTreeMap<GridPos, RegionTerrain>,
    origin: GridPos,
) -> Result<(), RegionalGenerationError> {
    let reachable = reachable_walkable(map, origin);
    let Some(barrier) = terrain.values().copied().find(|kind| !kind.is_walkable()) else {
        return Ok(());
    };
    let disconnected = terrain
        .keys()
        .copied()
        .filter(|position| map.is_walkable(*position) && !reachable.contains(position))
        .collect::<Vec<_>>();
    for position in disconnected {
        set_regional_terrain(map, terrain, position, barrier)?;
    }
    Ok(())
}

fn reachable_walkable(map: &Map, origin: GridPos) -> BTreeSet<GridPos> {
    let mut visited = BTreeSet::from([origin]);
    let mut frontier = VecDeque::from([origin]);
    while let Some(position) = frontier.pop_front() {
        for neighbor in position.cardinal_neighbors() {
            if map.is_walkable(neighbor) && visited.insert(neighbor) {
                frontier.push_back(neighbor);
            }
        }
    }
    visited
}

pub const fn cardinal_passage(size: RegionMapSize, direction: Direction) -> GridPos {
    cardinal_passages(size)[direction_index(direction)]
}

/// Stable central anchors for explicit atlas links between two depths. Up and
/// down use different cells so a future intermediate region may expose both.
pub const fn vertical_passage(size: RegionMapSize, direction: RegionVerticalDirection) -> GridPos {
    let center_x = size.width() as i32 / 2;
    let center_y = size.height() as i32 / 2;
    match direction {
        RegionVerticalDirection::Up => GridPos::new(center_x - 2, center_y),
        RegionVerticalDirection::Down => GridPos::new(center_x + 2, center_y),
    }
}

const fn cardinal_passages(size: RegionMapSize) -> [GridPos; 4] {
    let width = size.width() as i32;
    let height = size.height() as i32;
    [
        GridPos::new(width / 2, 1),
        GridPos::new(width - 2, height / 2),
        GridPos::new(width / 2, height - 2),
        GridPos::new(1, height / 2),
    ]
}

const fn direction_index(direction: Direction) -> usize {
    match direction {
        Direction::North => 0,
        Direction::East => 1,
        Direction::South => 2,
        Direction::West => 3,
    }
}

fn fill_interior(
    map: &mut Map,
    terrain: &mut BTreeMap<GridPos, RegionTerrain>,
    ground: RegionTerrain,
) -> Result<(), RegionalGenerationError> {
    for y in 1..map.height() as i32 - 1 {
        for x in 1..map.width() as i32 - 1 {
            set_regional_terrain(map, terrain, GridPos::new(x, y), ground)?;
        }
    }
    Ok(())
}

fn paint_random_patch(
    map: &mut Map,
    terrain: &mut BTreeMap<GridPos, RegionTerrain>,
    profile: &RegionTerrainProfile,
    rng: &mut GameRng,
) -> Result<(), RegionalGenerationError> {
    let selected = select_feature(profile, rng);
    let minimum = usize::from(profile.minimum_patch_radius());
    let maximum = usize::from(profile.maximum_patch_radius());
    let radius_x = rng
        .usize_inclusive(minimum, maximum)
        .expect("validated regional radius range") as i32;
    let radius_y = rng
        .usize_inclusive(minimum, maximum)
        .expect("validated regional radius range") as i32;
    let center = GridPos::new(
        rng.usize_inclusive(2, map.width() - 3)
            .expect("validated regional map width") as i32,
        rng.usize_inclusive(2, map.height() - 3)
            .expect("validated regional map height") as i32,
    );
    let radius_x_squared = i64::from(radius_x) * i64::from(radius_x);
    let radius_y_squared = i64::from(radius_y) * i64::from(radius_y);
    let limit = radius_x_squared * radius_y_squared;
    for y in (center.y - radius_y).max(1)..=(center.y + radius_y).min(map.height() as i32 - 2) {
        for x in (center.x - radius_x).max(1)..=(center.x + radius_x).min(map.width() as i32 - 2) {
            let position = GridPos::new(x, y);
            let dx = i64::from(x - center.x);
            let dy = i64::from(y - center.y);
            let distance = dx * dx * radius_y_squared + dy * dy * radius_x_squared;
            if distance > limit {
                continue;
            }
            let kind = match selected {
                RegionTerrain::DeepWater if distance.saturating_mul(100) > limit * 48 => {
                    RegionTerrain::ShallowWater
                }
                RegionTerrain::Tree | RegionTerrain::Boulder
                    if !rng.next_u64().is_multiple_of(2) =>
                {
                    continue;
                }
                RegionTerrain::RuinWall if !rng.next_u64().is_multiple_of(3) => continue,
                other => other,
            };
            set_regional_terrain(map, terrain, position, kind)?;
        }
    }
    Ok(())
}

fn select_feature(profile: &RegionTerrainProfile, rng: &mut GameRng) -> RegionTerrain {
    let total: u64 = profile
        .features()
        .iter()
        .map(|rule| u64::from(rule.weight()))
        .sum();
    let mut roll = rng.next_u64() % total;
    profile
        .features()
        .iter()
        .find_map(|rule| {
            let weight = u64::from(rule.weight());
            if roll < weight {
                Some(rule.terrain())
            } else {
                roll -= weight;
                None
            }
        })
        .expect("validated regional terrain rules have a positive total")
}

fn carve_passage_routes(
    map: &mut Map,
    terrain: &mut BTreeMap<GridPos, RegionTerrain>,
    passages: [GridPos; 4],
    ground: RegionTerrain,
    rng: &mut GameRng,
) -> Result<(), RegionalGenerationError> {
    let center = GridPos::new(map.width() as i32 / 2, map.height() as i32 / 2);
    for passage in passages {
        let horizontal_first = rng.next_u64().is_multiple_of(2);
        carve_route(map, terrain, passage, center, horizontal_first, ground)?;
        clear_square(map, terrain, passage, 1, ground)?;
    }
    clear_square(map, terrain, center, 2, ground)
}

fn carve_route(
    map: &mut Map,
    terrain: &mut BTreeMap<GridPos, RegionTerrain>,
    start: GridPos,
    end: GridPos,
    horizontal_first: bool,
    ground: RegionTerrain,
) -> Result<(), RegionalGenerationError> {
    let bend = if horizontal_first {
        GridPos::new(end.x, start.y)
    } else {
        GridPos::new(start.x, end.y)
    };
    carve_segment(map, terrain, start, bend, ground)?;
    carve_segment(map, terrain, bend, end, ground)
}

fn carve_segment(
    map: &mut Map,
    terrain: &mut BTreeMap<GridPos, RegionTerrain>,
    start: GridPos,
    end: GridPos,
    ground: RegionTerrain,
) -> Result<(), RegionalGenerationError> {
    let mut current = start;
    loop {
        clear_square(map, terrain, current, 1, ground)?;
        if current == end {
            return Ok(());
        }
        current = GridPos::new(
            current.x + (end.x - current.x).signum(),
            current.y + (end.y - current.y).signum(),
        );
    }
}

fn clear_square(
    map: &mut Map,
    terrain: &mut BTreeMap<GridPos, RegionTerrain>,
    center: GridPos,
    radius: i32,
    ground: RegionTerrain,
) -> Result<(), RegionalGenerationError> {
    for y in center.y - radius..=center.y + radius {
        for x in center.x - radius..=center.x + radius {
            if x > 0 && y > 0 && x < map.width() as i32 - 1 && y < map.height() as i32 - 1 {
                set_regional_terrain(map, terrain, GridPos::new(x, y), ground)?;
            }
        }
    }
    Ok(())
}

fn set_regional_terrain(
    map: &mut Map,
    terrain: &mut BTreeMap<GridPos, RegionTerrain>,
    position: GridPos,
    kind: RegionTerrain,
) -> Result<(), RegionalGenerationError> {
    let runtime = match kind {
        RegionTerrain::ShallowWater => Terrain::ShallowWater,
        RegionTerrain::DeepWater => Terrain::DeepWater,
        RegionTerrain::Tree | RegionTerrain::Boulder | RegionTerrain::RuinWall => Terrain::Wall,
        RegionTerrain::Gravel
        | RegionTerrain::Grass
        | RegionTerrain::Scrub
        | RegionTerrain::Mud
        | RegionTerrain::RuinFloor => Terrain::Floor,
    };
    map.set_terrain(position, runtime)
        .map_err(RegionalGenerationError::MapBuild)?;
    terrain.insert(position, kind);
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegionalGenerationError {
    MapBuild(MapBuildError),
    InvalidBlockingDecoration(MapValidationError),
    CouldNotCreateConnectedMap {
        attempts: u16,
        last_validation: MapValidationError,
    },
}

impl Display for RegionalGenerationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MapBuild(error) => write!(formatter, "regional map construction failed: {error}"),
            Self::InvalidBlockingDecoration(error) => write!(
                formatter,
                "regional blocking decoration made the map invalid: {error}"
            ),
            Self::CouldNotCreateConnectedMap {
                attempts,
                last_validation,
            } => write!(
                formatter,
                "regional map remained invalid after {attempts} attempts: {last_validation}"
            ),
        }
    }
}

impl Error for RegionalGenerationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{RegionTerrainProfile, RegionTerrainRule};

    fn profile(ground: RegionTerrain) -> RegionTerrainProfile {
        RegionTerrainProfile::new(
            ground,
            24,
            2,
            10,
            vec![
                RegionTerrainRule::new(RegionTerrain::Tree, 4).unwrap(),
                RegionTerrainRule::new(RegionTerrain::Boulder, 2).unwrap(),
                RegionTerrainRule::new(RegionTerrain::DeepWater, 2).unwrap(),
                RegionTerrainRule::new(RegionTerrain::Mud, 1).unwrap(),
            ],
        )
        .unwrap()
    }

    #[test]
    fn same_region_seed_produces_the_same_local_map_and_passages() {
        let size = RegionMapSize::new(96, 64).unwrap();
        let profile = profile(RegionTerrain::Grass);
        let generator = RegionalMapGenerator::new(size, &profile);

        assert_eq!(generator.generate(42), generator.generate(42));
        assert_ne!(
            generator.generate(42).unwrap().map(),
            generator.generate(43).unwrap().map()
        );
    }

    #[test]
    fn every_cardinal_passage_remains_connected_across_many_seeds() {
        let size = RegionMapSize::new(96, 64).unwrap();
        let profile = profile(RegionTerrain::Grass);
        let generator = RegionalMapGenerator::new(size, &profile);
        for seed in 0..128 {
            let generated = generator.generate(seed).unwrap();
            let north = generated.passage(Direction::North);
            let south = generated.passage(Direction::South);
            let east = generated.passage(Direction::East);
            let west = generated.passage(Direction::West);
            validate_playable_map(
                generated.map(),
                north,
                south,
                &[east, west],
                MapValidationRules::default(),
            )
            .unwrap();
        }
    }

    #[test]
    fn different_profiles_change_semantic_terrain_without_changing_anchors() {
        let size = RegionMapSize::new(64, 48).unwrap();
        let grass = profile(RegionTerrain::Grass);
        let gravel = profile(RegionTerrain::Gravel);
        let first = RegionalMapGenerator::new(size, &grass).generate(7).unwrap();
        let second = RegionalMapGenerator::new(size, &gravel)
            .generate(7)
            .unwrap();

        assert_eq!(first.passages, second.passages);
        assert_ne!(first.terrain, second.terrain);
        assert!(
            first
                .terrain
                .values()
                .any(|kind| *kind == RegionTerrain::Grass)
        );
        assert!(
            second
                .terrain
                .values()
                .any(|kind| *kind == RegionTerrain::Gravel)
        );
    }

    #[test]
    fn invalid_blocking_overlay_is_rejected_atomically() {
        let size = RegionMapSize::new(64, 48).unwrap();
        let terrain = profile(RegionTerrain::Grass);
        let mut generated = RegionalMapGenerator::new(size, &terrain)
            .generate(17)
            .unwrap();
        let before = generated.clone();
        let north = generated.passage(Direction::North);
        let overlay = BTreeMap::from([(north, RegionTerrain::RuinWall)]);

        assert!(matches!(
            generated.apply_terrain_overlay(&overlay, &[]),
            Err(RegionalGenerationError::InvalidBlockingDecoration(
                MapValidationError::BlockedPlayerStart(position)
            )) if position == north
        ));
        assert_eq!(generated, before);
    }
}
