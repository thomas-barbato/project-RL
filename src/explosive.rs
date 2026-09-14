use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::combat::{AttackArea, AttackAreaCell, AttackProfile, ConeAttack, DamagePacket};
use crate::effects::{DamageFalloff, GroundEffectSpec, RadialDamageEffect};
use crate::entity::EntityId;
use crate::item::ItemId;
use crate::world::{
    Direction, DistanceMetric, GridPos, Map, NeighborMode, TerrainPropagationPolicy,
};

/// Compact authored shape used by skill and item content. Runtime devices
/// expand it to the same wall-aware propagation primitives as every other
/// explosion; renderers never approximate this profile independently.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExplosiveAreaProfile {
    Radial { radius: u16, falloff_per_step: u16 },
    Directional { range: u16, cone: ConeAttack },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExplosivePayloadProfile {
    area: ExplosiveAreaProfile,
    damage: DamagePacket,
    center_damage: Option<DamagePacket>,
    terrain_breach_cells: u8,
}

impl ExplosivePayloadProfile {
    pub const fn new(area: ExplosiveAreaProfile, damage: DamagePacket) -> Self {
        Self {
            area,
            damage,
            center_damage: None,
            terrain_breach_cells: 0,
        }
    }

    pub const fn with_center_damage(mut self, damage: DamagePacket) -> Self {
        self.center_damage = Some(damage);
        self
    }

    pub const fn with_terrain_breach_cells(mut self, maximum_cells: u8) -> Self {
        self.terrain_breach_cells = maximum_cells;
        self
    }

    pub const fn area(self) -> ExplosiveAreaProfile {
        self.area
    }

    pub const fn damage(self) -> DamagePacket {
        self.damage
    }

    pub const fn center_damage(self) -> Option<DamagePacket> {
        self.center_damage
    }

    pub const fn terrain_breach_cells(self) -> u8 {
        self.terrain_breach_cells
    }

    pub fn into_payload(self) -> ExplosivePayload {
        let footprint = match self.area {
            ExplosiveAreaProfile::Radial {
                radius,
                falloff_per_step,
            } => ExplosiveFootprint::Radial(RadialDamageEffect {
                maximum_cost: radius,
                neighbor_mode: NeighborMode::CardinalAndDiagonal,
                propagation_policy: TerrainPropagationPolicy::blocked_by_walls(1),
                damage: self.damage,
                falloff: if falloff_per_step == 0 {
                    DamageFalloff::None
                } else {
                    DamageFalloff::PerPropagationCost(falloff_per_step)
                },
            }),
            ExplosiveAreaProfile::Directional { range, cone } => ExplosiveFootprint::Directional {
                range,
                cone,
                damage: self.damage,
            },
        };
        let mut payload =
            ExplosivePayload::new(footprint).with_terrain_breach_cells(self.terrain_breach_cells);
        if let Some(damage) = self.center_damage {
            payload = payload.with_center_damage(damage);
        }
        payload
    }
}

/// Stable identity of one deployed explosive device during a run.
///
/// Commands and presentation refer to this identity instead of a position: a
/// mod may therefore move, conceal or stack devices without silently changing
/// which one a scheduled order controls.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExplosiveDeviceId(u64);

impl ExplosiveDeviceId {
    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExplosiveFootprint {
    Radial(RadialDamageEffect),
    Directional {
        range: u16,
        cone: ConeAttack,
        damage: DamagePacket,
    },
}

impl ExplosiveFootprint {
    pub fn affected_cells(
        &self,
        map: &Map,
        origin: GridPos,
        facing: Direction,
    ) -> Vec<AttackAreaCell> {
        match self {
            Self::Radial(effect) => effect
                .affected_cells(map, origin)
                .into_iter()
                .map(|cell| AttackAreaCell {
                    position: cell.position,
                    step: cell.cost,
                })
                .collect(),
            Self::Directional {
                range,
                cone,
                damage,
            } => {
                let (delta_x, delta_y) = facing.delta();
                let target = GridPos::new(
                    origin.x + delta_x * i32::from(*range),
                    origin.y + delta_y * i32::from(*range),
                );
                AttackProfile::new(
                    *range,
                    DistanceMetric::Chebyshev,
                    false,
                    damage.damage_type,
                    damage.amount,
                    damage.penetration,
                )
                .with_area(AttackArea::Cone(*cone))
                .affected_cells(map, origin, target)
            }
        }
    }

    pub fn damage_at_step(&self, step: u16) -> Option<DamagePacket> {
        match self {
            Self::Radial(effect) => effect.damage_at_cost(step),
            Self::Directional { damage, .. } => Some(*damage),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExplosivePayload {
    footprint: ExplosiveFootprint,
    center_damage: Option<DamagePacket>,
    ground_effect: Option<GroundEffectSpec>,
    terrain_breach_cells: u8,
}

impl ExplosivePayload {
    pub const fn new(footprint: ExplosiveFootprint) -> Self {
        Self {
            footprint,
            center_damage: None,
            ground_effect: None,
            terrain_breach_cells: 0,
        }
    }

    pub const fn with_center_damage(mut self, damage: DamagePacket) -> Self {
        self.center_damage = Some(damage);
        self
    }

    pub fn with_ground_effect(mut self, effect: GroundEffectSpec) -> Self {
        self.ground_effect = Some(effect);
        self
    }

    pub const fn with_terrain_breach_cells(mut self, maximum_cells: u8) -> Self {
        self.terrain_breach_cells = maximum_cells;
        self
    }

    pub const fn footprint(&self) -> &ExplosiveFootprint {
        &self.footprint
    }

    pub const fn center_damage(&self) -> Option<DamagePacket> {
        self.center_damage
    }

    pub const fn ground_effect(&self) -> Option<&GroundEffectSpec> {
        self.ground_effect.as_ref()
    }

    pub const fn terrain_breach_cells(&self) -> u8 {
        self.terrain_breach_cells
    }
}

/// One payload in a device sequence. Delay zero resolves when the device is
/// first triggered; later delays are counted from that same trigger turn.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScheduledExplosivePayload {
    delay_after_trigger: u16,
    payload: ExplosivePayload,
}

impl ScheduledExplosivePayload {
    pub const fn new(delay_after_trigger: u16, payload: ExplosivePayload) -> Self {
        Self {
            delay_after_trigger,
            payload,
        }
    }

    pub const fn delay_after_trigger(&self) -> u16 {
        self.delay_after_trigger
    }

    pub const fn payload(&self) -> &ExplosivePayload {
        &self.payload
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExplosiveActivation {
    Timed { trigger_turn: u64 },
    Proximity { armed_turn: u64, radius: u16 },
    Remote { maximum_link_range: u16 },
}

#[derive(Clone, PartialEq, Eq)]
pub struct ExplosiveDevice {
    id: ExplosiveDeviceId,
    material: ItemId,
    source: Option<EntityId>,
    position: GridPos,
    facing: Direction,
    activation: ExplosiveActivation,
    identified: bool,
    /// Additional optical concealment supplied by a removable physical cover.
    /// Zero preserves the historical detection profile and suspension Debug.
    optical_concealment: i16,
    neutralized: bool,
    triggered_on: Option<u64>,
    next_payload: usize,
    payloads: Vec<ScheduledExplosivePayload>,
}

impl Debug for ExplosiveDevice {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut state = formatter.debug_struct("ExplosiveDevice");
        state
            .field("id", &self.id)
            .field("material", &self.material)
            .field("source", &self.source)
            .field("position", &self.position)
            .field("facing", &self.facing)
            .field("activation", &self.activation)
            .field("identified", &self.identified);
        if self.optical_concealment != 0 {
            state.field("optical_concealment", &self.optical_concealment);
        }
        state
            .field("neutralized", &self.neutralized)
            .field("triggered_on", &self.triggered_on)
            .field("next_payload", &self.next_payload)
            .field("payloads", &self.payloads)
            .finish()
    }
}

impl ExplosiveDevice {
    #[allow(clippy::too_many_arguments)]
    fn new(
        id: ExplosiveDeviceId,
        material: ItemId,
        source: Option<EntityId>,
        position: GridPos,
        facing: Direction,
        activation: ExplosiveActivation,
        identified: bool,
        payloads: Vec<ScheduledExplosivePayload>,
    ) -> Result<Self, ExplosiveDeviceError> {
        if payloads.is_empty() {
            return Err(ExplosiveDeviceError::NoPayloads);
        }
        if !payloads
            .windows(2)
            .all(|pair| pair[0].delay_after_trigger <= pair[1].delay_after_trigger)
        {
            return Err(ExplosiveDeviceError::UnorderedPayloads);
        }
        match activation {
            ExplosiveActivation::Proximity { radius: 0, .. } => {
                return Err(ExplosiveDeviceError::ZeroActivationRange);
            }
            ExplosiveActivation::Remote {
                maximum_link_range: 0,
            } => return Err(ExplosiveDeviceError::ZeroActivationRange),
            _ => {}
        }
        Ok(Self {
            id,
            material,
            source,
            position,
            facing,
            activation,
            identified,
            optical_concealment: 0,
            neutralized: false,
            triggered_on: None,
            next_payload: 0,
            payloads,
        })
    }

    pub const fn id(&self) -> ExplosiveDeviceId {
        self.id
    }

    pub const fn material(&self) -> &ItemId {
        &self.material
    }

    pub const fn source(&self) -> Option<EntityId> {
        self.source
    }

    pub const fn position(&self) -> GridPos {
        self.position
    }

    pub const fn facing(&self) -> Direction {
        self.facing
    }

    pub const fn activation(&self) -> ExplosiveActivation {
        self.activation
    }

    pub const fn is_identified(&self) -> bool {
        self.identified
    }

    pub const fn optical_concealment(&self) -> i16 {
        self.optical_concealment
    }

    pub const fn is_neutralized(&self) -> bool {
        self.neutralized
    }

    pub const fn triggered_on(&self) -> Option<u64> {
        self.triggered_on
    }

    pub fn remaining_payloads(&self) -> usize {
        self.payloads.len().saturating_sub(self.next_payload)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ResolvedExplosivePayload {
    pub device: ExplosiveDeviceId,
    pub material: ItemId,
    pub source: Option<EntityId>,
    pub position: GridPos,
    pub facing: Direction,
    pub payload_index: usize,
    pub first_payload: bool,
    pub final_payload: bool,
    pub payload: ExplosivePayload,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExplosiveDeviceMap {
    next_id: u64,
    devices: BTreeMap<ExplosiveDeviceId, ExplosiveDevice>,
}

impl Default for ExplosiveDeviceMap {
    fn default() -> Self {
        Self {
            next_id: 1,
            devices: BTreeMap::new(),
        }
    }
}

impl ExplosiveDeviceMap {
    #[allow(clippy::too_many_arguments)]
    pub fn deploy(
        &mut self,
        material: ItemId,
        source: Option<EntityId>,
        position: GridPos,
        facing: Direction,
        activation: ExplosiveActivation,
        identified: bool,
        payloads: Vec<ScheduledExplosivePayload>,
    ) -> Result<ExplosiveDeviceId, ExplosiveDeviceError> {
        let id = ExplosiveDeviceId(self.next_id);
        let next_id = self
            .next_id
            .checked_add(1)
            .ok_or(ExplosiveDeviceError::IdSpaceExhausted)?;
        let device = ExplosiveDevice::new(
            id, material, source, position, facing, activation, identified, payloads,
        )?;
        self.next_id = next_id;
        self.devices.insert(id, device);
        Ok(id)
    }

    pub fn get(&self, id: ExplosiveDeviceId) -> Option<&ExplosiveDevice> {
        self.devices.get(&id)
    }

    pub fn at(&self, position: GridPos) -> impl Iterator<Item = &ExplosiveDevice> {
        self.devices
            .values()
            .filter(move |device| device.position == position)
    }

    pub fn iter(&self) -> impl Iterator<Item = &ExplosiveDevice> {
        self.devices.values()
    }

    pub fn is_empty(&self) -> bool {
        self.devices.is_empty()
    }

    pub fn identify(&mut self, id: ExplosiveDeviceId) -> bool {
        let Some(device) = self.devices.get_mut(&id) else {
            return false;
        };
        device.identified = true;
        true
    }

    pub fn set_optical_concealment(
        &mut self,
        id: ExplosiveDeviceId,
        bonus: i16,
    ) -> Result<(), ExplosiveDeviceError> {
        let device = self
            .devices
            .get_mut(&id)
            .ok_or(ExplosiveDeviceError::UnknownDevice(id))?;
        if device.triggered_on.is_some() {
            return Err(ExplosiveDeviceError::AlreadyTriggered(id));
        }
        device.optical_concealment = bonus.max(0);
        Ok(())
    }

    pub fn neutralize(&mut self, id: ExplosiveDeviceId) -> Result<(), ExplosiveDeviceError> {
        let device = self
            .devices
            .get_mut(&id)
            .ok_or(ExplosiveDeviceError::UnknownDevice(id))?;
        if device.triggered_on.is_some() {
            return Err(ExplosiveDeviceError::AlreadyTriggered(id));
        }
        device.neutralized = true;
        Ok(())
    }

    pub fn recover(&mut self, id: ExplosiveDeviceId) -> Result<ItemId, ExplosiveDeviceError> {
        let device = self
            .devices
            .get(&id)
            .ok_or(ExplosiveDeviceError::UnknownDevice(id))?;
        if !device.neutralized {
            return Err(ExplosiveDeviceError::NotNeutralized(id));
        }
        Ok(self
            .devices
            .remove(&id)
            .expect("checked device remains present")
            .material)
    }

    pub fn trigger_remote(
        &mut self,
        id: ExplosiveDeviceId,
        current_turn: u64,
    ) -> Result<(), ExplosiveDeviceError> {
        let device = self
            .devices
            .get_mut(&id)
            .ok_or(ExplosiveDeviceError::UnknownDevice(id))?;
        if device.neutralized {
            return Err(ExplosiveDeviceError::Neutralized(id));
        }
        if device.triggered_on.is_some() {
            return Err(ExplosiveDeviceError::AlreadyTriggered(id));
        }
        if !matches!(device.activation, ExplosiveActivation::Remote { .. }) {
            return Err(ExplosiveDeviceError::NotRemote(id));
        }
        device.triggered_on = Some(current_turn);
        Ok(())
    }

    /// Replaces the activation of known, untriggered receivers atomically.
    /// Each delay is measured from installation, excluding that command's own
    /// environment phase just like other Pn+A1 procedures.
    pub fn program(
        &mut self,
        orders: &[(ExplosiveDeviceId, u16)],
        current_turn: u64,
    ) -> Result<(), ExplosiveDeviceError> {
        for (id, delay) in orders {
            let device = self
                .devices
                .get(id)
                .ok_or(ExplosiveDeviceError::UnknownDevice(*id))?;
            if *delay == 0 {
                return Err(ExplosiveDeviceError::ZeroProgrammingDelay);
            }
            if device.neutralized {
                return Err(ExplosiveDeviceError::Neutralized(*id));
            }
            if device.triggered_on.is_some() {
                return Err(ExplosiveDeviceError::AlreadyTriggered(*id));
            }
            if !device.identified {
                return Err(ExplosiveDeviceError::NotIdentified(*id));
            }
        }
        for (id, delay) in orders {
            self.devices
                .get_mut(id)
                .expect("validated program device remains present")
                .activation = ExplosiveActivation::Timed {
                trigger_turn: current_turn.saturating_add(u64::from(*delay)),
            };
        }
        Ok(())
    }

    /// Resolves every payload due on this exact environment phase in stable ID
    /// and stage order. Actor positions are facts supplied by the simulation;
    /// the device layer does not need access to actor registries or AI.
    pub fn take_ready_payloads(
        &mut self,
        current_turn: u64,
        actors: &[(EntityId, GridPos)],
    ) -> Vec<ResolvedExplosivePayload> {
        let ids: Vec<_> = self.devices.keys().copied().collect();
        let mut ready = Vec::new();
        let mut spent = Vec::new();
        for id in ids {
            let Some(device) = self.devices.get_mut(&id) else {
                continue;
            };
            if device.neutralized {
                continue;
            }
            if device.triggered_on.is_none() {
                let triggered = match device.activation {
                    ExplosiveActivation::Timed { trigger_turn } => current_turn >= trigger_turn,
                    ExplosiveActivation::Proximity { armed_turn, radius } => {
                        current_turn >= armed_turn
                            && actors.iter().any(|(entity, position)| {
                                Some(*entity) != device.source
                                    && is_within_chebyshev(device.position, *position, radius)
                            })
                    }
                    ExplosiveActivation::Remote { .. } => false,
                };
                if triggered {
                    device.triggered_on = Some(current_turn);
                }
            }
            let Some(triggered_on) = device.triggered_on else {
                continue;
            };
            while let Some(stage) = device.payloads.get(device.next_payload) {
                if triggered_on.saturating_add(u64::from(stage.delay_after_trigger)) > current_turn
                {
                    break;
                }
                let payload_index = device.next_payload;
                device.next_payload += 1;
                let final_payload = device.next_payload == device.payloads.len();
                ready.push(ResolvedExplosivePayload {
                    device: id,
                    material: device.material.clone(),
                    source: device.source,
                    position: device.position,
                    facing: device.facing,
                    payload_index,
                    first_payload: payload_index == 0,
                    final_payload,
                    payload: stage.payload.clone(),
                });
            }
            if device.next_payload == device.payloads.len() {
                spent.push(id);
            }
        }
        for id in spent {
            self.devices.remove(&id);
        }
        ready
    }
}

fn is_within_chebyshev(origin: GridPos, target: GridPos, radius: u16) -> bool {
    let x = (i64::from(origin.x) - i64::from(target.x)).abs();
    let y = (i64::from(origin.y) - i64::from(target.y)).abs();
    x.max(y) <= i64::from(radius)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExplosiveDeviceError {
    NoPayloads,
    UnorderedPayloads,
    ZeroActivationRange,
    ZeroProgrammingDelay,
    IdSpaceExhausted,
    UnknownDevice(ExplosiveDeviceId),
    AlreadyTriggered(ExplosiveDeviceId),
    NotRemote(ExplosiveDeviceId),
    NotNeutralized(ExplosiveDeviceId),
    Neutralized(ExplosiveDeviceId),
    NotIdentified(ExplosiveDeviceId),
}

impl Display for ExplosiveDeviceError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoPayloads => write!(formatter, "an explosive device requires a payload"),
            Self::UnorderedPayloads => {
                write!(formatter, "explosive payload delays must be ordered")
            }
            Self::ZeroActivationRange => {
                write!(formatter, "explosive activation range must be positive")
            }
            Self::ZeroProgrammingDelay => {
                write!(formatter, "programmed explosive delay must be positive")
            }
            Self::IdSpaceExhausted => write!(formatter, "explosive device ID space exhausted"),
            Self::UnknownDevice(id) => write!(formatter, "unknown explosive device {}", id.get()),
            Self::AlreadyTriggered(id) => {
                write!(
                    formatter,
                    "explosive device {} has already triggered",
                    id.get()
                )
            }
            Self::NotRemote(id) => {
                write!(
                    formatter,
                    "explosive device {} has no remote receiver",
                    id.get()
                )
            }
            Self::NotNeutralized(id) => {
                write!(
                    formatter,
                    "explosive device {} is not neutralized",
                    id.get()
                )
            }
            Self::Neutralized(id) => {
                write!(formatter, "explosive device {} is neutralized", id.get())
            }
            Self::NotIdentified(id) => {
                write!(formatter, "explosive device {} is not identified", id.get())
            }
        }
    }
}

impl Error for ExplosiveDeviceError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::DamageType;
    use crate::effects::DamageFalloff;
    use crate::entity::{Actor, ActorRegistry};
    use crate::world::{NeighborMode, Terrain, TerrainPropagationPolicy};

    fn material() -> ItemId {
        "core:test_charge".parse().unwrap()
    }

    fn payload(amount: u16) -> ScheduledExplosivePayload {
        ScheduledExplosivePayload::new(
            0,
            ExplosivePayload::new(ExplosiveFootprint::Radial(RadialDamageEffect {
                maximum_cost: 1,
                neighbor_mode: NeighborMode::CardinalAndDiagonal,
                propagation_policy: TerrainPropagationPolicy::blocked_by_walls(1),
                damage: DamagePacket::new(amount, DamageType::Explosive, 0),
                falloff: DamageFalloff::None,
            })),
        )
    }

    fn two_actor_ids() -> (EntityId, EntityId) {
        let mut actors = ActorRegistry::default();
        let source = actors
            .spawn(Actor::new(GridPos::new(1, 1), 5).unwrap())
            .unwrap();
        let contact = actors
            .spawn(Actor::new(GridPos::new(2, 1), 5).unwrap())
            .unwrap();
        (source, contact)
    }

    #[test]
    fn timed_device_triggers_once_in_stable_order() {
        let mut devices = ExplosiveDeviceMap::default();
        let first = devices
            .deploy(
                material(),
                None,
                GridPos::new(2, 2),
                Direction::North,
                ExplosiveActivation::Timed { trigger_turn: 4 },
                true,
                vec![payload(12)],
            )
            .unwrap();
        let second = devices
            .deploy(
                material(),
                None,
                GridPos::new(3, 2),
                Direction::North,
                ExplosiveActivation::Timed { trigger_turn: 4 },
                true,
                vec![payload(10)],
            )
            .unwrap();

        assert!(devices.take_ready_payloads(3, &[]).is_empty());
        assert_eq!(
            devices
                .take_ready_payloads(4, &[])
                .iter()
                .map(|resolved| resolved.device)
                .collect::<Vec<_>>(),
            vec![first, second]
        );
        assert!(devices.is_empty());
        assert!(devices.take_ready_payloads(5, &[]).is_empty());
    }

    #[test]
    fn proximity_device_waits_for_arming_and_a_compatible_contact() {
        let mut devices = ExplosiveDeviceMap::default();
        let (source, contact) = two_actor_ids();
        devices
            .deploy(
                material(),
                Some(source),
                GridPos::new(2, 2),
                Direction::North,
                ExplosiveActivation::Proximity {
                    armed_turn: 3,
                    radius: 1,
                },
                true,
                vec![payload(20)],
            )
            .unwrap();

        assert!(
            devices
                .take_ready_payloads(2, &[(contact, GridPos::new(2, 3))])
                .is_empty()
        );
        assert!(
            devices
                .take_ready_payloads(
                    3,
                    &[(source, GridPos::new(2, 3)), (contact, GridPos::new(4, 4)),],
                )
                .is_empty()
        );
        assert_eq!(
            devices
                .take_ready_payloads(
                    3,
                    &[(source, GridPos::new(2, 3)), (contact, GridPos::new(2, 3)),],
                )
                .len(),
            1
        );
    }

    #[test]
    fn sequential_payloads_keep_the_original_clock() {
        let mut devices = ExplosiveDeviceMap::default();
        devices
            .deploy(
                material(),
                None,
                GridPos::new(2, 2),
                Direction::East,
                ExplosiveActivation::Timed { trigger_turn: 5 },
                true,
                vec![
                    payload(30),
                    ScheduledExplosivePayload::new(1, payload(12).payload().clone()),
                ],
            )
            .unwrap();

        let first = devices.take_ready_payloads(5, &[]);
        assert_eq!(first.len(), 1);
        assert!(first[0].first_payload);
        assert!(!first[0].final_payload);
        assert_eq!(devices.iter().next().unwrap().remaining_payloads(), 1);
        let second = devices.take_ready_payloads(6, &[]);
        assert_eq!(second.len(), 1);
        assert_eq!(second[0].payload_index, 1);
        assert!(second[0].final_payload);
        assert!(devices.is_empty());
    }

    #[test]
    fn programming_is_atomic_and_remote_trigger_is_explicit() {
        let mut devices = ExplosiveDeviceMap::default();
        let remote = devices
            .deploy(
                material(),
                None,
                GridPos::new(2, 2),
                Direction::North,
                ExplosiveActivation::Remote {
                    maximum_link_range: 6,
                },
                true,
                vec![payload(10)],
            )
            .unwrap();
        let before = devices.clone();
        assert_eq!(
            devices.program(&[(remote, 0)], 7),
            Err(ExplosiveDeviceError::ZeroProgrammingDelay)
        );
        assert_eq!(devices, before);
        devices.trigger_remote(remote, 7).unwrap();
        assert_eq!(devices.take_ready_payloads(7, &[]).len(), 1);
    }

    #[test]
    fn neutralized_device_is_inert_and_recoverable_as_its_real_material() {
        let mut devices = ExplosiveDeviceMap::default();
        let id = devices
            .deploy(
                material(),
                None,
                GridPos::new(2, 2),
                Direction::North,
                ExplosiveActivation::Timed { trigger_turn: 1 },
                true,
                vec![payload(10)],
            )
            .unwrap();
        devices.neutralize(id).unwrap();
        assert!(devices.take_ready_payloads(2, &[]).is_empty());
        assert_eq!(devices.recover(id), Ok(material()));
        assert!(devices.is_empty());
    }

    #[test]
    fn directional_payload_reuses_the_combat_cone_without_a_black_box_shape() {
        let map = Map::filled(9, 7, Terrain::Floor).unwrap();
        let footprint = ExplosiveFootprint::Directional {
            range: 4,
            cone: ConeAttack::new(1, 2, 2).unwrap(),
            damage: DamagePacket::new(8, DamageType::Explosive, 0),
        };
        let cells = footprint.affected_cells(&map, GridPos::new(2, 3), Direction::East);

        assert!(cells.iter().all(|cell| cell.position.x > 2));
        assert!(cells.iter().any(|cell| cell.position == GridPos::new(6, 3)));
        assert!(!cells.iter().any(|cell| cell.position == GridPos::new(2, 2)));
    }
}
