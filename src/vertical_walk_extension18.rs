//! Explicit C06 review fixture. Never selected by the ordinary renderer.
use crate::{combat_visuals::CombatMotion, world_readability};
use macroquad::prelude::*;
use project_rl::{asset_cache::ByteCache, world::GridPos};
use serde::Deserialize;
use std::{path::Path, time::Instant};

#[derive(Deserialize)]
struct Case {
    id: String,
    file: String,
    kind: String,
    side: u16,
    foot: [f32; 2],
    #[serde(default)]
    save: bool,
    #[serde(default)]
    camp_group: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Catalogue {
    Named { review: String, cases: Vec<Case> },
    Legacy(Vec<Case>),
}

fn cases() -> Result<(String, Vec<Case>), String> {
    let path = std::env::var("RL_VERTICAL_REVIEW_CASES").unwrap_or_else(|_| {
        "assets/source-art/vertical-walk-review-18/extension-C06/engine-cases.json".into()
    });
    if !path.starts_with("assets/source-art/vertical-walk-review-18/")
        || path.contains("..")
        || path.contains('\\')
    {
        return Err("Vertical review catalogue outside its source bank".into());
    }
    let fallback = if path.contains("/armor-C10/") {
        "VERT18-C10"
    } else {
        "VERT18-C06"
    };
    let value = std::fs::read(&path).map_err(|e| e.to_string())?;
    let catalogue: Catalogue = serde_json::from_slice(&value).map_err(|e| e.to_string())?;
    let (review, cases) = match catalogue {
        Catalogue::Named { review, cases } => (review, cases),
        Catalogue::Legacy(cases) => (fallback.to_owned(), cases),
    };
    if !review.starts_with("VERT18-")
        || review.len() > 48
        || !review
            .bytes()
            .all(|c| c.is_ascii_alphanumeric() || c == b'-')
    {
        return Err("Invalid vertical review identifier".into());
    }
    Ok((review, cases))
}

pub async fn capture(output: &Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(960., 760.);
    for _ in 0..3 {
        next_frame().await;
    }
    let (review, mut cases) = cases()?;
    if let Ok(prefix) = std::env::var("RL_VERTICAL_REVIEW_PREFIX") {
        cases.retain(|case| case.id.starts_with(&prefix));
        if cases.is_empty() {
            return Err("C06 case filter matched nothing".into());
        }
    }
    let mut textures = ByteCache::new(32 * 1024 * 1024);
    let mut rows = Vec::new();
    let mut front_returns = 0;
    let mut saved = 0;
    for (index, case) in cases.iter().enumerate() {
        if !case.file.starts_with("assets/") || case.file.contains("..") {
            return Err(format!("C06 source outside assets: {}", case.id));
        }
        // Read and decode on a preparation worker, then upload on the GL
        // thread before drawing. No capture directory is a source dependency.
        let file = case.file.clone();
        let prepared = std::thread::spawn(move || -> Result<_, String> {
            let start = Instant::now();
            let bytes = std::fs::read(file).map_err(|e| e.to_string())?;
            let read_ms = start.elapsed().as_secs_f64() * 1000.;
            let start = Instant::now();
            let image = Image::from_file_with_format(&bytes, Some(ImageFormat::Png))
                .map_err(|e| e.to_string())?;
            Ok((image, read_ms, start.elapsed().as_secs_f64() * 1000.))
        });
        while !prepared.is_finished() {
            next_frame().await;
        }
        let (image, read_ms, decode_ms) = prepared.join().map_err(|_| "C06 worker panic")??;
        if (image.width, image.height) != (case.side * 6, case.side * 3) {
            return Err(format!("C06 atlas dimensions: {}", case.id));
        }
        let start = Instant::now();
        let texture = crate::asset_loading::from_image(&image);
        texture.set_filter(FilterMode::Nearest);
        let upload_ms = start.elapsed().as_secs_f64() * 1000.;
        textures.insert(case.id.clone(), texture, image.bytes.len());
        drop(image);
        next_frame().await;
        let texture = textures.get(&case.id).unwrap();
        let mut motion = CombatMotion::default();
        let actor = serde_json::from_str("1").unwrap();
        for dy in [-1, 1] {
            for step in 0..2 {
                motion.glide_timed(
                    actor,
                    GridPos::new(0, step * dy),
                    GridPos::new(0, (step + 1) * dy),
                    1. + f64::from(step),
                    400,
                );
                for tick in 0..12 {
                    let now = 1. + f64::from(step) + (f64::from(tick) + 0.5) / 12. * 0.4;
                    let pose = motion
                        .vertical_walk_pose(actor, now, false)
                        .ok_or("C06 no Y pose")?;
                    if i32::from(pose.facing) != dy * 2 || pose.frame >= 6 {
                        return Err(format!("C06 heading/phase: {}", case.id));
                    }
                }
                if motion
                    .vertical_walk_pose(actor, 1.401 + f64::from(step), false)
                    .is_some()
                {
                    return Err(format!("C06 front idle not released: {}", case.id));
                }
                front_returns += 1;
            }
        }
        let start = Instant::now();
        for warm in 0..2 {
            set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
            clear_background(Color::from_rgba(30, 40, 49, 255));
            draw_text(
                &format!("{review} / {} / natif64", case.id),
                16.,
                24.,
                21.,
                WHITE,
            );
            draw_text(
                "Face 01-06 / dos 07-12 / repos frontal 13",
                16.,
                48.,
                17.,
                LIGHTGRAY,
            );
            let _world = world_readability::world(false);
            for n in 0..13 {
                let x = 16. + (n % 4) as f32 * 232.;
                let y = 62. + (n / 4) as f32 * 164.;
                draw_text(&format!("{:02}", n + 1), x, y + 16., 14., GRAY);
                let s = f32::from(case.side);
                let audit = world_readability::audit_textures(|| {
                    let _actor = world_readability::actor(true);
                    world_readability::texture(
                        texture,
                        x + 116. - case.foot[0],
                        y + 148. - case.foot[1],
                        WHITE,
                        DrawTextureParams {
                            source: Some(Rect::new((n % 6) as f32 * s, (n / 6) as f32 * s, s, s)),
                            dest_size: Some(vec2(s, s)),
                            ..Default::default()
                        },
                    );
                });
                if audit.len() != 1
                    || audit
                        .iter()
                        .any(|d| !d.actor || !d.world || !d.finish || d.gain != 1.)
                {
                    return Err(format!("C06 inconsistent actor finish: {}", case.id));
                }
            }
            if warm == 0 {
                next_frame().await;
            } else if case.save {
                crate::ui_capture::framebuffer()?
                    .export_png(output.join(format!("{}.png", case.id)).to_str().unwrap());
                saved += 1;
            }
        }
        rows.push(serde_json::json!({"id":case.id,"kind":case.kind,"read_ms":read_ms,"decode_ms":decode_ms,"upload_ms":upload_ms,"board_ms_including_frame_wait":start.elapsed().as_secs_f64()*1000.}));
        if index % 100 == 0 {
            eprintln!("[C06] {}/{}", index + 1, cases.len());
        }
        next_frame().await;
    }
    std::fs::write(output.join("checks.json"), serde_json::to_vec_pretty(&serde_json::json!({
        "review":review,"normal_game_enabled":false,"native_tile_px":64,
        "prepared_atlases":cases.len(),"native_draws":cases.len()*13,"saved_contact_boards":saved,
        "front_returns":front_returns,"same_actor_finish_every_pose":true,"neutral_actor_gain":1.,
        "texture_cache_limit_bytes":32*1024*1024,"read_decode_in_draw":false,"gpu_upload_on_context_thread":true,
        "saved_game_loaded":false,"preferences_written":false,"measurements":rows
    })).unwrap()).map_err(|e|e.to_string())?;
    let groups: std::collections::BTreeSet<_> =
        cases.iter().filter_map(|c| c.camp_group.as_ref()).collect();
    for group in groups {
        let selected: Vec<_> = cases
            .iter()
            .filter(|c| c.camp_group.as_ref() == Some(group))
            .collect();
        let mut prepared = Vec::new();
        for case in &selected {
            let file = case.file.clone();
            let worker = std::thread::spawn(move || -> Result<Image, String> {
                let bytes = std::fs::read(file).map_err(|e| e.to_string())?;
                Image::from_file_with_format(&bytes, Some(ImageFormat::Png))
                    .map_err(|e| e.to_string())
            });
            while !worker.is_finished() {
                next_frame().await;
            }
            let image = worker.join().map_err(|_| "C10 camp worker panic")??;
            let texture = crate::asset_loading::from_image(&image);
            texture.set_filter(FilterMode::Nearest);
            prepared.push(texture);
        }
        let directory = output.join(format!("camp-{group}"));
        std::fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
        crate::vertical_walk_review18::capture_prepared_camp(
            &directory,
            &review,
            selected.len(),
            |cell, index, pose| {
                let case = selected[index];
                let n = match pose {
                    Some(p) if p.frame < 6 => {
                        usize::from(p.frame) + if p.facing == -2 { 6 } else { 0 }
                    }
                    Some(_) => return false,
                    None => 12,
                };
                draw_ellipse(
                    cell.center().x,
                    cell.bottom() - 1.,
                    18.,
                    2.5,
                    0.,
                    Color::from_rgba(8, 12, 16, 65),
                );
                let _actor = world_readability::actor(true);
                let s = f32::from(case.side);
                world_readability::texture(
                    &prepared[index],
                    cell.center().x - case.foot[0],
                    cell.bottom() - 1. - case.foot[1],
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new((n % 6) as f32 * s, (n / 6) as f32 * s, s, s)),
                        dest_size: Some(vec2(s, s)),
                        ..Default::default()
                    },
                );
                true
            },
        )
        .await?;
    }
    Ok(())
}
