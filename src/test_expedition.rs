//! Content adapter for the first expedition; all travel/persistence is headless.
use crate::test_sector::{Decor, SectorDecor};
use project_rl::ai::AiProfile;
use project_rl::combat::{AttackProfile, DamageType};
use project_rl::content::{ContentId, ExpeditionDefinition};
use project_rl::entity::Actor;
use project_rl::game::{GameRng, WorldState, ZoneBlueprint, ZoneInfo};
use project_rl::loot::{LootCatalog, LootContext};
use project_rl::progression::DefeatReward;
use project_rl::world::generation::RoomsGenerator;
use project_rl::world::{DistanceMetric, GridPos};
use std::collections::BTreeMap;

pub struct ExpeditionPresentation {
    pub source_passage: GridPos,
    pub decor: BTreeMap<ContentId, SectorDecor>,
}

pub fn attach(
    world: &mut WorldState,
    seed: u64,
    loot_catalog: Option<&LootCatalog>,
    definition: &ExpeditionDefinition,
) -> Result<ExpeditionPresentation, String> {
    let source = ZoneInfo {
        id: definition.hub.id.clone(),
        name: definition.hub.name.clone(),
        kind: definition.hub.kind.clone(),
        depth: definition.hub.depth,
    };
    let destination = ZoneInfo {
        id: definition.destination.zone.id.clone(),
        name: definition.destination.zone.name.clone(),
        kind: definition.destination.zone.kind.clone(),
        depth: definition.destination.zone.depth,
    };
    let mut rng = GameRng::from_seed(seed ^ definition.destination.seed_salt);
    let generated = RoomsGenerator::new(definition.destination.generator)
        .map_err(|e| e.to_string())?
        .generate(&mut rng)
        .map_err(|e| e.to_string())?;
    let (map, entrance, _) = generated.into_parts();
    let mut cells: Vec<_> = (1..map.height() as i32 - 1)
        .flat_map(|y| (1..map.width() as i32 - 1).map(move |x| GridPos::new(x, y)))
        .filter(|p| map.is_walkable(*p) && *p != entrance)
        .collect();
    cells.sort_by_key(|p| {
        (
            (p.x - entrance.x).abs() + (p.y - entrance.y).abs(),
            p.y,
            p.x,
        )
    });
    let loot = if let Some(catalog) = loot_catalog {
        let id: ContentId = definition
            .destination
            .loot_table
            .as_ref()
            .ok_or("Missing expedition loot table")?
            .clone();
        let table = catalog
            .get(&id)
            .ok_or_else(|| format!("Unknown expedition loot table '{id}'"))?;
        table
            .validate_items(&world.rules().items, &world.rules().weapons)
            .map_err(|e| e.to_string())?;
        let context = LootContext {
            depth: destination.depth,
            map_kind: destination.kind.clone(),
            source: definition.destination.loot_source.clone(),
        };
        // Separate stream: adding loot rolls never changes terrain or AI seeds.
        let mut loot_rng =
            GameRng::from_seed(seed ^ definition.destination.seed_salt ^ 0x4c4f_4f54_5f46_4c52);
        let drops = table
            .draw(&context, definition.destination.loot_draws, &mut loot_rng)
            .map_err(|e| e.to_string())?;
        if drops.len() > cells.len() {
            return Err("Not enough floor cells for expedition loot".into());
        }
        cells
            .iter()
            .copied()
            .zip(drops)
            .map(|(p, drop)| (p, drop.item, drop.quantity))
            .collect()
    } else {
        // v1/v2 run generation is immutable: old suspensions retain fixed loot.
        let mut legacy = Vec::new();
        for (position, id) in cells
            .iter()
            .copied()
            .take(2)
            .zip(["core:repair_patch", "core:needle_launcher"])
        {
            legacy.push((
                position,
                id.parse()
                    .map_err(|e: project_rl::content::ContentIdError| e.to_string())?,
                1,
            ));
        }
        legacy
    };
    let mut actors = Vec::new();
    for (index, position) in cells
        .iter()
        .copied()
        .filter(|p| (p.x - entrance.x).abs() + (p.y - entrance.y).abs() > 18)
        .rev()
        .take(3)
        .enumerate()
    {
        actors.push(
            Actor::new(position, 9)
                .map_err(|e| e.to_string())?
                .with_attack(AttackProfile::new(
                    1,
                    DistanceMetric::Chebyshev,
                    false,
                    DamageType::Kinetic,
                    3,
                    0,
                ))
                .with_ai(if index == 1 {
                    AiProfile::sentry(8, 0)
                } else {
                    AiProfile::hunter(8, 0)
                })
                .with_defeat_reward(DefeatReward::persistent(8, 2)),
        );
    }
    let source_passage = definition.hub_passage;
    let mut decor = SectorDecor::default();
    for position in cells {
        decor.cells.insert(position, Decor::Grate);
    }
    decor.cells.insert(entrance, Decor::Passage);
    world.enable(source.clone())?;
    world.add_zone(ZoneBlueprint {
        info: destination.clone(),
        map,
        entrance,
        seed: rng.state(),
        actors,
        loot,
    })?;
    world.connect(source.id, source_passage, destination.id.clone(), entrance)?;
    Ok(ExpeditionPresentation {
        source_passage,
        decor: [(destination.id, decor)].into(),
    })
}
