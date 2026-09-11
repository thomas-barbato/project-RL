use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::combat::{DamagePacket, DamageType};
use crate::world::{DistanceMetric, GridPos, Map, has_line_of_sight};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttackArea {
    Single,
    Cone(ConeAttack),
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
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AttackProfile {
    range: u16,
    distance_metric: DistanceMetric,
    requires_line_of_sight: bool,
    damage: DamagePacket,
    area: AttackArea,
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
            damage: DamagePacket::new(damage_amount, damage_type, penetration),
            area: AttackArea::Single,
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

    pub const fn damage(self) -> DamagePacket {
        self.damage
    }

    pub const fn with_area(mut self, area: AttackArea) -> Self {
        self.area = area;
        self
    }

    pub const fn area(self) -> AttackArea {
        self.area
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
}
