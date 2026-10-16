//! ANIM-S35: prepared defensive-buff scenes, pending visual review, no campaign commands.
use super::super::melee::{actor, cut, player_visual};
use super::zones::{position, preview, terrain};
use super::*;
use project_rl::world::Map;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const IDS: [&str; 2] = ["B02", "SU03"];
const TITLES: [&str; 2] = ["Égide occulte", "Barrière d’épine"];
const MAP: &str = "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########";
const PLAYER: GridPos = GridPos::new(4, 4);
const FRAMES: usize = 120;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Case {
    Melee,
    Ranged,
    Persistent,
    Quiet,
    Silence,
}
impl Case {
    fn label(self) -> &'static str {
        match self {
            Self::Melee => "MELEE",
            Self::Ranged => "DISTANCE",
            Self::Persistent => "PERSISTANT",
            Self::Quiet => "ATTENTE",
            Self::Silence => "SILENCE",
        }
    }
    fn title(self) -> &'static str {
        match self {
            Self::Melee => "Coup de mêlée",
            Self::Ranged => "Tir direct",
            Self::Persistent => "Dégât persistant",
            Self::Quiet => "Attendre sans passer de tour",
            Self::Silence => "Lancement sous Silence",
        }
    }
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    cast_at: Option<f32>,
    hit_at: Option<f32>,
    turn: u16,
    remaining: u16,
    cooldown: u16,
    energy: u16,
    hp: u16,
    enemy_hp: u16,
    damage: u16,
    reflected: u16,
    protected_hit: bool,
    hits: u16,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.,
            cast_at: None,
            hit_at: None,
            turn: 0,
            remaining: 0,
            cooldown: 0,
            energy: 100,
            hp: 100,
            enemy_hp: 100,
            damage: 0,
            reflected: 0,
            protected_hit: false,
            hits: 0,
        }
    }
    fn parameter(&self, key: &str) -> u16 {
        super::super::parameter_for(IDS[self.skill], key, 11)
    }
    fn can_cast(&self) -> bool {
        self.case != Case::Silence && self.cooldown == 0 && self.energy >= self.parameter("energy")
    }
    fn cast(&mut self) {
        if !self.can_cast() {
            return;
        }
        self.energy -= self.parameter("energy");
        self.remaining = self.parameter("duration_turns");
        self.cooldown = self.parameter("cooldown_turns");
        self.cast_at = Some(self.clock);
    }
    fn receive(&mut self) {
        if self.cast_at.is_none() {
            return;
        }
        // Exact whole-number examples, not new weapon/monster stats or rounding rules.
        let incoming = if self.case == Case::Persistent {
            10
        } else {
            20
        };
        self.protected_hit = self.remaining > 0
            && if self.skill == 0 {
                self.case != Case::Persistent
            } else {
                matches!(self.case, Case::Melee | Case::Quiet)
            };
        self.damage = if self.skill == 0 && self.protected_hit {
            incoming * (100 - self.parameter("direct_damage_reduction_percent")) / 100
        } else {
            incoming
        };
        self.reflected = if self.skill == 1
            && self.remaining > 0
            && matches!(self.case, Case::Melee | Case::Quiet)
        {
            self.damage * self.parameter("reflected_direct_melee_damage_percent") / 100
        } else {
            0
        };
        self.hp = self.hp.saturating_sub(self.damage);
        self.enemy_hp = self.enemy_hp.saturating_sub(self.reflected);
        self.hit_at = Some(self.clock);
        self.hits += 1;
    }
    fn advance(&mut self, turns: u16) {
        if self.cast_at.is_none() {
            return;
        }
        self.turn += turns;
        self.remaining = self.remaining.saturating_sub(turns);
        self.cooldown = self.cooldown.saturating_sub(turns);
    }
    fn enemy_at(&self) -> GridPos {
        GridPos::new(4, if self.case == Case::Ranged { 1 } else { 3 })
    }
    fn at(&self, p: GridPos, rect: Rect) -> Vec2 {
        let p = position(p, self.direction);
        vec2(
            rect.x + (p.x as f32 + 0.5) * 64.,
            rect.y + (p.y as f32 + 0.5) * 64.,
        )
    }
    fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({"turn":self.turn,"remaining":self.remaining,"cooldown":self.cooldown,"energy":self.energy,"hp":self.hp,"enemy_hp":self.enemy_hp,"damage":self.damage,"reflected":self.reflected,"hits":self.hits,"preview":self.cast_at.is_none()&&self.can_cast(),"icon_visible":self.remaining>0,"protected_hit":self.protected_hit})
    }
}

struct Art {
    effects: [Texture2D; 2],
    icons: [Texture2D; 2],
    blood: Texture2D,
}
impl Art {
    fn load() -> Self {
        fn load(b: &[u8]) -> Texture2D {
            let t = Texture2D::from_file_with_format(b, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        }
        Self {
            effects: [
                load(include_bytes!(
                    "../../../assets/animations/agent/skill-animation-review-76/egide-source.png"
                )),
                load(include_bytes!(
                    "../../../assets/animations/agent/skill-animation-review-76/epines-source.png"
                )),
            ],
            icons: [
                load(include_bytes!(
                    "../../../assets/skills/agent/B02-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/SU03-compact.png"
                )),
            ],
            blood: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-40/blood-atlas.png"
            )),
        }
    }
    fn ring(&self, skill: usize, p: Vec2, size: f32, opacity: f32, angle: f32) {
        draw_texture_ex(
            &self.effects[skill],
            p.x - size / 2.,
            p.y - size / 2.,
            alpha(WHITE, opacity),
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                rotation: angle,
                ..Default::default()
            },
        );
    }
    fn blood(&self, p: Vec2, age: f32, angle: f32) {
        if !(0.02..0.31).contains(&age) {
            return;
        }
        let frame = ((age - 0.02) / 0.29 * 6.).min(5.) as u16;
        draw_texture_ex(
            &self.blood,
            p.x - 28.,
            p.y - 28.,
            alpha(WHITE, 1. - age / 0.35),
            DrawTextureParams {
                source: Some(Rect::new(f32::from(frame) * 256., 0., 256., 256.)),
                dest_size: Some(vec2(56., 56.)),
                rotation: angle,
                ..Default::default()
            },
        );
    }
}
fn physical_icon(p: Vec2, color: Color) {
    draw_line(p.x - 4., p.y + 5., p.x + 4., p.y - 5., 3., color);
    draw_line(p.x - 5., p.y + 1., p.x + 1., p.y + 5., 2., color);
}
pub(super) fn damage_label(p: Vec2, amount: u16, age: f32, poison: bool) {
    if !(0.0..0.9).contains(&age) {
        return;
    }
    let opacity = ((0.9 - age) / 0.25).clamp(0., 1.);
    let color = if poison { GREEN } else { TEXT };
    let y = p.y - 43. - age * 14.;
    let total = amount.to_string();
    let detail = format!("({amount}");
    let total_width = crate::ui_theme::measure_text_bold(&total, 22).width;
    let detail_width = crate::ui_theme::measure_text(&detail, None, 16, 1.).width;
    let width = total_width + 8. + detail_width + 24.;
    let x = p.x - width / 2.;
    let detail_x = x + total_width + 8.;
    let icon_x = detail_x + detail_width + 8.;
    draw_rectangle(x - 6., y - 20., width + 12., 30., alpha(INK, opacity * 0.9));
    draw_text_bold(total, x, y + 1., 22., alpha(color, opacity));
    draw_text(detail, detail_x, y, 16., alpha(color, opacity));
    if poison {
        draw_circle(icon_x, y - 6., 4., alpha(color, opacity));
    } else {
        physical_icon(vec2(icon_x, y - 6.), alpha(color, opacity));
    }
    draw_text(")", icon_x + 7., y, 16., alpha(color, opacity));
}
fn scene(art: &Art, s: &State, rect: Rect, interactive: bool) {
    let map = Map::from_ascii(MAP).unwrap();
    terrain(&map, s.direction, rect, false);
    if s.cast_at.is_none() && s.can_cast() {
        preview(&[PLAYER], s.direction, rect, s.clock);
    }
    terrain(&map, s.direction, rect, true);
    let p = s.at(PLAYER, rect);
    let e = s.at(s.enemy_at(), rect);
    let d = (p - e).normalize();
    let cast_age = s.cast_at.map(|t| s.clock - t);
    if let Some(a) = cast_age.filter(|a| *a < 0.34) {
        let u = (a / 0.34).clamp(0., 1.);
        art.ring(s.skill, p, 96. + 25. * u, (1. - u).powf(0.6), u * 0.16);
    }
    let age = s.hit_at.map(|t| s.clock - t);
    let recoil = age.map_or(Vec2::ZERO, |a| {
        crate::combat_visuals::hit_reaction_offset(p, e, f64::from(a), 64.)
    });
    let return_recoil = age.filter(|_| s.reflected > 0).map_or(Vec2::ZERO, |a| {
        crate::combat_visuals::hit_reaction_offset(e, p, f64::from(a - 0.05), 64.)
    });
    let lunge = age
        .filter(|a| (0.0..0.18).contains(a))
        .map_or(0., |a| (a / 0.18 * std::f32::consts::PI).sin() * 7.);
    actor(p + recoil, player_visual());
    enemy(
        e + return_recoil
            + if s.case == Case::Ranged || s.case == Case::Persistent {
                Vec2::ZERO
            } else {
                d * lunge
            },
    );
    if let Some(a) = age {
        if s.case == Case::Ranged && a < 0.13 {
            cut(e, p, 2.8, 1. - a / 0.13);
        } else if s.case != Case::Persistent && a < 0.18 {
            let n = vec2(-d.y, d.x);
            cut(p - n * 28., p + n * 24. + d * 8., 7., 1. - a / 0.18);
        }
        if s.protected_hit && s.remaining > 0 && a < 0.42 {
            let u = a / 0.42;
            if s.skill == 0 {
                art.ring(
                    0,
                    p + recoil,
                    112. + 12. * u,
                    (1. - u).powf(0.65),
                    -u * 0.18,
                );
            } else if s.reflected > 0 {
                art.ring(1, p + recoil, 88. + 70. * u, (1. - u).powf(0.6), u * 0.12);
            }
        }
        if s.case != Case::Persistent {
            art.blood(p + recoil, a, d.y.atan2(d.x));
        }
        if s.reflected > 0 {
            art.blood(e + return_recoil, a - 0.06, (-d).y.atan2((-d).x));
        }
        damage_label(p + recoil, s.damage, a, s.case == Case::Persistent);
        if s.reflected > 0 {
            damage_label(
                e + return_recoil + vec2(0., -30.),
                s.reflected,
                a - 0.07,
                false,
            );
        }
    }
    if let Some(a) = cast_age.filter(|a| *a < 0.35) {
        draw_text_bold(
            TITLES[s.skill],
            p.x - 66.,
            p.y - 76. - a * 12.,
            15.,
            alpha(TEXT, 1. - a / 0.35),
        );
    }
    // The persistent indication belongs to the state bar; cast/impact art is brief.
    if s.remaining > 0 {
        let badge = Rect::new(rect.x + 484., rect.y + 16., 42., 42.);
        draw_rectangle(badge.x, badge.y, badge.w, badge.h, alpha(INK, 0.95));
        draw_rectangle_lines(badge.x, badge.y, badge.w, badge.h, 1., DIM);
        draw_texture_ex(
            &art.icons[s.skill],
            badge.x + 2.,
            badge.y + 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(38., 38.)),
                ..Default::default()
            },
        );
        draw_rectangle(badge.right() - 15., badge.bottom() - 16., 16., 16., INK);
        draw_text_bold(
            s.remaining.to_string(),
            badge.right() - 12.,
            badge.bottom() - 3.,
            15.,
            TEXT,
        );
        if interactive && badge.contains(mouse_position().into()) {
            let x = rect.x + 270.;
            let y = rect.y + 66.;
            draw_rectangle(x, y, 288., 81., alpha(INK, 0.98));
            draw_text_bold(TITLES[s.skill], x + 9., y + 22., 17., TEXT);
            draw_text(
                if s.skill == 0 {
                    "Dégâts directs reçus : −55 %"
                } else {
                    "Dégâts de mêlée renvoyés : 40 %"
                },
                x + 9.,
                y + 45.,
                15.,
                TEXT,
            );
            draw_text(
                format!("Durée restante : {} tours", s.remaining),
                x + 9.,
                y + 67.,
                14.,
                TEXT,
            );
        }
    }
    draw_rectangle(
        rect.x + 8.,
        rect.bottom() - 56.,
        560.,
        48.,
        alpha(INK, 0.94),
    );
    draw_text(
        format!(
            "PV {} / 100 · cible {} / 100 · énergie {}",
            s.hp, s.enemy_hp, s.energy
        ),
        rect.x + 17.,
        rect.bottom() - 35.,
        16.,
        TEXT,
    );
    draw_text(
        format!(
            "Tour {} · effet {} tours · recharge {}",
            s.turn, s.remaining, s.cooldown
        ),
        rect.x + 17.,
        rect.bottom() - 16.,
        15.,
        DIM,
    );
}

pub(crate) async fn run() {
    request_new_screen_size(1280., 820.);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let mut s = State::new(0, Case::Melee, 2);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Protections · ANIM-S35", 20., 34., 25., TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(Rect::new(20., 88. + i as f32 * 40., 330., 33.), title, true) {
                s = State::new(i, s.case, s.direction);
            }
        }
        for (i, title) in ["DROITE", "GAUCHE", "HAUT", "BAS"].iter().enumerate() {
            if button(
                Rect::new(
                    20. + (i % 2) as f32 * 166.,
                    185. + (i / 2) as f32 * 37.,
                    156.,
                    31.,
                ),
                title,
                true,
            ) {
                s = State::new(s.skill, s.case, i);
            }
        }
        for (i, case) in [
            Case::Melee,
            Case::Ranged,
            Case::Persistent,
            Case::Quiet,
            Case::Silence,
        ]
        .into_iter()
        .enumerate()
        {
            if button(
                Rect::new(20., 280. + i as f32 * 36., 330., 30.),
                case.title(),
                true,
            ) {
                s = State::new(s.skill, case, s.direction);
            }
        }
        if button(Rect::new(20., 485., 330., 35.), "Lancer", s.can_cast())
            || is_key_pressed(KeyCode::Enter)
        {
            s.cast();
        }
        if button(
            Rect::new(20., 532., 330., 35.),
            "Recevoir le coup préparé",
            s.cast_at.is_some(),
        ) || is_key_pressed(KeyCode::A)
        {
            s.receive();
        }
        if button(
            Rect::new(20., 579., 330., 35.),
            "+1 tour",
            s.cast_at.is_some(),
        ) || is_key_pressed(KeyCode::Space)
        {
            s.advance(1);
        }
        if button(Rect::new(20., 626., 330., 35.), "Recommencer", true)
            || is_key_pressed(KeyCode::R)
        {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(20., 673., 330., 35.),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
        }
        scene(&art, &s, Rect::new(410., 88., 576., 576.), true);
        draw_text(
            "Candidats 79–80 · rang effectif 11 · effets à valider",
            410.,
            701.,
            15.,
            TEXT,
        );
        draw_text(
            "Exemples de dégâts préparés ; aucun équilibrage ajouté.",
            410.,
            725.,
            14.,
            DIM,
        );
        next_frame().await;
    }
}

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    let returns_only = std::env::args().any(|arg| arg == "--capture-protection-retours");
    if output.exists() && !returns_only {
        return Err("Dossier temporaire déjà présent.".into());
    }
    if output
        .parent()
        .is_some_and(|p| p.join("visual-acceptance-01.json").exists())
    {
        return Err("Revue validée : préparer une nouvelle variante.".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let art = Art::load();
    let target = render_target(576, 576);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 576., 576.))
    };
    let cases = [
        (Case::Melee, 0, "DROITE"),
        (Case::Melee, 1, "GAUCHE"),
        (Case::Melee, 2, "HAUT"),
        (Case::Melee, 3, "BAS"),
        (Case::Ranged, 2, "DISTANCE"),
        (Case::Persistent, 2, "PERSISTANT"),
        (Case::Quiet, 2, "ATTENTE"),
        (Case::Silence, 2, "SILENCE"),
    ];
    let mut entries: Vec<serde_json::Value> = if returns_only {
        serde_json::from_slice(
            &std::fs::read(output.join("capture.json")).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?
    } else {
        Vec::new()
    };
    for skill in 0..2 {
        for (case, direction, label) in cases {
            if returns_only && !(skill == 1 && matches!(case, Case::Melee | Case::Quiet)) {
                continue;
            }
            let folder = format!("{}-{label}", IDS[skill]);
            let dir = output.join(&folder);
            std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
            let mut s = State::new(skill, case, direction);
            let mut timeline = Vec::new();
            let mut indices = Vec::new();
            let mut known = BTreeMap::new();
            let mut images = Vec::new();
            for frame in 0..FRAMES {
                s.clock = frame as f32 / 24.;
                if frame == 18 {
                    s.cast();
                }
                if frame == 32 {
                    s.receive();
                }
                if case != Case::Quiet {
                    match frame {
                        56 => s.advance(1),
                        64 | 72 | 80 => s.advance(1),
                        92 => s.advance(1),
                        96 => s.receive(),
                        _ => {}
                    }
                }
                timeline.push(s.snapshot());
                set_camera(&camera);
                clear_background(INK);
                scene(&art, &s, Rect::new(0., 0., 576., 576.), false);
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
                let n = if let Some(n) = known.get(&hash) {
                    *n
                } else {
                    let n = images.len();
                    images.push(im);
                    known.insert(hash, n);
                    n
                };
                indices.push(n);
                set_default_camera();
                if frame % 8 == 0 {
                    next_frame().await;
                }
            }
            let columns = 4;
            let rows = images.len().div_ceil(columns);
            let mut atlas =
                Image::gen_image_color((576 * columns) as u16, (576 * rows) as u16, INK);
            for (index, image) in images.into_iter().enumerate() {
                for y in 0..576 {
                    let start = (((index / columns) * 576 + y) * 576 * columns
                        + (index % columns) * 576)
                        * 4;
                    atlas.bytes[start..start + 576 * 4]
                        .copy_from_slice(&image.bytes[y * 576 * 4..(y + 1) * 576 * 4]);
                }
            }
            atlas.export_png(dir.join("atlas.png").to_str().unwrap());
            entries.retain(|e| e["folder"].as_str() != Some(&folder));
            entries.push(serde_json::json!({"id":IDS[skill],"number":79+skill,"title":TITLES[skill],"variant":format!("{}-A01-{label}",IDS[skill]),"folder":folder,"case":case.label(),"case_title":case.title(),"direction":direction,"label":label,"frames":FRAMES,"fps":24,"indices":indices,"unique_images":known.len(),"native_columns":columns,"timeline":timeline}));
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
