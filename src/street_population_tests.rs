use super::*;

fn enemies(app: &AsciiApp) -> Vec<(EntityId, &project_rl::entity::Actor)> {
    app.game
        .actors()
        .iter()
        .filter(|(id, _)| *id != app.game.player_id())
        .collect()
}

#[test]
fn street_population_has_five_groups_and_preserves_existing_combat_profiles() {
    let _scope = crate::street_population::Scope::new();
    let app = AsciiApp::new_street_population_trial().unwrap();
    assert!(!app.test_lab);
    assert_eq!(
        app.character_class.as_ref().unwrap().as_str(),
        "core:bastion"
    );
    assert!(app.game.rules().agent_class_start.is_some());
    assert!(!app.crash_recovery_enabled);
    assert!(
        app.suspension_path
            .ends_with("street-population-review-01/trial-session.json")
    );
    assert_eq!(app.game.player_position(), Some(GridPos::new(7, 62)));
    let actors = enemies(&app);
    assert_eq!(actors.len(), 15);
    let mut counts = [0usize; 3];
    let mut groups = std::collections::BTreeSet::new();
    let world = app
        .regional_worlds
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap();
    let mut reference = project_rl::game::GameState::new_with_rules(
        app.game.map().clone(),
        GridPos::new(7, 62),
        1872,
        app.game.rules().clone(),
    )
    .unwrap();
    for (_, actor) in actors {
        let (number, _) = project_rl::monster_art::appearance(actor).unwrap();
        counts[match number {
            22 => 0,
            24 => 1,
            120 | 121 => 2,
            _ => panic!("unexpected art"),
        }] += 1;
        let at = actor.position();
        assert!(at.x.abs_diff(7).max(at.y.abs_diff(62)) > 6);
        assert!(app.game.map().is_walkable(at));
        let group = actor
            .tags()
            .iter()
            .find(|t| t.as_str().starts_with("core:street_group_"))
            .unwrap();
        groups.insert(group.as_str());
        let level = actor.defeat_reward().unwrap().threat_level;
        let biome = world
            .biome(
                &if level == 1 {
                    "core:human_habitat"
                } else {
                    "core:surface_wilds"
                }
                .parse()
                .unwrap(),
            )
            .unwrap();
        let source = &biome.encounters().rules()[usize::from(number == 24)];
        // Ordinary spawning applies the game's physical hit-point rules.
        // Compare the resulting HP/body with the same unmodified source profile.
        let expected = project_rl::entity::Actor::new(at, source.maximum_integrity())
            .unwrap()
            .with_body_profile(source.body_profile().unwrap())
            .with_primary_attributes(source.primary_attributes().unwrap());
        let expected = reference.spawn_actor(expected).unwrap();
        let expected = reference.actors().get(expected).unwrap();
        assert_eq!(actor.maximum_integrity(), expected.maximum_integrity());
        assert_eq!(actor.attack(0), Some(source.attack()));
        assert_eq!(actor.ai(), Some(source.ai()));
        assert_eq!(actor.primary_attributes(), source.primary_attributes());
        assert_eq!(actor.body_profile(), expected.body_profile());
        assert_eq!(
            actor.defeat_reward().unwrap().base_experience,
            source.defeat_reward().unwrap().base_experience
        );
    }
    assert_eq!(counts, [7, 2, 6]);
    assert_eq!(groups.len(), 5);
    assert!(!app.game.map().is_walkable(GridPos::new(7, 61)));
    assert!(app.game.map().is_walkable(GridPos::new(8, 61)));
}

#[test]
fn street_population_snapshot_and_prison_return_do_not_respawn_groups() {
    let _scope = crate::street_population::Scope::new();
    let mut app = AsciiApp::new_street_population_trial().unwrap();
    let ids: Vec<_> = enemies(&app).iter().map(|(id, _)| *id).collect();
    let saved = app.suspension().unwrap();
    let decoded = AsciiApp::decode_recovery_snapshot(
        saved
            .recovery
            .as_ref()
            .and_then(RecoveryPayload::snapshot_state)
            .unwrap(),
        app.game.rules().clone(),
    )
    .expect("trial snapshot must decode");
    if suspension::fingerprint(&app.game) != suspension::fingerprint(&decoded.game) {
        let original = format!("{:#?}", app.game);
        let restored = format!("{:#?}", decoded.game);
        let at = original
            .bytes()
            .zip(restored.bytes())
            .position(|(a, b)| a != b)
            .unwrap_or(0);
        panic!(
            "snapshot differs at {at}: original {:?}, decoded {:?}",
            original.get(at.saturating_sub(150)..(at + 450).min(original.len())),
            restored.get(at.saturating_sub(150)..(at + 450).min(restored.len()))
        );
    }
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let restored = AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
    assert_eq!(
        suspension::fingerprint(&app.game),
        suspension::fingerprint(&restored.game)
    );
    assert_eq!(enemies(&restored).len(), 15);
    assert_eq!(
        app.execute_command(GameCommand::Interact {
            target: GridPos::new(7, 62)
        }),
        CommandOutcome::Applied
    );
    assert_eq!(app.game.current_zone().unwrap().id.as_str(), camp::ZONE_ID);
    let exit = pos(camp::scene().exits[0]);
    app.walk_fixture_to_unchecked(exit).unwrap();
    assert_eq!(
        app.execute_command(GameCommand::Interact { target: exit }),
        CommandOutcome::Applied
    );
    assert_eq!(
        enemies(&app).iter().map(|(id, _)| *id).collect::<Vec<_>>(),
        ids
    );
}

#[test]
fn street_population_first_enemy_can_be_defeated_with_starting_equipment() {
    let _scope = crate::street_population::Scope::new();
    let mut app = AsciiApp::street_population_fixture(Some(GridPos::new(12, 54))).unwrap();
    let target = app.game.actors().entity_at(GridPos::new(13, 54)).unwrap();
    let xp = app.game.player_progression().experience();
    for _ in 0..12 {
        if app.game.actors().get(target).is_none_or(|a| !a.is_alive()) {
            break;
        }
        assert_eq!(
            app.execute_command(GameCommand::Attack { slot: 0, target }),
            CommandOutcome::Applied
        );
    }
    assert!(app.game.actors().get(target).is_none_or(|a| !a.is_alive()));
    assert!(app.game.player_progression().experience() > xp);
    assert!(
        app.game
            .actors()
            .get(app.game.player_id())
            .unwrap()
            .is_alive()
    );
}

#[test]
fn street_population_legacy_world_keeps_its_empty_street() {
    assert!(!crate::street_population::enabled());
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut app = AsciiApp::from_seed_version(
        1872,
        rules,
        texts,
        loot,
        expeditions,
        SEWER_GENERATION_VERSION,
    )
    .unwrap();
    app.crash_recovery_enabled = false;
    let exit = pos(camp::scene().exits[0]);
    app.walk_fixture_to_unchecked(exit).unwrap();
    assert_eq!(
        app.execute_command(GameCommand::Interact { target: exit }),
        CommandOutcome::Applied
    );
    assert!(enemies(&app).is_empty());
    assert!(app.game.map().is_walkable(GridPos::new(7, 61)));
}

#[test]
fn street_population_normal_campaign_and_sewer_launcher_are_populated_once() {
    assert!(!crate::street_population::enabled());
    let mut app = AsciiApp::sewer_trial_start(1, false).unwrap();
    assert_eq!(app.generation_version, CURRENT_GENERATION_VERSION);
    assert_eq!(app.game.player_position(), Some(GridPos::new(7, 62)));
    assert_eq!(enemies(&app).len(), 15);
    let ids = enemies(&app).iter().map(|(id, _)| *id).collect::<Vec<_>>();
    app.game.drain_events();
    let saved = app.suspension().unwrap();
    let (r, t, l, e) = ascii_game_content().unwrap();
    let mut restored = AsciiApp::restore_suspension(&saved, r, t, l, e).unwrap();
    assert_eq!(
        suspension::fingerprint(&restored.game),
        suspension::fingerprint(&app.game)
    );
    let gate = GridPos::new(7, 62);
    assert_eq!(
        restored.execute_command(GameCommand::Interact { target: gate }),
        CommandOutcome::Applied
    );
    let camp_exit = pos(camp::scene().exits[0]);
    assert_eq!(
        restored.execute_command(GameCommand::Interact { target: camp_exit }),
        CommandOutcome::Applied
    );
    assert_eq!(
        enemies(&restored)
            .iter()
            .map(|(id, _)| *id)
            .collect::<Vec<_>>(),
        ids
    );
    assert!(crate::sewer::has_street_entry(&restored.game));
    assert_eq!(
        restored.execute_command(GameCommand::Move(Direction::North)),
        CommandOutcome::Applied
    );
    assert_eq!(restored.game.player_position(), Some(GridPos::new(7, 61)));
}
