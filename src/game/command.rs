use crate::companion::CompanionBehavior;
use crate::drone::DroneDirective;
use crate::electronic_warfare::ElectronicDirective;
use crate::engineering::EngineeringDirective;
use crate::entity::{BodyComponentId, EntityId, EquipmentSlotId, ItemInstanceId};
use crate::intrusion::IntrusionDirective;
use crate::skills::TechniqueId;
use crate::world::{Direction, GridPos};

/// Player intent. Input code produces commands but never mutates the game.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameCommand {
    Move(Direction),
    Wait,
    SetCompanionBehavior {
        behavior: CompanionBehavior,
    },
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
    EquipItem {
        slot: EquipmentSlotId,
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
        /// Equipped weapon slot captured with the command when the selected
        /// technique executes a weapon attack. Other techniques leave it empty.
        weapon_slot: Option<u8>,
    },
    UseTechniqueOnComponent {
        technique: TechniqueId,
        target: EntityId,
        component: BodyComponentId,
        weapon_slot: u8,
    },
    /// Free world-space aim for a weapon technique whose authored area may be
    /// committed even when its center cell has no actor.
    UseTechniqueAt {
        technique: TechniqueId,
        target: GridPos,
        weapon_slot: u8,
    },
    UseDroneTechnique {
        technique: TechniqueId,
        directive: DroneDirective,
    },
    UseEngineeringTechnique {
        technique: TechniqueId,
        directive: EngineeringDirective,
    },
    UseIntrusionTechnique {
        technique: TechniqueId,
        directive: IntrusionDirective,
    },
    UseElectronicWarfareTechnique {
        technique: TechniqueId,
        directive: ElectronicDirective,
    },
}

impl GameCommand {
    /// Whether a successful command starts a normal simulation action. A
    /// rejected command never does; learning between commands is deliberately
    /// timeless and therefore does not refresh reaction availability.
    pub const fn consumes_time_on_success(&self) -> bool {
        !matches!(
            self,
            Self::LearnTechnique { .. } | Self::SetCompanionBehavior { .. }
        )
    }
}
