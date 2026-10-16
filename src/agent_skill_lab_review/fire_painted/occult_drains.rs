//! ANIM-S11: two distinct drains. Prepared counters only, no combat or resource changes.
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::{Map, has_line_of_sight};

const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 7);
const TARGET: GridPos = GridPos::new(4, 4);
const IDS: [&str; 2] = ["HY05", "OC05"];
const TITLES: [&str; 2] = ["Ponction spirituelle", "Rayon vorace"];
const DIRS: [&str; 4] = ["Droite", "Gauche", "Haut", "Bas"];
const MAPS: [&str; 3] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#....#..#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#########\n#.......#\n#.......#\n#########",
];
const TURNS: [usize; 3] = [60, 88, 116];
const FRAMES: usize = 144;

struct State {
    skill: usize,
    map: usize,
    direction: usize,
    interruption: usize,
    clock: f32,
    launched: Option<f32>,
    pulse: Option<f32>,
    stopped: Option<f32>,
    channel: bool,
    moved: bool,
    turn: u16,
    cooldown: u16,
    resolutions: u16,
}
impl State {
    fn new(skill: usize, map: usize, direction: usize, interruption: usize) -> Self {
        Self {
            skill,
            map,
            direction,
            interruption,
            clock: 0.0,
            launched: None,
            pulse: None,
            stopped: None,
            channel: false,
            moved: false,
            turn: 0,
            cooldown: 0,
            resolutions: 0,
        }
    }
    fn player(&self) -> GridPos {
        if self.moved {
            GridPos::new(3, 7)
        } else {
            PLAYER
        }
    }
    fn valid(&self) -> bool {
        let map = Map::from_ascii(MAPS[self.map]).unwrap();
        let p = self.player();
        let distance = (p.x - TARGET.x).abs().max((p.y - TARGET.y).abs());
        distance <= i32::from(parameter_for(IDS[self.skill], "range_cells", 11))
            && has_line_of_sight(&map, p, TARGET, true)
    }
    fn can_launch(&self) -> bool {
        self.cooldown == 0 && self.valid()
    }
    fn launch(&mut self) {
        if !self.can_launch() {
            return;
        }
        self.launched = Some(self.clock);
        self.pulse = Some(self.clock);
        self.stopped = None;
        self.resolutions = 1;
        self.channel = self.skill == 1;
        self.cooldown = parameter_for(IDS[self.skill], "cooldown_turns", 11);
    }
    fn stop(&mut self) {
        if self.channel {
            self.channel = false;
            self.stopped = Some(self.clock);
            self.pulse = None;
        }
    }
    fn advance(&mut self) {
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        if self.turn == 1 && self.interruption == 1 && self.launched.is_some() {
            self.stop();
            self.moved = true;
        }
        if self.turn == 1 && self.interruption == 2 {
            self.stop();
        }
        if self.channel {
            let maximum = parameter_for("OC05", "maximum_resolutions", 11);
            if self.resolutions < maximum && self.valid() {
                self.resolutions += 1;
                self.pulse = Some(self.clock);
            } else {
                self.stop();
            }
        }
    }
    fn age(&self) -> Option<f32> {
        self.launched.map(|t| self.clock - t)
    }
    fn pulse_age(&self) -> Option<f32> {
        self.pulse.map(|t| self.clock - t)
    }
    fn active(&self) -> bool {
        if self.skill == 0 {
            self.age().is_some_and(|t| t < 0.48)
        } else {
            self.channel || self.stopped.is_some_and(|t| self.clock - t < 0.12)
        }
    }
    fn point(&self, at: GridPos, rect: Rect) -> Vec2 {
        let p = position(at, self.direction);
        cell(rect, p.x as f32 + 0.5, p.y as f32 + 0.5)
    }
    fn cells(&self) -> Vec<GridPos> {
        if self.valid() {
            vec![TARGET]
        } else {
            Vec::new()
        }
    }
    fn targets(&self) -> [GridPos; 4] {
        [
            TARGET,
            GridPos::new(3, 4),
            GridPos::new(6, 4),
            GridPos::new(4, 3),
        ]
    }
}

struct Art {
    spirit: Texture2D,
    ray: Texture2D,
    thick_spirit: bool,
    intense: bool,
}
impl Art {
    fn load() -> Self {
        let load = |bytes: &[u8]| {
            let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            texture.set_filter(FilterMode::Linear);
            texture
        };
        Self {
            thick_spirit: std::env::args().any(|arg| arg == "--ponction-epaisse"),
            intense: std::env::args().any(|arg| arg == "--drains-intenses"),
            spirit: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-33/spirit-atlas.png"
            )),
            ray: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-33/ray-atlas.png"
            )),
        }
    }
    fn variant(&self, skill: usize) -> &'static str {
        if self.intense {
            if skill == 0 { "HY05-A03" } else { "OC05-A02" }
        } else if skill == 1 {
            "OC05-A01"
        } else if self.thick_spirit {
            "HY05-A02"
        } else {
            "HY05-A01"
        }
    }
    fn draw(texture: &Texture2D, frame: usize, centre: Vec2, size: Vec2, angle: f32, opacity: f32) {
        draw_texture_ex(
            texture,
            centre.x - size.x * 0.5,
            centre.y - size.y * 0.5,
            alpha(WHITE, opacity),
            DrawTextureParams {
                source: Some(Rect::new(frame as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(size),
                rotation: angle,
                pivot: Some(centre),
                ..Default::default()
            },
        );
    }
    fn shadow(&self, s: &State, rect: Rect) {
        if s.skill != 0 {
            return;
        }
        if let Some(t) = s.age() {
            if t < 0.20 {
                let frame = ((t / 0.20 * 3.0) as usize).min(2);
                let width = if self.intense {
                    138.0 - 36.0 * (t / 0.20)
                } else {
                    92.0 - 26.0 * (t / 0.20)
                };
                Self::draw(
                    &self.spirit,
                    frame,
                    s.point(TARGET, rect) + vec2(0.0, 20.0),
                    vec2(width, width * 0.68),
                    0.0,
                    if self.intense { 1.0 } else { 0.9 },
                );
            }
        }
    }
    fn foreground(&self, s: &State, rect: Rect) {
        let victim = s.point(TARGET, rect) + vec2(0.0, -8.0);
        let caster = s.point(s.player(), rect) + vec2(0.0, -8.0);
        let delta = caster - victim;
        let angle = delta.y.atan2(delta.x);
        if s.skill == 1 && s.active() {
            let fade = if s.channel {
                0.90
            } else {
                0.90 * (1.0 - (s.clock - s.stopped.unwrap()) / 0.12).clamp(0.0, 1.0)
            };
            let frame = (s.clock * 9.0) as usize % 3;
            let centre = (victim + caster) * 0.5;
            let length = delta.length() * 256.0 / 224.0;
            if self.intense {
                Self::draw(
                    &self.ray,
                    frame,
                    centre,
                    vec2(length, 240.0),
                    angle,
                    fade * 0.36,
                );
                Self::draw(
                    &self.ray,
                    frame,
                    centre,
                    vec2(length, 184.0),
                    angle,
                    (fade / 0.90).min(1.0),
                );
                Self::draw(
                    &self.ray,
                    (frame + 1) % 3,
                    centre,
                    vec2(length, 120.0),
                    angle,
                    fade * 0.72,
                );
            } else {
                Self::draw(&self.ray, frame, centre, vec2(length, 58.0), angle, fade);
            }
        }
        if let Some(age) = s.pulse_age() {
            let delay = if s.skill == 0 { 0.16 } else { 0.0 };
            let t = age - delay;
            if (0.0..0.30).contains(&t) {
                let u = t / 0.30;
                let frame = 3 + ((u * 3.0) as usize).min(2);
                let texture = if s.skill == 0 {
                    &self.spirit
                } else {
                    &self.ray
                };
                if self.intense {
                    // Several moving drawings form one cosmetic stream, not extra resolutions.
                    let count = if s.skill == 0 { 6 } else { 4 };
                    let fade = (u / 0.08).clamp(0.0, 1.0) * ((1.0 - u) / 0.18).clamp(0.0, 1.0);
                    for i in (0..count).rev() {
                        let lag = i as f32 * 0.105;
                        if u < lag {
                            continue;
                        }
                        let progress = u - lag;
                        let scale = 1.0 - i as f32 * 0.07;
                        let size = if s.skill == 0 {
                            vec2(120.0 - 22.0 * u, 132.0 - 20.0 * u)
                        } else {
                            vec2(102.0 - 18.0 * u, 116.0 - 18.0 * u)
                        };
                        let frame = 3 + ((u * 3.0 + i as f32 * 0.5) as usize % 3);
                        let side = vec2(-delta.y, delta.x).normalize_or_zero()
                            * (u * std::f32::consts::PI + i as f32 * 1.7).sin()
                            * 5.0;
                        Self::draw(
                            texture,
                            frame,
                            victim + delta * progress + side,
                            size * scale,
                            angle,
                            fade * (1.0 - i as f32 * 0.07),
                        );
                    }
                    return;
                }
                let size = 52.0 - 16.0 * u;
                let dimensions = if s.skill == 0 && self.thick_spirit {
                    vec2(size * 1.5, size * 2.0)
                } else {
                    vec2(size, size)
                };
                let opacity = (u / 0.12).clamp(0.0, 1.0) * ((1.0 - u) / 0.12).clamp(0.0, 1.0);
                Self::draw(
                    texture,
                    frame,
                    victim + delta * u,
                    dimensions,
                    angle,
                    opacity,
                );
            }
        }
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
        art.shadow(s, rect);
        gl_use_default_material();
    }
    terrain(&map, s.direction, rect, true);
    for (i, at) in s.targets().iter().enumerate() {
        let p = s.point(*at, rect);
        enemy(p);
        draw_text_bold(
            char::from(b'A' + i as u8).to_string(),
            p.x - 4.0,
            p.y - 30.0,
            13.0,
            TEXT,
        );
    }
    let player = s.point(s.player(), rect);
    actor(player, caster());
    if effects {
        clip.apply_for(s.map, s.direction, rect);
        art.foreground(s, rect);
        gl_use_default_material();
    }
    if let Some(t) = s.age() {
        if t < 0.7 {
            draw_text_bold(
                TITLES[s.skill],
                player.x - 57.0,
                player.y - 43.0 - t * 7.0,
                15.0,
                alpha(TEXT, 1.0 - t / 0.7),
            );
        }
    }
    let label = if !s.valid() {
        "Cible occultée par la cloison".to_string()
    } else if s.launched.is_none() {
        "Aperçu clignotant · avant lancement".to_string()
    } else if s.skill == 1 {
        format!(
            "Drains préparés : {} / 3 · recharge {} tours",
            s.resolutions, s.cooldown
        )
    } else {
        format!("Ponction unique · recharge {} tours", s.cooldown)
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
    let mut s = State::new(0, 0, 2, 0);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold("LABORATOIRE / DRAINS OCCULTES", 22.0, 35.0, 24.0, TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 88.0 + i as f32 * 44.0, 285.0, 35.0),
                title,
                true,
            ) {
                s = State::new(i, s.map, s.direction, s.interruption);
            }
        }
        for (i, title) in ["Zone ouverte", "Obstacle partiel", "Cloison pleine"]
            .iter()
            .enumerate()
        {
            if button(
                Rect::new(20.0, 198.0 + i as f32 * 44.0, 285.0, 35.0),
                title,
                true,
            ) {
                s = State::new(s.skill, i, s.direction, s.interruption);
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
                s = State::new(s.skill, s.map, i, s.interruption);
            }
        }
        for (i, title) in [
            "Attendre : trois drains",
            "Déplacement au tour 1",
            "Énergie épuisée au tour 1",
        ]
        .iter()
        .enumerate()
        {
            if button(
                Rect::new(20.0, 440.0 + i as f32 * 40.0, 285.0, 34.0),
                title,
                true,
            ) {
                s = State::new(s.skill, s.map, s.direction, i);
            }
        }
        let launch = if s.cooldown > 0 {
            format!("Recharge : {} tours", s.cooldown)
        } else {
            "Lancer".to_string()
        };
        if button(Rect::new(20.0, 572.0, 285.0, 34.0), &launch, s.can_launch())
            || (is_key_pressed(KeyCode::Enter) && s.can_launch())
        {
            s.launch();
            paused = false;
        }
        if button(Rect::new(20.0, 619.0, 138.0, 34.0), "+1 tour", true)
            || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(166.0, 619.0, 138.0, 34.0), "Nouvel aperçu", true) {
            s = State::new(s.skill, s.map, s.direction, s.interruption);
        }
        if button(
            Rect::new(20.0, 666.0, 285.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        draw_text(
            format!("Tour {} · rang 11", s.turn),
            22.0,
            735.0,
            16.0,
            TEXT,
        );
        draw_text("Aucun PV ni énergie calculé.", 22.0, 770.0, 15.0, DIM);
        scene(&art, &clip, &s, Rect::new(410.0, 88.0, SIZE, SIZE), true);
        draw_text(
            format!("{} / {}", 23 + s.skill, art.variant(s.skill)),
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
        3, 14, 17, 19, 21, 24, 29, 35, 47, 59, 60, 64, 69, 87, 88, 94, 115, 116, 120, 125, 143,
    ];
    let cases = [
        (0, 2, 0, "HAUT"),
        (0, 0, 0, "DROITE"),
        (0, 1, 0, "GAUCHE"),
        (0, 3, 0, "BAS"),
        (1, 2, 0, "OBSTACLE"),
        (2, 2, 0, "CLOISON"),
        (0, 2, 1, "DEPLACEMENT"),
        (0, 2, 2, "ENERGIE"),
    ];
    let mut entries = Vec::new();
    for (map, direction, interruption, folder) in cases {
        let dir = output.join(folder);
        std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
        let mut states = [
            State::new(0, map, direction, interruption),
            State::new(1, map, direction, interruption),
        ];
        let mut timeline = Vec::new();
        for frame in 0..FRAMES {
            for s in &mut states {
                s.clock = frame as f32 / 24.0;
                if frame == 17 {
                    s.launch();
                }
                if TURNS.contains(&frame) {
                    s.advance();
                }
            }
            timeline.push(serde_json::json!({"frame":frame,"states":states.iter().map(|s|serde_json::json!({"turn":s.turn,"cooldown":s.cooldown,"resolutions":s.resolutions,"channel":s.channel,"launched":s.launched.is_some(),"visual_active":s.active(),"moved":s.moved})).collect::<Vec<_>>()}));
            for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                set_camera(&camera);
                clear_background(INK);
                for (i, s) in states.iter().enumerate() {
                    let x = 32.0 + i as f32 * 616.0;
                    draw_text_bold(format!("{} · {}", 23 + i, TITLES[i]), x, 35.0, 23.0, TEXT);
                    draw_text(
                        format!("{} · {} · tour {}", art.variant(i), DIRS[direction], s.turn),
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
        let views=(0..2).map(|i|serde_json::json!({"id":IDS[i],"number":23+i,"title":TITLES[i],"variant":format!("{}-{folder}",art.variant(i)),"crop":[32+i*616,88,576,576],"valid":State::new(i,map,direction,interruption).valid(),"expected_resolutions":states[i].resolutions,"player_cell":coords(PLAYER),"target_cell":coords(TARGET),"affected_cells":if map==2 {Vec::new()} else {vec![coords(TARGET)]}})).collect::<Vec<_>>();
        entries.push(serde_json::json!({"folder":folder,"frames":FRAMES,"fps":24,"tile":64,"control_frames":controls,"launch_frame":17,"turn_frames":TURNS,"interruption":interruption,"capture_size":[1280,720],"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n"),"views":views}));
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
    fn waiting_keeps_the_channel_without_repeating_drains_or_reducing_cooldown() {
        let mut s = State::new(1, 0, 2, 0);
        s.launch();
        s.clock = 600.0;
        assert!(s.active());
        assert_eq!(s.resolutions, 1);
        assert_eq!(s.cooldown, 1);
        assert_eq!(s.turn, 0);
    }
    #[test]
    fn the_single_siphon_ends_independently_of_turns() {
        let mut s = State::new(0, 0, 2, 0);
        s.launch();
        s.clock = 600.0;
        assert!(!s.active());
        assert_eq!(s.resolutions, 1);
        assert_eq!(s.cooldown, 1);
    }
    #[test]
    fn each_explicit_wait_adds_one_drain_until_three_and_then_breaks_the_link() {
        let mut s = State::new(1, 0, 2, 0);
        s.launch();
        for expected in [2, 3] {
            s.advance();
            assert!(s.channel);
            assert_eq!(s.resolutions, expected);
        }
        s.advance();
        assert!(!s.channel);
        s.clock = 1.0;
        assert!(!s.active());
        assert_eq!(s.resolutions, 3);
    }
    #[test]
    fn movement_or_insufficient_energy_stops_before_the_next_drain() {
        for interruption in [1, 2] {
            let mut s = State::new(1, 0, 2, interruption);
            s.launch();
            s.advance();
            assert!(!s.channel);
            assert_eq!(s.resolutions, 1);
            assert_eq!(s.moved, interruption == 1);
        }
    }
    #[test]
    fn cloison_refuses_both_casts_without_starting_cooldown_or_drain() {
        for skill in 0..2 {
            let mut s = State::new(skill, 2, 2, 0);
            assert!(!s.can_launch());
            s.launch();
            assert_eq!(s.resolutions, 0);
            assert_eq!(s.cooldown, 0);
            assert!(s.launched.is_none());
        }
    }
    #[test]
    fn four_directions_preserve_the_target_and_do_not_target_the_caster() {
        for d in 0..4 {
            let mut s = State::new(1, 1, d, 0);
            s.launch();
            assert_eq!(s.cells(), vec![TARGET]);
            assert!(!s.cells().contains(&s.player()));
            assert_eq!(s.resolutions, 1);
        }
    }
}
