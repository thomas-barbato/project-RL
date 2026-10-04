use std::error::Error;
use std::fmt::{Display, Formatter};

use super::{GridPos, Terrain, TileState};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<TileState>,
    /// Transient placed obstacles. The editor reconstructs these from its document.
    /// Keep the campaign's existing JSON and bincode map layout unchanged.
    #[serde(skip)]
    obstructions: std::collections::BTreeMap<usize, (bool, bool)>,
}

impl Map {
    pub fn filled(width: usize, height: usize, terrain: Terrain) -> Result<Self, MapBuildError> {
        if width == 0 || height == 0 {
            return Err(MapBuildError::EmptyDimensions);
        }
        let tile_count = width
            .checked_mul(height)
            .ok_or(MapBuildError::DimensionsTooLarge { width, height })?;

        Ok(Self {
            width,
            height,
            tiles: vec![TileState::new(terrain); tile_count],
            obstructions: Default::default(),
        })
    }

    /// Parses a small engine-level map fixture. `#` is a wall and `.` a floor.
    /// Procedural generation will produce this same runtime representation.
    pub fn from_ascii(definition: &str) -> Result<Self, MapParseError> {
        let rows: Vec<&str> = definition.lines().filter(|row| !row.is_empty()).collect();
        let first_row = rows.first().ok_or(MapParseError::EmptyMap)?;
        let width = first_row.chars().count();

        if width == 0 {
            return Err(MapParseError::EmptyMap);
        }

        let mut tiles = Vec::with_capacity(width * rows.len());

        for (y, row) in rows.iter().enumerate() {
            let actual_width = row.chars().count();
            if actual_width != width {
                return Err(MapParseError::UnevenRow {
                    row: y,
                    expected: width,
                    actual: actual_width,
                });
            }

            for (x, symbol) in row.chars().enumerate() {
                let terrain = match symbol {
                    '.' => Terrain::Floor,
                    '#' => Terrain::Wall,
                    _ => return Err(MapParseError::UnknownSymbol { symbol, x, y }),
                };
                tiles.push(TileState::new(terrain));
            }
        }

        Ok(Self {
            width,
            height: rows.len(),
            tiles,
            obstructions: Default::default(),
        })
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }

    pub fn tile(&self, position: GridPos) -> Option<&TileState> {
        self.index(position).and_then(|index| self.tiles.get(index))
    }

    pub fn set_terrain(
        &mut self,
        position: GridPos,
        terrain: Terrain,
    ) -> Result<(), MapBuildError> {
        let index = self
            .index(position)
            .ok_or(MapBuildError::PositionOutsideMap(position))?;
        let tile = self
            .tiles
            .get_mut(index)
            .ok_or(MapBuildError::PositionOutsideMap(position))?;
        tile.terrain = terrain;
        Ok(())
    }

    pub fn contains(&self, position: GridPos) -> bool {
        self.index(position).is_some()
    }

    pub fn is_protected(&self, position: GridPos) -> bool {
        self.tile(position).is_some_and(|tile| tile.protected)
    }

    pub fn set_protected(
        &mut self,
        position: GridPos,
        protected: bool,
    ) -> Result<(), MapBuildError> {
        let index = self
            .index(position)
            .ok_or(MapBuildError::PositionOutsideMap(position))?;
        self.tiles[index].protected = protected;
        Ok(())
    }

    pub fn is_walkable(&self, position: GridPos) -> bool {
        self.tile(position)
            .is_some_and(|tile| !tile.terrain.blocks_movement())
            && !self.obstruction(position).0
    }

    pub fn blocks_vision(&self, position: GridPos) -> bool {
        self.tile(position)
            .is_none_or(|tile| tile.terrain.blocks_vision())
            || self.obstruction(position).1
    }

    /// Movement and sight are independent from the ground material.
    pub fn set_obstruction(
        &mut self,
        position: GridPos,
        blocks_movement: bool,
        blocks_vision: bool,
    ) -> Result<(), MapBuildError> {
        let index = self
            .index(position)
            .ok_or(MapBuildError::PositionOutsideMap(position))?;
        if blocks_movement || blocks_vision {
            self.obstructions
                .insert(index, (blocks_movement, blocks_vision));
        } else {
            self.obstructions.remove(&index);
        }
        Ok(())
    }

    pub fn obstruction(&self, position: GridPos) -> (bool, bool) {
        self.index(position)
            .and_then(|index| self.obstructions.get(&index).copied())
            .unwrap_or_default()
    }

    fn index(&self, position: GridPos) -> Option<usize> {
        if position.x < 0 || position.y < 0 {
            return None;
        }

        let x = usize::try_from(position.x).ok()?;
        let y = usize::try_from(position.y).ok()?;

        (x < self.width && y < self.height).then_some(y * self.width + x)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MapParseError {
    EmptyMap,
    UnevenRow {
        row: usize,
        expected: usize,
        actual: usize,
    },
    UnknownSymbol {
        symbol: char,
        x: usize,
        y: usize,
    },
}

impl Display for MapParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyMap => write!(formatter, "map definition is empty"),
            Self::UnevenRow {
                row,
                expected,
                actual,
            } => write!(
                formatter,
                "map row {row} has width {actual}, expected {expected}"
            ),
            Self::UnknownSymbol { symbol, x, y } => {
                write!(formatter, "unknown map symbol '{symbol}' at ({x}, {y})")
            }
        }
    }
}

impl Error for MapParseError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapBuildError {
    EmptyDimensions,
    DimensionsTooLarge { width: usize, height: usize },
    PositionOutsideMap(GridPos),
}

impl Display for MapBuildError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyDimensions => write!(formatter, "map dimensions must be non-zero"),
            Self::DimensionsTooLarge { width, height } => {
                write!(formatter, "map dimensions {width}x{height} are too large")
            }
            Self::PositionOutsideMap(position) => write!(
                formatter,
                "position ({}, {}) is outside the map",
                position.x, position.y
            ),
        }
    }
}

impl Error for MapBuildError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placed_obstacles_distinguish_movement_from_sight() {
        let mut map = Map::filled(3, 3, Terrain::Floor).unwrap();
        let pos = GridPos::new(1, 1);
        for (movement, vision) in [(true, false), (false, true), (true, true), (false, false)] {
            map.set_obstruction(pos, movement, vision).unwrap();
            assert_eq!(map.is_walkable(pos), !movement);
            assert_eq!(map.blocks_vision(pos), vision);
            assert_eq!(map.tile(pos).unwrap().terrain, Terrain::Floor);
        }
        map.set_terrain(pos, Terrain::Wall).unwrap();
        assert!(!map.is_walkable(pos));
        assert!(map.blocks_vision(pos));
        assert!(
            map.set_obstruction(GridPos::new(9, 9), true, false)
                .is_err()
        );
    }

    #[test]
    fn transient_obstacles_preserve_legacy_json_and_binary_layout() {
        #[derive(serde::Serialize)]
        struct Legacy<'a> {
            width: usize,
            height: usize,
            tiles: &'a [TileState],
        }
        let mut map = valid_test_map();
        let old = Legacy {
            width: map.width,
            height: map.height,
            tiles: &map.tiles,
        };
        let json = serde_json::to_vec(&old).unwrap();
        let binary = bincode::serialize(&old).unwrap();
        assert_eq!(serde_json::from_slice::<Map>(&json).unwrap(), map);
        assert_eq!(bincode::deserialize::<Map>(&binary).unwrap(), map);
        map.set_obstruction(GridPos::new(1, 1), true, false)
            .unwrap();
        assert_eq!(serde_json::to_vec(&map).unwrap(), json);
        assert_eq!(bincode::serialize(&map).unwrap(), binary);
    }

    fn valid_test_map() -> Map {
        match Map::from_ascii("###\n#.#\n###") {
            Ok(map) => map,
            Err(error) => panic!("valid test map failed to parse: {error}"),
        }
    }

    #[test]
    fn map_exposes_terrain_and_treats_out_of_bounds_as_blocked() {
        let map = valid_test_map();

        assert_eq!(map.width(), 3);
        assert_eq!(map.height(), 3);
        assert!(map.is_walkable(GridPos::new(1, 1)));
        assert!(!map.is_walkable(GridPos::new(0, 0)));
        assert!(!map.is_walkable(GridPos::new(-1, 1)));
        assert!(!map.is_walkable(GridPos::new(3, 1)));
    }

    #[test]
    fn parser_reports_an_uneven_row_with_context() {
        let result = Map::from_ascii("###\n##");

        assert_eq!(
            result,
            Err(MapParseError::UnevenRow {
                row: 1,
                expected: 3,
                actual: 2,
            })
        );
    }

    #[test]
    fn parser_reports_an_unknown_symbol_with_coordinates() {
        let result = Map::from_ascii("###\n#x#\n###");

        assert_eq!(
            result,
            Err(MapParseError::UnknownSymbol {
                symbol: 'x',
                x: 1,
                y: 1,
            })
        );
    }

    #[test]
    fn filled_map_can_be_carved_safely() {
        let mut map = match Map::filled(3, 2, Terrain::Wall) {
            Ok(map) => map,
            Err(error) => panic!("valid map dimensions were rejected: {error}"),
        };

        assert_eq!(map.set_terrain(GridPos::new(1, 1), Terrain::Floor), Ok(()));
        assert!(map.is_walkable(GridPos::new(1, 1)));
        assert_eq!(
            map.set_terrain(GridPos::new(3, 1), Terrain::Floor),
            Err(MapBuildError::PositionOutsideMap(GridPos::new(3, 1)))
        );
    }

    #[test]
    fn water_keeps_movement_and_vision_rules_distinct() {
        let mut map = Map::filled(3, 3, Terrain::Floor).unwrap();
        let shallow = GridPos::new(1, 1);
        let deep = GridPos::new(2, 1);
        map.set_terrain(shallow, Terrain::ShallowWater).unwrap();
        map.set_terrain(deep, Terrain::DeepWater).unwrap();

        assert!(map.is_walkable(shallow));
        assert!(!map.blocks_vision(shallow));
        assert!(!map.is_walkable(deep));
        assert!(!map.blocks_vision(deep));
    }
}
