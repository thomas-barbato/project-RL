use crate::world::{DistanceMetric, GridPos};

use super::{ApplyStatusEffect, RadialDamageEffect};

/// A data-shaped ability assembled from reusable effect primitives.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AbilityProfile {
    range: u16,
    distance_metric: DistanceMetric,
    requires_line_of_sight: bool,
    requires_walkable_target: bool,
    effects: Vec<EffectPrimitive>,
}

impl AbilityProfile {
    pub fn new(
        range: u16,
        distance_metric: DistanceMetric,
        requires_line_of_sight: bool,
        requires_walkable_target: bool,
        effects: Vec<EffectPrimitive>,
    ) -> Self {
        Self {
            range,
            distance_metric,
            requires_line_of_sight,
            requires_walkable_target,
            effects,
        }
    }

    pub const fn requires_line_of_sight(&self) -> bool {
        self.requires_line_of_sight
    }

    pub const fn requires_walkable_target(&self) -> bool {
        self.requires_walkable_target
    }

    pub fn effects(&self) -> &[EffectPrimitive] {
        &self.effects
    }

    pub fn is_in_range(&self, origin: GridPos, target: GridPos) -> bool {
        let delta_x = (i64::from(target.x) - i64::from(origin.x)).abs();
        let delta_y = (i64::from(target.y) - i64::from(origin.y)).abs();
        let range = i64::from(self.range);

        match self.distance_metric {
            DistanceMetric::Chebyshev => delta_x.max(delta_y) <= range,
            DistanceMetric::Euclidean => delta_x * delta_x + delta_y * delta_y <= range * range,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EffectPrimitive {
    RadialDamage(RadialDamageEffect),
    ApplyStatus(ApplyStatusEffect),
}
