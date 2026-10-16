//! ANIM-S25: prepared rocket barrage and stun; no campaign, loot or RNG changes.
use super::super::melee::actor;
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;

const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 6);
const TARGETS: [GridPos; 4] = [
    GridPos::new(3, 3),
    GridPos::new(4, 3),
    GridPos::new(5, 3),
    GridPos::new(6, 5),
];
const IDS: [&str; 2] = ["HL01", "HL02"];
const TITLES: [&str; 2] = ["Barrage", "Impact assommant"];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#########\n#.......#\n#.......#\n#.......#\n#########",
];
const FRAMES: usize = 128;
const ACTION_FRAMES: [usize; 3] = [64, 88, 112];
const FLIGHT: f32 = 0.20;
const BLAST: f32 = 0.42;
const SALVO_GAP: f32 = 0.045;
const GOLD: Color = Color::new(1.0, 0.73, 0.23, 1.0);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Case {
    Open,
    Quiet,
    Wall,
    Empty,
    One,
    NoBox,
    Melee,
    Silence,
    Elite,
    Boss,
}
impl Case {
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Trois ennemis",
            Self::Quiet => "Attente sans action",
            Self::Wall => "Cloison",
            Self::Empty => "Aucun ennemi",
            Self::One => "Un seul ennemi",
            Self::NoBox => "Boîte absente",
            Self::Melee => "Arme de mêlée active",
            Self::Silence => "Silence",
            Self::Elite => "Élite · jet préparé à 50 %",
            Self::Boss => "Boss · dégâts sans contrôle",
        }
    }
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    launch: Option<f32>,
    step: u16,
    turn: u16,
    energy: u16,
    cooldown: u16,
    damage: [u16; 4],
    stun: [u16; 4],
    reaction: [Option<f32>; 4],
    resolved: [bool; 3],
    stun_checked: [bool; 4],
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.0,
            launch: None,
            step: 0,
            turn: 0,
            energy: 100,
            cooldown: 0,
            damage: [0; 4],
            stun: [0; 4],
            reaction: [None; 4],
            resolved: [false; 3],
            stun_checked: [false; 4],
        }
    }
    fn map_index(&self) -> usize {
        usize::from(self.case == Case::Wall)
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn visible(&self, n: usize) -> bool {
        self.case != Case::Empty && (self.case != Case::One || n == 1)
    }
    fn allowed(&self) -> bool {
        !matches!(
            self.case,
            Case::Wall | Case::Empty | Case::NoBox | Case::Melee | Case::Silence
        )
    }
    fn aims(&self) -> Vec<GridPos> {
        if !self.allowed() {
            return Vec::new();
        }
        if self.skill == 1 || self.case == Case::One {
            vec![TARGETS[1]]
        } else {
            TARGETS[..3].to_vec()
        }
    }
    fn cells(&self) -> Vec<GridPos> {
        let map = self.map();
        let mut cells = std::collections::BTreeSet::new();
        for target in self.aims() {
            for (dx, dy) in [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)] {
                let at = GridPos::new(target.x + dx, target.y + dy);
                if map.is_walkable(at) {
                    cells.insert((at.x, at.y));
                }
            }
        }
        cells.into_iter().map(|(x, y)| GridPos::new(x, y)).collect()
    }
    fn begin(&mut self) {
        if self.launch.is_some() || !self.allowed() {
            return;
        }
        self.launch = Some(self.clock);
        self.turn = 1;
        self.energy -= if self.skill == 0 { 18 } else { 14 };
        self.cooldown = if self.skill == 0 { 5 } else { 6 };
    }
    fn age(&self) -> Option<f32> {
        self.launch.map(|t| self.clock - t)
    }
    fn resolve(&mut self) {
        let Some(age) = self.age() else {
            return;
        };
        for (salvo, at) in self.aims().into_iter().enumerate() {
            let impact = FLIGHT + salvo as f32 * SALVO_GAP;
            if self.resolved[salvo] || age < impact {
                continue;
            }
            self.resolved[salvo] = true;
            for (n, target) in TARGETS.into_iter().enumerate() {
                if !self.visible(n) || (target.x - at.x).pow(2) + (target.y - at.y).pow(2) > 1 {
                    continue;
                }
                self.damage[n] += if self.skill == 0 { 40 } else { 36 };
                self.reaction[n] = Some(self.launch.unwrap() + impact);
                if self.skill == 1 && !self.stun_checked[n] {
                    self.stun_checked[n] = true;
                    // Fixed 50% sample: ordinary succeeds; elite45% and boss0% fail.
                    if n != 1 || !matches!(self.case, Case::Elite | Case::Boss) {
                        self.stun[n] = 2;
                    }
                }
            }
        }
    }
    fn advance(&mut self) {
        if self.launch.is_none() || self.age().unwrap_or(0.0) < 0.72 {
            return;
        }
        self.step += 1;
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        for value in &mut self.stun {
            *value = value.saturating_sub(1);
        }
    }
    fn variant(&self) -> String {
        format!("{}-A01", IDS[self.skill])
    }
}
fn point(at: GridPos, direction: usize, rect: Rect) -> Vec2 {
    let p = position(at, direction);
    vec2(
        rect.x + (p.x as f32 + 0.5) * 64.0,
        rect.y + (p.y as f32 + 0.5) * 64.0,
    )
}
fn reaction(s: &State, n: usize, rect: Rect, effects: bool) -> Vec2 {
    if !effects {
        return Vec2::ZERO;
    }
    s.reaction[n].map_or(Vec2::ZERO, |t| {
        crate::combat_visuals::hit_reaction_offset(
            point(TARGETS[n], s.direction, rect),
            point(PLAYER, s.direction, rect),
            f64::from(s.clock - t),
            64.0,
        )
    })
}
struct Art {
    rocket: Texture2D,
    blast: Texture2D,
    blood: Texture2D,
    status: Texture2D,
}
impl Art {
    fn load() -> Self {
        let load = |b: &[u8]| {
            let t = Texture2D::from_file_with_format(b, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            rocket: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-55/rocket-sprite.png"
            )),
            blast: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-20/burst-atlas.png"
            )),
            blood: load(include_bytes!(
                "../../../artifacts/refonte-agent-2026-10-06/skill-animation-review-40/blood-atlas.png"
            )),
            status: load(include_bytes!(
                "../../../assets/skills/agent/HL02-compact.png"
            )),
        }
    }
    fn effects(&self, s: &State, rect: Rect) {
        let Some(age) = s.age() else {
            return;
        };
        let origin = point(PLAYER, s.direction, rect);
        for (salvo, target) in s.aims().into_iter().enumerate() {
            let a = age - salvo as f32 * SALVO_GAP;
            let at = point(target, s.direction, rect);
            let d = (at - origin).normalize_or_zero();
            if (0.0..FLIGHT).contains(&a) {
                let from = origin + d * 30.0;
                let p = from.lerp(at, (a / FLIGHT).clamp(0.0, 1.0));
                let width = 50.0;
                let height = width * self.rocket.height() / self.rocket.width();
                draw_texture_ex(
                    &self.rocket,
                    p.x - width / 2.0,
                    p.y - height / 2.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(width, height)),
                        rotation: d.y.atan2(d.x),
                        pivot: Some(p),
                        ..Default::default()
                    },
                );
            }
            let impact = a - FLIGHT;
            if (0.0..BLAST).contains(&impact) {
                let f = ((impact / BLAST * 6.0) as usize).min(5);
                let size = if s.skill == 0 { 134.0 } else { 146.0 };
                draw_texture_ex(
                    &self.blast,
                    at.x - size * 132.0 / 256.0,
                    at.y - size * 146.0 / 256.0,
                    alpha(WHITE, 0.90),
                    DrawTextureParams {
                        source: Some(Rect::new(f as f32 * 256.0, 0.0, 256.0, 256.0)),
                        dest_size: Some(vec2(size, size)),
                        ..Default::default()
                    },
                );
                if s.skill == 1 {
                    let q = impact / BLAST;
                    draw_circle_lines(at.x, at.y, 12.0 + q * 59.0, 4.0, alpha(INK, 1.0 - q));
                    draw_circle_lines(
                        at.x,
                        at.y,
                        12.0 + q * 59.0,
                        2.0,
                        alpha(GOLD, (1.0 - q) * 0.85),
                    );
                }
            }
        }
    }
    fn bodies(&self, s: &State, rect: Rect, effects: bool) {
        for (n, at) in TARGETS.into_iter().enumerate() {
            if !s.visible(n) {
                continue;
            }
            actor(
                point(at, s.direction, rect) + reaction(s, n, rect, effects),
                Visual::Bestiary {
                    number: 12,
                    corpse: false,
                },
            );
        }
        actor(
            point(PLAYER, s.direction, rect),
            game_art::wear(
                game_art::player(
                    Some("core:bastion"),
                    if s.case == Case::Melee {
                        PlayerWeaponPose::Melee
                    } else {
                        PlayerWeaponPose::Ranged
                    },
                    Some(if s.case == Case::Melee {
                        "core:melee_epee_longue"
                    } else {
                        "core:lance_roquettes"
                    }),
                ),
                [("core:body_armor".into(), "core:tenue_assaut".into())],
            ),
        );
    }
    fn wounds(&self, s: &State, rect: Rect) {
        for n in 0..4 {
            let Some(a) = s.reaction[n]
                .map(|t| s.clock - t)
                .filter(|a| (0.0..0.30).contains(a))
            else {
                continue;
            };
            let at = point(TARGETS[n], s.direction, rect) + reaction(s, n, rect, true);
            let f = ((a / 0.30 * 6.0) as usize).min(5);
            draw_texture_ex(
                &self.blood,
                at.x - 31.0,
                at.y - 25.0,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(f as f32 * 256.0, 0.0, 256.0, 256.0)),
                    dest_size: Some(vec2(62.0, 62.0)),
                    ..Default::default()
                },
            );
        }
    }
    fn statuses(&self, s: &State, rect: Rect) {
        for n in 0..4 {
            if s.stun[n] == 0 {
                continue;
            }
            let at = point(TARGETS[n], s.direction, rect)
                + reaction(s, n, rect, true)
                + vec2(0.0, -45.0);
            // Grounded brass arcs and sparks, kept above the actor rather than its tile.
            for i in 0..3 {
                let angle = s.clock * 4.0 + i as f32 * std::f32::consts::TAU / 3.0;
                let p = at + vec2(angle.cos() * 14.0, angle.sin() * 5.0);
                draw_line(p.x - 4.0, p.y, p.x + 4.0, p.y, 4.0, INK);
                draw_line(p.x, p.y - 4.0, p.x, p.y + 4.0, 4.0, INK);
                draw_line(p.x - 3.0, p.y, p.x + 3.0, p.y, 2.4, GOLD);
                draw_line(p.x, p.y - 3.0, p.x, p.y + 3.0, 2.4, GOLD);
            }
        }
        if s.stun.iter().all(|n| *n == 0) {
            return;
        }
        draw_rectangle(rect.x + 12.0, rect.y + 12.0, 244.0, 56.0, alpha(INK, 0.97));
        draw_text(
            "Analyse · états des cibles",
            rect.x + 20.0,
            rect.y + 29.0,
            13.0,
            DIM,
        );
        draw_texture_ex(
            &self.status,
            rect.x + 20.0,
            rect.y + 36.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(24.0, 24.0)),
                ..Default::default()
            },
        );
        draw_text(
            format!("Assommement : {} tours", s.stun.iter().max().unwrap()),
            rect.x + 52.0,
            rect.y + 53.0,
            14.0,
            TEXT,
        );
    }
}
fn feedback(s: &State, rect: Rect) {
    if let Some(a) = s.age().filter(|a| *a < 0.38) {
        let p = point(PLAYER, s.direction, rect);
        draw_text_bold(
            TITLES[s.skill],
            p.x - 62.0,
            p.y - 45.0 - a * 8.0,
            15.0,
            alpha(TEXT, 1.0 - a / 0.38),
        );
    }
    for n in 0..4 {
        let Some(a) = s.reaction[n]
            .map(|t| s.clock - t)
            .filter(|a| (0.0..0.58).contains(a))
        else {
            continue;
        };
        let at = point(TARGETS[n], s.direction, rect);
        let c = alpha(TEXT, 1.0 - a / 0.58);
        draw_text_bold(
            s.damage[n].to_string(),
            at.x - 12.0,
            at.y - 33.0 - a * 12.0,
            19.0,
            c,
        );
        draw_text("(", at.x - 18.0, at.y - 21.0 - a * 12.0, 12.0, c);
        draw_line(
            at.x - 9.0,
            at.y - 22.0 - a * 12.0,
            at.x - 3.0,
            at.y - 28.0 - a * 12.0,
            2.0,
            c,
        );
        draw_text(
            format!("{})", s.damage[n]),
            at.x + 1.0,
            at.y - 21.0 - a * 12.0,
            12.0,
            c,
        );
    }
    if matches!(s.case, Case::Boss | Case::Elite) {
        let at = point(TARGETS[1], s.direction, rect);
        draw_text(
            if s.case == Case::Boss {
                "Boss"
            } else {
                "Élite"
            },
            at.x - 16.0,
            at.y + 42.0,
            13.0,
            DIM,
        );
    }
}
fn scene(art: &Art, clip: &Clip, s: &State, rect: Rect, effects: bool) {
    terrain(&s.map(), s.direction, rect, false);
    if s.launch.is_none() && s.allowed() {
        preview(&s.cells(), s.direction, rect, s.clock);
        for (n, at) in s.aims().into_iter().enumerate() {
            let p = point(at, s.direction, rect);
            draw_text_bold((n + 1).to_string(), p.x - 5.0, p.y + 24.0, 16.0, TEXT);
        }
    }
    terrain(&s.map(), s.direction, rect, true);
    clip.apply_for(s.map_index(), s.direction, rect);
    if effects {
        art.effects(s, rect);
    }
    art.bodies(s, rect, effects);
    if effects {
        art.wounds(s, rect);
    }
    gl_use_default_material();
    art.statuses(s, rect);
    feedback(s, rect);
    draw_rectangle(
        rect.x + 5.0,
        rect.y + SIZE - 43.0,
        566.0,
        38.0,
        alpha(INK, 0.95),
    );
    draw_text(
        format!(
            "Tour {} · recharge {} · énergie {}",
            s.turn, s.cooldown, s.energy
        ),
        rect.x + 12.0,
        rect.y + SIZE - 27.0,
        14.0,
        TEXT,
    );
    draw_text(
        if !s.allowed() {
            "Lancement refusé · aucune ressource consommée."
        } else if s.skill == 0 {
            "Une cible par tir ; les explosions peuvent se recouvrir."
        } else {
            "Assommement préparé ; durée mesurée en tours."
        },
        rect.x + 12.0,
        rect.y + SIZE - 10.0,
        13.0,
        DIM,
    );
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
        Case::Empty,
        Case::One,
        Case::NoBox,
        Case::Melee,
        Case::Silence,
        Case::Elite,
        Case::Boss,
    ];
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Armes lourdes · ANIM-S25", 20.0, 33.0, 24.0, TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 82.0 + i as f32 * 42.0, 350.0, 34.0),
                title,
                true,
            ) {
                s = State::new(i, s.case, s.direction);
            }
        }
        for (i, title) in DIRS.iter().enumerate() {
            if button(
                Rect::new(
                    20.0 + (i % 2) as f32 * 174.0,
                    181.0 + (i / 2) as f32 * 36.0,
                    166.0,
                    30.0,
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
                    20.0 + (i % 2) as f32 * 185.0,
                    270.0 + (i / 2) as f32 * 34.0,
                    178.0,
                    29.0,
                ),
                case.title(),
                true,
            ) {
                s = State::new(s.skill, *case, s.direction);
            }
        }
        if button(
            Rect::new(20.0, 480.0, 350.0, 34.0),
            "Lancer",
            s.launch.is_none() && s.allowed(),
        ) {
            s.begin();
        }
        if button(
            Rect::new(20.0, 522.0, 350.0, 34.0),
            "Action suivante",
            s.launch.is_some() && s.age().unwrap_or(0.0) >= 0.72 && s.step < 3,
        ) {
            s.advance();
        }
        if button(Rect::new(20.0, 564.0, 160.0, 34.0), "Recommencer", true)
            || is_key_pressed(KeyCode::R)
        {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(190.0, 564.0, 160.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
            s.resolve();
        }
        scene(&art, &clip, &s, Rect::new(410.0, 88.0, SIZE, SIZE), true);
        draw_text(
            "Lance-roquettes préparé · 64 px · aucune partie modifiée.",
            410.0,
            704.0,
            14.0,
            DIM,
        );
        draw_text(
            "Monstres 32 px temporaires ; adaptation après les animations.",
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
    let clip = Clip::load_maps(&MAPS, PLAYER);
    let target = render_target(640, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 640.0, 720.0))
    };
    let controls = [
        14, 17, 20, 22, 24, 26, 28, 30, 32, 36, 44, 48, 60, 64, 68, 84, 88, 92, 112, 116, 126,
    ];
    let mut entries = Vec::new();
    for skill in 0..2 {
        let mut cases = vec![
            (Case::Open, 2, "HAUT"),
            (Case::Open, 0, "DROITE"),
            (Case::Open, 1, "GAUCHE"),
            (Case::Open, 3, "BAS"),
            (Case::Quiet, 2, "ATTENTE"),
            (Case::Wall, 2, "CLOISON"),
            (Case::Empty, 2, "VIDE"),
            (Case::NoBox, 2, "SANS-BOITE"),
            (Case::Melee, 2, "MELEE"),
            (Case::Silence, 2, "SILENCE"),
        ];
        if skill == 0 {
            cases.push((Case::One, 2, "UN-ENNEMI"));
        } else {
            cases.extend([(Case::Elite, 2, "ELITE"), (Case::Boss, 2, "BOSS")]);
        }
        for (case, direction, label) in cases {
            let folder = format!("{}-{label}", IDS[skill]);
            let dir = output.join(&folder);
            std::fs::create_dir(&dir).map_err(|e| e.to_string())?;
            let mut s = State::new(skill, case, direction);
            let permitted = s.allowed();
            let aims = s.aims();
            let mut timeline = Vec::new();
            for frame in 0..FRAMES {
                s.clock = frame as f32 / 24.0;
                if frame == 17 {
                    s.begin();
                }
                s.resolve();
                if case != Case::Quiet && ACTION_FRAMES.contains(&frame) {
                    s.advance();
                }
                timeline.push(serde_json::json!({"frame":frame,"launched":s.launch.is_some(),"turn":s.turn,"step":s.step,"energy":s.energy,"cooldown":s.cooldown,"damage":s.damage,"stun":s.stun,"stun_checked":s.stun_checked,"resolved":s.resolved,"preview_visible":s.launch.is_none()&&s.allowed(),"age":s.age()}));
                for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                    set_camera(&camera);
                    clear_background(INK);
                    draw_text_bold(
                        format!("{} · {}", 59 + skill, TITLES[skill]),
                        32.0,
                        35.0,
                        24.0,
                        TEXT,
                    );
                    draw_text(
                        format!("{} · {} · étape {}", s.variant(), label, s.step),
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
                    draw_text(
                        if skill == 0 {
                            "Rang 11 · jusqu’à 3 tirs · énergie 18 · recharge 5 tours"
                        } else {
                            "Rang 11 · assommement 2 tours · énergie 14 · recharge 6 tours"
                        },
                        32.0,
                        684.0,
                        14.0,
                        TEXT,
                    );
                    draw_text(
                        "Dégâts et réussite préparés ; aucun jet de combat réel.",
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
            entries.push(serde_json::json!({"folder":folder,"variant":format!("{}-{label}",s.variant()),"id":IDS[skill],"number":59+skill,"title":TITLES[skill],"case":label,"case_title":case.title(),"direction":direction,"permitted":permitted,"quiet":case==Case::Quiet,"control_frames":controls,"action_frames":if case==Case::Quiet{vec![]}else{ACTION_FRAMES.to_vec()},"aims":aims.into_iter().map(|p|[p.x,p.y]).collect::<Vec<_>>(),"tile":64,"frames":FRAMES,"fps":24,"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n")}));
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
    fn refusals_preserve_all_resources() {
        for case in [
            Case::Wall,
            Case::Empty,
            Case::NoBox,
            Case::Melee,
            Case::Silence,
        ] {
            let mut s = State::new(0, case, 2);
            s.begin();
            s.clock = 5.0;
            s.resolve();
            assert_eq!(
                (s.turn, s.energy, s.cooldown, s.damage),
                (0, 100, 0, [0; 4])
            );
            assert!(s.launch.is_none());
        }
    }
    #[test]
    fn barrage_targets_each_enemy_and_keeps_overlap() {
        let mut s = State::new(0, Case::Open, 2);
        assert_eq!(s.aims().len(), 3);
        s.begin();
        s.clock = 0.9;
        s.resolve();
        assert_eq!(s.damage, [80, 120, 80, 0]);
        assert_eq!(s.energy, 82);
        s.resolve();
        assert_eq!(s.damage, [80, 120, 80, 0]);
    }
    #[test]
    fn partial_barrage_never_targets_an_empty_cell() {
        let mut s = State::new(0, Case::One, 2);
        assert_eq!(s.aims(), vec![TARGETS[1]]);
        s.begin();
        s.clock = 1.0;
        s.resolve();
        assert_eq!(s.damage, [0, 40, 0, 0]);
    }
    #[test]
    fn status_only_expires_on_actions() {
        let mut s = State::new(1, Case::Quiet, 2);
        s.begin();
        s.clock = 1.0;
        s.resolve();
        assert_eq!(s.stun, [2, 2, 2, 0]);
        s.clock = 99.0;
        s.resolve();
        assert_eq!(s.stun, [2, 2, 2, 0]);
        s.advance();
        assert_eq!(s.stun, [1, 1, 1, 0]);
        s.advance();
        assert_eq!(s.stun, [0; 4]);
    }
    #[test]
    fn elite_and_boss_preserve_damage_without_control() {
        for case in [Case::Elite, Case::Boss] {
            let mut s = State::new(1, case, 2);
            s.begin();
            s.clock = 1.0;
            s.resolve();
            assert_eq!(s.damage, [36, 36, 36, 0]);
            assert_eq!(s.stun, [2, 0, 2, 0]);
            assert_eq!(s.stun_checked, [true, true, true, false]);
        }
    }
}
