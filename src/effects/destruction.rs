use crate::effects::{GroundEffectSpec, RadialDamageEffect};
use crate::world::{GridPos, Map};

/// Data-shaped consequences released when an entity is destroyed.
///
/// Keeping the explosion and the optional persistent hazard together lets
/// mods define volatile scenery without teaching the game loop about each
/// individual barrel, reactor or creature type.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::{DamagePacket, DamageType};
    use crate::effects::DamageFalloff;
    use crate::world::{NeighborMode, Terrain, TerrainPropagationPolicy};

    #[test]
    fn persistent_discharge_reuses_the_water_conductive_footprint() {
        let mut map = Map::filled(9, 5, Terrain::Wall).unwrap();
        for x in 1..=7 {
            map.set_terrain(GridPos::new(x, 2), Terrain::Floor).unwrap();
            map.set_terrain(GridPos::new(x, 1), Terrain::ShallowWater)
                .unwrap();
        }
        let field = GroundEffectSpec::new(
            "core:test_electrified_ground".parse().unwrap(),
            2,
            DamagePacket::new(2, DamageType::Electrical, 0),
        )
        .unwrap();
        let effect = DestructionEffect::new(RadialDamageEffect {
            maximum_cost: 4,
            neighbor_mode: NeighborMode::Cardinal,
            propagation_policy: TerrainPropagationPolicy::conductive(3, 1),
            damage: DamagePacket::new(5, DamageType::Electrical, 0),
            falloff: DamageFalloff::PerPropagationCost(1),
        })
        .with_ground_effect(field);
        let positions = effect.ground_effect_positions(&map, GridPos::new(1, 2));

        assert!(positions.contains(&GridPos::new(4, 1)));
        assert!(!positions.contains(&GridPos::new(3, 2)));
    }
}
