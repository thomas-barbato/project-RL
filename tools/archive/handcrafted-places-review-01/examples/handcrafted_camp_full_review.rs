//! The accepted four-room camp and proposed cryogenics extension at native scale.
//! Review-only executable: no campaign loading or save access.
use macroquad::prelude::*;
use project_rl::entity::Actor;
use project_rl::game::{CommandOutcome, GameCommand, GameState};
use project_rl::world::{Direction, GridPos, Map, Terrain};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::{Path, PathBuf};
#[path = "handcrafted_camp/atmosphere.rs"]
mod atmosphere;

#[derive(Clone, Deserialize)]
struct Sprite {
    id: String,
    file: String,
    at: [i32; 2],
    #[serde(default = "one")]
    width: i32,
    #[serde(default = "one")]
    height: i32,
}
fn one() -> i32 {
    1
}
#[derive(Deserialize)]
struct Camp {
    width: i32,
    height: i32,
    floor: String,
    structure: String,
    #[serde(default)]
    layout: String,
    #[serde(default)]
    capture_targets: Vec<CaptureTarget>,
    exits: Vec<[i32; 2]>,
    actors: Vec<Sprite>,
    props: Vec<Sprite>,
    #[serde(default)]
    atmosphere: Option<atmosphere::Config>,
}
#[derive(Deserialize)]
struct CaptureTarget {
    name: String,
    at: [i32; 2],
}
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
fn config() -> Camp {
    serde_json::from_slice(
        &std::fs::read(root().join(format!(
            "assets/prototypes/handcrafted-camp-01/{}/camp.json",
            revision()
        )))
        .unwrap(),
    )
    .unwrap()
}
fn revision() -> &'static str {
    if std::env::args().any(|a| a == "--legacy-r05") {
        "r05"
    } else if std::env::args().any(|a| a == "--legacy-r06") {
        "r06"
    } else if std::env::args().any(|a| a == "--legacy-r07") {
        "r07"
    } else if std::env::args().any(|a| a == "--legacy-r08") {
        "r08"
    } else {
        "r09"
    }
}
fn pos(at: [i32; 2]) -> GridPos {
    GridPos::new(at[0], at[1])
}

fn game(c: &Camp) -> GameState {
    let layout = if c.layout.is_empty() {
        format!(
            "assets/prototypes/handcrafted-camp-01/{}/camp-layout.txt",
            revision()
        )
    } else {
        c.layout.clone()
    };
    let text = std::fs::read_to_string(root().join(layout)).unwrap();
    let mut map = Map::filled(c.width as usize, c.height as usize, Terrain::Wall).unwrap();
    for (y, row) in text.lines().enumerate() {
        for (x, ch) in row.chars().enumerate() {
            if ch == '.' {
                map.set_terrain(GridPos::new(x as i32, y as i32), Terrain::Floor)
                    .unwrap();
            }
        }
    }
    for p in &c.props {
        for dy in 0..p.height {
            for dx in 0..p.width {
                map.set_obstruction(GridPos::new(p.at[0] + dx, p.at[1] + dy), true, false)
                    .unwrap();
            }
        }
    }
    let mut state = GameState::new(map, pos(c.actors[0].at), 64065).unwrap();
    for a in &c.actors[1..] {
        state
            .spawn_actor(Actor::new(pos(a.at), 1).unwrap())
            .unwrap();
    }
    state
}

fn route(game: &GameState, c: &Camp, target: GridPos) -> Option<Vec<Direction>> {
    let start = game.player_position().unwrap();
    let occupied: BTreeSet<_> = c.actors[1..].iter().map(|a| pos(a.at)).collect();
    let mut seen = BTreeSet::from([start]);
    let mut queue = VecDeque::from([start]);
    let mut previous = BTreeMap::new();
    while let Some(p) = queue.pop_front() {
        if p == target {
            let mut path = Vec::new();
            let mut at = p;
            while at != start {
                let (from, dir) = previous[&at];
                path.push(dir);
                at = from;
            }
            path.reverse();
            return Some(path);
        }
        for (dx, dy, d) in [
            (0, -1, Direction::North),
            (1, 0, Direction::East),
            (0, 1, Direction::South),
            (-1, 0, Direction::West),
        ] {
            let n = GridPos::new(p.x + dx, p.y + dy);
            if game.map().is_walkable(n) && !occupied.contains(&n) && seen.insert(n) {
                previous.insert(n, (p, d));
                queue.push_back(n);
            }
        }
    }
    None
}

struct Native {
    texture: Texture2D,
    shadow: Texture2D,
}
fn image(path: &str) -> Image {
    Image::from_file_with_format(&std::fs::read(root().join(path)).unwrap(), None).unwrap()
}
fn texture(im: &Image) -> Texture2D {
    let t = Texture2D::from_image(im);
    t.set_filter(FilterMode::Nearest);
    t
}
fn native(s: &Sprite) -> Native {
    assert!(!s.id.is_empty(), "native sprite needs a review identifier");
    let im = image(&s.file);
    assert_eq!((im.width, im.height), (128, 128));
    let mut shadow = Image::gen_image_color(128, 128, BLANK);
    for y in 93..98 {
        for x in 48..80 {
            let i = (y * 128 + x) * 4;
            shadow.bytes[i..i + 4].copy_from_slice(&[8, 12, 14, 76]);
        }
    }
    Native {
        texture: texture(&im),
        shadow: texture(&shadow),
    }
}
fn draw_native(s: &Sprite, n: &Native, offset: Vec2) {
    let at = offset
        + vec2(
            (s.at[0] as f32 + s.width as f32 / 2.) * 64. - 64.,
            (s.at[1] + s.height) as f32 * 64. - 96.,
        );
    draw_texture(&n.shadow, at.x, at.y, WHITE);
    draw_texture(&n.texture, at.x, at.y, WHITE);
}
fn world(
    c: &Camp,
    g: &GameState,
    floor: &Texture2D,
    structure: &Texture2D,
    actors: &[Native],
    props: &[Native],
    offset: Vec2,
    grid: bool,
    ambience: Option<(&atmosphere::Atmosphere, f32, bool)>,
) {
    let destination = offset;
    let offset = if let Some((fx, _, lights)) = ambience {
        fx.begin();
        fx.begin_background(lights);
        Vec2::ZERO
    } else {
        offset
    };
    draw_texture(floor, offset.x, offset.y, WHITE);
    draw_texture(structure, offset.x, offset.y, WHITE);
    if let Some((fx, _, _)) = ambience {
        fx.end_material();
    }
    let mut order: Vec<_> = c
        .props
        .iter()
        .enumerate()
        .map(|(i, p)| (p.at[1] + p.height, p.at[0], false, i))
        .collect();
    order.extend(c.actors.iter().enumerate().map(|(i, a)| {
        let p = if i == 0 {
            g.player_position().unwrap()
        } else {
            pos(a.at)
        };
        (p.y + 1, p.x, true, i)
    }));
    order.sort_by_key(|&(y, x, _, _)| (y, x));
    for (_, _, is_actor, i) in order {
        if is_actor {
            let mut a = c.actors[i].clone();
            if i == 0 {
                let p = g.player_position().unwrap();
                a.at = [p.x, p.y];
            }
            draw_native(&a, &actors[i], offset);
        } else {
            if let Some((fx, time, _)) = ambience {
                fx.begin_prop(
                    &c.props[i].id,
                    time,
                    c.props[i].at[0] as f32 * 0.71 + i as f32 * 0.43,
                );
            }
            draw_native(&c.props[i], &props[i], offset);
            if let Some((fx, time, _)) = ambience {
                fx.end_material();
                if c.props[i].id == "DEC-CAMP-16" {
                    let p = &c.props[i];
                    fx.steam(
                        vec2((p.at[0] as f32 + 0.5) * 64., (p.at[1] + 1) as f32 * 64.),
                        time,
                    );
                }
            }
        }
    }
    if grid {
        for x in 0..=c.width {
            draw_line(
                offset.x + x as f32 * 64.,
                offset.y,
                offset.x + x as f32 * 64.,
                offset.y + c.height as f32 * 64.,
                1.,
                Color::new(0.5, 0.6, 0.6, 0.2),
            );
        }
        for y in 0..=c.height {
            draw_line(
                offset.x,
                offset.y + y as f32 * 64.,
                offset.x + c.width as f32 * 64.,
                offset.y + y as f32 * 64.,
                1.,
                Color::new(0.5, 0.6, 0.6, 0.2),
            );
        }
    }
    if let Some((fx, _, lights)) = ambience {
        fx.finish(destination, lights);
    }
}
fn window_conf() -> Conf {
    let args: Vec<_> = std::env::args().collect();
    let (width, height) = args
        .windows(2)
        .find(|pair| pair[0] == "--resolution")
        .and_then(|pair| pair[1].split_once('x'))
        .and_then(|(w, h)| Some((w.parse::<i32>().ok()?, h.parse::<i32>().ok()?)))
        .filter(|&(w, h)| (960..=7680).contains(&w) && (720..=4320).contains(&h))
        .unwrap_or((1600, 1000));
    Conf {
        window_title: format!("Project RL · camp de référence · {}", revision()),
        window_width: width,
        window_height: height,
        high_dpi: false,
        sample_count: 1,
        ..Default::default()
    }
}
fn label(font: &Font, s: &str, x: f32, y: f32, size: u16) {
    draw_text_ex(
        s,
        x,
        y,
        TextParams {
            font: Some(font),
            font_size: size,
            color: LIGHTGRAY,
            ..Default::default()
        },
    );
}

async fn capture(
    path: Option<&Path>,
    c: &Camp,
    g: &GameState,
    floor: &Texture2D,
    structure: &Texture2D,
    actors: &[Native],
    props: &[Native],
    ambience: Option<(&atmosphere::Atmosphere, f32, bool)>,
) -> Image {
    let target = render_target((c.width * 64) as u32, (c.height * 64) as u32);
    target.texture.set_filter(FilterMode::Nearest);
    let mut camera = Camera2D::from_display_rect(Rect::new(
        0.,
        0.,
        (c.width * 64) as f32,
        (c.height * 64) as f32,
    ));
    camera.render_target = Some(target.clone());
    set_camera(&camera);
    clear_background(BLACK);
    world(
        c,
        g,
        floor,
        structure,
        actors,
        props,
        Vec2::ZERO,
        false,
        ambience,
    );
    unsafe {
        get_internal_gl().flush();
    }
    let im = target.texture.get_texture_data();
    if let Some(path) = path {
        im.export_png(path.to_str().unwrap());
    }
    set_default_camera();
    next_frame().await;
    im
}

#[macroquad::main(window_conf)]
async fn main() {
    let c = config();
    let mut g = game(&c);
    let floor = texture(&image(&c.floor));
    let structure = texture(&image(&c.structure));
    let actors: Vec<_> = c.actors.iter().map(native).collect();
    let props: Vec<_> = c.props.iter().map(native).collect();
    let atmosphere = c
        .atmosphere
        .as_ref()
        .map(|cfg| atmosphere::Atmosphere::new(cfg, (c.width * 64) as u32, (c.height * 64) as u32));
    let font =
        load_ttf_font_from_bytes(&std::fs::read("C:/Windows/Fonts/segoeui.ttf").unwrap()).unwrap();
    let args: Vec<_> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--capture") {
        let out = PathBuf::from(&args[i + 1]);
        std::fs::create_dir_all(&out).unwrap();
        let mut records = Vec::new();
        let legacy_targets = vec![
            CaptureTarget {
                name: "camp".into(),
                at: c.actors[0].at,
            },
            CaptureTarget {
                name: "missions".into(),
                at: [7, 4],
            },
            CaptureTarget {
                name: "soins".into(),
                at: [7, 13],
            },
        ];
        let targets = if c.capture_targets.is_empty() {
            &legacy_targets
        } else {
            &c.capture_targets
        };
        let revision = revision();
        if atmosphere.is_some() {
            capture(
                Some(&out.join(format!("{revision}-reference-native.png"))),
                &c,
                &g,
                &floor,
                &structure,
                &actors,
                &props,
                None,
            )
            .await;
        }
        for t in targets {
            let name = &t.name;
            let target = pos(t.at);
            g = game(&c);
            let path = route(&g, &c, target).expect("review target reachable");
            for d in &path {
                assert_eq!(
                    g.process_player_command(GameCommand::Move(*d)),
                    CommandOutcome::Applied
                );
            }
            capture(
                Some(&out.join(format!("{revision}-{name}-native.png"))),
                &c,
                &g,
                &floor,
                &structure,
                &actors,
                &props,
                atmosphere.as_ref().map(|fx| (fx, 1.0, true)),
            )
            .await;
            records.push(serde_json::json!({"name":name,"player_at":[target.x,target.y],"engine_steps":path.len()}));
        }
        // Walk the routes with the same commands used by interactive controls.
        for target in &c.exits {
            let mut walk = game(&c);
            for direction in route(&walk, &c, pos(*target)).expect("exit reachable") {
                assert_eq!(
                    walk.process_player_command(GameCommand::Move(direction)),
                    CommandOutcome::Applied
                );
            }
            assert_eq!(walk.player_position(), Some(pos(*target)));
        }
        if let Some(fx) = &atmosphere {
            g = game(&c);
            capture(
                Some(&out.join(format!("{revision}-lights-off-native.png"))),
                &c,
                &g,
                &floor,
                &structure,
                &actors,
                &props,
                Some((fx, 1.0, false)),
            )
            .await;
            let animation = out.join("animation");
            std::fs::create_dir_all(&animation).unwrap();
            let position = g.player_position();
            for i in 0..24 {
                let im = capture(
                    None,
                    &c,
                    &g,
                    &floor,
                    &structure,
                    &actors,
                    &props,
                    Some((fx, i as f32 * 0.32, true)),
                )
                .await;
                // GPU readbacks are bottom-up; export_png reverses each crop.
                im.sub_image(Rect::new(0., im.height as f32 - 832., 768., 640.))
                    .export_png(animation.join(format!("cryo-{i:02}.png")).to_str().unwrap());
                im.sub_image(Rect::new(1096., im.height as f32 - 224., 128., 96.))
                    .export_png(
                        animation
                            .join(format!("screen-{i:02}.png"))
                            .to_str()
                            .unwrap(),
                    );
            }
            assert_eq!(g.player_position(), position);
        }
        std::fs::write(out.join(format!("full-engine-checks-{revision}.json")),serde_json::to_vec_pretty(&serde_json::json!({"revision":revision,"actual_gpu_capture":true,"renderer":"native architecture and actors, optional approved GPU atmosphere pass","size":[c.width*64,c.height*64],"cell":64,"actor_count":actors.len(),"prop_count":props.len(),"spawn":c.actors[0].at,"captures":records,"campaign_connected":false,"open_passages":true,"door_overlays_added":false,"new_service_rules":false,"exits_reached":c.exits,"computed_lighting":atmosphere.is_some(),"ambient_animations":atmosphere.is_some(),"animation_frames":if atmosphere.is_some(){24}else{0},"effects_borrow_game_state_immutably":true})).unwrap()).unwrap();
        return;
    }
    let mut grid = false;
    let mut lights = true;
    let mut paused = false;
    let mut clock = 0.0_f32;
    let mut timings = Vec::new();
    let benchmark = args
        .iter()
        .position(|a| a == "--performance")
        .map(|i| PathBuf::from(&args[i + 1]));
    let mut frame_started = std::time::Instant::now();
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        for (keys, d) in [
            ([KeyCode::Up, KeyCode::Z, KeyCode::W], Direction::North),
            ([KeyCode::Right, KeyCode::D, KeyCode::D], Direction::East),
            ([KeyCode::Down, KeyCode::S, KeyCode::S], Direction::South),
            ([KeyCode::Left, KeyCode::Q, KeyCode::A], Direction::West),
        ] {
            if keys.into_iter().any(is_key_pressed) {
                g.process_player_command(GameCommand::Move(d));
            }
        }
        if is_key_pressed(KeyCode::R) {
            g = game(&c);
        }
        if is_key_pressed(KeyCode::G) {
            grid = !grid;
        }
        if is_key_pressed(KeyCode::L) {
            lights = !lights;
        }
        if is_key_pressed(KeyCode::P) {
            paused = !paused;
        }
        if !paused {
            clock += get_frame_time().min(0.1);
        }
        clear_background(Color::from_rgba(12, 18, 21, 255));
        let p = g.player_position().unwrap();
        let view_h = screen_height() - 120.;
        let ox = (screen_width() / 2. - (p.x as f32 + 0.5) * 64.)
            .clamp((screen_width() - c.width as f32 * 64.).min(0.), 0.)
            .floor();
        let oy = (view_h / 2. - (p.y as f32 + 0.5) * 64.)
            .clamp((view_h - c.height as f32 * 64.).min(0.), 0.)
            .floor()
            + 80.;
        world(
            &c,
            &g,
            &floor,
            &structure,
            &actors,
            &props,
            vec2(ox, oy),
            grid,
            atmosphere.as_ref().map(|fx| (fx, clock, lights)),
        );
        draw_rectangle(
            0.,
            0.,
            screen_width(),
            78.,
            Color::from_rgba(12, 18, 21, 255),
        );
        draw_rectangle(
            0.,
            screen_height() - 40.,
            screen_width(),
            40.,
            Color::from_rgba(12, 18, 21, 255),
        );
        label(
            &font,
            &format!(
                "PLACE-CAMP-01 · {} — carte de référence dans le moteur",
                revision()
            ),
            24.,
            32.,
            23,
        );
        label(
            &font,
            if revision() != "r05" {
                "Cryogénie et quatre pièces de services · taille native · aperçu séparé de la campagne"
            } else {
                "Quatre pièces · personnages à taille native · aperçu séparé de la campagne"
            },
            24.,
            61.,
            16,
        );
        label(
            &font,
            if atmosphere.is_some() {
                "Flèches / ZQSD / WASD : marcher · L : lampes · P : pause ambiance · G : grille · R : recommencer · Échap : quitter"
            } else {
                "Flèches / ZQSD / WASD : marcher · G : grille · R : recommencer · Échap : quitter"
            },
            24.,
            screen_height() - 14.,
            16,
        );
        next_frame().await;
        if let Some(path) = &benchmark {
            timings.push(frame_started.elapsed().as_secs_f64() * 1000.);
            frame_started = std::time::Instant::now();
            if timings.len() == 120 {
                let mut measured = timings[10..].to_vec();
                measured.sort_by(f64::total_cmp);
                std::fs::write(path, serde_json::to_vec_pretty(&serde_json::json!({
                    "frames":measured.len(), "window":[screen_width(),screen_height()],
                    "median_frame_ms":measured[measured.len()/2],"p95_frame_ms":measured[measured.len()*95/100],
                    "cpu_and_present_time":true,"computed_lighting":atmosphere.is_some(),
                    "per_frame_texture_uploads":0,"per_frame_light_raycasts":0
                })).unwrap()).unwrap();
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_four_services_are_reached_by_actual_engine_commands() {
        let c = config();
        for a in &c.actors[1..] {
            let mut g = game(&c);
            let p = pos(a.at);
            let path = p
                .cardinal_neighbors()
                .into_iter()
                .find_map(|n| route(&g, &c, n))
                .expect(&a.id);
            for d in path {
                assert_eq!(
                    g.process_player_command(GameCommand::Move(d)),
                    CommandOutcome::Applied
                );
            }
            let end = g.player_position().unwrap();
            assert_eq!((end.x - p.x).abs() + (end.y - p.y).abs(), 1);
        }
        for target in c.exits.iter().copied().map(pos) {
            let mut g = game(&c);
            for d in route(&g, &c, target).unwrap() {
                assert_eq!(
                    g.process_player_command(GameCommand::Move(d)),
                    CommandOutcome::Applied
                );
            }
            assert_eq!(g.player_position(), Some(target));
        }
    }
}
