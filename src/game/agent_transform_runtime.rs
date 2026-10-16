use super::*;
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct AgentTransformRuntime {
    pub active: Option<AgentActiveForm>,
}
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct AgentActiveForm {
    pub skill: String,
    pub rank: u8,
    pub expires: u64,
    pub extension_used: u16,
    pub reserve: u16,
    pub closure_until: u64,
    pub weapon_types: Vec<(crate::combat::DamageType, u16)>,
    pub original_weapon_slots: Vec<u8>,
}
