//! ANIM-S07: painted frost nova and destructible wall; isolated review, no GameState.
use super::obstacles::overlay::stopped::burn_on_body;
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;
use std::collections::{BTreeSet, VecDeque};

pub(crate) mod crystalline;
pub(crate) mod fronts;
pub(crate) mod gel_status;

const SIZE: f32 = 576.0;
const CENTRE: GridPos = GridPos::new(4, 4);
const WALL_CASTER: GridPos = GridPos::new(4, 7);
const NOVA_SECONDS: f32 = 0.45;
const GROW_SECONDS: f32 = 0.22;
const BREAK_SECONDS: f32 = 0.30;
const IDS: [&str; 2] = ["FR02", "FR03"];
const TITLES: [&str; 2] = ["Nova de givre", "Mur de glace"];
const DIRS: [&str; 4] = ["Droite", "Gauche", "Haut", "Bas"];
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#########",
];
const EVENTS: [f32; 5] = [3.0, 3.7, 4.4, 5.1, 5.8];

struct Art {
    nova: Texture2D,
    wall: Texture2D,
    gel: Texture2D,
    revised: bool,
}
impl Art {
    fn load() -> Self {
        let load = |bytes: &[u8]| {
            let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            texture.set_filter(FilterMode::Linear);
            texture
        };
        let original = load(include_bytes!(
            "../../../assets/animations/agent/skill-animation-review-23/wall-atlas.png"
        ));
        let revised = std::env::args().any(|arg| arg == "--mur-glace-v2");
        Self {
            nova: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-23/nova-atlas.png"
            )),
            wall: if revised {
                load(include_bytes!(
                    "../../../assets/animations/agent/skill-animation-review-24/wall-atlas.png"
                ))
            } else {
                original.clone()
            },
            gel: original,
            revised,
        }
    }
    fn nova(&self, centre: Vec2, age: f32) {
        let frame = ((age / NOVA_SECONDS * 6.0) as usize).min(5);
        let size = 304.0;
        draw_texture_ex(
            &self.nova,
            centre.x - size / 2.0,
            centre.y - size / 2.0,
            alpha(WHITE, ((NOVA_SECONDS - age) / 0.12).clamp(0.0, 1.0)),
            DrawTextureParams {
                source: Some(Rect::new(frame as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    }
    fn block(&self, ground: Vec2, frame: usize, size: f32, opacity: f32) {
        Self::legacy_block(&self.wall, ground, frame, size, opacity);
    }
    fn legacy_block(texture: &Texture2D, ground: Vec2, frame: usize, size: f32, opacity: f32) {
        draw_texture_ex(
            texture,
            ground.x - size * 0.5,
            ground.y - size * 0.9,
            alpha(WHITE, opacity),
            DrawTextureParams {
                source: Some(Rect::new(frame.min(5) as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    }
    fn wall_block(&self, ground: Vec2, at: GridPos, age: f32, cracked: bool) {
        let variant = (at.x - 1).rem_euclid(3) as usize;
        let frame = variant + if cracked { 3 } else { 0 };
        let size = 76.0;
        let rise = 1.0 - (1.0 - (age / GROW_SECONDS).clamp(0.0, 1.0)).powi(3);
        let height = size * rise.max(0.025);
        draw_texture_ex(
            &self.wall,
            ground.x - size * 0.5,
            ground.y - height * 0.9,
            alpha(WHITE, 0.94),
            DrawTextureParams {
                source: Some(Rect::new(frame as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(vec2(size, height)),
                ..Default::default()
            },
        );
    }
    fn broken_block(&self, ground: Vec2, at: GridPos, age: f32) {
        let frame = 6 + (at.x - 1).rem_euclid(3) as usize;
        let size = 76.0;
        draw_texture_ex(
            &self.wall,
            ground.x - size * 0.5,
            ground.y - size * 0.9 + age * 12.0,
            alpha(
                WHITE,
                0.94 * (1.0 - age / BREAK_SECONDS).clamp(0.0, 1.0).powf(0.7),
            ),
            DrawTextureParams {
                source: Some(Rect::new(frame as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    }
    fn variant(&self, skill: usize) -> &'static str {
        if self.revised && skill == 1 {
            "A02"
        } else {
            "A01"
        }
    }
    fn ice_feet(&self, at: Vec2, foreground: bool) {
        if foreground {
            Self::legacy_block(&self.gel, at + vec2(-7.0, 24.0), 2, 22.0, 0.66);
            Self::legacy_block(&self.gel, at + vec2(7.0, 24.0), 2, 21.0, 0.60);
        } else {
            Self::legacy_block(&self.gel, at + vec2(0.0, 23.0), 2, 33.0, 0.72);
        }
    }
}
fn nova_cells(map: &Map, radius: i32) -> Vec<GridPos> {
    let mut seen = BTreeSet::from([(CENTRE.x, CENTRE.y)]);
    let mut queue = VecDeque::from([CENTRE]);
    while let Some(at) = queue.pop_front() {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next = GridPos::new(at.x + dx, at.y + dy);
            if (next.x - CENTRE.x).pow(2) + (next.y - CENTRE.y).pow(2) <= radius.pow(2)
                && map.is_walkable(next)
                && seen.insert((next.x, next.y))
            {
                queue.push_back(next);
            }
        }
    }
    seen.into_iter().map(|(x, y)| GridPos::new(x, y)).collect()
}
fn wall_cells(map: &Map, rank: u16) -> Vec<GridPos> {
    let length = i32::from(parameter_for("FR03", "maximum_wall_length_cells", rank).min(7));
    let mut cells = vec![CENTRE];
    for sign in [-1, 1] {
        for offset in 1..=length / 2 {
            let at = GridPos::new(CENTRE.x + offset * sign, CENTRE.y);
            if !map.is_walkable(at) {
                break;
            }
            cells.push(at);
        }
    }
    cells.sort_by_key(|p| p.x);
    cells
}
struct State {
    skill: usize,
    map: usize,
    direction: usize,
    rank: u16,
    clock: f32,
    launched: Option<f32>,
    turn: u16,
    cooldown: u16,
    wall_remaining: u16,
    blocks: Vec<GridPos>,
    broken: Vec<(GridPos, f32)>,
    freeze: [u16; 5],
    burn: [u16; 5],
    attacker: GridPos,
    last_attack: Option<f32>,
    movement: Option<(GridPos, f32)>,
}
impl State {
    fn new(skill: usize, map: usize, direction: usize, rank: u16) -> Self {
        let mut burn = [0; 5];
        burn[0] = 3;
        Self {
            skill,
            map,
            direction,
            rank,
            clock: 0.0,
            launched: None,
            turn: 0,
            cooldown: 0,
            wall_remaining: 0,
            blocks: Vec::new(),
            broken: Vec::new(),
            freeze: [0; 5],
            burn,
            attacker: GridPos::new(4, 3),
            last_attack: None,
            movement: None,
        }
    }
    fn player(&self) -> GridPos {
        if self.skill == 0 { CENTRE } else { WALL_CASTER }
    }
    fn targets(&self) -> [GridPos; 5] {
        if self.skill == 0 {
            [
                GridPos::new(4, 3),
                GridPos::new(3, 4),
                GridPos::new(4, 6),
                GridPos::new(6, 4),
                GridPos::new(4, 1),
            ]
        } else {
            [
                self.attacker,
                GridPos::new(2, 2),
                GridPos::new(6, 3),
                GridPos::new(4, 1),
                GridPos::new(1, 6),
            ]
        }
    }
    fn cells(&self) -> Vec<GridPos> {
        let map = Map::from_ascii(MAPS[self.map]).unwrap();
        if self.skill == 0 {
            nova_cells(
                &map,
                i32::from(parameter_for("FR02", "radius_cells", self.rank)),
            )
        } else {
            wall_cells(&map, self.rank)
        }
    }
    fn age(&self) -> Option<f32> {
        self.launched.map(|at| (self.clock - at).max(0.0))
    }
    fn nova_active(&self) -> bool {
        self.skill == 0 && self.age().is_some_and(|a| a < NOVA_SECONDS)
    }
    fn can_launch(&self) -> bool {
        self.cooldown == 0
    }
    fn launch(&mut self) {
        if !self.can_launch() {
            return;
        }
        self.launched = Some(self.clock);
        self.cooldown = parameter_for(IDS[self.skill], "cooldown_turns", self.rank);
        if self.skill == 0 {
            let zone = self.cells();
            for (i, at) in self.targets().iter().enumerate() {
                if zone.contains(at) {
                    self.freeze[i] = parameter_for("FR02", "freeze_duration_turns", self.rank);
                    self.burn[i] = 0;
                }
            }
        } else {
            self.broken
                .extend(self.blocks.drain(..).map(|at| (at, self.clock)));
            self.blocks = self.cells();
            self.wall_remaining = parameter_for("FR03", "wall_duration_turns", self.rank);
        }
    }
    fn walkable(&self, at: GridPos) -> bool {
        Map::from_ascii(MAPS[self.map]).unwrap().is_walkable(at) && !self.blocks.contains(&at)
    }
    fn advance_turn(&mut self) {
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        for n in &mut self.freeze {
            *n = n.saturating_sub(1);
        }
        for n in &mut self.burn {
            *n = n.saturating_sub(1);
        }
        self.wall_remaining = self.wall_remaining.saturating_sub(1);
        if self.wall_remaining == 0 {
            self.broken
                .extend(self.blocks.drain(..).map(|at| (at, self.clock)));
        }
        if self.skill == 1 && self.launched.is_some() {
            let next = GridPos::new(self.attacker.x, self.attacker.y + 1);
            if self.blocks.contains(&next) {
                // Prepared review events, not random chances, monster damage or campaign AI.
                self.last_attack = Some(self.clock);
                if self.turn == 1 {
                    self.freeze[0] = parameter_for("FR03", "freeze_duration_turns", self.rank);
                    self.burn[0] = 0;
                }
                if self.turn == 2 {
                    self.blocks.retain(|&at| at != next);
                    self.broken.push((next, self.clock));
                }
            } else if self.freeze[0] == 0 && self.walkable(next) && next.y < WALL_CASTER.y {
                self.movement = Some((self.attacker, self.clock));
                self.attacker = next;
            }
        }
    }
    fn point(&self, at: GridPos, rect: Rect) -> Vec2 {
        let p = position(at, self.direction);
        cell(rect, p.x as f32 + 0.5, p.y as f32 + 0.5)
    }
    fn actor_point(&self, index: usize, rect: Rect) -> Vec2 {
        let mut p = self.point(self.targets()[index], rect);
        if self.skill == 1 && index == 0 {
            if let Some((old, start)) = self.movement {
                let progress = ((self.clock - start) / 0.18).clamp(0.0, 1.0);
                p = self.point(old, rect).lerp(p, progress);
            }
            if let Some(start) = self.last_attack {
                let age = self.clock - start;
                if (0.0..0.18).contains(&age) {
                    let target = self.point(GridPos::new(4, 4), rect);
                    p += (target - p).normalize_or_zero()
                        * ((age / 0.18) * std::f32::consts::PI).sin()
                        * 5.0;
                }
            }
        }
        p
    }
}
fn scene(art: &Art, fire: &DrawnFire, clip: &Clip, state: &State, rect: Rect, effects: bool) {
    let map = Map::from_ascii(MAPS[state.map]).unwrap();
    terrain(&map, state.direction, rect, false);
    if state.launched.is_none() {
        preview(&state.cells(), state.direction, rect, state.clock);
    }
    if effects {
        clip.apply_for(state.map, state.direction, rect);
        if state.nova_active() {
            art.nova(state.point(CENTRE, rect), state.age().unwrap());
        }
        if state.wall_remaining > 0 {
            let age = state.age().unwrap_or(0.0);
            let growth = ((age / GROW_SECONDS * 3.0) as usize).min(2);
            for &at in &state.blocks {
                let frame = if state.turn == 1 && at == CENTRE {
                    3
                } else {
                    growth
                };
                let ground = state.point(at, rect) + vec2(0.0, 24.0);
                if art.revised {
                    art.wall_block(ground, at, age, state.turn == 1 && at == CENTRE);
                } else {
                    art.block(ground, frame, 70.0, 0.85);
                }
            }
        }
        for &(at, start) in &state.broken {
            let age = state.clock - start;
            if (0.0..BREAK_SECONDS).contains(&age) {
                if art.revised {
                    art.broken_block(state.point(at, rect) + vec2(0.0, 24.0), at, age);
                } else {
                    art.block(
                        state.point(at, rect) + vec2(0.0, 24.0),
                        if age < 0.13 { 4 } else { 5 },
                        70.0,
                        0.85 * ((BREAK_SECONDS - age) / 0.12).clamp(0.0, 1.0),
                    );
                }
            }
        }
        gl_use_default_material();
        for i in 0..5 {
            let p = state.actor_point(i, rect);
            if state.freeze[i] > 0 {
                art.ice_feet(p, false);
            }
            if state.burn[i] > 0 {
                fire.burn(p, state.clock * (2.0 / 3.0), i as u32 * 2, 0.9);
            }
        }
    }
    terrain(&map, state.direction, rect, true);
    for i in 0..5 {
        let p = state.actor_point(i, rect);
        enemy(p);
        if effects && state.freeze[i] > 0 {
            art.ice_feet(p, true);
        }
        if effects && state.burn[i] > 0 {
            burn_on_body(fire, p, state.clock * (2.0 / 3.0), i as u32 * 2);
        }
        draw_text_bold(
            char::from(b'A' + i as u8).to_string(),
            p.x - 4.0,
            p.y - 30.0,
            13.0,
            TEXT,
        );
    }
    let player = state.point(state.player(), rect);
    actor(player, caster());
    if state.age().is_some_and(|age| age < 0.7) {
        let age = state.age().unwrap();
        draw_text_bold(
            TITLES[state.skill],
            player.x - 45.0,
            player.y - 43.0 - age * 7.0,
            15.0,
            alpha(TEXT, 1.0 - age / 0.7),
        );
    }
    let label = if state.launched.is_none() {
        "Aperçu clignotant · avant lancement".to_string()
    } else if state.skill == 0 {
        format!("Onde brève · Gel : {} tours", state.freeze[0])
    } else {
        format!(
            "Mur : {} tours · {} blocs · Gel A : {} tours",
            state.wall_remaining,
            state.blocks.len(),
            state.freeze[0]
        )
    };
    draw_rectangle(
        rect.x + 7.0,
        rect.y + rect.h - 27.0,
        rect.w - 14.0,
        22.0,
        alpha(INK, 0.94),
    );
    draw_text(label, rect.x + 14.0, rect.y + rect.h - 11.0, 14.0, TEXT);
}
pub(crate) async fn run() {
    request_new_screen_size(1280.0, 900.0);
    let art = Art::load();
    let fire = DrawnFire::load();
    let clip = Clip::load_maps(&MAPS, CENTRE);
    let mut state = State::new(0, 0, 2, 11);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold("LABORATOIRE / FROID", 22.0, 35.0, 24.0, TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 88.0 + i as f32 * 44.0, 285.0, 35.0),
                title,
                true,
            ) {
                state = State::new(i, state.map, state.direction, state.rank);
            }
        }
        for (i, title) in ["Zone ouverte", "Cloison pleine"].iter().enumerate() {
            if button(
                Rect::new(20.0, 198.0 + i as f32 * 44.0, 285.0, 35.0),
                title,
                true,
            ) {
                state = State::new(state.skill, i, state.direction, state.rank);
            }
        }
        for (i, title) in DIRS.iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 146.0,
                    305.0 + (i / 2) as f32 * 42.0,
                    138.0,
                    34.0,
                ),
                title,
                true,
            ) {
                state = State::new(state.skill, state.map, i, state.rank);
            }
        }
        for (i, rank) in [1, 11, 18].iter().enumerate() {
            if button(
                Rect::new(20.0 + i as f32 * 98.0, 418.0, 90.0, 34.0),
                &format!("Rang {rank}"),
                true,
            ) {
                state = State::new(state.skill, state.map, state.direction, *rank);
            }
        }
        let label = if state.cooldown == 0 {
            "Lancer".to_string()
        } else {
            format!("Recharge : {} tours", state.cooldown)
        };
        if button(
            Rect::new(20.0, 475.0, 285.0, 34.0),
            &label,
            state.can_launch(),
        ) || (is_key_pressed(KeyCode::Enter) && state.can_launch())
        {
            state.launch();
            paused = false;
        }
        if button(Rect::new(20.0, 520.0, 138.0, 34.0), "+1 tour", true)
            || is_key_pressed(KeyCode::Space)
        {
            state.advance_turn();
        }
        if button(Rect::new(166.0, 520.0, 138.0, 34.0), "Nouvel aperçu", true) {
            state = State::new(state.skill, state.map, state.direction, state.rank);
        }
        if button(
            Rect::new(20.0, 565.0, 285.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        draw_text(
            format!("Tour {} · rang {}", state.turn, state.rank),
            22.0,
            650.0,
            16.0,
            TEXT,
        );
        draw_text("Gel et destruction préparés", 22.0, 690.0, 15.0, DIM);
        draw_text(
            "pour la revue, sans dégâts calculés.",
            22.0,
            713.0,
            15.0,
            DIM,
        );
        scene(
            &art,
            &fire,
            &clip,
            &state,
            Rect::new(410.0, 88.0, SIZE, SIZE),
            true,
        );
        draw_text(
            format!(
                "{} / {}-{} · {}",
                state.skill + 15,
                IDS[state.skill],
                art.variant(state.skill),
                DIRS[state.direction]
            ),
            410.0,
            695.0,
            15.0,
            DIM,
        );
        if !paused {
            state.clock += get_frame_time();
        }
        next_frame().await;
    }
}
fn export(target: &RenderTarget, path: &Path) {
    unsafe {
        macroquad::window::get_internal_gl().flush();
    }
    let image = target.texture.get_texture_data();
    let stride = usize::from(image.width) * 4;
    let mut pixels = Vec::with_capacity(image.bytes.len());
    for row in image.bytes.chunks_exact(stride).rev() {
        pixels.extend_from_slice(row);
    }
    for pixel in pixels.as_chunks_mut::<4>().0 {
        pixel[3] = 255;
    }
    std::fs::write(path, pixels).unwrap();
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
    let fire = DrawnFire::load();
    let clip = Clip::load_maps(&MAPS, CENTRE);
    let target = render_target(1280, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1280.0, 720.0))
    };
    let cases = [
        (0, 2, "HAUT"),
        (0, 0, "DROITE"),
        (0, 1, "GAUCHE"),
        (0, 3, "BAS"),
        (1, 2, "CLOISON"),
    ];
    let controls = [
        3, 14, 19, 23, 29, 50, 71, 76, 93, 96, 108, 125, 143, 151, 167,
    ];
    let mut entries = Vec::new();
    for (map, direction, folder) in cases {
        let dir = output.join(folder);
        std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
        let mut states = [
            State::new(0, map, direction, 11),
            State::new(1, map, direction, 11),
        ];
        let mut timeline = Vec::new();
        let mut launched = false;
        let mut event = 0;
        for frame in 0..168 {
            let clock = frame as f32 / 24.0;
            for s in &mut states {
                s.clock = clock;
            }
            if !launched && frame >= 17 {
                for s in &mut states {
                    s.launch();
                }
                launched = true;
            }
            while event < EVENTS.len() && clock >= EVENTS[event] {
                for s in &mut states {
                    s.advance_turn();
                }
                event += 1;
            }
            timeline.push(serde_json::json!({"frame":frame,"turn":states[0].turn,
                "nova_active":states[0].nova_active(),"cooldowns":[states[0].cooldown,states[1].cooldown],
                "freeze":[states[0].freeze,states[1].freeze],"burn":[states[0].burn,states[1].burn],
                "wall_remaining":states[1].wall_remaining,"blocks":states[1].blocks.iter().map(|p|[p.x,p.y]).collect::<Vec<_>>(),
                "attacker":[states[1].attacker.x,states[1].attacker.y],
                "attacker_walkable":states[1].walkable(states[1].attacker)}));
            for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                set_camera(&camera);
                clear_background(INK);
                for (skill, state) in states.iter().enumerate() {
                    let x = 24.0 + skill as f32 * 616.0;
                    draw_text_bold(
                        format!("{} · {}", skill + 15, TITLES[skill]),
                        x,
                        35.0,
                        21.0,
                        TEXT,
                    );
                    draw_text(
                        format!(
                            "{}-{} · {} · tour {}",
                            IDS[skill],
                            art.variant(skill),
                            DIRS[direction],
                            state.turn
                        ),
                        x,
                        60.0,
                        14.0,
                        DIM,
                    );
                    scene(
                        &art,
                        &fire,
                        &clip,
                        state,
                        Rect::new(x, 80.0, SIZE, SIZE),
                        control == 0,
                    );
                }
                export(
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
        for skill in 0..2 {
            let state = &states[skill];
            let mut rows = vec![vec!['.'; 9]; 9];
            let m = Map::from_ascii(MAPS[map]).unwrap();
            for y in 0..9 {
                for x in 0..9 {
                    if m.blocks_vision(GridPos::new(x, y)) {
                        let p = position(GridPos::new(x, y), direction);
                        rows[p.y as usize][p.x as usize] = '#';
                    }
                }
            }
            let coords = |at| {
                let p = position(at, direction);
                [p.x, p.y]
            };
            entries.push(serde_json::json!({"number":skill+15,"id":IDS[skill],"title":TITLES[skill],
                "variant":format!("{}-{}-{}",IDS[skill],art.variant(skill),folder),"folder":folder,"direction":DIRS[direction],
                "crop":[24+skill*616,80,576,576],"frames":168,"fps":24,"tile":64,"control_frames":controls,
                "launch_frame":17,"nova_seconds":NOVA_SECONDS,"growth_seconds":GROW_SECONDS,"break_seconds":BREAK_SECONDS,
                "explicit_turn_frames":[72,89,106,123,140],"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n"),
                "affected_cells":state.cells().into_iter().map(coords).collect::<Vec<_>>(),
                "targets":State::new(skill,map,direction,11).targets().into_iter().enumerate().map(|(i,at)|serde_json::json!({"label":char::from(b'A'+i as u8).to_string(),"cell":coords(at)})).collect::<Vec<_>>(),
                "fixture":"Prepared Gel and block-break results. No campaign damage, RNG, final geometry or AI implementation."}));
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
    fn waiting_expires_only_the_nova_motion() {
        let mut s = State::new(0, 0, 2, 11);
        s.launch();
        s.clock = 600.0;
        assert!(!s.nova_active());
        assert_eq!(s.turn, 0);
        assert_eq!(s.cooldown, 1);
        // The accepted rank-11 radius is five: the fifth target at distance three
        // is now included. Waiting still must not spend any of its four turns.
        assert_eq!(s.freeze, [4; 5]);
        assert_eq!(s.burn[0], 0);
        s.advance_turn();
        assert_eq!(s.freeze, [3; 5]);
        assert_eq!(s.cooldown, 0);
    }
    #[test]
    fn the_cloison_stops_nova_and_wall_placement() {
        let m = Map::from_ascii(MAPS[1]).unwrap();
        assert!(!nova_cells(&m, 2).contains(&GridPos::new(6, 4)));
        assert!(wall_cells(&m, 11).iter().all(|p| p.x < 5));
        assert_eq!(wall_cells(&m, 11).len(), 4);
    }
    #[test]
    fn a_frozen_attacker_can_hit_and_break_the_block_but_cannot_cross_it() {
        let mut s = State::new(1, 0, 2, 11);
        s.launch();
        s.clock = 600.0;
        assert_eq!(s.wall_remaining, 5);
        assert_eq!(s.blocks.len(), 7);
        s.advance_turn();
        assert_eq!(s.attacker, GridPos::new(4, 3));
        assert_eq!(s.freeze[0], 2);
        assert!(!s.walkable(CENTRE));
        assert!(s.last_attack.is_some());
        s.advance_turn();
        assert_eq!(s.freeze[0], 1);
        assert_eq!(s.attacker, GridPos::new(4, 3));
        assert!(s.walkable(CENTRE));
        assert_eq!(s.blocks.len(), 6);
        s.advance_turn();
        assert_eq!(s.freeze[0], 0);
        assert_eq!(s.attacker, CENTRE);
        s.advance_turn();
        s.advance_turn();
        assert_eq!(s.wall_remaining, 0);
        assert!(s.blocks.is_empty());
    }
    #[test]
    fn wall_growth_is_capped_and_rotation_preserves_occupancy() {
        let m = Map::from_ascii(MAPS[0]).unwrap();
        assert_eq!(wall_cells(&m, 1).len(), 3);
        assert_eq!(wall_cells(&m, 11).len(), 7);
        assert_eq!(wall_cells(&m, 100).len(), 7);
        for direction in 0..4 {
            let cells: BTreeSet<_> = wall_cells(&m, 11)
                .into_iter()
                .map(|p| {
                    let r = position(p, direction);
                    (r.x, r.y)
                })
                .collect();
            assert_eq!(cells.len(), 7);
        }
    }
}
