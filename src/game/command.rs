use crate::entity::{EntityId, ItemInstanceId};
use crate::skills::TechniqueId;
use crate::world::{Direction, GridPos};

/// Player intent. Input code produces commands but never mutates the game.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameCommand {
    Move(Direction),
    Wait,
    Interact {
        target: GridPos,
    },
    Attack {
        slot: u8,
        target: EntityId,
    },
    /// A free world-space aim used by area attacks. Single-target attacks keep
    /// their stable entity target so an empty tile cannot silently consume a turn.
    AttackAt {
        slot: u8,
        target: GridPos,
    },
    EquipWeapon {
        slot: u8,
        item: ItemInstanceId,
    },
    UseItem {
        item: ItemInstanceId,
    },
    PickUp,
    DropItem {
        item: ItemInstanceId,
    },
    UseAbility {
        slot: u8,
        target: GridPos,
    },
    LearnTechnique {
        technique: TechniqueId,
    },
    UseTechnique {
        technique: TechniqueId,
        targets: Vec<EntityId>,
    },
}
