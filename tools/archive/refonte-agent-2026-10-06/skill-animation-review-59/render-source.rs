//! ANIM-S29: stealth direct-hit bonus and discretion, prepared visual fixtures.
use super::super::melee::{actor, cut};
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;
const SIZE: f32 = 576.0;
const START: GridPos = GridPos::new(4, 6);
const TARGET: GridPos = GridPos::new(4, 3);
const IDS: [&str; 2] = ["SF00", "SF02"];
const TITLES: [&str; 2] = ["Furtivité · attaque sournoise", "Discrétion renforcée"];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#########\n#.......#\n#.......#\n#.......#\n#########",
];
const FRAMES: usize = 160;
const ACTION_FRAMES: [usize; 4] = [64, 88, 112, 136];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    Open,
    Quiet,
    Magic,
    Melee,
    Wall,
    Normal,
    Rank11,
    Rank12,
    Walk,
}
impl Case {
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Tir, tir ordinaire, puis nouveau camouflage",
            Self::Quiet => "Attente sans action",
            Self::Magic => "Sort direct de feu (exemple)",
            Self::Melee => "Attaque de mêlée",
            Self::Wall => "Cible derrière une cloison",
            Self::Normal => "Attaques à découvert",
            Self::Rank11 => "Furtivité au rang 11",
            Self::Rank12 => "Furtivité au rang 12",
            Self::Walk => "Un pas avant l’attaque",
        }
    }
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    begun: Option<f32>,
    fade: Option<(f32, f32, f32)>,
    mode_event: Option<f32>,
    active: bool,
    step: u16,
    turn: u16,
    energy: u16,
    player: GridPos,
    from: GridPos,
    move_at: Option<f32>,
    shot: Option<f32>,
    resolved: bool,
    shots: u16,
    snapshot_bonus_percent: u16,
    last_damage: u16,
    last_bonus: u16,
    total_damage: u16,
    target_hp: u16,
    hit: Option<f32>,
    notice: &'static str,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.0,
            begun: None,
            fade: None,
            mode_event: None,
            active: false,
            step: 0,
            turn: 0,
            energy: 100,
            player: START,
            from: START,
            move_at: None,
            shot: None,
            resolved: true,
            shots: 0,
            snapshot_bonus_percent: 0,
            last_damage: 0,
            last_bonus: 0,
            total_damage: 0,
            target_hp: 100,
            hit: None,
            notice: "",
        }
    }
    fn map_index(&self) -> usize {
        usize::from(self.case == Case::Wall)
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn rank(&self) -> u16 {
        match self.case {
            Case::Rank11 => 11,
            Case::Rank12 => 12,
            _ => 10,
        }
    }
    fn bonus_percent(&self) -> u16 {
        self.rank() * 5
    }
    fn distance(&self) -> u16 {
        if self.skill == 1 { 1 } else { 2 }
    }
    fn target(&self) -> GridPos {
        if self.case == Case::Melee {
            GridPos::new(4, 5)
        } else {
            TARGET
        }
    }
    fn contact(&self) -> bool {
        self.case != Case::Wall
    }
    fn opacity(&self) -> f32 {
        self.fade
            .map_or(if self.active { 0.56 } else { 1.0 }, |(t, a, b)| {
                a + (b - a) * ((self.clock - t) / 0.24).clamp(0.0, 1.0)
            })
    }
    fn set_mode(&mut self, on: bool) {
        let before = self.opacity();
        self.active = on;
        self.fade = Some((self.clock, before, if on { 0.56 } else { 1.0 }));
        if on {
            self.mode_event = Some(self.clock);
        }
    }
    fn begin(&mut self) {
        if self.begun.is_some() {
            return;
        }
        self.begun = Some(self.clock);
        self.turn = 1;
        if self.case != Case::Normal {
            self.set_mode(true);
        }
    }
    fn attack(&mut self) {
        if !self.contact() {
            self.notice = "Attaque indisponible dans cette scène.";
            return;
        }
        self.snapshot_bonus_percent = if self.active { self.bonus_percent() } else { 0 };
        if self.active {
            self.set_mode(false);
        }
        self.shot = Some(self.clock);
        self.shots += 1;
        self.resolved = false;
        self.turn += 1;
    }
    fn flight(&self) -> f32 {
        if self.case == Case::Magic { 0.35 } else { 0.12 }
    }
    fn advance(&mut self) {
        if self.begun.is_none()
            || self.clock - self.begun.unwrap() < 0.85
            || self.step >= 4
            || !self.resolved
        {
            return;
        }
        self.step += 1;
        if self.case == Case::Walk && self.step == 1 {
            self.from = self.player;
            self.player = GridPos::new(4, 5);
            self.move_at = Some(self.clock);
            self.energy -= 1;
            self.turn += 1;
            return;
        }
        if self.step == 3 {
            if self.case != Case::Normal && self.contact() {
                self.set_mode(true);
                self.turn += 1;
            }
            return;
        }
        self.attack();
    }
    fn resolve(&mut self) {
        let Some(t) = self.shot else {
            return;
        };
        if self.resolved || self.clock - t < self.flight() {
            return;
        }
        self.resolved = true;
        self.last_bonus = 20 * self.snapshot_bonus_percent / 100;
        self.last_damage = 20 + self.last_bonus;
        self.total_damage += self.last_damage;
        self.target_hp -= self.last_damage;
        self.hit = Some(t + self.flight());
    }
    fn variant(&self) -> String {
        format!(
            "{}-{}",
            IDS[self.skill],
            if self.skill == 0 { "A02" } else { "A01" }
        )
    }
}
fn point(p: GridPos, d: usize, r: Rect) -> Vec2 {
    let p = position(p, d);
    vec2(
        r.x + (p.x as f32 + 0.5) * 64.0,
        r.y + (p.y as f32 + 0.5) * 64.0,
    )
}
fn player_point(s: &State, r: Rect) -> Vec2 {
    let p = point(s.player, s.direction, r);
    s.move_at.map_or(p, |t| {
        point(s.from, s.direction, r).lerp(p, ((s.clock - t) / 0.12).clamp(0.0, 1.0))
    })
}
fn reaction(s: &State, r: Rect, effects: bool) -> Vec2 {
    if !effects {
        return Vec2::ZERO;
    }
    s.hit.map_or(Vec2::ZERO, |t| {
        crate::combat_visuals::hit_reaction_offset(
            point(s.target(), s.direction, r),
            player_point(s, r),
            f64::from(s.clock - t),
            64.0,
        )
    })
}
fn visual(s: &State) -> Visual {
    game_art::wear(
        game_art::player(
            Some("core:traqueur"),
            if s.case == Case::Melee {
                PlayerWeaponPose::Melee
            } else {
                PlayerWeaponPose::Ranged
            },
            Some(if s.case == Case::Melee {
                "core:melee_epee_longue"
            } else {
                "core:fusil_de_patrouille"
            }),
        ),
        [("core:body_armor".into(), "core:tenue_traque".into())],
    )
}

struct Art {
    icons: [Texture2D; 2],
    blood: Texture2D,
    ball: Texture2D,
    blast: Texture2D,
    fire_icon: Texture2D,
}
impl Art {
    fn load() -> Self {
        let load = |b: &[u8]| {
            let t = Texture2D::from_file_with_format(b, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            icons: [
                load(include_bytes!(
                    "../../../assets/skills/agent/SF00-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/SF02-compact.png"
                )),
            ],
            blood: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-40/blood-atlas.png"
            )),
            ball: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-16/projectile-atlas.png"
            )),
            blast: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-20/burst-atlas.png"
            )),
            fire_icon: load(include_bytes!(
                "../../../assets/skills/agent/FE01-compact.png"
            )),
        }
    }
    fn cloak(&self, s: &State, r: Rect) {
        let Some((t, _, _)) = s.fade else {
            return;
        };
        let q = (s.clock - t) / 0.24;
        if !(0.0..1.0).contains(&q) {
            return;
        }
        let p = player_point(s, r);
        let y = p.y - 29.0 + q * 56.0;
        let life = (1.0 - q) * 0.85;
        let c = alpha(Color::new(0.40, 0.78, 0.90, 1.0), life);
        // A short technological scan, then only the translucent original layers remain.
        for side in [-1.0, 1.0] {
            let a = vec2(p.x + side * 16.0, y - 8.0);
            let b = vec2(p.x + side * 24.0, y);
            let d = vec2(p.x + side * 17.0, y + 7.0);
            draw_line(a.x, a.y, b.x, b.y, 4.7, alpha(INK, life));
            draw_line(b.x, b.y, d.x, d.y, 4.7, alpha(INK, life));
            draw_line(a.x, a.y, b.x, b.y, 2.4, c);
            draw_line(b.x, b.y, d.x, d.y, 2.4, c);
        }
    }

    fn attack_effect(&self, s: &State, r: Rect) {
        let Some(t) = s.shot else {
            return;
        };
        let age = s.clock - t;
        let from = player_point(s, r) + vec2(0.0, -9.0);
        let at = point(s.target(), s.direction, r);
        let d = (at - from).normalize();
        if s.case == Case::Magic {
            if (0.0..0.35).contains(&age) {
                let p = from.lerp(at, age / 0.35);
                let frames = [0, 1, 2, 3, 2, 1, 0, 1];
                let f = frames[((age * 8.0) as usize) % 8];
                let size = 96.0;
                draw_texture_ex(
                    &self.ball,
                    p.x - size * 190.0 / 256.0,
                    p.y - size * 132.0 / 256.0,
                    alpha(WHITE, 0.9),
                    DrawTextureParams {
                        source: Some(Rect::new(f as f32 * 256.0, 0.0, 256.0, 256.0)),
                        dest_size: Some(vec2(size, size)),
                        rotation: d.y.atan2(d.x),
                        pivot: Some(p),
                        ..Default::default()
                    },
                );
            }
            let q = age - 0.35;
            if (0.0..0.55).contains(&q) {
                let f = ((q / 0.55 * 6.0) as usize).min(5);
                let size = 134.0;
                draw_texture_ex(
                    &self.blast,
                    at.x - size * 132.0 / 256.0,
                    at.y - size * 146.0 / 256.0,
                    alpha(WHITE, 0.95),
                    DrawTextureParams {
                        source: Some(Rect::new(f as f32 * 256.0, 0.0, 256.0, 256.0)),
                        dest_size: Some(vec2(size, size)),
                        ..Default::default()
                    },
                );
            }
        } else if (0.0..0.12).contains(&age) {
            if s.case == Case::Melee {
                let n = vec2(-d.y, d.x);
                cut(at - n * 24.0, at + n * 24.0, 7.0, 1.0 - age / 0.12);
            } else {
                let p = from.lerp(at, age / 0.12);
                draw_line(p.x - d.x * 19.0, p.y - d.y * 19.0, p.x, p.y, 5.5, INK);
                draw_line(
                    p.x - d.x * 17.0,
                    p.y - d.y * 17.0,
                    p.x,
                    p.y,
                    3.3,
                    Color::new(0.88, 0.91, 0.92, 1.0),
                );
            }
        }
    }
    fn bodies(&self, s: &State, r: Rect, effects: bool) {
        if s.contact() {
            actor(
                point(s.target(), s.direction, r) + reaction(s, r, effects),
                Visual::Bestiary {
                    number: 12,
                    corpse: false,
                },
            );
        }
        actor(
            point(GridPos::new(6, 3), s.direction, r),
            Visual::Bestiary {
                number: 12,
                corpse: false,
            },
        );
        let p = player_point(s, r);
        game_art::draw_world(
            Rect::new(p.x - 32.0, p.y - 32.0, 64.0, 64.0),
            visual(s),
            if effects { s.opacity() } else { 1.0 },
        );
    }
    fn wound(&self, s: &State, r: Rect) {
        if s.case == Case::Magic {
            return;
        }
        let Some(a) = s
            .hit
            .map(|t| s.clock - t)
            .filter(|a| (0.0..0.30).contains(a))
        else {
            return;
        };
        let at = point(s.target(), s.direction, r) + reaction(s, r, true);
        let f = ((a / 0.30 * 6.0) as usize).min(5);
        draw_texture_ex(
            &self.blood,
            at.x - 31.0,
            at.y - 25.0,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(f as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(vec2(62.0, 62.0)),
                ..Default::default()
            },
        );
    }
    fn hud(&self, s: &State, r: Rect, interactive: bool) {
        draw_rectangle(r.x + 12.0, r.y + 12.0, 286.0, 85.0, alpha(INK, 0.96));
        draw_texture_ex(
            &self.icons[usize::from(s.skill == 1)],
            r.x + 20.0,
            r.y + 28.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                ..Default::default()
            },
        );
        draw_text(
            if s.skill == 0 {
                "Bonus de l’attaque furtive"
            } else {
                "Passif · rang 11"
            },
            r.x + 61.0,
            r.y + 34.0,
            14.0,
            if s.skill == 1 {
                Color::new(0.38, 0.77, 1.0, 1.0)
            } else {
                DIM
            },
        );
        draw_text(
            if s.skill == 0 {
                format!("Furtivité rang {} : +{} %", s.rank(), s.bonus_percent())
            } else {
                "Distance de repérage : 1 case".into()
            },
            r.x + 61.0,
            r.y + 57.0,
            14.0,
            TEXT,
        );
        draw_text(
            if s.skill == 0 {
                "Distance de repérage : 2 cases".to_string()
            } else {
                format!("Furtivité rang {} : +{} %", s.rank(), s.bonus_percent())
            },
            r.x + 20.0,
            r.y + 81.0,
            13.0,
            DIM,
        );
        if s.active {
            let icon = Rect::new(r.x + 488.0, r.y + 16.0, 40.0, 40.0);
            draw_rectangle(icon.x, icon.y, icon.w, icon.h, alpha(INK, 0.97));
            draw_rectangle_lines(icon.x, icon.y, icon.w, icon.h, 1.0, DIM);
            draw_texture_ex(
                &self.icons[0],
                icon.x + 4.0,
                icon.y + 4.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(32.0, 32.0)),
                    ..Default::default()
                },
            );
            if interactive && icon.contains(vec2(mouse_position().0, mouse_position().1)) {
                draw_rectangle(r.x + 268.0, r.y + 110.0, 286.0, 110.0, alpha(INK, 0.98));
                draw_text_bold("Furtivité", r.x + 280.0, r.y + 132.0, 16.0, TEXT);
                draw_text(
                    format!("Attaque directe : +{} %", s.bonus_percent()),
                    r.x + 280.0,
                    r.y + 157.0,
                    14.0,
                    TEXT,
                );
                draw_text(
                    format!(
                        "Distance de repérage : {} {}",
                        s.distance(),
                        if s.distance() == 1 { "case" } else { "cases" }
                    ),
                    r.x + 280.0,
                    r.y + 182.0,
                    14.0,
                    TEXT,
                );
                draw_text("Énergie par pas : 1", r.x + 280.0, r.y + 207.0, 14.0, TEXT);
            }
        }
    }
    fn floating(&self, s: &State, r: Rect) {
        let Some(a) = s
            .hit
            .map(|t| s.clock - t)
            .filter(|a| (0.0..0.65).contains(a))
        else {
            return;
        };
        let at = point(s.target(), s.direction, r);
        let c = alpha(
            if s.case == Case::Magic {
                Color::new(1.0, 0.61, 0.18, 1.0)
            } else {
                TEXT
            },
            1.0 - a / 0.65,
        );
        let y = at.y - 41.0 - a * 14.0;
        draw_text_bold(
            if s.last_bonus > 0 {
                format!("{} (+{})", s.last_damage, s.last_bonus)
            } else {
                s.last_damage.to_string()
            },
            at.x - if s.last_bonus > 0 { 37.0 } else { 12.0 },
            y,
            20.0,
            c,
        );
        draw_text("(", at.x - 22.0, y + 14.0, 12.0, c);
        if s.case == Case::Magic {
            draw_texture_ex(
                &self.fire_icon,
                at.x - 17.0,
                y + 3.0,
                alpha(WHITE, 1.0 - a / 0.65),
                DrawTextureParams {
                    dest_size: Some(vec2(12.0, 12.0)),
                    ..Default::default()
                },
            );
        } else {
            draw_line(at.x - 15.0, y + 13.0, at.x - 9.0, y + 6.0, 2.0, c);
        }
        draw_text(format!("{})", s.last_damage), at.x - 1.0, y + 14.0, 12.0, c);
    }
}
fn scene(art: &Art, clip: &Clip, s: &State, r: Rect, effects: bool, interactive: bool) {
    terrain(&s.map(), s.direction, r, false);
    if s.begun.is_none() {
        preview(&[START], s.direction, r, s.clock);
    }
    terrain(&s.map(), s.direction, r, true);
    clip.apply_for(s.map_index(), s.direction, r);
    if effects {
        art.attack_effect(s, r);
    }
    art.bodies(s, r, effects);
    if effects {
        art.cloak(s, r);
        art.wound(s, r);
    }
    gl_use_default_material();
    art.hud(s, r, interactive);
    art.floating(s, r);
    if let Some(t) = s.mode_event {
        let a = s.clock - t;
        if a < 0.38 {
            let p = player_point(s, r);
            draw_text_bold(
                "Furtivité",
                p.x - 29.0,
                p.y - 45.0 - a * 8.0,
                15.0,
                alpha(TEXT, 1.0 - a / 0.38),
            );
        }
    }
    draw_rectangle(r.x + 5.0, r.y + SIZE - 63.0, 566.0, 58.0, alpha(INK, 0.95));
    draw_text(
        format!(
            "Tour {} · énergie {} · tirs {} · PV cible {}",
            s.turn, s.energy, s.shots, s.target_hp
        ),
        r.x + 12.0,
        r.y + SIZE - 46.0,
        14.0,
        TEXT,
    );
    draw_text(
        if s.active {
            "Furtivité active"
        } else {
            "À découvert"
        },
        r.x + 12.0,
        r.y + SIZE - 27.0,
        14.0,
        DIM,
    );
    draw_text(
        if !s.notice.is_empty() {
            s.notice.to_string()
        } else if s.last_damage > 0 {
            format!(
                "Dernier impact : {} dégâts {} (+{} bonus)",
                s.last_damage,
                if s.case == Case::Magic {
                    "de feu"
                } else {
                    "physiques"
                },
                s.last_bonus
            )
        } else {
            "Dégâts directs préparés · base 20".into()
        },
        r.x + 12.0,
        r.y + SIZE - 10.0,
        12.0,
        TEXT,
    );
}
pub(crate) async fn run() {
    request_new_screen_size(1280.0, 820.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let clip = Clip::load_maps(&MAPS, START);
    let mut s = State::new(0, Case::Open, 2);
    let mut paused = false;
    let cases = [
        Case::Open,
        Case::Quiet,
        Case::Magic,
        Case::Melee,
        Case::Wall,
        Case::Normal,
        Case::Rank11,
        Case::Rank12,
        Case::Walk,
    ];
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Furtivité · ANIM-S29", 20.0, 33.0, 24.0, TEXT);
        for (i, t) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 80.0 + i as f32 * 40.0, 370.0, 34.0),
                t,
                true,
            ) {
                s = State::new(i, Case::Open, s.direction);
            }
        }
        for (i, t) in DIRS.iter().enumerate() {
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
                Rect::new(
                    20.0 + (i % 2) as f32 * 185.0,
                    270.0 + (i / 2) as f32 * 35.0,
                    178.0,
                    30.0,
                ),
                c.title(),
                true,
            ) {
                s = State::new(s.skill, *c, s.direction);
            }
        }
        if button(
            Rect::new(20.0, 465.0, 350.0, 34.0),
            "Préparer la scène",
            s.begun.is_none(),
        ) || is_key_pressed(KeyCode::Enter)
        {
            s.begin();
        }
        if button(
            Rect::new(20.0, 507.0, 350.0, 34.0),
            "Action suivante",
            s.begun.is_some()
                && s.clock - s.begun.unwrap_or(s.clock) >= 0.85
                && s.resolved
                && s.step < 4,
        ) || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(20.0, 549.0, 160.0, 34.0), "Recommencer", true)
            || is_key_pressed(KeyCode::R)
        {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(190.0, 549.0, 160.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
            s.resolve();
        }
        scene(
            &art,
            &clip,
            &s,
            Rect::new(410.0, 88.0, SIZE, SIZE),
            true,
            true,
        );
        draw_text(
            "Détection, sorts et dégâts préparés ; aucune IA simulée.",
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
    let clip = Clip::load_maps(&MAPS, START);
    let target = render_target(640, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 640.0, 720.0))
    };
    let controls = [
        14, 17, 20, 24, 28, 32, 44, 60, 64, 66, 68, 72, 76, 80, 84, 88, 92, 96, 100, 108, 112, 116,
        124, 136, 140, 144, 148, 158,
    ];
    let mut entries = Vec::new();
    for skill in 0..2 {
        let cases = vec![
            (Case::Open, 2, "HAUT"),
            (Case::Open, 0, "DROITE"),
            (Case::Open, 1, "GAUCHE"),
            (Case::Open, 3, "BAS"),
            (Case::Quiet, 2, "ATTENTE"),
            (Case::Magic, 2, "SORT"),
            (Case::Melee, 2, "MELEE"),
            (Case::Wall, 2, "CLOISON"),
            (Case::Normal, 2, "DECOUVERT"),
            (Case::Rank11, 2, "RANG-11"),
            (Case::Rank12, 2, "RANG-12"),
            (Case::Walk, 2, "PAS"),
        ];
        for (case, direction, label) in cases {
            let mut s = State::new(skill, case, direction);
            let folder = format!("{}-{label}", IDS[skill]);
            let dir = output.join(&folder);
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let mut timeline = Vec::new();
            for frame in 0..FRAMES {
                s.clock = frame as f32 / 24.0;
                if frame == 17 {
                    s.begin();
                }
                s.resolve();
                if case != Case::Quiet && ACTION_FRAMES.contains(&frame) {
                    s.advance();
                }
                timeline.push(serde_json::json!({"frame":frame,"begun":s.begun.is_some(),"active":s.active,"step":s.step,"turn":s.turn,"energy":s.energy,"shots":s.shots,"rank":s.rank(),"bonus_percent":s.bonus_percent(),"snapshot_bonus_percent":s.snapshot_bonus_percent,"last_damage":s.last_damage,"last_bonus":s.last_bonus,"total_damage":s.total_damage,"target_hp":s.target_hp,"distance":s.distance(),"opacity":s.opacity(),"player":[s.player.x,s.player.y],"resolved":s.resolved,"preview_visible":s.begun.is_none()}));
                for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                    set_camera(&camera);
                    clear_background(INK);
                    draw_text_bold(
                        format!("{} · {}", if skill == 0 { 65 } else { 67 }, TITLES[skill]),
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
                        control == 0,
                        false,
                    );
                    draw_text(
                        "Rang 10 : +50 % · première attaque renforcée puis fin du mode",
                        32.0,
                        684.0,
                        13.0,
                        TEXT,
                    );
                    draw_text(
                        "Exemples directs seulement ; variantes complexes non simulées.",
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
            entries.push(serde_json::json!({"folder":folder,"variant":format!("{}-{label}",s.variant()),"id":IDS[skill],"number":if skill==0{65}else{67},"title":TITLES[skill],"case":label,"case_title":case.title(),"direction":direction,"permitted":true,"quiet":case==Case::Quiet,"contact":s.contact(),"control_frames":controls,"action_frames":if case==Case::Quiet{vec![]}else{ACTION_FRAMES.to_vec()},"tile":64,"frames":FRAMES,"fps":24,"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n")}));
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
