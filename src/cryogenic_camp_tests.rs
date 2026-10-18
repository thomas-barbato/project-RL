use super::*;

fn current(seed: u64) -> AsciiApp {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    AsciiApp::from_seed(seed, rules, texts, loot, expeditions).unwrap()
}

#[test]
fn cryogenic_all_protocols_begin_in_the_accepted_camp() {
    let catalog = ascii_character_class_catalog().unwrap();
    for (class_id, class) in catalog.iter() {
        let (mut rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        class
            .apply_to_rules(&mut rules, class.recommended_attributes())
            .unwrap();
        let app = AsciiApp::from_seed(319, rules, texts, loot, expeditions).unwrap();
        assert_eq!(
            app.game.current_zone().unwrap().id.as_str(),
            camp::ZONE_ID,
            "{class_id}"
        );
        assert_eq!(app.game.player_position(), Some(GridPos::new(2, 8)));
        assert_eq!((app.game.map().width(), app.game.map().height()), (39, 18));
        assert_eq!(app.game.actors().iter().count(), 5);
        assert!(app.game.quest_journal().is_empty());
        assert!(app.log.is_empty());
        assert_eq!(app.primary_objective(), None);
        for a in &camp::scene().actors[1..] {
            assert!(app.game.actors().entity_at(pos(a.at)).is_some());
        }
        assert!(app.game.map().is_protected(GridPos::new(2, 8)));
        assert!(!app.game.map().is_walkable(GridPos::new(1, 8)));
        assert!(app.game.map().is_walkable(GridPos::new(8, 8)));
    }
}

#[test]
fn cryogenic_services_and_officer_can_be_reached_and_browsed_without_a_turn() {
    let mut app = current(144);
    assert!(app.context_npc_at(GridPos::new(6, 7)));
    let officer = app.game.actors().entity_at(GridPos::new(6, 7)).unwrap();
    assert!(app.game.npc_interaction(officer).is_none());
    for (index, at, approach) in [
        (1, [6, 7], [5, 7]),
        (2, [31, 4], [30, 4]),
        (3, [18, 13], [18, 12]),
        (4, [31, 13], [30, 13]),
    ] {
        app.npc_interaction = None;
        app.walk_fixture_to_unchecked(pos(approach)).unwrap();
        let provider = app.game.actors().entity_at(pos(at)).unwrap();
        let before = suspension::fingerprint(&app.game);
        assert!(app.interaction_candidates().contains(&pos(at)));
        assert!(app.interaction_command_at(pos(at)).is_none());
        assert_eq!(app.npc_interaction, Some(provider));
        let view = app.game.npc_interaction(provider).unwrap();
        assert_eq!(
            view.role,
            match index {
                1 => NpcRole::QuestContact,
                2 => NpcRole::Merchant,
                3 => NpcRole::Healer,
                _ => NpcRole::Artisan,
            }
        );
        if index == 1 {
            let text = app.npc_dialogue(&view);
            assert_eq!(text, camp::OFFICER_DIALOGUE);
            assert!(!text.contains("revenu"));
        }
        assert_eq!(
            app.front_actor_visual(provider),
            Some(crate::game_art::Visual::CampNpc(index))
        );
        assert_eq!(before, suspension::fingerprint(&app.game));
    }
}

#[test]
fn cryogenic_suspension_restores_the_same_camp_and_service_state() {
    let mut app = current(146);
    app.walk_fixture_to_unchecked(GridPos::new(22, 8)).unwrap();
    let saved = app.suspension().unwrap();
    let cache = saved
        .recovery
        .as_ref()
        .and_then(RecoveryPayload::snapshot_state)
        .unwrap();
    let decoded = AsciiApp::decode_recovery_snapshot(cache, app.game.rules().clone())
        .expect("camp snapshot must decode without command replay");
    let original_debug = format!("{:?}", app.game);
    let decoded_debug = format!("{:?}", decoded.game);
    if original_debug != decoded_debug {
        let at = original_debug
            .bytes()
            .zip(decoded_debug.bytes())
            .position(|(a, b)| a != b)
            .unwrap_or(0);
        panic!(
            "snapshot debug differs near byte {at}: original {:?}, decoded {:?}",
            original_debug
                .chars()
                .skip(at.saturating_sub(120))
                .take(400)
                .collect::<String>(),
            decoded_debug
                .chars()
                .skip(at.saturating_sub(120))
                .take(400)
                .collect::<String>()
        );
    }
    assert_eq!(
        suspension::fingerprint(&app.game),
        suspension::fingerprint(&decoded.game)
    );
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let restored = AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
    assert_eq!(
        suspension::fingerprint(&app.game),
        suspension::fingerprint(&restored.game)
    );
    assert_eq!(restored.game.player_position(), Some(GridPos::new(22, 8)));
    assert_eq!(
        restored.game.current_zone().unwrap().id.as_str(),
        camp::ZONE_ID
    );
    assert_eq!(app.terminal, restored.terminal);
}

#[test]
fn cryogenic_exit_uses_the_existing_lazy_sector_provider() {
    let mut app = current(147);
    let exit = pos(camp::scene().exits[0]);
    let npc_ids: Vec<_> = camp::scene().actors[1..]
        .iter()
        .map(|a| app.game.actors().entity_at(pos(a.at)).unwrap())
        .collect();
    app.walk_fixture_to_unchecked(exit).unwrap();
    assert!(
        !app.game
            .passage_is_irreversible(app.game.passage(exit).unwrap())
    );
    assert_eq!(app.game.status(), RunStatus::Active);
    app.crash_recovery_enabled = false;
    app.update_input_at(
        &InputFrame {
            pressed: [app.controls.binding(Action::Interact).clone()].into(),
            pointer: Some((1., 1.)),
            viewport: Some((1600., 1000.)),
            ..Default::default()
        },
        Some(0.),
    );
    assert_eq!(
        app.game.current_zone().unwrap().id.as_str(),
        "core:industrial_sector"
    );
    let saved = app.suspension().unwrap();
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let restored = AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
    assert_eq!(
        suspension::fingerprint(&app.game),
        suspension::fingerprint(&restored.game)
    );

    // Resume outside, use the actual reciprocal passage, then leave again.
    let mut restored = restored;
    let entrance = restored.game.player_position().unwrap();
    let back = restored.game.passage(entrance).unwrap();
    assert_eq!(back.destination.as_str(), camp::ZONE_ID);
    assert!(!restored.game.passage_returns_to_previous_layer(back));
    assert_eq!(
        restored
            .game
            .next_visited_passage_towards(&camp::ZONE_ID.parse().unwrap()),
        Some(entrance)
    );
    restored.crash_recovery_enabled = false;
    // The reciprocal passage uses the same generic left-click path, on the
    // player's cell, as NPCs and other nearby interactables.
    restored.select_pointer_world_cell(entrance, None);
    assert_eq!(
        restored.game.current_zone().unwrap().id.as_str(),
        camp::ZONE_ID
    );
    assert_eq!(restored.game.player_position(), Some(exit));
    assert_eq!(restored.game.visited_zone_count(), 2);
    for (sprite, id) in camp::scene().actors[1..].iter().zip(npc_ids) {
        assert_eq!(restored.game.actors().entity_at(pos(sprite.at)), Some(id));
    }
    assert!(!restored.game.map().is_walkable(GridPos::new(1, 8)));
    let saved = restored.suspension().unwrap();
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut resumed =
        AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
    assert_eq!(
        suspension::fingerprint(&restored.game),
        suspension::fingerprint(&resumed.game)
    );
    assert_eq!(
        resumed.execute_command(GameCommand::Interact { target: exit }),
        CommandOutcome::Applied
    );
    assert_eq!(
        resumed.game.current_zone().unwrap().id.as_str(),
        "core:industrial_sector"
    );
    assert_eq!(resumed.game.visited_zone_count(), 2);
}

#[test]
fn cryogenic_previous_generation_keeps_its_old_start_on_resume() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let app = AsciiApp::from_seed_version(
        148,
        rules.clone(),
        texts.clone(),
        loot.clone(),
        expeditions.clone(),
        MONSTER_BLUE_PALETTES_GENERATION_VERSION,
    )
    .unwrap();
    assert_ne!(app.game.current_zone().unwrap().id.as_str(), camp::ZONE_ID);
    let saved = app.suspension().unwrap();
    let restored = AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
    assert_eq!(
        suspension::fingerprint(&app.game),
        suspension::fingerprint(&restored.game)
    );
    assert_eq!(app.game.player_position(), restored.game.player_position());
}

#[test]
fn cryogenic_npc_markers_are_integrated_and_require_actual_visibility() {
    let mut app = current(149);
    let officer = GridPos::new(6, 7);
    let armorer = GridPos::new(31, 4);
    app.walk_fixture_to_unchecked(GridPos::new(5, 7)).unwrap();
    assert!(app.game.player_visibility().is_visible(officer));
    assert!(!app.game.player_visibility().is_visible(armorer));
    assert_eq!(
        crate::camp_npc_review::marker_at(&app.game, officer),
        Some(1)
    );
    {
        let _review = crate::camp_npc_review::begin();
        assert_eq!(
            crate::camp_npc_review::marker_at(&app.game, officer),
            Some(1)
        );
        assert_eq!(crate::camp_npc_review::marker_at(&app.game, armorer), None);
        assert_eq!(
            crate::camp_npc_review::marker_at(&app.game, GridPos::new(5, 7)),
            None
        );
    }
    assert_eq!(
        crate::camp_npc_review::marker_at(&app.game, officer),
        Some(1)
    );
}
