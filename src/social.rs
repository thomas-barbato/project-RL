//! Bounded social facts known by individual actors.
//!
//! This module does not calculate reputation or broadcast knowledge to a whole
//! group. It records only what one configured observer could actually see.

use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::content::ContentId;
use crate::entity::EntityId;
use crate::item::ItemId;
use crate::world::{DistanceMetric, FieldOfViewRules, GridPos};

pub type SocialGroupId = ContentId;

/// Authored combat disposition toward the player and their controlled allies.
///
/// This deliberately remains separate from social affiliation: sharing a
/// community is not sufficient to infer friendship, and having an AI is not
/// sufficient to infer hostility. A later diplomacy layer can resolve this
/// value from reputation and witnessed acts without changing combat callers.
/// The first consumer is autonomous companion target acquisition; this is not
/// yet a complete actor-to-actor diplomacy simulation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlayerRelation {
    Allied,
    #[default]
    Neutral,
    Hostile,
}

pub const MAX_WITNESS_RANGE: u16 = 64;
pub const MAX_WITNESS_MEMORIES: u16 = 256;
pub const MAX_LOCAL_ALERT_DURATION_TURNS: u16 = 10_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WitnessProfile {
    field_of_view: FieldOfViewRules,
    memory_capacity: u16,
}

impl WitnessProfile {
    pub fn new(
        radius: u16,
        distance_metric: DistanceMetric,
        block_closed_corners: bool,
        memory_capacity: u16,
    ) -> Result<Self, WitnessProfileError> {
        if radius == 0 || radius > MAX_WITNESS_RANGE {
            return Err(WitnessProfileError::InvalidRange(radius));
        }
        if memory_capacity == 0 || memory_capacity > MAX_WITNESS_MEMORIES {
            return Err(WitnessProfileError::InvalidMemoryCapacity(memory_capacity));
        }
        Ok(Self {
            field_of_view: FieldOfViewRules {
                radius,
                distance_metric,
                block_closed_corners,
            },
            memory_capacity,
        })
    }

    pub const fn field_of_view(self) -> FieldOfViewRules {
        self.field_of_view
    }

    pub const fn memory_capacity(self) -> usize {
        self.memory_capacity as usize
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WitnessProfileError {
    InvalidRange(u16),
    InvalidMemoryCapacity(u16),
}

impl Display for WitnessProfileError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidRange(value) => write!(
                formatter,
                "witness range must be between 1 and {MAX_WITNESS_RANGE}, got {value}"
            ),
            Self::InvalidMemoryCapacity(value) => write!(
                formatter,
                "witness memory capacity must be between 1 and {MAX_WITNESS_MEMORIES}, got {value}"
            ),
        }
    }
}

impl Error for WitnessProfileError {}

/// A bounded, data-driven reaction available to an individual witness.
/// Raising it does not inform any other actor or change global reputation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocalAlertProfile {
    duration_turns: u16,
}

impl LocalAlertProfile {
    pub fn new(duration_turns: u16) -> Result<Self, LocalAlertProfileError> {
        if duration_turns == 0 || duration_turns > MAX_LOCAL_ALERT_DURATION_TURNS {
            return Err(LocalAlertProfileError::InvalidDuration(duration_turns));
        }
        Ok(Self { duration_turns })
    }

    pub const fn duration_turns(self) -> u16 {
        self.duration_turns
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalAlertProfileError {
    InvalidDuration(u16),
}

impl Display for LocalAlertProfileError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDuration(value) => write!(
                formatter,
                "local alert duration must be between 1 and {MAX_LOCAL_ALERT_DURATION_TURNS}, got {value}"
            ),
        }
    }
}

impl Error for LocalAlertProfileError {}

/// A fact attributed to a visible taker by one individual witness.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ObservedPropertyTake {
    pub turn: u64,
    pub taker: EntityId,
    pub owner: SocialGroupId,
    pub item: ItemId,
    pub quantity: u16,
    pub at: GridPos,
}

/// The current local reaction of one actor. It remains an individual state:
/// transmission, installed alarms and faction-wide consequences are separate.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LocalAlert {
    pub incident: ObservedPropertyTake,
    pub expires_on_turn: u64,
}

impl LocalAlert {
    pub const fn is_active(&self, turn: u64) -> bool {
        turn < self.expires_on_turn
    }

    pub const fn remaining_turns(&self, turn: u64) -> u64 {
        self.expires_on_turn.saturating_sub(turn)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn witness_profiles_are_explicitly_bounded() {
        assert!(WitnessProfile::new(0, DistanceMetric::Euclidean, true, 4).is_err());
        assert!(
            WitnessProfile::new(MAX_WITNESS_RANGE + 1, DistanceMetric::Euclidean, true, 4).is_err()
        );
        assert!(WitnessProfile::new(8, DistanceMetric::Euclidean, true, 0).is_err());
        assert!(WitnessProfile::new(8, DistanceMetric::Euclidean, true, 4).is_ok());
    }

    #[test]
    fn local_alert_profiles_are_explicitly_bounded() {
        assert!(LocalAlertProfile::new(0).is_err());
        assert!(LocalAlertProfile::new(MAX_LOCAL_ALERT_DURATION_TURNS + 1).is_err());
        assert_eq!(LocalAlertProfile::new(8).unwrap().duration_turns(), 8);
    }
}
