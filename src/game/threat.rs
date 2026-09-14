use std::fmt::{Debug, Formatter};
use std::num::NonZeroU16;

use crate::entity::Actor;
use crate::world::GridPos;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThreatReinforcementRequestError {
    UnknownSource,
    SourceInactive,
    QuotaExhausted,
}

#[derive(Clone, Debug)]
pub struct ThreatSourceBlueprint {
    pub position: GridPos,
    pub interval_turns: NonZeroU16,
    pub maximum_active: NonZeroU16,
    pub maximum_total: NonZeroU16,
    pub actor: Actor,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ThreatSourceState {
    pub(crate) id: u16,
    pub(crate) position: GridPos,
    pub(crate) interval_turns: u16,
    pub(crate) remaining_turns: u16,
    pub(crate) maximum_active: u16,
    pub(crate) maximum_total: u16,
    pub(crate) spawned_total: u16,
    pub(crate) active: bool,
    pub(crate) actor: Actor,
    pub(crate) pending_investigation: Option<GridPos>,
}

// An absent investigation keeps the historical v20-v24 representation used
// by deterministic suspension fingerprints.
impl Debug for ThreatSourceState {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut source = formatter.debug_struct("ThreatSourceState");
        source
            .field("id", &self.id)
            .field("position", &self.position)
            .field("interval_turns", &self.interval_turns)
            .field("remaining_turns", &self.remaining_turns)
            .field("maximum_active", &self.maximum_active)
            .field("maximum_total", &self.maximum_total)
            .field("spawned_total", &self.spawned_total)
            .field("active", &self.active)
            .field("actor", &self.actor);
        if let Some(incident) = self.pending_investigation {
            source.field("pending_investigation", &incident);
        }
        source.finish()
    }
}

impl ThreatSourceState {
    pub(crate) fn instantiate(id: u16, blueprint: ThreatSourceBlueprint) -> Self {
        Self {
            id,
            position: blueprint.position,
            interval_turns: blueprint.interval_turns.get(),
            remaining_turns: blueprint.interval_turns.get(),
            maximum_active: blueprint.maximum_active.get(),
            maximum_total: blueprint.maximum_total.get(),
            spawned_total: 0,
            active: true,
            actor: blueprint.actor,
            pending_investigation: None,
        }
    }

    pub const fn position(&self) -> GridPos {
        self.position
    }

    pub const fn is_active(&self) -> bool {
        self.active
    }

    pub const fn spawned_total(&self) -> u16 {
        self.spawned_total
    }

    pub const fn remaining_turns(&self) -> u16 {
        self.remaining_turns
    }

    pub const fn maximum_active(&self) -> u16 {
        self.maximum_active
    }

    pub const fn maximum_total(&self) -> u16 {
        self.maximum_total
    }

    pub const fn pending_investigation(&self) -> Option<GridPos> {
        self.pending_investigation
    }
}
