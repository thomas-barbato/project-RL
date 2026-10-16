//! ANIM-S40: permanent cost/damage feedback in isolated prepared actions.
use super::super::melee::{actor, cut, player_visual};
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const IDS: [&str; 2] = ["A03", "M03"];
const TITLES: [&str; 2] = ["Canalisation", "Puissance métamorphe"];
const MAP: &str = "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########";
const FRAMES: usize = 112;
const ENERGY: Color = Color::new(0.63, 0.36, 1., 1.);
#[derive(Clone, Copy, PartialEq, Eq)]
enum Case {
    Bonus,
    Without,
    Outside,
    Quiet,
}
impl Case {
    fn label(self) -> &'static str {
        match self {
            Self::Bonus => "BONUS",
            Self::Without => "SANS",
            Self::Outside => "HORS",
            Self::Quiet => "ATTENTE",
        }
    }
}
struct State {
    skill: usize,
    case: Case,
    direction: usize,
    clock: f32,
    event: Option<f32>,
    hit_at: Option<f32>,
    resolved: bool,
    energy: u16,
    hp: u16,
    actions: u16,
    damage: u16,
    paid: u16,
}
impl State {
    fn new(skill: usize, case: Case, direction: usize) -> Self {
        Self {
            skill,
            case,
            direction,
            clock: 0.,
            event: None,
            hit_at: None,
            resolved: true,
            energy: 100,
            hp: 100,
            actions: 0,
            damage: 0,
            paid: 0,
        }
    }
    fn enabled(&self) -> bool {
        !matches!(self.case, Case::Without | Case::Outside)
    }
    fn magic(&self) -> bool {
        self.skill == 0 && self.case != Case::Outside
    }
    fn bonus(&self) -> u16 {
        if !self.enabled() {
            0
        } else if self.skill == 0 {
            parameter_for("A03", "magical_energy_cost_reduction_percent", 11).min(49)
        } else {
            parameter_for("M03", "transformed_damage_bonus_percent", 11)
        }
    }
    fn cost(&self) -> u16 {
        if !self.magic() {
            return 0;
        }
        let base = parameter_for("A01", "energy", 11);
        (base * (100 - self.bonus()))
            .div_ceil(100)
            .max(parameter_for("A03", "positive_payable_cost_floor", 11))
    }
    fn player(&self) -> GridPos {
        if self.magic() {
            GridPos::new(4, 5)
        } else {
            GridPos::new(4, 4)
        }
    }
    fn target(&self) -> GridPos {
        if self.magic() {
            GridPos::new(4, 2)
        } else {
            GridPos::new(4, 3)
        }
    }
    fn delay(&self) -> f32 {
        if self.magic() { 0.14 } else { 0.10 }
    }
    fn action(&mut self) {
        if !self.resolved || self.actions >= 3 || self.hp == 0 || self.energy < self.cost() {
            return;
        }
        self.paid = self.cost();
        self.energy -= self.paid;
        self.actions += 1;
        self.event = Some(self.clock);
        self.hit_at = None;
        self.resolved = false;
        self.damage = 0;
    }
    fn settle(&mut self) {
        if !self.resolved && self.event.is_some_and(|t| self.clock - t >= self.delay()) {
            let bonus = if self.skill == 1 { self.bonus() } else { 0 };
            self.damage = (20 + 20 * bonus / 100).min(self.hp);
            self.hp -= self.damage;
            self.hit_at = Some(self.event.unwrap() + self.delay());
            self.resolved = true;
        }
    }
    fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({"energy":self.energy,"target_hp":self.hp,"actions":self.actions,"damage":self.damage,"paid":self.paid,"bonus":self.bonus(),"resolved":self.resolved,"magic":self.magic(),"transformed":self.skill==1&&self.case!=Case::Outside,"passive_enabled":self.enabled(),"healed":0,"temporary_buff":false})
    }
}
struct Art {
    icons: [Texture2D; 2],
    body: Texture2D,
    bolt: Texture2D,
    claws: Texture2D,
    blood: Texture2D,
}
impl Art {
    fn load() -> Self {
        fn image(b: &[u8]) -> Texture2D {
            let t = Texture2D::from_file_with_format(b, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        }
        Self {
            icons: [
                image(include_bytes!(
                    "../../../assets/skills/agent/A03-compact.png"
                )),
                image(include_bytes!(
                    "../../../assets/skills/agent/M03-compact.png"
                )),
            ],
            body: image(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-39/predator-64.png"
            )),
            bolt: image(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-29/bolt-atlas.png"
            )),
            claws: image(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-40/claws-atlas.png"
            )),
            blood: image(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-40/blood-atlas.png"
            )),
        }
    }
    fn sheet(
        texture: &Texture2D,
        p: Vec2,
        size: f32,
        frame: usize,
        angle: f32,
        opacity: f32,
        anchor: Vec2,
    ) {
        draw_texture_ex(
            texture,
            p.x - anchor.x * size / 256.,
            p.y - anchor.y * size / 256.,
            alpha(WHITE, opacity),
            DrawTextureParams {
                source: Some(Rect::new(frame.min(5) as f32 * 256., 0., 256., 256.)),
                dest_size: Some(vec2(size, size)),
                rotation: angle,
                pivot: Some(p),
                ..Default::default()
            },
        );
    }
}
fn point(p: GridPos, dir: usize, rect: Rect) -> Vec2 {
    let p = position(p, dir);
    vec2(
        rect.x + (p.x as f32 + 0.5) * 64.,
        rect.y + (p.y as f32 + 0.5) * 64.,
    )
}
fn scene(art: &Art, clip: &Clip, s: &State, rect: Rect) {
    terrain(&Map::from_ascii(MAP).unwrap(), s.direction, rect, false);
    if s.actions == 0 {
        preview(&[s.target()], s.direction, rect, s.clock);
    }
    let start = point(s.player(), s.direction, rect);
    let target = point(s.target(), s.direction, rect);
    let impact_age = s.hit_at.map(|t| s.clock - t);
    let target = target
        + impact_age.map_or(Vec2::ZERO, |a| {
            crate::combat_visuals::hit_reaction_offset(target, start, f64::from(a), 64.)
        });
    enemy(target);
    let age = s.event.map(|t| s.clock - t);
    let lunge = if s.magic() {
        0.
    } else {
        age.map_or(0., |a| {
            if a < 0.22 {
                (a / 0.22 * std::f32::consts::PI).sin() * 6.
            } else {
                0.
            }
        })
    };
    let p = start + (target - start).normalize_or_zero() * lunge;
    if s.skill == 1 && s.case != Case::Outside {
        draw_ellipse(p.x, p.y + 29., 18., 4., 0., alpha(BLACK, 0.3));
        draw_texture_ex(
            &art.body,
            p.x - 32.,
            p.y - 32.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(64., 64.)),
                ..Default::default()
            },
        );
    } else {
        actor(p, player_visual());
    }
    if let Some(a) = age {
        if s.magic() && a < 0.36 {
            let origin = p + vec2(0., -8.);
            let end = target + vec2(0., -8.);
            let heading = end - origin;
            clip.apply_for(0, s.direction, rect);
            if a < 0.14 {
                Art::sheet(
                    &art.bolt,
                    origin.lerp(end, a / 0.14),
                    72.,
                    (a / 0.14 * 3.).floor() as usize,
                    heading.y.atan2(heading.x),
                    0.98,
                    vec2(232., 136.),
                );
            } else {
                Art::sheet(
                    &art.bolt,
                    end,
                    64.,
                    3 + ((a - 0.14) / 0.22 * 3.).floor() as usize,
                    heading.y.atan2(heading.x),
                    0.9 * ((0.36 - a) / 0.06).clamp(0., 1.),
                    vec2(128., 128.),
                );
            }
            gl_use_default_material();
        } else if !s.magic() && a < 0.33 {
            if s.skill == 1 && s.case != Case::Outside {
                Art::sheet(
                    &art.claws,
                    p.lerp(target, 0.75),
                    96.,
                    (a / 0.33 * 6.).floor() as usize,
                    (target - p).y.atan2((target - p).x),
                    0.9,
                    vec2(128., 128.),
                );
            } else if a < 0.15 {
                cut(
                    p.lerp(target, 0.65) + vec2(-15., -15.),
                    target + vec2(15., 15.),
                    5.,
                    1. - a / 0.15,
                );
            }
        }
    }
    if let Some(a) = impact_age.filter(|a| *a < 0.9) {
        if !s.magic() && a < 0.29 {
            Art::sheet(
                &art.blood,
                target,
                56.,
                (a / 0.29 * 6.).floor() as usize,
                0.,
                1. - a / 0.33,
                vec2(128., 128.),
            );
        }
        if s.magic() {
            let y = target.y - 43. - a * 14.;
            let opacity = ((0.9 - a) / 0.25).clamp(0., 1.);
            draw_rectangle(target.x - 48., y - 22., 96., 32., alpha(INK, 0.9 * opacity));
            draw_text_bold(
                s.damage.to_string(),
                target.x - 43.,
                y,
                22.,
                alpha(TEXT, opacity),
            );
            draw_text(
                format!("({}", s.damage),
                target.x - 12.,
                y,
                16.,
                alpha(TEXT, opacity),
            );
            draw_circle(target.x + 20., y - 6., 4., alpha(ENERGY, opacity));
            draw_text(")", target.x + 28., y, 16., alpha(TEXT, opacity));
        } else {
            super::protections::damage_label(target, s.damage, a, false);
        }
    }
    draw_text_bold(
        format!("PV cible {} / 100", s.hp),
        target.x - 66.,
        target.y + 48.,
        16.,
        TEXT,
    );
    draw_rectangle(
        rect.x + 8.,
        rect.bottom() - 156.,
        560.,
        148.,
        alpha(INK, 0.95),
    );
    let x = rect.x + 24.;
    let y = rect.bottom() - 115.;
    draw_rectangle(x - 3., y - 3., 206., 21., INK);
    draw_rectangle(x, y, 200., 15., Color::new(0.12, 0.06, 0.20, 1.));
    draw_rectangle(x, y, s.energy as f32 * 2., 15., ENERGY);
    draw_text_bold(format!("Énergie {} / 100", s.energy), x, y - 10., 18., TEXT);
    draw_texture_ex(
        &art.icons[s.skill],
        rect.right() - 84.,
        y - 10.,
        if s.enabled() {
            WHITE
        } else {
            alpha(WHITE, 0.35)
        },
        DrawTextureParams {
            dest_size: Some(vec2(38., 38.)),
            ..Default::default()
        },
    );
    let label = if s.skill == 0 {
        if s.magic() {
            format!(
                "Coût normal {} / payé {} énergie",
                parameter_for("A01", "energy", 11),
                s.cost()
            )
        } else {
            "Action physique · 0 énergie".into()
        }
    } else {
        format!(
            "Dégâts 20 + {} = {}",
            20 * s.bonus() / 100,
            20 + 20 * s.bonus() / 100
        )
    };
    draw_text_bold(label, x, y + 45., 18., TEXT);
    draw_text(
        format!(
            "{} actions · bonus {} % · passif permanent",
            s.actions,
            s.bonus()
        ),
        x,
        y + 73.,
        15.,
        DIM,
    );
    draw_text(
        if s.case == Case::Quiet {
            "Attente : aucune nouvelle action"
        } else {
            "Actions indépendantes préparées pour la revue"
        },
        x,
        y + 96.,
        14.,
        DIM,
    );
}
pub(crate) async fn run() {
    request_new_screen_size(1120., 820.);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let clip = Clip::load_maps_with_violet(&[MAP], GridPos::new(4, 5), 3.);
    let mut s = State::new(0, Case::Bonus, 2);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Passifs de puissance · ANIM-S40", 20., 34., 25., TEXT);
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
            (Case::Bonus, "Avec le passif"),
            (Case::Without, "Sans le passif"),
            (Case::Outside, "Action hors de son domaine"),
            (Case::Quiet, "Attente sans action"),
        ]
        .iter()
        .enumerate()
        {
            if button(
                Rect::new(20., 265. + i as f32 * 37., 330., 32.),
                title,
                true,
            ) {
                s = State::new(s.skill, *case, s.direction);
            }
        }
        if button(
            Rect::new(20., 435., 330., 34.),
            "Action préparée",
            s.resolved && s.actions < 3,
        ) || is_key_pressed(KeyCode::Space)
        {
            s.action();
        }
        if button(Rect::new(20., 480., 330., 34.), "Recommencer", true) {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(20., 525., 330., 34.),
            if paused { "Reprendre" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
        }
        s.settle();
        scene(&art, &clip, &s, Rect::new(400., 88., 576., 576.));
        draw_text(
            "89–90 · rang effectif 11 · aucun effet de campagne",
            400.,
            710.,
            16.,
            TEXT,
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
    let clip = Clip::load_maps_with_violet(&[MAP], GridPos::new(4, 5), 3.);
    let target = render_target(576, 576);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 576., 576.))
    };
    let mut entries = Vec::new();
    let cases = [
        (Case::Bonus, 0, "DROITE"),
        (Case::Bonus, 1, "GAUCHE"),
        (Case::Bonus, 2, "HAUT"),
        (Case::Bonus, 3, "BAS"),
        (Case::Without, 2, "SANS"),
        (Case::Outside, 2, "HORS"),
        (Case::Quiet, 2, "ATTENTE"),
    ];
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
                if frame == 12 || (case != Case::Quiet && [48, 84].contains(&frame)) {
                    s.action();
                }
                s.settle();
                timeline.push(s.snapshot());
                set_camera(&camera);
                clear_background(INK);
                scene(&art, &clip, &s, Rect::new(0., 0., 576., 576.));
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
            let columns = 4;
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
            entries.push(serde_json::json!({"id":IDS[skill],"number":89+skill,"title":TITLES[skill],"variant":format!("{}-A01-{label}",IDS[skill]),"folder":folder,"case":case.label(),"direction":direction,"frames":FRAMES,"fps":24,"indices":indices,"unique_images":known.len(),"native_columns":columns,"timeline":timeline}));
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
