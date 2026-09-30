use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::combat::DamagePacket;
pub use crate::content::{ContentId as StatusId, ContentIdError as StatusIdError};
pub type StatusFamilyId = crate::content::ContentId;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatusTrigger {
    TurnStart,
    TurnEnd,
    DamageReceived,
    DamageDealt,
    Movement,
    Death,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusStacking {
    /// Keeps the first application unchanged until it expires. This is used by
    /// bounded effects which explicitly forbid duration refreshes.
    KeepExisting,
    Replace,
    RefreshDuration,
    AddStacks {
        maximum_stacks: u16,
        refresh_duration: bool,
    },
}

impl StatusStacking {
    pub const fn maximum_stacks(self) -> u16 {
        match self {
            Self::KeepExisting | Self::Replace | Self::RefreshDuration => 1,
            Self::AddStacks { maximum_stacks, .. } => maximum_stacks,
        }
    }
}

/// Passive numeric contribution exposed by an active status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusModifier {
    /// Absorbs a bounded amount from one body-HP impact, after other defenses.
    DamageGuard {
        amount: u16,
    },
    ArmorFragilization {
        amount: u16,
    },
    Stability {
        amount: i16,
    },
    MovementTimeMinimum {
        time_units: u16,
    },
    Accuracy {
        amount: i16,
    },
}

/// Status applied only after the owning finite status expires naturally.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusTransition {
    status: StatusId,
    stacks: u16,
}

impl StatusTransition {
    pub fn new(status: StatusId, stacks: u16) -> Result<Self, StatusDefinitionError> {
        if stacks == 0 {
            return Err(StatusDefinitionError::ZeroTransitionStacks);
        }
        Ok(Self { status, stacks })
    }

    pub const fn status(&self) -> &StatusId {
        &self.status
    }

    pub const fn stacks(&self) -> u16 {
        self.stacks
    }
}

/// Generic work executed by a status hook.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusEffectPrimitive {
    DealDamage {
        packet: DamagePacket,
        multiply_by_stacks: bool,
    },
    /// Damages the other actor involved in the trigger, when one exists.
    ///
    /// The counterpart is the attacker for `DamageReceived` and `Death`, and
    /// the damaged target for `DamageDealt`. Turn and movement hooks have no
    /// counterpart, so this primitive is inert for those triggers.
    DealDamageToCounterpart {
        packet: DamagePacket,
        multiply_by_stacks: bool,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusHook {
    trigger: StatusTrigger,
    effects: Vec<StatusEffectPrimitive>,
}

impl StatusHook {
    pub fn new(trigger: StatusTrigger, effects: Vec<StatusEffectPrimitive>) -> Self {
        Self { trigger, effects }
    }

    pub const fn trigger(&self) -> StatusTrigger {
        self.trigger
    }

    pub fn effects(&self) -> &[StatusEffectPrimitive] {
        &self.effects
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct StatusDefinition {
    id: StatusId,
    duration_turns: Option<u16>,
    stacking: StatusStacking,
    hooks: Vec<StatusHook>,
    modifiers: Vec<StatusModifier>,
    family: Option<StatusFamilyId>,
    blocked_families: Vec<StatusFamilyId>,
    expiration_transition: Option<StatusTransition>,
}

impl std::fmt::Debug for StatusDefinition {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut definition = formatter.debug_struct("StatusDefinition");
        definition
            .field("id", &self.id)
            .field("duration_turns", &self.duration_turns)
            .field("stacking", &self.stacking)
            .field("hooks", &self.hooks);
        if !self.modifiers.is_empty() {
            definition.field("modifiers", &self.modifiers);
        }
        if let Some(family) = &self.family {
            definition.field("family", family);
        }
        if !self.blocked_families.is_empty() {
            definition.field("blocked_families", &self.blocked_families);
        }
        if let Some(transition) = &self.expiration_transition {
            definition.field("expiration_transition", transition);
        }
        definition.finish()
    }
}

impl StatusDefinition {
    pub fn new(
        id: StatusId,
        duration_turns: Option<u16>,
        stacking: StatusStacking,
        hooks: Vec<StatusHook>,
    ) -> Result<Self, StatusDefinitionError> {
        if duration_turns == Some(0) {
            return Err(StatusDefinitionError::ZeroDuration);
        }
        if stacking.maximum_stacks() == 0 {
            return Err(StatusDefinitionError::ZeroMaximumStacks);
        }
        Ok(Self {
            id,
            duration_turns,
            stacking,
            hooks,
            modifiers: Vec::new(),
            family: None,
            blocked_families: Vec::new(),
            expiration_transition: None,
        })
    }

    pub fn with_modifiers(
        mut self,
        modifiers: impl IntoIterator<Item = StatusModifier>,
    ) -> Result<Self, StatusDefinitionError> {
        let modifiers: Vec<_> = modifiers.into_iter().collect();
        let guards: Vec<_> = modifiers
            .iter()
            .filter_map(|modifier| match modifier {
                StatusModifier::DamageGuard { amount } => Some(*amount),
                _ => None,
            })
            .collect();
        if guards.len() > 1 {
            return Err(StatusDefinitionError::DuplicateDamageGuard);
        }
        if let Some(amount) = guards.first() {
            if *amount == 0 {
                return Err(StatusDefinitionError::ZeroDamageGuard);
            }
            if self.duration_turns.is_none() || self.stacking != StatusStacking::KeepExisting {
                return Err(StatusDefinitionError::UnboundedDamageGuard);
            }
        }
        let armor_fragilizations = modifiers
            .iter()
            .filter(|modifier| matches!(modifier, StatusModifier::ArmorFragilization { .. }))
            .count();
        if armor_fragilizations > 1 {
            return Err(StatusDefinitionError::DuplicateArmorFragilization);
        }
        if modifiers
            .iter()
            .any(|modifier| matches!(modifier, StatusModifier::ArmorFragilization { amount: 0 }))
        {
            return Err(StatusDefinitionError::ZeroArmorFragilization);
        }
        let stability_modifiers = modifiers
            .iter()
            .filter(|modifier| matches!(modifier, StatusModifier::Stability { .. }))
            .count();
        if stability_modifiers > 1 {
            return Err(StatusDefinitionError::DuplicateStabilityModifier);
        }
        let movement_minima = modifiers
            .iter()
            .filter(|modifier| matches!(modifier, StatusModifier::MovementTimeMinimum { .. }))
            .count();
        if movement_minima > 1 {
            return Err(StatusDefinitionError::DuplicateMovementTimeMinimum);
        }
        if modifiers.iter().any(|modifier| {
            matches!(
                modifier,
                StatusModifier::MovementTimeMinimum { time_units: 0 }
            )
        }) {
            return Err(StatusDefinitionError::ZeroMovementTimeMinimum);
        }
        let accuracy_modifiers = modifiers
            .iter()
            .filter(|modifier| matches!(modifier, StatusModifier::Accuracy { .. }))
            .count();
        if accuracy_modifiers > 1 {
            return Err(StatusDefinitionError::DuplicateAccuracyModifier);
        }
        self.modifiers = modifiers;
        Ok(self)
    }

    pub fn with_family(mut self, family: StatusFamilyId) -> Self {
        self.family = Some(family);
        self
    }

    pub fn with_blocked_families(
        mut self,
        families: impl IntoIterator<Item = StatusFamilyId>,
    ) -> Self {
        self.blocked_families = families.into_iter().collect();
        self
    }

    pub fn with_expiration_transition(mut self, transition: StatusTransition) -> Self {
        self.expiration_transition = Some(transition);
        self
    }

    pub const fn id(&self) -> &StatusId {
        &self.id
    }

    /// A charge counter cannot strengthen damage, defenses or expiration hooks.
    pub fn is_weapon_charge_marker(&self, threshold: u16) -> bool {
        threshold >= 2
            && self.duration_turns.is_some_and(|turns| turns >= 2)
            && self.stacking
                == (StatusStacking::AddStacks {
                    maximum_stacks: threshold,
                    refresh_duration: true,
                })
            && self.hooks.is_empty()
            && self.modifiers.is_empty()
            && self.family.is_none()
            && self.blocked_families.is_empty()
            && self.expiration_transition.is_none()
    }

    /// A cooldown marker is not a buff, debuff, hook or cleanse transition.
    pub fn is_weapon_recovery(&self) -> bool {
        self.duration_turns.is_some_and(|turns| turns >= 2)
            && self.stacking == StatusStacking::KeepExisting
            && self.hooks.is_empty()
            && self.modifiers.is_empty()
            && self.family.is_none()
            && self.blocked_families.is_empty()
            && self.expiration_transition.is_none()
    }

    pub const fn duration_turns(&self) -> Option<u16> {
        self.duration_turns
    }

    pub const fn stacking(&self) -> StatusStacking {
        self.stacking
    }

    pub fn hooks_for(&self, trigger: StatusTrigger) -> impl Iterator<Item = &StatusHook> {
        self.hooks
            .iter()
            .filter(move |hook| hook.trigger() == trigger)
    }

    pub fn modifiers(&self) -> &[StatusModifier] {
        &self.modifiers
    }

    pub const fn family(&self) -> Option<&StatusFamilyId> {
        self.family.as_ref()
    }

    pub fn blocked_families(&self) -> &[StatusFamilyId] {
        &self.blocked_families
    }

    pub const fn expiration_transition(&self) -> Option<&StatusTransition> {
        self.expiration_transition.as_ref()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusDefinitionError {
    ZeroDamageGuard,
    DuplicateDamageGuard,
    UnboundedDamageGuard,
    ZeroDuration,
    ZeroMaximumStacks,
    ZeroArmorFragilization,
    DuplicateArmorFragilization,
    DuplicateStabilityModifier,
    ZeroMovementTimeMinimum,
    DuplicateMovementTimeMinimum,
    DuplicateAccuracyModifier,
    ZeroTransitionStacks,
}

impl Display for StatusDefinitionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroDamageGuard => write!(formatter, "damage guard must be positive"),
            Self::DuplicateDamageGuard => {
                write!(formatter, "a status cannot define damage guard twice")
            }
            Self::UnboundedDamageGuard => write!(
                formatter,
                "damage guard requires a finite keep_existing status"
            ),
            Self::ZeroDuration => write!(formatter, "finite status duration must be positive"),
            Self::ZeroMaximumStacks => write!(formatter, "maximum status stacks must be positive"),
            Self::ZeroArmorFragilization => {
                write!(formatter, "armor fragilization must be positive")
            }
            Self::DuplicateArmorFragilization => {
                write!(
                    formatter,
                    "a status cannot define armor fragilization twice"
                )
            }
            Self::DuplicateStabilityModifier => {
                write!(formatter, "a status cannot modify Stability twice")
            }
            Self::ZeroMovementTimeMinimum => {
                write!(formatter, "movement time minimum must be positive")
            }
            Self::DuplicateMovementTimeMinimum => {
                write!(formatter, "a status cannot define movement time twice")
            }
            Self::DuplicateAccuracyModifier => {
                write!(formatter, "a status cannot modify Accuracy twice")
            }
            Self::ZeroTransitionStacks => {
                write!(formatter, "expiration transition stacks must be positive")
            }
        }
    }
}

impl Error for StatusDefinitionError {}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StatusCatalog {
    definitions: BTreeMap<StatusId, StatusDefinition>,
}

impl StatusCatalog {
    /// Reconstructs historical stacking rules for deterministic old replays.
    pub fn with_compatibility_stacking(&self, id: &StatusId, stacking: StatusStacking) -> Self {
        let mut catalog = self.clone();
        if let Some(definition) = catalog.definitions.get_mut(id) {
            definition.stacking = stacking;
        }
        catalog
    }

    pub fn register(&mut self, definition: StatusDefinition) -> Result<(), StatusCatalogError> {
        let id = definition.id().clone();
        if self.definitions.contains_key(&id) {
            return Err(StatusCatalogError::DuplicateId(id));
        }
        self.definitions.insert(id, definition);
        Ok(())
    }

    pub fn get(&self, id: &StatusId) -> Option<&StatusDefinition> {
        self.definitions.get(id)
    }

    pub fn contains(&self, id: &StatusId) -> bool {
        self.definitions.contains_key(id)
    }

    pub fn contains_family(&self, family: &StatusFamilyId) -> bool {
        self.definitions
            .values()
            .any(|definition| definition.family() == Some(family))
    }

    pub fn without_id(&self, excluded: &StatusId) -> Self {
        let mut definitions = self.definitions.clone();
        definitions.remove(excluded);
        Self { definitions }
    }

    pub fn validate_references(&self) -> Result<(), StatusCatalogError> {
        for (source, definition) in &self.definitions {
            if let Some(transition) = definition.expiration_transition()
                && !self.contains(transition.status())
            {
                return Err(StatusCatalogError::UnknownTransitionStatus {
                    source: Box::new(source.clone()),
                    target: Box::new(transition.status().clone()),
                });
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StatusCatalogError {
    DuplicateId(StatusId),
    UnknownTransitionStatus {
        source: Box<StatusId>,
        target: Box<StatusId>,
    },
}

impl Display for StatusCatalogError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateId(id) => write!(formatter, "duplicate status ID '{}'", id.as_str()),
            Self::UnknownTransitionStatus { source, target } => write!(
                formatter,
                "status '{}' transitions to unknown status '{}'",
                source.as_str(),
                target.as_str()
            ),
        }
    }
}

impl Error for StatusCatalogError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn status_id(value: &str) -> StatusId {
        value
            .parse()
            .unwrap_or_else(|error| panic!("valid status ID rejected: {error}"))
    }

    #[test]
    fn ids_require_a_stable_namespace_and_name() {
        assert!("core:corroded".parse::<StatusId>().is_ok());
        assert!("corroded".parse::<StatusId>().is_err());
        assert!("core:".parse::<StatusId>().is_err());
        assert!("core:bad:id".parse::<StatusId>().is_err());
    }

    #[test]
    fn catalog_rejects_duplicate_definitions() {
        let definition = StatusDefinition::new(
            status_id("core:corroded"),
            Some(3),
            StatusStacking::RefreshDuration,
            Vec::new(),
        )
        .unwrap_or_else(|error| panic!("valid definition rejected: {error}"));
        let mut catalog = StatusCatalog::default();

        assert_eq!(catalog.register(definition.clone()), Ok(()));
        assert_eq!(
            catalog.register(definition),
            Err(StatusCatalogError::DuplicateId(status_id("core:corroded")))
        );
    }

    #[test]
    fn armor_fragilization_modifier_is_positive_and_unique_per_status() {
        let base = StatusDefinition::new(
            status_id("core:fractured"),
            Some(3),
            StatusStacking::KeepExisting,
            Vec::new(),
        )
        .unwrap();

        assert_eq!(
            base.clone()
                .with_modifiers([StatusModifier::ArmorFragilization { amount: 0 }]),
            Err(StatusDefinitionError::ZeroArmorFragilization)
        );
        assert_eq!(
            base.with_modifiers([
                StatusModifier::ArmorFragilization { amount: 4 },
                StatusModifier::ArmorFragilization { amount: 6 },
            ]),
            Err(StatusDefinitionError::DuplicateArmorFragilization)
        );
    }

    #[test]
    fn movement_minimum_and_expiration_transition_are_validated() {
        let base = StatusDefinition::new(
            status_id("core:hindered"),
            Some(2),
            StatusStacking::KeepExisting,
            Vec::new(),
        )
        .unwrap();
        assert_eq!(
            base.clone()
                .with_modifiers([StatusModifier::MovementTimeMinimum { time_units: 0 }]),
            Err(StatusDefinitionError::ZeroMovementTimeMinimum)
        );
        assert_eq!(
            StatusTransition::new(status_id("core:protected"), 0),
            Err(StatusDefinitionError::ZeroTransitionStacks)
        );

        let mut catalog = StatusCatalog::default();
        catalog
            .register(base.with_expiration_transition(
                StatusTransition::new(status_id("core:protected"), 1).unwrap(),
            ))
            .unwrap();
        assert!(matches!(
            catalog.validate_references(),
            Err(StatusCatalogError::UnknownTransitionStatus { .. })
        ));
        catalog
            .register(
                StatusDefinition::new(
                    status_id("core:protected"),
                    Some(1),
                    StatusStacking::KeepExisting,
                    Vec::new(),
                )
                .unwrap(),
            )
            .unwrap();
        assert_eq!(catalog.validate_references(), Ok(()));
    }
}
