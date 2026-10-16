use std::collections::{BTreeMap, btree_map::Entry};

use crate::entity::EntityId;

use super::{StatusDefinition, StatusId, StatusStacking};

/// Caster power is sampled on application; target defenses stay live at each tick.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StatusPower {
    pub base_damage: u16,
    pub numerator: u64,
    pub denominator: u64,
    pub applied_on_turn: u64,
}

#[derive(Clone, PartialEq, Eq)]
pub struct StatusInstance {
    pub definition: StatusId,
    pub stacks: u16,
    pub remaining_turns: Option<u16>,
    pub source: Option<EntityId>,
    pub power: Option<StatusPower>,
}
thread_local! { static LEGACY_STATUS_ENCODING: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
pub(crate) fn with_legacy_status_encoding<T>(operation: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset {
        fn drop(&mut self) {
            LEGACY_STATUS_ENCODING.set(self.0);
        }
    }
    let _reset = Reset(LEGACY_STATUS_ENCODING.replace(true));
    crate::game::game_state::agent_runtime::with_legacy_encoding(operation)
}
#[derive(serde::Serialize, serde::Deserialize)]
struct LegacyInstance {
    definition: StatusId,
    stacks: u16,
    remaining_turns: Option<u16>,
    source: Option<EntityId>,
}
#[derive(serde::Serialize, serde::Deserialize)]
struct PoweredInstance {
    definition: StatusId,
    stacks: u16,
    remaining_turns: Option<u16>,
    source: Option<EntityId>,
    #[serde(default)]
    power: Option<StatusPower>,
}
impl serde::Serialize for StatusInstance {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if LEGACY_STATUS_ENCODING.get() && !serializer.is_human_readable() {
            LegacyInstance {
                definition: self.definition.clone(),
                stacks: self.stacks,
                remaining_turns: self.remaining_turns,
                source: self.source,
            }
            .serialize(serializer)
        } else {
            PoweredInstance {
                definition: self.definition.clone(),
                stacks: self.stacks,
                remaining_turns: self.remaining_turns,
                source: self.source,
                power: self.power,
            }
            .serialize(serializer)
        }
    }
}
impl<'de> serde::Deserialize<'de> for StatusInstance {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        if LEGACY_STATUS_ENCODING.get() && !deserializer.is_human_readable() {
            let s = LegacyInstance::deserialize(deserializer)?;
            Ok(Self {
                definition: s.definition,
                stacks: s.stacks,
                remaining_turns: s.remaining_turns,
                source: s.source,
                power: None,
            })
        } else {
            let s = PoweredInstance::deserialize(deserializer)?;
            Ok(Self {
                definition: s.definition,
                stacks: s.stacks,
                remaining_turns: s.remaining_turns,
                source: s.source,
                power: s.power,
            })
        }
    }
}
impl std::fmt::Debug for StatusInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("StatusInstance");
        s.field("definition", &self.definition)
            .field("stacks", &self.stacks)
            .field("remaining_turns", &self.remaining_turns)
            .field("source", &self.source);
        if let Some(power) = self.power {
            s.field("power", &power);
        }
        s.finish()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatusApplyOutcome {
    pub previous_stacks: u16,
    pub stacks: u16,
    pub remaining_turns: Option<u16>,
    pub kind: StatusApplyKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusApplyKind {
    Applied,
    Replaced,
    Refreshed,
    Stacked,
    Ignored,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StatusSet {
    instances: BTreeMap<StatusId, StatusInstance>,
}

impl StatusSet {
    pub fn get(&self, id: &StatusId) -> Option<&StatusInstance> {
        self.instances.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &StatusInstance> {
        self.instances.values()
    }

    pub(crate) fn set_duration(&mut self, id: &StatusId, duration: u16) {
        if let Some(instance) = self.instances.get_mut(id) {
            instance.remaining_turns = Some(duration);
        }
    }
    pub(crate) fn set_power(&mut self, id: &StatusId, power: StatusPower, duration: u16) {
        if let Some(instance) = self.instances.get_mut(id) {
            instance.power = Some(power);
            instance.remaining_turns = Some(duration);
        }
    }

    pub(crate) fn remove(&mut self, id: &StatusId) -> bool {
        self.instances.remove(id).is_some()
    }

    pub fn apply(
        &mut self,
        definition: &StatusDefinition,
        incoming_stacks: u16,
        source: Option<EntityId>,
    ) -> StatusApplyOutcome {
        let incoming_stacks = incoming_stacks.max(1);
        let maximum_stacks = definition.stacking().maximum_stacks();
        let initial_stacks = incoming_stacks.min(maximum_stacks);
        let instance = match self.instances.entry(definition.id().clone()) {
            Entry::Vacant(entry) => {
                let instance = entry.insert(StatusInstance {
                    definition: definition.id().clone(),
                    stacks: initial_stacks,
                    remaining_turns: definition.duration_turns(),
                    source,
                    power: None,
                });
                return StatusApplyOutcome {
                    previous_stacks: 0,
                    stacks: instance.stacks,
                    remaining_turns: instance.remaining_turns,
                    kind: StatusApplyKind::Applied,
                };
            }
            Entry::Occupied(entry) => entry.into_mut(),
        };
        let previous_stacks = instance.stacks;

        let kind = match definition.stacking() {
            StatusStacking::KeepExisting => StatusApplyKind::Ignored,
            StatusStacking::Replace => {
                instance.stacks = initial_stacks;
                instance.remaining_turns = definition.duration_turns();
                instance.source = source;
                StatusApplyKind::Replaced
            }
            StatusStacking::RefreshDuration => {
                instance.remaining_turns = definition.duration_turns();
                instance.source = source;
                StatusApplyKind::Refreshed
            }
            StatusStacking::AddStacks {
                maximum_stacks,
                refresh_duration,
            } => {
                instance.stacks = instance
                    .stacks
                    .saturating_add(incoming_stacks)
                    .min(maximum_stacks);
                if refresh_duration {
                    instance.remaining_turns = definition.duration_turns();
                }
                instance.source = source;
                StatusApplyKind::Stacked
            }
        };

        if kind != StatusApplyKind::Ignored {
            instance.power = None;
        }
        StatusApplyOutcome {
            previous_stacks,
            stacks: instance.stacks,
            remaining_turns: instance.remaining_turns,
            kind,
        }
    }

    /// Decrements one finite duration and removes the instance on zero.
    pub(crate) fn elapse_one_turn(&mut self, id: &StatusId) -> bool {
        let should_expire = self.instances.get_mut(id).is_some_and(|instance| {
            let Some(remaining) = &mut instance.remaining_turns else {
                return false;
            };
            *remaining = remaining.saturating_sub(1);
            *remaining == 0
        });
        if should_expire {
            self.instances.remove(id);
        }
        should_expire
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::status::{StatusHook, StatusStacking};

    fn definition(stacking: StatusStacking) -> StatusDefinition {
        let id = "core:test"
            .parse()
            .unwrap_or_else(|error| panic!("valid status ID rejected: {error}"));
        StatusDefinition::new(id, Some(3), stacking, Vec::<StatusHook>::new())
            .unwrap_or_else(|error| panic!("valid definition rejected: {error}"))
    }

    #[test]
    fn different_timed_effects_have_independent_lifetimes_and_no_numeric_limit_of_two() {
        let mut statuses = StatusSet::default();
        let definitions = (1..=5)
            .map(|duration| {
                StatusDefinition::new(
                    format!("core:timed_{duration}").parse().unwrap(),
                    Some(duration),
                    StatusStacking::RefreshDuration,
                    vec![],
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
        for definition in &definitions {
            for _ in 0..4 {
                statuses.apply(definition, 9, None);
            }
        }
        assert_eq!(statuses.iter().count(), 5);
        assert!(statuses.iter().all(|status| status.stacks == 1));
        for turn in 1..=5 {
            for definition in &definitions {
                statuses.elapse_one_turn(definition.id());
            }
            assert_eq!(statuses.iter().count(), 5 - turn);
        }
    }

    #[test]
    fn stacks_are_capped_and_duration_is_refreshed_by_policy() {
        let definition = definition(StatusStacking::AddStacks {
            maximum_stacks: 3,
            refresh_duration: true,
        });
        let mut statuses = StatusSet::default();

        statuses.apply(&definition, 2, None);
        assert!(!statuses.elapse_one_turn(definition.id()));
        let reapplied = statuses.apply(&definition, 2, None);

        assert_eq!(reapplied.previous_stacks, 2);
        assert_eq!(reapplied.stacks, 3);
        assert_eq!(reapplied.remaining_turns, Some(3));
    }

    #[test]
    fn finite_status_expires_after_its_exact_number_of_ticks() {
        let definition = definition(StatusStacking::RefreshDuration);
        let mut statuses = StatusSet::default();
        statuses.apply(&definition, 1, None);

        assert!(!statuses.elapse_one_turn(definition.id()));
        assert!(!statuses.elapse_one_turn(definition.id()));
        assert!(statuses.elapse_one_turn(definition.id()));
        assert!(statuses.get(definition.id()).is_none());
    }

    #[test]
    fn keep_existing_never_refreshes_an_active_duration() {
        let definition = definition(StatusStacking::KeepExisting);
        let mut statuses = StatusSet::default();
        let first = statuses.apply(&definition, 1, None);
        assert_eq!(first.kind, StatusApplyKind::Applied);
        assert!(!statuses.elapse_one_turn(definition.id()));

        let repeated = statuses.apply(&definition, 1, None);

        assert_eq!(repeated.kind, StatusApplyKind::Ignored);
        assert_eq!(repeated.remaining_turns, Some(2));
    }
}
