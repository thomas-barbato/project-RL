//! ANIM-OVER02: painted fire stops at physical obstacles, preview ends on cast.
//! Burn flames have their own lifetime and can affect either actor in the fixture.
use super::*;
pub(crate) mod expanded;

const CASE_IDS: [&str; 5] = ["COULOIR", "CLOISON", "OBSTACLE", "LONG", "LONG-CLOISON"];
const CASE_LABELS: [&str; 5] = [
    "Couloir puis pièce",
    "Cloison devant",
    "Obstacle partiel",
    "Couloir long",
    "Couloir long avec cloison",
];
const FPS: u32 = 24;
const FRAMES: u32 = 72;
const TARGETING_END: f32 = 0.7;
const BREATH_END: f32 = 1.8;
const BURN_END: f32 = 2.7;
const EMITTER: Vec2 = Vec2::new(114.0, 219.0);
const STOP_FRAGMENT: &str = r#"#version 100
precision highp float;
varying highp vec2 point;
varying highp vec2 uv;
varying lowp vec4 tint;
uniform sampler2D Texture;
uniform sampler2D visibilityMask;
uniform vec2 stageOrigin;
uniform vec2 stageSize;
uniform float aimDirection;
void main() {
    vec2 local = point - stageOrigin;
    vec2 canonical = local;
    if (aimDirection > 0.5 && aimDirection < 1.5) canonical = stageSize - local;
    else if (aimDirection > 1.5 && aimDirection < 2.5) canonical = vec2(stageSize.x-local.y,local.x);
    else if (aimDirection > 2.5) canonical = vec2(local.y,stageSize.y-local.x);
    if (canonical.x < 0.0 || canonical.y < 0.0 || canonical.x >= stageSize.x || canonical.y >= stageSize.y) discard;
    float visibility = texture2D(visibilityMask,canonical/stageSize).r;
    vec4 flame = texture2D(Texture,uv)*tint;
    gl_FragColor = vec4(flame.rgb,flame.a*visibility);
}"#;

struct Stage {
    fixture: Fixture,
    width: i32,
    height: i32,
    map_text: String,
}
impl Stage {
    fn new(case: usize) -> Self {
        if case < 3 {
            return Self {
                fixture: Fixture::new(case),
                width: 7,
                height: 7,
                map_text: MAPS[case].into(),
            };
        }
        let mut rows = vec!["#############"; 7];
        rows[3] = if case == 3 {
            "#...........#"
        } else {
            "#...#.......#"
        };
        let map_text = rows.join("\n");
        let map = Map::from_ascii(&map_text).unwrap();
        // Keep the accepted five-cell range; distant actors demonstrate its limit.
        let range = i32::from(parameter_for("FE02", "cone_range_cells", 11));
        let affected = (1..13)
            .map(|x| GridPos::new(x, 3))
            .filter(|&at| {
                at.x > ORIGIN.x
                    && at.x - ORIGIN.x <= range
                    && map.is_walkable(at)
                    && has_line_of_sight(&map, ORIGIN, at, true)
            })
            .collect();
        let targets = if case == 3 {
            vec![GridPos::new(2, 3), GridPos::new(5, 3), GridPos::new(9, 3)]
        } else {
            vec![GridPos::new(2, 3), GridPos::new(3, 3), GridPos::new(8, 3)]
        };
        Self {
            fixture: Fixture {
                map,
                affected,
                targets,
            },
            width: 13,
            height: 7,
            map_text,
        }
    }
    fn position(&self, at: GridPos, direction: usize) -> GridPos {
        match direction {
            1 => GridPos::new(self.width - 1 - at.x, self.height - 1 - at.y),
            2 => GridPos::new(at.y, self.width - 1 - at.x),
            3 => GridPos::new(self.height - 1 - at.y, at.x),
            _ => at,
        }
    }
    fn point(&self, at: Vec2, direction: usize) -> Vec2 {
        let w = self.width as f32 * TILE;
        let h = self.height as f32 * TILE;
        match direction {
            1 => vec2(w - at.x, h - at.y),
            2 => vec2(at.y, w - at.x),
            3 => vec2(h - at.y, at.x),
            _ => at,
        }
    }
    fn size(&self, direction: usize) -> Vec2 {
        if direction < 2 {
            vec2(self.width as f32 * TILE, self.height as f32 * TILE)
        } else {
            vec2(self.height as f32 * TILE, self.width as f32 * TILE)
        }
    }
    fn walls(&self) -> Vec<Rect> {
        (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| GridPos::new(x, y)))
            .filter(|&at| self.fixture.map.blocks_vision(at))
            .map(|at| Rect::new(at.x as f32 * TILE, at.y as f32 * TILE, TILE, TILE))
            .collect()
    }
}

fn intersects(start: Vec2, end: Vec2, wall: Rect) -> bool {
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
fn clear_ray(point: Vec2, walls: &[Rect]) -> bool {
    !walls.iter().any(|&wall| intersects(EMITTER, point, wall))
}
fn visibility(stage: &Stage) -> Image {
    let walls = stage.walls();
    let size = stage.size(0);
    let mut image = Image::gen_image_color(size.x as u16, size.y as u16, BLACK);
    for y in 0..image.height as u32 {
        for x in 0..image.width as u32 {
            if clear_ray(vec2(x as f32 + 0.5, y as f32 + 0.5), &walls) {
                image.set_pixel(x, y, WHITE);
            }
        }
    }
    image
}
struct Stopped {
    fire: DrawnFire,
    stages: Vec<Stage>,
    masks: Vec<Texture2D>,
    material: Material,
    expand_exit: bool,
}
impl Stopped {
    fn load(expand_exit: bool) -> Result<Self, String> {
        let stages: Vec<_> = (0..5).map(Stage::new).collect();
        let masks = stages
            .iter()
            .map(|stage| {
                let image = if expand_exit {
                    expanded::visibility(stage)
                } else {
                    visibility(stage)
                };
                let texture = Texture2D::from_image(&image);
                texture.set_filter(FilterMode::Nearest);
                texture
            })
            .collect();
        let material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX,
                fragment: STOP_FRAGMENT,
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("stageOrigin", UniformType::Float2),
                    UniformDesc::new("stageSize", UniformType::Float2),
                    UniformDesc::new("aimDirection", UniformType::Float1),
                ],
                textures: vec!["visibilityMask".into()],
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
            stages,
            masks,
            material,
            expand_exit,
        })
    }
    fn torrent(&self, case: usize, direction: usize, rect: Rect, age: f32) {
        let stage = &self.stages[case];
        self.material.set_uniform("stageOrigin", [rect.x, rect.y]);
        self.material.set_uniform(
            "stageSize",
            [stage.width as f32 * TILE, stage.height as f32 * TILE],
        );
        self.material.set_uniform("aimDirection", direction as f32);
        self.material
            .set_texture("visibilityMask", self.masks[case].clone());
        gl_use_material(&self.material);
        let center = stage.point(vec2(282.0, 219.0), direction) + vec2(rect.x, rect.y);
        let rotation = match direction {
            1 => std::f32::consts::PI,
            2 => -std::f32::consts::FRAC_PI_2,
            3 => std::f32::consts::FRAC_PI_2,
            _ => 0.0,
        };
        let frame = if age < 0.083 {
            0
        } else {
            1 + (age * 12.0) as u32 % 7
        };
        draw_texture_ex(
            &self.fire.cone,
            center.x - 192.0,
            center.y - 192.0,
            alpha(WHITE, 0.95),
            DrawTextureParams {
                source: Some(Rect::new(frame as f32 * 384.0, 0.0, 384.0, 384.0)),
                dest_size: Some(vec2(384.0, 384.0)),
                rotation,
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
}
fn terrain_stage(stage: &Stage, direction: usize, rect: Rect, walls_only: bool) {
    for y in 0..stage.height {
        for x in 0..stage.width {
            let source = GridPos::new(x, y);
            let wall = stage.fixture.map.blocks_vision(source);
            if walls_only && !wall {
                continue;
            }
            let at = stage.position(source, direction);
            let p = cell(rect, at.x as f32, at.y as f32);
            draw_rectangle(p.x, p.y, TILE, TILE, INK);
            game_art::draw_world(
                Rect::new(p.x, p.y, TILE, TILE),
                Visual::Terrain {
                    kind: if wall {
                        crate::test_sector::Decor::Wall
                    } else {
                        crate::test_sector::Decor::Deck
                    },
                    position: source,
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
fn targeting(stage: &Stage, direction: usize, rect: Rect, clock: f32) {
    let strength = blink(clock);
    for &source in &stage.fixture.affected {
        let at = stage.position(source, direction);
        let p = cell(rect, at.x as f32, at.y as f32);
        draw_rectangle(
            p.x + 2.0,
            p.y + 2.0,
            60.0,
            60.0,
            Color::new(0.03, 0.8, 0.98, 0.12 + 0.20 * strength),
        );
        draw_rectangle_lines(
            p.x + 2.0,
            p.y + 2.0,
            60.0,
            60.0,
            7.0,
            Color::new(0.01, 0.025, 0.04, 0.96),
        );
        draw_rectangle_lines(
            p.x + 2.0,
            p.y + 2.0,
            60.0,
            60.0,
            4.0,
            Color::new(0.10, 0.95, 1.0, strength),
        );
        draw_rectangle_lines(
            p.x + 4.0,
            p.y + 4.0,
            56.0,
            56.0,
            1.0,
            Color::new(0.78, 1.0, 1.0, strength),
        );
    }
}
fn scene(
    current: &Stopped,
    case: usize,
    direction: usize,
    rect: Rect,
    clock: f32,
    cast_age: Option<f32>,
    effects: bool,
) {
    let stage = &current.stages[case];
    terrain_stage(stage, direction, rect, false);
    if let Some(age) = cast_age {
        if effects && age < BREATH_END - TARGETING_END {
            current.torrent(case, direction, rect, age);
        }
        if effects && age < BURN_END - TARGETING_END {
            for (index, &source) in stage.fixture.targets.iter().enumerate() {
                if stage.fixture.affected.contains(&source) {
                    let at = stage.position(source, direction);
                    current.fire.burn(
                        cell(rect, at.x as f32 + 0.5, at.y as f32 + 0.5),
                        age,
                        index as u32 * 2,
                        0.9,
                    );
                }
            }
        }
    } else {
        targeting(stage, direction, rect, clock);
    }
    terrain_stage(stage, direction, rect, true);
    for (index, &source) in stage.fixture.targets.iter().enumerate() {
        let at = stage.position(source, direction);
        let p = cell(rect, at.x as f32 + 0.5, at.y as f32 + 0.5);
        enemy(p);
        if effects && stage.fixture.affected.contains(&source) {
            if let Some(age) = cast_age.filter(|&age| age < BURN_END - TARGETING_END) {
                burn_on_body(&current.fire, p, age, index as u32 * 2);
            }
        }
        draw_text_bold(
            char::from(b'A' + index as u8).to_string(),
            p.x - 4.0,
            p.y - 28.0,
            13.0,
            TEXT,
        );
    }
    let origin = stage.position(ORIGIN, direction);
    let player = cell(rect, origin.x as f32 + 0.5, origin.y as f32 + 0.5);
    actor(player, caster());
    if let Some(age) = cast_age {
        name(0, player, age);
    }
    let footer = if cast_age.is_none() {
        "Cases touchées · aperçu clignotant"
    } else if case < 3 {
        SCENARIOS[case].2
    } else if case == 3 {
        "A, B exposés · C hors de portée"
    } else {
        "A, B exposés · C derrière la cloison"
    };
    draw_rectangle(
        rect.x + 7.0,
        rect.y + rect.h - 28.0,
        rect.w - 14.0,
        22.0,
        alpha(INK, 0.94),
    );
    draw_text(footer, rect.x + 14.0, rect.y + rect.h - 12.0, 14.0, TEXT);
}
pub(in crate::agent_skill_lab_review::fire_painted) fn burn_on_body(
    fire: &DrawnFire,
    at: Vec2,
    age: f32,
    phase: u32,
) {
    // Sparse foreground tongues attach the approved flame art to the body.
    // The larger existing flames remain behind its silhouette.
    fire.plume(at + vec2(-7.0, 20.0), age, phase + 1, 0.29, 0.85);
    fire.plume(at + vec2(7.0, 19.0), age, phase + 4, 0.24, 0.70);
}
fn burn_scene(fire: &DrawnFire, rect: Rect, age: f32, effects: bool) {
    super::super::super::base(rect);
    for (index, x) in [2.5, 4.5].into_iter().enumerate() {
        let p = cell(rect, x, 3.5);
        if effects && (TARGETING_END..BURN_END).contains(&age) {
            fire.burn(p, age - TARGETING_END, index as u32 * 2, 0.9);
        }
        if index == 0 {
            actor(p, caster());
        } else {
            enemy(p);
        }
        if effects && (TARGETING_END..BURN_END).contains(&age) {
            burn_on_body(fire, p, age - TARGETING_END, index as u32 * 2);
        }
        draw_text_bold(
            if index == 0 { "Agent" } else { "Monstre" },
            p.x - 24.0,
            p.y - 44.0,
            15.0,
            TEXT,
        );
    }
    draw_text(
        "Brûlure · sans souffle ni dégâts alliés",
        rect.x + 25.0,
        rect.y + rect.h - 15.0,
        15.0,
        TEXT,
    );
}
fn variant(case: usize, direction: usize, expand_exit: bool) -> String {
    format!(
        "FE02-{}-{}-{}",
        if expand_exit { "A08" } else { "A07" },
        CASE_IDS[case],
        DIRECTIONS[direction]
    )
}

pub(crate) async fn run() {
    run_variant(false).await;
}
async fn run_variant(expand_exit: bool) {
    request_new_screen_size(1280.0, 1040.0);
    let current = Stopped::load(expand_exit).expect("review shader");
    let mut case = 0;
    let mut direction = 0;
    let mut clock = 0.0;
    let mut cast_start = None;
    let mut paused = false;
    let mut speed = 1.0;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold("LABORATOIRE / SOUFFLE ET BRÛLURE", 22.0, 35.0, 23.0, TEXT);
        for (index, label) in CASE_LABELS
            .iter()
            .copied()
            .chain(["Brûlure · agent et monstre"])
            .enumerate()
        {
            if button(
                Rect::new(20.0, 88.0 + index as f32 * 43.0, 275.0, 34.0),
                label,
                true,
            ) {
                case = index;
                cast_start = None;
            }
        }
        for (index, label) in LABELS.iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (index % 2) as f32 * 138.0,
                    366.0 + (index / 2) as f32 * 42.0,
                    132.0,
                    34.0,
                ),
                label,
                case < 5,
            ) {
                direction = index;
                cast_start = None;
            }
        }
        if button(Rect::new(20.0, 472.0, 115.0, 34.0), "Lancer", true)
            || is_key_pressed(KeyCode::Enter)
        {
            cast_start = Some(clock);
            paused = false;
        }
        if button(Rect::new(145.0, 472.0, 115.0, 34.0), "Viser", true) {
            cast_start = None;
        }
        if button(
            Rect::new(20.0, 516.0, 115.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if button(
            Rect::new(145.0, 516.0, 135.0, 34.0),
            if speed == 1.0 {
                "Vitesse ×1"
            } else {
                "Vitesse ×0,5"
            },
            true,
        ) {
            speed = if speed == 1.0 { 0.5 } else { 1.0 };
        }
        if case < 5 {
            let size = current.stages[case].size(direction);
            scene(
                &current,
                case,
                direction,
                Rect::new(320.0, 88.0, size.x, size.y),
                clock,
                cast_start.map(|start| clock - start),
                true,
            );
            draw_text(
                variant(case, direction, current.expand_exit),
                320.0,
                950.0,
                14.0,
                DIM,
            );
        } else {
            burn_scene(
                &current.fire,
                Rect::new(320.0, 88.0, 448.0, 448.0),
                cast_start.map_or(0.0, |start| clock - start + TARGETING_END),
                true,
            );
        }
        draw_text(
            "Aperçu avant lancement · feu arrêté aux obstacles · Brûlure attachée aux corps",
            22.0,
            1006.0,
            14.0,
            DIM,
        );
        if !paused {
            clock += get_frame_time() * speed;
        }
        next_frame().await;
    }
}

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    capture_variant(output, false).await
}
async fn capture_variant(output: &Path, expand_exit: bool) -> Result<(), String> {
    if output.exists() {
        return Err("Le dossier de revue existe déjà.".into());
    }
    std::fs::create_dir_all(output).map_err(|error| error.to_string())?;
    request_new_screen_size(1024.0, 1024.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let current = Stopped::load(expand_exit)?;
    let target = render_target(1024, 1024);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1024.0, 1024.0))
    };
    let mut entries = Vec::new();
    for case in 0..if expand_exit { 1 } else { 5 } {
        let stage = &current.stages[case];
        let folder = output.join(CASE_IDS[case]);
        std::fs::create_dir(&folder).map_err(|e| e.to_string())?;
        let directions = if case < 3 { 4 } else { 2 };
        let panels = if case < 3 {
            PANELS
        } else {
            [[24.0, 48.0], [24.0, 536.0], [0.0, 0.0], [0.0, 0.0]]
        };
        let mask = if expand_exit {
            expanded::visibility(stage)
        } else {
            visibility(stage)
        };
        mask.export_png(folder.join("visibility.png").to_str().unwrap());
        for frame in 0..FRAMES {
            let clock = frame as f32 / FPS as f32;
            let cast_age = (clock >= TARGETING_END).then_some(clock - TARGETING_END);
            for control in 0..if frame == 24 || frame == 52 { 2 } else { 1 } {
                set_camera(&camera);
                clear_background(INK);
                for direction in 0..directions {
                    let [x, y] = panels[direction];
                    let size = stage.size(direction);
                    draw_text_bold(LABELS[direction], x, y - 12.0, 18.0, TEXT);
                    scene(
                        &current,
                        case,
                        direction,
                        Rect::new(x, y, size.x, size.y),
                        clock,
                        cast_age,
                        control == 0,
                    );
                }
                let file = if control == 0 {
                    format!("{frame:03}.png")
                } else {
                    format!("control-{frame:03}.png")
                };
                export_target(&target, &folder.join(file));
            }
            set_default_camera();
            next_frame().await;
        }
        for direction in 0..directions {
            let size = stage.size(direction);
            let mut rows = vec![vec!['.'; (size.x / TILE) as usize]; (size.y / TILE) as usize];
            for y in 0..stage.height {
                for x in 0..stage.width {
                    let at = stage.position(GridPos::new(x, y), direction);
                    if stage.fixture.map.blocks_vision(GridPos::new(x, y)) {
                        rows[at.y as usize][at.x as usize] = '#';
                    }
                }
            }
            let coords = |at: GridPos| {
                let at = stage.position(at, direction);
                [at.x, at.y]
            };
            entries.push(serde_json::json!({"id":"FE02","number":10,"variant":variant(case,direction,expand_exit),"folder":CASE_IDS[case],"title":CASE_LABELS[case],"direction":DIRECTIONS[direction],"tile":64,"frames":FRAMES,"fps":FPS,
                "crop":[panels[direction][0] as u32,panels[direction][1] as u32,size.x as u32,size.y as u32],"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n"),"canonical_map":stage.map_text,"origin":coords(ORIGIN),
                "affected_cells":stage.fixture.affected.iter().copied().map(coords).collect::<Vec<_>>(),"targets":stage.fixture.targets.iter().enumerate().map(|(i,&at)|serde_json::json!({"label":char::from(b'A'+i as u8).to_string(),"cell":coords(at),"affected":stage.fixture.affected.contains(&at)})).collect::<Vec<_>>(),
                "control_frames":[24,52],"preview_end_seconds":TARGETING_END,"breath_end_seconds":BREATH_END,"burn_end_seconds":BURN_END,"status_lifetime_scope":"presentation only; no game turns advanced"}));
        }
    }
    if !expand_exit {
        let folder = output.join("BRULURE");
        std::fs::create_dir(&folder).map_err(|e| e.to_string())?;
        for frame in 0..FRAMES {
            for control in 0..if frame == 52 { 2 } else { 1 } {
                set_camera(&camera);
                clear_background(INK);
                burn_scene(
                    &current.fire,
                    Rect::new(24.0, 48.0, 448.0, 448.0),
                    frame as f32 / FPS as f32,
                    control == 0,
                );
                let name = if control == 0 {
                    format!("{frame:03}.png")
                } else {
                    "control-052.png".into()
                };
                export_target(&target, &folder.join(name));
            }
            set_default_camera();
            next_frame().await;
        }
        entries.push(serde_json::json!({"id":"BRULURE","number":"B01","variant":"BRULURE-A02-AGENT-MONSTRE","folder":"BRULURE","title":"Brûlure · agent et monstre","tile":64,"frames":FRAMES,"fps":FPS,"crop":[24,48,448,448],"control_frames":[52],"actors":[[2,3],[4,3]],"status_lifetime_scope":"presentation only; no friendly fire or gameplay damage"}));
    }
    if expand_exit {
        let mut regressions = Vec::new();
        for case in 1..5 {
            let stage = &current.stages[case];
            let same = expanded::visibility(stage).bytes == visibility(stage).bytes;
            if !same {
                return Err(format!("Masque inchangé attendu pour {}", CASE_IDS[case]));
            }
            regressions
                .push(serde_json::json!({"case":CASE_IDS[case],"mask_pixels_unchanged":same}));
        }
        std::fs::write(
            output.join("mask-regression.json"),
            serde_json::to_vec_pretty(&regressions).unwrap(),
        )
        .map_err(|e| e.to_string())?;
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
pub(in crate::agent_skill_lab_review::fire_painted) fn export_target(
    target: &RenderTarget,
    path: &Path,
) {
    // Fixed native render target: minimising the hidden capture window cannot
    // turn a review frame into a 1x1 framebuffer.
    unsafe {
        macroquad::window::get_internal_gl().flush();
    }
    let mut image = target.texture.get_texture_data();
    for pixel in image.bytes.as_chunks_mut::<4>().0 {
        pixel[3] = 255;
    }
    image.export_png(path.to_str().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn physical_cloison_stops_all_rays_to_the_far_room() {
        let stage = Stage::new(1);
        let walls = stage.walls();
        for y in 64..384 {
            assert!(!clear_ray(vec2(352.0, y as f32 + 0.5), &walls));
        }
        assert!(clear_ray(vec2(220.0, 219.0), &walls));
    }
    #[test]
    fn partial_obstacle_stops_center_but_leaves_both_sides_open() {
        let walls = Stage::new(2).walls();
        assert!(!clear_ray(vec2(352.0, 219.0), &walls));
        assert!(clear_ray(vec2(224.0, 160.0), &walls));
        assert!(clear_ray(vec2(224.0, 288.0), &walls));
    }
    #[test]
    fn long_corridor_preserves_range_and_stops_at_a_cross_wall() {
        let open = Stage::new(3);
        let closed = Stage::new(4);
        assert!(open.fixture.affected.contains(&GridPos::new(6, 3)));
        assert!(!open.fixture.affected.contains(&GridPos::new(9, 3)));
        assert!(!closed.fixture.affected.contains(&GridPos::new(5, 3)));
        assert!(clear_ray(vec2(350.0, 219.0), &open.walls()));
        assert!(!clear_ray(vec2(350.0, 219.0), &closed.walls()));
    }
}
