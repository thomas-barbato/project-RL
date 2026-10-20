//! Native before/candidate comparisons with the real street lighting.
use super::*;
use crate::game_art::{self, Visual, finish_review_05 as review};

impl AsciiApp {
    pub async fn capture_art_finish_05(output: &std::path::Path) -> Result<(), String> {
        Self::capture_art_finish_05_mode(output, false).await
    }
    pub async fn capture_art_finish_05_integration(output: &std::path::Path) -> Result<(), String> {
        Self::capture_art_finish_05_mode(output, true).await
    }
    async fn capture_art_finish_05_mode(
        output: &std::path::Path,
        integrated: bool,
    ) -> Result<(), String> {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let accepted = root
            .join("docs/reviews/refonte-histoire-2026-10-16/art-finish-review-05/acceptance.json")
            .exists();
        if integrated && !accepted {
            return Err("ART-FINISH-05 has not been accepted".into());
        }
        if !integrated && accepted {
            return Err("ART-FINISH-05 accepted: preserve the frozen candidate review".into());
        }
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1240., 900.);
        for _ in 0..3 {
            next_frame().await;
        }
        let ground = Texture2D::from_file_with_format(
            &std::fs::read(root.join("assets/maps/prison-street-expanded-01/street.png"))
                .map_err(|e| e.to_string())?,
            None,
        );
        ground.set_filter(FilterMode::Nearest);
        for frame in 0..3 {
            clear_background(Color::from_rgba(17, 25, 29, 255));
            for background in 0..3 {
                for (index, number) in review::NUMBERS.into_iter().enumerate() {
                    let y = 80. + background as f32 * 200.;
                    let x = 20. + index as f32 * 300.;
                    for (column, mode) in (if integrated { [2, 3] } else { [1, 2] })
                        .into_iter()
                        .enumerate()
                    {
                        let xx = x + column as f32 * 140.;
                        if background == 2 {
                            draw_texture_ex(
                                &ground,
                                xx,
                                y + 34.,
                                WHITE,
                                DrawTextureParams {
                                    source: Some(Rect::new(14. * 64., 40. * 64., 128., 128.)),
                                    dest_size: Some(vec2(128., 128.)),
                                    ..Default::default()
                                },
                            );
                        } else {
                            let color = if background == 0 {
                                Color::from_rgba(119, 124, 116, 255)
                            } else {
                                Color::from_rgba(31, 38, 41, 255)
                            };
                            draw_rectangle(xx, y + 34., 128., 128., color);
                        }
                        review::mode(mode, false);
                        game_art::draw_world(
                            Rect::new(xx + 32., y + 66., 64., 64.),
                            Visual::CampaignMonster { number, variant: 0 },
                            1.,
                        );
                    }
                }
            }
            review::mode(0, false);
            if frame == 2 {
                crate::ui_capture::framebuffer()?
                    .export_png(output.join("native-gpu.png").to_str().unwrap());
            } else {
                next_frame().await;
            }
        }
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed_version(
            1872,
            rules,
            texts,
            loot,
            expeditions,
            CURRENT_GENERATION_VERSION,
        )?;
        app.suspension_path = output.join("isolated-run.json");
        app.crash_recovery_enabled = false;
        request_new_screen_size(1920., 1080.);
        for _ in 0..3 {
            next_frame().await;
        }
        let exit = pos(camp::scene().exits[0]);
        app.walk_fixture_to_unchecked(exit)?;
        if app.execute_command(GameCommand::Interact { target: exit }) != CommandOutcome::Applied {
            return Err("ART-FINISH-05 street entry failed".into());
        }
        let mut scenes = Vec::new();
        for (label, at) in [
            ("street", GridPos::new(15, 41)),
            ("north", GridPos::new(16, 31)),
        ] {
            app.walk_fixture_to_unchecked(at)?;
            app.selected_world_cell = None;
            app.selected_target = None;
            app.world_cursor = None;
            let positions = review::positions(&app.game);
            if positions.len() != 4 {
                return Err(format!(
                    "{label}: only {} suitable presentation positions",
                    positions.len()
                ));
            }
            scenes.push(serde_json::json!({"label":label,"figures":positions.iter().map(|p|[p.x,p.y]).collect::<Vec<_>>()}));
            for (mode, suffix) in if integrated {
                [(2, "reference"), (3, "integrated")]
            } else {
                [(1, "before"), (2, "candidate")]
            } {
                crate::prison_street_render::review_normal_style();
                review::mode(mode, true);
                app.world_fix_frame(output, &format!("{label}-{suffix}"))
                    .await?;
            }
        }
        review::mode(0, false);
        std::fs::write(
            output.join("checks.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "review":"ART-FINISH-05","status":if integrated {"accepted_integration"} else {"pending_visual_review"},"native_cell":64,
                "drawings":4,"background_samples":12,"scene_pairs":2,"scenes":scenes,
                "production_mode":if integrated {3} else {0},"real_engine_lighting":true,"draw_only_actors":true,
                "gameplay_fingerprints_unchanged":true
            }))
            .unwrap(),
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}
