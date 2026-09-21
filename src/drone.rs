use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::companion::CompanionBehavior;
use crate::content::ContentId;
use crate::entity::{EntityId, GroundItemId};
use crate::item::ItemId;
use crate::resources::{EnergyReserve, EnergyReserveError, EnergySpendError};
use crate::social::SocialGroupId;
use crate::world::{
    GridPos, Map, NeighborMode, PropagationRequest, TerrainPropagationPolicy, propagate,
};

/// Material capabilities owned by one drone chassis. Orders may only use
/// information and functions declared here; a skill never invents hardware.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DroneCapabilities {
    pub manipulator_capacity_grams: Option<u32>,
    pub decoy_intensity: Option<u16>,
    pub can_interpose: bool,
    pub autonomous_scout_range: u8,
}

/// Data-driven chassis and control-link profile. `visual_profile` is consumed
/// by presentation adapters: the terminal can map it to a glyph today and a
/// graphical client to an image later without changing simulation state.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DroneProfile {
    visual_profile: ContentId,
    link_range: u16,
    link_power: u16,
    link_difficulty: u16,
    link_attenuation_per_cell: u16,
    link_wall_attenuation_multiplier: u16,
    sensor_radius: u16,
    bandwidth_required: u16,
    movement_energy_cost: u16,
    capabilities: DroneCapabilities,
}

impl DroneProfile {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        visual_profile: ContentId,
        link_range: u16,
        link_power: u16,
        link_difficulty: u16,
        link_attenuation_per_cell: u16,
        link_wall_attenuation_multiplier: u16,
        sensor_radius: u16,
        bandwidth_required: u16,
        movement_energy_cost: u16,
        capabilities: DroneCapabilities,
    ) -> Result<Self, DroneProfileError> {
        if link_range == 0 {
            return Err(DroneProfileError::ZeroLinkRange);
        }
        if link_power == 0 || link_difficulty == 0 {
            return Err(DroneProfileError::ZeroLinkScore);
        }
        if link_attenuation_per_cell == 0 || link_wall_attenuation_multiplier == 0 {
            return Err(DroneProfileError::ZeroLinkAttenuation);
        }
        if sensor_radius == 0 {
            return Err(DroneProfileError::ZeroSensorRadius);
        }
        if bandwidth_required == 0 {
            return Err(DroneProfileError::ZeroBandwidth);
        }
        Ok(Self {
            visual_profile,
            link_range,
            link_power,
            link_difficulty,
            link_attenuation_per_cell,
            link_wall_attenuation_multiplier,
            sensor_radius,
            bandwidth_required,
            movement_energy_cost,
            capabilities,
        })
    }

    pub const fn visual_profile(&self) -> &ContentId {
        &self.visual_profile
    }

    pub const fn link_range(&self) -> u16 {
        self.link_range
    }

    pub const fn sensor_radius(&self) -> u16 {
        self.sensor_radius
    }

    pub const fn bandwidth_required(&self) -> u16 {
        self.bandwidth_required
    }

    pub const fn movement_energy_cost(&self) -> u16 {
        self.movement_energy_cost
    }

    pub const fn capabilities(&self) -> DroneCapabilities {
        self.capabilities
    }

    /// Tests the real local link. Range is an absolute cap; walls attenuate
    /// the signal but do not become universal radio-proof barriers.
    pub fn link_reaches(
        &self,
        map: &Map,
        controller: GridPos,
        drone: GridPos,
        jamming_penalty: u16,
    ) -> bool {
        let distance = controller
            .x
            .abs_diff(drone.x)
            .max(controller.y.abs_diff(drone.y));
        if distance > u32::from(self.link_range) {
            return false;
        }
        let available_power = self.link_power.saturating_sub(jamming_penalty);
        let Some(excess) = available_power.checked_sub(self.link_difficulty) else {
            return false;
        };
        let maximum_cost = excess / self.link_attenuation_per_cell;
        propagate(
            map,
            PropagationRequest {
                origin: controller,
                maximum_cost,
                neighbor_mode: NeighborMode::CardinalAndDiagonal,
            },
            &TerrainPropagationPolicy {
                floor_cost: Some(1),
                shallow_water_cost: Some(1),
                deep_water_cost: Some(self.link_wall_attenuation_multiplier),
                wall_cost: Some(self.link_wall_attenuation_multiplier),
            },
        )
        .iter()
        .any(|cell| cell.position == drone)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum PatrolBlockedResponse {
    Stop,
    Return,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DroneCondition {
    IntegrityBelowPercent(u8),
    EnergyBelowPercent(u8),
    LocallyPerceivedDanger,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DroneConditionalResponse {
    Stop,
    Return,
    Protect(EntityId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DroneDeploymentRole {
    Hold,
    Escort,
    Guard,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DroneDeploymentAssignment {
    pub drone: EntityId,
    pub destination: GridPos,
    pub role: DroneDeploymentRole,
}

/// Explicit player-authored parameters for a drone technique. The skill
/// definition supplies limits and costs; this payload records only choices so
/// suspension/replay never has to infer them from a later UI state.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DroneDirective {
    Manifest {
        position: GridPos,
    },
    Escort {
        drone: EntityId,
        distance: u8,
    },
    Patrol {
        drone: EntityId,
        waypoints: Vec<GridPos>,
        blocked_response: PatrolBlockedResponse,
        autonomous: bool,
    },
    MobileDecoy {
        drone: EntityId,
        destination: GridPos,
    },
    Collect {
        drone: EntityId,
        item: GroundItemId,
    },
    CoordinateFire {
        drones: Vec<EntityId>,
        target: EntityId,
    },
    Interpose {
        drone: EntityId,
        ally: EntityId,
    },
    Conditional {
        drone: EntityId,
        condition: DroneCondition,
        response: DroneConditionalResponse,
    },
    Deploy {
        assignments: Vec<DroneDeploymentAssignment>,
    },
    EmergencyReturn {
        drones: Vec<EntityId>,
        destination: GridPos,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DroneCollectionPhase {
    ReachItem,
    PickUp,
    Return,
    ReportMissing,
}

/// Physical cargo currently carried by a drone. It remains attached to the
/// drone actor until an adjacent hand-off succeeds; collection never writes
/// directly into the remote controller's inventory.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DroneCargo {
    item: ItemId,
    quantity: u16,
    owner: Option<SocialGroupId>,
    mass_grams: u32,
}

impl DroneCargo {
    pub const fn new(
        item: ItemId,
        quantity: u16,
        owner: Option<SocialGroupId>,
        mass_grams: u32,
    ) -> Self {
        Self {
            item,
            quantity,
            owner,
            mass_grams,
        }
    }

    pub const fn item(&self) -> &ItemId {
        &self.item
    }

    pub const fn quantity(&self) -> u16 {
        self.quantity
    }

    pub const fn owner(&self) -> Option<&SocialGroupId> {
        self.owner.as_ref()
    }

    pub const fn mass_grams(&self) -> u32 {
        self.mass_grams
    }
}

/// Dated information accumulated locally by an autonomous scout. The report
/// is not copied into player exploration while the drone is out of contact.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DroneExplorationReport {
    cells: Vec<GridPos>,
    observed_on_turn: u64,
}

impl DroneExplorationReport {
    pub fn new(cells: Vec<GridPos>, observed_on_turn: u64) -> Self {
        Self {
            cells,
            observed_on_turn,
        }
    }

    pub fn cells(&self) -> &[GridPos] {
        &self.cells
    }

    pub const fn observed_on_turn(&self) -> u64 {
        self.observed_on_turn
    }
}

/// One persistent routine. Changing an order never grants an extra drone
/// action: the routine is consumed only during the drone's normal phase.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DroneOrder {
    Hold,
    Companion {
        controller: EntityId,
        behavior: CompanionBehavior,
    },
    Escort {
        controller: EntityId,
        distance: u8,
    },
    Patrol {
        waypoints: Vec<GridPos>,
        next_waypoint: usize,
        blocked_response: PatrolBlockedResponse,
    },
    MobileDecoy {
        destination: GridPos,
        intensity: u16,
        remaining_phases: u16,
        energy_per_phase: u16,
    },
    Collect {
        item: GroundItemId,
        return_to: GridPos,
        phase: DroneCollectionPhase,
    },
    CoordinatedAttack {
        target: EntityId,
    },
    Interpose {
        ally: EntityId,
        trigger_energy_cost: u16,
    },
    AutonomousScout {
        waypoints: Vec<GridPos>,
        next_waypoint: usize,
        remaining_unknown_steps: u8,
        return_to: GridPos,
    },
    Conditional {
        condition: DroneCondition,
        response: DroneConditionalResponse,
        base: Box<DroneOrder>,
    },
    Deploy {
        destination: GridPos,
        role: DroneDeploymentRole,
    },
    EmergencyReturn {
        destination: GridPos,
        remaining_phases: u16,
    },
}

impl DroneOrder {
    pub fn escort(controller: EntityId, distance: u8) -> Result<Self, DroneOrderError> {
        if !(1..=3).contains(&distance) {
            return Err(DroneOrderError::InvalidEscortDistance(distance));
        }
        Ok(Self::Escort {
            controller,
            distance,
        })
    }

    pub fn patrol(
        waypoints: Vec<GridPos>,
        blocked_response: PatrolBlockedResponse,
    ) -> Result<Self, DroneOrderError> {
        if waypoints.is_empty() || waypoints.len() > 6 {
            return Err(DroneOrderError::InvalidWaypointCount(waypoints.len()));
        }
        Ok(Self::Patrol {
            waypoints,
            next_waypoint: 0,
            blocked_response,
        })
    }

    pub fn conditional(
        condition: DroneCondition,
        response: DroneConditionalResponse,
        base: DroneOrder,
    ) -> Result<Self, DroneOrderError> {
        match condition {
            DroneCondition::IntegrityBelowPercent(value)
            | DroneCondition::EnergyBelowPercent(value)
                if value == 0 || value > 100 =>
            {
                return Err(DroneOrderError::InvalidPercentage(value));
            }
            _ => {}
        }
        if matches!(base, DroneOrder::Conditional { .. }) {
            return Err(DroneOrderError::NestedCondition);
        }
        Ok(Self::Conditional {
            condition,
            response,
            base: Box::new(base),
        })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DroneLifecycle {
    #[default]
    Persistent,
    Manifested,
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DroneState {
    profile: DroneProfile,
    controller: EntityId,
    energy: EnergyReserve,
    order: DroneOrder,
    order_bandwidth: u16,
    cargo: Option<DroneCargo>,
    pending_report: Option<DroneExplorationReport>,
    last_confirmed_position: GridPos,
    last_confirmed_turn: u64,
    last_confirmed_controller_position: Option<GridPos>,
    lifecycle: DroneLifecycle,
    link_active: Option<bool>,
}

impl Debug for DroneState {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut debug = formatter.debug_struct("DroneState");
        debug
            .field("profile", &self.profile)
            .field("controller", &self.controller)
            .field("energy", &self.energy)
            .field("order", &self.order)
            .field("order_bandwidth", &self.order_bandwidth)
            .field("cargo", &self.cargo)
            .field("pending_report", &self.pending_report)
            .field("last_confirmed_position", &self.last_confirmed_position)
            .field("last_confirmed_turn", &self.last_confirmed_turn);
        if self.lifecycle == DroneLifecycle::Manifested {
            debug.field("lifecycle", &self.lifecycle);
        }
        if self.link_active.is_some() {
            debug.field("link_active", &self.link_active);
        }
        if self.last_confirmed_controller_position.is_some() {
            debug.field(
                "last_confirmed_controller_position",
                &self.last_confirmed_controller_position,
            );
        }
        debug.finish()
    }
}

impl DroneState {
    pub fn new(
        profile: DroneProfile,
        controller: EntityId,
        energy_capacity: u16,
        starting_energy: u16,
        position: GridPos,
        turn: u64,
    ) -> Result<Self, EnergyReserveError> {
        Ok(Self {
            profile,
            controller,
            energy: EnergyReserve::new(energy_capacity, starting_energy)?,
            order: DroneOrder::Hold,
            order_bandwidth: 0,
            cargo: None,
            pending_report: None,
            last_confirmed_position: position,
            last_confirmed_turn: turn,
            last_confirmed_controller_position: None,
            lifecycle: DroneLifecycle::Persistent,
            link_active: None,
        })
    }

    pub const fn with_lifecycle(mut self, lifecycle: DroneLifecycle) -> Self {
        self.lifecycle = lifecycle;
        self
    }

    pub const fn lifecycle(&self) -> DroneLifecycle {
        self.lifecycle
    }

    pub const fn link_active(&self) -> Option<bool> {
        self.link_active
    }

    pub fn replace_link_active(&mut self, active: bool) -> Option<bool> {
        self.link_active.replace(active)
    }

    pub const fn profile(&self) -> &DroneProfile {
        &self.profile
    }

    pub const fn controller(&self) -> EntityId {
        self.controller
    }

    pub const fn energy(&self) -> EnergyReserve {
        self.energy
    }

    pub const fn order(&self) -> &DroneOrder {
        &self.order
    }

    pub fn replace_order(&mut self, order: DroneOrder) -> DroneOrder {
        std::mem::replace(&mut self.order, order)
    }

    pub const fn order_bandwidth(&self) -> u16 {
        self.order_bandwidth
    }

    pub fn replace_order_bandwidth(&mut self, bandwidth: u16) -> u16 {
        std::mem::replace(&mut self.order_bandwidth, bandwidth)
    }

    pub fn spend_energy(&mut self, amount: u16) -> Result<(), EnergySpendError> {
        self.energy.spend(amount)
    }

    pub const fn last_confirmed_position(&self) -> GridPos {
        self.last_confirmed_position
    }

    pub const fn last_confirmed_turn(&self) -> u64 {
        self.last_confirmed_turn
    }

    pub fn confirm_position(&mut self, position: GridPos, turn: u64) {
        self.last_confirmed_position = position;
        self.last_confirmed_turn = turn;
    }

    pub const fn last_confirmed_controller_position(&self) -> Option<GridPos> {
        self.last_confirmed_controller_position
    }

    pub fn confirm_controller_position(&mut self, position: GridPos) {
        self.last_confirmed_controller_position = Some(position);
    }

    pub const fn cargo(&self) -> Option<&DroneCargo> {
        self.cargo.as_ref()
    }

    pub fn load_cargo(&mut self, cargo: DroneCargo) -> Result<(), Box<DroneCargo>> {
        if self.cargo.is_some() {
            return Err(Box::new(cargo));
        }
        self.cargo = Some(cargo);
        Ok(())
    }

    pub fn unload_cargo(&mut self) -> Option<DroneCargo> {
        self.cargo.take()
    }

    pub const fn pending_report(&self) -> Option<&DroneExplorationReport> {
        self.pending_report.as_ref()
    }

    pub fn replace_pending_report(
        &mut self,
        report: Option<DroneExplorationReport>,
    ) -> Option<DroneExplorationReport> {
        std::mem::replace(&mut self.pending_report, report)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DroneProfileError {
    ZeroLinkRange,
    ZeroLinkScore,
    ZeroLinkAttenuation,
    ZeroSensorRadius,
    ZeroBandwidth,
}

impl Display for DroneProfileError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroLinkRange => formatter.write_str("drone link range must be positive"),
            Self::ZeroLinkScore => {
                formatter.write_str("drone link power and difficulty must be positive")
            }
            Self::ZeroLinkAttenuation => {
                formatter.write_str("drone link attenuation must be positive")
            }
            Self::ZeroSensorRadius => formatter.write_str("drone sensor radius must be positive"),
            Self::ZeroBandwidth => formatter.write_str("drone bandwidth must be positive"),
        }
    }
}

impl Error for DroneProfileError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DroneOrderError {
    InvalidEscortDistance(u8),
    InvalidWaypointCount(usize),
    InvalidPercentage(u8),
    NestedCondition,
}

impl Display for DroneOrderError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidEscortDistance(distance) => {
                write!(
                    formatter,
                    "escort distance {distance} must be between one and three"
                )
            }
            Self::InvalidWaypointCount(count) => {
                write!(
                    formatter,
                    "patrol waypoint count {count} must be between one and six"
                )
            }
            Self::InvalidPercentage(value) => {
                write!(
                    formatter,
                    "conditional percentage {value} must be between one and 100"
                )
            }
            Self::NestedCondition => formatter.write_str("drone conditions cannot be nested"),
        }
    }
}

impl Error for DroneOrderError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::{Actor, ActorRegistry};

    fn entity_at(position: GridPos) -> EntityId {
        let mut actors = ActorRegistry::default();
        actors.spawn(Actor::new(position, 10).unwrap()).unwrap()
    }

    fn profile() -> DroneProfile {
        DroneProfile::new(
            "core:test_drone_visual".parse().unwrap(),
            6,
            50,
            40,
            2,
            3,
            5,
            1,
            1,
            DroneCapabilities::default(),
        )
        .unwrap()
    }

    #[test]
    fn control_link_obeys_range_jamming_and_real_wall_attenuation() {
        let open = Map::from_ascii("#########\n#.......#\n#########").unwrap();
        let wall = Map::from_ascii("#########\n#..#....#\n#########").unwrap();
        let controller = GridPos::new(1, 1);
        let drone = GridPos::new(5, 1);

        assert!(profile().link_reaches(&open, controller, drone, 0));
        assert!(!profile().link_reaches(&wall, controller, drone, 0));
        assert!(!profile().link_reaches(&open, controller, drone, 4));
    }

    #[test]
    fn routines_are_bounded_and_keep_last_confirmed_information_dated() {
        let controller = entity_at(GridPos::new(1, 1));
        let mut state =
            DroneState::new(profile(), controller, 20, 20, GridPos::new(2, 2), 7).unwrap();
        let order = DroneOrder::patrol(
            vec![GridPos::new(3, 2), GridPos::new(4, 2)],
            PatrolBlockedResponse::Return,
        )
        .unwrap();

        state.replace_order(order.clone());
        state.confirm_position(GridPos::new(3, 2), 8);

        assert_eq!(state.order(), &order);
        assert_eq!(state.last_confirmed_position(), GridPos::new(3, 2));
        assert_eq!(state.last_confirmed_turn(), 8);
        assert!(DroneOrder::patrol(Vec::new(), PatrolBlockedResponse::Stop).is_err());
        assert!(DroneOrder::escort(controller, 4).is_err());
        assert!(
            DroneOrder::conditional(
                DroneCondition::EnergyBelowPercent(0),
                DroneConditionalResponse::Stop,
                DroneOrder::Hold,
            )
            .is_err()
        );
    }

    #[test]
    fn drone_state_lives_on_an_actor_that_occupies_its_own_grid_cell() {
        let position = GridPos::new(3, 2);
        let controller = entity_at(GridPos::new(1, 1));
        let drone = DroneState::new(profile(), controller, 20, 20, position, 0).unwrap();
        let mut actors = ActorRegistry::default();
        let id = actors
            .spawn(Actor::new(position, 10).unwrap().with_drone(drone))
            .unwrap();

        assert_eq!(actors.entity_at(position), Some(id));
        assert!(actors.get(id).unwrap().drone().is_some());
        actors.move_to(id, GridPos::new(4, 2)).unwrap();
        assert_eq!(actors.entity_at(GridPos::new(4, 2)), Some(id));
    }
}
