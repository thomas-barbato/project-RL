use crate::combat::DamagePacket;
use crate::world::{
    GridPos, Map, NeighborMode, PropagationCell, PropagationRequest, TerrainPropagationPolicy,
    propagate,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DamageFalloff {
    None,
    PerPropagationCost(u16),
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RadialDamageEffect {
    pub maximum_cost: u16,
    pub neighbor_mode: NeighborMode,
    pub propagation_policy: TerrainPropagationPolicy,
    pub damage: DamagePacket,
    pub falloff: DamageFalloff,
}

impl RadialDamageEffect {
    pub fn affected_cells(&self, map: &Map, origin: GridPos) -> Vec<PropagationCell> {
        propagate(
            map,
            PropagationRequest {
                origin,
                maximum_cost: self.maximum_cost,
                neighbor_mode: self.neighbor_mode,
            },
            &self.propagation_policy,
        )
    }

    pub fn damage_at_cost(&self, cost: u16) -> Option<DamagePacket> {
        let reduction = match self.falloff {
            DamageFalloff::None => 0,
            DamageFalloff::PerPropagationCost(per_cost) => per_cost.saturating_mul(cost),
        };
        let amount = self.damage.amount.saturating_sub(reduction);
        (amount > 0).then_some(DamagePacket {
            amount,
            ..self.damage
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::DamageType;

    #[test]
    fn falloff_reduces_damage_by_propagation_cost() {
        let effect = RadialDamageEffect {
            maximum_cost: 3,
            neighbor_mode: NeighborMode::Cardinal,
            propagation_policy: TerrainPropagationPolicy::blocked_by_walls(1),
            damage: DamagePacket::new(7, DamageType::Explosive, 0),
            falloff: DamageFalloff::PerPropagationCost(2),
        };

        assert_eq!(
            effect.damage_at_cost(0).map(|packet| packet.amount),
            Some(7)
        );
        assert_eq!(
            effect.damage_at_cost(2).map(|packet| packet.amount),
            Some(3)
        );
        assert_eq!(effect.damage_at_cost(4), None);
    }
}
