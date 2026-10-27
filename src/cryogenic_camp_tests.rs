use super::*;

fn current(seed: u64) -> AsciiApp {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    AsciiApp::from_seed(seed, rules, texts, loot, expeditions).unwrap()
}

// Geometry and door regressions retain the accepted pre-population fixture;
// the live campaign's actors and movement are checked in street_population_tests.
fn street_geometry(seed: u64) -> AsciiApp {
    let (r, t, l, e) = ascii_game_content().unwrap();
    AsciiApp::from_seed_version(seed, r, t, l, e, SEWER_GENERATION_VERSION).unwrap()
}

#[test]
fn normal_character_creation_enables_validated_outfits_city_and_sewer_without_trial_mode() {
    assert!(!crate::street_population::enabled());
    assert!(crate::terminal_fire::enabled());
    for selected in 0..ascii_character_class_catalog().unwrap().iter().count() {
        let mut app = current(1872);
        app.crash_recovery_enabled = false;
        app.suspension_path = std::env::temp_dir().join(format!(
            "project-rl-normal-campaign-{}-{selected}.json",
            std::process::id()
        ));
        // The same creation/rebuild path used by the ordinary main menu.
        app.begin_character_creation_with_seed(false, 1872).unwrap();
        let attributes = app
            .character_classes
            .iter()
            .nth(selected)
            .unwrap()
            .1
            .recommended_attributes();
        let creation = app.character_creation.as_mut().unwrap();
        creation.selected_class = selected;
        creation.attributes = attributes;
        let creation = app.character_creation.take().unwrap();
        app.rebuild_run_with_character_class(&creation).unwrap();
        assert_eq!(app.generation_version, CURRENT_GENERATION_VERSION);
        assert!(!app.test_lab);
        assert_eq!(app.game.current_zone().unwrap().id.as_str(), camp::ZONE_ID);
        let torso = app
            .game
            .player_equipment()
            .equipped(&"core:body_armor".parse().unwrap());
        assert_eq!(torso, None);
        assert!(
            app.game
                .player_inventory()
                .iter()
                .all(|e| !e.item().as_str().starts_with("core:tenue_"))
        );
        assert_eq!(
            app.ux.skill_bar[0].as_deref(),
            Some(["B01", "T01", "FE01", "M01"][selected])
        );
        let exit = pos(camp::scene().exits[0]);
        app.walk_fixture_to_unchecked(exit).unwrap();
        assert_eq!(
            app.execute_command(GameCommand::Interact { target: exit }),
            CommandOutcome::Applied
        );
        assert_eq!(
            app.game.current_zone().unwrap().id.as_str(),
            crate::prison_street::ZONE_ID
        );
        assert_eq!(app.game.actors().iter().count(), 16); // player + approved fifteen enemies
        let passage = app
            .game
            .passage(crate::sewer::ENTRY)
            .expect("sewer entry in the normal city");
        assert!(!app.game.passage_is_irreversible(passage));
        assert!(crate::sewer::has_street_entry(&app.game));
    }
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
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut app = AsciiApp::from_seed_version(
        147,
        rules,
        texts,
        loot,
        expeditions,
        WEAPON_SETS_GENERATION_VERSION,
    )
    .unwrap();
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
fn prison_street_camp_exit_arrives_at_the_south_gate_and_returns_to_the_same_camp() {
    let mut app = current(1818);
    app.crash_recovery_enabled = false;
    let exit = pos(camp::scene().exits[0]);
    let officer = app
        .game
        .actors()
        .entity_at(pos(camp::scene().actors[1].at))
        .unwrap();
    app.walk_fixture_to_unchecked(exit).unwrap();
    let street_id: ContentId = crate::prison_street::ZONE_ID.parse().unwrap();
    assert_eq!(
        app.game.unmaterialized_passage_destination(exit),
        Some(&street_id)
    );
    assert_eq!(
        app.execute_command(GameCommand::Interact { target: exit }),
        CommandOutcome::Applied
    );
    assert_eq!(app.game.current_zone().unwrap().id, street_id);
    let entrance =
        crate::prison_street::pos(crate::prison_street::scene_for_game(&app.game).entrance);
    assert_eq!(app.game.player_position(), Some(entrance));
    assert_eq!(app.game.actors().iter().count(), 16);
    assert!(app.game.quest_journal().is_empty());
    assert_eq!(
        app.game.passage(entrance).unwrap().destination.as_str(),
        camp::ZONE_ID
    );
    assert!(
        !app.game
            .passage_is_irreversible(app.game.passage(entrance).unwrap())
    );
    assert_eq!(
        app.execute_command(GameCommand::Interact { target: entrance }),
        CommandOutcome::Applied
    );
    assert_eq!(app.game.current_zone().unwrap().id.as_str(), camp::ZONE_ID);
    assert_eq!(app.game.player_position(), Some(exit));
    assert_eq!(
        app.game.actors().entity_at(pos(camp::scene().actors[1].at)),
        Some(officer)
    );
}

#[test]
fn prison_street_home_route_obstacles_and_snapshot_are_playable() {
    let mut app = street_geometry(1819);
    app.crash_recovery_enabled = false;
    let exit = pos(camp::scene().exits[0]);
    app.walk_fixture_to_unchecked(exit).unwrap();
    assert_eq!(
        app.execute_command(GameCommand::Interact { target: exit }),
        CommandOutcome::Applied
    );
    let s = crate::prison_street::scene_for_game(&app.game);
    let entrance = crate::prison_street::pos(s.entrance);
    let home = crate::prison_street::pos(s.home_inside);
    let turn = app.game.turn();
    app.walk_fixture_to_unchecked(home)
        .expect("real cardinal route through the home doorway");
    assert!(app.game.turn() > turn);
    let mut cleared = crate::prison_street::contact_repair_cells().unwrap();
    cleared.extend(crate::prison_street::warehouse_clearance_cells().unwrap());
    for &at in &s.obstructions {
        let at = crate::prison_street::pos(at);
        assert_eq!(app.game.map().is_walkable(at), cleared.contains(&at));
    }
    let saved = app.suspension().unwrap();
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut restored =
        AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
    restored.crash_recovery_enabled = false;
    assert_eq!(
        suspension::fingerprint(&app.game),
        suspension::fingerprint(&restored.game)
    );
    assert_eq!(restored.game.player_position(), Some(home));
    for &at in &s.obstructions {
        let at = crate::prison_street::pos(at);
        assert_eq!(restored.game.map().is_walkable(at), cleared.contains(&at));
    }
    restored.walk_fixture_to_unchecked(entrance).unwrap();
    assert_eq!(
        restored.execute_command(GameCommand::Interact { target: entrance }),
        CommandOutcome::Applied
    );
    restored.capture_events_at(Some(0.));
    assert_eq!(
        restored.game.current_zone().unwrap().id.as_str(),
        camp::ZONE_ID
    );
    let saved = restored.suspension().unwrap();
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let resumed = AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
    assert_eq!(
        suspension::fingerprint(&restored.game),
        suspension::fingerprint(&resumed.game)
    );
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
fn prison_street_roof_uses_keyboard_and_click_door_and_survives_resume() {
    use project_rl::world::DoorState;
    let mut app = street_geometry(1820);
    app.crash_recovery_enabled = false;
    let exit = pos(camp::scene().exits[0]);
    app.walk_fixture_to_unchecked(exit).unwrap();
    assert_eq!(
        app.execute_command(GameCommand::Interact { target: exit }),
        CommandOutcome::Applied
    );
    let s = crate::prison_street::scene_for_game(&app.game);
    let door = crate::prison_street::pos(s.home_entry);
    let roof = &s.roofs[0];
    app.walk_fixture_to_unchecked(GridPos::new(door.x, door.y + 1))
        .unwrap();
    assert_eq!(
        app.game.map().tile(door).unwrap().terrain,
        Terrain::Door(DoorState::Closed)
    );
    assert!(!roof.hidden(&app.game));
    assert!(
        app.game
            .player_visibility()
            .is_visible(crate::prison_street::pos(s.home_inside)),
        "Circular perception includes the interior; the closed roof still covers its pixels"
    );
    let turn = app.game.turn();
    app.select_world_target(None);
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
        app.game.map().tile(door).unwrap().terrain,
        Terrain::Door(DoorState::Open)
    );
    assert!(app.game.turn() > turn);
    assert!(roof.hidden(&app.game));
    app.walk_fixture_to_unchecked(GridPos::new(door.x, door.y - 1))
        .unwrap();
    app.select_pointer_world_cell(door, None);
    assert_eq!(
        app.game.map().tile(door).unwrap().terrain,
        Terrain::Door(DoorState::Closed)
    );
    assert!(
        roof.hidden(&app.game),
        "Closing inside must not hide the player"
    );
    app.capture_events_at(Some(0.));
    let saved = app.suspension().unwrap();
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut restored =
        AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
    restored.crash_recovery_enabled = false;
    assert_eq!(
        suspension::fingerprint(&app.game),
        suspension::fingerprint(&restored.game)
    );
    assert!(roof.hidden(&restored.game));
    assert_eq!(
        restored.execute_command(GameCommand::Interact { target: door }),
        CommandOutcome::Applied
    );
    restored
        .walk_fixture_to_unchecked(GridPos::new(door.x, door.y + 1))
        .unwrap();
    restored.select_pointer_world_cell(door, None);
    assert!(!roof.hidden(&restored.game));
    restored
        .walk_fixture_to_unchecked(crate::prison_street::pos(s.entrance))
        .unwrap();
    assert_eq!(
        restored.execute_command(GameCommand::Interact {
            target: crate::prison_street::pos(s.entrance)
        }),
        CommandOutcome::Applied
    );
    assert_eq!(
        restored.execute_command(GameCommand::Interact { target: exit }),
        CommandOutcome::Applied
    );
    assert_eq!(
        restored.game.map().tile(door).unwrap().terrain,
        Terrain::Door(DoorState::Closed)
    );
    assert!(!roof.hidden(&restored.game));
}

#[test]
fn prison_street_generation_155_keeps_its_existing_open_entrance() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut app = AsciiApp::from_seed_version(1821, rules, texts, loot, expeditions, 155).unwrap();
    app.crash_recovery_enabled = false;
    let exit = pos(camp::scene().exits[0]);
    app.walk_fixture_to_unchecked(exit).unwrap();
    assert_eq!(
        app.execute_command(GameCommand::Interact { target: exit }),
        CommandOutcome::Applied
    );
    let s = crate::prison_street::scene_for_game(&app.game);
    let door = crate::prison_street::pos(s.home_entry);
    assert_eq!(app.game.map().tile(door).unwrap().terrain, Terrain::Floor);
    assert!(s.roofs[0].hidden(&app.game));
    app.walk_fixture_to_unchecked(crate::prison_street::pos(s.home_inside))
        .unwrap();
    let saved = app.suspension().unwrap();
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let restored = AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
    assert_eq!(
        suspension::fingerprint(&app.game),
        suspension::fingerprint(&restored.game)
    );
    assert_eq!(
        restored.game.map().tile(door).unwrap().terrain,
        Terrain::Floor
    );
}

#[test]
fn prison_street_expanded_rooms_north_route_and_recovery_use_the_integrated_map() {
    let mut app = street_geometry(1872);
    app.crash_recovery_enabled = false;
    let exit = pos(camp::scene().exits[0]);
    app.walk_fixture_to_unchecked(exit).unwrap();
    assert_eq!(
        app.execute_command(GameCommand::Interact { target: exit }),
        CommandOutcome::Applied
    );
    assert_eq!(app.generation_version, SEWER_GENERATION_VERSION);
    assert_eq!((app.game.map().width(), app.game.map().height()), (26, 66));
    let s = crate::prison_street::scene_for_game(&app.game);
    assert_eq!(s.entrance, [7, 62]);
    for roof in &s.roofs {
        let Some(door) = roof.door.map(crate::prison_street::pos) else {
            continue;
        };
        app.walk_fixture_to_unchecked(GridPos::new(door.x, door.y + 1))
            .unwrap();
        if app.game.map().tile(door).unwrap().terrain
            != Terrain::Door(project_rl::world::DoorState::Open)
        {
            assert_eq!(
                app.execute_command(GameCommand::Interact { target: door }),
                CommandOutcome::Applied
            );
        }
        let inside = roof
            .interior
            .iter()
            .copied()
            .map(crate::prison_street::pos)
            .find(|at| app.game.map().is_walkable(*at))
            .unwrap();
        app.walk_fixture_to_unchecked(inside).unwrap();
        assert!(roof.hidden(&app.game));
    }
    let saved = app.suspension().unwrap();
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut restored =
        AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
    restored.crash_recovery_enabled = false;
    assert_eq!(
        suspension::fingerprint(&app.game),
        suspension::fingerprint(&restored.game)
    );
    assert!(!restored.game.map().blocks_vision(GridPos::new(10, 20)));
    assert!(!restored.game.map().is_walkable(GridPos::new(10, 20)));
    let north = GridPos::new(18, 1);
    restored.walk_fixture_to_unchecked(north).unwrap();
    assert!(
        restored.game.passage(north).is_none(),
        "U02 is not connected yet"
    );
    let entrance = crate::prison_street::pos(s.entrance);
    restored.walk_fixture_to_unchecked(entrance).unwrap();
    assert_eq!(
        restored.execute_command(GameCommand::Interact { target: entrance }),
        CommandOutcome::Applied
    );
    assert_eq!(
        restored.game.current_zone().unwrap().id.as_str(),
        camp::ZONE_ID
    );
}

#[test]
fn prison_street_previous_roof_generation_keeps_small_map_before_and_after_resume() {
    for visited in [false, true] {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed_version(
            1873,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
            156,
        )
        .unwrap();
        app.crash_recovery_enabled = false;
        let exit = pos(camp::scene().exits[0]);
        app.walk_fixture_to_unchecked(exit).unwrap();
        if visited {
            assert_eq!(
                app.execute_command(GameCommand::Interact { target: exit }),
                CommandOutcome::Applied
            );
        }
        app.capture_events_at(Some(0.));
        let saved = app.suspension().unwrap();
        let mut resumed =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        resumed.crash_recovery_enabled = false;
        assert_eq!(
            suspension::fingerprint(&app.game),
            suspension::fingerprint(&resumed.game)
        );
        if !visited {
            assert_eq!(
                resumed.execute_command(GameCommand::Interact { target: exit }),
                CommandOutcome::Applied
            );
        }
        assert_eq!(
            (resumed.game.map().width(), resumed.game.map().height()),
            (26, 18)
        );
        assert_eq!(
            crate::prison_street::scene_for_game(&resumed.game).entrance,
            [7, 14]
        );
    }
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
