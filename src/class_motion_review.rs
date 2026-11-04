//! Read-only review of the normal renderer, excluding the Traqueuse trial.
use crate::{
    game_art::{
        self, Visual,
        walking::{self, Pose},
    },
    world_readability::player_exposure_review::Scene,
};
use macroquad::prelude::*;
use project_rl::entity::EquipmentPalette;
use std::{collections::BTreeSet, path::Path};

const CLASSES: [(&str, &str, u8); 4] = [
    ("bastion", "Bastion", 3),
    ("technomancien", "Technomancien", 4),
    ("arcaniste", "Arcaniste", 6),
    ("metamorphe", "Métamorphe", 7),
];
const ARMORS: [Option<u16>; 5] = [None, Some(110), Some(102), Some(101), Some(103)];
const WEAPONS: [Option<u16>; 4] = [None, Some(2), Some(31), Some(66)];

fn ensure_current_roster() -> Result<(), String> {
    let catalog = crate::ascii_app::ascii_character_class_catalog()?;
    if CLASSES.iter().any(|(slug, _, _)| {
        !catalog
            .iter()
            .any(|(id, _)| id.as_str() == format!("core:{slug}"))
    }) {
        return Err("Cette revue historique contient une classe retirée ; utiliser --ui-cold-class-lateral-r02".into());
    }
    Ok(())
}

/// Current frontal art before lighting, for the lateral-only redraw.
pub(crate) async fn references(output: &Path) -> Result<(), String> {
    ensure_current_roster()?;
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let mut records = Vec::new();
    for (id, name, body) in CLASSES {
        for (tier, armor) in ARMORS.into_iter().enumerate() {
            let visual = Visual::Player {
                body,
                equipment: [armor, None, None, None],
                weapon: None,
                hands: None,
                palettes: [EquipmentPalette::Original; 3],
            };
            let (front, origin) = if game_art::vertical::supports(Some(visual)) {
                let p = game_art::vertical::review_image(visual)?;
                let frame = p.image.sub_image(Rect::new(
                    0.,
                    2. * f32::from(p.cell[1]),
                    f32::from(p.cell[0]),
                    f32::from(p.cell[1]),
                ));
                let mut image = Image::gen_image_color(128, 128, BLANK);
                let x = (64. - p.anchor[0]).round() as usize;
                let y = (96. - p.anchor[1]).round() as usize;
                for row in 0..usize::from(frame.height) {
                    let to = ((y + row) * 128 + x) * 4;
                    let width = usize::from(frame.width) * 4;
                    image.bytes[to..to + width]
                        .copy_from_slice(&frame.bytes[row * width..(row + 1) * width]);
                }
                (image, "VERT18 frontal idle")
            } else {
                (
                    game_art::vertical_review_standing(visual).ok_or("Frontal absent")?,
                    "normal static fallback",
                )
            };
            let file = format!("{id}-t{tier}-front.png");
            front.export_png(output.join(&file).to_str().unwrap());
            records.push(serde_json::json!({"class":id,"name":name,"body":body,"tier":tier,"armor":armor,"file":file,"origin":origin,"anchor":[64,96]}));
        }
        next_frame().await;
    }
    std::fs::write(
        output.join("references.json"),
        serde_json::to_vec_pretty(&records).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn crop(image: &Image) -> Vec<u8> {
    // The same native camp crop as LATERAL02, widened to retain long weapons.
    let mut pixels = Vec::with_capacity(128 * 128 * 4);
    for y in 278..406usize {
        let at = (y * 640 + 240) * 4;
        pixels.extend_from_slice(&image.bytes[at..at + 128 * 4]);
    }
    pixels
}
fn panel(board: &mut Image, pixels: &[u8], row: usize, col: usize) {
    for y in 0..128usize {
        let at = (((3 - row) * 160 + y) * 640 + col * 128) * 4;
        board.bytes[at..at + 512].copy_from_slice(&pixels[y * 512..(y + 1) * 512]);
    }
}
fn render(scene: &Scene, visual: Visual, pose: Option<Pose>) -> Result<Image, String> {
    let (image, audit) = scene.render_actor(|cell| {
        walking::with_pose(pose, || {
            game_art::draw_world_animated(cell, visual, 1., None)
        })
    });
    if audit.is_empty()
        || audit
            .iter()
            .any(|a| !a.actor || !a.world || (a.gain - 1.18).abs() > 0.0001)
    {
        return Err(format!(
            "Éclairage inattendu dans la revue des classes : {visual:?}"
        ));
    }
    Ok(image)
}

/// New lateral candidates beside the unchanged normal front and old sides.
pub(crate) async fn capture_lateral_candidates(output: &Path) -> Result<(), String> {
    ensure_current_roster()?;
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let source = "assets/source-art/animation-polish-01/class-lateral-02/r01";
    let metadata: serde_json::Value = serde_json::from_str(
        &project_rl::data_paths::read_to_string(&format!("{source}/manifest.json"))
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let profiles = metadata["profiles"].as_array().ok_or("Profils absents")?;
    let scene = Scene::new()?;
    let mut records = Vec::new();
    let tiers: BTreeSet<_> = profiles.iter().filter_map(|p| p["tier"].as_u64()).collect();
    for tier in tiers {
        let tier = tier as usize;
        let mut boards: Vec<_> = (0..7)
            .map(|_| Image::gen_image_color(640, 640, Color::from_rgba(30, 40, 49, 255)))
            .collect();
        for (row, (id, name, body)) in CLASSES.into_iter().enumerate() {
            let p = profiles
                .iter()
                .find(|p| {
                    p["class"].as_str() == Some(id) && p["tier"].as_u64() == Some(tier as u64)
                })
                .ok_or("Lot incomplet")?;
            let path = p["atlas"].as_str().ok_or("Atlas absent")?;
            let image = crate::asset_preparation::image(path)?;
            let texture = crate::asset_loading::from_image(&image);
            texture.set_filter(FilterMode::Nearest);
            let visual = Visual::Player {
                body,
                equipment: [ARMORS[tier], None, None, None],
                weapon: None,
                hands: None,
                palettes: [EquipmentPalette::Original; 3],
            };
            game_art::vertical::prepare(&[visual])?;
            let idle = crop(&render(&scene, visual, None)?);
            for board in &mut boards {
                panel(board, &idle, row, 0);
            }
            for col in 1..5 {
                panel(&mut boards[6], &idle, row, col);
            }
            let mut cycle_checks = Vec::new();
            for (side, facing) in [-1, 1].into_iter().enumerate() {
                let mut unique = BTreeSet::new();
                for frame in 0..6u8 {
                    let old = render(&scene, visual, Some(Pose { facing, frame }))?;
                    panel(
                        &mut boards[usize::from(frame)],
                        &crop(&old),
                        row,
                        1 + side * 2,
                    );
                    let (new, audit) = scene.render_actor(|cell| {
                        let _player = crate::world_readability::player_scope(true);
                        let _actor = crate::world_readability::actor(true);
                        game_art::polish_review_shadow(cell);
                        let scale = cell.w / 64.;
                        crate::world_readability::texture(
                            &texture,
                            cell.center().x - 48. * scale,
                            cell.bottom() - 81. * scale,
                            WHITE,
                            DrawTextureParams {
                                source: Some(Rect::new(
                                    f32::from(frame) * 96.,
                                    side as f32 * 96.,
                                    96.,
                                    96.,
                                )),
                                dest_size: Some(vec2(96. * scale, 96. * scale)),
                                ..Default::default()
                            },
                        );
                    });
                    if audit.is_empty()
                        || audit
                            .iter()
                            .any(|a| !a.actor || !a.world || (a.gain - 1.18).abs() > 0.0001)
                    {
                        return Err("Éclairage du candidat incorrect".into());
                    }
                    let pixels = crop(&new);
                    panel(&mut boards[usize::from(frame)], &pixels, row, 2 + side * 2);
                    unique.insert(pixels);
                }
                if unique.len() != 6 {
                    return Err(format!("Cycle incomplet : {id} / {tier} / {facing}"));
                }
                cycle_checks.push(
                    serde_json::json!({"facing":facing,"distinct_engine_frames":unique.len()}),
                );
            }
            records.push(serde_json::json!({"class":id,"name":name,"tier":tier,"cycles":cycle_checks,"front_unchanged":true,"atlas":path}));
            next_frame().await;
        }
        for (phase, board) in boards.into_iter().enumerate() {
            board.export_png(
                output
                    .join(format!("lateral-{tier}-{phase}.png"))
                    .to_str()
                    .unwrap(),
            );
        }
    }
    let report = serde_json::json!({"review":"CLASS-LATERAL02-r01","status":"native_candidate_ready","normal_game_enabled":false,"unarmed_only":true,"traqueuse_unchanged":true,"native_cell_px":64,"classes":CLASSES.map(|(id,name,_)|serde_json::json!({"id":id,"name":name})),"profiles":records,"columns":["Repos actuel","Ancien gauche","Nouveau gauche","Ancien droite","Nouveau droite"],"board":[640,640],"step_ms":400,"frame6":"unchanged frontal rest"});
    std::fs::write(
        output.join("checks.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    ensure_current_roster()?;
    if game_art::traqueuse_trial::enabled() || walking::corrections_enabled() {
        return Err("La revue doit utiliser le rendu normal, sans candidat activé".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let scene = Scene::new()?;
    let mut checks = Vec::new();
    let mut render_count = 0;
    for (tier, armor) in ARMORS.into_iter().enumerate() {
        for (loadout, weapon) in WEAPONS.into_iter().enumerate() {
            let mut boards: Vec<_> = (0..7)
                .map(|_| Image::gen_image_color(640, 640, Color::from_rgba(30, 40, 49, 255)))
                .collect();
            for (row, (id, name, body)) in CLASSES.into_iter().enumerate() {
                let visual = Visual::Player {
                    body,
                    equipment: [armor, None, None, None],
                    weapon,
                    hands: None,
                    palettes: [EquipmentPalette::Original; 3],
                };
                game_art::vertical::prepare(&[visual])?;
                let vertical = game_art::vertical::supports(Some(visual));
                let idle = render(&scene, visual, None)?;
                render_count += 1;
                if tier == 3 && loadout < 2 {
                    idle.export_png(
                        output
                            .join(format!("camp-{id}-{loadout}.png"))
                            .to_str()
                            .unwrap(),
                    );
                }
                let idle_pixels = crop(&idle);
                for board in &mut boards {
                    panel(board, &idle_pixels, row, 0);
                }
                for col in 1..5 {
                    panel(&mut boards[6], &idle_pixels, row, col);
                }
                let mut directions = Vec::new();
                for (col, (direction, facing)) in
                    [("left", -1), ("right", 1), ("up", -2), ("down", 2)]
                        .into_iter()
                        .enumerate()
                {
                    let mut unique = BTreeSet::new();
                    let mut vertical_draws = 0;
                    for frame in 0..6u8 {
                        // terminal_view uses facing 0 when VERT18 is unsupported.
                        let actual_facing = if col >= 2 && !vertical { 0 } else { facing };
                        let before = game_art::vertical::draw_count();
                        let image = render(
                            &scene,
                            visual,
                            Some(Pose {
                                facing: actual_facing,
                                frame,
                            }),
                        )?;
                        vertical_draws += game_art::vertical::draw_count() - before;
                        render_count += 1;
                        let pixels = crop(&image);
                        panel(&mut boards[usize::from(frame)], &pixels, row, col + 1);
                        unique.insert(pixels);
                    }
                    directions.push(serde_json::json!({"direction":direction,"distinct_frames":unique.len(),"vertical_draws":vertical_draws}));
                }
                checks.push(serde_json::json!({"class":id,"name":name,"body":body,"armor_stage":tier,"armor_model":armor,"loadout":loadout,"weapon_model":weapon,"vertical_supported":vertical,"directions":directions,"arrival":"normal frontal idle"}));
                eprintln!("[CLASS-MOTION01] {name} / armor {tier} / loadout {loadout} captured");
                next_frame().await;
            }
            for (phase, board) in boards.into_iter().enumerate() {
                board.export_png(
                    output
                        .join(format!("classes-{tier}-{loadout}-{phase}.png"))
                        .to_str()
                        .unwrap(),
                );
            }
        }
    }
    let report = serde_json::json!({"review":"CLASS-MOTION01-r01","capture_status":"passed","visual_approval":false,"normal_renderer_modified":false,"traqueuse_excluded":true,"native_cell_px":64,"step_ms":400,"board":[640,640],"columns":["Repos","Gauche","Droite","Haut","Bas"],"classes":CLASSES.map(|(id,name,body)| serde_json::json!({"id":id,"name":name,"body":body})),"weapons":WEAPONS,"armor_models":ARMORS,"boards":140,"engine_renders":render_count,"cases":checks});
    std::fs::write(
        output.join("checks.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
