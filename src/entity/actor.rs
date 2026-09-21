use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::ai::{AiProfile, AiState};
use crate::combat::{ArmorProfile, AttackProfile, ResistanceProfile};
use crate::content::ContentId;
use crate::drone::DroneState;
use crate::effects::{AbilityProfile, DestructionEffect};
use crate::electronic_warfare::{ElectronicSystemProfile, ElectronicSystemState};
use crate::progression::DefeatReward;
use crate::reaction::{ActionOrigin, PreparedReaction, ReactionState, ReactionTrigger};
use crate::skills::TechniqueId;
use crate::social::{
    LocalAlert, LocalAlertProfile, MAX_RECEIVED_PROPERTY_REPORTS, ObservedPropertyTake,
    PlayerRelation, PropertyReportProfile, ReportedPropertyTake, SocialGroupId, WitnessProfile,
};
use crate::stats::{
    BodyProfile, DisplacementProfile, HitPointRules, LocomotionProfile, PrimaryAttributes,
};
use crate::status::{StatusDefinition, StatusId, StatusInstance, StatusSet};
use crate::time::{
    ActionRecovery, CooldownAdvance, EnvironmentCooldown, RecoveryAdvance, TimeUnits,
};
use crate::world::GridPos;

use super::{
    BodyComponentId, BodyComponentProfile, BodyComponentState, ComponentFailureEffect, EntityId,
};

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Actor {
    position: GridPos,
    integrity: u16,
    maximum_integrity: u16,
    resistances: ResistanceProfile,
    attacks: Vec<AttackProfile>,
    abilities: Vec<AbilityProfile>,
    ai: Option<AiProfile>,
    ai_home: Option<GridPos>,
    ai_state: AiState,
    drone: Option<DroneState>,
    electronic_system: Option<ElectronicSystemState>,
    threat_source: Option<u16>,
    defeat_reward: Option<DefeatReward>,
    destruction_effect: Option<DestructionEffect>,
    statuses: StatusSet,
    primary_attributes: Option<PrimaryAttributes>,
    body_profile: Option<BodyProfile>,
    body_components: BTreeMap<BodyComponentId, BodyComponentState>,
    can_evade: bool,
    evasion_modifier: i16,
    affiliation: Option<SocialGroupId>,
    player_relation: PlayerRelation,
    tags: BTreeSet<ContentId>,
    property_take_authorizations: BTreeSet<SocialGroupId>,
    witness_profile: Option<WitnessProfile>,
    observed_property_takes: Vec<ObservedPropertyTake>,
    local_alert_profile: Option<LocalAlertProfile>,
    local_alert: Option<LocalAlert>,
    property_report_profile: Option<PropertyReportProfile>,
    received_property_take_reports: Vec<ReportedPropertyTake>,
    reaction_state: ReactionState,
    action_recovery: Option<ActionRecovery>,
    technique_cooldowns: BTreeMap<TechniqueId, EnvironmentCooldown>,
    next_action_turn: Option<u64>,
}

// Optional social and pursuit data is omitted while empty so older replay
// fingerprints retain their exact historical Debug representation.
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
        if self.player_relation != PlayerRelation::Neutral {
            actor.field("player_relation", &self.player_relation);
        }
        if !self.tags.is_empty() {
            actor.field("tags", &self.tags);
        }
        if let Some(home) = self.ai_home {
            actor.field("ai_home", &home);
        }
        if self.ai_state != AiState::Unaware {
            actor.field("ai_state", &self.ai_state);
        }
        if let Some(drone) = &self.drone {
            actor.field("drone", drone);
        }
        if let Some(system) = self.electronic_system {
            actor.field("electronic_system", &system);
        }
        if let Some(source) = self.threat_source {
            actor.field("threat_source", &source);
        }
        if let Some(effect) = &self.destruction_effect {
            actor.field("destruction_effect", effect);
        }
        if self.evasion_modifier != 0 {
            actor.field("evasion_modifier", &self.evasion_modifier);
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
        if let Some(profile) = self.property_report_profile {
            actor.field("property_report_profile", &profile);
        }
        if !self.received_property_take_reports.is_empty() {
            actor.field(
                "received_property_take_reports",
                &self.received_property_take_reports,
            );
        }
        if let Some(body_profile) = self.body_profile {
            actor.field("body_profile", &body_profile);
        }
        if !self.body_components.is_empty() {
            actor.field("body_components", &self.body_components);
        }
        if self.reaction_state != ReactionState::default() {
            actor.field("reaction_state", &self.reaction_state);
        }
        if let Some(recovery) = self.action_recovery {
            actor.field("action_recovery", &recovery);
        }
        if !self.technique_cooldowns.is_empty() {
            actor.field("technique_cooldowns", &self.technique_cooldowns);
        }
        if let Some(next_action_turn) = self.next_action_turn {
            actor.field("next_action_turn", &next_action_turn);
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
            ai_home: None,
            ai_state: AiState::Unaware,
            drone: None,
            electronic_system: None,
            threat_source: None,
            defeat_reward: None,
            destruction_effect: None,
            statuses: StatusSet::default(),
            primary_attributes: None,
            body_profile: None,
            body_components: BTreeMap::new(),
            can_evade: true,
            evasion_modifier: 0,
            affiliation: None,
            player_relation: PlayerRelation::Neutral,
            tags: BTreeSet::new(),
            property_take_authorizations: BTreeSet::new(),
            witness_profile: None,
            observed_property_takes: Vec::new(),
            local_alert_profile: None,
            local_alert: None,
            property_report_profile: None,
            received_property_take_reports: Vec::new(),
            reaction_state: ReactionState::default(),
            action_recovery: None,
            technique_cooldowns: BTreeMap::new(),
            next_action_turn: None,
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
        self.ai_home = if ai.maximum_pursuit_distance().is_some() {
            Some(self.position)
        } else {
            None
        };
        self.ai_state = AiState::Unaware;
        self
    }

    pub fn with_drone(mut self, drone: DroneState) -> Self {
        self.drone = Some(drone);
        self
    }

    pub const fn with_electronic_system(mut self, profile: ElectronicSystemProfile) -> Self {
        self.electronic_system = Some(ElectronicSystemState::new(profile));
        self
    }

    pub const fn drone(&self) -> Option<&DroneState> {
        self.drone.as_ref()
    }

    pub fn drone_mut(&mut self) -> Option<&mut DroneState> {
        self.drone.as_mut()
    }

    pub const fn electronic_system(&self) -> Option<ElectronicSystemState> {
        self.electronic_system
    }

    pub(crate) fn electronic_system_mut(&mut self) -> Option<&mut ElectronicSystemState> {
        self.electronic_system.as_mut()
    }

    pub(crate) fn ensure_electronic_system(&mut self, profile: ElectronicSystemProfile) {
        if self.electronic_system.is_none() {
            self.electronic_system = Some(ElectronicSystemState::new(profile));
        }
    }

    pub const fn with_defeat_reward(mut self, reward: DefeatReward) -> Self {
        self.defeat_reward = Some(reward);
        self
    }

    pub fn with_destruction_effect(mut self, effect: DestructionEffect) -> Self {
        self.destruction_effect = Some(effect);
        self
    }

    pub const fn with_primary_attributes(mut self, attributes: PrimaryAttributes) -> Self {
        self.primary_attributes = Some(attributes);
        self
    }

    pub const fn with_body_profile(mut self, profile: BodyProfile) -> Self {
        self.body_profile = Some(profile);
        self
    }

    /// Marks a stationary or otherwise certain target as exempt from passive
    /// accuracy/evasion rolls. The attack must still pass range, line-of-sight
    /// and protected-zone validation.
    pub const fn with_evasion_disabled(mut self) -> Self {
        self.can_evade = false;
        self
    }

    pub const fn with_evasion_modifier(mut self, modifier: i16) -> Self {
        self.evasion_modifier = modifier;
        self
    }

    pub fn with_affiliation(mut self, affiliation: SocialGroupId) -> Self {
        self.affiliation = Some(affiliation);
        self
    }

    pub const fn with_player_relation(mut self, relation: PlayerRelation) -> Self {
        self.player_relation = relation;
        self
    }

    pub fn with_tags(mut self, tags: impl IntoIterator<Item = ContentId>) -> Self {
        self.tags.extend(tags);
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

    pub const fn with_property_report_profile(mut self, profile: PropertyReportProfile) -> Self {
        self.property_report_profile = Some(profile);
        self
    }

    pub const fn position(&self) -> GridPos {
        self.position
    }

    pub const fn displacement_profile(&self) -> Option<DisplacementProfile> {
        match self.body_profile {
            Some(body) => body.displacement_profile(),
            None => None,
        }
    }

    pub const fn locomotion_profile(&self) -> Option<LocomotionProfile> {
        match self.body_profile {
            Some(body) => body.locomotion_profile(),
            None => None,
        }
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

    pub const fn ai_home(&self) -> Option<GridPos> {
        self.ai_home
    }

    pub const fn ai_state(&self) -> AiState {
        self.ai_state
    }

    pub(crate) const fn set_ai_state(&mut self, state: AiState) {
        self.ai_state = state;
    }

    pub(crate) const fn set_threat_source(&mut self, source: u16) {
        self.threat_source = Some(source);
    }

    pub const fn threat_source(&self) -> Option<u16> {
        self.threat_source
    }

    pub const fn defeat_reward(&self) -> Option<DefeatReward> {
        self.defeat_reward
    }

    pub const fn destruction_effect(&self) -> Option<&DestructionEffect> {
        self.destruction_effect.as_ref()
    }

    pub const fn primary_attributes(&self) -> Option<PrimaryAttributes> {
        self.primary_attributes
    }

    pub const fn body_profile(&self) -> Option<BodyProfile> {
        self.body_profile
    }

    pub fn with_body_components(
        mut self,
        profiles: impl IntoIterator<Item = BodyComponentProfile>,
    ) -> Self {
        self.body_components = profiles
            .into_iter()
            .map(|profile| (profile.id().clone(), BodyComponentState::new(profile)))
            .collect();
        self
    }

    pub fn body_components(&self) -> impl Iterator<Item = &BodyComponentState> {
        self.body_components.values()
    }

    pub fn body_component(&self, id: &BodyComponentId) -> Option<&BodyComponentState> {
        self.body_components.get(id)
    }

    pub(crate) fn body_component_mut(
        &mut self,
        id: &BodyComponentId,
    ) -> Option<&mut BodyComponentState> {
        self.body_components.get_mut(id)
    }

    pub fn failed_component_effects(&self) -> impl Iterator<Item = ComponentFailureEffect> + '_ {
        self.body_components
            .values()
            .filter(|component| component.is_failed())
            .map(|component| component.profile().failure_effect())
    }

    /// Recalculates intrinsic Armor from explicit actor sources instead of
    /// storing a second total that could diverge. `GameState` composes this
    /// profile with currently equipped items when resolving gameplay.
    pub const fn armor_profile(&self) -> ArmorProfile {
        ArmorProfile::new(
            match self.body_profile {
                Some(body) => body.base_armor,
                None => 0,
            },
            0,
            0,
            0,
        )
    }

    pub const fn reaction_available(&self) -> bool {
        self.reaction_state.is_available()
    }

    pub const fn prepared_reaction(&self) -> Option<&PreparedReaction> {
        self.reaction_state.prepared()
    }

    pub(crate) fn reaction_state(&self) -> ReactionState {
        self.reaction_state.clone()
    }

    pub(crate) fn restore_reaction_state(&mut self, state: ReactionState) {
        self.reaction_state = state;
    }

    pub(crate) fn begin_normal_action(&mut self) -> Option<PreparedReaction> {
        self.reaction_state.begin_normal_action()
    }

    pub const fn recovery_remaining(&self) -> Option<TimeUnits> {
        match self.action_recovery {
            Some(recovery) => Some(recovery.remaining_actions()),
            None => None,
        }
    }

    pub(crate) fn start_action_recovery(&mut self, duration: TimeUnits) {
        self.action_recovery = Some(ActionRecovery::new(duration));
    }

    pub(crate) fn advance_action_recovery(&mut self) -> Option<RecoveryAdvance> {
        let recovery = self.action_recovery.take()?;
        let advance = recovery.advance();
        if let RecoveryAdvance::Recovering(recovery) = advance {
            self.action_recovery = Some(recovery);
        }
        Some(advance)
    }

    pub fn technique_cooldown_remaining(&self, technique: &TechniqueId) -> Option<TimeUnits> {
        self.technique_cooldowns
            .get(technique)
            .map(|cooldown| cooldown.remaining_phases())
    }

    pub(crate) fn start_technique_cooldown(
        &mut self,
        technique: TechniqueId,
        current_turn: u64,
        duration: TimeUnits,
    ) {
        self.technique_cooldowns
            .insert(technique, EnvironmentCooldown::new(current_turn, duration));
    }

    pub(crate) fn technique_cooldowns(
        &self,
    ) -> impl Iterator<Item = (&TechniqueId, EnvironmentCooldown)> {
        self.technique_cooldowns
            .iter()
            .map(|(technique, cooldown)| (technique, *cooldown))
    }

    pub(crate) fn advance_technique_cooldown(
        &mut self,
        technique: &TechniqueId,
        environment_turn: u64,
    ) -> Option<CooldownAdvance> {
        let cooldown = self.technique_cooldowns.remove(technique)?;
        let advance = cooldown.advance(environment_turn);
        match advance {
            CooldownAdvance::Arming(next) | CooldownAdvance::Cooling(next) => {
                self.technique_cooldowns.insert(technique.clone(), next);
            }
            CooldownAdvance::Complete => {}
        }
        Some(advance)
    }

    pub(crate) const fn action_is_delayed(&self, current_turn: u64) -> bool {
        matches!(self.next_action_turn, Some(ready) if current_turn < ready)
    }

    pub(crate) fn delay_next_action_until(&mut self, turn: u64) {
        self.next_action_turn = Some(turn);
    }

    pub(crate) fn clear_elapsed_action_delay(&mut self, current_turn: u64) {
        if self
            .next_action_turn
            .is_some_and(|ready| current_turn >= ready)
        {
            self.next_action_turn = None;
        }
    }

    pub(crate) fn prepare_reaction(
        &mut self,
        reaction: PreparedReaction,
    ) -> Option<PreparedReaction> {
        self.reaction_state.prepare(reaction)
    }

    pub(crate) fn try_trigger_reaction(
        &mut self,
        trigger: ReactionTrigger,
        source: ActionOrigin,
    ) -> Option<PreparedReaction> {
        self.reaction_state.try_trigger(trigger, source)
    }

    pub const fn try_consume_reaction(&mut self, source: ActionOrigin) -> bool {
        self.reaction_state.try_consume_unprepared(source)
    }

    pub(crate) fn initialize_body_hit_points(&mut self, rules: HitPointRules) {
        let Some(profile) = self.body_profile else {
            return;
        };
        let maximum = rules.maximum_for_body(profile, self.primary_attributes, 0);
        self.maximum_integrity = maximum;
        self.integrity = maximum;
    }

    pub const fn can_evade(&self) -> bool {
        self.can_evade
    }

    pub const fn evasion_modifier(&self) -> i16 {
        self.evasion_modifier
    }

    pub const fn affiliation(&self) -> Option<&SocialGroupId> {
        self.affiliation.as_ref()
    }

    pub const fn player_relation(&self) -> PlayerRelation {
        self.player_relation
    }

    pub fn tags(&self) -> &BTreeSet<ContentId> {
        &self.tags
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

    pub(crate) fn revoke_property_take_authorization(&mut self, owner: &SocialGroupId) -> bool {
        self.property_take_authorizations.remove(owner)
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

    pub const fn property_report_profile(&self) -> Option<PropertyReportProfile> {
        self.property_report_profile
    }

    pub(crate) fn set_property_report_profile(&mut self, profile: PropertyReportProfile) {
        self.property_report_profile = Some(profile);
    }

    pub fn received_property_take_reports(&self) -> &[ReportedPropertyTake] {
        &self.received_property_take_reports
    }

    pub(crate) fn receive_property_take_report(
        &mut self,
        source: EntityId,
        incident: ObservedPropertyTake,
    ) {
        if self.received_property_take_reports.len() >= MAX_RECEIVED_PROPERTY_REPORTS {
            self.received_property_take_reports.remove(0);
        }
        self.received_property_take_reports
            .push(ReportedPropertyTake { source, incident });
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

    pub(crate) fn set_position(&mut self, position: GridPos) {
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
