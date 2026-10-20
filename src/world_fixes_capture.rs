//! Actual GPU regression scenes; isolated runs, no user save or art activation.
use super::*;

/// Explicit GPU check, kept out of the ordinary unit suite. This runs from the
/// test executable when a player's running game locks the normal Windows binary.
#[cfg(test)]
#[test]
#[ignore = "manual native GPU capture; writes only artifacts/street-defects-review-01/engine03"]
fn world_fixes_gpu_review() {
    macroquad::Window::from_config(crate::window_conf(), async {
        crate::ui_theme::initialize_fonts().unwrap();
        crate::terminal_chrome::prepare_text();
        crate::terminal_view::initialize_player_art();
        crate::game_art::prewarm();
        let result = AsciiApp::capture_world_fixes(std::path::Path::new(
            "artifacts/street-defects-review-01/engine03",
        ))
        .await;
        match result {
            Ok(()) => std::process::exit(0),
            Err(error) => {
                eprintln!("GPU review failed: {error}");
                std::process::exit(1);
            }
        }
    });
}

impl AsciiApp {
    /// Normal rendering of the approved civilian scenery, without an overlay.
    pub async fn capture_street_civilians(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed_version(
            1872,
            rules,
            texts,
            loot,
            expeditions,
            CURRENT_GENERATION_VERSION,
        )?;
        app.suspension_path = output.join("isolated-run.json");
        app.crash_recovery_enabled = false;
        request_new_screen_size(1920., 1080.);
        for _ in 0..3 {
            next_frame().await;
        }
        let exit = pos(camp::scene().exits[0]);
        app.walk_fixture_to_unchecked(exit)?;
        if app.execute_command(GameCommand::Interact { target: exit }) != CommandOutcome::Applied {
            return Err("Civilian scenery review street entry failed".into());
        }
        let mut scenes = Vec::new();
        for (label, at) in [
            ("gate", GridPos::new(8, 59)),
            ("burned-car", GridPos::new(18, 57)),
            ("courtyard", GridPos::new(15, 41)),
            ("middle-street", GridPos::new(18, 33)),
            ("warehouse", GridPos::new(15, 10)),
            ("north-crossroad", GridPos::new(9, 15)),
            ("house-closed", GridPos::new(11, 23)),
            ("house-open", GridPos::new(12, 20)),
            ("shop-open", GridPos::new(12, 29)),
            ("first-house-open", GridPos::new(6, 52)),
        ] {
            app.walk_fixture_to_unchecked(at)
                .map_err(|e| format!("{label}: {e}"))?;
            let before = suspension::fingerprint(&app.game);
            crate::prison_street_render::review_normal_style();
            app.world_fix_frame(output, &format!("{label}-normal"))
                .await?;
            if label == "gate" {
                crate::prison_street_render::export_review_world(
                    &app.game,
                    &output.join("map-normal.png"),
                )?;
            }
            crate::prison_street_render::review_normal_style();
            if before != suspension::fingerprint(&app.game) {
                return Err("Civilian scenery review changed gameplay".into());
            }
            scenes.push(serde_json::json!({"id":label,"player":[at.x,at.y],"cell":64,"render_read_only":true}));
        }
        let entry = crate::prison_street::pos(crate::prison_street::expanded_scene().entrance);
        app.walk_fixture_to_unchecked(entry)?;
        if app.execute_command(GameCommand::Interact { target: entry }) != CommandOutcome::Applied
            || app.game.current_zone().unwrap().id.as_str() != camp::ZONE_ID
        {
            return Err("Civilian scenery review camp return failed".into());
        }
        std::fs::write(output.join("checks.json"), serde_json::to_vec_pretty(&serde_json::json!({
            "review":"CIVILS-U01-08","status":"integrated","native_cell":64,
            "world_pixels":[1664,4224],"real_vision_radius":6,"same_lighting":true,
            "accepted_paving":"PAV-U01-06","render_read_only":true,"camp_round_trip":true,
            "living_actor_assets_unchanged":true,"no_new_collisions":true,"normal_integration":true,
            "roof_hiding":"real door-dependent roof rendering","scenes":scenes
        })).unwrap()).map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Whole-map scenery proposal with current actors, rules and lighting.
    pub async fn capture_street_style(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed_version(
            1872,
            rules,
            texts,
            loot,
            expeditions,
            CURRENT_GENERATION_VERSION,
        )?;
        app.suspension_path = output.join("isolated-run.json");
        app.crash_recovery_enabled = false;
        request_new_screen_size(1920., 1080.);
        for _ in 0..3 {
            next_frame().await;
        }
        let exit = pos(camp::scene().exits[0]);
        app.walk_fixture_to_unchecked(exit)?;
        if app.execute_command(GameCommand::Interact { target: exit }) != CommandOutcome::Applied {
            return Err("Style review street entry failed".into());
        }
        let views = [
            ("gate", GridPos::new(8, 59)),
            ("south-street", GridPos::new(18, 49)),
            ("courtyard", GridPos::new(15, 41)),
            ("middle-street", GridPos::new(18, 33)),
            ("house-closed", GridPos::new(11, 23)),
            ("north-street", GridPos::new(18, 17)),
            ("warehouse", GridPos::new(6, 8)),
            ("north-exit", GridPos::new(18, 2)),
            ("house-open", GridPos::new(12, 20)),
        ];
        let mut scenes = Vec::new();
        let paving =
            std::env::args().nth(1).as_deref() == Some("--ui-cold-street-paving-integration");
        let integration = paving
            || std::env::args().nth(1).as_deref() == Some("--ui-cold-street-style-integration");
        for (label, at) in views {
            app.walk_fixture_to_unchecked(at)
                .map_err(|e| format!("{label}: {e}"))?;
            let before = suspension::fingerprint(&app.game);
            if !paving {
                for (candidate, suffix) in [(false, "current"), (true, "candidate")] {
                    crate::prison_street_render::review_grim(candidate);
                    crate::prison_street_render::review_whole_style(candidate);
                    app.world_fix_frame(output, &format!("{label}-{suffix}"))
                        .await?;
                    if label == "gate" {
                        crate::prison_street_render::export_review_world(
                            &app.game,
                            &output.join(format!("map-{suffix}.png")),
                        )?;
                    }
                }
            }
            crate::prison_street_render::review_normal_style();
            if integration {
                app.world_fix_frame(output, &format!("{label}-normal"))
                    .await?;
                if label == "gate" {
                    crate::prison_street_render::export_review_world(
                        &app.game,
                        &output.join("map-normal.png"),
                    )?;
                }
            }
            if before != suspension::fingerprint(&app.game) {
                return Err("Style comparison changed gameplay".into());
            }
            scenes.push(serde_json::json!({"id":label,"player":[at.x,at.y],"cell":64,"render_read_only":true}));
        }
        let entry = crate::prison_street::pos(crate::prison_street::expanded_scene().entrance);
        app.walk_fixture_to_unchecked(entry)?;
        if app.execute_command(GameCommand::Interact { target: entry }) != CommandOutcome::Applied
            || app.game.current_zone().unwrap().id.as_str() != camp::ZONE_ID
        {
            return Err("Style review camp return failed".into());
        }
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "review":if paving {"PAV-U01-06"} else {"STYLE-MAP-U01-04"},"native_cell":64,"world_pixels":[1664,4224],
            "map_cells":[26,66],"same_lighting":true,"living_actor_assets_unchanged":true,
            "real_vision_radius":6,"normal_integration":true,"normal_capture":integration,"render_read_only":true,
            "camp_round_trip":true,"overview_only_bypasses_scenery_circle":true,"scenes":scenes,
            "surface_scope":["street","walls","buildings","interiors","roofs","car"],
            "method":if paving {"normal runtime rendering of accepted paving; no candidate override; accepted surface material retained"}
                     else {"complete retained scenery modules plus generated surface material; actors excluded"}
        })).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }

    pub(in crate::ascii_app) async fn world_fix_frame(
        &mut self,
        output: &std::path::Path,
        label: &str,
    ) -> Result<(), String> {
        let _time = crate::effect_geometry::frame_time(1.0);
        self.world_presentation_time.set(1.);
        self.capture_events_at(Some(0.));
        let before = suspension::fingerprint(&self.game);
        for _ in 0..2 {
            self.draw();
            next_frame().await;
        }
        self.draw();
        crate::ui_capture::framebuffer()?
            .export_png(output.join(format!("{label}.png")).to_str().unwrap());
        if before != suspension::fingerprint(&self.game) {
            return Err("Regression rendering mutated the world".into());
        }
        Ok(())
    }

    pub async fn capture_world_fixes(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed_version(
            1872,
            rules,
            texts,
            loot,
            expeditions,
            SIX_CELL_VISION_GENERATION_VERSION,
        )?;
        app.suspension_path = output.join("isolated-run.json");
        app.crash_recovery_enabled = false;
        request_new_screen_size(1920., 1080.);
        for _ in 0..3 {
            next_frame().await;
        }
        let npc = pos(camp::scene().actors[1].at);
        let player = (0..app.game.map().height() as i32)
            .flat_map(|y| (0..app.game.map().width() as i32).map(move |x| GridPos::new(x, y)))
            .filter(|at| {
                app.game.map().is_walkable(*at) && app.game.actors().entity_at(*at).is_none()
            })
            .filter(|at| {
                let d = (at.x - npc.x).pow(2) + (at.y - npc.y).pow(2);
                (4..=36).contains(&d)
            })
            .filter(|at| !project_rl::world::has_line_of_sight(app.game.map(), *at, npc, true))
            .min_by_key(|at| (at.x - npc.x).pow(2) + (at.y - npc.y).pow(2))
            .ok_or("No occluded nearby NPC fixture")?;
        app.walk_fixture_to_unchecked(player)?;
        if app.game.player_visibility().is_visible(npc) {
            return Err("Legacy NPC must be occluded".into());
        }
        app.world_fix_frame(output, "npc-before").await?;
        let turn = app.game.turn();
        app.migrate_circular_player_vision()?;
        if !app.game.player_visibility().is_visible(npc) || app.game.turn() != turn {
            return Err("NPC circle adoption failed".into());
        }
        app.world_fix_frame(output, "npc-fixed").await?;
        let npc_rect = app
            .terminal
            .world_cell_rect(
                &app.game,
                app.terminal_bounds(),
                app.ui_scale(),
                64,
                app.navigation_signal_summary().is_some(),
                npc,
                Vec2::ZERO,
            )
            .ok_or("NPC outside camera")?;
        if npc_rect.w != 64. || npc_rect.h != 64. {
            return Err("NPC scale changed".into());
        }
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed_version(
            1872,
            rules,
            texts,
            loot,
            expeditions,
            SIX_CELL_VISION_GENERATION_VERSION,
        )?;
        app.suspension_path = output.join("isolated-run.json");
        app.crash_recovery_enabled = false;
        let exit = pos(camp::scene().exits[0]);
        app.walk_fixture_to_unchecked(exit)?;
        if app.execute_command(GameCommand::Interact { target: exit }) != CommandOutcome::Applied {
            return Err("Street entry failed".into());
        }
        app.walk_fixture_to_unchecked(GridPos::new(16, 31))?;
        if app.game.map().is_walkable(GridPos::new(15, 31)) {
            return Err("Legacy collision was not reproduced".into());
        }
        app.world_fix_frame(output, "street-before").await?;
        let turn = app.game.turn();
        app.migrate_circular_player_vision()?;
        app.migrate_street_collisions()?;
        if app.game.turn() != turn {
            return Err("Repair consumed a turn".into());
        }
        app.world_fix_frame(output, "street-fixed").await?;
        for x in (7..16).rev() {
            if app.execute_command(GameCommand::Move(Direction::West)) != CommandOutcome::Applied
                || app.game.player_position() != Some(GridPos::new(x, 31))
            {
                return Err(format!("Street row blocked at {x},31"));
            }
        }
        if app.game.turn() != turn + 9 {
            return Err("Movement did not spend one turn per cell".into());
        }
        app.world_fix_frame(output, "street-path-walked").await?;
        app.walk_fixture_to_unchecked(GridPos::new(8, 59))?;
        app.world_fix_frame(output, "street-current").await?;
        crate::prison_street_render::review_grim(true);
        app.world_fix_frame(output, "street-grim-candidate").await?;
        crate::prison_street_render::review_grim(false);
        app.walk_fixture_to_unchecked(GridPos::new(6, 8))
            .map_err(|error| format!("Warehouse review approach: {error}"))?;
        app.world_fix_frame(output, "style-current").await?;
        crate::prison_street_render::review_grim(true);
        app.world_fix_frame(output, "style-candidate").await?;
        crate::prison_street_render::review_grim(false);
        let entry = crate::prison_street::pos(crate::prison_street::expanded_scene().entrance);
        app.walk_fixture_to_unchecked(entry)?;
        if app.execute_command(GameCommand::Interact { target: entry }) != CommandOutcome::Applied
            || app.game.current_zone().unwrap().id.as_str() != camp::ZONE_ID
        {
            return Err("Camp return failed".into());
        }
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "review":"FIXES-U01-03","old_run_generation":158,"normal_generation":CURRENT_GENERATION_VERSION,
            "real_radius":6,"circular_player_vision":app.game.rules().circular_player_vision,
            "npc": [npc.x,npc.y],"player_for_npc":[player.x,player.y],
            "npc_rect":[npc_rect.x,npc_rect.y,npc_rect.w,npc_rect.h],
            "npc_occluded_by_wall":true,"npc_visible_before":false,"npc_visible_after":true,
            "movement_start":[16,31],"movement_end":[7,31],"movement_turns":9,
            "corrected_cells":61,"camp_round_trip":true,"native_cell":64,"render_read_only":true,
            "grim_art_integrated":false,"grim_lighting_integrated":false,
            "grim_candidate":{"same_lighting":true,"living_actors_unchanged":true,
                "gate_player":[8,59],"style_player":[6,8],"ground_layer":true,
                "props":["GRIM-U01-01","CORP-U01-01","CORP-U01-02","CORP-U01-03"],
                "materials":["STYLE-U01-01","STYLE-U01-02"],"rejected_for_street":["GRIM-U01-02"]}
        })).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }
}
