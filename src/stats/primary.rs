use std::error::Error;
use std::fmt::{Display, Formatter};

pub const PRIMARY_ATTRIBUTE_COUNT: usize = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrimaryAttribute {
    Power,
    Coordination,
    Resilience,
    Perception,
    Processing,
}

impl PrimaryAttribute {
    pub const ALL: [Self; PRIMARY_ATTRIBUTE_COUNT] = [
        Self::Power,
        Self::Coordination,
        Self::Resilience,
        Self::Perception,
        Self::Processing,
    ];
}

impl Display for PrimaryAttribute {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Power => "power",
            Self::Coordination => "coordination",
            Self::Resilience => "resilience",
            Self::Perception => "perception",
            Self::Processing => "processing",
        };
        formatter.write_str(name)
    }
}

/// The five intrinsic attributes of one character.
///
/// Construction does not silently apply creation limits: a loaded class,
/// save, temporary test profile, or character-creation screen must validate
/// the values against the active [`PrimaryAttributeRules`] explicitly.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrimaryAttributes {
    values: [u8; PRIMARY_ATTRIBUTE_COUNT],
}

impl PrimaryAttributes {
    pub const fn new(
        power: u8,
        coordination: u8,
        resilience: u8,
        perception: u8,
        processing: u8,
    ) -> Self {
        Self {
            values: [power, coordination, resilience, perception, processing],
        }
    }

    /// Temporary classless profile used by the current prototype.
    /// It is the valid 6/6/6/5/5 example from the design rules, not a final
    /// starting class.
    pub const fn prototype_default() -> Self {
        Self::new(6, 6, 6, 5, 5)
    }

    pub const fn value(self, attribute: PrimaryAttribute) -> u8 {
        self.values[attribute as usize]
    }

    pub fn with_value(mut self, attribute: PrimaryAttribute, value: u8) -> Self {
        self.values[attribute as usize] = value;
        self
    }

    pub const fn values(self) -> [u8; PRIMARY_ATTRIBUTE_COUNT] {
        self.values
    }

    pub fn iter(self) -> impl Iterator<Item = (PrimaryAttribute, u8)> {
        PrimaryAttribute::ALL
            .into_iter()
            .map(move |attribute| (attribute, self.value(attribute)))
    }

    pub fn total(self) -> u16 {
        self.values.iter().map(|value| u16::from(*value)).sum()
    }

    pub fn validate_for_creation(
        self,
        rules: PrimaryAttributeRules,
    ) -> Result<(), PrimaryAttributesError> {
        rules
            .validate()
            .map_err(PrimaryAttributesError::InvalidRules)?;
        for (attribute, value) in self.iter() {
            if !(rules.creation_minimum..=rules.creation_maximum).contains(&value) {
                return Err(PrimaryAttributesError::OutsideCreationRange {
                    attribute,
                    value,
                    minimum: rules.creation_minimum,
                    maximum: rules.creation_maximum,
                });
            }
        }
        let actual = self.total();
        if actual != rules.creation_total {
            return Err(PrimaryAttributesError::WrongCreationTotal {
                actual,
                expected: rules.creation_total,
            });
        }
        Ok(())
    }

    pub fn validate_absolute(
        self,
        rules: PrimaryAttributeRules,
    ) -> Result<(), PrimaryAttributesError> {
        rules
            .validate()
            .map_err(PrimaryAttributesError::InvalidRules)?;
        for (attribute, value) in self.iter() {
            if !(rules.absolute_minimum..=rules.absolute_maximum).contains(&value) {
                return Err(PrimaryAttributesError::OutsideAbsoluteRange {
                    attribute,
                    value,
                    minimum: rules.absolute_minimum,
                    maximum: rules.absolute_maximum,
                });
            }
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PrimaryAttributeRules {
    pub absolute_minimum: u8,
    pub absolute_maximum: u8,
    pub creation_minimum: u8,
    pub creation_maximum: u8,
    pub creation_total: u16,
}

impl PrimaryAttributeRules {
    pub fn validate(self) -> Result<(), PrimaryAttributeRulesError> {
        if self.absolute_minimum == 0 {
            return Err(PrimaryAttributeRulesError::ZeroAbsoluteMinimum);
        }
        if self.absolute_minimum > self.creation_minimum
            || self.creation_minimum > self.creation_maximum
            || self.creation_maximum > self.absolute_maximum
        {
            return Err(PrimaryAttributeRulesError::InconsistentBounds);
        }
        let count = PRIMARY_ATTRIBUTE_COUNT as u16;
        let minimum_total = u16::from(self.creation_minimum).saturating_mul(count);
        let maximum_total = u16::from(self.creation_maximum).saturating_mul(count);
        if !(minimum_total..=maximum_total).contains(&self.creation_total) {
            return Err(PrimaryAttributeRulesError::ImpossibleCreationTotal {
                total: self.creation_total,
                minimum: minimum_total,
                maximum: maximum_total,
            });
        }
        Ok(())
    }
}

impl Default for PrimaryAttributeRules {
    fn default() -> Self {
        Self {
            absolute_minimum: 1,
            absolute_maximum: 10,
            creation_minimum: 3,
            creation_maximum: 8,
            creation_total: 28,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimaryAttributeRulesError {
    ZeroAbsoluteMinimum,
    InconsistentBounds,
    ImpossibleCreationTotal {
        total: u16,
        minimum: u16,
        maximum: u16,
    },
}

impl Display for PrimaryAttributeRulesError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroAbsoluteMinimum => {
                write!(
                    formatter,
                    "absolute primary attribute minimum must be positive"
                )
            }
            Self::InconsistentBounds => write!(
                formatter,
                "primary attribute bounds must satisfy absolute minimum <= creation minimum <= creation maximum <= absolute maximum"
            ),
            Self::ImpossibleCreationTotal {
                total,
                minimum,
                maximum,
            } => write!(
                formatter,
                "creation total {total} is impossible with the configured bounds ({minimum}..={maximum})"
            ),
        }
    }
}

impl Error for PrimaryAttributeRulesError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PrimaryAttributesError {
    InvalidRules(PrimaryAttributeRulesError),
    OutsideAbsoluteRange {
        attribute: PrimaryAttribute,
        value: u8,
        minimum: u8,
        maximum: u8,
    },
    OutsideCreationRange {
        attribute: PrimaryAttribute,
        value: u8,
        minimum: u8,
        maximum: u8,
    },
    WrongCreationTotal {
        actual: u16,
        expected: u16,
    },
}

impl Display for PrimaryAttributesError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRules(error) => {
                write!(formatter, "invalid primary attribute rules: {error}")
            }
            Self::OutsideAbsoluteRange {
                attribute,
                value,
                minimum,
                maximum,
            } => write!(
                formatter,
                "{attribute} value {value} is outside the absolute range {minimum}..={maximum}"
            ),
            Self::OutsideCreationRange {
                attribute,
                value,
                minimum,
                maximum,
            } => write!(
                formatter,
                "{attribute} value {value} is outside the creation range {minimum}..={maximum}"
            ),
            Self::WrongCreationTotal { actual, expected } => write!(
                formatter,
                "primary attribute creation total is {actual}, expected {expected}"
            ),
        }
    }
}

impl Error for PrimaryAttributesError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documented_prototype_profile_is_a_legal_creation() {
        let attributes = PrimaryAttributes::prototype_default();

        assert_eq!(attributes.total(), 28);
        assert_eq!(
            attributes.validate_for_creation(PrimaryAttributeRules::default()),
            Ok(())
        );
    }

    #[test]
    fn creation_rejects_each_out_of_range_value_before_mutation() {
        let attributes = PrimaryAttributes::new(9, 5, 5, 5, 4);

        assert!(matches!(
            attributes.validate_for_creation(PrimaryAttributeRules::default()),
            Err(PrimaryAttributesError::OutsideCreationRange {
                attribute: PrimaryAttribute::Power,
                value: 9,
                ..
            })
        ));
    }

    #[test]
    fn creation_requires_the_exact_documented_budget() {
        let attributes = PrimaryAttributes::new(5, 5, 5, 5, 5);

        assert_eq!(
            attributes.validate_for_creation(PrimaryAttributeRules::default()),
            Err(PrimaryAttributesError::WrongCreationTotal {
                actual: 25,
                expected: 28,
            })
        );
    }

    #[test]
    fn impossible_creation_rules_are_rejected_explicitly() {
        let rules = PrimaryAttributeRules {
            creation_total: 50,
            ..PrimaryAttributeRules::default()
        };

        assert_eq!(
            rules.validate(),
            Err(PrimaryAttributeRulesError::ImpossibleCreationTotal {
                total: 50,
                minimum: 15,
                maximum: 40,
            })
        );
    }

    #[test]
    fn non_player_profiles_use_absolute_bounds_without_creation_budget() {
        let unusual_but_legal = PrimaryAttributes::new(10, 1, 1, 1, 1);
        let illegal = PrimaryAttributes::new(11, 1, 1, 1, 1);

        assert_eq!(
            unusual_but_legal.validate_absolute(PrimaryAttributeRules::default()),
            Ok(())
        );
        assert!(matches!(
            illegal.validate_absolute(PrimaryAttributeRules::default()),
            Err(PrimaryAttributesError::OutsideAbsoluteRange {
                attribute: PrimaryAttribute::Power,
                value: 11,
                ..
            })
        ));
    }
}
