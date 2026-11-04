//! FIREARM01: compact shared firearms, reviewed at native64 before integration.
use crate::{
    animation_polish_review as review,
    game_art::{self, Visual, walking::Pose},
    world_readability::{self, player_exposure_review::Scene},
};
use macroquad::prelude::*;
use project_rl::entity::EquipmentPalette;
use std::path::Path;

pub(crate) fn clarify_metal(image: &mut Image) {
    let source = image.clone();
    for y in 0..usize::from(image.height) {
        for x in 0..usize::from(image.width) {
            let at = (y * usize::from(image.width) + x) * 4;
            let p = &source.bytes[at..at + 4];
            let low = *p[..3].iter().min().unwrap();
            let high = *p[..3].iter().max().unwrap();
            // Color panels, wood and very dark ink retain their paint. A
            // restrained metal bevel belongs inside the existing silhouette.
            if p[3] < 128 || high - low > 30 || high < 45 || high > 185 {
                continue;
            }
            let upper_edge = y > 0 && source.bytes[at - usize::from(image.width) * 4 + 3] < 96;
            for c in 0..3 {
                let value =
                    (f32::from(p[c]) - 80.) * 1.15 + 90. + if upper_edge { 18. } else { 0. };
                image.bytes[at + c] = value.round().clamp(20., 205.) as u8;
            }
        }
    }
}
struct Cycle {
    texture: Texture2D,
    cell: [u16; 2],
    anchor: [f32; 2],
}
impl Cycle {
    fn new(image: &Image, cell: [u16; 2], anchor: [f32; 2]) -> Self {
        let texture = crate::asset_loading::from_image(image);
        texture.set_filter(FilterMode::Nearest);
        Self {
            texture,
            cell,
            anchor,
        }
    }
    fn draw(&self, rect: Rect, frame: usize) {
        let _player = world_readability::player_scope(true);
        let _actor = world_readability::actor(true);
        game_art::polish_review_shadow(rect);
        let [w, h] = self.cell;
        let scale = rect.w / 64.;
        world_readability::texture(
            &self.texture,
            rect.center().x - self.anchor[0] * scale,
            rect.bottom() - self.anchor[1] * scale,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    (frame % 6) as f32 * f32::from(w),
                    (frame / 6) as f32 * f32::from(h),
                    f32::from(w),
                    f32::from(h),
                )),
                dest_size: Some(vec2(f32::from(w) * scale, f32::from(h) * scale)),
                ..Default::default()
            },
        );
    }
}
struct Figure {
    vertical: Cycle,
    lateral: [Cycle; 2],
}
impl Figure {
    fn prepare(visual: Visual, style: u8) -> Result<Self, String> {
        let p = review::with_source(style, || game_art::vertical::review_image(visual))?;
        let vertical = Cycle::new(&p.image, p.cell, p.anchor);
        let mut lateral = Vec::new();
        for facing in [-1, 1] {
            let mut image = Image::gen_image_color(768, 128, BLANK);
            for frame in 0..6 {
                let im = review::with_source(style, || {
                    game_art::polish_review_lateral(visual, Pose { facing, frame })
                })
                .ok_or("FIREARM01 missing lateral figure")?;
                for y in 0..128usize {
                    let to = (y * 768 + usize::from(frame) * 128) * 4;
                    image.bytes[to..to + 512].copy_from_slice(&im.bytes[y * 512..y * 512 + 512]);
                }
            }
            lateral.push(Cycle::new(&image, [128, 128], [64., 96.]));
        }
        Ok(Self {
            vertical,
            lateral: lateral.try_into().ok().unwrap(),
        })
    }
    fn draw(&self, rect: Rect, facing: i8, phase: u8) {
        if phase == 6 {
            self.vertical.draw(rect, 12);
        } else if facing.abs() == 2 {
            self.vertical
                .draw(rect, usize::from(phase) + if facing < 0 { 6 } else { 0 });
        } else {
            self.lateral[usize::from(facing > 0)].draw(rect, usize::from(phase));
        }
    }
}
fn visual(body: u8, armor: Option<u16>, model: u16) -> Visual {
    Visual::Player {
        body,
        equipment: [armor, None, None, None],
        weapon: Some(model),
        hands: None,
        palettes: [EquipmentPalette::Original; 3],
    }
}
fn bounds(image: &Image) -> [usize; 2] {
    let mut b = [usize::MAX, usize::MAX, 0, 0];
    for (i, p) in image.bytes.chunks_exact(4).enumerate() {
        if p[3] >= 96 {
            let (x, y) = (i % usize::from(image.width), i / usize::from(image.width));
            b = [b[0].min(x), b[1].min(y), b[2].max(x + 1), b[3].max(y + 1)];
        }
    }
    [b[2] - b[0], b[3] - b[1]]
}
fn source_audit() -> Result<Vec<serde_json::Value>, String> {
    let mut records = Vec::new();
    for model in 0..=60 {
        for color in EquipmentPalette::ALL {
            let (before, a, b) =
                review::with_source(0, || game_art::vertical_review_weapon(model, false, color))
                    .ok_or("FIREARM01 missing source")?;
            let (after, aa, bb) =
                review::with_source(4, || game_art::vertical_review_weapon(model, false, color))
                    .ok_or("FIREARM01 missing candidate")?;
            if a != aa || b != bb {
                return Err("FIREARM01 moved authored grips".into());
            }
            let b = b.ok_or("FIREARM01 missing supporting grip")?;
            let mut checked = 0;
            for y in 0..usize::from(before.height) {
                for x in 0..usize::from(before.width) {
                    if (x as f32) >= a[0].min(b[0])
                        && (x as f32) <= a[0].max(b[0])
                        && (y as f32) >= a[1].min(b[1])
                        && (y as f32) <= a[1].max(b[1])
                    {
                        let at = (y * usize::from(before.width) + x) * 4 + 3;
                        if before.bytes[at] != after.bytes[at] {
                            return Err(format!("FIREARM01 changed grip alpha {model} {x} {y}"));
                        }
                        checked += 1;
                    }
                }
            }
            let old = bounds(&before);
            let new = bounds(&after);
            if new[0] > old[0] || new[1] > old[1] {
                return Err(format!(
                    "FIREARM01 expanded weapon {model}: {old:?} -> {new:?}"
                ));
            }
            let restored =
                review::with_source(0, || game_art::vertical_review_weapon(model, false, color))
                    .unwrap()
                    .0;
            if restored.bytes != before.bytes {
                return Err("FIREARM01 contaminated normal source cache".into());
            }
            records.push(serde_json::json!({"model":model,"palette":color.index(),"before_extent":old,"candidate_extent":new,"grip_alpha_checks":checked,"normal_source_restored":true}));
        }
    }
    Ok(records)
}
fn panel(board: &mut Image, image: &Image, column: usize) {
    for y in 0..224usize {
        let from = ((610 - 156 - 224 + y) * 640 + 192) * 4;
        let to = (y * 448 + column * 224) * 4;
        board.bytes[to..to + 896].copy_from_slice(&image.bytes[from..from + 896]);
    }
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(640., 610.);
    for _ in 0..3 {
        next_frame().await;
    }
    let sources = source_audit()?;
    let scene = Scene::new()?;
    let mut subjects = Vec::new();
    for body in [3, 5, 6, 7] {
        for model in [2, 7, 35, 57, 17, 23, 31, 51] {
            subjects.push((body, 3, Some(101), model));
        }
    }
    for (tier, armor) in [(0, None), (1, Some(110)), (2, Some(102)), (4, Some(103))] {
        for model in [2, 31] {
            subjects.push((5, tier, armor, model));
        }
    }
    let mut rows = Vec::new();
    let mut normal_comparisons = 0;
    for (body, tier, armor, model) in subjects {
        let v = visual(body, armor, model);
        game_art::vertical::prepare(&[v])?;
        let before = Figure::prepare(v, 0)?;
        let candidate = Figure::prepare(v, 4)?;
        for (direction, facing) in [("front", 2), ("back", -2), ("left", -1), ("right", 1)] {
            for phase in 0..7 {
                let (a, _) = scene.render_actor(|cell| before.draw(cell, facing, phase));
                if !crate::rifle_visuals::rifle(model)
                    && (phase == 3 || (facing == 2 && phase == 6))
                {
                    let pose = (phase < 6).then_some(Pose {
                        facing,
                        frame: phase,
                    });
                    let (normal, _) = scene.render_actor(|cell| {
                        game_art::walking::with_pose(pose, || {
                            game_art::draw_world_animated(cell, v, 1., None)
                        })
                    });
                    if a.bytes != normal.bytes {
                        a.export_png(output.join("baseline-expected.png").to_str().unwrap());
                        normal.export_png(output.join("baseline-normal.png").to_str().unwrap());
                        return Err(format!(
                            "FIREARM01 comparator differs from normal renderer: {body} {tier} {model} {direction} {phase}"
                        ));
                    }
                    normal_comparisons += 1;
                }
                let (b, audit) = scene.render_actor(|cell| candidate.draw(cell, facing, phase));
                let (_, _, changed) = world_readability::player_exposure_review::compare(&a, &b)?;
                if audit.is_empty()
                    || audit
                        .iter()
                        .any(|a| !a.actor || !a.world || (a.gain - 1.18).abs() > 0.0001)
                {
                    return Err("FIREARM01 wrong player exposure".into());
                }
                let mut board = Image::gen_image_color(448, 256, Color::from_rgba(30, 40, 49, 255));
                panel(&mut board, &a, 0);
                panel(&mut board, &b, 1);
                board.export_png(
                    output
                        .join(format!(
                            "firearm-{body}-{tier}-{model}-{direction}-{phase}.png"
                        ))
                        .to_str()
                        .unwrap(),
                );
                rows.push(serde_json::json!({"body":body,"armor_stage":tier,"model":model,"direction":direction,"phase":phase,"changed_pixels":changed,"scenery_unchanged":true,"player_exposure":1.18}));
            }
        }
        let (a, _) = scene.render_actor(|cell| candidate.draw(cell, 2, 6));
        let _ = scene.render_actor(|cell| candidate.draw(cell, -1, 3));
        let (b, _) = scene.render_actor(|cell| candidate.draw(cell, 2, 6));
        if a.bytes != b.bytes {
            return Err("FIREARM01 changed rest after walking".into());
        }
        eprintln!(
            "[FIREARM01] body{body} armor{tier} weapon{model}: native comparison prepared, stable rest"
        );
        next_frame().await;
    }
    let report = serde_json::json!({"review":"FIREARM01-r01","normal_game_enabled":false,"native_cell_px":64,"boards":rows.len(),"classes":4,"main_armor_stage":3,"additional_armor_stages":[0,1,2,4],"sample_models":[2,7,35,57,17,23,31,51],"source_model_count":61,"source_palette_checks":sources.len(),"source_records":sources,"normal_renderer_pixel_comparisons":normal_comparisons,"rest_return_checks":40,"records":rows});
    std::fs::write(
        output.join("checks.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub(crate) async fn capture_rifles(output: &Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(640., 610.);
    for _ in 0..3 {
        next_frame().await;
    }
    let mut sources = Vec::new();
    let mut unaffected = 0;
    for model in 0..=60 {
        for color in EquipmentPalette::ALL {
            let (normal, _, _) =
                review::with_source(0, || game_art::vertical_review_weapon(model, false, color))
                    .unwrap();
            let (after, a, b) =
                review::with_source(5, || game_art::vertical_review_weapon(model, false, color))
                    .unwrap();
            let production = game_art::vertical_review_weapon(model, false, color)
                .unwrap()
                .0;
            if production.bytes != after.bytes {
                return Err(format!(
                    "FIREARM02 normal source differs from accepted source: {model}"
                ));
            }
            if !review::rifle(model) {
                if after.bytes != normal.bytes {
                    return Err("FIREARM02 changed an unrelated firearm".into());
                }
                unaffected += 1;
                continue;
            }
            let (before, aa, bb) =
                review::with_source(4, || game_art::vertical_review_weapon(model, false, color))
                    .unwrap();
            if a != aa || b != bb {
                return Err("FIREARM02 moved a hand anchor".into());
            }
            let b = b.unwrap();
            for y in 0..usize::from(normal.height) {
                for x in 0..usize::from(normal.width) {
                    if (x as f32) >= a[0].min(b[0])
                        && (x as f32) <= a[0].max(b[0])
                        && (y as f32) >= a[1].min(b[1])
                        && (y as f32) <= a[1].max(b[1])
                    {
                        let at = (y * usize::from(normal.width) + x) * 4 + 3;
                        if normal.bytes[at] != after.bytes[at] {
                            return Err("FIREARM02 changed grip alpha".into());
                        }
                    }
                }
            }
            let old = bounds(&before);
            let new = bounds(&after);
            if new[0] < old[0] || new[1] < old[1] {
                return Err("FIREARM02 unexpectedly shrank a rifle".into());
            }
            let restored =
                review::with_source(0, || game_art::vertical_review_weapon(model, false, color))
                    .unwrap()
                    .0;
            if restored.bytes != normal.bytes {
                return Err("FIREARM02 contaminated normal source cache".into());
            }
            sources.push(serde_json::json!({"model":model,"palette":color.index(),"previous_extent":old,"candidate_extent":new,"grips_preserved":true,"normal_source_restored":true}));
        }
    }
    let scene = Scene::new()?;
    let mut subjects = Vec::new();
    for body in [3, 5, 6, 7] {
        for model in [2, 7, 35, 57] {
            subjects.push((body, 3, Some(101), model));
        }
        for (tier, armor) in [(1, Some(110)), (4, Some(103))] {
            for model in [2, 57] {
                subjects.push((body, tier, armor, model));
            }
        }
    }
    let mut rows = Vec::new();
    let mut normal_matches = 0;
    let mut lateral_cycles = Vec::new();
    for (body, tier, armor, model) in subjects {
        let v = visual(body, armor, model);
        game_art::vertical::prepare(&[v])?;
        let previous = Figure::prepare(v, 4)?;
        let candidate = Figure::prepare(v, 5)?;
        for (direction, facing) in [("front", 2), ("back", -2), ("left", -1), ("right", 1)] {
            let mut normal_frames = Vec::new();
            let mut lower_frames = Vec::new();
            for phase in 0..7 {
                let (a, _) = scene.render_actor(|cell| previous.draw(cell, facing, phase));
                let (b, audit) = scene.render_actor(|cell| candidate.draw(cell, facing, phase));
                let pose = (phase < 6).then_some(Pose {
                    facing,
                    frame: phase,
                });
                let (normal, _) = scene.render_actor(|cell| {
                    game_art::walking::with_pose(pose, || {
                        game_art::draw_world_animated(cell, v, 1., None)
                    })
                });
                if normal.bytes != b.bytes {
                    normal.export_png(output.join("normal-mismatch.png").to_str().unwrap());
                    b.export_png(output.join("approved-mismatch.png").to_str().unwrap());
                    return Err(format!(
                        "FIREARM02 normal rendering differs from accepted candidate: {body} {tier} {model} {direction} {phase}"
                    ));
                }
                normal_matches += 1;
                if facing.abs() == 1 && phase < 6 {
                    let mut lower = Vec::new();
                    for y in 290..348usize {
                        let at = ((609 - y) * 640 + 208) * 4;
                        lower.extend_from_slice(&normal.bytes[at..at + 192 * 4]);
                    }
                    lower_frames.push(lower);
                    normal_frames.push(normal.bytes);
                }
                let (_, _, changed) = world_readability::player_exposure_review::compare(&a, &b)?;
                if audit.is_empty()
                    || audit
                        .iter()
                        .any(|a| !a.actor || !a.world || (a.gain - 1.18).abs() > 0.0001)
                {
                    return Err("FIREARM02 wrong player exposure".into());
                }
                let mut board = Image::gen_image_color(448, 256, Color::from_rgba(30, 40, 49, 255));
                panel(&mut board, &a, 0);
                panel(&mut board, &b, 1);
                board.export_png(
                    output
                        .join(format!(
                            "rifle-{body}-{tier}-{model}-{direction}-{phase}.png"
                        ))
                        .to_str()
                        .unwrap(),
                );
                rows.push(serde_json::json!({"body":body,"armor_stage":tier,"model":model,"direction":direction,"phase":phase,"changed_pixels":changed,"scenery_unchanged":true,"player_exposure":1.18}));
            }
            if facing.abs() == 1 {
                let unique = normal_frames
                    .iter()
                    .collect::<std::collections::BTreeSet<_>>()
                    .len();
                let lower_unique = lower_frames
                    .iter()
                    .collect::<std::collections::BTreeSet<_>>()
                    .len();
                if unique != 6 || lower_unique < 3 {
                    return Err(format!(
                        "FIREARM02 frozen actual lateral walk: {body} {tier} {model} {direction}, unique {unique}, legs {lower_unique}"
                    ));
                }
                lateral_cycles.push(serde_json::json!({"body":body,"armor_stage":tier,"model":model,"direction":direction,"distinct_frames":unique,"distinct_lower_body_frames":lower_unique}));
            }
        }
        let (a, _) = scene.render_actor(|cell| candidate.draw(cell, 2, 6));
        let _ = scene.render_actor(|cell| candidate.draw(cell, -1, 3));
        let (b, _) = scene.render_actor(|cell| candidate.draw(cell, 2, 6));
        if a.bytes != b.bytes {
            return Err("FIREARM02 changed rest after walking".into());
        }
        eprintln!("[FIREARM02] body{body} armor{tier} rifle{model}: slightly larger, stable rest");
        next_frame().await;
    }
    let unaffected_world = check_unaffected_world(&scene)?;
    let portrait_matches = check_portraits()?;
    let report = serde_json::json!({"review":"FIREARM02-r01","normal_game_enabled":true,"normal_vs_approved_pixel_comparisons":normal_matches,"normal_source_comparisons":427,"unaffected_world_comparisons":unaffected_world,"portrait_pixel_comparisons":portrait_matches,"actual_lateral_cycle_checks":lateral_cycles.len(),"lateral_cycles":lateral_cycles,"native_cell_px":64,"boards":rows.len(),"classes":4,"sample_models":[2,7,35,57],"armor_stages":[1,3,4],"source_model_count":32,"source_palette_checks":sources.len(),"unrelated_source_comparisons":unaffected,"source_records":sources,"rest_return_checks":32,"records":rows});
    std::fs::write(
        output.join("checks.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
fn check_unaffected_world(scene: &Scene) -> Result<usize, String> {
    let samples = [
        Visual::CampNpc(1),
        Visual::ArmedActor {
            body: 20,
            weapon: 2,
        },
        Visual::ArmedActor {
            body: 24,
            weapon: 57,
        },
        Visual::CampaignMonster {
            number: 120,
            variant: 0,
        },
        Visual::ColoredItem(
            game_art::item("core:fusil_d_assaut").unwrap(),
            EquipmentPalette::Original,
        ),
        Visual::ColoredItem(
            game_art::item("core:mitrailleuse_lourde").unwrap(),
            EquipmentPalette::Original,
        ),
    ];
    game_art::vertical::prepare(&samples)?;
    let mut count = 0;
    for visual in samples {
        for pose in [
            None,
            Some(Pose {
                facing: -1,
                frame: 3,
            }),
            Some(Pose {
                facing: 1,
                frame: 3,
            }),
        ] {
            let draw = |cell| {
                game_art::walking::with_pose(pose, || {
                    game_art::draw_world_animated(cell, visual, 1., None)
                })
            };
            let (before, _) = scene.render_actor(|cell| {
                let _old = crate::rifle_visuals::original_scope(true);
                draw(cell)
            });
            let (after, _) = scene.render_actor(draw);
            if before.bytes != after.bytes {
                return Err(format!(
                    "FIREARM02 changed unrelated actor/item: {visual:?} {pose:?}"
                ));
            }
            count += 1;
        }
    }
    Ok(count)
}
fn check_portraits() -> Result<usize, String> {
    let rt = render_target(256, 256);
    let mut checks = 0;
    for body in [3, 5, 6, 7] {
        for model in [2, 57] {
            let v = visual(body, Some(101), model);
            let mut images = Vec::new();
            for mode in 0..3 {
                let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., 256., 256.));
                camera.render_target = Some(rt.clone());
                set_camera(&camera);
                clear_background(BLACK);
                let draw = || {
                    let _color = world_readability::color();
                    game_art::draw_portrait(Rect::new(64., 64., 128., 128.), v);
                };
                match mode {
                    0 => {
                        let _old = crate::rifle_visuals::original_scope(true);
                        draw();
                    }
                    1 => review::with_source(5, draw),
                    _ => draw(),
                }
                unsafe {
                    get_internal_gl().flush();
                }
                images.push(rt.texture.get_texture_data().bytes);
            }
            if images[0] == images[1] || images[1] != images[2] {
                return Err(format!("FIREARM02 portrait/cache mismatch: {body} {model}"));
            }
            checks += 1;
        }
    }
    set_default_camera();
    Ok(checks)
}
