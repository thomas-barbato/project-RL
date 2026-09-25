use crate::combat::PreparationDisruptionFamily;
use crate::entity::{EntityId, EquipmentSlotId, GroundItemId, ItemInstanceId};
use crate::progression::RewardKey;
use crate::skills::{DisciplineId, TechniqueId};
use crate::status::{StatusId, StatusTrigger};
use crate::world::GridPos;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExperienceSource {
    DefeatedEntity(EntityId),
    OneTimeReward(RewardKey),
    System,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum QuestCompletion {
    Delivery {
        item: crate::item::ItemId,
        quantity: u16,
    },
    ExploreZones {
        zones: u16,
    },
    AccessDataRecord {
        record: crate::content::ContentId,
    },
    DefeatTargets {
        target_tag: crate::content::ContentId,
        quantity: u16,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusRemovalReason {
    Expired,
    Cleansed,
    Consumed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerrainAnalysis {
    pub position: GridPos,
    pub terrain: crate::world::Terrain,
    pub blocks_movement: bool,
    pub blocks_vision: bool,
}

/// Energy-state fields that genuinely exist on the observed machine. Missing
/// channels remain `None` instead of being invented by the analysis action.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnergyAnalysis {
    pub analysis_score: u16,
    pub energy_available: Option<u16>,
    pub energy_capacity: Option<u16>,
    pub heat: Option<u16>,
    pub bandwidth_occupied: Option<u16>,
    pub bandwidth_capacity: Option<u16>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForcedMovementOutcome {
    Incompatible,
    Fixed,
    Resisted,
    Blocked,
    Moved,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CounterattackOutcome {
    Performed,
    ReactorUnavailable,
    SourceUnavailable,
    NoMeleeWeapon,
    OutOfReach,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InterceptionOutcome {
    Performed,
    Missed,
    Resisted {
        intensity: u16,
        chance: u8,
        roll: u8,
    },
    MovementStopped {
        intensity: u16,
        chance: u8,
        roll: u8,
    },
    ReactorUnavailable,
    MoverUnavailable,
    NoMeleeWeapon,
    OutOfReach,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreparationDisruptionOutcome {
    Protected,
    Resisted,
    Interrupted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TechniqueEffectFailure {
    TargetHasNoArmor,
    TargetHasNoCompatibleLocomotion,
    TargetHasNoCompatibleSuppressionResponse,
    ProtectedFromEffect,
}

/// Facts emitted by the simulation for rendering, audio, logs and tests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameEvent {
    /// Fuel captured on a direct hit, including a lethal one. The burst remains
    /// at that hit's position even if another effect displaced the target.
    StatusCatalyzed {
        source: EntityId,
        target: EntityId,
        at: GridPos,
        status: StatusId,
    },
    WeaponFlameConeResolved {
        source: EntityId,
        origin: GridPos,
        cells: Vec<crate::world::PropagationCell>,
    },
    CatalyticExplosion {
        source: EntityId,
        at: GridPos,
    },
    WeaponEchoScheduled {
        source: EntityId,
        at: GridPos,
        delay_turns: u16,
    },
    WeaponEchoResolved {
        source: EntityId,
        at: GridPos,
    },
    WeaponFractureResolved {
        source: EntityId,
        target: EntityId,
        at: GridPos,
        status: StatusId,
        charges: u16,
        threshold: u16,
    },
    AttackTelegraphed {
        attacker: EntityId,
        origin: GridPos,
        target_at: GridPos,
    },
    AllyHealed {
        medic: EntityId,
        target: EntityId,
        amount: u16,
    },
    ZoneChanged {
        from: crate::content::ContentId,
        to: crate::content::ContentId,
        arrival: GridPos,
    },
    Facility(crate::facility::FacilityEvent),
    ItemBought {
        merchant: EntityId,
        definition: crate::item::ItemId,
        price: u32,
        player_credits: u32,
    },
    ItemSold {
        merchant: EntityId,
        definition: crate::item::ItemId,
        price: u32,
        player_credits: u32,
    },
    GambleResolved {
        merchant: EntityId,
        definition: crate::item::ItemId,
        price: u32,
        player_credits: u32,
        modifiers: crate::entity::MagicItemModifiers,
    },
    TreatmentReceived {
        healer: EntityId,
        amount: u16,
        price: u32,
        player_credits: u32,
    },
    QuestAccepted {
        giver: EntityId,
        quest: crate::content::ContentId,
    },
    QuestProgressed {
        quest: crate::content::ContentId,
        current: u16,
        required: u16,
    },
    QuestCompleted {
        giver: EntityId,
        quest: crate::content::ContentId,
        objective: QuestCompletion,
        reward_credits: u32,
        reward_experience: u64,
        reward_items: Vec<crate::content::QuestItemRewardDefinition>,
        world_states: Vec<crate::content::QuestWorldStateDefinition>,
        world_effects: Vec<crate::content::QuestWorldEffectDefinition>,
        player_credits: u32,
    },
    TerrainInteracted {
        entity: EntityId,
        at: GridPos,
        terrain: crate::world::Terrain,
    },
    EntityMoved {
        entity: EntityId,
        from: GridPos,
        to: GridPos,
    },
    NoiseEmitted {
        source: Option<EntityId>,
        at: GridPos,
        intensity: u16,
    },
    MovementTimeCommitted {
        entity: EntityId,
        time_units: u16,
    },
    ForcedMovementResolved {
        source: EntityId,
        target: EntityId,
        from: GridPos,
        to: GridPos,
        force: u16,
        resistance: Option<u32>,
        requested_distance: u8,
        moved_distance: u8,
        outcome: ForcedMovementOutcome,
    },
    EntityWaited {
        entity: EntityId,
    },
    EntitySpawned {
        entity: EntityId,
        at: GridPos,
    },
    DroneControlEstablished {
        entity: EntityId,
        controller: EntityId,
        visual_profile: crate::content::ContentId,
        bandwidth_reserved: u16,
    },
    DroneControlReleased {
        entity: EntityId,
        controller: EntityId,
        bandwidth_released: u16,
    },
    DroneEnergyDepleted {
        entity: EntityId,
        at: GridPos,
    },
    DroneLinkLost {
        entity: EntityId,
        at: GridPos,
    },
    DroneLinkRestored {
        entity: EntityId,
        at: GridPos,
    },
    CompanionBehaviorChanged {
        entities: Vec<EntityId>,
        behavior: crate::companion::CompanionBehavior,
    },
    DroneOrderAdvanced {
        entity: EntityId,
        order: crate::drone::DroneOrder,
    },
    DronePositionConfirmed {
        entity: EntityId,
        at: GridPos,
        observed_on_turn: u64,
    },
    DroneCargoCollected {
        entity: EntityId,
        ground_item: GroundItemId,
        definition: crate::item::ItemId,
        quantity: u16,
    },
    DroneCargoDelivered {
        entity: EntityId,
        controller: EntityId,
        definition: crate::item::ItemId,
        quantity: u16,
    },
    DroneCollectionFailed {
        entity: EntityId,
        ground_item: GroundItemId,
    },
    DroneInterposed {
        entity: EntityId,
        protected: EntityId,
        attacker: EntityId,
        energy_spent: u16,
    },
    DroneExplorationReportReceived {
        entity: EntityId,
        cells: Vec<GridPos>,
        observed_on_turn: u64,
    },
    ThreatSourceDisabled {
        entity: EntityId,
        at: GridPos,
    },
    GroundItemSpawned {
        ground_item: GroundItemId,
        definition: crate::item::ItemId,
        quantity: u16,
        at: GridPos,
    },
    ActorEquipmentDropped {
        entity: EntityId,
        ground_item: GroundItemId,
        at: GridPos,
    },
    ItemPickedUp {
        entity: EntityId,
        ground_item: GroundItemId,
        definition: crate::item::ItemId,
        quantity: u16,
    },
    PropertyTakeWitnessed {
        taker: EntityId,
        witness: EntityId,
        owner: crate::social::SocialGroupId,
        definition: crate::item::ItemId,
        quantity: u16,
        at: GridPos,
    },
    PropertyTakeReported {
        source: EntityId,
        recipient: EntityId,
        owner: crate::social::SocialGroupId,
        definition: crate::item::ItemId,
        quantity: u16,
        at: GridPos,
    },
    LocalAlertRaised {
        source: EntityId,
        owner: crate::social::SocialGroupId,
        at: GridPos,
        duration_turns: u16,
    },
    ItemDropped {
        entity: EntityId,
        item: ItemInstanceId,
        ground_item: GroundItemId,
        definition: crate::item::ItemId,
        quantity: u16,
    },
    AttackPerformed {
        attacker: EntityId,
        /// Present for an entity-locked attack. A freely aimed area attack can
        /// legitimately target empty ground while affecting several actors.
        target: Option<EntityId>,
        slot: u8,
        origin: GridPos,
        target_at: GridPos,
        weapon: Option<crate::weapon::WeaponId>,
        damage_type: crate::combat::DamageType,
        affected_cells: Vec<crate::combat::AttackAreaCell>,
    },
    AmmunitionSpent {
        entity: EntityId,
        weapon: crate::weapon::WeaponId,
        amount: u16,
        remaining: u16,
    },
    /// Result of the single passive accuracy/evasion roll for an ordinary
    /// target. Area attacks and certain attacks against inert objects do not
    /// emit this event because they deliberately consume no such roll.
    AttackHitResolved {
        attacker: EntityId,
        target: EntityId,
        at: GridPos,
        chance: u8,
        roll: u8,
        hit: bool,
    },
    GroundEffectCreated {
        source: Option<EntityId>,
        effect: crate::effects::GroundEffectId,
        at: GridPos,
        remaining_turns: u16,
    },
    GroundEffectTriggered {
        source: Option<EntityId>,
        effect: crate::effects::GroundEffectId,
        at: GridPos,
        target: Option<EntityId>,
    },
    GroundEffectRemoved {
        effect: crate::effects::GroundEffectId,
        at: GridPos,
    },
    ExplosiveDeployed {
        entity: EntityId,
        device: crate::explosive::ExplosiveDeviceId,
        material: crate::item::ItemId,
        at: GridPos,
    },
    ExplosivePlacementResolved {
        entity: EntityId,
        aimed_at: GridPos,
        placed_at: GridPos,
        chance: u8,
        roll: u8,
    },
    ExplosiveTriggered {
        device: crate::explosive::ExplosiveDeviceId,
        source: Option<EntityId>,
        at: GridPos,
    },
    ExplosivePayloadResolved {
        device: crate::explosive::ExplosiveDeviceId,
        source: Option<EntityId>,
        at: GridPos,
        stage: u16,
        cells: Vec<crate::combat::AttackAreaCell>,
        final_stage: bool,
    },
    TerrainBreached {
        device: crate::explosive::ExplosiveDeviceId,
        cells: Vec<GridPos>,
    },
    ExplosiveNeutralized {
        entity: EntityId,
        device: crate::explosive::ExplosiveDeviceId,
        at: GridPos,
    },
    ExplosiveRecovered {
        entity: EntityId,
        device: crate::explosive::ExplosiveDeviceId,
        material: crate::item::ItemId,
    },
    ExplosivesProgrammed {
        entity: EntityId,
        devices: Vec<(crate::explosive::ExplosiveDeviceId, u16)>,
    },
    ExplosiveCamouflaged {
        entity: EntityId,
        device: crate::explosive::ExplosiveDeviceId,
        at: GridPos,
        optical_difficulty_bonus: i16,
    },
    SoundEmitterDeployed {
        entity: EntityId,
        emitter: crate::stealth::SoundEmitterId,
        at: GridPos,
        intensity: u16,
        remaining_phases: u16,
    },
    SoundEmitterExpired {
        emitter: crate::stealth::SoundEmitterId,
        at: GridPos,
    },
    WeaponEquipped {
        entity: EntityId,
        slot: u8,
        equipment_slot: EquipmentSlotId,
        item: ItemInstanceId,
        weapon: crate::weapon::WeaponId,
        displaced: Option<ItemInstanceId>,
    },
    ItemEquipped {
        entity: EntityId,
        equipment_slot: EquipmentSlotId,
        item: ItemInstanceId,
        definition: crate::item::ItemId,
        displaced: Option<ItemInstanceId>,
    },
    ItemUsed {
        entity: EntityId,
        item: ItemInstanceId,
        definition: crate::item::ItemId,
    },
    IntegrityRestored {
        entity: EntityId,
        amount: u16,
    },
    AbilityUsed {
        user: EntityId,
        slot: u8,
        target: GridPos,
    },
    /// Presentation of an equipment push; the ordinary forced-movement event
    /// separately reports its outcome. No extra attack or damage is implied.
    WeaponImpulseResolved {
        source: EntityId,
        target: EntityId,
        origin: GridPos,
        from: GridPos,
        to: GridPos,
    },
    PropagationResolved {
        source: Option<EntityId>,
        origin: GridPos,
        cells: Vec<crate::world::PropagationCell>,
        /// Exact definition and effect index; clients choose presentation without
        /// inferring it from whichever weapon happens to be equipped now.
        weapon_effect: Option<(crate::weapon::WeaponId, usize)>,
    },
    /// One guard consumed by one positive post-defense body impact.
    DamageGuardAbsorbed {
        target: EntityId,
        at: GridPos,
        status: StatusId,
        amount: u16,
    },
    DamageApplied {
        source: Option<EntityId>,
        target: EntityId,
        at: GridPos,
        amount: u16,
        damage_type: crate::combat::DamageType,
        effective_armor: u16,
        absorbed_by_armor: u16,
    },
    /// One indivisible impact carrying several damage families. The target's
    /// PV are changed once from the resolved total; components remain visible
    /// for combat logs and diagnostics without becoming separate impacts.
    DamageImpactApplied {
        source: Option<EntityId>,
        target: EntityId,
        at: GridPos,
        amount: u16,
        /// Typed amounts after armor/resistances, before guard and HP clamping.
        components: Vec<crate::combat::ResolvedDamageComponent>,
        effective_armor: u16,
        absorbed_by_armor: u16,
    },
    EntityDied {
        entity: EntityId,
        at: GridPos,
    },
    /// A death directly attributed to the player, with stable content tags
    /// captured before the actor leaves the registry. Quest tracking consumes
    /// this fact without reconstructing identity from visuals or combat logs.
    EntityDefeatedByPlayer {
        entity: EntityId,
        at: GridPos,
        tags: Vec<crate::content::ContentId>,
    },
    EntityDestructionTriggered {
        entity: EntityId,
        at: GridPos,
    },
    ExperienceAwarded {
        amount: u64,
        total: u64,
        source: ExperienceSource,
    },
    LevelGained {
        level: u16,
        skill_points_awarded: u16,
    },
    TechniqueLearned {
        entity: EntityId,
        technique: TechniqueId,
        discipline: DisciplineId,
        choice_number: usize,
        skill_points_spent: u16,
        skill_points_remaining: u32,
    },
    TechniqueUsed {
        entity: EntityId,
        technique: TechniqueId,
        observed_on_turn: u64,
    },
    EmissionSilenceChanged {
        entity: EntityId,
        channel: crate::stealth::SignatureChannel,
        silenced: bool,
    },
    LowProfileChanged {
        entity: EntityId,
        technique: TechniqueId,
        active: bool,
    },
    AmbushResolved {
        entity: EntityId,
        target: EntityId,
        bonuses_applied: bool,
        silent_neutralization: bool,
    },
    TrailBreakStarted {
        entity: EntityId,
        technique: TechniqueId,
        remaining_steps: u8,
        remaining_turns: u16,
    },
    TrailBreakAdvanced {
        entity: EntityId,
        remaining_steps: u8,
    },
    TrailBreakEnded {
        entity: EntityId,
        technique: TechniqueId,
    },
    ActiveCamouflageChanged {
        entity: EntityId,
        technique: TechniqueId,
        channel: crate::stealth::SignatureChannel,
        active: bool,
    },
    TechniquePreparationStarted {
        entity: EntityId,
        technique: TechniqueId,
        remaining_steps: u16,
    },
    TechniquePreparationAdvanced {
        entity: EntityId,
        technique: TechniqueId,
        remaining_steps: u16,
    },
    TechniquePreparationCompleted {
        entity: EntityId,
        technique: TechniqueId,
    },
    TechniquePreparationCancelled {
        entity: EntityId,
        technique: TechniqueId,
        reason: crate::game::PreparationCancellationReason,
    },
    ActionRecoveryStarted {
        entity: EntityId,
        remaining_actions: u16,
    },
    ActionRecoveryAdvanced {
        entity: EntityId,
        remaining_actions: u16,
    },
    ActionRecoveryCompleted {
        entity: EntityId,
    },
    TechniqueCooldownStarted {
        entity: EntityId,
        technique: TechniqueId,
        remaining_phases: u16,
    },
    TechniqueCooldownAdvanced {
        entity: EntityId,
        technique: TechniqueId,
        remaining_phases: u16,
    },
    TechniqueCooldownCompleted {
        entity: EntityId,
        technique: TechniqueId,
    },
    ReactionPrepared {
        entity: EntityId,
        technique: TechniqueId,
        reaction: crate::reaction::ReactionKind,
    },
    ReactionExpired {
        entity: EntityId,
        technique: TechniqueId,
        reaction: crate::reaction::ReactionKind,
    },
    ReactionTriggered {
        reactor: EntityId,
        source: EntityId,
        technique: TechniqueId,
        reaction: crate::reaction::ReactionKind,
    },
    PersistentRangedAimStarted {
        entity: EntityId,
        target: EntityId,
        technique: TechniqueId,
        accuracy_modifier: i16,
    },
    PersistentRangedAimEnded {
        entity: EntityId,
        target: EntityId,
        technique: TechniqueId,
    },
    WeaponBarrageStageResolved {
        entity: EntityId,
        technique: TechniqueId,
        remaining_stages: u8,
    },
    WeaponBarrageCancelled {
        entity: EntityId,
        technique: TechniqueId,
        remaining_stages: u8,
    },
    ChargeStarted {
        entity: EntityId,
        technique: TechniqueId,
        target: EntityId,
        target_at: GridPos,
        required_advances: u8,
    },
    ChargeAdvanced {
        entity: EntityId,
        technique: TechniqueId,
        from: GridPos,
        to: GridPos,
        completed_advances: u8,
        required_advances: u8,
    },
    ChargeCompleted {
        entity: EntityId,
        technique: TechniqueId,
        target: EntityId,
        recovery_suppressed: bool,
    },
    ChargeCancelled {
        entity: EntityId,
        technique: TechniqueId,
        completed_advances: u8,
        controlled: bool,
    },
    AnchorPrepared {
        entity: EntityId,
        technique: TechniqueId,
        displacement_resistance_bonus: u16,
    },
    AnchorEnded {
        entity: EntityId,
        technique: TechniqueId,
    },
    EvasiveStepResolved {
        entity: EntityId,
        from: GridPos,
        to: GridPos,
        moved: bool,
    },
    ObstacleTraversed {
        entity: EntityId,
        from: GridPos,
        over: GridPos,
        to: GridPos,
    },
    AllyExtracted {
        entity: EntityId,
        ally: EntityId,
        player_from: GridPos,
        player_to: GridPos,
        ally_from: GridPos,
        ally_to: GridPos,
    },
    PhysicalDamageParried {
        reactor: EntityId,
        source: EntityId,
        before: u16,
        after: u16,
    },
    CounterattackResolved {
        reactor: EntityId,
        source: EntityId,
        technique: TechniqueId,
        outcome: CounterattackOutcome,
    },
    InterceptionResolved {
        reactor: EntityId,
        mover: EntityId,
        technique: TechniqueId,
        from: GridPos,
        to: GridPos,
        outcome: InterceptionOutcome,
    },
    TechniqueOnHitEffectRejected {
        source: EntityId,
        target: EntityId,
        technique: TechniqueId,
        reason: TechniqueEffectFailure,
    },
    StabilityCheckResolved {
        source: EntityId,
        target: EntityId,
        technique: TechniqueId,
        intensity: u16,
        chance: u8,
        roll: u8,
        resisted: bool,
    },
    PreparationDisruptionResolved {
        source: EntityId,
        target: EntityId,
        family: PreparationDisruptionFamily,
        intensity: u16,
        chance: Option<u8>,
        roll: Option<u8>,
        outcome: PreparationDisruptionOutcome,
    },
    PreparationInterruptionProtectionChanged {
        entity: EntityId,
        family: PreparationDisruptionFamily,
        active: bool,
    },
    EnergySpent {
        entity: EntityId,
        amount: u16,
        remaining: u16,
    },
    BandwidthReserved {
        entity: EntityId,
        amount: u16,
        occupied: u16,
        capacity: u16,
    },
    BandwidthReleased {
        entity: EntityId,
        amount: u16,
        occupied: u16,
        capacity: u16,
    },
    HeatGenerated {
        entity: EntityId,
        amount: u16,
        current: u16,
    },
    HeatDissipated {
        entity: EntityId,
        amount: u16,
        current: u16,
    },
    HeatThresholdCrossed {
        entity: EntityId,
        critical: bool,
        current: u16,
    },
    TargetAnalyzed {
        observer: EntityId,
        target: EntityId,
        at: GridPos,
        integrity: u16,
        maximum_integrity: u16,
        armor: u16,
        resistances: crate::combat::ResistanceProfile,
    },
    PhysicalWeaknessIdentified {
        observer: EntityId,
        target: EntityId,
    },
    BodyComponentIdentified {
        observer: EntityId,
        target: EntityId,
        component: crate::entity::BodyComponentId,
    },
    BodyComponentDamaged {
        source: Option<EntityId>,
        target: EntityId,
        component: crate::entity::BodyComponentId,
        amount: u16,
        durability: u16,
        maximum_durability: u16,
        failed: bool,
    },
    WreckCreated {
        wreck: crate::engineering::WreckId,
        at: GridPos,
        components: Vec<crate::entity::BodyComponentId>,
    },
    BodyComponentRepaired {
        target: EntityId,
        component: crate::entity::BodyComponentId,
        amount: u16,
        durability: u16,
        maximum_durability: u16,
    },
    BodyComponentSalvaged {
        wreck: crate::engineering::WreckId,
        component: crate::entity::BodyComponentId,
        inventory_item: ItemInstanceId,
        durability: u16,
        maximum_durability: u16,
    },
    BodyComponentDiagnosed {
        target: EntityId,
        component: crate::entity::BodyComponentId,
        analysis_score: u16,
        durability: u16,
        maximum_durability: u16,
        failed: bool,
        destroyed: bool,
    },
    ModuleTuned {
        module: ItemInstanceId,
        tuning: crate::engineering::ModuleTuning,
        output_percentage: u16,
        energy_percentage: u16,
    },
    ModuleOverclockChanged {
        module: ItemInstanceId,
        output_percentage: u16,
        remaining_time_units: u16,
    },
    ModuleDurabilityDamaged {
        module: ItemInstanceId,
        amount: u16,
        durability: u16,
        maximum_durability: u16,
    },
    BodyComponentBypassed {
        target: EntityId,
        receiver: crate::entity::BodyComponentId,
        donor: crate::entity::BodyComponentId,
        restored_output_percentage: u16,
    },
    BodyComponentBypassEnded {
        target: EntityId,
        receiver: crate::entity::BodyComponentId,
        donor: crate::entity::BodyComponentId,
    },
    ModuleReconditioned {
        module: ItemInstanceId,
        amount: u16,
        durability: u16,
        maximum_durability: u16,
    },
    FieldBeaconAssembled {
        emitter: crate::stealth::SoundEmitterId,
        at: GridPos,
        integrity: u16,
        stored_energy: u16,
        energy_per_phase: u16,
    },
    DigitalInterfaceProbed {
        at: GridPos,
        analysis_score: u16,
        rights: Vec<crate::intrusion::AccessRight>,
        defense: u16,
        trace: crate::intrusion::SecurityTraceId,
    },
    IntrusionAttemptResolved {
        at: GridPos,
        chance: u8,
        roll: u8,
        succeeded: bool,
        hardening: u16,
        trace: crate::intrusion::SecurityTraceId,
    },
    DigitalAccessGranted {
        at: GridPos,
        origin: crate::intrusion::AccessOrigin,
        rights: Vec<crate::intrusion::AccessRight>,
        remaining_time_units: u16,
    },
    DigitalAccessExpired {
        at: GridPos,
    },
    ElectronicLockForced {
        interface: GridPos,
        door: GridPos,
    },
    DataLotExtracted {
        source: GridPos,
        recorded_on_turn: u64,
        extracted_on_turn: u64,
    },
    DeviceControlChanged {
        at: GridPos,
        command: crate::intrusion::DeviceCommand,
        active: bool,
    },
    DeviceControlRecaptureBlocked {
        at: GridPos,
    },
    DigitalRoutineChanged {
        at: GridPos,
        routine: crate::intrusion::DigitalRoutine,
        suspended: bool,
    },
    BackdoorChanged {
        at: GridPos,
        installed: bool,
    },
    SecurityTraceFalsified {
        trace: crate::intrusion::SecurityTraceId,
        at: GridPos,
    },
    SecurityTraceAudited {
        trace: crate::intrusion::SecurityTraceId,
        at: GridPos,
        falsified: bool,
    },
    SubnetCommandIssued {
        devices: Vec<GridPos>,
        command: crate::intrusion::DeviceCommand,
    },
    DeviceControlLockChanged {
        at: GridPos,
        active: bool,
    },
    ElectronicPulseResolved {
        source: EntityId,
        cells: Vec<GridPos>,
        affected: Vec<EntityId>,
        disruption_intensity: u16,
    },
    HostileProgramAttemptResolved {
        source: EntityId,
        target: EntityId,
        chance: u8,
        roll: u8,
        succeeded: bool,
    },
    HostileProgramChanged {
        program: crate::electronic_warfare::HostileProgramId,
        target: EntityId,
        active: bool,
    },
    HostileProgramTicked {
        program: crate::electronic_warfare::HostileProgramId,
        target: EntityId,
    },
    ElectronicJammingChanged {
        source: EntityId,
        channel: crate::electronic_warfare::ElectronicChannel,
        active: bool,
    },
    ElectronicCascadeResolved {
        source: EntityId,
        targets: Vec<EntityId>,
    },
    SaturationBeaconDeployed {
        beacon: EntityId,
        at: GridPos,
        active: bool,
    },
    SaturationBeaconActivated {
        beacon: EntityId,
    },
    SaturationBeaconExpired {
        beacon: EntityId,
        at: GridPos,
    },
    ElectronicImplosionDetonated {
        program: crate::electronic_warfare::HostileProgramId,
        target: EntityId,
        at: GridPos,
    },
    MovementTracesRead {
        observer: EntityId,
        traces: Vec<crate::world::ObservedMovementTrace>,
    },
    SecretsInspected {
        observer: EntityId,
        discovered_explosives: Vec<crate::explosive::ExplosiveDeviceId>,
    },
    TerrainAnalyzed {
        observer: EntityId,
        tiles: Vec<TerrainAnalysis>,
    },
    ThreatAnalyzed {
        observer: EntityId,
        target: EntityId,
        attacks: Vec<crate::combat::AttackProfile>,
        armor: u16,
        resistances: crate::combat::ResistanceProfile,
    },
    EnergyAnalyzed {
        observer: EntityId,
        target: EntityId,
        state: EnergyAnalysis,
    },
    StatusApplied {
        source: Option<EntityId>,
        target: EntityId,
        status: StatusId,
        stacks: u16,
        remaining_turns: Option<u16>,
        application: crate::status::StatusApplyKind,
    },
    StatusApplicationBlocked {
        source: Option<EntityId>,
        target: EntityId,
        status: StatusId,
        blocking_status: StatusId,
    },
    StatusTriggered {
        target: EntityId,
        status: StatusId,
        trigger: StatusTrigger,
    },
    StatusRemoved {
        target: EntityId,
        status: StatusId,
        reason: StatusRemovalReason,
    },
    VisibilityUpdated {
        observer: EntityId,
        origin: GridPos,
    },
    ExitReached {
        entity: EntityId,
        at: GridPos,
    },
    TurnCompleted {
        turn: u64,
    },
}
