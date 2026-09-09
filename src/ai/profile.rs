#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AiBehavior {
    Idle,
    Hunter,
    Sentry,
    Skirmisher,
}

/// Data-shaped AI parameters, suitable for later content definitions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AiProfile {
    pub behavior: AiBehavior,
    pub perception_radius: u16,
    pub preferred_attack_slot: u8,
    pub maximum_path_search: usize,
    pub preferred_minimum_distance: u16,
}

impl AiProfile {
    pub const fn idle() -> Self {
        Self {
            behavior: AiBehavior::Idle,
            perception_radius: 0,
            preferred_attack_slot: 0,
            maximum_path_search: 0,
            preferred_minimum_distance: 0,
        }
    }

    pub const fn hunter(perception_radius: u16, preferred_attack_slot: u8) -> Self {
        Self {
            behavior: AiBehavior::Hunter,
            perception_radius,
            preferred_attack_slot,
            maximum_path_search: 2_048,
            preferred_minimum_distance: 0,
        }
    }

    pub const fn sentry(perception_radius: u16, preferred_attack_slot: u8) -> Self {
        Self {
            behavior: AiBehavior::Sentry,
            perception_radius,
            preferred_attack_slot,
            maximum_path_search: 0,
            preferred_minimum_distance: 0,
        }
    }

    pub const fn skirmisher(
        perception_radius: u16,
        preferred_attack_slot: u8,
        preferred_minimum_distance: u16,
    ) -> Self {
        Self {
            behavior: AiBehavior::Skirmisher,
            perception_radius,
            preferred_attack_slot,
            maximum_path_search: 2_048,
            preferred_minimum_distance,
        }
    }
}
