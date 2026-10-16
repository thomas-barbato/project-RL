//! ANIM-S50: prepared physical shots; no campaign combat, RNG or weapon mutation.
use super::super::melee::actor;
use super::zones::{position, preview, terrain};
use super::*;
use project_rl::skills::spatial::{Area, affected_cells, in_cast_range};
use project_rl::world::{Map, has_line_of_sight};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const IDS: [&str; 2] = ["DI03", "DI04"];
const TITLES: [&str; 2] = ["Volée", "Tir de précision"];
const PLAYER: GridPos = GridPos::new(4, 7);
const FPS: usize = 30;
const FRAMES: usize = 120;
const START: usize = 18;
const TURNS: [usize; 6] = [60, 70, 80, 90, 100, 110];
const FLIGHT: f32 = 0.18;
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#...#...#\n#.......#\n#.......#\n#.......#\n#########",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    Open,
    Wall,
    Machine,
    Miss,
    Cap,
    Empty,
    NoAmmo,
    Energy,
    Melee,
    Quiet,
}
impl Case {
    fn label(self) -> &'static str {
        match self {
            Self::Open => "TIRS",
            Self::Wall => "CLOISON",
            Self::Machine => "MACHINE",
            Self::Miss => "MANQUE",
            Self::Cap => "MAXIMUM",
            Self::Empty => "VIDE",
            Self::NoAmmo => "MUNITIONS",
            Self::Energy => "ENERGIE",
            Self::Melee => "MELEE",
            Self::Quiet => "ATTENTE",
        }
    }
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Tirs",
            Self::Wall => "Cloison isolée",
            Self::Machine => "Cibles mécaniques",
            Self::Miss => "Échec préparé",
            Self::Cap => "Plus de cibles que la limite",
            Self::Empty => "Aucune cible",
            Self::NoAmmo => "Sans munitions",
            Self::Energy => "Énergie insuffisante",
            Self::Melee => "Arme de mêlée active",
            Self::Quiet => "Attente sans autre tour",
        }
    }
}
struct Packet {
    launch: f32,
    impact: f32,
    target: usize,
    damage: u16,
    missed: bool,
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
    ammo: u16,
    cooldown: u16,
    damage: [u16; 7],
    packets: Vec<Packet>,
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
            energy: if case == Case::Energy { 5 } else { 100 },
            ammo: if case == Case::NoAmmo { 0 } else { 20 },
            cooldown: 0,
            damage: [0; 7],
            packets: Vec::new(),
        }
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[usize::from(self.case == Case::Wall)]).unwrap()
    }
    fn targets(&self) -> Vec<GridPos> {
        if self.case == Case::Empty {
            return Vec::new();
        }
        let xy: &[(i32, i32)] = if self.skill == 1 {
            &[(4, 2), (2, 3), (6, 3)]
        } else if self.case == Case::Cap {
            &[(3, 6), (5, 6), (4, 5), (2, 4), (6, 4), (4, 2), (6, 2)]
        } else {
            &[(3, 5), (5, 5), (4, 3), (2, 3), (6, 3), (7, 6)]
        };
        xy.iter().map(|&(x, y)| GridPos::new(x, y)).collect()
    }
    fn weapon_range(&self) -> u16 {
        6 + parameter_for(IDS[self.skill], "weapon_range_bonus_cells", 11)
    }
    fn area(&self) -> Vec<GridPos> {
        if self.skill == 0 {
            affected_cells(
                &self.map(),
                PLAYER,
                Area::Cone {
                    length: self.weapon_range().min(parameter_for(
                        "DI03",
                        "range_skill_limit_cells",
                        11,
                    )),
                },
                (0, -1),
            )
        } else {
            self.targets()
                .first()
                .copied()
                .filter(|p| self.visible(*p) && in_cast_range(PLAYER, *p, self.weapon_range()))
                .into_iter()
                .collect()
        }
    }
    fn visible(&self, at: GridPos) -> bool {
        has_line_of_sight(&self.map(), PLAYER, at, true)
    }
    fn selected(&self) -> Vec<usize> {
        if self.skill == 1 {
            return if self.area().is_empty() {
                vec![]
            } else {
                vec![0]
            };
        }
        let area = self.area();
        let targets = self.targets();
        let mut selected: Vec<_> = targets
            .iter()
            .enumerate()
            .filter(|(_, p)| area.contains(p) && in_cast_range(PLAYER, **p, self.weapon_range()))
            .map(|(n, _)| n)
            .collect();
        // Prepared equal-distance fixtures use stable indices; this does not
        // establish the campaign's unresolved tie-breaking policy.
        selected.sort_by_key(|&n| {
            let p = targets[n];
            ((p.x - PLAYER.x).pow(2) + (p.y - PLAYER.y).pow(2), n)
        });
        selected.truncate(usize::from(parameter_for("DI03", "maximum_targets", 11)));
        selected
    }
    fn reason(&self) -> Option<&'static str> {
        if self.case == Case::Melee {
            Some("Nécessite une arme à distance active.")
        } else if self.selected().is_empty() {
            Some("Aucune cible visible et accessible.")
        } else if usize::from(self.ammo) < self.selected().len() {
            Some("Munitions insuffisantes.")
        } else if self.energy < parameter_for(IDS[self.skill], "energy", 11) {
            Some("Énergie insuffisante.")
        } else {
            None
        }
    }
    fn start(&mut self) {
        if self.started.is_some() || self.reason().is_some() {
            return;
        }
        let selected = self.selected();
        self.started = Some(self.clock);
        self.turn = 1;
        self.energy -= parameter_for(IDS[self.skill], "energy", 11);
        self.ammo -= selected.len() as u16;
        self.cooldown = parameter_for(IDS[self.skill], "cooldown_turns", 11);
        for (order, target) in selected.into_iter().enumerate() {
            let launch = if self.skill == 0 {
                order as f32 * 0.025
            } else {
                0.
            };
            self.packets.push(Packet {
                launch,
                impact: launch + FLIGHT,
                target,
                damage: if self.skill == 0 { 29 } else { 22 },
                missed: self.case == Case::Miss && (self.skill == 1 || order == 2),
                resolved: false,
            });
        }
    }
    fn resolve(&mut self) {
        let Some(start) = self.started else {
            return;
        };
        for packet in &mut self.packets {
            if !packet.resolved && self.clock - start + 0.00001 >= packet.impact {
                packet.resolved = true;
                if !packet.missed {
                    self.damage[packet.target] += packet.damage;
                }
            }
        }
    }
    fn advance(&mut self) {
        if self.started.is_none() || self.case == Case::Quiet {
            return;
        }
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
    }
    fn snapshot(&self) -> serde_json::Value {
        let hits = self
            .packets
            .iter()
            .filter(|p| p.resolved && !p.missed)
            .count();
        serde_json::json!({"started":self.started.is_some(),"turn":self.turn,"energy":self.energy,"ammo":self.ammo,
            "cooldown":self.cooldown,"damage":self.damage,"hits":hits,
            "misses":self.packets.iter().filter(|p|p.resolved&&p.missed).count(),
            "blood_events":if self.case==Case::Machine{0}else{hits},"reaction_events":hits,
            "projectiles":self.packets.len(),"selected_targets":self.selected(),
            "preview":self.started.is_none()&&self.reason().is_none(),
            "refusal":if self.started.is_none(){self.reason()}else{None},
            "maximum_target_hp_fixture":240,"rank_fixture":11,"weapon_range_fixture":self.weapon_range(),
            "cone_depth_fixture":if self.skill==0{self.weapon_range().min(parameter_for("DI03","range_skill_limit_cells",11))}else{0},
            "critical_rules_modeled":false})
    }
}
fn point(at: GridPos, direction: usize, rect: Rect) -> Vec2 {
    let p = position(at, direction);
    vec2(
        rect.x + (p.x as f32 + 0.5) * 64.,
        rect.y + (p.y as f32 + 0.5) * 64.,
    )
}
fn axis(s: &State, rect: Rect) -> Vec2 {
    (point(GridPos::new(4, 6), s.direction, rect) - point(PLAYER, s.direction, rect)).normalize()
}
fn origin(s: &State, rect: Rect) -> Vec2 {
    point(PLAYER, s.direction, rect) + vec2(0., -8.) + axis(s, rect) * 24.
}
fn spark(at: Vec2, age: f32, facing: Vec2) {
    if !(0.0..0.24).contains(&age) {
        return;
    }
    let life = 1. - age / 0.24;
    for n in 0..7 {
        let a = n as f32 * 0.91;
        let d = vec2(a.cos(), a.sin());
        let d = if d.dot(facing) < 0. { -d } else { d };
        let p = at + d * (5. + age * (70. + n as f32 * 9.));
        draw_line(
            p.x - d.x * 6. * life,
            p.y - d.y * 6. * life,
            p.x,
            p.y,
            4. * life,
            alpha(INK, life),
        );
        draw_line(
            p.x - d.x * 5. * life,
            p.y - d.y * 5. * life,
            p.x,
            p.y,
            2. * life,
            alpha(Color::new(1., 0.75, 0.38, 1.), life),
        );
    }
}
fn tracer(from: Vec2, to: Vec2, age: f32, piercing: bool) {
    if !(0.0..FLIGHT).contains(&age) {
        return;
    }
    let delta = to - from;
    let d = delta.normalize_or_zero();
    let head = from.lerp(to, age / FLIGHT);
    let len: f32 = if piercing { 50. } else { 25. };
    let tail = head - d * len.min((head - from).length());
    for (width, c) in [
        (8., INK),
        (5., Color::new(0.88, 0.63, 0.29, 1.)),
        (2.4, Color::new(1., 0.96, 0.79, 1.)),
    ] {
        draw_line(tail.x, tail.y, head.x, head.y, width, c);
    }
    draw_circle(head.x, head.y, 2., WHITE);
}

fn reticle(at: Vec2, age: f32) {
    if !(0.0..0.28).contains(&age) {
        return;
    }
    let life = (1. - (age - 0.16).max(0.) / 0.12).clamp(0., 1.);
    let radius = 21. - 5. * (age / 0.10).min(1.);
    for (width, c) in [(5., INK), (2.5, Color::new(1., 0.18, 0.13, 1.))] {
        draw_circle_lines(at.x, at.y, radius, width, alpha(c, life));
        for d in [vec2(1., 0.), vec2(-1., 0.), vec2(0., 1.), vec2(0., -1.)] {
            let a = at + d * (radius - 5.);
            let b = at + d * (radius + 5.);
            draw_line(a.x, a.y, b.x, b.y, width, alpha(c, life));
        }
    }
}
struct Art {
    blood: Texture2D,
}
impl Art {
    fn load() -> Self {
        let blood = Texture2D::from_file_with_format(
            include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-40/blood-atlas.png"
            ),
            Some(ImageFormat::Png),
        );
        blood.set_filter(FilterMode::Linear);
        Self { blood }
    }
    fn scene(&self, s: &State, rect: Rect) {
        terrain(&s.map(), s.direction, rect, false);
        if s.started.is_none() && s.reason().is_none() {
            preview(&s.area(), s.direction, rect, s.clock);
            if (s.clock * 2.) as usize % 2 == 0 {
                for n in s.selected() {
                    let p = point(s.targets()[n], s.direction, rect);
                    draw_rectangle_lines(
                        p.x - 27.,
                        p.y - 27.,
                        54.,
                        54.,
                        2.,
                        Color::new(0.85, 1., 1., 1.),
                    );
                }
            }
        }
        let from = origin(s, rect);
        let direction = axis(s, rect);
        let age = s.started.map_or(-1., |t| s.clock - t);
        let recoil = s
            .packets
            .iter()
            .map(|p| age - p.launch)
            .filter(|a| (0.0..0.12).contains(a))
            .map(|a| 3. * (a / 0.12 * std::f32::consts::PI).sin())
            .fold(0., f32::max);
        actor(
            point(PLAYER, s.direction, rect) - direction * recoil,
            game_art::wear(
                game_art::player(
                    Some("core:traqueur"),
                    if s.case == Case::Melee {
                        PlayerWeaponPose::Melee
                    } else {
                        PlayerWeaponPose::Ranged
                    },
                    Some(if s.case == Case::Melee {
                        "core:melee_epee_longue"
                    } else {
                        "core:modele_cogneur_p01"
                    }),
                ),
                [("core:body_armor".into(), "core:tenue_traque".into())],
            ),
        );
        for (n, at) in s.targets().iter().enumerate() {
            if !s.visible(*at) {
                continue;
            }
            let p = point(*at, s.direction, rect);
            let last = s
                .packets
                .iter()
                .find(|v| v.target == n && v.resolved && !v.missed);
            let offset = last.map_or(Vec2::ZERO, |v| {
                crate::combat_visuals::hit_reaction_offset(
                    p,
                    point(PLAYER, s.direction, rect),
                    f64::from(age - v.impact),
                    64.,
                )
            });
            if s.case == Case::Machine {
                actor(
                    p + offset,
                    Visual::Bestiary {
                        number: 43,
                        corpse: false,
                    },
                );
            } else {
                enemy(p + offset);
            }
        }
        if s.skill == 1 && s.started.is_some() {
            reticle(
                point(s.targets()[0], s.direction, rect) + vec2(0., -8.),
                age,
            );
        }
        for packet in &s.packets {
            let target = point(s.targets()[packet.target], s.direction, rect) + vec2(0., -8.);
            let d = (target - from).normalize_or_zero();
            tracer(
                from,
                target
                    + if packet.missed {
                        vec2(-d.y, d.x) * 26.
                    } else {
                        Vec2::ZERO
                    },
                age - packet.launch,
                s.skill == 1,
            );
            if !packet.resolved {
                continue;
            }
            let a = age - packet.impact;
            if !packet.missed {
                if s.case == Case::Machine {
                    spark(target, a, -d);
                } else if (0.0..0.30).contains(&a) {
                    let frame = ((a / 0.30 * 6.) as usize).min(5);
                    draw_texture_ex(
                        &self.blood,
                        target.x - 29.,
                        target.y - 17.,
                        WHITE,
                        DrawTextureParams {
                            source: Some(Rect::new(frame as f32 * 256., 0., 256., 256.)),
                            dest_size: Some(vec2(58., 58.)),
                            ..Default::default()
                        },
                    );
                }
            }
            if (0.0..0.65).contains(&a) {
                let text = if packet.missed {
                    "Manqué".to_string()
                } else {
                    format!("-{}", s.damage[packet.target])
                };
                let x = target.x - measure_text(&text, None, 22, 1.).width / 2.;
                let y = target.y - 32. - a * 20.;
                let life = (1. - a / 0.65).min(1.);
                draw_text_bold(&text, x + 1., y + 1., 22., alpha(INK, life));
                draw_text_bold(
                    text,
                    x,
                    y,
                    22.,
                    alpha(
                        if packet.missed {
                            TEXT
                        } else {
                            Color::new(1., 0.89, 0.73, 1.)
                        },
                        life,
                    ),
                );
            }
        }
    }
}
fn cases() -> Vec<(Case, usize, &'static str)> {
    let mut out = vec![
        (Case::Open, 2, "HAUT"),
        (Case::Open, 0, "DROITE"),
        (Case::Open, 1, "GAUCHE"),
        (Case::Open, 3, "BAS"),
    ];
    out.extend([
        (Case::Wall, 2, "CLOISON"),
        (Case::Machine, 2, "MACHINE"),
        (Case::Miss, 2, "MANQUE"),
        (Case::Cap, 2, "MAXIMUM"),
        (Case::Empty, 2, "VIDE"),
        (Case::NoAmmo, 2, "MUNITIONS"),
        (Case::Energy, 2, "ENERGIE"),
        (Case::Melee, 2, "MELEE"),
        (Case::Quiet, 2, "ATTENTE"),
    ]);
    out
}
pub(crate) async fn run() {
    request_new_screen_size(1240., 755.);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let mut case_index = 0;
    let all: Vec<_> = cases()
        .into_iter()
        .filter(|(case, _, label)| *case != Case::Open || *label == "HAUT")
        .collect();
    let mut direction = 2;
    let mut states = [
        State::new(0, Case::Open, direction),
        State::new(1, Case::Open, direction),
    ];
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        let mut reset = is_key_pressed(KeyCode::R);
        if is_key_pressed(KeyCode::C) {
            case_index = (case_index + 1) % all.len();
            reset = true;
        }
        for (key, dir) in [
            (KeyCode::Right, 0),
            (KeyCode::Left, 1),
            (KeyCode::Up, 2),
            (KeyCode::Down, 3),
        ] {
            if is_key_pressed(key) {
                direction = dir;
                reset = true;
            }
        }
        if reset {
            states = [
                State::new(0, all[case_index].0, direction),
                State::new(1, all[case_index].0, direction),
            ];
        }
        for s in &mut states {
            s.clock += get_frame_time().min(0.1);
            if is_key_pressed(KeyCode::Enter) {
                s.start();
            }
            if is_key_pressed(KeyCode::Space) {
                s.advance();
            }
            s.resolve();
        }
        clear_background(INK);
        for (i, s) in states.iter().enumerate() {
            let x = 22. + i as f32 * 614.;
            draw_text_bold(format!("{} · {}", 111 + i, TITLES[i]), x, 32., 23., TEXT);
            art.scene(s, Rect::new(x, 70., 576., 576.));
            draw_text(
                format!(
                    "Tour {} · Énergie {} · Munitions {} · Recharge {}",
                    s.turn, s.energy, s.ammo, s.cooldown
                ),
                x,
                670.,
                15.,
                TEXT,
            );
            draw_text(s.case.title(), x, 694., 15., DIM);
            if let Some(reason) = s.reason().filter(|_| s.started.is_none()) {
                draw_text(reason, x, 715., 15., Color::new(1., 0.64, 0.32, 1.));
            }
        }
        draw_text(
            "Entrée : lancer · Espace : +1 tour · C : situation · Flèches : orientation · R : rejouer · Échap : fermer",
            22.,
            735.,
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
    let target = render_target(576, 576);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 576., 576.))
    };
    let mut entries = Vec::new();
    for skill in 0..2 {
        for (case, direction, label) in cases() {
            let mut s = State::new(skill, case, direction);
            let mut known = BTreeMap::new();
            let mut images = Vec::new();
            let mut indices = Vec::new();
            let mut timeline = Vec::new();
            let mut static_images = BTreeMap::new();
            for frame in 0..FRAMES {
                s.clock = frame as f32 / FPS as f32;
                if frame == START {
                    s.start();
                }
                if TURNS.contains(&frame) {
                    s.advance();
                }
                s.resolve();
                timeline.push(s.snapshot());
                // The scene has no clock-dependent idle pose. Only two preview
                // blink states exist, and all shot feedback has ended by frame60.
                // Reuse those exact still frames; keep every gameplay snapshot.
                let static_key = if s.started.is_none() {
                    Some(if s.reason().is_some() {
                        0_u8
                    } else {
                        1 + ((s.clock * 2.) as u8 % 2)
                    })
                } else if frame >= 60 {
                    Some(3_u8)
                } else {
                    None
                };
                if let Some(index) = static_key.and_then(|key| static_images.get(&key)).copied() {
                    indices.push(index);
                    if frame % 8 == 0 {
                        next_frame().await;
                    }
                    continue;
                }
                set_camera(&camera);
                clear_background(INK);
                art.scene(&s, Rect::new(0., 0., 576., 576.));
                unsafe {
                    macroquad::window::get_internal_gl().flush();
                }
                let mut im = target.texture.get_texture_data();
                for p in im.bytes.as_chunks_mut::<4>().0 {
                    p[3] = 255;
                }
                let mut hash = DefaultHasher::new();
                im.bytes.hash(&mut hash);
                let hash = hash.finish();
                let index = if let Some(&i) = known.get(&hash) {
                    i
                } else {
                    let i = images.len();
                    known.insert(hash, i);
                    images.push(im);
                    i
                };
                indices.push(index);
                if let Some(key) = static_key {
                    static_images.insert(key, index);
                }
                set_default_camera();
                if frame % 8 == 0 {
                    next_frame().await;
                }
            }
            let folder = format!("{}-{label}", IDS[skill]);
            let path = output.join(&folder);
            std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
            let cols = 6;
            let rows = images.len().div_ceil(cols);
            let mut atlas = Image::gen_image_color((576 * cols) as u16, (576 * rows) as u16, INK);
            for (i, im) in images.into_iter().enumerate() {
                for y in 0..576 {
                    let at = (((i / cols) * 576 + y) * 576 * cols + (i % cols) * 576) * 4;
                    atlas.bytes[at..at + 576 * 4]
                        .copy_from_slice(&im.bytes[y * 576 * 4..(y + 1) * 576 * 4]);
                }
            }
            atlas.export_png(path.join("atlas.png").to_str().unwrap());
            entries.push(serde_json::json!({"id":IDS[skill],"number":111+skill,"variant":format!("{}-A01-{label}",IDS[skill]),
            "title":TITLES[skill],"case":case.label(),"case_title":case.title(),"label":label,"folder":folder,"direction":direction,
            "frames":FRAMES,"fps":FPS,"start_frame":START,"turn_frames":TURNS,"columns":cols,"unique_images":known.len(),
            "indices":indices,"timeline":timeline}));
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn finish(s: &mut State) {
        s.start();
        s.clock = 4.;
        s.resolve();
    }
    #[test]
    fn ranged_fan_precision_selects_nearest_cap_and_keeps_one_target() {
        let mut volley = State::new(0, Case::Open, 2);
        assert_eq!(volley.selected(), vec![0, 1, 2, 3, 4]);
        finish(&mut volley);
        assert_eq!(volley.damage, [29, 29, 29, 29, 29, 0, 0]);
        assert_eq!((volley.ammo, volley.energy, volley.cooldown), (15, 85, 1));
        let mut cap = State::new(0, Case::Cap, 2);
        assert_eq!(cap.selected(), vec![0, 1, 2, 3, 4]);
        finish(&mut cap);
        assert_eq!(cap.damage, [29, 29, 29, 29, 29, 0, 0]);
        let mut precision = State::new(1, Case::Open, 2);
        finish(&mut precision);
        assert_eq!(precision.damage, [22, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            (precision.ammo, precision.energy, precision.cooldown),
            (19, 89, 1)
        );
    }
    #[test]
    fn ranged_fan_precision_walls_and_refusals_are_atomic() {
        let mut wall = State::new(0, Case::Wall, 2);
        assert_eq!(wall.selected(), vec![0, 1, 3, 4]);
        finish(&mut wall);
        assert_eq!(wall.damage, [29, 29, 0, 29, 29, 0, 0]);
        for skill in 0..2 {
            for case in [Case::Empty, Case::NoAmmo, Case::Energy, Case::Melee] {
                let mut s = State::new(skill, case, 2);
                let before = s.snapshot();
                finish(&mut s);
                assert_eq!(s.snapshot(), before);
            }
        }
        let mut wall = State::new(1, Case::Wall, 2);
        let before = wall.snapshot();
        finish(&mut wall);
        assert_eq!(wall.snapshot(), before);
        let mut short = State::new(0, Case::Open, 2);
        short.ammo = 4;
        let before = short.snapshot();
        finish(&mut short);
        assert_eq!(short.snapshot(), before);
    }
    #[test]
    fn ranged_fan_precision_misses_machines_and_later_turns_do_not_repeat() {
        for skill in 0..2 {
            let mut miss = State::new(skill, Case::Miss, 2);
            finish(&mut miss);
            assert_eq!(miss.snapshot()["misses"], 1);
            assert_eq!(miss.damage[if skill == 0 { 2 } else { 0 }], 0);
            let damage = miss.damage;
            let ammo = miss.ammo;
            for _ in 0..8 {
                miss.advance();
                miss.clock += 3.;
                miss.resolve();
                miss.start();
            }
            assert_eq!(miss.damage, damage);
            assert_eq!(miss.ammo, ammo);
            assert_eq!(miss.cooldown, 0);
            let mut machine = State::new(skill, Case::Machine, 2);
            finish(&mut machine);
            assert_eq!(machine.snapshot()["blood_events"], 0);
            assert_eq!(
                machine.snapshot()["reaction_events"],
                machine.snapshot()["hits"]
            );
        }
    }
}
