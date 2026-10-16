//! FR06-A01: frozen-source snapshots and one-shot shatter. No GameState or damage.
use super::*;
const PLAYER: GridPos = GridPos::new(4, 7);
const ORIGIN: GridPos = GridPos::new(4, 4);
const BURST: f32 = 0.42;
const TURN_FRAMES: [usize; 3] = [48, 65, 82];
const MAPS: [&str; 3] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#########",
    "#########\n#.......#\n#.......#\n#...#...#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
];
fn footprint(map: &Map, origin: GridPos) -> Vec<GridPos> {
    [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .map(|(x, y)| GridPos::new(origin.x + x, origin.y + y))
        .filter(|&p| map.is_walkable(p))
        .collect()
}
struct State {
    map: usize,
    direction: usize,
    single: bool,
    rank: u16,
    clock: f32,
    launched: Option<f32>,
    turn: u16,
    cooldown: u16,
    gel: [u16; 6],
    pool: Vec<usize>,
    contacts: [u16; 6],
}
impl State {
    fn new(map: usize, direction: usize, single: bool, rank: u16) -> Self {
        Self {
            map,
            direction,
            single,
            rank,
            clock: 0.0,
            launched: None,
            turn: 0,
            cooldown: 0,
            gel: if single {
                [0, 3, 0, 0, 0, 3]
            } else {
                [3, 3, 3, 0, 0, 3]
            },
            pool: Vec::new(),
            contacts: [0; 6],
        }
    }
    fn visible(index: usize) -> bool {
        index < 5
    }
    fn targets(&self) -> [GridPos; 6] {
        [
            GridPos::new(3, 4),
            ORIGIN,
            GridPos::new(4, 5),
            GridPos::new(if self.map == 1 { 6 } else { 5 }, 4),
            GridPos::new(if self.map == 1 { 6 } else { 5 }, 5),
            GridPos::new(7, 4),
        ]
    }
    fn sources(&self) -> Vec<usize> {
        (0..6)
            .filter(|&i| Self::visible(i) && self.gel[i] > 0)
            .collect()
    }
    fn cells(&self) -> Vec<GridPos> {
        let map = Map::from_ascii(MAPS[self.map]).unwrap();
        let targets = self.targets();
        let sources = if self.launched.is_some() {
            self.pool.clone()
        } else {
            self.sources()
        };
        let mut seen = BTreeSet::new();
        for i in sources {
            for p in footprint(&map, targets[i]) {
                seen.insert((p.x, p.y));
            }
        }
        seen.into_iter().map(|(x, y)| GridPos::new(x, y)).collect()
    }
    fn can_launch(&self) -> bool {
        self.cooldown == 0 && !self.sources().is_empty()
    }
    fn launch(&mut self) {
        if !self.can_launch() {
            return;
        }
        self.pool = self.sources();
        self.launched = Some(self.clock);
        self.cooldown = parameter_for("FR06", "cooldown_turns", self.rank);
        self.contacts = [0; 6];
        let map = Map::from_ascii(MAPS[self.map]).unwrap();
        let targets = self.targets();
        // Count prepared local contacts, not damage packets, actual defenses or death.
        for &source in &self.pool {
            let zone = footprint(&map, targets[source]);
            for (i, at) in targets.iter().enumerate() {
                if zone.contains(at) {
                    self.contacts[i] += 1;
                }
            }
        }
        for &source in &self.pool {
            self.gel[source] = 0;
        }
    }
    fn advance(&mut self) {
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        for n in &mut self.gel {
            *n = n.saturating_sub(1);
        }
    }
    fn age(&self) -> Option<f32> {
        self.launched.map(|time| (self.clock - time).max(0.0))
    }
    fn active(&self) -> bool {
        self.age().is_some_and(|age| age < BURST)
    }
    fn point(&self, at: GridPos, rect: Rect) -> Vec2 {
        let p = position(at, self.direction);
        cell(rect, p.x as f32 + 0.5, p.y as f32 + 0.5)
    }
    fn actor_point(&self, index: usize, rect: Rect) -> Vec2 {
        let mut p = self.point(self.targets()[index], rect);
        if self.contacts[index] > 0 {
            if let Some(age) = self.age() {
                if age < 0.18 {
                    p.x += (age / 0.18 * std::f32::consts::TAU).sin() * 2.5;
                }
            }
        }
        p
    }
}
struct Art {
    burst: Texture2D,
    ice: super::Art,
    animated_gel: Option<super::gel_status::GelArt>,
}
impl Art {
    fn load() -> Self {
        let burst = Texture2D::from_file_with_format(
            include_bytes!(
                "../../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-26/burst-atlas.png"
            ),
            Some(ImageFormat::Png),
        );
        burst.set_filter(FilterMode::Linear);
        Self {
            burst,
            ice: super::Art::load(),
            animated_gel: std::env::args()
                .any(|arg| arg == "--gel-status-v2")
                .then(super::gel_status::GelArt::load),
        }
    }
    fn variant(&self) -> &'static str {
        if self.animated_gel.is_some() {
            "FR06-A02"
        } else {
            "FR06-A01"
        }
    }
    fn gel(&self, at: Vec2, time: f32, phase: usize, front: bool) {
        if let Some(ice) = &self.animated_gel {
            ice.draw(at, time, phase * 2, front);
        } else {
            self.ice.ice_feet(at, front);
        }
    }
    fn shatter(&self, p: Vec2, age: f32, index: usize, direction: usize) {
        let frame = ((age / BURST * 6.0) as usize).min(5);
        let size = 160.0;
        let angle = match direction {
            0 => std::f32::consts::FRAC_PI_2,
            1 => -std::f32::consts::FRAC_PI_2,
            3 => std::f32::consts::PI,
            _ => 0.0,
        } + index as f32 * 1.4;
        draw_texture_ex(
            &self.burst,
            p.x - size / 2.0,
            p.y - size / 2.0,
            alpha(WHITE, 0.72 * ((BURST - age) / 0.12).clamp(0.0, 1.0)),
            DrawTextureParams {
                source: Some(Rect::new(frame as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(vec2(size, size)),
                rotation: angle,
                pivot: Some(p),
                ..Default::default()
            },
        );
    }
}
fn scene(art: &Art, clip: &Clip, state: &State, rect: Rect, effects: bool) {
    let map = Map::from_ascii(MAPS[state.map]).unwrap();
    terrain(&map, state.direction, rect, false);
    if state.launched.is_none() {
        preview(&state.cells(), state.direction, rect, state.clock);
    }
    if effects && state.active() {
        clip.apply_for(state.map, state.direction, rect);
        for &i in &state.pool {
            art.shatter(
                state.point(state.targets()[i], rect),
                state.age().unwrap(),
                i,
                state.direction,
            );
        }
        gl_use_default_material();
    }
    if effects {
        for i in 0..6 {
            if State::visible(i) && state.gel[i] > 0 {
                clip.apply_for(state.map, state.direction, rect);
                art.gel(state.actor_point(i, rect), state.clock, i, false);
                gl_use_default_material();
            }
        }
    }
    terrain(&map, state.direction, rect, true);
    for i in 0..6 {
        if !State::visible(i) {
            continue;
        }
        let p = state.actor_point(i, rect);
        enemy(p);
        if effects && state.gel[i] > 0 {
            clip.apply_for(state.map, state.direction, rect);
            art.gel(p, state.clock, i, true);
            gl_use_default_material();
        }
        draw_text_bold(
            char::from(b'A' + i as u8).to_string(),
            p.x - 4.0,
            p.y - 30.0,
            13.0,
            TEXT,
        );
    }
    let p = state.point(PLAYER, rect);
    actor(p, caster());
    if let Some(age) = state.age() {
        if age < 0.7 {
            draw_text_bold(
                "Explosion cristalline",
                p.x - 60.0,
                p.y - 43.0 - age * 7.0,
                15.0,
                alpha(TEXT, 1.0 - age / 0.7),
            );
        }
    }
    let info = if state.launched.is_none() {
        format!("{} sources gelées visibles · aperçu", state.sources().len())
    } else {
        format!(
            "{} détonations · Gel consommé · recharge {}",
            state.pool.len(),
            state.cooldown
        )
    };
    draw_rectangle(
        rect.x + 7.0,
        rect.y + rect.h - 27.0,
        rect.w - 14.0,
        22.0,
        alpha(INK, 0.94),
    );
    draw_text(info, rect.x + 14.0, rect.y + rect.h - 11.0, 14.0, TEXT);
}
pub(crate) async fn run() {
    request_new_screen_size(1280.0, 900.0);
    let art = Art::load();
    let clip = Clip::load_maps(&MAPS, ORIGIN);
    let mut s = State::new(0, 2, false, 11);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold(
            "LABORATOIRE / EXPLOSION CRISTALLINE",
            22.0,
            35.0,
            24.0,
            TEXT,
        );
        for (i, title) in ["Sources proches", "Cloison pleine", "Obstacle local"]
            .iter()
            .enumerate()
        {
            if button(
                Rect::new(20.0, 88.0 + i as f32 * 44.0, 285.0, 35.0),
                title,
                true,
            ) {
                s = State::new(i, s.direction, s.single, s.rank);
            }
        }
        if button(
            Rect::new(20.0, 242.0, 285.0, 35.0),
            if s.single {
                "Une source"
            } else {
                "Trois sources"
            },
            true,
        ) {
            s = State::new(s.map, s.direction, !s.single, s.rank);
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
                s = State::new(s.map, i, s.single, s.rank);
            }
        }
        for (i, rank) in [1, 11, 18].iter().enumerate() {
            if button(
                Rect::new(20.0 + i as f32 * 98.0, 418.0, 90.0, 34.0),
                &format!("Rang {rank}"),
                true,
            ) {
                s = State::new(s.map, s.direction, s.single, *rank);
            }
        }
        let label = if s.cooldown > 0 {
            format!("Recharge : {} tours", s.cooldown)
        } else if s.sources().is_empty() {
            "Aucune cible gelée visible".to_string()
        } else {
            "Lancer".to_string()
        };
        if button(Rect::new(20.0, 475.0, 285.0, 34.0), &label, s.can_launch())
            || (is_key_pressed(KeyCode::Enter) && s.can_launch())
        {
            s.launch();
            paused = false;
        }
        if button(Rect::new(20.0, 520.0, 138.0, 34.0), "+1 tour", true)
            || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(166.0, 520.0, 138.0, 34.0), "Nouvel aperçu", true) {
            s = State::new(s.map, s.direction, s.single, s.rank);
        }
        if button(
            Rect::new(20.0, 565.0, 285.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        draw_text(
            format!("Tour {} · rang {}", s.turn, s.rank),
            22.0,
            650.0,
            16.0,
            TEXT,
        );
        draw_text("Sources et contacts préparés.", 22.0, 691.0, 15.0, DIM);
        draw_text("Aucun dégât ni décès calculé.", 22.0, 714.0, 15.0, DIM);
        scene(&art, &clip, &s, Rect::new(410.0, 88.0, SIZE, SIZE), true);
        draw_text(format!("19 / {}", art.variant()), 410.0, 695.0, 15.0, DIM);
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
    request_new_screen_size(720.0, 720.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let clip = Clip::load_maps(&MAPS, ORIGIN);
    let target = render_target(720, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 720.0, 720.0))
    };
    let cases = [
        (0, 2, false, "HAUT"),
        (0, 0, false, "DROITE"),
        (0, 1, false, "GAUCHE"),
        (0, 3, false, "BAS"),
        (1, 2, false, "CLOISON"),
        (2, 2, false, "OBSTACLE"),
        (0, 2, true, "UNIQUE"),
    ];
    let controls = [3, 14, 17, 19, 21, 24, 27, 28, 35, 47, 50, 66, 83, 95];
    let mut entries = Vec::new();
    for (map, direction, single, folder) in cases {
        let dir = output.join(folder);
        std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
        let mut s = State::new(map, direction, single, 11);
        let mut timeline = Vec::new();
        for frame in 0..96 {
            s.clock = frame as f32 / 24.0;
            if frame == 17 {
                s.launch();
            }
            if TURN_FRAMES.contains(&frame) {
                s.advance();
            }
            timeline.push(serde_json::json!({"frame":frame,"turn":s.turn,"cooldown":s.cooldown,"gel":s.gel,"sources":s.pool,"contacts":s.contacts,"visual_active":s.active(),"hidden_is_source":s.pool.contains(&5)}));
            for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                set_camera(&camera);
                clear_background(INK);
                draw_text_bold("19 · Explosion cristalline", 72.0, 35.0, 23.0, TEXT);
                draw_text(
                    format!("{} · {} · tour {}", art.variant(), DIRS[direction], s.turn),
                    72.0,
                    60.0,
                    14.0,
                    DIM,
                );
                scene(
                    &art,
                    &clip,
                    &s,
                    Rect::new(72.0, 88.0, SIZE, SIZE),
                    control == 0,
                );
                super::export(
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
        if art.animated_gel.is_some() {
            let mut preview_state = State::new(map, direction, single, 11);
            for i in 0..48 {
                preview_state.clock = i as f32 / 24.0;
                set_camera(&camera);
                clear_background(INK);
                scene(
                    &art,
                    &clip,
                    &preview_state,
                    Rect::new(72.0, 88.0, SIZE, SIZE),
                    true,
                );
                super::export(&target, &dir.join(format!("preview-idle-{i:02}.rgba")));
            }
            set_default_camera();
        }
        std::fs::write(
            dir.join("timeline.json"),
            serde_json::to_vec_pretty(&timeline).unwrap(),
        )
        .map_err(|e| e.to_string())?;
        let mut rows = vec![vec!['.'; 9]; 9];
        let m = Map::from_ascii(MAPS[map]).unwrap();
        let coords = |at| {
            let p = position(at, direction);
            [p.x, p.y]
        };
        for y in 0..9 {
            for x in 0..9 {
                if m.blocks_vision(GridPos::new(x, y)) {
                    let p = position(GridPos::new(x, y), direction);
                    rows[p.y as usize][p.x as usize] = '#';
                }
            }
        }
        let original = State::new(map, direction, single, 11);
        entries.push(serde_json::json!({"number":19,"id":"FR06","title":"Explosion cristalline","variant":format!("{}-{folder}",art.variant()),"folder":folder,"direction":DIRS[direction],"crop":[72,88,576,576],"frames":96,"fps":24,"tile":64,"single":single,
            "control_frames":controls,"launch_frame":17,"end_frame":28,"turn_frames":TURN_FRAMES,"burst_seconds":BURST,"capture_size":[720,720],
            "map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n"),"affected_cells":original.cells().into_iter().map(coords).collect::<Vec<_>>(),
            "visible_targets":original.targets().into_iter().enumerate().filter(|(i,_)|State::visible(*i)).map(|(i,at)|serde_json::json!({"label":char::from(b'A'+i as u8).to_string(),"cell":coords(at),"initial_gel":original.gel[i]})).collect::<Vec<_>>(),
            "fixture":"Only visible frozen sources are selected. Contact counters are examples, not damage. No death, new Gel, RNG, hidden-actor display or campaign integration."}));
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
    fn snapshot_selects_all_visible_frozen_sources_and_consumes_only_their_gel() {
        let mut s = State::new(0, 2, false, 11);
        s.launch();
        assert_eq!(s.pool, [0, 1, 2]);
        assert_eq!(s.gel, [0, 0, 0, 0, 0, 3]);
        assert!(!s.pool.contains(&5));
        assert_eq!(s.contacts, [2, 3, 2, 1, 1, 0]);
    }
    #[test]
    fn waiting_finishes_the_burst_without_advancing_turns_or_repeating_contacts() {
        let mut s = State::new(0, 2, false, 11);
        s.launch();
        let contacts = s.contacts;
        s.clock = 600.0;
        assert!(!s.active());
        assert_eq!(s.cooldown, 8);
        assert_eq!(s.turn, 0);
        assert_eq!(s.contacts, contacts);
        assert_eq!(s.gel[5], 3);
    }
    #[test]
    fn unfrozen_neighbours_never_become_sources_or_get_frozen() {
        let mut s = State::new(0, 2, true, 11);
        s.launch();
        assert_eq!(s.pool, [1]);
        assert_eq!(s.contacts, [1, 1, 1, 1, 0, 0]);
        assert_eq!(&s.gel[..5], &[0; 5]);
    }
    #[test]
    fn a_cloison_protects_neighbours_and_preview_has_no_solid_cells() {
        let mut s = State::new(1, 2, false, 11);
        let map = Map::from_ascii(MAPS[1]).unwrap();
        assert!(s.cells().iter().all(|&p| map.is_walkable(p)));
        s.launch();
        assert_eq!(s.contacts[3], 0);
        assert_eq!(s.contacts[4], 0);
    }
    #[test]
    fn launch_refuses_cooldown_or_no_visible_source_without_mutation() {
        let mut s = State::new(0, 2, false, 11);
        s.launch();
        s.launch();
        assert_eq!(s.cooldown, 8);
        for _ in 0..8 {
            s.advance();
        }
        let contacts = s.contacts;
        s.launch();
        assert_eq!(s.cooldown, 0);
        assert_eq!(s.contacts, contacts);
    }
}
