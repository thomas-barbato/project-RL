//! DUAL15 native engine comparison; no campaign or preferences are written.
use crate::{
    game_art::{
        self, Visual,
        walking::{self, Pose},
    },
    world_readability,
};
use macroquad::prelude::*;
use project_rl::entity::EquipmentPalette;
use std::path::Path;

pub async fn capture(output: &Path) -> Result<(), String> {
    let integrated = std::env::args().nth(1).as_deref() == Some("--ui-cold-dual-walk-integration");
    let normal_enabled = walking::dual15_enabled();
    if integrated && !normal_enabled {
        return Err("DUAL15 is not active in the normal renderer".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(1280., 800.);
    for _ in 0..3 {
        next_frame().await;
    }
    let rt = render_target(128, 128);
    rt.texture.set_filter(FilterMode::Nearest);
    let light = Texture2D::from_image(&Image::gen_image_color(1, 1, Color::new(0.3, 0.3, 0.3, 1.)));
    let mut records = Vec::new();
    for lit in [false, true] {
        if lit {
            world_readability::begin_lighting(&light, vec2(128., 128.), [0.6; 3], true);
        } else {
            world_readability::end_lighting();
        }
        for index in [0, 4, 10, 14, 15, 20] {
            for (label, hands) in [
                ("pistols", [Some(101), Some(101)]),
                ("blades", [Some(68), Some(79)]),
                ("mixed", [Some(68), Some(101)]),
                ("mixed-reversed", [Some(101), Some(68)]),
            ] {
                let visual = Visual::Player {
                    body: 3 + index / 5,
                    equipment: [
                        if index % 5 == 4 { Some(103) } else { None },
                        None,
                        None,
                        None,
                    ],
                    weapon: None,
                    hands: Some(hands),
                    palettes: [EquipmentPalette::Original; 3],
                };
                let mut variants = Vec::new();
                let mut current = Vec::new();
                for proposed in [false, true, false] {
                    let _review =
                        (!integrated || proposed).then(|| walking::review_dual15(proposed));
                    let mut atlas = Image::gen_image_color(13 * 128, 128, BLANK);
                    for n in 0..13usize {
                        let pose = if n == 0 {
                            None
                        } else {
                            Some(Pose {
                                facing: if n <= 6 { -1 } else { 1 },
                                frame: ((n - 1) % 6) as u8,
                            })
                        };
                        let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., 128., 128.));
                        camera.render_target = Some(rt.clone());
                        set_camera(&camera);
                        clear_background(Color::from_rgba(30, 40, 49, 255));
                        let draws = world_readability::audit_textures(|| {
                            let _world = world_readability::world(false);
                            walking::with_pose(pose, || {
                                game_art::draw_world_animated(
                                    Rect::new(32., 32., 64., 64.),
                                    visual,
                                    1.,
                                    Some(0.),
                                )
                            })
                        });
                        if draws.is_empty()
                            || draws.iter().any(|d| {
                                !d.actor
                                    || !d.finish
                                    || !d.world
                                    || d.gain != if lit { 1.48 } else { 1. }
                            })
                        {
                            return Err("DUAL15 exposure pipeline mismatch".into());
                        }
                        unsafe {
                            get_internal_gl().flush();
                        }
                        let image = rt.texture.get_texture_data();
                        if variants.is_empty() {
                            current.push(image.bytes.clone());
                        } else if !proposed && image.bytes != current[n] {
                            return Err(format!("DUAL15 cache/guard leaked: {index} {label} {n}"));
                        } else if proposed && n == 0 && image.bytes != current[n] {
                            return Err("DUAL15 front idle changed".into());
                        }
                        if integrated && proposed && image.bytes != current[n] {
                            return Err(format!(
                                "Normal DUAL15 differs from the accepted recipe: {index} {label} {n}"
                            ));
                        }
                        for y in 0..128usize {
                            let at = (y * 1664 + n * 128) * 4;
                            atlas.bytes[at..at + 512]
                                .copy_from_slice(&image.bytes[y * 512..(y + 1) * 512]);
                        }
                    }
                    if !proposed && !variants.is_empty() {
                        break;
                    }
                    let file = format!(
                        "CLASS-{index:02}-{label}-{}-{}.png",
                        if lit { "lit" } else { "plain" },
                        if proposed { "candidate" } else { "current" }
                    );
                    atlas.export_png(output.join(&file).to_str().ok_or("DUAL15 path")?);
                    variants.push(serde_json::json!({"proposed":proposed,"file":file}));
                }
                records.push(serde_json::json!({"index":index,"loadout":label,"hands":hands,"lighting":lit,"variants":variants}));
            }
        }
    }
    world_readability::end_lighting();
    set_default_camera();
    if walking::dual15_enabled() != normal_enabled {
        return Err("DUAL15 review guard leaked".into());
    }
    std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":"DUAL15-r01","status":if integrated {"integrated"} else {"historical_comparison"},"normal_game_activation":normal_enabled,"normal_matches_accepted_recipe":integrated,"native_tile_px":64,"exported_pngs_need_vertical_flip":false,"front_idle_unchanged":true,"restored_normal_pixels_after_every_comparison":true,"campaign_loaded":false,"preferences_written":false,"records":records})).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    Ok(())
}
