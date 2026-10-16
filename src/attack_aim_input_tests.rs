use super::*;

fn pointer_frame(app: &AsciiApp, at: GridPos, viewport: (f32, f32)) -> InputFrame {
    let scale = app.graphics.active.ui_scale(viewport.0, viewport.1);
    let rect = app
        .terminal
        .world_cell_rect(
            &app.game,
            app.terminal_bounds_for(viewport.0, viewport.1, scale),
            scale,
            app.graphics.active.world_cell_px,
            app.navigation_signal_summary().is_some(),
            at,
        )
        .expect("fixture cell must fit in the camera");
    app.graphics.active.transform_input(InputFrame {
        pointer: Some(rect.center().into()),
        viewport: Some(viewport),
        ..Default::default()
    })
}

fn skill_fixture(id: &str) -> AsciiApp {
    let mut app = match id {
        "A01" => AsciiApp::new_agent_weapon_switch_trial(),
        "A02" => AsciiApp::new_agent_blast_trial(),
        "B01" => AsciiApp::new_agent_melee_trial(),
        "FE01" => AsciiApp::new_agent_ember_trial(),
        "FE05" => AsciiApp::new_agent_fireball_trial(),
        _ => AsciiApp::new_agent_drone_trial(),
    }
    .unwrap();
    app.skills_open = false;
    app
}

#[test]
fn hover_moves_all_connected_skill_previews_without_click_or_simulation_change() {
    for id in ["A01", "A02", "B01", "D58", "D59", "D66", "FE01", "FE05"] {
        let mut app = skill_fixture(id);
        app.begin_technique_attack_aim(project_rl::skills::agent::technique(id));
        let before = app.game.recovery_snapshot_bytes().unwrap();
        let history = app.history.len();
        for at in [GridPos::new(5, 3), GridPos::new(6, 4), GridPos::new(5, 5)] {
            let frame = pointer_frame(&app, at, (1280., 800.));
            assert!(frame.pressed.is_empty());
            app.update_input(&frame);
            assert_eq!(app.attack_aim.unwrap().cursor, at, "{id}");
            assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before, "{id}");
            assert_eq!(app.history.len(), history, "{id}");
        }
    }
}

#[test]
fn keyboard_owns_preview_until_mouse_moves_again() {
    let mut app = AsciiApp::new_agent_blast_trial().unwrap();
    app.begin_technique_attack_aim(project_rl::skills::agent::technique("A02"));
    let hover = pointer_frame(&app, GridPos::new(6, 4), (1280., 800.));
    app.update_input(&hover);
    let before = app.game.recovery_snapshot_bytes().unwrap();
    let mut key = hover.clone();
    key.pressed
        .insert(app.controls.binding(Action::MoveNorth).clone());
    app.update_input(&key);
    assert_eq!(app.attack_aim.unwrap().cursor, GridPos::new(6, 3));
    for _ in 0..3 {
        app.update_input(&hover);
    }
    assert_eq!(app.attack_aim.unwrap().cursor, GridPos::new(6, 3));
    key.pressed = [app.controls.binding(Action::CycleTarget).clone()].into();
    app.update_input(&key);
    let cycled = app.attack_aim.unwrap().cursor;
    app.update_input(&hover);
    assert_eq!(app.attack_aim.unwrap().cursor, cycled);
    let moved = pointer_frame(&app, GridPos::new(5, 5), (1280., 800.));
    app.update_input(&moved);
    assert_eq!(app.attack_aim.unwrap().cursor, GridPos::new(5, 5));
    assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
}

#[test]
fn hover_uses_same_camera_at_native_tile_sizes_and_ui_scales() {
    let mut app = AsciiApp::new_agent_blast_trial().unwrap();
    app.begin_technique_attack_aim(project_rl::skills::agent::technique("A02"));
    for viewport in [(960., 540.), (1280., 800.), (3840., 2160.)] {
        for cell in [32, 64] {
            for percent in [75, 100, 150] {
                app.graphics.active.world_cell_px = cell;
                app.graphics.active.ui_scale_percent = percent;
                for at in [GridPos::new(4, 4), GridPos::new(5, 4)] {
                    let frame = pointer_frame(&app, at, viewport);
                    app.update_input(&frame);
                    assert_eq!(
                        app.attack_aim.unwrap().cursor,
                        at,
                        "{viewport:?} {cell} {percent}"
                    );
                }
            }
        }
    }
    assert_eq!(app.game.turn(), 0);
}

#[test]
fn hover_changes_area_and_hud_confirmation_keeps_last_world_cell() {
    let mut app = AsciiApp::new_agent_blast_trial().unwrap();
    app.begin_technique_attack_aim(project_rl::skills::agent::technique("A02"));
    let initial = app.aimed_attack_footprint(app.attack_aim.unwrap()).unwrap();
    let at = GridPos::new(7, 5);
    let hover = pointer_frame(&app, at, (1280., 800.));
    app.update_input(&hover);
    assert_eq!(
        app.aimed_attack_footprint(app.attack_aim.unwrap())
            .unwrap()
            .target(),
        at
    );
    assert_ne!(initial.target(), at);
    let mut hud = InputFrame {
        pointer: Some(
            AsciiApp::attack_aim_confirm_rect(1280., 800.)
                .center()
                .into(),
        ),
        viewport: Some((1280., 800.)),
        ..Default::default()
    };
    app.update_input(&hud);
    assert_eq!(app.attack_aim.unwrap().cursor, at);
    assert_eq!(app.game.turn(), 0);
    hud.pressed.insert(controls::Binding::MouseLeft);
    app.update_input(&hud);
    assert!(app.attack_aim.is_none());
    assert_eq!(app.game.turn(), 1);
}

#[test]
fn hidden_cells_and_off_map_hover_keep_last_preview() {
    let mut app = AsciiApp::new_agent_blast_trial().unwrap();
    // Narrow the live FOV, but retain the terminal's previously known terrain.
    let mut rules = app.game.rules().clone();
    rules.player_field_of_view.radius = 2;
    app.game = WorldState::from_recovery_snapshot_bytes(
        &app.game.recovery_snapshot_bytes().unwrap(),
        rules,
    )
    .unwrap();
    assert_eq!(
        app.game
            .process_player_command(GameCommand::Move(Direction::West)),
        CommandOutcome::Applied
    );
    app.game.drain_events();
    app.begin_technique_attack_aim(project_rl::skills::agent::technique("A02"));
    let hover = pointer_frame(&app, GridPos::new(3, 5), (1280., 800.));
    app.update_input(&hover);
    let initial = app.attack_aim.unwrap();
    let before = app.game.recovery_snapshot_bytes().unwrap();
    for pointer in [Some((0., 0.)), Some((1279., 799.)), None] {
        for pressed in [Default::default(), [controls::Binding::MouseLeft].into()] {
            app.update_input(&InputFrame {
                pointer,
                pressed,
                viewport: Some((1280., 800.)),
                ..Default::default()
            });
            assert_eq!(app.attack_aim.unwrap(), initial);
        }
    }
    let hidden = GridPos::new(6, 4);
    assert!(!app.game.player_visibility().is_visible(hidden));
    let mut frame = pointer_frame(&app, hidden, (1280., 800.));
    frame.pressed.insert(controls::Binding::MouseLeft);
    app.update_input(&frame);
    assert_eq!(app.attack_aim.unwrap(), initial);
    assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
}

#[test]
fn left_click_confirms_each_connected_skill_once_at_the_hovered_cell() {
    for (id, at) in [
        ("A01", GridPos::new(7, 4)),
        ("A02", GridPos::new(7, 5)),
        ("B01", GridPos::new(5, 4)),
        ("D58", GridPos::new(5, 3)),
        ("D59", GridPos::new(5, 3)),
        ("D66", GridPos::new(5, 3)),
        ("FE01", GridPos::new(5, 4)),
        ("FE05", GridPos::new(8, 4)),
    ] {
        let mut app = skill_fixture(id);
        let technique = project_rl::skills::agent::technique(id);
        app.begin_technique_attack_aim(technique.clone());
        let mut click = pointer_frame(&app, at, (1280., 800.));
        app.update_input(&click);
        assert_eq!(app.game.turn(), 0, "{id}");
        assert!(
            app.aimed_attack_preview(app.attack_aim.unwrap()).is_ok(),
            "{id}"
        );
        click.pressed.insert(controls::Binding::MouseLeft);
        click.mouse_left_down = true;
        app.update_input(&click);
        assert_eq!(app.game.turn(), 1, "{id}");
        assert!(app.attack_aim.is_none(), "{id}");
        assert!(
            matches!(app.history.last(), Some(RecordedCommand::TechniqueAt {
            technique: used, x, y, ..
        }) if *used == technique.to_string() && *x == at.x && *y == at.y),
            "{id}"
        );
        let history = app.history.len();
        click.pressed.clear();
        app.update_input(&click); // Holding the button must never cast twice.
        assert_eq!(app.game.turn(), 1, "{id}");
        assert_eq!(app.history.len(), history, "{id}");
    }
}

#[test]
fn invalid_map_click_does_not_fire_previous_target_and_a_valid_click_can_retarget_and_cast() {
    let mut app = skill_fixture("A01");
    app.begin_technique_attack_aim(project_rl::skills::agent::technique("A01"));
    assert!(app.aimed_attack_preview(app.attack_aim.unwrap()).is_ok());
    let before = app.game.recovery_snapshot_bytes().unwrap();
    let mut click = pointer_frame(&app, GridPos::new(6, 5), (1280., 800.));
    click.pressed.insert(controls::Binding::MouseLeft);
    app.update_input(&click);
    assert_eq!(app.attack_aim.unwrap().cursor, GridPos::new(6, 5));
    assert!(app.aimed_attack_preview(app.attack_aim.unwrap()).is_err());
    assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
    let at = GridPos::new(7, 4);
    let mut click = pointer_frame(&app, at, (1280., 800.));
    click.pressed.insert(controls::Binding::MouseLeft);
    app.update_input(&click); // No preceding hover frame is required.
    assert_eq!(app.game.turn(), 1);
    assert!(app.attack_aim.is_none());
}

#[test]
fn left_click_also_confirms_an_aimed_area_weapon_instead_of_deselecting_it() {
    let mut app = AsciiApp::new_mixed_weapon_trial().unwrap();
    app.active_weapon_slot = 2;
    let at = GridPos::new(6, 5);
    app.begin_attack_aim_at(Some(at), None, None);
    let mut click = pointer_frame(&app, at, (1280., 800.));
    app.update_input(&click);
    assert!(app.aimed_attack_preview(app.attack_aim.unwrap()).is_ok());
    click.pressed.insert(controls::Binding::MouseLeft);
    app.update_input(&click);
    assert_eq!(app.game.turn(), 1);
    assert!(app.attack_aim.is_none());
}
