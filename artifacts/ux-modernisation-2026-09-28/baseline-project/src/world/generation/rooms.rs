use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::game::GameRng;
use crate::world::{GridPos, Map, MapBuildError, Terrain};

use super::{MapValidationError, MapValidationRules, validate_playable_map};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RoomsGeneratorConfig {
    pub width: usize,
    pub height: usize,
    pub room_count: usize,
    pub minimum_room_width: usize,
    pub maximum_room_width: usize,
    pub minimum_room_height: usize,
    pub maximum_room_height: usize,
    pub placement_attempts: usize,
    pub validation: MapValidationRules,
}

impl Default for RoomsGeneratorConfig {
    fn default() -> Self {
        Self {
            width: 48,
            height: 32,
            room_count: 10,
            minimum_room_width: 4,
            maximum_room_width: 9,
            minimum_room_height: 4,
            maximum_room_height: 8,
            placement_attempts: 300,
            validation: MapValidationRules::default(),
        }
    }
}

pub struct RoomsGenerator {
    config: RoomsGeneratorConfig,
}

impl RoomsGenerator {
    pub fn new(config: RoomsGeneratorConfig) -> Result<Self, GenerationError> {
        validate_config(config)?;
        Ok(Self { config })
    }

    pub const fn config(&self) -> &RoomsGeneratorConfig {
        &self.config
    }

    pub fn generate(&self, rng: &mut GameRng) -> Result<GeneratedMap, GenerationError> {
        let mut map = Map::filled(self.config.width, self.config.height, Terrain::Wall)
            .map_err(GenerationError::MapBuild)?;
        let mut rooms = Vec::with_capacity(self.config.room_count);

        for _ in 0..self.config.placement_attempts {
            if rooms.len() == self.config.room_count {
                break;
            }

            let candidate = self.random_room(rng)?;
            if rooms
                .iter()
                .any(|room| candidate.overlaps_with_margin(*room))
            {
                continue;
            }

            carve_room(&mut map, candidate)?;
            if let Some(previous) = rooms.last().copied() {
                connect_rooms(&mut map, previous, candidate, rng)?;
            }
            rooms.push(candidate);
        }

        if rooms.len() != self.config.room_count {
            return Err(GenerationError::CouldNotPlaceRooms {
                requested: self.config.room_count,
                placed: rooms.len(),
                attempts: self.config.placement_attempts,
            });
        }

        let Some(first_room) = rooms.first().copied() else {
            return Err(GenerationError::NoRooms);
        };
        let Some(last_room) = rooms.last().copied() else {
            return Err(GenerationError::NoRooms);
        };
        let player_start = first_room.center();
        let exit = last_room.center();

        validate_playable_map(&map, player_start, exit, &[], self.config.validation)
            .map_err(GenerationError::Validation)?;

        Ok(GeneratedMap {
            map,
            player_start,
            exit,
            rooms,
        })
    }

    fn random_room(&self, rng: &mut GameRng) -> Result<Room, GenerationError> {
        let width = random_inclusive(
            rng,
            self.config.minimum_room_width,
            self.config.maximum_room_width,
        )?;
        let height = random_inclusive(
            rng,
            self.config.minimum_room_height,
            self.config.maximum_room_height,
        )?;
        let maximum_x = self.config.width - width - 1;
        let maximum_y = self.config.height - height - 1;

        Ok(Room {
            x: random_inclusive(rng, 1, maximum_x)?,
            y: random_inclusive(rng, 1, maximum_y)?,
            width,
            height,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedMap {
    map: Map,
    player_start: GridPos,
    exit: GridPos,
    rooms: Vec<Room>,
}

impl GeneratedMap {
    /// Accept an authored layout through the same navigation contract as a
    /// procedural level. Validation runs on the final, decorated terrain.
    pub fn from_layout(
        map: Map,
        player_start: GridPos,
        exit: GridPos,
        required_positions: &[GridPos],
        validation: MapValidationRules,
    ) -> Result<Self, MapValidationError> {
        super::validate_interactive_map(&map, player_start, exit, required_positions, validation)?;
        Ok(Self {
            map,
            player_start,
            exit,
            rooms: Vec::new(),
        })
    }

    pub const fn map(&self) -> &Map {
        &self.map
    }

    pub const fn player_start(&self) -> GridPos {
        self.player_start
    }

    pub const fn exit(&self) -> GridPos {
        self.exit
    }

    pub fn rooms(&self) -> &[Room] {
        &self.rooms
    }

    pub fn into_parts(self) -> (Map, GridPos, GridPos) {
        (self.map, self.player_start, self.exit)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Room {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl Room {
    pub fn center(self) -> GridPos {
        GridPos::new(
            (self.x + self.width / 2) as i32,
            (self.y + self.height / 2) as i32,
        )
    }

    fn overlaps_with_margin(self, other: Self) -> bool {
        let self_right = self.x + self.width - 1;
        let self_bottom = self.y + self.height - 1;
        let other_right = other.x + other.width - 1;
        let other_bottom = other.y + other.height - 1;

        self.x <= other_right.saturating_add(1)
            && self_right.saturating_add(1) >= other.x
            && self.y <= other_bottom.saturating_add(1)
            && self_bottom.saturating_add(1) >= other.y
    }
}

fn validate_config(config: RoomsGeneratorConfig) -> Result<(), GenerationError> {
    if config.width > i32::MAX as usize || config.height > i32::MAX as usize {
        return Err(GenerationError::InvalidConfig(
            "map dimensions exceed the grid coordinate type",
        ));
    }
    if config.room_count == 0 {
        return Err(GenerationError::InvalidConfig(
            "room_count must be at least one",
        ));
    }
    if config.minimum_room_width < 2 || config.minimum_room_height < 2 {
        return Err(GenerationError::InvalidConfig(
            "minimum room dimensions must be at least two",
        ));
    }
    if config.minimum_room_width > config.maximum_room_width
        || config.minimum_room_height > config.maximum_room_height
    {
        return Err(GenerationError::InvalidConfig(
            "minimum room dimensions cannot exceed maximum dimensions",
        ));
    }
    if config.maximum_room_width.saturating_add(2) > config.width
        || config.maximum_room_height.saturating_add(2) > config.height
    {
        return Err(GenerationError::InvalidConfig(
            "map must leave a one-tile border around the largest room",
        ));
    }
    if config.placement_attempts < config.room_count {
        return Err(GenerationError::InvalidConfig(
            "placement_attempts cannot be lower than room_count",
        ));
    }
    Ok(())
}

fn random_inclusive(
    rng: &mut GameRng,
    minimum: usize,
    maximum: usize,
) -> Result<usize, GenerationError> {
    rng.usize_inclusive(minimum, maximum)
        .ok_or(GenerationError::InvalidRandomRange { minimum, maximum })
}

fn carve_room(map: &mut Map, room: Room) -> Result<(), GenerationError> {
    for y in room.y..room.y + room.height {
        for x in room.x..room.x + room.width {
            set_floor(map, x, y)?;
        }
    }
    Ok(())
}

fn connect_rooms(
    map: &mut Map,
    first: Room,
    second: Room,
    rng: &mut GameRng,
) -> Result<(), GenerationError> {
    let first_center = first.center();
    let second_center = second.center();

    if rng.next_u64() & 1 == 0 {
        carve_horizontal(map, first_center.x, second_center.x, first_center.y)?;
        carve_vertical(map, first_center.y, second_center.y, second_center.x)?;
    } else {
        carve_vertical(map, first_center.y, second_center.y, first_center.x)?;
        carve_horizontal(map, first_center.x, second_center.x, second_center.y)?;
    }
    Ok(())
}

fn carve_horizontal(
    map: &mut Map,
    start_x: i32,
    end_x: i32,
    y: i32,
) -> Result<(), GenerationError> {
    for x in start_x.min(end_x)..=start_x.max(end_x) {
        map.set_terrain(GridPos::new(x, y), Terrain::Floor)
            .map_err(GenerationError::MapBuild)?;
    }
    Ok(())
}

fn carve_vertical(map: &mut Map, start_y: i32, end_y: i32, x: i32) -> Result<(), GenerationError> {
    for y in start_y.min(end_y)..=start_y.max(end_y) {
        map.set_terrain(GridPos::new(x, y), Terrain::Floor)
            .map_err(GenerationError::MapBuild)?;
    }
    Ok(())
}

fn set_floor(map: &mut Map, x: usize, y: usize) -> Result<(), GenerationError> {
    let position = GridPos::new(
        i32::try_from(x).map_err(|_| GenerationError::CoordinateOverflow)?,
        i32::try_from(y).map_err(|_| GenerationError::CoordinateOverflow)?,
    );
    map.set_terrain(position, Terrain::Floor)
        .map_err(GenerationError::MapBuild)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GenerationError {
    InvalidConfig(&'static str),
    InvalidRandomRange {
        minimum: usize,
        maximum: usize,
    },
    CouldNotPlaceRooms {
        requested: usize,
        placed: usize,
        attempts: usize,
    },
    NoRooms,
    CoordinateOverflow,
    MapBuild(MapBuildError),
    Validation(MapValidationError),
}

impl Display for GenerationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidConfig(message) => {
                write!(formatter, "invalid generator config: {message}")
            }
            Self::InvalidRandomRange { minimum, maximum } => {
                write!(formatter, "invalid random range {minimum}..={maximum}")
            }
            Self::CouldNotPlaceRooms {
                requested,
                placed,
                attempts,
            } => write!(
                formatter,
                "placed {placed} of {requested} rooms after {attempts} attempts"
            ),
            Self::NoRooms => write!(formatter, "generator produced no rooms"),
            Self::CoordinateOverflow => write!(formatter, "generated coordinate does not fit i32"),
            Self::MapBuild(error) => write!(formatter, "could not build generated map: {error}"),
            Self::Validation(error) => write!(formatter, "generated map is not playable: {error}"),
        }
    }
}

impl Error for GenerationError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> RoomsGeneratorConfig {
        RoomsGeneratorConfig {
            width: 36,
            height: 24,
            room_count: 7,
            minimum_room_width: 3,
            maximum_room_width: 7,
            minimum_room_height: 3,
            maximum_room_height: 6,
            placement_attempts: 500,
            validation: MapValidationRules::default(),
        }
    }

    #[test]
    fn same_seed_produces_identical_map_and_landmarks() {
        let generator = match RoomsGenerator::new(test_config()) {
            Ok(generator) => generator,
            Err(error) => panic!("valid generator config was rejected: {error}"),
        };
        let mut first_rng = GameRng::from_seed(2026);
        let mut second_rng = GameRng::from_seed(2026);

        let first = generator.generate(&mut first_rng);
        let second = generator.generate(&mut second_rng);

        assert_eq!(first, second);
    }

    #[test]
    fn generated_maps_are_connected_and_valid_across_many_seeds() {
        let generator = match RoomsGenerator::new(test_config()) {
            Ok(generator) => generator,
            Err(error) => panic!("valid generator config was rejected: {error}"),
        };

        for seed in 0..64 {
            let mut rng = GameRng::from_seed(seed);
            let generated = match generator.generate(&mut rng) {
                Ok(generated) => generated,
                Err(error) => panic!("seed {seed} produced an invalid map: {error}"),
            };
            assert_eq!(generated.rooms().len(), test_config().room_count);
            assert_eq!(
                validate_playable_map(
                    generated.map(),
                    generated.player_start(),
                    generated.exit(),
                    &[],
                    test_config().validation,
                ),
                Ok(()),
                "seed {seed} failed independent validation"
            );
        }
    }

    #[test]
    fn impossible_room_size_is_rejected_before_generation() {
        let mut config = test_config();
        config.maximum_room_width = config.width;

        assert!(matches!(
            RoomsGenerator::new(config),
            Err(GenerationError::InvalidConfig(_))
        ));
    }
}
