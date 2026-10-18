//! PLACE-CAMP-01: illustrated r04 kit and optional legacy r03 review fixture.
//! Isolated review only; no campaign loading, saves or accepted art replacement.
#[path = "surface_preview/assets.rs"]
#[allow(dead_code)]
mod assets;
#[path = "surface_preview/author.rs"]
#[allow(dead_code)]
mod author;
#[path = "surface_preview/catalog.rs"]
#[allow(dead_code)]
mod catalog;
#[path = "surface_preview/paint.rs"]
#[allow(dead_code)]
mod paint;
#[path = "surface_preview/rendering.rs"]
#[allow(dead_code)]
mod rendering;
#[path = "surface_preview/scene.rs"]
#[allow(dead_code)]
mod scene;
#[path = "handcrafted_camp/scenery.rs"]
mod scenery;

use macroquad::prelude::*;
use project_rl::entity::Actor;
use project_rl::game::{CommandOutcome, GameCommand};
use project_rl::world::{Direction, DoorState, GridPos, Terrain};
use scene::{Document, Prop, PropDetails, Scene, Structure};
use std::path::{Path, PathBuf};

const W: i32 = 15;
const H: i32 = 8;
const CELL: f32 = 64.;
const DOOR: GridPos = GridPos::new(7, 4);
const START: GridPos = GridPos::new(5, 4);
const NPCS: [GridPos; 2] = [GridPos::new(4, 3), GridPos::new(11, 4)];
const FURNITURE: [(usize, i32, i32); 7] = [
    (0, 2, 2),
    (1, 2, 3),
    (2, 5, 2),
    (3, 1, 2),
    (4, 10, 2),
    (2, 12, 2),
    (5, 12, 5),
];
const SOURCES: [&str; 3] = [
    "assets/source-art/refonte-agent-2026-10-06/actor-resolution-review-01/native64/AR-N07-0-00.png",
    "assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/accepted/NPC-02-r02/NPC-02-r02-native64.png",
    "assets/source-art/refonte-histoire-2026-10-16/npc-monsters-review-01/accepted/NPC-03-r02/NPC-03-r02-native64.png",
];

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn fixture() -> Scene {
    let mut document = Document {
        version: 1,
        width: W,
        height: H,
        spawn: START,
        floors: (0..H)
            .flat_map(|_| (0..W).map(|x| Some(if x < 7 { 0 } else { 2 })))
            .collect(),
        background_offset: GridPos::new(0, 0),
        background_tiles: Default::default(),
        paint: vec![],
        structures: vec![],
        props: vec![],
        markers: vec![],
    };
    for y in 0..H {
        for x in 0..W {
            if x == 0 || x == W - 1 || y == 0 || y == H - 1 || x == 7 {
                let pos = GridPos::new(x, y);
                document.structures.push(Structure {
                    pos,
                    door: (pos == DOOR).then_some(DoorState::Open),
                    rotation: usize::from(pos == DOOR),
                    style: 0,
                    fixed_connections: None,
                });
            }
        }
    }
    // Simulation footprints are explicit, not inferred from illustration pixels.
    for &(sprite, x, y) in &FURNITURE {
        document.props.push(Prop {
            pos: GridPos::new(x, y),
            sprite,
            furniture: true,
            rotation: 0,
            blocking: true,
            details: PropDetails {
                opaque: Some(false),
                ..Default::default()
            },
        });
    }
    let mut result = Scene::from_document(document).unwrap();
    for position in NPCS {
        // Neutral, stationary review actors: no service dialogue, combat or AI.
        result
            .game
            .spawn_actor(Actor::new(position, 1).unwrap())
            .unwrap();
    }
    result
}

fn interact(scene: &mut Scene) -> CommandOutcome {
    scene
        .game
        .process_player_command(GameCommand::Interact { target: DOOR })
}

fn window_conf() -> Conf {
    Conf {
        window_title: if std::env::args().any(|a| a == "--legacy-r03") {
            "Project RL · PLACE-CAMP-01 r03 · essai moteur"
        } else {
            "Project RL · PLACE-CAMP-01 r04 · décors illustrés"
        }
        .into(),
        window_width: 1440,
        window_height: 900,
        high_dpi: false,
        sample_count: 1,
        window_resizable: false,
        ..Default::default()
    }
}

struct Native {
    texture: Texture2D,
    shadow: Texture2D,
    image: Image,
}

fn load_native(path: impl AsRef<Path>) -> Native {
    let image =
        Image::from_file_with_format(&std::fs::read(root().join(path)).unwrap(), None).unwrap();
    assert_eq!((image.width, image.height), (128, 128));
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    let opaque_x: Vec<_> = (90..96)
        .flat_map(|y| {
            let image = &image;
            (0..128).filter(move |&x| image.bytes[(y * 128 + x) * 4 + 3] > 128)
        })
        .collect();
    let width = opaque_x
        .iter()
        .max()
        .zip(opaque_x.iter().min())
        .map_or(24, |(a, b)| (a - b + 6).clamp(16, 64));
    let mut shadow = Image::gen_image_color(128, 128, BLANK);
    for y in 93..98 {
        for x in (64 - width / 2)..(64 + width / 2) {
            let i = (y * 128 + x) * 4;
            shadow.bytes[i..i + 4].copy_from_slice(&[8, 12, 14, 84]);
        }
    }
    let shadow = Texture2D::from_image(&shadow);
    shadow.set_filter(FilterMode::Nearest);
    Native {
        texture,
        shadow,
        image,
    }
}

fn label(font: &Font, text: &str, x: f32, y: f32, size: u16, color: Color) {
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font: Some(font),
            font_size: size,
            color,
            ..Default::default()
        },
    );
}

fn origin() -> Vec2 {
    vec2(((screen_width() - W as f32 * CELL) / 2.).floor(), 128.)
}
fn foot(pos: GridPos) -> Vec2 {
    origin() + vec2((pos.x as f32 + 0.5) * CELL, (pos.y + 1) as f32 * CELL)
}
fn draw_native(native: &Native, baseline: Vec2) {
    let start = baseline - vec2(64., 96.);
    draw_texture(&native.shadow, start.x, start.y, WHITE);
    draw_texture(&native.texture, start.x, start.y, WHITE);
}

fn render(
    scene: &Scene,
    assets: &assets::Assets,
    kit: Option<&scenery::Kit>,
    actors: &[Native],
    props: &[Native],
    font: &Font,
    grid: bool,
) {
    clear_background(Color::from_rgba(16, 22, 25, 255));
    label(
        font,
        if kit.is_some() {
            "PLACE-CAMP-01 · r04 — décors illustrés dans le moteur"
        } else {
            "PLACE-CAMP-01 · r03 — essai dans le moteur"
        },
        32.,
        40.,
        25,
        WHITE,
    );
    label(
        font,
        "Décors modulaires · cases de 64 px · silhouettes humaines de 60 à 65 px · proposition à examiner",
        32.,
        72.,
        17,
        GRAY,
    );
    let at = origin();
    label(font, "Armurerie / atelier", at.x + 68., 111., 18, LIGHTGRAY);
    label(
        font,
        "Infirmerie",
        at.x + 8. * CELL + 24.,
        111.,
        18,
        LIGHTGRAY,
    );
    for y in 0..H {
        for x in 0..W {
            let pos = GridPos::new(x, y);
            if let Some(kit) = kit {
                kit.ground(x, y, W, H, at + vec2(x as f32, y as f32) * CELL);
            } else {
                rendering::draw_ground(
                    scene,
                    assets,
                    &[],
                    pos,
                    at + vec2(x as f32, y as f32) * CELL,
                    CELL,
                );
            }
        }
    }
    if let Some(kit) = kit {
        for shadow in [true, false] {
            for part in &scene.document.structures {
                kit.wall(
                    scene,
                    part,
                    at + vec2(part.pos.x as f32, part.pos.y as f32) * CELL,
                    shadow,
                );
            }
        }
        kit.lamps(at);
    } else {
        for part in &scene.document.structures {
            rendering::draw_structure(
                scene,
                assets,
                part,
                at + vec2(part.pos.x as f32, part.pos.y as f32) * CELL,
                CELL,
            );
        }
    }
    if grid {
        for x in 0..=W {
            draw_line(
                at.x + x as f32 * CELL,
                at.y,
                at.x + x as f32 * CELL,
                at.y + H as f32 * CELL,
                1.,
                Color::new(0.6, 0.7, 0.7, 0.14),
            );
        }
        for y in 0..=H {
            draw_line(
                at.x,
                at.y + y as f32 * CELL,
                at.x + W as f32 * CELL,
                at.y + y as f32 * CELL,
                1.,
                Color::new(0.6, 0.7, 0.7, 0.14),
            );
        }
    }
    let player = scene.game.player_position().unwrap();
    let mut entries: Vec<(i32, i32, &Native)> = FURNITURE
        .iter()
        .map(|&(index, x, y)| (y, x, &props[index]))
        .collect();
    entries.extend([
        (player.y, player.x, &actors[0]),
        (NPCS[0].y, NPCS[0].x, &actors[1]),
        (NPCS[1].y, NPCS[1].x, &actors[2]),
    ]);
    entries.sort_by_key(|&(y, x, _)| (y, x));
    for (y, x, sprite) in entries {
        draw_native(sprite, foot(GridPos::new(x, y)));
    }
    label(
        font,
        "Comparaison des proportions — mêmes dessins et même taille que dans les pièces",
        32.,
        695.,
        18,
        LIGHTGRAY,
    );
    let names = [
        "Joueur · 65 px",
        "PNJ · 60 px",
        "01 · Établi",
        "02 · Siège",
        "03 · Armoire",
        "04 · Étagère",
        "05 · Lit",
        "06 · Banc",
    ];
    for (i, native) in [
        &actors[0], &actors[1], &props[0], &props[1], &props[2], &props[3], &props[4], &props[5],
    ]
    .into_iter()
    .enumerate()
    {
        let x = 80. + i as f32 * 170.;
        draw_native(native, vec2(x, 803.));
        label(font, names[i], x - 52., 839., 15, LIGHTGRAY);
    }
    let open = scene.game.map().tile(DOOR).unwrap().terrain == Terrain::Door(DoorState::Open);
    let state = if open {
        "Porte ouverte"
    } else {
        "Porte fermée"
    };
    label(
        font,
        &format!(
            "Flèches / ZQSD / WASD : déplacement · E : porte à proximité · G : grille · R : recommencer · Échap : quitter — {state}"
        ),
        32.,
        883.,
        16,
        GRAY,
    );
}

async fn capture(
    path: &Path,
    scene: &Scene,
    assets: &assets::Assets,
    kit: Option<&scenery::Kit>,
    actors: &[Native],
    props: &[Native],
    font: &Font,
) {
    for _ in 0..2 {
        render(scene, assets, kit, actors, props, font, false);
        next_frame().await;
    }
    render(scene, assets, kit, actors, props, font, false);
    let mut image = get_screen_data();
    for p in image.bytes.chunks_exact_mut(4) {
        p[3] = 255;
    }
    image.export_png(path.to_str().unwrap());
    // Saved GPU framebuffer must retain every opaque actor pixel at native size.
    for (native, pos) in [
        (&actors[0], scene.game.player_position().unwrap()),
        (&actors[1], NPCS[0]),
        (&actors[2], NPCS[1]),
    ] {
        let start = foot(pos) - vec2(64., 96.);
        for y in 0..128 {
            for x in 0..128 {
                let s = (y * 128 + x) * 4;
                if native.image.bytes[s + 3] == 255 {
                    // get_screen_data is bottom-up; export_png flips it.
                    let d = ((image.height as usize - 1 - start.y as usize - y)
                        * image.width as usize
                        + start.x as usize
                        + x)
                        * 4;
                    assert_eq!(
                        &native.image.bytes[s..s + 3],
                        &image.bytes[d..d + 3],
                        "native actor mismatch"
                    );
                }
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let args: Vec<_> = std::env::args().collect();
    let legacy = args.iter().any(|a| a == "--legacy-r03");
    let corrected_doors = args.iter().any(|a| a == "--corrected-doors");
    assert!(
        !(legacy && corrected_doors),
        "choose one historical fixture"
    );
    let revision = if legacy {
        "r03"
    } else if corrected_doors {
        "r05-door-fixture"
    } else {
        "r04"
    };
    let prop_revision = if legacy { "r03" } else { "r04" };
    let prepared = assets::Prepared::load();
    let assets = prepared.upload();
    let kit = (!legacy).then(|| scenery::Kit::load(corrected_doors));
    let actors: Vec<_> = SOURCES.into_iter().map(load_native).collect();
    let props: Vec<_> = (1..=6)
        .map(|i| {
            load_native(format!(
                "assets/prototypes/handcrafted-camp-01/{}DEC-CAMP-{i:02}-{prop_revision}.png",
                if legacy { "" } else { "r04/" }
            ))
        })
        .collect();
    let font =
        load_ttf_font_from_bytes(&std::fs::read("C:/Windows/Fonts/segoeui.ttf").unwrap()).unwrap();
    let mut scene = fixture();
    if let Some(i) = args.iter().position(|arg| arg == "--capture") {
        let out = PathBuf::from(args.get(i + 1).expect("capture output directory"));
        std::fs::create_dir_all(&out).unwrap();
        capture(
            &out.join(format!("{revision}-native-open.png")),
            &scene,
            &assets,
            kit.as_ref(),
            &actors,
            &props,
            &font,
        )
        .await;
        assert_eq!(scene.move_player(Direction::East), CommandOutcome::Applied);
        assert_eq!(interact(&mut scene), CommandOutcome::Applied);
        assert!(!scene.game.map().is_walkable(DOOR));
        capture(
            &out.join(format!("{revision}-native-closed.png")),
            &scene,
            &assets,
            kit.as_ref(),
            &actors,
            &props,
            &font,
        )
        .await;
        assert_eq!(interact(&mut scene), CommandOutcome::Applied);
        for _ in 0..2 {
            assert_eq!(scene.move_player(Direction::East), CommandOutcome::Applied);
        }
        assert_eq!(scene.game.player_position(), Some(GridPos::new(8, 4)));
        capture(
            &out.join(format!("{revision}-native-crossed.png")),
            &scene,
            &assets,
            kit.as_ref(),
            &actors,
            &props,
            &font,
        )
        .await;
        std::fs::write(out.join(format!("engine-checks-{revision}.json")), serde_json::to_vec_pretty(&serde_json::json!({
            "place":"PLACE-CAMP-01", "revision":revision, "actual_gpu_capture":true,
            "simulation":"project_rl::game::GameState", "renderer":if legacy { "shared surface_preview modules" } else { "illustrated native64 kit; painted T pieces and zero wall rotations" },
            "painted_T_masks":if legacy { vec![] } else { vec![7,11,13,14] }, "wall_rotation_operations":if legacy { serde_json::Value::Null } else { serde_json::json!(0) },
            "door_open_walkable":true, "door_closed_blocks":true, "player_crossed_door":true,
            "corrected_door_jambs":corrected_doors,
            "npc_count":2, "npc_ai":false, "actor_opaque_gpu_pixels_exact":true,
            "cell_pixels":64, "actor_resize_operations":0, "campaign_connected":false,
            "capture_size":[screen_width() as u32,screen_height() as u32], "map_origin":[origin().x,origin().y],
            "map_size_cells":[W,H]
        })).unwrap()).unwrap();
        return;
    }
    let mut grid = false;
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        for (keys, direction) in [
            ([KeyCode::Up, KeyCode::Z, KeyCode::W], Direction::North),
            ([KeyCode::Down, KeyCode::S, KeyCode::S], Direction::South),
            ([KeyCode::Left, KeyCode::Q, KeyCode::A], Direction::West),
            ([KeyCode::Right, KeyCode::D, KeyCode::D], Direction::East),
        ] {
            if keys.into_iter().any(is_key_pressed) {
                scene.move_player(direction);
            }
        }
        if is_key_pressed(KeyCode::E) {
            interact(&mut scene);
        }
        if is_key_pressed(KeyCode::G) {
            grid = !grid;
        }
        if is_key_pressed(KeyCode::R) {
            scene = fixture();
        }
        render(&scene, &assets, kit.as_ref(), &actors, &props, &font, grid);
        next_frame().await;
    }
}

#[cfg(test)]
mod fixture_tests {
    use super::*;

    #[test]
    fn same_engine_controls_crossing_wall_furniture_and_npc_occupancy() {
        let mut scene = fixture();
        assert_eq!(scene.neighbors(GridPos::new(7, 0)), 14);
        assert_eq!(scene.neighbors(GridPos::new(7, H - 1)), 11);
        assert!(scene.game.map().is_walkable(DOOR));
        assert_eq!(scene.move_player(Direction::East), CommandOutcome::Applied);
        assert_eq!(interact(&mut scene), CommandOutcome::Applied);
        let before = scene.game.player_position();
        assert_ne!(scene.move_player(Direction::East), CommandOutcome::Applied);
        assert_eq!(scene.game.player_position(), before);
        assert_eq!(interact(&mut scene), CommandOutcome::Applied);
        for _ in 0..2 {
            assert_eq!(scene.move_player(Direction::East), CommandOutcome::Applied);
        }
        assert_eq!(scene.game.player_position(), Some(GridPos::new(8, 4)));
        assert!(!scene.game.map().is_walkable(GridPos::new(7, 3)));
        for &(_, x, y) in &FURNITURE {
            assert!(!scene.game.map().is_walkable(GridPos::new(x, y)));
        }
        for _ in 0..2 {
            assert_eq!(scene.move_player(Direction::East), CommandOutcome::Applied);
        }
        let before = scene.game.player_position();
        scene.move_player(Direction::East);
        assert_eq!(
            scene.game.player_position(),
            before,
            "cannot walk through the stationary NPC"
        );
    }
}
