//! ANIM-S45: prepared combat-drone phases, isolated from campaign AI.
use super::super::melee::{actor, blood_spray, cut};
use super::zones::{Clip, position, preview, terrain};
use super::*;
use macroquad::miniquad::{BlendFactor, BlendState, BlendValue, Equation, PipelineParams};
use project_rl::world::{Map, has_line_of_sight};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const IDS: [&str; 2] = ["D58", "D59"];
const TITLES: [&str; 2] = ["Invocation du drone Traqueur", "Invocation de l’Artilleur"];
const PLAYER: GridPos = GridPos::new(4, 6);
const SPAWN: GridPos = GridPos::new(4, 5);
const FRAMES: usize = 144;
const DAMAGE: [u16; 2] = [18, 12]; // Visual fixtures, not native weapon statistics.
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#########\n#.......#\n#.......#\n#.......#\n#########",
];

#[derive(Clone, Copy, PartialEq, Eq)]
enum Case {
    Combat,
    Contact,
    Move,
    Wall,
    Machine,
    Quiet,
    Energy,
    Existing,
}
impl Case {
    fn label(self) -> &'static str {
        match self {
            Self::Combat => "COMBAT",
            Self::Contact => "CONTACT",
            Self::Move => "DEPLACEMENT",
            Self::Wall => "CLOISON",
            Self::Machine => "MACHINE",
            Self::Quiet => "ATTENTE",
            Self::Energy => "ENERGIE",
            Self::Existing => "EXISTANT",
        }
    }
    fn title(self) -> &'static str {
        match self {
            Self::Combat => "Poursuite et tir",
            Self::Contact => "Cible au contact",
            Self::Move => "Déplacement préparé",
            Self::Wall => "Cloison infranchissable",
            Self::Machine => "Cible mécanique",
            Self::Quiet => "Attente sans nouveau tour",
            Self::Energy => "Énergie insuffisante",
            Self::Existing => "Modèle déjà déployé",
        }
    }
}

struct Attack {
    time: f32,
    from: GridPos,
    resolved: bool,
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    started: Option<f32>,
    movement: Option<(GridPos, f32)>,
    attacks: Vec<Attack>,
    turn: u16,
    energy: u16,
    cooldown: u16,
    deployed: bool,
    deployments: u16,
    spawn: GridPos,
    drone: GridPos,
    enemy_hp: u16,
    moves: u16,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.,
            started: None,
            movement: None,
            attacks: Vec::new(),
            turn: 0,
            energy: if case == Case::Energy { 11 } else { 100 },
            cooldown: 0,
            deployed: case == Case::Existing,
            deployments: 0,
            spawn: SPAWN,
            drone: SPAWN,
            enemy_hp: 100,
            moves: 0,
        }
    }
    fn map_index(&self) -> usize {
        usize::from(self.case == Case::Wall)
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn target(&self) -> GridPos {
        GridPos::new(
            4,
            if self.case == Case::Contact {
                4
            } else if self.case == Case::Wall {
                3
            } else {
                2
            },
        )
    }
    fn reason(&self) -> Option<&'static str> {
        if self.deployed {
            Some("Ce modèle est déjà déployé.")
        } else if self.energy < parameter_for(IDS[self.skill], "energy", 1) {
            Some("Énergie insuffisante.")
        } else if (self.spawn.x - PLAYER.x)
            .abs()
            .max((self.spawn.y - PLAYER.y).abs())
            != 1
            || !self.map().is_walkable(self.spawn)
            || !has_line_of_sight(&self.map(), PLAYER, self.spawn, true)
        {
            Some("Choisir une case adjacente libre.")
        } else if self.spawn == self.target() {
            Some("Case occupée.")
        } else {
            None
        }
    }
    fn deploy(&mut self) {
        if self.reason().is_some() {
            return;
        }
        self.energy -= parameter_for(IDS[self.skill], "energy", 1);
        self.cooldown = parameter_for(IDS[self.skill], "cooldown_turns", 1);
        self.deployed = true;
        self.deployments += 1;
        self.started = Some(self.clock);
        // This fixture includes the ordinary familiar phase of the invocation
        // turn. It grants no second action and defines no campaign scheduler.
        self.familiar_phase(self.clock + 0.34);
    }
    fn familiar_phase(&mut self, time: f32) {
        let to = self.target();
        // One prepared movement order replaces this phase's attack. It does
        // not add an action or define autonomous campaign target priorities.
        if self.case == Case::Move && self.turn == 1 {
            let next = GridPos::new(self.drone.x + 1, self.drone.y);
            if self.map().is_walkable(next) && next != PLAYER && next != to {
                self.movement = Some((self.drone, time));
                self.drone = next;
                self.moves += 1;
            }
            return;
        }
        if self.enemy_hp == 0 || !has_line_of_sight(&self.map(), self.drone, to, true) {
            return;
        }
        let distance = (to.x - self.drone.x).abs().max((to.y - self.drone.y).abs());
        if distance <= if self.skill == 0 { 1 } else { 4 } {
            self.attacks.push(Attack {
                time,
                from: self.drone,
                resolved: false,
            });
        } else if self.skill == 0 {
            // Prepared straight route, one normal action per illustrated phase.
            // General pursuit/pathfinding and speed remain outside this review.
            let next = GridPos::new(
                self.drone.x + (to.x - self.drone.x).signum(),
                self.drone.y + (to.y - self.drone.y).signum(),
            );
            if self.map().is_walkable(next) && next != PLAYER && next != to {
                self.movement = Some((self.drone, time));
                self.drone = next;
                self.moves += 1;
            }
        }
    }
    fn impact_delay(&self) -> f32 {
        if self.skill == 0 { 0.12 } else { 0.10 }
    }
    fn settle(&mut self) {
        let delay = self.impact_delay();
        for attack in &mut self.attacks {
            if !attack.resolved && self.clock >= attack.time + delay {
                self.enemy_hp = self.enemy_hp.saturating_sub(DAMAGE[self.skill]);
                attack.resolved = true;
            }
        }
    }
    fn busy(&self) -> bool {
        self.started.is_some_and(|t| self.clock < t + 0.34)
            || self.movement.is_some_and(|(_, t)| self.clock < t + 0.16)
            || self
                .attacks
                .last()
                .is_some_and(|a| self.clock < a.time + 0.32)
    }
    fn advance(&mut self) {
        self.settle();
        if self.started.is_none() || self.busy() {
            return;
        }
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        self.familiar_phase(self.clock);
    }
    fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({"turn":self.turn,"energy":self.energy,"cooldown":self.cooldown,
            "launched":self.started.is_some(),"deployed":self.deployed,"deployments":self.deployments,
            "actor_identity":if self.deployed {Some(if self.case==Case::Existing {"existing-drone"}else{"new-drone"})}else{None},
            "spawn":[self.spawn.x,self.spawn.y],"drone":[self.drone.x,self.drone.y],
            "target":[self.target().x,self.target().y],"target_kind":if self.case==Case::Machine {"machine"}else{"living"},
            "preview":self.started.is_none()&&self.reason().is_none(),
            "refusal":if self.started.is_none(){self.reason()}else{None},
            "enemy_hp":self.enemy_hp,"damage_fixture":DAMAGE[self.skill],"moves":self.moves,
            "attacks_scheduled":self.attacks.len(),"impacts":self.attacks.iter().filter(|a|a.resolved).count(),
            "blood_events":if self.case==Case::Machine {0}else{self.attacks.iter().filter(|a|a.resolved).count()},
            "line_of_sight":has_line_of_sight(&self.map(),self.drone,self.target(),true)})
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
    artilleur: Texture2D,
    traqueur_palette: Material,
}
impl Art {
    fn load() -> Self {
        let artilleur = Texture2D::from_file_with_format(
            include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-92/artilleur-body03/D59-BODY03-native64.png"
            ),
            Some(ImageFormat::Png),
        );
        artilleur.set_filter(FilterMode::Nearest);
        let traqueur_palette = load_material(
            ShaderSource::Glsl {
                vertex: r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;
varying lowp vec4 tint;varying highp vec2 uv;
void main(){gl_Position=Projection*Model*vec4(position,1.0);uv=texcoord;tint=color0/255.0;}"#,
                fragment: r#"#version 100
precision highp float;uniform sampler2D Texture;
varying lowp vec4 tint;varying highp vec2 uv;
void main(){vec4 c=texture2D(Texture,uv);vec3 p=c.rgb;
if(c.b>c.g*1.2&&c.r>c.g*1.15&&c.b>0.13){
 float v=max(c.r,c.b);p=vec3(0.57,0.64,0.36)*(v/0.65);
}else if(c.g>c.r*1.4&&c.b>c.r*1.5&&min(c.g,c.b)>0.14){
 float v=max(c.g,c.b);p=mix(vec3(0.22,0.27,0.28),vec3(0.58,0.64,0.63),v);
}else if(c.r>0.35&&c.g>0.28&&c.b<c.g*0.85){
 p=vec3(0.85,0.90,0.94)*max(c.r,c.g);
}
gl_FragColor=vec4(clamp(p,0.0,1.0)*tint.rgb,c.a*tint.a);}"#,
            },
            MaterialParams {
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    )),
                    alpha_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::One,
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .expect("Traqueur review palette");
        Self {
            artilleur,
            traqueur_palette,
        }
    }
    fn draw_drone(&self, skill: usize, p: Vec2, opacity: f32) {
        if skill == 0 {
            gl_use_material(&self.traqueur_palette);
            game_art::draw_world(
                Rect::new(p.x - 32., p.y - 32., 64., 64.),
                Visual::Bestiary {
                    number: 58,
                    corpse: false,
                },
                opacity,
            );
            gl_use_default_material();
        } else {
            // Native64 candidate is confined to this isolated laboratory.
            // Contact shadow is separate from its transparent source.
            draw_ellipse(p.x, p.y + 29., 25., 4., 0., alpha(BLACK, opacity * 0.30));
            draw_texture(&self.artilleur, p.x - 32., p.y - 32., alpha(WHITE, opacity));
        }
    }
}
fn shot(from: Vec2, to: Vec2, age: f32) {
    let d = (to - from).normalize_or_zero();
    // D59-BODY03 barrel openings: (13,35) and (51,35) in its64px canvas.
    let muzzle = from + vec2(if d.x < -0.1 { -19. } else { 19. }, 3.);
    let dir = (to - muzzle).normalize_or_zero();
    let normal = vec2(-dir.y, dir.x);
    if (0.0..0.10).contains(&age) {
        let f = 1. - age / 0.10;
        draw_triangle(
            muzzle - normal * 5.,
            muzzle + dir * (15. + 6. * f),
            muzzle + normal * 5.,
            alpha(INK, f),
        );
        draw_triangle(
            muzzle - normal * 3.,
            muzzle + dir * 17.,
            muzzle + normal * 3.,
            alpha(GOLD, f),
        );
        let p = muzzle.lerp(to, age / 0.10);
        draw_line(
            p.x - dir.x * 22.,
            p.y - dir.y * 22.,
            p.x,
            p.y,
            4.,
            alpha(INK, 0.8),
        );
        draw_line(
            p.x - dir.x * 18.,
            p.y - dir.y * 18.,
            p.x,
            p.y,
            2.,
            alpha(GOLD, 0.9),
        );
        draw_line(
            p.x - dir.x * 7.,
            p.y - dir.y * 7.,
            p.x,
            p.y,
            4.,
            Color::new(0.73, 0.86, 0.90, 1.),
        );
        draw_circle(p.x, p.y, 1.8, WHITE);
    }
}
fn scene(clip: &Clip, art: &Art, s: &State, rect: Rect) {
    terrain(&s.map(), s.direction, rect, false);
    if s.started.is_none() && s.reason().is_none() {
        preview(&[s.spawn], s.direction, rect, s.clock);
    }
    let at = point(s.target(), s.direction, rect);
    let mut target = at;
    for attack in &s.attacks {
        let age = s.clock - attack.time - s.impact_delay();
        target += crate::combat_visuals::hit_reaction_offset(
            at,
            point(attack.from, s.direction, rect),
            f64::from(age),
            64.,
        );
    }
    actor(point(PLAYER, s.direction, rect), player_visual());
    actor(
        target,
        Visual::Bestiary {
            number: if s.case == Case::Machine { 21 } else { 12 },
            corpse: false,
        },
    );
    if s.deployed {
        let mut p = point(s.drone, s.direction, rect);
        if let Some((from, time)) = s.movement {
            p = point(from, s.direction, rect).lerp(p, ((s.clock - time) / 0.16).clamp(0., 1.));
        }
        if s.skill == 0 {
            if let Some(a) = s.attacks.last() {
                let age = s.clock - a.time;
                if (0.0..0.27).contains(&age) {
                    p += (at - p).normalize_or_zero()
                        * (std::f32::consts::PI * age / 0.27).sin()
                        * 13.;
                }
            }
        }
        let opacity = s
            .started
            .map_or(1., |t| ((s.clock - t) / 0.16).clamp(0., 1.));
        art.draw_drone(s.skill, p, opacity);
        clip.apply_for(s.map_index(), s.direction, rect);
        if let Some(t) = s.started {
            arrival(point(s.spawn, s.direction, rect), s.clock - t);
        }
        for (i, a) in s.attacks.iter().enumerate() {
            let source = point(a.from, s.direction, rect);
            let age = s.clock - a.time;
            if s.skill == 0 && (0.04..0.25).contains(&age) {
                let d = (at - source).normalize_or_zero();
                let n = vec2(-d.y, d.x);
                cut(
                    at - n * 23. - d * 13.,
                    at + n * 21. + d * 8.,
                    11.,
                    (1. - (age - 0.04) / 0.21).max(0.),
                );
            } else if s.skill == 1 {
                shot(source, at, age);
            }
            let hit = age - s.impact_delay();
            if s.case == Case::Machine {
                sparks(at, hit);
            } else {
                blood_spray(at, hit, 911 + i as u32 * 43, 0.78);
            }
        }
        gl_use_default_material();
        for a in &s.attacks {
            let age = s.clock - a.time - s.impact_delay();
            if (0.0..0.75).contains(&age) {
                let text = format!("−{}", DAMAGE[s.skill]);
                let w = measure_text(&text, None, 22, 1.).width;
                // When attacking downward the drone occupies the space above
                // the victim. Move its damage number to the victim's side.
                let source = point(a.from, s.direction, rect);
                let offset_x = if source.y < at.y - 24. && (source.x - at.x).abs() < 40. {
                    44.
                } else {
                    0.
                };
                draw_text_bold(
                    text,
                    at.x + offset_x - w / 2.,
                    at.y - 45. - age * 19.,
                    22.,
                    alpha(TEXT, ((0.75 - age) * 3.).min(1.)),
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
        (Case::Contact, 2, "CONTACT"),
        (Case::Move, 2, "DEPLACEMENT"),
        (Case::Wall, 2, "CLOISON"),
        (Case::Machine, 2, "MACHINE"),
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
    let clip = Clip::load_maps(&MAPS, PLAYER);
    let art = Art::load();
    let mut s = State::new(0, Case::Combat, 2);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Drones de combat · 99–100", 20., 35., 24., TEXT);
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
            Case::Contact,
            Case::Move,
            Case::Wall,
            Case::Machine,
            Case::Quiet,
            Case::Energy,
            Case::Existing,
        ]
        .iter()
        .enumerate()
        {
            if button(
                Rect::new(20., 235. + i as f32 * 33., 330., 29.),
                case.title(),
                true,
            ) {
                s = State::new(s.skill, *case, s.direction);
            }
        }
        let rect = Rect::new(400., 88., 576., 576.);
        if button(
            Rect::new(20., 510., 330., 34.),
            "Déployer",
            s.reason().is_none(),
        ) || is_key_pressed(KeyCode::Enter)
        {
            s.deploy();
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
        scene(&clip, &art, &s, rect);
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
                "Cible : {} / 100 PV · impacts : {} · déplacements : {}",
                s.enemy_hp,
                s.attacks.iter().filter(|a| a.resolved).count(),
                s.moves
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
    let clip = Clip::load_maps(&MAPS, PLAYER);
    let art = Art::load();
    capture_palette_proof(&art, output)?;
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
                    s.deploy();
                }
                if case != Case::Quiet && [48, 72, 96].contains(&frame) {
                    s.advance();
                }
                s.settle();
                timeline.push(s.snapshot());
                set_camera(&camera);
                clear_background(INK);
                scene(&clip, &art, &s, Rect::new(0., 0., 576., 576.));
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
            atlas.export_png(dir.join("atlas.png").to_str().unwrap());
            entries.push(serde_json::json!({"id":IDS[skill],"number":99+skill,"title":TITLES[skill],"variant":format!("{}-A02-{label}",IDS[skill]),"folder":folder,"case":case.label(),"direction":direction,"frames":FRAMES,"fps":24,"indices":indices,"unique_images":known.len(),"native_columns":columns,"timeline":timeline}));
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
fn capture_palette_proof(art: &Art, output: &Path) -> Result<(), String> {
    let target = render_target(64, 64);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 64., 64.))
    };
    let mut before = None;
    let mut changed = 0;
    for recolored in [false, true] {
        set_camera(&camera);
        clear_background(Color::new(0., 0., 0., 0.));
        if recolored {
            gl_use_material(&art.traqueur_palette);
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
        let image = target.texture.get_texture_data();
        if let Some(old) = &before {
            let old: &Image = old;
            for (a, b) in old.bytes.chunks_exact(4).zip(image.bytes.chunks_exact(4)) {
                if (a[3] > 0) != (b[3] > 0) {
                    return Err("La palette a modifié la silhouette.".into());
                }
                if a[..3] != b[..3] {
                    changed += 1;
                }
            }
            image.export_png(output.join("traqueur-palette64.png").to_str().unwrap());
        } else {
            before = Some(image);
        }
    }
    let icon = Texture2D::from_file_with_format(
        include_bytes!("../../../assets/skills/agent/D58-compact.png"),
        Some(ImageFormat::Png),
    );
    set_camera(&camera);
    clear_background(Color::new(0., 0., 0., 0.));
    gl_use_material(&art.traqueur_palette);
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
        .export_png(output.join("traqueur-icon64.png").to_str().unwrap());
    set_default_camera();
    std::fs::write(output.join("palette-proof.json"),serde_json::to_vec_pretty(&serde_json::json!({"compared_pixels":4096,"occupied_alpha_mask_unchanged":true,"changed_RGB_pixels":changed,"source_asset_modified":false,"method":"Isolated GPU palette material; original sprite geometry and alpha retained."})).unwrap()).map_err(|e|e.to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn next_phase(s: &mut State) {
        s.clock += 1.;
        s.settle();
        s.advance();
        s.clock += 0.4;
        s.settle();
    }
    #[test]
    fn drone_combat_deployment_is_atomic_and_waiting_grants_no_new_action() {
        for skill in 0..2 {
            for case in [Case::Energy, Case::Existing] {
                let mut s = State::new(skill, case, 2);
                let before = s.snapshot();
                s.deploy();
                s.advance();
                s.settle();
                assert_eq!(s.snapshot(), before);
            }
            let mut s = State::new(skill, Case::Combat, 2);
            s.deploy();
            assert_eq!((s.energy, s.cooldown, s.deployments), (88, 1, 1));
            let before = s.snapshot();
            s.deploy();
            assert_eq!(s.snapshot(), before);
            s.clock = 1.;
            s.settle();
            let settled = s.snapshot();
            s.clock = 100.;
            s.settle();
            assert_eq!(s.snapshot(), settled);
            assert!(s.deployed);
            assert_eq!(s.turn, 0);
            assert_eq!(s.cooldown, 1);
        }
    }
    #[test]
    fn drone_combat_first_familiar_phase_moves_or_attacks_and_wall_blocks_both() {
        let mut melee = State::new(0, Case::Combat, 2);
        melee.deploy();
        assert_eq!(melee.drone, GridPos::new(4, 4));
        assert_eq!(melee.moves, 1);
        assert!(melee.attacks.is_empty());
        next_phase(&mut melee);
        assert_eq!(melee.drone, GridPos::new(4, 3));
        assert!(melee.attacks.is_empty());
        next_phase(&mut melee);
        assert_eq!(melee.enemy_hp, 82);
        assert_eq!(melee.attacks.len(), 1);
        let mut ranged = State::new(1, Case::Combat, 2);
        ranged.deploy();
        assert_eq!(ranged.attacks.len(), 1);
        assert_eq!(ranged.turn, 0);
        ranged.clock = 1.;
        ranged.settle();
        assert_eq!(ranged.enemy_hp, 88);
        assert_eq!(ranged.drone, SPAWN);
        for skill in 0..2 {
            let mut wall = State::new(skill, Case::Wall, 2);
            wall.deploy();
            for _ in 0..4 {
                next_phase(&mut wall);
            }
            assert_eq!(wall.enemy_hp, 100);
            assert!(wall.attacks.is_empty());
            assert_eq!(wall.drone, SPAWN);
            assert_eq!(wall.moves, 0);
        }
    }
    #[test]
    fn drone_combat_machine_has_identical_hits_but_no_blood() {
        for skill in 0..2 {
            let mut flesh = State::new(skill, Case::Combat, 2);
            flesh.deploy();
            let mut machine = State::new(skill, Case::Machine, 2);
            machine.deploy();
            for _ in 0..3 {
                next_phase(&mut flesh);
                next_phase(&mut machine);
            }
            assert_eq!(machine.enemy_hp, flesh.enemy_hp);
            assert_eq!(machine.attacks.len(), flesh.attacks.len());
            assert_eq!(machine.snapshot()["blood_events"], 0);
            assert!(flesh.snapshot()["blood_events"].as_u64().unwrap() > 0);
        }
    }
    #[test]
    fn drone_combat_prepared_move_replaces_an_attack_instead_of_adding_one() {
        for skill in 0..2 {
            let mut s = State::new(skill, Case::Move, 2);
            s.deploy();
            s.clock = 1.;
            s.settle();
            let attacks = s.attacks.len();
            let before = s.drone;
            next_phase(&mut s);
            assert_eq!(s.drone, GridPos::new(before.x + 1, before.y));
            assert_eq!(s.attacks.len(), attacks);
            next_phase(&mut s);
            next_phase(&mut s);
            assert_eq!(s.attacks.len(), if skill == 0 { 1 } else { 3 });
            assert_eq!(s.enemy_hp, if skill == 0 { 82 } else { 64 });
        }
    }
}
