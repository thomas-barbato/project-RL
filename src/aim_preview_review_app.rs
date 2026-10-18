//! Native-size comparisons and integration checks in disposable games.
use super::*;
use crate::attack_preview_visuals::{self as aim_preview_review, Style};

impl AsciiApp {
    pub async fn capture_aim_styles(
        output: &std::path::Path,
        combined: bool,
    ) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1280., 800.);
        for _ in 0..3 {
            next_frame().await;
        }
        fn click_at(app: &AsciiApp, at: GridPos) -> Result<InputFrame, String> {
            let cell = app
                .terminal
                .world_cell_rect(
                    &app.game,
                    app.terminal_bounds(),
                    app.ui_scale(),
                    app.graphics.active.world_cell_px,
                    app.navigation_signal_summary().is_some(),
                    at,
                    app.camera_glide_offset(),
                )
                .ok_or("Case de diagnostic hors caméra")?;
            Ok(app.graphics.active.transform_input(InputFrame {
                pointer: Some(cell.center().into()),
                viewport: Some((1280., 800.)),
                pressed: [controls::Binding::MouseLeft].into(),
                ..Default::default()
            }))
        }
        let styles: &[(&str, Option<Style>)] = if combined {
            &[("choix", Some(Style::Combined))]
        } else {
            &[
                ("actuel", None),
                ("a", Some(Style::Hologram)),
                ("b", Some(Style::Seal)),
                ("c", Some(Style::Etched)),
            ]
        };
        let mut pictures = Vec::new();
        let mut checks = Vec::new();
        let target = GridPos::new(6, 4);
        let technique = project_rl::skills::agent::technique("A02");
        for valid in [true, false] {
            let mut app = Self::new_agent_blast_trial()?;
            app.suspension_path = output.join("isolated-run.json");
            app.crash_recovery_enabled = false;
            app.graphics.active.world_cell_px = 64;
            app.graphics.active.reduced_motion = true;
            if combined && valid {
                let slot = app.active_weapon_slot;
                app.active_weapon_slot = 1;
                let click = click_at(&app, GridPos::new(6, 5))?;
                let before = suspension::fingerprint(&app.game);
                let history = app.history.len();
                app.update_input(&click);
                if app.attack_aim.is_some()
                    || suspension::fingerprint(&app.game) != before
                    || app.history.len() != history
                {
                    return Err("Un aperçu de trajet a exécuté une attaque".into());
                }
                app.update_input(&InputFrame {
                    pressed: [controls::Binding::MouseRight].into(),
                    ..Default::default()
                });
                app.active_weapon_slot = slot;
            }
            let aim = AttackAim {
                slot: app.active_weapon_slot,
                cursor: target,
            };
            app.attack_aim = Some(aim);
            app.attack_aim_technique = Some(technique.clone());
            let footprint = app
                .aimed_attack_footprint(aim)
                .map_err(|e| format!("Empreinte : {e:?}"))?;
            if !valid {
                // Obtain a real rejection (energy/cooldown), without changing
                // validity rules or any normal campaign/saved game.
                let mut refused = false;
                for _ in 0..64 {
                    if matches!(
                        app.execute_command(GameCommand::UseTechniqueAt {
                            technique: technique.clone(),
                            target,
                            weapon_slot: app.active_weapon_slot,
                        }),
                        CommandOutcome::Rejected(_)
                    ) {
                        refused = true;
                        break;
                    }
                }
                if !refused {
                    return Err("Diagnostic sans rejet de compétence".into());
                }
            }
            app.combat_motion.clear();
            app.visual_cues.clear_world();
            app.select_world_target(Some(target));
            app.attack_aim = Some(AttackAim {
                slot: app.active_weapon_slot,
                cursor: target,
            });
            app.attack_aim_technique = Some(technique.clone());
            let aim = app.attack_aim.unwrap();
            let preview = app.aimed_attack_preview(aim);
            if preview.is_ok() != valid || footprint.cells().len() != 9 {
                return Err(format!(
                    "État de revue incorrect : {preview:?}, {} cases",
                    footprint.cells().len()
                ));
            }
            let before = suspension::fingerprint(&app.game);
            let state = if valid { "valide" } else { "invalide" };
            for &(code, style) in styles {
                // The selected pair uses the normal runtime renderer without
                // a review override or a cached footprint.
                let _guard = (!combined).then(|| aim_preview_review::begin(style));
                for _ in 0..3 {
                    app.draw();
                    next_frame().await;
                }
                app.draw();
                let image = crate::ui_capture::framebuffer()?;
                if (image.width, image.height) != (1280, 800) {
                    return Err("Capture attendue en 1280 × 800".into());
                }
                image.export_png(
                    output
                        .join(format!("{code}-{state}.png"))
                        .to_str()
                        .ok_or("Chemin de capture")?,
                );
                let texture = Texture2D::from_image(&image);
                texture.set_filter(FilterMode::Nearest);
                pictures.push(texture);
                if suspension::fingerprint(&app.game) != before {
                    return Err("Le dessin a modifié la partie de diagnostic".into());
                }
                next_frame().await;
            }
            checks.push(serde_json::json!({
                "state":state, "actual_validity":preview.is_ok(),
                "rejection":preview.err().map(|e| format!("{e:?}")),
                "cells":footprint.cells().len(), "draw_preserved_game":true,
                "energy":app.game.player_energy().available(),
                "footprint_computed_by_engine":app.aimed_attack_footprint(aim).is_ok_and(|p| p.cells()==footprint.cells()),
            }));
            if combined && valid {
                let turn = app.game.turn();
                app.update_input(&click_at(&app, target)?);
                if app.attack_aim.is_some() || app.game.turn() != turn + 1 {
                    return Err("La compétence sélectionnée n'est plus confirmée au clic".into());
                }
            }
        }
        // Native-size source crops, composed by the same GPU renderer. There
        // is no resampling or external repainting of the captured scene.
        set_default_camera();
        // Framebuffer images are bottom-up in memory (PNG export flips them).
        // Texture upload preserves that memory orientation: mirror the source
        // Y coordinate and flip the drawn crop to match the upright PNGs.
        let crop = Rect::new(520., 800. - 232. - 288., 384., 288.);
        if combined {
            request_new_screen_size(960., 480.);
            for _ in 0..3 {
                next_frame().await;
            }
            for frame in 0..2 {
                clear_background(Color::from_rgba(12, 18, 23, 255));
                crate::ui_theme::draw_text_bold(
                    "Zone de compétence · A + repères de B",
                    40.,
                    32.,
                    23.,
                    Color::from_rgba(230, 225, 208, 255),
                );
                crate::ui_theme::draw_text(
                    "AIM-VIS-02 · captures natives · 64 px par case",
                    40.,
                    56.,
                    16.,
                    Color::from_rgba(165, 183, 190, 255),
                );
                for (i, title) in [
                    "Valide · hologramme bleu + cercles",
                    "Invalide · style B rouge",
                ]
                .iter()
                .enumerate()
                {
                    let x = 40. + i as f32 * 496.;
                    crate::ui_theme::draw_text_bold(
                        title,
                        x,
                        92.,
                        18.,
                        Color::from_rgba(230, 225, 208, 255),
                    );
                    draw_texture_ex(
                        &pictures[i],
                        x,
                        108.,
                        WHITE,
                        DrawTextureParams {
                            source: Some(crop),
                            flip_y: true,
                            ..Default::default()
                        },
                    );
                }
                crate::ui_theme::draw_text(
                    "Halo bleu, transparence et petits cercles · rouge franc hachuré pour une action impossible",
                    40.,
                    433.,
                    17.,
                    Color::from_rgba(165, 183, 190, 255),
                );
                crate::ui_theme::draw_text(
                    "Version validée · intégrée au jeu",
                    40.,
                    459.,
                    16.,
                    Color::from_rgba(165, 183, 190, 255),
                );
                if frame == 1 {
                    let image = crate::ui_capture::framebuffer()?;
                    if (image.width, image.height) != (960, 480) {
                        return Err("Planche attendue en 960 × 480".into());
                    }
                    image.export_png(
                        output
                            .join("choix.png")
                            .to_str()
                            .ok_or("Chemin de planche")?,
                    );
                }
                next_frame().await;
            }
            std::fs::write(output.join("choix-checks.json"), serde_json::to_vec_pretty(&serde_json::json!({
                "review":"AIM-VIS-02", "status":"integrated_after_explicit_approval", "normal_runs_changed":true,
                "frame_pixels":[1280,800], "comparison_pixels":[960,480], "tile_pixels":64, "crop":[520,232,384,288],
                "source_pixels_per_display_pixel":1, "states":checks,
                "selection":{"valid":"A hologram with small circular contour markers inspired by B", "invalid":"B unchanged"},
                "invalid_rgb":[255,48,38], "orientation":"upright, native framebuffer texture Y corrected",
                "normal_runtime_renderer":true, "review_override":false,
                "empty_click_then_cancel_without_skill":"free movement preview, no attack or game/history change",
                "selected_skill_left_click":"confirmed, exactly one turn",
            })).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
            return Ok(());
        }
        for frame in 0..2 {
            clear_background(Color::from_rgba(12, 18, 23, 255));
            crate::ui_theme::draw_text_bold(
                "Zones de compétence · AIM-VIS-01",
                24.,
                29.,
                23.,
                Color::from_rgba(230, 225, 208, 255),
            );
            crate::ui_theme::draw_text(
                "Trois propositions · captures du moteur · taille native 64 px",
                24.,
                51.,
                16.,
                Color::from_rgba(165, 183, 190, 255),
            );
            for (i, title) in ["A · Hologramme", "B · Sceau occulte", "C · Gravure"]
                .iter()
                .enumerate()
            {
                let x = 24. + i as f32 * 424.;
                crate::ui_theme::draw_text_bold(
                    title,
                    x,
                    78.,
                    21.,
                    Color::from_rgba(231, 225, 207, 255),
                );
                for (row, label) in ["VALIDE", "INVALIDE"].iter().enumerate() {
                    let y = if row == 0 { 108. } else { 462. };
                    crate::ui_theme::draw_text(
                        label,
                        x,
                        y - 9.,
                        15.,
                        Color::from_rgba(170, 188, 194, 255),
                    );
                    draw_texture_ex(
                        &pictures[row * 4 + i + 1],
                        x,
                        y,
                        WHITE,
                        DrawTextureParams {
                            source: Some(crop),
                            flip_y: true,
                            ..Default::default()
                        },
                    );
                }
            }
            crate::ui_theme::draw_text(
                "Contours et motifs à comparer · aucune proposition activée dans la partie normale",
                24.,
                783.,
                17.,
                Color::from_rgba(168, 185, 192, 255),
            );
            if frame == 1 {
                crate::ui_capture::framebuffer()?.export_png(
                    output
                        .join("comparaison.png")
                        .to_str()
                        .ok_or("Chemin de planche")?,
                );
            }
            next_frame().await;
        }
        std::fs::write(output.join("checks.json"), serde_json::to_vec_pretty(&serde_json::json!({
            "review":"AIM-VIS-01", "status":"awaiting_visual_approval", "normal_runs_changed":false,
            "frame_pixels":[1280,800], "tile_pixels":64, "crop":[520,232,384,288],
            "source_pixels_per_display_pixel":1, "states":checks,
            "candidates":["A · Hologramme", "B · Sceau occulte", "C · Gravure"],
        })).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
        Ok(())
    }
}
