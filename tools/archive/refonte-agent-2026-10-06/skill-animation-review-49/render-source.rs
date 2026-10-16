//! ANIM-S20: prepared swap/domination visuals, separate from campaign rules and AI.
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;
const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 6);
const TARGET: GridPos = GridPos::new(4, 3);
const OTHER: GridPos = GridPos::new(4, 4);
const PET: GridPos = GridPos::new(3, 5);
const IDS: [&str; 2] = ["HY04", "HY06"];
const TITLES: [&str; 2] = ["Échange de position", "Lien de domination"];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const COOLDOWNS: [u16; 2] = [6, 10];
const COSTS: [u16; 2] = [8, 19];
const TURN_FRAMES: [usize; 4] = [48, 60, 72, 84];
const MAPS: [&str; 3] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#..###..#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#########",
];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    Open,
    Wall,
    Quiet,
    Silence,
    Frozen,
    Hidden,
    Foreign,
    Illegal,
    Mule,
    Scout,
    Transformed,
    Boss,
    Far,
    Damage,
    Replace,
}
impl Case {
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Pièce ouverte",
            Self::Wall => "Mur",
            Self::Quiet => "Attente sans tour",
            Self::Silence => "Silence",
            Self::Frozen => "Agent gelé",
            Self::Hidden => "Cible cachée",
            Self::Foreign => "Drone non contrôlé",
            Self::Illegal => "Destination illégale",
            Self::Mule => "Mule et cargaison",
            Self::Scout => "Éclaireur",
            Self::Transformed => "Forme transformée",
            Self::Boss => "Boss immunisé",
            Self::Far => "Cible hors portée",
            Self::Damage => "Dégâts reçus",
            Self::Replace => "Ancien lien remplacé",
        }
    }
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    started: Option<f32>,
    turn: u16,
    cooldown: u16,
    player: GridPos,
    drone: GridPos,
    controlled: Option<GridPos>,
    remaining: u16,
    hit_clock: Option<f32>,
    damage_clock: Option<f32>,
    cargo: [&'static str; 2],
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.0,
            started: None,
            turn: 0,
            cooldown: 0,
            player: PLAYER,
            drone: TARGET,
            controlled: if case == Case::Replace {
                Some(OTHER)
            } else {
                None
            },
            remaining: if case == Case::Replace { 2 } else { 0 },
            hit_clock: None,
            damage_clock: None,
            cargo: ["Kit de soin", "Boîte de munitions"],
        }
    }
    fn map_index(&self) -> usize {
        if self.case == Case::Wall {
            if self.skill == 0 { 2 } else { 1 }
        } else {
            0
        }
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn target(&self) -> GridPos {
        if self.case == Case::Far {
            GridPos::new(4, 1)
        } else {
            TARGET
        }
    }
    fn model(&self) -> u8 {
        match self.case {
            Case::Mule => 57,
            Case::Scout => 56,
            _ => 58,
        }
    }
    fn visible(&self, at: GridPos) -> bool {
        match self.case {
            Case::Hidden => at != self.target(),
            Case::Wall if self.skill == 1 => at.y >= 5,
            Case::Wall => at.x < 5,
            _ => true,
        }
    }
    fn enemies(&self) -> Vec<GridPos> {
        if self.skill == 0 {
            vec![GridPos::new(2, 2), GridPos::new(6, 5)]
        } else {
            vec![self.target(), OTHER, GridPos::new(6, 3)]
        }
        .into_iter()
        .filter(|at| self.map().is_walkable(*at))
        .collect()
    }
    fn pet_targets(&self) -> Vec<GridPos> {
        self.enemies()
            .into_iter()
            .filter(|at| self.visible(*at) && Some(*at) != self.controlled)
            .collect()
    }
    fn can_launch(&self) -> bool {
        if self.started.is_some()
            || self.cooldown > 0
            || matches!(self.case, Case::Silence | Case::Hidden | Case::Transformed)
        {
            return false;
        }
        if self.skill == 0 {
            !matches!(self.case, Case::Frozen | Case::Foreign | Case::Illegal)
                && self.visible(self.drone)
                && self.map().is_walkable(self.player)
                && self.map().is_walkable(self.drone)
        } else {
            self.case != Case::Boss
                && self.visible(self.target())
                && (PLAYER.y - self.target().y).abs() <= 4
                && (self.target().y..PLAYER.y).all(|y| self.map().is_walkable(GridPos::new(4, y)))
        }
    }
    fn footprint(&self) -> Vec<GridPos> {
        if !self.can_launch() {
            return Vec::new();
        }
        if self.skill == 0 {
            vec![self.player, self.drone]
        } else {
            vec![self.target()]
        }
    }
    fn launch(&mut self) {
        if !self.can_launch() {
            return;
        }
        self.started = Some(self.clock);
        self.cooldown = COOLDOWNS[self.skill];
        if self.skill == 0 {
            std::mem::swap(&mut self.player, &mut self.drone);
        } else {
            self.controlled = Some(self.target());
            self.remaining = 4;
            self.hit_clock = Some(self.clock);
        }
    }
    fn advance(&mut self) {
        if self.started.is_none() {
            return;
        }
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        if self.skill == 1 {
            self.remaining = self.remaining.saturating_sub(1);
            if self.remaining == 0 {
                self.controlled = None;
            }
            if self.case == Case::Damage && self.turn == 1 {
                self.damage_clock = Some(self.clock);
            }
        }
    }
    fn age(&self) -> Option<f32> {
        self.started.map(|clock| self.clock - clock)
    }
    fn victim_point(&self, at: GridPos, rect: Rect, effects: bool) -> Vec2 {
        let p = point(at, self.direction, rect);
        if !effects || at != self.target() {
            return p;
        }
        let clock = self.damage_clock.or(self.hit_clock);
        clock.map_or(p, |clock| {
            p + crate::combat_visuals::hit_reaction_offset(
                p,
                point(PLAYER, self.direction, rect),
                f64::from(self.clock - clock),
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
    fragments: Texture2D,
    sigil: Texture2D,
    gel: Texture2D,
    form: Texture2D,
}
impl Art {
    fn load() -> Self {
        let load = |bytes: &[u8]| {
            let t = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            fragments: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-49/fragments-atlas.png"
            )),
            sigil: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-49/sigil-32.png"
            )),
            gel: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-28/gel-atlas.png"
            )),
            form: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-39/predator-64.png"
            )),
        }
    }
    fn player(&self, s: &State, at: Vec2, opacity: f32) {
        if s.case == Case::Transformed {
            draw_texture_ex(
                &self.form,
                at.x - 32.0,
                at.y - 32.0,
                alpha(WHITE, opacity),
                DrawTextureParams {
                    dest_size: Some(vec2(64.0, 64.0)),
                    ..Default::default()
                },
            );
            return;
        }
        game_art::draw_world(
            Rect::new(at.x - 32.0, at.y - 32.0, 64.0, 64.0),
            game_art::wear(
                game_art::player(Some("core:technomancien"), PlayerWeaponPose::Unarmed, None),
                [("core:body_armor".into(), "core:tenue_technique".into())],
            ),
            opacity,
        );
        if s.case == Case::Frozen {
            let phase = s.clock * 3.0;
            let frame = phase as usize % 6;
            let blend = phase.fract();
            for (frame, weight) in [(frame, 1.0 - blend), ((frame + 1) % 6, blend)] {
                draw_texture_ex(
                    &self.gel,
                    at.x - 36.0,
                    at.y - 18.0,
                    alpha(WHITE, 0.70 * weight),
                    DrawTextureParams {
                        source: Some(Rect::new(frame as f32 * 256.0, 64.0, 256.0, 192.0)),
                        dest_size: Some(vec2(72.0, 54.0)),
                        ..Default::default()
                    },
                );
            }
        }
    }
    fn drone(at: Vec2, model: u8, controlled: bool, opacity: f32) {
        game_art::draw_world(
            Rect::new(at.x - 32.0, at.y - 32.0, 64.0, 64.0),
            Visual::Bestiary {
                number: model,
                corpse: false,
            },
            opacity,
        );
        if controlled {
            draw_circle(
                at.x + 18.0,
                at.y - 23.0,
                3.0,
                alpha(Color::new(0.20, 0.95, 0.25, 1.0), opacity),
            );
        }
    }
    fn pair(&self, s: &State, rect: Rect, effects: bool) {
        let age = s.age().unwrap_or(99.0);
        let (player, drone, opacity) =
            if effects && s.skill == 0 && s.started.is_some() && age < 0.32 {
                if age < 0.14 {
                    (PLAYER, TARGET, 1.0 - age / 0.14)
                } else {
                    (s.player, s.drone, ((age - 0.14) / 0.18).clamp(0.0, 1.0))
                }
            } else {
                (s.player, s.drone, 1.0)
            };
        self.player(s, point(player, s.direction, rect), opacity);
        if s.visible(drone) {
            Self::drone(
                point(drone, s.direction, rect),
                s.model(),
                s.case != Case::Foreign,
                opacity,
            );
        }
        if effects && s.skill == 0 && s.started.is_some() && age < 0.32 {
            let frame = ((age / 0.32 * 6.0) as usize).min(5);
            for at in [PLAYER, TARGET] {
                let p = point(at, s.direction, rect);
                draw_texture_ex(
                    &self.fragments,
                    p.x - 56.0,
                    p.y - 76.0,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(frame as f32 * 256.0, 0.0, 256.0, 256.0)),
                        dest_size: Some(vec2(112.0, 112.0)),
                        ..Default::default()
                    },
                );
            }
        }
    }
    fn marker(&self, at: Vec2, clock: f32) {
        let p = at - vec2(0.0, 53.0 + (clock * 3.0).sin());
        draw_texture_ex(
            &self.sigil,
            p.x - 16.0,
            p.y - 16.0,
            alpha(WHITE, 0.90 + 0.10 * (clock * 3.0).sin()),
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                ..Default::default()
            },
        );
    }
}
fn scene(art: &Art, clip: &Clip, s: &State, rect: Rect, effects: bool) {
    let map = s.map();
    terrain(&map, s.direction, rect, false);
    if s.started.is_none() {
        preview(&s.footprint(), s.direction, rect, s.clock);
    }
    terrain(&map, s.direction, rect, true);
    clip.apply_for(s.map_index(), s.direction, rect);
    for at in s.enemies().into_iter().filter(|at| s.visible(*at)) {
        enemy(s.victim_point(at, rect, effects));
    }
    if s.skill == 0 {
        art.pair(s, rect, effects);
    } else {
        art.player(s, point(PLAYER, s.direction, rect), 1.0);
        Art::drone(point(PET, s.direction, rect), 58, true, 1.0);
        if let Some(at) = s.controlled.filter(|at| s.visible(*at)) {
            // Pre-existing link belongs to the scene; controls only suppress new VFX.
            if effects || s.started.is_none() {
                art.marker(s.victim_point(at, rect, effects), s.clock);
            }
        }
    }
    gl_use_default_material();
    if let Some(age) = s.age().filter(|age| *age < 0.65) {
        let p = point(s.player, s.direction, rect);
        draw_text_bold(
            TITLES[s.skill],
            p.x - 76.0,
            p.y - 43.0 - age * 6.0,
            15.0,
            alpha(TEXT, 1.0 - age / 0.65),
        );
    }
    draw_rectangle(
        rect.x + 5.0,
        rect.y + SIZE - 25.0,
        556.0,
        20.0,
        alpha(INK, 0.92),
    );
    let text = if s.skill == 0 {
        format!(
            "Recharge {} tours · cargaison {} objets",
            s.cooldown,
            s.cargo.len()
        )
    } else {
        format!(
            "Domination {} tours · recharge {} · cibles du drone {}",
            s.remaining,
            s.cooldown,
            s.pet_targets().len()
        )
    };
    draw_text(text, rect.x + 12.0, rect.y + SIZE - 10.0, 14.0, TEXT);
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
        Case::Quiet,
        Case::Silence,
        Case::Frozen,
        Case::Hidden,
        Case::Foreign,
        Case::Illegal,
        Case::Mule,
        Case::Scout,
        Case::Transformed,
        Case::Boss,
        Case::Far,
        Case::Damage,
        Case::Replace,
    ];
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        clear_background(INK);
        draw_text_bold("Échange et domination · ANIM-S20", 20.0, 33.0, 23.0, TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 95.0 + i as f32 * 42.0, 320.0, 34.0),
                title,
                true,
            ) {
                s = State::new(i, Case::Open, s.direction);
            }
        }
        for (i, title) in DIRS.iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 164.0,
                    193.0 + (i / 2) as f32 * 37.0,
                    156.0,
                    31.0,
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
                    285.0 + (i / 2) as f32 * 34.0,
                    176.0,
                    28.0,
                ),
                case.title(),
                true,
            ) {
                s = State::new(s.skill, *case, s.direction);
            }
        }
        if button(
            Rect::new(20.0, 588.0, 320.0, 34.0),
            "Lancer",
            s.can_launch(),
        ) || (is_key_pressed(KeyCode::Enter) && s.can_launch())
        {
            s.launch();
        }
        if button(
            Rect::new(20.0, 634.0, 151.0, 34.0),
            "+1 tour",
            s.started.is_some(),
        ) || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(181.0, 634.0, 159.0, 34.0), "Nouvel aperçu", true) {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(20.0, 680.0, 320.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
        }
        scene(&art, &clip, &s, Rect::new(410.0, 88.0, SIZE, SIZE), true);
        draw_text(
            format!(
                "Énergie {} · recharge {} tours · portée {} cases",
                COSTS[s.skill],
                COOLDOWNS[s.skill],
                if s.skill == 0 { 6 } else { 4 }
            ),
            410.0,
            705.0,
            16.0,
            TEXT,
        );
        draw_text(
            "Ciblage allié préparé : aucune IA de campagne modifiée.",
            410.0,
            735.0,
            13.0,
            DIM,
        );
        draw_text(
            "Monstres et drones actuels 32 px agrandis ; adaptation prévue ensuite.",
            410.0,
            763.0,
            13.0,
            DIM,
        );
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
    let target = render_target(640, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 640.0, 720.0))
    };
    let controls = [14, 17, 20, 22, 25, 35, 47, 48, 60, 61, 63, 72, 84, 95];
    let mut entries = Vec::new();
    for skill in 0..2 {
        let mut cases = vec![
            (Case::Open, 2, "HAUT"),
            (Case::Open, 0, "DROITE"),
            (Case::Open, 1, "GAUCHE"),
            (Case::Open, 3, "BAS"),
            (Case::Wall, 2, "CLOISON"),
            (Case::Quiet, 2, "ATTENTE"),
            (Case::Silence, 2, "SILENCE"),
            (Case::Hidden, 2, "CACHE"),
        ];
        if skill == 0 {
            cases.extend([
                (Case::Frozen, 2, "GELE"),
                (Case::Foreign, 2, "NON-CONTROLE"),
                (Case::Illegal, 2, "DESTINATION"),
                (Case::Mule, 2, "MULE"),
                (Case::Scout, 2, "ECLAIREUR"),
                (Case::Transformed, 2, "TRANSFORME"),
            ]);
        } else {
            cases.extend([
                (Case::Transformed, 2, "TRANSFORME"),
                (Case::Boss, 2, "BOSS"),
                (Case::Far, 2, "HORS-PORTEE"),
                (Case::Damage, 2, "DEGATS"),
                (Case::Replace, 2, "REMPLACEMENT"),
            ]);
        }
        for (case, direction, label) in cases {
            let folder = format!("{}-{label}", IDS[skill]);
            let dir = output.join(&folder);
            std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
            let mut s = State::new(skill, case, direction);
            let permitted = s.can_launch();
            let mut timeline = Vec::new();
            for frame in 0..96 {
                s.clock = frame as f32 / 24.0;
                if frame == 17 {
                    s.launch();
                }
                if case != Case::Quiet && TURN_FRAMES.contains(&frame) {
                    s.advance();
                }
                timeline.push(serde_json::json!({"frame":frame,"turn":s.turn,"cooldown":s.cooldown,"remaining":s.remaining,"launched":s.started.is_some(),
                    "player":[s.player.x,s.player.y],"drone":[s.drone.x,s.drone.y],"controlled":s.controlled.map(|a|[a.x,a.y]),
                    "pet_targets":s.pet_targets().iter().map(|a|[a.x,a.y]).collect::<Vec<_>>(),"cargo":s.cargo,"damage_feedback":s.damage_clock.is_some()}));
                for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                    set_camera(&camera);
                    clear_background(INK);
                    draw_text_bold(
                        format!("{} · {}", 49 + skill, TITLES[skill]),
                        32.0,
                        35.0,
                        23.0,
                        TEXT,
                    );
                    draw_text(
                        format!("{}-A01 · {} · tour {}", IDS[skill], label, s.turn),
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
            entries.push(serde_json::json!({"folder":folder,"variant":format!("{}-A01-{label}",IDS[skill]),"id":IDS[skill],"number":49+skill,"title":TITLES[skill],
                "case":label,"case_title":case.title(),"direction":direction,"permitted":permitted,"quiet":case==Case::Quiet,"cooldown":COOLDOWNS[skill],
                "control_frames":controls,"turn_frames":if case==Case::Quiet{vec![]}else{TURN_FRAMES.to_vec()},"tile":64,"frames":96,"fps":24,
                "map":rows.into_iter().map(|row|row.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n")}));
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
    fn swap_preserves_cargo_for_all_shown_models_and_changes_only_positions() {
        for case in [Case::Open, Case::Mule, Case::Scout, Case::Wall] {
            let mut s = State::new(0, case, 2);
            let cargo = s.cargo;
            s.launch();
            assert_eq!(s.player, TARGET);
            assert_eq!(s.drone, PLAYER);
            assert_eq!(s.cargo, cargo);
            assert_eq!(s.cooldown, 6);
        }
    }
    #[test]
    fn illegal_conditions_refuse_without_state_change() {
        for case in [
            Case::Silence,
            Case::Frozen,
            Case::Hidden,
            Case::Foreign,
            Case::Illegal,
            Case::Transformed,
        ] {
            let mut s = State::new(0, case, 2);
            s.launch();
            assert!(s.started.is_none());
            assert_eq!(s.player, PLAYER);
            assert_eq!(s.drone, TARGET);
            assert_eq!(s.cooldown, 0);
        }
        for case in [
            Case::Silence,
            Case::Hidden,
            Case::Transformed,
            Case::Wall,
            Case::Far,
            Case::Boss,
        ] {
            let mut s = State::new(1, case, 2);
            s.launch();
            assert!(s.started.is_none());
            assert_eq!(s.remaining, 0);
        }
    }
    #[test]
    fn domination_waits_for_turns_and_restores_hostility_without_healing() {
        let mut s = State::new(1, Case::Damage, 2);
        s.launch();
        s.clock = 100.0;
        assert_eq!(s.remaining, 4);
        assert!(!s.pet_targets().contains(&TARGET));
        s.advance();
        assert_eq!(s.remaining, 3);
        assert!(s.damage_clock.is_some());
        assert_eq!(s.controlled, Some(TARGET));
        for _ in 0..3 {
            s.advance();
        }
        assert_eq!(s.controlled, None);
        assert_eq!(s.remaining, 0);
        assert!(s.pet_targets().contains(&TARGET));
        assert_eq!(s.cooldown, 6);
    }
    #[test]
    fn replacement_retargets_pets_and_removes_old_symbol() {
        let mut s = State::new(1, Case::Replace, 2);
        assert_eq!(s.controlled, Some(OTHER));
        assert!(!s.pet_targets().contains(&OTHER));
        s.launch();
        assert_eq!(s.controlled, Some(TARGET));
        assert_eq!(s.remaining, 4);
        assert!(s.pet_targets().contains(&OTHER));
        assert!(!s.pet_targets().contains(&TARGET));
    }
}
