//! ANIM-S22: prepared charge and reflection feedback; never a campaign resolver.
use super::super::melee::{cut, player_visual};
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;
const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 4);
const IDS: [&str; 2] = ["BM03", "BM05"];
const TITLES: [&str; 2] = ["Tempo offensif", "Sceau de vindicte"];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const FRAMES: usize = 192;
const ACTION_FRAMES: [usize; 6] = [48, 72, 96, 120, 144, 168];
const MAGIC: Color = Color::new(0.85, 0.22, 1.0, 1.0);
const MAPS: [&str; 3] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#..###..#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    Open,
    Quiet,
    Wall,
    Hidden,
    Ranged,
    Silence,
    Hover,
    Miss,
    Move,
    Other,
    Menu,
    Poison,
    Indirect,
    Melee,
    Transformed,
}
impl Case {
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Série et renvoi",
            Self::Quiet => "Attente sans action",
            Self::Wall => "Obstacle",
            Self::Hidden => "Cible cachée",
            Self::Ranged => "Arme à distance active",
            Self::Silence => "Silence",
            Self::Hover => "Survol du buff",
            Self::Miss => "Coup manqué",
            Self::Move => "Déplacement",
            Self::Other => "Autre action",
            Self::Menu => "Menu sans passage de tour",
            Self::Poison => "Poison exclu",
            Self::Indirect => "Dégâts indirects exclus",
            Self::Melee => "Coup de mêlée exclu",
            Self::Transformed => "Forme transformée",
        }
    }
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    begun: Option<f32>,
    event: Option<f32>,
    miss: Option<f32>,
    step: u16,
    turn: u16,
    remaining: u16,
    cooldown: u16,
    charges: u16,
    hits: u16,
    damage: u16,
    bonus: u16,
    returned: u16,
    player: GridPos,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.0,
            begun: None,
            event: None,
            miss: None,
            step: 0,
            turn: 0,
            remaining: 0,
            cooldown: 0,
            charges: 0,
            hits: 0,
            damage: 0,
            bonus: 0,
            returned: 0,
            player: PLAYER,
        }
    }
    fn map_index(&self) -> usize {
        if self.case == Case::Wall {
            if self.skill == 0 { 1 } else { 2 }
        } else {
            0
        }
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn target(&self) -> GridPos {
        GridPos::new(
            4,
            if self.skill == 0 || self.case == Case::Melee {
                3
            } else {
                1
            },
        )
    }
    fn allowed(&self) -> bool {
        if self.skill == 0 {
            self.case != Case::Ranged
        } else {
            !matches!(self.case, Case::Silence | Case::Transformed)
        }
    }
    fn begin(&mut self) {
        if self.begun.is_none() && self.allowed() {
            self.begun = Some(self.clock);
            if self.skill == 1 {
                self.remaining = 6;
                self.cooldown = parameter_for("BM05", "cooldown_turns", 11);
            }
        }
    }
    fn can_event(&self) -> bool {
        self.begun.is_some() && self.allowed() && !matches!(self.case, Case::Wall | Case::Hidden)
    }
    fn reflectable(&self) -> bool {
        self.remaining > 0 && !matches!(self.case, Case::Poison | Case::Indirect | Case::Melee)
    }
    fn action(&mut self) {
        if !self.can_event() {
            return;
        }
        if self.skill == 0 && self.step == 2 {
            match self.case {
                Case::Miss => {
                    self.charges = 0;
                    self.event = None;
                    self.miss = Some(self.clock);
                    return;
                }
                Case::Move => {
                    self.charges = 0;
                    self.event = None;
                    self.player = GridPos::new(3, 4);
                    return;
                }
                Case::Other => {
                    self.charges = 0;
                    self.event = None;
                    return;
                }
                Case::Menu => return,
                _ => {}
            }
        }
        if self.skill == 0 && self.step >= 3 && matches!(self.case, Case::Move | Case::Other) {
            return;
        }
        self.event = Some(self.clock);
        self.miss = None;
        self.hits += 1;
        if self.skill == 0 {
            self.bonus = self.charges * 5;
            self.damage = 20 + self.bonus;
            self.charges = (self.charges + 1).min(3);
        } else {
            self.damage = 20;
            self.bonus = 0;
            self.returned = if self.reflectable() { 8 } else { 0 };
        }
    }
    fn advance(&mut self) {
        if self.begun.is_none() {
            return;
        }
        self.step += 1;
        if self.skill == 0 {
            if self.step <= 3 && !(self.case == Case::Menu && self.step == 2) {
                self.turn += 1;
            }
        } else {
            self.turn += 1;
            self.remaining = self.remaining.saturating_sub(1);
            self.cooldown = self.cooldown.saturating_sub(1);
        }
    }
    fn wait(&mut self) {
        self.advance();
        if self.skill == 0 {
            self.charges = 0;
        }
    }
    fn age(&self) -> Option<f32> {
        self.event.map(|t| self.clock - t)
    }
    fn icon(&self) -> bool {
        if self.skill == 0 {
            self.charges > 0
        } else {
            self.remaining > 0
        }
    }
}
fn point(at: GridPos, direction: usize, rect: Rect) -> Vec2 {
    let p = position(at, direction);
    vec2(
        rect.x + (p.x as f32 + 0.5) * 64.0,
        rect.y + (p.y as f32 + 0.5) * 64.0,
    )
}
struct Art {
    icons: [Texture2D; 2],
    blood: Texture2D,
    bolt: Texture2D,
    predator: Texture2D,
}
impl Art {
    fn load() -> Self {
        let load = |p: &[u8]| {
            let t = Texture2D::from_file_with_format(p, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            icons: [
                load(include_bytes!(
                    "../../../assets/skills/agent/BM03-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/BM05-compact.png"
                )),
            ],
            blood: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-40/blood-atlas.png"
            )),
            bolt: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-32/bolt-atlas.png"
            )),
            predator: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-39/predator-64.png"
            )),
        }
    }
    fn bodies(&self, s: &State, rect: Rect, effects: bool) {
        let p = point(s.player, s.direction, rect);
        let t = point(s.target(), s.direction, rect);
        let age = s.age().unwrap_or(99.0);
        let received = if s.skill == 0 { age } else { age - 0.20 };
        let recoil = if effects {
            crate::combat_visuals::hit_reaction_offset(
                if s.skill == 0 { t } else { p },
                if s.skill == 0 { p } else { t },
                f64::from(received),
                64.0,
            )
        } else {
            Vec2::ZERO
        };
        if s.case != Case::Hidden && s.map().is_walkable(s.target()) && s.case != Case::Wall {
            let r = if s.skill == 0 {
                recoil
            } else if effects && s.returned > 0 {
                crate::combat_visuals::hit_reaction_offset(t, p, f64::from(age - 0.40), 64.0)
            } else {
                Vec2::ZERO
            };
            enemy(t + r);
        }
        let at = if s.skill == 1 {
            p + recoil
        } else if effects && (0.0..0.20).contains(&age) {
            p + (t - p).normalize() * (age / 0.20 * std::f32::consts::PI).sin() * 8.0
        } else {
            p
        };
        if s.case == Case::Transformed {
            draw_texture(&self.predator, at.x - 32.0, at.y - 32.0, WHITE);
        } else if s.case == Case::Ranged {
            actor(
                at,
                game_art::wear(
                    game_art::player(
                        Some("core:bastion"),
                        PlayerWeaponPose::Ranged,
                        Some("core:modele_cogneur_p01"),
                    ),
                    [("core:body_armor".into(), "core:tenue_assaut".into())],
                ),
            );
        } else {
            actor(at, player_visual());
        }
        enemy(point(GridPos::new(6, 6), s.direction, rect));
    }
    fn melee(&self, s: &State, rect: Rect) {
        let age = s
            .age()
            .or_else(|| s.miss.map(|t| s.clock - t))
            .unwrap_or(99.0);
        let t = point(s.target(), s.direction, rect);
        let d = (t - point(s.player, s.direction, rect)).normalize();
        let n = vec2(-d.y, d.x);
        if (0.0..0.20).contains(&age) {
            cut(
                t - n * 28.0 - d * 12.0,
                t + n * 30.0 + d * 10.0,
                7.0,
                1.0 - age / 0.20,
            );
        }
        if s.event.is_some() && (0.03..0.39).contains(&age) {
            let f = (((age - 0.03) / 0.36 * 6.0) as usize).min(5);
            draw_texture_ex(
                &self.blood,
                t.x - 31.0 + d.x * 18.0,
                t.y - 25.0 + d.y * 18.0,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(f as f32 * 256.0, 0.0, 256.0, 256.0)),
                    dest_size: Some(vec2(62.0, 62.0)),
                    rotation: d.y.atan2(d.x),
                    ..Default::default()
                },
            );
        }
    }
    fn shot(&self, from: Vec2, to: Vec2, age: f32) {
        let d = to - from;
        if !(0.0..0.32).contains(&age) {
            return;
        }
        let (f, at, anchor, size, a) = if age < 0.20 {
            (
                ((age / 0.20 * 3.0) as usize).min(2),
                from + d * (age / 0.20),
                vec2(232.0, 136.0),
                72.0,
                1.0,
            )
        } else {
            (
                3 + (((age - 0.20) / 0.12 * 3.0) as usize).min(2),
                to,
                vec2(128.0, 128.0),
                54.0,
                ((0.32 - age) / 0.05).clamp(0.0, 1.0),
            )
        };
        draw_texture_ex(
            &self.bolt,
            at.x - anchor.x * size / 256.0,
            at.y - anchor.y * size / 256.0,
            alpha(WHITE, a),
            DrawTextureParams {
                source: Some(Rect::new(f as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(vec2(size, size)),
                rotation: d.y.atan2(d.x),
                pivot: Some(at),
                ..Default::default()
            },
        );
    }
    fn effects(&self, s: &State, rect: Rect) {
        if s.skill == 0 {
            self.melee(s, rect);
            return;
        }
        let Some(age) = s.age() else {
            return;
        };
        let p = point(s.player, s.direction, rect) + vec2(0.0, -8.0);
        let t = point(s.target(), s.direction, rect) + vec2(0.0, -8.0);
        if s.case == Case::Melee {
            let d = (p - t).normalize();
            let n = vec2(-d.y, d.x);
            if (0.14..0.32).contains(&age) {
                cut(
                    p - n * 27.0 - d * 10.0,
                    p + n * 29.0 + d * 8.0,
                    7.0,
                    (0.32 - age) / 0.18,
                );
            }
            if (0.20..0.56).contains(&age) {
                let f = (((age - 0.20) / 0.36 * 6.0) as usize).min(5);
                draw_texture_ex(
                    &self.blood,
                    p.x - 31.0 + d.x * 18.0,
                    p.y - 25.0 + d.y * 18.0,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(f as f32 * 256.0, 0.0, 256.0, 256.0)),
                        dest_size: Some(vec2(62.0, 62.0)),
                        rotation: d.y.atan2(d.x),
                        ..Default::default()
                    },
                );
            }
        }
        if !matches!(s.case, Case::Poison | Case::Indirect | Case::Melee) {
            self.shot(t, p, age);
            if s.returned > 0 {
                self.shot(p, t, age - 0.20);
            }
        }
    }
    fn hud(&self, s: &State, rect: Rect, interactive: bool) {
        if !s.icon() {
            return;
        }
        let r = Rect::new(rect.x + 488.0, rect.y + 66.0, 38.0, 38.0);
        draw_rectangle(r.x, r.y, r.w, r.h, alpha(INK, 0.92));
        draw_rectangle_lines(r.x, r.y, r.w, r.h, 1.0, DIM);
        draw_texture_ex(
            &self.icons[s.skill],
            r.x + 3.0,
            r.y + 3.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                ..Default::default()
            },
        );
        draw_rectangle(r.x + 23.0, r.y + 23.0, 14.0, 14.0, INK);
        draw_text_bold(
            if s.skill == 0 { s.charges } else { s.remaining }.to_string(),
            r.x + 26.0,
            r.y + 35.0,
            13.0,
            TEXT,
        );
        if s.case == Case::Hover
            || (interactive && r.contains(vec2(mouse_position().0, mouse_position().1)))
        {
            let x = rect.x + 308.0;
            let y = rect.y + 114.0;
            draw_rectangle(x, y, 250.0, 106.0, alpha(INK, 0.98));
            draw_rectangle_lines(x, y, 250.0, 106.0, 1.0, DIM);
            draw_text_bold(TITLES[s.skill], x + 10.0, y + 22.0, 17.0, TEXT);
            if s.skill == 0 {
                draw_text(
                    format!("Charges : {} / 3", s.charges),
                    x + 10.0,
                    y + 45.0,
                    14.0,
                    TEXT,
                );
                draw_text(
                    format!("Bonus de dégâts : +{} %", s.charges * 25),
                    x + 10.0,
                    y + 68.0,
                    14.0,
                    TEXT,
                );
                draw_text(
                    "Actif tant que la série se poursuit.",
                    x + 10.0,
                    y + 91.0,
                    13.0,
                    DIM,
                );
            } else {
                draw_text(
                    "Renvoi magique direct : 40 %",
                    x + 10.0,
                    y + 45.0,
                    14.0,
                    TEXT,
                );
                draw_text(
                    format!("Durée restante : {} tours", s.remaining),
                    x + 10.0,
                    y + 68.0,
                    14.0,
                    TEXT,
                );
                draw_text(
                    "Poison et dégâts indirects exclus.",
                    x + 10.0,
                    y + 91.0,
                    13.0,
                    DIM,
                );
            }
        }
    }
}
fn feedback(s: &State, rect: Rect, effects: bool) {
    if s.skill == 1 {
        if let Some(age) = s.begun.map(|t| s.clock - t).filter(|a| *a < 0.29) {
            let p = point(s.player, s.direction, rect);
            draw_text_bold(
                TITLES[1],
                p.x - 70.0,
                p.y - 44.0,
                15.0,
                alpha(TEXT, 1.0 - age / 0.29),
            );
        }
    }
    if !effects {
        return;
    }
    let t = point(s.target(), s.direction, rect);
    let p = point(s.player, s.direction, rect);
    if s.skill == 0 {
        if let Some(age) = s.age().filter(|a| *a < 0.65) {
            draw_text_bold(
                format!("{} (+{})", s.damage, s.bonus),
                t.x - 40.0,
                t.y - 48.0 - age * 20.0,
                20.0,
                alpha(TEXT, 1.0 - age / 0.65),
            );
            let q = t + vec2(-17.0, -40.0 - age * 20.0);
            let a = 1.0 - age / 0.65;
            draw_line(
                q.x - 3.0,
                q.y + 3.0,
                q.x + 3.0,
                q.y - 3.0,
                2.2,
                alpha(TEXT, a),
            );
            draw_line(q.x - 4.0, q.y, q.x, q.y + 4.0, 1.6, alpha(TEXT, a));
            draw_text(
                s.damage.to_string(),
                q.x + 9.0,
                q.y + 5.0,
                13.0,
                alpha(TEXT, a),
            );
        }
        if let Some(age) = s.miss.map(|v| s.clock - v).filter(|a| *a < 0.50) {
            draw_text_bold(
                "Manqué",
                t.x - 27.0,
                t.y - 44.0,
                18.0,
                alpha(TEXT, 1.0 - age / 0.50),
            );
        }
    } else if let Some(age) = s.age() {
        for (at, start, amount) in [(p, 0.20, s.damage), (t, 0.40, s.returned)] {
            let a = age - start;
            if amount > 0 && (0.0..0.35).contains(&a) {
                let colour = alpha(
                    if s.case == Case::Poison {
                        GREEN
                    } else if s.case == Case::Melee {
                        TEXT
                    } else {
                        MAGIC
                    },
                    1.0 - a / 0.35,
                );
                let y = at.y - 47.0 - a * 20.0;
                draw_text_bold(
                    amount.to_string(),
                    at.x - if amount >= 10 { 40.0 } else { 29.0 },
                    y,
                    19.0,
                    colour,
                );
                draw_text("(", at.x - 12.0, y, 17.0, colour);
                let q = vec2(at.x + 1.0, y - 6.0);
                if s.case == Case::Melee {
                    draw_line(q.x - 3.0, q.y + 3.0, q.x + 3.0, q.y - 3.0, 2.2, colour);
                    draw_line(q.x - 4.0, q.y, q.x, q.y + 4.0, 1.6, colour);
                } else {
                    draw_triangle(
                        q + vec2(0.0, -5.0),
                        q + vec2(-3.0, 0.0),
                        q + vec2(3.0, 0.0),
                        colour,
                    );
                    draw_triangle(
                        q + vec2(0.0, 5.0),
                        q + vec2(-3.0, 0.0),
                        q + vec2(3.0, 0.0),
                        colour,
                    );
                }
                draw_text(format!("{})", amount), at.x + 9.0, y, 17.0, colour);
            }
        }
    }
}
fn scene(
    art: &Art,
    clip: &Clip,
    magic_clip: &Clip,
    s: &State,
    rect: Rect,
    effects: bool,
    interactive: bool,
) {
    let map = s.map();
    terrain(&map, s.direction, rect, false);
    if s.begun.is_none()
        && s.allowed()
        && (s.skill == 1 || !matches!(s.case, Case::Wall | Case::Hidden))
    {
        preview(
            &[if s.skill == 0 { s.target() } else { s.player }],
            s.direction,
            rect,
            s.clock,
        );
    }
    terrain(&map, s.direction, rect, true);
    clip.apply_for(s.map_index(), s.direction, rect);
    art.bodies(s, rect, effects);
    if effects && s.skill == 0 {
        art.effects(s, rect);
    }
    gl_use_default_material();
    if effects && s.skill == 1 {
        magic_clip.apply_for(s.map_index(), s.direction, rect);
        art.effects(s, rect);
        gl_use_default_material();
    }
    // HUD is identical in controls; compare only world VFX and cosmetic motion.
    feedback(s, rect, true);
    art.hud(s, rect, interactive);
    draw_rectangle(
        rect.x + 5.0,
        rect.y + SIZE - 43.0,
        556.0,
        38.0,
        alpha(INK, 0.95),
    );
    draw_text(
        if s.skill == 0 {
            format!(
                "Charges {} / 3 · bonus +{} % · frappes {}",
                s.charges,
                s.charges * 25,
                s.hits
            )
        } else {
            format!(
                "Sceau {} tours · recharge {} · renvoi {}",
                s.remaining, s.cooldown, s.returned
            )
        },
        rect.x + 12.0,
        rect.y + SIZE - 27.0,
        14.0,
        TEXT,
    );
    let log = if !s.allowed() {
        if s.case == Case::Ranged {
            "Arme de mêlée active requise pour cette série."
        } else if s.case == Case::Silence {
            "Silence empêche le lancement du sceau."
        } else {
            "Forme humaine requise pour lancer le sceau."
        }
    } else if s.skill == 0 && s.step == 2 {
        match s.case {
            Case::Miss => "Coup manqué : série interrompue.",
            Case::Move => "Déplacement : série interrompue.",
            Case::Other => "Autre action : série interrompue.",
            Case::Menu => "Menu sans tour : charges conservées.",
            _ => "Chaque charge profite aux frappes suivantes.",
        }
    } else if s.case == Case::Wall {
        "L'obstacle bloque le contact."
    } else if s.case == Case::Hidden {
        "Cible cachée : aucun contact préparé."
    } else if s.skill == 1 && s.hits > 0 {
        match s.case {
            Case::Poison => "Poison : aucun renvoi.",
            Case::Indirect => "Dégâts indirects : aucun renvoi.",
            Case::Melee => "Dégâts de mêlée : aucun renvoi.",
            _ => {
                if s.returned > 0 {
                    "20 dégâts reçus · 8 dégâts magiques renvoyés."
                } else {
                    "20 dégâts reçus · sceau expiré : aucun renvoi."
                }
            }
        }
    } else {
        "Valeurs de frappe préparées pour la revue visuelle."
    };
    draw_text(log, rect.x + 12.0, rect.y + SIZE - 10.0, 13.0, DIM);
}
pub(crate) async fn run() {
    request_new_screen_size(1280.0, 820.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let clip = Clip::load_maps(&MAPS, PLAYER);
    let magic = Clip::load_maps_with_violet(&MAPS, PLAYER, 3.0);
    let mut s = State::new(0, Case::Open, 2);
    let mut paused = false;
    let cases = [
        Case::Open,
        Case::Quiet,
        Case::Wall,
        Case::Hidden,
        Case::Ranged,
        Case::Silence,
        Case::Hover,
        Case::Miss,
        Case::Move,
        Case::Other,
        Case::Menu,
        Case::Poison,
        Case::Indirect,
        Case::Melee,
        Case::Transformed,
    ];
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Tempo et vindicte · ANIM-S22", 20.0, 33.0, 24.0, TEXT);
        for (i, t) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 88.0 + i as f32 * 42.0, 330.0, 34.0),
                t,
                true,
            ) {
                s = State::new(i, Case::Open, s.direction);
            }
        }
        for (i, t) in DIRS.iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 164.0,
                    185.0 + (i / 2) as f32 * 37.0,
                    156.0,
                    31.0,
                ),
                t,
                true,
            ) {
                s = State::new(s.skill, s.case, i);
            }
        }
        for (i, c) in cases.iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 184.0,
                    278.0 + (i / 2) as f32 * 33.0,
                    176.0,
                    28.0,
                ),
                c.title(),
                true,
            ) {
                s = State::new(s.skill, *c, s.direction);
            }
        }
        if button(
            Rect::new(20.0, 558.0, 330.0, 34.0),
            if s.skill == 0 {
                "Préparer la série (aperçu)"
            } else {
                "Lancer · 13 énergie"
            },
            s.begun.is_none() && s.allowed(),
        ) {
            s.begin();
        }
        if button(
            Rect::new(20.0, 600.0, 330.0, 34.0),
            if s.skill == 0 {
                "Frapper"
            } else {
                "Attaque ennemie préparée"
            },
            s.can_event() && s.age().is_none_or(|a| a > 0.80),
        ) || (is_key_pressed(KeyCode::A) && s.can_event())
        {
            if s.skill == 0 && s.hits > 0 {
                s.advance();
            }
            s.action();
        }
        if button(
            Rect::new(20.0, 642.0, 330.0, 34.0),
            if s.skill == 0 {
                "Attendre un tour (rompt la série)"
            } else {
                "+1 tour"
            },
            s.begun.is_some(),
        ) || is_key_pressed(KeyCode::Space)
        {
            s.wait();
        }
        if button(Rect::new(20.0, 684.0, 160.0, 32.0), "Recommencer", true)
            || is_key_pressed(KeyCode::R)
        {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(190.0, 684.0, 160.0, 32.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
        }
        scene(
            &art,
            &clip,
            &magic,
            &s,
            Rect::new(410.0, 88.0, SIZE, SIZE),
            true,
            true,
        );
        draw_text(
            "Cas préparés · aucune nouvelle statistique d'arme ou de monstre.",
            410.0,
            702.0,
            14.0,
            DIM,
        );
        draw_text(
            "Le projectile réutilise le violet franc accepté ; aucun sang magique.",
            410.0,
            726.0,
            14.0,
            DIM,
        );
        draw_text(
            "Monstres 32 px temporaires ; adaptation prévue après les animations.",
            410.0,
            750.0,
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
    let magic = Clip::load_maps_with_violet(&MAPS, PLAYER, 3.0);
    let target = render_target(640, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 640.0, 720.0))
    };
    let controls = [
        14, 17, 24, 26, 29, 31, 34, 38, 44, 48, 53, 58, 68, 72, 77, 92, 96, 110, 120, 144, 168,
        174, 180, 190,
    ];
    let mut entries = Vec::new();
    for skill in 0..2 {
        let mut cases = vec![
            (Case::Open, 2, "HAUT"),
            (Case::Open, 0, "DROITE"),
            (Case::Open, 1, "GAUCHE"),
            (Case::Open, 3, "BAS"),
            (Case::Quiet, 2, "ATTENTE"),
            (Case::Wall, 2, "CLOISON"),
            (Case::Hidden, 2, "CACHE"),
            (Case::Ranged, 2, "DISTANCE"),
            (Case::Silence, 2, "SILENCE"),
            (Case::Hover, 2, "SURVOL"),
        ];
        if skill == 0 {
            cases.extend([
                (Case::Miss, 2, "MANQUE"),
                (Case::Move, 2, "DEPLACEMENT"),
                (Case::Other, 2, "AUTRE-ACTION"),
                (Case::Menu, 2, "SANS-TOUR"),
            ]);
        } else {
            cases.extend([
                (Case::Poison, 2, "POISON"),
                (Case::Indirect, 2, "INDIRECT"),
                (Case::Melee, 2, "MELEE"),
                (Case::Transformed, 2, "TRANSFORME"),
            ]);
        }
        for (case, direction, label) in cases {
            let folder = format!("{}-{label}", IDS[skill]);
            let dir = output.join(&folder);
            std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
            let mut s = State::new(skill, case, direction);
            let permitted = s.allowed();
            let mut timeline = Vec::new();
            for frame in 0..FRAMES {
                s.clock = frame as f32 / 24.0;
                if frame == 17 {
                    s.begin();
                }
                if frame == 24 {
                    s.action();
                }
                if case != Case::Quiet && ACTION_FRAMES.contains(&frame) {
                    s.advance();
                    if skill == 1 || s.step <= 3 {
                        s.action();
                    }
                }
                timeline.push(serde_json::json!({"frame":frame,"begun":s.begun.is_some(),"step":s.step,"turn":s.turn,"charges":s.charges,"bonus_percent":s.charges*25,"remaining":s.remaining,"cooldown":s.cooldown,"hits":s.hits,"damage":s.damage,"bonus":s.bonus,"returned":s.returned,"event_age":s.age(),"icon_visible":s.icon(),"player":[s.player.x,s.player.y]}));
                for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                    set_camera(&camera);
                    clear_background(INK);
                    draw_text_bold(
                        format!("{} · {}", 53 + skill, TITLES[skill]),
                        32.0,
                        35.0,
                        23.0,
                        TEXT,
                    );
                    draw_text(
                        format!("{}-A01 · {} · étape {}", IDS[skill], label, s.step),
                        32.0,
                        60.0,
                        14.0,
                        DIM,
                    );
                    scene(
                        &art,
                        &clip,
                        &magic,
                        &s,
                        Rect::new(32.0, 88.0, SIZE, SIZE),
                        control == 0,
                        false,
                    );
                    draw_text(
                        if skill == 0 {
                            "Rang 11 · +25 % par charge · maximum 3 charges"
                        } else {
                            "Rang 11 · renvoi 40 % · durée 6 · énergie 13 · recharge 8"
                        },
                        32.0,
                        684.0,
                        14.0,
                        TEXT,
                    );
                    draw_text(
                        "Frappe 20 préparée ; valeurs illustratives. Monstres 32 px temporaires.",
                        32.0,
                        707.0,
                        12.0,
                        DIM,
                    );
                    super::occult::export(
                        &target,
                        &dir.join(if control == 0 {
                            format!("{frame:03}.rgba")
                        } else {
                            format!("control-{frame:03}.rgba")
                        }),
                    );
                }
                set_default_camera();
                next_frame().await;
            }
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
            entries.push(serde_json::json!({"folder":folder,"variant":format!("{}-A01-{label}",IDS[skill]),"id":IDS[skill],"number":53+skill,"title":TITLES[skill],"case":label,"case_title":case.title(),"direction":direction,"permitted":permitted,"quiet":case==Case::Quiet,"control_frames":controls,"action_frames":if case==Case::Quiet{vec![]}else{ACTION_FRAMES.to_vec()},"tile":64,"frames":FRAMES,"fps":24,"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n")}));
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
    #[test]
    fn charges_benefit_next_hit_and_stop_at_three() {
        let mut s = State::new(0, Case::Open, 2);
        s.begin();
        for expected in [20, 25, 30, 35, 35] {
            s.action();
            assert_eq!(s.damage, expected);
            assert!(s.charges <= 3);
        }
        s.clock = 100.0;
        assert_eq!(s.charges, 3);
        s.wait();
        assert_eq!(s.charges, 0);
    }
    #[test]
    fn rupture_and_no_time_menu_are_distinct() {
        for c in [Case::Miss, Case::Move, Case::Other, Case::Menu] {
            let mut s = State::new(0, c, 2);
            s.begin();
            s.action();
            s.advance();
            s.action();
            s.advance();
            s.action();
            assert_eq!(s.charges, if c == Case::Menu { 2 } else { 0 });
            assert_eq!(s.turn, if c == Case::Menu { 1 } else { 2 });
        }
    }
    #[test]
    fn reflection_exclusions_and_expiry_never_prevent_received_damage() {
        for c in [
            Case::Open,
            Case::Ranged,
            Case::Poison,
            Case::Indirect,
            Case::Melee,
        ] {
            let mut s = State::new(1, c, 2);
            s.begin();
            s.action();
            assert_eq!(s.damage, 20);
            assert_eq!(
                s.returned,
                if matches!(c, Case::Open | Case::Ranged) {
                    8
                } else {
                    0
                }
            );
            s.clock = 100.0;
            assert_eq!(s.remaining, 6);
            for _ in 0..6 {
                s.advance();
            }
            s.action();
            assert_eq!(s.returned, 0);
            assert_eq!(s.cooldown, 0);
        }
    }
    #[test]
    fn refusal_and_blocked_contact_are_separate() {
        for c in [Case::Silence, Case::Transformed] {
            let mut s = State::new(1, c, 2);
            s.begin();
            s.action();
            assert!(s.begun.is_none());
            assert_eq!(s.hits, 0);
        }
        for skill in 0..2 {
            let mut s = State::new(skill, Case::Wall, 2);
            s.begin();
            s.action();
            assert_eq!(s.hits, 0);
        }
    }
}
