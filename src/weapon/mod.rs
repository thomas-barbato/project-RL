use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::combat::AttackProfile;
use crate::content::ContentId;

pub type WeaponId = ContentId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WeaponDefinition {
    id: WeaponId,
    name_key: String,
    description_key: String,
    attack: AttackProfile,
}

impl WeaponDefinition {
    pub fn new(
        id: WeaponId,
        name_key: String,
        description_key: String,
        attack: AttackProfile,
    ) -> Result<Self, WeaponDefinitionError> {
        if name_key.trim().is_empty() {
            return Err(WeaponDefinitionError::EmptyNameKey);
        }
        if description_key.trim().is_empty() {
            return Err(WeaponDefinitionError::EmptyDescriptionKey);
        }
        if attack.range() == 0 {
            return Err(WeaponDefinitionError::ZeroRange);
        }
        if attack.damage().amount == 0 {
            return Err(WeaponDefinitionError::ZeroDamage);
        }
        Ok(Self {
            id,
            name_key,
            description_key,
            attack,
        })
    }

    pub const fn id(&self) -> &WeaponId {
        &self.id
    }

    pub fn name_key(&self) -> &str {
        &self.name_key
    }

    pub fn description_key(&self) -> &str {
        &self.description_key
    }

    pub const fn attack(&self) -> AttackProfile {
        self.attack
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeaponDefinitionError {
    EmptyNameKey,
    EmptyDescriptionKey,
    ZeroRange,
    ZeroDamage,
}

impl Display for WeaponDefinitionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyNameKey => write!(formatter, "weapon name_key must not be empty"),
            Self::EmptyDescriptionKey => {
                write!(formatter, "weapon description_key must not be empty")
            }
            Self::ZeroRange => write!(formatter, "weapon attack range must be positive"),
            Self::ZeroDamage => write!(formatter, "weapon attack damage must be positive"),
        }
    }
}

impl Error for WeaponDefinitionError {}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WeaponCatalog {
    definitions: BTreeMap<WeaponId, WeaponDefinition>,
}

impl WeaponCatalog {
    pub fn register(&mut self, definition: WeaponDefinition) -> Result<(), WeaponCatalogError> {
        let id = definition.id().clone();
        if self.definitions.contains_key(&id) {
            return Err(WeaponCatalogError::DuplicateId(id));
        }
        self.definitions.insert(id, definition);
        Ok(())
    }

    pub fn get(&self, id: &WeaponId) -> Option<&WeaponDefinition> {
        self.definitions.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&WeaponId, &WeaponDefinition)> {
        self.definitions.iter()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WeaponCatalogError {
    DuplicateId(WeaponId),
}

impl Display for WeaponCatalogError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateId(id) => write!(formatter, "duplicate weapon ID '{id}'"),
        }
    }
}

impl Error for WeaponCatalogError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::DamageType;

    fn weapon(id: &str) -> WeaponDefinition {
        WeaponDefinition::new(
            id.parse()
                .unwrap_or_else(|error| panic!("valid weapon ID rejected: {error}")),
            "weapon.test.name".to_owned(),
            "weapon.test.description".to_owned(),
            AttackProfile::melee(DamageType::Kinetic, 3),
        )
        .unwrap_or_else(|error| panic!("valid weapon rejected: {error}"))
    }

    #[test]
    fn catalog_rejects_silent_weapon_overrides() {
        let mut catalog = WeaponCatalog::default();

        assert_eq!(catalog.register(weapon("core:test")), Ok(()));
        assert!(matches!(
            catalog.register(weapon("core:test")),
            Err(WeaponCatalogError::DuplicateId(_))
        ));
    }
}
