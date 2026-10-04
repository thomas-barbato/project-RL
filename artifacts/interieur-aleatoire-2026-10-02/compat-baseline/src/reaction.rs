use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::skills::TechniqueId;
use crate::world::GridPos;

/// Whether an action is a normal opportunity or was itself produced by a
/// reaction. Reaction-produced actions can never open another reaction chain.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActionOrigin {
    Normal,
    Reaction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReactionTrigger {
    BeforeIncomingAttack,
    AfterMeleeHit,
    BeforeVoluntaryMeleeContactBroken,
    AfterActorEntersCoveredCell,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReactionKind {
    EvasiveStep,
    MeleeParry,
    MeleeInterception,
    RangedOverwatch,
}

/// Reusable effect carried by a prepared reaction. The trigger decides when it
/// may happen; this value decides what it changes once the shared right is
/// consumed.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReactionEffect {
    MoveTo {
        destination: GridPos,
    },
    ReducePhysicalDamage {
        percentage: u8,
    },
    PerformMeleeWeaponAttack,
    PerformControllingMeleeWeaponAttack {
        stability_intensity: u16,
    },
    PerformRangedWeaponAttack {
        slot: u8,
        covered_cells: Vec<GridPos>,
    },
}

/// Optional action produced by a reaction after its primary effect resolves.
/// The originating improvement is retained so events and saves never need to
/// recover behavior from a hard-coded content ID.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ReactionFollowUp {
    MeleeCounterattack { technique: TechniqueId },
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PreparedReaction {
    technique: TechniqueId,
    trigger: ReactionTrigger,
    kind: ReactionKind,
    effect: ReactionEffect,
    trigger_energy_cost: u16,
    follow_up: Option<ReactionFollowUp>,
}

impl PreparedReaction {
    pub fn evasive_step(
        technique: TechniqueId,
        destination: GridPos,
        trigger_energy_cost: u16,
    ) -> Self {
        Self {
            technique,
            trigger: ReactionTrigger::BeforeIncomingAttack,
            kind: ReactionKind::EvasiveStep,
            effect: ReactionEffect::MoveTo { destination },
            trigger_energy_cost,
            follow_up: None,
        }
    }

    pub fn melee_parry(
        technique: TechniqueId,
        physical_reduction_percentage: u8,
        trigger_energy_cost: u16,
    ) -> Result<Self, PreparedReactionError> {
        validate_percentage(physical_reduction_percentage)?;
        Ok(Self {
            technique,
            trigger: ReactionTrigger::AfterMeleeHit,
            kind: ReactionKind::MeleeParry,
            effect: ReactionEffect::ReducePhysicalDamage {
                percentage: physical_reduction_percentage,
            },
            trigger_energy_cost,
            follow_up: None,
        })
    }

    pub fn with_melee_counterattack(mut self, technique: TechniqueId) -> Self {
        self.follow_up = Some(ReactionFollowUp::MeleeCounterattack { technique });
        self
    }

    pub fn melee_interception(technique: TechniqueId) -> Self {
        Self {
            technique,
            trigger: ReactionTrigger::BeforeVoluntaryMeleeContactBroken,
            kind: ReactionKind::MeleeInterception,
            effect: ReactionEffect::PerformMeleeWeaponAttack,
            trigger_energy_cost: 0,
            follow_up: None,
        }
    }

    pub fn controlling_melee_interception(
        technique: TechniqueId,
        stability_intensity: u16,
    ) -> Result<Self, PreparedReactionError> {
        if stability_intensity == 0 {
            return Err(PreparedReactionError::ZeroStabilityIntensity);
        }
        Ok(Self {
            technique,
            trigger: ReactionTrigger::BeforeVoluntaryMeleeContactBroken,
            kind: ReactionKind::MeleeInterception,
            effect: ReactionEffect::PerformControllingMeleeWeaponAttack {
                stability_intensity,
            },
            trigger_energy_cost: 0,
            follow_up: None,
        })
    }

    pub fn ranged_overwatch(
        technique: TechniqueId,
        slot: u8,
        covered_cells: Vec<GridPos>,
    ) -> Result<Self, PreparedReactionError> {
        if covered_cells.is_empty() {
            return Err(PreparedReactionError::EmptyCoveredArea);
        }
        Ok(Self {
            technique,
            trigger: ReactionTrigger::AfterActorEntersCoveredCell,
            kind: ReactionKind::RangedOverwatch,
            effect: ReactionEffect::PerformRangedWeaponAttack {
                slot,
                covered_cells,
            },
            trigger_energy_cost: 0,
            follow_up: None,
        })
    }

    pub const fn technique(&self) -> &TechniqueId {
        &self.technique
    }

    pub const fn trigger(&self) -> ReactionTrigger {
        self.trigger
    }

    pub const fn kind(&self) -> ReactionKind {
        self.kind
    }

    pub const fn effect(&self) -> &ReactionEffect {
        &self.effect
    }

    pub const fn trigger_energy_cost(&self) -> u16 {
        self.trigger_energy_cost
    }

    pub const fn follow_up(&self) -> Option<&ReactionFollowUp> {
        self.follow_up.as_ref()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreparedReactionError {
    ZeroPercentage,
    PercentageAboveHundred(u8),
    EmptyCoveredArea,
    ZeroStabilityIntensity,
}

impl Display for PreparedReactionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroPercentage => write!(formatter, "reaction percentage must be positive"),
            Self::PercentageAboveHundred(percentage) => write!(
                formatter,
                "reaction percentage must not exceed 100, found {percentage}"
            ),
            Self::EmptyCoveredArea => write!(formatter, "reaction covered area must not be empty"),
            Self::ZeroStabilityIntensity => {
                write!(formatter, "reaction Stability intensity must be positive")
            }
        }
    }
}

impl Error for PreparedReactionError {}

const fn validate_percentage(percentage: u8) -> Result<(), PreparedReactionError> {
    if percentage == 0 {
        Err(PreparedReactionError::ZeroPercentage)
    } else if percentage > 100 {
        Err(PreparedReactionError::PercentageAboveHundred(percentage))
    } else {
        Ok(())
    }
}

/// Shared per-actor reaction right. Prepared guards consume this budget; their
/// concrete targeting and effects remain separate data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ReactionBudget {
    available: bool,
}

impl Default for ReactionBudget {
    fn default() -> Self {
        Self { available: true }
    }
}

impl ReactionBudget {
    pub const fn is_available(self) -> bool {
        self.available
    }

    /// A real normal action opens one new reaction window. Menus and rejected
    /// commands must never call this method.
    pub const fn begin_normal_action(&mut self) {
        self.available = true;
    }

    /// Consumes the right once for an opportunity caused by a normal action.
    /// Returning false leaves the budget untouched.
    pub const fn try_consume(&mut self, source: ActionOrigin) -> bool {
        if matches!(source, ActionOrigin::Reaction) || !self.available {
            return false;
        }
        self.available = false;
        true
    }
}

/// Complete reaction state owned by one actor. A prepared guard survives until
/// it triggers or until that actor begins its next accepted normal action.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ReactionState {
    budget: ReactionBudget,
    prepared: Option<PreparedReaction>,
}

impl ReactionState {
    pub const fn is_available(&self) -> bool {
        self.budget.is_available()
    }

    pub const fn prepared(&self) -> Option<&PreparedReaction> {
        self.prepared.as_ref()
    }

    pub fn prepare(&mut self, reaction: PreparedReaction) -> Option<PreparedReaction> {
        self.prepared.replace(reaction)
    }

    /// Starts a normal action and returns the unused guard which just expired,
    /// if any. Callers can therefore publish expiration without hiding it in
    /// presentation code.
    pub fn begin_normal_action(&mut self) -> Option<PreparedReaction> {
        self.budget.begin_normal_action();
        self.prepared.take()
    }

    /// Returns and consumes a matching prepared reaction. An opportunity
    /// created by another reaction and a spent budget leave the guard intact.
    pub fn try_trigger(
        &mut self,
        trigger: ReactionTrigger,
        source: ActionOrigin,
    ) -> Option<PreparedReaction> {
        if self.prepared.as_ref().map(PreparedReaction::trigger) != Some(trigger)
            || !self.budget.try_consume(source)
        {
            return None;
        }
        self.prepared.take()
    }

    pub const fn try_consume_unprepared(&mut self, source: ActionOrigin) -> bool {
        self.budget.try_consume(source)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parry() -> PreparedReaction {
        PreparedReaction::melee_parry("core:parry".parse().unwrap(), 50, 0).unwrap()
    }

    #[test]
    fn one_reaction_is_available_between_normal_actions() {
        let mut budget = ReactionBudget::default();

        assert!(budget.try_consume(ActionOrigin::Normal));
        assert!(!budget.try_consume(ActionOrigin::Normal));
        budget.begin_normal_action();
        assert!(budget.try_consume(ActionOrigin::Normal));
    }

    #[test]
    fn reaction_actions_never_open_reaction_chains() {
        let mut budget = ReactionBudget::default();

        assert!(!budget.try_consume(ActionOrigin::Reaction));
        assert!(budget.is_available());
    }

    #[test]
    fn parry_percentage_is_bounded() {
        let technique: TechniqueId = "core:parry".parse().unwrap();

        assert_eq!(
            PreparedReaction::melee_parry(technique.clone(), 0, 0),
            Err(PreparedReactionError::ZeroPercentage)
        );
        assert_eq!(
            PreparedReaction::melee_parry(technique, 101, 0),
            Err(PreparedReactionError::PercentageAboveHundred(101))
        );
    }

    #[test]
    fn controlling_interception_requires_positive_stability_intensity() {
        let technique: TechniqueId = "core:interception".parse().unwrap();

        assert_eq!(
            PreparedReaction::controlling_melee_interception(technique.clone(), 0),
            Err(PreparedReactionError::ZeroStabilityIntensity)
        );
        assert!(matches!(
            PreparedReaction::controlling_melee_interception(technique, 60)
                .unwrap()
                .effect(),
            ReactionEffect::PerformControllingMeleeWeaponAttack {
                stability_intensity: 60,
            }
        ));
    }

    #[test]
    fn matching_normal_opportunity_consumes_guard_exactly_once() {
        let mut state = ReactionState::default();
        state.prepare(parry());

        assert!(
            state
                .try_trigger(ReactionTrigger::AfterMeleeHit, ActionOrigin::Normal)
                .is_some()
        );
        assert!(!state.is_available());
        assert!(
            state
                .try_trigger(ReactionTrigger::AfterMeleeHit, ActionOrigin::Normal)
                .is_none()
        );
    }

    #[test]
    fn reaction_opportunity_does_not_consume_or_remove_guard() {
        let mut state = ReactionState::default();
        state.prepare(parry());

        assert!(
            state
                .try_trigger(ReactionTrigger::AfterMeleeHit, ActionOrigin::Reaction)
                .is_none()
        );
        assert!(state.is_available());
        assert!(state.prepared().is_some());
    }

    #[test]
    fn next_normal_action_expires_guard_and_refreshes_budget() {
        let mut state = ReactionState::default();
        state.prepare(parry());
        assert!(state.try_consume_unprepared(ActionOrigin::Normal));

        let expired = state.begin_normal_action();

        assert_eq!(
            expired.as_ref().map(PreparedReaction::kind),
            Some(ReactionKind::MeleeParry)
        );
        assert!(state.prepared().is_none());
        assert!(state.is_available());
    }
}
