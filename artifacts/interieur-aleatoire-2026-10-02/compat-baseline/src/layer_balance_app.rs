use super::*;

#[test]
fn layer_balance_preserves_placements_groups_loot_and_cities() {
    use crate::test_regional::{RegionalGenerationFeatures, generate, zone_info};
    use project_rl::content::{RegionCoord, RegionDescriptor};
    use project_rl::world::generation::cardinal_passage;
    let catalog = ascii_regional_world_catalog().unwrap();
    let old = catalog.without_layer_balance_metadata();
    let id = "core:simulation_overworld".parse().unwrap();
    let world = catalog.get(&id).unwrap();
    let old = old.get(&id).unwrap();
    let (_, _, loot, _) = ascii_game_content().unwrap();
    let features = RegionalGenerationFeatures {
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
    };
    let mut changed = 0;
    for (biome, depth) in [
        ("core:maintenance", 1),
        ("core:production", 3),
        ("core:research", 4),
        ("core:security", 6),
    ] {
        for seed in 0..4 {
            let descriptor = RegionDescriptor {
                coordinate: RegionCoord::new(3, -2, depth),
                seed,
                biome: biome.parse().unwrap(),
            };
            let entry = cardinal_passage(world.local_map_size(), Direction::West);
            let mut current = generate(
                world,
                &descriptor,
                zone_info(world, &descriptor).unwrap(),
                entry,
                Some(&loot),
                features,
            )
            .unwrap();
            let previous = generate(
                old,
                &descriptor,
                zone_info(old, &descriptor).unwrap(),
                entry,
                Some(&loot),
                features,
            )
            .unwrap();
            assert_eq!(
                current.blueprint.actors.len(),
                previous.blueprint.actors.len()
            );
            for (actor, before) in current
                .blueprint
                .actors
                .iter()
                .zip(&previous.blueprint.actors)
            {
                assert_eq!(actor.position(), before.position());
                assert_eq!(actor.ai(), before.ai());
                assert_eq!(
                    actor.body_profile().map(|b| b.base_armor),
                    before.body_profile().map(|b| b.base_armor)
                );
                if actor != before {
                    changed += 1;
                }
            }
            current.blueprint.actors = previous.blueprint.actors.clone();
            assert_eq!(
                current.blueprint.threat_sources.len(),
                previous.blueprint.threat_sources.len()
            );
            for (source, previous) in current
                .blueprint
                .threat_sources
                .iter_mut()
                .zip(&previous.blueprint.threat_sources)
            {
                assert_eq!(source.actor.defeat_reward().unwrap().base_experience, 0);
                assert_eq!(
                    source.actor.body_profile().map(|b| b.base_armor),
                    previous.actor.body_profile().map(|b| b.base_armor)
                );
                source.actor = previous.actor.clone();
            }
            assert_eq!(
                bincode::serialize(&current.blueprint).unwrap(),
                bincode::serialize(&previous.blueprint).unwrap()
            );
            assert_eq!(current.decor, previous.decor);
            assert_eq!(current.facility, previous.facility);
        }
    }
    assert!(changed > 0);
    for city in world.cities() {
        let descriptor = world.region(INITIAL_SEED, city.coordinate()).unwrap();
        let entry = cardinal_passage(city.map_size(), Direction::West);
        let current = generate(
            world,
            &descriptor,
            zone_info(world, &descriptor).unwrap(),
            entry,
            Some(&loot),
            features,
        )
        .unwrap();
        let previous = generate(
            old,
            &descriptor,
            zone_info(old, &descriptor).unwrap(),
            entry,
            Some(&loot),
            features,
        )
        .unwrap();
        assert_eq!(
            bincode::serialize(&current.blueprint).unwrap(),
            bincode::serialize(&previous.blueprint).unwrap()
        );
    }
}

#[test]
fn layer_balance_sentinel_stats_and_display_match_the_layer_and_survive_snapshot() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut app =
        AsciiApp::from_seed(INITIAL_SEED, rules.clone(), texts, loot, expeditions).unwrap();
    let mut previous_hp = 0;
    let mut previous_damage = 0;
    for depth in 3..=6 {
        app.prepare_layer_balance_diagnostic(depth).unwrap();
        let id = app.selected_target.unwrap();
        let actor = app.game.actors().get(id).unwrap();
        let level = actor.defeat_reward().unwrap().threat_level;
        let damage = app
            .game
            .resolved_attack_damage(id, actor.attack(0).unwrap())
            .unwrap()
            .raw_total();
        println!(
            "Sentinelle couche {depth}: niveau {level}, {} PV effectifs, dégâts {damage}",
            actor.maximum_integrity()
        );
        assert!(actor.maximum_integrity() > previous_hp);
        assert!(damage > previous_damage);
        previous_hp = actor.maximum_integrity();
        previous_damage = damage;
        assert!(
            app.terminal_target_summary()
                .unwrap()
                .name
                .ends_with(&format!("niv. {level}"))
        );
        let bytes = app.game.recovery_snapshot_bytes().unwrap();
        let mut restored = WorldState::from_recovery_snapshot_bytes(&bytes, rules.clone()).unwrap();
        for _ in 0..4 {
            app.game.process_player_command(GameCommand::Wait);
            restored.process_player_command(GameCommand::Wait);
            assert_eq!(app.game.drain_events(), restored.drain_events());
            assert_eq!(
                app.game.recovery_snapshot_bytes().unwrap(),
                restored.recovery_snapshot_bytes().unwrap()
            );
        }
    }
}

#[test]
fn layer_balance_keeps_v112_unchanged_and_v113_replayable() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    for version in [112, 113] {
        let mut app = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
            version,
        )
        .unwrap();
        let balanced = app
            .game
            .actors()
            .iter()
            .filter(|(_, actor)| {
                actor
                    .tags()
                    .iter()
                    .any(|tag| tag.as_str().starts_with("core:encounter_level_"))
            })
            .count();
        assert_eq!(balanced > 0, version == 113);
        for _ in 0..2 {
            app.execute_command(GameCommand::Wait);
        }
        app.game.drain_events();
        let expected = app.game.recovery_snapshot_bytes().unwrap();
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
            assert_eq!(restored.game.recovery_snapshot_bytes().unwrap(), expected);
        }
    }
}
