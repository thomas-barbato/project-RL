//! ANIM-S21: prepared melee buff presentation; no campaign combat or stat formulas.
use super::super::melee::{cut, player_visual};
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;

const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 4);
const TARGET: GridPos = GridPos::new(4, 3);
const IDS: [&str; 2] = ["BM01", "BM02"];
const TITLES: [&str; 2] = ["Lame incendiaire", "Faim vampirique"];
const DURATIONS: [u16; 2] = [6, 4];
const COOLDOWNS: [u16; 2] = [1, 1];
const COSTS: [u16; 2] = [11, 13];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const TURN_FRAMES: [usize; 6] = [52, 76, 88, 100, 112, 124];
const FRAMES: usize = 128;
const FIRE: Color = Color::new(1.0, 0.43, 0.08, 1.0);
const HEAL: Color = Color::new(0.43, 0.98, 0.35, 1.0);
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#..###..#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
];
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    Open,
    Quiet,
    Wall,
    Hidden,
    Silence,
    Ranged,
    Transformed,
    Hover,
}
impl Case {
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Deux frappes",
            Self::Quiet => "Attente sans tour",
            Self::Wall => "Mur devant la frappe",
            Self::Hidden => "Cible cachée",
            Self::Silence => "Silence",
            Self::Ranged => "Arme à distance active",
            Self::Transformed => "Forme transformée",
            Self::Hover => "Survol du buff",
        }
    }
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    started: Option<f32>,
    strike: Option<f32>,
    turn: u16,
    remaining: u16,
    cooldown: u16,
    hits: u16,
    // Fixed visual fixtures: no RNG, weapon variance, defenses or healing solver.
    damage: u16,
    fire: u16,
    healing: u16,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.0,
            started: None,
            strike: None,
            turn: 0,
            remaining: 0,
            cooldown: 0,
            hits: 0,
            damage: 0,
            fire: 0,
            healing: 0,
        }
    }
    fn map_index(&self) -> usize {
        usize::from(self.case == Case::Wall)
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn can_launch(&self) -> bool {
        self.started.is_none()
            && self.cooldown == 0
            && !matches!(self.case, Case::Silence | Case::Ranged | Case::Transformed)
    }
    fn can_strike(&self) -> bool {
        self.started.is_some()
            && self.remaining > 0
            && self.map().is_walkable(TARGET)
            && self.case != Case::Hidden
    }
    fn launch(&mut self) {
        if self.can_launch() {
            self.started = Some(self.clock);
            self.remaining = DURATIONS[self.skill];
            self.cooldown = COOLDOWNS[self.skill];
        }
    }
    fn attack(&mut self) {
        if !self.can_strike() {
            return;
        }
        self.strike = Some(self.clock);
        self.hits += 1;
        self.fire = if self.skill == 0 { 14 } else { 0 };
        self.damage = 20 + self.fire;
        self.healing = if self.skill == 1 { 8 } else { 0 };
    }
    fn advance(&mut self) {
        if self.started.is_none() {
            return;
        }
        self.turn += 1;
        self.remaining = self.remaining.saturating_sub(1);
        self.cooldown = self.cooldown.saturating_sub(1);
    }
    fn strike_age(&self) -> Option<f32> {
        self.strike.map(|c| self.clock - c)
    }
    fn footprint(&self) -> Vec<GridPos> {
        if self.can_launch() {
            vec![PLAYER]
        } else {
            vec![]
        }
    }
}
fn point(at: GridPos, direction: usize, rect: Rect) -> Vec2 {
    let p = position(at, direction);
    vec2(
        rect.x + (p.x as f32 + 0.5) * 64.0,
        rect.y + (p.y as f32 + 0.5) * 64.0,
    )
}
struct Art {
    jaw: Texture2D,
    blood: Texture2D,
    icons: [Texture2D; 2],
    predator: Texture2D,
}
impl Art {
    fn load() -> Self {
        let load = |bytes: &[u8]| {
            let image = Image::from_file_with_format(bytes, Some(ImageFormat::Png)).unwrap();
            let t = Texture2D::from_image(&image);
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            jaw: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-50/jaw-atlas.png"
            )),
            blood: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-40/blood-atlas.png"
            )),
            icons: [
                load(include_bytes!(
                    "../../../assets/skills/agent/BM01-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/BM02-compact.png"
                )),
            ],
            predator: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-39/predator-64.png"
            )),
        }
    }
    fn bodies(&self, s: &State, rect: Rect, effects: bool) {
        let source = point(PLAYER, s.direction, rect);
        let target = point(TARGET, s.direction, rect);
        let age = s.strike_age().unwrap_or(99.0);
        let recoil = if effects {
            crate::combat_visuals::hit_reaction_offset(target, source, f64::from(age), 64.0)
        } else {
            Vec2::ZERO
        };
        if s.case != Case::Hidden && s.map().is_walkable(TARGET) {
            enemy(target + recoil);
        }
        enemy(point(GridPos::new(6, 5), s.direction, rect));
        let lunge = if effects && (0.0..0.20).contains(&age) {
            (age / 0.20 * std::f32::consts::PI).sin() * 8.0
        } else {
            0.0
        };
        let at = source + (target - source).normalize() * lunge;
        if s.case == Case::Transformed {
            draw_texture(&self.predator, at.x - 32.0, at.y - 32.0, WHITE);
        } else if s.case == Case::Ranged {
            actor(
                at,
                game_art::wear(
                    game_art::player(
                        Some("core:bastion"),
                        PlayerWeaponPose::Ranged,
                        Some("core:modele_cogneur_p01"),
                    ),
                    [("core:body_armor".into(), "core:tenue_assaut".into())],
                ),
            );
        } else {
            actor(at, player_visual());
        }
    }
    fn strike(&self, s: &State, rect: Rect) {
        let Some(age) = s.strike_age() else {
            return;
        };
        let source = point(PLAYER, s.direction, rect);
        let target = point(TARGET, s.direction, rect);
        let d = (target - source).normalize();
        let n = vec2(-d.y, d.x);
        if (0.0..0.20).contains(&age) {
            let k = age / 0.20;
            cut(
                target - n * 27.0 - d * 12.0,
                target + n * (20.0 + k * 16.0) + d * 9.0,
                7.0,
                (1.0 - k) * 0.85,
            );
        }
        if (0.03..0.39).contains(&age) {
            let frame = (((age - 0.03) / 0.36 * 6.0) as usize).min(5);
            draw_texture_ex(
                &self.blood,
                target.x - 31.0 + d.x * 18.0,
                target.y - 25.0 + d.y * 18.0,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(frame as f32 * 256.0, 0.0, 256.0, 256.0)),
                    dest_size: Some(vec2(62.0, 62.0)),
                    rotation: d.y.atan2(d.x),
                    ..Default::default()
                },
            );
        }
        if s.skill == 1 && (0.0..0.40).contains(&age) {
            let frame = ((age / 0.40 * 6.0) as usize).min(5);
            let recoil =
                crate::combat_visuals::hit_reaction_offset(target, source, f64::from(age), 64.0);
            let p = target + recoil;
            draw_texture_ex(
                &self.jaw,
                p.x - 46.0,
                p.y - 46.0,
                alpha(WHITE, 0.90),
                DrawTextureParams {
                    source: Some(Rect::new(frame as f32 * 256.0, 0.0, 256.0, 256.0)),
                    dest_size: Some(vec2(92.0, 92.0)),
                    ..Default::default()
                },
            );
        }
    }
    fn buff(&self, s: &State, rect: Rect, interactive: bool) {
        if s.remaining == 0 {
            return;
        }
        let icon_rect = Rect::new(rect.x + 488.0, rect.y + 66.0, 38.0, 38.0);
        draw_rectangle(icon_rect.x, icon_rect.y, 38.0, 38.0, alpha(INK, 0.92));
        draw_rectangle_lines(icon_rect.x, icon_rect.y, 38.0, 38.0, 1.0, DIM);
        draw_texture_ex(
            &self.icons[s.skill],
            icon_rect.x + 3.0,
            icon_rect.y + 3.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                ..Default::default()
            },
        );
        draw_rectangle(icon_rect.x + 23.0, icon_rect.y + 23.0, 14.0, 14.0, INK);
        draw_text_bold(
            s.remaining.to_string(),
            icon_rect.x + 26.0,
            icon_rect.y + 35.0,
            13.0,
            TEXT,
        );
        let hovering = s.case == Case::Hover
            || (interactive && icon_rect.contains(vec2(mouse_position().0, mouse_position().1)));
        if hovering {
            let x = rect.x + 308.0;
            let y = rect.y + 114.0;
            let height = if s.skill == 0 { 83.0 } else { 106.0 };
            draw_rectangle(x, y, 250.0, height, alpha(INK, 0.98));
            draw_rectangle_lines(x, y, 250.0, height, 1.0, DIM);
            draw_text_bold(TITLES[s.skill], x + 10.0, y + 22.0, 17.0, TEXT);
            draw_text(
                if s.skill == 0 {
                    "+14 dégâts de feu / frappe"
                } else {
                    "Vie rendue : 40 %"
                },
                x + 10.0,
                y + 45.0,
                14.0,
                TEXT,
            );
            if s.skill == 1 {
                draw_text(
                    "Soin max : 10 % des PV max / tour",
                    x + 10.0,
                    y + 68.0,
                    14.0,
                    TEXT,
                );
            }
            draw_text(
                format!("Durée restante : {} tours", s.remaining),
                x + 10.0,
                y + height - 15.0,
                14.0,
                TEXT,
            );
        }
    }
}
fn physical_icon(at: Vec2, opacity: f32) {
    let d = vec2(0.7, -0.7);
    draw_line(
        at.x - 4.0,
        at.y + 4.0,
        at.x + 4.0,
        at.y - 4.0,
        2.5,
        alpha(TEXT, opacity),
    );
    draw_line(
        at.x - 4.0,
        at.y,
        at.x,
        at.y + 4.0,
        2.0,
        alpha(TEXT, opacity),
    );
    draw_line(
        at.x - 6.0 * d.x,
        at.y - 6.0 * d.y,
        at.x - 3.0 * d.x,
        at.y - 3.0 * d.y,
        2.0,
        alpha(TEXT, opacity),
    );
}
fn fire_icon(at: Vec2, opacity: f32) {
    draw_triangle(
        at + vec2(0.0, -7.0),
        at + vec2(-5.0, 4.0),
        at + vec2(5.0, 4.0),
        alpha(FIRE, opacity),
    );
    draw_triangle(
        at + vec2(1.0, -3.0),
        at + vec2(-2.0, 4.0),
        at + vec2(3.0, 4.0),
        alpha(Color::new(1.0, 0.90, 0.35, 1.0), opacity),
    );
}
fn feedback(s: &State, rect: Rect) {
    if let Some(age) = s.strike_age().filter(|a| *a < 0.65) {
        let p = point(TARGET, s.direction, rect) + vec2(0.0, -50.0 - age * 20.0);
        let a = (1.0 - age / 0.65).min(1.0);
        if s.fire > 0 {
            draw_text_bold(s.damage.to_string(), p.x - 67.0, p.y, 21.0, alpha(TEXT, a));
            draw_text("(", p.x - 38.0, p.y, 17.0, alpha(TEXT, a));
            physical_icon(p + vec2(-27.0, -6.0), a);
            draw_text("20", p.x - 18.0, p.y, 17.0, alpha(TEXT, a));
            draw_text("+", p.x + 4.0, p.y, 16.0, alpha(TEXT, a));
            fire_icon(p + vec2(25.0, -6.0), a);
            draw_text("14)", p.x + 35.0, p.y, 17.0, alpha(FIRE, a));
        } else {
            draw_text_bold("20", p.x - 29.0, p.y, 21.0, alpha(TEXT, a));
            draw_text("(", p.x - 5.0, p.y, 17.0, alpha(TEXT, a));
            physical_icon(p + vec2(7.0, -6.0), a);
            draw_text("20)", p.x + 16.0, p.y, 17.0, alpha(TEXT, a));
        }
    }
    // Presentation only: reveal healing after the jaw has cleared the actor.
    if let Some(age) = s
        .strike_age()
        .filter(|a| s.healing > 0 && (0.40..0.80).contains(a))
    {
        let q = point(PLAYER, s.direction, rect) + vec2(0.0, -42.0 - (age - 0.40) * 20.0);
        draw_text_bold(
            "+8 PV",
            q.x - 24.0,
            q.y,
            20.0,
            alpha(HEAL, (0.80 - age) / 0.40),
        );
    }
    if let Some(age) = s.started.map(|t| s.clock - t).filter(|a| *a < 0.29) {
        let p = point(PLAYER, s.direction, rect);
        draw_text_bold(
            TITLES[s.skill],
            p.x - 72.0,
            p.y - 44.0 - age * 8.0,
            15.0,
            alpha(TEXT, 1.0 - age / 0.29),
        );
    }
}
fn scene(art: &Art, clip: &Clip, s: &State, rect: Rect, effects: bool, interactive: bool) {
    let map = s.map();
    terrain(&map, s.direction, rect, false);
    if s.started.is_none() {
        preview(&s.footprint(), s.direction, rect, s.clock);
    }
    terrain(&map, s.direction, rect, true);
    clip.apply_for(s.map_index(), s.direction, rect);
    art.bodies(s, rect, effects);
    if effects {
        art.strike(s, rect);
    }
    gl_use_default_material();
    if effects {
        feedback(s, rect);
    }
    art.buff(s, rect, interactive);
    draw_rectangle(
        rect.x + 5.0,
        rect.y + SIZE - 43.0,
        556.0,
        38.0,
        alpha(INK, 0.95),
    );
    draw_text(
        format!(
            "Buff {} tours · recharge {} · frappes {}",
            s.remaining, s.cooldown, s.hits
        ),
        rect.x + 12.0,
        rect.y + SIZE - 27.0,
        14.0,
        TEXT,
    );
    let log = if s.hits > 0 {
        if s.skill == 0 {
            "34 dégâts : 20 physiques + 14 de feu."
        } else {
            "20 dégâts physiques. Faim vampirique : +8 PV."
        }
    } else if s.started.is_none() && !s.can_launch() {
        match s.case {
            Case::Silence => "Silence empêche le lancement.",
            Case::Ranged => "Arme de mêlée active requise.",
            _ => "Forme humaine requise pour lancer le buff.",
        }
    } else {
        "Scène préparée · valeurs de frappe illustratives."
    };
    draw_text(log, rect.x + 12.0, rect.y + SIZE - 10.0, 13.0, DIM);
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
        Case::Quiet,
        Case::Wall,
        Case::Hidden,
        Case::Silence,
        Case::Ranged,
        Case::Transformed,
        Case::Hover,
    ];
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Buffs de mêlée · ANIM-S21", 20.0, 33.0, 24.0, TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 88.0 + i as f32 * 42.0, 330.0, 34.0),
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
                    185.0 + (i / 2) as f32 * 37.0,
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
                Rect::new(20.0, 274.0 + i as f32 * 33.0, 330.0, 28.0),
                case.title(),
                true,
            ) {
                s = State::new(s.skill, *case, s.direction);
            }
        }
        if button(
            Rect::new(20.0, 560.0, 330.0, 34.0),
            format!(
                "Lancer · {} énergie · recharge {}",
                COSTS[s.skill], s.cooldown
            )
            .as_str(),
            s.can_launch(),
        ) || (is_key_pressed(KeyCode::Enter) && s.can_launch())
        {
            s.launch();
        }
        let strike_ready = s.can_strike() && s.strike_age().is_none_or(|a| a > 0.65);
        if button(
            Rect::new(20.0, 602.0, 330.0, 34.0),
            "Frapper (scène préparée)",
            strike_ready,
        ) || (is_key_pressed(KeyCode::A) && strike_ready)
        {
            s.attack();
        }
        if button(
            Rect::new(20.0, 644.0, 330.0, 34.0),
            "+1 tour",
            s.started.is_some(),
        ) || (is_key_pressed(KeyCode::Space) && s.started.is_some())
        {
            s.advance();
        }
        if button(Rect::new(20.0, 686.0, 160.0, 32.0), "Recommencer", true)
            || is_key_pressed(KeyCode::R)
        {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(190.0, 686.0, 160.0, 32.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
        }
        scene(
            &art,
            &clip,
            &s,
            Rect::new(410.0, 88.0, SIZE, SIZE),
            true,
            true,
        );
        draw_text(
            "Icône du buff : nom, effet et tours restants au survol.",
            410.0,
            699.0,
            15.0,
            TEXT,
        );
        draw_text(
            "20 dégâts physiques préparés ; aucune formule de variance ajoutée.",
            410.0,
            724.0,
            13.0,
            DIM,
        );
        draw_text(
            "Monstres actuels 32 px agrandis ; adaptation prévue ensuite.",
            410.0,
            746.0,
            13.0,
            DIM,
        );
        draw_text(
            "Miss, absorption, vie pleine et plafond atteint : cas encore à définir.",
            410.0,
            768.0,
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
    let controls = [
        14, 17, 24, 26, 28, 31, 36, 47, 52, 53, 55, 58, 63, 76, 88, 112, 127,
    ];
    let mut entries = Vec::new();
    for skill in 0..2 {
        for (case, direction, label) in [
            (Case::Open, 2, "HAUT"),
            (Case::Open, 0, "DROITE"),
            (Case::Open, 1, "GAUCHE"),
            (Case::Open, 3, "BAS"),
            (Case::Quiet, 2, "ATTENTE"),
            (Case::Wall, 2, "CLOISON"),
            (Case::Hidden, 2, "CACHE"),
            (Case::Silence, 2, "SILENCE"),
            (Case::Ranged, 2, "DISTANCE"),
            (Case::Transformed, 2, "TRANSFORME"),
            (Case::Hover, 2, "SURVOL"),
        ] {
            let folder = format!("{}-{label}", IDS[skill]);
            let dir = output.join(&folder);
            std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
            let mut s = State::new(skill, case, direction);
            let permitted = s.can_launch();
            let mut timeline = Vec::new();
            for frame in 0..FRAMES {
                s.clock = frame as f32 / 24.0;
                if frame == 17 {
                    s.launch();
                }
                if case != Case::Quiet && TURN_FRAMES.contains(&frame) {
                    s.advance();
                }
                if frame == 24 || (frame == 53 && case != Case::Quiet) {
                    s.attack();
                }
                timeline.push(serde_json::json!({"frame":frame,"turn":s.turn,"cooldown":s.cooldown,"remaining":s.remaining,"launched":s.started.is_some(),"hits":s.hits,"damage":s.damage,"fire":s.fire,"heal":s.healing,"strike_age":s.strike_age(),"icon_visible":s.remaining>0,"preview_visible":s.started.is_none()&&s.can_launch()}));
                for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                    set_camera(&camera);
                    clear_background(INK);
                    draw_text_bold(
                        format!("{} · {}", 51 + skill, TITLES[skill]),
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
                        false,
                    );
                    draw_text(
                        format!(
                            "Rang 11 · durée {} tours · coût {} · recharge {}",
                            DURATIONS[skill], COSTS[skill], COOLDOWNS[skill]
                        ),
                        32.0,
                        684.0,
                        14.0,
                        TEXT,
                    );
                    draw_text(
                        "Frappe 20 préparée ; monstres 32 px temporaires. Aucun équilibrage nouveau.",
                        32.0,
                        707.0,
                        12.0,
                        DIM,
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
            entries.push(serde_json::json!({"folder":folder,"variant":format!("{}-A01-{label}",IDS[skill]),"id":IDS[skill],"number":51+skill,"title":TITLES[skill],"case":label,"case_title":case.title(),"direction":direction,"permitted":permitted,"strike_permitted":s.can_strike()||s.hits>0,"quiet":case==Case::Quiet,"cooldown":COOLDOWNS[skill],"duration":DURATIONS[skill],"control_frames":controls,"turn_frames":if case==Case::Quiet{vec![]}else{TURN_FRAMES.to_vec()},"tile":64,"frames":FRAMES,"fps":24,"map":rows.into_iter().map(|row|row.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n")}));
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
    fn quiet_wait_keeps_buffs_and_cooldowns_until_explicit_turns() {
        for skill in 0..2 {
            let mut s = State::new(skill, Case::Quiet, 2);
            s.launch();
            s.clock = 100.0;
            assert_eq!(s.remaining, DURATIONS[skill]);
            assert_eq!(s.cooldown, COOLDOWNS[skill]);
            for _ in 0..DURATIONS[skill] {
                s.advance();
            }
            assert_eq!(s.remaining, 0);
            assert!(!s.can_strike());
        }
    }
    #[test]
    fn wrong_weapon_silence_and_transformation_refuse_without_launch() {
        for skill in 0..2 {
            for case in [Case::Ranged, Case::Silence, Case::Transformed] {
                let mut s = State::new(skill, case, 2);
                s.launch();
                s.attack();
                assert!(s.started.is_none());
                assert_eq!((s.cooldown, s.hits, s.remaining), (0, 0, 0));
            }
        }
    }
    #[test]
    fn only_successful_visible_prepared_hits_retrigger_feedback() {
        for skill in 0..2 {
            for case in [Case::Wall, Case::Hidden] {
                let mut s = State::new(skill, case, 2);
                s.launch();
                s.attack();
                assert!(s.started.is_some());
                assert_eq!(s.hits, 0);
                assert!(s.strike.is_none());
            }
            let mut s = State::new(skill, Case::Open, 2);
            s.launch();
            s.attack();
            let first = s.strike;
            s.clock = 2.0;
            s.advance();
            s.attack();
            assert_ne!(s.strike, first);
            assert_eq!(s.hits, 2);
            assert_eq!(s.damage, if skill == 0 { 34 } else { 20 });
            assert_eq!(s.healing, if skill == 1 { 8 } else { 0 });
        }
    }
}
