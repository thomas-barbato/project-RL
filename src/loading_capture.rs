//! Loading-only diagnostics: current shipped scenes, isolated state, no screenshots in timed frames.
use super::*;
use project_rl::loading_profile;
use std::time::Instant;

impl AsciiApp {
    pub async fn capture_loading_performance(
        output: &std::path::Path,
        startup_ms: f64,
    ) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1280., 800.);
        for _ in 0..4 {
            next_frame().await;
        }
        let fixture = std::env::args()
            .find_map(|arg| arg.strip_prefix("--loading-fixture=").map(str::to_owned))
            .unwrap_or_else(|| "route".into());
        let mut scenes = Vec::new();
        let mut setup = Vec::new();
        if fixture == "route" {
            loading_profile::phase("camp_setup");
            let started = Instant::now();
            let (mut rules, texts, loot, expeditions) = ascii_game_content()?;
            let classes = ascii_character_class_catalog()?;
            let id: CharacterClassId = "core:bastion".parse().unwrap();
            let class = classes.get(&id).ok_or("Bastion profile absent")?;
            class
                .apply_to_rules(&mut rules, class.recommended_attributes())
                .map_err(|error| error.to_string())?;
            weapon_supplies_app::ensure_class_weapon_supplies(
                &mut rules,
                CURRENT_GENERATION_VERSION,
            );
            let mut app = Self::from_seed(1872, rules, texts, loot, expeditions)?;
            app.character_class = Some(id);
            app.select_weapon_slot(app.preferred_starting_weapon_slot());
            app.crash_recovery_enabled = false;
            app.suspension_path = output.join("unused-run.json");
            app.graphics.active.world_cell_px = 64;
            app.menu = MenuScreen::Hidden;
            app.character_creation = None;
            app.game
                .player_position()
                .ok_or("Camp fixture has no player")?;
            setup.push(serde_json::json!({"phase":"camp_setup","cpu_ms":started.elapsed().as_secs_f64()*1000.}));
            app.measure_loading_scene("camp", 90, false, &mut scenes)
                .await?;

            loading_profile::phase("walk_to_camp_exit");
            let exit = crate::cryogenic_camp::pos(crate::cryogenic_camp::scene().exits[0]);
            app.walk_fixture_to_unchecked(exit)?;
            app.measure_loading_transition("camp_to_street", exit, &mut setup)?;
            app.measure_loading_scene("street", 90, false, &mut scenes)
                .await?;
        } else if fixture != "sewer" && fixture != "effects" {
            return Err(format!("Unknown loading fixture: {fixture}"));
        }
        if fixture == "route" || fixture == "sewer" {
            // This fixture starts by the real hatch, without a combat march across town.
            loading_profile::phase("near_hatch_fixture_setup");
            let started = Instant::now();
            let mut app = Self::connected_sewer_fixture(1)?;
            app.suspension_path = output.join("unused-sewer-run.json");
            setup.push(serde_json::json!({"phase":"near_hatch_fixture_setup","cpu_ms":started.elapsed().as_secs_f64()*1000.}));
            if fixture == "sewer" {
                app.measure_loading_scene("street", 90, false, &mut scenes)
                    .await?;
            }
            app.measure_loading_transition("street_to_sewer", crate::sewer::ENTRY, &mut setup)?;
            app.measure_loading_scene("sewer", 90, false, &mut scenes)
                .await?;
            let exit = crate::prison_street::pos(crate::sewer::connected_plan(1).spawn);
            app.measure_loading_transition("sewer_to_street", exit, &mut setup)?;
            app.measure_loading_scene("street_return", 90, false, &mut scenes)
                .await?;
            app.measure_loading_transition(
                "street_to_sewer_return",
                crate::sewer::ENTRY,
                &mut setup,
            )?;
            app.measure_loading_scene("sewer_return", 90, false, &mut scenes)
                .await?;
        }
        if fixture == "effects" {
            loading_profile::phase("effect_host_setup");
            let mut app = Self::new_skill_demo_review_with_bonus_seed(Some(1872))?;
            app.suspension_path = output.join("unused-effect-run.json");
            app.ux.skill_demo_request = None;
            app.measure_loading_scene("agent_window", 90, false, &mut scenes)
                .await?;
            for skill in ["FE02", "FR02", "D58"] {
                app.select_agent_skill_for_capture(skill)?;
                app.ux.skill_demo_request = Some(skill.into());
                app.measure_loading_scene(&format!("effect_{skill}"), 420, true, &mut scenes)
                    .await?;
            }
        }
        let samples = loading_profile::finish();
        let report = serde_json::json!({
            "schema":1, "fixture":fixture, "laboratory_bonus_seed":1872, "debug_assertions":cfg!(debug_assertions),
            "native_cell_px":64, "window":[screen_width(),screen_height()],
            "data_root":project_rl::data_paths::root(),
            "packed":project_rl::data_paths::source()?.is_packed(),
            "startup_before_first_yield_ms":startup_ms, "setup_and_transitions":setup,
            "rendering_preserves_engine_and_terminal_bytes":true, "scenes":scenes, "samples":samples,
            "notes":[
                "Fresh process; operating-system file caches are not flushed.",
                "read_resource includes archive decompression when packed; it is not pure disk I/O.",
                "texture_create_cpu measures CPU/driver submission, not GPU execution time.",
                "frame_with_present includes swap/vsync/wait and diagnostic scheduling.",
                "The near-hatch fixture is separate from the camp-to-street session.",
                "Embedded image banks are attributed to their source call site. Instrumentation is not exhaustive.",
                "Nested preparation spans are inclusive and must not be added to their child stages."
            ]
        });
        std::fs::write(
            output.join("loading.json"),
            serde_json::to_vec_pretty(&report).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        eprintln!(
            "[LOADING] {fixture}: {} scenes; {} loading samples",
            scenes.len(),
            report["samples"].as_array().unwrap().len()
        );
        Ok(())
    }

    fn measure_loading_transition(
        &mut self,
        label: &str,
        at: GridPos,
        rows: &mut Vec<serde_json::Value>,
    ) -> Result<(), String> {
        loading_profile::phase(label);
        let started = Instant::now();
        let outcome = self.execute_command(GameCommand::Interact { target: at });
        let cpu_ms = started.elapsed().as_secs_f64() * 1000.;
        if outcome != CommandOutcome::Applied {
            return Err(format!(
                "{label}: transition rejected: {:?}",
                self.log.last()
            ));
        }
        self.capture_events_at(Some(0.));
        self.combat_motion = crate::combat_visuals::CombatMotion::default();
        self.world_cursor = None;
        self.selected_target = None;
        self.selected_world_cell = None;
        rows.push(serde_json::json!({"phase":label,"cpu_ms":cpu_ms,"zone":self.game.current_zone().map(|zone|zone.id.as_str())}));
        Ok(())
    }

    async fn measure_loading_scene(
        &mut self,
        label: &str,
        frames: usize,
        demo: bool,
        rows: &mut Vec<serde_json::Value>,
    ) -> Result<(), String> {
        let before = self.game.recovery_snapshot_bytes()?;
        let terminal_before = bincode::serialize(&self.terminal).map_err(|e| e.to_string())?;
        let history_before = self.history.len();
        let mut cpu = Vec::with_capacity(frames);
        let mut present = Vec::with_capacity(frames);
        for frame in 0..frames {
            loading_profile::phase(&format!("{label}/frame-{frame}"));
            if demo {
                self.ux
                    .skill_demo
                    .capture_time(Some(10. + frame as f64 * 0.01));
            }
            let started = Instant::now();
            self.draw();
            cpu.push(started.elapsed().as_secs_f64() * 1000.);
            next_frame().await;
            present.push(started.elapsed().as_secs_f64() * 1000.);
            if demo && let Some(error) = self.ux.skill_demo.failure() {
                return Err(error);
            }
        }
        if self.game.recovery_snapshot_bytes()? != before
            || bincode::serialize(&self.terminal).map_err(|e| e.to_string())? != terminal_before
            || self.history.len() != history_before
        {
            return Err(format!(
                "{label}: rendering changed engine, terminal or history"
            ));
        }
        let summary = |values: &[f64]| {
            let mut sorted = values.to_vec();
            sorted.sort_by(f64::total_cmp);
            serde_json::json!({"median_ms":sorted[sorted.len()/2],"p95_ms":sorted[sorted.len()*95/100],
                "max_ms":sorted.last(),"over_16_67_ms":values.iter().filter(|&&v|v>1000./60.).count(),
                "over_50_ms":values.iter().filter(|&&v|v>50.).count()})
        };
        let row = serde_json::json!({"scene":label,"frames":frames,"first_cpu_ms":cpu[0],
            "first_frame_with_present_ms":present[0], "all_cpu":summary(&cpu),
            "steady_cpu_excluding_first_10":summary(&cpu[10..]), "frame_with_present":summary(&present),
            "cpu_by_frame_ms":cpu,"frame_with_present_by_frame_ms":present,
            "engine_fingerprint":suspension::fingerprint(&self.game),
            "terminal_fingerprint":suspension::fingerprint(&self.terminal),"read_only":true});
        eprintln!(
            "[LOADING] {label}: first CPU {:.2} ms, steady median {} ms",
            cpu[0], row["steady_cpu_excluding_first_10"]["median_ms"]
        );
        rows.push(row);
        Ok(())
    }
}
