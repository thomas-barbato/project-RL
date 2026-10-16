//! ANIM-S41: targeted weakening, isolated prepared outcomes; no campaign changes.
use super::super::melee::{actor, player_visual};
use super::zones::{Clip, position, preview, terrain};
use super::*;
use project_rl::world::Map;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const IDS: [&str; 2] = ["O02", "AF02"];
const TITLES: [&str; 2] = ["Interférence", "Faille d’armure"];
const OPEN: &str = "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########";
const WALL: &str = "#########\n#.......#\n#.......#\n#.#####.#\n#.......#\n#.......#\n#.......#\n#.......#\n#########";
const FRAMES: usize = 112;
const PURPLE: Color = Color::new(0.76, 0.24, 1., 1.);
const RUST: Color = Color::new(0.96, 0.40, 0.13, 1.);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Case {
    Target,
    Player,
    Quiet,
    Wall,
    Resist,
}
impl Case {
    fn label(self) -> &'static str {
        match self {
            Self::Target => "CIBLE",
            Self::Player => "JOUEUR",
            Self::Quiet => "ATTENTE",
            Self::Wall => "MUR",
            Self::Resist => "RESISTE",
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
        }
    }
    fn parameter(&self, key: &str) -> u16 {
        parameter_for(IDS[self.skill], key, 1)
    }
    fn can_cast(&self) -> bool {
        self.case != Case::Wall && self.cooldown == 0 && self.energy >= self.parameter("energy")
    }
    fn cast(&mut self) {
        if !self.can_cast() {
            return;
        }
        self.energy -= self.parameter("energy");
        self.cooldown = self.parameter("cooldown_turns");
        self.cast_at = Some(self.clock);
        if self.case != Case::Resist {
            self.remaining = self.parameter("duration_rank_turns");
            self.hit_at = Some(self.clock);
        }
    }
    fn advance(&mut self) {
        if self.cast_at.is_none() {
            return;
        }
        self.turn += 1;
        self.remaining = self.remaining.saturating_sub(1);
        self.cooldown = self.cooldown.saturating_sub(1);
    }
    fn target(&self) -> GridPos {
        GridPos::new(4, if self.case == Case::Player { 5 } else { 2 })
    }
    fn caster(&self) -> GridPos {
        GridPos::new(4, if self.case == Case::Player { 2 } else { 5 })
    }
    fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({"turn":self.turn,"remaining":self.remaining,"cooldown":self.cooldown,"energy":self.energy,"cast":self.cast_at.is_some(),"applied":self.hit_at.is_some(),"target_player":self.case==Case::Player,"preview":self.cast_at.is_none()&&self.can_cast(),"icon_visible":self.remaining>0,"damage":0,"hp":100,"reduction":self.parameter(if self.skill==0 {"direct_damage_reduction_percent"} else {"physical_armor_reduction_percent"})})
    }
}
pub(super) struct Art {
    icons: [Texture2D; 2],
    bolt: Texture2D,
}
impl Art {
    pub(super) fn load() -> Self {
        fn load(b: &[u8]) -> Texture2D {
            let t = Texture2D::from_file_with_format(b, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        }
        Self {
            icons: [
                load(include_bytes!(
                    "../../../assets/skills/agent/O02-compact.png"
                )),
                load(include_bytes!(
                    "../../../assets/skills/agent/AF02-compact.png"
                )),
            ],
            bolt: load(include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-29/bolt-atlas.png"
            )),
        }
    }
    fn flash(&self, p: Vec2, age: f32) {
        self.flash_scaled(p, age, 112.);
    }
    pub(super) fn flash_scaled(&self, p: Vec2, age: f32, size: f32) {
        let frame = 3 + ((age / 0.34 * 3.).floor() as usize).min(2);
        draw_texture_ex(
            &self.bolt,
            p.x - size / 2.,
            p.y - size / 2.,
            alpha(WHITE, ((0.34 - age) / 0.10).clamp(0., 1.) * 0.85),
            DrawTextureParams {
                source: Some(Rect::new(frame as f32 * 256., 0., 256., 256.)),
                dest_size: Some(vec2(size, size)),
                ..Default::default()
            },
        );
    }
    pub(super) fn icon(&self, skill: usize, p: Vec2, size: f32) {
        draw_texture_ex(
            &self.icons[skill],
            p.x - size / 2.,
            p.y - size / 2.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(size, size)),
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
fn stroke(a: Vec2, b: Vec2, width: f32, color: Color, opacity: f32) {
    draw_line(a.x, a.y, b.x, b.y, width + 2.3, alpha(INK, opacity * 0.9));
    draw_line(a.x, a.y, b.x, b.y, width, alpha(color, opacity));
}
pub(super) fn weakening(skill: usize, p: Vec2, clock: f32, age: f32) {
    let fade = (age / 0.12).clamp(0., 1.);
    if skill == 0 {
        // Broken occult/electronic arcs, outside the body. Eight slow phases;
        // no timed expiration and no motion implying a repeated application.
        let phase = (clock * 5.).floor() as i32;
        for side in [-1., 1.] {
            for i in 0..3 {
                let shift = if (phase + i) % 3 == 0 { 3. } else { 0. };
                let y = p.y - 19. + i as f32 * 17.;
                let a = p + vec2(side * (25. + shift), y - p.y);
                let b = a + vec2(side * 8., -4.);
                let c = b + vec2(side * 5., 6.);
                stroke(a, b, 3.2, PURPLE, fade * 0.85);
                stroke(b, c, 2.4, PURPLE, fade * 0.72);
                if (phase + i) % 3 == 0 {
                    stroke(
                        c,
                        c + vec2(7. * side, 0.),
                        2.1,
                        Color::new(0.25, 0.80, 0.84, 1.),
                        fade * 0.8,
                    );
                }
            }
        }
    } else {
        // Rust-coloured fractures across the existing armor, no silhouette swap.
        let paths: [&[(f32, f32)]; 3] = [
            &[
                (-17., -19.),
                (-7., -10.),
                (-12., -1.),
                (-1., 8.),
                (-5., 19.),
            ],
            &[(16., -13.), (7., -4.), (12., 4.), (5., 12.), (10., 23.)],
            &[(-10., -3.), (-20., 1.), (-23., 10.)],
        ];
        for path in paths {
            for line in path.windows(2) {
                stroke(
                    p + vec2(line[0].0, line[0].1) * 0.65,
                    p + vec2(line[1].0, line[1].1) * 0.65,
                    1.8,
                    RUST,
                    fade * 0.9,
                );
            }
        }
        // A few unconnected flakes; never animate the target armor itself.
        for i in 0..5 {
            let t = (clock * 0.45 + i as f32 * 0.21).fract();
            let x = if i % 2 == 0 { -21. } else { 20. };
            let q = p + vec2(x + i as f32 % 3., -8. + t * 38.);
            draw_triangle(
                q,
                q + vec2(3., 1.),
                q + vec2(1., 4.),
                alpha(RUST, fade * 0.7 * (1. - t)),
            );
        }
    }
}
fn scene(art: &Art, clip: &Clip, s: &State, rect: Rect) {
    let map = Map::from_ascii(if s.case == Case::Wall { WALL } else { OPEN }).unwrap();
    terrain(&map, s.direction, rect, false);
    if s.cast_at.is_none() && s.can_cast() {
        preview(&[s.target()], s.direction, rect, s.clock);
    }
    let caster = point(s.caster(), s.direction, rect);
    let base = point(s.target(), s.direction, rect);
    let age = s.hit_at.map(|t| s.clock - t);
    let target = base
        + age.map_or(Vec2::ZERO, |a| {
            crate::combat_visuals::hit_reaction_offset(base, caster, f64::from(a), 64.)
        });
    if s.case == Case::Player {
        enemy(caster);
        actor(target, player_visual());
    } else {
        actor(caster, player_visual());
        enemy(target);
    }
    if let Some(a) = s.cast_at.map(|t| s.clock - t) {
        if s.case != Case::Resist && a < 0.34 {
            clip.apply_for(if s.case == Case::Wall { 1 } else { 0 }, s.direction, rect);
            art.flash(target, a);
            gl_use_default_material();
        }
        if s.case == Case::Resist && a < 0.8 {
            draw_text_bold(
                "Résiste",
                target.x - 28.,
                target.y - 50. - a * 10.,
                19.,
                TEXT,
            );
        }
    }
    if s.remaining > 0 {
        weakening(s.skill, target, s.clock, age.unwrap_or(0.));
        let badge = target + vec2(0., -57.);
        draw_rectangle(badge.x - 20., badge.y - 20., 40., 40., alpha(INK, 0.94));
        art.icon(s.skill, badge, 36.);
        draw_text_bold(
            s.remaining.to_string(),
            badge.x + 16.,
            badge.y + 19.,
            17.,
            TEXT,
        );
    }
    draw_rectangle(
        rect.x + 8.,
        rect.bottom() - 116.,
        560.,
        108.,
        alpha(INK, 0.96),
    );
    let x = rect.x + 24.;
    let y = rect.bottom() - 96.;
    let label = if s.case == Case::Wall {
        "Obstacle : cible inaccessible".to_string()
    } else if s.case == Case::Resist {
        "Résultat préparé : la cible résiste".to_string()
    } else if s.remaining > 0 {
        format!("{} : {} tours", TITLES[s.skill], s.remaining)
    } else if s.cast_at.is_some() {
        "Affaiblissement terminé".to_string()
    } else {
        "Cible visible · avant lancement".to_string()
    };
    draw_text_bold(label, x, y, 21., TEXT);
    draw_text(
        format!("Énergie {} / 100 · recharge {} tours", s.energy, s.cooldown),
        x,
        y + 29.,
        18.,
        TEXT,
    );
    draw_text(
        format!("Tours passés {} · PV cible 100 / 100", s.turn),
        x,
        y + 55.,
        17.,
        TEXT,
    );
    draw_text(
        if s.case == Case::Quiet {
            "Attente réelle : aucun tour écoulé"
        } else {
            "Les tours avancent uniquement par les étapes du lecteur"
        },
        x,
        y + 80.,
        14.,
        DIM,
    );
}
fn cases() -> [(Case, usize, &'static str); 8] {
    [
        (Case::Target, 0, "DROITE"),
        (Case::Target, 1, "GAUCHE"),
        (Case::Target, 2, "HAUT"),
        (Case::Target, 3, "BAS"),
        (Case::Player, 2, "JOUEUR"),
        (Case::Quiet, 2, "ATTENTE"),
        (Case::Wall, 2, "MUR"),
        (Case::Resist, 2, "RESISTE"),
    ]
}
pub(crate) async fn run() {
    request_new_screen_size(1120., 820.);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let clip = Clip::load_maps_with_violet(&[OPEN, WALL], GridPos::new(4, 5), 3.);
    let mut s = State::new(0, Case::Target, 2);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("Affaiblissements · ANIM-S41", 20., 35., 25., TEXT);
        for (i, title) in TITLES.iter().enumerate() {
            if button(Rect::new(20., 70. + i as f32 * 40., 330., 34.), title, true) {
                s = State::new(i, s.case, s.direction);
            }
        }
        for (i, title) in ["Droite", "Gauche", "Haut", "Bas"].iter().enumerate() {
            if button(
                Rect::new(
                    20. + (i % 2) as f32 * 166.,
                    170. + (i / 2) as f32 * 38.,
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
            (Case::Target, "Cible ennemie"),
            (Case::Player, "Joueur touché"),
            (Case::Quiet, "Attendre sans passer de tour"),
            (Case::Wall, "Obstacle"),
            (Case::Resist, "Cible résistante"),
        ]
        .iter()
        .enumerate()
        {
            if button(
                Rect::new(20., 260. + i as f32 * 37., 330., 32.),
                title,
                true,
            ) {
                s = State::new(s.skill, *case, s.direction);
            }
        }
        if button(Rect::new(20., 465., 330., 34.), "Lancer", s.can_cast())
            || is_key_pressed(KeyCode::Space)
        {
            s.cast();
        }
        if button(
            Rect::new(20., 507., 330., 34.),
            "Passer un tour",
            s.cast_at.is_some(),
        ) {
            s.advance();
        }
        if button(Rect::new(20., 549., 158., 34.), "Rejouer", true) {
            s = State::new(s.skill, s.case, s.direction);
        }
        if button(
            Rect::new(186., 549., 164., 34.),
            if paused { "Reprendre" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
        }
        scene(&art, &clip, &s, Rect::new(400., 88., 576., 576.));
        draw_text(
            "91–92 · rang 1 · aucun effet de campagne",
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
    let clip = Clip::load_maps_with_violet(&[OPEN, WALL], GridPos::new(4, 5), 3.);
    let target = render_target(576, 576);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 576., 576.))
    };
    let mut entries = Vec::new();
    for skill in 0..2 {
        for (case, direction, label) in cases() {
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
                if case != Case::Quiet && [48, 72, 96].contains(&frame) {
                    s.advance();
                }
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
            entries.push(serde_json::json!({"id":IDS[skill],"number":91+skill,"title":TITLES[skill],"variant":format!("{}-A01-{label}",IDS[skill]),"folder":folder,"case":case.label(),"direction":direction,"frames":FRAMES,"fps":24,"indices":indices,"unique_images":known.len(),"native_columns":columns,"timeline":timeline}));
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
    #[test]
    fn afflictions_duration_is_turn_based_and_refusals_are_atomic() {
        for skill in 0..2 {
            let mut s = State::new(skill, Case::Target, 2);
            s.cast();
            let energy = s.energy;
            assert_eq!(s.remaining, 3);
            assert_eq!(s.cooldown, 1);
            s.clock += 600.;
            assert_eq!(s.remaining, 3);
            assert_eq!(s.energy, energy);
            s.cast();
            assert_eq!(s.energy, energy);
            for n in (0..3).rev() {
                s.advance();
                assert_eq!(s.remaining, n);
            }
            assert_eq!(s.cooldown, 0);
            let mut wall = State::new(skill, Case::Wall, 2);
            wall.cast();
            wall.advance();
            assert_eq!(wall.energy, 100);
            assert_eq!(wall.turn, 0);
            assert_eq!(wall.remaining, 0);
            let mut resisted = State::new(skill, Case::Resist, 2);
            resisted.cast();
            assert!(resisted.energy < 100);
            assert_eq!(resisted.remaining, 0);
            assert!(resisted.hit_at.is_none());
        }
    }
}
