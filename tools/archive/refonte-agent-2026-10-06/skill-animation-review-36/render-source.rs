//! ANIM-S12: segmented blood circle and marked resurrection. Scripted visual fixtures only.
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::{Map, has_line_of_sight};

const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 7);
const CENTRE: GridPos = GridPos::new(4, 4);
const IDS: [&str; 2] = ["OC04", "OC07"];
const TITLES: [&str; 2] = ["Prison sigillaire", "Réanimation"];
const DIRS: [&str; 4] = ["Droite", "Gauche", "Haut", "Bas"];
const OFFSETS: [(i32, i32); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];
const MAPS: [&str; 3] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#....#..#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#########\n#.......#\n#.......#\n#########",
];
const TURNS: [usize; 13] = [48, 60, 72, 84, 90, 96, 102, 108, 114, 120, 126, 132, 138];

struct State {
    skill: usize,
    map: usize,
    direction: usize,
    quiet: bool,
    clock: f32,
    launched: Option<f32>,
    turn: u16,
    cooldown: u16,
    remaining: u16,
    segments: [bool; 8],
    broken: Option<f32>,
    marked: bool,
    raised: bool,
    birth: Option<f32>,
    expired: bool,
    target: GridPos,
    from: GridPos,
    moved_at: Option<f32>,
    strike: Option<f32>,
}
impl State {
    fn new(skill: usize, map: usize, direction: usize, quiet: bool) -> Self {
        Self {
            skill,
            map,
            direction,
            quiet,
            clock: 0.0,
            launched: None,
            turn: 0,
            cooldown: 0,
            remaining: 0,
            segments: [false; 8],
            broken: None,
            marked: false,
            raised: false,
            birth: None,
            expired: false,
            target: CENTRE,
            from: CENTRE,
            moved_at: None,
            strike: None,
        }
    }
    fn ring() -> [GridPos; 8] {
        OFFSETS.map(|(x, y)| GridPos::new(CENTRE.x + x, CENTRE.y + y))
    }
    fn valid(&self) -> bool {
        let map = Map::from_ascii(MAPS[self.map]).unwrap();
        let key = if self.skill == 0 {
            "center_range_cells"
        } else {
            "range_cells"
        };
        let distance = (CENTRE.x - PLAYER.x).abs().max((CENTRE.y - PLAYER.y).abs());
        distance <= i32::from(parameter_for(IDS[self.skill], key, 11))
            && has_line_of_sight(&map, PLAYER, CENTRE, true)
            && (self.skill == 1 || Self::ring().iter().all(|p| map.is_walkable(*p)))
    }
    fn can_launch(&self) -> bool {
        self.launched.is_none() && self.cooldown == 0 && self.valid()
    }
    fn cells(&self) -> Vec<GridPos> {
        if !self.valid() {
            vec![]
        } else if self.skill == 0 {
            Self::ring().to_vec()
        } else {
            vec![CENTRE]
        }
    }
    fn launch(&mut self) {
        if !self.can_launch() {
            return;
        }
        self.launched = Some(self.clock);
        self.cooldown = parameter_for(IDS[self.skill], "cooldown_turns", 11);
        if self.skill == 0 {
            self.segments = [true; 8];
            self.remaining = parameter_for("OC04", "duration_turns", 11);
        } else {
            self.marked = true;
        }
    }
    fn move_to(&mut self, target: GridPos) {
        if Map::from_ascii(MAPS[self.map]).unwrap().is_walkable(target) {
            self.from = self.target;
            self.target = target;
            self.moved_at = Some(self.clock);
        }
    }
    fn advance(&mut self) {
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        if self.launched.is_none() {
            return;
        }
        if self.skill == 0 && self.remaining > 0 {
            self.remaining -= 1;
            if !self.quiet {
                if self.turn == 1 {
                    self.segments[4] = false;
                    self.broken = Some(self.clock);
                }
                if self.turn == 2 && !self.segments[4] {
                    self.move_to(GridPos::new(4, 5));
                }
                if self.turn == 3 && !self.segments[4] {
                    self.move_to(GridPos::new(4, 6));
                }
            }
            if self.remaining == 0 {
                self.segments = [false; 8];
            }
        } else if self.skill == 1 {
            if self.raised {
                self.remaining = self.remaining.saturating_sub(1);
                if self.remaining == 0 {
                    self.raised = false;
                    self.expired = true;
                }
            } else if self.marked && !self.quiet && self.turn == 2 {
                self.marked = false;
                self.raised = true;
                self.birth = Some(self.clock);
                self.remaining = parameter_for("OC07", "raised_duration_turns", 11);
            }
            if self.raised && !self.quiet {
                if self.turn == 3 {
                    self.move_to(GridPos::new(if self.map == 1 { 3 } else { 5 }, 4));
                }
                if self.turn == 4 {
                    self.strike = Some(self.clock);
                }
            }
        }
    }
    fn age(&self) -> Option<f32> {
        self.launched.map(|t| self.clock - t)
    }
    fn point(&self, at: GridPos, rect: Rect) -> Vec2 {
        let p = position(at, self.direction);
        cell(rect, p.x as f32 + 0.5, p.y as f32 + 0.5)
    }
    fn target_point(&self, rect: Rect) -> Vec2 {
        let to = self.point(self.target, rect);
        if let Some(t) = self.moved_at {
            let u = ((self.clock - t) / 0.16).clamp(0.0, 1.0);
            self.point(self.from, rect).lerp(to, u)
        } else {
            to
        }
    }
    fn hostiles() -> [GridPos; 3] {
        [GridPos::new(6, 4), GridPos::new(2, 4), GridPos::new(4, 2)]
    }
}

struct Art {
    prison: Texture2D,
    reanimation: Texture2D,
}
impl Art {
    fn load() -> Self {
        let load = |bytes: &[u8]| {
            let t = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            prison: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-36/prison-atlas.png"
            )),
            reanimation: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-36/reanimation-atlas.png"
            )),
        }
    }
    fn draw(texture: &Texture2D, frame: usize, at: Vec2, size: Vec2, angle: f32, opacity: f32) {
        draw_texture_ex(
            texture,
            at.x - size.x * 0.5,
            at.y - size.y * 0.5,
            alpha(WHITE, opacity),
            DrawTextureParams {
                source: Some(Rect::new(frame as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(size),
                rotation: angle,
                pivot: Some(at),
                ..Default::default()
            },
        );
    }
    fn segment(s: &State, rect: Rect, i: usize) -> (Vec2, f32) {
        let (dx, dy) = OFFSETS[i];
        let direction = vec2(dx as f32, dy as f32).normalize();
        let at = vec2(CENTRE.x as f32 + 0.5, CENTRE.y as f32 + 0.5) + direction * 1.22;
        let local = vec2(at.x * 64.0, at.y * 64.0);
        let point = match s.direction {
            0 => vec2(SIZE - local.y, local.x),
            1 => vec2(local.y, SIZE - local.x),
            3 => vec2(SIZE, SIZE) - local,
            _ => local,
        };
        let aim = match s.direction {
            0 => std::f32::consts::FRAC_PI_2,
            1 => -std::f32::consts::FRAC_PI_2,
            3 => std::f32::consts::PI,
            _ => 0.0,
        };
        (
            point + vec2(rect.x, rect.y),
            (i as f32) * std::f32::consts::FRAC_PI_4 + aim,
        )
    }
    fn ground(&self, s: &State, rect: Rect) {
        if s.skill == 0 {
            if let Some(age) = s.age() {
                for i in 0..8 {
                    if s.segments[i] {
                        let (at, angle) = Self::segment(s, rect, i);
                        let reveal = ((age - i as f32 * 0.025) / 0.10).clamp(0.0, 1.0);
                        let frame = ((s.clock * 6.0) as usize + i) % 3;
                        Self::draw(&self.prison, frame, at, vec2(76.0, 76.0), angle, reveal);
                    }
                }
            }
            if let Some(t) = s.broken {
                let age = s.clock - t;
                if age < 0.25 {
                    let (at, angle) = Self::segment(s, rect, 4);
                    Self::draw(
                        &self.prison,
                        3 + ((age / 0.25 * 3.0) as usize).min(2),
                        at,
                        vec2(80.0, 80.0),
                        angle,
                        ((0.25 - age) / 0.06).clamp(0.0, 1.0),
                    );
                }
            }
        } else {
            if s.marked {
                Self::draw(
                    &self.reanimation,
                    (s.clock * 6.0) as usize % 3,
                    s.point(CENTRE, rect) + vec2(0.0, 19.0),
                    vec2(88.0, 66.0),
                    0.0,
                    1.0,
                );
            }
            if let Some(t) = s.birth {
                let age = s.clock - t;
                if age < 0.36 {
                    let frame = if age < 0.20 { 3 } else { 4 };
                    Self::draw(
                        &self.reanimation,
                        frame,
                        s.target_point(rect) + vec2(0.0, -5.0),
                        vec2(122.0, 142.0),
                        0.0,
                        ((0.36 - age) / 0.09).clamp(0.0, 1.0),
                    );
                }
            }
        }
    }
    fn ally_marker(&self, s: &State, rect: Rect) {
        if s.raised {
            Self::draw(
                &self.reanimation,
                5,
                s.target_point(rect) + vec2(23.0, -25.0),
                vec2(24.0, 24.0),
                0.0,
                1.0,
            );
        }
    }
}

fn target_body(s: &State, rect: Rect) {
    let p = s.target_point(rect);
    let draw = |corpse, opacity, offset: Vec2| {
        game_art::draw_world(
            Rect::new(p.x - 32.0 + offset.x, p.y - 32.0 + offset.y, 64.0, 64.0),
            Visual::Bestiary { number: 12, corpse },
            opacity,
        )
    };
    if s.skill == 1 && s.expired {
        draw(true, 1.0, Vec2::ZERO);
    } else if s.skill == 1 && s.raised {
        let age = s.clock - s.birth.unwrap();
        if age < 0.25 {
            draw(true, (1.0 - age / 0.25).clamp(0.0, 1.0), Vec2::ZERO);
        }
        let progress = (age / 0.30).clamp(0.0, 1.0);
        draw(
            false,
            0.25 + 0.75 * progress,
            vec2(0.0, 4.0 * (1.0 - progress)),
        );
    } else {
        draw(false, 1.0, Vec2::ZERO);
    }
}

fn scene(art: &Art, clip: &Clip, s: &State, rect: Rect, effects: bool) {
    let map = Map::from_ascii(MAPS[s.map]).unwrap();
    terrain(&map, s.direction, rect, false);
    if s.launched.is_none() {
        preview(&s.cells(), s.direction, rect, s.clock);
    }
    if effects {
        clip.apply_for(s.map, s.direction, rect);
        art.ground(s, rect);
        gl_use_default_material();
    }
    terrain(&map, s.direction, rect, true);
    target_body(s, rect);
    for (i, at) in State::hostiles().iter().enumerate() {
        let mut p = s.point(*at, rect);
        let victim = if s.map == 1 { 1 } else { 0 };
        if i == victim {
            if let Some(t) = s.strike {
                let age = s.clock - t;
                if age < 0.16 {
                    p.x += (age / 0.16 * std::f32::consts::PI).sin() * 3.0;
                }
            }
        }
        enemy(p);
        draw_text_bold(
            char::from(b'B' + i as u8).to_string(),
            p.x - 4.0,
            p.y - 30.0,
            13.0,
            TEXT,
        );
    }
    let player = s.point(PLAYER, rect);
    actor(player, caster());
    if effects {
        art.ally_marker(s, rect);
    }
    let p = s.target_point(rect);
    draw_text_bold("A", p.x - 4.0, p.y - 30.0, 13.0, TEXT);
    if let Some(age) = s.age() {
        if age < 0.7 {
            draw_text_bold(
                TITLES[s.skill],
                player.x - 57.0,
                player.y - 43.0 - age * 7.0,
                15.0,
                alpha(TEXT, 1.0 - age / 0.7),
            );
        }
    }
    let label = if !s.valid() {
        if s.skill == 0 {
            "Placement refusé · pourtour bloqué".to_string()
        } else {
            "Cible occultée par la cloison".to_string()
        }
    } else if s.launched.is_none() {
        "Aperçu clignotant · avant lancement".to_string()
    } else if s.skill == 0 {
        format!(
            "Cercle : {} segments · {} tours",
            s.segments.iter().filter(|v| **v).count(),
            s.remaining
        )
    } else if s.marked {
        "Marque conservée jusqu’à la mort".to_string()
    } else if s.raised {
        format!("Allié autonome · {} tours", s.remaining)
    } else {
        "Réanimation terminée".to_string()
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
    let clip = Clip::load_maps_with_violet(&MAPS, PLAYER, 3.0);
    let mut s = State::new(0, 0, 2, false);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold("LABORATOIRE / RITUELS OCCULTES", 22.0, 35.0, 24.0, TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 88.0 + i as f32 * 44.0, 285.0, 35.0),
                title,
                true,
            ) {
                s = State::new(i, s.map, s.direction, s.quiet);
            }
        }
        for (i, title) in ["Zone ouverte", "Pourtour bloqué", "Cloison pleine"]
            .iter()
            .enumerate()
        {
            if button(
                Rect::new(20.0, 198.0 + i as f32 * 44.0, 285.0, 35.0),
                title,
                true,
            ) {
                s = State::new(s.skill, i, s.direction, s.quiet);
            }
        }
        for (i, title) in DIRS.iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 146.0,
                    345.0 + (i / 2) as f32 * 42.0,
                    138.0,
                    34.0,
                ),
                title,
                true,
            ) {
                s = State::new(s.skill, s.map, i, s.quiet);
            }
        }
        for (i, title) in ["Brèche et mort préparées", "Attente sans événement"]
            .iter()
            .enumerate()
        {
            if button(
                Rect::new(20.0, 440.0 + i as f32 * 42.0, 285.0, 34.0),
                title,
                true,
            ) {
                s = State::new(s.skill, s.map, s.direction, i == 1);
            }
        }
        let label = if s.launched.is_some() && s.cooldown == 0 {
            "Scène jouée".to_string()
        } else if s.cooldown > 0 {
            format!("Recharge : {} tours", s.cooldown)
        } else {
            "Lancer".to_string()
        };
        if button(Rect::new(20.0, 546.0, 285.0, 34.0), &label, s.can_launch())
            || (is_key_pressed(KeyCode::Enter) && s.can_launch())
        {
            s.launch();
            paused = false;
        }
        if button(Rect::new(20.0, 591.0, 138.0, 34.0), "+1 tour", true)
            || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(166.0, 591.0, 138.0, 34.0), "Nouvel aperçu", true) {
            s = State::new(s.skill, s.map, s.direction, s.quiet);
        }
        if button(
            Rect::new(20.0, 636.0, 285.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        draw_text(
            format!("Tour {} · rang 11", s.turn),
            22.0,
            709.0,
            16.0,
            TEXT,
        );
        draw_text("Événements préparés, aucune IA.", 22.0, 751.0, 14.0, DIM);
        scene(&art, &clip, &s, Rect::new(410.0, 88.0, SIZE, SIZE), true);
        draw_text(
            format!("{} / {}-A01", 25 + s.skill, IDS[s.skill]),
            410.0,
            695.0,
            15.0,
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
    let clip = Clip::load_maps_with_violet(&MAPS, PLAYER, 3.0);
    let target = render_target(1280, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1280.0, 720.0))
    };
    let controls = [
        3, 14, 17, 23, 35, 47, 48, 54, 60, 65, 71, 72, 78, 84, 87, 100, 137, 138, 143,
    ];
    let cases = [
        (0, 2, false, "HAUT"),
        (0, 0, false, "DROITE"),
        (0, 1, false, "GAUCHE"),
        (0, 3, false, "BAS"),
        (1, 2, false, "OBSTACLE"),
        (2, 2, false, "CLOISON"),
        (0, 2, true, "ATTENTE"),
    ];
    let mut entries = Vec::new();
    for (map, direction, quiet, folder) in cases {
        let dir = output.join(folder);
        std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
        let mut states = [
            State::new(0, map, direction, quiet),
            State::new(1, map, direction, quiet),
        ];
        let mut timeline = Vec::new();
        for frame in 0..144 {
            for s in &mut states {
                s.clock = frame as f32 / 24.0;
                if frame == 17 {
                    s.launch();
                }
                if !quiet && TURNS.contains(&frame) {
                    s.advance();
                }
            }
            timeline.push(serde_json::json!({"frame":frame,"states":states.iter().map(|s|serde_json::json!({"turn":s.turn,"cooldown":s.cooldown,"remaining":s.remaining,"segments":s.segments,"marked":s.marked,"raised":s.raised,"expired":s.expired,"target":[s.target.x,s.target.y],"launched":s.launched.is_some()})).collect::<Vec<_>>()}));
            for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                set_camera(&camera);
                clear_background(INK);
                for (i, s) in states.iter().enumerate() {
                    let x = 32.0 + i as f32 * 616.0;
                    draw_text_bold(format!("{} · {}", 25 + i, TITLES[i]), x, 35.0, 23.0, TEXT);
                    draw_text(
                        format!("{}-A01 · {} · tour {}", IDS[i], DIRS[direction], s.turn),
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
        let coords = |at| {
            let p = position(at, direction);
            [p.x, p.y]
        };
        let m = Map::from_ascii(MAPS[map]).unwrap();
        let mut rows = vec![vec!['.'; 9]; 9];
        for y in 0..9 {
            for x in 0..9 {
                if m.blocks_vision(GridPos::new(x, y)) {
                    let p = position(GridPos::new(x, y), direction);
                    rows[p.y as usize][p.x as usize] = '#';
                }
            }
        }
        let views=(0..2).map(|i|{let s=State::new(i,map,direction,quiet);serde_json::json!({"id":IDS[i],"number":25+i,"title":TITLES[i],"variant":format!("{}-A01-{folder}",IDS[i]),"crop":[32+i*616,88,576,576],"valid":s.valid(),"player_cell":coords(PLAYER),"target_cell":coords(CENTRE),"affected_cells":s.cells().into_iter().map(coords).collect::<Vec<_>>()})}).collect::<Vec<_>>();
        entries.push(serde_json::json!({"folder":folder,"frames":144,"fps":24,"tile":64,"quiet":quiet,"control_frames":controls,"launch_frame":17,"turn_frames":if quiet {vec![]}else{TURNS.to_vec()},"capture_size":[1280,720],"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n"),"views":views}));
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
    fn eight_independent_segments_leave_the_centre_free() {
        let mut s = State::new(0, 0, 2, false);
        s.launch();
        assert_eq!(s.cells().len(), 8);
        assert!(!s.cells().contains(&CENTRE));
        assert!(s.segments.into_iter().all(|v| v));
    }
    #[test]
    fn blocked_perimeter_refuses_the_entire_circle_but_not_the_visible_mark() {
        let mut s = State::new(0, 1, 2, false);
        assert!(!s.can_launch());
        s.launch();
        assert!(s.launched.is_none());
        assert_eq!(s.cooldown, 0);
        assert!(State::new(1, 1, 2, false).can_launch());
    }
    #[test]
    fn one_broken_segment_creates_a_passage_and_leaves_seven() {
        let mut s = State::new(0, 0, 2, false);
        s.launch();
        s.advance();
        assert_eq!(s.segments.iter().filter(|v| **v).count(), 7);
        assert_eq!(s.target, CENTRE);
        s.advance();
        assert_eq!(s.target, GridPos::new(4, 5));
    }
    #[test]
    fn waiting_does_not_expire_circle_or_mark() {
        for skill in 0..2 {
            let mut s = State::new(skill, 0, 2, true);
            s.launch();
            s.clock = 600.0;
            assert_eq!(s.turn, 0);
            if skill == 0 {
                assert_eq!(s.remaining, 4);
            } else {
                assert!(s.marked && !s.raised);
            }
        }
    }
    #[test]
    fn circle_expires_after_four_explicit_turns() {
        let mut s = State::new(0, 0, 2, false);
        s.launch();
        for _ in 0..4 {
            s.advance();
        }
        assert_eq!(s.remaining, 0);
        assert!(!s.segments.into_iter().any(|v| v));
    }
    #[test]
    fn resurrection_consumes_mark_and_preserves_full_eleven_turn_duration_at_birth() {
        let mut s = State::new(1, 0, 2, false);
        s.launch();
        s.advance();
        assert!(s.marked);
        s.advance();
        assert!(!s.marked && s.raised);
        assert_eq!(s.remaining, 11);
        for _ in 0..11 {
            s.advance();
        }
        assert!(s.expired && !s.raised);
    }
    #[test]
    fn marked_target_without_a_death_keeps_its_mark() {
        let mut s = State::new(1, 0, 2, true);
        s.launch();
        for _ in 0..20 {
            s.advance();
        }
        assert!(s.marked && !s.raised);
    }
    #[test]
    fn ally_script_never_walks_into_the_partial_obstacle() {
        let mut s = State::new(1, 1, 2, false);
        s.launch();
        for _ in 0..3 {
            s.advance();
        }
        assert_eq!(s.target, GridPos::new(3, 4));
    }
    #[test]
    fn replay_uses_a_fresh_scene_instead_of_remarking_an_existing_ally() {
        let mut s = State::new(1, 0, 2, false);
        s.launch();
        for _ in 0..8 {
            s.advance();
        }
        assert_eq!(s.cooldown, 0);
        assert!(!s.can_launch());
        let remaining = s.remaining;
        s.launch();
        assert_eq!(s.remaining, remaining);
        assert!(State::new(1, 0, 2, false).can_launch());
    }
}
