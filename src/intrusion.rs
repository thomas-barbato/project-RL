//! Deterministic, zone-owned digital access and security state.
//!
//! Rendering never infers access from a glyph. Commands name concrete map
//! positions and the simulation records rights, traces and finite durations.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::NonZeroU64;

use crate::world::GridPos;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SecurityTraceId(NonZeroU64);

impl SecurityTraceId {
    pub const fn from_raw(value: u64) -> Option<Self> {
        match NonZeroU64::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccessRight {
    Read,
    Command,
    ModifyRegister,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccessOrigin {
    Authentic,
    Spoofed,
    Backdoor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceCommand {
    Open,
    Close,
    Disable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DigitalRoutine {
    AutomaticResponse,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IntrusionDirective {
    Interface {
        position: GridPos,
    },
    Command {
        position: GridPos,
        command: DeviceCommand,
    },
    Routine {
        position: GridPos,
        routine: DigitalRoutine,
    },
    Trace {
        trace: SecurityTraceId,
    },
    Subnet {
        positions: Vec<GridPos>,
        command: DeviceCommand,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AccessSession {
    rights: BTreeSet<AccessRight>,
    origin: AccessOrigin,
    expires_on_turn: u64,
    bandwidth_reserved: u16,
}

impl AccessSession {
    pub fn new(
        rights: impl IntoIterator<Item = AccessRight>,
        origin: AccessOrigin,
        expires_on_turn: u64,
        bandwidth_reserved: u16,
    ) -> Result<Self, IntrusionStateError> {
        let rights = rights.into_iter().collect::<BTreeSet<_>>();
        if rights.is_empty() {
            return Err(IntrusionStateError::EmptyRights);
        }
        Ok(Self {
            rights,
            origin,
            expires_on_turn,
            bandwidth_reserved,
        })
    }

    pub fn has(&self, right: AccessRight, turn: u64) -> bool {
        turn < self.expires_on_turn && self.rights.contains(&right)
    }

    pub const fn origin(&self) -> AccessOrigin {
        self.origin
    }

    pub const fn expires_on_turn(&self) -> u64 {
        self.expires_on_turn
    }

    pub const fn bandwidth_reserved(&self) -> u16 {
        self.bandwidth_reserved
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SecurityTrace {
    id: SecurityTraceId,
    source: GridPos,
    created_on_turn: u64,
    audit_on_turn: u64,
    falsified: bool,
    audited: bool,
}

impl SecurityTrace {
    pub const fn id(&self) -> SecurityTraceId {
        self.id
    }

    pub const fn source(&self) -> GridPos {
        self.source
    }

    pub const fn created_on_turn(&self) -> u64 {
        self.created_on_turn
    }

    pub const fn audit_on_turn(&self) -> u64 {
        self.audit_on_turn
    }

    pub const fn is_falsified(&self) -> bool {
        self.falsified
    }

    pub const fn was_audited(&self) -> bool {
        self.audited
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataLot {
    pub source: GridPos,
    pub recorded_on_turn: u64,
    pub extracted_on_turn: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveDeviceControl {
    pub command: DeviceCommand,
    pub expires_on_turn: u64,
    pub recapture_on_turn: u64,
    pub bandwidth_reserved: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveRoutineSuspension {
    pub routine: DigitalRoutine,
    pub expires_on_turn: u64,
    pub protected_until_turn: u64,
    pub active: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct IntrusionState {
    sessions: BTreeMap<GridPos, AccessSession>,
    probed_interfaces: BTreeSet<GridPos>,
    credentials: BTreeSet<GridPos>,
    backdoors: BTreeSet<GridPos>,
    data_lots: Vec<DataLot>,
    traces: BTreeMap<SecurityTraceId, SecurityTrace>,
    failed_attempts: BTreeMap<GridPos, FailedAttemptHardening>,
    device_controls: BTreeMap<GridPos, ActiveDeviceControl>,
    routine_suspensions: BTreeMap<GridPos, ActiveRoutineSuspension>,
    control_locks: BTreeMap<GridPos, ActiveControlLock>,
    next_trace_id: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FailedAttemptHardening {
    pub bonus: u16,
    pub expires_on_turn: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActiveControlLock {
    pub expires_on_turn: u64,
    pub bandwidth_reserved: u16,
}

impl IntrusionState {
    pub fn is_empty(&self) -> bool {
        self.sessions.is_empty()
            && self.probed_interfaces.is_empty()
            && self.credentials.is_empty()
            && self.backdoors.is_empty()
            && self.data_lots.is_empty()
            && self.traces.is_empty()
            && self.failed_attempts.is_empty()
            && self.device_controls.is_empty()
            && self.routine_suspensions.is_empty()
            && self.control_locks.is_empty()
    }

    pub fn session(&self, position: GridPos) -> Option<&AccessSession> {
        self.sessions.get(&position)
    }

    pub fn has_right(&self, position: GridPos, right: AccessRight, turn: u64) -> bool {
        self.session(position)
            .is_some_and(|session| session.has(right, turn))
    }

    pub fn grant_session(
        &mut self,
        position: GridPos,
        session: AccessSession,
    ) -> Option<AccessSession> {
        self.sessions.insert(position, session)
    }

    pub fn mark_probed(&mut self, position: GridPos) {
        self.probed_interfaces.insert(position);
    }

    pub fn was_probed(&self, position: GridPos) -> bool {
        self.probed_interfaces.contains(&position)
    }

    pub fn add_data_lot(&mut self, lot: DataLot) {
        self.credentials.insert(lot.source);
        self.data_lots.push(lot);
    }

    pub fn data_lots(&self) -> &[DataLot] {
        &self.data_lots
    }

    pub fn has_credential(&self, position: GridPos) -> bool {
        self.credentials.contains(&position)
    }

    pub fn backdoors(&self) -> impl Iterator<Item = GridPos> + '_ {
        self.backdoors.iter().copied()
    }

    pub fn has_backdoor(&self, position: GridPos) -> bool {
        self.backdoors.contains(&position)
    }

    pub fn install_backdoor(&mut self, position: GridPos) {
        self.backdoors.insert(position);
    }

    pub fn traces(&self) -> impl Iterator<Item = &SecurityTrace> {
        self.traces.values()
    }

    pub fn trace(&self, id: SecurityTraceId) -> Option<&SecurityTrace> {
        self.traces.get(&id)
    }

    pub fn create_trace(
        &mut self,
        source: GridPos,
        turn: u64,
        audit_delay: u16,
    ) -> SecurityTraceId {
        self.next_trace_id = self.next_trace_id.saturating_add(1).max(1);
        let id = SecurityTraceId(
            NonZeroU64::new(self.next_trace_id).expect("trace sequence remains positive"),
        );
        self.traces.insert(
            id,
            SecurityTrace {
                id,
                source,
                created_on_turn: turn,
                audit_on_turn: turn.saturating_add(u64::from(audit_delay.saturating_sub(1))),
                falsified: false,
                audited: false,
            },
        );
        id
    }

    pub fn falsify_trace(&mut self, id: SecurityTraceId) -> bool {
        let Some(trace) = self.traces.get_mut(&id) else {
            return false;
        };
        if trace.audited || trace.falsified {
            return false;
        }
        trace.falsified = true;
        true
    }

    pub fn due_audits(&mut self, turn: u64) -> Vec<(SecurityTraceId, GridPos, bool)> {
        self.traces
            .values_mut()
            .filter(|trace| !trace.audited && turn >= trace.audit_on_turn)
            .map(|trace| {
                trace.audited = true;
                (trace.id, trace.source, trace.falsified)
            })
            .collect()
    }

    pub fn hardening(&self, position: GridPos, turn: u64) -> u16 {
        self.failed_attempts
            .get(&position)
            .filter(|state| turn < state.expires_on_turn)
            .map_or(0, |state| state.bonus)
    }

    pub fn register_failure(&mut self, position: GridPos, turn: u64, duration: u16) -> u16 {
        let state = self
            .failed_attempts
            .entry(position)
            .or_insert(FailedAttemptHardening {
                bonus: 0,
                expires_on_turn: turn,
            });
        state.bonus = state.bonus.saturating_add(10).min(20);
        state.expires_on_turn = turn.saturating_add(u64::from(duration));
        state.bonus
    }

    pub fn control(&self, position: GridPos) -> Option<ActiveDeviceControl> {
        self.device_controls.get(&position).copied()
    }

    pub fn controls(&self) -> impl Iterator<Item = (GridPos, ActiveDeviceControl)> + '_ {
        self.device_controls
            .iter()
            .map(|(position, control)| (*position, *control))
    }

    pub fn remove_control(&mut self, position: GridPos) -> Option<ActiveDeviceControl> {
        self.device_controls.remove(&position)
    }

    pub fn set_control(
        &mut self,
        position: GridPos,
        control: ActiveDeviceControl,
    ) -> Option<ActiveDeviceControl> {
        self.device_controls.insert(position, control)
    }

    pub fn suspend_routine(
        &mut self,
        position: GridPos,
        suspension: ActiveRoutineSuspension,
    ) -> bool {
        if self.routine_suspensions.contains_key(&position) {
            return false;
        }
        self.routine_suspensions.insert(position, suspension);
        true
    }

    pub fn routine_suspension(&self, position: GridPos) -> Option<ActiveRoutineSuspension> {
        self.routine_suspensions.get(&position).copied()
    }

    pub fn set_control_lock(
        &mut self,
        position: GridPos,
        lock: ActiveControlLock,
    ) -> Option<ActiveControlLock> {
        self.control_locks.insert(position, lock)
    }

    pub fn control_lock(&self, position: GridPos) -> Option<ActiveControlLock> {
        self.control_locks.get(&position).copied()
    }

    pub fn expire(&mut self, turn: u64) -> IntrusionExpirations {
        let mut expired = IntrusionExpirations::default();
        self.sessions.retain(|position, session| {
            let active = turn < session.expires_on_turn;
            if !active {
                expired.bandwidth_released = expired
                    .bandwidth_released
                    .saturating_add(session.bandwidth_reserved);
                expired.sessions.push(*position);
            }
            active
        });
        self.failed_attempts
            .retain(|_, state| turn < state.expires_on_turn);
        let control_locks = &self.control_locks;
        self.device_controls.retain(|position, control| {
            let active = turn < control.expires_on_turn
                || control_locks
                    .get(position)
                    .is_some_and(|lock| turn < lock.expires_on_turn);
            if !active {
                expired.bandwidth_released = expired
                    .bandwidth_released
                    .saturating_add(control.bandwidth_reserved);
                expired.controls.push((*position, control.command));
            }
            active
        });
        self.routine_suspensions.retain(|position, suspension| {
            if suspension.active && turn >= suspension.expires_on_turn {
                suspension.active = false;
                expired.routines.push(*position);
            }
            turn < suspension.protected_until_turn
        });
        self.control_locks.retain(|position, lock| {
            let active = turn < lock.expires_on_turn;
            if !active {
                expired.bandwidth_released = expired
                    .bandwidth_released
                    .saturating_add(lock.bandwidth_reserved);
                expired.locks.push(*position);
            }
            active
        });
        expired
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IntrusionExpirations {
    pub sessions: Vec<GridPos>,
    pub controls: Vec<(GridPos, DeviceCommand)>,
    pub routines: Vec<GridPos>,
    pub locks: Vec<GridPos>,
    pub bandwidth_released: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntrusionStateError {
    EmptyRights,
}

impl Display for IntrusionStateError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyRights => formatter.write_str("an access session needs at least one right"),
        }
    }
}

impl Error for IntrusionStateError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traces_hardening_and_access_expire_deterministically() {
        let at = GridPos::new(2, 3);
        let mut state = IntrusionState::default();
        let _ = state.grant_session(
            at,
            AccessSession::new([AccessRight::Read], AccessOrigin::Spoofed, 5, 1).unwrap(),
        );
        let trace = state.create_trace(at, 1, 5);
        assert_eq!(state.trace(trace).unwrap().audit_on_turn(), 5);
        assert_eq!(state.register_failure(at, 1, 4), 10);
        assert_eq!(state.register_failure(at, 2, 4), 20);
        assert_eq!(state.hardening(at, 3), 20);
        assert_eq!(state.due_audits(4), Vec::new());
        assert_eq!(state.due_audits(5), vec![(trace, at, false)]);
        let expired = state.expire(5);
        assert_eq!(expired.sessions, vec![at]);
        assert_eq!(expired.bandwidth_released, 1);
    }

    #[test]
    fn falsification_preserves_the_trace_but_changes_its_audit_result() {
        let at = GridPos::new(4, 7);
        let mut state = IntrusionState::default();
        let trace = state.create_trace(at, 10, 5);
        assert!(state.falsify_trace(trace));
        assert!(!state.falsify_trace(trace));
        assert_eq!(state.due_audits(14), vec![(trace, at, true)]);
    }
}
