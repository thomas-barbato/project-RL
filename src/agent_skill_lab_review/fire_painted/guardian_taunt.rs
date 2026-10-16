//! D66-TAUNT-A01/I01: visual review only. No campaign AI or balance integration.
//! Radius two and two turns are now accepted under D66-TAUNT01-NUM01.
use super::super::melee::actor;
use super::zones::{position, preview, terrain};
use super::*;
use project_rl::world::{Map, has_line_of_sight};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Write;

const OPEN: &str = "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########";
const WALL: &str = "#########\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#....#..#\n#.......#\n#########";
const FRAMES: usize = 120;
const FPS: usize = 24;
const CAST_FRAME: usize = 12;
const TURN_FRAMES: [usize; 2] = [54, 96];
const PLAYER: GridPos = GridPos::new(4, 7);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Case {
    Open,
    Wall,
    Multiple,
    Recall,
    Quiet,
}
impl Case {
    fn label(self) -> &'static str {
        match self {
            Self::Open => "LIBRE",
            Self::Wall => "CLOISON",
            Self::Multiple => "DEUX_GARDIENS",
            Self::Recall => "RAPPEL",
            Self::Quiet => "ATTENTE",
        }
    }
    fn title(self) -> &'static str {
        match self {
            Self::Open => "Zone ouverte",
            Self::Wall => "Derrière une cloison",
            Self::Multiple => "Deux Gardiens",
            Self::Recall => "Gardien rappelé",
            Self::Quiet => "Attente sans passer de tour",
        }
    }
}
struct State {
    case: Case,
    direction: usize,
    clock: f32,
    cast: Option<f32>,
    turn: u8,
    guardians: Vec<GridPos>,
    enemies: Vec<GridPos>,
    sources: Vec<Option<usize>>,
    remaining: Vec<u8>,
    recalled: bool,
}
impl State {
    fn new(case: Case, direction: usize) -> Self {
        let (guardians, enemies) = if case == Case::Multiple {
            (
                vec![GridPos::new(3, 5), GridPos::new(5, 5)],
                vec![GridPos::new(3, 3), GridPos::new(5, 3), GridPos::new(4, 4)],
            )
        } else if case == Case::Wall {
            (
                vec![GridPos::new(4, 5)],
                vec![GridPos::new(4, 3), GridPos::new(6, 5), GridPos::new(2, 5)],
            )
        } else {
            (
                vec![GridPos::new(4, 5)],
                vec![GridPos::new(3, 3), GridPos::new(5, 3), GridPos::new(6, 5)],
            )
        };
        let n = enemies.len();
        Self {
            case,
            direction,
            clock: 0.,
            cast: None,
            turn: 0,
            guardians,
            enemies,
            sources: vec![None; n],
            remaining: vec![0; n],
            recalled: false,
        }
    }
    fn map(&self) -> Map {
        Map::from_ascii(if self.case == Case::Wall { WALL } else { OPEN }).unwrap()
    }
    fn eligible(&self, from: GridPos, to: GridPos) -> bool {
        (from.x - to.x).abs().max((from.y - to.y).abs()) <= 2
            && has_line_of_sight(&self.map(), from, to, true)
    }
    fn launch(&mut self) {
        if self.cast.is_some() {
            return;
        }
        self.cast = Some(self.clock);
        self.assign();
    }
    fn assign(&mut self) {
        for i in 0..self.enemies.len() {
            if self.remaining[i] > 0 {
                continue;
            }
            // Squared direct distance and fixture index are display-only tie
            // choices. They do not define the pending campaign distance policy.
            self.sources[i] = self
                .guardians
                .iter()
                .enumerate()
                .filter(|(g, p)| !(*g == 0 && self.recalled) && self.eligible(**p, self.enemies[i]))
                .min_by_key(|(g, p)| {
                    let d = self.enemies[i];
                    ((p.x - d.x).pow(2) + (p.y - d.y).pow(2), *g)
                })
                .map(|(g, _)| g);
            self.remaining[i] = if self.sources[i].is_some() { 2 } else { 0 };
        }
    }
    fn advance(&mut self) {
        if self.cast.is_none() {
            return;
        }
        self.turn += 1;
        for i in 0..self.remaining.len() {
            self.remaining[i] = self.remaining[i].saturating_sub(1);
            if self.remaining[i] == 0 {
                self.sources[i] = None;
            }
        }
    }
    fn recall(&mut self) {
        self.recalled = true;
        for i in 0..self.sources.len() {
            if self.sources[i] == Some(0) {
                self.sources[i] = None;
                self.remaining[i] = 0;
            }
        }
    }
    fn cells(&self) -> Vec<GridPos> {
        let map = self.map();
        (1..8)
            .flat_map(|y| (1..8).map(move |x| GridPos::new(x, y)))
            .filter(|&p| map.is_walkable(p) && self.guardians.iter().any(|&g| self.eligible(g, p)))
            .collect()
    }
    fn snapshot(&self) -> serde_json::Value {
        serde_json::json!({"turn":self.turn,"launched":self.cast.is_some(),"preview":self.cast.is_none(),"sources":self.sources,"remaining":self.remaining,"icons":self.remaining.iter().filter(|&&r|r>0).count(),"recalled":self.recalled,"damage":0,"balance_fixture_not_accepted":true})
    }
}
fn point(at: GridPos, direction: usize, rect: Rect) -> Vec2 {
    let p = position(at, direction);
    vec2(
        rect.x + (p.x as f32 + 0.5) * 64.,
        rect.y + (p.y as f32 + 0.5) * 64.,
    )
}
fn player_visual() -> Visual {
    game_art::wear(
        game_art::player(Some("core:technomancien"), PlayerWeaponPose::Unarmed, None),
        [("core:body_armor".into(), "core:tenue_technique".into())],
    )
}
/// Transparent, upright 24px status symbol. No filled badge or portrait.
fn marker(p: Vec2, opacity: f32) {
    crate::guardian_taunt_visuals::marker(p, opacity);
}
struct Art {
    guardian: Texture2D,
}
impl Art {
    fn load() -> Self {
        let guardian = Texture2D::from_file_with_format(
            include_bytes!(
                "../../../assets/animations/agent/skill-animation-review-94/gardien-size01/D66-BODY02-SIZE01-native128.png"
            ),
            Some(ImageFormat::Png),
        );
        guardian.set_filter(FilterMode::Nearest);
        Self { guardian }
    }
    fn drone(&self, p: Vec2) {
        draw_ellipse(p.x, p.y + 30., 30., 4., 0., alpha(BLACK, 0.3));
        draw_texture(&self.guardian, p.x - 64., p.y - 64., WHITE);
    }
}
fn pulse(s: &State, source: GridPos, rect: Rect, age: f32) {
    if !(0.0..0.48).contains(&age) {
        return;
    }
    let map = s.map();
    let origin = vec2((source.x as f32 + 0.5) * 64., (source.y as f32 + 0.5) * 64.);
    let t = age / 0.48;
    let fade = (1. - t).powf(0.65);
    for ring in [0., 0.19] {
        let phase = (t - ring).max(0.) / (1. - ring);
        if phase <= 0. {
            continue;
        }
        let radius = 26. + phase * 128.;
        for n in 0..96 {
            let angle = n as f32 * std::f32::consts::TAU / 96.;
            let next = (n + 1) as f32 * std::f32::consts::TAU / 96.;
            let local = |a: f32| origin + vec2(a.cos(), a.sin()) * radius;
            let a = local(angle);
            let b = local(next);
            let free = |v: Vec2| {
                let at = GridPos::new((v.x / 64.).floor() as i32, (v.y / 64.).floor() as i32);
                map.is_walkable(at) && has_line_of_sight(&map, source, at, true)
            };
            if !free(a) || !free(b) {
                continue;
            }
            let screen = |p: Vec2| {
                let q = match s.direction {
                    0 => vec2(576. - p.y, p.x),
                    1 => vec2(p.y, 576. - p.x),
                    3 => vec2(576. - p.x, 576. - p.y),
                    _ => p,
                };
                q + vec2(rect.x, rect.y)
            };
            let (a, b) = (screen(a), screen(b));
            draw_line(a.x, a.y, b.x, b.y, 8., alpha(INK, fade * 0.65));
            draw_line(
                a.x,
                a.y,
                b.x,
                b.y,
                4.2,
                alpha(Color::from_rgba(213, 99, 42, 255), fade),
            );
            draw_line(
                a.x,
                a.y,
                b.x,
                b.y,
                1.4,
                alpha(Color::from_rgba(255, 210, 137, 255), fade),
            );
        }
    }
}
fn scene(art: &Art, s: &State, rect: Rect) {
    terrain(&s.map(), s.direction, rect, false);
    if s.cast.is_none() {
        preview(&s.cells(), s.direction, rect, s.clock);
    }
    if let Some(cast) = s.cast {
        for (i, &g) in s.guardians.iter().enumerate() {
            if !(s.recalled && i == 0) {
                pulse(s, g, rect, s.clock - cast);
            }
        }
    }
    let mut actors: Vec<(f32, Vec2, Option<usize>)> = s
        .guardians
        .iter()
        .enumerate()
        .filter(|(i, _)| !(s.recalled && *i == 0))
        .map(|(i, &p)| {
            let q = point(p, s.direction, rect);
            (q.y, q, Some(i))
        })
        .collect();
    for (i, &enemy) in s.enemies.iter().enumerate() {
        let mut p = point(enemy, s.direction, rect);
        if let (Some(cast), Some(g)) = (s.cast, s.sources[i]) {
            p += crate::combat_visuals::hit_reaction_offset(
                p,
                point(s.guardians[g], s.direction, rect),
                f64::from(s.clock - cast),
                64.,
            );
        }
        actors.push((p.y, p, None));
    }
    actors.sort_by(|a, b| a.0.total_cmp(&b.0));
    for (_, p, guardian) in actors {
        if guardian.is_some() {
            art.drone(p);
        } else {
            actor(
                p,
                Visual::Bestiary {
                    number: 12,
                    corpse: false,
                },
            );
        }
    }
    actor(point(PLAYER, s.direction, rect), player_visual());
    for (i, &at) in s.enemies.iter().enumerate() {
        if s.remaining[i] > 0 {
            let mut p = point(at, s.direction, rect);
            if let (Some(cast), Some(g)) = (s.cast, s.sources[i]) {
                p += crate::combat_visuals::hit_reaction_offset(
                    p,
                    point(s.guardians[g], s.direction, rect),
                    f64::from(s.clock - cast),
                    64.,
                );
            }
            marker(p + vec2(0., -46.), 1.);
        }
    }
}
fn cases() -> Vec<(Case, usize, &'static str)> {
    vec![
        (Case::Open, 0, "DROITE"),
        (Case::Open, 1, "GAUCHE"),
        (Case::Open, 2, "HAUT"),
        (Case::Open, 3, "BAS"),
        (Case::Wall, 2, "CLOISON"),
        (Case::Multiple, 2, "DEUX_GARDIENS"),
        (Case::Recall, 2, "RAPPEL"),
        (Case::Quiet, 2, "ATTENTE"),
    ]
}
pub(crate) async fn run() {
    request_new_screen_size(1080., 820.);
    for _ in 0..4 {
        next_frame().await;
    }
    let art = Art::load();
    let mut s = State::new(Case::Open, 2);
    let mut paused = false;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            break;
        }
        clear_background(INK);
        draw_text_bold("101 · Gardien : provocation", 20., 36., 25., TEXT);
        for (i, c) in [
            Case::Open,
            Case::Wall,
            Case::Multiple,
            Case::Recall,
            Case::Quiet,
        ]
        .iter()
        .enumerate()
        {
            if button(
                Rect::new(20., 70. + i as f32 * 38., 330., 32.),
                c.title(),
                true,
            ) {
                s = State::new(*c, s.direction);
            }
        }
        for (i, title) in ["Droite", "Gauche", "Haut", "Bas"].iter().enumerate() {
            if button(
                Rect::new(
                    20. + (i % 2) as f32 * 165.,
                    280. + (i / 2) as f32 * 38.,
                    155.,
                    32.,
                ),
                title,
                true,
            ) {
                s = State::new(s.case, i);
            }
        }
        if button(
            Rect::new(20., 390., 330., 34.),
            "Provoquer",
            s.cast.is_none(),
        ) || is_key_pressed(KeyCode::Enter)
        {
            s.launch();
        }
        if button(
            Rect::new(20., 434., 330., 34.),
            "Passer un tour",
            s.cast.is_some(),
        ) || is_key_pressed(KeyCode::Space)
        {
            s.advance();
        }
        if button(
            Rect::new(20., 478., 330., 34.),
            "Rappeler le premier Gardien",
            !s.recalled && s.cast.is_some(),
        ) {
            s.recall();
        }
        if button(Rect::new(20., 522., 155., 34.), "Rejouer", true) {
            s = State::new(s.case, s.direction);
        }
        if button(
            Rect::new(185., 522., 165., 34.),
            if paused { "Reprendre" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if !paused {
            s.clock += get_frame_time().min(0.05);
        }
        scene(&art, &s, Rect::new(400., 80., 576., 576.));
        draw_text_bold(
            format!(
                "Tour {} · {} cibles provoquées",
                s.turn,
                s.remaining.iter().filter(|&&r| r > 0).count()
            ),
            400.,
            704.,
            20.,
            TEXT,
        );
        draw_text(
            "Aperçu visuel : 2 tours et rayon 2 sont des valeurs d’essai.",
            400.,
            740.,
            17.,
            DIM,
        );
        draw_text(
            "Le combat autonome de la campagne reste à intégrer.",
            400.,
            770.,
            17.,
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
    let icon_target = render_target(32, 32);
    set_camera(&Camera2D {
        render_target: Some(icon_target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 32., 32.))
    });
    clear_background(Color::new(0., 0., 0., 0.));
    marker(vec2(16., 16.), 1.);
    unsafe {
        macroquad::window::get_internal_gl().flush();
    }
    icon_target
        .texture
        .get_texture_data()
        .export_png(output.join("D66-TAUNT-I01-native32.png").to_str().unwrap());
    set_default_camera();
    next_frame().await;
    let target = render_target(576, 576);
    let camera = Camera2D {
        render_target: Some(target.clone()),
        ..Camera2D::from_display_rect(Rect::new(0., 0., 576., 576.))
    };
    let mut entries = Vec::new();
    for (case, direction, label) in cases() {
        let mut s = State::new(case, direction);
        let mut images = Vec::new();
        let mut known = BTreeMap::new();
        let mut indices = Vec::new();
        let mut timeline = Vec::new();
        for frame in 0..FRAMES {
            s.clock = frame as f32 / FPS as f32;
            if frame == CAST_FRAME {
                s.launch();
            }
            if case != Case::Quiet && TURN_FRAMES.contains(&frame) {
                s.advance();
                if case == Case::Recall && frame == TURN_FRAMES[0] {
                    s.recall();
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
            let h = hash.finish();
            let index = if let Some(&n) = known.get(&h) {
                n
            } else {
                let n = images.len();
                known.insert(h, n);
                images.push(im);
                n
            };
            indices.push(index);
            set_default_camera();
            if frame % 8 == 0 {
                next_frame().await;
            }
        }
        let folder = output.join(label);
        std::fs::create_dir_all(&folder).map_err(|e| e.to_string())?;
        let columns = 6;
        let rows = images.len().div_ceil(columns);
        let mut atlas = Image::gen_image_color((576 * columns) as u16, (576 * rows) as u16, INK);
        for (i, im) in images.into_iter().enumerate() {
            for y in 0..576 {
                let k = (((i / columns) * 576 + y) * 576 * columns + (i % columns) * 576) * 4;
                atlas.bytes[k..k + 576 * 4]
                    .copy_from_slice(&im.bytes[y * 576 * 4..(y + 1) * 576 * 4]);
            }
        }
        let mut file = std::io::BufWriter::new(
            std::fs::File::create(folder.join("atlas.ppm")).map_err(|e| e.to_string())?,
        );
        write!(file, "P6\n{} {}\n255\n", atlas.width, atlas.height).map_err(|e| e.to_string())?;
        let mut row = Vec::with_capacity(usize::from(atlas.width) * 3);
        for rgba in atlas.bytes.chunks_exact(usize::from(atlas.width) * 4).rev() {
            row.clear();
            for p in rgba.chunks_exact(4) {
                row.extend_from_slice(&p[..3]);
            }
            file.write_all(&row).map_err(|e| e.to_string())?;
        }
        file.flush().map_err(|e| e.to_string())?;
        entries.push(serde_json::json!({"id":"D66","variant":format!("D66-TAUNT-A01-{label}"),"number":101,"case":case.label(),"label":label,"title":case.title(),"direction":direction,"frames":FRAMES,"fps":FPS,"cast_frame":CAST_FRAME,"turn_frames":TURN_FRAMES,"indices":indices,"unique_images":known.len(),"columns":columns,"timeline":timeline}));
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
    fn guardian_taunt_is_stable_and_non_stacking() {
        let mut s = State::new(Case::Multiple, 2);
        s.launch();
        assert_eq!(s.sources, vec![Some(0), Some(1), Some(0)]);
        s.advance();
        let before = s.snapshot();
        s.assign();
        assert_eq!(s.snapshot(), before);
        s.clock = 200.;
        assert_eq!(s.snapshot(), before);
        s.advance();
        assert!(s.remaining.iter().all(|&r| r == 0));
    }
    #[test]
    fn guardian_taunt_wall_recall_and_preview_lifecycle() {
        let mut s = State::new(Case::Wall, 2);
        assert_eq!(s.snapshot()["preview"], true);
        s.launch();
        assert_eq!(s.sources, vec![Some(0), None, Some(0)]);
        assert_eq!(s.snapshot()["preview"], false);
        s.recall();
        assert!(s.sources.iter().all(Option::is_none));
        assert!(s.remaining.iter().all(|&r| r == 0));
    }
}
