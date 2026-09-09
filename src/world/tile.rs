/// Static terrain category. Dynamic environmental layers will be added to
/// `TileState` as their simulation systems are implemented.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Terrain {
    Floor,
    Wall,
}

impl Terrain {
    pub const fn blocks_movement(self) -> bool {
        matches!(self, Self::Wall)
    }

    pub const fn blocks_vision(self) -> bool {
        matches!(self, Self::Wall)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TileState {
    pub terrain: Terrain,
}

impl TileState {
    pub const fn new(terrain: Terrain) -> Self {
        Self { terrain }
    }
}
