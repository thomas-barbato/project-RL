use super::*;
use crate::test_sector::Decor;
use project_rl::world::DoorState;
use std::time::Instant;

#[test]
fn dense_surface_doors_open_and_furniture_blocks_movement_and_vision_in_the_engine() {
    let sector = crate::surface_layout::densify_hub(
        TestSector::build(17)
            .unwrap()
            .with_surface_contacts()
            .unwrap(),
        17,
    )
    .unwrap();
    let map = sector.level.map();
    let door = sector
        .decor
        .zones
        .iter()
        .filter(|z| z.name == "Entrepôt de surface")
        .flat_map(|z| {
            let [x, y, w, h] = z.bounds;
            (y..y + h).flat_map(move |py| (x..x + w).map(move |px| GridPos::new(px, py)))
        })
        .find(|p| {
            map.tile(*p)
                .is_some_and(|t| t.terrain == Terrain::Door(DoorState::Closed))
        })
        .unwrap();
    let start = door
        .cardinal_neighbors()
        .into_iter()
        .find(|p| map.is_walkable(*p))
        .unwrap();
    let mut game = GameState::new(map.clone(), start, 17).unwrap();
    assert!(!game.map().is_walkable(door));
    assert!(game.map().blocks_vision(door));
    assert_eq!(
        game.process_player_command(GameCommand::Interact { target: door }),
        CommandOutcome::Applied
    );
    assert!(game.map().is_walkable(door));
    assert!(!game.map().blocks_vision(door));
    let prop = sector
        .decor
        .cells
        .iter()
        .find(|(_, d)| **d == Decor::Crate)
        .unwrap()
        .0;
    assert!(!game.map().is_walkable(*prop));
    assert!(game.map().blocks_vision(*prop));
}

#[test]
fn dense_surface_generations_resume_without_rewriting_previous_maps() {
    for version in [
        EQUIPMENT_UPGRADE_GENERATION_VERSION,
        DENSE_SURFACE_GENERATION_VERSION,
        RANDOM_HOME_GENERATION_VERSION,
    ] {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app =
            AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, version)
                .unwrap();
        app.capture_events_at(Some(0.0));
        let folder =
            std::env::temp_dir().join(format!("rl-density-{}-{version}", std::process::id()));
        app.suspension_path = folder.join("run.json");
        let before = app.game.map().clone();
        let decor = app.terminal.decor.clone();
        let saved = app.suspension().unwrap();
        saved.write(&app.suspension_path).unwrap();
        app.resume_run().unwrap();
        assert_eq!(app.game.map(), &before);
        assert_eq!(app.terminal.decor, decor);
        assert_eq!(suspension::fingerprint(&app.game), saved.state);
        assert_eq!(app.generation_version, version);
        let buildings = decor
            .zones
            .iter()
            .filter(|z| {
                z.name.ends_with("abandonné")
                    || z.name == "Entrepôt de surface"
                    || z.name == "Local technique"
                    || z.name == "Bloc urbain"
            })
            .count();
        assert_eq!(
            buildings == 0,
            version == EQUIPMENT_UPGRADE_GENERATION_VERSION
        );
        std::fs::remove_dir(folder).unwrap();
    }
}

#[test]
fn dense_surface_regions_preserve_sites_loot_actors_and_all_passages() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
    let world = app
        .regional_worlds
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap();
    let mut total = 0;
    for seed in [1, 17, INITIAL_SEED] {
        for coord in [
            RegionCoord::new(-1, 0, 0),
            RegionCoord::new(1, 0, 0),
            RegionCoord::new(0, -1, 0),
            RegionCoord::new(0, 1, 0),
        ] {
            if world.city_at(coord).is_some() {
                continue;
            }
            let descriptor = world.region(seed, coord).unwrap();
            let entrance = project_rl::world::generation::cardinal_passage(
                world.map_size_at(coord),
                Direction::West,
            );
            let mut generated = crate::test_regional::generate(
                world,
                &descriptor,
                crate::test_regional::zone_info(world, &descriptor).unwrap(),
                entrance,
                Some(&app.loot),
                first_layer_landscapes::diagnostic_features(),
            )
            .unwrap();
            let before = generated.blueprint.map.clone();
            let actors = suspension::fingerprint(&generated.blueprint.actors);
            let loot = suspension::fingerprint(&generated.blueprint.loot);
            let facility = suspension::fingerprint(&generated.facility);
            let threats = suspension::fingerprint(&generated.blueprint.threat_sources);
            let sites: Vec<_> = generated
                .decor
                .cells
                .iter()
                .filter(|(_, d)| {
                    matches!(
                        d,
                        Decor::SupplyCache
                            | Decor::ThreatCamp
                            | Decor::DataTerminalOnline
                            | Decor::SensorOnline
                            | Decor::ActuatorOnline
                            | Decor::Descent
                            | Decor::Ascent
                    )
                })
                .map(|(p, d)| (*p, *d))
                .collect();
            let start = Instant::now();
            crate::surface_layout::densify_regional(&mut generated, descriptor.seed).unwrap();
            let added = generated.decor.zones.len();
            total += added;
            eprintln!(
                "density {seed} {coord:?}: {added} buildings, {:?}",
                start.elapsed()
            );
            assert!(added >= 1, "No infill for {seed} {coord:?}");
            assert_eq!(suspension::fingerprint(&generated.blueprint.actors), actors);
            assert_eq!(suspension::fingerprint(&generated.blueprint.loot), loot);
            assert_eq!(suspension::fingerprint(&generated.facility), facility);
            assert_eq!(
                suspension::fingerprint(&generated.blueprint.threat_sources),
                threats
            );
            for (p, d) in sites {
                assert_eq!(generated.decor.cells.get(&p), Some(&d));
                assert_eq!(generated.blueprint.map.tile(p), before.tile(p));
            }
            for actor in &generated.blueprint.actors {
                assert_eq!(
                    generated.blueprint.map.tile(actor.position()),
                    before.tile(actor.position())
                );
            }
            project_rl::world::generation::validate_interactive_map(
                &generated.blueprint.map,
                entrance,
                generated.passages[0].1,
                &generated
                    .passages
                    .iter()
                    .map(|(_, p)| *p)
                    .chain(generated.vertical_passages.iter().map(|(_, p)| *p))
                    .collect::<Vec<_>>(),
                project_rl::world::generation::MapValidationRules::default(),
            )
            .unwrap();
        }
    }
    assert!(total >= 40, "Only {total} buildings across sampled regions");
}
