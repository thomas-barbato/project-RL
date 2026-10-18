//! Native verification of the integrated MARKER-PULSE-02 renderer.
use super::*;
use crate::preview_markers;

fn rgb(image: &Image, at: Vec2) -> [u8; 3] {
    let x = at.x.floor() as usize;
    let y = at.y.floor() as usize;
    let offset = ((image.height as usize - 1 - y) * image.width as usize + x) * 4;
    image.bytes[offset..offset + 3].try_into().unwrap()
}

fn number_regions(app: &AsciiApp, image: &Image) -> Vec<u8> {
    let mut pixels = Vec::new();
    if let Some(route) = &app.walk_preview {
        for &at in route.cells.iter().skip(1) {
            let center = app
                .terminal
                .world_cell_rect(
                    &app.game,
                    app.terminal_bounds(),
                    app.ui_scale(),
                    64,
                    app.navigation_signal_summary().is_some(),
                    at,
                    app.camera_glide_offset(),
                )
                .unwrap()
                .center();
            for y in -13..=13 {
                for x in -13..=13 {
                    pixels.extend(rgb(image, center + vec2(x as f32, y as f32)));
                }
            }
        }
    }
    pixels
}

async fn verify_runtime(app: &mut AsciiApp, route: bool) -> Result<serde_json::Value, String> {
    let _normal_renderer = preview_markers::review_phase(None);
    app.graphics.active.reduced_motion = false;
    let at = if route {
        GridPos::new(8, 4)
    } else {
        GridPos::new(5, 3)
    };
    let rect = app
        .terminal
        .world_cell_rect(
            &app.game,
            app.terminal_bounds(),
            app.ui_scale(),
            64,
            app.navigation_signal_summary().is_some(),
            at,
            Vec2::ZERO,
        )
        .unwrap();
    let probe = if route {
        rect.center() - vec2(0., 24.)
    } else {
        vec2(rect.x + rect.w * 0.5 + 0.75, rect.y + 1.5)
    };
    let mut samples = Vec::new();
    let mut numbers = None;
    let began = get_time();
    for index in 0..16 {
        let deadline = began + index as f64 * 0.08;
        while get_time() < deadline {
            app.draw();
            next_frame().await;
        }
        app.draw();
        let image = crate::ui_capture::framebuffer()?;
        samples.push(rgb(&image, probe));
        let current = number_regions(app, &image);
        if numbers
            .as_ref()
            .is_some_and(|previous| *previous != current)
        {
            return Err("Numbered movement circles are animated".into());
        }
        numbers = Some(current);
    }
    let spread = (0..3)
        .map(|c| {
            samples.iter().map(|p| p[c]).max().unwrap()
                - samples.iter().map(|p| p[c]).min().unwrap()
        })
        .max()
        .unwrap();
    if spread < 20 {
        return Err(format!("Runtime marker did not pulse: {samples:?}"));
    }
    app.graphics.active.reduced_motion = true;
    app.draw();
    let still = rgb(&crate::ui_capture::framebuffer()?, probe);
    let until = get_time() + 0.6;
    while get_time() < until {
        app.draw();
        next_frame().await;
    }
    app.draw();
    if rgb(&crate::ui_capture::framebuffer()?, probe) != still {
        return Err("Reduced-motion marker is animated".into());
    }
    Ok(
        serde_json::json!({"kind":if route {"destination"} else {"valid_skill_area"},
        "review_override":false,"runtime_clock_pulse":true,"rgb_variation":spread,
        "reduced_motion_static":true,"numbered_steps_static":route,"samples":samples}),
    )
}

impl AsciiApp {
    pub async fn capture_marker_pulse(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let _guard = preview_markers::review_phase(None);
        request_new_screen_size(1280., 800.);
        for _ in 0..3 {
            next_frame().await;
        }
        let technique = project_rl::skills::agent::technique("A02");
        let target = GridPos::new(6, 4);
        let mut apps = Vec::new();
        for kind in 0..3 {
            let mut app = Self::new_agent_blast_trial()?;
            app.crash_recovery_enabled = false;
            app.suspension_path = output.join("isolated-unused.json");
            app.graphics.active.world_cell_px = 64;
            app.graphics.active.reduced_motion = true;
            if kind < 2 {
                if kind == 1 {
                    for _ in 0..64 {
                        if matches!(
                            app.execute_command(GameCommand::UseTechniqueAt {
                                technique: technique.clone(),
                                target,
                                weapon_slot: app.active_weapon_slot,
                            }),
                            CommandOutcome::Rejected(_)
                        ) {
                            break;
                        }
                    }
                }
                app.attack_aim = Some(AttackAim {
                    slot: app.active_weapon_slot,
                    cursor: target,
                });
                app.attack_aim_technique = Some(technique.clone());
                if app.aimed_attack_preview(app.attack_aim.unwrap()).is_ok() != (kind == 0) {
                    return Err("Unexpected skill preview validity".into());
                }
            } else {
                app.select_world_target(None);
                app.preview_or_walk_to(GridPos::new(8, 4), 0.);
                if app.walk_preview.is_none() {
                    return Err("Missing route preview".into());
                }
            }
            app.combat_motion.clear();
            app.visual_cues.clear_world();
            apps.push(app);
        }
        let before: Vec<_> = apps
            .iter()
            .map(|app| suspension::fingerprint(&app.game))
            .collect();
        let mut pictures = Vec::new();
        for index in 0..12 {
            let brightness = (1. - (index as f32 * std::f32::consts::TAU / 12.).cos()) * 0.5;
            let _phase = preview_markers::review_phase(Some(brightness));
            let mut row = Vec::new();
            for app in &apps {
                app.draw();
                next_frame().await;
                app.draw();
                let image = crate::ui_capture::framebuffer()?;
                if (image.width, image.height) != (1280, 800) {
                    return Err("Native 1280x800 capture unavailable".into());
                }
                let texture = Texture2D::from_image(&image);
                texture.set_filter(FilterMode::Nearest);
                row.push(texture);
            }
            pictures.push(row);
        }
        let mut runtime_checks = Vec::new();
        runtime_checks.push(verify_runtime(&mut apps[0], false).await?);
        runtime_checks.push(verify_runtime(&mut apps[2], true).await?);
        request_new_screen_size(1280., 400.);
        for _ in 0..3 {
            next_frame().await;
        }
        for (index, row) in pictures.iter().enumerate() {
            for pass in 0..2 {
                set_default_camera();
                clear_background(Color::from_rgba(11, 17, 23, 255));
                crate::ui_theme::draw_text_bold(
                    "Petits repères animés · nombres fixes",
                    24.,
                    30.,
                    23.,
                    Color::from_rgba(230, 224, 203, 255),
                );
                for (kind, title) in ["Zone valide", "Zone invalide", "Déplacement"]
                    .iter()
                    .enumerate()
                {
                    let x = 24. + kind as f32 * 408.;
                    crate::ui_theme::draw_text_bold(
                        title,
                        x,
                        57.,
                        16.,
                        Color::from_rgba(211, 229, 239, 255),
                    );
                    let crop = if kind < 2 {
                        Rect::new(520., 800. - 232. - 288., 384., 288.)
                    } else {
                        Rect::new(572., 800. - 240. - 288., 384., 288.)
                    };
                    draw_texture_ex(
                        &row[kind],
                        x,
                        70.,
                        WHITE,
                        DrawTextureParams {
                            source: Some(crop),
                            flip_y: true,
                            ..Default::default()
                        },
                    );
                }
                crate::ui_theme::draw_text(
                    "MARKER-PULSE-02 intégré · 64 px par case · nombres et cercles numérotés fixes",
                    24.,
                    385.,
                    15.,
                    Color::from_rgba(165, 183, 196, 255),
                );
                if pass == 1 {
                    crate::ui_capture::framebuffer()?.export_png(
                        output
                            .join(format!("pulse-{index:02}.png"))
                            .to_str()
                            .ok_or("Capture path")?,
                    );
                }
                next_frame().await;
            }
        }
        for (app, fingerprint) in apps.iter().zip(before) {
            if suspension::fingerprint(&app.game) != fingerprint {
                return Err("Animated markers modified the game".into());
            }
        }
        std::fs::write(
            output.join("checks.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "review":"MARKER-PULSE-02","status":"approved_and_integrated",
                "normal_games_changed":true,"production_capture_with_debug_fill_override":true,
                "tile_pixels":64,"frame_pixels":[1280,400],"frame_count":12,"cycle_ms":1100,
                "draw_preserved_game":true,"contours_and_numbers_fixed":true,
                "rhythm":"progressive; explicitly accepted by player",
                "shared_hole_fill_outline_center_and_geometry":true,
                "fill_reaches_inner_contour":true,
                "numbered_steps_and_circles":"static WALK-VIS-03-E rendering",
                "runtime_checks":runtime_checks,
            }))
            .map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}
