use super::*;
use project_rl::content::{FirstLayerPlace, FirstLayerPlanDefinition};

const WORLD: &str = "core:simulation_overworld";

#[test]
fn first_layer_encounters_are_opt_in_and_do_not_change_terrain_or_outside_regions() {
    let catalog = ascii_regional_world_catalog().unwrap();
    let old = catalog.without_first_layer_encounters_metadata();
    let (_, _, loot, _) = ascii_game_content().unwrap();
    let source = catalog.get(&WORLD.parse().unwrap()).unwrap();
    for seed in 0..8 {
        let new = source.resolved_for_seed(seed);
        let old = old
            .get(&WORLD.parse().unwrap())
            .unwrap()
            .resolved_for_seed(seed);
        assert_eq!(new.biomes(), old.biomes());
        assert_eq!(new.vertical_links(), old.vertical_links());
        for coordinate in source
            .first_layer_plan(seed)
            .unwrap()
            .places
            .into_iter()
            .map(|(_, at)| at)
            .chain([RegionCoord::new(9, 9, 1), RegionCoord::new(9, 9, 2)])
        {
            let descriptor = new.region(seed, coordinate).unwrap();
            let make = |w: &project_rl::content::RegionalWorldDefinition| {
                crate::test_regional::generate(
                    w,
                    &descriptor,
                    crate::test_regional::zone_info(w, &descriptor).unwrap(),
                    project_rl::world::generation::cardinal_passage(
                        w.map_size_at(coordinate),
                        Direction::West,
                    ),
                    Some(&loot),
                    all_features(),
                )
                .unwrap()
            };
            let a = make(&new);
            let b = make(&old);
            assert_eq!(a.blueprint.map, b.blueprint.map);
            assert_eq!(a.blueprint.loot.len(), b.blueprint.loot.len());
            assert!(!old.has_planned_encounters_at(coordinate));
            if new.planned_landscape_at(coordinate).is_none() {
                assert!(!new.has_planned_encounters_at(coordinate));
                assert_eq!(format!("{:?}", a.blueprint), format!("{:?}", b.blueprint));
                assert_eq!(a.decor, b.decor);
            } else {
                assert!(new.has_planned_encounters_at(coordinate));
            }
        }
    }
}

#[test]
fn first_layer_plan_is_seeded_connected_open_and_keeps_other_shafts() {
    let catalog = ascii_regional_world_catalog().unwrap();
    let world = catalog.get(&WORLD.parse().unwrap()).unwrap();
    let mut orientations = std::collections::BTreeSet::new();
    for seed in 0..128 {
        let plan = world.first_layer_plan(seed).unwrap();
        assert_eq!(Some(plan.clone()), world.first_layer_plan(seed));
        assert_eq!(plan.places.len(), 6);
        assert_eq!(
            plan.places[0],
            (FirstLayerPlace::City, RegionCoord::new(-1, 0, 1))
        );
        orientations.insert((plan.places[2].1, plan.places[3].1));
        for route in plan
            .routes
            .iter()
            .map(Vec::as_slice)
            .chain([plan.cross_link.as_slice(), plan.arrival_route.as_slice()])
        {
            for pair in route.windows(2) {
                assert!(world.cardinal_neighbors(pair[0]).contains(&pair[1]));
            }
        }
        let resolved = world.resolved_for_seed(seed);
        assert_eq!(resolved, resolved.resolved_for_seed(seed));
        assert_eq!(
            resolved.vertical_links().len(),
            world.vertical_links().len()
        );
        for link in world
            .vertical_links()
            .iter()
            .filter(|l| l.upper().depth != 1)
        {
            assert!(resolved.vertical_links().contains(link));
        }
        assert!(
            resolved
                .vertical_neighbor(plan.places[0].1, RegionVerticalDirection::Down)
                .is_none()
        );
        assert!(
            resolved
                .vertical_neighbor(plan.arrival_route[2], RegionVerticalDirection::Up)
                .is_none()
        );
        assert_eq!(
            resolved.vertical_neighbor(plan.descent.upper(), RegionVerticalDirection::Down),
            Some(plan.descent.lower())
        );
        assert_eq!(
            resolved.vertical_neighbor(plan.descent.lower(), RegionVerticalDirection::Up),
            Some(plan.descent.upper())
        );
        for (_, coordinate) in plan.places {
            assert_eq!(resolved.cardinal_neighbors(coordinate).len(), 4);
            assert_eq!(
                world.region(seed, coordinate),
                resolved.region(seed, coordinate)
            );
        }
    }
    assert_eq!(orientations.len(), 8);
}

#[test]
fn first_layer_plan_is_opt_in_and_rejects_invalid_anchors() {
    let catalog = ascii_regional_world_catalog().unwrap();
    let world = catalog.get(&WORLD.parse().unwrap()).unwrap().clone();
    assert!(
        world
            .clone()
            .with_first_layer_plan(Some(FirstLayerPlanDefinition {
                city: "core:unknown_city".parse().unwrap(),
                next_city: "core:coolant_crown".parse().unwrap(),
                landscapes: false,
                encounters: false,
            }))
            .is_err()
    );
    assert!(
        world
            .clone()
            .with_first_layer_plan(Some(FirstLayerPlanDefinition {
                city: "core:coolant_crown".parse().unwrap(),
                next_city: "core:maintenance_exchange".parse().unwrap(),
                landscapes: false,
                encounters: false,
            }))
            .is_err()
    );
    let legacy = catalog.without_first_layer_plan_metadata();
    let legacy = legacy.get(&WORLD.parse().unwrap()).unwrap();
    assert!(legacy.first_layer_plan(7).is_none());
    assert_eq!(*legacy, legacy.resolved_for_seed(7));
    assert_eq!(
        legacy.vertical_neighbor(RegionCoord::new(-1, 0, 1), RegionVerticalDirection::Down),
        Some(RegionCoord::new(-1, 0, 2))
    );
    assert!(world.clone().with_cities(vec![]).is_err());
    assert!(world.with_vertical_links(vec![]).is_err());
}

fn all_features() -> crate::test_regional::RegionalGenerationFeatures {
    crate::test_regional::RegionalGenerationFeatures {
        vertical_travel: true,
        population: true,
        encounters: true,
        exploration_variety: true,
        exploration_salvage: true,
        exploration_salvage_breaches: true,
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
    }
}

#[test]
fn first_layer_plan_generated_places_and_landing_keep_all_passages_reachable() {
    use project_rl::world::generation::{
        MapValidationRules, cardinal_passage, validate_interactive_map,
    };
    let catalog = ascii_regional_world_catalog().unwrap();
    let (_, _, loot, _) = ascii_game_content().unwrap();
    let source = catalog.get(&WORLD.parse().unwrap()).unwrap();
    for seed in 0..8 {
        let plan = source.first_layer_plan(seed).unwrap();
        let world = source.resolved_for_seed(seed);
        for coordinate in plan
            .places
            .iter()
            .map(|(_, coordinate)| *coordinate)
            .chain(plan.arrival_route)
        {
            let descriptor = world.region(seed, coordinate).unwrap();
            let entrance = cardinal_passage(world.map_size_at(coordinate), Direction::West);
            let generated = crate::test_regional::generate(
                &world,
                &descriptor,
                crate::test_regional::zone_info(&world, &descriptor).unwrap(),
                entrance,
                Some(&loot),
                all_features(),
            )
            .unwrap();
            let passages = generated
                .passages
                .iter()
                .map(|(_, p)| *p)
                .chain(generated.vertical_passages.iter().map(|(_, p)| *p))
                .collect::<Vec<_>>();
            validate_interactive_map(
                &generated.blueprint.map,
                entrance,
                passages[0],
                &passages,
                MapValidationRules::default(),
            )
            .unwrap();
            assert!(
                generated
                    .blueprint
                    .actors
                    .iter()
                    .all(|actor| !passages.contains(&actor.position()))
            );
            assert_eq!(
                generated.vertical_passages.len(),
                world.vertical_neighbors(coordinate).len()
            );
            for (direction, at) in &generated.vertical_passages {
                assert_eq!(
                    generated.decor.cells.get(at),
                    Some(&match direction {
                        RegionVerticalDirection::Up => crate::test_sector::Decor::Ascent,
                        RegionVerticalDirection::Down => crate::test_sector::Decor::Descent,
                    })
                );
            }
            if coordinate == plan.places[0].1 {
                assert!(
                    !generated
                        .decor
                        .cells
                        .values()
                        .any(|d| *d == crate::test_sector::Decor::Descent)
                );
            }
        }
    }
}

#[test]
fn first_layer_landscapes_annex_budget_shortcut_and_persistence() {
    use project_rl::world::{DoorState, find_path};
    let catalog = ascii_regional_world_catalog().unwrap();
    let old = catalog.without_first_layer_landscapes_metadata();
    let source = catalog.get(&WORLD.parse().unwrap()).unwrap();
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let app = AsciiApp::from_seed(
        INITIAL_SEED,
        rules.clone(),
        texts,
        loot.clone(),
        expeditions,
    )
    .unwrap();
    for seed in 0..8 {
        let world = source.resolved_for_seed(seed);
        let legacy = old
            .get(&WORLD.parse().unwrap())
            .unwrap()
            .resolved_for_seed(seed);
        assert_eq!(world.vertical_links(), legacy.vertical_links());
        for (role, coordinate) in source.first_layer_plan(seed).unwrap().places {
            let descriptor = world.region(seed, coordinate).unwrap();
            let entrance = project_rl::world::generation::cardinal_passage(
                world.map_size_at(coordinate),
                Direction::West,
            );
            let make = |world: &project_rl::content::RegionalWorldDefinition, features| {
                crate::test_regional::generate(
                    world,
                    &descriptor,
                    crate::test_regional::zone_info(world, &descriptor).unwrap(),
                    entrance,
                    Some(&loot),
                    features,
                )
                .unwrap()
            };
            let mut generated = make(&world, all_features());
            let repeated = make(&world, all_features());
            assert_eq!(
                format!("{:?}", generated.blueprint),
                format!("{:?}", repeated.blueprint)
            );
            assert_eq!(generated.decor, repeated.decor);
            let previous = make(&legacy, all_features());
            if role == FirstLayerPlace::City {
                assert_eq!(
                    format!("{:?}", generated.blueprint),
                    format!("{:?}", previous.blueprint)
                );
                continue;
            }
            assert_ne!(generated.blueprint.map, previous.blueprint.map);
            assert_eq!(
                generated.blueprint.loot.len(),
                previous.blueprint.loot.len(),
                "no extra loot draws"
            );
            if !matches!(role, FirstLayerPlace::Workshops | FirstLayerPlace::Pumps) {
                continue;
            }
            let bounds = generated.decor.zones[0].bounds;
            let cache = GridPos::new(bounds[0] + 4, bounds[1] + 4);
            let reward = generated
                .blueprint
                .loot
                .iter()
                .find(|l| l.position() == cache)
                .unwrap();
            assert!(if role == FirstLayerPlace::Workshops {
                matches!(
                    reward.item().as_str(),
                    "core:integrity_blade"
                        | "core:needle_launcher"
                        | "core:patched_plating"
                        | "core:composite_carapace"
                )
            } else {
                matches!(
                    reward.item().as_str(),
                    "core:repair_patch" | "core:charged_battery"
                )
            });
            assert!(
                generated
                    .blueprint
                    .actors
                    .iter()
                    .all(|a| a.position() != cache)
            );
            let base_reward = reward.item().clone();
            let draws = generated.blueprint.loot.len();
            app.populate_regional_equipment(
                &mut generated.blueprint,
                &descriptor.biome,
                descriptor.seed,
            )
            .unwrap();
            assert_eq!(draws, generated.blueprint.loot.len());
            if role == FirstLayerPlace::Workshops {
                assert_ne!(
                    generated
                        .blueprint
                        .loot
                        .iter()
                        .find(|l| l.position() == cache)
                        .unwrap()
                        .item(),
                    &base_reward
                );
                let door = GridPos::new(bounds[0], bounds[1] + 4);
                let control = GridPos::new(door.x + 2, door.y);
                let start = GridPos::new(door.x + 1, door.y);
                let before = find_path(&generated.blueprint.map, start, entrance, 10000, |_| true)
                    .unwrap()
                    .len();
                // No actors in this interaction-only fixture; combat balance is a separate test.
                let mut game = WorldState::single(
                    GameState::new_with_rules(
                        generated.blueprint.map.clone(),
                        start,
                        seed,
                        rules.clone(),
                    )
                    .unwrap(),
                );
                assert_eq!(
                    game.process_player_command(GameCommand::Interact { target: control }),
                    CommandOutcome::Applied
                );
                assert_eq!(
                    game.process_player_command(GameCommand::Interact { target: door }),
                    CommandOutcome::Applied
                );
                assert_eq!(
                    game.map().tile(door).unwrap().terrain,
                    Terrain::Door(DoorState::Open)
                );
                let after = find_path(game.map(), start, entrance, 10000, |_| true)
                    .unwrap()
                    .len();
                assert!(
                    after + 10 < before,
                    "shortcut must shorten the route: {before} -> {after}"
                );
                let reward = generated
                    .blueprint
                    .loot
                    .iter()
                    .find(|l| l.position() == cache)
                    .unwrap();
                game.spawn_ground_item(cache, reward.item().clone(), reward.quantity())
                    .unwrap();
                // The wall-mounted control is solid: go around it.
                for direction in [
                    Direction::South,
                    Direction::East,
                    Direction::East,
                    Direction::East,
                    Direction::North,
                ] {
                    assert_eq!(
                        game.process_player_command(GameCommand::Move(direction)),
                        CommandOutcome::Applied
                    );
                }
                assert_eq!(game.player_position(), Some(cache));
                assert_eq!(
                    game.process_player_command(GameCommand::PickUp),
                    CommandOutcome::Applied
                );
                assert!(game.ground_items().item_at(cache).is_none());
                game.drain_events();
                let bytes = game.recovery_snapshot_bytes().unwrap();
                let restored =
                    WorldState::from_recovery_snapshot_bytes(&bytes, rules.clone()).unwrap();
                assert_eq!(restored.map(), game.map());
                assert!(restored.ground_items().item_at(cache).is_none());
                assert_eq!(bytes, restored.recovery_snapshot_bytes().unwrap());
                assert_eq!(
                    restored.map().tile(control).unwrap().terrain,
                    Terrain::ControlPanel {
                        door,
                        activated: true
                    }
                );
            }
            let mut features = all_features();
            features.landmarks = false;
            make(&world, features); // Optional diagnostic feature combinations remain usable.
        }
        let coordinate = RegionCoord::new(9, 9, 1);
        assert_eq!(world.planned_landscape_at(coordinate), None);
        assert_eq!(
            world.region(seed, coordinate),
            legacy.region(seed, coordinate)
        );
    }
}

fn cross(app: &mut AsciiApp, destination: RegionCoord) {
    use project_rl::content::RegionDirection;
    use project_rl::world::generation::{cardinal_passage, vertical_passage};
    let current = app.regional_zones[&app.game.current_zone().unwrap().id];
    let world = app
        .regional_worlds
        .get(&WORLD.parse().unwrap())
        .unwrap()
        .resolved_for_seed(app.seed);
    let at = if current.depth != destination.depth {
        let direction = if destination.depth > current.depth {
            RegionVerticalDirection::Down
        } else {
            RegionVerticalDirection::Up
        };
        assert_eq!(
            world.vertical_neighbor(current, direction),
            Some(destination)
        );
        vertical_passage(world.map_size_at(current), direction)
    } else {
        let direction = [
            RegionDirection::North,
            RegionDirection::East,
            RegionDirection::South,
            RegionDirection::West,
        ]
        .into_iter()
        .find(|d| current.step(*d) == Some(destination))
        .unwrap();
        if current == RegionCoord::new(0, 0, 0) {
            AsciiApp::hub_regional_passage(direction).unwrap()
        } else {
            cardinal_passage(world.map_size_at(current), direction_from_region(direction))
        }
    };
    app.walk_fixture_to(at).unwrap();
    assert_eq!(
        app.execute_command(GameCommand::Interact { target: at }),
        CommandOutcome::Applied
    );
    app.capture_events_at(Some(0.0));
    assert_eq!(
        app.regional_zones[&app.game.current_zone().unwrap().id],
        destination
    );
}

#[test]
fn first_layer_plan_runtime_round_trip_snapshot_and_replay() {
    let (mut rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    // Connectivity/replay probe, NOT a survival or balance measurement.
    // Avoid an unrelated death ending a long route; do not refill on passage.
    rules.player_body_profile = None;
    rules.player_maximum_integrity = 60_000;
    let mut app = AsciiApp::from_seed_version(
        7,
        rules.clone(),
        texts.clone(),
        loot.clone(),
        expeditions.clone(),
        118,
    )
    .unwrap();
    let world = app.regional_worlds.get(&WORLD.parse().unwrap()).unwrap();
    let plan = world.first_layer_plan(app.seed).unwrap();
    cross(&mut app, RegionCoord::new(-1, 0, 0));
    cross(&mut app, plan.places[0].1);
    assert!(
        app.regional_depth_route_signal_summary().is_none(),
        "unexplored descent leaked"
    );
    let city_descent = project_rl::world::generation::vertical_passage(
        app.regional_worlds
            .get(&WORLD.parse().unwrap())
            .unwrap()
            .map_size_at(plan.places[0].1),
        RegionVerticalDirection::Down,
    );
    assert!(app.game.passage(city_descent).is_none());
    for coordinate in plan.routes[0].iter().skip(1) {
        cross(&mut app, *coordinate);
    }
    let descent_at = project_rl::world::generation::vertical_passage(
        app.regional_worlds
            .get(&WORLD.parse().unwrap())
            .unwrap()
            .map_size_at(plan.descent.upper()),
        RegionVerticalDirection::Down,
    );
    if !app.game.player_visibility().is_explored(descent_at) {
        assert!(app.vertical_navigation_signal_summary().is_none());
    }
    cross(&mut app, plan.descent.lower());
    // At a new landing, returning through the same passage spends no extra
    // traversal resources and must never grant free healing.
    let hp = app
        .game
        .actors()
        .get(app.game.player_id())
        .unwrap()
        .integrity();
    cross(&mut app, plan.descent.upper());
    assert_eq!(
        app.game
            .actors()
            .get(app.game.player_id())
            .unwrap()
            .integrity(),
        hp
    );
    cross(&mut app, plan.descent.lower());
    for coordinate in plan.arrival_route.iter().skip(1) {
        cross(&mut app, *coordinate);
    }
    assert_eq!(
        app.game.current_zone().unwrap().name,
        "Couronne de refroidissement"
    );
    assert!(
        app.game
            .actors()
            .iter()
            .any(|(id, _)| app.game.active_merchant(id))
    );
    // The deeper shaft is unchanged and still usable in a new-generation run.
    cross(&mut app, RegionCoord::new(-1, 0, 3));
    cross(&mut app, plan.arrival_route[2]);
    let saved = app.suspension().unwrap();
    let before = app.game.recovery_snapshot_bytes().unwrap();
    for replay in [false, true] {
        let mut saved = saved.clone();
        if replay {
            saved.build = "0000000000000000".into();
        }
        let restored = AsciiApp::restore_suspension(
            &saved,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
        )
        .unwrap();
        assert_eq!(before, restored.game.recovery_snapshot_bytes().unwrap());
        assert_eq!(
            Some(plan.clone()),
            restored
                .regional_worlds
                .get(&WORLD.parse().unwrap())
                .unwrap()
                .first_layer_plan(restored.seed)
        );
        if !replay {
            app = restored;
        }
    }
    for coordinate in plan.arrival_route.iter().rev().skip(1) {
        cross(&mut app, *coordinate);
    }
    cross(&mut app, plan.descent.upper());
    for coordinate in plan.routes[1].iter().rev().skip(1) {
        cross(&mut app, *coordinate);
    }
    assert_eq!(app.game.current_zone().unwrap().name, "Nœud de maintenance");
    cross(&mut app, RegionCoord::new(-1, 0, 0));
}
