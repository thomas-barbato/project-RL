//! Temporary Terminal adapter for local maps addressed by the regional atlas.
//! Coordinate identity, biome choice and terrain generation all remain in the
//! headless library; only semantic terrain-to-glyph decoration lives here.
use crate::test_sector::{Decor, SectorDecor, Zone};
use project_rl::ai::AiProfile;
use project_rl::content::{
    ContentId, RegionCityDefinition, RegionCityLayout, RegionCoord, RegionDescriptor,
    RegionDirection, RegionTerrain, RegionVerticalDirection, RegionalWorldDefinition,
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
    GeneratedRegionalSite, GeneratedRegionalSiteTerminal, MapValidationRules,
    RegionSiteEntranceKind, RegionalCityFeature, RegionalLandmarkKind, RegionalLootRequest,
    RegionalMapGenerator, RegionalPopulationFeatures, generate_regional_city,
    generate_regional_destructibles, generate_regional_encounters, generate_regional_landmarks,
    generate_regional_loot, generate_regional_population, generate_regional_site_terminals,
    generate_regional_sites, validate_playable_map, vertical_passage,
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
    pub site_terminal_navigation_signals: bool,
    pub threat_renewal: bool,
    pub destructibles: bool,
    pub environmental_conduction: bool,
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
    if let Some(city) = world.city_at(descriptor.coordinate) {
        return Ok(ZoneInfo {
            id: zone_id(world, descriptor.coordinate)?,
            name: city.name().to_owned(),
            kind: city.kind().clone(),
            depth: descriptor.coordinate.depth,
        });
    }
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
    if let Some(city) = world.city_at(descriptor.coordinate) {
        return generate_city_destination(world, descriptor, info, entrance, city);
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
        && (features.environmental_conduction || !profile.uses_distinct_water_propagation())
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
                        .with_defeat_reward(DefeatReward::summoned(0, 0))
                        .with_tags(profile.tags().iter().cloned());
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
        terminal_navigation_signals: features.site_terminal_navigation_signals,
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

fn generate_city_destination(
    world: &RegionalWorldDefinition,
    descriptor: &RegionDescriptor,
    info: ZoneInfo,
    entrance: GridPos,
    city: &RegionCityDefinition,
) -> Result<GeneratedRegionalDestination, String> {
    let generated = generate_regional_city(city.map_size(), city.layout())
        .map_err(|error| error.to_string())?;
    let cardinal = generated.passages();
    let passages = [
        (RegionDirection::North, cardinal[0]),
        (RegionDirection::East, cardinal[1]),
        (RegionDirection::South, cardinal[2]),
        (RegionDirection::West, cardinal[3]),
    ];
    let vertical_passages = world
        .vertical_neighbors(descriptor.coordinate)
        .into_iter()
        .map(|(direction, _)| (direction, vertical_passage(city.map_size(), direction)))
        .collect::<Vec<_>>();
    if !passages.iter().any(|(_, passage)| *passage == entrance)
        && !vertical_passages
            .iter()
            .any(|(_, passage)| *passage == entrance)
    {
        return Err("Regional city entrance is not a declared passage".into());
    }

    let required_positions = std::iter::once(city.merchant().position)
        .chain(std::iter::once(city.clinic().work_position))
        .chain(std::iter::once(city.clinic().break_position))
        .chain(
            city.residents()
                .iter()
                .flat_map(|resident| [resident.residence_position, resident.gathering_position]),
        )
        .chain(passages.iter().map(|(_, passage)| *passage))
        .chain(vertical_passages.iter().map(|(_, passage)| *passage))
        .collect::<Vec<_>>();
    validate_playable_map(
        generated.map(),
        entrance,
        passages[0].1,
        &required_positions,
        MapValidationRules::default(),
    )
    .map_err(|error| format!("Regional city services are unreachable: {error}"))?;

    let mut actors = Vec::with_capacity(city.residents().len() + 2);
    actors.push(
        Actor::new(city.merchant().position, city.merchant().maximum_integrity)
            .map_err(|error| error.to_string())?
            .with_ai(AiProfile::idle()),
    );
    actors.push(
        Actor::new(city.clinic().work_position, city.clinic().maximum_integrity)
            .map_err(|error| error.to_string())?
            .with_ai(AiProfile::idle()),
    );
    for resident in city.residents() {
        actors.push(
            Actor::new(resident.residence_position, resident.maximum_integrity)
                .map_err(|error| error.to_string())?
                .with_ai(AiProfile::idle()),
        );
    }

    let (map, features, _) = generated.into_parts();
    let mut decor = SectorDecor {
        fallback_name: Some(city.name().to_owned()),
        zones: city_zones(city.layout()),
        ..SectorDecor::default()
    };
    decor.cells.extend(
        features
            .into_iter()
            .map(|(position, feature)| (position, city_decor_for(feature))),
    );
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
    Ok(GeneratedRegionalDestination {
        blueprint: ZoneBlueprint {
            info,
            map,
            entrance,
            seed: descriptor.seed,
            actors,
            loot: Vec::new(),
            threat_sources: Vec::new(),
        },
        facility: None,
        decor,
        passages,
        vertical_passages,
    })
}

fn city_zones(layout: RegionCityLayout) -> Vec<Zone> {
    match layout {
        RegionCityLayout::MaintenanceSpine => vec![
            Zone {
                name: "Marché de récupération".to_owned(),
                bounds: [27, 24, 18, 16],
            },
            Zone {
                name: "Clinique de couche".to_owned(),
                bounds: [52, 24, 18, 16],
            },
            Zone {
                name: "Nef de maintenance".to_owned(),
                bounds: [45, 1, 7, 62],
            },
            Zone {
                name: "Quartiers habités".to_owned(),
                bounds: [24, 7, 52, 50],
            },
        ],
        RegionCityLayout::CoolantRings => vec![
            Zone {
                name: "Marché radial".to_owned(),
                bounds: [27, 28, 13, 17],
            },
            Zone {
                name: "Clinique radiale".to_owned(),
                bounds: [40, 28, 14, 17],
            },
            Zone {
                name: "Cœur technique".to_owned(),
                bounds: [15, 17, 50, 38],
            },
            Zone {
                name: "Anneau de refroidissement".to_owned(),
                bounds: [3, 4, 74, 64],
            },
        ],
        RegionCityLayout::DissonantLattice => vec![
            Zone {
                name: "Échange de lisière".to_owned(),
                bounds: [6, 23, 43, 11],
            },
            Zone {
                name: "Infirmerie greffée".to_owned(),
                bounds: [64, 35, 42, 16],
            },
            Zone {
                name: "Nexus dissonant".to_owned(),
                bounds: [48, 19, 17, 19],
            },
            Zone {
                name: "Cavités habitées".to_owned(),
                bounds: [6, 5, 100, 46],
            },
        ],
        RegionCityLayout::RecursiveBloom => vec![
            Zone {
                name: "Troc des mues".to_owned(),
                bounds: [10, 34, 28, 20],
            },
            Zone {
                name: "Infirmerie enchâssée".to_owned(),
                bounds: [56, 33, 28, 25],
            },
            Zone {
                name: "Pli central".to_owned(),
                bounds: [28, 27, 31, 37],
            },
            Zone {
                name: "Alvéoles habitées".to_owned(),
                bounds: [4, 6, 80, 76],
            },
        ],
        RegionCityLayout::ProcessRuin => vec![
            Zone {
                name: "Échange résiduel".to_owned(),
                bounds: [12, 32, 28, 20],
            },
            Zone {
                name: "Routine de restauration".to_owned(),
                bounds: [68, 29, 29, 23],
            },
            Zone {
                name: "Processus central".to_owned(),
                bounds: [38, 29, 29, 23],
            },
            Zone {
                name: "Fenêtres mortes".to_owned(),
                bounds: [7, 6, 90, 68],
            },
        ],
    }
}

const fn city_decor_for(feature: RegionalCityFeature) -> Decor {
    match feature {
        RegionalCityFeature::Deck => Decor::Deck,
        RegionalCityFeature::CorrodedDeck => Decor::RuinFloor,
        RegionalCityFeature::Grate => Decor::Grate,
        RegionalCityFeature::Lane => Decor::Lane,
        RegionalCityFeature::Threshold => Decor::Threshold,
        RegionalCityFeature::CoolantChannel => Decor::ShallowWater,
        RegionalCityFeature::ForeignFloor => Decor::ForeignFloor,
        RegionalCityFeature::VeinedFloor => Decor::VeinedFloor,
        RegionalCityFeature::ChitinFloor => Decor::ChitinFloor,
        RegionalCityFeature::PulseChannel => Decor::PulseChannel,
        RegionalCityFeature::MemoryFloor => Decor::MemoryFloor,
        RegionalCityFeature::WindowFrame => Decor::WindowFrame,
        RegionalCityFeature::FaultTrace => Decor::FaultTrace,
        RegionalCityFeature::Wall => Decor::Wall,
        RegionalCityFeature::MembraneWall => Decor::MembraneWall,
        RegionalCityFeature::VoidWall => Decor::VoidWall,
        RegionalCityFeature::DeadScreen => Decor::DeadScreen,
        RegionalCityFeature::Pillar => Decor::Pillar,
        RegionalCityFeature::Crate => Decor::Crate,
        RegionalCityFeature::Server => Decor::Server,
        RegionalCityFeature::Console => Decor::Console,
        RegionalCityFeature::Coolant => Decor::Coolant,
        RegionalCityFeature::Resonator => Decor::Resonator,
        RegionalCityFeature::GrowthNode => Decor::GrowthNode,
        RegionalCityFeature::EyeNode => Decor::EyeNode,
        RegionalCityFeature::RootMass => Decor::RootMass,
        RegionalCityFeature::KernelFault => Decor::KernelFault,
        RegionalCityFeature::OrphanProcess => Decor::OrphanProcess,
        RegionalCityFeature::ClinicBed => Decor::ClinicBed,
        RegionalCityFeature::ClinicCounter => Decor::ClinicCounter,
    }
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
    terminal_navigation_signals: bool,
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
        terminal_navigation_signals,
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
    let has_terminals = !terminal_by_site.is_empty();
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
            if navigation_signals
                && (!terminal_navigation_signals || !has_terminals)
                && let Some(range) = profile.navigation_signal_range()
            {
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
            let mut capabilities = vec![InstallationCapability::DataTerminal {
                record: terminal.record.clone(),
            }];
            if navigation_signals
                && terminal_navigation_signals
                && let Some(range) = security.and_then(|profile| profile.navigation_signal_range())
            {
                capabilities.push(InstallationCapability::NavigationBeacon { range });
            }
            installations.push(InstallationBlueprint {
                id: generated_installation_id(zone, index, "terminal")?,
                position: terminal.position,
                maximum_integrity: 10,
                integrity: 10,
                capabilities,
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
                site_terminal_navigation_signals: false,
                threat_renewal: false,
                destructibles: false,
                environmental_conduction: false,
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
                    site_terminal_navigation_signals: true,
                    threat_renewal: true,
                    destructibles: true,
                    environmental_conduction: true,
                    electronic_systems: true,
                    player_relations: true,
                };
                let generated = generate(
                    world,
                    &descriptor,
                    zone_info(world, &descriptor).unwrap(),
                    entrance,
                    Some(loaded.loot()),
                    features,
                )
                .unwrap();
                assert!(
                    generated.blueprint.actors.len() >= expected_minimum,
                    "{} at ({x}, {y}) generated only {} actors",
                    descriptor.biome,
                    generated.blueprint.actors.len()
                );
                if let Some(facility) = &generated.facility {
                    let mut facility_map = generated.blueprint.map.clone();
                    let mut facility_actors = ActorRegistry::default();
                    FacilityState::instantiate(
                        facility.clone(),
                        &mut facility_map,
                        &mut facility_actors,
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
                    let beacons = facility
                        .installations
                        .iter()
                        .filter(|installation| {
                            installation.capabilities.iter().any(|capability| {
                                matches!(
                                    capability,
                                    InstallationCapability::NavigationBeacon { .. }
                                )
                            })
                        })
                        .collect::<Vec<_>>();
                    assert!(!beacons.is_empty(), "a survey terminal needs a site signal");
                    assert!(beacons.iter().all(|installation| {
                        installation.capabilities.iter().any(|capability| {
                            matches!(capability, InstallationCapability::DataTerminal { .. })
                        })
                    }));
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
                        let previous = generate(
                            world,
                            &descriptor,
                            zone_info(world, &descriptor).unwrap(),
                            entrance,
                            Some(loaded.loot()),
                            RegionalGenerationFeatures {
                                site_terminal_navigation_signals: false,
                                ..features
                            },
                        )
                        .unwrap();
                        let previous_installations =
                            &previous.facility.as_ref().unwrap().installations;
                        assert!(previous_installations.iter().any(|installation| {
                            installation.capabilities.iter().any(|capability| {
                                matches!(capability, InstallationCapability::SecuritySensor)
                            }) && installation.capabilities.iter().any(|capability| {
                                matches!(
                                    capability,
                                    InstallationCapability::NavigationBeacon { .. }
                                )
                            })
                        }));
                        assert!(
                            previous_installations
                                .iter()
                                .filter(|installation| {
                                    installation.capabilities.iter().any(|capability| {
                                        matches!(
                                            capability,
                                            InstallationCapability::DataTerminal { .. }
                                        )
                                    })
                                })
                                .all(|installation| installation.capabilities.iter().all(
                                    |capability| {
                                        !matches!(
                                            capability,
                                            InstallationCapability::NavigationBeacon { .. }
                                        )
                                    }
                                ))
                        );
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
                                site_terminal_navigation_signals: false,
                                threat_renewal: true,
                                destructibles: false,
                                environmental_conduction: false,
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
                                site_terminal_navigation_signals: true,
                                threat_renewal: true,
                                destructibles: false,
                                environmental_conduction: false,
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
    fn core_descent_chain_is_stable_bidirectional_and_reaches_layer_five() {
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
            site_terminal_navigation_signals: true,
            threat_renewal: true,
            destructibles: true,
            environmental_conduction: true,
            electronic_systems: true,
            player_relations: true,
        };
        let upper_coordinate = RegionCoord::new(-1, 0, 0);
        let lower_coordinate = RegionCoord::new(-1, 0, 1);
        let second_layer_coordinate = RegionCoord::new(-1, 0, 2);
        let third_layer_coordinate = RegionCoord::new(-1, 0, 3);
        let fourth_layer_coordinate = RegionCoord::new(-1, 0, 4);
        let fifth_layer_coordinate = RegionCoord::new(-1, 0, 5);
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
            vec![
                (RegionVerticalDirection::Up, ascent),
                (RegionVerticalDirection::Down, descent),
            ]
        );
        assert_eq!(lower.decor.cells.get(&ascent), Some(&Decor::Ascent));
        assert_eq!(lower.decor.cells.get(&descent), Some(&Decor::Descent));
        assert!(lower.blueprint.map.is_walkable(ascent));
        assert!(lower.blueprint.map.is_walkable(descent));
        assert_eq!(lower.blueprint.info.name, "Nœud de maintenance");
        assert_eq!(lower.blueprint.info.kind.as_str(), "core:maintenance_city");
        assert_eq!(lower.blueprint.actors.len(), 4);
        assert!(lower.blueprint.loot.is_empty());
        assert!(
            lower
                .blueprint
                .actors
                .iter()
                .all(|actor| actor.destruction_effect().is_none())
        );
        assert!((0..lower.blueprint.map.height()).all(|y| {
            (0..lower.blueprint.map.width()).all(|x| {
                let position = GridPos::new(x as i32, y as i32);
                !lower.blueprint.map.is_walkable(position)
                    || lower.blueprint.map.is_protected(position)
            })
        }));
        assert!(
            lower
                .decor
                .cells
                .values()
                .any(|decor| *decor == Decor::ClinicBed)
        );

        assert_eq!(
            world.vertical_neighbor(lower_coordinate, RegionVerticalDirection::Down),
            Some(second_layer_coordinate)
        );
        let second_layer_descriptor = world.region(17, second_layer_coordinate).unwrap();
        let second_layer_ascent = vertical_passage(
            world.map_size_at(second_layer_coordinate),
            RegionVerticalDirection::Up,
        );
        let second_layer_descent = vertical_passage(
            world.map_size_at(second_layer_coordinate),
            RegionVerticalDirection::Down,
        );
        let second_layer = generate(
            world,
            &second_layer_descriptor,
            zone_info(world, &second_layer_descriptor).unwrap(),
            second_layer_ascent,
            Some(loaded.loot()),
            features,
        )
        .unwrap();
        assert_eq!(
            second_layer.vertical_passages,
            vec![
                (RegionVerticalDirection::Up, second_layer_ascent),
                (RegionVerticalDirection::Down, second_layer_descent),
            ]
        );
        assert_eq!(
            second_layer.decor.cells.get(&second_layer_ascent),
            Some(&Decor::Ascent)
        );
        assert!(second_layer.blueprint.map.is_walkable(second_layer_ascent));
        assert_eq!(
            second_layer.decor.cells.get(&second_layer_descent),
            Some(&Decor::Descent)
        );
        assert!(second_layer.blueprint.map.is_walkable(second_layer_descent));
        assert_eq!(second_layer.blueprint.map.width(), 80);
        assert_eq!(second_layer.blueprint.map.height(), 72);
        assert_eq!(
            second_layer.blueprint.info.name,
            "Couronne de refroidissement"
        );
        assert_eq!(
            second_layer.blueprint.info.kind.as_str(),
            "core:coolant_city"
        );
        assert_eq!(second_layer.blueprint.actors.len(), 5);
        assert!(second_layer.blueprint.loot.is_empty());
        assert!(
            second_layer
                .blueprint
                .actors
                .iter()
                .all(|actor| actor.destruction_effect().is_none())
        );
        assert!(
            second_layer
                .decor
                .cells
                .values()
                .any(|decor| *decor == Decor::ShallowWater)
        );

        assert_eq!(
            world.vertical_neighbor(second_layer_coordinate, RegionVerticalDirection::Down),
            Some(third_layer_coordinate)
        );
        let third_layer_descriptor = world.region(17, third_layer_coordinate).unwrap();
        let third_layer_ascent = vertical_passage(
            world.map_size_at(third_layer_coordinate),
            RegionVerticalDirection::Up,
        );
        let third_layer_descent = vertical_passage(
            world.map_size_at(third_layer_coordinate),
            RegionVerticalDirection::Down,
        );
        let third_layer = generate(
            world,
            &third_layer_descriptor,
            zone_info(world, &third_layer_descriptor).unwrap(),
            third_layer_ascent,
            Some(loaded.loot()),
            features,
        )
        .unwrap();
        assert_eq!(
            third_layer.vertical_passages,
            vec![
                (RegionVerticalDirection::Up, third_layer_ascent),
                (RegionVerticalDirection::Down, third_layer_descent),
            ]
        );
        assert_eq!(
            third_layer.decor.cells.get(&third_layer_ascent),
            Some(&Decor::Ascent)
        );
        assert_eq!(third_layer.blueprint.map.width(), 112);
        assert_eq!(third_layer.blueprint.map.height(), 56);
        assert_eq!(third_layer.blueprint.info.name, "Bastion dissonant");
        assert_eq!(
            third_layer.blueprint.info.kind.as_str(),
            "core:security_city"
        );
        assert_eq!(third_layer.blueprint.actors.len(), 6);
        assert!(third_layer.blueprint.loot.is_empty());
        assert!(
            third_layer
                .decor
                .cells
                .values()
                .any(|decor| *decor == Decor::ForeignFloor)
        );
        assert!(
            third_layer
                .decor
                .cells
                .values()
                .any(|decor| *decor == Decor::MembraneWall)
        );

        assert_eq!(
            world.vertical_neighbor(third_layer_coordinate, RegionVerticalDirection::Down),
            Some(fourth_layer_coordinate)
        );
        let fourth_layer_descriptor = world.region(17, fourth_layer_coordinate).unwrap();
        let fourth_layer_ascent = vertical_passage(
            world.map_size_at(fourth_layer_coordinate),
            RegionVerticalDirection::Up,
        );
        let fourth_layer_descent = vertical_passage(
            world.map_size_at(fourth_layer_coordinate),
            RegionVerticalDirection::Down,
        );
        let fourth_layer = generate(
            world,
            &fourth_layer_descriptor,
            zone_info(world, &fourth_layer_descriptor).unwrap(),
            fourth_layer_ascent,
            Some(loaded.loot()),
            features,
        )
        .unwrap();
        assert_eq!(
            fourth_layer.vertical_passages,
            vec![
                (RegionVerticalDirection::Up, fourth_layer_ascent),
                (RegionVerticalDirection::Down, fourth_layer_descent),
            ]
        );
        assert_eq!(
            fourth_layer.decor.cells.get(&fourth_layer_ascent),
            Some(&Decor::Ascent)
        );
        assert_eq!(fourth_layer.blueprint.map.width(), 88);
        assert_eq!(fourth_layer.blueprint.map.height(), 88);
        assert_eq!(fourth_layer.blueprint.info.name, "Rosace des mues");
        assert_eq!(
            fourth_layer.blueprint.info.kind.as_str(),
            "core:recursive_city"
        );
        assert_eq!(fourth_layer.blueprint.actors.len(), 7);
        assert!(fourth_layer.blueprint.loot.is_empty());
        assert!(
            fourth_layer
                .decor
                .cells
                .values()
                .any(|decor| *decor == Decor::ChitinFloor)
        );
        assert!(
            fourth_layer
                .decor
                .cells
                .values()
                .any(|decor| *decor == Decor::EyeNode)
        );

        assert_eq!(
            world.vertical_neighbor(fourth_layer_coordinate, RegionVerticalDirection::Down),
            Some(fifth_layer_coordinate)
        );
        let fifth_layer_descriptor = world.region(17, fifth_layer_coordinate).unwrap();
        let fifth_layer_ascent = vertical_passage(
            world.map_size_at(fifth_layer_coordinate),
            RegionVerticalDirection::Up,
        );
        let fifth_layer = generate(
            world,
            &fifth_layer_descriptor,
            zone_info(world, &fifth_layer_descriptor).unwrap(),
            fifth_layer_ascent,
            Some(loaded.loot()),
            features,
        )
        .unwrap();
        assert_eq!(
            fifth_layer.vertical_passages,
            vec![(RegionVerticalDirection::Up, fifth_layer_ascent)]
        );
        assert_eq!(
            fifth_layer.decor.cells.get(&fifth_layer_ascent),
            Some(&Decor::Ascent)
        );
        assert_eq!(fifth_layer.blueprint.map.width(), 104);
        assert_eq!(fifth_layer.blueprint.map.height(), 80);
        assert_eq!(fifth_layer.blueprint.info.name, "Noyau des erreurs");
        assert_eq!(
            fifth_layer.blueprint.info.kind.as_str(),
            "core:dead_system_city"
        );
        assert_eq!(fifth_layer.blueprint.actors.len(), 8);
        assert!(fifth_layer.blueprint.loot.is_empty());
        assert!(
            fifth_layer
                .decor
                .cells
                .values()
                .any(|decor| *decor == Decor::MemoryFloor)
        );
        assert!(
            fifth_layer
                .decor
                .cells
                .values()
                .any(|decor| *decor == Decor::OrphanProcess)
        );

        let wild_coordinate = RegionCoord::new(-2, 0, 1);
        let wild_entrance = project_rl::world::generation::cardinal_passage(
            world.local_map_size(),
            Direction::East,
        );
        for seed in 0..64 {
            let descriptor = world.region(seed, wild_coordinate).unwrap();
            let generated = generate(
                world,
                &descriptor,
                zone_info(world, &descriptor).unwrap(),
                wild_entrance,
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

    #[test]
    fn research_relays_are_bounded_electrical_and_version_gated() {
        let content_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content");
        let loaded = ContentLoader::load(&[content_root], &semver::Version::new(0, 1, 0)).unwrap();
        let world = loaded
            .regional_worlds()
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        let descriptor = RegionDescriptor {
            coordinate: RegionCoord::new(3, -2, 2),
            seed: 0xE1EC_7A1C,
            biome: "core:research".parse().unwrap(),
        };
        let entrance = project_rl::world::generation::cardinal_passage(
            world.local_map_size(),
            Direction::West,
        );
        let current_features = RegionalGenerationFeatures {
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
            site_terminal_navigation_signals: true,
            threat_renewal: true,
            destructibles: true,
            environmental_conduction: true,
            electronic_systems: true,
            player_relations: true,
        };
        let current = generate(
            world,
            &descriptor,
            zone_info(world, &descriptor).unwrap(),
            entrance,
            Some(loaded.loot()),
            current_features,
        )
        .unwrap();
        let relays = current
            .blueprint
            .actors
            .iter()
            .filter_map(Actor::destruction_effect)
            .collect::<Vec<_>>();
        assert!((2..=3).contains(&relays.len()));
        assert!(relays.iter().all(|effect| {
            effect.explosion().damage.damage_type == project_rl::combat::DamageType::Electrical
                && effect
                    .ground_effect()
                    .is_some_and(|ground| ground.id().as_str() == "core:electrified_ground")
        }));

        let legacy = generate(
            world,
            &descriptor,
            zone_info(world, &descriptor).unwrap(),
            entrance,
            Some(loaded.loot()),
            RegionalGenerationFeatures {
                environmental_conduction: false,
                ..current_features
            },
        )
        .unwrap();
        assert!(
            legacy
                .blueprint
                .actors
                .iter()
                .all(|actor| actor.destruction_effect().is_none())
        );
    }
}
