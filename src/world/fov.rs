use std::collections::BTreeSet;

use super::{GridPos, Map};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DistanceMetric {
    Chebyshev,
    Euclidean,
}

/// Data-shaped field-of-view rules. These values can later be loaded from a
/// content package or exposed to a sandboxed rule script.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FieldOfViewRules {
    pub radius: u16,
    pub distance_metric: DistanceMetric,
    pub block_closed_corners: bool,
}

impl Default for FieldOfViewRules {
    fn default() -> Self {
        Self {
            radius: 8,
            distance_metric: DistanceMetric::Euclidean,
            block_closed_corners: true,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VisibilityState {
    visible: BTreeSet<GridPos>,
    explored: BTreeSet<GridPos>,
}

impl VisibilityState {
    pub(crate) fn clear_visible(&mut self) {
        self.visible.clear();
    }

    pub fn recompute(&mut self, map: &Map, origin: GridPos, rules: FieldOfViewRules) {
        self.visible = compute_visible_tiles(map, origin, rules);
        self.explored.extend(self.visible.iter().copied());
    }

    pub fn is_visible(&self, position: GridPos) -> bool {
        self.visible.contains(&position)
    }

    pub fn is_explored(&self, position: GridPos) -> bool {
        self.explored.contains(&position)
    }

    pub fn visible_positions(&self) -> impl Iterator<Item = GridPos> + '_ {
        self.visible.iter().copied()
    }

    pub fn explored_positions(&self) -> impl Iterator<Item = GridPos> + '_ {
        self.explored.iter().copied()
    }
}

pub fn compute_visible_tiles(
    map: &Map,
    origin: GridPos,
    rules: FieldOfViewRules,
) -> BTreeSet<GridPos> {
    let mut visible = BTreeSet::new();
    if !map.contains(origin) {
        return visible;
    }

    let radius = i32::from(rules.radius);
    let minimum_x = origin.x.saturating_sub(radius);
    let maximum_x = origin.x.saturating_add(radius);
    let minimum_y = origin.y.saturating_sub(radius);
    let maximum_y = origin.y.saturating_add(radius);

    for y in minimum_y..=maximum_y {
        for x in minimum_x..=maximum_x {
            let target = GridPos::new(x, y);
            if map.contains(target)
                && is_within_radius(origin, target, radius, rules.distance_metric)
                && has_line_of_sight(map, origin, target, rules.block_closed_corners)
            {
                visible.insert(target);
            }
        }
    }

    visible
}

fn is_within_radius(origin: GridPos, target: GridPos, radius: i32, metric: DistanceMetric) -> bool {
    let delta_x = i64::from((target.x - origin.x).abs());
    let delta_y = i64::from((target.y - origin.y).abs());
    let radius = i64::from(radius);

    match metric {
        DistanceMetric::Chebyshev => delta_x.max(delta_y) <= radius,
        DistanceMetric::Euclidean => delta_x * delta_x + delta_y * delta_y <= radius * radius,
    }
}

pub fn has_line_of_sight(
    map: &Map,
    origin: GridPos,
    target: GridPos,
    block_closed_corners: bool,
) -> bool {
    if origin == target {
        return true;
    }

    let delta_x = (target.x - origin.x).abs();
    let step_x = (target.x - origin.x).signum();
    let delta_y = -(target.y - origin.y).abs();
    let step_y = (target.y - origin.y).signum();
    let mut error = delta_x + delta_y;
    let mut current = origin;

    loop {
        let previous = current;
        let doubled_error = error.saturating_mul(2);

        if doubled_error >= delta_y {
            error += delta_y;
            current.x += step_x;
        }
        if doubled_error <= delta_x {
            error += delta_x;
            current.y += step_y;
        }

        if block_closed_corners && current.x != previous.x && current.y != previous.y {
            let horizontal_side = GridPos::new(current.x, previous.y);
            let vertical_side = GridPos::new(previous.x, current.y);
            if map.blocks_vision(horizontal_side) && map.blocks_vision(vertical_side) {
                return false;
            }
        }

        if current == target {
            return true;
        }
        if map.blocks_vision(current) {
            return false;
        }
    }
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
    fn wall_is_visible_but_tiles_behind_it_are_hidden() {
        let map = parse_map("#######\n#.....#\n#..#..#\n#.....#\n#######");
        let visible = compute_visible_tiles(
            &map,
            GridPos::new(1, 2),
            FieldOfViewRules {
                radius: 8,
                ..FieldOfViewRules::default()
            },
        );

        assert!(visible.contains(&GridPos::new(3, 2)));
        assert!(!visible.contains(&GridPos::new(4, 2)));
    }

    #[test]
    fn two_touching_walls_block_diagonal_corner_vision() {
        let map = parse_map("#####\n#.#.#\n##..#\n#...#\n#####");
        let visible = compute_visible_tiles(&map, GridPos::new(1, 1), FieldOfViewRules::default());

        assert!(!visible.contains(&GridPos::new(2, 2)));
    }

    #[test]
    fn explored_tiles_remain_known_after_moving_away() {
        let map = parse_map("#########\n#.......#\n#########");
        let rules = FieldOfViewRules {
            radius: 2,
            distance_metric: DistanceMetric::Chebyshev,
            block_closed_corners: true,
        };
        let mut visibility = VisibilityState::default();

        visibility.recompute(&map, GridPos::new(2, 1), rules);
        visibility.recompute(&map, GridPos::new(6, 1), rules);

        assert!(!visibility.is_visible(GridPos::new(1, 1)));
        assert!(visibility.is_explored(GridPos::new(1, 1)));
        assert!(visibility.is_visible(GridPos::new(7, 1)));
    }
}
