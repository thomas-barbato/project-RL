//! ANIM-S23: passive-feedback fixtures, never a campaign combat resolver.
use super::super::melee::actor;
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;

const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 4);
const IDS: [&str; 2] = ["BD04", "BD06"];
const TITLES: [&str; 2] = ["Visée stable", "Récupération d’énergie"];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const FRAMES: usize = 128;
const ACTION_FRAMES: [usize; 3] = [48, 72, 96];
const ENERGY: Color = Color::new(0.63, 0.36, 1.0, 1.0);
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
    Hover,
    Menu,
    Move,
    Forced,
    Teleport,
    Full,
    Limited,
    Familiar,
    Reflection,
    Counter,
    Decoy,
}
impl Case {
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Actions immobiles",
            Self::Quiet => "Attente sans action",
            Self::Wall => "Obstacle",
            Self::Hidden => "Cible cachée",
            Self::Hover => "Survol",
            Self::Menu => "Menu sans tour",
            Self::Move => "Déplacement",
            Self::Forced => "Déplacement forcé",
            Self::Teleport => "Changement magique de case",
            Self::Full => "Énergie pleine",
            Self::Limited => "Énergie presque pleine",
            Self::Familiar => "Élimination par un familier",
            Self::Reflection => "Renvoi exclu",
            Self::Counter => "Riposte automatique exclue",
            Self::Decoy => "Leurre exclu",
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
    gain_event: Option<f32>,
    resolved: bool,
    step: u16,
    turn: u16,
    charges: u16,
    energy: u16,
    gained: u16,
    interval: u16,
    kills: u16,
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
            gain_event: None,
            resolved: true,
            step: 0,
            turn: 0,
            charges: 0,
            energy: match case {
                Case::Full => 100,
                Case::Limited => 95,
                _ => 40,
            },
            gained: 0,
            interval: 0,
            kills: 0,
            player: PLAYER,
        }
    }
    fn map_index(&self) -> usize {
        usize::from(self.case == Case::Wall)
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn target(&self) -> GridPos {
        GridPos::new(4, 1)
    }
    fn contact(&self) -> bool {
        !matches!(self.case, Case::Wall | Case::Hidden)
    }
    fn eligible(&self) -> bool {
        !matches!(
            self.case,
            Case::Familiar | Case::Reflection | Case::Counter | Case::Decoy
        )
    }
    fn begin(&mut self) {
        if self.begun.is_none() {
            self.begun = Some(self.clock);
        }
    }
    fn action(&mut self) {
        if self.begun.is_none() {
            return;
        }
        self.gained = 0;
        if self.step == 3 && self.case == Case::Menu {
            self.event = None;
            return;
        }
        self.turn += 1;
        self.interval = self.interval.saturating_sub(1);
        if self.skill == 0
            && self.step == 3
            && matches!(self.case, Case::Move | Case::Forced | Case::Teleport)
        {
            self.charges = 0;
            self.player = GridPos::new(3, 4);
            self.event = None;
            return;
        }
        self.event = if self.contact() {
            Some(self.clock)
        } else {
            None
        };
        self.resolved = !self.contact();
        // The no-contact fixture still performs a stationary prepared action.
        if !self.contact() && self.skill == 0 {
            self.charges = (self.charges + 1).min(3);
        }
    }
    fn resolve(&mut self) {
        if self.resolved || self.age().is_none_or(|a| a < 0.12) {
            return;
        }
        self.resolved = true;
        if self.skill == 0 {
            self.charges = (self.charges + 1).min(3);
        } else {
            self.kills += 1;
            if self.eligible() && self.interval == 0 && self.energy < 100 {
                self.gained = (100 - self.energy).min(22);
                self.energy += self.gained;
                self.interval = 3;
                self.gain_event = Some(self.clock);
            }
        }
    }
    fn advance(&mut self) {
        self.step += 1;
        self.action();
    }
    fn age(&self) -> Option<f32> {
        self.event.map(|t| self.clock - t)
    }
    fn gain_age(&self) -> Option<f32> {
        self.gain_event.map(|t| self.clock - t)
    }
    fn icon(&self) -> bool {
        self.skill == 0 && self.charges > 0
    }
    fn dead(&self) -> bool {
        self.skill == 1 && self.kills > 0 && self.resolved
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
}
impl Art {
    fn load() -> Self {
        let load = |data: &[u8]| {
            let t = Texture2D::from_file_with_format(data, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            icons: [
                load(include_bytes!(
                    "../../../assets/skills/agent/BD04-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/BD06-compact.png"
                )),
            ],
            blood: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-40/blood-atlas.png"
            )),
        }
    }
    fn bodies(&self, s: &State, rect: Rect, effects: bool) {
        let p = point(s.player, s.direction, rect);
        let t = point(s.target(), s.direction, rect);
        if s.contact() {
            let recoil = if effects {
                crate::combat_visuals::hit_reaction_offset(
                    t,
                    p,
                    f64::from(s.age().unwrap_or(99.0) - 0.12),
                    64.0,
                )
            } else {
                Vec2::ZERO
            };
            actor(
                t + recoil,
                Visual::Bestiary {
                    number: 12,
                    corpse: s.dead(),
                },
            );
        }
        actor(
            p,
            game_art::wear(
                game_art::player(
                    Some("core:traqueur"),
                    PlayerWeaponPose::Ranged,
                    Some("core:modele_cogneur_p01"),
                ),
                [("core:body_armor".into(), "core:tenue_traque".into())],
            ),
        );
        enemy(point(GridPos::new(6, 6), s.direction, rect));
    }
    fn reticle(&self, s: &State, rect: Rect) {
        if s.skill != 0 || !s.contact() || s.charges == 0 {
            return;
        }
        let t = point(s.target(), s.direction, rect) + vec2(0.0, -8.0);
        let radius = 30.0 - s.charges as f32 * 5.0;
        for (dx, dy) in [(-1.0, -1.0), (-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)] {
            let q = t + vec2(dx, dy) * radius;
            for (width, col) in [
                (5.5, INK),
                (3.0, AIM),
                (0.8, Color::new(1.0, 0.76, 0.57, 1.0)),
            ] {
                draw_line(q.x, q.y, q.x - dx * 9.0, q.y, width, col);
                draw_line(q.x, q.y, q.x, q.y - dy * 9.0, width, col);
            }
        }
    }
    fn effects(&self, s: &State, rect: Rect) {
        let Some(age) = s.age() else {
            return;
        };
        let p = point(s.player, s.direction, rect) + vec2(0.0, -9.0);
        let t = point(s.target(), s.direction, rect) + vec2(0.0, -9.0);
        let d = (t - p).normalize();
        let n = vec2(-d.y, d.x);
        // Prepared physical shot; no new weapon pose, accuracy or damage resolver.
        if s.eligible() && (0.0..0.12).contains(&age) {
            let at = p + (t - p) * (age / 0.12);
            draw_line(at.x - d.x * 18.0, at.y - d.y * 18.0, at.x, at.y, 5.5, INK);
            draw_line(
                at.x - d.x * 15.0,
                at.y - d.y * 15.0,
                at.x,
                at.y,
                3.3,
                Color::new(0.88, 0.91, 0.92, 1.0),
            );
            if age < 0.08 {
                draw_triangle(
                    p + d * 8.0 - n * 6.0,
                    p + d * 21.0,
                    p + d * 7.0 + n * 6.0,
                    Color::new(1.0, 0.71, 0.34, 1.0),
                );
            }
        }
        if s.case != Case::Reflection && (0.12..0.42).contains(&age) {
            let f = (((age - 0.12) / 0.30 * 6.0) as usize).min(5);
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
    fn energy_feedback(&self, s: &State, rect: Rect) {
        let Some(age) = s.gain_age().filter(|a| *a < 0.55) else {
            return;
        };
        let target = point(s.target(), s.direction, rect) + vec2(0.0, -12.0);
        let end = vec2(rect.x + 453.0, rect.y + 36.0);
        let bend = (target + end) * 0.5 + vec2(20.0, -72.0);
        let pos = |u: f32| target * (1.0 - u).powi(2) + bend * 2.0 * u * (1.0 - u) + end * u * u;
        let u = (age / 0.34).min(1.0);
        let a = (1.0 - (age - 0.34).max(0.0) / 0.21).clamp(0.0, 1.0);
        for k in 0..10 {
            let hi = (u - k as f32 * 0.018).max(0.0);
            let lo = (hi - 0.018).max(0.0);
            let q = pos(hi);
            let r = pos(lo);
            let opacity = a * (1.0 - k as f32 / 12.0);
            draw_line(q.x, q.y, r.x, r.y, 9.0, alpha(INK, opacity));
            draw_line(q.x, q.y, r.x, r.y, 6.0, alpha(ENERGY, opacity));
            draw_line(
                q.x,
                q.y,
                r.x,
                r.y,
                1.9,
                alpha(Color::new(0.94, 0.86, 1.0, 1.0), opacity),
            );
        }
        let q = pos(u);
        for (size, col) in [(9.0, INK), (6.5, ENERGY), (2.5, WHITE)] {
            draw_triangle(
                q + vec2(0.0, -size),
                q + vec2(-size * 0.6, 0.0),
                q + vec2(size * 0.6, 0.0),
                alpha(col, a),
            );
            draw_triangle(
                q + vec2(0.0, size),
                q + vec2(-size * 0.6, 0.0),
                q + vec2(size * 0.6, 0.0),
                alpha(col, a),
            );
        }
        draw_text_bold(
            format!("+{} énergie", s.gained),
            rect.x + 359.0,
            rect.y + 68.0 - age * 12.0,
            18.0,
            alpha(Color::new(0.85, 0.69, 1.0, 1.0), a),
        );
    }
    fn hud(&self, s: &State, rect: Rect, interactive: bool) {
        if s.skill == 1 {
            draw_rectangle(rect.x + 350.0, rect.y + 20.0, 188.0, 29.0, alpha(INK, 0.95));
            draw_rectangle(
                rect.x + 353.0,
                rect.y + 23.0,
                182.0 * s.energy as f32 / 100.0,
                23.0,
                Color::new(0.32, 0.16, 0.58, 1.0),
            );
            draw_rectangle_lines(rect.x + 350.0, rect.y + 20.0, 188.0, 29.0, 1.2, DIM);
            draw_text_bold(
                format!("Énergie : {} / 100", s.energy),
                rect.x + 365.0,
                rect.y + 40.0,
                15.0,
                TEXT,
            );
            draw_texture_ex(
                &self.icons[1],
                rect.x + 315.0,
                rect.y + 19.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(30.0, 30.0)),
                    ..Default::default()
                },
            );
        }
        if !s.icon() {
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
        draw_text_bold(s.charges.to_string(), r.x + 26.0, r.y + 35.0, 13.0, TEXT);
        if s.case == Case::Hover
            || (interactive && r.contains(vec2(mouse_position().0, mouse_position().1)))
        {
            let x = rect.x + 291.0;
            let y = rect.y + 114.0;
            draw_rectangle(x, y, 267.0, 84.0, alpha(INK, 0.98));
            draw_rectangle_lines(x, y, 267.0, 84.0, 1.0, DIM);
            draw_text_bold(TITLES[0], x + 10.0, y + 22.0, 17.0, TEXT);
            draw_text(
                format!("Cumuls : {} / 3", s.charges),
                x + 10.0,
                y + 45.0,
                14.0,
                TEXT,
            );
            draw_text(
                format!("Précision à distance : +{} %", s.charges * 15),
                x + 10.0,
                y + 68.0,
                14.0,
                TEXT,
            );
        }
    }
}
fn scene(art: &Art, clip: &Clip, s: &State, rect: Rect, effects: bool, interactive: bool) {
    let map = s.map();
    terrain(&map, s.direction, rect, false);
    if s.begun.is_none() && s.contact() {
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
    art.hud(s, rect, interactive);
    // Resource transfer travels to the HUD, distinct from world damage effects.
    art.energy_feedback(s, rect);
    if let Some(age) = s.age().filter(|a| (0.12..0.54).contains(a)) {
        let t = point(s.target(), s.direction, rect);
        draw_text_bold(
            "20",
            t.x - 10.0,
            t.y - 47.0 - age * 14.0,
            19.0,
            alpha(TEXT, 1.0 - (age - 0.12) / 0.42),
        );
        let y = t.y - 35.0 - age * 14.0;
        let color = alpha(TEXT, 1.0 - (age - 0.12) / 0.42);
        draw_text("(", t.x - 19.0, y, 13.0, color);
        draw_line(t.x - 9.0, y - 1.0, t.x - 3.0, y - 7.0, 2.2, color);
        draw_line(t.x - 11.0, y - 4.0, t.x - 6.0, y + 1.0, 1.6, color);
        draw_text("20)", t.x + 2.0, y, 13.0, color);
    }
    draw_rectangle(
        rect.x + 5.0,
        rect.y + SIZE - 43.0,
        556.0,
        38.0,
        alpha(INK, 0.95),
    );
    draw_text(
        if s.skill == 0 {
            format!("Cumuls {} / 3 · précision +{} %", s.charges, s.charges * 15)
        } else {
            format!(
                "Énergie {} / 100 · gain {} · intervalle {} tours",
                s.energy, s.gained, s.interval
            )
        },
        rect.x + 12.0,
        rect.y + SIZE - 27.0,
        14.0,
        TEXT,
    );
    let log = if s.step == 3 && s.case == Case::Menu {
        "Menu sans tour : compteurs conservés."
    } else if s.skill == 0
        && s.step == 3
        && matches!(s.case, Case::Move | Case::Forced | Case::Teleport)
    {
        "Changement de case : cumuls supprimés."
    } else if s.case == Case::Wall {
        "Obstacle : aucun contact avec la cible."
    } else if s.case == Case::Hidden {
        "Cible cachée : aucun contact préparé."
    } else if !s.eligible() {
        s.case.title()
    } else if s.skill == 0 {
        "Bonus de précision ; aucune chance de toucher inventée."
    } else {
        "Éliminations préparées ; gain limité à l'énergie manquante."
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
        Case::Hover,
        Case::Menu,
        Case::Move,
        Case::Forced,
        Case::Teleport,
        Case::Full,
        Case::Limited,
        Case::Familiar,
        Case::Reflection,
        Case::Counter,
        Case::Decoy,
    ];
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Visée et énergie · ANIM-S23", 20.0, 33.0, 24.0, TEXT);
        for (i, t) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 88.0 + i as f32 * 42.0, 350.0, 34.0),
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
            s.begun.is_none(),
        ) {
            s.begin();
            s.action();
        }
        if button(
            Rect::new(20.0, 600.0, 350.0, 34.0),
            "Action suivante",
            s.begun.is_some() && s.resolved,
        ) || is_key_pressed(KeyCode::Space)
        {
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
            "Passifs · aucun lancement de compétence ni timer artificiel.",
            410.0,
            702.0,
            14.0,
            DIM,
        );
        draw_text(
            "Cas préparés ; effets de campagne inchangés.",
            410.0,
            726.0,
            14.0,
            DIM,
        );
        draw_text(
            "Monstres 32 px temporaires ; adaptation après les animations.",
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
    let target = render_target(640, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 640.0, 720.0))
    };
    let controls = [
        14, 17, 24, 26, 28, 31, 34, 38, 42, 44, 48, 52, 58, 68, 72, 76, 82, 92, 96, 100, 110, 126,
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
            (Case::Hover, 2, "SURVOL"),
            (Case::Menu, 2, "SANS-TOUR"),
        ];
        if skill == 0 {
            cases.extend([
                (Case::Move, 2, "DEPLACEMENT"),
                (Case::Forced, 2, "FORCE"),
                (Case::Teleport, 2, "MAGIQUE"),
            ]);
        } else {
            cases.extend([
                (Case::Full, 2, "PLEINE"),
                (Case::Limited, 2, "LIMITE"),
                (Case::Familiar, 2, "FAMILIER"),
                (Case::Reflection, 2, "RENVOI"),
                (Case::Counter, 2, "RIPOSTE"),
                (Case::Decoy, 2, "LEURRE"),
            ]);
        }
        for (case, direction, label) in cases {
            if std::env::args().any(|arg| arg == "--cas-visee-energie-menu") && case != Case::Menu {
                continue;
            }
            let folder = format!("{}-{label}", IDS[skill]);
            let dir = output.join(&folder);
            std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
            let mut s = State::new(skill, case, direction);
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
                }
                s.resolve();
                timeline.push(serde_json::json!({"frame":frame,"begun":s.begun.is_some(),"step":s.step,"turn":s.turn,"charges":s.charges,"bonus_percent":s.charges*15,"energy":s.energy,"gained":s.gained,"interval":s.interval,"kills":s.kills,"event_age":s.age(),"gain_age":s.gain_age(),"icon_visible":s.icon(),"dead":s.dead(),"player":[s.player.x,s.player.y]}));
                for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                    set_camera(&camera);
                    clear_background(INK);
                    draw_text_bold(
                        format!("{} · {}", 55 + skill, TITLES[skill]),
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
                        &s,
                        Rect::new(32.0, 88.0, SIZE, SIZE),
                        control == 0,
                        false,
                    );
                    draw_text(
                        if skill == 0 {
                            "Rang 11 · précision +15 % par cumul · maximum 3 cumuls"
                        } else {
                            "Rang 11 · gain 22 % de l'énergie max · intervalle 3 tours"
                        },
                        32.0,
                        684.0,
                        14.0,
                        TEXT,
                    );
                    draw_text(
                        "100 énergie max et frappe 20 préparées. Monstres 32 px temporaires.",
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
            entries.push(serde_json::json!({"folder":folder,"variant":format!("{}-A01-{label}",IDS[skill]),"id":IDS[skill],"number":55+skill,"title":TITLES[skill],"case":label,"case_title":case.title(),"direction":direction,"permitted":true,"quiet":case==Case::Quiet,"control_frames":controls,"action_frames":if case==Case::Quiet{vec![]}else{ACTION_FRAMES.to_vec()},"tile":64,"frames":FRAMES,"fps":24,"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n")}));
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
    fn resolve_action(s: &mut State) {
        s.action();
        s.clock += 0.2;
        s.resolve();
    }
    #[test]
    fn stationary_actions_gain_bounded_charges_but_idle_clock_does_not() {
        let mut s = State::new(0, Case::Open, 2);
        s.begin();
        for n in 1..=5 {
            resolve_action(&mut s);
            assert_eq!(s.charges, n.min(3));
        }
        s.clock += 999.0;
        s.resolve();
        assert_eq!(s.charges, 3);
    }
    #[test]
    fn movement_clears_and_untimed_menu_preserves() {
        for c in [Case::Move, Case::Forced, Case::Teleport, Case::Menu] {
            let mut s = State::new(0, c, 2);
            s.begin();
            for _ in 0..3 {
                resolve_action(&mut s);
            }
            let turn = s.turn;
            s.step = 3;
            s.action();
            assert_eq!(s.charges, if c == Case::Menu { 3 } else { 0 });
            assert_eq!(s.turn, turn + u16::from(c != Case::Menu));
        }
    }
    #[test]
    fn recovery_limits_and_exclusions() {
        for (c, expected) in [
            (Case::Open, 22),
            (Case::Limited, 5),
            (Case::Full, 0),
            (Case::Familiar, 0),
            (Case::Reflection, 0),
            (Case::Counter, 0),
            (Case::Decoy, 0),
        ] {
            let mut s = State::new(1, c, 2);
            s.begin();
            resolve_action(&mut s);
            assert_eq!(s.gained, expected);
            assert!(s.energy <= 100);
        }
    }
    #[test]
    fn interval_counts_actions_without_idle_replay() {
        let mut s = State::new(1, Case::Open, 2);
        s.begin();
        for expected in [22, 0, 0, 22] {
            resolve_action(&mut s);
            assert_eq!(s.gained, expected);
        }
        assert_eq!(s.energy, 84);
        s.clock += 999.0;
        s.resolve();
        assert_eq!(s.energy, 84);
        assert_eq!(s.interval, 3);
    }
    #[test]
    fn blocked_and_hidden_never_kill_or_gain() {
        for c in [Case::Wall, Case::Hidden] {
            let mut s = State::new(1, c, 2);
            s.begin();
            resolve_action(&mut s);
            assert_eq!(s.kills, 0);
            assert_eq!(s.energy, 40);
        }
    }
    #[test]
    fn untimed_menu_preserves_the_dead_victim_and_resource_state() {
        let mut s = State::new(1, Case::Menu, 2);
        s.begin();
        for _ in 0..3 {
            resolve_action(&mut s);
        }
        let before = (s.energy, s.interval, s.turn, s.kills);
        assert!(s.dead());
        s.step = 3;
        s.action();
        assert_eq!((s.energy, s.interval, s.turn, s.kills), before);
        assert!(s.dead());
    }
}
