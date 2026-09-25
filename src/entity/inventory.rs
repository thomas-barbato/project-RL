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
#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(try_from = "StoredMagicItemModifiers")]
pub struct MagicItemModifiers {
    armor_bonus: u16,
    mass_reduction_percent: u8,
    #[serde(default)]
    attribute_bonuses: [u8; 5],
    #[serde(default)]
    accuracy_bonus: u16,
    #[serde(default)]
    armor_penetration_bonus: u16,
    #[serde(default)]
    maximum_hit_points_bonus: u16,
    #[serde(default)]
    energy_capacity_bonus: u16,
    #[serde(default)]
    heat_dissipation_bonus: u16,
    #[serde(default)]
    named_affixes: Option<crate::item::NamedEquipmentAffixes>,
    #[serde(default)]
    effect_affix: Option<crate::content::ContentId>,
}

// Validate the redundant numeric cache against its rolled identities on load.
// Anonymous historical bonuses keep their old values and Debug representation.
#[derive(serde::Deserialize)]
struct StoredMagicItemModifiers {
    armor_bonus: u16,
    mass_reduction_percent: u8,
    #[serde(default)]
    attribute_bonuses: [u8; 5],
    #[serde(default)]
    accuracy_bonus: u16,
    #[serde(default)]
    armor_penetration_bonus: u16,
    #[serde(default)]
    maximum_hit_points_bonus: u16,
    #[serde(default)]
    energy_capacity_bonus: u16,
    #[serde(default)]
    heat_dissipation_bonus: u16,
    #[serde(default)]
    named_affixes: Option<crate::item::NamedEquipmentAffixes>,
    #[serde(default)]
    effect_affix: Option<crate::content::ContentId>,
}

impl TryFrom<StoredMagicItemModifiers> for MagicItemModifiers {
    type Error = &'static str;
    fn try_from(raw: StoredMagicItemModifiers) -> Result<Self, Self::Error> {
        let result = Self {
            armor_bonus: raw.armor_bonus,
            mass_reduction_percent: raw.mass_reduction_percent,
            attribute_bonuses: raw.attribute_bonuses,
            accuracy_bonus: raw.accuracy_bonus,
            armor_penetration_bonus: raw.armor_penetration_bonus,
            maximum_hit_points_bonus: raw.maximum_hit_points_bonus,
            energy_capacity_bonus: raw.energy_capacity_bonus,
            heat_dissipation_bonus: raw.heat_dissipation_bonus,
            named_affixes: raw.named_affixes,
            effect_affix: raw.effect_affix,
        };
        if let Some(affixes) = result.named_affixes
            && result
                != Self::from_affixes(affixes)
                    .with_optional_effect_affix(result.effect_affix.clone())
        {
            return Err("equipment bonuses disagree with their named affixes");
        }
        Ok(result)
    }
}

// Keep fingerprints of old magical armor and mass-only instances unchanged.
impl Debug for MagicItemModifiers {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("MagicItemModifiers");
        d.field("armor_bonus", &self.armor_bonus)
            .field("mass_reduction_percent", &self.mass_reduction_percent);
        if self.attribute_bonuses != [0; 5] {
            d.field("attribute_bonuses", &self.attribute_bonuses);
        }
        if self.accuracy_bonus != 0 {
            d.field("accuracy_bonus", &self.accuracy_bonus);
        }
        if self.armor_penetration_bonus != 0 {
            d.field("armor_penetration_bonus", &self.armor_penetration_bonus);
        }
        if self.maximum_hit_points_bonus != 0 {
            d.field("maximum_hit_points_bonus", &self.maximum_hit_points_bonus);
        }
        if self.energy_capacity_bonus != 0 {
            d.field("energy_capacity_bonus", &self.energy_capacity_bonus);
        }
        if self.heat_dissipation_bonus != 0 {
            d.field("heat_dissipation_bonus", &self.heat_dissipation_bonus);
        }
        if let Some(affixes) = self.named_affixes {
            d.field("named_affixes", &affixes);
        }
        if let Some(effect) = &self.effect_affix {
            d.field("effect_affix", effect);
        }
        d.finish()
    }
}

impl MagicItemModifiers {
    /// A lighter weapon need not also grant armor or change its base profile.
    pub const fn mass_reduction_only(percent: u8) -> Option<Self> {
        if percent == 0 || percent > 80 {
            return None;
        }
        Some(Self {
            armor_bonus: 0,
            mass_reduction_percent: percent,
            attribute_bonuses: [0; 5],
            accuracy_bonus: 0,
            armor_penetration_bonus: 0,
            maximum_hit_points_bonus: 0,
            energy_capacity_bonus: 0,
            heat_dissipation_bonus: 0,
            named_affixes: None,
            effect_affix: None,
        })
    }

    pub const fn new(armor_bonus: u16, mass_reduction_percent: u8) -> Option<Self> {
        if armor_bonus == 0 || mass_reduction_percent == 0 || mass_reduction_percent > 80 {
            return None;
        }
        Some(Self {
            armor_bonus,
            mass_reduction_percent,
            attribute_bonuses: [0; 5],
            accuracy_bonus: 0,
            armor_penetration_bonus: 0,
            maximum_hit_points_bonus: 0,
            energy_capacity_bonus: 0,
            heat_dissipation_bonus: 0,
            named_affixes: None,
            effect_affix: None,
        })
    }

    pub const fn armor_bonus(&self) -> u16 {
        self.armor_bonus
    }

    pub fn weapon_bonuses(
        attribute_bonuses: [u8; 5],
        accuracy_bonus: u16,
        armor_penetration_bonus: u16,
    ) -> Option<Self> {
        Self::rpg_bonuses(
            attribute_bonuses,
            accuracy_bonus,
            armor_penetration_bonus,
            0,
            0,
            0,
        )
    }

    pub fn rpg_bonuses(
        attribute_bonuses: [u8; 5],
        accuracy_bonus: u16,
        armor_penetration_bonus: u16,
        maximum_hit_points_bonus: u16,
        energy_capacity_bonus: u16,
        heat_dissipation_bonus: u16,
    ) -> Option<Self> {
        if attribute_bonuses == [0; 5]
            && accuracy_bonus == 0
            && armor_penetration_bonus == 0
            && maximum_hit_points_bonus == 0
            && energy_capacity_bonus == 0
            && heat_dissipation_bonus == 0
        {
            return None;
        }
        Some(Self {
            armor_bonus: 0,
            mass_reduction_percent: 0,
            attribute_bonuses,
            accuracy_bonus,
            armor_penetration_bonus,
            maximum_hit_points_bonus,
            energy_capacity_bonus,
            heat_dissipation_bonus,
            named_affixes: None,
            effect_affix: None,
        })
    }

    pub fn from_affixes(affixes: crate::item::NamedEquipmentAffixes) -> Self {
        use crate::item::EquipmentAffixId as Id;
        let mut attributes = [0; 5];
        let (mut accuracy, mut penetration, mut hp, mut energy, mut cooling) = (0, 0, 0, 0, 0);
        for roll in affixes.iter() {
            match roll.id() {
                Id::Power => attributes[0] = roll.value() as u8,
                Id::Coordination => attributes[1] = roll.value() as u8,
                Id::Resilience => attributes[2] = roll.value() as u8,
                Id::Perception => attributes[3] = roll.value() as u8,
                Id::Processing => attributes[4] = roll.value() as u8,
                Id::Accuracy => accuracy = roll.value(),
                Id::ArmorPenetration => penetration = roll.value(),
                Id::Vitality => hp = roll.value(),
                Id::EnergyReserve => energy = roll.value(),
                Id::HeatDissipation => cooling = roll.value(),
            }
        }
        let mut result = Self::rpg_bonuses(attributes, accuracy, penetration, hp, energy, cooling)
            .expect("validated affixes carry at least one positive numeric bonus");
        result.named_affixes = Some(affixes);
        result
    }

    /// An effect-only item has no fictitious numerical bonus.
    pub fn effect_only(effect: crate::content::ContentId) -> Self {
        Self {
            armor_bonus: 0,
            mass_reduction_percent: 0,
            attribute_bonuses: [0; 5],
            accuracy_bonus: 0,
            armor_penetration_bonus: 0,
            maximum_hit_points_bonus: 0,
            energy_capacity_bonus: 0,
            heat_dissipation_bonus: 0,
            named_affixes: None,
            effect_affix: Some(effect),
        }
    }

    pub fn with_effect_affix(self, effect: crate::content::ContentId) -> Self {
        self.with_optional_effect_affix(Some(effect))
    }

    fn with_optional_effect_affix(mut self, effect: Option<crate::content::ContentId>) -> Self {
        self.effect_affix = effect;
        self
    }

    pub fn effect_affix(&self) -> Option<&crate::content::ContentId> {
        self.effect_affix.as_ref()
    }

    pub const fn named_affixes(&self) -> Option<crate::item::NamedEquipmentAffixes> {
        self.named_affixes
    }
    pub const fn attribute_bonus(&self, attribute: crate::stats::PrimaryAttribute) -> u8 {
        self.attribute_bonuses[attribute as usize]
    }
    pub const fn accuracy_bonus(&self) -> u16 {
        self.accuracy_bonus
    }
    pub const fn armor_penetration_bonus(&self) -> u16 {
        self.armor_penetration_bonus
    }
    pub const fn maximum_hit_points_bonus(&self) -> u16 {
        self.maximum_hit_points_bonus
    }
    pub const fn energy_capacity_bonus(&self) -> u16 {
        self.energy_capacity_bonus
    }
    pub const fn heat_dissipation_bonus(&self) -> u16 {
        self.heat_dissipation_bonus
    }

    /// Modifies a resolved copy, never the catalog's intrinsic attack.
    pub fn modify_weapon_attack(
        &self,
        attack: crate::combat::AttackProfile,
    ) -> crate::combat::AttackProfile {
        attack
            .with_accuracy_modifier(
                attack
                    .accuracy_modifier()
                    .saturating_add(i16::try_from(self.accuracy_bonus).unwrap_or(i16::MAX)),
            )
            .with_damage(
                attack
                    .damage()
                    .with_additional_armor_penetration(self.armor_penetration_bonus),
            )
    }

    pub const fn mass_reduction_percent(&self) -> u8 {
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
        if let Some(modifiers) = &self.magic_modifiers {
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

    pub fn magic_modifiers(&self) -> Option<MagicItemModifiers> {
        self.magic_modifiers.clone()
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

    pub(crate) fn attach_effect_affix(
        &mut self,
        instance: ItemInstanceId,
        effect: crate::content::ContentId,
    ) -> Result<(), InventoryError> {
        let entry = self
            .entries
            .iter_mut()
            .find(|entry| entry.instance == instance)
            .ok_or(InventoryError::UnknownInstance(instance))?;
        entry.magic_modifiers = Some(match entry.magic_modifiers.take() {
            Some(bonus) => bonus.with_effect_affix(effect),
            None => MagicItemModifiers::effect_only(effect),
        });
        Ok(())
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

    #[test]
    fn named_affix_values_drive_real_bonuses_and_cannot_disagree_on_load() {
        use crate::item::{
            EquipmentAffixId as Id, EquipmentNameGrammar as Grammar, NamedEquipmentAffixes,
            RolledEquipmentAffix as Roll,
        };
        for id in Id::ALL {
            let roll = Roll::new(id, 6, id.range(6).unwrap().1).unwrap();
            let affixes = NamedEquipmentAffixes::new(Grammar::FeminineSingular, &[roll]).unwrap();
            let bonus = MagicItemModifiers::from_affixes(affixes);
            assert_eq!(bonus.named_affixes(), Some(affixes));
            assert_eq!(
                bincode::deserialize::<MagicItemModifiers>(&bincode::serialize(&bonus).unwrap())
                    .unwrap(),
                bonus
            );
            assert_eq!(
                serde_json::from_str::<MagicItemModifiers>(&serde_json::to_string(&bonus).unwrap())
                    .unwrap(),
                bonus
            );
            let actual = match id {
                Id::Power => {
                    u16::from(bonus.attribute_bonus(crate::stats::PrimaryAttribute::Power))
                }
                Id::Coordination => {
                    u16::from(bonus.attribute_bonus(crate::stats::PrimaryAttribute::Coordination))
                }
                Id::Resilience => {
                    u16::from(bonus.attribute_bonus(crate::stats::PrimaryAttribute::Resilience))
                }
                Id::Perception => {
                    u16::from(bonus.attribute_bonus(crate::stats::PrimaryAttribute::Perception))
                }
                Id::Processing => {
                    u16::from(bonus.attribute_bonus(crate::stats::PrimaryAttribute::Processing))
                }
                Id::Accuracy => bonus.accuracy_bonus(),
                Id::ArmorPenetration => bonus.armor_penetration_bonus(),
                Id::Vitality => bonus.maximum_hit_points_bonus(),
                Id::EnergyReserve => bonus.energy_capacity_bonus(),
                Id::HeatDissipation => bonus.heat_dissipation_bonus(),
            };
            assert_eq!(actual, roll.value());
            let mut invalid = serde_json::to_value(bonus).unwrap();
            invalid["armor_bonus"] = 1.into();
            assert!(serde_json::from_value::<MagicItemModifiers>(invalid).is_err());
        }
    }

    #[test]
    fn old_anonymous_stat_bonuses_are_not_renamed_or_rerolled() {
        let old: MagicItemModifiers = serde_json::from_str(
            r#"{
            "armor_bonus":0,"mass_reduction_percent":0,"attribute_bonuses":[2,0,0,0,0],
            "accuracy_bonus":12,"maximum_hit_points_bonus":15
        }"#,
        )
        .unwrap();
        assert_eq!(old.named_affixes(), None);
        assert_eq!(old.accuracy_bonus(), 12);
        assert_eq!(
            format!("{old:?}"),
            "MagicItemModifiers { armor_bonus: 0, mass_reduction_percent: 0, attribute_bonuses: [2, 0, 0, 0, 0], accuracy_bonus: 12, maximum_hit_points_bonus: 15 }"
        );
    }

    #[test]
    fn mass_only_bonus_has_no_armor_and_keeps_the_existing_bounds() {
        assert!(MagicItemModifiers::mass_reduction_only(0).is_none());
        assert!(MagicItemModifiers::mass_reduction_only(81).is_none());
        for percent in [1, 25, 80] {
            let bonus = MagicItemModifiers::mass_reduction_only(percent).unwrap();
            assert_eq!(bonus.armor_bonus(), 0);
            assert_eq!(bonus.mass_reduction_percent(), percent);
        }
    }

    #[test]
    fn rpg_bonuses_round_trip_and_legacy_modifiers_keep_their_fingerprint() {
        let legacy: MagicItemModifiers =
            serde_json::from_str(r#"{"armor_bonus":3,"mass_reduction_percent":20}"#).unwrap();
        assert_eq!(
            format!("{legacy:?}"),
            "MagicItemModifiers { armor_bonus: 3, mass_reduction_percent: 20 }"
        );
        assert_eq!(legacy, MagicItemModifiers::new(3, 20).unwrap());
        let bonus = MagicItemModifiers::rpg_bonuses([2, 1, 0, 0, 0], 12, 3, 10, 15, 2).unwrap();
        assert_eq!(
            serde_json::from_str::<MagicItemModifiers>(&serde_json::to_string(&bonus).unwrap())
                .unwrap(),
            bonus
        );
        assert!(MagicItemModifiers::weapon_bonuses([0; 5], 0, 0).is_none());
    }

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
