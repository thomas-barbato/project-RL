//! Historical ANIM-S46 fixture, superseded by D66-TAUNT01 in the live game.
//! Its old protection values stay local; the current catalogue contains taunt instead.
//! Incoming20, contact12 and base20 damage are visual fixtures, not actor statistics.
use super::super::melee::{actor, blood_spray, cut};
use super::zones::{position, preview, terrain};
use super::*;
use project_rl::world::{Map, has_line_of_sight};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const IDS: [&str; 2] = ["D66", "O03"];
const TITLES: [&str; 2] = ["Invocation du Gardien", "Algorithmes de combat améliorés"];
const PLAYER: GridPos = GridPos::new(4, 6);
const SPAWN: GridPos = GridPos::new(4, 5);
const FRAMES: usize = 144;
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#..#....#\n#..#....#\n#..#....#\n#..#....#\n#..#....#\n#.......#\n#.......#\n#########",
];
#[derive(Clone, Copy, PartialEq, Eq)]
enum Case {
    Combat,
    Tracker,
    Gunner,
    Far,
    Wall,
    Machine,
    Support,
    Quiet,
    Energy,
    Existing,
}
impl Case {
    fn label(self) -> &'static str {
        match self {
            Self::Combat => "COMBAT",
            Self::Tracker => "TRAQUEUR",
            Self::Gunner => "ARTILLEUR",
            Self::Far => "HORS_PORTEE",
            Self::Wall => "CLOISON",
            Self::Machine => "MACHINE",
            Self::Support => "SOUTIEN",
            Self::Quiet => "ATTENTE",
            Self::Energy => "ENERGIE",
            Self::Existing => "EXISTANT",
        }
    }
    fn title(self) -> &'static str {
        match self {
            Self::Combat => "Gardien : protection et contact",
            Self::Tracker => "Avec le Traqueur",
            Self::Gunner => "Avec l’Artilleur",
            Self::Far => "Allié hors de portée",
            Self::Wall => "Protection et cloison",
            Self::Machine => "Cible mécanique",
            Self::Support => "Mule et Éclaireur sans attaque",
            Self::Quiet => "Attente sans nouveau tour",
            Self::Energy => "Énergie insuffisante",
            Self::Existing => "Gardien déjà déployé",
        }
    }
}
struct Attack {
    time: f32,
    resolved: bool,
}
struct Hit {
    time: f32,
    protected: bool,
    resolved: bool,
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    started: Option<f32>,
    turn: u16,
    energy: u16,
    cooldown: u16,
    deployed: bool,
    deployments: u16,
    attacks: Vec<Attack>,
    hits: Vec<Hit>,
    enemy_hp: u16,
    ally_hp: u16,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.,
            started: None,
            turn: 0,
            energy: if case == Case::Energy { 13 } else { 100 },
            cooldown: 0,
            deployed: skill == 1 || case == Case::Existing,
            deployments: 0,
            attacks: Vec::new(),
            hits: Vec::new(),
            enemy_hp: 160,
            ally_hp: 120,
        }
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[usize::from(self.case == Case::Wall)]).unwrap()
    }
    fn model(&self) -> u8 {
        if self.skill == 0 {
            66
        } else {
            match self.case {
                Case::Tracker => 58,
                Case::Gunner => 59,
                Case::Support => 57,
                _ => 66,
            }
        }
    }
    fn target(&self) -> GridPos {
        if self.case == Case::Wall {
            GridPos::new(1, 4)
        } else if self.skill == 1 && self.case == Case::Gunner {
            GridPos::new(4, 2)
        } else {
            GridPos::new(4, 4)
        }
    }
    fn ally(&self) -> GridPos {
        match self.case {
            Case::Far => GridPos::new(4, 2),
            Case::Wall => GridPos::new(2, 5),
            Case::Support | Case::Tracker | Case::Gunner => GridPos::new(2, 5),
            _ => PLAYER,
        }
    }
    fn ally_model(&self) -> Option<u8> {
        match self.case {
            Case::Far | Case::Wall | Case::Support => Some(57),
            Case::Tracker => Some(58),
            Case::Gunner => Some(59),
            _ => None,
        }
    }
    fn protection(&self, at: GridPos) -> bool {
        // Only axis-aligned/clearly outside fixtures are used to avoid choosing
        // an unreviewed diagonal distance convention for the campaign.
        self.skill == 0
            && self.deployed
            && (at.x - SPAWN.x).abs().max((at.y - SPAWN.y).abs()) <= i32::from(2u16)
            && has_line_of_sight(&self.map(), SPAWN, at, true)
    }
    fn reason(&self) -> Option<&'static str> {
        if self.skill == 1 {
            None
        } else if self.deployed {
            Some("Ce modèle est déjà déployé.")
        } else if self.energy < parameter_for("D66", "energy", 1) {
            Some("Énergie insuffisante.")
        } else {
            None
        }
    }
    fn launch(&mut self) {
        if self.started.is_some() || self.reason().is_some() {
            return;
        }
        if self.skill == 0 {
            self.energy -= parameter_for("D66", "energy", 1);
            self.cooldown = parameter_for("D66", "cooldown_turns", 1);
            self.deployed = true;
            self.deployments += 1;
        }
        self.started = Some(self.clock);
        self.phase(self.clock + if self.skill == 0 { 0.34 } else { 0. });
    }
    fn phase(&mut self, time: f32) {
        if self.model() != 57
            && self.enemy_hp > 0
            && has_line_of_sight(&self.map(), SPAWN, self.target(), true)
        {
            self.attacks.push(Attack {
                time,
                resolved: false,
            });
        }
        if self.skill == 0 && self.turn > 0 {
            self.hits.push(Hit {
                time: time + 0.24,
                protected: self.protection(self.ally()),
                resolved: false,
            });
        }
    }
    fn damage(&self) -> u16 {
        if self.skill == 0 {
            12
        } else {
            20 * (100 + parameter_for("O03", "drone_damage_bonus_percent", 1)) / 100
        }
    }
    fn received(protected: bool) -> u16 {
        if protected {
            20 * (100 - 5u16) / 100
        } else {
            20
        }
    }
    fn settle(&mut self) {
        let damage = self.damage();
        for a in &mut self.attacks {
            if !a.resolved && self.clock >= a.time + 0.12 {
                self.enemy_hp = self.enemy_hp.saturating_sub(damage);
                a.resolved = true;
            }
        }
        for h in &mut self.hits {
            if !h.resolved && self.clock >= h.time {
                self.ally_hp = self.ally_hp.saturating_sub(Self::received(h.protected));
                h.resolved = true;
            }
        }
    }
    fn busy(&self) -> bool {
        self.started.is_some_and(|t| self.clock < t + 0.34)
            || self
                .attacks
                .last()
                .is_some_and(|a| self.clock < a.time + 0.4)
            || self.hits.last().is_some_and(|h| self.clock < h.time + 0.3)
    }
    fn advance(&mut self) {
        self.settle();
        if self.started.is_none() || self.busy() {
            return;
        }
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        self.phase(self.clock);
    }
    fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({"turn":self.turn,"energy":self.energy,"cooldown":self.cooldown,"launched":self.started.is_some(),"deployed":self.deployed,"deployments":self.deployments,"model":self.model(),"enemy_hp":self.enemy_hp,"ally_hp":self.ally_hp,"damage_fixture":self.damage(),"incoming_fixture":20,"protection_percent":if self.skill==0 {5u16}else{0},"ally_protected":self.protection(self.ally()),"player_protected":self.protection(PLAYER),"self_protected":self.protection(SPAWN),"passive_bonus_percent":if self.skill==1 {parameter_for("O03","drone_damage_bonus_percent",1)}else{0},"attacks_scheduled":self.attacks.len(),"impacts":self.attacks.iter().filter(|a|a.resolved).count(),"received_hits":self.hits.iter().filter(|h|h.resolved).count(),"blood_events":if self.case==Case::Machine {0}else{self.attacks.iter().filter(|a|a.resolved).count()},"machine_hits":if self.case==Case::Machine {self.attacks.iter().filter(|a|a.resolved).count()}else{0},"ally_blood_events":if self.ally_model().is_none(){self.hits.iter().filter(|h|h.resolved).count()}else{0},"preview":self.skill==0&&self.started.is_none()&&self.reason().is_none(),"refusal":if self.started.is_none(){self.reason()}else{None},"line_of_sight":has_line_of_sight(&self.map(),SPAWN,self.target(),true)})
    }
}
fn point(at: GridPos, direction: usize, rect: Rect) -> Vec2 {
    let at = position(at, direction);
    vec2(
        rect.x + (at.x as f32 + 0.5) * 64.,
        rect.y + (at.y as f32 + 0.5) * 64.,
    )
}
fn player_visual() -> Visual {
    game_art::wear(
        game_art::player(Some("core:technomancien"), PlayerWeaponPose::Unarmed, None),
        [("core:body_armor".into(), "core:tenue_technique".into())],
    )
}
fn arrival(p: Vec2, age: f32) {
    if !(0.0..0.34).contains(&age) {
        return;
    }
    let t = age / 0.34;
    let opacity = (1. - t).powf(0.6);
    let light = Color::new(0.82, 0.96, 1., opacity);
    let halo = Color::new(0.27, 0.67, 0.81, opacity * 0.24);
    for offset in [-12., 0., 12.] {
        let y = p.y + 20. - (1. - t) * offset;
        draw_ellipse(p.x, y, 14. + t * 21., 5. + t * 5., 0., halo);
        draw_ellipse_lines(p.x, y, 14. + t * 21., 5. + t * 5., 0., 2.4, light);
    }
    for n in 0..8 {
        let a = n as f32 * std::f32::consts::TAU / 8. + t * 0.5;
        let d = vec2(a.cos(), a.sin() * 0.65);
        let from = p + d * (16. + t * 20.) - vec2(0., (1. - t) * 18.);
        let to = from + d * (4. + (1. - t) * 9.);
        draw_line(from.x, from.y, to.x, to.y, 6., halo);
        draw_line(from.x, from.y, to.x, to.y, 2.5, light);
    }
}
fn sparks(p: Vec2, age: f32) {
    if !(0.0..0.30).contains(&age) {
        return;
    }
    let fade = 1. - age / 0.30;
    for n in 0..7 {
        let angle = n as f32 * 2.399;
        let d = vec2(angle.cos(), angle.sin());
        let tip = p + d * (7. + age * 72.);
        draw_line(
            tip.x - d.x * 6.,
            tip.y - d.y * 6.,
            tip.x,
            tip.y,
            4. * fade,
            alpha(INK, fade),
        );
        draw_line(
            tip.x - d.x * 5.,
            tip.y - d.y * 5.,
            tip.x,
            tip.y,
            2. * fade,
            alpha(GOLD, fade),
        );
    }
}

struct Art {
    guardian: Texture2D,
    artilleur: Texture2D,
    traqueur: Texture2D,
}
impl Art {
    fn load() -> Self {
        let decode = |b: &[u8]| {
            let t = Texture2D::from_file_with_format(b, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Nearest);
            t
        };
        Self {
            guardian: decode(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-93/gardien-body02/D66-BODY02-native64.png"
            )),
            artilleur: decode(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-92/artilleur-body03/D59-BODY03-native64.png"
            )),
            traqueur: decode(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-93/traqueur-palette02/D58-PAL02-native64.png"
            )),
        }
    }
    fn drone(&self, model: u8, p: Vec2, opacity: f32) {
        match model {
            66 => {
                draw_ellipse(p.x, p.y + 29., 24., 4., 0., alpha(BLACK, opacity * 0.3));
                draw_texture(&self.guardian, p.x - 32., p.y - 32., alpha(WHITE, opacity));
            }
            58 => draw_texture(&self.traqueur, p.x - 32., p.y - 32., alpha(WHITE, opacity)),
            59 => {
                draw_ellipse(p.x, p.y + 29., 25., 4., 0., alpha(BLACK, opacity * 0.3));
                draw_texture(&self.artilleur, p.x - 32., p.y - 32., alpha(WHITE, opacity));
            }
            _ => game_art::draw_world(
                Rect::new(p.x - 32., p.y - 32., 64., 64.),
                Visual::Bestiary {
                    number: model,
                    corpse: false,
                },
                opacity,
            ),
        }
    }
}
fn shield(p: Vec2, strength: f32) {
    let bright = Color::new(0.74, 0.85, 0.98, 0.38 + strength * 0.6);
    for sign in [-1., 1.] {
        let pts = [
            p + vec2(sign * 23., -22.),
            p + vec2(sign * 30., -9.),
            p + vec2(sign * 30., 13.),
            p + vec2(sign * 19., 27.),
        ];
        for line in pts.windows(2) {
            draw_line(
                line[0].x,
                line[0].y,
                line[1].x,
                line[1].y,
                6.,
                alpha(INK, 0.8),
            );
            draw_line(line[0].x, line[0].y, line[1].x, line[1].y, 3., bright);
        }
    }
    if strength > 0. {
        draw_ellipse_lines(
            p.x,
            p.y + 26.,
            28. + strength * 6.,
            7.,
            0.,
            2.,
            alpha(bright, strength * 0.65),
        );
    }
}
fn projectile(from: Vec2, to: Vec2, age: f32) {
    if !(0.0..0.12).contains(&age) {
        return;
    }
    let d = (to - from).normalize_or_zero();
    let muzzle = from + vec2(if d.x < -0.1 { -19. } else { 19. }, 3.);
    let p = muzzle.lerp(to, age / 0.12);
    let d = (to - muzzle).normalize_or_zero();
    draw_line(p.x - d.x * 20., p.y - d.y * 20., p.x, p.y, 5., INK);
    draw_line(p.x - d.x * 17., p.y - d.y * 17., p.x, p.y, 3., GOLD);
    draw_circle(p.x, p.y, 2.5, WHITE);
}
fn number(p: Vec2, text: &str, age: f32, color: Color) {
    if !(0.0..0.7).contains(&age) {
        return;
    }
    let w = measure_text(text, None, 22, 1.).width;
    draw_text_bold(
        text,
        p.x - w / 2.,
        p.y - 44. - age * 19.,
        22.,
        alpha(color, ((0.7 - age) * 4.).min(1.)),
    );
}
fn scene(art: &Art, s: &State, rect: Rect) {
    terrain(&s.map(), s.direction, rect, false);
    if s.skill == 0 && s.started.is_none() && s.reason().is_none() {
        preview(&[SPAWN], s.direction, rect, s.clock);
    }
    let enemy = point(s.target(), s.direction, rect);
    let source = point(SPAWN, s.direction, rect);
    let ally = point(s.ally(), s.direction, rect);
    let recoil = |at: Vec2, from: Vec2, age: f32| {
        crate::combat_visuals::hit_reaction_offset(at, from, f64::from(age), 64.)
    };
    let mut victim = enemy;
    for a in &s.attacks {
        victim += recoil(enemy, source, s.clock - a.time - 0.12);
    }
    let mut beneficiary = ally;
    for h in &s.hits {
        beneficiary += recoil(ally, enemy, s.clock - h.time);
    }
    let player = if s.ally_model().is_none() {
        beneficiary
    } else {
        point(PLAYER, s.direction, rect)
    };
    actor(player, player_visual());
    if s.skill == 0 {
        if let Some(model) = s.ally_model() {
            art.drone(model, beneficiary, 1.);
        }
    }
    actor(
        victim,
        Visual::Bestiary {
            number: if s.case == Case::Machine { 21 } else { 12 },
            corpse: false,
        },
    );
    if s.deployed {
        let mut p = source;
        if s.model() != 59 {
            if let Some(a) = s.attacks.last() {
                let age = s.clock - a.time;
                if (0.0..0.27).contains(&age) {
                    p += (enemy - source).normalize_or_zero()
                        * (std::f32::consts::PI * age / 0.27).sin()
                        * 12.;
                }
            }
        }
        art.drone(
            s.model(),
            p,
            if s.skill == 0 {
                s.started
                    .map_or(1., |t| ((s.clock - t) / 0.16).clamp(0., 1.))
            } else {
                1.
            },
        );
        if s.skill == 1 && s.case == Case::Support {
            art.drone(56, source + vec2(-64., 0.), 1.);
        }
        if s.skill == 0 && s.started.is_some() {
            let pulse = s.hits.last().map_or(0., |h| {
                let age = s.clock - h.time;
                if (0.0..0.32).contains(&age) && h.protected {
                    1. - age / 0.32
                } else {
                    0.
                }
            });
            if s.protection(PLAYER) {
                shield(player, if s.ally_model().is_none() { pulse } else { 0. });
            }
            if s.ally_model().is_some() && s.protection(s.ally()) {
                shield(beneficiary, pulse);
            }
            shield(p, 0.);
            if let Some(t) = s.started {
                arrival(source, s.clock - t);
            }
        }
        for (i, a) in s.attacks.iter().enumerate() {
            let age = s.clock - a.time;
            if s.model() == 59 {
                projectile(source, enemy, age);
            } else if (0.04..0.25).contains(&age) {
                let d = (enemy - source).normalize_or_zero();
                let n = vec2(-d.y, d.x);
                cut(
                    enemy - n * 22. - d * 10.,
                    enemy + n * 20. + d * 7.,
                    9.,
                    (1. - (age - 0.04) / 0.21).max(0.),
                );
            }
            let hit = age - 0.12;
            if s.case == Case::Machine {
                sparks(enemy, hit);
            } else {
                blood_spray(enemy, hit, 981 + i as u32 * 43, 0.72);
            }
            let label = enemy
                + if (source.x - enemy.x).abs() < 40. && source.y < enemy.y {
                    vec2(64., 0.)
                } else {
                    vec2(0., 0.)
                };
            number(label, &format!("−{}", s.damage()), hit, TEXT);
        }
        for (i, h) in s.hits.iter().enumerate() {
            let age = s.clock - h.time;
            if s.ally_model().is_none() {
                blood_spray(ally, age, 1171 + i as u32 * 37, 0.65);
            } else {
                sparks(ally, age);
            }
            let label = ally
                + if (source.x - ally.x).abs() < 40. && source.y < ally.y {
                    vec2(64., 0.)
                } else {
                    vec2(0., 0.)
                };
            number(
                label,
                &format!("−{}", State::received(h.protected)),
                age,
                TEXT,
            );
            if h.protected && (0.0..0.55).contains(&age) {
                number(
                    label + vec2(0., -22.),
                    "Protégé",
                    age,
                    Color::new(0.71, 0.84, 1., 1.),
                );
            }
        }
    }
}
fn cases() -> Vec<(Case, usize, &'static str)> {
    vec![
        (Case::Combat, 0, "DROITE"),
        (Case::Combat, 1, "GAUCHE"),
        (Case::Combat, 2, "HAUT"),
        (Case::Combat, 3, "BAS"),
        (Case::Tracker, 2, "TRAQUEUR"),
        (Case::Gunner, 2, "ARTILLEUR"),
        (Case::Far, 2, "HORS_PORTEE"),
        (Case::Wall, 2, "CLOISON"),
        (Case::Machine, 2, "MACHINE"),
        (Case::Support, 2, "SOUTIEN"),
        (Case::Quiet, 2, "ATTENTE"),
        (Case::Energy, 2, "ENERGIE"),
        (Case::Existing, 2, "EXISTANT"),
    ]
}
pub(crate) async fn run() {
    request_new_screen_size(1120., 850.);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let mut s = State::new(0, Case::Combat, 2);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Gardien et algorithmes · 101–102", 20., 35., 24., TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(Rect::new(20., 65. + i as f32 * 40., 330., 34.), title, true) {
                s = State::new(i, s.case, s.direction);
            }
        }
        for (i, title) in ["Droite", "Gauche", "Haut", "Bas"].iter().enumerate() {
            if button(
                Rect::new(
                    20. + (i % 2) as f32 * 166.,
                    155. + (i / 2) as f32 * 37.,
                    156.,
                    32.,
                ),
                title,
                true,
            ) {
                s = State::new(s.skill, s.case, i);
            }
        }
        for (i, case) in [
            Case::Combat,
            Case::Tracker,
            Case::Gunner,
            Case::Far,
            Case::Wall,
            Case::Machine,
            Case::Support,
            Case::Quiet,
            Case::Energy,
            Case::Existing,
        ]
        .iter()
        .enumerate()
        {
            if button(
                Rect::new(20., 235. + i as f32 * 25., 330., 23.),
                case.title(),
                true,
            ) {
                s = State::new(s.skill, *case, s.direction);
            }
        }
        let rect = Rect::new(400., 88., 576., 576.);
        if button(
            Rect::new(20., 510., 330., 34.),
            if s.skill == 0 {
                "Déployer"
            } else {
                "Observer une attaque"
            },
            s.reason().is_none() && s.started.is_none(),
        ) || is_key_pressed(KeyCode::Enter)
        {
            s.launch();
        }
        if button(
            Rect::new(20., 552., 330., 34.),
            "Passer un tour",
            s.started.is_some() && !s.busy(),
        ) || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(Rect::new(20., 594., 158., 34.), "Rejouer", true) {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(186., 594., 164., 34.),
            if paused { "Reprendre" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
        }
        s.settle();
        scene(&art, &s, rect);
        draw_text_bold(
            format!(
                "Énergie {} / 100 · recharge {} tours · tour {}",
                s.energy, s.cooldown, s.turn
            ),
            400.,
            704.,
            18.,
            TEXT,
        );
        draw_text(
            format!(
                "Cible : {} / 160 PV · impacts : {} · allié : {} / 120 PV",
                s.enemy_hp,
                s.attacks.iter().filter(|a| a.resolved).count(),
                s.ally_hp
            ),
            400.,
            734.,
            17.,
            TEXT,
        );
        draw_text(
            "Dégâts et actions préparés pour la revue ; IA de campagne séparée.",
            400.,
            764.,
            16.,
            DIM,
        );
        next_frame().await;
    }
}

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    if output.exists()
        || output
            .parent()
            .is_some_and(|p| p.join("visual-acceptance-01.json").exists())
    {
        return Err("Utiliser une revue neuve non validée.".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let art = Art::load();
    if std::env::args().any(|a| a == "--review-static-body") {
        return capture_body_proof(&art, output).await;
    }
    let target = render_target(576, 576);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 576., 576.))
    };
    let only = std::env::args().find_map(|a| a.strip_prefix("--review-case=").map(str::to_owned));
    let only_skill =
        std::env::args().find_map(|a| a.strip_prefix("--review-skill=").map(str::to_owned));
    let mut entries = Vec::new();
    for skill in 0..2 {
        if only_skill.as_deref().is_some_and(|id| id != IDS[skill]) {
            continue;
        }
        for (case, direction, label) in cases() {
            if only.as_deref().is_some_and(|wanted| wanted != label) {
                continue;
            }
            let mut s = State::new(skill, case, direction);
            let folder = format!("{}-{label}", IDS[skill]);
            let dir = output.join(&folder);
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let mut timeline = Vec::new();
            let mut indices = Vec::new();
            let mut known = BTreeMap::new();
            let mut images = Vec::new();
            for frame in 0..FRAMES {
                s.clock = frame as f32 / 24.;
                if frame == 12 {
                    s.launch();
                }
                if case != Case::Quiet && [48, 72, 96].contains(&frame) {
                    s.advance();
                }
                s.settle();
                timeline.push(s.snapshot());
                set_camera(&camera);
                clear_background(INK);
                scene(&art, &s, Rect::new(0., 0., 576., 576.));
                unsafe {
                    macroquad::window::get_internal_gl().flush();
                }
                let mut im = target.texture.get_texture_data();
                for p in im.bytes.as_chunks_mut::<4>().0 {
                    p[3] = 255;
                }
                let mut hash = DefaultHasher::new();
                im.bytes.hash(&mut hash);
                let h = hash.finish();
                let n = if let Some(n) = known.get(&h) {
                    *n
                } else {
                    let n = images.len();
                    images.push(im);
                    known.insert(h, n);
                    n
                };
                indices.push(n);
                set_default_camera();
                if frame % 8 == 0 {
                    next_frame().await;
                }
            }
            let columns = 6;
            let rows = images.len().div_ceil(columns);
            let mut atlas =
                Image::gen_image_color((576 * columns) as u16, (576 * rows) as u16, INK);
            for (i, im) in images.into_iter().enumerate() {
                for y in 0..576 {
                    let k = (((i / columns) * 576 + y) * 576 * columns + (i % columns) * 576) * 4;
                    atlas.bytes[k..k + 576 * 4]
                        .copy_from_slice(&im.bytes[y * 576 * 4..(y + 1) * 576 * 4]);
                }
            }
            // Raw RGB is only a temporary capture transport. This avoids
            // expensive debug PNG encoding; the packager verifies lossless
            // WebP pixels before removing this atlas.
            use std::io::Write;
            let mut file = std::io::BufWriter::new(
                std::fs::File::create(dir.join("atlas.ppm")).map_err(|e| e.to_string())?,
            );
            write!(file, "P6\n{} {}\n255\n", atlas.width, atlas.height)
                .map_err(|e| e.to_string())?;
            let stride = usize::from(atlas.width) * 4;
            let mut row = Vec::with_capacity(usize::from(atlas.width) * 3);
            for rgba in atlas.bytes.chunks_exact(stride).rev() {
                row.clear();
                for p in rgba.chunks_exact(4) {
                    row.extend_from_slice(&p[..3]);
                }
                file.write_all(&row).map_err(|e| e.to_string())?;
            }
            file.flush().map_err(|e| e.to_string())?;
            entries.push(serde_json::json!({"id":IDS[skill],"number":101+skill,"title":TITLES[skill],"variant":format!("{}-A01-{label}",IDS[skill]),"folder":folder,"case":case.label(),"direction":direction,"frames":FRAMES,"fps":24,"indices":indices,"unique_images":known.len(),"native_columns":columns,"timeline":timeline}));
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

async fn capture_body_proof(art: &Art, output: &Path) -> Result<(), String> {
    use macroquad::miniquad::{BlendFactor, BlendState, BlendValue, Equation, PipelineParams};
    let palette=load_material(ShaderSource::Glsl{vertex:r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;uniform mat4 Model;uniform mat4 Projection;varying lowp vec4 tint;varying highp vec2 uv;
void main(){gl_Position=Projection*Model*vec4(position,1.0);uv=texcoord;tint=color0/255.0;}"#,fragment:r#"#version 100
precision highp float;uniform sampler2D Texture;varying lowp vec4 tint;varying highp vec2 uv;
void main(){vec4 c=texture2D(Texture,uv);vec3 p=c.rgb;
if(c.b>c.g*1.2&&c.r>c.g*1.15&&c.b>0.13){float v=max(c.r,c.b);p=vec3(0.57,0.64,0.36)*(v/0.65);}
else if(c.g>c.r*1.4&&c.b>c.r*1.5&&min(c.g,c.b)>0.14){float v=max(c.g,c.b);p=v>0.7?vec3(0.957,0.902,0.773):v>0.4?vec3(0.827,0.780,0.663):vec3(0.525,0.486,0.361);}
else if(c.r>0.35&&c.g>0.28&&c.b<c.g*0.85){p=vec3(0.85,0.90,0.94)*max(c.r,c.g);}
gl_FragColor=vec4(clamp(p,0.0,1.0)*tint.rgb,c.a*tint.a);}"#},MaterialParams{pipeline_params:PipelineParams{color_blend:Some(BlendState::new(Equation::Add,BlendFactor::Value(BlendValue::SourceAlpha),BlendFactor::OneMinusValue(BlendValue::SourceAlpha))),alpha_blend:Some(BlendState::new(Equation::Add,BlendFactor::One,BlendFactor::OneMinusValue(BlendValue::SourceAlpha))),..Default::default()},..Default::default()}).map_err(|e|format!("{e:?}"))?;
    let target = render_target(64, 64);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 64., 64.))
    };
    for original in [true, false] {
        set_camera(&camera);
        clear_background(Color::new(0., 0., 0., 0.));
        if !original {
            gl_use_material(&palette);
        }
        game_art::draw_world(
            Rect::new(0., 0., 64., 64.),
            Visual::Bestiary {
                number: 58,
                corpse: false,
            },
            1.,
        );
        gl_use_default_material();
        unsafe {
            macroquad::window::get_internal_gl().flush();
        }
        target.texture.get_texture_data().export_png(
            output
                .join(if original {
                    "traqueur-original64.png"
                } else {
                    "D58-PAL02-native64.png"
                })
                .to_str()
                .unwrap(),
        );
        set_default_camera();
        next_frame().await;
    }
    let icon = Texture2D::from_file_with_format(
        include_bytes!("../../../assets/skills/agent/D58-compact.png"),
        Some(ImageFormat::Png),
    );
    icon.set_filter(FilterMode::Nearest);
    set_camera(&camera);
    clear_background(Color::new(0., 0., 0., 0.));
    gl_use_material(&palette);
    draw_texture_ex(
        &icon,
        0.,
        0.,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(64., 64.)),
            ..Default::default()
        },
    );
    gl_use_default_material();
    unsafe {
        macroquad::window::get_internal_gl().flush();
    }
    target
        .texture
        .get_texture_data()
        .export_png(output.join("D58-PAL02-icon64.png").to_str().unwrap());
    set_default_camera();
    next_frame().await;
    let stage = render_target(576, 576);
    let scene_camera = Camera2D {
        render_target: Some(stage.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 576., 576.))
    };
    let proposed = Texture2D::from_file_with_format(
        &std::fs::read(output.join("D58-PAL02-native64.png")).map_err(|e| e.to_string())?,
        Some(ImageFormat::Png),
    );
    proposed.set_filter(FilterMode::Nearest);
    for new_guardian in [true, false] {
        set_camera(&scene_camera);
        clear_background(INK);
        let s = State::new(0, Case::Combat, 2);
        terrain(&s.map(), 2, Rect::new(0., 0., 576., 576.), false);
        actor(
            point(PLAYER, 2, Rect::new(0., 0., 576., 576.)),
            player_visual(),
        );
        let p = point(SPAWN, 2, Rect::new(0., 0., 576., 576.));
        if new_guardian {
            art.drone(66, p, 1.);
        } else {
            draw_texture(&proposed, p.x - 32., p.y - 32., WHITE);
        }
        art.drone(
            59,
            point(GridPos::new(6, 6), 2, Rect::new(0., 0., 576., 576.)),
            1.,
        );
        unsafe {
            macroquad::window::get_internal_gl().flush();
        }
        stage.texture.get_texture_data().export_png(
            output
                .join(if new_guardian {
                    "guardian-scene64.png"
                } else {
                    "traqueur-scene64.png"
                })
                .to_str()
                .unwrap(),
        );
        set_default_camera();
        next_frame().await;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn next(s: &mut State) {
        s.clock += 1.;
        s.settle();
        s.advance();
        s.clock += 0.7;
        s.settle();
    }
    #[test]
    fn drone_guardian_atomic_invocation_and_passive_costs() {
        for case in [Case::Energy, Case::Existing] {
            let mut s = State::new(0, case, 2);
            let old = s.snapshot();
            s.launch();
            s.advance();
            assert_eq!(s.snapshot(), old);
        }
        let mut a = State::new(0, Case::Combat, 2);
        a.launch();
        assert_eq!((a.energy, a.cooldown, a.deployments), (86, 1, 1));
        let old = a.snapshot();
        a.launch();
        assert_eq!(old, a.snapshot());
        let mut p = State::new(1, Case::Energy, 2);
        p.launch();
        assert_eq!((p.energy, p.cooldown, p.deployments), (13, 0, 0));
        assert_eq!(p.damage(), 25);
    }
    #[test]
    fn drone_guardian_protection_respects_wall_distance_and_self() {
        for case in [Case::Combat, Case::Far, Case::Wall] {
            let mut s = State::new(0, case, 2);
            s.launch();
            assert!(s.protection(PLAYER));
            assert!(s.protection(SPAWN));
            assert_eq!(s.protection(s.ally()), case == Case::Combat);
            next(&mut s);
            assert_eq!(s.ally_hp, if case == Case::Combat { 101 } else { 100 });
        }
    }
    #[test]
    fn drone_guardian_machine_support_and_wait() {
        let mut machine = State::new(1, Case::Machine, 2);
        machine.launch();
        next(&mut machine);
        assert_eq!(machine.snapshot()["blood_events"], 0);
        assert!(machine.snapshot()["machine_hits"].as_u64().unwrap() > 0);
        let mut support = State::new(1, Case::Support, 2);
        support.launch();
        next(&mut support);
        assert!(support.attacks.is_empty());
        assert_eq!(support.enemy_hp, 160);
        let mut wall = State::new(1, Case::Wall, 2);
        wall.launch();
        next(&mut wall);
        assert!(wall.attacks.is_empty());
        for skill in 0..2 {
            let mut s = State::new(skill, Case::Combat, 2);
            s.launch();
            s.clock = 2.;
            s.settle();
            let before = s.snapshot();
            s.clock = 50.;
            s.settle();
            assert_eq!(s.snapshot(), before);
            assert_eq!(s.turn, 0);
        }
    }
}
