use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::content::ContentId;

use super::{Inventory, ItemInstanceId};

/// Content-addressed slot ID. The ordered list of slots lives in `GameRules`,
/// so a ruleset can replace or extend the loadout without changing this type.
pub type EquipmentSlotId = ContentId;

/// Cosmetic reference stored in the existing slot map, preserving save layout.
/// It is deliberately excluded from the mechanical equipment iterator.
pub const BODY_APPEARANCE_SLOT: &str = "core:body_appearance";

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Equipment {
    slots: BTreeMap<EquipmentSlotId, ItemInstanceId>,
}

impl Equipment {
    pub fn equipped(&self, slot: &EquipmentSlotId) -> Option<ItemInstanceId> {
        self.slots.get(slot).copied()
    }

    pub fn slot_of(&self, item: ItemInstanceId) -> Option<&EquipmentSlotId> {
        self.iter()
            .find_map(|(slot, equipped)| (equipped == item).then_some(slot))
            .or_else(|| {
                self.slots.iter().find_map(|(slot, equipped)| {
                    (slot.as_str() == BODY_APPEARANCE_SLOT && *equipped == item).then_some(slot)
                })
            })
    }

    /// Real equipment only: appearance must never contribute stats or effects.
    pub fn iter(&self) -> impl Iterator<Item = (&EquipmentSlotId, ItemInstanceId)> {
        self.slots
            .iter()
            .filter(|(slot, _)| slot.as_str() != BODY_APPEARANCE_SLOT)
            .map(|(slot, item)| (slot, *item))
    }

    pub fn body_appearance(&self) -> Option<ItemInstanceId> {
        self.slots
            .iter()
            .find_map(|(slot, item)| (slot.as_str() == BODY_APPEARANCE_SLOT).then_some(*item))
    }

    /// Selects an owned appearance without moving or duplicating its instance.
    pub(crate) fn set_body_appearance(
        &mut self,
        item: Option<ItemInstanceId>,
        inventory: &Inventory,
    ) -> Result<(), EquipmentError> {
        if let Some(item) = item
            && inventory.get(item).is_none()
        {
            return Err(EquipmentError::ItemNotInInventory(item));
        }
        let slot = BODY_APPEARANCE_SLOT
            .parse()
            .expect("static appearance slot");
        if let Some(item) = item {
            self.slots.insert(slot, item);
        } else {
            self.slots.remove(&slot);
        }
        Ok(())
    }

    /// Equips one inventory instance. An instance can occupy only one slot.
    pub fn equip(
        &mut self,
        slot: EquipmentSlotId,
        item: ItemInstanceId,
        inventory: &Inventory,
    ) -> Result<EquipOutcome, EquipmentError> {
        if inventory.get(item).is_none() {
            return Err(EquipmentError::ItemNotInInventory(item));
        }

        let previous_slot = self
            .iter()
            .find_map(|(slot, equipped)| (equipped == item).then(|| slot.clone()));
        if let Some(previous_slot) = &previous_slot {
            self.slots.remove(previous_slot);
        }
        let displaced = self.slots.insert(slot, item).filter(|other| *other != item);
        Ok(EquipOutcome {
            previous_slot,
            displaced,
        })
    }

    pub fn unequip(&mut self, slot: &EquipmentSlotId) -> Option<ItemInstanceId> {
        self.slots.remove(slot)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EquipOutcome {
    pub previous_slot: Option<EquipmentSlotId>,
    pub displaced: Option<ItemInstanceId>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EquipmentError {
    ItemNotInInventory(ItemInstanceId),
}

impl Display for EquipmentError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ItemNotInInventory(item) => {
                write!(
                    formatter,
                    "item instance {} is not in this inventory",
                    item.get()
                )
            }
        }
    }
}

impl Error for EquipmentError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn content_id(id: &str) -> ContentId {
        id.parse()
            .unwrap_or_else(|error| panic!("valid content ID rejected: {error}"))
    }

    #[test]
    fn moving_an_item_between_slots_clears_its_previous_slot() {
        let mut inventory = Inventory::new(2);
        let item = inventory
            .add(content_id("core:blade"), 1, 1)
            .unwrap_or_else(|error| panic!("valid addition rejected: {error}"))[0];
        let primary = content_id("core:primary_weapon");
        let auxiliary = content_id("core:auxiliary_weapon");
        let mut equipment = Equipment::default();

        equipment
            .equip(primary.clone(), item, &inventory)
            .unwrap_or_else(|error| panic!("valid equipment rejected: {error}"));
        equipment
            .equip(auxiliary.clone(), item, &inventory)
            .unwrap_or_else(|error| panic!("valid equipment rejected: {error}"));

        assert_eq!(equipment.equipped(&primary), None);
        assert_eq!(equipment.equipped(&auxiliary), Some(item));
    }

    #[test]
    fn equipment_rejects_an_unknown_instance() {
        let mut inventory = Inventory::new(1);
        let removed = inventory
            .add(content_id("core:blade"), 1, 1)
            .unwrap_or_else(|error| panic!("valid addition rejected: {error}"))[0];
        inventory
            .remove(removed, 1)
            .unwrap_or_else(|error| panic!("valid removal rejected: {error}"));
        let mut equipment = Equipment::default();

        assert_eq!(
            equipment.equip(content_id("core:primary_weapon"), removed, &inventory),
            Err(EquipmentError::ItemNotInInventory(removed))
        );
    }
}
