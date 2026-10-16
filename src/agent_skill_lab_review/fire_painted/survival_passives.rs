//! ANIM-S39: prepared maximum-HP and last-survival feedback; no campaign mutation.
use super::super::melee::{cut, player_visual};
use super::zones::{position, terrain};
use super::*;
use project_rl::world::Map;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const IDS: [&str; 2] = ["B03", "SU06"];
const TITLES: [&str; 2] = ["Vitalité améliorée", "Dernier sursaut"];
const PLAYER: GridPos = GridPos::new(4, 4);
const FRAMES: usize = 112;
const MAP: &str = "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########";
const GOLD: Color = Color::new(1., 0.81, 0.45, 1.);
const LIFE: Color = Color::new(0.82, 0.18, 0.16, 1.);
#[derive(Clone, Copy, PartialEq, Eq)]
enum Case {
    Direct,
    Quiet,
    Poison,
    Unavailable,
    Second,
}
impl Case {
    fn label(self) -> &'static str {
        match self {
            Self::Direct => "DIRECT",
            Self::Quiet => "ATTENTE",
            Self::Poison => "PERSISTANT",
            Self::Unavailable => "RECHARGE",
            Self::Second => "SECOND",
        }
    }
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    begun: Option<f32>,
    hit_at: Option<f32>,
    saved_at: Option<f32>,
    hp: u16,
    maximum: u16,
    energy: u16,
    turn: u16,
    cooldown: u16,
    saves: u16,
    hits: u16,
    damage: u16,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.,
            begun: None,
            hit_at: None,
            saved_at: None,
            hp: if skill == 0 { 60 } else { 20 },
            maximum: 100,
            energy: 100,
            turn: 0,
            cooldown: if skill == 1 && case == Case::Unavailable {
                3
            } else {
                0
            },
            saves: 0,
            hits: 0,
            damage: 0,
        }
    }
    fn begin(&mut self) {
        if self.begun.is_some() {
            return;
        }
        self.begun = Some(self.clock);
        if self.skill == 0 {
            self.maximum = 100 + parameter_for("B03", "maximum_HP_bonus_percent", 11);
        }
    }
    fn receive(&mut self, amount: u16) {
        if self.begun.is_none() || self.hp == 0 {
            return;
        }
        let ready = self.cooldown == 0;
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        let before = self.hp;
        if self.skill == 1 && self.case != Case::Poison && ready && amount >= self.hp {
            self.hp = parameter_for("SU06", "HP_after_eligible_fatal_hit", 11);
            self.cooldown = parameter_for("SU06", "cooldown_turns", 11).max(30);
            self.saved_at = Some(self.clock);
            self.saves += 1;
        } else {
            self.hp = self.hp.saturating_sub(amount);
        }
        self.damage = before - self.hp;
        self.hit_at = Some(self.clock);
        self.hits += 1;
    }
    fn first(&mut self) {
        if self.hits == 0 && (self.skill == 1 || self.case != Case::Quiet) {
            self.receive(30);
        }
    }
    fn advance(&mut self) {
        if self.begun.is_none() || self.hp == 0 {
            return;
        }
        if self.case == Case::Second && self.hits == 1 {
            self.receive(5);
            return;
        }
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
    }
    fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({"hp":self.hp,"maximum":self.maximum,"energy":self.energy,"turn":self.turn,
            "cooldown":self.cooldown,"saves":self.saves,"hits":self.hits,"damage":self.damage,
            "begun":self.begun.is_some(),"alive":self.hp>0,"healed":0,
            "persistent":self.case==Case::Poison,"aura":false,"invulnerable":false})
    }
}
struct Art {
    icons: [Texture2D; 2],
    wings: Texture2D,
    blood: Texture2D,
}
impl Art {
    async fn load() -> Self {
        let icons = [
            include_bytes!("../../../assets/skills/agent/B03-compact.png").as_slice(),
            // Approved wing source; campaign effect integration remains separate.
            include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-83/SU06-C02-64.png"
            )
            .as_slice(),
        ]
        .map(|b| Texture2D::from_file_with_format(b, Some(ImageFormat::Png)));
        let wings = Texture2D::from_file_with_format(
            include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-83/SU06-C02-192.png"
            ),
            Some(ImageFormat::Png),
        );
        wings.set_filter(FilterMode::Linear);
        let blood = Texture2D::from_file_with_format(
            include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-40/blood-atlas.png"
            ),
            Some(ImageFormat::Png),
        );
        Self {
            icons,
            wings,
            blood,
        }
    }
    fn icon(&self, id: usize, p: Vec2, size: f32, opacity: f32) {
        draw_texture_ex(
            &self.icons[id],
            p.x - size / 2.,
            p.y - size / 2.,
            alpha(WHITE, opacity),
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    }
    fn rear_wings(&self, p: Vec2, age: f32) {
        let opacity = (age / 0.07).min(1.) * ((0.8 - age) / 0.23).clamp(0., 1.);
        let pulse = (age / 0.16).min(1.) * ((0.8 - age) / 0.64).clamp(0., 1.);
        let center = p + vec2(0., -10.);
        // Soft white light and feathers are both drawn before the complete actor.
        // Small overlapping steps avoid a hard disc or an opaque white backdrop.
        for ring in 0..32 {
            let fraction = ring as f32 / 31.;
            let radius = 68. - 43. * fraction;
            draw_ellipse(
                center.x,
                center.y,
                radius,
                radius * 0.92,
                0.,
                alpha(WHITE, opacity * (0.016 + 0.024 * fraction)),
            );
        }
        let size = 84. + pulse * 8.;
        draw_texture_ex(
            &self.wings,
            center.x - size / 2.,
            center.y - size / 2.,
            alpha(WHITE, opacity),
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    }
}
fn point(at: GridPos, direction: usize, rect: Rect) -> Vec2 {
    let p = position(at, direction);
    vec2(
        rect.x + (p.x as f32 + 0.5) * 64.,
        rect.y + (p.y as f32 + 0.5) * 64.,
    )
}
fn scene(art: &Art, s: &State, rect: Rect) {
    terrain(&Map::from_ascii(MAP).unwrap(), s.direction, rect, false);
    let base = point(PLAYER, s.direction, rect);
    let e = point(GridPos::new(4, 3), s.direction, rect);
    let hit_age = s.hit_at.map(|t| s.clock - t);
    let save_age = s.saved_at.map(|t| s.clock - t);
    let source = if s.case == Case::Poison { base } else { e };
    let p = base
        + hit_age.map_or(Vec2::ZERO, |a| {
            crate::combat_visuals::hit_reaction_offset(base, source, f64::from(a), 64.)
        });
    if s.begun.is_some() && s.case != Case::Poison && (s.skill == 1 || s.case != Case::Quiet) {
        enemy(e);
    }
    if let Some(a) = save_age.filter(|a| *a < 0.8) {
        art.rear_wings(p, a);
    }
    game_art::draw_world(
        Rect::new(p.x - 32., p.y - 32., 64., 64.),
        player_visual(),
        if s.hp > 0 || hit_age.is_some_and(|a| a < 0.22) {
            1.
        } else {
            0.28
        },
    );
    if let Some(a) = hit_age.filter(|a| *a < 0.9) {
        if s.case != Case::Poison {
            if a < 0.12 {
                cut(p + vec2(-20., -13.), p + vec2(20., 13.), 5., 1. - a / 0.12);
            }
            if a < 0.29 {
                let f = (a / 0.29 * 6.).min(5.).floor();
                draw_texture_ex(
                    &art.blood,
                    p.x - 28.,
                    p.y - 28.,
                    alpha(WHITE, 1. - a / 0.33),
                    DrawTextureParams {
                        source: Some(Rect::new(f * 256., 0., 256., 256.)),
                        dest_size: Some(vec2(56., 56.)),
                        ..Default::default()
                    },
                );
            }
        }
        super::protections::damage_label(p, s.damage, a, s.case == Case::Poison);
    }
    if let Some(a) = save_age.filter(|a| *a < 0.8) {
        draw_text_bold(
            "Dernier sursaut",
            p.x - 72.,
            p.y - 74. - a * 10.,
            19.,
            alpha(GOLD, ((0.8 - a) / 0.22).min(1.)),
        );
    }
    if s.hp == 0 {
        draw_text_bold("Mort", base.x - 19., base.y + 51., 18., DIM);
    }
    // A fixed scale makes the extra maximum visibly extend the bar, without adding HP.
    let x = rect.x + 24.;
    let y = rect.bottom() - 111.;
    let growth = if s.skill == 0 {
        s.begun.map_or(0., |t| ((s.clock - t) / 0.32).clamp(0., 1.))
    } else {
        0.
    };
    let maximum_drawn = 100. + (s.maximum as f32 - 100.) * growth;
    draw_rectangle(x - 3., y - 3., maximum_drawn * 1.5 + 6., 21., INK);
    draw_rectangle(
        x,
        y,
        maximum_drawn * 1.5,
        15.,
        Color::new(0.24, 0.06, 0.06, 1.),
    );
    draw_rectangle(x, y, s.hp as f32 * 1.5, 15., LIFE);
    if s.skill == 0 && s.begun.is_some() {
        draw_rectangle_lines(x + 150., y, maximum_drawn * 1.5 - 150., 15., 1.5, GOLD);
        let a = s.clock - s.begun.unwrap();
        art.icon(
            0,
            vec2(x + maximum_drawn * 1.5 + 30., y + 7.),
            32. * (1. + (a * 10.).sin().max(0.) * (1. - a).max(0.) * 0.12),
            1.,
        );
    }
    draw_text_bold(
        format!("PV {} / {}", s.hp, s.maximum),
        x,
        y - 10.,
        18.,
        TEXT,
    );
    if s.skill == 1 {
        let at = vec2(rect.x + 493., rect.bottom() - 99.);
        art.icon(1, at, 38., if s.cooldown == 0 { 1. } else { 0.36 });
        draw_text_bold(
            if s.cooldown == 0 {
                "Prêt".into()
            } else {
                s.cooldown.to_string()
            },
            at.x - 16.,
            at.y + 38.,
            16.,
            if s.cooldown == 0 { GOLD } else { DIM },
        );
    }
    draw_rectangle(
        rect.x + 8.,
        rect.bottom() - 56.,
        560.,
        48.,
        alpha(INK, 0.95),
    );
    draw_text(
        format!(
            "Énergie {} · tour {} · déclenchements {}",
            s.energy, s.turn, s.saves
        ),
        rect.x + 17.,
        rect.bottom() - 35.,
        16.,
        TEXT,
    );
    let detail = if s.skill == 0 {
        "Maximum de base 100 PV · aucun soin".into()
    } else {
        format!(
            "Recharge {} tours · {}",
            s.cooldown,
            if s.hp == 0 {
                "personnage mort"
            } else if s.saves > 0 {
                "aucune protection supplémentaire"
            } else {
                "coup direct requis"
            }
        )
    };
    draw_text(detail, rect.x + 17., rect.bottom() - 16., 15., DIM);
}
pub(crate) async fn run() {
    request_new_screen_size(1120., 820.);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load().await;
    let mut s = State::new(1, Case::Direct, 2);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Passifs de survie · ANIM-S39", 20., 34., 25., TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(Rect::new(20., 75. + i as f32 * 40., 330., 34.), title, true) {
                s = State::new(i, s.case, s.direction);
            }
        }
        for (i, title) in ["Droite", "Gauche", "Haut", "Bas"].iter().enumerate() {
            if button(
                Rect::new(
                    20. + (i % 2) as f32 * 166.,
                    175. + (i / 2) as f32 * 38.,
                    156.,
                    32.,
                ),
                title,
                true,
            ) {
                s = State::new(s.skill, s.case, i);
            }
        }
        for (i, (case, title)) in [
            (Case::Direct, "Coup direct"),
            (Case::Quiet, "Attente sans tour"),
            (Case::Poison, "Dégât persistant"),
            (Case::Unavailable, "Passif déjà en recharge"),
            (Case::Second, "Second coup"),
        ]
        .iter()
        .enumerate()
        {
            if button(
                Rect::new(20., 265. + i as f32 * 35., 330., 30.),
                title,
                true,
            ) {
                s = State::new(s.skill, *case, s.direction);
            }
        }
        if button(
            Rect::new(20., 465., 330., 34.),
            "Démarrer l’exemple",
            s.begun.is_none(),
        ) || is_key_pressed(KeyCode::Enter)
        {
            s.begin();
        }
        if button(
            Rect::new(20., 510., 330., 34.),
            if s.hits == 0 {
                "Premier coup préparé"
            } else {
                "+1 tour / second coup"
            },
            s.begun.is_some(),
        ) || is_key_pressed(KeyCode::Space)
        {
            if s.hits == 0 {
                s.first();
            } else {
                s.advance();
            }
        }
        if button(Rect::new(20., 555., 330., 34.), "Recommencer", true)
            || is_key_pressed(KeyCode::R)
        {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(20., 600., 330., 34.),
            if paused { "Reprendre" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
        }
        scene(&art, &s, Rect::new(400., 88., 576., 576.));
        draw_text(
            "87–88 · exemples préparés · rang effectif 11",
            400.,
            705.,
            16.,
            TEXT,
        );
        draw_text(
            "Aucun effet raccordé au combat de campagne.",
            400.,
            732.,
            15.,
            DIM,
        );
        next_frame().await;
    }
}

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    let wings_only = std::env::args().any(|a| a == "--capture-sursaut-ailes");
    if output.exists()
        || output
            .parent()
            .is_some_and(|p| p.join("visual-acceptance-01.json").exists())
    {
        return Err("Utiliser un dossier neuf et une revue non validée.".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let art = Art::load().await;
    let target = render_target(576, 576);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 576., 576.))
    };
    let cases = [
        (Case::Direct, 0, "DROITE"),
        (Case::Direct, 1, "GAUCHE"),
        (Case::Direct, 2, "HAUT"),
        (Case::Direct, 3, "BAS"),
        (Case::Quiet, 2, "ATTENTE"),
        (Case::Poison, 2, "PERSISTANT"),
        (Case::Unavailable, 2, "RECHARGE"),
        (Case::Second, 2, "SECOND"),
    ];
    let mut entries: Vec<serde_json::Value> = Vec::new();
    for skill in 0..2 {
        for (case, direction, label) in cases {
            if wings_only && skill != 1 {
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
                if frame == 12 {
                    s.begin();
                }
                if frame == 24 {
                    s.first();
                }
                if case != Case::Quiet && [48, 72, 96].contains(&frame) {
                    s.advance();
                }
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
            entries.push(
                serde_json::json!({"id":IDS[skill],"number":87+skill,"title":TITLES[skill],
                "variant":format!("{}-{}-{label}",IDS[skill],if skill == 1 { "A03" } else { "A01" }),"folder":folder,"case":case.label(),
                "direction":direction,"label":label,"frames":FRAMES,"fps":24,"indices":indices,
                "unique_images":known.len(),"native_columns":columns,"timeline":timeline}),
            );
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
