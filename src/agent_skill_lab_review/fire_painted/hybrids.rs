//! ANIM-S19: mutagenic strike and drone sacrifice, prepared visual fixtures only.
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;

const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 6);
const TARGET: GridPos = GridPos::new(4, 3);
const DRONE: GridPos = GridPos::new(4, 4);
const ALLY: GridPos = GridPos::new(3, 4);
const IDS: [&str; 2] = ["M02", "HY07"];
const TITLES: [&str; 2] = ["Décharge mutagène", "Autodestruction"];
const FORMS: [&str; 6] = [
    "Humain",
    "Mue prédatrice",
    "Colosse",
    "Cuirassé",
    "Possédé",
    "Abysse",
];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const MAPS: [&str; 3] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#..###..#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#########",
];
const COOLDOWNS: [u16; 2] = [1, 1];
const COSTS: [u16; 2] = [9, 6];
const TURN_FRAMES: [usize; 3] = [48, 66, 84];
const FLIGHT: f32 = 0.20;
const CORE: f32 = 0.10;
const IMPACT: f32 = 0.32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    Open,
    Wall,
    Quiet,
    Silence,
    NoDamage,
    Cap,
    Scout,
    Mule,
    Hidden,
    Transformed,
}
impl Case {
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Pièce ouverte",
            Self::Wall => "Mur",
            Self::Quiet => "Attente sans tour",
            Self::Silence => "Silence",
            Self::NoDamage => "Aucun PV retiré",
            Self::Cap => "Prolongation plafonnée",
            Self::Scout => "Éclaireur exclu",
            Self::Mule => "Mule exclue",
            Self::Hidden => "Drone caché",
            Self::Transformed => "Forme transformée",
        }
    }
}

struct State {
    skill: usize,
    case: Case,
    direction: usize,
    form: usize,
    model: u8,
    clock: f32,
    started: Option<f32>,
    cooldown: u16,
    turn: u16,
    form_remaining: u16,
    bonus_used: u16,
    gained: u16,
    destroyed: bool,
    hits: Vec<GridPos>,
    cast_cells: Vec<GridPos>,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize, form: usize, model: u8) -> Self {
        Self {
            skill,
            case,
            direction,
            form,
            model,
            clock: 0.0,
            started: None,
            cooldown: 0,
            turn: 0,
            form_remaining: if form == 0 { 0 } else { 22 },
            bonus_used: if case == Case::Cap { 4 } else { 0 },
            gained: 0,
            destroyed: false,
            hits: Vec::new(),
            cast_cells: Vec::new(),
        }
    }
    fn map_index(&self) -> usize {
        if self.case == Case::Wall {
            self.skill + 1
        } else {
            0
        }
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn visible(&self, at: GridPos) -> bool {
        match (self.skill, self.case) {
            (0, Case::Wall) => at.y >= 5,
            (1, Case::Wall) => at.x < 5,
            (1, Case::Hidden) => at != DRONE,
            _ => true,
        }
    }
    fn enemies(&self) -> Vec<GridPos> {
        [
            TARGET,
            GridPos::new(3, 3),
            GridPos::new(6, 4),
            GridPos::new(7, 6),
        ]
        .into_iter()
        .filter(|at| self.map().is_walkable(*at))
        .collect()
    }
    fn blast(&self) -> Vec<GridPos> {
        // Prepared radius-two area with ordinary cardinal flood and wall barriers.
        // This prototype does not establish a new global diagonal metric.
        let mut seen = vec![DRONE];
        let mut frontier = vec![(DRONE, 0)];
        while let Some((at, distance)) = frontier.pop() {
            if distance == 2 {
                continue;
            }
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let next = GridPos::new(at.x + dx, at.y + dy);
                if self.map().is_walkable(next) && !seen.contains(&next) {
                    seen.push(next);
                    frontier.push((next, distance + 1));
                }
            }
        }
        seen.into_iter().filter(|at| self.visible(*at)).collect()
    }
    fn line_clear(&self) -> bool {
        (3..6).all(|y| self.map().is_walkable(GridPos::new(4, y))) && self.visible(TARGET)
    }
    fn can_launch(&self) -> bool {
        if self.started.is_some() || self.cooldown > 0 || self.case == Case::Silence {
            return false;
        }
        if self.skill == 0 {
            self.line_clear()
        } else {
            self.form == 0 && matches!(self.model, 58 | 59 | 66) && self.visible(DRONE)
        }
    }
    fn footprint(&self) -> Vec<GridPos> {
        if !self.can_launch() {
            return Vec::new();
        }
        if self.skill == 0 {
            vec![TARGET]
        } else {
            self.blast()
        }
    }
    fn launch(&mut self) {
        if !self.can_launch() {
            return;
        }
        self.cast_cells = self.footprint();
        self.started = Some(self.clock);
        self.cooldown = COOLDOWNS[self.skill];
        if self.skill == 0 {
            self.hits = if self.case == Case::NoDamage {
                Vec::new()
            } else {
                vec![TARGET]
            };
            if self.form > 0 && !self.hits.is_empty() {
                self.gained = 2.min(4u16.saturating_sub(self.bonus_used));
                self.form_remaining += self.gained;
                self.bonus_used += self.gained;
            }
        } else {
            self.destroyed = true;
            self.hits = self
                .enemies()
                .into_iter()
                .filter(|at| self.cast_cells.contains(at))
                .collect();
        }
    }
    fn advance(&mut self) {
        if self.started.is_none() {
            return;
        }
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        self.form_remaining = self.form_remaining.saturating_sub(1);
    }
    fn age(&self) -> Option<f32> {
        self.started.map(|t| self.clock - t)
    }
    fn contact_time(&self) -> f32 {
        if self.skill == 0 { FLIGHT } else { CORE }
    }
    fn hit_point(&self, at: GridPos, rect: Rect, effects: bool) -> Vec2 {
        let p = point(at, self.direction, rect);
        if !effects || !self.hits.contains(&at) {
            return p;
        }
        let age = self.age().unwrap_or(-1.0) - self.contact_time();
        p + crate::combat_visuals::hit_reaction_offset(
            p,
            point(
                if self.skill == 0 { PLAYER } else { DRONE },
                self.direction,
                rect,
            ),
            f64::from(age),
            64.0,
        )
    }
}
fn point(at: GridPos, direction: usize, rect: Rect) -> Vec2 {
    let p = position(at, direction);
    cell(rect, p.x as f32 + 0.5, p.y as f32 + 0.5)
}
struct Art {
    shot: Texture2D,
    impact: Texture2D,
    burst: Texture2D,
    bodies: Vec<Texture2D>,
}
impl Art {
    fn load() -> Self {
        let load = |bytes: &[u8]| {
            let t = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            shot: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-48/shot-atlas.png"
            )),
            impact: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-48/impact-atlas.png"
            )),
            burst: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-20/burst-atlas.png"
            )),
            bodies: vec![
                load(include_bytes!(
                    "../../../assets/animations/agent/skill-animation-review-39/predator-64.png"
                )),
                load(include_bytes!(
                    "../../../assets/animations/agent/skill-animation-review-39/colossus-64.png"
                )),
                load(include_bytes!(
                    "../../../assets/animations/agent/skill-animation-review-41/cuirasse-64.png"
                )),
                load(include_bytes!(
                    "../../../assets/animations/agent/skill-animation-review-47/possede-64.png"
                )),
                load(include_bytes!(
                    "../../../assets/animations/agent/skill-animation-review-44/abysse-64.png"
                )),
            ],
        }
    }
    fn sheet(texture: &Texture2D, at: Vec2, size: f32, frame: usize, opacity: f32, rotation: f32) {
        draw_texture_ex(
            texture,
            at.x - size * 0.5,
            at.y - size * 0.5,
            alpha(WHITE, opacity),
            DrawTextureParams {
                source: Some(Rect::new(frame.min(5) as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(vec2(size, size)),
                rotation,
                pivot: Some(at),
                ..Default::default()
            },
        );
    }
    fn body(&self, s: &State, rect: Rect, effects: bool) {
        let p = point(PLAYER, s.direction, rect);
        if s.form == 0 {
            actor(
                p,
                game_art::wear(
                    game_art::player(Some("core:metamorphe"), PlayerWeaponPose::Unarmed, None),
                    [("core:body_armor".into(), "core:tenue_metamorphe".into())],
                ),
            );
        } else {
            draw_ellipse(p.x, p.y + 29.0, 22.0, 4.0, 0.0, alpha(BLACK, 0.3));
            draw_texture_ex(
                &self.bodies[s.form - 1],
                p.x - 32.0,
                p.y - 32.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(64.0, 64.0)),
                    ..Default::default()
                },
            );
        }
        if effects && s.gained > 0 {
            if let Some(age) = s.age().filter(|a| *a >= FLIGHT && *a < FLIGHT + 0.24) {
                Self::sheet(
                    &self.impact,
                    p,
                    72.0,
                    ((age - FLIGHT) / 0.24 * 6.0) as usize,
                    0.8,
                    0.0,
                );
            }
        }
    }
    fn vfx(&self, s: &State, rect: Rect) {
        let Some(age) = s.age() else { return };
        if s.skill == 0 {
            let start = point(PLAYER, s.direction, rect);
            let end = s.hit_point(TARGET, rect, true);
            let axis = (end - start).normalize();
            if age < FLIGHT {
                let head = start.lerp(end, (age / FLIGHT).clamp(0.0, 1.0));
                Self::sheet(
                    &self.shot,
                    head - axis * 18.0,
                    96.0,
                    ((age / FLIGHT * 6.0) as usize).min(5),
                    1.0,
                    axis.y.atan2(axis.x),
                );
            } else if age < FLIGHT + IMPACT {
                Self::sheet(
                    &self.impact,
                    end,
                    112.0,
                    ((age - FLIGHT) / IMPACT * 6.0) as usize,
                    0.94,
                    0.0,
                );
            }
        } else {
            let centre = point(DRONE, s.direction, rect);
            if age < CORE {
                let power = (age / CORE * std::f32::consts::PI).sin();
                draw_circle(
                    centre.x,
                    centre.y,
                    7.0,
                    alpha(Color::new(1.0, 0.10, 0.04, 1.0), power),
                );
            } else if age < CORE + 0.42 {
                let elapsed = age - CORE;
                let phase = (elapsed / 0.42).clamp(0.0, 1.0);
                Self::sheet(
                    &self.burst,
                    // Keep the approved fire atlas's painted impact anchor.
                    centre - vec2(4.0, 18.0) * (300.0 / 256.0),
                    300.0,
                    (phase * 6.0) as usize,
                    0.70,
                    0.0,
                );
                for i in 0..9 {
                    let theta = i as f32 * std::f32::consts::TAU / 9.0 + 0.17;
                    let axis = vec2(theta.cos(), theta.sin());
                    let normal = vec2(-axis.y, axis.x);
                    let at = centre + axis * (16.0 + phase * 105.0);
                    let length = 8.0 * (1.0 - phase);
                    let color = alpha(Color::new(0.72, 0.75, 0.74, 1.0), 0.9 * (1.0 - phase));
                    draw_triangle(
                        at + axis * length,
                        at - axis * length + normal * 3.0,
                        at - axis * length - normal * 2.0,
                        color,
                    );
                }
            }
        }
    }
}
fn draw_drone(at: Vec2, model: u8) {
    actor(
        at,
        Visual::Bestiary {
            number: model,
            corpse: false,
        },
    );
    draw_circle(
        at.x + 18.0,
        at.y - 23.0,
        3.0,
        Color::new(0.20, 0.95, 0.25, 1.0),
    );
}
fn scene(art: &Art, clip: &Clip, s: &State, rect: Rect, effects: bool) {
    let map = s.map();
    terrain(&map, s.direction, rect, false);
    if s.started.is_none() {
        preview(&s.footprint(), s.direction, rect, s.clock);
    }
    terrain(&map, s.direction, rect, true);
    clip.apply_for(s.map_index(), s.direction, rect);
    for at in s.enemies().into_iter().filter(|at| s.visible(*at)) {
        enemy(s.hit_point(at, rect, effects));
    }
    art.body(s, rect, effects);
    if s.skill == 1 {
        if s.visible(DRONE) && (!s.destroyed || s.age().is_some_and(|a| a < CORE)) {
            draw_drone(point(DRONE, s.direction, rect), s.model);
        }
        draw_drone(point(ALLY, s.direction, rect), 59);
    }
    if effects {
        art.vfx(s, rect);
    }
    gl_use_default_material();
    if let Some(age) = s.age().filter(|a| *a < 0.65) {
        let p = point(PLAYER, s.direction, rect);
        draw_text_bold(
            TITLES[s.skill],
            p.x - 70.0,
            p.y - 43.0 - age * 6.0,
            15.0,
            alpha(TEXT, 1.0 - age / 0.65),
        );
        if s.gained > 0 {
            draw_text_bold(
                format!("+{} tours", s.gained),
                p.x - 26.0,
                p.y - 60.0 - age * 9.0,
                16.0,
                alpha(Color::new(0.70, 1.0, 0.48, 1.0), 1.0 - age / 0.65),
            );
        }
    }
    draw_rectangle(
        rect.x + 5.0,
        rect.y + SIZE - 25.0,
        556.0,
        20.0,
        alpha(INK, 0.92),
    );
    draw_text(
        format!(
            "{} · forme {} tours · recharge {} · contacts {}",
            FORMS[s.form],
            s.form_remaining,
            s.cooldown,
            s.hits.len()
        ),
        rect.x + 12.0,
        rect.y + SIZE - 10.0,
        14.0,
        TEXT,
    );
}

pub(crate) async fn run() {
    request_new_screen_size(1280.0, 820.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let clip = Clip::load_maps(&MAPS, PLAYER);
    let mut s = State::new(0, Case::Open, 2, 0, 58);
    let mut paused = false;
    let stage = Rect::new(410.0, 88.0, SIZE, SIZE);
    let cases = [
        Case::Open,
        Case::Wall,
        Case::Quiet,
        Case::Silence,
        Case::NoDamage,
        Case::Cap,
        Case::Scout,
        Case::Mule,
        Case::Hidden,
        Case::Transformed,
    ];
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(INK);
        draw_text_bold("Pouvoirs hybrides · ANIM-S19", 20.0, 33.0, 23.0, TEXT);
        draw_text(
            "Deux compositions proposées · cases 64 px",
            20.0,
            59.0,
            14.0,
            DIM,
        );
        for (i, title) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 95.0 + i as f32 * 42.0, 320.0, 34.0),
                title,
                true,
            ) {
                s = State::new(i, Case::Open, s.direction, 0, 58);
            }
        }
        for (i, title) in DIRS.iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 164.0,
                    193.0 + (i / 2) as f32 * 37.0,
                    156.0,
                    31.0,
                ),
                title,
                true,
            ) {
                s = State::new(s.skill, s.case, i, s.form, s.model);
            }
        }
        for (i, title) in FORMS.iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 164.0,
                    275.0 + (i / 2) as f32 * 34.0,
                    156.0,
                    28.0,
                ),
                title,
                true,
            ) {
                s = State::new(s.skill, Case::Open, s.direction, i, s.model);
            }
        }
        for (i, case) in cases.iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 184.0,
                    387.0 + (i / 2) as f32 * 32.0,
                    176.0,
                    27.0,
                ),
                case.title(),
                true,
            ) {
                let model = match case {
                    Case::Scout => 56,
                    Case::Mule => 57,
                    _ => 58,
                };
                let form = if matches!(case, Case::NoDamage | Case::Cap | Case::Transformed) {
                    4
                } else {
                    s.form
                };
                s = State::new(s.skill, *case, s.direction, form, model);
            }
        }
        for (i, (model, title)) in [(58, "Traqueur"), (59, "Artilleur"), (66, "Gardien")]
            .iter()
            .enumerate()
        {
            if button(
                Rect::new(20.0 + i as f32 * 112.0, 557.0, 106.0, 28.0),
                title,
                s.skill == 1,
            ) {
                s = State::new(1, Case::Open, s.direction, 0, *model);
            }
        }
        if button(
            Rect::new(20.0, 604.0, 320.0, 34.0),
            "Lancer",
            s.can_launch(),
        ) || (is_key_pressed(KeyCode::Enter) && s.can_launch())
        {
            s.launch();
        }
        if button(
            Rect::new(20.0, 647.0, 151.0, 34.0),
            "+1 tour",
            s.started.is_some(),
        ) || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(181.0, 647.0, 159.0, 34.0), "Nouvel aperçu", true) {
            s = State::new(s.skill, s.case, s.direction, s.form, s.model);
        }
        if button(
            Rect::new(20.0, 690.0, 320.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
        }
        scene(&art, &clip, &s, stage, true);
        draw_text(
            format!(
                "Énergie {} · recharge {} tours · portée {} cases",
                COSTS[s.skill],
                COOLDOWNS[s.skill],
                if s.skill == 0 { 3 } else { 6 }
            ),
            410.0,
            705.0,
            16.0,
            TEXT,
        );
        draw_text(
            "Monstres et drones provisoires 32 px agrandis ; adaptation après les animations.",
            410.0,
            735.0,
            13.0,
            DIM,
        );
        draw_text(
            "Prototype visuel ; aucun combat ou sauvegarde de campagne modifié.",
            410.0,
            763.0,
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
    let controls = [14, 17, 20, 22, 25, 28, 32, 35, 47, 48, 66, 84, 95];
    let mut entries = Vec::new();
    for skill in 0..2 {
        let mut cases = vec![
            (Case::Open, 2, 0, 58, "HAUT"),
            (Case::Open, 0, 0, 58, "DROITE"),
            (Case::Open, 1, 0, 58, "GAUCHE"),
            (Case::Open, 3, 0, 58, "BAS"),
            (Case::Wall, 2, 0, 58, "CLOISON"),
            (Case::Silence, 2, 0, 58, "SILENCE"),
            (
                Case::Quiet,
                2,
                if skill == 0 { 4 } else { 0 },
                58,
                "ATTENTE",
            ),
        ];
        if skill == 0 {
            cases.extend([
                (Case::Open, 2, 1, 58, "PREDATEUR"),
                (Case::Open, 2, 2, 58, "COLOSSE"),
                (Case::Open, 2, 3, 58, "CUIRASSE"),
                (Case::Open, 2, 4, 58, "POSSEDE"),
                (Case::Open, 2, 5, 58, "ABYSSE"),
                (Case::NoDamage, 2, 4, 58, "SANS-PV"),
                (Case::Cap, 2, 4, 58, "PLAFOND"),
            ]);
        } else {
            cases.extend([
                (Case::Open, 2, 0, 59, "ARTILLEUR"),
                (Case::Open, 2, 0, 66, "GARDIEN"),
                (Case::Scout, 2, 0, 56, "ECLAIREUR"),
                (Case::Mule, 2, 0, 57, "MULE"),
                (Case::Hidden, 2, 0, 58, "CACHE"),
                (Case::Transformed, 2, 4, 58, "TRANSFORME"),
            ]);
        }
        for (case, direction, form, model, label) in cases {
            let folder = format!("{}-{label}", IDS[skill]);
            let dir = output.join(&folder);
            std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
            let mut s = State::new(skill, case, direction, form, model);
            let permitted = s.can_launch();
            let mut timeline = Vec::new();
            for frame in 0..96 {
                s.clock = frame as f32 / 24.0;
                if frame == 17 {
                    s.launch();
                }
                if case != Case::Quiet && TURN_FRAMES.contains(&frame) {
                    s.advance();
                }
                timeline.push(serde_json::json!({"frame":frame,"turn":s.turn,"cooldown":s.cooldown,"form_remaining":s.form_remaining,
                    "launched":s.started.is_some(),"destroyed":s.destroyed,"gained":s.gained,"bonus_used":s.bonus_used,
                    "hits":s.hits.iter().map(|a|[a.x,a.y]).collect::<Vec<_>>(),"cast_cells":s.cast_cells.iter().map(|a|[a.x,a.y]).collect::<Vec<_>>() }));
                for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                    set_camera(&camera);
                    clear_background(INK);
                    draw_text_bold(
                        format!("{} · {}", 47 + skill, TITLES[skill]),
                        32.0,
                        35.0,
                        23.0,
                        TEXT,
                    );
                    draw_text(
                        format!(
                            "{}-A01 · {} · {} · tour {}",
                            IDS[skill], label, FORMS[form], s.turn
                        ),
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
            entries.push(serde_json::json!({"folder":folder,"variant":format!("{}-A01-{label}",IDS[skill]),"id":IDS[skill],"number":47+skill,
                "title":TITLES[skill],"case":label,"case_title":case.title(),"form":form,"form_title":FORMS[form],"model":model,
                "direction":direction,"permitted":permitted,"quiet":case==Case::Quiet,"cooldown":COOLDOWNS[skill],"crop":[32,88,576,576],
                "tile":64,"frames":96,"fps":24,"control_frames":controls,"turn_frames":if case==Case::Quiet{vec![]}else{TURN_FRAMES.to_vec()},
                "map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n")}));
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
    fn mutagene_extends_all_forms_only_for_real_damage_with_shared_cap() {
        for form in 1..6 {
            let mut s = State::new(0, Case::Open, 2, form, 58);
            s.launch();
            assert_eq!(s.gained, 2);
            assert_eq!(s.form_remaining, 24);
        }
        for case in [Case::NoDamage, Case::Cap] {
            let mut s = State::new(0, case, 2, 4, 58);
            s.launch();
            assert_eq!(s.gained, 0);
            assert_eq!(s.form_remaining, 22);
        }
        let mut s = State::new(0, Case::Open, 2, 0, 58);
        s.launch();
        assert_eq!(s.gained, 0);
        assert_eq!(s.form_remaining, 0);
    }
    #[test]
    fn sacrifice_only_three_models_and_only_enemies_in_reachable_area() {
        for model in [58, 59, 66] {
            let mut s = State::new(1, Case::Open, 2, 0, model);
            s.launch();
            assert!(s.destroyed);
            assert!(!s.hits.contains(&PLAYER) && !s.hits.contains(&ALLY));
            assert_eq!(s.hits.len(), 3);
        }
        for model in [56, 57] {
            let mut s = State::new(1, Case::Open, 2, 0, model);
            s.launch();
            assert!(!s.destroyed);
            assert_eq!(s.cooldown, 0);
        }
        let mut s = State::new(1, Case::Wall, 2, 0, 58);
        s.launch();
        assert_eq!(s.hits.len(), 2);
        assert!(!s.cast_cells.contains(&GridPos::new(6, 4)));
    }
    #[test]
    fn refusals_keep_state_and_turns_are_explicit() {
        for (skill, case, form) in [
            (0, Case::Wall, 0),
            (0, Case::Silence, 4),
            (1, Case::Silence, 0),
            (1, Case::Hidden, 0),
            (1, Case::Transformed, 4),
        ] {
            let mut s = State::new(skill, case, 2, form, 58);
            s.launch();
            assert!(s.started.is_none());
            assert_eq!(s.cooldown, 0);
            assert!(s.hits.is_empty());
            assert!(!s.destroyed);
        }
        let mut s = State::new(0, Case::Quiet, 2, 4, 58);
        s.launch();
        s.clock = 100.0;
        assert_eq!(s.form_remaining, 24);
        assert_eq!(s.cooldown, 1);
        s.advance();
        assert_eq!(s.form_remaining, 23);
        assert_eq!(s.cooldown, 0);
    }
}
