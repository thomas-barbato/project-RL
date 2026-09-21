use std::fmt::{Debug, Formatter};
use std::num::NonZeroU16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AiBehavior {
    Idle,
    Hunter,
    Sentry,
    Skirmisher,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PursuitLifecycle {
    maximum_pursuit_turns: NonZeroU16,
    search_turns: NonZeroU16,
    cooldown_turns: NonZeroU16,
}

impl PursuitLifecycle {
    pub const fn new(
        maximum_pursuit_turns: NonZeroU16,
        search_turns: NonZeroU16,
        cooldown_turns: NonZeroU16,
    ) -> Self {
        Self {
            maximum_pursuit_turns,
            search_turns,
            cooldown_turns,
        }
    }

    pub const fn maximum_pursuit_turns(self) -> u16 {
        self.maximum_pursuit_turns.get()
    }

    pub const fn search_turns(self) -> u16 {
        self.search_turns.get()
    }

    pub const fn cooldown_turns(self) -> u16 {
        self.cooldown_turns.get()
    }
}

/// Persistent perception state. It belongs to the simulation rather than to
/// the renderer, so suspending or leaving a zone cannot reset a pursuit.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AiState {
    #[default]
    Unaware,
    Pursuing {
        remaining_turns: u16,
        last_seen: crate::world::GridPos,
    },
    Searching {
        remaining_turns: u16,
        last_seen: crate::world::GridPos,
    },
    /// Moving toward a reported incident rather than the target's live
    /// position. Perception may still turn this into an ordinary pursuit.
    Responding {
        remaining_turns: u16,
        incident: crate::world::GridPos,
    },
    Returning,
    Cooldown {
        remaining_turns: u16,
    },
}

/// Data-shaped AI parameters, suitable for later content definitions.
#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AiProfile {
    pub behavior: AiBehavior,
    pub perception_radius: u16,
    pub preferred_attack_slot: u8,
    pub maximum_path_search: usize,
    pub preferred_minimum_distance: u16,
    maximum_pursuit_distance: Option<NonZeroU16>,
    pursuit_lifecycle: Option<PursuitLifecycle>,
}

// Omit the new optional pursuit rule when absent so deterministic suspension
// fingerprints from versions 1-12 retain their historical representation.
impl Debug for AiProfile {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut profile = formatter.debug_struct("AiProfile");
        profile
            .field("behavior", &self.behavior)
            .field("perception_radius", &self.perception_radius)
            .field("preferred_attack_slot", &self.preferred_attack_slot)
            .field("maximum_path_search", &self.maximum_path_search)
            .field(
                "preferred_minimum_distance",
                &self.preferred_minimum_distance,
            );
        if let Some(distance) = self.maximum_pursuit_distance {
            profile.field("maximum_pursuit_distance", &distance);
        }
        if let Some(lifecycle) = self.pursuit_lifecycle {
            profile.field("pursuit_lifecycle", &lifecycle);
        }
        profile.finish()
    }
}

impl AiProfile {
    pub const fn new(
        behavior: AiBehavior,
        perception_radius: u16,
        preferred_attack_slot: u8,
        maximum_path_search: usize,
        preferred_minimum_distance: u16,
    ) -> Self {
        Self {
            behavior,
            perception_radius,
            preferred_attack_slot,
            maximum_path_search,
            preferred_minimum_distance,
            maximum_pursuit_distance: None,
            pursuit_lifecycle: None,
        }
    }

    pub const fn idle() -> Self {
        Self::new(AiBehavior::Idle, 0, 0, 0, 0)
    }

    pub const fn hunter(perception_radius: u16, preferred_attack_slot: u8) -> Self {
        Self::new(
            AiBehavior::Hunter,
            perception_radius,
            preferred_attack_slot,
            2_048,
            0,
        )
    }

    pub const fn sentry(perception_radius: u16, preferred_attack_slot: u8) -> Self {
        Self::new(
            AiBehavior::Sentry,
            perception_radius,
            preferred_attack_slot,
            0,
            0,
        )
    }

    pub const fn skirmisher(
        perception_radius: u16,
        preferred_attack_slot: u8,
        preferred_minimum_distance: u16,
    ) -> Self {
        Self::new(
            AiBehavior::Skirmisher,
            perception_radius,
            preferred_attack_slot,
            2_048,
            preferred_minimum_distance,
        )
    }

    pub const fn with_maximum_pursuit_distance(mut self, distance: NonZeroU16) -> Self {
        self.maximum_pursuit_distance = Some(distance);
        self
    }

    pub const fn maximum_pursuit_distance(self) -> Option<u16> {
        match self.maximum_pursuit_distance {
            Some(distance) => Some(distance.get()),
            None => None,
        }
    }

    pub const fn without_pursuit_limit(mut self) -> Self {
        self.maximum_pursuit_distance = None;
        self
    }

    pub const fn with_pursuit_lifecycle(mut self, lifecycle: PursuitLifecycle) -> Self {
        self.pursuit_lifecycle = Some(lifecycle);
        self
    }

    pub const fn pursuit_lifecycle(self) -> Option<PursuitLifecycle> {
        self.pursuit_lifecycle
    }

    pub const fn without_pursuit_lifecycle(mut self) -> Self {
        self.pursuit_lifecycle = None;
        self
    }
}
