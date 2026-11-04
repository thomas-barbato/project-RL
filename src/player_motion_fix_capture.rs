//! Exercise the map's shared heading selector with colored player outfits.
use crate::{
    combat_visuals::CombatMotion,
    game_art::{
        self, Visual,
        walking::{self, Pose},
    },
    world_readability::player_exposure_review::Scene,
};
use macroquad::prelude::*;
use project_rl::{
    entity::{EntityId, EquipmentPalette},
    world::GridPos,
};
use std::{collections::BTreeSet, path::Path};

fn visual(body: u8, torso: Option<u16>, color: EquipmentPalette, weapon: Option<u16>) -> Visual {
    Visual::Player {
        body,
        equipment: [torso, None, None, None],
        weapon,
        hands: None,
        palettes: [
            color,
            EquipmentPalette::Original,
            EquipmentPalette::Original,
        ],
    }
}
fn panel(board: &mut Image, source: &Image, col: usize) {
    for y in 0..160 {
        let src = ((y + 278) * 640 + 224) * 4;
        let dst = (y * 800 + col * 160) * 4;
        board.bytes[dst..dst + 640].copy_from_slice(&source.bytes[src..src + 640]);
    }
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let mut cases = Vec::new();
    for (tier, torso) in [None, Some(110), Some(102), Some(101), Some(103)]
        .into_iter()
        .enumerate()
    {
        for (name, color) in [
            ("original", EquipmentPalette::Original),
            ("blue", EquipmentPalette::Blue),
        ] {
            cases.push((
                format!("traqueuse-{tier}-{name}"),
                visual(5, torso, color, None),
            ));
        }
    }
    for body in [3, 5, 6, 7] {
        cases.push((
            format!("class-{body}-blue-weapon67"),
            visual(body, Some(109), EquipmentPalette::Blue, Some(67)),
        ));
    }
    let visuals: Vec<_> = cases.iter().map(|(_, v)| *v).collect();
    game_art::vertical::prepare(&visuals)?;
    let scene = Scene::new()?;
    let mut rows = Vec::new();
    let mut exposures = 0;
    for (name, visual) in cases {
        let draw = |pose: Option<Pose>| -> Result<Image, String> {
            let (image, audit) = scene.render_actor(|cell| {
                walking::with_pose(pose, || {
                    game_art::draw_world_animated(cell, visual, 1., None)
                })
            });
            if audit.is_empty()
                || audit.iter().any(|a| {
                    !a.world
                        || !a.actor
                        || (a.gain - 1.18).abs() > 0.001
                        || (a.lift - 0.30).abs() > 0.001
                        || (a.edge - 0.012).abs() > 0.001
                })
            {
                return Err(format!("Player lighting differs: {name} {pose:?}"));
            }
            Ok(image)
        };
        let mut board = Image::gen_image_color(800, 192, Color::from_rgba(30, 40, 49, 255));
        let previously_supported = matches!(
            name.as_str(),
            "traqueuse-0-original" | "traqueuse-4-original"
        );
        let previous_pose = if previously_supported {
            None
        } else {
            Some(Pose {
                facing: 0,
                frame: 8,
            })
        };
        panel(&mut board, &draw(previous_pose)?, 0);
        panel(&mut board, &draw(None)?, 1);
        exposures += 2;
        let mut directions = Vec::new();
        for (index, (dx, dy)) in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().enumerate() {
            let id: EntityId = serde_json::from_str("1").unwrap();
            let mut motion = CombatMotion::default();
            let mut poses = Vec::new();
            let mut distinct = BTreeSet::new();
            for step in 0..2 {
                let start = 1. + f64::from(step) * 0.4;
                motion.glide_timed(
                    id,
                    GridPos::new(step * dx, step * dy),
                    GridPos::new((step + 1) * dx, (step + 1) * dy),
                    start,
                    400,
                );
                for (p, t) in [0.01, 0.15, 0.29].into_iter().enumerate() {
                    let pose =
                        walking::world_pose(&motion, id, Some(visual), true, start + t, false)
                            .ok_or("Missing player pose")?;
                    let expected = if dy != 0 { (dy * 2) as i8 } else { dx as i8 };
                    if pose.facing != expected {
                        return Err(format!("Wrong heading: {name} {pose:?}"));
                    }
                    let image = draw(Some(pose))?;
                    exposures += 1;
                    distinct.insert(image.bytes.clone());
                    if step == 0 && p == 1 && index < 3 {
                        panel(&mut board, &image, index + 2);
                    }
                    poses.push([pose.facing, pose.frame as i8]);
                }
                if walking::world_pose(&motion, id, Some(visual), true, start + 0.401, false)
                    .is_some()
                {
                    return Err("Arrival is not frontal".into());
                }
            }
            if distinct.len() != 6 {
                return Err(format!(
                    "Frozen cycle {name}: {dx},{dy} has {} drawings",
                    distinct.len()
                ));
            }
            directions.push(serde_json::json!({"delta":[dx,dy],"distinct_drawings":distinct.len(),"poses":poses}));
        }
        board.export_png(output.join(format!("{name}.png")).to_str().unwrap());
        rows.push(serde_json::json!({"id":name,"visual":format!("{visual:?}"),"directions":directions,"lighting":{"gain":1.18,"lift":0.30,"edge":0.012}}));
        next_frame().await;
    }
    let report = serde_json::json!({"review":"PLAYER-MOTION-FIX01","cases":rows,"player_texture_exposure_checks":exposures,"normal_renderer":true,"shared_map_pose_selector":true,"step_ms":400,"six_poses_per_direction":true,"original_source_art_changed":false,"user_data_loaded":false});
    std::fs::write(
        output.join("checks.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
