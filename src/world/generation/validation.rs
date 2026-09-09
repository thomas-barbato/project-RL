use std::collections::{BTreeSet, VecDeque};
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::world::{GridPos, Map};

/// Navigation view consumed by validation. A future decorated-level wrapper can
/// include blocking props or actors without changing the validator.
pub trait WalkabilityQuery {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn is_walkable(&self, position: GridPos) -> bool;
}

impl WalkabilityQuery for Map {
    fn width(&self) -> usize {
        self.width()
    }

    fn height(&self) -> usize {
        self.height()
    }

    fn is_walkable(&self, position: GridPos) -> bool {
        self.is_walkable(position)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MapValidationRules {
    pub require_sealed_border: bool,
    pub require_all_walkable_connected: bool,
}

impl Default for MapValidationRules {
    fn default() -> Self {
        Self {
            require_sealed_border: true,
            require_all_walkable_connected: true,
        }
    }
}

/// Verifies navigation invariants after generation and again after decoration.
/// `required_positions` represents mandatory objectives, machines or pickups.
pub fn validate_playable_map<Q: WalkabilityQuery>(
    map: &Q,
    player_start: GridPos,
    exit: GridPos,
    required_positions: &[GridPos],
    rules: MapValidationRules,
) -> Result<(), MapValidationError> {
    if !map.is_walkable(player_start) {
        return Err(MapValidationError::BlockedPlayerStart(player_start));
    }
    if !map.is_walkable(exit) {
        return Err(MapValidationError::BlockedExit(exit));
    }

    for (index, position) in required_positions.iter().copied().enumerate() {
        if !map.is_walkable(position) {
            return Err(MapValidationError::BlockedRequiredPosition { index, position });
        }
    }

    if rules.require_sealed_border {
        validate_sealed_border(map)?;
    }

    let reachable = flood_fill(map, player_start);
    if !reachable.contains(&exit) {
        return Err(MapValidationError::UnreachableExit(exit));
    }

    for (index, position) in required_positions.iter().copied().enumerate() {
        if !reachable.contains(&position) {
            return Err(MapValidationError::UnreachableRequiredPosition { index, position });
        }
    }

    if rules.require_all_walkable_connected {
        for y in 0..map.height() {
            for x in 0..map.width() {
                let Some(position) = grid_position(x, y) else {
                    return Err(MapValidationError::DimensionsExceedGridCoordinates);
                };
                if map.is_walkable(position) && !reachable.contains(&position) {
                    return Err(MapValidationError::DisconnectedWalkableArea(position));
                }
            }
        }
    }

    Ok(())
}

fn validate_sealed_border<Q: WalkabilityQuery>(map: &Q) -> Result<(), MapValidationError> {
    if map.width() == 0 || map.height() == 0 {
        return Err(MapValidationError::EmptyMap);
    }

    let last_x = map.width() - 1;
    let last_y = map.height() - 1;

    for x in 0..map.width() {
        for y in [0, last_y] {
            let Some(position) = grid_position(x, y) else {
                return Err(MapValidationError::DimensionsExceedGridCoordinates);
            };
            if map.is_walkable(position) {
                return Err(MapValidationError::OpenBorder(position));
            }
        }
    }

    for y in 0..map.height() {
        for x in [0, last_x] {
            let Some(position) = grid_position(x, y) else {
                return Err(MapValidationError::DimensionsExceedGridCoordinates);
            };
            if map.is_walkable(position) {
                return Err(MapValidationError::OpenBorder(position));
            }
        }
    }

    Ok(())
}

fn flood_fill<Q: WalkabilityQuery>(map: &Q, origin: GridPos) -> BTreeSet<GridPos> {
    let mut visited = BTreeSet::new();
    let mut frontier = VecDeque::from([origin]);
    visited.insert(origin);

    while let Some(position) = frontier.pop_front() {
        for neighbor in position.cardinal_neighbors() {
            if map.is_walkable(neighbor) && visited.insert(neighbor) {
                frontier.push_back(neighbor);
            }
        }
    }

    visited
}

fn grid_position(x: usize, y: usize) -> Option<GridPos> {
    Some(GridPos::new(i32::try_from(x).ok()?, i32::try_from(y).ok()?))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapValidationError {
    EmptyMap,
    DimensionsExceedGridCoordinates,
    BlockedPlayerStart(GridPos),
    BlockedExit(GridPos),
    BlockedRequiredPosition { index: usize, position: GridPos },
    OpenBorder(GridPos),
    UnreachableExit(GridPos),
    UnreachableRequiredPosition { index: usize, position: GridPos },
    DisconnectedWalkableArea(GridPos),
}

impl Display for MapValidationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyMap => write!(formatter, "generated map is empty"),
            Self::DimensionsExceedGridCoordinates => {
                write!(
                    formatter,
                    "map dimensions exceed supported grid coordinates"
                )
            }
            Self::BlockedPlayerStart(position) => write!(
                formatter,
                "player start ({}, {}) is blocked",
                position.x, position.y
            ),
            Self::BlockedExit(position) => {
                write!(
                    formatter,
                    "exit ({}, {}) is blocked",
                    position.x, position.y
                )
            }
            Self::BlockedRequiredPosition { index, position } => write!(
                formatter,
                "required position #{index} ({}, {}) is blocked",
                position.x, position.y
            ),
            Self::OpenBorder(position) => write!(
                formatter,
                "walkable tile ({}, {}) opens the outer map border",
                position.x, position.y
            ),
            Self::UnreachableExit(position) => write!(
                formatter,
                "exit ({}, {}) cannot be reached from player start",
                position.x, position.y
            ),
            Self::UnreachableRequiredPosition { index, position } => write!(
                formatter,
                "required position #{index} ({}, {}) cannot be reached",
                position.x, position.y
            ),
            Self::DisconnectedWalkableArea(position) => write!(
                formatter,
                "walkable area near ({}, {}) is disconnected from player start",
                position.x, position.y
            ),
        }
    }
}

impl Error for MapValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_map(definition: &str) -> Map {
        match Map::from_ascii(definition) {
            Ok(map) => map,
            Err(error) => panic!("valid test map failed to parse: {error}"),
        }
    }

    #[test]
    fn connected_sealed_map_is_accepted() {
        let map = parse_map("#######\n#.....#\n###.###\n#.....#\n#######");

        assert_eq!(
            validate_playable_map(
                &map,
                GridPos::new(1, 1),
                GridPos::new(5, 3),
                &[GridPos::new(3, 2)],
                MapValidationRules::default(),
            ),
            Ok(())
        );
    }

    #[test]
    fn disconnected_exit_is_rejected() {
        let map = parse_map("#########\n#..#....#\n#..#....#\n#########");

        assert_eq!(
            validate_playable_map(
                &map,
                GridPos::new(1, 1),
                GridPos::new(5, 1),
                &[],
                MapValidationRules::default(),
            ),
            Err(MapValidationError::UnreachableExit(GridPos::new(5, 1)))
        );
    }

    #[test]
    fn open_outer_border_is_rejected() {
        let map = parse_map("##.##\n#...#\n#####");

        assert_eq!(
            validate_playable_map(
                &map,
                GridPos::new(1, 1),
                GridPos::new(3, 1),
                &[],
                MapValidationRules::default(),
            ),
            Err(MapValidationError::OpenBorder(GridPos::new(2, 0)))
        );
    }

    #[test]
    fn disconnected_optional_floor_is_rejected_even_if_exit_is_reachable() {
        let map = parse_map("#########\n#...#...#\n#########");

        assert_eq!(
            validate_playable_map(
                &map,
                GridPos::new(1, 1),
                GridPos::new(3, 1),
                &[],
                MapValidationRules::default(),
            ),
            Err(MapValidationError::DisconnectedWalkableArea(GridPos::new(
                5, 1
            )))
        );
    }
}
