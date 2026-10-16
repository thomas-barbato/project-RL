//! ANIM-S06: fireball and focused combustion; isolated visual review.
use super::obstacles::overlay::stopped::{burn_on_body, export_target};
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;
use std::collections::{BTreeSet, VecDeque};

const SIZE: f32 = 576.0;
const FLIGHT_SECONDS: f32 = 0.65;
const IMPACT: GridPos = GridPos::new(4, 4);
const PLAYER: GridPos = GridPos::new(4, 7);
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#########\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
];
const IDS: [&str; 2] = ["FE05", "FE06"];
const TITLES: [&str; 2] = ["Boule de feu", "Combustion"];
const DIRS: [&str; 4] = ["Droite", "Gauche", "Haut", "Bas"];
const TARGETS: [GridPos; 7] = [
    IMPACT,
    GridPos::new(3, 4),
    GridPos::new(5, 4),
    GridPos::new(4, 5),
    GridPos::new(2, 6),
    GridPos::new(4, 2),
    GridPos::new(7, 1),
];

struct Art {
    ball: Texture2D,
    blast: Texture2D,
}
impl Art {
    fn load() -> Self {
        let decode = |bytes: &[u8]| {
            let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            texture.set_filter(FilterMode::Linear);
            texture
        };
        Self {
            ball: decode(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-16/projectile-atlas.png"
            )),
            blast: decode(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-16/blast-atlas.png"
            )),
        }
    }
    fn draw(&self, ball: bool, at: Vec2, age: f32, size: f32, rotation: f32, opacity: f32) {
        let frames = [0, 1, 2, 3, 2, 1, 0, 1];
        let frame = frames[((age.max(0.0) * 8.0) as usize) % 8];
        let local = if ball {
            vec2(190.0, 132.0)
        } else {
            vec2(128.0, 128.0)
        } * (size / 256.0);
        draw_texture_ex(
            if ball { &self.ball } else { &self.blast },
            at.x - local.x,
            at.y - local.y,
            alpha(WHITE, opacity),
            DrawTextureParams {
                source: Some(Rect::new(frame as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(vec2(size, size)),
                rotation,
                pivot: Some(at),
                ..Default::default()
            },
        );
    }
}
fn last_free(map: &Map, start: GridPos, target: GridPos) -> GridPos {
    let mut at = start;
    let step = (target.y - start.y).signum();
    while at.y != target.y {
        let next = GridPos::new(at.x, at.y + step);
        if !map.is_walkable(next) {
            break;
        }
        at = next;
    }
    at
}
fn footprint(map: &Map, skill: usize, impact: GridPos) -> Vec<GridPos> {
    if skill == 1 {
        return vec![impact];
    }
    let mut seen = BTreeSet::from([(impact.x, impact.y)]);
    let mut queue = VecDeque::from([impact]);
    while let Some(at) = queue.pop_front() {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next = GridPos::new(at.x + dx, at.y + dy);
            if next.x >= 0
                && next.x < 9
                && next.y >= 0
                && next.y < 9
                && map.is_walkable(next)
                && seen.insert((next.x, next.y))
            {
                queue.push_back(next);
            }
        }
    }
    seen.into_iter()
        .filter(|&(x, y)| (x - impact.x).pow(2) + (y - impact.y).pow(2) <= 9)
        .map(|(x, y)| GridPos::new(x, y))
        .collect()
}
struct State {
    skill: usize,
    obstacle: usize,
    direction: usize,
    fresh_burn: bool,
    intercept: bool,
    clock: f32,
    launched: Option<f32>,
    remaining: u16,
    turn: u16,
    cooldown: u16,
    burn: [u16; 7],
}
impl State {
    fn new(skill: usize, obstacle: usize, direction: usize, fresh_burn: bool) -> Self {
        let mut burn = [0; 7];
        if skill == 1 {
            burn[0] = 3;
        }
        Self {
            skill,
            obstacle,
            direction,
            fresh_burn,
            intercept: false,
            clock: 0.0,
            launched: None,
            remaining: 0,
            turn: 0,
            cooldown: 0,
            burn,
        }
    }
    fn targets(&self) -> [GridPos; 7] {
        let mut targets = TARGETS;
        if !self.intercept {
            targets[3] = GridPos::new(5, 5);
        }
        if self.obstacle == 1 {
            targets[0] = GridPos::new(5, 4);
            targets[2] = GridPos::new(6, 4);
        }
        targets
    }
    fn collision(&self) -> (GridPos, Option<usize>) {
        let targets = self.targets();
        if self.skill == 1 {
            return (targets[0], Some(0));
        }
        let map = Map::from_ascii(MAPS[self.obstacle]).unwrap();
        let aimed = if self.obstacle == 1 {
            GridPos::new(4, 1)
        } else {
            IMPACT
        };
        let end = last_free(&map, PLAYER, aimed);
        for y in (end.y..PLAYER.y).rev() {
            let at = GridPos::new(PLAYER.x, y);
            if let Some(index) = targets.iter().position(|&target| target == at) {
                return (at, Some(index));
            }
        }
        (end, None)
    }
    fn burst_point(&self, rect: Rect) -> Vec2 {
        let (at, target) = self.collision();
        let p = self.point(at, rect);
        if self.skill == 0 && self.obstacle == 1 && target.is_none() {
            p + (self.point(GridPos::new(at.x, at.y - 1), rect) - p) * 0.5
        } else {
            p
        }
    }
    fn can_launch(&self) -> bool {
        self.cooldown == 0 && (self.skill == 0 || self.burn[0] > 0)
    }
    fn launch(&mut self) {
        if !self.can_launch() {
            return;
        }
        self.launched = Some(self.clock);
        self.remaining = 1;
        self.cooldown = parameter_for(IDS[self.skill], "cooldown_turns", 11);
        let zone = footprint(
            &Map::from_ascii(MAPS[self.obstacle]).unwrap(),
            self.skill,
            self.collision().0,
        );
        if self.skill == 1 {
            self.burn[0] = 0;
        }
        if self.fresh_burn {
            let key = if self.skill == 0 {
                "burn_duration_turns"
            } else {
                "fresh_burn_duration_turns"
            };
            for (i, at) in self.targets().iter().enumerate() {
                if zone.contains(at) {
                    self.burn[i] = parameter_for(IDS[self.skill], key, 11);
                }
            }
        }
    }
    fn advance_turn(&mut self) {
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        self.remaining = self.remaining.saturating_sub(1);
        for burn in &mut self.burn {
            *burn = burn.saturating_sub(1);
        }
    }
    fn point(&self, source: GridPos, rect: Rect) -> Vec2 {
        let p = position(source, self.direction);
        cell(rect, p.x as f32 + 0.5, p.y as f32 + 0.5)
    }
}
fn scene(fire: &DrawnFire, art: &Art, clip: &Clip, state: &State, rect: Rect, effects: bool) {
    let map = Map::from_ascii(MAPS[state.obstacle]).unwrap();
    terrain(&map, state.direction, rect, false);
    let zone = footprint(&map, state.skill, state.collision().0);
    let age = state
        .launched
        .map(|start| state.clock - start)
        .unwrap_or(0.0);
    if state.launched.is_none() {
        preview(&zone, state.direction, rect, state.clock);
    }
    if effects && state.remaining > 0 {
        clip.apply_for(state.obstacle, state.direction, rect);
        let at = state.burst_point(rect);
        if state.skill == 0 {
            if age < FLIGHT_SECONDS {
                let origin = state.point(PLAYER, rect) + vec2(0.0, -8.0);
                let destination = at + vec2(0.0, -8.0);
                let heading = destination - origin;
                art.draw(
                    true,
                    origin + heading * (age / FLIGHT_SECONDS).clamp(0.0, 1.0),
                    age,
                    96.0,
                    heading.y.atan2(heading.x),
                    0.90,
                );
            } else {
                let rise = (0.5 + (age - FLIGHT_SECONDS) / 0.12).min(1.0);
                art.draw(false, at, age - FLIGHT_SECONDS, 352.0 * rise, 0.0, 0.52);
            }
        } else {
            let closing = (age / 0.20).clamp(0.0, 1.0);
            for (phase, sign) in [(0, -1.0), (3, 1.0)] {
                fire.plume(
                    at + vec2(sign * (13.0 - 8.0 * closing), 21.0),
                    age * (2.0 / 3.0),
                    phase,
                    if phase == 0 { 0.52 } else { 0.42 },
                    0.65,
                );
            }
        }
        gl_use_default_material();
    }
    let show_burn = state.skill == 1
        || state.launched.is_none()
        || age >= FLIGHT_SECONDS
        || state.remaining == 0;
    if effects && show_burn {
        for (i, &at) in state.targets().iter().enumerate() {
            if state.burn[i] > 0 {
                fire.burn(
                    state.point(at, rect),
                    state.clock * (2.0 / 3.0),
                    i as u32 * 2,
                    0.9,
                );
            }
        }
    }
    terrain(&map, state.direction, rect, true);
    for (i, &at) in state.targets().iter().enumerate() {
        let p = state.point(at, rect);
        enemy(p);
        if effects && show_burn && state.burn[i] > 0 {
            burn_on_body(fire, p, state.clock * (2.0 / 3.0), i as u32 * 2);
        }
        draw_text_bold(
            char::from(b'A' + i as u8).to_string(),
            p.x - 4.0,
            p.y - 29.0,
            13.0,
            TEXT,
        );
    }
    let player = state.point(PLAYER, rect);
    actor(player, caster());
    if state.launched.is_some() && age < 1.1 {
        draw_text_bold(
            TITLES[state.skill],
            player.x - 45.0,
            player.y - 43.0 - age * 7.0,
            15.0,
            alpha(TEXT, (1.0 - age / 1.1).sqrt()),
        );
    }
    let label = if state.launched.is_none() {
        "Aperçu clignotant · avant lancement"
    } else if state.remaining > 0 {
        "Effet animé · jusqu'au prochain tour"
    } else if state.burn.iter().any(|&n| n > 0) {
        "Impact terminé · Brûlure sur les corps"
    } else {
        "Effets terminés"
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
    let fire = DrawnFire::load();
    let art = Art::load();
    let clip = Clip::load_maps(&MAPS, IMPACT);
    let mut state = State::new(0, 0, 0, true);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold("LABORATOIRE / FEU · IMPACTS", 22.0, 35.0, 24.0, TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 88.0 + i as f32 * 44.0, 285.0, 35.0),
                title,
                true,
            ) {
                state = State::new(i, state.obstacle, state.direction, state.fresh_burn);
            }
        }
        for (i, title) in ["Zone ouverte", "Au contact d'une cloison"]
            .iter()
            .enumerate()
        {
            if button(
                Rect::new(20.0, 198.0 + i as f32 * 44.0, 285.0, 35.0),
                title,
                true,
            ) {
                state = State::new(state.skill, i, state.direction, state.fresh_burn);
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
                state = State::new(state.skill, state.obstacle, i, state.fresh_burn);
            }
        }
        if button(
            Rect::new(20.0, 420.0, 285.0, 34.0),
            if state.fresh_burn {
                "Nouvelle Brûlure : oui"
            } else {
                "Nouvelle Brûlure : non"
            },
            state.launched.is_none(),
        ) {
            state.fresh_burn = !state.fresh_burn;
        }
        let label = if state.cooldown > 0 {
            format!("Recharge : {} tours", state.cooldown)
        } else if !state.can_launch() {
            "Cible sans Brûlure".into()
        } else {
            "Lancer".into()
        };
        if button(
            Rect::new(20.0, 462.0, 285.0, 30.0),
            if state.intercept {
                "Ennemi devant la cible"
            } else {
                "Trajet direct vers la cible"
            },
            state.skill == 0 && state.launched.is_none(),
        ) {
            state.intercept = !state.intercept;
        }
        if button(
            Rect::new(20.0, 501.0, 285.0, 34.0),
            &label,
            state.can_launch(),
        ) || (is_key_pressed(KeyCode::Enter) && state.can_launch())
        {
            state.launch();
            paused = false;
        }
        if button(Rect::new(20.0, 545.0, 138.0, 34.0), "+1 tour", true)
            || is_key_pressed(KeyCode::Space)
        {
            state.advance_turn();
        }
        if button(Rect::new(166.0, 545.0, 138.0, 34.0), "Nouvel aperçu", true) {
            state = State::new(
                state.skill,
                state.obstacle,
                state.direction,
                state.fresh_burn,
            );
        }
        if button(
            Rect::new(20.0, 589.0, 285.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        draw_text(
            format!("Tour {} · recharge : {} tours", state.turn, state.cooldown),
            22.0,
            670.0,
            16.0,
            TEXT,
        );
        scene(
            &fire,
            &art,
            &clip,
            &state,
            Rect::new(410.0, 88.0, SIZE, SIZE),
            true,
        );
        draw_text(
            format!(
                "{} / {}-{} · {}",
                state.skill + 13,
                IDS[state.skill],
                if state.skill == 0 { "A03" } else { "A02" },
                DIRS[state.direction]
            ),
            410.0,
            695.0,
            15.0,
            DIM,
        );
        draw_text(
            "Les tours avancent uniquement avec +1 tour · cas de Brûlure préparés pour l'aperçu",
            22.0,
            857.0,
            15.0,
            DIM,
        );
        if !paused {
            state.clock += get_frame_time();
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
    let fire = DrawnFire::load();
    let art = Art::load();
    let clip = Clip::load_maps(&MAPS, IMPACT);
    let target = render_target(1280, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 1280.0, 720.0))
    };
    let cases = [
        (0, 0, true, false, "DROITE"),
        (0, 1, true, false, "GAUCHE"),
        (0, 2, true, false, "HAUT"),
        (1, 3, false, false, "CLOISON-BAS"),
        (0, 0, true, true, "INTERCEPTION"),
    ];
    let controls = [3, 14, 24, 30, 32, 33, 40, 52, 61, 85, 95];
    let mut entries = Vec::new();
    for (obstacle, direction, fresh_burn, intercept, folder) in cases {
        let path = output.join(folder);
        std::fs::create_dir(&path).map_err(|e| e.to_string())?;
        let mut states = [
            State::new(0, obstacle, direction, true),
            State::new(1, obstacle, direction, fresh_burn),
        ];
        states[0].intercept = intercept;
        let mut launched = false;
        let mut event = 0;
        let mut timeline = Vec::new();
        for frame in 0..96 {
            let clock = frame as f32 / 24.0;
            for state in &mut states {
                state.clock = clock;
            }
            if !launched && clock >= 0.7 {
                for state in &mut states {
                    state.launch();
                }
                launched = true;
            }
            while event < 3 && clock >= [2.5, 3.0, 3.5][event] {
                for state in &mut states {
                    state.advance_turn();
                }
                event += 1;
            }
            let age = states[0].launched.map(|start| clock - start);
            timeline.push(serde_json::json!({"frame":frame,"turn":states[0].turn,"remaining":states[0].remaining,"cooldowns":[states[0].cooldown,states[1].cooldown],"burn":[states[0].burn,states[1].burn],"projectile_visible":states[0].remaining>0 && age.is_some_and(|age|age<FLIGHT_SECONDS),"explosion_visible":states[0].remaining>0 && age.is_some_and(|age|age>=FLIGHT_SECONDS)}));
            for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                set_camera(&camera);
                clear_background(INK);
                for (skill, state) in states.iter().enumerate() {
                    let x = 24.0 + skill as f32 * 616.0;
                    draw_text_bold(
                        format!("{} · {}", skill + 13, TITLES[skill]),
                        x,
                        35.0,
                        21.0,
                        TEXT,
                    );
                    draw_text(
                        format!(
                            "{}-{} · {} · tour {}",
                            IDS[skill],
                            if skill == 0 { "A03" } else { "A02" },
                            DIRS[direction],
                            state.turn
                        ),
                        x,
                        60.0,
                        14.0,
                        DIM,
                    );
                    scene(
                        &fire,
                        &art,
                        &clip,
                        state,
                        Rect::new(x, 80.0, SIZE, SIZE),
                        control == 0,
                    );
                }
                export_target(
                    &target,
                    &path.join(if control == 0 {
                        format!("{frame:03}.png")
                    } else {
                        format!("control-{frame:03}.png")
                    }),
                );
            }
            set_default_camera();
            next_frame().await;
        }
        std::fs::write(
            path.join("timeline.json"),
            serde_json::to_vec_pretty(&timeline).unwrap(),
        )
        .map_err(|e| e.to_string())?;
        for skill in 0..2 {
            let map = Map::from_ascii(MAPS[obstacle]).unwrap();
            let (impact, hit) = states[skill].collision();
            let zone = footprint(&map, skill, impact);
            let coords = |at| {
                let p = position(at, direction);
                [p.x, p.y]
            };
            let mut rows = vec![vec!['.'; 9]; 9];
            for y in 0..9 {
                for x in 0..9 {
                    if map.blocks_vision(GridPos::new(x, y)) {
                        let p = position(GridPos::new(x, y), direction);
                        rows[p.y as usize][p.x as usize] = '#';
                    }
                }
            }
            entries.push(serde_json::json!({"number":skill+13,"id":IDS[skill],"title":TITLES[skill],"variant":format!("{}-{}-{}",IDS[skill],if skill==0 {"A03"} else {"A02"},folder),"folder":folder,"direction":DIRS[direction],"crop":[24+skill*616,80,576,576],"frames":96,"fps":24,"tile":64,"control_frames":controls,"launch_frame":17,"contact_frame":33,"end_frame":60,"flight_seconds":FLIGHT_SECONDS,"impact_cell":coords(impact),"collision_target":hit.map(|index|char::from(b'A'+index as u8).to_string()),"interception":intercept && skill==0,"fresh_burn":if skill==0 {true} else {fresh_burn},"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n"),"affected_cells":zone.iter().copied().map(coords).collect::<Vec<_>>(),"targets":states[skill].targets().iter().enumerate().map(|(i,&at)|serde_json::json!({"label":char::from(b'A'+i as u8).to_string(),"cell":coords(at),"affected":zone.contains(&at)})).collect::<Vec<_>>(),"geometry":"Visual collision fixture only; final campaign damage and collision rules are not implemented."}));
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
    fn the_projectile_explodes_on_the_first_monster_it_encounters() {
        let direct = State::new(0, 0, 0, true);
        assert_eq!(direct.collision(), (IMPACT, Some(0)));
        let mut intercepted = State::new(0, 0, 0, true);
        intercepted.intercept = true;
        assert_eq!(intercepted.collision(), (GridPos::new(4, 5), Some(3)));
        assert_eq!(
            intercepted.burst_point(Rect::new(0.0, 0.0, SIZE, SIZE)),
            intercepted.point(GridPos::new(4, 5), Rect::new(0.0, 0.0, SIZE, SIZE))
        );
    }
    #[test]
    fn waiting_keeps_impact_and_cooldowns_until_a_turn_passes() {
        let mut state = State::new(0, 0, 0, true);
        state.launch();
        state.clock = 600.0;
        assert_eq!(state.remaining, 1);
        assert_eq!(state.cooldown, 8);
        assert_eq!(state.burn[0], 3);
        state.advance_turn();
        assert_eq!(state.remaining, 0);
        assert_eq!(state.cooldown, 7);
        assert_eq!(state.burn[0], 2);
    }
    #[test]
    fn full_cloison_excludes_the_far_room_and_stops_the_projectile() {
        let map = Map::from_ascii(MAPS[1]).unwrap();
        assert_eq!(last_free(&map, PLAYER, GridPos::new(4, 1)), IMPACT);
        assert!(!footprint(&map, 0, IMPACT).contains(&GridPos::new(4, 2)));
        assert_eq!(State::new(0, 1, 0, true).collision(), (IMPACT, None));
    }
    #[test]
    fn combustion_consumes_burn_and_recast_requires_a_compatible_target() {
        let mut state = State::new(1, 0, 0, false);
        state.launch();
        assert_eq!(state.burn[0], 0);
        state.clock = 600.0;
        assert_eq!(state.remaining, 1);
        for _ in 0..3 {
            state.advance_turn();
        }
        assert!(!state.can_launch());
        let start = state.launched;
        state.launch();
        assert_eq!(state.launched, start);
        let mut other = State::new(1, 0, 0, true);
        other.launch();
        assert_eq!(other.burn[0], 3);
        assert!(other.burn[1..].iter().all(|&n| n == 0));
    }
    #[test]
    fn four_directions_preserve_the_circle_and_body_coordinates() {
        let zone = footprint(&Map::from_ascii(MAPS[0]).unwrap(), 0, IMPACT);
        assert_eq!(zone.len(), 29);
        for dir in 0..4 {
            let mapped = zone
                .iter()
                .map(|&p| {
                    let q = position(p, dir);
                    (q.x, q.y)
                })
                .collect::<BTreeSet<_>>();
            assert_eq!(mapped.len(), 29);
            assert!(mapped.contains(&(4, 4)));
        }
    }
}
