mod regional;
mod regional_city;
mod regional_destructible;
mod regional_landmark;
mod regional_loot;
mod regional_population;
mod rooms;
mod validation;

pub use regional::{
    GeneratedRegionalMap, RegionalGenerationError, RegionalMapGenerator, cardinal_passage,
    vertical_passage,
};
pub use regional_city::{
    GeneratedRegionalCityMap, RegionalCityFeature, RegionalCityGenerationError,
    generate_regional_city,
};
pub use regional_destructible::{RegionalDestructibleError, generate_regional_destructibles};
pub use regional_landmark::{
    GeneratedRegionalLandmark, GeneratedRegionalSite, GeneratedRegionalSiteLayout,
    GeneratedRegionalSiteParts, GeneratedRegionalSiteTerminal, RegionSiteEntranceKind,
    RegionalLandmarkError, RegionalLandmarkKind, generate_regional_landmarks,
    generate_regional_site_terminals, generate_regional_sites,
    generate_regional_sites_with_detours, generate_regional_sites_with_salvage,
    generate_regional_sites_with_variety, salvage_yard_is_breached,
};
pub use regional_loot::{
    RegionalLootError, RegionalLootRequest, generate_regional_loot,
    generate_regional_loot_with_scatter, generate_regional_salvage_reward,
};
pub use regional_population::{
    RegionalPopulationError, RegionalPopulationFeatures, generate_regional_encounters,
    generate_regional_encounters_with_roles, generate_regional_population,
};
pub use rooms::{GeneratedMap, GenerationError, Room, RoomsGenerator, RoomsGeneratorConfig};
pub use validation::{
    MapValidationError, MapValidationRules, WalkabilityQuery, validate_interactive_map,
    validate_playable_map,
};
