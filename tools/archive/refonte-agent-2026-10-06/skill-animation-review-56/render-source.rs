//! ANIM-S26: channelled heavy fire and permanent mastery, prepared review only.
use super::super::melee::{actor, cut};
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;
const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 6);
const CENTER: GridPos = GridPos::new(4, 3);
const TARGETS: [GridPos; 4] = [
    GridPos::new(3, 3),
    CENTER,
    GridPos::new(5, 3),
    GridPos::new(6, 5),
];
const IDS: [&str; 2] = ["HL03", "HL04"];
const TITLES: [&str; 2] = ["Déluge d’acier", "Maîtrise des armes lourdes"];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#########\n#.......#\n#.......#\n#.......#\n#########",
];
const FRAMES: usize = 160;
const ACTION_FRAMES: [usize; 4] = [64, 88, 112, 136];
const FLIGHT: f32 = 0.20;
const BLAST: f32 = 0.42;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    Open,
    Quiet,
    Wall,
    Empty,
    NoBox,
    Silence,
    Poor,
    Move,
    Attack,
    Skill,
    Hurt,
    Lost,
    Light,
    Melee,
}
impl Case {
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Bombardement complet",
            Self::Quiet => "Attente sans action",
            Self::Wall => "Cloison",
            Self::Empty => "Aucun ennemi",
            Self::NoBox => "Boîte absente",
            Self::Silence => "Silence",
            Self::Poor => "Énergie limitée",
            Self::Move => "Déplacement",
            Self::Attack => "Autre attaque",
            Self::Skill => "Autre compétence",
            Self::Hurt => "Dégâts reçus",
            Self::Lost => "Plus de cible repérée",
            Self::Light => "Arme légère",
            Self::Melee => "Arme de mêlée",
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
    step: u16,
    turn: u16,
    energy: u16,
    hp: u16,
    cooldown: u16,
    active: bool,
    remaining: u16,
    shots: u16,
    damage: [u16; 4],
    last_damage: [u16; 4],
    reaction: [Option<f32>; 4],
    resolved: bool,
    aim: GridPos,
    gone: bool,
    moved: Option<f32>,
    player_hit: Option<f32>,
    stop_reason: &'static str,
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
            step: 0,
            turn: 0,
            energy: if case == Case::Poor { 12 } else { 100 },
            hp: 100,
            cooldown: 0,
            active: false,
            remaining: 0,
            shots: 0,
            damage: [0; 4],
            last_damage: [0; 4],
            reaction: [None; 4],
            resolved: true,
            aim: if case == Case::Melee {
                GridPos::new(4, 5)
            } else {
                CENTER
            },
            gone: false,
            moved: None,
            player_hit: None,
            stop_reason: "",
        }
    }
    fn map_index(&self) -> usize {
        usize::from(self.case == Case::Wall)
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn allowed(&self) -> bool {
        !matches!(self.case, Case::Wall | Case::Empty | Case::NoBox)
            && !(self.skill == 0 && matches!(self.case, Case::Silence | Case::Light | Case::Melee))
    }
    fn target(&self, n: usize) -> GridPos {
        if self.case == Case::Melee && n == 1 {
            GridPos::new(4, 5)
        } else {
            TARGETS[n]
        }
    }
    fn visible(&self, n: usize) -> bool {
        self.case != Case::Empty && (!self.gone || n == 3)
    }
    fn heavy(&self) -> bool {
        self.skill == 0 || !matches!(self.case, Case::Light | Case::Melee)
    }
    fn zone(&self) -> Vec<GridPos> {
        (-2..=2)
            .flat_map(|dy| (-2..=2).map(move |dx| (dx, dy)))
            .filter(|&(x, y)| x * x + y * y <= 4)
            .map(|(x, y)| GridPos::new(CENTER.x + x, CENTER.y + y))
            .filter(|p| self.map().is_walkable(*p))
            .collect()
    }
    fn footprint(&self) -> Vec<GridPos> {
        if !self.heavy() {
            return vec![self.aim];
        }
        [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(x, y)| GridPos::new(self.aim.x + x, self.aim.y + y))
            .filter(|p| self.map().is_walkable(*p))
            .collect()
    }
    fn stop(&mut self, reason: &'static str) {
        self.active = false;
        self.remaining = 0;
        self.cooldown = 8;
        self.stop_reason = reason;
    }
    fn fire(&mut self) {
        if self.skill == 0 {
            if self.gone {
                self.stop("Plus de cible repérée");
                return;
            }
            if self.energy < 8 {
                self.stop("Énergie insuffisante");
                return;
            }
            self.energy -= 8;
            self.aim = TARGETS[(self.shots % 3) as usize];
        } else {
            self.aim = self.target(1);
        }
        self.shots += 1;
        self.event = Some(self.clock);
        self.resolved = false;
    }
    fn begin(&mut self) {
        if self.begun.is_some() || !self.allowed() {
            return;
        }
        self.begun = Some(self.clock);
        self.turn = 1;
        if self.skill == 0 {
            self.active = true;
            self.remaining = 4;
        }
        self.fire();
    }
    fn age(&self) -> Option<f32> {
        self.event.map(|t| self.clock - t)
    }
    fn resolve(&mut self) {
        let flight = if self.heavy() { FLIGHT } else { 0.12 };
        if self.resolved || self.age().is_none_or(|age| age < flight) {
            return;
        }
        self.resolved = true;
        let amount = if self.skill == 0 {
            46
        } else if self.heavy() {
            42
        } else {
            20
        };
        let cells = self.footprint();
        for n in 0..4 {
            if self.visible(n) && cells.contains(&self.target(n)) {
                self.damage[n] += amount;
                self.last_damage[n] = amount;
                self.reaction[n] = Some(self.event.unwrap() + flight);
            }
        }
        if self.skill == 0 && self.shots == 4 && self.active {
            self.stop("Bombardement terminé");
        }
    }
    fn advance(&mut self) {
        if self.begun.is_none() || !self.resolved || self.age().unwrap_or(0.0) < 0.72 {
            return;
        }
        self.step += 1;
        self.turn += 1;
        if self.skill == 1 {
            self.fire();
            return;
        }
        if !self.active {
            self.cooldown = self.cooldown.saturating_sub(1);
            return;
        }
        if self.step == 1 {
            match self.case {
                Case::Move => {
                    self.moved = Some(self.clock);
                    self.stop("Déplacement : bombardement interrompu");
                    return;
                }
                Case::Attack => {
                    self.stop("Autre attaque : bombardement interrompu");
                    return;
                }
                Case::Skill => {
                    self.stop("Autre compétence : bombardement interrompu");
                    return;
                }
                Case::Lost => self.gone = true,
                Case::Hurt => {
                    self.hp -= 5;
                    self.player_hit = Some(self.clock);
                }
                _ => {}
            }
        }
        self.remaining = self.remaining.saturating_sub(1);
        self.fire();
    }
    fn variant(&self) -> String {
        format!("{}-A01", IDS[self.skill])
    }
}
fn point(at: GridPos, direction: usize, rect: Rect) -> Vec2 {
    let p = position(at, direction);
    vec2(
        rect.x + (p.x as f32 + 0.5) * 64.0,
        rect.y + (p.y as f32 + 0.5) * 64.0,
    )
}
fn player_point(s: &State, rect: Rect) -> Vec2 {
    let at = point(PLAYER, s.direction, rect);
    let at = if let Some(t) = s.moved {
        at.lerp(
            point(GridPos::new(3, 6), s.direction, rect),
            ((s.clock - t) / 0.16).clamp(0.0, 1.0),
        )
    } else {
        at
    };
    at + s.player_hit.map_or(Vec2::ZERO, |t| {
        crate::combat_visuals::hit_reaction_offset(
            at,
            point(TARGETS[3], s.direction, rect),
            f64::from(s.clock - t),
            64.0,
        )
    })
}
fn target_offset(s: &State, n: usize, rect: Rect, effects: bool) -> Vec2 {
    if !effects {
        return Vec2::ZERO;
    }
    s.reaction[n].map_or(Vec2::ZERO, |t| {
        crate::combat_visuals::hit_reaction_offset(
            point(s.target(n), s.direction, rect),
            point(PLAYER, s.direction, rect),
            f64::from(s.clock - t),
            64.0,
        )
    })
}
struct Art {
    shared: super::heavy_opening::Art,
    icons: [Texture2D; 2],
}
impl Art {
    fn load() -> Self {
        let load = |bytes: &[u8]| {
            let t = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            shared: super::heavy_opening::Art::load(),
            icons: [
                load(include_bytes!(
                    "../../../assets/skills/agent/HL03-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/HL04-compact.png"
                )),
            ],
        }
    }
    fn effects(&self, s: &State, rect: Rect) {
        let Some(age) = s.age() else {
            return;
        };
        let origin = player_point(s, rect);
        let at = point(s.aim, s.direction, rect);
        let d = (at - origin).normalize_or_zero();
        if s.heavy() {
            if (0.0..FLIGHT).contains(&age) {
                let p = (origin + d * 30.0).lerp(at, age / FLIGHT);
                let w = 50.0;
                let h = w * self.shared.rocket.height() / self.shared.rocket.width();
                draw_texture_ex(
                    &self.shared.rocket,
                    p.x - w / 2.0,
                    p.y - h / 2.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(w, h)),
                        rotation: d.y.atan2(d.x),
                        pivot: Some(p),
                        ..Default::default()
                    },
                );
            }
            let a = age - FLIGHT;
            if (0.0..BLAST).contains(&a) {
                let f = ((a / BLAST * 6.0) as usize).min(5);
                let size = 134.0;
                draw_texture_ex(
                    &self.shared.blast,
                    at.x - size * 132.0 / 256.0,
                    at.y - size * 146.0 / 256.0,
                    alpha(WHITE, 0.9),
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
                cut(at - n * 23.0, at + n * 25.0, 7.0, 1.0 - age / 0.12);
            } else {
                let p = origin.lerp(at, age / 0.12);
                draw_line(p.x - d.x * 18.0, p.y - d.y * 18.0, p.x, p.y, 5.5, INK);
                draw_line(
                    p.x - d.x * 16.0,
                    p.y - d.y * 16.0,
                    p.x,
                    p.y,
                    3.3,
                    Color::new(0.88, 0.91, 0.92, 1.0),
                );
            }
        }
        if let Some(a) = s
            .player_hit
            .map(|t| s.clock - t)
            .filter(|a| (0.0..0.15).contains(a))
        {
            let from = point(TARGETS[3], s.direction, rect);
            let to = point(PLAYER, s.direction, rect);
            let p = from.lerp(to, (a / 0.08).min(1.0));
            let d = (to - from).normalize();
            draw_line(p.x - d.x * 16.0, p.y - d.y * 16.0, p.x, p.y, 5.0, INK);
            draw_line(p.x - d.x * 16.0, p.y - d.y * 16.0, p.x, p.y, 2.8, WHITE);
        }
    }
    fn bodies(&self, s: &State, rect: Rect, effects: bool) {
        for n in 0..4 {
            if s.visible(n) {
                actor(
                    point(s.target(n), s.direction, rect) + target_offset(s, n, rect, effects),
                    Visual::Bestiary {
                        number: 12,
                        corpse: false,
                    },
                );
            }
        }
        let p = if effects {
            player_point(s, rect)
        } else if s.moved.is_some() {
            point(GridPos::new(3, 6), s.direction, rect)
        } else {
            point(PLAYER, s.direction, rect)
        };
        actor(
            p,
            game_art::wear(
                game_art::player(
                    Some("core:bastion"),
                    if s.case == Case::Melee {
                        PlayerWeaponPose::Melee
                    } else {
                        PlayerWeaponPose::Ranged
                    },
                    Some(match s.case {
                        Case::Melee => "core:melee_epee_longue",
                        Case::Light => "core:modele_cogneur_p01",
                        _ => "core:lance_roquettes",
                    }),
                ),
                [("core:body_armor".into(), "core:tenue_assaut".into())],
            ),
        );
    }
    fn blood(&self, at: Vec2, age: f32) {
        if !(0.0..0.30).contains(&age) {
            return;
        }
        let f = ((age / 0.30 * 6.0) as usize).min(5);
        draw_texture_ex(
            &self.shared.blood,
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
    fn wounds(&self, s: &State, rect: Rect) {
        for n in 0..4 {
            if let Some(t) = s.reaction[n] {
                self.blood(
                    point(s.target(n), s.direction, rect) + target_offset(s, n, rect, true),
                    s.clock - t,
                );
            }
        }
        if let Some(t) = s.player_hit {
            self.blood(player_point(s, rect), s.clock - t);
        }
    }
    fn hud(&self, s: &State, rect: Rect, interactive: bool) {
        if s.skill == 1 {
            draw_rectangle(rect.x + 12.0, rect.y + 12.0, 246.0, 69.0, alpha(INK, 0.97));
            draw_texture_ex(
                &self.icons[1],
                rect.x + 20.0,
                rect.y + 24.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(32.0, 32.0)),
                    ..Default::default()
                },
            );
            draw_text("Passif permanent", rect.x + 60.0, rect.y + 34.0, 14.0, DIM);
            draw_text(
                if s.heavy() {
                    "Dégâts : 42 (+22)"
                } else {
                    "Arme incompatible : bonus 0"
                },
                rect.x + 60.0,
                rect.y + 58.0,
                14.0,
                TEXT,
            );
            return;
        }
        if !s.active {
            return;
        }
        let r = Rect::new(rect.x + 488.0, rect.y + 16.0, 40.0, 40.0);
        draw_rectangle(r.x, r.y, r.w, r.h, alpha(INK, 0.97));
        draw_rectangle_lines(r.x, r.y, r.w, r.h, 1.0, DIM);
        draw_texture_ex(
            &self.icons[0],
            r.x + 4.0,
            r.y + 4.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                ..Default::default()
            },
        );
        draw_rectangle(r.x + 24.0, r.y + 24.0, 15.0, 15.0, INK);
        draw_text_bold(s.remaining.to_string(), r.x + 27.0, r.y + 37.0, 13.0, TEXT);
        if interactive && r.contains(vec2(mouse_position().0, mouse_position().1)) {
            draw_rectangle(rect.x + 272.0, rect.y + 67.0, 284.0, 88.0, alpha(INK, 0.98));
            draw_text_bold(TITLES[0], rect.x + 282.0, rect.y + 91.0, 16.0, TEXT);
            draw_text(
                "Un tir par tour · dégâts ×2,30",
                rect.x + 282.0,
                rect.y + 115.0,
                14.0,
                TEXT,
            );
            draw_text(
                format!("Durée restante : {} tours", s.remaining),
                rect.x + 282.0,
                rect.y + 139.0,
                14.0,
                TEXT,
            );
        }
    }
}
fn floating(at: Vec2, damage: u16, bonus: u16, age: f32) {
    if !(0.0..0.58).contains(&age) {
        return;
    }
    let c = alpha(TEXT, 1.0 - age / 0.58);
    let y = at.y - 33.0 - age * 12.0;
    draw_text_bold(
        if bonus > 0 {
            format!("{damage} (+{bonus})")
        } else {
            damage.to_string()
        },
        at.x - if bonus > 0 { 38.0 } else { 12.0 },
        y,
        19.0,
        c,
    );
    draw_text("(", at.x - 18.0, y + 12.0, 12.0, c);
    draw_line(at.x - 9.0, y + 11.0, at.x - 3.0, y + 5.0, 2.0, c);
    draw_text(format!("{damage})"), at.x + 1.0, y + 12.0, 12.0, c);
}
fn feedback(s: &State, rect: Rect) {
    if s.skill == 0 {
        if let Some(a) = s.begun.map(|t| s.clock - t).filter(|a| *a < 0.38) {
            let p = point(PLAYER, s.direction, rect);
            draw_text_bold(
                TITLES[0],
                p.x - 62.0,
                p.y - 45.0 - a * 8.0,
                15.0,
                alpha(TEXT, 1.0 - a / 0.38),
            );
        }
    }
    for n in 0..4 {
        if let Some(t) = s.reaction[n] {
            let stagger = if s.skill == 1 && s.heavy() && n < 3 && n != 1 {
                vec2(0.0, -24.0)
            } else {
                Vec2::ZERO
            };
            floating(
                point(s.target(n), s.direction, rect) + stagger,
                s.last_damage[n],
                if s.skill == 1 && s.heavy() { 22 } else { 0 },
                s.clock - t,
            );
        }
    }
    if let Some(t) = s.player_hit {
        floating(point(PLAYER, s.direction, rect), 5, 0, s.clock - t);
    }
}
fn scene(art: &Art, clip: &Clip, s: &State, rect: Rect, effects: bool, interactive: bool) {
    terrain(&s.map(), s.direction, rect, false);
    if s.begun.is_none() && s.allowed() {
        preview(
            &if s.skill == 0 {
                s.zone()
            } else {
                s.footprint()
            },
            s.direction,
            rect,
            s.clock,
        );
    }
    terrain(&s.map(), s.direction, rect, true);
    clip.apply_for(s.map_index(), s.direction, rect);
    if s.skill == 0 && s.active {
        let c = point(CENTER, s.direction, rect);
        draw_circle_lines(c.x, c.y, 144.0, 3.0, alpha(INK, 0.8));
        draw_circle_lines(c.x, c.y, 144.0, 1.4, Color::new(1.0, 0.70, 0.28, 0.62));
    }
    if effects {
        art.effects(s, rect);
    }
    art.bodies(s, rect, effects);
    if effects {
        art.wounds(s, rect);
    }
    gl_use_default_material();
    art.hud(s, rect, interactive);
    feedback(s, rect);
    draw_rectangle(
        rect.x + 5.0,
        rect.y + SIZE - 43.0,
        566.0,
        38.0,
        alpha(INK, 0.95),
    );
    draw_text(
        format!(
            "Tour {} · tirs {} · énergie {} · recharge {}",
            s.turn, s.shots, s.energy, s.cooldown
        ),
        rect.x + 12.0,
        rect.y + SIZE - 27.0,
        14.0,
        TEXT,
    );
    draw_text(
        if !s.allowed() {
            "Indisponible · aucune consommation."
        } else if !s.stop_reason.is_empty() {
            s.stop_reason
        } else if s.skill == 0 {
            "Zone fixe · un tir à chaque tour."
        } else {
            "Bonus permanent ; aucune durée ni recharge de maîtrise."
        },
        rect.x + 12.0,
        rect.y + SIZE - 10.0,
        13.0,
        DIM,
    );
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
        Case::Empty,
        Case::NoBox,
        Case::Silence,
        Case::Poor,
        Case::Move,
        Case::Attack,
        Case::Skill,
        Case::Hurt,
        Case::Lost,
        Case::Light,
        Case::Melee,
    ];
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Armes lourdes · ANIM-S26", 20.0, 33.0, 24.0, TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 80.0 + i as f32 * 40.0, 360.0, 34.0),
                title,
                true,
            ) {
                s = State::new(i, Case::Open, s.direction);
            }
        }
        for (i, title) in DIRS.iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 174.0,
                    179.0 + (i / 2) as f32 * 36.0,
                    166.0,
                    30.0,
                ),
                title,
                true,
            ) {
                s = State::new(s.skill, s.case, i);
            }
        }
        for (i, case) in cases.iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 185.0,
                    270.0 + (i / 2) as f32 * 32.0,
                    178.0,
                    27.0,
                ),
                case.title(),
                true,
            ) {
                s = State::new(s.skill, *case, s.direction);
            }
        }
        if button(
            Rect::new(20.0, 520.0, 350.0, 34.0),
            if s.skill == 0 {
                "Lancer le bombardement"
            } else {
                "Attaque ordinaire"
            },
            s.begun.is_none() && s.allowed(),
        ) {
            s.begin();
        }
        if button(
            Rect::new(20.0, 562.0, 350.0, 34.0),
            "Action suivante",
            s.begun.is_some() && s.resolved && s.age().unwrap_or(0.0) >= 0.72 && s.step < 4,
        ) {
            s.advance();
        }
        if button(Rect::new(20.0, 604.0, 160.0, 34.0), "Recommencer", true)
            || is_key_pressed(KeyCode::R)
        {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(190.0, 604.0, 160.0, 34.0),
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
            "Événements préparés · dégâts d’exemple · aucune partie modifiée.",
            410.0,
            704.0,
            13.0,
            DIM,
        );
        draw_text(
            "Monstres32 temporaires ; adaptation après les animations.",
            410.0,
            728.0,
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
        14, 17, 20, 22, 24, 26, 28, 32, 36, 44, 60, 64, 68, 72, 76, 84, 88, 92, 100, 112, 116, 124,
        136, 140, 148, 158,
    ];
    let mut entries = Vec::new();
    for skill in 0..2 {
        if skill == 0 && std::env::args().any(|arg| arg == "--cas-maitrise-seule") {
            continue;
        }
        let mut cases = vec![
            (Case::Open, 2, "HAUT"),
            (Case::Open, 0, "DROITE"),
            (Case::Open, 1, "GAUCHE"),
            (Case::Open, 3, "BAS"),
            (Case::Quiet, 2, "ATTENTE"),
            (Case::Wall, 2, "CLOISON"),
            (Case::NoBox, 2, "SANS-BOITE"),
        ];
        if skill == 0 {
            cases.extend([
                (Case::Empty, 2, "VIDE"),
                (Case::Silence, 2, "SILENCE"),
                (Case::Poor, 2, "ENERGIE"),
                (Case::Move, 2, "DEPLACEMENT"),
                (Case::Attack, 2, "AUTRE-ATTAQUE"),
                (Case::Skill, 2, "AUTRE-COMPETENCE"),
                (Case::Hurt, 2, "DEGATS-RECUS"),
                (Case::Lost, 2, "PLUS-DE-CIBLE"),
            ]);
        } else {
            cases.extend([(Case::Light, 2, "LEGERE"), (Case::Melee, 2, "MELEE")]);
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
                s.resolve();
                if case != Case::Quiet && ACTION_FRAMES.contains(&frame) {
                    s.advance();
                }
                timeline.push(serde_json::json!({"frame":frame,"begun":s.begun.is_some(),"active":s.active,"remaining":s.remaining,"shots":s.shots,"turn":s.turn,"step":s.step,"energy":s.energy,"hp":s.hp,"cooldown":s.cooldown,"damage":s.damage,"last_damage":s.last_damage,"resolved":s.resolved,"event_age":s.age(),"preview_visible":s.begun.is_none()&&s.allowed(),"aim":[s.aim.x,s.aim.y],"stop_reason":s.stop_reason,"mastery_bonus_percent":if skill==1&&s.heavy(){110}else{0}}));
                for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                    set_camera(&camera);
                    clear_background(INK);
                    draw_text_bold(
                        format!("{} · {}", 61 + skill, TITLES[skill]),
                        32.0,
                        35.0,
                        if skill == 0 { 24.0 } else { 22.0 },
                        TEXT,
                    );
                    draw_text(
                        format!("{} · {} · étape {}", s.variant(), label, s.step),
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
                        if skill == 0 {
                            "Rang 11 · maximum 4 tirs · énergie 8 par tir · recharge finale 8"
                        } else {
                            "Rang 11 · dégâts des armes lourdes +110 % · permanent"
                        },
                        32.0,
                        684.0,
                        13.0,
                        TEXT,
                    );
                    draw_text(
                        "Tirs et dégâts préparés ; ordre réel du combat à raccorder.",
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
            entries.push(serde_json::json!({"folder":folder,"variant":format!("{}-{label}",s.variant()),"id":IDS[skill],"number":61+skill,"title":TITLES[skill],"case":label,"case_title":case.title(),"direction":direction,"permitted":permitted,"quiet":case==Case::Quiet,"control_frames":controls,"action_frames":if case==Case::Quiet{vec![]}else{ACTION_FRAMES.to_vec()},"tile":64,"frames":FRAMES,"fps":24,"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n")}));
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
    fn settle(s: &mut State) {
        s.clock += 0.8;
        s.resolve();
    }
    #[test]
    fn four_shots_then_cooldown() {
        let mut s = State::new(0, Case::Open, 2);
        s.begin();
        settle(&mut s);
        assert_eq!((s.energy, s.shots, s.cooldown, s.remaining), (92, 1, 0, 4));
        for _ in 0..3 {
            s.advance();
            settle(&mut s);
        }
        assert_eq!((s.energy, s.shots, s.cooldown, s.active), (68, 4, 8, false));
        s.advance();
        settle(&mut s);
        assert_eq!((s.shots, s.cooldown), (4, 7));
    }
    #[test]
    fn idle_does_not_add_shots_or_advance_status() {
        let mut s = State::new(0, Case::Quiet, 2);
        s.begin();
        settle(&mut s);
        s.clock = 99.0;
        s.resolve();
        assert_eq!((s.shots, s.remaining, s.energy, s.cooldown), (1, 4, 92, 0));
    }
    #[test]
    fn listed_interruptions_stop_without_paying_another_shot() {
        for case in [
            Case::Move,
            Case::Attack,
            Case::Skill,
            Case::Lost,
            Case::Poor,
        ] {
            let mut s = State::new(0, case, 2);
            s.begin();
            settle(&mut s);
            let energy = s.energy;
            s.advance();
            settle(&mut s);
            assert_eq!(
                (s.shots, s.energy, s.active, s.cooldown),
                (1, energy, false, 8)
            );
        }
    }
    #[test]
    fn physical_hurt_does_not_interrupt_channel() {
        let mut s = State::new(0, Case::Hurt, 2);
        s.begin();
        settle(&mut s);
        s.advance();
        settle(&mut s);
        assert_eq!((s.hp, s.shots, s.active), (95, 2, true));
    }
    #[test]
    fn refused_casts_are_atomic() {
        for case in [
            Case::Wall,
            Case::NoBox,
            Case::Empty,
            Case::Silence,
            Case::Light,
            Case::Melee,
        ] {
            let mut s = State::new(0, case, 2);
            s.begin();
            settle(&mut s);
            assert_eq!((s.turn, s.shots, s.energy, s.cooldown), (0, 0, 100, 0));
        }
    }
    #[test]
    fn mastery_is_permanent_and_requires_heavy_weapon_damage() {
        for (case, damage) in [(Case::Open, 42), (Case::Light, 20), (Case::Melee, 20)] {
            let mut s = State::new(1, case, 2);
            s.begin();
            settle(&mut s);
            assert_eq!(s.last_damage[1], damage);
            assert_eq!(
                (s.energy, s.cooldown, s.active, s.remaining),
                (100, 0, false, 0)
            );
        }
    }
    #[test]
    fn silence_still_allows_an_ordinary_weapon_attack() {
        let mut s = State::new(1, Case::Silence, 2);
        s.begin();
        settle(&mut s);
        assert_eq!((s.shots, s.last_damage[1], s.energy), (1, 42, 100));
    }
}
