//! Isolated native art trial. No campaign, save, or graphics-setting changes.
//! Run: cargo run --locked --example textured_surface_preview
#[path = "surface_preview/assets.rs"]
#[allow(dead_code)] // Shared module also supplies the map editor.
mod assets;
#[path = "surface_preview/paint.rs"]
mod paint;
#[path = "surface_preview/rendering.rs"]
mod rendering;
#[path = "surface_preview/scene.rs"]
#[allow(dead_code)]
mod scene;

use macroquad::prelude::*;
use project_rl::game::CommandOutcome;
use project_rl::world::{Direction, GridPos};
use scene::{HEIGHT, Scene, WIDTH};
use std::path::PathBuf;

fn window_conf() -> Conf {
    Conf {
        window_title: "Project RL — essai textures de surface 64".to_owned(),
        window_width: 1360,
        window_height: 840,
        high_dpi: false,
        sample_count: 1,
        ..Default::default()
    }
}

struct View {
    cell: f32,
    visibility: bool,
    follow: bool,
    message: String,
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

fn sprite(texture: &Texture2D, x: f32, y: f32, cell: f32, color: Color) {
    draw_texture_ex(
        texture,
        x,
        y,
        color,
        DrawTextureParams {
            dest_size: Some(vec2(cell, cell)),
            ..Default::default()
        },
    );
}

fn origin(scene: &Scene, view: &View, bounds: Rect) -> Vec2 {
    let world = vec2(WIDTH as f32 * view.cell, HEIGHT as f32 * view.cell);
    let player = scene.game.player_position().unwrap();
    let center = if view.follow {
        vec2(player.x as f32 + 0.5, player.y as f32 + 0.5)
    } else {
        vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0)
    };
    let mut result =
        vec2(bounds.x + bounds.w / 2.0, bounds.y + bounds.h / 2.0) - center * view.cell;
    result.x = if world.x <= bounds.w {
        bounds.x + (bounds.w - world.x) / 2.0
    } else {
        result.x.clamp(bounds.x + bounds.w - world.x, bounds.x)
    };
    result.y = if world.y <= bounds.h {
        bounds.y + (bounds.h - world.y) / 2.0
    } else {
        result.y.clamp(bounds.y + bounds.h - world.y, bounds.y)
    };
    vec2(result.x.floor(), result.y.floor())
}

fn render(scene: &Scene, view: &View, assets: &assets::Assets, ground: &[Texture2D], font: &Font) {
    assets.paint.borrow_mut().sync(&scene.document.paint);
    clear_background(Color::from_rgba(24, 28, 31, 255));
    let wide = screen_width() >= 1100.0;
    label(
        font,
        "SURFACE · ESSAI NATIF",
        24.0,
        36.0,
        25,
        Color::from_rgba(217, 221, 217, 255),
    );
    label(
        font,
        &format!(
            "Tuiles 64 × 64 · affichage {} px · tour {}",
            view.cell as u16,
            scene.game.turn()
        ),
        24.0,
        64.0,
        18,
        Color::from_rgba(164, 177, 177, 255),
    );
    if wide {
        label(
            font,
            "Scène de test · validation visuelle en attente",
            920.0,
            36.0,
            17,
            Color::from_rgba(187, 163, 109, 255),
        );
    }
    let bounds = Rect::new(24.0, 88.0, screen_width() - 48.0, screen_height() - 184.0);
    let start = origin(scene, view, bounds);
    draw_rectangle(
        bounds.x,
        bounds.y,
        bounds.w,
        bounds.h,
        Color::from_rgba(16, 20, 22, 255),
    );
    let mut camera = Camera2D {
        viewport: Some((
            bounds.x as i32,
            (screen_height() - bounds.y - bounds.h) as i32,
            bounds.w as i32,
            bounds.h as i32,
        )),
        ..Camera2D::from_display_rect(bounds)
    };
    // Same on-screen Y convention as src/graphics.rs::ui_camera.
    camera.zoom.y = -camera.zoom.y;
    set_camera(&camera);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let p = start + vec2(x as f32, y as f32) * view.cell;
            rendering::draw_ground(scene, assets, ground, GridPos::new(x, y), p, view.cell);
        }
    }
    for part in &scene.document.structures {
        let p = start + vec2(part.pos.x as f32, part.pos.y as f32) * view.cell;
        rendering::draw_structure(scene, assets, part, p, view.cell);
    }
    for prop in &scene.props {
        let p = start + vec2(prop.pos.x as f32, prop.pos.y as f32) * view.cell;
        let texture = if prop.furniture {
            &assets.furniture[prop.sprite]
        } else {
            &assets.terrain[prop.sprite]
        };
        sprite(
            texture,
            p.x + view.cell / 32.0,
            p.y + view.cell / 16.0,
            view.cell,
            Color::new(0.0, 0.0, 0.0, 0.32),
        );
        sprite(texture, p.x, p.y, view.cell, WHITE);
    }
    let player = scene.game.player_position().unwrap();
    let p = start + vec2(player.x as f32 + 0.5, player.y as f32 + 0.5) * view.cell;
    draw_circle(
        p.x,
        p.y,
        view.cell * 0.27,
        Color::new(0.02, 0.08, 0.09, 0.7),
    );
    let size = (view.cell * 0.57) as u16;
    let metrics = measure_text("@", Some(font), size, 1.0);
    label(
        font,
        "@",
        p.x - metrics.width / 2.0,
        p.y + metrics.height / 2.0,
        size,
        Color::from_rgba(84, 218, 226, 255),
    );
    if view.visibility {
        let visibility = scene.game.player_visibility();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pos = GridPos::new(x, y);
                if !visibility.is_visible(pos) {
                    let p = start + vec2(x as f32, y as f32) * view.cell;
                    let alpha = if visibility.is_explored(pos) {
                        0.75
                    } else {
                        1.0
                    };
                    draw_rectangle(
                        p.x,
                        p.y,
                        view.cell,
                        view.cell,
                        Color::new(0.04, 0.06, 0.07, alpha),
                    );
                }
            }
        }
    }
    set_default_camera();
    let y = screen_height() - 68.0;
    label(
        font,
        "Flèches / ZQSD : marcher · E : porte à proximité · 1 / 2 / 3 : 32 / 48 / 64 px",
        24.0,
        y,
        if wide { 18 } else { 15 },
        Color::from_rgba(205, 209, 200, 255),
    );
    label(
        font,
        &format!(
            "F : vision du jeu ({}) · C : suivre ({}) · R : recommencer · Échap : quitter",
            if view.visibility { "oui" } else { "non" },
            if view.follow { "oui" } else { "non" }
        ),
        24.0,
        y + 23.0,
        if wide { 17 } else { 15 },
        Color::from_rgba(157, 173, 173, 255),
    );
    label(
        font,
        &view.message,
        24.0,
        y + 47.0,
        15,
        Color::from_rgba(187, 163, 109, 255),
    );
}

fn apply_result(view: &mut View, outcome: CommandOutcome) {
    view.message = match outcome {
        CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime => "Joueur provisoire : @ · mobilier et collisions réservés à cette scène.".to_owned(),
        CommandOutcome::Rejected(_) => "Action impossible ici. Pour la porte : place-toi dans une case voisine puis appuie sur E.".to_owned(),
    };
}

async fn capture(
    path: PathBuf,
    scene: &Scene,
    view: &View,
    assets: &assets::Assets,
    ground: &[Texture2D],
    font: &Font,
) {
    for _ in 0..2 {
        render(scene, view, assets, ground, font);
        next_frame().await;
    }
    render(scene, view, assets, ground, font);
    let mut image = get_screen_data();
    for pixel in image.bytes.chunks_exact_mut(4) {
        pixel[3] = 255;
    }
    image.export_png(path.to_str().unwrap());
    println!(
        "Capture native : {} ({} × {})",
        path.display(),
        image.width,
        image.height
    );
}

#[macroquad::main(window_conf)]
async fn main() {
    let prepared = assets::Prepared::load();
    let assets = prepared.upload();
    let ground: Vec<_> = (0..HEIGHT)
        .flat_map(|y| (0..WIDTH).map(move |x| (x, y)))
        .map(|(x, y)| prepared.ground(x, y))
        .collect();
    let mut font = load_ttf_font_from_bytes(include_bytes!(
        "../assets/fonts/AtkinsonHyperlegible-Regular.ttf"
    ))
    .unwrap();
    font.set_filter(FilterMode::Nearest);
    let mut scene = Scene::new();
    let mut view = View {
        cell: 64.0,
        visibility: false,
        follow: false,
        message: "Joueur provisoire : @ · mobilier et collisions réservés à cette scène."
            .to_owned(),
    };
    let args: Vec<_> = std::env::args_os().collect();
    if let Some(index) = args.iter().position(|arg| arg == "--capture") {
        let directory = PathBuf::from(
            args.get(index + 1)
                .expect("--capture needs a new output directory"),
        );
        std::fs::create_dir_all(&directory).unwrap();
        for name in [
            "native-64.png",
            "native-32.png",
            "native-door-open.png",
            "native-fov.png",
            "native-960.png",
        ] {
            assert!(
                !directory.join(name).exists(),
                "capture would overwrite {name}"
            );
        }
        capture(
            directory.join("native-64.png"),
            &scene,
            &view,
            &assets,
            &ground,
            &font,
        )
        .await;
        view.cell = 32.0;
        capture(
            directory.join("native-32.png"),
            &scene,
            &view,
            &assets,
            &ground,
            &font,
        )
        .await;
        view.cell = 64.0;
        scene.move_player(Direction::East);
        scene.move_player(Direction::East);
        scene.interaction();
        capture(
            directory.join("native-door-open.png"),
            &scene,
            &view,
            &assets,
            &ground,
            &font,
        )
        .await;
        view.visibility = true;
        capture(
            directory.join("native-fov.png"),
            &scene,
            &view,
            &assets,
            &ground,
            &font,
        )
        .await;
        view.visibility = false;
        view.follow = true;
        request_new_screen_size(960.0, 540.0);
        next_frame().await;
        capture(
            directory.join("native-960.png"),
            &scene,
            &view,
            &assets,
            &ground,
            &font,
        )
        .await;
        return;
    }
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        for (keys, direction) in [
            ([KeyCode::Up, KeyCode::Z, KeyCode::W], Direction::North),
            ([KeyCode::Right, KeyCode::D, KeyCode::D], Direction::East),
            ([KeyCode::Down, KeyCode::S, KeyCode::S], Direction::South),
            ([KeyCode::Left, KeyCode::Q, KeyCode::A], Direction::West),
        ] {
            if keys.into_iter().any(is_key_pressed) {
                apply_result(&mut view, scene.move_player(direction));
            }
        }
        if is_key_pressed(KeyCode::E) {
            apply_result(&mut view, scene.interaction());
        }
        for (key, size) in [
            (KeyCode::Key1, 32.0),
            (KeyCode::Key2, 48.0),
            (KeyCode::Key3, 64.0),
        ] {
            if is_key_pressed(key) {
                view.cell = size;
            }
        }
        if is_key_pressed(KeyCode::F) {
            view.visibility = !view.visibility;
        }
        if is_key_pressed(KeyCode::C) {
            view.follow = !view.follow;
        }
        if is_key_pressed(KeyCode::R) {
            scene = Scene::new();
        }
        render(&scene, &view, &assets, &ground, &font);
        next_frame().await;
    }
}
