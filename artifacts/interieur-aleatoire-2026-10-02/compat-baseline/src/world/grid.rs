/// A position in simulation space. It is never expressed in screen pixels.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}

impl GridPos {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn step(self, direction: Direction) -> Self {
        let (delta_x, delta_y) = direction.delta();
        Self::new(self.x + delta_x, self.y + delta_y)
    }

    pub const fn cardinal_neighbors(self) -> [Self; 4] {
        [
            self.step(Direction::North),
            self.step(Direction::East),
            self.step(Direction::South),
            self.step(Direction::West),
        ]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub const fn delta(self) -> (i32, i32) {
        match self {
            Self::North => (0, -1),
            Self::East => (1, 0),
            Self::South => (0, 1),
            Self::West => (-1, 0),
        }
    }

    pub const fn from_delta(delta_x: i32, delta_y: i32) -> Option<Self> {
        match (delta_x, delta_y) {
            (0, -1) => Some(Self::North),
            (1, 0) => Some(Self::East),
            (0, 1) => Some(Self::South),
            (-1, 0) => Some(Self::West),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cardinal_neighbors_are_stable_and_clockwise() {
        let origin = GridPos::new(4, 7);

        assert_eq!(
            origin.cardinal_neighbors(),
            [
                GridPos::new(4, 6),
                GridPos::new(5, 7),
                GridPos::new(4, 8),
                GridPos::new(3, 7),
            ]
        );
    }
}
