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
    SetCompanionBehaviorFor {
        entity: EntityId,
        behavior: CompanionBehavior,
    },
    EndMaintainedEffect {
        effect: super::MaintainedEffectId,
    },
    DismissCompanion {
        entity: EntityId,
    },
    Interact {
        target: GridPos,
    },
    Attack {
        slot: u8,
        target: EntityId,
    },
    /// A world-space aim used by area attacks. Single-target weapons with a
    /// secondary cone require an occupant; empty ground cannot spend a turn.
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
    BuyItem {
        merchant: EntityId,
        item: crate::item::ItemId,
    },
    BuyResaleItem {
        merchant: EntityId,
        listing: u64,
    },
    GambleItem {
        merchant: EntityId,
        item: crate::item::ItemId,
    },
    SellItem {
        merchant: EntityId,
        item: ItemInstanceId,
    },
    ImproveEquipment {
        artisan: EntityId,
        item: ItemInstanceId,
    },
    ReceiveTreatment {
        healer: EntityId,
    },
    AcceptQuest {
        giver: EntityId,
        quest: crate::content::ContentId,
    },
    CompleteQuest {
        giver: EntityId,
        quest: crate::content::ContentId,
    },
    ChooseDialogue {
        speaker: EntityId,
        node: String,
        choice: u16,
    },
    UseAbility {
        slot: u8,
        target: GridPos,
    },
    LearnTechnique {
        technique: TechniqueId,
    },
    LearnAgentSkill {
        skill: String,
    },
    AllocateAttribute {
        attribute: crate::stats::PrimaryAttribute,
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
    UseInstallation {
        target: GridPos,
        action: crate::facility::InstallationAction,
    },
    SetAgentMineVariant {
        variant: super::AgentMineVariant,
    },
    SetAgentGrenadeVariant {
        variant: super::AgentGrenadeVariant,
    },
}

impl GameCommand {
    /// Whether a successful command starts a normal simulation action. A
    /// rejected command never does; learning between commands is deliberately
    /// timeless and therefore does not refresh reaction availability.
    pub fn consumes_time_on_success(&self) -> bool {
        if matches!(self, Self::EquipItem { slot, .. } if slot.as_str() == crate::entity::BODY_APPEARANCE_SLOT)
        {
            return false;
        }
        !matches!(
            self,
            Self::SetAgentMineVariant { .. }
                | Self::SetAgentGrenadeVariant { .. }
                | Self::LearnTechnique { .. }
                | Self::LearnAgentSkill { .. }
                | Self::AllocateAttribute { .. }
                | Self::SetCompanionBehavior { .. }
                | Self::SetCompanionBehaviorFor { .. }
                | Self::AcceptQuest { .. }
                | Self::ChooseDialogue { .. }
        )
    }
}
