//! ANIM-S51: prepared physical shots; no campaign combat, RNG or weapon mutation.
use super::super::melee::actor;
use super::zones::{position, preview, terrain};
use super::*;
use project_rl::skills::spatial::in_cast_range;
use project_rl::world::{Map, has_line_of_sight};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const IDS: [&str; 2] = ["DI05", "DI07"];
const TITLES: [&str; 2] = ["Tir de harpon", "Tir vampirique"];
const PLAYER: GridPos = GridPos::new(4, 7);
const FPS: usize = 30;
const FRAMES: usize = 120;
const START: usize = 18;
const TURNS: [usize; 8] = [56, 64, 72, 80, 88, 96, 104, 112];
const FLIGHT: f32 = 0.18;
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#...#...#\n#.......#\n#.......#\n#.......#\n#########",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Case {
    Open,
    Anchor,
    Elite,
    Reduced,
    Missing,
    Wall,
    Miss,
    NoAmmo,
    Energy,
    Melee,
    Quiet,
}
impl Case {
    fn label(self) -> &'static str {
        match self {
            Self::Open => "TIRS",
            Self::Anchor => "ANCRE",
            Self::Elite => "ELITE",
            Self::Reduced => "REDUIT",
            Self::Missing => "PV_MANQUANTS",
            Self::Wall => "CLOISON",
            Self::Miss => "MANQUE",
            Self::NoAmmo => "MUNITIONS",
            Self::Energy => "ENERGIE",
            Self::Melee => "MELEE",
            Self::Quiet => "ATTENTE",
        }
    }
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Tirs",
            Self::Anchor => "Cible ancrée : traction impossible",
            Self::Elite => "Élite : traction résistée, résultat préparé",
            Self::Reduced => "Impact réduit, exemple après défense",
            Self::Missing => "Joueur presque soigné",
            Self::Wall => "Cloison isolée",
            Self::Miss => "Tir manqué, résultat préparé",
            Self::NoAmmo => "Sans munitions",
            Self::Energy => "Énergie insuffisante",
            Self::Melee => "Arme de mêlée active",
            Self::Quiet => "Attente sans autre tour",
        }
    }
}
const TARGET: GridPos = GridPos::new(4, 2);
const PULL_TIME: f32 = 0.22;
fn pull_destination(
    map: &Map,
    mut at: GridPos,
    player: GridPos,
    occupied: &[GridPos],
    maximum: u16,
) -> GridPos {
    // Straight prepared alignment only. Diagonal paths and tie policies remain
    // outside this visual fixture; this never establishes a general pull rule.
    for _ in 0..maximum {
        let next = GridPos::new(at.x, at.y + 1);
        if next == player || !map.is_walkable(next) || occupied.contains(&next) {
            break;
        }
        at = next;
    }
    at
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    started: Option<f32>,
    resolved: bool,
    turn: u16,
    energy: u16,
    ammo: u16,
    cooldown: u16,
    damage: [u16; 3],
    target: GridPos,
    hp: u16,
    healed: u16,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.,
            started: None,
            resolved: false,
            turn: 0,
            energy: if case == Case::Energy { 5 } else { 100 },
            ammo: if case == Case::NoAmmo { 0 } else { 20 },
            cooldown: 0,
            damage: [0; 3],
            target: TARGET,
            hp: if case == Case::Missing { 98 } else { 40 },
            healed: 0,
        }
    }
    fn map(&self) -> Map {
        Map::from_ascii(MAPS[usize::from(self.case == Case::Wall)]).unwrap()
    }
    fn targets(&self) -> [GridPos; 3] {
        [TARGET, GridPos::new(2, 3), GridPos::new(6, 3)]
    }
    fn weapon_range(&self) -> u16 {
        6 + parameter_for(IDS[self.skill], "weapon_range_bonus_cells", 11)
    }
    fn reason(&self) -> Option<&'static str> {
        if self.case == Case::Melee {
            Some("Nécessite une arme à distance active.")
        } else if self.ammo < 1 {
            Some("Munitions insuffisantes.")
        } else if self.energy < parameter_for(IDS[self.skill], "energy", 11) {
            Some("Énergie insuffisante.")
        } else if !has_line_of_sight(&self.map(), PLAYER, TARGET, true)
            || !in_cast_range(PLAYER, TARGET, self.weapon_range())
        {
            Some("La cloison masque la cible.")
        } else {
            None
        }
    }
    fn start(&mut self) {
        if self.started.is_some() || self.reason().is_some() {
            return;
        }
        self.started = Some(self.clock);
        self.turn = 1;
        self.ammo -= 1;
        self.energy -= parameter_for(IDS[self.skill], "energy", 11);
        self.cooldown = parameter_for(IDS[self.skill], "cooldown_turns", 11);
    }
    fn resolve(&mut self) {
        if self.resolved
            || self
                .started
                .is_none_or(|start| self.clock - start + 0.00001 < FLIGHT)
        {
            return;
        }
        self.resolved = true;
        if self.case == Case::Miss {
            return;
        }
        // Exact prepared post-defence impacts, not a new defence or variance law.
        self.damage[0] = if self.case == Case::Reduced {
            12
        } else if self.skill == 0 {
            31
        } else {
            36
        };
        if self.skill == 0 && !matches!(self.case, Case::Anchor | Case::Elite) {
            self.target = pull_destination(
                &self.map(),
                TARGET,
                PLAYER,
                &self.targets()[1..],
                parameter_for("DI05", "maximum_pull_cells", 11).min(3),
            );
        }
        if self.skill == 1 {
            // All fixtures use even actual HP losses and exactly 100 maxHP.
            // They do not arbitrate integer rounding or a full-health cast policy.
            self.healed = (self.damage[0] / 2).min(10).min(100 - self.hp);
            self.hp += self.healed;
        }
    }
    fn advance(&mut self) {
        if self.started.is_none() || self.case == Case::Quiet {
            return;
        }
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
    }
    fn target_point(&self, rect: Rect) -> Vec2 {
        let from = point(TARGET, self.direction, rect);
        let to = point(self.target, self.direction, rect);
        let t = self.started.map_or(0., |start| {
            ((self.clock - start - FLIGHT) / PULL_TIME).clamp(0., 1.)
        });
        from.lerp(to, 1. - (1. - t).powi(2))
    }
    fn snapshot(&self) -> serde_json::Value {
        let hit = self.resolved && self.case != Case::Miss;
        serde_json::json!({"started":self.started.is_some(),"turn":self.turn,"energy":self.energy,"ammo":self.ammo,
            "cooldown":self.cooldown,"damage":self.damage,"hits":u8::from(hit),
            "misses":u8::from(self.resolved&&self.case==Case::Miss),"blood_events":u8::from(hit),"reaction_events":u8::from(hit),
            "projectiles":u8::from(self.started.is_some()),"player_hp":self.hp,"player_max_hp_fixture":100,"healed":self.healed,
            "target_cell":[self.target.x,self.target.y],"pulled_cells":self.target.y-TARGET.y,
            "preview":self.started.is_none()&&self.reason().is_none(),
            "refusal":if self.started.is_none(){self.reason()}else{None},
            "maximum_target_hp_fixture":240,"rank_fixture":11,"weapon_range_fixture":self.weapon_range(),
            "persistent_status_created":false,"critical_rules_modeled":false})
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

fn cable(from: Vec2, to: Vec2, opacity: f32) {
    let mid = (from + to) / 2. + vec2(0., 2.);
    for pts in [[from, mid], [mid, to]] {
        draw_line(
            pts[0].x,
            pts[0].y,
            pts[1].x,
            pts[1].y,
            5.,
            alpha(INK, opacity),
        );
        draw_line(
            pts[0].x,
            pts[0].y,
            pts[1].x,
            pts[1].y,
            2.3,
            alpha(Color::new(0.55, 0.62, 0.64, 1.), opacity),
        );
        draw_line(
            pts[0].x,
            pts[0].y - 0.7,
            pts[1].x,
            pts[1].y - 0.7,
            0.8,
            alpha(Color::new(0.92, 0.94, 0.92, 1.), opacity),
        );
    }
}
fn harpoon(from: Vec2, to: Vec2, t: f32, attached: Option<Vec2>) {
    if t < 0. || t > FLIGHT + PULL_TIME + 0.10 {
        return;
    }
    let d = (to - from).normalize_or_zero();
    let side = vec2(-d.y, d.x);
    let head = if let Some(p) = attached {
        p
    } else {
        from.lerp(to, (t / FLIGHT).min(1.))
    };
    let opacity = if t > FLIGHT + PULL_TIME {
        1. - (t - FLIGHT - PULL_TIME) / 0.10
    } else {
        1.
    };
    cable(from, head, opacity);
    let tail = head - d * 18.;
    draw_triangle(
        head + d * 4.,
        tail + side * 5.,
        tail - side * 5.,
        alpha(INK, opacity),
    );
    draw_triangle(
        head + d * 2.,
        tail + side * 3.,
        tail - side * 3.,
        alpha(Color::new(0.86, 0.90, 0.89, 1.), opacity),
    );
    for sign in [-1., 1.] {
        let a = tail - side * sign * 3.;
        let b = tail - d * 4. + side * sign * 7.;
        draw_line(
            a.x,
            a.y,
            b.x,
            b.y,
            2.,
            alpha(Color::new(0.69, 0.75, 0.74, 1.), opacity),
        );
    }
}
struct Art {
    blood: Texture2D,
    heal: Texture2D,
}
impl Art {
    fn load() -> Self {
        fn decode(bytes: &[u8]) -> Texture2D {
            let t = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        }
        Self {
            blood: decode(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-40/blood-atlas.png"
            )),
            heal: decode(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-77/regeneration-source.png"
            )),
        }
    }
    fn scene(&self, s: &State, rect: Rect) {
        terrain(&s.map(), s.direction, rect, false);
        if s.started.is_none() && s.reason().is_none() {
            preview(&[TARGET], s.direction, rect, s.clock);
        }
        let age = s.started.map_or(-1., |t| s.clock - t);
        let impact = age - FLIGHT;
        let player = point(PLAYER, s.direction, rect);
        if s.healed > 0 && (0.0..0.55).contains(&impact) {
            let opacity = (std::f32::consts::PI * (impact / 0.55)).sin().max(0.);
            draw_texture_ex(
                &self.heal,
                player.x - 26.,
                player.y - 40.,
                alpha(WHITE, opacity),
                DrawTextureParams {
                    dest_size: Some(vec2(52., 78.)),
                    ..Default::default()
                },
            );
        }
        let direction = axis(s, rect);
        let recoil = if (0.0..0.12).contains(&age) {
            3. * (age / 0.12 * std::f32::consts::PI).sin()
        } else {
            0.
        };
        actor(
            player - direction * recoil,
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
            if !has_line_of_sight(&s.map(), PLAYER, *at, true) {
                continue;
            }
            let mut p = if n == 0 {
                s.target_point(rect)
            } else {
                point(*at, s.direction, rect)
            };
            if n == 0 && s.damage[0] > 0 {
                p += crate::combat_visuals::hit_reaction_offset(p, player, f64::from(impact), 64.);
            }
            enemy(p);
        }
        if s.started.is_some() {
            let from = origin(s, rect);
            let original = point(TARGET, s.direction, rect) + vec2(0., -8.);
            let target = s.target_point(rect) + vec2(0., -8.);
            let missed = if s.case == Case::Miss {
                vec2(-direction.y, direction.x) * 26.
            } else {
                Vec2::ZERO
            };
            if s.skill == 0 {
                // A miss has no anchor or traction; its free cable quickly retracts.
                if s.case == Case::Miss {
                    let retract = if age >= FLIGHT {
                        Some(
                            (original + missed)
                                .lerp(from, ((age - FLIGHT) / PULL_TIME).clamp(0., 1.)),
                        )
                    } else {
                        None
                    };
                    harpoon(from, original + missed, age, retract);
                } else {
                    harpoon(
                        from,
                        original,
                        age,
                        if s.resolved { Some(target) } else { None },
                    );
                }
            } else {
                tracer(from, original + missed, age, false);
            }
            if s.damage[0] > 0 && (0.0..0.30).contains(&impact) {
                let frame = ((impact / 0.30 * 6.) as usize).min(5);
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
            if s.resolved && (0.0..0.65).contains(&impact) {
                let life = 1. - impact / 0.65;
                let text = if s.case == Case::Miss {
                    "Manqué".to_string()
                } else {
                    format!("-{}", s.damage[0])
                };
                let p = target + vec2(0., -32. - impact * 20.);
                floating(
                    &text,
                    p,
                    alpha(
                        if s.case == Case::Miss {
                            TEXT
                        } else {
                            Color::new(1., 0.89, 0.73, 1.)
                        },
                        life,
                    ),
                );
                if s.healed > 0 {
                    floating(
                        &format!("+{} PV", s.healed),
                        player + vec2(0., -44. - impact * 18.),
                        alpha(Color::new(0.61, 1., 0.42, 1.), life),
                    );
                }
            }
        }
    }
}
fn floating(text: &str, p: Vec2, c: Color) {
    let x = p.x - measure_text(text, None, 22, 1.).width / 2.;
    draw_text_bold(text, x + 1., p.y + 1., 22., alpha(INK, c.a));
    draw_text_bold(text, x, p.y, 22., c);
}
fn cases() -> Vec<(Case, usize, &'static str)> {
    let mut out = vec![
        (Case::Open, 2, "HAUT"),
        (Case::Open, 0, "DROITE"),
        (Case::Open, 1, "GAUCHE"),
        (Case::Open, 3, "BAS"),
    ];
    out.extend([
        (Case::Anchor, 2, "ANCRE"),
        (Case::Elite, 2, "ELITE"),
        (Case::Reduced, 2, "REDUIT"),
        (Case::Missing, 2, "PV_MANQUANTS"),
        (Case::Wall, 2, "CLOISON"),
        (Case::Miss, 2, "MANQUE"),
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
            draw_text_bold(format!("{} · {}", 113 + i, TITLES[i]), x, 32., 23., TEXT);
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
            entries.push(serde_json::json!({"id":IDS[skill],"number":113+skill,"variant":format!("{}-A01-{label}",IDS[skill]),
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
    fn ranged_harpoon_vampiric_pull_stops_at_occupied_wall_and_player() {
        let map = Map::from_ascii(MAPS[0]).unwrap();
        assert_eq!(
            pull_destination(&map, TARGET, PLAYER, &[], 3),
            GridPos::new(4, 5)
        );
        assert_eq!(
            pull_destination(&map, TARGET, PLAYER, &[GridPos::new(4, 4)], 3),
            GridPos::new(4, 3)
        );
        assert_eq!(
            pull_destination(&Map::from_ascii(MAPS[1]).unwrap(), TARGET, PLAYER, &[], 3),
            GridPos::new(4, 3)
        );
        assert_eq!(
            pull_destination(&map, GridPos::new(4, 6), PLAYER, &[], 3),
            GridPos::new(4, 6)
        );
        let mut normal = State::new(0, Case::Open, 2);
        finish(&mut normal);
        assert_eq!(normal.target, GridPos::new(4, 5));
        assert_eq!(normal.damage, [31, 0, 0]);
        for case in [Case::Anchor, Case::Elite] {
            let mut s = State::new(0, case, 2);
            finish(&mut s);
            assert_eq!(s.target, TARGET);
            assert_eq!(s.damage[0], 31);
        }
    }
    #[test]
    fn ranged_harpoon_vampiric_healing_obeys_actual_loss_cap_and_missing_hp() {
        for (case, loss, heal, hp) in [
            (Case::Open, 36, 10, 50),
            (Case::Reduced, 12, 6, 46),
            (Case::Missing, 36, 2, 100),
            (Case::Miss, 0, 0, 40),
        ] {
            let mut s = State::new(1, case, 2);
            finish(&mut s);
            assert_eq!(s.damage[0], loss);
            assert_eq!((s.healed, s.hp), (heal, hp));
            assert_eq!(s.target, TARGET);
            let before = s.snapshot();
            s.clock += 6.;
            s.resolve();
            assert_eq!(s.snapshot(), before);
        }
    }
    #[test]
    fn ranged_harpoon_vampiric_refusals_and_later_turns_do_not_repeat() {
        for skill in 0..2 {
            for case in [Case::Wall, Case::NoAmmo, Case::Energy, Case::Melee] {
                let mut s = State::new(skill, case, 2);
                let before = s.snapshot();
                finish(&mut s);
                assert_eq!(s.snapshot(), before);
            }
            let mut s = State::new(skill, Case::Open, 2);
            finish(&mut s);
            let expected = (s.damage, s.hp, s.target);
            assert_eq!(s.ammo, 19);
            assert_eq!(s.energy, if skill == 0 { 87 } else { 88 });
            for _ in 0..8 {
                s.advance();
                s.clock += 4.;
                s.resolve();
                s.start();
            }
            assert_eq!((s.damage, s.hp, s.target), expected);
            assert_eq!(s.ammo, 19);
            assert_eq!(s.cooldown, 0);
        }
    }
}
