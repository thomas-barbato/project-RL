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
use project_rl::world::generation::{MapValidationRules, validate_interactive_map};
use project_rl::world::{DistanceMetric, DoorState, GridPos, Terrain};
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

/// A guaranteed, occupied relay within the seeded industrial map. Reserving an
/// existing open patch preserves every generated corridor and reciprocal exit.
pub fn install_narrative_relay(
    generated: &mut GeneratedExpeditionDestination,
    narrative: &project_rl::content::NarrativeDefinition,
) -> Result<project_rl::facility::FacilityBlueprint, String> {
    use project_rl::facility::{FacilityBlueprint, InstallationBlueprint, InstallationCapability};
    let blueprint = &mut generated.blueprint;
    let occupied: BTreeSet<_> = blueprint
        .actors
        .iter()
        .map(Actor::position)
        .chain(blueprint.loot.iter().map(|loot| loot.position()))
        .chain(std::iter::once(blueprint.entrance))
        .collect();
    let mut candidates = Vec::new();
    for y in 2..blueprint.map.height() as i32 - 2 {
        for x in 2..blueprint.map.width() as i32 - 2 {
            let at = GridPos::new(x, y);
            let distance = (x - blueprint.entrance.x).abs() + (y - blueprint.entrance.y).abs();
            if distance < 14 {
                continue;
            }
            if (-1..=1).all(|dy| {
                (-2..=2).all(|dx| {
                    let p = GridPos::new(x + dx, y + dy);
                    blueprint.map.is_walkable(p) && !occupied.contains(&p)
                })
            }) {
                candidates.push((distance, at.y, at.x, at));
            }
        }
    }
    candidates.sort_by_key(|(distance, y, x, _)| (*distance, *y, *x));
    let at = candidates
        .first()
        .ok_or("Aucun emplacement accessible pour le relais")?
        .3;
    let rivet = GridPos::new(at.x - 1, at.y);
    let depot = GridPos::new(at.x + 1, at.y);
    // Both installations occupy real tiles. The untouched perimeter of the
    // reserved 5x3 patch keeps all former paths connected around them.
    for position in [at, depot] {
        blueprint
            .map
            .set_terrain(position, project_rl::world::Terrain::Wall)
            .map_err(|e| e.to_string())?;
    }
    blueprint.actors.push(
        Actor::new(rivet, 18)
            .map_err(|e| e.to_string())?
            .with_ai(AiProfile::idle())
            .with_tags([narrative.relay_character.clone()]),
    );
    generated.decor.cells.insert(at, Decor::DataTerminalOnline);
    generated.decor.cells.insert(depot, Decor::Depot);
    let depot_id: ContentId = "core:relay_depot".parse().unwrap();
    Ok(FacilityBlueprint {
        installations: vec![
            InstallationBlueprint {
                id: "core:relay_register".parse().unwrap(),
                position: at,
                maximum_integrity: 12,
                integrity: 12,
                capabilities: vec![InstallationCapability::DataTerminal {
                    record: narrative.investigation.record.clone(),
                }],
                dependencies: Vec::new(),
                security_alarm_profile: None,
            },
            InstallationBlueprint {
                id: depot_id.clone(),
                position: depot,
                maximum_integrity: 12,
                integrity: 12,
                capabilities: vec![InstallationCapability::Storage],
                dependencies: Vec::new(),
                security_alarm_profile: None,
            },
        ],
        depot: depot_id,
        workers: Vec::new(),
        repair_orders: Vec::new(),
        maximum_path_search: 512,
        owner: None,
    })
}

/// A bounded courtyard carved only into an already open generated room. The
/// service door is the ordinary entrance; the former road stays barricaded.
/// Every candidate is checked against the complete post-decoration map, so a
/// quest site cannot silently sever a procedural corridor or its return exit.
pub fn install_narrative_relay_with_service_route(
    generated: &mut GeneratedExpeditionDestination,
    narrative: &project_rl::content::NarrativeDefinition,
) -> Result<project_rl::facility::FacilityBlueprint, String> {
    use project_rl::facility::{FacilityBlueprint, InstallationBlueprint, InstallationCapability};
    let blueprint = &mut generated.blueprint;
    let occupied: BTreeSet<_> = blueprint
        .actors
        .iter()
        .map(Actor::position)
        .chain(blueprint.loot.iter().map(|loot| loot.position()))
        .chain(std::iter::once(blueprint.entrance))
        .collect();
    let mut candidates = Vec::new();
    for y in 3..blueprint.map.height() as i32 - 3 {
        for x in 4..blueprint.map.width() as i32 - 4 {
            let at = GridPos::new(x, y);
            let distance = (x - blueprint.entrance.x).abs() + (y - blueprint.entrance.y).abs();
            if distance < 14
                || !(-2..=2).all(|dy| {
                    (-3..=3).all(|dx| {
                        let position = GridPos::new(x + dx, y + dy);
                        blueprint.map.is_walkable(position)
                            && !blueprint.map.is_protected(position)
                            && !occupied.contains(&position)
                    })
                })
                || [GridPos::new(x - 4, y), GridPos::new(x, y + 3)]
                    .into_iter()
                    .any(|position| {
                        !blueprint.map.is_walkable(position) || occupied.contains(&position)
                    })
            {
                continue;
            }
            candidates.push((distance, y, x, at));
        }
    }
    candidates.sort_by_key(|(distance, y, x, _)| (*distance, *y, *x));
    let mut selected = None;
    for (_, _, _, at) in candidates {
        let mut trial = blueprint.map.clone();
        for dy in -2_i32..=2 {
            for dx in -3_i32..=3 {
                if dx.abs() == 3 || dy.abs() == 2 {
                    let position = GridPos::new(at.x + dx, at.y + dy);
                    let terrain = if dx == 0 && dy == 2 {
                        Terrain::Door(DoorState::Closed)
                    } else {
                        Terrain::Wall
                    };
                    trial
                        .set_terrain(position, terrain)
                        .map_err(|error| error.to_string())?;
                }
            }
        }
        // The barricade is narrative scenery until removing it has a quest outcome.
        trial
            .set_protected(GridPos::new(at.x - 3, at.y), true)
            .map_err(|error| error.to_string())?;
        for position in [
            at,
            GridPos::new(at.x + 1, at.y),
            GridPos::new(at.x - 2, at.y + 1),
        ] {
            trial
                .set_terrain(position, Terrain::Wall)
                .map_err(|error| error.to_string())?;
        }
        let rivet = GridPos::new(at.x - 1, at.y);
        let service_approach = GridPos::new(at.x, at.y + 3);
        let old_approach = GridPos::new(at.x - 4, at.y);
        if validate_interactive_map(
            &trial,
            blueprint.entrance,
            blueprint.entrance,
            &[rivet, service_approach, old_approach],
            MapValidationRules::default(),
        )
        .is_ok()
        {
            selected = Some((at, trial));
            break;
        }
    }
    let (at, map) = selected.ok_or("Aucun emplacement connecté pour la cour du relais")?;
    blueprint.map = map;
    let rivet = GridPos::new(at.x - 1, at.y);
    let depot = GridPos::new(at.x + 1, at.y);
    let service_plan = GridPos::new(at.x - 2, at.y + 1);
    for dy in -2_i32..=2 {
        for dx in -3_i32..=3 {
            let position = GridPos::new(at.x + dx, at.y + dy);
            let decor = if dx == -3 && dy == 0 {
                Decor::Barricade
            } else if dx.abs() == 3 || dy.abs() == 2 {
                Decor::RuinWall
            } else {
                Decor::RuinFloor
            };
            generated.decor.cells.insert(position, decor);
        }
    }
    generated.decor.cells.insert(at, Decor::DataTerminalOnline);
    generated.decor.cells.insert(depot, Decor::Depot);
    generated
        .decor
        .cells
        .insert(service_plan, Decor::ServicePlan);
    generated.decor.zones.push(crate::test_sector::Zone {
        name: "Cour du relais".to_owned(),
        bounds: [at.x - 3, at.y - 2, 7, 5],
    });
    blueprint.actors.push(
        Actor::new(rivet, 18)
            .map_err(|error| error.to_string())?
            .with_ai(AiProfile::idle())
            .with_tags([narrative.relay_character.clone()]),
    );
    let depot_id: ContentId = "core:relay_depot".parse().unwrap();
    Ok(FacilityBlueprint {
        installations: vec![
            InstallationBlueprint {
                id: "core:relay_register".parse().unwrap(),
                position: at,
                maximum_integrity: 12,
                integrity: 12,
                capabilities: vec![InstallationCapability::DataTerminal {
                    record: narrative.investigation.record.clone(),
                }],
                dependencies: Vec::new(),
                security_alarm_profile: None,
            },
            InstallationBlueprint {
                id: "core:relay_service_plan".parse().unwrap(),
                position: service_plan,
                maximum_integrity: 8,
                integrity: 8,
                capabilities: vec![InstallationCapability::DataTerminal {
                    record: "core:relay_service_route_verified".parse().unwrap(),
                }],
                dependencies: Vec::new(),
                security_alarm_profile: None,
            },
            InstallationBlueprint {
                id: depot_id.clone(),
                position: depot,
                maximum_integrity: 12,
                integrity: 12,
                capabilities: vec![InstallationCapability::Storage],
                dependencies: Vec::new(),
                security_alarm_profile: None,
            },
        ],
        depot: depot_id,
        workers: Vec::new(),
        repair_orders: Vec::new(),
        maximum_path_search: 512,
        owner: None,
    })
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
