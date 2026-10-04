//! Repeatable native rendering and action measurements, isolated from user saves/settings.
use super::*;
use std::time::Instant;

impl AsciiApp {
    pub async fn capture_lighting_performance(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(INITIAL_SEED, rules, texts, loot, expeditions)?;
        app.suspension_path = output.join("diagnostic-run.json");
        app.graphics.active.mode = WindowMode::Windowed;
        app.graphics.active.world_cell_px = 20;
        app.graphics.active.interface_theme = graphics::InterfaceTheme::Violet;
        request_new_screen_size(1280., 800.);
        for _ in 0..5 {
            next_frame().await;
        }
        let mut rows = Vec::new();
        for scene in ["recycling", "lighting-room"] {
            if scene == "lighting-room" {
                app.prepare_lighting_diagnostic()?;
            }
            let before = suspension::fingerprint(&app.game);
            let bytes_before = app.game.recovery_snapshot_bytes()?;
            let terminal_before = bincode::serialize(&app.terminal).map_err(|e| e.to_string())?;
            for repeat in 0..2 {
                for mode in if repeat == 0 {
                    ["off", "fixed", "dynamic"]
                } else {
                    ["dynamic", "fixed", "off"]
                } {
                    crate::terminal_view::set_debug_lighting(mode != "off", mode == "fixed", None);
                    let mut cpu = Vec::new();
                    let mut frames = Vec::new();
                    let mut lighting_cpu = Vec::new();
                    let mut rebuild_cpu = Vec::new();
                    let mut sources = 0;
                    let mut quads = 0;
                    for _ in 0..120 {
                        let start = Instant::now();
                        app.draw();
                        cpu.push(start.elapsed().as_secs_f64() * 1000.);
                        let (cost, rebuilt, count, drawn) =
                            crate::terminal_view::debug_lighting_cost();
                        lighting_cpu.push(cost);
                        if rebuilt {
                            rebuild_cpu.push(cost);
                        }
                        sources = count;
                        quads = drawn;
                        next_frame().await;
                        frames.push(start.elapsed().as_secs_f64() * 1000.);
                    }
                    let stats = |values: &[f64]| {
                        let mut sorted = values[10..].to_vec();
                        sorted.sort_by(f64::total_cmp);
                        serde_json::json!({"median_ms": sorted[sorted.len()/2], "p95_ms": sorted[sorted.len()*95/100]})
                    };
                    let row = serde_json::json!({"scene":scene,"mode":mode,"repeat":repeat,"cpu":stats(&cpu),"frame_with_present":stats(&frames),"lighting_cpu":stats(&lighting_cpu),"rebuild_ms":rebuild_cpu,"sources":sources,"floor_quads":quads});
                    eprintln!("[PERF LIGHT] {row}");
                    rows.push(row);
                }
            }
            if before != suspension::fingerprint(&app.game)
                || bytes_before != app.game.recovery_snapshot_bytes()?
                || terminal_before
                    != bincode::serialize(&app.terminal).map_err(|e| e.to_string())?
            {
                return Err(format!(
                    "{scene}: lights changed simulation or saved presentation"
                ));
            }
        }
        let movement_checks = app.measure_moving_lights(&mut rows).await?;
        crate::terminal_view::set_debug_lighting(true, false, None);
        let pixel_checks = app.verify_lighting_pixels(output).await?;
        std::fs::write(
            output.join("timings.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "build":"debug", "size":[screen_width(),screen_height()], "frames_per_mode":120,
            "warmup_frames":10, "simulation_and_snapshot_unchanged":true, "pixel_checks":pixel_checks, "movement_checks":movement_checks, "scenes":rows,
            }))
            .map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())
    }

    async fn measure_moving_lights(
        &mut self,
        rows: &mut Vec<serde_json::Value>,
    ) -> Result<serde_json::Value, String> {
        let mut final_states = Vec::new();
        let mut final_views = Vec::new();
        for enabled in [false, true] {
            self.prepare_lighting_diagnostic()?;
            crate::terminal_view::set_debug_lighting(enabled, false, None);
            let mut command_cpu = Vec::new();
            let mut render_cpu = Vec::new();
            let mut lighting_cpu = Vec::new();
            let mut rebuilds = 0;
            for step in 0..80 {
                let command_start = Instant::now();
                let direction = if step % 2 == 0 {
                    Direction::East
                } else {
                    Direction::West
                };
                if self
                    .game
                    .process_player_command(GameCommand::Move(direction))
                    != CommandOutcome::Applied
                {
                    return Err("Lighting movement fixture rejected a step".into());
                }
                self.terminal
                    .observe(self.game.map(), self.game.player_visibility());
                self.capture_events_at(Some(0.));
                command_cpu.push(command_start.elapsed().as_secs_f64() * 1000.);
                let render_start = Instant::now();
                self.draw();
                render_cpu.push(render_start.elapsed().as_secs_f64() * 1000.);
                let (cost, rebuilt, _, _) = crate::terminal_view::debug_lighting_cost();
                lighting_cpu.push(cost);
                rebuilds += usize::from(rebuilt);
                next_frame().await;
            }
            let stats = |values: &[f64]| {
                let mut sorted = values[10..].to_vec();
                sorted.sort_by(f64::total_cmp);
                serde_json::json!({"median_ms":sorted[sorted.len()/2],"p95_ms":sorted[sorted.len()*95/100]})
            };
            let row = serde_json::json!({"scene":"lighting-walk","mode":if enabled { "dynamic" } else { "off" },"steps":80,"command_cpu":stats(&command_cpu),"cpu":stats(&render_cpu),"lighting_cpu":stats(&lighting_cpu),"geometry_rebuilds":rebuilds,"autosave_enabled":false});
            eprintln!("[PERF LIGHT WALK] {row}");
            rows.push(row);
            final_states.push(self.game.recovery_snapshot_bytes()?);
            final_views.push(bincode::serialize(&self.terminal).map_err(|e| e.to_string())?);
        }
        if final_states[0] != final_states[1] || final_views[0] != final_views[1] {
            return Err("Lights changed movement state or remembered terrain".into());
        }
        Ok(
            serde_json::json!({"identical_engine_and_view_bytes_after_80_steps":true,"autosave_enabled":false}),
        )
    }

    async fn verify_lighting_pixels(
        &self,
        output: &std::path::Path,
    ) -> Result<serde_json::Value, String> {
        let mut images = Vec::new();
        for (name, enabled, reduced, time) in [
            ("off", false, false, 0.),
            ("dynamic-a", true, false, 0.),
            ("dynamic-b", true, false, 3.),
            ("reduced-a", true, true, 0.),
            ("reduced-b", true, true, 3.),
        ] {
            crate::terminal_view::set_debug_lighting(enabled, false, Some(time));
            for _ in 0..3 {
                self.terminal.draw_debug_light_mask(&self.game, reduced);
                next_frame().await;
            }
            let camera = self.terminal.draw_debug_light_mask(&self.game, reduced);
            let image = crate::ui_capture::framebuffer()?;
            image.export_png(
                output
                    .join(format!("mask-{name}.png"))
                    .to_str()
                    .ok_or("Chemin non UTF-8")?,
            );
            for y in 0..usize::from(image.height) {
                for x in 0..usize::from(image.width) {
                    let offset =
                        ((usize::from(image.height) - 1 - y) * usize::from(image.width) + x) * 4;
                    let pixel = &image.bytes[offset..offset + 3];
                    let at = camera.hit((x as f32 + 0.5, y as f32 + 0.5));
                    let visible = at.is_some_and(|at| self.game.player_visibility().is_visible(at));
                    if !visible && pixel.iter().any(|&channel| channel != 0) {
                        return Err(format!(
                            "{name}: light leaked into unknown or remembered terrain at {at:?}"
                        ));
                    }
                    if let Some(tile) = at.and_then(|at| self.terminal.known(at))
                        && tile.terrain.blocks_vision()
                        && !matches!(
                            tile.decor,
                            crate::test_sector::Decor::Server
                                | crate::test_sector::Decor::ElectricalCabinet
                                | crate::test_sector::Decor::DataTerminalOnline
                        )
                        && pixel.iter().any(|&channel| channel != 0)
                    {
                        return Err(format!(
                            "{name}: halo painted an opaque non-emitter at {at:?}"
                        ));
                    }
                }
            }
            images.push(image.bytes);
        }
        if images[1] == images[2] {
            return Err("Dynamic LEDs did not animate".into());
        }
        if images[3] != images[4] {
            return Err("Reduced motion lights still animated".into());
        }
        if images[0]
            .as_chunks::<4>()
            .0
            .iter()
            .any(|pixel| pixel[..3] != [0, 0, 0])
        {
            return Err("Disabled lighting mask was not black".into());
        }
        crate::terminal_view::set_debug_lighting(true, false, None);
        Ok(
            serde_json::json!({"no_memory_or_unknown_spill":true,"opaque_walls_unlit":true,"dynamic_leds_animate":true,"reduced_motion_identical_pixels":true,"off_mask_black":true}),
        )
    }

    /// Simulation/save timings without rendering, using only diagnostic recovery files.
    pub fn capture_action_performance(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(INITIAL_SEED, rules, texts, loot, expeditions)?;
        app.suspension_path = output.join("diagnostic-run.json");
        let mut rows = Vec::new();
        app.measure_action_state("recycling", &mut rows)?;
        app.walk_fixture_out_of_recycling()?;
        let passage = app
            .hub_regional_passage(RegionDirection::West)
            .ok_or("Missing western passage")?;
        app.walk_fixture_to(passage.step(Direction::East))?;
        app.measure_action_state("hub", &mut rows)?;
        let start = Instant::now();
        if app.execute_command(GameCommand::Interact { target: passage }) != CommandOutcome::Applied
        {
            return Err("Surface travel rejected".into());
        }
        app.capture_events_at(Some(0.0));
        rows.push(serde_json::json!({"scene":"surface", "operation":"generate-and-travel", "ms":start.elapsed().as_secs_f64()*1000.0}));
        app.measure_action_state("surface", &mut rows)?;
        let world = app
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .ok_or("Missing regional world")?;
        let descent = project_rl::world::generation::vertical_passage(
            world.map_size_at(RegionCoord::new(-1, 0, 0)),
            project_rl::content::RegionVerticalDirection::Down,
        );
        app.walk_fixture_to(descent)?;
        let start = Instant::now();
        if app.execute_command(GameCommand::Interact { target: descent }) != CommandOutcome::Applied
        {
            return Err("Depth travel rejected".into());
        }
        app.capture_events_at(Some(0.0));
        rows.push(serde_json::json!({"scene":"depth-one", "operation":"generate-and-travel", "ms":start.elapsed().as_secs_f64()*1000.0}));
        app.measure_action_state("depth-one", &mut rows)?;
        let start = Instant::now();
        app.suspend_run()?;
        rows.push(serde_json::json!({"scene":"depth-one", "operation":"verified-suspension", "ms":start.elapsed().as_secs_f64()*1000.0}));
        std::fs::write(
            output.join("actions.json"),
            serde_json::to_vec_pretty(&rows).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())
    }

    fn measure_action_state(
        &mut self,
        scene: &str,
        rows: &mut Vec<serde_json::Value>,
    ) -> Result<(), String> {
        for operation in [
            "wait",
            "engine-snapshot",
            "app-snapshot",
            "state-fingerprint",
            "content-fingerprints",
            "suspension",
            "checkpoint",
        ] {
            let mut times = Vec::new();
            for _ in 0..8 {
                let start = Instant::now();
                match operation {
                    "wait" => {
                        if matches!(
                            self.execute_command(GameCommand::Wait),
                            CommandOutcome::Rejected(_)
                        ) {
                            return Err(format!("{scene}: wait rejected"));
                        }
                        self.capture_events_at(Some(0.0));
                    }
                    "engine-snapshot" => {
                        std::hint::black_box(self.game.recovery_snapshot_bytes()?);
                    }
                    "app-snapshot" => {
                        std::hint::black_box(self.encode_recovery_snapshot()?);
                    }
                    "state-fingerprint" => {
                        std::hint::black_box(suspension::fingerprint(&self.game));
                    }
                    "content-fingerprints" => {
                        std::hint::black_box(rules_fingerprint_for_version(
                            &self.rules,
                            self.generation_version,
                        ));
                        std::hint::black_box(suspension::fingerprint(&self.loot));
                        std::hint::black_box(world_fingerprint_for_version(
                            &self.expeditions,
                            &self.regional_worlds,
                            self.generation_version,
                        ));
                    }
                    "suspension" => {
                        std::hint::black_box(self.suspension()?);
                    }
                    "checkpoint" => {
                        self.write_crash_recovery_checkpoint()?;
                    }
                    _ => unreachable!(),
                }
                times.push(start.elapsed().as_secs_f64() * 1000.0);
            }
            times.sort_by(f64::total_cmp);
            let row = serde_json::json!({"scene":scene,"operation":operation,"median_ms":times[times.len()/2],"max_ms":times.last()});
            eprintln!("[PERF ACTION] {row}");
            rows.push(row);
        }
        let snapshot = self.game.recovery_snapshot_bytes()?;
        rows.push(serde_json::json!({"scene":scene,"operation":"state", "commands":self.history.len(), "fingerprint":suspension::fingerprint(&self.game), "snapshot_bytes":snapshot.len(), "snapshot_fingerprint":suspension::fingerprint(&snapshot)}));
        Ok(())
    }

    pub async fn capture_performance(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(INITIAL_SEED, rules, texts, loot, expeditions)?;
        app.suspension_path = output.join("diagnostic-run.json");
        app.graphics.active.mode = WindowMode::Windowed;
        request_new_screen_size(1280.0, 800.0);
        for _ in 0..5 {
            next_frame().await;
        }
        let mut rows = Vec::new();
        for scene in [
            "world",
            "pause",
            "world-return",
            "inventory",
            "skills",
            "help",
            "main",
            "ground-fire",
            "ground-fire-pause",
            "ground-fire-return",
        ] {
            app.menu = MenuScreen::Hidden;
            app.inventory_open = false;
            app.skills_open = false;
            app.legend_open = false;
            match scene {
                "pause" | "ground-fire-pause" => app.open_menu(MenuScreen::Pause),
                "inventory" => app.inventory_open = true,
                "skills" => app.skills_open = true,
                "help" => app.legend_open = true,
                "main" => app.open_menu(MenuScreen::Main),
                "ground-fire" => app.prepare_lab_effect_visual("ground-fire")?,
                _ => {}
            }
            let before = suspension::fingerprint(&app.game);
            let input = InputFrame {
                viewport: Some((1280.0, 800.0)),
                ..Default::default()
            };
            let mut cpu = Vec::new();
            let mut frames = Vec::new();
            for _ in 0..120 {
                let start = Instant::now();
                app.update_input_at(&input, Some(get_time()));
                app.draw();
                cpu.push(start.elapsed().as_secs_f64() * 1000.0);
                next_frame().await;
                frames.push(start.elapsed().as_secs_f64() * 1000.0);
            }
            if before != suspension::fingerprint(&app.game) {
                return Err(format!("{scene}: rendering changed simulation"));
            }
            let first = cpu[0];
            // Keep first-frame cost separate from steady-state measurements.
            let stats = |values: &[f64]| {
                let mut sorted = values[10..].to_vec();
                sorted.sort_by(f64::total_cmp);
                serde_json::json!({"median_ms": sorted[sorted.len()/2], "p95_ms": sorted[sorted.len()*95/100], "max_ms": sorted.last()})
            };
            let row = serde_json::json!({"scene":scene,"first_cpu_ms":first,"cpu":stats(&cpu),"frame_with_present":stats(&frames)});
            eprintln!("[PERF] {row}");
            rows.push(row);
            // Screenshots use the separate cold-UI diagnostics. A GPU readback
            // here can stall the first frame of the NEXT scene as well.
        }
        let report = serde_json::json!({"build":"debug", "size":[screen_width(),screen_height()], "frames_per_scene":120,"scenes":rows});
        std::fs::write(
            output.join("timings.json"),
            serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())
    }
}
