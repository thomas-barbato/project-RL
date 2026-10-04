//! Validate and render an existing large editor document without changing it.
use super::{App, assets::Assets, author, capture, rendering, scene::Scene};
use macroquad::prelude::*;
use project_rl::world::GridPos;
use std::path::Path;

async fn region_image(
    path: &Path,
    scene: &Scene,
    assets: &Assets,
    background: &[Texture2D],
    font: &Font,
    region: author::Region,
    cell: u32,
    title: &str,
) {
    let width = region.w as u32 * cell;
    let height = region.h as u32 * cell + 48;
    let target = render_target(width, height);
    target.texture.set_filter(FilterMode::Nearest);
    let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., width as f32, height as f32));
    camera.render_target = Some(target.clone());
    set_camera(&camera);
    clear_background(Color::from_rgba(19, 24, 25, 255));
    let origin = |pos: GridPos| {
        vec2(
            (pos.x - region.x) as f32 * cell as f32,
            48. + (pos.y - region.y) as f32 * cell as f32,
        )
    };
    for pos in region.cells() {
        rendering::draw_ground(scene, assets, background, pos, origin(pos), cell as f32);
    }
    for part in &scene.document.structures {
        if region.contains(part.pos) {
            rendering::draw_structure(scene, assets, part, origin(part.pos), cell as f32);
        }
    }
    for prop in &scene.document.props {
        if prop.pos.x >= region.x - 4
            && prop.pos.y >= region.y - 4
            && prop.pos.x < region.x + region.w + 4
            && prop.pos.y < region.y + region.h + 4
        {
            rendering::draw_prop(assets, prop, origin(prop.pos), cell as f32);
        }
    }
    draw_rectangle(0., 0., width as f32, 48., Color::from_rgba(19, 24, 25, 255));
    super::label(
        font,
        title,
        16.,
        31.,
        23,
        Color::from_rgba(213, 226, 219, 255),
    );
    set_default_camera();
    next_frame().await;
    let mut image = target.texture.get_texture_data();
    for pixel in image.bytes.chunks_exact_mut(4) {
        pixel[3] = 255;
    }
    image.export_png(path.to_str().unwrap());
    println!("Landscape capture: {} ({width}x{height})", path.display());
}

pub async fn capture_review(
    directory: &Path,
    scene: &Scene,
    assets: &Assets,
    background: &[Texture2D],
    font: &Font,
) {
    std::fs::create_dir(directory).expect("capture directory must be new");
    // The real loader, game map and cardinal traversal must agree on this file.
    let roundtrip = serde_json::to_vec(&scene.document).unwrap();
    let restored = Scene::from_document(serde_json::from_slice(&roundtrip).unwrap()).unwrap();
    assert_eq!(restored.document, scene.document);
    let access = author::reachable(&scene.document);
    let issues = author::validate(&scene.document);
    let text = issues
        .iter()
        .map(|i| format!("{},{}: {}", i.pos.x, i.pos.y, i.message))
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(directory.join("acces.txt"), format!(
        "{}x{} cases; {} structures; {} objets; {} coups de pinceau; {} marqueurs.\n{} cases accessibles; {} problèmes.\n{}\n",
        scene.document.width, scene.document.height, scene.document.structures.len(),
        scene.document.props.len(), scene.document.paint.len(), scene.document.markers.len(),
        access.len(), issues.len(), text)).unwrap();
    println!(
        "Accessibility: {} reachable cells, {} issues\n{}",
        access.len(),
        issues.len(),
        text
    );
    assets.paint.borrow_mut().sync(&scene.document.paint);
    let views = [
        (
            "01-vue-ensemble.png",
            author::Region {
                x: 0,
                y: 0,
                w: 160,
                h: 112,
            },
            12,
            "CARTE DE TEST — VILLE, HAMEAU ET NATURE · 160 × 112",
        ),
        (
            "02-ville.png",
            author::Region {
                x: 8,
                y: 8,
                w: 75,
                h: 80,
            },
            24,
            "VILLE · commerces, clinique, serveurs, armurerie, marché, logements",
        ),
        (
            "03-centre-ville.png",
            author::Region {
                x: 24,
                y: 27,
                w: 39,
                h: 27,
            },
            40,
            "CENTRE-VILLE · restaurant, place du marché et avenues",
        ),
        (
            "04-hameau.png",
            author::Region {
                x: 116,
                y: 51,
                w: 44,
                h: 34,
            },
            32,
            "HAMEAU · quatre maisons, un atelier et une réserve",
        ),
        (
            "05-grotte-du-bois.png",
            author::Region {
                x: 129,
                y: 12,
                w: 24,
                h: 22,
            },
            48,
            "GROTTE DU BOIS · entrée de donjon",
        ),
        (
            "06-ancienne-carriere.png",
            author::Region {
                x: 19,
                y: 85,
                w: 25,
                h: 24,
            },
            48,
            "ANCIENNE CARRIÈRE · entrée de donjon",
        ),
        (
            "07-pont-et-riviere.png",
            author::Region {
                x: 83,
                y: 33,
                w: 34,
                h: 23,
            },
            48,
            "RIVIÈRE · pont et route vers le hameau",
        ),
        (
            "08-lac.png",
            author::Region {
                x: 122,
                y: 82,
                w: 38,
                h: 30,
            },
            40,
            "LAC · berges, roseaux et sentier",
        ),
    ];
    for (name, region, cell, title) in views {
        region_image(
            &directory.join(name),
            scene,
            assets,
            background,
            font,
            region,
            cell,
            title,
        )
        .await;
    }
    let mut app = App::new();
    app.scene = restored;
    app.pointer_preview = false;
    app.grid = false;
    app.camera.center = vec2(49., 41.);
    app.camera.cell = 32.;
    app.message =
        "Grande carte de test — 23 bâtiments meublés, hameau, deux entrées de donjon et nature."
            .into();
    capture(
        &directory.join("09-editeur-ville.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    app.editor.category = 2;
    app.editor.object_group = 6;
    super::workbench::open(&mut app, super::workbench::Panel::Catalogue);
    capture(
        &directory.join("10-catalogue-nature.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    assert!(
        issues.is_empty(),
        "The test map must not contain inaccessible areas"
    );
}
