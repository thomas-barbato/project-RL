//! ANIM-S14: prepared attacks of approved forms; no campaign combat resolution.
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;

const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 4);
const TARGET: GridPos = GridPos::new(4, 3);
const LANDING: GridPos = GridPos::new(4, 2);
const IDS: [&str; 6] = [
    "M01-GRIFFE",
    "M01-LACERATION",
    "M01-BOND",
    "TR61-COUP",
    "TR61-ECRASEMENT",
    "TR61-ONDE",
];
const TITLES: [&str; 6] = [
    "Griffe",
    "Lacération",
    "Bond",
    "Coup lourd",
    "Écrasement",
    "Onde de choc",
];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const MAPS: [&str; 3] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#....#..#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#..###..#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
];
const TURN_FRAMES: [usize; 3] = [48, 66, 84];

struct State {
    skill: usize,
    map: usize,
    direction: usize,
    clock: f32,
    started: Option<f32>,
    turn: u16,
    cooldown: u16,
    bleed: u16,
    stun: u16,
    player: GridPos,
    hits: Vec<GridPos>,
}
impl State {
    fn new(skill: usize, map: usize, direction: usize) -> Self {
        Self {
            skill,
            map,
            direction,
            clock: 0.0,
            started: None,
            turn: 0,
            cooldown: 0,
            bleed: 0,
            stun: 0,
            player: PLAYER,
            hits: Vec::new(),
        }
    }
    fn monsters(&self) -> Vec<GridPos> {
        let map = Map::from_ascii(MAPS[self.map]).unwrap();
        [
            GridPos::new(3, 3),
            TARGET,
            GridPos::new(5, 3),
            GridPos::new(3, 4),
            GridPos::new(5, 4),
            GridPos::new(4, 5),
            GridPos::new(4, 1),
            GridPos::new(6, 6),
        ]
        .into_iter()
        .filter(|p| map.is_walkable(*p) && !(self.skill == 2 && *p == TARGET))
        .collect()
    }
    fn footprint(&self) -> Vec<GridPos> {
        let map = Map::from_ascii(MAPS[self.map]).unwrap();
        if self.skill == 2
            && [TARGET, LANDING]
                .into_iter()
                .any(|p| !map.is_walkable(p) || self.monsters().contains(&p))
        {
            return Vec::new();
        }
        let cells = match self.skill {
            1 => vec![GridPos::new(3, 3), TARGET, GridPos::new(5, 3)],
            2 => vec![TARGET, LANDING],
            5 => (-1..=1)
                .flat_map(|dy| (-1..=1).map(move |dx| GridPos::new(PLAYER.x + dx, PLAYER.y + dy)))
                .filter(|p| *p != PLAYER)
                .collect(),
            _ => vec![TARGET],
        };
        cells.into_iter().filter(|p| map.is_walkable(*p)).collect()
    }
    fn can_launch(&self) -> bool {
        if self.started.is_some() || self.cooldown > 0 {
            return false;
        }
        if self.skill == 2 {
            let map = Map::from_ascii(MAPS[self.map]).unwrap();
            [TARGET, LANDING]
                .into_iter()
                .all(|p| map.is_walkable(p) && !self.monsters().contains(&p))
        } else {
            !self.footprint().is_empty()
        }
    }
    fn launch(&mut self) {
        if !self.can_launch() {
            return;
        }
        self.started = Some(self.clock);
        self.cooldown = [0, 3, 3, 0, 4, 4][self.skill];
        if self.skill == 2 {
            self.player = LANDING;
            return;
        }
        let cells = self.footprint();
        self.hits = self
            .monsters()
            .into_iter()
            .filter(|p| cells.contains(p))
            .collect();
        // Fixed successful statuses demonstrate previously approved durations, never RNG.
        if self.skill < 2 && !self.hits.is_empty() {
            self.bleed = 3;
        }
        if self.skill == 5 && !self.hits.is_empty() {
            self.stun = 1;
        }
    }
    fn advance(&mut self) {
        if self.started.is_none() {
            return;
        }
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        self.bleed = self.bleed.saturating_sub(1);
        self.stun = self.stun.saturating_sub(1);
    }
    fn age(&self) -> Option<f32> {
        self.started.map(|t| self.clock - t)
    }
    fn motion(&self, rect: Rect) -> Vec2 {
        let start = point(PLAYER, self.direction, rect);
        let end = point(self.player, self.direction, rect);
        let Some(age) = self.age() else {
            return start;
        };
        if self.skill == 2 {
            let p = (age / 0.26).clamp(0.0, 1.0);
            return start.lerp(end, p) - vec2(0.0, (p * std::f32::consts::PI).sin() * 16.0);
        }
        let impulse = if age < 0.22 {
            (age / 0.22 * std::f32::consts::PI).sin()
        } else {
            0.0
        };
        let aim = (point(TARGET, self.direction, rect) - start).normalize();
        if self.skill == 4 {
            start + aim * impulse * 5.0 - vec2(0.0, impulse * 7.0)
        } else if self.skill == 5 {
            start + vec2(0.0, impulse * 3.0)
        } else {
            start + aim * impulse * 9.0
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
    claws: Texture2D,
    impact: Texture2D,
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
            bodies: [
                load(include_bytes!(
                    "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-39/predator-64.png"
                )),
                load(include_bytes!(
                    "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-39/colossus-64.png"
                )),
            ],
            claws: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-40/claws-atlas.png"
            )),
            impact: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-40/impact-atlas.png"
            )),
            blood: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-40/blood-atlas.png"
            )),
        }
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
    fn ground(&self, s: &State, rect: Rect) {
        let Some(age) = s.age() else {
            return;
        };
        let (center, start, duration, size, opacity) = match s.skill {
            2 => (
                point(LANDING, s.direction, rect) + vec2(0.0, 22.0),
                0.24,
                0.26,
                vec2(76.0, 50.0),
                0.60,
            ),
            3 => (
                point(TARGET, s.direction, rect) + vec2(0.0, 9.0),
                0.06,
                0.30,
                vec2(96.0, 86.0),
                0.95,
            ),
            4 => (
                point(TARGET, s.direction, rect),
                0.07,
                0.36,
                vec2(132.0, 132.0),
                1.0,
            ),
            5 => (
                point(PLAYER, s.direction, rect) + vec2(0.0, 17.0),
                0.03,
                0.40,
                vec2(230.0, 210.0),
                1.0,
            ),
            _ => return,
        };
        if (start..start + duration).contains(&age) {
            Self::sheet(
                &self.impact,
                center,
                size,
                (((age - start) / duration) * 6.0) as usize,
                opacity,
                0.0,
            );
        }
    }
    fn front(&self, s: &State, rect: Rect) {
        let Some(age) = s.age() else {
            return;
        };
        if s.skill < 2 && (0.03..0.33).contains(&age) {
            let source = point(PLAYER, s.direction, rect);
            let target = point(TARGET, s.direction, rect);
            Self::sheet(
                &self.claws,
                source.lerp(target, 0.75),
                if s.skill == 0 {
                    vec2(98.0, 90.0)
                } else {
                    vec2(126.0, 200.0)
                },
                (((age - 0.03) / 0.30) * 6.0) as usize,
                1.0,
                angle(s.direction),
            );
        }
        if s.skill != 2 && (0.08..0.54).contains(&age) {
            for &at in &s.hits {
                Self::sheet(
                    &self.blood,
                    point(at, s.direction, rect)
                        + vec2(angle(s.direction).cos(), angle(s.direction).sin()) * 24.0
                        + vec2(0.0, 4.0),
                    vec2(80.0, 80.0),
                    (((age - 0.08) / 0.46) * 6.0) as usize,
                    1.0,
                    angle(s.direction),
                );
            }
        }
        for (i, &at) in s.hits.iter().enumerate() {
            let p = point(at, s.direction, rect);
            if s.bleed > 0 && age > 0.5 {
                Self::sheet(
                    &self.blood,
                    p + vec2(0.0, 14.0),
                    vec2(36.0, 43.0),
                    4 + ((s.clock * 3.0 + i as f32) as usize % 2),
                    0.95,
                    std::f32::consts::FRAC_PI_2,
                );
            }
            if s.stun > 0 {
                let q = p + vec2(0.0, -37.0);
                draw_ellipse_lines(q.x, q.y, 13.0, 4.0, 0.0, 1.6, alpha(TEXT, 0.90));
                for n in 0..3 {
                    let a = s.clock * 4.0 + n as f32 * 2.094;
                    draw_circle(q.x + a.cos() * 13.0, q.y + a.sin() * 4.0, 2.6, TEXT);
                }
            }
        }
    }
    fn body(&self, s: &State, rect: Rect) {
        let at = s.motion(rect);
        let ground = if s.skill == 2 {
            at + vec2(
                0.0,
                (s.age().unwrap_or(0.0) / 0.26)
                    .clamp(0.0, 1.0)
                    .mul_add(std::f32::consts::PI, 0.0)
                    .sin()
                    * 16.0,
            )
        } else {
            at
        };
        draw_ellipse(
            ground.x,
            ground.y + 29.0,
            if s.skill < 3 { 18.0 } else { 23.0 },
            4.0,
            0.0,
            alpha(BLACK, 0.3),
        );
        draw_texture_ex(
            &self.bodies[usize::from(s.skill >= 3)],
            at.x - 32.0,
            at.y - 32.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(64.0, 64.0)),
                ..Default::default()
            },
        );
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
    for at in s.monsters() {
        let mut p = point(at, s.direction, rect);
        if effects && s.hits.contains(&at) {
            if let Some(age) = s.age() {
                if (0.08..0.30).contains(&age) {
                    p.x += ((age - 0.08) * 60.0).sin() * 3.0;
                }
            }
        }
        enemy(p);
    }
    art.body(s, rect);
    if effects {
        clip.apply_for(s.map, s.direction, rect);
        art.front(s, rect);
        gl_use_default_material();
    }
    if let Some(age) = s.age() {
        if age < 0.65 {
            let p = s.motion(rect);
            draw_text_bold(
                TITLES[s.skill],
                p.x - 45.0,
                p.y - 47.0 - age * 6.0,
                15.0,
                alpha(TEXT, 1.0 - age / 0.65),
            );
        }
    }
    let label = if s.started.is_none() && !s.can_launch() {
        "Trajet bloqué".into()
    } else {
        format!(
            "Recharge {} · saignement {} · assommé {}",
            s.cooldown, s.bleed, s.stun
        )
    };
    draw_rectangle(
        rect.x + 5.0,
        rect.y + SIZE - 25.0,
        450.0,
        20.0,
        alpha(INK, 0.90),
    );
    draw_text(label, rect.x + 12.0, rect.y + SIZE - 10.0, 14.0, TEXT);
}

pub(crate) async fn run() {
    request_new_screen_size(1280.0, 820.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let clip = Clip::load_maps(&MAPS, PLAYER);
    let mut s = State::new(0, 0, 2);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(INK);
        draw_text_bold("Attaques transformées · ANIM-S14", 20.0, 33.0, 23.0, TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 75.0 + i as f32 * 38.0, 320.0, 32.0),
                title,
                true,
            ) {
                s = State::new(i, s.map, s.direction);
            }
        }
        for (i, title) in DIRS.iter().enumerate() {
            if button(
                Rect::new(20.0, 319.0 + i as f32 * 35.0, 320.0, 30.0),
                title,
                true,
            ) {
                s = State::new(s.skill, s.map, i);
            }
        }
        for (i, title) in ["Pièce ouverte", "Obstacle partiel", "Front bloqué"]
            .iter()
            .enumerate()
        {
            if button(
                Rect::new(20.0, 475.0 + i as f32 * 36.0, 320.0, 30.0),
                title,
                true,
            ) {
                s = State::new(s.skill, i, s.direction);
            }
        }
        if button(
            Rect::new(20.0, 598.0, 151.0, 34.0),
            "Lancer",
            s.can_launch(),
        ) || (is_key_pressed(KeyCode::Enter) && s.can_launch())
        {
            s.launch();
            paused = false;
        }
        if button(
            Rect::new(182.0, 598.0, 158.0, 34.0),
            "+1 tour",
            s.started.is_some(),
        ) || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(20.0, 645.0, 151.0, 34.0), "Nouvel aperçu", true) {
            s = State::new(s.skill, s.map, s.direction);
        }
        if button(
            Rect::new(182.0, 645.0, 158.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        draw_text(
            "Impacts préparés, sans dégâts ni IA.",
            20.0,
            723.0,
            13.0,
            DIM,
        );
        draw_text(
            "Saignement : 3 tours · assommé : 1 tour.",
            20.0,
            750.0,
            13.0,
            DIM,
        );
        scene(&art, &clip, &s, Rect::new(410.0, 88.0, SIZE, SIZE), true);
        draw_text(
            format!("{} / {}-A01 · tour {}", 29 + s.skill, IDS[s.skill], s.turn),
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
        3, 14, 17, 19, 21, 23, 25, 27, 30, 35, 47, 48, 65, 66, 83, 84, 95,
    ];
    let mut entries = Vec::new();
    for (map, direction, quiet, case) in [
        (0, 2, false, "HAUT"),
        (0, 0, false, "DROITE"),
        (0, 1, false, "GAUCHE"),
        (0, 3, false, "BAS"),
        (1, 2, false, "OBSTACLE"),
        (2, 2, false, "BLOQUE"),
        (0, 2, true, "ATTENTE"),
    ] {
        for (pair, skills) in [[0, 3], [1, 4], [2, 5]].into_iter().enumerate() {
            let folder = format!("{case}-{}", pair + 1);
            let dir = output.join(&folder);
            std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
            let mut states = skills.map(|i| State::new(i, map, direction));
            let mut timeline = Vec::new();
            for frame in 0..96 {
                for s in &mut states {
                    s.clock = frame as f32 / 24.0;
                    if frame == 17 {
                        s.launch();
                    }
                    if !quiet && TURN_FRAMES.contains(&frame) {
                        s.advance();
                    }
                }
                timeline.push(serde_json::json!({"frame":frame,"states":states.iter().map(|s|serde_json::json!({"turn":s.turn,"cooldown":s.cooldown,"bleed":s.bleed,"stun":s.stun,"player":[s.player.x,s.player.y],"hits":s.hits.iter().map(|p|[p.x,p.y]).collect::<Vec<_>>(),"launched":s.started.is_some()})).collect::<Vec<_>>()}));
                for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                    set_camera(&camera);
                    clear_background(INK);
                    for (j, s) in states.iter().enumerate() {
                        let x = 32.0 + j as f32 * 616.0;
                        draw_text_bold(
                            format!("{} · {}", 29 + s.skill, TITLES[s.skill]),
                            x,
                            35.0,
                            23.0,
                            TEXT,
                        );
                        draw_text(
                            format!("{}-A01 · {case} · tour {}", IDS[s.skill], s.turn),
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
            entries.push(serde_json::json!({"folder":folder,"case":case,"frames":96,"fps":24,"tile":64,"quiet":quiet,"launch_frame":17,"turn_frames":if quiet{vec![]}else{TURN_FRAMES.to_vec()},"control_frames":controls,"capture_size":[1280,720],"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n"),"views":skills.into_iter().enumerate().map(|(j,i)|serde_json::json!({"id":IDS[i],"number":29+i,"title":TITLES[i],"variant":format!("{}-A01-{case}",IDS[i]),"crop":[32+j*616,88,576,576]})).collect::<Vec<_>>()}));
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
    fn attacks_only_affect_the_prepared_footprint() {
        for (skill, count) in [(0, 1), (1, 3), (2, 0), (3, 1), (4, 1), (5, 6)] {
            let mut s = State::new(skill, 0, 2);
            s.launch();
            assert_eq!(s.hits.len(), count);
            assert!(!s.hits.contains(&GridPos::new(6, 6)));
        }
    }
    #[test]
    fn blocked_jump_refuses_without_movement_or_cooldown() {
        let mut s = State::new(2, 2, 2);
        assert!(s.footprint().is_empty());
        s.launch();
        assert!(s.started.is_none());
        assert_eq!(s.player, PLAYER);
        assert_eq!(s.cooldown, 0);
    }
    #[test]
    fn jump_changes_logical_position_at_launch_and_causes_no_hit() {
        let mut s = State::new(2, 0, 2);
        s.launch();
        assert_eq!(s.player, LANDING);
        assert!(s.hits.is_empty());
        assert_eq!(s.cooldown, 3);
    }
    #[test]
    fn walls_remove_area_cells() {
        let mut s = State::new(1, 1, 2);
        s.launch();
        assert_eq!(s.hits.len(), 2);
        assert!(!s.hits.contains(&GridPos::new(5, 3)));
    }
    #[test]
    fn waiting_preserves_turn_based_statuses() {
        for skill in [0, 5] {
            let mut s = State::new(skill, 0, 2);
            s.launch();
            s.clock = 100.0;
            assert_eq!((s.bleed, s.stun), if skill == 0 { (3, 0) } else { (0, 1) });
        }
    }
    #[test]
    fn statuses_expire_only_on_their_explicit_turn() {
        let mut s = State::new(1, 0, 2);
        s.launch();
        s.advance();
        s.advance();
        assert_eq!(s.bleed, 1);
        s.advance();
        assert_eq!(s.bleed, 0);
        let mut s = State::new(5, 0, 2);
        s.launch();
        s.advance();
        assert_eq!(s.stun, 0);
        assert_eq!(s.cooldown, 3);
    }
}
