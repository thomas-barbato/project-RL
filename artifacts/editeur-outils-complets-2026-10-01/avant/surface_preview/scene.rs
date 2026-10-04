//! Deliberately authored test room, not a replacement for campaign generation.
use project_rl::game::{CommandOutcome, GameCommand, GameState};
use project_rl::world::{Direction, DoorState, GridPos, Map, Terrain};

pub const WIDTH: i32 = 20;
pub const HEIGHT: i32 = 10;
pub const DOOR: GridPos = GridPos::new(9, 4);
pub const START: GridPos = GridPos::new(6, 4);
pub const MAX_SIDE: i32 = 256;
/// Palette order: upper left, upper right, lower left, lower right.
pub const CORNER_MASKS: [u8; 4] = [6, 12, 3, 9];

pub fn rotate_connections(mask: u8, turns: usize) -> u8 {
    let turns = turns % 4;
    ((mask << turns) | (mask >> (4 - turns))) & 15
}

/// Horizontal and vertical sources are drawn separately. The opposite face
/// uses a half turn; corners always select their own painted orientation.
pub fn fixed_wall_sprite(mask: u8, turns: usize) -> (usize, usize) {
    if CORNER_MASKS.contains(&mask) {
        return (rotate_connections(mask, turns) as usize, 0);
    }
    if mask == 10 || mask == 5 {
        let face = (usize::from(mask == 5) + turns) % 4;
        return match face {
            0 => (10, 0),
            1 => (5, 2),
            2 => (10, 2),
            _ => (5, 0),
        };
    }
    (mask as usize, turns)
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Prop {
    pub pos: GridPos,
    pub sprite: usize,
    pub furniture: bool,
    #[serde(default)]
    pub rotation: usize,
    pub blocking: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Structure {
    pub pos: GridPos,
    /// None means wall; Some means a door with an explicit initial state.
    pub door: Option<DoorState>,
    pub rotation: usize,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub style: usize,
    /// None follows neighbors. Some stores a wall shape before its rotation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_connections: Option<u8>,
}

fn is_zero(value: &usize) -> bool {
    *value == 0
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Document {
    pub version: u32,
    pub width: i32,
    pub height: i32,
    pub spawn: GridPos,
    /// None retains the surface test's authored background.
    pub floors: Vec<Option<usize>>,
    #[serde(default = "origin", skip_serializing_if = "is_origin")]
    pub background_offset: GridPos,
    /// Freehand ground appearance, independent from simulation terrain.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub paint: Vec<super::paint::Stroke>,
    pub structures: Vec<Structure>,
    pub props: Vec<Prop>,
}

impl Document {
    pub fn background_position(&self, pos: GridPos) -> GridPos {
        GridPos::new(
            pos.x - self.background_offset.x,
            pos.y - self.background_offset.y,
        )
    }
    pub fn ground_terrain(&self, pos: GridPos) -> Terrain {
        let base = self.floors[(pos.y * self.width + pos.x) as usize].unwrap_or_else(|| {
            let source = self.background_position(pos);
            match if (0..WIDTH).contains(&source.x) && (0..HEIGHT).contains(&source.y) {
                water(source)
            } else {
                None
            } {
                Some(Terrain::ShallowWater) => 8,
                Some(Terrain::DeepWater) => 9,
                _ => 6,
            }
        });
        let point = super::paint::Point::new(pos.x * 64 + 32, pos.y * 64 + 32);
        match super::paint::material_at(point, &self.paint, base) {
            8 => Terrain::ShallowWater,
            9 => Terrain::DeepWater,
            _ => Terrain::Floor,
        }
    }
}

fn origin() -> GridPos {
    GridPos::new(0, 0)
}
fn is_origin(pos: &GridPos) -> bool {
    *pos == origin()
}

pub struct Scene {
    pub game: GameState,
    pub props: Vec<Prop>,
    pub document: Document,
    structural_positions: std::collections::BTreeSet<GridPos>,
}

pub fn structure(pos: GridPos) -> bool {
    (pos.x == 1 || pos.x == 9) && (1..=8).contains(&pos.y)
        || (pos.y == 1 || pos.y == 8) && (1..=9).contains(&pos.x)
}

pub fn shoreline(y: f32) -> f32 {
    18.8 - (y - 3.0).max(0.0) * 0.52 + (y * 1.7).sin() * 0.18
}

pub fn water(pos: GridPos) -> Option<Terrain> {
    let distance = pos.x as f32 + 0.5 - shoreline(pos.y as f32 + 0.5);
    if distance > 1.0 {
        Some(Terrain::DeepWater)
    } else if distance > 0.0 {
        Some(Terrain::ShallowWater)
    } else {
        None
    }
}

impl Scene {
    pub fn new() -> Self {
        let props = vec![
            Prop {
                pos: GridPos::new(6, 2),
                sprite: 0,
                furniture: true,
                rotation: 0,
                blocking: true,
            },
            Prop {
                pos: GridPos::new(6, 3),
                sprite: 1,
                furniture: true,
                rotation: 0,
                blocking: true,
            },
            Prop {
                pos: GridPos::new(8, 2),
                sprite: 2,
                furniture: true,
                rotation: 0,
                blocking: true,
            },
            Prop {
                pos: GridPos::new(2, 6),
                sprite: 3,
                furniture: true,
                rotation: 0,
                blocking: true,
            },
            Prop {
                pos: GridPos::new(7, 6),
                sprite: 4,
                furniture: true,
                rotation: 0,
                blocking: true,
            },
            Prop {
                pos: GridPos::new(2, 2),
                sprite: 5,
                furniture: true,
                rotation: 0,
                blocking: true,
            },
            Prop {
                pos: GridPos::new(4, 7),
                sprite: 6,
                furniture: true,
                rotation: 0,
                blocking: true,
            },
            Prop {
                pos: GridPos::new(8, 7),
                sprite: 7,
                furniture: true,
                rotation: 0,
                blocking: true,
            },
            Prop {
                pos: GridPos::new(3, 6),
                sprite: 14,
                furniture: false,
                rotation: 0,
                blocking: true,
            },
            Prop {
                pos: GridPos::new(8, 3),
                sprite: 15,
                furniture: false,
                rotation: 0,
                blocking: true,
            },
        ];
        let mut structures = Vec::new();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pos = GridPos::new(x, y);
                if structure(pos) {
                    structures.push(Structure {
                        pos,
                        door: if pos == DOOR {
                            Some(DoorState::Closed)
                        } else {
                            None
                        },
                        rotation: if pos == DOOR { 1 } else { 0 },
                        style: 0,
                        fixed_connections: None,
                    });
                }
            }
        }
        Self::from_document(Document {
            background_offset: origin(),
            version: 1,
            width: WIDTH,
            height: HEIGHT,
            spawn: START,
            floors: vec![None; (WIDTH * HEIGHT) as usize],
            paint: vec![],
            structures,
            props,
        })
        .unwrap()
    }

    pub fn from_document(document: Document) -> Result<Self, String> {
        if document.version != 1 {
            return Err("Version de carte incompatible.".into());
        }
        if !(1..=MAX_SIDE).contains(&document.width) || !(1..=MAX_SIDE).contains(&document.height) {
            return Err(format!(
                "Chaque dimension doit être comprise entre 1 et {MAX_SIDE} cases."
            ));
        }
        if document.floors.len() != (document.width * document.height) as usize
            || !(-MAX_SIDE * 3..=MAX_SIDE * 3).contains(&document.background_offset.x)
            || !(-MAX_SIDE * 3..=MAX_SIDE * 3).contains(&document.background_offset.y)
            || document
                .floors
                .iter()
                .flatten()
                .any(|&i| !super::catalog::is_floor(i))
        {
            return Err("Données de sol invalides.".into());
        }
        super::paint::validate(&document.paint, document.width, document.height)?;
        let mut map = Map::filled(
            document.width as usize,
            document.height as usize,
            Terrain::Floor,
        )
        .unwrap();
        for y in 0..document.height {
            for x in 0..document.width {
                let pos = GridPos::new(x, y);
                let terrain = document.ground_terrain(pos);
                map.set_terrain(pos, terrain).unwrap();
            }
        }
        let mut occupied = std::collections::BTreeSet::new();
        for part in &document.structures {
            if !map.contains(part.pos)
                || part.rotation > 3
                || part.style >= super::catalog::WALL_STYLE_COUNT
                || part
                    .fixed_connections
                    .is_some_and(|mask| mask > 15 || part.door.is_some())
                || !occupied.insert(part.pos)
            {
                return Err(
                    "Mur ou porte invalide, ou plusieurs éléments sur la même case.".into(),
                );
            }
            map.set_terrain(
                part.pos,
                part.door.map(Terrain::Door).unwrap_or(Terrain::Wall),
            )
            .unwrap();
        }
        for prop in &document.props {
            if !map.contains(prop.pos)
                || prop.rotation > 3
                || (prop.furniture && prop.sprite >= super::catalog::FURNITURE_COUNT)
                || (!prop.furniture && ![14, 15].contains(&prop.sprite))
                || !occupied.insert(prop.pos)
            {
                return Err("Objet invalide, ou plusieurs éléments sur la même case.".into());
            }
            // Fixture collision proxies. They are not campaign decoration data.
            if prop.blocking {
                map.set_terrain(prop.pos, Terrain::Wall).unwrap();
            }
        }
        let game = GameState::new(map, document.spawn, 64064)
            .map_err(|_| "Le départ doit être sur une case accessible.".to_owned())?;
        let structural_positions = document.structures.iter().map(|part| part.pos).collect();
        Ok(Self {
            game,
            props: document.props.clone(),
            document,
            structural_positions,
        })
    }

    pub fn is_structure(&self, pos: GridPos) -> bool {
        self.structural_positions.contains(&pos)
    }

    /// Visible ground beyond a wall face comes from that side of the wall,
    /// rather than revealing the whole stored floor tile through its margins.
    /// The center and door threshold keep their authored floor unchanged.
    pub fn margin_ground(&self, pos: GridPos, dx: i32, dy: i32) -> Option<GridPos> {
        assert!((-1..=1).contains(&dx) && (-1..=1).contains(&dy) && (dx, dy) != (0, 0));
        let target = GridPos::new(pos.x + dx, pos.y + dy);
        if !self.game.map().contains(target) {
            return None;
        }
        if !self.is_structure(target) {
            return Some(target);
        }
        // A diagonal wall can leave a margin adjoining an accessible side.
        if dx != 0 && dy != 0 {
            for side in [
                GridPos::new(pos.x + dx, pos.y),
                GridPos::new(pos.x, pos.y + dy),
            ] {
                if !self.is_structure(side) {
                    return Some(side);
                }
            }
        }
        Some(pos)
    }

    pub fn empty(width: i32, height: i32) -> Result<Self, String> {
        if !(1..=MAX_SIDE).contains(&width) || !(1..=MAX_SIDE).contains(&height) {
            return Err(format!(
                "Chaque dimension doit être comprise entre 1 et {MAX_SIDE} cases."
            ));
        }
        Self::from_document(Document {
            background_offset: origin(),
            version: 1,
            width,
            height,
            spawn: GridPos::new(0, 0),
            floors: vec![Some(6); (width * height) as usize],
            paint: vec![],
            structures: vec![],
            props: vec![],
        })
    }

    pub fn interaction(&mut self) -> CommandOutcome {
        let player = self.game.player_position().unwrap();
        let target = player
            .cardinal_neighbors()
            .into_iter()
            .find(|&pos| {
                matches!(
                    self.game.map().tile(pos).map(|tile| tile.terrain),
                    Some(Terrain::Door(_))
                )
            })
            .unwrap_or(DOOR);
        self.game
            .process_player_command(GameCommand::Interact { target })
    }

    pub fn neighbors(&self, pos: GridPos) -> u8 {
        pos.cardinal_neighbors()
            .into_iter()
            .enumerate()
            .fold(0, |mask, (index, neighbor)| {
                if self.is_structure(neighbor) {
                    mask | (1 << index)
                } else {
                    mask
                }
            })
    }

    pub fn wall_sprite(&self, part: &Structure) -> (usize, usize) {
        self.wall_preview(part.pos, part.rotation, part.fixed_connections)
    }

    /// Manual shapes keep all four faces. Automatic straight runs follow the
    /// facing of the corner at their end, rather than rotating one source.
    pub fn wall_preview(&self, pos: GridPos, rotation: usize, fixed: Option<u8>) -> (usize, usize) {
        match fixed {
            Some(mask) => fixed_wall_sprite(mask, rotation),
            None => match self.neighbors(pos) {
                0 => fixed_wall_sprite(10, rotation),
                mask @ (5 | 10) => fixed_wall_sprite(mask, self.straight_facing(pos, mask)),
                mask => (mask as usize, 0),
            },
        }
    }

    pub fn straight_facing(&self, pos: GridPos, mask: u8) -> usize {
        let directions = if mask == 10 {
            [(-1, 0), (1, 0)]
        } else {
            [(0, -1), (0, 1)]
        };
        for (dx, dy) in directions {
            let mut cursor = GridPos::new(pos.x + dx, pos.y + dy);
            while self.is_structure(cursor) {
                let end = self.neighbors(cursor);
                if end != mask {
                    if CORNER_MASKS.contains(&end) {
                        return if mask == 10 {
                            usize::from(end & 1 != 0) * 2
                        } else {
                            usize::from(end & 2 != 0) * 2
                        };
                    }
                    break;
                }
                cursor = GridPos::new(cursor.x + dx, cursor.y + dy);
            }
        }
        0
    }

    pub fn move_player(&mut self, direction: Direction) -> CommandOutcome {
        self.game
            .process_player_command(GameCommand::Move(direction))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corners_follow_the_actual_map_connections() {
        let scene = Scene::new();
        for (pos, mask) in [
            (GridPos::new(1, 1), 6),
            (GridPos::new(9, 1), 12),
            (GridPos::new(1, 8), 3),
            (GridPos::new(9, 8), 9),
        ] {
            let part = scene
                .document
                .structures
                .iter()
                .find(|part| part.pos == pos)
                .unwrap();
            assert_eq!(scene.wall_sprite(part), (mask, 0));
        }
    }

    #[test]
    fn wall_margins_use_the_floor_on_each_side_and_each_corner() {
        let scene = Scene::new();
        for (wall, outside, inside) in [
            (GridPos::new(4, 1), (0, -1), (0, 1)),
            (GridPos::new(9, 4), (1, 0), (-1, 0)),
            (GridPos::new(4, 8), (0, 1), (0, -1)),
            (GridPos::new(1, 4), (-1, 0), (1, 0)),
        ] {
            for (dx, dy) in [outside, inside] {
                assert_eq!(
                    scene.margin_ground(wall, dx, dy),
                    Some(GridPos::new(wall.x + dx, wall.y + dy))
                );
            }
        }
        for corner in [
            GridPos::new(1, 1),
            GridPos::new(9, 1),
            GridPos::new(1, 8),
            GridPos::new(9, 8),
        ] {
            for dx in [-1, 1] {
                for dy in [-1, 1] {
                    assert_eq!(
                        scene.margin_ground(corner, dx, dy),
                        Some(GridPos::new(corner.x + dx, corner.y + dy))
                    );
                }
            }
        }
    }

    #[test]
    fn wall_margins_handle_map_edges_and_diagonal_structures() {
        let mut document = Scene::empty(5, 5).unwrap().document;
        document.structures = [(0, 1), (1, 1), (2, 0)]
            .into_iter()
            .map(|(x, y)| Structure {
                pos: GridPos::new(x, y),
                door: None,
                rotation: 0,
                style: 0,
                fixed_connections: None,
            })
            .collect();
        // Deliberately paint concrete under the boundary wall: its outside
        // margin must still use the exterior, not leak that stored concrete.
        document.floors[5] = Some(2);
        let scene = Scene::from_document(document).unwrap();
        assert_eq!(scene.margin_ground(GridPos::new(0, 1), -1, 0), None);
        assert_eq!(
            scene.margin_ground(GridPos::new(1, 1), 1, -1),
            Some(GridPos::new(2, 1))
        );
        assert_eq!(scene.document.floors[5], Some(2));
    }

    #[test]
    fn engine_door_controls_passage_and_can_be_closed_again() {
        let mut scene = Scene::new();
        for _ in 0..2 {
            assert_eq!(scene.move_player(Direction::East), CommandOutcome::Applied);
        }
        assert_eq!(scene.game.player_position(), Some(GridPos::new(8, 4)));
        assert!(!scene.game.map().is_walkable(DOOR));
        assert_eq!(scene.interaction(), CommandOutcome::Applied);
        assert!(scene.game.map().is_walkable(DOOR));
        assert_eq!(scene.move_player(Direction::East), CommandOutcome::Applied);
        assert_eq!(scene.move_player(Direction::East), CommandOutcome::Applied);
        assert_eq!(scene.game.player_position(), Some(GridPos::new(10, 4)));
        assert_eq!(scene.interaction(), CommandOutcome::Applied);
        assert!(!scene.game.map().is_walkable(DOOR));
    }

    #[test]
    fn furniture_and_deep_water_remain_obstacles_in_the_fixture() {
        let mut scene = Scene::new();
        assert!(!scene.game.map().is_walkable(GridPos::new(6, 3)));
        scene.move_player(Direction::North);
        assert_eq!(scene.game.player_position(), Some(START));
        assert!(!scene.game.map().is_walkable(GridPos::new(19, 9)));
        assert_eq!(scene.game.turn(), 0);
    }

    #[test]
    fn largest_allowed_map_and_fallback_ground_have_matching_semantics() {
        let mut document = Scene::empty(MAX_SIDE, MAX_SIDE).unwrap().document;
        let corner = GridPos::new(MAX_SIDE - 1, MAX_SIDE - 1);
        document.floors[(corner.y * MAX_SIDE + corner.x) as usize] = None;
        let scene = Scene::from_document(document).unwrap();
        assert!(scene.game.map().is_walkable(corner));
        assert_eq!(scene.game.map().width(), MAX_SIDE as usize);
    }
}
