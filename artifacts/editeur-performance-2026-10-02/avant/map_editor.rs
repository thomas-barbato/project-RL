//! Native standalone authoring tool; campaign generation remains untouched.
//! Run from the repository root: cargo run --locked --example map_editor
#[path = "surface_preview/assets.rs"]
mod assets;
#[path = "surface_preview/author.rs"]
mod author;
#[path = "surface_preview/catalog.rs"]
mod catalog;
#[path = "surface_preview/editor.rs"]
mod editor;
#[path = "surface_preview/landscape.rs"]
mod landscape;
#[path = "surface_preview/paint.rs"]
mod paint;
#[path = "surface_preview/rendering.rs"]
mod rendering;
#[path = "surface_preview/resize.rs"]
mod resize;
#[path = "surface_preview/review.rs"]
mod review;
#[path = "surface_preview/scene.rs"]
mod scene;
#[path = "surface_preview/sf.rs"]
mod sf;
#[path = "surface_preview/storage.rs"]
mod storage;
#[path = "surface_preview/urban.rs"]
mod urban;
#[path = "surface_preview/workbench.rs"]
mod workbench;

use editor::{Brush, Editor};
use macroquad::prelude::*;
use project_rl::game::CommandOutcome;
use project_rl::world::{Direction, DoorState, GridPos};
use scene::Scene;
use std::path::PathBuf;

fn window_conf() -> Conf {
    Conf {
        window_title: if std::env::args().any(|arg| arg.starts_with("--capture")) {
            "Project RL — capture automatique".into()
        } else {
            "Project RL — éditeur de cartes".into()
        },
        window_width: 1360,
        window_height: 840,
        high_dpi: false,
        sample_count: 1,
        ..Default::default()
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

fn sprite_rotated(texture: &Texture2D, x: f32, y: f32, cell: f32, turns: usize, color: Color) {
    draw_texture_ex(
        texture,
        x,
        y,
        color,
        DrawTextureParams {
            dest_size: Some(vec2(cell, cell)),
            rotation: turns as f32 * std::f32::consts::FRAC_PI_2,
            ..Default::default()
        },
    );
}

#[derive(Clone, Copy)]
struct Camera {
    center: Vec2,
    cell: f32,
}

impl Camera {
    fn origin(&self, width: i32, height: i32, bounds: Rect) -> Vec2 {
        let world = vec2(width as f32, height as f32) * self.cell;
        let mut result = bounds.point() + bounds.size() / 2.0 - self.center * self.cell;
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

    fn cell_at(&self, mouse: Vec2, width: i32, height: i32, bounds: Rect) -> Option<GridPos> {
        let point = self.point_at(mouse, width, height, bounds)?;
        Some(GridPos::new(point.x.floor() as i32, point.y.floor() as i32))
    }

    fn point_at(&self, mouse: Vec2, width: i32, height: i32, bounds: Rect) -> Option<Vec2> {
        if !bounds.contains(mouse) {
            return None;
        }
        let point = (mouse - self.origin(width, height, bounds)) / self.cell;
        (point.x >= 0.0 && point.y >= 0.0 && point.x < width as f32 && point.y < height as f32)
            .then_some(point)
    }
}

enum Modal {
    Dimensions {
        width: String,
        height: String,
        focus: usize,
        resize: bool,
        anchor: usize,
    },
    File {
        save: bool,
        path: String,
    },
    Quit,
}

struct App {
    scene: Scene,
    editor: Editor,
    camera: Camera,
    testing: bool,
    visibility: bool,
    grid: bool,
    pointer_preview: bool,
    selecting: bool,
    selection: Option<GridPos>,
    drag_from: Option<GridPos>,
    last_painted: Option<GridPos>,
    last_mouse: Vec2,
    saved: Option<scene::Document>,
    modal: Option<Modal>,
    message: String,
    quit: bool,
    workbench: workbench::Workbench,
}

impl App {
    fn new() -> Self {
        let mut editor = Editor::new(
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("artifacts/map-editor/maps/carte.json"),
        );
        editor.active = true;
        let automatic =
            std::env::args_os().any(|arg| arg.to_string_lossy().starts_with("--capture"));
        if !automatic {
            storage::load_preferences(&mut editor);
        }
        let scene = if !automatic {
            storage::read_json::<scene::Document>(&editor.path)
                .ok()
                .and_then(|doc| Scene::from_document(doc).ok())
                .unwrap_or_else(Scene::new)
        } else {
            Scene::new()
        };
        let saved = Some(scene.document.clone());
        let mut workbench = workbench::Workbench::new();
        if !automatic && !std::env::args_os().any(|arg| arg == "--map") {
            workbench.recovery = storage::load_recovery();
            if workbench.recovery.is_some() {
                workbench.panel = Some(workbench::Panel::Recovery);
            }
        }
        Self {
            scene,
            editor,
            camera: Camera {
                center: vec2(10.0, 5.0),
                cell: 48.0,
            },
            testing: false,
            visibility: false,
            grid: false,
            pointer_preview: true,
            selecting: false,
            selection: None,
            drag_from: None,
            last_painted: None,
            last_mouse: Vec2::ZERO,
            saved,
            modal: None,
            message: "Choisis un sol, une famille de murs ou un objet. Taille permet de redimensionner la carte."
                .into(),
            quit: false,
            workbench,
        }
    }

    fn dirty(&self) -> bool {
        self.saved.as_ref() != Some(&self.scene.document)
    }

    fn bounds(&self) -> Rect {
        let left = if self.testing { 24.0 } else { 304.0 };
        let top = if self.testing {
            88.0
        } else {
            88.0 + workbench::toolbar_height()
        };
        Rect::new(
            left,
            top,
            (screen_width() - left - 24.0).max(64.0),
            (screen_height() - top - 96.0).max(64.0),
        )
    }

    fn hover(&self) -> Option<GridPos> {
        self.camera
            .cell_at(
                mouse_position().into(),
                self.scene.document.width,
                self.scene.document.height,
                self.bounds(),
            )
            .filter(|_| {
                self.testing
                    || !workbench::minimap_rect(self.bounds()).contains(mouse_position().into())
            })
    }

    fn report(&mut self, result: Result<bool, String>, success: &str) {
        self.message = match result {
            Ok(_) => success.into(),
            Err(error) => format!("Action impossible : {error}"),
        };
    }

    fn save(&mut self) -> bool {
        match self.editor.save(&self.scene) {
            Ok(()) => {
                self.saved = Some(self.scene.document.clone());
                if !std::env::args().any(|arg| arg.starts_with("--capture")) {
                    let _ = storage::save_preferences(&self.editor);
                }
                self.message = format!("Carte sauvegardée : {}", self.editor.path.display());
                true
            }
            Err(error) => {
                self.message = format!("Sauvegarde impossible : {error}");
                false
            }
        }
    }

    fn reset_camera(&mut self) {
        self.workbench.clear_selection();
        self.editor.end_stroke(&self.scene);
        self.camera.center = vec2(
            self.scene.document.width as f32 / 2.0,
            self.scene.document.height as f32 / 2.0,
        );
        self.selection = None;
        self.drag_from = None;
        self.last_painted = None;
    }

    fn toggle_test(&mut self) {
        self.editor.end_stroke(&self.scene);
        self.testing = !self.testing;
        self.scene = Scene::from_document(self.scene.document.clone()).unwrap();
        self.selection = None;
        self.message = if self.testing {
            "Test : flèches pour marcher, E près d'une porte, F pour la vision, Tab pour revenir."
                .into()
        } else {
            "Retour à l'éditeur. La carte composée est conservée.".into()
        };
    }

    fn input(&mut self) {
        if (is_mouse_button_released(MouseButton::Left)
            || is_mouse_button_released(MouseButton::Right))
            && !is_mouse_button_down(MouseButton::Left)
            && !is_mouse_button_down(MouseButton::Right)
        {
            self.editor.end_stroke(&self.scene);
        }
        prevent_quit();
        if is_quit_requested() {
            if self.dirty() {
                self.modal = Some(Modal::Quit);
            } else {
                self.quit = true;
            }
        }
        if workbench::input(self) {
            return;
        }
        if self.modal.is_some() {
            self.editor.end_stroke(&self.scene);
            self.modal_input();
            return;
        }
        if is_key_pressed(KeyCode::Escape) {
            if self.testing {
                self.toggle_test();
            } else if self.dirty() {
                self.modal = Some(Modal::Quit);
            } else {
                self.quit = true;
            }
            return;
        }
        if is_key_pressed(KeyCode::Tab) {
            self.toggle_test();
        }
        let ctrl = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);
        if ctrl && is_key_pressed(KeyCode::S) {
            self.save();
            return;
        }
        if ctrl && is_key_pressed(KeyCode::O) {
            self.modal = Some(Modal::File {
                save: false,
                path: self.editor.path.to_string_lossy().into_owned(),
            });
            return;
        }
        if ctrl && is_key_pressed(KeyCode::N) {
            self.new_dialog();
            return;
        }
        if ctrl && is_key_pressed(KeyCode::R) {
            self.resize_dialog();
            return;
        }
        let mouse: Vec2 = mouse_position().into();
        if is_mouse_button_pressed(MouseButton::Left) {
            for (i, rect) in top_buttons().into_iter().enumerate() {
                if rect.contains(mouse) {
                    match i {
                        0 => self.new_dialog(),
                        1 => self.resize_dialog(),
                        2 => {
                            let result = self.editor.restore_demo(&mut self.scene);
                            self.report(result, "Carte de démonstration restaurée. Annuler permet de retrouver ton travail.");
                            self.reset_camera();
                        }
                        3 => {
                            self.workbench.tool = if self.workbench.tool == author::Tool::Select {
                                author::Tool::Pencil
                            } else {
                                author::Tool::Select
                            };
                            self.selecting = false;
                            self.selection = None;
                        }
                        _ => self.toggle_test(),
                    }
                    return;
                }
            }
        }
        for (key, cell) in [
            (KeyCode::Key1, 32.0),
            (KeyCode::Key2, 48.0),
            (KeyCode::Key3, 64.0),
        ] {
            if is_key_pressed(key) {
                self.camera.cell = cell;
            }
        }
        if self.testing {
            self.test_input();
            return;
        }
        if is_key_pressed(KeyCode::G) {
            self.grid = !self.grid;
        }
        if ctrl && is_key_pressed(KeyCode::Z) {
            self.editor.end_stroke(&self.scene);
            let result = self.editor.undo(&mut self.scene);
            self.report(result, "Modification annulée.");
            self.selection = None;
            self.workbench.clear_selection();
        }
        if ctrl && is_key_pressed(KeyCode::Y) {
            self.editor.end_stroke(&self.scene);
            let result = self.editor.redo(&mut self.scene);
            self.report(result, "Modification rétablie.");
            self.selection = None;
            self.workbench.clear_selection();
        }
        if is_key_pressed(KeyCode::V) && !ctrl {
            self.workbench.tool = if self.workbench.tool == author::Tool::Select {
                author::Tool::Pencil
            } else {
                author::Tool::Select
            };
            self.selecting = false;
            self.selection = None;
        }
        if is_key_pressed(KeyCode::R) && !ctrl {
            if let Some(pos) = self.selection {
                let result = self.editor.rotate_at(&mut self.scene, pos);
                self.report(result, "Élément tourné de 90°.");
            } else {
                self.editor.rotate_brush();
            }
        }
        if is_key_pressed(KeyCode::A) && !ctrl {
            if let Some(pos) = self.selection {
                let result = self.editor.use_auto_wall_at(&mut self.scene, pos);
                self.report(result, "Raccords automatiques du mur rétablis.");
            } else if matches!(self.editor.brush, Brush::Wall | Brush::Corner(_)) {
                self.editor.brush = Brush::Wall;
                self.editor.fixed_wall_connections = None;
                self.editor.rotation = 0;
                self.message = "Les prochains murs suivront leurs voisins automatiquement.".into();
            }
        }
        if is_key_pressed(KeyCode::B) {
            if let Some(pos) = self.selection {
                let result = self.editor.toggle_blocking_at(&mut self.scene, pos);
                self.report(result, "Collision de l'objet modifiée.");
            } else {
                self.editor.blocking = !self.editor.blocking;
            }
        }
        if is_key_pressed(KeyCode::Delete) {
            if let Some(pos) = self.selection.take() {
                let result = self.editor.erase(&mut self.scene, pos);
                self.report(result, "Élément retiré ; le sol reste en place.");
            }
        }
        if ctrl && is_key_pressed(KeyCode::C) {
            if let Some(pos) = self.selection {
                self.editor.pick(&self.scene, pos);
                self.message = "Élément copié. Ctrl+V le place sous le pointeur.".into();
            }
        }
        if ctrl && is_key_pressed(KeyCode::V) {
            if let Some(pos) = self.hover() {
                let result = self.editor.place(&mut self.scene, pos);
                self.report(result, "Copie placée.");
            }
        }
        if Editor::sidebar_contains(mouse) {
            if is_mouse_button_pressed(MouseButton::Left) {
                if Rect::new(32.0, 122.0, 57.0, 30.0).contains(mouse) {
                    self.modal = Some(Modal::File {
                        save: true,
                        path: self.editor.path.to_string_lossy().into_owned(),
                    });
                    return;
                }
                if Rect::new(93.0, 122.0, 57.0, 30.0).contains(mouse) {
                    self.modal = Some(Modal::File {
                        save: false,
                        path: self.editor.path.to_string_lossy().into_owned(),
                    });
                    return;
                }
                // Palette clicks return to placement mode, history buttons don't.
                if mouse.y >= 168.0 {
                    self.selecting = false;
                    self.selection = None;
                    self.workbench.clear_selection();
                    if matches!(
                        self.workbench.tool,
                        author::Tool::Select | author::Tool::Move
                    ) {
                        self.workbench.tool = author::Tool::Pencil;
                    }
                } else if mouse.y >= 122. && mouse.x >= 154. {
                    self.workbench.clear_selection();
                }
            }
            if let Some(result) = self.editor.ui_input(&mut self.scene) {
                self.message = result.unwrap_or_else(|error| error);
            }
        }
        if self.bounds().contains(mouse) {
            let wheel = mouse_wheel().1;
            if wheel != 0.0 {
                let shift = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
                if shift && self.editor.round_ground() {
                    self.editor.resize_brush(wheel > 0.0);
                } else {
                    const ZOOMS: [f32; 6] = [16.0, 24.0, 32.0, 48.0, 64.0, 96.0];
                    let index = ZOOMS
                        .iter()
                        .position(|&size| size == self.camera.cell)
                        .unwrap_or(3);
                    let next = if wheel > 0.0 {
                        (index + 1).min(5)
                    } else {
                        index.saturating_sub(1)
                    };
                    self.camera.cell = ZOOMS[next];
                }
            }
            if is_mouse_button_down(MouseButton::Middle)
                && !is_mouse_button_pressed(MouseButton::Middle)
            {
                self.camera.center -= (mouse - self.last_mouse) / self.camera.cell;
            }
        }
        for (key, delta) in [
            (KeyCode::Left, vec2(-1.0, 0.0)),
            (KeyCode::Right, vec2(1.0, 0.0)),
            (KeyCode::Up, vec2(0.0, -1.0)),
            (KeyCode::Down, vec2(0.0, 1.0)),
        ] {
            if is_key_down(key) {
                self.camera.center += delta * get_frame_time() * 12.0;
            }
        }
        self.camera.center.x = self
            .camera
            .center
            .x
            .clamp(0.0, self.scene.document.width as f32);
        self.camera.center.y = self
            .camera
            .center
            .y
            .clamp(0.0, self.scene.document.height as f32);
        if let Some(pos) = self.hover() {
            if is_key_pressed(KeyCode::P) {
                self.editor.pick(&self.scene, pos);
                self.selecting = false;
                self.selection = None;
            }
            if is_mouse_button_pressed(MouseButton::Left) && self.selecting {
                self.selection = (self.scene.props.iter().any(|prop| prop.pos == pos)
                    || self.scene.is_structure(pos))
                .then_some(pos);
                self.drag_from = self.selection;
                if self.selection.is_some() {
                    self.editor.pick(&self.scene, pos);
                }
            }
            if self.editor.round_ground()
                && !self.selecting
                && self.workbench.tool == author::Tool::Pencil
                && !self.workbench.pasting
            {
                if (is_mouse_button_down(MouseButton::Left)
                    || is_mouse_button_down(MouseButton::Right))
                    && !is_mouse_button_down(MouseButton::Middle)
                    && !ctrl
                {
                    let point = self
                        .camera
                        .point_at(
                            mouse,
                            self.scene.document.width,
                            self.scene.document.height,
                            self.bounds(),
                        )
                        .unwrap();
                    let erase = is_mouse_button_down(MouseButton::Right);
                    let result = self.editor.paint_to(
                        &mut self.scene,
                        paint::Point::new(
                            (point.x * 64.0).floor() as i32,
                            (point.y * 64.0).floor() as i32,
                        ),
                        erase,
                    );
                    self.report(
                        result,
                        if erase {
                            "Peinture effacée. Ctrl+Z pour annuler le trait."
                        } else {
                            "Sol peint au pinceau rond. Ctrl+Z pour annuler le trait."
                        },
                    );
                } else {
                    self.editor.break_paint_path();
                }
            } else if is_mouse_button_down(MouseButton::Left)
                && !self.selecting
                && self.workbench.tool == author::Tool::Pencil
                && !self.workbench.pasting
                && self.last_painted != Some(pos)
            {
                self.editor.begin_stroke(&self.scene);
                let result = self.editor.place(&mut self.scene, pos);
                self.report(result, "Élément placé. Ctrl+Z pour annuler.");
                self.last_painted = Some(pos);
            }
            if is_mouse_button_pressed(MouseButton::Right)
                && (!self.editor.round_ground() || self.selecting)
            {
                let result = self.editor.erase(&mut self.scene, pos);
                self.report(result, "Élément retiré ; le sol reste en place.");
                self.selection = None;
            }
            if is_mouse_button_released(MouseButton::Left) {
                if let Some(from) = self.drag_from.take() {
                    let result = self.editor.move_at(&mut self.scene, from, pos);
                    let moved = matches!(result, Ok(true));
                    self.report(result, "Élément déplacé.");
                    self.selection = Some(if moved { pos } else { from });
                }
            }
        } else {
            self.editor.break_paint_path();
        }
        if is_mouse_button_released(MouseButton::Left) {
            self.last_painted = None;
            self.drag_from = None;
        }
        self.last_mouse = mouse;
    }

    fn test_input(&mut self) {
        for (keys, direction) in [
            ([KeyCode::Up, KeyCode::Z, KeyCode::W], Direction::North),
            ([KeyCode::Right, KeyCode::D, KeyCode::D], Direction::East),
            ([KeyCode::Down, KeyCode::S, KeyCode::S], Direction::South),
            ([KeyCode::Left, KeyCode::Q, KeyCode::A], Direction::West),
        ] {
            if keys.into_iter().any(is_key_pressed) {
                self.message = if matches!(
                    self.scene.move_player(direction),
                    CommandOutcome::Rejected(_)
                ) {
                    "Passage bloqué.".into()
                } else {
                    "Déplacement effectué.".into()
                };
            }
        }
        if is_key_pressed(KeyCode::E) {
            self.message = if matches!(self.scene.interaction(), CommandOutcome::Rejected(_)) {
                "Aucune porte utilisable à proximité.".into()
            } else {
                "Porte actionnée.".into()
            };
        }
        if is_key_pressed(KeyCode::F) {
            self.visibility = !self.visibility;
        }
        if let Some(pos) = self.scene.game.player_position() {
            self.camera.center = vec2(pos.x as f32 + 0.5, pos.y as f32 + 0.5);
        }
    }

    fn new_dialog(&mut self) {
        self.modal = Some(Modal::Dimensions {
            width: "40".into(),
            height: "25".into(),
            focus: 0,
            resize: false,
            anchor: 0,
        });
    }

    fn resize_dialog(&mut self) {
        self.editor.end_stroke(&self.scene);
        self.modal = Some(Modal::Dimensions {
            width: self.scene.document.width.to_string(),
            height: self.scene.document.height.to_string(),
            focus: 0,
            resize: true,
            anchor: 0,
        });
    }

    fn modal_input(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.modal = None;
            return;
        }
        let mouse: Vec2 = mouse_position().into();
        let panel = modal_rect(matches!(
            self.modal,
            Some(Modal::Dimensions { resize: true, .. })
        ));
        let mut modal = self.modal.take().unwrap();
        let mut accept = is_key_pressed(KeyCode::Enter);
        if is_mouse_button_pressed(MouseButton::Left) {
            if Rect::new(
                panel.x + panel.w - 130.0,
                panel.y + panel.h - 48.0,
                106.0,
                30.0,
            )
            .contains(mouse)
            {
                accept = true;
            }
            if Rect::new(panel.x + 24.0, panel.y + panel.h - 48.0, 110.0, 30.0).contains(mouse) {
                return;
            }
        }
        match &mut modal {
            Modal::Dimensions {
                width,
                height,
                focus,
                resize,
                anchor,
            } => {
                if is_key_pressed(KeyCode::Tab) {
                    *focus = 1 - *focus;
                }
                if is_mouse_button_pressed(MouseButton::Left) {
                    for i in 0..2 {
                        if Rect::new(
                            panel.x + 24.0 + i as f32 * 210.0,
                            panel.y + 83.0,
                            188.0,
                            38.0,
                        )
                        .contains(mouse)
                        {
                            *focus = i;
                        }
                    }
                }
                edit_text(if *focus == 0 { width } else { height }, true, 3);
                if *resize && is_mouse_button_pressed(MouseButton::Left) {
                    for i in 0..9 {
                        if anchor_rect(panel, i).contains(mouse) {
                            *anchor = i;
                        }
                    }
                }
                if accept {
                    let dimensions = width.parse::<i32>().ok().zip(height.parse::<i32>().ok());
                    let result = dimensions
                        .ok_or_else(|| "Saisis deux dimensions valides.".to_owned())
                        .and_then(|(w, h)| {
                            if *resize {
                                self.editor.resize_map(&mut self.scene, w, h, *anchor)
                            } else {
                                self.editor.new_map(&mut self.scene, w, h)
                            }
                        });
                    if result.is_ok() {
                        self.reset_camera();
                        self.camera.cell = 32.0;
                        self.testing = false;
                        self.report(result, if *resize { "Carte redimensionnée. Ctrl+Z permet de retrouver la taille et tout le contenu précédents." } else { "Carte vide créée. Le départ est en haut à gauche ; tu peux le déplacer dans la palette Murs." });
                        return;
                    }
                    self.report(result, "");
                }
            }
            Modal::File { save, path } => {
                edit_text(path, false, 1024);
                if accept {
                    if path.trim().is_empty() {
                        self.message = "Le chemin de fichier est vide.".into();
                    } else {
                        let old_path = self.editor.path.clone();
                        self.editor.path = PathBuf::from(path.trim());
                        if *save {
                            if self.save() {
                                return;
                            }
                        } else {
                            let result = self.editor.load(&mut self.scene);
                            if result.is_ok() {
                                self.saved = Some(self.scene.document.clone());
                                self.reset_camera();
                                self.report(result, "Carte ouverte.");
                                return;
                            }
                            self.report(result, "");
                        }
                        self.editor.path = old_path;
                    }
                }
            }
            Modal::Quit => {
                if is_mouse_button_pressed(MouseButton::Left)
                    && Rect::new(panel.x + 190.0, panel.y + panel.h - 48.0, 190.0, 30.0)
                        .contains(mouse)
                {
                    self.quit = true;
                    return;
                }
                if accept && self.save() {
                    self.quit = true;
                    return;
                }
            }
        }
        self.modal = Some(modal);
    }
}

fn edit_text(value: &mut String, digits_only: bool, maximum: usize) {
    while let Some(ch) = get_char_pressed() {
        if !ch.is_control()
            && (!digits_only || ch.is_ascii_digit())
            && value.chars().count() < maximum
        {
            value.push(ch);
        }
    }
    if is_key_pressed(KeyCode::Backspace) {
        value.pop();
    }
    let ctrl = is_key_down(KeyCode::LeftControl) || is_key_down(KeyCode::RightControl);
    if ctrl && is_key_pressed(KeyCode::A) {
        value.clear();
    }
}

fn top_buttons() -> [Rect; 5] {
    std::array::from_fn(|i| Rect::new(screen_width() - 587.0 + i as f32 * 113.0, 22.0, 105.0, 32.0))
}
fn modal_rect(resize: bool) -> Rect {
    let height = if resize { 430.0 } else { 260.0 };
    Rect::new(
        (screen_width() - 620.0) / 2.0,
        (screen_height() - height) / 2.0,
        620.0,
        height,
    )
}

fn anchor_rect(panel: Rect, index: usize) -> Rect {
    Rect::new(
        panel.x + 24.0 + (index % 3) as f32 * 190.0,
        panel.y + 193.0 + (index / 3) as f32 * 34.0,
        180.0,
        28.0,
    )
}

fn button(font: &Font, rect: Rect, text: &str, active: bool) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if active {
            Color::from_rgba(64, 108, 100, 255)
        } else {
            Color::from_rgba(50, 63, 66, 255)
        },
    );
    label(font, text, rect.x + 8.0, rect.y + 22.0, 16, WHITE);
}

fn render(app: &App, assets: &assets::Assets, background: &[Texture2D], font: &Font) {
    assets.paint.borrow_mut().sync(&app.scene.document.paint);
    clear_background(Color::from_rgba(24, 28, 31, 255));
    label(
        font,
        if app.editor.sf_review {
            "ÉDITEUR — SALLES SF"
        } else if app.editor.urban_review {
            "ÉDITEUR — VILLE"
        } else {
            "ÉDITEUR DE CARTES"
        },
        24.0,
        36.0,
        25,
        Color::from_rgba(219, 224, 218, 255),
    );
    label(
        font,
        &format!(
            "{} × {} cases · tuiles 64 × 64 · zoom {} px · {}{}",
            app.scene.document.width,
            app.scene.document.height,
            app.camera.cell as u16,
            if app.testing { "test" } else { "composition" },
            if app.dirty() { " · modifiée" } else { "" }
        ),
        24.0,
        68.0,
        17,
        Color::from_rgba(163, 185, 181, 255),
    );
    for (i, rect) in top_buttons().into_iter().enumerate() {
        button(
            font,
            rect,
            [
                "Nouvelle",
                "Taille",
                "Démo",
                if app.workbench.tool == author::Tool::Select {
                    "Sélection"
                } else {
                    "Placement"
                },
                if app.testing { "Composer" } else { "Tester" },
            ][i],
            (i == 3 && app.workbench.tool == author::Tool::Select) || (i == 4 && app.testing),
        );
    }
    let bounds = app.bounds();
    let doc = &app.scene.document;
    let start = app.camera.origin(doc.width, doc.height, bounds);
    let cell = app.camera.cell;
    draw_rectangle(
        bounds.x,
        bounds.y,
        bounds.w,
        bounds.h,
        Color::from_rgba(14, 19, 21, 255),
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
    camera.zoom.y = -camera.zoom.y;
    set_camera(&camera);
    let left = ((bounds.x - start.x) / cell).floor().max(0.0) as i32;
    let top = ((bounds.y - start.y) / cell).floor().max(0.0) as i32;
    let right = (((bounds.x + bounds.w - start.x) / cell).ceil() as i32).min(doc.width);
    let bottom = (((bounds.y + bounds.h - start.y) / cell).ceil() as i32).min(doc.height);
    for y in top..bottom {
        for x in left..right {
            let p = start + vec2(x as f32, y as f32) * cell;
            if app.testing || app.editor.layers.visible[0] {
                rendering::draw_floor(
                    &app.scene,
                    assets,
                    background,
                    GridPos::new(x, y),
                    p,
                    cell,
                    app.testing || app.editor.layers.visible[1],
                );
            }
        }
    }
    for part in &doc.structures {
        if !app.testing && !app.editor.layers.visible[1] {
            continue;
        }
        let pos = part.pos;
        if pos.x < left || pos.x >= right || pos.y < top || pos.y >= bottom {
            continue;
        }
        let p = start + vec2(pos.x as f32, pos.y as f32) * cell;
        rendering::draw_structure(&app.scene, assets, part, p, cell);
    }
    for prop in &doc.props {
        if !app.testing && !app.editor.layers.visible[author::prop_layer(prop)] {
            continue;
        }
        let (w, h) = prop.size();
        let center = vec2(
            prop.pos.x as f32 + w as f32 / 2.,
            prop.pos.y as f32 + h as f32 / 2.,
        ) + vec2(prop.details.offset_x as f32, prop.details.offset_y as f32) / 64.;
        let size = vec2(w as f32, h as f32) * prop.details.scale as f32 / 100.;
        let visual = Rect::new(
            center.x - size.x / 2.,
            center.y - size.y / 2.,
            size.x + 0.0625,
            size.y + 0.0625,
        );
        if !visual.overlaps(&Rect::new(
            left as f32,
            top as f32,
            (right - left) as f32,
            (bottom - top) as f32,
        )) {
            continue;
        }
        rendering::draw_prop(
            assets,
            prop,
            start + vec2(prop.pos.x as f32, prop.pos.y as f32) * cell,
            cell,
        );
    }
    workbench::draw_world(app, assets, background, font);
    if !app.testing {
        if app.grid {
            for y in top..bottom {
                for x in left..right {
                    let p = start + vec2(x as f32, y as f32) * cell;
                    draw_rectangle_lines(p.x, p.y, cell, cell, 1.0, Color::new(0.8, 0.9, 0.9, 0.1));
                }
            }
        }
        if let Some(pos) = app.selection {
            let p = start + vec2(pos.x as f32, pos.y as f32) * cell;
            draw_rectangle_lines(
                p.x,
                p.y,
                cell,
                cell,
                3.0,
                Color::from_rgba(242, 196, 98, 255),
            );
        }
        if let Some(pos) = app.hover().filter(|_| {
            app.pointer_preview
                && app.workbench.panel.is_none()
                && app.workbench.tool == author::Tool::Pencil
                && !app.workbench.pasting
        }) {
            let p = start + vec2(pos.x as f32, pos.y as f32) * cell;
            if app.editor.round_ground() && !app.selecting && app.modal.is_none() {
                let mouse: Vec2 = mouse_position().into();
                let radius = app.editor.diameter as f32 / 128.0 * cell;
                draw_circle_lines(
                    mouse.x,
                    mouse.y,
                    radius,
                    1.5,
                    Color::from_rgba(87, 221, 205, 255),
                );
                draw_line(mouse.x - 4.0, mouse.y, mouse.x + 4.0, mouse.y, 1.0, WHITE);
                draw_line(mouse.x, mouse.y - 4.0, mouse.x, mouse.y + 4.0, 1.0, WHITE);
            } else {
                if !app.selecting && app.modal.is_none() {
                    if let Brush::Object(sprite, furniture) = app.editor.brush {
                        let prop = scene::Prop {
                            pos,
                            sprite,
                            furniture,
                            rotation: app.editor.rotation,
                            blocking: app.editor.blocking,
                            details: app.editor.details.clone(),
                        };
                        let valid = prop.cells().all(|q| {
                            q.x >= 0
                                && q.y >= 0
                                && q.x < doc.width
                                && q.y < doc.height
                                && (!app.scene.is_structure(q) || prop.details.decoration)
                                && !doc.props.iter().any(|old| {
                                    old.contains(q)
                                        && !old.details.decoration
                                        && !prop.details.decoration
                                        && !old.contains(pos)
                                })
                        });
                        rendering::draw_prop_tinted(
                            assets,
                            &prop,
                            p,
                            cell,
                            Color::new(1., 1., 1., 0.55),
                        );
                        let (w, h) = prop.size();
                        draw_rectangle_lines(
                            p.x,
                            p.y,
                            w as f32 * cell,
                            h as f32 * cell,
                            2.,
                            if valid { GREEN } else { RED },
                        );
                    } else if let Some(texture) = app.editor.texture(app.editor.brush, assets) {
                        let (texture, turns) = if app.editor.brush == Brush::Wall {
                            let (mask, turns) = app.scene.wall_preview(
                                pos,
                                app.editor.rotation,
                                app.editor.fixed_wall_connections,
                            );
                            (assets.connection(app.editor.wall_style, mask), turns)
                        } else {
                            (
                                texture,
                                if matches!(app.editor.brush, Brush::Corner(_)) {
                                    0
                                } else {
                                    app.editor.rotation
                                },
                            )
                        };
                        sprite_rotated(
                            texture,
                            p.x,
                            p.y,
                            cell,
                            turns,
                            Color::new(1.0, 1.0, 1.0, 0.55),
                        );
                    }
                }
                draw_rectangle_lines(
                    p.x,
                    p.y,
                    cell,
                    cell,
                    1.0,
                    Color::from_rgba(87, 221, 205, 255),
                );
            }
        }
    }
    if app.testing || app.editor.layers.visible[4] {
        let player = app.scene.game.player_position().unwrap();
        let p = start + vec2(player.x as f32 + 0.5, player.y as f32 + 0.5) * cell;
        draw_circle(p.x, p.y, cell * 0.27, Color::new(0.02, 0.08, 0.09, 0.7));
        let size = (cell * 0.57) as u16;
        let metrics = measure_text("@", Some(font), size, 1.0);
        label(
            font,
            "@",
            p.x - metrics.width / 2.0,
            p.y + metrics.height / 2.0,
            size,
            Color::from_rgba(84, 218, 226, 255),
        );
    }
    if app.testing && app.visibility {
        let visibility = app.scene.game.player_visibility();
        for y in top..bottom {
            for x in left..right {
                let pos = GridPos::new(x, y);
                if !visibility.is_visible(pos) {
                    let p = start + vec2(x as f32, y as f32) * cell;
                    draw_rectangle(
                        p.x,
                        p.y,
                        cell,
                        cell,
                        Color::new(
                            0.04,
                            0.06,
                            0.07,
                            if visibility.is_explored(pos) {
                                0.75
                            } else {
                                1.0
                            },
                        ),
                    );
                }
            }
        }
    }
    set_default_camera();
    if !app.testing {
        app.editor.draw_sidebar(assets, font);
    }
    let wide = screen_width() >= 1100.0;
    let y = screen_height() - 68.0;
    label(
        font,
        if app.testing {
            "Flèches / ZQSD : marcher · E : porte · F : vision du jeu · Tab : retour à la composition"
        } else if app.workbench.pasting {
            "Clic : placer l'ensemble · R : tourner · Échap : abandonner"
        } else if matches!(
            app.workbench.tool,
            author::Tool::Select | author::Tool::Move
        ) {
            "Glisser : sélectionner ou déplacer · Ctrl+C / V : copier / coller · R : tourner · Suppr : retirer"
        } else if matches!(
            app.workbench.tool,
            author::Tool::Line | author::Tool::Rectangle
        ) {
            "Glisser : préparer la forme · relâcher : placer · aperçu rouge : placement impossible"
        } else if app.workbench.tool == author::Tool::Fill {
            "Clic sur un sol : remplir la zone de même matériau, jusqu'aux murs"
        } else if matches!(
            app.workbench.tool,
            author::Tool::Enemies | author::Tool::Cave | author::Tool::Exit
        ) {
            "Glisser : créer une zone · Propriétés : profil, destination et quantité"
        } else if app.editor.round_ground() && !app.selecting {
            "Clic : peindre · clic droit : gomme · Maj + molette : taille du pinceau · V : sélectionner"
        } else {
            "Clic : placer · clic droit : retirer · R : tourner · B : obstacle / décor · V : sélectionner"
        },
        24.0,
        y,
        if wide { 17 } else { 14 },
        WHITE,
    );
    label(
        font,
        if app.testing {
            "La carte et ses modifications restent séparées de la campagne."
        } else {
            "Molette : zoom · clic milieu : caméra · P : pipette · G : grille · Ctrl+Z / Y : annuler / rétablir"
        },
        24.0,
        y + 22.0,
        if wide { 16 } else { 13 },
        Color::from_rgba(164, 177, 177, 255),
    );
    let message = app
        .message
        .chars()
        .take(if wide { 150 } else { 118 })
        .collect::<String>();
    label(
        font,
        &message,
        24.0,
        y + 46.0,
        if wide { 15 } else { 12 },
        Color::from_rgba(198, 179, 124, 255),
    );
    workbench::draw_ui(app, assets, background, font);
    if let Some(modal) = &app.modal {
        draw_modal(modal, &app.message, font, &app.scene.document);
    }
}

fn draw_modal(modal: &Modal, message: &str, font: &Font, document: &scene::Document) {
    let error = message.chars().take(84).collect::<String>();
    let has_error =
        error.contains("impossible") || error.contains("doit") || error.contains("invalid");
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.65),
    );
    let panel = modal_rect(matches!(modal, Modal::Dimensions { resize: true, .. }));
    draw_rectangle(
        panel.x,
        panel.y,
        panel.w,
        panel.h,
        Color::from_rgba(39, 48, 51, 255),
    );
    let title = match modal {
        Modal::Dimensions { resize, .. } => {
            if *resize {
                "Redimensionner la carte"
            } else {
                "Nouvelle carte"
            }
        }
        Modal::File { save, .. } => {
            if *save {
                "Sauvegarder la carte"
            } else {
                "Ouvrir une carte"
            }
        }
        Modal::Quit => "La carte contient des modifications",
    };
    label(font, title, panel.x + 24.0, panel.y + 37.0, 24, WHITE);
    match modal {
        Modal::Dimensions {
            width,
            height,
            focus,
            resize,
            anchor,
        } => {
            for (i, (name, value)) in [("Largeur en cases", width), ("Hauteur en cases", height)]
                .into_iter()
                .enumerate()
            {
                let x = panel.x + 24.0 + i as f32 * 210.0;
                label(font, name, x, panel.y + 73.0, 16, WHITE);
                draw_rectangle(
                    x,
                    panel.y + 83.0,
                    188.0,
                    38.0,
                    Color::from_rgba(21, 29, 32, 255),
                );
                if *focus == i {
                    draw_rectangle_lines(
                        x,
                        panel.y + 83.0,
                        188.0,
                        38.0,
                        2.0,
                        Color::from_rgba(99, 208, 187, 255),
                    );
                }
                label(font, value, x + 12.0, panel.y + 110.0, 22, WHITE);
            }
            label(
                font,
                "1 à 256 cases par côté · Tab : changer de champ · Ctrl+A : vider",
                panel.x + 24.0,
                panel.y + 147.0,
                15,
                Color::from_rgba(167, 188, 183, 255),
            );
            if *resize {
                label(
                    font,
                    "Conserver l'ancrage :",
                    panel.x + 24.0,
                    panel.y + 178.0,
                    16,
                    WHITE,
                );
                for (i, name) in resize::ANCHOR_NAMES.iter().enumerate() {
                    button(font, anchor_rect(panel, i), name, *anchor == i);
                }
                if let Some((w, h)) = width.parse::<i32>().ok().zip(height.parse::<i32>().ok()) {
                    match resize::impact(document, w, h, *anchor) {
                        Ok(plan) => {
                            label(
                                font,
                                &format!(
                                    "Hors limites : {} murs/portes et {} objets.",
                                    plan.removed_walls, plan.removed_objects
                                ),
                                panel.x + 24.0,
                                panel.y + 312.0,
                                16,
                                WHITE,
                            );
                            label(
                                font,
                                if plan.moves_spawn {
                                    "Le départ sera déplacé vers une case accessible."
                                } else {
                                    "Le contenu conservé et les peintures suivent l'ancrage."
                                },
                                panel.x + 24.0,
                                panel.y + 338.0,
                                15,
                                WHITE,
                            );
                        }
                        Err(_) => label(
                            font,
                            "Dimensions attendues : de 1 à 256 cases.",
                            panel.x + 24.0,
                            panel.y + 312.0,
                            16,
                            WHITE,
                        ),
                    }
                }
                if !has_error {
                    label(
                        font,
                        "Les nouvelles cases ont un sol de terre. Ctrl+Z annule le changement.",
                        panel.x + 24.0,
                        panel.y + 362.0,
                        14,
                        Color::from_rgba(167, 188, 183, 255),
                    );
                }
            } else {
                label(
                    font,
                    "La carte actuelle reste accessible avec Annuler.",
                    panel.x + 24.0,
                    panel.y + 172.0,
                    15,
                    WHITE,
                );
            }
        }
        Modal::File { path, .. } => {
            label(
                font,
                "Fichier JSON · chemin relatif au projet ou chemin complet",
                panel.x + 24.0,
                panel.y + 72.0,
                16,
                WHITE,
            );
            draw_rectangle(
                panel.x + 24.0,
                panel.y + 83.0,
                panel.w - 48.0,
                40.0,
                Color::from_rgba(21, 29, 32, 255),
            );
            let visible = path
                .chars()
                .rev()
                .take(70)
                .collect::<String>()
                .chars()
                .rev()
                .collect::<String>();
            label(font, &visible, panel.x + 33.0, panel.y + 109.0, 14, WHITE);
            label(
                font,
                "Ctrl+A : vider le champ · Entrée : valider · Échap : revenir",
                panel.x + 24.0,
                panel.y + 154.0,
                15,
                Color::from_rgba(167, 188, 183, 255),
            );
        }
        Modal::Quit => {
            label(
                font,
                "Tu peux sauvegarder avant de fermer, ou revenir à la carte.",
                panel.x + 24.0,
                panel.y + 86.0,
                17,
                WHITE,
            );
            button(
                font,
                Rect::new(panel.x + 190.0, panel.y + panel.h - 48.0, 190.0, 30.0),
                "Quitter sans sauvegarder",
                false,
            );
        }
    }
    if has_error {
        label(
            font,
            &error,
            panel.x + 24.0,
            panel.y + panel.h - 66.0,
            13,
            Color::from_rgba(233, 179, 135, 255),
        );
    }
    button(
        font,
        Rect::new(panel.x + 24.0, panel.y + panel.h - 48.0, 110.0, 30.0),
        if matches!(modal, Modal::Quit) {
            "Revenir"
        } else {
            "Annuler"
        },
        false,
    );
    button(
        font,
        Rect::new(
            panel.x + panel.w - 130.0,
            panel.y + panel.h - 48.0,
            106.0,
            30.0,
        ),
        if matches!(modal, Modal::Quit) {
            "Sauver"
        } else {
            "Valider"
        },
        true,
    );
}

async fn capture(
    path: &std::path::Path,
    app: &App,
    assets: &assets::Assets,
    background: &[Texture2D],
    font: &Font,
) {
    for _ in 0..2 {
        render(app, assets, background, font);
        next_frame().await;
    }
    render(app, assets, background, font);
    let mut image = get_screen_data();
    assert!(
        image.width > 1 && image.height > 1,
        "La fenêtre de capture n'a plus de surface de rendu utilisable."
    );
    for pixel in image.bytes.chunks_exact_mut(4) {
        pixel[3] = 255;
    }
    image.export_png(path.to_str().unwrap());
    println!(
        "Capture : {} ({} × {})",
        path.display(),
        image.width,
        image.height
    );
}

async fn capture_urban_map(
    path: &std::path::Path,
    scene: &Scene,
    assets: &assets::Assets,
    background: &[Texture2D],
) {
    let cell = 64.0;
    let width = scene.document.width as u32 * 64;
    let height = scene.document.height as u32 * 64;
    let target = render_target(width, height);
    target.texture.set_filter(FilterMode::Nearest);
    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, width as f32, height as f32));
    camera.render_target = Some(target.clone());
    assets.paint.borrow_mut().sync(&scene.document.paint);
    set_camera(&camera);
    clear_background(BLACK);
    for y in 0..scene.document.height {
        for x in 0..scene.document.width {
            rendering::draw_ground(
                scene,
                assets,
                background,
                GridPos::new(x, y),
                vec2(x as f32, y as f32) * cell,
                cell,
            );
        }
    }
    for part in &scene.document.structures {
        rendering::draw_structure(
            scene,
            assets,
            part,
            vec2(part.pos.x as f32, part.pos.y as f32) * cell,
            cell,
        );
    }
    for prop in &scene.document.props {
        rendering::draw_prop(
            assets,
            prop,
            vec2(prop.pos.x as f32, prop.pos.y as f32) * cell,
            cell,
        );
    }
    set_default_camera();
    next_frame().await;
    let mut image = target.texture.get_texture_data();
    for pixel in image.bytes.chunks_exact_mut(4) {
        pixel[3] = 255;
    }
    image.export_png(path.to_str().unwrap());
    println!("Carte native : {} ({width} × {height})", path.display());
}

async fn capture_urban_catalog(directory: &std::path::Path, assets: &assets::Assets, font: &Font) {
    // Fill the font atlas before drawing to offscreen targets. Growing it in
    // the middle of the first catalogue draw can invalidate its first label.
    for name in catalog::CITY_FLOORS
        .iter()
        .chain(catalog::CITY_OBJECTS.iter())
        .chain(catalog::SF_OBJECTS.iter())
    {
        for size in [16, 18] {
            let _ = measure_text(name, Some(font), size, 1.0);
        }
    }
    next_frame().await;
    for (file, names, textures, tiled) in [
        (
            "sols-64.png",
            &catalog::CITY_FLOORS[..],
            &assets.terrain[16..],
            true,
        ),
        (
            "mobilier-64.png",
            &catalog::CITY_OBJECTS[..16],
            &assets.furniture[8..24],
            false,
        ),
        (
            "voirie-64.png",
            &catalog::CITY_OBJECTS[16..32],
            &assets.furniture[24..40],
            false,
        ),
        (
            "technologie-64.png",
            &catalog::CITY_OBJECTS[32..],
            &assets.furniture[40..56],
            false,
        ),
        (
            "serveurs-64.png",
            &catalog::SF_OBJECTS[..16],
            &assets.furniture[56..72],
            false,
        ),
        (
            "habitat-64.png",
            &catalog::SF_OBJECTS[16..32],
            &assets.furniture[72..88],
            false,
        ),
        (
            "armurerie-64.png",
            &catalog::SF_OBJECTS[32..],
            &assets.furniture[88..104],
            false,
        ),
    ] {
        let row_height = if tiled { 184 } else { 122 };
        let width = 992;
        let height = names.len().div_ceil(4) as u32 * row_height;
        let target = render_target(width, height);
        target.texture.set_filter(FilterMode::Nearest);
        let mut camera =
            Camera2D::from_display_rect(Rect::new(0.0, 0.0, width as f32, height as f32));
        camera.render_target = Some(target.clone());
        set_camera(&camera);
        clear_background(Color::from_rgba(24, 28, 31, 255));
        for (i, (name, texture)) in names.iter().zip(textures).enumerate() {
            let x = (i % 4) as f32 * 248.0;
            let y = (i / 4) as f32 * row_height as f32;
            if tiled {
                for dy in 0..2 {
                    for dx in 0..2 {
                        sprite_rotated(
                            texture,
                            x + 60.0 + dx as f32 * 64.0,
                            y + 12.0 + dy as f32 * 64.0,
                            64.0,
                            0,
                            WHITE,
                        );
                    }
                }
            } else {
                sprite_rotated(texture, x + 92.0, y + 8.0, 64.0, 0, WHITE);
            }
            let size = if tiled { 18 } else { 16 };
            let text_width = measure_text(name, Some(font), size, 1.0).width;
            label(
                font,
                name,
                x + (248.0 - text_width) / 2.0,
                y + row_height as f32 - 20.0,
                size,
                WHITE,
            );
        }
        set_default_camera();
        next_frame().await;
        let mut image = target.texture.get_texture_data();
        for pixel in image.bytes.chunks_exact_mut(4) {
            pixel[3] = 255;
        }
        image.export_png(directory.join(file).to_str().unwrap());
        println!("Catalogue à 64 pixels : {file}");
    }
}

/// Composed demonstration, exercising the same strokes and history as the UI.
fn brush_demo(app: &mut App) {
    let mut document = Scene::empty(16, 10).unwrap().document;
    document.floors.fill(Some(7));
    document.spawn = GridPos::new(7, 7);
    for y in 1..=5 {
        for x in 1..=5 {
            document.floors[(y * 16 + x) as usize] = Some(0);
            if x == 1 || x == 5 || y == 1 || y == 5 {
                document.structures.push(scene::Structure {
                    pos: GridPos::new(x, y),
                    door: (x == 3 && y == 5).then_some(DoorState::Open),
                    rotation: 0,
                    style: 0,
                    fixed_connections: None,
                });
            }
        }
    }
    document.props = [
        (2, 2, 0, true),
        (4, 2, 15, false),
        (2, 4, 5, true),
        (4, 4, 14, false),
    ]
    .into_iter()
    .map(|(x, y, sprite, furniture)| scene::Prop {
        details: Default::default(),
        pos: GridPos::new(x, y),
        sprite,
        furniture,
        rotation: 0,
        blocking: true,
    })
    .collect();
    app.scene = Scene::from_document(document).unwrap();
    app.editor.category = 0;
    app.editor.round = true;
    app.editor.brush = Brush::Floor(Some(6));
    for (x, y, diameter) in [
        (8.0, 1.6, 104),
        (12.6, 8.2, 120),
        (1.6, 8.1, 112),
        (13.8, 1.7, 88),
    ] {
        app.editor.diameter = diameter;
        app.editor
            .paint_to(
                &mut app.scene,
                paint::Point::new((x * 64.0) as i32, (y * 64.0) as i32),
                false,
            )
            .unwrap();
        app.editor.end_stroke(&app.scene);
    }
    // A continuous S curve. The broad earth shoulder and gravel core blend.
    let points: Vec<_> = (0..=120)
        .map(|step| {
            let t = step as f32 / 120.0;
            let u = 1.0 - t;
            let point = vec2(3.5, 5.75) * u.powi(3)
                + vec2(5.8, 11.0) * (3.0 * u * u * t)
                + vec2(9.4, -1.3) * (3.0 * u * t * t)
                + vec2(15.9, 6.7) * t.powi(3);
            paint::Point::new((point.x * 64.0) as i32, (point.y * 64.0) as i32)
        })
        .collect();
    for (material, diameter) in [(6, 144), (5, 104)] {
        app.editor.brush = Brush::Floor(Some(material));
        app.editor.diameter = diameter;
        for &point in &points {
            app.editor.paint_to(&mut app.scene, point, false).unwrap();
        }
        app.editor.end_stroke(&app.scene);
    }
    app.editor.diameter = 96;
    app.editor.scroll = 5;
    app.camera.cell = 64.0;
    app.reset_camera();
    app.selecting = false;
    app.message = "Chemin courbe peint sur l'herbe : bord en terre, cœur en gravier, taches de sol libres. La grille du jeu est conservée.".into();
}

fn assert_paint_stays_local(
    before: &std::path::Path,
    after: &std::path::Path,
    app: &App,
    point: paint::Point,
) {
    let before = Image::from_file_with_format(&std::fs::read(before).unwrap(), None).unwrap();
    let after = Image::from_file_with_format(&std::fs::read(after).unwrap(), None).unwrap();
    assert_eq!((before.width, before.height), (after.width, after.height));
    let start = app.camera.origin(
        app.scene.document.width,
        app.scene.document.height,
        app.bounds(),
    );
    let mut changed = 0;
    let mut stray = vec![];
    for y in
        start.y as usize..(start.y + app.scene.document.height as f32 * app.camera.cell) as usize
    {
        for x in
            start.x as usize..(start.x + app.scene.document.width as f32 * app.camera.cell) as usize
        {
            let index = (y * before.width as usize + x) * 4;
            if before.bytes[index..index + 4] != after.bytes[index..index + 4] {
                changed += 1;
                let world =
                    (vec2(x as f32 + 0.5, y as f32 + 0.5) - start) * (64.0 / app.camera.cell);
                if (world - vec2(point.x as f32, point.y as f32)).length()
                    > app.editor.diameter as f32 / 2.0 + 1.0
                {
                    stray.push((world.x, world.y));
                }
            }
        }
    }
    println!(
        "Peinture localisée : {changed} pixels modifiés, {} hors du pinceau. Premiers : {:?}",
        stray.len(),
        &stray[..stray.len().min(4)]
    );
    assert!(changed > 0, "Le trait doit être visible.");
    assert!(
        stray.is_empty(),
        "La peinture est recopiée hors de son emplacement."
    );
}

#[macroquad::main(window_conf)]
async fn main() {
    let prepared = assets::Prepared::load();
    let assets = prepared.upload();
    let background: Vec<_> = (0..scene::HEIGHT)
        .flat_map(|y| (0..scene::WIDTH).map(move |x| (x, y)))
        .map(|(x, y)| prepared.ground(x, y))
        .collect();
    let mut font = load_ttf_font_from_bytes(include_bytes!(
        "../assets/fonts/AtkinsonHyperlegible-Regular.ttf"
    ))
    .unwrap();
    font.set_filter(FilterMode::Nearest);
    let mut app = App::new();
    if catalog::review_requested() {
        app.editor.urban_review = true;
        app.editor.category = 0;
        app.editor.scroll = 10;
        app.editor.brush = Brush::Floor(Some(16));
        app.editor.path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("artifacts/map-editor/maps/ville-essai.json");
        app.scene = urban::demo();
        app.reset_camera();
        app.camera.cell = 32.0;
        app.message =
            "LOT URBAIN À VALIDER — commerces, logements, atelier et place du marché.".into();
    }
    let args: Vec<_> = std::env::args_os().collect();
    if args
        .iter()
        .any(|arg| arg == "--sf-demo" || arg == "--sf-review" || arg == "--capture-sf")
    {
        app.scene = sf::demo();
        app.editor.sf_review = true;
        app.editor.urban_review = false;
        app.editor.category = 2;
        app.editor.object_group = 3;
        app.editor.scroll = 0;
        app.editor.choose_brush(Brush::Object(56, true));
        app.editor.path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("artifacts/map-editor/maps/salles-sf.json");
        app.reset_camera();
        app.camera.cell = 32.0;
        app.message="Catalogue SF : serveurs, armurerie, habitat et accès souterrains visuels. Taille permet de redimensionner.".into();
    }
    if let Some(index) = args.iter().position(|arg| arg == "--map") {
        app.editor.path = PathBuf::from(args.get(index + 1).expect("--map needs a file path"));
        let result = app.editor.load(&mut app.scene);
        if args.iter().any(|arg| arg == "--capture-landscape") {
            result
                .as_ref()
                .expect("The landscape file must load successfully before capture");
        }
        if result.is_ok() {
            app.saved = Some(app.scene.document.clone());
            app.reset_camera();
        }
        app.report(result, "Carte ouverte.");
    }
    if args.iter().any(|arg| arg == "--center-on-spawn") {
        app.camera.center = vec2(
            app.scene.document.spawn.x as f32 + 0.5,
            app.scene.document.spawn.y as f32 + 0.5,
        );
        app.camera.cell = 32.;
    }
    app.saved = Some(app.scene.document.clone());
    if let Some(index) = args.iter().position(|arg| arg == "--capture-landscape") {
        landscape::capture_review(
            std::path::Path::new(
                args.get(index + 1)
                    .expect("--capture-landscape needs a fresh directory"),
            ),
            &app.scene,
            &assets,
            &background,
            &font,
        )
        .await;
        return;
    }
    if let Some(index) = args.iter().position(|arg| arg == "--capture-viewport") {
        review::viewport(
            std::path::Path::new(
                args.get(index + 1)
                    .expect("--capture-viewport needs a fresh directory"),
            ),
            &assets,
            &background,
            &font,
        )
        .await;
        return;
    }
    if let Some(index) = args.iter().position(|arg| arg == "--capture-workbench") {
        review::run(
            std::path::Path::new(
                args.get(index + 1)
                    .expect("--capture-workbench needs a fresh directory"),
            ),
            &assets,
            &background,
            &font,
        )
        .await;
        return;
    }
    if let Some(index) = args.iter().position(|arg| arg == "--capture-sf") {
        let directory = PathBuf::from(
            args.get(index + 1)
                .expect("--capture-sf needs a new directory"),
        );
        std::fs::create_dir(&directory).expect("capture directory must not already exist");
        app.pointer_preview = false;
        app.editor.path = directory.join("salles-sf.json");
        assert!(app.save());
        capture_urban_catalog(&directory, &assets, &font).await;
        capture_urban_map(
            &directory.join("salles-sf-native-64.png"),
            &app.scene,
            &assets,
            &background,
        )
        .await;
        capture(
            &directory.join("editeur-sf-ensemble.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.camera.cell = 64.0;
        app.camera.center = vec2(7.0, 5.0);
        capture(
            &directory.join("serveurs-et-cameras.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.editor.category = 1;
        app.editor.wall_style = 3;
        app.editor.scroll = 0;
        app.editor.choose_brush(Brush::Wall);
        app.camera.center = vec2(17.0, 5.0);
        capture(
            &directory.join("murs-laser-armurerie.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        for (group, file, center) in [
            (4, "habitat-et-escaliers.png", vec2(7.0, 12.0)),
            (5, "palette-armurerie.png", vec2(17.0, 5.0)),
        ] {
            app.editor.category = 2;
            app.editor.object_group = group;
            app.editor.scroll = 0;
            app.camera.center = center;
            capture(&directory.join(file), &app, &assets, &background, &font).await;
        }
        app.editor.sf_review = false;
        app.editor.object_group = 0;
        brush_demo(&mut app);
        app.editor.category = 0;
        app.editor.scroll = 0;
        app.camera.cell = 32.0;
        app.editor.path = directory.join("redimensionnement.json");
        let original = app.scene.document.clone();
        capture_urban_map(
            &directory.join("taille-avant-64.png"),
            &app.scene,
            &assets,
            &background,
        )
        .await;
        app.modal = Some(Modal::Dimensions {
            width: "22".into(),
            height: "14".into(),
            focus: 0,
            resize: true,
            anchor: 0,
        });
        capture(
            &directory.join("dialogue-agrandissement.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        let valid_message = app.message.clone();
        app.modal = Some(Modal::Dimensions {
            width: "0".into(),
            height: "14".into(),
            focus: 0,
            resize: true,
            anchor: 0,
        });
        let result = app.editor.resize_map(&mut app.scene, 0, 14, 0);
        assert!(result.is_err());
        app.report(result, "Carte redimensionnée.");
        assert_eq!(app.scene.document, original);
        capture(
            &directory.join("dialogue-taille-invalide.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.message = valid_message;
        app.modal = None;
        app.editor.resize_map(&mut app.scene, 22, 14, 0).unwrap();
        app.reset_camera();
        capture_urban_map(
            &directory.join("taille-agrandie-64.png"),
            &app.scene,
            &assets,
            &background,
        )
        .await;
        let before = Image::from_file_with_format(
            &std::fs::read(directory.join("taille-avant-64.png")).unwrap(),
            None,
        )
        .unwrap();
        let after = Image::from_file_with_format(
            &std::fs::read(directory.join("taille-agrandie-64.png")).unwrap(),
            None,
        )
        .unwrap();
        let mut changed = 0;
        for y in 0..before.height as usize {
            for x in 0..before.width as usize {
                let original = (y * before.width as usize + x) * 4;
                let enlarged = (y * after.width as usize + x) * 4;
                if before.bytes[original..][..4] != after.bytes[enlarged..][..4] {
                    changed += 1;
                }
            }
        }
        assert_eq!(changed, 0, "L'agrandissement a changé le rendu existant.");
        println!(
            "Comparaison de l'agrandissement : {} pixels conservés exactement.",
            before.width as usize * before.height as usize
        );
        capture(
            &directory.join("carte-agrandie.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        assert!(app.save());
        let enlarged = app.scene.document.clone();
        app.editor.load(&mut app.scene).unwrap();
        assert_eq!(app.scene.document, enlarged);
        app.modal = Some(Modal::Dimensions {
            width: "8".into(),
            height: "7".into(),
            focus: 0,
            resize: true,
            anchor: 8,
        });
        capture(
            &directory.join("dialogue-reduction.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.modal = None;
        app.editor.resize_map(&mut app.scene, 8, 7, 8).unwrap();
        app.reset_camera();
        capture(
            &directory.join("carte-reduite.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.editor.undo(&mut app.scene).unwrap();
        assert_eq!(app.scene.document, enlarged);
        app.editor.undo(&mut app.scene).unwrap();
        assert_eq!(app.scene.document, original);
        println!("Redimensionnement natif : sauvegarde, rechargement et annulation intégrale OK.");
        request_new_screen_size(960.0, 540.0);
        next_frame().await;
        app.reset_camera();
        app.message = "Choisir la taille sans recréer la carte.".into();
        app.resize_dialog();
        capture(
            &directory.join("dialogue-taille-960.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.modal = None;
        app.scene = sf::demo();
        app.editor.sf_review = true;
        app.editor.category = 2;
        app.editor.object_group = 5;
        app.editor.scroll = 0;
        app.reset_camera();
        app.message =
            "Serveurs, habitat et armurerie sont disponibles dans les filtres Objets.".into();
        capture(
            &directory.join("editeur-sf-960.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        return;
    }
    if let Some(index) = args.iter().position(|arg| arg == "--capture-urban") {
        let directory = PathBuf::from(
            args.get(index + 1)
                .expect("--capture-urban needs a new directory"),
        );
        std::fs::create_dir(&directory).expect("capture directory must not already exist");
        app.pointer_preview = false;
        app.editor.path = directory.join("ville-essai.json");
        assert!(app.save());
        capture_urban_catalog(&directory, &assets, &font).await;
        capture_urban_map(
            &directory.join("ville-native-64.png"),
            &app.scene,
            &assets,
            &background,
        )
        .await;
        capture(
            &directory.join("editeur-vue-ensemble.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.camera.cell = 64.0;
        app.camera.center = vec2(7.0, 5.0);
        app.editor.category = 1;
        app.editor.scroll = 0;
        app.editor.wall_style = 1;
        capture(
            &directory.join("boutique-murs-brique-64.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.camera.center = vec2(20.0, 5.0);
        app.editor.wall_style = 2;
        capture(
            &directory.join("logement-murs-crepi-64.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.editor.category = 2;
        app.editor.scroll = 10;
        capture(
            &directory.join("palette-mobilier.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.editor.scroll = 26;
        capture(
            &directory.join("palette-voirie.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.editor.scroll = 42;
        capture(
            &directory.join("palette-technologie.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.scene = Scene::from_document(app.scene.document.clone()).unwrap();
        app.testing = true;
        app.camera.center = vec2(13.0, 8.0);
        capture(
            &directory.join("test-ville.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        return;
    }
    if let Some(index) = args.iter().position(|arg| arg == "--capture-brush-fixes") {
        let directory = PathBuf::from(
            args.get(index + 1)
                .expect("--capture-brush-fixes needs a new directory"),
        );
        std::fs::create_dir(&directory).expect("capture directory must not already exist");
        let mut document = Scene::empty(8, 7).unwrap().document;
        for y in 1..=5 {
            for x in 1..=5 {
                document.floors[(y * 8 + x) as usize] = Some(2);
                if x == 1 || x == 5 || y == 1 || y == 5 {
                    document.structures.push(scene::Structure {
                        pos: GridPos::new(x, y),
                        door: None,
                        rotation: 0,
                        style: 0,
                        fixed_connections: None,
                    });
                }
            }
        }
        app.scene = Scene::from_document(document).unwrap();
        app.reset_camera();
        app.camera.cell = 64.0;
        app.pointer_preview = false;
        app.editor.category = 0;
        app.editor.scroll = 5;
        app.editor.round = true;
        app.editor.brush = Brush::Floor(Some(5));
        app.editor.diameter = 16;
        app.message =
            "Contrôle : aucun trait de peinture ne doit être recopié dans les bordures des murs."
                .into();
        let base = app.scene.document.clone();
        for (name, point) in [
            ("interieur", paint::Point::new(224, 186)),
            ("exterieur", paint::Point::new(224, 4)),
            ("mur-droit", paint::Point::new(260, 224)),
            ("mur-gauche", paint::Point::new(188, 224)),
            ("mur-bas", paint::Point::new(224, 260)),
            ("coin", paint::Point::new(188, 188)),
        ] {
            app.scene = Scene::from_document(base.clone()).unwrap();
            let before = directory.join(format!("{name}-avant.png"));
            let after = directory.join(format!("{name}-apres.png"));
            capture(&before, &app, &assets, &background, &font).await;
            app.editor.paint_to(&mut app.scene, point, false).unwrap();
            app.editor.end_stroke(&app.scene);
            capture(&after, &app, &assets, &background, &font).await;
            assert_paint_stays_local(&before, &after, &app, point);
        }
        let mut document = Scene::empty(16, 10).unwrap().document;
        document.floors.fill(Some(7));
        document.spawn = GridPos::new(7, 8);
        document.structures = base.structures.clone();
        for y in 1..=5 {
            for x in 1..=5 {
                document.floors[(y * 16 + x) as usize] = Some(2);
            }
        }
        app.scene = Scene::from_document(document).unwrap();
        let points: Vec<_> = (0..=80)
            .map(|step| {
                let t = step as f32 / 80.0;
                let u = 1.0 - t;
                let point = vec2(10.0, 5.8) * u.powi(3)
                    + vec2(10.0, 1.8) * (3.0 * u * u * t)
                    + vec2(15.0, 3.3) * (3.0 * u * t * t)
                    + vec2(12.0, 6.4) * t.powi(3);
                paint::Point::new((point.x * 64.0) as i32, (point.y * 64.0) as i32)
            })
            .collect();
        for (material, diameter) in [(8, 192), (9, 96)] {
            app.editor.brush = Brush::Floor(Some(material));
            app.editor.diameter = diameter;
            for &point in &points {
                app.editor.paint_to(&mut app.scene, point, false).unwrap();
            }
            app.editor.end_stroke(&app.scene);
        }
        app.reset_camera();
        app.editor.path = directory.join("eau-pinceau-rond.json");
        assert!(app.save());
        let expected = app.scene.document.clone();
        app.editor.new_map(&mut app.scene, 4, 4).unwrap();
        app.editor.load(&mut app.scene).unwrap();
        assert_eq!(app.scene.document, expected);
        let terrain_counts = (0..10)
            .flat_map(|y| (0..16).map(move |x| GridPos::new(x, y)))
            .fold([0; 2], |mut counts, pos| {
                match app.scene.game.map().tile(pos).unwrap().terrain {
                    project_rl::world::Terrain::ShallowWater => counts[0] += 1,
                    project_rl::world::Terrain::DeepWater => counts[1] += 1,
                    _ => (),
                }
                counts
            });
        assert!(terrain_counts.iter().all(|&count| count > 0));
        println!(
            "Eau rechargée : {} cases peu profondes, {} cases profondes.",
            terrain_counts[0], terrain_counts[1]
        );
        app.editor.diameter = 96;
        app.message = "Eau peu profonde et eau profonde peintes au pinceau rond, sauvegardées puis rechargées.".into();
        capture(
            &directory.join("eau-ronde.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.toggle_test();
        capture(
            &directory.join("eau-test.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.toggle_test();
        app.message = "Eau peu profonde et eau profonde : pinceau rond, gomme et sauvegarde fonctionnent sur les deux textures.".into();
        capture(
            &directory.join("eau-editeur.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        return;
    }
    if let Some(index) = args.iter().position(|arg| arg == "--capture-brush") {
        let directory = PathBuf::from(
            args.get(index + 1)
                .expect("--capture-brush needs a new output directory"),
        );
        std::fs::create_dir(&directory).expect("capture directory must not already exist");
        brush_demo(&mut app);
        app.editor.path = directory.join("carte-pinceau-rond.json");
        app.save();
        let expected = app.scene.document.clone();
        app.editor.new_map(&mut app.scene, 4, 4).unwrap();
        app.editor.load(&mut app.scene).unwrap();
        assert_eq!(expected, app.scene.document);
        app.message = "Chemin courbe et taches de sol peints librement, sauvegardés puis rechargés. Sols > Rond pour essayer.".into();
        capture(
            &directory.join("chemin-courbe.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.grid = true;
        capture(
            &directory.join("chemin-courbe-grille.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.grid = false;
        app.editor.diameter = 80;
        app.editor
            .paint_to(&mut app.scene, paint::Point::new(9 * 64, 294), true)
            .unwrap();
        app.editor.end_stroke(&app.scene);
        capture(
            &directory.join("gomme-ronde.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.editor.undo(&mut app.scene).unwrap();
        assert_eq!(expected, app.scene.document);
        app.camera.cell = 128.0;
        app.camera.center = vec2(9.0, 6.0);
        capture(
            &directory.join("chemin-detail.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        request_new_screen_size(960.0, 540.0);
        next_frame().await;
        app.camera.cell = 32.0;
        capture(
            &directory.join("pinceau-960.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.toggle_test();
        capture(
            &directory.join("chemin-test.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        return;
    }
    if let Some(index) = args.iter().position(|arg| arg == "--capture") {
        let directory = PathBuf::from(
            args.get(index + 1)
                .expect("--capture needs a new output directory"),
        );
        std::fs::create_dir(&directory).expect("capture directory must not already exist");
        // Large repeated surfaces make tile borders and unintended seams easy
        // to inspect independently of walls and furnishing.
        let mut floors = Scene::empty(20, 10).unwrap().document;
        for y in 0..10 {
            for x in 0..20 {
                floors.floors[(y * 20 + x) as usize] =
                    Some(usize::from(x >= 10) + if y >= 5 { 2 } else { 0 });
            }
        }
        app.scene = Scene::from_document(floors).unwrap();
        app.editor.category = 0;
        app.selecting = true;
        app.message = "Sols répétés sur de grandes surfaces. G permet d'afficher la grille uniquement pour composer.".into();
        capture(
            &directory.join("floor-surfaces.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.grid = true;
        capture(
            &directory.join("floor-surfaces-grid.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.grid = false;
        app.scene = Scene::empty(16, 10).unwrap();
        app.editor.brush = Brush::Wall;
        app.editor.category = 1;
        app.editor.rotation = 0;
        app.editor.fixed_wall_connections = Some(10);
        for x in [2, 5, 8, 11] {
            app.editor
                .place(&mut app.scene, GridPos::new(x, 2))
                .unwrap();
            app.editor.rotate_brush();
        }
        app.editor.fixed_wall_connections = Some(9);
        app.editor.brush = Brush::Corner(9);
        for x in [2, 5, 8, 11] {
            app.editor
                .place(&mut app.scene, GridPos::new(x, 5))
                .unwrap();
            app.editor.rotate_brush();
        }
        app.camera.cell = 64.0;
        app.camera.center = vec2(8.0, 5.0);
        app.message = "De gauche à droite : 0°, 90°, 180°, 270°. R garde l'orientation choisie ; A rétablit les raccords automatiques.".into();
        capture(
            &directory.join("wall-rotations.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.editor.path = directory.join("wall-rotations.json");
        app.editor.save(&app.scene).unwrap();
        app.editor.fixed_wall_connections = None;
        app.editor.rotation = 0;
        // Floors deliberately extend underneath every perimeter wall. The
        // visible margins must show the exterior on the outside of each room.
        let mut rooms = Scene::empty(16, 10).unwrap().document;
        rooms.spawn = GridPos::new(8, 0);
        for (left, right, floor) in [(1, 6, 0), (9, 14, 2)] {
            for y in 1..=8 {
                for x in left..=right {
                    rooms.floors[(y * 16 + x) as usize] = Some(floor);
                    if x == left || x == right || y == 1 || y == 8 {
                        rooms.structures.push(scene::Structure {
                            pos: GridPos::new(x, y),
                            door: if x == right && y == 4 {
                                Some(DoorState::Closed)
                            } else {
                                None
                            },
                            rotation: if x == right && y == 4 { 1 } else { 0 },
                            style: 0,
                            fixed_connections: None,
                        });
                    }
                }
            }
        }
        app.scene = Scene::from_document(rooms.clone()).unwrap();
        app.editor.category = 1;
        app.camera.cell = 64.0;
        app.camera.center = vec2(8.0, 5.0);
        app.message = "Angles dans les quatre sens ; le sol intérieur s'arrête au pied des murs. Les données peintes sous les murs sont conservées.".into();
        capture(
            &directory.join("wall-corners.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        let mut explicit = rooms.clone();
        for part in &mut explicit.structures {
            if part.door.is_some() {
                continue;
            }
            let left = if part.pos.x < 8 { 1 } else { 9 };
            let right = left + 5;
            let mask = match (part.pos.x, part.pos.y) {
                (x, 1) if x == left => Some(6),
                (x, 1) if x == right => Some(12),
                (x, 8) if x == left => Some(3),
                (x, 8) if x == right => Some(9),
                _ => None,
            };
            if let Some(mask) = mask {
                part.fixed_connections = Some(mask);
                part.rotation = 0;
            } else {
                part.fixed_connections = Some(10);
                part.rotation = if part.pos.x == left {
                    3
                } else if part.pos.x == right {
                    1
                } else if part.pos.y == 8 {
                    2
                } else {
                    0
                };
            }
        }
        app.scene = Scene::from_document(explicit).unwrap();
        app.editor.brush = Brush::Corner(3);
        app.editor.fixed_wall_connections = Some(3);
        app.message = "Quatre pièces de coin choisies dans la palette. Les murs droits sont orientés manuellement ; chaque angle est une image dédiée.".into();
        capture(
            &directory.join("four-corners-manual.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.editor.path = directory.join("four-corners-manual.json");
        app.editor.save(&app.scene).unwrap();

        let mut closeup = Scene::empty(8, 5).unwrap().document;
        closeup.spawn = GridPos::new(3, 0);
        for (left, floor) in [(0, 0), (5, 2)] {
            for y in 1..=3 {
                for x in left..=left + 2 {
                    closeup.floors[(y * 8 + x) as usize] = Some(floor);
                    if x == left || x == left + 2 || y == 1 || y == 3 {
                        closeup.structures.push(scene::Structure {
                            pos: GridPos::new(x, y),
                            door: None,
                            rotation: 0,
                            style: 0,
                            fixed_connections: None,
                        });
                    }
                }
            }
        }
        app.scene = Scene::from_document(closeup).unwrap();
        app.camera.cell = 128.0;
        app.camera.center = vec2(4.0, 2.5);
        app.message =
            "Bordures des murs et des angles : grossissement ×2 des tuiles natives de 64 pixels."
                .into();
        capture(
            &directory.join("wall-borders-closeup.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.camera.cell = 64.0;
        app.camera.center = vec2(8.0, 5.0);

        for part in &mut rooms.structures {
            if part.door.is_some() {
                part.door = Some(DoorState::Open);
            }
        }
        app.scene = Scene::from_document(rooms).unwrap();
        capture(
            &directory.join("wall-corners-open-doors.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.scene = Scene::new();
        app.reset_camera();
        app.editor.category = 2;
        app.editor.brush = Brush::Object(5, true);
        app.editor.rotation = 1;
        app.editor
            .place(&mut app.scene, GridPos::new(4, 3))
            .unwrap();
        app.selecting = true;
        app.selection = Some(GridPos::new(4, 3));
        app.message = "Lit placé sur le sol, tourné à 90°. Sélectionner permet de tourner, déplacer ou supprimer cet objet.".into();
        capture(
            &directory.join("editor-1360.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.camera.cell = 64.0;
        app.camera.center = vec2(6.0, 4.5);
        capture(
            &directory.join("editor-native64.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.new_dialog();
        capture(
            &directory.join("new-map.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.modal = None;
        app.editor.new_map(&mut app.scene, 37, 19).unwrap();
        app.reset_camera();
        app.editor.brush = Brush::Floor(Some(2));
        app.editor
            .place(&mut app.scene, GridPos::new(30, 17))
            .unwrap();
        app.editor.brush = Brush::Object(5, true);
        app.editor.rotation = 3;
        app.editor
            .place(&mut app.scene, GridPos::new(30, 17))
            .unwrap();
        app.editor.path = directory.join("custom-37x19.json");
        app.editor.save(&app.scene).unwrap();
        app.scene = Scene::new();
        app.editor.load(&mut app.scene).unwrap();
        app.camera.cell = 32.0;
        app.reset_camera();
        app.message =
            "Carte 37 × 19 créée, sauvegardée et rechargée avec son objet tourné à 270°.".into();
        capture(
            &directory.join("custom-map.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.editor.restore_demo(&mut app.scene).unwrap();
        app.reset_camera();
        app.camera.cell = 48.0;
        app.selection = None;
        app.editor.rotation = 0;
        app.message = "Palette et composition en 960 × 540. La caméra et le zoom permettent de parcourir toute la carte.".into();
        request_new_screen_size(960.0, 540.0);
        next_frame().await;
        capture(
            &directory.join("editor-960.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        app.toggle_test();
        app.camera.cell = 64.0;
        app.scene.move_player(Direction::East);
        app.scene.move_player(Direction::East);
        app.scene.interaction();
        app.camera.center = vec2(8.0, 4.0);
        capture(
            &directory.join("play-test.png"),
            &app,
            &assets,
            &background,
            &font,
        )
        .await;
        return;
    }
    loop {
        app.input();
        workbench::autosave(&mut app);
        if app.quit {
            if app.dirty() {
                workbench::force_recovery(&mut app);
            }
            let _ = storage::save_preferences(&app.editor);
            break;
        }
        render(&app, &assets, &background, &font);
        next_frame().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picking_matches_rendered_cells_at_both_sizes_and_viewports() {
        for cell in [32.0, 48.0, 64.0] {
            for bounds in [
                Rect::new(304.0, 88.0, 1032.0, 656.0),
                Rect::new(304.0, 88.0, 632.0, 356.0),
            ] {
                let camera = Camera {
                    center: vec2(10.0, 5.0),
                    cell,
                };
                let origin = camera.origin(37, 19, bounds);
                for (x, y) in [(10, 5), (12, 7)] {
                    let point = origin + vec2(x as f32 + 0.5, y as f32 + 0.5) * cell;
                    if bounds.contains(point) {
                        assert_eq!(
                            camera.cell_at(point, 37, 19, bounds),
                            Some(GridPos::new(x, y))
                        );
                    }
                }
                assert_eq!(camera.cell_at(vec2(10.0, 10.0), 37, 19, bounds), None);
            }
        }
    }
}
