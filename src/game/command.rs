use crate::entity::{EntityId, ItemInstanceId};
use crate::world::{Direction, GridPos};

/// Player intent. Input code produces commands but never mutates the game.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GameCommand {
    Move(Direction),
    Wait,
    Attack { slot: u8, target: EntityId },
    EquipWeapon { slot: u8, item: ItemInstanceId },
    UseAbility { slot: u8, target: GridPos },
}
