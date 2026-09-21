use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::NonZeroU16;

/// Positive simulation duration. Rendering time never enters this type.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub struct TimeUnits(NonZeroU16);

impl TimeUnits {
    pub const ONE: Self = Self(NonZeroU16::MIN);

    pub const fn new(value: u16) -> Result<Self, TimeUnitsError> {
        match NonZeroU16::new(value) {
            Some(value) => Ok(Self(value)),
            None => Err(TimeUnitsError::Zero),
        }
    }

    pub const fn get(self) -> u16 {
        self.0.get()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimeUnitsError {
    Zero,
}

impl Display for TimeUnitsError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero => write!(
                formatter,
                "a simulation duration must be at least one time unit"
            ),
        }
    }
}

impl Error for TimeUnitsError {}

/// Semantic intent of a normal action. Recovery rules inspect intent rather
/// than concrete command or content IDs, so mods can add new offensive and
/// support actions without teaching the scheduler their names.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ActionKind {
    Offensive,
    #[default]
    Support,
}

impl ActionKind {
    pub const fn is_offensive(self) -> bool {
        matches!(self, Self::Offensive)
    }
}

/// A recovery window measured in the actor's next accepted normal actions.
/// Rejected commands never reach `advance`, which keeps recovery transactional.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ActionRecovery {
    remaining_actions: TimeUnits,
}

impl ActionRecovery {
    pub const fn new(duration: TimeUnits) -> Self {
        Self {
            remaining_actions: duration,
        }
    }

    pub const fn remaining_actions(self) -> TimeUnits {
        self.remaining_actions
    }

    pub fn advance(self) -> RecoveryAdvance {
        let remaining = self.remaining_actions.get();
        if remaining == 1 {
            RecoveryAdvance::Complete
        } else {
            RecoveryAdvance::Recovering(Self::new(
                TimeUnits::new(remaining - 1)
                    .expect("subtracting one from a duration above one stays positive"),
            ))
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RecoveryAdvance {
    Recovering(ActionRecovery),
    Complete,
}

/// Cooldown advanced by eligible environment phases. The phase which arms it
/// is excluded, so a duration of two armed at C blocks launches at C+1 and
/// C+2, then completes for C+3.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EnvironmentCooldown {
    armed_on_turn: u64,
    remaining_phases: TimeUnits,
}

impl EnvironmentCooldown {
    pub const fn new(armed_on_turn: u64, duration: TimeUnits) -> Self {
        Self {
            armed_on_turn,
            remaining_phases: duration,
        }
    }

    pub const fn remaining_phases(self) -> TimeUnits {
        self.remaining_phases
    }

    pub fn advance(self, environment_turn: u64) -> CooldownAdvance {
        if environment_turn <= self.armed_on_turn {
            return CooldownAdvance::Arming(self);
        }
        let remaining = self.remaining_phases.get();
        if remaining == 1 {
            CooldownAdvance::Complete
        } else {
            CooldownAdvance::Cooling(Self {
                armed_on_turn: self.armed_on_turn,
                remaining_phases: TimeUnits::new(remaining - 1)
                    .expect("subtracting one from a cooldown above one stays positive"),
            })
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CooldownAdvance {
    Arming(EnvironmentCooldown),
    Cooling(EnvironmentCooldown),
    Complete,
}

/// Persistent deadline for an announced delayed effect. The arming cycle is
/// excluded naturally: a delay of one armed at C is first due at C+1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnnouncedDeadline {
    armed_at: u64,
    due_at: u64,
    triggered: bool,
}

impl AnnouncedDeadline {
    pub fn new(armed_at: u64, delay: TimeUnits) -> Result<Self, DeadlineError> {
        let due_at = armed_at
            .checked_add(u64::from(delay.get()))
            .ok_or(DeadlineError::TimeOverflow)?;
        Ok(Self {
            armed_at,
            due_at,
            triggered: false,
        })
    }

    pub const fn armed_at(self) -> u64 {
        self.armed_at
    }

    pub const fn due_at(self) -> u64 {
        self.due_at
    }

    pub const fn has_triggered(self) -> bool {
        self.triggered
    }

    /// Returns true exactly once, on or after the due cycle.
    pub fn trigger_if_due(&mut self, current_time: u64) -> bool {
        if self.triggered || current_time < self.due_at {
            return false;
        }
        self.triggered = true;
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeadlineError {
    TimeOverflow,
}

impl Display for DeadlineError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TimeOverflow => write!(formatter, "announced deadline exceeds simulation time"),
        }
    }
}

impl Error for DeadlineError {}

/// One action split into a positive number of preparation steps followed by
/// its final execution step. The payload remains owned by the simulation so a
/// client cannot manufacture a completed action by reopening a menu.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ActionPreparation<T> {
    payload: T,
    remaining_steps: TimeUnits,
}

impl<T> ActionPreparation<T> {
    pub const fn new(payload: T, preparation_steps: TimeUnits) -> Self {
        Self {
            payload,
            remaining_steps: preparation_steps,
        }
    }

    pub const fn payload(&self) -> &T {
        &self.payload
    }

    pub const fn remaining_steps(&self) -> TimeUnits {
        self.remaining_steps
    }

    /// Consumes one preparation step. A preparation with one step remaining
    /// becomes ready; longer preparations return their persisted next state.
    pub fn advance(self) -> PreparationAdvance<T> {
        let remaining = self.remaining_steps.get();
        if remaining == 1 {
            PreparationAdvance::Ready(self.payload)
        } else {
            PreparationAdvance::Preparing(Self {
                payload: self.payload,
                remaining_steps: TimeUnits::new(remaining - 1)
                    .expect("subtracting one from a duration above one stays positive"),
            })
        }
    }

    pub fn cancel(self) -> T {
        self.payload
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreparationAdvance<T> {
    Preparing(ActionPreparation<T>),
    Ready(T),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn durations_are_positive_and_use_integer_simulation_units() {
        assert_eq!(TimeUnits::new(0), Err(TimeUnitsError::Zero));
        assert_eq!(TimeUnits::ONE.get(), 1);
        assert_eq!(TimeUnits::new(3).unwrap().get(), 3);
    }

    #[test]
    fn action_kinds_expose_offensive_intent_without_content_ids() {
        assert!(ActionKind::Offensive.is_offensive());
        assert!(!ActionKind::Support.is_offensive());
    }

    #[test]
    fn action_recovery_advances_only_through_explicit_accepted_actions() {
        let recovery = ActionRecovery::new(TimeUnits::new(2).unwrap());
        let RecoveryAdvance::Recovering(recovery) = recovery.advance() else {
            panic!("two recovery actions must not complete after one advancement");
        };
        assert_eq!(recovery.remaining_actions().get(), 1);
        assert_eq!(recovery.advance(), RecoveryAdvance::Complete);
    }

    #[test]
    fn announced_deadline_excludes_arming_cycle_and_triggers_once() {
        let mut deadline = AnnouncedDeadline::new(12, TimeUnits::ONE).unwrap();

        assert_eq!(deadline.armed_at(), 12);
        assert_eq!(deadline.due_at(), 13);
        assert!(!deadline.trigger_if_due(12));
        assert!(deadline.trigger_if_due(13));
        assert!(!deadline.trigger_if_due(13));
        assert!(!deadline.trigger_if_due(14));
        assert!(deadline.has_triggered());
    }

    #[test]
    fn environment_cooldown_excludes_arming_phase_and_completes_after_duration() {
        let cooldown = EnvironmentCooldown::new(7, TimeUnits::new(2).unwrap());
        let CooldownAdvance::Arming(cooldown) = cooldown.advance(7) else {
            panic!("arming phase must not consume cooldown")
        };
        let CooldownAdvance::Cooling(cooldown) = cooldown.advance(8) else {
            panic!("first eligible phase must leave one phase")
        };
        assert_eq!(cooldown.remaining_phases().get(), 1);
        assert_eq!(cooldown.advance(9), CooldownAdvance::Complete);
    }

    #[test]
    fn deadline_creation_rejects_clock_overflow() {
        assert_eq!(
            AnnouncedDeadline::new(u64::MAX, TimeUnits::ONE),
            Err(DeadlineError::TimeOverflow)
        );
    }

    #[test]
    fn action_preparation_preserves_payload_until_the_exact_final_step() {
        let preparation = ActionPreparation::new("scan", TimeUnits::new(2).unwrap());
        assert_eq!(preparation.payload(), &"scan");
        assert_eq!(preparation.remaining_steps().get(), 2);

        let PreparationAdvance::Preparing(preparation) = preparation.advance() else {
            panic!("two preparation steps must not complete after the first one");
        };
        assert_eq!(preparation.remaining_steps().get(), 1);
        assert_eq!(preparation.advance(), PreparationAdvance::Ready("scan"));
    }

    #[test]
    fn cancelling_a_preparation_returns_its_unmodified_payload() {
        let preparation = ActionPreparation::new(42, TimeUnits::ONE);
        assert_eq!(preparation.cancel(), 42);
    }
}
