use std::error::Error;
use std::fmt::{Display, Formatter};

use super::{PrimaryAttribute, PrimaryAttributes};

/// Data-shaped resistance to a precise functional perturbation.
///
/// Stability never reduces damage. Callers decide whether a perturbation is
/// compatible before requesting a chance, and own the deterministic RNG roll.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StabilityRules {
    pub neutral_resilience: u8,
    pub base_stability: i16,
    pub stability_per_resilience: i16,
    pub base_resistance_chance: i16,
    pub minimum_resistance_chance: u8,
    pub maximum_resistance_chance: u8,
}

impl StabilityRules {
    pub fn validate(self) -> Result<(), StabilityRulesError> {
        if self.neutral_resilience == 0 {
            return Err(StabilityRulesError::ZeroNeutralResilience);
        }
        if self.stability_per_resilience <= 0 {
            return Err(StabilityRulesError::NonPositiveResilienceContribution);
        }
        if self.minimum_resistance_chance == 0
            || self.maximum_resistance_chance > 100
            || self.minimum_resistance_chance > self.maximum_resistance_chance
        {
            return Err(StabilityRulesError::InvalidChanceBounds {
                minimum: self.minimum_resistance_chance,
                maximum: self.maximum_resistance_chance,
            });
        }
        Ok(())
    }

    pub fn stability(self, attributes: Option<PrimaryAttributes>, explicit_modifier: i16) -> u16 {
        let resilience = attributes.map_or(self.neutral_resilience, |values| {
            values.value(PrimaryAttribute::Resilience)
        });
        let delta = i32::from(resilience) - i32::from(self.neutral_resilience);
        let raw = i32::from(self.base_stability)
            + delta * i32::from(self.stability_per_resilience)
            + i32::from(explicit_modifier);
        raw.clamp(0, i32::from(u16::MAX)) as u16
    }

    pub fn resistance_chance(
        self,
        attributes: Option<PrimaryAttributes>,
        explicit_modifier: i16,
        intensity: u16,
    ) -> u8 {
        let raw = i32::from(self.base_resistance_chance)
            + i32::from(self.stability(attributes, explicit_modifier))
            - i32::from(intensity);
        raw.clamp(
            i32::from(self.minimum_resistance_chance),
            i32::from(self.maximum_resistance_chance),
        ) as u8
    }
}

impl Default for StabilityRules {
    fn default() -> Self {
        Self {
            neutral_resilience: 5,
            base_stability: 50,
            stability_per_resilience: 5,
            base_resistance_chance: 50,
            minimum_resistance_chance: 5,
            maximum_resistance_chance: 95,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StabilityRulesError {
    ZeroNeutralResilience,
    NonPositiveResilienceContribution,
    InvalidChanceBounds { minimum: u8, maximum: u8 },
}

impl Display for StabilityRulesError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroNeutralResilience => {
                write!(formatter, "neutral Stability resilience must be positive")
            }
            Self::NonPositiveResilienceContribution => write!(
                formatter,
                "Stability contribution per resilience point must be positive"
            ),
            Self::InvalidChanceBounds { minimum, maximum } => write!(
                formatter,
                "Stability resistance chance bounds must satisfy 1 <= minimum <= maximum <= 100, found {minimum}..={maximum}"
            ),
        }
    }
}

impl Error for StabilityRulesError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documented_stability_examples_are_exact_and_bounded() {
        let rules = StabilityRules::default();
        for (resilience, modifier, intensity, stability, chance) in [
            (3, 0, 50, 40, 40),
            (5, 0, 50, 50, 50),
            (8, 0, 50, 65, 65),
            (8, 20, 50, 85, 85),
            (5, 0, 70, 50, 30),
        ] {
            let attributes = PrimaryAttributes::new(5, 5, resilience, 5, 5);
            assert_eq!(rules.stability(Some(attributes), modifier), stability);
            assert_eq!(
                rules.resistance_chance(Some(attributes), modifier, intensity),
                chance
            );
        }
        assert_eq!(rules.resistance_chance(None, -500, u16::MAX), 5);
        assert_eq!(rules.resistance_chance(None, 500, 0), 95);
    }

    #[test]
    fn invalid_stability_coefficients_are_rejected_before_simulation() {
        assert_eq!(
            StabilityRules {
                neutral_resilience: 0,
                ..StabilityRules::default()
            }
            .validate(),
            Err(StabilityRulesError::ZeroNeutralResilience)
        );
        assert!(
            StabilityRules {
                minimum_resistance_chance: 96,
                maximum_resistance_chance: 95,
                ..StabilityRules::default()
            }
            .validate()
            .is_err()
        );
    }
}
