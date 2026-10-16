//! Shared targeting geometry for the reviewed skill catalogue and its laboratory.
use crate::world::{GridPos, Map, has_line_of_sight};

#[derive(Clone, Copy, Debug)]
pub enum Area {
    Point,
    Square { radius: u16 },
    Disc { radius: u16 },
    Diamond { radius: u16 },
    Ring { radius: u16 },
    Wall { length: u16 },
    Line { length: u16, width: u16 },
    Cone { length: u16 },
}

/// Walls stop the effect on rays from its centre, including isolated obstacles.
/// An explosion cannot take a path around a wall to hit its shadow.
pub fn affected_cells(
    map: &Map,
    origin: GridPos,
    area: Area,
    direction: (i32, i32),
) -> Vec<GridPos> {
    if !map.is_walkable(origin) {
        return Vec::new();
    }
    if let Area::Line { length, width } = area {
        let mut result = Vec::new();
        for side in -(i32::from(width) / 2)..=i32::from(width) / 2 {
            for forward in 0..i32::from(length) {
                let at = GridPos::new(
                    origin.x + forward * direction.0 - side * direction.1,
                    origin.y + forward * direction.1 + side * direction.0,
                );
                if !map.is_walkable(at) {
                    break;
                }
                result.push(at);
            }
        }
        result.sort_by_key(|at| (at.x, at.y));
        return result;
    }
    let extent = match area {
        Area::Point => 0,
        Area::Square { radius } => i32::from(radius),
        Area::Disc { radius } | Area::Diamond { radius } | Area::Ring { radius } => {
            i32::from(radius)
        }
        Area::Wall { length } => i32::from(length),
        Area::Line { length, width } => i32::from(length.max(width)),
        Area::Cone { length } => i32::from(length),
    };
    let mut result = Vec::new();
    // Bound work by the real map, even for unusually large equipment ranks.
    let left = origin.x.saturating_sub(extent).max(0);
    let top = origin.y.saturating_sub(extent).max(0);
    let right = origin.x.saturating_add(extent).min(map.width() as i32 - 1);
    let bottom = origin.y.saturating_add(extent).min(map.height() as i32 - 1);
    for x in left..=right {
        for y in top..=bottom {
            let dx = x - origin.x;
            let dy = y - origin.y;
            let forward = dx * direction.0 + dy * direction.1;
            let side = dx * -direction.1 + dy * direction.0;
            let inside = match area {
                Area::Point => dx == 0 && dy == 0,
                Area::Square { radius } => dx.abs().max(dy.abs()) <= i32::from(radius),
                Area::Disc { radius } => {
                    i64::from(dx).pow(2) + i64::from(dy).pow(2) <= i64::from(radius).pow(2)
                }
                Area::Diamond { radius } => dx.abs() + dy.abs() <= i32::from(radius),
                Area::Ring { radius } => dx.abs().max(dy.abs()) == i32::from(radius),
                Area::Wall { length } => forward == 0 && side.abs() <= i32::from(length) / 2,
                Area::Line { length, width } => {
                    forward >= 0
                        && forward < i32::from(length)
                        && side.abs() <= i32::from(width) / 2
                }
                Area::Cone { length } => {
                    forward >= 1 && forward <= i32::from(length) && side.abs() <= forward
                }
            };
            let at = GridPos::new(x, y);
            if inside && map.is_walkable(at) && has_line_of_sight(map, origin, at, true) {
                result.push(at);
            }
        }
    }
    result
}

pub fn in_cast_range(from: GridPos, to: GridPos, range: u16) -> bool {
    let dx = i64::from(from.x) - i64::from(to.x);
    let dy = i64::from(from.y) - i64::from(to.y);
    dx * dx + dy * dy <= i64::from(range) * i64::from(range)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn square_keeps_corners_and_a_partial_wall_casts_a_shadow() {
        let map = Map::from_ascii(
            ".........\n.........\n....#....\n....#....\n....#....\n.........\n.........",
        )
        .unwrap();
        let origin = GridPos::new(2, 3);
        let cells = affected_cells(&map, origin, Area::Square { radius: 4 }, (1, 0));
        assert!(cells.contains(&GridPos::new(1, 2)));
        assert!(cells.contains(&GridPos::new(5, 0)));
        assert!(!cells.contains(&GridPos::new(5, 3)));
        assert!(!cells.contains(&GridPos::new(6, 3)));
        assert!(!cells.contains(&GridPos::new(4, 3)));
    }
    #[test]
    fn closed_corners_do_not_leak_and_blocked_centres_are_empty() {
        let map = Map::from_ascii(".....\n..#..\n.#...\n.....\n.....").unwrap();
        let area = Area::Square { radius: 3 };
        assert!(
            !affected_cells(&map, GridPos::new(1, 1), area, (1, 0)).contains(&GridPos::new(2, 2))
        );
        assert!(affected_cells(&map, GridPos::new(2, 1), area, (1, 0)).is_empty());
    }
    #[test]
    fn all_four_cone_directions_and_long_corridor_respect_walls() {
        let open = Map::from_ascii(&vec!["........."; 9].join("\n")).unwrap();
        for direction in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            assert_eq!(
                affected_cells(
                    &open,
                    GridPos::new(4, 4),
                    Area::Cone { length: 3 },
                    direction
                )
                .len(),
                15
            );
        }
        let corridor = Map::from_ascii("#############\n.............\n#############").unwrap();
        let cells = affected_cells(
            &corridor,
            GridPos::new(1, 1),
            Area::Square { radius: 5 },
            (1, 0),
        );
        assert_eq!(cells.len(), 7);
        assert!(cells.iter().all(|at| at.y == 1));
    }
    #[test]
    fn eleven_square_can_be_placed_clear_of_caster_with_range_eight() {
        let map = Map::from_ascii(&vec!["....................."; 21].join("\n")).unwrap();
        let player = GridPos::new(2, 10);
        let centre = GridPos::new(10, 10);
        assert!(in_cast_range(player, centre, 8));
        assert!(!in_cast_range(player, centre, 5));
        let cells = affected_cells(&map, centre, Area::Square { radius: 5 }, (1, 0));
        assert_eq!(cells.len(), 121);
        assert!(!cells.contains(&player));
    }

    #[test]
    fn wave_lanes_stop_independently_and_travel_has_no_extra_cell() {
        let map = Map::from_ascii(".........\n.........\n....#....\n.........\n.........").unwrap();
        let cells = affected_cells(
            &map,
            GridPos::new(2, 2),
            Area::Line {
                length: 5,
                width: 3,
            },
            (1, 0),
        );
        assert!(cells.contains(&GridPos::new(6, 1)));
        assert!(cells.contains(&GridPos::new(6, 3)));
        assert!(!cells.contains(&GridPos::new(5, 2)));
        assert!(!cells.contains(&GridPos::new(7, 1)));
        assert_eq!(cells.len(), 12);
    }

    #[test]
    fn round_shapes_and_ritual_perimeters_keep_their_geometry() {
        let map = Map::from_ascii(&vec!["..........."; 11].join("\n")).unwrap();
        let centre = GridPos::new(5, 5);
        assert_eq!(
            affected_cells(&map, centre, Area::Disc { radius: 2 }, (1, 0)).len(),
            13
        );
        assert_eq!(
            affected_cells(&map, centre, Area::Square { radius: 2 }, (1, 0)).len(),
            25
        );
        assert_eq!(
            affected_cells(&map, centre, Area::Ring { radius: 3 }, (1, 0)).len(),
            24
        );
        assert_eq!(
            affected_cells(&map, centre, Area::Diamond { radius: 1 }, (1, 0)).len(),
            5
        );
    }
}
