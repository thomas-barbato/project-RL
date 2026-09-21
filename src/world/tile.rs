use super::GridPos;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Terrain {
    Floor,
    /// Traversable water. It has no movement penalty yet, but remains a
    /// distinct simulation value so future movement profiles can treat it
    /// differently without asking the renderer what it looks like.
    ShallowWater,
    /// Water that cannot be crossed by ordinary movement. It does not block
    /// sight, unlike a wall or dense obstacle.
    DeepWater,
    Wall,
    Door(DoorState),
    ControlPanel {
        door: GridPos,
        activated: bool,
    },
}

impl Terrain {
    pub const fn blocks_movement(self) -> bool {
        !matches!(
            self,
            Self::Floor | Self::ShallowWater | Self::Door(DoorState::Open)
        )
    }

    pub const fn blocks_vision(self) -> bool {
        !matches!(
            self,
            Self::Floor | Self::ShallowWater | Self::DeepWater | Self::Door(DoorState::Open)
        )
    }

    pub const fn is_interactive(self) -> bool {
        matches!(self, Self::Door(_) | Self::ControlPanel { .. })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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
