use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::item::ItemId;
use crate::social::SocialGroupId;
use crate::world::GridPos;

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct GroundItemId(u64);

impl GroundItemId {
    const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GroundItem {
    position: GridPos,
    item: ItemId,
    quantity: u16,
    owner: Option<SocialGroupId>,
    #[serde(default)]
    magic_modifiers: Option<super::MagicItemModifiers>,
}

impl Debug for GroundItem {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut item = formatter.debug_struct("GroundItem");
        item.field("position", &self.position)
            .field("item", &self.item)
            .field("quantity", &self.quantity);
        if let Some(owner) = &self.owner {
            item.field("owner", owner);
        }
        if let Some(bonus) = &self.magic_modifiers {
            item.field("magic_modifiers", bonus);
        }
        item.finish()
    }
}

impl GroundItem {
    pub const fn position(&self) -> GridPos {
        self.position
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

    pub fn magic_modifiers(&self) -> Option<super::MagicItemModifiers> {
        self.magic_modifiers.clone()
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct GroundItemRegistry {
    items: BTreeMap<GroundItemId, GroundItem>,
    next_id: u64,
}

impl Default for GroundItemRegistry {
    fn default() -> Self {
        Self {
            items: BTreeMap::new(),
            next_id: 1,
        }
    }
}

impl GroundItemRegistry {
    pub(crate) fn synchronize_ids(&mut self, other: &Self) {
        self.next_id = self.next_id.max(other.next_id);
    }

    pub fn spawn(
        &mut self,
        position: GridPos,
        item: ItemId,
        quantity: u16,
    ) -> Result<GroundItemId, GroundItemRegistryError> {
        self.spawn_with_owner(position, item, quantity, None)
    }

    pub fn spawn_with_owner(
        &mut self,
        position: GridPos,
        item: ItemId,
        quantity: u16,
        owner: Option<SocialGroupId>,
    ) -> Result<GroundItemId, GroundItemRegistryError> {
        if quantity == 0 {
            return Err(GroundItemRegistryError::ZeroQuantity);
        }
        if self.item_at(position).is_some() {
            return Err(GroundItemRegistryError::Occupied(position));
        }
        self.insert_at(position, item, quantity, owner, None)
    }

    /// Death must not erase loot just because another item occupies the tile.
    /// Ordinary placements retain their historical one-stack-per-cell rule.
    pub(crate) fn spawn_remains(
        &mut self,
        position: GridPos,
        item: ItemId,
        modifiers: Option<super::MagicItemModifiers>,
    ) -> Result<GroundItemId, GroundItemRegistryError> {
        self.insert_at(position, item, 1, None, modifiers)
    }

    fn insert_at(
        &mut self,
        position: GridPos,
        item: ItemId,
        quantity: u16,
        owner: Option<SocialGroupId>,
        magic_modifiers: Option<super::MagicItemModifiers>,
    ) -> Result<GroundItemId, GroundItemRegistryError> {
        let following_id = self
            .next_id
            .checked_add(1)
            .ok_or(GroundItemRegistryError::IdSpaceExhausted)?;
        let id = GroundItemId::new(self.next_id);
        self.next_id = following_id;
        self.items.insert(
            id,
            GroundItem {
                position,
                item,
                quantity,
                owner,
                magic_modifiers,
            },
        );
        Ok(id)
    }

    pub fn get(&self, id: GroundItemId) -> Option<&GroundItem> {
        self.items.get(&id)
    }

    pub(crate) fn retain_magic_modifiers(
        &mut self,
        id: GroundItemId,
        modifiers: super::MagicItemModifiers,
    ) {
        if let Some(item) = self.items.get_mut(&id) {
            item.magic_modifiers = Some(modifiers);
        }
    }

    pub fn item_at(&self, position: GridPos) -> Option<GroundItemId> {
        self.items
            .iter()
            .find_map(|(id, item)| (item.position == position).then_some(*id))
    }

    pub fn count_at(&self, position: GridPos) -> usize {
        self.items
            .values()
            .filter(|item| item.position == position)
            .count()
    }

    pub fn iter(&self) -> impl Iterator<Item = (GroundItemId, &GroundItem)> {
        self.items.iter().map(|(id, item)| (*id, item))
    }

    pub fn remove(&mut self, id: GroundItemId) -> Option<GroundItem> {
        self.items.remove(&id)
    }

    /// Detaches an exact quantity from a ground stack without changing its
    /// definition or position. Logistics systems can then move that returned
    /// stack to one—and only one—new custodian.
    pub fn take(
        &mut self,
        id: GroundItemId,
        quantity: u16,
    ) -> Result<GroundItem, GroundItemTakeError> {
        if quantity == 0 {
            return Err(GroundItemTakeError::ZeroQuantity);
        }
        let item = self
            .items
            .get_mut(&id)
            .ok_or(GroundItemTakeError::UnknownItem(id))?;
        if item.quantity < quantity {
            return Err(GroundItemTakeError::InsufficientQuantity {
                available: item.quantity,
                requested: quantity,
            });
        }
        let detached = GroundItem {
            position: item.position,
            item: item.item.clone(),
            quantity,
            owner: item.owner.clone(),
            magic_modifiers: item.magic_modifiers.clone(),
        };
        item.quantity -= quantity;
        if item.quantity == 0 {
            self.items.remove(&id);
        }
        Ok(detached)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroundItemTakeError {
    ZeroQuantity,
    UnknownItem(GroundItemId),
    InsufficientQuantity { available: u16, requested: u16 },
}

impl Display for GroundItemTakeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroQuantity => write!(formatter, "taken quantity must be positive"),
            Self::UnknownItem(id) => write!(formatter, "unknown ground item {}", id.get()),
            Self::InsufficientQuantity {
                available,
                requested,
            } => write!(
                formatter,
                "ground stack has {available} units but {requested} were requested"
            ),
        }
    }
}

impl Error for GroundItemTakeError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroundItemRegistryError {
    ZeroQuantity,
    Occupied(GridPos),
    IdSpaceExhausted,
}

impl Display for GroundItemRegistryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroQuantity => write!(formatter, "ground item quantity must be positive"),
            Self::Occupied(position) => write!(
                formatter,
                "ground item position ({}, {}) is already occupied",
                position.x, position.y
            ),
            Self::IdSpaceExhausted => write!(formatter, "ground item ID space is exhausted"),
        }
    }
}

impl Error for GroundItemRegistryError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ground_items_have_stable_ids_and_one_stack_per_tile() {
        let mut items = GroundItemRegistry::default();
        let first = items
            .spawn(
                GridPos::new(2, 3),
                "core:repair_patch"
                    .parse()
                    .unwrap_or_else(|error| panic!("valid item ID rejected: {error}")),
                2,
            )
            .unwrap_or_else(|error| panic!("valid ground item rejected: {error}"));

        assert_eq!(first.get(), 1);
        assert_eq!(items.item_at(GridPos::new(2, 3)), Some(first));
        assert!(matches!(
            items.spawn(
                GridPos::new(2, 3),
                "core:other"
                    .parse()
                    .unwrap_or_else(|error| panic!("valid item ID rejected: {error}")),
                1,
            ),
            Err(GroundItemRegistryError::Occupied(_))
        ));
    }

    #[test]
    fn taking_a_partial_stack_preserves_one_total_quantity() {
        let mut items = GroundItemRegistry::default();
        let id = items
            .spawn(GridPos::new(2, 3), "core:part".parse().unwrap(), 3)
            .unwrap();

        let detached = items.take(id, 1).unwrap();

        assert_eq!(detached.item().as_str(), "core:part");
        assert_eq!(detached.quantity(), 1);
        assert_eq!(items.get(id).map(GroundItem::quantity), Some(2));
        assert_eq!(
            items.take(id, 3),
            Err(GroundItemTakeError::InsufficientQuantity {
                available: 2,
                requested: 3,
            })
        );
    }
}
