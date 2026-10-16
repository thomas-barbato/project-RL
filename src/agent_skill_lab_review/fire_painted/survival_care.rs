//! ANIM-S36: visual fixtures for healing and cleansing, never campaign commands.
use super::super::melee::{actor, cut, player_visual};
use super::zones::{position, preview, terrain};
use super::*;
use project_rl::world::Map;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const IDS: [&str; 2] = ["SU04", "SU05"];
const TITLES: [&str; 2] = ["Régénération", "Purge"];
const MAP: &str = "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########";
const PLAYER: GridPos = GridPos::new(4, 4);
const FRAMES: usize = 112;
const GREEN: Color = Color::new(0.45, 1., 0.32, 1.);
const PURGE_LIGHT: Color = Color::new(1., 0.91, 0.63, 1.);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Case {
    Complete,
    Quiet,
    Interrupted,
}
impl Case {
    fn label(self) -> &'static str {
        match self {
            Self::Complete => "COMPLET",
            Self::Quiet => "ATTENTE",
            Self::Interrupted => "INTERRUPTION",
        }
    }
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    cast_at: Option<f32>,
    pulse_at: Option<f32>,
    harm_at: Option<f32>,
    hp: u16,
    energy: u16,
    turn: u16,
    cooldown: u16,
    doses: u16,
    channel: bool,
    healed: u16,
    poison: bool,
    burn: bool,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.,
            cast_at: None,
            pulse_at: None,
            harm_at: None,
            hp: 40,
            energy: 100,
            turn: 0,
            cooldown: 0,
            doses: 0,
            channel: false,
            healed: 0,
            poison: skill == 1,
            burn: skill == 1,
        }
    }
    fn heal(&mut self) {
        let cost = parameter_for("SU04", "energy_per_turn", 11);
        if self.energy < cost || self.hp == 100 {
            self.channel = false;
            return;
        }
        self.energy -= cost;
        // Exact prepared 100 maximum HP; no new rounding or HP rule.
        self.healed =
            parameter_for("SU04", "heal_percent_of_max_HP_per_turn", 11).min(100 - self.hp);
        self.hp += self.healed;
        self.doses += 1;
        self.pulse_at = Some(self.clock);
        self.channel =
            self.doses < parameter_for("SU04", "maximum_channel_turns", 11) && self.hp < 100;
    }
    fn cast(&mut self) {
        if self.cast_at.is_some() {
            return;
        }
        self.cast_at = Some(self.clock);
        self.turn += 1;
        self.cooldown = parameter_for(IDS[self.skill], "cooldown_turns", 11);
        if self.skill == 0 {
            self.heal();
        } else {
            // The catalogue's decreasing curve has an explicit floor of one.
            self.energy -= parameter_for("SU05", "energy", 11).max(1);
            // Two compatible statuses, both within the current limit of three.
            // No recency tie, equipment malus or Silence eligibility is decided here.
            self.poison = false;
            self.burn = false;
        }
    }
    fn advance(&mut self) {
        if self.cast_at.is_none() {
            return;
        }
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        if self.channel {
            self.heal();
        }
    }
    fn interrupt(&mut self) {
        if self.cast_at.is_none() || self.skill != 0 {
            return;
        }
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        self.harm_at = Some(self.clock);
        self.hp = self.hp.saturating_sub(8);
        self.channel = false;
    }
    fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({"hp":self.hp,"energy":self.energy,"turn":self.turn,
            "cooldown":self.cooldown,"doses":self.doses,"channel":self.channel,
            "poison":self.poison,"burn":self.burn,"preview":self.cast_at.is_none(),
            "healed":self.healed,"interrupted":self.harm_at.is_some()})
    }
}
struct Art {
    repair: Texture2D,
    purge: Texture2D,
    icons: [Texture2D; 4],
    blood: Texture2D,
}
impl Art {
    fn load() -> Self {
        fn load(bytes: &[u8]) -> Texture2D {
            let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            texture.set_filter(FilterMode::Linear);
            texture
        }
        Self {
            repair: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-77/regeneration-source.png"
            )),
            purge: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-78/purge-source.png"
            )),
            icons: [
                load(include_bytes!(
                    "../../../assets/skills/agent/SU04-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/SU05-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/SM02-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/FE06-compact.png"
                )),
            ],
            blood: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-40/blood-atlas.png"
            )),
        }
    }
    fn picture(texture: &Texture2D, p: Vec2, size: Vec2, opacity: f32) {
        draw_texture_ex(
            texture,
            p.x - size.x / 2.,
            p.y - size.y / 2.,
            alpha(WHITE, opacity),
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );
    }
    fn badge(&self, icon: usize, at: Vec2, remaining: Option<u16>) {
        draw_rectangle(at.x, at.y, 42., 42., alpha(INK, 0.95));
        draw_rectangle_lines(at.x, at.y, 42., 42., 1., DIM);
        self.symbol(icon, at + vec2(21., 21.), vec2(38., 38.), 1.);
        if let Some(remaining) = remaining {
            draw_rectangle(at.x + 26., at.y + 26., 16., 16., INK);
            draw_text_bold(remaining.to_string(), at.x + 29., at.y + 39., 15., TEXT);
        }
    }
    fn symbol(&self, icon: usize, p: Vec2, size: Vec2, opacity: f32) {
        // Reuse the poison droplet alone, never a mine in the status bar.
        draw_texture_ex(
            &self.icons[icon],
            p.x - size.x / 2.,
            p.y - size.y / 2.,
            alpha(WHITE, opacity),
            DrawTextureParams {
                source: if icon == 2 {
                    Some(Rect::new(22., 0., 20., 25.))
                } else {
                    None
                },
                dest_size: Some(size),
                ..Default::default()
            },
        );
    }
}
fn scene(art: &Art, s: &State, rect: Rect) {
    let map = Map::from_ascii(MAP).unwrap();
    terrain(&map, s.direction, rect, false);
    if s.cast_at.is_none() {
        preview(&[PLAYER], s.direction, rect, s.clock);
    }
    terrain(&map, s.direction, rect, true);
    let point = |at| {
        let at = position(at, s.direction);
        vec2(
            rect.x + (at.x as f32 + 0.5) * 64.,
            rect.y + (at.y as f32 + 0.5) * 64.,
        )
    };
    let base = point(PLAYER);
    let e = point(GridPos::new(4, 3));
    let harm_age = s.harm_at.map(|t| s.clock - t);
    let recoil = harm_age.map_or(Vec2::ZERO, |a| {
        crate::combat_visuals::hit_reaction_offset(base, e, f64::from(a), 64.)
    });
    let p = base + recoil;
    let purge_age = if s.skill == 1 {
        s.cast_at.map(|t| s.clock - t).filter(|a| *a < 0.8)
    } else {
        None
    };
    // Purge is reviewed alone: existing afflictions are HUD markers, not extra actor VFX.
    if s.channel {
        let pulse = (s.clock * 4.).sin() * 0.5 + 0.5;
        Art::picture(
            &art.repair,
            p + vec2(0., -2. + (s.clock * 2.8).sin() * 1.5),
            vec2(49. + pulse * 4., 78.),
            0.8 + pulse * 0.2,
        );
    }
    if let Some(age) = purge_age {
        let pulse = (s.clock * 4.).sin() * 0.5 + 0.5;
        Art::picture(
            &art.purge,
            p + vec2(0., -2. + (s.clock * 2.8).sin() * 1.5),
            vec2(49. + pulse * 4., 78.),
            (1. - age / 0.8).powf(0.4),
        );
    }
    actor(p, player_visual());
    if s.case == Case::Interrupted && s.skill == 0 {
        enemy(e);
    }
    if s.channel {
        // Short stitches cross the body without obscuring its silhouette.
        for i in 0..3 {
            let phase = (s.clock * 0.65 + i as f32 / 3.).fract();
            let y = p.y + 22. - phase * 45.;
            let x = p.x + (phase * 6. + i as f32).sin() * 8.;
            let opacity = (std::f32::consts::PI * phase).sin().max(0.) * 0.8;
            draw_line(x - 5., y, x + 5., y, 2., alpha(GREEN, opacity));
        }
    }
    if let Some(age) = purge_age {
        for i in 0..3 {
            let phase = (s.clock * 0.65 + i as f32 / 3.).fract();
            let y = p.y + 22. - phase * 45.;
            let x = p.x + (phase * 6. + i as f32).sin() * 8.;
            let opacity = (std::f32::consts::PI * phase).sin().max(0.) * 0.8 * (1. - age / 0.8);
            draw_line(x - 5., y, x + 5., y, 2., alpha(PURGE_LIGHT, opacity));
        }
    }
    if s.skill == 0
        && let Some(a) = s.cast_at.map(|t| s.clock - t).filter(|a| *a < 0.5)
    {
        draw_text_bold(
            TITLES[s.skill],
            p.x - 61.,
            p.y - 70. - a * 12.,
            16.,
            alpha(TEXT, 1. - a / 0.5),
        );
    }
    if s.skill == 1
        && let Some(age) = s.cast_at.map(|t| s.clock - t).filter(|a| *a < 0.95)
    {
        let x = p.x - crate::ui_theme::measure_text_bold("Purge", 22).width / 2.;
        let y = p.y - 51. - age * 16.;
        let opacity = ((0.95 - age) / 0.25).min(1.);
        draw_text_bold("Purge", x + 1., y + 1., 22., alpha(INK, opacity));
        draw_text_bold("Purge", x, y, 22., alpha(PURGE_LIGHT, opacity));
    }
    if let Some(a) = s.pulse_at.map(|t| s.clock - t).filter(|a| *a < 0.8) {
        draw_text_bold(
            format!("+{} PV", s.healed),
            p.x - 28.,
            p.y - 48. - a * 14.,
            21.,
            alpha(GREEN, ((0.8 - a) / 0.2).min(1.)),
        );
    }
    if let Some(a) = harm_age {
        if s.skill == 0 {
            if a < 0.16 {
                cut(p + vec2(-20., -12.), p + vec2(20., 12.), 6., 1. - a / 0.16);
            }
            if (0.02..0.31).contains(&a) {
                let frame = ((a - 0.02) / 0.29 * 6.).min(5.);
                draw_texture_ex(
                    &art.blood,
                    p.x - 28.,
                    p.y - 28.,
                    alpha(WHITE, 1. - a / 0.35),
                    DrawTextureParams {
                        source: Some(Rect::new(frame.floor() * 256., 0., 256., 256.)),
                        dest_size: Some(vec2(56., 56.)),
                        ..Default::default()
                    },
                );
            }
            super::protections::damage_label(p, 8, a, false);
        }
    }
    let badge = vec2(rect.x + 484., rect.y + 16.);
    if s.channel {
        art.badge(
            0,
            badge,
            Some(parameter_for("SU04", "maximum_channel_turns", 11) - s.doses),
        );
    }
    if s.poison {
        art.badge(2, badge, None);
    }
    if s.burn {
        art.badge(3, badge - vec2(48., 0.), None);
    }
    draw_rectangle(
        rect.x + 8.,
        rect.bottom() - 56.,
        560.,
        48.,
        alpha(INK, 0.94),
    );
    draw_text(
        format!("PV {} / 100 · énergie {} · tour {}", s.hp, s.energy, s.turn),
        rect.x + 17.,
        rect.bottom() - 35.,
        16.,
        TEXT,
    );
    draw_text(
        if s.skill == 0 {
            format!(
                "Soins : {} / 3 · recharge {} · {}",
                s.doses,
                s.cooldown,
                if s.channel {
                    "canalisation"
                } else {
                    "arrêtée"
                }
            )
        } else {
            format!(
                "Poison : {} · brûlure : {} · recharge {}",
                if s.poison { "actif" } else { "absent" },
                if s.burn { "active" } else { "absente" },
                s.cooldown
            )
        },
        rect.x + 17.,
        rect.bottom() - 16.,
        15.,
        DIM,
    );
}

pub(crate) async fn run() {
    request_new_screen_size(1120., 820.);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let mut s = State::new(0, Case::Complete, 2);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Soins de survie · ANIM-S36", 20., 34., 25., TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(Rect::new(20., 88. + i as f32 * 42., 330., 34.), title, true) {
                s = State::new(i, s.case, s.direction);
            }
        }
        for (i, title) in ["Droite", "Gauche", "Haut", "Bas"].iter().enumerate() {
            if button(
                Rect::new(
                    20. + (i % 2) as f32 * 166.,
                    200. + (i / 2) as f32 * 40.,
                    156.,
                    33.,
                ),
                title,
                true,
            ) {
                s = State::new(s.skill, s.case, i);
            }
        }
        if button(
            Rect::new(20., 310., 330., 35.),
            "Lancer",
            s.cast_at.is_none(),
        ) || is_key_pressed(KeyCode::Enter)
        {
            s.cast();
        }
        if button(
            Rect::new(20., 360., 330., 35.),
            "Maintenir / +1 tour",
            s.cast_at.is_some(),
        ) || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if s.skill == 0
            && button(
                Rect::new(20., 410., 330., 35.),
                "Subir une blessure",
                s.cast_at.is_some(),
            )
        {
            s.case = Case::Interrupted;
            s.interrupt();
        }
        if button(Rect::new(20., 460., 330., 35.), "Recommencer", true)
            || is_key_pressed(KeyCode::R)
        {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(20., 510., 330., 35.),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
        }
        scene(&art, &s, Rect::new(400., 88., 576., 576.));
        draw_text(
            "81–82 · exemples préparés · rang effectif 11",
            400.,
            705.,
            16.,
            TEXT,
        );
        draw_text(
            "Aucun effet de campagne raccordé par cet aperçu.",
            400.,
            731.,
            15.,
            DIM,
        );
        next_frame().await;
    }
}

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    let purge_only = std::env::args().any(|arg| arg == "--capture-soins-purge");
    let healing_reference = std::env::args().any(|arg| arg == "--capture-soins-reference");
    if output.exists() && !purge_only {
        return Err("Dossier de capture déjà présent.".into());
    }
    if output
        .parent()
        .is_some_and(|p| p.join("visual-acceptance-01.json").exists())
    {
        return Err("Revue déjà validée.".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    let art = Art::load();
    let target = render_target(576, 576);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 576., 576.))
    };
    let cases = [
        (Case::Complete, 2, "COMPLET"),
        (Case::Quiet, 2, "ATTENTE"),
        (Case::Interrupted, 0, "DROITE"),
        (Case::Interrupted, 1, "GAUCHE"),
        (Case::Interrupted, 2, "HAUT"),
        (Case::Interrupted, 3, "BAS"),
    ];
    let mut entries: Vec<serde_json::Value> = if purge_only && output.join("capture.json").exists()
    {
        serde_json::from_slice(
            &std::fs::read(output.join("capture.json")).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?
    } else {
        Vec::new()
    };
    for skill in 0..2 {
        if purge_only && skill == 0 && !healing_reference {
            continue;
        }
        for (case, direction, label) in cases {
            if purge_only && skill == 0 && case != Case::Complete {
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
                    s.cast();
                }
                if case != Case::Quiet {
                    if frame == 48 {
                        if case == Case::Interrupted && skill == 0 {
                            s.interrupt();
                        } else {
                            s.advance();
                        }
                    }
                    if frame == 78 || frame == 96 {
                        s.advance();
                    }
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
            let variant = if skill == 0 { "A01" } else { "A03" };
            entries.push(serde_json::json!({"id":IDS[skill],"number":81+skill,"title":TITLES[skill],"variant":format!("{}-{variant}-{label}",IDS[skill]),"folder":folder,"case":case.label(),"direction":direction,"label":label,"frames":FRAMES,"fps":24,"indices":indices,"unique_images":known.len(),"native_columns":columns,"timeline":timeline}));
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
