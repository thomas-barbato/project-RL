//! ANIM-S34: approved utility rules, new native visual candidates only.
//! Responses are prepared fixtures, not campaign actions or probability rolls.
use super::super::melee::actor;
use super::zones::{position, preview, terrain};
use super::*;
use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Equation, PipelineParams, UniformDesc, UniformType,
};
use project_rl::skills::spatial::{Area, affected_cells, in_cast_range};
use project_rl::stealth::StealthRules;
use project_rl::world::{
    FieldOfViewRules, Map, compute_visible_tiles, find_path, has_line_of_sight,
};

const PLAYER: GridPos = GridPos::new(4, 6);
const TARGET: GridPos = GridPos::new(4, 3);
const FLIGHT: f32 = 0.18;
const BLAST: f32 = 0.42;
const IDS: [&str; 4] = ["SG01", "SG02", "SG04", "SG00"];
const NUMBERS: [u16; 4] = [75, 76, 78, 74];
const TITLES: [&str; 4] = [
    "Grenade aveuglante",
    "Leurre",
    "Puissance des grenades",
    "Grenade de référence",
];
const CASES: [&str; 3] = [
    "Terrain ouvert",
    "Au bord d’une cloison",
    "Trajectoire bloquée",
];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const MAPS: [&str; 3] = [
    ".........\n.........\n.........\n.........\n.........\n.........\n.........\n.........\n.........",
    ".....#...\n.....#...\n.....#...\n.....#...\n.....#...\n.....#...\n.....#...\n.........\n.........",
    ".........\n.........\n.........\n.........\n#########\n.........\n.........\n.........\n.........",
];

// A display-only revision. The preserved A01 drawings remain available for comparison.
fn refined_visuals() -> bool {
    !std::env::args().any(|arg| arg == "--historique-grenades-detaillees")
}

fn blindness_marker(p: Vec2, remaining: u16) {
    if refined_visuals() {
        let center = p + vec2(-10.0, -43.0);
        let lid = [
            vec2(-14.0, 0.0),
            vec2(-9.0, -5.0),
            vec2(-4.0, -7.0),
            vec2(4.0, -7.0),
            vec2(9.0, -5.0),
            vec2(14.0, 0.0),
            vec2(9.0, 5.0),
            vec2(4.0, 7.0),
            vec2(-4.0, 7.0),
            vec2(-9.0, 5.0),
        ];
        for i in 0..lid.len() {
            let a = center + lid[i];
            let b = center + lid[(i + 1) % lid.len()];
            draw_triangle(center, a, b, Color::from_rgba(242, 240, 222, 255));
            draw_line(a.x, a.y, b.x, b.y, 2.6, Color::from_rgba(13, 16, 20, 255));
        }
        draw_circle(center.x, center.y, 5.2, Color::from_rgba(13, 16, 20, 255));
        draw_circle(center.x - 1.0, center.y + 2.0, 1.1, WHITE);
        let from = center + vec2(-12.0, 8.0);
        let to = center + vec2(12.0, -12.0);
        draw_line(
            from.x,
            from.y,
            to.x,
            to.y,
            5.2,
            Color::from_rgba(13, 16, 20, 255),
        );
        draw_line(
            from.x,
            from.y,
            to.x,
            to.y,
            3.2,
            Color::from_rgba(237, 76, 59, 255),
        );
        draw_text_bold(format!("{remaining}"), p.x + 12.0, p.y - 35.0, 18.0, TEXT);
    } else {
        draw_ellipse(p.x - 8.0, p.y - 40.0, 10.0, 6.0, 0.0, BLACK);
        draw_ellipse_lines(p.x - 8.0, p.y - 40.0, 10.0, 6.0, 0.0, 2.0, TEXT);
        draw_line(p.x - 17.0, p.y - 34.0, p.x + 1.0, p.y - 46.0, 3.0, WHITE);
        draw_text_bold(format!("{remaining}"), p.x + 7.0, p.y - 34.0, 18.0, TEXT);
    }
}

struct State {
    skill: usize,
    case: usize,
    direction: usize,
    clock: f32,
    launched: Option<f32>,
    resolved: bool,
    turn: u16,
    energy: u16,
    cooldown: u16,
    hp: [u16; 3],
    blindness: [u16; 3],
    attracted: [bool; 3],
    attempts: [u16; 3],
    positions: [GridPos; 3],
    hit_at: [Option<f32>; 3],
    noise_turns: u16,
}
impl State {
    fn new(skill: usize, case: usize, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.0,
            launched: None,
            resolved: false,
            turn: 0,
            energy: 100,
            cooldown: 0,
            hp: [100; 3],
            blindness: [0; 3],
            attracted: [false; 3],
            attempts: [0; 3],
            hit_at: [None; 3],
            noise_turns: 0,
            positions: if skill == 1 {
                [GridPos::new(1, 3), GridPos::new(3, 6), GridPos::new(7, 7)]
            } else {
                [TARGET, GridPos::new(3, 4), GridPos::new(6, 3)]
            },
        }
    }
    fn rank(&self) -> u16 {
        if self.skill == 1 || self.skill == 3 {
            1
        } else {
            11
        }
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.case]).unwrap()
    }
    fn visible(&self, at: GridPos) -> bool {
        compute_visible_tiles(
            &self.map(),
            PLAYER,
            FieldOfViewRules {
                radius: 12,
                ..Default::default()
            },
        )
        .contains(&at)
    }
    fn sound_cells(&self) -> Vec<GridPos> {
        let rules = StealthRules::default();
        let reach = parameter_for("SG02", "open_terrain_sound_reach_cells", self.rank());
        rules
            .sound_field_on_map(
                &self.map(),
                1 + reach * rules.sound_attenuation_per_cell,
                TARGET,
            )
            .collect()
    }
    fn cells(&self) -> Vec<GridPos> {
        if self.skill == 1 {
            self.sound_cells()
        } else {
            affected_cells(&self.map(), TARGET, Area::Square { radius: 1 }, (1, 0))
        }
    }
    fn centre_damage(&self) -> u16 {
        match self.skill {
            2 => {
                ((24_u32
                    * (100
                        + u32::from(parameter_for(
                            "SG04",
                            "grenade_damage_bonus_percent",
                            self.rank(),
                        ))))
                    / 100) as u16
            }
            3 => 24,
            _ => 0,
        }
    }
    fn damage(&self, at: GridPos) -> u16 {
        super::super::area_falloff::damage(self.centre_damage(), TARGET, at, 1).unwrap_or(0)
    }
    fn can_launch(&self) -> bool {
        self.cooldown == 0
            && self.energy >= 8
            && self.map().is_walkable(TARGET)
            && self.visible(TARGET)
            && in_cast_range(
                PLAYER,
                TARGET,
                parameter_for("SG00", "base_throw_range_cells", 1),
            )
            && has_line_of_sight(&self.map(), PLAYER, TARGET, true)
    }
    fn launch(&mut self) {
        if !self.can_launch() {
            return;
        }
        self.launched = Some(self.clock);
        self.resolved = false;
        self.energy -= 8;
        self.cooldown = parameter_for("SG00", "cooldown_turns", 1);
        self.turn += 1;
    }
    fn refresh(&mut self) {
        if self.resolved || self.launched.is_none_or(|t| self.clock - t < FLIGHT) {
            return;
        }
        let cells = self.cells();
        if self.skill == 1 {
            self.noise_turns = parameter_for("SG02", "emission_turns", self.rank());
        }
        for (i, at) in self.positions.into_iter().enumerate() {
            if !cells.contains(&at) {
                continue;
            }
            self.hit_at[i] = self.launched.map(|t| t + FLIGHT);
            match self.skill {
                0 => {
                    self.blindness[i] =
                        parameter_for("SG01", "blindness_duration_turns", self.rank())
                }
                1 => {
                    self.attempts[i] += 1;
                    self.attracted[i] = true;
                }
                _ => self.hp[i] = self.hp[i].saturating_sub(self.damage(at)),
            }
        }
        self.resolved = true;
    }
    fn advance(&mut self, count: u16) {
        for _ in 0..count {
            self.turn += 1;
            self.cooldown = self.cooldown.saturating_sub(1);
            self.blindness = self.blindness.map(|v| v.saturating_sub(1));
            self.noise_turns = self.noise_turns.saturating_sub(1);
            // Prepared ordinary listeners; campaign AI, elites and occupied-centre rules are not rewritten.
            if self.skill == 1 {
                for i in 0..3 {
                    if !self.attracted[i]
                        || self.positions[i] == TARGET
                        || self.positions.contains(&TARGET)
                    {
                        continue;
                    }
                    let Some(path) = find_path(&self.map(), self.positions[i], TARGET, 81, |p| {
                        p != PLAYER && !self.positions.contains(&p)
                    }) else {
                        continue;
                    };
                    if let Some(next) = path.iter().copied().find(|p| *p != self.positions[i]) {
                        self.positions[i] = next;
                    }
                }
            }
        }
    }
    fn age(&self) -> Option<f32> {
        self.launched.map(|t| (self.clock - t).max(0.0))
    }
    fn record(&self) -> serde_json::Value {
        serde_json::json!({"clock":self.clock,"turn":self.turn,"energy":self.energy,"cooldown":self.cooldown,
            "hp":self.hp,"blindness":self.blindness,"attracted":self.attracted,"attempts":self.attempts,
            "positions":self.positions.map(|p|[p.x,p.y]),"noise_turns":self.noise_turns,"player_hp":100,
            "player_blindness_eligibility":"not_tested_not_decided","rank":self.rank(),"centre_damage":self.centre_damage(),
            "accuracy_percent":self.blindness.map(|v|if v>0{70}else{100}),"damage":self.positions.map(|p|if self.cells().contains(&p){self.damage(p)}else{0}),
            "visible_targets":self.positions.map(|p|self.visible(p)),"preview":self.launched.is_none(),"valid":self.can_launch(),
            "projectile":self.age().is_some_and(|a|a<FLIGHT),"explosion":self.age().is_some_and(|a|(FLIGHT..FLIGHT+BLAST).contains(&a)),
            "prepared_application":true})
    }
}

fn point(at: GridPos, direction: usize, r: Rect) -> Vec2 {
    let p = position(at, direction);
    vec2(
        r.x + (p.x as f32 + 0.5) * 64.0,
        r.y + (p.y as f32 + 0.5) * 64.0,
    )
}
fn load(bytes: &[u8]) -> Texture2D {
    let t = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
    t.set_filter(FilterMode::Linear);
    t
}
struct Art {
    grenade: [Texture2D; 3],
    simple_grenades: Texture2D,
    burst: Texture2D,
    blood: Texture2D,
    clip: Material,
    masks: Vec<Texture2D>,
}
impl Art {
    fn load() -> Self {
        let clip = load_material(ShaderSource::Glsl { vertex: r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;uniform mat4 Model;uniform mat4 Projection;varying highp vec2 p;varying highp vec2 uv;varying lowp vec4 tint;
void main(){p=position.xy;uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#,
        fragment: r#"#version 100
precision highp float;varying highp vec2 p;varying highp vec2 uv;varying lowp vec4 tint;uniform sampler2D Texture;uniform sampler2D mask;uniform vec2 origin;uniform float direction;uniform float keyBlack;
void main(){vec2 q=p-origin;if(direction<0.5)q=vec2(q.y,576.0-q.x);else if(direction<1.5)q=vec2(576.0-q.y,q.x);else if(direction>2.5)q=vec2(576.0)-q;if(q.x<0.0||q.y<0.0||q.x>=576.0||q.y>=576.0)discard;if(texture2D(mask,(floor(q/64.0)+vec2(0.5))/9.0).r<0.5)discard;vec4 c=texture2D(Texture,uv);if(keyBlack>0.5&&max(c.r,max(c.g,c.b))<0.02)discard;gl_FragColor=c*tint;}"# },
        MaterialParams { uniforms:vec![UniformDesc::new("origin",UniformType::Float2),UniformDesc::new("direction",UniformType::Float1),UniformDesc::new("keyBlack",UniformType::Float1)],textures:vec!["mask".into()],pipeline_params:PipelineParams{color_blend:Some(BlendState::new(Equation::Add,BlendFactor::Value(BlendValue::SourceAlpha),BlendFactor::OneMinusValue(BlendValue::SourceAlpha))),..Default::default()},..Default::default() }).unwrap();
        let masks = (0..4)
            .flat_map(|skill| (0..3).map(move |case| (skill, case)))
            .map(|(skill, case)| {
                let s = State::new(skill, case, 2);
                let cells = if skill == 1 {
                    s.sound_cells()
                } else {
                    affected_cells(&s.map(), TARGET, Area::Square { radius: 12 }, (1, 0))
                };
                let mut im = Image::gen_image_color(9, 9, BLACK);
                for p in cells {
                    if s.map().is_walkable(p) {
                        im.set_pixel(p.x as u32, p.y as u32, WHITE);
                    }
                }
                let t = Texture2D::from_image(&im);
                t.set_filter(FilterMode::Nearest);
                t
            })
            .collect();
        Self {
            simple_grenades: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-75/grenades-simple-source.png"
            )),
            grenade: [
                load(include_bytes!(
                    "../../../assets/animations/agent/skill-icon-review-53/SG01-C02-master.png"
                )),
                load(include_bytes!(
                    "../../../assets/animations/agent/skill-icon-review-53/SG02-C02-master.png"
                )),
                load(include_bytes!(
                    "../../../assets/animations/agent/skill-animation-review-72/grenade-source.png"
                )),
            ],
            burst: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-72/fragment-burst-source.png"
            )),
            blood: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-40/blood-atlas.png"
            )),
            clip,
            masks,
        }
    }
    fn mask(&self, s: &State, r: Rect, key: bool) {
        self.clip.set_uniform("origin", [r.x, r.y]);
        self.clip.set_uniform("direction", s.direction as f32);
        self.clip
            .set_uniform("keyBlack", if key { 1.0_f32 } else { 0.0 });
        self.clip
            .set_texture("mask", self.masks[s.skill * 3 + s.case].clone());
        gl_use_material(&self.clip);
    }
    fn device(&self, s: &State, r: Rect, p: Vec2, rotation: f32) {
        if refined_visuals() {
            // Tight alpha-bound source rectangles, with two source pixels of padding.
            // Full visible silhouette is about 18 native pixels tall, with flat colors.
            let source = [
                Rect::new(232.0, 27.0, 385.0, 666.0),
                Rect::new(864.0, 27.0, 386.0, 665.0),
                Rect::new(1545.0, 49.0, 446.0, 634.0),
            ][s.skill.min(2)];
            let size = vec2(18.0 * source.w / source.h, 18.0);
            self.mask(s, r, false);
            draw_texture_ex(
                &self.simple_grenades,
                p.x - size.x / 2.0,
                p.y - size.y / 2.0,
                WHITE,
                DrawTextureParams {
                    source: Some(source),
                    dest_size: Some(size),
                    rotation,
                    ..Default::default()
                },
            );
            gl_use_default_material();
            return;
        }
        self.mask(s, r, s.skill < 2);
        let size = if s.skill < 2 { 36.0 } else { 32.0 };
        draw_texture_ex(
            &self.grenade[s.skill.min(2)],
            p.x - size / 2.0,
            p.y - size / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                rotation,
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
    fn effects(&self, s: &State, r: Rect) {
        let Some(age) = s.age() else {
            return;
        };
        let to = point(TARGET, s.direction, r);
        if age < FLIGHT {
            let t = age / FLIGHT;
            let p = (point(PLAYER, s.direction, r) + vec2(12.0, -8.0)).lerp(to, t)
                + vec2(0.0, -(t * std::f32::consts::PI).sin() * 30.0);
            self.device(s, r, p, t * 2.7);
            return;
        }
        let a = age - FLIGHT;
        self.mask(s, r, false);
        if s.skill == 0 && a < 0.32 {
            let u = a / 0.32;
            let opacity = (1.0 - u).powf(1.2);
            draw_circle(to.x, to.y, 16.0 + 68.0 * u, alpha(WHITE, 0.55 * opacity));
            for k in 0..12 {
                let theta = k as f32 * std::f32::consts::TAU / 12.0 + 0.12;
                let dir = vec2(theta.cos(), theta.sin());
                let from = to + dir * (14.0 + 23.0 * u);
                let end = to + dir * (40.0 + 66.0 * u);
                draw_line(
                    from.x,
                    from.y,
                    end.x,
                    end.y,
                    5.0 * opacity + 1.0,
                    alpha(WHITE, opacity),
                );
            }
        } else if s.skill == 1 && s.noise_turns > 0 {
            let u = (a / 0.95).fract();
            let radius = 28.0 + u * 155.0;
            let color = alpha(Color::from_rgba(255, 194, 76, 255), 0.85 * (1.0 - u));
            draw_circle_lines(to.x, to.y, radius, 6.5, alpha(BLACK, 0.65 * (1.0 - u)));
            draw_circle_lines(to.x, to.y, radius, 4.0, color);
            draw_circle_lines(to.x, to.y, radius * 0.75, 2.0, alpha(color, 0.40));
        } else if s.skill >= 2 && a < BLAST {
            let frame = ((a / BLAST * 6.0) as u32).min(5);
            draw_texture_ex(
                &self.burst,
                to.x - 110.0,
                to.y - 110.0,
                alpha(WHITE, 0.94),
                DrawTextureParams {
                    source: Some(Rect::new(
                        (frame % 3) as f32 * 512.0,
                        (frame / 3) as f32 * 512.0,
                        512.0,
                        512.0,
                    )),
                    dest_size: Some(vec2(220.0, 220.0)),
                    ..Default::default()
                },
            );
        }
        gl_use_default_material();
        if s.skill == 1 && s.noise_turns > 0 {
            self.device(s, r, to, 0.0);
            draw_circle(
                to.x + 2.0,
                to.y - 2.0,
                if refined_visuals() { 1.8 } else { 2.8 },
                alpha(
                    Color::from_rgba(255, 200, 75, 255),
                    0.55 + 0.45 * (a * 6.0).sin().abs(),
                ),
            );
        }
    }
    fn blood(&self, s: &State, r: Rect) {
        if s.skill < 2 {
            return;
        }
        self.mask(s, r, false);
        for (i, at) in s.positions.into_iter().enumerate() {
            if let Some(t) = s.hit_at[i] {
                let age = s.clock - t;
                if (0.0..0.30).contains(&age) {
                    let p = point(at, s.direction, r);
                    let frame = ((age / 0.30 * 6.0) as u32).min(5);
                    draw_texture_ex(
                        &self.blood,
                        p.x - 31.0,
                        p.y - 25.0,
                        WHITE,
                        DrawTextureParams {
                            source: Some(Rect::new(frame as f32 * 256.0, 0.0, 256.0, 256.0)),
                            dest_size: Some(vec2(62.0, 62.0)),
                            ..Default::default()
                        },
                    );
                }
            }
        }
        gl_use_default_material();
    }
}
fn draw(s: &State, art: &Art, r: Rect, mode: u8) {
    terrain(&s.map(), s.direction, r, false);
    if mode == 0 && s.launched.is_none() {
        preview(
            &s.cells()
                .into_iter()
                .filter(|p| s.map().is_walkable(*p) && s.visible(*p))
                .collect::<Vec<_>>(),
            s.direction,
            r,
            s.clock,
        );
    }
    terrain(&s.map(), s.direction, r, true);
    if mode == 0 {
        art.blood(s, r);
    }
    if mode < 2 {
        for (i, at) in s.positions.into_iter().enumerate() {
            if !s.visible(at) {
                continue;
            }
            let p = point(at, s.direction, r);
            let offset = if mode == 0 {
                s.hit_at[i]
                    .map(|t| {
                        crate::combat_visuals::hit_reaction_offset(
                            p,
                            point(TARGET, s.direction, r),
                            f64::from(s.clock - t),
                            64.0,
                        )
                    })
                    .unwrap_or(Vec2::ZERO)
            } else {
                Vec2::ZERO
            };
            actor(p + offset, Visual::Actor(5));
            if mode == 0 && s.blindness[i] > 0 {
                blindness_marker(p, s.blindness[i]);
            }
        }
        actor(
            point(PLAYER, s.direction, r),
            super::super::melee::player_visual(),
        );
    }
    if mode == 0 || mode == 2 {
        art.effects(s, r);
    }
    if mode == 0 {
        for (i, at) in s.positions.into_iter().enumerate() {
            if !s.visible(at) {
                continue;
            }
            if let Some(t) = s.hit_at[i] {
                let a = s.clock - t;
                if (0.0..0.65).contains(&a) {
                    let p = point(at, s.direction, r);
                    let c = alpha(TEXT, 1.0 - a / 0.65);
                    let label = match s.skill {
                        0 => "Aveuglé".into(),
                        1 => "Attiré".into(),
                        _ => format!("{}", s.damage(at)),
                    };
                    draw_text_bold(label, p.x - 27.0, p.y - 57.0 - a * 14.0, 20.0, c);
                    if s.skill >= 2 {
                        draw_text(
                            format!("(   {})", s.damage(at)),
                            p.x - 26.0,
                            p.y - 40.0 - a * 14.0,
                            16.0,
                            c,
                        );
                        draw_line(
                            p.x - 17.0,
                            p.y - 46.0 - a * 14.0,
                            p.x - 10.0,
                            p.y - 39.0 - a * 14.0,
                            2.4,
                            c,
                        );
                    }
                }
            }
        }
    }
}
fn hud(s: &State, r: Rect) {
    draw_text_bold(
        format!("{} · {}", NUMBERS[s.skill], TITLES[s.skill]),
        r.x,
        r.y - 37.0,
        20.0,
        TEXT,
    );
    let detail = match s.skill {
        0 => "Rang effectif 11 · précision −30 % · 12 tours".into(),
        1 => "Rang 1 · portée sonore 3 cases · 2 tours".into(),
        2 => format!(
            "Rang effectif 11 · +110 % · centre {} / bord {}",
            s.centre_damage(),
            s.centre_damage() / 2
        ),
        _ => "Sans passif · centre 24 / bord 12".into(),
    };
    draw_text(detail, r.x, r.y - 15.0, 14.0, DIM);
    draw_text(
        format!(
            "Tour {} · énergie {} · recharge {} · PV joueur 100",
            s.turn, s.energy, s.cooldown
        ),
        r.x + 8.0,
        r.bottom() + 24.0,
        15.0,
        TEXT,
    );
    draw_text(
        if s.skill == 1 {
            format!(
                "Bruit : {} tour(s) · recherche sur les tours suivants",
                s.noise_turns
            )
        } else if s.skill == 0 {
            format!(
                "Aveuglement restant : {} / {} tours",
                s.blindness[0], s.blindness[1]
            )
        } else {
            format!("PV des deux cibles : {} / {}", s.hp[0], s.hp[1])
        },
        r.x + 8.0,
        r.bottom() + 46.0,
        14.0,
        DIM,
    );
}
pub(crate) async fn run() {
    let art = Art::load();
    let mut pair = 0;
    let mut states = [State::new(0, 0, 2), State::new(1, 0, 2)];
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        for s in &mut states {
            s.clock += get_frame_time();
            s.refresh();
        }
        if is_key_pressed(KeyCode::Space) {
            for s in &mut states {
                s.launch();
            }
        }
        if is_key_pressed(KeyCode::N) {
            for s in &mut states {
                s.advance(1);
            }
        }
        if is_key_pressed(KeyCode::R) {
            states = states.map(|s| State::new(s.skill, s.case, s.direction));
        }
        if is_key_pressed(KeyCode::W) {
            states = states.map(|s| State::new(s.skill, (s.case + 1) % 3, s.direction));
        }
        if is_key_pressed(KeyCode::D) {
            states = states.map(|s| State::new(s.skill, s.case, (s.direction + 1) % 4));
        }
        if is_key_pressed(KeyCode::P) {
            pair = 1 - pair;
            states = if pair == 0 {
                [State::new(0, 0, 2), State::new(1, 0, 2)]
            } else {
                [State::new(3, 0, 2), State::new(2, 0, 2)]
            };
        }
        clear_background(INK);
        draw_text(
            "Grenades · Espace : lancer · N : tour · P : comparaison · W : décor · D : direction · R : reset",
            32.0,
            28.0,
            16.0,
            TEXT,
        );
        for (i, s) in states.iter().enumerate() {
            let r = Rect::new(32.0 + i as f32 * 616.0, 88.0, 576.0, 576.0);
            draw(s, &art, r, 0);
            hud(s, r);
        }
        next_frame().await;
    }
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    if output.exists() {
        return Err("Dossier de capture existant.".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let art = Art::load();
    let target = render_target(640, 720);
    let mut entries = Vec::new();
    let delta = std::env::args().any(|arg| arg == "--capture-grenades-visual-delta");
    for skill in 0..4 {
        if std::env::args().any(|arg| {
            arg.starts_with("--capture-grenade-id=")
                && arg != format!("--capture-grenade-id={}", IDS[skill])
        }) {
            continue;
        }
        for case in 0..3 {
            for direction in 0..4 {
                let mut s = State::new(skill, case, direction);
                let revision = if refined_visuals() {
                    if skill == 3 { "A03" } else { "A02" }
                } else if skill == 3 {
                    "A02"
                } else {
                    "A01"
                };
                let folder = format!("{}-{}-C{}-{}", IDS[skill], revision, case, DIRS[direction]);
                let dir = output.join(&folder);
                std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
                let mut timeline = Vec::new();
                for n in 0..128 {
                    s.clock = n as f32 / 24.0;
                    if n == 18 {
                        s.launch();
                    }
                    s.refresh();
                    if s.launched.is_some() {
                        if n == 64 || n == 88 {
                            s.advance(1);
                        }
                        if n == 112 {
                            s.advance(10);
                        }
                    }
                    timeline.push(s.record());
                    let r = Rect::new(32.0, 88.0, 576.0, 576.0);
                    for (mode, name) in [
                        (0, format!("{n:03}.rgba")),
                        (1, format!("control-{n:03}.rgba")),
                        (2, format!("effect-proof-{n:03}.rgba")),
                        (3, format!("terrain-proof-{n:03}.rgba")),
                    ] {
                        if mode > 0 && n != 27 {
                            continue;
                        }
                        if delta {
                            let changed = case != 2
                                && match skill {
                                    0 => (18..112).contains(&n),
                                    1 => (18..88).contains(&n),
                                    _ => (18..23).contains(&n),
                                };
                            // Other layers and unchanged frames are reused from native review74.
                            if !(mode == 0 && changed
                                || mode == 2 && skill == 1 && changed && n == 27)
                            {
                                continue;
                            }
                        }
                        set_camera(&Camera2D {
                            render_target: Some(target.clone()),
                            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 640.0, 720.0))
                        });
                        clear_background(INK);
                        draw(&s, &art, r, mode);
                        if mode < 2 {
                            hud(&s, r);
                        }
                        super::occult::export(&target, &dir.join(name));
                        set_default_camera();
                    }
                }
                std::fs::write(
                    dir.join("timeline.json"),
                    serde_json::to_vec_pretty(&timeline).unwrap(),
                )
                .map_err(|e| e.to_string())?;
                entries.push(serde_json::json!({"id":IDS[skill],"number":NUMBERS[skill],"title":TITLES[skill],"folder":folder,"variant":folder,"case":case,"case_title":CASES[case],"direction":direction,"tile":64,"frames":128,"fps":24,"launch_frame":18,"turn_frames":[64,88,112],"turn_counts":[1,1,10],"map":MAPS[case],"target":[TARGET.x,TARGET.y],"player":[PLAYER.x,PLAYER.y],"mechanical_cells":s.cells().iter().map(|p|[p.x,p.y]).collect::<Vec<_>>(),"rank":s.rank()}));
            }
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn resolved(skill: usize) -> State {
        let mut s = State::new(skill, 0, 2);
        s.launch();
        s.clock = FLIGHT;
        s.refresh();
        s
    }
    #[test]
    fn utility_damage_and_turn_bound_expiry_are_preserved() {
        let mut flash = resolved(0);
        assert_eq!(flash.hp, [100; 3]);
        assert_eq!(flash.blindness, [12, 12, 0]);
        flash.clock = 600.0;
        flash.refresh();
        assert_eq!(
            (flash.turn, flash.cooldown, flash.blindness),
            (1, 1, [12, 12, 0])
        );
        flash.advance(12);
        assert_eq!(flash.blindness, [0; 3]);
        assert_eq!(
            flash.record()["accuracy_percent"],
            serde_json::json!([100, 100, 100])
        );
        let mut lure = resolved(1);
        assert_eq!(lure.hp, [100; 3]);
        assert_eq!(lure.attempts, [1, 1, 0]);
        assert_eq!(lure.noise_turns, 2);
        lure.clock = 600.0;
        lure.refresh();
        assert_eq!(lure.noise_turns, 2);
        lure.advance(2);
        assert_eq!(lure.noise_turns, 0);
        assert_eq!(lure.attempts, [1, 1, 0]);
        assert!(lure.attracted[0]);
        let positions = lure.positions;
        lure.advance(1);
        assert_ne!(lure.positions, positions);
    }
    #[test]
    fn damage_bonus_preserves_geometry_and_uniform_utility_immunity() {
        let power = resolved(2);
        let base = resolved(3);
        assert_eq!(power.cells(), base.cells());
        assert_eq!((base.centre_damage(), power.centre_damage()), (24, 50));
        assert_eq!(base.hp, [76, 88, 100]);
        assert_eq!(power.hp, [50, 75, 100]);
        assert_eq!((power.energy, power.cooldown), (92, 1));
        assert_eq!(power.record()["player_hp"], 100);
    }
    #[test]
    fn blocked_cast_is_atomic_for_every_variant() {
        for skill in 0..4 {
            let mut s = State::new(skill, 2, 2);
            s.launch();
            s.clock = 10.0;
            s.refresh();
            assert!(s.launched.is_none());
            assert_eq!((s.turn, s.energy, s.cooldown), (0, 100, 0));
            assert_eq!(s.hp, [100; 3]);
            assert_eq!(s.blindness, [0; 3]);
            assert_eq!(s.noise_turns, 0);
        }
    }
    #[test]
    fn sound_preview_uses_actual_attenuation_and_does_not_use_an_explosion_mask() {
        let mut lure = State::new(1, 1, 2);
        assert_eq!(lure.rank(), 1);
        let rules = StealthRules::default();
        let intensity = 16;
        for y in 0..9 {
            for x in 0..9 {
                let p = GridPos::new(x, y);
                assert_eq!(
                    lure.sound_cells().contains(&p),
                    rules.sound_reaches_on_map(&lure.map(), intensity, TARGET, p)
                );
            }
        }
        assert!(!lure.sound_cells().contains(&GridPos::new(6, 3)));
        lure.launch();
        lure.clock = FLIGHT;
        lure.refresh();
        assert_eq!(lure.attempts, [1, 1, 0]);
    }
}
