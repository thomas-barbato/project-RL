//! Current creation roster and a slower eight-pose lateral motion study.
use crate::{
    game_art::{self, Visual},
    world_readability::player_exposure_review::Scene,
};
use macroquad::prelude::*;
use project_rl::entity::EquipmentPalette;
use serde::Serialize;
use std::{collections::BTreeSet, path::Path};

const PHASE_MS: [u32; 8] = [180, 180, 170, 120, 180, 180, 170, 120];
const SAMPLES: usize = 32;

#[derive(Serialize)]
struct Class {
    id: String,
    slug: &'static str,
    name: &'static str,
    body: u8,
}

fn active_classes() -> Result<Vec<Class>, String> {
    let catalog = crate::ascii_app::ascii_character_class_catalog()?;
    catalog
        .iter()
        .filter(|(id, _)| id.as_str() != "core:traqueur")
        .map(|(id, _)| {
            let (slug, name, body) = match id.as_str() {
                "core:bastion" => ("bastion", "Bastion", 3),
                "core:arcaniste" => ("arcaniste", "Arcaniste", 6),
                "core:metamorphe" => ("metamorphe", "Métamorphe", 7),
                other => return Err(format!("Classe active sans référence de revue : {other}")),
            };
            Ok(Class {
                id: id.to_string(),
                slug,
                name,
                body,
            })
        })
        .collect()
}

fn crop(image: &Image) -> Vec<u8> {
    let mut pixels = Vec::with_capacity(128 * 128 * 4);
    for y in 278..406usize {
        let at = (y * 640 + 240) * 4;
        pixels.extend_from_slice(&image.bytes[at..at + 512]);
    }
    pixels
}
fn panel(board: &mut Image, pixels: &[u8], row: usize, col: usize, rows: usize) {
    for y in 0..128usize {
        let at = (((rows - 1 - row) * 160 + y) * 384 + col * 128) * 4;
        board.bytes[at..at + 512].copy_from_slice(&pixels[y * 512..(y + 1) * 512]);
    }
}
fn pose_at(time: f32) -> usize {
    let mut end = 0;
    for (phase, duration) in PHASE_MS.into_iter().enumerate() {
        end += duration;
        if time < (end as f32) {
            return phase;
        }
    }
    7
}
fn normal(scene: &Scene, visual: Visual, shift: f32) -> Image {
    scene
        .render_actor(|cell| {
            game_art::walking::with_pose(None, || {
                let mut at = cell;
                at.x += shift;
                game_art::draw_world_animated(at, visual, 1., None)
            })
        })
        .0
}
fn sprite(
    scene: &Scene,
    texture: &Texture2D,
    side: usize,
    phase: usize,
    shift: f32,
) -> Result<Image, String> {
    let (image, audit) = scene.render_actor(|cell| {
        let _player = crate::world_readability::player_scope(true);
        let _actor = crate::world_readability::actor(true);
        let mut cell = cell;
        cell.x += shift;
        game_art::polish_review_shadow(cell);
        crate::world_readability::texture(
            texture,
            cell.center().x - 48.,
            cell.bottom() - 81.,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(phase as f32 * 96., side as f32 * 96., 96., 96.)),
                dest_size: Some(vec2(96., 96.)),
                ..Default::default()
            },
        );
    });
    if audit.is_empty()
        || audit
            .iter()
            .any(|a| !a.world || !a.actor || (a.gain - 1.18).abs() > 0.0001)
    {
        return Err("Éclairage de revue incorrect".into());
    }
    Ok(image)
}

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    let classes = active_classes()?;
    if classes.len() != 3 {
        return Err("Périmètre des classes à réexaminer".into());
    }
    let source = "assets/source-art/animation-polish-01/class-lateral-02/r02";
    let text = project_rl::data_paths::read_to_string(&format!("{source}/manifest.json"))
        .map_err(|e| e.to_string())?;
    let metadata: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let profiles = metadata["profiles"].as_array().ok_or("Profils absents")?;
    if profiles.len() != classes.len()
        || profiles
            .iter()
            .any(|p| !classes.iter().any(|c| Some(c.slug) == p["class"].as_str()))
    {
        return Err("La banque contient une classe absente de la création".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let scene = Scene::new()?;
    let height = (classes.len() * 160) as u16;
    let mut records = Vec::new();
    let mut captures = Vec::new();
    for travel in [false, true] {
        for (side, facing) in [-1, 1].into_iter().enumerate() {
            let count = if travel { SAMPLES + 1 } else { 9 };
            let mut boards: Vec<_> = (0..count)
                .map(|_| Image::gen_image_color(384, height, Color::from_rgba(30, 40, 49, 255)))
                .collect();
            for (row, class) in classes.iter().enumerate() {
                let p = profiles
                    .iter()
                    .find(|p| p["class"].as_str() == Some(class.slug))
                    .unwrap();
                let new_image =
                    crate::asset_preparation::image(p["atlas"].as_str().ok_or("Atlas absent")?)?;
                let old_image = crate::asset_preparation::image(&format!(
                    "assets/source-art/animation-polish-01/class-lateral-02/r01/{}-t3-atlas.png",
                    class.slug
                ))?;
                let new = crate::asset_loading::from_image(&new_image);
                new.set_filter(FilterMode::Nearest);
                let old = crate::asset_loading::from_image(&old_image);
                old.set_filter(FilterMode::Nearest);
                let visual = Visual::Player {
                    body: class.body,
                    equipment: [Some(101), None, None, None],
                    weapon: None,
                    hands: None,
                    palettes: [EquipmentPalette::Original; 3],
                };
                game_art::vertical::prepare(&[visual])?;
                let idle = crop(&normal(&scene, visual, 0.));
                let mut unique = BTreeSet::new();
                for (sample, board) in boards.iter_mut().enumerate() {
                    let stop = sample == count - 1;
                    let phase = if stop {
                        8
                    } else if travel {
                        pose_at(sample as f32 * 1300. / SAMPLES as f32)
                    } else {
                        sample
                    };
                    let progress = if stop {
                        1.
                    } else {
                        sample as f32 / (count - 1) as f32
                    };
                    // Translate the entire actor; its limbs remain authored pixels.
                    let shift = if travel {
                        -(facing as f32) * (20. - 40. * progress)
                    } else {
                        0.
                    };
                    panel(board, &idle, row, 0, classes.len());
                    if stop {
                        let rest = crop(&normal(&scene, visual, shift));
                        for col in [1, 2] {
                            panel(board, &rest, row, col, classes.len());
                        }
                    } else {
                        let previous = crop(&sprite(&scene, &old, side, phase * 6 / 8, shift)?);
                        let candidate = crop(&sprite(&scene, &new, side, phase, shift)?);
                        panel(board, &previous, row, 1, classes.len());
                        panel(board, &candidate, row, 2, classes.len());
                        if !travel {
                            unique.insert(candidate);
                        }
                    }
                }
                if !travel {
                    if unique.len() != 8 {
                        return Err(format!(
                            "Huit poses absentes pour {} / {facing}",
                            class.slug
                        ));
                    }
                    records.push(serde_json::json!({"class":class.slug,"facing":facing,"distinct_key_poses":unique.len()}));
                }
                next_frame().await;
            }
            for (sample, board) in boards.into_iter().enumerate() {
                let kind = if travel { "travel" } else { "keys" };
                let direction = if facing < 0 { "left" } else { "right" };
                let file = format!("{kind}-{direction}-{sample:02}.png");
                board.export_png(output.join(&file).to_str().unwrap());
                captures.push(file);
            }
        }
    }
    let phase_samples = [0, 5, 9, 14, 16, 21, 25, 30, 32];
    let report = serde_json::json!({"review":"CLASS-LATERAL02-r02","status":"native_candidate_ready","classes":classes,"class_source":"CharacterClassCatalog::iter, available_at_creation","retired_technomancien_excluded":true,"normal_game_enabled":false,"traqueuse_unchanged":true,"unarmed_only":true,"armor_stage":3,"native_cell_px":64,"board":[384,height],"walk_frames":8,"step_ms":650,"cycle_ms":1300,"phase_durations_ms":PHASE_MS,"travel_samples":SAMPLES,"travel_px_per_cycle":40,"travel_is_review_study":true,"phase_samples":phase_samples,"key_pose_checks":records,"captures":captures,"previous_candidate_same_cadence":true});
    std::fs::write(
        output.join("checks.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn review_roster_follows_creation_and_excludes_retired_and_traqueuse() {
        let roster = active_classes().unwrap();
        assert_eq!(
            roster.iter().map(|c| c.id.as_str()).collect::<Vec<_>>(),
            vec!["core:bastion", "core:arcaniste", "core:metamorphe"]
        );
        let catalog = crate::ascii_app::ascii_character_class_catalog().unwrap();
        assert!(
            !catalog
                .get(&"core:technomancien".parse().unwrap())
                .unwrap()
                .available_at_creation()
        );
        assert_eq!(PHASE_MS.iter().sum::<u32>(), 1300);
        assert_eq!(PHASE_MS[..4].iter().sum::<u32>(), 650);
    }
}
