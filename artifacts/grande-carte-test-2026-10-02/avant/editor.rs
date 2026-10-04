//! Native composition tools and transactional document history.
use super::scene::{Document, HEIGHT, Scene, WIDTH};
use macroquad::prelude::*;
use project_rl::world::{DoorState, GridPos, Terrain};
use std::path::PathBuf;

pub use super::author::Brush;

pub struct Editor {
    pub active: bool,
    pub brush: Brush,
    pub rotation: usize,
    pub fixed_wall_connections: Option<u8>,
    pub blocking: bool,
    pub details: super::scene::PropDetails,
    pub layers: super::author::Layers,
    pub search: String,
    pub favorites: std::collections::BTreeSet<String>,
    pub recent: Vec<String>,
    pub filter: u8,
    pub category: usize,
    pub urban_review: bool,
    pub sf_review: bool,
    pub object_group: usize,
    pub wall_style: usize,
    pub scroll: usize,
    pub round: bool,
    pub diameter: u16,
    pub center: Vec2,
    pub path: PathBuf,
    undo: Vec<Document>,
    redo: Vec<Document>,
    stroke: Option<Document>,
    paint_tail: Option<usize>,
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
            details: Default::default(),
            layers: Default::default(),
            search: String::new(),
            favorites: Default::default(),
            recent: vec![],
            filter: 0,
            category: 2,
            urban_review: false,
            sf_review: false,
            object_group: 0,
            wall_style: 0,
            scroll: 0,
            round: false,
            diameter: 96,
            center: vec2(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0),
            path,
            undo: vec![],
            redo: vec![],
            stroke: None,
            paint_tail: None,
        }
    }

    pub fn entries(&self) -> Vec<(&'static str, Brush)> {
        let mut entries: Vec<_> = match self.category {
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
        };
        match self.category {
            0 => entries.extend(
                super::catalog::CITY_FLOORS
                    .iter()
                    .enumerate()
                    .map(|(i, &name)| (name, Brush::Floor(Some(16 + i)))),
            ),
            2 => {
                entries.extend(
                    super::catalog::CITY_OBJECTS
                        .iter()
                        .enumerate()
                        .map(|(i, &name)| (name, Brush::Object(8 + i, true))),
                );
                entries.extend(
                    super::catalog::SF_OBJECTS
                        .iter()
                        .enumerate()
                        .map(|(i, &name)| (name, Brush::Object(56 + i, true))),
                );
                entries.retain(|(_,brush)| matches!(*brush,Brush::Object(i,f) if super::catalog::object_in_group(i,f,self.object_group)));
            }
            _ => {}
        }
        let query = fold(&self.search);
        entries.retain(|(name, brush)| {
            fold(name).contains(&query)
                && (self.filter != 1 || self.favorites.contains(&key(*brush)))
                && (self.filter != 2 || self.recent.contains(&key(*brush)))
        });
        if self.filter == 2 {
            entries.sort_by_key(|(_, brush)| {
                self.recent
                    .iter()
                    .position(|entry| *entry == key(*brush))
                    .unwrap_or(usize::MAX)
            });
        }
        entries
    }

    pub fn choose_brush(&mut self, brush: Brush) {
        let identity = key(brush);
        self.recent.retain(|entry| *entry != identity);
        self.recent.insert(0, identity);
        self.recent.truncate(24);
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
            Brush::Object(sprite, furniture) => {
                self.blocking = super::catalog::default_blocking(sprite, furniture);
                let (width, height) = if furniture {
                    super::catalog::default_size(sprite)
                } else {
                    (1, 1)
                };
                self.details = super::scene::PropDetails {
                    width,
                    height,
                    opaque: Some(false),
                    ..Default::default()
                };
            }
            _ => {}
        }
    }

    pub fn commit(&mut self, scene: &mut Scene, document: Document) -> Result<bool, String> {
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

    pub fn in_stroke(&self) -> bool {
        self.stroke.is_some()
    }

    pub fn begin_stroke(&mut self, scene: &Scene) {
        if self.stroke.is_none() {
            self.stroke = Some(scene.document.clone());
            self.paint_tail = None;
        }
    }

    pub fn end_stroke(&mut self, scene: &Scene) {
        self.paint_tail = None;
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

    pub fn round_ground(&self) -> bool {
        self.round && matches!(self.brush, Brush::Floor(Some(i)) if super::catalog::is_floor(i))
    }

    pub fn break_paint_path(&mut self) {
        self.paint_tail = None;
    }

    pub fn resize_brush(&mut self, grow: bool) {
        self.diameter = if grow {
            self.diameter
                .saturating_add(16)
                .min(super::paint::MAX_DIAMETER)
        } else {
            self.diameter
                .saturating_sub(16)
                .max(super::paint::MIN_DIAMETER)
        };
    }

    pub fn paint_to(
        &mut self,
        scene: &mut Scene,
        point: super::paint::Point,
        erase: bool,
    ) -> Result<bool, String> {
        if !self.layers.editable(0) {
            return Err("Le calque des sols est masqué ou verrouillé.".into());
        }
        if !self.round_ground() {
            return Ok(false);
        }
        let material = if erase {
            None
        } else if let Brush::Floor(index) = self.brush {
            index
        } else {
            unreachable!()
        };
        let mut probe = super::paint::Stroke {
            clip: None,
            material,
            diameter: self.diameter,
            points: vec![point],
        };
        super::paint::validate(
            std::slice::from_ref(&probe),
            scene.document.width,
            scene.document.height,
        )?;
        if scene
            .document
            .paint
            .iter()
            .map(|stroke| stroke.points.len())
            .sum::<usize>()
            >= 1_000_000
        {
            return Err("La limite de peinture de cette carte est atteinte.".into());
        }
        self.begin_stroke(scene);
        if let Some(index) = self.paint_tail {
            let tail = &mut scene.document.paint[index];
            if tail.material == material && tail.diameter == self.diameter {
                if tail.points.last() == Some(&point) {
                    return Ok(false);
                }
                tail.points.append(&mut probe.points);
                if scene.document.ground_terrain(scene.document.spawn) == Terrain::DeepWater {
                    scene.document.paint[index].points.pop();
                    return Err("Le départ du joueur doit rester accessible. Déplace-le avant de peindre de l'eau profonde ici.".into());
                }
                return Ok(true);
            }
        }
        if scene.document.paint.len() >= 16_384 {
            return Err("La limite de peinture de cette carte est atteinte.".into());
        }
        scene.document.paint.push(probe);
        if scene.document.ground_terrain(scene.document.spawn) == Terrain::DeepWater {
            scene.document.paint.pop();
            return Err("Le départ du joueur doit rester accessible. Déplace-le avant de peindre de l'eau profonde ici.".into());
        }
        self.paint_tail = Some(scene.document.paint.len() - 1);
        Ok(true)
    }

    pub fn place(&mut self, scene: &mut Scene, pos: GridPos) -> Result<bool, String> {
        if !scene.game.map().contains(pos) {
            return Ok(false);
        }
        let mut document = scene.document.clone();
        super::author::place(
            &mut document,
            &[pos],
            &super::author::Placement {
                brush: self.brush,
                rotation: self.rotation,
                style: self.wall_style,
                fixed: self.fixed_wall_connections,
                blocking: self.blocking,
                details: &self.details,
                layers: &self.layers,
            },
        )?;
        self.commit(scene, document)
    }

    pub fn erase(&mut self, scene: &mut Scene, pos: GridPos) -> Result<bool, String> {
        let mut document = scene.document.clone();
        if let Some(index) = document.props.iter().rposition(|prop| {
            prop.contains(pos) && self.layers.editable(super::author::prop_layer(prop))
        }) {
            document.props.remove(index);
        } else {
            document
                .structures
                .retain(|part| part.pos != pos || !self.layers.editable(1));
        }
        self.commit(scene, document)
    }

    pub fn undo(&mut self, scene: &mut Scene) -> Result<bool, String> {
        self.end_stroke(scene);
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
        self.end_stroke(scene);
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
        super::storage::save_map(&self.path, &scene.document)
    }

    pub fn load(&mut self, scene: &mut Scene) -> Result<bool, String> {
        self.end_stroke(scene);
        let bytes = std::fs::read(&self.path).map_err(|e| e.to_string())?;
        let document: Document = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        self.commit(scene, document)
    }

    pub fn pick(&mut self, scene: &Scene, pos: GridPos) {
        if let Some(prop) = scene.props.iter().rev().find(|prop| prop.contains(pos)) {
            self.brush = Brush::Object(prop.sprite, prop.furniture);
            self.rotation = prop.rotation;
            self.blocking = prop.blocking;
            self.details = prop.details.clone();
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
            self.wall_style = part.style;
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
        self.end_stroke(scene);
        let changed = self.commit(scene, document)?;
        self.center = vec2(width as f32 / 2.0, height as f32 / 2.0);
        Ok(changed)
    }

    pub fn resize_map(
        &mut self,
        scene: &mut Scene,
        width: i32,
        height: i32,
        anchor: usize,
    ) -> Result<bool, String> {
        self.end_stroke(scene);
        let document = scene.document.resized(width, height, anchor)?;
        self.commit(scene, document)
    }

    pub fn restore_demo(&mut self, scene: &mut Scene) -> Result<bool, String> {
        self.end_stroke(scene);
        let document = if self.sf_review {
            super::sf::demo().document
        } else if self.urban_review {
            super::urban::demo().document
        } else {
            Scene::new().document
        };
        self.commit(scene, document)
    }

    pub fn rotate_at(&mut self, scene: &mut Scene, pos: GridPos) -> Result<bool, String> {
        let layer = scene
            .document
            .props
            .iter()
            .rev()
            .find(|p| p.contains(pos))
            .map(super::author::prop_layer)
            .unwrap_or(1);
        if !self.layers.editable(layer) {
            return Err("Ce calque est masqué ou verrouillé.".into());
        }

        let wall_sprite = scene
            .document
            .structures
            .iter()
            .find(|part| part.pos == pos && part.door.is_none())
            .map(|part| scene.wall_sprite(part));
        let mut document = scene.document.clone();
        if let Some(prop) = document
            .props
            .iter_mut()
            .rev()
            .find(|prop| prop.contains(pos))
        {
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
            self.wall_style = part.style;
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
        let layer = scene
            .document
            .props
            .iter()
            .rev()
            .find(|p| p.contains(pos))
            .map(super::author::prop_layer)
            .unwrap_or(1);
        if !self.layers.editable(layer) {
            return Err("Ce calque est masqué ou verrouillé.".into());
        }

        let mut document = scene.document.clone();
        let Some(part) = document
            .structures
            .iter_mut()
            .find(|part| part.pos == pos && part.door.is_none())
        else {
            return Ok(false);
        };
        part.fixed_connections = None;
        self.wall_style = part.style;
        part.rotation = 0;
        let result = self.commit(scene, document)?;
        self.fixed_wall_connections = None;
        self.rotation = 0;
        self.brush = Brush::Wall;
        Ok(result)
    }

    pub fn toggle_blocking_at(&mut self, scene: &mut Scene, pos: GridPos) -> Result<bool, String> {
        let layer = scene
            .document
            .props
            .iter()
            .rev()
            .find(|p| p.contains(pos))
            .map(super::author::prop_layer)
            .unwrap_or(1);
        if !self.layers.editable(layer) {
            return Err("Ce calque est masqué ou verrouillé.".into());
        }

        let mut document = scene.document.clone();
        let Some(prop) = document
            .props
            .iter_mut()
            .rev()
            .find(|prop| prop.contains(pos))
        else {
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
        let layer = scene
            .document
            .props
            .iter()
            .rev()
            .find(|p| p.contains(from))
            .map(super::author::prop_layer)
            .unwrap_or(1);
        if !self.layers.editable(layer) {
            return Err("Ce calque est masqué ou verrouillé.".into());
        }

        if from == to {
            return Ok(false);
        }
        let mut document = scene.document.clone();
        if let Some(prop) = document
            .props
            .iter_mut()
            .rev()
            .find(|prop| prop.contains(from))
        {
            prop.pos = to;
        } else if let Some(part) = document.structures.iter_mut().find(|part| part.pos == from) {
            part.pos = to;
        } else {
            return Ok(false);
        }
        self.commit(scene, document)
    }

    pub fn visible_rows(&self) -> usize {
        ((screen_height() - 130.0 - self.row_top()) / 52.0)
            .floor()
            .max(1.0) as usize
    }

    fn row_top(&self) -> f32 {
        if self.category == 0 {
            290.0
        } else if self.category == 1 || self.category == 2 {
            288.0
        } else {
            216.0
        }
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
        let rows = self.visible_rows();
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
        if self.category == 0 {
            for (i, rect) in paint_buttons().into_iter().enumerate() {
                if rect.contains(mouse) {
                    match i {
                        0 => self.round = false,
                        1 => self.round = true,
                        2 => self.resize_brush(false),
                        _ => self.resize_brush(true),
                    }
                }
            }
        }
        if self.category == 1 {
            for i in 0..super::catalog::WALL_STYLE_COUNT {
                if palette_filter_rect(i).contains(mouse) {
                    self.wall_style = i;
                }
            }
        }
        if self.category == 2 {
            for i in 0..super::catalog::OBJECT_GROUPS.len() {
                if palette_filter_rect(i).contains(mouse) {
                    self.object_group = i;
                    self.scroll = 0;
                }
            }
        }
        for (i, (_, brush)) in self
            .entries()
            .into_iter()
            .skip(self.scroll)
            .take(rows)
            .enumerate()
        {
            if row_rect(i, self.row_top()).contains(mouse) {
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
        if self.category == 1 || self.category == 2 {
            let (choices, selected) = if self.category == 1 {
                (&super::catalog::WALL_STYLES[..], self.wall_style)
            } else {
                (&super::catalog::OBJECT_GROUPS[..], self.object_group)
            };
            for (i, name) in choices.iter().enumerate() {
                let rect = palette_filter_rect(i);
                draw_rectangle(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    if selected == i {
                        Color::from_rgba(72, 100, 96, 255)
                    } else {
                        Color::from_rgba(45, 52, 55, 255)
                    },
                );
                let width = measure_text(name, Some(font), 14, 1.0).width;
                super::label(
                    font,
                    name,
                    rect.x + (rect.w - width) / 2.0,
                    rect.y + 21.0,
                    14,
                    WHITE,
                );
            }
        }
        let entries = self.entries();
        if self.category == 0 {
            for (i, rect) in paint_buttons().into_iter().enumerate() {
                draw_rectangle(
                    rect.x,
                    rect.y,
                    rect.w,
                    rect.h,
                    if (i == 0 && !self.round) || (i == 1 && self.round) {
                        Color::from_rgba(72, 100, 96, 255)
                    } else {
                        Color::from_rgba(45, 52, 55, 255)
                    },
                );
                super::label(
                    font,
                    ["Case", "Rond", "−", "+"][i],
                    rect.x + 10.0,
                    rect.y + 22.0,
                    16,
                    WHITE,
                );
            }
            super::label(
                font,
                &format!("Largeur : {:.2} cases", self.diameter as f32 / 64.0),
                78.0,
                270.0,
                15,
                WHITE,
            );
        }
        for (i, (name, brush)) in entries
            .iter()
            .skip(self.scroll)
            .take(self.visible_rows())
            .enumerate()
        {
            let rect = row_rect(i, self.row_top());
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
                    self.preview_turns(*brush),
                    WHITE,
                );
            }
            let size = if measure_text(name, Some(font), 16, 1.0).width > rect.w - 54.0 {
                14
            } else {
                16
            };
            super::label(font, name, rect.x + 50.0, rect.y + 29.0, size, WHITE);
        }
        let bottom = screen_height() - 119.0;
        super::label(
            font,
            &if self.round_ground() {
                "Clic : peindre · droit : gommer".to_owned()
            } else if matches!(self.brush, Brush::Floor(_)) {
                "Placement du sol par case".to_owned()
            } else if matches!(self.brush, Brush::Corner(_)) {
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
            if self.round_ground() {
                "Maj + molette : taille du pinceau"
            } else if matches!(self.brush, Brush::Wall | Brush::Corner(_)) {
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
            Brush::Wall => Some(assets.connection(
                self.wall_style,
                if self.brush == Brush::Wall {
                    super::scene::fixed_wall_sprite(
                        self.fixed_wall_connections.unwrap_or(10),
                        self.rotation,
                    )
                    .0
                } else {
                    10
                },
            )),
            Brush::Corner(mask) => Some(assets.connection(self.wall_style, mask as usize)),
            Brush::Door(open) => Some(&assets.walls[if open { 3 } else { 2 }][0]),
            Brush::Object(i, furniture) => Some(if furniture {
                assets.object_texture(i, self.rotation).0
            } else {
                &assets.terrain[i]
            }),
            _ => None,
        }
    }

    pub fn preview_turns(&self, brush: Brush) -> usize {
        match brush {
            Brush::Object(i, true) if super::catalog::has_directions(i) => 0,
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

fn palette_filter_rect(index: usize) -> Rect {
    Rect::new(
        32.0 + (index % 3) as f32 * 82.0,
        208.0 + (index / 3) as f32 * 36.0,
        78.0,
        30.0,
    )
}

fn row_rect(index: usize, top: f32) -> Rect {
    Rect::new(32.0, top + index as f32 * 52.0, 244.0, 46.0)
}

fn paint_buttons() -> [Rect; 4] {
    [
        Rect::new(32.0, 208.0, 118.0, 30.0),
        Rect::new(158.0, 208.0, 118.0, 30.0),
        Rect::new(32.0, 246.0, 34.0, 30.0),
        Rect::new(242.0, 246.0, 34.0, 30.0),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workbench_group_move_preserves_locked_decor_partial_markers_and_atomic_history() {
        use super::super::{
            author::{Region, Stamp},
            scene::{Marker, MarkerKind, Prop, PropDetails},
        };
        let mut doc = Scene::empty(12, 10).unwrap().document;
        doc.props.push(Prop {
            pos: GridPos::new(2, 2),
            sprite: 72,
            furniture: true,
            rotation: 0,
            blocking: true,
            details: PropDetails {
                width: 1,
                height: 2,
                opaque: Some(false),
                ..Default::default()
            },
        });
        doc.props.push(Prop {
            pos: GridPos::new(3, 1),
            sprite: 103,
            furniture: true,
            rotation: 0,
            blocking: false,
            details: PropDetails {
                decoration: true,
                ..Default::default()
            },
        });
        doc.markers.push(Marker {
            pos: GridPos::new(1, 1),
            width: 5,
            height: 3,
            kind: MarkerKind::Enemies,
            name: "Partiellement sélectionné".into(),
            target: String::new(),
            count: 1,
        });
        doc.paint.push(super::super::paint::Stroke {
            clip: None,
            material: Some(5),
            diameter: 96,
            points: vec![
                super::super::paint::Point::new(20, 100),
                super::super::paint::Point::new(300, 220),
            ],
        });
        let original = doc.clone();
        let mut scene = Scene::from_document(doc).unwrap();
        let mut editor = Editor::new(PathBuf::new());
        editor.layers.locked[3] = true;
        let r = Region {
            x: 1,
            y: 1,
            w: 4,
            h: 4,
        };
        let stamp = Stamp::take(&original, r, &editor.layers);
        assert_eq!(stamp.props.len(), 1);
        assert!(stamp.markers.is_empty());
        let mut moved = original.clone();
        Stamp::remove(&mut moved, r, &editor.layers);
        assert_eq!(moved.markers, original.markers);
        stamp
            .paste(&mut moved, GridPos::new(6, 1), &editor.layers, true)
            .unwrap();
        editor.commit(&mut scene, moved.clone()).unwrap();
        assert!(
            scene
                .document
                .props
                .iter()
                .any(|p| p.sprite == 72 && p.pos == GridPos::new(7, 2))
        );
        assert!(
            scene
                .document
                .props
                .iter()
                .any(|p| p.sprite == 103 && p.pos == GridPos::new(3, 1))
        );
        editor.undo(&mut scene).unwrap();
        assert_eq!(scene.document, original);
        editor.redo(&mut scene).unwrap();
        assert_eq!(scene.document, moved);
    }

    #[test]
    fn workbench_multicell_rotation_and_locked_edits_are_atomic() {
        let mut scene = Scene::empty(8, 8).unwrap();
        let mut editor = Editor::new(PathBuf::new());
        editor.choose_brush(Brush::Wall);
        editor.place(&mut scene, GridPos::new(3, 2)).unwrap();
        editor.choose_brush(Brush::Object(72, true));
        editor.place(&mut scene, GridPos::new(2, 2)).unwrap();
        let original = scene.document.clone();
        assert!(editor.rotate_at(&mut scene, GridPos::new(2, 3)).is_err());
        assert_eq!(scene.document, original);
        editor.layers.locked[2] = true;
        assert!(
            editor
                .toggle_blocking_at(&mut scene, GridPos::new(2, 2))
                .is_err()
        );
        editor.erase(&mut scene, GridPos::new(2, 2)).unwrap();
        assert_eq!(scene.document, original);
        editor.layers.visible[0] = false;
        editor.choose_brush(Brush::Floor(Some(4)));
        assert!(editor.place(&mut scene, GridPos::new(5, 5)).is_err());
        assert_eq!(scene.document, original);
        editor.undo(&mut scene).unwrap();
        assert_eq!(scene.document.props.len(), 0);
    }

    #[test]
    fn workbench_catalogue_search_ignores_accents_and_favorites_and_recent_filter() {
        let mut editor = Editor::new(PathBuf::new());
        editor.category = 2;
        editor.search = "coeur".into();
        assert_eq!(editor.entries().len(), 1);
        let brush = editor.entries()[0].1;
        editor.choose_brush(brush);
        editor.filter = 2;
        assert_eq!(editor.entries().len(), 1);
        editor.filter = 1;
        assert!(editor.entries().is_empty());
        editor.favorites.insert(key(brush));
        assert_eq!(editor.entries().len(), 1);
    }

    #[test]
    fn resizing_can_be_undone_and_invalid_crops_leave_document_and_history_intact() {
        let mut scene = Scene::empty(4, 4).unwrap();
        let mut editor = Editor::new(PathBuf::new());
        editor.choose_brush(Brush::Object(103, true));
        editor.place(&mut scene, GridPos::new(3, 3)).unwrap();
        let original = scene.document.clone();
        editor.resize_map(&mut scene, 2, 2, 0).unwrap();
        assert!(scene.document.props.is_empty());
        editor.undo(&mut scene).unwrap();
        assert_eq!(scene.document, original);
        editor.redo(&mut scene).unwrap();
        assert_eq!(scene.document.width, 2);
        let before = scene.document.clone();
        for dimensions in [(0, 2), (2, 257)] {
            assert!(
                editor
                    .resize_map(&mut scene, dimensions.0, dimensions.1, 0)
                    .is_err()
            );
            assert_eq!(scene.document, before);
        }
        editor.undo(&mut scene).unwrap();
        assert_eq!(scene.document, original);
        scene.document.spawn = GridPos::new(3, 2);
        editor.choose_brush(Brush::Wall);
        for y in 0..2 {
            for x in 0..2 {
                editor.place(&mut scene, GridPos::new(x, y)).unwrap();
            }
        }
        let blocked = scene.document.clone();
        assert!(editor.resize_map(&mut scene, 2, 2, 0).is_err());
        assert_eq!(scene.document, blocked);
    }

    #[test]
    fn sf_objects_are_available_without_review_mode_and_filters_and_accesses_work() {
        let mut editor = Editor::new(PathBuf::new());
        editor.category = 2;
        for group in 2..6 {
            editor.object_group = group;
            assert_eq!(editor.entries().len(), 16);
        }
        editor.object_group = 5;
        assert_eq!(editor.entries()[0].1, Brush::Object(88, true));
        editor.choose_brush(Brush::Object(80, true));
        assert!(!editor.blocking);
        let mut scene = Scene::empty(4, 4).unwrap();
        editor.place(&mut scene, GridPos::new(1, 1)).unwrap();
        assert!(scene.game.map().is_walkable(GridPos::new(1, 1)));
        editor.choose_brush(Brush::Object(56, true));
        assert!(editor.blocking);
        for style in 3..5 {
            editor.wall_style = style;
            editor.choose_brush(Brush::Corner(6));
            editor
                .place(&mut scene, GridPos::new(style as i32 - 1, 2))
                .unwrap();
            editor
                .rotate_at(&mut scene, GridPos::new(style as i32 - 1, 2))
                .unwrap();
        }
        let restored = Scene::from_document(
            serde_json::from_slice(&serde_json::to_vec(&scene.document).unwrap()).unwrap(),
        )
        .unwrap();
        assert_eq!(restored.document, scene.document);
    }

    #[test]
    fn urban_catalog_styles_rotations_and_paint_survive_history_and_json() {
        let mut scene = Scene::empty(8, 8).unwrap();
        let mut editor = Editor::new(PathBuf::new());
        editor.category = 0;
        assert_eq!(editor.entries().len(), 22);
        editor.urban_review = true;
        assert_eq!(editor.entries().len(), 22);
        for index in 16..28 {
            editor.brush = Brush::Floor(Some(index));
            editor.round = true;
            assert!(editor.round_ground());
        }
        editor
            .paint_to(&mut scene, super::super::paint::Point::new(96, 96), false)
            .unwrap();
        editor.end_stroke(&scene);
        editor.category = 2;
        assert_eq!(editor.entries().len(), 106);
        editor.brush = Brush::Object(55, true);
        editor.rotation = 3;
        editor.place(&mut scene, GridPos::new(3, 3)).unwrap();
        for style in 1..=2 {
            editor.wall_style = style;
            editor.choose_brush(Brush::Corner(6));
            let pos = GridPos::new(4, style as i32);
            editor.place(&mut scene, pos).unwrap();
            editor.rotate_at(&mut scene, pos).unwrap();
            assert_eq!(scene.document.structures.last().unwrap().style, style);
            editor.pick(&scene, pos);
            assert_eq!(editor.wall_style, style);
        }
        let expected = scene.document.clone();
        editor.undo(&mut scene).unwrap();
        editor.redo(&mut scene).unwrap();
        assert_eq!(scene.document, expected);
        let restored = Scene::from_document(
            serde_json::from_slice(&serde_json::to_vec(&expected).unwrap()).unwrap(),
        )
        .unwrap();
        assert_eq!(restored.document, expected);
        assert_eq!(restored.document.props[0].rotation, 3);
        for reserved in 10..16 {
            let mut invalid = expected.clone();
            invalid.floors[0] = Some(reserved);
            assert!(Scene::from_document(invalid).is_err());
        }
        let mut invalid = expected.clone();
        invalid.structures[0].style = super::super::catalog::WALL_STYLE_COUNT;
        assert!(Scene::from_document(invalid).is_err());
    }

    #[test]
    fn round_water_survives_export_and_history_and_controls_test_passage() {
        let mut document = Scene::empty(8, 6).unwrap().document;
        document.spawn = GridPos::new(3, 2);
        let mut scene = Scene::from_document(document).unwrap();
        let base = scene.document.clone();
        let mut editor = Editor::new(PathBuf::new());
        editor.round = true;
        editor.diameter = 96;
        let pos = GridPos::new(3, 3);
        let point = super::super::paint::Point::new(3 * 64 + 32, 3 * 64 + 32);
        for (material, terrain) in [(8, Terrain::ShallowWater), (9, Terrain::DeepWater)] {
            editor.brush = Brush::Floor(Some(material));
            assert!(editor.round_ground());
            editor.paint_to(&mut scene, point, false).unwrap();
            editor.end_stroke(&scene);
            let json = serde_json::to_vec(&scene.document).unwrap();
            let mut restored =
                Scene::from_document(serde_json::from_slice(&json).unwrap()).unwrap();
            assert_eq!(restored.document, scene.document);
            assert_eq!(restored.game.map().tile(pos).unwrap().terrain, terrain);
            assert_eq!(restored.game.map().is_walkable(pos), material == 8);
            let moved = restored.move_player(project_rl::world::Direction::South);
            assert_eq!(
                matches!(moved, project_rl::game::CommandOutcome::Applied),
                material == 8
            );
        }
        editor.undo(&mut scene).unwrap();
        assert_eq!(
            scene.game.map().tile(pos).unwrap().terrain,
            Terrain::ShallowWater
        );
        editor.redo(&mut scene).unwrap();
        assert_eq!(
            scene.game.map().tile(pos).unwrap().terrain,
            Terrain::DeepWater
        );
        editor.paint_to(&mut scene, point, true).unwrap();
        editor.end_stroke(&scene);
        assert_eq!(
            Scene::from_document(scene.document.clone())
                .unwrap()
                .game
                .map()
                .tile(pos)
                .unwrap()
                .terrain,
            Terrain::Floor
        );
        editor.undo(&mut scene).unwrap();
        assert_eq!(
            scene.game.map().tile(pos).unwrap().terrain,
            Terrain::DeepWater
        );
        editor.undo(&mut scene).unwrap();
        editor.undo(&mut scene).unwrap();
        assert_eq!(scene.document, base);
    }

    #[test]
    fn water_cannot_block_spawn_and_an_invalid_drag_segment_is_rolled_back() {
        let mut scene = Scene::empty(8, 6).unwrap();
        let base = scene.document.clone();
        let mut editor = Editor::new(PathBuf::new());
        editor.round = true;
        editor.diameter = 16;
        editor.brush = Brush::Floor(Some(9));
        let spawn = super::super::paint::Point::new(32, 32);
        assert!(editor.paint_to(&mut scene, spawn, false).is_err());
        assert_eq!(scene.document, base);
        editor.end_stroke(&scene);
        assert!(editor.undo.is_empty());
        editor
            .paint_to(&mut scene, super::super::paint::Point::new(200, 160), false)
            .unwrap();
        let valid = scene.document.clone();
        assert!(editor.paint_to(&mut scene, spawn, false).is_err());
        assert_eq!(scene.document, valid);
        editor.end_stroke(&scene);
        editor.undo(&mut scene).unwrap();
        assert_eq!(scene.document, base);
    }

    #[test]
    fn freehand_history_and_export_preserve_visuals_without_changing_game_terrain() {
        let mut scene = Scene::new();
        let original = scene.document.clone();
        let terrain: Vec<_> = (0..HEIGHT)
            .flat_map(|y| (0..WIDTH).map(move |x| GridPos::new(x, y)))
            .map(|pos| scene.game.map().tile(pos).unwrap().terrain)
            .collect();
        let mut editor = Editor::new(PathBuf::new());
        editor.round = true;
        editor.brush = Brush::Floor(Some(5));
        for point in [(650, 200), (720, 250), (800, 310)] {
            editor
                .paint_to(
                    &mut scene,
                    super::super::paint::Point::new(point.0, point.1),
                    false,
                )
                .unwrap();
        }
        editor.end_stroke(&scene);
        let painted = scene.document.clone();
        assert_eq!(painted.paint.len(), 1);
        assert_eq!(painted.paint[0].points.len(), 3);
        assert_eq!(painted.floors, original.floors);
        assert_eq!(painted.structures, original.structures);
        assert_eq!(painted.props, original.props);
        assert_eq!(scene.game.turn(), 0);
        assert_eq!(scene.game.player_position(), Some(original.spawn));
        editor.undo(&mut scene).unwrap();
        assert_eq!(scene.document, original);
        editor.redo(&mut scene).unwrap();
        assert_eq!(scene.document, painted);
        let restored = Scene::from_document(
            serde_json::from_slice(&serde_json::to_vec(&painted).unwrap()).unwrap(),
        )
        .unwrap();
        assert_eq!(restored.document, painted);
        let restored_terrain: Vec<_> = (0..HEIGHT)
            .flat_map(|y| (0..WIDTH).map(move |x| GridPos::new(x, y)))
            .map(|pos| restored.game.map().tile(pos).unwrap().terrain)
            .collect();
        assert_eq!(restored_terrain, terrain);
        editor
            .paint_to(&mut scene, super::super::paint::Point::new(720, 250), true)
            .unwrap();
        editor.end_stroke(&scene);
        assert_eq!(scene.document.paint.last().unwrap().material, None);
        editor.undo(&mut scene).unwrap();
        assert_eq!(scene.document, painted);
    }

    #[test]
    fn old_maps_and_invalid_freehand_data_leave_the_current_scene_safe() {
        let original = Scene::new().document;
        let json = serde_json::to_value(&original).unwrap();
        assert!(json.get("paint").is_none());
        assert_eq!(
            Scene::from_document(serde_json::from_value(json).unwrap())
                .unwrap()
                .document,
            original
        );
        for (material, diameter, point) in [
            (Some(10), 96, (80, 80)),
            (Some(5), 0, (80, 80)),
            (Some(5), 96, (-1, 80)),
            (Some(5), 96, (WIDTH * 64, 80)),
        ] {
            let mut document = original.clone();
            document.paint.push(super::super::paint::Stroke {
                clip: None,
                material,
                diameter,
                points: vec![super::super::paint::Point::new(point.0, point.1)],
            });
            assert!(Scene::from_document(document).is_err());
        }
        let mut scene = Scene::new();
        let mut editor = Editor::new(PathBuf::new());
        editor.round = true;
        editor.brush = Brush::Floor(Some(5));
        assert!(
            editor
                .paint_to(&mut scene, super::super::paint::Point::new(-1, 80), false)
                .is_err()
        );
        assert_eq!(scene.document, original);
        assert!(editor.undo.is_empty());
    }

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
                        style: 0,
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

pub fn fold(text: &str) -> String {
    text.to_lowercase()
        .replace("œ", "oe")
        .replace("æ", "ae")
        .chars()
        .map(|c| match c {
            'é' | 'è' | 'ê' | 'ë' => 'e',
            'à' | 'â' | 'ä' => 'a',
            'î' | 'ï' => 'i',
            'ô' | 'ö' => 'o',
            'ù' | 'û' | 'ü' => 'u',
            'ç' => 'c',
            _ => c,
        })
        .collect()
}
pub fn key(brush: Brush) -> String {
    match brush {
        Brush::Floor(Some(i)) => format!("floor.{i}"),
        Brush::Floor(None) => "floor.background".into(),
        Brush::Wall => "wall.auto".into(),
        Brush::Corner(mask) => format!("wall.corner.{mask}"),
        Brush::Door(open) => format!("door.{open}"),
        Brush::Object(i, true) => format!("object.{i}"),
        Brush::Object(i, false) => format!("terrain.{i}"),
        Brush::Spawn => "spawn".into(),
    }
}
