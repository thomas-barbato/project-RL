//! Accepted EG-COMP-04 / EG-POP-04: persistent side dungeon, native scenery.
use crate::prison_street::{Street, pos};
use project_rl::{
    content::RegionalWorldCatalog,
    entity::Actor,
    game::{WorldState, ZoneBlueprint, ZoneConnectionBlueprint, ZoneInfo},
    world::{GridPos, Map, Terrain},
};
use serde::Deserialize;
use std::sync::OnceLock;

pub const NAME: &str = "Égouts";
pub const ENTRY: GridPos = GridPos::new(2, 14);
const IDS: [&str; 3] = [
    "core:prison_sewer_01",
    "core:prison_sewer_02",
    "core:prison_sewer_03",
];
const CONNECTED_IDS: [&str; 3] = [
    "core:prison_sewer_canals_01",
    "core:prison_sewer_canals_02",
    "core:prison_sewer_canals_03",
];

#[derive(Deserialize)]
pub struct Member {
    pub at: [i32; 2],
    pub number: u16,
    pub variant: u8,
    #[serde(default = "level_one")]
    pub level: u16,
}
fn level_one() -> u16 {
    1
}
#[derive(Deserialize)]
pub struct Group {
    pub id: u8,
    #[cfg_attr(not(debug_assertions), allow(dead_code))]
    pub family: String,
    pub members: Vec<Member>,
}
#[derive(Deserialize)]
#[cfg_attr(not(debug_assertions), allow(dead_code))]
pub struct View {
    pub id: String,
    pub at: [i32; 2],
}
#[derive(Deserialize)]
pub struct RatRoute {
    pub path: Vec<[f32; 2]>,
    pub speed: f32,
    pub phase: f32,
}
#[derive(Deserialize)]
pub struct Plan {
    pub seed: u64,
    pub width: usize,
    pub height: usize,
    pub spawn: [i32; 2],
    pub layout: Vec<String>,
    pub groups: Vec<Group>,
    #[cfg_attr(not(debug_assertions), allow(dead_code))]
    pub views: Vec<View>,
    #[serde(default)]
    pub blocked: Vec<[i32; 2]>,
    #[serde(default)]
    pub rats: Vec<RatRoute>,
}

pub fn index_for_id(id: &str) -> Option<usize> {
    IDS.iter()
        .chain(CONNECTED_IDS.iter())
        .position(|&s| s == id)
        .map(|i| i % 3 + 1)
}
pub fn connected_id(id: &str) -> bool {
    CONNECTED_IDS.contains(&id)
}
fn plan_for_id(id: &str) -> Option<&'static Plan> {
    let index = index_for_id(id)?;
    Some(if connected_id(id) {
        connected_plan(index)
    } else {
        plan(index)
    })
}
pub fn current_plan(game: &WorldState) -> Option<&'static Plan> {
    plan_for_id(game.current_zone()?.id.as_str())
}
pub fn index(game: &WorldState) -> Option<usize> {
    index_for_id(game.current_zone()?.id.as_str())
}
pub fn variant(seed: u64) -> usize {
    let mut value = seed ^ 0x5345_5745_5230_3034;
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    (value % 3) as usize + 1
}
pub fn plan(index: usize) -> &'static Plan {
    static PLANS: OnceLock<[Plan; 3]> = OnceLock::new();
    &PLANS.get_or_init(|| {
        [
            serde_json::from_str(include_str!(
                "../assets/maps/prison-sewer-01/variant-1/plan.json"
            ))
            .unwrap(),
            serde_json::from_str(include_str!(
                "../assets/maps/prison-sewer-01/variant-2/plan.json"
            ))
            .unwrap(),
            serde_json::from_str(include_str!(
                "../assets/maps/prison-sewer-01/variant-3/plan.json"
            ))
            .unwrap(),
        ]
    })[index - 1]
}
pub fn has_street_entry(game: &WorldState) -> bool {
    game.passage_in(&crate::prison_street::ZONE_ID.parse().unwrap(), ENTRY)
        .is_some_and(|p| index_for_id(p.destination.as_str()).is_some())
}
pub fn scene(game: &WorldState) -> Option<&'static Street> {
    if let Some(index) = index(game) {
        static SCENES: OnceLock<[Street; 3]> = OnceLock::new();
        return Some(
            &SCENES.get_or_init(|| {
                [
                    serde_json::from_str(include_str!(
                        "../assets/maps/prison-sewer-01/variant-1/street.json"
                    ))
                    .unwrap(),
                    serde_json::from_str(include_str!(
                        "../assets/maps/prison-sewer-01/variant-2/street.json"
                    ))
                    .unwrap(),
                    serde_json::from_str(include_str!(
                        "../assets/maps/prison-sewer-01/variant-3/street.json"
                    ))
                    .unwrap(),
                ]
            })[index - 1],
        );
    }
    if game.current_zone()?.id.as_str() != crate::prison_street::ZONE_ID || !has_street_entry(game)
    {
        return None;
    }
    #[cfg(debug_assertions)]
    if crate::street_defense_review::enabled() {
        static DEFENSE: OnceLock<Street> = OnceLock::new();
        return Some(DEFENSE.get_or_init(|| {
            serde_json::from_str(include_str!(
                "../assets/maps/prison-sewer-01/entry-defense.json"
            ))
            .unwrap()
        }));
    }
    static ENTRY_SCENE: OnceLock<Street> = OnceLock::new();
    Some(ENTRY_SCENE.get_or_init(|| {
        serde_json::from_str(include_str!("../assets/maps/prison-sewer-01/entry.json")).unwrap()
    }))
}
pub fn connection(seed: u64, sector_depth: u16) -> ZoneConnectionBlueprint {
    let index = variant(seed);
    ZoneConnectionBlueprint {
        at: ENTRY,
        destination: ZoneInfo {
            id: IDS[index - 1].parse().unwrap(),
            name: NAME.into(),
            kind: "core:dungeon".parse().unwrap(),
            // This is a side area of the same sector. Its return is reversible
            // without changing the campaign's deeper-layer travel policy.
            depth: sector_depth,
        },
        arrival: pos(plan(index).spawn),
    }
}
pub fn connected_connection(seed: u64, sector_depth: u16) -> ZoneConnectionBlueprint {
    let mut result = connection(seed, sector_depth);
    result.destination.id = CONNECTED_IDS[variant(seed) - 1].parse().unwrap();
    result
}
pub fn prepare_street(map: &mut Map) -> Result<(), String> {
    #[derive(Deserialize)]
    struct Clearance {
        cells: Vec<[i32; 2]>,
    }
    static CLEARANCE: OnceLock<Clearance> = OnceLock::new();
    let cells = &CLEARANCE
        .get_or_init(|| {
            serde_json::from_str(include_str!(
                "../assets/maps/prison-sewer-01/entry-clearance.json"
            ))
            .unwrap()
        })
        .cells;
    for &cell in cells {
        let at = pos(cell);
        #[cfg(debug_assertions)]
        if crate::street_population::enabled()
            && crate::street_defense_review::data()
                .proposed_solid_cells
                .contains(&cell)
        {
            continue;
        }
        if map.tile(at).is_some_and(|t| t.terrain == Terrain::Floor) {
            map.set_obstruction(at, false, false)
                .map_err(|e| e.to_string())?;
        }
    }
    if !map.is_walkable(ENTRY) {
        return Err("Accepted sewer entry is blocked".into());
    }
    map.set_protected(ENTRY, true).map_err(|e| e.to_string())
}
fn obstacles(plan: &Plan) -> Vec<(GridPos, bool, bool)> {
    let mut result: Vec<_> = plan
        .layout
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes().enumerate().filter_map(move |(x, c)| {
                (c == b'~').then_some((GridPos::new(x as i32, y as i32), true, false))
            })
        })
        .collect();
    result.extend(plan.blocked.iter().map(|&p| (pos(p), true, false)));
    result
}
pub fn actors(plan: &Plan, catalog: &RegionalWorldCatalog) -> Result<Vec<Actor>, String> {
    let world = catalog
        .get(&"core:simulation_overworld".parse().unwrap())
        .ok_or("Sewer surface profiles absent")?;
    let mut result = Vec::new();
    for group in &plan.groups {
        for member in &group.members {
            let biome = world
                .biome(
                    &if member.level == 1 {
                        "core:human_habitat"
                    } else {
                        "core:surface_wilds"
                    }
                    .parse()
                    .unwrap(),
                )
                .ok_or("Sewer ordinary profile absent")?;
            let rule = &biome.encounters().rules()[usize::from(member.number == 24)];
            let mut actor = Actor::new(pos(member.at), rule.maximum_integrity())
                .map_err(|e| e.to_string())?
                .with_attack(rule.attack())
                .with_ai(rule.ai())
                .with_player_relation(rule.player_relation())
                .with_tags(rule.tags().iter().cloned());
            if let Some(a) = rule.primary_attributes() {
                actor = actor.with_primary_attributes(a);
            }
            if let Some(b) = rule.body_profile() {
                actor = actor.with_body_profile(b);
            }
            if !rule.body_components().is_empty() {
                actor = actor.with_body_components(rule.body_components().iter().cloned());
            }
            if let Some(mut reward) = rule.defeat_reward() {
                reward.threat_level = member.level;
                actor = actor.with_defeat_reward(reward);
            }
            result.push(
                actor.with_tags([
                    format!("core:monster_art_{}_{}", member.number, member.variant)
                        .parse()
                        .unwrap(),
                    format!("core:encounter_level_{}", member.level)
                        .parse()
                        .unwrap(),
                    format!("core:sewer_group_{}", group.id).parse().unwrap(),
                    "core:sewer_population_04".parse().unwrap(),
                ]),
            );
        }
    }
    Ok(result)
}
pub fn blueprint(
    info: ZoneInfo,
    seed: u64,
    catalog: &RegionalWorldCatalog,
) -> Result<ZoneBlueprint, String> {
    let index = index_for_id(info.id.as_str()).ok_or("Unknown accepted sewer variant")?;
    let connected = connected_id(info.id.as_str());
    let plan = if connected {
        connected_plan(index)
    } else {
        plan(index)
    };
    let map = construct_map(plan, connected)?;
    Ok(ZoneBlueprint {
        info,
        map,
        entrance: pos(plan.spawn),
        seed: seed ^ plan.seed,
        actors: actors(plan, catalog)?,
        loot: Vec::new(),
        threat_sources: Vec::new(),
    })
}

// Historical layouts keep their exact geometry. Every connected-sewer build
// must pass the water and navigable-floor invariants before being published.
fn construct_map(plan: &Plan, connected_channels: bool) -> Result<Map, String> {
    let mut map = Map::filled(plan.width, plan.height, Terrain::Wall).map_err(|e| e.to_string())?;
    for (y, row) in plan.layout.iter().enumerate() {
        for (x, c) in row.bytes().enumerate() {
            if c != b'#' {
                map.set_terrain(GridPos::new(x as i32, y as i32), Terrain::Floor)
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    for (at, movement, vision) in obstacles(plan) {
        map.set_obstruction(at, movement, vision)
            .map_err(|e| e.to_string())?;
    }
    let entrance = pos(plan.spawn);
    map.set_protected(entrance, true)
        .map_err(|e| e.to_string())?;
    if connected_channels {
        validate_channels(plan, &map)?;
    }
    Ok(map)
}

// The network continues under walkable grates. Cardinal joins match the
// actual movement graph; diagonal contact is insufficient for either graph.
fn validate_channels(plan: &Plan, map: &Map) -> Result<(), String> {
    use std::collections::{BTreeSet, VecDeque};
    fn flood(cells: &BTreeSet<GridPos>, start: GridPos) -> BTreeSet<GridPos> {
        let mut seen = BTreeSet::new();
        let mut todo = VecDeque::new();
        if cells.contains(&start) {
            seen.insert(start);
            todo.push_back(start);
        }
        while let Some(at) = todo.pop_front() {
            for next in at.cardinal_neighbors() {
                if cells.contains(&next) && seen.insert(next) {
                    todo.push_back(next);
                }
            }
        }
        seen
    }
    let channels: BTreeSet<_> = plan
        .layout
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes().enumerate().filter_map(move |(x, c)| {
                matches!(c, b'~' | b'=').then_some(GridPos::new(x as i32, y as i32))
            })
        })
        .collect();
    let Some(&first) = channels.first() else {
        return Err("Sewer construction requires a water network".into());
    };
    if flood(&channels, first) != channels {
        return Err("Sewer construction left disconnected water channels".into());
    }
    let dry: BTreeSet<_> = (0..map.height() as i32)
        .flat_map(|y| (0..map.width() as i32).map(move |x| GridPos::new(x, y)))
        .filter(|&at| map.is_walkable(at))
        .collect();
    let start = pos(plan.spawn);
    if dry.len() < 2 || !dry.contains(&start) || flood(&dry, start) != dry {
        return Err("Sewer construction cut a dry route".into());
    }
    for at in plan
        .groups
        .iter()
        .flat_map(|g| g.members.iter().map(|m| m.at))
        .chain(plan.views.iter().map(|v| v.at))
    {
        if !dry.contains(&pos(at)) {
            return Err("Sewer construction blocked a character or room access".into());
        }
    }
    for (y, row) in plan.layout.iter().enumerate() {
        for (x, c) in row.bytes().enumerate() {
            if c == b'=' && !map.is_walkable(GridPos::new(x as i32, y as i32)) {
                return Err("Sewer grate is not a usable crossing".into());
            }
        }
    }
    Ok(())
}

pub fn connected_plan(index: usize) -> &'static Plan {
    static PLANS: OnceLock<[Plan; 3]> = OnceLock::new();
    &PLANS.get_or_init(|| {
        [
            serde_json::from_str(include_str!(
                "../assets/maps/prison-sewer-canals-02/variant-1-plan.json"
            ))
            .unwrap(),
            serde_json::from_str(include_str!(
                "../assets/maps/prison-sewer-canals-02/variant-2-plan.json"
            ))
            .unwrap(),
            serde_json::from_str(include_str!(
                "../assets/maps/prison-sewer-canals-02/variant-3-plan.json"
            ))
            .unwrap(),
        ]
    })[index - 1]
}

#[cfg(any(test, debug_assertions))]
pub(crate) fn canal_candidate(index: usize) -> &'static Plan {
    connected_plan(index)
}

#[cfg(any(test, debug_assertions))]
pub(crate) fn canal_candidate_map(index: usize) -> Result<Map, String> {
    construct_map(canal_candidate(index), true)
}
pub fn restore_obstructions(world: &mut WorldState) -> Result<(), String> {
    if has_street_entry(world) {
        let id = crate::prison_street::ZONE_ID.parse().unwrap();
        if world.map_in_zone(&id).is_some() {
            let info = world.zone_info(&id).unwrap().clone();
            let mut map =
                crate::prison_street::blueprint(info, 0, true, true, true, true, true)?.map;
            prepare_street(&mut map)?;
            let fields: Vec<_> = (0..map.height() as i32)
                .flat_map(|y| (0..map.width() as i32).map(move |x| GridPos::new(x, y)))
                .map(|at| {
                    let (m, v) = map.obstruction(at);
                    (at, m, v)
                })
                .collect();
            world.restore_static_zone_obstructions(&id, &fields)?;
        }
    }
    for &name in IDS.iter().chain(CONNECTED_IDS.iter()) {
        let id = name.parse().unwrap();
        if world.map_in_zone(&id).is_some() {
            let plan = plan_for_id(name).unwrap();
            world.restore_static_zone_obstructions(&id, &obstacles(plan))?;
            if connected_id(name) {
                validate_channels(plan, world.map_in_zone(&id).unwrap())?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "sewer_tests.rs"]
mod tests;
