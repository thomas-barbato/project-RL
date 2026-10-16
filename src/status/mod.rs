mod definition;
mod instance;

pub use definition::{
    StatusCatalog, StatusCatalogError, StatusDefinition, StatusDefinitionError,
    StatusEffectPrimitive, StatusFamilyId, StatusHook, StatusId, StatusIdError, StatusModifier,
    StatusStacking, StatusTransition, StatusTrigger,
};
pub(crate) use instance::with_legacy_status_encoding;
pub use instance::{StatusApplyKind, StatusApplyOutcome, StatusInstance, StatusPower, StatusSet};
