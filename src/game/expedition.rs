//! Persistent, deterministic zones. Presentation never advances this simulation.
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

use base64::Engine;
use bincode::Options;
use serde::{Deserialize, Serialize};

use crate::ai::{AiAction, AiBehavior, AiSituation, AiState, decide_known_action};
use crate::content::{
    ClinicDefinition, ContentId, GambleScalingDefinition, HubQuestDefinition,
    MAX_QUEST_WORLD_EFFECTS, MAX_QUEST_WORLD_STATES, MerchantDefinition, QuestItemRewardDefinition,
    QuestWorldEffectDefinition, QuestWorldStateDefinition, ResidentDefinition,
};
pub use crate::content::{
    DataRecordQuestDefinition, DefeatTargetsQuestDefinition, DeliveryQuestDefinition,
    ExplorationQuestDefinition, QuestDefinition,
};
use crate::drone::DroneOrder;
use crate::effects::GroundEffectMap;
use crate::electronic_warfare::ElectronicWarfareState;
use crate::entity::{Actor, ActorRegistry, GroundItemRegistry, ItemInstanceId, MagicItemModifiers};
use crate::explosive::ExplosiveDeviceMap;
use crate::facility::{
    FacilityBlueprint, FacilityEvent, FacilityState, ReinforcementRequestFailure, RepairStatus,
    SecurityAlarmResponse, SecurityAlarmResponseContext, WorkOrderId, WorkerRole,
};
use crate::intrusion::IntrusionState;
use crate::item::ItemId;
use crate::progression::RewardKey;
use crate::social::{ObservedPropertyTake, SocialGroupId};
use crate::status::StatusTrigger;
use crate::world::generation::{MapValidationRules, validate_interactive_map};
use crate::world::{
    Direction, DoorState, GridPos, Map, MovementTraceMap, Terrain, VisibilityState, find_path,
};

use super::{
    CommandOutcome, CommandRejection, GameCommand, GameEvent, GameRng, GameRules, GameState,
    QuestCompletion, RunStatus, ThreatReinforcementRequestError, ThreatSourceBlueprint,
    ThreatSourceState, TurnPhase, game_state::GameStateSnapshot,
};

const MAX_WORLD_SNAPSHOT_BYTES: usize = 12 * 1024 * 1024;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZoneInfo {
    pub id: ContentId,
    pub name: String,
    pub kind: ContentId,
    pub depth: u16,
}

/// Read-only interaction offered by one simulated NPC. Opening this view does
/// not advance time; only selecting a service or quest action sends a recorded
/// game command.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NpcInteraction {
    pub provider: crate::entity::EntityId,
    pub position: GridPos,
    pub role: NpcRole,
    pub locally_alerted: bool,
    pub resident_routine: Option<ResidentRoutineState>,
    pub contextual_dialogue_key: Option<String>,
    pub services: Vec<NpcService>,
    pub quests: Vec<NpcQuestView>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NpcRole {
    Worker(WorkerRole),
    Merchant,
    Healer,
    Resident,
    QuestContact,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NpcService {
    FacilityMaintenance {
        order: WorkOrderId,
        required_item: ItemId,
        missing_quantity: u16,
        state: NpcServiceState,
    },
    Trade {
        player_credits: u32,
        merchant_credits: u32,
        offers: Vec<TradeOfferView>,
        resale: Vec<TradeResaleView>,
        gambles: Vec<TradeGambleView>,
        sellable: Vec<TradeSellView>,
    },
    Treatment {
        player_credits: u32,
        clinic_credits: u32,
        current_integrity: u16,
        maximum_integrity: u16,
        maximum_restoration: u16,
        restore_amount: u16,
        price: u32,
        routine: ClinicRoutineState,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClinicRoutineState {
    AtWork,
    MovingToBreak,
    OnBreak,
    ReturningToWork,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResidentRoutineState {
    AtResidence,
    MovingToGathering,
    AtGathering,
    ReturningToResidence,
}

pub type QuestId = ContentId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuestStatus {
    Available,
    Active,
    ReadyToComplete,
    Completed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NpcQuestView {
    pub id: QuestId,
    pub title_key: String,
    pub summary_key: String,
    pub status: QuestStatus,
    pub objective: QuestObjectiveView,
    pub reward_credits: u32,
    pub reward_experience: u64,
    pub reward_items: Vec<QuestItemRewardDefinition>,
    pub completion_world_states: Vec<QuestWorldStateDefinition>,
    pub completion_world_effects: Vec<QuestWorldEffectDefinition>,
    pub choice_prompt_key: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QuestMarker {
    Available,
    ReadyToComplete,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuestObjectiveView {
    Delivery {
        required_item: ItemId,
        required_quantity: u16,
        carried_quantity: u16,
    },
    ExploreZones {
        required_zones: u16,
        explored_zones: u16,
        site_record_required: bool,
    },
    AccessDataRecord {
        record: ContentId,
        accessed: bool,
    },
    DefeatTargets {
        target_tag: ContentId,
        required_quantity: u16,
        defeated_quantity: u16,
    },
}

/// Accepted and completed contracts visible to any presentation adapter.
/// Available offers are deliberately absent until the player accepts them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuestJournalEntry {
    pub zone: ZoneInfo,
    pub giver: crate::entity::EntityId,
    pub quest: NpcQuestView,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TradeOfferView {
    pub item: ItemId,
    pub stock: u16,
    pub price: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TradeResaleView {
    pub listing: u64,
    pub item: ItemId,
    pub price: u32,
    pub magic_modifiers: Option<MagicItemModifiers>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TradeGambleView {
    pub item: ItemId,
    pub stock: u16,
    pub price: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TradeSellView {
    pub instance: ItemInstanceId,
    pub item: ItemId,
    pub quantity: u16,
    pub price: u32,
    pub magic_modifiers: Option<MagicItemModifiers>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct MerchantOfferState {
    item: ItemId,
    stock: u16,
    buy_price: u32,
    sell_price: u32,
    maximum_stack: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct MerchantResaleState {
    listing: u64,
    item: ItemId,
    price: u32,
    maximum_stack: u16,
    magic_modifiers: Option<MagicItemModifiers>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct MerchantGambleState {
    item: ItemId,
    stock: u16,
    price: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct MerchantState {
    provider: crate::entity::EntityId,
    credits: u32,
    offers: Vec<MerchantOfferState>,
    resale: Vec<MerchantResaleState>,
    gambles: Vec<MerchantGambleState>,
    next_listing: u64,
    gamble_rng: GameRng,
    gamble_scaling: GambleScalingDefinition,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct ClinicState {
    provider: crate::entity::EntityId,
    credits: u32,
    maximum_restoration: u16,
    price_per_point: u32,
    work_position: GridPos,
    break_position: GridPos,
    work_turns: u16,
    break_turns: u16,
    maximum_path_search: usize,
    routine_working: bool,
    routine_remaining_turns: u16,
}

impl ClinicState {
    fn routine_state(&self, position: GridPos) -> ClinicRoutineState {
        if self.routine_working && position == self.work_position {
            ClinicRoutineState::AtWork
        } else if self.routine_working {
            ClinicRoutineState::ReturningToWork
        } else if position == self.break_position {
            ClinicRoutineState::OnBreak
        } else {
            ClinicRoutineState::MovingToBreak
        }
    }

    fn routine_target(&self) -> GridPos {
        if self.routine_working {
            self.work_position
        } else {
            self.break_position
        }
    }

    fn elapse_at_target(&mut self) {
        if self.routine_remaining_turns > 1 {
            self.routine_remaining_turns -= 1;
        } else {
            self.routine_working = !self.routine_working;
            self.routine_remaining_turns = if self.routine_working {
                self.work_turns
            } else {
                self.break_turns
            };
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct ResidentState {
    provider: crate::entity::EntityId,
    residence_position: GridPos,
    gathering_position: GridPos,
    residence_turns: u16,
    gathering_turns: u16,
    maximum_path_search: usize,
    routine_at_residence: bool,
    routine_remaining_turns: u16,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
struct QuestState {
    provider: crate::entity::EntityId,
    definition: QuestDefinition,
    progress: QuestProgress,
    prerequisites: Vec<QuestId>,
    required_world_states: Vec<ContentId>,
    completion_world_states: Vec<QuestWorldStateDefinition>,
    completion_world_effects: Vec<QuestWorldEffectDefinition>,
    choice_group: Option<ContentId>,
    choice_prompt_key: Option<String>,
    reward_experience: u64,
    reward_items: Vec<QuestItemRewardDefinition>,
    accepted: bool,
    completed: bool,
    excluded: bool,
}

// Empty v66 state metadata is omitted so replayed v65 quests keep the exact
// historical state fingerprint.
impl Debug for QuestState {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut state = formatter.debug_struct("QuestState");
        state
            .field("provider", &self.provider)
            .field("definition", &self.definition)
            .field("progress", &self.progress)
            .field("prerequisites", &self.prerequisites)
            .field("choice_group", &self.choice_group)
            .field("choice_prompt_key", &self.choice_prompt_key)
            .field("reward_experience", &self.reward_experience)
            .field("reward_items", &self.reward_items)
            .field("accepted", &self.accepted)
            .field("completed", &self.completed)
            .field("excluded", &self.excluded);
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum QuestProgress {
    Delivery,
    ExploreZones {
        baseline: BTreeSet<ContentId>,
        discovered: BTreeSet<ContentId>,
    },
    AccessDataRecord {
        accessed: bool,
    },
    DefeatTargets {
        defeated_quantity: u16,
    },
}

impl ResidentState {
    fn routine_state(&self, position: GridPos) -> ResidentRoutineState {
        if self.routine_at_residence && position == self.residence_position {
            ResidentRoutineState::AtResidence
        } else if self.routine_at_residence {
            ResidentRoutineState::ReturningToResidence
        } else if position == self.gathering_position {
            ResidentRoutineState::AtGathering
        } else {
            ResidentRoutineState::MovingToGathering
        }
    }

    fn routine_target(&self) -> GridPos {
        if self.routine_at_residence {
            self.residence_position
        } else {
            self.gathering_position
        }
    }

    fn elapse_at_target(&mut self) {
        if self.routine_remaining_turns > 1 {
            self.routine_remaining_turns -= 1;
        } else {
            self.routine_at_residence = !self.routine_at_residence;
            self.routine_remaining_turns = if self.routine_at_residence {
                self.residence_turns
            } else {
                self.gathering_turns
            };
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct GambleRollProfile {
    rank: u16,
    quality_draws: u16,
    quality_ceiling: u16,
}

impl GambleRollProfile {
    fn for_progression(
        scaling: GambleScalingDefinition,
        player_level: u16,
        zone_depth: u16,
    ) -> Self {
        let level_ranks = player_level
            .saturating_sub(1)
            .checked_div(scaling.player_levels_per_rank)
            .unwrap_or(0);
        let zone_ranks = zone_depth
            .checked_div(scaling.zone_depths_per_rank)
            .unwrap_or(0);
        let rank = 1_u16
            .saturating_add(level_ranks)
            .saturating_add(zone_ranks)
            .min(scaling.maximum_rank);
        Self {
            rank,
            // Taking the best of more bounded samples progressively shifts
            // probability upward without ever guaranteeing a strong result.
            quality_draws: 1_u16.saturating_add(rank.saturating_sub(1) / 4).min(4),
            // The ceiling prevents end-game modifiers from appearing in an
            // early zone even when its single random draw is excellent.
            quality_ceiling: 24_u16.saturating_add(rank.saturating_mul(7)).min(100),
        }
    }

    fn roll(self, rng: &mut GameRng) -> MagicItemModifiers {
        let mut quality = 0_u16;
        for _ in 0..self.quality_draws {
            let sample = rng
                .usize_inclusive(0, usize::from(self.quality_ceiling))
                .expect("authored gamble quality bounds are valid") as u16;
            quality = quality.max(sample);
        }
        let armor_bonus = 1_u16
            .saturating_add(self.rank.saturating_sub(1) / 5)
            .saturating_add(quality / 25);
        let mass_reduction_percent = 5_u16
            .saturating_add(self.rank.saturating_sub(1))
            .saturating_add(quality / 4)
            .min(80) as u8;
        MagicItemModifiers::new(armor_bonus, mass_reduction_percent)
            .expect("generated magical modifiers are bounded and positive")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NpcServiceState {
    MaterialRequired {
        player_can_supply: bool,
        /// Last source known to the local maintenance circuit. This is only
        /// disclosed through an adjacent NPC interaction; it is not player
        /// perception and clients should present it as an approximate report.
        known_source: Option<GridPos>,
    },
    InTransit,
    Queued,
    InProgress {
        remaining_turns: u16,
    },
    Operational,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroundLootBlueprint {
    position: GridPos,
    item: ItemId,
    quantity: u16,
    owner: Option<SocialGroupId>,
}

impl GroundLootBlueprint {
    pub fn new(position: GridPos, item: ItemId, quantity: u16) -> Self {
        Self {
            position,
            item,
            quantity,
            owner: None,
        }
    }

    pub fn with_owner(mut self, owner: SocialGroupId) -> Self {
        self.owner = Some(owner);
        self
    }

    pub const fn position(&self) -> GridPos {
        self.position
    }

    pub const fn item(&self) -> &ItemId {
        &self.item
    }

    pub const fn quantity(&self) -> u16 {
        self.quantity
    }

    pub const fn owner(&self) -> Option<&SocialGroupId> {
        self.owner.as_ref()
    }
}

impl From<(GridPos, ItemId, u16)> for GroundLootBlueprint {
    fn from((position, item, quantity): (GridPos, ItemId, u16)) -> Self {
        Self::new(position, item, quantity)
    }
}

impl Debug for GroundLootBlueprint {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        if self.owner.is_none() {
            return (&self.position, &self.item, self.quantity).fmt(formatter);
        }
        formatter
            .debug_struct("GroundLootBlueprint")
            .field("position", &self.position)
            .field("item", &self.item)
            .field("quantity", &self.quantity)
            .field("owner", &self.owner)
            .finish()
    }
}

/// Input from any authored/procedural content provider. Spawns are instantiated
/// once, on first entry, with global IDs and their own deterministic RNG stream.
#[derive(Clone, Serialize, Deserialize)]
pub struct ZoneBlueprint {
    pub info: ZoneInfo,
    pub map: Map,
    pub entrance: GridPos,
    pub seed: u64,
    pub actors: Vec<Actor>,
    pub loot: Vec<GroundLootBlueprint>,
    pub threat_sources: Vec<ThreatSourceBlueprint>,
}

impl Debug for ZoneBlueprint {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut blueprint = formatter.debug_struct("ZoneBlueprint");
        blueprint
            .field("info", &self.info)
            .field("map", &self.map)
            .field("entrance", &self.entrance)
            .field("seed", &self.seed)
            .field("actors", &self.actors)
            .field("loot", &self.loot);
        if !self.threat_sources.is_empty() {
            blueprint.field("threat_sources", &self.threat_sources);
        }
        blueprint.finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZoneConnectionBlueprint {
    pub at: GridPos,
    pub destination: ZoneInfo,
    pub arrival: GridPos,
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZoneLink {
    pub destination: ContentId,
    pub arrival: Option<GridPos>,
}

// Keep resolved links byte-for-byte identical to their historical Debug form;
// only genuinely deferred links expose `arrival: None` in v14 state.
impl Debug for ZoneLink {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut link = formatter.debug_struct("ZoneLink");
        link.field("destination", &self.destination);
        match self.arrival {
            Some(arrival) => link.field("arrival", &arrival),
            None => link.field("arrival", &Option::<GridPos>::None),
        };
        link.finish()
    }
}

#[derive(Clone, Serialize, Deserialize)]
struct ZoneState {
    map: Map,
    actors: ActorRegistry,
    exit: Option<GridPos>,
    rng: GameRng,
    visibility: VisibilityState,
    ground: GroundItemRegistry,
    traces: MovementTraceMap,
    ground_effects: GroundEffectMap,
    explosive_devices: ExplosiveDeviceMap,
    intrusion: IntrusionState,
    electronic_warfare: ElectronicWarfareState,
    threat_sources: Vec<ThreatSourceState>,
}

impl Debug for ZoneState {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut state = formatter.debug_struct("ZoneState");
        state
            .field("map", &self.map)
            .field("actors", &self.actors)
            .field("exit", &self.exit)
            .field("rng", &self.rng)
            .field("visibility", &self.visibility)
            .field("ground", &self.ground)
            .field("traces", &self.traces);
        if !self.ground_effects.is_empty() {
            state.field("ground_effects", &self.ground_effects);
        }
        if !self.explosive_devices.is_empty() {
            state.field("explosive_devices", &self.explosive_devices);
        }
        if !self.intrusion.is_empty() {
            state.field("intrusion", &self.intrusion);
        }
        if !self.electronic_warfare.is_empty() {
            state.field("electronic_warfare", &self.electronic_warfare);
        }
        if !self.threat_sources.is_empty() {
            state.field("threat_sources", &self.threat_sources);
        }
        state.finish()
    }
}

pub struct WorldState {
    active: GameState,
    current: Option<ContentId>,
    information: BTreeMap<ContentId, ZoneInfo>,
    pending: BTreeMap<ContentId, ZoneBlueprint>,
    inactive: BTreeMap<ContentId, ZoneState>,
    links: BTreeMap<(ContentId, GridPos), ZoneLink>,
    facilities: BTreeMap<ContentId, FacilityState>,
    pending_facilities: BTreeMap<ContentId, FacilityBlueprint>,
    merchants: BTreeMap<ContentId, MerchantState>,
    clinics: BTreeMap<ContentId, ClinicState>,
    residents: BTreeMap<ContentId, Vec<ResidentState>>,
    quests: BTreeMap<ContentId, Vec<QuestState>>,
    player_credits: u32,
}

// Binary recovery boundary: every nested world type deliberately implements
// Serde so the compiler checks the complete snapshot contract in one place.
#[derive(Serialize, Deserialize)]
struct WorldStateSnapshot {
    active: GameStateSnapshot,
    current: Option<ContentId>,
    information: BTreeMap<ContentId, ZoneInfo>,
    pending: BTreeMap<ContentId, ZoneBlueprint>,
    inactive: BTreeMap<ContentId, ZoneState>,
    links: BTreeMap<(ContentId, GridPos), ZoneLink>,
    facilities: BTreeMap<ContentId, FacilityState>,
    pending_facilities: BTreeMap<ContentId, FacilityBlueprint>,
    merchants: BTreeMap<ContentId, MerchantState>,
    clinics: BTreeMap<ContentId, ClinicState>,
    residents: BTreeMap<ContentId, Vec<ResidentState>>,
    quests: BTreeMap<ContentId, Vec<QuestState>>,
    player_credits: u32,
}

// Keep the historical Debug representation byte-for-byte identical while the
// new registries are empty: suspension versions 1-4 fingerprint this text.
impl Debug for WorldState {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut state = formatter.debug_struct("WorldState");
        state
            .field("active", &self.active)
            .field("current", &self.current)
            .field("information", &self.information)
            .field("pending", &self.pending)
            .field("inactive", &self.inactive)
            .field("links", &self.links);
        if !self.facilities.is_empty() {
            state.field("facilities", &self.facilities);
        }
        if !self.pending_facilities.is_empty() {
            state.field("pending_facilities", &self.pending_facilities);
        }
        if !self.merchants.is_empty() {
            state.field("merchants", &self.merchants);
        }
        if !self.clinics.is_empty() {
            state.field("clinics", &self.clinics);
        }
        if !self.residents.is_empty() {
            state.field("residents", &self.residents);
        }
        if !self.quests.is_empty() {
            state.field("quests", &self.quests);
        }
        if self.player_credits > 0 {
            state.field("player_credits", &self.player_credits);
        }
        state.finish()
    }
}

// Existing read APIs still refer to the active zone. World commands must go
// through WorldState::process_player_command, never through a rendered view.
impl Deref for WorldState {
    type Target = GameState;
    fn deref(&self) -> &GameState {
        &self.active
    }
}
impl DerefMut for WorldState {
    fn deref_mut(&mut self) -> &mut GameState {
        &mut self.active
    }
}

impl WorldState {
    pub fn recovery_snapshot_bytes(&self) -> Result<Vec<u8>, String> {
        let snapshot = WorldStateSnapshot {
            active: self.active.snapshot()?,
            current: self.current.clone(),
            information: self.information.clone(),
            pending: self.pending.clone(),
            inactive: self.inactive.clone(),
            links: self.links.clone(),
            facilities: self.facilities.clone(),
            pending_facilities: self.pending_facilities.clone(),
            merchants: self.merchants.clone(),
            clinics: self.clinics.clone(),
            residents: self.residents.clone(),
            quests: self.quests.clone(),
            player_credits: self.player_credits,
        };
        let bytes = bincode::serialize(&snapshot)
            .map_err(|error| format!("Impossible d'encoder l'instantané : {error}"))?;
        if bytes.len() > MAX_WORLD_SNAPSHOT_BYTES {
            return Err("Instantané moteur trop volumineux.".to_owned());
        }
        Ok(bytes)
    }

    pub fn from_recovery_snapshot_bytes(bytes: &[u8], rules: GameRules) -> Result<Self, String> {
        if bytes.len() > MAX_WORLD_SNAPSHOT_BYTES {
            return Err("Instantané moteur trop volumineux.".to_owned());
        }
        let snapshot: WorldStateSnapshot = bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .with_limit(MAX_WORLD_SNAPSHOT_BYTES as u64)
            .reject_trailing_bytes()
            .deserialize(bytes)
            .map_err(|error| format!("Instantané moteur incompatible : {error}"))?;
        Ok(Self {
            active: GameState::from_snapshot(snapshot.active, rules),
            current: snapshot.current,
            information: snapshot.information,
            pending: snapshot.pending,
            inactive: snapshot.inactive,
            links: snapshot.links,
            facilities: snapshot.facilities,
            pending_facilities: snapshot.pending_facilities,
            merchants: snapshot.merchants,
            clinics: snapshot.clinics,
            residents: snapshot.residents,
            quests: snapshot.quests,
            player_credits: snapshot.player_credits,
        })
    }

    pub fn encode_recovery_snapshot(&self) -> Result<String, String> {
        Ok(base64::engine::general_purpose::STANDARD.encode(self.recovery_snapshot_bytes()?))
    }

    pub fn decode_recovery_snapshot(source: &str, rules: GameRules) -> Result<Self, String> {
        if source.len() > MAX_WORLD_SNAPSHOT_BYTES.saturating_mul(2) {
            return Err("Instantané moteur encodé trop volumineux.".to_owned());
        }
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(source)
            .map_err(|error| format!("Instantané moteur illisible : {error}"))?;
        Self::from_recovery_snapshot_bytes(&bytes, rules)
    }

    /// Legacy single-map replay, with byte-for-byte unchanged GameState Debug.
    pub fn single(active: GameState) -> Self {
        Self {
            active,
            current: None,
            information: BTreeMap::new(),
            pending: BTreeMap::new(),
            inactive: BTreeMap::new(),
            links: BTreeMap::new(),
            facilities: BTreeMap::new(),
            pending_facilities: BTreeMap::new(),
            merchants: BTreeMap::new(),
            clinics: BTreeMap::new(),
            residents: BTreeMap::new(),
            quests: BTreeMap::new(),
            player_credits: 0,
        }
    }

    pub fn enable(&mut self, info: ZoneInfo) -> Result<(), String> {
        if self.current.is_some() {
            return Err("World already initialized".into());
        }
        self.current = Some(info.id.clone());
        self.information.insert(info.id.clone(), info);
        self.active.exit = None;
        Ok(())
    }

    pub fn active_game(&self) -> &GameState {
        &self.active
    }

    pub fn player_may_take_property_of(&self, owner: &SocialGroupId) -> bool {
        self.active.player_may_take_property_of(owner)
    }

    pub fn grant_player_property_take_authorization(&mut self, owner: SocialGroupId) -> bool {
        self.active.grant_player_property_take_authorization(owner)
    }
    pub fn current_zone(&self) -> Option<&ZoneInfo> {
        self.current
            .as_ref()
            .and_then(|id| self.information.get(id))
    }
    pub fn zone_info(&self, id: &ContentId) -> Option<&ZoneInfo> {
        self.information.get(id)
    }
    pub fn visited_zone_count(&self) -> usize {
        1 + self.inactive.len()
    }
    pub fn passage(&self, at: GridPos) -> Option<&ZoneLink> {
        self.current
            .as_ref()
            .and_then(|id| self.links.get(&(id.clone(), at)))
    }

    pub fn passage_in(&self, zone: &ContentId, at: GridPos) -> Option<&ZoneLink> {
        self.links.get(&(zone.clone(), at))
    }

    /// Returns the first passage of a route that only crosses zones the player
    /// has already visited. Presentations can guide a return trip without
    /// exposing an unmaterialized map or an undiscovered shortcut.
    pub fn next_visited_passage_towards(&self, target: &ContentId) -> Option<GridPos> {
        let start = self.current.as_ref()?;
        if start == target {
            return None;
        }

        let mut visited_zones = self.inactive.keys().cloned().collect::<BTreeSet<_>>();
        visited_zones.insert(start.clone());
        if !visited_zones.contains(target) {
            return None;
        }

        let mut searched = BTreeSet::from([start.clone()]);
        let mut queue = VecDeque::from([(start.clone(), None)]);
        while let Some((zone, first_passage)) = queue.pop_front() {
            for ((source, position), link) in &self.links {
                if source != &zone
                    || !visited_zones.contains(&link.destination)
                    || !searched.insert(link.destination.clone())
                {
                    continue;
                }
                let first_passage = first_passage.unwrap_or(*position);
                if &link.destination == target {
                    return Some(first_passage);
                }
                queue.push_back((link.destination.clone(), Some(first_passage)));
            }
        }
        None
    }

    fn zone_egresses(&self, zone: &ContentId) -> Vec<GridPos> {
        let mut egresses: Vec<_> = self
            .links
            .keys()
            .filter_map(|(source, position)| (source == zone).then_some(*position))
            .collect();
        if self.current.as_ref() == Some(zone)
            && let Some(exit) = self.active.exit()
            && !egresses.contains(&exit)
        {
            egresses.push(exit);
        }
        egresses.sort();
        egresses
    }
    pub fn destination_name(&self, link: &ZoneLink) -> &str {
        self.information
            .get(&link.destination)
            .map_or("Zone", |info| info.name.as_str())
    }

    pub fn active_facility(&self) -> Option<&FacilityState> {
        self.current
            .as_ref()
            .and_then(|zone| self.facilities.get(zone))
    }

    pub fn facility_in_zone(&self, zone: &ContentId) -> Option<&FacilityState> {
        self.facilities.get(zone)
    }

    /// Records discovered anywhere in the instantiated world, including
    /// facilities that currently evolve off screen. Duplicate records are one
    /// discovery, even when several terminals contain the same text.
    pub fn discovered_data_terminal_records(&self) -> BTreeSet<ContentId> {
        self.facilities
            .values()
            .flat_map(FacilityState::accessed_data_terminal_records)
            .cloned()
            .collect()
    }

    pub fn active_worker_role(&self, worker: crate::entity::EntityId) -> Option<WorkerRole> {
        self.active_facility()
            .and_then(|facility| facility.worker_role(worker))
    }

    pub fn active_merchant(&self, provider: crate::entity::EntityId) -> bool {
        self.current
            .as_ref()
            .and_then(|zone| self.merchants.get(zone))
            .is_some_and(|merchant| merchant.provider == provider)
    }

    pub fn active_clinic(&self, provider: crate::entity::EntityId) -> bool {
        self.current
            .as_ref()
            .and_then(|zone| self.clinics.get(zone))
            .is_some_and(|clinic| clinic.provider == provider)
    }

    pub fn active_resident(&self, provider: crate::entity::EntityId) -> bool {
        self.current
            .as_ref()
            .and_then(|zone| self.residents.get(zone))
            .is_some_and(|residents| {
                residents
                    .iter()
                    .any(|resident| resident.provider == provider)
            })
    }

    pub fn active_quest_provider(&self, provider: crate::entity::EntityId) -> bool {
        self.current
            .as_ref()
            .and_then(|zone| self.quests.get(zone))
            .is_some_and(|quests| quests.iter().any(|quest| quest.provider == provider))
    }

    pub const fn player_credits(&self) -> u32 {
        self.player_credits
    }

    /// Isolated fixture support for UI diagnostics and deterministic replay
    /// tests. This is deliberately absent from release builds: normal damage
    /// must always enter through a gameplay command or an elapsed-turn effect.
    #[cfg(any(test, debug_assertions))]
    pub fn damage_player_for_diagnostic(&mut self, amount: u16) -> u16 {
        let player = self.active.player_id();
        self.active
            .actors
            .get_mut(player)
            .map_or(0, |actor| actor.apply_damage(amount))
    }

    /// Deterministic fixture support. Production acquisition still has to use
    /// loot, commerce or another recorded gameplay command.
    #[cfg(any(test, debug_assertions))]
    pub fn grant_player_item_for_diagnostic(
        &mut self,
        item: ItemId,
        quantity: u16,
    ) -> Result<(), String> {
        let maximum_stack = self
            .active
            .rules()
            .items
            .get(&item)
            .ok_or_else(|| format!("Unknown diagnostic item '{item}'"))?
            .maximum_stack();
        self.active
            .player_inventory_mut()
            .add(item, quantity, maximum_stack)
            .map(|_| ())
            .map_err(|error| error.to_string())
    }

    /// Returns only information the adjacent player can currently obtain by
    /// talking to this NPC. It is a query, not a command, so browsing a
    /// conversation cannot mutate or advance the deterministic simulation.
    pub fn npc_interaction(&self, provider: crate::entity::EntityId) -> Option<NpcInteraction> {
        if self.active.status != RunStatus::Active || self.active.phase != TurnPhase::AwaitingPlayer
        {
            return None;
        }
        let player = self.active.player_position()?;
        let actor = self.active.actors.get(provider)?;
        let position = actor.position();
        if !player.cardinal_neighbors().contains(&position)
            || !self.active.player_visibility.is_visible(position)
        {
            return None;
        }
        let locally_alerted = actor
            .local_alert()
            .is_some_and(|alert| alert.is_active(self.active.turn));
        let quests = self.npc_quest_views(provider);
        if let Some(merchant) = self
            .current
            .as_ref()
            .and_then(|zone| self.merchants.get(zone))
            .filter(|merchant| merchant.provider == provider)
        {
            let offers = merchant
                .offers
                .iter()
                .map(|offer| TradeOfferView {
                    item: offer.item.clone(),
                    stock: offer.stock,
                    price: offer.buy_price,
                })
                .collect();
            let resale = merchant
                .resale
                .iter()
                .map(|listing| TradeResaleView {
                    listing: listing.listing,
                    item: listing.item.clone(),
                    price: listing.price,
                    magic_modifiers: listing.magic_modifiers,
                })
                .collect();
            let gambles = merchant
                .gambles
                .iter()
                .map(|gamble| TradeGambleView {
                    item: gamble.item.clone(),
                    stock: gamble.stock,
                    price: gamble.price,
                })
                .collect();
            let sellable = self
                .active
                .player_inventory()
                .iter()
                .filter(|entry| {
                    self.active
                        .player_equipment()
                        .slot_of(entry.instance())
                        .is_none()
                        && entry.owner().is_none()
                })
                .filter_map(|entry| {
                    let standard = merchant
                        .offers
                        .iter()
                        .find(|offer| offer.item == *entry.item())
                        .map(|offer| offer.sell_price);
                    let gamble = merchant
                        .gambles
                        .iter()
                        .find(|gamble| gamble.item == *entry.item())
                        .map(|gamble| gamble.price / 2);
                    standard.or(gamble).map(|price| TradeSellView {
                        instance: entry.instance(),
                        item: entry.item().clone(),
                        quantity: entry.quantity(),
                        price,
                        magic_modifiers: entry.magic_modifiers(),
                    })
                })
                .collect();
            return Some(NpcInteraction {
                provider,
                position,
                role: NpcRole::Merchant,
                locally_alerted,
                resident_routine: None,
                contextual_dialogue_key: self.npc_world_state_dialogue(provider),
                services: vec![NpcService::Trade {
                    player_credits: self.player_credits,
                    merchant_credits: merchant.credits,
                    offers,
                    resale,
                    gambles,
                    sellable,
                }],
                quests: quests.clone(),
            });
        }
        if let Some(clinic) = self
            .current
            .as_ref()
            .and_then(|zone| self.clinics.get(zone))
            .filter(|clinic| clinic.provider == provider)
        {
            let player_actor = self.active.actors.get(self.active.player)?;
            let current_integrity = player_actor.integrity();
            let maximum_integrity = player_actor.maximum_integrity();
            let restore_amount = maximum_integrity
                .saturating_sub(current_integrity)
                .min(clinic.maximum_restoration);
            let price = u32::from(restore_amount)
                .checked_mul(clinic.price_per_point)
                .expect("validated clinic price must remain bounded");
            return Some(NpcInteraction {
                provider,
                position,
                role: NpcRole::Healer,
                locally_alerted,
                resident_routine: None,
                contextual_dialogue_key: self.npc_world_state_dialogue(provider),
                services: vec![NpcService::Treatment {
                    player_credits: self.player_credits,
                    clinic_credits: clinic.credits,
                    current_integrity,
                    maximum_integrity,
                    maximum_restoration: clinic.maximum_restoration,
                    restore_amount,
                    price,
                    routine: clinic.routine_state(position),
                }],
                quests: quests.clone(),
            });
        }
        if let Some(resident) = self
            .current
            .as_ref()
            .and_then(|zone| self.residents.get(zone))
            .and_then(|residents| {
                residents
                    .iter()
                    .find(|resident| resident.provider == provider)
            })
        {
            return Some(NpcInteraction {
                provider,
                position,
                role: NpcRole::Resident,
                locally_alerted,
                resident_routine: Some(resident.routine_state(position)),
                contextual_dialogue_key: self.npc_world_state_dialogue(provider),
                services: Vec::new(),
                quests: quests.clone(),
            });
        }
        if !quests.is_empty() {
            return Some(NpcInteraction {
                provider,
                position,
                role: NpcRole::QuestContact,
                locally_alerted,
                resident_routine: None,
                contextual_dialogue_key: self.npc_world_state_dialogue(provider),
                services: Vec::new(),
                quests,
            });
        }
        let facility = self.active_facility()?;
        let role = facility.worker_role(provider)?;
        let services = if role == WorkerRole::Technician {
            let inventory = self.active.player_inventory();
            facility
                .repair_order_summaries()
                .map(|summary| {
                    let carried: u32 = inventory
                        .iter()
                        .filter(|entry| entry.item() == &summary.required_item)
                        .map(|entry| u32::from(entry.quantity()))
                        .sum();
                    let state = match summary.status {
                        RepairStatus::WaitingForMaterial if summary.missing_quantity > 0 => {
                            let known_source =
                                self.active.ground_items().iter().find_map(|(_, stack)| {
                                    (stack.item() == &summary.required_item)
                                        .then_some(stack.position())
                                });
                            NpcServiceState::MaterialRequired {
                                player_can_supply: carried >= u32::from(summary.missing_quantity),
                                known_source,
                            }
                        }
                        RepairStatus::WaitingForMaterial => NpcServiceState::InTransit,
                        RepairStatus::MaterialAvailable | RepairStatus::Assigned { .. } => {
                            NpcServiceState::Queued
                        }
                        RepairStatus::InProgress {
                            remaining_turns, ..
                        } => NpcServiceState::InProgress { remaining_turns },
                        RepairStatus::Completed => NpcServiceState::Operational,
                    };
                    NpcService::FacilityMaintenance {
                        order: summary.order,
                        required_item: summary.required_item,
                        missing_quantity: summary.missing_quantity,
                        state,
                    }
                })
                .collect()
        } else {
            Vec::new()
        };
        Some(NpcInteraction {
            provider,
            position,
            role: NpcRole::Worker(role),
            locally_alerted,
            resident_routine: None,
            contextual_dialogue_key: self.npc_world_state_dialogue(provider),
            services,
            quests,
        })
    }

    fn npc_quest_views(&self, provider: crate::entity::EntityId) -> Vec<NpcQuestView> {
        let Some(quests) = self.current.as_ref().and_then(|zone| self.quests.get(zone)) else {
            return Vec::new();
        };
        let completed = self.completed_quest_ids();
        let world_states = self.active_world_state_ids();
        quests
            .iter()
            .filter(|quest| {
                quest.provider == provider
                    && !quest.excluded
                    && quest
                        .prerequisites
                        .iter()
                        .all(|required| completed.contains(required))
                    && quest
                        .required_world_states
                        .iter()
                        .all(|required| world_states.contains(required))
            })
            .map(|quest| self.quest_view(quest))
            .collect()
    }

    fn completed_quest_ids(&self) -> BTreeSet<QuestId> {
        self.quests
            .values()
            .flatten()
            .filter(|quest| quest.completed)
            .map(|quest| quest.definition.id().clone())
            .collect()
    }

    fn active_world_state_ids(&self) -> BTreeSet<ContentId> {
        self.quests
            .values()
            .flatten()
            .filter(|quest| quest.completed)
            .flat_map(|quest| {
                quest
                    .completion_world_states
                    .iter()
                    .map(|state| state.id.clone())
            })
            .collect()
    }

    pub fn world_state_active(&self, state: &ContentId) -> bool {
        self.quests.values().flatten().any(|quest| {
            quest.completed
                && quest
                    .completion_world_states
                    .iter()
                    .any(|known| &known.id == state)
        })
    }

    fn npc_world_state_dialogue(&self, provider: crate::entity::EntityId) -> Option<String> {
        self.quests
            .values()
            .flatten()
            .filter(|quest| quest.completed && quest.provider == provider)
            .flat_map(|quest| quest.completion_world_states.iter())
            .filter_map(|state| state.provider_dialogue_key.clone())
            .next_back()
    }

    pub fn quest_marker(&self, provider: crate::entity::EntityId) -> Option<QuestMarker> {
        let views = self.npc_quest_views(provider);
        if views
            .iter()
            .any(|quest| quest.status == QuestStatus::ReadyToComplete)
        {
            Some(QuestMarker::ReadyToComplete)
        } else if views
            .iter()
            .any(|quest| quest.status == QuestStatus::Available)
        {
            Some(QuestMarker::Available)
        } else {
            None
        }
    }

    /// Stable journal order is zone ID then authored quest order. Reading it
    /// never advances time or reveals an offer that was not accepted.
    pub fn quest_journal(&self) -> Vec<QuestJournalEntry> {
        self.quests
            .iter()
            .filter_map(|(zone, quests)| {
                self.information.get(zone).map(|information| {
                    quests
                        .iter()
                        .filter(|quest| quest.accepted || quest.completed)
                        .map(|quest| QuestJournalEntry {
                            zone: information.clone(),
                            giver: quest.provider,
                            quest: self.quest_view(quest),
                        })
                        .collect::<Vec<_>>()
                })
            })
            .flatten()
            .collect()
    }

    /// The displayed role of an accepted quest's giver, even when their zone
    /// is inactive. This does not expose unaccepted quest contacts.
    pub fn quest_giver_role(
        &self,
        zone: &ContentId,
        giver: crate::entity::EntityId,
    ) -> Option<NpcRole> {
        self.quests
            .get(zone)?
            .iter()
            .any(|quest| quest.provider == giver && (quest.accepted || quest.completed))
            .then_some(())?;

        if self
            .merchants
            .get(zone)
            .is_some_and(|merchant| merchant.provider == giver)
        {
            return Some(NpcRole::Merchant);
        }
        if self
            .clinics
            .get(zone)
            .is_some_and(|clinic| clinic.provider == giver)
        {
            return Some(NpcRole::Healer);
        }
        if self
            .residents
            .get(zone)
            .is_some_and(|residents| residents.iter().any(|resident| resident.provider == giver))
        {
            return Some(NpcRole::Resident);
        }
        // A quest-bearing worker is presented as a quest contact by the
        // interaction view too; keep the journal designation consistent.
        Some(NpcRole::QuestContact)
    }

    fn quest_view(&self, quest: &QuestState) -> NpcQuestView {
        let (objective, objective_complete) = match (&quest.definition, &quest.progress) {
            (QuestDefinition::Delivery(definition), QuestProgress::Delivery) => {
                let carried_quantity = self
                    .active
                    .player_inventory()
                    .iter()
                    .filter(|entry| {
                        entry.item() == &definition.required_item
                            && self
                                .active
                                .player_equipment()
                                .slot_of(entry.instance())
                                .is_none()
                    })
                    .map(|entry| u32::from(entry.quantity()))
                    .sum::<u32>()
                    .min(u32::from(u16::MAX)) as u16;
                (
                    QuestObjectiveView::Delivery {
                        required_item: definition.required_item.clone(),
                        required_quantity: definition.required_quantity,
                        carried_quantity,
                    },
                    carried_quantity >= definition.required_quantity,
                )
            }
            (
                QuestDefinition::ExploreZones(definition),
                QuestProgress::ExploreZones { discovered, .. },
            ) => {
                let explored_zones = discovered.len().min(usize::from(u16::MAX)) as u16;
                (
                    QuestObjectiveView::ExploreZones {
                        required_zones: definition.required_zones,
                        explored_zones,
                        site_record_required: !definition.qualifying_records.is_empty(),
                    },
                    explored_zones >= definition.required_zones,
                )
            }
            (
                QuestDefinition::AccessDataRecord(definition),
                QuestProgress::AccessDataRecord { accessed },
            ) => (
                QuestObjectiveView::AccessDataRecord {
                    record: definition.record.clone(),
                    accessed: *accessed,
                },
                *accessed,
            ),
            (
                QuestDefinition::DefeatTargets(definition),
                QuestProgress::DefeatTargets { defeated_quantity },
            ) => (
                QuestObjectiveView::DefeatTargets {
                    target_tag: definition.target_tag.clone(),
                    required_quantity: definition.required_quantity,
                    defeated_quantity: *defeated_quantity,
                },
                *defeated_quantity >= definition.required_quantity,
            ),
            _ => unreachable!("quest definition and progress are created together"),
        };
        let status = if quest.completed {
            QuestStatus::Completed
        } else if quest.accepted && objective_complete {
            QuestStatus::ReadyToComplete
        } else if quest.accepted {
            QuestStatus::Active
        } else {
            QuestStatus::Available
        };
        NpcQuestView {
            id: quest.definition.id().clone(),
            title_key: quest.definition.title_key().to_owned(),
            summary_key: quest.definition.summary_key().to_owned(),
            status,
            objective,
            reward_credits: quest.definition.reward_credits(),
            reward_experience: quest.reward_experience,
            reward_items: quest.reward_items.clone(),
            completion_world_states: quest.completion_world_states.clone(),
            completion_world_effects: quest.completion_world_effects.clone(),
            choice_prompt_key: quest.choice_prompt_key.clone(),
        }
    }

    pub fn register_merchant(
        &mut self,
        zone: ContentId,
        provider: crate::entity::EntityId,
        player_starting_credits: u32,
        definition: MerchantDefinition,
        gamble_seed: u64,
    ) -> Result<(), String> {
        let MerchantDefinition {
            initial_credits: merchant_credits,
            offers,
            gambles,
            gamble_scaling,
            ..
        } = definition;
        if self.merchants.contains_key(&zone) {
            return Err("Zone already has a merchant".into());
        }
        if !gamble_scaling.is_valid() {
            return Err("Invalid merchant gamble scaling".into());
        }
        let actor_exists = if self.current.as_ref() == Some(&zone) {
            self.active.actors.get(provider).is_some()
        } else {
            self.inactive
                .get(&zone)
                .is_some_and(|state| state.actors.get(provider).is_some())
        };
        if !actor_exists {
            return Err("Merchant provider is absent from its zone".into());
        }
        let zone_depth = self
            .information
            .get(&zone)
            .ok_or("Merchant zone information is absent")?
            .depth;
        let mut resolved = Vec::new();
        for offer in offers {
            let item = offer.item;
            let stock = offer.initial_stock;
            let buy_price = offer.buy_price;
            let sell_price = offer.sell_price;
            let minimum_depth = offer.minimum_depth;
            let maximum_depth = offer.maximum_depth;
            if maximum_depth.is_some_and(|maximum| maximum < minimum_depth) {
                return Err("Invalid merchant offer depth range".into());
            }
            if zone_depth < minimum_depth
                || maximum_depth.is_some_and(|maximum| zone_depth > maximum)
            {
                continue;
            }
            let maximum_stack = self
                .active
                .rules()
                .items
                .get(&item)
                .ok_or_else(|| format!("Unknown merchant item '{item}'"))?
                .maximum_stack();
            if stock == 0 || buy_price == 0 || sell_price == 0 || sell_price > buy_price {
                return Err("Invalid merchant offer".into());
            }
            if resolved
                .iter()
                .any(|known: &MerchantOfferState| known.item == item)
            {
                return Err("Duplicate merchant offer".into());
            }
            resolved.push(MerchantOfferState {
                item,
                stock,
                buy_price,
                sell_price,
                maximum_stack,
            });
        }
        if resolved.is_empty() {
            return Err("Merchant has no offers".into());
        }
        let mut resolved_gambles = Vec::new();
        for gamble in gambles {
            let item = gamble.item;
            let stock = gamble.initial_stock;
            let price = gamble.price;
            let definition = self
                .active
                .rules()
                .items
                .get(&item)
                .ok_or_else(|| format!("Unknown merchant gamble item '{item}'"))?;
            if definition.kind() != crate::item::ItemKind::Armor || stock == 0 || price == 0 {
                return Err("Invalid merchant gamble".into());
            }
            resolved_gambles.push(MerchantGambleState { item, stock, price });
        }
        if resolved_gambles.is_empty() {
            return Err("Merchant has no gambles".into());
        }
        let initialize_player_wallet = self.merchants.is_empty() && self.clinics.is_empty();
        self.merchants.insert(
            zone,
            MerchantState {
                provider,
                credits: merchant_credits,
                offers: resolved,
                resale: Vec::new(),
                gambles: resolved_gambles,
                next_listing: 1,
                gamble_rng: GameRng::from_seed(gamble_seed),
                gamble_scaling,
            },
        );
        if initialize_player_wallet {
            self.player_credits = player_starting_credits;
        }
        Ok(())
    }

    pub fn register_clinic(
        &mut self,
        zone: ContentId,
        provider: crate::entity::EntityId,
        player_starting_credits: u32,
        definition: ClinicDefinition,
    ) -> Result<(), String> {
        let ClinicDefinition {
            work_position,
            break_position,
            maximum_integrity: _,
            initial_credits,
            maximum_restoration,
            price_per_point,
            work_turns,
            break_turns,
            maximum_path_search,
        } = definition;
        if self.clinics.contains_key(&zone) {
            return Err("Zone already has a clinic".into());
        }
        if maximum_restoration == 0
            || price_per_point == 0
            || work_turns == 0
            || break_turns == 0
            || maximum_path_search == 0
            || u32::from(maximum_restoration)
                .checked_mul(price_per_point)
                .is_none()
        {
            return Err("Invalid clinic definition".into());
        }
        let (map, actors) = if self.current.as_ref() == Some(&zone) {
            (&self.active.map, &self.active.actors)
        } else {
            let state = self
                .inactive
                .get(&zone)
                .ok_or("Clinic zone is unavailable")?;
            (&state.map, &state.actors)
        };
        let provider_position = actors
            .get(provider)
            .map(Actor::position)
            .ok_or("Clinic provider is absent from its zone")?;
        if provider_position != work_position
            || !map.is_walkable(work_position)
            || !map.is_walkable(break_position)
            || find_path(
                map,
                work_position,
                break_position,
                maximum_path_search,
                |_| true,
            )
            .is_none()
        {
            return Err("Clinic routine anchors are invalid or unreachable".into());
        }
        let initialize_player_wallet = self.merchants.is_empty() && self.clinics.is_empty();
        self.clinics.insert(
            zone,
            ClinicState {
                provider,
                credits: initial_credits,
                maximum_restoration,
                price_per_point,
                work_position,
                break_position,
                work_turns,
                break_turns,
                maximum_path_search,
                routine_working: true,
                routine_remaining_turns: work_turns,
            },
        );
        if initialize_player_wallet {
            self.player_credits = player_starting_credits;
        }
        Ok(())
    }

    pub fn register_resident(
        &mut self,
        zone: ContentId,
        provider: crate::entity::EntityId,
        definition: ResidentDefinition,
    ) -> Result<(), String> {
        let ResidentDefinition {
            residence_position,
            gathering_position,
            maximum_integrity: _,
            residence_turns,
            gathering_turns,
            maximum_path_search,
        } = definition;
        if residence_position == gathering_position
            || residence_turns == 0
            || gathering_turns == 0
            || maximum_path_search == 0
        {
            return Err("Invalid resident definition".into());
        }
        if self
            .residents
            .get(&zone)
            .is_some_and(|residents| residents.iter().any(|known| known.provider == provider))
        {
            return Err("Resident provider is already registered in this zone".into());
        }
        let (map, actors) = if self.current.as_ref() == Some(&zone) {
            (&self.active.map, &self.active.actors)
        } else {
            let state = self
                .inactive
                .get(&zone)
                .ok_or("Resident zone is unavailable")?;
            (&state.map, &state.actors)
        };
        let provider_position = actors
            .get(provider)
            .map(Actor::position)
            .ok_or("Resident provider is absent from its zone")?;
        if provider_position != residence_position
            || !map.is_walkable(residence_position)
            || !map.is_walkable(gathering_position)
            || find_path(
                map,
                residence_position,
                gathering_position,
                maximum_path_search,
                |_| true,
            )
            .is_none()
        {
            return Err("Resident routine anchors are invalid or unreachable".into());
        }
        self.residents.entry(zone).or_default().push(ResidentState {
            provider,
            residence_position,
            gathering_position,
            residence_turns,
            gathering_turns,
            maximum_path_search,
            routine_at_residence: true,
            routine_remaining_turns: residence_turns,
        });
        Ok(())
    }

    /// Attaches a quest contract to an actor without changing its ordinary
    /// resident, merchant, healer or worker role.
    pub fn register_delivery_quest(
        &mut self,
        zone: ContentId,
        provider: crate::entity::EntityId,
        definition: DeliveryQuestDefinition,
    ) -> Result<(), String> {
        self.register_quest(zone, provider, definition.into())
    }

    pub fn register_exploration_quest(
        &mut self,
        zone: ContentId,
        provider: crate::entity::EntityId,
        definition: ExplorationQuestDefinition,
    ) -> Result<(), String> {
        self.register_quest(zone, provider, definition.into())
    }

    pub fn register_data_record_quest(
        &mut self,
        zone: ContentId,
        provider: crate::entity::EntityId,
        definition: DataRecordQuestDefinition,
    ) -> Result<(), String> {
        self.register_quest(zone, provider, definition.into())
    }

    pub fn register_defeat_targets_quest(
        &mut self,
        zone: ContentId,
        provider: crate::entity::EntityId,
        definition: DefeatTargetsQuestDefinition,
    ) -> Result<(), String> {
        self.register_quest(zone, provider, definition.into())
    }

    pub fn register_quest(
        &mut self,
        zone: ContentId,
        provider: crate::entity::EntityId,
        definition: QuestDefinition,
    ) -> Result<(), String> {
        self.register_quest_with_metadata(
            zone,
            provider,
            definition,
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            None,
            None,
            0,
            Vec::new(),
        )
    }

    pub fn register_authored_quest(
        &mut self,
        zone: ContentId,
        provider: crate::entity::EntityId,
        authored: HubQuestDefinition,
    ) -> Result<(), String> {
        self.register_quest_with_metadata(
            zone,
            provider,
            authored.quest,
            authored.prerequisites,
            authored.required_world_states,
            authored.completion_world_states,
            authored.completion_world_effects,
            authored.choice_group,
            authored.choice_prompt_key,
            authored.reward_experience,
            authored.reward_items,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn register_quest_with_metadata(
        &mut self,
        zone: ContentId,
        provider: crate::entity::EntityId,
        definition: QuestDefinition,
        prerequisites: Vec<QuestId>,
        required_world_states: Vec<ContentId>,
        completion_world_states: Vec<QuestWorldStateDefinition>,
        completion_world_effects: Vec<QuestWorldEffectDefinition>,
        choice_group: Option<ContentId>,
        choice_prompt_key: Option<String>,
        reward_experience: u64,
        reward_items: Vec<QuestItemRewardDefinition>,
    ) -> Result<(), String> {
        let valid = match &definition {
            QuestDefinition::Delivery(delivery) => {
                !delivery.title_key.trim().is_empty()
                    && !delivery.summary_key.trim().is_empty()
                    && delivery.required_quantity > 0
            }
            QuestDefinition::ExploreZones(exploration) => {
                !exploration.title_key.trim().is_empty()
                    && !exploration.summary_key.trim().is_empty()
                    && exploration.required_zones > 0
            }
            QuestDefinition::AccessDataRecord(data_record) => {
                !data_record.title_key.trim().is_empty()
                    && !data_record.summary_key.trim().is_empty()
            }
            QuestDefinition::DefeatTargets(defeat) => {
                !defeat.title_key.trim().is_empty()
                    && !defeat.summary_key.trim().is_empty()
                    && defeat.required_quantity > 0
            }
        };
        if !valid {
            return Err("Invalid quest definition".into());
        }
        if self
            .quests
            .values()
            .flatten()
            .any(|known| known.definition.id() == definition.id())
        {
            return Err("Duplicate quest ID".into());
        }
        if self
            .quests
            .get(&zone)
            .is_some_and(|quests| quests.len() >= 16)
        {
            return Err("Too many quests for this zone".into());
        }
        if prerequisites.iter().any(|required| {
            !self
                .quests
                .values()
                .flatten()
                .any(|known| known.definition.id() == required)
        }) {
            return Err(
                "Quest prerequisite is unknown or registered after its dependent quest".into(),
            );
        }
        if required_world_states.len() > MAX_QUEST_WORLD_STATES
            || completion_world_states.len() > MAX_QUEST_WORLD_STATES
            || required_world_states
                .iter()
                .enumerate()
                .any(|(index, required)| {
                    required_world_states[..index].contains(required)
                        || !self.quests.values().flatten().any(|known| {
                            known
                                .completion_world_states
                                .iter()
                                .any(|completed| &completed.id == required)
                        })
                })
            || completion_world_states
                .iter()
                .enumerate()
                .any(|(index, state)| {
                    state.summary_key.trim().is_empty()
                        || state
                            .provider_dialogue_key
                            .as_ref()
                            .is_some_and(|key| key.trim().is_empty())
                        || completion_world_states[..index]
                            .iter()
                            .any(|known| known.id == state.id)
                        || self.quests.values().flatten().any(|known| {
                            known
                                .completion_world_states
                                .iter()
                                .any(|completed| completed.id == state.id)
                        })
                })
        {
            return Err("Invalid or unavailable quest world state metadata".into());
        }
        let effect_map = if self.current.as_ref() == Some(&zone) {
            Some(&self.active.map)
        } else {
            self.inactive.get(&zone).map(|state| &state.map)
        };
        let effect_facility = self.facilities.get(&zone);
        if completion_world_effects.len() > MAX_QUEST_WORLD_EFFECTS
            || completion_world_effects
                .iter()
                .enumerate()
                .any(|(index, effect)| {
                    !effect.has_valid_shape()
                        || completion_world_effects[..index]
                            .iter()
                            .any(|known| known.targets_same_element(effect))
                        || self.quests.get(&zone).is_some_and(|quests| {
                            quests.iter().any(|known| {
                                known
                                    .completion_world_effects
                                    .iter()
                                    .any(|known| known.targets_same_element(effect))
                            })
                        })
                        || match effect {
                            QuestWorldEffectDefinition::UnlockDoor { position, .. } => !effect_map
                                .is_some_and(|map| {
                                    matches!(
                                        map.tile(*position).map(|tile| tile.terrain),
                                        Some(Terrain::Door(DoorState::Locked))
                                    )
                                }),
                            QuestWorldEffectDefinition::UpdateDataTerminal {
                                installation,
                                record,
                                ..
                            } => !effect_facility.is_some_and(|facility| {
                                facility
                                    .installation(installation)
                                    .is_some_and(|candidate| {
                                        candidate.capabilities().iter().any(|capability| {
                                        matches!(
                                            capability,
                                            crate::facility::InstallationCapability::DataTerminal {
                                                record: current_record
                                            } if current_record != record
                                        )
                                    })
                                    })
                            }),
                            QuestWorldEffectDefinition::GrantPropertyTakeAuthorization {
                                owner,
                                ..
                            } => self.player_may_take_property_of(owner),
                        }
                })
        {
            return Err("Invalid or unavailable quest world effect metadata".into());
        }
        if choice_group.is_some() != choice_prompt_key.is_some()
            || choice_prompt_key
                .as_ref()
                .is_some_and(|prompt| prompt.trim().is_empty())
        {
            return Err("Invalid quest choice metadata".into());
        }
        if let Some(group) = &choice_group
            && self.quests.values().flatten().any(|known| {
                known.choice_group.as_ref() == Some(group)
                    && (known.provider != provider || known.choice_prompt_key != choice_prompt_key)
            })
        {
            return Err("Quest choice group must share one provider and prompt".into());
        }
        if let QuestDefinition::Delivery(delivery) = &definition
            && self
                .active
                .rules()
                .items
                .get(&delivery.required_item)
                .is_none()
        {
            return Err(format!("Unknown quest item '{}'", delivery.required_item));
        }
        for reward in &reward_items {
            if reward.quantity == 0 || self.active.rules().items.get(&reward.item).is_none() {
                return Err(format!("Invalid quest reward item '{}'", reward.item));
            }
        }
        let provider_exists = if self.current.as_ref() == Some(&zone) {
            self.active.actors.get(provider).is_some()
        } else {
            self.inactive
                .get(&zone)
                .is_some_and(|state| state.actors.get(provider).is_some())
        };
        if !provider_exists {
            return Err("Quest provider is absent from its zone".into());
        }
        let progress = match &definition {
            QuestDefinition::Delivery(_) => QuestProgress::Delivery,
            QuestDefinition::ExploreZones(_) => QuestProgress::ExploreZones {
                baseline: BTreeSet::new(),
                discovered: BTreeSet::new(),
            },
            QuestDefinition::AccessDataRecord(_) => {
                QuestProgress::AccessDataRecord { accessed: false }
            }
            QuestDefinition::DefeatTargets(_) => QuestProgress::DefeatTargets {
                defeated_quantity: 0,
            },
        };
        self.quests.entry(zone).or_default().push(QuestState {
            provider,
            definition,
            progress,
            prerequisites,
            required_world_states,
            completion_world_states,
            completion_world_effects,
            choice_group,
            choice_prompt_key,
            reward_experience,
            reward_items,
            accepted: false,
            completed: false,
            excluded: false,
        });
        Ok(())
    }

    /// Installs a deterministic local simulation in the current, pending or
    /// already visited zone. Pending definitions are instantiated only once,
    /// when their actors receive global entity IDs.
    pub fn register_facility(
        &mut self,
        zone: ContentId,
        blueprint: FacilityBlueprint,
    ) -> Result<(), String> {
        if self.facilities.contains_key(&zone) || self.pending_facilities.contains_key(&zone) {
            return Err("Zone already has a facility simulation".into());
        }
        let threat_sources: Vec<_> = if self.current.as_ref() == Some(&zone) {
            self.active
                .threat_sources
                .iter()
                .map(|source| {
                    (
                        source.position(),
                        threat_actor_can_investigate(&source.actor),
                    )
                })
                .collect()
        } else if let Some(state) = self.inactive.get(&zone) {
            state
                .threat_sources
                .iter()
                .map(|source| {
                    (
                        source.position(),
                        threat_actor_can_investigate(&source.actor),
                    )
                })
                .collect()
        } else if let Some(pending) = self.pending.get(&zone) {
            pending
                .threat_sources
                .iter()
                .map(|source| (source.position, threat_actor_can_investigate(&source.actor)))
                .collect()
        } else {
            return Err("Unknown zone for facility simulation".into());
        };
        validate_facility_reinforcement_sources(&blueprint, &threat_sources)?;
        if self.current.as_ref() == Some(&zone) {
            let facility = FacilityState::instantiate(
                blueprint,
                &mut self.active.map,
                &mut self.active.actors,
            )
            .map_err(|error| error.to_string())?;
            self.facilities.insert(zone, facility);
            return Ok(());
        }
        if let Some(state) = self.inactive.get_mut(&zone) {
            let facility = FacilityState::instantiate(blueprint, &mut state.map, &mut state.actors)
                .map_err(|error| error.to_string())?;
            self.facilities.insert(zone, facility);
            return Ok(());
        }
        if self.pending.contains_key(&zone) {
            self.pending_facilities.insert(zone, blueprint);
            return Ok(());
        }
        unreachable!("zone existence was checked before facility validation")
    }

    pub fn add_zone(&mut self, blueprint: ZoneBlueprint) -> Result<(), String> {
        if self.current.is_none() || self.information.contains_key(&blueprint.info.id) {
            return Err("Zone missing its world or duplicate zone ID".into());
        }
        validate_interactive_map(
            &blueprint.map,
            blueprint.entrance,
            blueprint.entrance,
            &[],
            MapValidationRules::default(),
        )
        .map_err(|e| e.to_string())?;
        // Validate all definitions and placements now, not after leaving a zone.
        self.prepare_zone(&blueprint)?;
        self.information
            .insert(blueprint.info.id.clone(), blueprint.info.clone());
        self.pending.insert(blueprint.info.id.clone(), blueprint);
        Ok(())
    }

    fn map_for(&self, id: &ContentId) -> Option<(&Map, GridPos)> {
        if self.current.as_ref() == Some(id) {
            return self.active.player_position().map(|p| (&self.active.map, p));
        }
        self.pending.get(id).map(|b| (&b.map, b.entrance))
    }

    fn map_for_anchor(&self, id: &ContentId, anchor: GridPos) -> Option<(&Map, GridPos)> {
        self.map_for(id)
            .or_else(|| self.inactive.get(id).map(|state| (&state.map, anchor)))
    }

    /// Register a reciprocal connection before play. Both directions must be
    /// walkable/reachable after decoration and cannot overwrite another link.
    pub fn connect(
        &mut self,
        from: ContentId,
        at: GridPos,
        to: ContentId,
        arrival: GridPos,
    ) -> Result<(), String> {
        if from == to
            || self.links.contains_key(&(from.clone(), at))
            || self.links.contains_key(&(to.clone(), arrival))
        {
            return Err("Duplicate passage or self connection".into());
        }
        for (id, anchor) in [(&from, at), (&to, arrival)] {
            let (map, start) = self.map_for(id).ok_or("Unknown/unavailable zone")?;
            if !map.is_walkable(anchor) {
                return Err("Blocked passage".into());
            }
            validate_interactive_map(map, start, anchor, &[], MapValidationRules::default())
                .map_err(|e| e.to_string())?;
            if self
                .pending
                .get(id)
                .is_some_and(|b| b.actors.iter().any(|a| a.position() == anchor))
            {
                return Err("Actor spawned on a passage".into());
            }
        }
        self.links.insert(
            (from.clone(), at),
            ZoneLink {
                destination: to.clone(),
                arrival: Some(arrival),
            },
        );
        self.links.insert(
            (to, arrival),
            ZoneLink {
                destination: from,
                arrival: Some(at),
            },
        );
        Ok(())
    }

    /// Declares only the known end of a connection. The destination metadata
    /// is cheap, while its map remains absent until a content provider resolves
    /// the passage immediately before first travel.
    pub fn declare_deferred_connection(
        &mut self,
        from: ContentId,
        at: GridPos,
        destination: ZoneInfo,
    ) -> Result<(), String> {
        if from == destination.id
            || self.current.is_none()
            || self.information.contains_key(&destination.id)
            || self.links.contains_key(&(from.clone(), at))
        {
            return Err("Duplicate passage, zone ID or self connection".into());
        }
        let (map, start) = self
            .map_for(&from)
            .ok_or("Unknown/unavailable source zone")?;
        if !map.is_walkable(at) {
            return Err("Blocked passage".into());
        }
        validate_interactive_map(map, start, at, &[], MapValidationRules::default())
            .map_err(|error| error.to_string())?;
        if self
            .pending
            .get(&from)
            .is_some_and(|blueprint| blueprint.actors.iter().any(|actor| actor.position() == at))
        {
            return Err("Actor spawned on a passage".into());
        }
        self.links.insert(
            (from, at),
            ZoneLink {
                destination: destination.id.clone(),
                arrival: None,
            },
        );
        self.information.insert(destination.id.clone(), destination);
        Ok(())
    }

    /// Declares a reciprocal connection whose stable arrival tile is already
    /// known even though one or both local maps may not have been generated.
    /// Repeating the exact declaration is harmless, which lets independently
    /// discovered atlas paths close a cycle without replacing either link.
    pub fn declare_deferred_connection_at(
        &mut self,
        from: ContentId,
        at: GridPos,
        destination: ZoneInfo,
        arrival: GridPos,
    ) -> Result<(), String> {
        self.declare_deferred_connections_at(
            from,
            &[ZoneConnectionBlueprint {
                at,
                destination,
                arrival,
            }],
        )
    }

    /// Atomically registers several known-arrival passages leaving one zone.
    /// The source map is validated once with every requested anchor.
    pub fn declare_deferred_connections_at(
        &mut self,
        from: ContentId,
        connections: &[ZoneConnectionBlueprint],
    ) -> Result<(), String> {
        if connections.is_empty() {
            return Ok(());
        }
        if self.current.is_none() {
            return Err("World not initialized".into());
        }
        let source_anchors = connections
            .iter()
            .map(|connection| connection.at)
            .collect::<Vec<_>>();
        let (source_map, source_start) = self
            .map_for_anchor(&from, source_anchors[0])
            .ok_or("Unknown/unavailable source zone")?;
        validate_interactive_map(
            source_map,
            source_start,
            source_anchors[0],
            &source_anchors[1..],
            MapValidationRules::default(),
        )
        .map_err(|error| error.to_string())?;
        if self.pending.get(&from).is_some_and(|blueprint| {
            connections.iter().any(|connection| {
                blueprint
                    .actors
                    .iter()
                    .any(|actor| actor.position() == connection.at)
            })
        }) {
            return Err("Actor spawned on a passage".into());
        }

        let mut next_information = self.information.clone();
        let mut next_links = self.links.clone();
        for connection in connections {
            if from == connection.destination.id {
                return Err("Self connection".into());
            }
            if next_information
                .get(&connection.destination.id)
                .is_some_and(|known| known != &connection.destination)
            {
                return Err("Conflicting metadata for known destination".into());
            }
            let forward = ZoneLink {
                destination: connection.destination.id.clone(),
                arrival: Some(connection.arrival),
            };
            let reverse = ZoneLink {
                destination: from.clone(),
                arrival: Some(connection.at),
            };
            let forward_key = (from.clone(), connection.at);
            let reverse_key = (connection.destination.id.clone(), connection.arrival);
            match (next_links.get(&forward_key), next_links.get(&reverse_key)) {
                (Some(known_forward), Some(known_reverse))
                    if known_forward == &forward && known_reverse == &reverse =>
                {
                    continue;
                }
                (Some(_), _) | (_, Some(_)) => {
                    return Err("Conflicting passage declaration".into());
                }
                (None, None) => {}
            }
            if let Some((map, start)) =
                self.map_for_anchor(&connection.destination.id, connection.arrival)
            {
                if !map.is_walkable(connection.arrival) {
                    return Err("Blocked passage arrival".into());
                }
                validate_interactive_map(
                    map,
                    start,
                    connection.arrival,
                    &[],
                    MapValidationRules::default(),
                )
                .map_err(|error| error.to_string())?;
                if self
                    .pending
                    .get(&connection.destination.id)
                    .is_some_and(|blueprint| {
                        blueprint
                            .actors
                            .iter()
                            .any(|actor| actor.position() == connection.arrival)
                    })
                {
                    return Err("Actor spawned on a passage arrival".into());
                }
            }
            next_information
                .entry(connection.destination.id.clone())
                .or_insert_with(|| connection.destination.clone());
            next_links.insert(forward_key, forward);
            next_links.insert(reverse_key, reverse);
        }
        self.information = next_information;
        self.links = next_links;
        Ok(())
    }

    pub fn deferred_passage_destination(&self, at: GridPos) -> Option<&ContentId> {
        self.passage(at)
            .filter(|link| link.arrival.is_none())
            .map(|link| &link.destination)
    }

    pub fn unmaterialized_passage_destination(&self, at: GridPos) -> Option<&ContentId> {
        self.passage(at)
            .filter(|link| {
                self.current.as_ref() != Some(&link.destination)
                    && !self.pending.contains_key(&link.destination)
                    && !self.inactive.contains_key(&link.destination)
            })
            .map(|link| &link.destination)
    }

    /// A provider may perform expensive generation only after this preflight.
    /// The check is non-mutating and mirrors the travel interaction boundary.
    pub fn can_materialize_passage(&self, at: GridPos) -> bool {
        let Some(origin) = self.active.player_position() else {
            return false;
        };
        self.active.status == RunStatus::Active
            && self.active.phase == TurnPhase::AwaitingPlayer
            && (origin == at || origin.cardinal_neighbors().contains(&at))
            && self.active.player_visibility.is_visible(at)
            && self.active.map.is_walkable(at)
            && self.unmaterialized_passage_destination(at).is_some()
    }

    /// Installs a generated destination without advancing time. Validation is
    /// completed before any registry changes; the following ordinary Interact
    /// command performs the actual travel and remains the only recorded action.
    pub fn materialize_passage_destination(
        &mut self,
        at: GridPos,
        blueprint: ZoneBlueprint,
    ) -> Result<(), String> {
        self.materialize_passage_destination_with_connections(at, blueprint, &[])
    }

    /// Atomically installs a generated zone and every outward connection its
    /// provider derived from the same regional descriptor. A bad exit cannot
    /// leave behind a half-generated zone or a partially connected atlas.
    pub fn materialize_passage_destination_with_connections(
        &mut self,
        at: GridPos,
        blueprint: ZoneBlueprint,
        connections: &[ZoneConnectionBlueprint],
    ) -> Result<(), String> {
        self.materialize_passage_destination_with_connections_and_facility(
            at,
            blueprint,
            connections,
            None,
        )
    }

    /// Atomically validates and installs a generated zone, its atlas links and
    /// its optional local simulation. A malformed generated facility can never
    /// leave a half-materialized destination behind.
    pub fn materialize_passage_destination_with_connections_and_facility(
        &mut self,
        at: GridPos,
        blueprint: ZoneBlueprint,
        connections: &[ZoneConnectionBlueprint],
        facility: Option<FacilityBlueprint>,
    ) -> Result<(), String> {
        if !self.can_materialize_passage(at) {
            return Err("Deferred passage is not currently interactable".into());
        }
        let from = self.current.clone().ok_or("World is not initialized")?;
        let destination = self
            .unmaterialized_passage_destination(at)
            .cloned()
            .ok_or("Passage destination is already materialized")?;
        let declared_arrival = self
            .links
            .get(&(from.clone(), at))
            .and_then(|link| link.arrival);
        if blueprint.info.id != destination
            || self.information.get(&destination) != Some(&blueprint.info)
            || self.pending.contains_key(&destination)
            || self.inactive.contains_key(&destination)
            || declared_arrival.is_some_and(|arrival| arrival != blueprint.entrance)
        {
            return Err("Generated zone does not match its deferred destination".into());
        }
        let reciprocal_key = (destination.clone(), blueprint.entrance);
        if let Some(link) = self.links.get(&reciprocal_key)
            && (link.destination != from || link.arrival != Some(at))
        {
            return Err("Generated zone entrance already has a passage".into());
        }
        let mut probe = self.prepare_zone(&blueprint)?;
        if probe.actors.entity_at(blueprint.entrance).is_some() {
            return Err("Actor spawned on a passage".into());
        }
        if facility.is_some()
            && (self.facilities.contains_key(&destination)
                || self.pending_facilities.contains_key(&destination))
        {
            return Err("Generated zone already has a facility simulation".into());
        }
        if let Some(facility) = facility.as_ref() {
            validate_facility_reinforcement_sources(
                facility,
                &blueprint
                    .threat_sources
                    .iter()
                    .map(|source| (source.position, threat_actor_can_investigate(&source.actor)))
                    .collect::<Vec<_>>(),
            )?;
            FacilityState::instantiate(facility.clone(), &mut probe.map, &mut probe.actors)
                .map_err(|error| error.to_string())?;
        }
        let generated_passages = connections
            .iter()
            .map(|connection| connection.at)
            .collect::<Vec<_>>();
        let validation_exit = generated_passages
            .first()
            .copied()
            .unwrap_or(blueprint.entrance);
        validate_interactive_map(
            &blueprint.map,
            blueprint.entrance,
            validation_exit,
            generated_passages.get(1..).unwrap_or_default(),
            MapValidationRules::default(),
        )
        .map_err(|error| format!("Generated destination navigation is invalid: {error}"))?;
        if connections.iter().any(|connection| {
            blueprint
                .actors
                .iter()
                .any(|actor| actor.position() == connection.at)
        }) {
            return Err("Actor spawned on a generated passage".into());
        }

        let mut next_information = self.information.clone();
        let mut next_links = self.links.clone();
        next_links
            .get_mut(&(from.clone(), at))
            .expect("checked deferred passage")
            .arrival = Some(blueprint.entrance);
        next_links.entry(reciprocal_key).or_insert(ZoneLink {
            destination: from,
            arrival: Some(at),
        });
        for connection in connections {
            if connection.destination.id == destination {
                return Err("Generated zone cannot connect to itself".into());
            }
            if !blueprint.map.is_walkable(connection.at) {
                return Err("Generated zone contains a blocked passage".into());
            }
            if next_information
                .get(&connection.destination.id)
                .is_some_and(|known| known != &connection.destination)
            {
                return Err("Conflicting metadata for generated destination".into());
            }
            if let Some((map, start)) =
                self.map_for_anchor(&connection.destination.id, connection.arrival)
            {
                if !map.is_walkable(connection.arrival) {
                    return Err("Generated connection has a blocked arrival".into());
                }
                validate_interactive_map(
                    map,
                    start,
                    connection.arrival,
                    &[],
                    MapValidationRules {
                        require_all_walkable_connected: false,
                        ..MapValidationRules::default()
                    },
                )
                .map_err(|error| format!("Known destination navigation is invalid: {error}"))?;
                if self
                    .pending
                    .get(&connection.destination.id)
                    .is_some_and(|target| {
                        target
                            .actors
                            .iter()
                            .any(|actor| actor.position() == connection.arrival)
                    })
                {
                    return Err("Actor spawned on a generated arrival".into());
                }
            }
            let forward_key = (destination.clone(), connection.at);
            let reverse_key = (connection.destination.id.clone(), connection.arrival);
            let forward = ZoneLink {
                destination: connection.destination.id.clone(),
                arrival: Some(connection.arrival),
            };
            let reverse = ZoneLink {
                destination: destination.clone(),
                arrival: Some(connection.at),
            };
            match (next_links.get(&forward_key), next_links.get(&reverse_key)) {
                (Some(known_forward), Some(known_reverse))
                    if known_forward == &forward && known_reverse == &reverse => {}
                (Some(_), _) | (_, Some(_)) => {
                    return Err("Conflicting generated passage declaration".into());
                }
                (None, None) => {
                    next_links.insert(forward_key, forward);
                    next_links.insert(reverse_key, reverse);
                }
            }
            next_information
                .entry(connection.destination.id.clone())
                .or_insert_with(|| connection.destination.clone());
        }

        self.information = next_information;
        self.links = next_links;
        if let Some(facility) = facility {
            self.pending_facilities
                .insert(destination.clone(), facility);
        }
        self.pending.insert(destination, blueprint);
        Ok(())
    }

    fn prepare_zone(&self, blueprint: &ZoneBlueprint) -> Result<ZoneState, String> {
        let mut probe = GameState::new_with_rules(
            blueprint.map.clone(),
            blueprint.entrance,
            blueprint.seed,
            self.active.rules.clone(),
        )
        .map_err(|e| e.to_string())?;
        probe.actors.remove(probe.player);
        probe.actors.synchronize_ids(&self.active.actors);
        probe
            .ground_items
            .synchronize_ids(&self.active.ground_items);
        for actor in &blueprint.actors {
            probe
                .spawn_actor(actor.clone())
                .map_err(|e| e.to_string())?;
        }
        for loot in &blueprint.loot {
            probe
                .spawn_ground_item_with_owner(
                    loot.position(),
                    loot.item().clone(),
                    loot.quantity(),
                    loot.owner().cloned(),
                )
                .map_err(|e| e.to_string())?;
        }
        probe.install_threat_sources(blueprint.threat_sources.clone())?;
        Ok(ZoneState {
            map: probe.map,
            actors: probe.actors,
            exit: None,
            rng: probe.rng,
            visibility: VisibilityState::default(),
            ground: probe.ground_items,
            traces: MovementTraceMap::default(),
            ground_effects: GroundEffectMap::default(),
            explosive_devices: ExplosiveDeviceMap::default(),
            intrusion: IntrusionState::default(),
            electronic_warfare: ElectronicWarfareState::default(),
            threat_sources: probe.threat_sources,
        })
    }

    pub fn process_player_command(&mut self, command: GameCommand) -> CommandOutcome {
        let previous_turn = self.active.turn;
        let event_checkpoint = self.active.events.len();
        let unauthorized_property_take = if matches!(&command, GameCommand::PickUp) {
            self.active.player_position().and_then(|position| {
                let ground_item = self.active.ground_items().item_at(position)?;
                let stack = self.active.ground_items().get(ground_item)?;
                let owner = stack.owner()?.clone();
                (!self.active.player_may_take_property_of(&owner)).then(|| ObservedPropertyTake {
                    turn: self.active.turn(),
                    taker: self.active.player_id(),
                    owner,
                    item: stack.item().clone(),
                    quantity: stack.quantity(),
                    at: position,
                })
            })
        } else {
            None
        };
        let interaction_target = match &command {
            GameCommand::Interact { target } => Some(*target),
            _ => None,
        };
        let world_action = match &command {
            GameCommand::BuyItem { merchant, item } => {
                Some(self.buy_from_merchant(*merchant, item.clone(), command.clone()))
            }
            GameCommand::BuyResaleItem { merchant, listing } => {
                Some(self.buy_resale_from_merchant(*merchant, *listing, command.clone()))
            }
            GameCommand::GambleItem { merchant, item } => {
                Some(self.gamble_with_merchant(*merchant, item.clone(), command.clone()))
            }
            GameCommand::SellItem { merchant, item } => {
                Some(self.sell_to_merchant(*merchant, *item, command.clone()))
            }
            GameCommand::ReceiveTreatment { healer } => {
                Some(self.receive_treatment(*healer, command.clone()))
            }
            GameCommand::AcceptQuest { giver, quest } => {
                Some(self.accept_quest(*giver, quest, command.clone()))
            }
            GameCommand::CompleteQuest { giver, quest } => {
                Some(self.complete_quest(*giver, quest, command.clone()))
            }
            _ => None,
        };
        let outcome = if let Some(outcome) = world_action {
            outcome
        } else if let Some(target) = interaction_target
            && let Some(link) = self.passage(target).cloned()
        {
            self.travel(target, link)
        } else if let Some(target) = interaction_target
            && let Some(outcome) = self.interact_with_facility(target)
        {
            outcome
        } else {
            self.active.process_player_command(command)
        };
        if outcome == CommandOutcome::Applied {
            let reported_incidents: Vec<_> = self.active.events[event_checkpoint..]
                .iter()
                .filter_map(|event| match event {
                    GameEvent::PropertyTakeReported { recipient, at, .. } => {
                        Some((*recipient, *at))
                    }
                    _ => None,
                })
                .collect();
            if let Some(facility) = self
                .current
                .as_ref()
                .and_then(|zone| self.facilities.get_mut(zone))
            {
                let response_events: Vec<_> = reported_incidents
                    .into_iter()
                    .filter_map(|(recipient, at)| {
                        facility.assign_reported_incident_investigation(recipient, at)
                    })
                    .collect();
                let visible: Vec<_> = response_events
                    .into_iter()
                    .filter(|event| {
                        facility_event_is_visible(
                            event,
                            facility,
                            &self.active.actors,
                            &self.active.player_visibility,
                        )
                    })
                    .map(GameEvent::Facility)
                    .collect();
                self.active.events.extend(visible);
            }
        }
        if outcome == CommandOutcome::Applied
            && let Some(incident) = unauthorized_property_take
        {
            let witness_sources: Vec<_> = self.active.events[event_checkpoint..]
                .iter()
                .filter_map(|event| match event {
                    GameEvent::PropertyTakeWitnessed { witness, .. } => Some(*witness),
                    _ => None,
                })
                .collect();
            let current = self.current.clone();
            let egresses = current
                .as_ref()
                .map(|zone| self.zone_egresses(zone))
                .unwrap_or_default();
            if let Some(facility) = current
                .as_ref()
                .and_then(|zone| self.facilities.get_mut(zone))
            {
                let mut events = Vec::new();
                for source in witness_sources {
                    events.extend(facility.receive_installed_property_report(
                        source,
                        &self.active.map,
                        &self.active.actors,
                        &incident,
                    ));
                }
                events.extend(
                    facility.observe_unauthorized_property_take(&self.active.map, &incident),
                );
                let alarm_sources: Vec<_> = events
                    .iter()
                    .filter_map(|event| match event {
                        FacilityEvent::SecurityAlarmRaised { installation, .. } => {
                            Some(installation.clone())
                        }
                        _ => None,
                    })
                    .collect();
                match facility.activate_security_alarm_responses(
                    &alarm_sources,
                    &mut self.active.map,
                    SecurityAlarmResponseContext {
                        actors: &self.active.actors,
                        ground: &self.active.ground_items,
                        protected_position: Some(incident.at),
                        egresses: &egresses,
                        turn: self.active.turn,
                    },
                ) {
                    Ok(response_events) => events.extend(response_events),
                    Err(error) => events.push(FacilityEvent::SimulationFault(error.to_string())),
                }
                resolve_reinforcement_requests(&mut self.active, &mut events);
                let visible: Vec<_> = events
                    .into_iter()
                    .filter(|event| {
                        facility_event_is_visible(
                            event,
                            facility,
                            &self.active.actors,
                            &self.active.player_visibility,
                        )
                    })
                    .map(GameEvent::Facility)
                    .collect();
                self.active.events.extend(visible);
            }
        }
        if self.active.turn != previous_turn && self.active.status == RunStatus::Active {
            for elapsed_turn in previous_turn..self.active.turn {
                if let Some(current) = self.current.clone()
                    && let Some(facility) = self.facilities.get_mut(&current)
                {
                    let result = (|| {
                        let mut events = facility.expire_security_alarm_responses(
                            &mut self.active.map,
                            elapsed_turn.saturating_add(1),
                        )?;
                        events.extend(facility.tick(
                            &mut self.active.map,
                            &mut self.active.actors,
                            &mut self.active.ground_items,
                        )?);
                        Ok::<_, crate::facility::FacilityRuntimeError>(events)
                    })();
                    if let Some(origin) = self.active.player_position() {
                        self.active.player_visibility.recompute(
                            &self.active.map,
                            origin,
                            self.active.rules.player_field_of_view,
                        );
                    }
                    let events = result.unwrap_or_else(|error| {
                        vec![FacilityEvent::SimulationFault(error.to_string())]
                    });
                    let visible: Vec<_> = events
                        .into_iter()
                        .filter(|event| {
                            facility_event_is_visible(
                                event,
                                facility,
                                &self.active.actors,
                                &self.active.player_visibility,
                            )
                        })
                        .map(GameEvent::Facility)
                        .collect();
                    self.active.events.extend(visible);
                }
                if let Some(clinic) = self
                    .current
                    .as_ref()
                    .and_then(|zone| self.clinics.get_mut(zone))
                {
                    tick_clinic_routine(clinic, &self.active.map, &mut self.active.actors);
                }
                if let Some(residents) = self
                    .current
                    .as_ref()
                    .and_then(|zone| self.residents.get_mut(zone))
                {
                    for resident in residents {
                        tick_resident_routine(resident, &self.active.map, &mut self.active.actors);
                    }
                }
                // BTreeMap order + independent RNG streams make the result stable.
                for (id, zone) in &mut self.inactive {
                    self.active.tick_background(zone, elapsed_turn);
                    if let Some(facility) = self.facilities.get_mut(id) {
                        let _ = facility.expire_security_alarm_responses(
                            &mut zone.map,
                            elapsed_turn.saturating_add(1),
                        );
                        let _ = facility.tick(&mut zone.map, &mut zone.actors, &mut zone.ground);
                    }
                    if let Some(clinic) = self.clinics.get_mut(id) {
                        tick_clinic_routine(clinic, &zone.map, &mut zone.actors);
                    }
                    if let Some(residents) = self.residents.get_mut(id) {
                        for resident in residents {
                            tick_resident_routine(resident, &zone.map, &mut zone.actors);
                        }
                    }
                }
            }
        }
        let defeated_tags = self.active.events[event_checkpoint..]
            .iter()
            .filter_map(|event| match event {
                GameEvent::EntityDefeatedByPlayer { tags, .. } => Some(tags.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        for tags in defeated_tags {
            self.record_defeat_quest_progress(&tags);
        }
        outcome
    }

    fn validate_quest_access(
        &self,
        provider: crate::entity::EntityId,
        quest: &QuestId,
    ) -> Result<(ContentId, usize), CommandRejection> {
        if self.active.status != RunStatus::Active {
            return Err(CommandRejection::RunEnded);
        }
        if self.active.phase != TurnPhase::AwaitingPlayer {
            return Err(CommandRejection::NotPlayersTurn);
        }
        let zone = self
            .current
            .as_ref()
            .ok_or(CommandRejection::QuestUnavailable)?;
        let quests = self
            .quests
            .get(zone)
            .ok_or(CommandRejection::QuestUnavailable)?;
        let index = quests
            .iter()
            .position(|known| known.provider == provider && known.definition.id() == quest)
            .ok_or(CommandRejection::QuestUnavailable)?;
        let player = self
            .active
            .player_position()
            .ok_or(CommandRejection::MissingPlayer)?;
        let position = self
            .active
            .actors
            .get(provider)
            .map(Actor::position)
            .ok_or(CommandRejection::QuestUnavailable)?;
        if !player.cardinal_neighbors().contains(&position)
            || !self.active.player_visibility.is_visible(position)
        {
            return Err(CommandRejection::InteractionOutOfReach);
        }
        Ok((zone.clone(), index))
    }

    fn accept_quest(
        &mut self,
        provider: crate::entity::EntityId,
        quest: &QuestId,
        command: GameCommand,
    ) -> CommandOutcome {
        let (zone, index) = match self.validate_quest_access(provider, quest) {
            Ok(access) => access,
            Err(reason) => return CommandOutcome::Rejected(reason),
        };
        let previous = self.quests[&zone][index].clone();
        if previous.excluded {
            return CommandOutcome::Rejected(CommandRejection::QuestBranchUnavailable);
        }
        let completed = self.completed_quest_ids();
        if previous
            .prerequisites
            .iter()
            .any(|required| !completed.contains(required))
        {
            return CommandOutcome::Rejected(CommandRejection::QuestPrerequisiteIncomplete);
        }
        let world_states = self.active_world_state_ids();
        if previous
            .required_world_states
            .iter()
            .any(|required| !world_states.contains(required))
        {
            return CommandOutcome::Rejected(CommandRejection::QuestWorldStateUnavailable);
        }
        if previous.completed {
            return CommandOutcome::Rejected(CommandRejection::QuestAlreadyCompleted);
        }
        if previous.accepted {
            return CommandOutcome::Rejected(CommandRejection::QuestAlreadyAccepted);
        }
        let visited_zones = self
            .current
            .iter()
            .chain(self.inactive.keys())
            .cloned()
            .collect::<BTreeSet<_>>();
        let previous_quests = self.quests.clone();
        let accepted = &mut self.quests.get_mut(&zone).expect("quest access validated")[index];
        accepted.accepted = true;
        if let QuestProgress::ExploreZones {
            baseline,
            discovered,
        } = &mut accepted.progress
        {
            *baseline = visited_zones;
            discovered.clear();
        }
        if let QuestProgress::AccessDataRecord { accessed } = &mut accepted.progress {
            *accessed = false;
        }
        if let QuestProgress::DefeatTargets { defeated_quantity } = &mut accepted.progress {
            *defeated_quantity = 0;
        }
        if let Some(group) = previous.choice_group.as_ref() {
            for known in self.quests.values_mut().flatten() {
                if known.definition.id() != quest && known.choice_group.as_ref() == Some(group) {
                    known.excluded = true;
                }
            }
        }
        let outcome = self.active.process_player_command(command);
        if outcome == CommandOutcome::AppliedWithoutTime {
            self.active.events.push(GameEvent::QuestAccepted {
                giver: provider,
                quest: quest.clone(),
            });
        } else {
            self.quests = previous_quests;
        }
        outcome
    }

    fn complete_quest(
        &mut self,
        provider: crate::entity::EntityId,
        quest: &QuestId,
        command: GameCommand,
    ) -> CommandOutcome {
        let (zone, index) = match self.validate_quest_access(provider, quest) {
            Ok(access) => access,
            Err(reason) => return CommandOutcome::Rejected(reason),
        };
        let previous_quest = self.quests[&zone][index].clone();
        if previous_quest.completed {
            return CommandOutcome::Rejected(CommandRejection::QuestAlreadyCompleted);
        }
        if !previous_quest.accepted {
            return CommandOutcome::Rejected(CommandRejection::QuestNotAccepted);
        }
        let previous_inventory = self.active.player_inventory().clone();
        let (objective_inventory, completion) =
            match (&previous_quest.definition, &previous_quest.progress) {
                (QuestDefinition::Delivery(definition), QuestProgress::Delivery) => {
                    let removable = previous_inventory
                        .iter()
                        .filter(|entry| {
                            entry.item() == &definition.required_item
                                && self
                                    .active
                                    .player_equipment()
                                    .slot_of(entry.instance())
                                    .is_none()
                        })
                        .map(|entry| (entry.instance(), entry.quantity()))
                        .collect::<Vec<_>>();
                    let available = removable
                        .iter()
                        .fold(0u16, |total, (_, quantity)| total.saturating_add(*quantity));
                    if available < definition.required_quantity {
                        return CommandOutcome::Rejected(
                            CommandRejection::QuestObjectiveIncomplete,
                        );
                    }
                    let mut next_inventory = previous_inventory.clone();
                    let mut remaining = definition.required_quantity;
                    for (instance, quantity) in removable {
                        let taken = remaining.min(quantity);
                        next_inventory
                            .remove(instance, taken)
                            .expect("quest inventory was cloned from validated entries");
                        remaining -= taken;
                        if remaining == 0 {
                            break;
                        }
                    }
                    (
                        Some(next_inventory),
                        QuestCompletion::Delivery {
                            item: definition.required_item.clone(),
                            quantity: definition.required_quantity,
                        },
                    )
                }
                (
                    QuestDefinition::ExploreZones(definition),
                    QuestProgress::ExploreZones { discovered, .. },
                ) => {
                    let explored = discovered.len().min(usize::from(u16::MAX)) as u16;
                    if explored < definition.required_zones {
                        return CommandOutcome::Rejected(
                            CommandRejection::QuestObjectiveIncomplete,
                        );
                    }
                    (
                        None,
                        QuestCompletion::ExploreZones {
                            zones: definition.required_zones,
                        },
                    )
                }
                (
                    QuestDefinition::AccessDataRecord(definition),
                    QuestProgress::AccessDataRecord { accessed },
                ) => {
                    if !accessed {
                        return CommandOutcome::Rejected(
                            CommandRejection::QuestObjectiveIncomplete,
                        );
                    }
                    (
                        None,
                        QuestCompletion::AccessDataRecord {
                            record: definition.record.clone(),
                        },
                    )
                }
                (
                    QuestDefinition::DefeatTargets(definition),
                    QuestProgress::DefeatTargets { defeated_quantity },
                ) => {
                    if *defeated_quantity < definition.required_quantity {
                        return CommandOutcome::Rejected(
                            CommandRejection::QuestObjectiveIncomplete,
                        );
                    }
                    (
                        None,
                        QuestCompletion::DefeatTargets {
                            target_tag: definition.target_tag.clone(),
                            quantity: definition.required_quantity,
                        },
                    )
                }
                _ => return CommandOutcome::Rejected(CommandRejection::QuestUnavailable),
            };
        let inventory_will_change =
            objective_inventory.is_some() || !previous_quest.reward_items.is_empty();
        let mut next_inventory = objective_inventory.unwrap_or_else(|| previous_inventory.clone());
        for reward in &previous_quest.reward_items {
            let Some(definition) = self.active.rules().items.get(&reward.item) else {
                return CommandOutcome::Rejected(CommandRejection::QuestRewardUnavailable);
            };
            if next_inventory
                .add(
                    reward.item.clone(),
                    reward.quantity,
                    definition.maximum_stack(),
                )
                .is_err()
            {
                return CommandOutcome::Rejected(CommandRejection::QuestRewardUnavailable);
            }
        }
        let previous_player_credits = self.player_credits;
        let Some(next_player_credits) = self
            .player_credits
            .checked_add(previous_quest.definition.reward_credits())
        else {
            return CommandOutcome::Rejected(CommandRejection::QuestRewardUnavailable);
        };
        let mut next_map = None;
        let mut next_facility = None;
        let mut next_authorizations = Vec::new();
        for effect in &previous_quest.completion_world_effects {
            match effect {
                QuestWorldEffectDefinition::UnlockDoor { position, .. } => {
                    let map = next_map.get_or_insert_with(|| self.active.map.clone());
                    if !matches!(
                        map.tile(*position).map(|tile| tile.terrain),
                        Some(Terrain::Door(DoorState::Locked))
                    ) || map
                        .set_terrain(*position, Terrain::Door(DoorState::Closed))
                        .is_err()
                    {
                        return CommandOutcome::Rejected(
                            CommandRejection::QuestWorldEffectUnavailable,
                        );
                    }
                }
                QuestWorldEffectDefinition::UpdateDataTerminal {
                    installation,
                    record,
                    ..
                } => {
                    let Some(facility) = next_facility
                        .get_or_insert_with(|| self.facilities.get(&zone).cloned())
                        .as_mut()
                    else {
                        return CommandOutcome::Rejected(
                            CommandRejection::QuestWorldEffectUnavailable,
                        );
                    };
                    if facility
                        .update_data_terminal_record(installation, record.clone())
                        .is_err()
                    {
                        return CommandOutcome::Rejected(
                            CommandRejection::QuestWorldEffectUnavailable,
                        );
                    }
                }
                QuestWorldEffectDefinition::GrantPropertyTakeAuthorization { owner, .. } => {
                    if self.player_may_take_property_of(owner) {
                        return CommandOutcome::Rejected(
                            CommandRejection::QuestWorldEffectUnavailable,
                        );
                    }
                    next_authorizations.push(owner.clone());
                }
            }
        }
        if inventory_will_change {
            *self.active.player_inventory_mut() = next_inventory;
        }
        self.player_credits = next_player_credits;
        let previous_map = next_map.map(|next| std::mem::replace(&mut self.active.map, next));
        let previous_facility = next_facility.flatten().map(|next| {
            self.facilities
                .insert(zone.clone(), next)
                .expect("quest installation effect requires a registered facility")
        });
        for owner in &next_authorizations {
            assert!(
                self.active
                    .grant_player_property_take_authorization(owner.clone()),
                "quest authorization effect was preflighted"
            );
        }
        self.quests.get_mut(&zone).expect("quest access validated")[index].completed = true;
        let outcome = self.active.process_player_command(command);
        if outcome == CommandOutcome::Applied {
            if previous_quest.reward_experience > 0 {
                let reward_key =
                    RewardKey::new(format!("quest:{}", previous_quest.definition.id().as_str()))
                        .expect("validated content IDs produce stable reward keys");
                self.active
                    .award_one_time_experience(previous_quest.reward_experience, reward_key);
            }
            self.active.events.push(GameEvent::QuestCompleted {
                giver: provider,
                quest: quest.clone(),
                objective: completion,
                reward_credits: previous_quest.definition.reward_credits(),
                reward_experience: previous_quest.reward_experience,
                reward_items: previous_quest.reward_items.clone(),
                world_states: previous_quest.completion_world_states.clone(),
                world_effects: previous_quest.completion_world_effects.clone(),
                player_credits: next_player_credits,
            });
        } else {
            *self.active.player_inventory_mut() = previous_inventory;
            self.player_credits = previous_player_credits;
            if let Some(previous_map) = previous_map {
                self.active.map = previous_map;
            }
            if let Some(previous_facility) = previous_facility {
                self.facilities.insert(zone.clone(), previous_facility);
            }
            for owner in &next_authorizations {
                assert!(
                    self.active.revoke_player_property_take_authorization(owner),
                    "only the current quest transaction can own this new authorization"
                );
            }
            self.quests.get_mut(&zone).expect("quest access validated")[index] = previous_quest;
        }
        outcome
    }

    fn validate_merchant_access(
        &self,
        provider: crate::entity::EntityId,
    ) -> Result<ContentId, CommandRejection> {
        if self.active.status != RunStatus::Active {
            return Err(CommandRejection::RunEnded);
        }
        if self.active.phase != TurnPhase::AwaitingPlayer {
            return Err(CommandRejection::NotPlayersTurn);
        }
        let zone = self
            .current
            .as_ref()
            .ok_or(CommandRejection::MerchantUnavailable)?;
        let merchant = self
            .merchants
            .get(zone)
            .filter(|merchant| merchant.provider == provider)
            .ok_or(CommandRejection::MerchantUnavailable)?;
        let player = self
            .active
            .player_position()
            .ok_or(CommandRejection::MissingPlayer)?;
        let position = self
            .active
            .actors
            .get(merchant.provider)
            .map(Actor::position)
            .ok_or(CommandRejection::MerchantUnavailable)?;
        if !player.cardinal_neighbors().contains(&position)
            || !self.active.player_visibility.is_visible(position)
        {
            return Err(CommandRejection::InteractionOutOfReach);
        }
        Ok(zone.clone())
    }

    fn validate_clinic_access(
        &self,
        provider: crate::entity::EntityId,
    ) -> Result<ContentId, CommandRejection> {
        if self.active.status != RunStatus::Active {
            return Err(CommandRejection::RunEnded);
        }
        if self.active.phase != TurnPhase::AwaitingPlayer {
            return Err(CommandRejection::NotPlayersTurn);
        }
        let zone = self
            .current
            .as_ref()
            .ok_or(CommandRejection::ClinicUnavailable)?;
        let clinic = self
            .clinics
            .get(zone)
            .filter(|clinic| clinic.provider == provider)
            .ok_or(CommandRejection::ClinicUnavailable)?;
        let player = self
            .active
            .player_position()
            .ok_or(CommandRejection::MissingPlayer)?;
        let position = self
            .active
            .actors
            .get(clinic.provider)
            .map(Actor::position)
            .ok_or(CommandRejection::ClinicUnavailable)?;
        if !player.cardinal_neighbors().contains(&position)
            || !self.active.player_visibility.is_visible(position)
        {
            return Err(CommandRejection::InteractionOutOfReach);
        }
        Ok(zone.clone())
    }

    fn receive_treatment(
        &mut self,
        provider: crate::entity::EntityId,
        command: GameCommand,
    ) -> CommandOutcome {
        let reject = |reason| CommandOutcome::Rejected(reason);
        let zone = match self.validate_clinic_access(provider) {
            Ok(zone) => zone,
            Err(reason) => return reject(reason),
        };
        let player = self.active.player;
        let previous_integrity = self
            .active
            .actors
            .get(player)
            .map(Actor::integrity)
            .unwrap_or(0);
        let maximum_integrity = self
            .active
            .actors
            .get(player)
            .map(Actor::maximum_integrity)
            .unwrap_or(previous_integrity);
        let previous_player_credits = self.player_credits;
        let previous_clinic = self
            .clinics
            .get(&zone)
            .expect("clinic access validated")
            .clone();
        let amount = maximum_integrity
            .saturating_sub(previous_integrity)
            .min(previous_clinic.maximum_restoration);
        if amount == 0 {
            return reject(CommandRejection::TreatmentNotNeeded);
        }
        let price = u32::from(amount)
            .checked_mul(previous_clinic.price_per_point)
            .expect("validated clinic price must remain bounded");
        let Some(next_player_credits) = self.player_credits.checked_sub(price) else {
            return reject(CommandRejection::InsufficientCredits);
        };
        let Some(next_clinic_credits) = previous_clinic.credits.checked_add(price) else {
            return reject(CommandRejection::ClinicUnavailable);
        };
        let mut next_clinic = previous_clinic.clone();
        next_clinic.credits = next_clinic_credits;
        let restored = self
            .active
            .actors
            .get_mut(player)
            .map(|actor| actor.restore_integrity(amount))
            .unwrap_or(0);
        if restored != amount {
            return reject(CommandRejection::ClinicUnavailable);
        }
        self.player_credits = next_player_credits;
        self.clinics.insert(zone.clone(), next_clinic);
        let outcome = self.active.process_player_command(command);
        if outcome == CommandOutcome::Applied {
            self.active.events.push(GameEvent::TreatmentReceived {
                healer: provider,
                amount,
                price,
                player_credits: next_player_credits,
            });
        } else {
            if let Some(actor) = self.active.actors.get_mut(player) {
                actor.apply_damage(restored);
            }
            self.player_credits = previous_player_credits;
            self.clinics.insert(zone, previous_clinic);
        }
        outcome
    }

    fn buy_from_merchant(
        &mut self,
        provider: crate::entity::EntityId,
        item: ItemId,
        command: GameCommand,
    ) -> CommandOutcome {
        let reject = |reason| CommandOutcome::Rejected(reason);
        let zone = match self.validate_merchant_access(provider) {
            Ok(zone) => zone,
            Err(reason) => return reject(reason),
        };
        let previous_inventory = self.active.player_inventory().clone();
        let previous_player_credits = self.player_credits;
        let previous_merchant = self
            .merchants
            .get(&zone)
            .expect("merchant access validated")
            .clone();
        let Some(index) = previous_merchant
            .offers
            .iter()
            .position(|offer| offer.item == item)
        else {
            return reject(CommandRejection::MerchantOfferUnavailable);
        };
        let offer = &previous_merchant.offers[index];
        let price = offer.buy_price;
        let maximum_stack = offer.maximum_stack;
        if offer.stock == 0 {
            return reject(CommandRejection::MerchantOutOfStock);
        }
        let Some(next_player_credits) = self.player_credits.checked_sub(price) else {
            return reject(CommandRejection::InsufficientCredits);
        };
        let Some(next_merchant_credits) = previous_merchant.credits.checked_add(price) else {
            return reject(CommandRejection::MerchantUnavailable);
        };
        let mut next_inventory = previous_inventory.clone();
        if next_inventory.add(item.clone(), 1, maximum_stack).is_err() {
            return reject(CommandRejection::InventoryCannotFitItem);
        }
        let mut next_merchant = previous_merchant.clone();
        next_merchant.offers[index].stock -= 1;
        next_merchant.credits = next_merchant_credits;

        *self.active.player_inventory_mut() = next_inventory;
        self.player_credits = next_player_credits;
        self.merchants.insert(zone.clone(), next_merchant);
        let outcome = self.active.process_player_command(command);
        if outcome == CommandOutcome::Applied {
            self.active.events.push(GameEvent::ItemBought {
                merchant: provider,
                definition: item,
                price,
                player_credits: next_player_credits,
            });
        } else {
            *self.active.player_inventory_mut() = previous_inventory;
            self.player_credits = previous_player_credits;
            self.merchants.insert(zone, previous_merchant);
        }
        outcome
    }

    fn sell_to_merchant(
        &mut self,
        provider: crate::entity::EntityId,
        instance: ItemInstanceId,
        command: GameCommand,
    ) -> CommandOutcome {
        let reject = |reason| CommandOutcome::Rejected(reason);
        let zone = match self.validate_merchant_access(provider) {
            Ok(zone) => zone,
            Err(reason) => return reject(reason),
        };
        if self.active.player_equipment().slot_of(instance).is_some() {
            return reject(CommandRejection::EquippedItemCannotBeSold);
        }
        let previous_inventory = self.active.player_inventory().clone();
        let previous_player_credits = self.player_credits;
        let Some(entry) = previous_inventory.get(instance) else {
            return reject(CommandRejection::UnknownInventoryItem(instance));
        };
        let item = entry.item().clone();
        let magic_modifiers = entry.magic_modifiers();
        let previous_merchant = self
            .merchants
            .get(&zone)
            .expect("merchant access validated")
            .clone();
        let standard = previous_merchant
            .offers
            .iter()
            .find(|offer| offer.item == item)
            .map(|offer| (offer.sell_price, offer.buy_price, offer.maximum_stack));
        let gamble =
            previous_merchant
                .gambles
                .iter()
                .find(|gamble| gamble.item == item)
                .and_then(|gamble| {
                    self.active.rules().items.get(&item).map(|definition| {
                        (gamble.price / 2, gamble.price, definition.maximum_stack())
                    })
                });
        let Some((price, resale_price, maximum_stack)) = standard.or(gamble) else {
            return reject(CommandRejection::MerchantOfferUnavailable);
        };
        let Some(next_merchant_credits) = previous_merchant.credits.checked_sub(price) else {
            return reject(CommandRejection::MerchantInsufficientCredits);
        };
        let Some(next_player_credits) = self.player_credits.checked_add(price) else {
            return reject(CommandRejection::MerchantUnavailable);
        };
        let Some(next_listing) = previous_merchant.next_listing.checked_add(1) else {
            return reject(CommandRejection::MerchantOfferUnavailable);
        };
        let mut next_inventory = previous_inventory.clone();
        if next_inventory.remove(instance, 1).is_err() {
            return reject(CommandRejection::UnknownInventoryItem(instance));
        }
        let mut next_merchant = previous_merchant.clone();
        next_merchant.resale.push(MerchantResaleState {
            listing: previous_merchant.next_listing,
            item: item.clone(),
            price: resale_price,
            maximum_stack,
            magic_modifiers,
        });
        next_merchant.next_listing = next_listing;
        next_merchant.credits = next_merchant_credits;

        *self.active.player_inventory_mut() = next_inventory;
        self.player_credits = next_player_credits;
        self.merchants.insert(zone.clone(), next_merchant);
        let outcome = self.active.process_player_command(command);
        if outcome == CommandOutcome::Applied {
            self.active.events.push(GameEvent::ItemSold {
                merchant: provider,
                definition: item,
                price,
                player_credits: next_player_credits,
            });
        } else {
            *self.active.player_inventory_mut() = previous_inventory;
            self.player_credits = previous_player_credits;
            self.merchants.insert(zone, previous_merchant);
        }
        outcome
    }

    fn buy_resale_from_merchant(
        &mut self,
        provider: crate::entity::EntityId,
        listing: u64,
        command: GameCommand,
    ) -> CommandOutcome {
        let reject = |reason| CommandOutcome::Rejected(reason);
        let zone = match self.validate_merchant_access(provider) {
            Ok(zone) => zone,
            Err(reason) => return reject(reason),
        };
        let previous_inventory = self.active.player_inventory().clone();
        let previous_player_credits = self.player_credits;
        let previous_merchant = self
            .merchants
            .get(&zone)
            .expect("merchant access validated")
            .clone();
        let Some(index) = previous_merchant
            .resale
            .iter()
            .position(|entry| entry.listing == listing)
        else {
            return reject(CommandRejection::MerchantOfferUnavailable);
        };
        let entry = previous_merchant.resale[index].clone();
        let Some(next_player_credits) = self.player_credits.checked_sub(entry.price) else {
            return reject(CommandRejection::InsufficientCredits);
        };
        let Some(next_merchant_credits) = previous_merchant.credits.checked_add(entry.price) else {
            return reject(CommandRejection::MerchantUnavailable);
        };
        let mut next_inventory = previous_inventory.clone();
        let added = if let Some(modifiers) = entry.magic_modifiers {
            next_inventory
                .add_magic(entry.item.clone(), None, modifiers)
                .map(|_| ())
        } else {
            next_inventory
                .add(entry.item.clone(), 1, entry.maximum_stack)
                .map(|_| ())
        };
        if added.is_err() {
            return reject(CommandRejection::InventoryCannotFitItem);
        }
        let mut next_merchant = previous_merchant.clone();
        next_merchant.resale.remove(index);
        next_merchant.credits = next_merchant_credits;
        *self.active.player_inventory_mut() = next_inventory;
        self.player_credits = next_player_credits;
        self.merchants.insert(zone.clone(), next_merchant);
        let outcome = self.active.process_player_command(command);
        if outcome == CommandOutcome::Applied {
            self.active.events.push(GameEvent::ItemBought {
                merchant: provider,
                definition: entry.item,
                price: entry.price,
                player_credits: next_player_credits,
            });
        } else {
            *self.active.player_inventory_mut() = previous_inventory;
            self.player_credits = previous_player_credits;
            self.merchants.insert(zone, previous_merchant);
        }
        outcome
    }

    fn gamble_with_merchant(
        &mut self,
        provider: crate::entity::EntityId,
        item: ItemId,
        command: GameCommand,
    ) -> CommandOutcome {
        let reject = |reason| CommandOutcome::Rejected(reason);
        let zone = match self.validate_merchant_access(provider) {
            Ok(zone) => zone,
            Err(reason) => return reject(reason),
        };
        let previous_inventory = self.active.player_inventory().clone();
        let previous_player_credits = self.player_credits;
        let player_level = self.active.player_progression().level();
        let zone_depth = self.current_zone().map_or(0, |info| info.depth);
        let previous_merchant = self
            .merchants
            .get(&zone)
            .expect("merchant access validated")
            .clone();
        let Some(index) = previous_merchant
            .gambles
            .iter()
            .position(|gamble| gamble.item == item)
        else {
            return reject(CommandRejection::MerchantOfferUnavailable);
        };
        let gamble = &previous_merchant.gambles[index];
        if gamble.stock == 0 {
            return reject(CommandRejection::MerchantOutOfStock);
        }
        let price = gamble.price;
        let Some(next_player_credits) = self.player_credits.checked_sub(price) else {
            return reject(CommandRejection::InsufficientCredits);
        };
        let Some(next_merchant_credits) = previous_merchant.credits.checked_add(price) else {
            return reject(CommandRejection::MerchantUnavailable);
        };
        let mut next_merchant = previous_merchant.clone();
        let roll_profile = GambleRollProfile::for_progression(
            next_merchant.gamble_scaling,
            player_level,
            zone_depth,
        );
        let modifiers = roll_profile.roll(&mut next_merchant.gamble_rng);
        let mut next_inventory = previous_inventory.clone();
        if next_inventory
            .add_magic(item.clone(), None, modifiers)
            .is_err()
        {
            return reject(CommandRejection::InventoryCannotFitItem);
        }
        next_merchant.gambles[index].stock -= 1;
        next_merchant.credits = next_merchant_credits;
        *self.active.player_inventory_mut() = next_inventory;
        self.player_credits = next_player_credits;
        self.merchants.insert(zone.clone(), next_merchant);
        let outcome = self.active.process_player_command(command);
        if outcome == CommandOutcome::Applied {
            self.active.events.push(GameEvent::GambleResolved {
                merchant: provider,
                definition: item,
                price,
                player_credits: next_player_credits,
                modifiers,
            });
        } else {
            *self.active.player_inventory_mut() = previous_inventory;
            self.player_credits = previous_player_credits;
            self.merchants.insert(zone, previous_merchant);
        }
        outcome
    }

    fn interact_with_facility(&mut self, target: GridPos) -> Option<CommandOutcome> {
        let zone = self.current.as_ref()?.clone();
        let facility = self.facilities.get(&zone)?;
        let data_terminal_record = facility.data_terminal_record_at(target).cloned();
        let worker = self
            .active
            .actors
            .entity_at(target)
            .and_then(|entity| facility.worker_role(entity).map(|role| (entity, role)));
        if !facility.is_player_interactive_at(target) && worker.is_none() {
            return None;
        }
        let reject = |reason| Some(CommandOutcome::Rejected(reason));
        if self.active.status != RunStatus::Active {
            return reject(CommandRejection::RunEnded);
        }
        if self.active.phase != TurnPhase::AwaitingPlayer {
            return reject(CommandRejection::NotPlayersTurn);
        }
        let Some(origin) = self.active.player_position() else {
            return reject(CommandRejection::MissingPlayer);
        };
        if !origin.cardinal_neighbors().contains(&target)
            || !self.active.player_visibility.is_visible(target)
        {
            return reject(CommandRejection::InteractionOutOfReach);
        }
        let player = self.active.player;
        if let Some((_worker, role)) = worker {
            if role != WorkerRole::Technician {
                return reject(CommandRejection::NothingToInteract);
            }
            let facility = self
                .facilities
                .get_mut(&zone)
                .expect("facility checked before interaction");
            return match facility.deposit_player_material(self.active.player_inventory_mut()) {
                Ok(Some(deposit)) => {
                    self.active.events.push(GameEvent::Facility(
                        FacilityEvent::PlayerMaterialDeposited {
                            player,
                            at: target,
                            item: deposit.item,
                            quantity: deposit.quantity,
                        },
                    ));
                    self.active.complete_turn();
                    Some(CommandOutcome::Applied)
                }
                Ok(None) => reject(CommandRejection::NoMaterialForDepot),
                Err(_) => reject(CommandRejection::FacilityUnavailable),
            };
        }
        if let Some(record) = data_terminal_record {
            let first_discovery = !self.discovered_data_terminal_records().contains(&record);
            let facility = self
                .facilities
                .get_mut(&zone)
                .expect("facility checked before interaction");
            let Some(access) = facility.access_data_terminal(target) else {
                return reject(CommandRejection::FacilityUnavailable);
            };
            debug_assert_eq!(access.record, record);
            self.active
                .events
                .push(GameEvent::Facility(FacilityEvent::DataTerminalAccessed {
                    player,
                    installation: access.installation,
                    at: target,
                    record: access.record,
                    first_access: first_discovery,
                }));
            self.record_data_record_quest_progress(&record);
            self.active.complete_turn();
            return Some(CommandOutcome::Applied);
        }
        let facility = self
            .facilities
            .get_mut(&zone)
            .expect("facility checked before interaction");
        match facility.deposit_player_material(self.active.player_inventory_mut()) {
            Ok(Some(deposit)) => {
                self.active.events.push(GameEvent::Facility(
                    FacilityEvent::PlayerMaterialDeposited {
                        player,
                        at: target,
                        item: deposit.item,
                        quantity: deposit.quantity,
                    },
                ));
                self.active.complete_turn();
                Some(CommandOutcome::Applied)
            }
            Ok(None) => reject(CommandRejection::NoMaterialForDepot),
            Err(_) => reject(CommandRejection::FacilityUnavailable),
        }
    }

    fn record_exploration_quest_progress(&mut self, destination: &ContentId) {
        let mut progressed = Vec::new();
        for quests in self.quests.values_mut() {
            for quest in quests {
                if !quest.accepted || quest.completed {
                    continue;
                }
                let QuestDefinition::ExploreZones(definition) = &quest.definition else {
                    continue;
                };
                let QuestProgress::ExploreZones {
                    baseline,
                    discovered,
                } = &mut quest.progress
                else {
                    continue;
                };
                if discovered.len() >= usize::from(definition.required_zones)
                    || baseline.contains(destination)
                    || !definition.qualifying_records.is_empty()
                    || !discovered.insert(destination.clone())
                {
                    continue;
                }
                progressed.push((
                    definition.id.clone(),
                    discovered.len().min(usize::from(u16::MAX)) as u16,
                    definition.required_zones,
                ));
            }
        }
        self.active
            .events
            .extend(progressed.into_iter().map(|(quest, current, required)| {
                GameEvent::QuestProgressed {
                    quest,
                    current,
                    required,
                }
            }));
    }

    fn record_data_record_quest_progress(&mut self, record: &ContentId) {
        let mut progressed = Vec::new();
        let current_zone = self.current.as_ref();
        for quests in self.quests.values_mut() {
            for quest in quests {
                if !quest.accepted || quest.completed {
                    continue;
                }
                match (&quest.definition, &mut quest.progress) {
                    (
                        QuestDefinition::AccessDataRecord(definition),
                        QuestProgress::AccessDataRecord { accessed },
                    ) if !*accessed && definition.record == *record => {
                        *accessed = true;
                        progressed.push((definition.id.clone(), 1, 1));
                    }
                    (
                        QuestDefinition::ExploreZones(definition),
                        QuestProgress::ExploreZones {
                            baseline,
                            discovered,
                        },
                    ) if definition.qualifying_records.contains(record)
                        && current_zone.is_some_and(|zone| !baseline.contains(zone))
                        && discovered.len() < usize::from(definition.required_zones) =>
                    {
                        let zone = current_zone.expect("checked above");
                        if discovered.insert(zone.clone()) {
                            progressed.push((
                                definition.id.clone(),
                                discovered.len().min(usize::from(u16::MAX)) as u16,
                                definition.required_zones,
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
        self.active
            .events
            .extend(progressed.into_iter().map(|(quest, current, required)| {
                GameEvent::QuestProgressed {
                    quest,
                    current,
                    required,
                }
            }));
    }

    fn record_defeat_quest_progress(&mut self, tags: &[ContentId]) {
        let mut progressed = Vec::new();
        for quests in self.quests.values_mut() {
            for quest in quests {
                if !quest.accepted || quest.completed {
                    continue;
                }
                let QuestDefinition::DefeatTargets(definition) = &quest.definition else {
                    continue;
                };
                let QuestProgress::DefeatTargets { defeated_quantity } = &mut quest.progress else {
                    continue;
                };
                if *defeated_quantity >= definition.required_quantity
                    || !tags.contains(&definition.target_tag)
                {
                    continue;
                }
                *defeated_quantity += 1;
                progressed.push((
                    definition.id.clone(),
                    *defeated_quantity,
                    definition.required_quantity,
                ));
            }
        }
        self.active
            .events
            .extend(progressed.into_iter().map(|(quest, current, required)| {
                GameEvent::QuestProgressed {
                    quest,
                    current,
                    required,
                }
            }));
    }

    fn travel(&mut self, at: GridPos, link: ZoneLink) -> CommandOutcome {
        let reject = |reason| CommandOutcome::Rejected(reason);
        if self.active.status != RunStatus::Active {
            return reject(CommandRejection::RunEnded);
        }
        if self.active.phase != TurnPhase::AwaitingPlayer {
            return reject(CommandRejection::NotPlayersTurn);
        }
        let Some(origin) = self.active.player_position() else {
            return reject(CommandRejection::MissingPlayer);
        };
        if (origin != at && !origin.cardinal_neighbors().contains(&at))
            || !self.active.player_visibility.is_visible(at)
        {
            return reject(CommandRejection::InteractionOutOfReach);
        }
        if !self.active.map.is_walkable(at) {
            return reject(CommandRejection::PassageUnavailable);
        }
        let Some(arrival) = link.arrival else {
            return reject(CommandRejection::PassageUnavailable);
        };
        if let Some(blueprint) = self.pending.get(&link.destination) {
            let Ok(mut zone) = self.prepare_zone(blueprint) else {
                return reject(CommandRejection::PassageUnavailable);
            };
            let pending_facility = self.pending_facilities.get(&link.destination).cloned();
            let facility = if let Some(blueprint) = pending_facility {
                let Ok(facility) =
                    FacilityState::instantiate(blueprint, &mut zone.map, &mut zone.actors)
                else {
                    return reject(CommandRejection::PassageUnavailable);
                };
                Some(facility)
            } else {
                None
            };
            if zone.actors.entity_at(arrival).is_some() {
                return reject(CommandRejection::PassageObstructed);
            }
            self.pending.remove(&link.destination);
            if let Some(facility) = facility {
                self.pending_facilities.remove(&link.destination);
                self.facilities.insert(link.destination.clone(), facility);
            }
            self.inactive.insert(link.destination.clone(), zone);
        }
        let Some(zone) = self.inactive.get(&link.destination) else {
            return reject(CommandRejection::PassageUnavailable);
        };
        if !zone.map.is_walkable(arrival) {
            return reject(CommandRejection::PassageUnavailable);
        }
        if zone.actors.entity_at(arrival).is_some() {
            return reject(CommandRejection::PassageObstructed);
        }
        let escorting_drones = self
            .active
            .actors
            .iter()
            .filter_map(|(entity, actor)| {
                let drone = actor.drone()?;
                let controller = match drone.order() {
                    DroneOrder::Escort { controller, .. } => controller,
                    DroneOrder::Companion { controller, .. }
                        if self.active.rules.player_drone_link_recovery =>
                    {
                        controller
                    }
                    _ => return None,
                };
                (*controller == self.active.player
                    && drone.controller() == self.active.player
                    && drone
                        .profile()
                        .link_reaches(&self.active.map, origin, actor.position(), 0))
                .then_some(entity)
            })
            .collect::<Vec<_>>();
        let mut occupied_arrivals = zone
            .actors
            .iter()
            .map(|(_, actor)| actor.position())
            .collect::<BTreeSet<_>>();
        occupied_arrivals.insert(arrival);
        let mut escort_arrivals = Vec::with_capacity(escorting_drones.len());
        for _ in &escorting_drones {
            let mut candidates = (0..zone.map.height())
                .flat_map(|y| (0..zone.map.width()).map(move |x| GridPos::new(x as i32, y as i32)))
                .filter(|position| {
                    zone.map.is_walkable(*position)
                        && !occupied_arrivals.contains(position)
                        && !zone.map.is_protected(*position)
                })
                .collect::<Vec<_>>();
            candidates.sort_by_key(|position| {
                (
                    position.x.abs_diff(arrival.x) + position.y.abs_diff(arrival.y),
                    position.y,
                    position.x,
                )
            });
            let Some(destination) = candidates.first().copied() else {
                return reject(CommandRejection::PassageObstructed);
            };
            occupied_arrivals.insert(destination);
            escort_arrivals.push(destination);
        }
        let mut next = self
            .inactive
            .remove(&link.destination)
            .expect("checked zone");
        let from = self
            .current
            .replace(link.destination.clone())
            .expect("initialized world");
        let player = self
            .active
            .actors
            .remove(self.active.player)
            .expect("checked player");
        let escorts = escorting_drones
            .iter()
            .map(|entity| {
                let actor = self
                    .active
                    .actors
                    .remove(*entity)
                    .expect("escorting drone was preflighted");
                (*entity, actor.position(), actor)
            })
            .collect::<Vec<_>>();
        next.actors.synchronize_ids(&self.active.actors);
        next.ground.synchronize_ids(&self.active.ground_items);
        self.active.swap_zone(&mut next);
        next.visibility.clear_visible();
        self.active
            .actors
            .insert_existing(self.active.player, player);
        self.active
            .actors
            .move_to(self.active.player, arrival)
            .expect("inserted player");
        for ((entity, origin, actor), destination) in escorts.into_iter().zip(escort_arrivals) {
            self.active.actors.insert_existing(entity, actor);
            self.active
                .actors
                .move_to(entity, destination)
                .expect("inserted escorting drone");
            if let Some(drone) = self
                .active
                .actors
                .get_mut(entity)
                .and_then(Actor::drone_mut)
            {
                drone.confirm_position(destination, self.active.turn);
            }
            self.active.events.push(GameEvent::EntityMoved {
                entity,
                from: origin,
                to: destination,
            });
            self.active
                .resolve_status_trigger_for(entity, StatusTrigger::Movement);
        }
        self.inactive.insert(from.clone(), next);
        self.active.player_visibility.recompute(
            &self.active.map,
            arrival,
            self.active.rules.player_field_of_view,
        );
        self.active.events.push(GameEvent::ZoneChanged {
            from,
            to: link.destination.clone(),
            arrival,
        });
        self.active.events.push(GameEvent::VisibilityUpdated {
            observer: self.active.player,
            origin: arrival,
        });
        self.record_exploration_quest_progress(&link.destination);
        self.active.complete_turn();
        CommandOutcome::Applied
    }
}

fn validate_facility_reinforcement_sources(
    facility: &FacilityBlueprint,
    threat_sources: &[(GridPos, bool)],
) -> Result<(), String> {
    for response in facility
        .installations
        .iter()
        .filter_map(|installation| installation.security_alarm_profile.as_ref())
        .flat_map(|profile| profile.responses())
    {
        let (source, requires_investigation) = match response {
            SecurityAlarmResponse::CallReinforcements { source, .. } => (source, false),
            SecurityAlarmResponse::CallInvestigatingReinforcements { source, .. } => (source, true),
            SecurityAlarmResponse::LockDoors { .. } => continue,
        };
        let Some((_, supports_investigation)) = threat_sources
            .iter()
            .find(|(position, _)| position == source)
        else {
            return Err(format!(
                "Facility alarm references unknown threat source at {}, {}",
                source.x, source.y
            ));
        };
        if requires_investigation && !supports_investigation {
            return Err(format!(
                "Facility alarm investigation source at {}, {} requires mobile AI with a pursuit lifecycle",
                source.x, source.y
            ));
        }
    }
    Ok(())
}

fn threat_actor_can_investigate(actor: &Actor) -> bool {
    actor.ai().is_some_and(|profile| {
        matches!(
            profile.behavior,
            AiBehavior::Hunter | AiBehavior::Skirmisher
        ) && profile.pursuit_lifecycle().is_some()
    })
}

fn facility_event_is_visible(
    event: &FacilityEvent,
    facility: &FacilityState,
    actors: &ActorRegistry,
    visibility: &VisibilityState,
) -> bool {
    let worker_visible = |worker| {
        actors
            .get(worker)
            .is_some_and(|actor| visibility.is_visible(actor.position()))
    };
    let order_visible = |order: &ContentId| {
        facility
            .repair_target(order)
            .is_some_and(|installation| visibility.is_visible(installation.position()))
    };
    match event {
        FacilityEvent::WorkerMoved { from, to, .. } => {
            visibility.is_visible(*from) || visibility.is_visible(*to)
        }
        FacilityEvent::DoorOpened { worker, at } => {
            visibility.is_visible(*at) || worker_visible(*worker)
        }
        FacilityEvent::PlayerMaterialDeposited { player, at, .. } => {
            visibility.is_visible(*at) || worker_visible(*player)
        }
        FacilityEvent::DataTerminalAccessed { player, at, .. } => {
            visibility.is_visible(*at) || worker_visible(*player)
        }
        FacilityEvent::MaterialCollected { worker, .. }
        | FacilityEvent::MaterialDelivered { worker, .. }
        | FacilityEvent::RepairAssigned { worker, .. }
        | FacilityEvent::RepairStarted { worker, .. }
        | FacilityEvent::ReportedIncidentInvestigationAssigned { worker, .. }
        | FacilityEvent::ReportedIncidentInvestigationEnded { worker, .. } => {
            worker_visible(*worker)
        }
        FacilityEvent::InstalledPropertyReportReceived {
            source,
            installation,
            ..
        } => {
            worker_visible(*source)
                || facility
                    .installation(installation)
                    .is_some_and(|state| visibility.is_visible(state.position()))
        }
        FacilityEvent::InstallationRepaired { installation, .. } => facility
            .installation(installation)
            .is_some_and(|state| visibility.is_visible(state.position())),
        FacilityEvent::SecurityAlarmRaised { installation, .. } => facility
            .installation(installation)
            .is_some_and(|state| visibility.is_visible(state.position())),
        FacilityEvent::ReinforcementsRequested {
            installation,
            source,
            ..
        }
        | FacilityEvent::InvestigatingReinforcementsRequested {
            installation,
            source,
            ..
        }
        | FacilityEvent::ReinforcementsUnavailable {
            installation,
            source,
            ..
        } => {
            visibility.is_visible(*source)
                || facility
                    .installation(installation)
                    .is_some_and(|state| visibility.is_visible(state.position()))
        }
        FacilityEvent::DoorLockdownStarted {
            installation, door, ..
        }
        | FacilityEvent::DoorLockdownPrevented {
            installation, door, ..
        }
        | FacilityEvent::DoorLockdownEnded {
            installation, door, ..
        } => {
            visibility.is_visible(*door)
                || facility
                    .installation(installation)
                    .is_some_and(|state| visibility.is_visible(state.position()))
        }
        FacilityEvent::MaterialSpilled { at, .. } => visibility.is_visible(*at),
        FacilityEvent::WorkInterrupted { order } => order_visible(order),
        FacilityEvent::SimulationFault(_) => false,
    }
}

fn tick_clinic_routine(clinic: &mut ClinicState, map: &Map, actors: &mut ActorRegistry) {
    let target = clinic.routine_target();
    if tick_actor_towards_anchor(
        clinic.provider,
        target,
        clinic.maximum_path_search,
        map,
        actors,
    ) {
        clinic.elapse_at_target();
    }
}

fn tick_resident_routine(resident: &mut ResidentState, map: &Map, actors: &mut ActorRegistry) {
    let target = resident.routine_target();
    if tick_actor_towards_anchor(
        resident.provider,
        target,
        resident.maximum_path_search,
        map,
        actors,
    ) {
        resident.elapse_at_target();
    }
}

/// Moves one actor by at most one cardinal cell and reports whether it began
/// the tick on its anchor. Service NPCs and ambient residents share this
/// bounded movement rule so they cannot drift into separate AI systems.
fn tick_actor_towards_anchor(
    provider: crate::entity::EntityId,
    target: GridPos,
    maximum_path_search: usize,
    map: &Map,
    actors: &mut ActorRegistry,
) -> bool {
    let Some(origin) = actors.get(provider).map(Actor::position) else {
        return false;
    };
    if origin == target {
        return true;
    }
    if actors.entity_at(target).is_some() {
        return false;
    }
    let Some(path) = find_path(map, origin, target, maximum_path_search, |position| {
        actors.entity_at(position).is_none()
    }) else {
        return false;
    };
    let Some(destination) = path.get(1).copied() else {
        return false;
    };
    if actors.entity_at(destination).is_none() {
        let _ = actors.move_to(provider, destination);
    }
    false
}

fn resolve_reinforcement_requests(game: &mut GameState, events: &mut [FacilityEvent]) {
    for event in events {
        let (installation, source, result) = match event {
            FacilityEvent::ReinforcementsRequested {
                installation,
                source,
                delay_turns,
            } => (
                installation.clone(),
                *source,
                game.request_threat_reinforcement(*source, *delay_turns),
            ),
            FacilityEvent::InvestigatingReinforcementsRequested {
                installation,
                source,
                incident,
                delay_turns,
            } => (
                installation.clone(),
                *source,
                game.request_investigating_threat_reinforcement(*source, *incident, *delay_turns),
            ),
            _ => continue,
        };
        let failure = match result {
            Ok(()) => continue,
            Err(ThreatReinforcementRequestError::SourceInactive) => {
                Some(ReinforcementRequestFailure::SourceInactive)
            }
            Err(ThreatReinforcementRequestError::QuotaExhausted) => {
                Some(ReinforcementRequestFailure::QuotaExhausted)
            }
            Err(ThreatReinforcementRequestError::UnknownSource) => None,
        };
        *event = if let Some(reason) = failure {
            FacilityEvent::ReinforcementsUnavailable {
                installation,
                source,
                reason,
            }
        } else {
            FacilityEvent::SimulationFault(format!(
                "security alarm references unknown threat source at {}, {}",
                source.x, source.y
            ))
        };
    }
}

impl GameState {
    fn swap_zone(&mut self, zone: &mut ZoneState) {
        std::mem::swap(&mut self.map, &mut zone.map);
        std::mem::swap(&mut self.actors, &mut zone.actors);
        std::mem::swap(&mut self.exit, &mut zone.exit);
        std::mem::swap(&mut self.rng, &mut zone.rng);
        std::mem::swap(&mut self.player_visibility, &mut zone.visibility);
        std::mem::swap(&mut self.ground_items, &mut zone.ground);
        std::mem::swap(&mut self.movement_traces, &mut zone.traces);
        std::mem::swap(&mut self.ground_effects, &mut zone.ground_effects);
        std::mem::swap(&mut self.explosive_devices, &mut zone.explosive_devices);
        std::mem::swap(&mut self.intrusion, &mut zone.intrusion);
        std::mem::swap(&mut self.electronic_warfare, &mut zone.electronic_warfare);
        std::mem::swap(&mut self.threat_sources, &mut zone.threat_sources);
    }

    fn tick_background(&mut self, zone: &mut ZoneState, previous_turn: u64) {
        let visible_events = std::mem::take(&mut self.events);
        let current_turn = self.turn;
        self.swap_zone(zone);
        self.turn = previous_turn;
        let actor_ids: Vec<_> = self.actors.iter().map(|(entity, _)| entity).collect();
        for entity in actor_ids {
            if let Some(actor) = self.actors.get_mut(entity) {
                actor.clear_elapsed_action_delay(self.turn);
            }
        }
        self.resolve_status_trigger(StatusTrigger::TurnStart);
        let actors: Vec<_> = self
            .actors
            .iter()
            .filter_map(|(id, actor)| {
                actor
                    .ai()
                    .filter(|ai| matches!(ai.behavior, AiBehavior::Hunter | AiBehavior::Skirmisher))
                    .map(|ai| (id, actor.position(), ai, actor.ai_home(), actor.ai_state()))
            })
            .collect();
        // No player position or remote target is available. Territorial actors
        // return home before resuming local patrols; sentries/idle actors stay
        // put. Nobody crosses portals.
        for (id, origin, profile, home, state) in actors {
            if self
                .actors
                .get(id)
                .is_some_and(|actor| actor.action_is_delayed(self.turn))
            {
                continue;
            }
            // Every off-screen actor still receives one normal opportunity.
            // Movement or local waiting is non-offensive and therefore spends
            // recovery, while the emitted transition remains hidden below.
            if self
                .actors
                .get(id)
                .is_some_and(|actor| actor.recovery_remaining().is_some())
            {
                self.advance_action_recovery(id);
            }
            if let Some(lifecycle) = profile.pursuit_lifecycle() {
                if let AiState::Responding {
                    remaining_turns,
                    incident,
                } = state
                {
                    let next_state = if remaining_turns <= 1 || origin == incident {
                        AiState::Returning
                    } else {
                        AiState::Responding {
                            remaining_turns: remaining_turns - 1,
                            incident,
                        }
                    };
                    if let Some(actor) = self.actors.get_mut(id) {
                        actor.set_ai_state(next_state);
                    }
                    if matches!(next_state, AiState::Returning) {
                        continue;
                    }
                    let occupied_positions = self
                        .actors
                        .iter()
                        .filter_map(|(other_id, actor)| {
                            (other_id != id).then_some(actor.position())
                        })
                        .collect();
                    if let AiAction::Move(direction) = decide_known_action(AiSituation {
                        map: &self.map,
                        actor_position: origin,
                        home_position: home,
                        target_position: incident,
                        occupied_positions: &occupied_positions,
                        profile,
                        preferred_attack: None,
                    }) {
                        let _ = self.move_ai_entity(id, direction);
                    }
                    continue;
                }
                let next_state = match state {
                    AiState::Pursuing { .. } | AiState::Searching { .. }
                        if home == Some(origin) =>
                    {
                        AiState::Cooldown {
                            remaining_turns: lifecycle.cooldown_turns(),
                        }
                    }
                    AiState::Pursuing { .. } | AiState::Searching { .. } => AiState::Returning,
                    AiState::Returning if home == Some(origin) => AiState::Cooldown {
                        remaining_turns: lifecycle.cooldown_turns(),
                    },
                    AiState::Cooldown { remaining_turns } if remaining_turns <= 1 => {
                        AiState::Unaware
                    }
                    AiState::Cooldown { remaining_turns } => AiState::Cooldown {
                        remaining_turns: remaining_turns - 1,
                    },
                    other => other,
                };
                if let Some(actor) = self.actors.get_mut(id) {
                    actor.set_ai_state(next_state);
                }
                if matches!(next_state, AiState::Cooldown { .. }) {
                    continue;
                }
            }
            if let Some(home) = home
                && origin != home
            {
                let path = find_path(
                    &self.map,
                    origin,
                    home,
                    profile.maximum_path_search,
                    |position| {
                        !self.map.is_protected(position)
                            && self.actors.entity_at(position).is_none()
                            && profile.maximum_pursuit_distance().is_none_or(|maximum| {
                                position.x.abs_diff(home.x).max(position.y.abs_diff(home.y))
                                    <= u32::from(maximum)
                            })
                    },
                );
                if let Some(next) = path.and_then(|path| path.get(1).copied())
                    && let Some(direction) =
                        Direction::from_delta(next.x - origin.x, next.y - origin.y)
                {
                    let _ = self.move_ai_entity(id, direction);
                }
                continue;
            }
            let choices: Vec<_> = [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .into_iter()
            .filter(|d| {
                let p = origin.step(*d);
                self.map.is_walkable(p)
                    && !self.map.is_protected(p)
                    && self.actors.entity_at(p).is_none()
                    && profile.maximum_pursuit_distance().is_none_or(|maximum| {
                        home.is_some_and(|home| {
                            p.x.abs_diff(home.x).max(p.y.abs_diff(home.y)) <= u32::from(maximum)
                        })
                    })
            })
            .collect();
            if let Some(index) = self.rng.usize_inclusive(0, choices.len())
                && let Some(direction) = choices.get(index)
            {
                let _ = self.move_ai_entity(id, *direction);
            }
        }
        self.resolve_status_trigger(StatusTrigger::TurnEnd);
        self.resolve_explosive_devices();
        self.resolve_ground_effects();
        self.elapse_status_durations();
        self.advance_technique_cooldowns();
        self.resolve_threat_sources();
        self.advance_intrusion_state();
        self.advance_electronic_warfare_state();
        self.turn = current_turn;
        self.movement_traces
            .prune(current_turn, self.rules.movement_traces);
        // Off-screen movement, death, positions and status events are not player
        // knowledge. Legitimate player XP from an ongoing effect is still global.
        let rewards: Vec<_> = self
            .events
            .drain(..)
            .filter(|event| {
                matches!(
                    event,
                    GameEvent::ExperienceAwarded { .. } | GameEvent::LevelGained { .. }
                )
            })
            .collect();
        // Off-screen reinforcements still consume IDs from the one global
        // namespace. Synchronize both registries before restoring the active
        // zone so a later spawn can never reuse an unseen entity ID.
        self.actors.synchronize_ids(&zone.actors);
        zone.actors.synchronize_ids(&self.actors);
        self.swap_zone(zone);
        self.events = visible_events;
        self.events.extend(rewards);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::AiProfile;
    use crate::combat::{DamagePacket, DamageType};
    use crate::companion::CompanionBehavior;
    use crate::content::{ClinicDefinition, MerchantGambleDefinition, MerchantOfferDefinition};
    use crate::drone::{DroneCapabilities, DroneProfile, DroneState};
    use crate::effects::GroundEffectSpec;
    use crate::facility::{
        FacilityBlueprint, InstallationBlueprint, InstallationCapability, RepairOrderBlueprint,
        RepairStatus, SecurityAlarmProfile, SecurityAlarmResponse, WorkerBlueprint,
        WorkerInstalledPropertyReportBlueprint, WorkerPropertyReportBlueprint,
        WorkerReportedIncidentResponseBlueprint, WorkerRole,
    };
    use crate::game::GameRules;
    use crate::item::{EquipmentProfile, ItemDefinition, ItemEffect, ItemKind};
    use crate::social::{LocalAlertProfile, PropertyReportChannel, SocialGroupId, WitnessProfile};
    use crate::status::{
        StatusDefinition, StatusEffectPrimitive, StatusHook, StatusModifier, StatusStacking,
    };
    use crate::time::TimeUnits;
    use crate::world::{DistanceMetric, DoorState, Terrain};

    fn id(name: &str) -> ContentId {
        format!("test:{name}").parse().unwrap()
    }
    fn info(name: &str, depth: u16) -> ZoneInfo {
        ZoneInfo {
            id: id(name),
            name: name.into(),
            kind: id("industrial"),
            depth,
        }
    }
    fn map() -> Map {
        Map::from_ascii(
            "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
        )
        .unwrap()
    }
    fn world() -> WorldState {
        let mut rules = GameRules::default();
        rules
            .items
            .register(
                ItemDefinition::new(
                    id("repair"),
                    "repair".into(),
                    "repair".into(),
                    9,
                    ItemKind::Consumable,
                    None,
                    vec![ItemEffect::RestoreIntegrity { amount: 1 }],
                )
                .unwrap(),
            )
            .unwrap();
        let mut game = GameState::new_with_rules(map(), GridPos::new(1, 1), 42, rules).unwrap();
        game.map
            .set_terrain(GridPos::new(1, 2), Terrain::Door(DoorState::Closed))
            .unwrap();
        game.spawn_actor(
            Actor::new(GridPos::new(7, 5), 10)
                .unwrap()
                .with_ai(AiProfile::hunter(0, 0)),
        )
        .unwrap();
        let mut world = WorldState::single(game);
        world.enable(info("a", 0)).unwrap();
        world
            .add_zone(ZoneBlueprint {
                info: info("b", 1),
                map: map(),
                entrance: GridPos::new(1, 1),
                seed: 888,
                actors: vec![
                    Actor::new(GridPos::new(7, 5), 10)
                        .unwrap()
                        .with_ai(AiProfile::idle()),
                ],
                loot: vec![(GridPos::new(2, 1), id("repair"), 1).into()],
                threat_sources: vec![],
            })
            .unwrap();
        world
            .connect(id("a"), GridPos::new(2, 1), id("b"), GridPos::new(1, 1))
            .unwrap();
        world.drain_events();
        world
    }

    fn commerce_world() -> (WorldState, crate::entity::EntityId, ItemId) {
        let armor = id("market_armor");
        let mut rules = GameRules::default();
        rules
            .items
            .register(
                ItemDefinition::new(
                    armor.clone(),
                    "armor.name".into(),
                    "armor.description".into(),
                    1,
                    ItemKind::Armor,
                    Some(EquipmentProfile::new(id("body"), 1).unwrap()),
                    vec![],
                )
                .unwrap()
                .with_mass_grams(10_000)
                .unwrap(),
            )
            .unwrap();
        let mut game = GameState::new_with_rules(map(), GridPos::new(2, 2), 42, rules).unwrap();
        let provider = game
            .spawn_actor(Actor::new(GridPos::new(3, 2), 10).unwrap())
            .unwrap();
        let mut world = WorldState::single(game);
        world.enable(info("market", 0)).unwrap();
        let merchant = MerchantDefinition::new(
            GridPos::new(3, 2),
            10,
            500,
            vec![MerchantOfferDefinition {
                item: armor.clone(),
                initial_stock: 2,
                buy_price: 30,
                sell_price: 15,
                minimum_depth: 0,
                maximum_depth: None,
            }],
            vec![MerchantGambleDefinition {
                item: armor.clone(),
                initial_stock: 2,
                price: 60,
            }],
            GambleScalingDefinition::new(3, 1, 12).unwrap(),
        )
        .unwrap();
        world
            .register_merchant(id("market"), provider, 200, merchant, 7)
            .unwrap();
        world.drain_events();
        (world, provider, armor)
    }

    fn clinic_world(player_credits: u32) -> (WorldState, crate::entity::EntityId) {
        let game =
            GameState::new_with_rules(map(), GridPos::new(2, 2), 42, GameRules::default()).unwrap();
        let mut world = WorldState::single(game);
        world.enable(info("clinic", 0)).unwrap();
        let provider = world
            .spawn_actor(Actor::new(GridPos::new(3, 2), 10).unwrap())
            .unwrap();
        let clinic = ClinicDefinition::new(
            GridPos::new(3, 2),
            GridPos::new(5, 2),
            10,
            20,
            4,
            3,
            2,
            2,
            128,
        )
        .unwrap();
        world
            .register_clinic(id("clinic"), provider, player_credits, clinic)
            .unwrap();
        world.drain_events();
        (world, provider)
    }

    fn resident_world() -> (WorldState, crate::entity::EntityId) {
        let game =
            GameState::new_with_rules(map(), GridPos::new(2, 2), 42, GameRules::default()).unwrap();
        let mut world = WorldState::single(game);
        world.enable(info("resident", 0)).unwrap();
        let provider = world
            .spawn_actor(Actor::new(GridPos::new(3, 2), 10).unwrap())
            .unwrap();
        let resident =
            ResidentDefinition::new(GridPos::new(3, 2), GridPos::new(5, 2), 10, 2, 2, 128).unwrap();
        world
            .register_resident(id("resident"), provider, resident)
            .unwrap();
        world.drain_events();
        (world, provider)
    }

    fn quest_world() -> (WorldState, crate::entity::EntityId, QuestId, ItemId) {
        let required_item = id("quest_material");
        let mut rules = GameRules::default();
        rules
            .items
            .register(
                ItemDefinition::new(
                    required_item.clone(),
                    "quest_material.name".into(),
                    "quest_material.description".into(),
                    9,
                    ItemKind::Consumable,
                    None,
                    vec![ItemEffect::RestoreIntegrity { amount: 1 }],
                )
                .unwrap(),
            )
            .unwrap();
        let game = GameState::new_with_rules(map(), GridPos::new(2, 2), 42, rules).unwrap();
        let mut world = WorldState::single(game);
        let zone = id("quest_city");
        world.enable(info("quest_city", 0)).unwrap();
        let provider = world
            .spawn_actor(Actor::new(GridPos::new(3, 2), 10).unwrap())
            .unwrap();
        let resident =
            ResidentDefinition::new(GridPos::new(3, 2), GridPos::new(5, 2), 10, 4, 4, 128).unwrap();
        world
            .register_resident(zone.clone(), provider, resident)
            .unwrap();
        let quest = id("delivery");
        world
            .register_delivery_quest(
                zone,
                provider,
                DeliveryQuestDefinition::new(
                    quest.clone(),
                    "quest.delivery.title".into(),
                    "quest.delivery.summary".into(),
                    required_item.clone(),
                    2,
                    45,
                )
                .unwrap(),
            )
            .unwrap();
        world.drain_events();
        (world, provider, quest, required_item)
    }

    fn world_effect_quest_world() -> (
        WorldState,
        crate::entity::EntityId,
        QuestId,
        ItemId,
        GridPos,
    ) {
        let required_item = id("access_report");
        let mut rules = GameRules::default();
        rules
            .items
            .register(
                ItemDefinition::new(
                    required_item.clone(),
                    "access_report.name".into(),
                    "access_report.description".into(),
                    1,
                    ItemKind::Consumable,
                    None,
                    vec![ItemEffect::RestoreIntegrity { amount: 1 }],
                )
                .unwrap(),
            )
            .unwrap();
        let door = GridPos::new(4, 3);
        let mut game = GameState::new_with_rules(map(), GridPos::new(2, 2), 42, rules).unwrap();
        game.map
            .set_terrain(door, Terrain::Door(DoorState::Locked))
            .unwrap();
        let provider = game
            .spawn_actor(Actor::new(GridPos::new(3, 2), 10).unwrap())
            .unwrap();
        let mut world = WorldState::single(game);
        let zone = id("world_effect_city");
        world.enable(info("world_effect_city", 0)).unwrap();
        let quest = id("unlock_access");
        let authored = HubQuestDefinition::new(
            crate::content::HubQuestProviderDefinition::existing(GridPos::new(3, 2)).unwrap(),
            DeliveryQuestDefinition::new(
                quest.clone(),
                "quest.unlock_access.title".into(),
                "quest.unlock_access.summary".into(),
                required_item.clone(),
                1,
                12,
            )
            .unwrap()
            .into(),
        )
        .with_world_effects(vec![
            QuestWorldEffectDefinition::unlock_door(
                door,
                "world_effect.unlock_access.summary".into(),
            )
            .unwrap(),
        ])
        .unwrap();
        world
            .register_authored_quest(zone, provider, authored)
            .unwrap();
        world.drain_events();
        (world, provider, quest, required_item, door)
    }

    fn authorization_quest_world() -> (
        WorldState,
        crate::entity::EntityId,
        crate::entity::EntityId,
        QuestId,
        ItemId,
        SocialGroupId,
    ) {
        let required_item = id("salvage_material");
        let mut rules = GameRules::default();
        rules
            .items
            .register(
                ItemDefinition::new(
                    required_item.clone(),
                    "salvage_material.name".into(),
                    "salvage_material.description".into(),
                    4,
                    ItemKind::Material,
                    None,
                    vec![],
                )
                .unwrap(),
            )
            .unwrap();
        let owner: SocialGroupId = "test:salvage_collective".parse().unwrap();
        let mut game = GameState::new_with_rules(map(), GridPos::new(2, 2), 42, rules).unwrap();
        let provider = game
            .spawn_actor(Actor::new(GridPos::new(3, 2), 10).unwrap())
            .unwrap();
        let witness = game
            .spawn_actor(
                Actor::new(GridPos::new(2, 3), 10)
                    .unwrap()
                    .with_affiliation(owner.clone())
                    .with_witness_profile(
                        WitnessProfile::new(4, DistanceMetric::Euclidean, true, 4).unwrap(),
                    )
                    .with_local_alert_profile(LocalAlertProfile::new(8).unwrap()),
            )
            .unwrap();
        game.spawn_ground_item_with_owner(
            GridPos::new(2, 2),
            required_item.clone(),
            1,
            Some(owner.clone()),
        )
        .unwrap();
        let mut world = WorldState::single(game);
        let zone = id("authorization_city");
        world.enable(info("authorization_city", 0)).unwrap();
        let quest = id("earn_salvage_right");
        let authored = HubQuestDefinition::new(
            crate::content::HubQuestProviderDefinition::existing(GridPos::new(3, 2)).unwrap(),
            DeliveryQuestDefinition::new(
                quest.clone(),
                "quest.earn_salvage_right.title".into(),
                "quest.earn_salvage_right.summary".into(),
                required_item.clone(),
                1,
                17,
            )
            .unwrap()
            .into(),
        )
        .with_world_effects(vec![
            QuestWorldEffectDefinition::grant_property_take_authorization(
                owner.clone(),
                "world_effect.salvage_authorized.summary".into(),
            )
            .unwrap(),
        ])
        .unwrap();
        world
            .register_authored_quest(zone, provider, authored)
            .unwrap();
        world.drain_events();
        (world, provider, witness, quest, required_item, owner)
    }

    fn exploration_quest_world() -> (WorldState, crate::entity::EntityId, QuestId) {
        exploration_quest_world_with_records(Vec::new())
    }

    fn exploration_quest_world_with_records(
        qualifying_records: Vec<ContentId>,
    ) -> (WorldState, crate::entity::EntityId, QuestId) {
        let game =
            GameState::new_with_rules(map(), GridPos::new(2, 2), 42, GameRules::default()).unwrap();
        let mut world = WorldState::single(game);
        let city = id("exploration_city");
        world
            .enable(ZoneInfo {
                id: city.clone(),
                name: "exploration_city".into(),
                kind: id("city"),
                depth: 0,
            })
            .unwrap();
        let provider = world
            .spawn_actor(Actor::new(GridPos::new(3, 2), 10).unwrap())
            .unwrap();
        for (name, depth) in [("exploration_b", 1), ("exploration_c", 2)] {
            world
                .add_zone(ZoneBlueprint {
                    info: info(name, depth),
                    map: map(),
                    entrance: GridPos::new(1, 1),
                    seed: u64::from(depth) + 900,
                    actors: vec![],
                    loot: vec![],
                    threat_sources: vec![],
                })
                .unwrap();
        }
        world
            .connect(
                city.clone(),
                GridPos::new(2, 1),
                id("exploration_b"),
                GridPos::new(1, 1),
            )
            .unwrap();
        world
            .connect(
                id("exploration_b"),
                GridPos::new(2, 1),
                id("exploration_c"),
                GridPos::new(1, 1),
            )
            .unwrap();
        let quest = id("exploration");
        let mut definition = ExplorationQuestDefinition::new(
            quest.clone(),
            "quest.exploration.title".into(),
            "quest.exploration.summary".into(),
            2,
            70,
        )
        .unwrap();
        if !qualifying_records.is_empty() {
            definition = definition
                .with_qualifying_records(qualifying_records)
                .unwrap();
        }
        world
            .register_exploration_quest(city, provider, definition)
            .unwrap();
        world.drain_events();
        (world, provider, quest)
    }

    fn register_survey_terminal(
        world: &mut WorldState,
        zone: ContentId,
        record: ContentId,
        wrong_record: Option<ContentId>,
    ) {
        let mut installations = vec![
            InstallationBlueprint {
                id: id("survey_terminal"),
                position: GridPos::new(4, 1),
                maximum_integrity: 10,
                integrity: 10,
                capabilities: vec![InstallationCapability::DataTerminal { record }],
                dependencies: vec![],
                security_alarm_profile: None,
            },
            InstallationBlueprint {
                id: id("survey_depot"),
                position: GridPos::new(5, 1),
                maximum_integrity: 10,
                integrity: 10,
                capabilities: vec![InstallationCapability::Storage],
                dependencies: vec![],
                security_alarm_profile: None,
            },
        ];
        if let Some(record) = wrong_record {
            installations.push(InstallationBlueprint {
                id: id("unrelated_terminal"),
                position: GridPos::new(2, 2),
                maximum_integrity: 10,
                integrity: 10,
                capabilities: vec![InstallationCapability::DataTerminal { record }],
                dependencies: vec![],
                security_alarm_profile: None,
            });
        }
        world
            .register_facility(
                zone,
                FacilityBlueprint {
                    installations,
                    depot: id("survey_depot"),
                    workers: vec![],
                    repair_orders: vec![],
                    maximum_path_search: 256,
                    owner: None,
                },
            )
            .unwrap();
    }

    fn data_record_quest_world() -> (
        WorldState,
        crate::entity::EntityId,
        QuestId,
        ContentId,
        GridPos,
        GridPos,
    ) {
        let game =
            GameState::new_with_rules(map(), GridPos::new(2, 2), 42, GameRules::default()).unwrap();
        let mut world = WorldState::single(game);
        let zone = id("data_record_city");
        world.enable(info("data_record_city", 0)).unwrap();
        let provider = world
            .spawn_actor(Actor::new(GridPos::new(3, 2), 10).unwrap())
            .unwrap();
        let terminal_position = GridPos::new(2, 1);
        let other_terminal_position = GridPos::new(1, 2);
        let record = id("sealed_archive");
        world
            .register_facility(
                zone.clone(),
                FacilityBlueprint {
                    installations: vec![
                        InstallationBlueprint {
                            id: id("archive_terminal"),
                            position: terminal_position,
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::DataTerminal {
                                record: record.clone(),
                            }],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                        InstallationBlueprint {
                            id: id("unrelated_terminal"),
                            position: other_terminal_position,
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::DataTerminal {
                                record: id("unrelated_archive"),
                            }],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                        InstallationBlueprint {
                            id: id("archive_depot"),
                            position: GridPos::new(3, 1),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::Storage],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                    ],
                    depot: id("archive_depot"),
                    workers: vec![],
                    repair_orders: vec![],
                    maximum_path_search: 256,
                    owner: None,
                },
            )
            .unwrap();
        let quest = id("consult_archive");
        world
            .register_data_record_quest(
                zone,
                provider,
                DataRecordQuestDefinition::new(
                    quest.clone(),
                    "quest.consult_archive.title".into(),
                    "quest.consult_archive.summary".into(),
                    record.clone(),
                    55,
                )
                .unwrap(),
            )
            .unwrap();
        world.drain_events();
        (
            world,
            provider,
            quest,
            record,
            terminal_position,
            other_terminal_position,
        )
    }

    fn terminal_update_quest_world() -> (
        WorldState,
        crate::entity::EntityId,
        QuestId,
        ContentId,
        ContentId,
        ContentId,
        GridPos,
    ) {
        let (mut world, provider, _, initial_record, terminal_position, _) =
            data_record_quest_world();
        let zone = id("data_record_city");
        let quest = id("correct_archive");
        let terminal = id("archive_terminal");
        let revised_record = id("verified_archive");
        let authored = HubQuestDefinition::new(
            crate::content::HubQuestProviderDefinition::existing(GridPos::new(3, 2)).unwrap(),
            DataRecordQuestDefinition::new(
                quest.clone(),
                "quest.correct_archive.title".into(),
                "quest.correct_archive.summary".into(),
                initial_record.clone(),
                21,
            )
            .unwrap()
            .into(),
        )
        .with_world_effects(vec![
            QuestWorldEffectDefinition::update_data_terminal(
                terminal.clone(),
                revised_record.clone(),
                "world_effect.archive_corrected.summary".into(),
            )
            .unwrap(),
        ])
        .unwrap();
        world
            .register_authored_quest(zone, provider, authored)
            .unwrap();
        world.drain_events();
        (
            world,
            provider,
            quest,
            terminal,
            initial_record,
            revised_record,
            terminal_position,
        )
    }

    fn defeat_targets_quest_world() -> (WorldState, crate::entity::EntityId, QuestId, ContentId) {
        let game =
            GameState::new_with_rules(map(), GridPos::new(2, 2), 42, GameRules::default()).unwrap();
        let mut world = WorldState::single(game);
        let zone = id("combat_quest_city");
        world.enable(info("combat_quest_city", 0)).unwrap();
        let provider = world
            .spawn_actor(Actor::new(GridPos::new(3, 2), 10).unwrap())
            .unwrap();
        let quest = id("defeat_rust_hounds");
        let target_tag = id("rust_hound");
        world
            .register_defeat_targets_quest(
                zone,
                provider,
                DefeatTargetsQuestDefinition::new(
                    quest.clone(),
                    "quest.defeat_rust_hounds.title".into(),
                    "quest.defeat_rust_hounds.summary".into(),
                    target_tag.clone(),
                    2,
                    80,
                )
                .unwrap(),
            )
            .unwrap();
        world.drain_events();
        (world, provider, quest, target_tag)
    }

    fn spawn_quest_target(world: &mut WorldState, tag: ContentId) -> crate::entity::EntityId {
        world
            .spawn_actor(
                Actor::new(GridPos::new(2, 1), 1)
                    .unwrap()
                    .with_evasion_disabled()
                    .with_tags([tag]),
            )
            .unwrap()
    }

    #[test]
    fn delivery_quest_keeps_the_npc_role_and_tracks_all_four_states() {
        let (mut world, giver, quest, required_item) = quest_world();
        let interaction = world.npc_interaction(giver).unwrap();
        assert_eq!(interaction.role, NpcRole::Resident);
        assert!(interaction.services.is_empty());
        assert_eq!(interaction.quests.len(), 1);
        assert_eq!(interaction.quests[0].status, QuestStatus::Available);
        assert!(world.quest_journal().is_empty());
        assert_eq!(world.quest_giver_role(&id("quest_city"), giver), None);
        let turn = world.turn();

        assert_eq!(
            world.process_player_command(GameCommand::AcceptQuest {
                giver,
                quest: quest.clone(),
            }),
            CommandOutcome::AppliedWithoutTime
        );
        assert_eq!(world.turn(), turn);
        let journal = world.quest_journal();
        assert_eq!(journal.len(), 1);
        assert_eq!(journal[0].zone.id, id("quest_city"));
        assert_eq!(journal[0].giver, giver);
        assert_eq!(
            world.quest_giver_role(&journal[0].zone.id, giver),
            Some(NpcRole::Resident)
        );
        assert_eq!(journal[0].quest.status, QuestStatus::Active);
        assert_eq!(
            world.npc_interaction(giver).unwrap().quests[0].status,
            QuestStatus::Active
        );
        assert!(world.events().contains(&GameEvent::QuestAccepted {
            giver,
            quest: quest.clone(),
        }));

        world
            .grant_player_item_for_diagnostic(required_item.clone(), 2)
            .unwrap();
        assert_eq!(
            world.npc_interaction(giver).unwrap().quests[0].status,
            QuestStatus::ReadyToComplete
        );
        assert_eq!(
            world.quest_journal()[0].quest.status,
            QuestStatus::ReadyToComplete
        );
        assert_eq!(
            world.process_player_command(GameCommand::CompleteQuest {
                giver,
                quest: quest.clone(),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.turn(), turn + 1);
        assert_eq!(world.player_credits(), 45);
        assert!(
            world
                .player_inventory()
                .iter()
                .all(|entry| entry.item() != &required_item)
        );
        assert_eq!(
            world.npc_interaction(giver).unwrap().quests[0].status,
            QuestStatus::Completed
        );
        assert_eq!(
            world.quest_journal()[0].quest.status,
            QuestStatus::Completed
        );
        assert!(world.events().contains(&GameEvent::QuestCompleted {
            giver,
            quest,
            objective: QuestCompletion::Delivery {
                item: required_item,
                quantity: 2,
            },
            reward_credits: 45,
            reward_experience: 0,
            reward_items: vec![],
            world_states: vec![],
            world_effects: vec![],
            player_credits: 45,
        }));
    }

    #[test]
    fn incomplete_delivery_quest_rejection_is_atomic() {
        let (mut world, giver, quest, required_item) = quest_world();
        assert_eq!(
            world.process_player_command(GameCommand::AcceptQuest {
                giver,
                quest: quest.clone(),
            }),
            CommandOutcome::AppliedWithoutTime
        );
        world
            .grant_player_item_for_diagnostic(required_item, 1)
            .unwrap();
        let before = format!("{world:?}");

        assert_eq!(
            world.process_player_command(GameCommand::CompleteQuest { giver, quest }),
            CommandOutcome::Rejected(CommandRejection::QuestObjectiveIncomplete)
        );
        assert_eq!(format!("{world:?}"), before);
    }

    #[test]
    fn quest_completion_applies_a_visible_material_effect_atomically() {
        let (mut world, giver, quest, required_item, door) = world_effect_quest_world();
        let view = &world.npc_interaction(giver).unwrap().quests[0];
        assert_eq!(
            view.completion_world_effects,
            vec![
                QuestWorldEffectDefinition::unlock_door(
                    door,
                    "world_effect.unlock_access.summary".into(),
                )
                .unwrap()
            ]
        );
        world.process_player_command(GameCommand::AcceptQuest {
            giver,
            quest: quest.clone(),
        });
        world
            .grant_player_item_for_diagnostic(required_item, 1)
            .unwrap();

        assert_eq!(
            world.process_player_command(GameCommand::CompleteQuest {
                giver,
                quest: quest.clone(),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.map().tile(door).map(|tile| tile.terrain),
            Some(Terrain::Door(DoorState::Closed))
        );
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::QuestCompleted { world_effects, .. }
                if world_effects.len() == 1 && world_effects[0].door_position() == Some(door)
        )));
    }

    #[test]
    fn unavailable_material_effect_rejects_the_entire_quest_completion() {
        let (mut world, giver, quest, required_item, door) = world_effect_quest_world();
        world.process_player_command(GameCommand::AcceptQuest {
            giver,
            quest: quest.clone(),
        });
        world
            .grant_player_item_for_diagnostic(required_item, 1)
            .unwrap();
        world
            .active
            .map
            .set_terrain(door, Terrain::Door(DoorState::Closed))
            .unwrap();
        let before = format!("{world:?}");

        assert_eq!(
            world.process_player_command(GameCommand::CompleteQuest { giver, quest }),
            CommandOutcome::Rejected(CommandRejection::QuestWorldEffectUnavailable)
        );
        assert_eq!(format!("{world:?}"), before);
    }

    #[test]
    fn quest_completion_updates_a_terminal_without_erasing_the_read_record() {
        let (mut world, giver, quest, terminal, initial_record, revised_record, terminal_position) =
            terminal_update_quest_world();
        let effect = QuestWorldEffectDefinition::update_data_terminal(
            terminal.clone(),
            revised_record.clone(),
            "world_effect.archive_corrected.summary".into(),
        )
        .unwrap();
        let view = world
            .npc_interaction(giver)
            .unwrap()
            .quests
            .into_iter()
            .find(|candidate| candidate.id == quest)
            .unwrap();
        assert_eq!(view.completion_world_effects, vec![effect.clone()]);
        assert_eq!(
            world.process_player_command(GameCommand::AcceptQuest {
                giver,
                quest: quest.clone(),
            }),
            CommandOutcome::AppliedWithoutTime
        );
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: terminal_position,
            }),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.discovered_data_terminal_records(),
            BTreeSet::from([initial_record.clone()])
        );

        assert_eq!(
            world.process_player_command(GameCommand::CompleteQuest {
                giver,
                quest: quest.clone(),
            }),
            CommandOutcome::Applied
        );
        let facility = world.active_facility().unwrap();
        assert!(facility.data_terminal_was_updated(&terminal));
        assert!(!facility.data_terminal_was_accessed(&terminal));
        assert_eq!(
            facility.data_terminal_record_at(terminal_position),
            Some(&revised_record)
        );
        assert_eq!(
            world.discovered_data_terminal_records(),
            BTreeSet::from([initial_record.clone()])
        );
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::QuestCompleted { world_effects, .. }
                if world_effects == &vec![effect.clone()]
        )));

        world.drain_events();
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: terminal_position,
            }),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.discovered_data_terminal_records(),
            BTreeSet::from([initial_record, revised_record.clone()])
        );
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::Facility(FacilityEvent::DataTerminalAccessed {
                installation,
                record,
                first_access: true,
                ..
            }) if installation == &terminal && record == &revised_record
        )));
    }

    #[test]
    fn unavailable_terminal_effect_rejects_the_entire_quest_completion() {
        let (mut world, giver, quest, terminal, _, revised_record, terminal_position) =
            terminal_update_quest_world();
        world.process_player_command(GameCommand::AcceptQuest {
            giver,
            quest: quest.clone(),
        });
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: terminal_position,
            }),
            CommandOutcome::Applied
        );
        world
            .facilities
            .get_mut(&id("data_record_city"))
            .unwrap()
            .update_data_terminal_record(&terminal, revised_record)
            .unwrap();
        let before = format!("{world:?}");

        assert_eq!(
            world.process_player_command(GameCommand::CompleteQuest { giver, quest }),
            CommandOutcome::Rejected(CommandRejection::QuestWorldEffectUnavailable)
        );
        assert_eq!(format!("{world:?}"), before);
    }

    #[test]
    fn quest_authorization_permanently_allows_that_owners_property_without_an_incident() {
        let (mut world, giver, witness, quest, required_item, owner) = authorization_quest_world();
        let effect = QuestWorldEffectDefinition::grant_property_take_authorization(
            owner.clone(),
            "world_effect.salvage_authorized.summary".into(),
        )
        .unwrap();
        assert!(!world.player_may_take_property_of(&owner));
        assert_eq!(
            world.npc_interaction(giver).unwrap().quests[0].completion_world_effects,
            vec![effect.clone()]
        );
        assert_eq!(
            world.process_player_command(GameCommand::AcceptQuest {
                giver,
                quest: quest.clone(),
            }),
            CommandOutcome::AppliedWithoutTime
        );
        world
            .grant_player_item_for_diagnostic(required_item.clone(), 1)
            .unwrap();

        assert_eq!(
            world.process_player_command(GameCommand::CompleteQuest {
                giver,
                quest: quest.clone(),
            }),
            CommandOutcome::Applied
        );
        assert!(world.player_may_take_property_of(&owner));
        assert!(!world.player_may_take_property_of(&id("unrelated_collective")));
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::QuestCompleted { world_effects, .. }
                if world_effects == &vec![effect.clone()]
        )));

        world.drain_events();
        assert_eq!(
            world.process_player_command(GameCommand::PickUp),
            CommandOutcome::Applied
        );
        let witness = world.actors().get(witness).unwrap();
        assert!(witness.observed_property_takes().is_empty());
        assert!(witness.local_alert().is_none());
        assert!(world.events().iter().all(|event| !matches!(
            event,
            GameEvent::PropertyTakeWitnessed { .. }
                | GameEvent::PropertyTakeReported { .. }
                | GameEvent::LocalAlertRaised { .. }
        )));
        assert!(
            world
                .player_inventory()
                .iter()
                .any(|entry| { entry.item() == &required_item && entry.owner() == Some(&owner) })
        );
    }

    #[test]
    fn redundant_quest_authorization_rejects_the_entire_completion() {
        let (mut world, giver, _, quest, required_item, owner) = authorization_quest_world();
        assert_eq!(
            world.process_player_command(GameCommand::AcceptQuest {
                giver,
                quest: quest.clone(),
            }),
            CommandOutcome::AppliedWithoutTime
        );
        world
            .grant_player_item_for_diagnostic(required_item, 1)
            .unwrap();
        assert!(world.grant_player_property_take_authorization(owner));
        let before = format!("{world:?}");

        assert_eq!(
            world.process_player_command(GameCommand::CompleteQuest { giver, quest }),
            CommandOutcome::Rejected(CommandRejection::QuestWorldEffectUnavailable)
        );
        assert_eq!(format!("{world:?}"), before);
    }

    #[test]
    fn quest_chain_unlocks_after_completion_and_pays_atomic_item_and_experience_rewards() {
        let (mut world, giver, first, reward_item) = quest_world();
        let zone = id("quest_city");
        let follow_up = id("follow_up");
        let final_report = id("final_report");
        let reported = id("reported");
        let world_state =
            QuestWorldStateDefinition::new(reported.clone(), "world.reported.summary".into())
                .unwrap()
                .with_provider_dialogue("world.reported.dialogue".into())
                .unwrap();
        let authored = HubQuestDefinition::new(
            crate::content::HubQuestProviderDefinition::existing(GridPos::new(3, 2)).unwrap(),
            DeliveryQuestDefinition::new(
                follow_up.clone(),
                "quest.follow_up.title".into(),
                "quest.follow_up.summary".into(),
                reward_item.clone(),
                1,
                5,
            )
            .unwrap()
            .into(),
        )
        .with_prerequisites(vec![first.clone()])
        .unwrap()
        .with_world_states(Vec::new(), vec![world_state.clone()])
        .unwrap()
        .with_additional_rewards(
            20,
            vec![QuestItemRewardDefinition::new(reward_item.clone(), 2).unwrap()],
        )
        .unwrap();
        world
            .register_authored_quest(zone.clone(), giver, authored)
            .unwrap();
        world
            .register_authored_quest(
                zone,
                giver,
                HubQuestDefinition::new(
                    crate::content::HubQuestProviderDefinition::existing(GridPos::new(3, 2))
                        .unwrap(),
                    DeliveryQuestDefinition::new(
                        final_report.clone(),
                        "quest.final.title".into(),
                        "quest.final.summary".into(),
                        reward_item.clone(),
                        1,
                        0,
                    )
                    .unwrap()
                    .into(),
                )
                .with_prerequisites(vec![follow_up.clone()])
                .unwrap()
                .with_world_states(vec![reported.clone()], Vec::new())
                .unwrap(),
            )
            .unwrap();

        assert_eq!(world.npc_interaction(giver).unwrap().quests.len(), 1);
        world.process_player_command(GameCommand::AcceptQuest {
            giver,
            quest: first.clone(),
        });
        world
            .grant_player_item_for_diagnostic(reward_item.clone(), 2)
            .unwrap();
        assert_eq!(
            world.process_player_command(GameCommand::CompleteQuest {
                giver,
                quest: first,
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.quest_marker(giver), Some(QuestMarker::Available));
        assert!(
            world
                .npc_interaction(giver)
                .unwrap()
                .quests
                .iter()
                .any(|quest| quest.id == follow_up && quest.status == QuestStatus::Available)
        );

        assert_eq!(
            world.process_player_command(GameCommand::AcceptQuest {
                giver,
                quest: follow_up.clone(),
            }),
            CommandOutcome::AppliedWithoutTime
        );
        world
            .grant_player_item_for_diagnostic(reward_item.clone(), 1)
            .unwrap();
        world.drain_events();
        assert_eq!(
            world.process_player_command(GameCommand::CompleteQuest {
                giver,
                quest: follow_up.clone(),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.player_progression().experience(), 20);
        assert!(world.world_state_active(&reported));
        let interaction = world.npc_interaction(giver).unwrap();
        assert_eq!(
            interaction.contextual_dialogue_key.as_deref(),
            Some("world.reported.dialogue")
        );
        assert!(
            interaction.quests.iter().any(|quest| {
                quest.id == final_report && quest.status == QuestStatus::Available
            })
        );
        assert_eq!(
            world
                .player_inventory()
                .iter()
                .filter(|entry| entry.item() == &reward_item)
                .map(|entry| u32::from(entry.quantity()))
                .sum::<u32>(),
            2
        );
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::QuestCompleted {
                quest,
                reward_experience: 20,
                reward_items,
                world_states,
                ..
            } if quest == &follow_up && reward_items == &vec![QuestItemRewardDefinition {
                item: reward_item.clone(),
                quantity: 2,
            }] && world_states == &vec![world_state.clone()]
        )));
    }

    #[test]
    fn accepting_one_dialogue_offer_excludes_the_other_branch() {
        let (mut world, giver) = resident_world();
        let zone = id("resident");
        let group = id("approach");
        let prompt = "dialogue.approach.prompt".to_owned();
        let left = id("left_branch");
        let right = id("right_branch");
        for quest in [left.clone(), right.clone()] {
            let authored = HubQuestDefinition::new(
                crate::content::HubQuestProviderDefinition::existing(GridPos::new(3, 2)).unwrap(),
                ExplorationQuestDefinition::new(
                    quest.clone(),
                    format!("quest.{}.title", quest.name()),
                    format!("quest.{}.summary", quest.name()),
                    1,
                    0,
                )
                .unwrap()
                .into(),
            )
            .with_choice(group.clone(), prompt.clone())
            .unwrap();
            world
                .register_authored_quest(zone.clone(), giver, authored)
                .unwrap();
        }
        assert_eq!(world.npc_interaction(giver).unwrap().quests.len(), 2);
        assert_eq!(world.quest_marker(giver), Some(QuestMarker::Available));

        assert_eq!(
            world.process_player_command(GameCommand::AcceptQuest {
                giver,
                quest: left.clone(),
            }),
            CommandOutcome::AppliedWithoutTime
        );
        let interaction = world.npc_interaction(giver).unwrap();
        assert_eq!(interaction.quests.len(), 1);
        assert_eq!(interaction.quests[0].id, left);
        assert_eq!(interaction.quests[0].status, QuestStatus::Active);
        assert_eq!(
            world.process_player_command(GameCommand::AcceptQuest {
                giver,
                quest: right,
            }),
            CommandOutcome::Rejected(CommandRejection::QuestBranchUnavailable)
        );
    }

    #[test]
    fn data_record_quest_requires_a_successful_terminal_access_after_acceptance() {
        let (mut world, giver, quest, record, terminal, other_terminal) = data_record_quest_world();

        assert_eq!(
            world.process_player_command(GameCommand::Interact { target: terminal }),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.process_player_command(GameCommand::AcceptQuest {
                giver,
                quest: quest.clone(),
            }),
            CommandOutcome::AppliedWithoutTime
        );
        assert!(matches!(
            world.quest_journal()[0].quest.objective,
            QuestObjectiveView::AccessDataRecord {
                accessed: false,
                ref record,
            } if record == &id("sealed_archive")
        ));

        world.drain_events();
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: other_terminal,
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.quest_journal()[0].quest.status, QuestStatus::Active);
        assert!(
            !world
                .events()
                .iter()
                .any(|event| matches!(event, GameEvent::QuestProgressed { .. }))
        );

        world.drain_events();
        assert_eq!(
            world.process_player_command(GameCommand::Interact { target: terminal }),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.quest_journal()[0].quest.status,
            QuestStatus::ReadyToComplete
        );
        assert!(world.events().contains(&GameEvent::QuestProgressed {
            quest: quest.clone(),
            current: 1,
            required: 1,
        }));

        world.drain_events();
        assert_eq!(
            world.process_player_command(GameCommand::Interact { target: terminal }),
            CommandOutcome::Applied
        );
        assert!(
            !world
                .events()
                .iter()
                .any(|event| matches!(event, GameEvent::QuestProgressed { .. }))
        );

        assert_eq!(
            world.process_player_command(GameCommand::CompleteQuest {
                giver,
                quest: quest.clone(),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.player_credits(), 55);
        assert!(world.events().contains(&GameEvent::QuestCompleted {
            giver,
            quest,
            objective: QuestCompletion::AccessDataRecord { record },
            reward_credits: 55,
            reward_experience: 0,
            reward_items: vec![],
            world_states: vec![],
            world_effects: vec![],
            player_credits: 55,
        }));
    }

    #[test]
    fn defeat_targets_quest_counts_only_matching_player_defeats_after_acceptance() {
        let (mut world, giver, quest, target_tag) = defeat_targets_quest_world();

        let early = spawn_quest_target(&mut world, target_tag.clone());
        assert_eq!(
            world.process_player_command(GameCommand::Attack {
                slot: 0,
                target: early,
            }),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.process_player_command(GameCommand::AcceptQuest {
                giver,
                quest: quest.clone(),
            }),
            CommandOutcome::AppliedWithoutTime
        );
        assert!(matches!(
            world.quest_journal()[0].quest.objective,
            QuestObjectiveView::DefeatTargets {
                defeated_quantity: 0,
                required_quantity: 2,
                ref target_tag,
            } if target_tag == &id("rust_hound")
        ));

        let unrelated = spawn_quest_target(&mut world, id("scrap_drone"));
        assert_eq!(
            world.process_player_command(GameCommand::Attack {
                slot: 0,
                target: unrelated,
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.quest_journal()[0].quest.status, QuestStatus::Active);

        for expected in 1..=2 {
            let target = spawn_quest_target(&mut world, target_tag.clone());
            world.drain_events();
            assert_eq!(
                world.process_player_command(GameCommand::Attack { slot: 0, target }),
                CommandOutcome::Applied
            );
            assert!(world.events().contains(&GameEvent::EntityDefeatedByPlayer {
                entity: target,
                at: GridPos::new(2, 1),
                tags: vec![target_tag.clone()],
            }));
            assert!(world.events().contains(&GameEvent::QuestProgressed {
                quest: quest.clone(),
                current: expected,
                required: 2,
            }));
        }
        assert_eq!(
            world.quest_journal()[0].quest.status,
            QuestStatus::ReadyToComplete
        );

        assert_eq!(
            world.process_player_command(GameCommand::CompleteQuest {
                giver,
                quest: quest.clone(),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.player_credits(), 80);
        assert!(world.events().contains(&GameEvent::QuestCompleted {
            giver,
            quest,
            objective: QuestCompletion::DefeatTargets {
                target_tag,
                quantity: 2,
            },
            reward_credits: 80,
            reward_experience: 0,
            reward_items: vec![],
            world_states: vec![],
            world_effects: vec![],
            player_credits: 80,
        }));
    }

    #[test]
    fn exploration_quest_counts_only_distinct_new_zones_and_completes_at_its_giver() {
        let (mut world, giver, quest) = exploration_quest_world();
        assert_eq!(
            world.process_player_command(GameCommand::AcceptQuest {
                giver,
                quest: quest.clone(),
            }),
            CommandOutcome::AppliedWithoutTime
        );
        assert!(matches!(
            world.quest_journal()[0].quest.objective,
            QuestObjectiveView::ExploreZones {
                explored_zones: 0,
                required_zones: 2,
                ..
            }
        ));

        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(2, 1),
            }),
            CommandOutcome::Applied
        );
        assert!(matches!(
            world.quest_journal()[0].quest.objective,
            QuestObjectiveView::ExploreZones {
                explored_zones: 1,
                required_zones: 2,
                ..
            }
        ));
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(1, 1),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(2, 1),
            }),
            CommandOutcome::Applied
        );
        assert!(matches!(
            world.quest_journal()[0].quest.objective,
            QuestObjectiveView::ExploreZones {
                explored_zones: 1,
                required_zones: 2,
                ..
            }
        ));

        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(2, 1),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.quest_journal()[0].quest.status,
            QuestStatus::ReadyToComplete
        );
        assert!(world.events().contains(&GameEvent::QuestProgressed {
            quest: quest.clone(),
            current: 2,
            required: 2,
        }));
        assert_eq!(
            world.next_visited_passage_towards(&"test:exploration_city".parse().unwrap()),
            Some(GridPos::new(1, 1))
        );
        let entry = &world.quest_journal()[0];
        assert_ne!(
            world.current_zone().map(|zone| &zone.id),
            Some(&entry.zone.id)
        );
        assert_eq!(
            world.quest_giver_role(&entry.zone.id, entry.giver),
            Some(NpcRole::QuestContact)
        );

        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(1, 1),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(1, 1),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.process_player_command(GameCommand::Move(Direction::South)),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.process_player_command(GameCommand::CompleteQuest {
                giver,
                quest: quest.clone(),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.player_credits(), 70);
        assert_eq!(
            world.quest_journal()[0].quest.status,
            QuestStatus::Completed
        );
        assert!(world.events().contains(&GameEvent::QuestCompleted {
            giver,
            quest,
            objective: QuestCompletion::ExploreZones { zones: 2 },
            reward_credits: 70,
            reward_experience: 0,
            reward_items: vec![],
            world_states: vec![],
            world_effects: vec![],
            player_credits: 70,
        }));
    }

    #[test]
    fn site_survey_requires_a_matching_terminal_in_each_new_zone() {
        let record = id("site_record");
        let (mut world, giver, quest) = exploration_quest_world_with_records(vec![record.clone()]);
        register_survey_terminal(
            &mut world,
            id("exploration_b"),
            record.clone(),
            Some(id("unrelated_record")),
        );
        register_survey_terminal(&mut world, id("exploration_c"), record, None);
        assert_eq!(
            world.process_player_command(GameCommand::AcceptQuest {
                giver,
                quest: quest.clone(),
            }),
            CommandOutcome::AppliedWithoutTime
        );
        world.drain_events();

        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(2, 1),
            }),
            CommandOutcome::Applied
        );
        assert!(matches!(
            world.quest_journal()[0].quest.objective,
            QuestObjectiveView::ExploreZones {
                explored_zones: 0,
                site_record_required: true,
                ..
            }
        ));
        assert_eq!(
            world.process_player_command(GameCommand::Move(Direction::East)),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(2, 2),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.quest_journal()[0].quest.status, QuestStatus::Active);
        assert_eq!(
            world.process_player_command(GameCommand::Move(Direction::East)),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(4, 1),
            }),
            CommandOutcome::Applied
        );
        assert!(matches!(
            world.quest_journal()[0].quest.objective,
            QuestObjectiveView::ExploreZones {
                explored_zones: 1,
                site_record_required: true,
                ..
            }
        ));
        world.drain_events();
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(4, 1),
            }),
            CommandOutcome::Applied
        );
        assert!(!world.events().iter().any(|event| matches!(
            event,
            GameEvent::QuestProgressed { quest: progressed, .. } if progressed == &quest
        )));

        assert_eq!(
            world.process_player_command(GameCommand::Move(Direction::West)),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(2, 1),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.quest_journal()[0].quest.status, QuestStatus::Active);
        for _ in 0..2 {
            assert_eq!(
                world.process_player_command(GameCommand::Move(Direction::East)),
                CommandOutcome::Applied
            );
        }
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(4, 1),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.quest_journal()[0].quest.status,
            QuestStatus::ReadyToComplete
        );

        for _ in 0..2 {
            assert_eq!(
                world.process_player_command(GameCommand::Move(Direction::West)),
                CommandOutcome::Applied
            );
        }
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(1, 1),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(1, 1),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.process_player_command(GameCommand::Move(Direction::South)),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.process_player_command(GameCommand::CompleteQuest { giver, quest }),
            CommandOutcome::Applied
        );
        assert_eq!(world.player_credits(), 70);
        assert_eq!(
            world.quest_journal()[0].quest.status,
            QuestStatus::Completed
        );
    }

    #[test]
    fn clinic_treatment_is_atomic_priced_by_restored_points_and_spends_one_turn() {
        let (mut world, healer) = clinic_world(100);
        let player = world.player_id();
        let actor = world.active.actors.get_mut(player).unwrap();
        assert_eq!(actor.apply_damage(6), 6);
        let integrity_after_damage = actor.integrity();
        let turn = world.turn();

        let interaction = world.npc_interaction(healer).unwrap();
        let [
            NpcService::Treatment {
                restore_amount,
                price,
                routine,
                ..
            },
        ] = interaction.services.as_slice()
        else {
            panic!("clinic treatment service missing")
        };
        assert_eq!((*restore_amount, *price), (4, 12));
        assert_eq!(*routine, ClinicRoutineState::AtWork);

        assert_eq!(
            world.process_player_command(GameCommand::ReceiveTreatment { healer }),
            CommandOutcome::Applied
        );
        assert_eq!(world.turn(), turn + 1);
        assert_eq!(world.player_credits(), 88);
        assert_eq!(
            world.actors().get(player).unwrap().integrity(),
            integrity_after_damage + 4
        );
        assert!(world.events().contains(&GameEvent::TreatmentReceived {
            healer,
            amount: 4,
            price: 12,
            player_credits: 88,
        }));
        let interaction = world.npc_interaction(healer).unwrap();
        let [NpcService::Treatment { clinic_credits, .. }] = interaction.services.as_slice() else {
            panic!("clinic treatment service missing")
        };
        assert_eq!(*clinic_credits, 32);
    }

    #[test]
    fn clinic_refusals_mutate_neither_time_health_nor_money() {
        let (mut full, healer) = clinic_world(100);
        let turn = full.turn();
        assert_eq!(
            full.process_player_command(GameCommand::ReceiveTreatment { healer }),
            CommandOutcome::Rejected(CommandRejection::TreatmentNotNeeded)
        );
        assert_eq!(full.turn(), turn);
        assert_eq!(full.player_credits(), 100);

        let (mut poor, healer) = clinic_world(5);
        let player = poor.player_id();
        poor.active.actors.get_mut(player).unwrap().apply_damage(4);
        let integrity = poor.actors().get(player).unwrap().integrity();
        let turn = poor.turn();
        assert_eq!(
            poor.process_player_command(GameCommand::ReceiveTreatment { healer }),
            CommandOutcome::Rejected(CommandRejection::InsufficientCredits)
        );
        assert_eq!(poor.turn(), turn);
        assert_eq!(poor.player_credits(), 5);
        assert_eq!(poor.actors().get(player).unwrap().integrity(), integrity);
    }

    #[test]
    fn clinic_provider_follows_a_bounded_routine_without_disabling_care() {
        let (mut world, healer) = clinic_world(100);
        assert_eq!(
            world.actors().get(healer).unwrap().position(),
            GridPos::new(3, 2)
        );
        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.actors().get(healer).unwrap().position(),
            GridPos::new(3, 2)
        );
        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.actors().get(healer).unwrap().position(),
            GridPos::new(3, 2)
        );

        let interaction = world.npc_interaction(healer).unwrap();
        let [NpcService::Treatment { routine, .. }] = interaction.services.as_slice() else {
            panic!("clinic treatment service missing before provider moves")
        };
        assert_eq!(*routine, ClinicRoutineState::MovingToBreak);

        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.actors().get(healer).unwrap().position(),
            GridPos::new(4, 2)
        );

        let player = world.player_id();
        world
            .active
            .actors
            .move_to(player, GridPos::new(3, 2))
            .unwrap();
        let interaction = world.npc_interaction(healer).unwrap();
        let [NpcService::Treatment { routine, .. }] = interaction.services.as_slice() else {
            panic!("clinic treatment service missing while provider is moving")
        };
        assert_eq!(*routine, ClinicRoutineState::MovingToBreak);

        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.actors().get(healer).unwrap().position(),
            GridPos::new(5, 2)
        );
    }

    #[test]
    fn resident_conversation_is_free_of_services_and_reflects_the_bounded_routine() {
        let (mut world, resident) = resident_world();
        let turn = world.turn();
        let interaction = world.npc_interaction(resident).unwrap();
        assert_eq!(interaction.role, NpcRole::Resident);
        assert_eq!(
            interaction.resident_routine,
            Some(ResidentRoutineState::AtResidence)
        );
        assert!(interaction.services.is_empty());
        assert_eq!(world.turn(), turn, "opening dialogue must not spend a turn");

        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        let interaction = world.npc_interaction(resident).unwrap();
        assert_eq!(
            interaction.resident_routine,
            Some(ResidentRoutineState::MovingToGathering)
        );
        assert!(interaction.services.is_empty());

        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.actors().get(resident).unwrap().position(),
            GridPos::new(4, 2)
        );
    }

    #[test]
    fn resident_routine_continues_in_a_visited_offscreen_zone() {
        let mut world = world();
        let resident = world
            .spawn_actor(Actor::new(GridPos::new(3, 2), 10).unwrap())
            .unwrap();
        world
            .register_resident(
                id("a"),
                resident,
                ResidentDefinition::new(GridPos::new(3, 2), GridPos::new(5, 2), 10, 1, 20, 128)
                    .unwrap(),
            )
            .unwrap();

        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(2, 1),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.current_zone().unwrap().id, id("b"));
        for _ in 0..3 {
            assert_eq!(
                world.process_player_command(GameCommand::Wait),
                CommandOutcome::Applied
            );
        }
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(1, 1),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.current_zone().unwrap().id, id("a"));
        assert_eq!(
            world.actors().get(resident).unwrap().position(),
            GridPos::new(5, 2)
        );
    }

    #[test]
    fn commerce_preserves_resold_items_and_hides_gamble_properties_until_purchase() {
        let (mut world, merchant, armor) = commerce_world();
        let interaction = world.npc_interaction(merchant).unwrap();
        let [
            NpcService::Trade {
                offers,
                resale,
                gambles,
                ..
            },
        ] = interaction.services.as_slice()
        else {
            panic!("merchant trade service missing")
        };
        assert_eq!(offers[0].stock, 2);
        assert!(resale.is_empty());
        assert_eq!(gambles[0].item, armor);

        assert_eq!(
            world.process_player_command(GameCommand::BuyItem {
                merchant,
                item: armor.clone(),
            }),
            CommandOutcome::Applied
        );
        let bought = world
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &armor)
            .unwrap();
        assert_eq!(bought.magic_modifiers(), None);
        let bought = bought.instance();
        assert_eq!(world.player_credits(), 170);

        assert_eq!(
            world.process_player_command(GameCommand::SellItem {
                merchant,
                item: bought,
            }),
            CommandOutcome::Applied
        );
        let interaction = world.npc_interaction(merchant).unwrap();
        let [NpcService::Trade { resale, .. }] = interaction.services.as_slice() else {
            panic!("merchant trade service missing")
        };
        assert_eq!(resale.len(), 1);
        assert_eq!(resale[0].item, armor);
        assert_eq!(resale[0].magic_modifiers, None);
    }

    #[test]
    fn white_merchant_stock_uses_the_current_zone_depth() {
        let shallow_armor = id("shallow_market_armor");
        let deep_armor = id("deep_market_armor");
        let mut rules = GameRules::default();
        for armor in [&shallow_armor, &deep_armor] {
            rules
                .items
                .register(
                    ItemDefinition::new(
                        armor.clone(),
                        "armor.name".into(),
                        "armor.description".into(),
                        1,
                        ItemKind::Armor,
                        Some(EquipmentProfile::new(id("body"), 1).unwrap()),
                        vec![],
                    )
                    .unwrap(),
                )
                .unwrap();
        }
        let mut game = GameState::new_with_rules(map(), GridPos::new(2, 2), 42, rules).unwrap();
        let provider = game
            .spawn_actor(Actor::new(GridPos::new(3, 2), 10).unwrap())
            .unwrap();
        let mut world = WorldState::single(game);
        world.enable(info("deep_market", 3)).unwrap();
        let merchant = MerchantDefinition::new(
            GridPos::new(3, 2),
            10,
            500,
            vec![
                MerchantOfferDefinition {
                    item: shallow_armor.clone(),
                    initial_stock: 2,
                    buy_price: 30,
                    sell_price: 15,
                    minimum_depth: 0,
                    maximum_depth: Some(2),
                },
                MerchantOfferDefinition {
                    item: deep_armor.clone(),
                    initial_stock: 2,
                    buy_price: 90,
                    sell_price: 45,
                    minimum_depth: 3,
                    maximum_depth: None,
                },
            ],
            vec![MerchantGambleDefinition {
                item: shallow_armor,
                initial_stock: 1,
                price: 60,
            }],
            GambleScalingDefinition::new(3, 1, 12).unwrap(),
        )
        .unwrap();
        world
            .register_merchant(id("deep_market"), provider, 200, merchant, 7)
            .unwrap();

        let interaction = world.npc_interaction(provider).unwrap();
        let [NpcService::Trade { offers, .. }] = interaction.services.as_slice() else {
            panic!("merchant trade service missing")
        };
        assert_eq!(offers.len(), 1);
        assert_eq!(offers[0].item, deep_armor);
    }

    #[test]
    fn gambling_rolls_real_deterministic_magic_modifiers() {
        let (mut first, merchant, armor) = commerce_world();
        let (mut second, other_merchant, _) = commerce_world();
        for (world, provider) in [(&mut first, merchant), (&mut second, other_merchant)] {
            assert_eq!(
                world.process_player_command(GameCommand::GambleItem {
                    merchant: provider,
                    item: armor.clone(),
                }),
                CommandOutcome::Applied
            );
        }
        let first_modifiers = first
            .player_inventory()
            .iter()
            .find_map(|entry| entry.magic_modifiers())
            .unwrap();
        let second_modifiers = second
            .player_inventory()
            .iter()
            .find_map(|entry| entry.magic_modifiers())
            .unwrap();
        assert_eq!(first_modifiers, second_modifiers);
        assert!((1..=2).contains(&first_modifiers.armor_bonus()));
        assert!((5..=12).contains(&first_modifiers.mass_reduction_percent()));
        assert_eq!(first.player_credits(), 140);
    }

    #[test]
    fn gamble_quality_is_weighted_by_zone_depth_and_player_level() {
        let scaling = GambleScalingDefinition::new(3, 1, 12).unwrap();
        let early = GambleRollProfile::for_progression(scaling, 1, 0);
        let veteran = GambleRollProfile::for_progression(scaling, 10, 0);
        let deep = GambleRollProfile::for_progression(scaling, 1, 4);
        let deep_veteran = GambleRollProfile::for_progression(scaling, 10, 4);

        assert_eq!(early.rank, 1);
        assert!(veteran.rank > early.rank);
        assert!(deep.rank > early.rank);
        assert!(deep_veteran.rank > veteran.rank);
        assert!(deep_veteran.rank > deep.rank);
        assert!(deep_veteran.quality_draws > early.quality_draws);
        assert!(deep_veteran.quality_ceiling > early.quality_ceiling);

        let mut early_rng = GameRng::from_seed(99);
        let mut advanced_rng = GameRng::from_seed(99);
        let mut early_total = 0_u64;
        let mut advanced_total = 0_u64;
        for _ in 0..4_096 {
            let early_roll = early.roll(&mut early_rng);
            let advanced_roll = deep_veteran.roll(&mut advanced_rng);
            early_total += u64::from(early_roll.armor_bonus())
                + u64::from(early_roll.mass_reduction_percent());
            advanced_total += u64::from(advanced_roll.armor_bonus())
                + u64::from(advanced_roll.mass_reduction_percent());
        }
        assert!(advanced_total > early_total);
    }
    fn apply(world: &mut WorldState, command: GameCommand) {
        assert_eq!(
            world.process_player_command(command),
            CommandOutcome::Applied
        );
        world.drain_events();
    }
    fn outward(world: &mut WorldState) {
        apply(
            world,
            GameCommand::Interact {
                target: GridPos::new(2, 1),
            },
        );
    }
    fn home(world: &mut WorldState) {
        apply(
            world,
            GameCommand::Interact {
                target: GridPos::new(1, 1),
            },
        );
    }

    #[test]
    fn npc_dialogue_is_free_and_technician_delivery_uses_the_real_repair_order() {
        let mut world = world();
        apply(&mut world, GameCommand::Move(Direction::East));
        apply(&mut world, GameCommand::Move(Direction::South));
        let technician = world
            .spawn_actor(Actor::new(GridPos::new(3, 2), 10).unwrap())
            .unwrap();
        world
            .register_facility(
                id("a"),
                FacilityBlueprint {
                    installations: vec![
                        InstallationBlueprint {
                            id: id("relay"),
                            position: GridPos::new(6, 5),
                            maximum_integrity: 10,
                            integrity: 0,
                            capabilities: vec![InstallationCapability::PowerRelay],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                        InstallationBlueprint {
                            id: id("depot"),
                            position: GridPos::new(6, 1),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::Storage],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                    ],
                    depot: id("depot"),
                    workers: vec![WorkerBlueprint {
                        actor_position: GridPos::new(3, 2),
                        role: WorkerRole::Technician,
                        maximum_integrity: 10,
                        affiliation: None,
                        witness_profile: None,
                        local_alert_profile: None,
                        property_report: None,
                        installed_property_report: None,
                        reported_incident_response: None,
                    }],
                    repair_orders: vec![RepairOrderBlueprint {
                        id: id("restore"),
                        target: id("relay"),
                        required_item: id("repair"),
                        required_quantity: 1,
                        work_turns: 2,
                    }],
                    maximum_path_search: 256,
                    owner: None,
                },
            )
            .unwrap();
        world.drain_events();

        let turn = world.turn();
        let interaction = world.npc_interaction(technician).unwrap();
        assert_eq!(world.turn(), turn, "opening dialogue must not spend a turn");
        assert!(matches!(
            interaction.services.as_slice(),
            [NpcService::FacilityMaintenance {
                state: NpcServiceState::MaterialRequired {
                    player_can_supply: false,
                    known_source: None,
                },
                missing_quantity: 1,
                ..
            }]
        ));

        let reported_source = GridPos::new(4, 2);
        world
            .spawn_ground_item(reported_source, id("repair"), 1)
            .unwrap();
        let interaction = world.npc_interaction(technician).unwrap();
        assert!(matches!(
            interaction.services.as_slice(),
            [NpcService::FacilityMaintenance {
                state: NpcServiceState::MaterialRequired {
                    player_can_supply: false,
                    known_source: Some(source),
                },
                ..
            }] if *source == reported_source
        ));
        assert_eq!(
            world.turn(),
            turn,
            "the NPC report must remain a free query"
        );

        world
            .active
            .player_inventory_mut()
            .add(id("repair"), 1, 9)
            .unwrap();
        let interaction = world.npc_interaction(technician).unwrap();
        assert!(npc_service_can_supply(&interaction));
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: interaction.position,
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.turn(), turn + 1);
        assert!(world.player_inventory().is_empty());
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::Facility(FacilityEvent::PlayerMaterialDeposited {
                item,
                quantity: 1,
                ..
            }) if item == &id("repair")
        )));
        assert_ne!(
            world
                .active_facility()
                .and_then(|facility| facility.repair_status(&id("restore"))),
            Some(RepairStatus::WaitingForMaterial)
        );
    }

    fn npc_service_can_supply(interaction: &NpcInteraction) -> bool {
        interaction.services.iter().any(|service| {
            matches!(
                service,
                NpcService::FacilityMaintenance {
                    state: NpcServiceState::MaterialRequired {
                        player_can_supply: true,
                        ..
                    },
                    ..
                }
            )
        })
    }

    #[test]
    fn offscreen_normal_opportunity_clears_recovery_without_leaking_its_event() {
        let mut world = world();
        outward(&mut world);
        home(&mut world);
        let inactive = world.inactive.get_mut(&id("b")).unwrap();
        let idle = inactive
            .actors
            .iter()
            .find_map(|(entity, actor)| (actor.ai().is_some()).then_some(entity))
            .unwrap();
        let position = inactive.actors.get(idle).unwrap().position();
        inactive.actors.remove(idle);
        let actor = inactive
            .actors
            .spawn(
                Actor::new(position, 10)
                    .unwrap()
                    .with_ai(AiProfile::hunter(0, 0)),
            )
            .unwrap();
        inactive
            .actors
            .get_mut(actor)
            .unwrap()
            .start_action_recovery(TimeUnits::ONE);

        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(
            world
                .inactive
                .get(&id("b"))
                .and_then(|zone| zone.actors.get(actor))
                .and_then(Actor::recovery_remaining),
            None
        );
        assert!(!world.events().iter().any(|event| matches!(
            event,
            GameEvent::ActionRecoveryAdvanced { entity, .. }
                | GameEvent::ActionRecoveryCompleted { entity }
                if *entity == actor
        )));
    }

    #[test]
    fn multi_ut_player_movement_advances_each_materialized_zone_once_per_unit() {
        let mut world = world();
        outward(&mut world);
        home(&mut world);
        let hindered = id("hindered");
        let definition = StatusDefinition::new(
            hindered.clone(),
            Some(2),
            StatusStacking::KeepExisting,
            Vec::new(),
        )
        .unwrap()
        .with_modifiers([StatusModifier::MovementTimeMinimum { time_units: 2 }])
        .unwrap();
        world
            .active
            .rules
            .statuses
            .register(definition.clone())
            .unwrap();
        world
            .active
            .actors
            .get_mut(world.active.player)
            .unwrap()
            .apply_status(&definition, 1, None);
        let remote = world.inactive[&id("b")]
            .actors
            .iter()
            .find_map(|(entity, actor)| (actor.ai().is_some()).then_some(entity))
            .unwrap();
        world
            .inactive
            .get_mut(&id("b"))
            .unwrap()
            .actors
            .get_mut(remote)
            .unwrap()
            .apply_status(&definition, 1, None);
        let before = world.turn();

        assert_eq!(
            world.process_player_command(GameCommand::Move(Direction::East)),
            CommandOutcome::Applied
        );
        assert_eq!(world.turn(), before + 2);
        assert!(
            world
                .actors()
                .get(world.player_id())
                .unwrap()
                .status(&hindered)
                .is_none()
        );
        assert!(
            world.inactive[&id("b")]
                .actors
                .get(remote)
                .unwrap()
                .status(&hindered)
                .is_none()
        );
    }

    #[test]
    fn zones_keep_doors_loot_player_and_global_ids_on_round_trips() {
        let mut world = world();
        let player = world.player_id();
        let initial_enemy = world.actors().iter().map(|(id, _)| id).max().unwrap();
        apply(
            &mut world,
            GameCommand::Interact {
                target: GridPos::new(1, 2),
            },
        );
        let explored: Vec<_> = world.player_visibility().explored_positions().collect();
        outward(&mut world);
        assert_eq!(world.current_zone().unwrap().depth, 1);
        assert_eq!(world.player_id(), player);
        assert!(
            world
                .actors()
                .iter()
                .all(|(id, _)| id == player || id > initial_enemy)
        );
        apply(&mut world, GameCommand::Move(Direction::East));
        apply(&mut world, GameCommand::PickUp);
        let item = world.player_inventory().iter().next().unwrap().instance();
        home(&mut world);
        assert_eq!(world.current_zone().unwrap().id, id("a"));
        assert_eq!(
            world.map().tile(GridPos::new(1, 2)).unwrap().terrain,
            Terrain::Door(DoorState::Open)
        );
        assert!(
            explored
                .iter()
                .all(|p| world.player_visibility().is_explored(*p))
        );
        outward(&mut world);
        assert!(world.ground_items().item_at(GridPos::new(2, 1)).is_none());
        assert!(world.player_inventory().get(item).is_some());
        assert_eq!(world.visited_zone_count(), 2);
        assert_eq!(world.turn(), 6);
    }

    #[test]
    fn following_companion_travels_with_the_player_between_zones() {
        let mut world = world();
        let profile = DroneProfile::new(
            id("travelling_companion"),
            6,
            50,
            40,
            1,
            3,
            4,
            1,
            1,
            DroneCapabilities::default(),
        )
        .unwrap();
        let player = world.player_id();
        let mut state =
            DroneState::new(profile, player, 10, 10, GridPos::new(1, 3), world.turn()).unwrap();
        state.replace_order(DroneOrder::Companion {
            controller: player,
            behavior: CompanionBehavior::Follow,
        });
        let drone = world
            .spawn_actor(
                Actor::new(GridPos::new(1, 3), 10)
                    .unwrap()
                    .with_drone(state),
            )
            .unwrap();
        assert!(matches!(
            world
                .actors()
                .get(drone)
                .and_then(Actor::drone)
                .map(|drone| drone.order()),
            Some(DroneOrder::Companion {
                behavior: CompanionBehavior::Follow,
                ..
            })
        ));

        outward(&mut world);

        assert_eq!(world.current_zone().unwrap().id, id("b"));
        assert!(world.actors().get(drone).is_some());
        assert!(world.inactive[&id("a")].actors.get(drone).is_none());
        assert!(world.player_companion_is_linked(drone));
    }

    #[test]
    fn deferred_zone_is_generated_only_at_an_interactable_first_passage() {
        let game = GameState::new(map(), GridPos::new(1, 1), 42).unwrap();
        let mut world = WorldState::single(game);
        world.enable(info("a", 0)).unwrap();
        let blueprint = ZoneBlueprint {
            info: info("b", 1),
            map: map(),
            entrance: GridPos::new(1, 1),
            seed: 888,
            actors: vec![],
            loot: vec![],
            threat_sources: vec![],
        };
        world
            .declare_deferred_connection(id("a"), GridPos::new(2, 1), blueprint.info.clone())
            .unwrap();

        assert!(world.pending.is_empty());
        assert!(world.inactive.is_empty());
        assert_eq!(world.visited_zone_count(), 1);
        assert_eq!(
            world.deferred_passage_destination(GridPos::new(2, 1)),
            Some(&id("b"))
        );
        assert!(world.can_materialize_passage(GridPos::new(2, 1)));

        world
            .materialize_passage_destination(GridPos::new(2, 1), blueprint)
            .unwrap();
        assert!(world.pending.contains_key(&id("b")));
        assert_eq!(world.visited_zone_count(), 1);
        assert!(
            world
                .passage(GridPos::new(2, 1))
                .is_some_and(|link| link.arrival == Some(GridPos::new(1, 1)))
        );

        apply(
            &mut world,
            GameCommand::Interact {
                target: GridPos::new(2, 1),
            },
        );
        assert_eq!(world.current_zone().unwrap().id, id("b"));
        assert_eq!(world.visited_zone_count(), 2);
        assert!(world.pending.is_empty());
        apply(
            &mut world,
            GameCommand::Interact {
                target: GridPos::new(1, 1),
            },
        );
        assert_eq!(world.current_zone().unwrap().id, id("a"));
    }

    #[test]
    fn known_atlas_arrival_can_materialize_lazily_and_be_declared_idempotently() {
        let game = GameState::new(map(), GridPos::new(1, 1), 42).unwrap();
        let mut world = WorldState::single(game);
        world.enable(info("a", 0)).unwrap();
        let destination = info("b", 0);
        for _ in 0..2 {
            world
                .declare_deferred_connection_at(
                    id("a"),
                    GridPos::new(2, 1),
                    destination.clone(),
                    GridPos::new(1, 1),
                )
                .unwrap();
        }

        assert!(
            world
                .deferred_passage_destination(GridPos::new(2, 1))
                .is_none()
        );
        assert_eq!(
            world.unmaterialized_passage_destination(GridPos::new(2, 1)),
            Some(&id("b"))
        );
        assert!(world.can_materialize_passage(GridPos::new(2, 1)));

        world
            .materialize_passage_destination(
                GridPos::new(2, 1),
                ZoneBlueprint {
                    info: destination,
                    map: map(),
                    entrance: GridPos::new(1, 1),
                    seed: 888,
                    actors: vec![],
                    loot: vec![],
                    threat_sources: vec![],
                },
            )
            .unwrap();
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(2, 1),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.current_zone().unwrap().id, id("b"));
        assert_eq!(world.player_position(), Some(GridPos::new(1, 1)));
    }

    #[test]
    fn rejected_deferred_materialization_is_atomic() {
        let game = GameState::new(map(), GridPos::new(1, 1), 42).unwrap();
        let mut world = WorldState::single(game);
        world.enable(info("a", 0)).unwrap();
        world
            .declare_deferred_connection(id("a"), GridPos::new(5, 1), info("b", 1))
            .unwrap();
        let before = format!("{world:?}");
        let blueprint = ZoneBlueprint {
            info: info("b", 1),
            map: map(),
            entrance: GridPos::new(1, 1),
            seed: 888,
            actors: vec![],
            loot: vec![],
            threat_sources: vec![],
        };

        assert!(!world.can_materialize_passage(GridPos::new(5, 1)));
        assert!(
            world
                .materialize_passage_destination(GridPos::new(5, 1), blueprint)
                .is_err()
        );
        assert_eq!(format!("{world:?}"), before);
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(5, 1),
            }),
            CommandOutcome::Rejected(CommandRejection::InteractionOutOfReach)
        );
        assert_eq!(format!("{world:?}"), before);
    }

    #[test]
    fn invalid_generated_outgoing_connection_rejects_the_whole_zone_atomically() {
        let game = GameState::new(map(), GridPos::new(1, 1), 42).unwrap();
        let mut world = WorldState::single(game);
        world.enable(info("a", 0)).unwrap();
        world
            .declare_deferred_connection_at(
                id("a"),
                GridPos::new(2, 1),
                info("b", 0),
                GridPos::new(1, 1),
            )
            .unwrap();
        let before = format!("{world:?}");

        assert!(
            world
                .materialize_passage_destination_with_connections(
                    GridPos::new(2, 1),
                    ZoneBlueprint {
                        info: info("b", 0),
                        map: map(),
                        entrance: GridPos::new(1, 1),
                        seed: 888,
                        actors: vec![],
                        loot: vec![],
                        threat_sources: vec![],
                    },
                    &[ZoneConnectionBlueprint {
                        at: GridPos::new(0, 0),
                        destination: info("c", 0),
                        arrival: GridPos::new(1, 1),
                    }],
                )
                .is_err()
        );
        assert_eq!(format!("{world:?}"), before);
        assert!(world.pending.is_empty());
        assert!(!world.information.contains_key(&id("c")));
    }

    #[test]
    fn bounded_threat_source_renews_offscreen_without_leaking_spawn_information() {
        let mut world = world();
        world
            .pending
            .get_mut(&id("b"))
            .unwrap()
            .threat_sources
            .push(ThreatSourceBlueprint {
                position: GridPos::new(6, 5),
                interval_turns: std::num::NonZeroU16::new(2).unwrap(),
                maximum_active: std::num::NonZeroU16::new(1).unwrap(),
                maximum_total: std::num::NonZeroU16::new(1).unwrap(),
                actor: Actor::new(GridPos::new(6, 5), 4)
                    .unwrap()
                    .with_ai(AiProfile::idle())
                    .with_defeat_reward(crate::progression::DefeatReward::summoned(0, 0)),
            });

        outward(&mut world);
        home(&mut world);

        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert!(
            !world
                .events()
                .iter()
                .any(|event| matches!(event, GameEvent::EntitySpawned { .. }))
        );
        world.drain_events();
        for _ in 0..3 {
            apply(&mut world, GameCommand::Wait);
        }
        outward(&mut world);

        assert_eq!(world.threat_sources()[0].spawned_total(), 1);
        assert_eq!(
            world
                .actors()
                .iter()
                .filter(|(_, actor)| actor.threat_source() == Some(0))
                .count(),
            1
        );
    }

    #[test]
    fn turn_start_status_advances_in_an_inactive_zone_without_leaking_events() {
        let mut world = world();
        let status = StatusDefinition::new(
            id("background_start_decay"),
            None,
            StatusStacking::Replace,
            vec![StatusHook::new(
                StatusTrigger::TurnStart,
                vec![StatusEffectPrimitive::DealDamage {
                    packet: DamagePacket::new(4, DamageType::Chemical, 0),
                    multiply_by_stacks: false,
                }],
            )],
        )
        .unwrap();
        world
            .active
            .rules
            .statuses
            .register(status.clone())
            .unwrap();
        let enemy = world
            .actors()
            .iter()
            .find(|(id, _)| *id != world.player_id())
            .unwrap()
            .0;
        world
            .active
            .actors
            .get_mut(enemy)
            .unwrap()
            .apply_status(&status, 1, None);

        outward(&mut world);

        assert_eq!(
            world.inactive[&id("a")]
                .actors
                .get(enemy)
                .map(Actor::integrity),
            Some(6)
        );
        assert!(!world.events().iter().any(|event| matches!(
            event,
            GameEvent::StatusTriggered {
                target,
                trigger: StatusTrigger::TurnStart,
                ..
            } if *target == enemy
        )));
    }

    #[test]
    fn offscreen_ai_statuses_and_deaths_evolve_without_visual_information() {
        let mut world = world();
        let status = StatusDefinition::new(
            id("decay"),
            Some(3),
            StatusStacking::Replace,
            vec![StatusHook::new(
                StatusTrigger::TurnEnd,
                vec![StatusEffectPrimitive::DealDamage {
                    packet: DamagePacket::new(4, DamageType::Chemical, 0),
                    multiply_by_stacks: false,
                }],
            )],
        )
        .unwrap();
        world
            .active
            .rules
            .statuses
            .register(status.clone())
            .unwrap();
        let enemy = world
            .actors()
            .iter()
            .find(|(id, _)| *id != world.player_id())
            .unwrap()
            .0;
        world
            .active
            .actors
            .get_mut(enemy)
            .unwrap()
            .apply_status(&status, 1, None);
        outward(&mut world); // A first advances without the player.
        assert_eq!(
            world.inactive[&id("a")]
                .actors
                .get(enemy)
                .unwrap()
                .integrity(),
            6
        );
        assert!(
            !world.inactive[&id("a")]
                .visibility
                .visible_positions()
                .any(|_| true)
        );
        let first_rng = world.inactive[&id("a")].rng.state();
        apply(&mut world, GameCommand::Wait);
        assert_eq!(
            world.inactive[&id("a")]
                .actors
                .get(enemy)
                .unwrap()
                .integrity(),
            2
        );
        assert_ne!(world.inactive[&id("a")].rng.state(), first_rng);
        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert!(world.inactive[&id("a")].actors.get(enemy).is_none());
        assert!(!world.events().iter().any(|event| matches!(event, GameEvent::EntityDied { entity, .. } | GameEvent::EntityMoved { entity, .. } if *entity == enemy)));
        world.drain_events();
        home(&mut world);
        assert!(world.actors().get(enemy).is_none());
        outward(&mut world);
        home(&mut world);
        assert!(world.actors().get(enemy).is_none());
    }

    #[test]
    fn offscreen_leashed_actor_returns_to_its_home() {
        let mut world = world();
        let enemy = world
            .actors()
            .iter()
            .find(|(id, _)| *id != world.player_id())
            .unwrap()
            .0;
        let home = GridPos::new(7, 5);
        let profile = AiProfile::hunter(8, 0).with_maximum_pursuit_distance(
            std::num::NonZeroU16::new(4).expect("constant is non-zero"),
        );
        let actor = world.active.actors.remove(enemy).unwrap().with_ai(profile);
        world.active.actors.insert_existing(enemy, actor);
        world
            .active
            .actors
            .move_to(enemy, GridPos::new(5, 5))
            .unwrap();

        outward(&mut world);
        assert_eq!(
            world.inactive[&id("a")]
                .actors
                .get(enemy)
                .unwrap()
                .position(),
            GridPos::new(6, 5)
        );
        apply(&mut world, GameCommand::Wait);
        assert_eq!(
            world.inactive[&id("a")]
                .actors
                .get(enemy)
                .unwrap()
                .position(),
            home
        );
    }

    #[test]
    fn offscreen_alarm_responder_follows_only_its_recorded_incident() {
        let mut world = world();
        let enemy = world
            .actors()
            .iter()
            .find(|(id, _)| *id != world.player_id())
            .unwrap()
            .0;
        let profile = AiProfile::hunter(8, 0)
            .with_maximum_pursuit_distance(std::num::NonZeroU16::new(4).unwrap())
            .with_pursuit_lifecycle(crate::ai::PursuitLifecycle::new(
                std::num::NonZeroU16::new(6).unwrap(),
                std::num::NonZeroU16::new(2).unwrap(),
                std::num::NonZeroU16::new(2).unwrap(),
            ));
        let mut actor = world.active.actors.remove(enemy).unwrap().with_ai(profile);
        actor.set_ai_state(AiState::Responding {
            remaining_turns: 6,
            incident: GridPos::new(4, 5),
        });
        world.active.actors.insert_existing(enemy, actor);

        outward(&mut world);
        assert_eq!(
            world.inactive[&id("a")]
                .actors
                .get(enemy)
                .unwrap()
                .position(),
            GridPos::new(6, 5)
        );
        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.inactive[&id("a")]
                .actors
                .get(enemy)
                .unwrap()
                .position(),
            GridPos::new(5, 5)
        );
        assert!(!world.events().iter().any(
            |event| matches!(event, GameEvent::EntityMoved { entity, .. } if *entity == enemy)
        ));
    }

    #[test]
    fn ground_effects_remain_with_their_zone_and_expire_offscreen() {
        let mut world = world();
        let position = GridPos::new(3, 3);
        let fire = GroundEffectSpec::new(
            id("ground_fire"),
            2,
            DamagePacket::new(1, DamageType::Thermal, 0),
        )
        .unwrap();
        let current_turn = world.turn();
        world
            .active
            .ground_effects
            .apply(position, None, &fire, current_turn);

        outward(&mut world);
        assert_eq!(
            world.inactive[&id("a")]
                .ground_effects
                .at(position)
                .next()
                .map(|effect| effect.remaining_turns()),
            Some(2)
        );
        apply(&mut world, GameCommand::Wait);
        assert_eq!(
            world.inactive[&id("a")]
                .ground_effects
                .at(position)
                .next()
                .map(|effect| effect.remaining_turns()),
            Some(1)
        );
        apply(&mut world, GameCommand::Wait);
        assert!(world.inactive[&id("a")].ground_effects.is_empty());
    }

    #[test]
    fn data_terminal_interaction_costs_time_and_discovers_each_record_only_once() {
        let game = GameState::new(map(), GridPos::new(1, 1), 42).unwrap();
        let mut world = WorldState::single(game);
        world.enable(info("a", 0)).unwrap();
        let terminal = id("terminal");
        let second_terminal = id("second_terminal");
        let record = id("record");
        world
            .register_facility(
                id("a"),
                FacilityBlueprint {
                    installations: vec![
                        InstallationBlueprint {
                            id: terminal.clone(),
                            position: GridPos::new(2, 1),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::DataTerminal {
                                record: record.clone(),
                            }],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                        InstallationBlueprint {
                            id: second_terminal.clone(),
                            position: GridPos::new(1, 2),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::DataTerminal {
                                record: record.clone(),
                            }],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                        InstallationBlueprint {
                            id: id("depot"),
                            position: GridPos::new(3, 1),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::Storage],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                    ],
                    depot: id("depot"),
                    workers: vec![],
                    repair_orders: vec![],
                    maximum_path_search: 256,
                    owner: None,
                },
            )
            .unwrap();
        world.drain_events();

        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(2, 1),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.turn(), 1);
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::Facility(FacilityEvent::DataTerminalAccessed {
                installation,
                record: event_record,
                first_access: true,
                ..
            }) if installation == &terminal && event_record == &record
        )));
        assert!(
            world
                .active_facility()
                .unwrap()
                .data_terminal_was_accessed(&terminal)
        );
        assert_eq!(
            world.discovered_data_terminal_records(),
            BTreeSet::from([record.clone()])
        );
        world.drain_events();

        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(1, 2),
            }),
            CommandOutcome::Applied
        );
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::Facility(FacilityEvent::DataTerminalAccessed {
                installation,
                record: event_record,
                first_access: false,
                ..
            }) if installation == &second_terminal && event_record == &record
        )));
        assert!(
            world
                .active_facility()
                .unwrap()
                .data_terminal_was_accessed(&second_terminal)
        );
        assert_eq!(world.discovered_data_terminal_records().len(), 1);
    }

    #[test]
    fn data_terminal_discovery_is_shared_between_active_and_offscreen_facilities() {
        let mut world = world();
        let record = id("shared_record");
        let facility = |terminal: ContentId, terminal_position: GridPos| FacilityBlueprint {
            installations: vec![
                InstallationBlueprint {
                    id: terminal,
                    position: terminal_position,
                    maximum_integrity: 10,
                    integrity: 10,
                    capabilities: vec![InstallationCapability::DataTerminal {
                        record: record.clone(),
                    }],
                    dependencies: vec![],
                    security_alarm_profile: None,
                },
                InstallationBlueprint {
                    id: id(&format!(
                        "depot_{}_{}",
                        terminal_position.x, terminal_position.y
                    )),
                    position: GridPos::new(4, 1),
                    maximum_integrity: 10,
                    integrity: 10,
                    capabilities: vec![InstallationCapability::Storage],
                    dependencies: vec![],
                    security_alarm_profile: None,
                },
            ],
            depot: id(&format!(
                "depot_{}_{}",
                terminal_position.x, terminal_position.y
            )),
            workers: vec![],
            repair_orders: vec![],
            maximum_path_search: 256,
            owner: None,
        };
        let first_terminal = id("terminal_a");
        let second_terminal = id("terminal_b");
        world
            .register_facility(
                id("a"),
                facility(first_terminal.clone(), GridPos::new(3, 1)),
            )
            .unwrap();
        world
            .register_facility(
                id("b"),
                facility(second_terminal.clone(), GridPos::new(1, 2)),
            )
            .unwrap();

        apply(&mut world, GameCommand::Move(Direction::East));
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(3, 1),
            }),
            CommandOutcome::Applied
        );
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::Facility(FacilityEvent::DataTerminalAccessed {
                installation,
                first_access: true,
                ..
            }) if installation == &first_terminal
        )));
        world.drain_events();

        outward(&mut world);
        assert_eq!(world.current_zone().unwrap().id, id("b"));
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(1, 2),
            }),
            CommandOutcome::Applied
        );
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::Facility(FacilityEvent::DataTerminalAccessed {
                installation,
                record: event_record,
                first_access: false,
                ..
            }) if installation == &second_terminal && event_record == &record
        )));
        assert_eq!(
            world.discovered_data_terminal_records(),
            BTreeSet::from([record])
        );
        assert!(
            world
                .facility_in_zone(&id("a"))
                .unwrap()
                .data_terminal_was_accessed(&first_terminal)
        );
    }

    #[test]
    fn facility_repairs_continue_offscreen_without_leaking_events() {
        let mut world = world();
        world
            .active
            .map
            .set_terrain(GridPos::new(4, 3), Terrain::Door(DoorState::Closed))
            .unwrap();
        for position in [GridPos::new(1, 5), GridPos::new(2, 5)] {
            world
                .spawn_actor(Actor::new(position, 10).unwrap().with_ai(AiProfile::idle()))
                .unwrap();
        }
        world
            .spawn_ground_item(GridPos::new(3, 5), id("repair"), 1)
            .unwrap();
        world
            .register_facility(
                id("a"),
                FacilityBlueprint {
                    installations: vec![
                        InstallationBlueprint {
                            id: id("relay"),
                            position: GridPos::new(6, 5),
                            maximum_integrity: 10,
                            integrity: 0,
                            capabilities: vec![InstallationCapability::PowerRelay],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                        InstallationBlueprint {
                            id: id("actuator"),
                            position: GridPos::new(7, 3),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::DoorActuator {
                                door: GridPos::new(4, 3),
                            }],
                            dependencies: vec![id("relay")],
                            security_alarm_profile: None,
                        },
                        InstallationBlueprint {
                            id: id("depot"),
                            position: GridPos::new(6, 1),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::Storage],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                    ],
                    depot: id("depot"),
                    workers: vec![
                        WorkerBlueprint {
                            actor_position: GridPos::new(1, 5),
                            role: WorkerRole::Retriever,
                            maximum_integrity: 10,
                            affiliation: None,
                            witness_profile: None,
                            local_alert_profile: None,
                            property_report: None,
                            installed_property_report: None,
                            reported_incident_response: None,
                        },
                        WorkerBlueprint {
                            actor_position: GridPos::new(2, 5),
                            role: WorkerRole::Technician,
                            maximum_integrity: 10,
                            affiliation: None,
                            witness_profile: None,
                            local_alert_profile: None,
                            property_report: None,
                            installed_property_report: None,
                            reported_incident_response: None,
                        },
                    ],
                    repair_orders: vec![RepairOrderBlueprint {
                        id: id("restore"),
                        target: id("relay"),
                        required_item: id("repair"),
                        required_quantity: 1,
                        work_turns: 2,
                    }],
                    maximum_path_search: 256,
                    owner: None,
                },
            )
            .unwrap();
        world.drain_events();

        outward(&mut world);
        for _ in 0..24 {
            assert_eq!(
                world.process_player_command(GameCommand::Wait),
                CommandOutcome::Applied
            );
            assert!(
                !world
                    .events()
                    .iter()
                    .any(|event| matches!(event, GameEvent::Facility(_)))
            );
            world.drain_events();
        }

        let facility = world.facility_in_zone(&id("a")).unwrap();
        assert_eq!(
            facility.repair_status(&id("restore")),
            Some(RepairStatus::Completed)
        );
        assert!(facility.is_operational(&id("actuator")));
        assert_eq!(
            world.inactive[&id("a")]
                .map
                .tile(GridPos::new(4, 3))
                .unwrap()
                .terrain,
            Terrain::Door(DoorState::Closed)
        );
    }

    #[test]
    fn direct_property_report_starts_a_fixed_local_worker_investigation() {
        let mut world = world();
        let incident = world.active.player_position().unwrap();
        world
            .active
            .map
            .set_terrain(GridPos::new(1, 2), Terrain::Floor)
            .unwrap();
        let owner: SocialGroupId = id("collective");
        let reporter_position = GridPos::new(2, 1);
        let recipient_position = GridPos::new(6, 1);
        let reporter = world
            .spawn_actor(
                Actor::new(reporter_position, 10)
                    .unwrap()
                    .with_ai(AiProfile::idle())
                    .with_affiliation(owner.clone())
                    .with_witness_profile(
                        WitnessProfile::new(8, DistanceMetric::Euclidean, true, 4).unwrap(),
                    )
                    .with_local_alert_profile(LocalAlertProfile::new(8).unwrap()),
            )
            .unwrap();
        let recipient = world
            .spawn_actor(
                Actor::new(recipient_position, 10)
                    .unwrap()
                    .with_ai(AiProfile::idle())
                    .with_affiliation(owner.clone())
                    .with_local_alert_profile(LocalAlertProfile::new(8).unwrap()),
            )
            .unwrap();
        world
            .active
            .spawn_ground_item_with_owner(incident, id("repair"), 1, Some(owner.clone()))
            .unwrap();
        world
            .register_facility(
                id("a"),
                FacilityBlueprint {
                    installations: vec![InstallationBlueprint {
                        id: id("response_depot"),
                        position: GridPos::new(7, 4),
                        maximum_integrity: 10,
                        integrity: 10,
                        capabilities: vec![InstallationCapability::Storage],
                        dependencies: vec![],
                        security_alarm_profile: None,
                    }],
                    depot: id("response_depot"),
                    workers: vec![
                        WorkerBlueprint {
                            actor_position: reporter_position,
                            role: WorkerRole::Retriever,
                            maximum_integrity: 10,
                            affiliation: Some(owner.clone()),
                            witness_profile: Some(
                                WitnessProfile::new(8, DistanceMetric::Euclidean, true, 4).unwrap(),
                            ),
                            local_alert_profile: Some(LocalAlertProfile::new(8).unwrap()),
                            property_report: Some(WorkerPropertyReportBlueprint {
                                recipient_position,
                                channel: PropertyReportChannel::new(
                                    8,
                                    DistanceMetric::Euclidean,
                                    true,
                                )
                                .unwrap(),
                            }),
                            installed_property_report: None,
                            reported_incident_response: None,
                        },
                        WorkerBlueprint {
                            actor_position: recipient_position,
                            role: WorkerRole::Technician,
                            maximum_integrity: 10,
                            affiliation: Some(owner),
                            witness_profile: None,
                            local_alert_profile: Some(LocalAlertProfile::new(8).unwrap()),
                            property_report: None,
                            installed_property_report: None,
                            reported_incident_response: Some(
                                WorkerReportedIncidentResponseBlueprint {
                                    inspection_turns: 2,
                                    maximum_response_turns: 12,
                                },
                            ),
                        },
                    ],
                    repair_orders: vec![],
                    maximum_path_search: 256,
                    owner: None,
                },
            )
            .unwrap();
        world.drain_events();

        assert_eq!(
            world.process_player_command(GameCommand::PickUp),
            CommandOutcome::Applied
        );
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::PropertyTakeReported {
                source,
                recipient: target,
                at,
                ..
            } if *source == reporter && *target == recipient && *at == incident
        )));
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::Facility(FacilityEvent::ReportedIncidentInvestigationAssigned {
                worker,
                at,
                ..
            }) if *worker == recipient && *at == incident
        )));

        for _ in 0..4 {
            assert_eq!(
                world.process_player_command(GameCommand::Move(Direction::South)),
                CommandOutcome::Applied
            );
        }
        for _ in 0..4 {
            assert_eq!(
                world.process_player_command(GameCommand::Wait),
                CommandOutcome::Applied
            );
        }
        let player_position = world.active.player_position().unwrap();
        let recipient_position = world.active.actors.get(recipient).unwrap().position();
        assert_eq!(player_position, GridPos::new(1, 5));
        assert!(
            recipient_position == incident
                || incident.cardinal_neighbors().contains(&recipient_position),
            "the recipient must inspect the recorded incident rather than follow the player"
        );
    }

    #[test]
    fn alarm_lockdown_expires_offscreen_without_leaking_events() {
        let mut world = world();
        let door = GridPos::new(4, 3);
        world
            .active
            .map
            .set_terrain(door, Terrain::Door(DoorState::Closed))
            .unwrap();
        world
            .spawn_ground_item_with_owner(
                GridPos::new(1, 1),
                id("repair"),
                1,
                Some(id("collective")),
            )
            .unwrap();
        world
            .register_facility(
                id("a"),
                FacilityBlueprint {
                    installations: vec![
                        InstallationBlueprint {
                            id: id("actuator"),
                            position: GridPos::new(6, 4),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::DoorActuator { door }],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                        InstallationBlueprint {
                            id: id("sensor"),
                            position: GridPos::new(2, 2),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::SecuritySensor],
                            dependencies: vec![],
                            security_alarm_profile: Some(
                                SecurityAlarmProfile::new(
                                    8,
                                    crate::world::DistanceMetric::Euclidean,
                                    true,
                                    4,
                                )
                                .unwrap()
                                .with_responses(vec![SecurityAlarmResponse::LockDoors {
                                    actuator: id("actuator"),
                                }])
                                .unwrap(),
                            ),
                        },
                        InstallationBlueprint {
                            id: id("depot"),
                            position: GridPos::new(7, 4),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::Storage],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                    ],
                    depot: id("depot"),
                    workers: vec![],
                    repair_orders: vec![],
                    maximum_path_search: 256,
                    owner: Some(id("collective")),
                },
            )
            .unwrap();
        world.drain_events();

        assert_eq!(
            world.process_player_command(GameCommand::PickUp),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.map().tile(door).unwrap().terrain,
            Terrain::Door(DoorState::Locked)
        );
        world.drain_events();
        outward(&mut world);
        for _ in 0..3 {
            assert_eq!(
                world.process_player_command(GameCommand::Wait),
                CommandOutcome::Applied
            );
            assert!(
                !world
                    .events()
                    .iter()
                    .any(|event| matches!(event, GameEvent::Facility(_)))
            );
            world.drain_events();
        }
        assert_eq!(
            world.inactive[&id("a")].map.tile(door).unwrap().terrain,
            Terrain::Door(DoorState::Closed)
        );
        assert!(
            world
                .facility_in_zone(&id("a"))
                .unwrap()
                .security_door_lockdown_at(door, world.turn())
                .is_none()
        );
    }

    #[test]
    fn observed_owned_loot_accelerates_a_real_finite_threat_source() {
        let mut world = world();
        let source = GridPos::new(6, 1);
        world
            .spawn_ground_item_with_owner(
                GridPos::new(1, 1),
                id("repair"),
                1,
                Some(id("collective")),
            )
            .unwrap();
        world
            .active
            .install_threat_sources(vec![ThreatSourceBlueprint {
                position: source,
                interval_turns: std::num::NonZeroU16::new(20).unwrap(),
                maximum_active: std::num::NonZeroU16::new(1).unwrap(),
                maximum_total: std::num::NonZeroU16::new(1).unwrap(),
                actor: Actor::new(source, 4)
                    .unwrap()
                    .with_ai(
                        AiProfile::hunter(1, 0)
                            .with_maximum_pursuit_distance(std::num::NonZeroU16::new(6).unwrap())
                            .with_pursuit_lifecycle(crate::ai::PursuitLifecycle::new(
                                std::num::NonZeroU16::new(6).unwrap(),
                                std::num::NonZeroU16::new(2).unwrap(),
                                std::num::NonZeroU16::new(2).unwrap(),
                            )),
                    )
                    .with_defeat_reward(crate::progression::DefeatReward::summoned(0, 0)),
            }])
            .unwrap();
        world
            .register_facility(
                id("a"),
                FacilityBlueprint {
                    installations: vec![
                        InstallationBlueprint {
                            id: id("sensor"),
                            position: GridPos::new(2, 2),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::SecuritySensor],
                            dependencies: vec![],
                            security_alarm_profile: Some(
                                SecurityAlarmProfile::new(
                                    8,
                                    crate::world::DistanceMetric::Euclidean,
                                    true,
                                    8,
                                )
                                .unwrap()
                                .with_responses(vec![
                                    SecurityAlarmResponse::CallInvestigatingReinforcements {
                                        source,
                                        delay_turns: 3,
                                    },
                                ])
                                .unwrap(),
                            ),
                        },
                        InstallationBlueprint {
                            id: id("depot"),
                            position: GridPos::new(7, 4),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::Storage],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                    ],
                    depot: id("depot"),
                    workers: vec![],
                    repair_orders: vec![],
                    maximum_path_search: 256,
                    owner: Some(id("collective")),
                },
            )
            .unwrap();
        world.drain_events();

        assert_eq!(
            world.process_player_command(GameCommand::PickUp),
            CommandOutcome::Applied
        );
        assert_eq!(world.threat_sources()[0].remaining_turns(), 3);
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::Facility(FacilityEvent::InvestigatingReinforcementsRequested {
                source: event_source,
                incident: GridPos { x: 1, y: 1 },
                delay_turns: 3,
                ..
            }) if *event_source == source
        )));
        world.drain_events();
        for _ in 0..3 {
            apply(&mut world, GameCommand::Wait);
        }
        assert_eq!(world.threat_sources()[0].spawned_total(), 1);
        assert_eq!(
            world
                .actors()
                .iter()
                .filter(|(_, actor)| actor.threat_source() == Some(0))
                .count(),
            1
        );
        assert!(world.actors().iter().any(|(_, actor)| matches!(
            actor.ai_state(),
            AiState::Responding {
                incident: GridPos { x: 1, y: 1 },
                ..
            }
        )));
    }

    #[test]
    fn installed_witness_report_mobilizes_one_finite_investigating_responder() {
        let mut world = world();
        let owner: SocialGroupId = id("collective");
        let reporter_position = GridPos::new(2, 1);
        let sensor_position = GridPos::new(5, 1);
        let source = GridPos::new(6, 4);
        let reporter = world
            .spawn_actor(
                Actor::new(reporter_position, 10)
                    .unwrap()
                    .with_ai(AiProfile::idle())
                    .with_affiliation(owner.clone())
                    .with_witness_profile(
                        WitnessProfile::new(8, DistanceMetric::Euclidean, true, 4).unwrap(),
                    ),
            )
            .unwrap();
        world
            .spawn_ground_item_with_owner(GridPos::new(1, 1), id("repair"), 1, Some(owner.clone()))
            .unwrap();
        world
            .active
            .install_threat_sources(vec![ThreatSourceBlueprint {
                position: source,
                interval_turns: std::num::NonZeroU16::new(20).unwrap(),
                maximum_active: std::num::NonZeroU16::new(1).unwrap(),
                maximum_total: std::num::NonZeroU16::new(1).unwrap(),
                actor: Actor::new(source, 4)
                    .unwrap()
                    .with_ai(
                        AiProfile::hunter(1, 0)
                            .with_maximum_pursuit_distance(std::num::NonZeroU16::new(8).unwrap())
                            .with_pursuit_lifecycle(crate::ai::PursuitLifecycle::new(
                                std::num::NonZeroU16::new(6).unwrap(),
                                std::num::NonZeroU16::new(2).unwrap(),
                                std::num::NonZeroU16::new(2).unwrap(),
                            )),
                    )
                    .with_defeat_reward(crate::progression::DefeatReward::summoned(0, 0)),
            }])
            .unwrap();
        world
            .register_facility(
                id("a"),
                FacilityBlueprint {
                    installations: vec![
                        InstallationBlueprint {
                            id: id("reported_sensor"),
                            position: sensor_position,
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::SecuritySensor],
                            dependencies: vec![],
                            security_alarm_profile: Some(
                                SecurityAlarmProfile::new(1, DistanceMetric::Euclidean, true, 8)
                                    .unwrap()
                                    .with_responses(vec![
                                        SecurityAlarmResponse::CallInvestigatingReinforcements {
                                            source,
                                            delay_turns: 3,
                                        },
                                    ])
                                    .unwrap(),
                            ),
                        },
                        InstallationBlueprint {
                            id: id("reported_depot"),
                            position: GridPos::new(7, 4),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::Storage],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                    ],
                    depot: id("reported_depot"),
                    workers: vec![WorkerBlueprint {
                        actor_position: reporter_position,
                        role: WorkerRole::Retriever,
                        maximum_integrity: 10,
                        affiliation: Some(owner.clone()),
                        witness_profile: Some(
                            WitnessProfile::new(8, DistanceMetric::Euclidean, true, 4).unwrap(),
                        ),
                        local_alert_profile: None,
                        property_report: None,
                        installed_property_report: Some(WorkerInstalledPropertyReportBlueprint {
                            installation: id("reported_sensor"),
                            channel: PropertyReportChannel::new(8, DistanceMetric::Euclidean, true)
                                .unwrap(),
                        }),
                        reported_incident_response: None,
                    }],
                    repair_orders: vec![],
                    maximum_path_search: 256,
                    owner: Some(owner),
                },
            )
            .unwrap();
        world.drain_events();

        assert_eq!(
            world.process_player_command(GameCommand::PickUp),
            CommandOutcome::Applied
        );
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::Facility(FacilityEvent::InstalledPropertyReportReceived {
                source,
                installation,
                at: GridPos { x: 1, y: 1 },
            }) if *source == reporter && installation == &id("reported_sensor")
        )));
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::Facility(FacilityEvent::InvestigatingReinforcementsRequested {
                source: event_source,
                incident: GridPos { x: 1, y: 1 },
                delay_turns: 3,
                ..
            }) if *event_source == source
        )));
        assert_eq!(world.threat_sources()[0].remaining_turns(), 3);

        world.drain_events();
        for _ in 0..3 {
            apply(&mut world, GameCommand::Wait);
        }
        assert_eq!(world.threat_sources()[0].spawned_total(), 1);
        assert!(world.actors().iter().any(|(_, actor)| matches!(
            actor.ai_state(),
            AiState::Responding {
                incident: GridPos { x: 1, y: 1 },
                ..
            }
        )));
    }

    #[test]
    fn investigating_alarm_rejects_a_source_without_compatible_mobile_ai() {
        let mut world = world();
        let source = GridPos::new(6, 1);
        world
            .active
            .install_threat_sources(vec![ThreatSourceBlueprint {
                position: source,
                interval_turns: std::num::NonZeroU16::new(20).unwrap(),
                maximum_active: std::num::NonZeroU16::new(1).unwrap(),
                maximum_total: std::num::NonZeroU16::new(1).unwrap(),
                actor: Actor::new(source, 4)
                    .unwrap()
                    .with_ai(AiProfile::idle())
                    .with_defeat_reward(crate::progression::DefeatReward::summoned(0, 0)),
            }])
            .unwrap();
        let result = world.register_facility(
            id("a"),
            FacilityBlueprint {
                installations: vec![
                    InstallationBlueprint {
                        id: id("sensor"),
                        position: GridPos::new(2, 2),
                        maximum_integrity: 10,
                        integrity: 10,
                        capabilities: vec![InstallationCapability::SecuritySensor],
                        dependencies: vec![],
                        security_alarm_profile: Some(
                            SecurityAlarmProfile::new(
                                8,
                                crate::world::DistanceMetric::Euclidean,
                                true,
                                8,
                            )
                            .unwrap()
                            .with_responses(vec![
                                SecurityAlarmResponse::CallInvestigatingReinforcements {
                                    source,
                                    delay_turns: 3,
                                },
                            ])
                            .unwrap(),
                        ),
                    },
                    InstallationBlueprint {
                        id: id("depot"),
                        position: GridPos::new(7, 4),
                        maximum_integrity: 10,
                        integrity: 10,
                        capabilities: vec![InstallationCapability::Storage],
                        dependencies: vec![],
                        security_alarm_profile: None,
                    },
                ],
                depot: id("depot"),
                workers: vec![],
                repair_orders: vec![],
                maximum_path_search: 256,
                owner: Some(id("collective")),
            },
        );

        assert!(result.unwrap_err().contains("requires mobile AI"));
        assert!(world.active_facility().is_none());
    }

    #[test]
    fn blocked_and_remote_transitions_are_atomic_and_background_does_not_tick() {
        let mut world = world();
        outward(&mut world);
        world
            .inactive
            .get_mut(&id("a"))
            .unwrap()
            .map
            .set_terrain(GridPos::new(2, 1), Terrain::Wall)
            .unwrap();
        let before = format!("{world:?}");
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(1, 1)
            }),
            CommandOutcome::Rejected(CommandRejection::PassageUnavailable)
        );
        assert_eq!(format!("{world:?}"), before);
        apply(&mut world, GameCommand::Move(Direction::East));
        apply(&mut world, GameCommand::Move(Direction::East));
        let before = format!("{world:?}");
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(1, 1)
            }),
            CommandOutcome::Rejected(CommandRejection::InteractionOutOfReach)
        );
        assert_eq!(format!("{world:?}"), before);
    }

    #[test]
    fn all_world_updates_are_deterministic_and_unvisited_zones_do_not_spawn() {
        let mut a = world();
        let mut b = world();
        for command in [
            GameCommand::Wait,
            GameCommand::Interact {
                target: GridPos::new(2, 1),
            },
            GameCommand::Wait,
            GameCommand::Wait,
            GameCommand::Interact {
                target: GridPos::new(1, 1),
            },
            GameCommand::Wait,
        ] {
            assert_eq!(
                a.process_player_command(command.clone()),
                b.process_player_command(command)
            );
            assert_eq!(format!("{a:?}"), format!("{b:?}"));
            a.drain_events();
            b.drain_events();
        }
        assert_eq!(world().visited_zone_count(), 1);
    }

    #[test]
    fn invalid_connections_and_duplicate_zone_ids_do_not_replace_content() {
        let mut world = world();
        let before = format!("{world:?}");
        assert!(
            world
                .connect(id("a"), GridPos::new(0, 0), id("b"), GridPos::new(1, 2))
                .is_err()
        );
        assert!(
            world
                .connect(id("a"), GridPos::new(2, 1), id("b"), GridPos::new(1, 1))
                .is_err()
        );
        let duplicate = world.pending[&id("b")].clone();
        assert!(world.add_zone(duplicate).is_err());
        assert_eq!(format!("{world:?}"), before);
    }

    #[test]
    fn occupied_arrival_is_rejected_without_a_turn_or_mutation() {
        let mut world = world();
        outward(&mut world);
        let zone = world.inactive.get_mut(&id("a")).unwrap();
        let enemy = zone.actors.iter().next().unwrap().0;
        zone.actors.move_to(enemy, GridPos::new(2, 1)).unwrap();
        let before = format!("{world:?}");
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(1, 1)
            }),
            CommandOutcome::Rejected(CommandRejection::PassageObstructed)
        );
        assert_eq!(format!("{world:?}"), before);
    }

    #[test]
    fn rejected_first_visit_keeps_its_pending_facility_uninstantiated() {
        let mut world = world();
        world
            .register_facility(
                id("b"),
                FacilityBlueprint {
                    installations: vec![InstallationBlueprint {
                        id: id("remote_depot"),
                        position: GridPos::new(3, 3),
                        maximum_integrity: 10,
                        integrity: 10,
                        capabilities: vec![InstallationCapability::Storage],
                        dependencies: vec![],
                        security_alarm_profile: None,
                    }],
                    depot: id("remote_depot"),
                    workers: vec![],
                    repair_orders: vec![],
                    maximum_path_search: 64,
                    owner: None,
                },
            )
            .unwrap();
        world.pending.get_mut(&id("b")).unwrap().actors.push(
            Actor::new(GridPos::new(1, 1), 5)
                .unwrap()
                .with_ai(AiProfile::idle()),
        );
        let before_turn = world.turn();

        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(2, 1),
            }),
            CommandOutcome::Rejected(CommandRejection::PassageObstructed)
        );

        assert_eq!(world.turn(), before_turn);
        assert!(world.pending.contains_key(&id("b")));
        assert!(world.pending_facilities.contains_key(&id("b")));
        assert!(!world.facilities.contains_key(&id("b")));
        assert!(!world.inactive.contains_key(&id("b")));
    }

    #[test]
    fn player_status_ticks_once_and_offscreen_player_kills_award_xp_once() {
        let mut world = world();
        let status = StatusDefinition::new(
            id("decay"),
            Some(3),
            StatusStacking::Replace,
            vec![StatusHook::new(
                StatusTrigger::TurnEnd,
                vec![StatusEffectPrimitive::DealDamage {
                    packet: DamagePacket::new(4, DamageType::Chemical, 0),
                    multiply_by_stacks: false,
                }],
            )],
        )
        .unwrap();
        world
            .active
            .rules
            .statuses
            .register(status.clone())
            .unwrap();
        let player = world.player_id();
        let enemy = world
            .actors()
            .iter()
            .find(|(id, _)| *id != player)
            .unwrap()
            .0;
        let mut actor = world
            .active
            .actors
            .remove(enemy)
            .unwrap()
            .with_defeat_reward(crate::progression::DefeatReward::persistent(12, 1));
        actor.apply_status(&status, 1, Some(player));
        world.active.actors.insert_existing(enemy, actor);
        let actor = world.active.actors.get_mut(player).unwrap();
        let integrity = actor.integrity();
        actor.apply_status(&status, 1, None);
        outward(&mut world);
        assert_eq!(
            world.actors().get(player).unwrap().integrity(),
            integrity - 4
        );
        apply(&mut world, GameCommand::Wait);
        assert_eq!(
            world.actors().get(player).unwrap().integrity(),
            integrity - 8
        );
        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.actors().get(player).unwrap().integrity(),
            integrity - 12
        );
        assert_eq!(
            world
                .events()
                .iter()
                .filter(|e| matches!(e, GameEvent::ExperienceAwarded { .. }))
                .count(),
            1
        );
        world.drain_events();
        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert!(
            !world
                .events()
                .iter()
                .any(|e| matches!(e, GameEvent::ExperienceAwarded { .. }))
        );
    }
}
