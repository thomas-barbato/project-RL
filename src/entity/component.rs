use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::content::ContentId;

pub type BodyComponentId = ContentId;

/// Concrete engine consequence of a failed body component. Content chooses
/// the primitive; combat techniques only address the component ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComponentFailureEffect {
    DisableMovement,
    DisableAttackSlot(u8),
    ReduceArmor(u16),
    ReducePerception(u16),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BodyComponentProfile {
    id: BodyComponentId,
    name_key: String,
    maximum_durability: u16,
    failure_threshold: u16,
    failure_effect: ComponentFailureEffect,
}

impl BodyComponentProfile {
    pub fn new(
        id: BodyComponentId,
        name_key: String,
        maximum_durability: u16,
        failure_threshold: u16,
        failure_effect: ComponentFailureEffect,
    ) -> Result<Self, BodyComponentError> {
        if maximum_durability == 0 {
            return Err(BodyComponentError::ZeroMaximumDurability);
        }
        if failure_threshold >= maximum_durability {
            return Err(BodyComponentError::InvalidFailureThreshold {
                threshold: failure_threshold,
                maximum: maximum_durability,
            });
        }
        Ok(Self {
            id,
            name_key,
            maximum_durability,
            failure_threshold,
            failure_effect,
        })
    }

    pub const fn id(&self) -> &BodyComponentId {
        &self.id
    }

    pub fn name_key(&self) -> &str {
        &self.name_key
    }

    pub const fn maximum_durability(&self) -> u16 {
        self.maximum_durability
    }

    pub const fn failure_threshold(&self) -> u16 {
        self.failure_threshold
    }

    pub const fn failure_effect(&self) -> ComponentFailureEffect {
        self.failure_effect
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BodyComponentState {
    profile: BodyComponentProfile,
    durability: u16,
}

impl BodyComponentState {
    pub fn new(profile: BodyComponentProfile) -> Self {
        let durability = profile.maximum_durability();
        Self {
            profile,
            durability,
        }
    }

    pub const fn profile(&self) -> &BodyComponentProfile {
        &self.profile
    }

    pub const fn durability(&self) -> u16 {
        self.durability
    }

    pub const fn maximum_durability(&self) -> u16 {
        self.profile.maximum_durability()
    }

    pub const fn is_failed(&self) -> bool {
        self.durability <= self.profile.failure_threshold()
    }

    pub const fn is_destroyed(&self) -> bool {
        self.durability == 0
    }

    pub fn apply_damage(&mut self, amount: u16) -> u16 {
        let applied = amount.min(self.durability);
        self.durability -= applied;
        applied
    }

    pub fn restore(&mut self, amount: u16) -> u16 {
        let restored = amount.min(self.maximum_durability().saturating_sub(self.durability));
        self.durability = self.durability.saturating_add(restored);
        restored
    }

    /// Restores a repairable component without recreating a component whose
    /// own durability reached zero.
    pub fn restore_if_repairable(&mut self, amount: u16) -> u16 {
        if self.is_destroyed() {
            return 0;
        }
        self.restore(amount)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BodyComponentError {
    ZeroMaximumDurability,
    InvalidFailureThreshold { threshold: u16, maximum: u16 },
}

impl Display for BodyComponentError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroMaximumDurability => {
                write!(formatter, "body component durability must be positive")
            }
            Self::InvalidFailureThreshold { threshold, maximum } => write!(
                formatter,
                "body component failure threshold {threshold} must be below maximum {maximum}"
            ),
        }
    }
}

impl Error for BodyComponentError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn component_failure_uses_its_own_durability_threshold() {
        let profile = BodyComponentProfile::new(
            "core:locomotion".parse().unwrap(),
            "component.locomotion.name".to_owned(),
            10,
            3,
            ComponentFailureEffect::DisableMovement,
        )
        .unwrap();
        let mut component = BodyComponentState::new(profile);

        assert_eq!(component.apply_damage(6), 6);
        assert!(!component.is_failed());
        assert_eq!(component.apply_damage(1), 1);
        assert!(component.is_failed());
        assert_eq!(component.restore(2), 2);
        assert!(!component.is_failed());
    }
}
