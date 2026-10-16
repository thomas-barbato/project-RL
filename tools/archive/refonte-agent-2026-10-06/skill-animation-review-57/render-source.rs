//! ANIM-S27: amplified explosion and impact healing, isolated review fixtures.
use super::super::melee::actor;
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;
const SIZE: f32 = 576.0;
const PLAYER: GridPos = GridPos::new(4, 6);
const CENTER: GridPos = GridPos::new(4, 3);
const TARGETS: [GridPos; 4] = [
    GridPos::new(3, 3),
    CENTER,
    GridPos::new(5, 3),
    GridPos::new(7, 3),
];
const IDS: [&str; 2] = ["HL05", "HL06"];
const TITLES: [&str; 2] = ["Déflagration amplifiée", "Drain d’impact"];
const DIRS: [&str; 4] = ["DROITE", "GAUCHE", "HAUT", "BAS"];
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.....#.#\n#.....#.#\n#.....#.#\n#.....#.#\n#.....#.#\n#.....#.#\n#.....#.#\n#########",
];
const FRAMES: usize = 128;
const ACTION_FRAMES: [usize; 3] = [64, 88, 112];
const FLIGHT: f32 = 0.20;
const BLAST: f32 = 0.42;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    Open,
    Quiet,
    Wall,
    Empty,
    NoBox,
    Light,
    NonExplosive,
    Silence,
    Poor,
    Full,
    NearlyFull,
    LowTarget,
    Protected,
    Delayed,
}
impl Case {
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Zone ouverte",
            Self::Quiet => "Attente sans action",
            Self::Wall => "Explosion coupée par la cloison",
            Self::Empty => "Aucune cible",
            Self::NoBox => "Boîte absente",
            Self::Light => "Arme légère",
            Self::NonExplosive => "Arme lourde sans explosion",
            Self::Silence => "Silence",
            Self::Poor => "Énergie insuffisante",
            Self::Full => "PV déjà au maximum",
            Self::NearlyFull => "Seulement 3 PV manquants",
            Self::LowTarget => "Cible principale : 10 PV",
            Self::Protected => "Cible protégée : 20 PV retirés",
            Self::Delayed => "Explosion au tour suivant (exemple)",
        }
    }
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    launch: Option<f32>,
    impact: Option<f32>,
    step: u16,
    turn: u16,
    energy: u16,
    hp: u16,
    cooldown: u16,
    damage: [u16; 4],
    reaction: [Option<f32>; 4],
    resolved: bool,
    healed: u16,
    heal_at: Option<f32>,
    pending: bool,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.0,
            launch: None,
            impact: None,
            step: 0,
            turn: 0,
            energy: if case == Case::Poor { 8 } else { 100 },
            hp: match case {
                Case::Full => 100,
                Case::NearlyFull => 97,
                _ => 50,
            },
            cooldown: 0,
            damage: [0; 4],
            reaction: [None; 4],
            resolved: false,
            healed: 0,
            heal_at: None,
            pending: false,
        }
    }
    fn map_index(&self) -> usize {
        usize::from(self.case == Case::Wall)
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn allowed(&self) -> bool {
        !matches!(self.case, Case::Empty | Case::NoBox)
            && !(self.skill == 1 && matches!(self.case, Case::Light | Case::Silence | Case::Poor))
    }
    fn explosive(&self) -> bool {
        !matches!(self.case, Case::Light | Case::NonExplosive)
    }
    fn radius(&self) -> i32 {
        if !self.explosive() {
            0
        } else if self.skill == 0 {
            4
        } else {
            1
        }
    }
    fn visible(&self, _n: usize) -> bool {
        self.case != Case::Empty
    }
    fn cells(&self) -> Vec<GridPos> {
        let map = self.map();
        let r = self.radius();
        // Prepared maps have either one room or two rooms separated by a full wall.
        (-r..=r)
            .flat_map(|y| (-r..=r).map(move |x| (x, y)))
            .filter(|&(x, y)| x * x + y * y <= r * r)
            .map(|(x, y)| GridPos::new(CENTER.x + x, CENTER.y + y))
            .filter(|p| map.is_walkable(*p) && (self.case != Case::Wall || p.x < 6))
            .collect()
    }
    fn begin(&mut self) {
        if self.launch.is_some() || !self.allowed() {
            return;
        }
        self.launch = Some(self.clock);
        self.turn = 1;
        if self.skill == 1 {
            self.energy -= 16;
            self.cooldown = 10;
        }
    }
    fn age(&self) -> Option<f32> {
        self.launch.map(|t| self.clock - t)
    }
    fn resolve(&mut self) {
        if self.launch.is_none() || self.resolved || self.age().unwrap_or(0.0) < FLIGHT {
            return;
        }
        if self.case == Case::Delayed && self.impact.is_none() {
            self.pending = true;
            return;
        }
        let hit = *self.impact.get_or_insert(self.launch.unwrap() + FLIGHT);
        self.resolved = true;
        self.pending = false;
        let cells = self.cells();
        for (n, at) in TARGETS.into_iter().enumerate() {
            if self.visible(n) && cells.contains(&at) {
                self.damage[n] = if self.skill == 0 {
                    20
                } else if n == 1 && self.case == Case::LowTarget {
                    10
                } else if n == 1 && self.case == Case::Protected {
                    20
                } else {
                    40
                };
                self.reaction[n] = Some(hit);
            }
        }
        if self.skill == 1 {
            // Only the primary enemy's actually removed HP funds the return.
            // These exact integer examples avoid choosing a general rounding policy.
            self.healed = (self.damage[1] * 60 / 100).min(15).min(100 - self.hp);
            self.hp += self.healed;
            if self.healed > 0 {
                self.heal_at = Some(hit);
            }
        }
    }
    fn advance(&mut self) {
        if self.launch.is_none() || self.age().unwrap_or(0.0) < 0.85 {
            return;
        }
        self.step += 1;
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        if self.pending {
            self.impact = Some(self.clock);
            self.resolve();
        }
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
fn offset(s: &State, n: usize, r: Rect, effects: bool) -> Vec2 {
    if !effects {
        return Vec2::ZERO;
    }
    s.reaction[n].map_or(Vec2::ZERO, |t| {
        crate::combat_visuals::hit_reaction_offset(
            point(TARGETS[n], s.direction, r),
            point(PLAYER, s.direction, r),
            f64::from(s.clock - t),
            64.0,
        )
    })
}
struct Art {
    shared: super::heavy_opening::Art,
    icons: [Texture2D; 2],
}
impl Art {
    fn load() -> Self {
        let load = |bytes: &[u8]| {
            let t = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            shared: super::heavy_opening::Art::load(),
            icons: [
                load(include_bytes!(
                    "../../../assets/skills/agent/HL05-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/HL06-compact.png"
                )),
            ],
        }
    }

    fn projectile(&self, s: &State, r: Rect) {
        let Some(a) = s.age().filter(|a| (0.0..FLIGHT).contains(a)) else {
            return;
        };
        let origin = point(PLAYER, s.direction, r);
        let at = point(CENTER, s.direction, r);
        let d = (at - origin).normalize();
        let p = (origin + d * 30.0).lerp(at, a / FLIGHT);
        if s.explosive() {
            let w = 50.0;
            let h = w * self.shared.rocket.height() / self.shared.rocket.width();
            draw_texture_ex(
                &self.shared.rocket,
                p.x - w / 2.0,
                p.y - h / 2.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(w, h)),
                    rotation: d.y.atan2(d.x),
                    pivot: Some(p),
                    ..Default::default()
                },
            );
        } else {
            draw_line(p.x - d.x * 18.0, p.y - d.y * 18.0, p.x, p.y, 5.5, INK);
            draw_line(
                p.x - d.x * 16.0,
                p.y - d.y * 16.0,
                p.x,
                p.y,
                3.3,
                Color::new(0.88, 0.91, 0.92, 1.0),
            );
        }
    }
    fn explosion(&self, s: &State, r: Rect) {
        let Some(a) = s
            .impact
            .map(|t| s.clock - t)
            .filter(|a| (0.0..BLAST).contains(a))
        else {
            return;
        };
        if !s.explosive() {
            return;
        }
        let at = point(CENTER, s.direction, r);
        let f = ((a / BLAST * 6.0) as usize).min(5);
        let size = if s.skill == 0 { 478.0 } else { 134.0 };
        draw_texture_ex(
            &self.shared.blast,
            at.x - size * 132.0 / 256.0,
            at.y - size * 146.0 / 256.0,
            alpha(WHITE, 0.9),
            DrawTextureParams {
                source: Some(Rect::new(f as f32 * 256.0, 0.0, 256.0, 256.0)),
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    }
    fn bodies(&self, s: &State, r: Rect, effects: bool) {
        for (n, at) in TARGETS.into_iter().enumerate() {
            if s.visible(n) {
                actor(
                    point(at, s.direction, r) + offset(s, n, r, effects),
                    Visual::Bestiary {
                        number: 12,
                        corpse: false,
                    },
                );
            }
        }
        actor(
            point(PLAYER, s.direction, r),
            game_art::wear(
                game_art::player(
                    Some("core:bastion"),
                    PlayerWeaponPose::Ranged,
                    Some(match s.case {
                        Case::Light => "core:modele_cogneur_p01",
                        Case::NonExplosive => "core:mitrailleuse_lourde",
                        _ => "core:lance_roquettes",
                    }),
                ),
                [("core:body_armor".into(), "core:tenue_assaut".into())],
            ),
        );
    }
    fn blood(&self, at: Vec2, age: f32) {
        if !(0.0..0.30).contains(&age) {
            return;
        }
        let f = ((age / 0.30 * 6.0) as usize).min(5);
        draw_texture_ex(
            &self.shared.blood,
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

    fn wounds(&self, s: &State, r: Rect) {
        for n in 0..4 {
            if let Some(t) = s.reaction[n] {
                self.blood(
                    point(TARGETS[n], s.direction, r) + offset(s, n, r, true),
                    s.clock - t,
                );
            }
        }
    }
    fn healing(&self, s: &State, r: Rect) {
        let Some(a) = s
            .heal_at
            .map(|t| s.clock - t)
            .filter(|a| (0.0..0.55).contains(a))
        else {
            return;
        };
        let p = point(PLAYER, s.direction, r);
        let q = a / 0.55;
        let colour = Color::new(0.34, 0.94, 0.27, (1.0 - q) * 0.85);
        // Brief outlined beneficial pulse, attached to the player; no timed buff.
        for i in 0..3 {
            let spread = 18.0 + q * 23.0 + i as f32 * 5.0;
            draw_ellipse_lines(
                p.x,
                p.y + 13.0,
                spread,
                spread * 0.36,
                0.0,
                5.0,
                alpha(INK, (1.0 - q) * 0.8),
            );
            draw_ellipse_lines(p.x, p.y + 13.0, spread, spread * 0.36, 0.0, 2.7, colour);
        }
        for side in [-1.0, 1.0] {
            let x = p.x + side * (21.0 + q * 6.0);
            let y = p.y + 15.0 - q * 48.0;
            draw_line(x, y + 14.0, x, y, 5.0, alpha(INK, 1.0 - q));
            draw_line(x, y + 12.0, x, y, 2.7, colour);
        }
    }
    fn hud(&self, s: &State, r: Rect) {
        draw_rectangle(r.x + 12.0, r.y + 12.0, 252.0, 65.0, alpha(INK, 0.96));
        draw_texture_ex(
            &self.icons[s.skill],
            r.x + 20.0,
            r.y + 26.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(32.0, 32.0)),
                ..Default::default()
            },
        );
        draw_text(
            if s.skill == 0 {
                "Passif permanent"
            } else {
                "Soin sur la cible principale"
            },
            r.x + 60.0,
            r.y + 33.0,
            14.0,
            DIM,
        );
        draw_text(
            if s.skill == 0 {
                if s.explosive() {
                    "Rayon : 4 cases (+3)"
                } else {
                    "Sans explosion : aucun bonus"
                }
            } else {
                "Retour : 60 % · limite : 15 PV"
            },
            r.x + 60.0,
            r.y + 57.0,
            13.0,
            TEXT,
        );
    }
}
fn floating(at: Vec2, damage: u16, bonus: u16, age: f32) {
    if !(0.0..0.58).contains(&age) {
        return;
    }
    let c = alpha(TEXT, 1.0 - age / 0.58);
    let y = at.y - 33.0 - age * 12.0;
    draw_text_bold(
        if bonus > 0 {
            format!("{damage} (+{bonus})")
        } else {
            damage.to_string()
        },
        at.x - if bonus > 0 { 38.0 } else { 12.0 },
        y,
        19.0,
        c,
    );
    draw_text("(", at.x - 18.0, y + 12.0, 12.0, c);
    draw_line(at.x - 9.0, y + 11.0, at.x - 3.0, y + 5.0, 2.0, c);
    draw_text(format!("{damage})"), at.x + 1.0, y + 12.0, 12.0, c);
}

fn feedback(s: &State, r: Rect) {
    if s.skill == 1 {
        if let Some(a) = s.age().filter(|a| *a < 0.38) {
            let p = point(PLAYER, s.direction, r);
            draw_text_bold(
                TITLES[1],
                p.x - 50.0,
                p.y - 47.0 - a * 8.0,
                15.0,
                alpha(TEXT, 1.0 - a / 0.38),
            );
        }
    }
    for n in 0..4 {
        if let Some(t) = s.reaction[n] {
            floating(
                point(TARGETS[n], s.direction, r)
                    + if n == 0 || n == 2 {
                        vec2(0.0, -23.0)
                    } else {
                        Vec2::ZERO
                    },
                s.damage[n],
                0,
                s.clock - t,
            );
        }
    }
    if let Some(a) = s
        .heal_at
        .map(|t| s.clock - t)
        .filter(|a| (0.0..0.70).contains(a))
    {
        let p = point(PLAYER, s.direction, r);
        draw_text_bold(
            format!("+{} PV", s.healed),
            p.x - 25.0,
            p.y - 45.0 - a * 18.0,
            20.0,
            alpha(Color::new(0.5, 1.0, 0.4, 1.0), 1.0 - a / 0.7),
        );
    }
}
fn scene(art: &Art, clip: &Clip, s: &State, r: Rect, effects: bool) {
    terrain(&s.map(), s.direction, r, false);
    if s.launch.is_none() && s.allowed() {
        preview(&s.cells(), s.direction, r, s.clock);
    }
    terrain(&s.map(), s.direction, r, true);
    clip.apply_for(s.map_index(), s.direction, r);
    if effects {
        art.projectile(s, r);
        art.explosion(s, r);
    }
    gl_use_default_material();
    // Bodies use a walkable-cell mask too, so a large hit reaction never draws over a wall.
    clip.apply_for(s.map_index(), s.direction, r);
    art.bodies(s, r, effects);
    if effects {
        art.wounds(s, r);
    }
    gl_use_default_material();
    if effects {
        art.healing(s, r);
    }
    art.hud(s, r);
    feedback(s, r);
    draw_rectangle(r.x + 5.0, r.y + SIZE - 43.0, 566.0, 38.0, alpha(INK, 0.95));
    draw_text(
        format!(
            "Tour {} · PV {}/100 · énergie {} · recharge {}",
            s.turn, s.hp, s.energy, s.cooldown
        ),
        r.x + 12.0,
        r.y + SIZE - 27.0,
        14.0,
        TEXT,
    );
    draw_text(
        if !s.allowed() {
            "Indisponible : aucune consommation."
        } else if s.pending {
            "Explosion en attente du prochain tour."
        } else if s.skill == 0 {
            "Explosion seule agrandie ; dégâts d’exemple inchangés."
        } else {
            "Soin à l’impact ; aucun statut temporaire ajouté."
        },
        r.x + 12.0,
        r.y + SIZE - 10.0,
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
    let clip = Clip::load_maps(&MAPS, CENTER);
    let mut s = State::new(0, Case::Open, 2);
    let mut paused = false;
    let cases = [
        Case::Open,
        Case::Quiet,
        Case::Wall,
        Case::Empty,
        Case::NoBox,
        Case::Light,
        Case::NonExplosive,
        Case::Silence,
        Case::Poor,
        Case::Full,
        Case::NearlyFull,
        Case::LowTarget,
        Case::Protected,
        Case::Delayed,
    ];
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Armes lourdes · ANIM-S27", 20.0, 33.0, 24.0, TEXT);
        for (i, t) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 80.0 + i as f32 * 40.0, 360.0, 34.0),
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
                    270.0 + (i / 2) as f32 * 32.0,
                    178.0,
                    27.0,
                ),
                c.title(),
                true,
            ) {
                s = State::new(s.skill, *c, s.direction);
            }
        }
        if button(
            Rect::new(20.0, 520.0, 350.0, 34.0),
            if s.skill == 0 {
                "Attaque ordinaire"
            } else {
                "Lancer Drain d’impact"
            },
            s.launch.is_none() && s.allowed(),
        ) || is_key_pressed(KeyCode::Enter)
        {
            s.begin();
        }
        if button(
            Rect::new(20.0, 562.0, 350.0, 34.0),
            "Action suivante",
            s.launch.is_some() && s.age().unwrap_or(0.0) >= 0.85 && s.step < 3,
        ) || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(20.0, 604.0, 160.0, 34.0), "Recommencer", true)
            || is_key_pressed(KeyCode::R)
        {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(190.0, 604.0, 160.0, 34.0),
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
            "Événements et nombres préparés · aucune partie modifiée.",
            410.0,
            704.0,
            13.0,
            DIM,
        );
        draw_text(
            "Monstres32 temporaires ; adaptation après les animations.",
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
    let clip = Clip::load_maps(&MAPS, CENTER);
    let target = render_target(640, 720);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, 640.0, 720.0))
    };
    let controls = [
        14, 17, 20, 22, 24, 26, 28, 32, 36, 44, 60, 64, 68, 72, 76, 84, 88, 100, 112, 124,
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
            (Case::Light, 2, "LEGERE"),
            (Case::NonExplosive, 2, "SANS-EXPLOSION"),
        ];
        if skill == 1 {
            cases.extend([
                (Case::Silence, 2, "SILENCE"),
                (Case::Poor, 2, "ENERGIE"),
                (Case::Full, 2, "PV-PLEINS"),
                (Case::NearlyFull, 2, "PV-MANQUANTS"),
                (Case::LowTarget, 2, "CIBLE-10-PV"),
                (Case::Protected, 2, "PROTECTIONS"),
                (Case::Delayed, 2, "RETARDEMENT"),
            ]);
        }
        for (case, direction, label) in cases {
            let mut s = State::new(skill, case, direction);
            let permitted = s.allowed();
            let folder = format!("{}-{label}", IDS[skill]);
            let dir = output.join(&folder);
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
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
                timeline.push(serde_json::json!({"frame":frame,"begun":s.launch.is_some(),"turn":s.turn,"step":s.step,"energy":s.energy,"hp":s.hp,"cooldown":s.cooldown,"damage":s.damage,"healed":s.healed,"pending":s.pending,"resolved":s.resolved,"preview_visible":s.launch.is_none()&&s.allowed(),"explosive":s.explosive(),"radius":s.radius(),"impact_at":s.impact,"heal_at":s.heal_at}));
                for control in 0..if controls.contains(&frame) { 2 } else { 1 } {
                    set_camera(&camera);
                    clear_background(INK);
                    draw_text_bold(
                        format!("{} · {}", 63 + skill, TITLES[skill]),
                        32.0,
                        35.0,
                        24.0,
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
                    );
                    draw_text(
                        if skill == 0 {
                            "Rang 11 · rayon +3 cases · aucun bonus de dégâts"
                        } else {
                            "Rang 11 · retour 60 % · plafond 15 % des PV max · recharge 10"
                        },
                        32.0,
                        684.0,
                        13.0,
                        TEXT,
                    );
                    draw_text(
                        "Impact et soin préparés ; équilibrage réel à raccorder.",
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
            entries.push(serde_json::json!({"folder":folder,"variant":format!("{}-{label}",s.variant()),"id":IDS[skill],"number":63+skill,"title":TITLES[skill],"case":label,"case_title":case.title(),"direction":direction,"permitted":permitted,"quiet":case==Case::Quiet,"control_frames":controls,"action_frames":if case==Case::Quiet{vec![]}else{ACTION_FRAMES.to_vec()},"tile":64,"frames":FRAMES,"fps":24,"map":rows.into_iter().map(|r|r.into_iter().collect::<String>()).collect::<Vec<_>>().join("\n")}));
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
