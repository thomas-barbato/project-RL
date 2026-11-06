//! First native building production pass on the accepted PLAN09 footprint.
//! This visual review does not change campaign terrain or player saves.
#![allow(dead_code)]
#[path = "../src/asset_loading.rs"]
mod asset_loading;
#[path = "../src/asset_preparation.rs"]
mod asset_preparation;
use macroquad::prelude::*;
use serde::Deserialize;
use std::{path::Path, sync::OnceLock};
const PLAYER: &str = "assets/source-art/class-readability-01/bastion-copper-wave-01/accepted/native64/BA-CU01-P01-r01-native64.png";
const REFERENCE: &str =
    "assets/source-art/first-sector-review-01/u01-expansion-r06/west-garage-native.png";
const ORIGINAL: &str = "assets/source-art/north-city-01/production10/B01-closed-r01-source.png";
const PAVE: &str = "assets/source-art/floor-continuous-01/material-source.png";
const ROAD: &str =
    "assets/source-art/first-sector-review-01/u01-repairs-r09/ROAD-U01-09-native.png";
const GROUND: &str = "north-city-production10:ground";
const W: i32 = 1024;
const H: i32 = 768;
const LEFT: i32 = 544;
const TOP: i32 = 5672;
#[derive(Deserialize)]
struct Spec {
    review: String,
    label: String,
    status: String,
    source: String,
    crop: [f32; 4],
    native: [f32; 2],
    rect: [i32; 4],
    approach: [i32; 2],
    props: Vec<Prop>,
}
#[derive(Deserialize)]
struct Prop {
    file: String,
    at: [f32; 2],
}
#[derive(Clone)]
struct Segment {
    a: [f32; 2],
    b: [f32; 2],
    half: f32,
}
struct Data {
    spec: Spec,
    segments: Vec<Segment>,
}
static DATA: OnceLock<Data> = OnceLock::new();
fn data() -> &'static Data {
    DATA.get().expect("data prepared before display")
}
fn review_status() -> &'static str {
    if data().spec.status == "artwork-prevalidated" {
        "dessin pre-valide"
    } else {
        "candidat"
    }
}
fn config() -> Conf {
    Conf {
        window_title: "Project RL - B01 - production 10 sur PLAN09".into(),
        window_width: 1040,
        window_height: 816,
        high_dpi: false,
        ..Default::default()
    }
}
fn load() -> Result<(), String> {
    let bytes =
        project_rl::data_paths::read("assets/source-art/north-city-01/production10/B01.json")?;
    let spec: Spec = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let bytes = project_rl::data_paths::read("assets/maps/north-city-01/city-plan09.json")?;
    let plan: serde_json::Value = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    let building = plan["buildings"]
        .as_array()
        .ok_or("buildings missing")?
        .iter()
        .find(|b| b["id"] == "B01")
        .ok_or("B01 missing")?;
    if serde_json::from_value::<Vec<[i32; 4]>>(building["rects"].clone())
        .map_err(|e| e.to_string())?
        != vec![spec.rect]
        || serde_json::from_value::<[i32; 2]>(building["door"].clone())
            .map_err(|e| e.to_string())?
            != spec.approach
    {
        return Err("B01 artwork no longer matches the accepted plan".into());
    }
    let mut segments = vec![];
    for road in plan["roads"].as_array().ok_or("roads missing")? {
        if road["kind"] == "path" {
            continue;
        }
        let half = road["width"].as_f64().ok_or("road width")? as f32 * 32.;
        let points: Vec<[f32; 2]> =
            serde_json::from_value(road["points"].clone()).map_err(|e| e.to_string())?;
        for p in points.windows(2) {
            let a = p[0].map(|v| (v + 0.5) * 64.);
            let b = p[1].map(|v| (v + 0.5) * 64.);
            let extra = half + 80.;
            if a[0].min(b[0]) - extra < (LEFT + W) as f32
                && a[0].max(b[0]) + extra > LEFT as f32
                && a[1].min(b[1]) - extra < (TOP + H) as f32
                && a[1].max(b[1]) + extra > TOP as f32
            {
                segments.push(Segment { a, b, half })
            }
        }
    }
    DATA.set(Data { spec, segments })
        .map_err(|_| "already prepared".to_string())?;
    Ok(())
}
fn mirror(v: i32, n: i32) -> i32 {
    let a = v.rem_euclid(n * 2);
    if a < n { a } else { n * 2 - 1 - a }
}
fn pixel(i: &Image, x: i32, y: i32) -> [u8; 4] {
    let off = ((y as usize * i.width as usize + x as usize) * 4) as usize;
    i.bytes[off..off + 4].try_into().unwrap()
}
fn ground(segments: &[Segment]) -> Result<asset_preparation::PreparedImage, String> {
    let pave = asset_preparation::image(PAVE)?;
    let road = asset_preparation::image(ROAD)?;
    let mut out = Image {
        width: W as u16,
        height: H as u16,
        bytes: vec![0; (W * H * 4) as usize],
    };
    for y in 0..H {
        for x in 0..W {
            let wx = x + LEFT;
            let wy = y + TOP;
            let mut edge = f32::INFINITY;
            let mut along = 0.;
            for s in segments {
                let dx = s.b[0] - s.a[0];
                let dy = s.b[1] - s.a[1];
                let len = dx * dx + dy * dy;
                let t = if len > 0. {
                    (((wx as f32 - s.a[0]) * dx + (wy as f32 - s.a[1]) * dy) / len).clamp(0., 1.)
                } else {
                    0.
                };
                let dist = ((wx as f32 - s.a[0] - t * dx).powi(2)
                    + (wy as f32 - s.a[1] - t * dy).powi(2))
                .sqrt()
                    - s.half;
                if dist < edge {
                    edge = dist;
                    along = t * len.sqrt()
                }
            }
            let p = if edge < 0. {
                pixel(&road, mirror(wx, 256), mirror(wy, 256))
            } else if edge < 3. {
                [30, 35, 37, 255]
            } else if edge < 16. {
                if along.rem_euclid(52.) < 2. {
                    [45, 48, 46, 255]
                } else if edge > 13. {
                    [174, 170, 145, 255]
                } else {
                    [117, 119, 106, 255]
                }
            } else {
                pixel(
                    &pave,
                    mirror(wx, 512) * pave.width as i32 / 512,
                    mirror(wy, 512) * pave.height as i32 / 512,
                )
            };
            out.bytes[((y * W + x) * 4) as usize..][..4].copy_from_slice(&p);
        }
    }
    Ok(asset_preparation::PreparedImage {
        image: out,
        cell: [64, 64],
        anchor: [32., 32.],
    })
}
async fn prepare() -> Result<(), String> {
    loop {
        let mut done = true;
        let paths = [
            PLAYER,
            REFERENCE,
            ORIGINAL,
            PAVE,
            ROAD,
            data().spec.source.as_str(),
        ]
        .into_iter()
        .chain(data().spec.props.iter().map(|p| p.file.as_str()));
        for path in paths {
            if !asset_preparation::ready(path)? {
                done = false;
                let p = path.to_string();
                asset_preparation::request(
                    p.clone(),
                    Box::new(move || {
                        Ok(asset_preparation::PreparedImage {
                            image: (*asset_preparation::image(&p)?).clone(),
                            cell: [64, 64],
                            anchor: [32., 32.],
                        })
                    }),
                );
            }
        }
        if !asset_preparation::ready(GROUND)? {
            done = false;
            let segments = data().segments.clone();
            asset_preparation::request(GROUND.into(), Box::new(move || ground(&segments)));
        }
        asset_preparation::pump();
        if done {
            return Ok(());
        }
        clear_background(BLACK);
        next_frame().await;
    }
}
fn image(path: &str, at: Vec2, source: Option<Rect>, size: Option<Vec2>) {
    asset_preparation::with_texture(path, |p| {
        draw_texture_ex(
            &p.texture,
            at.x,
            at.y,
            WHITE,
            DrawTextureParams {
                source,
                dest_size: size,
                ..Default::default()
            },
        )
    });
}
fn scene() {
    let s = &data().spec;
    scene_with_building(&s.source, s.crop, s.native);
}
fn scene_with_building(source: &str, crop: [f32; 4], native: [f32; 2]) {
    let s = &data().spec;
    image(GROUND, Vec2::ZERO, None, None);
    for p in &s.props {
        image(
            &p.file,
            vec2(p.at[0] - LEFT as f32, p.at[1] - TOP as f32),
            None,
            None,
        )
    }
    let [l, _, _, b] = s.rect;
    image(
        source,
        vec2(
            (l * 64 - LEFT) as f32,
            ((b + 1) * 64 - TOP) as f32 - native[1],
        ),
        Some(Rect::new(crop[0], crop[1], crop[2], crop[3])),
        Some(vec2(native[0], native[1])),
    );
    image(
        PLAYER,
        vec2(
            (s.approach[0] * 64 - LEFT) as f32,
            (s.approach[1] * 64 - TOP) as f32,
        ),
        None,
        Some(vec2(64., 64.)),
    );
}
fn title(text: &str, width: f32) {
    draw_rectangle(0., 0., width, 32., Color::from_hex(0x111820));
    draw_text(text, 16., 23., 20., Color::from_hex(0xe1c593));
}
fn target(w: u32, h: u32) -> RenderTarget {
    let t = render_target(w, h);
    t.texture.set_filter(FilterMode::Nearest);
    set_camera(&Camera2D {
        render_target: Some(t.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., w as f32, h as f32))
    });
    clear_background(BLACK);
    t
}
async fn export(t: &RenderTarget, out: &Path, name: &str) -> Result<(), String> {
    set_default_camera();
    next_frame().await;
    t.texture
        .get_texture_data()
        .export_png(out.join(name).to_str().ok_or("capture path")?);
    Ok(())
}
async fn capture(out: &Path) -> Result<(), String> {
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let t = target(1024, 768);
    scene();
    title(
        &format!(
            "{} - B01 - 64 px/case - {}",
            data().spec.label,
            review_status()
        ),
        1024.,
    );
    export(&t, out, "production10-B01-native.png").await?;
    let original = target(1024, 768);
    scene_with_building(
        ORIGINAL,
        [119., 73., 1115., 968.],
        [576., 968. * 576. / 1115.],
    );
    title("B01 r01 - image signalee par le joueur - natif", 1024.);
    export(&original, out, "production10-B01-original.png").await?;
    let before_after = target(2048, 800);
    for (tex, x) in [(&original.texture, 0.), (&t.texture, 1024.)] {
        draw_texture_ex(
            tex,
            x,
            32.,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()
            },
        );
    }
    title(
        "B01 r01 a gauche / candidat futuriste a droite - meme plan, meme acteur natif",
        2048.,
    );
    export(&before_after, out, "production10-B01-avant-apres.png").await?;
    let c = target(1024, 768);
    scene();
    let [l, t, r, b] = data().spec.rect;
    let (x, y, w, h) = (
        (l * 64 - LEFT) as f32,
        (t * 64 - TOP) as f32,
        ((r - l + 1) * 64) as f32,
        ((b - t + 1) * 64) as f32,
    );
    draw_rectangle_lines(x, y, w, h, 1., Color::from_hex(0xe1c593));
    title("10-1 - B01 - repere de l'emprise au sol", 1024.);
    export(&c, out, "production10-B01-emprise.png").await?;
    let left = target(512, 768);
    image(GROUND, Vec2::ZERO, None, None);
    let baseline = ((data().spec.rect[3] + 1) * 64 - TOP) as f32;
    image(REFERENCE, vec2(90., baseline - 302.), None, None);
    image(PLAYER, vec2(250., baseline), None, None);
    set_default_camera();
    next_frame().await;
    let right = target(1024, 768);
    scene();
    set_default_camera();
    next_frame().await;
    let compare = target(1536, 800);
    for (tex, x) in [(&left.texture, 0.), (&right.texture, 512.)] {
        draw_texture_ex(
            tex,
            x,
            32.,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()
            },
        )
    }
    title("Premiere ville - natif", 512.);
    draw_text(
        &format!("{} B01 - meme acteur 64 px", data().spec.label),
        528.,
        23.,
        20.,
        Color::from_hex(0xe1c593),
    );
    export(&compare, out, "production10-B01-comparaison.png").await?;
    let mut checks = serde_json::json!({"review":data().spec.review,"building":"B01","status":data().spec.status,"rect":data().spec.rect,"approach":data().spec.approach,"native":data().spec.native,"actor":PLAYER,"actor_size":[64,64],"world_baseline":(data().spec.rect[3]+1)*64,"plan_geometry_unchanged":true,"campaign_integrated":false,"scope":"Closed building perspective and futuristic grimdark direction review on PLAN09 coordinates; no interior or collision validation claimed."});
    #[cfg(debug_assertions)]
    {
        checks["resource_cache"] = asset_preparation::stats();
    }
    std::fs::write(
        out.join("production10-checks.json"),
        serde_json::to_vec_pretty(&checks).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
#[macroquad::main(config)]
async fn main() {
    asset_preparation::initialize_graphics_thread();
    let result = async {
        load()?;
        prepare().await?;
        let args = std::env::args().collect::<Vec<_>>();
        if let Some(i) = args.iter().position(|a| a == "--capture") {
            return capture(Path::new(args.get(i + 1).ok_or("output missing")?)).await;
        }
        loop {
            if is_key_pressed(KeyCode::Escape) {
                break;
            }
            clear_background(BLACK);
            scene();
            title(
                &format!("B01 - production sur PLAN09 - {}", review_status()),
                screen_width(),
            );
            next_frame().await;
        }
        Ok(())
    }
    .await;
    if let Err(e) = result {
        eprintln!("PRODUCTION10: {e}");
        std::process::exit(1)
    }
}
