use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use super::{Direction, GridPos, VisibilityState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MovementTraceRules {
    pub maximum_per_tile: usize,
    pub lifetime_turns: u64,
}

impl Default for MovementTraceRules {
    fn default() -> Self {
        Self {
            maximum_per_tile: 3,
            lifetime_turns: 12,
        }
    }
}

impl MovementTraceRules {
    pub const fn validate(self) -> Result<(), MovementTraceRulesError> {
        if self.maximum_per_tile == 0 {
            return Err(MovementTraceRulesError::ZeroMaximumPerTile);
        }
        if self.lifetime_turns == 0 {
            return Err(MovementTraceRulesError::ZeroLifetime);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MovementTraceRulesError {
    ZeroMaximumPerTile,
    ZeroLifetime,
}

impl Display for MovementTraceRulesError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroMaximumPerTile => write!(formatter, "trace limit per tile must be positive"),
            Self::ZeroLifetime => write!(formatter, "trace lifetime must be positive"),
        }
    }
}

impl Error for MovementTraceRulesError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MovementTrace {
    direction: Direction,
    created_on_turn: u64,
}

impl MovementTrace {
    pub const fn direction(self) -> Direction {
        self.direction
    }

    pub const fn created_on_turn(self) -> u64 {
        self.created_on_turn
    }

    pub const fn age_on_turn(self, turn: u64) -> u64 {
        turn.saturating_sub(self.created_on_turn)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObservedMovementTrace {
    pub position: GridPos,
    pub direction: Direction,
    pub age_turns: u64,
}

/// Local, bounded evidence left by movement.
///
/// Traces deliberately store no entity identifier: reading old evidence must
/// not become a live tracker for its author.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MovementTraceMap {
    by_tile: BTreeMap<GridPos, Vec<MovementTrace>>,
}

impl MovementTraceMap {
    pub fn record(
        &mut self,
        position: GridPos,
        direction: Direction,
        turn: u64,
        rules: MovementTraceRules,
    ) {
        if rules.maximum_per_tile == 0 || rules.lifetime_turns == 0 {
            return;
        }

        let traces = self.by_tile.entry(position).or_default();
        traces.push(MovementTrace {
            direction,
            created_on_turn: turn,
        });
        let excess = traces.len().saturating_sub(rules.maximum_per_tile);
        if excess > 0 {
            traces.drain(..excess);
        }
    }

    pub fn traces_at(&self, position: GridPos) -> &[MovementTrace] {
        self.by_tile.get(&position).map_or(&[], Vec::as_slice)
    }

    pub fn observe(
        &self,
        origin: GridPos,
        radius: u16,
        turn: u64,
        visibility: &VisibilityState,
        rules: MovementTraceRules,
    ) -> Vec<ObservedMovementTrace> {
        let radius = i64::from(radius);
        self.by_tile
            .iter()
            .filter(|(position, _)| visibility.is_visible(**position))
            .filter(|(position, _)| {
                let dx = (i64::from(position.x) - i64::from(origin.x)).abs();
                let dy = (i64::from(position.y) - i64::from(origin.y)).abs();
                dx.max(dy) <= radius
            })
            .flat_map(|(position, traces)| {
                traces.iter().filter_map(move |trace| {
                    let age_turns = trace.age_on_turn(turn);
                    (age_turns <= rules.lifetime_turns).then_some(ObservedMovementTrace {
                        position: *position,
                        direction: trace.direction,
                        age_turns,
                    })
                })
            })
            .collect()
    }

    pub fn prune(&mut self, turn: u64, rules: MovementTraceRules) {
        for traces in self.by_tile.values_mut() {
            traces.retain(|trace| trace.age_on_turn(turn) <= rules.lifetime_turns);
        }
        self.by_tile.retain(|_, traces| !traces.is_empty());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::world::{DistanceMetric, FieldOfViewRules, Map};

    #[test]
    fn traces_are_bounded_age_limited_and_anonymous() {
        let rules = MovementTraceRules {
            maximum_per_tile: 3,
            lifetime_turns: 12,
        };
        let position = GridPos::new(2, 1);
        let mut traces = MovementTraceMap::default();
        for turn in 0..4 {
            traces.record(position, Direction::East, turn, rules);
        }

        assert_eq!(traces.traces_at(position).len(), 3);
        assert_eq!(traces.traces_at(position)[0].created_on_turn(), 1);

        traces.prune(14, rules);
        assert_eq!(traces.traces_at(position).len(), 2);
    }

    #[test]
    fn observation_never_crosses_range_or_current_visibility() {
        let map = Map::from_ascii("#######\n#.....#\n#######")
            .unwrap_or_else(|error| panic!("valid map rejected: {error}"));
        let mut visibility = VisibilityState::default();
        visibility.recompute(
            &map,
            GridPos::new(1, 1),
            FieldOfViewRules {
                radius: 2,
                distance_metric: DistanceMetric::Chebyshev,
                block_closed_corners: true,
            },
        );
        let rules = MovementTraceRules::default();
        let mut traces = MovementTraceMap::default();
        traces.record(GridPos::new(2, 1), Direction::East, 0, rules);
        traces.record(GridPos::new(4, 1), Direction::West, 0, rules);

        assert_eq!(
            traces.observe(GridPos::new(1, 1), 3, 1, &visibility, rules),
            vec![ObservedMovementTrace {
                position: GridPos::new(2, 1),
                direction: Direction::East,
                age_turns: 1,
            }]
        );
    }
}
