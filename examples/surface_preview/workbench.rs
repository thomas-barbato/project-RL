//! Editor workspace UI and gestures. Document mutations remain transactional.
use super::{
    App,
    author::{self, Region, Stamp, Tool},
    editor::{self, Brush},
    scene::{self, Document, Marker, MarkerKind, PropDetails},
    storage,
};
use macroquad::prelude::*;
use project_rl::world::GridPos;
use std::path::PathBuf;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Catalogue,
    Layers,
    Inspector,
    Maps,
    Prefabs,
    Versions,
    Issues,
    Recovery,
}
pub struct Thumbnail {
    document: Document,
    scene: scene::Scene,
    target: RenderTarget,
    paint: super::paint::Cache,
    cursor: usize,
}
impl Thumbnail {
    fn total(&self) -> usize {
        (self.document.width * self.document.height) as usize
            + self.document.structures.len()
            + self.document.props.len()
    }
    fn ready(&self) -> bool {
        self.cursor == self.total()
    }
}
pub struct Workbench {
    pub tool: Tool,
    pub panel: Option<Panel>,
    pub region: Option<Region>,
    pub gesture: Option<GridPos>,
    pub clipboard: Option<Stamp>,
    pub pasting: bool,
    pub prop_index: Option<usize>,
    pub marker_index: Option<usize>,
    pub issues: Vec<author::Issue>,
    pub overlay: u8,
    pub random: Option<author::RandomPreview>,
    pub seed: u64,
    pub page: usize,
    pub focus: u8,
    pub files: Vec<PathBuf>,
    pub file_index: usize,
    pub file_doc: Option<Document>,
    pub thumbnail: std::cell::RefCell<Option<Thumbnail>>,
    minimap: std::cell::RefCell<Option<(Document, RenderTarget)>>,
    pub prefab_name: String,
    pub recovery: Option<storage::Recovery>,
    pub last_recovery: Option<Document>,
    pub last_autosave: f64,
    pub recovery_error: Option<String>,
    pub preview: Vec<GridPos>,
    pub preview_valid: bool,
    pub preview_scene: Option<scene::Scene>,
    pub accessible: std::collections::BTreeSet<GridPos>,
    pub access_document: Option<Document>,
    pub inspection_scene: Option<scene::Scene>,
}
impl Workbench {
    pub fn thumbnail_ready(&self) -> bool {
        self.thumbnail
            .borrow()
            .as_ref()
            .is_some_and(Thumbnail::ready)
    }
    pub fn new() -> Self {
        Self {
            tool: Tool::Pencil,
            panel: None,
            region: None,
            gesture: None,
            clipboard: None,
            pasting: false,
            prop_index: None,
            marker_index: None,
            issues: vec![],
            overlay: 0,
            random: None,
            seed: 1,
            page: 0,
            focus: 0,
            files: vec![],
            file_index: 0,
            file_doc: None,
            thumbnail: Default::default(),
            minimap: Default::default(),
            prefab_name: "Mon ensemble".into(),
            recovery: None,
            last_recovery: None,
            last_autosave: 0.0,
            recovery_error: None,
            preview: vec![],
            preview_valid: true,
            preview_scene: None,
            accessible: Default::default(),
            access_document: None,
            inspection_scene: None,
        }
    }
    pub fn clear_selection(&mut self) {
        self.region = None;
        self.gesture = None;
        self.prop_index = None;
        self.marker_index = None;
        self.preview.clear();
        self.preview_scene = None;
    }
}
pub fn toolbar_height() -> f32 {
    let columns = ((screen_width() - 328.0) / 94.0).floor().max(1.0) as usize;
    17_usize.div_ceil(columns) as f32 * 34.0 + 6.0
}
fn toolbar_rect(index: usize) -> Rect {
    let columns = ((screen_width() - 328.0) / 94.0).floor().max(1.0) as usize;
    Rect::new(
        304.0 + (index % columns) as f32 * 94.0,
        88.0 + (index / columns) as f32 * 34.0,
        90.0,
        28.0,
    )
}
fn panel_rect() -> Rect {
    let w = (screen_width() - 48.0).min(1040.0);
    let h = (screen_height() - 64.0).min(720.0);
    Rect::new(
        (screen_width() - w) / 2.0,
        (screen_height() - h) / 2.0,
        w,
        h,
    )
}
fn ctl(panel: Rect, x: f32, y: f32, w: f32) -> Rect {
    Rect::new(panel.x + x, panel.y + y, w, 28.0)
}
fn clicked(rect: Rect) -> bool {
    is_mouse_button_pressed(MouseButton::Left) && rect.contains(mouse_position().into())
}
fn btn(font: &Font, rect: Rect, text: &str, active: bool) {
    super::button(font, rect, text, active);
}
fn text(font: &Font, text: &str, x: f32, y: f32, size: u16) {
    super::label(font, text, x, y, size, WHITE);
}
fn close_panel(app: &mut App) {
    app.workbench.panel = None;
    app.workbench.focus = 0;
    *app.workbench.thumbnail.borrow_mut() = None;
    app.workbench.file_doc = None;
    if std::env::args().any(|a| a.starts_with("--capture")) {
        return;
    }
    let result = storage::save_preferences(&app.editor);
    if let Err(e) = result {
        app.message = format!("Préférences : {e}");
    }
}
pub fn open_selected_map(app: &mut App, panel: Panel) -> Result<bool, String> {
    let path = app
        .workbench
        .files
        .get(app.workbench.file_index)
        .cloned()
        .ok_or_else(|| "Choisis une carte dans la liste.".to_owned())?;
    let result = storage::read_json::<Document>(&path)
        .and_then(|doc| app.editor.commit(&mut app.scene, doc));
    if result.is_ok() {
        if panel == Panel::Maps {
            app.editor.path = path;
            app.saved = Some(app.scene.document.clone());
        }
        app.reset_camera();
        close_panel(app);
    }
    app.report(
        result.clone(),
        if panel == Panel::Versions {
            "Version restaurée. F5 pour la conserver ; Ctrl+Z pour revenir."
        } else {
            "Carte ouverte. Ctrl+Z retrouve la composition précédente."
        },
    );
    result
}
pub fn open(app: &mut App, panel: Panel) {
    app.editor.end_stroke(&app.scene);
    app.workbench.gesture = None;
    app.workbench.preview.clear();
    app.workbench.preview_scene = None;
    app.workbench.panel = Some(panel);
    app.workbench.page = 0;
    app.workbench.focus = 0;
    if matches!(panel, Panel::Maps | Panel::Prefabs | Panel::Versions) {
        let folder = match panel {
            Panel::Maps => app
                .editor
                .path
                .parent()
                .unwrap_or(std::path::Path::new("."))
                .to_path_buf(),
            Panel::Prefabs => storage::root().join("ensembles"),
            _ => storage::versions(&app.editor.path),
        };
        app.workbench.files = storage::documents(&folder);
        app.workbench.file_index = if panel == Panel::Maps {
            app.workbench
                .files
                .iter()
                .position(|path| path == &app.editor.path)
                .unwrap_or(0)
        } else {
            0
        };
        app.workbench.page = app.workbench.file_index.saturating_sub(3);
        load_preview(app);
    }
    if panel == Panel::Issues {
        app.workbench.issues = author::validate(&app.scene.document);
        app.workbench.random = Some(author::random_preview(
            &app.scene.document,
            app.workbench.seed,
        ));
    }
}
pub fn load_preview(app: &mut App) {
    app.workbench.file_doc = None;
    *app.workbench.thumbnail.borrow_mut() = None;
    if let Some(path) = app.workbench.files.get(app.workbench.file_index) {
        if let Ok(doc) = storage::read_json::<Document>(path) {
            if scene::Scene::from_document(doc.clone()).is_ok() {
                app.workbench.file_doc = Some(doc);
            }
        } else if app.workbench.panel == Some(Panel::Prefabs) {
            if let Ok(stamp) = storage::read_json::<Stamp>(path) {
                if storage::validate_stamp(&stamp).is_ok() {
                    let mut doc = scene::Scene::empty(stamp.width, stamp.height)
                        .unwrap()
                        .document;
                    doc.floors = stamp.floors.unwrap_or(doc.floors);
                    doc.background_tiles = stamp.backgrounds;
                    doc.paint = stamp.paint;
                    doc.structures = stamp.structures;
                    doc.props = stamp.props;
                    doc.markers = stamp.markers;
                    app.workbench.file_doc = Some(doc);
                }
            }
        }
    }
}
fn apply(app: &mut App, document: Document, message: &str) {
    let result = app.editor.commit(&mut app.scene, document);
    app.report(result, message);
    app.workbench.random = None;
    app.workbench.preview.clear();
    app.workbench.preview_scene = None;
}
pub fn autosave(app: &mut App) {
    if get_time() - app.workbench.last_autosave < 30.0 || app.editor.in_stroke() {
        return;
    }
    app.workbench.last_autosave = get_time();
    if !app.dirty() || app.workbench.last_recovery.as_ref() == Some(&app.scene.document) {
        return;
    }
    let recovery = storage::Recovery {
        path: app.editor.path.clone(),
        document: app.scene.document.clone(),
    };
    match storage::write_json(&storage::recover_path(), &recovery) {
        Ok(()) => {
            app.workbench.last_recovery = Some(recovery.document);
            app.workbench.recovery_error = None;
        }
        Err(e) => {
            app.workbench.recovery_error = Some(e.clone());
            app.message = format!("Récupération automatique impossible : {e}");
        }
    }
}
pub fn force_recovery(app: &mut App) {
    let recovery = storage::Recovery {
        path: app.editor.path.clone(),
        document: app.scene.document.clone(),
    };
    if let Err(error) = storage::write_json(&storage::recover_path(), &recovery) {
        app.message = format!("Récupération impossible : {error}");
    }
    let _ = storage::save_preferences(&app.editor);
}
pub fn input(app: &mut App) -> bool {
    if app.workbench.overlay != 0
        && app.workbench.access_document.as_ref() != Some(&app.scene.document)
    {
        app.workbench.accessible = author::reachable(&app.scene.document);
        app.workbench.inspection_scene =
            scene::Scene::from_document(app.scene.document.clone()).ok();
        app.workbench.access_document = Some(app.scene.document.clone());
    }
    let ctrl = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);
    if let Some(panel) = app.workbench.panel {
        panel_input(app, panel);
        return true;
    }
    if app.modal.is_some() || app.testing {
        return false;
    }
    for (key, panel) in [
        (KeyCode::F2, Panel::Catalogue),
        (KeyCode::F3, Panel::Layers),
        (KeyCode::F4, Panel::Inspector),
        (KeyCode::F8, Panel::Issues),
        (KeyCode::F6, Panel::Maps),
        (KeyCode::F7, Panel::Prefabs),
    ] {
        if is_key_pressed(key) {
            open(app, panel);
            return true;
        }
    }
    if ctrl && is_key_pressed(KeyCode::F) {
        open(app, Panel::Catalogue);
        app.workbench.focus = 1;
        return true;
    }
    if app.workbench.pasting && is_key_pressed(KeyCode::Escape) {
        app.workbench.pasting = false;
        app.workbench.preview.clear();
        return true;
    }
    for i in 0..17 {
        if clicked(toolbar_rect(i)) {
            if i < 9 {
                app.workbench.tool = author::TOOLS[i].0;
                app.workbench.gesture = None;
                app.workbench.pasting = false;
                app.selecting = false;
                app.selection = None;
            } else {
                match i {
                    9 => open(app, Panel::Catalogue),
                    10 => open(app, Panel::Layers),
                    11 => open(app, Panel::Inspector),
                    12 => open(app, Panel::Maps),
                    13 => open(app, Panel::Prefabs),
                    14 => open(app, Panel::Issues),
                    15 => center(app),
                    _ => app.animated = !app.animated,
                }
            }
            return true;
        }
    }
    let mini = minimap_rect(app.bounds());
    if clicked(mini) {
        let mouse: Vec2 = mouse_position().into();
        app.camera.center = vec2(
            (mouse.x - mini.x) / mini.w * app.scene.document.width as f32,
            (mouse.y - mini.y) / mini.h * app.scene.document.height as f32,
        );
        return true;
    }
    if is_key_pressed(KeyCode::Escape)
        && (app.workbench.region.is_some() || app.workbench.gesture.is_some())
    {
        app.workbench.clear_selection();
        app.workbench.tool = Tool::Pencil;
        app.selection = None;
        return true;
    }
    if app.workbench.region.is_some() && !app.workbench.pasting {
        if ctrl && is_key_pressed(KeyCode::C) {
            let region = app.workbench.region.unwrap();
            app.workbench.clipboard =
                Some(Stamp::take(&app.scene.document, region, &app.editor.layers));
            app.message = "Ensemble copié. Ctrl+V active son placement ; R le tourne.".into();
            return true;
        }
        if is_key_pressed(KeyCode::Delete) {
            let mut doc = app.scene.document.clone();
            Stamp::remove(&mut doc, app.workbench.region.unwrap(), &app.editor.layers);
            apply(
                app,
                doc,
                "Sélection supprimée sur les calques déverrouillés. Ctrl+Z restaure tout.",
            );
            app.workbench.clear_selection();
            return true;
        }
        if is_key_pressed(KeyCode::R) && !ctrl {
            let r = app.workbench.region.unwrap();
            let mut stamp = Stamp::take(&app.scene.document, r, &app.editor.layers);
            stamp.rotate();
            let mut doc = app.scene.document.clone();
            Stamp::remove(&mut doc, r, &app.editor.layers);
            if let Err(e) = stamp.paste(&mut doc, GridPos::new(r.x, r.y), &app.editor.layers, true)
            {
                app.message = e;
            } else {
                let size = (stamp.width, stamp.height);
                let result = app.editor.commit(&mut app.scene, doc);
                if result.is_ok() {
                    app.workbench.region = Some(Region {
                        x: r.x,
                        y: r.y,
                        w: size.0,
                        h: size.1,
                    });
                }
                app.report(result, "Ensemble tourné. Ctrl+Z pour annuler.");
            }
            return true;
        }
    }
    if ctrl && is_key_pressed(KeyCode::V) && app.workbench.clipboard.is_some() {
        app.workbench.pasting = true;
        app.workbench.tool = Tool::Pencil;
        app.message = "Clique pour placer l'ensemble. R : tourner · Échap : abandonner.".into();
        return true;
    }
    if app.workbench.pasting && is_key_pressed(KeyCode::R) && !ctrl {
        if let Some(stamp) = &mut app.workbench.clipboard {
            stamp.rotate();
        }
        app.workbench.preview.clear();
        return true;
    }
    if app.workbench.pasting {
        if let Some(pos) = app.hover() {
            set_paste_preview(app, pos);
            if is_mouse_button_pressed(MouseButton::Left) {
                let mut doc = app.scene.document.clone();
                let stamp = app.workbench.clipboard.as_ref().unwrap();
                match stamp.paste(&mut doc, pos, &app.editor.layers, false) {
                    Ok(()) => apply(app, doc, "Ensemble placé. Ctrl+Z annule toute la pose."),
                    Err(e) => app.message = e,
                }
                return true;
            }
        }
        return false;
    }
    if app.workbench.tool == Tool::Pencil {
        return false;
    }
    let hover = app.hover();
    if let Some(pos) = hover {
        if is_mouse_button_pressed(MouseButton::Left) {
            if app.workbench.tool == Tool::Move
                && !app.workbench.region.is_some_and(|r| r.contains(pos))
            {
                app.message = "Glisse depuis l'intérieur de la sélection.".into();
                return true;
            }
            if app.workbench.tool == Tool::Fill {
                if !matches!(app.editor.brush, Brush::Floor(_)) {
                    app.message = "Choisis un sol pour utiliser le remplissage.".into();
                    return true;
                }
                let points = author::fill_cells(&app.scene.document, pos);
                commit_shape(app, &points);
                return true;
            }
            app.workbench.gesture = Some(pos);
        }
        if let Some(start) = app.workbench.gesture {
            if is_mouse_button_down(MouseButton::Left) {
                let points = if matches!(app.workbench.tool, Tool::Line | Tool::Rectangle) {
                    author::shape(app.workbench.tool, start, pos, app.editor.brush)
                } else if app.workbench.tool == Tool::Move {
                    app.workbench
                        .region
                        .map(|r| {
                            Region {
                                x: r.x + pos.x - start.x,
                                y: r.y + pos.y - start.y,
                                ..r
                            }
                            .cells()
                            .collect()
                        })
                        .unwrap_or_default()
                } else {
                    Region::between(start, pos).cells().collect()
                };
                set_preview(app, points);
            }
            if is_mouse_button_released(MouseButton::Left) {
                app.workbench.gesture = None;
                app.workbench.preview.clear();
                app.workbench.preview_scene = None;
                let region = Region::between(start, pos);
                match app.workbench.tool {
                    Tool::Select => {
                        app.workbench.region = Some(region);
                        app.workbench.prop_index = app
                            .scene
                            .document
                            .props
                            .iter()
                            .rposition(|p| p.contains(pos));
                        app.workbench.marker_index =
                            app.scene.document.markers.iter().rposition(|m| {
                                Region {
                                    x: m.pos.x,
                                    y: m.pos.y,
                                    w: m.width,
                                    h: m.height,
                                }
                                .contains(pos)
                            });
                        app.selection = Some(pos);
                        app.message = format!(
                            "Sélection {} × {}. Ctrl+C : copier · R : tourner · Suppr : retirer · Déplacer : glisser.",
                            region.w, region.h
                        );
                    }
                    Tool::Move => move_region(app, start, pos),
                    Tool::Enemies | Tool::Cave | Tool::Exit => place_marker(app, region),
                    _ => {
                        let points =
                            author::shape(app.workbench.tool, start, pos, app.editor.brush);
                        commit_shape(app, &points);
                    }
                }
            }
        }
    } else if is_mouse_button_released(MouseButton::Left) {
        app.workbench.gesture = None;
        app.workbench.preview.clear();
        app.workbench.preview_scene = None;
    }
    // Consume only painting gestures: scrolling and panning still use the normal input.
    false
}
pub fn set_preview(app: &mut App, points: Vec<GridPos>) {
    if points == app.workbench.preview {
        return;
    }
    app.workbench.preview = points;
    app.workbench.preview_scene = None;
    app.workbench.preview_valid = true;
    if app.workbench.tool == Tool::Move {
        if let (Some(r), Some(at)) = (app.workbench.region, app.workbench.preview.first().copied())
        {
            let stamp = Stamp::take(&app.scene.document, r, &app.editor.layers);
            let mut doc = app.scene.document.clone();
            Stamp::remove(&mut doc, r, &app.editor.layers);
            app.workbench.preview_scene = stamp
                .paste(&mut doc, at, &app.editor.layers, true)
                .ok()
                .and_then(|()| scene::Scene::from_document(doc).ok());
            app.workbench.preview_valid = app.workbench.preview_scene.is_some();
        }
        return;
    }
    if !matches!(app.workbench.tool, Tool::Line | Tool::Rectangle) {
        return;
    }
    let mut doc = app.scene.document.clone();
    let placement = author::Placement {
        brush: app.editor.brush,
        rotation: app.editor.rotation,
        style: app.editor.wall_style,
        fixed: app.editor.fixed_wall_connections,
        blocking: app.editor.blocking,
        details: &app.editor.details,
        layers: &app.editor.layers,
    };
    app.workbench.preview_scene = author::place(&mut doc, &app.workbench.preview, &placement)
        .ok()
        .and_then(|()| scene::Scene::from_document(doc).ok());
    app.workbench.preview_valid = app.workbench.preview_scene.is_some();
}
fn set_paste_preview(app: &mut App, pos: GridPos) {
    if app.workbench.preview == [pos] {
        return;
    }
    app.workbench.preview = vec![pos];
    let mut doc = app.scene.document.clone();
    app.workbench.preview_scene = app
        .workbench
        .clipboard
        .as_ref()
        .and_then(|s| s.paste(&mut doc, pos, &app.editor.layers, false).ok())
        .and_then(|()| scene::Scene::from_document(doc).ok());
    app.workbench.preview_valid = app.workbench.preview_scene.is_some();
}
fn commit_shape(app: &mut App, points: &[GridPos]) {
    let mut doc = app.scene.document.clone();
    let placement = author::Placement {
        brush: app.editor.brush,
        rotation: app.editor.rotation,
        style: app.editor.wall_style,
        fixed: app.editor.fixed_wall_connections,
        blocking: app.editor.blocking,
        details: &app.editor.details,
        layers: &app.editor.layers,
    };
    match author::place(&mut doc, points, &placement) {
        Ok(()) => apply(app, doc, "Forme placée. Ctrl+Z annule toute l'opération."),
        Err(e) => app.message = e,
    }
}
fn move_region(app: &mut App, start: GridPos, end: GridPos) {
    let Some(r) = app.workbench.region else {
        app.message = "Sélectionne d'abord une zone avec Sélection.".into();
        return;
    };
    let at = GridPos::new(r.x + end.x - start.x, r.y + end.y - start.y);
    let stamp = Stamp::take(&app.scene.document, r, &app.editor.layers);
    let mut doc = app.scene.document.clone();
    Stamp::remove(&mut doc, r, &app.editor.layers);
    match stamp.paste(&mut doc, at, &app.editor.layers, true) {
        Ok(()) => {
            let result = app.editor.commit(&mut app.scene, doc);
            if result.is_ok() {
                app.workbench.region = Some(Region {
                    x: at.x,
                    y: at.y,
                    ..r
                });
            }
            app.report(result, "Ensemble déplacé. Ctrl+Z annule toute l'opération.");
        }
        Err(e) => app.message = e,
    }
}
fn place_marker(app: &mut App, r: Region) {
    if !app.editor.layers.editable(4) {
        app.message = "Le calque des marqueurs est verrouillé ou masqué.".into();
        return;
    }
    let kind = match app.workbench.tool {
        Tool::Enemies => MarkerKind::Enemies,
        Tool::Cave => MarkerKind::Cave,
        _ => MarkerKind::Exit,
    };
    let mut doc = app.scene.document.clone();
    doc.markers.push(Marker {
        pos: GridPos::new(r.x, r.y),
        width: r.w,
        height: r.h,
        kind,
        name: match kind {
            MarkerKind::Enemies => "Zone d'ennemis",
            MarkerKind::Cave => "Entrée de cave",
            MarkerKind::Exit => "Sortie candidate",
        }
        .into(),
        target: String::new(),
        count: 1,
    });
    let index = doc.markers.len() - 1;
    let result = app.editor.commit(&mut app.scene, doc);
    if result.is_ok() {
        app.workbench.marker_index = Some(index);
        app.workbench.prop_index = None;
        app.workbench.panel = Some(Panel::Inspector);
        app.workbench.focus = 0;
    }
    app.report(
        result,
        "Marqueur placé ; renseigne son profil ou sa destination.",
    );
}
fn center(app: &mut App) {
    if let Some(r) = app.workbench.region {
        app.camera.center = vec2(r.x as f32 + r.w as f32 / 2.0, r.y as f32 + r.h as f32 / 2.0);
    } else if let Some(pos) = app.selection {
        app.camera.center = vec2(pos.x as f32 + 0.5, pos.y as f32 + 0.5);
    } else {
        app.reset_camera();
    }
}

fn panel_input(app: &mut App, panel: Panel) {
    let p = panel_rect();
    let close = ctl(p, p.w - 116.0, p.h - 42.0, 92.0);
    if is_key_pressed(KeyCode::Escape) || clicked(close) {
        app.editor.end_stroke(&app.scene);
        close_panel(app);
        return;
    }
    let mouse: Vec2 = mouse_position().into();
    match panel {
        Panel::Catalogue => {
            if clicked(ctl(p, 24.0, 58.0, p.w - 130.0)) {
                app.workbench.focus = 1
            }
            if clicked(ctl(p, p.w - 96.0, 58.0, 72.0)) {
                app.editor.search.clear();
                app.workbench.page = 0
            }
            if app.workbench.focus == 1 {
                let old = app.editor.search.clone();
                super::edit_text(&mut app.editor.search, false, 80);
                if old != app.editor.search {
                    app.editor.scroll = 0;
                    app.workbench.page = 0;
                }
            }
            for i in 0..3 {
                if clicked(ctl(p, 24.0 + i as f32 * 108.0, 98.0, 102.0)) {
                    app.editor.category = i;
                    app.workbench.page = 0;
                }
            }
            for i in 0..3 {
                if clicked(ctl(p, p.w - 348.0 + i as f32 * 108.0, 98.0, 102.0)) {
                    app.editor.filter = i as u8;
                    app.workbench.page = 0;
                }
            }
            let names = if app.editor.category == 1 {
                &super::catalog::WALL_STYLES[..]
            } else if app.editor.category == 2 {
                &super::catalog::OBJECT_GROUPS[..]
            } else {
                &[]
            };
            for i in 0..names.len() {
                if clicked(ctl(p, 24.0 + i as f32 * 116.0, 138.0, 110.0)) {
                    if app.editor.category == 1 {
                        app.editor.wall_style = i
                    } else {
                        app.editor.object_group = i
                    }
                    app.workbench.page = 0;
                }
            }
            let (columns, rows) = catalogue_grid(p);
            let pages = app.editor.entries().len().div_ceil(columns * rows).max(1);
            let wheel = mouse_wheel().1;
            if wheel < 0.0 {
                app.workbench.page = (app.workbench.page + 1).min(pages - 1)
            }
            if wheel > 0.0 {
                app.workbench.page = app.workbench.page.saturating_sub(1)
            }
            if clicked(ctl(p, 24.0, p.h - 42.0, 96.0)) {
                app.workbench.page = app.workbench.page.saturating_sub(1)
            }
            if clicked(ctl(p, 128.0, p.h - 42.0, 96.0)) {
                app.workbench.page = (app.workbench.page + 1).min(pages - 1)
            }
            let entries = app.editor.entries();
            app.workbench.page = app.workbench.page.min(pages - 1);
            for (i, (_, brush)) in entries
                .into_iter()
                .skip(app.workbench.page * columns * rows)
                .take(columns * rows)
                .enumerate()
            {
                let r = card_rect(p, i, columns);
                let star = Rect::new(r.x + r.w - 26.0, r.y + 4.0, 22.0, 24.0);
                if clicked(star) {
                    let identity = editor::key(brush);
                    if !app.editor.favorites.remove(&identity) {
                        app.editor.favorites.insert(identity);
                    }
                    return;
                }
                if clicked(r) {
                    app.editor.choose_brush(brush);
                    app.editor.scroll = 0;
                    app.selecting = false;
                    app.selection = None;
                    app.workbench.clear_selection();
                    if matches!(app.workbench.tool, Tool::Select | Tool::Move) {
                        app.workbench.tool = Tool::Pencil;
                    }
                    close_panel(app);
                    return;
                }
            }
        }
        Panel::Layers => {
            for i in 0..5 {
                if clicked(ctl(p, 240.0, 72.0 + i as f32 * 48.0, 144.0)) {
                    app.editor.layers.visible[i] = !app.editor.layers.visible[i];
                }
                if clicked(ctl(p, 400.0, 72.0 + i as f32 * 48.0, 144.0)) {
                    app.editor.layers.locked[i] = !app.editor.layers.locked[i];
                }
            }
        }
        Panel::Inspector => inspector_input(app, p),
        Panel::Maps | Panel::Prefabs | Panel::Versions => {
            let rows = ((p.h - 150.0) / 36.0).floor().max(1.0) as usize;
            let maximum = app.workbench.files.len().saturating_sub(rows);
            let before = app.workbench.file_index;
            if is_key_pressed(KeyCode::Down) {
                app.workbench.file_index =
                    (before + 1).min(app.workbench.files.len().saturating_sub(1));
            }
            if is_key_pressed(KeyCode::Up) {
                app.workbench.file_index = before.saturating_sub(1);
            }
            if before != app.workbench.file_index {
                app.workbench.page = app.workbench.page.min(app.workbench.file_index);
                if app.workbench.file_index >= app.workbench.page + rows {
                    app.workbench.page = app.workbench.file_index + 1 - rows;
                }
                load_preview(app);
            }
            let wheel = mouse_wheel().1;
            if wheel < 0.0 {
                app.workbench.page = (app.workbench.page + 1).min(maximum)
            }
            if wheel > 0.0 {
                app.workbench.page = app.workbench.page.saturating_sub(1)
            }
            for i in 0..rows {
                let index = i + app.workbench.page;
                if index < app.workbench.files.len()
                    && clicked(ctl(
                        p,
                        24.0,
                        70.0 + i as f32 * 36.0,
                        (p.w * 0.48 - 36.0).max(100.0),
                    ))
                {
                    app.workbench.file_index = index;
                    load_preview(app);
                }
            }
            if panel == Panel::Prefabs {
                if clicked(ctl(p, p.w * 0.52, 82.0, p.w * 0.48 - 24.0)) {
                    app.workbench.focus = 1
                }
                if app.workbench.focus == 1 {
                    super::edit_text(&mut app.workbench.prefab_name, false, 80);
                }
                if clicked(ctl(p, p.w * 0.52, 120.0, p.w * 0.48 - 24.0)) {
                    if let Some(region) = app.workbench.region {
                        let mut stamp =
                            Stamp::take(&app.scene.document, region, &app.editor.layers);
                        stamp.name = app.workbench.prefab_name.clone();
                        match storage::save_stamp(&stamp) {
                            Ok(path) => {
                                app.message = format!(
                                    "Ensemble enregistré : {}",
                                    path.file_name().unwrap_or_default().to_string_lossy()
                                );
                                app.workbench.files =
                                    storage::documents(&storage::root().join("ensembles"));
                            }
                            Err(e) => app.message = e,
                        }
                    } else {
                        app.message =
                            "Sélectionne une zone avant d'enregistrer un ensemble.".into();
                    }
                }
            }
            if clicked(ctl(p, 24.0, p.h - 42.0, 144.0)) || is_key_pressed(KeyCode::Enter) {
                if let Some(path) = app.workbench.files.get(app.workbench.file_index).cloned() {
                    if panel == Panel::Prefabs {
                        match storage::read_json::<Stamp>(&path).and_then(|stamp| {
                            storage::validate_stamp(&stamp)?;
                            Ok(stamp)
                        }) {
                            Ok(stamp) => {
                                app.workbench.clipboard = Some(stamp);
                                app.workbench.pasting = true;
                                app.workbench.tool = Tool::Pencil;
                                close_panel(app);
                                app.message="Ensemble chargé : clique pour le placer. R : tourner · Échap : revenir.".into();
                            }
                            Err(e) => app.message = e,
                        }
                    } else {
                        let _ = open_selected_map(app, panel);
                    }
                }
            }
            if panel == Panel::Maps && clicked(ctl(p, 180.0, p.h - 42.0, 144.0)) {
                open(app, Panel::Versions);
                return;
            }
            if panel == Panel::Maps && clicked(ctl(p, 336.0, p.h - 42.0, 168.0)) {
                close_panel(app);
                app.modal = Some(super::Modal::File {
                    save: false,
                    path: app.editor.path.to_string_lossy().into_owned(),
                });
            }
        }
        Panel::Issues => {
            for i in 0..4 {
                if clicked(ctl(p, 24.0 + i as f32 * 150.0, 68.0, 144.0)) {
                    app.workbench.overlay = i as u8;
                }
            }
            if clicked(ctl(p, 24.0, 110.0, 160.0)) {
                app.workbench.seed = app.workbench.seed.wrapping_add(1);
                app.workbench.random = Some(author::random_preview(
                    &app.scene.document,
                    app.workbench.seed,
                ));
            }
            if clicked(ctl(p, 200.0, 110.0, 160.0)) {
                app.workbench.random = Some(author::random_preview(
                    &app.scene.document,
                    app.workbench.seed,
                ));
            }
            if clicked(ctl(p, 376.0, 110.0, 160.0)) {
                app.workbench.random = None;
            }
            let rows = ((p.h - 246.0) / 36.0).floor().max(1.0) as usize;
            let maximum = app.workbench.issues.len().saturating_sub(rows);
            if mouse_wheel().1 < 0.0 {
                app.workbench.page = (app.workbench.page + 1).min(maximum)
            }
            if mouse_wheel().1 > 0.0 {
                app.workbench.page = app.workbench.page.saturating_sub(1)
            }
            for (i, issue) in app
                .workbench
                .issues
                .iter()
                .skip(app.workbench.page)
                .take(rows)
                .enumerate()
            {
                let rect = ctl(p, 24.0, 178.0 + i as f32 * 36.0, p.w - 48.0);
                if clicked(rect) {
                    app.camera.center = vec2(issue.pos.x as f32 + 0.5, issue.pos.y as f32 + 0.5);
                    app.selection = Some(issue.pos);
                    app.workbench.panel = None;
                    return;
                }
            }
        }
        Panel::Recovery => {
            if clicked(ctl(p, 24.0, 180.0, 172.0)) {
                if let Some(recovery) = app.workbench.recovery.take() {
                    let result = app.editor.commit(&mut app.scene, recovery.document);
                    if result.is_ok() {
                        app.editor.path = recovery.path;
                        app.saved = storage::read_json(&app.editor.path).ok();
                        app.reset_camera();
                        close_panel(app);
                    }
                    app.report(
                        result,
                        "Session récupérée. Ctrl+S pour enregistrer ta carte.",
                    );
                }
            }
            if clicked(ctl(p, 208.0, 180.0, 172.0)) {
                let result = storage::dismiss_recovery();
                if result.is_ok() {
                    app.workbench.recovery = None;
                    close_panel(app);
                }
                app.message = match result {
                    Ok(()) => "Ancienne récupération archivée.".into(),
                    Err(e) => e,
                };
            }
        }
    }
    let _ = mouse;
}
fn edit_object(app: &mut App, operation: impl FnOnce(&mut PropDetails, &mut bool, &mut usize)) {
    if let Some(index) = app
        .workbench
        .prop_index
        .filter(|i| *i < app.scene.document.props.len())
    {
        let mut doc = app.scene.document.clone();
        let prop = &mut doc.props[index];
        if !app.editor.layers.editable(author::prop_layer(prop)) {
            app.message = "Le calque de cet objet est verrouillé ou masqué.".into();
            return;
        }
        operation(&mut prop.details, &mut prop.blocking, &mut prop.rotation);
        if !app.editor.layers.editable(author::prop_layer(prop)) {
            app.message = "Le calque de destination est verrouillé ou masqué.".into();
            return;
        }
        apply(app, doc, "Propriétés modifiées. Ctrl+Z pour annuler.");
    } else {
        operation(
            &mut app.editor.details,
            &mut app.editor.blocking,
            &mut app.editor.rotation,
        );
        app.message = "Propriétés de la prochaine pose modifiées.".into();
    }
}
fn inspector_input(app: &mut App, p: Rect) {
    if let Some(index) = app
        .workbench
        .marker_index
        .filter(|i| *i < app.scene.document.markers.len())
    {
        if !app.editor.layers.editable(4) {
            return;
        }
        for i in 0..2 {
            if clicked(ctl(p, 24.0, 88.0 + i as f32 * 68.0, p.w - 48.0)) {
                app.editor.end_stroke(&app.scene);
                app.workbench.focus = i as u8 + 1;
            }
        }
        if app.workbench.focus > 0 {
            let mut doc = app.scene.document.clone();
            let field = if app.workbench.focus == 1 {
                &mut doc.markers[index].name
            } else {
                &mut doc.markers[index].target
            };
            let old = field.clone();
            super::edit_text(
                field,
                false,
                if app.workbench.focus == 1 { 80 } else { 240 },
            );
            if *field != old {
                app.editor.begin_stroke(&app.scene);
                apply(app, doc, "Marqueur modifié.");
            }
        }
        for (i, delta) in [-1_i32, 1].into_iter().enumerate() {
            if clicked(ctl(p, 240.0 + i as f32 * 56.0, 232.0, 48.0)) {
                let mut doc = app.scene.document.clone();
                doc.markers[index].count =
                    (doc.markers[index].count as i32 + delta).clamp(1, 64) as u16;
                apply(app, doc, "Quantité du marqueur modifiée.");
            }
        }
        if clicked(ctl(p, 24.0, 276.0, 180.0)) {
            let mut doc = app.scene.document.clone();
            doc.markers.remove(index);
            apply(app, doc, "Marqueur supprimé.");
            app.workbench.marker_index = None;
            close_panel(app);
        }
        return;
    }
    if let Some(index) = selected_structure(app) {
        if !app.editor.layers.editable(1) {
            app.message = "Le calque des murs est verrouillé ou masqué.".into();
            return;
        }
        let part = app.scene.document.structures[index].clone();
        if clicked(ctl(p, 24., 120., 172.)) {
            let result = app.editor.rotate_at(&mut app.scene, part.pos);
            app.report(result, "Structure tournée.");
        }
        if clicked(ctl(p, 208., 120., 180.)) && part.door.is_none() {
            let result = app.editor.use_auto_wall_at(&mut app.scene, part.pos);
            app.report(result, "Raccords automatiques rétablis.");
        }
        let choices = if part.door.is_some() {
            4
        } else {
            super::catalog::WALL_STYLE_COUNT
        };
        for i in 0..choices {
            if clicked(ctl(p, 24. + i as f32 * 144., 176., 138.)) {
                let mut doc = app.scene.document.clone();
                let part = &mut doc.structures[index];
                if part.door.is_some() {
                    part.door = Some(
                        [
                            project_rl::world::DoorState::Closed,
                            project_rl::world::DoorState::Open,
                            project_rl::world::DoorState::Locked,
                            project_rl::world::DoorState::Unpowered,
                        ][i],
                    );
                } else {
                    part.style = i;
                }
                apply(app, doc, "Structure modifiée. Ctrl+Z pour annuler.");
            }
        }
        if clicked(ctl(p, 24., 232., 172.)) {
            let mut doc = app.scene.document.clone();
            doc.structures.remove(index);
            apply(app, doc, "Structure retirée ; le sol est conservé.");
            app.selection = None;
            close_panel(app);
        }
        return;
    }
    for i in 0..3 {
        if clicked(ctl(p, 24.0 + i as f32 * 228.0, 80.0, 218.0)) {
            edit_object(app, |details, blocking, _| match i {
                0 => *blocking = !*blocking,
                1 => details.opaque = Some(!details.opaque.unwrap_or(*blocking)),
                _ => {
                    details.decoration = !details.decoration;
                    if details.decoration {
                        *blocking = false;
                        details.opaque = Some(false);
                    }
                }
            });
        }
    }
    for row in 0..5 {
        for (i, delta) in [-1_i16, 1].into_iter().enumerate() {
            if clicked(ctl(
                p,
                260.0 + i as f32 * 56.0,
                136.0 + row as f32 * 44.0,
                48.0,
            )) {
                let step = if is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift) {
                    4
                } else {
                    1
                };
                edit_object(app, |details, _, _| match row {
                    0 => details.width = (details.width as i16 + delta).clamp(1, 8) as u8,
                    1 => details.height = (details.height as i16 + delta).clamp(1, 8) as u8,
                    2 => details.offset_x = (details.offset_x + delta * step).clamp(-31, 31),
                    3 => details.offset_y = (details.offset_y + delta * step).clamp(-31, 31),
                    _ => details.scale = (details.scale as i16 + delta * 5).clamp(25, 200) as u16,
                });
            }
        }
    }
    if clicked(ctl(p, 400.0, 136.0, 188.0)) {
        edit_object(app, |_, _, turns| *turns = (*turns + 1) % 4);
    }
    if clicked(ctl(p, 400.0, 180.0, 188.0)) {
        edit_object(app, |details, _, _| {
            details.offset_x = 0;
            details.offset_y = 0;
        });
    }
    if clicked(ctl(p, 400.0, 224.0, 188.0)) {
        let pos = app
            .workbench
            .prop_index
            .and_then(|i| app.scene.document.props.get(i))
            .map(|p| p.pos);
        if let Some(pos) = pos {
            let choices: Vec<_> = app
                .scene
                .document
                .props
                .iter()
                .enumerate()
                .filter(|(_, p)| p.contains(pos))
                .map(|(i, _)| i)
                .collect();
            if let Some(current) = choices
                .iter()
                .position(|i| Some(*i) == app.workbench.prop_index)
            {
                app.workbench.prop_index = Some(choices[(current + 1) % choices.len()]);
            }
        }
    }
}

fn selected_structure(app: &App) -> Option<usize> {
    if app.workbench.prop_index.is_some() || app.workbench.marker_index.is_some() {
        return None;
    }
    app.selection.and_then(|pos| {
        app.scene
            .document
            .structures
            .iter()
            .position(|part| part.pos == pos)
    })
}
fn draw_inspector(app: &App, p: Rect, font: &Font) {
    if let Some(marker) = app
        .workbench
        .marker_index
        .and_then(|i| app.scene.document.markers.get(i))
    {
        text(font, "Nom du marqueur", p.x + 24.0, p.y + 76.0, 16);
        for (i, value) in [&marker.name, &marker.target].into_iter().enumerate() {
            let r = ctl(p, 24.0, 88.0 + i as f32 * 68.0, p.w - 48.0);
            draw_rectangle(r.x, r.y, r.w, r.h, Color::from_rgba(18, 27, 30, 255));
            let short: String = value.chars().take(105).collect();
            text(font, &short, r.x + 8.0, r.y + 21.0, 16);
        }
        text(
            font,
            if marker.kind == MarkerKind::Enemies {
                "Profil d'ennemis à associer"
            } else {
                "Destination — nom ou identifiant de carte"
            },
            p.x + 24.0,
            p.y + 144.0,
            16,
        );
        text(
            font,
            &format!("Zone : {} × {} cases", marker.width, marker.height),
            p.x + 24.0,
            p.y + 214.0,
            16,
        );
        text(
            font,
            &format!("Quantité : {}", marker.count),
            p.x + 24.0,
            p.y + 254.0,
            18,
        );
        btn(font, ctl(p, 240.0, 232.0, 48.0), "−", false);
        btn(font, ctl(p, 296.0, 232.0, 48.0), "+", false);
        btn(
            font,
            ctl(p, 24.0, 276.0, 180.0),
            "Supprimer le marqueur",
            false,
        );
        text(
            font,
            "Les marqueurs préparent le tirage. Les connexions à la campagne viendront séparément.",
            p.x + 24.0,
            p.y + 334.0,
            14,
        );
        return;
    }
    if let Some(index) = selected_structure(app) {
        let part = &app.scene.document.structures[index];
        text(
            font,
            &format!(
                "{} — case ({}, {})",
                if part.door.is_some() {
                    "Porte sélectionnée"
                } else {
                    "Mur sélectionné"
                },
                part.pos.x,
                part.pos.y
            ),
            p.x + 24.,
            p.y + 82.,
            18,
        );
        btn(
            font,
            ctl(p, 24., 120., 172.),
            &format!("Rotation : {}°", part.rotation * 90),
            false,
        );
        if part.door.is_none() {
            btn(
                font,
                ctl(p, 208., 120., 180.),
                "Raccords automatiques",
                part.fixed_connections.is_none(),
            );
        }
        let labels = if part.door.is_some() {
            &["Fermée", "Ouverte", "Verrouillée", "Sans courant"][..]
        } else {
            &super::catalog::WALL_STYLES[..]
        };
        for (i, name) in labels.iter().enumerate() {
            let active = if let Some(state) = part.door {
                state
                    == [
                        project_rl::world::DoorState::Closed,
                        project_rl::world::DoorState::Open,
                        project_rl::world::DoorState::Locked,
                        project_rl::world::DoorState::Unpowered,
                    ][i]
            } else {
                part.style == i
            };
            btn(
                font,
                ctl(p, 24. + i as f32 * 144., 176., 138.),
                name,
                active,
            );
        }
        btn(font, ctl(p, 24., 232., 172.), "Retirer la structure", false);
        text(
            font,
            "Les coins et les murs se raccordent selon leur famille et leur orientation.",
            p.x + 24.,
            p.y + 302.,
            16,
        );
        text(
            font,
            "Le contrôle des accès considère les portes fermées comme ouvrables.",
            p.x + 24.,
            p.y + 328.,
            16,
        );
        return;
    }
    let prop = app
        .workbench
        .prop_index
        .and_then(|i| app.scene.document.props.get(i));
    let (details, blocking, rotation) = if let Some(prop) = prop {
        (&prop.details, prop.blocking, prop.rotation)
    } else {
        (
            &app.editor.details,
            app.editor.blocking,
            app.editor.rotation,
        )
    };
    text(
        font,
        if prop.is_some() {
            "Objet sélectionné"
        } else {
            "Prochaine pose — choisir un objet dans le catalogue"
        },
        p.x + 24.0,
        p.y + 64.0,
        16,
    );
    for (i, (name, value)) in [
        ("Bloque le déplacement", blocking),
        ("Bloque la vision", details.opaque.unwrap_or(blocking)),
        ("Petite décoration", details.decoration),
    ]
    .into_iter()
    .enumerate()
    {
        btn(
            font,
            ctl(p, 24.0 + i as f32 * 228.0, 80.0, 218.0),
            name,
            value,
        );
    }
    for (row, (name, value)) in [
        ("Largeur", details.width as i16),
        ("Hauteur", details.height as i16),
        ("Décalage X (pixels)", details.offset_x),
        ("Décalage Y (pixels)", details.offset_y),
        ("Taille du dessin (%)", details.scale as i16),
    ]
    .into_iter()
    .enumerate()
    {
        text(
            font,
            &format!("{name} : {value}"),
            p.x + 24.0,
            p.y + 158.0 + row as f32 * 44.0,
            17,
        );
        btn(
            font,
            ctl(p, 260.0, 136.0 + row as f32 * 44.0, 48.0),
            "−",
            false,
        );
        btn(
            font,
            ctl(p, 316.0, 136.0 + row as f32 * 44.0, 48.0),
            "+",
            false,
        );
    }
    btn(
        font,
        ctl(p, 400.0, 136.0, 188.0),
        &format!("Rotation : {}°", rotation * 90),
        false,
    );
    btn(
        font,
        ctl(p, 400.0, 180.0, 188.0),
        "Recentrer le dessin",
        false,
    );
    btn(
        font,
        ctl(p, 400.0, 224.0, 188.0),
        "Objet superposé suivant",
        false,
    );
    text(
        font,
        "Dimensions : 1 à 8 cases. La rotation transforme aussi les cases occupées.",
        p.x + 24.0,
        p.y + 372.0,
        15,
    );
    text(
        font,
        "Décalages : −31 à +31 pixels. Maj accélère le réglage. Décoration permet la superposition.",
        p.x + 24.0,
        p.y + 398.0,
        14,
    );
}
fn draw_thumbnail(
    app: &App,
    doc: &Document,
    rect: Rect,
    assets: &super::assets::Assets,
    background: &[Texture2D],
    font: &Font,
) {
    let mut cache = app.workbench.thumbnail.borrow_mut();
    if cache.as_ref().is_none_or(|job| {
        &job.document != doc
            || job.target.texture.width() != rect.w.floor()
            || job.target.texture.height() != rect.h.floor()
    }) {
        let mut view = doc.clone();
        // A saved ensemble may include only furniture, without a player start.
        if let Some(pos) = (Region {
            x: 0,
            y: 0,
            w: doc.width,
            h: doc.height,
        })
        .cells()
        .find(|p| {
            view.ground_terrain(*p) != project_rl::world::Terrain::DeepWater
                && !view.structures.iter().any(|s| s.pos == *p)
                && !view.props.iter().any(|o| o.blocking && o.contains(*p))
        }) {
            view.spawn = pos;
        }
        if let Ok(scene) = scene::Scene::from_document(view) {
            let w = rect.w.max(1.).floor() as u32;
            let h = rect.h.max(1.).floor() as u32;
            let target = render_target(w, h);
            target.texture.set_filter(FilterMode::Nearest);
            let resolution = ((w as f32 / doc.width as f32)
                .min(h as f32 / doc.height as f32)
                .ceil() as usize)
                .max(1);
            let mut paint = assets.paint.borrow().preview(resolution);
            paint.sync(&doc.paint);
            *cache = Some(Thumbnail {
                document: doc.clone(),
                scene,
                target,
                paint,
                cursor: 0,
            });
        }
    }
    if let Some(job) = cache.as_mut().filter(|job| &job.document == doc) {
        if !job.ready() {
            let timer = std::time::Instant::now();
            let (w, h) = (job.target.texture.width(), job.target.texture.height());
            let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., w, h));
            camera.render_target = Some(job.target.clone());
            set_camera(&camera);
            if job.cursor == 0 {
                clear_background(Color::from_rgba(18, 27, 30, 255));
            }
            let cell = (w / doc.width as f32).min(h / doc.height as f32);
            let offset = vec2(
                (w - doc.width as f32 * cell) / 2.,
                (h - doc.height as f32 * cell) / 2.,
            );
            let ground_count = (doc.width * doc.height) as usize;
            // Keep the canvas cache intact, even when inspecting a different file.
            std::mem::swap(&mut *assets.paint.borrow_mut(), &mut job.paint);
            let mut drawn = 0;
            while job.cursor < job.total() && drawn < 256 {
                let index = job.cursor;
                if index < ground_count {
                    let pos = GridPos::new(index as i32 % doc.width, index as i32 / doc.width);
                    super::rendering::draw_ground(
                        &job.scene,
                        assets,
                        background,
                        pos,
                        offset + vec2(pos.x as f32, pos.y as f32) * cell,
                        cell,
                    );
                } else if index - ground_count < doc.structures.len() {
                    let part = &doc.structures[index - ground_count];
                    super::rendering::draw_structure(
                        &job.scene,
                        assets,
                        part,
                        offset + vec2(part.pos.x as f32, part.pos.y as f32) * cell,
                        cell,
                    );
                } else {
                    let prop = &doc.props[index - ground_count - doc.structures.len()];
                    super::rendering::draw_prop(
                        assets,
                        prop,
                        offset + vec2(prop.pos.x as f32, prop.pos.y as f32) * cell,
                        cell,
                    );
                }
                job.cursor += 1;
                drawn += 1;
                if timer.elapsed().as_secs_f64() >= 0.004 {
                    break;
                }
            }
            std::mem::swap(&mut *assets.paint.borrow_mut(), &mut job.paint);
            set_default_camera();
        }
        draw_texture_ex(
            &job.target.texture,
            rect.x,
            rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(rect.w, rect.h)),
                flip_y: true,
                ..Default::default()
            },
        );
        if !job.ready() {
            let progress = job.cursor as f32 / job.total() as f32;
            draw_rectangle(
                rect.x,
                rect.y + rect.h - 26.,
                rect.w,
                26.,
                Color::new(0.04, 0.07, 0.08, 0.95),
            );
            text(
                font,
                &format!("Préparation de l'aperçu · {:.0} %", progress * 100.),
                rect.x + 8.,
                rect.y + rect.h - 8.,
                14,
            );
        }
    } else {
        draw_minimap(doc, rect, None, font);
    }
}

fn draw_browser(
    app: &App,
    p: Rect,
    panel: Panel,
    assets: &super::assets::Assets,
    background: &[Texture2D],
    font: &Font,
) {
    let rows = ((p.h - 150.0) / 36.0).floor().max(1.0) as usize;
    for (i, path) in app
        .workbench
        .files
        .iter()
        .skip(app.workbench.page)
        .take(rows)
        .enumerate()
    {
        let name = path.file_stem().unwrap_or_default().to_string_lossy();
        let name: String = name.chars().take(44).collect();
        btn(
            font,
            ctl(
                p,
                24.0,
                70.0 + i as f32 * 36.0,
                (p.w * 0.48 - 36.0).max(100.0),
            ),
            &name,
            app.workbench.file_index == i + app.workbench.page,
        );
    }
    if app.workbench.files.is_empty() {
        text(
            font,
            "Aucun fichier ici pour le moment.",
            p.x + 24.0,
            p.y + 102.0,
            16,
        );
    }
    if panel == Panel::Prefabs {
        text(
            font,
            "Nom de la sélection à enregistrer",
            p.x + p.w * 0.52,
            p.y + 70.0,
            16,
        );
        let r = ctl(p, p.w * 0.52, 82.0, p.w * 0.48 - 24.0);
        draw_rectangle(r.x, r.y, r.w, r.h, Color::from_rgba(18, 27, 30, 255));
        text(font, &app.workbench.prefab_name, r.x + 8.0, r.y + 21.0, 16);
        btn(
            font,
            ctl(p, p.w * 0.52, 120.0, p.w * 0.48 - 24.0),
            "Enregistrer la sélection",
            false,
        );
        if let Some(path) = app.workbench.files.get(app.workbench.file_index) {
            if let Ok(stamp) = storage::read_json::<Stamp>(path) {
                text(
                    font,
                    &format!(
                        "{} × {} · {} murs · {} objets",
                        stamp.width,
                        stamp.height,
                        stamp.structures.len(),
                        stamp.props.len()
                    ),
                    p.x + p.w * 0.52,
                    p.y + 186.0,
                    16,
                );
            }
        }
        if let Some(doc) = &app.workbench.file_doc {
            draw_thumbnail(
                app,
                doc,
                Rect::new(
                    p.x + p.w * 0.52,
                    p.y + 210.,
                    p.w * 0.48 - 24.,
                    (p.h - 300.).min(270.),
                ),
                assets,
                background,
                font,
            );
        }
    } else if let Some(doc) = &app.workbench.file_doc {
        let r = Rect::new(
            p.x + p.w * 0.52,
            p.y + 70.0,
            p.w * 0.48 - 24.0,
            (p.h - 190.0).min(320.0),
        );
        draw_thumbnail(app, doc, r, assets, background, font);
        text(
            font,
            &format!(
                "{} × {} cases · {} objets · {} marqueurs",
                doc.width,
                doc.height,
                doc.props.len(),
                doc.markers.len()
            ),
            p.x + p.w * 0.52,
            p.y + p.h - 82.0,
            15,
        );
    }
    btn(
        font,
        ctl(p, 24.0, p.h - 42.0, 144.0),
        if panel == Panel::Prefabs {
            "Placer l'ensemble"
        } else {
            "Ouvrir"
        },
        true,
    );
    if panel == Panel::Maps {
        btn(font, ctl(p, 180.0, p.h - 42.0, 144.0), "Versions", false);
        btn(
            font,
            ctl(p, 336.0, p.h - 42.0, 168.0),
            "Autre chemin…",
            false,
        );
    }
}
pub fn minimap_rect(bounds: Rect) -> Rect {
    Rect::new(bounds.x + bounds.w - 148.0, bounds.y + 8.0, 140.0, 92.0)
}
fn draw_cached_minimap(app: &App, rect: Rect, font: &Font) {
    let doc = &app.scene.document;
    let mut cache = app.workbench.minimap.borrow_mut();
    if cache.as_ref().is_none_or(|(old, target)| {
        old != doc
            || target.texture.width() != rect.w.floor()
            || target.texture.height() != rect.h.floor()
    }) {
        let (w, h) = (rect.w.max(1.).floor() as u32, rect.h.max(1.).floor() as u32);
        let target = render_target(w, h);
        target.texture.set_filter(FilterMode::Nearest);
        let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., w as f32, h as f32));
        camera.render_target = Some(target.clone());
        set_camera(&camera);
        clear_background(BLACK);
        draw_minimap(doc, Rect::new(0., 0., w as f32, h as f32), None, font);
        set_default_camera();
        // The target owns the render pass and its attachment, so retain both.
        *cache = Some((doc.clone(), target));
    }
    if let Some((_, target)) = cache.as_ref() {
        draw_texture_ex(
            &target.texture,
            rect.x,
            rect.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(rect.size()),
                flip_y: true,
                ..Default::default()
            },
        );
        let view = super::navigation::visible_region(app);
        draw_rectangle_lines(
            rect.x + view.x * rect.w / doc.width as f32,
            rect.y + view.y * rect.h / doc.height as f32,
            view.w * rect.w / doc.width as f32,
            view.h * rect.h / doc.height as f32,
            1.,
            Color::from_rgba(179, 244, 219, 255),
        );
    }
}
fn draw_minimap(doc: &Document, rect: Rect, center: Option<Vec2>, font: &Font) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::from_rgba(12, 20, 23, 245),
    );
    let (sx, sy) = (rect.w / doc.width as f32, rect.h / doc.height as f32);
    for y in 0..doc.height {
        for x in 0..doc.width {
            let floor = doc.floors[(y * doc.width + x) as usize].unwrap_or(6);
            let color = match floor {
                8 => Color::from_rgba(77, 145, 151, 255),
                9 => Color::from_rgba(24, 77, 110, 255),
                0 | 1 => Color::from_rgba(98, 108, 112, 255),
                2 | 3 | 18 | 22 => Color::from_rgba(148, 146, 131, 255),
                6 | 7 => Color::from_rgba(100, 91, 57, 255),
                _ => Color::from_rgba(74, 82, 80, 255),
            };
            draw_rectangle(
                rect.x + x as f32 * sx,
                rect.y + y as f32 * sy,
                sx,
                sy,
                color,
            );
        }
    }
    for part in &doc.structures {
        draw_rectangle(
            rect.x + part.pos.x as f32 * sx,
            rect.y + part.pos.y as f32 * sy,
            sx.max(1.0),
            sy.max(1.0),
            if part.door.is_some() {
                Color::from_rgba(204, 158, 59, 255)
            } else {
                Color::from_rgba(38, 49, 54, 255)
            },
        );
    }
    for prop in &doc.props {
        let (w, h) = prop.size();
        draw_rectangle(
            rect.x + prop.pos.x as f32 * sx,
            rect.y + prop.pos.y as f32 * sy,
            (w as f32 * sx).max(1.0),
            (h as f32 * sy).max(1.0),
            Color::from_rgba(96, 166, 146, 255),
        );
    }
    draw_circle(
        rect.x + (doc.spawn.x as f32 + 0.5) * sx,
        rect.y + (doc.spawn.y as f32 + 0.5) * sy,
        2.5,
        SKYBLUE,
    );
    if let Some(center) = center {
        draw_circle_lines(
            rect.x + center.x * sx,
            rect.y + center.y * sy,
            5.0,
            1.0,
            WHITE,
        );
    }
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        Color::from_rgba(159, 184, 182, 255),
    );
    let _ = font;
}
pub fn draw_world(
    app: &App,
    assets: &super::assets::Assets,
    background: &[Texture2D],
    font: &Font,
) {
    if app.testing {
        return;
    }
    let bounds = app.bounds();
    let start = app
        .camera
        .origin(app.scene.document.width, app.scene.document.height, bounds);
    let cell = app.camera.cell;
    let point = |p: GridPos| start + vec2(p.x as f32, p.y as f32) * cell;
    let region = |r: Region, color: Color| {
        let p = point(GridPos::new(r.x, r.y));
        draw_rectangle_lines(p.x, p.y, r.w as f32 * cell, r.h as f32 * cell, 2.0, color);
    };
    if let Some(r) = app.workbench.region {
        region(r, Color::from_rgba(245, 202, 103, 255));
    }
    let preview_cells: Vec<_> = if app.workbench.pasting {
        app.workbench
            .clipboard
            .as_ref()
            .zip(app.hover())
            .map(|(stamp, pos)| {
                Region {
                    x: pos.x,
                    y: pos.y,
                    w: stamp.width,
                    h: stamp.height,
                }
                .cells()
                .collect()
            })
            .unwrap_or_default()
    } else {
        app.workbench.preview.clone()
    };
    if let Some(proposal) = &app.workbench.preview_scene {
        assets.paint.borrow_mut().sync(&proposal.document.paint);
        for pos in &preview_cells {
            if pos.x < 0
                || pos.y < 0
                || pos.x >= proposal.document.width
                || pos.y >= proposal.document.height
            {
                continue;
            }
            if app.editor.layers.visible[0] {
                super::rendering::draw_floor(
                    proposal,
                    assets,
                    background,
                    *pos,
                    point(*pos),
                    cell,
                    app.editor.layers.visible[1],
                );
            }
        }
        for part in &proposal.document.structures {
            if app.editor.layers.visible[1]
                && preview_cells
                    .iter()
                    .any(|p| (p.x - part.pos.x).abs() + (p.y - part.pos.y).abs() <= 1)
            {
                super::rendering::draw_structure_tinted(
                    proposal,
                    assets,
                    part,
                    point(part.pos),
                    cell,
                    Color::new(1., 1., 1., 0.75),
                );
            }
        }
        for prop in &proposal.document.props {
            if app.editor.layers.visible[author::prop_layer(prop)]
                && preview_cells.iter().any(|p| prop.contains(*p))
            {
                super::rendering::draw_prop_tinted(
                    assets,
                    prop,
                    point(prop.pos),
                    cell,
                    Color::new(1., 1., 1., 0.75),
                );
            }
        }
        assets.paint.borrow_mut().sync(&app.scene.document.paint);
    }
    for p in &preview_cells {
        let at = point(*p);
        let color = if app.workbench.preview_valid {
            Color::new(0.2, 0.8, 0.65, 0.22)
        } else {
            Color::new(0.95, 0.2, 0.2, 0.35)
        };
        draw_rectangle(at.x, at.y, cell, cell, color);
        draw_rectangle_lines(
            at.x,
            at.y,
            cell,
            cell,
            1.,
            if app.workbench.preview_valid {
                GREEN
            } else {
                RED
            },
        );
    }
    if app.editor.layers.visible[4] {
        for marker in &app.scene.document.markers {
            let color = match marker.kind {
                MarkerKind::Enemies => Color::from_rgba(239, 126, 85, 255),
                MarkerKind::Cave => Color::from_rgba(185, 145, 243, 255),
                MarkerKind::Exit => Color::from_rgba(111, 224, 153, 255),
            };
            region(
                Region {
                    x: marker.pos.x,
                    y: marker.pos.y,
                    w: marker.width,
                    h: marker.height,
                },
                color,
            );
            let p = point(marker.pos);
            text(
                font,
                match marker.kind {
                    MarkerKind::Enemies => "E",
                    MarkerKind::Cave => "C",
                    MarkerKind::Exit => "S",
                },
                p.x + 4.0,
                p.y + 18.0,
                16,
            );
        }
    }
    if let Some(random) = &app
        .workbench
        .random
        .as_ref()
        .filter(|_| app.editor.layers.visible[4])
    {
        for (kind, positions) in [
            ("E", random.enemies.clone()),
            ("C", random.cave.into_iter().collect()),
            ("S", random.exit.into_iter().collect()),
        ] {
            for pos in positions {
                let p = point(pos);
                draw_circle(
                    p.x + cell / 2.0,
                    p.y + cell / 2.0,
                    cell * 0.25,
                    Color::new(0.8, 0.2, 0.3, 0.75),
                );
                text(font, kind, p.x + cell * 0.38, p.y + cell * 0.66, 16);
            }
        }
    }
    if app.workbench.overlay != 0 {
        let accessible = &app.workbench.accessible;
        let left = ((bounds.x - start.x) / cell).floor().max(0.0) as i32;
        let top = ((bounds.y - start.y) / cell).floor().max(0.0) as i32;
        let right =
            (((bounds.x + bounds.w - start.x) / cell).ceil() as i32).min(app.scene.document.width);
        let bottom =
            (((bounds.y + bounds.h - start.y) / cell).ceil() as i32).min(app.scene.document.height);
        for y in top..bottom {
            for x in left..right {
                let pos = GridPos::new(x, y);
                let marked = match app.workbench.overlay {
                    1 => !app
                        .workbench
                        .inspection_scene
                        .as_ref()
                        .unwrap_or(&app.scene)
                        .game
                        .map()
                        .is_walkable(pos),
                    2 => app
                        .workbench
                        .inspection_scene
                        .as_ref()
                        .unwrap_or(&app.scene)
                        .game
                        .map()
                        .blocks_vision(pos),
                    _ => !accessible.contains(&pos),
                };
                if marked {
                    let p = point(pos);
                    draw_rectangle(p.x, p.y, cell, cell, Color::new(0.95, 0.2, 0.2, 0.3));
                }
            }
        }
    }
}

fn catalogue_grid(p: Rect) -> (usize, usize) {
    (
        ((p.w - 48.0) / 144.0).floor().max(1.0) as usize,
        ((p.h - 236.0) / 112.0).floor().max(1.0) as usize,
    )
}
fn card_rect(p: Rect, i: usize, columns: usize) -> Rect {
    let w = (p.w - 48.0) / columns as f32;
    Rect::new(
        p.x + 24.0 + (i % columns) as f32 * w,
        p.y + 180.0 + (i / columns) as f32 * 112.0,
        w - 8.0,
        104.0,
    )
}
pub fn draw_ui(app: &App, assets: &super::assets::Assets, background: &[Texture2D], font: &Font) {
    if !app.testing {
        for i in 0..17 {
            let name = if i < 9 {
                author::TOOLS[i].1
            } else {
                [
                    "Catalogue",
                    "Calques",
                    "Propriétés",
                    "Cartes",
                    "Ensembles",
                    "Vérifier",
                    "Centrer",
                    "Animer · F9",
                ][i - 9]
            };
            btn(
                font,
                toolbar_rect(i),
                name,
                (i < 9 && app.workbench.tool == author::TOOLS[i].0) || (i == 16 && app.animated),
            );
        }
        draw_cached_minimap(app, minimap_rect(app.bounds()), font);
    }
    let Some(panel) = app.workbench.panel else {
        return;
    };
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.72),
    );
    let p = panel_rect();
    draw_rectangle(p.x, p.y, p.w, p.h, Color::from_rgba(25, 36, 46, 255));
    draw_rectangle_lines(p.x, p.y, p.w, p.h, 1., Color::from_rgba(69, 101, 110, 255));
    let title = match panel {
        Panel::Catalogue => "Catalogue — recherche et favoris",
        Panel::Layers => "Calques",
        Panel::Inspector => "Propriétés",
        Panel::Maps => "Cartes enregistrées",
        Panel::Prefabs => "Ensembles réutilisables",
        Panel::Versions => "Versions précédentes",
        Panel::Issues => "Contrôle de la carte",
        Panel::Recovery => "Une session peut être récupérée",
    };
    text(font, title, p.x + 24.0, p.y + 34.0, 23);
    match panel {
        Panel::Catalogue => {
            let search = ctl(p, 24.0, 58.0, p.w - 130.0);
            draw_rectangle(
                search.x,
                search.y,
                search.w,
                search.h,
                Color::from_rgba(18, 27, 30, 255),
            );
            text(
                font,
                if app.editor.search.is_empty() {
                    "Rechercher un élément…"
                } else {
                    &app.editor.search
                },
                search.x + 8.0,
                search.y + 21.0,
                16,
            );
            btn(font, ctl(p, p.w - 96.0, 58.0, 72.0), "Vider", false);
            for i in 0..3 {
                btn(
                    font,
                    ctl(p, 24.0 + i as f32 * 108.0, 98.0, 102.0),
                    ["Sols", "Murs", "Objets"][i],
                    app.editor.category == i,
                );
                btn(
                    font,
                    ctl(p, p.w - 348.0 + i as f32 * 108.0, 98.0, 102.0),
                    ["Tout", "Favoris", "Récents"][i],
                    app.editor.filter == i as u8,
                );
            }
            let (names, selected) = if app.editor.category == 1 {
                (&super::catalog::WALL_STYLES[..], app.editor.wall_style)
            } else if app.editor.category == 2 {
                (&super::catalog::OBJECT_GROUPS[..], app.editor.object_group)
            } else {
                (&[][..], 0)
            };
            for (i, name) in names.iter().enumerate() {
                btn(
                    font,
                    ctl(p, 24.0 + i as f32 * 116.0, 138.0, 110.0),
                    name,
                    selected == i,
                );
            }
            let (columns, rows) = catalogue_grid(p);
            let entries = app.editor.entries();
            let pages = entries.len().div_ceil(columns * rows).max(1);
            for (i, (name, brush)) in entries
                .iter()
                .skip(app.workbench.page * columns * rows)
                .take(columns * rows)
                .enumerate()
            {
                let r = card_rect(p, i, columns);
                draw_rectangle(
                    r.x,
                    r.y,
                    r.w,
                    r.h,
                    if app.editor.brush == *brush {
                        Color::from_rgba(60, 85, 80, 255)
                    } else {
                        Color::from_rgba(26, 35, 38, 255)
                    },
                );
                if let Some(texture) = app.editor.texture(*brush, assets) {
                    super::sprite_rotated(
                        texture,
                        r.x + (r.w - 64.0) / 2.0,
                        r.y + 4.0,
                        64.0,
                        app.editor.preview_turns(*brush),
                        WHITE,
                    );
                }
                let short: String = name.chars().take(22).collect();
                text(font, &short, r.x + 6.0, r.y + 86.0, 13);
                btn(
                    font,
                    Rect::new(r.x + r.w - 26.0, r.y + 4.0, 22.0, 24.0),
                    if app.editor.favorites.contains(&editor::key(*brush)) {
                        "F"
                    } else {
                        "+"
                    },
                    app.editor.favorites.contains(&editor::key(*brush)),
                );
            }
            if entries.is_empty() {
                text(
                    font,
                    "Aucun élément. Essaie un autre filtre ou vide la recherche.",
                    p.x + 24.0,
                    p.y + 220.0,
                    16,
                );
            }
            btn(font, ctl(p, 24.0, p.h - 42.0, 96.0), "Précédent", false);
            btn(font, ctl(p, 128.0, p.h - 42.0, 96.0), "Suivant", false);
            text(
                font,
                &format!(
                    "{} éléments · page {}/{} · + : favori",
                    entries.len(),
                    app.workbench.page + 1,
                    pages
                ),
                p.x + 240.0,
                p.y + p.h - 20.0,
                14,
            );
        }
        Panel::Layers => {
            for (i, name) in author::LAYER_NAMES.iter().enumerate() {
                text(font, name, p.x + 24.0, p.y + 94.0 + i as f32 * 48.0, 18);
                btn(
                    font,
                    ctl(p, 240.0, 72.0 + i as f32 * 48.0, 144.0),
                    if app.editor.layers.visible[i] {
                        "Visible"
                    } else {
                        "Masqué"
                    },
                    app.editor.layers.visible[i],
                );
                btn(
                    font,
                    ctl(p, 400.0, 72.0 + i as f32 * 48.0, 144.0),
                    if app.editor.layers.locked[i] {
                        "Verrouillé"
                    } else {
                        "Modifiable"
                    },
                    app.editor.layers.locked[i],
                );
            }
            text(
                font,
                "La sélection et les outils modifient uniquement les calques visibles et déverrouillés.",
                p.x + 24.0,
                p.y + 342.0,
                15,
            );
        }
        Panel::Inspector => draw_inspector(app, p, font),
        Panel::Maps | Panel::Prefabs | Panel::Versions => {
            draw_browser(app, p, panel, assets, background, font)
        }
        Panel::Issues => {
            for i in 0..4 {
                btn(
                    font,
                    ctl(p, 24.0 + i as f32 * 150.0, 68.0, 144.0),
                    ["Sans filtre", "Déplacement", "Vision", "Accès"][i],
                    app.workbench.overlay == i as u8,
                );
            }
            btn(font, ctl(p, 24.0, 110.0, 160.0), "Graine suivante", false);
            btn(font, ctl(p, 200.0, 110.0, 160.0), "Même graine", false);
            btn(font, ctl(p, 376.0, 110.0, 160.0), "Masquer tirage", false);
            text(
                font,
                &format!(
                    "Graine {} · {} remarques · clique une remarque pour la retrouver.",
                    app.workbench.seed,
                    app.workbench.issues.len()
                ),
                p.x + 24.0,
                p.y + 164.0,
                16,
            );
            let rows = ((p.h - 246.0) / 36.0).floor().max(1.0) as usize;
            for (i, issue) in app
                .workbench
                .issues
                .iter()
                .skip(app.workbench.page)
                .take(rows)
                .enumerate()
            {
                btn(
                    font,
                    ctl(p, 24.0, 178.0 + i as f32 * 36.0, p.w - 48.0),
                    &format!("({}, {}) — {}", issue.pos.x, issue.pos.y, issue.message),
                    false,
                );
            }
            if app.workbench.issues.is_empty() {
                text(
                    font,
                    "Aucune anomalie détectée dans les contrôles actuels.",
                    p.x + 24.0,
                    p.y + 222.0,
                    17,
                );
            }
        }
        Panel::Recovery => {
            text(
                font,
                "La récupération contient les dernières modifications non enregistrées.",
                p.x + 24.0,
                p.y + 84.0,
                17,
            );
            if let Some(recovery) = &app.workbench.recovery {
                let name = recovery
                    .path
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy();
                text(
                    font,
                    &format!(
                        "{} — {} × {} cases",
                        name, recovery.document.width, recovery.document.height
                    ),
                    p.x + 24.0,
                    p.y + 128.0,
                    18,
                );
            }
            btn(font, ctl(p, 24.0, 180.0, 172.0), "Reprendre", true);
            btn(
                font,
                ctl(p, 208.0, 180.0, 172.0),
                "Archiver et ignorer",
                false,
            );
        }
    }
    let status: String = app
        .message
        .chars()
        .take((p.w / 8.).floor() as usize)
        .collect();
    text(font, &status, p.x + 24., p.y + p.h - 54., 14);
    btn(font, ctl(p, p.w - 116.0, p.h - 42.0, 92.0), "Fermer", false);
}
