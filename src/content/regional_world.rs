use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::NonZeroU16;

use crate::ai::AiProfile;
use crate::combat::AttackProfile;
use crate::effects::{DamageFalloff, DestructionEffect};
use crate::facility::{
    MAX_NAVIGATION_BEACON_RANGE, SecurityAlarmProfile, SecurityAlarmProfileError,
};
use crate::progression::DefeatReward;
use crate::social::PlayerRelation;
use crate::stats::PrimaryAttributes;
use crate::world::DistanceMetric;

use super::ContentId;
use super::expedition::{
    ClinicDefinition, ExpeditionDefinitionError, MerchantDefinition, PopulationGroupDefinition,
    ResidentDefinition,
};

pub const MAX_REGIONAL_WORLD_SIDE: u32 = 4_096;
pub const MAX_REGIONAL_WORLD_DEPTH: u16 = 255;
pub const MAX_REGION_PROVINCE_SIZE: u16 = 64;
pub const MAX_REGION_BIOME_RULES: usize = 128;
pub const MIN_REGION_MAP_SIDE: u16 = 24;
pub const MAX_REGION_MAP_SIDE: u16 = 256;
pub const MAX_REGION_TERRAIN_RULES: usize = 16;
pub const MAX_REGION_TERRAIN_PATCHES: u16 = 128;
pub const MAX_REGION_PATCH_RADIUS: u16 = 32;
pub const MAX_REGION_POPULATION_RULES: usize = 32;
pub const MAX_REGION_POPULATION_ROLLS: u16 = 32;
pub const MAX_REGION_POPULATION_ACTORS: u16 = 128;
pub const MAX_REGION_LOOT_DRAWS: u16 = 16;
pub const MAX_REGION_LANDMARKS: u16 = 24;
pub const MAX_REGION_SITES: u16 = 4;
pub const MAX_REGION_SITE_SIDE: u16 = 15;
pub const MAX_REGION_SITE_TERMINAL_RECORDS: usize = 32;
pub const MAX_REGION_VERTICAL_LINKS: usize = 1_024;
pub const MAX_REGION_CITIES: usize = 64;
pub const MAX_REGION_CITY_RESIDENTS: usize = 32;
pub const MAX_REGION_DESTRUCTIBLES: u16 = 32;
pub const MAX_REGION_DESTRUCTION_COST: u16 = 8;

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct RegionCoord {
    pub x: i32,
    pub y: i32,
    pub depth: u16,
}

impl RegionCoord {
    pub const fn new(x: i32, y: i32, depth: u16) -> Self {
        Self { x, y, depth }
    }

    pub fn step(self, direction: RegionDirection) -> Option<Self> {
        let (delta_x, delta_y) = direction.delta();
        Some(Self {
            x: self.x.checked_add(delta_x)?,
            y: self.y.checked_add(delta_y)?,
            depth: self.depth,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionDirection {
    North,
    East,
    South,
    West,
}

impl RegionDirection {
    pub const fn delta(self) -> (i32, i32) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }

    pub const fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RegionVerticalDirection {
    Up,
    Down,
}

impl RegionVerticalDirection {
    pub const fn opposite(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RegionVerticalLink {
    upper: RegionCoord,
    lower: RegionCoord,
}

impl RegionVerticalLink {
    pub fn new(upper: RegionCoord, lower: RegionCoord) -> Result<Self, RegionalWorldError> {
        if upper.x != lower.x
            || upper.y != lower.y
            || upper.depth.checked_add(1) != Some(lower.depth)
        {
            return Err(RegionalWorldError::InvalidVerticalLink);
        }
        Ok(Self { upper, lower })
    }

    pub const fn upper(self) -> RegionCoord {
        self.upper
    }

    pub const fn lower(self) -> RegionCoord {
        self.lower
    }

    pub const fn endpoint(self, direction: RegionVerticalDirection) -> RegionCoord {
        match direction {
            RegionVerticalDirection::Up => self.lower,
            RegionVerticalDirection::Down => self.upper,
        }
    }

    pub const fn destination(self, direction: RegionVerticalDirection) -> RegionCoord {
        match direction {
            RegionVerticalDirection::Up => self.upper,
            RegionVerticalDirection::Down => self.lower,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegionMapSize {
    width: u16,
    height: u16,
}

impl RegionMapSize {
    pub fn new(width: u16, height: u16) -> Result<Self, RegionalWorldError> {
        if width < MIN_REGION_MAP_SIDE || height < MIN_REGION_MAP_SIDE {
            return Err(RegionalWorldError::LocalMapTooSmall);
        }
        if width > MAX_REGION_MAP_SIDE || height > MAX_REGION_MAP_SIDE {
            return Err(RegionalWorldError::LocalMapTooLarge);
        }
        Ok(Self { width, height })
    }

    pub const fn width(self) -> u16 {
        self.width
    }

    pub const fn height(self) -> u16 {
        self.height
    }
}

/// Semantic terrain shared by deterministic generation and presentation.
/// Movement and vision semantics remain owned by the headless generator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegionTerrain {
    Gravel,
    Grass,
    Scrub,
    Mud,
    ShallowWater,
    DeepWater,
    Tree,
    Boulder,
    RuinFloor,
    RuinWall,
}

impl RegionTerrain {
    pub const fn is_walkable(self) -> bool {
        matches!(
            self,
            Self::Gravel
                | Self::Grass
                | Self::Scrub
                | Self::Mud
                | Self::ShallowWater
                | Self::RuinFloor
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegionTerrainRule {
    terrain: RegionTerrain,
    weight: u32,
}

impl RegionTerrainRule {
    pub fn new(terrain: RegionTerrain, weight: u32) -> Result<Self, RegionalWorldError> {
        if weight == 0 {
            return Err(RegionalWorldError::ZeroTerrainWeight);
        }
        Ok(Self { terrain, weight })
    }

    pub const fn terrain(self) -> RegionTerrain {
        self.terrain
    }

    pub const fn weight(self) -> u32 {
        self.weight
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegionTerrainProfile {
    ground: RegionTerrain,
    patch_count: u16,
    minimum_patch_radius: u16,
    maximum_patch_radius: u16,
    features: Vec<RegionTerrainRule>,
}

impl RegionTerrainProfile {
    pub fn new(
        ground: RegionTerrain,
        patch_count: u16,
        minimum_patch_radius: u16,
        maximum_patch_radius: u16,
        features: Vec<RegionTerrainRule>,
    ) -> Result<Self, RegionalWorldError> {
        if !ground.is_walkable() {
            return Err(RegionalWorldError::BlockedBaseTerrain);
        }
        if patch_count > MAX_REGION_TERRAIN_PATCHES {
            return Err(RegionalWorldError::TerrainPatchBudgetExceeded);
        }
        if features.len() > MAX_REGION_TERRAIN_RULES {
            return Err(RegionalWorldError::TerrainRuleBudgetExceeded);
        }
        if patch_count == 0 {
            if !features.is_empty() {
                return Err(RegionalWorldError::UnusedTerrainRules);
            }
        } else if features.is_empty() {
            return Err(RegionalWorldError::MissingTerrainRules);
        }
        if patch_count > 0
            && (minimum_patch_radius == 0
                || minimum_patch_radius > maximum_patch_radius
                || maximum_patch_radius > MAX_REGION_PATCH_RADIUS)
        {
            return Err(RegionalWorldError::InvalidTerrainPatchRadius);
        }
        let mut terrain = BTreeSet::new();
        for feature in &features {
            if !terrain.insert(feature.terrain) {
                return Err(RegionalWorldError::DuplicateTerrain(feature.terrain));
            }
        }
        Ok(Self {
            ground,
            patch_count,
            minimum_patch_radius,
            maximum_patch_radius,
            features,
        })
    }

    pub const fn ground(&self) -> RegionTerrain {
        self.ground
    }

    pub const fn patch_count(&self) -> u16 {
        self.patch_count
    }

    pub const fn minimum_patch_radius(&self) -> u16 {
        self.minimum_patch_radius
    }

    pub const fn maximum_patch_radius(&self) -> u16 {
        self.maximum_patch_radius
    }

    pub fn features(&self) -> &[RegionTerrainRule] {
        &self.features
    }
}

/// One weighted actor group available to a regional biome. The existing
/// population definition owns combat and AI validation; this wrapper adds the
/// variable count and selection weight needed by procedural regions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegionPopulationRule {
    weight: u32,
    minimum_count: u16,
    group: PopulationGroupDefinition,
}

impl RegionPopulationRule {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        weight: u32,
        minimum_count: u16,
        maximum_count: u16,
        minimum_passage_distance: u16,
        maximum_integrity: u16,
        attack: AttackProfile,
        ai: AiProfile,
        defeat_reward: Option<DefeatReward>,
    ) -> Result<Self, RegionalWorldError> {
        if weight == 0 {
            return Err(RegionalWorldError::ZeroPopulationWeight);
        }
        if minimum_count == 0 || minimum_count > maximum_count {
            return Err(RegionalWorldError::InvalidPopulationCountRange);
        }
        let group = PopulationGroupDefinition::new(
            maximum_count,
            minimum_passage_distance,
            maximum_integrity,
            attack,
            ai,
            defeat_reward,
        )
        .map_err(|error| RegionalWorldError::InvalidPopulation(Box::new(error)))?;
        Ok(Self {
            weight,
            minimum_count,
            group,
        })
    }

    pub fn with_primary_attributes(
        mut self,
        attributes: PrimaryAttributes,
    ) -> Result<Self, RegionalWorldError> {
        self.group = self
            .group
            .with_primary_attributes(attributes)
            .map_err(|error| RegionalWorldError::InvalidPopulation(Box::new(error)))?;
        Ok(self)
    }

    pub fn with_body_profile(mut self, body: crate::stats::BodyProfile) -> Self {
        self.group = self.group.with_body_profile(body);
        self
    }

    pub fn with_body_components(
        mut self,
        components: impl IntoIterator<Item = crate::entity::BodyComponentProfile>,
    ) -> Self {
        self.group = self.group.with_body_components(components);
        self
    }

    pub fn with_electronic_system(
        mut self,
        profile: crate::electronic_warfare::ElectronicSystemProfile,
    ) -> Self {
        self.group = self.group.with_electronic_system(profile);
        self
    }

    pub fn with_player_relation(mut self, relation: PlayerRelation) -> Self {
        self.group = self.group.with_player_relation(relation);
        self
    }

    pub fn with_tags(mut self, tags: Vec<ContentId>) -> Result<Self, RegionalWorldError> {
        self.group = self
            .group
            .with_tags(tags)
            .map_err(|error| RegionalWorldError::InvalidPopulation(Box::new(error)))?;
        Ok(self)
    }

    pub const fn weight(&self) -> u32 {
        self.weight
    }

    pub const fn minimum_count(&self) -> u16 {
        self.minimum_count
    }

    pub const fn maximum_count(&self) -> u16 {
        self.group.count()
    }

    pub const fn minimum_passage_distance(&self) -> u16 {
        self.group.minimum_entrance_distance()
    }

    pub const fn maximum_integrity(&self) -> u16 {
        self.group.maximum_integrity()
    }

    pub const fn attack(&self) -> AttackProfile {
        self.group.attack()
    }

    pub const fn ai(&self) -> AiProfile {
        self.group.ai()
    }

    pub const fn defeat_reward(&self) -> Option<DefeatReward> {
        self.group.defeat_reward()
    }

    pub const fn primary_attributes(&self) -> Option<PrimaryAttributes> {
        self.group.primary_attributes()
    }

    pub const fn body_profile(&self) -> Option<crate::stats::BodyProfile> {
        self.group.body_profile()
    }

    pub fn body_components(&self) -> &[crate::entity::BodyComponentProfile] {
        self.group.body_components()
    }

    pub const fn electronic_system(
        &self,
    ) -> Option<crate::electronic_warfare::ElectronicSystemProfile> {
        self.group.electronic_system()
    }

    pub const fn player_relation(&self) -> PlayerRelation {
        self.group.player_relation()
    }

    pub fn tags(&self) -> &[ContentId] {
        self.group.tags()
    }
}

/// Bounded weighted draws keep different regions of one biome from receiving
/// an identical roster while remaining deterministic for a given region seed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RegionPopulationProfile {
    minimum_group_rolls: u16,
    maximum_group_rolls: u16,
    rules: Vec<RegionPopulationRule>,
}

impl RegionPopulationProfile {
    pub fn new(
        minimum_group_rolls: u16,
        maximum_group_rolls: u16,
        rules: Vec<RegionPopulationRule>,
    ) -> Result<Self, RegionalWorldError> {
        if minimum_group_rolls > maximum_group_rolls {
            return Err(RegionalWorldError::InvalidPopulationRollRange);
        }
        if maximum_group_rolls > MAX_REGION_POPULATION_ROLLS {
            return Err(RegionalWorldError::PopulationRollBudgetExceeded);
        }
        if rules.len() > MAX_REGION_POPULATION_RULES {
            return Err(RegionalWorldError::PopulationRuleBudgetExceeded);
        }
        if maximum_group_rolls == 0 {
            if !rules.is_empty() {
                return Err(RegionalWorldError::UnusedPopulationRules);
            }
        } else if rules.is_empty() {
            return Err(RegionalWorldError::MissingPopulationRules);
        }
        let largest_group = rules
            .iter()
            .map(RegionPopulationRule::maximum_count)
            .max()
            .unwrap_or(0);
        if maximum_group_rolls
            .checked_mul(largest_group)
            .is_none_or(|actors| actors > MAX_REGION_POPULATION_ACTORS)
        {
            return Err(RegionalWorldError::PopulationActorBudgetExceeded);
        }
        Ok(Self {
            minimum_group_rolls,
            maximum_group_rolls,
            rules,
        })
    }

    pub const fn minimum_group_rolls(&self) -> u16 {
        self.minimum_group_rolls
    }

    pub const fn maximum_group_rolls(&self) -> u16 {
        self.maximum_group_rolls
    }

    pub fn rules(&self) -> &[RegionPopulationRule] {
        &self.rules
    }

    pub const fn is_empty(&self) -> bool {
        self.maximum_group_rolls == 0
    }

    fn maximum_actor_count(&self) -> u16 {
        self.maximum_group_rolls.saturating_mul(
            self.rules
                .iter()
                .map(RegionPopulationRule::maximum_count)
                .max()
                .unwrap_or(0),
        )
    }
}

/// Bounded stationary entities whose destruction releases a shared radial
/// effect. Placement is a separate deterministic layer so adding or tuning
/// them cannot reroll terrain, hostiles, sites or loot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegionDestructibleProfile {
    minimum_count: u16,
    maximum_count: u16,
    minimum_passage_distance: u16,
    maximum_integrity: u16,
    destruction_effect: DestructionEffect,
}

impl RegionDestructibleProfile {
    pub fn new(
        minimum_count: u16,
        maximum_count: u16,
        minimum_passage_distance: u16,
        maximum_integrity: u16,
        destruction_effect: DestructionEffect,
    ) -> Result<Self, RegionalWorldError> {
        if minimum_count > maximum_count
            || maximum_count == 0
            || maximum_count > MAX_REGION_DESTRUCTIBLES
        {
            return Err(RegionalWorldError::InvalidDestructibleCountRange);
        }
        if maximum_integrity == 0 {
            return Err(RegionalWorldError::ZeroDestructibleIntegrity);
        }
        let explosion = destruction_effect.explosion();
        let propagation = explosion.propagation_policy;
        if explosion.maximum_cost == 0
            || explosion.maximum_cost > MAX_REGION_DESTRUCTION_COST
            || explosion.damage.amount == 0
            || propagation.floor_cost.is_none_or(|cost| cost == 0)
            || propagation.shallow_water_cost.is_some_and(|cost| cost == 0)
            || propagation.deep_water_cost.is_some_and(|cost| cost == 0)
            || propagation.wall_cost.is_some_and(|cost| cost == 0)
            || matches!(
                destruction_effect.explosion().falloff,
                DamageFalloff::PerPropagationCost(0)
            )
        {
            return Err(RegionalWorldError::InvalidDestructionEffect);
        }
        Ok(Self {
            minimum_count,
            maximum_count,
            minimum_passage_distance,
            maximum_integrity,
            destruction_effect,
        })
    }

    pub const fn count_range(&self) -> (u16, u16) {
        (self.minimum_count, self.maximum_count)
    }

    pub const fn minimum_passage_distance(&self) -> u16 {
        self.minimum_passage_distance
    }

    pub const fn maximum_integrity(&self) -> u16 {
        self.maximum_integrity
    }

    pub const fn destruction_effect(&self) -> &DestructionEffect {
        &self.destruction_effect
    }

    pub fn uses_distinct_water_propagation(&self) -> bool {
        self.destruction_effect
            .explosion()
            .propagation_policy
            .uses_distinct_water_costs()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegionLootProfile {
    table: ContentId,
    source: ContentId,
    minimum_draws: u16,
    maximum_draws: u16,
}

impl RegionLootProfile {
    pub fn new(
        table: ContentId,
        source: ContentId,
        minimum_draws: u16,
        maximum_draws: u16,
    ) -> Result<Self, RegionalWorldError> {
        if minimum_draws == 0
            || minimum_draws > maximum_draws
            || maximum_draws > MAX_REGION_LOOT_DRAWS
        {
            return Err(RegionalWorldError::InvalidLootDrawRange);
        }
        Ok(Self {
            table,
            source,
            minimum_draws,
            maximum_draws,
        })
    }

    pub const fn table(&self) -> &ContentId {
        &self.table
    }

    pub const fn source(&self) -> &ContentId {
        &self.source
    }

    pub const fn minimum_draws(&self) -> u16 {
        self.minimum_draws
    }

    pub const fn maximum_draws(&self) -> u16 {
        self.maximum_draws
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RegionLandmarkProfile {
    minimum_caches: u16,
    maximum_caches: u16,
    minimum_threat_camps: u16,
    maximum_threat_camps: u16,
    minimum_passage_distance: u16,
}

impl RegionLandmarkProfile {
    pub fn new(
        minimum_caches: u16,
        maximum_caches: u16,
        minimum_threat_camps: u16,
        maximum_threat_camps: u16,
        minimum_passage_distance: u16,
    ) -> Result<Self, RegionalWorldError> {
        if minimum_caches > maximum_caches
            || minimum_threat_camps > maximum_threat_camps
            || maximum_caches.saturating_add(maximum_threat_camps) > MAX_REGION_LANDMARKS
        {
            return Err(RegionalWorldError::InvalidLandmarkRange);
        }
        Ok(Self {
            minimum_caches,
            maximum_caches,
            minimum_threat_camps,
            maximum_threat_camps,
            minimum_passage_distance,
        })
    }

    pub const fn cache_range(self) -> (u16, u16) {
        (self.minimum_caches, self.maximum_caches)
    }

    pub const fn threat_camp_range(self) -> (u16, u16) {
        (self.minimum_threat_camps, self.maximum_threat_camps)
    }

    pub const fn minimum_passage_distance(self) -> u16 {
        self.minimum_passage_distance
    }

    pub const fn is_empty(self) -> bool {
        self.maximum_caches == 0 && self.maximum_threat_camps == 0
    }
}

/// Data-defined v23 weights for the interactive entrance of a regional site.
/// A profile containing only zeroes retains the legacy open entrance.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RegionSiteEntranceProfile {
    open_weight: u16,
    closed_door_weight: u16,
    locked_console_weight: u16,
}

impl RegionSiteEntranceProfile {
    pub const fn new(
        open_weight: u16,
        closed_door_weight: u16,
        locked_console_weight: u16,
    ) -> Self {
        Self {
            open_weight,
            closed_door_weight,
            locked_console_weight,
        }
    }

    pub const fn open_weight(self) -> u16 {
        self.open_weight
    }

    pub const fn closed_door_weight(self) -> u16 {
        self.closed_door_weight
    }

    pub const fn locked_console_weight(self) -> u16 {
        self.locked_console_weight
    }

    pub const fn total_weight(self) -> u32 {
        self.open_weight as u32 + self.closed_door_weight as u32 + self.locked_console_weight as u32
    }

    pub const fn is_legacy_open(self) -> bool {
        self.total_weight() == 0
    }
}

/// Optional v22 layout for pairing caches and threat camps inside small,
/// navigable compounds. Counts, dimensions and v23 entrance weights remain
/// content data so mods can tune sites without replacing the placement
/// algorithm.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct RegionSiteProfile {
    minimum_compounds: u16,
    maximum_compounds: u16,
    width: u16,
    height: u16,
    entrances: RegionSiteEntranceProfile,
}

impl Debug for RegionSiteProfile {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug = formatter.debug_struct("RegionSiteProfile");
        debug
            .field("minimum_compounds", &self.minimum_compounds)
            .field("maximum_compounds", &self.maximum_compounds)
            .field("width", &self.width)
            .field("height", &self.height);
        // Omitting the empty v23 extension preserves the serialized debug
        // fingerprint used by v22 suspensions.
        if !self.entrances.is_legacy_open() {
            debug.field("entrances", &self.entrances);
        }
        debug.finish()
    }
}

impl RegionSiteProfile {
    pub fn new(
        minimum_compounds: u16,
        maximum_compounds: u16,
        width: u16,
        height: u16,
    ) -> Result<Self, RegionalWorldError> {
        if minimum_compounds > maximum_compounds || maximum_compounds > MAX_REGION_SITES {
            return Err(RegionalWorldError::InvalidSiteCountRange);
        }
        if maximum_compounds == 0 {
            if width != 0 || height != 0 {
                return Err(RegionalWorldError::UnusedSiteDimensions);
            }
        } else if width < 5
            || height < 5
            || width > MAX_REGION_SITE_SIDE
            || height > MAX_REGION_SITE_SIDE
            || width.is_multiple_of(2)
            || height.is_multiple_of(2)
        {
            return Err(RegionalWorldError::InvalidSiteDimensions);
        }
        Ok(Self {
            minimum_compounds,
            maximum_compounds,
            width,
            height,
            entrances: RegionSiteEntranceProfile::default(),
        })
    }

    pub fn with_entrances(
        mut self,
        entrances: RegionSiteEntranceProfile,
    ) -> Result<Self, RegionalWorldError> {
        if self.is_empty() && !entrances.is_legacy_open() {
            return Err(RegionalWorldError::UnusedSiteEntrances);
        }
        self.entrances = entrances;
        Ok(self)
    }

    pub const fn compound_range(self) -> (u16, u16) {
        (self.minimum_compounds, self.maximum_compounds)
    }

    pub const fn size(self) -> (u16, u16) {
        (self.width, self.height)
    }

    pub const fn entrances(self) -> RegionSiteEntranceProfile {
        self.entrances
    }

    pub const fn without_interactions(mut self) -> Self {
        self.entrances = RegionSiteEntranceProfile::new(0, 0, 0);
        self
    }

    pub const fn is_empty(self) -> bool {
        self.maximum_compounds == 0
    }
}

/// Optional v28 content carried by regional compounds. Terminal count and
/// record selection use their own deterministic stream, so adding archives
/// does not move sites, entrances, actors, loot or security rolls.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegionSiteTerminalProfile {
    minimum_terminals: u16,
    maximum_terminals: u16,
    records: Vec<ContentId>,
}

impl RegionSiteTerminalProfile {
    pub fn new(
        minimum_terminals: u16,
        maximum_terminals: u16,
        records: Vec<ContentId>,
    ) -> Result<Self, RegionalWorldError> {
        if minimum_terminals > maximum_terminals
            || maximum_terminals == 0
            || maximum_terminals > MAX_REGION_SITES
        {
            return Err(RegionalWorldError::InvalidSiteTerminalCountRange);
        }
        if records.is_empty() {
            return Err(RegionalWorldError::MissingSiteTerminalRecords);
        }
        if records.len() > MAX_REGION_SITE_TERMINAL_RECORDS {
            return Err(RegionalWorldError::SiteTerminalRecordBudgetExceeded);
        }
        let mut unique = BTreeSet::new();
        for record in &records {
            if !unique.insert(record) {
                return Err(RegionalWorldError::DuplicateSiteTerminalRecord(
                    record.clone(),
                ));
            }
        }
        Ok(Self {
            minimum_terminals,
            maximum_terminals,
            records,
        })
    }

    pub const fn terminal_range(&self) -> (u16, u16) {
        (self.minimum_terminals, self.maximum_terminals)
    }

    pub fn records(&self) -> &[ContentId] {
        &self.records
    }
}

/// Optional v24+ security policy for locked regional compounds. The generated
/// site supplies the physical sensor and threat-source coordinates; this data
/// controls ownership, perception and response timing without hard-coding a
/// faction or balance values in the generator.
#[derive(Clone, PartialEq, Eq)]
pub struct RegionSiteSecurityProfile {
    owner: ContentId,
    sensor_radius: u16,
    distance_metric: DistanceMetric,
    block_closed_corners: bool,
    alarm_duration_turns: u16,
    reinforcement_delay_turns: u16,
    navigation_signal_range: Option<u16>,
}

impl Debug for RegionSiteSecurityProfile {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut profile = formatter.debug_struct("RegionSiteSecurityProfile");
        profile
            .field("owner", &self.owner)
            .field("sensor_radius", &self.sensor_radius)
            .field("distance_metric", &self.distance_metric)
            .field("block_closed_corners", &self.block_closed_corners)
            .field("alarm_duration_turns", &self.alarm_duration_turns)
            .field("reinforcement_delay_turns", &self.reinforcement_delay_turns);
        if let Some(range) = self.navigation_signal_range {
            profile.field("navigation_signal_range", &range);
        }
        profile.finish()
    }
}

impl RegionSiteSecurityProfile {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        owner: ContentId,
        sensor_radius: u16,
        distance_metric: DistanceMetric,
        block_closed_corners: bool,
        alarm_duration_turns: u16,
        reinforcement_delay_turns: u16,
    ) -> Result<Self, RegionalWorldError> {
        SecurityAlarmProfile::new(
            sensor_radius,
            distance_metric,
            block_closed_corners,
            alarm_duration_turns,
        )
        .and_then(|profile| {
            profile.with_responses(vec![
                crate::facility::SecurityAlarmResponse::CallReinforcements {
                    source: crate::world::GridPos::new(0, 0),
                    delay_turns: reinforcement_delay_turns,
                },
            ])
        })
        .map_err(RegionalWorldError::InvalidSiteSecurityProfile)?;
        Ok(Self {
            owner,
            sensor_radius,
            distance_metric,
            block_closed_corners,
            alarm_duration_turns,
            reinforcement_delay_turns,
            navigation_signal_range: None,
        })
    }

    pub fn with_navigation_signal_range(mut self, range: u16) -> Result<Self, RegionalWorldError> {
        if range == 0 || range > MAX_NAVIGATION_BEACON_RANGE {
            return Err(RegionalWorldError::InvalidSiteNavigationSignalRange);
        }
        self.navigation_signal_range = Some(range);
        Ok(self)
    }

    pub const fn owner(&self) -> &ContentId {
        &self.owner
    }

    pub const fn sensor_radius(&self) -> u16 {
        self.sensor_radius
    }

    pub const fn distance_metric(&self) -> DistanceMetric {
        self.distance_metric
    }

    pub const fn block_closed_corners(&self) -> bool {
        self.block_closed_corners
    }

    pub const fn alarm_duration_turns(&self) -> u16 {
        self.alarm_duration_turns
    }

    pub const fn reinforcement_delay_turns(&self) -> u16 {
        self.reinforcement_delay_turns
    }

    pub const fn navigation_signal_range(&self) -> Option<u16> {
        self.navigation_signal_range
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegionThreatProfile {
    interval_turns: NonZeroU16,
    maximum_active: NonZeroU16,
    maximum_total: NonZeroU16,
    actor: PopulationGroupDefinition,
}

impl RegionThreatProfile {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        interval_turns: NonZeroU16,
        maximum_active: NonZeroU16,
        maximum_total: NonZeroU16,
        maximum_integrity: u16,
        attack: AttackProfile,
        ai: AiProfile,
    ) -> Result<Self, RegionalWorldError> {
        if maximum_active > maximum_total {
            return Err(RegionalWorldError::InvalidThreatLimits);
        }
        let actor = PopulationGroupDefinition::new(
            1,
            0,
            maximum_integrity,
            attack,
            ai,
            Some(DefeatReward::summoned(0, 0)),
        )
        .map_err(|error| RegionalWorldError::InvalidPopulation(Box::new(error)))?;
        Ok(Self {
            interval_turns,
            maximum_active,
            maximum_total,
            actor,
        })
    }

    pub fn with_primary_attributes(
        mut self,
        attributes: PrimaryAttributes,
    ) -> Result<Self, RegionalWorldError> {
        self.actor = self
            .actor
            .with_primary_attributes(attributes)
            .map_err(|error| RegionalWorldError::InvalidPopulation(Box::new(error)))?;
        Ok(self)
    }

    pub fn with_body_profile(mut self, body: crate::stats::BodyProfile) -> Self {
        self.actor = self.actor.with_body_profile(body);
        self
    }

    pub fn with_body_components(
        mut self,
        components: impl IntoIterator<Item = crate::entity::BodyComponentProfile>,
    ) -> Self {
        self.actor = self.actor.with_body_components(components);
        self
    }

    pub fn with_electronic_system(
        mut self,
        profile: crate::electronic_warfare::ElectronicSystemProfile,
    ) -> Self {
        self.actor = self.actor.with_electronic_system(profile);
        self
    }

    pub fn with_player_relation(mut self, relation: PlayerRelation) -> Self {
        self.actor = self.actor.with_player_relation(relation);
        self
    }

    pub fn with_tags(mut self, tags: Vec<ContentId>) -> Result<Self, RegionalWorldError> {
        self.actor = self
            .actor
            .with_tags(tags)
            .map_err(|error| RegionalWorldError::InvalidPopulation(Box::new(error)))?;
        Ok(self)
    }

    pub const fn interval_turns(&self) -> u16 {
        self.interval_turns.get()
    }

    pub const fn maximum_active(&self) -> u16 {
        self.maximum_active.get()
    }

    pub const fn maximum_total(&self) -> u16 {
        self.maximum_total.get()
    }

    pub const fn maximum_integrity(&self) -> u16 {
        self.actor.maximum_integrity()
    }

    pub const fn attack(&self) -> AttackProfile {
        self.actor.attack()
    }

    pub const fn ai(&self) -> AiProfile {
        self.actor.ai()
    }

    pub const fn primary_attributes(&self) -> Option<PrimaryAttributes> {
        self.actor.primary_attributes()
    }

    pub const fn body_profile(&self) -> Option<crate::stats::BodyProfile> {
        self.actor.body_profile()
    }

    pub fn body_components(&self) -> &[crate::entity::BodyComponentProfile] {
        self.actor.body_components()
    }

    pub const fn electronic_system(
        &self,
    ) -> Option<crate::electronic_warfare::ElectronicSystemProfile> {
        self.actor.electronic_system()
    }

    pub const fn player_relation(&self) -> PlayerRelation {
        self.actor.player_relation()
    }

    pub fn tags(&self) -> &[ContentId] {
        self.actor.tags()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RegionBounds {
    minimum_x: i32,
    maximum_x: i32,
    minimum_y: i32,
    maximum_y: i32,
    maximum_depth: u16,
}

impl RegionBounds {
    pub fn new(
        minimum_x: i32,
        maximum_x: i32,
        minimum_y: i32,
        maximum_y: i32,
        maximum_depth: u16,
    ) -> Result<Self, RegionalWorldError> {
        if minimum_x > maximum_x || minimum_y > maximum_y {
            return Err(RegionalWorldError::InvertedBounds);
        }
        let width = i64::from(maximum_x) - i64::from(minimum_x) + 1;
        let height = i64::from(maximum_y) - i64::from(minimum_y) + 1;
        if width > i64::from(MAX_REGIONAL_WORLD_SIDE)
            || height > i64::from(MAX_REGIONAL_WORLD_SIDE)
            || maximum_depth > MAX_REGIONAL_WORLD_DEPTH
        {
            return Err(RegionalWorldError::WorldBudgetExceeded);
        }
        Ok(Self {
            minimum_x,
            maximum_x,
            minimum_y,
            maximum_y,
            maximum_depth,
        })
    }

    pub const fn minimum_x(self) -> i32 {
        self.minimum_x
    }

    pub const fn maximum_x(self) -> i32 {
        self.maximum_x
    }

    pub const fn minimum_y(self) -> i32 {
        self.minimum_y
    }

    pub const fn maximum_y(self) -> i32 {
        self.maximum_y
    }

    pub const fn maximum_depth(self) -> u16 {
        self.maximum_depth
    }

    pub const fn contains(self, coordinate: RegionCoord) -> bool {
        coordinate.x >= self.minimum_x
            && coordinate.x <= self.maximum_x
            && coordinate.y >= self.minimum_y
            && coordinate.y <= self.maximum_y
            && coordinate.depth <= self.maximum_depth
    }

    pub fn addressable_region_count(self) -> u64 {
        let width = (i64::from(self.maximum_x) - i64::from(self.minimum_x) + 1) as u64;
        let height = (i64::from(self.maximum_y) - i64::from(self.minimum_y) + 1) as u64;
        width * height * (u64::from(self.maximum_depth) + 1)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct RegionBiomeRule {
    biome: ContentId,
    weight: u32,
    minimum_depth: u16,
    maximum_depth: Option<u16>,
    terrain: RegionTerrainProfile,
    population: RegionPopulationProfile,
    encounters: RegionPopulationProfile,
    loot: Option<RegionLootProfile>,
    salvage_loot: Option<RegionLootProfile>,
    landmarks: RegionLandmarkProfile,
    sites: RegionSiteProfile,
    site_terminals: Option<RegionSiteTerminalProfile>,
    site_security: Option<RegionSiteSecurityProfile>,
    threats: Option<RegionThreatProfile>,
    destructibles: Option<RegionDestructibleProfile>,
}

// Empty population metadata is omitted so the v15 world fingerprint remains
// byte-for-byte reproducible after the v16 field was introduced.
impl Debug for RegionBiomeRule {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut biome = formatter.debug_struct("RegionBiomeRule");
        biome
            .field("biome", &self.biome)
            .field("weight", &self.weight)
            .field("minimum_depth", &self.minimum_depth)
            .field("maximum_depth", &self.maximum_depth)
            .field("terrain", &self.terrain);
        if !self.population.is_empty() {
            biome.field("population", &self.population);
        }
        if !self.encounters.is_empty() {
            biome.field("encounters", &self.encounters);
        }
        if let Some(loot) = &self.loot {
            biome.field("loot", loot);
        }
        if let Some(loot) = &self.salvage_loot {
            biome.field("salvage_loot", loot);
        }
        if !self.landmarks.is_empty() {
            biome.field("landmarks", &self.landmarks);
        }
        if !self.sites.is_empty() {
            biome.field("sites", &self.sites);
        }
        if let Some(terminals) = &self.site_terminals {
            biome.field("site_terminals", terminals);
        }
        if let Some(security) = &self.site_security {
            biome.field("site_security", security);
        }
        if let Some(threats) = &self.threats {
            biome.field("threats", threats);
        }
        if let Some(destructibles) = &self.destructibles {
            biome.field("destructibles", destructibles);
        }
        biome.finish()
    }
}

impl RegionBiomeRule {
    pub fn new(
        biome: ContentId,
        weight: u32,
        minimum_depth: u16,
        maximum_depth: Option<u16>,
        terrain: RegionTerrainProfile,
    ) -> Result<Self, RegionalWorldError> {
        if weight == 0 {
            return Err(RegionalWorldError::ZeroBiomeWeight);
        }
        if maximum_depth.is_some_and(|maximum| maximum < minimum_depth) {
            return Err(RegionalWorldError::InvertedBiomeDepth);
        }
        Ok(Self {
            biome,
            weight,
            minimum_depth,
            maximum_depth,
            terrain,
            population: RegionPopulationProfile::default(),
            encounters: RegionPopulationProfile::default(),
            loot: None,
            salvage_loot: None,
            landmarks: RegionLandmarkProfile::default(),
            sites: RegionSiteProfile::default(),
            site_terminals: None,
            site_security: None,
            threats: None,
            destructibles: None,
        })
    }

    pub fn with_population(mut self, population: RegionPopulationProfile) -> Self {
        self.population = population;
        self
    }

    /// Adds v21 encounter groups. They use a separate deterministic random
    /// stream and may be anchored to landmarks without changing the historical
    /// biome population generated by older suspensions.
    pub fn with_encounters(mut self, encounters: RegionPopulationProfile) -> Self {
        self.encounters = encounters;
        self
    }

    pub fn with_loot(mut self, loot: RegionLootProfile) -> Self {
        self.loot = Some(loot);
        self
    }

    pub fn with_salvage_loot(mut self, loot: RegionLootProfile) -> Self {
        self.salvage_loot = Some(loot);
        self
    }

    pub fn with_landmarks(mut self, landmarks: RegionLandmarkProfile) -> Self {
        self.landmarks = landmarks;
        self
    }

    pub fn with_sites(mut self, sites: RegionSiteProfile) -> Self {
        self.sites = sites;
        self
    }

    pub fn with_site_security(mut self, security: RegionSiteSecurityProfile) -> Self {
        self.site_security = Some(security);
        self
    }

    pub fn with_site_terminals(mut self, terminals: RegionSiteTerminalProfile) -> Self {
        self.site_terminals = Some(terminals);
        self
    }

    pub fn with_threats(mut self, threats: RegionThreatProfile) -> Self {
        self.threats = Some(threats);
        self
    }

    pub fn with_destructibles(mut self, destructibles: RegionDestructibleProfile) -> Self {
        self.destructibles = Some(destructibles);
        self
    }

    pub const fn biome(&self) -> &ContentId {
        &self.biome
    }

    pub const fn weight(&self) -> u32 {
        self.weight
    }

    pub const fn minimum_depth(&self) -> u16 {
        self.minimum_depth
    }

    pub const fn maximum_depth(&self) -> Option<u16> {
        self.maximum_depth
    }

    pub const fn terrain(&self) -> &RegionTerrainProfile {
        &self.terrain
    }

    pub const fn population(&self) -> &RegionPopulationProfile {
        &self.population
    }

    pub const fn encounters(&self) -> &RegionPopulationProfile {
        &self.encounters
    }

    pub const fn loot(&self) -> Option<&RegionLootProfile> {
        self.loot.as_ref()
    }

    pub const fn salvage_loot(&self) -> Option<&RegionLootProfile> {
        self.salvage_loot.as_ref()
    }

    pub const fn landmarks(&self) -> RegionLandmarkProfile {
        self.landmarks
    }

    pub const fn sites(&self) -> RegionSiteProfile {
        self.sites
    }

    pub const fn site_security(&self) -> Option<&RegionSiteSecurityProfile> {
        self.site_security.as_ref()
    }

    pub const fn site_terminals(&self) -> Option<&RegionSiteTerminalProfile> {
        self.site_terminals.as_ref()
    }

    pub const fn threats(&self) -> Option<&RegionThreatProfile> {
        self.threats.as_ref()
    }

    pub const fn destructibles(&self) -> Option<&RegionDestructibleProfile> {
        self.destructibles.as_ref()
    }

    fn supports(&self, depth: u16) -> bool {
        depth >= self.minimum_depth && self.maximum_depth.is_none_or(|maximum| depth <= maximum)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegionDescriptor {
    pub coordinate: RegionCoord,
    pub seed: u64,
    pub biome: ContentId,
}

/// Authored topology family for one settlement. Each future layer may select
/// a genuinely different layout without changing the generic city services.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionCityLayout {
    MaintenanceSpine,
    CoolantRings,
    DissonantLattice,
    RecursiveBloom,
    ProcessRuin,
}

impl RegionCityLayout {
    pub const fn minimum_size(self) -> (u16, u16) {
        match self {
            Self::MaintenanceSpine => (48, 40),
            Self::CoolantRings => (64, 56),
            Self::DissonantLattice => (112, 56),
            Self::RecursiveBloom => (88, 88),
            Self::ProcessRuin => (104, 80),
        }
    }

    pub const fn supports(self, size: RegionMapSize) -> bool {
        let (minimum_width, minimum_height) = self.minimum_size();
        size.width >= minimum_width && size.height >= minimum_height
    }
}

/// One persistent settlement anchored to an atlas coordinate. Merchant,
/// clinic and resident definitions are deliberately independent from quests:
/// arriving in the city is sufficient to meet and use these NPCs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegionCityDefinition {
    id: ContentId,
    coordinate: RegionCoord,
    name: String,
    kind: ContentId,
    map_size: RegionMapSize,
    layout: RegionCityLayout,
    merchant: MerchantDefinition,
    clinic: ClinicDefinition,
    residents: Vec<ResidentDefinition>,
}

impl RegionCityDefinition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: ContentId,
        coordinate: RegionCoord,
        name: String,
        kind: ContentId,
        map_size: RegionMapSize,
        layout: RegionCityLayout,
        merchant: MerchantDefinition,
        clinic: ClinicDefinition,
        residents: Vec<ResidentDefinition>,
    ) -> Result<Self, RegionalWorldError> {
        if name.trim().is_empty() {
            return Err(RegionalWorldError::EmptyCityName);
        }
        if residents.is_empty() || residents.len() > MAX_REGION_CITY_RESIDENTS {
            return Err(RegionalWorldError::InvalidCityResidentCount);
        }
        if !layout.supports(map_size) {
            return Err(RegionalWorldError::CityLayoutTooSmall(layout));
        }
        if !merchant
            .offers
            .iter()
            .any(|offer| offer.available_at_depth(coordinate.depth))
        {
            return Err(RegionalWorldError::InvalidCity(Box::new(
                ExpeditionDefinitionError::InvalidMerchant,
            )));
        }
        let mut provider_positions = BTreeSet::from([merchant.position, clinic.work_position]);
        for resident in &residents {
            if !provider_positions.insert(resident.residence_position) {
                return Err(RegionalWorldError::DuplicateCityProviderPosition(
                    resident.residence_position,
                ));
            }
        }
        if provider_positions.len() != residents.len().saturating_add(2) {
            return Err(RegionalWorldError::DuplicateCityProviderPosition(
                clinic.work_position,
            ));
        }
        Ok(Self {
            id,
            coordinate,
            name,
            kind,
            map_size,
            layout,
            merchant,
            clinic,
            residents,
        })
    }

    pub const fn id(&self) -> &ContentId {
        &self.id
    }

    pub const fn coordinate(&self) -> RegionCoord {
        self.coordinate
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub const fn kind(&self) -> &ContentId {
        &self.kind
    }

    pub const fn map_size(&self) -> RegionMapSize {
        self.map_size
    }

    pub const fn layout(&self) -> RegionCityLayout {
        self.layout
    }

    pub const fn merchant(&self) -> &MerchantDefinition {
        &self.merchant
    }

    pub const fn clinic(&self) -> &ClinicDefinition {
        &self.clinic
    }

    pub fn residents(&self) -> &[ResidentDefinition] {
        &self.residents
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct RegionalWorldDefinition {
    id: ContentId,
    bounds: RegionBounds,
    province_size: u16,
    local_map_size: RegionMapSize,
    surface_map_size: Option<RegionMapSize>,
    biomes: Vec<RegionBiomeRule>,
    vertical_links: Vec<RegionVerticalLink>,
    cities: Vec<RegionCityDefinition>,
}

impl Debug for RegionalWorldDefinition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug = formatter.debug_struct("RegionalWorldDefinition");
        debug
            .field("id", &self.id)
            .field("bounds", &self.bounds)
            .field("province_size", &self.province_size)
            .field("local_map_size", &self.local_map_size)
            .field("biomes", &self.biomes);
        if let Some(surface_map_size) = self.surface_map_size {
            debug.field("surface_map_size", &surface_map_size);
        }
        // Empty link metadata must keep the exact pre-v29 debug shape because
        // legacy suspension fingerprints are intentionally replay-compatible.
        if !self.vertical_links.is_empty() {
            debug.field("vertical_links", &self.vertical_links);
        }
        // As with links, an empty collection preserves every pre-city world
        // fingerprint byte-for-byte.
        if !self.cities.is_empty() {
            debug.field("cities", &self.cities);
        }
        debug.finish()
    }
}

impl RegionalWorldDefinition {
    pub fn new(
        id: ContentId,
        bounds: RegionBounds,
        province_size: u16,
        local_map_size: RegionMapSize,
        biomes: Vec<RegionBiomeRule>,
    ) -> Result<Self, RegionalWorldError> {
        if province_size == 0 {
            return Err(RegionalWorldError::ZeroProvinceSize);
        }
        if province_size > MAX_REGION_PROVINCE_SIZE {
            return Err(RegionalWorldError::ProvinceBudgetExceeded);
        }
        if biomes.is_empty() {
            return Err(RegionalWorldError::MissingBiomes);
        }
        if biomes.len() > MAX_REGION_BIOME_RULES {
            return Err(RegionalWorldError::BiomeBudgetExceeded);
        }
        let mut ids = BTreeSet::new();
        for biome in &biomes {
            if !ids.insert(biome.biome.clone()) {
                return Err(RegionalWorldError::DuplicateBiome(biome.biome.clone()));
            }
            if biome.minimum_depth > bounds.maximum_depth
                || biome
                    .maximum_depth
                    .is_some_and(|maximum| maximum > bounds.maximum_depth)
            {
                return Err(RegionalWorldError::BiomeDepthOutsideWorld(
                    biome.biome.clone(),
                ));
            }
            if biome.terrain.maximum_patch_radius.saturating_mul(2)
                >= local_map_size.width.min(local_map_size.height)
            {
                return Err(RegionalWorldError::TerrainPatchTooLargeForMap(
                    biome.biome.clone(),
                ));
            }
            if biome
                .population
                .rules
                .iter()
                .chain(&biome.encounters.rules)
                .any(|rule| {
                    rule.minimum_passage_distance()
                        >= local_map_size.width.min(local_map_size.height) / 2
                })
            {
                return Err(RegionalWorldError::PopulationDistanceTooLargeForMap(
                    biome.biome.clone(),
                ));
            }
            if biome
                .population
                .maximum_actor_count()
                .saturating_add(biome.encounters.maximum_actor_count())
                > MAX_REGION_POPULATION_ACTORS
            {
                return Err(RegionalWorldError::PopulationActorBudgetExceeded);
            }
            if biome.landmarks.minimum_passage_distance
                >= local_map_size.width.min(local_map_size.height) / 2
            {
                return Err(RegionalWorldError::LandmarkDistanceTooLargeForMap(
                    biome.biome.clone(),
                ));
            }
            if !biome.sites.is_empty() {
                let (minimum_sites, maximum_sites) = biome.sites.compound_range();
                let (minimum_caches, maximum_caches) = biome.landmarks.cache_range();
                let (minimum_camps, maximum_camps) = biome.landmarks.threat_camp_range();
                if minimum_caches < minimum_sites
                    || minimum_camps < minimum_sites
                    || maximum_caches < maximum_sites
                    || maximum_camps < maximum_sites
                {
                    return Err(RegionalWorldError::SitesRequirePairedLandmarks(
                        biome.biome.clone(),
                    ));
                }
                let (site_width, site_height) = biome.sites.size();
                if site_width.saturating_add(4) >= local_map_size.width
                    || site_height.saturating_add(4) >= local_map_size.height
                {
                    return Err(RegionalWorldError::SiteTooLargeForMap(biome.biome.clone()));
                }
            }
            if let Some(salvage) = &biome.salvage_loot {
                if salvage.minimum_draws != 1 || salvage.maximum_draws != 1 {
                    return Err(RegionalWorldError::InvalidLootDrawRange);
                }
                if biome.sites.is_empty()
                    || biome.landmarks.minimum_caches <= biome.sites.maximum_compounds
                    || biome
                        .destructibles
                        .as_ref()
                        .is_none_or(|profile| profile.count_range() != (1, 1))
                {
                    return Err(RegionalWorldError::SalvageRequiresStandaloneCache(
                        biome.biome.clone(),
                    ));
                }
            }
            if let Some(terminals) = &biome.site_terminals {
                if biome.sites.is_empty() {
                    return Err(RegionalWorldError::SiteTerminalsRequireSites(
                        biome.biome.clone(),
                    ));
                }
                let (minimum_terminals, maximum_terminals) = terminals.terminal_range();
                let (minimum_sites, maximum_sites) = biome.sites.compound_range();
                if minimum_terminals > minimum_sites || maximum_terminals > maximum_sites {
                    return Err(RegionalWorldError::SiteTerminalCountExceedsSites(
                        biome.biome.clone(),
                    ));
                }
            }
            if biome.site_security.is_some() {
                if biome.sites.is_empty() {
                    return Err(RegionalWorldError::SiteSecurityRequiresSites(
                        biome.biome.clone(),
                    ));
                }
                if biome.sites.entrances().locked_console_weight() == 0 {
                    return Err(RegionalWorldError::SiteSecurityRequiresLockedEntrances(
                        biome.biome.clone(),
                    ));
                }
                if biome.threats.is_none() {
                    return Err(RegionalWorldError::SiteSecurityRequiresThreats(
                        biome.biome.clone(),
                    ));
                }
            }
            if biome.threats.is_some() && biome.landmarks.maximum_threat_camps == 0 {
                return Err(RegionalWorldError::ThreatsRequireCamps(biome.biome.clone()));
            }
            if biome.destructibles.as_ref().is_some_and(|profile| {
                profile.minimum_passage_distance()
                    >= local_map_size.width.min(local_map_size.height) / 2
            }) {
                return Err(RegionalWorldError::DestructibleDistanceTooLargeForMap(
                    biome.biome.clone(),
                ));
            }
        }
        for depth in 0..=bounds.maximum_depth {
            if !biomes.iter().any(|biome| biome.supports(depth)) {
                return Err(RegionalWorldError::UncoveredDepth(depth));
            }
        }
        Ok(Self {
            id,
            bounds,
            province_size,
            local_map_size,
            surface_map_size: None,
            biomes,
            vertical_links: Vec::new(),
            cities: Vec::new(),
        })
    }

    pub fn with_vertical_links(
        mut self,
        vertical_links: Vec<RegionVerticalLink>,
    ) -> Result<Self, RegionalWorldError> {
        if vertical_links.len() > MAX_REGION_VERTICAL_LINKS {
            return Err(RegionalWorldError::VerticalLinkBudgetExceeded);
        }
        let mut unique = BTreeSet::new();
        for link in &vertical_links {
            if !self.bounds.contains(link.upper()) || !self.bounds.contains(link.lower()) {
                return Err(RegionalWorldError::VerticalLinkOutsideWorld);
            }
            if !unique.insert(*link) {
                return Err(RegionalWorldError::DuplicateVerticalLink(*link));
            }
        }
        self.vertical_links = vertical_links;
        Ok(self)
    }

    pub fn with_cities(
        mut self,
        cities: Vec<RegionCityDefinition>,
    ) -> Result<Self, RegionalWorldError> {
        if cities.len() > MAX_REGION_CITIES {
            return Err(RegionalWorldError::CityBudgetExceeded);
        }
        let mut ids = BTreeSet::new();
        let mut coordinates = BTreeSet::new();
        for city in &cities {
            if !self.bounds.contains(city.coordinate) {
                return Err(RegionalWorldError::CityOutsideWorld(city.id.clone()));
            }
            if !ids.insert(city.id.clone()) {
                return Err(RegionalWorldError::DuplicateCity(city.id.clone()));
            }
            if !coordinates.insert(city.coordinate) {
                return Err(RegionalWorldError::DuplicateCityCoordinate(city.coordinate));
            }
            for position in std::iter::once(city.merchant.position)
                .chain(std::iter::once(city.clinic.work_position))
                .chain(std::iter::once(city.clinic.break_position))
                .chain(city.residents.iter().flat_map(|resident| {
                    [resident.residence_position, resident.gathering_position]
                }))
            {
                if position.x <= 0
                    || position.y <= 0
                    || position.x >= i32::from(city.map_size.width) - 1
                    || position.y >= i32::from(city.map_size.height) - 1
                {
                    return Err(RegionalWorldError::CityPositionOutsideMap {
                        city: city.id.clone(),
                        position,
                    });
                }
            }
        }
        self.cities = cities;
        Ok(self)
    }

    pub const fn id(&self) -> &ContentId {
        &self.id
    }

    pub const fn bounds(&self) -> RegionBounds {
        self.bounds
    }

    pub const fn province_size(&self) -> u16 {
        self.province_size
    }

    pub const fn local_map_size(&self) -> RegionMapSize {
        self.local_map_size
    }

    pub fn with_surface_map_size(
        mut self,
        size: Option<RegionMapSize>,
    ) -> Result<Self, RegionalWorldError> {
        if size.is_some_and(|size| {
            size.width < self.local_map_size.width || size.height < self.local_map_size.height
        }) {
            return Err(RegionalWorldError::SurfaceMapSmallerThanLocalMap);
        }
        self.surface_map_size = size;
        Ok(self)
    }

    pub fn biomes(&self) -> &[RegionBiomeRule] {
        &self.biomes
    }

    pub fn vertical_links(&self) -> &[RegionVerticalLink] {
        &self.vertical_links
    }

    pub fn cities(&self) -> &[RegionCityDefinition] {
        &self.cities
    }

    pub fn city_at(&self, coordinate: RegionCoord) -> Option<&RegionCityDefinition> {
        self.cities
            .iter()
            .find(|city| city.coordinate == coordinate)
    }

    /// Returns the actual dimensions of a local destination. Surface regions
    /// and authored cities may differ from the procedural regions below;
    /// passage arrivals must therefore be resolved from the destination, not
    /// from the atlas-wide procedural default.
    pub fn map_size_at(&self, coordinate: RegionCoord) -> RegionMapSize {
        self.city_at(coordinate).map_or_else(
            || {
                if coordinate.depth == 0 {
                    self.surface_map_size.unwrap_or(self.local_map_size)
                } else {
                    self.local_map_size
                }
            },
            RegionCityDefinition::map_size,
        )
    }

    pub fn biome(&self, id: &ContentId) -> Option<&RegionBiomeRule> {
        self.biomes.iter().find(|biome| &biome.biome == id)
    }

    /// Resolves only lightweight metadata. The local map, actors and loot are
    /// deliberately left to a zone provider when the player first visits.
    pub fn region(&self, world_seed: u64, coordinate: RegionCoord) -> Option<RegionDescriptor> {
        if !self.bounds.contains(coordinate) {
            return None;
        }
        let province_size = i32::from(self.province_size);
        let province_x = coordinate.x.div_euclid(province_size);
        let province_y = coordinate.y.div_euclid(province_size);
        let total_weight: u64 = self
            .biomes
            .iter()
            .filter(|biome| biome.supports(coordinate.depth))
            .map(|biome| u64::from(biome.weight))
            .sum();
        let mut roll = coordinate_hash(
            world_seed,
            province_x,
            province_y,
            coordinate.depth,
            0x4249_4f4d_455f_5052,
        ) % total_weight;
        let biome = self
            .biomes
            .iter()
            .filter(|biome| biome.supports(coordinate.depth))
            .find(|biome| {
                let weight = u64::from(biome.weight);
                if roll < weight {
                    true
                } else {
                    roll -= weight;
                    false
                }
            })
            .expect("validated depths always have an eligible biome")
            .biome
            .clone();
        Some(RegionDescriptor {
            coordinate,
            seed: coordinate_hash(
                world_seed,
                coordinate.x,
                coordinate.y,
                coordinate.depth,
                0x5245_4749_4f4e_5344,
            ),
            biome,
        })
    }

    pub fn cardinal_neighbors(&self, coordinate: RegionCoord) -> Vec<RegionCoord> {
        [
            RegionDirection::North,
            RegionDirection::East,
            RegionDirection::South,
            RegionDirection::West,
        ]
        .into_iter()
        .filter_map(|direction| coordinate.step(direction))
        .filter(|neighbor| self.bounds.contains(*neighbor))
        .collect()
    }

    pub fn vertical_neighbor(
        &self,
        coordinate: RegionCoord,
        direction: RegionVerticalDirection,
    ) -> Option<RegionCoord> {
        self.vertical_links
            .iter()
            .copied()
            .find(|link| link.endpoint(direction) == coordinate)
            .map(|link| link.destination(direction))
    }

    pub fn vertical_neighbors(
        &self,
        coordinate: RegionCoord,
    ) -> Vec<(RegionVerticalDirection, RegionCoord)> {
        [RegionVerticalDirection::Up, RegionVerticalDirection::Down]
            .into_iter()
            .filter_map(|direction| {
                self.vertical_neighbor(coordinate, direction)
                    .map(|destination| (direction, destination))
            })
            .collect()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RegionalWorldCatalog {
    definitions: BTreeMap<ContentId, RegionalWorldDefinition>,
}

impl RegionalWorldCatalog {
    pub fn register(
        &mut self,
        definition: RegionalWorldDefinition,
    ) -> Result<(), RegionalWorldError> {
        if self.definitions.contains_key(definition.id()) {
            return Err(RegionalWorldError::DuplicateWorld(definition.id().clone()));
        }
        self.definitions.insert(definition.id().clone(), definition);
        Ok(())
    }

    pub fn get(&self, id: &ContentId) -> Option<&RegionalWorldDefinition> {
        self.definitions.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ContentId, &RegionalWorldDefinition)> {
        self.definitions.iter()
    }

    pub fn without_surface_map_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            definition.surface_map_size = None;
        }
        catalog
    }

    /// Restores the two core surface biomes as they were before the v93
    /// exploration trial. Existing suspensions must retain their terrain,
    /// site, threat-source and loot draws even when the live data is richer.
    /// Other worlds and modded biomes are left untouched.
    pub fn without_surface_exploration_metadata(&self) -> Self {
        let mut catalog = self.clone();
        if let Some(definition) = catalog
            .definitions
            .values_mut()
            .find(|definition| definition.id.as_str() == "core:simulation_overworld")
        {
            for biome in &mut definition.biomes {
                let (patch_count, encounter_maximum) = match biome.biome.as_str() {
                    "core:human_habitat" => (16, 5),
                    "core:surface_wilds" => (24, 6),
                    _ => continue,
                };
                biome.terrain.patch_count = patch_count;
                biome.encounters.maximum_group_rolls = encounter_maximum;
                biome.landmarks.minimum_caches = 2;
                biome.landmarks.maximum_caches = 2;
                biome.landmarks.minimum_threat_camps = 1;
                biome.landmarks.maximum_threat_camps = 2;
                biome.sites.minimum_compounds = 1;
                biome.sites.maximum_compounds = 2;
                if let Some(loot) = &mut biome.loot {
                    loot.minimum_draws = 2;
                    loot.maximum_draws = 4;
                }
            }
        }
        catalog
    }

    /// Keeps v93 surface encounter tables and loot budgets intact on resume.
    /// The new layout and placement algorithms are gated separately by the
    /// saved generation version.
    pub fn without_surface_variety_metadata(&self) -> Self {
        let mut catalog = self.clone();
        if let Some(definition) = catalog
            .definitions
            .values_mut()
            .find(|definition| definition.id.as_str() == "core:simulation_overworld")
        {
            for biome in &mut definition.biomes {
                if !matches!(
                    biome.biome.as_str(),
                    "core:human_habitat" | "core:surface_wilds"
                ) {
                    continue;
                }
                biome.encounters.rules.truncate(2);
                if let Some(loot) = &mut biome.loot {
                    loot.minimum_draws = 3;
                }
            }
        }
        catalog
    }

    /// Surface salvage containers were introduced after v94; earlier runs
    /// retain their original content fingerprint and region population.
    pub fn without_surface_salvage_metadata(&self) -> Self {
        let mut catalog = self.clone();
        if let Some(definition) = catalog
            .definitions
            .values_mut()
            .find(|definition| definition.id.as_str() == "core:simulation_overworld")
        {
            for biome in &mut definition.biomes {
                if matches!(
                    biome.biome.as_str(),
                    "core:human_habitat" | "core:surface_wilds"
                ) {
                    biome.destructibles = None;
                    biome.salvage_loot = None;
                }
            }
        }
        catalog
    }

    pub fn without_population_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                biome.population = RegionPopulationProfile::default();
            }
        }
        catalog
    }

    pub fn without_vertical_link_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            definition.vertical_links.clear();
        }
        catalog
    }

    /// Restores the exact v29-v74 core atlas: those generations know the first
    /// surface shaft, but none of its later continuations below layer 1.
    /// Other worlds and links are left untouched so modded historical routes
    /// retain their own fingerprints and replay behavior.
    pub fn without_second_layer_route_metadata(&self) -> Self {
        let mut catalog = self.clone();
        let core_world = "core:simulation_overworld";
        if let Some(definition) = catalog
            .definitions
            .values_mut()
            .find(|definition| definition.id.as_str() == core_world)
        {
            definition.vertical_links.retain(|link| {
                link.upper().x != -1 || link.upper().y != 0 || link.upper().depth == 0
            });
        }
        catalog
    }

    /// Restores the v75-v77 core shaft, which ends on layer 2. Other worlds
    /// and unrelated authored links are not altered.
    pub fn without_third_layer_route_metadata(&self) -> Self {
        let mut catalog = self.clone();
        if let Some(definition) = catalog
            .definitions
            .values_mut()
            .find(|definition| definition.id.as_str() == "core:simulation_overworld")
        {
            definition.vertical_links.retain(|link| {
                link.upper().x != -1 || link.upper().y != 0 || link.upper().depth < 2
            });
        }
        catalog
    }

    /// Restores the v78-v79 core shaft, which ends on layer 3. Other worlds
    /// and unrelated authored links are not altered.
    pub fn without_fourth_layer_route_metadata(&self) -> Self {
        let mut catalog = self.clone();
        if let Some(definition) = catalog
            .definitions
            .values_mut()
            .find(|definition| definition.id.as_str() == "core:simulation_overworld")
        {
            definition.vertical_links.retain(|link| {
                link.upper().x != -1 || link.upper().y != 0 || link.upper().depth < 3
            });
        }
        catalog
    }

    /// Restores the v80-v81 core shaft, which ends on layer 4. Other worlds
    /// and unrelated authored links are not altered.
    pub fn without_fifth_layer_route_metadata(&self) -> Self {
        let mut catalog = self.clone();
        if let Some(definition) = catalog
            .definitions
            .values_mut()
            .find(|definition| definition.id.as_str() == "core:simulation_overworld")
        {
            definition.vertical_links.retain(|link| {
                link.upper().x != -1 || link.upper().y != 0 || link.upper().depth < 4
            });
        }
        catalog
    }

    /// Removes v76+ settlements while preserving the v75 atlas and its second
    /// vertical route. Older saves therefore keep their procedural region at
    /// the same coordinate instead of silently acquiring a city and services.
    pub fn without_city_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            definition.cities.clear();
        }
        catalog
    }

    /// Restores the exact v76 core atlas. That generation contains the first
    /// city on layer 1, while every deeper destination remains procedural.
    /// Authored cities from other packages are left untouched.
    pub fn without_second_city_metadata(&self) -> Self {
        let mut catalog = self.clone();
        if let Some(definition) = catalog
            .definitions
            .values_mut()
            .find(|definition| definition.id.as_str() == "core:simulation_overworld")
        {
            definition.cities.retain(|city| city.coordinate.depth < 2);
        }
        catalog
    }

    /// Restores the v77-v78 settlement catalogue: cities exist on layers 1
    /// and 2, while every deeper destination remains procedural.
    pub fn without_third_city_metadata(&self) -> Self {
        let mut catalog = self.clone();
        if let Some(definition) = catalog
            .definitions
            .values_mut()
            .find(|definition| definition.id.as_str() == "core:simulation_overworld")
        {
            definition.cities.retain(|city| city.coordinate.depth < 3);
        }
        catalog
    }

    /// Restores the v79-v80 settlement catalogue: cities exist through layer
    /// 3, while layer 4 remains procedural even when its route is present.
    pub fn without_fourth_city_metadata(&self) -> Self {
        let mut catalog = self.clone();
        if let Some(definition) = catalog
            .definitions
            .values_mut()
            .find(|definition| definition.id.as_str() == "core:simulation_overworld")
        {
            definition.cities.retain(|city| city.coordinate.depth < 4);
        }
        catalog
    }

    /// Restores the v81-v82 settlement catalogue: cities exist through layer
    /// 4, while layer 5 remains procedural even when its route is present.
    pub fn without_fifth_city_metadata(&self) -> Self {
        let mut catalog = self.clone();
        if let Some(definition) = catalog
            .definitions
            .values_mut()
            .find(|definition| definition.id.as_str() == "core:simulation_overworld")
        {
            definition.cities.retain(|city| city.coordinate.depth < 5);
        }
        catalog
    }

    pub fn without_deeper_layer_gameplay_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                if biome.minimum_depth == 0 {
                    continue;
                }
                biome.population = RegionPopulationProfile::default();
                biome.encounters = RegionPopulationProfile::default();
                biome.loot = None;
                biome.salvage_loot = None;
                biome.landmarks = RegionLandmarkProfile::default();
                biome.sites = RegionSiteProfile::default();
                biome.site_terminals = None;
                biome.site_security = None;
                biome.threats = None;
                biome.destructibles = None;
            }
        }
        catalog
    }

    pub fn without_encounter_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                biome.encounters = RegionPopulationProfile::default();
            }
        }
        catalog
    }

    pub fn without_pursuit_lifecycle_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                for rule in &mut biome.population.rules {
                    rule.group.remove_pursuit_lifecycle();
                }
                for rule in &mut biome.encounters.rules {
                    rule.group.remove_pursuit_lifecycle();
                }
            }
        }
        catalog
    }

    /// Removes v32 attributes from roaming populations, landmark encounters
    /// and renewable threats without changing any older regional metadata.
    pub fn without_primary_attribute_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                for rule in &mut biome.population.rules {
                    rule.group.remove_primary_attributes();
                }
                for rule in &mut biome.encounters.rules {
                    rule.group.remove_primary_attributes();
                }
                if let Some(threats) = &mut biome.threats {
                    threats.actor.remove_primary_attributes();
                }
            }
        }
        catalog
    }

    /// Removes explicit v60 combat dispositions from every generated actor
    /// source so older regional catalogue fingerprints remain unchanged.
    pub fn without_player_relation_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                for rule in &mut biome.population.rules {
                    rule.group.remove_player_relation_metadata();
                }
                for rule in &mut biome.encounters.rules {
                    rule.group.remove_player_relation_metadata();
                }
                if let Some(threats) = &mut biome.threats {
                    threats.actor.remove_player_relation_metadata();
                }
            }
        }
        catalog
    }

    /// Removes v33 body and Impact profiles from every hostile source while
    /// retaining the historical final integrity and damage fields.
    pub fn without_physical_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                for rule in &mut biome.population.rules {
                    rule.group.remove_physical_metadata();
                }
                for rule in &mut biome.encounters.rules {
                    rule.group.remove_physical_metadata();
                }
                if let Some(threats) = &mut biome.threats {
                    threats.actor.remove_physical_metadata();
                }
            }
        }
        catalog
    }

    /// Removes v42 displacement and locomotion capabilities without erasing
    /// the earlier body profiles, attributes or attack Impact metadata.
    pub fn without_melee_skill_body_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                for rule in &mut biome.population.rules {
                    rule.group.remove_melee_skill_body_metadata();
                }
                for rule in &mut biome.encounters.rules {
                    rule.group.remove_melee_skill_body_metadata();
                }
                if let Some(threats) = &mut biome.threats {
                    threats.actor.remove_melee_skill_body_metadata();
                }
            }
        }
        catalog
    }

    /// Removes v43 suppression compatibility while retaining all earlier body
    /// and melee metadata used by historical world fingerprints.
    pub fn without_ranged_skill_body_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                for rule in &mut biome.population.rules {
                    rule.group.remove_ranged_skill_body_metadata();
                }
                for rule in &mut biome.encounters.rules {
                    rule.group.remove_ranged_skill_body_metadata();
                }
                if let Some(threats) = &mut biome.threats {
                    threats.actor.remove_ranged_skill_body_metadata();
                }
            }
        }
        catalog
    }

    /// Removes v51 electronic target profiles from roaming populations,
    /// encounters and renewable threats for historical replay fingerprints.
    pub fn without_electronic_system_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                for rule in &mut biome.population.rules {
                    rule.group.remove_electronic_system_metadata();
                }
                for rule in &mut biome.encounters.rules {
                    rule.group.remove_electronic_system_metadata();
                }
                if let Some(threats) = &mut biome.threats {
                    threats.actor.remove_electronic_system_metadata();
                }
            }
        }
        catalog
    }

    pub fn without_loot_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                biome.loot = None;
                biome.salvage_loot = None;
            }
        }
        catalog
    }

    pub fn without_landmark_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                biome.landmarks = RegionLandmarkProfile::default();
                biome.salvage_loot = None;
                biome.sites = RegionSiteProfile::default();
                biome.site_terminals = None;
                biome.site_security = None;
                biome.threats = None;
            }
        }
        catalog
    }

    pub fn without_threat_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                biome.threats = None;
                biome.site_security = None;
            }
        }
        catalog
    }

    pub fn without_site_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                biome.sites = RegionSiteProfile::default();
                biome.salvage_loot = None;
                biome.site_terminals = None;
                biome.site_security = None;
            }
        }
        catalog
    }

    pub fn without_site_interaction_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                biome.sites = biome.sites.without_interactions();
                biome.site_security = None;
            }
        }
        catalog
    }

    pub fn without_site_security_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                biome.site_security = None;
            }
        }
        catalog
    }

    pub fn without_site_terminal_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                biome.site_terminals = None;
            }
        }
        catalog
    }

    pub fn without_site_navigation_signal_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                if let Some(security) = &mut biome.site_security {
                    security.navigation_signal_range = None;
                }
            }
        }
        catalog
    }

    pub fn without_destructible_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                biome.destructibles = None;
                biome.salvage_loot = None;
            }
        }
        catalog
    }

    /// Removes the v74 water-conduction proof while retaining the volatile
    /// destructibles that already belonged to v30-v73 regional generation.
    pub fn without_environmental_conduction_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            for biome in &mut definition.biomes {
                if biome
                    .destructibles
                    .as_ref()
                    .is_some_and(RegionDestructibleProfile::uses_distinct_water_propagation)
                {
                    biome.destructibles = None;
                }
            }
        }
        catalog
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegionalWorldError {
    InvertedBounds,
    WorldBudgetExceeded,
    ZeroProvinceSize,
    ProvinceBudgetExceeded,
    MissingBiomes,
    BiomeBudgetExceeded,
    ZeroBiomeWeight,
    InvertedBiomeDepth,
    DuplicateBiome(ContentId),
    BiomeDepthOutsideWorld(ContentId),
    UncoveredDepth(u16),
    DuplicateWorld(ContentId),
    LocalMapTooSmall,
    LocalMapTooLarge,
    SurfaceMapSmallerThanLocalMap,
    BlockedBaseTerrain,
    ZeroTerrainWeight,
    TerrainPatchBudgetExceeded,
    TerrainRuleBudgetExceeded,
    MissingTerrainRules,
    UnusedTerrainRules,
    InvalidTerrainPatchRadius,
    DuplicateTerrain(RegionTerrain),
    TerrainPatchTooLargeForMap(ContentId),
    ZeroPopulationWeight,
    InvalidPopulationCountRange,
    InvalidPopulationRollRange,
    PopulationRollBudgetExceeded,
    PopulationRuleBudgetExceeded,
    PopulationActorBudgetExceeded,
    MissingPopulationRules,
    UnusedPopulationRules,
    InvalidPopulation(Box<ExpeditionDefinitionError>),
    PopulationDistanceTooLargeForMap(ContentId),
    InvalidLootDrawRange,
    InvalidLandmarkRange,
    InvalidSiteCountRange,
    InvalidSiteDimensions,
    UnusedSiteDimensions,
    UnusedSiteEntrances,
    InvalidSiteTerminalCountRange,
    MissingSiteTerminalRecords,
    SiteTerminalRecordBudgetExceeded,
    DuplicateSiteTerminalRecord(ContentId),
    InvalidSiteSecurityProfile(SecurityAlarmProfileError),
    InvalidSiteNavigationSignalRange,
    InvalidThreatLimits,
    LandmarkDistanceTooLargeForMap(ContentId),
    SitesRequirePairedLandmarks(ContentId),
    SalvageRequiresStandaloneCache(ContentId),
    SiteTooLargeForMap(ContentId),
    SiteTerminalsRequireSites(ContentId),
    SiteTerminalCountExceedsSites(ContentId),
    ThreatsRequireCamps(ContentId),
    SiteSecurityRequiresSites(ContentId),
    SiteSecurityRequiresLockedEntrances(ContentId),
    SiteSecurityRequiresThreats(ContentId),
    UnknownLootTable(ContentId),
    InvalidVerticalLink,
    VerticalLinkOutsideWorld,
    DuplicateVerticalLink(RegionVerticalLink),
    VerticalLinkBudgetExceeded,
    EmptyCityName,
    CityBudgetExceeded,
    InvalidCityResidentCount,
    CityLayoutTooSmall(RegionCityLayout),
    InvalidCity(Box<ExpeditionDefinitionError>),
    CityOutsideWorld(ContentId),
    DuplicateCity(ContentId),
    DuplicateCityCoordinate(RegionCoord),
    DuplicateCityProviderPosition(crate::world::GridPos),
    CityPositionOutsideMap {
        city: ContentId,
        position: crate::world::GridPos,
    },
    InvalidDestructibleCountRange,
    ZeroDestructibleIntegrity,
    InvalidDestructionEffect,
    DestructibleDistanceTooLargeForMap(ContentId),
}

impl Display for RegionalWorldError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvertedBounds => write!(formatter, "regional world bounds are inverted"),
            Self::WorldBudgetExceeded => write!(
                formatter,
                "regional world exceeds {MAX_REGIONAL_WORLD_SIDE} regions per side or depth {MAX_REGIONAL_WORLD_DEPTH}"
            ),
            Self::ZeroProvinceSize => write!(formatter, "region province size must be positive"),
            Self::ProvinceBudgetExceeded => write!(
                formatter,
                "region province size exceeds {MAX_REGION_PROVINCE_SIZE}"
            ),
            Self::MissingBiomes => write!(formatter, "regional world requires biome rules"),
            Self::BiomeBudgetExceeded => write!(
                formatter,
                "regional world exceeds {MAX_REGION_BIOME_RULES} biome rules"
            ),
            Self::ZeroBiomeWeight => write!(formatter, "region biome weight must be positive"),
            Self::InvertedBiomeDepth => {
                write!(formatter, "region biome depth range is inverted")
            }
            Self::DuplicateBiome(biome) => {
                write!(formatter, "duplicate region biome '{biome}'")
            }
            Self::BiomeDepthOutsideWorld(biome) => write!(
                formatter,
                "region biome '{biome}' has a depth outside the regional world"
            ),
            Self::UncoveredDepth(depth) => {
                write!(
                    formatter,
                    "regional world depth {depth} has no eligible biome"
                )
            }
            Self::DuplicateWorld(world) => write!(formatter, "duplicate regional world '{world}'"),
            Self::LocalMapTooSmall => write!(
                formatter,
                "regional local maps must be at least {MIN_REGION_MAP_SIDE} tiles per side"
            ),
            Self::LocalMapTooLarge => write!(
                formatter,
                "regional local maps cannot exceed {MAX_REGION_MAP_SIDE} tiles per side"
            ),
            Self::SurfaceMapSmallerThanLocalMap => write!(
                formatter,
                "regional surface maps cannot be smaller than the ordinary local map"
            ),
            Self::BlockedBaseTerrain => {
                write!(formatter, "regional base terrain must be walkable")
            }
            Self::ZeroTerrainWeight => write!(formatter, "region terrain weight must be positive"),
            Self::TerrainPatchBudgetExceeded => write!(
                formatter,
                "regional terrain exceeds {MAX_REGION_TERRAIN_PATCHES} patches"
            ),
            Self::TerrainRuleBudgetExceeded => write!(
                formatter,
                "regional terrain exceeds {MAX_REGION_TERRAIN_RULES} feature rules"
            ),
            Self::MissingTerrainRules => {
                write!(formatter, "regional terrain patches require feature rules")
            }
            Self::UnusedTerrainRules => {
                write!(
                    formatter,
                    "regional terrain rules require at least one patch"
                )
            }
            Self::InvalidTerrainPatchRadius => write!(
                formatter,
                "regional terrain patch radii must be ordered, positive and at most {MAX_REGION_PATCH_RADIUS}"
            ),
            Self::DuplicateTerrain(terrain) => {
                write!(formatter, "duplicate regional terrain rule '{terrain:?}'")
            }
            Self::TerrainPatchTooLargeForMap(biome) => write!(
                formatter,
                "regional biome '{biome}' has patches too large for its local map"
            ),
            Self::ZeroPopulationWeight => {
                write!(formatter, "regional population weight must be positive")
            }
            Self::InvalidPopulationCountRange => write!(
                formatter,
                "regional population counts must be positive and ordered"
            ),
            Self::InvalidPopulationRollRange => write!(
                formatter,
                "regional population group-roll range is inverted"
            ),
            Self::PopulationRollBudgetExceeded => write!(
                formatter,
                "regional population exceeds {MAX_REGION_POPULATION_ROLLS} group rolls"
            ),
            Self::PopulationRuleBudgetExceeded => write!(
                formatter,
                "regional population exceeds {MAX_REGION_POPULATION_RULES} weighted rules"
            ),
            Self::PopulationActorBudgetExceeded => write!(
                formatter,
                "regional population can exceed {MAX_REGION_POPULATION_ACTORS} actors"
            ),
            Self::MissingPopulationRules => write!(
                formatter,
                "regional population rolls require weighted rules"
            ),
            Self::UnusedPopulationRules => write!(
                formatter,
                "regional population rules require at least one group roll"
            ),
            Self::InvalidPopulation(error) => {
                write!(formatter, "invalid regional population: {error}")
            }
            Self::PopulationDistanceTooLargeForMap(biome) => write!(
                formatter,
                "regional biome '{biome}' keeps population too far from every passage for its local map"
            ),
            Self::InvalidLootDrawRange => write!(
                formatter,
                "regional loot draws must be ordered and cannot exceed {MAX_REGION_LOOT_DRAWS}"
            ),
            Self::InvalidLandmarkRange => write!(
                formatter,
                "regional landmark counts must be ordered and cannot exceed {MAX_REGION_LANDMARKS}"
            ),
            Self::InvalidSiteCountRange => write!(
                formatter,
                "regional site counts must be ordered and cannot exceed {MAX_REGION_SITES}"
            ),
            Self::InvalidSiteDimensions => write!(
                formatter,
                "regional site dimensions must be odd and between 5 and {MAX_REGION_SITE_SIDE}"
            ),
            Self::UnusedSiteDimensions => write!(
                formatter,
                "regional site dimensions require at least one compound"
            ),
            Self::UnusedSiteEntrances => write!(
                formatter,
                "regional site entrance weights require at least one compound"
            ),
            Self::InvalidSiteTerminalCountRange => write!(
                formatter,
                "regional site terminal counts must be positive, ordered and cannot exceed {MAX_REGION_SITES}"
            ),
            Self::MissingSiteTerminalRecords => {
                write!(formatter, "regional site terminals require record IDs")
            }
            Self::SiteTerminalRecordBudgetExceeded => write!(
                formatter,
                "regional site terminals exceed {MAX_REGION_SITE_TERMINAL_RECORDS} record IDs"
            ),
            Self::DuplicateSiteTerminalRecord(record) => {
                write!(
                    formatter,
                    "duplicate regional site terminal record '{record}'"
                )
            }
            Self::InvalidSiteSecurityProfile(error) => {
                write!(formatter, "invalid regional site security profile: {error}")
            }
            Self::InvalidSiteNavigationSignalRange => write!(
                formatter,
                "regional site navigation signal range must be between 1 and {MAX_NAVIGATION_BEACON_RANGE}"
            ),
            Self::InvalidThreatLimits => write!(
                formatter,
                "regional threat active limit cannot exceed its finite total limit"
            ),
            Self::LandmarkDistanceTooLargeForMap(biome) => write!(
                formatter,
                "regional biome '{biome}' keeps landmarks too far from every passage for its local map"
            ),
            Self::SitesRequirePairedLandmarks(biome) => write!(
                formatter,
                "regional biome '{biome}' requires at least one cache and threat camp per guaranteed compound"
            ),
            Self::SalvageRequiresStandaloneCache(biome) => write!(
                formatter,
                "regional biome '{biome}' requires one standalone cache and one volatile container for salvage loot"
            ),
            Self::SiteTooLargeForMap(biome) => write!(
                formatter,
                "regional biome '{biome}' defines a compound too large for its local map"
            ),
            Self::SiteTerminalsRequireSites(biome) => write!(
                formatter,
                "regional biome '{biome}' defines site terminals without compounds"
            ),
            Self::SiteTerminalCountExceedsSites(biome) => write!(
                formatter,
                "regional biome '{biome}' can generate more terminals than compounds"
            ),
            Self::ThreatsRequireCamps(biome) => write!(
                formatter,
                "regional biome '{biome}' defines threat renewal without any threat camp"
            ),
            Self::SiteSecurityRequiresSites(biome) => write!(
                formatter,
                "regional biome '{biome}' defines site security without compounds"
            ),
            Self::SiteSecurityRequiresLockedEntrances(biome) => write!(
                formatter,
                "regional biome '{biome}' defines site security without locked console entrances"
            ),
            Self::SiteSecurityRequiresThreats(biome) => write!(
                formatter,
                "regional biome '{biome}' defines site security without finite threat sources"
            ),
            Self::UnknownLootTable(table) => {
                write!(formatter, "unknown regional loot table '{table}'")
            }
            Self::InvalidVerticalLink => write!(
                formatter,
                "regional vertical links must keep x/y and connect one depth to the next"
            ),
            Self::VerticalLinkOutsideWorld => {
                write!(
                    formatter,
                    "regional vertical link lies outside world bounds"
                )
            }
            Self::DuplicateVerticalLink(link) => {
                write!(formatter, "duplicate regional vertical link '{link:?}'")
            }
            Self::VerticalLinkBudgetExceeded => write!(
                formatter,
                "regional world exceeds {MAX_REGION_VERTICAL_LINKS} vertical links"
            ),
            Self::EmptyCityName => write!(formatter, "regional city name must not be empty"),
            Self::CityBudgetExceeded => write!(
                formatter,
                "regional world exceeds {MAX_REGION_CITIES} authored cities"
            ),
            Self::InvalidCityResidentCount => write!(
                formatter,
                "regional city requires between 1 and {MAX_REGION_CITY_RESIDENTS} residents"
            ),
            Self::CityLayoutTooSmall(layout) => {
                let (width, height) = layout.minimum_size();
                write!(
                    formatter,
                    "regional city layout '{layout:?}' requires at least {width} x {height} tiles"
                )
            }
            Self::InvalidCity(error) => write!(formatter, "invalid regional city: {error}"),
            Self::CityOutsideWorld(city) => {
                write!(formatter, "regional city '{city}' is outside world bounds")
            }
            Self::DuplicateCity(city) => write!(formatter, "duplicate regional city '{city}'"),
            Self::DuplicateCityCoordinate(coordinate) => write!(
                formatter,
                "multiple regional cities occupy [{}, {}, {}]",
                coordinate.x, coordinate.y, coordinate.depth
            ),
            Self::DuplicateCityProviderPosition(position) => write!(
                formatter,
                "regional city has multiple providers at [{}, {}]",
                position.x, position.y
            ),
            Self::CityPositionOutsideMap { city, position } => write!(
                formatter,
                "regional city '{city}' uses an out-of-bounds service position [{}, {}]",
                position.x, position.y
            ),
            Self::InvalidDestructibleCountRange => write!(
                formatter,
                "regional destructible counts must be ordered, positive and cannot exceed {MAX_REGION_DESTRUCTIBLES}"
            ),
            Self::ZeroDestructibleIntegrity => {
                write!(
                    formatter,
                    "regional destructible integrity must be positive"
                )
            }
            Self::InvalidDestructionEffect => write!(
                formatter,
                "regional destruction effects require positive bounded propagation and damage"
            ),
            Self::DestructibleDistanceTooLargeForMap(biome) => write!(
                formatter,
                "regional biome '{biome}' keeps destructibles too far from every passage for its local map"
            ),
        }
    }
}

impl Error for RegionalWorldError {}

fn coordinate_hash(seed: u64, x: i32, y: i32, depth: u16, salt: u64) -> u64 {
    let mut value = mix(seed ^ salt);
    value = mix(value ^ (i64::from(x) as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15));
    value = mix(value ^ (i64::from(y) as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9));
    mix(value ^ u64::from(depth).wrapping_mul(0x94D0_49BB_1331_11EB))
}

fn mix(mut value: u64) -> u64 {
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> ContentId {
        format!("test:{value}").parse().unwrap()
    }

    fn terrain_profile() -> RegionTerrainProfile {
        RegionTerrainProfile::new(
            RegionTerrain::Grass,
            12,
            2,
            6,
            vec![
                RegionTerrainRule::new(RegionTerrain::Tree, 3).unwrap(),
                RegionTerrainRule::new(RegionTerrain::DeepWater, 1).unwrap(),
            ],
        )
        .unwrap()
    }

    fn population_rule() -> RegionPopulationRule {
        RegionPopulationRule::new(
            3,
            1,
            2,
            6,
            8,
            AttackProfile::melee(crate::combat::DamageType::Kinetic, 2),
            AiProfile::hunter(8, 0)
                .with_maximum_pursuit_distance(std::num::NonZeroU16::new(10).unwrap()),
            Some(DefeatReward::persistent(4, 1)),
        )
        .unwrap()
    }

    fn definition() -> RegionalWorldDefinition {
        RegionalWorldDefinition::new(
            id("world"),
            RegionBounds::new(-512, 511, -512, 511, 7).unwrap(),
            4,
            RegionMapSize::new(96, 64).unwrap(),
            vec![
                RegionBiomeRule::new(id("surface"), 3, 0, Some(0), terrain_profile()).unwrap(),
                RegionBiomeRule::new(id("maintenance"), 3, 1, Some(4), terrain_profile()).unwrap(),
                RegionBiomeRule::new(id("corrupted"), 1, 1, Some(7), terrain_profile()).unwrap(),
            ],
        )
        .unwrap()
    }

    #[test]
    fn a_large_atlas_resolves_regions_without_preallocating_local_maps() {
        let world = definition();

        assert_eq!(world.bounds().addressable_region_count(), 8_388_608);
        let coordinate = RegionCoord::new(411, -207, 3);
        assert_eq!(world.region(42, coordinate), world.region(42, coordinate));
        assert_ne!(
            world.region(42, coordinate).unwrap().seed,
            world
                .region(42, RegionCoord::new(412, -207, 3))
                .unwrap()
                .seed
        );
        assert!(world.region(42, RegionCoord::new(512, 0, 0)).is_none());
    }

    #[test]
    fn one_province_keeps_a_coherent_biome_while_region_seeds_stay_unique() {
        let world = definition();
        let regions = [
            RegionCoord::new(8, 12, 2),
            RegionCoord::new(9, 12, 2),
            RegionCoord::new(8, 13, 2),
            RegionCoord::new(11, 15, 2),
        ]
        .map(|coordinate| world.region(99, coordinate).unwrap());

        assert!(
            regions
                .iter()
                .all(|region| region.biome == regions[0].biome)
        );
        assert_eq!(
            regions
                .iter()
                .map(|region| region.seed)
                .collect::<BTreeSet<_>>()
                .len(),
            regions.len()
        );
    }

    #[test]
    fn depth_filters_and_world_edges_are_respected() {
        let world = definition();

        assert_eq!(
            world.region(7, RegionCoord::new(0, 0, 0)).unwrap().biome,
            id("surface")
        );
        assert!(
            [
                RegionCoord::new(-512, -512, 0),
                RegionCoord::new(-511, -512, 0),
                RegionCoord::new(-512, -511, 0),
            ]
            .into_iter()
            .all(|coordinate| world
                .cardinal_neighbors(coordinate)
                .into_iter()
                .all(|neighbor| world.bounds().contains(neighbor)))
        );
        assert_eq!(
            world
                .cardinal_neighbors(RegionCoord::new(-512, -512, 0))
                .len(),
            2
        );
    }

    #[test]
    fn explicit_vertical_links_are_bounded_bidirectional_and_non_branching() {
        let upper = RegionCoord::new(-1, 0, 0);
        let lower = RegionCoord::new(-1, 0, 1);
        let link = RegionVerticalLink::new(upper, lower).unwrap();
        let world = definition().with_vertical_links(vec![link]).unwrap();

        assert_eq!(
            world.vertical_neighbor(upper, RegionVerticalDirection::Down),
            Some(lower)
        );
        assert_eq!(
            world.vertical_neighbor(lower, RegionVerticalDirection::Up),
            Some(upper)
        );
        assert!(
            world
                .vertical_neighbor(upper, RegionVerticalDirection::Up)
                .is_none()
        );
        assert_eq!(
            definition().with_vertical_links(vec![link, link]),
            Err(RegionalWorldError::DuplicateVerticalLink(link))
        );
        assert_eq!(
            RegionVerticalLink::new(upper, RegionCoord::new(0, 0, 1)),
            Err(RegionalWorldError::InvalidVerticalLink)
        );
    }

    #[test]
    fn invalid_worlds_fail_before_any_region_is_sampled() {
        assert_eq!(
            RegionBounds::new(1, 0, 0, 1, 0),
            Err(RegionalWorldError::InvertedBounds)
        );
        assert_eq!(
            RegionBiomeRule::new(id("bad"), 0, 0, None, terrain_profile()),
            Err(RegionalWorldError::ZeroBiomeWeight)
        );
        let bounds = RegionBounds::new(0, 3, 0, 3, 2).unwrap();
        assert_eq!(
            RegionalWorldDefinition::new(
                id("gap"),
                bounds,
                2,
                RegionMapSize::new(48, 32).unwrap(),
                vec![
                    RegionBiomeRule::new(id("surface"), 1, 0, Some(0), terrain_profile(),).unwrap(),
                ],
            ),
            Err(RegionalWorldError::UncoveredDepth(1))
        );
    }

    #[test]
    fn local_map_profiles_are_bounded_and_data_driven() {
        assert_eq!(
            RegionMapSize::new(MIN_REGION_MAP_SIDE - 1, 64),
            Err(RegionalWorldError::LocalMapTooSmall)
        );
        assert_eq!(
            RegionTerrainProfile::new(RegionTerrain::Tree, 0, 0, 0, vec![]),
            Err(RegionalWorldError::BlockedBaseTerrain)
        );
        assert_eq!(
            RegionTerrainRule::new(RegionTerrain::Tree, 0),
            Err(RegionalWorldError::ZeroTerrainWeight)
        );
        assert_eq!(
            RegionSiteProfile::new(2, 1, 9, 7),
            Err(RegionalWorldError::InvalidSiteCountRange)
        );
        assert_eq!(
            RegionSiteProfile::new(1, 1, 8, 7),
            Err(RegionalWorldError::InvalidSiteDimensions)
        );
        assert_eq!(
            RegionSiteProfile::new(0, 0, 9, 7),
            Err(RegionalWorldError::UnusedSiteDimensions)
        );
        assert_eq!(
            RegionSiteProfile::new(0, 0, 0, 0)
                .unwrap()
                .with_entrances(RegionSiteEntranceProfile::new(1, 0, 0)),
            Err(RegionalWorldError::UnusedSiteEntrances)
        );
        assert_eq!(
            format!("{:?}", RegionSiteProfile::new(1, 1, 9, 7).unwrap()),
            "RegionSiteProfile { minimum_compounds: 1, maximum_compounds: 1, width: 9, height: 7 }"
        );
        assert_eq!(
            RegionSiteTerminalProfile::new(0, 0, vec![id("record")]),
            Err(RegionalWorldError::InvalidSiteTerminalCountRange)
        );
        assert_eq!(
            RegionSiteTerminalProfile::new(1, 1, vec![]),
            Err(RegionalWorldError::MissingSiteTerminalRecords)
        );
        assert_eq!(
            RegionSiteTerminalProfile::new(1, 1, vec![id("record"), id("record")]),
            Err(RegionalWorldError::DuplicateSiteTerminalRecord(id(
                "record"
            )))
        );
        assert_eq!(
            RegionSiteSecurityProfile::new(
                id("security"),
                8,
                DistanceMetric::Euclidean,
                true,
                8,
                0,
            ),
            Err(RegionalWorldError::InvalidSiteSecurityProfile(
                SecurityAlarmProfileError::InvalidReinforcementDelay(0)
            ))
        );
        let legacy_security = RegionSiteSecurityProfile::new(
            id("security"),
            8,
            DistanceMetric::Euclidean,
            true,
            8,
            3,
        )
        .unwrap();
        assert!(!format!("{legacy_security:?}").contains("navigation_signal"));
        assert_eq!(
            legacy_security.clone().with_navigation_signal_range(0),
            Err(RegionalWorldError::InvalidSiteNavigationSignalRange)
        );
        assert_eq!(
            legacy_security.with_navigation_signal_range(MAX_NAVIGATION_BEACON_RANGE + 1),
            Err(RegionalWorldError::InvalidSiteNavigationSignalRange)
        );
    }

    #[test]
    fn regional_population_profiles_are_bounded_and_removable_for_legacy_replays() {
        assert_eq!(
            RegionPopulationProfile::new(2, 1, vec![population_rule()]),
            Err(RegionalWorldError::InvalidPopulationRollRange)
        );
        assert_eq!(
            RegionPopulationProfile::new(1, 1, Vec::new()),
            Err(RegionalWorldError::MissingPopulationRules)
        );
        assert_eq!(
            RegionPopulationRule::new(
                1,
                2,
                1,
                6,
                8,
                AttackProfile::melee(crate::combat::DamageType::Kinetic, 2),
                AiProfile::hunter(8, 0),
                None,
            ),
            Err(RegionalWorldError::InvalidPopulationCountRange)
        );

        let mut biome =
            RegionBiomeRule::new(id("surface"), 1, 0, Some(0), terrain_profile()).unwrap();
        biome = biome
            .with_population(RegionPopulationProfile::new(1, 2, vec![population_rule()]).unwrap())
            .with_encounters(RegionPopulationProfile::new(2, 3, vec![population_rule()]).unwrap());
        let world = RegionalWorldDefinition::new(
            id("populated_world"),
            RegionBounds::new(-2, 2, -2, 2, 0).unwrap(),
            1,
            RegionMapSize::new(48, 36).unwrap(),
            vec![biome],
        )
        .unwrap();
        let mut catalog = RegionalWorldCatalog::default();
        catalog.register(world).unwrap();

        assert!(
            !catalog.iter().next().unwrap().1.biomes()[0]
                .population()
                .is_empty()
        );
        assert!(
            !catalog.iter().next().unwrap().1.biomes()[0]
                .encounters()
                .is_empty()
        );
        assert!(
            catalog
                .without_population_metadata()
                .iter()
                .next()
                .unwrap()
                .1
                .biomes()[0]
                .population()
                .is_empty()
        );
        assert!(
            catalog
                .without_encounter_metadata()
                .iter()
                .next()
                .unwrap()
                .1
                .biomes()[0]
                .encounters()
                .is_empty()
        );
    }

    #[test]
    fn regional_destructibles_are_bounded_and_removable_for_legacy_replays() {
        let effect = DestructionEffect::new(crate::effects::RadialDamageEffect {
            maximum_cost: 2,
            neighbor_mode: crate::world::NeighborMode::CardinalAndDiagonal,
            propagation_policy: crate::world::TerrainPropagationPolicy::blocked_by_walls(1),
            damage: crate::combat::DamagePacket::new(8, crate::combat::DamageType::Explosive, 0),
            falloff: DamageFalloff::PerPropagationCost(2),
        });
        assert_eq!(
            RegionDestructibleProfile::new(0, 0, 6, 4, effect.clone()),
            Err(RegionalWorldError::InvalidDestructibleCountRange)
        );
        let profile = RegionDestructibleProfile::new(2, 4, 6, 4, effect).unwrap();
        let biome = RegionBiomeRule::new(id("maintenance"), 1, 0, Some(0), terrain_profile())
            .unwrap()
            .with_destructibles(profile);
        let world = RegionalWorldDefinition::new(
            id("destructible_world"),
            RegionBounds::new(-2, 2, -2, 2, 0).unwrap(),
            1,
            RegionMapSize::new(48, 36).unwrap(),
            vec![biome],
        )
        .unwrap();
        let mut catalog = RegionalWorldCatalog::default();
        catalog.register(world).unwrap();

        assert!(
            catalog.iter().next().unwrap().1.biomes()[0]
                .destructibles()
                .is_some()
        );
        assert!(
            catalog
                .without_destructible_metadata()
                .iter()
                .next()
                .unwrap()
                .1
                .biomes()[0]
                .destructibles()
                .is_none()
        );
        assert!(
            catalog
                .without_environmental_conduction_metadata()
                .iter()
                .next()
                .unwrap()
                .1
                .biomes()[0]
                .destructibles()
                .is_some()
        );
    }

    #[test]
    fn conductive_destructibles_can_be_removed_without_removing_legacy_fire() {
        let effect = DestructionEffect::new(crate::effects::RadialDamageEffect {
            maximum_cost: 4,
            neighbor_mode: crate::world::NeighborMode::CardinalAndDiagonal,
            propagation_policy: crate::world::TerrainPropagationPolicy::conductive(3, 1),
            damage: crate::combat::DamagePacket::new(5, crate::combat::DamageType::Electrical, 0),
            falloff: DamageFalloff::PerPropagationCost(1),
        });
        let profile = RegionDestructibleProfile::new(2, 3, 6, 4, effect).unwrap();
        assert!(profile.uses_distinct_water_propagation());
        let biome = RegionBiomeRule::new(id("research"), 1, 0, Some(0), terrain_profile())
            .unwrap()
            .with_destructibles(profile);
        let world = RegionalWorldDefinition::new(
            id("conductive_world"),
            RegionBounds::new(-2, 2, -2, 2, 0).unwrap(),
            1,
            RegionMapSize::new(48, 36).unwrap(),
            vec![biome],
        )
        .unwrap();
        let mut catalog = RegionalWorldCatalog::default();
        catalog.register(world).unwrap();

        assert!(
            catalog
                .without_environmental_conduction_metadata()
                .iter()
                .next()
                .unwrap()
                .1
                .biomes()[0]
                .destructibles()
                .is_none()
        );
    }
}
