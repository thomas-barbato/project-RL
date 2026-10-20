//! ANIM-FLOW-01: complete painted effects on free floor, without damage-cell cuts.
//! Diagnostic only. No production rendering, damage geometry or save changes.
use super::*;
use macroquad::miniquad::{BlendFactor, BlendState, BlendValue, Equation, PipelineParams};
use project_rl::skills::spatial::{Area, affected_cells};
use project_rl::world::has_line_of_sight;

const SHAPES: [(&str, &str, f64, f64, bool); 4] = [
    ("FR06", "Glace · sol libre", 0.42, 1.50, false),
    ("FE05", "Feu · sol libre", 0.90, 1.30, false),
    ("SM02", "Poison · sol libre", 0.46, 1.50, false),
    ("FR06", "Glace · contre une cloison", 0.42, 1.50, true),
];

struct Sheet {
    texture: Texture2D,
    side: f32,
    painted: Rect,
}

impl Sheet {
    fn load(bytes: &[u8]) -> Self {
        let image = Image::from_file_with_format(bytes, Some(ImageFormat::Png)).unwrap();
        let side = usize::from(image.height);
        let mut low = (side, side);
        let mut high = (0, 0);
        for y in 0..side {
            for x in 0..usize::from(image.width) {
                if image.bytes[(y * usize::from(image.width) + x) * 4 + 3] >= 16 {
                    low = (low.0.min(x % side), low.1.min(y));
                    high = (high.0.max(x % side), high.1.max(y));
                }
            }
        }
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Linear);
        Self {
            texture,
            side: side as f32,
            painted: Rect::new(
                low.0 as f32,
                low.1 as f32,
                (high.0 - low.0 + 1) as f32,
                (high.1 - low.1 + 1) as f32,
            ),
        }
    }

    fn draw(&self, frame: usize, diameter: f32, opacity: f32) {
        // Fit once across the whole sequence, preserving the full source frame
        // and its aspect ratio. Neither frame crops nor cell-sized scissors.
        let scale = diameter / self.painted.w.max(self.painted.h);
        let center = cell(GridPos::new(4, 2)).unwrap().center();
        let size = self.side * scale;
        let origin = center - self.painted.center() * scale;
        draw_texture_ex(
            &self.texture,
            origin.x,
            origin.y,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                source: Some(Rect::new(
                    frame as f32 * self.side,
                    0.,
                    self.side,
                    self.side,
                )),
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    }
}

fn allowed(game: &GameState, at: GridPos) -> bool {
    game.map().is_walkable(at)
        && game.player_visibility().is_visible(at)
        && has_line_of_sight(game.map(), GridPos::new(4, 2), at, true)
}

fn mask(game: &GameState) -> Texture2D {
    let mut image = Image::gen_image_color(7, 5, BLACK);
    for y in 0..5 {
        for x in 0..7 {
            if allowed(game, GridPos::new(x, y)) {
                image.set_pixel(x as u32, y as u32, WHITE);
            }
        }
    }
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    texture
}

fn material() -> Result<Material, String> {
    load_material(ShaderSource::Glsl {
        vertex: r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;varying highp vec2 point;varying highp vec2 uv;varying lowp vec4 tint;
void main(){point=position.xy;uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#,
        fragment: r#"#version 100
precision highp float;varying highp vec2 point;varying highp vec2 uv;varying lowp vec4 tint;
uniform sampler2D Texture;uniform sampler2D floorMask;
void main(){vec2 q=point/64.0;if(q.x<0.0||q.y<0.0||q.x>=7.0||q.y>=5.0)discard;
if(texture2D(floorMask,(floor(q)+vec2(0.5))/vec2(7.0,5.0)).r<0.5)discard;
gl_FragColor=texture2D(Texture,uv)*tint;}"#,
    }, MaterialParams {
        textures: vec!["floorMask".into()],
        pipeline_params: PipelineParams { color_blend: Some(BlendState::new(Equation::Add, BlendFactor::Value(BlendValue::SourceAlpha), BlendFactor::OneMinusValue(BlendValue::SourceAlpha))), ..Default::default() },
        ..Default::default()
    }).map_err(|e| e.to_string())
}

fn natural(sheet: &Sheet, id: &str, time: f64, background: bool, material: &Material) {
    let age = time as f32;
    let sample = match id {
        "FR06" if background && (0.0..0.42).contains(&age) => Some((
            (age / 0.42 * 6.) as usize,
            0.88 * ((0.42 - age) / 0.12).clamp(0., 1.),
        )),
        "FE05" if background && (0.35..0.90).contains(&age) => Some((
            ((age - 0.35) / 0.55 * 6.) as usize,
            0.95 * ((0.90 - age) / 0.16).clamp(0., 1.),
        )),
        "SM02" if !background && (0.0..0.46).contains(&age) => {
            Some(((age / 0.46 * 6.) as usize, 0.88))
        }
        _ => None,
    };
    if let Some((frame, opacity)) = sample {
        gl_use_material(material);
        // Visual overhang proposed in this review; never a damage radius.
        sheet.draw(frame.min(5), 192. * 1.12, opacity);
        gl_use_default_material();
    }
}

fn proposed(
    game: &GameState,
    motion: &CombatMotion,
    id: &str,
    time: f64,
    dark: bool,
    sheet: &Sheet,
    material: &Material,
) {
    terrain(game, dark);
    natural(sheet, id, time, true, material);
    actors(game, motion, id, time, dark);
    if id == "FE05" {
        motion.draw_fireballs(game, time, false, false, cell);
    }
    natural(sheet, id, time, false, material);
}

pub async fn capture(output: &std::path::Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    macroquad::miniquad::window::set_fullscreen(false);
    request_new_screen_size(944., 408.);
    for _ in 0..3 {
        next_frame().await;
    }
    let sheets = [
        Sheet::load(include_bytes!(
            "../assets/animations/agent/skill-animation-review-26/burst-atlas.png"
        )),
        Sheet::load(include_bytes!(
            "../assets/animations/agent/skill-animation-review-20/burst-atlas.png"
        )),
        Sheet::load(include_bytes!(
            "../assets/animations/agent/skill-animation-review-62/poison-burst-atlas.png"
        )),
    ];
    let stages: [RenderTarget; 2] =
        std::array::from_fn(|_| render_target(WIDTH as u32, HEIGHT as u32));
    for stage in &stages {
        stage.texture.set_filter(FilterMode::Nearest);
    }
    let opaque = contrast_material()?;
    let clip = material()?;
    let mut rows = Vec::new();
    for (index, (id, title, seconds, scale, wall)) in SHAPES.into_iter().enumerate() {
        let mut map = Map::filled(7, 5, Terrain::Floor).map_err(|e| e.to_string())?;
        if wall {
            for y in 0..5 {
                map.set_terrain(GridPos::new(5, y), Terrain::Wall)
                    .map_err(|e| e.to_string())?;
            }
        }
        let game = GameState::new(map, GridPos::new(1, 2), 1872).map_err(|e| e.to_string())?;
        let before = format!("{game:?}");
        let area = match id {
            "FR06" => Area::Diamond { radius: 1 },
            "FE05" => Area::Disc { radius: 1 },
            _ => Area::Square { radius: 1 },
        };
        let cells = affected_cells(game.map(), GridPos::new(4, 2), area, (1, 0));
        let mut motion = CombatMotion::default();
        motion.prepare_cast(Some(crate::effect_geometry::PreparedCast {
            skill: id.into(),
            origin: GridPos::new(1, 2),
            target: GridPos::new(4, 2),
            cells: cells.clone(),
        }));
        if id == "FE05" {
            motion.play_fireball(GridPos::new(1, 2), GridPos::new(4, 2), 1, 0.);
        } else {
            motion.play_agent_magic(id, GridPos::new(1, 2), GridPos::new(4, 2), 1, 0.);
        }
        let sheet = &sheets[match id {
            "FR06" => 0,
            "FE05" => 1,
            _ => 2,
        }];
        // A material stores only a GPU handle; retain its Texture2D owner for
        // the entire sequence, otherwise another uploaded mask can reuse it.
        let floor_mask = mask(&game);
        clip.set_texture("floorMask", floor_mask.clone());
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
                        scene(&game, &motion, id, time, dark, true);
                    } else {
                        proposed(&game, &motion, id, time, dark, sheet, &clip);
                    }
                }
                set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
                clear_background(Color::from_rgba(13, 18, 20, 255));
                for (panel, stage) in stages.iter().enumerate() {
                    let x = 16. + panel as f32 * 464.;
                    draw_text(
                        if panel == 0 {
                            "Découpe actuelle"
                        } else {
                            "Dessin entier · débordement naturel"
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
                    return Err("Dimensions natives de capture incorrectes".into());
                }
                for (panel, sheet) in frames.iter_mut().enumerate() {
                    copy_panel(&image, sheet, frame, panel);
                }
                if frame == if id == "FE05" { 36 } else { 12 } && dark {
                    image.export_png(
                        output
                            .join(format!("{:02}-reference.png", index + 1))
                            .to_str()
                            .ok_or("capture path")?,
                    );
                }
                next_frame().await;
            }
            for (panel, sheet) in frames.iter().enumerate() {
                std::fs::write(
                    output.join(format!("{:02}-{light}-{panel}.rgba", index + 1)),
                    &sheet.bytes,
                )
                .map_err(|e| e.to_string())?;
            }
            println!("ANIM-FLOW-01 {id} {light} wall={wall}: {FRAMES} phases");
        }
        if before != format!("{game:?}") {
            return Err("La revue a modifié l'état moteur".into());
        }
        let excluded: Vec<_> = (0..5)
            .flat_map(|y| (0..7).map(move |x| GridPos::new(x, y)))
            .filter(|at| !allowed(&game, *at))
            .map(|p| [p.x, p.y])
            .collect();
        rows.push(serde_json::json!({"number":index+1,"id":id,"title":title,"seconds":seconds,"duration_scale":scale,"physical_wall":wall,"affected_cells":cells.iter().map(|p|[p.x,p.y]).collect::<Vec<_>>(),"excluded_visual_cells":excluded}));
    }
    std::fs::write(output.join("capture.json"), serde_json::to_vec_pretty(&serde_json::json!({
        "review":"ANIM-FLOW-01","status":"proposal_pending_player_review","native_tile":64,
        "frame_width":WIDTH,"frame_height":HEIGHT,"frames":FRAMES,"columns":COLUMNS,"fps":FPS,
        "visual_overhang_factor":1.12,"same_phase_and_intensity":true,"game_state_unchanged":true,"normal_game_activation":false,"cases":rows
    })).map_err(|e| e.to_string())?).map_err(|e| e.to_string())?;
    Ok(())
}
