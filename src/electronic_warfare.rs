//! Persistent, deterministic electronic systems and hostile programs.
//!
//! This module owns simulation data only. Rendering adapters may visualize
//! pulses, programs and beacons, but never decide compatibility, propagation
//! or expiry from glyphs.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::NonZeroU64;

use crate::entity::EntityId;
use crate::resources::{EnergyReserve, EnergyReserveError};
use crate::world::{Direction, GridPos};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElectronicSystemProfile {
    digital_defense: u16,
    heat_alert_threshold: u16,
    heat_critical_threshold: u16,
    heat_dissipation_per_phase: u16,
    stored_energy: u16,
}

impl ElectronicSystemProfile {
    pub fn new(
        digital_defense: u16,
        heat_alert_threshold: u16,
        heat_critical_threshold: u16,
        heat_dissipation_per_phase: u16,
        stored_energy: u16,
    ) -> Result<Self, ElectronicSystemError> {
        if heat_alert_threshold == 0
            || heat_critical_threshold == 0
            || heat_alert_threshold > heat_critical_threshold
        {
            return Err(ElectronicSystemError::InvalidHeatThresholds);
        }
        Ok(Self {
            digital_defense,
            heat_alert_threshold,
            heat_critical_threshold,
            heat_dissipation_per_phase,
            stored_energy,
        })
    }

    pub const fn digital_defense(self) -> u16 {
        self.digital_defense
    }

    pub const fn heat_alert_threshold(self) -> u16 {
        self.heat_alert_threshold
    }

    pub const fn heat_critical_threshold(self) -> u16 {
        self.heat_critical_threshold
    }

    pub const fn heat_dissipation_per_phase(self) -> u16 {
        self.heat_dissipation_per_phase
    }

    pub const fn stored_energy(self) -> u16 {
        self.stored_energy
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ElectronicSystemState {
    profile: ElectronicSystemProfile,
    heat: u16,
    stored_energy: u16,
}

impl ElectronicSystemState {
    pub const fn new(profile: ElectronicSystemProfile) -> Self {
        Self {
            profile,
            heat: 0,
            stored_energy: profile.stored_energy,
        }
    }

    pub const fn profile(self) -> ElectronicSystemProfile {
        self.profile
    }

    pub const fn digital_defense(self) -> u16 {
        self.profile.digital_defense
    }

    pub const fn heat(self) -> u16 {
        self.heat
    }

    pub const fn stored_energy(self) -> u16 {
        self.stored_energy
    }

    pub fn add_heat(&mut self, amount: u16) -> u16 {
        let before = self.heat;
        self.heat = self.heat.saturating_add(amount);
        self.heat.saturating_sub(before)
    }

    pub fn dissipate(&mut self, inhibition: u16) -> u16 {
        let amount = self
            .profile
            .heat_dissipation_per_phase
            .saturating_sub(inhibition)
            .min(self.heat);
        self.heat -= amount;
        amount
    }

    /// Removes energy from the target storage at arming time. Cancellation
    /// deliberately does not return it, preventing save/rearm duplication.
    pub fn reserve_stored_energy(&mut self, amount: u16) -> bool {
        if self.stored_energy < amount {
            return false;
        }
        self.stored_energy -= amount;
        true
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectronicChannel {
    OpticalSensor,
    ThermalSensor,
    ControlLink,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ElectronicDirective {
    Pulse {
        direction: Option<Direction>,
    },
    Target {
        target: EntityId,
    },
    Jam {
        channel: ElectronicChannel,
    },
    Purge {
        target: EntityId,
        program: HostileProgramId,
    },
    Cascade {
        targets: Vec<EntityId>,
    },
    DeployBeacon {
        position: GridPos,
    },
    ActivateBeacon {
        beacon: EntityId,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HostileProgramId(NonZeroU64);

impl HostileProgramId {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InfectionCampaignId(NonZeroU64);

impl InfectionCampaignId {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostileProgramKind {
    Overheat {
        heat_per_tick: u16,
        dissipation_penalty: u16,
    },
    Infection {
        campaign: InfectionCampaignId,
        thermal_damage_per_tick: u16,
    },
    Implosion {
        reserved_energy: u16,
        detonate_on_turn: u64,
        physical_damage: u16,
        thermal_damage: u16,
        radius: u16,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HostileProgram {
    id: HostileProgramId,
    source: EntityId,
    target: EntityId,
    strength: u16,
    installed_on_turn: u64,
    expires_on_turn: u64,
    ticks_remaining: u16,
    transmission_pending: bool,
    kind: HostileProgramKind,
}

impl HostileProgram {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        id: HostileProgramId,
        source: EntityId,
        target: EntityId,
        strength: u16,
        installed_on_turn: u64,
        expires_on_turn: u64,
        ticks_remaining: u16,
        transmission_pending: bool,
        kind: HostileProgramKind,
    ) -> Self {
        Self {
            id,
            source,
            target,
            strength,
            installed_on_turn,
            expires_on_turn,
            ticks_remaining,
            transmission_pending,
            kind,
        }
    }

    pub const fn id(self) -> HostileProgramId {
        self.id
    }

    pub const fn source(self) -> EntityId {
        self.source
    }

    pub const fn target(self) -> EntityId {
        self.target
    }

    pub const fn strength(self) -> u16 {
        self.strength
    }

    pub const fn installed_on_turn(self) -> u64 {
        self.installed_on_turn
    }

    pub const fn expires_on_turn(self) -> u64 {
        self.expires_on_turn
    }

    pub const fn ticks_remaining(self) -> u16 {
        self.ticks_remaining
    }

    pub const fn kind(self) -> HostileProgramKind {
        self.kind
    }

    pub const fn transmission_pending(self) -> bool {
        self.transmission_pending
    }

    pub(crate) fn consume_tick(&mut self) {
        self.ticks_remaining = self.ticks_remaining.saturating_sub(1);
    }

    pub(crate) fn consume_transmission(&mut self) {
        self.transmission_pending = false;
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InfectionCampaign {
    id: InfectionCampaignId,
    source: EntityId,
    strength: u16,
    maximum_hosts: u8,
    transmissions_per_host: u8,
    propagation_range: u16,
    thermal_damage_per_tick: u16,
    ticks_per_host: u16,
    expires_on_turn: u64,
    hosts: BTreeSet<EntityId>,
    attempted: BTreeSet<EntityId>,
}

impl InfectionCampaign {
    #[allow(clippy::too_many_arguments)]
    fn new(
        id: InfectionCampaignId,
        source: EntityId,
        initial_host: EntityId,
        strength: u16,
        maximum_hosts: u8,
        transmissions_per_host: u8,
        propagation_range: u16,
        thermal_damage_per_tick: u16,
        ticks_per_host: u16,
        expires_on_turn: u64,
    ) -> Self {
        Self {
            id,
            source,
            strength,
            maximum_hosts,
            transmissions_per_host,
            propagation_range,
            thermal_damage_per_tick,
            ticks_per_host,
            expires_on_turn,
            hosts: [initial_host].into_iter().collect(),
            attempted: [initial_host].into_iter().collect(),
        }
    }

    pub const fn id(&self) -> InfectionCampaignId {
        self.id
    }

    pub const fn source(&self) -> EntityId {
        self.source
    }

    pub const fn strength(&self) -> u16 {
        self.strength
    }

    pub const fn maximum_hosts(&self) -> u8 {
        self.maximum_hosts
    }

    pub const fn transmissions_per_host(&self) -> u8 {
        self.transmissions_per_host
    }

    pub const fn propagation_range(&self) -> u16 {
        self.propagation_range
    }

    pub const fn thermal_damage_per_tick(&self) -> u16 {
        self.thermal_damage_per_tick
    }

    pub const fn ticks_per_host(&self) -> u16 {
        self.ticks_per_host
    }

    pub const fn expires_on_turn(&self) -> u64 {
        self.expires_on_turn
    }

    pub fn hosts(&self) -> impl Iterator<Item = EntityId> + '_ {
        self.hosts.iter().copied()
    }

    pub fn was_attempted(&self, target: EntityId) -> bool {
        self.attempted.contains(&target)
    }

    pub fn may_add_host(&self) -> bool {
        self.hosts.len() < usize::from(self.maximum_hosts)
    }

    pub(crate) fn record_attempt(&mut self, target: EntityId, succeeded: bool) {
        self.attempted.insert(target);
        if succeeded {
            self.hosts.insert(target);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JammingField {
    pub source: EntityId,
    pub center: GridPos,
    pub channel: ElectronicChannel,
    pub radius: u16,
    pub penalty: u16,
    pub expires_on_turn: u64,
    pub energy_per_phase: u16,
    pub heat_per_phase: u16,
    pub bandwidth_reserved: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SaturationBeacon {
    pub entity: EntityId,
    pub owner: EntityId,
    pub damage: u16,
    pub radius: u16,
    pub remaining_phases: u16,
    pub battery: EnergyReserve,
    pub energy_per_phase: u16,
    pub active: bool,
    pub activation_energy: u16,
    pub activation_bandwidth: u16,
    pub activation_link_range: u16,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ElectronicWarfareState {
    programs: BTreeMap<HostileProgramId, HostileProgram>,
    campaigns: BTreeMap<InfectionCampaignId, InfectionCampaign>,
    jamming: Option<JammingField>,
    beacons: BTreeMap<EntityId, SaturationBeacon>,
    next_program_id: u64,
    next_campaign_id: u64,
}

impl ElectronicWarfareState {
    pub fn is_empty(&self) -> bool {
        self.programs.is_empty()
            && self.campaigns.is_empty()
            && self.jamming.is_none()
            && self.beacons.is_empty()
    }

    pub fn programs(&self) -> impl Iterator<Item = &HostileProgram> {
        self.programs.values()
    }

    pub fn program(&self, id: HostileProgramId) -> Option<HostileProgram> {
        self.programs.get(&id).copied()
    }

    pub fn programs_on(&self, target: EntityId) -> impl Iterator<Item = &HostileProgram> {
        self.programs
            .values()
            .filter(move |program| program.target == target)
    }

    pub fn has_program_family(
        &self,
        target: EntityId,
        predicate: impl Fn(HostileProgramKind) -> bool,
    ) -> bool {
        self.programs_on(target)
            .any(|program| predicate(program.kind))
    }

    pub(crate) fn add_program(
        &mut self,
        source: EntityId,
        target: EntityId,
        strength: u16,
        installed_on_turn: u64,
        expires_on_turn: u64,
        ticks: u16,
        transmission_pending: bool,
        kind: HostileProgramKind,
    ) -> HostileProgramId {
        self.next_program_id = self.next_program_id.saturating_add(1).max(1);
        let id = HostileProgramId(
            NonZeroU64::new(self.next_program_id).expect("program sequence remains positive"),
        );
        self.programs.insert(
            id,
            HostileProgram::new(
                id,
                source,
                target,
                strength,
                installed_on_turn,
                expires_on_turn,
                ticks,
                transmission_pending,
                kind,
            ),
        );
        id
    }

    pub(crate) fn add_infection_campaign(
        &mut self,
        source: EntityId,
        initial_host: EntityId,
        strength: u16,
        maximum_hosts: u8,
        transmissions_per_host: u8,
        propagation_range: u16,
        thermal_damage_per_tick: u16,
        ticks_per_host: u16,
        expires_on_turn: u64,
    ) -> InfectionCampaignId {
        self.next_campaign_id = self.next_campaign_id.saturating_add(1).max(1);
        let id = InfectionCampaignId(
            NonZeroU64::new(self.next_campaign_id).expect("campaign sequence remains positive"),
        );
        self.campaigns.insert(
            id,
            InfectionCampaign::new(
                id,
                source,
                initial_host,
                strength,
                maximum_hosts,
                transmissions_per_host,
                propagation_range,
                thermal_damage_per_tick,
                ticks_per_host,
                expires_on_turn,
            ),
        );
        id
    }

    pub fn campaign(&self, id: InfectionCampaignId) -> Option<&InfectionCampaign> {
        self.campaigns.get(&id)
    }

    pub(crate) fn campaign_mut(
        &mut self,
        id: InfectionCampaignId,
    ) -> Option<&mut InfectionCampaign> {
        self.campaigns.get_mut(&id)
    }

    pub(crate) fn remove_campaign(&mut self, id: InfectionCampaignId) -> Option<InfectionCampaign> {
        self.campaigns.remove(&id)
    }

    pub(crate) fn program_mut(&mut self, id: HostileProgramId) -> Option<&mut HostileProgram> {
        self.programs.get_mut(&id)
    }

    pub(crate) fn remove_program(&mut self, id: HostileProgramId) -> Option<HostileProgram> {
        self.programs.remove(&id)
    }

    pub const fn jamming(&self) -> Option<JammingField> {
        self.jamming
    }

    pub(crate) fn replace_jamming(&mut self, field: JammingField) -> Option<JammingField> {
        self.jamming.replace(field)
    }

    pub(crate) fn take_jamming(&mut self) -> Option<JammingField> {
        self.jamming.take()
    }

    pub fn beacons(&self) -> impl Iterator<Item = &SaturationBeacon> {
        self.beacons.values()
    }

    pub fn beacon(&self, entity: EntityId) -> Option<SaturationBeacon> {
        self.beacons.get(&entity).copied()
    }

    pub(crate) fn beacon_mut(&mut self, entity: EntityId) -> Option<&mut SaturationBeacon> {
        self.beacons.get_mut(&entity)
    }

    pub(crate) fn add_beacon(&mut self, beacon: SaturationBeacon) {
        self.beacons.insert(beacon.entity, beacon);
    }

    pub(crate) fn remove_beacon(&mut self, entity: EntityId) -> Option<SaturationBeacon> {
        self.beacons.remove(&entity)
    }

    pub(crate) fn program_ids(&self) -> Vec<HostileProgramId> {
        self.programs.keys().copied().collect()
    }

    pub(crate) fn beacon_ids(&self) -> Vec<EntityId> {
        self.beacons.keys().copied().collect()
    }

    pub(crate) fn retain_campaigns(&mut self, turn: u64) {
        self.campaigns
            .retain(|_, campaign| turn < campaign.expires_on_turn);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElectronicSystemError {
    InvalidHeatThresholds,
    InvalidBattery(EnergyReserveError),
}

impl From<EnergyReserveError> for ElectronicSystemError {
    fn from(error: EnergyReserveError) -> Self {
        Self::InvalidBattery(error)
    }
}

impl Display for ElectronicSystemError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidHeatThresholds => {
                formatter.write_str("electronic heat thresholds must be positive and ordered")
            }
            Self::InvalidBattery(error) => Display::fmt(error, formatter),
        }
    }
}

impl Error for ElectronicSystemError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn electronic_heat_and_storage_are_independent_finite_reserves() {
        let profile = ElectronicSystemProfile::new(55, 80, 100, 5, 30).unwrap();
        let mut state = ElectronicSystemState::new(profile);
        assert_eq!(state.add_heat(15), 15);
        assert_eq!(state.dissipate(3), 2);
        assert_eq!(state.heat(), 13);
        assert!(state.reserve_stored_energy(20));
        assert_eq!(state.stored_energy(), 10);
        assert!(!state.reserve_stored_energy(20));
    }

    #[test]
    fn infection_campaign_never_forgets_attempted_or_successful_hosts() {
        let source = EntityId::new(1);
        let first = EntityId::new(2);
        let failed = EntityId::new(3);
        let second = EntityId::new(4);
        let id = InfectionCampaignId::from_raw_for_test(1);
        let mut campaign = InfectionCampaign::new(id, source, first, 65, 3, 1, 2, 4, 3, 6);
        campaign.record_attempt(failed, false);
        campaign.record_attempt(second, true);
        assert!(campaign.was_attempted(failed));
        assert_eq!(campaign.hosts().collect::<Vec<_>>(), vec![first, second]);
        assert!(campaign.may_add_host());
    }
}

#[cfg(test)]
impl InfectionCampaignId {
    const fn from_raw_for_test(value: u64) -> Self {
        Self(NonZeroU64::new(value).expect("test campaign ID is positive"))
    }
}
