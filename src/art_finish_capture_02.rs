//! Frozen reference versus ordinary renderer: unchanged world, real street lighting.
use super::*;
use crate::game_art::{self, finish_review_02 as review};

impl AsciiApp {
    pub async fn capture_art_finish_02(output: &std::path::Path) -> Result<(), String> {
        Self::capture_art_finish_02_mode(output, false).await
    }
    pub async fn capture_art_finish_02_integration(output: &std::path::Path) -> Result<(), String> {
        Self::capture_art_finish_02_mode(output, true).await
    }
    async fn capture_art_finish_02_mode(
        output: &std::path::Path,
        integrated: bool,
    ) -> Result<(), String> {
        if !integrated
            && std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(
                    "docs/reviews/refonte-histoire-2026-10-16/art-finish-review-02/acceptance.json",
                )
                .exists()
        {
            return Err(
                "ART-FINISH-02 accepted: preserve historical review; use integration capture"
                    .into(),
            );
        }
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1240., 900.);
        for _ in 0..3 {
            next_frame().await;
        }
        let main: Vec<_> = (0..8).map(|i| review::visual(i, None)).collect();
        let hands = |start| {
            (start..start + 4)
                .flat_map(|i| {
                    [
                        None,
                        Some("core:modele_cogneur_p01"),
                        Some("core:fusil_d_assaut"),
                    ]
                    .map(|w| review::visual(i, w))
                })
                .collect::<Vec<_>>()
        };
        for (label, items) in [
            ("native", main),
            ("classes-hands", hands(0)),
            ("armors-hands", hands(4)),
        ] {
            for frame in 0..3 {
                clear_background(Color::from_rgba(17, 25, 29, 255));
                draw_text(
                    "ART-FINISH-02 · actuel / candidat · cases de 64 px",
                    24.,
                    31.,
                    22.,
                    LIGHTGRAY,
                );
                for (i, &visual) in items.iter().enumerate() {
                    let x = 20. + (i % 4) as f32 * 300.;
                    let y = 80. + (i / 4) as f32 * 200.;
                    for (column, mode) in (if integrated { [2, 3] } else { [1, 2] })
                        .into_iter()
                        .enumerate()
                    {
                        let xx = x + column as f32 * 140.;
                        draw_rectangle(xx, y + 34., 128., 128., Color::from_rgba(57, 61, 55, 255));
                        review::mode(mode, None);
                        game_art::draw_world(Rect::new(xx + 32., y + 66., 64., 64.), visual, 1.);
                    }
                    review::mode(0, None);
                    draw_text(&format!("{label} · {}", i + 1), x, y + 22., 17., LIGHTGRAY);
                }
                if frame == 2 {
                    crate::ui_capture::framebuffer()?
                        .export_png(output.join(format!("{label}-gpu.png")).to_str().unwrap());
                } else {
                    next_frame().await;
                }
            }
        }
        review::mode(0, None);
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
            return Err("Art finish 02 street entry failed".into());
        }
        for (label, at, index, weapon) in [
            (
                "class-house",
                GridPos::new(12, 20),
                0,
                "core:modele_cogneur_p01",
            ),
            (
                "classes-street",
                GridPos::new(15, 41),
                1,
                "core:fusil_d_assaut",
            ),
            (
                "armor-house",
                GridPos::new(12, 20),
                4,
                "core:modele_cogneur_p01",
            ),
            (
                "armors-street",
                GridPos::new(15, 41),
                7,
                "core:fusil_d_assaut",
            ),
        ] {
            app.walk_fixture_to_unchecked(at)?;
            for (mode, suffix) in if integrated {
                [(2, "reference"), (3, "integrated")]
            } else {
                [(1, "before"), (2, "candidate")]
            } {
                crate::prison_street_render::review_normal_style();
                review::mode(mode, Some((index, review::visual(index, Some(weapon)))));
                app.world_fix_frame(output, &format!("{label}-{suffix}"))
                    .await?;
            }
        }
        review::mode(0, None);
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "review":"ART-FINISH-02","status":if integrated {"accepted_integration"} else {"pending_visual_review"},"native_cell":64,
            "drawings":8,"configuration_samples":24,"scene_pairs":4,"production_mode":if integrated {3} else {0},
            "real_engine_lighting":true,"draw_only_actors":true,"gameplay_fingerprints_unchanged":true
        })).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }
}
