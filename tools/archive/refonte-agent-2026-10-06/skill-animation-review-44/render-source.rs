//! ANIM-S17: native body and metamorphosis proposals. No combat/stat/save mutations.
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;

const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 4);
const IDS: [&str; 1] = ["TR65"];
const TITLES: [&str; 1] = ["Abysse"];
const DIRS: [&str; 4] = ["Droite", "Gauche", "Haut", "Bas"];
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#....#..#\n#....#..#\n#.......#\n#.......#\n#########",
];
const TURNS: [usize; 22] = [
    48, 60, 72, 84, 90, 94, 98, 102, 106, 110, 126, 130, 134, 138, 142, 146, 150, 154, 158, 162,
    166, 170,
];
const ENTER: f32 = 0.38;
const EXIT: f32 = 0.30;

struct State {
    skill: usize,
    map: usize,
    direction: usize,
    quiet: bool,
    clock: f32,
    turn: u16,
    cooldown: u16,
    remaining: u16,
    active: bool,
    started: Option<f32>,
    returned: Option<f32>,
    renewal: bool,
    launches: u16,
}
impl State {
    fn new(skill: usize, map: usize, direction: usize, quiet: bool) -> Self {
        Self {
            skill,
            map,
            direction,
            quiet,
            clock: 0.0,
            turn: 0,
            cooldown: 0,
            remaining: 0,
            active: false,
            started: None,
            returned: None,
            renewal: false,
            launches: 0,
        }
    }
    fn duration(&self) -> u16 {
        parameter_for(IDS[self.skill], "duration_rank_turns", 11)
    }
    fn can_launch(&self) -> bool {
        self.cooldown == 0 && Map::from_ascii(MAPS[self.map]).unwrap().is_walkable(PLAYER)
    }
    fn launch(&mut self) {
        if !self.can_launch() {
            return;
        }
        self.renewal = self.active;
        self.active = true;
        self.remaining = self.duration();
        self.cooldown = parameter_for(IDS[self.skill], "cooldown_turns", 11);
        self.started = Some(self.clock);
        self.returned = None;
        self.launches += 1;
    }
    fn advance(&mut self) {
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        if self.active {
            self.remaining = self.remaining.saturating_sub(1);
            if self.remaining == 0 {
                self.active = false;
                self.returned = Some(self.clock);
            }
        }
    }
    fn point(&self, rect: Rect) -> Vec2 {
        let at = position(PLAYER, self.direction);
        cell(rect, at.x as f32 + 0.5, at.y as f32 + 0.5)
    }
    fn form_opacity(&self) -> f32 {
        if self.active {
            if self.renewal {
                1.0
            } else {
                ((self.clock - self.started.unwrap()) / 0.25).clamp(0.0, 1.0)
            }
        } else if let Some(t) = self.returned {
            (1.0 - (self.clock - t) / EXIT).clamp(0.0, 1.0)
        } else {
            0.0
        }
    }
    fn transition(&self) -> Option<(usize, f32)> {
        if let Some(t) = self.returned {
            let age = self.clock - t;
            if age < EXIT {
                return Some((
                    5 - ((age / EXIT * 6.0) as usize).min(5),
                    (1.0 - age / EXIT).clamp(0.0, 1.0),
                ));
            }
        }
        if let Some(t) = self.started {
            let age = self.clock - t;
            if age < ENTER {
                return Some((((age / ENTER * 6.0) as usize).min(5), 1.0));
            }
        }
        None
    }
}

struct Art {
    bodies: [Texture2D; 1],
    effects: [Texture2D; 1],
}
impl Art {
    fn load() -> Self {
        let load = |bytes: &[u8]| {
            let t = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            bodies: [load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-44/abysse-64.png"
            ))],
            effects: [load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-44/abysse_fx-atlas.png"
            ))],
        }
    }
    fn transition(&self, s: &State, rect: Rect, front: bool) {
        if let Some((frame, opacity)) = s.transition() {
            let at = s.point(rect);
            let size = vec2(168.0, 178.0);
            draw_texture_ex(
                &self.effects[s.skill],
                at.x - size.x * 0.5,
                at.y + 28.0 - size.y * 420.0 / 512.0,
                alpha(WHITE, opacity * if front { 0.24 } else { 0.95 }),
                DrawTextureParams {
                    source: Some(Rect::new(frame as f32 * 256.0, 0.0, 256.0, 256.0)),
                    dest_size: Some(size),
                    ..Default::default()
                },
            );
        }
    }
    fn body(&self, s: &State, rect: Rect) {
        let at = s.point(rect);
        let opacity = s.form_opacity();
        if opacity < 1.0 {
            game_art::draw_world(
                Rect::new(at.x - 32.0, at.y - 32.0, 64.0, 64.0),
                human(),
                1.0 - opacity,
            );
        }
        if opacity > 0.0 {
            draw_ellipse(
                at.x,
                at.y + 29.0,
                if s.skill == 0 { 18.0 } else { 23.0 },
                4.0,
                0.0,
                alpha(BLACK, opacity * 0.30),
            );
            draw_texture_ex(
                &self.bodies[s.skill],
                at.x - 32.0,
                at.y - 32.0,
                alpha(WHITE, opacity),
                DrawTextureParams {
                    dest_size: Some(vec2(64.0, 64.0)),
                    ..Default::default()
                },
            );
        }
    }
}
fn human() -> Visual {
    game_art::wear(
        game_art::player(Some("core:metamorphe"), PlayerWeaponPose::Unarmed, None),
        [("core:body_armor".into(), "core:tenue_metamorphe".into())],
    )
}
fn scene(art: &Art, clip: &Clip, s: &State, rect: Rect, effects: bool) {
    let map = Map::from_ascii(MAPS[s.map]).unwrap();
    terrain(&map, s.direction, rect, false);
    if s.started.is_none() {
        preview(&[PLAYER], s.direction, rect, s.clock);
    }
    if effects {
        clip.apply_for(s.map, s.direction, rect);
        art.transition(s, rect, false);
        gl_use_default_material();
    }
    terrain(&map, s.direction, rect, true);
    for at in [GridPos::new(6, 4), GridPos::new(2, 4), GridPos::new(4, 2)] {
        if map.is_walkable(at) {
            let p = position(at, s.direction);
            enemy(cell(rect, p.x as f32 + 0.5, p.y as f32 + 0.5));
        }
    }
    art.body(s, rect);
    if effects {
        clip.apply_for(s.map, s.direction, rect);
        art.transition(s, rect, true);
        gl_use_default_material();
    }
    if let Some(t) = s.started {
        let age = s.clock - t;
        if age < 0.7 {
            let p = s.point(rect);
            draw_text_bold(
                TITLES[s.skill],
                p.x - 65.0,
                p.y - 45.0 - age * 7.0,
                15.0,
                alpha(TEXT, 1.0 - age / 0.7),
            );
        }
    }
    let label = if s.active {
        format!("Forme : {} tours · recharge {}", s.remaining, s.cooldown)
    } else {
        format!("Forme humaine · recharge {}", s.cooldown)
    };
    draw_rectangle(
        rect.x + 5.0,
        rect.y + SIZE - 25.0,
        365.0,
        20.0,
        alpha(INK, 0.9),
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
    let mut s = State::new(0, 0, 2, false);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(INK);
        draw_text_bold("Transformations · ANIM-S17", 20.0, 33.0, 23.0, TEXT);
        draw_text(
            "Corps et transitions à valider · 64 px",
            20.0,
            59.0,
            14.0,
            DIM,
        );
        for (i, title) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 95.0 + i as f32 * 45.0, 320.0, 34.0),
                title,
                true,
            ) {
                s = State::new(i, s.map, s.direction, s.quiet);
            }
        }
        for (i, title) in DIRS.iter().enumerate() {
            if button(
                Rect::new(20.0, 202.0 + i as f32 * 39.0, 320.0, 32.0),
                title,
                true,
            ) {
                s = State::new(s.skill, s.map, i, s.quiet);
            }
        }
        for (i, title) in ["Pièce ouverte", "Mur proche"].iter().enumerate() {
            if button(
                Rect::new(20.0, 380.0 + i as f32 * 41.0, 320.0, 34.0),
                title,
                true,
            ) {
                s = State::new(s.skill, i, s.direction, s.quiet);
            }
        }
        let label = if s.cooldown > 0 {
            format!("Recharge : {} tours", s.cooldown)
        } else if s.active {
            "Renouveler".into()
        } else {
            "Lancer".into()
        };
        if button(Rect::new(20.0, 483.0, 320.0, 34.0), &label, s.can_launch())
            || (is_key_pressed(KeyCode::Enter) && s.can_launch())
        {
            s.launch();
            paused = false;
        }
        if button(Rect::new(20.0, 529.0, 151.0, 34.0), "+1 tour", true)
            || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(182.0, 529.0, 158.0, 34.0), "Nouvel aperçu", true) {
            s = State::new(s.skill, s.map, s.direction, s.quiet);
        }
        if button(
            Rect::new(20.0, 575.0, 320.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        draw_text(
            format!("Tour {} · rang 11 · durée 22 tours", s.turn),
            20.0,
            655.0,
            14.0,
            TEXT,
        );
        draw_text(
            "Transition cosmétique, aucune IA ni dégâts.",
            20.0,
            690.0,
            13.0,
            DIM,
        );
        draw_text(
            "Les attaques propres seront revues séparément.",
            20.0,
            715.0,
            13.0,
            DIM,
        );
        scene(&art, &clip, &s, Rect::new(410.0, 88.0, SIZE, SIZE), true);
        draw_text(
            format!(
                "{} / {}-A01 · corps {}-BODY02",
                43 + s.skill,
                IDS[s.skill],
                IDS[s.skill]
            ),
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
        3, 14, 17, 19, 21, 23, 25, 27, 35, 47, 48, 60, 90, 110, 115, 119, 125, 126, 130, 154, 166,
        170, 174, 179,
    ];
    let cases = [
        (0, 2, false, false, "HAUT"),
        (0, 0, false, false, "DROITE"),
        (0, 1, false, false, "GAUCHE"),
        (0, 3, false, false, "BAS"),
        (1, 2, false, false, "MUR"),
        (0, 2, true, false, "ATTENTE"),
        (0, 2, false, true, "RENOUVELLEMENT"),
    ];
    let mut entries = Vec::new();
    for (map, direction, quiet, renew, folder) in cases {
        let dir = output.join(folder);
        std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
        let mut states = [State::new(0, map, direction, quiet)];
        let mut timeline = Vec::new();
        for frame in 0..180 {
            for s in &mut states {
                s.clock = frame as f32 / 24.0;
                if frame == 17 {
                    s.launch();
                }
                if !quiet && TURNS.contains(&frame) {
                    s.advance();
                }
                if renew && frame == 115 {
                    s.launch();
                }
            }
            timeline.push(serde_json::json!({"frame":frame,"states":states.iter().map(|s|serde_json::json!({"turn":s.turn,"cooldown":s.cooldown,"remaining":s.remaining,"active":s.active,"launches":s.launches,"renewal":s.renewal})).collect::<Vec<_>>()}));
            for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                set_camera(&camera);
                clear_background(INK);
                for (i, s) in states.iter().enumerate() {
                    let x = 32.0 + i as f32 * 616.0;
                    draw_text_bold(format!("{} · {}", 43 + i, TITLES[i]), x, 35.0, 23.0, TEXT);
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
        let mut rows = vec![vec!['.'; 9]; 9];
        let m = Map::from_ascii(MAPS[map]).unwrap();
        for y in 0..9 {
            for x in 0..9 {
                if m.blocks_vision(GridPos::new(x, y)) {
                    let at = position(GridPos::new(x, y), direction);
                    rows[at.y as usize][at.x as usize] = '#';
                }
            }
        }
        entries.push(serde_json::json!({"folder":folder,"frames":180,"fps":24,"tile":64,"quiet":quiet,"renew":renew,"control_frames":controls,"launch_frame":17,"renewal_frame":if renew {Some(115)}else{None},"turn_frames":if quiet {vec![]}else{TURNS.to_vec()},"capture_size":[640,720],"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n"),"views":(0..1).map(|i|serde_json::json!({"id":IDS[i],"number":43+i,"title":TITLES[i],"variant":format!("{}-A01-{folder}",IDS[i]),"body_variant":format!("{}-BODY02",IDS[i]),"crop":[32+i*616,88,576,576],"player_cell":[4,4],"duration_turns":22})).collect::<Vec<_>>()}));
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
    fn abysse_form_activate_for_twenty_two_turns_and_renew_without_a_human_flash() {
        for skill in 0..1 {
            let mut s = State::new(skill, 0, 2, false);
            s.launch();
            assert!(s.active);
            assert_eq!((s.remaining, s.cooldown), (22, 10));
            for _ in 0..10 {
                s.advance();
            }
            s.clock = 2.0;
            s.launch();
            assert_eq!(s.remaining, 22);
            assert_eq!(s.form_opacity(), 1.0);
            assert!(s.renewal);
        }
    }
}
