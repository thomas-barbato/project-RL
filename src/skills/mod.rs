use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::combat::{AttackDelivery, MeleeArc};
use crate::content::ContentId;
use crate::effects::ApplyStatusEffect;
use crate::explosive::ExplosivePayloadProfile;
use crate::item::{ItemCatalog, ItemId};
use crate::stats::{PrimaryAttribute, PrimaryAttributes};
use crate::status::{StatusCatalog, StatusFamilyId, StatusId};
use crate::stealth::SignatureChannel;
use crate::time::{ActionKind, TimeUnits};

mod progression;

pub use progression::{
    SkillProgressionState, SkillProgressionStateError, TechniqueLearned, TechniqueLearningError,
};

pub type DisciplineId = ContentId;
pub type TechniqueId = ContentId;
pub type SystemFeatureId = ContentId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TechniqueAttributeRequirement {
    attribute: PrimaryAttribute,
    minimum: u8,
}

impl TechniqueAttributeRequirement {
    pub fn new(attribute: PrimaryAttribute, minimum: u8) -> Result<Self, SkillDefinitionError> {
        if minimum == 0 {
            return Err(SkillDefinitionError::ZeroMinimumAttribute);
        }
        Ok(Self { attribute, minimum })
    }

    pub const fn attribute(self) -> PrimaryAttribute {
        self.attribute
    }

    pub const fn minimum(self) -> u8 {
        self.minimum
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SystemFeatureSet {
    enabled: BTreeSet<SystemFeatureId>,
}

impl SystemFeatureSet {
    pub fn new(features: impl IntoIterator<Item = SystemFeatureId>) -> Self {
        Self {
            enabled: features.into_iter().collect(),
        }
    }

    pub fn contains(&self, feature: &SystemFeatureId) -> bool {
        self.enabled.contains(feature)
    }

    pub fn iter(&self) -> impl Iterator<Item = &SystemFeatureId> {
        self.enabled.iter()
    }

    pub fn without(&self, feature: &SystemFeatureId) -> Self {
        let mut set = self.clone();
        set.enabled.remove(feature);
        set
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TechniqueKind {
    Action,
    Posture,
    Procedure,
    Behavior,
    Improvement,
}

/// Optional post-hit displacement attached to a weapon technique.
/// Force is derived from the weapon's capped melee Impact; the technique can
/// only add its explicit modifier and requested distance.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ForcedMovement {
    distance: u8,
    impact_modifier: i16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TechniqueMaterialCost {
    item: ItemId,
    quantity: u16,
}

/// Intrinsic resources consumed when a learned technique manifests an effect.
///
/// This is deliberately separate from inventory materials: a technique with
/// this profile remains usable without a matching item or tool. Persistent
/// bandwidth is released when the manifested device or drone leaves play.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TechniqueActivationCost {
    energy: u16,
    heat: u16,
    persistent_bandwidth: u16,
    active_limit: Option<u8>,
}

impl TechniqueActivationCost {
    pub const fn new(
        energy: u16,
        heat: u16,
        persistent_bandwidth: u16,
        active_limit: Option<u8>,
    ) -> Result<Self, SkillDefinitionError> {
        if matches!(active_limit, Some(0)) {
            return Err(SkillDefinitionError::ZeroMaximumTargets);
        }
        Ok(Self {
            energy,
            heat,
            persistent_bandwidth,
            active_limit,
        })
    }

    pub const fn energy(self) -> u16 {
        self.energy
    }

    pub const fn heat(self) -> u16 {
        self.heat
    }

    pub const fn persistent_bandwidth(self) -> u16 {
        self.persistent_bandwidth
    }

    pub const fn active_limit(self) -> Option<u8> {
        self.active_limit
    }
}

impl TechniqueMaterialCost {
    pub fn new(item: ItemId, quantity: u16) -> Result<Self, SkillDefinitionError> {
        if quantity == 0 {
            return Err(SkillDefinitionError::ZeroMaterialQuantity);
        }
        Ok(Self { item, quantity })
    }

    pub const fn item(&self) -> &ItemId {
        &self.item
    }

    pub const fn quantity(&self) -> u16 {
        self.quantity
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExplosivePlacementTarget {
    KnownCell,
    FreeCell,
    DestructibleOccupant,
    StructuralSupport { maximum_cells: u8 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExplosiveDeployment {
    ThrownImpact {
        range: u16,
        exact_placement_modifier: i16,
    },
    AdjacentTimed {
        delay_turns: u16,
        target: ExplosivePlacementTarget,
    },
    AdjacentProximity {
        arming_delay_turns: u16,
        trigger_radius: u16,
    },
    AdjacentRemote {
        maximum_link_range: u16,
        target: ExplosivePlacementTarget,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SecondaryExplosivePayload {
    delay_after_first: u16,
    payload: ExplosivePayloadProfile,
}

impl SecondaryExplosivePayload {
    pub const fn new(delay_after_first: u16, payload: ExplosivePayloadProfile) -> Self {
        Self {
            delay_after_first,
            payload,
        }
    }

    pub const fn delay_after_first(self) -> u16 {
        self.delay_after_first
    }

    pub const fn payload(self) -> ExplosivePayloadProfile {
        self.payload
    }
}

impl ForcedMovement {
    pub const fn new(distance: u8, impact_modifier: i16) -> Self {
        Self {
            distance,
            impact_modifier,
        }
    }

    pub const fn distance(self) -> u8 {
        self.distance
    }

    pub const fn impact_modifier(self) -> i16 {
        self.impact_modifier
    }
}

/// Engine-supported behavior selected by data for an active technique.
///
/// The catalogue binds an ID to one of these reusable primitives; game logic
/// therefore never branches on a particular `core:*` technique identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TechniqueAction {
    AnalyzeTarget {
        range: u16,
    },
    /// Uses the analysis primitive and range of its learned prerequisite.
    AnalyzeMultipleTargets {
        maximum_targets: u8,
        energy_cost: u16,
    },
    ReadMovementTraces {
        radius: u16,
    },
    /// Examines currently observable cells for already-present hidden
    /// interactables. Discovery changes knowledge only; it never opens,
    /// disarms or collects the discovered object.
    InspectNearbySecrets {
        radius: u16,
        detection_bonus: i16,
    },
    AnalyzeNearbyWalls {
        radius: u16,
        maximum_tiles: u8,
    },
    AnalyzeThreat {
        range: u16,
    },
    /// Reads only energy-state components that the selected actor really owns.
    DiagnoseEnergy {
        range: u16,
        analysis_bonus: i16,
        energy_cost: u16,
    },
    RepairComponent {
        durability_restored: u16,
        energy_cost: u16,
    },
    SalvageComponent,
    DiagnoseComponent {
        analysis_bonus: i16,
        energy_cost: u16,
    },
    TuneModule {
        economy_output_percentage: u16,
        economy_energy_percentage: u16,
        power_output_percentage: u16,
        power_energy_percentage: u16,
    },
    EmergencyRepairComponent {
        durability_restored: u16,
    },
    OverclockModule {
        output_percentage: u16,
        usage_energy_percentage: u16,
        heat_per_use: u16,
        safe_heat_threshold: u16,
        maximum_heat_threshold: u16,
        duration_time_units: u16,
        activation_energy: u16,
        durability_damage_when_hot: u16,
    },
    BypassComponent {
        restored_output_percentage: u16,
        energy_cost: u16,
    },
    ReconditionModule {
        durability_restored: u16,
    },
    AssembleFieldBeacon {
        integrity: u16,
        battery_energy: u16,
        energy_per_phase: u16,
        noise_intensity: u16,
    },
    /// Executes the selected equipped weapon through its ordinary hit, cost
    /// and effect pipeline while applying declarative technique modifiers.
    WeaponAttack {
        required_delivery: AttackDelivery,
        physical_damage_percentage: Option<u16>,
        armor_penetration_bonus: u16,
        accuracy_modifier: i16,
        energy_cost: u16,
        recovery_time_units: Option<u16>,
        forced_movement: Option<ForcedMovement>,
        melee_arc: Option<MeleeArc>,
    },
    /// Resolves a finite number of ordinary projectiles through one compatible
    /// ranged weapon. Targets are ordered explicitly by the command; each
    /// receives one projectile before remaining projectiles return to the
    /// primary target.
    WeaponVolley {
        projectiles: u8,
        maximum_targets: u8,
        maximum_target_separation: Option<u16>,
        accuracy_modifier: i16,
        energy_cost: u16,
        requires_automatic_fire: bool,
    },
    /// Resolves ordinary weapon damage against one identified component's
    /// independent durability instead of also subtracting body hit points.
    WeaponComponentAttack {
        required_delivery: AttackDelivery,
        accuracy_modifier: i16,
        energy_cost: u16,
    },
    /// Fires one ordinary projectile into each cell of a small aimed line for
    /// several accepted stages. Stage continuity is persistent simulation
    /// state and is cancelled by any different normal action.
    WeaponBarrage {
        stages: u8,
        cells: u8,
        accuracy_modifier: i16,
        energy_cost_per_stage: u16,
        requires_automatic_fire: bool,
    },
    /// Prepares one ordinary ranged reaction against the first perceived enemy
    /// entering a data-defined line. A learned improvement may widen this to a
    /// 90-degree sector without adding range or another reaction.
    PrepareRangedOverwatch {
        maximum_line_cells: u8,
    },
    /// Prepares a one-use guard for the next admissible melee hit. The weapon
    /// capability is checked when the action is executed, not inferred here.
    PrepareMeleeParry {
        physical_reduction_percentage: u8,
        trigger_energy_cost: u16,
    },
    /// Prepares a one-use ordinary melee strike before another actor
    /// voluntarily leaves contact. Forced movement never opens this trigger.
    PrepareMeleeInterception,
    /// Consumes authored inventory material only when the final deployment
    /// step succeeds, then creates a persistent zone-owned device.
    DeployExplosive {
        deployment: ExplosiveDeployment,
        primary_payload: ExplosivePayloadProfile,
        secondary_payload: Option<SecondaryExplosivePayload>,
    },
    NeutralizeExplosive {
        range: u16,
        analysis_bonus: i16,
        energy_cost: u16,
    },
    RecoverNeutralizedExplosive {
        range: u16,
    },
    TriggerRemoteExplosive {
        range: u16,
        energy_cost: u16,
        bandwidth_required: u16,
    },
    ProgramExplosives {
        range: u16,
        maximum_devices: u8,
        minimum_delay: u16,
        maximum_delay: u16,
        energy_cost: u16,
        bandwidth_required: u16,
    },
    CautiousMove {
        interception_evasion_modifier: i16,
    },
    PrepareAnchor {
        displacement_resistance_bonus: u16,
    },
    TraverseSingleObstacle {
        maximum_distance: u16,
        energy_cost: u16,
    },
    ChargeAttack {
        minimum_advance: u8,
        maximum_advance: u8,
        physical_damage_percentage: u16,
        energy_per_step: u16,
        recovery_time_units: u16,
    },
    PrepareEvasiveStep {
        trigger_energy_cost: u16,
    },
    PropelledMove {
        distance: u8,
        energy_cost: u16,
        heat_generated: u16,
    },
    Breakthrough {
        impact_modifier: i16,
        energy_cost: u16,
        recovery_time_units: u16,
    },
    ExtractAlly {
        energy_cost: u16,
    },
    /// Moves one cell using the ordinary movement pipeline while lowering
    /// only the acoustic signature of that step.
    SilentMove {
        noise_reduction: u16,
        minimum_time_units: u16,
    },
    ToggleEmissionSilence {
        channel: SignatureChannel,
    },
    ToggleLowProfile {
        optical_difficulty_bonus: i16,
        minimum_movement_time_units: u16,
    },
    AmbushAttack {
        accuracy_modifier: i16,
        physical_damage_percentage: u16,
    },
    DeploySoundDecoy {
        range: u16,
        intensity: u16,
        duration_phases: u16,
        integrity: u16,
    },
    BreakTrail {
        energy_cost: u16,
        maximum_steps: u8,
        maximum_duration: u16,
    },
    CamouflageExplosive {
        range: u16,
        optical_difficulty_bonus: i16,
    },
    ToggleActiveCamouflage {
        channel: SignatureChannel,
        optical_difficulty_bonus: i16,
        maximum_duration: u16,
        activation_energy: u16,
        upkeep_energy: u16,
        heat_per_phase: u16,
    },
    /// Creates a baseline physical drone on an adjacent world cell.
    ManifestDrone {
        integrity: u16,
        energy_capacity: u16,
        starting_energy: u16,
        link_range: u16,
        link_power: u16,
        link_difficulty: u16,
        link_attenuation_per_cell: u16,
        link_wall_attenuation_multiplier: u16,
        sensor_radius: u16,
        bandwidth_required: u16,
        movement_energy_cost: u16,
        manipulator_capacity_grams: u32,
        decoy_intensity: u16,
        attack_range: u16,
        attack_damage: u16,
    },
    DroneEscort {
        link_range: u16,
        minimum_distance: u8,
        maximum_distance: u8,
        energy_cost: u16,
    },
    DronePatrol {
        link_range: u16,
        maximum_waypoints: u8,
        energy_cost: u16,
    },
    DroneMobileDecoy {
        link_range: u16,
        controller_energy_cost: u16,
        drone_energy_per_phase: u16,
        intensity: u16,
        maximum_duration: u16,
    },
    DroneCollect {
        link_range: u16,
        energy_cost: u16,
    },
    DroneCoordinateFire {
        link_range: u16,
        maximum_drones: u8,
        energy_cost: u16,
        transmission_bandwidth: u16,
    },
    DroneInterpose {
        link_range: u16,
        controller_energy_cost: u16,
        drone_trigger_energy_cost: u16,
    },
    DroneConditionalRoutine {
        link_range: u16,
        energy_cost: u16,
        additional_bandwidth: u16,
    },
    DroneCoordinatedDeployment {
        link_range: u16,
        maximum_drones: u8,
        energy_cost: u16,
        transmission_bandwidth: u16,
    },
    DroneEmergencyReturn {
        link_range: u16,
        maximum_drones: u8,
        energy_cost: u16,
        transmission_bandwidth: u16,
        duration_phases: u16,
    },
    ProbeInterface {
        range: u16,
        analysis_bonus: i16,
        energy_cost: u16,
        audit_delay: u16,
    },
    ForceElectronicLock {
        range: u16,
        energy_cost: u16,
        bandwidth_required: u16,
        failure_hardening_duration: u16,
        audit_delay: u16,
    },
    ExtractData {
        range: u16,
        energy_cost: u16,
    },
    SpoofAuthorization {
        range: u16,
        energy_cost: u16,
        duration_time_units: u16,
    },
    DivertDevice {
        range: u16,
        energy_cost: u16,
        additional_bandwidth: u16,
        duration_time_units: u16,
    },
    SuspendDigitalRoutine {
        range: u16,
        energy_cost: u16,
        duration_time_units: u16,
        repeat_protection_time_units: u16,
    },
    MaintainBackdoor {
        range: u16,
        installation_energy_cost: u16,
        reconnection_energy_cost: u16,
        maximum_backdoors: u8,
        session_duration_time_units: u16,
    },
    FalsifySecurityTrace {
        range: u16,
        energy_cost: u16,
    },
    DivertSubnet {
        range: u16,
        maximum_devices: u8,
        energy_cost: u16,
        bandwidth_per_device: u16,
        duration_time_units: u16,
    },
    LockDeviceControl {
        range: u16,
        energy_cost: u16,
        additional_bandwidth: u16,
        duration_time_units: u16,
    },
    ElectronicPulse {
        radius: u16,
        damage: u16,
        energy_cost: u16,
        heat_generated: u16,
        disruption_intensity: u16,
        directional: bool,
        filter_identified_allies: bool,
        bandwidth_required: u16,
    },
    ImplantOverheat {
        range: u16,
        energy_cost: u16,
        heat_generated: u16,
        bandwidth_required: u16,
        heat_per_tick: u16,
        dissipation_penalty: u16,
        duration_time_units: u16,
        audit_delay: u16,
    },
    MaintainJamming {
        radius: u16,
        penalty: u16,
        activation_energy: u16,
        energy_per_phase: u16,
        heat_per_phase: u16,
        bandwidth_required: u16,
        maximum_duration: u16,
    },
    PurgeHostileProgram {
        range: u16,
        energy_cost: u16,
        intrusion_bonus: i16,
    },
    ElectronicCascade {
        range: u16,
        jump_range: u16,
        maximum_targets: u8,
        damage_by_target: [u16; 4],
        energy_cost: u16,
        heat_generated: u16,
    },
    ImplantInfection {
        range: u16,
        propagation_range: u16,
        energy_cost: u16,
        heat_generated: u16,
        bandwidth_required: u16,
        thermal_damage_per_tick: u16,
        ticks_per_host: u16,
        maximum_hosts: u8,
        transmissions_per_host: u8,
        campaign_duration: u16,
        audit_delay: u16,
    },
    DeploySaturationBeacon {
        radius: u16,
        damage: u16,
        duration_time_units: u16,
        integrity: u16,
        battery_energy: u16,
        energy_per_phase: u16,
        manual_activation: bool,
        activation_energy: u16,
        activation_bandwidth: u16,
        activation_link_range: u16,
    },
    ImplantImplosion {
        range: u16,
        energy_cost: u16,
        heat_generated: u16,
        bandwidth_required: u16,
        minimum_stored_energy: u16,
        reserved_energy: u16,
        delay_time_units: u16,
        radius: u16,
        physical_damage: u16,
        thermal_damage: u16,
        audit_delay: u16,
    },
}

impl TechniqueAction {
    pub const fn is_explosive_action(self) -> bool {
        matches!(
            self,
            Self::DeployExplosive { .. }
                | Self::NeutralizeExplosive { .. }
                | Self::RecoverNeutralizedExplosive { .. }
                | Self::TriggerRemoteExplosive { .. }
                | Self::ProgramExplosives { .. }
        )
    }

    pub const fn is_movement_aim_action(self) -> bool {
        matches!(
            self,
            Self::CautiousMove { .. }
                | Self::TraverseSingleObstacle { .. }
                | Self::PrepareEvasiveStep { .. }
                | Self::PropelledMove { .. }
                | Self::SilentMove { .. }
        )
    }

    pub const fn is_stealth_world_aim_action(self) -> bool {
        matches!(
            self,
            Self::DeploySoundDecoy { .. } | Self::CamouflageExplosive { .. }
        )
    }

    /// Operations which necessarily transmit or maintain an active control
    /// link. A silenced emission channel rejects these operations explicitly.
    pub const fn requires_active_emission(self) -> bool {
        matches!(
            self,
            Self::TriggerRemoteExplosive { .. }
                | Self::ProgramExplosives { .. }
                | Self::DroneEscort { .. }
                | Self::DronePatrol { .. }
                | Self::DroneMobileDecoy { .. }
                | Self::DroneCollect { .. }
                | Self::DroneCoordinateFire { .. }
                | Self::DroneInterpose { .. }
                | Self::DroneConditionalRoutine { .. }
                | Self::DroneCoordinatedDeployment { .. }
                | Self::DroneEmergencyReturn { .. }
                | Self::ProbeInterface { .. }
                | Self::ForceElectronicLock { .. }
                | Self::ExtractData { .. }
                | Self::SpoofAuthorization { .. }
                | Self::DivertDevice { .. }
                | Self::SuspendDigitalRoutine { .. }
                | Self::MaintainBackdoor { .. }
                | Self::FalsifySecurityTrace { .. }
                | Self::DivertSubnet { .. }
                | Self::LockDeviceControl { .. }
                | Self::ElectronicPulse { .. }
                | Self::ImplantOverheat { .. }
                | Self::MaintainJamming { .. }
                | Self::PurgeHostileProgram { .. }
                | Self::ElectronicCascade { .. }
                | Self::ImplantInfection { .. }
                | Self::DeploySaturationBeacon { .. }
                | Self::ImplantImplosion { .. }
        )
    }

    pub const fn is_drone_action(self) -> bool {
        matches!(
            self,
            Self::ManifestDrone { .. }
                | Self::DroneEscort { .. }
                | Self::DronePatrol { .. }
                | Self::DroneMobileDecoy { .. }
                | Self::DroneCollect { .. }
                | Self::DroneCoordinateFire { .. }
                | Self::DroneInterpose { .. }
                | Self::DroneConditionalRoutine { .. }
                | Self::DroneCoordinatedDeployment { .. }
                | Self::DroneEmergencyReturn { .. }
        )
    }

    pub const fn is_engineering_action(self) -> bool {
        matches!(
            self,
            Self::RepairComponent { .. }
                | Self::SalvageComponent
                | Self::DiagnoseComponent { .. }
                | Self::TuneModule { .. }
                | Self::EmergencyRepairComponent { .. }
                | Self::OverclockModule { .. }
                | Self::BypassComponent { .. }
                | Self::ReconditionModule { .. }
                | Self::AssembleFieldBeacon { .. }
        )
    }

    pub const fn is_intrusion_action(self) -> bool {
        matches!(
            self,
            Self::ProbeInterface { .. }
                | Self::ForceElectronicLock { .. }
                | Self::ExtractData { .. }
                | Self::SpoofAuthorization { .. }
                | Self::DivertDevice { .. }
                | Self::SuspendDigitalRoutine { .. }
                | Self::MaintainBackdoor { .. }
                | Self::FalsifySecurityTrace { .. }
                | Self::DivertSubnet { .. }
                | Self::LockDeviceControl { .. }
        )
    }

    pub const fn is_electronic_warfare_action(self) -> bool {
        matches!(
            self,
            Self::ElectronicPulse { .. }
                | Self::ImplantOverheat { .. }
                | Self::MaintainJamming { .. }
                | Self::PurgeHostileProgram { .. }
                | Self::ElectronicCascade { .. }
                | Self::ImplantInfection { .. }
                | Self::DeploySaturationBeacon { .. }
                | Self::ImplantImplosion { .. }
        )
    }

    pub const fn is_same_electronic_family(self, other: Self) -> bool {
        matches!(
            (self, other),
            (Self::ElectronicPulse { .. }, Self::ElectronicPulse { .. })
                | (Self::ImplantOverheat { .. }, Self::ImplantOverheat { .. })
                | (
                    Self::ElectronicCascade { .. },
                    Self::ElectronicCascade { .. }
                )
                | (Self::ImplantInfection { .. }, Self::ImplantInfection { .. })
                | (
                    Self::DeploySaturationBeacon { .. },
                    Self::DeploySaturationBeacon { .. }
                )
        )
    }
}

/// Engine-supported passive behavior granted by an improvement technique.
///
/// Improvements remain distinct from active actions: learning one changes a
/// compatible prerequisite without creating a second command in the UI.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TechniqueImprovement {
    /// Adds one ordinary melee weapon strike after a successful prepared parry.
    MeleeCounterattack,
    ExtendedRangedOverwatch,
    PersistentRangedAim {
        retained_accuracy_modifier: i16,
    },
    ControlledChargeInertia,
    CoveredApproach {
        optical_difficulty_bonus: i16,
    },
    SilentNeutralization {
        physical_damage_percentage: u16,
        extra_energy_cost: u16,
        noise_reduction: u16,
    },
    DroneAutonomousScout {
        maximum_unknown_steps: u8,
        energy_cost_override: u16,
        additional_bandwidth: u16,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TechniqueTargetRequirement {
    HasArmor,
    HasCompatibleLocomotion,
    HasCompatibleSuppressionResponse,
}

/// Condition checked before a targeted technique commits time, energy or RNG.
///
/// Status families keep the condition open to terrain, allies and modded
/// effects instead of coupling a technique to one concrete status ID.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TechniqueEngagementRequirement {
    TargetHasAnyStatusFamily(BTreeSet<StatusFamilyId>),
    TargetHasKnownPhysicalWeakness,
}

impl TechniqueEngagementRequirement {
    pub fn target_has_any_status_family(
        families: impl IntoIterator<Item = StatusFamilyId>,
    ) -> Result<Self, SkillDefinitionError> {
        let families = families.into_iter().collect::<BTreeSet<_>>();
        if families.is_empty() {
            return Err(SkillDefinitionError::EmptyEngagementStatusFamilies);
        }
        Ok(Self::TargetHasAnyStatusFamily(families))
    }

    pub fn status_families(&self) -> Vec<&StatusFamilyId> {
        match self {
            Self::TargetHasAnyStatusFamily(families) => families.iter().collect(),
            Self::TargetHasKnownPhysicalWeakness => Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TechniqueEffectResistance {
    Stability { intensity: u16 },
}

/// Reusable effect attached to a successful weapon-technique hit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TechniqueOnHitEffect {
    application: ApplyStatusEffect,
    target_requirement: TechniqueTargetRequirement,
    resistance: Option<TechniqueEffectResistance>,
}

impl TechniqueOnHitEffect {
    pub fn new(
        application: ApplyStatusEffect,
        target_requirement: TechniqueTargetRequirement,
    ) -> Self {
        Self {
            application,
            target_requirement,
            resistance: None,
        }
    }

    pub fn with_stability_resistance(
        mut self,
        intensity: u16,
    ) -> Result<Self, SkillDefinitionError> {
        if intensity == 0 {
            return Err(SkillDefinitionError::ZeroEffectIntensity);
        }
        self.resistance = Some(TechniqueEffectResistance::Stability { intensity });
        Ok(self)
    }

    pub const fn application(&self) -> &ApplyStatusEffect {
        &self.application
    }

    pub const fn target_requirement(&self) -> TechniqueTargetRequirement {
        self.target_requirement
    }

    pub const fn resistance(&self) -> Option<TechniqueEffectResistance> {
        self.resistance
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisciplineDefinition {
    id: DisciplineId,
    name_key: String,
    description_key: String,
}

impl DisciplineDefinition {
    pub fn new(
        id: DisciplineId,
        name_key: String,
        description_key: String,
    ) -> Result<Self, SkillDefinitionError> {
        validate_text_keys(&name_key, &description_key)?;
        Ok(Self {
            id,
            name_key,
            description_key,
        })
    }

    pub const fn id(&self) -> &DisciplineId {
        &self.id
    }

    pub fn name_key(&self) -> &str {
        &self.name_key
    }

    pub fn description_key(&self) -> &str {
        &self.description_key
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct TechniqueDefinition {
    id: TechniqueId,
    discipline: DisciplineId,
    name_key: String,
    description_key: String,
    minimum_level: u16,
    minimum_attributes: BTreeMap<PrimaryAttribute, u8>,
    kind: TechniqueKind,
    prerequisite: Option<TechniqueId>,
    required_features: BTreeSet<SystemFeatureId>,
    action: Option<TechniqueAction>,
    activation_cost: Option<TechniqueActivationCost>,
    manifestation_item: Option<ItemId>,
    manifestation_profile: Option<ContentId>,
    material_cost: Option<TechniqueMaterialCost>,
    additional_material_costs: Vec<TechniqueMaterialCost>,
    required_tool: Option<ItemId>,
    produced_item: Option<ItemId>,
    improvement: Option<TechniqueImprovement>,
    on_hit_effect: Option<TechniqueOnHitEffect>,
    engagement_requirement: Option<TechniqueEngagementRequirement>,
    action_kind: ActionKind,
    preparation_steps: Option<TimeUnits>,
    cooldown: Option<TimeUnits>,
}

// Keep the historical shape unchanged while no preparation is declared. Run
// suspension fingerprints are based on Debug and old generations must not be
// invalidated merely because the engine learned a new optional rule.
impl Debug for TechniqueDefinition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut definition = formatter.debug_struct("TechniqueDefinition");
        definition
            .field("id", &self.id)
            .field("discipline", &self.discipline)
            .field("name_key", &self.name_key)
            .field("description_key", &self.description_key)
            // Keep this historical Debug label while its value has the same
            // numeric shape. Old suspension fingerprints must not change just
            // because the requirement now refers to character level.
            .field("minimum_rank", &self.minimum_level)
            .field("kind", &self.kind)
            .field("prerequisite", &self.prerequisite)
            .field("required_features", &self.required_features)
            .field("action", &self.action);
        if let Some(cost) = self.activation_cost {
            definition.field("activation_cost", &cost);
        }
        if let Some(item) = &self.manifestation_item {
            definition.field("manifestation_item", item);
        }
        if let Some(profile) = &self.manifestation_profile {
            definition.field("manifestation_profile", profile);
        }
        if let Some(material_cost) = &self.material_cost {
            definition.field("material_cost", material_cost);
        }
        if !self.additional_material_costs.is_empty() {
            definition.field("additional_material_costs", &self.additional_material_costs);
        }
        if let Some(required_tool) = &self.required_tool {
            definition.field("required_tool", required_tool);
        }
        if let Some(produced_item) = &self.produced_item {
            definition.field("produced_item", produced_item);
        }
        if let Some(improvement) = self.improvement {
            definition.field("improvement", &improvement);
        }
        if let Some(on_hit_effect) = &self.on_hit_effect {
            definition.field("on_hit_effect", on_hit_effect);
        }
        if let Some(requirement) = &self.engagement_requirement {
            definition.field("engagement_requirement", requirement);
        }
        if let Some(preparation_steps) = self.preparation_steps {
            definition.field("preparation_steps", &preparation_steps);
        }
        if let Some(cooldown) = self.cooldown {
            definition.field("cooldown", &cooldown);
        }
        if !self.minimum_attributes.is_empty() {
            definition.field("minimum_attributes", &self.minimum_attributes);
        }
        if self.action_kind != ActionKind::Support {
            definition.field("action_kind", &self.action_kind);
        }
        definition.finish()
    }
}

impl TechniqueDefinition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: TechniqueId,
        discipline: DisciplineId,
        name_key: String,
        description_key: String,
        minimum_level: u16,
        kind: TechniqueKind,
        prerequisite: Option<TechniqueId>,
        required_features: impl IntoIterator<Item = SystemFeatureId>,
    ) -> Result<Self, SkillDefinitionError> {
        validate_text_keys(&name_key, &description_key)?;
        if minimum_level == 0 {
            return Err(SkillDefinitionError::ZeroMinimumLevel);
        }
        if prerequisite.as_ref() == Some(&id) {
            return Err(SkillDefinitionError::SelfPrerequisite);
        }
        Ok(Self {
            id,
            discipline,
            name_key,
            description_key,
            minimum_level,
            minimum_attributes: BTreeMap::new(),
            kind,
            prerequisite,
            required_features: required_features.into_iter().collect(),
            action: None,
            activation_cost: None,
            manifestation_item: None,
            manifestation_profile: None,
            material_cost: None,
            additional_material_costs: Vec::new(),
            required_tool: None,
            produced_item: None,
            improvement: None,
            on_hit_effect: None,
            engagement_requirement: None,
            action_kind: ActionKind::Support,
            preparation_steps: None,
            cooldown: None,
        })
    }

    pub fn with_attribute_requirements(
        mut self,
        requirements: impl IntoIterator<Item = TechniqueAttributeRequirement>,
    ) -> Result<Self, SkillDefinitionError> {
        for requirement in requirements {
            if self
                .minimum_attributes
                .insert(requirement.attribute(), requirement.minimum())
                .is_some()
            {
                return Err(SkillDefinitionError::DuplicateMinimumAttribute(
                    requirement.attribute(),
                ));
            }
        }
        Ok(self)
    }

    pub fn with_action(mut self, action: TechniqueAction) -> Result<Self, SkillDefinitionError> {
        if self.improvement.is_some() {
            return Err(SkillDefinitionError::ActionAndImprovement);
        }
        if self.on_hit_effect.is_some()
            && !matches!(
                action,
                TechniqueAction::WeaponAttack { .. } | TechniqueAction::WeaponVolley { .. }
            )
        {
            return Err(SkillDefinitionError::OnHitEffectWithoutWeaponAttack);
        }
        let analysis_improvement = matches!(action, TechniqueAction::AnalyzeMultipleTargets { .. });
        let active_improvement = self.kind == TechniqueKind::Improvement
            && matches!(
                action,
                TechniqueAction::RecoverNeutralizedExplosive { .. }
                    | TechniqueAction::EmergencyRepairComponent { .. }
                    | TechniqueAction::OverclockModule { .. }
            )
            || (self.kind == TechniqueKind::Improvement && action.is_electronic_warfare_action());
        if analysis_improvement
            && (self.kind != TechniqueKind::Improvement || self.prerequisite.is_none())
        {
            return Err(SkillDefinitionError::InvalidAnalysisImprovement);
        }
        if active_improvement
            && (self.kind != TechniqueKind::Improvement || self.prerequisite.is_none())
        {
            return Err(SkillDefinitionError::InvalidActiveImprovement);
        }
        if !analysis_improvement
            && !active_improvement
            && !matches!(
                self.kind,
                TechniqueKind::Action | TechniqueKind::Posture | TechniqueKind::Procedure
            )
        {
            return Err(SkillDefinitionError::ActionOnNonActionTechnique);
        }
        match action {
            TechniqueAction::AnalyzeMultipleTargets {
                maximum_targets: 0, ..
            } => {
                return Err(SkillDefinitionError::ZeroMaximumTargets);
            }
            TechniqueAction::AnalyzeTarget { range: 0 } => {
                return Err(SkillDefinitionError::ZeroActionRange);
            }
            TechniqueAction::ReadMovementTraces { radius: 0 } => {
                return Err(SkillDefinitionError::ZeroActionRange);
            }
            TechniqueAction::InspectNearbySecrets { radius: 0, .. } => {
                return Err(SkillDefinitionError::ZeroActionRange);
            }
            TechniqueAction::AnalyzeNearbyWalls { radius: 0, .. } => {
                return Err(SkillDefinitionError::ZeroActionRange);
            }
            TechniqueAction::AnalyzeNearbyWalls {
                maximum_tiles: 0, ..
            } => {
                return Err(SkillDefinitionError::ZeroMaximumTargets);
            }
            TechniqueAction::AnalyzeThreat { range: 0 } => {
                return Err(SkillDefinitionError::ZeroActionRange);
            }
            TechniqueAction::DiagnoseEnergy { range: 0, .. } => {
                return Err(SkillDefinitionError::ZeroActionRange);
            }
            TechniqueAction::RepairComponent {
                durability_restored: 0,
                ..
            }
            | TechniqueAction::EmergencyRepairComponent {
                durability_restored: 0,
            }
            | TechniqueAction::ReconditionModule {
                durability_restored: 0,
            } => return Err(SkillDefinitionError::ZeroDurabilityRestored),
            TechniqueAction::TuneModule {
                economy_output_percentage: 0,
                ..
            }
            | TechniqueAction::TuneModule {
                economy_energy_percentage: 0,
                ..
            }
            | TechniqueAction::TuneModule {
                power_output_percentage: 0,
                ..
            }
            | TechniqueAction::TuneModule {
                power_energy_percentage: 0,
                ..
            }
            | TechniqueAction::OverclockModule {
                output_percentage: 0,
                ..
            }
            | TechniqueAction::OverclockModule {
                usage_energy_percentage: 0,
                ..
            }
            | TechniqueAction::BypassComponent {
                restored_output_percentage: 0,
                ..
            } => return Err(SkillDefinitionError::ZeroOutputPercentage),
            TechniqueAction::OverclockModule {
                duration_time_units: 0,
                ..
            } => return Err(SkillDefinitionError::ZeroEffectDuration),
            TechniqueAction::OverclockModule {
                safe_heat_threshold,
                maximum_heat_threshold,
                ..
            } if safe_heat_threshold > maximum_heat_threshold => {
                return Err(SkillDefinitionError::InvalidHeatThresholds);
            }
            TechniqueAction::AssembleFieldBeacon { integrity: 0, .. } => {
                return Err(SkillDefinitionError::ZeroDeviceIntegrity);
            }
            TechniqueAction::AssembleFieldBeacon {
                battery_energy: 0, ..
            }
            | TechniqueAction::AssembleFieldBeacon {
                energy_per_phase: 0,
                ..
            } => return Err(SkillDefinitionError::ZeroEnergyAmount),
            TechniqueAction::AssembleFieldBeacon {
                noise_intensity: 0, ..
            } => return Err(SkillDefinitionError::ZeroSoundIntensity),
            TechniqueAction::WeaponAttack {
                physical_damage_percentage: Some(0),
                ..
            } => {
                return Err(SkillDefinitionError::ZeroPhysicalDamagePercentage);
            }
            TechniqueAction::WeaponAttack {
                recovery_time_units: Some(0),
                ..
            } => {
                return Err(SkillDefinitionError::ZeroActionRecovery);
            }
            TechniqueAction::WeaponAttack {
                forced_movement: Some(forced_movement),
                ..
            } if forced_movement.distance() == 0 => {
                return Err(SkillDefinitionError::ZeroForcedMovementDistance);
            }
            TechniqueAction::WeaponAttack {
                required_delivery,
                forced_movement: Some(_),
                ..
            } if required_delivery != AttackDelivery::Melee => {
                return Err(SkillDefinitionError::ForcedMovementRequiresMeleeDelivery);
            }
            TechniqueAction::WeaponAttack {
                required_delivery,
                melee_arc: Some(_),
                ..
            } if required_delivery != AttackDelivery::Melee => {
                return Err(SkillDefinitionError::MeleeArcRequiresMeleeDelivery);
            }
            TechniqueAction::WeaponAttack {
                forced_movement: Some(_),
                melee_arc: Some(_),
                ..
            } => {
                return Err(SkillDefinitionError::IncompatibleWeaponAttackModifiers);
            }
            TechniqueAction::WeaponVolley { projectiles: 0, .. } => {
                return Err(SkillDefinitionError::ZeroProjectiles);
            }
            TechniqueAction::WeaponBarrage { stages: 0, .. } => {
                return Err(SkillDefinitionError::ZeroBarrageStages);
            }
            TechniqueAction::WeaponBarrage { cells: 0, .. } => {
                return Err(SkillDefinitionError::ZeroMaximumTargets);
            }
            TechniqueAction::WeaponVolley {
                maximum_targets: 0, ..
            } => return Err(SkillDefinitionError::ZeroMaximumTargets),
            TechniqueAction::WeaponVolley {
                maximum_target_separation: Some(0),
                ..
            } => return Err(SkillDefinitionError::ZeroTargetSeparation),
            TechniqueAction::WeaponVolley {
                projectiles,
                maximum_targets,
                ..
            } if maximum_targets > projectiles => {
                return Err(SkillDefinitionError::MoreTargetsThanProjectiles {
                    projectiles,
                    targets: maximum_targets,
                });
            }
            TechniqueAction::PrepareRangedOverwatch {
                maximum_line_cells: 0,
            } => return Err(SkillDefinitionError::ZeroMaximumTargets),
            TechniqueAction::PrepareMeleeParry {
                physical_reduction_percentage,
                ..
            } if !(1..=100).contains(&physical_reduction_percentage) => {
                return Err(SkillDefinitionError::InvalidReactionPercentage(
                    physical_reduction_percentage,
                ));
            }
            TechniqueAction::DeployExplosive {
                primary_payload,
                secondary_payload,
                ..
            } if !explosive_payload_profile_is_valid(primary_payload)
                || secondary_payload.is_some_and(|secondary| {
                    !explosive_payload_profile_is_valid(secondary.payload())
                }) =>
            {
                return Err(SkillDefinitionError::InvalidExplosivePayload);
            }
            TechniqueAction::DeployExplosive {
                deployment:
                    ExplosiveDeployment::ThrownImpact { range: 0, .. }
                    | ExplosiveDeployment::AdjacentTimed { delay_turns: 0, .. }
                    | ExplosiveDeployment::AdjacentProximity {
                        arming_delay_turns: 0,
                        ..
                    }
                    | ExplosiveDeployment::AdjacentRemote {
                        maximum_link_range: 0,
                        ..
                    },
                ..
            } => return Err(SkillDefinitionError::ZeroActionRange),
            TechniqueAction::DeployExplosive {
                deployment:
                    ExplosiveDeployment::AdjacentProximity {
                        trigger_radius: 0, ..
                    },
                ..
            } => return Err(SkillDefinitionError::ZeroActionRange),
            TechniqueAction::DeployExplosive {
                deployment:
                    ExplosiveDeployment::AdjacentTimed {
                        target: ExplosivePlacementTarget::StructuralSupport { maximum_cells: 0 },
                        ..
                    }
                    | ExplosiveDeployment::AdjacentRemote {
                        target: ExplosivePlacementTarget::StructuralSupport { maximum_cells: 0 },
                        ..
                    },
                ..
            } => return Err(SkillDefinitionError::ZeroMaximumTargets),
            TechniqueAction::DeployExplosive {
                secondary_payload: Some(secondary),
                ..
            } if secondary.delay_after_first() == 0 => {
                return Err(SkillDefinitionError::ZeroExplosiveStageDelay);
            }
            TechniqueAction::NeutralizeExplosive { range: 0, .. }
            | TechniqueAction::RecoverNeutralizedExplosive { range: 0 }
            | TechniqueAction::TriggerRemoteExplosive { range: 0, .. }
            | TechniqueAction::ProgramExplosives { range: 0, .. } => {
                return Err(SkillDefinitionError::ZeroActionRange);
            }
            TechniqueAction::ProgramExplosives {
                maximum_devices: 0, ..
            } => return Err(SkillDefinitionError::ZeroMaximumTargets),
            TechniqueAction::ProgramExplosives {
                minimum_delay,
                maximum_delay,
                ..
            } if minimum_delay == 0 || minimum_delay > maximum_delay => {
                return Err(SkillDefinitionError::InvalidExplosiveDelayRange);
            }
            TechniqueAction::TraverseSingleObstacle {
                maximum_distance: 0,
                ..
            }
            | TechniqueAction::PropelledMove { distance: 0, .. } => {
                return Err(SkillDefinitionError::ZeroActionRange);
            }
            TechniqueAction::ChargeAttack {
                minimum_advance,
                maximum_advance,
                ..
            } if minimum_advance == 0 || minimum_advance > maximum_advance => {
                return Err(SkillDefinitionError::InvalidMovementDistanceRange);
            }
            TechniqueAction::ChargeAttack {
                physical_damage_percentage: 0,
                ..
            } => return Err(SkillDefinitionError::ZeroPhysicalDamagePercentage),
            TechniqueAction::ChargeAttack {
                recovery_time_units: 0,
                ..
            }
            | TechniqueAction::Breakthrough {
                recovery_time_units: 0,
                ..
            } => return Err(SkillDefinitionError::ZeroActionRecovery),
            TechniqueAction::SilentMove {
                minimum_time_units: 0,
                ..
            }
            | TechniqueAction::ToggleLowProfile {
                minimum_movement_time_units: 0,
                ..
            } => return Err(SkillDefinitionError::ZeroMovementTime),
            TechniqueAction::AmbushAttack {
                physical_damage_percentage: 0,
                ..
            } => return Err(SkillDefinitionError::ZeroPhysicalDamagePercentage),
            TechniqueAction::DeploySoundDecoy { range: 0, .. }
            | TechniqueAction::CamouflageExplosive { range: 0, .. } => {
                return Err(SkillDefinitionError::ZeroActionRange);
            }
            TechniqueAction::DeploySoundDecoy { intensity: 0, .. } => {
                return Err(SkillDefinitionError::ZeroSoundIntensity);
            }
            TechniqueAction::DeploySoundDecoy {
                duration_phases: 0, ..
            }
            | TechniqueAction::ToggleActiveCamouflage {
                maximum_duration: 0,
                ..
            }
            | TechniqueAction::BreakTrail {
                maximum_duration: 0,
                ..
            } => return Err(SkillDefinitionError::ZeroEffectDuration),
            TechniqueAction::DeploySoundDecoy { integrity: 0, .. } => {
                return Err(SkillDefinitionError::ZeroDeviceIntegrity);
            }
            TechniqueAction::BreakTrail {
                maximum_steps: 0, ..
            } => return Err(SkillDefinitionError::ZeroMaximumTargets),
            TechniqueAction::ManifestDrone { integrity: 0, .. }
            | TechniqueAction::ManifestDrone {
                energy_capacity: 0, ..
            }
            | TechniqueAction::ManifestDrone {
                starting_energy: 0, ..
            }
            | TechniqueAction::ManifestDrone { link_range: 0, .. }
            | TechniqueAction::ManifestDrone { link_power: 0, .. }
            | TechniqueAction::ManifestDrone {
                link_difficulty: 0, ..
            }
            | TechniqueAction::ManifestDrone {
                link_attenuation_per_cell: 0,
                ..
            }
            | TechniqueAction::ManifestDrone {
                link_wall_attenuation_multiplier: 0,
                ..
            }
            | TechniqueAction::ManifestDrone {
                sensor_radius: 0, ..
            }
            | TechniqueAction::ManifestDrone {
                bandwidth_required: 0,
                ..
            }
            | TechniqueAction::ManifestDrone {
                manipulator_capacity_grams: 0,
                ..
            }
            | TechniqueAction::ManifestDrone {
                decoy_intensity: 0, ..
            }
            | TechniqueAction::ManifestDrone {
                attack_range: 0, ..
            }
            | TechniqueAction::ManifestDrone {
                attack_damage: 0, ..
            } => return Err(SkillDefinitionError::ZeroEnergyAmount),
            TechniqueAction::ManifestDrone {
                starting_energy,
                energy_capacity,
                ..
            } if starting_energy > energy_capacity => {
                return Err(SkillDefinitionError::ZeroEnergyAmount);
            }
            TechniqueAction::DroneEscort { link_range: 0, .. }
            | TechniqueAction::DronePatrol { link_range: 0, .. }
            | TechniqueAction::DroneMobileDecoy { link_range: 0, .. }
            | TechniqueAction::DroneCollect { link_range: 0, .. }
            | TechniqueAction::DroneCoordinateFire { link_range: 0, .. }
            | TechniqueAction::DroneInterpose { link_range: 0, .. }
            | TechniqueAction::DroneConditionalRoutine { link_range: 0, .. }
            | TechniqueAction::DroneCoordinatedDeployment { link_range: 0, .. }
            | TechniqueAction::DroneEmergencyReturn { link_range: 0, .. } => {
                return Err(SkillDefinitionError::ZeroActionRange);
            }
            TechniqueAction::DroneEscort {
                minimum_distance,
                maximum_distance,
                ..
            } if minimum_distance == 0 || minimum_distance > maximum_distance => {
                return Err(SkillDefinitionError::InvalidMovementDistanceRange);
            }
            TechniqueAction::DronePatrol {
                maximum_waypoints: 0,
                ..
            }
            | TechniqueAction::DroneCoordinateFire {
                maximum_drones: 0, ..
            }
            | TechniqueAction::DroneCoordinatedDeployment {
                maximum_drones: 0, ..
            }
            | TechniqueAction::DroneEmergencyReturn {
                maximum_drones: 0, ..
            } => return Err(SkillDefinitionError::ZeroMaximumTargets),
            TechniqueAction::DroneMobileDecoy { intensity: 0, .. } => {
                return Err(SkillDefinitionError::ZeroSoundIntensity);
            }
            TechniqueAction::DroneMobileDecoy {
                maximum_duration: 0,
                ..
            }
            | TechniqueAction::DroneEmergencyReturn {
                duration_phases: 0, ..
            } => return Err(SkillDefinitionError::ZeroEffectDuration),
            TechniqueAction::ProbeInterface { range: 0, .. }
            | TechniqueAction::ForceElectronicLock { range: 0, .. }
            | TechniqueAction::ExtractData { range: 0, .. }
            | TechniqueAction::SpoofAuthorization { range: 0, .. }
            | TechniqueAction::DivertDevice { range: 0, .. }
            | TechniqueAction::SuspendDigitalRoutine { range: 0, .. }
            | TechniqueAction::MaintainBackdoor { range: 0, .. }
            | TechniqueAction::FalsifySecurityTrace { range: 0, .. }
            | TechniqueAction::DivertSubnet { range: 0, .. }
            | TechniqueAction::LockDeviceControl { range: 0, .. } => {
                return Err(SkillDefinitionError::ZeroActionRange);
            }
            TechniqueAction::ProbeInterface { audit_delay: 0, .. }
            | TechniqueAction::ForceElectronicLock { audit_delay: 0, .. }
            | TechniqueAction::SpoofAuthorization {
                duration_time_units: 0,
                ..
            }
            | TechniqueAction::DivertDevice {
                duration_time_units: 0,
                ..
            }
            | TechniqueAction::SuspendDigitalRoutine {
                duration_time_units: 0,
                ..
            }
            | TechniqueAction::SuspendDigitalRoutine {
                repeat_protection_time_units: 0,
                ..
            }
            | TechniqueAction::MaintainBackdoor {
                session_duration_time_units: 0,
                ..
            }
            | TechniqueAction::DivertSubnet {
                duration_time_units: 0,
                ..
            }
            | TechniqueAction::LockDeviceControl {
                duration_time_units: 0,
                ..
            }
            | TechniqueAction::ImplantOverheat {
                duration_time_units: 0,
                ..
            }
            | TechniqueAction::MaintainJamming {
                maximum_duration: 0,
                ..
            }
            | TechniqueAction::ImplantInfection {
                campaign_duration: 0,
                ..
            }
            | TechniqueAction::DeploySaturationBeacon {
                duration_time_units: 0,
                ..
            }
            | TechniqueAction::ImplantImplosion {
                delay_time_units: 0,
                ..
            } => return Err(SkillDefinitionError::ZeroEffectDuration),
            TechniqueAction::MaintainBackdoor {
                maximum_backdoors: 0,
                ..
            }
            | TechniqueAction::DivertSubnet {
                maximum_devices: 0, ..
            }
            | TechniqueAction::ElectronicCascade {
                maximum_targets: 0, ..
            }
            | TechniqueAction::ImplantInfection {
                maximum_hosts: 0, ..
            } => return Err(SkillDefinitionError::ZeroMaximumTargets),
            TechniqueAction::ElectronicPulse { radius: 0, .. }
            | TechniqueAction::ImplantOverheat { range: 0, .. }
            | TechniqueAction::MaintainJamming { radius: 0, .. }
            | TechniqueAction::PurgeHostileProgram { range: 0, .. }
            | TechniqueAction::ElectronicCascade { range: 0, .. }
            | TechniqueAction::ElectronicCascade { jump_range: 0, .. }
            | TechniqueAction::ImplantInfection { range: 0, .. }
            | TechniqueAction::ImplantInfection {
                propagation_range: 0,
                ..
            }
            | TechniqueAction::DeploySaturationBeacon { radius: 0, .. }
            | TechniqueAction::DeploySaturationBeacon {
                manual_activation: true,
                activation_link_range: 0,
                ..
            }
            | TechniqueAction::ImplantImplosion { range: 0, .. }
            | TechniqueAction::ImplantImplosion { radius: 0, .. } => {
                return Err(SkillDefinitionError::ZeroActionRange);
            }
            TechniqueAction::ElectronicPulse { damage: 0, .. }
            | TechniqueAction::ElectronicCascade {
                damage_by_target: [0, _, _, _],
                ..
            }
            | TechniqueAction::ImplantInfection {
                thermal_damage_per_tick: 0,
                ..
            }
            | TechniqueAction::DeploySaturationBeacon { damage: 0, .. }
            | TechniqueAction::ImplantImplosion {
                physical_damage: 0, ..
            }
            | TechniqueAction::ImplantImplosion {
                thermal_damage: 0, ..
            } => return Err(SkillDefinitionError::InvalidExplosivePayload),
            TechniqueAction::ElectronicCascade {
                maximum_targets, ..
            } if maximum_targets > 4 => {
                return Err(SkillDefinitionError::ZeroMaximumTargets);
            }
            TechniqueAction::ElectronicCascade {
                maximum_targets,
                damage_by_target,
                ..
            } if damage_by_target[..usize::from(maximum_targets)].contains(&0) => {
                return Err(SkillDefinitionError::InvalidExplosivePayload);
            }
            TechniqueAction::ImplantInfection {
                ticks_per_host: 0, ..
            } => return Err(SkillDefinitionError::ZeroEffectDuration),
            TechniqueAction::ImplantInfection {
                transmissions_per_host: 0,
                maximum_hosts,
                ..
            } if maximum_hosts > 1 => return Err(SkillDefinitionError::ZeroMaximumTargets),
            TechniqueAction::DeploySaturationBeacon { integrity: 0, .. } => {
                return Err(SkillDefinitionError::ZeroDeviceIntegrity);
            }
            TechniqueAction::DeploySaturationBeacon {
                battery_energy: 0, ..
            }
            | TechniqueAction::DeploySaturationBeacon {
                energy_per_phase: 0,
                ..
            }
            | TechniqueAction::ImplantImplosion {
                minimum_stored_energy: 0,
                ..
            }
            | TechniqueAction::ImplantImplosion {
                reserved_energy: 0, ..
            } => return Err(SkillDefinitionError::ZeroEnergyAmount),
            TechniqueAction::ImplantImplosion {
                minimum_stored_energy,
                reserved_energy,
                ..
            } if reserved_energy > minimum_stored_energy => {
                return Err(SkillDefinitionError::ZeroEnergyAmount);
            }
            _ => {}
        }
        self.action = Some(action);
        if matches!(
            action,
            TechniqueAction::WeaponAttack { .. }
                | TechniqueAction::WeaponVolley { .. }
                | TechniqueAction::WeaponComponentAttack { .. }
                | TechniqueAction::WeaponBarrage { .. }
                | TechniqueAction::DeployExplosive { .. }
                | TechniqueAction::ChargeAttack { .. }
                | TechniqueAction::Breakthrough { .. }
                | TechniqueAction::AmbushAttack { .. }
                | TechniqueAction::ElectronicPulse { .. }
                | TechniqueAction::ImplantOverheat { .. }
                | TechniqueAction::ElectronicCascade { .. }
                | TechniqueAction::ImplantInfection { .. }
                | TechniqueAction::DeploySaturationBeacon { .. }
                | TechniqueAction::ImplantImplosion { .. }
        ) {
            self.action_kind = ActionKind::Offensive;
        }
        Ok(self)
    }

    pub fn with_material_cost(
        mut self,
        cost: TechniqueMaterialCost,
    ) -> Result<Self, SkillDefinitionError> {
        if !matches!(
            self.action,
            Some(
                TechniqueAction::DeployExplosive { .. }
                    | TechniqueAction::DeploySoundDecoy { .. }
                    | TechniqueAction::CamouflageExplosive { .. }
                    | TechniqueAction::RepairComponent { .. }
                    | TechniqueAction::TuneModule { .. }
                    | TechniqueAction::EmergencyRepairComponent { .. }
                    | TechniqueAction::ReconditionModule { .. }
                    | TechniqueAction::AssembleFieldBeacon { .. }
                    | TechniqueAction::DeploySaturationBeacon { .. }
            )
        ) {
            return Err(SkillDefinitionError::MaterialCostWithoutDeployment);
        }
        self.material_cost = Some(cost);
        Ok(self)
    }

    pub const fn with_activation_cost(mut self, cost: TechniqueActivationCost) -> Self {
        self.activation_cost = Some(cost);
        self
    }

    pub fn with_manifestation_item(mut self, item: ItemId) -> Self {
        self.manifestation_item = Some(item);
        self
    }

    pub fn with_manifestation_profile(mut self, profile: ContentId) -> Self {
        self.manifestation_profile = Some(profile);
        self
    }

    /// Adds another independently authored ingredient to a material-using
    /// technique. The legacy primary cost remains available to deployment
    /// code which uses its item as the produced device definition.
    pub fn with_additional_material_cost(
        mut self,
        cost: TechniqueMaterialCost,
    ) -> Result<Self, SkillDefinitionError> {
        if self.material_cost.is_none() {
            return Err(SkillDefinitionError::MaterialCostWithoutDeployment);
        }
        self.additional_material_costs.push(cost);
        Ok(self)
    }

    pub fn with_required_tool(mut self, tool: ItemId) -> Self {
        self.required_tool = Some(tool);
        self
    }

    pub fn with_produced_item(mut self, item: ItemId) -> Self {
        self.produced_item = Some(item);
        self
    }

    pub fn with_improvement(
        mut self,
        improvement: TechniqueImprovement,
    ) -> Result<Self, SkillDefinitionError> {
        let dependent = self.kind == TechniqueKind::Improvement && self.prerequisite.is_some();
        let standalone_behavior =
            self.kind == TechniqueKind::Behavior && self.prerequisite.is_none();
        if !dependent && !standalone_behavior {
            return Err(SkillDefinitionError::ImprovementOnIncompatibleTechnique);
        }
        if self.action.is_some() {
            return Err(SkillDefinitionError::ActionAndImprovement);
        }
        if matches!(
            improvement,
            TechniqueImprovement::DroneAutonomousScout {
                maximum_unknown_steps: 0,
                ..
            }
        ) {
            return Err(SkillDefinitionError::ZeroMaximumTargets);
        }
        self.improvement = Some(improvement);
        Ok(self)
    }

    pub fn with_on_hit_effect(
        mut self,
        effect: TechniqueOnHitEffect,
    ) -> Result<Self, SkillDefinitionError> {
        if !matches!(
            self.action,
            Some(
                TechniqueAction::WeaponAttack { .. }
                    | TechniqueAction::WeaponVolley { .. }
                    | TechniqueAction::WeaponComponentAttack { .. }
                    | TechniqueAction::WeaponBarrage { .. }
            )
        ) {
            return Err(SkillDefinitionError::OnHitEffectWithoutWeaponAttack);
        }
        self.on_hit_effect = Some(effect);
        Ok(self)
    }

    pub fn with_engagement_requirement(
        mut self,
        requirement: TechniqueEngagementRequirement,
    ) -> Result<Self, SkillDefinitionError> {
        if !matches!(self.action, Some(TechniqueAction::WeaponAttack { .. })) {
            return Err(SkillDefinitionError::EngagementRequirementWithoutWeaponAttack);
        }
        self.engagement_requirement = Some(requirement);
        Ok(self)
    }

    /// Declares Pn+A1 timing without coupling the scheduler to a specific
    /// technique ID. Every preparation step and the final execution each cost
    /// one normal simulation action.
    pub fn with_preparation_steps(
        mut self,
        preparation_steps: u16,
    ) -> Result<Self, SkillDefinitionError> {
        if self.action.is_none() {
            return Err(SkillDefinitionError::PreparationWithoutAction);
        }
        self.preparation_steps = Some(
            TimeUnits::new(preparation_steps)
                .map_err(|_| SkillDefinitionError::ZeroPreparationSteps)?,
        );
        Ok(self)
    }

    /// Blocks a new launch until this many environment phases have completed
    /// after execution. Preparation alone never arms the cooldown.
    pub fn with_cooldown(mut self, turns: u16) -> Result<Self, SkillDefinitionError> {
        if self.action.is_none() {
            return Err(SkillDefinitionError::CooldownWithoutAction);
        }
        self.cooldown =
            Some(TimeUnits::new(turns).map_err(|_| SkillDefinitionError::ZeroCooldown)?);
        Ok(self)
    }

    /// Declares whether executing or preparing this technique is offensive.
    /// Support remains the compatibility default for the existing analyses
    /// and guards.
    pub fn with_action_kind(
        mut self,
        action_kind: ActionKind,
    ) -> Result<Self, SkillDefinitionError> {
        if self.action.is_none() {
            return Err(SkillDefinitionError::ActionKindWithoutAction);
        }
        if matches!(
            self.action,
            Some(TechniqueAction::WeaponAttack { .. } | TechniqueAction::WeaponVolley { .. })
        ) && action_kind != ActionKind::Offensive
        {
            return Err(SkillDefinitionError::WeaponAttackMustBeOffensive);
        }
        self.action_kind = action_kind;
        Ok(self)
    }

    pub const fn id(&self) -> &TechniqueId {
        &self.id
    }

    pub const fn discipline(&self) -> &DisciplineId {
        &self.discipline
    }

    pub fn name_key(&self) -> &str {
        &self.name_key
    }

    pub fn description_key(&self) -> &str {
        &self.description_key
    }

    pub const fn minimum_level(&self) -> u16 {
        self.minimum_level
    }

    pub fn minimum_attributes(&self) -> impl Iterator<Item = TechniqueAttributeRequirement> + '_ {
        self.minimum_attributes
            .iter()
            .map(|(attribute, minimum)| TechniqueAttributeRequirement {
                attribute: *attribute,
                minimum: *minimum,
            })
    }

    pub fn unmet_attribute_requirement(
        &self,
        attributes: Option<PrimaryAttributes>,
    ) -> Option<TechniqueAttributeRequirement> {
        self.minimum_attributes().find(|requirement| {
            attributes.map_or(0, |values| values.value(requirement.attribute()))
                < requirement.minimum()
        })
    }

    pub const fn kind(&self) -> TechniqueKind {
        self.kind
    }

    pub const fn prerequisite(&self) -> Option<&TechniqueId> {
        self.prerequisite.as_ref()
    }

    pub fn required_features(&self) -> impl Iterator<Item = &SystemFeatureId> {
        self.required_features.iter()
    }

    pub const fn action(&self) -> Option<TechniqueAction> {
        self.action
    }

    pub const fn material_cost(&self) -> Option<&TechniqueMaterialCost> {
        self.material_cost.as_ref()
    }

    pub const fn activation_cost(&self) -> Option<TechniqueActivationCost> {
        self.activation_cost
    }

    pub const fn manifestation_item(&self) -> Option<&ItemId> {
        self.manifestation_item.as_ref()
    }

    pub const fn manifestation_profile(&self) -> Option<&ContentId> {
        self.manifestation_profile.as_ref()
    }

    pub fn material_costs(&self) -> impl Iterator<Item = &TechniqueMaterialCost> {
        self.material_cost
            .iter()
            .chain(self.additional_material_costs.iter())
    }

    pub const fn required_tool(&self) -> Option<&ItemId> {
        self.required_tool.as_ref()
    }

    pub const fn produced_item(&self) -> Option<&ItemId> {
        self.produced_item.as_ref()
    }

    pub const fn improvement(&self) -> Option<TechniqueImprovement> {
        self.improvement
    }

    pub const fn on_hit_effect(&self) -> Option<&TechniqueOnHitEffect> {
        self.on_hit_effect.as_ref()
    }

    pub const fn engagement_requirement(&self) -> Option<&TechniqueEngagementRequirement> {
        self.engagement_requirement.as_ref()
    }

    pub const fn has_runtime_behavior(&self) -> bool {
        self.action.is_some() || self.improvement.is_some()
    }

    pub const fn preparation_steps(&self) -> Option<TimeUnits> {
        self.preparation_steps
    }

    pub const fn cooldown(&self) -> Option<TimeUnits> {
        self.cooldown
    }

    pub const fn action_kind(&self) -> ActionKind {
        self.action_kind
    }

    pub(crate) const fn remove_action_kind(&mut self) {
        self.action_kind = ActionKind::Support;
    }
}

fn validate_text_keys(name: &str, description: &str) -> Result<(), SkillDefinitionError> {
    if name.trim().is_empty() {
        return Err(SkillDefinitionError::EmptyNameKey);
    }
    if description.trim().is_empty() {
        return Err(SkillDefinitionError::EmptyDescriptionKey);
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkillDefinitionError {
    EmptyNameKey,
    EmptyDescriptionKey,
    ZeroMinimumLevel,
    ZeroMinimumAttribute,
    DuplicateMinimumAttribute(PrimaryAttribute),
    SelfPrerequisite,
    ActionOnNonActionTechnique,
    ZeroActionRange,
    ZeroMaximumTargets,
    InvalidAnalysisImprovement,
    InvalidActiveImprovement,
    InvalidReactionPercentage(u8),
    ZeroPhysicalDamagePercentage,
    ZeroProjectiles,
    ZeroBarrageStages,
    ZeroTargetSeparation,
    MoreTargetsThanProjectiles { projectiles: u8, targets: u8 },
    ZeroActionRecovery,
    ZeroForcedMovementDistance,
    ForcedMovementRequiresMeleeDelivery,
    MeleeArcRequiresMeleeDelivery,
    IncompatibleWeaponAttackModifiers,
    ZeroMeleeArcCells,
    TooManyMeleeArcCells(u8),
    PreparationWithoutAction,
    ZeroPreparationSteps,
    ActionKindWithoutAction,
    WeaponAttackMustBeOffensive,
    ImprovementOnIncompatibleTechnique,
    ActionAndImprovement,
    OnHitEffectWithoutWeaponAttack,
    ZeroOnHitStatusStacks,
    ZeroEffectIntensity,
    EmptyEngagementStatusFamilies,
    EngagementRequirementWithoutWeaponAttack,
    CooldownWithoutAction,
    ZeroCooldown,
    ZeroMaterialQuantity,
    MaterialCostWithoutDeployment,
    ZeroExplosiveStageDelay,
    InvalidExplosiveDelayRange,
    InvalidExplosivePayload,
    InvalidMovementDistanceRange,
    ZeroMovementTime,
    ZeroSoundIntensity,
    ZeroEffectDuration,
    ZeroDeviceIntegrity,
    ZeroDurabilityRestored,
    ZeroOutputPercentage,
    ZeroEnergyAmount,
    InvalidHeatThresholds,
}

impl Display for SkillDefinitionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyNameKey => write!(formatter, "skill name_key must not be empty"),
            Self::EmptyDescriptionKey => {
                write!(formatter, "skill description_key must not be empty")
            }
            Self::ZeroMinimumLevel => write!(formatter, "technique minimum level must be positive"),
            Self::ZeroMinimumAttribute => {
                write!(formatter, "technique minimum attribute must be positive")
            }
            Self::DuplicateMinimumAttribute(attribute) => write!(
                formatter,
                "technique declares minimum attribute '{attribute}' more than once"
            ),
            Self::SelfPrerequisite => write!(formatter, "a technique cannot require itself"),
            Self::ActionOnNonActionTechnique => {
                write!(
                    formatter,
                    "only an action technique can define an active action"
                )
            }
            Self::ZeroActionRange => write!(formatter, "technique action range must be positive"),
            Self::ZeroMaximumTargets => {
                write!(formatter, "technique action target limit must be positive")
            }
            Self::InvalidAnalysisImprovement => {
                write!(
                    formatter,
                    "multiple analysis must be an improvement of a target analysis"
                )
            }
            Self::InvalidActiveImprovement => write!(
                formatter,
                "an executable improvement requires an improvement technique with a prerequisite"
            ),
            Self::InvalidReactionPercentage(percentage) => write!(
                formatter,
                "reaction damage reduction must be between 1 and 100 percent, found {percentage}"
            ),
            Self::ZeroPhysicalDamagePercentage => {
                write!(formatter, "physical damage percentage must be positive")
            }
            Self::ZeroProjectiles => {
                write!(
                    formatter,
                    "a weapon volley must fire at least one projectile"
                )
            }
            Self::ZeroBarrageStages => {
                write!(formatter, "a weapon barrage must have at least one stage")
            }
            Self::ZeroTargetSeparation => {
                write!(formatter, "volley target separation must be positive")
            }
            Self::MoreTargetsThanProjectiles {
                projectiles,
                targets,
            } => write!(
                formatter,
                "a {projectiles}-projectile volley cannot require {targets} distinct targets"
            ),
            Self::ZeroActionRecovery => {
                write!(
                    formatter,
                    "technique recovery must take at least one time unit"
                )
            }
            Self::ZeroForcedMovementDistance => {
                write!(formatter, "forced movement distance must be positive")
            }
            Self::ForcedMovementRequiresMeleeDelivery => {
                write!(
                    formatter,
                    "Impact-based forced movement requires melee delivery"
                )
            }
            Self::MeleeArcRequiresMeleeDelivery => {
                write!(formatter, "a melee arc requires melee delivery")
            }
            Self::IncompatibleWeaponAttackModifiers => write!(
                formatter,
                "forced movement and a melee arc cannot modify the same weapon attack"
            ),
            Self::ZeroMeleeArcCells => {
                write!(formatter, "melee arc must cover at least one cell")
            }
            Self::TooManyMeleeArcCells(cells) => write!(
                formatter,
                "melee arc cannot cover more than eight cells, found {cells}"
            ),
            Self::PreparationWithoutAction => {
                write!(
                    formatter,
                    "only an executable technique can require preparation"
                )
            }
            Self::ZeroPreparationSteps => {
                write!(
                    formatter,
                    "technique preparation must take at least one time unit"
                )
            }
            Self::ActionKindWithoutAction => {
                write!(
                    formatter,
                    "only an executable technique can declare action intent"
                )
            }
            Self::WeaponAttackMustBeOffensive => {
                write!(formatter, "a weapon attack technique must be offensive")
            }
            Self::ImprovementOnIncompatibleTechnique => write!(
                formatter,
                "a passive improvement requires an improvement technique with a prerequisite"
            ),
            Self::ActionAndImprovement => write!(
                formatter,
                "a technique cannot define both an active action and a passive improvement"
            ),
            Self::OnHitEffectWithoutWeaponAttack => {
                write!(
                    formatter,
                    "an on-hit effect requires a weapon attack technique"
                )
            }
            Self::ZeroOnHitStatusStacks => {
                write!(
                    formatter,
                    "an on-hit status application requires positive stacks"
                )
            }
            Self::ZeroEffectIntensity => {
                write!(formatter, "effect resistance intensity must be positive")
            }
            Self::EmptyEngagementStatusFamilies => {
                write!(formatter, "engagement status families must not be empty")
            }
            Self::EngagementRequirementWithoutWeaponAttack => write!(
                formatter,
                "an engagement requirement currently requires a weapon attack technique"
            ),
            Self::CooldownWithoutAction => {
                write!(
                    formatter,
                    "only an executable technique can define a cooldown"
                )
            }
            Self::ZeroCooldown => write!(formatter, "technique cooldown must be positive"),
            Self::ZeroMaterialQuantity => {
                write!(formatter, "technique material quantity must be positive")
            }
            Self::MaterialCostWithoutDeployment => write!(
                formatter,
                "only an explosive deployment can consume authored material"
            ),
            Self::ZeroExplosiveStageDelay => {
                write!(
                    formatter,
                    "a secondary explosive stage requires a positive delay"
                )
            }
            Self::InvalidExplosiveDelayRange => write!(
                formatter,
                "explosive programming delays must be positive and ordered"
            ),
            Self::InvalidExplosivePayload => write!(
                formatter,
                "explosive payload range and damage must be positive"
            ),
            Self::InvalidMovementDistanceRange => write!(
                formatter,
                "movement distance bounds must be positive and ordered"
            ),
            Self::ZeroMovementTime => {
                write!(formatter, "stealth movement time must be positive")
            }
            Self::ZeroSoundIntensity => {
                write!(formatter, "sound intensity must be positive")
            }
            Self::ZeroEffectDuration => {
                write!(formatter, "effect duration must be positive")
            }
            Self::ZeroDeviceIntegrity => {
                write!(formatter, "deployed device integrity must be positive")
            }
            Self::ZeroDurabilityRestored => {
                write!(
                    formatter,
                    "component durability restoration must be positive"
                )
            }
            Self::ZeroOutputPercentage => {
                write!(
                    formatter,
                    "module output and energy percentages must be positive"
                )
            }
            Self::ZeroEnergyAmount => {
                write!(
                    formatter,
                    "stored and per-phase energy amounts must be positive"
                )
            }
            Self::InvalidHeatThresholds => {
                write!(formatter, "overclock heat thresholds must be ordered")
            }
        }
    }
}

const fn explosive_payload_profile_is_valid(payload: ExplosivePayloadProfile) -> bool {
    let area_is_valid = match payload.area() {
        crate::explosive::ExplosiveAreaProfile::Radial { radius, .. } => radius > 0,
        crate::explosive::ExplosiveAreaProfile::Directional { range, .. } => range > 0,
    };
    area_is_valid
        && payload.damage().amount > 0
        && match payload.center_damage() {
            Some(damage) => damage.amount > 0,
            None => true,
        }
}

impl Error for SkillDefinitionError {}

#[derive(Clone, PartialEq, Eq)]
pub struct SkillProgressionRules {
    choice_costs: Vec<u16>,
    enforce_authored_requirements: bool,
}

impl Debug for SkillProgressionRules {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut rules = formatter.debug_struct("SkillProgressionRules");
        // Keep the historical field name for suspension fingerprints. It is a
        // curve of successive choice costs; it no longer grants access.
        rules.field("rank_costs", &self.choice_costs);
        if self.enforce_authored_requirements {
            rules.field(
                "enforce_authored_requirements",
                &self.enforce_authored_requirements,
            );
        }
        rules.finish()
    }
}

impl SkillProgressionRules {
    pub fn new(choice_costs: Vec<u16>) -> Result<Self, SkillProgressionRulesError> {
        let rules = Self {
            choice_costs,
            enforce_authored_requirements: true,
        };
        rules.validate()?;
        Ok(rules)
    }

    pub fn validate(&self) -> Result<(), SkillProgressionRulesError> {
        if self.choice_costs.is_empty() {
            return Err(SkillProgressionRulesError::NoChoiceCosts);
        }
        if self.choice_costs.len() > usize::from(u8::MAX) {
            return Err(SkillProgressionRulesError::TooManyCostTiers);
        }
        if self.choice_costs.contains(&0) {
            return Err(SkillProgressionRulesError::ZeroChoiceCost);
        }
        Ok(())
    }

    pub fn configured_choice_count(&self) -> usize {
        self.choice_costs.len()
    }

    /// Cost of the one-based technique choice. Choices beyond the final
    /// configured step retain its cost instead of becoming forbidden.
    pub fn cost_for_choice_number(&self, choice_number: usize) -> Option<u16> {
        let last = self.choice_costs.len().checked_sub(1)?;
        let index = choice_number.checked_sub(1)?.min(last);
        self.choice_costs.get(index).copied()
    }

    pub fn choice_costs(&self) -> &[u16] {
        &self.choice_costs
    }

    pub const fn enforces_authored_requirements(&self) -> bool {
        self.enforce_authored_requirements
    }

    /// Compatibility policy for journals recorded before level and attribute
    /// requirements replaced discipline-rank gates. Replay still validates the
    /// exact historical rules and final state before accepting a suspension.
    pub fn without_authored_requirements(mut self) -> Self {
        self.enforce_authored_requirements = false;
        self
    }
}

impl Default for SkillProgressionRules {
    fn default() -> Self {
        Self {
            choice_costs: vec![1, 1, 2, 2, 3],
            enforce_authored_requirements: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkillProgressionRulesError {
    NoChoiceCosts,
    TooManyCostTiers,
    ZeroChoiceCost,
}

impl Display for SkillProgressionRulesError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoChoiceCosts => {
                write!(
                    formatter,
                    "skill progression must define at least one choice cost"
                )
            }
            Self::TooManyCostTiers => {
                write!(formatter, "skill progression defines too many cost tiers")
            }
            Self::ZeroChoiceCost => write!(formatter, "skill choice costs must be positive"),
        }
    }
}

impl Error for SkillProgressionRulesError {}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SkillCatalog {
    disciplines: BTreeMap<DisciplineId, DisciplineDefinition>,
    techniques: BTreeMap<TechniqueId, TechniqueDefinition>,
}

impl SkillCatalog {
    pub fn without_action_kinds(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.techniques.values_mut() {
            definition.remove_action_kind();
        }
        catalog
    }

    /// Reconstructs selected inert declarations for an older replay
    /// generation. Current content keeps its executable behavior; only the
    /// compatibility adapter calls this transform.
    pub fn without_runtime_behaviors(
        &self,
        techniques: impl IntoIterator<Item = (TechniqueId, TechniqueKind)>,
    ) -> Self {
        let mut catalog = self.clone();
        for (id, legacy_kind) in techniques {
            if let Some(definition) = catalog.techniques.get_mut(&id) {
                definition.kind = legacy_kind;
                definition.action = None;
                definition.preparation_steps = None;
                definition.cooldown = None;
                definition.remove_action_kind();
            }
        }
        catalog
    }

    /// Removes intrinsic manifestation metadata when replaying a generation
    /// that still consumed inventory materials and required physical tools.
    pub fn without_intrinsic_manifestations(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.techniques.values_mut() {
            definition.activation_cost = None;
            definition.manifestation_item = None;
            definition.manifestation_profile = None;
        }
        catalog
    }

    /// Restores an action shape used by a historical replay generation.
    pub fn with_compatibility_action(&self, id: &TechniqueId, action: TechniqueAction) -> Self {
        let mut catalog = self.clone();
        if let Some(definition) = catalog.techniques.get_mut(id) {
            definition.action = Some(action);
            definition.cooldown = None;
        }
        catalog
    }

    pub fn without_discipline(&self, excluded: &DisciplineId) -> Self {
        let mut catalog = self.clone();
        catalog.disciplines.remove(excluded);
        catalog
            .techniques
            .retain(|_, definition| definition.discipline() != excluded);
        catalog
    }

    pub fn register_discipline(
        &mut self,
        definition: DisciplineDefinition,
    ) -> Result<(), SkillCatalogError> {
        let id = definition.id().clone();
        if self.disciplines.contains_key(&id) {
            return Err(SkillCatalogError::DuplicateDiscipline(id));
        }
        self.disciplines.insert(id, definition);
        Ok(())
    }

    pub fn register_technique(
        &mut self,
        definition: TechniqueDefinition,
    ) -> Result<(), SkillCatalogError> {
        let id = definition.id().clone();
        if self.techniques.contains_key(&id) {
            return Err(SkillCatalogError::DuplicateTechnique(id));
        }
        self.techniques.insert(id, definition);
        Ok(())
    }

    pub fn discipline(&self, id: &DisciplineId) -> Option<&DisciplineDefinition> {
        self.disciplines.get(id)
    }

    pub fn technique(&self, id: &TechniqueId) -> Option<&TechniqueDefinition> {
        self.techniques.get(id)
    }

    /// Targeted improvements inherit their prerequisite's sensor range.
    pub fn target_range(&self, id: &TechniqueId) -> Option<u16> {
        let definition = self.technique(id)?;
        match definition.action()? {
            TechniqueAction::AnalyzeTarget { range }
            | TechniqueAction::AnalyzeThreat { range }
            | TechniqueAction::DiagnoseEnergy { range, .. } => Some(range),
            TechniqueAction::Breakthrough { .. } | TechniqueAction::ExtractAlly { .. } => Some(1),
            TechniqueAction::AnalyzeMultipleTargets { .. } => {
                match self.technique(definition.prerequisite()?)?.action()? {
                    TechniqueAction::AnalyzeTarget { range } => Some(range),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn disciplines(&self) -> impl Iterator<Item = (&DisciplineId, &DisciplineDefinition)> {
        self.disciplines.iter()
    }

    pub fn techniques(&self) -> impl Iterator<Item = (&TechniqueId, &TechniqueDefinition)> {
        self.techniques.iter()
    }

    pub fn validate(&self, rules: &SkillProgressionRules) -> Result<(), SkillCatalogError> {
        rules.validate().map_err(SkillCatalogError::Rules)?;
        self.validate_structure()?;

        let all_features = SystemFeatureSet::new(
            self.techniques
                .values()
                .flat_map(TechniqueDefinition::required_features)
                .cloned(),
        );
        for discipline in self.disciplines.keys() {
            let availability = self
                .analyze_discipline(discipline, &all_features, &[], rules)
                .map_err(SkillCatalogError::InvalidInitialChoices)?;
            if !availability.is_open() {
                return Err(SkillCatalogError::IncompleteDiscipline(discipline.clone()));
            }
        }
        Ok(())
    }

    pub fn validate_structure(&self) -> Result<(), SkillCatalogError> {
        for technique in self.techniques.values() {
            if !self.disciplines.contains_key(technique.discipline()) {
                return Err(SkillCatalogError::UnknownDiscipline {
                    technique: Box::new(technique.id().clone()),
                    discipline: Box::new(technique.discipline().clone()),
                });
            }
            if matches!(
                technique.action(),
                Some(
                    TechniqueAction::DeployExplosive { .. }
                        | TechniqueAction::DeploySoundDecoy { .. }
                        | TechniqueAction::CamouflageExplosive { .. }
                        | TechniqueAction::DeploySaturationBeacon { .. }
                )
            ) && technique.material_cost().is_none()
                && technique.activation_cost().is_none()
            {
                return Err(SkillCatalogError::MissingDeploymentMaterial(
                    technique.id().clone(),
                ));
            }
            if matches!(
                technique.action(),
                Some(TechniqueAction::DeployExplosive { .. })
            ) && technique.activation_cost().is_some()
                && technique.manifestation_item().is_none()
            {
                return Err(SkillCatalogError::MissingDeploymentMaterial(
                    technique.id().clone(),
                ));
            }
            if matches!(technique.action(), Some(TechniqueAction::SalvageComponent))
                && technique.produced_item().is_none()
            {
                return Err(SkillCatalogError::MissingDeploymentMaterial(
                    technique.id().clone(),
                ));
            }
            if let Some(prerequisite_id) = technique.prerequisite() {
                let prerequisite = self.techniques.get(prerequisite_id).ok_or_else(|| {
                    SkillCatalogError::UnknownPrerequisite {
                        technique: Box::new(technique.id().clone()),
                        prerequisite: Box::new(prerequisite_id.clone()),
                    }
                })?;
                if prerequisite.discipline() != technique.discipline() {
                    return Err(SkillCatalogError::CrossDisciplinePrerequisite {
                        technique: Box::new(technique.id().clone()),
                        prerequisite: Box::new(prerequisite_id.clone()),
                    });
                }
                if matches!(
                    technique.action(),
                    Some(TechniqueAction::AnalyzeMultipleTargets { .. })
                ) && !matches!(
                    prerequisite.action(),
                    Some(TechniqueAction::AnalyzeTarget { .. })
                ) {
                    return Err(SkillCatalogError::InvalidAnalysisImprovement(
                        technique.id().clone(),
                    ));
                }
                if matches!(
                    technique.action(),
                    Some(TechniqueAction::RecoverNeutralizedExplosive { .. })
                ) && !matches!(
                    prerequisite.action(),
                    Some(TechniqueAction::NeutralizeExplosive { .. })
                ) {
                    return Err(SkillCatalogError::InvalidExplosiveRecoveryImprovement(
                        technique.id().clone(),
                    ));
                }
                if technique.kind() == TechniqueKind::Improvement
                    && technique
                        .action()
                        .is_some_and(TechniqueAction::is_electronic_warfare_action)
                    && !matches!(
                        (technique.action(), prerequisite.action()),
                        (Some(variant), Some(parent))
                            if variant.is_same_electronic_family(parent)
                    )
                {
                    return Err(SkillCatalogError::InvalidElectronicVariant(
                        technique.id().clone(),
                    ));
                }
                if technique.improvement() == Some(TechniqueImprovement::MeleeCounterattack)
                    && !matches!(
                        prerequisite.action(),
                        Some(TechniqueAction::PrepareMeleeParry { .. })
                    )
                {
                    return Err(SkillCatalogError::InvalidMeleeCounterattackImprovement(
                        technique.id().clone(),
                    ));
                }
                if technique.improvement() == Some(TechniqueImprovement::ExtendedRangedOverwatch)
                    && !matches!(
                        prerequisite.action(),
                        Some(TechniqueAction::PrepareRangedOverwatch { .. })
                    )
                {
                    return Err(SkillCatalogError::InvalidRangedOverwatchImprovement(
                        technique.id().clone(),
                    ));
                }
                if matches!(
                    technique.improvement(),
                    Some(TechniqueImprovement::PersistentRangedAim { .. })
                ) && !matches!(
                    prerequisite.action(),
                    Some(TechniqueAction::WeaponAttack {
                        required_delivery: AttackDelivery::Ranged,
                        ..
                    })
                ) {
                    return Err(SkillCatalogError::InvalidPersistentRangedAimImprovement(
                        technique.id().clone(),
                    ));
                }
                if technique.improvement() == Some(TechniqueImprovement::ControlledChargeInertia)
                    && !matches!(
                        prerequisite.action(),
                        Some(TechniqueAction::ChargeAttack { .. })
                    )
                {
                    return Err(SkillCatalogError::InvalidChargeImprovement(
                        technique.id().clone(),
                    ));
                }
                if matches!(
                    technique.improvement(),
                    Some(TechniqueImprovement::SilentNeutralization { .. })
                ) && !matches!(
                    prerequisite.action(),
                    Some(TechniqueAction::AmbushAttack { .. })
                ) {
                    return Err(SkillCatalogError::InvalidAmbushImprovement(
                        technique.id().clone(),
                    ));
                }
                if matches!(
                    technique.improvement(),
                    Some(TechniqueImprovement::DroneAutonomousScout { .. })
                ) && !matches!(
                    prerequisite.action(),
                    Some(TechniqueAction::DronePatrol { .. })
                ) {
                    return Err(SkillCatalogError::InvalidDroneScoutImprovement(
                        technique.id().clone(),
                    ));
                }
                if prerequisite.minimum_level() >= technique.minimum_level() {
                    return Err(SkillCatalogError::PrerequisiteLevelNotEarlier {
                        technique: Box::new(technique.id().clone()),
                        prerequisite: Box::new(prerequisite_id.clone()),
                    });
                }
            }
            if matches!(
                technique.improvement(),
                Some(TechniqueImprovement::CoveredApproach { .. })
            ) && (technique.kind() != TechniqueKind::Behavior
                || technique.prerequisite().is_some())
            {
                return Err(SkillCatalogError::InvalidStandaloneBehavior(
                    technique.id().clone(),
                ));
            }
        }
        Ok(())
    }

    pub fn validate_item_references(&self, items: &ItemCatalog) -> Result<(), SkillCatalogError> {
        for technique in self.techniques.values() {
            for cost in technique.material_costs() {
                if items.get(cost.item()).is_none() {
                    return Err(SkillCatalogError::UnknownTechniqueMaterial {
                        technique: Box::new(technique.id().clone()),
                        item: cost.item().clone(),
                    });
                }
            }
            if let Some(tool) = technique.required_tool()
                && items.get(tool).is_none()
            {
                return Err(SkillCatalogError::UnknownTechniqueMaterial {
                    technique: Box::new(technique.id().clone()),
                    item: tool.clone(),
                });
            }
            if let Some(item) = technique.produced_item()
                && items.get(item).is_none()
            {
                return Err(SkillCatalogError::UnknownTechniqueMaterial {
                    technique: Box::new(technique.id().clone()),
                    item: item.clone(),
                });
            }
            if let Some(item) = technique.manifestation_item()
                && items.get(item).is_none()
            {
                return Err(SkillCatalogError::UnknownTechniqueMaterial {
                    technique: Box::new(technique.id().clone()),
                    item: item.clone(),
                });
            }
        }
        Ok(())
    }

    pub fn validate_status_references(
        &self,
        statuses: &StatusCatalog,
    ) -> Result<(), SkillCatalogError> {
        for technique in self.techniques.values() {
            if let Some(effect) = technique.on_hit_effect() {
                let status = effect.application().status();
                if !statuses.contains(status) {
                    return Err(SkillCatalogError::UnknownTechniqueStatus {
                        technique: Box::new(technique.id().clone()),
                        status: status.clone(),
                    });
                }
            }
            if let Some(requirement) = technique.engagement_requirement() {
                let families = requirement.status_families();
                if families.is_empty() {
                    continue;
                }
                if !families
                    .iter()
                    .any(|family| statuses.contains_family(family))
                {
                    return Err(SkillCatalogError::UnknownTechniqueStatusFamily {
                        technique: Box::new(technique.id().clone()),
                        family: (*families[0]).clone(),
                    });
                }
            }
        }
        Ok(())
    }

    /// Structural validation can load future content. Running an open
    /// discipline additionally requires executable behavior for every purchase.
    pub fn validate_runtime(
        &self,
        features: &SystemFeatureSet,
        rules: &SkillProgressionRules,
    ) -> Result<(), SkillCatalogError> {
        self.validate(rules)?;
        for discipline in self.disciplines.keys() {
            let availability = self
                .analyze_discipline(discipline, features, &[], rules)
                .map_err(SkillCatalogError::InvalidInitialChoices)?;
            if availability.is_open() {
                for id in availability.available {
                    if !self
                        .technique(&id)
                        .is_some_and(TechniqueDefinition::has_runtime_behavior)
                    {
                        return Err(SkillCatalogError::MissingTechniqueBehavior(id));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn analyze_discipline(
        &self,
        discipline: &DisciplineId,
        features: &SystemFeatureSet,
        initial_choices: &[TechniqueId],
        rules: &SkillProgressionRules,
    ) -> Result<DisciplineAvailability, InitialChoicesError> {
        let required_path_length = rules.configured_choice_count();
        let pool: Vec<&TechniqueDefinition> = self
            .techniques
            .values()
            .filter(|technique| technique.discipline() == discipline)
            .collect();
        let mut available: BTreeSet<TechniqueId> = pool
            .iter()
            .filter(|technique| {
                technique
                    .required_features()
                    .all(|feature| features.contains(feature))
            })
            .map(|technique| technique.id().clone())
            .collect();

        loop {
            let invalid: Vec<TechniqueId> = available
                .iter()
                .filter(|id| {
                    self.techniques
                        .get(*id)
                        .and_then(TechniqueDefinition::prerequisite)
                        .is_some_and(|prerequisite| !available.contains(prerequisite))
                })
                .cloned()
                .collect();
            if invalid.is_empty() {
                break;
            }
            for id in invalid {
                available.remove(&id);
            }
        }

        let mut initial = BTreeSet::new();
        for id in initial_choices {
            let technique = self
                .techniques
                .get(id)
                .ok_or_else(|| InitialChoicesError::UnknownTechnique(id.clone()))?;
            if technique.discipline() != discipline {
                return Err(InitialChoicesError::WrongDiscipline(id.clone()));
            }
            if !available.contains(id) {
                return Err(InitialChoicesError::UnavailableTechnique(id.clone()));
            }
            if !initial.insert(id.clone()) {
                return Err(InitialChoicesError::DuplicateTechnique(id.clone()));
            }
            if technique
                .prerequisite()
                .is_some_and(|prerequisite| !initial.contains(prerequisite))
            {
                return Err(InitialChoicesError::MissingPrerequisite(id.clone()));
            }
        }

        let mut states = vec![BTreeSet::new(); required_path_length + 1];
        let initial_choice_count = initial.len().min(required_path_length);
        states[initial_choice_count].insert(initial.clone());
        let mut dead_ends = Vec::new();
        for choice_count in initial_choice_count..required_path_length {
            let current_states: Vec<BTreeSet<TechniqueId>> =
                states[choice_count].iter().cloned().collect();
            for learned in current_states {
                let successors: Vec<TechniqueId> = available
                    .iter()
                    .filter(|id| !learned.contains(*id))
                    .filter(|id| {
                        self.techniques
                            .get(*id)
                            .and_then(TechniqueDefinition::prerequisite)
                            .is_none_or(|prerequisite| learned.contains(prerequisite))
                    })
                    .cloned()
                    .collect();
                if successors.is_empty() {
                    dead_ends.push(learned);
                } else {
                    for successor in successors {
                        let mut next = learned.clone();
                        next.insert(successor);
                        states[choice_count + 1].insert(next);
                    }
                }
            }
        }

        let unavailable = pool
            .iter()
            .map(|technique| technique.id().clone())
            .filter(|id| !available.contains(id))
            .collect();
        let complete_paths = states[required_path_length].len();
        Ok(DisciplineAvailability {
            discipline: discipline.clone(),
            available,
            unavailable,
            dead_ends,
            complete_paths,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisciplineAvailability {
    pub discipline: DisciplineId,
    pub available: BTreeSet<TechniqueId>,
    pub unavailable: BTreeSet<TechniqueId>,
    pub dead_ends: Vec<BTreeSet<TechniqueId>>,
    pub complete_paths: usize,
}

impl DisciplineAvailability {
    pub fn is_open(&self) -> bool {
        self.dead_ends.is_empty() && self.complete_paths > 0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InitialChoicesError {
    UnknownTechnique(TechniqueId),
    WrongDiscipline(TechniqueId),
    UnavailableTechnique(TechniqueId),
    DuplicateTechnique(TechniqueId),
    MissingPrerequisite(TechniqueId),
}

impl Display for InitialChoicesError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownTechnique(id) => write!(formatter, "unknown initial technique '{id}'"),
            Self::WrongDiscipline(id) => {
                write!(
                    formatter,
                    "initial technique '{id}' belongs to another discipline"
                )
            }
            Self::UnavailableTechnique(id) => {
                write!(
                    formatter,
                    "initial technique '{id}' is unavailable in this version"
                )
            }
            Self::DuplicateTechnique(id) => {
                write!(formatter, "initial technique '{id}' is duplicated")
            }
            Self::MissingPrerequisite(id) => {
                write!(
                    formatter,
                    "initial technique '{id}' is missing its prerequisite"
                )
            }
        }
    }
}

impl Error for InitialChoicesError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SkillCatalogError {
    MissingTechniqueBehavior(TechniqueId),
    MissingDeploymentMaterial(TechniqueId),
    InvalidAnalysisImprovement(TechniqueId),
    InvalidExplosiveRecoveryImprovement(TechniqueId),
    InvalidMeleeCounterattackImprovement(TechniqueId),
    InvalidRangedOverwatchImprovement(TechniqueId),
    InvalidPersistentRangedAimImprovement(TechniqueId),
    InvalidChargeImprovement(TechniqueId),
    InvalidAmbushImprovement(TechniqueId),
    InvalidDroneScoutImprovement(TechniqueId),
    InvalidElectronicVariant(TechniqueId),
    InvalidStandaloneBehavior(TechniqueId),
    UnknownTechniqueStatus {
        technique: Box<TechniqueId>,
        status: StatusId,
    },
    UnknownTechniqueStatusFamily {
        technique: Box<TechniqueId>,
        family: StatusFamilyId,
    },
    UnknownTechniqueMaterial {
        technique: Box<TechniqueId>,
        item: ItemId,
    },
    Rules(SkillProgressionRulesError),
    DuplicateDiscipline(DisciplineId),
    DuplicateTechnique(TechniqueId),
    UnknownDiscipline {
        technique: Box<TechniqueId>,
        discipline: Box<DisciplineId>,
    },
    UnknownPrerequisite {
        technique: Box<TechniqueId>,
        prerequisite: Box<TechniqueId>,
    },
    CrossDisciplinePrerequisite {
        technique: Box<TechniqueId>,
        prerequisite: Box<TechniqueId>,
    },
    PrerequisiteLevelNotEarlier {
        technique: Box<TechniqueId>,
        prerequisite: Box<TechniqueId>,
    },
    InvalidInitialChoices(InitialChoicesError),
    IncompleteDiscipline(DisciplineId),
}

impl Display for SkillCatalogError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTechniqueBehavior(id) => write!(
                formatter,
                "purchasable technique '{id}' has no executable behavior"
            ),
            Self::MissingDeploymentMaterial(id) => write!(
                formatter,
                "explosive deployment '{id}' has no authored material cost"
            ),
            Self::InvalidAnalysisImprovement(id) => write!(
                formatter,
                "multiple analysis '{id}' must require a target analysis"
            ),
            Self::InvalidExplosiveRecoveryImprovement(id) => write!(
                formatter,
                "explosive recovery '{id}' must improve an explosive neutralization"
            ),
            Self::InvalidMeleeCounterattackImprovement(id) => write!(
                formatter,
                "melee counterattack '{id}' must improve a melee parry preparation"
            ),
            Self::InvalidRangedOverwatchImprovement(id) => write!(
                formatter,
                "extended ranged overwatch '{id}' must improve a ranged overwatch preparation"
            ),
            Self::InvalidPersistentRangedAimImprovement(id) => write!(
                formatter,
                "persistent ranged aim '{id}' must improve a ranged weapon attack"
            ),
            Self::InvalidChargeImprovement(id) => write!(
                formatter,
                "controlled charge inertia '{id}' must improve a charge attack"
            ),
            Self::InvalidAmbushImprovement(id) => write!(
                formatter,
                "silent neutralization '{id}' must improve an ambush attack"
            ),
            Self::InvalidDroneScoutImprovement(id) => write!(
                formatter,
                "autonomous scout '{id}' must improve a drone patrol"
            ),
            Self::InvalidElectronicVariant(id) => write!(
                formatter,
                "electronic warfare variant '{id}' must transform the same action family as its prerequisite"
            ),
            Self::InvalidStandaloneBehavior(id) => write!(
                formatter,
                "standalone behavior '{id}' must not declare a prerequisite"
            ),
            Self::UnknownTechniqueStatus { technique, status } => write!(
                formatter,
                "technique '{technique}' references unknown status '{status}'"
            ),
            Self::UnknownTechniqueStatusFamily { technique, family } => write!(
                formatter,
                "technique '{technique}' references unknown status family '{family}'"
            ),
            Self::UnknownTechniqueMaterial { technique, item } => write!(
                formatter,
                "technique '{technique}' references unknown material '{item}'"
            ),
            Self::Rules(error) => write!(formatter, "invalid skill progression rules: {error}"),
            Self::DuplicateDiscipline(id) => write!(formatter, "duplicate discipline ID '{id}'"),
            Self::DuplicateTechnique(id) => write!(formatter, "duplicate technique ID '{id}'"),
            Self::UnknownDiscipline {
                technique,
                discipline,
            } => write!(
                formatter,
                "technique '{technique}' references unknown discipline '{discipline}'"
            ),
            Self::UnknownPrerequisite {
                technique,
                prerequisite,
            } => write!(
                formatter,
                "technique '{technique}' references unknown prerequisite '{prerequisite}'"
            ),
            Self::CrossDisciplinePrerequisite {
                technique,
                prerequisite,
            } => write!(
                formatter,
                "technique '{technique}' cannot require cross-discipline technique '{prerequisite}'"
            ),
            Self::PrerequisiteLevelNotEarlier {
                technique,
                prerequisite,
            } => write!(
                formatter,
                "prerequisite '{prerequisite}' must have an earlier minimum level than '{technique}'"
            ),
            Self::InvalidInitialChoices(error) => {
                write!(formatter, "invalid initial skill choices: {error}")
            }
            Self::IncompleteDiscipline(id) => write!(
                formatter,
                "discipline '{id}' cannot complete every legal progression path"
            ),
        }
    }
}

impl Error for SkillCatalogError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn technique_action_kind_defaults_to_support_and_can_be_declared_offensive() {
        let definition = TechniqueDefinition::new(
            "core:test_action_kind".parse().unwrap(),
            "core:test_discipline".parse().unwrap(),
            "technique.test.name".to_owned(),
            "technique.test.description".to_owned(),
            1,
            TechniqueKind::Action,
            None,
            [],
        )
        .unwrap()
        .with_action(TechniqueAction::ReadMovementTraces { radius: 1 })
        .unwrap();

        assert_eq!(definition.action_kind(), ActionKind::Support);
        assert!(!format!("{definition:?}").contains("action_kind:"));
        let offensive = definition
            .clone()
            .with_action_kind(ActionKind::Offensive)
            .unwrap();
        assert_eq!(offensive.action_kind(), ActionKind::Offensive);
        assert!(format!("{offensive:?}").contains("action_kind:"));

        let passive = TechniqueDefinition::new(
            "core:test_passive_kind".parse().unwrap(),
            "core:test_discipline".parse().unwrap(),
            "technique.test.name".to_owned(),
            "technique.test.description".to_owned(),
            1,
            TechniqueKind::Posture,
            None,
            [],
        )
        .unwrap();
        assert_eq!(
            passive.with_action_kind(ActionKind::Offensive),
            Err(SkillDefinitionError::ActionKindWithoutAction)
        );
    }

    fn id(value: &str) -> ContentId {
        value
            .parse()
            .unwrap_or_else(|error| panic!("valid content ID rejected: {error}"))
    }

    fn reconnaissance_catalog() -> SkillCatalog {
        let discipline = id("core:reconnaissance");
        let mut catalog = SkillCatalog::default();
        catalog
            .register_discipline(
                DisciplineDefinition::new(
                    discipline.clone(),
                    "discipline.reconnaissance.name".to_owned(),
                    "discipline.reconnaissance.description".to_owned(),
                )
                .unwrap_or_else(|error| panic!("valid discipline rejected: {error}")),
            )
            .unwrap_or_else(|error| panic!("valid discipline registration rejected: {error}"));
        let definitions = [
            ("rec_01", 1, None, None),
            ("rec_02", 1, None, Some("traces")),
            ("rec_03", 2, None, Some("secrets")),
            ("rec_04", 2, None, None),
            ("rec_05", 3, None, None),
            ("rec_09", 3, Some("rec_01"), None),
            ("rec_08", 4, None, Some("energy_states")),
        ];
        for (name, minimum_level, prerequisite, feature) in definitions {
            catalog
                .register_technique(
                    TechniqueDefinition::new(
                        id(&format!("core:{name}")),
                        discipline.clone(),
                        format!("technique.{name}.name"),
                        format!("technique.{name}.description"),
                        minimum_level,
                        if prerequisite.is_some() {
                            TechniqueKind::Improvement
                        } else {
                            TechniqueKind::Action
                        },
                        prerequisite.map(|parent| id(&format!("core:{parent}"))),
                        feature.map(|required| id(&format!("core:{required}"))),
                    )
                    .unwrap_or_else(|error| panic!("valid technique rejected: {error}")),
                )
                .unwrap_or_else(|error| panic!("valid registration rejected: {error}"));
        }
        catalog
    }

    #[test]
    fn complete_documented_reconnaissance_catalog_is_valid() {
        let catalog = reconnaissance_catalog();
        assert_eq!(catalog.validate(&SkillProgressionRules::default()), Ok(()));
    }

    #[test]
    fn multiple_analysis_requires_a_compatible_parent_and_inherits_its_range() {
        let mut catalog = reconnaissance_catalog();
        let parent = id("core:rec_01");
        let child = id("core:rec_09");
        let improvement = TechniqueAction::AnalyzeMultipleTargets {
            maximum_targets: 3,
            energy_cost: 2,
        };
        assert_eq!(
            catalog
                .technique(&parent)
                .unwrap()
                .clone()
                .with_action(improvement),
            Err(SkillDefinitionError::InvalidAnalysisImprovement)
        );
        assert_eq!(
            catalog.technique(&child).unwrap().clone().with_action(
                TechniqueAction::AnalyzeMultipleTargets {
                    maximum_targets: 0,
                    energy_cost: 2
                }
            ),
            Err(SkillDefinitionError::ZeroMaximumTargets)
        );
        catalog.techniques.get_mut(&child).unwrap().action = Some(improvement);
        assert_eq!(catalog.target_range(&child), None);
        assert_eq!(
            catalog.validate_structure(),
            Err(SkillCatalogError::InvalidAnalysisImprovement(child.clone()))
        );
        catalog.techniques.get_mut(&parent).unwrap().action =
            Some(TechniqueAction::AnalyzeTarget { range: 2 });
        assert_eq!(catalog.validate_structure(), Ok(()));
        assert_eq!(catalog.target_range(&child), Some(2));
    }

    #[test]
    fn prepared_reaction_percentage_is_validated_in_content_shape() {
        let action = TechniqueDefinition::new(
            id("core:test_parry"),
            id("core:test"),
            "technique.test_parry.name".to_owned(),
            "technique.test_parry.description".to_owned(),
            1,
            TechniqueKind::Action,
            None,
            [],
        )
        .unwrap();

        for percentage in [0, 101] {
            assert_eq!(
                action
                    .clone()
                    .with_action(TechniqueAction::PrepareMeleeParry {
                        physical_reduction_percentage: percentage,
                        trigger_energy_cost: 0,
                    }),
                Err(SkillDefinitionError::InvalidReactionPercentage(percentage))
            );
        }
    }

    #[test]
    fn melee_counterattack_is_a_passive_improvement_of_a_parry_only() {
        let discipline = id("core:test_melee");
        let parry = id("core:test_parry");
        let riposte = id("core:test_riposte");
        let mut catalog = SkillCatalog::default();
        catalog
            .register_discipline(
                DisciplineDefinition::new(
                    discipline.clone(),
                    "discipline.test_melee.name".to_owned(),
                    "discipline.test_melee.description".to_owned(),
                )
                .unwrap(),
            )
            .unwrap();
        catalog
            .register_technique(
                TechniqueDefinition::new(
                    parry.clone(),
                    discipline.clone(),
                    "technique.test_parry.name".to_owned(),
                    "technique.test_parry.description".to_owned(),
                    1,
                    TechniqueKind::Action,
                    None,
                    [],
                )
                .unwrap()
                .with_action(TechniqueAction::PrepareMeleeParry {
                    physical_reduction_percentage: 50,
                    trigger_energy_cost: 0,
                })
                .unwrap(),
            )
            .unwrap();
        catalog
            .register_technique(
                TechniqueDefinition::new(
                    riposte.clone(),
                    discipline,
                    "technique.test_riposte.name".to_owned(),
                    "technique.test_riposte.description".to_owned(),
                    2,
                    TechniqueKind::Improvement,
                    Some(parry.clone()),
                    [],
                )
                .unwrap()
                .with_improvement(TechniqueImprovement::MeleeCounterattack)
                .unwrap(),
            )
            .unwrap();

        assert_eq!(
            catalog.validate_runtime(
                &SystemFeatureSet::default(),
                &SkillProgressionRules::new(vec![1, 1]).unwrap()
            ),
            Ok(())
        );
        catalog.techniques.get_mut(&parry).unwrap().action =
            Some(TechniqueAction::AnalyzeTarget { range: 2 });
        assert_eq!(
            catalog.validate_structure(),
            Err(SkillCatalogError::InvalidMeleeCounterattackImprovement(
                riposte
            ))
        );
    }

    #[test]
    fn weapon_attack_technique_validates_damage_recovery_and_forced_movement() {
        let action = TechniqueDefinition::new(
            id("core:test_weapon_attack"),
            id("core:test"),
            "technique.test_weapon_attack.name".to_owned(),
            "technique.test_weapon_attack.description".to_owned(),
            1,
            TechniqueKind::Action,
            None,
            [],
        )
        .unwrap();

        assert_eq!(
            action.clone().with_action(TechniqueAction::WeaponAttack {
                required_delivery: AttackDelivery::Melee,
                physical_damage_percentage: Some(0),
                armor_penetration_bonus: 0,
                accuracy_modifier: 0,
                energy_cost: 0,
                recovery_time_units: Some(1),
                forced_movement: None,
                melee_arc: None,
            }),
            Err(SkillDefinitionError::ZeroPhysicalDamagePercentage)
        );
        assert_eq!(
            action.clone().with_action(TechniqueAction::WeaponAttack {
                required_delivery: AttackDelivery::Melee,
                physical_damage_percentage: Some(150),
                armor_penetration_bonus: 0,
                accuracy_modifier: 0,
                energy_cost: 0,
                recovery_time_units: Some(0),
                forced_movement: None,
                melee_arc: None,
            }),
            Err(SkillDefinitionError::ZeroActionRecovery)
        );
        assert_eq!(
            action.clone().with_action(TechniqueAction::WeaponAttack {
                required_delivery: AttackDelivery::Melee,
                physical_damage_percentage: Some(50),
                armor_penetration_bonus: 0,
                accuracy_modifier: 0,
                energy_cost: 0,
                recovery_time_units: None,
                forced_movement: Some(ForcedMovement::new(0, 0)),
                melee_arc: None,
            }),
            Err(SkillDefinitionError::ZeroForcedMovementDistance)
        );
        assert_eq!(
            action.clone().with_action(TechniqueAction::WeaponAttack {
                required_delivery: AttackDelivery::Ranged,
                physical_damage_percentage: None,
                armor_penetration_bonus: 0,
                accuracy_modifier: 0,
                energy_cost: 0,
                recovery_time_units: None,
                forced_movement: Some(ForcedMovement::new(1, 0)),
                melee_arc: None,
            }),
            Err(SkillDefinitionError::ForcedMovementRequiresMeleeDelivery)
        );
        assert_eq!(
            action.clone().with_action(TechniqueAction::WeaponAttack {
                required_delivery: AttackDelivery::Ranged,
                physical_damage_percentage: Some(70),
                armor_penetration_bonus: 0,
                accuracy_modifier: 0,
                energy_cost: 4,
                recovery_time_units: Some(1),
                forced_movement: None,
                melee_arc: Some(MeleeArc::new(3).unwrap()),
            }),
            Err(SkillDefinitionError::MeleeArcRequiresMeleeDelivery)
        );
        assert_eq!(
            action.with_action(TechniqueAction::WeaponAttack {
                required_delivery: AttackDelivery::Melee,
                physical_damage_percentage: Some(70),
                armor_penetration_bonus: 0,
                accuracy_modifier: 0,
                energy_cost: 4,
                recovery_time_units: Some(1),
                forced_movement: Some(ForcedMovement::new(1, 0)),
                melee_arc: Some(MeleeArc::new(3).unwrap()),
            }),
            Err(SkillDefinitionError::IncompatibleWeaponAttackModifiers)
        );

        let weapon_attack = TechniqueDefinition::new(
            id("core:test_weapon_attack_kind"),
            id("core:test"),
            "technique.test_weapon_attack_kind.name".to_owned(),
            "technique.test_weapon_attack_kind.description".to_owned(),
            1,
            TechniqueKind::Action,
            None,
            [],
        )
        .unwrap()
        .with_action(TechniqueAction::WeaponAttack {
            required_delivery: AttackDelivery::Melee,
            physical_damage_percentage: Some(150),
            armor_penetration_bonus: 0,
            accuracy_modifier: 0,
            energy_cost: 0,
            recovery_time_units: Some(1),
            forced_movement: None,
            melee_arc: None,
        })
        .unwrap();
        assert_eq!(weapon_attack.action_kind(), ActionKind::Offensive);
        assert_eq!(
            weapon_attack.with_action_kind(ActionKind::Support),
            Err(SkillDefinitionError::WeaponAttackMustBeOffensive)
        );
    }

    #[test]
    fn on_hit_status_effect_requires_a_weapon_attack_and_a_known_status() {
        let discipline = id("core:test_melee");
        let technique = id("core:test_armor_break");
        let status: StatusId = id("core:test_armor_fracture");
        let effect = TechniqueOnHitEffect::new(
            ApplyStatusEffect::new(status.clone(), 1).unwrap(),
            TechniqueTargetRequirement::HasArmor,
        );
        let base = TechniqueDefinition::new(
            technique.clone(),
            discipline.clone(),
            "technique.test_armor_break.name".to_owned(),
            "technique.test_armor_break.description".to_owned(),
            1,
            TechniqueKind::Action,
            None,
            [],
        )
        .unwrap();

        assert_eq!(
            base.clone()
                .with_action(TechniqueAction::AnalyzeTarget { range: 1 })
                .unwrap()
                .with_on_hit_effect(effect.clone()),
            Err(SkillDefinitionError::OnHitEffectWithoutWeaponAttack)
        );

        let definition = base
            .with_action(TechniqueAction::WeaponAttack {
                required_delivery: AttackDelivery::Melee,
                physical_damage_percentage: Some(60),
                armor_penetration_bonus: 0,
                accuracy_modifier: 0,
                energy_cost: 3,
                recovery_time_units: None,
                forced_movement: None,
                melee_arc: None,
            })
            .unwrap()
            .with_on_hit_effect(effect)
            .unwrap();
        let mut catalog = SkillCatalog::default();
        catalog
            .register_discipline(
                DisciplineDefinition::new(
                    discipline,
                    "discipline.test_melee.name".to_owned(),
                    "discipline.test_melee.description".to_owned(),
                )
                .unwrap(),
            )
            .unwrap();
        catalog.register_technique(definition).unwrap();

        assert_eq!(
            catalog.validate_status_references(&StatusCatalog::default()),
            Err(SkillCatalogError::UnknownTechniqueStatus {
                technique: Box::new(technique),
                status,
            })
        );
    }

    #[test]
    fn preparation_timing_requires_an_action_and_a_positive_duration() {
        let definition = TechniqueDefinition::new(
            id("core:test_preparation"),
            id("core:test"),
            "technique.test_preparation.name".to_owned(),
            "technique.test_preparation.description".to_owned(),
            1,
            TechniqueKind::Action,
            None,
            [],
        )
        .unwrap();

        assert_eq!(
            definition.clone().with_preparation_steps(1),
            Err(SkillDefinitionError::PreparationWithoutAction)
        );
        let executable = definition
            .with_action(TechniqueAction::ReadMovementTraces { radius: 2 })
            .unwrap();
        assert_eq!(
            executable.clone().with_preparation_steps(0),
            Err(SkillDefinitionError::ZeroPreparationSteps)
        );
        assert_eq!(
            executable
                .with_preparation_steps(2)
                .unwrap()
                .preparation_steps()
                .map(TimeUnits::get),
            Some(2)
        );
    }

    #[test]
    fn unknown_prerequisite_is_rejected_explicitly() {
        let mut catalog = SkillCatalog::default();
        catalog
            .register_discipline(
                DisciplineDefinition::new(
                    id("core:test"),
                    "discipline.test.name".to_owned(),
                    "discipline.test.description".to_owned(),
                )
                .unwrap_or_else(|error| panic!("valid discipline rejected: {error}")),
            )
            .unwrap_or_else(|error| panic!("valid registration rejected: {error}"));
        catalog
            .register_technique(
                TechniqueDefinition::new(
                    id("core:child"),
                    id("core:test"),
                    "technique.child.name".to_owned(),
                    "technique.child.description".to_owned(),
                    2,
                    TechniqueKind::Improvement,
                    Some(id("core:missing")),
                    [],
                )
                .unwrap_or_else(|error| panic!("valid technique shape rejected: {error}")),
            )
            .unwrap_or_else(|error| panic!("valid registration rejected: {error}"));

        assert!(matches!(
            catalog.validate_structure(),
            Err(SkillCatalogError::UnknownPrerequisite { .. })
        ));
    }

    #[test]
    fn cyclic_prerequisites_are_impossible_across_strictly_earlier_levels() {
        let mut catalog = SkillCatalog::default();
        catalog
            .register_discipline(
                DisciplineDefinition::new(
                    id("core:test"),
                    "discipline.test.name".to_owned(),
                    "discipline.test.description".to_owned(),
                )
                .unwrap_or_else(|error| panic!("valid discipline rejected: {error}")),
            )
            .unwrap_or_else(|error| panic!("valid registration rejected: {error}"));
        for (technique, prerequisite, minimum_level) in
            [("first", "second", 1), ("second", "first", 2)]
        {
            catalog
                .register_technique(
                    TechniqueDefinition::new(
                        id(&format!("core:{technique}")),
                        id("core:test"),
                        format!("technique.{technique}.name"),
                        format!("technique.{technique}.description"),
                        minimum_level,
                        TechniqueKind::Improvement,
                        Some(id(&format!("core:{prerequisite}"))),
                        [],
                    )
                    .unwrap_or_else(|error| panic!("valid technique shape rejected: {error}")),
                )
                .unwrap_or_else(|error| panic!("valid registration rejected: {error}"));
        }

        assert!(matches!(
            catalog.validate_structure(),
            Err(SkillCatalogError::PrerequisiteLevelNotEarlier { .. })
        ));
    }

    #[test]
    fn optional_diagnostic_can_be_absent_without_closing_reconnaissance() {
        let catalog = reconnaissance_catalog();
        let features = SystemFeatureSet::new([id("core:traces"), id("core:secrets")]);
        let availability = catalog
            .analyze_discipline(
                &id("core:reconnaissance"),
                &features,
                &[],
                &SkillProgressionRules::default(),
            )
            .unwrap_or_else(|error| panic!("valid version rejected: {error}"));

        assert!(availability.is_open());
        assert_eq!(availability.available.len(), 6);
        assert!(availability.unavailable.contains(&id("core:rec_08")));
    }

    #[test]
    fn incomplete_version_defers_the_whole_discipline() {
        let catalog = reconnaissance_catalog();
        let availability = catalog
            .analyze_discipline(
                &id("core:reconnaissance"),
                &SystemFeatureSet::default(),
                &[],
                &SkillProgressionRules::default(),
            )
            .unwrap_or_else(|error| panic!("version analysis failed: {error}"));

        assert!(!availability.is_open());
        assert_eq!(availability.available.len(), 4);
        assert_eq!(availability.complete_paths, 0);
    }

    #[test]
    fn disabling_a_parent_transitively_disables_its_improvement() {
        let mut catalog = reconnaissance_catalog();
        let parent = id("core:rec_01");
        catalog
            .techniques
            .get_mut(&parent)
            .unwrap_or_else(|| panic!("parent fixture missing"))
            .required_features
            .insert(id("core:target_analysis"));
        let availability = catalog
            .analyze_discipline(
                &id("core:reconnaissance"),
                &SystemFeatureSet::new([
                    id("core:traces"),
                    id("core:secrets"),
                    id("core:energy_states"),
                ]),
                &[],
                &SkillProgressionRules::default(),
            )
            .unwrap_or_else(|error| panic!("version analysis failed: {error}"));

        assert!(availability.unavailable.contains(&parent));
        assert!(availability.unavailable.contains(&id("core:rec_09")));
    }

    #[test]
    fn initial_grants_must_follow_prerequisite_order() {
        let catalog = reconnaissance_catalog();
        let features = SystemFeatureSet::new([
            id("core:traces"),
            id("core:secrets"),
            id("core:energy_states"),
        ]);

        assert_eq!(
            catalog.analyze_discipline(
                &id("core:reconnaissance"),
                &features,
                &[id("core:rec_09")],
                &SkillProgressionRules::default(),
            ),
            Err(InitialChoicesError::MissingPrerequisite(id("core:rec_09")))
        );
        assert!(
            catalog
                .analyze_discipline(
                    &id("core:reconnaissance"),
                    &features,
                    &[id("core:rec_01"), id("core:rec_02"), id("core:rec_09"),],
                    &SkillProgressionRules::default(),
                )
                .is_ok()
        );
    }

    #[test]
    fn final_cost_tier_does_not_limit_learned_techniques() {
        let catalog = reconnaissance_catalog();
        let features = SystemFeatureSet::new([
            id("core:traces"),
            id("core:secrets"),
            id("core:energy_states"),
        ]);
        let rules = SkillProgressionRules::default();
        let choices = [
            id("core:rec_01"),
            id("core:rec_02"),
            id("core:rec_03"),
            id("core:rec_04"),
            id("core:rec_05"),
            id("core:rec_09"),
            id("core:rec_08"),
        ];

        let availability = catalog
            .analyze_discipline(&id("core:reconnaissance"), &features, &choices, &rules)
            .unwrap();

        assert!(availability.is_open());
        assert_eq!(rules.configured_choice_count(), 5);
        assert_eq!(rules.cost_for_choice_number(5), Some(3));
        assert_eq!(rules.cost_for_choice_number(choices.len()), Some(3));
    }

    #[test]
    fn learning_checks_level_and_authored_primary_attributes_independently_from_choice_count() {
        let mut catalog = reconnaissance_catalog();
        let technique = id("core:rec_04");
        catalog
            .techniques
            .get_mut(&technique)
            .unwrap()
            .minimum_attributes
            .insert(PrimaryAttribute::Perception, 7);
        let features = SystemFeatureSet::new([
            id("core:traces"),
            id("core:secrets"),
            id("core:energy_states"),
        ]);
        let rules = SkillProgressionRules::default();
        let mut state = SkillProgressionState::default();

        assert!(matches!(
            state.learn(
                &technique,
                &catalog,
                &features,
                &rules,
                1,
                Some(PrimaryAttributes::new(5, 5, 5, 7, 5)),
            ),
            Err(TechniqueLearningError::LevelTooLow {
                required: 2,
                current: 1,
                ..
            })
        ));
        assert!(matches!(
            state.learn(
                &technique,
                &catalog,
                &features,
                &rules,
                2,
                Some(PrimaryAttributes::new(5, 5, 5, 6, 5)),
            ),
            Err(TechniqueLearningError::AttributeTooLow {
                attribute: PrimaryAttribute::Perception,
                required: 7,
                current: 6,
                ..
            })
        ));
        assert!(
            state
                .learn(
                    &technique,
                    &catalog,
                    &features,
                    &rules,
                    2,
                    Some(PrimaryAttributes::new(5, 5, 5, 7, 5)),
                )
                .is_ok()
        );
    }
}
