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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusRemovalReason {
    Expired,
    Cleansed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerrainAnalysis {
    pub position: GridPos,
    pub terrain: crate::world::Terrain,
    pub blocks_movement: bool,
    pub blocks_vision: bool,
}

/// Facts emitted by the simulation for rendering, audio, logs and tests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameEvent {
    ZoneChanged {
        from: crate::content::ContentId,
        to: crate::content::ContentId,
        arrival: GridPos,
    },
    Facility(crate::facility::FacilityEvent),
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
    EntityWaited {
        entity: EntityId,
    },
    EntitySpawned {
        entity: EntityId,
        at: GridPos,
    },
    GroundItemSpawned {
        ground_item: GroundItemId,
        definition: crate::item::ItemId,
        quantity: u16,
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
    WeaponEquipped {
        entity: EntityId,
        slot: u8,
        equipment_slot: EquipmentSlotId,
        item: ItemInstanceId,
        weapon: crate::weapon::WeaponId,
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
    PropagationResolved {
        source: Option<EntityId>,
        origin: GridPos,
        cells: Vec<crate::world::PropagationCell>,
    },
    DamageApplied {
        source: Option<EntityId>,
        target: EntityId,
        amount: u16,
        damage_type: crate::combat::DamageType,
    },
    EntityDied {
        entity: EntityId,
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
        rank: u8,
        skill_points_spent: u16,
        skill_points_remaining: u32,
    },
    TechniqueUsed {
        entity: EntityId,
        technique: TechniqueId,
        observed_on_turn: u64,
    },
    EnergySpent {
        entity: EntityId,
        amount: u16,
        remaining: u16,
    },
    TargetAnalyzed {
        observer: EntityId,
        target: EntityId,
        at: GridPos,
        integrity: u16,
        maximum_integrity: u16,
        resistances: crate::combat::ResistanceProfile,
    },
    MovementTracesRead {
        observer: EntityId,
        traces: Vec<crate::world::ObservedMovementTrace>,
    },
    TerrainAnalyzed {
        observer: EntityId,
        tiles: Vec<TerrainAnalysis>,
    },
    ThreatAnalyzed {
        observer: EntityId,
        target: EntityId,
        attacks: Vec<crate::combat::AttackProfile>,
        resistances: crate::combat::ResistanceProfile,
    },
    StatusApplied {
        source: Option<EntityId>,
        target: EntityId,
        status: StatusId,
        stacks: u16,
        remaining_turns: Option<u16>,
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
