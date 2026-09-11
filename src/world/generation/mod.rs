mod rooms;
mod validation;

pub use rooms::{GeneratedMap, GenerationError, Room, RoomsGenerator, RoomsGeneratorConfig};
pub use validation::{
    MapValidationError, MapValidationRules, WalkabilityQuery, validate_interactive_map,
    validate_playable_map,
};
