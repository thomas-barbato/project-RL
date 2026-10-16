//! ANIM-S38: prepared projectile outcomes and a spectral step, never a combat resolver.
use super::super::melee::{actor, player_visual};
use super::zones::{position, preview, terrain};
use super::*;
use macroquad::miniquad::{BlendFactor, BlendState, BlendValue, Equation, PipelineParams};
use project_rl::world::Map;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const IDS: [&str; 2] = ["ES06", "T02"];
const TITLES: [&str; 2] = ["Réflexes balistiques", "Pas spectral"];
const START: GridPos = GridPos::new(4, 5);
const END: GridPos = GridPos::new(4, 4);
const SHOOTER: GridPos = GridPos::new(4, 1);
const FRAMES: usize = 96;
const CYAN: Color = Color::new(0.48, 0.82, 1., 1.);
const MAPS: [&str; 2] = [
    "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
    "#########\n#.......#\n#.......#\n#.......#\n#...#...#\n#.......#\n#.......#\n#.......#\n#########",
];
#[derive(Clone, Copy, PartialEq, Eq)]
enum Case {
    Dodge,
    Hit,
    Quiet,
    Wall,
}
impl Case {
    fn label(self) -> &'static str {
        match self {
            Self::Dodge => "ESQUIVE",
            Self::Hit => "TOUCHE",
            Self::Quiet => "ATTENTE",
            Self::Wall => "OBSTACLE",
        }
    }
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    begun: Option<f32>,
    shot_at: Option<f32>,
    hit_at: Option<f32>,
    resolved: bool,
    attempted: bool,
    turn: u16,
    hp: u16,
    energy: u16,
    cooldown: u16,
    remaining: u16,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.,
            begun: None,
            shot_at: None,
            hit_at: None,
            resolved: false,
            attempted: false,
            turn: 0,
            hp: 100,
            energy: 100,
            cooldown: 0,
            remaining: 0,
        }
    }
    fn begin(&mut self) {
        if self.attempted {
            return;
        }
        self.attempted = true;
        if self.skill == 1 && self.case == Case::Wall {
            return;
        }
        self.begun = Some(self.clock);
        self.turn = 1;
        if self.skill == 1 {
            self.energy -= parameter_for("T02", "energy", 1);
            self.cooldown = parameter_for("T02", "cooldown_turns", 1);
            self.remaining = parameter_for("T02", "duration_turns", 1);
        } else if self.case != Case::Quiet {
            self.shot_at = Some(self.clock);
        }
    }
    fn advance(&mut self) {
        if self.begun.is_none() {
            return;
        }
        self.turn += 1;
        if self.skill == 1 {
            self.cooldown = self.cooldown.saturating_sub(1);
            self.remaining = self.remaining.saturating_sub(1);
            if self.case == Case::Hit && self.turn == 2 {
                self.shot_at = Some(self.clock);
            }
        }
    }
    fn resolve(&mut self) {
        if self.resolved || self.shot_at.is_none_or(|t| self.clock - t < 0.20) {
            return;
        }
        self.resolved = true;
        if self.case == Case::Hit {
            self.hp -= 12;
            self.hit_at = self.shot_at.map(|t| t + 0.20);
        }
    }
    fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({"turn":self.turn,"hp":self.hp,"energy":self.energy,"cooldown":self.cooldown,
            "remaining":self.remaining,"preview":!self.attempted,"begun":self.begun.is_some(),
            "shot":self.shot_at.is_some(),"hit":self.hit_at.is_some(),"resolved":self.resolved,
            "dodge":if self.skill==0 {parameter_for("ES06","projectile_evasion_bonus_units",1)}
              else if self.remaining>0 {parameter_for("T02","evasion_bonus_units",1)} else {0},
            "player":if self.skill==1&&self.begun.is_some() {[END.x,END.y]} else {[START.x,START.y]},
            "passive":self.skill==0,"permanent_aura":false})
    }
}
struct Art {
    body: super::evasion::Art,
    silhouette: Material,
    step_icon: Texture2D,
    blood: Texture2D,
}
impl Art {
    async fn load() -> Self {
        let body = super::evasion::Art::load().await;
        let silhouette = load_material(
            ShaderSource::Glsl {
                vertex: r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;varying highp vec2 uv;varying lowp vec4 tint;
void main(){uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#,
                fragment: r#"#version 100
precision highp float;varying highp vec2 uv;varying lowp vec4 tint;uniform sampler2D Texture;
void main(){float a=texture2D(Texture,uv).a;gl_FragColor=vec4(tint.rgb,a*tint.a);}"#,
            },
            MaterialParams {
                pipeline_params: PipelineParams {
                    color_blend: Some(BlendState::new(
                        Equation::Add,
                        BlendFactor::Value(BlendValue::SourceAlpha),
                        BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .expect("spectral silhouette shader");
        let step_icon = Texture2D::from_file_with_format(
            include_bytes!("../../../assets/skills/agent/T02-compact.png"),
            Some(ImageFormat::Png),
        );
        let blood = Texture2D::from_file_with_format(
            include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-40/blood-atlas.png"
            ),
            Some(ImageFormat::Png),
        );
        Self {
            body,
            silhouette,
            step_icon,
            blood,
        }
    }
    fn ghost(&self, at: Vec2, opacity: f32) {
        gl_use_material(&self.silhouette);
        self.body
            .piece(at, Rect::new(0., 0., 128., 128.), alpha(CYAN, opacity));
        gl_use_default_material();
    }
    fn contour(&self, at: Vec2, opacity: f32) {
        for offset in [vec2(-1.5, 0.), vec2(1.5, 0.), vec2(0., -1.5)] {
            self.ghost(at + offset, opacity);
        }
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
    let map = Map::from_ascii(MAPS[usize::from(s.case == Case::Wall)]).unwrap();
    terrain(&map, s.direction, rect, false);
    if !s.attempted {
        preview(
            &[if s.skill == 1 { END } else { START }],
            s.direction,
            rect,
            s.clock,
        );
    }
    terrain(&map, s.direction, rect, true);
    let from = point(START, s.direction, rect);
    let to = point(END, s.direction, rect);
    let source = point(SHOOTER, s.direction, rect);
    if s.skill == 0 || s.case == Case::Hit {
        enemy(source);
    }
    let age = s.begun.map(|t| s.clock - t);
    let shot_age = s.shot_at.map(|t| s.clock - t);
    let mut p = if s.skill == 1 && age.is_some() {
        from.lerp(to, (age.unwrap() / 0.18).min(1.))
    } else {
        from
    };
    let direction = (from - source).normalize_or_zero();
    let sideways = vec2(-direction.y, direction.x);
    // Prepared miss: a small sideways move returns to exactly the same tile.
    let dodge_age = if s.skill == 0 && s.case == Case::Dodge {
        shot_age.map(|a| a - 0.12)
    } else {
        None
    };
    if let Some(a) = dodge_age.filter(|a| (0.0..0.24).contains(a)) {
        let pulse = (std::f32::consts::PI * a / 0.24).sin();
        art.ghost(from, 0.24 * pulse);
        p += sideways * (12. * pulse);
    }
    if s.skill == 1 && s.remaining > 0 {
        if let Some(a) = age.filter(|a| *a < 0.28) {
            for trail in (1..=3).rev() {
                let k = ((a - trail as f32 * 0.04) / 0.18).clamp(0., 1.);
                art.ghost(from.lerp(to, k), 0.27 * (1. - a / 0.28));
            }
        }
        art.contour(p, 0.31 + (s.clock * 3.2).sin() * 0.05);
    }
    if let Some(a) = s.hit_at.map(|t| s.clock - t) {
        p += crate::combat_visuals::hit_reaction_offset(p, source, f64::from(a), 64.);
    }
    actor(p, player_visual());
    if let Some(a) = shot_age {
        let end = if s.case == Case::Wall {
            point(END, s.direction, rect) - direction * 32.
        } else if s.case == Case::Hit {
            if s.skill == 1 { to } else { from }
        } else {
            from + direction * 100.
        };
        let duration = if s.case == Case::Dodge { 0.29 } else { 0.20 };
        if a < duration {
            let head = source.lerp(end, (a / duration).clamp(0., 1.));
            let tail = head - direction * 13.;
            draw_line(tail.x, tail.y, head.x, head.y, 5., alpha(INK, 0.9));
            draw_line(
                tail.x,
                tail.y,
                head.x,
                head.y,
                2.2,
                Color::new(1., 0.88, 0.53, 1.),
            );
        }
        if s.case == Case::Wall && (0.20..0.32).contains(&a) {
            let strength = 1. - (a - 0.20) / 0.12;
            for i in 0..4 {
                let d = vec2((i as f32 * 1.8).cos(), (i as f32 * 1.8).sin());
                let q = end + d * (4. + (a - 0.20) * 90.);
                draw_line(
                    q.x,
                    q.y,
                    q.x + d.x * 5.,
                    q.y + d.y * 5.,
                    1.8,
                    alpha(TEXT, strength),
                );
            }
        }
    }
    if let Some(a) = dodge_age.filter(|a| (0.0..0.65).contains(a)) {
        draw_text_bold(
            "Esquive",
            p.x - 29.,
            p.y - 46. - a * 13.,
            18.,
            alpha(CYAN, ((0.65 - a) / 0.18).min(1.)),
        );
    }
    if let Some(a) = s.hit_at.map(|t| s.clock - t).filter(|a| *a < 0.8) {
        if a < 0.29 {
            let frame = (a / 0.29 * 6.).min(5.).floor();
            draw_texture_ex(
                &art.blood,
                p.x - 28.,
                p.y - 28.,
                alpha(WHITE, 1. - a / 0.33),
                DrawTextureParams {
                    source: Some(Rect::new(frame * 256., 0., 256., 256.)),
                    dest_size: Some(vec2(56., 56.)),
                    ..Default::default()
                },
            );
        }
        super::protections::damage_label(p, 12, a, false);
    }
    if s.skill == 1 && s.remaining > 0 {
        let at = vec2(rect.x + 492., rect.y + 16.);
        draw_rectangle(at.x, at.y, 42., 42., alpha(INK, 0.95));
        draw_texture_ex(
            &art.step_icon,
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
    if s.skill == 1 && s.case == Case::Wall && s.attempted {
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
        format!("PV {} / 100 · énergie {} · tour {}", s.hp, s.energy, s.turn),
        rect.x + 17.,
        rect.bottom() - 35.,
        16.,
        TEXT,
    );
    draw_text(
        if s.skill == 0 {
            "Bonus d’esquive contre les projectiles : +12".into()
        } else {
            format!(
                "Esquive +{} · reste {} tours · recharge {}",
                if s.remaining > 0 { 15 } else { 0 },
                s.remaining,
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
    let art = Art::load().await;
    let mut s = State::new(0, Case::Dodge, 2);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Esquive · ANIM-S38", 20., 34., 25., TEXT);
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
            (Case::Dodge, "Esquive / pas libre"),
            (Case::Hit, "Tir qui touche"),
            (Case::Quiet, "Attente sans tour"),
            (Case::Wall, "Obstacle"),
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
        if button(
            Rect::new(20., 460., 330., 34.),
            "Lancer l’aperçu",
            !s.attempted,
        ) || is_key_pressed(KeyCode::Enter)
        {
            s.begin();
        }
        if button(
            Rect::new(20., 505., 330., 34.),
            "+1 tour",
            s.begun.is_some(),
        ) || is_key_pressed(KeyCode::Space)
        {
            s.advance();
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
        s.resolve();
        scene(&art, &s, Rect::new(400., 88., 576., 576.));
        draw_text(
            "85–86 · résultats préparés · rang effectif 1",
            400.,
            705.,
            16.,
            TEXT,
        );
        draw_text(
            "Aucune probabilité ou règle de combat créée.",
            400.,
            732.,
            15.,
            DIM,
        );
        next_frame().await;
    }
}

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    let wall_only = std::env::args().any(|a| a == "--capture-reflexes-obstacle");
    if (output.exists() && !wall_only)
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
        (Case::Dodge, 0, "DROITE"),
        (Case::Dodge, 1, "GAUCHE"),
        (Case::Dodge, 2, "HAUT"),
        (Case::Dodge, 3, "BAS"),
        (Case::Hit, 2, "TOUCHE"),
        (Case::Quiet, 2, "ATTENTE"),
        (Case::Wall, 2, "OBSTACLE"),
    ];
    let mut entries: Vec<serde_json::Value> = if wall_only && output.join("capture.json").exists() {
        serde_json::from_slice(
            &std::fs::read(output.join("capture.json")).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?
    } else {
        Vec::new()
    };
    for skill in 0..2 {
        for (case, direction, label) in cases {
            if wall_only && (skill != 0 || case != Case::Wall) {
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
                if case != Case::Quiet && [48, 72, 84].contains(&frame) {
                    s.advance();
                }
                s.resolve();
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
                serde_json::json!({"id":IDS[skill],"number":85+skill,"title":TITLES[skill],
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
