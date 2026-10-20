//! Isolated end-to-end captures through character creation and real HUD.
use super::*;

impl AsciiApp {
    pub async fn capture_prison_street(output: &std::path::Path) -> Result<(), String> {
        let lighting_review = std::env::args()
            .nth(1)
            .is_some_and(|s| s == "--ui-cold-prison-street-lighting");
        let _review = lighting_review.then(crate::prison_street_render::review);
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(1818, rules, texts, loot, expeditions)?;
        app.suspension_path = output.join("isolated-run.json");
        app.crash_recovery_enabled = false;
        app.begin_character_creation_with_seed(false, 1818)?;
        app.update_input(&InputFrame {
            pressed: [crate::controls::Binding::key("Enter")].into(),
            ..Default::default()
        });
        request_new_screen_size(1920., 1080.);
        for _ in 0..3 {
            next_frame().await;
        }
        let exit = pos(camp::scene().exits[0]);
        app.walk_fixture_to_unchecked(exit)?;
        if crate::prison_street::expansion_review() {
            app.materialize_deferred_destination_for(&GameCommand::Interact { target: exit })?;
        }
        if app.execute_command(GameCommand::Interact { target: exit }) != CommandOutcome::Applied {
            return Err(format!(
                "Camp to street passage failed: {:?}",
                app.log.last()
            ));
        }
        app.capture_events_at(Some(0.));
        if crate::prison_street::expansion_review()
            || std::env::args().nth(1).as_deref() == Some("--ui-cold-street-integration")
            || std::env::args().nth(1).as_deref() == Some("--ui-cold-street-defects")
        {
            return app.capture_street_expansion(output).await;
        }
        if matches!(
            std::env::args().nth(1).as_deref(),
            Some("--ui-cold-street-roofs" | "--ui-cold-street-roofs-integration")
        ) {
            return app.capture_street_roof_integration(output).await;
        }
        let s = crate::prison_street::scene_for_game(&app.game);
        let mut records = Vec::new();
        for (label, at) in [
            ("gate", s.entrance),
            ("road", [10, 8]),
            ("home-entry", s.home_entry),
            ("home", s.home_inside),
            ("north", [19, 3]),
        ] {
            app.walk_fixture_to_unchecked(crate::prison_street::pos(at))?;
            app.world_presentation_time.set(1.);
            let before = suspension::fingerprint(&app.game);
            for _ in 0..2 {
                app.draw();
                next_frame().await;
            }
            app.draw();
            let image = crate::ui_capture::framebuffer()?;
            image.export_png(output.join(format!("street-{label}.png")).to_str().unwrap());
            if lighting_review {
                crate::prison_street_render::review_lights(false);
                for _ in 0..2 {
                    app.draw();
                    next_frame().await;
                }
                app.draw();
                crate::ui_capture::framebuffer()?.export_png(
                    output
                        .join(format!("street-{label}-off.png"))
                        .to_str()
                        .unwrap(),
                );
                crate::prison_street_render::review_lights(true);
            }
            let r = app
                .terminal
                .world_cell_rect(
                    &app.game,
                    app.terminal_bounds(),
                    app.ui_scale(),
                    64,
                    app.navigation_signal_summary().is_some(),
                    crate::prison_street::pos(at),
                    Vec2::ZERO,
                )
                .ok_or("Player outside camera")?;
            if r.w != 64. || r.h != 64. || suspension::fingerprint(&app.game) != before {
                return Err("Street render changed scale or world".into());
            }
            records.push(serde_json::json!({"scene":label,"at":at,"cell":[r.x,r.y,r.w,r.h],"render_read_only":true,"turn":app.game.turn()}));
            next_frame().await;
        }
        app.walk_fixture_to_unchecked(crate::prison_street::pos(s.entrance))?;
        if app.execute_command(GameCommand::Interact {
            target: crate::prison_street::pos(s.entrance),
        }) != CommandOutcome::Applied
            || app.game.current_zone().unwrap().id.as_str() != camp::ZONE_ID
        {
            return Err("Street return failed".into());
        }
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":if lighting_review || s.atmosphere.is_some() {"SCENE-U01-07-r01 / LUM-U01-07-r01"} else {"SCENE-U01-06-r01"},"lighting_review":lighting_review,"lighting_production_enabled":s.atmosphere.is_some(),"generation":app.generation_version,"native_cell":64,"scenes":records,"camp_round_trip":true,"isolated_save":app.suspension_path})).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }

    async fn capture_unified_vision_after(
        &mut self,
        output: &std::path::Path,
        label: &str,
    ) -> Result<(), String> {
        crate::terminal_view::world_visibility_review::set_scenery_circle(
            std::env::args().any(|arg| arg == "--vision-circle"),
        );
        crate::terminal_view::world_visibility_review::set_enabled(true);
        for _ in 0..2 {
            self.draw();
            next_frame().await;
        }
        self.draw();
        let result = crate::ui_capture::framebuffer().map(|image| {
            image.export_png(
                output
                    .join(format!("{label}-unified.png"))
                    .to_str()
                    .unwrap(),
            );
        });
        crate::terminal_view::world_visibility_review::set_enabled(false);
        result
    }

    async fn capture_street_expansion(&mut self, output: &std::path::Path) -> Result<(), String> {
        let s = crate::prison_street::scene_for_game(&self.game);
        self.graphics.active.reduced_motion = false;
        let mut records = Vec::new();
        let manifest: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("assets/source-art/first-sector-review-01/u01-expansion-r06/street.json"),
            )
            .map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let mut views: Vec<(String, [i32; 2])> =
            serde_json::from_value(manifest["review_views"].clone()).map_err(|e| e.to_string())?;
        let defects_review = std::env::args().nth(1).as_deref() == Some("--ui-cold-street-defects");
        if defects_review {
            let index = views
                .iter()
                .position(|(label, _)| label == "house-a-closed")
                .unwrap()
                + 1;
            views.insert(index, ("door-hidden".to_owned(), [17, 19]));
        }
        let integrated = std::env::args().nth(1).as_deref() == Some("--ui-cold-street-integration");
        let circle_review = std::env::args().any(|arg| arg == "--vision-circle");
        let unified_review = circle_review || std::env::args().any(|arg| arg == "--vision-unified");
        let six_cell_vision = std::env::args().any(|arg| arg == "--vision-six");
        if six_cell_vision && self.game.rules().player_field_of_view.radius != 6 {
            return Err("Six-cell capture must use the real accepted perception rules".into());
        }
        let vision_review = defects_review
            || unified_review
            || six_cell_vision
            || std::env::args().any(|arg| arg == "--vision-layout");
        for (label, at) in views {
            if label.ends_with("-open") {
                let door = if label.starts_with("house-a") {
                    s.extra_doorways[0].door
                } else {
                    s.extra_doorways[1].door
                };
                if defects_review {
                    self.walk_fixture_to_unchecked(GridPos::new(door[0], door[1] + 1))?;
                }
                if self.execute_command(GameCommand::Interact {
                    target: crate::prison_street::pos(door),
                }) != CommandOutcome::Applied
                {
                    return Err(format!("Review door interaction failed: {label}"));
                }
            }
            self.walk_fixture_to_unchecked(crate::prison_street::pos(at))?;
            if defects_review
                && !matches!(
                    label.as_str(),
                    "gate"
                        | "join-middle"
                        | "court-clean"
                        | "house-a-closed"
                        | "house-a-open"
                        | "house-b-closed"
                        | "house-b-open"
                        | "door-hidden"
                        | "west-corner"
                )
            {
                continue;
            }
            if !defects_review
                && vision_review
                && !(unified_review && label == "court-clean")
                && !matches!(
                    label.as_str(),
                    "gate"
                        | "join-middle"
                        | "house-a-closed"
                        | "house-a-open"
                        | "house-b-open"
                        | "west-corner"
                )
            {
                continue;
            }
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
            if defects_review {
                if label == "door-hidden" {
                    let door = crate::prison_street::pos(s.extra_doorways[0].door);
                    if self.game.player_visibility().is_visible(door)
                        || self.game.map().tile(door).unwrap().terrain
                            != Terrain::Door(project_rl::world::DoorState::Closed)
                    {
                        return Err(
                            "Door popping fixture must have a closed door outside actor LOS".into(),
                        );
                    }
                    crate::prison_street_render::review_legacy_doors(true);
                    for _ in 0..2 {
                        self.draw();
                        next_frame().await;
                    }
                    self.draw();
                    crate::ui_capture::framebuffer()?
                        .export_png(output.join("door-hidden-avant.png").to_str().unwrap());
                    crate::prison_street_render::review_legacy_doors(false);
                }
                for (suffix, radius, ambient) in [
                    ("cercle8", Some(8), None),
                    ("cercle6", Some(6), None),
                    ("sombre", Some(6), Some([0.28, 0.30, 0.36])),
                ] {
                    crate::terminal_view::painted_scene_visibility::review_radius(radius);
                    crate::prison_street_render::review_ambient(ambient);
                    for _ in 0..2 {
                        self.draw();
                        next_frame().await;
                    }
                    self.draw();
                    crate::ui_capture::framebuffer()?.export_png(
                        output
                            .join(format!("{label}-{suffix}.png"))
                            .to_str()
                            .unwrap(),
                    );
                }
                crate::terminal_view::painted_scene_visibility::review_radius(None);
                crate::prison_street_render::review_ambient(None);
            }
            if unified_review {
                self.capture_unified_vision_after(output, &label).await?;
            }
            let r = self
                .terminal
                .world_cell_rect(
                    &self.game,
                    self.terminal_bounds(),
                    self.ui_scale(),
                    64,
                    self.navigation_signal_summary().is_some(),
                    crate::prison_street::pos(at),
                    Vec2::ZERO,
                )
                .ok_or("Review actor outside viewport")?;
            if r.w != 64. || r.h != 64. || suspension::fingerprint(&self.game) != before {
                return Err("Expansion rendering changed scale or world".into());
            }
            records.push(serde_json::json!({"scene":label,"player":at,"cell":[r.x,r.y,r.w,r.h],
                "roofs_hidden":s.roofs.iter().map(|r|r.hidden(&self.game)).collect::<Vec<_>>(),"render_read_only":true}));
        }
        let animations: Vec<(String, [i32; 2])> = if vision_review {
            Vec::new()
        } else {
            serde_json::from_value(manifest["review_animations"].clone())
                .map_err(|e| e.to_string())?
        };
        for (label, at) in animations {
            self.walk_fixture_to_unchecked(crate::prison_street::pos(at))?;
            self.capture_events_at(Some(0.));
            let before = suspension::fingerprint(&self.game);
            let r = self
                .terminal
                .world_cell_rect(
                    &self.game,
                    self.terminal_bounds(),
                    self.ui_scale(),
                    64,
                    self.navigation_signal_summary().is_some(),
                    crate::prison_street::pos(at),
                    Vec2::ZERO,
                )
                .ok_or("Animation actor outside camera")?;
            let crop = Rect::new((r.x - 320.).max(0.), (r.y - 256.).max(0.), 640., 384.);
            for i in 0..24 {
                let _time = crate::effect_geometry::frame_time(i as f64 * 0.125 + 0.4);
                self.draw();
                next_frame().await;
                self.draw();
                let frame = crate::ui_capture::framebuffer()?;
                frame
                    .sub_image(Rect::new(
                        crop.x,
                        frame.height as f32 - crop.y - crop.h,
                        crop.w,
                        crop.h,
                    ))
                    .export_png(output.join(format!("{label}-{i:02}.png")).to_str().unwrap());
            }
            if suspension::fingerprint(&self.game) != before {
                return Err("Animation mutated simulation".into());
            }
        }
        self.walk_fixture_to_unchecked(crate::prison_street::pos(s.entrance))?;
        if self.execute_command(GameCommand::Interact {
            target: crate::prison_street::pos(s.entrance),
        }) != CommandOutcome::Applied
            || self.game.current_zone().unwrap().id.as_str() != camp::ZONE_ID
        {
            return Err("Expansion return to camp failed".into());
        }
        if unified_review {
            self.world_presentation_time.set(1.);
            self.capture_events_at(Some(0.));
            let before = suspension::fingerprint(&self.game);
            for _ in 0..2 {
                self.draw();
                next_frame().await;
            }
            self.draw();
            crate::ui_capture::framebuffer()?.export_png(output.join("camp.png").to_str().unwrap());
            self.capture_unified_vision_after(output, "camp").await?;
            let at = self
                .game
                .player_position()
                .ok_or("Camp review player absent")?;
            let r = self
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
                .ok_or("Camp review actor outside viewport")?;
            if r.w != 64. || suspension::fingerprint(&self.game) != before {
                return Err("Unified camp review mutated world".into());
            }
            records.push(serde_json::json!({"scene":"camp","player":[at.x,at.y],"cell":[r.x,r.y,r.w,r.h],"render_read_only":true}));
        }
        std::fs::write(
            output.join("checks.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "review":if defects_review {"DEFECTS-U01-01"} else if integrated {"MAP-U01-EXP-01-r06 / VISION-U01-04 integrated"} else if circle_review {"VISION-U01-04"} else if unified_review { "VISION-U01-03" } else if vision_review { "VISION-U01-02" } else { "MAP-U01-EXP-01-r06" },
                "defects_review":defects_review,
                "circle_review":circle_review,"unified_review":unified_review,"vision_review":vision_review,"integrated":integrated,"candidate_only":!integrated,"dimensions":[s.width,s.height],
                "native_cell":64,"camp_round_trip":true,"animation_read_only":true,"scenes":records,
                "player_vision_radius":self.game.rules().player_field_of_view.radius,
                "generation":self.generation_version,"six_cell_vision":six_cell_vision,
                "isolated_save":self.suspension_path,"next_zone_not_generated":true
            }))
            .unwrap(),
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn capture_street_roof_stage(
        &mut self,
        output: &std::path::Path,
        label: &str,
        records: &mut Vec<serde_json::Value>,
    ) -> Result<(), String> {
        self.world_presentation_time.set(1.);
        self.capture_events_at(Some(0.));
        let before = suspension::fingerprint(&self.game);
        for _ in 0..2 {
            self.draw();
            next_frame().await;
        }
        self.draw();
        crate::ui_capture::framebuffer()?
            .export_png(output.join(format!("roof-{label}.png")).to_str().unwrap());
        if suspension::fingerprint(&self.game) != before {
            return Err("Roof rendering mutated the world".into());
        }
        let s = crate::prison_street::scene_for_game(&self.game);
        let door = crate::prison_street::pos(s.home_entry);
        let hidden = s.roofs[0].hidden(&self.game);
        let cell = self
            .terminal
            .world_cell_rect(
                &self.game,
                self.terminal_bounds(),
                self.ui_scale(),
                64,
                self.navigation_signal_summary().is_some(),
                door,
                Vec2::ZERO,
            )
            .map(|cell| [cell.x, cell.y, cell.w, cell.h]);
        let visibility = self.game.player_visibility();
        records.push(serde_json::json!({
            "scene":label,"player":self.game.player_position().map(|p| [p.x,p.y]),
            "door":format!("{:?}",self.game.map().tile(door).unwrap().terrain),
            "roof_hidden":hidden,"render_read_only":true,
            "door_cell":cell,
            "roof_exposure":s.roofs[0].exposure(|p| visibility.is_visible(p),|p| self.terminal.known(p).is_some()),
            "home_floor_visible":visibility.is_visible(crate::prison_street::pos(s.home_inside))
        }));
        next_frame().await;
        Ok(())
    }

    async fn capture_street_roof_integration(
        &mut self,
        output: &std::path::Path,
    ) -> Result<(), String> {
        use project_rl::world::DoorState;
        let s = crate::prison_street::scene_for_game(&self.game);
        let door = crate::prison_street::pos(s.home_entry);
        let mut records = Vec::new();
        self.capture_street_roof_stage(output, "unknown", &mut records)
            .await?;
        self.walk_fixture_to_unchecked(GridPos::new(door.x, door.y + 1))?;
        if self.game.map().tile(door).unwrap().terrain != Terrain::Door(DoorState::Closed) {
            return Err("New street door must start closed".into());
        }
        self.capture_street_roof_stage(output, "closed", &mut records)
            .await?;
        self.select_world_target(None);
        self.update_input_at(
            &InputFrame {
                pressed: [self.controls.binding(Action::Interact).clone()].into(),
                pointer: Some((1., 1.)),
                viewport: Some((1920., 1080.)),
                ..Default::default()
            },
            Some(0.),
        );
        if self.game.map().tile(door).unwrap().terrain != Terrain::Door(DoorState::Open) {
            return Err("Keyboard context did not open the door".into());
        }
        self.capture_street_roof_stage(output, "open", &mut records)
            .await?;
        self.walk_fixture_to_unchecked(door)?;
        self.capture_street_roof_stage(output, "threshold", &mut records)
            .await?;
        self.walk_fixture_to_unchecked(GridPos::new(door.x, door.y - 1))?;
        self.select_pointer_world_cell(door, None);
        if self.game.map().tile(door).unwrap().terrain != Terrain::Door(DoorState::Closed)
            || !s.roofs[0].hidden(&self.game)
        {
            return Err("Closing inside must keep the roof hidden".into());
        }
        self.capture_street_roof_stage(output, "inside-closed", &mut records)
            .await?;
        if self.execute_command(GameCommand::Interact { target: door }) != CommandOutcome::Applied {
            return Err("Could not reopen from inside".into());
        }
        self.walk_fixture_to_unchecked(GridPos::new(door.x, door.y + 1))?;
        self.select_pointer_world_cell(door, None);
        if s.roofs[0].hidden(&self.game) {
            return Err("Roof must return after closing outside".into());
        }
        self.capture_street_roof_stage(output, "reclosed", &mut records)
            .await?;
        self.walk_fixture_to_unchecked(crate::prison_street::pos(s.entrance))?;
        if self.execute_command(GameCommand::Interact {
            target: crate::prison_street::pos(s.entrance),
        }) != CommandOutcome::Applied
            || self.game.current_zone().unwrap().id.as_str() != camp::ZONE_ID
        {
            return Err("Camp return failed after door interactions".into());
        }
        std::fs::write(
            output.join("checks.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "review":"ROOF-U01-01-r01","integrated":true,"generation":self.generation_version,
                "lighting_production_enabled":s.atmosphere.is_some(),"native_cell":64,
                "keyboard_without_hover":true,"left_click_interaction":true,
                "camp_round_trip":true,"scenes":records,
                "isolated_save":self.suspension_path
            }))
            .unwrap(),
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn capture_target_outline(output: &std::path::Path) -> Result<(), String> {
        use project_rl::{
            monster_art::{ArtRole, MONSTER_ART},
            social::PlayerRelation,
        };
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let _clock = crate::camp_npc_review::begin();
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(1810, rules, texts, loot, expeditions)?;
        app.suspension_path = output.join("isolated-run.json");
        app.begin_character_creation_with_seed(false, 1810)?;
        app.update_input(&InputFrame {
            pressed: [crate::controls::Binding::key("Enter")].into(),
            ..Default::default()
        });
        request_new_screen_size(1600., 1000.);
        for _ in 0..3 {
            next_frame().await;
        }
        let large = MONSTER_ART
            .iter()
            .find(|a| a.variant == 0 && a.side >= 192 && a.role == ArtRole::DemonMelee)
            .ok_or("Grand démon absent")?;
        let ally = MONSTER_ART
            .iter()
            .find(|a| a.variant == 0 && a.role == ArtRole::PossessedMelee)
            .ok_or("Corps allié absent")?;
        // Review fixtures only: never installed by the ordinary camp startup.
        for (at, number, relation) in [
            ([22, 8], 164, PlayerRelation::Hostile),
            ([24, 8], ally.number, PlayerRelation::Allied),
            ([26, 8], large.number, PlayerRelation::Hostile),
        ] {
            app.game
                .spawn_actor(
                    Actor::new(pos(at), 30)
                        .map_err(|e| e.to_string())?
                        .with_ai(AiProfile::idle())
                        .with_player_relation(relation)
                        .with_tags(vec![
                            format!("core:monster_art_{number}_0").parse().unwrap(),
                        ]),
                )
                .map_err(|e| e.to_string())?;
        }
        let mut scenes = Vec::new();
        for (name, at, approach) in [
            ("officer", [6, 7], [5, 7]),
            ("medical", [18, 13], [18, 12]),
            ("cultist", [22, 8], [21, 8]),
            ("ally", [24, 8], [23, 8]),
            ("large-demon", [26, 8], [25, 8]),
        ] {
            app.walk_fixture_to_unchecked(pos(approach))?;
            let tone =
                crate::target_outline::tone_at(&app.game, pos(at)).ok_or("Relation non perçue")?;
            let before = suspension::fingerprint(&app.game);
            let mut cell_rect = None;
            for phase in 0..3 {
                crate::camp_npc_review::set_capture_time(if phase == 2 {
                    0.
                } else {
                    phase as f64 * 1.5
                });
                let review = (phase != 2).then(|| crate::target_outline::review(pos(at)));
                app.draw();
                next_frame().await;
                app.draw();
                let image = crate::ui_capture::framebuffer()?;
                image.export_png(output.join(format!("{name}-{phase}.png")).to_str().unwrap());
                drop(review);
                let r = app
                    .terminal
                    .world_cell_rect(
                        &app.game,
                        app.terminal_bounds(),
                        app.ui_scale(),
                        64,
                        app.navigation_signal_summary().is_some(),
                        pos(at),
                        Vec2::ZERO,
                    )
                    .ok_or("Cible hors caméra")?;
                if r.w != 64. {
                    return Err("Échelle native modifiée".into());
                }
                cell_rect = Some([r.x, r.y, r.w, r.h]);
                next_frame().await;
            }
            if suspension::fingerprint(&app.game) != before {
                return Err("Le survol a modifié le jeu".into());
            }
            scenes.push(serde_json::json!({"name":name,"at":at,"tone":format!("{tone:?}"),"cell":cell_rect,"render_is_read_only":true}));
        }
        // Unlit GPU samples isolate the stroke, alpha holes and current pose.
        let visuals = [
            crate::game_art::Visual::CampNpc(1),
            app.front_player_visual(),
            crate::game_art::Visual::DroneScout,
            crate::game_art::Visual::CampaignMonster {
                number: 164,
                variant: 0,
            },
            crate::game_art::Visual::CampaignMonster {
                number: large.number,
                variant: 0,
            },
        ];
        let mut masks = Vec::new();
        for pass in 0..2 {
            let review = crate::target_outline::review(GridPos::new(0, 0));
            set_default_camera();
            unsafe {
                macroquad::window::get_internal_gl().quad_gl.scissor(None);
            }
            clear_background(Color::from_rgba(18, 23, 30, 255));
            for (i, &visual) in visuals.iter().enumerate() {
                let r = Rect::new(128. + i as f32 * 288., 400., 64., 64.);
                crate::game_art::draw_world_animated(r, visual, 1., Some(0.));
                if pass == 1 {
                    crate::target_outline::draw(
                        r,
                        visual,
                        Some(0.),
                        if i < 3 {
                            crate::target_outline::Tone::Friendly
                        } else {
                            crate::target_outline::Tone::Hostile
                        },
                        None,
                    );
                    let placement = crate::target_outline::export_mask(
                        visual,
                        r,
                        &output.join(format!("mask-{i}.png")),
                    );
                    masks.push(serde_json::json!({"index":i,"placement":placement,"tone":if i<3 {"Friendly"} else {"Hostile"}}));
                }
            }
            crate::ui_capture::framebuffer()?.export_png(
                output
                    .join(format!("silhouettes-{pass}.png"))
                    .to_str()
                    .unwrap(),
            );
            drop(review);
            next_frame().await;
        }
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":"HOVER-OUTLINE-01","revision":"r01","outline_production_enabled":false,"npc_presentation_production_enabled":true,"native_cell":64,"border_px":2,"large_demon":large.number,"scenes":scenes,"masks":masks})).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }

    pub async fn capture_camp_npc_review(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let _review = crate::camp_npc_review::begin();
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(1810, rules, texts, loot, expeditions)?;
        app.suspension_path = output.join("isolated-run.json");
        app.begin_character_creation_with_seed(false, 1810)?;
        app.update_input(&InputFrame {
            pressed: [crate::controls::Binding::key("Enter")].into(),
            ..Default::default()
        });
        if app.character_creation.is_some() {
            return Err("Création du personnage incomplète".into());
        }
        request_new_screen_size(1600., 1000.);
        for _ in 0..3 {
            next_frame().await;
        }
        let mut records = Vec::new();
        for (index, at, approach) in [
            (1, [6, 7], [5, 7]),
            (2, [31, 4], [30, 4]),
            (3, [18, 13], [18, 12]),
            (4, [31, 13], [30, 13]),
        ] {
            app.walk_fixture_to_unchecked(pos(approach))?;
            let before = suspension::fingerprint(&app.game);
            for phase in 0..2 {
                crate::camp_npc_review::set_capture_time(phase as f64 * 1.5);
                app.draw();
                next_frame().await;
                app.draw();
                let image = crate::ui_capture::framebuffer()?;
                image.export_png(
                    output
                        .join(format!("npc-{index}-scene-{phase}.png"))
                        .to_str()
                        .unwrap(),
                );
                let cell = app
                    .terminal
                    .world_cell_rect(
                        &app.game,
                        app.terminal_bounds(),
                        app.ui_scale(),
                        64,
                        app.navigation_signal_summary().is_some(),
                        pos(at),
                        Vec2::ZERO,
                    )
                    .ok_or("PNJ hors caméra")?;
                if cell.w != 64.
                    || crate::camp_npc_review::marker_at(&app.game, pos(at)) != Some(index)
                {
                    return Err("Échelle ou pictogramme absent".into());
                }
                if phase == 0 {
                    records.push(serde_json::json!({"index":index,"at":at,"cell":[cell.x,cell.y,cell.w,cell.h],"render_is_read_only":true}));
                }
                next_frame().await;
            }
            if suspension::fingerprint(&app.game) != before {
                return Err("La revue a modifié la simulation".into());
            }
        }
        // Isolate the four sprites to verify fixed feet, shadows and portraits
        // without changes from the already accepted scene lighting.
        for phase in 0..3 {
            crate::camp_npc_review::set_capture_time(phase as f64 * 1.5);
            set_default_camera();
            clear_background(Color::from_rgba(18, 23, 30, 255));
            for index in 1..=4 {
                let cell = Rect::new(160. + (index - 1) as f32 * 256., 160., 64., 64.);
                crate::game_art::draw_world_animated(
                    cell,
                    crate::game_art::Visual::CampNpc(index),
                    1.,
                    (phase != 2).then_some(0.),
                );
                crate::camp_npc_review::draw_marker(cell, index, 0.);
                crate::game_art::draw_portrait(
                    Rect::new(cell.x, 320., 128., 128.),
                    crate::game_art::Visual::CampNpc(index),
                );
            }
            let image = crate::ui_capture::framebuffer()?;
            image.export_png(
                output
                    .join(format!("npc-poses-{phase}.png"))
                    .to_str()
                    .unwrap(),
            );
            next_frame().await;
        }
        app.graphics.active.reduced_motion = true;
        crate::camp_npc_review::set_capture_time(1.5);
        app.draw();
        crate::ui_capture::framebuffer()?
            .export_png(output.join("npc-reduced-motion.png").to_str().unwrap());
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":"CAMP-NPC-01","revision":"r01","production_enabled":false,"native_cell":64,"pose_seconds":1.5,"offset_px":2,"scenes":records})).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }

    pub async fn capture_cryogenic_camp(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(1810, rules, texts, loot, expeditions)?;
        app.suspension_path = output.join("isolated-run.json");
        app.begin_character_creation_with_seed(false, 1810)?;
        app.update_input(&InputFrame {
            pressed: [crate::controls::Binding::key("Enter")].into(),
            ..Default::default()
        });
        if app.character_creation.is_some() {
            return Err("Création du personnage incomplète".into());
        }
        request_new_screen_size(1600., 1000.);
        for _ in 0..3 {
            next_frame().await;
        }
        app.world_presentation_time.set(1.);
        let mut records = Vec::new();
        for (name, at, npc) in [
            ("start", [2, 8], None),
            ("officer", [5, 7], Some([6, 7])),
            ("hall", [22, 8], None),
            ("medical", [18, 12], None),
            ("armorer", [30, 4], Some([31, 4])),
            ("technician", [30, 13], Some([31, 13])),
        ] {
            app.npc_interaction = None;
            app.walk_fixture_to_unchecked(pos(at))?;
            if let Some(npc) = npc {
                app.interaction_command_at(pos(npc));
            }
            let before = suspension::fingerprint(&app.game);
            for _ in 0..2 {
                app.draw();
                next_frame().await;
            }
            app.draw();
            let image = crate::ui_capture::framebuffer()?;
            image.export_png(output.join(format!("camp-{name}.png")).to_str().unwrap());
            let mut native_cell = None;
            if npc.is_none() {
                let navigation = app.navigation_signal_summary().is_some();
                let viewport = crate::terminal_view::terminal_playfield(
                    &app.game,
                    app.terminal_bounds(),
                    app.ui_scale(),
                    navigation,
                );
                let pixel = |x: f32, y: f32| {
                    let i = ((usize::from(image.height) - 1 - y as usize)
                        * usize::from(image.width)
                        + x as usize)
                        * 4;
                    &image.bytes[i..i + 3]
                };
                // The post-process must not spill into the gap beside the HUD.
                let gap = pixel(viewport.right() + 2., viewport.center().y);
                // The existing violet bezel reaches 31; lit floor/furniture
                // must never appear in this border sample.
                if gap.iter().copied().max().unwrap_or(0) > 40 {
                    return Err(format!("L’éclairage déborde sur l’interface : {gap:?}"));
                }
                let cell = app
                    .terminal
                    .world_cell_rect(
                        &app.game,
                        app.terminal_bounds(),
                        app.ui_scale(),
                        app.graphics.active.world_cell_px,
                        navigation,
                        pos(at),
                        Vec2::ZERO,
                    )
                    .ok_or("Joueur hors caméra")?;
                if cell.w != 64. || cell.h != 64. {
                    return Err("Échelle native modifiée".into());
                }
                native_cell = Some(cell.w);
                if name == "start" {
                    let unknown = GridPos::new(-2, 8);
                    let hidden = app
                        .terminal
                        .world_cell_rect(
                            &app.game,
                            app.terminal_bounds(),
                            app.ui_scale(),
                            64,
                            navigation,
                            unknown,
                            Vec2::ZERO,
                        )
                        .ok_or("Témoin de brouillard hors caméra")?;
                    if app.terminal.known(unknown).is_some()
                        || pixel(hidden.center().x, hidden.center().y) != [0, 0, 0]
                    {
                        return Err("Le rendu révèle une case inconnue".into());
                    }
                }
            }
            if before != suspension::fingerprint(&app.game) {
                return Err("Le rendu a modifié la simulation".into());
            }
            records.push(serde_json::json!({"name":name,"position":at,"turn":app.game.turn(),"render_is_read_only":true,"native_cell":native_cell,"hud_clip_checked":npc.is_none(),"size":[image.width,image.height]}));
        }
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({"generation_version":app.generation_version,"zone":camp::ZONE_ID,"character_class":app.character_class.as_ref().map(|id|id.as_str()),"native_cell":app.graphics.active.world_cell_px,"scenes":records,"isolated_save_path":app.suspension_path})).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }
}
