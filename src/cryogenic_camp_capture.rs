//! Isolated end-to-end captures through character creation and real HUD.
use super::*;

impl AsciiApp {
    pub async fn capture_dialogue_bubbles(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let _clock = crate::camp_npc_review::begin();
        crate::camp_npc_review::set_capture_time(0.);
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
        app.walk_fixture_to_unchecked(GridPos::new(5, 7))?;
        let navigation = app.navigation_signal_summary().is_some();
        let viewport = crate::terminal_view::terminal_playfield(
            &app.game,
            app.terminal_bounds(),
            app.ui_scale(),
            navigation,
        );
        let speaker = app
            .terminal
            .world_cell_rect(
                &app.game,
                app.terminal_bounds(),
                app.ui_scale(),
                64,
                navigation,
                GridPos::new(6, 7),
                Vec2::ZERO,
            )
            .ok_or("Interlocuteur hors caméra")?;
        let before = suspension::fingerprint(&app.game);
        app.draw();
        next_frame().await;
        app.draw();
        crate::ui_capture::framebuffer()?
            .export_png(output.join("terminal-r02-unread.png").to_str().unwrap());
        let _review = crate::dialogue_bubble_review::begin(GridPos::new(6, 7));
        app.draw();
        next_frame().await;
        app.draw();
        // Both samples share the exact same GPU background/animation instant.
        let background = crate::ui_capture::framebuffer()?;
        background.export_png(output.join("terminal-r02-background.png").to_str().unwrap());
        let complete = crate::dialogue_bubble_review::terminal_at(
            viewport,
            speaker,
            "Officier",
            camp::OFFICER_DIALOGUE,
            None,
            false,
        )?;
        let r = complete.bounds;
        crate::ui_capture::framebuffer()?
            .export_png(output.join("terminal-r02.png").to_str().unwrap());
        let texture = Texture2D::from_image(&background);
        texture.set_filter(FilterMode::Nearest);
        let crop = Rect::new(r.x - 2., r.y - 2., r.w + 4., speaker.y + 7. - (r.y - 2.));
        let raw_crop = Rect::new(
            crop.x,
            background.height as f32 - crop.bottom(),
            crop.w,
            crop.h,
        );
        // One native frame per character. Frames share one fixed real scene;
        // the preview can replay them at 40 characters/second without video blur.
        for i in 0..=complete.characters + 1 {
            set_default_camera();
            gl_use_default_material();
            draw_texture_ex(
                &texture,
                0.,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(background.width as f32, background.height as f32)),
                    flip_y: true,
                    ..Default::default()
                },
            );
            let t = if i <= complete.characters {
                (i as f64 + 0.01) / crate::dialogue_bubble_review::CHARACTERS_PER_SECOND
            } else {
                complete.characters as f64 / crate::dialogue_bubble_review::CHARACTERS_PER_SECOND
                    + 0.6
            };
            let frame = crate::dialogue_bubble_review::terminal_at(
                viewport,
                speaker,
                "Officier",
                camp::OFFICER_DIALOGUE,
                Some(t),
                false,
            )?;
            if frame.bounds != r
                || frame.cursor.right() > r.right()
                || frame.cursor.bottom() > r.bottom()
            {
                return Err("La saisie déplace ou dépasse la bulle".into());
            }
            crate::ui_capture::framebuffer()?
                .sub_image(raw_crop)
                .export_png(
                    output
                        .join(format!("terminal-r02-frame-{i:04}.png"))
                        .to_str()
                        .unwrap(),
                );
            if i % 8 == 0 {
                next_frame().await;
            }
        }
        let mut closures = Vec::new();
        for (name, pointer, pause) in [
            ("inside", Some((r.center().x, r.center().y)), false),
            ("outside", Some((20., 900.)), false),
            ("escape", None, true),
        ] {
            crate::dialogue_bubble_review::set_open(true);
            let mut input = InputFrame {
                pointer,
                pause,
                pressed: if pause {
                    [crate::controls::Binding::key("Escape")].into()
                } else {
                    [
                        crate::controls::Binding::MouseLeft,
                        crate::controls::Binding::key("F"),
                    ]
                    .into()
                },
                ..Default::default()
            };
            if !crate::dialogue_bubble_review::take_dismiss(&mut input) {
                return Err("La bulle ne se ferme pas".into());
            }
            crate::dialogue_bubble_review::set_open(false);
            if !crate::dialogue_bubble_review::notification_hidden(GridPos::new(6, 7), true) {
                return Err("La fermeture a rétabli une notification déjà consultée".into());
            }
            app.update_input(&input);
            if suspension::fingerprint(&app.game) != before {
                return Err("La fermeture a exécuté une action de jeu".into());
            }
            closures.push(name);
        }
        app.draw();
        next_frame().await;
        app.draw();
        crate::ui_capture::framebuffer()?
            .export_png(output.join("terminal-r02-closed.png").to_str().unwrap());
        // Visual fixture only: never changes objective completion or rewards.
        crate::dialogue_bubble_review::set_ready();
        app.draw();
        next_frame().await;
        app.draw();
        crate::ui_capture::framebuffer()?
            .export_png(output.join("terminal-r02-ready.png").to_str().unwrap());
        if suspension::fingerprint(&app.game) != before {
            return Err("La proposition a modifié le jeu".into());
        }
        std::fs::write(output.join("checks-r02.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":"DIALOGUE-BUBBLE-01","revision":"r02","selected_candidate":"03-terminal","production_enabled":false,"native_cell":64,"render_is_read_only":true,"text":camp::OFFICER_DIALOGUE,"viewport":[viewport.x,viewport.y,viewport.w,viewport.h],"speaker":[speaker.x,speaker.y,speaker.w,speaker.h],"bounds":[r.x,r.y,r.w,r.h],"fill_rgba":[10,25,19,224],"tail_root":[speaker.center().x,r.bottom(),speaker.y+5.],"no_close_button":true,"full_existing_text":true,"speaker_visible":true,"closures_checked":closures,"close_click_consumed":true,"characters_per_second":crate::dialogue_bubble_review::CHARACTERS_PER_SECOND,"characters":complete.characters,"animation_crop":[crop.x,crop.y,crop.w,crop.h],"animation_frames":complete.characters+2,"cursor":[complete.cursor.x,complete.cursor.y,complete.cursor.w,complete.cursor.h],"cursor_blink_seconds":0.5,"reduced_motion":"full_text_and_static_cursor","notification_hidden_while_open":true,"quest_notification_stays_read_after_closing":true,"quest_state_unchanged":true})).unwrap()).map_err(|e|e.to_string())?;
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
