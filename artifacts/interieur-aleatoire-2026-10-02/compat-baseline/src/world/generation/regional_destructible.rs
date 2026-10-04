use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::content::RegionDestructibleProfile;
use crate::entity::Actor;
use crate::game::GameRng;
use crate::world::{Direction, GridPos, Map};

const DESTRUCTIBLE_SEED_SALT: u64 = 0x4445_5354_5255_4354;

/// Places stationary destructible entities through their own deterministic
/// stream. They deliberately remain ordinary actors in the simulation so all
/// attacks, damage types, resistances, deaths and chain reactions share the
/// same resolution path as other entities.
pub fn generate_regional_destructibles(
    map: &Map,
    passages: &[GridPos],
    reserved: &BTreeSet<GridPos>,
    profile: &RegionDestructibleProfile,
    region_seed: u64,
) -> Result<Vec<Actor>, RegionalDestructibleError> {
    let mut rng = GameRng::from_seed(region_seed ^ DESTRUCTIBLE_SEED_SALT);
    let (minimum, maximum) = profile.count_range();
    let count = inclusive_u16(&mut rng, minimum, maximum);
    let mut candidates = destructible_spawn_cells(map, passages, reserved, profile);
    if candidates.len() < usize::from(count) {
        return Err(RegionalDestructibleError::InsufficientSpawnSpace {
            requested: count,
            available: candidates.len(),
            minimum_passage_distance: profile.minimum_passage_distance(),
        });
    }

    let mut actors = Vec::with_capacity(usize::from(count));
    for _ in 0..count {
        let index = below(&mut rng, candidates.len() as u64) as usize;
        let position = candidates.swap_remove(index);
        actors.push(
            Actor::new(position, profile.maximum_integrity())
                .expect("regional destructible validation rejects zero integrity")
                .with_destruction_effect(profile.destruction_effect().clone())
                .with_evasion_disabled(),
        );
    }
    Ok(actors)
}

fn destructible_spawn_cells(
    map: &Map,
    passages: &[GridPos],
    reserved: &BTreeSet<GridPos>,
    profile: &RegionDestructibleProfile,
) -> Vec<GridPos> {
    (1..map.height() as i32 - 1)
        .flat_map(|y| (1..map.width() as i32 - 1).map(move |x| GridPos::new(x, y)))
        .filter(|position| {
            map.is_walkable(*position)
                && !map.is_protected(*position)
                && !reserved.contains(position)
                && [
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ]
                .into_iter()
                .filter(|direction| map.is_walkable(position.step(*direction)))
                .count()
                    >= 3
                && passages.iter().all(|passage| {
                    manhattan_distance(*position, *passage)
                        >= u32::from(profile.minimum_passage_distance())
                })
        })
        .collect()
}

fn inclusive_u16(rng: &mut GameRng, minimum: u16, maximum: u16) -> u16 {
    let span = u64::from(maximum - minimum) + 1;
    minimum + below(rng, span) as u16
}

fn below(rng: &mut GameRng, bound: u64) -> u64 {
    debug_assert!(bound > 0);
    let threshold = bound.wrapping_neg() % bound;
    loop {
        let value = rng.next_u64();
        if value >= threshold {
            return value % bound;
        }
    }
}

fn manhattan_distance(left: GridPos, right: GridPos) -> u32 {
    left.x.abs_diff(right.x) + left.y.abs_diff(right.y)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegionalDestructibleError {
    InsufficientSpawnSpace {
        requested: u16,
        available: usize,
        minimum_passage_distance: u16,
    },
}

impl Display for RegionalDestructibleError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientSpawnSpace {
                requested,
                available,
                minimum_passage_distance,
            } => write!(
                formatter,
                "regional destructibles requested {requested} actors but only {available} cells remain at least {minimum_passage_distance} tiles from every passage"
            ),
        }
    }
}

impl Error for RegionalDestructibleError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::{DamagePacket, DamageType};
    use crate::content::{RegionMapSize, RegionTerrain, RegionTerrainProfile};
    use crate::effects::{DamageFalloff, DestructionEffect, RadialDamageEffect};
    use crate::world::generation::{RegionalMapGenerator, cardinal_passage};
    use crate::world::{Direction, NeighborMode, TerrainPropagationPolicy};

    fn profile() -> RegionDestructibleProfile {
        RegionDestructibleProfile::new(
            3,
            5,
            6,
            4,
            DestructionEffect::new(RadialDamageEffect {
                maximum_cost: 2,
                neighbor_mode: NeighborMode::CardinalAndDiagonal,
                propagation_policy: TerrainPropagationPolicy::blocked_by_walls(1),
                damage: DamagePacket::new(8, DamageType::Explosive, 0),
                falloff: DamageFalloff::PerPropagationCost(2),
            }),
        )
        .unwrap()
    }

    fn generated_map() -> (Map, [GridPos; 4]) {
        let size = RegionMapSize::new(48, 36).unwrap();
        let terrain = RegionTerrainProfile::new(RegionTerrain::Gravel, 0, 0, 0, vec![]).unwrap();
        let generated = RegionalMapGenerator::new(size, &terrain)
            .generate(17)
            .unwrap();
        let passages = [
            cardinal_passage(size, Direction::North),
            cardinal_passage(size, Direction::East),
            cardinal_passage(size, Direction::South),
            cardinal_passage(size, Direction::West),
        ];
        let (map, _, _) = generated.into_parts();
        (map, passages)
    }

    #[test]
    fn placement_is_bounded_deterministic_and_uses_a_real_destruction_effect() {
        let (map, passages) = generated_map();
        let reserved = BTreeSet::from([GridPos::new(10, 10), GridPos::new(20, 20)]);
        let first =
            generate_regional_destructibles(&map, &passages, &reserved, &profile(), 91).unwrap();
        let second =
            generate_regional_destructibles(&map, &passages, &reserved, &profile(), 91).unwrap();

        assert_eq!(first, second);
        assert!((3..=5).contains(&first.len()));
        assert!(first.iter().all(|actor| {
            !reserved.contains(&actor.position())
                && actor.ai().is_none()
                && actor.attacks().is_empty()
                && actor.destruction_effect().is_some()
                && !actor.can_evade()
        }));
    }
}
