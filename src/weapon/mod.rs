use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::combat::{AttackProfile, ConeAttackError};
use crate::content::{ContentId, ContentIdError};
use crate::effects::{
    ApplyStatusEffect, ApplyStatusEffectError, GroundEffectSpec, GroundEffectSpecError,
};
use crate::status::StatusId;

pub type WeaponId = ContentId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WeaponEffect {
    ApplyStatus(ApplyStatusEffect),
    CreateGroundEffect(GroundEffectSpec),
}

#[derive(Clone, PartialEq, Eq)]
pub struct WeaponDefinition {
    id: WeaponId,
    name_key: String,
    description_key: String,
    attack: AttackProfile,
    effects: Vec<WeaponEffect>,
}

// Empty effects are intentionally omitted to preserve the exact historical
// Debug representation used by suspension rules fingerprints.
impl Debug for WeaponDefinition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut weapon = formatter.debug_struct("WeaponDefinition");
        weapon
            .field("id", &self.id)
            .field("name_key", &self.name_key)
            .field("description_key", &self.description_key)
            .field("attack", &self.attack);
        if !self.effects.is_empty() {
            weapon.field("effects", &self.effects);
        }
        weapon.finish()
    }
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
            effects: Vec::new(),
        })
    }

    pub fn with_effects(mut self, effects: impl IntoIterator<Item = WeaponEffect>) -> Self {
        self.effects.extend(effects);
        self
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

    pub fn effects(&self) -> &[WeaponEffect] {
        &self.effects
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WeaponDefinitionError {
    EmptyNameKey,
    EmptyDescriptionKey,
    ZeroRange,
    ZeroDamage,
    InvalidCone(ConeAttackError),
    InvalidEffectId(ContentIdError),
    InvalidStatusEffect(ApplyStatusEffectError),
    InvalidGroundEffect(GroundEffectSpecError),
    UnknownStatus(StatusId),
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
            Self::InvalidCone(error) => write!(formatter, "invalid cone attack: {error}"),
            Self::InvalidEffectId(error) => write!(formatter, "invalid weapon effect ID: {error}"),
            Self::InvalidStatusEffect(error) => {
                write!(formatter, "invalid weapon status effect: {error}")
            }
            Self::InvalidGroundEffect(error) => {
                write!(formatter, "invalid weapon ground effect: {error}")
            }
            Self::UnknownStatus(status) => {
                write!(formatter, "weapon references unknown status '{status}'")
            }
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

    pub fn without_id(&self, excluded: &WeaponId) -> Self {
        let mut definitions = self.definitions.clone();
        definitions.remove(excluded);
        Self { definitions }
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
