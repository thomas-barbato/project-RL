use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::ai::AiProfile;
use crate::combat::{AttackArea, AttackProfile};
use crate::electronic_warfare::ElectronicSystemProfile;
use crate::entity::BodyComponentProfile;
use crate::facility::{
    FacilityBlueprint, FacilityBuildError, FacilityState, InstallationCapability,
    SecurityAlarmProfileError,
};
use crate::item::{ItemCatalog, ItemId, ItemKind};
use crate::loot::{LootCatalog, LootTableId, MAX_DRAWS};
use crate::progression::DefeatReward;
use crate::social::PlayerRelation;
use crate::social::{LocalAlertProfileError, SocialGroupId, WitnessProfileError};
use crate::stats::{
    BodyProfile, PhysicalRulesError, PrimaryAttributeRules, PrimaryAttributes,
    PrimaryAttributesError,
};
use crate::weapon::WeaponDefinitionError;
use crate::world::GridPos;
use crate::world::generation::{RoomsGenerator, RoomsGeneratorConfig};

use super::ContentId;

pub type ExpeditionId = ContentId;
pub const MAX_ZONE_SIDE: usize = 256;
pub const MAX_ROOMS: usize = 64;
pub const MAX_PLACEMENT_ATTEMPTS: usize = 100_000;
pub const MAX_FACILITY_MATERIAL_SPAWNS: usize = 256;
pub const MAX_PLAYER_PROPERTY_AUTHORIZATIONS: usize = 64;
pub const MAX_POPULATION_GROUPS: usize = 64;
pub const MAX_POPULATION_ACTORS: usize = 256;
pub const MAX_POPULATION_PATH_SEARCH: usize = 100_000;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZoneDefinition {
    pub id: ContentId,
    pub name: String,
    pub kind: ContentId,
    pub depth: u16,
}

/// Opt-in larger layout used by a new generation revision. Keeping it next to
/// the legacy layout lets deterministic suspensions rebuild the exact map they
/// started with while new runs adopt the expanded world.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExpandedWorldDefinition {
    pub hub_passage: GridPos,
    pub destination_generator: RoomsGeneratorConfig,
}

#[derive(Clone, PartialEq, Eq)]
pub struct GeneratedZoneDefinition {
    pub zone: ZoneDefinition,
    pub generator: RoomsGeneratorConfig,
    pub seed_salt: u64,
    pub loot_table: Option<LootTableId>,
    pub loot_source: ContentId,
    pub loot_draws: u16,
    pub population: Vec<PopulationGroupDefinition>,
}

// Empty population data is omitted so suspension versions created before
// data-driven populations retain their historical world fingerprint.
impl Debug for GeneratedZoneDefinition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut definition = formatter.debug_struct("GeneratedZoneDefinition");
        definition
            .field("zone", &self.zone)
            .field("generator", &self.generator)
            .field("seed_salt", &self.seed_salt)
            .field("loot_table", &self.loot_table)
            .field("loot_source", &self.loot_source)
            .field("loot_draws", &self.loot_draws);
        if !self.population.is_empty() {
            definition.field("population", &self.population);
        }
        definition.finish()
    }
}

/// One authored group in a generated zone population. Placement is resolved by
/// the zone provider, while every combat and AI parameter remains content data.
#[derive(Clone, PartialEq, Eq)]
pub struct PopulationGroupDefinition {
    count: u16,
    minimum_entrance_distance: u16,
    maximum_integrity: u16,
    attack: AttackProfile,
    ai: AiProfile,
    defeat_reward: Option<DefeatReward>,
    primary_attributes: Option<PrimaryAttributes>,
    body_profile: Option<BodyProfile>,
    body_components: Vec<BodyComponentProfile>,
    electronic_system: Option<ElectronicSystemProfile>,
    player_relation: PlayerRelation,
}

// Optional v32 attributes are omitted so catalogs stripped for older
// suspension versions retain their historical fingerprints exactly.
impl Debug for PopulationGroupDefinition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut group = formatter.debug_struct("PopulationGroupDefinition");
        group
            .field("count", &self.count)
            .field("minimum_entrance_distance", &self.minimum_entrance_distance)
            .field("maximum_integrity", &self.maximum_integrity)
            .field("attack", &self.attack)
            .field("ai", &self.ai)
            .field("defeat_reward", &self.defeat_reward);
        if let Some(attributes) = self.primary_attributes {
            group.field("primary_attributes", &attributes);
        }
        if let Some(body) = self.body_profile {
            group.field("body_profile", &body);
        }
        if !self.body_components.is_empty() {
            group.field("body_components", &self.body_components);
        }
        if let Some(electronic_system) = self.electronic_system {
            group.field("electronic_system", &electronic_system);
        }
        if self.player_relation != PlayerRelation::Neutral {
            group.field("player_relation", &self.player_relation);
        }
        group.finish()
    }
}

impl PopulationGroupDefinition {
    pub fn new(
        count: u16,
        minimum_entrance_distance: u16,
        maximum_integrity: u16,
        attack: AttackProfile,
        ai: AiProfile,
        defeat_reward: Option<DefeatReward>,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if count == 0 {
            return Err(ExpeditionDefinitionError::ZeroPopulationCount);
        }
        if maximum_integrity == 0 {
            return Err(ExpeditionDefinitionError::ZeroPopulationIntegrity);
        }
        if attack.range() == 0 || attack.damage().raw_total() == 0 {
            return Err(ExpeditionDefinitionError::InvalidPopulationAttack);
        }
        if usize::from(attack.range()) > MAX_ZONE_SIDE
            || matches!(
                attack.area(),
                AttackArea::Cone(cone)
                    if usize::from(cone.maximum_half_width()) > MAX_ZONE_SIDE
            )
        {
            return Err(ExpeditionDefinitionError::PopulationAttackBudgetExceeded);
        }
        if ai.preferred_attack_slot != 0 {
            return Err(ExpeditionDefinitionError::InvalidPopulationAttackSlot);
        }
        if ai.maximum_path_search > MAX_POPULATION_PATH_SEARCH {
            return Err(ExpeditionDefinitionError::PopulationPathBudgetExceeded);
        }
        if usize::from(ai.perception_radius) > MAX_ZONE_SIDE {
            return Err(ExpeditionDefinitionError::PopulationPerceptionBudgetExceeded);
        }
        if ai
            .maximum_pursuit_distance()
            .is_some_and(|distance| usize::from(distance) > MAX_ZONE_SIDE)
        {
            return Err(ExpeditionDefinitionError::PopulationPursuitBudgetExceeded);
        }
        if ai.preferred_minimum_distance > attack.range() {
            return Err(ExpeditionDefinitionError::InvalidPreferredDistance);
        }
        Ok(Self {
            count,
            minimum_entrance_distance,
            maximum_integrity,
            attack,
            ai,
            defeat_reward,
            primary_attributes: None,
            body_profile: None,
            body_components: Vec::new(),
            electronic_system: None,
            player_relation: PlayerRelation::Neutral,
        })
    }

    pub fn with_primary_attributes(
        mut self,
        attributes: PrimaryAttributes,
    ) -> Result<Self, ExpeditionDefinitionError> {
        attributes
            .validate_absolute(PrimaryAttributeRules::default())
            .map_err(ExpeditionDefinitionError::InvalidPopulationAttributes)?;
        self.primary_attributes = Some(attributes);
        Ok(self)
    }

    pub const fn with_body_profile(mut self, body: BodyProfile) -> Self {
        self.body_profile = Some(body);
        self
    }

    pub fn with_body_components(
        mut self,
        components: impl IntoIterator<Item = BodyComponentProfile>,
    ) -> Self {
        self.body_components = components.into_iter().collect();
        self
    }

    pub const fn with_electronic_system(mut self, profile: ElectronicSystemProfile) -> Self {
        self.electronic_system = Some(profile);
        self
    }

    pub const fn with_player_relation(mut self, relation: PlayerRelation) -> Self {
        self.player_relation = relation;
        self
    }

    pub const fn count(&self) -> u16 {
        self.count
    }

    pub const fn minimum_entrance_distance(&self) -> u16 {
        self.minimum_entrance_distance
    }

    pub const fn maximum_integrity(&self) -> u16 {
        self.maximum_integrity
    }

    pub const fn attack(&self) -> AttackProfile {
        self.attack
    }

    pub const fn ai(&self) -> AiProfile {
        self.ai
    }

    pub const fn defeat_reward(&self) -> Option<DefeatReward> {
        self.defeat_reward
    }

    pub const fn primary_attributes(&self) -> Option<PrimaryAttributes> {
        self.primary_attributes
    }

    pub const fn body_profile(&self) -> Option<BodyProfile> {
        self.body_profile
    }

    pub fn body_components(&self) -> &[BodyComponentProfile] {
        &self.body_components
    }

    pub const fn electronic_system(&self) -> Option<ElectronicSystemProfile> {
        self.electronic_system
    }

    pub const fn player_relation(&self) -> PlayerRelation {
        self.player_relation
    }

    pub(crate) fn remove_pursuit_lifecycle(&mut self) {
        self.ai = self.ai.without_pursuit_lifecycle();
    }

    pub(crate) fn remove_primary_attributes(&mut self) {
        self.primary_attributes = None;
    }

    pub(crate) fn remove_physical_metadata(&mut self) {
        self.body_profile = None;
        self.attack = self.attack.without_melee_impact();
    }

    pub(crate) fn remove_melee_skill_body_metadata(&mut self) {
        self.body_profile = self
            .body_profile
            .map(BodyProfile::without_melee_skill_metadata);
    }

    pub(crate) fn remove_ranged_skill_body_metadata(&mut self) {
        self.body_profile = self
            .body_profile
            .map(BodyProfile::without_ranged_skill_metadata);
        self.body_components.clear();
    }

    pub(crate) fn remove_electronic_system_metadata(&mut self) {
        self.electronic_system = None;
    }

    pub(crate) fn remove_player_relation_metadata(&mut self) {
        self.player_relation = PlayerRelation::Neutral;
    }

    pub(crate) fn remove_preparation_disruption_metadata(&mut self) {
        self.attack = self.attack.without_preparation_disruption();
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct FacilityMaterialSpawn {
    pub position: GridPos,
    pub item: ItemId,
    pub quantity: u16,
    pub owner: Option<SocialGroupId>,
}

impl Debug for FacilityMaterialSpawn {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut material = formatter.debug_struct("FacilityMaterialSpawn");
        material
            .field("position", &self.position)
            .field("item", &self.item)
            .field("quantity", &self.quantity);
        if let Some(owner) = &self.owner {
            material.field("owner", owner);
        }
        material.finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FacilityDefinition {
    pub blueprint: FacilityBlueprint,
    pub materials: Vec<FacilityMaterialSpawn>,
}

impl FacilityDefinition {
    pub fn new(
        blueprint: FacilityBlueprint,
        materials: Vec<FacilityMaterialSpawn>,
    ) -> Result<Self, ExpeditionDefinitionError> {
        FacilityState::validate_blueprint(&blueprint)
            .map_err(ExpeditionDefinitionError::InvalidFacility)?;
        if materials.len() > MAX_FACILITY_MATERIAL_SPAWNS
            || materials.iter().any(|material| material.quantity == 0)
        {
            return Err(ExpeditionDefinitionError::InvalidFacilityMaterials);
        }
        Ok(Self {
            blueprint,
            materials,
        })
    }

    fn validate_references(&self, items: &ItemCatalog) -> Result<(), ExpeditionDefinitionError> {
        for item in self
            .blueprint
            .repair_orders
            .iter()
            .map(|order| &order.required_item)
            .chain(self.materials.iter().map(|material| &material.item))
        {
            let definition = items
                .get(item)
                .ok_or_else(|| ExpeditionDefinitionError::UnknownFacilityItem(item.clone()))?;
            if definition.kind() != ItemKind::Material {
                return Err(ExpeditionDefinitionError::FacilityItemIsNotMaterial(
                    item.clone(),
                ));
            }
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct ExpeditionDefinition {
    id: ExpeditionId,
    pub hub: ZoneDefinition,
    pub destination: GeneratedZoneDefinition,
    pub hub_passage: GridPos,
    pub expanded_world: Option<ExpandedWorldDefinition>,
    pub hub_facility: Option<FacilityDefinition>,
    player_property_take_authorizations: Vec<SocialGroupId>,
}

// Optional v5 data is omitted when absent so a stripped catalog retains the
// exact pre-v5 Debug representation used by v4 suspension fingerprints.
impl Debug for ExpeditionDefinition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut definition = formatter.debug_struct("ExpeditionDefinition");
        definition
            .field("id", &self.id)
            .field("hub", &self.hub)
            .field("destination", &self.destination)
            .field("hub_passage", &self.hub_passage);
        if let Some(expanded_world) = &self.expanded_world {
            definition.field("expanded_world", expanded_world);
        }
        if let Some(facility) = &self.hub_facility {
            definition.field("hub_facility", facility);
        }
        if !self.player_property_take_authorizations.is_empty() {
            definition.field(
                "player_property_take_authorizations",
                &self.player_property_take_authorizations,
            );
        }
        definition.finish()
    }
}

impl ExpeditionDefinition {
    pub fn new(
        id: ExpeditionId,
        hub: ZoneDefinition,
        destination: GeneratedZoneDefinition,
        hub_passage: GridPos,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if hub.name.trim().is_empty() || destination.zone.name.trim().is_empty() {
            return Err(ExpeditionDefinitionError::EmptyZoneName);
        }
        if hub.id == destination.zone.id {
            return Err(ExpeditionDefinitionError::DuplicateZoneId(hub.id));
        }
        if hub_passage.x < 0 || hub_passage.y < 0 {
            return Err(ExpeditionDefinitionError::NegativePassage);
        }
        if destination.loot_draws > MAX_DRAWS {
            return Err(ExpeditionDefinitionError::TooManyLootDraws);
        }
        if destination.generator.width > MAX_ZONE_SIDE
            || destination.generator.height > MAX_ZONE_SIDE
            || destination.generator.room_count > MAX_ROOMS
            || destination.generator.placement_attempts > MAX_PLACEMENT_ATTEMPTS
        {
            return Err(ExpeditionDefinitionError::GeneratorBudgetExceeded);
        }
        if destination.population.len() > MAX_POPULATION_GROUPS
            || destination
                .population
                .iter()
                .try_fold(0_usize, |total, group| {
                    total.checked_add(usize::from(group.count()))
                })
                .is_none_or(|total| total > MAX_POPULATION_ACTORS)
        {
            return Err(ExpeditionDefinitionError::PopulationBudgetExceeded);
        }
        if destination.loot_draws > 0 && destination.loot_table.is_none() {
            return Err(ExpeditionDefinitionError::MissingLootTable);
        }
        RoomsGenerator::new(destination.generator)
            .map_err(|error| ExpeditionDefinitionError::InvalidGenerator(error.to_string()))?;
        Ok(Self {
            id,
            hub,
            destination,
            hub_passage,
            expanded_world: None,
            hub_facility: None,
            player_property_take_authorizations: Vec::new(),
        })
    }

    pub fn with_expanded_world(
        mut self,
        expanded_world: ExpandedWorldDefinition,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if expanded_world.hub_passage.x < 0 || expanded_world.hub_passage.y < 0 {
            return Err(ExpeditionDefinitionError::NegativePassage);
        }
        validate_generator_budget(expanded_world.destination_generator)?;
        RoomsGenerator::new(expanded_world.destination_generator)
            .map_err(|error| ExpeditionDefinitionError::InvalidGenerator(error.to_string()))?;
        self.expanded_world = Some(expanded_world);
        Ok(self)
    }

    pub fn hub_passage_for_expanded_world(&self, expanded: bool) -> GridPos {
        if expanded {
            self.expanded_world
                .as_ref()
                .map_or(self.hub_passage, |world| world.hub_passage)
        } else {
            self.hub_passage
        }
    }

    pub fn destination_generator_for_expanded_world(&self, expanded: bool) -> RoomsGeneratorConfig {
        if expanded {
            self.expanded_world
                .as_ref()
                .map_or(self.destination.generator, |world| {
                    world.destination_generator
                })
        } else {
            self.destination.generator
        }
    }

    pub fn with_hub_facility(mut self, facility: FacilityDefinition) -> Self {
        self.hub_facility = Some(facility);
        self
    }

    pub fn with_player_property_take_authorizations(
        mut self,
        authorizations: Vec<SocialGroupId>,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if authorizations.len() > MAX_PLAYER_PROPERTY_AUTHORIZATIONS
            || authorizations
                .iter()
                .enumerate()
                .any(|(index, owner)| authorizations[..index].contains(owner))
        {
            return Err(ExpeditionDefinitionError::InvalidPlayerPropertyAuthorizations);
        }
        self.player_property_take_authorizations = authorizations;
        Ok(self)
    }

    pub fn player_property_take_authorizations(&self) -> &[SocialGroupId] {
        &self.player_property_take_authorizations
    }

    pub fn id(&self) -> &ExpeditionId {
        &self.id
    }

    pub fn validate_references(
        &self,
        loot: &LootCatalog,
        items: &ItemCatalog,
    ) -> Result<(), ExpeditionDefinitionError> {
        if let Some(table) = &self.destination.loot_table
            && loot.get(table).is_none()
        {
            return Err(ExpeditionDefinitionError::UnknownLootTable(table.clone()));
        }
        if let Some(facility) = &self.hub_facility {
            facility.validate_references(items)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ExpeditionCatalog {
    definitions: BTreeMap<ExpeditionId, ExpeditionDefinition>,
}

impl ExpeditionCatalog {
    pub fn register(
        &mut self,
        definition: ExpeditionDefinition,
    ) -> Result<(), ExpeditionDefinitionError> {
        if self.definitions.contains_key(definition.id()) {
            return Err(ExpeditionDefinitionError::DuplicateExpedition(
                definition.id().clone(),
            ));
        }
        self.definitions.insert(definition.id().clone(), definition);
        Ok(())
    }

    pub fn get(&self, id: &ExpeditionId) -> Option<&ExpeditionDefinition> {
        self.definitions.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ExpeditionId, &ExpeditionDefinition)> {
        self.definitions.iter()
    }

    pub fn without_facilities(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                definition.hub_facility = None;
                definition.player_property_take_authorizations.clear();
                definition.destination.population.clear();
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v6 social metadata while preserving the v5 facility graph and
    /// its historical Debug fingerprint.
    pub fn without_social_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                definition.destination.population.clear();
                if let Some(facility) = &mut definition.hub_facility {
                    facility.blueprint.owner = None;
                    for installation in &mut facility.blueprint.installations {
                        installation.security_alarm_profile = None;
                    }
                    for worker in &mut facility.blueprint.workers {
                        worker.affiliation = None;
                        worker.witness_profile = None;
                        worker.local_alert_profile = None;
                    }
                    for material in &mut facility.materials {
                        material.owner = None;
                    }
                }
                definition.player_property_take_authorizations.clear();
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v7 local-alert reactions while preserving the v6 ownership and
    /// individual-witness rules used by existing suspension fingerprints.
    pub fn without_local_alert_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                definition.destination.population.clear();
                if let Some(facility) = &mut definition.hub_facility {
                    for installation in &mut facility.blueprint.installations {
                        installation.security_alarm_profile = None;
                    }
                    for worker in &mut facility.blueprint.workers {
                        worker.local_alert_profile = None;
                    }
                }
                definition.player_property_take_authorizations.clear();
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v8 installed-alarm behavior while preserving the exact v7
    /// installation graph and world-catalogue fingerprint.
    pub fn without_security_alarm_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                definition.destination.population.clear();
                if let Some(facility) = &mut definition.hub_facility {
                    for installation in &mut facility.blueprint.installations {
                        installation.security_alarm_profile = None;
                    }
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v9 alarm responses while retaining v8 sensor detection and its
    /// historical catalogue fingerprint.
    pub fn without_security_alarm_response_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                definition.destination.population.clear();
                if let Some(facility) = &mut definition.hub_facility {
                    for installation in &mut facility.blueprint.installations {
                        installation.security_alarm_profile = installation
                            .security_alarm_profile
                            .as_ref()
                            .map(|profile| profile.without_responses());
                    }
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v11 generated-zone population data while preserving the exact
    /// catalogue used by suspension versions 9 and 10.
    pub fn without_population_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                definition.destination.population.clear();
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes the expanded v12 layout so older suspension fingerprints and
    /// their smaller deterministic maps remain reproducible.
    pub fn without_expanded_world_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                definition.expanded_world = None;
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes the v13 territorial leash while keeping v12 population and map
    /// definitions byte-for-byte compatible for deterministic replay.
    pub fn without_pursuit_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                for group in &mut definition.destination.population {
                    group.ai = group.ai.without_pursuit_limit();
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes the v17 pursuit lifecycle while retaining the v13 territorial
    /// leash used by older deterministic suspensions.
    pub fn without_pursuit_lifecycle_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                for group in &mut definition.destination.population {
                    group.remove_pursuit_lifecycle();
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v32 non-player attributes while keeping every older authored
    /// combat and AI field intact for deterministic suspension replay.
    pub fn without_primary_attribute_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                for group in &mut definition.destination.population {
                    group.remove_primary_attributes();
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes explicit v60 combat dispositions while retaining every older
    /// population field and its historical catalogue fingerprint.
    pub fn without_player_relation_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                for group in &mut definition.destination.population {
                    group.remove_player_relation_metadata();
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v33 body and Impact profiles while retaining the historical
    /// final integrity and attack damage used by older suspension versions.
    pub fn without_physical_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                for group in &mut definition.destination.population {
                    group.remove_physical_metadata();
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v42 displacement and locomotion capabilities while preserving
    /// the v33 body profiles used by existing deterministic suspensions.
    pub fn without_melee_skill_body_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                for group in &mut definition.destination.population {
                    group.remove_melee_skill_body_metadata();
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v43 suppression compatibility while preserving all earlier
    /// physical body metadata for deterministic historical suspensions.
    pub fn without_ranged_skill_body_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                for group in &mut definition.destination.population {
                    group.remove_ranged_skill_body_metadata();
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v51 electronic target profiles while retaining the exact
    /// population definitions used by older deterministic suspensions.
    pub fn without_electronic_system_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                for group in &mut definition.destination.population {
                    group.remove_electronic_system_metadata();
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v56 attack-side preparation disruption while retaining the
    /// exact population attacks used by older deterministic suspensions.
    pub fn without_preparation_disruption_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                for group in &mut definition.destination.population {
                    group.remove_preparation_disruption_metadata();
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v27 readable-terminal capabilities and any installation left
    /// empty by that removal. This preserves the exact installation graph used
    /// by earlier replay suspensions after current content gains terminals.
    pub fn without_data_terminal_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                if let Some(facility) = &mut definition.hub_facility {
                    for installation in &mut facility.blueprint.installations {
                        installation.capabilities.retain(|capability| {
                            !matches!(capability, InstallationCapability::DataTerminal { .. })
                        });
                    }
                    facility
                        .blueprint
                        .installations
                        .retain(|installation| !installation.capabilities.is_empty());
                    let known_installations = facility
                        .blueprint
                        .installations
                        .iter()
                        .map(|installation| installation.id.clone())
                        .collect::<Vec<_>>();
                    for installation in &mut facility.blueprint.installations {
                        installation
                            .dependencies
                            .retain(|dependency| known_installations.contains(dependency));
                    }
                    facility
                        .blueprint
                        .repair_orders
                        .retain(|order| known_installations.contains(&order.target));
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }
}

fn validate_generator_budget(
    generator: RoomsGeneratorConfig,
) -> Result<(), ExpeditionDefinitionError> {
    if generator.width > MAX_ZONE_SIDE
        || generator.height > MAX_ZONE_SIDE
        || generator.room_count > MAX_ROOMS
        || generator.placement_attempts > MAX_PLACEMENT_ATTEMPTS
    {
        return Err(ExpeditionDefinitionError::GeneratorBudgetExceeded);
    }
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExpeditionDefinitionError {
    EmptyZoneName,
    DuplicateZoneId(ContentId),
    NegativePassage,
    TooManyLootDraws,
    GeneratorBudgetExceeded,
    MissingLootTable,
    UnknownLootTable(LootTableId),
    InvalidGenerator(String),
    DuplicateExpedition(ExpeditionId),
    InvalidFacility(FacilityBuildError),
    InvalidFacilityMaterials,
    UnknownFacilityItem(ItemId),
    FacilityItemIsNotMaterial(ItemId),
    InvalidWitnessProfile(WitnessProfileError),
    InvalidLocalAlertProfile(LocalAlertProfileError),
    InvalidSecurityAlarmProfile(SecurityAlarmProfileError),
    InvalidPlayerPropertyAuthorizations,
    ZeroPopulationCount,
    ZeroPopulationIntegrity,
    InvalidPopulationAttack,
    InvalidPopulationAttackDefinition(WeaponDefinitionError),
    InvalidPopulationAttributes(PrimaryAttributesError),
    InvalidPopulationBody(PhysicalRulesError),
    InvalidPopulationComponent(String),
    InvalidPopulationElectronicSystem(String),
    InvalidPopulationAttackSlot,
    PopulationAttackBudgetExceeded,
    PopulationPathBudgetExceeded,
    PopulationPerceptionBudgetExceeded,
    ZeroPopulationPursuitDistance,
    PopulationPursuitBudgetExceeded,
    InvalidPopulationPursuitLifecycle,
    InvalidPreferredDistance,
    PopulationBudgetExceeded,
}

impl Display for ExpeditionDefinitionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyZoneName => write!(formatter, "zone names must not be empty"),
            Self::DuplicateZoneId(id) => write!(formatter, "duplicate zone ID '{id}'"),
            Self::NegativePassage => {
                write!(formatter, "hub passage coordinates must be non-negative")
            }
            Self::TooManyLootDraws => {
                write!(formatter, "zone loot cannot exceed {MAX_DRAWS} draws")
            }
            Self::GeneratorBudgetExceeded => {
                write!(formatter, "generated zone exceeds its size or work budget")
            }
            Self::MissingLootTable => write!(formatter, "positive loot draws require a loot table"),
            Self::UnknownLootTable(id) => write!(formatter, "unknown loot table '{id}'"),
            Self::InvalidGenerator(error) => write!(formatter, "invalid rooms generator: {error}"),
            Self::DuplicateExpedition(id) => write!(formatter, "duplicate expedition ID '{id}'"),
            Self::InvalidFacility(error) => write!(formatter, "invalid hub facility: {error}"),
            Self::InvalidFacilityMaterials => {
                write!(formatter, "invalid or excessive facility material spawns")
            }
            Self::UnknownFacilityItem(id) => write!(formatter, "unknown facility item '{id}'"),
            Self::FacilityItemIsNotMaterial(id) => {
                write!(formatter, "facility item '{id}' is not a material")
            }
            Self::InvalidWitnessProfile(error) => write!(formatter, "{error}"),
            Self::InvalidLocalAlertProfile(error) => write!(formatter, "{error}"),
            Self::InvalidSecurityAlarmProfile(error) => write!(formatter, "{error}"),
            Self::InvalidPlayerPropertyAuthorizations => write!(
                formatter,
                "player property authorizations contain duplicates or exceed {MAX_PLAYER_PROPERTY_AUTHORIZATIONS} entries"
            ),
            Self::ZeroPopulationCount => {
                write!(formatter, "population group count must be positive")
            }
            Self::ZeroPopulationIntegrity => {
                write!(formatter, "population integrity must be positive")
            }
            Self::InvalidPopulationAttack => {
                write!(
                    formatter,
                    "population attacks require positive range and damage"
                )
            }
            Self::InvalidPopulationAttackDefinition(error) => {
                write!(formatter, "invalid population attack: {error}")
            }
            Self::InvalidPopulationAttributes(error) => {
                write!(formatter, "invalid population primary attributes: {error}")
            }
            Self::InvalidPopulationBody(error) => {
                write!(formatter, "invalid population body profile: {error}")
            }
            Self::InvalidPopulationComponent(error) => {
                write!(formatter, "invalid population body component: {error}")
            }
            Self::InvalidPopulationElectronicSystem(error) => {
                write!(formatter, "invalid population electronic system: {error}")
            }
            Self::InvalidPopulationAttackSlot => {
                write!(
                    formatter,
                    "population AI must select its only attack slot (0)"
                )
            }
            Self::PopulationAttackBudgetExceeded => write!(
                formatter,
                "population attack range or cone width exceeds the maximum zone side {MAX_ZONE_SIDE}"
            ),
            Self::PopulationPathBudgetExceeded => write!(
                formatter,
                "population AI path search exceeds {MAX_POPULATION_PATH_SEARCH} nodes"
            ),
            Self::PopulationPerceptionBudgetExceeded => write!(
                formatter,
                "population perception radius exceeds the maximum zone side {MAX_ZONE_SIDE}"
            ),
            Self::ZeroPopulationPursuitDistance => {
                write!(formatter, "population pursuit distance must be positive")
            }
            Self::PopulationPursuitBudgetExceeded => write!(
                formatter,
                "population pursuit distance exceeds the maximum zone side {MAX_ZONE_SIDE}"
            ),
            Self::InvalidPopulationPursuitLifecycle => write!(
                formatter,
                "population pursuit, search and cooldown durations must be positive"
            ),
            Self::InvalidPreferredDistance => write!(
                formatter,
                "population preferred distance cannot exceed attack range"
            ),
            Self::PopulationBudgetExceeded => write!(
                formatter,
                "generated zone population exceeds {MAX_POPULATION_GROUPS} groups or {MAX_POPULATION_ACTORS} actors"
            ),
        }
    }
}

impl Error for ExpeditionDefinitionError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::DamageType;
    use crate::stats::{PrimaryAttribute, PrimaryAttributesError};

    #[test]
    fn non_player_attributes_are_optional_and_reject_out_of_bounds_values() {
        let group = PopulationGroupDefinition::new(
            1,
            0,
            5,
            AttackProfile::melee(DamageType::Kinetic, 1),
            AiProfile::hunter(8, 0),
            None,
        )
        .unwrap();
        assert_eq!(group.primary_attributes(), None);

        let error = group
            .with_primary_attributes(PrimaryAttributes::new(11, 5, 5, 5, 5))
            .unwrap_err();
        assert!(matches!(
            error,
            ExpeditionDefinitionError::InvalidPopulationAttributes(
                PrimaryAttributesError::OutsideAbsoluteRange {
                    attribute: PrimaryAttribute::Power,
                    value: 11,
                    ..
                }
            )
        ));
    }
}
