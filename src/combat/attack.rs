use crate::combat::{DamagePacket, DamageType};
use crate::world::{DistanceMetric, GridPos};

/// A reusable attack description. Content definitions will construct these
/// profiles; combat resolution does not branch on weapon classes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttackProfile {
    range: u16,
    distance_metric: DistanceMetric,
    requires_line_of_sight: bool,
    damage: DamagePacket,
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

    pub fn is_in_range(self, origin: GridPos, target: GridPos) -> bool {
        let delta_x = (i64::from(target.x) - i64::from(origin.x)).abs();
        let delta_y = (i64::from(target.y) - i64::from(origin.y)).abs();
        let range = i64::from(self.range);

        match self.distance_metric {
            DistanceMetric::Chebyshev => delta_x.max(delta_y) <= range,
            DistanceMetric::Euclidean => delta_x * delta_x + delta_y * delta_y <= range * range,
        }
    }
}
