//! ANIM-OCC02: a continuous visual current over the unchanged discrete footprint.
//! World-space obstacle rays replace the rejected per-affected-cell visual clip.
use super::*;

const VARIANTS: [&str; 3] = ["FE02-A04-COULOIR", "FE02-A04-CLOISON", "FE02-A04-OBSTACLE"];
const SIZE: u16 = 448;
const EMITTER: Vec2 = Vec2::new(114.0, 219.0);
const FRAGMENT: &str = r#"#version 100
precision highp float;
varying highp vec2 point;
varying highp vec2 uv;
varying lowp vec4 tint;
uniform sampler2D Texture;
uniform sampler2D continuousField;
uniform sampler2D floorMask;
uniform vec2 stageOrigin;
uniform vec2 paintOrigin;
uniform float paintFrame;
uniform float flowAge;
void main() {
    vec2 world = point - stageOrigin;
    if (world.x < 0.0 || world.y < 0.0 || world.x >= 448.0 || world.y >= 448.0) discard;
    vec2 cell = floor(world / 64.0);
    float floorAllowed = texture2D(floorMask, (cell+vec2(0.5))/7.0).r;
    if (floorAllowed < 0.5) discard;
    vec2 fieldUV = world/448.0;
    vec4 field = texture2D(continuousField, fieldUV);
    vec2 bend = (field.gb*2.0-1.0)*18.0;
    vec2 local = (point-paintOrigin)/384.0;
    float ripple = sin(flowAge*7.0-local.x*13.0+local.y*4.0);
    // Drawn masses bend near the nearest physical surface, without cell-edge cuts.
    local += bend*(0.65+0.35*ripple)/384.0;
    local.y += sin(flowAge*5.0-local.x*16.0)*0.007*local.x;
    if (local.x < 0.0 || local.y < 0.0 || local.x >= 1.0 || local.y >= 1.0) discard;
    // Gentle motion at a shadow boundary; solid terrain is always discarded above.
    vec2 drift = vec2(1.0, ripple*1.5)/448.0;
    float visibility = texture2D(continuousField, fieldUV+drift).r;
    vec4 flame = texture2D(Texture, vec2((paintFrame+local.x)/8.0, local.y))*tint;
    gl_FragColor = vec4(flame.rgb, flame.a*visibility);
}"#;

fn wall_rectangles(map: &Map) -> Vec<Rect> {
    (0..7)
        .flat_map(|y| (0..7).map(move |x| GridPos::new(x, y)))
        .filter(|&at| map.blocks_vision(at))
        .map(|at| Rect::new(at.x as f32 * TILE, at.y as f32 * TILE, TILE, TILE))
        .collect()
}

/// Exact segment/rectangle intersection; a closed corner also blocks the ray.
fn hits_rectangle(start: Vec2, end: Vec2, wall: Rect) -> bool {
    let delta = end - start;
    let mut near = 0.0_f32;
    let mut far = 1.0_f32;
    for (origin, direction, lower, upper) in [
        (start.x, delta.x, wall.x, wall.x + wall.w),
        (start.y, delta.y, wall.y, wall.y + wall.h),
    ] {
        if direction.abs() < 0.0001 {
            if origin < lower || origin > upper {
                return false;
            }
        } else {
            let a = (lower - origin) / direction;
            let b = (upper - origin) / direction;
            near = near.max(a.min(b));
            far = far.min(a.max(b));
            if far < near {
                return false;
            }
        }
    }
    far >= near && far > 0.0 && near < 1.0
}

fn nearest_surface(point: Vec2, walls: &[Rect]) -> (f32, Vec2) {
    let mut distance = f32::MAX;
    let mut normal = Vec2::ZERO;
    for wall in walls {
        let nearest = vec2(
            point.x.clamp(wall.x, wall.x + wall.w),
            point.y.clamp(wall.y, wall.y + wall.h),
        );
        let delta = point - nearest;
        let length = delta.length();
        if length < distance {
            distance = length;
            normal = delta.normalize_or_zero();
        }
    }
    (distance, normal)
}

struct Field {
    image: Image,
    floor: Image,
}
impl Field {
    fn new(fixture: &Fixture) -> Self {
        let walls = wall_rectangles(&fixture.map);
        let mut image = Image::gen_image_color(SIZE, SIZE, BLACK);
        let mut floor = Image::gen_image_color(7, 7, BLACK);
        for y in 0..7 {
            for x in 0..7 {
                if !fixture.map.blocks_vision(GridPos::new(x, y)) {
                    floor.set_pixel(x as u32, y as u32, WHITE);
                }
            }
        }
        for y in 0..u32::from(SIZE) {
            for x in 0..u32::from(SIZE) {
                let point = vec2(x as f32 + 0.5, y as f32 + 0.5);
                let blocked = fixture
                    .map
                    .blocks_vision(GridPos::new(x as i32 / 64, y as i32 / 64));
                let visible = !blocked
                    && !walls
                        .iter()
                        .any(|&wall| hits_rectangle(EMITTER, point, wall));
                let (distance, normal) = if blocked {
                    (0.0, Vec2::ZERO)
                } else {
                    nearest_surface(point, &walls)
                };
                let edge = (distance / 5.0).clamp(0.0, 1.0);
                let opacity = if visible {
                    edge * edge * (3.0 - 2.0 * edge)
                } else {
                    0.0
                };
                let bend = normal * (1.0 - distance / 42.0).clamp(0.0, 1.0);
                let offset = (y as usize * usize::from(SIZE) + x as usize) * 4;
                image.bytes[offset..offset + 4].copy_from_slice(&[
                    (opacity * 255.0).round() as u8,
                    ((bend.x * 0.5 + 0.5) * 255.0).round() as u8,
                    ((bend.y * 0.5 + 0.5) * 255.0).round() as u8,
                    255,
                ]);
            }
        }
        Self { image, floor }
    }
}

struct Current {
    fire: DrawnFire,
    fixtures: Vec<Fixture>,
    fields: Vec<Field>,
    textures: Vec<(Texture2D, Texture2D)>,
    material: Material,
}
impl Current {
    fn load() -> Result<Self, String> {
        let fixtures: Vec<_> = (0..3).map(Fixture::new).collect();
        let fields: Vec<_> = fixtures.iter().map(Field::new).collect();
        let textures = fields
            .iter()
            .map(|field| {
                let texture = Texture2D::from_image(&field.image);
                texture.set_filter(FilterMode::Linear);
                let floor = Texture2D::from_image(&field.floor);
                floor.set_filter(FilterMode::Nearest);
                (texture, floor)
            })
            .collect();
        let material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX,
                fragment: FRAGMENT,
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("stageOrigin", UniformType::Float2),
                    UniformDesc::new("paintOrigin", UniformType::Float2),
                    UniformDesc::new("paintFrame", UniformType::Float1),
                    UniformDesc::new("flowAge", UniformType::Float1),
                ],
                textures: vec!["continuousField".into(), "floorMask".into()],
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .map_err(|error| error.to_string())?;
        Ok(Self {
            fire: DrawnFire::load(),
            fixtures,
            fields,
            textures,
            material,
        })
    }
    fn torrent(&self, scenario: usize, rect: Rect, age: f32) {
        let emitter = vec2(rect.x, rect.y) + EMITTER;
        self.material.set_uniform("stageOrigin", [rect.x, rect.y]);
        self.material
            .set_uniform("paintOrigin", [emitter.x - 24.0, emitter.y - 192.0]);
        self.material.set_uniform(
            "paintFrame",
            if age < 0.083 {
                0.0
            } else {
                (1 + (age * 12.0) as u32 % 7) as f32
            },
        );
        self.material.set_uniform("flowAge", age);
        self.material
            .set_texture("continuousField", self.textures[scenario].0.clone());
        self.material
            .set_texture("floorMask", self.textures[scenario].1.clone());
        gl_use_material(&self.material);
        self.fire.torrent(emitter, age, 1.0);
        gl_use_default_material();
    }
}

fn scene(current: &Current, scenario: usize, rect: Rect, age: f32, show_effect: bool) {
    let fixture = &current.fixtures[scenario];
    terrain(fixture, rect);
    let aiming = age < 0.7;
    if aiming {
        for at in &fixture.affected {
            let p = cell(rect, at.x as f32, at.y as f32);
            draw_rectangle(
                p.x + 2.0,
                p.y + 2.0,
                60.0,
                60.0,
                Color::new(0.9, 0.45, 0.06, 0.18),
            );
            draw_rectangle_lines(
                p.x + 2.0,
                p.y + 2.0,
                60.0,
                60.0,
                1.0,
                Color::new(0.9, 0.55, 0.12, 0.65),
            );
        }
    } else if show_effect {
        current.torrent(scenario, rect, age - 0.7);
    }
    for (index, &at) in fixture.targets.iter().enumerate() {
        let p = cell(rect, at.x as f32 + 0.5, at.y as f32 + 0.5);
        enemy(p);
        if !aiming && show_effect && fixture.affected.contains(&at) {
            current.fire.burn(p, age - 0.7, index as u32 * 2, 0.9);
        }
        draw_text_bold(
            char::from(b'A' + index as u8).to_string(),
            p.x - 4.0,
            p.y - 28.0,
            13.0,
            TEXT,
        );
    }
    let player = cell(rect, 1.5, 3.5);
    actor(player, caster());
    if !aiming {
        name(0, player, age - 0.7);
    }
    draw_rectangle(
        rect.x + 7.0,
        rect.y + rect.h - 28.0,
        rect.w - 14.0,
        22.0,
        alpha(INK, 0.94),
    );
    draw_text(
        if aiming {
            "Aperçu de ciblage · cases touchées"
        } else {
            SCENARIOS[scenario].2
        },
        rect.x + 14.0,
        rect.y + rect.h - 12.0,
        14.0,
        TEXT,
    );
}

pub(crate) async fn run() {
    request_new_screen_size(1060.0, 670.0);
    let current = Current::load().expect("continuous visual current proposal");
    let mut scenario = 0;
    let mut age = 0.0;
    let mut paused = false;
    let mut speed = 1.0;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold("LABORATOIRE / SOUFFLE CONTINU", 22.0, 35.0, 23.0, TEXT);
        draw_text(
            "ANIM-OCC02 · rendu continu proposé · cases de 64 px",
            22.0,
            62.0,
            14.0,
            DIM,
        );
        for (index, (_, title, _)) in SCENARIOS.iter().enumerate() {
            if button(
                Rect::new(20.0, 96.0 + index as f32 * 46.0, 270.0, 35.0),
                title,
                true,
            ) {
                scenario = index;
                age = 0.0;
            }
        }
        scene(
            &current,
            scenario,
            Rect::new(320.0, 96.0, 448.0, 448.0),
            age,
            true,
        );
        draw_text(VARIANTS[scenario], 320.0, 567.0, 14.0, DIM);
        if button(Rect::new(20.0, 596.0, 115.0, 34.0), "Rejouer", true)
            || is_key_pressed(KeyCode::R)
        {
            age = 0.0;
        }
        if button(
            Rect::new(145.0, 596.0, 115.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if button(
            Rect::new(270.0, 596.0, 135.0, 34.0),
            if speed == 1.0 {
                "Vitesse ×1"
            } else {
                "Vitesse ×0,5"
            },
            true,
        ) {
            speed = if speed == 1.0 { 0.5 } else { 1.0 };
        }
        draw_text(
            "Aperçu puis lancement · zone touchée inchangée · aucune avance de tour",
            22.0,
            655.0,
            14.0,
            DIM,
        );
        if !paused {
            age += get_frame_time() * speed;
        }
        next_frame().await;
    }
}

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    if output.exists() {
        return Err("Le dossier de revue existe déjà.".into());
    }
    std::fs::create_dir_all(output).map_err(|error| error.to_string())?;
    request_new_screen_size(512.0, 512.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let current = Current::load()?;
    let mut entries = Vec::new();
    for (scenario, variant) in VARIANTS.iter().enumerate() {
        let directory = output.join(variant);
        std::fs::create_dir(&directory).map_err(|error| error.to_string())?;
        current.fields[scenario]
            .image
            .export_png(directory.join("continuous-field.png").to_str().unwrap());
        for frame in 0..120 {
            clear_background(INK);
            let age = frame as f32 / 30.0;
            let rect = Rect::new(24.0, 24.0, 448.0, 448.0);
            scene(&current, scenario, rect, age, true);
            crate::ui_capture::framebuffer()?
                .export_png(directory.join(format!("{frame:03}.png")).to_str().unwrap());
            if frame == 60 {
                clear_background(INK);
                scene(&current, scenario, rect, age, false);
                crate::ui_capture::framebuffer()?
                    .export_png(directory.join("control.png").to_str().unwrap());
            }
            next_frame().await;
        }
        let fixture = &current.fixtures[scenario];
        entries.push(serde_json::json!({"id":"FE02","number":10,"variant":variant,
            "title":SCENARIOS[scenario].1,"tile":64,"folder":variant,"frames":120,"fps":30,
            "framebuffer_size":[512,512],"crop":[24,24,448,448],"map":MAPS[scenario],
            "origin":[ORIGIN.x,ORIGIN.y],"affected_cells":fixture.affected.iter().map(|p| [p.x,p.y]).collect::<Vec<_>>(),
            "targets":fixture.targets.iter().enumerate().map(|(i,p)| serde_json::json!({
                "label":char::from(b'A'+i as u8).to_string(),"cell":[p.x,p.y],
                "affected":fixture.affected.contains(p)})).collect::<Vec<_>>(),
            "control_frame":60,"visual_mask":"Continuous pixel rays and solid terrain, independent of affected cell boundaries"}));
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn continuous_ray_keeps_full_wall_and_obstacle_shadow_blocking() {
        assert!(hits_rectangle(
            EMITTER,
            vec2(352.0, 224.0),
            Rect::new(192.0, 192.0, 64.0, 64.0)
        ));
        assert!(!hits_rectangle(
            EMITTER,
            vec2(224.0, 160.0),
            Rect::new(192.0, 192.0, 64.0, 64.0)
        ));
        assert!(hits_rectangle(
            EMITTER,
            vec2(352.0, 160.0),
            Rect::new(256.0, 64.0, 64.0, 320.0)
        ));
    }
    #[test]
    fn continuous_field_follows_obstacle_projection_inside_each_cell() {
        let fixture = Fixture::new(2);
        assert!(!fixture.affected.contains(&GridPos::new(4, 3)));
        let field = Field::new(&fixture);
        // The target stays protected. Two pixels inside the same floor cell have
        // different visibility: the visual shadow follows a diagonal ray boundary.
        let at = |x: usize, y: usize| field.image.bytes[(y * usize::from(SIZE) + x) * 4];
        assert_eq!(at(352, 224), 0);
        assert!(at(262, 140) > 0);
        assert_eq!(at(262, 185), 0);
        assert_eq!(at(220, 220), 0);
    }
}
