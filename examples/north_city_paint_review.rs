//! PAINT05: isolated native art review; no campaign, map or save changes.
//! cargo run --locked --example north_city_paint_review -- artifacts/north-city-01
#![allow(dead_code)]
#[path = "../src/asset_loading.rs"]
mod asset_loading;
#[path = "../src/asset_preparation.rs"]
mod asset_preparation;
use macroquad::prelude::*;
use std::path::Path;

const PAINT: &str = "assets/source-art/north-city-01/paint05/B01-PAINT05-r01.png";
const REFERENCE: &str =
    "assets/source-art/first-sector-review-01/u01-style-r07/SCENE-U01-07-r01.png";
const LONG: &str =
    "assets/source-art/first-sector-review-01/u01-expansion-r06/east-long-native.png";
const PLAYER: &str = "assets/source-art/class-readability-01/bastion-copper-wave-01/accepted/native64/BA-CU01-P01-r01-native64.png";
const DX: [f32; 7] = [0., 160., 608., 800., 1056., 1376., 1536.];
const SX: [f32; 7] = [0., 160., 626., 834., 1083., 1427., 1585.];
const DY: [f32; 5] = [0., 176., 496., 816., 960.];
const SY: [f32; 5] = [0., 153., 507., 837., 992.];

fn config() -> Conf {
    Conf {
        window_title: "Project RL - PAINT05 - revue du dessin".into(),
        window_width: 1024,
        window_height: 768,
        high_dpi: false,
        ..Default::default()
    }
}
async fn prepare() -> Result<(), String> {
    loop {
        let mut done = true;
        for path in [PAINT, REFERENCE, LONG, PLAYER] {
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
fn background() {
    // Review-space registration of the new large painting at the existing
    // 64 px wall axes. Original source pixels remain untouched on disk.
    asset_preparation::with_texture(PAINT, |p| {
        for row in 0..4 {
            for col in 0..6 {
                draw_texture_ex(
                    &p.texture,
                    DX[col],
                    DY[row],
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(
                            SX[col],
                            SY[row],
                            SX[col + 1] - SX[col],
                            SY[row + 1] - SY[row],
                        )),
                        dest_size: Some(vec2(DX[col + 1] - DX[col], DY[row + 1] - DY[row])),
                        ..Default::default()
                    },
                );
            }
        }
    });
}
fn prefab(at: Vec2, clip: Rect) {
    asset_preparation::with_texture(LONG, |p| {
        let l = at.x.max(clip.x);
        let t = at.y.max(clip.y);
        let r = (at.x + p.texture.width()).min(clip.right());
        let b = (at.y + p.texture.height()).min(clip.bottom());
        if r > l && b > t {
            draw_texture_ex(
                &p.texture,
                l,
                t,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(l - at.x, t - at.y, r - l, b - t)),
                    ..Default::default()
                },
            );
        }
    });
}
fn neighbours() {
    // Complete native prefabs from the first city: roof + cornice + full
    // facade + plinth are never repainted into an invented perspective.
    let north = Rect::new(0., 0., 1408., 150.);
    let west = Rect::new(0., 0., 132., 960.);
    draw_rectangle(
        north.x,
        north.y,
        north.w,
        north.h,
        Color::from_hex(0x20221f),
    );
    draw_rectangle(west.x, west.y, west.w, west.h, Color::from_hex(0x20221f));
    for x in [132., 397., 662., 927., 1192.] {
        prefab(vec2(x, 150. - 563.), north);
    }
    for y in [-116., 447.] {
        prefab(vec2(132. - 265., y), west);
    }
    // Narrow contact shadow at the two adjoining structures. It masks the
    // old painted blue eave strip, without changing any room or furnishing.
    draw_rectangle(
        132.,
        150.,
        1276.,
        17.,
        Color::new(0.025, 0.027, 0.029, 0.88),
    );
}
fn actor(at: Vec2) {
    asset_preparation::with_texture(PLAYER, |p| {
        draw_texture_ex(
            &p.texture,
            at.x,
            at.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(64., 64.)),
                ..Default::default()
            },
        )
    });
}
fn camera(target: &RenderTarget, w: f32, h: f32) {
    set_camera(&Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., w, h))
    });
}
async fn capture(out: &Path) -> Result<(), String> {
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let target = render_target(1536, 960);
    target.texture.set_filter(FilterMode::Nearest);
    camera(&target, 1536., 960.);
    clear_background(BLACK);
    background();
    set_default_camera();
    next_frame().await;
    let before = target.texture.get_texture_data();
    camera(&target, 1536., 960.);
    background();
    neighbours();
    set_default_camera();
    next_frame().await;
    let after = target.texture.get_texture_data();
    let mut preserved = 0;
    for y in 167..960 {
        for x in 132..1536 {
            // GPU readback rows are bottom-to-top; export_png flips them.
            let i = ((959 - y) * 1536 + x) * 4;
            if before.bytes[i..i + 4] != after.bytes[i..i + 4] {
                return Err(format!(
                    "Neighbour edit changed protected apartment at {x},{y}"
                ));
            }
            preserved += 1;
        }
    }
    camera(&target, 1536., 960.);
    actor(vec2(896., 336.));
    set_default_camera();
    next_frame().await;
    let picture = target.texture.get_texture_data();
    picture.export_png(out.join("paint05-logement.png").to_str().unwrap());
    let picture_texture = asset_loading::from_image(&picture);
    picture_texture.set_filter(FilterMode::Nearest);
    let board = render_target(1024, 416);
    board.texture.set_filter(FilterMode::Nearest);
    camera(&board, 1024., 416.);
    clear_background(Color::from_hex(0x111820));
    asset_preparation::with_texture(REFERENCE, |p| {
        draw_texture_ex(
            &p.texture,
            0.,
            32.,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(0., 0., 512., 384.)),
                ..Default::default()
            },
        )
    });
    actor(vec2(300., 216.));
    draw_texture_ex(
        &picture_texture,
        512.,
        32.,
        WHITE,
        DrawTextureParams {
            source: Some(Rect::new(640., 960. - 144. - 384., 512., 384.)),
            flip_y: true,
            ..Default::default()
        },
    );
    draw_text(
        "Premiere ville - dessin source - 64 px",
        12.,
        23.,
        20.,
        Color::from_hex(0xdec69b),
    );
    draw_text(
        "PAINT05 - etude - 64 px/case",
        524.,
        23.,
        20.,
        Color::from_hex(0xdec69b),
    );
    set_default_camera();
    next_frame().await;
    board
        .texture
        .get_texture_data()
        .export_png(out.join("paint05-comparaison.png").to_str().unwrap());
    let checks = serde_json::json!({"review":"CITY-NORTH01-PAINT05-r04","status":"art-study-awaiting-user-prevalidation","campaign_integrated":false,"simulation_modified":false,"native_cell":64,"source":PAINT,"neighbour_prefab":LONG,"prefab_size":[265,563],"prefabs_drawn_at_original_native_size":true,"neighbour_edit_protected_pixels":preserved,"neighbour_edit_changed_protected_pixels":0,"registration":{"x_native":DX,"x_source":SX,"y_native":DY,"y_source":SY},"outputs":["paint05-logement.png","paint05-comparaison.png"],"limits":["Art direction study; not yet used as a playable map background.","Neutral albedo comparison, not the final engine lighting.","Production layer separation and exact door/collision registration remain subject to prevalidation."]});
    std::fs::write(
        out.join("paint05-checks.json"),
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
        eprintln!("PAINT05: {e}");
        std::process::exit(1);
    }
}
