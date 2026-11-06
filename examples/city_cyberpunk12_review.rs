//! Isolated native64 review; presentation only, no campaign or save writes.
#![allow(dead_code)]
#[path = "../src/asset_loading.rs"]
mod asset_loading;
#[path = "../src/asset_preparation.rs"]
mod asset_preparation;
use macroquad::prelude::*;
use serde::Deserialize;
use std::{collections::HashMap, path::Path, sync::OnceLock};
#[derive(Deserialize)]
struct Review {
    review: String,
    status: String,
    before: String,
    after: String,
    roof_overrides: HashMap<String, String>,
    #[serde(default)]
    before_roof_overrides: HashMap<String, String>,
    #[serde(default)]
    before_label: String,
    #[serde(default)]
    after_label: String,
    views: Vec<View>,
}
#[derive(Deserialize, serde::Serialize)]
struct View {
    id: String,
    name: String,
    rect: [f32; 4],
    actor: [i32; 2],
    hide_roofs: Vec<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    after_patch: Option<ReviewPatch>,
}
#[derive(Deserialize, serde::Serialize)]
struct ReviewPatch {
    file: String,
    at_px: [f32; 2],
}
static REVIEW: OnceLock<Review> = OnceLock::new();
fn review() -> &'static Review {
    REVIEW.get().expect("review loaded before drawing")
}
const PLAYER: &str = "assets/source-art/class-readability-01/bastion-copper-wave-01/accepted/native64/BA-CU01-P01-r01-native64.png";
const ROOFS: [(&str, Vec2); 4] = [
    (
        "assets/maps/prison-street-expanded-01/roof-0.png",
        vec2(140., 3168.),
    ),
    (
        "assets/maps/prison-street-expanded-01/roof-1.png",
        vec2(711., 3208.),
    ),
    (
        "assets/maps/prison-street-expanded-01/roof-2.png",
        vec2(448., 1152.),
    ),
    (
        "assets/maps/prison-street-expanded-01/roof-3.png",
        vec2(464., 1648.),
    ),
];
fn config() -> Conf {
    Conf {
        window_title: "Project RL - premiere ville - CYBERPUNK12".into(),
        window_width: 1664,
        window_height: 1056,
        high_dpi: false,
        ..Default::default()
    }
}
async fn prepare() -> Result<(), String> {
    let bytes =
        project_rl::data_paths::read("assets/source-art/city-cyberpunk12/review-current.json")?;
    let spec: Review = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    REVIEW.set(spec).map_err(|_| "already prepared")?;
    loop {
        let mut ready = true;
        for path in [review().before.as_str(), review().after.as_str(), PLAYER]
            .into_iter()
            .chain(ROOFS.iter().map(|v| v.0))
            .chain(review().roof_overrides.values().map(String::as_str))
            .chain(review().before_roof_overrides.values().map(String::as_str))
            .chain(review().views.iter().filter_map(|view| {
                view.after_patch.as_ref().map(|patch| patch.file.as_str())
            }))
        {
            if !asset_preparation::ready(path)? {
                ready = false;
                let key = path.to_string();
                asset_preparation::request(
                    key.clone(),
                    Box::new(move || {
                        Ok(asset_preparation::PreparedImage {
                            image: (*asset_preparation::image(&key)?).clone(),
                            cell: [64, 64],
                            anchor: [32., 32.],
                        })
                    }),
                );
            }
        }
        asset_preparation::pump();
        if ready {
            return Ok(());
        }
        clear_background(BLACK);
        next_frame().await;
    }
}
fn draw(path: &str, at: Vec2, source: Option<Rect>) {
    asset_preparation::with_texture(path, |p| {
        draw_texture_ex(
            &p.texture,
            at.x,
            at.y,
            WHITE,
            DrawTextureParams {
                source,
                ..Default::default()
            },
        )
    });
}
fn scene(new: bool, view: Rect, spec: &View) {
    clear_background(BLACK);
    draw(
        if new {
            &review().after
        } else {
            &review().before
        },
        vec2(0., 32.),
        Some(view),
    );
    if new {
        if let Some(patch) = &spec.after_patch {
            let [x, y] = patch.at_px;
            draw(&patch.file, vec2(x, y) - view.point() + vec2(0., 32.), None);
        }
    }
    for (index, (original, at)) in ROOFS.into_iter().enumerate() {
        if spec.hide_roofs.contains(&index) {
            continue;
        }
        let overrides = if new {
            &review().roof_overrides
        } else {
            &review().before_roof_overrides
        };
        let file = overrides
            .get(&index.to_string())
            .map(String::as_str)
            .unwrap_or(original);
        draw(file, at - view.point() + vec2(0., 32.), None);
    }
    let actor = vec2(spec.actor[0] as f32 * 64., spec.actor[1] as f32 * 64.);
    draw(PLAYER, actor - view.point() + vec2(0., 32.), None);
    draw_rectangle(0., 0., view.w, 32., Color::from_hex(0x111820));
    draw_text(
        &format!(
            "{} - {} - 64 px/case",
            spec.name,
            if new && !review().after_label.is_empty() {
                &review().after_label
            } else if !new && !review().before_label.is_empty() {
                &review().before_label
            } else if new {
                "apres / lot 02 / candidat"
            } else {
                "avant / lot 01 conserve"
            }
        ),
        12.,
        23.,
        19.,
        Color::from_hex(0xe1c593),
    );
}
fn target(w: u32, h: u32) -> RenderTarget {
    let t = render_target(w, h);
    t.texture.set_filter(FilterMode::Nearest);
    set_camera(&Camera2D {
        render_target: Some(t.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., w as f32, h as f32))
    });
    t
}
async fn capture(out: &Path, only: Option<&str>) -> Result<(), String> {
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    for spec in &review().views {
        if only.is_some_and(|name| name != spec.id) {
            continue;
        }
        let [x, y, w, h] = spec.rect;
        let view = Rect::new(x, y, w, h);
        for new in [false, true] {
            let t = target(view.w as u32, view.h as u32 + 32);
            scene(new, view, spec);
            set_default_camera();
            next_frame().await;
            t.texture.get_texture_data().export_png(
                out.join(format!(
                    "{}-{}.png",
                    spec.id,
                    if new { "apres" } else { "avant" }
                ))
                .to_str()
                .ok_or("capture path")?,
            );
        }
    }
    let mut checks = serde_json::json!({"review":review().review,"status":review().status,"native_cell":64,"actor":PLAYER,"actor_size":[64,64],"views":review().views,"roof_visibility":"Explicit inspection views only; game door/roof rules are unchanged.","campaign_integrated":false,"review_only":true,"lighting":"Source map illumination only; live campaign light shader and visibility are not evaluated by this review.","resources_prepared_before_display":true});
    #[cfg(debug_assertions)]
    {
        checks["resource_cache"] = asset_preparation::stats();
    }
    std::fs::write(
        out.join("checks.json"),
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
        if let Some(i) = args.iter().position(|v| v == "--capture") {
            let only = args
                .iter()
                .position(|v| v == "--view")
                .and_then(|index| args.get(index + 1))
                .map(String::as_str);
            return capture(Path::new(args.get(i + 1).ok_or("capture path")?), only).await;
        }
        let mut selected = 0;
        let mut y = review().views[0].rect[1];
        let mut new = true;
        loop {
            if is_key_pressed(KeyCode::Escape) {
                break;
            }
            if is_key_pressed(KeyCode::Tab) {
                new = !new;
            }
            for (index, key) in [
                KeyCode::Key1,
                KeyCode::Key2,
                KeyCode::Key3,
                KeyCode::Key4,
                KeyCode::Key5,
                KeyCode::Key6,
                KeyCode::Key7,
                KeyCode::Key8,
                KeyCode::Key9,
            ]
            .into_iter()
            .enumerate()
            {
                if is_key_pressed(key) && index < review().views.len() {
                    selected = index;
                    y = review().views[index].rect[1];
                }
            }
            if is_key_down(KeyCode::Up) {
                y = (y - 320. * get_frame_time()).max(0.);
            }
            if is_key_down(KeyCode::Down) {
                y = (y + 320. * get_frame_time()).min(3168.);
            }
            let spec = &review().views[selected];
            scene(
                new,
                Rect::new(spec.rect[0], y, spec.rect[2], spec.rect[3]),
                spec,
            );
            next_frame().await;
        }
        Ok(())
    }
    .await;
    if let Err(e) = result {
        eprintln!("CYBERPUNK12: {e}");
        std::process::exit(1);
    }
}
