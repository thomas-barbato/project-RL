use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::content::ContentId;

pub type ItemId = ContentId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemKind {
    Armor,
    Consumable,
    Material,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EquipmentProfile {
    slot: ContentId,
    armor: u16,
}

impl EquipmentProfile {
    pub fn new(slot: ContentId, armor: u16) -> Result<Self, ItemDefinitionError> {
        if armor == 0 {
            return Err(ItemDefinitionError::ZeroArmor);
        }
        Ok(Self { slot, armor })
    }

    pub const fn slot(&self) -> &ContentId {
        &self.slot
    }

    pub const fn armor(&self) -> u16 {
        self.armor
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemEffect {
    RestoreIntegrity { amount: u16 },
}

#[derive(Clone, PartialEq, Eq)]
pub struct ItemDefinition {
    id: ItemId,
    name_key: String,
    description_key: String,
    maximum_stack: u16,
    kind: ItemKind,
    mass_grams: Option<u32>,
    equipment: Option<EquipmentProfile>,
    effects: Vec<ItemEffect>,
}

// Omit absent optional fields so replay fingerprints of older item definitions
// remain byte-for-byte identical to their historical Debug form.
impl Debug for ItemDefinition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut item = formatter.debug_struct("ItemDefinition");
        item.field("id", &self.id)
            .field("name_key", &self.name_key)
            .field("description_key", &self.description_key)
            .field("maximum_stack", &self.maximum_stack)
            .field("kind", &self.kind);
        if let Some(mass_grams) = self.mass_grams {
            item.field("mass_grams", &mass_grams);
        }
        if let Some(equipment) = &self.equipment {
            item.field("equipment", equipment);
        }
        item.field("effects", &self.effects).finish()
    }
}

impl ItemDefinition {
    pub fn new(
        id: ItemId,
        name_key: String,
        description_key: String,
        maximum_stack: u16,
        kind: ItemKind,
        equipment: Option<EquipmentProfile>,
        effects: Vec<ItemEffect>,
    ) -> Result<Self, ItemDefinitionError> {
        if name_key.trim().is_empty() {
            return Err(ItemDefinitionError::EmptyNameKey);
        }
        if description_key.trim().is_empty() {
            return Err(ItemDefinitionError::EmptyDescriptionKey);
        }
        if maximum_stack == 0 {
            return Err(ItemDefinitionError::ZeroMaximumStack);
        }
        if matches!(kind, ItemKind::Consumable) && effects.is_empty() {
            return Err(ItemDefinitionError::ConsumableWithoutEffects);
        }
        if matches!(kind, ItemKind::Armor) != equipment.is_some() {
            return Err(ItemDefinitionError::ArmorEquipmentMismatch);
        }
        if matches!(kind, ItemKind::Armor) && maximum_stack != 1 {
            return Err(ItemDefinitionError::ArmorMustNotStack);
        }
        if matches!(kind, ItemKind::Armor) && !effects.is_empty() {
            return Err(ItemDefinitionError::ArmorWithConsumableEffects);
        }
        if effects
            .iter()
            .any(|effect| matches!(effect, ItemEffect::RestoreIntegrity { amount: 0 }))
        {
            return Err(ItemDefinitionError::ZeroEffectAmount);
        }
        Ok(Self {
            id,
            name_key,
            description_key,
            maximum_stack,
            kind,
            mass_grams: None,
            equipment,
            effects,
        })
    }

    pub fn with_mass_grams(mut self, mass_grams: u32) -> Result<Self, ItemDefinitionError> {
        if mass_grams == 0 {
            return Err(ItemDefinitionError::ZeroMass);
        }
        self.mass_grams = Some(mass_grams);
        Ok(self)
    }

    pub const fn id(&self) -> &ItemId {
        &self.id
    }

    pub fn name_key(&self) -> &str {
        &self.name_key
    }

    pub fn description_key(&self) -> &str {
        &self.description_key
    }

    pub const fn maximum_stack(&self) -> u16 {
        self.maximum_stack
    }

    pub const fn kind(&self) -> ItemKind {
        self.kind
    }

    /// Mass of one unit. `None` keeps legacy definitions compatible and is
    /// treated as no declared contribution by load-sensitive rules.
    pub const fn mass_grams(&self) -> Option<u32> {
        self.mass_grams
    }

    pub const fn equipment(&self) -> Option<&EquipmentProfile> {
        self.equipment.as_ref()
    }

    pub fn effects(&self) -> &[ItemEffect] {
        &self.effects
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemDefinitionError {
    EmptyNameKey,
    EmptyDescriptionKey,
    ZeroMaximumStack,
    ConsumableWithoutEffects,
    ArmorEquipmentMismatch,
    ArmorMustNotStack,
    ArmorWithConsumableEffects,
    ZeroArmor,
    ZeroEffectAmount,
    ZeroMass,
}

impl Display for ItemDefinitionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyNameKey => write!(formatter, "item name_key must not be empty"),
            Self::EmptyDescriptionKey => {
                write!(formatter, "item description_key must not be empty")
            }
            Self::ZeroMaximumStack => write!(formatter, "item maximum_stack must be positive"),
            Self::ConsumableWithoutEffects => {
                write!(
                    formatter,
                    "a consumable item must define at least one effect"
                )
            }
            Self::ArmorEquipmentMismatch => {
                write!(formatter, "armor kind and equipment profile must match")
            }
            Self::ArmorMustNotStack => write!(formatter, "armor maximum_stack must be one"),
            Self::ArmorWithConsumableEffects => {
                write!(formatter, "armor cannot define consumable effects")
            }
            Self::ZeroArmor => write!(formatter, "equipment armor must be positive"),
            Self::ZeroEffectAmount => write!(formatter, "item effect amount must be positive"),
            Self::ZeroMass => write!(formatter, "item mass_grams must be positive when declared"),
        }
    }
}

impl Error for ItemDefinitionError {}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ItemCatalog {
    definitions: BTreeMap<ItemId, ItemDefinition>,
}

impl ItemCatalog {
    pub fn register(&mut self, definition: ItemDefinition) -> Result<(), ItemCatalogError> {
        let id = definition.id().clone();
        if self.definitions.contains_key(&id) {
            return Err(ItemCatalogError::DuplicateId(id));
        }
        self.definitions.insert(id, definition);
        Ok(())
    }

    pub fn get(&self, id: &ItemId) -> Option<&ItemDefinition> {
        self.definitions.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&ItemId, &ItemDefinition)> {
        self.definitions.iter()
    }

    pub fn without_kind(&self, excluded: ItemKind) -> Self {
        Self {
            definitions: self
                .definitions
                .iter()
                .filter(|(_, definition)| definition.kind() != excluded)
                .map(|(id, definition)| (id.clone(), definition.clone()))
                .collect(),
        }
    }

    pub fn without_id(&self, excluded: &ItemId) -> Self {
        let mut catalog = self.clone();
        catalog.definitions.remove(excluded);
        catalog
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ItemCatalogError {
    DuplicateId(ItemId),
}

impl Display for ItemCatalogError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateId(id) => write!(formatter, "duplicate item ID '{id}'"),
        }
    }
}

impl Error for ItemCatalogError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn repair_item(id: &str) -> ItemDefinition {
        ItemDefinition::new(
            id.parse()
                .unwrap_or_else(|error| panic!("valid item ID rejected: {error}")),
            "item.repair.name".to_owned(),
            "item.repair.description".to_owned(),
            3,
            ItemKind::Consumable,
            None,
            vec![ItemEffect::RestoreIntegrity { amount: 6 }],
        )
        .unwrap_or_else(|error| panic!("valid item rejected: {error}"))
    }

    #[test]
    fn consumables_require_a_real_effect() {
        let result = ItemDefinition::new(
            "core:empty_consumable"
                .parse()
                .unwrap_or_else(|error| panic!("valid item ID rejected: {error}")),
            "item.empty.name".to_owned(),
            "item.empty.description".to_owned(),
            1,
            ItemKind::Consumable,
            None,
            Vec::new(),
        );

        assert_eq!(result, Err(ItemDefinitionError::ConsumableWithoutEffects));
    }

    #[test]
    fn catalog_rejects_silent_item_overrides() {
        let mut catalog = ItemCatalog::default();

        assert_eq!(catalog.register(repair_item("core:repair_patch")), Ok(()));
        assert!(matches!(
            catalog.register(repair_item("core:repair_patch")),
            Err(ItemCatalogError::DuplicateId(_))
        ));
    }

    #[test]
    fn catalog_can_reproduce_the_pre_material_ruleset() {
        let mut catalog = ItemCatalog::default();
        catalog.register(repair_item("core:repair_patch")).unwrap();
        catalog
            .register(
                ItemDefinition::new(
                    "core:part".parse().unwrap(),
                    "item.part.name".to_owned(),
                    "item.part.description".to_owned(),
                    4,
                    ItemKind::Material,
                    None,
                    Vec::new(),
                )
                .unwrap(),
            )
            .unwrap();

        let legacy = catalog.without_kind(ItemKind::Material);

        assert!(legacy.get(&"core:repair_patch".parse().unwrap()).is_some());
        assert!(legacy.get(&"core:part".parse().unwrap()).is_none());
        assert_eq!(catalog.iter().count(), 2);
    }

    #[test]
    fn armor_requires_a_single_non_stackable_equipment_profile() {
        let slot: ContentId = "core:body_armor".parse().unwrap();
        let armor = ItemDefinition::new(
            "core:test_armor".parse().unwrap(),
            "item.test_armor.name".to_owned(),
            "item.test_armor.description".to_owned(),
            1,
            ItemKind::Armor,
            Some(EquipmentProfile::new(slot.clone(), 2).unwrap()),
            Vec::new(),
        )
        .unwrap();

        assert_eq!(armor.equipment().map(EquipmentProfile::slot), Some(&slot));
        assert_eq!(armor.equipment().map(EquipmentProfile::armor), Some(2));
        assert_eq!(
            EquipmentProfile::new(slot.clone(), 0),
            Err(ItemDefinitionError::ZeroArmor)
        );
        assert_eq!(
            ItemDefinition::new(
                "core:stacked_armor".parse().unwrap(),
                "item.stacked_armor.name".to_owned(),
                "item.stacked_armor.description".to_owned(),
                2,
                ItemKind::Armor,
                Some(EquipmentProfile::new(slot.clone(), 1).unwrap()),
                Vec::new(),
            ),
            Err(ItemDefinitionError::ArmorMustNotStack)
        );
        assert_eq!(
            ItemDefinition::new(
                "core:fake_armor".parse().unwrap(),
                "item.fake_armor.name".to_owned(),
                "item.fake_armor.description".to_owned(),
                1,
                ItemKind::Material,
                Some(EquipmentProfile::new(slot, 1).unwrap()),
                Vec::new(),
            ),
            Err(ItemDefinitionError::ArmorEquipmentMismatch)
        );
    }

    #[test]
    fn mass_is_per_unit_and_zero_is_rejected() {
        let definition = repair_item("core:weighted_patch")
            .with_mass_grams(750)
            .unwrap();

        assert_eq!(definition.mass_grams(), Some(750));
        assert_eq!(
            repair_item("core:massless_patch").with_mass_grams(0),
            Err(ItemDefinitionError::ZeroMass)
        );
    }
}
