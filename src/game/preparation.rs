use crate::entity::EntityId;
use crate::skills::TechniqueId;
use crate::time::TimeUnits;
use crate::world::GridPos;

/// Read-only presentation of the player's persisted multi-UT technique state.
/// Commands remain the only way to advance or abandon it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TechniquePreparationView<'a> {
    pub(super) technique: &'a TechniqueId,
    pub(super) targets: &'a [EntityId],
    pub(super) target_at: Option<GridPos>,
    pub(super) remaining_steps: TimeUnits,
}

impl<'a> TechniquePreparationView<'a> {
    pub const fn technique(self) -> &'a TechniqueId {
        self.technique
    }

    pub const fn targets(self) -> &'a [EntityId] {
        self.targets
    }

    pub const fn target_at(self) -> Option<GridPos> {
        self.target_at
    }

    pub const fn remaining_steps(self) -> TimeUnits {
        self.remaining_steps
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreparationCancellationReason {
    DifferentAction,
    TargetUnavailable,
    Disrupted,
}
