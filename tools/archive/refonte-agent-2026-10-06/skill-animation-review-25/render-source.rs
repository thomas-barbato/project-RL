//! ANIM-S08. Prepared wave/storm visuals, no GameState, RNG, damage or real AI.
use super::*;

const PLAYER: GridPos = GridPos::new(4, 6);
const AREA: GridPos = GridPos::new(4, 4);
const IDS: [&str; 2] = ["FR04", "FR05"];
const NAMES: [&str; 2] = ["Vague de froid", "Blizzard"];
const MAPS: [&str; 6] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#....#..#\n#..#....#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#########\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#....#..#\n#..#....#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#########",
];
const TURN_FRAMES: [usize; 8] = [48, 65, 82, 99, 116, 133, 150, 167];
const MOTION: f32 = 0.18;
const IMPACT: f32 = 0.28;
const DISSOLVE: f32 = 0.25;

fn step(direction: usize) -> (i32, i32) {
    match direction {
        0 => (1, 0),
        1 => (-1, 0),
        3 => (0, 1),
        _ => (0, -1),
    }
}
fn quarter(direction: usize) -> i32 {
    match direction {
        0 => 1,
        1 => 3,
        3 => 2,
        _ => 0,
    }
}
fn angle(direction: usize) -> f32 {
    quarter(direction) as f32 * std::f32::consts::FRAC_PI_2
}
fn area_cells(map: &Map) -> Vec<GridPos> {
    let mut seen = BTreeSet::from([(AREA.x, AREA.y)]);
    let mut queue = VecDeque::from([AREA]);
    while let Some(at) = queue.pop_front() {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next = GridPos::new(at.x + dx, at.y + dy);
            if (next.x - AREA.x).pow(2) + (next.y - AREA.y).pow(2) <= 4
                && map.is_walkable(next)
                && seen.insert((next.x, next.y))
            {
                queue.push_back(next);
            }
        }
    }
    seen.into_iter().map(|(x, y)| GridPos::new(x, y)).collect()
}
struct State {
    skill: usize,
    map: usize,
    scene_dir: usize,
    aim: usize,
    rank: u16,
    clock: f32,
    launched: Option<f32>,
    turn: u16,
    cooldown: u16,
    burn: [u16; 6],
    gel: [u16; 6],
    exposure: [u16; 6],
    distance: u16,
    lanes: [bool; 3],
    previous_lanes: [bool; 3],
    previous_centre: Option<GridPos>,
    changed: f32,
    pending: u16,
    remaining: u16,
    started: bool,
    ended: Option<f32>,
    last_tick: Option<f32>,
    hits: Vec<GridPos>,
    ticks: u16,
}
impl State {
    fn new(skill: usize, map: usize, scene_dir: usize, aim: usize, rank: u16) -> Self {
        let mut burn = [0; 6];
        burn[0] = 3;
        Self {
            skill,
            map,
            scene_dir,
            aim,
            rank,
            clock: 0.,
            launched: None,
            turn: 0,
            cooldown: 0,
            burn,
            gel: [0; 6],
            exposure: [0; 6],
            distance: 0,
            lanes: [false; 3],
            previous_lanes: [false; 3],
            previous_centre: None,
            changed: 0.,
            pending: 0,
            remaining: 0,
            started: false,
            ended: None,
            last_tick: None,
            hits: Vec::new(),
            ticks: 0,
        }
    }
    fn map_index(&self) -> usize {
        self.skill * 3 + self.map
    }
    fn map_data(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn player(&self) -> GridPos {
        if self.skill == 0 {
            PLAYER
        } else {
            GridPos::new(4, 7)
        }
    }
    fn targets(&self) -> [GridPos; 6] {
        if self.skill == 0 {
            [
                GridPos::new(3, 5),
                GridPos::new(4, 4),
                GridPos::new(5, if self.map == 2 { 2 } else { 3 }),
                GridPos::new(4, 1),
                GridPos::new(6, 4),
                GridPos::new(3, 2),
            ]
        } else {
            [
                GridPos::new(4, 3),
                GridPos::new(3, 4),
                GridPos::new(
                    if self.map == 2 { 6 } else { 5 },
                    if self.map == 2 { 3 } else { 4 },
                ),
                GridPos::new(4, 6),
                GridPos::new(6, 4),
                GridPos::new(4, 1),
            ]
        }
    }
    fn centre_at(&self, distance: u16) -> GridPos {
        let (dx, dy) = step(self.aim);
        GridPos::new(
            PLAYER.x + dx * i32::from(distance),
            PLAYER.y + dy * i32::from(distance),
        )
    }
    fn lane_at(&self, lane: usize, distance: u16) -> GridPos {
        let (dx, dy) = step(self.aim);
        let c = self.centre_at(distance);
        let offset = lane as i32 - 1;
        GridPos::new(c.x - dy * offset, c.y + dx * offset)
    }
    fn cells(&self) -> Vec<GridPos> {
        if self.skill == 1 {
            return area_cells(&self.map_data());
        }
        let distance = self.distance.max(1);
        (0..3)
            .filter_map(|i| {
                let at = self.lane_at(i, distance);
                (self.map_data().is_walkable(at) && (self.launched.is_none() || self.lanes[i]))
                    .then_some(at)
            })
            .collect()
    }
    fn launch(&mut self) {
        if self.cooldown > 0 {
            return;
        }
        self.launched = Some(self.clock);
        self.cooldown = parameter_for(IDS[self.skill], "cooldown_turns", self.rank);
        if self.skill == 0 {
            self.distance = 1;
            self.lanes = std::array::from_fn(|i| self.map_data().is_walkable(self.lane_at(i, 1)));
            self.changed = self.clock;
            self.wave_contacts();
        } else {
            self.pending = parameter_for("FR05", "first_impact_delay_complete_turns", self.rank);
            self.remaining = parameter_for("FR05", "duration_turns", self.rank);
            if self.pending == 0 {
                self.started = true;
                self.storm_tick();
            }
        }
    }
    fn wave_contacts(&mut self) {
        let front = self.cells();
        for (i, at) in self.targets().iter().enumerate() {
            if front.contains(at) {
                self.gel[i] = parameter_for("FR04", "freeze_duration_turns", self.rank);
                self.burn[i] = 0;
            }
        }
    }
    fn storm_tick(&mut self) {
        // Fixed examples of distinct legal cells. No real random sampling or chance roll.
        const PICKS: [[(i32, i32); 4]; 3] = [
            [(4, 3), (3, 4), (4, 4), (6, 4)],
            [(5, 4), (4, 6), (3, 3), (5, 5)],
            [(4, 3), (5, 4), (4, 5), (3, 5)],
        ];
        let area = self.cells();
        let limit = usize::from(parameter_for("FR05", "impacts_per_turn", self.rank).min(5));
        self.hits.clear();
        for (x, y) in PICKS[self.ticks as usize % 3] {
            let p = GridPos::new(x, y);
            if area.contains(&p) {
                self.hits.push(p);
            }
        }
        for &p in &area {
            if self.hits.len() >= limit {
                break;
            }
            if !self.hits.contains(&p) {
                self.hits.push(p);
            }
        }
        self.hits.truncate(limit);
        self.ticks += 1;
        self.last_tick = Some(self.clock);
        for (i, at) in self.targets().iter().enumerate() {
            if area.contains(at) {
                self.exposure[i] += 1;
            } else {
                self.exposure[i] = 0;
            }
            if self.hits.contains(at) {
                self.gel[i] = parameter_for("FR05", "freeze_duration_turns", self.rank);
                self.burn[i] = 0;
            }
        }
    }
    fn advance(&mut self) {
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        for n in &mut self.burn {
            *n = n.saturating_sub(1);
        }
        for n in &mut self.gel {
            *n = n.saturating_sub(1);
        }
        if self.launched.is_none() {
            return;
        }
        if self.skill == 0 && self.lanes.iter().any(|&x| x) {
            self.previous_centre = Some(self.centre_at(self.distance));
            self.previous_lanes = self.lanes;
            self.changed = self.clock;
            self.distance += 1;
            for i in 0..3 {
                self.lanes[i] = self.lanes[i]
                    && self.distance
                        <= parameter_for("FR04", "maximum_wave_travel_cells", self.rank)
                    && self.map_data().is_walkable(self.lane_at(i, self.distance));
            }
            if self.lanes.iter().any(|&x| x) {
                self.wave_contacts();
            } else {
                self.ended = Some(self.clock);
            }
        } else if self.skill == 1 && self.remaining > 0 {
            if self.pending > 0 {
                self.pending -= 1;
                if self.pending == 0 {
                    self.started = true;
                    self.storm_tick();
                }
            } else {
                self.remaining -= 1;
                if self.remaining > 0 {
                    self.storm_tick();
                } else {
                    self.ended = Some(self.clock);
                    self.exposure = [0; 6];
                }
            }
        }
    }
    fn point(&self, at: GridPos, rect: Rect) -> Vec2 {
        let p = position(at, self.scene_dir);
        cell(rect, p.x as f32 + 0.5, p.y as f32 + 0.5)
    }
    fn front_point(&self, rect: Rect) -> Vec2 {
        let current = self.point(self.centre_at(self.distance), rect);
        self.previous_centre.map_or(current, |old| {
            self.point(old, rect).lerp(
                current,
                ((self.clock - self.changed) / MOTION).clamp(0., 1.),
            )
        })
    }
    fn rotation(&self) -> f32 {
        angle(self.aim) + angle(self.scene_dir)
    }
    fn screen_aim(&self) -> usize {
        match (quarter(self.aim) + quarter(self.scene_dir)) % 4 {
            1 => 0,
            3 => 1,
            2 => 3,
            _ => 2,
        }
    }
}
struct Art {
    wave: Texture2D,
    storm: Texture2D,
    ice: super::Art,
}
impl Art {
    fn load() -> Self {
        let load = |bytes: &[u8]| {
            let t = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            wave: load(include_bytes!(
                "../../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-25/wave-atlas.png"
            )),
            storm: load(include_bytes!(
                "../../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-25/storm-atlas.png"
            )),
            ice: super::Art::load(),
        }
    }
    fn front(&self, state: &State, rect: Rect) {
        let alive = state.lanes.iter().any(|&x| x);
        let (lanes, centre, opacity) = if alive {
            (state.lanes, state.front_point(rect), 0.92)
        } else {
            let Some(ended) = state.ended else {
                return;
            };
            let age = state.clock - ended;
            if age >= DISSOLVE {
                return;
            }
            (
                state.previous_lanes,
                state.point(state.previous_centre.unwrap(), rect),
                0.92 * (1. - age / DISSOLVE),
            )
        };
        let frame = ((state.clock * 6.) as usize) % 6;
        let width = 192.;
        let mut start = 0;
        while start < 3 {
            if !lanes[start] {
                start += 1;
                continue;
            }
            let mut end = start + 1;
            while end < 3 && lanes[end] {
                end += 1;
            }
            let fraction = (end - start) as f32 / 3.;
            draw_texture_ex(
                &self.wave,
                centre.x - width / 2. + width * start as f32 / 3.,
                centre.y - width / 2.,
                alpha(WHITE, opacity),
                DrawTextureParams {
                    source: Some(Rect::new(
                        frame as f32 * 256. + 256. * start as f32 / 3.,
                        0.,
                        256. * fraction,
                        256.,
                    )),
                    dest_size: Some(vec2(width * fraction, width)),
                    rotation: state.rotation(),
                    pivot: Some(centre),
                    ..Default::default()
                },
            );
            start = end;
        }
    }
    fn blizzard(&self, state: &State, rect: Rect) {
        let opacity = if state.remaining > 0 {
            if state.started { 0.68 } else { 0.28 }
        } else {
            let Some(end) = state.ended else {
                return;
            };
            (0.68 * (1. - (state.clock - end) / DISSOLVE)).max(0.)
        };
        if opacity <= 0. {
            return;
        }
        let p = state.point(AREA, rect);
        let size = 352.;
        let frame = ((state.clock * 6.) as usize) % 6;
        draw_texture_ex(
            &self.storm,
            p.x - size / 2.,
            p.y - size / 2.,
            alpha(WHITE, opacity),
            DrawTextureParams {
                source: Some(Rect::new(frame as f32 * 256., 0., 256., 256.)),
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    }
    fn impacts(&self, state: &State, rect: Rect) {
        if let Some(start) = state.last_tick {
            let age = state.clock - start;
            if (0.0..IMPACT).contains(&age) {
                for &at in &state.hits {
                    super::Art::legacy_block(
                        &self.ice.gel,
                        state.point(at, rect) + vec2(0., 22.),
                        if age < 0.12 { 4 } else { 5 },
                        54.,
                        0.85 * ((IMPACT - age) / 0.12).clamp(0., 1.),
                    );
                }
            }
        }
    }
}
fn scene(art: &Art, fire: &DrawnFire, clip: &Clip, state: &State, rect: Rect, effects: bool) {
    let map = state.map_data();
    terrain(&map, state.scene_dir, rect, false);
    if state.launched.is_none() {
        preview(&state.cells(), state.scene_dir, rect, state.clock);
        if state.skill == 0 {
            let from = state.point(PLAYER, rect);
            let to = state.point(state.centre_at(1), rect);
            let direction = (to - from).normalize_or_zero();
            let side = vec2(-direction.y, direction.x);
            draw_line(
                from.x,
                from.y,
                to.x,
                to.y,
                3.,
                Color::new(0.2, 0.85, 0.92, 0.85),
            );
            draw_triangle(
                to,
                to - direction * 12. + side * 7.,
                to - direction * 12. - side * 7.,
                Color::new(0.2, 0.85, 0.92, 0.85),
            );
        }
    }
    if effects && state.launched.is_some() {
        clip.apply_for(state.map_index(), state.scene_dir, rect);
        if state.skill == 0 {
            art.front(state, rect);
        } else {
            art.blizzard(state, rect);
            art.impacts(state, rect);
        }
        gl_use_default_material();
    }
    if effects {
        for i in 0..6 {
            let p = state.point(state.targets()[i], rect);
            if state.gel[i] > 0 {
                art.ice.ice_feet(p, false);
            }
            if state.burn[i] > 0 {
                fire.burn(p, state.clock * (2. / 3.), i as u32 * 2, 0.9);
            }
        }
    }
    terrain(&map, state.scene_dir, rect, true);
    for i in 0..6 {
        let p = state.point(state.targets()[i], rect);
        enemy(p);
        if effects && state.gel[i] > 0 {
            art.ice.ice_feet(p, true);
        }
        if effects && state.burn[i] > 0 {
            burn_on_body(fire, p, state.clock * (2. / 3.), i as u32 * 2);
        }
        draw_text_bold(
            char::from(b'A' + i as u8).to_string(),
            p.x - 4.,
            p.y - 30.,
            13.,
            TEXT,
        );
    }
    let player = state.point(state.player(), rect);
    actor(player, caster());
    if let Some(start) = state.launched {
        let age = state.clock - start;
        if age < 0.7 {
            draw_text_bold(
                NAMES[state.skill],
                player.x - 45.,
                player.y - 43. - age * 7.,
                15.,
                alpha(TEXT, 1. - age / 0.7),
            );
        }
    }
    let info = if state.launched.is_none() {
        "Bande de départ · avant lancement".to_string()
    } else if state.skill == 0 {
        format!(
            "Front : {}/5 · Gel A : {} tours",
            state.distance.min(5),
            state.gel[0]
        )
    } else if state.pending > 0 {
        "Formation · premières frappes au prochain tour".to_string()
    } else {
        format!(
            "Tempête : {} tours · {} impacts · Gel A : {}",
            state.remaining,
            state.hits.len(),
            state.gel[0]
        )
    };
    draw_rectangle(
        rect.x + 7.,
        rect.y + rect.h - 27.,
        rect.w - 14.,
        22.,
        alpha(INK, 0.94),
    );
    draw_text(info, rect.x + 14., rect.y + rect.h - 11., 14., TEXT);
}
pub(crate) async fn run() {
    request_new_screen_size(1280., 900.);
    let art = Art::load();
    let fire = DrawnFire::load();
    let clip = Clip::load_maps(&MAPS, AREA);
    let mut s = State::new(0, 0, 2, 2, 11);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold("LABORATOIRE / TEMPÊTES DE FROID", 22., 35., 24., TEXT);
        for (i, title) in NAMES.iter().enumerate() {
            if button(Rect::new(20., 88. + i as f32 * 44., 285., 35.), title, true) {
                s = State::new(i, s.map, 2, s.aim, s.rank);
            }
        }
        for (i, title) in ["Zone ouverte", "Obstacles partiels", "Cloison pleine"]
            .iter()
            .enumerate()
        {
            if button(
                Rect::new(20., 198. + i as f32 * 44., 285., 35.),
                title,
                true,
            ) {
                s = State::new(s.skill, i, 2, s.aim, s.rank);
            }
        }
        for (i, title) in DIRS.iter().enumerate() {
            if button(
                Rect::new(
                    20. + (i % 2) as f32 * 146.,
                    355. + (i / 2) as f32 * 42.,
                    138.,
                    34.,
                ),
                title,
                s.skill == 0 && s.launched.is_none(),
            ) {
                s = State::new(s.skill, s.map, 2, i, s.rank);
            }
        }
        for (i, rank) in [1, 11, 18].iter().enumerate() {
            if button(
                Rect::new(20. + i as f32 * 98., 455., 90., 34.),
                &format!("Rang {rank}"),
                true,
            ) {
                s = State::new(s.skill, s.map, 2, s.aim, *rank);
            }
        }
        let label = if s.cooldown == 0 {
            "Lancer".to_string()
        } else {
            format!("Recharge : {} tours", s.cooldown)
        };
        if button(Rect::new(20., 510., 285., 34.), &label, s.cooldown == 0)
            || (is_key_pressed(KeyCode::Enter) && s.cooldown == 0)
        {
            s.launch();
            paused = false;
        }
        if button(Rect::new(20., 555., 138., 34.), "+1 tour", true)
            || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(166., 555., 138., 34.), "Nouvel aperçu", true) {
            s = State::new(s.skill, s.map, 2, s.aim, s.rank);
        }
        if button(
            Rect::new(20., 600., 285., 34.),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        draw_text(
            format!("Tour {} · rang {}", s.turn, s.rank),
            22.,
            680.,
            16.,
            TEXT,
        );
        draw_text("Contacts et Gel préparés.", 22., 714., 15., DIM);
        draw_text("Aucun dégât ni tirage calculé.", 22., 738., 15., DIM);
        scene(
            &art,
            &fire,
            &clip,
            &s,
            Rect::new(410., 88., SIZE, SIZE),
            true,
        );
        draw_text(
            format!("{} / {}-A01", s.skill + 17, IDS[s.skill]),
            410.,
            695.,
            15.,
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
    request_new_screen_size(1280., 720.);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let fire = DrawnFire::load();
    let clip = Clip::load_maps(&MAPS, AREA);
    let target = render_target(1280, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 1280., 720.))
    };
    let cases = [
        (0, 2, 2, "HAUT"),
        (0, 0, 2, "DROITE"),
        (0, 1, 2, "GAUCHE"),
        (0, 3, 2, "BAS"),
        (1, 2, 2, "OBSTACLES"),
        (2, 2, 2, "CLOISON"),
        (0, 2, 0, "PLACEMENT-DROITE"),
    ];
    let idle_ends = [47, 64, 81, 98, 115, 132];
    let controls = [
        3, 14, 23, 40, 47, 51, 64, 68, 81, 85, 98, 102, 115, 119, 132, 136, 153, 170, 183, 191,
    ];
    let mut entries = Vec::new();
    for (map, scene_dir, aim, folder) in cases {
        let dir = output.join(folder);
        std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
        let mut states = [
            State::new(0, map, scene_dir, aim, 11),
            State::new(1, map, scene_dir, 2, 11),
        ];
        let mut timeline = Vec::new();
        for frame in 0..192 {
            let time = frame as f32 / 24.;
            for s in &mut states {
                s.clock = time;
                if frame == 17 {
                    s.launch();
                }
                if TURN_FRAMES.contains(&frame) {
                    s.advance();
                }
            }
            timeline.push(serde_json::json!({"frame":frame,"turn":states[0].turn,"cooldowns":[states[0].cooldown,states[1].cooldown],"gel":[states[0].gel,states[1].gel],"burn":[states[0].burn,states[1].burn],
                "wave_distance":states[0].distance,"lanes":states[0].lanes,"front":states[0].cells().iter().map(|p|[p.x,p.y]).collect::<Vec<_>>(),
                "pending":states[1].pending,"remaining":states[1].remaining,"ticks":states[1].ticks,"exposure":states[1].exposure,
                "hits":states[1].hits.iter().map(|p|[p.x,p.y]).collect::<Vec<_>>() }));
            for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                set_camera(&camera);
                clear_background(INK);
                for (skill, s) in states.iter().enumerate() {
                    let x = 24. + skill as f32 * 616.;
                    draw_text_bold(
                        format!("{} · {}", skill + 17, NAMES[skill]),
                        x,
                        35.,
                        21.,
                        TEXT,
                    );
                    draw_text(
                        format!("{}-A01 · tour {}", IDS[skill], s.turn),
                        x,
                        60.,
                        14.,
                        DIM,
                    );
                    scene(
                        &art,
                        &fire,
                        &clip,
                        s,
                        Rect::new(x, 80., SIZE, SIZE),
                        control == 0,
                    );
                }
                super::export(
                    &target,
                    &dir.join(if control == 0 {
                        format!("{frame:03}.rgba")
                    } else {
                        format!("control-{frame:03}.rgba")
                    }),
                );
            }
            if let Some(stage) = idle_ends.iter().position(|&end| end == frame) {
                // Sample a whole visual loop after the one-time movement/impact.
                // These reads change only the visual clock; no turn or application.
                for idle in 0..6 {
                    for s in &mut states {
                        s.clock = time + 0.4 + idle as f32 / 6.;
                    }
                    set_camera(&camera);
                    clear_background(INK);
                    for (skill, s) in states.iter().enumerate() {
                        let x = 24. + skill as f32 * 616.;
                        scene(&art, &fire, &clip, s, Rect::new(x, 80., SIZE, SIZE), true);
                    }
                    super::export(&target, &dir.join(format!("idle-{stage}-{idle}.rgba")));
                }
            }
            set_default_camera();
            next_frame().await;
        }
        std::fs::write(
            dir.join("timeline.json"),
            serde_json::to_vec_pretty(&timeline).unwrap(),
        )
        .map_err(|e| e.to_string())?;
        for (skill, s) in states.iter().enumerate() {
            let initial = State::new(skill, map, scene_dir, if skill == 0 { aim } else { 2 }, 11);
            let m = initial.map_data();
            let mut rows = vec![vec!['.'; 9]; 9];
            for y in 0..9 {
                for x in 0..9 {
                    if m.blocks_vision(GridPos::new(x, y)) {
                        let p = position(GridPos::new(x, y), scene_dir);
                        rows[p.y as usize][p.x as usize] = '#';
                    }
                }
            }
            let coords = |at| {
                let p = position(at, scene_dir);
                [p.x, p.y]
            };
            entries.push(serde_json::json!({"number":skill+17,"id":IDS[skill],"title":NAMES[skill],"variant":format!("{}-A01-{}",IDS[skill],folder),"folder":folder,"direction":DIRS[if skill==0{s.screen_aim()}else{scene_dir}],
                "crop":[24+skill*616,80,576,576],"frames":192,"fps":24,"tile":64,"control_frames":controls,"launch_frame":17,"turn_frames":TURN_FRAMES,"idle_frames":6,"idle_stages":6,
                "map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n"),"affected_cells":initial.cells().into_iter().map(coords).collect::<Vec<_>>(),
                "targets":initial.targets().into_iter().enumerate().map(|(i,at)|serde_json::json!({"label":char::from(b'A'+i as u8).to_string(),"cell":coords(at)})).collect::<Vec<_>>(),
                "scene_dir":scene_dir,"aim":initial.aim,"motion_seconds":MOTION,"impact_seconds":IMPACT,"dissolve_seconds":DISSOLVE,
                "fixture":"Prepared legal impact cells and successful Gel; no damage, RNG, campaign or final AI."}));
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
    fn wave_advances_only_on_turns_and_stops_at_five_cells() {
        let mut s = State::new(0, 0, 2, 2, 11);
        s.launch();
        assert_eq!(s.gel[0], 3);
        s.clock = 600.;
        assert_eq!(s.distance, 1);
        assert_eq!(s.cooldown, 8);
        for _ in 0..4 {
            s.advance();
        }
        assert_eq!(s.distance, 5);
        assert_eq!(s.gel[3], 3);
        s.advance();
        assert!(!s.lanes.iter().any(|&x| x));
    }
    #[test]
    fn blocked_portions_never_return_behind_an_obstacle() {
        let mut s = State::new(0, 1, 2, 2, 11);
        s.launch();
        s.advance();
        s.advance();
        assert_eq!(s.lanes, [false, true, true]);
        s.advance();
        assert_eq!(s.lanes, [false, true, false]);
        s.advance();
        assert_eq!(s.gel[5], 0);
        assert_eq!(s.gel[3], 3);
    }
    #[test]
    fn initial_band_can_change_direction_without_moving_or_spending() {
        let mut s = State::new(0, 0, 2, 2, 11);
        let old = s.cells();
        s.aim = 0;
        assert_ne!(old, s.cells());
        assert_eq!(s.player(), PLAYER);
        assert_eq!(s.cooldown, 0);
        assert!(s.launched.is_none());
    }
    #[test]
    fn blizzard_preserves_its_explicit_delay_and_exact_application_count() {
        let mut s = State::new(1, 0, 2, 2, 11);
        s.launch();
        s.clock = 600.;
        assert_eq!(s.ticks, 0);
        assert_eq!(s.pending, 1);
        assert_eq!(s.gel, [0; 6]);
        s.advance();
        assert_eq!(s.ticks, 1);
        assert_eq!(s.gel[0], 3);
        assert_eq!(s.burn[0], 0);
        for _ in 0..4 {
            s.advance();
        }
        assert_eq!(s.ticks, 5);
        assert_eq!(s.remaining, 1);
        s.advance();
        assert_eq!(s.ticks, 5);
        assert_eq!(s.remaining, 0);
        assert!(s.ended.is_some());
    }
    #[test]
    fn storm_prepared_impacts_are_distinct_legal_cells_and_far_room_is_protected() {
        let mut s = State::new(1, 2, 2, 2, 11);
        s.launch();
        for _ in 0..5 {
            s.advance();
            let area = s.cells();
            assert_eq!(s.hits.len(), 4);
            assert!(s.hits.iter().all(|p| area.contains(p)));
            let seen: BTreeSet<_> = s.hits.iter().map(|p| (p.x, p.y)).collect();
            assert_eq!(seen.len(), 4);
            assert_eq!(s.gel[2], 0);
            assert_eq!(s.gel[4], 0);
            assert_eq!(s.exposure[2], 0);
        }
    }
}
