use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::combat::DamagePacket;
use crate::content::ContentId;
use crate::entity::EntityId;
use crate::world::GridPos;

pub type GroundEffectId = ContentId;

/// Complete, data-shaped rules for a persistent effect left on a map cell.
/// Carrying the rules in the instance keeps modded fields deterministic even
/// when several weapons create different kinds of hazards.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GroundEffectSpec {
    id: GroundEffectId,
    duration_turns: u16,
    damage_each_turn: DamagePacket,
}

impl GroundEffectSpec {
    pub fn new(
        id: GroundEffectId,
        duration_turns: u16,
        damage_each_turn: DamagePacket,
    ) -> Result<Self, GroundEffectSpecError> {
        if duration_turns == 0 {
            return Err(GroundEffectSpecError::ZeroDuration);
        }
        if damage_each_turn.amount == 0 {
            return Err(GroundEffectSpecError::ZeroDamage);
        }
        Ok(Self {
            id,
            duration_turns,
            damage_each_turn,
        })
    }

    pub const fn id(&self) -> &GroundEffectId {
        &self.id
    }

    pub const fn duration_turns(&self) -> u16 {
        self.duration_turns
    }

    pub const fn damage_each_turn(&self) -> DamagePacket {
        self.damage_each_turn
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GroundEffectSpecError {
    ZeroDuration,
    ZeroDamage,
}

impl Display for GroundEffectSpecError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroDuration => write!(formatter, "ground effect duration must be positive"),
            Self::ZeroDamage => write!(formatter, "ground effect damage must be positive"),
        }
    }
}

impl Error for GroundEffectSpecError {}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GroundEffectInstance {
    definition: GroundEffectId,
    position: GridPos,
    source: Option<EntityId>,
    remaining_turns: u16,
    next_trigger_turn: u64,
    damage_each_turn: DamagePacket,
}

impl GroundEffectInstance {
    pub const fn definition(&self) -> &GroundEffectId {
        &self.definition
    }

    pub const fn position(&self) -> GridPos {
        self.position
    }

    pub const fn source(&self) -> Option<EntityId> {
        self.source
    }

    pub const fn remaining_turns(&self) -> u16 {
        self.remaining_turns
    }

    pub const fn damage_each_turn(&self) -> DamagePacket {
        self.damage_each_turn
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GroundEffectMap {
    instances: BTreeMap<(GridPos, GroundEffectId), GroundEffectInstance>,
}

impl GroundEffectMap {
    pub fn apply(
        &mut self,
        position: GridPos,
        source: Option<EntityId>,
        spec: &GroundEffectSpec,
        current_turn: u64,
    ) -> &GroundEffectInstance {
        let key = (position, spec.id().clone());
        let instance = self
            .instances
            .entry(key)
            .or_insert_with(|| GroundEffectInstance {
                definition: spec.id().clone(),
                position,
                source,
                remaining_turns: spec.duration_turns(),
                next_trigger_turn: current_turn.saturating_add(1),
                damage_each_turn: spec.damage_each_turn(),
            });
        instance.source = source;
        instance.remaining_turns = instance.remaining_turns.max(spec.duration_turns());
        instance.damage_each_turn = spec.damage_each_turn();
        instance
    }

    pub fn ready_on(&self, turn: u64) -> Vec<GroundEffectInstance> {
        self.instances
            .values()
            .filter(|instance| instance.next_trigger_turn <= turn)
            .cloned()
            .collect()
    }

    /// Advances exactly one field after its trigger was resolved. Returns true
    /// when the field expired and was removed.
    pub fn elapse(&mut self, position: GridPos, id: &GroundEffectId) -> bool {
        let key = (position, id.clone());
        let Some(instance) = self.instances.get_mut(&key) else {
            return false;
        };
        instance.remaining_turns = instance.remaining_turns.saturating_sub(1);
        if instance.remaining_turns == 0 {
            self.instances.remove(&key);
            true
        } else {
            instance.next_trigger_turn = instance.next_trigger_turn.saturating_add(1);
            false
        }
    }

    pub fn at(&self, position: GridPos) -> impl Iterator<Item = &GroundEffectInstance> {
        self.instances
            .values()
            .filter(move |instance| instance.position == position)
    }

    pub fn iter(&self) -> impl Iterator<Item = &GroundEffectInstance> {
        self.instances.values()
    }

    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::DamageType;

    fn spec() -> GroundEffectSpec {
        GroundEffectSpec::new(
            "core:test_fire".parse().unwrap(),
            3,
            DamagePacket::new(1, DamageType::Thermal, 0),
        )
        .unwrap()
    }

    #[test]
    fn a_new_field_waits_until_the_following_turn_and_then_expires_exactly() {
        let mut fields = GroundEffectMap::default();
        let position = GridPos::new(2, 3);
        fields.apply(position, None, &spec(), 4);

        assert!(fields.ready_on(4).is_empty());
        for turn in 5..=7 {
            assert_eq!(fields.ready_on(turn).len(), 1);
            assert_eq!(fields.elapse(position, spec().id()), turn == 7);
        }
        assert!(fields.is_empty());
    }

    #[test]
    fn reapplication_refreshes_duration_without_duplicating_the_same_field() {
        let mut fields = GroundEffectMap::default();
        let position = GridPos::new(2, 3);
        fields.apply(position, None, &spec(), 1);
        assert!(!fields.elapse(position, spec().id()));
        fields.apply(position, None, &spec(), 2);
        fields.apply(position, None, &spec(), 2);

        assert_eq!(fields.iter().count(), 1);
        assert_eq!(
            fields
                .at(position)
                .next()
                .map(|field| field.remaining_turns()),
            Some(3)
        );
    }
}
