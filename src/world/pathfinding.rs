use std::cmp::Ordering;
use std::collections::{BTreeMap, BinaryHeap};

use super::{GridPos, Map};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct OpenNode {
    position: GridPos,
    estimated_total_cost: u32,
    cost_from_start: u32,
}

impl Ord for OpenNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .estimated_total_cost
            .cmp(&self.estimated_total_cost)
            .then_with(|| other.cost_from_start.cmp(&self.cost_from_start))
            .then_with(|| other.position.cmp(&self.position))
    }
}

impl PartialOrd for OpenNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Deterministic A* pathfinding. `can_enter` adds dynamic blockers such as
/// actors or mod-defined hazards on top of static terrain.
pub fn find_path<F>(
    map: &Map,
    start: GridPos,
    goal: GridPos,
    maximum_visited: usize,
    can_enter: F,
) -> Option<Vec<GridPos>>
where
    F: Fn(GridPos) -> bool,
{
    if !map.is_walkable(start) || !map.is_walkable(goal) {
        return None;
    }

    let mut frontier = BinaryHeap::new();
    let mut previous = BTreeMap::<GridPos, GridPos>::new();
    let mut costs = BTreeMap::from([(start, 0_u32)]);
    frontier.push(OpenNode {
        position: start,
        estimated_total_cost: heuristic(start, goal),
        cost_from_start: 0,
    });

    let mut visited = 0;
    while let Some(current) = frontier.pop() {
        if current.position == goal {
            return reconstruct_path(previous, start, goal);
        }
        if visited >= maximum_visited {
            return None;
        }
        visited += 1;

        for neighbor in current.position.cardinal_neighbors() {
            if !map.is_walkable(neighbor) || (neighbor != goal && !can_enter(neighbor)) {
                continue;
            }

            let new_cost = current.cost_from_start.saturating_add(1);
            let known_cost = costs.get(&neighbor).copied().unwrap_or(u32::MAX);
            if new_cost >= known_cost {
                continue;
            }

            costs.insert(neighbor, new_cost);
            previous.insert(neighbor, current.position);
            frontier.push(OpenNode {
                position: neighbor,
                estimated_total_cost: new_cost.saturating_add(heuristic(neighbor, goal)),
                cost_from_start: new_cost,
            });
        }
    }

    None
}

fn heuristic(first: GridPos, second: GridPos) -> u32 {
    let delta_x = (i64::from(first.x) - i64::from(second.x)).unsigned_abs();
    let delta_y = (i64::from(first.y) - i64::from(second.y)).unsigned_abs();
    delta_x.saturating_add(delta_y).min(u64::from(u32::MAX)) as u32
}

fn reconstruct_path(
    previous: BTreeMap<GridPos, GridPos>,
    start: GridPos,
    goal: GridPos,
) -> Option<Vec<GridPos>> {
    let mut current = goal;
    let mut reversed = vec![goal];

    while current != start {
        current = *previous.get(&current)?;
        reversed.push(current);
    }
    reversed.reverse();
    Some(reversed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_map(definition: &str) -> Map {
        match Map::from_ascii(definition) {
            Ok(map) => map,
            Err(error) => panic!("valid test map failed to parse: {error}"),
        }
    }

    #[test]
    fn path_routes_around_walls_deterministically() {
        let map = parse_map("#######\n#..#..#\n#.....#\n#######");

        let path = find_path(&map, GridPos::new(1, 1), GridPos::new(5, 1), 100, |_| true);

        assert_eq!(
            path,
            Some(vec![
                GridPos::new(1, 1),
                GridPos::new(2, 1),
                GridPos::new(2, 2),
                GridPos::new(3, 2),
                GridPos::new(4, 2),
                GridPos::new(4, 1),
                GridPos::new(5, 1),
            ])
        );
    }

    #[test]
    fn dynamic_blocker_can_make_route_unreachable() {
        let map = parse_map("#####\n#...#\n#####");

        let path = find_path(
            &map,
            GridPos::new(1, 1),
            GridPos::new(3, 1),
            100,
            |position| position != GridPos::new(2, 1),
        );

        assert_eq!(path, None);
    }
}
