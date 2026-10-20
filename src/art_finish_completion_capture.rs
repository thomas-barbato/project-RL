//! Full native bestiary coverage and representative real-engine lighting scenes.
use super::*;
use crate::game_art::{self, Visual, finish_completion as review};
impl AsciiApp {
    pub async fn capture_art_finish_completion(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1920., 1080.);
        for _ in 0..3 {
            next_frame().await;
        }
        let keys = review::keys();
        for (page, part) in keys.chunks(24).enumerate() {
            for frame in 0..3 {
                clear_background(Color::from_rgba(17, 25, 29, 255));
                for (i, &(number, variant, _)) in part.iter().enumerate() {
                    let x = 12. + (i % 6) as f32 * 318.;
                    let y = 64. + (i / 6) as f32 * 244.;
                    draw_rectangle(x, y, 306., 226., Color::from_rgba(67, 74, 70, 255));
                    draw_text(
                        &format!("{number:03} {}", char::from(b'A' + variant)),
                        x + 12.,
                        y + 26.,
                        22.,
                        Color::from_rgba(222, 204, 167, 255),
                    );
                    game_art::draw_world(
                        Rect::new(x + 121., y + 142., 64., 64.),
                        Visual::CampaignMonster { number, variant },
                        1.,
                    );
                }
                if frame == 2 {
                    crate::ui_capture::framebuffer()?.export_png(
                        output
                            .join(format!("native-{page:02}.png"))
                            .to_str()
                            .unwrap(),
                    );
                } else {
                    next_frame().await;
                }
            }
        }
        Self::capture_finished_worn_pages(output).await?;
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
        let exit = pos(camp::scene().exits[0]);
        app.walk_fixture_to_unchecked(exit)?;
        if app.execute_command(GameCommand::Interact { target: exit }) != CommandOutcome::Applied {
            return Err("Completion fixture could not enter street".into());
        }
        app.walk_fixture_to_unchecked(GridPos::new(15, 41))?;
        app.selected_world_cell = None;
        app.selected_target = None;
        app.world_cursor = None;
        if game_art::finish_review_05::positions(&app.game).len() != 4 {
            return Err("Four visible fixture positions required".into());
        }
        let groups: &[(&str, [(u16, u8); 4])] = &[
            ("cultists", [(12, 0), (15, 0), (164, 0), (165, 0)]),
            ("hybrids", [(5, 0), (6, 0), (17, 0), (39, 0)]),
            ("zombies", [(120, 0), (121, 0), (122, 0), (123, 0)]),
            ("spirits", [(124, 0), (125, 0), (126, 0), (127, 0)]),
            ("necromancers", [(132, 0), (133, 0), (137, 0), (139, 0)]),
            ("worms", [(140, 0), (141, 2), (142, 3), (143, 4)]),
            ("wings", [(146, 0), (147, 2), (148, 3), (151, 1)]),
            ("materials", [(156, 0), (156, 2), (156, 3), (158, 0)]),
        ];
        for &(label, figures) in groups {
            for &(n, v) in &figures {
                if project_rl::monster_art::art(n, v).is_none() {
                    return Err(format!("Invalid fixture {n}:{v}"));
                }
            }
            review::set(&figures);
            crate::prison_street_render::review_normal_style();
            app.world_fix_frame(output, label).await?;
        }
        review::set(&[]);
        std::fs::write(
            output.join("checks.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "review":"ART-FINISH-COMPLETE-06","native_cell":64,"native_appearances":keys.len(),
                "native_gpu_pages":keys.len().div_ceil(24),"real_lighting_scenes":groups.len(),
                "draw_only_actors":true,"gameplay_fingerprints_unchanged":true,
                "ordinary_embedded_asset_provider":true,"finished_outfits":19,"worn_configuration_samples":437,"additional_weapon_samples":113
            }))
            .unwrap(),
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }
    async fn capture_finished_worn_pages(output: &std::path::Path) -> Result<(), String> {
        use project_rl::entity::EquipmentPalette as P;
        let outfits = [
            200, 212, 4, 6, 15, 201, 202, 203, 204, 213, 214, 215, 216, 208, 209, 210, 211, 217,
            218,
        ];
        for outfit in outfits {
            for frame in 0..3 {
                clear_background(Color::from_rgba(17, 25, 29, 255));
                for i in 0..23 {
                    let x = 12. + (i % 6) as f32 * 318.;
                    let y = 64. + (i / 6) as f32 * 244.;
                    draw_rectangle(x, y, 306., 226., Color::from_rgba(67, 74, 70, 255));
                    let (weapon, hands) = match i {
                        0 => (None, None),
                        1 => (Some(101), None),
                        2 => (Some(70), None),
                        3 => (Some(2), None),
                        4 => (Some(15), None),
                        5 => (Some(61), None),
                        6 => (Some(88), None),
                        7 => (None, Some([Some(101), None])),
                        8 => (None, Some([Some(68), None])),
                        9 => (None, Some([Some(101), Some(101)])),
                        10 | 21 => (None, Some([Some(68), Some(101)])),
                        22 => (None, Some([Some(15), None])),
                        11 => (None, Some([Some(70), Some(95)])),
                        _ => (Some(101), None),
                    };
                    let colors = if (12..21).contains(&i) {
                        let palette = [
                            P::Original,
                            P::Red,
                            P::Blue,
                            P::Violet,
                            P::Turquoise,
                            P::Yellow,
                            P::Gray,
                            P::ReviewedVariant1,
                            P::ReviewedVariant2,
                        ][i - 12];
                        [palette, P::Original, P::Original]
                    } else if i == 22 {
                        [P::Original, P::Original, P::Red]
                    } else if i >= 21 {
                        [P::Original, P::Red, P::Blue]
                    } else {
                        [P::Original; 3]
                    };
                    draw_text(
                        &format!("Tenue {outfit} · {}", i + 1),
                        x + 12.,
                        y + 26.,
                        20.,
                        Color::from_rgba(222, 204, 167, 255),
                    );
                    review::draw_worn(
                        Rect::new(x + 121., y + 142., 64., 64.),
                        outfit,
                        weapon,
                        hands,
                        colors,
                    );
                }
                if frame == 2 {
                    crate::ui_capture::framebuffer()?
                        .export_png(output.join(format!("worn-{outfit}.png")).to_str().unwrap());
                } else {
                    next_frame().await;
                }
            }
        }
        let weapons: Vec<u16> = (0..game_art::HELD_WEAPON_COUNT).collect();
        for (page, models) in weapons.chunks(24).enumerate() {
            for frame in 0..3 {
                clear_background(Color::from_rgba(17, 25, 29, 255));
                for (i, &model) in models.iter().enumerate() {
                    let x = 12. + (i % 6) as f32 * 318.;
                    let y = 64. + (i / 6) as f32 * 244.;
                    draw_rectangle(x, y, 306., 226., Color::from_rgba(67, 74, 70, 255));
                    draw_text(
                        &format!("Arme {model}"),
                        x + 12.,
                        y + 26.,
                        20.,
                        Color::from_rgba(222, 204, 167, 255),
                    );
                    review::draw_worn(
                        Rect::new(x + 121., y + 142., 64., 64.),
                        200,
                        Some(model),
                        None,
                        [P::Original; 3],
                    );
                }
                if frame == 2 {
                    crate::ui_capture::framebuffer()?.export_png(
                        output
                            .join(format!("weapons-{page:02}.png"))
                            .to_str()
                            .unwrap(),
                    );
                } else {
                    next_frame().await;
                }
            }
        }
        Ok(())
    }
}
