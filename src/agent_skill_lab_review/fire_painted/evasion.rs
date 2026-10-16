//! ANIM-S37: unchanged player artwork composed as a decoy and phase transition.
//! Prepared visual fixtures only; no campaign AI, damage, targeting or saves.
use super::super::melee::{actor, cut, player_visual};
use super::zones::{position, preview, terrain};
use super::*;
use project_rl::world::Map;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const IDS: [&str; 2] = ["ES03", "ES05"];
const TITLES: [&str; 2] = ["Leurre holographique", "Traversée de phase"];
const START: GridPos = GridPos::new(4, 6);
const END: GridPos = GridPos::new(4, 4);
const FRAMES: usize = 96;
const PURPLE: Color = Color::new(0.82, 0.29, 1.0, 1.0);
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#...#...#\n#.......#\n#.......#\n#.......#\n#########",
];
#[derive(Clone, Copy, PartialEq, Eq)]
enum Case {
    Impact,
    Duration,
    Quiet,
    Blocked,
}
impl Case {
    fn label(self) -> &'static str {
        match self {
            Self::Impact => "IMPACT",
            Self::Duration => "DUREE",
            Self::Quiet => "ATTENTE",
            Self::Blocked => "OBSTACLE",
        }
    }
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    cast_at: Option<f32>,
    impact_at: Option<f32>,
    end_at: Option<f32>,
    attempted: bool,
    remaining: u16,
    turn: u16,
    energy: u16,
    cooldown: u16,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.,
            cast_at: None,
            impact_at: None,
            end_at: None,
            attempted: false,
            remaining: 0,
            turn: 0,
            energy: 100,
            cooldown: 0,
        }
    }
    fn cast(&mut self) {
        if self.attempted {
            return;
        }
        self.attempted = true;
        if self.case == Case::Blocked {
            return;
        }
        self.cast_at = Some(self.clock);
        self.turn = 1;
        self.energy -= parameter_for(IDS[self.skill], "energy", 1);
        self.cooldown = parameter_for(IDS[self.skill], "cooldown_turns", 1);
        self.remaining = parameter_for(IDS[self.skill], "duration_turns", 1);
    }
    fn advance(&mut self) {
        if self.cast_at.is_none() {
            return;
        }
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        self.remaining = self.remaining.saturating_sub(1);
        if self.remaining == 0 && self.end_at.is_none() {
            self.end_at = Some(self.clock);
        }
    }
    fn impact(&mut self) {
        if self.skill != 0 || self.remaining == 0 {
            self.advance();
            return;
        }
        self.advance();
        self.remaining = 0;
        self.impact_at = Some(self.clock);
        self.end_at = Some(self.clock);
    }
    fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({"turn":self.turn,"energy":self.energy,"cooldown":self.cooldown,
            "remaining":self.remaining,"preview":!self.attempted,"blocked":self.case==Case::Blocked,
            "cast":self.cast_at.is_some(),"impact":self.impact_at.is_some(),
            "lure":self.skill==0&&self.remaining>0,
            "dodge":if self.skill==1&&self.remaining>0 {parameter_for("ES05","evasion_bonus_units",1)} else {0},
            "player":if self.skill==1&&self.cast_at.is_some() {[END.x,END.y]} else {[START.x,START.y]},
            "damage":0})
    }
}
pub(super) struct Art {
    player: RenderTarget,
    icons: [Texture2D; 2],
}
impl Art {
    pub(super) async fn load() -> Self {
        // Compose the existing native-64 body, fingers and held weapon once.
        // A 128-pixel canvas preserves the accepted equipment overhang.
        let player = render_target(128, 128);
        player.texture.set_filter(FilterMode::Nearest);
        let camera = Camera2D {
            render_target: Some(player.clone()),
            ..Camera2D::from_display_rect(Rect::new(0., 0., 128., 128.))
        };
        set_camera(&camera);
        clear_background(Color::new(0., 0., 0., 0.));
        actor(vec2(64., 64.), player_visual());
        unsafe {
            macroquad::window::get_internal_gl().flush();
        }
        set_default_camera();
        next_frame().await;
        let icons = [
            include_bytes!("../../../assets/skills/agent/ES03-compact.png").as_slice(),
            include_bytes!("../../../assets/skills/agent/ES05-compact.png").as_slice(),
        ]
        .map(|bytes| Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png)));
        Self { player, icons }
    }
    pub(super) fn piece(&self, p: Vec2, source: Rect, tint: Color) {
        draw_texture_ex(
            &self.player.texture,
            p.x - 64. + source.x,
            p.y - 64. + source.y,
            tint,
            DrawTextureParams {
                source: Some(Rect::new(
                    source.x,
                    128. - source.y - source.h,
                    source.w,
                    source.h,
                )),
                dest_size: Some(vec2(source.w, source.h)),
                flip_y: true,
                ..Default::default()
            },
        );
    }
    fn ghost(&self, p: Vec2, clock: f32, opacity: f32, breaking: f32) {
        // The thin vertical scan lines follow the artwork's alpha, never its box.
        for x in 0..128 {
            let lit = (x as f32 * 0.28 - clock * 6.).sin().max(0.);
            let offset = if breaking > 0. {
                (x as f32 * 1.9).sin() * breaking * 25.
            } else {
                0.
            };
            let tint = Color::new(
                0.58 + lit * 0.20,
                0.78 + lit * 0.20,
                1.,
                opacity * (1. - breaking),
            );
            self.piece(
                p + vec2(offset, -breaking * (x as f32 * 0.67).sin() * 15.),
                Rect::new(x as f32, 0., 1., 128.),
                tint,
            );
        }
    }
    fn phase(&self, from: Vec2, to: Vec2, age: f32) {
        let t = (age / 0.24).clamp(0., 1.);
        let envelope = (std::f32::consts::PI * t).sin();
        for trail in (1..=3).rev() {
            let k = (t - trail as f32 * 0.12).max(0.);
            let p = from.lerp(to, k);
            for band in 0..16 {
                let y = band as f32 * 8.;
                let offset = (band as f32 * 2.3).sin() * envelope * 12.;
                self.piece(
                    p + vec2(offset, 0.),
                    Rect::new(0., y, 128., 8.),
                    alpha(PURPLE, envelope * 0.38 / trail as f32),
                );
            }
        }
        let p = from.lerp(to, t);
        for band in 0..16 {
            let y = band as f32 * 8.;
            let offset = (band as f32 * 2.3).sin() * envelope * 8.;
            self.piece(
                p + vec2(offset, 0.),
                Rect::new(0., y, 128., 8.),
                Color::new(1., 1. - envelope * 0.5, 1., 1. - envelope * 0.12),
            );
        }
    }
}
fn scene(art: &Art, s: &State, rect: Rect) {
    let map = Map::from_ascii(MAPS[usize::from(s.case == Case::Blocked)]).unwrap();
    terrain(&map, s.direction, rect, false);
    if !s.attempted {
        preview(&[END], s.direction, rect, s.clock);
    }
    terrain(&map, s.direction, rect, true);
    let point = |at| {
        let p = position(at, s.direction);
        vec2(
            rect.x + (p.x as f32 + 0.5) * 64.,
            rect.y + (p.y as f32 + 0.5) * 64.,
        )
    };
    let start = point(START);
    let end = point(END);
    let e = point(GridPos::new(4, if s.skill == 0 { 3 } else { 5 }));
    let age = s.cast_at.map(|t| s.clock - t);
    // Only the prepared successful diversion is a negative effect on the enemy.
    let recoil = if s.skill == 0 {
        age.map_or(Vec2::ZERO, |a| {
            crate::combat_visuals::hit_reaction_offset(e, end, f64::from(a), 64.)
        })
    } else {
        Vec2::ZERO
    };
    enemy(e + recoil);
    if s.skill == 0 {
        actor(start, player_visual());
        if s.remaining > 0 {
            let build = age.map_or(0., |a| (a / 0.16).min(1.));
            art.ghost(
                end,
                s.clock,
                (0.64 + (s.clock * 4.).sin() * 0.06) * build,
                0.,
            );
        } else if let Some(a) = s.end_at.map(|t| s.clock - t).filter(|a| *a < 0.24) {
            art.ghost(end, s.clock, 0.64, a / 0.24);
        }
        if let Some(a) = s.impact_at.map(|t| s.clock - t).filter(|a| *a < 0.15) {
            cut(e.lerp(end, 0.25), end, 5., 1. - a / 0.15);
        }
    } else if let Some(a) = age.filter(|a| *a < 0.24) {
        art.phase(start, end, a);
    } else {
        actor(if age.is_some() { end } else { start }, player_visual());
    }
    if s.remaining > 0 {
        let at = vec2(rect.x + 492., rect.y + 16.);
        draw_rectangle(at.x, at.y, 42., 42., alpha(INK, 0.95));
        draw_texture_ex(
            &art.icons[s.skill],
            at.x + 2.,
            at.y + 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(38., 38.)),
                ..Default::default()
            },
        );
        draw_rectangle(at.x + 26., at.y + 26., 18., 18., INK);
        draw_text_bold(s.remaining.to_string(), at.x + 29., at.y + 40., 16., TEXT);
    }
    if s.case == Case::Blocked && s.attempted {
        draw_text_bold(
            "Case d’arrivée bloquée",
            rect.x + 148.,
            rect.y + 92.,
            20.,
            Color::new(1., 0.48, 0.35, 1.),
        );
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
            "Énergie {} · tour {} · recharge {}",
            s.energy, s.turn, s.cooldown
        ),
        rect.x + 17.,
        rect.bottom() - 35.,
        16.,
        TEXT,
    );
    let detail = if s.skill == 0 {
        format!(
            "Leurre : {} · durée restante {} tours",
            if s.remaining > 0 { "actif" } else { "absent" },
            s.remaining
        )
    } else {
        format!(
            "Esquive : +{} · durée restante {} tours",
            if s.remaining > 0 { 10 } else { 0 },
            s.remaining
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
    let mut s = State::new(0, Case::Impact, 2);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Esquive · ANIM-S37", 20., 34., 25., TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(Rect::new(20., 80. + i as f32 * 40., 330., 34.), title, true) {
                s = State::new(i, s.case, s.direction);
            }
        }
        for (i, title) in ["Droite", "Gauche", "Haut", "Bas"].iter().enumerate() {
            if button(
                Rect::new(
                    20. + (i % 2) as f32 * 166.,
                    180. + (i / 2) as f32 * 40.,
                    156.,
                    33.,
                ),
                title,
                true,
            ) {
                s = State::new(s.skill, s.case, i);
            }
        }
        for (i, (case, title)) in [
            (Case::Impact, "Premier impact"),
            (Case::Duration, "Fin de durée"),
            (Case::Quiet, "Attente sans tour"),
            (Case::Blocked, "Destination occupée par un mur"),
        ]
        .iter()
        .enumerate()
        {
            if button(
                Rect::new(20., 280. + i as f32 * 38., 330., 32.),
                title,
                true,
            ) {
                s = State::new(s.skill, *case, s.direction);
            }
        }
        if button(Rect::new(20., 460., 330., 34.), "Lancer", !s.attempted)
            || is_key_pressed(KeyCode::Enter)
        {
            s.cast();
        }
        if button(
            Rect::new(20., 505., 330., 34.),
            "+1 tour / impact préparé",
            s.cast_at.is_some(),
        ) || is_key_pressed(KeyCode::Space)
        {
            if s.case == Case::Impact && s.skill == 0 && s.remaining > 0 {
                s.impact();
            } else {
                s.advance();
            }
        }
        if button(Rect::new(20., 550., 330., 34.), "Recommencer", true)
            || is_key_pressed(KeyCode::R)
        {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(20., 595., 330., 34.),
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
            "83–84 · exemples préparés · rang effectif 1",
            400.,
            705.,
            16.,
            TEXT,
        );
        draw_text(
            "Rendu proposé ; aucun effet raccordé à la campagne.",
            400.,
            732.,
            15.,
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
        (Case::Impact, 0, "DROITE"),
        (Case::Impact, 1, "GAUCHE"),
        (Case::Impact, 2, "HAUT"),
        (Case::Impact, 3, "BAS"),
        (Case::Duration, 2, "DUREE"),
        (Case::Quiet, 2, "ATTENTE"),
        (Case::Blocked, 2, "OBSTACLE"),
    ];
    let mut entries = Vec::new();
    for skill in 0..2 {
        for (case, direction, label) in cases {
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
                if case != Case::Quiet && [48, 72, 84].contains(&frame) {
                    if case == Case::Impact && frame == 48 && skill == 0 {
                        s.impact();
                    } else {
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
            entries.push(
                serde_json::json!({"id":IDS[skill],"number":83+skill,"title":TITLES[skill],
                "variant":format!("{}-A01-{label}",IDS[skill]),"folder":folder,"case":case.label(),
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
