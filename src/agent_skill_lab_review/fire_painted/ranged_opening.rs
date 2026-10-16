//! ANIM-S49: prepared physical shots; no campaign combat, RNG or weapon mutation.
use super::super::melee::actor;
use super::zones::{position, preview, terrain};
use super::*;
use project_rl::world::{Map, has_line_of_sight};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const IDS: [&str; 2] = ["DI01", "DI02"];
const TITLES: [&str; 2] = ["Tir perforant", "Feu nourri"];
const PLAYER: GridPos = GridPos::new(4, 7);
const FPS: usize = 30;
const FRAMES: usize = 120;
const START: usize = 18;
const TURNS: [usize; 4] = [66, 78, 90, 102];
const FLIGHT: f32 = 0.18;
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#########\n#.......#\n#.......#\n#.......#\n#########",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    Open,
    Wall,
    Machine,
    Miss,
    Ally,
    Cap,
    Burst,
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
            Self::Ally => "ALLIE",
            Self::Cap => "MAXIMUM",
            Self::Burst => "RAFALE",
            Self::NoAmmo => "MUNITIONS",
            Self::Energy => "ENERGIE",
            Self::Melee => "MELEE",
            Self::Quiet => "ATTENTE",
        }
    }
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Tirs",
            Self::Wall => "Cloison",
            Self::Machine => "Cibles mécaniques",
            Self::Miss => "Échecs préparés",
            Self::Ally => "Allié dans la ligne",
            Self::Cap => "Limite de traversée",
            Self::Burst => "Mode rafale de trois",
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
    damage: [u16; 5],
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
            damage: [0; 5],
            packets: Vec::new(),
        }
    }
    fn map_index(&self) -> usize {
        usize::from(self.case == Case::Wall)
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[self.map_index()]).unwrap()
    }
    fn targets(&self) -> Vec<GridPos> {
        if self.skill == 0 && self.case == Case::Cap {
            return (2..=6).rev().map(|y| GridPos::new(4, y)).collect();
        }
        if self.skill == 0 {
            vec![
                GridPos::new(4, 5),
                GridPos::new(4, 3),
                GridPos::new(4, 1),
                GridPos::new(6, 3),
            ]
        } else {
            vec![GridPos::new(4, 2), GridPos::new(6, 3), GridPos::new(2, 3)]
        }
    }
    fn native_projectiles(&self) -> u16 {
        if self.case == Case::Burst { 3 } else { 1 }
    }
    fn shot_count(&self) -> u16 {
        if self.skill == 0 {
            1
        } else {
            2 * self.native_projectiles()
        }
    }
    fn reason(&self) -> Option<&'static str> {
        if self.case == Case::Melee {
            Some("Nécessite une arme à distance active.")
        } else if self.ammo < self.shot_count() {
            Some("Munitions insuffisantes.")
        } else if self.energy < parameter_for(IDS[self.skill], "energy", 11) {
            Some("Énergie insuffisante.")
        } else if self.skill == 1
            && !has_line_of_sight(&self.map(), PLAYER, self.targets()[0], true)
        {
            Some("La cloison masque la cible.")
        } else {
            None
        }
    }
    fn path(&self) -> Vec<GridPos> {
        let map = self.map();
        let mut cells = Vec::new();
        let cap_y = if self.case == Case::Cap && self.skill == 0 {
            3
        } else {
            1
        };
        let target_y = if self.skill == 1 {
            self.targets()[0].y
        } else {
            cap_y
        };
        for y in (target_y..PLAYER.y).rev() {
            let at = GridPos::new(4, y);
            if !map.is_walkable(at) {
                break;
            }
            cells.push(at);
        }
        cells
    }
    fn start(&mut self) {
        if self.started.is_some() || self.reason().is_some() {
            return;
        }
        self.started = Some(self.clock);
        self.turn = 1;
        self.energy -= parameter_for(IDS[self.skill], "energy", 11);
        self.ammo -= self.shot_count();
        self.cooldown = parameter_for(IDS[self.skill], "cooldown_turns", 11);
        if self.skill == 0 {
            let path = self.path();
            let flight_pixels = if self.case == Case::Wall {
                // From the muzzle to the near face of the wall, never its centre.
                128.
            } else {
                path.last()
                    .map_or(1., |p| (PLAYER.y - p.y) as f32 * 64. - 24.)
            };
            for (n, target) in self.targets().iter().enumerate() {
                if !path.contains(target) {
                    continue;
                }
                let missed = self.case == Case::Miss && n == 1;
                let traversal = self.packets.iter().filter(|p| !p.missed).count();
                if traversal >= 4 {
                    break;
                }
                self.packets.push(Packet {
                    launch: 0.,
                    impact: FLIGHT * ((PLAYER.y - target.y) as f32 * 64. - 24.) / flight_pixels,
                    target: n,
                    damage: (38. * 0.75_f32.powi(traversal as i32)).floor() as u16,
                    missed,
                    resolved: false,
                });
            }
        } else {
            for sequence in 0..2 {
                for bullet in 0..self.native_projectiles() {
                    let launch = sequence as f32
                        * if self.native_projectiles() == 3 {
                            0.24
                        } else {
                            0.18
                        }
                        + bullet as f32 * 0.055;
                    self.packets.push(Packet {
                        launch,
                        impact: launch + FLIGHT,
                        target: 0,
                        damage: 20,
                        missed: self.case == Case::Miss && (sequence + bullet) % 2 == 1,
                        resolved: false,
                    });
                }
            }
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
            "cooldown":self.cooldown,"damage":self.damage,"hits":hits,"misses":self.packets.iter().filter(|p|p.resolved&&p.missed).count(),
            "blood_events":if self.case==Case::Machine{0}else{hits},"reaction_events":hits,
            "projectiles":if self.started.is_some(){self.shot_count()}else{0},
            "native_projectiles_per_sequence":self.native_projectiles(),"sequences":if self.skill==1{2}else{1},
            "preview":self.started.is_none()&&self.reason().is_none(),"refusal":if self.started.is_none(){self.reason()}else{None},
            "maximum_target_hp_fixture":240,"rank_fixture":11})
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
fn end(s: &State, rect: Rect) -> Vec2 {
    if s.case == Case::Wall {
        let d = axis(s, rect);
        let offset = vec2(0., -8.);
        point(GridPos::new(4, 4), s.direction, rect) - d * 32. + offset - d * offset.dot(d)
    } else {
        point(*s.path().last().unwrap(), s.direction, rect) + vec2(0., -8.)
    }
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
            preview(&s.path(), s.direction, rect, s.clock);
        }
        let from = origin(s, rect);
        let direction = axis(s, rect);
        let age = s.started.map_or(-1., |t| s.clock - t);
        let shot_times = if s.skill == 0 {
            vec![0.]
        } else {
            s.packets.iter().map(|p| p.launch).collect()
        };
        let recoil = shot_times
            .iter()
            .map(|t| age - t)
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
            let p = point(*at, s.direction, rect);
            let last = s
                .packets
                .iter()
                .filter(|v| v.target == n && v.resolved && !v.missed)
                .map(|v| v.impact)
                .reduce(f32::max);
            let offset = last.map_or(Vec2::ZERO, |t| {
                crate::combat_visuals::hit_reaction_offset(
                    p,
                    point(PLAYER, s.direction, rect),
                    f64::from(age - t),
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
            } else if s.case == Case::Ally && s.skill == 0 && n == 1 {
                actor(
                    p + offset,
                    game_art::wear(
                        game_art::player(
                            Some("core:technomancien"),
                            PlayerWeaponPose::Unarmed,
                            None,
                        ),
                        [("core:body_armor".into(), "core:tenue_technique".into())],
                    ),
                );
                draw_text_bold("Allié", p.x + 24., p.y, 16., Color::new(0.5, 0.85, 1., 1.));
            } else {
                enemy(p + offset);
            }
        }
        if s.started.is_some() {
            for launch in shot_times {
                let a = age - launch;
                let missed =
                    s.skill == 1 && s.packets.iter().any(|p| p.launch == launch && p.missed);
                let target = end(s, rect)
                    + if missed {
                        vec2(-direction.y, direction.x) * 26.
                    } else {
                        Vec2::ZERO
                    };
                tracer(from, target, a, s.skill == 0);
            }
            if s.case == Case::Wall && s.skill == 0 {
                spark(end(s, rect), age - FLIGHT, -direction);
            }
        }
        for packet in s.packets.iter().filter(|p| p.resolved) {
            let a = age - packet.impact;
            let p = point(s.targets()[packet.target], s.direction, rect);
            if !packet.missed {
                if s.case == Case::Machine {
                    spark(p + vec2(0., -8.), a, -direction);
                } else if (0.0..0.30).contains(&a) {
                    let frame = ((a / 0.30 * 6.) as usize).min(5);
                    draw_texture_ex(
                        &self.blood,
                        p.x - 29.,
                        p.y - 25.,
                        WHITE,
                        DrawTextureParams {
                            source: Some(Rect::new(frame as f32 * 256., 0., 256., 256.)),
                            dest_size: Some(vec2(58., 58.)),
                            ..Default::default()
                        },
                    );
                }
            }
        }
        // One readable accumulated value per victim, rather than stacked burst numbers.
        for (n, at) in s.targets().iter().enumerate() {
            let Some(packet) = s.packets.iter().rev().find(|p| p.target == n && p.resolved) else {
                continue;
            };
            let a = age - packet.impact;
            let p = point(*at, s.direction, rect);
            if (0.0..0.65).contains(&a) {
                let text = if packet.missed {
                    "Manqué".to_string()
                } else {
                    format!("-{}", s.damage[n])
                };
                let x = p.x - measure_text(&text, None, 22, 1.).width / 2.;
                let y = p.y - 40. - a * 20.;
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
        (Case::Ally, 2, "ALLIE"),
        (Case::Cap, 2, "MAXIMUM"),
        (Case::Burst, 2, "RAFALE"),
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
            draw_text_bold(format!("{} · {}", 109 + i, TITLES[i]), x, 32., 23., TEXT);
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
            entries.push(serde_json::json!({"id":IDS[skill],"number":109+skill,"variant":format!("{}-A01-{label}",IDS[skill]),
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
    fn ranged_opening_piercing_stops_at_wall_and_target_limit() {
        let mut wall = State::new(0, Case::Wall, 2);
        finish(&mut wall);
        assert_eq!(wall.damage, [38, 0, 0, 0, 0]);
        let mut limit = State::new(0, Case::Cap, 2);
        finish(&mut limit);
        assert_eq!(limit.damage, [38, 28, 21, 16, 0]);
        let mut ally = State::new(0, Case::Ally, 2);
        finish(&mut ally);
        assert_eq!(ally.damage[1], 28);
        let mut miss = State::new(0, Case::Miss, 2);
        finish(&mut miss);
        assert_eq!(miss.damage, [38, 0, 28, 0, 0]);
    }
    #[test]
    fn ranged_opening_two_sequences_respect_native_mode_and_do_not_repeat() {
        for (case, count) in [(Case::Open, 2), (Case::Burst, 6)] {
            let mut s = State::new(1, case, 2);
            finish(&mut s);
            assert_eq!(s.damage[0], count * 20);
            assert_eq!(s.ammo, 20 - count);
            assert_eq!(s.energy, 87);
            for _ in 0..8 {
                s.advance();
                s.clock += 5.;
                s.resolve();
                s.start();
            }
            assert_eq!(s.damage[0], count * 20);
            assert_eq!(s.ammo, 20 - count);
            assert_eq!(s.cooldown, 0);
        }
    }
    #[test]
    fn ranged_opening_refusals_and_machine_feedback_stay_separate() {
        for skill in 0..2 {
            for case in [Case::NoAmmo, Case::Energy, Case::Melee] {
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
        let mut machine = State::new(0, Case::Machine, 2);
        finish(&mut machine);
        assert_eq!(machine.snapshot()["blood_events"], 0);
        assert_eq!(machine.snapshot()["reaction_events"], 3);
    }
}
