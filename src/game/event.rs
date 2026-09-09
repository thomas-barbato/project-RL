use crate::entity::{EntityId, EquipmentSlotId, ItemInstanceId};
use crate::progression::RewardKey;
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

/// Facts emitted by the simulation for rendering, audio, logs and tests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameEvent {
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
    AttackPerformed {
        attacker: EntityId,
        target: EntityId,
        slot: u8,
    },
    WeaponEquipped {
        entity: EntityId,
        slot: u8,
        equipment_slot: EquipmentSlotId,
        item: ItemInstanceId,
        weapon: crate::weapon::WeaponId,
        displaced: Option<ItemInstanceId>,
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
