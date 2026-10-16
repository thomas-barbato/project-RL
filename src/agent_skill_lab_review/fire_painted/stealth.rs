//! ANIM-S28: prepared stealth appearance and step economy; no campaign perception.
use super::super::melee::actor;
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;
const SIZE: f32 = 576.0;
const START: GridPos = GridPos::new(4, 6);
const IDS: [&str; 2] = ["SF00", "SF01"];
const TITLES: [&str; 2] = ["Furtivité", "Économie de mouvement furtif"];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#...#...#\n#.......#\n#.......#\n#########",
];
const FRAMES: usize = 160;
const ACTION_FRAMES: [usize; 4] = [64, 88, 112, 136];
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Case {
    Open,
    Quiet,
    Wall,
    Menu,
    Heal,
    Offense,
    Zero,
    Exhaust,
    RankNine,
    BonusRank,
}
impl Case {
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Quatre pas",
            Self::Quiet => "Attente sans action",
            Self::Wall => "Pas bloqué par un obstacle",
            Self::Menu => "Consultation d’un menu",
            Self::Heal => "Soin reçu",
            Self::Offense => "Action offensive",
            Self::Zero => "Énergie nulle au premier pas",
            Self::Exhaust => "Réserve pour trois pas furtifs",
            Self::RankNine => "Passif au rang 9",
            Self::BonusRank => "Passif au rang 12 avec équipement",
        }
    }
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    begun: Option<f32>,
    event: Option<f32>,
    fade: Option<(f32, f32, f32)>,
    active: bool,
    step: u16,
    turn: u16,
    energy: u16,
    hp: u16,
    spent: u16,
    walks: u16,
    player: GridPos,
    from: GridPos,
    menu: bool,
    notice: &'static str,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        let cost = if skill == 0 {
            11
        } else if case == Case::RankNine {
            2
        } else {
            1
        };
        Self {
            skill,
            case,
            direction,
            clock: 0.0,
            begun: None,
            event: None,
            fade: None,
            active: false,
            step: 0,
            turn: 0,
            energy: match case {
                Case::Zero => 0,
                Case::Exhaust => cost * 3,
                _ => 100,
            },
            hp: 50,
            spent: 0,
            walks: 0,
            player: START,
            from: START,
            menu: false,
            notice: "",
        }
    }
    fn map_index(&self) -> usize {
        usize::from(self.case == Case::Wall)
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn cost(&self) -> u16 {
        if self.skill == 0 {
            11
        } else if self.case == Case::RankNine {
            2
        } else {
            1
        }
    }
    fn rank(&self) -> u16 {
        if self.skill == 0 {
            1
        } else {
            match self.case {
                Case::RankNine => 9,
                Case::BonusRank => 12,
                _ => 10,
            }
        }
    }
    fn opacity(&self) -> f32 {
        self.fade
            .map_or(if self.active { 0.56 } else { 1.0 }, |(t, a, b)| {
                a + (b - a) * ((self.clock - t) / 0.24).clamp(0.0, 1.0)
            })
    }
    fn set_mode(&mut self, active: bool) {
        let before = self.opacity();
        self.active = active;
        self.fade = Some((self.clock, before, if active { 0.56 } else { 1.0 }));
    }
    fn begin(&mut self) {
        if self.begun.is_some() {
            return;
        }
        self.begun = Some(self.clock);
        self.set_mode(true);
        self.turn = 1;
    }
    fn advance(&mut self) {
        if self.begun.is_none() || self.clock - self.begun.unwrap() < 0.85 || self.step >= 4 {
            return;
        }
        self.step += 1;
        match self.case {
            Case::Menu => {
                self.menu = !self.menu;
                self.notice = "Menu consulté : mode conservé.";
                return;
            }
            Case::Heal => {
                self.turn += 1;
                if self.step == 1 {
                    self.hp += 5;
                    self.event = Some(self.clock);
                    self.notice = "Soin reçu : mode conservé.";
                }
                return;
            }
            Case::Offense => {
                self.turn += 1;
                if self.step == 1 {
                    self.set_mode(false);
                    self.event = Some(self.clock);
                    self.notice = "Action offensive : camouflage levé.";
                }
                return;
            }
            _ => {}
        }
        let next = GridPos::new(self.player.x, self.player.y - 1);
        if !self.map().is_walkable(next) {
            self.notice = "Pas bloqué : aucune énergie dépensée.";
            return;
        }
        // Only accepted exact-payment and zero-energy cases are prepared here.
        // Positive-but-insufficient balances, regeneration and action ordering are not modeled.
        if self.active && self.energy == 0 {
            self.set_mode(false);
            self.notice = "Sans énergie, tenter de vous déplacer met fin à la furtivité.";
        }
        if self.active {
            assert!(self.energy >= self.cost());
            self.energy -= self.cost();
            self.spent += self.cost();
        }
        self.from = self.player;
        self.player = next;
        self.walks += 1;
        self.turn += 1;
        self.event = Some(self.clock);
    }
    fn variant(&self) -> String {
        format!("{}-A01", IDS[self.skill])
    }
}
fn point(p: GridPos, d: usize, r: Rect) -> Vec2 {
    let p = position(p, d);
    vec2(
        r.x + (p.x as f32 + 0.5) * 64.0,
        r.y + (p.y as f32 + 0.5) * 64.0,
    )
}
fn player_point(s: &State, r: Rect) -> Vec2 {
    let at = point(s.player, s.direction, r);
    if s.walks == 0 {
        return at;
    }
    let q = s
        .event
        .map_or(1.0, |t| ((s.clock - t) / 0.12).clamp(0.0, 1.0));
    point(s.from, s.direction, r).lerp(at, q)
}
fn visual() -> Visual {
    game_art::wear(
        game_art::player(
            Some("core:traqueur"),
            PlayerWeaponPose::Ranged,
            Some("core:fusil_de_patrouille"),
        ),
        [("core:body_armor".into(), "core:tenue_traque".into())],
    )
}
struct Art {
    icons: [Texture2D; 2],
}
impl Art {
    fn load() -> Self {
        let load = |b: &[u8]| {
            let t = Texture2D::from_file_with_format(b, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            icons: [
                load(include_bytes!(
                    "../../../assets/skills/agent/SF00-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/SF01-compact.png"
                )),
            ],
        }
    }
    fn cloak(&self, s: &State, r: Rect) {
        let Some((t, _, _)) = s.fade else {
            return;
        };
        let q = (s.clock - t) / 0.24;
        if !(0.0..1.0).contains(&q) {
            return;
        }
        let p = player_point(s, r);
        let y = p.y - 29.0 + q * 56.0;
        let life = (1.0 - q) * 0.85;
        let c = alpha(Color::new(0.40, 0.78, 0.90, 1.0), life);
        // A short technological scan, then only the translucent original layers remain.
        for side in [-1.0, 1.0] {
            let a = vec2(p.x + side * 16.0, y - 8.0);
            let b = vec2(p.x + side * 24.0, y);
            let d = vec2(p.x + side * 17.0, y + 7.0);
            draw_line(a.x, a.y, b.x, b.y, 4.7, alpha(INK, life));
            draw_line(b.x, b.y, d.x, d.y, 4.7, alpha(INK, life));
            draw_line(a.x, a.y, b.x, b.y, 2.4, c);
            draw_line(b.x, b.y, d.x, d.y, 2.4, c);
        }
    }
    fn bodies(&self, s: &State, r: Rect, effects: bool) {
        for at in [GridPos::new(2, 3), GridPos::new(6, 3)] {
            actor(
                point(at, s.direction, r),
                Visual::Bestiary {
                    number: 12,
                    corpse: false,
                },
            );
        }
        let p = player_point(s, r);
        game_art::draw_world(
            Rect::new(p.x - 32.0, p.y - 32.0, 64.0, 64.0),
            visual(),
            if effects { s.opacity() } else { 1.0 },
        );
    }
    fn status(&self, s: &State, r: Rect, interactive: bool) {
        if s.skill == 1 {
            draw_rectangle(r.x + 12.0, r.y + 12.0, 278.0, 70.0, alpha(INK, 0.96));
            draw_texture_ex(
                &self.icons[1],
                r.x + 20.0,
                r.y + 28.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(32.0, 32.0)),
                    ..Default::default()
                },
            );
            draw_text("Passif permanent", r.x + 61.0, r.y + 34.0, 14.0, DIM);
            draw_text(
                format!("Rang {} · {} énergie par pas", s.rank(), s.cost()),
                r.x + 61.0,
                r.y + 58.0,
                14.0,
                TEXT,
            );
        }
        if !s.active {
            return;
        }
        let icon = Rect::new(r.x + 488.0, r.y + 16.0, 40.0, 40.0);
        draw_rectangle(icon.x, icon.y, icon.w, icon.h, alpha(INK, 0.97));
        draw_rectangle_lines(icon.x, icon.y, icon.w, icon.h, 1.0, DIM);
        draw_texture_ex(
            &self.icons[0],
            icon.x + 4.0,
            icon.y + 4.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                ..Default::default()
            },
        );
        if interactive && icon.contains(vec2(mouse_position().0, mouse_position().1)) {
            draw_rectangle(r.x + 270.0, r.y + 68.0, 282.0, 96.0, alpha(INK, 0.98));
            draw_text_bold("Furtivité", r.x + 280.0, r.y + 91.0, 16.0, TEXT);
            draw_text(
                format!("Énergie par pas : {}", s.cost()),
                r.x + 280.0,
                r.y + 116.0,
                14.0,
                TEXT,
            );
            draw_text(
                "Distance de repérage : 2 cases",
                r.x + 280.0,
                r.y + 143.0,
                14.0,
                TEXT,
            );
        }
    }
}
fn feedback(s: &State, r: Rect) {
    if let Some(t) = s.begun {
        let a = s.clock - t;
        if a < 0.38 {
            let p = player_point(s, r);
            draw_text_bold(
                "Furtivité",
                p.x - 29.0,
                p.y - 45.0 - a * 8.0,
                15.0,
                alpha(TEXT, 1.0 - a / 0.38),
            );
        }
    }
    if let Some(t) = s.event {
        let a = s.clock - t;
        if a < 0.55 {
            let p = player_point(s, r);
            let text = if s.case == Case::Heal {
                "+5 PV".to_string()
            } else if s.active && s.walks > 0 {
                format!("-{} énergie", s.cost())
            } else {
                String::new()
            };
            if !text.is_empty() {
                draw_text_bold(
                    text,
                    p.x - 38.0,
                    p.y - 43.0 - a * 14.0,
                    16.0,
                    alpha(
                        if s.case == Case::Heal {
                            Color::new(0.5, 1.0, 0.4, 1.0)
                        } else {
                            TEXT
                        },
                        1.0 - a / 0.55,
                    ),
                );
            }
        }
    }
}
fn scene(art: &Art, clip: &Clip, s: &State, r: Rect, effects: bool, interactive: bool) {
    terrain(&s.map(), s.direction, r, false);
    if s.begun.is_none() {
        preview(&[START], s.direction, r, s.clock);
    }
    terrain(&s.map(), s.direction, r, true);
    clip.apply_for(s.map_index(), s.direction, r);
    art.bodies(s, r, effects);
    if effects {
        art.cloak(s, r);
    }
    gl_use_default_material();
    art.status(s, r, interactive);
    feedback(s, r);
    if s.menu {
        draw_rectangle(r.x + 144.0, r.y + 180.0, 288.0, 130.0, alpha(INK, 0.97));
        draw_rectangle_lines(r.x + 144.0, r.y + 180.0, 288.0, 130.0, 2.0, DIM);
        draw_text_bold("Inventaire · aperçu", r.x + 162.0, r.y + 214.0, 18.0, TEXT);
        draw_text(
            "Consultation sans passage de tour",
            r.x + 162.0,
            r.y + 247.0,
            14.0,
            TEXT,
        );
        draw_text("Furtivité conservée", r.x + 162.0, r.y + 278.0, 14.0, DIM);
    }
    draw_rectangle(r.x + 5.0, r.y + SIZE - 64.0, 566.0, 59.0, alpha(INK, 0.95));
    draw_text(
        format!(
            "Tour {} · énergie {} · dépensée {} · PV {}",
            s.turn, s.energy, s.spent, s.hp
        ),
        r.x + 12.0,
        r.y + SIZE - 46.0,
        14.0,
        TEXT,
    );
    draw_text(
        if s.active {
            "Furtivité active"
        } else {
            "À découvert"
        },
        r.x + 12.0,
        r.y + SIZE - 27.0,
        14.0,
        DIM,
    );
    draw_text(s.notice, r.x + 12.0, r.y + SIZE - 10.0, 11.5, TEXT);
}
pub(crate) async fn run() {
    request_new_screen_size(1280.0, 820.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let clip = Clip::load_maps(&MAPS, START);
    let mut s = State::new(0, Case::Open, 2);
    let mut paused = false;
    let cases = [
        Case::Open,
        Case::Quiet,
        Case::Wall,
        Case::Menu,
        Case::Heal,
        Case::Offense,
        Case::Zero,
        Case::Exhaust,
        Case::RankNine,
        Case::BonusRank,
    ];
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Furtivité · ANIM-S28", 20.0, 33.0, 24.0, TEXT);
        for (i, t) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 80.0 + i as f32 * 40.0, 366.0, 34.0),
                t,
                true,
            ) {
                s = State::new(i, Case::Open, s.direction);
            }
        }
        for (i, t) in DIRS.iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 174.0,
                    179.0 + (i / 2) as f32 * 36.0,
                    166.0,
                    30.0,
                ),
                t,
                true,
            ) {
                s = State::new(s.skill, s.case, i);
            }
        }
        for (i, c) in cases.iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 185.0,
                    270.0 + (i / 2) as f32 * 35.0,
                    178.0,
                    30.0,
                ),
                c.title(),
                true,
            ) {
                s = State::new(s.skill, *c, s.direction);
            }
        }
        if button(
            Rect::new(20.0, 465.0, 350.0, 34.0),
            "Activer Furtivité",
            s.begun.is_none(),
        ) || is_key_pressed(KeyCode::Enter)
        {
            s.begin();
        }
        if button(
            Rect::new(20.0, 507.0, 350.0, 34.0),
            "Action suivante",
            s.begun.is_some() && s.clock - s.begun.unwrap_or(s.clock) >= 0.85 && s.step < 4,
        ) || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(20.0, 549.0, 160.0, 34.0), "Recommencer", true)
            || is_key_pressed(KeyCode::R)
        {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(190.0, 549.0, 160.0, 34.0),
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
            "Étapes préparées · perception et régénération non simulées.",
            410.0,
            704.0,
            13.0,
            DIM,
        );
        draw_text(
            "Modèles existants conservés ; aucune partie modifiée.",
            410.0,
            728.0,
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
    let clip = Clip::load_maps(&MAPS, START);
    let target = render_target(640, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 640.0, 720.0))
    };
    let controls = [
        14, 17, 20, 22, 24, 26, 28, 32, 36, 44, 60, 64, 68, 72, 76, 84, 88, 92, 100, 112, 116, 124,
        136, 140, 148, 158,
    ];
    let mut entries = Vec::new();
    for skill in 0..2 {
        let mut cases = vec![
            (Case::Open, 2, "HAUT"),
            (Case::Open, 0, "DROITE"),
            (Case::Open, 1, "GAUCHE"),
            (Case::Open, 3, "BAS"),
            (Case::Quiet, 2, "ATTENTE"),
            (Case::Wall, 2, "OBSTACLE"),
            (Case::Menu, 2, "MENU"),
            (Case::Heal, 2, "SOIN"),
            (Case::Offense, 2, "OFFENSIVE"),
            (Case::Zero, 2, "ZERO-ENERGIE"),
            (Case::Exhaust, 2, "EPUISEMENT"),
        ];
        if skill == 1 {
            cases.extend([
                (Case::RankNine, 2, "RANG-9"),
                (Case::BonusRank, 2, "BONUS-EQUIPEMENT"),
            ]);
        }
        for (case, direction, label) in cases {
            let mut s = State::new(skill, case, direction);
            let folder = format!("{}-{label}", IDS[skill]);
            let dir = output.join(&folder);
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let mut timeline = Vec::new();
            for frame in 0..FRAMES {
                s.clock = frame as f32 / 24.0;
                if frame == 17 {
                    s.begin();
                }
                if case != Case::Quiet && ACTION_FRAMES.contains(&frame) {
                    s.advance();
                }
                timeline.push(serde_json::json!({"frame":frame,"begun":s.begun.is_some(),"active":s.active,"step":s.step,"turn":s.turn,"energy":s.energy,"spent":s.spent,"hp":s.hp,"walks":s.walks,"player":[s.player.x,s.player.y],"opacity":s.opacity(),"menu":s.menu,"cost":s.cost(),"rank":s.rank(),"preview_visible":s.begun.is_none(),"notice":s.notice}));
                for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                    set_camera(&camera);
                    clear_background(INK);
                    draw_text_bold(
                        format!("{} · {}", 65 + skill, TITLES[skill]),
                        32.0,
                        35.0,
                        if skill == 1 { 22.0 } else { 24.0 },
                        TEXT,
                    );
                    draw_text(
                        format!("{} · {label} · étape {}", s.variant(), s.step),
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
                        if skill == 0 {
                            "Sans passif : 11 énergie par pas · à l’arrêt : 0"
                        } else {
                            "Avec Économie : 1 énergie par pas au rang 10 · minimum 1"
                        },
                        32.0,
                        684.0,
                        13.0,
                        TEXT,
                    );
                    draw_text(
                        "Transparence et déplacements préparés · aucune IA modifiée.",
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
            entries.push(serde_json::json!({"folder":folder,"variant":format!("{}-{label}",s.variant()),"id":IDS[skill],"number":65+skill,"title":TITLES[skill],"case":label,"case_title":case.title(),"direction":direction,"permitted":true,"quiet":case==Case::Quiet,"control_frames":controls,"action_frames":if case==Case::Quiet{vec![]}else{ACTION_FRAMES.to_vec()},"tile":64,"frames":FRAMES,"fps":24,"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n")}));
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
