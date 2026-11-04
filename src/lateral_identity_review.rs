//! LATERAL02: isolated complete lateral figures derived from VERT18 identity.
use crate::{
    game_art::{
        self, Visual,
        walking::{self, Pose},
    },
    world_readability::{self, player_exposure_review::Scene},
};
use macroquad::prelude::*;
use project_rl::entity::EquipmentPalette;
use std::path::Path;
const SOURCE: &str = "assets/source-art/animation-polish-01/lateral-identity-02";
fn upload(name: &str) -> Result<Texture2D, String> {
    let image = crate::asset_preparation::image(&format!("{SOURCE}/{name}"))?;
    let t = crate::asset_loading::from_image(&image);
    t.set_filter(FilterMode::Nearest);
    Ok(t)
}
fn draw(texture: &Texture2D, cell: Rect) {
    let _player = world_readability::player_scope(true);
    let _actor = world_readability::actor(true);
    game_art::polish_review_shadow(cell);
    let scale = cell.w / 64.;
    world_readability::texture(
        texture,
        cell.center().x - 48. * scale,
        cell.bottom() - 81. * scale,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(96. * scale, 96. * scale)),
            ..Default::default()
        },
    );
}
fn panel(board: &mut Image, image: &Image, column: usize) {
    for y in 0..128usize {
        let source = ((610 - 204 - 128 + y) * 640 + 256) * 4;
        let target = (y * 512 + column * 128 + 16) * 4;
        board.bytes[target..target + 96 * 4].copy_from_slice(&image.bytes[source..source + 96 * 4]);
    }
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    capture_revision(output, 1).await
}
pub(crate) async fn capture_gait(output: &Path) -> Result<(), String> {
    capture_revision(output, 2).await
}
pub(crate) async fn capture_compact(output: &Path) -> Result<(), String> {
    capture_revision(output, 3).await
}
async fn capture_revision(output: &Path, revision: u8) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let scene = Scene::new()?;
    let front = upload("reference-12.png")?;
    let back = upload("reference-09.png")?;
    let mut figures = Vec::new();
    let mut previous = Vec::new();
    for direction in ["left", "right"] {
        figures.push(
            (0..7)
                .map(|n| {
                    upload(&format!(
                        "{}candidate-0{revision}-{direction}-{n}.png",
                        if revision > 1 {
                            format!("r0{revision}/")
                        } else {
                            String::new()
                        }
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?,
        );
        previous.push(
            (0..7)
                .map(|n| {
                    let before = revision.saturating_sub(1).max(1);
                    upload(&format!(
                        "{}candidate-0{before}-{direction}-{n}.png",
                        if before > 1 {
                            format!("r0{before}/")
                        } else {
                            String::new()
                        }
                    ))
                })
                .collect::<Result<Vec<_>, _>>()?,
        );
    }
    let old = Visual::Player {
        body: 5,
        equipment: [Some(101), None, None, None],
        weapon: None,
        hands: None,
        palettes: [EquipmentPalette::Original; 3],
    };
    let mut cycles = Vec::new();
    for (row, (direction, facing)) in [("left", -1), ("right", 1)].into_iter().enumerate() {
        let mut captured = Vec::new();
        for phase in 0..7 {
            let (a, _) = scene.render_actor(|cell| draw(&front, cell));
            let (b, _) = scene.render_actor(|cell| {
                if revision > 1 {
                    draw(&previous[row][usize::from(phase)], cell);
                    return;
                }
                walking::with_pose(
                    Some(Pose {
                        facing,
                        frame: if phase == 6 { 8 } else { phase },
                    }),
                    || game_art::draw_world_animated(cell, old, 1., None),
                )
            });
            let (c, audit) =
                scene.render_actor(|cell| draw(&figures[row][usize::from(phase)], cell));
            let (d, _) = scene.render_actor(|cell| draw(&back, cell));
            if audit.is_empty()
                || audit
                    .iter()
                    .any(|a| !a.actor || !a.world || (a.gain - 1.18).abs() > 0.0001)
            {
                return Err("LATERAL02 wrong player lighting".into());
            }
            let mut board = Image::gen_image_color(512, 160, Color::from_rgba(30, 40, 49, 255));
            for (col, image) in [&a, &b, &c, &d].into_iter().enumerate() {
                panel(&mut board, image, col);
            }
            board.export_png(
                output
                    .join(format!("identity-0{revision}-{direction}-{phase}.png"))
                    .to_str()
                    .unwrap(),
            );
            if phase < 6 {
                captured.push(c.bytes);
            }
        }
        let unique = captured
            .iter()
            .collect::<std::collections::BTreeSet<_>>()
            .len();
        if unique != 6 {
            return Err("LATERAL02 cycle lacks six distinct images".into());
        }
        cycles.push(serde_json::json!({"direction":direction,"distinct_engine_frames":unique}));
        next_frame().await;
    }
    std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":format!("LATERAL02-r0{revision}"),"normal_game_enabled":false,"class":"Traqueuse","armor_stage":3,"source_profile":"traqueuse-armor3","native_cell_px":64,"candidate_has_weapon":false,"references_unchanged":true,"bastion_unchanged":true,"comparison":if revision>1 {format!("0{} previous candidate / 0{revision} gait revision",revision-1)} else {"current game / 01 candidate".to_string()},"engine_scene":"approved camp / LIGHT01 Discret","boards":14,"cycles":cycles})).unwrap()).map_err(|e|e.to_string())?;
    Ok(())
}
