//! First native editor, intentionally limited to the review map and asset lot.
use super::scene::{Document, HEIGHT, Prop, Scene, Structure, WIDTH};
use macroquad::prelude::*;
use project_rl::world::{DoorState, GridPos};
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brush {
    Floor(Option<usize>),
    Wall,
    Corner(u8),
    Door(bool),
    Object(usize, bool),
    Spawn,
}

pub struct Editor {
    pub active: bool,
    pub brush: Brush,
    pub rotation: usize,
    pub fixed_wall_connections: Option<u8>,
    pub blocking: bool,
    pub category: usize,
    pub scroll: usize,
    pub center: Vec2,
    pub path: PathBuf,
    undo: Vec<Document>,
    redo: Vec<Document>,
    stroke: Option<Document>,
}

const FLOORS: [&str; 10] = [
    "Métal A",
    "Métal B",
    "Béton A",
    "Béton B",
    "Grille",
    "Gravier",
    "Terre",
    "Herbe sèche",
    "Eau peu profonde",
    "Eau profonde",
];
const OBJECTS: [&str; 10] = [
    "Établi et outils",
    "Chaise",
    "Armoire",
    "Étagère garnie",
    "Table et tasse",
    "Lit",
    "Banc",
    "Jardinière",
    "Caisse",
    "Terminal",
];

impl Editor {
    pub fn new(path: PathBuf) -> Self {
        Self {
            active: false,
            brush: Brush::Object(0, true),
            rotation: 0,
            fixed_wall_connections: None,
            blocking: true,
            category: 2,
            scroll: 0,
            center: vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
            path,
            undo: vec![],
            redo: vec![],
            stroke: None,
        }
    }

    pub fn entries(&self) -> Vec<(&'static str, Brush)> {
        match self.category {
            0 => FLOORS
                .iter()
                .enumerate()
                .map(|(i, &name)| (name, Brush::Floor(if i < 10 { Some(i) } else { None })))
                .collect(),
            1 => vec![
                ("Mur et raccords", Brush::Wall),
                ("Coin haut gauche", Brush::Corner(6)),
                ("Coin haut droit", Brush::Corner(12)),
                ("Coin bas gauche", Brush::Corner(3)),
                ("Coin bas droit", Brush::Corner(9)),
                ("Porte fermée", Brush::Door(false)),
                ("Porte ouverte", Brush::Door(true)),
                ("Départ du joueur", Brush::Spawn),
            ],
            _ => OBJECTS
                .iter()
                .enumerate()
                .map(|(i, &name)| (name, Brush::Object(if i < 8 { i } else { i + 6 }, i < 8)))
                .collect(),
        }
    }

    pub fn choose_brush(&mut self, brush: Brush) {
        self.brush = brush;
        match brush {
            Brush::Wall => {
                self.fixed_wall_connections = None;
                self.rotation = 0;
            }
            Brush::Corner(mask) => {
                self.fixed_wall_connections = Some(mask);
                self.rotation = 0;
            }
            _ => {}
        }
    }

    fn commit(&mut self, scene: &mut Scene, document: Document) -> Result<bool, String> {
        if document == scene.document {
            return Ok(false);
        }
        let replacement = Scene::from_document(document)?;
        if self.stroke.is_none() {
            self.undo.push(scene.document.clone());
            if self.undo.len() > 64 {
                self.undo.remove(0);
            }
            self.redo.clear();
        }
        *scene = replacement;
        Ok(true)
    }

    pub fn begin_stroke(&mut self, scene: &Scene) {
        if self.stroke.is_none() {
            self.stroke = Some(scene.document.clone());
        }
    }

    pub fn end_stroke(&mut self, scene: &Scene) {
        if let Some(document) = self.stroke.take() {
            if document != scene.document {
                self.undo.push(document);
                if self.undo.len() > 64 {
                    self.undo.remove(0);
                }
                self.redo.clear();
            }
        }
    }

    pub fn place(&mut self, scene: &mut Scene, pos: GridPos) -> Result<bool, String> {
        if !scene.game.map().contains(pos) {
            return Ok(false);
        }
        let mut document = scene.document.clone();
        match self.brush {
            Brush::Floor(index) => {
                document.floors[(pos.y * document.width + pos.x) as usize] = index
            }
            Brush::Spawn => document.spawn = pos,
            brush => {
                if let Brush::Object(sprite, furniture) = brush {
                    let wanted = Prop {
                        pos,
                        sprite,
                        furniture,
                        rotation: self.rotation,
                        blocking: self.blocking,
                    };
                    if document.props.iter().any(|prop| *prop == wanted) {
                        return Ok(false);
                    }
                }
                if let Brush::Wall = brush {
                    if document.structures.iter().any(|part| {
                        part.pos == pos
                            && part.door.is_none()
                            && part.rotation == self.rotation
                            && part.fixed_connections == self.fixed_wall_connections
                    }) {
                        return Ok(false);
                    }
                }
                document.structures.retain(|part| part.pos != pos);
                document.props.retain(|prop| prop.pos != pos);
                match brush {
                    Brush::Wall | Brush::Corner(_) => document.structures.push(Structure {
                        pos,
                        door: None,
                        rotation: if matches!(brush, Brush::Corner(_)) {
                            0
                        } else {
                            self.rotation
                        },
                        fixed_connections: if let Brush::Corner(mask) = brush {
                            Some(mask)
                        } else {
                            self.fixed_wall_connections
                        },
                    }),
                    Brush::Door(open) => document.structures.push(Structure {
                        pos,
                        door: Some(if open {
                            DoorState::Open
                        } else {
                            DoorState::Closed
                        }),
                        rotation: self.rotation,
                        fixed_connections: None,
                    }),
                    Brush::Object(sprite, furniture) => document.props.push(Prop {
                        pos,
                        sprite,
                        furniture,
                        rotation: self.rotation,
                        blocking: self.blocking,
                    }),
                    _ => unreachable!(),
                }
            }
        }
        self.commit(scene, document)
    }

    pub fn erase(&mut self, scene: &mut Scene, pos: GridPos) -> Result<bool, String> {
        let mut document = scene.document.clone();
        document.structures.retain(|part| part.pos != pos);
        document.props.retain(|prop| prop.pos != pos);
        self.commit(scene, document)
    }

    pub fn undo(&mut self, scene: &mut Scene) -> Result<bool, String> {
        let Some(document) = self.undo.last().cloned() else {
            return Ok(false);
        };
        let replacement = Scene::from_document(document)?;
        self.undo.pop();
        self.redo.push(scene.document.clone());
        *scene = replacement;
        Ok(true)
    }

    pub fn redo(&mut self, scene: &mut Scene) -> Result<bool, String> {
        let Some(document) = self.redo.last().cloned() else {
            return Ok(false);
        };
        let replacement = Scene::from_document(document)?;
        self.redo.pop();
        self.undo.push(scene.document.clone());
        *scene = replacement;
        Ok(true)
    }

    pub fn save(&self, scene: &Scene) -> Result<(), String> {
        let bytes = serde_json::to_vec_pretty(&scene.document).map_err(|e| e.to_string())?;
        if let Some(parent) = self
            .path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let stamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_nanos();
        let temporary = self
            .path
            .with_extension(format!("{}.{}.tmp", std::process::id(), stamp));
        std::fs::write(&temporary, bytes).map_err(|e| e.to_string())?;
        if let Err(error) = std::fs::rename(&temporary, &self.path) {
            let _ = std::fs::remove_file(&temporary);
            return Err(error.to_string());
        }
        Ok(())
    }

    pub fn load(&mut self, scene: &mut Scene) -> Result<bool, String> {
        let bytes = std::fs::read(&self.path).map_err(|e| e.to_string())?;
        let document: Document = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        self.commit(scene, document)
    }

    pub fn pick(&mut self, scene: &Scene, pos: GridPos) {
        if let Some(prop) = scene.props.iter().find(|prop| prop.pos == pos) {
            self.brush = Brush::Object(prop.sprite, prop.furniture);
            self.rotation = prop.rotation;
            self.blocking = prop.blocking;
        } else if let Some(part) = scene
            .document
            .structures
            .iter()
            .find(|part| part.pos == pos)
        {
            self.brush = part
                .door
                .map(|door| Brush::Door(door == DoorState::Open))
                .unwrap_or(Brush::Wall);
            self.rotation = part.rotation;
            self.fixed_wall_connections = part.fixed_connections;
            if part.door.is_none() && part.fixed_connections.is_some() {
                let (mask, turns) = scene.wall_sprite(part);
                if super::scene::CORNER_MASKS.contains(&(mask as u8)) {
                    self.brush = Brush::Corner(mask as u8);
                    self.fixed_wall_connections = Some(mask as u8);
                    self.rotation = turns;
                }
            }
        } else if scene.game.map().contains(pos) {
            self.brush = Brush::Floor(
                scene.document.floors[(pos.y * scene.document.width + pos.x) as usize],
            );
        }
    }

    pub fn new_map(&mut self, scene: &mut Scene, width: i32, height: i32) -> Result<bool, String> {
        let document = Scene::empty(width, height)?.document;
        let changed = self.commit(scene, document)?;
        self.center = vec2(width as f32 / 2.0, height as f32 / 2.0);
        Ok(changed)
    }

    pub fn restore_demo(&mut self, scene: &mut Scene) -> Result<bool, String> {
        self.commit(scene, Scene::new().document)
    }

    pub fn rotate_at(&mut self, scene: &mut Scene, pos: GridPos) -> Result<bool, String> {
        let wall_sprite = scene
            .document
            .structures
            .iter()
            .find(|part| part.pos == pos && part.door.is_none())
            .map(|part| scene.wall_sprite(part));
        let mut document = scene.document.clone();
        if let Some(prop) = document.props.iter_mut().find(|prop| prop.pos == pos) {
            prop.rotation = (prop.rotation + 1) % 4;
            self.rotation = prop.rotation;
        } else if let Some(part) = document.structures.iter_mut().find(|part| part.pos == pos) {
            if let Some((mask, _)) = wall_sprite {
                if super::scene::CORNER_MASKS.contains(&(mask as u8)) {
                    part.fixed_connections = Some(super::scene::rotate_connections(mask as u8, 1));
                    part.rotation = 0;
                } else if part.fixed_connections.is_some() {
                    part.rotation = (part.rotation + 1) % 4;
                } else if scene.neighbors(pos) == 0 {
                    part.fixed_connections = Some(10);
                    part.rotation = (part.rotation + 1) % 4;
                } else {
                    part.fixed_connections = Some(mask as u8);
                    part.rotation = (scene.straight_facing(pos, mask as u8) + 1) % 4;
                }
            } else {
                part.rotation = (part.rotation + 1) % 4;
            }
            self.rotation = part.rotation;
            self.fixed_wall_connections = part.fixed_connections;
            if part.door.is_none() {
                self.brush = if let Some(mask) = part
                    .fixed_connections
                    .filter(|mask| super::scene::CORNER_MASKS.contains(mask))
                {
                    Brush::Corner(mask)
                } else {
                    Brush::Wall
                };
            }
        } else {
            return Ok(false);
        }
        self.commit(scene, document)
    }

    pub fn rotate_brush(&mut self) {
        if let Brush::Corner(mask) = self.brush {
            let next = super::scene::rotate_connections(mask, 1);
            self.brush = Brush::Corner(next);
            self.fixed_wall_connections = Some(next);
            self.rotation = 0;
            return;
        }
        if self.brush == Brush::Wall {
            self.fixed_wall_connections.get_or_insert(10);
        }
        self.rotation = (self.rotation + 1) % 4;
    }

    pub fn use_auto_wall_at(&mut self, scene: &mut Scene, pos: GridPos) -> Result<bool, String> {
        let mut document = scene.document.clone();
        let Some(part) = document
            .structures
            .iter_mut()
            .find(|part| part.pos == pos && part.door.is_none())
        else {
            return Ok(false);
        };
        part.fixed_connections = None;
        part.rotation = 0;
        let result = self.commit(scene, document)?;
        self.fixed_wall_connections = None;
        self.rotation = 0;
        self.brush = Brush::Wall;
        Ok(result)
    }

    pub fn toggle_blocking_at(&mut self, scene: &mut Scene, pos: GridPos) -> Result<bool, String> {
        let mut document = scene.document.clone();
        let Some(prop) = document.props.iter_mut().find(|prop| prop.pos == pos) else {
            return Ok(false);
        };
        prop.blocking = !prop.blocking;
        let blocking = prop.blocking;
        let result = self.commit(scene, document)?;
        self.blocking = blocking;
        Ok(result)
    }

    pub fn move_at(
        &mut self,
        scene: &mut Scene,
        from: GridPos,
        to: GridPos,
    ) -> Result<bool, String> {
        if from == to {
            return Ok(false);
        }
        let mut document = scene.document.clone();
        if let Some(prop) = document.props.iter_mut().find(|prop| prop.pos == from) {
            prop.pos = to;
        } else if let Some(part) = document.structures.iter_mut().find(|part| part.pos == from) {
            part.pos = to;
        } else {
            return Ok(false);
        }
        self.commit(scene, document)
    }

    pub fn visible_rows() -> usize {
        ((screen_height() - 130.0 - 216.0) / 52.0).floor().max(2.0) as usize
    }

    pub fn sidebar_contains(point: Vec2) -> bool {
        Rect::new(24.0, 88.0, 260.0, screen_height() - 184.0).contains(point)
    }

    pub fn ui_input(&mut self, scene: &mut Scene) -> Option<Result<String, String>> {
        let mouse: Vec2 = mouse_position().into();
        if !Self::sidebar_contains(mouse) {
            return None;
        }
        let wheel = mouse_wheel().1;
        let rows = Self::visible_rows();
        let maximum = self.entries().len().saturating_sub(rows);
        if wheel < 0.0 {
            self.scroll = (self.scroll + 1).min(maximum);
        }
        if wheel > 0.0 {
            self.scroll = self.scroll.saturating_sub(1);
        }
        if !is_mouse_button_pressed(MouseButton::Left) {
            return None;
        }
        for (i, rect) in action_buttons().into_iter().enumerate() {
            if rect.contains(mouse) {
                return Some(match i {
                    0 => self
                        .save(scene)
                        .map(|_| format!("Carte sauvegardée : {}", self.path.display())),
                    1 => self.load(scene).map(|_| "Carte chargée.".into()),
                    2 => self.undo(scene).map(|_| "Modification annulée.".into()),
                    _ => self.redo(scene).map(|_| "Modification rétablie.".into()),
                });
            }
        }
        for i in 0..3 {
            if Rect::new(32.0 + i as f32 * 82.0, 168.0, 78.0, 30.0).contains(mouse) {
                self.category = i;
                self.scroll = 0;
            }
        }
        for (i, (_, brush)) in self
            .entries()
            .into_iter()
            .skip(self.scroll)
            .take(rows)
            .enumerate()
        {
            if row_rect(i).contains(mouse) {
                self.choose_brush(brush);
            }
        }
        None
    }

    pub fn draw_sidebar(&self, assets: &super::assets::Assets, font: &Font) {
        draw_rectangle(
            24.0,
            88.0,
            260.0,
            screen_height() - 184.0,
            Color::from_rgba(34, 40, 43, 255),
        );
        super::label(font, "PALETTE", 34.0, 111.0, 19, WHITE);
        for (i, rect) in action_buttons().into_iter().enumerate() {
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                Color::from_rgba(57, 67, 69, 255),
            );
            super::label(
                font,
                ["Sauver", "Charger", "Annuler", "Rétablir"][i],
                rect.x + 5.0,
                rect.y + 20.0,
                14,
                WHITE,
            );
        }
        for i in 0..3 {
            let rect = Rect::new(32.0 + i as f32 * 82.0, 168.0, 78.0, 30.0);
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if self.category == i {
                    Color::from_rgba(72, 100, 96, 255)
                } else {
                    Color::from_rgba(45, 52, 55, 255)
                },
            );
            super::label(
                font,
                ["Sols", "Murs", "Objets"][i],
                rect.x + 12.0,
                rect.y + 22.0,
                16,
                WHITE,
            );
        }
        let entries = self.entries();
        for (i, (name, brush)) in entries
            .iter()
            .skip(self.scroll)
            .take(Self::visible_rows())
            .enumerate()
        {
            let rect = row_rect(i);
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if self.brush == *brush {
                    Color::from_rgba(65, 86, 84, 255)
                } else {
                    Color::from_rgba(42, 49, 52, 255)
                },
            );
            if let Some(texture) = self.texture(*brush, assets) {
                super::sprite_rotated(
                    texture,
                    rect.x + 4.0,
                    rect.y + 4.0,
                    38.0,
                    self.brush_turns(*brush),
                    WHITE,
                );
            }
            super::label(font, name, rect.x + 50.0, rect.y + 29.0, 16, WHITE);
        }
        let bottom = screen_height() - 119.0;
        super::label(
            font,
            &if matches!(self.brush, Brush::Corner(_)) {
                "R : choisir le coin suivant".to_owned()
            } else if self.brush == Brush::Wall {
                format!(
                    "R : {}° · mur {}",
                    self.rotation * 90,
                    if self.fixed_wall_connections.is_some() {
                        "manuel"
                    } else {
                        "auto"
                    }
                )
            } else {
                format!(
                    "R : {}° · B : {}",
                    self.rotation * 90,
                    if self.blocking { "obstacle" } else { "décor" }
                )
            },
            34.0,
            bottom,
            15,
            Color::from_rgba(176, 215, 205, 255),
        );
        super::label(
            font,
            if matches!(self.brush, Brush::Wall | Brush::Corner(_)) {
                "A : revenir aux raccords auto"
            } else {
                "Molette sur la palette : défiler"
            },
            34.0,
            bottom + 19.0,
            13,
            Color::from_rgba(164, 177, 177, 255),
        );
    }

    pub fn texture<'a>(
        &self,
        brush: Brush,
        assets: &'a super::assets::Assets,
    ) -> Option<&'a Texture2D> {
        match brush {
            Brush::Floor(Some(i)) => Some(&assets.terrain[i]),
            Brush::Wall => Some(
                &assets.connections[if self.brush == Brush::Wall {
                    super::scene::fixed_wall_sprite(
                        self.fixed_wall_connections.unwrap_or(10),
                        self.rotation,
                    )
                    .0
                } else {
                    10
                }],
            ),
            Brush::Corner(mask) => Some(&assets.connections[mask as usize]),
            Brush::Door(open) => Some(&assets.walls[if open { 3 } else { 2 }][0]),
            Brush::Object(i, furniture) => Some(if furniture {
                &assets.furniture[i]
            } else {
                &assets.terrain[i]
            }),
            _ => None,
        }
    }

    fn brush_turns(&self, brush: Brush) -> usize {
        match brush {
            Brush::Corner(_) => 0,
            Brush::Wall if self.brush == Brush::Wall => {
                super::scene::fixed_wall_sprite(
                    self.fixed_wall_connections.unwrap_or(10),
                    self.rotation,
                )
                .1
            }
            Brush::Wall => 0,
            _ => self.rotation,
        }
    }
}

fn action_buttons() -> [Rect; 4] {
    std::array::from_fn(|i| Rect::new(32.0 + i as f32 * 61.0, 122.0, 57.0, 30.0))
}

fn row_rect(index: usize) -> Rect {
    Rect::new(32.0, 216.0 + index as f32 * 52.0, 244.0, 46.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn straight_runs_follow_corner_faces_and_manual_rotation_freezes_the_choice() {
        let mut document = Scene::empty(10, 10).unwrap().document;
        for y in 1..=8 {
            for x in 1..=8 {
                if x == 1 || x == 8 || y == 1 || y == 8 {
                    document.structures.push(super::super::scene::Structure {
                        pos: GridPos::new(x, y),
                        door: None,
                        rotation: 0,
                        fixed_connections: None,
                    });
                }
            }
        }
        let mut scene = Scene::from_document(document).unwrap();
        for (pos, expected) in [
            (GridPos::new(4, 1), (10, 0)),
            (GridPos::new(4, 8), (10, 2)),
            (GridPos::new(1, 4), (5, 0)),
            (GridPos::new(8, 4), (5, 2)),
        ] {
            assert_eq!(scene.wall_preview(pos, 0, None), expected);
            let mut editor = Editor::new(std::path::PathBuf::new());
            editor.rotate_at(&mut scene, pos).unwrap();
            let part = scene
                .document
                .structures
                .iter()
                .find(|part| part.pos == pos)
                .unwrap();
            assert!(part.fixed_connections.is_some());
            let frozen = scene.wall_sprite(part);
            assert_ne!(frozen, expected);
            let mut document = scene.document.clone();
            document.structures.retain(|part| {
                !super::super::scene::CORNER_MASKS.contains(&scene.neighbors(part.pos))
            });
            let without_corners = Scene::from_document(document).unwrap();
            let part = without_corners
                .document
                .structures
                .iter()
                .find(|part| part.pos == pos)
                .unwrap();
            assert_eq!(without_corners.wall_sprite(part), frozen);
        }
    }

    #[test]
    fn four_explicit_corners_place_exactly_without_neighbors_and_cycle_dedicated_sprites() {
        let mut scene = Scene::empty(16, 6).unwrap();
        let mut editor = Editor::new(PathBuf::new());
        for (x, mask) in [2, 5, 8, 11]
            .into_iter()
            .zip(super::super::scene::CORNER_MASKS)
        {
            editor.rotation = 3; // A previous object orientation must not affect the choice.
            editor.choose_brush(Brush::Corner(mask));
            let pos = GridPos::new(x, 3);
            editor.place(&mut scene, pos).unwrap();
            let part = scene
                .document
                .structures
                .iter()
                .find(|part| part.pos == pos)
                .unwrap();
            assert_eq!(scene.wall_sprite(part), (mask as usize, 0));
            editor.rotate_at(&mut scene, pos).unwrap();
            let next = super::super::scene::rotate_connections(mask, 1);
            let part = scene
                .document
                .structures
                .iter()
                .find(|part| part.pos == pos)
                .unwrap();
            assert_eq!(scene.wall_sprite(part), (next as usize, 0));
            editor.pick(&scene, pos);
            assert_eq!(editor.brush, Brush::Corner(next));
            assert_eq!(editor.rotation, 0);
        }
        let json = serde_json::to_vec(&scene.document).unwrap();
        let restored = Scene::from_document(serde_json::from_slice(&json).unwrap()).unwrap();
        assert_eq!(scene.document, restored.document);
        editor.choose_brush(Brush::Wall);
        assert_eq!(editor.fixed_wall_connections, None);
        assert_eq!(editor.rotation, 0);
    }

    #[test]
    fn rotating_before_wall_placement_matches_preview_and_survives_new_neighbors() {
        let mut scene = Scene::empty(8, 6).unwrap();
        let mut editor = Editor::new(PathBuf::new());
        let pos = GridPos::new(3, 3);
        editor.brush = Brush::Wall;
        editor.rotate_brush();
        let preview = scene.wall_preview(pos, editor.rotation, editor.fixed_wall_connections);
        editor.place(&mut scene, pos).unwrap();
        assert_eq!(preview, (5, 2));
        assert_eq!(scene.wall_sprite(&scene.document.structures[0]), preview);
        editor.rotation = 0;
        editor.fixed_wall_connections = None;
        editor.place(&mut scene, GridPos::new(2, 3)).unwrap();
        editor.place(&mut scene, GridPos::new(4, 3)).unwrap();
        assert_eq!(scene.wall_sprite(&scene.document.structures[0]), preview);
        let restored = Scene::from_document(
            serde_json::from_slice(&serde_json::to_vec(&scene.document).unwrap()).unwrap(),
        )
        .unwrap();
        assert_eq!(
            restored.wall_sprite(&restored.document.structures[0]),
            preview
        );
    }

    #[test]
    fn placed_wall_rotates_all_four_faces_and_can_be_copied_and_undone() {
        let mut scene = Scene::empty(8, 6).unwrap();
        let mut editor = Editor::new(PathBuf::new());
        let pos = GridPos::new(3, 3);
        editor.brush = Brush::Wall;
        editor.place(&mut scene, pos).unwrap();
        for turns in [1, 2, 3, 0] {
            editor.rotate_at(&mut scene, pos).unwrap();
            assert_eq!(
                scene.wall_sprite(&scene.document.structures[0]),
                super::super::scene::fixed_wall_sprite(10, turns)
            );
            assert_eq!(scene.document.structures[0].rotation, turns);
        }
        editor.undo(&mut scene).unwrap();
        assert_eq!(scene.wall_sprite(&scene.document.structures[0]), (5, 0));
        editor.redo(&mut scene).unwrap();
        assert_eq!(scene.wall_sprite(&scene.document.structures[0]), (10, 0));
        editor.rotate_at(&mut scene, pos).unwrap();
        editor.pick(&scene, pos);
        let copy = GridPos::new(6, 3);
        editor.place(&mut scene, copy).unwrap();
        let part = scene
            .document
            .structures
            .iter()
            .find(|part| part.pos == copy)
            .unwrap();
        assert_eq!(part.fixed_connections, Some(10));
        assert_eq!(scene.wall_sprite(part), (5, 2));
    }

    #[test]
    fn rotating_an_automatic_corner_freezes_its_shape_until_auto_is_restored() {
        let mut scene = Scene::new();
        let mut editor = Editor::new(PathBuf::new());
        let pos = GridPos::new(1, 1);
        editor.rotate_at(&mut scene, pos).unwrap();
        let part = scene
            .document
            .structures
            .iter()
            .find(|part| part.pos == pos)
            .unwrap();
        assert_eq!(scene.wall_sprite(part), (12, 0));
        assert_eq!(part.fixed_connections, Some(12));
        editor.erase(&mut scene, GridPos::new(2, 1)).unwrap();
        let part = scene
            .document
            .structures
            .iter()
            .find(|part| part.pos == pos)
            .unwrap();
        assert_eq!(scene.wall_sprite(part), (12, 0));
        editor.use_auto_wall_at(&mut scene, pos).unwrap();
        let part = scene
            .document
            .structures
            .iter()
            .find(|part| part.pos == pos)
            .unwrap();
        assert_eq!(scene.wall_sprite(part), (4, 0));
        assert_eq!(part.fixed_connections, None);
        editor.undo(&mut scene).unwrap();
        let part = scene
            .document
            .structures
            .iter()
            .find(|part| part.pos == pos)
            .unwrap();
        assert_eq!(scene.wall_sprite(part), (12, 0));
    }

    #[test]
    fn old_map_files_keep_automatic_walls_and_invalid_shapes_are_rejected() {
        let scene = Scene::new();
        let old_json = serde_json::to_value(&scene.document).unwrap();
        assert!(
            old_json["structures"]
                .as_array()
                .unwrap()
                .iter()
                .all(|part| part.get("fixed_connections").is_none())
        );
        let restored = Scene::from_document(serde_json::from_value(old_json).unwrap()).unwrap();
        for part in &restored.document.structures {
            assert_eq!(part.fixed_connections, None);
            if part.door.is_none() {
                assert_eq!(
                    restored.wall_sprite(part).0,
                    restored.neighbors(part.pos) as usize
                );
            }
        }
        let mut invalid = scene.document.clone();
        invalid.structures[0].fixed_connections = Some(16);
        assert!(Scene::from_document(invalid).is_err());
    }

    #[test]
    fn rotated_object_export_round_trips_and_keeps_collision_choice() {
        let mut scene = Scene::new();
        let mut editor = Editor::new(PathBuf::new());
        editor.brush = Brush::Object(5, true);
        editor.rotation = 3;
        editor.blocking = false;
        let pos = GridPos::new(4, 3);
        editor.place(&mut scene, pos).unwrap();
        let json = serde_json::to_string(&scene.document).unwrap();
        let restored = Scene::from_document(serde_json::from_str(&json).unwrap()).unwrap();
        let prop = restored.props.iter().find(|p| p.pos == pos).unwrap();
        assert_eq!(prop.rotation, 3);
        assert!(!prop.blocking);
        assert!(restored.game.map().is_walkable(pos));
    }

    #[test]
    fn replacing_erasing_and_undoing_an_object_preserves_other_cells() {
        let mut scene = Scene::new();
        let original = scene.document.clone();
        let mut editor = Editor::new(PathBuf::new());
        editor.brush = Brush::Object(6, true);
        editor.rotation = 1;
        let pos = GridPos::new(6, 2);
        editor.place(&mut scene, pos).unwrap();
        assert_eq!(scene.props.iter().filter(|p| p.pos == pos).count(), 1);
        editor.erase(&mut scene, pos).unwrap();
        assert!(scene.game.map().is_walkable(pos));
        editor.undo(&mut scene).unwrap();
        editor.undo(&mut scene).unwrap();
        assert_eq!(scene.document, original);
        editor.redo(&mut scene).unwrap();
        assert_eq!(
            scene.props.iter().find(|p| p.pos == pos).unwrap().rotation,
            1
        );
    }

    #[test]
    fn invalid_edit_does_not_destroy_scene_or_undo_history() {
        let mut scene = Scene::new();
        let original = scene.document.clone();
        let mut editor = Editor::new(PathBuf::new());
        editor.brush = Brush::Wall;
        assert!(editor.place(&mut scene, original.spawn).is_err());
        assert_eq!(scene.document, original);
        assert!(editor.undo.is_empty());
    }

    #[test]
    fn save_load_preserves_rotations_and_rejects_invalid_documents() {
        let path = std::env::temp_dir().join(format!(
            "rl-surface-editor-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut scene = Scene::new();
        let mut editor = Editor::new(path.clone());
        editor.rotation = 2;
        editor.place(&mut scene, GridPos::new(4, 3)).unwrap();
        editor.save(&scene).unwrap();
        // Saving an existing map also replaces it successfully on Windows.
        editor.save(&scene).unwrap();
        let saved = scene.document.clone();
        editor.erase(&mut scene, GridPos::new(4, 3)).unwrap();
        editor.load(&mut scene).unwrap();
        assert_eq!(scene.document, saved);
        std::fs::write(&path, "{\"version\": 99}").unwrap();
        assert!(editor.load(&mut scene).is_err());
        assert_eq!(scene.document, saved);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn custom_size_keeps_floor_and_rotated_object_as_separate_layers() {
        let mut scene = Scene::new();
        let mut editor = Editor::new(PathBuf::new());
        editor.new_map(&mut scene, 37, 19).unwrap();
        let pos = GridPos::new(30, 17);
        editor.brush = Brush::Floor(Some(2));
        editor.place(&mut scene, pos).unwrap();
        editor.brush = Brush::Object(5, true);
        editor.place(&mut scene, pos).unwrap();
        editor.rotate_at(&mut scene, pos).unwrap();
        editor
            .move_at(&mut scene, pos, GridPos::new(31, 17))
            .unwrap();
        assert_eq!(scene.document.floors[(17 * 37 + 30) as usize], Some(2));
        assert_eq!(scene.props[0].rotation, 1);
        assert_eq!(scene.props[0].pos, GridPos::new(31, 17));
        assert!(!scene.game.map().is_walkable(GridPos::new(31, 17)));
        editor
            .toggle_blocking_at(&mut scene, GridPos::new(31, 17))
            .unwrap();
        assert!(scene.game.map().is_walkable(GridPos::new(31, 17)));
        editor.erase(&mut scene, GridPos::new(31, 17)).unwrap();
        assert_eq!(scene.document.floors[(17 * 37 + 30) as usize], Some(2));
    }

    #[test]
    fn invalid_size_and_collision_leave_current_map_intact() {
        let mut scene = Scene::new();
        let original = scene.document.clone();
        let mut editor = Editor::new(PathBuf::new());
        assert!(editor.new_map(&mut scene, i32::MAX, 5).is_err());
        assert!(editor.new_map(&mut scene, 0, 5).is_err());
        assert!(
            editor
                .move_at(&mut scene, GridPos::new(6, 2), GridPos::new(8, 2))
                .is_err()
        );
        assert_eq!(scene.document, original);
    }

    #[test]
    fn a_brush_drag_is_one_undo_operation() {
        let mut scene = Scene::new();
        let original = scene.document.clone();
        let mut editor = Editor::new(PathBuf::new());
        editor.brush = Brush::Floor(Some(2));
        editor.begin_stroke(&scene);
        for x in 10..15 {
            editor.place(&mut scene, GridPos::new(x, 3)).unwrap();
        }
        editor.end_stroke(&scene);
        editor.undo(&mut scene).unwrap();
        assert_eq!(scene.document, original);
        editor.redo(&mut scene).unwrap();
        for x in 10..15 {
            assert_eq!(scene.document.floors[(3 * WIDTH + x) as usize], Some(2));
        }
    }
}
