//! Repeatable native rendering measurements, isolated from user saves/settings.
use super::*;
use std::time::Instant;

impl AsciiApp {
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
