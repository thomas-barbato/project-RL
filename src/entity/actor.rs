use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::ai::AiProfile;
use crate::combat::{AttackProfile, ResistanceProfile};
use crate::effects::AbilityProfile;
use crate::progression::DefeatReward;
use crate::status::{StatusDefinition, StatusId, StatusInstance, StatusSet};
use crate::world::GridPos;

#[derive(Clone, Debug, PartialEq, Eq)]
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
