//! Temporary Terminal adapter for local maps addressed by the regional atlas.
//! Coordinate identity, biome choice and terrain generation all remain in the
//! headless library; only semantic terrain-to-glyph decoration lives here.
use crate::test_sector::{Decor, SectorDecor};
use project_rl::content::{
    ContentId, RegionCoord, RegionDescriptor, RegionDirection, RegionTerrain,
    RegionVerticalDirection, RegionalWorldDefinition,
};
use project_rl::entity::Actor;
use project_rl::facility::{
    FacilityBlueprint, InstallationBlueprint, InstallationCapability, SecurityAlarmProfile,
    SecurityAlarmResponse,
};
use project_rl::game::{GroundLootBlueprint, ThreatSourceBlueprint, ZoneBlueprint, ZoneInfo};
use project_rl::loot::LootCatalog;
use project_rl::progression::DefeatReward;
use project_rl::world::generation::{
    GeneratedRegionalSite, GeneratedRegionalSiteTerminal, RegionSiteEntranceKind,
    RegionalLandmarkKind, RegionalLootRequest, RegionalMapGenerator, RegionalPopulationFeatures,
    generate_regional_destructibles, generate_regional_encounters, generate_regional_landmarks,
    generate_regional_loot, generate_regional_population, generate_regional_site_terminals,
    generate_regional_sites, vertical_passage,
};
use project_rl::world::{Direction, GridPos};
use std::collections::BTreeSet;

pub struct GeneratedRegionalDestination {
    pub blueprint: ZoneBlueprint,
    pub facility: Option<FacilityBlueprint>,
    pub decor: SectorDecor,
    pub passages: [(RegionDirection, GridPos); 4],
    pub vertical_passages: Vec<(RegionVerticalDirection, GridPos)>,
}

#[derive(Clone, Copy, Debug)]
pub struct RegionalGenerationFeatures {
    pub vertical_travel: bool,
    pub population: bool,
    pub encounters: bool,
    pub pursuit_lifecycle: bool,
    pub primary_attributes: bool,
    pub physical_profiles: bool,
    pub loot: bool,
    pub landmarks: bool,
    pub sites: bool,
    pub site_interactions: bool,
    pub site_security: bool,
    pub site_terminals: bool,
    pub reinforcement_investigation: bool,
    pub site_navigation_signals: bool,
    pub threat_renewal: bool,
    pub destructibles: bool,
    pub electronic_systems: bool,
    pub player_relations: bool,
}

pub fn zone_id(
    world: &RegionalWorldDefinition,
    coordinate: RegionCoord,
) -> Result<ContentId, String> {
    ContentId::new(
        world.id().namespace().clone(),
        format!(
            "{}_region_x{}_y{}_d{}",
            world.id().name(),
            signed_component(coordinate.x),
            signed_component(coordinate.y),
            coordinate.depth
        ),
    )
    .map_err(|error| error.to_string())
}

pub fn zone_info(
    world: &RegionalWorldDefinition,
    descriptor: &RegionDescriptor,
) -> Result<ZoneInfo, String> {
    Ok(ZoneInfo {
        id: zone_id(world, descriptor.coordinate)?,
        name: format!(
            "{} · région {:+}, {:+}",
            readable_id(&descriptor.biome),
            descriptor.coordinate.x,
            descriptor.coordinate.y
        ),
        kind: descriptor.biome.clone(),
        depth: descriptor.coordinate.depth,
    })
}

pub fn generate(
    world: &RegionalWorldDefinition,
    descriptor: &RegionDescriptor,
    info: ZoneInfo,
    entrance: GridPos,
    loot_catalog: Option<&LootCatalog>,
    features: RegionalGenerationFeatures,
) -> Result<GeneratedRegionalDestination, String> {
    if info != zone_info(world, descriptor)? {
        return Err("Regional zone metadata does not match its atlas coordinate".into());
    }
    let biome = world
        .biome(&descriptor.biome)
        .ok_or_else(|| format!("Unknown regional biome '{}'", descriptor.biome))?;
    let mut generated = RegionalMapGenerator::new(world.local_map_size(), biome.terrain())
        .generate(descriptor.seed)
        .map_err(|error| error.to_string())?;
    let passages = [
        (RegionDirection::North, generated.passage(Direction::North)),
        (RegionDirection::East, generated.passage(Direction::East)),
        (RegionDirection::South, generated.passage(Direction::South)),
        (RegionDirection::West, generated.passage(Direction::West)),
    ];
    let vertical_passages = if features.vertical_travel {
        world
            .vertical_neighbors(descriptor.coordinate)
            .into_iter()
            .map(|(direction, _)| {
                (
                    direction,
                    vertical_passage(world.local_map_size(), direction),
                )
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    if !passages.iter().any(|(_, passage)| *passage == entrance)
        && !vertical_passages
            .iter()
            .any(|(_, passage)| *passage == entrance)
    {
        return Err("Regional entrance is not a declared passage".into());
    }
    let passage_positions = passages
        .iter()
        .map(|(_, passage)| *passage)
        .chain(vertical_passages.iter().map(|(_, passage)| *passage))
        .collect::<Vec<_>>();
    let mut actors = if features.population {
        generate_regional_population(
            generated.map(),
            &passage_positions,
            biome.population(),
            descriptor.seed,
            RegionalPopulationFeatures {
                pursuit_lifecycle: features.pursuit_lifecycle,
                primary_attributes: features.primary_attributes,
                physical_profiles: features.physical_profiles,
                electronic_systems: features.electronic_systems,
                player_relations: features.player_relations,
            },
        )
        .map_err(|error| error.to_string())?
    } else {
        Vec::new()
    };
    let mut actor_positions: BTreeSet<_> = actors.iter().map(Actor::position).collect();
    let mut site_interaction_positions = BTreeSet::new();
    let mut generated_sites = Vec::new();
    let landmarks = if features.landmarks {
        if features.sites && !biome.sites().is_empty() {
            let layout = generate_regional_sites(
                generated.map(),
                &passage_positions,
                &actor_positions,
                biome.landmarks(),
                biome.sites(),
                features.site_interactions,
                descriptor.seed,
            )
            .map_err(|error| error.to_string())?;
            generated
                .apply_site_overlay(
                    layout.terrain(),
                    layout.interactions(),
                    &layout
                        .landmarks()
                        .iter()
                        .map(|landmark| landmark.position)
                        .collect::<Vec<_>>(),
                )
                .map_err(|error| error.to_string())?;
            let (landmarks, _, interactions, sites) = layout.into_parts();
            site_interaction_positions.extend(interactions.into_keys());
            generated_sites = sites;
            landmarks
        } else {
            generate_regional_landmarks(
                generated.map(),
                &passage_positions,
                &actor_positions,
                biome.landmarks(),
                descriptor.seed,
            )
            .map_err(|error| error.to_string())?
        }
    } else {
        Vec::new()
    };
    let cache_positions: Vec<_> = landmarks
        .iter()
        .filter_map(|landmark| {
            (landmark.kind == RegionalLandmarkKind::SupplyCache).then_some(landmark.position)
        })
        .collect();
    let landmark_positions: BTreeSet<_> =
        landmarks.iter().map(|landmark| landmark.position).collect();
    let secured_sites: Vec<_> = if features.site_security && biome.site_security().is_some() {
        generated_sites
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, site)| site.entrance_kind == RegionSiteEntranceKind::LockedConsole)
            .collect()
    } else {
        Vec::new()
    };
    let site_terminals = if features.site_terminals {
        biome
            .site_terminals()
            .map(|profile| {
                generate_regional_site_terminals(&generated_sites, profile, descriptor.seed)
            })
            .unwrap_or_default()
    } else {
        Vec::new()
    };
    let security_installation_positions: BTreeSet<_> = secured_sites
        .iter()
        .flat_map(|(_, site)| site_security_positions(*site))
        .collect();
    let terminal_positions: BTreeSet<_> = site_terminals
        .iter()
        .map(|terminal| terminal.position)
        .collect();
    let reserved_site_positions: BTreeSet<_> = landmark_positions
        .union(&site_interaction_positions)
        .copied()
        .chain(security_installation_positions.iter().copied())
        .chain(terminal_positions.iter().copied())
        .collect();
    if features.encounters {
        let reserved: BTreeSet<_> = actor_positions
            .union(&reserved_site_positions)
            .copied()
            .collect();
        let encounter_anchors = landmarks
            .iter()
            .map(|landmark| landmark.position)
            .collect::<Vec<_>>();
        let generated_encounters = generate_regional_encounters(
            generated.map(),
            &passage_positions,
            &encounter_anchors,
            &reserved,
            biome.encounters(),
            descriptor.seed,
            RegionalPopulationFeatures {
                pursuit_lifecycle: features.pursuit_lifecycle,
                primary_attributes: features.primary_attributes,
                physical_profiles: features.physical_profiles,
                electronic_systems: features.electronic_systems,
                player_relations: features.player_relations,
            },
        )
        .map_err(|error| error.to_string())?;
        actor_positions.extend(generated_encounters.iter().map(Actor::position));
        actors.extend(generated_encounters);
    }
    if features.destructibles
        && let Some(profile) = biome.destructibles()
    {
        let reserved: BTreeSet<_> = actor_positions
            .union(&reserved_site_positions)
            .copied()
            .collect();
        let generated_destructibles = generate_regional_destructibles(
            generated.map(),
            &passage_positions,
            &reserved,
            profile,
            descriptor.seed,
        )
        .map_err(|error| error.to_string())?;
        actor_positions.extend(generated_destructibles.iter().map(Actor::position));
        actors.extend(generated_destructibles);
    }
    let reserved_for_loot: BTreeSet<_> = actor_positions
        .union(&reserved_site_positions)
        .copied()
        .collect();
    let generated_loot = if features.loot {
        match (biome.loot(), loot_catalog) {
            (Some(profile), Some(catalog)) => generate_regional_loot(RegionalLootRequest {
                map: generated.map(),
                passages: &passage_positions,
                reserved: &reserved_for_loot,
                cache_positions: &cache_positions,
                profile,
                catalog,
                biome: &descriptor.biome,
                depth: descriptor.coordinate.depth,
                region_seed: descriptor.seed,
            })
            .map_err(|error| error.to_string())?,
            (Some(_), None) => return Err("Regional loot catalog is missing".into()),
            (None, _) => Vec::new(),
        }
    } else {
        Vec::new()
    };
    let secured_cache_positions: BTreeSet<_> =
        secured_sites.iter().map(|(_, site)| site.cache).collect();
    let security_owner = biome.site_security().map(|profile| profile.owner().clone());
    let loot: Vec<_> = generated_loot
        .into_iter()
        .map(|drop| {
            let mut loot = GroundLootBlueprint::from(drop);
            if secured_cache_positions.contains(&loot.position())
                && let Some(owner) = security_owner.as_ref()
            {
                loot = loot.with_owner(owner.clone());
            }
            loot
        })
        .collect();
    let threat_sources = if features.threat_renewal {
        if let Some(profile) = biome.threats() {
            landmarks
                .iter()
                .filter(|landmark| landmark.kind == RegionalLandmarkKind::ThreatCamp)
                .map(|landmark| {
                    let mut actor = Actor::new(landmark.position, profile.maximum_integrity())
                        .expect("regional threat validation rejects zero integrity")
                        .with_attack(if features.physical_profiles {
                            profile.attack()
                        } else {
                            profile.attack().without_melee_impact()
                        })
                        .with_ai(profile.ai())
                        .with_defeat_reward(DefeatReward::summoned(0, 0));
                    if features.player_relations {
                        actor = actor.with_player_relation(profile.player_relation());
                    }
                    if features.primary_attributes
                        && let Some(attributes) = profile.primary_attributes()
                    {
                        actor = actor.with_primary_attributes(attributes);
                    }
                    if features.physical_profiles
                        && let Some(body) = profile.body_profile()
                    {
                        actor = actor.with_body_profile(body);
                    }
                    if features.physical_profiles && !profile.body_components().is_empty() {
                        actor =
                            actor.with_body_components(profile.body_components().iter().cloned());
                    }
                    if features.electronic_systems
                        && let Some(electronic_system) = profile.electronic_system()
                    {
                        actor = actor.with_electronic_system(electronic_system);
                    }
                    ThreatSourceBlueprint {
                        position: landmark.position,
                        interval_turns: std::num::NonZeroU16::new(profile.interval_turns())
                            .expect("validated threat interval is positive"),
                        maximum_active: std::num::NonZeroU16::new(profile.maximum_active())
                            .expect("validated active threat limit is positive"),
                        maximum_total: std::num::NonZeroU16::new(profile.maximum_total())
                            .expect("validated total threat limit is positive"),
                        actor,
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };
    let facility = build_site_facility(SiteFacilityRequest {
        zone: &info.id,
        security: if features.site_security {
            biome.site_security()
        } else {
            None
        },
        secured_sites: &secured_sites,
        terminals: &site_terminals,
        sites: &generated_sites,
        loot: &loot,
        reinforcement_investigation: features.reinforcement_investigation,
        navigation_signals: features.site_navigation_signals,
    })?;
    let (map, terrain, _) = generated.into_parts();
    let mut decor = SectorDecor {
        fallback_name: Some(format!("Région · {}", readable_id(&descriptor.biome))),
        ..SectorDecor::default()
    };
    decor.cells.extend(
        terrain
            .into_iter()
            .map(|(position, terrain)| (position, decor_for(terrain))),
    );
    for position in site_interaction_positions {
        let runtime = map
            .tile(position)
            .expect("generated site interaction remains inside the map")
            .terrain;
        let interaction_decor = decor.at(position, runtime);
        decor.cells.insert(position, interaction_decor);
    }
    for (_, passage) in passages {
        decor.cells.insert(passage, Decor::Passage);
    }
    for (direction, passage) in &vertical_passages {
        decor.cells.insert(
            *passage,
            match direction {
                RegionVerticalDirection::Up => Decor::Ascent,
                RegionVerticalDirection::Down => Decor::Descent,
            },
        );
    }
    for landmark in landmarks {
        decor.cells.insert(
            landmark.position,
            match landmark.kind {
                RegionalLandmarkKind::SupplyCache => Decor::SupplyCache,
                RegionalLandmarkKind::ThreatCamp if features.threat_renewal => Decor::ThreatCamp,
                RegionalLandmarkKind::ThreatCamp => Decor::ThreatCampDisabled,
            },
        );
    }
    if features.site_security {
        for (_, site) in &secured_sites {
            let [sensor, actuator] = site_security_positions(*site);
            decor.cells.insert(sensor, Decor::SensorOnline);
            decor.cells.insert(actuator, Decor::ActuatorOnline);
        }
    }
    for terminal in &site_terminals {
        decor
            .cells
            .insert(terminal.position, Decor::DataTerminalOnline);
    }
    Ok(GeneratedRegionalDestination {
        blueprint: ZoneBlueprint {
            info,
            map,
            entrance,
            seed: descriptor.seed,
            actors,
            loot,
            threat_sources,
        },
        facility,
        decor,
        passages,
        vertical_passages,
    })
}

fn site_security_positions(site: GeneratedRegionalSite) -> [GridPos; 2] {
    [
        GridPos::new(site.camp.x, site.camp.y - 2),
        GridPos::new(site.camp.x + 2, site.camp.y - 2),
    ]
}

struct SiteFacilityRequest<'a> {
    zone: &'a ContentId,
    security: Option<&'a project_rl::content::RegionSiteSecurityProfile>,
    secured_sites: &'a [(usize, GeneratedRegionalSite)],
    terminals: &'a [GeneratedRegionalSiteTerminal],
    sites: &'a [GeneratedRegionalSite],
    loot: &'a [GroundLootBlueprint],
    reinforcement_investigation: bool,
    navigation_signals: bool,
}

fn build_site_facility(
    request: SiteFacilityRequest<'_>,
) -> Result<Option<FacilityBlueprint>, String> {
    let SiteFacilityRequest {
        zone,
        security,
        secured_sites,
        terminals,
        sites,
        loot,
        reinforcement_investigation,
        navigation_signals,
    } = request;
    let protected_caches: BTreeSet<_> = security
        .map(|profile| {
            loot.iter()
                .filter(|drop| drop.owner() == Some(profile.owner()))
                .map(GroundLootBlueprint::position)
                .collect()
        })
        .unwrap_or_default();
    let secured: BTreeSet<_> = secured_sites
        .iter()
        .filter(|(_, site)| protected_caches.contains(&site.cache))
        .map(|(index, _)| *index)
        .collect();
    let terminal_by_site: std::collections::BTreeMap<_, _> = terminals
        .iter()
        .map(|terminal| (terminal.site_index, terminal))
        .collect();
    let has_security = !secured.is_empty();
    if secured.is_empty() && terminal_by_site.is_empty() {
        return Ok(None);
    }
    let mut installations = Vec::with_capacity((secured.len() + terminal_by_site.len()) * 3);
    let mut first_depot = None;
    for (index, site) in sites.iter().copied().enumerate() {
        let is_secured = secured.contains(&index);
        let terminal = terminal_by_site.get(&index).copied();
        if !is_secured && terminal.is_none() {
            continue;
        }
        let depot = generated_installation_id(zone, index, "depot")?;
        first_depot.get_or_insert_with(|| depot.clone());
        installations.push(InstallationBlueprint {
            id: depot,
            position: site.cache,
            maximum_integrity: 10,
            integrity: 10,
            capabilities: vec![InstallationCapability::Storage],
            dependencies: vec![],
            security_alarm_profile: None,
        });
        if is_secured {
            let profile = security.expect("secured sites require their security profile");
            let [sensor_position, actuator_position] = site_security_positions(site);
            let sensor = generated_installation_id(zone, index, "sensor")?;
            let actuator = generated_installation_id(zone, index, "actuator")?;
            let mut sensor_capabilities = vec![InstallationCapability::SecuritySensor];
            if navigation_signals && let Some(range) = profile.navigation_signal_range() {
                sensor_capabilities.push(InstallationCapability::NavigationBeacon { range });
            }
            installations.push(InstallationBlueprint {
                id: sensor,
                position: sensor_position,
                maximum_integrity: 10,
                integrity: 10,
                capabilities: sensor_capabilities,
                dependencies: vec![],
                security_alarm_profile: Some(
                    SecurityAlarmProfile::new(
                        profile.sensor_radius(),
                        profile.distance_metric(),
                        profile.block_closed_corners(),
                        profile.alarm_duration_turns(),
                    )
                    .and_then(|alarm| {
                        let response = if reinforcement_investigation {
                            SecurityAlarmResponse::CallInvestigatingReinforcements {
                                source: site.camp,
                                delay_turns: profile.reinforcement_delay_turns(),
                            }
                        } else {
                            SecurityAlarmResponse::CallReinforcements {
                                source: site.camp,
                                delay_turns: profile.reinforcement_delay_turns(),
                            }
                        };
                        alarm.with_responses(vec![response])
                    })
                    .map_err(|error| error.to_string())?,
                ),
            });
            installations.push(InstallationBlueprint {
                id: actuator,
                position: actuator_position,
                maximum_integrity: 10,
                integrity: 10,
                capabilities: vec![InstallationCapability::DoorActuator {
                    door: site.entrance,
                }],
                dependencies: vec![],
                security_alarm_profile: None,
            });
        }
        if let Some(terminal) = terminal {
            installations.push(InstallationBlueprint {
                id: generated_installation_id(zone, index, "terminal")?,
                position: terminal.position,
                maximum_integrity: 10,
                integrity: 10,
                capabilities: vec![InstallationCapability::DataTerminal {
                    record: terminal.record.clone(),
                }],
                dependencies: vec![],
                security_alarm_profile: None,
            });
        }
    }
    Ok(Some(FacilityBlueprint {
        installations,
        depot: first_depot.expect("an interactive site provides a physical cache"),
        workers: vec![],
        repair_orders: vec![],
        maximum_path_search: 4096,
        owner: security
            .filter(|_| has_security)
            .map(|profile| profile.owner().clone()),
    }))
}

fn generated_installation_id(
    zone: &ContentId,
    site_index: usize,
    suffix: &str,
) -> Result<ContentId, String> {
    ContentId::new(
        zone.namespace().clone(),
        format!("{}_site_{site_index}_{suffix}", zone.name()),
    )
    .map_err(|error| error.to_string())
}

fn signed_component(value: i32) -> String {
    if value < 0 {
        format!("n{}", value.unsigned_abs())
    } else {
        format!("p{value}")
    }
}

fn readable_id(id: &ContentId) -> String {
    id.name().replace(['_', '-'], " ").to_uppercase()
}

const fn decor_for(terrain: RegionTerrain) -> Decor {
    match terrain {
        RegionTerrain::Gravel => Decor::Gravel,
        RegionTerrain::Grass => Decor::Grass,
        RegionTerrain::Scrub => Decor::Scrub,
        RegionTerrain::Mud => Decor::Mud,
        RegionTerrain::ShallowWater => Decor::ShallowWater,
        RegionTerrain::DeepWater => Decor::DeepWater,
        RegionTerrain::Tree => Decor::Tree,
        RegionTerrain::Boulder => Decor::Boulder,
        RegionTerrain::RuinFloor => Decor::RuinFloor,
        RegionTerrain::RuinWall => Decor::RuinWall,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use project_rl::content::{
        ContentLoader, RegionBiomeRule, RegionBounds, RegionMapSize, RegionTerrainProfile,
        RegionTerrainRule,
    };
    use project_rl::entity::ActorRegistry;
    use project_rl::facility::FacilityState;

    fn definition() -> RegionalWorldDefinition {
        RegionalWorldDefinition::new(
            "test:world".parse().unwrap(),
            RegionBounds::new(-4, 4, -4, 4, 0).unwrap(),
            2,
            RegionMapSize::new(64, 48).unwrap(),
            vec![
                RegionBiomeRule::new(
                    "test:wilds".parse().unwrap(),
                    1,
                    0,
                    Some(0),
                    RegionTerrainProfile::new(
                        RegionTerrain::Grass,
                        8,
                        2,
                        6,
                        vec![RegionTerrainRule::new(RegionTerrain::Tree, 1).unwrap()],
                    )
                    .unwrap(),
                )
                .unwrap(),
            ],
        )
        .unwrap()
    }

    #[test]
    fn coordinate_ids_are_stable_and_generated_decor_matches_runtime_terrain() {
        let world = definition();
        let coordinate = RegionCoord::new(-2, 3, 0);
        let descriptor = world.region(17, coordinate).unwrap();
        let info = zone_info(&world, &descriptor).unwrap();
        assert_eq!(info.id.as_str(), "test:world_region_xn2_yp3_d0");
        let entrance = project_rl::world::generation::cardinal_passage(
            world.local_map_size(),
            Direction::West,
        );
        let generated = generate(
            &world,
            &descriptor,
            info,
            entrance,
            None,
            RegionalGenerationFeatures {
                vertical_travel: true,
                population: true,
                encounters: false,
                pursuit_lifecycle: true,
                primary_attributes: true,
                physical_profiles: true,
                loot: false,
                landmarks: false,
                sites: false,
                site_interactions: false,
                site_security: false,
                site_terminals: false,
                reinforcement_investigation: false,
                site_navigation_signals: false,
                threat_renewal: false,
                destructibles: false,
                electronic_systems: true,
                player_relations: true,
            },
        )
        .unwrap();

        assert_eq!(generated.blueprint.entrance, entrance);
        assert_eq!(generated.passages.len(), 4);
        for (position, kind) in &generated.decor.cells {
            assert_eq!(
                kind.blocks(),
                generated
                    .blueprint
                    .map
                    .tile(*position)
                    .unwrap()
                    .terrain
                    .blocks_movement()
            );
        }
    }

    #[test]
    fn core_surface_regions_keep_their_biome_presence_floor() {
        let content_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content");
        let loaded = ContentLoader::load(&[content_root], &semver::Version::new(0, 1, 0)).unwrap();
        let world = loaded
            .regional_worlds()
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        let entrance = project_rl::world::generation::cardinal_passage(
            world.local_map_size(),
            Direction::West,
        );
        let mut closed_doors = 0;
        let mut locked_doors = 0;
        let mut consoles = 0;
        let mut secured_regions = 0;
        let mut terminal_regions = 0;
        let mut independent_terminal_checked = false;
        let mut legacy_response_checked = false;

        for y in -4..=3 {
            for x in -4..=3 {
                let descriptor = world.region(17, RegionCoord::new(x, y, 0)).unwrap();
                let expected_minimum = match descriptor.biome.as_str() {
                    "core:human_habitat" => 4,
                    "core:surface_wilds" => 6,
                    unexpected => panic!("unexpected surface biome {unexpected}"),
                };
                let generated = generate(
                    world,
                    &descriptor,
                    zone_info(world, &descriptor).unwrap(),
                    entrance,
                    Some(loaded.loot()),
                    RegionalGenerationFeatures {
                        vertical_travel: true,
                        population: true,
                        encounters: true,
                        pursuit_lifecycle: true,
                        primary_attributes: true,
                        physical_profiles: true,
                        loot: true,
                        landmarks: true,
                        sites: true,
                        site_interactions: true,
                        site_security: true,
                        site_terminals: true,
                        reinforcement_investigation: true,
                        site_navigation_signals: true,
                        threat_renewal: true,
                        destructibles: true,
                        electronic_systems: true,
                        player_relations: true,
                    },
                )
                .unwrap();
                assert!(
                    generated.blueprint.actors.len() >= expected_minimum,
                    "{} at ({x}, {y}) generated only {} actors",
                    descriptor.biome,
                    generated.blueprint.actors.len()
                );
                if let Some(facility) = &generated.facility {
                    FacilityState::instantiate(
                        facility.clone(),
                        &mut generated.blueprint.map.clone(),
                        &ActorRegistry::default(),
                    )
                    .unwrap();
                    let has_terminal = facility.installations.iter().any(|installation| {
                        installation.capabilities.iter().any(|capability| {
                            matches!(capability, InstallationCapability::DataTerminal { .. })
                        })
                    });
                    assert!(
                        has_terminal,
                        "every core surface region requests one terminal"
                    );
                    terminal_regions += 1;
                    if facility.owner.is_some() {
                        secured_regions += 1;
                        assert_eq!(
                            facility.owner.as_ref(),
                            Some(&"core:system_security".parse().unwrap())
                        );
                        assert!(generated.blueprint.loot.iter().any(|loot| {
                            loot.owner()
                                .is_some_and(|owner| owner == facility.owner.as_ref().unwrap())
                        }));
                        assert!(facility.installations.iter().any(|installation| {
                            installation
                                .capabilities
                                .contains(&InstallationCapability::NavigationBeacon { range: 96 })
                        }));
                        for response in facility
                            .installations
                            .iter()
                            .filter_map(|installation| installation.security_alarm_profile.as_ref())
                            .flat_map(SecurityAlarmProfile::responses)
                        {
                            let SecurityAlarmResponse::CallInvestigatingReinforcements {
                                source,
                                delay_turns,
                            } = response
                            else {
                                panic!("regional security generated an unrelated alarm response")
                            };
                            assert_eq!(*delay_turns, 3);
                            assert!(
                                generated
                                    .blueprint
                                    .threat_sources
                                    .iter()
                                    .any(|threat| threat.position == *source)
                            );
                        }
                    }
                    if facility.owner.is_some() && !legacy_response_checked {
                        let legacy = generate(
                            world,
                            &descriptor,
                            zone_info(world, &descriptor).unwrap(),
                            entrance,
                            Some(loaded.loot()),
                            RegionalGenerationFeatures {
                                vertical_travel: true,
                                population: true,
                                encounters: true,
                                pursuit_lifecycle: true,
                                primary_attributes: true,
                                physical_profiles: true,
                                loot: true,
                                landmarks: true,
                                sites: true,
                                site_interactions: true,
                                site_security: true,
                                site_terminals: false,
                                reinforcement_investigation: false,
                                site_navigation_signals: false,
                                threat_renewal: true,
                                destructibles: false,
                                electronic_systems: true,
                                player_relations: true,
                            },
                        )
                        .unwrap();
                        let responses = legacy
                            .facility
                            .as_ref()
                            .unwrap()
                            .installations
                            .iter()
                            .filter_map(|installation| installation.security_alarm_profile.as_ref())
                            .flat_map(SecurityAlarmProfile::responses)
                            .collect::<Vec<_>>();
                        assert!(responses.iter().all(|response| matches!(
                            response,
                            SecurityAlarmResponse::CallReinforcements { .. }
                        )));
                        assert!(legacy.facility.as_ref().unwrap().installations.iter().all(
                            |installation| installation.capabilities.iter().all(|capability| {
                                !matches!(
                                    capability,
                                    InstallationCapability::NavigationBeacon { .. }
                                )
                            })
                        ));
                        legacy_response_checked = true;
                    }
                    if !independent_terminal_checked {
                        let terminal_only = generate(
                            world,
                            &descriptor,
                            zone_info(world, &descriptor).unwrap(),
                            entrance,
                            Some(loaded.loot()),
                            RegionalGenerationFeatures {
                                vertical_travel: true,
                                population: true,
                                encounters: true,
                                pursuit_lifecycle: true,
                                primary_attributes: true,
                                physical_profiles: true,
                                loot: true,
                                landmarks: true,
                                sites: true,
                                site_interactions: true,
                                site_security: false,
                                site_terminals: true,
                                reinforcement_investigation: true,
                                site_navigation_signals: true,
                                threat_renewal: true,
                                destructibles: false,
                                electronic_systems: true,
                                player_relations: true,
                            },
                        )
                        .unwrap();
                        let terminal_only = terminal_only.facility.unwrap();
                        assert_eq!(terminal_only.owner, None);
                        assert!(terminal_only.installations.iter().any(|installation| {
                            installation.capabilities.iter().any(|capability| {
                                matches!(capability, InstallationCapability::DataTerminal { .. })
                            })
                        }));
                        independent_terminal_checked = true;
                    }
                }
                for row in 0..generated.blueprint.map.height() as i32 {
                    for column in 0..generated.blueprint.map.width() as i32 {
                        match generated
                            .blueprint
                            .map
                            .tile(GridPos::new(column, row))
                            .unwrap()
                            .terrain
                        {
                            project_rl::world::Terrain::Door(
                                project_rl::world::DoorState::Closed,
                            ) => closed_doors += 1,
                            project_rl::world::Terrain::Door(
                                project_rl::world::DoorState::Locked,
                            ) => locked_doors += 1,
                            project_rl::world::Terrain::ControlPanel { .. } => consoles += 1,
                            _ => {}
                        }
                    }
                }
            }
        }
        assert!(
            closed_doors > 0,
            "core regions never generated a closed site"
        );
        assert!(
            locked_doors > 0,
            "core regions never generated a locked site"
        );
        assert_eq!(consoles, locked_doors);
        assert!(
            secured_regions > 0,
            "core regions never generated site security"
        );
        assert_eq!(terminal_regions, 64);
        assert!(independent_terminal_checked);
        assert!(legacy_response_checked);
    }

    #[test]
    fn core_descent_is_stable_bidirectional_and_leads_to_a_playable_layer() {
        let content_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content");
        let loaded = ContentLoader::load(&[content_root], &semver::Version::new(0, 1, 0)).unwrap();
        let world = loaded
            .regional_worlds()
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        let features = RegionalGenerationFeatures {
            vertical_travel: true,
            population: true,
            encounters: true,
            pursuit_lifecycle: true,
            primary_attributes: true,
            physical_profiles: true,
            loot: true,
            landmarks: true,
            sites: true,
            site_interactions: true,
            site_security: true,
            site_terminals: true,
            reinforcement_investigation: true,
            site_navigation_signals: true,
            threat_renewal: true,
            destructibles: true,
            electronic_systems: true,
            player_relations: true,
        };
        let upper_coordinate = RegionCoord::new(-1, 0, 0);
        let lower_coordinate = RegionCoord::new(-1, 0, 1);
        assert_eq!(
            world.vertical_neighbor(upper_coordinate, RegionVerticalDirection::Down),
            Some(lower_coordinate)
        );

        let upper_descriptor = world.region(17, upper_coordinate).unwrap();
        let upper = generate(
            world,
            &upper_descriptor,
            zone_info(world, &upper_descriptor).unwrap(),
            project_rl::world::generation::cardinal_passage(
                world.local_map_size(),
                Direction::East,
            ),
            Some(loaded.loot()),
            features,
        )
        .unwrap();
        let descent = vertical_passage(world.local_map_size(), RegionVerticalDirection::Down);
        assert_eq!(
            upper.vertical_passages,
            vec![(RegionVerticalDirection::Down, descent)]
        );
        assert_eq!(upper.decor.cells.get(&descent), Some(&Decor::Descent));
        assert!(upper.blueprint.map.is_walkable(descent));

        let lower_descriptor = world.region(17, lower_coordinate).unwrap();
        let ascent = vertical_passage(world.local_map_size(), RegionVerticalDirection::Up);
        let lower = generate(
            world,
            &lower_descriptor,
            zone_info(world, &lower_descriptor).unwrap(),
            ascent,
            Some(loaded.loot()),
            features,
        )
        .unwrap();
        assert_eq!(
            lower.vertical_passages,
            vec![(RegionVerticalDirection::Up, ascent)]
        );
        assert_eq!(lower.decor.cells.get(&ascent), Some(&Decor::Ascent));
        assert!(lower.blueprint.map.is_walkable(ascent));
        assert!(!lower.blueprint.actors.is_empty());
        assert!(!lower.blueprint.loot.is_empty());
        assert!(
            (2..=5).contains(
                &lower
                    .blueprint
                    .actors
                    .iter()
                    .filter(|actor| actor.destruction_effect().is_some())
                    .count()
            )
        );

        for seed in 0..64 {
            let descriptor = world.region(seed, lower_coordinate).unwrap();
            let generated = generate(
                world,
                &descriptor,
                zone_info(world, &descriptor).unwrap(),
                ascent,
                Some(loaded.loot()),
                features,
            )
            .unwrap_or_else(|error| panic!("depth generation seed {seed}: {error}"));
            let reserved = [
                generated.blueprint.entrance,
                project_rl::world::generation::cardinal_passage(
                    world.local_map_size(),
                    Direction::North,
                ),
                project_rl::world::generation::cardinal_passage(
                    world.local_map_size(),
                    Direction::East,
                ),
                project_rl::world::generation::cardinal_passage(
                    world.local_map_size(),
                    Direction::South,
                ),
                project_rl::world::generation::cardinal_passage(
                    world.local_map_size(),
                    Direction::West,
                ),
            ];
            assert!(generated.blueprint.actors.iter().all(|actor| {
                !reserved.contains(&actor.position())
                    && generated.blueprint.map.is_walkable(actor.position())
            }));
            assert!(generated.blueprint.loot.iter().all(|drop| {
                !reserved.contains(&drop.position())
                    && generated.blueprint.map.is_walkable(drop.position())
            }));
            assert!(!generated.blueprint.actors.is_empty());
            assert!(
                (2..=5).contains(
                    &generated
                        .blueprint
                        .actors
                        .iter()
                        .filter(|actor| actor.destruction_effect().is_some())
                        .count()
                )
            );
            assert!(generated.blueprint.loot.len() >= 3);
        }
    }
}
