//! The reviewed starting camp. Scene data is independent of campaign RNG.
use crate::test_sector::{Decor, SectorDecor, TestSector};
use project_rl::entity::Actor;
use project_rl::world::generation::{GeneratedMap, MapValidationRules};
use project_rl::world::{GridPos, Map, Terrain};
use serde::Deserialize;
use std::sync::OnceLock;

pub const ZONE_ID: &str = "core:cryogenic_camp";
pub const NAME: &str = "Camp";
pub const OFFICER_DIALOGUE: &str = "Debout, détenu. Les démons ravagent la ville. Nous n’avons plus assez d’hommes à envoyer sous terre.\n\nLe passage qu’ils empruntent est maintenu ouvert par un dispositif : le Puits. Vous allez le rejoindre et couper ce qui alimente l’invasion.\n\nVous réussissez, votre peine sera commuée. Vous serez libre. Vous n’avez de toute façon pas le choix.";

#[derive(Deserialize)]
pub struct Sprite {
    pub id: String,
    pub file: String,
    pub at: [i32; 2],
    #[serde(default = "one")]
    pub width: i32,
    #[serde(default = "one")]
    pub height: i32,
}
fn one() -> i32 {
    1
}

#[derive(Deserialize)]
pub struct Camp {
    pub width: usize,
    pub height: usize,
    pub floor: String,
    pub structure: String,
    pub actors: Vec<Sprite>,
    pub props: Vec<Sprite>,
    pub exits: Vec<[i32; 2]>,
    pub atmosphere: crate::camp_atmosphere::Config,
}

pub fn scene() -> &'static Camp {
    static SCENE: OnceLock<Camp> = OnceLock::new();
    SCENE.get_or_init(|| {
        serde_json::from_str(include_str!("../assets/maps/cryogenic-camp-01/camp.json"))
            .expect("reviewed camp manifest")
    })
}
pub fn pos(at: [i32; 2]) -> GridPos {
    GridPos::new(at[0], at[1])
}

pub fn actor_index(actor: &Actor) -> Option<u8> {
    actor.tags().iter().find_map(|tag| {
        tag.as_str()
            .strip_prefix("core:camp_npc_")?
            .parse::<u8>()
            .ok()
            .filter(|i| (1..=4).contains(i))
    })
}
pub fn npc_name(index: u8) -> &'static str {
    match index {
        1 => "Officier",
        2 => "Armurier",
        3 => "Médecin",
        _ => "Technicien",
    }
}

pub fn restore_obstructions(world: &mut project_rl::game::WorldState) -> Result<(), String> {
    let zone = ZONE_ID.parse().unwrap();
    if world.zone_info(&zone).is_none() {
        return Ok(());
    }
    let obstacles: Vec<_> = scene()
        .props
        .iter()
        .flat_map(|p| {
            (0..p.height).flat_map(move |dy| {
                (0..p.width).map(move |dx| (GridPos::new(p.at[0] + dx, p.at[1] + dy), true, false))
            })
        })
        .collect();
    world.restore_static_zone_obstructions(&zone, &obstacles)
}

pub fn sector() -> Result<TestSector, String> {
    let c = scene();
    // The illustrated exit lies on the right image edge. An unpainted outer
    // wall column keeps the simulation's sealed-border invariant, without
    // moving any reviewed cell, furniture, actor or source pixel.
    let mut map = Map::filled(c.width + 1, c.height, Terrain::Wall).map_err(|e| e.to_string())?;
    let mut decor = SectorDecor {
        fallback_name: Some(NAME.to_owned()),
        ..Default::default()
    };
    for (y, row) in include_str!("../assets/maps/cryogenic-camp-01/layout.txt")
        .lines()
        .enumerate()
    {
        for (x, ch) in row.chars().enumerate() {
            let at = GridPos::new(x as i32, y as i32);
            if ch == '.' {
                map.set_terrain(at, Terrain::Floor)
                    .map_err(|e| e.to_string())?;
                map.set_protected(at, true).map_err(|e| e.to_string())?;
                decor.cells.insert(at, Decor::Deck);
            }
        }
    }
    for p in &c.props {
        for dy in 0..p.height {
            for dx in 0..p.width {
                map.set_obstruction(GridPos::new(p.at[0] + dx, p.at[1] + dy), true, false)
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    let exit = pos(c.exits[0]);
    let required: Vec<_> = c.actors[1..].iter().map(|a| pos(a.at)).collect();
    let level = GeneratedMap::from_layout(
        map,
        pos(c.actors[0].at),
        exit,
        &required,
        MapValidationRules::default(),
    )
    .map_err(|e| e.to_string())?;
    Ok(TestSector {
        level,
        decor,
        enemies: Vec::new(),
        loot: Vec::new(),
    })
}
