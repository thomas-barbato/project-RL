//! Repeatable native review, writing only into a fresh review directory.
use super::{
    App, Brush, assets,
    author::{self, Placement, Region, Stamp, Tool},
    capture, capture_urban_map, paint,
    scene::{Marker, MarkerKind, Prop, PropDetails, Scene},
    storage,
    workbench::{self, Panel},
};
use macroquad::prelude::*;
use project_rl::world::{DoorState, GridPos};
use std::path::Path;

fn demo() -> Scene {
    let mut doc = Scene::empty(26, 18).unwrap().document;
    doc.spawn = GridPos::new(4, 11);
    let details = PropDetails::default();
    let layers = Default::default();
    let walls = author::shape(
        Tool::Rectangle,
        GridPos::new(2, 2),
        GridPos::new(14, 13),
        Brush::Wall,
    );
    author::place(
        &mut doc,
        &walls,
        &Placement {
            brush: Brush::Wall,
            rotation: 0,
            style: 0,
            fixed: None,
            blocking: true,
            details: &details,
            layers: &layers,
        },
    )
    .unwrap();
    let inside: Vec<_> = Region {
        x: 3,
        y: 3,
        w: 11,
        h: 10,
    }
    .cells()
    .collect();
    author::place(
        &mut doc,
        &inside,
        &Placement {
            brush: Brush::Floor(Some(0)),
            rotation: 0,
            style: 0,
            fixed: None,
            blocking: false,
            details: &details,
            layers: &layers,
        },
    )
    .unwrap();
    let door = doc
        .structures
        .iter_mut()
        .find(|p| p.pos == GridPos::new(14, 8))
        .unwrap();
    door.door = Some(DoorState::Closed);
    door.rotation = 1;
    for (sprite, x, y, rotation) in [
        (56, 4, 4, 0),
        (57, 4, 5, 0),
        (58, 4, 6, 0),
        (4, 6, 5, 0),
        (1, 6, 7, 0),
        (1, 7, 4, 2),
        (72, 10, 6, 0),
        (75, 10, 9, 1),
        (65, 6, 10, 0),
        (80, 21, 15, 0),
        (29, 18, 2, 0),
        (25, 23, 5, 0),
    ] {
        let (width, height) = super::catalog::default_size(sprite);
        doc.props.push(Prop {
            pos: GridPos::new(x, y),
            sprite,
            furniture: true,
            rotation,
            blocking: true,
            details: PropDetails {
                width,
                height,
                opaque: Some(false),
                ..Default::default()
            },
        });
    }
    // Two decorations in a single cell exercise offsets and graphic scale.
    for (sprite, offset_x, offset_y) in [(14, -16, -12), (15, 16, 12)] {
        doc.props.push(Prop {
            pos: GridPos::new(7, 6),
            sprite,
            furniture: false,
            rotation: 0,
            blocking: false,
            details: PropDetails {
                decoration: true,
                opaque: Some(false),
                offset_x,
                offset_y,
                scale: 45,
                ..Default::default()
            },
        });
    }
    doc.paint.push(paint::Stroke {
        material: Some(5),
        diameter: 112,
        points: vec![
            paint::Point::new(16 * 64 + 32, 3 * 64 + 32),
            paint::Point::new(18 * 64 + 32, 5 * 64 + 32),
            paint::Point::new(20 * 64 + 32, 8 * 64 + 32),
            paint::Point::new(22 * 64 + 32, 12 * 64 + 32),
        ],
        clip: None,
    });
    for (kind, pos, width, height, target, count) in [
        (
            MarkerKind::Enemies,
            GridPos::new(17, 3),
            8,
            11,
            "patrouille",
            6,
        ),
        (MarkerKind::Cave, GridPos::new(20, 15), 2, 2, "cave-demo", 1),
        (
            MarkerKind::Exit,
            GridPos::new(24, 15),
            2,
            2,
            "zone-suivante",
            1,
        ),
    ] {
        doc.markers.push(Marker {
            pos,
            width,
            height,
            kind,
            name: match kind {
                MarkerKind::Enemies => "Rencontre",
                MarkerKind::Cave => "Cave",
                MarkerKind::Exit => "Sortie",
            }
            .into(),
            target: target.into(),
            count,
        });
    }
    Scene::from_document(doc).unwrap()
}

pub async fn viewport(
    directory: &Path,
    assets: &assets::Assets,
    background: &[Texture2D],
    font: &Font,
) {
    std::fs::create_dir(directory).expect("viewport review directory must be new");
    let mut app = App::new();
    app.scene = demo();
    app.pointer_preview = false;
    app.reset_camera();
    app.editor.choose_brush(Brush::Object(4, true));
    app.scene.document.props[3].details.scale = 200;
    app.scene.document.props[3].details.offset_x = 31;
    app.scene = Scene::from_document(app.scene.document.clone()).unwrap();
    app.camera.cell = 64.;
    app.camera.center = vec2(16.2, 8.);
    app.message =
        "Le meuble agrandi et décalé reste visible lorsque son ancrage quitte la vue.".into();
    super::capture(
        &directory.join("objet-partiellement-visible.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    app.workbench.prop_index = None;
    app.selection = Some(GridPos::new(14, 8));
    workbench::open(&mut app, Panel::Inspector);
    super::capture(
        &directory.join("proprietes-porte.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
}

pub async fn run(directory: &Path, assets: &assets::Assets, background: &[Texture2D], font: &Font) {
    std::fs::create_dir(directory).expect("review directory must be new");
    let mut app = App::new();
    app.scene = demo();
    app.editor.path = directory.join("maps/atelier.json");
    app.editor.category = 2;
    app.editor.object_group = 0;
    app.editor.choose_brush(Brush::Object(72, true));
    app.pointer_preview = false;
    app.reset_camera();
    app.camera.cell = 32.;
    app.message="Outils de composition, calques, ensembles et marqueurs — carte de démonstration technique.".into();
    assert!(app.save());
    capture_urban_map(
        &directory.join("carte-native.png"),
        &app.scene,
        assets,
        background,
    )
    .await;
    app.workbench.region = Some(Region {
        x: 2,
        y: 2,
        w: 13,
        h: 12,
    });
    capture(
        &directory.join("editeur-complet.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;

    let original = app.scene.document.clone();
    let layers = app.editor.layers.clone();
    let r = app.workbench.region.unwrap();
    let mut stamp = Stamp::take(&original, r, &layers);
    stamp.name = "Salle technique".into();
    storage::validate_stamp(&stamp).unwrap();
    storage::write_json(&directory.join("ensembles/salle-technique.json"), &stamp).unwrap();
    let mut moved = original.clone();
    Stamp::remove(&mut moved, r, &layers);
    stamp
        .paste(&mut moved, GridPos::new(0, 6), &layers, true)
        .unwrap();
    assert!(app.editor.commit(&mut app.scene, moved).unwrap());
    assert_eq!(
        app.scene.document.spawn,
        GridPos::new(original.spawn.x - 2, original.spawn.y + 4)
    );
    app.editor.undo(&mut app.scene).unwrap();
    assert_eq!(app.scene.document, original);
    let mut copy = original.clone();
    stamp
        .paste(&mut copy, GridPos::new(12, 2), &layers, false)
        .unwrap();
    assert!(app.editor.commit(&mut app.scene, copy).is_err());
    assert_eq!(
        app.scene.document, original,
        "invalid overlapping paste must be atomic"
    );

    workbench::open(&mut app, Panel::Catalogue);
    app.editor.search = "lit".into();
    app.editor
        .favorites
        .insert(super::editor::key(Brush::Object(72, true)));
    capture(
        &directory.join("catalogue-recherche.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    app.workbench.panel = None;
    workbench::open(&mut app, Panel::Layers);
    app.editor.layers.locked[0] = true;
    capture(
        &directory.join("calques.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    let immutable = app.scene.document.clone();
    app.editor.brush = Brush::Floor(Some(4));
    assert!(
        app.editor
            .place(&mut app.scene, GridPos::new(5, 5))
            .is_err()
    );
    assert_eq!(app.scene.document, immutable);
    app.editor.layers.locked[0] = false;
    app.workbench.panel = None;
    app.workbench.prop_index = Some(6);
    workbench::open(&mut app, Panel::Inspector);
    capture(
        &directory.join("proprietes-lit.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    app.workbench.panel = None;
    app.workbench.prop_index = None;
    app.selection = Some(GridPos::new(14, 8));
    workbench::open(&mut app, Panel::Inspector);
    capture(
        &directory.join("proprietes-porte.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    app.selection = None;
    app.workbench.panel = None;
    app.editor.search.clear();
    app.editor.category = 1;
    app.editor.brush = Brush::Wall;
    app.workbench.tool = Tool::Rectangle;
    app.editor.wall_style = 1;
    workbench::set_preview(
        &mut app,
        author::shape(
            Tool::Rectangle,
            GridPos::new(17, 6),
            GridPos::new(24, 12),
            Brush::Wall,
        ),
    );
    assert!(app.workbench.preview_valid);
    assert_eq!(app.scene.document, original);
    capture(
        &directory.join("apercu-murs.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    app.workbench.preview.clear();
    app.workbench.preview_scene = None;
    app.workbench.region = None;
    app.workbench.tool = Tool::Line;
    app.editor.choose_brush(Brush::Object(72, true));
    workbench::set_preview(&mut app, vec![GridPos::new(25, 17)]);
    assert!(!app.workbench.preview_valid);
    capture(
        &directory.join("placement-refuse.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    app.workbench.preview.clear();
    app.workbench.preview_scene = None;
    app.workbench.tool = Tool::Pencil;
    workbench::open(&mut app, Panel::Issues);
    capture(
        &directory.join("controle.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    app.workbench.panel = None;
    let random = app.workbench.random.as_ref().unwrap();
    assert_eq!(random.enemies.len(), 6);
    assert!(random.cave.is_some() && random.exit.is_some());
    capture(
        &directory.join("marqueurs-tirage.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    app.workbench.random = None;
    for (panel, file) in [
        (Panel::Maps, "cartes.png"),
        (Panel::Prefabs, "ensembles.png"),
    ] {
        workbench::open(&mut app, panel);
        if panel == Panel::Catalogue {
            app.editor.category = 2;
        }
        if panel == Panel::Prefabs {
            app.workbench.files = storage::documents(&directory.join("ensembles"));
            workbench::load_preview(&mut app);
        }
        capture(&directory.join(file), &app, assets, background, font).await;
        app.workbench.panel = None;
    }
    let mut changed = original.clone();
    changed.floors[0] = Some(7);
    storage::save_map(&app.editor.path, &changed).unwrap();
    storage::save_map(&app.editor.path, &original).unwrap();
    workbench::open(&mut app, Panel::Versions);
    capture(
        &directory.join("versions.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    app.workbench.recovery = Some(storage::Recovery {
        path: app.editor.path.clone(),
        document: changed,
    });
    app.workbench.panel = Some(Panel::Recovery);
    capture(
        &directory.join("recuperation.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;

    request_new_screen_size(960., 540.);
    next_frame().await;
    app.editor.category = 2;
    for (panel, file) in [
        (Panel::Catalogue, "catalogue-960.png"),
        (Panel::Inspector, "proprietes-960.png"),
        (Panel::Issues, "controle-960.png"),
        (Panel::Maps, "cartes-960.png"),
    ] {
        workbench::open(&mut app, panel);
        app.editor.search = if panel == Panel::Catalogue {
            "lit".into()
        } else {
            String::new()
        };
        app.workbench.prop_index = Some(6);
        capture(&directory.join(file), &app, assets, background, font).await;
    }
    app.workbench.panel = None;
    app.workbench.random = None;
    capture(
        &directory.join("editeur-960.png"),
        &app,
        assets,
        background,
        font,
    )
    .await;
    println!(
        "Native review complete: group move/undo, atomic paste refusal, locked layer, previews, seeded markers, map versions and 960x540 UI."
    );
}
