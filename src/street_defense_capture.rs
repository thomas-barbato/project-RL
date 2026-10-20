//! Opt-in native proposal. Existing game, saves, collisions and actors unchanged.
use super::*;
impl AsciiApp {
    pub async fn capture_street_defense(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (mut rules, texts, loot, expeditions) = ascii_game_content()?;
        rules.player_inventory_capacity = 96;
        let mut app = Self::from_seed_version(
            1872,
            rules,
            texts,
            loot,
            expeditions,
            CURRENT_GENERATION_VERSION,
        )?;
        app.crash_recovery_enabled = false;
        app.suspension_path = output.join("isolated-run.json");
        // Borderless fullscreen can become a 1px framebuffer on focus loss.
        // Keep this opt-in capture windowed without writing player preferences.
        macroquad::miniquad::window::set_fullscreen(false);
        request_new_screen_size(1920., 1080.);
        for _ in 0..3 {
            next_frame().await;
        }
        let exit = pos(camp::scene().exits[0]);
        app.walk_fixture_to_unchecked(exit)?;
        if app.execute_command(GameCommand::Interact { target: exit }) != CommandOutcome::Applied {
            return Err("Street defense review could not enter street".into());
        }
        let item = app.game.grant_colored_equipment_for_diagnostic(
            "core:tenue_assaut".parse().unwrap(),
            project_rl::entity::EquipmentPalette::Original,
        )?;
        app.execute_command(GameCommand::EquipItem {
            slot: "core:body_armor".parse().unwrap(),
            item,
        });
        let data = crate::street_defense_review::data();
        let mut proposed = app.game.map().clone();
        for &at in &data.proposed_solid_cells {
            proposed
                .set_obstruction(GridPos::new(at[0], at[1]), true, false)
                .map_err(|e| e.to_string())?;
        }
        for &at in &data.central_free_cells {
            if !proposed.is_walkable(GridPos::new(at[0], at[1])) {
                return Err(format!("Proposal obstructs central lane {at:?}"));
            }
        }
        let route = project_rl::world::find_path(
            &proposed,
            GridPos::new(7, 62),
            GridPos::new(8, 55),
            256,
            |_| true,
        )
        .ok_or("Proposal seals prison exit")?;
        let baseline_route = project_rl::world::find_path(
            app.game.map(),
            GridPos::new(7, 62),
            GridPos::new(8, 55),
            256,
            |_| true,
        )
        .ok_or("Existing prison approach is disconnected")?;
        if route.len() != baseline_route.len() {
            return Err("Added obstacles force a detour on the direct prison approach".into());
        }
        let mut circulation_routes = Vec::new();
        for (name, destination) in [
            ("west_flank", GridPos::new(6, 57)),
            ("east_flank", GridPos::new(10, 56)),
            ("next_map", GridPos::new(18, 1)),
        ] {
            let path = project_rl::world::find_path(
                &proposed,
                GridPos::new(7, 62),
                destination,
                4096,
                |_| true,
            )
            .ok_or_else(|| format!("Added obstacles seal route {name}"))?;
            circulation_routes.push(serde_json::json!({"id":name,"steps":path.len()}));
        }
        let mut scenes = Vec::new();
        for (name, at) in [
            ("gate", [8, 61]),
            ("approach", [8, 57]),
            ("south-fire", [14, 58]),
            ("van-fire", [18, 32]),
            ("west-fire", [6, 23]),
            ("court-fire", [15, 40]),
        ] {
            app.walk_fixture_to_unchecked(GridPos::new(at[0], at[1]))?;
            app.selected_world_cell = None;
            app.selected_target = None;
            app.world_cursor = None;
            for (suffix, candidate) in [("before", false), ("proposal", true)] {
                crate::street_defense_review::enable(candidate);
                app.world_fix_frame(output, &format!("{name}-{suffix}"))
                    .await?;
            }
            if name == "gate" || name == "south-fire" {
                // A short real-engine loop includes breathing, flames and particles.
                let before_motion = suspension::fingerprint(&app.game);
                for i in 0..12 {
                    let time = 1. + i as f64 * 0.125;
                    let _clock = crate::effect_geometry::frame_time(time);
                    app.world_presentation_time.set(time);
                    for _ in 0..2 {
                        app.draw();
                        next_frame().await;
                    }
                    app.draw();
                    crate::ui_capture::framebuffer()?.export_png(
                        output
                            .join(format!("{name}-motion-{i:02}.png"))
                            .to_str()
                            .unwrap(),
                    );
                }
                if before_motion != suspension::fingerprint(&app.game) {
                    return Err("Defense motion preview mutated the game".into());
                }
                if name == "gate" {
                    crate::prison_street_render::export_review_world(
                        &app.game,
                        &output.join("map-proposal.png"),
                    )?;
                }
            }
            scenes.push(serde_json::json!({"id":name,"player":at}));
        }
        crate::street_defense_review::enable(false);
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "review":"DEFENSE-U01-10-r05","status":"visual_proposal_only","native_cell":64,
            "live_guards_installed":false,"new_damage_or_ai":false,"production_map_changed":false,
            "user_saves_touched":false,"render_read_only":true,"new_corpses":21,"new_blood_props":11,
            "guards":2,"sandbag_positions":2,"extra_obstacles":0,"fires":4,"animated_frames":24,
            "cover_shape":"L","armor":"light silver steel","feet_offset_keeps_guards_off_pillars":true,
            "central_lane_clear":true,"reserved_street_width_cells":3,
            "candidate_obstacle_route_len":route.len(),"baseline_approach_route_len":baseline_route.len(),
            "no_added_detour":true,"circulation_routes":circulation_routes,"scenes":scenes
        })).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }
}
