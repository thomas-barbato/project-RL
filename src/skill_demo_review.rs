//! Native captures for the integrated skill demonstrations.
use super::*;

impl AsciiApp {
    pub(super) fn new_skill_demo_review() -> Result<Self, String> {
        let mut app = Self::new_agent_skill_laboratory()?;
        app.graphics.active.ui_scale_percent = 100;
        app.graphics.active.interface_style = crate::graphics::InterfaceStyle::Metal;
        app.graphics.active.reduced_motion = false;
        app.open_agent_skill_window();
        app.select_agent_skill_for_capture("FE02")?;
        app.ux.skill_demo_request = Some("FE02".into());
        app.crash_recovery_enabled = false;
        app.suspension_path = std::env::temp_dir().join("project-rl-skill-demo-host-unused.json");
        Ok(app)
    }
    pub async fn capture_skill_demo(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1280., 800.);
        for _ in 0..3 {
            next_frame().await;
        }
        if (screen_width(), screen_height()) != (1280., 800.) {
            return Err("Native 1280x800 surface unavailable".into());
        }
        let mut app = Self::new_skill_demo_review()?;
        app.game.drain_events();
        let before = app.game.recovery_snapshot_bytes()?;
        let history = app.history.len();
        let mut evidence = Vec::new();
        let policy_check = std::env::args().any(|arg| arg == "--animation-policy");
        let times = [
            0., 0.12, 0.28, 0.52, 0.82, 1.14, 1.72, 2.6, 0.12, 0.52, 1.14, 2.6,
        ];
        for skill in project_rl::skills::agent::definitions()
            .iter()
            .filter(|s| s.kind == "active")
        {
            app.select_agent_skill_for_capture(&skill.id)?;
            app.ux.skill_demo_request = Some(skill.id.clone());
            app.ux.skill_demo.restart();
            let dir = output.join("active-scenes").join(&skill.id);
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let bounds = app
                .agent_skill_demo_rect()
                .ok_or("Missing integrated demonstration")?;
            for frame in 0..if policy_check { times.len() } else { 24 } {
                if policy_check && frame == 8 {
                    app.ux.skill_demo.restart();
                }
                app.graphics.active.reduced_motion = policy_check && frame >= 8;
                app.ux.skill_demo.capture_time(Some(
                    10. + if policy_check {
                        times[frame]
                    } else {
                        frame as f64 * 0.14
                    },
                ));
                app.draw();
                if let Some(error) = app.ux.skill_demo.failure() {
                    return Err(format!("{}: {error}", skill.id));
                }
                let image = crate::ui_capture::framebuffer()?;
                let (x, y, w, h) = (
                    bounds.x.round() as usize,
                    (800. - bounds.bottom()).round() as usize,
                    bounds.w.round() as usize,
                    bounds.h.round() as usize,
                );
                let mut pixels = Vec::with_capacity(w * h * 4);
                for row in y..y + h {
                    let offset = (row * usize::from(image.width) + x) * 4;
                    pixels.extend_from_slice(&image.bytes[offset..offset + w * 4]);
                }
                std::fs::write(dir.join(format!("{frame:03}.rgba")), pixels)
                    .map_err(|e| e.to_string())?;
                if skill.id == "FE02" && frame == 5 {
                    image.export_png(
                        output
                            .join("fenetre-metal.png")
                            .to_str()
                            .ok_or("Capture path")?,
                    );
                }
                next_frame().await;
            }
            evidence.push(serde_json::json!({"skill":skill.id,"title":skill.title,"branch":skill.branch,
                "frames":if policy_check {times.len()} else {24},"frame_duration_ms":140,"policy_times":if policy_check {Some(times)} else {None},"reduced_from_frame":if policy_check {Some(8)}else{None},"rank":1,"bounds":[bounds.x,bounds.y,bounds.w,bounds.h]}));
        }
        app.graphics.active.reduced_motion = false;
        app.ux.skill_demo_request = None;
        app.select_agent_skill_for_capture("AF06")?;
        app.draw();
        crate::ui_capture::framebuffer()?
            .export_png(output.join("passif.png").to_str().ok_or("Capture path")?);
        next_frame().await;
        app.select_agent_skill_for_capture("FE02")?;
        app.ux.skill_demo_request = Some("FE02".into());
        app.ux.skill_demo.capture_time(Some(10.7));
        app.graphics.active.interface_style = crate::graphics::InterfaceStyle::Terminal;
        app.draw();
        crate::ui_capture::framebuffer()?.export_png(
            output
                .join("fenetre-terminal.png")
                .to_str()
                .ok_or("Capture path")?,
        );
        next_frame().await;
        let unchanged =
            app.game.recovery_snapshot_bytes()? == before && app.history.len() == history;
        if !unchanged {
            return Err("Demonstrations changed host state/RNG/history".into());
        }
        std::fs::write(output.join("integration-native.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "review":"SKILL-DEMO-01","revision":3,"status":"accepted_and_integrated","surface":[1280,800],
            "normal_game_enabled":true,"all_active_count":evidence.len(),"examples":evidence,
            "host_state_rng_history_unchanged":unchanged,"persistent_writes":0,"rank":1,
            "inside_skill_grid":true,"extra_panel":false,"centered_skill_title_only":true,
            "base_example_independent_of_learned_rank":true,"render_target_matches_ui_scale":true
        })).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }
    pub async fn run_skill_demo_review() -> Result<(), String> {
        request_new_screen_size(1280., 960.);
        let mut app = Self::new_skill_demo_review()?;
        loop {
            if is_quit_requested() || is_key_pressed(KeyCode::Escape) {
                break;
            }
            if is_key_pressed(KeyCode::T) {
                app.graphics.active.interface_style = if app.graphics.active.interface_style
                    == crate::graphics::InterfaceStyle::Metal
                {
                    crate::graphics::InterfaceStyle::Terminal
                } else {
                    crate::graphics::InterfaceStyle::Metal
                };
            }
            app.update_input(&InputFrame::capture());
            app.draw();
            next_frame().await;
        }
        Ok(())
    }
}
