use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::entity::{BodyComponentId, BodyComponentState, EntityId, ItemInstanceId};
use crate::world::GridPos;

/// Player-selected compromise for a compatible powered module.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ModuleTuning {
    Economy,
    Power,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ActiveTuning {
    mode: ModuleTuning,
    output_percentage: u16,
    energy_percentage: u16,
}

impl ActiveTuning {
    pub const fn new(
        mode: ModuleTuning,
        output_percentage: u16,
        energy_percentage: u16,
    ) -> Option<Self> {
        if output_percentage == 0 || energy_percentage == 0 {
            return None;
        }
        Some(Self {
            mode,
            output_percentage,
            energy_percentage,
        })
    }

    pub const fn mode(self) -> ModuleTuning {
        self.mode
    }

    pub const fn output_percentage(self) -> u16 {
        self.output_percentage
    }

    pub const fn energy_percentage(self) -> u16 {
        self.energy_percentage
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ActiveBypass {
    donor: BodyComponentId,
    restored_output_percentage: u16,
}

impl ActiveBypass {
    pub fn new(donor: BodyComponentId, restored_output_percentage: u16) -> Option<Self> {
        if restored_output_percentage == 0 {
            return None;
        }
        Some(Self {
            donor,
            restored_output_percentage,
        })
    }

    pub const fn donor(&self) -> &BodyComponentId {
        &self.donor
    }

    pub const fn restored_output_percentage(&self) -> u16 {
        self.restored_output_percentage
    }
}

/// Explicit target parameters for an engineering technique.
///
/// Keeping those parameters in the command makes preparations replayable and
/// prevents the engine from silently selecting a different component later.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum EngineeringDirective {
    Component {
        target: EntityId,
        component: BodyComponentId,
    },
    WreckComponent {
        wreck: WreckId,
        component: BodyComponentId,
    },
    TuneModule {
        module: ItemInstanceId,
        tuning: ModuleTuning,
    },
    OverclockModule {
        module: ItemInstanceId,
    },
    Bypass {
        target: EntityId,
        receiver: BodyComponentId,
        donor: BodyComponentId,
    },
    Module {
        module: ItemInstanceId,
    },
    AssembleAt {
        position: GridPos,
    },
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct WreckId(u64);

impl WreckId {
    pub const fn get(self) -> u64 {
        self.0
    }
}

/// Persistent remains of a destroyed actor. Components are moved here with
/// their exact durability; salvage removes the selected entry exactly once.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Wreckage {
    id: WreckId,
    position: GridPos,
    components: BTreeMap<BodyComponentId, BodyComponentState>,
}

impl Wreckage {
    pub const fn id(&self) -> WreckId {
        self.id
    }

    pub const fn position(&self) -> GridPos {
        self.position
    }

    pub fn components(&self) -> impl Iterator<Item = &BodyComponentState> {
        self.components.values()
    }

    pub fn component(&self, id: &BodyComponentId) -> Option<&BodyComponentState> {
        self.components.get(id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct WreckRegistry {
    next_id: u64,
    wrecks: BTreeMap<WreckId, Wreckage>,
}

impl Default for WreckRegistry {
    fn default() -> Self {
        Self {
            next_id: 1,
            wrecks: BTreeMap::new(),
        }
    }
}

impl WreckRegistry {
    pub fn create(
        &mut self,
        position: GridPos,
        components: impl IntoIterator<Item = BodyComponentState>,
    ) -> Result<Option<WreckId>, WreckError> {
        let components = components
            .into_iter()
            .filter(|component| !component.is_destroyed())
            .map(|component| (component.profile().id().clone(), component))
            .collect::<BTreeMap<_, _>>();
        if components.is_empty() {
            return Ok(None);
        }
        let id = WreckId(self.next_id);
        let next_id = self
            .next_id
            .checked_add(1)
            .ok_or(WreckError::IdSpaceExhausted)?;
        self.wrecks.insert(
            id,
            Wreckage {
                id,
                position,
                components,
            },
        );
        self.next_id = next_id;
        Ok(Some(id))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Wreckage> {
        self.wrecks.values()
    }

    pub fn get(&self, id: WreckId) -> Option<&Wreckage> {
        self.wrecks.get(&id)
    }

    pub fn recover_component(
        &mut self,
        wreck: WreckId,
        component: &BodyComponentId,
    ) -> Result<BodyComponentState, WreckError> {
        let wreckage = self
            .wrecks
            .get_mut(&wreck)
            .ok_or(WreckError::UnknownWreck(wreck))?;
        let component = wreckage
            .components
            .remove(component)
            .ok_or_else(|| WreckError::UnknownComponent(component.clone()))?;
        if wreckage.components.is_empty() {
            self.wrecks.remove(&wreck);
        }
        Ok(component)
    }

    pub fn is_empty(&self) -> bool {
        self.wrecks.is_empty()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WreckError {
    IdSpaceExhausted,
    UnknownWreck(WreckId),
    UnknownComponent(BodyComponentId),
}

impl Display for WreckError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IdSpaceExhausted => write!(formatter, "wreck ID space exhausted"),
            Self::UnknownWreck(wreck) => write!(formatter, "unknown wreck {}", wreck.get()),
            Self::UnknownComponent(component) => {
                write!(formatter, "wreck has no component '{}'", component.as_str())
            }
        }
    }
}

impl Error for WreckError {}

/// Mutable state belonging to one concrete equipment instance.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EquipmentEngineeringState {
    durability: u16,
    maximum_durability: u16,
    tuning: Option<ActiveTuning>,
    overclock: Option<ActiveOverclock>,
    suspended_as_donor: bool,
}

impl EquipmentEngineeringState {
    pub const fn new(maximum_durability: u16) -> Option<Self> {
        if maximum_durability == 0 {
            return None;
        }
        Some(Self {
            durability: maximum_durability,
            maximum_durability,
            tuning: None,
            overclock: None,
            suspended_as_donor: false,
        })
    }

    pub const fn durability(self) -> u16 {
        self.durability
    }

    pub const fn maximum_durability(self) -> u16 {
        self.maximum_durability
    }

    pub const fn tuning(self) -> Option<ModuleTuning> {
        match self.tuning {
            Some(tuning) => Some(tuning.mode()),
            None => None,
        }
    }

    pub const fn tuning_profile(self) -> Option<ActiveTuning> {
        self.tuning
    }

    pub const fn overclock(self) -> Option<ActiveOverclock> {
        self.overclock
    }

    pub const fn is_suspended_as_donor(self) -> bool {
        self.suspended_as_donor
    }

    pub fn damage(&mut self, amount: u16) -> u16 {
        let applied = amount.min(self.durability);
        self.durability -= applied;
        applied
    }

    /// Zero durability represents destruction and cannot be repaired.
    pub fn repair(&mut self, amount: u16) -> u16 {
        if self.durability == 0 {
            return 0;
        }
        let restored = amount.min(self.maximum_durability.saturating_sub(self.durability));
        self.durability = self.durability.saturating_add(restored);
        restored
    }

    pub fn tune(&mut self, tuning: ActiveTuning) {
        self.tuning = Some(tuning);
    }

    pub fn start_overclock(&mut self, overclock: ActiveOverclock) {
        self.overclock = Some(overclock);
    }

    pub fn stop_overclock(&mut self) {
        self.overclock = None;
    }

    pub fn set_suspended_as_donor(&mut self, suspended: bool) {
        self.suspended_as_donor = suspended;
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ActiveOverclock {
    output_percentage: u16,
    usage_energy_percentage: u16,
    heat_per_use: u16,
    safe_heat_threshold: u16,
    maximum_heat_threshold: u16,
    remaining_time_units: u16,
    durability_damage_when_hot: u16,
}

impl ActiveOverclock {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        output_percentage: u16,
        usage_energy_percentage: u16,
        heat_per_use: u16,
        safe_heat_threshold: u16,
        maximum_heat_threshold: u16,
        remaining_time_units: u16,
        durability_damage_when_hot: u16,
    ) -> Option<Self> {
        if output_percentage == 0
            || usage_energy_percentage == 0
            || remaining_time_units == 0
            || safe_heat_threshold > maximum_heat_threshold
        {
            return None;
        }
        Some(Self {
            output_percentage,
            usage_energy_percentage,
            heat_per_use,
            safe_heat_threshold,
            maximum_heat_threshold,
            remaining_time_units,
            durability_damage_when_hot,
        })
    }

    pub const fn output_percentage(self) -> u16 {
        self.output_percentage
    }

    pub const fn usage_energy_percentage(self) -> u16 {
        self.usage_energy_percentage
    }

    pub const fn heat_per_use(self) -> u16 {
        self.heat_per_use
    }

    pub const fn safe_heat_threshold(self) -> u16 {
        self.safe_heat_threshold
    }

    pub const fn maximum_heat_threshold(self) -> u16 {
        self.maximum_heat_threshold
    }

    pub const fn remaining_time_units(self) -> u16 {
        self.remaining_time_units
    }

    pub const fn durability_damage_when_hot(self) -> u16 {
        self.durability_damage_when_hot
    }

    pub fn elapse(&mut self) -> bool {
        self.remaining_time_units = self.remaining_time_units.saturating_sub(1);
        self.remaining_time_units == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{BodyComponentProfile, ComponentFailureEffect};

    fn component(id: &str, durability_damage: u16) -> BodyComponentState {
        let mut state = BodyComponentState::new(
            BodyComponentProfile::new(
                id.parse().unwrap(),
                format!("{id}.name"),
                20,
                2,
                ComponentFailureEffect::DisableMovement,
            )
            .unwrap(),
        );
        state.apply_damage(durability_damage);
        state
    }

    #[test]
    fn wreck_preserves_component_state_and_yields_it_only_once() {
        let mut wrecks = WreckRegistry::default();
        let locomotion = component("core:locomotion", 7);
        let id = wrecks
            .create(GridPos::new(3, 4), [locomotion.clone()])
            .unwrap()
            .unwrap();
        assert_eq!(
            wrecks.get(id).unwrap().component(locomotion.profile().id()),
            Some(&locomotion)
        );

        let recovered = wrecks
            .recover_component(id, locomotion.profile().id())
            .unwrap();
        assert_eq!(recovered, locomotion);
        assert!(wrecks.get(id).is_none());
    }

    #[test]
    fn destroyed_modules_cannot_be_resurrected_by_repair() {
        let mut module = EquipmentEngineeringState::new(10).unwrap();
        module.damage(10);
        assert_eq!(module.repair(50), 0);
        assert_eq!(module.durability(), 0);
    }
}
