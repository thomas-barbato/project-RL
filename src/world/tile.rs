use super::GridPos;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DoorState {
    Closed,
    Open,
    Locked,
    /// The actuator cannot move the door until its supplying installation
    /// becomes operational again.
    Unpowered,
}

/// Terrain and the first declarative interaction primitives. Renderers must
/// obtain movement/vision semantics here, never infer them from a glyph.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Terrain {
    Floor,
    Wall,
    Door(DoorState),
    ControlPanel { door: GridPos, activated: bool },
}

impl Terrain {
    pub const fn blocks_movement(self) -> bool {
        !matches!(self, Self::Floor | Self::Door(DoorState::Open))
    }

    pub const fn blocks_vision(self) -> bool {
        self.blocks_movement()
    }

    pub const fn is_interactive(self) -> bool {
        matches!(self, Self::Door(_) | Self::ControlPanel { .. })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TileState {
    pub terrain: Terrain,
    /// Protected ground is explicit level data, not a renderer or AI exception
    /// tied to a particular town name or coordinate.
    pub protected: bool,
}

impl TileState {
    pub const fn new(terrain: Terrain) -> Self {
        Self {
            terrain,
            protected: false,
        }
    }
}
