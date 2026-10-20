//! ANIM-READ-01: isolated comparison, using production renderers and unchanged art.
//! Only the diagnostic entry point calls this module. Never opens a campaign save.
use crate::{combat_visuals::CombatMotion, game_art, terminal_view::PlayerWeaponPose};
use macroquad::{miniquad::UniformDesc, miniquad::UniformType, prelude::*};
use project_rl::{
    game::GameState,
    world::{GridPos, Map, Terrain},
};

#[path = "blizzard_readability_review.rs"]
pub(crate) mod blizzard;
#[path = "animation_shape_review.rs"]
pub(crate) mod shape;

const WIDTH: usize = 448;
const HEIGHT: usize = 320;
const FRAMES: usize = 72;
const COLUMNS: usize = 8;
const FPS: f64 = 60.;
const CASES: [(&str, &str, f64, f64, f32); 8] = [
    ("A01", "Projectile occulte", 0.40, 1.50, 1.30),
    ("OC05", "Drain occulte", 0.30, 1.50, 1.40),
    ("FR06", "Éclatement de glace", 0.42, 1.50, 1.12),
    ("SM02", "Explosion de poison", 0.46, 1.50, 1.18),
    ("B01", "Frappe de rupture", 0.84, 1.35, 1.18),
    ("FE05", "Boule de feu", 0.90, 1.30, 1.00),
    ("glide", "Déplacement entre cases", 0.08, 1.50, 1.00),
    ("reaction", "Réaction à un coup", 0.18, 1.40, 1.00),
];

fn cell(at: GridPos) -> Option<Rect> {
    (at.x >= 0 && at.x < 7 && at.y >= 0 && at.y < 5)
        .then(|| Rect::new(at.x as f32 * 64., at.y as f32 * 64., 64., 64.))
}

fn motion(game: &GameState, id: &str) -> CombatMotion {
    let mut value = CombatMotion::default();
    let from = GridPos::new(1, 2);
    let to = if id == "B01" {
        GridPos::new(2, 2)
    } else {
        GridPos::new(4, 2)
    };
    match id {
        "A01" => value.play_spell(game.player_id(), from, to, 0.),
        "B01" => value.play_rupture(from, to, 0.),
        "FE05" => value.play_fireball(from, to, 1, 0.),
        "glide" => value.glide(game.player_id(), from, GridPos::new(2, 2), 0.),
        "reaction" => value.play(game.player_id(), from, to, true, 0.),
        _ => value.play_agent_magic(id, from, to, 1, 0.),
    }
    value
}

fn actors(game: &GameState, motion: &CombatMotion, id: &str, time: f64, dark: bool) {
    let player = if id == "glide" {
        GridPos::new(2, 2)
    } else {
        GridPos::new(1, 2)
    };
    let rect = motion.actor_rect(game.player_id(), cell(player).unwrap(), time, false);
    game_art::draw_world(
        rect,
        game_art::player(Some("core:arcaniste"), PlayerWeaponPose::Unarmed, None),
        1.,
    );
    let target = if id == "B01" {
        GridPos::new(2, 2)
    } else {
        GridPos::new(4, 2)
    };
    game_art::draw_world(
        cell(target).unwrap(),
        game_art::Visual::CampaignMonster {
            number: 120,
            variant: 0,
        },
        1.,
    );
    // Controlled review illumination. Same ground and actors in both columns.
    draw_rectangle(
        0.,
        0.,
        WIDTH as f32,
        HEIGHT as f32,
        Color::new(0., 0., 0., if dark { 0.28 } else { 0.08 }),
    );
}

fn terrain(game: &GameState, dark: bool) {
    clear_background(Color::from_rgba(13, 18, 20, 255));
    for y in 0..5 {
        for x in 0..7 {
            let at = GridPos::new(x, y);
            game_art::draw_world(
                cell(at).unwrap(),
                game_art::Visual::Terrain {
                    kind: if game.map().is_walkable(at) {
                        crate::test_sector::Decor::Deck
                    } else {
                        crate::test_sector::Decor::Wall
                    },
                    position: at,
                },
                1.,
            );
        }
    }
    draw_rectangle(
        0.,
        0.,
        WIDTH as f32,
        HEIGHT as f32,
        Color::new(0., 0., 0., if dark { 0.60 } else { 0.10 }),
    );
}

fn scene(game: &GameState, motion: &CombatMotion, id: &str, time: f64, dark: bool, effects: bool) {
    terrain(game, dark);
    let clip = Rect::new(0., 0., WIDTH as f32, HEIGHT as f32);
    if effects {
        motion.draw_agent_world(game, time, false, true, clip, cell);
        motion.draw_fireballs(game, time, false, true, cell);
    }
    actors(game, motion, id, time, dark);
    if effects {
        motion.draw_agent_world(game, time, false, false, clip, cell);
        motion.draw_fireballs(game, time, false, false, cell);
        motion.draw_spells(game, time, false, clip, cell);
    }
}

fn contrast_material() -> Result<Material, String> {
    load_material(
        ShaderSource::Glsl {
            vertex: r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;varying highp vec2 uv;
void main(){uv=texcoord;gl_Position=Projection*Model*vec4(position,1.0);}"#,
            fragment: r#"#version 100
precision highp float;varying highp vec2 uv;
uniform sampler2D Texture;uniform sampler2D cleanScene;uniform float gain;
void main(){vec3 clean=texture2D(cleanScene,uv).rgb;vec3 effect=texture2D(Texture,uv).rgb;
gl_FragColor=vec4(clamp(clean+(effect-clean)*gain,0.0,1.0),1.0);}"#,
        },
        MaterialParams {
            uniforms: vec![UniformDesc::new("gain", UniformType::Float1)],
            textures: vec!["cleanScene".into()],
            ..Default::default()
        },
    )
    .map_err(|e| e.to_string())
}

fn copy_panel(image: &Image, sheet: &mut Image, frame: usize, panel: usize) {
    let left = 16 + panel * 464;
    let top = 56;
    for y in 0..HEIGHT {
        let source =
            ((usize::from(image.height) - 1 - (top + y)) * usize::from(image.width) + left) * 4;
        let dest_y = usize::from(sheet.height) - 1 - (frame / COLUMNS * HEIGHT + y);
        let dest = (dest_y * usize::from(sheet.width) + frame % COLUMNS * WIDTH) * 4;
        sheet.bytes[dest..dest + WIDTH * 4]
            .copy_from_slice(&image.bytes[source..source + WIDTH * 4]);
    }
}

pub async fn capture(output: &std::path::Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    macroquad::miniquad::window::set_fullscreen(false);
    request_new_screen_size(944., 408.);
    for _ in 0..3 {
        next_frame().await;
    }
    let game = GameState::new(
        Map::filled(7, 5, Terrain::Floor).map_err(|e| e.to_string())?,
        GridPos::new(1, 2),
        1872,
    )
    .map_err(|e| e.to_string())?;
    let state_before = format!("{game:?}");
    let clean = render_target(WIDTH as u32, HEIGHT as u32);
    let effect = render_target(WIDTH as u32, HEIGHT as u32);
    clean.texture.set_filter(FilterMode::Nearest);
    effect.texture.set_filter(FilterMode::Nearest);
    let material = contrast_material()?;
    let mut rows = Vec::new();
    for (index, (id, title, seconds, scale, gain)) in CASES.into_iter().enumerate() {
        let motion = motion(&game, id);
        for dark in [true, false] {
            let light = if dark { "sombre" } else { "clair" };
            let mut sheets: [Image; 2] = std::array::from_fn(|_| {
                Image::gen_image_color(
                    (WIDTH * COLUMNS) as u16,
                    (HEIGHT * FRAMES.div_ceil(COLUMNS)) as u16,
                    BLACK,
                )
            });
            for frame in 0..FRAMES {
                let time = frame as f64 / FPS;
                let _clock = crate::effect_geometry::frame_time(time);
                for (target, effects) in [(&clean, false), (&effect, true)] {
                    let mut camera =
                        Camera2D::from_display_rect(Rect::new(0., 0., WIDTH as f32, HEIGHT as f32));
                    camera.render_target = Some(target.clone());
                    set_camera(&camera);
                    scene(&game, &motion, id, time, dark, effects);
                }
                set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
                clear_background(Color::from_rgba(13, 18, 20, 255));
                for panel in 0..2 {
                    let x = 16. + panel as f32 * 464.;
                    draw_text(
                        if panel == 0 {
                            "Actuel"
                        } else {
                            "Contraste proposé · même phase source"
                        },
                        x,
                        32.,
                        20.,
                        WHITE,
                    );
                    // Both columns use opaque compositing. The default sprite
                    // blend would multiply render-target alpha a second time.
                    material.set_texture("cleanScene", clean.texture.clone());
                    material.set_uniform("gain", if panel == 0 { 1. } else { gain });
                    gl_use_material(&material);
                    draw_texture_ex(
                        &effect.texture,
                        x,
                        56.,
                        WHITE,
                        DrawTextureParams {
                            flip_y: true,
                            ..Default::default()
                        },
                    );
                    gl_use_default_material();
                }
                let image = crate::ui_capture::framebuffer()?;
                if usize::from(image.width) != 944 || usize::from(image.height) != 408 {
                    return Err(format!(
                        "Capture native attendue 944x408, reçue {}x{}",
                        image.width, image.height
                    ));
                }
                for (panel, sheet) in sheets.iter_mut().enumerate() {
                    copy_panel(&image, sheet, frame, panel);
                }
                if frame == 12 && dark {
                    image.export_png(
                        output
                            .join(format!("{:02}-reference.png", index + 1))
                            .to_str()
                            .ok_or("capture path")?,
                    );
                }
                next_frame().await;
            }
            for (panel, sheet) in sheets.iter().enumerate() {
                // The review publisher encodes these generated framebuffer
                // bytes with Pillow; avoid the debug PNG encoder's long stall.
                std::fs::write(
                    output.join(format!("{:02}-{light}-{panel}.rgba", index + 1)),
                    &sheet.bytes,
                )
                .map_err(|e| e.to_string())?;
            }
            println!("ANIM-READ-01 {id} {light}: {FRAMES} phases natives");
        }
        rows.push(serde_json::json!({"number":index+1,"id":id,"title":title,"seconds":seconds,"duration_scale":scale,"contrast_gain":gain}));
    }
    if state_before != format!("{game:?}") {
        return Err("La capture a modifié l'état moteur".into());
    }
    std::fs::write(output.join("capture.json"), serde_json::to_vec_pretty(&serde_json::json!({
        "review":"ANIM-READ-01","status":"proposal_pending_player_review","native_tile":64,
        "frame_width":WIDTH,"frame_height":HEIGHT,"frames":FRAMES,"columns":COLUMNS,"fps":FPS,
        "source_phases_identical_in_columns":true,"game_state_unchanged":true,"normal_game_activation":false,
        "backgrounds":"Controlled dim and brighter review illumination on native game floor and actors",
        "cases":rows
    })).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    Ok(())
}
