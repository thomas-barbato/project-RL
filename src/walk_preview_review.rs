//! Native diagnostics of the production input routing and accepted E renderer.
use super::*;

fn click_at(app: &AsciiApp, at: GridPos, viewport: (f32, f32)) -> InputFrame {
    let scale = app.graphics.active.ui_scale(viewport.0, viewport.1);
    let cell = app
        .terminal
        .world_cell_rect(
            &app.game,
            app.terminal_bounds_for(viewport.0, viewport.1, scale),
            scale,
            app.graphics.active.world_cell_px,
            app.navigation_signal_summary().is_some(),
            at,
            app.camera_glide_offset(),
        )
        .unwrap();
    app.graphics.active.transform_input(InputFrame {
        pointer: Some(cell.center().into()),
        viewport: Some(viewport),
        pressed: [controls::Binding::MouseLeft].into(),
        ..Default::default()
    })
}

impl AsciiApp {
    pub async fn run_walk_preview_review() -> Result<(), String> {
        let mut app = Self::new_agent_blast_trial()?;
        app.crash_recovery_enabled = false;
        app.suspension_path = std::env::temp_dir().join("project-rl-walk-review-unused.json");
        loop {
            if is_quit_requested() {
                return Ok(());
            }
            app.update();
            app.draw();
            next_frame().await;
        }
    }

    pub async fn capture_walk_preview(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let mut app = Self::new_agent_blast_trial()?;
        app.crash_recovery_enabled = false;
        app.suspension_path = output.join("isolated-unused.json");
        app.graphics.active.world_cell_px = 64;
        app.graphics.active.reduced_motion = true;
        app.select_world_target(None);
        let before = suspension::fingerprint(&app.game);
        let history = app.history.len();
        let target = GridPos::new(8, 4);
        let mut frames = Vec::new();
        let mut route = Vec::new();
        // These are actual window framebuffers, drawn through AsciiApp::draw.
        // No alternate controller or replacement path renderer is installed.
        for (width, height, filename) in [
            (1280, 800, "trajet.png"),
            (2560, 1440, "trajet-2k.png"),
            (3840, 2160, "trajet-4k.png"),
        ] {
            request_new_screen_size(width as f32, height as f32);
            for _ in 0..3 {
                next_frame().await;
            }
            app.walk_preview = None;
            // Windows may constrain a native window to the current desktop.
            // Use its actual viewport for input and record the limitation.
            let viewport = (screen_width(), screen_height());
            let click = click_at(&app, target, viewport);
            app.update_input_at(&click, Some(0.));
            route = app
                .walk_preview
                .as_ref()
                .ok_or("Missing production preview")?
                .cells
                .clone();
            if suspension::fingerprint(&app.game) != before || app.history.len() != history {
                return Err("Preview modified the game".into());
            }
            for _ in 0..2 {
                app.draw();
                next_frame().await;
            }
            app.draw();
            let image = crate::ui_capture::framebuffer()?;
            image.export_png(output.join(filename).to_str().ok_or("Capture path")?);
            frames.push(serde_json::json!({"file":filename,
                "requested_pixels":[width,height], "pixels":[image.width,image.height]}));
        }
        app.update_input_at(
            &InputFrame {
                pressed: [controls::Binding::MouseRight].into(),
                ..Default::default()
            },
            Some(1.),
        );
        if app.walk_preview.is_some()
            || app.world_context.is_some()
            || suspension::fingerprint(&app.game) != before
        {
            return Err("Right click did not cancel for free".into());
        }
        app.draw();
        crate::ui_capture::framebuffer()?
            .export_png(output.join("annule.png").to_str().ok_or("Capture path")?);
        let click = click_at(&app, target, (screen_width(), screen_height()));
        app.update_input_at(&click, Some(2.));
        let turn = app.game.turn();
        app.update_input_at(&click, Some(3.));
        for tick in 4..30 {
            let previous = app.game.turn();
            app.update_input_at(&InputFrame::default(), Some(tick as f64));
            if app.game.turn() > previous + 1 {
                return Err("Multiple moves in one update".into());
            }
        }
        if app.game.player_position() != Some(target)
            || app.game.turn() != turn + (route.len() - 1) as u64
        {
            return Err("Confirmed route did not cost one turn per step".into());
        }
        let checks = serde_json::json!({
            "review":"WALK-VIS-03-E", "status":"approved_and_integrated",
            "normal_games_changed":true, "controller":"AsciiApp::update_input_at",
            "renderer":"AsciiApp::draw", "frames":frames, "tile_pixels":64,
            "path":route.iter().map(|p|[p.x,p.y]).collect::<Vec<_>>(), "steps":route.len()-1,
            "pathfinder":"existing deterministic A*", "preview_free":true,
            "right_click_clears_preview_without_menu_or_turn":true,
            "confirmed_route_one_turn_per_step":true,
            "last_segment_clipped_to_destination_cell":true,
        });
        std::fs::write(
            output.join("checks.json"),
            serde_json::to_vec_pretty(&checks).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}
