use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::world::GridPos;

use super::{Actor, EntityId};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ActorRegistry {
    actors: BTreeMap<EntityId, Actor>,
    next_id: u64,
}

impl Default for ActorRegistry {
    fn default() -> Self {
        Self {
            actors: BTreeMap::new(),
            next_id: 1,
        }
    }
}

impl ActorRegistry {
    pub(crate) fn synchronize_ids(&mut self, other: &Self) {
        self.next_id = self.next_id.max(other.next_id);
    }

    pub(crate) fn insert_existing(&mut self, id: EntityId, actor: Actor) {
        assert!(!self.actors.contains_key(&id), "entity ID already present");
        self.actors.insert(id, actor);
    }

    pub fn spawn(&mut self, actor: Actor) -> Result<EntityId, RegistryError> {
        let following_id = self
            .next_id
            .checked_add(1)
            .ok_or(RegistryError::IdSpaceExhausted)?;
        let id = EntityId::new(self.next_id);
        self.next_id = following_id;
        self.actors.insert(id, actor);
        Ok(id)
    }

    pub fn get(&self, id: EntityId) -> Option<&Actor> {
        self.actors.get(&id)
    }

    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut Actor> {
        self.actors.get_mut(&id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (EntityId, &Actor)> {
        self.actors.iter().map(|(id, actor)| (*id, actor))
    }

    pub fn entity_at(&self, position: GridPos) -> Option<EntityId> {
        self.actors
            .iter()
            .find_map(|(id, actor)| (actor.position() == position).then_some(*id))
    }

    pub fn move_to(&mut self, id: EntityId, destination: GridPos) -> Result<(), RegistryError> {
        let actor = self
            .actors
            .get_mut(&id)
            .ok_or(RegistryError::UnknownEntity(id))?;
        actor.set_position(destination);
        Ok(())
    }

    pub fn remove(&mut self, id: EntityId) -> Option<Actor> {
        self.actors.remove(&id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegistryError {
    IdSpaceExhausted,
    UnknownEntity(EntityId),
}

impl Display for RegistryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IdSpaceExhausted => write!(formatter, "entity ID space is exhausted"),
            Self::UnknownEntity(id) => write!(formatter, "unknown entity ID {}", id.get()),
        }
    }
}

impl Error for RegistryError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spawned_entities_receive_stable_monotonic_ids() {
        let mut registry = ActorRegistry::default();
        let first_actor = Actor::new(GridPos::new(1, 1), 10)
            .unwrap_or_else(|error| panic!("valid actor failed to build: {error}"));
        let second_actor = Actor::new(GridPos::new(2, 1), 10)
            .unwrap_or_else(|error| panic!("valid actor failed to build: {error}"));
        let first = registry.spawn(first_actor);
        let second = registry.spawn(second_actor);

        assert_eq!(first.map(EntityId::get), Ok(1));
        assert_eq!(second.map(EntityId::get), Ok(2));
    }
}
