mod expedition;
mod id;
mod loader;
mod manifest;
mod regional_world;
mod resolver;

pub use expedition::{
    ExpandedWorldDefinition, ExpeditionCatalog, ExpeditionDefinition, ExpeditionDefinitionError,
    ExpeditionId, FacilityDefinition, FacilityMaterialSpawn, GeneratedZoneDefinition,
    PopulationGroupDefinition, ZoneDefinition,
};
pub use id::{ContentId, ContentIdError, PackageId, PackageIdError};
pub use loader::{ContentLoadError, ContentLoader, LoadedContent};
pub use manifest::{DependencySpec, ManifestError, PackageManifest};
pub use regional_world::{
    MAX_REGION_BIOME_RULES, MAX_REGION_DESTRUCTIBLES, MAX_REGION_DESTRUCTION_COST,
    MAX_REGION_MAP_SIDE, MAX_REGION_PATCH_RADIUS, MAX_REGION_POPULATION_ACTORS,
    MAX_REGION_POPULATION_ROLLS, MAX_REGION_POPULATION_RULES, MAX_REGION_PROVINCE_SIZE,
    MAX_REGION_SITE_SIDE, MAX_REGION_SITES, MAX_REGION_TERRAIN_PATCHES, MAX_REGION_TERRAIN_RULES,
    MAX_REGION_VERTICAL_LINKS, MAX_REGIONAL_WORLD_DEPTH, MAX_REGIONAL_WORLD_SIDE,
    MIN_REGION_MAP_SIDE, RegionBiomeRule, RegionBounds, RegionCoord, RegionDescriptor,
    RegionDestructibleProfile, RegionDirection, RegionLandmarkProfile, RegionLootProfile,
    RegionMapSize, RegionPopulationProfile, RegionPopulationRule, RegionSiteEntranceProfile,
    RegionSiteProfile, RegionSiteSecurityProfile, RegionSiteTerminalProfile, RegionTerrain,
    RegionTerrainProfile, RegionTerrainRule, RegionThreatProfile, RegionVerticalDirection,
    RegionVerticalLink, RegionalWorldCatalog, RegionalWorldDefinition, RegionalWorldError,
};
pub use resolver::{PackageResolutionError, resolve_package_order};
