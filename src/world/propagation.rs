use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};

use super::{GridPos, Map, Terrain};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NeighborMode {
    Cardinal,
    CardinalAndDiagonal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PropagationRequest {
    pub origin: GridPos,
    pub maximum_cost: u16,
    pub neighbor_mode: NeighborMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PropagationCell {
    pub position: GridPos,
    pub cost: u16,
    pub step: u16,
}

/// Supplies effect-specific traversal rules while the propagation algorithm
/// remains shared by explosions, fire, electricity, gas and modded effects.
pub trait PropagationPolicy {
    fn traversal_cost(&self, map: &Map, from: GridPos, to: GridPos) -> Option<u16>;
}

/// Initial data-shaped policy based on terrain. `None` makes a terrain
/// impassable to the effect; a larger cost slows or weakens propagation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerrainPropagationPolicy {
    pub floor_cost: Option<u16>,
    pub wall_cost: Option<u16>,
}

impl TerrainPropagationPolicy {
    pub const fn blocked_by_walls(floor_cost: u16) -> Self {
        Self {
            floor_cost: Some(floor_cost),
            wall_cost: None,
        }
    }
}

impl PropagationPolicy for TerrainPropagationPolicy {
    fn traversal_cost(&self, map: &Map, _from: GridPos, to: GridPos) -> Option<u16> {
        match map.tile(to)?.terrain {
            Terrain::Floor => self.floor_cost,
            Terrain::Wall => self.wall_cost,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FrontierCell {
    position: GridPos,
    cost: u16,
    step: u16,
}

impl Ord for FrontierCell {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.step.cmp(&self.step))
            .then_with(|| other.position.cmp(&self.position))
    }
}

impl PartialOrd for FrontierCell {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn propagate<P: PropagationPolicy>(
    map: &Map,
    request: PropagationRequest,
    policy: &P,
) -> Vec<PropagationCell> {
    if !map.contains(request.origin) {
        return Vec::new();
    }

    let origin = FrontierCell {
        position: request.origin,
        cost: 0,
        step: 0,
    };
    let mut frontier = BinaryHeap::from([origin]);
    let mut best = BTreeMap::from([(request.origin, (0_u16, 0_u16))]);

    while let Some(current) = frontier.pop() {
        if best.get(&current.position) != Some(&(current.cost, current.step)) {
            continue;
        }

        for neighbor in neighbors(current.position, request.neighbor_mode) {
            let Some(traversal_cost) = policy.traversal_cost(map, current.position, neighbor)
            else {
                continue;
            };
            let Some(cost) = current.cost.checked_add(traversal_cost) else {
                continue;
            };
            if cost > request.maximum_cost {
                continue;
            }
            let step = current.step.saturating_add(1);
            let candidate = (cost, step);
            let is_better = best.get(&neighbor).is_none_or(|known| candidate < *known);
            if !is_better {
                continue;
            }

            best.insert(neighbor, candidate);
            frontier.push(FrontierCell {
                position: neighbor,
                cost,
                step,
            });
        }
    }

    let mut result: Vec<PropagationCell> = best
        .into_iter()
        .map(|(position, (cost, step))| PropagationCell {
            position,
            cost,
            step,
        })
        .collect();
    result.sort_by_key(|cell| (cell.cost, cell.step, cell.position));
    result
}

fn neighbors(position: GridPos, mode: NeighborMode) -> Vec<GridPos> {
    let mut neighbors = position.cardinal_neighbors().to_vec();
    if mode == NeighborMode::CardinalAndDiagonal {
        neighbors.extend([
            GridPos::new(position.x - 1, position.y - 1),
            GridPos::new(position.x + 1, position.y - 1),
            GridPos::new(position.x + 1, position.y + 1),
            GridPos::new(position.x - 1, position.y + 1),
        ]);
    }
    neighbors
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_map(definition: &str) -> Map {
        Map::from_ascii(definition)
            .unwrap_or_else(|error| panic!("valid test map failed to parse: {error}"))
    }

    #[test]
    fn walls_block_a_standard_radial_propagation() {
        let map = parse_map("#######\n#..#..#\n#.....#\n#######");
        let cells = propagate(
            &map,
            PropagationRequest {
                origin: GridPos::new(1, 1),
                maximum_cost: 3,
                neighbor_mode: NeighborMode::Cardinal,
            },
            &TerrainPropagationPolicy::blocked_by_walls(1),
        );

        assert!(cells.iter().any(|cell| cell.position == GridPos::new(2, 1)));
        assert!(cells.iter().all(|cell| cell.position != GridPos::new(3, 1)));
        assert!(cells.iter().all(|cell| cell.position != GridPos::new(4, 1)));
    }

    #[test]
    fn a_policy_can_cross_terrain_at_a_custom_cost() {
        let map = parse_map("#####\n#.#.#\n#####");
        let cells = propagate(
            &map,
            PropagationRequest {
                origin: GridPos::new(1, 1),
                maximum_cost: 3,
                neighbor_mode: NeighborMode::Cardinal,
            },
            &TerrainPropagationPolicy {
                floor_cost: Some(1),
                wall_cost: Some(2),
            },
        );

        assert_eq!(
            cells
                .iter()
                .find(|cell| cell.position == GridPos::new(2, 1))
                .map(|cell| cell.cost),
            Some(2)
        );
        assert_eq!(
            cells
                .iter()
                .find(|cell| cell.position == GridPos::new(3, 1))
                .map(|cell| cell.cost),
            Some(3)
        );
    }

    #[test]
    fn result_order_is_stable_and_suitable_for_delayed_animation() {
        let map = parse_map("#####\n#...#\n#...#\n#...#\n#####");
        let request = PropagationRequest {
            origin: GridPos::new(2, 2),
            maximum_cost: 2,
            neighbor_mode: NeighborMode::Cardinal,
        };
        let policy = TerrainPropagationPolicy::blocked_by_walls(1);

        let first = propagate(&map, request, &policy);
        let second = propagate(&map, request, &policy);

        assert_eq!(first, second);
        assert!(first.windows(2).all(|pair| pair[0].cost <= pair[1].cost));
        assert_eq!(first.first().map(|cell| cell.step), Some(0));
    }
}
