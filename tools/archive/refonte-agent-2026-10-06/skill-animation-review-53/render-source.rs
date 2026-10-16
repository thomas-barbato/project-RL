//! ANIM-S24: prepared mark and ranged mastery; no campaign or accuracy solver.
use super::super::melee::{actor, cut};
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;
const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 4);
const IDS: [&str; 2] = ["T01", "T03"];
const TITLES: [&str; 2] = ["Marquage balistique", "Maîtrise des attaques à distance"];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const FRAMES: usize = 160;
const ACTION_FRAMES: [usize; 6] = [48, 68, 88, 108, 128, 148];
const AIM: Color = Color::new(1.0, 0.22, 0.19, 1.0);
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    Open,
    Quiet,
    Wall,
    Hidden,
    Melee,
    Silence,
    Transformed,
    Hover,
    Miss,
    Burst,
    Expiry,
    Reapply,
    Drone,
}
impl Case {
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Cible visible",
            Self::Quiet => "Attente sans action",
            Self::Wall => "Obstacle",
            Self::Hidden => "Cible cachée",
            Self::Melee => "Arme de mêlée active",
            Self::Silence => "Silence",
            Self::Transformed => "Forme transformée",
            Self::Hover => "Survol",
            Self::Miss => "Tir manqué",
            Self::Burst => "Rafale préparée",
            Self::Expiry => "Expiration sans tir",
            Self::Reapply => "Nouvelle cible",
            Self::Drone => "Tir d'un drone",
        }
    }
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    begun: Option<f32>,
    mark_event: Option<f32>,
    event: Option<f32>,
    step: u16,
    turn: u16,
    remaining: u16,
    cooldown: u16,
    energy: u16,
    damage: u16,
    bonus: u16,
    used_mark: bool,
    resolved: bool,
    hits: u16,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.0,
            begun: None,
            mark_event: None,
            event: None,
            step: 0,
            turn: 0,
            remaining: 0,
            cooldown: 0,
            energy: 100,
            damage: 0,
            bonus: 0,
            used_mark: false,
            resolved: true,
            hits: 0,
        }
    }
    fn map_index(&self) -> usize {
        usize::from(self.case == Case::Wall)
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn contact(&self) -> bool {
        !matches!(self.case, Case::Wall | Case::Hidden)
    }
    fn allowed(&self) -> bool {
        self.skill == 1
            || (self.contact()
                && !matches!(self.case, Case::Melee | Case::Silence | Case::Transformed))
    }
    fn target(&self) -> GridPos {
        if self.skill == 1 && self.case == Case::Melee {
            GridPos::new(4, 3)
        } else if self.case == Case::Reapply && self.step >= 3 {
            GridPos::new(6, 2)
        } else {
            GridPos::new(4, 1)
        }
    }
    fn marked(&self) -> bool {
        self.skill == 0 && self.remaining > 0
    }
    fn mark(&mut self) {
        if self.skill == 0 && self.allowed() && self.cooldown == 0 && self.energy >= 9 {
            self.remaining = 6;
            self.cooldown = 3;
            self.energy -= 9;
            self.mark_event = Some(self.clock);
        }
    }
    fn begin(&mut self) {
        if self.begun.is_none() && self.allowed() {
            self.begun = Some(self.clock);
            self.mark();
        }
    }
    fn shot(&mut self) {
        if self.begun.is_none() || !self.contact() {
            return;
        }
        self.used_mark = self.marked();
        self.remaining = 0;
        self.event = Some(self.clock);
        self.resolved = false;
        self.damage = 0;
        self.bonus = 0;
    }
    fn advance(&mut self) {
        if self.begun.is_none() {
            return;
        }
        self.step += 1;
        self.turn += 1;
        self.remaining = self.remaining.saturating_sub(1);
        self.cooldown = self.cooldown.saturating_sub(1);
        if self.case == Case::Expiry {
            if self.step == 6 {
                self.shot();
            }
        } else if self.case == Case::Reapply {
            if self.step == 3 {
                self.mark();
            }
        } else if self.skill == 1 || self.step == 1 {
            self.shot();
        }
    }
    fn age(&self) -> Option<f32> {
        self.event.map(|t| self.clock - t)
    }
    fn mark_age(&self) -> Option<f32> {
        self.mark_event.map(|t| self.clock - t)
    }
    fn burst(&self) -> bool {
        self.skill == 0 && self.case == Case::Burst
    }
    fn resolve(&mut self) {
        if self.resolved
            || self
                .age()
                .is_none_or(|a| a < if self.burst() { 0.26 } else { 0.12 })
        {
            return;
        }
        self.resolved = true;
        if self.case == Case::Miss {
            return;
        }
        self.bonus = if self.skill == 1 && !matches!(self.case, Case::Melee | Case::Drone) {
            8
        } else {
            0
        };
        self.damage = if self.burst() { 60 } else { 20 + self.bonus };
        self.hits += if self.burst() { 3 } else { 1 };
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
    predator: Texture2D,
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
                    "../../../assets/skills/agent/T01-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/T03-compact.png"
                )),
            ],
            blood: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-40/blood-atlas.png"
            )),
            predator: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-39/predator-64.png"
            )),
        }
    }
    fn origin(&self, s: &State, rect: Rect) -> Vec2 {
        point(
            if s.case == Case::Drone {
                GridPos::new(3, 4)
            } else {
                PLAYER
            },
            s.direction,
            rect,
        )
    }
    fn bodies(&self, s: &State, rect: Rect, effects: bool) {
        let p = point(PLAYER, s.direction, rect);
        let t = point(s.target(), s.direction, rect);
        if s.case != Case::Hidden {
            let offset = if effects && (s.marked() || s.case != Case::Miss) {
                crate::combat_visuals::hit_reaction_offset(
                    t,
                    self.origin(s, rect),
                    f64::from(if s.marked() {
                        s.mark_age().unwrap_or(99.0)
                    } else {
                        s.age().unwrap_or(99.0) - 0.12
                    }),
                    64.0,
                )
            } else {
                Vec2::ZERO
            };
            enemy(t + offset);
        }
        if s.case == Case::Reapply {
            enemy(point(
                if s.step >= 3 {
                    GridPos::new(4, 1)
                } else {
                    GridPos::new(6, 2)
                },
                s.direction,
                rect,
            ));
        }
        if s.case == Case::Transformed {
            draw_texture(&self.predator, p.x - 32.0, p.y - 32.0, WHITE);
        } else {
            actor(
                p,
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
                            "core:modele_cogneur_p01"
                        }),
                    ),
                    [("core:body_armor".into(), "core:tenue_traque".into())],
                ),
            );
        }
        if s.case == Case::Drone {
            let at = self.origin(s, rect);
            actor(
                at,
                Visual::Bestiary {
                    number: 58,
                    corpse: false,
                },
            );
            draw_circle(
                at.x + 22.0,
                at.y + 25.0,
                2.8,
                Color::new(0.2, 1.0, 0.3, 1.0),
            );
        }
        enemy(point(GridPos::new(6, 6), s.direction, rect));
    }
    fn reticle(&self, s: &State, rect: Rect) {
        if !s.marked() {
            return;
        }
        let t = point(s.target(), s.direction, rect) + vec2(0.0, -8.0);
        let u = (s.mark_age().unwrap_or(99.0) / 0.22).clamp(0.0, 1.0);
        let radius = 18.0 + 22.0 * (1.0 - u).powi(2);
        for (dx, dy) in [(-1.0, -1.0), (-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)] {
            let q = t + vec2(dx, dy) * radius;
            for (width, c) in [
                (6.0, INK),
                (3.3, AIM),
                (0.9, Color::new(1.0, 0.78, 0.55, 1.0)),
            ] {
                draw_line(q.x, q.y, q.x - dx * 10.0, q.y, width, c);
                draw_line(q.x, q.y, q.x, q.y - dy * 10.0, width, c);
            }
        }
    }
    fn effects(&self, s: &State, rect: Rect) {
        let Some(age) = s.age() else {
            return;
        };
        let p = self.origin(s, rect) + vec2(0.0, -9.0);
        let t = point(s.target(), s.direction, rect) + vec2(0.0, -9.0);
        let d = (t - p).normalize();
        let n = vec2(-d.y, d.x);
        for i in 0..if s.burst() { 3 } else { 1 } {
            let a = age - i as f32 * 0.07;
            if (0.0..0.12).contains(&a) && s.case != Case::Melee {
                let to = if s.case == Case::Miss {
                    t + n * 35.0
                } else {
                    t
                };
                let at = p + (to - p) * (a / 0.12);
                let bright = s.skill == 1 && s.case != Case::Drone;
                draw_line(
                    at.x - d.x * 19.0,
                    at.y - d.y * 19.0,
                    at.x,
                    at.y,
                    if bright { 7.0 } else { 5.5 },
                    INK,
                );
                draw_line(
                    at.x - d.x * 17.0,
                    at.y - d.y * 17.0,
                    at.x,
                    at.y,
                    if bright { 4.5 } else { 3.3 },
                    Color::new(0.88, 0.91, 0.92, 1.0),
                );
                if bright {
                    draw_line(at.x - d.x * 15.0, at.y - d.y * 15.0, at.x, at.y, 1.6, WHITE);
                }
                if a < 0.06 {
                    draw_triangle(
                        p + d * 8.0 - n * 6.0,
                        p + d * 21.0,
                        p + d * 7.0 + n * 6.0,
                        Color::new(1.0, 0.71, 0.34, 1.0),
                    );
                }
            }
            if s.case == Case::Melee && (0.0..0.20).contains(&a) {
                cut(
                    t - n * 27.0 - d * 12.0,
                    t + n * 28.0 + d * 10.0,
                    7.0,
                    1.0 - a / 0.20,
                );
            }
            if s.case != Case::Miss && (0.12..0.42).contains(&a) {
                let f = (((a - 0.12) / 0.30 * 6.0) as usize).min(5);
                draw_texture_ex(
                    &self.blood,
                    t.x - 31.0,
                    t.y - 25.0,
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
    }
    fn hud(&self, s: &State, rect: Rect, interactive: bool) {
        if !s.marked() {
            return;
        }
        let r = Rect::new(rect.x + 488.0, rect.y + 66.0, 38.0, 38.0);
        draw_rectangle(r.x, r.y, r.w, r.h, alpha(INK, 0.95));
        draw_rectangle_lines(r.x, r.y, r.w, r.h, 1.0, DIM);
        draw_texture_ex(
            &self.icons[0],
            r.x + 3.0,
            r.y + 3.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                ..Default::default()
            },
        );
        draw_rectangle(r.x + 23.0, r.y + 23.0, 14.0, 14.0, INK);
        draw_text_bold(s.remaining.to_string(), r.x + 26.0, r.y + 35.0, 13.0, TEXT);
        // Prepared target-analysis status strip; independent of reconnaissance access.
        draw_rectangle(rect.x + 12.0, rect.y + 12.0, 236.0, 56.0, alpha(INK, 0.97));
        draw_text(
            "Analyse · statut de la cible",
            rect.x + 20.0,
            rect.y + 29.0,
            13.0,
            DIM,
        );
        draw_texture_ex(
            &self.icons[0],
            rect.x + 20.0,
            rect.y + 36.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(24.0, 24.0)),
                ..Default::default()
            },
        );
        draw_text(
            format!("Marque : {} tours", s.remaining),
            rect.x + 51.0,
            rect.y + 53.0,
            14.0,
            TEXT,
        );
        if s.case == Case::Hover
            || (interactive && r.contains(vec2(mouse_position().0, mouse_position().1)))
        {
            let x = rect.x + 282.0;
            let y = rect.y + 114.0;
            draw_rectangle(x, y, 276.0, 84.0, alpha(INK, 0.98));
            draw_rectangle_lines(x, y, 276.0, 84.0, 1.0, DIM);
            draw_text_bold(TITLES[0], x + 10.0, y + 22.0, 17.0, TEXT);
            draw_text(
                "Précision contre la cible : ×1,35",
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
        }
    }
}
fn feedback(s: &State, rect: Rect) {
    let t = point(s.target(), s.direction, rect);
    let p = point(PLAYER, s.direction, rect);
    if let Some(a) = s.mark_age().filter(|a| *a < 0.32) {
        draw_text_bold(
            TITLES[0],
            p.x - 72.0,
            p.y - 45.0 - a * 8.0,
            15.0,
            alpha(TEXT, 1.0 - a / 0.32),
        );
    }
    if let Some(a) = s.age().filter(|a| (0.12..0.62).contains(a)) {
        let c = alpha(TEXT, 1.0 - (a - 0.12) / 0.50);
        let y = t.y - 47.0 - a * 14.0;
        if s.case == Case::Miss {
            draw_text_bold("Manqué", t.x - 28.0, y, 18.0, c);
        } else if s.damage > 0 {
            draw_text_bold(
                if s.skill == 1 {
                    format!("{} (+{})", s.damage, s.bonus)
                } else {
                    s.damage.to_string()
                },
                t.x - if s.skill == 1 { 39.0 } else { 10.0 },
                y,
                19.0,
                c,
            );
            draw_text("(", t.x - 19.0, y + 12.0, 13.0, c);
            draw_line(t.x - 9.0, y + 11.0, t.x - 3.0, y + 5.0, 2.2, c);
            draw_line(t.x - 11.0, y + 8.0, t.x - 6.0, y + 13.0, 1.6, c);
            draw_text(format!("{})", s.damage), t.x + 2.0, y + 12.0, 13.0, c);
        }
    }
}
fn scene(art: &Art, clip: &Clip, s: &State, rect: Rect, effects: bool, interactive: bool) {
    let map = s.map();
    terrain(&map, s.direction, rect, false);
    if s.begun.is_none() && s.allowed() && s.contact() {
        preview(&[s.target()], s.direction, rect, s.clock);
    }
    terrain(&map, s.direction, rect, true);
    clip.apply_for(s.map_index(), s.direction, rect);
    art.bodies(s, rect, effects);
    if effects {
        art.effects(s, rect);
        art.reticle(s, rect);
    }
    gl_use_default_material();
    feedback(s, rect);
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
                "Marque {} tours · recharge {} · énergie {}",
                s.remaining, s.cooldown, s.energy
            )
        } else {
            format!("Bonus permanent +40 % · frappe {} (+{})", s.damage, s.bonus)
        },
        rect.x + 12.0,
        rect.y + SIZE - 27.0,
        14.0,
        TEXT,
    );
    let log = if !s.allowed() {
        match s.case {
            Case::Melee => "Arme à distance équipée et active requise.",
            Case::Silence => "Silence empêche le marquage.",
            Case::Transformed => "Forme humaine requise pour le marquage.",
            _ => "Cible visible et accessible requise.",
        }
    } else if s.skill == 1 && matches!(s.case, Case::Melee | Case::Drone) {
        "Cette attaque ne reçoit pas le bonus de maîtrise."
    } else if s.case == Case::Expiry && s.step == 6 {
        "Marque expirée : le tir ne reçoit aucun bonus de précision."
    } else if s.used_mark {
        "Marque consommée · précision du tir préparé : ×1,35."
    } else {
        "Valeurs et contacts préparés ; aucun jet de précision réel."
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
    let mut s = State::new(0, Case::Open, 2);
    let mut paused = false;
    let cases = [
        Case::Open,
        Case::Quiet,
        Case::Wall,
        Case::Hidden,
        Case::Melee,
        Case::Silence,
        Case::Transformed,
        Case::Hover,
        Case::Miss,
        Case::Burst,
        Case::Expiry,
        Case::Reapply,
        Case::Drone,
    ];
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Marquage et maîtrise · ANIM-S24", 20.0, 33.0, 24.0, TEXT);
        for (i, t) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 88.0 + i as f32 * 42.0, 380.0, 34.0),
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
                    185.0 + (i / 2) as f32 * 37.0,
                    166.0,
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
                    20.0 + (i % 2) as f32 * 190.0,
                    278.0 + (i / 2) as f32 * 33.0,
                    182.0,
                    28.0,
                ),
                c.title(),
                true,
            ) {
                s = State::new(s.skill, *c, s.direction);
            }
        }
        if button(
            Rect::new(20.0, 558.0, 350.0, 34.0),
            "Commencer l'aperçu",
            s.begun.is_none() && s.allowed(),
        ) {
            s.begin();
            if s.skill == 1 {
                s.turn += 1;
                s.shot();
            }
        }
        if button(
            Rect::new(20.0, 600.0, 350.0, 34.0),
            "Action suivante",
            s.begun.is_some() && s.resolved && s.step < 6,
        ) {
            s.advance();
        }
        if button(Rect::new(20.0, 642.0, 160.0, 34.0), "Recommencer", true)
            || is_key_pressed(KeyCode::R)
        {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(190.0, 642.0, 160.0, 34.0),
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
            "Contacts préparés · aucune probabilité de toucher ajoutée.",
            410.0,
            702.0,
            14.0,
            DIM,
        );
        draw_text(
            "Monstres et drones 32 px temporaires ; adaptation ensuite.",
            410.0,
            726.0,
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
        14, 17, 20, 24, 28, 34, 44, 48, 50, 52, 55, 60, 64, 68, 72, 84, 88, 92, 104, 108, 128, 148,
        152, 158,
    ];
    let mut entries = Vec::new();
    for skill in 0..2 {
        if skill == 1 && std::env::args().any(|arg| arg == "--cas-marquage-seul") {
            continue;
        }
        let mut cases = vec![
            (Case::Open, 2, "HAUT"),
            (Case::Open, 0, "DROITE"),
            (Case::Open, 1, "GAUCHE"),
            (Case::Open, 3, "BAS"),
            (Case::Quiet, 2, "ATTENTE"),
            (Case::Wall, 2, "CLOISON"),
            (Case::Hidden, 2, "CACHE"),
            (Case::Hover, 2, "SURVOL"),
        ];
        if skill == 0 {
            cases.extend([
                (Case::Melee, 2, "MELEE"),
                (Case::Silence, 2, "SILENCE"),
                (Case::Transformed, 2, "TRANSFORME"),
                (Case::Miss, 2, "MANQUE"),
                (Case::Burst, 2, "RAFALE"),
                (Case::Expiry, 2, "EXPIRATION"),
                (Case::Reapply, 2, "REMPLACEMENT"),
            ]);
        } else {
            cases.extend([(Case::Melee, 2, "MELEE"), (Case::Drone, 2, "DRONE")]);
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
                if frame == 24 && skill == 1 {
                    s.turn += 1;
                    s.shot();
                }
                if case != Case::Quiet && ACTION_FRAMES.contains(&frame) {
                    s.advance();
                }
                s.resolve();
                timeline.push(serde_json::json!({"frame":frame,"begun":s.begun.is_some(),"step":s.step,"turn":s.turn,"remaining":s.remaining,"cooldown":s.cooldown,"energy":s.energy,"damage":s.damage,"bonus":s.bonus,"used_mark":s.used_mark,"precision_percent":if s.used_mark{135}else{100},"hits":s.hits,"event_age":s.age(),"icon_visible":s.marked(),"marked_target":if s.marked(){Some([s.target().x,s.target().y])}else{None}}));
                for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                    set_camera(&camera);
                    clear_background(INK);
                    draw_text_bold(
                        format!("{} · {}", 57 + skill, TITLES[skill]),
                        32.0,
                        35.0,
                        if skill == 0 { 23.0 } else { 21.0 },
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
                        &s,
                        Rect::new(32.0, 88.0, SIZE, SIZE),
                        control == 0,
                        false,
                    );
                    draw_text(
                        if skill == 0 {
                            "Rang 11 · précision ×1,35 · durée max 6 · énergie 9 · recharge 3"
                        } else {
                            "Rang 11 · dégâts à distance +40 % · bonus permanent"
                        },
                        32.0,
                        684.0,
                        14.0,
                        TEXT,
                    );
                    draw_text(
                        "Frappe 20 préparée ; aucun équilibrage ni jet de précision réel.",
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
            entries.push(serde_json::json!({"folder":folder,"variant":format!("{}-A01-{label}",IDS[skill]),"id":IDS[skill],"number":57+skill,"title":TITLES[skill],"case":label,"case_title":case.title(),"direction":direction,"permitted":permitted,"quiet":case==Case::Quiet,"control_frames":controls,"action_frames":if case==Case::Quiet{vec![]}else{ACTION_FRAMES.to_vec()},"tile":64,"frames":FRAMES,"fps":24,"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n")}));
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
    fn mark_is_instant_and_idle_time_preserves_duration() {
        let mut s = State::new(0, Case::Open, 2);
        s.begin();
        assert_eq!((s.turn, s.energy, s.remaining, s.cooldown), (0, 91, 6, 3));
        s.clock = 999.0;
        s.resolve();
        assert!(s.marked());
        assert_eq!(s.remaining, 6);
    }
    #[test]
    fn next_shot_consumes_even_on_miss_and_carries_precision_snapshot() {
        for c in [Case::Open, Case::Miss, Case::Burst] {
            let mut s = State::new(0, c, 2);
            s.begin();
            s.advance();
            assert_eq!(s.remaining, 0);
            assert!(s.used_mark);
            s.clock += 0.3;
            s.resolve();
            assert_eq!(
                s.damage,
                if c == Case::Miss {
                    0
                } else if c == Case::Burst {
                    60
                } else {
                    20
                }
            );
        }
    }
    #[test]
    fn blocked_casts_are_atomic_in_the_fixture() {
        for c in [
            Case::Wall,
            Case::Hidden,
            Case::Melee,
            Case::Silence,
            Case::Transformed,
        ] {
            let mut s = State::new(0, c, 2);
            s.begin();
            assert!(s.begun.is_none());
            assert_eq!((s.energy, s.cooldown, s.remaining), (100, 0, 0));
        }
    }
    #[test]
    fn expiry_never_applies_mark_to_later_shot() {
        let mut s = State::new(0, Case::Expiry, 2);
        s.begin();
        for _ in 0..6 {
            s.advance();
        }
        assert_eq!((s.remaining, s.cooldown, s.turn), (0, 0, 6));
        assert!(!s.used_mark);
        s.clock += 0.2;
        s.resolve();
        assert_eq!(s.damage, 20);
    }
    #[test]
    fn replacement_marks_exactly_one_new_target() {
        let mut s = State::new(0, Case::Reapply, 2);
        s.begin();
        for _ in 0..3 {
            s.advance();
        }
        assert_eq!(s.target(), GridPos::new(6, 2));
        assert_eq!((s.remaining, s.cooldown, s.energy), (6, 3, 82));
        assert_eq!(s.turn, 3);
    }
    #[test]
    fn permanent_mastery_has_no_status_or_timer_and_excludes_melee_and_drones() {
        for (c, damage, bonus) in [
            (Case::Open, 28, 8),
            (Case::Melee, 20, 0),
            (Case::Drone, 20, 0),
        ] {
            let mut s = State::new(1, c, 2);
            s.begin();
            s.shot();
            s.clock += 0.2;
            s.resolve();
            assert_eq!((s.damage, s.bonus), (damage, bonus));
            assert!(!s.marked());
            assert_eq!((s.remaining, s.cooldown, s.energy), (0, 0, 100));
            s.clock += 999.0;
            s.resolve();
            assert_eq!(s.hits, 1);
        }
    }
}
