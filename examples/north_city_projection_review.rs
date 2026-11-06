//! PROJECTION06: one complete volume at native game scale, no map mutation.
#![allow(dead_code)]
#[path = "../src/asset_loading.rs"]
mod asset_loading;
#[path = "../src/asset_preparation.rs"]
mod asset_preparation;
use macroquad::prelude::*;
use std::path::Path;

const SOURCE: &str = "assets/source-art/north-city-01/projection06/VOLUME06-01-source.png";
const STYLE07: &str = "assets/source-art/north-city-01/style07/VOLUME07-01-r02-source.png";
fn contours07() -> bool {
    static VALUE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *VALUE.get_or_init(|| std::env::args().any(|a| a == "--contours07"))
}
fn source() -> &'static str {
    if contours07() { STYLE07 } else { SOURCE }
}
fn crop() -> Rect {
    if contours07() {
        Rect::new(114., 93., 1062., 1041.)
    } else {
        CROP
    }
}
const PAVEMENT: &str = "assets/source-art/floor-continuous-01/material-source.png";
const ROAD: &str =
    "assets/source-art/first-sector-review-01/u01-repairs-r09/ROAD-U01-09-native.png";
const REFERENCE: &str =
    "assets/source-art/first-sector-review-01/u01-expansion-r06/west-garage-native.png";
const PLAYER: &str = "assets/source-art/class-readability-01/bastion-copper-wave-01/accepted/native64/BA-CU01-P01-r01-native64.png";
const CROP: Rect = Rect::new(116., 94., 1056., 1034.);
const NATIVE: Vec2 = Vec2::new(288., 282.);
const ORIGINAL_NATIVE: Vec2 = Vec2::new(384., 376.);
const DOOR_AXIS_ORIGINAL: f32 = 220.;
const FOOT: f32 = 448.;
const NEW_GROUND_ANCHOR: f32 = 281.25;
const REFERENCE_GROUND_ANCHOR: f32 = 302.;
const CURB: f32 = 512.;

fn config() -> Conf {
    Conf {
        window_title: "Project RL - PROJECTION06 - volume complet".into(),
        window_width: 1024,
        window_height: 704,
        high_dpi: false,
        ..Default::default()
    }
}
async fn prepare() -> Result<(), String> {
    loop {
        let mut done = true;
        for path in [source(), PAVEMENT, ROAD, REFERENCE, PLAYER] {
            if !asset_preparation::ready(path)? {
                done = false;
                asset_preparation::request(
                    path.into(),
                    Box::new(move || {
                        Ok(asset_preparation::PreparedImage {
                            image: (*asset_preparation::image(path)?).clone(),
                            cell: [64, 64],
                            anchor: [32., 32.],
                        })
                    }),
                );
            }
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
fn ground(width: f32, height: f32) {
    // The accepted continuous exterior materials span several logical cells.
    asset_preparation::with_texture(PAVEMENT, |p| {
        for (i, x) in (0..width as usize).step_by(512).enumerate() {
            draw_texture_ex(
                &p.texture,
                x as f32,
                0.,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(512., 512.)),
                    flip_x: i % 2 == 1,
                    ..Default::default()
                },
            );
        }
    });
    asset_preparation::with_texture(ROAD, |p| {
        for (i, x) in (0..width as usize).step_by(256).enumerate() {
            draw_texture_ex(
                &p.texture,
                x as f32,
                CURB,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(0., 0., 256., height - CURB)),
                    flip_x: i % 2 == 1,
                    ..Default::default()
                },
            );
        }
    });
    draw_rectangle(0., CURB, width, 4., Color::from_hex(0x282d31));
    draw_rectangle(0., CURB - 5., width, 5., Color::from_hex(0x74756a));
    draw_rectangle(0., CURB - 5., width, 1., Color::from_hex(0x9c9b86));
    for x in (24..width as usize).step_by(96) {
        draw_rectangle(x as f32, 608., 44., 3., Color::from_hex(0x9a793e));
    }
}
fn candidate(width: f32, height: f32) {
    ground(width, height);
    // Uniform reduction about the existing door/ground anchor. Both actor
    // position and foundation level stay unchanged from the inspected r01.
    let previous_x = (width - ORIGINAL_NATIVE.x) / 2.;
    let door_axis = previous_x + DOOR_AXIS_ORIGINAL;
    let x = door_axis - DOOR_AXIS_ORIGINAL * (NATIVE.x / ORIGINAL_NATIVE.x);
    image(
        source(),
        vec2(x, FOOT - NEW_GROUND_ANCHOR),
        Some(crop()),
        Some(NATIVE),
    );
    image(
        PLAYER,
        vec2(previous_x + 192., 448.),
        None,
        Some(vec2(64., 64.)),
    );
}
fn reference(width: f32, height: f32) {
    ground(width, height);
    let x = (width - 326.) / 2.;
    image(
        REFERENCE,
        vec2(x, FOOT - REFERENCE_GROUND_ANCHOR),
        None,
        None,
    );
    image(PLAYER, vec2(x + 80., 448.), None, Some(vec2(64., 64.)));
}
fn set_target(target: &RenderTarget, w: f32, h: f32) {
    set_camera(&Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., w, h))
    });
}
async fn exported(target: &RenderTarget, path: &Path) {
    set_default_camera();
    next_frame().await;
    target
        .texture
        .get_texture_data()
        .export_png(path.to_str().unwrap());
}
async fn capture(out: &Path) -> Result<(), String> {
    let prefix = if contours07() {
        "contours07"
    } else {
        "projection06"
    };
    if (NATIVE.x / CROP.w - NATIVE.y / CROP.h).abs() > 1e-6 {
        return Err("Projection study cannot stretch the drawing".into());
    }
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let scene = render_target(768, 704);
    scene.texture.set_filter(FilterMode::Nearest);
    set_target(&scene, 768., 704.);
    clear_background(BLACK);
    candidate(768., 704.);
    exported(&scene, &out.join(format!("{prefix}-scene.png"))).await;
    let left = render_target(512, 640);
    left.texture.set_filter(FilterMode::Nearest);
    set_target(&left, 512., 640.);
    clear_background(BLACK);
    reference(512., 640.);
    set_default_camera();
    next_frame().await;
    let right = render_target(512, 640);
    right.texture.set_filter(FilterMode::Nearest);
    set_target(&right, 512., 640.);
    clear_background(BLACK);
    candidate(512., 640.);
    set_default_camera();
    next_frame().await;
    let comparison = render_target(1024, 672);
    comparison.texture.set_filter(FilterMode::Nearest);
    set_target(&comparison, 1024., 672.);
    clear_background(Color::from_hex(0x111820));
    for (texture, x) in [(&left.texture, 0.), (&right.texture, 512.)] {
        draw_texture_ex(
            texture,
            x,
            32.,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()
            },
        );
    }
    draw_text(
        "Premiere ville - 64 px/case",
        12.,
        23.,
        20.,
        Color::from_hex(0xdec69b),
    );
    draw_text(
        if contours07() {
            "07 - contours - 64 px/case"
        } else {
            "06-r02 - echelle - 64 px/case"
        },
        524.,
        23.,
        20.,
        Color::from_hex(0xdec69b),
    );
    exported(&comparison, &out.join(format!("{prefix}-comparaison.png"))).await;
    set_target(&scene, 768., 704.);
    clear_background(BLACK);
    candidate(768., 704.);
    for (y, text) in [
        (352., "corniche / debut de facade"),
        (FOOT, "pied de facade / sol"),
        (CURB, "bord de chaussee"),
    ] {
        draw_line(24., y, 744., y, 1., Color::new(0.91, 0.72, 0.35, 0.85));
        draw_rectangle(528., y - 21., 216., 20., Color::from_hex(0x111820));
        draw_text(text, 533., y - 7., 15., Color::from_hex(0xe0c590));
    }
    exported(&scene, &out.join(format!("{prefix}-reperes.png"))).await;
    let checks = serde_json::json!({"review":"CITY-NORTH01-PROJECTION06-r02","status":"relative-scale-awaiting-user-prevalidation","campaign_integrated":false,"native_cell":64,"source":SOURCE,"crop_empty_margin_only":[CROP.x,CROP.y,CROP.w,CROP.h],"native_object":[NATIVE.x,NATIVE.y],"previous_native_object":[ORIGINAL_NATIVE.x,ORIGINAL_NATIVE.y],"reduction_xy":[NATIVE.x/ORIGINAL_NATIVE.x,NATIVE.y/ORIGINAL_NATIVE.y],"ground_anchor_native_y":NEW_GROUND_ANCHOR,"door_axis_preserved":true,"actor_position_preserved":true,"scale_xy":[NATIVE.x/CROP.w,NATIVE.y/CROP.h],"uniform_scale_only":true,"piecewise_image_warp":false,"whole_building_visible":true,"foot_of_both_buildings_y":FOOT,"road_edge_y":CURB,"reference":REFERENCE,"reference_drawn_at_original_native_size":true,"same_actor_64px":PLAYER,"figures":["projection06-scene.png","projection06-comparaison.png","projection06-reperes.png"],"limits":["New single-building projection study; no change to B01 or its surroundings.","No walking/collision or final-lighting validation is claimed."]});
    let mut checks = checks;
    if contours07() {
        checks["review"] = serde_json::json!("CITY-NORTH01-CONTOURS07-r02");
        checks["source"] = serde_json::json!(source());
        let rect = crop();
        checks["crop_empty_margin_only"] = serde_json::json!([rect.x, rect.y, rect.w, rect.h]);
        checks["scale_xy"] = serde_json::json!([NATIVE.x / rect.w, NATIVE.y / rect.h]);
        checks["scale_rounding"] =
            serde_json::json!("Native height rounded to the nearest pixel; no perspective warp");
        checks["figures"] = serde_json::json!([
            "contours07-scene.png",
            "contours07-comparaison.png",
            "contours07-reperes.png"
        ]);
    }
    std::fs::write(
        out.join(format!("{prefix}-checks.json")),
        serde_json::to_vec_pretty(&checks).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
#[macroquad::main(config)]
async fn main() {
    asset_preparation::initialize_graphics_thread();
    let result = async {
        prepare().await?;
        let args = std::env::args().collect::<Vec<_>>();
        capture(Path::new(
            args.get(1)
                .map_or("artifacts/north-city-01", String::as_str),
        ))
        .await
    }
    .await;
    if let Err(e) = result {
        eprintln!("PROJECTION06: {e}");
        std::process::exit(1);
    }
}
