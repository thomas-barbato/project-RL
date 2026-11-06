//! A whole-sector visual candidate, before any change to the playable layout.
//! The original native actor and first-city image are compared in one renderer.
#![allow(dead_code)]
#[path = "../src/asset_loading.rs"]
mod asset_loading;
#[path = "../src/asset_preparation.rs"]
mod asset_preparation;
use macroquad::prelude::*;
use std::path::Path;
use std::sync::OnceLock;

const SOURCE: &str = "assets/source-art/north-city-01/sector08/SECTEUR08-01-r02-source.png";
const REFERENCE: &str = "assets/maps/prison-street-expanded-01/street.png";
const PLAYER: &str = "assets/source-art/class-readability-01/bastion-copper-wave-01/accepted/native64/BA-CU01-P01-r01-native64.png";
const SCALE: f32 = 0.75;
const SIZE: Vec2 = Vec2::new(1152., 768.);
#[derive(serde::Deserialize)]
struct Layout {
    source: String,
    patches: Vec<Patch>,
}
#[derive(serde::Deserialize)]
struct Patch {
    id: String,
    file: String,
    rect: [u32; 4],
}
static LAYOUT: OnceLock<Layout> = OnceLock::new();
fn layout() -> &'static Layout {
    LAYOUT.get().expect("layout prepared before drawing")
}

fn config() -> Conf {
    Conf {
        window_title: "Project RL - SECTEUR08 - revue visuelle".into(),
        window_width: 1184,
        window_height: 864,
        high_dpi: false,
        ..Default::default()
    }
}
async fn prepare() -> Result<(), String> {
    loop {
        let mut done = true;
        for path in [SOURCE, REFERENCE, PLAYER]
            .into_iter()
            .chain(layout().patches.iter().map(|p| p.file.as_str()))
        {
            if !asset_preparation::ready(path)? {
                done = false;
                let path = path.to_string();
                asset_preparation::request(
                    path.clone(),
                    Box::new(move || {
                        Ok(asset_preparation::PreparedImage {
                            image: (*asset_preparation::image(&path)?).clone(),
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
        clear_background(Color::from_hex(0x111820));
        draw_text("Preparation du secteur 08...", 32., 48., 24., LIGHTGRAY);
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
        );
    });
}
fn actor(at: Vec2) {
    image(PLAYER, at, None, Some(vec2(64., 64.)));
}
fn candidate(origin: Vec2, interior: bool, patched: bool) {
    image(SOURCE, origin, None, Some(SIZE));
    if patched {
        for p in &layout().patches {
            let [x, y, w, h] = p.rect.map(|v| v as f32);
            image(
                &p.file,
                origin + vec2(x, y) * SCALE,
                Some(Rect::new(x, y, w, h)),
                Some(vec2(w, h) * SCALE),
            );
        }
    }
    actor(
        origin
            + if interior {
                vec2(832., 128.)
            } else {
                vec2(384., 320.)
            },
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
    for (interior, name) in [
        (false, "secteur08-ensemble.png"),
        (true, "secteur08-interieur.png"),
    ] {
        let t = target(1152, 800);
        candidate(vec2(0., 32.), interior, true);
        title(
            if interior {
                "08-1 - interieur - acteur original de 64 px"
            } else {
                "08-1 - quartier - acteur original de 64 px"
            },
            1152.,
        );
        export(&t, out, name).await?;
    }
    let baseline = target(1152, 800);
    candidate(vec2(0., 32.), false, false);
    title("08-1 - quartier - acteur original de 64 px", 1152.);
    export(&baseline, out, "secteur08-base.png").await?;
    let t = target(1024, 704);
    candidate(vec2(-64., -16.), false, true);
    export(&t, out, "secteur08-native.png").await?;
    // Separate targets clip each image without scaling the approved reference.
    let a = target(768, 704);
    image(REFERENCE, vec2(-448., -1088.), None, None);
    actor(vec2(192., 448.));
    set_default_camera();
    next_frame().await;
    let b = target(768, 704);
    candidate(vec2(-128., 0.), false, true);
    set_default_camera();
    next_frame().await;
    let t = target(1536, 736);
    for (texture, x) in [(&a.texture, 0.), (&b.texture, 768.)] {
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
    title("Premiere ville - natif 64 px/case", 768.);
    draw_text(
        "08-1 - meme acteur de 64 px",
        784.,
        23.,
        20.,
        Color::from_hex(0xe1c593),
    );
    export(&t, out, "secteur08-comparaison.png").await?;
    let mut checks = serde_json::json!({"review":"CITY-NORTH01-SECTEUR08-r03","status":"visual-candidate-awaiting-prevalidation",
        "source":SOURCE,"reference":REFERENCE,"source_dimensions":[1536,1024],"source_uniform_scale":SCALE,
        "native_environment":[1152,768],"native_cell":64,"actor":PLAYER,"actor_size":[64,64],
        "actor_pixels_unchanged":true,"scale_motivation":"Doors, furniture and bodies matched against the approved native actor; the initial image canvas is not the world scale.",
        "local_patch":"S08-B02 only; source rect 992,24,432,440, native destination 744,18,324,330",
        "scene_mode":"Whole-sector visual composition; no gameplay or collision validation claimed",
        "map_data_changed":false,"campaign_integrated":false,"figures":["secteur08-ensemble.png","secteur08-interieur.png","secteur08-native.png","secteur08-comparaison.png"]});
    #[cfg(debug_assertions)]
    {
        checks["resource_cache"] = asset_preparation::stats();
    }
    std::fs::write(
        out.join("secteur08-checks.json"),
        serde_json::to_vec_pretty(&checks).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
#[macroquad::main(config)]
async fn main() {
    asset_preparation::initialize_graphics_thread();
    let result = async {
        let bytes =
            project_rl::data_paths::read("assets/source-art/north-city-01/sector08/scene.json")?;
        let data: Layout = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        if data.source != SOURCE {
            return Err("Sector source mismatch".into());
        }
        LAYOUT
            .set(data)
            .map_err(|_| "sector layout already initialized".to_string())?;
        prepare().await?;
        let args = std::env::args().collect::<Vec<_>>();
        if let Some(i) = args.iter().position(|a| a == "--capture") {
            return capture(Path::new(args.get(i + 1).ok_or("capture folder missing")?)).await;
        }
        let mut interior = false;
        loop {
            if is_key_pressed(KeyCode::Escape) {
                break;
            }
            if is_key_pressed(KeyCode::I) {
                interior = !interior;
            }
            clear_background(Color::from_hex(0x111820));
            candidate(vec2(16., 48.), interior, true);
            title(
                "SECTEUR08 - revue de composition et d'echelle",
                screen_width(),
            );
            draw_text(
                "I : repere rue / interieur - Echap : fermer",
                16.,
                850.,
                19.,
                LIGHTGRAY,
            );
            next_frame().await;
        }
        Ok(())
    }
    .await;
    if let Err(error) = result {
        eprintln!("SECTEUR08: {error}");
        std::process::exit(1);
    }
}
