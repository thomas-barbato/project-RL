//! ANIM-PART01: complete animated fragments over cached rounded visual paths.
//! No whole-cone bitmap, shaders, gameplay change or per-cell alpha clipping.
use super::*;
use std::collections::{BTreeSet, VecDeque};

const EMITTER: Vec2 = Vec2::new(114.0, 219.0);
const ALPHA_RADIUS: f32 = 31.0;
const CASES: [&str; 3] = ["COULOIR", "CLOISON", "OBSTACLE"];

fn variant(case: usize, left: bool) -> String {
    format!(
        "FE02-A05-{}-{}",
        CASES[case],
        if left { "GAUCHE" } else { "DROITE" }
    )
}
fn mirrored(at: Vec2, left: bool) -> Vec2 {
    if left { vec2(448.0 - at.x, at.y) } else { at }
}
fn mirrored_cell(at: GridPos, left: bool) -> GridPos {
    if left {
        GridPos::new(6 - at.x, at.y)
    } else {
        at
    }
}
fn walls(map: &Map) -> Vec<Rect> {
    (0..7)
        .flat_map(|y| (0..7).map(move |x| GridPos::new(x, y)))
        .filter(|&at| map.blocks_vision(at))
        .map(|at| Rect::new(at.x as f32 * TILE, at.y as f32 * TILE, TILE, TILE))
        .collect()
}
fn clear(point: Vec2, radius: f32, walls: &[Rect]) -> bool {
    walls.iter().all(|wall| {
        let nearest = vec2(
            point.x.clamp(wall.x, wall.x + wall.w),
            point.y.clamp(wall.y, wall.y + wall.h),
        );
        point.distance_squared(nearest) > radius * radius
    })
}

/// These paths shape only the visual stream, not monster AI or damage propagation.
fn route(allowed: &BTreeSet<GridPos>, target: GridPos) -> Vec<GridPos> {
    let mut parents = BTreeMap::new();
    parents.insert(ORIGIN, ORIGIN);
    let mut queue = VecDeque::from([ORIGIN]);
    while let Some(at) = queue.pop_front() {
        if at == target {
            break;
        }
        for delta in [(1, 0), (0, -1), (0, 1), (-1, 0)] {
            let next = GridPos::new(at.x + delta.0, at.y + delta.1);
            if allowed.contains(&next) && !parents.contains_key(&next) {
                parents.insert(next, at);
                queue.push_back(next);
            }
        }
    }
    if !parents.contains_key(&target) {
        return vec![];
    }
    let mut nodes = vec![target];
    while *nodes.last().unwrap() != ORIGIN {
        nodes.push(parents[nodes.last().unwrap()]);
    }
    nodes.reverse();
    nodes
}

struct Stream {
    points: Vec<Vec2>,
    distances: Vec<f32>,
    length: f32,
    seed: u32,
}
impl Stream {
    fn new(nodes: &[GridPos], seed: u32) -> Self {
        let mut raw: Vec<_> = nodes
            .iter()
            .map(|p| vec2(p.x as f32 + 0.5, p.y as f32 + 0.5) * TILE)
            .collect();
        raw[0] = EMITTER;
        let mut points = vec![EMITTER];
        let line = |points: &mut Vec<Vec2>, end: Vec2| {
            let start = *points.last().unwrap();
            let steps = (start.distance(end) / 7.0).ceil().max(1.0) as usize;
            for step in 1..=steps {
                points.push(start.lerp(end, step as f32 / steps as f32));
            }
        };
        for index in 1..raw.len().saturating_sub(1) {
            let at = raw[index];
            let incoming = (at - raw[index - 1]).normalize_or_zero();
            let outgoing = (raw[index + 1] - at).normalize_or_zero();
            let start = at - incoming * 18.0;
            let end = at + outgoing * 18.0;
            line(&mut points, start);
            for step in 1..=8 {
                let t = step as f32 / 8.0;
                points.push(start * (1.0 - t).powi(2) + at * (2.0 * t * (1.0 - t)) + end * t * t);
            }
        }
        line(&mut points, *raw.last().unwrap());
        let mut distances = vec![0.0];
        for pair in points.windows(2) {
            distances.push(distances.last().unwrap() + pair[0].distance(pair[1]));
        }
        let length = *distances.last().unwrap();
        Self {
            points,
            distances,
            length,
            seed,
        }
    }
    fn at(&self, distance: f32) -> (Vec2, Vec2) {
        let distance = distance.clamp(0.0, self.length - 0.001);
        let index = self
            .distances
            .partition_point(|&d| d <= distance)
            .max(1)
            .min(self.points.len() - 1);
        let start = self.distances[index - 1];
        let span = self.distances[index] - start;
        let point =
            self.points[index - 1].lerp(self.points[index], (distance - start) / span.max(0.001));
        let tangent = (self.points[index] - self.points[index - 1]).normalize_or_zero();
        (point, tangent)
    }
}

struct Flow {
    fixture: Fixture,
    walls: Vec<Rect>,
    streams: Vec<Stream>,
}
impl Flow {
    fn new(case: usize) -> Self {
        let fixture = Fixture::new(case);
        let allowed: BTreeSet<_> = fixture.affected.iter().copied().chain([ORIGIN]).collect();
        let farthest = fixture.affected.iter().map(|p| p.x).max().unwrap();
        let streams = fixture
            .affected
            .iter()
            .filter(|p| p.x == farthest)
            .enumerate()
            .filter_map(|(index, &target)| {
                let nodes = route(&allowed, target);
                (nodes.len() > 1).then(|| Stream::new(&nodes, index as u32 * 19))
            })
            .collect();
        let walls = walls(&fixture.map);
        Self {
            fixture,
            walls,
            streams,
        }
    }
    fn particles(&self, age: f32) -> Vec<(Vec2, f32, f32, f32, u32)> {
        let mut particles = Vec::new();
        for stream in &self.streams {
            // Prefill the whole stream at launch: travel is purely visual.
            let count = (stream.length / 13.0).ceil() as usize;
            for index in 0..count {
                let seed = stream.seed + index as u32 * 7;
                let distance = (age * 118.0
                    + index as f32 * stream.length / count as f32
                    + noise(seed) * 11.0)
                    % stream.length;
                let (base, tangent) = stream.at(distance);
                let normal = vec2(-tangent.y, tangent.x);
                let wobble = (age * 6.0 + distance * 0.043 + seed as f32).sin() * 7.0;
                let candidate = base + normal * wobble;
                let mut size = 40.0 + noise(seed + 3) * 12.0;
                let radius = ALPHA_RADIUS * size / 64.0 + 1.5;
                let point = if clear(candidate, radius, &self.walls) {
                    candidate
                } else {
                    base
                };
                if !clear(point, radius, &self.walls) {
                    size *= 0.80;
                    if !clear(point, ALPHA_RADIUS * size / 64.0 + 1.5, &self.walls) {
                        continue;
                    }
                }
                let remaining = stream.length - distance;
                let opacity = (remaining / 30.0).clamp(0.0, 1.0) * 0.87;
                particles.push((point, tangent.y.atan2(tangent.x), size, opacity, seed));
            }
        }
        particles
    }
}

struct ReviewParticles {
    texture: Texture2D,
    burn: DrawnFire,
    flows: Vec<Flow>,
}
impl ReviewParticles {
    fn load() -> Self {
        let image = Image::from_file_with_format(
            include_bytes!(
                "../../../../assets/animations/agent/skill-animation-review-09/particle-atlas.png"
            ),
            Some(ImageFormat::Png),
        )
        .unwrap();
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Linear);
        Self {
            texture,
            burn: DrawnFire::load(),
            flows: (0..3).map(Flow::new).collect(),
        }
    }
    fn draw(&self, case: usize, left: bool, rect: Rect, age: f32) {
        for (point, angle, size, opacity, seed) in self.flows[case].particles(age) {
            let point = mirrored(point, left) + vec2(rect.x, rect.y);
            let frame = ((age * 12.0) as u32 + seed) % 8;
            // Mirror the drawing and negate its rotation. The body stays frontal.
            draw_texture_ex(
                &self.texture,
                point.x - size * 0.5,
                point.y - size * 0.5,
                alpha(WHITE, opacity),
                DrawTextureParams {
                    source: Some(Rect::new(frame as f32 * 64.0, 0.0, 64.0, 64.0)),
                    dest_size: Some(vec2(size, size)),
                    rotation: if left { -angle } else { angle },
                    flip_x: left,
                    ..Default::default()
                },
            );
        }
    }
}

fn scene(
    review: &ReviewParticles,
    case: usize,
    left: bool,
    rect: Rect,
    age: f32,
    show_effect: bool,
) {
    let fixture = &review.flows[case].fixture;
    for y in 0..7 {
        for x in 0..7 {
            let source = GridPos::new(x, y);
            let at = mirrored_cell(source, left);
            let p = cell(rect, at.x as f32, at.y as f32);
            game_art::draw_world(
                Rect::new(p.x, p.y, TILE, TILE),
                Visual::Terrain {
                    kind: if fixture.map.blocks_vision(source) {
                        crate::test_sector::Decor::Wall
                    } else {
                        crate::test_sector::Decor::Deck
                    },
                    position: source,
                },
                1.0,
            );
            draw_rectangle_lines(
                p.x,
                p.y,
                TILE,
                TILE,
                0.7,
                Color::new(0.15, 0.18, 0.19, 0.75),
            );
        }
    }
    let aiming = age < 0.7;
    if aiming {
        for &source in &fixture.affected {
            let at = mirrored_cell(source, left);
            let p = cell(rect, at.x as f32, at.y as f32);
            draw_rectangle(
                p.x + 2.0,
                p.y + 2.0,
                60.0,
                60.0,
                Color::new(0.9, 0.45, 0.06, 0.18),
            );
            draw_rectangle_lines(
                p.x + 2.0,
                p.y + 2.0,
                60.0,
                60.0,
                1.0,
                Color::new(0.9, 0.55, 0.12, 0.65),
            );
        }
    } else if show_effect {
        review.draw(case, left, rect, age - 0.7);
    }
    for (index, &source) in fixture.targets.iter().enumerate() {
        let at = mirrored_cell(source, left);
        let p = cell(rect, at.x as f32 + 0.5, at.y as f32 + 0.5);
        enemy(p);
        if !aiming && show_effect && fixture.affected.contains(&source) {
            review.burn.burn(p, age - 0.7, index as u32 * 2, 0.9);
        }
        draw_text_bold(
            char::from(b'A' + index as u8).to_string(),
            p.x - 4.0,
            p.y - 28.0,
            13.0,
            TEXT,
        );
    }
    let origin = mirrored_cell(ORIGIN, left);
    let player = cell(rect, origin.x as f32 + 0.5, origin.y as f32 + 0.5);
    actor(player, caster());
    if !aiming {
        name(0, player, age - 0.7);
    }
    draw_rectangle(
        rect.x + 7.0,
        rect.y + rect.h - 28.0,
        rect.w - 14.0,
        22.0,
        alpha(INK, 0.94),
    );
    draw_text(
        if aiming {
            "Aperçu de ciblage · cases touchées"
        } else {
            SCENARIOS[case].2
        },
        rect.x + 14.0,
        rect.y + rect.h - 12.0,
        14.0,
        TEXT,
    );
}

pub(crate) async fn run() {
    request_new_screen_size(1060.0, 670.0);
    let review = ReviewParticles::load();
    let mut case = 0;
    let mut left = false;
    let mut aiming = true;
    let mut age = 0.0;
    let mut paused = false;
    let mut speed = 1.0;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold("LABORATOIRE / SOUFFLE MODULAIRE", 22.0, 35.0, 23.0, TEXT);
        draw_text(
            "ANIM-PART01 · fragments animés · cases natives de 64 px",
            22.0,
            62.0,
            14.0,
            DIM,
        );
        for (index, (_, title, _)) in SCENARIOS.iter().enumerate() {
            if button(
                Rect::new(20.0, 96.0 + index as f32 * 46.0, 270.0, 35.0),
                title,
                true,
            ) {
                case = index;
                aiming = true;
                age = 0.0;
            }
        }
        if button(
            Rect::new(20.0, 255.0, 270.0, 35.0),
            if left {
                "Direction : gauche"
            } else {
                "Direction : droite"
            },
            true,
        ) {
            left = !left;
            aiming = true;
            age = 0.0;
        }
        scene(
            &review,
            case,
            left,
            Rect::new(320.0, 96.0, 448.0, 448.0),
            if aiming { 0.0 } else { age + 0.7 },
            true,
        );
        draw_text(variant(case, left), 320.0, 567.0, 14.0, DIM);
        if button(Rect::new(20.0, 596.0, 115.0, 34.0), "Lancer", true)
            || is_key_pressed(KeyCode::Enter)
        {
            aiming = false;
            age = 0.0;
            paused = false;
        }
        if button(Rect::new(145.0, 596.0, 115.0, 34.0), "Viser", true) {
            aiming = true;
            age = 0.0;
        }
        if button(
            Rect::new(270.0, 596.0, 115.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if button(
            Rect::new(395.0, 596.0, 135.0, 34.0),
            if speed == 1.0 {
                "Vitesse ×1"
            } else {
                "Vitesse ×0,5"
            },
            true,
        ) {
            speed = if speed == 1.0 { 0.5 } else { 1.0 };
        }
        draw_text(
            "Choisir une direction · vérifier l’aperçu · lancer · aucun tour simulé",
            22.0,
            655.0,
            14.0,
            DIM,
        );
        if !paused && !aiming {
            age += get_frame_time() * speed;
        }
        next_frame().await;
    }
}

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    if output.exists() {
        return Err("Le dossier de revue existe déjà.".into());
    }
    std::fs::create_dir_all(output).map_err(|error| error.to_string())?;
    request_new_screen_size(512.0, 512.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let review = ReviewParticles::load();
    let mut entries = Vec::new();
    for case in 0..3 {
        for left in [false, true] {
            let variant = variant(case, left);
            let folder = output.join(&variant);
            std::fs::create_dir(&folder).map_err(|error| error.to_string())?;
            for frame in 0..90 {
                clear_background(INK);
                let age = frame as f32 / 30.0;
                let rect = Rect::new(24.0, 24.0, 448.0, 448.0);
                scene(&review, case, left, rect, age, true);
                crate::ui_capture::framebuffer()?
                    .export_png(folder.join(format!("{frame:03}.png")).to_str().unwrap());
                if frame == 45 {
                    clear_background(INK);
                    scene(&review, case, left, rect, age, false);
                    crate::ui_capture::framebuffer()?
                        .export_png(folder.join("control.png").to_str().unwrap());
                }
                next_frame().await;
            }
            let fixture = &review.flows[case].fixture;
            let definition = if left {
                MAPS[case]
                    .lines()
                    .map(|row| row.chars().rev().collect::<String>())
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                MAPS[case].to_owned()
            };
            let coords = |at: GridPos| {
                let at = mirrored_cell(at, left);
                [at.x, at.y]
            };
            entries.push(serde_json::json!({"id":"FE02","number":10,"variant":variant,"folder":variant,
                "title":SCENARIOS[case].1,"left":left,"tile":64,"frames":90,"fps":30,
                "framebuffer_size":[512,512],"crop":[24,24,448,448],"map":definition,"origin":coords(ORIGIN),
                "affected_cells":fixture.affected.iter().copied().map(coords).collect::<Vec<_>>(),
                "targets":fixture.targets.iter().enumerate().map(|(i,&at)| serde_json::json!({
                    "label":char::from(b'A'+i as u8).to_string(),"cell":coords(at),"affected":fixture.affected.contains(&at)})).collect::<Vec<_>>(),
                "control_frame":45,"streams":review.flows[case].streams.len(),
                "method":"Complete textured particles following rounded cached paths; no image clipping or shader"}));
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_flame_bounds_stay_outside_walls_on_every_path_and_direction() {
        for case in 0..3 {
            let flow = Flow::new(case);
            assert!(!flow.streams.is_empty());
            let reflected: Vec<_> = flow
                .walls
                .iter()
                .map(|r| Rect::new(448.0 - r.x - r.w, r.y, r.w, r.h))
                .collect();
            for frame in 0..90 {
                for (point, _, size, _, _) in flow.particles(frame as f32 / 30.0) {
                    let radius = ALPHA_RADIUS * size / 64.0 + 1.5;
                    assert!(clear(point, radius, &flow.walls));
                    assert!(clear(mirrored(point, true), radius, &reflected));
                }
            }
        }
    }
    #[test]
    fn visual_routing_uses_only_the_unchanged_footprint() {
        for case in 0..3 {
            let flow = Flow::new(case);
            let allowed: BTreeSet<_> = flow
                .fixture
                .affected
                .iter()
                .copied()
                .chain([ORIGIN])
                .collect();
            for &target in &flow.fixture.affected {
                let nodes = route(&allowed, target);
                assert!(!nodes.is_empty());
                assert!(
                    nodes
                        .iter()
                        .all(|p| allowed.contains(p) && flow.fixture.map.is_walkable(*p))
                );
            }
            for &target in &flow.fixture.targets {
                if !flow.fixture.affected.contains(&target) {
                    assert!(route(&allowed, target).is_empty());
                }
            }
        }
    }
}
