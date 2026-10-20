//! ANIM-OCC01: terrain clipping proposal using unchanged painted fire frames.
//! Fixture geometry and GPU mask are review-only, never campaign simulation.
use super::*;
use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Equation, PipelineParams, UniformDesc, UniformType,
};
use project_rl::world::{Map, has_line_of_sight};

pub(crate) mod fluid;
pub(crate) mod overlay;
pub(crate) mod particles;

const ORIGIN: GridPos = GridPos::new(1, 3);
// Frozen ANIM-OCC01 geometry: FE02 was still a cone at effective rank 11.
// Keep the old proposal reproducible independently of today's corridor skill.
const REVIEW_CONE_RANGE: i32 = 6;
const SCENARIOS: [(&str, &str, &str); 3] = [
    (
        "FE02-A03-COULOIR",
        "Couloir puis pièce",
        "A, B, C exposés · D protégé",
    ),
    (
        "FE02-A03-CLOISON",
        "Cloison devant",
        "A exposé · B, C protégés",
    ),
    (
        "FE02-A03-OBSTACLE",
        "Obstacle dans le cône",
        "A, B exposés · C protégé",
    ),
];
const MAPS: [&str; 3] = [
    "#######\n###...#\n###...#\n#.....#\n###...#\n###...#\n#######",
    "#######\n#...#.#\n#...#.#\n#...#.#\n#...#.#\n#...#.#\n#######",
    "#######\n#.....#\n#.....#\n#..#..#\n#.....#\n#.....#\n#######",
];
const VERTEX: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
uniform mat4 Model;
uniform mat4 Projection;
varying highp vec2 point;
varying highp vec2 uv;
varying lowp vec4 tint;
void main() {
    point = position.xy;
    uv = texcoord;
    tint = color0 / 255.0;
    gl_Position = Projection * Model * vec4(position, 1.0);
}"#;
const FRAGMENT: &str = r#"#version 100
precision highp float;
varying highp vec2 point;
varying highp vec2 uv;
varying lowp vec4 tint;
uniform sampler2D Texture;
uniform sampler2D effectMask;
uniform vec2 stageOrigin;
float allowed(vec2 cell) {
    if (cell.x < 0.0 || cell.y < 0.0 || cell.x >= 7.0 || cell.y >= 7.0) return 0.0;
    return texture2D(effectMask, (cell + vec2(0.5)) / 7.0).r;
}
void main() {
    vec2 grid = (point - stageOrigin) / 64.0;
    vec2 cell = floor(grid);
    vec2 part = fract(grid);
    float visibility = allowed(cell);
    // A two-pixel inward feather softens clipping, never leaking onto blocked cells.
    if (allowed(cell + vec2(-1.0, 0.0)) < 0.5) visibility *= smoothstep(0.0, 0.035, part.x);
    if (allowed(cell + vec2(1.0, 0.0)) < 0.5) visibility *= smoothstep(0.0, 0.035, 1.0-part.x);
    if (allowed(cell + vec2(0.0, -1.0)) < 0.5) visibility *= smoothstep(0.0, 0.035, part.y);
    if (allowed(cell + vec2(0.0, 1.0)) < 0.5) visibility *= smoothstep(0.0, 0.035, 1.0-part.y);
    vec4 flame = texture2D(Texture, uv) * tint;
    gl_FragColor = vec4(flame.rgb, flame.a * visibility);
}"#;

struct Fixture {
    map: Map,
    affected: Vec<GridPos>,
    targets: Vec<GridPos>,
}
impl Fixture {
    fn new(scenario: usize) -> Self {
        let map = Map::from_ascii(MAPS[scenario]).unwrap();
        let range = REVIEW_CONE_RANGE;
        let affected = (0..7)
            .flat_map(|y| (0..7).map(move |x| GridPos::new(x, y)))
            .filter(|&at| {
                let dx = at.x - ORIGIN.x;
                let dy = (at.y - ORIGIN.y).abs();
                // Discrete forward 90-degree cone. Exact footprint is proposed here.
                dx > 0
                    && dx <= range
                    && dy <= dx
                    && map.is_walkable(at)
                    && has_line_of_sight(&map, ORIGIN, at, true)
            })
            .collect();
        let coordinates: &[(i32, i32)] = match scenario {
            0 => &[(2, 3), (4, 2), (4, 4), (3, 1)],
            1 => &[(3, 3), (5, 3), (5, 2)],
            _ => &[(3, 2), (3, 4), (5, 3)],
        };
        Self {
            map,
            affected,
            targets: coordinates
                .iter()
                .map(|&(x, y)| GridPos::new(x, y))
                .collect(),
        }
    }
    fn mask_image(&self) -> Image {
        let mut image = Image::gen_image_color(7, 7, BLACK);
        // The caster's tile contains only the emitter; it is excluded from affected.
        for at in self.affected.iter().copied().chain([ORIGIN]) {
            image.set_pixel(at.x as u32, at.y as u32, WHITE);
        }
        image
    }
}

struct ClippedFire {
    fire: DrawnFire,
    material: Material,
    masks: Vec<Texture2D>,
    fixtures: Vec<Fixture>,
}
impl ClippedFire {
    fn load() -> Result<Self, String> {
        let fixtures: Vec<_> = (0..3).map(Fixture::new).collect();
        let masks = fixtures
            .iter()
            .map(|fixture| {
                let texture = Texture2D::from_image(&fixture.mask_image());
                texture.set_filter(FilterMode::Nearest);
                texture
            })
            .collect();
        let material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX,
                fragment: FRAGMENT,
            },
            MaterialParams {
                uniforms: vec![UniformDesc::new("stageOrigin", UniformType::Float2)],
                textures: vec!["effectMask".to_owned()],
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
            material,
            masks,
            fixtures,
        })
    }
    fn torrent(&self, scenario: usize, rect: Rect, age: f32) {
        self.material.set_uniform("stageOrigin", [rect.x, rect.y]);
        self.material
            .set_texture("effectMask", self.masks[scenario].clone());
        gl_use_material(&self.material);
        self.fire
            .torrent(cell(rect, 1.5, 3.5) + vec2(18.0, -5.0), age, 1.0);
        gl_use_default_material();
    }
}

fn terrain(fixture: &Fixture, rect: Rect) {
    for y in 0..7 {
        for x in 0..7 {
            let at = GridPos::new(x, y);
            let p = cell(rect, x as f32, y as f32);
            game_art::draw_world(
                Rect::new(p.x, p.y, TILE, TILE),
                Visual::Terrain {
                    kind: if fixture.map.blocks_vision(at) {
                        crate::test_sector::Decor::Wall
                    } else {
                        crate::test_sector::Decor::Deck
                    },
                    position: at,
                },
                1.0,
            );
            draw_rectangle_lines(
                p.x,
                p.y,
                TILE,
                TILE,
                0.7,
                Color::new(0.15, 0.18, 0.19, 0.75),
            );
        }
    }
}

fn scene(review: &ClippedFire, scenario: usize, rect: Rect, age: f32, show_effect: bool) {
    let fixture = &review.fixtures[scenario];
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
        review.torrent(scenario, rect, age - 0.7);
    }
    for (index, &at) in fixture.targets.iter().enumerate() {
        let p = cell(rect, at.x as f32 + 0.5, at.y as f32 + 0.5);
        enemy(p);
        if !aiming && show_effect && fixture.affected.contains(&at) {
            review.fire.burn(p, age - 0.7, index as u32 * 2, 0.9);
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
    // Review labels, never a new player status or a reveal of hidden campaign actors.
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
    let review = ClippedFire::load().expect("material for isolated obstacle review");
    let mut scenario = 0;
    let mut age = 0.0;
    let mut paused = false;
    let mut speed = 1.0;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold("LABORATOIRE / SOUFFLE ET OBSTACLES", 22.0, 35.0, 23.0, TEXT);
        draw_text(
            "ANIM-OCC01 · proposition · cases natives de 64 px",
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
            &review,
            scenario,
            Rect::new(320.0, 96.0, 448.0, 448.0),
            age,
            true,
        );
        draw_text(SCENARIOS[scenario].0, 320.0, 567.0, 14.0, DIM);
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
            "Aperçu puis lancement · aucune attente ajoutée au sort · aucun tour simulé",
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
    let review = ClippedFire::load()?;
    let mut entries = Vec::new();
    for (scenario, &(variant, title, _)) in SCENARIOS.iter().enumerate() {
        let directory = output.join(variant);
        std::fs::create_dir(&directory).map_err(|error| error.to_string())?;
        let fixture = &review.fixtures[scenario];
        fixture
            .mask_image()
            .export_png(directory.join("mask.png").to_str().unwrap());
        for frame in 0..120 {
            clear_background(INK);
            let age = frame as f32 / 30.0;
            let rect = Rect::new(24.0, 24.0, 448.0, 448.0);
            scene(&review, scenario, rect, age, true);
            crate::ui_capture::framebuffer()?
                .export_png(directory.join(format!("{frame:03}.png")).to_str().unwrap());
            if frame == 60 {
                clear_background(INK);
                scene(&review, scenario, rect, age, false);
                crate::ui_capture::framebuffer()?
                    .export_png(directory.join("control.png").to_str().unwrap());
            }
            next_frame().await;
        }
        let affected: Vec<_> = fixture.affected.iter().map(|p| [p.x, p.y]).collect();
        let targets: Vec<_> = fixture
            .targets
            .iter()
            .enumerate()
            .map(|(i, p)| {
                serde_json::json!({
                    "label": char::from(b'A'+i as u8).to_string(), "cell":[p.x,p.y],
                    "affected":fixture.affected.contains(p)
                })
            })
            .collect();
        entries.push(
            serde_json::json!({"id":"FE02", "number":10,"variant":variant,
            "title":title,"tile":64,"folder":variant,"frames":120,"fps":30,
            "framebuffer_size":[512,512],"crop":[24,24,448,448],"map":MAPS[scenario],
            "origin":[ORIGIN.x,ORIGIN.y],"affected_cells":affected,"targets":targets,
            "control_frame":60,"mask":"mask.png"}),
        );
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
    fn corridor_widens_after_opening_but_closed_corner_protects_target() {
        let fixture = Fixture::new(0);
        for at in [GridPos::new(2, 3), GridPos::new(4, 2), GridPos::new(4, 4)] {
            assert!(fixture.affected.contains(&at));
        }
        assert!(!fixture.affected.contains(&GridPos::new(3, 1)));
    }
    #[test]
    fn full_cloison_stops_all_cells_behind_it() {
        let fixture = Fixture::new(1);
        assert!(fixture.affected.contains(&GridPos::new(3, 3)));
        assert!(fixture.affected.iter().all(|p| p.x < 4));
    }
    #[test]
    fn partial_obstacle_leaves_two_sides_but_blocks_center() {
        let fixture = Fixture::new(2);
        assert!(fixture.affected.contains(&GridPos::new(3, 2)));
        assert!(fixture.affected.contains(&GridPos::new(3, 4)));
        assert!(!fixture.affected.contains(&GridPos::new(5, 3)));
        assert!(!fixture.affected.contains(&GridPos::new(3, 3)));
    }
}
