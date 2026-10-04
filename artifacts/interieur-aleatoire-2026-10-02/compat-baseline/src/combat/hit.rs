use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::stats::{PrimaryAttribute, PrimaryAttributes};

use super::AttackDelivery;

/// Data-shaped accuracy and evasion coefficients used by the deterministic
/// combat resolver. Keeping the coefficients outside actor or weapon code lets
/// a rules package replace the balance without replacing resolution logic.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HitRules {
    pub neutral_attribute: u8,
    pub melee_base_accuracy: i16,
    pub ranged_base_accuracy: i16,
    pub accuracy_per_coordination: i16,
    pub ranged_accuracy_per_perception: i16,
    pub base_evasion: i16,
    pub evasion_per_coordination: i16,
    pub minimum_hit_chance: u8,
    pub maximum_hit_chance: u8,
}

impl HitRules {
    pub fn validate(self) -> Result<(), HitRulesError> {
        if self.neutral_attribute == 0 {
            return Err(HitRulesError::ZeroNeutralAttribute);
        }
        if self.accuracy_per_coordination < 0
            || self.ranged_accuracy_per_perception < 0
            || self.evasion_per_coordination < 0
        {
            return Err(HitRulesError::NegativeAttributeCoefficient);
        }
        if self.minimum_hit_chance > self.maximum_hit_chance || self.maximum_hit_chance > 100 {
            return Err(HitRulesError::InvalidChanceBounds {
                minimum: self.minimum_hit_chance,
                maximum: self.maximum_hit_chance,
            });
        }
        Ok(())
    }

    pub fn accuracy(
        self,
        delivery: AttackDelivery,
        attributes: Option<PrimaryAttributes>,
        modifier: i16,
    ) -> i32 {
        let coordination = attribute_value(
            attributes,
            PrimaryAttribute::Coordination,
            self.neutral_attribute,
        );
        let perception = attribute_value(
            attributes,
            PrimaryAttribute::Perception,
            self.neutral_attribute,
        );
        let coordination_bonus = attribute_delta(coordination, self.neutral_attribute)
            * i32::from(self.accuracy_per_coordination);
        let (base, perception_bonus) = match delivery {
            AttackDelivery::Melee => (self.melee_base_accuracy, 0),
            AttackDelivery::Ranged => (
                self.ranged_base_accuracy,
                attribute_delta(perception, self.neutral_attribute)
                    * i32::from(self.ranged_accuracy_per_perception),
            ),
        };
        i32::from(base) + coordination_bonus + perception_bonus + i32::from(modifier)
    }

    pub fn evasion(self, attributes: Option<PrimaryAttributes>, modifier: i16) -> i32 {
        let coordination = attribute_value(
            attributes,
            PrimaryAttribute::Coordination,
            self.neutral_attribute,
        );
        (i32::from(self.base_evasion)
            + attribute_delta(coordination, self.neutral_attribute)
                * i32::from(self.evasion_per_coordination)
            + i32::from(modifier))
        .max(0)
    }

    pub fn hit_chance(
        self,
        delivery: AttackDelivery,
        attacker_attributes: Option<PrimaryAttributes>,
        accuracy_modifier: i16,
        defender_attributes: Option<PrimaryAttributes>,
        evasion_modifier: i16,
        context_modifier: i16,
    ) -> u8 {
        let raw = self.accuracy(delivery, attacker_attributes, accuracy_modifier)
            - self.evasion(defender_attributes, evasion_modifier)
            + i32::from(context_modifier);
        raw.clamp(
            i32::from(self.minimum_hit_chance),
            i32::from(self.maximum_hit_chance),
        ) as u8
    }
}

impl Default for HitRules {
    fn default() -> Self {
        Self {
            neutral_attribute: 5,
            melee_base_accuracy: 80,
            ranged_base_accuracy: 80,
            accuracy_per_coordination: 4,
            ranged_accuracy_per_perception: 2,
            base_evasion: 10,
            evasion_per_coordination: 4,
            minimum_hit_chance: 5,
            maximum_hit_chance: 95,
        }
    }
}

fn attribute_value(
    attributes: Option<PrimaryAttributes>,
    attribute: PrimaryAttribute,
    neutral: u8,
) -> u8 {
    attributes.map_or(neutral, |values| values.value(attribute))
}

fn attribute_delta(value: u8, neutral: u8) -> i32 {
    i32::from(value) - i32::from(neutral)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HitRulesError {
    ZeroNeutralAttribute,
    NegativeAttributeCoefficient,
    InvalidChanceBounds { minimum: u8, maximum: u8 },
}

impl Display for HitRulesError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroNeutralAttribute => {
                write!(formatter, "neutral hit-rule attribute must be positive")
            }
            Self::NegativeAttributeCoefficient => {
                write!(
                    formatter,
                    "hit-rule attribute coefficients cannot be negative"
                )
            }
            Self::InvalidChanceBounds { minimum, maximum } => write!(
                formatter,
                "hit chance bounds must be ordered percentages, got {minimum}..={maximum}"
            ),
        }
    }
}

impl Error for HitRulesError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn attributes(coordination: u8, perception: u8) -> PrimaryAttributes {
        PrimaryAttributes::new(5, coordination, 5, perception, 5)
    }

    #[test]
    fn documented_accuracy_examples_are_exact() {
        let rules = HitRules::default();
        let ordinary = Some(attributes(5, 5));
        let coordinated = Some(attributes(8, 5));
        let perceptive = Some(attributes(5, 8));

        assert_eq!(
            rules.hit_chance(AttackDelivery::Ranged, ordinary, 0, ordinary, 0, 0),
            70
        );
        assert_eq!(
            rules.hit_chance(AttackDelivery::Ranged, coordinated, 0, ordinary, 0, 0,),
            82
        );
        assert_eq!(
            rules.hit_chance(AttackDelivery::Ranged, perceptive, 0, ordinary, 0, 0,),
            76
        );
        assert_eq!(
            rules.hit_chance(AttackDelivery::Ranged, ordinary, 0, coordinated, 0, 0,),
            58
        );
        assert_eq!(
            rules.hit_chance(AttackDelivery::Ranged, coordinated, 0, ordinary, 0, -20,),
            62
        );
    }

    #[test]
    fn melee_ignores_perception_and_missing_attributes_use_the_neutral_profile() {
        let rules = HitRules::default();

        assert_eq!(
            rules.hit_chance(
                AttackDelivery::Melee,
                Some(attributes(5, 10)),
                0,
                None,
                0,
                0,
            ),
            70
        );
        assert_eq!(
            rules.hit_chance(AttackDelivery::Ranged, None, 0, None, 0, 0),
            70
        );
    }

    #[test]
    fn configured_bounds_and_validation_are_enforced() {
        let mut rules = HitRules::default();
        assert_eq!(
            rules.hit_chance(
                AttackDelivery::Ranged,
                Some(attributes(10, 10)),
                100,
                Some(attributes(1, 1)),
                0,
                0,
            ),
            95
        );
        rules.minimum_hit_chance = 101;
        assert!(matches!(
            rules.validate(),
            Err(HitRulesError::InvalidChanceBounds { .. })
        ));
    }
}
