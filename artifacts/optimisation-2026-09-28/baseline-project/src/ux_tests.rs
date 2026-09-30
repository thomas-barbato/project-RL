// Included in ascii_app::tests to exercise the same input route as the game.
#[test]
fn ux_reserved_energy_panel_is_free_modal_and_stop_commands_can_be_replayed() {
    let mut app = app_with_test_controls();
    app.prepare_reserved_energy_diagnostic().unwrap();
    let energy = app.game.player_energy();
    assert_eq!(energy.reserved(), 16);
    let gauge = app.resource_gauges().into_iter().find(|g| g.label == "Énergie").unwrap();
    assert_eq!(gauge.reserved_ratio, 0.16);
    assert_eq!(gauge.ratio, energy.available() as f32 / 100.0);
    assert_eq!(app.game.player_companion_count(), 3);
    app.controls.rebind(Action::Learn, Binding::key("F6")).unwrap();
    let before = (suspension::fingerprint(&app.game), app.history.len());
    app.ux.inspected_target = true;
    app.update_input(&input("F6"));
    assert!(app.ux.resources_open);
    app.update_input(&input("Z"));
    assert_eq!((suspension::fingerprint(&app.game), app.history.len()), before);
    let bytes = app.game.recovery_snapshot_bytes().unwrap();
    let mut replay = WorldState::from_recovery_snapshot_bytes(&bytes, app.game.rules().clone()).unwrap();
    app.update_input(&input("F6"));
    assert_eq!(app.game.turn(), replay.turn() + 1);
    let recorded = app.history.last().unwrap();
    let serialized = serde_json::to_string(recorded).unwrap();
    let decoded: suspension::RecordedCommand = serde_json::from_str(&serialized).unwrap();
    assert_eq!(replay.process_player_command(decoded.command(&replay).unwrap()), CommandOutcome::Applied);
    replay.drain_events();
    assert_eq!(suspension::fingerprint(&app.game), suspension::fingerprint(&replay));
    assert!(app.game.player_energy().reserved() < energy.reserved());
    app.update_input(&input("Escape"));
    assert!(!app.ux.resources_open);
    assert_eq!(app.menu, MenuScreen::Hidden);
}

#[test]
fn ux_current_resource_copy_has_no_bandwidth_cost_and_v125_keeps_legacy_rules() {
    let app = app_with_test_controls();
    for (id, definition) in app.game.rules().skills.techniques() {
        let usage = app.technique_usage(definition);
        assert!(!app.resource_technique_description(definition).contains("bande passante"), "{id}");
        assert!(!usage.split(|c: char| !c.is_alphanumeric()).any(|word| word == "B") && !usage.contains("bande passante"), "{id}: {usage}");
    }
    let sections = app.statistics_help();
    assert_eq!(sections[0].entries[3].0, "Compagnons");
    assert!(sections.iter().flat_map(|section| &section.entries).all(|(name, text)| !format!("{name} {text}").to_lowercase().contains("chaleur")));
    assert!(sections[0].entries[4].1.contains("violet hachuré"));
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let old = AsciiApp::from_seed_version(0, rules, texts, loot, expeditions, 125).unwrap();
    assert!(!old.game.rules().maintained_energy_reservations);
    assert_eq!(old.game.rules().player_companion_limit, None);
    assert!(old.game.player_bandwidth().is_some());
    assert!(old.resource_gauges().iter().any(|g| g.label == "Chaleur"));
}

#[test]
fn ux_resource_gauges_follow_heat_bandwidth_and_energy_after_real_actions() {
    let mut app = app_with_test_controls();
    app.prepare_shared_supplies_diagnostic(true).unwrap();
    let id: TechniqueId = "core:gel_01".parse().unwrap();
    assert_eq!(app.execute_command(GameCommand::LearnTechnique { technique: id.clone() }), CommandOutcome::AppliedWithoutTime);
    assert_eq!(app.execute_command(GameCommand::UseElectronicWarfareTechnique { technique: id, directive: ElectronicDirective::Pulse { direction: None } }), CommandOutcome::Applied);
    assert_eq!(app.game.player_heat().unwrap().current(), 0);
    let profile = DroneProfile::new("core:diagnostic_companion".parse().unwrap(), 6, 50, 40, 1, 3, 6, 1, 1, DroneCapabilities::default()).unwrap();
    app.game.spawn_manifested_player_drone(Actor::new(GridPos::new(9, 10), 10).unwrap(), profile, 10, 10).unwrap();
    let gauges = app.resource_gauges();
    assert_eq!(gauges.iter().map(|g| g.label).collect::<Vec<_>>(), ["PV", "Énergie", "Munitions", "Compagnons"]);
    assert!(app.game.player_bandwidth().is_none());
    assert_eq!(gauges.last().unwrap().companion_slots, Some((1, 5)));
    let energy = app.game.player_energy();
    assert!(energy.available() < 20);
    assert_eq!(gauges.iter().find(|g| g.label == "Énergie").unwrap().value, format!("{} libres · {} réservés", energy.available(), energy.reserved()));
    for width in [700.0, 960.0, 1280.0 / 1.5] {
        let rows = AsciiApp::compact_resource_rects(width, gauges.len());
        assert_eq!(rows.len(), 4);
        assert!(rows.iter().all(|r| r.x >= 0.0 && r.right() <= width && r.bottom() < 69.0 && r.w > 120.0));
        assert!(rows.windows(2).all(|r| r[0].right() < r[1].x));
    }
}

#[test]
fn ux_statistics_help_is_complete_live_and_free_to_read() {
    let mut app = app_with_test_controls();
    let before = (suspension::fingerprint(&app.game), app.history.len());
    let sections = app.statistics_help();
    assert_eq!(sections.len(), 6);
    assert_eq!(sections.last().unwrap().entries[0].1.contains("restaure aussi tous vos PV"), app.game.rules().player_full_heal_on_level_up);
    for primary in PrimaryAttribute::ALL {
        assert!(sections.iter().flat_map(|s| &s.entries).any(|(name, text)| name == primary_attribute_label(primary) && text.starts_with(primary_attribute_description(primary))));
    }
    for gauge in app.resource_gauges() {
        assert!(sections[0].entries.iter().any(|(name, _)| name.contains(gauge.label) || gauge.label == "PV" && name.contains("PV")));
    }
    app.update_input(&input("F1"));
    app.update_input(&input("Right")); app.update_input(&input("Right"));
    assert_eq!(app.ux.help_tab, 2);
    app.ux.help_scroll.maximum.set(4000.0);
    app.update_input(&input("PageDown")); assert!(app.ux.help_scroll.offset > 0.0);
    app.update_input(&input("Escape")); assert!(!app.legend_open);
    assert_eq!((suspension::fingerprint(&app.game), app.history.len()), before);
}

#[test]
fn ux_statistics_legacy_resources_do_not_promise_modern_regeneration_or_level_healing() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let app = AsciiApp::from_seed_version(0, rules, texts, loot, expeditions, 122).unwrap();
    let sections = app.statistics_help();
    let energy = sections[0].entries.iter().find(|(name, _)| name == "Énergie (E)").unwrap();
    assert_eq!(app.game.rules().player_energy_regeneration, 0);
    assert!(energy.1.contains("n'accorde pas de récupération automatique"));
    assert_eq!(sections.last().unwrap().entries[0].1.contains("restaure aussi tous vos PV"), app.game.rules().player_full_heal_on_level_up);
    assert!(!app.resource_gauges().iter().any(|g| g.label == "Munitions" && g.value == "40"));
}

#[test]
fn ux_help_uses_current_bindings_layout_and_mouse_labels() {
    let mut app=app_with_test_controls();
    let before=(suspension::fingerprint(&app.game),app.history.len());
    for (action,key) in [(Action::Attack,"F6"),(Action::Legend,"F7"),(Action::Inventory,"F8"),(Action::MenuDown,"F9")] {
        app.controls.rebind(action,Binding::key(key)).unwrap();
    }
    let lines=app.help_command_lines().join("\n");
    assert!(lines.contains("Attaquer · F6"));
    assert!(lines.contains("Inventaire · F8"));
    assert!(lines.contains("Aide : F7"));
    assert!(lines.contains(&app.controls.label(Action::MenuDown)));
    assert!(!lines.contains("Attaquer · F."));
    app.update_input(&input("F7"));assert!(app.legend_open);
    app.update_input(&input("F7"));assert!(!app.legend_open);
    app.controls.rebind(Action::Attack,Binding::MouseRight).unwrap();
    assert!(app.help_command_lines().join("\n").contains(&format!("Attaquer · {}",app.controls.label(Action::Attack))));
    for layout in [controls::Layout::Azerty,controls::Layout::Qwerty] {
        app.controls=Controls::preset(layout,controls::KeySemantics::Physical);
        assert!(app.help_command_lines()[0].contains(&format!("{} {} {} {}",app.controls.label(Action::MoveNorth),app.controls.label(Action::MoveWest),app.controls.label(Action::MoveSouth),app.controls.label(Action::MoveEast))));
    }
    assert_eq!((suspension::fingerprint(&app.game),app.history.len()),before);
}

#[test]
fn ux_restart_requires_confirmation_and_cancel_keeps_the_run() {
    let mut app=app_with_test_controls();
    let before=(suspension::fingerprint(&app.game),app.seed,app.history.len());
    app.update_input(&input("R"));
    assert_eq!(app.menu,MenuScreen::ConfirmRestart);assert_eq!(app.menu_selection,0);
    app.update_input(&input("Enter"));assert_eq!(app.menu,MenuScreen::Hidden);
    assert_eq!((suspension::fingerprint(&app.game),app.seed,app.history.len()),before);
    app.update_input(&input("R"));app.update_input(&input("Down"));app.update_input(&input("Enter"));
    assert_eq!(app.seed,before.1.wrapping_add(1));assert_eq!(app.game.turn(),0);
}

#[test]
fn ux_inventory_hover_and_readers_never_equip_or_spend_a_turn() {
    let mut app=app_with_test_controls();
    app.update_input(&input("I"));
    let before=(suspension::fingerprint(&app.game),app.history.len());
    let layout=InventoryLayout::new(1280.0,800.0,0,app.inventory_entries().len());
    for rect in layout.actions.iter().take(3) {
        let mut hover=rect_pointer(*rect,0.0);hover.pressed.clear();app.update_input(&hover);
        assert_eq!((suspension::fingerprint(&app.game),app.history.len()),before);
    }
    app.update_input(&input("F4"));assert!(app.ux.item_card.is_some());
    app.update_input(&input("Key2"));assert_eq!(app.ux.comparison_slot,Some(1));
    app.update_input(&input("Escape"));assert!(app.inventory_open);
    assert_eq!((suspension::fingerprint(&app.game),app.history.len()),before);
}

#[test]
fn ux_personalized_attributes_survive_back_and_forward() {
    let mut app=app_with_test_controls();app.begin_character_creation(false).unwrap();
    app.update_input(&input("Tab"));app.update_input(&input("Left"));
    app.update_input(&input("Down"));app.update_input(&input("Right"));
    let attributes=app.character_creation.as_ref().unwrap().attributes;
    app.update_input(&input("Escape"));assert_eq!(app.character_creation.as_ref().unwrap().stage,CharacterCreationStage::Protocol);
    app.update_input(&input("Tab"));assert_eq!(app.character_creation.as_ref().unwrap().attributes,attributes);
    assert_eq!(app.game.turn(),0);assert!(app.history.is_empty());
}

#[test]
fn ux_history_is_bounded_modal_and_restored_with_legacy_fallback() {
    let mut app=app_with_test_controls();
    let before=suspension::fingerprint(&app.game);
    for i in 0..650 {app.push_log(format!("Événement {i}"));}
    assert_eq!(app.ux.history.len(),600);assert_eq!(app.log.len(),6);
    app.update_input(&input("V"));assert!(app.ux.history_open);
    app.ux.history_scroll.maximum.set(1000.0);app.update_input(&input("PageDown"));assert!(app.ux.history_scroll.offset>0.0);
    app.update_input(&input("Escape"));assert!(!app.ux.history_open);
    assert_eq!(suspension::fingerprint(&app.game),before);
    let saved=app.suspension().unwrap();
    let restored=AsciiApp::restore_suspension(&saved,app.rules.clone(),app.texts.clone(),app.loot.clone(),app.expeditions.clone()).unwrap();
    assert_eq!(restored.ux.history,app.ux.history);
    let mut legacy=serde_json::to_value(saved).unwrap();
    legacy.as_object_mut().unwrap().remove("observed_events");legacy.as_object_mut().unwrap().remove("tracked_quest");
    let saved:Suspension=serde_json::from_value(legacy).unwrap();saved.validate().unwrap();
    assert!(saved.observed_events.is_empty());
}

#[test]
fn ux_gamble_card_does_not_reveal_unknown_properties() {
    let app=app_with_test_controls();
    let lines=app.item_card_lines(&"core:rifle".parse().unwrap(),None,true);
    assert_eq!(lines.len(),2);
    assert!(lines[0].contains("non identifié"));
    assert!(!lines.join(" ").contains("Dégâts"));
}

#[test]
fn ux_help_scroll_and_tabs_do_not_close_on_an_unrelated_click() {
    let mut app=app_with_test_controls();app.update_input(&input("F1"));
    let before=suspension::fingerprint(&app.game);
    app.update_input(&InputFrame{pressed:[Binding::MouseLeft].into(),pointer:Some((5.0,5.0)),viewport:Some((960.0,540.0)),..Default::default()});
    assert!(app.legend_open);
    app.update_input(&input("Right"));assert_eq!(app.ux.help_tab,1);
    app.ux.help_scroll.maximum.set(1000.0);app.update_input(&input("PageDown"));assert!(app.ux.help_scroll.offset>0.0);
    app.update_input(&input("Left"));assert_eq!(app.ux.help_scroll.offset,0.0);
    assert_eq!(suspension::fingerprint(&app.game),before);
}

#[test]
fn ux_hud_mouse_buttons_follow_rebound_actions_without_advancing_time() {
    let mut app=app_with_test_controls();
    app.controls.rebind(Action::Legend,Binding::key("F8")).unwrap();
    let before=(suspension::fingerprint(&app.game),app.history.len());
    let rect=AsciiApp::hud_shortcuts(1280.0,800.0).iter().find(|(_,a,_,_)|*a==Some(Action::Legend)).unwrap().0;
    app.update_input(&rect_pointer(rect,0.0));assert!(app.legend_open);
    app.update_input(&input("Escape"));
    assert_eq!((suspension::fingerprint(&app.game),app.history.len()),before);
}

#[test]
fn ux_chosen_quest_is_persistent_and_navigation_is_free() {
    let mut app=app_with_test_controls();app.prepare_exploration_quest_diagnostic().unwrap();
    app.open_quest_journal();
    let before=(suspension::fingerprint(&app.game),app.history.len());
    let id=app.journal_entries()[app.quest_journal_selection].quest.id.clone();
    app.update_input(&input("Enter"));assert_eq!(app.ux.tracked_quest,Some(id.clone()));
    assert_eq!(app.suspension().unwrap().tracked_quest,Some(id.to_string()));
    assert_eq!((suspension::fingerprint(&app.game),app.history.len()),before);
}

#[test]
fn ux_end_screen_mouse_and_keyboard_open_history_without_underlying_hud_actions() {
    let mut app=app_with_test_controls();app.prepare_end_diagnostic().unwrap();
    let before=suspension::fingerprint(&app.game);
    app.update_input(&rect_pointer(AsciiApp::end_actions(1280.0,800.0)[2],0.0));
    assert!(app.ux.history_open);assert!(!app.inventory_open);
    app.update_input(&input("Escape"));
    app.update_input(&input("Down"));app.update_input(&input("Enter"));assert!(app.ux.history_open);
    assert_eq!(suspension::fingerprint(&app.game),before);
}
