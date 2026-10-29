//! SOL-CONT01: native before/after through the production camera, lights and UI.
use super::*;
impl AsciiApp {
    async fn continuous_floor_pair(
        &mut self,
        output: &std::path::Path,
        label: &str,
        records: &mut Vec<serde_json::Value>,
        interior: bool,
        integrated: bool,
    ) -> Result<(), String> {
        self.combat_motion = crate::combat_visuals::CombatMotion::default();
        self.log.clear();
        self.world_cursor = None;
        self.selected_world_cell = None;
        self.selected_target = None;
        self.world_presentation_time.set(1.);
        let before = self.game.recovery_snapshot_bytes()?;
        let mut frames = Vec::new();
        let future = std::env::args().nth(1).as_deref() == Some("--ui-cold-future-floor");
        let grate = std::env::args().nth(1).as_deref() == Some("--ui-cold-corridor-grate");
        let wear = matches!(
            std::env::args().nth(1).as_deref(),
            Some("--ui-cold-corridor-wear" | "--ui-cold-corridor-wear-integration")
        );
        for proposed in [false, true] {
            if wear {
                if proposed {
                    crate::floor_surface_review::set_corridor_wear(true);
                } else {
                    crate::floor_surface_review::set_corridor_grate(!integrated);
                }
            } else if grate {
                crate::floor_surface_review::set_corridor_grate(proposed);
            } else if interior && future {
                if proposed {
                    crate::floor_surface_review::set_future(true);
                } else {
                    crate::floor_surface_review::set_interior(true);
                }
            } else if interior && proposed {
                crate::floor_surface_review::set_interior(true);
            } else {
                crate::floor_surface_review::set_enabled(!integrated && (interior || proposed));
            }
            let _clock = crate::effect_geometry::frame_time(1.);
            crate::camp_npc_review::set_capture_time(1.);
            for _ in 0..2 {
                self.draw();
                next_frame().await;
            }
            self.draw();
            let image = crate::ui_capture::framebuffer()?;
            image.export_png(
                output
                    .join(format!(
                        "{label}-{}.png",
                        if proposed { "after" } else { "before" }
                    ))
                    .to_str()
                    .ok_or("Capture path")?,
            );
            frames.push(image);
            next_frame().await;
        }
        crate::floor_surface_review::set_enabled(false);
        if before != self.game.recovery_snapshot_bytes()? {
            return Err("Floor review changed simulation".into());
        }
        let at = self.game.player_position().ok_or("Player missing")?;
        let cell = self
            .terminal
            .world_cell_rect(
                &self.game,
                self.terminal_bounds(),
                self.ui_scale(),
                64,
                self.navigation_signal_summary().is_some(),
                at,
                Vec2::ZERO,
            )
            .ok_or("Player outside camera")?;
        let viewport = crate::terminal_view::terminal_playfield(
            &self.game,
            self.terminal_bounds(),
            self.ui_scale(),
            self.navigation_signal_summary().is_some(),
        );
        let pointer = mouse_position();
        let mut ui_changes = 0;
        let mut world_changes = 0;
        for y in 0..frames[0].height as usize {
            for x in 0..frames[0].width as usize {
                let i = ((frames[0].height as usize - 1 - y) * frames[0].width as usize + x) * 4;
                if frames[0].bytes[i..i + 3] == frames[1].bytes[i..i + 3] {
                    continue;
                }
                if viewport.contains(vec2(x as f32, y as f32)) {
                    world_changes += 1;
                } else if (x as f32 - pointer.0).abs() > 48. || (y as f32 - pointer.1).abs() > 48. {
                    ui_changes += 1;
                }
            }
        }
        let needs_change = !integrated
            && (!(grate || wear)
                || matches!(label, "camp-corridor" | "camp-doorway" | "camp-threshold"));
        if ui_changes != 0
            || (world_changes == 0 && needs_change)
            || (world_changes != 0 && integrated)
            || cell.w != 64.
        {
            return Err(format!(
                "Floor capture {label}: UI={ui_changes}, world={world_changes}, cell={}",
                cell.w
            ));
        }
        records.push(serde_json::json!({"scene":label,"native_cell":64,"crop":[cell.x-4.*64.,cell.y-3.*64.,640.,448.],"camera_world_origin":[cell.x-at.x as f32*64.,cell.y-at.y as f32*64.],"world_pixels_changed":world_changes,"ui_pixels_changed":ui_changes,"simulation_unchanged":true}));
        Ok(())
    }
    pub async fn capture_continuous_floor(output: &std::path::Path) -> Result<(), String> {
        Self::capture_floor_scenes(output, false, false).await
    }
    pub async fn capture_camp_floor_03(output: &std::path::Path) -> Result<(), String> {
        Self::capture_floor_scenes(output, true, false).await
    }
    pub async fn capture_future_floor(output: &std::path::Path) -> Result<(), String> {
        Self::capture_floor_scenes(output, true, false).await
    }
    pub async fn capture_corridor_grate(output: &std::path::Path) -> Result<(), String> {
        Self::capture_floor_scenes(output, true, false).await
    }
    pub async fn capture_corridor_wear(output: &std::path::Path) -> Result<(), String> {
        Self::capture_floor_scenes(output, true, false).await
    }
    pub async fn capture_corridor_wear_integration(output: &std::path::Path) -> Result<(), String> {
        Self::capture_floor_scenes(output, true, true).await
    }
    pub async fn capture_exterior_floor_integration(
        output: &std::path::Path,
    ) -> Result<(), String> {
        Self::capture_floor_scenes(output, false, true).await
    }
    async fn capture_floor_scenes(
        output: &std::path::Path,
        interior: bool,
        integrated: bool,
    ) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1280., 800.);
        for _ in 0..4 {
            next_frame().await;
        }
        let (r, t, l, e) = ascii_game_content()?;
        let mut app = Self::from_seed(2701, r, t, l, e)?;
        app.suspension_path = output.join("isolated-run.json");
        app.crash_recovery_enabled = false;
        app.begin_character_creation_with_seed(false, 2701)?;
        app.update_input(&InputFrame {
            pressed: [crate::controls::Binding::key("Enter")].into(),
            ..Default::default()
        });
        app.graphics.active.world_cell_px = 64;
        app.graphics.active.world_grayscale = false;
        app.graphics.active.reduced_motion = false;
        let mut records = Vec::new();
        if !integrated || interior {
            app.continuous_floor_pair(output, "camp-arrival", &mut records, interior, integrated)
                .await?;
            app.walk_fixture_to_unchecked(GridPos::new(22, 8))?;
            app.continuous_floor_pair(output, "camp-corridor", &mut records, interior, integrated)
                .await?;
        }
        if interior {
            for (label, at) in [
                ("camp-storage", GridPos::new(17, 3)),
                ("camp-security", GridPos::new(30, 3)),
                ("camp-medical", GridPos::new(17, 13)),
                ("camp-workshop", GridPos::new(30, 13)),
                ("camp-doorway", GridPos::new(9, 8)),
                ("camp-threshold", GridPos::new(17, 10)),
            ] {
                app.walk_fixture_to_unchecked(at)?;
                app.continuous_floor_pair(output, label, &mut records, true, integrated)
                    .await?;
            }
        }
        if !interior {
            let info = crate::prison_street::info(app.game.current_zone().unwrap().clone());
            for (label, at) in [
                ("street-arrival", GridPos::new(7, 62)),
                ("street-plaza", GridPos::new(13, 54)),
                ("street-north", GridPos::new(10, 43)),
            ] {
                let mut blueprint = crate::prison_street::blueprint(
                    info.clone(),
                    2701,
                    true,
                    true,
                    true,
                    true,
                    true,
                )?;
                crate::sewer::prepare_street(&mut blueprint.map)?;
                let game =
                    GameState::new_with_rules(blueprint.map, at, 2701, app.game.rules().clone())
                        .map_err(|e| e.to_string())?;
                app.game = WorldState::single(game);
                app.game.enable(info.clone())?;
                let connection = crate::sewer::connected_connection(2701, 1);
                app.game.declare_deferred_connection_at(
                    info.id.clone(),
                    connection.at,
                    connection.destination,
                    connection.arrival,
                )?;
                app.terminal = TerminalView::new(
                    SectorDecor::default(),
                    app.game.map(),
                    app.game.player_visibility(),
                );
                app.refresh_zone_title();
                app.continuous_floor_pair(output, label, &mut records, false, true)
                    .await?;
            }
        }
        let review = if matches!(
            std::env::args().nth(1).as_deref(),
            Some("--ui-cold-corridor-wear" | "--ui-cold-corridor-wear-integration")
        ) {
            "SOL-FUT01-r03"
        } else if std::env::args().nth(1).as_deref() == Some("--ui-cold-corridor-grate") {
            "SOL-FUT01-r02"
        } else if std::env::args().nth(1).as_deref() == Some("--ui-cold-future-floor") {
            "SOL-FUT01-r01"
        } else if interior {
            "SOL-CONT01-r03"
        } else {
            "SOL-CONT01-r01"
        };
        let status = if integrated {
            "integrated"
        } else {
            "awaiting_visual_approval"
        };
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":review,"status":status,"normal_game_activation":integrated,"native_cell":64,"saved_campaign_access":false,"settings_written":false,"records":records})).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
        Ok(())
    }
}
