use std::error::Error;
use std::fmt::{Display, Formatter};

pub const PRIMARY_ATTRIBUTE_COUNT: usize = 6;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PrimaryAttribute {
    Power,
    Coordination,
    Resilience,
    Perception,
    Processing,
    Willpower,
}

impl PrimaryAttribute {
    pub const ALL: [Self; PRIMARY_ATTRIBUTE_COUNT] = [
        Self::Power,
        Self::Coordination,
        Self::Resilience,
        Self::Perception,
        Self::Processing,
        Self::Willpower,
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
            Self::Willpower => "willpower",
        };
        formatter.write_str(name)
    }
}

/// Intrinsic attributes, including Willpower for cryogenic agents.
///
/// Construction does not silently apply creation limits: a loaded class,
/// save, temporary test profile, or character-creation screen must validate
/// the values against the active [`PrimaryAttributeRules`] explicitly.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PrimaryAttributes {
    values: [u16; 5],
    willpower: u16,
}

thread_local! {
    static LEGACY_ATTRIBUTE_ENCODING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// The v12 recovery format stored attributes in bytes. Scope that codec to
/// decoding old snapshots; current saves can represent normal high-level stats.
pub(crate) fn with_legacy_attribute_encoding<T>(operation: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset {
        fn drop(&mut self) {
            LEGACY_ATTRIBUTE_ENCODING.set(self.0);
        }
    }
    let _reset = Reset(LEGACY_ATTRIBUTE_ENCODING.replace(true));
    crate::status::with_legacy_status_encoding(operation)
}

#[derive(serde::Serialize, serde::Deserialize)]
struct WideAttributes {
    values: [u16; 5],
    #[serde(default)]
    willpower: u16,
}
#[derive(serde::Serialize, serde::Deserialize)]
struct LegacyAttributes {
    values: [u8; 5],
    #[serde(default)]
    willpower: u8,
}
impl serde::Serialize for PrimaryAttributes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if LEGACY_ATTRIBUTE_ENCODING.get() && !serializer.is_human_readable() {
            let mut values = [0; 5];
            for (destination, value) in values.iter_mut().zip(self.values) {
                *destination = u8::try_from(value).map_err(serde::ser::Error::custom)?;
            }
            LegacyAttributes {
                values,
                willpower: u8::try_from(self.willpower).map_err(serde::ser::Error::custom)?,
            }
            .serialize(serializer)
        } else {
            WideAttributes {
                values: self.values,
                willpower: self.willpower,
            }
            .serialize(serializer)
        }
    }
}
impl<'de> serde::Deserialize<'de> for PrimaryAttributes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if LEGACY_ATTRIBUTE_ENCODING.get() && !deserializer.is_human_readable() {
            let value = LegacyAttributes::deserialize(deserializer)?;
            Ok(Self {
                values: value.values.map(u16::from),
                willpower: u16::from(value.willpower),
            })
        } else {
            let value = WideAttributes::deserialize(deserializer)?;
            Ok(Self {
                values: value.values,
                willpower: value.willpower,
            })
        }
    }
}

impl std::fmt::Debug for PrimaryAttributes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut values = f.debug_struct("PrimaryAttributes");
        values.field("values", &self.values);
        if self.willpower > 0 {
            values.field("willpower", &self.willpower);
        }
        values.finish()
    }
}

impl PrimaryAttributes {
    pub const fn new(
        power: u16,
        coordination: u16,
        resilience: u16,
        perception: u16,
        processing: u16,
    ) -> Self {
        Self {
            values: [power, coordination, resilience, perception, processing],
            willpower: 0,
        }
    }

    /// Temporary classless profile used by the current prototype.
    /// It is the valid 6/6/6/5/5 example from the design rules, not a final
    /// starting class.
    pub const fn prototype_default() -> Self {
        Self::new(6, 6, 6, 5, 5)
    }

    pub const fn value(self, attribute: PrimaryAttribute) -> u16 {
        if matches!(attribute, PrimaryAttribute::Willpower) {
            self.willpower
        } else {
            self.values[attribute as usize]
        }
    }

    pub const fn with_willpower(mut self, value: u16) -> Self {
        self.willpower = value;
        self
    }

    pub fn with_value(mut self, attribute: PrimaryAttribute, value: u16) -> Self {
        if matches!(attribute, PrimaryAttribute::Willpower) {
            self.willpower = value;
        } else {
            self.values[attribute as usize] = value;
        }
        self
    }

    pub const fn values(self) -> [u16; 5] {
        self.values
    }

    pub fn iter(self) -> impl Iterator<Item = (PrimaryAttribute, u16)> {
        PrimaryAttribute::ALL
            .into_iter()
            .filter(move |attribute| {
                !matches!(attribute, PrimaryAttribute::Willpower) || self.willpower > 0
            })
            .map(move |attribute| (attribute, self.value(attribute)))
    }

    pub fn total(self) -> u32 {
        self.values
            .iter()
            .map(|value| u32::from(*value))
            .sum::<u32>()
            + u32::from(self.willpower)
    }

    pub fn validate_for_creation(
        self,
        rules: PrimaryAttributeRules,
    ) -> Result<(), PrimaryAttributesError> {
        rules
            .validate()
            .map_err(PrimaryAttributesError::InvalidRules)?;
        for (attribute, value) in self.iter() {
            if !(u16::from(rules.creation_minimum)..=u16::from(rules.creation_maximum))
                .contains(&value)
            {
                return Err(PrimaryAttributesError::OutsideCreationRange {
                    attribute,
                    value,
                    minimum: rules.creation_minimum,
                    maximum: rules.creation_maximum,
                });
            }
        }
        let actual = self.total();
        if actual != u32::from(rules.creation_total) {
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
            if !(u16::from(rules.absolute_minimum)..=u16::from(rules.absolute_maximum))
                .contains(&value)
            {
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
        value: u16,
        minimum: u8,
        maximum: u8,
    },
    OutsideCreationRange {
        attribute: PrimaryAttribute,
        value: u16,
        minimum: u8,
        maximum: u8,
    },
    WrongCreationTotal {
        actual: u32,
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
                minimum: 18,
                maximum: 48,
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
