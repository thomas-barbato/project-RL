//! ANIM-S31-REV02: revised explosions, original LED and status fixtures, independent prepared fixtures.
use super::super::melee::actor;
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;

const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 6);
const MINE: GridPos = GridPos::new(4, 5);
const IDS: [&str; 2] = ["SM02", "SM03"];
const TITLES: [&str; 2] = ["Mines de poison", "Mines de glace"];
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#########",
];
const FRAMES: usize = 256;
const ACTIONS: [usize; 8] = [64, 88, 112, 136, 160, 184, 208, 232];
#[derive(Clone, Copy, PartialEq, Eq)]
enum Case {
    Open,
    Quiet,
    Wall,
    Rank12,
    AtArming,
    Escape,
    NoGel,
}
impl Case {
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Armement, explosion et statut",
            Self::Quiet => "Attente sans passage de tour",
            Self::Wall => "Explosion contre une cloison",
            Self::Rank12 => "Rang effectif 12",
            Self::AtArming => "Ennemi arrivé avant l’armement",
            Self::Escape => "La victime essaie de s’éloigner",
            Self::NoGel => "Une cible résiste au Gel (exemple)",
        }
    }
}
#[derive(Clone)]
struct Mine {
    at: GridPos,
    armed: bool,
    serial: u16,
}
#[derive(Clone)]
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    begun: Option<f32>,
    step: u16,
    turn: u16,
    energy: u16,
    cooldown: u16,
    mines: Vec<Mine>,
    primary: GridPos,
    from: GridPos,
    moved: Option<f32>,
    blast: Option<f32>,
    hp: [u16; 3],
    hits: [bool; 3],
    status: [u16; 3],
    ticks: [u16; 3],
    last_damage: [u16; 3],
    hit_at: [Option<f32>; 3],
    explosions: u16,
    blocked_walk: bool,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.0,
            begun: None,
            step: 0,
            turn: 0,
            energy: 100,
            cooldown: 0,
            mines: vec![],
            primary: if case == Case::AtArming {
                GridPos::new(4, 4)
            } else {
                GridPos::new(4, 3)
            },
            from: GridPos::new(4, 3),
            moved: None,
            blast: None,
            hp: [400; 3],
            hits: [false; 3],
            status: [0; 3],
            ticks: [0; 3],
            last_damage: [0; 3],
            hit_at: [None; 3],
            explosions: 0,
            blocked_walk: false,
        }
    }
    fn capacity(&self) -> usize {
        1
    }
    fn rank(&self) -> u16 {
        if self.case == Case::Rank12 { 12 } else { 11 }
    }
    fn max_steps(&self) -> u16 {
        match self.case {
            Case::Quiet => 0,
            Case::AtArming => 6,
            _ => 8,
        }
    }
    fn map_index(&self) -> usize {
        usize::from(self.case == Case::Wall)
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn variant(&self) -> String {
        format!("{}-A02", IDS[self.skill])
    }
    fn color(&self) -> Color {
        if self.skill == 0 {
            Color::new(0.38, 0.94, 0.12, 1.0)
        } else {
            Color::new(0.28, 0.78, 1.0, 1.0)
        }
    }
    fn explosion_damage(&self) -> u16 {
        if self.skill == 0 {
            if self.rank() == 12 { 20 } else { 19 }
        } else if self.rank() == 12 {
            17
        } else {
            16
        }
    }
    fn poison_damage(&self) -> u16 {
        if self.rank() == 12 { 47 } else { 42 }
    }
    fn type_name(&self) -> &'static str {
        if self.skill == 0 { "poison" } else { "froid" }
    }
    fn status_name(&self) -> &'static str {
        if self.skill == 0 {
            "Empoisonné"
        } else {
            "Gel"
        }
    }
    fn targets(&self) -> [GridPos; 3] {
        [
            self.primary,
            GridPos::new(3, 5),
            GridPos::new(if self.case == Case::Wall { 6 } else { 5 }, 5),
        ]
    }
    fn area(&self) -> Vec<GridPos> {
        [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(x, y)| GridPos::new(MINE.x + x, MINE.y + y))
            .filter(|&p| self.map().is_walkable(p))
            .collect()
    }
    fn led_on(&self) -> bool {
        (self.clock * 24.0).round() as u32 % 18 < 7
    }
    fn begin(&mut self) {
        if self.begun.is_some() {
            return;
        }
        self.begun = Some(self.clock);
        self.turn = 1;
        self.energy -= 10;
        self.cooldown = 1;
        self.mines.push(Mine {
            at: MINE,
            armed: false,
            serial: 1,
        });
    }
    fn move_primary(&mut self, at: GridPos) {
        self.from = self.primary;
        self.primary = at;
        self.moved = Some(self.clock);
    }
    fn damage(&mut self, i: usize, n: u16) {
        self.hp[i] -= n;
        self.last_damage[i] = n;
        self.hit_at[i] = Some(self.clock);
    }
    fn detonate(&mut self) {
        if let Some(index) = self
            .mines
            .iter()
            .position(|m| m.at == self.primary && m.armed)
        {
            self.mines.remove(index);
            self.explosions += 1;
            self.blast = Some(self.clock);
            let area = self.area();
            for (i, at) in self.targets().into_iter().enumerate() {
                self.hits[i] = area.contains(&at);
                if !self.hits[i] {
                    continue;
                }
                let apply = self.skill == 0 || self.case != Case::NoGel || i != 0;
                self.status[i] = if apply { 5 } else { 0 };
                let mut n = self.explosion_damage();
                if self.skill == 0 {
                    n += self.poison_damage();
                    self.ticks[i] = 1;
                }
                self.damage(i, n);
            }
        }
    }
    fn tick(&mut self) {
        if self.case == Case::AtArming
            && self.step == 0
            && self.begun.is_some_and(|t| self.clock - t >= 0.18)
            && self.primary != MINE
        {
            self.move_primary(MINE);
        }
    }
    fn advance(&mut self) {
        if self.begun.is_none() || self.step >= self.max_steps() {
            return;
        }
        self.step += 1;
        self.turn += 1;
        self.cooldown = 0;
        if self.explosions > 0 {
            if self.case == Case::Escape && self.step == 4 {
                if self.skill == 0 {
                    self.move_primary(GridPos::new(3, 4));
                } else if self.status[0] > 0 {
                    self.blocked_walk = true;
                }
            }
            for i in 0..3 {
                self.status[i] = self.status[i].saturating_sub(1);
                if self.skill == 0 && self.status[i] > 0 && self.ticks[i] < 5 {
                    self.ticks[i] += 1;
                    self.damage(i, self.poison_damage());
                }
            }
            if self.status[0] == 0 {
                self.blocked_walk = false;
            }
            return;
        }
        match self.step {
            1 => {
                for m in &mut self.mines {
                    m.armed = true;
                }
                self.detonate();
            }
            2 => self.move_primary(GridPos::new(4, 4)),
            3 => {
                self.move_primary(MINE);
                self.detonate();
            }
            _ => (),
        }
    }
}
fn point(at: GridPos, direction: usize, r: Rect) -> Vec2 {
    let p = position(at, direction);
    vec2(
        r.x + (p.x as f32 + 0.5) * 64.0,
        r.y + (p.y as f32 + 0.5) * 64.0,
    )
}
struct Art {
    mine: Texture2D,
    lens: Material,
    bursts: [Texture2D; 2],
    status_poison: Texture2D,
    gel: Texture2D,
    icons: [Texture2D; 2],
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
        let lens=load_material(ShaderSource::Glsl{vertex:r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;varying lowp vec4 tint;varying highp vec2 uv;
void main(){uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#,
fragment:r#"#version 100
precision highp float;varying lowp vec4 tint;varying highp vec2 uv;uniform sampler2D Texture;uniform vec3 lensTint;
void main(){vec4 c=texture2D(Texture,uv)*tint;
if(uv.x>0.46&&uv.x<0.69&&uv.y>0.345&&uv.y<0.46&&c.r>c.g*1.45&&c.r>c.b*1.45){c.rgb=lensTint*c.r;}
gl_FragColor=c;}"#},MaterialParams{uniforms:vec![UniformDesc::new("lensTint",UniformType::Float3)],pipeline_params:PipelineParams{color_blend:Some(BlendState::new(Equation::Add,BlendFactor::Value(BlendValue::SourceAlpha),BlendFactor::OneMinusValue(BlendValue::SourceAlpha))),..Default::default()},..Default::default()}).expect("mine lens material");
        Self {
            mine: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-60/mine-sprite.png"
            )),
            lens,
            bursts: [
                load(include_bytes!(
                    "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-62/poison-burst-atlas.png"
                )),
                load(include_bytes!(
                    "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-62/frost-burst-atlas.png"
                )),
            ],
            status_poison: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-61/poison-burst-atlas.png"
            )),
            gel: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-28/gel-atlas.png"
            )),
            icons: [
                load(include_bytes!(
                    "../../../assets/skills/agent/SM02-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/SM03-compact.png"
                )),
            ],
        }
    }
    fn mine(&self, m: &Mine, s: &State, r: Rect) {
        let p = point(m.at, s.direction, r);
        draw_ellipse(p.x, p.y + 29.0, 22.0, 4.0, 0.0, alpha(BLACK, 0.48));
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
        if s.led_on() {
            let a = if m.armed { 1.0 } else { 0.76 };
            draw_ellipse(p.x, p.y + 13.25, 5.2, 2.3, 0.0, alpha(c, a));
            draw_ellipse(
                p.x - 1.0,
                p.y + 12.6,
                2.2,
                0.9,
                0.0,
                alpha(Color::new(0.85, 1.0, 0.87, 1.0), a),
            );
        }
    }
    fn effect(&self, s: &State, r: Rect) {
        let duration = if s.skill == 0 { 0.46 } else { 0.40 };
        let Some(a) = s
            .blast
            .map(|t| s.clock - t)
            .filter(|a| (0.0..duration).contains(a))
        else {
            return;
        };
        let p = point(MINE, s.direction, r);
        let f = ((a / duration * 6.0) as usize).min(5);
        let size = if s.skill == 0 { 180.0 } else { 176.0 };
        draw_texture_ex(
            &self.bursts[s.skill],
            p.x - size / 2.0,
            p.y - size / 2.0,
            alpha(WHITE, 0.82),
            DrawTextureParams {
                source: Some(Rect::new(f as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    }
    fn actor_point(&self, s: &State, i: usize, r: Rect, effects: bool) -> Vec2 {
        let mut p = point(s.targets()[i], s.direction, r);
        if i == 0 {
            if let Some(t) = s.moved {
                p = point(s.from, s.direction, r).lerp(p, ((s.clock - t) / 0.12).clamp(0.0, 1.0));
            }
        }
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
        p
    }
    fn poison(&self, at: Vec2, clock: f32, phase: usize) {
        // Sparse attached vapour, not a persistent terrain cloud. Same painted toxic material.
        let f = 4 + ((clock * 2.0 + phase as f32) as usize % 2);
        for (dx, offset) in [(-11.0, 0.0), (12.0, 1.1)] {
            draw_texture_ex(
                &self.status_poison,
                at.x + dx - 24.0,
                at.y - 16.0 - offset * 3.0,
                alpha(WHITE, 0.52),
                DrawTextureParams {
                    source: Some(Rect::new(f as f32 * 256.0, 0.0, 256.0, 256.0)),
                    dest_size: Some(vec2(48.0, 48.0)),
                    ..Default::default()
                },
            );
        }
    }
    fn gel(&self, at: Vec2, clock: f32, phase: usize) {
        let t = clock * 3.0 + phase as f32 * 2.0;
        let f = t.floor() as usize % 6;
        let blend = t.fract();
        for (f, w) in [(f, 1.0 - blend), ((f + 1) % 6, blend)] {
            draw_texture_ex(
                &self.gel,
                at.x - 36.0,
                at.y - 18.0,
                alpha(WHITE, 0.70 * w),
                DrawTextureParams {
                    source: Some(Rect::new(f as f32 * 256.0, 64.0, 256.0, 192.0)),
                    dest_size: Some(vec2(72.0, 54.0)),
                    ..Default::default()
                },
            );
        }
    }
    fn bodies(&self, s: &State, r: Rect, effects: bool) {
        for i in 0..3 {
            let p = self.actor_point(s, i, r, effects);
            if effects && s.skill == 0 && s.status[i] > 0 {
                self.poison(p, s.clock, i);
            }
            actor(
                p,
                Visual::Bestiary {
                    number: 12 + i as u8,
                    corpse: false,
                },
            );
            if effects && s.skill == 1 && s.status[i] > 0 {
                self.gel(p, s.clock, i);
            }
        }
        actor(
            point(PLAYER, s.direction, r),
            game_art::player(Some("core:technomancien"), PlayerWeaponPose::Unarmed, None),
        );
    }
    fn hud(&self, s: &State, r: Rect, hover: bool) {
        draw_rectangle(r.x + 10.0, r.y + 10.0, 340.0, 88.0, alpha(INK, 0.96));
        draw_texture_ex(
            &self.icons[s.skill],
            r.x + 18.0,
            r.y + 24.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                ..Default::default()
            },
        );
        draw_text(
            format!("Passif · rang {}", s.rank()),
            r.x + 60.0,
            r.y + 32.0,
            14.0,
            Color::new(0.38, 0.77, 1.0, 1.0),
        );
        draw_text(
            format!(
                "Mine de {} · LED {}",
                s.type_name(),
                if s.skill == 0 { "verte" } else { "bleu glacé" }
            ),
            r.x + 60.0,
            r.y + 54.0,
            16.0,
            TEXT,
        );
        draw_text(
            format!(
                "Impact {} · {} : 5 tours",
                s.explosion_damage(),
                s.status_name()
            ),
            r.x + 60.0,
            r.y + 74.0,
            13.0,
            DIM,
        );
        draw_text(
            if s.skill == 0 {
                format!(
                    "Poison : {} par application (5 au total)",
                    s.poison_damage()
                )
            } else {
                format!(
                    "Chance de Gel : {} (succès préparés)",
                    if s.rank() == 12 { "73,8 %" } else { "69,2 %" }
                )
            },
            r.x + 18.0,
            r.y + 92.0,
            12.0,
            DIM,
        );
        if let Some(t) = s.begun {
            if s.clock - t < 0.4 {
                let p = point(PLAYER, s.direction, r);
                draw_text_bold(
                    "Pose de mine",
                    p.x - 42.0,
                    p.y - 40.0 - (s.clock - t) * 8.0,
                    15.0,
                    alpha(TEXT, 1.0 - (s.clock - t) / 0.4),
                );
            }
        }
        if hover {
            draw_rectangle(r.x + 286.0, r.y + 110.0, 274.0, 106.0, alpha(INK, 0.97));
            draw_text_bold(
                format!("Mine de {}", s.type_name()),
                r.x + 298.0,
                r.y + 132.0,
                17.0,
                TEXT,
            );
            draw_text(
                if s.mines.iter().any(|m| m.armed) {
                    "Armée"
                } else {
                    "Armement : 1 tour"
                },
                r.x + 298.0,
                r.y + 156.0,
                14.0,
                TEXT,
            );
            draw_text(
                format!(
                    "{} dégâts de {} · rayon 1 case",
                    s.explosion_damage(),
                    s.type_name()
                ),
                r.x + 298.0,
                r.y + 180.0,
                12.0,
                TEXT,
            );
            draw_text(
                "Statut sur cible · sans résidu au sol",
                r.x + 298.0,
                r.y + 204.0,
                12.0,
                DIM,
            );
        }
        for (i, at) in s.targets().into_iter().enumerate() {
            if s.status[i] > 0 {
                let p = self.actor_point(s, i, r, true);
                draw_texture_ex(
                    &self.icons[s.skill],
                    p.x + 17.0,
                    p.y - 31.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(14.0, 14.0)),
                        ..Default::default()
                    },
                );
                draw_text(
                    s.status[i].to_string(),
                    p.x + 21.0,
                    p.y - 9.0,
                    12.0,
                    s.color(),
                );
            }
            if let Some(a) = s.hit_at[i]
                .map(|t| s.clock - t)
                .filter(|a| (0.0..0.65).contains(a))
            {
                let p = point(at, s.direction, r);
                let y = p.y - 42.0 - a * 16.0;
                let c = alpha(s.color(), 1.0 - a / 0.65);
                let n = s.last_damage[i];
                draw_text_bold(n.to_string(), p.x - 12.0, y, 21.0, c);
                draw_text("(", p.x - 22.0, y + 14.0, 12.0, c);
                draw_texture_ex(
                    &self.icons[s.skill],
                    p.x - 17.0,
                    y + 3.0,
                    alpha(WHITE, 1.0 - a / 0.65),
                    DrawTextureParams {
                        dest_size: Some(vec2(12.0, 12.0)),
                        ..Default::default()
                    },
                );
                draw_text(format!("{n})"), p.x - 1.0, y + 14.0, 12.0, c);
            }
        }
        draw_rectangle(r.x + 5.0, r.y + SIZE - 62.0, 566.0, 57.0, alpha(INK, 0.96));
        draw_text(
            format!(
                "Tour {} · énergie {} · recharge {} · explosions {}",
                s.turn, s.energy, s.cooldown, s.explosions
            ),
            r.x + 12.0,
            r.y + SIZE - 45.0,
            14.0,
            TEXT,
        );
        draw_text(
            if s.explosions > 0 && s.status.iter().all(|n| *n == 0) {
                "Mine consommée · statuts terminés"
            } else if s.blocked_walk {
                "Gel : déplacement impossible ; les autres actions restent possibles."
            } else if s.begun.is_none() {
                "Confirmez la pose · le voyant central clignote dès la pose"
            } else if s.mines.iter().any(|m| !m.armed) {
                "Mine posée · armement au passage du tour"
            } else if !s.mines.is_empty() {
                "Mine armée · prête au contact d’un ennemi"
            } else if s.skill == 0 {
                "Empoisonné suit la victime · aucun nuage de terrain (SM05 absent)"
            } else {
                "Gel attaché aux victimes · aucun résidu au sol (SM05 absent)"
            },
            r.x + 12.0,
            r.y + SIZE - 26.0,
            11.0,
            DIM,
        );
        draw_text(
            format!(
                "PV ennemis {}/{}/{} · statuts {}/{}/{} · joueur 100/100",
                s.hp[0], s.hp[1], s.hp[2], s.status[0], s.status[1], s.status[2]
            ),
            r.x + 12.0,
            r.y + SIZE - 9.0,
            12.0,
            TEXT,
        );
    }
}
fn scene(art: &Art, clip: &Clip, s: &State, r: Rect, effects: bool, hover: bool) {
    terrain(&s.map(), s.direction, r, false);
    if s.begun.is_none() || hover && !s.mines.is_empty() {
        preview(&s.area(), s.direction, r, s.clock);
    }
    terrain(&s.map(), s.direction, r, true);
    for m in &s.mines {
        art.mine(m, s, r);
    }
    clip.apply_for(s.map_index(), s.direction, r);
    if effects {
        art.effect(s, r);
    }
    art.bodies(s, r, effects);
    gl_use_default_material();
    art.hud(s, r, hover);
}
pub(crate) async fn run() {
    request_new_screen_size(1280.0, 820.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let clip = Clip::load_maps(&MAPS, PLAYER);
    let mut s = State::new(0, Case::Open, 2);
    let mut paused = false;
    let cases = [
        Case::Open,
        Case::Quiet,
        Case::Wall,
        Case::Escape,
        Case::Rank12,
        Case::AtArming,
        Case::NoGel,
    ];
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Mines élémentaires · révision 2", 20.0, 33.0, 24.0, TEXT);
        for (i, t) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 80.0 + i as f32 * 40.0, 370.0, 34.0),
                t,
                true,
            ) {
                s = State::new(i, s.case, s.direction);
            }
        }
        for (i, t) in ["DROITE", "GAUCHE", "HAUT", "BAS"].iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 174.0,
                    179.0 + (i / 2) as f32 * 36.0,
                    166.0,
                    30.0,
                ),
                t,
                true,
            ) {
                s = State::new(s.skill, s.case, i);
            }
        }
        for (i, c) in cases.iter().enumerate() {
            if button(
                Rect::new(20.0, 270.0 + i as f32 * 35.0, 370.0, 30.0),
                c.title(),
                true,
            ) {
                s = State::new(s.skill, *c, s.direction);
            }
        }
        if button(
            Rect::new(20.0, 535.0, 350.0, 34.0),
            "Poser la mine",
            s.begun.is_none(),
        ) || is_key_pressed(KeyCode::Enter)
        {
            s.begin();
        }
        if button(
            Rect::new(20.0, 577.0, 350.0, 34.0),
            "Action suivante",
            s.begun.is_some() && s.step < s.max_steps(),
        ) || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(20.0, 619.0, 160.0, 34.0), "Recommencer", true)
            || is_key_pressed(KeyCode::R)
        {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(190.0, 619.0, 160.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
            s.tick();
        }
        let r = Rect::new(410.0, 88.0, SIZE, SIZE);
        let p = point(MINE, s.direction, r);
        let (x, y) = mouse_position();
        let hover = s.mines.iter().any(|m| m.at == MINE)
            && (x - p.x).abs() < 32.0
            && (y - p.y).abs() < 32.0;
        scene(&art, &clip, &s, r, true, hover);
        draw_text(
            "Scènes préparées · survolez la mine pour voir son rayon",
            410.0,
            704.0,
            13.0,
            DIM,
        );
        next_frame().await;
    }
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    if output.exists() {
        return Err("Le dossier de revue existe déjà.".into());
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
    let controls = [
        14, 17, 44, 54, 60, 64, 66, 68, 72, 76, 80, 84, 88, 108, 112, 114, 116, 120, 124, 128, 132,
        136, 138, 144, 148, 158, 160, 168, 184, 192, 208, 216, 232, 240, 254,
    ];
    let hover_frames = [44, 54, 66, 76, 96, 104];
    let mut entries = Vec::new();
    for skill in 0..2 {
        for (case, direction, label) in [
            (Case::Open, 2, "HAUT"),
            (Case::Open, 0, "DROITE"),
            (Case::Open, 1, "GAUCHE"),
            (Case::Open, 3, "BAS"),
            (Case::Quiet, 2, "ATTENTE"),
            (Case::Wall, 2, "CLOISON"),
            (Case::Escape, 2, "ELOIGNEMENT"),
            (Case::Rank12, 2, "RANG-12"),
            (Case::AtArming, 2, "ENNEMI-ARMEMENT"),
            (Case::NoGel, 2, "GEL-RESISTE"),
        ] {
            let mut s = State::new(skill, case, direction);
            let folder = format!("{}-{label}", s.variant());
            let dir = output.join(&folder);
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let mut timeline = Vec::new();
            let mut hovered = Vec::new();
            for frame in 0..FRAMES {
                s.clock = frame as f32 / 24.0;
                if frame == 17 {
                    s.begin();
                }
                s.tick();
                if ACTIONS.contains(&frame) {
                    s.advance();
                }
                timeline.push(serde_json::json!({"frame":frame,"begun":s.begun.is_some(),"step":s.step,"turn":s.turn,"energy":s.energy,"cooldown":s.cooldown,"rank":s.rank(),"capacity":s.capacity(),"count":s.mines.len(),"mines":s.mines.iter().map(|m|serde_json::json!({"at":[m.at.x,m.at.y],"armed":m.armed,"serial":m.serial})).collect::<Vec<_>>(),"led_on":s.led_on(),"primary":[s.primary.x,s.primary.y],"hp":s.hp,"hits":s.hits,"status":s.status,"status_name":s.status_name(),"poison_ticks":s.ticks,"explosion_damage":s.explosion_damage(),"poison_damage":s.poison_damage(),"last_damage":s.last_damage,"blocked_walk":s.blocked_walk,"explosions":s.explosions,"player_hp":100,"preview_visible":s.begun.is_none()}));
                let variants = 1
                    + usize::from(controls.contains(&frame))
                    + usize::from(
                        hover_frames.contains(&frame) && s.mines.iter().any(|m| m.at == MINE),
                    );
                for v in 0..variants {
                    let control = v == 1 && controls.contains(&frame);
                    let hover = v > 0 && !control;
                    set_camera(&camera);
                    clear_background(INK);
                    draw_text_bold(
                        format!("{} · {}", 70 + skill, TITLES[skill]),
                        32.0,
                        35.0,
                        22.0,
                        TEXT,
                    );
                    draw_text(
                        format!("{} · {label} · étape {}", s.variant(), s.step),
                        32.0,
                        60.0,
                        14.0,
                        DIM,
                    );
                    scene(
                        &art,
                        &clip,
                        &s,
                        Rect::new(32.0, 88.0, SIZE, SIZE),
                        !control,
                        hover,
                    );
                    draw_text(
                        "Explosion du type choisi · statuts attachés aux victimes",
                        32.0,
                        684.0,
                        13.0,
                        TEXT,
                    );
                    draw_text(
                        "SM05 absent : pas de nuage de terrain · tours explicites",
                        32.0,
                        707.0,
                        12.0,
                        DIM,
                    );
                    let name = if control {
                        format!("control-{frame:03}.rgba")
                    } else if hover {
                        hovered.push(frame);
                        format!("hover-{frame:03}.rgba")
                    } else {
                        format!("{frame:03}.rgba")
                    };
                    super::occult::export(&target, &dir.join(name));
                }
                set_default_camera();
                next_frame().await;
            }
            // The user asked to revise explosions only. Reuse exact native status-idle drawings.
            let old_folder = format!("{}-A01-{label}", IDS[skill]);
            let old_manifest:Vec<serde_json::Value>=serde_json::from_slice(&std::fs::read("artifacts/refonte-agent-2026-10-06/skill-animation-review-61/native/capture.json").map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
            let old = old_manifest
                .iter()
                .find(|e| e["folder"].as_str() == Some(old_folder.as_str()))
                .ok_or("Ancienne scène de statut absente")?;
            let idle_images = old["idle_images"].clone();
            std::fs::write(
                dir.join("timeline.json"),
                serde_json::to_vec_pretty(&timeline).unwrap(),
            )
            .map_err(|e| e.to_string())?;
            let mut rows = vec![vec!['.'; 9]; 9];
            for y in 0..9 {
                for x in 0..9 {
                    if s.map().blocks_vision(GridPos::new(x, y)) {
                        let p = position(GridPos::new(x, y), direction);
                        rows[p.y as usize][p.x as usize] = '#';
                    }
                }
            }
            let p = position(MINE, direction);
            entries.push(serde_json::json!({"folder":folder,"variant":format!("{}-{label}",s.variant()),"id":IDS[skill],"number":70+skill,"title":TITLES[skill],"case":label,"case_title":case.title(),"direction":direction,"permitted":true,"quiet":case==Case::Quiet,"control_frames":controls.as_slice(),"hover_frames":hovered,"idle_images":idle_images,"idle_source":format!("../skill-animation-review-61/native/{old_folder}"),"mine_cell":[p.x,p.y],"action_frames":ACTIONS[..s.max_steps() as usize].to_vec(),"tile":64,"frames":FRAMES,"fps":24,"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n")}));
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
