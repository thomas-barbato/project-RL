//! Review-only whole-figure creature gaits and independent rifle grip study.
use crate::{
    game_art::{self, Visual, walking::Pose},
    world_readability::player_exposure_review::Scene,
};
use macroquad::prelude::*;
use project_rl::entity::EquipmentPalette;
use serde::Deserialize;
use std::{collections::BTreeSet, path::Path};

const PHASE_MS: [u32; 8] = [180, 180, 170, 120, 180, 180, 170, 120];
const SAMPLES: usize = 32;
#[path = "biped_player_review.rs"]
mod from_player;
#[path = "biped_next_review.rs"]
mod next_batch;
pub(crate) async fn capture_next_batch(output: &Path) -> Result<(), String> {
    next_batch::capture(output).await
}
pub(crate) async fn capture_armed_batch(output: &Path) -> Result<(), String> {
    next_batch::capture_armed(output).await
}
pub(crate) async fn capture_hoods(output: &Path) -> Result<(), String> {
    next_batch::capture_hoods(output).await
}
pub(crate) async fn capture_fronts(output: &Path) -> Result<(), String> {
    next_batch::capture_fronts(output).await
}
pub(crate) async fn capture_frontmatch(output: &Path) -> Result<(), String> {
    next_batch::capture_frontmatch(output).await
}
pub(crate) async fn capture_winged(output: &Path) -> Result<(), String> {
    next_batch::capture_winged(output).await
}
#[path = "biped_low_review.rs"]
mod low;
pub(crate) async fn capture_player_reference(output: &Path) -> Result<(), String> {
    from_player::capture(output).await
}
pub(crate) async fn capture_low(output: &Path) -> Result<(), String> {
    low::capture(output).await
}
#[derive(Deserialize)]
struct Profile {
    id: String,
    number: u16,
    name: String,
    atlas: String,
    vertical_offset: i16,
}
#[derive(Deserialize)]
struct Old {
    number: u16,
    file: String,
    foot: [u16; 2],
    vertical_offset: i16,
}

fn phase_at(t: f32) -> usize {
    let mut end = 0;
    for (i, d) in PHASE_MS.into_iter().enumerate() {
        end += d;
        if t < end as f32 {
            return i;
        }
    }
    7
}
fn upload(image: &Image) -> Texture2D {
    let t = crate::asset_loading::from_image(image);
    t.set_filter(FilterMode::Nearest);
    t
}
fn articulation_checks(image: &Image) -> Result<[usize; 2], String> {
    if ![1152, 1440].contains(&image.width) || image.height != 256 {
        return Err("Dimensions du nouveau cycle incorrectes".into());
    }
    let width = f32::from(image.width / 9);
    let counts = std::array::from_fn(|side| {
        (0..8)
            .map(|phase| {
                image
                    .sub_image(Rect::new(
                        phase as f32 * width,
                        side as f32 * 128.,
                        width,
                        128.,
                    ))
                    .bytes
            })
            .collect::<BTreeSet<_>>()
            .len()
    });
    if counts != [8, 8] {
        return Err("Le nouveau cycle ne contient pas huit dessins distincts par cote".into());
    }
    Ok(counts)
}
fn file(path: &str) -> Result<Image, String> {
    Ok((*crate::asset_preparation::image(path)?).clone())
}
fn crop(image: &Image) -> Vec<u8> {
    let mut out = Vec::with_capacity(160 * 160 * 4);
    for y in 278..438 {
        let start = (y * 640 + 224) * 4;
        out.extend_from_slice(&image.bytes[start..start + 640]);
    }
    out
}
fn panel(board: &mut Image, image: &Image, col: usize) {
    let bytes = crop(image);
    for y in 0..160 {
        let at = (y * 480 + col * 160) * 4;
        board.bytes[at..at + 640].copy_from_slice(&bytes[y * 640..(y + 1) * 640]);
    }
}
fn static_render(scene: &Scene, visual: Visual, shift: f32) -> Image {
    scene
        .render_actor(|mut cell| {
            cell.x += shift;
            game_art::walking::with_pose(None, || {
                game_art::draw_world_animated(cell, visual, 1., None)
            });
        })
        .0
}
fn sprite_render(
    scene: &Scene,
    texture: &Texture2D,
    source: Rect,
    anchor: [f32; 2],
    offset: i16,
    shift: f32,
    player: bool,
) -> Result<Image, String> {
    sprite_render_shadow(scene, texture, source, anchor, offset, shift, player, 17)
}
fn sprite_render_shadow(
    scene: &Scene,
    texture: &Texture2D,
    source: Rect,
    anchor: [f32; 2],
    offset: i16,
    shift: f32,
    player: bool,
    shadow_width: u8,
) -> Result<Image, String> {
    let (image, audit) = scene.render_actor(|mut cell| {
        let _player = crate::world_readability::player_scope(player);
        let _actor = crate::world_readability::actor(true);
        cell.x += shift;
        cell.y += f32::from(offset);
        game_art::polish_review_shadow_width(cell, shadow_width);
        crate::world_readability::texture(
            texture,
            cell.center().x - anchor[0],
            cell.bottom() - anchor[1],
            WHITE,
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(vec2(source.w, source.h)),
                ..Default::default()
            },
        );
    });
    let expected = if player { 1.18 } else { 1.48 };
    if audit.is_empty()
        || audit
            .iter()
            .any(|a| !a.world || !a.actor || (a.gain - expected).abs() > 0.001)
    {
        return Err("Exposition de l'acteur incoherente".into());
    }
    Ok(image)
}

fn over(dest: &mut Image, src: &Image) {
    for (d, s) in dest
        .bytes
        .chunks_exact_mut(4)
        .zip(src.bytes.chunks_exact(4))
    {
        let a = u32::from(s[3]);
        if a == 0 {
            continue;
        }
        let old = u32::from(d[3]) * (255 - a);
        let total = a * 255 + old;
        for c in 0..3 {
            d[c] = ((u32::from(s[c]) * a * 255 + u32::from(d[c]) * old + total / 2) / total) as u8;
        }
        d[3] = ((total + 127) / 255) as u8;
    }
}
fn palms(side: usize, phase: usize) -> [[f32; 2]; 2] {
    const LEFT: [[[f32; 2]; 2]; 8] = [
        [[61., 80.], [50., 77.]],
        [[61., 78.], [50., 76.]],
        [[62., 79.], [51., 77.]],
        [[61., 78.], [50., 76.]],
        [[60., 79.], [50., 76.]],
        [[61., 80.], [51., 78.]],
        [[61., 78.], [51., 76.]],
        [[61., 79.], [50., 77.]],
    ];
    const RIGHT: [[[f32; 2]; 2]; 8] = [
        [[67., 77.], [78., 74.]],
        [[67., 78.], [78., 75.]],
        [[67., 77.], [77., 75.]],
        [[67., 78.], [78., 75.]],
        [[67., 78.], [78., 75.]],
        [[67., 77.], [78., 74.]],
        [[66., 78.], [77., 75.]],
        [[67., 78.], [78., 75.]],
    ];
    if side == 0 { LEFT[phase] } else { RIGHT[phase] }
}
fn rifle_pose(
    body: &Image,
    weapon: &Image,
    pivot: [f32; 2],
    support: [f32; 2],
    side: usize,
    phase: usize,
) -> Result<(Image, serde_json::Value), String> {
    let [a, b] = palms(side, phase);
    let flip = if side == 0 { -1. } else { 1. };
    let vector = vec2(flip * (support[0] - pivot[0]), support[1] - pivot[1]);
    let delta = vec2(b[0] - a[0], b[1] - a[1]);
    let angle = delta.y.atan2(delta.x) - vector.y.atan2(vector.x);
    let scale = 0.68;
    let error = (delta.length() - vector.length() * scale).abs();
    if error > 2.0 {
        return Err(format!("Ecart des deux prises: {side}/{phase} = {error}"));
    }
    for p in [a, b] {
        if body.bytes[(p[1] as usize * 128 + p[0] as usize) * 4 + 3] < 64 {
            return Err(format!("Prise hors du gant: {side}/{phase}/{p:?}"));
        }
    }
    let (sn, cs) = angle.sin_cos();
    let mut gun = Image::gen_image_color(128, 128, BLANK);
    for y in 0..128 {
        for x in 0..128 {
            let dx = (x as f32 - a[0]) / scale;
            let dy = (y as f32 - a[1]) / scale;
            let sx = (pivot[0] + flip * (cs * dx + sn * dy)).round() as i32;
            let sy = (pivot[1] - sn * dx + cs * dy).round() as i32;
            if (0..i32::from(weapon.width)).contains(&sx)
                && (0..i32::from(weapon.height)).contains(&sy)
            {
                let to = (y * 128 + x) * 4;
                let from = (sy as usize * usize::from(weapon.width) + sx as usize) * 4;
                gun.bytes[to..to + 4].copy_from_slice(&weapon.bytes[from..from + 4]);
            }
        }
    }
    let mut out = body.clone();
    over(&mut out, &gun);
    // Foreground fingers belong to this very authored arm pose. Only layer
    // ownership changes; no limb, wrist or sleeve is translated or deformed.
    let mut fingers = Image::gen_image_color(128, 128, BLANK);
    for p in [a, b] {
        for y in p[1] as usize - 2..=p[1] as usize + 2 {
            for x in p[0] as usize - 2..=p[0] as usize + 2 {
                let at = (y * 128 + x) * 4;
                fingers.bytes[at..at + 4].copy_from_slice(&body.bytes[at..at + 4]);
            }
        }
    }
    over(&mut out, &fingers);
    Ok((
        out,
        serde_json::json!({"side":side,"phase":phase,"primary":a,"support":b,"fixed_scale":scale,"angle":angle,"support_distance_error_px":error}),
    ))
}

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let text = project_rl::data_paths::read_to_string(
        "assets/source-art/animation-polish-01/biped-lateral-01/r01/manifest.json",
    )
    .map_err(|e| e.to_string())?;
    let metadata: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    let profiles: Vec<Profile> =
        serde_json::from_value(metadata["profiles"].clone()).map_err(|e| e.to_string())?;
    let old_base: serde_json::Value =
        serde_json::from_str(include_str!("../assets/animations/walk-07/manifest.json")).unwrap();
    let mut old: Vec<Old> = serde_json::from_value(old_base["entries"].clone()).unwrap();
    let overrides: Vec<Old> = serde_json::from_str(include_str!(
        "../assets/animations/walk-11/creatures/manifest.json"
    ))
    .unwrap();
    for replacement in overrides {
        if let Some(p) = old.iter_mut().find(|e| e.number == replacement.number) {
            *p = replacement;
        }
    }
    let scene = Scene::new()?;
    let mut records = Vec::new();
    let mut articulation = Vec::new();
    let mut grip_checks = Vec::new();
    let mut items = Vec::new();
    for p in profiles {
        project_rl::monster_art::art(p.number, 0).ok_or("Modele retire du bestiaire")?;
        let previous = old
            .iter()
            .find(|e| e.number == p.number)
            .ok_or("Ancien cycle absent")?;
        let visual = Visual::CampaignMonster {
            number: p.number,
            variant: 0,
        };
        game_art::vertical::prepare(&[visual])?;
        let authored = file(&p.atlas)?;
        articulation.push(serde_json::json!({"id":p.id,"distinct_whole_poses_each_direction":articulation_checks(&authored)?}));
        items.push((
            p.id,
            p.name,
            visual,
            upload(&authored),
            upload(&file(&previous.file)?),
            [f32::from(previous.foot[0]), f32::from(previous.foot[1])],
            previous.vertical_offset,
            p.vertical_offset,
            false,
        ));
    }
    let visual = Visual::Player {
        body: 3,
        equipment: [Some(101), None, None, None],
        weapon: Some(2),
        hands: None,
        palettes: [EquipmentPalette::Original; 3],
    };
    game_art::vertical::prepare(&[visual])?;
    let bodies =
        file("assets/source-art/animation-polish-01/weapon-fit-01/r01/bastion-ready-atlas.png")?;
    let (gun, pivot, support) =
        game_art::vertical_review_weapon(2, false, EquipmentPalette::Original)
            .ok_or("Arme commune absente")?;
    let mut composed = Image::gen_image_color(1152, 256, BLANK);
    let mut previous = Image::gen_image_color(1024, 256, BLANK);
    for side in 0..2 {
        for phase in 0..8 {
            let body = bodies.sub_image(Rect::new(
                phase as f32 * 128.,
                side as f32 * 128.,
                128.,
                128.,
            ));
            let (pose, audit) = rifle_pose(
                &body,
                &gun,
                pivot,
                support.ok_or("Prise de soutien absente")?,
                side,
                phase,
            )?;
            grip_checks.push(audit);
            for y in 0..128 {
                let at = ((side * 128 + y) * 1152 + phase * 128) * 4;
                composed.bytes[at..at + 512].copy_from_slice(&pose.bytes[y * 512..(y + 1) * 512]);
            }
            let old = game_art::polish_review_lateral(
                visual,
                Pose {
                    facing: if side == 0 { -1 } else { 1 },
                    frame: phase.min(5) as u8,
                },
            )
            .ok_or("Marche armee actuelle absente")?;
            for y in 0..128 {
                let at = ((side * 128 + y) * 1024 + phase * 128) * 4;
                previous.bytes[at..at + 512].copy_from_slice(&old.bytes[y * 512..(y + 1) * 512]);
            }
        }
    }
    articulation.push(serde_json::json!({"id":"bastion-rifle","distinct_whole_poses_each_direction":articulation_checks(&composed)?}));
    items.push((
        "bastion-rifle".into(),
        "Bastion · fusil d'assaut".into(),
        visual,
        upload(&composed),
        upload(&previous),
        [64., 96.],
        0,
        0,
        true,
    ));
    for (id, name, visual, new, old, old_anchor, old_offset, offset, player) in items {
        let dir = output.join(&id);
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let frontal = static_render(&scene, visual, 0.);
        let mut distinct = Vec::new();
        let mut captures = Vec::new();
        for (side, facing) in [-1, 1].into_iter().enumerate() {
            let mut unique = BTreeSet::new();
            for sample in 0..=SAMPLES {
                let stop = sample == SAMPLES;
                let phase = phase_at(sample as f32 * 1300. / SAMPLES as f32);
                let shift = -(facing as f32) * (20. - 40. * sample as f32 / SAMPLES as f32);
                let mut board = Image::gen_image_color(480, 192, Color::from_rgba(30, 40, 49, 255));
                panel(&mut board, &frontal, 0);
                if stop {
                    let rest = static_render(&scene, visual, shift);
                    panel(&mut board, &rest, 1);
                    panel(&mut board, &rest, 2);
                } else {
                    // Retain the old six equally timed poses; only its full
                    // cycle duration is slowed to match the candidate.
                    let old_phase = sample * 6 / SAMPLES + usize::from(!player);
                    let before = sprite_render(
                        &scene,
                        &old,
                        Rect::new(old_phase as f32 * 128., side as f32 * 128., 128., 128.),
                        old_anchor,
                        old_offset,
                        shift,
                        player,
                    )?;
                    let after = sprite_render(
                        &scene,
                        &new,
                        Rect::new(phase as f32 * 128., side as f32 * 128., 128., 128.),
                        [64., 112.],
                        offset,
                        shift,
                        player,
                    )?;
                    panel(&mut board, &before, 1);
                    panel(&mut board, &after, 2);
                    // Unique authored articulation, measured with root shift removed.
                    unique.insert(phase);
                }
                let file = format!(
                    "{}-{sample:02}.png",
                    if side == 0 { "left" } else { "right" }
                );
                board.export_png(dir.join(&file).to_str().unwrap());
                captures.push(file);
            }
            distinct.push(unique.len());
            next_frame().await;
        }
        if distinct != [8, 8] {
            return Err("Couverture de poses incomplete".into());
        }
        records.push(serde_json::json!({"id":id,"name":name,"player":player,"captures":captures,"poses_each_direction":distinct,"native_board":[480,192]}));
    }
    let report = serde_json::json!({"review":"BIPED-LATERAL01-r01","status":"native_candidate_awaiting_visual_approval","normal_game_enabled":false,"native_cell_px":64,"step_ms":650,"phase_durations_ms":PHASE_MS,"samples":32,"travel_px":40,"rest_ms":650,"items":records,"articulation_checks":articulation,"grip_checks":grip_checks,"weapon_model":2,"weapon_drawings_shared":true,"weapon_coverage":"Bastion heavy outfit, assault rifle, original palette only","monster_coverage":[5,14,156,144],"vertical_unchanged":true});
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
    fn biped_lateral_actual_sources_and_sixteen_rifle_grips_are_complete() {
        for n in [5, 14, 156, 144] {
            assert!(project_rl::monster_art::art(n, 0).is_some());
            let image = file(&format!(
                "assets/source-art/animation-polish-01/biped-lateral-01/r01/MON-{n:03}-atlas.png"
            ))
            .unwrap();
            assert_eq!(articulation_checks(&image).unwrap(), [8, 8]);
        }
        let body =
            file("assets/source-art/animation-polish-01/weapon-fit-01/r01/bastion-ready-atlas.png")
                .unwrap();
        let (weapon, pivot, support) =
            game_art::vertical_review_weapon(2, false, EquipmentPalette::Original).unwrap();
        for side in 0..2 {
            for phase in 0..8 {
                let frame = body.sub_image(Rect::new(
                    phase as f32 * 128.,
                    side as f32 * 128.,
                    128.,
                    128.,
                ));
                let (out, audit) =
                    rifle_pose(&frame, &weapon, pivot, support.unwrap(), side, phase).unwrap();
                assert_ne!(
                    out.bytes, frame.bytes,
                    "missing independent weapon {side}/{phase}"
                );
                assert!(audit["support_distance_error_px"].as_f64().unwrap() <= 2.);
                for y in 88..128 {
                    assert_eq!(
                        &out.bytes[y * 512..(y + 1) * 512],
                        &frame.bytes[y * 512..(y + 1) * 512],
                        "weapon changed legs {side}/{phase}"
                    );
                }
            }
        }
        assert_eq!(PHASE_MS.iter().sum::<u32>(), 1300);
    }
}
