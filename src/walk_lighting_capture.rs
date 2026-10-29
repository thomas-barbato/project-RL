//! Fixed-position exposure audit: real actor draw paths, no campaign loaded.
use crate::{
    game_art::{
        self, Visual,
        walking::{self, Pose},
    },
    world_readability,
};
use macroquad::prelude::*;
use std::path::Path;

fn cases() -> Vec<(String, Visual)> {
    let mut rows = Vec::new();
    for index in 1..=4 {
        rows.push((format!("CAMP-{index}"), Visual::CampNpc(index)));
    }
    for index in 20..=25 {
        rows.push((format!("SERVICE-{index}"), Visual::Actor(index)));
    }
    for class in ["bastion", "traqueur", "arcaniste", "metamorphe"] {
        rows.push((
            class.into(),
            game_art::player(
                Some(&format!("core:{class}")),
                crate::terminal_view::PlayerWeaponPose::Melee,
                Some("core:melee_hachette"),
            ),
        ));
    }
    for number in [14, 120, 22, 24] {
        rows.push((
            format!("MON-{number}"),
            Visual::CampaignMonster { number, variant: 0 },
        ));
    }
    rows
}

fn phases() -> Vec<(String, Option<Pose>, Option<f64>)> {
    let mut phases = vec![
        ("repos".into(), None, Some(0.)),
        ("sans-horloge".into(), None, None),
        ("respiration".into(), None, Some(1.6)),
    ];
    for facing in [-1, 1] {
        for frame in 0..6 {
            phases.push((
                format!("{facing}-{frame}"),
                Some(Pose { facing, frame }),
                None,
            ));
        }
    }
    phases.push(("retour-face".into(), None, Some(0.)));
    phases
}

pub async fn capture(output: &Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(1280., 800.);
    for _ in 0..3 {
        next_frame().await;
    }
    let rt = render_target(256, 256);
    rt.texture.set_filter(FilterMode::Nearest);
    let light = Texture2D::from_image(&Image::gen_image_color(1, 1, Color::new(0.3, 0.3, 0.3, 1.)));
    let mut rows = Vec::new();
    let mut failures = 0;
    for lit in [false, true] {
        if lit {
            world_readability::begin_lighting(&light, vec2(256., 256.), [0.6; 3], true);
        } else {
            world_readability::end_lighting();
        }
        for (label, visual) in cases() {
            let mut reference = None;
            let mut sheet = Image::gen_image_color(128 * 6, 160, Color::from_rgba(28, 35, 43, 255));
            for (index, (phase, pose, time)) in phases().into_iter().enumerate() {
                let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., 256., 256.));
                camera.render_target = Some(rt.clone());
                set_camera(&camera);
                clear_background(Color::from_rgba(28, 35, 43, 255));
                let draws = world_readability::audit_textures(|| {
                    let _world = world_readability::world(false);
                    walking::with_pose(pose, || {
                        game_art::draw_world_animated(
                            Rect::new(96., 128., 64., 64.),
                            visual,
                            1.,
                            time,
                        )
                    });
                });
                unsafe {
                    get_internal_gl().flush();
                }
                let image = rt.texture.get_texture_data();
                if index == 0 {
                    reference = Some(image.bytes.clone());
                }
                let difference = reference
                    .as_ref()
                    .unwrap()
                    .chunks_exact(4)
                    .zip(image.bytes.chunks_exact(4))
                    .filter(|(a, b)| a != b)
                    .count();
                let pipeline_ok = !draws.is_empty()
                    && draws.iter().all(|d| {
                        d.world && d.actor && d.finish && d.gain == if lit { 1.48 } else { 1. }
                    });
                let same_clock = ![1, 15].contains(&index) || difference == 0;
                let breathing_pixels = if index == 2 && matches!(visual, Visual::CampNpc(_)) {
                    let a = reference.as_ref().unwrap();
                    let b = &image.bytes;
                    let mut changed = 0;
                    // Register the existing 2 px upper-body translation and
                    // exclude the waist overlap; compare the same source ink.
                    for source_y in 0..76usize {
                        for x in 64..192usize {
                            let ia = ((255 - (96 + source_y)) * 256 + x) * 4;
                            let ib = ((255 - (98 + source_y)) * 256 + x) * 4;
                            if a[ia..ia + 4] != b[ib..ib + 4] {
                                changed += 1;
                            }
                        }
                    }
                    for source_y in 80..128usize {
                        for x in 64..192usize {
                            let i = ((255 - (96 + source_y)) * 256 + x) * 4;
                            if a[i..i + 4] != b[i..i + 4] {
                                changed += 1;
                            }
                        }
                    }
                    Some(changed)
                } else {
                    None
                };
                if !pipeline_ok || !same_clock {
                    failures += 1;
                }
                if breathing_pixels.is_some_and(|n| n != 0) {
                    failures += 1;
                }
                if let Some(column) = match index {
                    0 => Some(0),
                    1 => Some(1),
                    2 => Some(2),
                    5 => Some(3),
                    11 => Some(4),
                    15 => Some(5),
                    _ => None,
                } {
                    // Native 128 px crop around the 64 px cell; no resampling.
                    for y in 0..128usize {
                        let src = ((64 + y) * 256 + 64) * 4;
                        let dest = ((y + 24) * 768 + column * 128) * 4;
                        sheet.bytes[dest..dest + 512].copy_from_slice(&image.bytes[src..src + 512]);
                    }
                }
                rows.push(serde_json::json!({"actor":label,"lit":lit,"phase":phase,"same_shader":pipeline_ok,"same_image_without_idle_clock":same_clock,"different_pixels_from_rest":difference,"breathing_registered_pixel_differences":breathing_pixels,"draws":draws}));
            }
            sheet.export_png(
                output
                    .join(format!("{label}-{}.png", if lit { "lit" } else { "unlit" }))
                    .to_str()
                    .ok_or("capture path")?,
            );
            next_frame().await;
        }
    }
    world_readability::end_lighting();
    let fixed = failures == 0;
    std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":"POLISH01-WALK-LIGHT01","status":if fixed{"passed"}else{"mismatch_reproduced"},"cases":rows,"failures":failures,"native_cell":64,"positions_fixed":true,"source_art_changed":false,"saved_campaign_access":false,"sheet_columns":["repos","sans horloge","respiration","marche gauche","marche droite","retour face"]})).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    println!("WALK-LIGHT01: {failures} inconsistent lighting/image routes");
    if !fixed && std::env::args().any(|a| a == "--expect-fixed") {
        return Err(format!("{failures} lighting/image routes still differ"));
    }
    Ok(())
}
