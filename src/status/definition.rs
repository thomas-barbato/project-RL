use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::combat::DamagePacket;
pub use crate::content::{ContentId as StatusId, ContentIdError as StatusIdError};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum StatusTrigger {
    TurnStart,
    TurnEnd,
    DamageReceived,
    DamageDealt,
    Movement,
    Death,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusStacking {
    Replace,
    RefreshDuration,
    AddStacks {
        maximum_stacks: u16,
        refresh_duration: bool,
    },
}

impl StatusStacking {
    pub const fn maximum_stacks(self) -> u16 {
        match self {
            Self::Replace | Self::RefreshDuration => 1,
            Self::AddStacks { maximum_stacks, .. } => maximum_stacks,
        }
    }
}

/// Generic work executed by a status hook.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusEffectPrimitive {
    DealDamage {
        packet: DamagePacket,
        multiply_by_stacks: bool,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusHook {
    trigger: StatusTrigger,
    effects: Vec<StatusEffectPrimitive>,
}

impl StatusHook {
    pub fn new(trigger: StatusTrigger, effects: Vec<StatusEffectPrimitive>) -> Self {
        Self { trigger, effects }
    }

    pub const fn trigger(&self) -> StatusTrigger {
        self.trigger
    }

    pub fn effects(&self) -> &[StatusEffectPrimitive] {
        &self.effects
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StatusDefinition {
    id: StatusId,
    duration_turns: Option<u16>,
    stacking: StatusStacking,
    hooks: Vec<StatusHook>,
}

impl StatusDefinition {
    pub fn new(
        id: StatusId,
        duration_turns: Option<u16>,
        stacking: StatusStacking,
        hooks: Vec<StatusHook>,
    ) -> Result<Self, StatusDefinitionError> {
        if duration_turns == Some(0) {
            return Err(StatusDefinitionError::ZeroDuration);
        }
        if stacking.maximum_stacks() == 0 {
            return Err(StatusDefinitionError::ZeroMaximumStacks);
        }
        Ok(Self {
            id,
            duration_turns,
            stacking,
            hooks,
        })
    }

    pub const fn id(&self) -> &StatusId {
        &self.id
    }

    pub const fn duration_turns(&self) -> Option<u16> {
        self.duration_turns
    }

    pub const fn stacking(&self) -> StatusStacking {
        self.stacking
    }

    pub fn hooks_for(&self, trigger: StatusTrigger) -> impl Iterator<Item = &StatusHook> {
        self.hooks
            .iter()
            .filter(move |hook| hook.trigger() == trigger)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusDefinitionError {
    ZeroDuration,
    ZeroMaximumStacks,
}

impl Display for StatusDefinitionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroDuration => write!(formatter, "finite status duration must be positive"),
            Self::ZeroMaximumStacks => write!(formatter, "maximum status stacks must be positive"),
        }
    }
}

impl Error for StatusDefinitionError {}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StatusCatalog {
    definitions: BTreeMap<StatusId, StatusDefinition>,
}

impl StatusCatalog {
    pub fn register(&mut self, definition: StatusDefinition) -> Result<(), StatusCatalogError> {
        let id = definition.id().clone();
        if self.definitions.contains_key(&id) {
            return Err(StatusCatalogError::DuplicateId(id));
        }
        self.definitions.insert(id, definition);
        Ok(())
    }

    pub fn get(&self, id: &StatusId) -> Option<&StatusDefinition> {
        self.definitions.get(id)
    }

    pub fn contains(&self, id: &StatusId) -> bool {
        self.definitions.contains_key(id)
    }

    pub fn without_id(&self, excluded: &StatusId) -> Self {
        let mut definitions = self.definitions.clone();
        definitions.remove(excluded);
        Self { definitions }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StatusCatalogError {
    DuplicateId(StatusId),
}

impl Display for StatusCatalogError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateId(id) => write!(formatter, "duplicate status ID '{}'", id.as_str()),
        }
    }
}

impl Error for StatusCatalogError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn status_id(value: &str) -> StatusId {
        value
            .parse()
            .unwrap_or_else(|error| panic!("valid status ID rejected: {error}"))
    }

    #[test]
    fn ids_require_a_stable_namespace_and_name() {
        assert!("core:corroded".parse::<StatusId>().is_ok());
        assert!("corroded".parse::<StatusId>().is_err());
        assert!("core:".parse::<StatusId>().is_err());
        assert!("core:bad:id".parse::<StatusId>().is_err());
    }

    #[test]
    fn catalog_rejects_duplicate_definitions() {
        let definition = StatusDefinition::new(
            status_id("core:corroded"),
            Some(3),
            StatusStacking::RefreshDuration,
            Vec::new(),
        )
        .unwrap_or_else(|error| panic!("valid definition rejected: {error}"));
        let mut catalog = StatusCatalog::default();

        assert_eq!(catalog.register(definition.clone()), Ok(()));
        assert_eq!(
            catalog.register(definition),
            Err(StatusCatalogError::DuplicateId(status_id("core:corroded")))
        );
    }
}
