//! ANIM-REACTION01: prepared Cuirasse/Possede actions; no real combat or resources.
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::{Map, has_line_of_sight};

const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 4);
const CONTACT: GridPos = GridPos::new(4, 3);
const DISTANT: GridPos = GridPos::new(4, 1);
const IDS: [&str; 6] = [
    "TR63-HEURT",
    "TR63-DECHARGE",
    "TR63-FERMETURE",
    "TR64-TRAIT",
    "TR64-PULSATION",
    "TR64-DRAIN",
];
const TITLES: [&str; 6] = [
    "Heurt",
    "Décharge de plaques",
    "Fermeture des plaques",
    "Trait corporel",
    "Pulsation occulte",
    "Drain organique",
];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const MAPS: [&str; 3] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#....#..#\n#....#..#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#########\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
];
const TURNS: [usize; 3] = [48, 66, 84];

struct State {
    skill: usize,
    map: usize,
    direction: usize,
    clock: f32,
    started: Option<f32>,
    turn: u16,
    cooldown: u16,
    closed: u16,
    reopened: Option<f32>,
    hits: Vec<GridPos>,
    resolutions: u16,
    charged: bool,
    was_charged: bool,
    acting: bool,
    silenced: bool,
    moved: Option<f32>,
    extra_hit: Option<f32>,
    player: GridPos,
}
impl State {
    fn new(skill: usize, map: usize, direction: usize, scenario: &str) -> Self {
        Self {
            skill,
            map,
            direction,
            clock: 0.0,
            started: None,
            turn: 0,
            cooldown: 0,
            closed: 0,
            reopened: None,
            hits: Vec::new(),
            resolutions: 0,
            charged: skill == 1 && scenario == "RESERVE",
            was_charged: false,
            acting: skill == 2 && scenario == "AGIR",
            silenced: skill >= 3 && scenario == "SILENCE",
            moved: None,
            extra_hit: None,
            player: PLAYER,
        }
    }
    fn target(&self) -> GridPos {
        if self.skill < 3 { CONTACT } else { DISTANT }
    }
    fn monsters(&self) -> Vec<GridPos> {
        let map = Map::from_ascii(MAPS[self.map]).unwrap();
        [
            GridPos::new(3, 3),
            CONTACT,
            GridPos::new(5, 3),
            GridPos::new(3, 4),
            GridPos::new(5, 4),
            GridPos::new(4, 5),
            GridPos::new(4, 2),
            DISTANT,
            GridPos::new(6, 4),
        ]
        .into_iter()
        .filter(|p| {
            map.is_walkable(*p)
                && !([3, 5].contains(&self.skill) && [CONTACT, GridPos::new(4, 2)].contains(p))
        })
        .collect()
    }
    fn footprint(&self) -> Vec<GridPos> {
        let map = Map::from_ascii(MAPS[self.map]).unwrap();
        let cells = match self.skill {
            2 => vec![self.player],
            1 => (-1_i32..=1)
                .flat_map(|dy| (-1..=1).map(move |dx| GridPos::new(PLAYER.x + dx, PLAYER.y + dy)))
                .filter(|p| *p != PLAYER)
                .collect(),
            4 => (-2_i32..=2)
                .flat_map(|dy| (-2..=2).map(move |dx| GridPos::new(PLAYER.x + dx, PLAYER.y + dy)))
                .filter(|p| *p != PLAYER && (p.x - PLAYER.x).pow(2) + (p.y - PLAYER.y).pow(2) <= 4)
                .collect(),
            _ => vec![self.target()],
        };
        cells
            .into_iter()
            .filter(|p| map.is_walkable(*p) && has_line_of_sight(&map, self.player, *p, true))
            .collect()
    }
    fn can_launch(&self) -> bool {
        self.started.is_none()
            && self.cooldown == 0
            && !(self.silenced && [4, 5].contains(&self.skill))
            && !self.footprint().is_empty()
    }
    fn launch(&mut self) {
        if !self.can_launch() {
            return;
        }
        self.started = Some(self.clock);
        self.cooldown = [0, 4, 6, 0, 4, 4][self.skill];
        self.resolutions = 1;
        if self.skill == 2 {
            self.closed = 3;
            return;
        }
        let cells = self.footprint();
        self.hits = self
            .monsters()
            .into_iter()
            .filter(|p| cells.contains(p))
            .collect();
        if self.skill == 1 {
            self.was_charged = self.charged;
            self.charged = false;
        }
    }
    fn advance(&mut self) {
        if self.started.is_none() {
            return;
        }
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        if self.closed > 0 {
            self.closed -= 1;
            if self.closed == 0 {
                self.reopened = Some(self.clock);
            }
        }
        if self.acting && self.turn == 1 {
            self.player = GridPos::new(3, 5);
            self.moved = Some(self.clock);
        }
        if self.acting && self.turn == 2 {
            self.extra_hit = Some(self.clock);
        }
    }
    fn age(&self) -> Option<f32> {
        self.started.map(|t| self.clock - t)
    }
    fn body_point(&self, rect: Rect) -> Vec2 {
        let mut p = point(self.player, self.direction, rect);
        if let Some(t) = self.moved {
            p = point(PLAYER, self.direction, rect)
                .lerp(p, ((self.clock - t) / 0.18).clamp(0.0, 1.0));
        }
        let age = if self.skill == 0 {
            self.age()
        } else {
            self.extra_hit.map(|t| self.clock - t)
        };
        if let Some(age) = age {
            if age < 0.22 {
                let to = if self.extra_hit.is_some() {
                    GridPos::new(3, 4)
                } else {
                    self.target()
                };
                p += (point(to, self.direction, rect) - p).normalize_or_zero()
                    * (age / 0.22 * std::f32::consts::PI).sin()
                    * 8.0;
            }
        }
        p
    }
    fn monster_point(&self, at: GridPos, rect: Rect, reactions: bool) -> Vec2 {
        let p = point(at, self.direction, rect);
        if !reactions {
            return p;
        }
        let age = if at == GridPos::new(3, 4) && self.extra_hit.is_some() {
            self.extra_hit.map(|t| self.clock - t - 0.02)
        } else if self.hits.contains(&at) {
            self.age()
                .map(|t| t - [0.02, 0.04, 0.0, 0.16, 0.01, 0.01][self.skill])
        } else {
            None
        };
        p + age.map_or(Vec2::ZERO, |age| {
            crate::combat_visuals::hit_reaction_offset(
                p,
                point(self.player, self.direction, rect),
                f64::from(age),
                64.0,
            )
        })
    }
    fn closure(&self) -> f32 {
        if self.closed > 0 {
            ((self.clock - self.started.unwrap()) / 0.16).clamp(0.0, 1.0)
        } else if let Some(t) = self.reopened {
            (1.0 - (self.clock - t) / 0.16).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
}
fn point(at: GridPos, direction: usize, rect: Rect) -> Vec2 {
    let p = position(at, direction);
    cell(rect, p.x as f32 + 0.5, p.y as f32 + 0.5)
}
fn angle(direction: usize) -> f32 {
    [
        0.0,
        std::f32::consts::PI,
        -std::f32::consts::FRAC_PI_2,
        std::f32::consts::FRAC_PI_2,
    ][direction]
}

struct Art {
    bodies: [Texture2D; 2],
    closed: Texture2D,
    impact: Texture2D,
    blood: Texture2D,
    plates: Texture2D,
    bolt: Texture2D,
    pulse: Texture2D,
    ray: Texture2D,
}
impl Art {
    fn load() -> Self {
        let load = |data: &[u8]| {
            let t = Texture2D::from_file_with_format(data, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            bodies: [
                load(include_bytes!(
                    "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-41/cuirasse-64.png"
                )),
                load(include_bytes!(
                    "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-41/possede-64.png"
                )),
            ],
            closed: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-42/closed-64.png"
            )),
            impact: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-40/impact-atlas.png"
            )),
            blood: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-40/blood-atlas.png"
            )),
            plates: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-42/plates-atlas.png"
            )),
            bolt: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-42/bolt-atlas.png"
            )),
            pulse: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-42/pulse-atlas.png"
            )),
            ray: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-33/ray-atlas.png"
            )),
        }
    }
    fn draw(t: &Texture2D, frame: usize, at: Vec2, size: Vec2, rotation: f32, opacity: f32) {
        draw_texture_ex(
            t,
            at.x - size.x * 0.5,
            at.y - size.y * 0.5,
            alpha(WHITE, opacity),
            DrawTextureParams {
                source: Some(Rect::new(frame.min(5) as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(size),
                rotation,
                pivot: Some(at),
                ..Default::default()
            },
        );
    }
    fn physical(&self, at: Vec2, age: f32, direction: usize, weight: f32) {
        if (0.02..0.34).contains(&age) {
            Self::draw(
                &self.impact,
                (((age - 0.02) / 0.32) * 6.0) as usize,
                at,
                vec2(96.0, 88.0) * weight,
                0.0,
                1.0,
            );
        }
        if (0.05..0.49).contains(&age) {
            let a = angle(direction);
            Self::draw(
                &self.blood,
                (((age - 0.05) / 0.44) * 6.0) as usize,
                at + vec2(a.cos(), a.sin()) * 22.0,
                vec2(76.0, 76.0) * weight,
                a,
                1.0,
            );
        }
    }
    fn ground(&self, s: &State, rect: Rect) {
        let Some(age) = s.age() else {
            return;
        };
        let p = point(PLAYER, s.direction, rect);
        if s.skill == 1 && (0.01..0.39).contains(&age) {
            Self::draw(
                &self.plates,
                (((age - 0.01) / 0.38) * 6.0) as usize,
                p + vec2(0.0, 12.0),
                vec2(230.0, 206.0),
                0.0,
                if s.was_charged { 1.0 } else { 0.82 },
            );
        }
        if s.skill == 4 && (0.01..0.41).contains(&age) {
            Self::draw(
                &self.pulse,
                (((age - 0.01) / 0.40) * 6.0) as usize,
                p,
                vec2(340.0, 324.0),
                0.0,
                1.0,
            );
        }
        if s.skill == 5 && (0.01..0.29).contains(&age) {
            let victim = s.monster_point(DISTANT, rect, true);
            let delta = p - victim;
            let mid = (p + victim) * 0.5;
            let a = delta.y.atan2(delta.x);
            let frame = ((age * 11.0) as usize) % 3;
            let fade = ((0.29 - age) / 0.09).clamp(0.0, 1.0);
            Self::draw(
                &self.ray,
                frame,
                mid,
                vec2(delta.length() * 256.0 / 224.0, 150.0),
                a,
                fade * 0.35,
            );
            Self::draw(
                &self.ray,
                frame,
                mid,
                vec2(delta.length() * 256.0 / 224.0, 112.0),
                a,
                fade,
            );
        }
    }
    fn front(&self, s: &State, rect: Rect) {
        let Some(age) = s.age() else {
            return;
        };
        if s.skill == 0 {
            self.physical(s.monster_point(CONTACT, rect, true), age, s.direction, 1.0);
        }
        if s.skill == 1 && (0.04..0.48).contains(&age) {
            for &at in &s.hits {
                let a = angle(s.direction);
                Self::draw(
                    &self.blood,
                    (((age - 0.04) / 0.44) * 6.0) as usize,
                    s.monster_point(at, rect, true) + vec2(a.cos(), a.sin()) * 18.0,
                    vec2(65.0, 65.0),
                    a,
                    0.85,
                );
            }
        }
        if let Some(t) = s.extra_hit {
            self.physical(
                s.monster_point(GridPos::new(3, 4), rect, true),
                s.clock - t,
                s.direction,
                1.0,
            );
        }
        if s.skill == 3 {
            let caster = point(PLAYER, s.direction, rect) - vec2(0.0, 6.0);
            let target = s.monster_point(DISTANT, rect, true) - vec2(0.0, 6.0);
            let d = (target - caster).normalize_or_zero();
            if (0.0..0.16).contains(&age) {
                Self::draw(
                    &self.bolt,
                    ((age / 0.16 * 3.0) as usize).min(2),
                    caster.lerp(target, age / 0.16) - d * 28.0,
                    vec2(64.0, 48.0),
                    angle(s.direction),
                    1.0,
                );
            } else if (0.16..0.36).contains(&age) {
                Self::draw(
                    &self.bolt,
                    3 + (((age - 0.16) / 0.20 * 3.0) as usize).min(2),
                    target,
                    vec2(78.0, 78.0),
                    angle(s.direction),
                    1.0,
                );
            }
        }
        if s.skill == 5 && (0.04..0.40).contains(&age) {
            let victim = s.monster_point(DISTANT, rect, true) - vec2(0.0, 6.0);
            let caster = point(PLAYER, s.direction, rect) - vec2(0.0, 6.0);
            let delta = caster - victim;
            let a = delta.y.atan2(delta.x);
            let u = (age - 0.04) / 0.36;
            let fade = (u / 0.08).clamp(0.0, 1.0) * ((1.0 - u) / 0.12).clamp(0.0, 1.0);
            for i in (0..4).rev() {
                let lag = i as f32 * 0.12;
                if u < lag {
                    continue;
                }
                let q = (u - lag).clamp(0.0, 1.0);
                Self::draw(
                    &self.ray,
                    3 + ((u * 3.0 + i as f32 * 0.4) as usize % 3),
                    victim + delta * q,
                    vec2(84.0, 100.0) * (1.0 - i as f32 * 0.06),
                    a,
                    fade,
                );
            }
        }
    }
    fn body(&self, s: &State, rect: Rect, effects: bool) {
        let p = s.body_point(rect);
        let close = if effects { s.closure() } else { 0.0 };
        draw_ellipse(
            p.x,
            p.y + 29.0,
            if s.skill < 3 { 23.0 } else { 18.0 },
            4.0,
            0.0,
            alpha(BLACK, 0.3),
        );
        draw_texture_ex(
            &self.bodies[usize::from(s.skill >= 3)],
            p.x - 32.0,
            p.y - 32.0,
            alpha(WHITE, 1.0 - close),
            DrawTextureParams {
                dest_size: Some(vec2(64.0, 64.0)),
                ..Default::default()
            },
        );
        if close > 0.0 {
            draw_texture_ex(
                &self.closed,
                p.x - 32.0,
                p.y - 32.0,
                alpha(WHITE, close),
                DrawTextureParams {
                    dest_size: Some(vec2(64.0, 64.0)),
                    ..Default::default()
                },
            );
        }
        if effects && s.charged {
            Self::draw(
                &self.plates,
                5,
                p,
                vec2(104.0, 106.0),
                0.0,
                0.45 + 0.20 * (s.clock * 5.0).sin(),
            );
        }
    }
}

fn scene(art: &Art, clip: &Clip, s: &State, rect: Rect, effects: bool) {
    let map = Map::from_ascii(MAPS[s.map]).unwrap();
    terrain(&map, s.direction, rect, false);
    if s.started.is_none() {
        preview(&s.footprint(), s.direction, rect, s.clock);
    }
    if effects {
        clip.apply_for(s.map, s.direction, rect);
        art.ground(s, rect);
        gl_use_default_material();
    }
    terrain(&map, s.direction, rect, true);
    clip.apply_for(s.map, s.direction, rect);
    for at in s.monsters() {
        enemy(s.monster_point(at, rect, effects));
    }
    gl_use_default_material();
    art.body(s, rect, effects);
    if effects {
        clip.apply_for(s.map, s.direction, rect);
        art.front(s, rect);
        gl_use_default_material();
    }
    if let Some(age) = s.age() {
        if age < 0.65 {
            let p = s.body_point(rect);
            draw_text_bold(
                TITLES[s.skill],
                p.x - 68.0,
                p.y - 47.0 - age * 7.0,
                15.0,
                alpha(TEXT, 1.0 - age / 0.65),
            );
        }
    }
    draw_rectangle(
        rect.x + 5.0,
        rect.y + SIZE - 27.0,
        550.0,
        22.0,
        alpha(INK, 0.92),
    );
    let label = if s.started.is_none() && !s.can_launch() {
        if s.silenced {
            "Silence : pouvoir indisponible".into()
        } else {
            "Trajet bloqué".into()
        }
    } else {
        format!(
            "Recharge {} · protection {} tours{}",
            s.cooldown,
            s.closed,
            if s.charged {
                " · réserve préparée"
            } else if s.was_charged {
                " · réserve vidée"
            } else {
                ""
            }
        )
    };
    draw_text(label, rect.x + 12.0, rect.y + SIZE - 12.0, 14.0, TEXT);
    if effects && s.closed > 0 {
        draw_texture_ex(
            &art.closed,
            rect.x + SIZE - 99.0,
            rect.y + SIZE - 112.0,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(20.0, 18.0, 24.0, 24.0)),
                dest_size: Some(vec2(28.0, 28.0)),
                ..Default::default()
            },
        );
        draw_text(
            s.closed.to_string(),
            rect.x + SIZE - 85.0,
            rect.y + SIZE - 85.0,
            12.0,
            TEXT,
        );
    }
}

pub(crate) async fn run() {
    request_new_screen_size(1280.0, 820.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let clip = Clip::load_maps(&MAPS, PLAYER);
    let mut s = State::new(0, 0, 2, "HAUT");
    let mut scenario = "HAUT";
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(INK);
        draw_text_bold(
            "Cuirassé / Possédé · ANIM-REACTION01",
            20.0,
            33.0,
            23.0,
            TEXT,
        );
        for (i, title) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 75.0 + i as f32 * 37.0, 320.0, 31.0),
                title,
                true,
            ) {
                s = State::new(i, s.map, s.direction, scenario);
            }
        }
        for (i, title) in DIRS.iter().enumerate() {
            if button(
                Rect::new(20.0, 312.0 + i as f32 * 32.0, 320.0, 27.0),
                title,
                true,
            ) {
                s = State::new(s.skill, s.map, i, scenario);
            }
        }
        for (i, (name, map, case)) in [
            ("Pièce ouverte", 0, "HAUT"),
            ("Obstacle partiel", 1, "OBSTACLE"),
            ("Cloison", 2, "BLOQUE"),
            ("Réserve préparée", 0, "RESERVE"),
            ("Protection : agir", 0, "AGIR"),
            ("Possédé silencé", 0, "SILENCE"),
        ]
        .iter()
        .enumerate()
        {
            if button(
                Rect::new(20.0, 451.0 + i as f32 * 32.0, 320.0, 27.0),
                name,
                true,
            ) {
                scenario = case;
                s = State::new(s.skill, *map, s.direction, scenario);
            }
        }
        if button(
            Rect::new(20.0, 660.0, 151.0, 33.0),
            "Lancer",
            s.can_launch(),
        ) || (is_key_pressed(KeyCode::Enter) && s.can_launch())
        {
            s.launch();
            paused = false;
        }
        if button(
            Rect::new(182.0, 660.0, 158.0, 33.0),
            "+1 tour",
            s.started.is_some(),
        ) || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(20.0, 705.0, 151.0, 33.0), "Nouvel aperçu", true) {
            s = State::new(s.skill, s.map, s.direction, scenario);
        }
        if button(
            Rect::new(182.0, 705.0, 158.0, 33.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        draw_text(
            "Événements préparés, aucune résolution de PV.",
            20.0,
            781.0,
            12.0,
            DIM,
        );
        scene(&art, &clip, &s, Rect::new(410.0, 88.0, SIZE, SIZE), true);
        draw_text(
            format!("{} / {}-A02 · tour {}", 37 + s.skill, IDS[s.skill], s.turn),
            410.0,
            695.0,
            14.0,
            DIM,
        );
        if !paused {
            s.clock += get_frame_time();
        }
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
    let target = render_target(1280, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1280.0, 720.0))
    };
    let controls = [
        3, 14, 17, 19, 21, 23, 25, 27, 30, 35, 47, 48, 52, 65, 66, 70, 72, 83, 84, 89, 95,
    ];
    let mut common_entries = Vec::new();
    let control_only = std::env::args().any(|arg| arg == "--capture-reaction-controls-only");
    for (case, label) in [
        ("COMMON-DAMAGE", "Dégâts"),
        ("COMMON-STATUS", "Effet négatif sans dégâts"),
    ] {
        let folder = output.join(case);
        std::fs::create_dir(&folder).map_err(|e| e.to_string())?;
        for frame in 0..96 {
            let age = (frame as f32 - if frame < 48 { 17.0 } else { 48.0 }) / 24.0;
            for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                if control_only && control == 0 && ![3, 19, 23, 52].contains(&frame) {
                    continue;
                }
                set_camera(&camera);
                clear_background(INK);
                for side in 0..2 {
                    let rect = Rect::new(32.0 + side as f32 * 616.0, 88.0, SIZE, SIZE);
                    let map = Map::from_ascii(MAPS[0]).unwrap();
                    terrain(&map, 2, rect, false);
                    terrain(&map, 2, rect, true);
                    let anchor = point(PLAYER, 2, rect);
                    let recoil = if control == 0 {
                        crate::combat_visuals::hit_reaction_offset(
                            anchor,
                            anchor + vec2(0.0, -64.0),
                            f64::from(age),
                            64.0,
                        )
                    } else {
                        Vec2::ZERO
                    };
                    let p = anchor + recoil;
                    if side == 0 {
                        enemy(p);
                    } else {
                        draw_texture_ex(
                            &art.bodies[0],
                            p.x - 32.0,
                            p.y - 32.0,
                            WHITE,
                            DrawTextureParams {
                                dest_size: Some(vec2(64.0, 64.0)),
                                ..Default::default()
                            },
                        );
                    }
                    let who = if side == 0 { "Monstre" } else { "Joueur" };
                    draw_text_bold(format!("{who} · {label}"), rect.x, 35.0, 21.0, TEXT);
                    draw_text("Aller-retour visuel · même case", rect.x, 60.0, 14.0, DIM);
                    draw_text(
                        "Application préparée, aucun calcul de PV",
                        rect.x + 12.0,
                        rect.bottom() - 12.0,
                        14.0,
                        TEXT,
                    );
                }
                super::occult::export(
                    &target,
                    &folder.join(if control == 0 {
                        format!("{frame:03}.rgba")
                    } else {
                        format!("control-{frame:03}.rgba")
                    }),
                );
            }
            set_default_camera();
            next_frame().await;
        }
        std::fs::write(folder.join("timeline.json"), "{\"contact_frames\":[17,48],\"logical_positions\":[[4,4],[4,4]],\"real_damage_or_status_calculated\":false}")
            .map_err(|e| e.to_string())?;
        common_entries.push(serde_json::json!({"folder":case,"label":label,"frames":96,"fps":24,"controls":controls,"contact_frames":[17,48],"map":MAPS[0],"capture_size":[1280,720]}));
    }
    std::fs::write(
        output.join("common-capture.json"),
        serde_json::to_vec_pretty(&common_entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    let mut entries = Vec::new();
    let closure_only = std::env::args().any(|arg| arg == "--capture-fermeture-only");
    for (map, direction, quiet, case) in [
        (0, 2, false, "HAUT"),
        (0, 0, false, "DROITE"),
        (0, 1, false, "GAUCHE"),
        (0, 3, false, "BAS"),
        (1, 2, false, "OBSTACLE"),
        (2, 2, false, "BLOQUE"),
        (0, 2, true, "ATTENTE"),
        (0, 2, false, "RESERVE"),
        (0, 2, false, "AGIR"),
        (0, 2, false, "SILENCE"),
    ] {
        for (pair, skills) in [[0, 3], [1, 4], [2, 5]].into_iter().enumerate() {
            if closure_only && pair != 2 {
                continue;
            }
            let folder = format!("{case}-{}", pair + 1);
            let dir = output.join(&folder);
            std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
            let mut states = skills.map(|skill| State::new(skill, map, direction, case));
            let mut timeline = Vec::new();
            for frame in 0..96 {
                for s in &mut states {
                    s.clock = frame as f32 / 24.0;
                    if frame == 17 {
                        s.launch();
                    }
                    if !quiet && TURNS.contains(&frame) {
                        s.advance();
                    }
                }
                timeline.push(serde_json::json!({"frame":frame,"states":states.iter().map(|s|serde_json::json!({"turn":s.turn,"cooldown":s.cooldown,"closed":s.closed,"resolutions":s.resolutions,"charged":s.charged,"was_charged":s.was_charged,"acting":s.acting,"extra_hit":s.extra_hit.is_some(),"silenced":s.silenced,"player":[s.player.x,s.player.y],"hits":s.hits.iter().map(|p|[p.x,p.y]).collect::<Vec<_>>(),"launched":s.started.is_some()})).collect::<Vec<_>>()}));
                for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                    if control_only && control == 0 && ![3, 19, 23, 52].contains(&frame) {
                        continue;
                    }
                    set_camera(&camera);
                    clear_background(INK);
                    for (j, s) in states.iter().enumerate() {
                        let x = 32.0 + j as f32 * 616.0;
                        draw_text_bold(
                            format!("{} · {}", 37 + s.skill, TITLES[s.skill]),
                            x,
                            35.0,
                            23.0,
                            TEXT,
                        );
                        draw_text(
                            format!("{}-A02 · {case} · tour {}", IDS[s.skill], s.turn),
                            x,
                            60.0,
                            14.0,
                            DIM,
                        );
                        scene(&art, &clip, s, Rect::new(x, 88.0, SIZE, SIZE), control == 0);
                    }
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
            let map = Map::from_ascii(MAPS[map]).unwrap();
            let mut rows = vec![vec!['.'; 9]; 9];
            for y in 0..9 {
                for x in 0..9 {
                    if map.blocks_vision(GridPos::new(x, y)) {
                        let p = position(GridPos::new(x, y), direction);
                        rows[p.y as usize][p.x as usize] = '#';
                    }
                }
            }
            entries.push(serde_json::json!({"folder":folder,"case":case,"frames":96,"fps":24,"tile":64,"quiet":quiet,"launch_frame":17,"turn_frames":if quiet{vec![]}else{TURNS.to_vec()},"control_frames":controls,"capture_size":[1280,720],"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n"),"views":skills.into_iter().enumerate().map(|(j,i)|serde_json::json!({"id":IDS[i],"number":37+i,"title":TITLES[i],"variant":format!("{}-A02-{case}",IDS[i]),"crop":[32+j*616,88,576,576]})).collect::<Vec<_>>()}));
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
    fn closure_is_turn_based_and_does_not_prevent_prepared_movement_or_striking() {
        let mut s = State::new(2, 0, 2, "AGIR");
        s.launch();
        s.clock = 20.0;
        assert_eq!(s.closed, 3);
        s.advance();
        assert_eq!(s.player, GridPos::new(3, 5));
        assert_eq!(s.closed, 2);
        s.advance();
        assert!(s.extra_hit.is_some());
        assert_eq!(s.closed, 1);
        s.advance();
        assert_eq!(s.closed, 0);
        assert_eq!(s.resolutions, 1);
    }
    #[test]
    fn reserve_is_consumed_once_at_discharge_launch() {
        let mut s = State::new(1, 0, 2, "RESERVE");
        s.launch();
        assert!(!s.charged);
        assert!(s.was_charged);
        s.clock = 100.0;
        assert_eq!(s.resolutions, 1);
    }
    #[test]
    fn silence_refuses_the_two_powers_but_preserves_the_ordinary_trait() {
        for (skill, allowed) in [(3, true), (4, false), (5, false)] {
            let mut s = State::new(skill, 0, 2, "SILENCE");
            s.launch();
            assert_eq!(s.started.is_some(), allowed);
            assert_eq!(s.resolutions, usize::from(allowed) as u16);
            if !allowed {
                assert_eq!(s.cooldown, 0);
                assert!(s.hits.is_empty());
            }
        }
    }
    #[test]
    fn cloison_refuses_contact_and_ranged_targets_without_spending_a_cooldown() {
        for skill in [0, 3, 5] {
            let mut s = State::new(skill, 2, 2, "BLOQUE");
            s.launch();
            assert!(s.started.is_none());
            assert_eq!(s.cooldown, 0);
            assert_eq!(s.resolutions, 0);
        }
        let mut s = State::new(4, 2, 2, "BLOQUE");
        s.launch();
        assert!(s.started.is_some());
        assert!(s.hits.iter().all(|p| p.y > 3));
    }
    #[test]
    fn single_target_rays_have_a_clear_prepared_path_and_no_extra_contacts() {
        for skill in [3, 5] {
            let mut s = State::new(skill, 0, 2, "HAUT");
            s.launch();
            assert_eq!(s.hits, vec![DISTANT]);
            assert!(!s.monsters().contains(&CONTACT));
            s.clock = 100.0;
            assert_eq!(s.resolutions, 1);
        }
    }
}
