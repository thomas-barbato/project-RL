use crate::effects::{GroundEffectSpec, RadialDamageEffect};
use crate::world::{GridPos, Map};

/// Data-shaped consequences released when an entity is destroyed.
///
/// Keeping the explosion and the optional persistent hazard together lets
/// mods define volatile scenery without teaching the game loop about each
/// individual barrel, reactor or creature type.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DestructionEffect {
    explosion: RadialDamageEffect,
    ground_effect: Option<GroundEffectSpec>,
}

impl DestructionEffect {
    pub const fn new(explosion: RadialDamageEffect) -> Self {
        Self {
            explosion,
            ground_effect: None,
        }
    }

    pub fn with_ground_effect(mut self, effect: GroundEffectSpec) -> Self {
        self.ground_effect = Some(effect);
        self
    }

    pub const fn explosion(&self) -> &RadialDamageEffect {
        &self.explosion
    }

    pub const fn ground_effect(&self) -> Option<&GroundEffectSpec> {
        self.ground_effect.as_ref()
    }

    /// Reuses the explosion's wall-aware footprint. With the standard
    /// eight-neighbor policy this is the project's documented Chebyshev disk,
    /// so explosion and resulting hazard cannot disagree about cover.
    pub fn ground_effect_positions(&self, map: &Map, origin: GridPos) -> Vec<GridPos> {
        if self.ground_effect.is_none() {
            return Vec::new();
        }
        self.explosion
            .affected_cells(map, origin)
            .into_iter()
            .map(|cell| cell.position)
            .collect()
    }
}
