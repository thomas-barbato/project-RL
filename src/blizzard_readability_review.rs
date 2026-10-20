//! BLIZZARD-VIS-01: isolated visibility proposal driven by a real FR05 cast.
use super::*;
use macroquad::miniquad::{BlendFactor, BlendState, BlendValue, Equation, PipelineParams};
use project_rl::{
    content::ContentLoader,
    game::{CommandOutcome, GameCommand, GameRules},
    world::has_line_of_sight,
};

fn fixture(warning: bool, wall: bool) -> Result<GameState, String> {
    let loaded = ContentLoader::load(
        &[std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content")],
        &semver::Version::new(0, 1, 0),
    )
    .map_err(|e| e.to_string())?;
    let class = loaded
        .character_classes()
        .get(&"core:arcaniste".parse().unwrap())
        .ok_or("Arcaniste absent")?;
    let mut rules = GameRules {
        weapons: loaded.weapons().clone(),
        items: loaded.items().clone(),
        statuses: loaded.statuses().clone(),
        player_weapon_slots: ["core:left_hand", "core:right_hand", "core:reserve"]
            .map(|s| s.parse().unwrap())
            .to_vec(),
        ..Default::default()
    };
    class
        .apply_to_rules(&mut rules, class.recommended_attributes())
        .map_err(|e| e.to_string())?;
    rules
        .agent_class_start
        .as_mut()
        .ok_or("Agent absent")?
        .skills = vec!["FR05".into()];
    let mut map = Map::filled(7, 5, Terrain::Floor).map_err(|e| e.to_string())?;
    if wall {
        for y in 0..5 {
            map.set_terrain(GridPos::new(5, y), Terrain::Wall)
                .map_err(|e| e.to_string())?;
        }
    }
    let mut game = GameState::new_with_rules(map, GridPos::new(1, 2), 1872, rules)
        .map_err(|e| e.to_string())?;
    let outcome = game.process_player_command(GameCommand::UseTechniqueAt {
        technique: project_rl::skills::agent::technique("FR05"),
        target: GridPos::new(3, 2),
        weapon_slot: 0,
    });
    if outcome != CommandOutcome::Applied {
        return Err(format!("Blizzard refusé : {outcome:?}"));
    }
    if !warning {
        if game.process_player_command(GameCommand::Wait) != CommandOutcome::Applied {
            return Err("Attente Blizzard refusée".into());
        }
    }
    let effects = game.agent_magic_visual_effects();
    let effect = effects
        .iter()
        .find(|e| e.skill == "FR05")
        .ok_or("Zone Blizzard absente")?;
    if (effect.stage == 0) != warning {
        return Err(format!("Étape Blizzard inattendue : {}", effect.stage));
    }
    Ok(game)
}

fn allowed(game: &GameState, at: GridPos) -> bool {
    game.map().is_walkable(at)
        && game.player_visibility().is_visible(at)
        && has_line_of_sight(game.map(), GridPos::new(3, 2), at, true)
}

fn material() -> Result<Material, String> {
    load_material(ShaderSource::Glsl {
        vertex:r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;varying highp vec2 point;varying highp vec2 uv;varying lowp vec4 tint;
void main(){point=position.xy;uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#,
        fragment:r#"#version 100
precision highp float;varying highp vec2 point;varying highp vec2 uv;varying lowp vec4 tint;
uniform sampler2D Texture;uniform sampler2D floorMask;
void main(){vec2 p=point/64.0;if(p.x<0.0||p.y<0.0||p.x>=7.0||p.y>=5.0)discard;
if(texture2D(floorMask,(floor(p)+vec2(0.5))/vec2(7.0,5.0)).r<0.5)discard;
vec4 c=texture2D(Texture,uv);float light=dot(c.rgb,vec3(0.2126,0.7152,0.0722));
float ice=smoothstep(0.40,0.82,light);c.rgb=mix(min(c.rgb*1.18,vec3(1.0)),vec3(0.80,0.94,1.0),ice*0.72);
c.a*=mix(0.76,1.0,ice);gl_FragColor=c*tint;}"#,
    },MaterialParams { textures:vec!["floorMask".into()],pipeline_params:PipelineParams { color_blend:Some(BlendState::new(Equation::Add,BlendFactor::Value(BlendValue::SourceAlpha),BlendFactor::OneMinusValue(BlendValue::SourceAlpha))),..Default::default() },..Default::default() }).map_err(|e|e.to_string())
}

pub async fn capture(output: &std::path::Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    macroquad::miniquad::window::set_fullscreen(false);
    request_new_screen_size(944., 408.);
    for _ in 0..3 {
        next_frame().await;
    }
    let image = Image::from_file_with_format(
        include_bytes!("../assets/animations/agent/skill-animation-review-25/storm-atlas.png"),
        Some(ImageFormat::Png),
    )
    .map_err(|e| e.to_string())?;
    let storm = Texture2D::from_image(&image);
    storm.set_filter(FilterMode::Linear);
    let clip = material()?;
    let opaque = contrast_material()?;
    let stages: [RenderTarget; 2] =
        std::array::from_fn(|_| render_target(WIDTH as u32, HEIGHT as u32));
    for stage in &stages {
        stage.texture.set_filter(FilterMode::Nearest);
    }
    let mut rows = Vec::new();
    for (index, (warning, wall, title)) in [
        (true, false, "Blizzard · avertissement"),
        (false, false, "Blizzard · tempête active"),
        (false, true, "Blizzard · près d’une cloison"),
    ]
    .into_iter()
    .enumerate()
    {
        let game = fixture(warning, wall)?;
        let state_before = format!("{game:?}");
        let motion = CombatMotion::default();
        let mut mask = Image::gen_image_color(7, 5, BLACK);
        let mut excluded = Vec::new();
        for y in 0..5 {
            for x in 0..7 {
                if allowed(&game, GridPos::new(x, y)) {
                    mask.set_pixel(x as u32, y as u32, WHITE);
                } else {
                    excluded.push([x, y]);
                }
            }
        }
        let mask = Texture2D::from_image(&mask);
        mask.set_filter(FilterMode::Nearest);
        clip.set_texture("floorMask", mask.clone());
        let effects = game.agent_magic_visual_effects();
        let effect = effects.iter().find(|e| e.skill == "FR05").unwrap();
        let diameter = 320. * 1.10;
        let center = cell(GridPos::new(3, 2)).unwrap().center();
        for dark in [true, false] {
            let light = if dark { "sombre" } else { "clair" };
            let mut frames: [Image; 2] = std::array::from_fn(|_| {
                Image::gen_image_color(
                    (WIDTH * COLUMNS) as u16,
                    (HEIGHT * FRAMES.div_ceil(COLUMNS)) as u16,
                    BLACK,
                )
            });
            for frame in 0..FRAMES {
                let time = frame as f64 / FPS;
                let _clock = crate::effect_geometry::frame_time(time);
                for (panel, stage) in stages.iter().enumerate() {
                    let mut camera =
                        Camera2D::from_display_rect(Rect::new(0., 0., WIDTH as f32, HEIGHT as f32));
                    camera.render_target = Some(stage.clone());
                    set_camera(&camera);
                    if panel == 0 {
                        scene(&game, &motion, "FR05", time, dark, true);
                    } else {
                        terrain(&game, dark);
                        gl_use_material(&clip);
                        draw_texture_ex(
                            &storm,
                            center.x - diameter * 0.5,
                            center.y - diameter * 0.5,
                            Color::new(1., 1., 1., if warning { 0.42 } else { 0.94 }),
                            DrawTextureParams {
                                source: Some(Rect::new(
                                    ((time * 6.) as usize % 6) as f32 * 256.,
                                    0.,
                                    256.,
                                    256.,
                                )),
                                dest_size: Some(vec2(diameter, diameter)),
                                ..Default::default()
                            },
                        );
                        gl_use_default_material();
                        actors(&game, &motion, "FR05", time, dark);
                    }
                }
                set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
                clear_background(Color::from_rgba(13, 18, 20, 255));
                for (panel, stage) in stages.iter().enumerate() {
                    let x = 16. + panel as f32 * 464.;
                    draw_text(
                        if panel == 0 {
                            "Blizzard actuel"
                        } else {
                            "Blizzard proposé · neige plus claire"
                        },
                        x,
                        32.,
                        19.,
                        WHITE,
                    );
                    opaque.set_texture("cleanScene", stage.texture.clone());
                    opaque.set_uniform("gain", 1.0f32);
                    gl_use_material(&opaque);
                    draw_texture_ex(
                        &stage.texture,
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
                if (image.width, image.height) != (944, 408) {
                    return Err("Dimensions natives incorrectes".into());
                }
                for (panel, sheet) in frames.iter_mut().enumerate() {
                    copy_panel(&image, sheet, frame, panel);
                }
                if frame == 24 && dark {
                    image.export_png(
                        output
                            .join(format!("{:02}-reference.png", index + 1))
                            .to_str()
                            .ok_or("capture path")?,
                    );
                }
                next_frame().await;
            }
            // A clean plate permits strict wall/visibility tests for a loop
            // that deliberately remains active at the end of the capture.
            set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
            terrain(&game, dark);
            actors(&game, &motion, "FR05", 0., dark);
            let image = crate::ui_capture::framebuffer()?;
            let mut clean = Image::gen_image_color(WIDTH as u16, HEIGHT as u16, BLACK);
            for y in 0..HEIGHT {
                let src = ((usize::from(image.height) - 1 - y) * usize::from(image.width)) * 4;
                let dst = (HEIGHT - 1 - y) * WIDTH * 4;
                clean.bytes[dst..dst + WIDTH * 4]
                    .copy_from_slice(&image.bytes[src..src + WIDTH * 4]);
            }
            clean.export_png(
                output
                    .join(format!("{:02}-{light}-clean.png", index + 1))
                    .to_str()
                    .ok_or("plate path")?,
            );
            for (panel, sheet) in frames.iter().enumerate() {
                std::fs::write(
                    output.join(format!("{:02}-{light}-{panel}.rgba", index + 1)),
                    &sheet.bytes,
                )
                .map_err(|e| e.to_string())?;
            }
            println!("BLIZZARD-VIS-01 warning={warning} wall={wall} {light}: {FRAMES} phases");
        }
        if state_before != format!("{game:?}") {
            return Err("La capture a modifié l'état moteur".into());
        }
        rows.push(serde_json::json!({"number":index+1,"id":"FR05","review_id":format!("BLIZZARD-VIS-01-{:02}",index+1),"title":title,"seconds":1.0,"duration_scale":1.30,"loop_seconds":1.0,"physical_wall":wall,"warning":warning,"stage":effect.stage,"rank":effect.rank,"affected_cells":effect.cells.iter().map(|p|[p.x,p.y]).collect::<Vec<_>>(),"excluded_visual_cells":excluded,"before_label":"Blizzard actuel","after_label":"Blizzard plus visible","rationale":if warning {"L’avertissement devient plus lisible tout en restant moins intense que la tempête active."} else {"La neige et les éclats sont plus clairs ; la brume reste transparente et le dessin dépasse naturellement sur le sol libre."}}));
    }
    std::fs::write(output.join("capture.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":"BLIZZARD-VIS-01","status":"proposal_pending_player_review","native_tile":64,"frame_width":WIDTH,"frame_height":HEIGHT,"frames":FRAMES,"columns":COLUMNS,"fps":FPS,"real_FR05_cast":true,"game_state_unchanged":true,"normal_game_activation":false,"source_opacity":{"warning":0.28,"active":0.68},"proposed_opacity":{"warning":0.42,"active":0.94},"visual_overhang_factor":1.10,"cases":rows})).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
    Ok(())
}
