//! Production-renderer coverage for the approved finish, with isolated worlds.
use super::*;

impl AsciiApp {
    fn polish_world(
        &mut self,
        map: project_rl::world::Map,
        at: GridPos,
        info: Option<project_rl::game::ZoneInfo>,
        decor: SectorDecor,
    ) -> Result<(), String> {
        let rules = self.game.rules().clone();
        let game = GameState::new_with_rules(map, at, 2701, rules).map_err(|e| e.to_string())?;
        self.game = WorldState::single(game);
        if let Some(info) = info {
            self.game.enable(info)?;
        }
        for (number, next) in [
            (120, GridPos::new(at.x + 1, at.y)),
            (14, GridPos::new(at.x + 2, at.y + 1)),
        ] {
            if self.game.map().is_walkable(next)
                && self.game.player_visibility().is_visible(next)
                && self.game.actors().entity_at(next).is_none()
            {
                let actor = Actor::new(next, 100)
                    .map_err(|e| e.to_string())?
                    .with_player_relation(PlayerRelation::Hostile)
                    .with_tags([format!("core:monster_art_{number}_0").parse().unwrap()]);
                let id = self.game.spawn_actor(actor).map_err(|e| e.to_string())?;
                self.actor_glyphs.insert(id, 'd');
            }
        }
        self.game.drain_events();
        self.terminal = TerminalView::new(decor, self.game.map(), self.game.player_visibility());
        self.combat_motion = crate::combat_visuals::CombatMotion::default();
        self.world_presentation_time.set(1.);
        Ok(())
    }

    async fn polish_pair(
        &mut self,
        output: &std::path::Path,
        label: &str,
        records: &mut Vec<serde_json::Value>,
    ) -> Result<(), String> {
        self.world_presentation_time.set(1.);
        self.combat_motion = crate::combat_visuals::CombatMotion::default();
        if label.starts_with("contact") {
            let from = self
                .game
                .player_position()
                .ok_or("Contact fixture player missing")?;
            self.combat_motion.play_rupture(
                from,
                GridPos::new(from.x + 1, from.y),
                1. - 0.22 * crate::animation_policy::duration_scale("B01") - 0.025,
            );
        }
        let before = self.game.recovery_snapshot_bytes()?;
        let viewport = crate::terminal_view::terminal_playfield(
            &self.game,
            self.terminal_bounds(),
            self.ui_scale(),
            self.navigation_signal_summary().is_some(),
        );
        let mut images = Vec::new();
        let mut render_ms = Vec::new();
        for enabled in [false, true] {
            let _finish = crate::visual_polish::review(enabled);
            let _clock = crate::effect_geometry::frame_time(1.);
            crate::camp_npc_review::set_capture_time(1.);
            for _ in 0..3 {
                self.draw();
                next_frame().await;
            }
            let start = std::time::Instant::now();
            self.draw();
            unsafe {
                macroquad::window::get_internal_gl().flush();
            }
            render_ms.push(start.elapsed().as_secs_f64() * 1000.);
            let image = crate::ui_capture::framebuffer()?;
            image.export_png(
                output
                    .join(format!(
                        "{label}-{}.png",
                        if enabled { "integrated" } else { "before" }
                    ))
                    .to_str()
                    .ok_or("capture path")?,
            );
            images.push(image);
            next_frame().await;
        }
        if before != self.game.recovery_snapshot_bytes()? {
            return Err(format!("{label}: rendering changed simulation"));
        }
        let (a, b) = (&images[0], &images[1]);
        let pointer = mouse_position();
        let player = self
            .game
            .player_position()
            .ok_or("Capture player missing")?;
        let player_rect = self
            .terminal
            .world_cell_rect(
                &self.game,
                self.terminal_bounds(),
                self.ui_scale(),
                64,
                self.navigation_signal_summary().is_some(),
                player,
                Vec2::ZERO,
            )
            .ok_or("Capture player outside camera")?;
        let mut ui_changed = 0;
        let mut hidden_changed = 0;
        let mut world_changed = 0;
        let mut black_checked = 0;
        let mut black_edge_rounding = 0;
        for y in 0..a.height as usize {
            for x in 0..a.width as usize {
                let i = ((a.height as usize - 1 - y) * a.width as usize + x) * 4;
                let old = &a.bytes[i..i + 3];
                let new = &b.bytes[i..i + 3];
                if viewport.contains(vec2(x as f32, y as f32)) {
                    if old != new {
                        world_changed += 1;
                    }
                    let cell = GridPos::new(
                        player.x + ((x as f32 - player_rect.x) / 64.).floor() as i32,
                        player.y + ((y as f32 - player_rect.y) / 64.).floor() as i32,
                    );
                    if old == [0, 0, 0] && !self.game.player_visibility().is_visible(cell) {
                        black_checked += 1;
                        if new.iter().any(|v| *v > 1) {
                            hidden_changed += 1;
                        } else if new != [0, 0, 0] {
                            black_edge_rounding += 1;
                        }
                    }
                } else if ((x as f32 - pointer.0).abs() > 48. || (y as f32 - pointer.1).abs() > 48.)
                    && old != new
                {
                    ui_changed += 1;
                }
            }
        }
        if hidden_changed != 0 || ui_changed != 0 || world_changed == 0 {
            return Err(format!(
                "{label}: UI={ui_changed}, black={hidden_changed}, changed={world_changed}"
            ));
        }
        records.push(serde_json::json!({"scene":label,"native_cell":64,"simulation_unchanged":true,"ui_pixels_changed_outside_pointer":ui_changed,"black_pixels_checked":black_checked,"black_pixels_changed_above_one_lsb":hidden_changed,"black_feather_rounding_one_lsb":black_edge_rounding,"world_pixels_changed":world_changed,"draw_cpu_ms_before_after":render_ms}));
        println!("POLISH01 integration {label}: state, UI and black masks unchanged");
        Ok(())
    }

    pub async fn capture_polish_integration(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1280., 800.);
        for _ in 0..4 {
            next_frame().await;
        }
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(2701, rules, texts, loot, expeditions)?;
        app.suspension_path = output.join("isolated-run.json");
        app.crash_recovery_enabled = false;
        app.begin_character_creation_with_seed(false, 2701)?;
        app.update_input(&InputFrame {
            pressed: [crate::controls::Binding::key("Enter")].into(),
            ..Default::default()
        });
        if app.character_creation.is_some() {
            return Err("Incomplete isolated character creation".into());
        }
        app.menu = MenuScreen::Hidden;
        app.capture_events_at(Some(0.));
        app.graphics.active.ui_scale_percent = 100;
        app.graphics.active.world_cell_px = 64;
        app.graphics.active.world_grayscale = false;
        app.graphics.active.reduced_motion = false;
        let template = app
            .game
            .current_zone()
            .ok_or("Isolated camp missing")?
            .clone();
        let mut records = Vec::new();
        app.walk_fixture_to_unchecked(crate::cryogenic_camp::pos([22, 8]))?;
        app.capture_events_at(Some(0.));
        app.polish_pair(output, "camp", &mut records).await?;
        let street = crate::prison_street::info(template.clone());
        for (label, at) in [
            ("street", GridPos::new(13, 54)),
            ("interior", GridPos::new(6, 52)),
        ] {
            let zone = crate::prison_street::blueprint(
                street.clone(),
                2701,
                true,
                true,
                true,
                true,
                true,
            )?;
            app.polish_world(zone.map, at, Some(street.clone()), SectorDecor::default())?;
            app.polish_pair(output, label, &mut records).await?;
        }
        for index in 1..=3 {
            let mut info = template.clone();
            info.id = format!("core:prison_sewer_0{index}").parse().unwrap();
            info.name = "Égouts".into();
            info.kind = "core:dungeon".parse().unwrap();
            let zone = crate::sewer::blueprint(info.clone(), 2701, &app.regional_worlds)?;
            let at = crate::sewer::plan(index)
                .views
                .first()
                .map_or(zone.entrance, |v| crate::prison_street::pos(v.at));
            app.polish_world(zone.map, at, Some(info), SectorDecor::default())?;
            app.polish_pair(output, &format!("sewer-{index}"), &mut records)
                .await?;
        }
        for seed in [2701, 2717] {
            let sector = crate::test_sector::TestSector::build_for_generation(
                seed, true, true, true, true, true,
            )?;
            let at = sector.level.player_start();
            app.polish_world(sector.level.map().clone(), at, None, sector.decor)?;
            app.polish_pair(output, &format!("generated-surface-{seed}"), &mut records)
                .await?;
        }
        for (name, decor) in [
            ("underground", crate::test_sector::Decor::Grate),
            ("organic", crate::test_sector::Decor::ChitinFloor),
            ("occult", crate::test_sector::Decor::ForeignFloor),
        ] {
            let mut map = project_rl::world::Map::filled(24, 20, Terrain::Floor)
                .map_err(|e| e.to_string())?;
            let mut scenery = SectorDecor::default();
            for y in 0..20 {
                for x in 0..24 {
                    let at = GridPos::new(x, y);
                    if x == 0 || y == 0 || x == 23 || y == 19 || x == 18 {
                        map.set_terrain(at, Terrain::Wall)
                            .map_err(|e| e.to_string())?;
                    } else {
                        scenery.cells.insert(at, decor);
                    }
                }
            }
            app.polish_world(map, GridPos::new(12, 10), None, scenery)?;
            app.polish_pair(output, name, &mut records).await?;
        }
        // The same generated world exercises optional grayscale and reduced motion.
        app.graphics.active.world_grayscale = true;
        app.polish_pair(output, "grayscale", &mut records).await?;
        app.graphics.active.world_grayscale = false;
        app.polish_pair(output, "contact", &mut records).await?;
        app.graphics.active.reduced_motion = true;
        app.polish_pair(output, "contact-reduced-motion", &mut records)
            .await?;
        for id in ["core:traqueur", "core:arcaniste", "core:metamorphe"] {
            app.begin_character_creation_with_seed(false, 2701)?;
            let (index, attributes) = app
                .character_classes
                .iter()
                .enumerate()
                .find(|(_, (key, _))| key.as_str() == id)
                .map(|(index, (_, class))| (index, class.recommended_attributes()))
                .ok_or("Playable class missing")?;
            let creation = app.character_creation.as_mut().ok_or("Creation missing")?;
            creation.selected_class = index;
            creation.attributes = attributes;
            let creation = app.character_creation.take().unwrap();
            app.rebuild_run_with_character_class(&creation)?;
            app.menu = MenuScreen::Hidden;
            app.capture_events_at(Some(0.));
            app.graphics.active.reduced_motion = false;
            app.polish_pair(output, id.trim_start_matches("core:"), &mut records)
                .await?;
        }
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":"POLISH01-INTEGRATION01","scenes":records,"production_renderer":true,"normal_game_activation":true,"settings_written":false,"saved_campaign_access":false,"cursor_exclusion_px":48})).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
        Ok(())
    }
}
