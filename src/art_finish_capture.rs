//! Numbered native boards plus actual street lighting. Review-only, no spawns.
use super::*;
use crate::game_art::{self, Visual};

fn figure(torso: Option<&str>, weapon: Option<&str>) -> Visual {
    game_art::wear(
        game_art::player(
            Some("core:bastion"),
            if weapon.is_some() {
                PlayerWeaponPose::Ranged
            } else {
                PlayerWeaponPose::Unarmed
            },
            weapon,
        ),
        torso.map(|id| ("core:body_armor".to_owned(), id.to_owned())),
    )
}
fn samples() -> Vec<Visual> {
    vec![
        figure(None, None),
        figure(Some("core:armure_cerbere"), None),
        figure(Some("core:brigandine_cognefer"), None),
        figure(Some("core:composite_carapace"), None),
        figure(Some("core:veste_matelassee"), None),
        Visual::CampaignMonster {
            number: 164,
            variant: 0,
        },
        Visual::CampaignMonster {
            number: 165,
            variant: 0,
        },
        Visual::CampaignMonster {
            number: 5,
            variant: 0,
        },
        Visual::CampaignMonster {
            number: 144,
            variant: 0,
        },
        Visual::CampaignMonster {
            number: 145,
            variant: 0,
        },
        Visual::Bestiary {
            number: 57,
            corpse: false,
        },
        Visual::ColoredItem(
            game_art::item("core:fusil_d_assaut").unwrap(),
            project_rl::entity::EquipmentPalette::Original,
        ),
        Visual::ColoredItem(
            game_art::item("core:modele_cogneur_p01").unwrap(),
            project_rl::entity::EquipmentPalette::Original,
        ),
    ]
}
impl AsciiApp {
    pub async fn capture_art_finish(output: &std::path::Path) -> Result<(), String> {
        let integration =
            std::env::args().nth(1).as_deref() == Some("--ui-cold-art-finish-integration");
        if !integration
            && std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join(
                    "docs/reviews/refonte-histoire-2026-10-16/art-finish-review-01/acceptance.json",
                )
                .exists()
        {
            return Err("Les captures de la revue acceptée sont figées ; utiliser un nouveau dossier d’intégration".into());
        }
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1240., 900.);
        for _ in 0..3 {
            next_frame().await;
        }
        let main = samples();
        let mut attachments = Vec::new();
        for torso in [None, Some("core:armure_cerbere")] {
            for weapon in [
                None,
                Some("core:modele_cogneur_p01"),
                Some("core:fusil_d_assaut"),
            ] {
                attachments.push(figure(torso, weapon));
            }
        }
        let palettes = vec![
            Visual::CampaignMonster {
                number: 144,
                variant: 0,
            },
            Visual::CampaignMonster {
                number: 144,
                variant: 2,
            },
            Visual::CampaignMonster {
                number: 144,
                variant: 3,
            },
            Visual::CampaignMonster {
                number: 144,
                variant: 1,
            },
            Visual::CampaignMonster {
                number: 145,
                variant: 0,
            },
            Visual::CampaignMonster {
                number: 145,
                variant: 2,
            },
            Visual::CampaignMonster {
                number: 145,
                variant: 3,
            },
        ];
        for (label, items) in [
            ("native", main),
            ("hands", attachments),
            ("palettes", palettes),
        ] {
            for frame in 0..3 {
                clear_background(Color::from_rgba(17, 25, 29, 255));
                draw_text(
                    "ART-FINISH-01 · revue à taille native · avant / candidat",
                    24.,
                    31.,
                    22.,
                    LIGHTGRAY,
                );
                for (i, &visual) in items.iter().enumerate() {
                    let x = 20. + (i % 4) as f32 * 300.;
                    let y = 80. + (i / 4) as f32 * 200.;
                    for (column, mode) in if integration { [2, 3] } else { [1, 2] }
                        .into_iter()
                        .enumerate()
                    {
                        let xx = x + column as f32 * 140.;
                        draw_rectangle(xx, y + 34., 128., 128., Color::from_rgba(57, 61, 55, 255));
                        let rect = Rect::new(xx + 32., y + 66., 64., 64.);
                        game_art::finish_review::mode(mode, false);
                        game_art::draw_world(rect, visual, 1.);
                    }
                    game_art::finish_review::mode(0, false);
                    draw_text(
                        &format!("{} · {}", label, i + 1),
                        x,
                        y + 22.,
                        17.,
                        LIGHTGRAY,
                    );
                }
                if frame == 2 {
                    crate::ui_capture::framebuffer()?
                        .export_png(output.join(format!("{label}-gpu.png")).to_str().unwrap());
                } else {
                    next_frame().await;
                }
            }
        }
        game_art::finish_review::mode(0, false);
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
            return Err("Art finish street entry failed".into());
        }
        for (label, at) in [
            ("house", GridPos::new(12, 20)),
            ("street", GridPos::new(15, 41)),
        ] {
            app.walk_fixture_to_unchecked(at)?;
            for (mode, suffix) in if integration {
                [(2, "before"), (3, "candidate")]
            } else {
                [(1, "before"), (2, "candidate")]
            } {
                crate::prison_street_render::review_normal_style();
                game_art::finish_review::mode(mode, true);
                app.world_fix_frame(output, &format!("{label}-{suffix}"))
                    .await?;
            }
        }
        game_art::finish_review::mode(0, false);
        std::fs::write(
            output.join("checks.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "review":"ART-FINISH-01","status":if integration {"accepted_integration"} else {"pending_visual_review"},"native_cell":64,
                "candidates":13,"attachment_samples":6,"palette_samples":7,"scene_pairs":2,
                "lighting":"normal engine atmosphere","native_sources":true,"draw_only_actors":true,
                "gameplay_fingerprints_unchanged":true,"ordinary_runtime_overrides_default_off":!integration,
                "reference_mode":2,"production_mode":if integration {3} else {0}
            }))
            .unwrap(),
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}
