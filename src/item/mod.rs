use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::content::ContentId;

pub type ItemId = ContentId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemKind {
    Consumable,
    Material,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemEffect {
    RestoreIntegrity { amount: u16 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ItemDefinition {
    id: ItemId,
    name_key: String,
    description_key: String,
    maximum_stack: u16,
    kind: ItemKind,
    effects: Vec<ItemEffect>,
}

impl ItemDefinition {
    pub fn new(
        id: ItemId,
        name_key: String,
        description_key: String,
        maximum_stack: u16,
        kind: ItemKind,
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
            effects,
        })
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
    ZeroEffectAmount,
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
            Self::ZeroEffectAmount => write!(formatter, "item effect amount must be positive"),
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
}
