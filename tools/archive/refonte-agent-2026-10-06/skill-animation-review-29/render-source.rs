//! ANIM-S10: occult impacts; prepared visual fixtures, never a combat resolver.
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::{Map, has_line_of_sight};
use std::collections::{BTreeSet, VecDeque};

const SIZE: f32 = 576.0;
const FLIGHT: f32 = 0.18;
const IMPACT_TIME: f32 = 0.22;
const BLAST: f32 = 0.55;
const CENTRE: GridPos = GridPos::new(4, 4);
const PLAYER: GridPos = GridPos::new(4, 7);
const IDS: [&str; 2] = ["A01", "A02"];
const TITLES: [&str; 2] = ["Trait arcanique", "Déflagration occulte"];
const DIRS: [&str; 4] = ["Droite", "Gauche", "Haut", "Bas"];
const MAPS: [&str; 3] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#....#..#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#########\n#.......#\n#.......#\n#########",
];
const TURNS: [usize; 4] = [48, 65, 82, 99];

fn area(map: &Map, centre: GridPos) -> Vec<GridPos> {
    let mut seen = BTreeSet::from([(centre.x, centre.y)]);
    let mut queue = VecDeque::from([centre]);
    while let Some(at) = queue.pop_front() {
        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = GridPos::new(at.x + dx, at.y + dy);
            if (next.x - centre.x).abs() <= 1
                && (next.y - centre.y).abs() <= 1
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
    direction: usize,
    close: bool,
    rank: u16,
    clock: f32,
    launched: Option<f32>,
    turn: u16,
    cooldown: u16,
    contacts: [u16; 6],
}
impl State {
    fn new(skill: usize, map: usize, direction: usize, close: bool, rank: u16) -> Self {
        Self {
            skill,
            map,
            direction,
            close,
            rank,
            clock: 0.0,
            launched: None,
            turn: 0,
            cooldown: 0,
            contacts: [0; 6],
        }
    }
    fn player(&self) -> GridPos {
        if self.close && self.map != 2 {
            GridPos::new(4, 5)
        } else {
            PLAYER
        }
    }
    fn target(&self) -> GridPos {
        if self.skill == 1 && self.map == 2 {
            GridPos::new(4, 6)
        } else {
            CENTRE
        }
    }
    fn targets(&self) -> [GridPos; 6] {
        [
            CENTRE,
            GridPos::new(3, 4),
            if self.map == 1 {
                GridPos::new(6, 3)
            } else {
                GridPos::new(5, 4)
            },
            GridPos::new(4, 3),
            GridPos::new(6, 4),
            if self.map == 2 {
                GridPos::new(4, 6)
            } else {
                GridPos::new(4, 2)
            },
        ]
    }
    fn valid_target(&self) -> bool {
        let map = Map::from_ascii(MAPS[self.map]).unwrap();
        let origin = self.player();
        let target = self.target();
        let distance = (origin.x - target.x).abs().max((origin.y - target.y).abs());
        distance <= i32::from(parameter_for(IDS[self.skill], "range_cells", self.rank))
            && map.is_walkable(target)
            && has_line_of_sight(&map, origin, target, true)
    }
    fn cells(&self) -> Vec<GridPos> {
        if !self.valid_target() {
            return Vec::new();
        }
        if self.skill == 0 {
            vec![self.target()]
        } else {
            area(&Map::from_ascii(MAPS[self.map]).unwrap(), self.target())
        }
    }
    fn can_launch(&self) -> bool {
        self.cooldown == 0 && self.valid_target()
    }
    fn launch(&mut self) {
        if !self.can_launch() {
            return;
        }
        self.launched = Some(self.clock);
        self.cooldown = parameter_for(IDS[self.skill], "cooldown_turns", self.rank);
        let cells = self.cells();
        self.contacts = self.targets().map(|p| u16::from(cells.contains(&p)));
    }
    fn advance(&mut self) {
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
    }
    fn age(&self) -> Option<f32> {
        self.launched.map(|t| (self.clock - t).max(0.0))
    }
    fn duration(&self) -> f32 {
        if self.skill == 0 {
            FLIGHT + IMPACT_TIME
        } else {
            BLAST
        }
    }
    fn active(&self) -> bool {
        self.age().is_some_and(|t| t < self.duration())
    }
    fn point(&self, at: GridPos, rect: Rect) -> Vec2 {
        let p = position(at, self.direction);
        cell(rect, p.x as f32 + 0.5, p.y as f32 + 0.5)
    }
    fn actor_point(&self, index: usize, rect: Rect) -> Vec2 {
        let mut p = self.point(self.targets()[index], rect);
        if self.contacts[index] > 0 {
            if let Some(age) = self.age() {
                let start = if self.skill == 0 { FLIGHT } else { 0.10 };
                let bump = age - start;
                if (0.0..0.18).contains(&bump) {
                    p.x += (bump / 0.18 * std::f32::consts::TAU).sin() * 2.5;
                }
            }
        }
        p
    }
}

struct Art {
    bolt: Texture2D,
    blast: Texture2D,
}
impl Art {
    fn load() -> Self {
        let load = |data: &[u8]| {
            let texture = Texture2D::from_file_with_format(data, Some(ImageFormat::Png));
            texture.set_filter(FilterMode::Linear);
            texture
        };
        Self {
            bolt: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-29/bolt-atlas.png"
            )),
            blast: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-29/burst-atlas.png"
            )),
        }
    }
    fn draw(
        texture: &Texture2D,
        frame: usize,
        at: Vec2,
        anchor: Vec2,
        size: f32,
        angle: f32,
        opacity: f32,
    ) {
        draw_texture_ex(
            texture,
            at.x - anchor.x * size / 256.0,
            at.y - anchor.y * size / 256.0,
            alpha(WHITE, opacity),
            DrawTextureParams {
                source: Some(Rect::new(frame as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(vec2(size, size)),
                rotation: angle,
                pivot: Some(at),
                ..Default::default()
            },
        );
    }
    fn bolt(&self, state: &State, rect: Rect) {
        let age = state.age().unwrap();
        let origin = state.point(state.player(), rect) + vec2(0.0, -8.0);
        let destination = state.point(state.target(), rect) + vec2(0.0, -8.0);
        let heading = destination - origin;
        if age < FLIGHT {
            let frame = ((age / FLIGHT * 3.0) as usize).min(2);
            Self::draw(
                &self.bolt,
                frame,
                origin + heading * (age / FLIGHT),
                vec2(232.0, 136.0),
                72.0,
                heading.y.atan2(heading.x),
                0.98,
            );
        } else {
            let t = age - FLIGHT;
            let frame = 3 + ((t / IMPACT_TIME * 3.0) as usize).min(2);
            Self::draw(
                &self.bolt,
                frame,
                destination,
                vec2(128.0, 128.0),
                64.0,
                heading.y.atan2(heading.x),
                0.9 * ((IMPACT_TIME - t) / 0.06).clamp(0.0, 1.0),
            );
        }
    }
    fn blast(&self, state: &State, rect: Rect) {
        let age = state.age().unwrap();
        let frame = ((age / BLAST * 6.0) as usize).min(5);
        let anchor = if frame < 2 {
            vec2(128.0, 170.0)
        } else {
            vec2(128.0, 128.0)
        };
        let rise = 18.0 * (1.0 - (age / 0.22).clamp(0.0, 1.0));
        Self::draw(
            &self.blast,
            frame,
            state.point(state.target(), rect) + vec2(0.0, rise),
            anchor,
            224.0,
            0.0,
            0.83 * ((BLAST - age) / 0.10).clamp(0.0, 1.0),
        );
    }
}

fn scene(art: &Art, clip: &Clip, state: &State, rect: Rect, effects: bool) {
    let map = Map::from_ascii(MAPS[state.map]).unwrap();
    terrain(&map, state.direction, rect, false);
    if state.launched.is_none() {
        preview(&state.cells(), state.direction, rect, state.clock);
    }
    let ground_effect = state.skill == 1 && state.active() && state.age().unwrap() < BLAST / 6.0;
    if effects && ground_effect {
        clip.apply_for(state.map, state.direction, rect);
        art.blast(state, rect);
        gl_use_default_material();
    }
    terrain(&map, state.direction, rect, true);
    for i in 0..6 {
        enemy(state.actor_point(i, rect));
    }
    let player = state.point(state.player(), rect);
    actor(player, caster());
    if effects && state.active() && !ground_effect {
        clip.apply_for(state.map, state.direction, rect);
        if state.skill == 0 {
            art.bolt(state, rect);
        } else {
            art.blast(state, rect);
        }
        gl_use_default_material();
    }
    for i in 0..6 {
        let p = state.actor_point(i, rect);
        draw_text_bold(
            char::from(b'A' + i as u8).to_string(),
            p.x - 4.0,
            p.y - 30.0,
            13.0,
            TEXT,
        );
    }
    if let Some(age) = state.age() {
        if age < 0.7 {
            draw_text_bold(
                TITLES[state.skill],
                player.x - 57.0,
                player.y - 43.0 - age * 7.0,
                15.0,
                alpha(TEXT, 1.0 - age / 0.7),
            );
        }
    }
    let label = if !state.valid_target() {
        "Cible occultée par la cloison".to_string()
    } else if state.launched.is_none() {
        "Aperçu clignotant · avant lancement".to_string()
    } else if state.active() {
        "Impact bref · contacts préparés".to_string()
    } else {
        format!("Impact terminé · recharge {} tours", state.cooldown)
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
    let clip = Clip::load_maps(&MAPS, PLAYER);
    let mut state = State::new(0, 0, 2, false, 11);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold("LABORATOIRE / IMPACTS OCCULTES", 22.0, 35.0, 24.0, TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 88.0 + i as f32 * 44.0, 285.0, 35.0),
                title,
                true,
            ) {
                state = State::new(i, state.map, state.direction, state.close, state.rank);
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
                state = State::new(state.skill, i, state.direction, state.close, state.rank);
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
                state = State::new(state.skill, state.map, i, state.close, state.rank);
            }
        }
        if button(
            Rect::new(20.0, 440.0, 285.0, 34.0),
            if state.close {
                "Agent proche"
            } else {
                "Agent à distance"
            },
            true,
        ) {
            state = State::new(
                state.skill,
                state.map,
                state.direction,
                !state.close,
                state.rank,
            );
        }
        for (i, rank) in [1, 11, 18].iter().enumerate() {
            if button(
                Rect::new(20.0 + i as f32 * 98.0, 487.0, 90.0, 34.0),
                &format!("Rang {rank}"),
                true,
            ) {
                state = State::new(state.skill, state.map, state.direction, state.close, *rank);
            }
        }
        let label = if state.cooldown > 0 {
            format!("Recharge : {} tours", state.cooldown)
        } else if !state.valid_target() {
            "Cible inaccessible".to_string()
        } else {
            "Lancer".to_string()
        };
        if button(
            Rect::new(20.0, 537.0, 285.0, 34.0),
            &label,
            state.can_launch(),
        ) || (is_key_pressed(KeyCode::Enter) && state.can_launch())
        {
            state.launch();
            paused = false;
        }
        if button(Rect::new(20.0, 582.0, 138.0, 34.0), "+1 tour", true)
            || is_key_pressed(KeyCode::Space)
        {
            state.advance();
        }
        if button(Rect::new(166.0, 582.0, 138.0, 34.0), "Nouvel aperçu", true) {
            state = State::new(
                state.skill,
                state.map,
                state.direction,
                state.close,
                state.rank,
            );
        }
        if button(
            Rect::new(20.0, 627.0, 285.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        draw_text(
            format!("Tour {} · rang {}", state.turn, state.rank),
            22.0,
            710.0,
            16.0,
            TEXT,
        );
        draw_text("Aucun dégât ni statut calculé.", 22.0, 747.0, 15.0, DIM);
        scene(
            &art,
            &clip,
            &state,
            Rect::new(410.0, 88.0, SIZE, SIZE),
            true,
        );
        draw_text(
            format!("{} / {}-A01", 21 + state.skill, IDS[state.skill]),
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
    let mut bytes = Vec::with_capacity(image.bytes.len());
    for row in image.bytes.chunks_exact(stride).rev() {
        bytes.extend_from_slice(row);
    }
    for pixel in bytes.as_chunks_mut::<4>().0 {
        pixel[3] = 255;
    }
    std::fs::write(path, bytes).unwrap();
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
    let controls = [3, 14, 17, 19, 21, 23, 26, 29, 35, 47, 50, 66, 83, 100, 111];
    let cases = [
        (0, 2, false, "HAUT"),
        (0, 0, false, "DROITE"),
        (0, 1, false, "GAUCHE"),
        (0, 3, false, "BAS"),
        (1, 2, false, "OBSTACLE"),
        (2, 2, false, "CLOISON"),
        (0, 2, true, "PROCHE"),
    ];
    let mut entries = Vec::new();
    for (map, direction, close, folder) in cases {
        let dir = output.join(folder);
        std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
        let mut states = [
            State::new(0, map, direction, close, 11),
            State::new(1, map, direction, close, 11),
        ];
        let mut timeline = Vec::new();
        for frame in 0..112 {
            for state in &mut states {
                state.clock = frame as f32 / 24.0;
                if frame == 17 {
                    state.launch();
                }
                if TURNS.contains(&frame) {
                    state.advance();
                }
            }
            timeline.push(serde_json::json!({"frame":frame,"states":states.iter().map(|s|serde_json::json!({"turn":s.turn,"cooldown":s.cooldown,"contacts":s.contacts,"player_contacts":0,"launched":s.launched.is_some(),"visual_active":s.active()})).collect::<Vec<_>>()}));
            for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                set_camera(&camera);
                clear_background(INK);
                for (index, state) in states.iter().enumerate() {
                    let x = 32.0 + index as f32 * 616.0;
                    draw_text_bold(
                        format!("{} · {}", 21 + index, TITLES[index]),
                        x,
                        35.0,
                        23.0,
                        TEXT,
                    );
                    draw_text(
                        format!(
                            "{}-A01 · {} · tour {}",
                            IDS[index], DIRS[direction], state.turn
                        ),
                        x,
                        60.0,
                        14.0,
                        DIM,
                    );
                    scene(
                        &art,
                        &clip,
                        state,
                        Rect::new(x, 88.0, SIZE, SIZE),
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
        let views=(0..2).map(|index| {
            let original=State::new(index,map,direction,close,11);
            serde_json::json!({"id":IDS[index],"number":21+index,"title":TITLES[index],"variant":format!("{}-A01-{folder}",IDS[index]),"crop":[32+index*616,88,576,576],"valid":original.valid_target(),"expected_contacts":states[index].contacts,"player_cell":coords(original.player()),"target_cells":original.targets().map(coords),"affected_cells":original.cells().into_iter().map(coords).collect::<Vec<_>>()})
        }).collect::<Vec<_>>();
        entries.push(serde_json::json!({"folder":folder,"frames":112,"fps":24,"tile":64,"control_frames":controls,"launch_frame":17,"turn_frames":TURNS,"capture_size":[1280,720],"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n"),"views":views}));
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
    fn waiting_ends_impacts_without_advancing_cooldowns_or_contacts() {
        for skill in 0..2 {
            let mut s = State::new(skill, 0, 2, false, 11);
            s.launch();
            let contacts = s.contacts;
            s.clock = 600.0;
            assert!(!s.active());
            assert_eq!(s.turn, 0);
            assert_eq!(s.cooldown, [2, 4][skill]);
            assert_eq!(s.contacts, contacts);
        }
    }
    #[test]
    fn target_is_singular_and_the_zone_is_three_by_three() {
        let mut bolt = State::new(0, 0, 2, false, 11);
        bolt.launch();
        assert_eq!(bolt.contacts, [1, 0, 0, 0, 0, 0]);
        let mut blast = State::new(1, 0, 2, false, 11);
        assert_eq!(blast.cells().len(), 9);
        blast.launch();
        assert_eq!(blast.contacts, [1, 1, 1, 1, 0, 0]);
    }
    #[test]
    fn full_cloison_refuses_bolt_and_protects_far_targets_from_blast() {
        let mut bolt = State::new(0, 2, 2, false, 11);
        assert!(!bolt.can_launch());
        bolt.launch();
        assert!(bolt.launched.is_none());
        assert_eq!(bolt.cooldown, 0);
        let mut blast = State::new(1, 2, 2, false, 11);
        assert!(blast.can_launch());
        blast.launch();
        assert_eq!(blast.contacts, [0, 0, 0, 0, 0, 1]);
        assert!(blast.cells().iter().all(|p| p.y > 5));
    }
    #[test]
    fn blocked_cells_are_excluded_and_four_orientations_preserve_contacts() {
        for direction in 0..4 {
            let mut s = State::new(1, 1, direction, false, 11);
            assert!(!s.cells().contains(&GridPos::new(5, 4)));
            s.launch();
            assert_eq!(s.contacts, [1, 1, 0, 1, 0, 0]);
        }
    }
    #[test]
    fn cooldown_blocks_recast_until_explicit_turns() {
        let mut s = State::new(1, 0, 2, true, 11);
        s.launch();
        assert!(s.cells().contains(&s.player()));
        let old = s.launched;
        s.clock = 900.0;
        s.launch();
        assert_eq!(s.launched, old);
        for _ in 0..4 {
            s.advance();
        }
        assert!(s.can_launch());
        s.launch();
        assert_eq!(s.launched, Some(900.0));
        assert_eq!(s.contacts, [1, 1, 1, 1, 0, 0]);
    }
}
