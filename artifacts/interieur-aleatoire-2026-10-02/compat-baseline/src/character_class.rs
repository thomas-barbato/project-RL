use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::content::ContentId;
use crate::game::{GameRules, StartingItemStack};
use crate::item::{ItemCatalog, ItemId};
use crate::stats::{PrimaryAttributeRules, PrimaryAttributes};
use crate::weapon::{WeaponCatalog, WeaponId};

pub type CharacterClassId = ContentId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClassStartingItem {
    item: ItemId,
    quantity: u16,
}

impl ClassStartingItem {
    pub const fn new(item: ItemId, quantity: u16) -> Self {
        Self { item, quantity }
    }

    pub const fn item(&self) -> &ItemId {
        &self.item
    }

    pub const fn quantity(&self) -> u16 {
        self.quantity
    }
}

/// A moddable starting profile. It grants an initial configuration, never a
/// permanent permission or restriction: progression remains entirely separate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CharacterClassDefinition {
    id: CharacterClassId,
    name_key: String,
    role_key: String,
    description_key: String,
    recommended_attributes: PrimaryAttributes,
    starting_weapons: Vec<WeaponId>,
    starting_items: Vec<ClassStartingItem>,
    starting_equipment: Vec<Option<WeaponId>>,
}

impl CharacterClassDefinition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: CharacterClassId,
        name_key: String,
        role_key: String,
        description_key: String,
        recommended_attributes: PrimaryAttributes,
        starting_weapons: Vec<WeaponId>,
        starting_items: Vec<ClassStartingItem>,
        starting_equipment: Vec<Option<WeaponId>>,
    ) -> Result<Self, CharacterClassDefinitionError> {
        if name_key.trim().is_empty() {
            return Err(CharacterClassDefinitionError::EmptyNameKey);
        }
        if role_key.trim().is_empty() {
            return Err(CharacterClassDefinitionError::EmptyRoleKey);
        }
        if description_key.trim().is_empty() {
            return Err(CharacterClassDefinitionError::EmptyDescriptionKey);
        }
        recommended_attributes
            .validate_for_creation(PrimaryAttributeRules::default())
            .map_err(CharacterClassDefinitionError::InvalidRecommendedAttributes)?;

        let mut weapons = BTreeSet::new();
        for weapon in &starting_weapons {
            if !weapons.insert(weapon.clone()) {
                return Err(CharacterClassDefinitionError::DuplicateStartingWeapon(
                    weapon.clone(),
                ));
            }
        }
        let mut items = BTreeSet::new();
        for item in &starting_items {
            if item.quantity == 0 {
                return Err(CharacterClassDefinitionError::ZeroStartingItemQuantity(
                    item.item.clone(),
                ));
            }
            if !items.insert(item.item.clone()) {
                return Err(CharacterClassDefinitionError::DuplicateStartingItem(
                    item.item.clone(),
                ));
            }
        }
        for weapon in starting_equipment.iter().flatten() {
            if !weapons.contains(weapon) {
                return Err(CharacterClassDefinitionError::EquippedWeaponNotGranted(
                    weapon.clone(),
                ));
            }
        }

        Ok(Self {
            id,
            name_key,
            role_key,
            description_key,
            recommended_attributes,
            starting_weapons,
            starting_items,
            starting_equipment,
        })
    }

    pub const fn id(&self) -> &CharacterClassId {
        &self.id
    }

    pub fn name_key(&self) -> &str {
        &self.name_key
    }

    pub fn role_key(&self) -> &str {
        &self.role_key
    }

    pub fn description_key(&self) -> &str {
        &self.description_key
    }

    pub const fn recommended_attributes(&self) -> PrimaryAttributes {
        self.recommended_attributes
    }

    pub fn starting_weapons(&self) -> &[WeaponId] {
        &self.starting_weapons
    }

    pub fn starting_items(&self) -> &[ClassStartingItem] {
        &self.starting_items
    }

    pub fn starting_equipment(&self) -> &[Option<WeaponId>] {
        &self.starting_equipment
    }

    pub fn validate_references(
        &self,
        weapons: &WeaponCatalog,
        items: &ItemCatalog,
    ) -> Result<(), CharacterClassDefinitionError> {
        for weapon in &self.starting_weapons {
            if weapons.get(weapon).is_none() {
                return Err(CharacterClassDefinitionError::UnknownWeapon(weapon.clone()));
            }
        }
        for item in &self.starting_items {
            if items.get(&item.item).is_none() {
                return Err(CharacterClassDefinitionError::UnknownItem(
                    item.item.clone(),
                ));
            }
        }
        Ok(())
    }

    pub fn apply_to_rules(
        &self,
        rules: &mut GameRules,
        attributes: PrimaryAttributes,
    ) -> Result<(), CharacterClassApplicationError> {
        attributes
            .validate_for_creation(rules.primary_attribute_rules)
            .map_err(CharacterClassApplicationError::InvalidAttributes)?;
        self.validate_references(&rules.weapons, &rules.items)
            .map_err(CharacterClassApplicationError::InvalidDefinition)?;
        if self.starting_equipment.len() > rules.player_weapon_slots.len() {
            return Err(CharacterClassApplicationError::TooManyEquipmentChannels {
                actual: self.starting_equipment.len(),
                maximum: rules.player_weapon_slots.len(),
            });
        }

        rules.player_starting_attributes = attributes;
        rules.player_starting_weapons = self.starting_weapons.clone();
        rules.player_starting_items = self
            .starting_items
            .iter()
            .map(|entry| StartingItemStack::new(entry.item.clone(), entry.quantity))
            .collect();
        rules.player_starting_equipment = self.starting_equipment.clone();
        rules
            .player_starting_equipment
            .resize(rules.player_weapon_slots.len(), None);
        rules.player_base_attacks = self
            .starting_weapons
            .iter()
            .filter_map(|id| rules.weapons.get(id).map(|weapon| weapon.attack()))
            .collect();
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CharacterClassDefinitionError {
    EmptyNameKey,
    EmptyRoleKey,
    EmptyDescriptionKey,
    InvalidRecommendedAttributes(crate::stats::PrimaryAttributesError),
    DuplicateStartingWeapon(WeaponId),
    DuplicateStartingItem(ItemId),
    ZeroStartingItemQuantity(ItemId),
    EquippedWeaponNotGranted(WeaponId),
    UnknownWeapon(WeaponId),
    UnknownItem(ItemId),
}

impl Display for CharacterClassDefinitionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyNameKey => write!(formatter, "character class name_key must not be empty"),
            Self::EmptyRoleKey => write!(formatter, "character class role_key must not be empty"),
            Self::EmptyDescriptionKey => {
                write!(
                    formatter,
                    "character class description_key must not be empty"
                )
            }
            Self::InvalidRecommendedAttributes(error) => {
                write!(formatter, "invalid recommended attributes: {error}")
            }
            Self::DuplicateStartingWeapon(id) => {
                write!(formatter, "duplicate starting weapon '{id}'")
            }
            Self::DuplicateStartingItem(id) => write!(formatter, "duplicate starting item '{id}'"),
            Self::ZeroStartingItemQuantity(id) => {
                write!(formatter, "starting item '{id}' has zero quantity")
            }
            Self::EquippedWeaponNotGranted(id) => {
                write!(
                    formatter,
                    "equipped weapon '{id}' is not granted by this class"
                )
            }
            Self::UnknownWeapon(id) => write!(formatter, "unknown starting weapon '{id}'"),
            Self::UnknownItem(id) => write!(formatter, "unknown starting item '{id}'"),
        }
    }
}

impl Error for CharacterClassDefinitionError {}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CharacterClassCatalog {
    definitions: BTreeMap<CharacterClassId, CharacterClassDefinition>,
}

impl CharacterClassCatalog {
    pub fn register(
        &mut self,
        definition: CharacterClassDefinition,
    ) -> Result<(), CharacterClassCatalogError> {
        let id = definition.id.clone();
        if self.definitions.contains_key(&id) {
            return Err(CharacterClassCatalogError::DuplicateId(id));
        }
        self.definitions.insert(id, definition);
        Ok(())
    }

    pub fn get(&self, id: &CharacterClassId) -> Option<&CharacterClassDefinition> {
        self.definitions.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&CharacterClassId, &CharacterClassDefinition)> {
        self.definitions.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.definitions.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CharacterClassCatalogError {
    DuplicateId(CharacterClassId),
}

impl Display for CharacterClassCatalogError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateId(id) => write!(formatter, "duplicate character class ID '{id}'"),
        }
    }
}

impl Error for CharacterClassCatalogError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CharacterClassApplicationError {
    InvalidAttributes(crate::stats::PrimaryAttributesError),
    InvalidDefinition(CharacterClassDefinitionError),
    TooManyEquipmentChannels { actual: usize, maximum: usize },
}

impl Display for CharacterClassApplicationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidAttributes(error) => {
                write!(formatter, "invalid starting attributes: {error}")
            }
            Self::InvalidDefinition(error) => write!(formatter, "invalid character class: {error}"),
            Self::TooManyEquipmentChannels { actual, maximum } => write!(
                formatter,
                "character class defines {actual} equipment channels, but the rules expose only {maximum}"
            ),
        }
    }
}

impl Error for CharacterClassApplicationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::DamageType;
    use crate::item::{ItemDefinition, ItemEffect, ItemKind};
    use crate::weapon::WeaponDefinition;

    fn definition() -> CharacterClassDefinition {
        CharacterClassDefinition::new(
            "core:breach".parse().unwrap(),
            "class.breach.name".to_owned(),
            "class.breach.role".to_owned(),
            "class.breach.description".to_owned(),
            PrimaryAttributes::new(8, 5, 7, 4, 4),
            vec!["core:blade".parse().unwrap()],
            vec![ClassStartingItem::new("core:repair".parse().unwrap(), 2)],
            vec![Some("core:blade".parse().unwrap())],
        )
        .unwrap()
    }

    #[test]
    fn starting_profile_is_validated_without_locking_progression() {
        let mut rules = GameRules {
            player_weapon_slots: vec!["core:channel".parse().unwrap()],
            ..GameRules::default()
        };
        rules
            .weapons
            .register(
                WeaponDefinition::new(
                    "core:blade".parse().unwrap(),
                    "weapon.blade.name".to_owned(),
                    "weapon.blade.description".to_owned(),
                    crate::combat::AttackProfile::melee(DamageType::Kinetic, 3),
                )
                .unwrap(),
            )
            .unwrap();
        rules
            .items
            .register(
                ItemDefinition::new(
                    "core:repair".parse().unwrap(),
                    "item.repair.name".to_owned(),
                    "item.repair.description".to_owned(),
                    3,
                    ItemKind::Consumable,
                    None,
                    vec![ItemEffect::RestoreIntegrity { amount: 2 }],
                )
                .unwrap(),
            )
            .unwrap();

        let class = definition();
        class
            .apply_to_rules(&mut rules, PrimaryAttributes::new(7, 6, 7, 4, 4))
            .unwrap();

        assert_eq!(
            rules.player_starting_attributes,
            PrimaryAttributes::new(7, 6, 7, 4, 4)
        );
        assert_eq!(rules.player_starting_weapons, class.starting_weapons());
        assert_eq!(rules.player_starting_items[0].quantity, 2);
    }

    #[test]
    fn equipped_weapons_must_belong_to_the_starting_loadout() {
        let result = CharacterClassDefinition::new(
            "core:test".parse().unwrap(),
            "name".to_owned(),
            "role".to_owned(),
            "description".to_owned(),
            PrimaryAttributes::prototype_default(),
            Vec::new(),
            Vec::new(),
            vec![Some("core:blade".parse().unwrap())],
        );

        assert!(matches!(
            result,
            Err(CharacterClassDefinitionError::EquippedWeaponNotGranted(_))
        ));
    }
}
