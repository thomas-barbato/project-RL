//! ANIM-S18: prepared Abysse actions, isolated from campaign combat/visibility.
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;

const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 5);
const TARGET: GridPos = GridPos::new(4, 3);
const LANDING: GridPos = GridPos::new(4, 2);
const IDS: [&str; 3] = ["TR65-ALLONGE", "TR65-FENTE", "TR65-PAS"];
const TITLES: [&str; 3] = ["Allonge", "Fente abyssale", "Pas d’ombre"];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const MAPS: [&str; 4] = [
    "#########\n#.......#\n#....#..#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#....#..#\n#.......#\n#..###..#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#....#..#\n#.......#\n#...#...#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#...##..#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
];
const TURN_FRAMES: [usize; 3] = [48, 66, 84];
const COOLDOWNS: [u16; 3] = [0, 1, 1];
const COSTS: [u16; 3] = [0, 6, 5];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    Open,
    Wall,
    Obstacle,
    Quiet,
    Silence,
    Occupied,
    Hidden,
    NoShadow,
    TooFar,
    Partial,
}
impl Case {
    fn short_title(self) -> &'static str {
        match self {
            Self::Wall => "Cloison",
            Self::Obstacle => "Obstacle / repérée",
            Self::Partial => "Fente coupée",
            _ => self.title(),
        }
    }
    fn map(self) -> usize {
        match self {
            Self::Wall => 1,
            Self::Obstacle => 2,
            Self::Partial => 3,
            _ => 0,
        }
    }
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Pièce ouverte",
            Self::Wall => "Cloison / cible cachée",
            Self::Obstacle => "Obstacle / destination repérée",
            Self::Quiet => "Attente sans tour",
            Self::Silence => "Silence",
            Self::Occupied => "Destination occupée",
            Self::Hidden => "Destination cachée",
            Self::NoShadow => "Destination sans ombre",
            Self::TooFar => "Destination hors portée",
            Self::Partial => "Fente coupée après deux cases",
        }
    }
}

struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    started: Option<f32>,
    player: GridPos,
    destination: GridPos,
    turn: u16,
    cooldown: u16,
    form_remaining: u16,
    hits: Vec<GridPos>,
    cast_cells: Vec<GridPos>,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.0,
            started: None,
            player: PLAYER,
            destination: match case {
                Case::NoShadow => GridPos::new(2, 5),
                Case::TooFar => GridPos::new(4, 1),
                _ => LANDING,
            },
            turn: 0,
            cooldown: 0,
            form_remaining: 22,
            hits: Vec::new(),
            cast_cells: Vec::new(),
        }
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.case.map()]).unwrap()
    }
    fn visible(&self, at: GridPos) -> bool {
        // Prepared visibility snapshots only. Obstacle deliberately supplies an
        // already-revealed destination; this is not a substitute for game FOV.
        match self.case {
            Case::Wall => at.y >= 5,
            Case::Hidden | Case::Partial => at.y >= 3,
            _ => true,
        }
    }
    fn monsters(&self) -> Vec<GridPos> {
        let mut actors = if self.skill == 1 {
            vec![
                GridPos::new(4, 4),
                TARGET,
                GridPos::new(4, 1),
                GridPos::new(3, 3),
                GridPos::new(6, 6),
            ]
        } else {
            vec![TARGET, GridPos::new(3, 3), GridPos::new(6, 6)]
        };
        if self.case == Case::Occupied {
            actors.push(LANDING);
        }
        actors
            .into_iter()
            .filter(|at| self.map().is_walkable(*at))
            .collect()
    }
    fn line(&self, reach: i32) -> Vec<GridPos> {
        (1..=reach)
            .map(|step| GridPos::new(PLAYER.x, PLAYER.y - step))
            .take_while(|at| self.map().is_walkable(*at) && self.visible(*at))
            .collect()
    }
    fn shadow_valid(&self, at: GridPos) -> bool {
        let distance = (at.x - PLAYER.x).abs() + (at.y - PLAYER.y).abs();
        // These fixtures show cardinal destinations only; diagonals/metrics
        // outside the displayed cases are not decided by this visual prototype.
        distance > 0
            && distance <= 3
            && (at.x == PLAYER.x || at.y == PLAYER.y)
            && self.map().is_walkable(at)
            && self.visible(at)
            && !self.monsters().contains(&at)
            && [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .into_iter()
                .any(|(dx, dy)| self.map().blocks_vision(GridPos::new(at.x + dx, at.y + dy)))
    }
    fn shadow_cells(&self) -> Vec<GridPos> {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .flat_map(|(dx, dy)| {
                (1..=3).map(move |step| GridPos::new(PLAYER.x + dx * step, PLAYER.y + dy * step))
            })
            .filter(|at| self.shadow_valid(*at))
            .collect()
    }
    fn footprint(&self) -> Vec<GridPos> {
        match self.skill {
            0 if self.line(2).contains(&TARGET) => vec![TARGET],
            0 => Vec::new(),
            1 => self.line(4),
            _ => self.shadow_cells(),
        }
    }
    fn can_launch(&self) -> bool {
        if self.started.is_some() || self.cooldown > 0 {
            return false;
        }
        if self.skill == 2 {
            self.case != Case::Silence && self.shadow_valid(self.destination)
        } else {
            !self.footprint().is_empty()
        }
    }
    fn launch(&mut self) {
        if !self.can_launch() {
            return;
        }
        self.cast_cells = self.footprint();
        self.started = Some(self.clock);
        self.cooldown = COOLDOWNS[self.skill];
        if self.skill == 2 {
            self.player = self.destination;
        } else {
            self.hits = self
                .monsters()
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
    fn hit_age(&self) -> Option<f32> {
        self.age()
            .map(|age| age - if self.skill == 0 { 0.08 } else { 0.04 })
    }
    fn monster_point(&self, at: GridPos, rect: Rect, effects: bool) -> Vec2 {
        let p = point(at, self.direction, rect);
        if !effects || !self.hits.contains(&at) {
            return p;
        }
        p + self.hit_age().map_or(Vec2::ZERO, |age| {
            crate::combat_visuals::hit_reaction_offset(
                p,
                point(PLAYER, self.direction, rect),
                f64::from(age),
                64.0,
            )
        })
    }
}

fn point(at: GridPos, direction: usize, rect: Rect) -> Vec2 {
    let p = position(at, direction);
    cell(rect, p.x as f32 + 0.5, p.y as f32 + 0.5)
}
struct Art {
    body: Texture2D,
    allonge: Texture2D,
    fente: Texture2D,
    shadows: Texture2D,
    blood: Texture2D,
}
impl Art {
    fn load() -> Self {
        let load = |bytes: &[u8]| {
            let t = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            body: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-44/abysse-64.png"
            )),
            allonge: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-45/allonge-atlas.png"
            )),
            fente: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-45/fente-atlas.png"
            )),
            shadows: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-44/abysse_fx-atlas.png"
            )),
            blood: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-40/blood-atlas.png"
            )),
        }
    }
    fn body_at(&self, at: Vec2, opacity: f32) {
        draw_ellipse(
            at.x,
            at.y + 29.0,
            20.0,
            4.0,
            0.0,
            alpha(BLACK, opacity * 0.3),
        );
        draw_texture_ex(
            &self.body,
            at.x - 32.0,
            at.y - 32.0,
            alpha(WHITE, opacity),
            DrawTextureParams {
                dest_size: Some(vec2(64.0, 64.0)),
                ..Default::default()
            },
        );
    }
    fn sheet(texture: &Texture2D, at: Vec2, size: Vec2, frame: usize, opacity: f32, rotation: f32) {
        draw_texture_ex(
            texture,
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
    fn body(&self, s: &State, rect: Rect, effects: bool) {
        let age = s.age().unwrap_or(99.0);
        if effects && s.skill == 2 && s.started.is_some() && age < 0.32 {
            let fade = (1.0 - age / 0.32).clamp(0.0, 1.0);
            self.body_at(point(PLAYER, s.direction, rect), fade * 0.42);
            self.body_at(
                point(s.player, s.direction, rect),
                (age / 0.16).clamp(0.0, 1.0),
            );
        } else {
            self.body_at(point(s.player, s.direction, rect), 1.0);
        }
    }
    fn effects(&self, s: &State, rect: Rect) {
        let Some(age) = s.age() else { return };
        if s.skill == 2 {
            if age >= 0.32 {
                return;
            }
            let frame = ((age / 0.32 * 6.0) as usize).min(5);
            for (at, opacity) in [(PLAYER, 0.58 * (1.0 - age / 0.32)), (s.player, 0.70)] {
                Self::sheet(
                    &self.shadows,
                    point(at, s.direction, rect) - vec2(0.0, 12.0),
                    vec2(112.0, 128.0),
                    frame,
                    opacity,
                    0.0,
                );
            }
            return;
        }
        if age < 0.32 {
            let frame = ((age / 0.32 * 6.0) as usize).min(5);
            let p = point(PLAYER, s.direction, rect);
            if s.skill == 0 {
                let offset = if s.direction == 1 {
                    vec2(-18.0, 12.0)
                } else {
                    vec2(18.0, 12.0)
                };
                let start = p + offset;
                let end = point(TARGET, s.direction, rect);
                let axis = (end - start).normalize();
                let width = start.distance(end) / 0.9;
                let centre = start + axis * width * 0.45;
                Self::sheet(
                    &self.allonge,
                    centre,
                    vec2(width, 64.0),
                    frame,
                    if age > 0.27 { (0.32 - age) / 0.05 } else { 1.0 },
                    axis.y.atan2(axis.x),
                );
            } else if let Some(last) = s.cast_cells.last() {
                let end = point(*last, s.direction, rect);
                let axis = (end - p).normalize();
                let start = p + axis * 18.0;
                let finish = end + axis * 20.0;
                Self::sheet(
                    &self.fente,
                    (start + finish) * 0.5,
                    vec2(start.distance(finish), 100.0),
                    frame,
                    0.95,
                    axis.y.atan2(axis.x),
                );
            }
        }
        if let Some(t) = s.hit_age() {
            if (0.0..0.32).contains(&t) {
                let frame = ((t / 0.32 * 6.0) as usize).min(5);
                for &at in &s.hits {
                    let p = s.monster_point(at, rect, true);
                    Self::sheet(
                        &self.blood,
                        p + vec2(0.0, 6.0),
                        vec2(78.0, 72.0),
                        frame,
                        0.80,
                        0.0,
                    );
                }
            }
        }
    }
}

fn scene(art: &Art, clip: &Clip, s: &State, rect: Rect, effects: bool) {
    let map = s.map();
    terrain(&map, s.direction, rect, false);
    if s.started.is_none() {
        preview(&s.footprint(), s.direction, rect, s.clock);
        if s.skill == 2 && s.visible(s.destination) {
            let p = point(s.destination, s.direction, rect);
            draw_rectangle_lines(
                p.x - 29.0,
                p.y - 29.0,
                58.0,
                58.0,
                3.0,
                if s.shadow_valid(s.destination) {
                    GOLD
                } else {
                    Color::new(0.9, 0.25, 0.25, 1.0)
                },
            );
        }
    }
    terrain(&map, s.direction, rect, true);
    // Apply the same wall mask in effects and control captures: motion must not
    // push a victim's sprite through an adjacent wall or change its static shadow.
    clip.apply_for(s.case.map(), s.direction, rect);
    for at in s.monsters().into_iter().filter(|at| s.visible(*at)) {
        enemy(s.monster_point(at, rect, effects));
    }
    art.body(s, rect, effects);
    if effects {
        art.effects(s, rect);
    }
    gl_use_default_material();
    // Previously known terrain can remain drawn, while hidden actors and
    // destination previews are absent. The fixture does not calculate live FOV.
    if matches!(s.case, Case::Hidden | Case::Wall | Case::Partial) {
        let top = if s.case == Case::Wall { 5 } else { 3 };
        for y in 1..top {
            for x in 1..8 {
                if map.is_walkable(GridPos::new(x, y)) {
                    let p = point(GridPos::new(x, y), s.direction, rect);
                    draw_rectangle(p.x - 32.0, p.y - 32.0, 64.0, 64.0, alpha(BLACK, 0.66));
                }
            }
        }
    }
    // Floating skill names are UI. Keep the same cast UI in control captures,
    // while toggling only world VFX/recoil, as in the approved form transitions.
    if let Some(age) = s.age().filter(|age| *age < 0.65) {
        let p = point(s.player, s.direction, rect);
        draw_text_bold(
            TITLES[s.skill],
            p.x - 62.0,
            p.y - 43.0 - age * 6.0,
            15.0,
            alpha(TEXT, 1.0 - age / 0.65),
        );
    }
    draw_rectangle(
        rect.x + 5.0,
        rect.y + SIZE - 25.0,
        510.0,
        20.0,
        alpha(INK, 0.92),
    );
    draw_text(
        format!(
            "Forme {} tours · recharge {} · contacts {}",
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
    let mut s = State::new(0, Case::Open, 2);
    let mut paused = false;
    let cases = [
        Case::Open,
        Case::Wall,
        Case::Obstacle,
        Case::Quiet,
        Case::Silence,
        Case::Occupied,
        Case::Hidden,
        Case::NoShadow,
        Case::TooFar,
        Case::Partial,
    ];
    let stage = Rect::new(410.0, 88.0, SIZE, SIZE);
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(INK);
        draw_text_bold("Abysse · actions · ANIM-S18", 20.0, 33.0, 23.0, TEXT);
        draw_text(
            "Corps validé, nouvelles actions à examiner · 64 px",
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
                s = State::new(i, s.case, s.direction);
            }
        }
        for (i, title) in DIRS.iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 164.0,
                    236.0 + (i / 2) as f32 * 39.0,
                    156.0,
                    32.0,
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
                    20.0 + (i % 2) as f32 * 184.0,
                    333.0 + (i / 2) as f32 * 34.0,
                    176.0,
                    28.0,
                ),
                case.short_title(),
                true,
            ) {
                s = State::new(s.skill, *case, s.direction);
            }
        }
        if s.skill == 2 && s.started.is_none() && is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            if stage.contains(vec2(mx, my)) {
                for at in s.shadow_cells() {
                    let p = point(at, s.direction, stage);
                    if Rect::new(p.x - 32.0, p.y - 32.0, 64.0, 64.0).contains(vec2(mx, my)) {
                        s.destination = at;
                        break;
                    }
                }
            }
        }
        if button(
            Rect::new(20.0, 530.0, 320.0, 34.0),
            "Lancer",
            s.can_launch(),
        ) || (is_key_pressed(KeyCode::Enter) && s.can_launch())
        {
            s.launch();
        }
        if button(
            Rect::new(20.0, 575.0, 151.0, 34.0),
            "+1 tour",
            s.started.is_some(),
        ) || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(181.0, 575.0, 159.0, 34.0), "Nouvel aperçu", true) {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(20.0, 620.0, 320.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        draw_text(
            format!(
                "Tour {} · coût {} énergie · recharge {} tours",
                s.turn, COSTS[s.skill], s.cooldown
            ),
            20.0,
            688.0,
            14.0,
            TEXT,
        );
        draw_text(
            "Choisissez une case proposée pour Pas d’ombre.",
            20.0,
            715.0,
            13.0,
            DIM,
        );
        draw_text(
            "Cas de visibilité préparés ; aucune IA ni dégâts.",
            20.0,
            740.0,
            13.0,
            DIM,
        );
        scene(&art, &clip, &s, stage, true);
        draw_text(
            format!("{} / {}-A01 · TR65-BODY02", 44 + s.skill, IDS[s.skill]),
            410.0,
            694.0,
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
    request_new_screen_size(640.0, 720.0);
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
        3, 14, 17, 18, 19, 20, 21, 22, 23, 24, 25, 27, 30, 35, 40, 47, 48, 66, 84, 95,
    ];
    let mut entries = Vec::new();
    let common = [
        (Case::Open, 2, "HAUT"),
        (Case::Open, 0, "DROITE"),
        (Case::Open, 1, "GAUCHE"),
        (Case::Open, 3, "BAS"),
        (Case::Wall, 2, "CLOISON"),
        (Case::Obstacle, 2, "OBSTACLE"),
        (Case::Quiet, 2, "ATTENTE"),
        (Case::Silence, 2, "SILENCE"),
    ];
    for skill in 0..3 {
        let mut cases = common.to_vec();
        if skill == 2 {
            cases.extend([
                (Case::Occupied, 2, "OCCUPEE"),
                (Case::Hidden, 2, "CACHEE"),
                (Case::NoShadow, 2, "SANS-OMBRE"),
                (Case::TooFar, 2, "HORS-PORTEE"),
            ]);
        }
        if skill == 1 {
            cases.push((Case::Partial, 2, "PARTIEL"));
        }
        for (case, direction, label) in cases {
            let folder = format!("{}-{label}", IDS[skill]);
            let dir = output.join(&folder);
            std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
            let mut s = State::new(skill, case, direction);
            let permitted = s.can_launch();
            let initial_monsters = s.monsters();
            let mut timeline = Vec::new();
            for frame in 0..96 {
                s.clock = frame as f32 / 24.0;
                if frame == 17 {
                    s.launch();
                }
                if case != Case::Quiet && TURN_FRAMES.contains(&frame) {
                    s.advance();
                }
                timeline.push(serde_json::json!({"frame":frame,"turn":s.turn,"cooldown":s.cooldown,
                    "form_remaining":s.form_remaining,"launched":s.started.is_some(),"player":[s.player.x,s.player.y],
                    "hits":s.hits.iter().map(|at|[at.x,at.y]).collect::<Vec<_>>(),
                    "cast_cells":s.cast_cells.iter().map(|at|[at.x,at.y]).collect::<Vec<_>>()}));
                for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                    set_camera(&camera);
                    clear_background(INK);
                    draw_text_bold(
                        format!("{} · {}", 44 + skill, TITLES[skill]),
                        32.0,
                        35.0,
                        23.0,
                        TEXT,
                    );
                    draw_text(
                        format!("{}-A01 · {} · tour {}", IDS[skill], DIRS[direction], s.turn),
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
            entries.push(serde_json::json!({"folder":folder,"variant":format!("{}-A01-{label}", IDS[skill]),
                "id":IDS[skill],"number":44+skill,"title":TITLES[skill],"case":label,"case_title":case.title(),
                "direction":direction,"permitted":permitted,"quiet":case==Case::Quiet,"cooldown":COOLDOWNS[skill],
                "crop":[32,88,576,576],"tile":64,"frames":96,"fps":24,"control_frames":controls,
                "turn_frames":if case==Case::Quiet {vec![]}else{TURN_FRAMES.to_vec()},
                "player_start":[PLAYER.x,PLAYER.y],"destination":[s.destination.x,s.destination.y],
                "prepared_monsters":initial_monsters.iter().map(|at|[at.x,at.y]).collect::<Vec<_>>(),
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
    fn allonge_reaches_two_cells_and_only_the_selected_enemy() {
        let mut s = State::new(0, Case::Open, 2);
        s.launch();
        assert_eq!(s.hits, vec![TARGET]);
        assert_eq!(s.player, PLAYER);
        assert_eq!(s.cooldown, 0);
    }
    #[test]
    fn fente_hits_the_four_cell_line_without_moving_or_stopping_at_monsters() {
        let mut s = State::new(1, Case::Open, 2);
        s.launch();
        assert_eq!(s.cast_cells.len(), 4);
        assert_eq!(s.hits, vec![GridPos::new(4, 4), TARGET, GridPos::new(4, 1)]);
        assert_eq!(s.player, PLAYER);
        assert_eq!(s.cooldown, 1);
        let mut stopped = State::new(1, Case::Partial, 2);
        stopped.launch();
        assert_eq!(stopped.cast_cells, vec![GridPos::new(4, 4), TARGET]);
        assert_eq!(stopped.hits, vec![GridPos::new(4, 4), TARGET]);
    }
    #[test]
    fn invalid_shadow_destinations_refuse_without_movement_or_cooldown() {
        for case in [
            Case::Occupied,
            Case::Hidden,
            Case::NoShadow,
            Case::TooFar,
            Case::Wall,
            Case::Silence,
            Case::Partial,
        ] {
            let mut s = State::new(2, case, 2);
            s.launch();
            assert_eq!(s.player, PLAYER);
            assert_eq!(s.cooldown, 0);
            assert!(s.started.is_none() && s.hits.is_empty());
            assert_eq!(s.form_remaining, 22);
        }
    }
    #[test]
    fn shadow_step_crosses_prepared_obstacles_only_to_an_already_visible_destination() {
        let mut s = State::new(2, Case::Obstacle, 2);
        assert!(s.visible(LANDING) && s.shadow_valid(LANDING));
        s.launch();
        assert_eq!(s.player, LANDING);
        assert_eq!(s.cooldown, 1);
        assert!(s.hits.is_empty());
    }
    #[test]
    fn silence_preserves_the_two_natural_attacks() {
        for skill in [0, 1] {
            let mut s = State::new(skill, Case::Silence, 2);
            s.launch();
            assert!(s.started.is_some() && !s.hits.is_empty());
        }
    }
    #[test]
    fn wall_clock_does_not_add_hits_or_consume_form_duration_and_cooldown() {
        for skill in 0..3 {
            let mut s = State::new(skill, Case::Quiet, 2);
            s.launch();
            let hits = s.hits.clone();
            let player = s.player;
            s.clock = 500.0;
            assert_eq!(s.hits, hits);
            assert_eq!(s.player, player);
            assert_eq!(s.form_remaining, 22);
            assert_eq!(s.cooldown, COOLDOWNS[skill]);
            for _ in 0..3 {
                s.advance();
            }
            assert_eq!(s.form_remaining, 19);
            assert_eq!(s.cooldown, COOLDOWNS[skill].saturating_sub(3));
        }
    }
    #[test]
    fn all_directions_keep_target_reaction_cosmetic_and_restore_the_original_position() {
        let rect = Rect::new(0.0, 0.0, SIZE, SIZE);
        for direction in 0..4 {
            let mut s = State::new(0, Case::Open, direction);
            s.launch();
            s.clock = 0.13;
            let original = point(TARGET, direction, rect);
            assert!(s.monster_point(TARGET, rect, true).distance(original) > 1.0);
            assert_eq!(s.monster_point(TARGET, rect, false), original);
            s.clock = 1.0;
            assert_eq!(s.monster_point(TARGET, rect, true), original);
            assert_eq!(s.hits, vec![TARGET]);
            assert_eq!(s.player, PLAYER);
        }
    }
}
