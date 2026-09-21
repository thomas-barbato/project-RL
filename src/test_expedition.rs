//! Content adapter for the first expedition; all travel/persistence is headless.
use crate::test_sector::{Decor, SectorDecor};
use project_rl::ai::AiProfile;
use project_rl::combat::{AttackProfile, DamageType};
use project_rl::content::{ContentId, ExpeditionDefinition};
use project_rl::entity::Actor;
use project_rl::game::{GameRng, GameRules, WorldState, ZoneBlueprint, ZoneInfo};
use project_rl::loot::{LootCatalog, LootContext};
use project_rl::progression::DefeatReward;
use project_rl::world::generation::RoomsGenerator;
use project_rl::world::{DistanceMetric, GridPos};
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};

pub struct ExpeditionPresentation {
    pub source_passage: GridPos,
    pub decor: BTreeMap<ContentId, SectorDecor>,
}

pub struct GeneratedExpeditionDestination {
    pub blueprint: ZoneBlueprint,
    pub decor: SectorDecor,
}

#[derive(Clone, Copy, Debug)]
pub struct ExpeditionGenerationFeatures {
    pub defined_population: bool,
    pub expanded_world: bool,
    pub pursuit_limits: bool,
    pub pursuit_lifecycle: bool,
    pub primary_attributes: bool,
    pub physical_profiles: bool,
    pub electronic_systems: bool,
    pub preparation_disruption: bool,
    pub player_relations: bool,
}

pub fn attach(
    world: &mut WorldState,
    seed: u64,
    loot_catalog: Option<&LootCatalog>,
    definition: &ExpeditionDefinition,
    features: ExpeditionGenerationFeatures,
) -> Result<ExpeditionPresentation, String> {
    let source = source_info(definition);
    let destination =
        generate_destination(world.rules(), seed, loot_catalog, definition, features)?;
    let source_passage = definition.hub_passage_for_expanded_world(features.expanded_world);
    let destination_id = destination.blueprint.info.id.clone();
    let entrance = destination.blueprint.entrance;
    world.enable(source.clone())?;
    world.add_zone(destination.blueprint)?;
    world.connect(source.id, source_passage, destination_id.clone(), entrance)?;
    Ok(ExpeditionPresentation {
        source_passage,
        decor: [(destination_id, destination.decor)].into(),
    })
}

pub fn declare(
    world: &mut WorldState,
    definition: &ExpeditionDefinition,
    use_expanded_world: bool,
) -> Result<ExpeditionPresentation, String> {
    let source = source_info(definition);
    let destination = destination_info(definition);
    let source_passage = definition.hub_passage_for_expanded_world(use_expanded_world);
    world.enable(source.clone())?;
    world.declare_deferred_connection(source.id, source_passage, destination)?;
    Ok(ExpeditionPresentation {
        source_passage,
        decor: BTreeMap::new(),
    })
}

pub fn generate_destination(
    rules: &GameRules,
    seed: u64,
    loot_catalog: Option<&LootCatalog>,
    definition: &ExpeditionDefinition,
    features: ExpeditionGenerationFeatures,
) -> Result<GeneratedExpeditionDestination, String> {
    let destination = destination_info(definition);
    let mut rng = GameRng::from_seed(seed ^ definition.destination.seed_salt);
    let generated = RoomsGenerator::new(
        definition.destination_generator_for_expanded_world(features.expanded_world),
    )
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
            .validate_items(&rules.items, &rules.weapons)
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
    let actors = if features.defined_population {
        defined_population(definition, &cells, entrance, features)?
    } else {
        legacy_population(&cells, entrance)?
    };
    let mut decor = SectorDecor::default();
    for position in cells {
        decor.cells.insert(position, Decor::Grate);
    }
    decor.cells.insert(entrance, Decor::Passage);
    Ok(GeneratedExpeditionDestination {
        blueprint: ZoneBlueprint {
            info: destination,
            map,
            entrance,
            seed: rng.state(),
            actors,
            loot: loot.into_iter().map(Into::into).collect(),
            threat_sources: Vec::new(),
        },
        decor,
    })
}

fn source_info(definition: &ExpeditionDefinition) -> ZoneInfo {
    ZoneInfo {
        id: definition.hub.id.clone(),
        name: definition.hub.name.clone(),
        kind: definition.hub.kind.clone(),
        depth: definition.hub.depth,
    }
}

fn destination_info(definition: &ExpeditionDefinition) -> ZoneInfo {
    ZoneInfo {
        id: definition.destination.zone.id.clone(),
        name: definition.destination.zone.name.clone(),
        kind: definition.destination.zone.kind.clone(),
        depth: definition.destination.zone.depth,
    }
}

fn legacy_population(cells: &[GridPos], entrance: GridPos) -> Result<Vec<Actor>, String> {
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
    Ok(actors)
}

fn defined_population(
    definition: &ExpeditionDefinition,
    cells: &[GridPos],
    entrance: GridPos,
    features: ExpeditionGenerationFeatures,
) -> Result<Vec<Actor>, String> {
    let mut requests = definition
        .destination
        .population
        .iter()
        .enumerate()
        .flat_map(|(group_index, group)| {
            (0..group.count()).map(move |member_index| {
                (group.minimum_entrance_distance(), group_index, member_index)
            })
        })
        .collect::<Vec<_>>();
    // Satisfy the most constrained groups first, then preserve authored order.
    requests.sort_by_key(|(distance, group, member)| (Reverse(*distance), *group, *member));

    let mut occupied = BTreeSet::new();
    let mut actors = Vec::with_capacity(requests.len());
    for (minimum_distance, group_index, _) in requests {
        let position = cells
            .iter()
            .rev()
            .copied()
            .find(|position| {
                !occupied.contains(position)
                    && manhattan_distance(*position, entrance) >= u32::from(minimum_distance)
            })
            .ok_or_else(|| {
                format!(
                    "Generated zone '{}' has no valid cell for population group {} at minimum entrance distance {}",
                    definition.destination.zone.id, group_index, minimum_distance
                )
            })?;
        occupied.insert(position);
        let group = &definition.destination.population[group_index];
        let mut ai = if features.pursuit_limits {
            group.ai()
        } else {
            group.ai().without_pursuit_limit()
        };
        if !features.pursuit_lifecycle {
            ai = ai.without_pursuit_lifecycle();
        }
        let attack = if features.physical_profiles {
            group.attack()
        } else {
            group.attack().without_melee_impact()
        };
        let attack = if features.preparation_disruption {
            attack
        } else {
            attack.without_preparation_disruption()
        };
        let mut actor = Actor::new(position, group.maximum_integrity())
            .map_err(|error| error.to_string())?
            .with_attack(attack)
            .with_ai(ai)
            .with_tags(group.tags().iter().cloned());
        if features.player_relations {
            actor = actor.with_player_relation(group.player_relation());
        }
        if features.primary_attributes
            && let Some(attributes) = group.primary_attributes()
        {
            actor = actor.with_primary_attributes(attributes);
        }
        if features.physical_profiles
            && let Some(body) = group.body_profile()
        {
            actor = actor.with_body_profile(body);
        }
        if features.physical_profiles && !group.body_components().is_empty() {
            actor = actor.with_body_components(group.body_components().iter().cloned());
        }
        if features.electronic_systems
            && let Some(profile) = group.electronic_system()
        {
            actor = actor.with_electronic_system(profile);
        }
        if let Some(reward) = group.defeat_reward() {
            actor = actor.with_defeat_reward(reward);
        }
        actors.push(actor);
    }
    Ok(actors)
}

fn manhattan_distance(left: GridPos, right: GridPos) -> u32 {
    left.x.abs_diff(right.x) + left.y.abs_diff(right.y)
}
