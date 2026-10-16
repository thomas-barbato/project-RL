//! Persistent, zone-local state of reviewed elemental and occult spells.
use super::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AgentMagicRuntime {
    pub effects: Vec<AgentMagicEffect>,
    pub frozen: BTreeMap<EntityId, AgentFrozen>,
    pub barriers: BTreeMap<EntityId, AgentMagicBarrier>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AgentFrozen {
    pub expires_at: u64,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AgentMagicBarrier {
    pub centre: GridPos,
    pub skill: String,
    pub rank: u8,
    pub expires_at: u64,
    pub last_contact: BTreeMap<EntityId, u64>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AgentMagicEffect {
    pub skill: String,
    pub rank: u8,
    pub origin: GridPos,
    pub target: GridPos,
    pub direction: (i32, i32),
    pub cells: Vec<GridPos>,
    pub remaining: u16,
    pub next_turn: u64,
    pub stage: u16,
    pub base_damage: u16,
    pub numerator: u64,
    pub denominator: u64,
    pub status_numerator: u64,
    pub status_denominator: u64,
    pub energy: u16,
    pub victim: Option<EntityId>,
    pub exposure: BTreeMap<EntityId, u16>,
    pub last_hit: BTreeMap<EntityId, u64>,
}
