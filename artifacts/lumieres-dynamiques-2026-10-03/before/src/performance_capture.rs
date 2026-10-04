//! Repeatable native rendering and action measurements, isolated from user saves/settings.
use super::*;
use std::time::Instant;

impl AsciiApp {
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
