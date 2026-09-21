use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

pub use crate::item::ItemId;
use crate::social::SocialGroupId;

/// Stable identity of one inventory stack during a run.
///
/// UI code and equipment refer to this value instead of a vector index, so
/// sorting or removing another entry cannot silently retarget an action.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct ItemInstanceId(u64);

impl ItemInstanceId {
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Rolled properties carried by one identified magical equipment instance.
/// White items keep this absent, preserving their historical stacking rules.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MagicItemModifiers {
    armor_bonus: u16,
    mass_reduction_percent: u8,
}

impl MagicItemModifiers {
    pub const fn new(armor_bonus: u16, mass_reduction_percent: u8) -> Option<Self> {
        if armor_bonus == 0 || mass_reduction_percent == 0 || mass_reduction_percent > 80 {
            return None;
        }
        Some(Self {
            armor_bonus,
            mass_reduction_percent,
        })
    }

    pub const fn armor_bonus(self) -> u16 {
        self.armor_bonus
    }

    pub const fn mass_reduction_percent(self) -> u8 {
        self.mass_reduction_percent
    }
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct InventoryEntry {
    instance: ItemInstanceId,
    item: ItemId,
    quantity: u16,
    owner: Option<SocialGroupId>,
    magic_modifiers: Option<MagicItemModifiers>,
}

impl Debug for InventoryEntry {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut entry = formatter.debug_struct("InventoryEntry");
        entry
            .field("instance", &self.instance)
            .field("item", &self.item)
            .field("quantity", &self.quantity);
        if let Some(owner) = &self.owner {
            entry.field("owner", owner);
        }
        if let Some(modifiers) = self.magic_modifiers {
            entry.field("magic_modifiers", &modifiers);
        }
        entry.finish()
    }
}

impl InventoryEntry {
    pub const fn instance(&self) -> ItemInstanceId {
        self.instance
    }

    pub const fn item(&self) -> &ItemId {
        &self.item
    }

    pub const fn quantity(&self) -> u16 {
        self.quantity
    }

    pub const fn owner(&self) -> Option<&SocialGroupId> {
        self.owner.as_ref()
    }

    pub const fn magic_modifiers(&self) -> Option<MagicItemModifiers> {
        self.magic_modifiers
    }
}

/// Slot-based inventory with deterministic insertion and stacking order.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Inventory {
    capacity: usize,
    entries: Vec<InventoryEntry>,
    next_instance: u64,
}

impl Inventory {
    pub const fn new(capacity: usize) -> Self {
        Self {
            capacity,
            entries: Vec::new(),
            next_instance: 1,
        }
    }

    pub const fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn remaining_slots(&self) -> usize {
        self.capacity.saturating_sub(self.entries.len())
    }

    pub fn get(&self, instance: ItemInstanceId) -> Option<&InventoryEntry> {
        self.entries.iter().find(|entry| entry.instance == instance)
    }

    pub fn iter(&self) -> impl Iterator<Item = &InventoryEntry> {
        self.entries.iter()
    }

    /// Adds a quantity atomically and returns every stack that received items.
    ///
    /// `maximum_stack` comes from the resolved item definition. Weapons use
    /// one, while future consumables and materials may opt into larger stacks.
    pub fn add(
        &mut self,
        item: ItemId,
        quantity: u16,
        maximum_stack: u16,
    ) -> Result<Vec<ItemInstanceId>, InventoryError> {
        self.add_with_owner(item, quantity, maximum_stack, None)
    }

    /// Owned and unowned lots never merge: provenance remains attached to the
    /// exact quantity through pickup, stacking and dropping.
    pub fn add_with_owner(
        &mut self,
        item: ItemId,
        quantity: u16,
        maximum_stack: u16,
        owner: Option<SocialGroupId>,
    ) -> Result<Vec<ItemInstanceId>, InventoryError> {
        if quantity == 0 {
            return Err(InventoryError::ZeroQuantity);
        }
        if maximum_stack == 0 {
            return Err(InventoryError::ZeroMaximumStack);
        }

        let reusable_capacity: u32 = self
            .entries
            .iter()
            .filter(|entry| {
                entry.item == item && entry.owner == owner && entry.magic_modifiers.is_none()
            })
            .map(|entry| u32::from(maximum_stack.saturating_sub(entry.quantity)))
            .sum();
        let remaining_after_reuse = u32::from(quantity).saturating_sub(reusable_capacity);
        let required_new_slots = if remaining_after_reuse == 0 {
            0
        } else {
            remaining_after_reuse.div_ceil(u32::from(maximum_stack)) as usize
        };
        if required_new_slots > self.remaining_slots() {
            let storable = reusable_capacity
                .saturating_add(
                    (self.remaining_slots() as u32).saturating_mul(u32::from(maximum_stack)),
                )
                .min(u32::from(u16::MAX)) as u16;
            return Err(InventoryError::InsufficientCapacity {
                requested: quantity,
                storable,
            });
        }
        let required_new_ids =
            u64::try_from(required_new_slots).map_err(|_| InventoryError::InstanceIdExhausted)?;
        self.next_instance
            .checked_add(required_new_ids)
            .ok_or(InventoryError::InstanceIdExhausted)?;

        let mut remaining = quantity;
        let mut affected = Vec::new();
        for entry in self.entries.iter_mut().filter(|entry| {
            entry.item == item && entry.owner == owner && entry.magic_modifiers.is_none()
        }) {
            if remaining == 0 {
                break;
            }
            let added = remaining.min(maximum_stack.saturating_sub(entry.quantity));
            if added > 0 {
                entry.quantity += added;
                remaining -= added;
                affected.push(entry.instance);
            }
        }
        while remaining > 0 {
            let stacked = remaining.min(maximum_stack);
            let instance = ItemInstanceId(self.next_instance);
            self.next_instance += 1;
            self.entries.push(InventoryEntry {
                instance,
                item: item.clone(),
                quantity: stacked,
                owner: owner.clone(),
                magic_modifiers: None,
            });
            affected.push(instance);
            remaining -= stacked;
        }
        Ok(affected)
    }

    /// Adds one identified magical item without merging it into white stacks.
    pub fn add_magic(
        &mut self,
        item: ItemId,
        owner: Option<SocialGroupId>,
        modifiers: MagicItemModifiers,
    ) -> Result<ItemInstanceId, InventoryError> {
        if self.remaining_slots() == 0 {
            return Err(InventoryError::InsufficientCapacity {
                requested: 1,
                storable: 0,
            });
        }
        self.next_instance
            .checked_add(1)
            .ok_or(InventoryError::InstanceIdExhausted)?;
        let instance = ItemInstanceId(self.next_instance);
        self.next_instance += 1;
        self.entries.push(InventoryEntry {
            instance,
            item,
            quantity: 1,
            owner,
            magic_modifiers: Some(modifiers),
        });
        Ok(instance)
    }

    pub fn remove(
        &mut self,
        instance: ItemInstanceId,
        quantity: u16,
    ) -> Result<(), InventoryError> {
        if quantity == 0 {
            return Err(InventoryError::ZeroQuantity);
        }
        let index = self
            .entries
            .iter()
            .position(|entry| entry.instance == instance)
            .ok_or(InventoryError::UnknownInstance(instance))?;
        let available = self.entries[index].quantity;
        if quantity > available {
            return Err(InventoryError::InsufficientQuantity {
                instance,
                requested: quantity,
                available,
            });
        }
        if quantity == available {
            self.entries.remove(index);
        } else {
            self.entries[index].quantity -= quantity;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InventoryError {
    ZeroQuantity,
    ZeroMaximumStack,
    InsufficientCapacity {
        requested: u16,
        storable: u16,
    },
    UnknownInstance(ItemInstanceId),
    InsufficientQuantity {
        instance: ItemInstanceId,
        requested: u16,
        available: u16,
    },
    InstanceIdExhausted,
}

impl Display for InventoryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroQuantity => write!(formatter, "item quantity must be positive"),
            Self::ZeroMaximumStack => write!(formatter, "maximum stack must be positive"),
            Self::InsufficientCapacity {
                requested,
                storable,
            } => write!(
                formatter,
                "inventory cannot store {requested} items; available capacity fits {storable}"
            ),
            Self::UnknownInstance(instance) => {
                write!(formatter, "unknown item instance {}", instance.get())
            }
            Self::InsufficientQuantity {
                instance,
                requested,
                available,
            } => write!(
                formatter,
                "item instance {} has quantity {available}, cannot remove {requested}",
                instance.get()
            ),
            Self::InstanceIdExhausted => write!(formatter, "item instance ID space exhausted"),
        }
    }
}

impl Error for InventoryError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(id: &str) -> ItemId {
        id.parse()
            .unwrap_or_else(|error| panic!("valid item ID rejected: {error}"))
    }

    #[test]
    fn stacking_is_deterministic_and_keeps_instance_ids_stable() {
        let mut inventory = Inventory::new(3);
        let first = inventory
            .add(item("core:repair_charge"), 3, 4)
            .unwrap_or_else(|error| panic!("valid addition rejected: {error}"))[0];
        let affected = inventory
            .add(item("core:repair_charge"), 4, 4)
            .unwrap_or_else(|error| panic!("valid addition rejected: {error}"));

        assert_eq!(affected[0], first);
        assert_eq!(inventory.get(first).map(InventoryEntry::quantity), Some(4));
        assert_eq!(
            inventory
                .iter()
                .map(InventoryEntry::quantity)
                .collect::<Vec<_>>(),
            vec![4, 3]
        );
    }

    #[test]
    fn failed_addition_is_atomic() {
        let mut inventory = Inventory::new(1);
        let instance = inventory
            .add(item("core:charge"), 2, 2)
            .unwrap_or_else(|error| panic!("valid addition rejected: {error}"))[0];
        let before = inventory.clone();

        assert_eq!(
            inventory.add(item("core:other"), 1, 1),
            Err(InventoryError::InsufficientCapacity {
                requested: 1,
                storable: 0,
            })
        );
        assert_eq!(inventory, before);
        assert_eq!(
            inventory.get(instance).map(InventoryEntry::quantity),
            Some(2)
        );
    }

    #[test]
    fn removing_a_whole_stack_does_not_retarget_other_instances() {
        let mut inventory = Inventory::new(3);
        let first = inventory
            .add(item("core:first"), 1, 1)
            .unwrap_or_else(|error| panic!("valid addition rejected: {error}"))[0];
        let second = inventory
            .add(item("core:second"), 1, 1)
            .unwrap_or_else(|error| panic!("valid addition rejected: {error}"))[0];

        inventory
            .remove(first, 1)
            .unwrap_or_else(|error| panic!("valid removal rejected: {error}"));

        assert!(inventory.get(first).is_none());
        assert_eq!(
            inventory.get(second).map(InventoryEntry::item),
            Some(&item("core:second"))
        );
    }

    #[test]
    fn differently_owned_lots_never_merge() {
        let mut inventory = Inventory::new(3);
        let definition = item("core:component");
        let owner: SocialGroupId = "core:maintainers".parse().unwrap();
        let unowned = inventory.add(definition.clone(), 2, 4).unwrap()[0];
        let owned = inventory
            .add_with_owner(definition, 2, 4, Some(owner.clone()))
            .unwrap()[0];

        assert_ne!(unowned, owned);
        assert_eq!(inventory.get(unowned).unwrap().owner(), None);
        assert_eq!(inventory.get(owned).unwrap().owner(), Some(&owner));
        assert_eq!(inventory.len(), 2);
    }
}
