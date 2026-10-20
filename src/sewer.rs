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
    IDS.iter().position(|&s| s == id).map(|i| i + 1)
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
    let plan = plan(index);
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
    Ok(ZoneBlueprint {
        info,
        map,
        entrance,
        seed: seed ^ plan.seed,
        actors: actors(plan, catalog)?,
        loot: Vec::new(),
        threat_sources: Vec::new(),
    })
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
    for (i, id) in IDS.iter().enumerate() {
        let id = id.parse().unwrap();
        if world.map_in_zone(&id).is_some() {
            world.restore_static_zone_obstructions(&id, &obstacles(plan(i + 1)))?;
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "sewer_tests.rs"]
mod tests;
