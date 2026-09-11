use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::ai::AiProfile;
use crate::combat::{AttackProfile, ResistanceProfile};
use crate::effects::AbilityProfile;
use crate::progression::DefeatReward;
use crate::social::{
    LocalAlert, LocalAlertProfile, ObservedPropertyTake, SocialGroupId, WitnessProfile,
};
use crate::stats::PrimaryAttributes;
use crate::status::{StatusDefinition, StatusId, StatusInstance, StatusSet};
use crate::world::GridPos;

#[derive(Clone, PartialEq, Eq)]
pub struct Actor {
    position: GridPos,
    integrity: u16,
    maximum_integrity: u16,
    resistances: ResistanceProfile,
    attacks: Vec<AttackProfile>,
    abilities: Vec<AbilityProfile>,
    ai: Option<AiProfile>,
    defeat_reward: Option<DefeatReward>,
    statuses: StatusSet,
    primary_attributes: Option<PrimaryAttributes>,
    affiliation: Option<SocialGroupId>,
    property_take_authorizations: BTreeSet<SocialGroupId>,
    witness_profile: Option<WitnessProfile>,
    observed_property_takes: Vec<ObservedPropertyTake>,
    local_alert_profile: Option<LocalAlertProfile>,
    local_alert: Option<LocalAlert>,
}

// Optional social data is omitted while empty so pre-social replay fingerprints
// retain the exact Debug representation used by suspension versions 1 to 5.
impl Debug for Actor {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut actor = formatter.debug_struct("Actor");
        actor
            .field("position", &self.position)
            .field("integrity", &self.integrity)
            .field("maximum_integrity", &self.maximum_integrity)
            .field("resistances", &self.resistances)
            .field("attacks", &self.attacks)
            .field("abilities", &self.abilities)
            .field("ai", &self.ai)
            .field("defeat_reward", &self.defeat_reward)
            .field("statuses", &self.statuses)
            .field("primary_attributes", &self.primary_attributes);
        if let Some(affiliation) = &self.affiliation {
            actor.field("affiliation", affiliation);
        }
        if !self.property_take_authorizations.is_empty() {
            actor.field(
                "property_take_authorizations",
                &self.property_take_authorizations,
            );
        }
        if let Some(profile) = self.witness_profile {
            actor.field("witness_profile", &profile);
        }
        if !self.observed_property_takes.is_empty() {
            actor.field("observed_property_takes", &self.observed_property_takes);
        }
        if let Some(profile) = self.local_alert_profile {
            actor.field("local_alert_profile", &profile);
        }
        if let Some(alert) = &self.local_alert {
            actor.field("local_alert", alert);
        }
        actor.finish()
    }
}

impl Actor {
    pub fn new(position: GridPos, maximum_integrity: u16) -> Result<Self, ActorBuildError> {
        if maximum_integrity == 0 {
            return Err(ActorBuildError::ZeroMaximumIntegrity);
        }

        Ok(Self {
            position,
            integrity: maximum_integrity,
            maximum_integrity,
            resistances: ResistanceProfile::default(),
            attacks: Vec::new(),
            abilities: Vec::new(),
            ai: None,
            defeat_reward: None,
            statuses: StatusSet::default(),
            primary_attributes: None,
            affiliation: None,
            property_take_authorizations: BTreeSet::new(),
            witness_profile: None,
            observed_property_takes: Vec::new(),
            local_alert_profile: None,
            local_alert: None,
        })
    }

    pub fn with_attack(mut self, attack: AttackProfile) -> Self {
        self.attacks.push(attack);
        self
    }

    pub fn with_attacks(mut self, attacks: impl IntoIterator<Item = AttackProfile>) -> Self {
        self.attacks.extend(attacks);
        self
    }

    pub fn with_abilities(mut self, abilities: impl IntoIterator<Item = AbilityProfile>) -> Self {
        self.abilities.extend(abilities);
        self
    }

    pub const fn with_resistances(mut self, resistances: ResistanceProfile) -> Self {
        self.resistances = resistances;
        self
    }

    pub const fn with_ai(mut self, ai: AiProfile) -> Self {
        self.ai = Some(ai);
        self
    }

    pub const fn with_defeat_reward(mut self, reward: DefeatReward) -> Self {
        self.defeat_reward = Some(reward);
        self
    }

    pub const fn with_primary_attributes(mut self, attributes: PrimaryAttributes) -> Self {
        self.primary_attributes = Some(attributes);
        self
    }

    pub fn with_affiliation(mut self, affiliation: SocialGroupId) -> Self {
        self.affiliation = Some(affiliation);
        self
    }

    pub fn with_property_take_authorization(mut self, owner: SocialGroupId) -> Self {
        self.property_take_authorizations.insert(owner);
        self
    }

    pub const fn with_witness_profile(mut self, profile: WitnessProfile) -> Self {
        self.witness_profile = Some(profile);
        self
    }

    pub const fn with_local_alert_profile(mut self, profile: LocalAlertProfile) -> Self {
        self.local_alert_profile = Some(profile);
        self
    }

    pub const fn position(&self) -> GridPos {
        self.position
    }

    pub const fn integrity(&self) -> u16 {
        self.integrity
    }

    pub const fn maximum_integrity(&self) -> u16 {
        self.maximum_integrity
    }

    pub const fn resistances(&self) -> ResistanceProfile {
        self.resistances
    }

    pub fn attack(&self, slot: u8) -> Option<AttackProfile> {
        self.attacks.get(usize::from(slot)).copied()
    }

    pub fn attacks(&self) -> &[AttackProfile] {
        &self.attacks
    }

    pub fn ability(&self, slot: u8) -> Option<&AbilityProfile> {
        self.abilities.get(usize::from(slot))
    }

    pub fn abilities(&self) -> &[AbilityProfile] {
        &self.abilities
    }

    pub const fn ai(&self) -> Option<AiProfile> {
        self.ai
    }

    pub const fn defeat_reward(&self) -> Option<DefeatReward> {
        self.defeat_reward
    }

    pub const fn primary_attributes(&self) -> Option<PrimaryAttributes> {
        self.primary_attributes
    }

    pub const fn affiliation(&self) -> Option<&SocialGroupId> {
        self.affiliation.as_ref()
    }

    pub fn may_take_property_of(&self, owner: &SocialGroupId) -> bool {
        self.affiliation.as_ref() == Some(owner)
            || self.property_take_authorizations.contains(owner)
    }

    pub fn property_take_authorizations(&self) -> impl Iterator<Item = &SocialGroupId> {
        self.property_take_authorizations.iter()
    }

    pub(crate) fn grant_property_take_authorization(&mut self, owner: SocialGroupId) -> bool {
        self.property_take_authorizations.insert(owner)
    }

    pub const fn witness_profile(&self) -> Option<WitnessProfile> {
        self.witness_profile
    }

    pub fn observed_property_takes(&self) -> &[ObservedPropertyTake] {
        &self.observed_property_takes
    }

    pub const fn local_alert_profile(&self) -> Option<LocalAlertProfile> {
        self.local_alert_profile
    }

    pub const fn local_alert(&self) -> Option<&LocalAlert> {
        self.local_alert.as_ref()
    }

    pub(crate) fn remember_property_take(&mut self, incident: ObservedPropertyTake) {
        let Some(profile) = self.witness_profile else {
            return;
        };
        if self.observed_property_takes.len() >= profile.memory_capacity() {
            self.observed_property_takes.remove(0);
        }
        self.observed_property_takes.push(incident);
    }

    pub(crate) fn raise_local_alert(&mut self, incident: ObservedPropertyTake) -> bool {
        let Some(profile) = self.local_alert_profile else {
            return false;
        };
        let expires_on_turn = incident
            .turn
            .saturating_add(u64::from(profile.duration_turns()))
            .saturating_add(1);
        self.local_alert = Some(LocalAlert {
            incident,
            expires_on_turn,
        });
        true
    }

    pub fn statuses(&self) -> impl Iterator<Item = &StatusInstance> {
        self.statuses.iter()
    }

    pub fn status(&self, id: &StatusId) -> Option<&StatusInstance> {
        self.statuses.get(id)
    }

    pub(crate) fn apply_status(
        &mut self,
        definition: &StatusDefinition,
        stacks: u16,
        source: Option<crate::entity::EntityId>,
    ) -> crate::status::StatusApplyOutcome {
        self.statuses.apply(definition, stacks, source)
    }

    pub(crate) fn elapse_status_turn(&mut self, id: &StatusId) -> bool {
        self.statuses.elapse_one_turn(id)
    }

    pub(super) fn set_position(&mut self, position: GridPos) {
        self.position = position;
    }

    pub(crate) fn apply_damage(&mut self, amount: u16) -> u16 {
        let applied = amount.min(self.integrity);
        self.integrity -= applied;
        applied
    }

    pub(crate) fn restore_integrity(&mut self, amount: u16) -> u16 {
        let missing = self.maximum_integrity.saturating_sub(self.integrity);
        let restored = amount.min(missing);
        self.integrity += restored;
        restored
    }

    pub const fn is_alive(&self) -> bool {
        self.integrity > 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActorBuildError {
    ZeroMaximumIntegrity,
}

impl Display for ActorBuildError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroMaximumIntegrity => write!(formatter, "maximum integrity must be positive"),
        }
    }
}

impl Error for ActorBuildError {}
