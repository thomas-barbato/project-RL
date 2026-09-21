use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::combat::{DamageImpact, DamagePacket, DamageType};
use crate::time::TimeUnits;
use crate::world::{DistanceMetric, GridPos, Map, has_line_of_sight};

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PreparationDisruptionFamily {
    SystemShock,
}

/// Explicit functional perturbation carried by a successful attack. Damage
/// never implies disruption on its own; content must opt into a named family
/// and intensity so protections and future countermeasures remain moddable.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PreparationDisruption {
    family: PreparationDisruptionFamily,
    intensity: u16,
}

impl PreparationDisruption {
    pub fn new(
        family: PreparationDisruptionFamily,
        intensity: u16,
    ) -> Result<Self, AttackImpactError> {
        if intensity == 0 {
            return Err(AttackImpactError::ZeroDisruptionIntensity);
        }
        Ok(Self { family, intensity })
    }

    pub const fn family(self) -> PreparationDisruptionFamily {
        self.family
    }

    pub const fn intensity(self) -> u16 {
        self.intensity
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ConeAttack {
    narrow_length: u16,
    maximum_half_width: u16,
    widen_every: u16,
}

impl ConeAttack {
    pub const fn new(
        narrow_length: u16,
        maximum_half_width: u16,
        widen_every: u16,
    ) -> Result<Self, ConeAttackError> {
        if maximum_half_width == 0 {
            return Err(ConeAttackError::ZeroMaximumHalfWidth);
        }
        if widen_every == 0 {
            return Err(ConeAttackError::ZeroWidenEvery);
        }
        Ok(Self {
            narrow_length,
            maximum_half_width,
            widen_every,
        })
    }

    pub const fn narrow_length(self) -> u16 {
        self.narrow_length
    }

    pub const fn maximum_half_width(self) -> u16 {
        self.maximum_half_width
    }

    pub const fn widen_every(self) -> u16 {
        self.widen_every
    }
}

fn cone_half_width(cone: ConeAttack, step: u16) -> u16 {
    if step <= cone.narrow_length {
        return 0;
    }

    // `step` is strictly greater than `narrow_length`, and `widen_every`
    // cannot be zero after construction. The addition cannot overflow u16.
    let widened = 1 + (step - cone.narrow_length - 1) / cone.widen_every;
    if widened < cone.maximum_half_width {
        widened
    } else {
        cone.maximum_half_width
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConeAttackError {
    ZeroMaximumHalfWidth,
    ZeroWidenEvery,
}

impl Display for ConeAttackError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroMaximumHalfWidth => {
                write!(formatter, "cone maximum half width must be positive")
            }
            Self::ZeroWidenEvery => write!(formatter, "cone widening interval must be positive"),
        }
    }
}

impl Error for ConeAttackError {}

/// A contiguous slice of the eight cells touching an attacker, centred on the
/// selected adjacent cell. Missing or blocked cells are not replaced by cells
/// elsewhere in the ring.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MeleeArc {
    maximum_cells: u8,
}

impl MeleeArc {
    pub const fn new(maximum_cells: u8) -> Result<Self, MeleeArcError> {
        if maximum_cells == 0 {
            return Err(MeleeArcError::ZeroMaximumCells);
        }
        if maximum_cells > 8 {
            return Err(MeleeArcError::TooManyCells(maximum_cells));
        }
        Ok(Self { maximum_cells })
    }

    pub const fn maximum_cells(self) -> u8 {
        self.maximum_cells
    }

    pub fn affected_cells(
        self,
        map: &Map,
        origin: GridPos,
        selected: GridPos,
    ) -> Vec<AttackAreaCell> {
        const RING: [(i32, i32); 8] = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
        ];
        let delta = (selected.x - origin.x, selected.y - origin.y);
        let Some(center) = RING.iter().position(|offset| *offset == delta) else {
            return Vec::new();
        };
        let mut indices = Vec::with_capacity(usize::from(self.maximum_cells));
        indices.push(center);
        for distance in 1..=4 {
            if indices.len() >= usize::from(self.maximum_cells) {
                break;
            }
            indices.push((center + RING.len() - distance) % RING.len());
            if indices.len() >= usize::from(self.maximum_cells) {
                break;
            }
            indices.push((center + distance) % RING.len());
        }
        indices
            .into_iter()
            .map(|index| {
                let (delta_x, delta_y) = RING[index];
                GridPos::new(origin.x + delta_x, origin.y + delta_y)
            })
            .filter(|position| map.is_walkable(*position))
            .map(|position| AttackAreaCell { position, step: 1 })
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeleeArcError {
    ZeroMaximumCells,
    TooManyCells(u8),
}

impl Display for MeleeArcError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroMaximumCells => write!(formatter, "melee arc must cover at least one cell"),
            Self::TooManyCells(cells) => {
                write!(
                    formatter,
                    "melee arc cannot cover more than eight cells, found {cells}"
                )
            }
        }
    }
}

impl Error for MeleeArcError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AttackArea {
    Single,
    Cone(ConeAttack),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AttackDelivery {
    Melee,
    Ranged,
}

/// Authored material limits for an attack whose physical component benefits
/// from the attacker's Power.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MeleeImpactProfile {
    pub material_cap: u16,
    pub impact_modifier: i16,
}

impl MeleeImpactProfile {
    pub const fn new(material_cap: u16, impact_modifier: i16) -> Self {
        Self {
            material_cap,
            impact_modifier,
        }
    }
}

impl AttackDelivery {
    const fn inferred_from_range(range: u16) -> Self {
        if range <= 1 {
            Self::Melee
        } else {
            Self::Ranged
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttackAreaCell {
    pub position: GridPos,
    pub step: u16,
}

/// Read-only footprint returned before an area attack is committed. The
/// simulation produces it from the same rules used by execution, so a client
/// never has to imitate range, cone or obstacle logic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AttackPreview {
    origin: GridPos,
    target: GridPos,
    cells: Vec<AttackAreaCell>,
}

impl AttackPreview {
    pub(crate) const fn new(origin: GridPos, target: GridPos, cells: Vec<AttackAreaCell>) -> Self {
        Self {
            origin,
            target,
            cells,
        }
    }

    pub const fn origin(&self) -> GridPos {
        self.origin
    }

    pub const fn target(&self) -> GridPos {
        self.target
    }

    pub fn cells(&self) -> &[AttackAreaCell] {
        &self.cells
    }
}

/// A reusable attack description. Content definitions will construct these
/// profiles; combat resolution does not branch on weapon classes.
#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AttackProfile {
    range: u16,
    distance_metric: DistanceMetric,
    requires_line_of_sight: bool,
    damage: DamageImpact,
    area: AttackArea,
    delivery: AttackDelivery,
    accuracy_modifier: i16,
    melee_impact: Option<MeleeImpactProfile>,
    physical_damage_percentage: u16,
    damage_output_percentage: u16,
    recovery_after_attack: Option<TimeUnits>,
    preparation_disruption: Option<PreparationDisruption>,
}

// Keep the historical representation for ordinary single-target attacks so
// suspension versions created before attack areas remain replay-compatible.
impl Debug for AttackProfile {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut profile = formatter.debug_struct("AttackProfile");
        profile
            .field("range", &self.range)
            .field("distance_metric", &self.distance_metric)
            .field("requires_line_of_sight", &self.requires_line_of_sight)
            .field("damage", &self.damage);
        if self.area != AttackArea::Single {
            profile.field("area", &self.area);
        }
        if self.delivery != AttackDelivery::inferred_from_range(self.range) {
            profile.field("delivery", &self.delivery);
        }
        if self.accuracy_modifier != 0 {
            profile.field("accuracy_modifier", &self.accuracy_modifier);
        }
        if let Some(melee_impact) = self.melee_impact {
            profile.field("melee_impact", &melee_impact);
        }
        if self.physical_damage_percentage != 100 {
            profile.field(
                "physical_damage_percentage",
                &self.physical_damage_percentage,
            );
        }
        if self.damage_output_percentage != 100 {
            profile.field("damage_output_percentage", &self.damage_output_percentage);
        }
        if let Some(recovery) = self.recovery_after_attack {
            profile.field("recovery_after_attack", &recovery);
        }
        if let Some(disruption) = &self.preparation_disruption {
            profile.field("preparation_disruption", disruption);
        }
        profile.finish()
    }
}

impl AttackProfile {
    pub const fn new(
        range: u16,
        distance_metric: DistanceMetric,
        requires_line_of_sight: bool,
        damage_type: DamageType,
        damage_amount: u16,
        penetration: u16,
    ) -> Self {
        Self {
            range,
            distance_metric,
            requires_line_of_sight,
            damage: DamageImpact::single(DamagePacket::new(
                damage_amount,
                damage_type,
                penetration,
            )),
            area: AttackArea::Single,
            delivery: AttackDelivery::inferred_from_range(range),
            accuracy_modifier: 0,
            melee_impact: None,
            physical_damage_percentage: 100,
            damage_output_percentage: 100,
            recovery_after_attack: None,
            preparation_disruption: None,
        }
    }

    pub const fn melee(damage_type: DamageType, damage_amount: u16) -> Self {
        Self::new(
            1,
            DistanceMetric::Chebyshev,
            false,
            damage_type,
            damage_amount,
            0,
        )
    }

    pub const fn range(self) -> u16 {
        self.range
    }

    pub const fn requires_line_of_sight(self) -> bool {
        self.requires_line_of_sight
    }

    pub const fn damage(self) -> DamageImpact {
        self.damage
    }

    pub const fn with_damage(mut self, damage: DamageImpact) -> Self {
        self.damage = damage;
        self
    }

    /// Adds temporary physical Armor penetration without changing the
    /// weapon's permanent profile or any specialized resistance penetration.
    pub const fn with_additional_armor_penetration(mut self, amount: u16) -> Self {
        self.damage = self.damage.with_additional_armor_penetration(amount);
        self
    }

    pub const fn with_area(mut self, area: AttackArea) -> Self {
        self.area = area;
        self
    }

    pub const fn area(self) -> AttackArea {
        self.area
    }

    pub const fn with_delivery(mut self, delivery: AttackDelivery) -> Self {
        self.delivery = delivery;
        self
    }

    pub const fn delivery(self) -> AttackDelivery {
        self.delivery
    }

    pub const fn with_accuracy_modifier(mut self, modifier: i16) -> Self {
        self.accuracy_modifier = modifier;
        self
    }

    pub const fn accuracy_modifier(self) -> i16 {
        self.accuracy_modifier
    }

    pub fn with_melee_impact(
        mut self,
        profile: MeleeImpactProfile,
    ) -> Result<Self, AttackImpactError> {
        if self.delivery != AttackDelivery::Melee {
            return Err(AttackImpactError::RequiresMeleeDelivery);
        }
        if !self.damage.contains(DamageType::Kinetic) && !self.damage.contains(DamageType::Piercing)
        {
            return Err(AttackImpactError::RequiresPhysicalDamage);
        }
        self.melee_impact = Some(profile);
        Ok(self)
    }

    pub const fn melee_impact(self) -> Option<MeleeImpactProfile> {
        self.melee_impact
    }

    /// Scales the complete physical part after melee Impact has been resolved,
    /// but before reactions and Armor. Non-physical components are preserved.
    pub fn with_physical_damage_percentage(
        mut self,
        percentage: u16,
    ) -> Result<Self, AttackImpactError> {
        if percentage == 0 {
            return Err(AttackImpactError::ZeroPhysicalDamagePercentage);
        }
        if !self.damage.has_physical_component() {
            return Err(AttackImpactError::RequiresPhysicalDamage);
        }
        self.physical_damage_percentage = percentage;
        Ok(self)
    }

    pub const fn physical_damage_percentage(self) -> u16 {
        self.physical_damage_percentage
    }

    /// Applies a temporary output multiplier after the attack's ordinary
    /// physical resolution. Engineering uses this for concrete tuned modules;
    /// content still owns the percentages.
    pub const fn with_damage_output_percentage(mut self, percentage: u16) -> Self {
        self.damage_output_percentage = percentage;
        self
    }

    pub const fn damage_output_percentage(self) -> u16 {
        self.damage_output_percentage
    }

    /// Declares Rn independently from hit resolution. Once the attack is
    /// committed, this recovery starts even when every target evades it.
    pub const fn with_recovery_after_attack(mut self, duration: TimeUnits) -> Self {
        self.recovery_after_attack = Some(duration);
        self
    }

    pub const fn recovery_after_attack(self) -> Option<TimeUnits> {
        self.recovery_after_attack
    }

    pub fn with_preparation_disruption(mut self, disruption: PreparationDisruption) -> Self {
        self.preparation_disruption = Some(disruption);
        self
    }

    pub const fn preparation_disruption(self) -> Option<PreparationDisruption> {
        self.preparation_disruption
    }

    pub fn without_preparation_disruption(mut self) -> Self {
        self.preparation_disruption = None;
        self
    }

    /// Compatibility helper for rulesets authored before recovery metadata.
    pub const fn without_recovery_after_attack(mut self) -> Self {
        self.recovery_after_attack = None;
        self
    }

    /// Compatibility helper for replaying rulesets authored before Impact.
    pub const fn without_melee_impact(mut self) -> Self {
        self.melee_impact = None;
        self
    }

    pub fn is_in_range(self, origin: GridPos, target: GridPos) -> bool {
        let delta_x = (i64::from(target.x) - i64::from(origin.x)).abs();
        let delta_y = (i64::from(target.y) - i64::from(origin.y)).abs();
        let range = i64::from(self.range);

        match self.distance_metric {
            DistanceMetric::Chebyshev => delta_x.max(delta_y) <= range,
            DistanceMetric::Euclidean => delta_x * delta_x + delta_y * delta_y <= range * range,
        }
    }

    /// Resolves an attack footprint from simulation coordinates only. Cone
    /// centre lines follow the exact aimed vector; they are not inferred from
    /// a renderer glyph or reduced to four directions.
    pub fn affected_cells(
        self,
        map: &Map,
        origin: GridPos,
        target: GridPos,
    ) -> Vec<AttackAreaCell> {
        match self.area {
            AttackArea::Single => vec![AttackAreaCell {
                position: target,
                step: grid_step(origin, target),
            }],
            AttackArea::Cone(cone) => cone_cells(map, origin, target, self.range, cone),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttackImpactError {
    RequiresMeleeDelivery,
    RequiresPhysicalDamage,
    ZeroPhysicalDamagePercentage,
    ZeroDisruptionIntensity,
}

impl Display for AttackImpactError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequiresMeleeDelivery => {
                write!(formatter, "an impact profile requires melee delivery")
            }
            Self::RequiresPhysicalDamage => write!(
                formatter,
                "an impact profile requires kinetic or piercing damage"
            ),
            Self::ZeroPhysicalDamagePercentage => {
                write!(formatter, "physical damage percentage must be positive")
            }
            Self::ZeroDisruptionIntensity => {
                write!(
                    formatter,
                    "preparation disruption intensity must be positive"
                )
            }
        }
    }
}

impl Error for AttackImpactError {}

fn cone_cells(
    map: &Map,
    origin: GridPos,
    target: GridPos,
    range: u16,
    cone: ConeAttack,
) -> Vec<AttackAreaCell> {
    let delta_x = i64::from(target.x) - i64::from(origin.x);
    let delta_y = i64::from(target.y) - i64::from(origin.y);
    let aim_length = delta_x.abs().max(delta_y.abs());
    if aim_length == 0 {
        return Vec::new();
    }
    let lateral_is_y = delta_x.abs() >= delta_y.abs();
    let mut cells = BTreeMap::<GridPos, u16>::new();
    for step in 1..=range {
        let step = i64::from(step);
        let center = GridPos::new(
            origin
                .x
                .saturating_add(round_ratio(delta_x * step, aim_length)),
            origin
                .y
                .saturating_add(round_ratio(delta_y * step, aim_length)),
        );
        let half_width = i32::from(cone_half_width(cone, step as u16));
        for lateral in -half_width..=half_width {
            let position = if lateral_is_y {
                GridPos::new(center.x, center.y.saturating_add(lateral))
            } else {
                GridPos::new(center.x.saturating_add(lateral), center.y)
            };
            if !map.is_walkable(position) || !has_line_of_sight(map, origin, position, true) {
                continue;
            }
            let current_step = step as u16 - 1;
            cells
                .entry(position)
                .and_modify(|known_step| {
                    if current_step < *known_step {
                        *known_step = current_step;
                    }
                })
                .or_insert(current_step);
        }
    }
    cells
        .into_iter()
        .map(|(position, step)| AttackAreaCell { position, step })
        .collect()
}

fn round_ratio(numerator: i64, denominator: i64) -> i32 {
    let rounded = if numerator >= 0 {
        (numerator + denominator / 2) / denominator
    } else {
        (numerator - denominator / 2) / denominator
    };
    i32::try_from(rounded).unwrap_or_else(|_| {
        if rounded.is_negative() {
            i32::MIN
        } else {
            i32::MAX
        }
    })
}

fn grid_step(origin: GridPos, target: GridPos) -> u16 {
    let distance = (i64::from(target.x) - i64::from(origin.x))
        .abs()
        .max((i64::from(target.y) - i64::from(origin.y)).abs());
    u16::try_from(distance).unwrap_or(u16::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cone_profile() -> AttackProfile {
        AttackProfile::new(
            7,
            DistanceMetric::Euclidean,
            true,
            DamageType::Thermal,
            4,
            0,
        )
        .with_area(AttackArea::Cone(ConeAttack::new(2, 2, 3).unwrap()))
    }

    #[test]
    fn melee_arc_is_centered_on_the_selected_neighbor_and_never_replaces_a_wall() {
        let map = Map::from_ascii("#####\n#...#\n#...#\n#..##\n#####").unwrap();
        let origin = GridPos::new(2, 2);
        let arc = MeleeArc::new(3).unwrap();

        assert_eq!(
            arc.affected_cells(&map, origin, GridPos::new(3, 2)),
            vec![
                AttackAreaCell {
                    position: GridPos::new(3, 2),
                    step: 1,
                },
                AttackAreaCell {
                    position: GridPos::new(3, 1),
                    step: 1,
                },
            ]
        );
        assert!(
            arc.affected_cells(&map, origin, GridPos::new(4, 2))
                .is_empty()
        );
        assert_eq!(MeleeArc::new(0), Err(MeleeArcError::ZeroMaximumCells));
        assert_eq!(MeleeArc::new(9), Err(MeleeArcError::TooManyCells(9)));
    }

    #[test]
    fn attack_recovery_is_optional_and_keeps_legacy_debug_shape_when_absent() {
        let ordinary = AttackProfile::melee(DamageType::Kinetic, 3);
        assert_eq!(ordinary.recovery_after_attack(), None);
        assert!(!format!("{ordinary:?}").contains("recovery_after_attack"));

        let recovering = ordinary.with_recovery_after_attack(TimeUnits::ONE);
        assert_eq!(recovering.recovery_after_attack(), Some(TimeUnits::ONE));
        assert!(format!("{recovering:?}").contains("recovery_after_attack"));
        assert_eq!(recovering.without_recovery_after_attack(), ordinary);
    }

    #[test]
    fn physical_damage_scale_is_explicit_and_requires_a_physical_component() {
        let ordinary = AttackProfile::melee(DamageType::Kinetic, 3);
        assert_eq!(ordinary.physical_damage_percentage(), 100);
        assert!(!format!("{ordinary:?}").contains("physical_damage_percentage"));

        let empowered = ordinary.with_physical_damage_percentage(150).unwrap();
        assert_eq!(empowered.physical_damage_percentage(), 150);
        assert!(format!("{empowered:?}").contains("physical_damage_percentage"));
        assert_eq!(
            ordinary.with_physical_damage_percentage(0),
            Err(AttackImpactError::ZeroPhysicalDamagePercentage)
        );
        assert_eq!(
            AttackProfile::melee(DamageType::Thermal, 3).with_physical_damage_percentage(150),
            Err(AttackImpactError::RequiresPhysicalDamage)
        );
    }

    #[test]
    fn long_cone_stays_narrow_before_widening_at_the_tip() {
        let map = Map::from_ascii(&format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}",
            "###########",
            "#.........#",
            "#.........#",
            "#.........#",
            "#.........#",
            "#.........#",
            "###########"
        ))
        .unwrap();
        let cells = cone_profile().affected_cells(&map, GridPos::new(1, 3), GridPos::new(6, 3));
        let width_at = |x| cells.iter().filter(|cell| cell.position.x == x).count();

        assert_eq!(width_at(2), 1);
        assert_eq!(width_at(3), 1);
        assert_eq!(width_at(4), 3);
        assert_eq!(width_at(6), 3);
        assert_eq!(width_at(7), 5);
        assert_eq!(width_at(8), 5);
    }

    #[test]
    fn cone_follows_an_oblique_aim_and_never_crosses_a_wall() {
        let map = Map::from_ascii(
            "##########\n#...#....#\n#...#....#\n#........#\n#........#\n#........#\n##########",
        )
        .unwrap();
        let origin = GridPos::new(1, 4);
        let target = GridPos::new(5, 3);
        let cells = cone_profile().affected_cells(&map, origin, target);

        assert!(cells.iter().any(|cell| cell.position == target));
        assert!(cells.iter().all(|cell| {
            map.is_walkable(cell.position) && has_line_of_sight(&map, origin, cell.position, true)
        }));
        assert!(!cells.iter().any(|cell| cell.position == GridPos::new(5, 1)));
    }

    #[test]
    fn attack_delivery_is_inferred_but_can_be_overridden_by_content() {
        let melee = AttackProfile::melee(DamageType::Kinetic, 3);
        let ranged = AttackProfile::new(
            4,
            DistanceMetric::Euclidean,
            true,
            DamageType::Piercing,
            3,
            0,
        );

        assert_eq!(melee.delivery(), AttackDelivery::Melee);
        assert_eq!(ranged.delivery(), AttackDelivery::Ranged);
        assert_eq!(
            ranged.with_delivery(AttackDelivery::Melee).delivery(),
            AttackDelivery::Melee
        );
    }

    #[test]
    fn preparation_disruption_is_explicit_validated_attack_metadata() {
        assert_eq!(
            PreparationDisruption::new(PreparationDisruptionFamily::SystemShock, 0),
            Err(AttackImpactError::ZeroDisruptionIntensity)
        );
        let disruption =
            PreparationDisruption::new(PreparationDisruptionFamily::SystemShock, 55).unwrap();
        let attack =
            AttackProfile::melee(DamageType::Kinetic, 3).with_preparation_disruption(disruption);

        assert_eq!(attack.preparation_disruption(), Some(disruption));
        assert_eq!(
            attack
                .without_preparation_disruption()
                .preparation_disruption(),
            None
        );
    }

    #[test]
    fn impact_profiles_require_an_explicitly_physical_melee_attack() {
        let impact = MeleeImpactProfile::new(14, 0);

        assert!(
            AttackProfile::melee(DamageType::Kinetic, 3)
                .with_melee_impact(impact)
                .is_ok()
        );
        assert_eq!(
            AttackProfile::new(
                4,
                DistanceMetric::Euclidean,
                true,
                DamageType::Piercing,
                3,
                0,
            )
            .with_melee_impact(impact),
            Err(AttackImpactError::RequiresMeleeDelivery)
        );
        assert_eq!(
            AttackProfile::melee(DamageType::Thermal, 3).with_melee_impact(impact),
            Err(AttackImpactError::RequiresPhysicalDamage)
        );
    }
}
