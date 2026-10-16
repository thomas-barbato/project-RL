//! Mine footprints share targeting geometry with the range/vision laboratory.
use project_rl::skills::spatial::{Area, affected_cells};
use project_rl::world::{GridPos, Map};

pub(super) fn square(map: &Map, origin: GridPos, radius: i32) -> Vec<GridPos> {
    affected_cells(
        map,
        origin,
        Area::Square {
            radius: radius.max(0) as u16,
        },
        (1, 0),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn radius_one_includes_the_four_corners() {
        let map = Map::from_ascii("#####\n#...#\n#...#\n#...#\n#####").unwrap();
        let cells = square(&map, GridPos::new(2, 2), 1);
        assert_eq!(cells.len(), 9);
        for p in [
            GridPos::new(1, 1),
            GridPos::new(3, 1),
            GridPos::new(1, 3),
            GridPos::new(3, 3),
        ] {
            assert!(cells.contains(&p));
        }
    }
    #[test]
    fn a_complete_wall_shields_its_other_side() {
        let map = Map::from_ascii("#######\n#..#..#\n#..#..#\n#..#..#\n#######").unwrap();
        let cells = square(&map, GridPos::new(2, 2), 4);
        assert_eq!(cells.len(), 6);
        assert!(cells.iter().all(|p| p.x < 3));
    }
}
