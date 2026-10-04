//! Repeatable native editor benchmark. Never writes the supplied map or prefs.
use super::{App, assets::Assets, editor::Brush, paint, render, workbench};
use macroquad::prelude::*;
use project_rl::world::GridPos;
use std::{path::Path, time::Instant};

#[derive(Default, serde::Serialize)]
pub struct FrameCost {
    pub ground_ms: f64,
    pub objects_ms: f64,
    pub ui_ms: f64,
    pub total_ms: f64,
}

pub async fn ux(
    directory: &Path,
    mut app: App,
    assets: &Assets,
    background: &[Texture2D],
    font: &Font,
) {
    use super::{
        scene::Scene,
        storage,
        workbench::{self, Panel},
    };
    std::fs::create_dir(directory).expect("UX review directory must be new");
    let source = app.editor.path.clone();
    let source_bytes = std::fs::read(&source).unwrap();
    let original = app.scene.document.clone();
    assert_eq!((original.width, original.height), (160, 112));
    app.pointer_preview = false;
    let mut samples = vec![];
    for pass in 0..3 {
        app.scene = Scene::empty(8, 8).unwrap();
        app.saved = Some(app.scene.document.clone());
        app.reset_camera();
        workbench::open(&mut app, Panel::Maps);
        app.workbench.file_index = app
            .workbench
            .files
            .iter()
            .position(|p| p == &source)
            .unwrap();
        workbench::load_preview(&mut app);
        assert_eq!(app.workbench.file_doc.as_ref(), Some(&original));
        let mut frames = 0;
        let mut preview_cpu_ms = vec![];
        while !app.workbench.thumbnail_ready() {
            let cost = render(&app, assets, background, font);
            preview_cpu_ms.push(cost.total_ms);
            next_frame().await;
            frames += 1;
            assert!(frames < 1200, "the incremental preview must finish");
        }
        let timer = Instant::now();
        workbench::open_selected_map(&mut app, Panel::Maps).unwrap();
        let load_ms = timer.elapsed().as_secs_f64() * 1000.;
        assert_eq!(app.scene.document, original);
        assert_eq!(app.editor.path, source);
        app.camera.cell = 16.;
        app.camera.center = vec2(130., 25.);
        let mut preparation = vec![];
        let mut frames = 0;
        loop {
            assets
                .paint
                .borrow_mut()
                .begin_frame(Some(std::time::Duration::from_millis(4)));
            let cost = render(&app, assets, background, font);
            let deferred = assets.paint.borrow().deferred;
            preparation.push(serde_json::json!({"cpu_ms":cost.total_ms,"ground_ms":cost.ground_ms,"deferred":deferred}));
            next_frame().await;
            frames += 1;
            if deferred == 0 {
                break;
            }
            assert!(frames < 1200);
        }
        let mut warm = vec![];
        app.animated = true;
        for _ in 0..8 {
            assets
                .paint
                .borrow_mut()
                .begin_frame(Some(std::time::Duration::from_millis(4)));
            let before = assets.paint.borrow().rasterized;
            let timer = Instant::now();
            let cost = render(&app, assets, background, font);
            next_frame().await;
            let new_tiles = assets.paint.borrow().rasterized - before;
            assert_eq!(new_tiles, 0, "animation must not rebuild painted images");
            warm.push(
                serde_json::json!({"frame_ms":timer.elapsed().as_secs_f64()*1000.,"cost":cost}),
            );
        }
        app.animated = false;
        println!(
            "Library pass {pass}: {load_ms:.2} ms to open; {} preview frames, {} canvas preparation frames",
            preview_cpu_ms.len(),
            preparation.len()
        );
        samples.push(serde_json::json!({"pass":pass,"load_ms":load_ms,"preview_cpu_ms":preview_cpu_ms,"preparation":preparation,"warm_animated":warm}));
    }
    assets.paint.borrow_mut().begin_frame(None);
    app.camera.cell = 32.;
    app.camera.center = vec2(41.5, 32.5);
    super::capture(
        &directory.join("editeur-ville.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    workbench::open(&mut app, Panel::Maps);
    super::capture(
        &directory.join("bibliotheque.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    app.workbench.panel = None;
    let old_path = app.editor.path.clone();
    app.editor.path = directory.join("sauvegarde-verifiee.json");
    assert!(app.save());
    assert_eq!(
        storage::read_json::<super::scene::Document>(&app.editor.path).unwrap(),
        original
    );
    app.editor.path = old_path;
    std::fs::write(directory.join("invalide.json"), "{}").unwrap();
    app.workbench.files = vec![directory.join("invalide.json")];
    app.workbench.file_index = 0;
    assert!(workbench::open_selected_map(&mut app, Panel::Maps).is_err());
    assert_eq!(app.scene.document, original);
    assert_eq!(app.editor.path, source);
    app.message =
        "Carte ouverte. Molette pour zoomer, barres ou Espace + glisser pour parcourir.".into();
    macroquad::window::request_new_screen_size(960., 640.);
    for _ in 0..3 {
        next_frame().await;
    }
    super::capture(
        &directory.join("editeur-960.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    workbench::open(&mut app, Panel::Maps);
    super::capture(
        &directory.join("bibliotheque-960.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    app.workbench.panel = None;
    macroquad::window::request_new_screen_size(1360., 840.);
    for _ in 0..3 {
        next_frame().await;
    }
    app.scene = animation_demo();
    app.editor.path = directory.join("essai-animations.json");
    app.saved = Some(app.scene.document.clone());
    app.reset_camera();
    app.camera.cell = 32.;
    app.animated = true;
    app.message="Animations : voyants, lasers, reflets de l'eau et mouvement léger des arbres. F9 pour arrêter.".into();
    for (time, name) in [(0.2, "animations-a.png"), (2.5, "animations-b.png")] {
        app.animation_clock = Some(time);
        super::capture(&directory.join(name), &app, assets, background, font).await;
    }
    let a = Image::from_file_with_format(
        &std::fs::read(directory.join("animations-a.png")).unwrap(),
        None,
    )
    .unwrap();
    let b = Image::from_file_with_format(
        &std::fs::read(directory.join("animations-b.png")).unwrap(),
        None,
    )
    .unwrap();
    let changed = a
        .bytes
        .chunks_exact(4)
        .zip(b.bytes.chunks_exact(4))
        .filter(|(a, b)| a != b)
        .count();
    assert!(
        changed > 100,
        "cosmetic animation must actually change the native render"
    );
    assert_eq!(std::fs::read(&source).unwrap(), source_bytes);
    let origin = app.camera.origin(
        app.scene.document.width,
        app.scene.document.height,
        app.bounds(),
    );
    let cell = app.camera.cell;
    let mut regions = serde_json::Map::new();
    for (name, world) in [
        ("water", Rect::new(15., 7., 12., 8.)),
        ("laser", Rect::new(1., 1., 10., 1.)),
        ("lights", Rect::new(2., 2., 7., 6.)),
        ("trees", Rect::new(13., 2., 13., 4.)),
    ] {
        let rect = Rect::new(
            origin.x + world.x * cell,
            origin.y + world.y * cell,
            world.w * cell,
            world.h * cell,
        );
        let mut count = 0;
        for y in rect.y.max(0.) as usize..(rect.bottom().min(a.height as f32) as usize) {
            for x in rect.x.max(0.) as usize..(rect.right().min(a.width as f32) as usize) {
                let i = (y * a.width as usize + x) * 4;
                if a.bytes[i..i + 4] != b.bytes[i..i + 4] {
                    count += 1;
                }
            }
        }
        assert!(count > 0, "{name} animation must be visible");
        regions.insert(name.into(), serde_json::json!(count));
    }
    std::fs::write(directory.join("validation.json"),serde_json::to_vec_pretty(&serde_json::json!({"passes":samples,"animation_changed_pixels":changed,"animation_regions":regions,"save_roundtrip":true,"invalid_load_preserves_map":true,"source_unchanged":true})).unwrap()).unwrap();
}

pub fn animation_demo() -> super::scene::Scene {
    use super::editor::Brush;
    use super::{
        author::{self, Placement, Tool},
        scene::{Prop, PropDetails, Scene},
    };
    let mut doc = Scene::empty(28, 16).unwrap().document;
    doc.spawn = GridPos::new(3, 3);
    doc.floors.fill(Some(7));
    for y in 2..10 {
        for x in 2..10 {
            doc.floors[(y * doc.width + x) as usize] = Some(18);
        }
    }
    for y in 7..15 {
        for x in 15..27 {
            doc.floors[(y * doc.width + x) as usize] = Some(if x < 18 { 8 } else { 9 });
        }
    }
    let walls = author::shape(
        Tool::Rectangle,
        GridPos::new(1, 1),
        GridPos::new(10, 10),
        Brush::Wall,
    );
    author::place(
        &mut doc,
        &walls,
        &Placement {
            brush: Brush::Wall,
            rotation: 0,
            style: 3,
            fixed: None,
            blocking: true,
            details: &Default::default(),
            layers: &Default::default(),
        },
    )
    .unwrap();
    doc.structures.retain(|p| p.pos != GridPos::new(5, 10));
    for (sprite, x, y) in [
        (56, 3, 4),
        (57, 4, 4),
        (62, 5, 4),
        (66, 7, 3),
        (51, 7, 6),
        (104, 14, 3),
        (105, 20, 3),
        (108, 24, 4),
    ] {
        doc.props.push(Prop {
            pos: GridPos::new(x, y),
            sprite,
            furniture: true,
            rotation: 0,
            blocking: true,
            details: PropDetails::default(),
        });
    }
    doc.paint.push(paint::Stroke {
        material: Some(8),
        diameter: 192,
        points: vec![
            paint::Point::new(18 * 64, 10 * 64),
            paint::Point::new(20 * 64, 12 * 64),
        ],
        clip: None,
    });
    Scene::from_document(doc).unwrap()
}

pub async fn run(
    directory: &Path,
    mut app: App,
    assets: &Assets,
    background: &[Texture2D],
    font: &Font,
) {
    std::fs::create_dir(directory).expect("benchmark directory must be new");
    let original = app.scene.document.clone();
    app.pointer_preview = false;
    let mut samples = vec![];
    for (name, center, zoom, frames, pan) in [
        ("ville-32", vec2(41., 32.), 32., 7, false),
        ("nature-32", vec2(130., 25.), 32., 7, false),
        ("ville-16", vec2(41., 32.), 16., 5, false),
        ("nature-16", vec2(130., 25.), 16., 5, false),
        ("defilement-32", vec2(75., 38.), 32., 10, true),
    ] {
        app.camera.center = center;
        app.camera.cell = zoom;
        for frame in 0..frames {
            if pan {
                app.camera.center.x += 0.65;
            }
            let before = assets.paint.borrow().rasterized;
            let timer = Instant::now();
            let costs = render(&app, assets, background, font);
            next_frame().await;
            let frame_ms = timer.elapsed().as_secs_f64() * 1000.;
            let rasterized = assets.paint.borrow().rasterized - before;
            samples.push(
                serde_json::json!({"view":name,"frame":frame,"frame_ms":frame_ms,
                "rasterized_tiles":rasterized,"cost":costs}),
            );
            println!(
                "{name} #{frame}: {frame_ms:.2} ms, ground {:.2} ms, UI {:.2} ms, {rasterized} tiles rasterized",
                samples.last().unwrap()["cost"]["ground_ms"]
                    .as_f64()
                    .unwrap(),
                samples.last().unwrap()["cost"]["ui_ms"].as_f64().unwrap()
            );
        }
    }
    let timer = Instant::now();
    app.editor.choose_brush(Brush::Floor(Some(6)));
    app.editor
        .place(&mut app.scene, GridPos::new(45, 42))
        .unwrap();
    let placement_ms = timer.elapsed().as_secs_f64() * 1000.;
    app.editor.undo(&mut app.scene).unwrap();
    assert_eq!(app.scene.document, original);
    app.editor.round = true;
    app.editor.diameter = 128;
    let timer = Instant::now();
    for x in 0..8 {
        app.editor
            .paint_to(
                &mut app.scene,
                paint::Point::new(82 * 64 + x * 12, 77 * 64),
                false,
            )
            .unwrap();
    }
    app.editor.end_stroke(&app.scene);
    let painting_ms = timer.elapsed().as_secs_f64() * 1000.;
    app.editor.undo(&mut app.scene).unwrap();
    assert_eq!(app.scene.document, original);
    let timer = Instant::now();
    app.toggle_test();
    let enter_test_ms = timer.elapsed().as_secs_f64() * 1000.;
    app.camera.cell = 32.;
    for frame in 0..8 {
        let before = assets.paint.borrow().rasterized;
        let timer = Instant::now();
        app.scene.move_player(project_rl::world::Direction::South);
        let pos = app.scene.game.player_position().unwrap();
        app.camera.center = vec2(pos.x as f32 + 0.5, pos.y as f32 + 0.5);
        let costs = render(&app, assets, background, font);
        next_frame().await;
        let frame_ms = timer.elapsed().as_secs_f64() * 1000.;
        let rasterized = assets.paint.borrow().rasterized - before;
        samples.push(
            serde_json::json!({"view":"test-jouable-32","frame":frame,"frame_ms":frame_ms,
            "rasterized_tiles":rasterized,"cost":costs}),
        );
        println!("test-jouable-32 #{frame}: {frame_ms:.2} ms, {rasterized} tiles rasterized");
    }
    app.toggle_test();
    app.message = "Carte ouverte.".into();
    assert_eq!(app.scene.document, original);
    let result = serde_json::json!({"samples": samples, "placement_ms":placement_ms,
                                  "painting_eight_points_ms":painting_ms,"enter_test_ms":enter_test_ms});
    std::fs::write(
        directory.join("performance.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("Placement: {placement_ms:.2} ms; eight brush points: {painting_ms:.2} ms");
    app.camera.cell = 32.;
    app.camera.center = vec2(41., 32.);
    super::capture(&directory.join("ville.png"), &app, assets, background, font).await;
    app.camera.center = vec2(130., 25.);
    super::capture(
        &directory.join("nature.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    let source = app.editor.path.clone();
    workbench::open(&mut app, workbench::Panel::Maps);
    app.workbench.file_index = app
        .workbench
        .files
        .iter()
        .position(|path| path == &source)
        .expect("The supplied map must be available in the editor library");
    app.workbench.page = app.workbench.file_index.saturating_sub(3);
    workbench::load_preview(&mut app);
    assert_eq!(app.workbench.file_doc.as_ref(), Some(&original));
    super::capture(
        &directory.join("cartes.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    assert_eq!(app.scene.document, original);
}
