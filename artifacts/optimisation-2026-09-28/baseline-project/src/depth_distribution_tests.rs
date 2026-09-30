use super::*;
use std::collections::BTreeMap;

const WORLD: &str = "core:simulation_overworld";

#[test]
fn depth_distribution_legacy_fingerprints() {
    let regions = ascii_regional_world_catalog().unwrap();
    let (_, _, _, expeditions) = ascii_game_content().unwrap();
    // Recorded from the working v116 catalog before introducing depth weights.
    for (version, expected) in [
        (15, 15180535885259714919),
        (98, 9892304291343060175),
        (109, 15810520955881226503),
        (110, 747664789216364421),
        (111, 3468351940251262990),
        (112, 8903987091305210840),
        (113, 12013899932914374830),
        (114, 5616403682822122487),
        (115, 14713458714871674612),
        (116, 4342512790563608176),
        (117, 7512037609281599692),
        (118, 2694233124376214523),
        (119, 16365257538551790727),
    ] {
        assert_eq!(
            world_fingerprint_for_version(&expeditions, &regions, version),
            expected,
            "v{version}"
        );
    }
    assert_ne!(
        world_fingerprint_for_version(&expeditions, &regions, 117),
        4342512790563608176
    );
}

#[test]
fn depth_distribution_matches_authored_weights_over_independent_provinces() {
    let regions = ascii_regional_world_catalog().unwrap();
    let old = regions.without_depth_distribution_metadata();
    let id = WORLD.parse().unwrap();
    let world = regions.get(&id).unwrap();
    let old = old.get(&id).unwrap();
    let expected = [
        ("maintenance", vec![(1, 65), (2, 30)]),
        ("production", vec![(1, 35), (2, 45), (3, 25)]),
        ("research", vec![(2, 25), (3, 45), (4, 25)]),
        ("security", vec![(3, 30), (4, 40), (5, 20), (6, 10)]),
        ("network", vec![(4, 35), (5, 55), (6, 40), (7, 25)]),
        ("corrupted", vec![(5, 25), (6, 50), (7, 75)]),
    ];
    for (name, schedule) in expected {
        let id = format!("core:{name}").parse().unwrap();
        let biome = world.biome(&id).unwrap();
        for (depth, weight) in schedule {
            assert_eq!(biome.weight_at_depth(depth), weight);
        }
        // No body, AI, habitat, density, loot or layer limits changed.
        assert_eq!(
            biome.clone().with_depth_weights(vec![]).unwrap(),
            *old.biome(&id).unwrap()
        );
    }
    let province = i32::from(world.province_size());
    for depth in 0..=7 {
        let mut counts = BTreeMap::<ContentId, u32>::new();
        for seed in 0..32 {
            for x in -8..8 {
                for y in -8..8 {
                    let coordinate = RegionCoord::new(x * province, y * province, depth);
                    let descriptor = world.region(seed, coordinate).unwrap();
                    assert_eq!(Some(descriptor.clone()), world.region(seed, coordinate));
                    let neighboring = world
                        .region(
                            seed,
                            RegionCoord::new(coordinate.x + 1, coordinate.y + 1, depth),
                        )
                        .unwrap();
                    assert_eq!(descriptor.biome, neighboring.biome);
                    assert_ne!(descriptor.seed, neighboring.seed);
                    assert_eq!(descriptor.seed, old.region(seed, coordinate).unwrap().seed);
                    if depth == 0 {
                        assert_eq!(Some(descriptor.clone()), old.region(seed, coordinate));
                    }
                    *counts.entry(descriptor.biome).or_default() += 1;
                }
            }
        }
        for biome in world.biomes() {
            let actual = 100.0 * f64::from(*counts.get(biome.biome()).unwrap_or(&0)) / 8192.0;
            let expected = f64::from(biome.weight_at_depth(depth));
            // Authored weights total 100 at every depth, including surface.
            assert!(
                (actual - expected).abs() < 2.0,
                "depth {depth}, {}: {actual}% versus {expected}%",
                biome.biome()
            );
        }
        println!("depth {depth}: {counts:?}");
    }
}

#[test]
fn depth_distribution_survives_snapshot_and_command_replay_for_old_and_new_runs() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    for version in [116, 117, 118, 119, 120, 121, 122] {
        let mut app = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
            version,
        )
        .unwrap();
        app.execute_command(GameCommand::Wait);
        app.game.drain_events();
        let state = app.game.recovery_snapshot_bytes().unwrap();
        let saved = app.suspension().unwrap();
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
            assert_eq!(restored.generation_version, version);
            assert_eq!(state, restored.game.recovery_snapshot_bytes().unwrap());
            assert_eq!(app.regional_worlds, restored.regional_worlds);
            let world = restored
                .regional_worlds
                .get(&WORLD.parse().unwrap())
                .unwrap();
            assert_eq!(
                world
                    .biome(&"core:corrupted".parse().unwrap())
                    .unwrap()
                    .weight_at_depth(7),
                if version == 116 { 15 } else { 75 }
            );
            // Future unexplored destinations retain exactly the same draw.
            for depth in 1..=7 {
                let coordinate = RegionCoord::new(13, -11, depth);
                assert_eq!(
                    world.region(app.seed, coordinate),
                    app.regional_worlds
                        .get(&WORLD.parse().unwrap())
                        .unwrap()
                        .region(app.seed, coordinate)
                );
            }
        }
    }
}

#[test]
fn depth_distribution_generated_destinations_allow_immediate_retreat_and_snapshot_resume() {
    use project_rl::game::ZoneInfo;
    use project_rl::world::Map;
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let app = AsciiApp::from_seed_version(
        INITIAL_SEED,
        rules,
        texts,
        loot,
        expeditions,
        CURRENT_GENERATION_VERSION,
    )
    .unwrap();
    let definition = app.regional_worlds.get(&WORLD.parse().unwrap()).unwrap();
    let mut observed = std::collections::BTreeSet::new();
    for depth in 1..=7 {
        for seed in 0..4 {
            let descriptor = definition
                .region(seed, RegionCoord::new(3, -2, depth))
                .unwrap();
            observed.insert(descriptor.biome.clone());
            let info = crate::test_regional::zone_info(definition, &descriptor).unwrap();
            let entrance = project_rl::world::generation::cardinal_passage(
                definition.map_size_at(descriptor.coordinate),
                Direction::West,
            );
            let mut generated = crate::test_regional::generate(
                definition,
                &descriptor,
                info.clone(),
                entrance,
                Some(&app.loot),
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
                },
            )
            .unwrap();
            app.populate_regional_equipment(
                &mut generated.blueprint,
                &descriptor.biome,
                descriptor.seed,
            )
            .unwrap();
            app.populate_carried_weapons(&mut generated.blueprint, descriptor.seed)
                .unwrap();
            let refuge = ZoneInfo {
                id: "test:depth_refuge".parse().unwrap(),
                name: "Refuge test".into(),
                kind: "core:human_habitat".parse().unwrap(),
                depth,
            };
            let mut world = WorldState::single(
                GameState::new_with_rules(
                    Map::from_ascii("#####\n#...#\n#####").unwrap(),
                    GridPos::new(1, 1),
                    seed,
                    super::expedition_survival_tests::finite_rules(&app.rules),
                )
                .unwrap(),
            );
            world.enable(refuge.clone()).unwrap();
            world.add_zone(generated.blueprint).unwrap();
            if let Some(facility) = generated.facility {
                world.register_facility(info.id.clone(), facility).unwrap();
            }
            let gate = GridPos::new(2, 1);
            world
                .connect(refuge.id.clone(), gate, info.id.clone(), entrance)
                .unwrap();
            let hp = super::expedition_survival_tests::hp(&world);
            assert_eq!(
                world.process_player_command(GameCommand::Interact { target: gate }),
                CommandOutcome::Applied
            );
            assert_eq!(world.current_zone().unwrap().id, info.id);
            assert_eq!(super::expedition_survival_tests::hp(&world), hp);
            world.drain_events();
            let bytes = world.recovery_snapshot_bytes().unwrap();
            let mut resumed = WorldState::from_recovery_snapshot_bytes(
                &bytes,
                super::expedition_survival_tests::finite_rules(&app.rules),
            )
            .unwrap();
            for candidate in [&mut world, &mut resumed] {
                assert_eq!(
                    candidate.process_player_command(GameCommand::Interact { target: entrance }),
                    CommandOutcome::Applied
                );
                assert_eq!(candidate.current_zone().unwrap().id, refuge.id);
                assert_eq!(super::expedition_survival_tests::hp(candidate), hp);
            }
            assert_eq!(world.drain_events(), resumed.drain_events());
            assert_eq!(
                world.recovery_snapshot_bytes().unwrap(),
                resumed.recovery_snapshot_bytes().unwrap()
            );
        }
    }
    assert_eq!(observed.len(), 6, "all six deep biomes must be exercised");
}
