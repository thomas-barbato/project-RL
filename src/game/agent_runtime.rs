//! Runtime effects for the reviewed agent catalogue. Serialized independently
//! from content definitions, with an explicit legacy binary boundary.
use super::agent_drone_utilities::AgentDroneRuntime;
use super::agent_magic_runtime::AgentMagicRuntime;
use super::agent_support_cast::AgentSupportRuntime;
use super::agent_weapon_effects::AgentWeaponRuntime;
use super::*;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AgentRuntime {
    pub weapon: AgentWeaponRuntime,
    pub invocation_ranks: BTreeMap<String, u8>,
    pub devices: super::agent_devices::AgentDevicesRuntime,
    pub transform: super::agent_transform_cast::AgentTransformRuntime,
    pub magic: AgentMagicRuntime,
    pub drones: AgentDroneRuntime,
    pub support: AgentSupportRuntime,
}
impl AgentRuntime {
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
impl GameState {
    pub(super) fn agent_runtime(&self) -> &AgentRuntime {
        &self.agent_runtime
    }
    pub(super) fn agent_runtime_mut(&mut self) -> &mut AgentRuntime {
        &mut self.agent_runtime
    }
}
thread_local! { static LEGACY: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
pub(crate) fn legacy_encoding() -> bool {
    LEGACY.get()
}
pub(crate) fn with_legacy_encoding<T>(operation: impl FnOnce() -> T) -> T {
    struct Reset(bool);
    impl Drop for Reset {
        fn drop(&mut self) {
            LEGACY.set(self.0);
        }
    }
    let _reset = Reset(LEGACY.replace(true));
    operation()
}
/// A unit occupies zero bytes in bincode. Old snapshots therefore omit this
/// newly appended field without consuming the following world's fields.
pub(crate) mod codec {
    use super::*;
    pub fn serialize<S: serde::Serializer>(
        value: &AgentRuntime,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if LEGACY.get() && !serializer.is_human_readable() {
            ().serialize(serializer)
        } else {
            value.serialize(serializer)
        }
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<AgentRuntime, D::Error> {
        if LEGACY.get() && !deserializer.is_human_readable() {
            <()>::deserialize(deserializer)?;
            Ok(AgentRuntime::default())
        } else {
            AgentRuntime::deserialize(deserializer)
        }
    }
}
