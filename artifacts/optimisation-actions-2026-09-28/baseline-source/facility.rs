//! Deterministic local installations, material custody and bounded work.
//!
//! This module deliberately knows nothing about rendering, dialogue, quests or
//! reputation. A content adapter supplies installations, dependencies,
//! workers, ownership, local sensor reactions and work orders; the same state
//! can run in the active zone or off screen.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::content::ContentId;
use crate::entity::{
    ActorRegistry, EntityId, GroundItemId, GroundItemRegistry, Inventory, ItemInstanceId,
};
use crate::item::ItemId;
use crate::social::{
    LocalAlertProfile, ObservedPropertyTake, PropertyReportChannel, PropertyReportProfile,
    SocialGroupId, WitnessProfile,
};
use crate::world::{
    DistanceMetric, DoorState, FieldOfViewRules, GridPos, Map, Terrain, compute_visible_tiles,
    find_path_with,
};

pub type InstallationId = ContentId;
pub type WorkOrderId = ContentId;

pub const MAX_INSTALLATIONS: usize = 256;
pub const MAX_WORKERS: usize = 128;
pub const MAX_WORK_ORDERS: usize = 256;
pub const MAX_PATH_SEARCH: usize = 100_000;
pub const MAX_SECURITY_SENSOR_RANGE: u16 = 64;
pub const MAX_NAVIGATION_BEACON_RANGE: u16 = 1_024;
pub const MAX_SECURITY_ALARM_DURATION_TURNS: u16 = 10_000;
pub const MAX_SECURITY_ALARM_RESPONSES: usize = 32;
pub const MAX_SECURITY_REINFORCEMENT_DELAY_TURNS: u16 = 10_000;
pub const MAX_REPORTED_INCIDENT_RESPONSE_TURNS: u16 = 1_000;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum InstallationCapability {
    PowerRelay,
    DoorActuator {
        door: GridPos,
    },
    SecuritySensor,
    /// Emits an approximate, wall-independent navigation signal while the
    /// installation and all of its dependencies are operational.
    NavigationBeacon {
        range: u16,
    },
    /// Exposes a stable, content-defined record when the installation and all
    /// of its dependencies are operational. Interpretation and localization
    /// remain presentation concerns.
    DataTerminal {
        record: ContentId,
    },
    Storage,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetectedNavigationSignal {
    pub installation: InstallationId,
    pub position: GridPos,
    pub distance: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DataTerminalAccess {
    pub installation: InstallationId,
    pub record: ContentId,
    pub first_access: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataTerminalUpdateError {
    UnknownInstallation,
    MissingDataTerminal,
    UnchangedRecord,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SecurityAlarmResponse {
    /// Locks every door controlled by the referenced actuator. Referencing an
    /// installation ID keeps the rule stable when a map layout moves.
    LockDoors { actuator: InstallationId },
    /// Requests an accelerated spawn from an existing, finite threat source.
    /// The facility never creates an actor itself: the world simulation keeps
    /// the source's active and lifetime quotas authoritative.
    CallReinforcements { source: GridPos, delay_turns: u16 },
    /// Requests the same bounded reinforcement, but also transmits the
    /// recorded incident position. Spawned responders investigate that fixed
    /// location; this never grants the player's live position.
    CallInvestigatingReinforcements { source: GridPos, delay_turns: u16 },
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SecurityAlarmProfile {
    field_of_view: FieldOfViewRules,
    duration_turns: u16,
    responses: Vec<SecurityAlarmResponse>,
}

// Empty response lists retain the v8 catalogue representation exactly.
impl Debug for SecurityAlarmProfile {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut profile = formatter.debug_struct("SecurityAlarmProfile");
        profile
            .field("field_of_view", &self.field_of_view)
            .field("duration_turns", &self.duration_turns);
        if !self.responses.is_empty() {
            profile.field("responses", &self.responses);
        }
        profile.finish()
    }
}

impl SecurityAlarmProfile {
    pub fn new(
        radius: u16,
        distance_metric: DistanceMetric,
        block_closed_corners: bool,
        duration_turns: u16,
    ) -> Result<Self, SecurityAlarmProfileError> {
        if radius == 0 || radius > MAX_SECURITY_SENSOR_RANGE {
            return Err(SecurityAlarmProfileError::InvalidRange(radius));
        }
        if duration_turns == 0 || duration_turns > MAX_SECURITY_ALARM_DURATION_TURNS {
            return Err(SecurityAlarmProfileError::InvalidDuration(duration_turns));
        }
        Ok(Self {
            field_of_view: FieldOfViewRules {
                radius,
                distance_metric,
                block_closed_corners,
            },
            duration_turns,
            responses: Vec::new(),
        })
    }

    pub fn with_responses(
        mut self,
        responses: Vec<SecurityAlarmResponse>,
    ) -> Result<Self, SecurityAlarmProfileError> {
        if responses.len() > MAX_SECURITY_ALARM_RESPONSES {
            return Err(SecurityAlarmProfileError::TooManyResponses(responses.len()));
        }
        if let Some(delay) = responses.iter().find_map(|response| match response {
            SecurityAlarmResponse::CallReinforcements { delay_turns, .. }
            | SecurityAlarmResponse::CallInvestigatingReinforcements { delay_turns, .. }
                if *delay_turns == 0 || *delay_turns > MAX_SECURITY_REINFORCEMENT_DELAY_TURNS =>
            {
                Some(*delay_turns)
            }
            _ => None,
        }) {
            return Err(SecurityAlarmProfileError::InvalidReinforcementDelay(delay));
        }
        self.responses = responses;
        Ok(self)
    }

    pub const fn field_of_view(&self) -> FieldOfViewRules {
        self.field_of_view
    }

    pub const fn duration_turns(&self) -> u16 {
        self.duration_turns
    }

    pub fn responses(&self) -> &[SecurityAlarmResponse] {
        &self.responses
    }

    pub fn without_responses(&self) -> Self {
        let mut profile = self.clone();
        profile.responses.clear();
        profile
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecurityAlarmProfileError {
    InvalidRange(u16),
    InvalidDuration(u16),
    TooManyResponses(usize),
    InvalidReinforcementDelay(u16),
}

impl Display for SecurityAlarmProfileError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRange(value) => write!(
                formatter,
                "security sensor range must be between 1 and {MAX_SECURITY_SENSOR_RANGE}, got {value}"
            ),
            Self::InvalidDuration(value) => write!(
                formatter,
                "security alarm duration must be between 1 and {MAX_SECURITY_ALARM_DURATION_TURNS}, got {value}"
            ),
            Self::TooManyResponses(value) => write!(
                formatter,
                "security alarm response count cannot exceed {MAX_SECURITY_ALARM_RESPONSES}, got {value}"
            ),
            Self::InvalidReinforcementDelay(value) => write!(
                formatter,
                "security reinforcement delay must be between 1 and {MAX_SECURITY_REINFORCEMENT_DELAY_TURNS}, got {value}"
            ),
        }
    }
}

impl Error for SecurityAlarmProfileError {}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct InstallationBlueprint {
    pub id: InstallationId,
    pub position: GridPos,
    pub maximum_integrity: u16,
    pub integrity: u16,
    pub capabilities: Vec<InstallationCapability>,
    pub dependencies: Vec<InstallationId>,
    pub security_alarm_profile: Option<SecurityAlarmProfile>,
}

// New security metadata is omitted when absent so stripped catalogues retain
// the historical representation used by suspension versions 4 to 7.
impl Debug for InstallationBlueprint {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut installation = formatter.debug_struct("InstallationBlueprint");
        installation
            .field("id", &self.id)
            .field("position", &self.position)
            .field("maximum_integrity", &self.maximum_integrity)
            .field("integrity", &self.integrity)
            .field("capabilities", &self.capabilities)
            .field("dependencies", &self.dependencies);
        if let Some(profile) = &self.security_alarm_profile {
            installation.field("security_alarm_profile", &profile);
        }
        installation.finish()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum WorkerRole {
    Retriever,
    Technician,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WorkerPropertyReportBlueprint {
    pub recipient_position: GridPos,
    pub channel: PropertyReportChannel,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WorkerInstalledPropertyReportBlueprint {
    pub installation: InstallationId,
    pub channel: PropertyReportChannel,
}

/// A deliberately local, finite reaction to a fact received from another
/// worker. The facility remains the only movement authority for its workers;
/// this does not turn a service NPC into combat AI or reveal the taker's live
/// position.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WorkerReportedIncidentResponseBlueprint {
    pub inspection_turns: u16,
    pub maximum_response_turns: u16,
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WorkerBlueprint {
    /// The adapter does not know global entity IDs before spawning. A unique
    /// position resolves the already-created actor without storing an index.
    pub actor_position: GridPos,
    pub role: WorkerRole,
    pub maximum_integrity: u16,
    /// Optional actor metadata interpreted by the world, not this subsystem.
    pub affiliation: Option<SocialGroupId>,
    pub witness_profile: Option<WitnessProfile>,
    pub local_alert_profile: Option<LocalAlertProfile>,
    pub property_report: Option<WorkerPropertyReportBlueprint>,
    pub installed_property_report: Option<WorkerInstalledPropertyReportBlueprint>,
    pub reported_incident_response: Option<WorkerReportedIncidentResponseBlueprint>,
}

impl Debug for WorkerBlueprint {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut worker = formatter.debug_struct("WorkerBlueprint");
        worker
            .field("actor_position", &self.actor_position)
            .field("role", &self.role)
            .field("maximum_integrity", &self.maximum_integrity);
        if let Some(affiliation) = &self.affiliation {
            worker.field("affiliation", affiliation);
        }
        if let Some(profile) = self.witness_profile {
            worker.field("witness_profile", &profile);
        }
        if let Some(profile) = self.local_alert_profile {
            worker.field("local_alert_profile", &profile);
        }
        if let Some(report) = self.property_report {
            worker.field("property_report", &report);
        }
        if let Some(report) = &self.installed_property_report {
            worker.field("installed_property_report", report);
        }
        if let Some(response) = self.reported_incident_response {
            worker.field("reported_incident_response", &response);
        }
        worker.finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RepairOrderBlueprint {
    pub id: WorkOrderId,
    pub target: InstallationId,
    pub required_item: ItemId,
    pub required_quantity: u16,
    pub work_turns: u16,
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FacilityBlueprint {
    pub installations: Vec<InstallationBlueprint>,
    pub depot: InstallationId,
    pub workers: Vec<WorkerBlueprint>,
    pub repair_orders: Vec<RepairOrderBlueprint>,
    pub maximum_path_search: usize,
    pub owner: Option<SocialGroupId>,
}

impl Debug for FacilityBlueprint {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut facility = formatter.debug_struct("FacilityBlueprint");
        facility
            .field("installations", &self.installations)
            .field("depot", &self.depot)
            .field("workers", &self.workers)
            .field("repair_orders", &self.repair_orders)
            .field("maximum_path_search", &self.maximum_path_search);
        if let Some(owner) = &self.owner {
            facility.field("owner", owner);
        }
        facility.finish()
    }
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct InstallationState {
    id: InstallationId,
    position: GridPos,
    integrity: u16,
    maximum_integrity: u16,
    capabilities: Vec<InstallationCapability>,
    dependencies: Vec<InstallationId>,
    security_alarm_profile: Option<SecurityAlarmProfile>,
}

impl Debug for InstallationState {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut installation = formatter.debug_struct("InstallationState");
        installation
            .field("id", &self.id)
            .field("position", &self.position)
            .field("integrity", &self.integrity)
            .field("maximum_integrity", &self.maximum_integrity)
            .field("capabilities", &self.capabilities)
            .field("dependencies", &self.dependencies);
        if let Some(profile) = &self.security_alarm_profile {
            installation.field("security_alarm_profile", &profile);
        }
        installation.finish()
    }
}

impl InstallationState {
    pub const fn id(&self) -> &InstallationId {
        &self.id
    }

    pub const fn position(&self) -> GridPos {
        self.position
    }

    pub const fn integrity(&self) -> u16 {
        self.integrity
    }

    pub const fn maximum_integrity(&self) -> u16 {
        self.maximum_integrity
    }

    pub fn capabilities(&self) -> &[InstallationCapability] {
        &self.capabilities
    }

    pub fn dependencies(&self) -> &[InstallationId] {
        &self.dependencies
    }

    pub fn security_alarm_profile(&self) -> Option<&SecurityAlarmProfile> {
        self.security_alarm_profile.as_ref()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RepairStatus {
    WaitingForMaterial,
    MaterialAvailable,
    Assigned {
        worker: EntityId,
    },
    InProgress {
        worker: EntityId,
        remaining_turns: u16,
    },
    Completed,
}

/// Presentation-safe snapshot of a repair order. The facility remains the
/// authority for stock and worker cargo; adapters only receive the amount that
/// is still missing instead of trying to reconstruct it from rendered state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RepairOrderSummary {
    pub order: WorkOrderId,
    pub required_item: ItemId,
    pub missing_quantity: u16,
    pub status: RepairStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct RepairOrderState {
    id: WorkOrderId,
    target: InstallationId,
    required_item: ItemId,
    required_quantity: u16,
    work_turns: u16,
    status: RepairStatus,
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct Cargo {
    item: ItemId,
    quantity: u16,
    owner: Option<SocialGroupId>,
}

impl Debug for Cargo {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut cargo = formatter.debug_struct("Cargo");
        cargo
            .field("item", &self.item)
            .field("quantity", &self.quantity);
        if let Some(owner) = &self.owner {
            cargo.field("owner", owner);
        }
        cargo.finish()
    }
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct WorkerState {
    role: WorkerRole,
    last_position: GridPos,
    cargo: Option<Cargo>,
    reserved_ground: Option<GroundItemId>,
    assigned_order: Option<WorkOrderId>,
    reported_incident_response: Option<WorkerReportedIncidentResponseBlueprint>,
    reported_incident_investigation: Option<ReportedIncidentInvestigation>,
}

impl Debug for WorkerState {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut worker = formatter.debug_struct("WorkerState");
        worker
            .field("role", &self.role)
            .field("last_position", &self.last_position)
            .field("cargo", &self.cargo)
            .field("reserved_ground", &self.reserved_ground)
            .field("assigned_order", &self.assigned_order);
        if let Some(response) = self.reported_incident_response {
            worker.field("reported_incident_response", &response);
        }
        if let Some(investigation) = self.reported_incident_investigation {
            worker.field("reported_incident_investigation", &investigation);
        }
        worker.finish()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct ReportedIncidentInvestigation {
    at: GridPos,
    remaining_inspection_turns: u16,
    remaining_response_turns: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FacilityEvent {
    WorkerMoved {
        worker: EntityId,
        from: GridPos,
        to: GridPos,
    },
    DoorOpened {
        worker: EntityId,
        at: GridPos,
    },
    MaterialCollected {
        worker: EntityId,
        item: ItemId,
        quantity: u16,
    },
    MaterialDelivered {
        worker: EntityId,
        item: ItemId,
        quantity: u16,
    },
    PlayerMaterialDeposited {
        player: EntityId,
        at: GridPos,
        item: ItemId,
        quantity: u16,
    },
    DataTerminalAccessed {
        player: EntityId,
        installation: InstallationId,
        at: GridPos,
        record: ContentId,
        first_access: bool,
    },
    RepairAssigned {
        order: WorkOrderId,
        worker: EntityId,
    },
    RepairStarted {
        order: WorkOrderId,
        worker: EntityId,
        turns: u16,
    },
    ReportedIncidentInvestigationAssigned {
        worker: EntityId,
        at: GridPos,
        maximum_response_turns: u16,
    },
    ReportedIncidentInvestigationEnded {
        worker: EntityId,
        at: GridPos,
        reached: bool,
    },
    InstalledPropertyReportReceived {
        source: EntityId,
        installation: InstallationId,
        at: GridPos,
    },
    InstallationRepaired {
        order: WorkOrderId,
        installation: InstallationId,
    },
    SecurityAlarmRaised {
        installation: InstallationId,
        owner: SocialGroupId,
        at: GridPos,
        duration_turns: u16,
    },
    ReinforcementsRequested {
        installation: InstallationId,
        source: GridPos,
        delay_turns: u16,
    },
    InvestigatingReinforcementsRequested {
        installation: InstallationId,
        source: GridPos,
        incident: GridPos,
        delay_turns: u16,
    },
    ReinforcementsUnavailable {
        installation: InstallationId,
        source: GridPos,
        reason: ReinforcementRequestFailure,
    },
    DoorLockdownStarted {
        installation: InstallationId,
        actuator: InstallationId,
        door: GridPos,
        duration_turns: u64,
    },
    DoorLockdownPrevented {
        installation: InstallationId,
        actuator: InstallationId,
        door: GridPos,
        reason: DoorLockdownPrevention,
    },
    DoorLockdownEnded {
        installation: InstallationId,
        actuator: InstallationId,
        door: GridPos,
    },
    MaterialSpilled {
        item: ItemId,
        quantity: u16,
        at: GridPos,
    },
    WorkInterrupted {
        order: WorkOrderId,
    },
    SimulationFault(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MaterialDeposit {
    pub item: ItemId,
    pub quantity: u16,
}

pub struct SecurityAlarmResponseContext<'a> {
    pub actors: &'a ActorRegistry,
    pub ground: &'a GroundItemRegistry,
    pub protected_position: Option<GridPos>,
    pub egresses: &'a [GridPos],
    pub turn: u64,
}

/// A local installed alarm. It records only the incident seen by this sensor;
/// transmission and faction-wide consequences are deliberately separate.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SecurityAlarm {
    pub incident: ObservedPropertyTake,
    pub expires_on_turn: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoorLockdownPrevention {
    ActuatorUnavailable,
    DoorAlreadyLocked,
    DoorObstructed,
    NoSafeEgress,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReinforcementRequestFailure {
    SourceInactive,
    QuotaExhausted,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SecurityDoorLockdown {
    pub installation: InstallationId,
    pub actuator: InstallationId,
    pub expires_on_turn: u64,
}

impl SecurityDoorLockdown {
    pub const fn is_active(&self, turn: u64) -> bool {
        turn < self.expires_on_turn
    }

    pub const fn remaining_turns(&self, turn: u64) -> u64 {
        self.expires_on_turn.saturating_sub(turn)
    }
}

impl SecurityAlarm {
    pub const fn is_active(&self, turn: u64) -> bool {
        turn < self.expires_on_turn
    }

    pub const fn remaining_turns(&self, turn: u64) -> u64 {
        self.expires_on_turn.saturating_sub(turn)
    }
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FacilityState {
    installations: BTreeMap<InstallationId, InstallationState>,
    depot: InstallationId,
    owner: Option<SocialGroupId>,
    stock: BTreeMap<ItemId, u16>,
    workers: BTreeMap<EntityId, WorkerState>,
    installed_property_reports: BTreeMap<EntityId, WorkerInstalledPropertyReportBlueprint>,
    repair_orders: BTreeMap<WorkOrderId, RepairOrderState>,
    ground_reservations: BTreeMap<GroundItemId, EntityId>,
    accessed_data_terminals: BTreeSet<InstallationId>,
    retained_data_terminal_records: BTreeSet<ContentId>,
    updated_data_terminals: BTreeSet<InstallationId>,
    security_alarms: BTreeMap<InstallationId, SecurityAlarm>,
    security_door_lockdowns: BTreeMap<GridPos, SecurityDoorLockdown>,
    maximum_path_search: usize,
}

impl Debug for FacilityState {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut facility = formatter.debug_struct("FacilityState");
        facility
            .field("installations", &self.installations)
            .field("depot", &self.depot);
        if let Some(owner) = &self.owner {
            facility.field("owner", owner);
        }
        facility
            .field("stock", &self.stock)
            .field("workers", &self.workers);
        if !self.installed_property_reports.is_empty() {
            facility.field(
                "installed_property_reports",
                &self.installed_property_reports,
            );
        }
        facility
            .field("repair_orders", &self.repair_orders)
            .field("ground_reservations", &self.ground_reservations);
        if !self.accessed_data_terminals.is_empty() {
            facility.field("accessed_data_terminals", &self.accessed_data_terminals);
        }
        if !self.retained_data_terminal_records.is_empty() {
            facility.field(
                "retained_data_terminal_records",
                &self.retained_data_terminal_records,
            );
        }
        if !self.updated_data_terminals.is_empty() {
            facility.field("updated_data_terminals", &self.updated_data_terminals);
        }
        if !self.security_alarms.is_empty() {
            facility.field("security_alarms", &self.security_alarms);
        }
        if !self.security_door_lockdowns.is_empty() {
            facility.field("security_door_lockdowns", &self.security_door_lockdowns);
        }
        facility
            .field("maximum_path_search", &self.maximum_path_search)
            .finish()
    }
}

impl FacilityState {
    /// Content-only validation that does not require a generated map or global
    /// entity IDs. Runtime instantiation repeats the placement-specific checks.
    pub fn validate_blueprint(blueprint: &FacilityBlueprint) -> Result<(), FacilityBuildError> {
        validate_limits(blueprint)?;
        let mut installations = BTreeMap::new();
        let mut installation_positions = BTreeSet::new();
        for installation in &blueprint.installations {
            if installation.maximum_integrity == 0
                || installation.integrity > installation.maximum_integrity
            {
                return Err(FacilityBuildError::InvalidIntegrity(
                    installation.id.clone(),
                ));
            }
            if installation.capabilities.is_empty() {
                return Err(FacilityBuildError::InvalidInstallation(
                    installation.id.clone(),
                ));
            }
            if installation.capabilities.iter().any(|capability| {
                matches!(
                    capability,
                    InstallationCapability::NavigationBeacon { range }
                        if *range == 0 || *range > MAX_NAVIGATION_BEACON_RANGE
                )
            }) {
                return Err(FacilityBuildError::InvalidNavigationBeaconRange(
                    installation.id.clone(),
                ));
            }
            if installation
                .capabilities
                .iter()
                .filter(|capability| {
                    matches!(capability, InstallationCapability::DataTerminal { .. })
                })
                .count()
                > 1
            {
                return Err(FacilityBuildError::MultipleDataTerminalRecords(
                    installation.id.clone(),
                ));
            }
            if !installation_positions.insert(installation.position) {
                return Err(FacilityBuildError::DuplicateInstallationPosition(
                    installation.position,
                ));
            }
            let state = InstallationState {
                id: installation.id.clone(),
                position: installation.position,
                integrity: installation.integrity,
                maximum_integrity: installation.maximum_integrity,
                capabilities: installation.capabilities.clone(),
                dependencies: installation.dependencies.clone(),
                security_alarm_profile: installation.security_alarm_profile.clone(),
            };
            if installations
                .insert(installation.id.clone(), state)
                .is_some()
            {
                return Err(FacilityBuildError::DuplicateInstallation(
                    installation.id.clone(),
                ));
            }
        }
        let depot = installations
            .get(&blueprint.depot)
            .ok_or_else(|| FacilityBuildError::UnknownDepot(blueprint.depot.clone()))?;
        if !depot
            .capabilities
            .contains(&InstallationCapability::Storage)
        {
            return Err(FacilityBuildError::DepotWithoutStorage(
                blueprint.depot.clone(),
            ));
        }
        validate_dependencies(&installations)?;

        for installation in installations.values() {
            if installation.security_alarm_profile.is_some()
                && !installation
                    .capabilities
                    .contains(&InstallationCapability::SecuritySensor)
            {
                return Err(FacilityBuildError::AlarmProfileWithoutSensor(
                    installation.id.clone(),
                ));
            }
            if installation.security_alarm_profile.is_some() && blueprint.owner.is_none() {
                return Err(FacilityBuildError::AlarmProfileWithoutOwner(
                    installation.id.clone(),
                ));
            }
            if let Some(profile) = &installation.security_alarm_profile {
                let mut targets = BTreeSet::new();
                let mut reinforcement_sources = BTreeSet::new();
                for response in profile.responses() {
                    match response {
                        SecurityAlarmResponse::LockDoors { actuator } => {
                            if !targets.insert(actuator.clone()) {
                                return Err(FacilityBuildError::DuplicateAlarmResponse {
                                    installation: installation.id.clone(),
                                    actuator: Box::new(actuator.clone()),
                                });
                            }
                            let Some(target) = installations.get(actuator) else {
                                return Err(FacilityBuildError::UnknownAlarmResponseTarget {
                                    installation: installation.id.clone(),
                                    actuator: Box::new(actuator.clone()),
                                });
                            };
                            if !target.capabilities.iter().any(|capability| {
                                matches!(capability, InstallationCapability::DoorActuator { .. })
                            }) {
                                return Err(
                                    FacilityBuildError::AlarmResponseTargetWithoutDoorActuator {
                                        installation: installation.id.clone(),
                                        actuator: Box::new(actuator.clone()),
                                    },
                                );
                            }
                        }
                        SecurityAlarmResponse::CallReinforcements { source, .. }
                        | SecurityAlarmResponse::CallInvestigatingReinforcements {
                            source, ..
                        } => {
                            if !reinforcement_sources.insert(*source) {
                                return Err(FacilityBuildError::DuplicateReinforcementResponse {
                                    installation: installation.id.clone(),
                                    source: *source,
                                });
                            }
                        }
                    }
                }
            }
        }

        let mut worker_positions = BTreeSet::new();
        for worker in &blueprint.workers {
            if worker.maximum_integrity == 0 {
                return Err(FacilityBuildError::InvalidWorkerIntegrity(
                    worker.actor_position,
                ));
            }
            if worker.reported_incident_response.is_some_and(|response| {
                response.inspection_turns == 0
                    || response.maximum_response_turns < response.inspection_turns
                    || response.maximum_response_turns > MAX_REPORTED_INCIDENT_RESPONSE_TURNS
            }) {
                return Err(FacilityBuildError::InvalidReportedIncidentResponse(
                    worker.actor_position,
                ));
            }
            if !worker_positions.insert(worker.actor_position) {
                return Err(FacilityBuildError::DuplicateWorkerPosition(
                    worker.actor_position,
                ));
            }
        }
        for worker in &blueprint.workers {
            let Some(report) = worker.property_report else {
                if worker.installed_property_report.is_none() {
                    continue;
                }
                // Installed links are validated below without requiring an
                // actor recipient.
                continue;
            };
            if worker.witness_profile.is_none() {
                return Err(FacilityBuildError::PropertyReportWithoutWitness(
                    worker.actor_position,
                ));
            }
            if report.recipient_position == worker.actor_position {
                return Err(FacilityBuildError::PropertyReportToSelf(
                    worker.actor_position,
                ));
            }
            let Some(recipient) = blueprint
                .workers
                .iter()
                .find(|candidate| candidate.actor_position == report.recipient_position)
            else {
                return Err(FacilityBuildError::UnknownPropertyReportRecipient {
                    source: worker.actor_position,
                    recipient: report.recipient_position,
                });
            };
            if worker.affiliation.is_none() || worker.affiliation != recipient.affiliation {
                return Err(FacilityBuildError::PropertyReportAffiliationMismatch {
                    source: worker.actor_position,
                    recipient: report.recipient_position,
                });
            }
        }
        for worker in &blueprint.workers {
            let Some(report) = &worker.installed_property_report else {
                continue;
            };
            if worker.witness_profile.is_none() {
                return Err(FacilityBuildError::InstalledPropertyReportWithoutWitness(
                    worker.actor_position,
                ));
            }
            if worker.affiliation.is_none() || worker.affiliation != blueprint.owner {
                return Err(
                    FacilityBuildError::InstalledPropertyReportAffiliationMismatch(
                        worker.actor_position,
                    ),
                );
            }
            let Some(target) = installations.get(&report.installation) else {
                return Err(FacilityBuildError::UnknownInstalledPropertyReportTarget {
                    source: worker.actor_position,
                    installation: Box::new(report.installation.clone()),
                });
            };
            if target.security_alarm_profile.is_none()
                || !target
                    .capabilities
                    .contains(&InstallationCapability::SecuritySensor)
            {
                return Err(FacilityBuildError::InvalidInstalledPropertyReportTarget {
                    source: worker.actor_position,
                    installation: Box::new(report.installation.clone()),
                });
            }
        }
        let mut orders = BTreeSet::new();
        for order in &blueprint.repair_orders {
            if order.required_quantity == 0 || order.work_turns == 0 {
                return Err(FacilityBuildError::InvalidRepairOrder(order.id.clone()));
            }
            if !installations.contains_key(&order.target) {
                return Err(FacilityBuildError::UnknownRepairTarget(
                    order.target.clone(),
                ));
            }
            if !orders.insert(order.id.clone()) {
                return Err(FacilityBuildError::DuplicateRepairOrder(order.id.clone()));
            }
        }
        Ok(())
    }

    pub fn instantiate(
        blueprint: FacilityBlueprint,
        map: &mut Map,
        actors: &mut ActorRegistry,
    ) -> Result<Self, FacilityBuildError> {
        Self::validate_blueprint(&blueprint)?;
        let mut installations = BTreeMap::new();
        for installation in blueprint.installations {
            if installation.maximum_integrity == 0
                || installation.integrity > installation.maximum_integrity
            {
                return Err(FacilityBuildError::InvalidIntegrity(installation.id));
            }
            if !map.contains(installation.position) || installation.capabilities.is_empty() {
                return Err(FacilityBuildError::InvalidInstallation(installation.id));
            }
            for capability in &installation.capabilities {
                if let InstallationCapability::DoorActuator { door } = capability
                    && !matches!(
                        map.tile(*door).map(|tile| tile.terrain),
                        Some(Terrain::Door(_))
                    )
                {
                    return Err(FacilityBuildError::InvalidControlledDoor {
                        installation: installation.id,
                        door: *door,
                    });
                }
            }
            let id = installation.id.clone();
            let state = InstallationState {
                id: id.clone(),
                position: installation.position,
                integrity: installation.integrity,
                maximum_integrity: installation.maximum_integrity,
                capabilities: installation.capabilities,
                dependencies: installation.dependencies,
                security_alarm_profile: installation.security_alarm_profile,
            };
            if installations.insert(id.clone(), state).is_some() {
                return Err(FacilityBuildError::DuplicateInstallation(id));
            }
        }
        let depot = installations
            .get(&blueprint.depot)
            .ok_or_else(|| FacilityBuildError::UnknownDepot(blueprint.depot.clone()))?;
        if !depot
            .capabilities
            .contains(&InstallationCapability::Storage)
        {
            return Err(FacilityBuildError::DepotWithoutStorage(blueprint.depot));
        }
        validate_dependencies(&installations)?;

        let mut workers = BTreeMap::new();
        let mut property_reports = Vec::<(EntityId, PropertyReportProfile)>::new();
        let mut installed_property_reports = BTreeMap::new();
        for worker in blueprint.workers {
            let entity = actors
                .entity_at(worker.actor_position)
                .ok_or(FacilityBuildError::MissingWorker(worker.actor_position))?;
            let actor = actors
                .get(entity)
                .expect("entity_at returned an actor from the same registry");
            if actor.maximum_integrity() != worker.maximum_integrity {
                return Err(FacilityBuildError::WorkerIntegrityMismatch {
                    position: worker.actor_position,
                    expected: worker.maximum_integrity,
                    actual: actor.maximum_integrity(),
                });
            }
            if let Some(report) = worker.property_report {
                let recipient = actors
                    .entity_at(report.recipient_position)
                    .ok_or(FacilityBuildError::MissingWorker(report.recipient_position))?;
                property_reports.push((entity, report.channel.for_recipient(recipient)));
            }
            if let Some(report) = worker.installed_property_report {
                installed_property_reports.insert(entity, report);
            }
            workers.insert(
                entity,
                WorkerState {
                    role: worker.role,
                    last_position: worker.actor_position,
                    cargo: None,
                    reserved_ground: None,
                    assigned_order: None,
                    reported_incident_response: worker.reported_incident_response,
                    reported_incident_investigation: None,
                },
            );
        }

        let mut repair_orders = BTreeMap::new();
        for order in blueprint.repair_orders {
            if order.required_quantity == 0 || order.work_turns == 0 {
                return Err(FacilityBuildError::InvalidRepairOrder(order.id));
            }
            let target = installations
                .get(&order.target)
                .ok_or_else(|| FacilityBuildError::UnknownRepairTarget(order.target.clone()))?;
            let status = if target.integrity == target.maximum_integrity {
                RepairStatus::Completed
            } else {
                RepairStatus::WaitingForMaterial
            };
            let id = order.id.clone();
            let state = RepairOrderState {
                id: id.clone(),
                target: order.target,
                required_item: order.required_item,
                required_quantity: order.required_quantity,
                work_turns: order.work_turns,
                status,
            };
            if repair_orders.insert(id.clone(), state).is_some() {
                return Err(FacilityBuildError::DuplicateRepairOrder(id));
            }
        }

        let mut facility = Self {
            installations,
            depot: blueprint.depot,
            owner: blueprint.owner,
            stock: BTreeMap::new(),
            workers,
            installed_property_reports,
            repair_orders,
            ground_reservations: BTreeMap::new(),
            accessed_data_terminals: BTreeSet::new(),
            retained_data_terminal_records: BTreeSet::new(),
            updated_data_terminals: BTreeSet::new(),
            security_alarms: BTreeMap::new(),
            security_door_lockdowns: BTreeMap::new(),
            maximum_path_search: blueprint.maximum_path_search,
        };
        facility
            .synchronize_outputs(map)
            .map_err(|_| FacilityBuildError::OutputInitializationFailed)?;
        for (source, profile) in property_reports {
            actors
                .get_mut(source)
                .expect("property reporter was resolved from the same registry")
                .set_property_report_profile(profile);
        }
        Ok(facility)
    }

    pub fn installations(&self) -> impl Iterator<Item = (&InstallationId, &InstallationState)> {
        self.installations.iter()
    }

    pub fn installation(&self, id: &InstallationId) -> Option<&InstallationState> {
        self.installations.get(id)
    }

    pub fn installation_at(&self, position: GridPos) -> Option<&InstallationState> {
        self.installations
            .values()
            .find(|installation| installation.position == position)
    }

    pub fn is_operational(&self, id: &InstallationId) -> bool {
        self.is_operational_inner(id, &mut BTreeSet::new())
    }

    pub fn security_alarm(&self, installation: &InstallationId) -> Option<&SecurityAlarm> {
        self.security_alarms.get(installation)
    }

    pub fn security_alarm_at(
        &self,
        position: GridPos,
        turn: u64,
    ) -> Option<(&InstallationState, &SecurityAlarm)> {
        let installation = self.installation_at(position)?;
        let alarm = self
            .security_alarm(&installation.id)
            .filter(|alarm| alarm.is_active(turn))?;
        Some((installation, alarm))
    }

    /// Assigns one fixed incident to an explicitly configured, currently free
    /// worker. Busy workers keep their real maintenance responsibilities and
    /// an active response is never silently retargeted by a later report.
    pub fn assign_reported_incident_investigation(
        &mut self,
        worker: EntityId,
        at: GridPos,
    ) -> Option<FacilityEvent> {
        let state = self.workers.get_mut(&worker)?;
        let response = state.reported_incident_response?;
        if state.cargo.is_some()
            || state.reserved_ground.is_some()
            || state.assigned_order.is_some()
            || state.reported_incident_investigation.is_some()
        {
            return None;
        }
        state.reported_incident_investigation = Some(ReportedIncidentInvestigation {
            at,
            remaining_inspection_turns: response.inspection_turns,
            remaining_response_turns: response.maximum_response_turns,
        });
        Some(FacilityEvent::ReportedIncidentInvestigationAssigned {
            worker,
            at,
            maximum_response_turns: response.maximum_response_turns,
        })
    }

    pub fn active_security_alarms(
        &self,
        turn: u64,
    ) -> impl Iterator<Item = (&InstallationState, &SecurityAlarm)> {
        self.security_alarms.iter().filter_map(move |(id, alarm)| {
            alarm
                .is_active(turn)
                .then(|| self.installations.get(id).map(|source| (source, alarm)))
                .flatten()
        })
    }

    /// Returns only signals which the observer can currently receive. This is
    /// simulation knowledge, not map visibility: navigation beacons explicitly
    /// broadcast through opaque terrain and stop broadcasting when inoperable.
    pub fn detected_navigation_signals(&self, observer: GridPos) -> Vec<DetectedNavigationSignal> {
        let mut signals = self
            .installations
            .iter()
            .filter_map(|(id, installation)| {
                let range = installation
                    .capabilities
                    .iter()
                    .filter_map(|capability| match capability {
                        InstallationCapability::NavigationBeacon { range } => Some(*range),
                        _ => None,
                    })
                    .max()?;
                if !self.is_operational(id) {
                    return None;
                }
                let distance = observer
                    .x
                    .abs_diff(installation.position.x)
                    .max(observer.y.abs_diff(installation.position.y));
                (distance <= u32::from(range)).then(|| DetectedNavigationSignal {
                    installation: id.clone(),
                    position: installation.position,
                    distance,
                })
            })
            .collect::<Vec<_>>();
        signals.sort_by(|first, second| {
            (first.distance, &first.installation).cmp(&(second.distance, &second.installation))
        });
        signals
    }

    pub fn security_door_lockdown_at(
        &self,
        position: GridPos,
        turn: u64,
    ) -> Option<&SecurityDoorLockdown> {
        self.security_door_lockdowns
            .get(&position)
            .filter(|lockdown| lockdown.is_active(turn))
    }

    pub fn active_security_door_lockdowns(
        &self,
        turn: u64,
    ) -> impl Iterator<Item = (GridPos, &SecurityDoorLockdown)> {
        self.security_door_lockdowns
            .iter()
            .filter_map(move |(door, lockdown)| {
                lockdown.is_active(turn).then_some((*door, lockdown))
            })
    }

    /// Tests an already-authorized incident against every operational sensor
    /// in stable installation order. The caller decides which resulting
    /// events are perceptible; hidden alarms remain simulation state only.
    pub fn observe_unauthorized_property_take(
        &mut self,
        map: &Map,
        incident: &ObservedPropertyTake,
    ) -> Vec<FacilityEvent> {
        if self.owner.as_ref() != Some(&incident.owner) {
            return Vec::new();
        }
        let sources: Vec<_> = self
            .installations
            .iter()
            .filter_map(|(id, installation)| {
                let profile = installation.security_alarm_profile.as_ref()?;
                (self.is_operational(id)
                    && compute_visible_tiles(map, installation.position, profile.field_of_view())
                        .contains(&incident.at))
                .then(|| (id.clone(), profile.duration_turns()))
            })
            .collect();
        sources
            .into_iter()
            .filter_map(|(installation, duration_turns)| {
                self.raise_security_alarm(installation, duration_turns, incident)
            })
            .collect()
    }

    /// Receives one witnessed fact through one explicitly authored worker to
    /// installation link. The source must currently see the installed target,
    /// and the target must be operational. The alarm records only the supplied
    /// incident; it gains no live knowledge of the taker.
    pub fn receive_installed_property_report(
        &mut self,
        source: EntityId,
        map: &Map,
        actors: &ActorRegistry,
        incident: &ObservedPropertyTake,
    ) -> Vec<FacilityEvent> {
        if self.owner.as_ref() != Some(&incident.owner) {
            return Vec::new();
        }
        let Some(report) = self.installed_property_reports.get(&source).cloned() else {
            return Vec::new();
        };
        let Some(source_position) = actors.get(source).map(|actor| actor.position()) else {
            return Vec::new();
        };
        let Some(target) = self.installations.get(&report.installation) else {
            return Vec::new();
        };
        if !self.is_operational(&report.installation)
            || !compute_visible_tiles(map, source_position, report.channel.field_of_view())
                .contains(&target.position)
        {
            return Vec::new();
        }
        let installation = report.installation;
        let duration_turns = target
            .security_alarm_profile
            .as_ref()
            .expect("installed report targets are validated security sensors")
            .duration_turns();
        let mut events = vec![FacilityEvent::InstalledPropertyReportReceived {
            source,
            installation: installation.clone(),
            at: incident.at,
        }];
        if let Some(alarm) = self.raise_security_alarm(installation, duration_turns, incident) {
            events.push(alarm);
        }
        events
    }

    fn raise_security_alarm(
        &mut self,
        installation: InstallationId,
        duration_turns: u16,
        incident: &ObservedPropertyTake,
    ) -> Option<FacilityEvent> {
        if self
            .security_alarms
            .get(&installation)
            .is_some_and(|alarm| alarm.incident == *incident)
        {
            return None;
        }
        self.security_alarms.insert(
            installation.clone(),
            SecurityAlarm {
                incident: incident.clone(),
                expires_on_turn: incident
                    .turn
                    .saturating_add(u64::from(duration_turns))
                    .saturating_add(1),
            },
        );
        Some(FacilityEvent::SecurityAlarmRaised {
            installation,
            owner: incident.owner.clone(),
            at: incident.at,
            duration_turns,
        })
    }

    /// Applies only the response primitives declared by newly raised alarms.
    /// A lock is first tested on a cloned map. When a protected actor is
    /// supplied, at least one declared zone egress must remain reachable using
    /// ordinary openable doors; transient actors never make this safety check
    /// nondeterministic.
    pub fn activate_security_alarm_responses(
        &mut self,
        alarm_sources: &[InstallationId],
        map: &mut Map,
        context: SecurityAlarmResponseContext<'_>,
    ) -> Result<Vec<FacilityEvent>, FacilityRuntimeError> {
        let SecurityAlarmResponseContext {
            actors,
            ground,
            protected_position,
            egresses,
            turn,
        } = context;
        let mut events = Vec::new();
        for source in alarm_sources {
            let Some(alarm) = self
                .security_alarms
                .get(source)
                .filter(|alarm| alarm.is_active(turn))
                .cloned()
            else {
                continue;
            };
            let Some(profile) = self
                .installations
                .get(source)
                .and_then(|installation| installation.security_alarm_profile.clone())
            else {
                continue;
            };
            for response in profile.responses() {
                let actuator = match response {
                    SecurityAlarmResponse::CallReinforcements {
                        source: reinforcement_source,
                        delay_turns,
                    } => {
                        events.push(FacilityEvent::ReinforcementsRequested {
                            installation: source.clone(),
                            source: *reinforcement_source,
                            delay_turns: *delay_turns,
                        });
                        continue;
                    }
                    SecurityAlarmResponse::CallInvestigatingReinforcements {
                        source: reinforcement_source,
                        delay_turns,
                    } => {
                        events.push(FacilityEvent::InvestigatingReinforcementsRequested {
                            installation: source.clone(),
                            source: *reinforcement_source,
                            incident: alarm.incident.at,
                            delay_turns: *delay_turns,
                        });
                        continue;
                    }
                    SecurityAlarmResponse::LockDoors { actuator } => actuator,
                };
                let doors: Vec<_> = self
                    .installations
                    .get(actuator)
                    .ok_or_else(|| FacilityRuntimeError::UnknownInstallation(actuator.clone()))?
                    .capabilities
                    .iter()
                    .filter_map(|capability| match capability {
                        InstallationCapability::DoorActuator { door } => Some(*door),
                        _ => None,
                    })
                    .collect();
                let actuator_operational = self.is_operational(actuator);
                for door in doors {
                    let prevented = |reason| FacilityEvent::DoorLockdownPrevented {
                        installation: source.clone(),
                        actuator: actuator.clone(),
                        door,
                        reason,
                    };
                    if !actuator_operational {
                        events.push(prevented(DoorLockdownPrevention::ActuatorUnavailable));
                        continue;
                    }
                    if let Some(lockdown) = self.security_door_lockdowns.get_mut(&door) {
                        lockdown.expires_on_turn =
                            lockdown.expires_on_turn.max(alarm.expires_on_turn);
                        events.push(FacilityEvent::DoorLockdownStarted {
                            installation: source.clone(),
                            actuator: actuator.clone(),
                            door,
                            duration_turns: lockdown.remaining_turns(turn),
                        });
                        continue;
                    }
                    let terrain = map
                        .tile(door)
                        .ok_or(FacilityRuntimeError::InvalidControlledDoor(door))?
                        .terrain;
                    match terrain {
                        Terrain::Door(DoorState::Open | DoorState::Closed) => {}
                        Terrain::Door(DoorState::Locked) => {
                            events.push(prevented(DoorLockdownPrevention::DoorAlreadyLocked));
                            continue;
                        }
                        Terrain::Door(DoorState::Unpowered) => {
                            events.push(prevented(DoorLockdownPrevention::ActuatorUnavailable));
                            continue;
                        }
                        _ => return Err(FacilityRuntimeError::InvalidControlledDoor(door)),
                    }
                    if actors.entity_at(door).is_some() || ground.item_at(door).is_some() {
                        events.push(prevented(DoorLockdownPrevention::DoorObstructed));
                        continue;
                    }
                    let mut candidate = map.clone();
                    candidate
                        .set_terrain(door, Terrain::Door(DoorState::Locked))
                        .map_err(|_| FacilityRuntimeError::InvalidControlledDoor(door))?;
                    if let Some(position) = protected_position
                        && !has_safe_egress(
                            &candidate,
                            position,
                            egresses,
                            self.maximum_path_search,
                        )
                    {
                        events.push(prevented(DoorLockdownPrevention::NoSafeEgress));
                        continue;
                    }
                    map.set_terrain(door, Terrain::Door(DoorState::Locked))
                        .map_err(|_| FacilityRuntimeError::InvalidControlledDoor(door))?;
                    let lockdown = SecurityDoorLockdown {
                        installation: source.clone(),
                        actuator: actuator.clone(),
                        expires_on_turn: alarm.expires_on_turn,
                    };
                    let duration_turns = lockdown.remaining_turns(turn);
                    self.security_door_lockdowns.insert(door, lockdown);
                    events.push(FacilityEvent::DoorLockdownStarted {
                        installation: source.clone(),
                        actuator: actuator.clone(),
                        door,
                        duration_turns,
                    });
                }
            }
        }
        Ok(events)
    }

    /// Releases command locks whose absolute alarm duration elapsed. Static
    /// locks are never touched because only doors recorded above are removed.
    pub fn expire_security_alarm_responses(
        &mut self,
        map: &mut Map,
        turn: u64,
    ) -> Result<Vec<FacilityEvent>, FacilityRuntimeError> {
        let expired: Vec<_> = self
            .security_door_lockdowns
            .iter()
            .filter_map(|(door, lockdown)| {
                (!lockdown.is_active(turn)).then_some((*door, lockdown.clone()))
            })
            .collect();
        let mut events = Vec::with_capacity(expired.len());
        for (door, lockdown) in expired {
            self.security_door_lockdowns.remove(&door);
            let terrain = map
                .tile(door)
                .ok_or(FacilityRuntimeError::InvalidControlledDoor(door))?
                .terrain;
            if terrain == Terrain::Door(DoorState::Locked) {
                map.set_terrain(door, Terrain::Door(DoorState::Closed))
                    .map_err(|_| FacilityRuntimeError::InvalidControlledDoor(door))?;
            } else if !matches!(terrain, Terrain::Door(_)) {
                return Err(FacilityRuntimeError::InvalidControlledDoor(door));
            }
            events.push(FacilityEvent::DoorLockdownEnded {
                installation: lockdown.installation,
                actuator: lockdown.actuator,
                door,
            });
        }
        Ok(events)
    }

    pub fn worker_role(&self, worker: EntityId) -> Option<WorkerRole> {
        self.workers.get(&worker).map(|state| state.role)
    }

    pub fn depot_stock(&self, item: &ItemId) -> u16 {
        self.stock.get(item).copied().unwrap_or(0)
    }

    pub fn is_depot_at(&self, position: GridPos) -> bool {
        self.installations
            .get(&self.depot)
            .is_some_and(|depot| depot.position == position)
    }

    pub fn data_terminal_record_at(&self, position: GridPos) -> Option<&ContentId> {
        self.installation_at(position)?
            .capabilities
            .iter()
            .find_map(|capability| match capability {
                InstallationCapability::DataTerminal { record } => Some(record),
                _ => None,
            })
    }

    pub fn is_player_interactive_at(&self, position: GridPos) -> bool {
        self.is_depot_at(position) || self.data_terminal_record_at(position).is_some()
    }

    /// Reads a terminal through the real installation graph. Re-reading is
    /// allowed so the player never loses a discovered text; `first_access`
    /// remains available to progression and quest adapters without coupling
    /// those systems to the facility simulation.
    pub fn access_data_terminal(&mut self, position: GridPos) -> Option<DataTerminalAccess> {
        let installation = self.installation_at(position)?.id.clone();
        let record = self.data_terminal_record_at(position)?.clone();
        if !self.is_operational(&installation) {
            return None;
        }
        let first_access = self.accessed_data_terminals.insert(installation.clone());
        Some(DataTerminalAccess {
            installation,
            record,
            first_access,
        })
    }

    pub fn data_terminal_was_accessed(&self, installation: &InstallationId) -> bool {
        self.accessed_data_terminals.contains(installation)
    }

    pub fn data_terminal_was_updated(&self, installation: &InstallationId) -> bool {
        self.updated_data_terminals.contains(installation)
    }

    /// Replaces one terminal's current record without erasing information the
    /// player already read. The replacement itself remains undiscovered until
    /// a new successful interaction with the terminal.
    pub fn update_data_terminal_record(
        &mut self,
        installation: &InstallationId,
        next_record: ContentId,
    ) -> Result<ContentId, DataTerminalUpdateError> {
        let state = self
            .installations
            .get_mut(installation)
            .ok_or(DataTerminalUpdateError::UnknownInstallation)?;
        let current_record = state
            .capabilities
            .iter_mut()
            .find_map(|capability| match capability {
                InstallationCapability::DataTerminal { record } => Some(record),
                _ => None,
            })
            .ok_or(DataTerminalUpdateError::MissingDataTerminal)?;
        if current_record == &next_record {
            return Err(DataTerminalUpdateError::UnchangedRecord);
        }
        let previous_record = std::mem::replace(current_record, next_record);
        if self.accessed_data_terminals.remove(installation) {
            self.retained_data_terminal_records
                .insert(previous_record.clone());
        }
        self.updated_data_terminals.insert(installation.clone());
        Ok(previous_record)
    }

    /// Returns the records actually read through this facility. The records
    /// remain derived from terminal state so suspension replay has one source
    /// of truth and presentation never needs its own discovery registry.
    pub fn accessed_data_terminal_records(&self) -> impl Iterator<Item = &ContentId> {
        self.retained_data_terminal_records.iter().chain(
            self.accessed_data_terminals
                .iter()
                .filter_map(|installation| self.installations.get(installation))
                .filter_map(|installation| {
                    installation
                        .capabilities
                        .iter()
                        .find_map(|capability| match capability {
                            InstallationCapability::DataTerminal { record } => Some(record),
                            _ => None,
                        })
                }),
        )
    }

    /// Deposits the first complete missing material request the inventory can
    /// satisfy. Work-order and inventory iteration are stable, so replay never
    /// depends on renderer ordering. In-transit worker cargo is accounted for
    /// to avoid accepting a redundant player delivery.
    pub fn deposit_player_material(
        &mut self,
        inventory: &mut Inventory,
    ) -> Result<Option<MaterialDeposit>, FacilityRuntimeError> {
        let request = self.repair_orders.values().find_map(|order| {
            if order.status != RepairStatus::WaitingForMaterial {
                return None;
            }
            let missing = self.missing_material_for(order);
            if missing == 0 {
                return None;
            }
            let carried: u32 = inventory
                .iter()
                .filter(|entry| entry.item() == &order.required_item)
                .map(|entry| u32::from(entry.quantity()))
                .sum();
            (carried >= u32::from(missing)).then(|| (order.required_item.clone(), missing))
        });
        let Some((item, quantity)) = request else {
            return Ok(None);
        };
        let stock = self.depot_stock(&item);
        let next_stock = stock
            .checked_add(quantity)
            .ok_or_else(|| FacilityRuntimeError::StockOverflow(item.clone()))?;
        let mut next_inventory = inventory.clone();
        let stacks: Vec<_> = next_inventory
            .iter()
            .filter(|entry| entry.item() == &item)
            .map(|entry| (entry.instance(), entry.quantity()))
            .collect();
        let mut remaining = quantity;
        for (instance, available) in stacks {
            if remaining == 0 {
                break;
            }
            let removed = remaining.min(available);
            next_inventory
                .remove(instance, removed)
                .map_err(|_| FacilityRuntimeError::InventoryChanged(instance))?;
            remaining -= removed;
        }
        debug_assert_eq!(remaining, 0, "the availability check is atomic");
        *inventory = next_inventory;
        self.stock.insert(item.clone(), next_stock);
        self.refresh_available_material();
        Ok(Some(MaterialDeposit { item, quantity }))
    }

    pub fn repair_status(&self, order: &WorkOrderId) -> Option<RepairStatus> {
        self.repair_orders.get(order).map(|state| state.status)
    }

    pub fn repair_order_summaries(&self) -> impl Iterator<Item = RepairOrderSummary> + '_ {
        self.repair_orders.values().map(|order| RepairOrderSummary {
            order: order.id.clone(),
            required_item: order.required_item.clone(),
            missing_quantity: self.missing_material_for(order),
            status: order.status,
        })
    }

    pub fn repair_target(&self, order: &WorkOrderId) -> Option<&InstallationState> {
        self.repair_orders
            .get(order)
            .and_then(|order| self.installations.get(&order.target))
    }

    pub fn repair_orders(&self) -> impl Iterator<Item = (&WorkOrderId, RepairStatus)> + '_ {
        self.repair_orders
            .iter()
            .map(|(id, order)| (id, order.status))
    }

    fn missing_material_for(&self, order: &RepairOrderState) -> u16 {
        let in_transit: u32 = self
            .workers
            .values()
            .filter_map(|worker| worker.cargo.as_ref())
            .filter(|cargo| cargo.item == order.required_item)
            .map(|cargo| u32::from(cargo.quantity))
            .sum();
        let supplied = u32::from(self.depot_stock(&order.required_item)).saturating_add(in_transit);
        u16::try_from(u32::from(order.required_quantity).saturating_sub(supplied)).unwrap_or(0)
    }

    pub fn apply_damage(
        &mut self,
        installation: &InstallationId,
        amount: u16,
        map: &mut Map,
    ) -> Result<u16, FacilityRuntimeError> {
        let target = self
            .installations
            .get_mut(installation)
            .ok_or_else(|| FacilityRuntimeError::UnknownInstallation(installation.clone()))?;
        let applied = amount.min(target.integrity);
        target.integrity -= applied;
        if applied > 0 {
            for order in self
                .repair_orders
                .values_mut()
                .filter(|order| &order.target == installation)
            {
                if order.status == RepairStatus::Completed {
                    order.status = RepairStatus::WaitingForMaterial;
                }
            }
            self.synchronize_outputs(map)?;
        }
        Ok(applied)
    }

    pub fn tick(
        &mut self,
        map: &mut Map,
        actors: &mut ActorRegistry,
        ground: &mut GroundItemRegistry,
    ) -> Result<Vec<FacilityEvent>, FacilityRuntimeError> {
        let mut events = Vec::new();
        self.reconcile_missing_workers(map, actors, ground, &mut events)?;
        self.refresh_available_material();

        let responders: BTreeSet<_> = self
            .workers
            .iter()
            .filter_map(|(id, worker)| {
                worker
                    .reported_incident_investigation
                    .is_some()
                    .then_some(*id)
            })
            .collect();
        for worker in responders.iter().copied() {
            self.tick_reported_incident_investigation(worker, map, actors, &mut events);
        }

        let retrievers: Vec<_> = self
            .workers
            .iter()
            .filter_map(|(id, worker)| {
                (worker.role == WorkerRole::Retriever && !responders.contains(id)).then_some(*id)
            })
            .collect();
        for worker in retrievers {
            self.tick_retriever(worker, map, actors, ground, &mut events)?;
        }
        self.refresh_available_material();

        let technicians: Vec<_> = self
            .workers
            .iter()
            .filter_map(|(id, worker)| {
                (worker.role == WorkerRole::Technician && !responders.contains(id)).then_some(*id)
            })
            .collect();
        for worker in technicians {
            self.tick_technician(worker, map, actors, &mut events)?;
        }
        self.synchronize_outputs(map)?;
        Ok(events)
    }

    fn tick_reported_incident_investigation(
        &mut self,
        worker: EntityId,
        map: &mut Map,
        actors: &mut ActorRegistry,
        events: &mut Vec<FacilityEvent>,
    ) {
        let Some(position) = actors.get(worker).map(|actor| actor.position()) else {
            return;
        };
        let Some(mut investigation) = self
            .workers
            .get(&worker)
            .and_then(|state| state.reported_incident_investigation)
        else {
            return;
        };
        self.workers
            .get_mut(&worker)
            .expect("investigating worker exists")
            .last_position = position;

        let reached =
            position == investigation.at || is_cardinally_adjacent(position, investigation.at);
        let completed = if reached {
            investigation.remaining_inspection_turns =
                investigation.remaining_inspection_turns.saturating_sub(1);
            investigation.remaining_inspection_turns == 0
        } else {
            self.move_towards_interaction(worker, investigation.at, map, actors, events);
            false
        };
        investigation.remaining_response_turns =
            investigation.remaining_response_turns.saturating_sub(1);
        let expired = investigation.remaining_response_turns == 0;
        let state = self
            .workers
            .get_mut(&worker)
            .expect("investigating worker exists");
        if completed || expired {
            state.reported_incident_investigation = None;
            events.push(FacilityEvent::ReportedIncidentInvestigationEnded {
                worker,
                at: investigation.at,
                reached: completed,
            });
        } else {
            state.reported_incident_investigation = Some(investigation);
        }
    }

    fn is_operational_inner(
        &self,
        id: &InstallationId,
        visiting: &mut BTreeSet<InstallationId>,
    ) -> bool {
        let Some(installation) = self.installations.get(id) else {
            return false;
        };
        if installation.integrity == 0 || !visiting.insert(id.clone()) {
            return false;
        }
        let operational = installation
            .dependencies
            .iter()
            .all(|dependency| self.is_operational_inner(dependency, visiting));
        visiting.remove(id);
        operational
    }

    fn synchronize_outputs(&mut self, map: &mut Map) -> Result<(), FacilityRuntimeError> {
        let outputs: Vec<_> = self
            .installations
            .values()
            .flat_map(|installation| {
                let operational = self.is_operational(&installation.id);
                installation
                    .capabilities
                    .iter()
                    .filter_map(move |capability| {
                        if let InstallationCapability::DoorActuator { door } = capability {
                            Some((*door, operational))
                        } else {
                            None
                        }
                    })
            })
            .collect();
        for (door, operational) in outputs {
            let terrain = map
                .tile(door)
                .ok_or(FacilityRuntimeError::InvalidControlledDoor(door))?
                .terrain;
            let next = match (operational, terrain) {
                (true, Terrain::Door(DoorState::Unpowered)) => Terrain::Door(DoorState::Closed),
                (false, Terrain::Door(DoorState::Open | DoorState::Closed)) => {
                    Terrain::Door(DoorState::Unpowered)
                }
                (_, Terrain::Door(_)) => continue,
                (_, _) => return Err(FacilityRuntimeError::InvalidControlledDoor(door)),
            };
            map.set_terrain(door, next)
                .map_err(|_| FacilityRuntimeError::InvalidControlledDoor(door))?;
        }
        Ok(())
    }

    fn refresh_available_material(&mut self) {
        for order in self.repair_orders.values_mut() {
            if matches!(
                order.status,
                RepairStatus::WaitingForMaterial | RepairStatus::MaterialAvailable
            ) {
                order.status = if self.stock.get(&order.required_item).copied().unwrap_or(0)
                    >= order.required_quantity
                {
                    RepairStatus::MaterialAvailable
                } else {
                    RepairStatus::WaitingForMaterial
                };
            }
        }
    }

    fn tick_retriever(
        &mut self,
        worker: EntityId,
        map: &mut Map,
        actors: &mut ActorRegistry,
        ground: &mut GroundItemRegistry,
        events: &mut Vec<FacilityEvent>,
    ) -> Result<(), FacilityRuntimeError> {
        let Some(position) = actors.get(worker).map(|actor| actor.position()) else {
            return Ok(());
        };
        self.workers
            .get_mut(&worker)
            .expect("retriever selected from registry")
            .last_position = position;

        if let Some(cargo) = self.workers[&worker].cargo.clone() {
            let depot_position = self.installations[&self.depot].position;
            if is_cardinally_adjacent(position, depot_position) {
                let entry = self.stock.entry(cargo.item.clone()).or_default();
                let quantity = entry
                    .checked_add(cargo.quantity)
                    .ok_or_else(|| FacilityRuntimeError::StockOverflow(cargo.item.clone()))?;
                *entry = quantity;
                self.workers.get_mut(&worker).expect("worker exists").cargo = None;
                events.push(FacilityEvent::MaterialDelivered {
                    worker,
                    item: cargo.item,
                    quantity: cargo.quantity,
                });
            } else {
                self.move_towards_interaction(worker, depot_position, map, actors, events);
            }
            return Ok(());
        }

        let reserved = self.workers[&worker].reserved_ground;
        let source = reserved.and_then(|id| ground.get(id).map(|item| (id, item.clone())));
        let source = if let Some(source) = source {
            Some(source)
        } else {
            if let Some(id) = reserved {
                self.ground_reservations.remove(&id);
                self.workers
                    .get_mut(&worker)
                    .expect("worker exists")
                    .reserved_ground = None;
            }
            self.choose_material_source(worker, position, ground)
                .and_then(|id| ground.get(id).cloned().map(|item| (id, item)))
        };
        let Some((source_id, stack)) = source else {
            return Ok(());
        };
        if self.required_quantity_for(stack.item()) == 0 {
            self.ground_reservations.remove(&source_id);
            self.workers
                .get_mut(&worker)
                .expect("worker exists")
                .reserved_ground = None;
            return Ok(());
        }
        if position == stack.position() {
            let quantity = self
                .required_quantity_for(stack.item())
                .min(stack.quantity());
            if quantity == 0 {
                return Ok(());
            }
            let cargo = ground
                .take(source_id, quantity)
                .map_err(|_| FacilityRuntimeError::MaterialChanged(source_id))?;
            self.ground_reservations.remove(&source_id);
            let worker_state = self.workers.get_mut(&worker).expect("worker exists");
            worker_state.reserved_ground = None;
            worker_state.cargo = Some(Cargo {
                item: cargo.item().clone(),
                quantity: cargo.quantity(),
                owner: cargo.owner().cloned(),
            });
            events.push(FacilityEvent::MaterialCollected {
                worker,
                item: cargo.item().clone(),
                quantity: cargo.quantity(),
            });
        } else {
            self.move_towards_position(worker, stack.position(), map, actors, events);
        }
        Ok(())
    }

    fn choose_material_source(
        &mut self,
        worker: EntityId,
        origin: GridPos,
        ground: &GroundItemRegistry,
    ) -> Option<GroundItemId> {
        let mut sources: Vec<_> = ground
            .iter()
            .filter(|(id, stack)| {
                !self.ground_reservations.contains_key(id)
                    && self.required_quantity_for(stack.item()) > 0
                    && stack.quantity() >= self.required_quantity_for(stack.item())
            })
            .map(|(id, stack)| (manhattan(origin, stack.position()), id))
            .collect();
        sources.sort_unstable();
        let id = sources.first().map(|(_, id)| *id)?;
        self.ground_reservations.insert(id, worker);
        self.workers
            .get_mut(&worker)
            .expect("worker exists")
            .reserved_ground = Some(id);
        Some(id)
    }

    fn required_quantity_for(&self, item: &ItemId) -> u16 {
        self.repair_orders
            .values()
            .filter(|order| {
                order.required_item == *item && order.status == RepairStatus::WaitingForMaterial
            })
            .map(|order| order.required_quantity)
            .min()
            .unwrap_or(0)
    }

    fn tick_technician(
        &mut self,
        worker: EntityId,
        map: &mut Map,
        actors: &mut ActorRegistry,
        events: &mut Vec<FacilityEvent>,
    ) -> Result<(), FacilityRuntimeError> {
        let Some(position) = actors.get(worker).map(|actor| actor.position()) else {
            return Ok(());
        };
        self.workers
            .get_mut(&worker)
            .expect("technician selected from registry")
            .last_position = position;

        let assigned = self.workers[&worker].assigned_order.clone();
        if let Some(order_id) = assigned {
            let target = self.repair_orders[&order_id].target.clone();
            let target_position = self.installations[&target].position;
            if !is_cardinally_adjacent(position, target_position) {
                self.move_towards_interaction(worker, target_position, map, actors, events);
                return Ok(());
            }
            let status = self.repair_orders[&order_id].status;
            match status {
                RepairStatus::Assigned { .. } => {
                    let turns = self.repair_orders[&order_id].work_turns;
                    self.repair_orders
                        .get_mut(&order_id)
                        .expect("assigned order exists")
                        .status = RepairStatus::InProgress {
                        worker,
                        remaining_turns: turns,
                    };
                    events.push(FacilityEvent::RepairStarted {
                        order: order_id,
                        worker,
                        turns,
                    });
                }
                RepairStatus::InProgress {
                    remaining_turns, ..
                } if remaining_turns > 1 => {
                    self.repair_orders
                        .get_mut(&order_id)
                        .expect("assigned order exists")
                        .status = RepairStatus::InProgress {
                        worker,
                        remaining_turns: remaining_turns - 1,
                    };
                }
                RepairStatus::InProgress { .. } => {
                    let target = self.repair_orders[&order_id].target.clone();
                    let installation = self
                        .installations
                        .get_mut(&target)
                        .expect("validated repair target");
                    installation.integrity = installation.maximum_integrity;
                    self.repair_orders
                        .get_mut(&order_id)
                        .expect("assigned order exists")
                        .status = RepairStatus::Completed;
                    let worker_state = self.workers.get_mut(&worker).expect("worker exists");
                    worker_state.cargo = None;
                    worker_state.assigned_order = None;
                    self.synchronize_outputs(map)?;
                    events.push(FacilityEvent::InstallationRepaired {
                        order: order_id,
                        installation: target,
                    });
                }
                _ => {}
            }
            return Ok(());
        }

        let available = self.repair_orders.iter().find_map(|(id, order)| {
            (order.status == RepairStatus::MaterialAvailable).then_some(id.clone())
        });
        let Some(order_id) = available else {
            if self
                .repair_orders
                .values()
                .any(|order| order.status == RepairStatus::WaitingForMaterial)
            {
                let depot_position = self.installations[&self.depot].position;
                if !is_cardinally_adjacent(position, depot_position) {
                    self.move_towards_interaction(worker, depot_position, map, actors, events);
                }
            }
            return Ok(());
        };
        let depot_position = self.installations[&self.depot].position;
        if !is_cardinally_adjacent(position, depot_position) {
            self.move_towards_interaction(worker, depot_position, map, actors, events);
            return Ok(());
        }
        let order = &self.repair_orders[&order_id];
        let stock = self.stock.entry(order.required_item.clone()).or_default();
        if *stock < order.required_quantity {
            return Ok(());
        }
        *stock -= order.required_quantity;
        let cargo = Cargo {
            item: order.required_item.clone(),
            quantity: order.required_quantity,
            owner: self.owner.clone(),
        };
        let worker_state = self.workers.get_mut(&worker).expect("worker exists");
        worker_state.cargo = Some(cargo);
        worker_state.assigned_order = Some(order_id.clone());
        self.repair_orders
            .get_mut(&order_id)
            .expect("available order exists")
            .status = RepairStatus::Assigned { worker };
        events.push(FacilityEvent::RepairAssigned {
            order: order_id,
            worker,
        });
        Ok(())
    }

    fn move_towards_position(
        &self,
        worker: EntityId,
        target: GridPos,
        map: &mut Map,
        actors: &mut ActorRegistry,
        events: &mut Vec<FacilityEvent>,
    ) {
        let Some(origin) = actors.get(worker).map(|actor| actor.position()) else {
            return;
        };
        if actors.entity_at(target).is_some() {
            return;
        }
        let Some(path) = find_path_with(
            map,
            origin,
            target,
            self.maximum_path_search,
            worker_can_traverse,
            |position| actors.entity_at(position).is_none(),
        ) else {
            return;
        };
        self.apply_path_step(worker, origin, &path, map, actors, events);
    }

    fn move_towards_interaction(
        &self,
        worker: EntityId,
        target: GridPos,
        map: &mut Map,
        actors: &mut ActorRegistry,
        events: &mut Vec<FacilityEvent>,
    ) {
        let Some(origin) = actors.get(worker).map(|actor| actor.position()) else {
            return;
        };
        let mut paths: Vec<_> = target
            .cardinal_neighbors()
            .into_iter()
            .filter(|goal| {
                map.is_walkable(*goal) && (*goal == origin || actors.entity_at(*goal).is_none())
            })
            .filter_map(|goal| {
                find_path_with(
                    map,
                    origin,
                    goal,
                    self.maximum_path_search,
                    worker_can_traverse,
                    |position| actors.entity_at(position).is_none(),
                )
                .map(|path| (path.len(), goal, path))
            })
            .collect();
        paths.sort_by_key(|(length, goal, _)| (*length, *goal));
        if let Some((_, _, path)) = paths.first() {
            self.apply_path_step(worker, origin, path, map, actors, events);
        }
    }

    fn apply_path_step(
        &self,
        worker: EntityId,
        origin: GridPos,
        path: &[GridPos],
        map: &mut Map,
        actors: &mut ActorRegistry,
        events: &mut Vec<FacilityEvent>,
    ) {
        let Some(destination) = path.get(1).copied() else {
            return;
        };
        if actors.entity_at(destination).is_some() {
            return;
        }
        if matches!(
            map.tile(destination).map(|tile| tile.terrain),
            Some(Terrain::Door(DoorState::Closed))
        ) {
            if map
                .set_terrain(destination, Terrain::Door(DoorState::Open))
                .is_ok()
            {
                events.push(FacilityEvent::DoorOpened {
                    worker,
                    at: destination,
                });
            }
            return;
        }
        if actors.move_to(worker, destination).is_ok() {
            events.push(FacilityEvent::WorkerMoved {
                worker,
                from: origin,
                to: destination,
            });
        }
    }

    fn reconcile_missing_workers(
        &mut self,
        map: &Map,
        actors: &ActorRegistry,
        ground: &mut GroundItemRegistry,
        events: &mut Vec<FacilityEvent>,
    ) -> Result<(), FacilityRuntimeError> {
        let missing: Vec<_> = self
            .workers
            .keys()
            .copied()
            .filter(|worker| actors.get(*worker).is_none())
            .collect();
        for worker in missing {
            let Some(state) = self.workers.remove(&worker) else {
                continue;
            };
            if let Some(source) = state.reserved_ground {
                self.ground_reservations.remove(&source);
            }
            if let Some(order_id) = state.assigned_order {
                if let Some(order) = self.repair_orders.get_mut(&order_id) {
                    order.status = RepairStatus::WaitingForMaterial;
                }
                events.push(FacilityEvent::WorkInterrupted { order: order_id });
            }
            if let Some(cargo) = state.cargo {
                let at = nearest_free_ground(map, actors, ground, state.last_position)
                    .ok_or(FacilityRuntimeError::NoSpaceForInterruptedCargo)?;
                ground
                    .spawn_with_owner(at, cargo.item.clone(), cargo.quantity, cargo.owner)
                    .map_err(|_| FacilityRuntimeError::NoSpaceForInterruptedCargo)?;
                events.push(FacilityEvent::MaterialSpilled {
                    item: cargo.item,
                    quantity: cargo.quantity,
                    at,
                });
            }
        }
        Ok(())
    }
}

fn worker_can_traverse(map: &Map, position: GridPos) -> bool {
    map.is_walkable(position)
        || matches!(
            map.tile(position).map(|tile| tile.terrain),
            Some(Terrain::Door(DoorState::Closed))
        )
}

fn validate_limits(blueprint: &FacilityBlueprint) -> Result<(), FacilityBuildError> {
    if blueprint.installations.is_empty()
        || blueprint.installations.len() > MAX_INSTALLATIONS
        || blueprint.workers.len() > MAX_WORKERS
        || blueprint.repair_orders.len() > MAX_WORK_ORDERS
        || blueprint.maximum_path_search == 0
        || blueprint.maximum_path_search > MAX_PATH_SEARCH
    {
        return Err(FacilityBuildError::BudgetExceeded);
    }
    Ok(())
}

fn validate_dependencies(
    installations: &BTreeMap<InstallationId, InstallationState>,
) -> Result<(), FacilityBuildError> {
    fn visit(
        id: &InstallationId,
        installations: &BTreeMap<InstallationId, InstallationState>,
        visiting: &mut BTreeSet<InstallationId>,
        visited: &mut BTreeSet<InstallationId>,
    ) -> Result<(), FacilityBuildError> {
        if visited.contains(id) {
            return Ok(());
        }
        if !visiting.insert(id.clone()) {
            return Err(FacilityBuildError::DependencyCycle(id.clone()));
        }
        let installation = installations
            .get(id)
            .ok_or_else(|| FacilityBuildError::UnknownDependency(id.clone()))?;
        for dependency in &installation.dependencies {
            if !installations.contains_key(dependency) {
                return Err(FacilityBuildError::UnknownDependency(dependency.clone()));
            }
            visit(dependency, installations, visiting, visited)?;
        }
        visiting.remove(id);
        visited.insert(id.clone());
        Ok(())
    }

    let mut visiting = BTreeSet::new();
    let mut visited = BTreeSet::new();
    for id in installations.keys() {
        visit(id, installations, &mut visiting, &mut visited)?;
    }
    Ok(())
}

fn nearest_free_ground(
    map: &Map,
    actors: &ActorRegistry,
    ground: &GroundItemRegistry,
    origin: GridPos,
) -> Option<GridPos> {
    let mut cells: Vec<_> = (0..map.height() as i32)
        .flat_map(|y| (0..map.width() as i32).map(move |x| GridPos::new(x, y)))
        .filter(|position| {
            map.is_walkable(*position)
                && actors.entity_at(*position).is_none()
                && ground.item_at(*position).is_none()
        })
        .collect();
    cells.sort_by_key(|position| (manhattan(origin, *position), *position));
    cells.into_iter().next()
}

fn is_cardinally_adjacent(first: GridPos, second: GridPos) -> bool {
    first.cardinal_neighbors().contains(&second)
}

fn has_safe_egress(
    map: &Map,
    origin: GridPos,
    egresses: &[GridPos],
    maximum_path_search: usize,
) -> bool {
    !egresses.is_empty()
        && egresses.iter().copied().any(|egress| {
            find_path_with(
                map,
                origin,
                egress,
                maximum_path_search,
                |map, position| {
                    map.tile(position).is_some_and(|tile| match tile.terrain {
                        Terrain::Door(DoorState::Open | DoorState::Closed) => true,
                        other => !other.blocks_movement(),
                    })
                },
                |_| true,
            )
            .is_some()
        })
}

fn manhattan(first: GridPos, second: GridPos) -> u64 {
    (i64::from(first.x) - i64::from(second.x)).unsigned_abs()
        + (i64::from(first.y) - i64::from(second.y)).unsigned_abs()
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FacilityBuildError {
    BudgetExceeded,
    InvalidIntegrity(InstallationId),
    InvalidInstallation(InstallationId),
    InvalidNavigationBeaconRange(InstallationId),
    MultipleDataTerminalRecords(InstallationId),
    DuplicateInstallation(InstallationId),
    DuplicateInstallationPosition(GridPos),
    InvalidControlledDoor {
        installation: InstallationId,
        door: GridPos,
    },
    AlarmProfileWithoutSensor(InstallationId),
    AlarmProfileWithoutOwner(InstallationId),
    DuplicateAlarmResponse {
        installation: InstallationId,
        actuator: Box<InstallationId>,
    },
    DuplicateReinforcementResponse {
        installation: InstallationId,
        source: GridPos,
    },
    UnknownAlarmResponseTarget {
        installation: InstallationId,
        actuator: Box<InstallationId>,
    },
    AlarmResponseTargetWithoutDoorActuator {
        installation: InstallationId,
        actuator: Box<InstallationId>,
    },
    UnknownDepot(InstallationId),
    DepotWithoutStorage(InstallationId),
    UnknownDependency(InstallationId),
    DependencyCycle(InstallationId),
    DuplicateWorkerPosition(GridPos),
    InvalidWorkerIntegrity(GridPos),
    MissingWorker(GridPos),
    WorkerIntegrityMismatch {
        position: GridPos,
        expected: u16,
        actual: u16,
    },
    InvalidReportedIncidentResponse(GridPos),
    InstalledPropertyReportWithoutWitness(GridPos),
    InstalledPropertyReportAffiliationMismatch(GridPos),
    UnknownInstalledPropertyReportTarget {
        source: GridPos,
        installation: Box<InstallationId>,
    },
    InvalidInstalledPropertyReportTarget {
        source: GridPos,
        installation: Box<InstallationId>,
    },
    PropertyReportWithoutWitness(GridPos),
    PropertyReportToSelf(GridPos),
    UnknownPropertyReportRecipient {
        source: GridPos,
        recipient: GridPos,
    },
    PropertyReportAffiliationMismatch {
        source: GridPos,
        recipient: GridPos,
    },
    InvalidRepairOrder(WorkOrderId),
    DuplicateRepairOrder(WorkOrderId),
    UnknownRepairTarget(InstallationId),
    OutputInitializationFailed,
}

impl Display for FacilityBuildError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "invalid facility blueprint: {self:?}")
    }
}

impl Error for FacilityBuildError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FacilityRuntimeError {
    UnknownInstallation(InstallationId),
    InvalidControlledDoor(GridPos),
    MaterialChanged(GroundItemId),
    InventoryChanged(ItemInstanceId),
    StockOverflow(ItemId),
    NoSpaceForInterruptedCargo,
}

impl Display for FacilityRuntimeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "facility simulation failed: {self:?}")
    }
}

impl Error for FacilityRuntimeError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::AiProfile;
    use crate::entity::Actor;

    fn id(name: &str) -> ContentId {
        format!("test:{name}").parse().unwrap()
    }

    fn map() -> Map {
        let mut map = Map::from_ascii(
            "###########\n#.........#\n#.........#\n#.........#\n#.........#\n###########",
        )
        .unwrap();
        map.set_terrain(GridPos::new(8, 2), Terrain::Door(DoorState::Closed))
            .unwrap();
        map
    }

    fn blueprint(retrievers: &[GridPos], technician: GridPos) -> FacilityBlueprint {
        let relay = id("relay");
        let actuator = id("actuator");
        let sensor = id("sensor");
        let depot = id("depot");
        FacilityBlueprint {
            installations: vec![
                InstallationBlueprint {
                    id: relay.clone(),
                    position: GridPos::new(7, 1),
                    maximum_integrity: 10,
                    integrity: 0,
                    capabilities: vec![InstallationCapability::PowerRelay],
                    dependencies: vec![],
                    security_alarm_profile: None,
                },
                InstallationBlueprint {
                    id: actuator,
                    position: GridPos::new(8, 1),
                    maximum_integrity: 10,
                    integrity: 10,
                    capabilities: vec![InstallationCapability::DoorActuator {
                        door: GridPos::new(8, 2),
                    }],
                    dependencies: vec![relay.clone()],
                    security_alarm_profile: None,
                },
                InstallationBlueprint {
                    id: sensor,
                    position: GridPos::new(9, 1),
                    maximum_integrity: 10,
                    integrity: 10,
                    capabilities: vec![InstallationCapability::SecuritySensor],
                    dependencies: vec![relay.clone()],
                    security_alarm_profile: None,
                },
                InstallationBlueprint {
                    id: depot.clone(),
                    position: GridPos::new(2, 1),
                    maximum_integrity: 10,
                    integrity: 10,
                    capabilities: vec![InstallationCapability::Storage],
                    dependencies: vec![],
                    security_alarm_profile: None,
                },
            ],
            depot,
            workers: retrievers
                .iter()
                .copied()
                .map(|actor_position| WorkerBlueprint {
                    actor_position,
                    role: WorkerRole::Retriever,
                    maximum_integrity: 10,
                    affiliation: None,
                    witness_profile: None,
                    local_alert_profile: None,
                    property_report: None,
                    installed_property_report: None,
                    reported_incident_response: None,
                })
                .chain([WorkerBlueprint {
                    actor_position: technician,
                    role: WorkerRole::Technician,
                    maximum_integrity: 10,
                    affiliation: None,
                    witness_profile: None,
                    local_alert_profile: None,
                    property_report: None,
                    installed_property_report: None,
                    reported_incident_response: None,
                }])
                .collect(),
            repair_orders: vec![RepairOrderBlueprint {
                id: id("repair_relay"),
                target: relay,
                required_item: id("regulator"),
                required_quantity: 1,
                work_turns: 2,
            }],
            maximum_path_search: 256,
            owner: None,
        }
    }

    fn actors(retrievers: &[GridPos], technician: GridPos) -> ActorRegistry {
        let mut actors = ActorRegistry::default();
        for position in retrievers.iter().copied().chain([technician]) {
            actors
                .spawn(Actor::new(position, 10).unwrap().with_ai(AiProfile::idle()))
                .unwrap();
        }
        actors
    }

    #[test]
    fn authored_property_report_resolves_one_worker_and_rejects_implicit_links() {
        let reporter_position = GridPos::new(5, 3);
        let recipient_position = GridPos::new(3, 3);
        let owner = id("collective");
        let mut definition = blueprint(&[reporter_position], recipient_position);
        for worker in &mut definition.workers {
            worker.affiliation = Some(owner.clone());
        }
        definition.workers[0].witness_profile =
            Some(WitnessProfile::new(8, DistanceMetric::Euclidean, true, 4).unwrap());
        definition.workers[0].property_report = Some(WorkerPropertyReportBlueprint {
            recipient_position,
            channel: PropertyReportChannel::new(8, DistanceMetric::Euclidean, true).unwrap(),
        });
        let mut map = map();
        let mut actors = actors(&[reporter_position], recipient_position);
        let reporter = actors.entity_at(reporter_position).unwrap();
        let recipient = actors.entity_at(recipient_position).unwrap();

        FacilityState::instantiate(definition.clone(), &mut map, &mut actors).unwrap();

        assert_eq!(
            actors
                .get(reporter)
                .and_then(Actor::property_report_profile)
                .map(|profile| profile.recipient()),
            Some(recipient)
        );

        definition.workers[0].property_report = Some(WorkerPropertyReportBlueprint {
            recipient_position: reporter_position,
            channel: PropertyReportChannel::new(8, DistanceMetric::Euclidean, true).unwrap(),
        });
        assert!(matches!(
            FacilityState::validate_blueprint(&definition),
            Err(FacilityBuildError::PropertyReportToSelf(position))
                if position == reporter_position
        ));
    }

    #[test]
    fn reported_incident_response_walks_to_the_fixed_location_then_resumes() {
        let technician_position = GridPos::new(2, 3);
        let incident = GridPos::new(7, 3);
        let mut definition = blueprint(&[], technician_position);
        definition.workers[0].reported_incident_response =
            Some(WorkerReportedIncidentResponseBlueprint {
                inspection_turns: 2,
                maximum_response_turns: 12,
            });
        let mut map = map();
        let mut actors = actors(&[], technician_position);
        let technician = actors.entity_at(technician_position).unwrap();
        let mut ground = GroundItemRegistry::default();
        let mut facility = FacilityState::instantiate(definition, &mut map, &mut actors).unwrap();

        assert_eq!(
            facility.assign_reported_incident_investigation(technician, incident),
            Some(FacilityEvent::ReportedIncidentInvestigationAssigned {
                worker: technician,
                at: incident,
                maximum_response_turns: 12,
            })
        );
        assert_eq!(
            facility.assign_reported_incident_investigation(technician, GridPos::new(2, 1)),
            None,
            "an active response keeps its first fixed incident"
        );

        let mut events = Vec::new();
        for _ in 0..12 {
            events.extend(facility.tick(&mut map, &mut actors, &mut ground).unwrap());
            if events.iter().any(|event| {
                matches!(
                    event,
                    FacilityEvent::ReportedIncidentInvestigationEnded { reached: true, .. }
                )
            }) {
                break;
            }
        }
        let final_position = actors.get(technician).unwrap().position();
        assert!(
            final_position == incident || is_cardinally_adjacent(final_position, incident),
            "the worker must inspect the reported location, not the later report"
        );
        assert!(
            events.contains(&FacilityEvent::ReportedIncidentInvestigationEnded {
                worker: technician,
                at: incident,
                reached: true,
            })
        );
    }

    #[test]
    fn unreachable_reported_incident_expires_without_monopolizing_the_worker() {
        let technician_position = GridPos::new(2, 3);
        let incident = GridPos::new(7, 3);
        let mut definition = blueprint(&[], technician_position);
        definition.workers[0].reported_incident_response =
            Some(WorkerReportedIncidentResponseBlueprint {
                inspection_turns: 2,
                maximum_response_turns: 3,
            });
        let mut map = map();
        for blocked in technician_position.cardinal_neighbors() {
            map.set_terrain(blocked, Terrain::Wall).unwrap();
        }
        let mut actors = actors(&[], technician_position);
        let technician = actors.entity_at(technician_position).unwrap();
        let mut ground = GroundItemRegistry::default();
        let mut facility = FacilityState::instantiate(definition, &mut map, &mut actors).unwrap();
        facility
            .assign_reported_incident_investigation(technician, incident)
            .unwrap();

        let mut events = Vec::new();
        for _ in 0..3 {
            events.extend(facility.tick(&mut map, &mut actors, &mut ground).unwrap());
        }

        assert_eq!(
            actors.get(technician).unwrap().position(),
            technician_position
        );
        assert!(
            events.contains(&FacilityEvent::ReportedIncidentInvestigationEnded {
                worker: technician,
                at: incident,
                reached: false,
            })
        );
        assert!(
            facility
                .assign_reported_incident_investigation(technician, GridPos::new(3, 3))
                .is_some()
        );
    }

    #[test]
    fn reported_incident_response_profile_is_positive_and_bounded() {
        let mut definition = blueprint(&[], GridPos::new(3, 3));
        definition.workers[0].reported_incident_response =
            Some(WorkerReportedIncidentResponseBlueprint {
                inspection_turns: 0,
                maximum_response_turns: 3,
            });
        assert!(matches!(
            FacilityState::validate_blueprint(&definition),
            Err(FacilityBuildError::InvalidReportedIncidentResponse(_))
        ));

        definition.workers[0].reported_incident_response =
            Some(WorkerReportedIncidentResponseBlueprint {
                inspection_turns: 4,
                maximum_response_turns: 3,
            });
        assert!(matches!(
            FacilityState::validate_blueprint(&definition),
            Err(FacilityBuildError::InvalidReportedIncidentResponse(_))
        ));
    }

    #[test]
    fn installed_property_report_requires_its_real_link_and_operational_target() {
        let reporter_position = GridPos::new(5, 3);
        let technician_position = GridPos::new(3, 3);
        let incident = ObservedPropertyTake {
            turn: 4,
            taker: EntityId::new(99),
            owner: id("collective"),
            item: id("regulator"),
            quantity: 1,
            at: GridPos::new(1, 3),
        };
        let mut definition = blueprint(&[reporter_position], technician_position);
        definition.owner = Some(incident.owner.clone());
        definition.installations[0].integrity = 10;
        definition.installations[2].security_alarm_profile =
            Some(SecurityAlarmProfile::new(1, DistanceMetric::Euclidean, true, 8).unwrap());
        definition.workers[0].affiliation = Some(incident.owner.clone());
        definition.workers[0].witness_profile =
            Some(WitnessProfile::new(8, DistanceMetric::Euclidean, true, 4).unwrap());
        definition.workers[0].installed_property_report =
            Some(WorkerInstalledPropertyReportBlueprint {
                installation: id("sensor"),
                channel: PropertyReportChannel::new(8, DistanceMetric::Euclidean, true).unwrap(),
            });
        let mut map = map();
        let mut registry = actors(&[reporter_position], technician_position);
        let reporter = registry.entity_at(reporter_position).unwrap();
        let mut facility =
            FacilityState::instantiate(definition.clone(), &mut map, &mut registry).unwrap();

        assert!(
            facility
                .observe_unauthorized_property_take(&map, &incident)
                .is_empty(),
            "the installed sensor cannot directly see the remote incident"
        );
        assert_eq!(
            facility.receive_installed_property_report(reporter, &map, &registry, &incident),
            vec![
                FacilityEvent::InstalledPropertyReportReceived {
                    source: reporter,
                    installation: id("sensor"),
                    at: incident.at,
                },
                FacilityEvent::SecurityAlarmRaised {
                    installation: id("sensor"),
                    owner: incident.owner.clone(),
                    at: incident.at,
                    duration_turns: 8,
                },
            ]
        );
        assert!(facility.security_alarm(&id("sensor")).is_some());
        assert_eq!(
            facility.receive_installed_property_report(reporter, &map, &registry, &incident),
            vec![FacilityEvent::InstalledPropertyReportReceived {
                source: reporter,
                installation: id("sensor"),
                at: incident.at,
            }],
            "the same fact may be acknowledged but must not raise a duplicate alarm"
        );

        definition.installations[0].integrity = 0;
        let mut registry = actors(&[reporter_position], technician_position);
        let reporter = registry.entity_at(reporter_position).unwrap();
        let mut facility = FacilityState::instantiate(definition, &mut map, &mut registry).unwrap();
        assert!(
            facility
                .receive_installed_property_report(reporter, &map, &registry, &incident)
                .is_empty(),
            "an unpowered target cannot acknowledge a report"
        );
    }

    #[test]
    fn installed_property_report_target_and_affiliation_are_validated() {
        let reporter_position = GridPos::new(5, 3);
        let mut definition = blueprint(&[reporter_position], GridPos::new(3, 3));
        definition.owner = Some(id("collective"));
        definition.workers[0].witness_profile =
            Some(WitnessProfile::new(8, DistanceMetric::Euclidean, true, 4).unwrap());
        definition.workers[0].installed_property_report =
            Some(WorkerInstalledPropertyReportBlueprint {
                installation: id("sensor"),
                channel: PropertyReportChannel::new(8, DistanceMetric::Euclidean, true).unwrap(),
            });
        assert!(matches!(
            FacilityState::validate_blueprint(&definition),
            Err(FacilityBuildError::InstalledPropertyReportAffiliationMismatch(position))
                if position == reporter_position
        ));

        definition.workers[0].affiliation = definition.owner.clone();
        assert!(matches!(
            FacilityState::validate_blueprint(&definition),
            Err(FacilityBuildError::InvalidInstalledPropertyReportTarget { .. })
        ));
    }

    #[test]
    fn material_moves_once_then_repair_restores_real_outputs() {
        let mut map = map();
        let mut actors = actors(&[GridPos::new(5, 3)], GridPos::new(3, 3));
        let mut ground = GroundItemRegistry::default();
        ground
            .spawn(GridPos::new(7, 3), id("regulator"), 1)
            .unwrap();
        let mut facility = FacilityState::instantiate(
            blueprint(&[GridPos::new(5, 3)], GridPos::new(3, 3)),
            &mut map,
            &mut actors,
        )
        .unwrap();
        assert_eq!(
            map.tile(GridPos::new(8, 2)).unwrap().terrain,
            Terrain::Door(DoorState::Unpowered)
        );
        assert!(!facility.is_operational(&id("sensor")));

        for _ in 0..30 {
            facility.tick(&mut map, &mut actors, &mut ground).unwrap();
        }

        assert_eq!(
            facility.repair_status(&id("repair_relay")),
            Some(RepairStatus::Completed)
        );
        assert!(facility.is_operational(&id("relay")));
        assert!(facility.is_operational(&id("sensor")));
        assert_eq!(facility.depot_stock(&id("regulator")), 0);
        assert!(
            ground
                .iter()
                .all(|(_, stack)| stack.item() != &id("regulator"))
        );
        assert_eq!(
            map.tile(GridPos::new(8, 2)).unwrap().terrain,
            Terrain::Door(DoorState::Closed)
        );
    }

    #[test]
    fn two_retrievers_do_not_duplicate_or_reserve_the_same_piece() {
        let retrievers = [GridPos::new(4, 3), GridPos::new(5, 3)];
        let technician = GridPos::new(3, 3);
        let mut map = map();
        let mut actors = actors(&retrievers, technician);
        let mut ground = GroundItemRegistry::default();
        ground
            .spawn(GridPos::new(7, 3), id("regulator"), 1)
            .unwrap();
        let mut facility =
            FacilityState::instantiate(blueprint(&retrievers, technician), &mut map, &mut actors)
                .unwrap();

        for _ in 0..8 {
            facility.tick(&mut map, &mut actors, &mut ground).unwrap();
        }
        let quantity_on_ground: u16 = ground
            .iter()
            .filter(|(_, stack)| stack.item() == &id("regulator"))
            .map(|(_, stack)| stack.quantity())
            .sum();
        let quantity_in_stock = facility.depot_stock(&id("regulator"));
        let quantity_carried: u16 = facility
            .workers
            .values()
            .filter_map(|worker| worker.cargo.as_ref())
            .filter(|cargo| cargo.item == id("regulator"))
            .map(|cargo| cargo.quantity)
            .sum();
        assert_eq!(quantity_on_ground + quantity_in_stock + quantity_carried, 1);
    }

    #[test]
    fn player_delivery_supersedes_a_ground_reservation_without_extra_collection() {
        let retriever = GridPos::new(5, 3);
        let technician = GridPos::new(3, 3);
        let mut map = map();
        let mut actors = actors(&[retriever], technician);
        let mut ground = GroundItemRegistry::default();
        ground
            .spawn(GridPos::new(7, 3), id("regulator"), 1)
            .unwrap();
        let mut facility =
            FacilityState::instantiate(blueprint(&[retriever], technician), &mut map, &mut actors)
                .unwrap();
        facility.tick(&mut map, &mut actors, &mut ground).unwrap();
        assert_eq!(facility.ground_reservations.len(), 1);
        let mut inventory = Inventory::new(2);
        inventory.add(id("regulator"), 1, 4).unwrap();

        assert_eq!(
            facility.deposit_player_material(&mut inventory).unwrap(),
            Some(MaterialDeposit {
                item: id("regulator"),
                quantity: 1,
            })
        );
        assert!(inventory.is_empty());

        for _ in 0..20 {
            facility.tick(&mut map, &mut actors, &mut ground).unwrap();
        }
        assert_eq!(
            facility.repair_status(&id("repair_relay")),
            Some(RepairStatus::Completed)
        );
        assert!(facility.ground_reservations.is_empty());
        assert_eq!(
            ground
                .iter()
                .filter(|(_, stack)| stack.item() == &id("regulator"))
                .map(|(_, stack)| stack.quantity())
                .sum::<u16>(),
            1
        );
    }

    #[test]
    fn player_delivery_can_atomically_span_several_inventory_stacks() {
        let technician = GridPos::new(3, 3);
        let mut map = map();
        let mut actors = actors(&[], technician);
        let mut definition = blueprint(&[], technician);
        definition.repair_orders[0].required_quantity = 5;
        let mut facility = FacilityState::instantiate(definition, &mut map, &mut actors).unwrap();
        let mut inventory = Inventory::new(2);
        inventory.add(id("regulator"), 5, 3).unwrap();
        assert_eq!(inventory.len(), 2);

        assert_eq!(
            facility.deposit_player_material(&mut inventory).unwrap(),
            Some(MaterialDeposit {
                item: id("regulator"),
                quantity: 5,
            })
        );
        assert!(inventory.is_empty());
        assert_eq!(facility.depot_stock(&id("regulator")), 5);
        assert_eq!(
            facility.repair_status(&id("repair_relay")),
            Some(RepairStatus::MaterialAvailable)
        );
    }

    #[test]
    fn blocked_paths_pause_and_retry_without_teleporting() {
        let retriever = GridPos::new(2, 3);
        let mut map = map();
        map.set_terrain(GridPos::new(3, 3), Terrain::Wall).unwrap();
        let mut actors = ActorRegistry::default();
        let retriever_id = actors
            .spawn(
                Actor::new(retriever, 10)
                    .unwrap()
                    .with_ai(AiProfile::idle()),
            )
            .unwrap();
        let technician_id = actors
            .spawn(
                Actor::new(GridPos::new(2, 2), 10)
                    .unwrap()
                    .with_ai(AiProfile::idle()),
            )
            .unwrap();
        let mut blueprint = blueprint(&[retriever], GridPos::new(2, 2));
        blueprint.workers[1].actor_position = GridPos::new(2, 2);
        let mut ground = GroundItemRegistry::default();
        ground
            .spawn(GridPos::new(8, 3), id("regulator"), 1)
            .unwrap();
        for y in 1..5 {
            map.set_terrain(GridPos::new(5, y), Terrain::Wall).unwrap();
        }
        let mut facility = FacilityState::instantiate(blueprint, &mut map, &mut actors).unwrap();

        for _ in 0..5 {
            facility.tick(&mut map, &mut actors, &mut ground).unwrap();
        }
        assert_eq!(actors.get(retriever_id).unwrap().position(), retriever);
        assert_eq!(
            facility.repair_status(&id("repair_relay")),
            Some(RepairStatus::WaitingForMaterial)
        );

        map.set_terrain(GridPos::new(5, 3), Terrain::Floor).unwrap();
        for _ in 0..35 {
            facility.tick(&mut map, &mut actors, &mut ground).unwrap();
        }
        assert_eq!(
            facility.repair_status(&id("repair_relay")),
            Some(RepairStatus::Completed)
        );
        assert!(actors.get(technician_id).is_some());
    }

    #[test]
    fn operational_security_sensor_records_only_visible_owned_incidents() {
        assert!(SecurityAlarmProfile::new(0, DistanceMetric::Euclidean, true, 8).is_err());
        assert!(SecurityAlarmProfile::new(8, DistanceMetric::Euclidean, true, 0).is_err());
        let profile = SecurityAlarmProfile::new(8, DistanceMetric::Euclidean, true, 8).unwrap();
        let technician = GridPos::new(3, 3);
        let mut definition = blueprint(&[], technician);
        definition.owner = Some(id("collective"));
        definition.installations[0].integrity = 10;
        definition.installations[2].security_alarm_profile = Some(profile);
        let mut map = map();
        let mut actors = actors(&[], technician);
        let mut facility = FacilityState::instantiate(definition, &mut map, &mut actors).unwrap();
        let taker = actors.entity_at(technician).unwrap();

        let incident = |owner: SocialGroupId, at: GridPos| ObservedPropertyTake {
            turn: 4,
            taker,
            owner,
            item: id("regulator"),
            quantity: 1,
            at,
        };
        assert!(
            facility
                .observe_unauthorized_property_take(
                    &map,
                    &incident(id("other_group"), GridPos::new(8, 1))
                )
                .is_empty()
        );
        assert!(
            facility
                .observe_unauthorized_property_take(
                    &map,
                    &incident(id("collective"), GridPos::new(7, 3))
                )
                .is_empty(),
            "the closed door at 8,2 blocks the sensor"
        );

        assert_eq!(
            facility.observe_unauthorized_property_take(
                &map,
                &incident(id("collective"), GridPos::new(8, 1))
            ),
            vec![FacilityEvent::SecurityAlarmRaised {
                installation: id("sensor"),
                owner: id("collective"),
                at: GridPos::new(8, 1),
                duration_turns: 8,
            }]
        );
        let alarm = facility.security_alarm(&id("sensor")).unwrap();
        assert!(alarm.is_active(12));
        assert_eq!(alarm.remaining_turns(5), 8);
        assert!(!alarm.is_active(13));
    }

    #[test]
    fn alarm_lockdown_is_data_driven_temporary_and_cannot_trap_the_player() {
        let profile = SecurityAlarmProfile::new(8, DistanceMetric::Euclidean, true, 8)
            .unwrap()
            .with_responses(vec![SecurityAlarmResponse::LockDoors {
                actuator: id("actuator"),
            }])
            .unwrap();
        let technician = GridPos::new(3, 3);
        let mut definition = blueprint(&[], technician);
        definition.owner = Some(id("collective"));
        definition.installations[0].integrity = 10;
        definition.installations[2].security_alarm_profile = Some(profile);
        let mut map = map();
        for position in [GridPos::new(8, 1), GridPos::new(8, 3), GridPos::new(8, 4)] {
            map.set_terrain(position, Terrain::Wall).unwrap();
        }
        let mut actors = actors(&[], technician);
        let ground = GroundItemRegistry::default();
        let mut facility = FacilityState::instantiate(definition, &mut map, &mut actors).unwrap();
        let alarm_sources = [id("sensor")];
        let incident = ObservedPropertyTake {
            turn: 4,
            taker: actors.entity_at(technician).unwrap(),
            owner: id("collective"),
            item: id("regulator"),
            quantity: 1,
            at: GridPos::new(9, 2),
        };
        facility.observe_unauthorized_property_take(&map, &incident);

        assert_eq!(
            facility
                .activate_security_alarm_responses(
                    &alarm_sources,
                    &mut map,
                    SecurityAlarmResponseContext {
                        actors: &actors,
                        ground: &ground,
                        protected_position: Some(GridPos::new(9, 2)),
                        egresses: &[GridPos::new(1, 2)],
                        turn: 5,
                    },
                )
                .unwrap(),
            vec![FacilityEvent::DoorLockdownPrevented {
                installation: id("sensor"),
                actuator: id("actuator"),
                door: GridPos::new(8, 2),
                reason: DoorLockdownPrevention::NoSafeEgress,
            }]
        );
        assert_eq!(
            map.tile(GridPos::new(8, 2)).unwrap().terrain,
            Terrain::Door(DoorState::Closed)
        );

        assert_eq!(
            facility
                .activate_security_alarm_responses(
                    &alarm_sources,
                    &mut map,
                    SecurityAlarmResponseContext {
                        actors: &actors,
                        ground: &ground,
                        protected_position: Some(GridPos::new(7, 2)),
                        egresses: &[GridPos::new(1, 2)],
                        turn: 5,
                    },
                )
                .unwrap(),
            vec![FacilityEvent::DoorLockdownStarted {
                installation: id("sensor"),
                actuator: id("actuator"),
                door: GridPos::new(8, 2),
                duration_turns: 8,
            }]
        );
        assert_eq!(
            map.tile(GridPos::new(8, 2)).unwrap().terrain,
            Terrain::Door(DoorState::Locked)
        );
        assert_eq!(
            facility
                .security_door_lockdown_at(GridPos::new(8, 2), 5)
                .unwrap()
                .remaining_turns(5),
            8
        );

        assert!(
            facility
                .expire_security_alarm_responses(&mut map, 12)
                .unwrap()
                .is_empty()
        );
        assert_eq!(
            facility
                .expire_security_alarm_responses(&mut map, 13)
                .unwrap(),
            vec![FacilityEvent::DoorLockdownEnded {
                installation: id("sensor"),
                actuator: id("actuator"),
                door: GridPos::new(8, 2),
            }]
        );
        assert_eq!(
            map.tile(GridPos::new(8, 2)).unwrap().terrain,
            Terrain::Door(DoorState::Closed)
        );
    }

    #[test]
    fn alarm_reinforcement_response_emits_only_a_bounded_source_request() {
        assert_eq!(
            SecurityAlarmProfile::new(8, DistanceMetric::Euclidean, true, 8)
                .unwrap()
                .with_responses(vec![SecurityAlarmResponse::CallReinforcements {
                    source: GridPos::new(6, 3),
                    delay_turns: 0,
                }]),
            Err(SecurityAlarmProfileError::InvalidReinforcementDelay(0))
        );
        let profile = SecurityAlarmProfile::new(8, DistanceMetric::Euclidean, true, 8)
            .unwrap()
            .with_responses(vec![SecurityAlarmResponse::CallReinforcements {
                source: GridPos::new(6, 3),
                delay_turns: 3,
            }])
            .unwrap();
        let technician = GridPos::new(3, 3);
        let mut definition = blueprint(&[], technician);
        definition.owner = Some(id("collective"));
        definition.installations[0].integrity = 10;
        definition.installations[2].security_alarm_profile = Some(profile);
        let mut map = map();
        let mut actors = actors(&[], technician);
        let ground = GroundItemRegistry::default();
        let mut facility = FacilityState::instantiate(definition, &mut map, &mut actors).unwrap();
        facility.observe_unauthorized_property_take(
            &map,
            &ObservedPropertyTake {
                turn: 4,
                taker: actors.entity_at(technician).unwrap(),
                owner: id("collective"),
                item: id("regulator"),
                quantity: 1,
                at: GridPos::new(8, 1),
            },
        );

        assert_eq!(
            facility
                .activate_security_alarm_responses(
                    &[id("sensor")],
                    &mut map,
                    SecurityAlarmResponseContext {
                        actors: &actors,
                        ground: &ground,
                        protected_position: Some(GridPos::new(8, 1)),
                        egresses: &[],
                        turn: 5,
                    },
                )
                .unwrap(),
            vec![FacilityEvent::ReinforcementsRequested {
                installation: id("sensor"),
                source: GridPos::new(6, 3),
                delay_turns: 3,
            }]
        );
    }

    #[test]
    fn security_alarm_profiles_require_a_sensor_and_facility_owner() {
        let profile = SecurityAlarmProfile::new(8, DistanceMetric::Euclidean, true, 8).unwrap();
        let mut no_sensor = blueprint(&[], GridPos::new(3, 3));
        no_sensor.owner = Some(id("collective"));
        no_sensor.installations[0].security_alarm_profile = Some(profile.clone());
        assert!(matches!(
            FacilityState::validate_blueprint(&no_sensor),
            Err(FacilityBuildError::AlarmProfileWithoutSensor(_))
        ));

        let mut no_owner = blueprint(&[], GridPos::new(3, 3));
        no_owner.installations[2].security_alarm_profile = Some(profile);
        assert!(matches!(
            FacilityState::validate_blueprint(&no_owner),
            Err(FacilityBuildError::AlarmProfileWithoutOwner(_))
        ));
    }

    #[test]
    fn alarm_response_targets_must_be_unique_door_actuators() {
        let profile = SecurityAlarmProfile::new(8, DistanceMetric::Euclidean, true, 8)
            .unwrap()
            .with_responses(vec![SecurityAlarmResponse::LockDoors {
                actuator: id("missing"),
            }])
            .unwrap();
        let mut unknown = blueprint(&[], GridPos::new(3, 3));
        unknown.owner = Some(id("collective"));
        unknown.installations[2].security_alarm_profile = Some(profile);
        assert!(matches!(
            FacilityState::validate_blueprint(&unknown),
            Err(FacilityBuildError::UnknownAlarmResponseTarget { .. })
        ));

        let profile = SecurityAlarmProfile::new(8, DistanceMetric::Euclidean, true, 8)
            .unwrap()
            .with_responses(vec![
                SecurityAlarmResponse::LockDoors {
                    actuator: id("actuator"),
                },
                SecurityAlarmResponse::LockDoors {
                    actuator: id("actuator"),
                },
            ])
            .unwrap();
        let mut duplicate = blueprint(&[], GridPos::new(3, 3));
        duplicate.owner = Some(id("collective"));
        duplicate.installations[2].security_alarm_profile = Some(profile);
        assert!(matches!(
            FacilityState::validate_blueprint(&duplicate),
            Err(FacilityBuildError::DuplicateAlarmResponse { .. })
        ));
    }

    #[test]
    fn dependency_cycles_are_rejected_before_simulation() {
        let mut blueprint = blueprint(&[GridPos::new(5, 3)], GridPos::new(3, 3));
        blueprint.installations[0].dependencies = vec![id("sensor")];
        let mut map = map();
        let mut actors = actors(&[GridPos::new(5, 3)], GridPos::new(3, 3));

        assert!(matches!(
            FacilityState::instantiate(blueprint, &mut map, &mut actors),
            Err(FacilityBuildError::DependencyCycle(_))
        ));
    }

    #[test]
    fn duplicate_installation_positions_and_worker_mismatches_are_rejected() {
        let mut duplicate = blueprint(&[GridPos::new(5, 3)], GridPos::new(3, 3));
        duplicate.installations[1].position = duplicate.installations[0].position;
        assert!(matches!(
            FacilityState::validate_blueprint(&duplicate),
            Err(FacilityBuildError::DuplicateInstallationPosition(_))
        ));

        let mut map = map();
        let mut actors = actors(&[GridPos::new(5, 3)], GridPos::new(3, 3));
        let mut mismatch = blueprint(&[GridPos::new(5, 3)], GridPos::new(3, 3));
        mismatch.workers[0].maximum_integrity = 11;
        assert!(matches!(
            FacilityState::instantiate(mismatch, &mut map, &mut actors),
            Err(FacilityBuildError::WorkerIntegrityMismatch { .. })
        ));
    }

    #[test]
    fn operational_navigation_beacons_are_detected_without_revealing_other_installations() {
        let mut definition = blueprint(&[], GridPos::new(3, 3));
        definition.installations[0].integrity = 10;
        definition.installations[2]
            .capabilities
            .push(InstallationCapability::NavigationBeacon { range: 6 });
        let sensor = definition.installations[2].id.clone();
        let mut map = map();
        let mut actors = actors(&[], GridPos::new(3, 3));
        let mut facility = FacilityState::instantiate(definition, &mut map, &mut actors).unwrap();

        assert_eq!(
            facility.detected_navigation_signals(GridPos::new(3, 3)),
            vec![DetectedNavigationSignal {
                installation: sensor.clone(),
                position: GridPos::new(9, 1),
                distance: 6,
            }]
        );
        assert!(
            facility
                .detected_navigation_signals(GridPos::new(2, 3))
                .is_empty()
        );

        facility.apply_damage(&sensor, 10, &mut map).unwrap();
        assert!(
            facility
                .detected_navigation_signals(GridPos::new(3, 3))
                .is_empty()
        );
    }

    #[test]
    fn data_terminals_remember_first_access_and_follow_power_dependencies() {
        let mut definition = blueprint(&[], GridPos::new(3, 3));
        definition.installations[0].integrity = 10;
        let relay = definition.installations[0].id.clone();
        let terminal = id("archive_terminal");
        let record = id("archive_record");
        definition.installations.push(InstallationBlueprint {
            id: terminal.clone(),
            position: GridPos::new(4, 1),
            maximum_integrity: 10,
            integrity: 10,
            capabilities: vec![InstallationCapability::DataTerminal {
                record: record.clone(),
            }],
            dependencies: vec![relay.clone()],
            security_alarm_profile: None,
        });
        let mut map = map();
        let mut actors = actors(&[], GridPos::new(3, 3));
        let mut facility = FacilityState::instantiate(definition, &mut map, &mut actors).unwrap();

        assert!(facility.is_player_interactive_at(GridPos::new(4, 1)));
        assert_eq!(
            facility.access_data_terminal(GridPos::new(4, 1)),
            Some(DataTerminalAccess {
                installation: terminal.clone(),
                record: record.clone(),
                first_access: true,
            })
        );
        assert_eq!(
            facility.access_data_terminal(GridPos::new(4, 1)),
            Some(DataTerminalAccess {
                installation: terminal.clone(),
                record: record.clone(),
                first_access: false,
            })
        );
        assert!(facility.data_terminal_was_accessed(&terminal));
        assert_eq!(
            facility
                .accessed_data_terminal_records()
                .collect::<Vec<_>>(),
            vec![&record]
        );

        let revised_record = id("revised_archive_record");
        assert_eq!(
            facility.update_data_terminal_record(&terminal, revised_record.clone()),
            Ok(record.clone())
        );
        assert!(facility.data_terminal_was_updated(&terminal));
        assert!(!facility.data_terminal_was_accessed(&terminal));
        assert_eq!(
            facility
                .accessed_data_terminal_records()
                .collect::<Vec<_>>(),
            vec![&record]
        );
        assert_eq!(
            facility.access_data_terminal(GridPos::new(4, 1)),
            Some(DataTerminalAccess {
                installation: terminal.clone(),
                record: revised_record.clone(),
                first_access: true,
            })
        );
        assert_eq!(
            facility
                .accessed_data_terminal_records()
                .collect::<Vec<_>>(),
            vec![&record, &revised_record]
        );

        facility.apply_damage(&relay, 10, &mut map).unwrap();
        assert_eq!(facility.access_data_terminal(GridPos::new(4, 1)), None);
        assert!(facility.is_player_interactive_at(GridPos::new(4, 1)));
    }

    #[test]
    fn navigation_beacon_ranges_are_bounded_at_content_validation() {
        for range in [0, MAX_NAVIGATION_BEACON_RANGE + 1] {
            let mut definition = blueprint(&[], GridPos::new(3, 3));
            definition.installations[2]
                .capabilities
                .push(InstallationCapability::NavigationBeacon { range });
            assert!(matches!(
                FacilityState::validate_blueprint(&definition),
                Err(FacilityBuildError::InvalidNavigationBeaconRange(_))
            ));
        }
    }

    #[test]
    fn one_installation_cannot_silently_discard_a_second_terminal_record() {
        let mut definition = blueprint(&[], GridPos::new(3, 3));
        definition.installations[2].capabilities.extend([
            InstallationCapability::DataTerminal {
                record: id("record_a"),
            },
            InstallationCapability::DataTerminal {
                record: id("record_b"),
            },
        ]);
        assert!(matches!(
            FacilityState::validate_blueprint(&definition),
            Err(FacilityBuildError::MultipleDataTerminalRecords(_))
        ));
    }
}
