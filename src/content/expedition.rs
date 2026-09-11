use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::facility::{
    FacilityBlueprint, FacilityBuildError, FacilityState, SecurityAlarmProfileError,
};
use crate::item::{ItemCatalog, ItemId, ItemKind};
use crate::loot::{LootCatalog, LootTableId, MAX_DRAWS};
use crate::social::{LocalAlertProfileError, SocialGroupId, WitnessProfileError};
use crate::world::GridPos;
use crate::world::generation::{RoomsGenerator, RoomsGeneratorConfig};

use super::ContentId;

pub type ExpeditionId = ContentId;
pub const MAX_ZONE_SIDE: usize = 256;
pub const MAX_ROOMS: usize = 64;
pub const MAX_PLACEMENT_ATTEMPTS: usize = 100_000;
pub const MAX_FACILITY_MATERIAL_SPAWNS: usize = 256;
pub const MAX_PLAYER_PROPERTY_AUTHORIZATIONS: usize = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZoneDefinition {
    pub id: ContentId,
    pub name: String,
    pub kind: ContentId,
    pub depth: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedZoneDefinition {
    pub zone: ZoneDefinition,
    pub generator: RoomsGeneratorConfig,
    pub seed_salt: u64,
    pub loot_table: Option<LootTableId>,
    pub loot_source: ContentId,
    pub loot_draws: u16,
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
            hub_facility: None,
            player_property_take_authorizations: Vec::new(),
        })
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
        }
    }
}

impl Error for ExpeditionDefinitionError {}
