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

#[derive(Clone, Copy)]
struct SearchRecord {
    cost: u32,
    previous: Option<GridPos>,
}

impl SearchRecord {
    const UNSEEN: Self = Self {
        cost: u32::MAX,
        previous: None,
    };
}

/// The frontier still decides traversal order. These records are only looked up
/// by position, so grid indexing preserves every tie and search-budget decision.
struct SearchRecords {
    width: usize,
    height: usize,
    cells: Vec<SearchRecord>,
    sparse: BTreeMap<GridPos, SearchRecord>,
}

impl SearchRecords {
    fn new(map: &Map) -> Self {
        // Bound scratch memory for unusually large/modded maps. Custom traversal
        // policies may also accept coordinates outside the map; keep supporting
        // those in the sparse table instead of silently changing their paths.
        let count = map
            .width()
            .checked_mul(map.height())
            .filter(|count| *count <= 65_536)
            .unwrap_or(0);
        Self {
            width: map.width(),
            height: map.height(),
            cells: vec![SearchRecord::UNSEEN; count],
            sparse: BTreeMap::new(),
        }
    }

    fn index(&self, position: GridPos) -> Option<usize> {
        let x = usize::try_from(position.x).ok()?;
        let y = usize::try_from(position.y).ok()?;
        (!self.cells.is_empty() && x < self.width && y < self.height).then(|| y * self.width + x)
    }

    fn improve(&mut self, at: GridPos, previous: Option<GridPos>, cost: u32) -> bool {
        let record = if let Some(index) = self.index(at) {
            &mut self.cells[index]
        } else {
            self.sparse.entry(at).or_insert(SearchRecord::UNSEEN)
        };
        if cost >= record.cost {
            return false;
        }
        *record = SearchRecord { cost, previous };
        true
    }

    fn previous(&self, at: GridPos) -> Option<GridPos> {
        match self.index(at) {
            Some(index) => self.cells[index].previous,
            None => self.sparse.get(&at).and_then(|record| record.previous),
        }
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
    find_path_with(
        map,
        start,
        goal,
        maximum_visited,
        |map, position| map.is_walkable(position),
        can_enter,
    )
}

/// Deterministic A* with a caller-provided terrain traversal policy. This lets
/// actors plan through terrain they can change (for example an ordinary closed
/// door) without weakening the default pathfinder used by combat AI.
pub fn find_path_with<T, F>(
    map: &Map,
    start: GridPos,
    goal: GridPos,
    maximum_visited: usize,
    can_traverse: T,
    can_enter: F,
) -> Option<Vec<GridPos>>
where
    T: Fn(&Map, GridPos) -> bool,
    F: Fn(GridPos) -> bool,
{
    if !can_traverse(map, start) || !can_traverse(map, goal) {
        return None;
    }
    if start == goal {
        return Some(vec![start]);
    }
    if maximum_visited == 0 {
        return None;
    }

    let mut frontier = BinaryHeap::new();
    let mut records = SearchRecords::new(map);
    records.improve(start, None, 0);
    frontier.push(OpenNode {
        position: start,
        estimated_total_cost: heuristic(start, goal),
        cost_from_start: 0,
    });

    let mut visited = 0;
    while let Some(current) = frontier.pop() {
        if current.position == goal {
            return reconstruct_path(&records, start, goal);
        }
        if visited >= maximum_visited {
            return None;
        }
        visited += 1;

        for neighbor in current.position.cardinal_neighbors() {
            if !can_traverse(map, neighbor) || (neighbor != goal && !can_enter(neighbor)) {
                continue;
            }

            let new_cost = current.cost_from_start.saturating_add(1);
            if !records.improve(neighbor, Some(current.position), new_cost) {
                continue;
            }

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
    records: &SearchRecords,
    start: GridPos,
    goal: GridPos,
) -> Option<Vec<GridPos>> {
    let mut current = goal;
    let mut reversed = vec![goal];

    while current != start {
        current = records.previous(current)?;
        reversed.push(current);
    }
    reversed.reverse();
    Some(reversed)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Frozen pre-optimization search: old journals depend on the exact route,
    // including ties, exhausted budgets and the occupied-goal exception.
    fn legacy_path<T: Fn(&Map, GridPos) -> bool, F: Fn(GridPos) -> bool>(
        map: &Map,
        start: GridPos,
        goal: GridPos,
        maximum_visited: usize,
        can_traverse: T,
        can_enter: F,
    ) -> Option<Vec<GridPos>> {
        if !can_traverse(map, start) || !can_traverse(map, goal) {
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
                let mut at = goal;
                let mut reversed = vec![goal];
                while at != start {
                    at = *previous.get(&at)?;
                    reversed.push(at);
                }
                reversed.reverse();
                return Some(reversed);
            }
            if visited >= maximum_visited {
                return None;
            }
            visited += 1;
            for neighbor in current.position.cardinal_neighbors() {
                if !can_traverse(map, neighbor) || (neighbor != goal && !can_enter(neighbor)) {
                    continue;
                }
                let cost = current.cost_from_start.saturating_add(1);
                if cost >= costs.get(&neighbor).copied().unwrap_or(u32::MAX) {
                    continue;
                }
                costs.insert(neighbor, cost);
                previous.insert(neighbor, current.position);
                frontier.push(OpenNode {
                    position: neighbor,
                    estimated_total_cost: cost.saturating_add(heuristic(neighbor, goal)),
                    cost_from_start: cost,
                });
            }
        }
        None
    }

    #[test]
    fn indexed_paths_match_legacy_ties_blockers_and_budgets() {
        use super::super::Terrain;
        for seed in 0..24 {
            let mut map = Map::filled(9, 7, Terrain::Floor).unwrap();
            for y in 0..7 {
                for x in 0..9 {
                    if (x * 17 + y * 31 + seed * 13) % 11 < 3 {
                        map.set_terrain(GridPos::new(x, y), Terrain::Wall).unwrap();
                    }
                }
            }
            for pair in 0..12 {
                let start = GridPos::new((pair * 5 + seed) % 9, (pair * 3 + seed) % 7);
                let goal = if pair == 0 {
                    start
                } else {
                    GridPos::new((pair * 7 + seed) % 9, (pair * 4) % 7)
                };
                let enter = |at: GridPos| (at.x * 7 + at.y * 3 + seed) % 13 != 0;
                for budget in [0, 1, 2, 5, 12, 25, 63, 128] {
                    assert_eq!(
                        find_path(&map, start, goal, budget, enter),
                        legacy_path(&map, start, goal, budget, Map::is_walkable, enter),
                        "seed {seed}, pair {pair}, budget {budget}"
                    );
                }
            }
        }
    }

    #[test]
    fn sparse_paths_preserve_large_maps_and_custom_outside_coordinates() {
        use super::super::Terrain;
        for map in [
            Map::filled(2, 2, Terrain::Floor).unwrap(),
            Map::filled(300, 240, Terrain::Floor).unwrap(),
        ] {
            let traverse =
                |_: &Map, at: GridPos| (-3..=3).contains(&at.x) && (-2..=2).contains(&at.y);
            for start in [GridPos::new(-2, -1), GridPos::new(1, 1)] {
                for goal in [start, GridPos::new(3, 2)] {
                    for budget in [0, 1, 5, 15, 100] {
                        assert_eq!(
                            find_path_with(&map, start, goal, budget, traverse, |_| true),
                            legacy_path(&map, start, goal, budget, traverse, |_| true)
                        );
                    }
                }
            }
        }
    }

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

    #[test]
    fn custom_terrain_policy_can_plan_through_a_closed_door() {
        let mut map = parse_map("#####\n#...#\n#####");
        map.set_terrain(
            GridPos::new(2, 1),
            super::super::Terrain::Door(super::super::DoorState::Closed),
        )
        .unwrap();
        let path = find_path_with(
            &map,
            GridPos::new(1, 1),
            GridPos::new(3, 1),
            100,
            |map, position| {
                map.is_walkable(position)
                    || matches!(
                        map.tile(position).map(|tile| tile.terrain),
                        Some(super::super::Terrain::Door(super::super::DoorState::Closed))
                    )
            },
            |_| true,
        );

        assert_eq!(
            path,
            Some(vec![
                GridPos::new(1, 1),
                GridPos::new(2, 1),
                GridPos::new(3, 1),
            ])
        );
    }
}
