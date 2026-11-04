//! Normal renderer and real committed 400ms glides; no visual-review switches.
use crate::{
    asset_preparation,
    combat_visuals::CombatMotion,
    game_art::{
        self, Visual,
        walking::{self, Pose},
    },
    world_readability::player_exposure_review::Scene,
};
use macroquad::prelude::*;
use project_rl::{entity::EntityId, world::GridPos};
use std::path::Path;

fn panel(board: &mut Image, image: &Image, col: usize) {
    for y in 0..160 {
        let src = ((y + 278) * 640 + 208) * 4;
        let dst = (y * 384 + col * 192) * 4;
        board.bytes[dst..dst + 768].copy_from_slice(&image.bytes[src..src + 768]);
    }
}
fn same(a: &Image, b: &Image) -> bool {
    (278..438).all(|y| {
        a.bytes[(y * 640 + 208) * 4..(y * 640 + 400) * 4]
            == b.bytes[(y * 640 + 208) * 4..(y * 640 + 400) * 4]
    })
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let manifest: serde_json::Value = serde_json::from_str(include_str!(
        "../assets/animations/biped-lateral-01/manifest.json"
    ))
    .unwrap();
    let entries = manifest["entries"].as_array().unwrap();
    let visuals: Vec<_> = entries
        .iter()
        .map(|e| Visual::CampaignMonster {
            number: e["number"].as_u64().unwrap() as u16,
            variant: 0,
        })
        .collect();
    let cold = game_art::biped::prepare(&visuals)?;
    game_art::vertical::prepare(&visuals)?;
    let warm = game_art::biped::prepare(&visuals)?;
    if !warm.is_zero() {
        return Err("Warm biped cache unexpectedly waited".into());
    }
    let scene = Scene::new()?;
    let mut records = Vec::new();
    let mut vertical_checks = 0;
    for (e, visual) in entries.iter().zip(visuals) {
        let n = e["number"].as_u64().unwrap() as u16;
        let file = e["file"].as_str().unwrap();
        let authored = asset_preparation::image(file)?;
        let expected = crate::asset_loading::from_image(&authored);
        expected.set_filter(FilterMode::Nearest);
        let width = e["cell"][0].as_f64().unwrap() as f32;
        let offset = e["vertical_offset"].as_f64().unwrap() as f32;
        let before = asset_preparation::stats();
        let mut captures = Vec::new();
        let dir = output.join(format!("MON-{n:03}-0"));
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        for (side, facing) in [-1, 1].into_iter().enumerate() {
            let id: EntityId = serde_json::from_str("1").unwrap();
            let mut motion = CombatMotion::default();
            motion.glide_timed(id, GridPos::new(0, 0), GridPos::new(facing, 0), 10., 400);
            for sample in 0..=32 {
                let now = 10. + sample as f64 * 0.8 / 32.;
                if sample == 16 {
                    motion.glide_timed(
                        id,
                        GridPos::new(facing, 0),
                        GridPos::new(2 * facing, 0),
                        10.4,
                        400,
                    );
                }
                let pose = motion.biped_walk_pose(id, now, false);
                let phase = if sample == 32 {
                    8
                } else {
                    let t = (sample % 16) as f64 / 16. * 650.;
                    let within = [180., 360., 530., 650.]
                        .iter()
                        .position(|end| t < *end)
                        .unwrap_or(3);
                    (sample / 16 * 4 + within) as u8
                };
                if pose.map_or(8, |p| p.frame) != phase {
                    return Err(format!("Phase mismatch {n} {sample}"));
                }
                if motion.biped_walk_pose(id, now, true).is_some() {
                    return Err("Reduced motion must use the retained front".into());
                }
                // Track the real world glide separately; the capture camera follows
                // its actor so every silhouette remains wholly inside its panel.
                let destination = if sample < 16 { facing } else { 2 * facing };
                let actual = motion.actor_rect(
                    id,
                    Rect::new(destination as f32 * 64., 0., 64., 64.),
                    now,
                    false,
                );
                let travel = sample as f32 / 32. * 128. * facing as f32;
                if (actual.x - travel).abs() > 0.001 {
                    return Err("400ms cell interpolation drifted".into());
                }
                let normal = scene
                    .render_actor(|cell| {
                        walking::with_pose(pose, || {
                            game_art::draw_world_animated(cell, visual, 1., None)
                        })
                    })
                    .0;
                let reference = scene
                    .render_actor(|mut cell| {
                        let _actor = crate::world_readability::actor(true);
                        let _player = crate::world_readability::player_scope(false);
                        cell.y += offset;
                        game_art::polish_review_shadow_width(
                            cell,
                            e[if phase == 8 {
                                "front_shadow"
                            } else {
                                "walk_shadow"
                            }]
                            .as_u64()
                            .unwrap() as u8,
                        );
                        crate::world_readability::texture(
                            &expected,
                            cell.center().x - width / 2.,
                            cell.bottom() - 112.,
                            WHITE,
                            DrawTextureParams {
                                source: Some(Rect::new(
                                    f32::from(phase) * width,
                                    if phase == 8 { 0. } else { side as f32 * 128. },
                                    width,
                                    128.,
                                )),
                                dest_size: Some(vec2(width, 128.)),
                                ..Default::default()
                            },
                        );
                    })
                    .0;
                if !same(&normal, &reference) {
                    return Err(format!(
                        "Normal renderer differs from accepted pixels: {n} {side} {sample}"
                    ));
                }
                let mut board = Image::gen_image_color(384, 192, Color::from_rgba(30, 40, 49, 255));
                panel(&mut board, &reference, 0);
                panel(&mut board, &normal, 1);
                let name = format!(
                    "{}-{sample:02}.png",
                    if side == 0 { "left" } else { "right" }
                );
                board.export_png(dir.join(&name).to_str().unwrap());
                captures.push(name);
            }
            next_frame().await;
        }
        for facing in [-2, 2] {
            for frame in 0..6 {
                let pose = Some(Pose { facing, frame });
                let normal = scene
                    .render_actor(|cell| {
                        walking::with_pose(pose, || {
                            game_art::draw_world_animated(cell, visual, 1., None)
                        })
                    })
                    .0;
                let old = scene
                    .render_actor(|cell| {
                        let _actor = crate::world_readability::actor(true);
                        let _player = crate::world_readability::player_scope(false);
                        assert!(game_art::vertical::draw(cell, visual, 1., pose));
                    })
                    .0;
                if !same(&normal, &old) {
                    return Err(format!("Vertical rendering changed for {n}"));
                }
                vertical_checks += 1;
            }
        }
        let after = asset_preparation::stats();
        for metric in ["source_reads", "uploads", "jobs"] {
            if before[metric] != after[metric] {
                return Err(format!("Resource work in draw: {n} {metric}"));
            }
        }
        records.push(serde_json::json!({"number":n,"captures":captures,"accepted_pixels_match":true,"real_cell_duration_ms":400,"draw_resource_work":false}));
    }
    let report = serde_json::json!({"review":"BIPED-INTEGRATION01","normal_game_enabled":true,"models":records,"native_cell_px":64,"step_ms":400,"cycle_ms":800,"captures":660,"vertical_frames_identical":vertical_checks,"cold_prepare_ms":cold.as_secs_f64()*1000.,"warm_prepare_ms":0,"resources":asset_preparation::stats(),"camera":"actor centered for silhouette comparison; actual 64px cell interpolation asserted"});
    std::fs::write(
        output.join("checks.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
