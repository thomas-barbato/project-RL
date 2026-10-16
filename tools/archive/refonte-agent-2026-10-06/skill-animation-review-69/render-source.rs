//! SM05 visual review: immediate effects, then a fixed, fully marked ground zone.
//! Prepared debug scenes only; no campaign, AI, saves or new balance rules.
use super::super::melee::actor;
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;

const SIZE: f32 = 576.0;
const MINE: GridPos = GridPos::new(4, 5);
const PLAYER: GridPos = GridPos::new(4, 6);
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#########",
];
const TYPES: [&str; 3] = ["FEU", "POISON", "FROID"];
const NAMES: [&str; 3] = ["Sol enflammé", "Nuage de poison", "Sol gelé"];
const FRAMES: usize = 480;
fn actions() -> Vec<usize> {
    [64, 88, 112]
        .into_iter()
        .chain((0..13).map(|n| 160 + n * 24))
        .collect()
}
fn map(wall: bool) -> Map {
    Map::from_ascii(MAPS[usize::from(wall)]).unwrap()
}
fn area(wall: bool) -> Vec<GridPos> {
    super::mines_geometry::square(&map(wall), MINE, 1)
}
fn point(at: GridPos, direction: usize, r: Rect) -> Vec2 {
    let p = position(at, direction);
    vec2(
        r.x + (p.x as f32 + 0.5) * 64.0,
        r.y + (p.y as f32 + 0.5) * 64.0,
    )
}
// Cell-local visual noise; it never advances the gameplay RNG or a turn.
fn visual_hash(mut n: u32) -> u32 {
    n ^= n >> 16;
    n = n.wrapping_mul(0x7feb_352d);
    n ^= n >> 15;
    n = n.wrapping_mul(0x846c_a68b);
    n ^ (n >> 16)
}
fn cell_seed(at: GridPos) -> u32 {
    visual_hash((at.x as u32).wrapping_mul(73_856_093) ^ (at.y as u32).wrapping_mul(19_349_663))
}
fn unit(n: u32) -> f32 {
    (visual_hash(n) & 0xffff) as f32 / 65535.0
}
fn gas_states(at: GridPos, clock: f32) -> (usize, usize, f32) {
    let seed = cell_seed(at);
    let period = 0.72 + unit(seed) * 0.83;
    let t = clock / period + unit(seed ^ 19) * 13.0;
    let epoch = t.floor() as u32;
    let frame = |epoch| 3 + (visual_hash(seed ^ visual_hash(epoch)) % 3) as usize;
    let mix = (t.fract() * period / 0.25).min(1.0);
    (
        frame(epoch.saturating_sub(1)),
        frame(epoch),
        mix * mix * (3.0 - 2.0 * mix),
    )
}
#[derive(Clone)]
struct State {
    kind: usize,
    wall: bool,
    direction: usize,
    clock: f32,
    begun: bool,
    step: usize,
    blast: Option<f32>,
    remaining: u16,
    applications: u16,
    hp: [u16; 4],
    status: [u16; 4],
    zone_gel: [bool; 4],
    exposures: [u16; 4],
    poison_ticks: [u16; 4],
    last_damage: [u16; 4],
    hit_at: [Option<f32>; 4],
    entrant_inside: bool,
}
impl State {
    fn new(kind: usize, wall: bool, direction: usize) -> Self {
        Self {
            kind,
            wall,
            direction,
            clock: 0.0,
            begun: false,
            step: 0,
            blast: None,
            remaining: 0,
            applications: 0,
            hp: [400; 4],
            status: [0; 4],
            zone_gel: [false; 4],
            exposures: [0; 4],
            poison_ticks: [0; 4],
            last_damage: [0; 4],
            hit_at: [None; 4],
            entrant_inside: false,
        }
    }
    fn targets(&self) -> [GridPos; 4] {
        [
            if self.step < 2 {
                GridPos::new(3, 3)
            } else if self.step == 2 {
                GridPos::new(4, 4)
            } else {
                MINE
            },
            GridPos::new(3, 5),
            if self.entrant_inside {
                GridPos::new(4, 4)
            } else {
                GridPos::new(4, 3)
            },
            GridPos::new(6, 5),
        ]
    }
    fn color(&self) -> Color {
        [
            Color::new(1.0, 0.40, 0.12, 1.0),
            Color::new(0.38, 0.94, 0.12, 1.0),
            Color::new(0.28, 0.78, 1.0, 1.0),
        ][self.kind]
    }
    fn damage(&mut self, i: usize, n: u16) {
        if n > 0 {
            self.hp[i] -= n;
            self.last_damage[i] += n;
            self.hit_at[i] = Some(self.clock);
        }
    }
    fn zone_tick(&mut self) {
        self.applications += 1;
        let covered = area(self.wall);
        for (i, p) in self.targets().into_iter().enumerate() {
            if !covered.contains(&p) {
                self.exposures[i] = 0;
                self.zone_gel[i] = false;
                continue;
            }
            self.exposures[i] += 1;
            match self.kind {
                0 => self.damage(i, 3),
                1 => self.damage(i, 4),
                _ => {
                    if !self.zone_gel[i] {
                        self.hit_at[i] = Some(self.clock);
                    }
                    self.zone_gel[i] = true;
                }
            }
        }
    }
    fn advance(&mut self) {
        if !self.begun || self.step >= 16 {
            return;
        }
        self.step += 1;
        self.last_damage = [0; 4];
        if self.step == 3 {
            self.blast = Some(self.clock);
            self.remaining = 13;
            for (i, p) in self.targets().into_iter().enumerate() {
                if area(self.wall).contains(&p) {
                    self.damage(i, [13, 19, 16][self.kind]);
                    if self.kind == 1 {
                        self.status[i] = 5;
                        self.poison_ticks[i] = 1;
                        self.damage(i, 42);
                    }
                    if self.kind == 2 {
                        self.status[i] = 5;
                    }
                }
            }
            // The first ground application is immediate and counts among the 13.
            self.zone_tick();
        } else if self.step > 3 {
            if self.step == 4 {
                self.entrant_inside = true;
            }
            for i in 0..4 {
                self.status[i] = self.status[i].saturating_sub(1);
                if self.kind == 1 && self.status[i] > 0 && self.poison_ticks[i] < 5 {
                    self.poison_ticks[i] += 1;
                    self.damage(i, 42);
                }
            }
            self.remaining = self.remaining.saturating_sub(1);
            if self.remaining > 0 {
                self.zone_tick();
            } else {
                self.zone_gel = [false; 4];
            }
        }
    }
    fn duration(&self, i: usize) -> u16 {
        if self.zone_gel[i] {
            self.remaining.max(self.status[i])
        } else {
            self.status[i]
        }
    }
    fn variant(&self) -> String {
        format!(
            "SM05-{}-{}-{}",
            if self.kind == 2 { "A05" } else { "A03" },
            TYPES[self.kind],
            if self.wall {
                "CLOISON"
            } else {
                ["DROITE", "GAUCHE", "HAUT", "BAS"][self.direction]
            }
        )
    }
}
struct Art {
    mine: Texture2D,
    lens: Material,
    bursts: [Texture2D; 3],
    flame: Texture2D,
    gel: super::frost::gel_status::GelArt,
    status_poison: Texture2D,
    ice_floor: Texture2D,
    icons: [Texture2D; 3],
    marker: Material,
    masks: [Texture2D; 2],
}
impl Art {
    fn load() -> Self {
        use macroquad::miniquad::{
            BlendFactor, BlendState, BlendValue, Equation, PipelineParams, UniformDesc, UniformType,
        };
        let load = |b: &[u8]| {
            let t = Texture2D::from_file_with_format(b, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        let vertex = r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;uniform mat4 Model;uniform mat4 Projection;
varying highp vec2 p;varying highp vec2 uv;varying lowp vec4 tint;
void main(){p=position.xy;uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#;
        let blend = PipelineParams {
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
            )),
            ..Default::default()
        };
        let lens=load_material(ShaderSource::Glsl{vertex,fragment:r#"#version 100
precision highp float;varying highp vec2 uv;varying lowp vec4 tint;uniform sampler2D Texture;uniform vec3 lensTint;
void main(){vec4 c=texture2D(Texture,uv)*tint;if(uv.x>0.46&&uv.x<0.69&&uv.y>0.345&&uv.y<0.46&&c.r>c.g*1.45&&c.r>c.b*1.45)c.rgb=lensTint*c.r;gl_FragColor=c;}"#},MaterialParams{uniforms:vec![UniformDesc::new("lensTint",UniformType::Float3)],pipeline_params:blend,..Default::default()}).unwrap();
        let marker=load_material(ShaderSource::Glsl{vertex,fragment:r#"#version 100
precision highp float;varying highp vec2 p;varying highp vec2 uv;varying lowp vec4 tint;uniform sampler2D Texture;uniform sampler2D zoneMask;uniform vec2 origin;uniform float direction;
void main(){vec2 q=p-origin;if(direction<0.5)q=vec2(q.y,576.0-q.x);else if(direction<1.5)q=vec2(576.0-q.y,q.x);else if(direction>2.5)q=vec2(576.0)-q;
if(q.x<0.0||q.y<0.0||q.x>=576.0||q.y>=576.0)discard;float allowed=texture2D(zoneMask,(floor(q/64.0)+vec2(0.5))/9.0).r;if(allowed<0.5)discard;gl_FragColor=texture2D(Texture,uv)*tint;}"#},MaterialParams{uniforms:vec![UniformDesc::new("origin",UniformType::Float2),UniformDesc::new("direction",UniformType::Float1)],textures:vec!["zoneMask".into()],pipeline_params:blend,..Default::default()}).unwrap();
        let masks = [false, true].map(|wall| {
            let mut im = Image::gen_image_color(9, 9, BLACK);
            for p in area(wall) {
                im.set_pixel(p.x as u32, p.y as u32, WHITE);
            }
            let t = Texture2D::from_image(&im);
            t.set_filter(FilterMode::Nearest);
            t
        });
        Self {
            mine: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-60/mine-sprite.png"
            )),
            lens,
            marker,
            masks,
            bursts: [
                load(include_bytes!(
                    "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-63/fire-burst-atlas.png"
                )),
                load(include_bytes!(
                    "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-62/poison-burst-atlas.png"
                )),
                load(include_bytes!(
                    "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-62/frost-burst-atlas.png"
                )),
            ],
            flame: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-06/flame-atlas.png"
            )),
            gel: super::frost::gel_status::GelArt::load_approved(),
            status_poison: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-61/poison-burst-atlas.png"
            )),
            ice_floor: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-68/snow-ice-source.png"
            )),
            icons: [
                load(include_bytes!(
                    "../../../assets/skills/agent/SM00-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/SM02-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/SM03-compact.png"
                )),
            ],
        }
    }
    fn ground(&self, s: &State, r: Rect) {
        let Some(start) = s.blast else {
            return;
        };
        if s.remaining == 0 || s.clock - start < [0.44, 0.46, 0.40][s.kind] {
            return;
        }
        self.marker.set_uniform("origin", [r.x, r.y]);
        self.marker.set_uniform("direction", s.direction as f32);
        self.marker
            .set_texture("zoneMask", self.masks[usize::from(s.wall)].clone());
        gl_use_material(&self.marker);
        if s.kind == 2 {
            // One continuous snow/frost coating, rather than repeating a tile
            // texture on each square. The effective zone still clips the paint.
            let p = point(MINE, s.direction, r);
            let rotation = [
                -std::f32::consts::FRAC_PI_2,
                std::f32::consts::FRAC_PI_2,
                0.0,
                std::f32::consts::PI,
            ][s.direction];
            draw_texture_ex(
                &self.ice_floor,
                p.x - 108.0,
                p.y - 108.0,
                alpha(WHITE, 0.72 + (s.clock * 0.65).sin() * 0.025),
                DrawTextureParams {
                    dest_size: Some(vec2(216.0, 216.0)),
                    rotation,
                    ..Default::default()
                },
            );
            gl_use_default_material();
            return;
        }
        for (i, at) in area(s.wall).into_iter().enumerate() {
            let p = point(at, s.direction, r);
            if s.kind == 0 {
                draw_ellipse(
                    p.x,
                    p.y + 18.0,
                    32.0,
                    13.0,
                    0.0,
                    alpha(Color::new(0.22, 0.07, 0.015, 1.0), 0.62),
                );
                for (dx, dy) in [(-18.0, 6.0), (15.0, 14.0), (0.0, -13.0)] {
                    let f = ((s.clock * 8.0) as usize + i) % 8;
                    draw_texture_ex(
                        &self.flame,
                        p.x + dx - 30.0,
                        p.y + dy - 38.0,
                        alpha(WHITE, 0.79),
                        DrawTextureParams {
                            source: Some(Rect::new(f as f32 * 128.0, 0.0, 128.0, 128.0)),
                            dest_size: Some(vec2(60.0, 48.0)),
                            ..Default::default()
                        },
                    );
                }
            } else if s.kind == 1 {
                let seed = cell_seed(at);
                let phase = unit(seed ^ 31) * std::f32::consts::TAU;
                let dx = (s.clock * (0.7 + unit(seed ^ 47)) + phase).sin() * 5.0
                    + (s.clock * 1.7 + phase * 2.3).sin() * 2.0;
                let dy = (s.clock * (0.5 + unit(seed ^ 61)) + phase * 1.9).sin() * 4.0;
                let width = 94.0 + (s.clock * 0.8 + phase).sin() * 5.0;
                let height = 84.0 + (s.clock * 1.1 + phase * 0.7).sin() * 3.0;
                let (from, to, mix) = gas_states(at, s.clock);
                // Smoothly blend independently selected states, instead of marching
                // through the same three states along the rows of the zone.
                for (frame, opacity) in [(from, 1.0 - mix), (to, mix)] {
                    if opacity > 0.0 {
                        draw_texture_ex(
                            &self.bursts[1],
                            p.x - width * 0.5 + dx,
                            p.y - height * 0.5 + dy,
                            alpha(WHITE, 0.62 * opacity),
                            DrawTextureParams {
                                source: Some(Rect::new(frame as f32 * 256.0, 0.0, 256.0, 256.0)),
                                dest_size: Some(vec2(width, height)),
                                flip_x: seed & 1 != 0,
                                flip_y: seed & 2 != 0,
                                ..Default::default()
                            },
                        );
                    }
                }
            }
        }
        gl_use_default_material();
    }
    fn mine(&self, s: &State, r: Rect) {
        if !s.begun || s.blast.is_some() {
            return;
        }
        let p = point(MINE, s.direction, r);
        let c = s.color();
        self.lens.set_uniform("lensTint", [c.r, c.g, c.b]);
        gl_use_material(&self.lens);
        draw_texture_ex(
            &self.mine,
            p.x - 41.3,
            p.y - 16.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(72.0, 72.0)),
                ..Default::default()
            },
        );
        gl_use_default_material();
        if (s.clock * 24.0).round() as usize % 18 < 7 {
            draw_ellipse(p.x, p.y + 13.25, 5.2, 2.3, 0.0, c);
        }
    }
    fn burst(&self, s: &State, r: Rect, clip: &Clip) {
        let Some(start) = s.blast else {
            return;
        };
        let age = s.clock - start;
        let duration = [0.44, 0.46, 0.40][s.kind];
        if !(0.0..duration).contains(&age) {
            return;
        }
        let p = point(MINE, s.direction, r);
        let f = ((age / duration * 6.0) as usize).min(5);
        clip.apply_for(usize::from(s.wall), s.direction, r);
        draw_texture_ex(
            &self.bursts[s.kind],
            p.x - 90.0,
            p.y - 94.0,
            alpha(WHITE, [0.88, 0.82, 0.82][s.kind]),
            DrawTextureParams {
                source: Some(Rect::new(f as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(vec2(180.0, 180.0)),
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
    fn bodies(&self, s: &State, r: Rect, effects: bool, clip: &Clip) {
        for (i, at) in s.targets().into_iter().enumerate() {
            let mut p = point(at, s.direction, r);
            if effects {
                if let Some(t) = s.hit_at[i] {
                    p += crate::combat_visuals::hit_reaction_offset(
                        p,
                        point(MINE, s.direction, r),
                        f64::from(s.clock - t),
                        64.0,
                    );
                }
            }
            if effects && s.kind == 1 && s.status[i] > 0 {
                clip.apply_for(usize::from(s.wall), s.direction, r);
                let f = 4 + ((s.clock * 2.0 + i as f32) as usize % 2);
                for (dx, offset) in [(-11.0, 0.0), (12.0, 1.1)] {
                    draw_texture_ex(
                        &self.status_poison,
                        p.x + dx - 24.0,
                        p.y - 16.0 - offset * 3.0,
                        alpha(WHITE, 0.52),
                        DrawTextureParams {
                            source: Some(Rect::new(f as f32 * 256.0, 0.0, 256.0, 256.0)),
                            dest_size: Some(vec2(48.0, 48.0)),
                            ..Default::default()
                        },
                    );
                }
                gl_use_default_material();
            }
            if effects && s.kind == 2 && s.duration(i) > 0 {
                clip.apply_for(usize::from(s.wall), s.direction, r);
                self.gel.draw(p, s.clock, i * 2, false);
                gl_use_default_material();
            }
            actor(
                p,
                Visual::Bestiary {
                    number: 12 + i as u8,
                    corpse: false,
                },
            );
            if effects && s.kind == 2 && s.duration(i) > 0 {
                clip.apply_for(usize::from(s.wall), s.direction, r);
                self.gel.draw(p, s.clock, i * 2, true);
                gl_use_default_material();
            }
        }
        actor(
            point(PLAYER, s.direction, r),
            game_art::player(Some("core:technomancien"), PlayerWeaponPose::Unarmed, None),
        );
    }
    fn hud(&self, s: &State, r: Rect) {
        draw_rectangle(r.x + 6.0, r.y + 6.0, 354.0, 84.0, alpha(INK, 0.95));
        draw_text_bold(
            format!("73 · {}", NAMES[s.kind]),
            r.x + 16.0,
            r.y + 29.0,
            19.0,
            TEXT,
        );
        draw_text(
            "Résidu persistant · rang 11 · durée 13 tours",
            r.x + 16.0,
            r.y + 51.0,
            14.0,
            TEXT,
        );
        draw_text(
            "Rayon 1 case · sans Explosion étendue",
            r.x + 16.0,
            r.y + 72.0,
            13.0,
            DIM,
        );
        for (i, p) in s.targets().into_iter().enumerate() {
            let p = point(p, s.direction, r);
            let duration = s.duration(i);
            if duration > 0 {
                draw_texture_ex(
                    &self.icons[s.kind],
                    p.x + 24.0,
                    p.y - 31.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(18.0, 18.0)),
                        ..Default::default()
                    },
                );
                draw_text(duration.to_string(), p.x + 27.0, p.y + 3.0, 15.0, s.color());
            }
            if let Some(age) = s.hit_at[i]
                .map(|t| s.clock - t)
                .filter(|a| (0.0..0.65).contains(a))
                .filter(|_| s.last_damage[i] > 0)
            {
                let y = p.y - 43.0 - age * 16.0;
                let c = alpha(s.color(), 1.0 - age / 0.65);
                draw_text_bold(s.last_damage[i].to_string(), p.x - 14.0, y, 24.0, c);
                draw_text("(", p.x - 28.0, y + 18.0, 15.0, c);
                draw_texture_ex(
                    &self.icons[s.kind],
                    p.x - 22.0,
                    y + 3.0,
                    alpha(WHITE, 1.0 - age / 0.65),
                    DrawTextureParams {
                        dest_size: Some(vec2(16.0, 16.0)),
                        ..Default::default()
                    },
                );
                draw_text(
                    format!("{})", s.last_damage[i]),
                    p.x - 3.0,
                    y + 18.0,
                    15.0,
                    c,
                );
            }
        }
        draw_rectangle(r.x + 5.0, r.y + SIZE - 70.0, 566.0, 65.0, alpha(INK, 0.96));
        draw_text(
            format!(
                "Tour {} · zone : {} tours · applications : {}/13",
                usize::from(s.begun) + s.step,
                s.remaining,
                s.applications
            ),
            r.x + 13.0,
            r.y + SIZE - 50.0,
            15.0,
            TEXT,
        );
        draw_text(
            if s.step < 3 {
                "Pose, armement puis entrée sur la mine"
            } else if s.step == 3 {
                "Explosion puis résidu fixe · première application immédiate"
            } else if s.remaining > 0 {
                "Un ennemi est entré dans la zone · le terrain reste dangereux"
            } else {
                "Zone expirée · le marqueur et le Gel de zone disparaissent"
            },
            r.x + 13.0,
            r.y + SIZE - 29.0,
            12.0,
            DIM,
        );
        draw_text(
            format!(
                "PV : {} / {} / {} / {} · joueur 100/100",
                s.hp[0], s.hp[1], s.hp[2], s.hp[3]
            ),
            r.x + 13.0,
            r.y + SIZE - 10.0,
            13.0,
            TEXT,
        );
    }
}
fn scene(art: &Art, clip: &Clip, s: &State, r: Rect, effects: bool) {
    terrain(&map(s.wall), s.direction, r, false);
    if !s.begun {
        preview(&area(s.wall), s.direction, r, s.clock);
    }
    terrain(&map(s.wall), s.direction, r, true);
    if effects {
        art.ground(s, r);
    }
    art.mine(s, r);
    art.bodies(s, r, effects, clip);
    if effects {
        art.burst(s, r, clip);
    }
    art.hud(s, r);
}
pub(crate) async fn run() {
    let art = Art::load();
    let clip = Clip::load_maps(&MAPS, PLAYER);
    let mut s = State::new(0, false, 2);
    loop {
        s.clock = get_time() as f32;
        clear_background(INK);
        if is_key_pressed(KeyCode::Escape) {
            return;
        }
        if is_key_pressed(KeyCode::Key1) {
            s = State::new(0, s.wall, s.direction);
        }
        if is_key_pressed(KeyCode::Key2) {
            s = State::new(1, s.wall, s.direction);
        }
        if is_key_pressed(KeyCode::Key3) {
            s = State::new(2, s.wall, s.direction);
        }
        if is_key_pressed(KeyCode::R) {
            s = State::new(s.kind, s.wall, s.direction);
        }
        if is_key_pressed(KeyCode::W) {
            s = State::new(s.kind, !s.wall, s.direction);
        }
        if is_key_pressed(KeyCode::D) {
            s = State::new(s.kind, s.wall, (s.direction + 1) % 4);
        }
        if is_key_pressed(KeyCode::Space) {
            if !s.begun {
                s.begun = true;
            } else {
                s.advance();
            }
        }
        draw_text_bold("Résidu persistant · revue", 32.0, 36.0, 24.0, TEXT);
        draw_text(
            "1 Feu · 2 Poison · 3 Froid · Espace action · R recommencer · W cloison · D direction",
            32.0,
            61.0,
            15.0,
            DIM,
        );
        scene(&art, &clip, &s, Rect::new(32.0, 88.0, SIZE, SIZE), true);
        next_frame().await;
    }
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    let correcting = std::env::args().any(|arg| arg == "--corriger-zones-carrees");
    if output.exists() && !correcting {
        return Err("Le dossier de revue existe déjà.".into());
    }
    if output.exists() && correcting {
        let expected = std::env::current_dir()
            .map_err(|e| e.to_string())?
            .join("artifacts/refonte-agent-2026-10-06/skill-animation-review-65/native");
        if output.canonicalize().map_err(|e| e.to_string())?
            != expected.canonicalize().map_err(|e| e.to_string())?
        {
            return Err("La correction doit rester dans l’aperçu de résidu courant.".into());
        }
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(1280.0, 720.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let clip = Clip::load_maps(&MAPS, PLAYER);
    let target = render_target(640, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 640.0, 720.0))
    };
    let actions = actions();
    let mut entries = Vec::new();
    for kind in 0..3 {
        if std::env::args().any(|arg| arg == "--residu-froid-only") && kind != 2 {
            continue;
        }
        for (wall, direction) in [(false, 2), (true, 2)] {
            if std::env::args().any(|arg| arg == "--residu-froid-cloison") && (kind != 2 || !wall) {
                continue;
            }
            let mut s = State::new(kind, wall, direction);
            let folder = s.variant();
            let dir = output.join(&folder);
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let mut timeline = Vec::new();
            let mut idle_states = Vec::new();
            let control_frames = [14, 44, 116, 132, 164, 172, 448, 464];
            for frame in 0..FRAMES {
                s.clock = frame as f32 / 24.0;
                if frame == 17 {
                    s.begun = true;
                }
                if actions.contains(&frame) {
                    s.advance();
                    if s.remaining > 0 {
                        idle_states.push(s.clone());
                    }
                }
                timeline.push(serde_json::json!({"frame":frame,"step":s.step,"turn":usize::from(s.begun)+s.step,"begun":s.begun,"zone_remaining":s.remaining,"zone_applications":s.applications,"hp":s.hp,"status":s.status,"zone_gel":s.zone_gel,"exposures":s.exposures,"poison_ticks":s.poison_ticks,"last_damage":s.last_damage,"entrant_inside":s.entrant_inside,"explosions":usize::from(s.blast.is_some()),"player_hp":100,"energy":if s.begun {90}else{100},"preview_visible":!s.begun}));
                for effects in [true, false]
                    .into_iter()
                    .take(if control_frames.contains(&frame) {
                        2
                    } else {
                        1
                    })
                {
                    set_camera(&camera);
                    clear_background(INK);
                    draw_text_bold(
                        format!("73 · Résidu persistant · {}", TYPES[kind]),
                        32.0,
                        35.0,
                        22.0,
                        TEXT,
                    );
                    scene(&art, &clip, &s, Rect::new(32.0, 88.0, SIZE, SIZE), effects);
                    super::occult::export(
                        &target,
                        &dir.join(if effects {
                            format!("{frame:03}.rgba")
                        } else {
                            format!("control-{frame:03}.rgba")
                        }),
                    );
                    set_default_camera();
                }
                next_frame().await;
            }
            let mut idle_images = Vec::new();
            for mut idle in idle_states {
                let step = idle.step;
                let base = idle.clock + 1.0;
                for n in 0..24 {
                    idle.clock = base + n as f32 / 12.0;
                    set_camera(&camera);
                    clear_background(INK);
                    scene(&art, &clip, &idle, Rect::new(32.0, 88.0, SIZE, SIZE), true);
                    super::occult::export(&target, &dir.join(format!("idle-{step}-{n:03}.rgba")));
                    set_default_camera();
                    next_frame().await;
                }
                idle_images.push(serde_json::json!({"step":step,"frames":24,"hp":idle.hp,"zone_remaining":idle.remaining,"zone_applications":idle.applications}));
            }
            std::fs::write(
                dir.join("timeline.json"),
                serde_json::to_vec_pretty(&timeline).unwrap(),
            )
            .map_err(|e| e.to_string())?;
            entries.push(serde_json::json!({"folder":folder,"variant":folder,"id":format!("SM05-{}",TYPES[kind]),"source_skill":"SM05","number":73,"title":NAMES[kind],"type":TYPES[kind],"case":if wall {"CLOISON"}else{"HAUT"},"direction":direction,"action_frames":actions,"control_frames":control_frames,"idle_images":idle_images,"fps":24,"frames":FRAMES,"tile":64,"area":area(wall).iter().map(|p|[p.x,p.y]).collect::<Vec<_>>(),"map":MAPS[usize::from(wall)]}));
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
