// Included in ascii_app::tests to exercise the same input route as the game.
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
