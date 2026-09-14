use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::combat::{AttackImpactError, AttackProfile, ConeAttackError, DamageImpactError};
use crate::content::{ContentId, ContentIdError};
use crate::effects::{
    ApplyStatusEffect, ApplyStatusEffectError, GroundEffectSpec, GroundEffectSpecError,
};
use crate::status::StatusId;
use crate::time::TimeUnitsError;

pub type WeaponId = ContentId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WeaponEffectKind {
    ApplyStatus(ApplyStatusEffect),
    CreateGroundEffect(GroundEffectSpec),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeaponEffectTrigger {
    OnAttack,
    OnHit,
    OnDamage,
    OnTargetDestroyed,
}

#[derive(Clone, PartialEq, Eq)]
pub struct WeaponEffect {
    trigger: Option<WeaponEffectTrigger>,
    kind: WeaponEffectKind,
}

// Effects loaded from versions 10-37 retain the exact historical Debug
// representation used by suspension rules fingerprints. New explicit effects
// include their trigger in the fingerprint.
impl Debug for WeaponEffect {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let Some(trigger) = self.trigger else {
            return match &self.kind {
                WeaponEffectKind::ApplyStatus(effect) => {
                    formatter.debug_tuple("ApplyStatus").field(effect).finish()
                }
                WeaponEffectKind::CreateGroundEffect(effect) => formatter
                    .debug_tuple("CreateGroundEffect")
                    .field(effect)
                    .finish(),
            };
        };
        formatter
            .debug_struct("WeaponEffect")
            .field("trigger", &trigger)
            .field("kind", &self.kind)
            .finish()
    }
}

impl WeaponEffect {
    pub fn apply_status(
        effect: ApplyStatusEffect,
        trigger: WeaponEffectTrigger,
    ) -> Result<Self, WeaponEffectError> {
        if matches!(
            trigger,
            WeaponEffectTrigger::OnAttack | WeaponEffectTrigger::OnTargetDestroyed
        ) {
            return Err(WeaponEffectError::InvalidStatusTrigger(trigger));
        }
        Ok(Self {
            trigger: Some(trigger),
            kind: WeaponEffectKind::ApplyStatus(effect),
        })
    }

    pub const fn create_ground_effect(
        effect: GroundEffectSpec,
        trigger: WeaponEffectTrigger,
    ) -> Self {
        Self {
            trigger: Some(trigger),
            kind: WeaponEffectKind::CreateGroundEffect(effect),
        }
    }

    pub const fn trigger(&self) -> Option<WeaponEffectTrigger> {
        self.trigger
    }

    pub const fn kind(&self) -> &WeaponEffectKind {
        &self.kind
    }

    pub(crate) const fn legacy_apply_status(effect: ApplyStatusEffect) -> Self {
        Self {
            trigger: None,
            kind: WeaponEffectKind::ApplyStatus(effect),
        }
    }

    pub(crate) const fn legacy_create_ground_effect(effect: GroundEffectSpec) -> Self {
        Self {
            trigger: None,
            kind: WeaponEffectKind::CreateGroundEffect(effect),
        }
    }

    const fn remove_explicit_trigger(&mut self) {
        self.trigger = None;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeaponEffectError {
    InvalidStatusTrigger(WeaponEffectTrigger),
}

impl Display for WeaponEffectError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidStatusTrigger(trigger) => write!(
                formatter,
                "status effects require an on_hit or on_damage trigger, found {trigger:?}"
            ),
        }
    }
}

impl Error for WeaponEffectError {}

/// Explicit interaction hooks offered by a weapon independently from its
/// damage profile. Mods can opt into these capabilities without the engine
/// guessing them from an item name or a damage family.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct WeaponCapabilities {
    melee_parry: bool,
    automatic_fire: bool,
}

impl WeaponCapabilities {
    pub const fn new() -> Self {
        Self {
            melee_parry: false,
            automatic_fire: false,
        }
    }

    pub const fn with_melee_parry(mut self) -> Self {
        self.melee_parry = true;
        self
    }

    pub const fn can_melee_parry(self) -> bool {
        self.melee_parry
    }

    pub const fn with_automatic_fire(mut self) -> Self {
        self.automatic_fire = true;
        self
    }

    pub const fn can_automatic_fire(self) -> bool {
        self.automatic_fire
    }

    pub(crate) const fn without_ranged_skill_metadata(mut self) -> Self {
        self.automatic_fire = false;
        self
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct WeaponDefinition {
    id: WeaponId,
    name_key: String,
    description_key: String,
    mass_grams: Option<u32>,
    attack: AttackProfile,
    effects: Vec<WeaponEffect>,
    capabilities: WeaponCapabilities,
    ammunition_capacity: Option<u16>,
    power_draw: Option<u16>,
}

// Empty effects are intentionally omitted to preserve the exact historical
// Debug representation used by suspension rules fingerprints.
impl Debug for WeaponDefinition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut weapon = formatter.debug_struct("WeaponDefinition");
        weapon
            .field("id", &self.id)
            .field("name_key", &self.name_key)
            .field("description_key", &self.description_key);
        if let Some(mass_grams) = self.mass_grams {
            weapon.field("mass_grams", &mass_grams);
        }
        weapon.field("attack", &self.attack);
        if !self.effects.is_empty() {
            weapon.field("effects", &self.effects);
        }
        if self.capabilities != WeaponCapabilities::default() {
            weapon.field("capabilities", &self.capabilities);
        }
        if let Some(capacity) = self.ammunition_capacity {
            weapon.field("ammunition_capacity", &capacity);
        }
        if let Some(power_draw) = self.power_draw {
            weapon.field("power_draw", &power_draw);
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
        if attack.damage().raw_total() == 0 {
            return Err(WeaponDefinitionError::ZeroDamage);
        }
        Ok(Self {
            id,
            name_key,
            description_key,
            mass_grams: None,
            attack,
            effects: Vec::new(),
            capabilities: WeaponCapabilities::default(),
            ammunition_capacity: None,
            power_draw: None,
        })
    }

    pub fn with_effects(mut self, effects: impl IntoIterator<Item = WeaponEffect>) -> Self {
        self.effects.extend(effects);
        self
    }

    pub const fn with_capabilities(mut self, capabilities: WeaponCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Finite native projectile reserve used by every ordinary resolution of
    /// this weapon. Omission deliberately keeps legacy and energy-only weapons
    /// unlimited.
    pub fn with_ammunition_capacity(
        mut self,
        capacity: u16,
    ) -> Result<Self, WeaponDefinitionError> {
        if capacity == 0 {
            return Err(WeaponDefinitionError::ZeroAmmunitionCapacity);
        }
        if self.attack.delivery() != crate::combat::AttackDelivery::Ranged {
            return Err(WeaponDefinitionError::AmmunitionOnMeleeWeapon);
        }
        self.ammunition_capacity = Some(capacity);
        Ok(self)
    }

    pub fn with_mass_grams(mut self, mass_grams: u32) -> Result<Self, WeaponDefinitionError> {
        if mass_grams == 0 {
            return Err(WeaponDefinitionError::ZeroMass);
        }
        self.mass_grams = Some(mass_grams);
        Ok(self)
    }

    /// Native energy consumed by one use of this powered module. Omission
    /// deliberately marks purely mechanical weapons as incompatible with
    /// engineering output tuning and overclocking.
    pub fn with_power_draw(mut self, power_draw: u16) -> Result<Self, WeaponDefinitionError> {
        if power_draw == 0 {
            return Err(WeaponDefinitionError::ZeroPowerDraw);
        }
        self.power_draw = Some(power_draw);
        Ok(self)
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

    /// Mass of one weapon instance. An omitted value preserves old content
    /// and contributes no declared mass to load-sensitive rules.
    pub const fn mass_grams(&self) -> Option<u32> {
        self.mass_grams
    }

    pub const fn attack(&self) -> AttackProfile {
        self.attack
    }

    pub fn effects(&self) -> &[WeaponEffect] {
        &self.effects
    }

    pub const fn capabilities(&self) -> WeaponCapabilities {
        self.capabilities
    }

    pub const fn ammunition_capacity(&self) -> Option<u16> {
        self.ammunition_capacity
    }

    pub const fn power_draw(&self) -> Option<u16> {
        self.power_draw
    }

    pub(crate) fn remove_physical_metadata(&mut self) {
        self.attack = self.attack.without_melee_impact();
    }

    pub(crate) fn remove_effect_triggers(&mut self) {
        for effect in &mut self.effects {
            effect.remove_explicit_trigger();
        }
    }

    pub(crate) fn remove_reaction_metadata(&mut self) {
        self.capabilities = WeaponCapabilities::default();
    }

    pub(crate) fn remove_recovery_metadata(&mut self) {
        self.attack = self.attack.without_recovery_after_attack();
    }

    pub(crate) fn remove_ranged_skill_metadata(&mut self) {
        self.capabilities = self.capabilities.without_ranged_skill_metadata();
        self.ammunition_capacity = None;
    }

    pub(crate) fn remove_engineering_metadata(&mut self) {
        self.power_draw = None;
    }

    pub(crate) fn remove_preparation_disruption_metadata(&mut self) {
        self.attack = self.attack.without_preparation_disruption();
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WeaponDefinitionError {
    EmptyNameKey,
    EmptyDescriptionKey,
    ZeroRange,
    ZeroDamage,
    ZeroMass,
    ZeroAmmunitionCapacity,
    ZeroPowerDraw,
    AmmunitionOnMeleeWeapon,
    InvalidDamage(DamageImpactError),
    InvalidCone(ConeAttackError),
    InvalidImpact(AttackImpactError),
    InvalidRecovery(TimeUnitsError),
    InvalidEffectId(ContentIdError),
    InvalidStatusEffect(ApplyStatusEffectError),
    InvalidGroundEffect(GroundEffectSpecError),
    InvalidEffectTrigger(WeaponEffectError),
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
            Self::ZeroMass => {
                write!(
                    formatter,
                    "weapon mass_grams must be positive when declared"
                )
            }
            Self::ZeroAmmunitionCapacity => {
                write!(formatter, "weapon ammunition_capacity must be positive")
            }
            Self::ZeroPowerDraw => write!(formatter, "weapon power_draw must be positive"),
            Self::AmmunitionOnMeleeWeapon => {
                write!(formatter, "only a ranged weapon can declare ammunition")
            }
            Self::InvalidDamage(error) => write!(formatter, "invalid weapon damage: {error}"),
            Self::InvalidCone(error) => write!(formatter, "invalid cone attack: {error}"),
            Self::InvalidImpact(error) => write!(formatter, "invalid attack impact: {error}"),
            Self::InvalidRecovery(error) => write!(formatter, "invalid attack recovery: {error}"),
            Self::InvalidEffectId(error) => write!(formatter, "invalid weapon effect ID: {error}"),
            Self::InvalidStatusEffect(error) => {
                write!(formatter, "invalid weapon status effect: {error}")
            }
            Self::InvalidGroundEffect(error) => {
                write!(formatter, "invalid weapon ground effect: {error}")
            }
            Self::InvalidEffectTrigger(error) => {
                write!(formatter, "invalid weapon effect trigger: {error}")
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

    pub fn without_physical_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            definition.remove_physical_metadata();
        }
        catalog
    }

    pub fn without_effect_triggers(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            definition.remove_effect_triggers();
        }
        catalog
    }

    pub fn without_reaction_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            definition.remove_reaction_metadata();
        }
        catalog
    }

    pub fn without_recovery_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            definition.remove_recovery_metadata();
        }
        catalog
    }

    pub fn without_ranged_skill_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            definition.remove_ranged_skill_metadata();
        }
        catalog
    }

    pub fn without_engineering_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            definition.remove_engineering_metadata();
        }
        catalog
    }

    pub fn without_preparation_disruption_metadata(&self) -> Self {
        let mut catalog = self.clone();
        for definition in catalog.definitions.values_mut() {
            definition.remove_preparation_disruption_metadata();
        }
        catalog
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
    use crate::effects::ApplyStatusEffect;

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

    #[test]
    fn optional_weapon_mass_rejects_zero() {
        assert_eq!(
            weapon("core:weighted")
                .with_mass_grams(3_500)
                .unwrap()
                .mass_grams(),
            Some(3_500)
        );
        assert_eq!(
            weapon("core:massless").with_mass_grams(0),
            Err(WeaponDefinitionError::ZeroMass)
        );
    }

    #[test]
    fn status_effects_reject_triggers_without_a_live_target_outcome() {
        let status = ApplyStatusEffect::new("core:test".parse().unwrap(), 1).unwrap();

        assert_eq!(
            WeaponEffect::apply_status(status, WeaponEffectTrigger::OnAttack),
            Err(WeaponEffectError::InvalidStatusTrigger(
                WeaponEffectTrigger::OnAttack
            ))
        );
    }

    #[test]
    fn legacy_effect_debug_representation_does_not_include_a_trigger() {
        let effect = WeaponEffect::legacy_apply_status(
            ApplyStatusEffect::new("core:test".parse().unwrap(), 1).unwrap(),
        );

        assert!(format!("{effect:?}").starts_with("ApplyStatus("));
        assert!(!format!("{effect:?}").contains("trigger"));
    }

    #[test]
    fn weapon_capabilities_are_explicit_and_omitted_while_empty() {
        let ordinary = weapon("core:ordinary");
        let parrying =
            weapon("core:parrying").with_capabilities(WeaponCapabilities::new().with_melee_parry());

        assert!(!ordinary.capabilities().can_melee_parry());
        assert!(!format!("{ordinary:?}").contains("capabilities"));
        assert!(parrying.capabilities().can_melee_parry());
        assert!(format!("{parrying:?}").contains("melee_parry"));
    }

    #[test]
    fn finite_ammunition_is_explicit_positive_and_ranged_only() {
        let ranged = WeaponDefinition::new(
            "core:ranged".parse().unwrap(),
            "weapon.ranged.name".to_owned(),
            "weapon.ranged.description".to_owned(),
            AttackProfile::new(
                5,
                crate::world::DistanceMetric::Chebyshev,
                true,
                DamageType::Piercing,
                2,
                0,
            ),
        )
        .unwrap();
        assert_eq!(
            ranged
                .clone()
                .with_ammunition_capacity(12)
                .unwrap()
                .ammunition_capacity(),
            Some(12)
        );
        assert_eq!(
            ranged.with_ammunition_capacity(0),
            Err(WeaponDefinitionError::ZeroAmmunitionCapacity)
        );
        assert_eq!(
            weapon("core:melee_ammo").with_ammunition_capacity(1),
            Err(WeaponDefinitionError::AmmunitionOnMeleeWeapon)
        );
    }
}
