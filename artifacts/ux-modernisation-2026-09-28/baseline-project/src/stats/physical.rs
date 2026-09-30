use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use super::{PrimaryAttribute, PrimaryAttributes};

const GRAMS_PER_DISPLACEMENT_RESISTANCE: u32 = 10_000;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhysicalRules {
    pub impact: ImpactRules,
    pub hit_points: HitPointRules,
}

impl PhysicalRules {
    pub fn validate(self) -> Result<(), PhysicalRulesError> {
        self.impact.validate()?;
        self.hit_points.validate()
    }
}

/// Configurable coefficients for the physical effort supplied to one gesture.
///
/// A caller still has to decide whether the gesture is compatible and which
/// material cap applies. This resolver never grants the right to use an item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ImpactRules {
    pub neutral_attribute: u8,
    pub impact_per_power: u16,
    pub reference_impact: u16,
}

impl ImpactRules {
    pub fn validate(self) -> Result<(), PhysicalRulesError> {
        if self.neutral_attribute == 0 {
            return Err(PhysicalRulesError::ZeroNeutralAttribute);
        }
        Ok(())
    }

    pub fn available_impact(
        self,
        attributes: Option<PrimaryAttributes>,
        impact_modifier: i16,
    ) -> u16 {
        let power = attributes.map_or(self.neutral_attribute, |values| {
            values.value(PrimaryAttribute::Power)
        });
        let raw = i32::from(power) * i32::from(self.impact_per_power) + i32::from(impact_modifier);
        raw.clamp(0, i32::from(u16::MAX)) as u16
    }

    pub fn resolve_melee_damage(
        self,
        attributes: Option<PrimaryAttributes>,
        impact_modifier: i16,
        material_cap: u16,
        reference_physical_damage: u16,
    ) -> ResolvedImpact {
        let available = self.available_impact(attributes, impact_modifier);
        let used = available.min(material_cap);
        let bonus = i32::from(used) - i32::from(self.reference_impact);
        let raw_physical_damage =
            (i32::from(reference_physical_damage) + bonus).clamp(0, i32::from(u16::MAX)) as u16;

        ResolvedImpact {
            available,
            used,
            bonus,
            raw_physical_damage,
        }
    }
}

impl Default for ImpactRules {
    fn default() -> Self {
        Self {
            neutral_attribute: 5,
            impact_per_power: 2,
            reference_impact: 10,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResolvedImpact {
    pub available: u16,
    pub used: u16,
    pub bonus: i32,
    pub raw_physical_damage: u16,
}

impl ResolvedImpact {
    pub fn is_material_limited(self) -> bool {
        self.used < self.available
    }
}

/// Configurable contribution of Resilience to a body's maximum hit points.
///
/// The body base, equipment bonus and state modifier remain explicit inputs so
/// they cannot be counted a second time by this module.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HitPointRules {
    pub neutral_attribute: u8,
    pub hit_points_per_resilience: u16,
    pub minimum_maximum_hit_points: u16,
}

impl HitPointRules {
    pub fn validate(self) -> Result<(), PhysicalRulesError> {
        if self.neutral_attribute == 0 {
            return Err(PhysicalRulesError::ZeroNeutralAttribute);
        }
        if self.minimum_maximum_hit_points == 0 {
            return Err(PhysicalRulesError::ZeroMinimumMaximumHitPoints);
        }
        Ok(())
    }

    pub fn maximum_hit_points(
        self,
        body_base: u16,
        attributes: Option<PrimaryAttributes>,
        material_bonus: i16,
        state_modifier: i16,
    ) -> u16 {
        let resilience = attributes.map_or(self.neutral_attribute, |values| {
            values.value(PrimaryAttribute::Resilience)
        });
        let resilience_delta = i32::from(resilience) - i32::from(self.neutral_attribute);
        let raw = i32::from(body_base)
            + resilience_delta * i32::from(self.hit_points_per_resilience)
            + i32::from(material_bonus)
            + i32::from(state_modifier);
        raw.clamp(
            i32::from(self.minimum_maximum_hit_points),
            i32::from(u16::MAX),
        ) as u16
    }

    pub fn maximum_for_body(
        self,
        body: BodyProfile,
        attributes: Option<PrimaryAttributes>,
        state_modifier: i16,
    ) -> u16 {
        self.maximum_hit_points(
            body.base_hit_points,
            attributes,
            body.material_bonus,
            state_modifier,
        )
    }

    /// Applies a maximum change without healing or resurrecting the actor.
    pub fn retained_current_hit_points(current: u16, new_maximum: u16) -> u16 {
        current.min(new_maximum)
    }
}

/// Authored, persistent properties of one body before Resilience and states.
#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BodyProfile {
    pub base_hit_points: u16,
    pub material_bonus: i16,
    pub base_armor: u16,
    pub displacement: Option<DisplacementProfile>,
    pub locomotion: Option<LocomotionProfile>,
    pub suppression_compatible: bool,
}

// Zero Armor is omitted so rules and actor fingerprints from versions 33/34
// retain the exact BodyProfile representation they were created with.
impl Debug for BodyProfile {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut profile = formatter.debug_struct("BodyProfile");
        profile
            .field("base_hit_points", &self.base_hit_points)
            .field("material_bonus", &self.material_bonus);
        if self.base_armor > 0 {
            profile.field("base_armor", &self.base_armor);
        }
        if let Some(displacement) = self.displacement {
            profile.field("displacement", &displacement);
        }
        if let Some(locomotion) = self.locomotion {
            profile.field("locomotion", &locomotion);
        }
        if self.suppression_compatible {
            profile.field("suppression_compatible", &true);
        }
        profile.finish()
    }
}

impl BodyProfile {
    pub fn new(base_hit_points: u16, material_bonus: i16) -> Result<Self, PhysicalRulesError> {
        if base_hit_points == 0 {
            return Err(PhysicalRulesError::ZeroBodyBaseHitPoints);
        }
        Ok(Self {
            base_hit_points,
            material_bonus,
            base_armor: 0,
            displacement: None,
            locomotion: None,
            suppression_compatible: false,
        })
    }

    pub const fn with_base_armor(mut self, base_armor: u16) -> Self {
        self.base_armor = base_armor;
        self
    }

    pub const fn without_base_armor(mut self) -> Self {
        self.base_armor = 0;
        self
    }

    pub const fn with_displacement_profile(mut self, profile: DisplacementProfile) -> Self {
        self.displacement = Some(profile);
        self
    }

    pub const fn displacement_profile(self) -> Option<DisplacementProfile> {
        self.displacement
    }

    pub const fn with_locomotion_profile(mut self, profile: LocomotionProfile) -> Self {
        self.locomotion = Some(profile);
        self
    }

    pub const fn locomotion_profile(self) -> Option<LocomotionProfile> {
        self.locomotion
    }

    pub const fn with_suppression_compatibility(mut self, compatible: bool) -> Self {
        self.suppression_compatible = compatible;
        self
    }

    pub const fn is_suppression_compatible(self) -> bool {
        self.suppression_compatible
    }

    /// Removes the body capabilities introduced with the first integrated
    /// melee discipline while retaining older hit-point and Armor metadata.
    pub const fn without_melee_skill_metadata(mut self) -> Self {
        self.displacement = None;
        self.locomotion = None;
        self
    }

    pub const fn without_ranged_skill_metadata(mut self) -> Self {
        self.suppression_compatible = false;
        self
    }
}

/// Authored capability targeted by locomotor perturbations. Keeping this on
/// the body avoids guessing from glyphs, factions or AI behavior.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LocomotionProfile {
    hindrance_compatible: bool,
}

impl LocomotionProfile {
    pub const fn new(hindrance_compatible: bool) -> Self {
        Self {
            hindrance_compatible,
        }
    }

    pub const fn hindrance_compatible(self) -> bool {
        self.hindrance_compatible
    }
}

/// Authored physical resistance to an external displacement.
///
/// `mass_grams` is the body's own mass. Carried mass is supplied separately by
/// the actor-inventory adapter so it is counted once, independently from
/// anchoring and without changing the resistance formula.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DisplacementProfile {
    mass_grams: u32,
    anchoring: u16,
    fixed: bool,
}

impl DisplacementProfile {
    pub fn new(mass_grams: u32, anchoring: u16) -> Result<Self, PhysicalRulesError> {
        if mass_grams == 0 {
            return Err(PhysicalRulesError::ZeroDisplacementMass);
        }
        Ok(Self {
            mass_grams,
            anchoring,
            fixed: false,
        })
    }

    pub const fn fixed(mut self) -> Self {
        self.fixed = true;
        self
    }

    pub const fn mass_grams(self) -> u32 {
        self.mass_grams
    }

    pub const fn anchoring(self) -> u16 {
        self.anchoring
    }

    pub const fn is_fixed(self) -> bool {
        self.fixed
    }

    pub fn resistance(self, carried_mass_grams: u64) -> u32 {
        let total_mass = u64::from(self.mass_grams).saturating_add(carried_mass_grams);
        let mass_resistance = total_mass
            .div_ceil(u64::from(GRAMS_PER_DISPLACEMENT_RESISTANCE))
            .min(u64::from(u32::MAX)) as u32;
        mass_resistance.saturating_add(u32::from(self.anchoring))
    }
}

impl Default for HitPointRules {
    fn default() -> Self {
        Self {
            neutral_attribute: 5,
            hit_points_per_resilience: 5,
            minimum_maximum_hit_points: 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhysicalRulesError {
    ZeroNeutralAttribute,
    ZeroMinimumMaximumHitPoints,
    ZeroBodyBaseHitPoints,
    ZeroDisplacementMass,
    DisplacementPropertiesWithoutMass,
}

impl Display for PhysicalRulesError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroNeutralAttribute => {
                write!(
                    formatter,
                    "neutral physical-rule attribute must be positive"
                )
            }
            Self::ZeroMinimumMaximumHitPoints => {
                write!(formatter, "minimum maximum hit points must be positive")
            }
            Self::ZeroBodyBaseHitPoints => {
                write!(formatter, "body base hit points must be positive")
            }
            Self::ZeroDisplacementMass => {
                write!(formatter, "displacement mass must be positive")
            }
            Self::DisplacementPropertiesWithoutMass => write!(
                formatter,
                "anchoring or fixed displacement requires an explicit body mass"
            ),
        }
    }
}

impl Error for PhysicalRulesError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn with_power(power: u8) -> Option<PrimaryAttributes> {
        Some(PrimaryAttributes::new(power, 5, 5, 5, 5))
    }

    fn with_resilience(resilience: u8) -> Option<PrimaryAttributes> {
        Some(PrimaryAttributes::new(5, 5, resilience, 5, 5))
    }

    #[test]
    fn documented_melee_impact_examples_are_exact() {
        let rules = ImpactRules::default();
        let cases = [
            (3, 20, 6, 6, -4, 8),
            (5, 20, 10, 10, 0, 12),
            (8, 20, 16, 16, 6, 18),
            (10, 20, 20, 20, 10, 22),
            (8, 14, 16, 14, 4, 16),
        ];

        for (power, cap, available, used, bonus, damage) in cases {
            let result = rules.resolve_melee_damage(with_power(power), 0, cap, 12);
            assert_eq!(result.available, available);
            assert_eq!(result.used, used);
            assert_eq!(result.bonus, bonus);
            assert_eq!(result.raw_physical_damage, damage);
        }
    }

    #[test]
    fn impact_reports_a_known_material_limit() {
        let result = ImpactRules::default().resolve_melee_damage(with_power(8), 0, 14, 12);

        assert!(result.is_material_limited());
        assert_eq!(result.available, 16);
        assert_eq!(result.used, 14);
    }

    #[test]
    fn missing_attributes_use_the_explicit_neutral_profile() {
        let impact = ImpactRules::default().resolve_melee_damage(None, 0, 20, 12);
        let maximum = HitPointRules::default().maximum_hit_points(100, None, 0, 0);

        assert_eq!(impact.available, 10);
        assert_eq!(impact.raw_physical_damage, 12);
        assert_eq!(maximum, 100);
    }

    #[test]
    fn documented_maximum_hit_point_examples_are_exact() {
        let rules = HitPointRules::default();
        let cases = [
            (3, 0, 0, 90),
            (5, 0, 0, 100),
            (8, 0, 0, 115),
            (10, 0, 0, 125),
            (5, 20, 0, 120),
        ];

        for (resilience, material_bonus, state_modifier, expected) in cases {
            assert_eq!(
                rules.maximum_hit_points(
                    100,
                    with_resilience(resilience),
                    material_bonus,
                    state_modifier,
                ),
                expected
            );
        }
    }

    #[test]
    fn body_profile_keeps_base_and_material_contributions_explicit() {
        let body = BodyProfile::new(100, 20).unwrap();

        assert_eq!(
            HitPointRules::default().maximum_for_body(body, with_resilience(8), -5),
            130
        );
        assert_eq!(
            BodyProfile::new(0, 0),
            Err(PhysicalRulesError::ZeroBodyBaseHitPoints)
        );
    }

    #[test]
    fn displacement_resistance_uses_ceil_mass_then_adds_anchoring() {
        let free = DisplacementProfile::new(80_000, 0).unwrap();
        let anchored = DisplacementProfile::new(80_000, 10).unwrap();

        assert_eq!(free.resistance(0), 8);
        assert_eq!(free.resistance(1), 9);
        assert_eq!(
            DisplacementProfile::new(120_000, 0).unwrap().resistance(0),
            12
        );
        assert_eq!(anchored.resistance(0), 18);
        assert!(anchored.fixed().is_fixed());
        assert_eq!(
            DisplacementProfile::new(0, 0),
            Err(PhysicalRulesError::ZeroDisplacementMass)
        );
    }

    #[test]
    fn changing_the_maximum_never_heals_or_resurrects() {
        assert_eq!(HitPointRules::retained_current_hit_points(70, 120), 70);
        let after_reduction = HitPointRules::retained_current_hit_points(110, 100);
        assert_eq!(after_reduction, 100);
        assert_eq!(
            HitPointRules::retained_current_hit_points(after_reduction, 120),
            100
        );
        assert_eq!(HitPointRules::retained_current_hit_points(0, 120), 0);
    }

    #[test]
    fn physical_results_are_bounded_without_overflow() {
        let impact_rules = ImpactRules {
            impact_per_power: u16::MAX,
            ..ImpactRules::default()
        };
        let hit_point_rules = HitPointRules {
            hit_points_per_resilience: u16::MAX,
            ..HitPointRules::default()
        };

        assert_eq!(
            impact_rules.available_impact(with_power(u8::MAX), i16::MAX),
            u16::MAX
        );
        assert_eq!(
            hit_point_rules.maximum_hit_points(
                u16::MAX,
                with_resilience(u8::MAX),
                i16::MAX,
                i16::MAX,
            ),
            u16::MAX
        );
        assert_eq!(
            HitPointRules::default().maximum_hit_points(1, with_resilience(1), -100, -100),
            1
        );
    }

    #[test]
    fn invalid_physical_rule_minima_are_rejected() {
        assert_eq!(
            ImpactRules {
                neutral_attribute: 0,
                ..ImpactRules::default()
            }
            .validate(),
            Err(PhysicalRulesError::ZeroNeutralAttribute)
        );
        assert_eq!(
            HitPointRules {
                minimum_maximum_hit_points: 0,
                ..HitPointRules::default()
            }
            .validate(),
            Err(PhysicalRulesError::ZeroMinimumMaximumHitPoints)
        );
    }
}
