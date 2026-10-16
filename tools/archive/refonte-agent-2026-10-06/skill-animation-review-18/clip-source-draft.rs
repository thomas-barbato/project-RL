//! ANIM-S05: illustrated wall and delayed eruption, isolated visual fixtures.
use super::obstacles::overlay::stopped::{burn_on_body, export_target};
use super::*;
use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Equation, PipelineParams, UniformDesc, UniformType,
};
use project_rl::world::Map;

const SIZE: f32 = 576.0;
const CENTER: GridPos = GridPos::new(4, 4);
const PLAYER: GridPos = GridPos::new(4, 7);
const IDS: [&str; 2] = ["FE03", "FE04"];
const TITLES: [&str; 2] = ["Mur de feu", "Émanation soudaine"];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.....#.#\n#.....#.#\n#.....#.#\n#.....#.#\n#.....#.#\n#.....#.#\n#.....#.#\n#########",
];
const EVENTS: [f32; 7] = [1.35, 1.90, 2.40, 2.85, 3.30, 3.80, 4.30];

pub(super) struct Clip {
    material: Material,
    masks: Vec<Texture2D>,
}
impl Clip {
    fn load() -> Self {
        Self::load_maps(&MAPS, CENTER)
    }
    pub(super) fn load_maps(maps: &[&str], origin: GridPos) -> Self {
        let masks = maps
            .iter()
            .map(|source| {
                let map = Map::from_ascii(source).unwrap();
                let mut rooms = Image::gen_image_color(9, 9, BLACK);
                let mut queue = std::collections::VecDeque::from([origin]);
                let mut seen = std::collections::BTreeSet::from([(origin.x, origin.y)]);
                while let Some(at) = queue.pop_front() {
                    rooms.set_pixel(at.x as u32, at.y as u32, WHITE);
                    for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                        let next = GridPos::new(at.x + dx, at.y + dy);
                        if next.x >= 0
                            && next.y >= 0
                            && next.x < 9
                            && next.y < 9
                            && map.is_walkable(next)
                            && seen.insert((next.x, next.y))
                        {
                            queue.push_back(next);
                        }
                    }
                }
                let texture = Texture2D::from_image(&rooms);
                texture.set_filter(FilterMode::Nearest);
                texture
            })
            .collect();
        let material=load_material(ShaderSource::Glsl {vertex:r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;
varying highp vec2 point;varying highp vec2 uv;varying lowp vec4 tint;
void main(){point=position.xy;uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#,
fragment:r#"#version 100
precision highp float;varying highp vec2 point;varying highp vec2 uv;varying lowp vec4 tint;
uniform sampler2D Texture;uniform sampler2D roomMask;uniform vec2 stageOrigin;uniform float aimDirection;
void main(){vec2 p=point-stageOrigin;vec2 q=p;
if(aimDirection<0.5)q=vec2(p.y,576.0-p.x);
else if(aimDirection<1.5)q=vec2(576.0-p.y,p.x);
else if(aimDirection>2.5)q=vec2(576.0)-p;
if(q.x<0.0||q.y<0.0||q.x>=576.0||q.y>=576.0)discard;
float allowed=texture2D(roomMask,(floor(q/64.0)+vec2(0.5))/9.0).r;
vec4 colour=texture2D(Texture,uv)*tint;gl_FragColor=vec4(colour.rgb,colour.a*allowed);}"#},MaterialParams {
            uniforms:vec![UniformDesc::new("stageOrigin",UniformType::Float2),UniformDesc::new("aimDirection",UniformType::Float1)],textures:vec!["roomMask".into()],
            pipeline_params:PipelineParams {color_blend:Some(BlendState::new(Equation::Add,BlendFactor::Value(BlendValue::SourceAlpha),BlendFactor::OneMinusValue(BlendValue::SourceAlpha))),..Default::default()},..Default::default()
        }).expect("zone clipping shader");
        Self { material, masks }
    }
    fn apply(&self, state: &State, rect: Rect) {
        self.apply_for(state.obstacle, state.direction, rect);
    }
    pub(super) fn apply_for(&self, obstacle: usize, direction: usize, rect: Rect) {
        self.material.set_uniform("stageOrigin", [rect.x, rect.y]);
        self.material.set_uniform("aimDirection", direction as f32);
        self.material
            .set_texture("roomMask", self.masks[obstacle].clone());
        gl_use_material(&self.material);
    }
}

pub(super) fn position(at: GridPos, direction: usize) -> GridPos {
    match direction {
        0 => GridPos::new(8 - at.y, at.x),
        1 => GridPos::new(at.y, 8 - at.x),
        3 => GridPos::new(8 - at.x, 8 - at.y),
        _ => at,
    }
}
fn point(at: Vec2, direction: usize) -> Vec2 {
    match direction {
        0 => vec2(SIZE - at.y, at.x),
        1 => vec2(at.y, SIZE - at.x),
        3 => vec2(SIZE - at.x, SIZE - at.y),
        _ => at,
    }
}
fn wall_cells(map: &Map, rank: u16) -> Vec<GridPos> {
    let length = parameter_for("FE03", "maximum_wall_length_cells", rank).min(7) as i32;
    let mut cells = vec![CENTER];
    for sign in [-1, 1] {
        for step in 1..=length / 2 {
            let at = GridPos::new(CENTER.x + sign * step, CENTER.y);
            if !map.is_walkable(at) {
                break;
            }
            cells.push(at);
        }
    }
    cells.sort_by_key(|p| p.x);
    cells
}
fn affected(map: &Map, skill: usize, rank: u16) -> Vec<GridPos> {
    if skill == 0 {
        wall_cells(map, rank)
    } else {
        CROSS
            .iter()
            .map(|&(x, y)| GridPos::new(CENTER.x + x, CENTER.y + y))
            .filter(|&at| map.is_walkable(at))
            .collect()
    }
}
fn targets(skill: usize) -> Vec<GridPos> {
    if skill == 0 {
        vec![CENTER, GridPos::new(2, 4), GridPos::new(7, 4)]
    } else {
        CROSS
            .iter()
            .map(|&(x, y)| GridPos::new(CENTER.x + x, CENTER.y + y))
            .chain([GridPos::new(7, 4)])
            .collect()
    }
}
fn variant(skill: usize, obstacle: usize) -> String {
    format!(
        "{}-A03-{}",
        IDS[skill],
        if obstacle == 0 { "LIBRE" } else { "CLOISON" }
    )
}
pub(super) fn terrain(map: &Map, direction: usize, rect: Rect, walls_only: bool) {
    for y in 0..9 {
        for x in 0..9 {
            let source = GridPos::new(x, y);
            let wall = map.blocks_vision(source);
            if walls_only && !wall {
                continue;
            }
            let at = position(source, direction);
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
pub(super) fn preview(cells: &[GridPos], direction: usize, rect: Rect, clock: f32) {
    let strength = if (clock.max(0.0) * 2.0) as u32 % 2 == 0 {
        1.0
    } else {
        0.42
    };
    for &source in cells {
        let at = position(source, direction);
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
struct State {
    skill: usize,
    obstacle: usize,
    direction: usize,
    rank: u16,
    clock: f32,
    launched: Option<f32>,
    burst: Option<f32>,
    burst_turn: Option<u16>,
    hold_until_turn: bool,
    walk_start: Option<f32>,
    turn: u16,
    cooldown: u16,
    wall_remaining: u16,
    burn: [u16; 6],
}
impl State {
    fn new(skill: usize, obstacle: usize, direction: usize) -> Self {
        Self {
            skill,
            obstacle,
            direction,
            rank: 11,
            clock: 0.0,
            launched: None,
            burst: None,
            burst_turn: None,
            hold_until_turn: false,
            walk_start: None,
            turn: 0,
            cooldown: 0,
            wall_remaining: 0,
            burn: [0; 6],
        }
    }
    fn launch(&mut self) {
        if self.cooldown > 0 {
            return;
        }
        self.launched = Some(self.clock);
        self.burst = None;
        self.burst_turn = None;
        self.walk_start = None;
        self.turn = 0;
        self.burn = [0; 6];
        self.cooldown = parameter_for(IDS[self.skill], "cooldown_turns", self.rank);
        if self.skill == 0 {
            self.wall_remaining = parameter_for("FE03", "duration_turns", self.rank);
            let map = Map::from_ascii(MAPS[self.obstacle]).unwrap();
            let zone = affected(&map, 0, self.rank);
            for (i, at) in targets(0).iter().enumerate() {
                if zone.contains(at) {
                    self.burn[i] = parameter_for("FE03", "burn_duration_turns", self.rank);
                }
            }
        }
    }
    fn advance_turn(&mut self) {
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        for duration in &mut self.burn {
            *duration = duration.saturating_sub(1);
        }
        if self.launched.is_none() {
            return;
        }
        if self.skill == 0 {
            self.wall_remaining = self.wall_remaining.saturating_sub(1);
            if self.walk_start.is_none() {
                self.walk_start = Some(self.clock);
            }
            if self.wall_remaining > 0 {
                let map = Map::from_ascii(MAPS[self.obstacle]).unwrap();
                let zone = affected(&map, 0, self.rank);
                for (i, at) in targets(0).iter().enumerate().skip(1) {
                    if zone.contains(at) {
                        self.burn[i] = parameter_for("FE03", "burn_duration_turns", self.rank);
                    }
                }
            }
        } else if self.burst.is_none() {
            self.burst = Some(self.clock);
            self.burst_turn = Some(self.turn);
            let map = Map::from_ascii(MAPS[self.obstacle]).unwrap();
            let zone = affected(&map, 1, self.rank);
            for (i, at) in targets(1).iter().enumerate() {
                if zone.contains(at) {
                    self.burn[i] = parameter_for("FE04", "burn_duration_turns", self.rank);
                }
            }
        }
    }
    fn eruption_opacity(&self) -> f32 {
        let Some(start) = self.burst else {
            return 0.0;
        };
        if self.hold_until_turn {
            if self.burst_turn == Some(self.turn) {
                1.0
            } else {
                0.0
            }
        } else {
            (1.0 - (self.clock - start) / 0.80).clamp(0.0, 1.0).sqrt()
        }
    }
    fn replay(&mut self) {
        if self.launched.is_some() {
            self.launched = Some(self.clock);
        }
        if self.burst.is_some() {
            self.burst = Some(self.clock);
        }
        if self.walk_start.is_some() {
            self.walk_start = Some(self.clock);
        }
    }
    fn actor_point(&self, source: GridPos, index: usize, rect: Rect) -> Vec2 {
        let mut p = vec2(source.x as f32 + 0.5, source.y as f32 + 0.5) * TILE;
        if self.skill == 0 && index == 0 {
            if let Some(start) = self.walk_start {
                p.y += ((self.clock - start) / 0.45).clamp(0.0, 1.0) * TILE;
            }
        }
        point(p, self.direction) + vec2(rect.x, rect.y)
    }
}
fn painted_zone(fire: &DrawnFire, state: &State, zone: &[GridPos], rect: Rect, flame_speed: f32) {
    let Some(start) = state.launched else {
        return;
    };
    let age = state.clock - start;
    if state.skill == 0 && state.wall_remaining > 0 {
        // Half-cell spacing overlaps the painted drawings into one curtain.
        let first = zone.first().unwrap().x;
        let last = zone.last().unwrap().x;
        for n in 0..=(last - first) * 2 {
            let world = vec2(first as f32 + 0.5 + n as f32 * 0.5, CENTER.y as f32 + 0.5) * TILE;
            let at = point(world, state.direction) + vec2(rect.x, rect.y);
            fire.plume(
                at + vec2(0.0, 24.0),
                age * flame_speed,
                n as u32 * 3,
                1.0,
                1.0,
            );
        }
    }
    if state.skill == 1 {
        if let Some(burst) = state.burst {
            let age = state.clock - burst;
            let opacity = state.eruption_opacity();
            if opacity > 0.0 {
                let rise = (0.55 + age / 0.11).min(1.0);
                for (index, &source) in zone.iter().enumerate() {
                    let at = position(source, state.direction);
                    let p = cell(rect, at.x as f32 + 0.5, at.y as f32 + 0.5);
                    fire.plume(
                        p + vec2(0.0, 24.0),
                        age * flame_speed,
                        index as u32 * 2,
                        if source == CENTER {
                            1.45 * rise
                        } else {
                            1.10 * rise
                        },
                        opacity,
                    );
                }
            }
        } else {
            for (i, &source) in zone.iter().enumerate() {
                let at = position(source, state.direction);
                cracks(
                    cell(rect, at.x as f32 + 0.5, at.y as f32 + 0.5) + vec2(0.0, 20.0),
                    age,
                    i as u32 * 27,
                    1.0,
                );
            }
        }
    }
}
fn scene(
    fire: &DrawnFire,
    clip: &Clip,
    state: &State,
    rect: Rect,
    effects: bool,
    flame_speed: f32,
) {
    let map = Map::from_ascii(MAPS[state.obstacle]).unwrap();
    let zone = affected(&map, state.skill, state.rank);
    terrain(&map, state.direction, rect, false);
    if state.launched.is_none() {
        preview(&zone, state.direction, rect, state.clock);
    } else if effects {
        clip.apply(state, rect);
        // Only texture sampling slows down; rise, fade, movement and turns keep their clock.
        painted_zone(fire, state, &zone, rect, flame_speed);
        gl_use_default_material();
    }
    let actors = targets(state.skill);
    if effects {
        for (i, &at) in actors.iter().enumerate() {
            if state.burn[i] > 0 {
                fire.burn(
                    state.actor_point(at, i, rect),
                    state.clock * flame_speed,
                    i as u32 * 2,
                    0.9,
                );
            }
        }
    }
    terrain(&map, state.direction, rect, true);
    for (i, &at) in actors.iter().enumerate() {
        let p = state.actor_point(at, i, rect);
        enemy(p);
        if effects && state.burn[i] > 0 {
            burn_on_body(fire, p, state.clock * flame_speed, i as u32 * 2);
        }
        draw_text_bold(
            char::from(b'A' + i as u8).to_string(),
            p.x - 4.0,
            p.y - 29.0,
            13.0,
            TEXT,
        );
    }
    let at = position(PLAYER, state.direction);
    let player = cell(rect, at.x as f32 + 0.5, at.y as f32 + 0.5);
    actor(player, caster());
    if let Some(start) = state.launched {
        name(state.skill + 1, player, state.clock - start);
    }
    let label = if state.launched.is_none() {
        "Aperçu clignotant · avant lancement"
    } else if state.skill == 0 && state.wall_remaining > 0 {
        "Rideau traversable · A sort des flammes"
    } else if state.skill == 0 {
        "Mur terminé · Brûlure sur les corps"
    } else if state.burst.is_none() {
        "Préparation · 1 tour complet"
    } else if state.eruption_opacity() > 0.0 {
        if state.hold_until_turn {
            "Éruption · jusqu'au prochain tour"
        } else {
            "+1 tour · éruption"
        }
    } else {
        "Éruption terminée · Brûlure sur les corps"
    };
    draw_rectangle(
        rect.x + 7.0,
        rect.y + rect.h - 27.0,
        rect.w - 14.0,
        22.0,
        alpha(INK, 0.94),
    );
    draw_text(label, rect.x + 14.0, rect.y + rect.h - 11.0, 14.0, TEXT);
}
pub(crate) async fn run(flame_speed: f32) {
    run_mode(flame_speed, false).await;
}
pub(crate) async fn run_held() {
    run_mode(2.0 / 3.0, true).await;
}
async fn run_mode(flame_speed: f32, held: bool) {
    request_new_screen_size(1280.0, 900.0);
    let fire = DrawnFire::load();
    let clip = Clip::load();
    let mut state = State::new(if held { 1 } else { 0 }, 0, 2);
    let mut paused = false;
    let mut speed = 1.0;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold(
            if held {
                "LABORATOIRE / FEU · TOURS"
            } else if flame_speed == 1.0 {
                "LABORATOIRE / FEU · ZONES"
            } else {
                "LABORATOIRE / FEU · CADENCE"
            },
            22.0,
            35.0,
            24.0,
            TEXT,
        );
        for (i, title) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 88.0 + i as f32 * 44.0, 285.0, 35.0),
                title,
                true,
            ) {
                state = State::new(i, state.obstacle, state.direction);
            }
        }
        for (i, label) in ["Zone ouverte", "Avec cloison"].iter().enumerate() {
            if button(
                Rect::new(20.0, 198.0 + i as f32 * 44.0, 285.0, 35.0),
                label,
                true,
            ) {
                state = State::new(state.skill, i, state.direction);
            }
        }
        for (i, label) in ["Droite", "Gauche", "Haut", "Bas"].iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 146.0,
                    305.0 + (i / 2) as f32 * 42.0,
                    138.0,
                    34.0,
                ),
                label,
                true,
            ) {
                state = State::new(state.skill, state.obstacle, i);
            }
        }
        draw_text(
            format!("Rang effectif : {}", state.rank),
            22.0,
            419.0,
            17.0,
            TEXT,
        );
        for (i, label) in ["− rang", "+ rang"].iter().enumerate() {
            if button(
                Rect::new(20.0 + i as f32 * 146.0, 439.0, 138.0, 34.0),
                label,
                true,
            ) {
                let rank = if i == 0 {
                    state.rank.saturating_sub(1).max(1)
                } else {
                    state.rank.saturating_add(1).min(20)
                };
                state = State::new(state.skill, state.obstacle, state.direction);
                state.rank = rank;
            }
        }
        state.hold_until_turn = held;
        if button(
            Rect::new(20.0, 501.0, 138.0, 34.0),
            if state.cooldown > 0 {
                "En recharge"
            } else {
                "Lancer"
            },
            state.cooldown == 0,
        ) || (is_key_pressed(KeyCode::Enter) && state.cooldown == 0)
        {
            state.launch();
            paused = false;
        }
        if button(Rect::new(166.0, 501.0, 138.0, 34.0), "+1 tour", true)
            || is_key_pressed(KeyCode::Space)
        {
            state.advance_turn();
        }
        if button(Rect::new(20.0, 545.0, 138.0, 34.0), "Rejouer", true) {
            state.replay();
        }
        if button(Rect::new(166.0, 545.0, 138.0, 34.0), "Viser", true) {
            state = State::new(state.skill, state.obstacle, state.direction);
        }
        if button(
            Rect::new(20.0, 589.0, 138.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if button(
            Rect::new(166.0, 589.0, 138.0, 34.0),
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
            format!("Tour {} · recharge : {} tours", state.turn, state.cooldown),
            22.0,
            670.0,
            16.0,
            TEXT,
        );
        state.hold_until_turn = held;
        scene(
            &fire,
            &clip,
            &state,
            Rect::new(410.0, 88.0, SIZE, SIZE),
            true,
            flame_speed,
        );
        draw_text(
            if held {
                variant(state.skill, state.obstacle).replace("A03", "A05")
            } else if flame_speed == 1.0 {
                variant(state.skill, state.obstacle)
            } else {
                variant(state.skill, state.obstacle).replace("A03", "A04")
            },
            410.0,
            695.0,
            15.0,
            DIM,
        );
        draw_text(
            "Émanation attend +1 tour · les animations ne font pas avancer les tours",
            22.0,
            857.0,
            15.0,
            DIM,
        );
        if !paused {
            state.clock += get_frame_time() * speed;
        }
        next_frame().await;
    }
}
pub(crate) async fn capture(output: &Path, flame_speed: f32) -> Result<(), String> {
    if output.exists() {
        return Err("Le dossier de revue existe déjà.".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(1280.0, 720.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let fire = DrawnFire::load();
    let clip = Clip::load();
    let target = render_target(1280, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1280.0, 720.0))
    };
    let mut entries = Vec::new();
    for obstacle in 0..2 {
        let folder = output.join(if obstacle == 0 { "LIBRE" } else { "CLOISON" });
        std::fs::create_dir(&folder).map_err(|e| e.to_string())?;
        let mut states = [
            State::new(0, obstacle, if obstacle == 0 { 2 } else { 0 }),
            State::new(1, obstacle, if obstacle == 0 { 2 } else { 1 }),
        ];
        let mut event = 0;
        let mut launched = false;
        for frame in 0..120 {
            let clock = frame as f32 / 24.0;
            for state in &mut states {
                state.clock = clock;
            }
            if !launched && clock >= 0.7 {
                for s in &mut states {
                    s.launch();
                }
                launched = true;
            }
            while event < EVENTS.len() && clock >= EVENTS[event] {
                for s in &mut states {
                    s.advance_turn();
                }
                event += 1;
            }
            for control in 0..if [24, 40, 84, 119].contains(&frame) {
                2
            } else {
                1
            } {
                set_camera(&camera);
                clear_background(INK);
                for (skill, state) in states.iter().enumerate() {
                    let x = 24.0 + skill as f32 * 616.0;
                    draw_text_bold(
                        format!("{} · {}", skill + 11, TITLES[skill]),
                        x,
                        35.0,
                        21.0,
                        TEXT,
                    );
                    draw_text(
                        format!(
                            "{} · rang 11 · {}",
                            variant(skill, obstacle),
                            DIRS[state.direction]
                        ),
                        x,
                        60.0,
                        14.0,
                        DIM,
                    );
                    scene(
                        &fire,
                        &clip,
                        state,
                        Rect::new(x, 80.0, SIZE, SIZE),
                        control == 0,
                        flame_speed,
                    );
                }
                let name = if control == 0 {
                    format!("{frame:03}.png")
                } else {
                    format!("control-{frame:03}.png")
                };
                export_target(&target, &folder.join(name));
            }
            set_default_camera();
            next_frame().await;
        }
        for (skill, state) in states.iter().enumerate() {
            let map = Map::from_ascii(MAPS[obstacle]).unwrap();
            let zone = affected(&map, skill, 11);
            let mut rows = vec![vec!['.'; 9]; 9];
            for y in 0..9 {
                for x in 0..9 {
                    let at = position(GridPos::new(x, y), state.direction);
                    if map.blocks_vision(GridPos::new(x, y)) {
                        rows[at.y as usize][at.x as usize] = '#';
                    }
                }
            }
            let coords = |source: GridPos| {
                let p = position(source, state.direction);
                [p.x, p.y]
            };
            entries.push(serde_json::json!({"id":IDS[skill],"number":skill+11,"variant":if flame_speed==1.0 {variant(skill,obstacle)} else {variant(skill,obstacle).replace("A03","A04")},"flame_texture_fps":12.0*flame_speed,"folder":if obstacle==0 {"LIBRE"} else {"CLOISON"},"title":TITLES[skill],"direction":DIRS[state.direction],"tile":64,"frames":120,"fps":24,"framebuffer_size":[1280,720],"crop":[24+skill*616,80,576,576],
                "map":rows.into_iter().map(|row|row.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n"),"affected_cells":zone.iter().copied().map(coords).collect::<Vec<_>>(),"targets":targets(skill).iter().enumerate().map(|(i,&at)|serde_json::json!({"label":char::from(b'A'+i as u8).to_string(),"cell":coords(at),"affected":zone.contains(&at)})).collect::<Vec<_>>(),"moving_target":if skill==0 {Some("A")} else {None},"moving_target_destination":if skill==0 {Some(coords(GridPos::new(4,5)))} else {None},
                "preview_end_frame":17,"launch_frame":17,"scripted_turn_events_seconds":EVENTS,"eruption_preparation_complete_turns":if skill==1 {Some(1)} else {None},"control_frames":[24,40,84,119],"burn_rolls":"Scripted successes for visual review only; no RNG or damage calculation."}));
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
pub(crate) async fn capture_held(output: &Path) -> Result<(), String> {
    if output.exists() {
        return Err("Le dossier de revue existe déjà.".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(1280.0, 720.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let fire = DrawnFire::load();
    let clip = Clip::load();
    let target = render_target(1280, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1280.0, 720.0))
    };
    let mut states = [State::new(1, 0, 2), State::new(1, 0, 2)];
    states[1].hold_until_turn = true;
    let events = [1.35, 4.25, 4.75, 5.15];
    let controls = [24, 40, 60, 96, 103, 143];
    let mut event = 0;
    let mut launched = false;
    let mut timeline = Vec::new();
    for frame in 0..144 {
        let clock = frame as f32 / 24.0;
        for state in &mut states {
            state.clock = clock;
        }
        if !launched && clock >= 0.7 {
            for state in &mut states {
                state.launch();
            }
            launched = true;
        }
        while event < events.len() && clock >= events[event] {
            for state in &mut states {
                state.advance_turn();
            }
            event += 1;
        }
        timeline.push(serde_json::json!({"frame":frame,"turn":states[1].turn,"cooldown":states[1].cooldown,"burn":states[1].burn,"held_eruption_visible":states[1].eruption_opacity()>0.0,"old_eruption_visible":states[0].eruption_opacity()>0.0}));
        for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
            set_camera(&camera);
            clear_background(INK);
            for (column, state) in states.iter().enumerate() {
                let x = 24.0 + column as f32 * 616.0;
                draw_text_bold(
                    if column == 0 {
                        "Avant · disparition en secondes"
                    } else {
                        "12 · FE04-A05 · maintien par tours"
                    },
                    x,
                    35.0,
                    21.0,
                    TEXT,
                );
                draw_text(
                    format!("Tour {} · recharge : {} tours", state.turn, state.cooldown),
                    x,
                    60.0,
                    15.0,
                    DIM,
                );
                scene(
                    &fire,
                    &clip,
                    state,
                    Rect::new(x, 80.0, SIZE, SIZE),
                    control == 0,
                    2.0 / 3.0,
                );
            }
            let name = if control == 0 {
                format!("{frame:03}.png")
            } else {
                format!("control-{frame:03}.png")
            };
            export_target(&target, &output.join(name));
        }
        set_default_camera();
        next_frame().await;
    }
    let entries = [false,true].iter().enumerate().map(|(column,&held)|serde_json::json!({"id":"FE04","number":12,"variant":if held {"FE04-A05-TOUR"} else {"FE04-A04-TEMOIN"},"held":held,"frames":144,"fps":24,"tile":64,"crop":[24+column*616,80,576,576],"control_frames":controls,"scripted_turn_events_seconds":events,"launch_frame":17,"burst_frame":33,"end_frame":102,"map":MAPS[0],"affected_cells":CROSS.iter().map(|&(x,y)|[CENTER.x+x,CENTER.y+y]).collect::<Vec<_>>(),"note":"Visual duration only; no extra damage applications, turns or cooldown changes."})).collect::<Vec<_>>();
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    std::fs::write(
        output.join("timeline.json"),
        serde_json::to_vec_pretty(&timeline).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn held_eruption_waits_for_a_turn_and_replay_cannot_restore_an_expired_effect() {
        let mut state = State::new(1, 0, 2);
        state.hold_until_turn = true;
        state.launch();
        state.advance_turn();
        let burn = state.burn;
        let cooldown = state.cooldown;
        state.clock = 600.0;
        assert_eq!(state.eruption_opacity(), 1.0);
        assert_eq!(state.turn, 1);
        assert_eq!(state.burn, burn);
        assert_eq!(state.cooldown, cooldown);
        state.replay();
        assert_eq!(state.eruption_opacity(), 1.0);
        state.advance_turn();
        assert_eq!(state.eruption_opacity(), 0.0);
        assert_eq!(state.cooldown, cooldown - 1);
        state.replay();
        assert_eq!(state.eruption_opacity(), 0.0);
        assert_eq!(state.turn, 2);
    }
    #[test]
    fn wall_footprint_grows_with_rank_and_stops_at_the_first_solid_cell() {
        let open = Map::from_ascii(MAPS[0]).unwrap();
        let blocked = Map::from_ascii(MAPS[1]).unwrap();
        assert_eq!(wall_cells(&open, 1).len(), 3);
        assert_eq!(wall_cells(&open, 11).len(), 7);
        assert!(!wall_cells(&blocked, 11).contains(&GridPos::new(7, 4)));
        assert_eq!(wall_cells(&blocked, 11).len(), 5);
    }
    #[test]
    fn eruption_requires_a_game_turn_and_replay_preserves_its_cooldown() {
        let mut state = State::new(1, 0, 2);
        state.launch();
        state.clock = 100.0;
        assert!(state.burst.is_none());
        assert_eq!(state.turn, 0);
        state.advance_turn();
        assert_eq!(state.burst, Some(100.0));
        let cd = state.cooldown;
        state.clock = 200.0;
        state.replay();
        assert_eq!(state.turn, 1);
        assert_eq!(state.cooldown, cd);
    }
    #[test]
    fn crossing_the_wall_keeps_burn_attached_after_exit() {
        let mut state = State::new(0, 0, 2);
        state.launch();
        state.clock = 1.0;
        state.advance_turn();
        assert!(state.burn[0] > 0);
        state.clock = 2.0;
        assert_eq!(
            state.actor_point(CENTER, 0, Rect::new(0.0, 0.0, SIZE, SIZE)),
            vec2(288.0, 352.0)
        );
        assert!(!affected(&Map::from_ascii(MAPS[0]).unwrap(), 0, 11).contains(&GridPos::new(4, 5)));
    }
}
