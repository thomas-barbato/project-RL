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
use crate::social::{
    LocalAlertProfileError, PropertyReportProfileError, SocialGroupId, WitnessProfileError,
};
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
pub const MAX_MERCHANT_OFFERS: usize = 64;
pub const MAX_GAMBLE_RANK: u16 = 64;
pub const MAX_CLINIC_PATH_SEARCH: usize = 100_000;
pub const MAX_HUB_RESIDENTS: usize = 32;
pub const MAX_HUB_QUESTS: usize = 16;
pub const MAX_QUEST_PREREQUISITES: usize = 8;
pub const MAX_QUEST_REWARD_ITEMS: usize = 8;
pub const MAX_QUEST_WORLD_STATES: usize = 8;
pub const MAX_QUEST_WORLD_EFFECTS: usize = 8;
pub const MAX_POPULATION_TAGS: usize = 16;
pub const MAX_RESIDENT_PATH_SEARCH: usize = 100_000;
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
    tags: Vec<ContentId>,
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
        if !self.tags.is_empty() {
            group.field("tags", &self.tags);
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
            tags: Vec::new(),
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

    pub fn with_tags(mut self, tags: Vec<ContentId>) -> Result<Self, ExpeditionDefinitionError> {
        if tags.len() > MAX_POPULATION_TAGS
            || tags
                .iter()
                .enumerate()
                .any(|(index, tag)| tags[..index].contains(tag))
        {
            return Err(ExpeditionDefinitionError::InvalidPopulationTags);
        }
        self.tags = tags;
        Ok(self)
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

    pub fn tags(&self) -> &[ContentId] {
        &self.tags
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerchantOfferDefinition {
    pub item: ItemId,
    pub initial_stock: u16,
    pub buy_price: u32,
    pub sell_price: u32,
    pub minimum_depth: u16,
    pub maximum_depth: Option<u16>,
}

impl MerchantOfferDefinition {
    pub fn available_at_depth(&self, depth: u16) -> bool {
        depth >= self.minimum_depth && self.maximum_depth.is_none_or(|maximum| depth <= maximum)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerchantGambleDefinition {
    pub item: ItemId,
    pub initial_stock: u16,
    pub price: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GambleScalingDefinition {
    pub player_levels_per_rank: u16,
    pub zone_depths_per_rank: u16,
    pub maximum_rank: u16,
}

impl GambleScalingDefinition {
    pub const fn new(
        player_levels_per_rank: u16,
        zone_depths_per_rank: u16,
        maximum_rank: u16,
    ) -> Option<Self> {
        let definition = Self {
            player_levels_per_rank,
            zone_depths_per_rank,
            maximum_rank,
        };
        if !definition.is_valid() {
            return None;
        }
        Some(definition)
    }

    pub const fn is_valid(self) -> bool {
        self.player_levels_per_rank > 0
            && self.zone_depths_per_rank > 0
            && self.maximum_rank > 0
            && self.maximum_rank <= MAX_GAMBLE_RANK
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MerchantDefinition {
    pub position: GridPos,
    pub maximum_integrity: u16,
    pub initial_credits: u32,
    pub offers: Vec<MerchantOfferDefinition>,
    pub gambles: Vec<MerchantGambleDefinition>,
    pub gamble_scaling: GambleScalingDefinition,
}

impl MerchantDefinition {
    pub fn new(
        position: GridPos,
        maximum_integrity: u16,
        initial_credits: u32,
        offers: Vec<MerchantOfferDefinition>,
        gambles: Vec<MerchantGambleDefinition>,
        gamble_scaling: GambleScalingDefinition,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if maximum_integrity == 0
            || offers.is_empty()
            || offers.len() > MAX_MERCHANT_OFFERS
            || offers.iter().any(|offer| {
                offer.initial_stock == 0
                    || offer.buy_price == 0
                    || offer.sell_price == 0
                    || offer.sell_price > offer.buy_price
                    || offer
                        .maximum_depth
                        .is_some_and(|maximum| maximum < offer.minimum_depth)
            })
            || offers
                .iter()
                .enumerate()
                .any(|(index, offer)| offers[..index].iter().any(|known| known.item == offer.item))
            || gambles.is_empty()
            || gambles.len() > MAX_MERCHANT_OFFERS
            || gambles
                .iter()
                .any(|gamble| gamble.initial_stock == 0 || gamble.price == 0)
            || !gamble_scaling.is_valid()
        {
            return Err(ExpeditionDefinitionError::InvalidMerchant);
        }
        Ok(Self {
            position,
            maximum_integrity,
            initial_credits,
            offers,
            gambles,
            gamble_scaling,
        })
    }

    pub(crate) fn validate_references(
        &self,
        items: &ItemCatalog,
    ) -> Result<(), ExpeditionDefinitionError> {
        for offer in &self.offers {
            if items.get(&offer.item).is_none() {
                return Err(ExpeditionDefinitionError::UnknownMerchantItem(
                    offer.item.clone(),
                ));
            }
        }
        for gamble in &self.gambles {
            let definition = items.get(&gamble.item).ok_or_else(|| {
                ExpeditionDefinitionError::UnknownMerchantItem(gamble.item.clone())
            })?;
            if definition.kind() != ItemKind::Armor {
                return Err(ExpeditionDefinitionError::MerchantGambleItemIsNotArmor(
                    gamble.item.clone(),
                ));
            }
        }
        Ok(())
    }
}

/// One bounded local care provider. Treatment prices and the short work/break
/// routine are authored by content; the simulation only performs atomic
/// payment, restoration and deterministic movement between the two anchors.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClinicDefinition {
    pub work_position: GridPos,
    pub break_position: GridPos,
    pub maximum_integrity: u16,
    pub initial_credits: u32,
    pub maximum_restoration: u16,
    pub price_per_point: u32,
    pub work_turns: u16,
    pub break_turns: u16,
    pub maximum_path_search: usize,
}

impl ClinicDefinition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        work_position: GridPos,
        break_position: GridPos,
        maximum_integrity: u16,
        initial_credits: u32,
        maximum_restoration: u16,
        price_per_point: u32,
        work_turns: u16,
        break_turns: u16,
        maximum_path_search: usize,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if maximum_integrity == 0
            || maximum_restoration == 0
            || price_per_point == 0
            || work_turns == 0
            || break_turns == 0
            || maximum_path_search == 0
            || maximum_path_search > MAX_CLINIC_PATH_SEARCH
            || u32::from(maximum_restoration)
                .checked_mul(price_per_point)
                .is_none()
        {
            return Err(ExpeditionDefinitionError::InvalidClinic);
        }
        Ok(Self {
            work_position,
            break_position,
            maximum_integrity,
            initial_credits,
            maximum_restoration,
            price_per_point,
            work_turns,
            break_turns,
            maximum_path_search,
        })
    }
}

/// One non-service resident following a small, observable local circuit.
/// The two anchors and dwell times are authored by content so this remains a
/// bounded routine rather than a simulated daily life.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResidentDefinition {
    pub residence_position: GridPos,
    pub gathering_position: GridPos,
    pub maximum_integrity: u16,
    pub residence_turns: u16,
    pub gathering_turns: u16,
    pub maximum_path_search: usize,
}

impl ResidentDefinition {
    pub const fn new(
        residence_position: GridPos,
        gathering_position: GridPos,
        maximum_integrity: u16,
        residence_turns: u16,
        gathering_turns: u16,
        maximum_path_search: usize,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if residence_position.x == gathering_position.x
            && residence_position.y == gathering_position.y
            || maximum_integrity == 0
            || residence_turns == 0
            || gathering_turns == 0
            || maximum_path_search == 0
            || maximum_path_search > MAX_RESIDENT_PATH_SEARCH
        {
            return Err(ExpeditionDefinitionError::InvalidResident);
        }
        Ok(Self {
            residence_position,
            gathering_position,
            maximum_integrity,
            residence_turns,
            gathering_turns,
            maximum_path_search,
        })
    }
}

/// Stable delivery contract authored by a content package. The simulation
/// owns its accepted/completed state; this definition only supplies immutable
/// objective and reward data.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DeliveryQuestDefinition {
    pub id: ContentId,
    pub title_key: String,
    pub summary_key: String,
    pub required_item: ItemId,
    pub required_quantity: u16,
    pub reward_credits: u32,
}

impl DeliveryQuestDefinition {
    pub fn new(
        id: ContentId,
        title_key: String,
        summary_key: String,
        required_item: ItemId,
        required_quantity: u16,
        reward_credits: u32,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if title_key.trim().is_empty() || summary_key.trim().is_empty() || required_quantity == 0 {
            return Err(ExpeditionDefinitionError::InvalidDeliveryQuest);
        }
        Ok(Self {
            id,
            title_key,
            summary_key,
            required_item,
            required_quantity,
            reward_credits,
        })
    }
}

/// Exploration contract that counts distinct zones first entered after the
/// quest was accepted. The provider's zone and every zone already visited at
/// acceptance form the baseline and can never inflate this objective.
#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ExplorationQuestDefinition {
    pub id: ContentId,
    pub title_key: String,
    pub summary_key: String,
    pub required_zones: u16,
    pub reward_credits: u32,
    #[serde(default)]
    pub qualifying_records: Vec<ContentId>,
}

// Omit absent v85 metadata from historical content fingerprints.
impl Debug for ExplorationQuestDefinition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut state = formatter.debug_struct("ExplorationQuestDefinition");
        state
            .field("id", &self.id)
            .field("title_key", &self.title_key)
            .field("summary_key", &self.summary_key)
            .field("required_zones", &self.required_zones)
            .field("reward_credits", &self.reward_credits);
        if !self.qualifying_records.is_empty() {
            state.field("qualifying_records", &self.qualifying_records);
        }
        state.finish()
    }
}

impl ExplorationQuestDefinition {
    pub fn new(
        id: ContentId,
        title_key: String,
        summary_key: String,
        required_zones: u16,
        reward_credits: u32,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if title_key.trim().is_empty() || summary_key.trim().is_empty() || required_zones == 0 {
            return Err(ExpeditionDefinitionError::InvalidExplorationQuest);
        }
        Ok(Self {
            id,
            title_key,
            summary_key,
            required_zones,
            reward_credits,
            qualifying_records: Vec::new(),
        })
    }

    /// When configured, only a successful consultation of one of these
    /// records in a newly visited zone counts that zone as explored.
    pub fn with_qualifying_records(
        mut self,
        records: Vec<ContentId>,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if records.is_empty()
            || records.len() > 16
            || records
                .iter()
                .enumerate()
                .any(|(index, record)| records[..index].contains(record))
        {
            return Err(ExpeditionDefinitionError::InvalidExplorationQuest);
        }
        self.qualifying_records = records;
        Ok(self)
    }
}

/// Interaction contract completed by successfully consulting a data terminal
/// that exposes the authored record. Prior discoveries do not count: the
/// simulation observes an actual terminal access after quest acceptance.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DataRecordQuestDefinition {
    pub id: ContentId,
    pub title_key: String,
    pub summary_key: String,
    pub record: ContentId,
    pub reward_credits: u32,
}

impl DataRecordQuestDefinition {
    pub fn new(
        id: ContentId,
        title_key: String,
        summary_key: String,
        record: ContentId,
        reward_credits: u32,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if title_key.trim().is_empty() || summary_key.trim().is_empty() {
            return Err(ExpeditionDefinitionError::InvalidDataRecordQuest);
        }
        Ok(Self {
            id,
            title_key,
            summary_key,
            record,
            reward_credits,
        })
    }
}

/// Combat contract keyed by a stable actor tag rather than by glyph, stats,
/// affiliation or a particular runtime entity ID.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DefeatTargetsQuestDefinition {
    pub id: ContentId,
    pub title_key: String,
    pub summary_key: String,
    pub target_tag: ContentId,
    pub required_quantity: u16,
    pub reward_credits: u32,
}

impl DefeatTargetsQuestDefinition {
    pub fn new(
        id: ContentId,
        title_key: String,
        summary_key: String,
        target_tag: ContentId,
        required_quantity: u16,
        reward_credits: u32,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if title_key.trim().is_empty() || summary_key.trim().is_empty() || required_quantity == 0 {
            return Err(ExpeditionDefinitionError::InvalidDefeatTargetsQuest);
        }
        Ok(Self {
            id,
            title_key,
            summary_key,
            target_tag,
            required_quantity,
            reward_credits,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum QuestDefinition {
    Delivery(DeliveryQuestDefinition),
    ExploreZones(ExplorationQuestDefinition),
    AccessDataRecord(DataRecordQuestDefinition),
    DefeatTargets(DefeatTargetsQuestDefinition),
}

impl QuestDefinition {
    pub const fn id(&self) -> &ContentId {
        match self {
            Self::Delivery(definition) => &definition.id,
            Self::ExploreZones(definition) => &definition.id,
            Self::AccessDataRecord(definition) => &definition.id,
            Self::DefeatTargets(definition) => &definition.id,
        }
    }

    pub fn title_key(&self) -> &str {
        match self {
            Self::Delivery(definition) => &definition.title_key,
            Self::ExploreZones(definition) => &definition.title_key,
            Self::AccessDataRecord(definition) => &definition.title_key,
            Self::DefeatTargets(definition) => &definition.title_key,
        }
    }

    pub fn summary_key(&self) -> &str {
        match self {
            Self::Delivery(definition) => &definition.summary_key,
            Self::ExploreZones(definition) => &definition.summary_key,
            Self::AccessDataRecord(definition) => &definition.summary_key,
            Self::DefeatTargets(definition) => &definition.summary_key,
        }
    }

    pub const fn reward_credits(&self) -> u32 {
        match self {
            Self::Delivery(definition) => definition.reward_credits,
            Self::ExploreZones(definition) => definition.reward_credits,
            Self::AccessDataRecord(definition) => definition.reward_credits,
            Self::DefeatTargets(definition) => definition.reward_credits,
        }
    }
}

impl From<DeliveryQuestDefinition> for QuestDefinition {
    fn from(value: DeliveryQuestDefinition) -> Self {
        Self::Delivery(value)
    }
}

impl From<ExplorationQuestDefinition> for QuestDefinition {
    fn from(value: ExplorationQuestDefinition) -> Self {
        Self::ExploreZones(value)
    }
}

impl From<DataRecordQuestDefinition> for QuestDefinition {
    fn from(value: DataRecordQuestDefinition) -> Self {
        Self::AccessDataRecord(value)
    }
}

impl From<DefeatTargetsQuestDefinition> for QuestDefinition {
    fn from(value: DefeatTargetsQuestDefinition) -> Self {
        Self::DefeatTargets(value)
    }
}

/// How an authored hub quest obtains its NPC. `Existing` adds the quest to a
/// merchant, healer, resident or worker already starting at this position;
/// `Contact` creates a dedicated neutral quest NPC.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HubQuestProviderDefinition {
    Existing {
        position: GridPos,
    },
    Contact {
        position: GridPos,
        maximum_integrity: u16,
    },
}

impl HubQuestProviderDefinition {
    pub fn existing(position: GridPos) -> Result<Self, ExpeditionDefinitionError> {
        if position.x < 0 || position.y < 0 {
            return Err(ExpeditionDefinitionError::InvalidQuestProvider);
        }
        Ok(Self::Existing { position })
    }

    pub fn contact(
        position: GridPos,
        maximum_integrity: u16,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if position.x < 0 || position.y < 0 || maximum_integrity == 0 {
            return Err(ExpeditionDefinitionError::InvalidQuestProvider);
        }
        Ok(Self::Contact {
            position,
            maximum_integrity,
        })
    }

    pub const fn position(&self) -> GridPos {
        match self {
            Self::Existing { position } | Self::Contact { position, .. } => *position,
        }
    }
}

fn quest_providers_can_share(
    left: &HubQuestProviderDefinition,
    right: &HubQuestProviderDefinition,
) -> bool {
    match (left, right) {
        (
            HubQuestProviderDefinition::Existing { position: left },
            HubQuestProviderDefinition::Existing { position: right },
        ) => left == right,
        (
            HubQuestProviderDefinition::Contact {
                position: left,
                maximum_integrity: left_integrity,
            },
            HubQuestProviderDefinition::Contact {
                position: right,
                maximum_integrity: right_integrity,
            },
        ) => left == right && left_integrity == right_integrity,
        _ => false,
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct HubQuestDefinition {
    pub provider: HubQuestProviderDefinition,
    pub quest: QuestDefinition,
    pub prerequisites: Vec<ContentId>,
    pub required_world_states: Vec<ContentId>,
    pub completion_world_states: Vec<QuestWorldStateDefinition>,
    pub completion_world_effects: Vec<QuestWorldEffectDefinition>,
    pub choice_group: Option<ContentId>,
    pub choice_prompt_key: Option<String>,
    pub reward_experience: u64,
    pub reward_items: Vec<QuestItemRewardDefinition>,
}

// Empty v66/v67 consequence metadata is omitted so stripped definitions retain
// the exact historical Debug representation used by suspension fingerprints.
impl Debug for HubQuestDefinition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut state = formatter.debug_struct("HubQuestDefinition");
        state
            .field("provider", &self.provider)
            .field("quest", &self.quest)
            .field("prerequisites", &self.prerequisites)
            .field("choice_group", &self.choice_group)
            .field("choice_prompt_key", &self.choice_prompt_key)
            .field("reward_experience", &self.reward_experience)
            .field("reward_items", &self.reward_items);
        if !self.required_world_states.is_empty() {
            state.field("required_world_states", &self.required_world_states);
        }
        if !self.completion_world_states.is_empty() {
            state.field("completion_world_states", &self.completion_world_states);
        }
        if !self.completion_world_effects.is_empty() {
            state.field("completion_world_effects", &self.completion_world_effects);
        }
        state.finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct QuestItemRewardDefinition {
    pub item: ItemId,
    pub quantity: u16,
}

/// One durable, authored consequence activated when a quest is completed.
/// The state ID is simulation data; text keys only describe its visible result
/// and an optional contextual reply from the quest provider.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct QuestWorldStateDefinition {
    pub id: ContentId,
    pub summary_key: String,
    pub provider_dialogue_key: Option<String>,
}

impl QuestWorldStateDefinition {
    pub fn new(id: ContentId, summary_key: String) -> Result<Self, ExpeditionDefinitionError> {
        if summary_key.trim().is_empty() {
            return Err(ExpeditionDefinitionError::InvalidQuestWorldState);
        }
        Ok(Self {
            id,
            summary_key,
            provider_dialogue_key: None,
        })
    }

    pub fn with_provider_dialogue(
        mut self,
        dialogue_key: String,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if dialogue_key.trim().is_empty() {
            return Err(ExpeditionDefinitionError::InvalidQuestWorldState);
        }
        self.provider_dialogue_key = Some(dialogue_key);
        Ok(self)
    }
}

/// One bounded material change applied atomically when a quest is completed.
/// Effects are local to the quest's hub; the summary key makes the consequence
/// visible before the player accepts the quest.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum QuestWorldEffectDefinition {
    UnlockDoor {
        position: GridPos,
        summary_key: String,
    },
    UpdateDataTerminal {
        installation: ContentId,
        record: ContentId,
        summary_key: String,
    },
    GrantPropertyTakeAuthorization {
        owner: SocialGroupId,
        summary_key: String,
    },
}

impl QuestWorldEffectDefinition {
    pub fn unlock_door(
        position: GridPos,
        summary_key: String,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if position.x < 0 || position.y < 0 || summary_key.trim().is_empty() {
            return Err(ExpeditionDefinitionError::InvalidQuestWorldEffect);
        }
        Ok(Self::UnlockDoor {
            position,
            summary_key,
        })
    }

    pub fn update_data_terminal(
        installation: ContentId,
        record: ContentId,
        summary_key: String,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if summary_key.trim().is_empty() {
            return Err(ExpeditionDefinitionError::InvalidQuestWorldEffect);
        }
        Ok(Self::UpdateDataTerminal {
            installation,
            record,
            summary_key,
        })
    }

    pub fn grant_property_take_authorization(
        owner: SocialGroupId,
        summary_key: String,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if summary_key.trim().is_empty() {
            return Err(ExpeditionDefinitionError::InvalidQuestWorldEffect);
        }
        Ok(Self::GrantPropertyTakeAuthorization { owner, summary_key })
    }

    pub const fn door_position(&self) -> Option<GridPos> {
        match self {
            Self::UnlockDoor { position, .. } => Some(*position),
            Self::UpdateDataTerminal { .. } | Self::GrantPropertyTakeAuthorization { .. } => None,
        }
    }

    pub fn summary_key(&self) -> &str {
        match self {
            Self::UnlockDoor { summary_key, .. }
            | Self::UpdateDataTerminal { summary_key, .. }
            | Self::GrantPropertyTakeAuthorization { summary_key, .. } => summary_key,
        }
    }

    pub const fn is_installation_effect(&self) -> bool {
        matches!(self, Self::UpdateDataTerminal { .. })
    }

    pub const fn is_authorization_effect(&self) -> bool {
        matches!(self, Self::GrantPropertyTakeAuthorization { .. })
    }

    pub(crate) fn has_valid_shape(&self) -> bool {
        match self {
            Self::UnlockDoor {
                position,
                summary_key,
            } => position.x >= 0 && position.y >= 0 && !summary_key.trim().is_empty(),
            Self::UpdateDataTerminal { summary_key, .. } => !summary_key.trim().is_empty(),
            Self::GrantPropertyTakeAuthorization { summary_key, .. } => {
                !summary_key.trim().is_empty()
            }
        }
    }

    pub(crate) fn targets_same_element(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::UnlockDoor { position: left, .. },
                Self::UnlockDoor {
                    position: right, ..
                },
            ) => left == right,
            (
                Self::UpdateDataTerminal {
                    installation: left, ..
                },
                Self::UpdateDataTerminal {
                    installation: right,
                    ..
                },
            ) => left == right,
            (
                Self::GrantPropertyTakeAuthorization { owner: left, .. },
                Self::GrantPropertyTakeAuthorization { owner: right, .. },
            ) => left == right,
            _ => false,
        }
    }
}

impl QuestItemRewardDefinition {
    pub fn new(item: ItemId, quantity: u16) -> Result<Self, ExpeditionDefinitionError> {
        if quantity == 0 {
            return Err(ExpeditionDefinitionError::InvalidQuestReward);
        }
        Ok(Self { item, quantity })
    }
}

impl HubQuestDefinition {
    pub fn new(provider: HubQuestProviderDefinition, quest: QuestDefinition) -> Self {
        Self {
            provider,
            quest,
            prerequisites: Vec::new(),
            required_world_states: Vec::new(),
            completion_world_states: Vec::new(),
            completion_world_effects: Vec::new(),
            choice_group: None,
            choice_prompt_key: None,
            reward_experience: 0,
            reward_items: Vec::new(),
        }
    }

    pub fn with_prerequisites(
        mut self,
        prerequisites: Vec<ContentId>,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if prerequisites.len() > MAX_QUEST_PREREQUISITES
            || prerequisites
                .iter()
                .enumerate()
                .any(|(index, id)| prerequisites[..index].contains(id))
        {
            return Err(ExpeditionDefinitionError::InvalidQuestPrerequisites);
        }
        self.prerequisites = prerequisites;
        Ok(self)
    }

    pub fn with_choice(
        mut self,
        group: ContentId,
        prompt_key: String,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if prompt_key.trim().is_empty() {
            return Err(ExpeditionDefinitionError::InvalidQuestChoice);
        }
        self.choice_group = Some(group);
        self.choice_prompt_key = Some(prompt_key);
        Ok(self)
    }

    pub fn with_world_states(
        mut self,
        required: Vec<ContentId>,
        completed: Vec<QuestWorldStateDefinition>,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if required.len() > MAX_QUEST_WORLD_STATES
            || completed.len() > MAX_QUEST_WORLD_STATES
            || required
                .iter()
                .enumerate()
                .any(|(index, id)| required[..index].contains(id))
            || completed.iter().enumerate().any(|(index, state)| {
                state.summary_key.trim().is_empty()
                    || state
                        .provider_dialogue_key
                        .as_ref()
                        .is_some_and(|key| key.trim().is_empty())
                    || completed[..index].iter().any(|known| known.id == state.id)
            })
        {
            return Err(ExpeditionDefinitionError::InvalidQuestWorldState);
        }
        self.required_world_states = required;
        self.completion_world_states = completed;
        Ok(self)
    }

    pub fn with_world_effects(
        mut self,
        effects: Vec<QuestWorldEffectDefinition>,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if effects.len() > MAX_QUEST_WORLD_EFFECTS
            || effects.iter().enumerate().any(|(index, effect)| {
                !effect.has_valid_shape()
                    || effects[..index]
                        .iter()
                        .any(|known| known.targets_same_element(effect))
            })
        {
            return Err(ExpeditionDefinitionError::InvalidQuestWorldEffect);
        }
        self.completion_world_effects = effects;
        Ok(self)
    }

    pub fn with_additional_rewards(
        mut self,
        experience: u64,
        items: Vec<QuestItemRewardDefinition>,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if items.len() > MAX_QUEST_REWARD_ITEMS
            || items.iter().enumerate().any(|(index, reward)| {
                reward.quantity == 0 || items[..index].iter().any(|known| known.item == reward.item)
            })
        {
            return Err(ExpeditionDefinitionError::InvalidQuestReward);
        }
        self.reward_experience = experience;
        self.reward_items = items;
        Ok(self)
    }
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
    pub hub_merchant: Option<MerchantDefinition>,
    pub hub_clinic: Option<ClinicDefinition>,
    pub hub_residents: Vec<ResidentDefinition>,
    pub hub_quests: Vec<HubQuestDefinition>,
    pub player_starting_credits: u32,
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
        if let Some(merchant) = &self.hub_merchant {
            definition.field("hub_merchant", merchant);
        }
        if let Some(clinic) = &self.hub_clinic {
            definition.field("hub_clinic", clinic);
        }
        if !self.hub_residents.is_empty() {
            definition.field("hub_residents", &self.hub_residents);
        }
        if !self.hub_quests.is_empty() {
            definition.field("hub_quests", &self.hub_quests);
        }
        if self.player_starting_credits > 0 {
            definition.field("player_starting_credits", &self.player_starting_credits);
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
            hub_merchant: None,
            hub_clinic: None,
            hub_residents: Vec::new(),
            hub_quests: Vec::new(),
            player_starting_credits: 0,
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

    pub fn with_hub_merchant(
        mut self,
        merchant: MerchantDefinition,
        player_starting_credits: u32,
    ) -> Self {
        self.hub_merchant = Some(merchant);
        self.player_starting_credits = player_starting_credits;
        self
    }

    pub fn with_hub_clinic(mut self, clinic: ClinicDefinition) -> Self {
        self.hub_clinic = Some(clinic);
        self
    }

    pub fn with_hub_residents(
        mut self,
        residents: Vec<ResidentDefinition>,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if residents.len() > MAX_HUB_RESIDENTS
            || residents.iter().enumerate().any(|(index, resident)| {
                residents[..index].iter().any(|known| {
                    known.residence_position == resident.residence_position
                        || known.residence_position == resident.gathering_position
                        || known.gathering_position == resident.residence_position
                        || known.gathering_position == resident.gathering_position
                })
            })
        {
            return Err(ExpeditionDefinitionError::InvalidResident);
        }
        self.hub_residents = residents;
        Ok(self)
    }

    pub fn with_hub_quests(
        mut self,
        quests: Vec<HubQuestDefinition>,
    ) -> Result<Self, ExpeditionDefinitionError> {
        if quests.len() > MAX_HUB_QUESTS {
            return Err(ExpeditionDefinitionError::TooManyHubQuests);
        }
        for (index, quest) in quests.iter().enumerate() {
            if quest.prerequisites.len() > MAX_QUEST_PREREQUISITES
                || quest
                    .prerequisites
                    .iter()
                    .enumerate()
                    .any(|(required_index, required)| {
                        quest.prerequisites[..required_index].contains(required)
                    })
            {
                return Err(ExpeditionDefinitionError::InvalidQuestPrerequisites);
            }
            if quest.reward_items.len() > MAX_QUEST_REWARD_ITEMS
                || quest
                    .reward_items
                    .iter()
                    .enumerate()
                    .any(|(reward_index, reward)| {
                        reward.quantity == 0
                            || quest.reward_items[..reward_index]
                                .iter()
                                .any(|known| known.item == reward.item)
                    })
            {
                return Err(ExpeditionDefinitionError::InvalidQuestReward);
            }
            if quest.required_world_states.len() > MAX_QUEST_WORLD_STATES
                || quest.completion_world_states.len() > MAX_QUEST_WORLD_STATES
                || quest
                    .required_world_states
                    .iter()
                    .enumerate()
                    .any(|(state_index, state)| {
                        quest.required_world_states[..state_index].contains(state)
                            || !quests[..index].iter().any(|known| {
                                known
                                    .completion_world_states
                                    .iter()
                                    .any(|completed| &completed.id == state)
                            })
                    })
                || quest
                    .completion_world_states
                    .iter()
                    .enumerate()
                    .any(|(state_index, state)| {
                        state.summary_key.trim().is_empty()
                            || state
                                .provider_dialogue_key
                                .as_ref()
                                .is_some_and(|key| key.trim().is_empty())
                            || quest.completion_world_states[..state_index]
                                .iter()
                                .any(|known| known.id == state.id)
                            || quests[..index].iter().any(|known| {
                                known
                                    .completion_world_states
                                    .iter()
                                    .any(|completed| completed.id == state.id)
                            })
                    })
            {
                return Err(ExpeditionDefinitionError::InvalidQuestWorldState);
            }
            if quest.completion_world_effects.len() > MAX_QUEST_WORLD_EFFECTS
                || quest.completion_world_effects.iter().enumerate().any(
                    |(effect_index, effect)| {
                        !effect.has_valid_shape()
                            || quest.completion_world_effects[..effect_index]
                                .iter()
                                .any(|known| known.targets_same_element(effect))
                            || quests[..index].iter().any(|known| {
                                known
                                    .completion_world_effects
                                    .iter()
                                    .any(|known| known.targets_same_element(effect))
                            })
                    },
                )
            {
                return Err(ExpeditionDefinitionError::InvalidQuestWorldEffect);
            }
            if quests[..index]
                .iter()
                .any(|known| known.quest.id() == quest.quest.id())
            {
                return Err(ExpeditionDefinitionError::DuplicateQuest(
                    quest.quest.id().clone(),
                ));
            }
            if quest.prerequisites.iter().any(|required| {
                !quests[..index]
                    .iter()
                    .any(|known| known.quest.id() == required)
            }) {
                return Err(ExpeditionDefinitionError::InvalidQuestPrerequisites);
            }
            if quest.choice_group.is_some() != quest.choice_prompt_key.is_some() {
                return Err(ExpeditionDefinitionError::InvalidQuestChoice);
            }
            if let Some(group) = &quest.choice_group {
                let Some(first) = quests
                    .iter()
                    .find(|known| known.choice_group.as_ref() == Some(group))
                else {
                    unreachable!("the current quest belongs to its own choice group")
                };
                if first.provider.position() != quest.provider.position()
                    || first.choice_prompt_key != quest.choice_prompt_key
                    || quest.prerequisites.iter().any(|required| {
                        quests[..index].iter().any(|known| {
                            known.quest.id() == required
                                && known.choice_group.as_ref() == Some(group)
                        })
                    })
                {
                    return Err(ExpeditionDefinitionError::InvalidQuestChoice);
                }
            }
            if let Some(known) = quests[..index]
                .iter()
                .find(|known| known.provider.position() == quest.provider.position())
                && !quest_providers_can_share(&known.provider, &quest.provider)
            {
                return Err(ExpeditionDefinitionError::DuplicateQuestProviderPosition(
                    quest.provider.position(),
                ));
            }
        }
        for quest in &quests {
            if let Some(group) = &quest.choice_group
                && quests
                    .iter()
                    .filter(|known| known.choice_group.as_ref() == Some(group))
                    .count()
                    < 2
            {
                return Err(ExpeditionDefinitionError::InvalidQuestChoice);
            }
        }
        self.hub_quests = quests;
        Ok(self)
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
        if let Some(merchant) = &self.hub_merchant {
            merchant.validate_references(items)?;
        }
        for quest in &self.hub_quests {
            if let QuestDefinition::Delivery(delivery) = &quest.quest
                && items.get(&delivery.required_item).is_none()
            {
                return Err(ExpeditionDefinitionError::UnknownQuestItem(
                    delivery.required_item.clone(),
                ));
            }
            for reward in &quest.reward_items {
                if items.get(&reward.item).is_none() {
                    return Err(ExpeditionDefinitionError::UnknownQuestItem(
                        reward.item.clone(),
                    ));
                }
            }
            for effect in &quest.completion_world_effects {
                if let QuestWorldEffectDefinition::UpdateDataTerminal {
                    installation,
                    record,
                    ..
                } = effect
                {
                    let valid_target = self.hub_facility.as_ref().is_some_and(|facility| {
                        facility.blueprint.installations.iter().any(|candidate| {
                            &candidate.id == installation
                                && candidate.capabilities.iter().any(|capability| {
                                    matches!(
                                        capability,
                                        InstallationCapability::DataTerminal {
                                            record: current_record
                                        } if current_record != record
                                    )
                                })
                        })
                    });
                    if !valid_target {
                        return Err(ExpeditionDefinitionError::InvalidQuestWorldEffect);
                    }
                }
            }
            let position = quest.provider.position();
            match &quest.provider {
                HubQuestProviderDefinition::Existing { .. }
                    if !self.has_authored_hub_actor_at(position) =>
                {
                    return Err(ExpeditionDefinitionError::UnknownQuestProviderPosition(
                        position,
                    ));
                }
                HubQuestProviderDefinition::Contact { .. }
                    if self.has_authored_hub_actor_at(position) =>
                {
                    return Err(ExpeditionDefinitionError::OccupiedQuestContactPosition(
                        position,
                    ));
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn has_authored_hub_actor_at(&self, position: GridPos) -> bool {
        self.hub_merchant
            .as_ref()
            .is_some_and(|merchant| merchant.position == position)
            || self
                .hub_clinic
                .as_ref()
                .is_some_and(|clinic| clinic.work_position == position)
            || self
                .hub_residents
                .iter()
                .any(|resident| resident.residence_position == position)
            || self.hub_facility.as_ref().is_some_and(|facility| {
                facility
                    .blueprint
                    .workers
                    .iter()
                    .any(|worker| worker.actor_position == position)
            })
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
        if let Some(duplicate) = definition.hub_quests.iter().find_map(|quest| {
            self.definitions
                .values()
                .flat_map(|known| known.hub_quests.iter())
                .find(|known| known.quest.id() == quest.quest.id())
                .map(|_| quest.quest.id().clone())
        }) {
            return Err(ExpeditionDefinitionError::DuplicateQuest(duplicate));
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
                definition.hub_merchant = None;
                definition.hub_clinic = None;
                definition.hub_residents.clear();
                definition.hub_quests.clear();
                definition.player_starting_credits = 0;
                definition.player_property_take_authorizations.clear();
                definition.destination.population.clear();
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v62 commerce data while preserving older world fingerprints.
    pub fn without_commerce_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                definition.hub_merchant = None;
                // A catalogue reconstructed for any pre-commerce generation
                // must also discard services introduced after commerce.
                definition.hub_clinic = None;
                definition.hub_residents.clear();
                definition.hub_quests.clear();
                definition.player_starting_credits = 0;
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v63 clinic data while preserving v62 catalogue fingerprints.
    pub fn without_clinic_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                definition.hub_clinic = None;
                definition.hub_residents.clear();
                definition.hub_quests.clear();
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v64 resident data while preserving v63 catalogue fingerprints.
    pub fn without_resident_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                definition.hub_residents.clear();
                definition.hub_quests.clear();
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v65 authored quest chains while preserving v64 catalogue
    /// fingerprints and resident routines.
    pub fn without_quest_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                definition.hub_quests.clear();
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Restores the v84 border-crossing exploration rule for older runs.
    pub fn without_quest_site_record_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                for quest in &mut definition.hub_quests {
                    if let QuestDefinition::ExploreZones(exploration) = &mut quest.quest {
                        exploration.qualifying_records.clear();
                    }
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v66 durable quest consequences while preserving the v65 quest
    /// chain definitions and their historical fingerprints.
    pub fn without_quest_world_state_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                for quest in &mut definition.hub_quests {
                    quest.required_world_states.clear();
                    quest.completion_world_states.clear();
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v67 material quest consequences while preserving v66 world
    /// states and their historical catalogue fingerprints.
    pub fn without_quest_world_effect_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                for quest in &mut definition.hub_quests {
                    quest.completion_world_effects.clear();
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v68 installation-targeted quest effects while preserving v67
    /// door effects and their historical catalogue fingerprints.
    pub fn without_quest_installation_effect_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                for quest in &mut definition.hub_quests {
                    quest
                        .completion_world_effects
                        .retain(|effect| !effect.is_installation_effect());
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v69 dynamic property authorizations while preserving all v68
    /// installation and door effects for historical catalogue fingerprints.
    pub fn without_quest_authorization_effect_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                for quest in &mut definition.hub_quests {
                    quest
                        .completion_world_effects
                        .retain(|effect| !effect.is_authorization_effect());
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v70 direct witness-report links while preserving all v69 quest
    /// effects and historical catalogue fingerprints.
    pub fn without_property_report_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                if let Some(facility) = &mut definition.hub_facility {
                    for worker in &mut facility.blueprint.workers {
                        worker.property_report = None;
                        worker.installed_property_report = None;
                        worker.reported_incident_response = None;
                    }
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v71 recipient movement responses while preserving v70 direct
    /// report links and their historical catalogue fingerprints.
    pub fn without_reported_incident_response_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                if let Some(facility) = &mut definition.hub_facility {
                    for worker in &mut facility.blueprint.workers {
                        worker.installed_property_report = None;
                        worker.reported_incident_response = None;
                    }
                }
                (id.clone(), definition)
            })
            .collect();
        Self { definitions }
    }

    /// Removes v72 witness links to installed security systems while
    /// preserving v71 worker investigations and direct actor reports.
    pub fn without_installed_property_report_metadata(&self) -> Self {
        let definitions = self
            .definitions
            .iter()
            .map(|(id, definition)| {
                let mut definition = definition.clone();
                if let Some(facility) = &mut definition.hub_facility {
                    for worker in &mut facility.blueprint.workers {
                        worker.installed_property_report = None;
                    }
                }
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
                        worker.property_report = None;
                        worker.installed_property_report = None;
                        worker.reported_incident_response = None;
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
    InvalidMerchant,
    InvalidClinic,
    InvalidResident,
    InvalidDeliveryQuest,
    InvalidExplorationQuest,
    InvalidDataRecordQuest,
    InvalidDefeatTargetsQuest,
    InvalidQuestPrerequisites,
    InvalidQuestChoice,
    InvalidQuestReward,
    InvalidQuestWorldState,
    InvalidQuestWorldEffect,
    InvalidQuestProvider,
    TooManyHubQuests,
    DuplicateQuest(ContentId),
    DuplicateQuestProviderPosition(GridPos),
    UnknownQuestProviderPosition(GridPos),
    OccupiedQuestContactPosition(GridPos),
    UnknownQuestItem(ItemId),
    UnknownMerchantItem(ItemId),
    MerchantGambleItemIsNotArmor(ItemId),
    InvalidWitnessProfile(WitnessProfileError),
    InvalidLocalAlertProfile(LocalAlertProfileError),
    InvalidPropertyReportProfile(PropertyReportProfileError),
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
    InvalidPopulationTags,
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
            Self::InvalidMerchant => write!(
                formatter,
                "merchant requires unique stocked offers, positive prices, and sell prices no greater than buy prices"
            ),
            Self::InvalidClinic => write!(
                formatter,
                "clinic requires positive care, prices, routine durations, and a bounded path budget"
            ),
            Self::InvalidResident => write!(
                formatter,
                "resident requires distinct anchors, positive routine durations, and a bounded path budget"
            ),
            Self::InvalidDeliveryQuest => write!(
                formatter,
                "delivery quest requires non-empty text keys and a positive item quantity"
            ),
            Self::InvalidExplorationQuest => write!(
                formatter,
                "exploration quest requires non-empty text keys and a positive zone count"
            ),
            Self::InvalidDataRecordQuest => {
                write!(formatter, "data record quest requires non-empty text keys")
            }
            Self::InvalidDefeatTargetsQuest => write!(
                formatter,
                "defeat targets quest requires non-empty text keys and a positive target quantity"
            ),
            Self::InvalidQuestPrerequisites => write!(
                formatter,
                "quest prerequisites must be unique, bounded, and refer only to earlier quests in the same hub"
            ),
            Self::InvalidQuestChoice => write!(
                formatter,
                "quest choices require at least two offers from the same provider with one shared non-empty prompt"
            ),
            Self::InvalidQuestReward => write!(
                formatter,
                "quest item rewards must have unique item IDs, positive quantities, and remain within the reward budget"
            ),
            Self::InvalidQuestWorldState => write!(
                formatter,
                "quest world states must be unique, bounded, visibly described, and required only after an earlier quest can grant them"
            ),
            Self::InvalidQuestWorldEffect => write!(
                formatter,
                "quest world effects must be unique, bounded, visibly described, and target non-negative hub coordinates"
            ),
            Self::InvalidQuestProvider => write!(
                formatter,
                "quest provider requires non-negative coordinates and a positive integrity when it creates a contact"
            ),
            Self::TooManyHubQuests => write!(
                formatter,
                "hub quests cannot exceed {MAX_HUB_QUESTS} entries"
            ),
            Self::DuplicateQuest(id) => write!(formatter, "duplicate quest ID '{id}'"),
            Self::DuplicateQuestProviderPosition(position) => write!(
                formatter,
                "more than one quest targets the hub NPC at [{}, {}]",
                position.x, position.y
            ),
            Self::UnknownQuestProviderPosition(position) => write!(
                formatter,
                "quest targets no authored hub NPC at [{}, {}]",
                position.x, position.y
            ),
            Self::OccupiedQuestContactPosition(position) => write!(
                formatter,
                "dedicated quest contact overlaps an authored hub NPC at [{}, {}]",
                position.x, position.y
            ),
            Self::UnknownQuestItem(id) => write!(formatter, "unknown quest item '{id}'"),
            Self::UnknownMerchantItem(id) => write!(formatter, "unknown merchant item '{id}'"),
            Self::MerchantGambleItemIsNotArmor(id) => {
                write!(formatter, "merchant gamble item '{id}' is not armor")
            }
            Self::InvalidWitnessProfile(error) => write!(formatter, "{error}"),
            Self::InvalidLocalAlertProfile(error) => write!(formatter, "{error}"),
            Self::InvalidPropertyReportProfile(error) => write!(formatter, "{error}"),
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
            Self::InvalidPopulationTags => write!(
                formatter,
                "population actor tags must be unique and cannot exceed {MAX_POPULATION_TAGS} entries"
            ),
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
    fn exploration_site_records_are_optional_bounded_and_unique() {
        let definition = ExplorationQuestDefinition::new(
            "test:survey".parse().unwrap(),
            "quest.survey.title".into(),
            "quest.survey.summary".into(),
            1,
            10,
        )
        .unwrap();
        assert!(definition.qualifying_records.is_empty());
        let record: ContentId = "test:site_record".parse().unwrap();
        assert_eq!(
            definition
                .clone()
                .with_qualifying_records(vec![record.clone(), record.clone()])
                .unwrap_err(),
            ExpeditionDefinitionError::InvalidExplorationQuest
        );
        assert_eq!(
            definition
                .clone()
                .with_qualifying_records(vec![])
                .unwrap_err(),
            ExpeditionDefinitionError::InvalidExplorationQuest
        );
        let authored = definition.with_qualifying_records(vec![record]).unwrap();
        assert!(format!("{authored:?}").contains("qualifying_records"));
    }

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
