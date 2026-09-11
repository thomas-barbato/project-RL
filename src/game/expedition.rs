//! Persistent, deterministic zones. Presentation never advances this simulation.
use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

use crate::ai::AiBehavior;
use crate::content::ContentId;
use crate::effects::GroundEffectMap;
use crate::entity::{Actor, ActorRegistry, GroundItemRegistry};
use crate::facility::{
    FacilityBlueprint, FacilityEvent, FacilityState, SecurityAlarmResponseContext, WorkerRole,
};
use crate::item::ItemId;
use crate::social::{ObservedPropertyTake, SocialGroupId};
use crate::status::StatusTrigger;
use crate::world::generation::{MapValidationRules, validate_interactive_map};
use crate::world::{Direction, GridPos, Map, MovementTraceMap, VisibilityState};

use super::{
    CommandOutcome, CommandRejection, GameCommand, GameEvent, GameRng, GameState, RunStatus,
    TurnPhase,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZoneInfo {
    pub id: ContentId,
    pub name: String,
    pub kind: ContentId,
    pub depth: u16,
}

/// Input from any authored/procedural content provider. Spawns are instantiated
/// once, on first entry, with global IDs and their own deterministic RNG stream.
#[derive(Clone, Debug)]
pub struct ZoneBlueprint {
    pub info: ZoneInfo,
    pub map: Map,
    pub entrance: GridPos,
    pub seed: u64,
    pub actors: Vec<Actor>,
    pub loot: Vec<(GridPos, ItemId, u16)>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZoneLink {
    pub destination: ContentId,
    pub arrival: GridPos,
}

struct ZoneState {
    map: Map,
    actors: ActorRegistry,
    exit: Option<GridPos>,
    rng: GameRng,
    visibility: VisibilityState,
    ground: GroundItemRegistry,
    traces: MovementTraceMap,
    ground_effects: GroundEffectMap,
}

impl Debug for ZoneState {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut state = formatter.debug_struct("ZoneState");
        state
            .field("map", &self.map)
            .field("actors", &self.actors)
            .field("exit", &self.exit)
            .field("rng", &self.rng)
            .field("visibility", &self.visibility)
            .field("ground", &self.ground)
            .field("traces", &self.traces);
        if !self.ground_effects.is_empty() {
            state.field("ground_effects", &self.ground_effects);
        }
        state.finish()
    }
}

pub struct WorldState {
    active: GameState,
    current: Option<ContentId>,
    information: BTreeMap<ContentId, ZoneInfo>,
    pending: BTreeMap<ContentId, ZoneBlueprint>,
    inactive: BTreeMap<ContentId, ZoneState>,
    links: BTreeMap<(ContentId, GridPos), ZoneLink>,
    facilities: BTreeMap<ContentId, FacilityState>,
    pending_facilities: BTreeMap<ContentId, FacilityBlueprint>,
}

// Keep the historical Debug representation byte-for-byte identical while the
// new registries are empty: suspension versions 1-4 fingerprint this text.
impl Debug for WorldState {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut state = formatter.debug_struct("WorldState");
        state
            .field("active", &self.active)
            .field("current", &self.current)
            .field("information", &self.information)
            .field("pending", &self.pending)
            .field("inactive", &self.inactive)
            .field("links", &self.links);
        if !self.facilities.is_empty() {
            state.field("facilities", &self.facilities);
        }
        if !self.pending_facilities.is_empty() {
            state.field("pending_facilities", &self.pending_facilities);
        }
        state.finish()
    }
}

// Existing read APIs still refer to the active zone. World commands must go
// through WorldState::process_player_command, never through a rendered view.
impl Deref for WorldState {
    type Target = GameState;
    fn deref(&self) -> &GameState {
        &self.active
    }
}
impl DerefMut for WorldState {
    fn deref_mut(&mut self) -> &mut GameState {
        &mut self.active
    }
}

impl WorldState {
    /// Legacy single-map replay, with byte-for-byte unchanged GameState Debug.
    pub fn single(active: GameState) -> Self {
        Self {
            active,
            current: None,
            information: BTreeMap::new(),
            pending: BTreeMap::new(),
            inactive: BTreeMap::new(),
            links: BTreeMap::new(),
            facilities: BTreeMap::new(),
            pending_facilities: BTreeMap::new(),
        }
    }

    pub fn enable(&mut self, info: ZoneInfo) -> Result<(), String> {
        if self.current.is_some() {
            return Err("World already initialized".into());
        }
        self.current = Some(info.id.clone());
        self.information.insert(info.id.clone(), info);
        self.active.exit = None;
        Ok(())
    }

    pub fn active_game(&self) -> &GameState {
        &self.active
    }

    pub fn player_may_take_property_of(&self, owner: &SocialGroupId) -> bool {
        self.active.player_may_take_property_of(owner)
    }

    pub fn grant_player_property_take_authorization(&mut self, owner: SocialGroupId) -> bool {
        self.active.grant_player_property_take_authorization(owner)
    }
    pub fn current_zone(&self) -> Option<&ZoneInfo> {
        self.current
            .as_ref()
            .and_then(|id| self.information.get(id))
    }
    pub fn visited_zone_count(&self) -> usize {
        1 + self.inactive.len()
    }
    pub fn passage(&self, at: GridPos) -> Option<&ZoneLink> {
        self.current
            .as_ref()
            .and_then(|id| self.links.get(&(id.clone(), at)))
    }

    fn zone_egresses(&self, zone: &ContentId) -> Vec<GridPos> {
        let mut egresses: Vec<_> = self
            .links
            .keys()
            .filter_map(|(source, position)| (source == zone).then_some(*position))
            .collect();
        if self.current.as_ref() == Some(zone)
            && let Some(exit) = self.active.exit()
            && !egresses.contains(&exit)
        {
            egresses.push(exit);
        }
        egresses.sort();
        egresses
    }
    pub fn destination_name(&self, link: &ZoneLink) -> &str {
        self.information
            .get(&link.destination)
            .map_or("Zone", |info| info.name.as_str())
    }

    pub fn active_facility(&self) -> Option<&FacilityState> {
        self.current
            .as_ref()
            .and_then(|zone| self.facilities.get(zone))
    }

    pub fn facility_in_zone(&self, zone: &ContentId) -> Option<&FacilityState> {
        self.facilities.get(zone)
    }

    pub fn active_worker_role(&self, worker: crate::entity::EntityId) -> Option<WorkerRole> {
        self.active_facility()
            .and_then(|facility| facility.worker_role(worker))
    }

    /// Installs a deterministic local simulation in the current, pending or
    /// already visited zone. Pending definitions are instantiated only once,
    /// when their actors receive global entity IDs.
    pub fn register_facility(
        &mut self,
        zone: ContentId,
        blueprint: FacilityBlueprint,
    ) -> Result<(), String> {
        if self.facilities.contains_key(&zone) || self.pending_facilities.contains_key(&zone) {
            return Err("Zone already has a facility simulation".into());
        }
        if self.current.as_ref() == Some(&zone) {
            let facility =
                FacilityState::instantiate(blueprint, &mut self.active.map, &self.active.actors)
                    .map_err(|error| error.to_string())?;
            self.facilities.insert(zone, facility);
            return Ok(());
        }
        if let Some(state) = self.inactive.get_mut(&zone) {
            let facility = FacilityState::instantiate(blueprint, &mut state.map, &state.actors)
                .map_err(|error| error.to_string())?;
            self.facilities.insert(zone, facility);
            return Ok(());
        }
        if self.pending.contains_key(&zone) {
            self.pending_facilities.insert(zone, blueprint);
            return Ok(());
        }
        Err("Unknown zone for facility simulation".into())
    }

    pub fn add_zone(&mut self, blueprint: ZoneBlueprint) -> Result<(), String> {
        if self.current.is_none() || self.information.contains_key(&blueprint.info.id) {
            return Err("Zone missing its world or duplicate zone ID".into());
        }
        validate_interactive_map(
            &blueprint.map,
            blueprint.entrance,
            blueprint.entrance,
            &[],
            MapValidationRules::default(),
        )
        .map_err(|e| e.to_string())?;
        // Validate all definitions and placements now, not after leaving a zone.
        self.prepare_zone(&blueprint)?;
        self.information
            .insert(blueprint.info.id.clone(), blueprint.info.clone());
        self.pending.insert(blueprint.info.id.clone(), blueprint);
        Ok(())
    }

    fn map_for(&self, id: &ContentId) -> Option<(&Map, GridPos)> {
        if self.current.as_ref() == Some(id) {
            return self.active.player_position().map(|p| (&self.active.map, p));
        }
        self.pending.get(id).map(|b| (&b.map, b.entrance))
    }

    /// Register a reciprocal connection before play. Both directions must be
    /// walkable/reachable after decoration and cannot overwrite another link.
    pub fn connect(
        &mut self,
        from: ContentId,
        at: GridPos,
        to: ContentId,
        arrival: GridPos,
    ) -> Result<(), String> {
        if from == to
            || self.links.contains_key(&(from.clone(), at))
            || self.links.contains_key(&(to.clone(), arrival))
        {
            return Err("Duplicate passage or self connection".into());
        }
        for (id, anchor) in [(&from, at), (&to, arrival)] {
            let (map, start) = self.map_for(id).ok_or("Unknown/unavailable zone")?;
            if !map.is_walkable(anchor) {
                return Err("Blocked passage".into());
            }
            validate_interactive_map(map, start, anchor, &[], MapValidationRules::default())
                .map_err(|e| e.to_string())?;
            if self
                .pending
                .get(id)
                .is_some_and(|b| b.actors.iter().any(|a| a.position() == anchor))
            {
                return Err("Actor spawned on a passage".into());
            }
        }
        self.links.insert(
            (from.clone(), at),
            ZoneLink {
                destination: to.clone(),
                arrival,
            },
        );
        self.links.insert(
            (to, arrival),
            ZoneLink {
                destination: from,
                arrival: at,
            },
        );
        Ok(())
    }

    fn prepare_zone(&self, blueprint: &ZoneBlueprint) -> Result<ZoneState, String> {
        let mut probe = GameState::new_with_rules(
            blueprint.map.clone(),
            blueprint.entrance,
            blueprint.seed,
            self.active.rules.clone(),
        )
        .map_err(|e| e.to_string())?;
        probe.actors.remove(probe.player);
        probe.actors.synchronize_ids(&self.active.actors);
        probe
            .ground_items
            .synchronize_ids(&self.active.ground_items);
        for actor in &blueprint.actors {
            probe
                .spawn_actor(actor.clone())
                .map_err(|e| e.to_string())?;
        }
        for (position, item, quantity) in &blueprint.loot {
            probe
                .spawn_ground_item(*position, item.clone(), *quantity)
                .map_err(|e| e.to_string())?;
        }
        Ok(ZoneState {
            map: probe.map,
            actors: probe.actors,
            exit: None,
            rng: probe.rng,
            visibility: VisibilityState::default(),
            ground: probe.ground_items,
            traces: MovementTraceMap::default(),
            ground_effects: GroundEffectMap::default(),
        })
    }

    pub fn process_player_command(&mut self, command: GameCommand) -> CommandOutcome {
        let previous_turn = self.active.turn;
        let unauthorized_property_take = if matches!(&command, GameCommand::PickUp) {
            self.active.player_position().and_then(|position| {
                let ground_item = self.active.ground_items().item_at(position)?;
                let stack = self.active.ground_items().get(ground_item)?;
                let owner = stack.owner()?.clone();
                (!self.active.player_may_take_property_of(&owner)).then(|| ObservedPropertyTake {
                    turn: self.active.turn(),
                    taker: self.active.player_id(),
                    owner,
                    item: stack.item().clone(),
                    quantity: stack.quantity(),
                    at: position,
                })
            })
        } else {
            None
        };
        let interaction_target = match &command {
            GameCommand::Interact { target } => Some(*target),
            _ => None,
        };
        let outcome = if let Some(target) = interaction_target
            && let Some(link) = self.passage(target).cloned()
        {
            self.travel(target, link)
        } else if let Some(target) = interaction_target
            && let Some(outcome) = self.interact_with_facility(target)
        {
            outcome
        } else {
            self.active.process_player_command(command)
        };
        if outcome == CommandOutcome::Applied
            && let Some(incident) = unauthorized_property_take
        {
            let current = self.current.clone();
            let egresses = current
                .as_ref()
                .map(|zone| self.zone_egresses(zone))
                .unwrap_or_default();
            if let Some(facility) = current
                .as_ref()
                .and_then(|zone| self.facilities.get_mut(zone))
            {
                let mut events =
                    facility.observe_unauthorized_property_take(&self.active.map, &incident);
                let alarm_sources: Vec<_> = events
                    .iter()
                    .filter_map(|event| match event {
                        FacilityEvent::SecurityAlarmRaised { installation, .. } => {
                            Some(installation.clone())
                        }
                        _ => None,
                    })
                    .collect();
                match facility.activate_security_alarm_responses(
                    &alarm_sources,
                    &mut self.active.map,
                    SecurityAlarmResponseContext {
                        actors: &self.active.actors,
                        ground: &self.active.ground_items,
                        protected_position: Some(incident.at),
                        egresses: &egresses,
                        turn: self.active.turn,
                    },
                ) {
                    Ok(response_events) => events.extend(response_events),
                    Err(error) => events.push(FacilityEvent::SimulationFault(error.to_string())),
                }
                let visible: Vec<_> = events
                    .into_iter()
                    .filter(|event| {
                        facility_event_is_visible(
                            event,
                            facility,
                            &self.active.actors,
                            &self.active.player_visibility,
                        )
                    })
                    .map(GameEvent::Facility)
                    .collect();
                self.active.events.extend(visible);
            }
        }
        if self.active.turn != previous_turn && self.active.status == RunStatus::Active {
            if let Some(current) = self.current.clone()
                && let Some(facility) = self.facilities.get_mut(&current)
            {
                let result = (|| {
                    let mut events = facility
                        .expire_security_alarm_responses(&mut self.active.map, self.active.turn)?;
                    events.extend(facility.tick(
                        &mut self.active.map,
                        &mut self.active.actors,
                        &mut self.active.ground_items,
                    )?);
                    Ok::<_, crate::facility::FacilityRuntimeError>(events)
                })();
                if let Some(origin) = self.active.player_position() {
                    self.active.player_visibility.recompute(
                        &self.active.map,
                        origin,
                        self.active.rules.player_field_of_view,
                    );
                }
                let events = result.unwrap_or_else(|error| {
                    vec![FacilityEvent::SimulationFault(error.to_string())]
                });
                let visible: Vec<_> = events
                    .into_iter()
                    .filter(|event| {
                        facility_event_is_visible(
                            event,
                            facility,
                            &self.active.actors,
                            &self.active.player_visibility,
                        )
                    })
                    .map(GameEvent::Facility)
                    .collect();
                self.active.events.extend(visible);
            }
            // BTreeMap order + independent RNG streams make the result stable.
            for (id, zone) in &mut self.inactive {
                self.active.tick_background(zone, previous_turn);
                if let Some(facility) = self.facilities.get_mut(id) {
                    let _ =
                        facility.expire_security_alarm_responses(&mut zone.map, self.active.turn);
                    let _ = facility.tick(&mut zone.map, &mut zone.actors, &mut zone.ground);
                }
            }
        }
        outcome
    }

    fn interact_with_facility(&mut self, target: GridPos) -> Option<CommandOutcome> {
        let zone = self.current.as_ref()?;
        let facility = self.facilities.get_mut(zone)?;
        if !facility.is_depot_at(target) {
            return None;
        }
        let reject = |reason| Some(CommandOutcome::Rejected(reason));
        if self.active.status != RunStatus::Active {
            return reject(CommandRejection::RunEnded);
        }
        if self.active.phase != TurnPhase::AwaitingPlayer {
            return reject(CommandRejection::NotPlayersTurn);
        }
        let Some(origin) = self.active.player_position() else {
            return reject(CommandRejection::MissingPlayer);
        };
        if !origin.cardinal_neighbors().contains(&target)
            || !self.active.player_visibility.is_visible(target)
        {
            return reject(CommandRejection::InteractionOutOfReach);
        }
        let player = self.active.player;
        match facility.deposit_player_material(self.active.player_inventory_mut()) {
            Ok(Some(deposit)) => {
                self.active.events.push(GameEvent::Facility(
                    FacilityEvent::PlayerMaterialDeposited {
                        player,
                        at: target,
                        item: deposit.item,
                        quantity: deposit.quantity,
                    },
                ));
                self.active.complete_turn();
                Some(CommandOutcome::Applied)
            }
            Ok(None) => reject(CommandRejection::NoMaterialForDepot),
            Err(_) => reject(CommandRejection::FacilityUnavailable),
        }
    }

    fn travel(&mut self, at: GridPos, link: ZoneLink) -> CommandOutcome {
        let reject = |reason| CommandOutcome::Rejected(reason);
        if self.active.status != RunStatus::Active {
            return reject(CommandRejection::RunEnded);
        }
        if self.active.phase != TurnPhase::AwaitingPlayer {
            return reject(CommandRejection::NotPlayersTurn);
        }
        let Some(origin) = self.active.player_position() else {
            return reject(CommandRejection::MissingPlayer);
        };
        if (origin != at && !origin.cardinal_neighbors().contains(&at))
            || !self.active.player_visibility.is_visible(at)
        {
            return reject(CommandRejection::InteractionOutOfReach);
        }
        if !self.active.map.is_walkable(at) {
            return reject(CommandRejection::PassageUnavailable);
        }
        if let Some(blueprint) = self.pending.get(&link.destination) {
            let Ok(mut zone) = self.prepare_zone(blueprint) else {
                return reject(CommandRejection::PassageUnavailable);
            };
            let pending_facility = self.pending_facilities.get(&link.destination).cloned();
            let facility = if let Some(blueprint) = pending_facility {
                let Ok(facility) =
                    FacilityState::instantiate(blueprint, &mut zone.map, &zone.actors)
                else {
                    return reject(CommandRejection::PassageUnavailable);
                };
                Some(facility)
            } else {
                None
            };
            if zone.actors.entity_at(link.arrival).is_some() {
                return reject(CommandRejection::PassageObstructed);
            }
            self.pending.remove(&link.destination);
            if let Some(facility) = facility {
                self.pending_facilities.remove(&link.destination);
                self.facilities.insert(link.destination.clone(), facility);
            }
            self.inactive.insert(link.destination.clone(), zone);
        }
        let Some(zone) = self.inactive.get(&link.destination) else {
            return reject(CommandRejection::PassageUnavailable);
        };
        if !zone.map.is_walkable(link.arrival) {
            return reject(CommandRejection::PassageUnavailable);
        }
        if zone.actors.entity_at(link.arrival).is_some() {
            return reject(CommandRejection::PassageObstructed);
        }
        let mut next = self
            .inactive
            .remove(&link.destination)
            .expect("checked zone");
        let from = self
            .current
            .replace(link.destination.clone())
            .expect("initialized world");
        let player = self
            .active
            .actors
            .remove(self.active.player)
            .expect("checked player");
        next.actors.synchronize_ids(&self.active.actors);
        next.ground.synchronize_ids(&self.active.ground_items);
        self.active.swap_zone(&mut next);
        next.visibility.clear_visible();
        self.active
            .actors
            .insert_existing(self.active.player, player);
        self.active
            .actors
            .move_to(self.active.player, link.arrival)
            .expect("inserted player");
        self.inactive.insert(from.clone(), next);
        self.active.player_visibility.recompute(
            &self.active.map,
            link.arrival,
            self.active.rules.player_field_of_view,
        );
        self.active.events.push(GameEvent::ZoneChanged {
            from,
            to: link.destination,
            arrival: link.arrival,
        });
        self.active.events.push(GameEvent::VisibilityUpdated {
            observer: self.active.player,
            origin: link.arrival,
        });
        self.active.complete_turn();
        CommandOutcome::Applied
    }
}

fn facility_event_is_visible(
    event: &FacilityEvent,
    facility: &FacilityState,
    actors: &ActorRegistry,
    visibility: &VisibilityState,
) -> bool {
    let worker_visible = |worker| {
        actors
            .get(worker)
            .is_some_and(|actor| visibility.is_visible(actor.position()))
    };
    let order_visible = |order: &ContentId| {
        facility
            .repair_target(order)
            .is_some_and(|installation| visibility.is_visible(installation.position()))
    };
    match event {
        FacilityEvent::WorkerMoved { from, to, .. } => {
            visibility.is_visible(*from) || visibility.is_visible(*to)
        }
        FacilityEvent::DoorOpened { worker, at } => {
            visibility.is_visible(*at) || worker_visible(*worker)
        }
        FacilityEvent::PlayerMaterialDeposited { player, at, .. } => {
            visibility.is_visible(*at) || worker_visible(*player)
        }
        FacilityEvent::MaterialCollected { worker, .. }
        | FacilityEvent::MaterialDelivered { worker, .. }
        | FacilityEvent::RepairAssigned { worker, .. }
        | FacilityEvent::RepairStarted { worker, .. } => worker_visible(*worker),
        FacilityEvent::InstallationRepaired { installation, .. } => facility
            .installation(installation)
            .is_some_and(|state| visibility.is_visible(state.position())),
        FacilityEvent::SecurityAlarmRaised { installation, .. } => facility
            .installation(installation)
            .is_some_and(|state| visibility.is_visible(state.position())),
        FacilityEvent::DoorLockdownStarted {
            installation, door, ..
        }
        | FacilityEvent::DoorLockdownPrevented {
            installation, door, ..
        }
        | FacilityEvent::DoorLockdownEnded {
            installation, door, ..
        } => {
            visibility.is_visible(*door)
                || facility
                    .installation(installation)
                    .is_some_and(|state| visibility.is_visible(state.position()))
        }
        FacilityEvent::MaterialSpilled { at, .. } => visibility.is_visible(*at),
        FacilityEvent::WorkInterrupted { order } => order_visible(order),
        FacilityEvent::SimulationFault(_) => false,
    }
}

impl GameState {
    fn swap_zone(&mut self, zone: &mut ZoneState) {
        std::mem::swap(&mut self.map, &mut zone.map);
        std::mem::swap(&mut self.actors, &mut zone.actors);
        std::mem::swap(&mut self.exit, &mut zone.exit);
        std::mem::swap(&mut self.rng, &mut zone.rng);
        std::mem::swap(&mut self.player_visibility, &mut zone.visibility);
        std::mem::swap(&mut self.ground_items, &mut zone.ground);
        std::mem::swap(&mut self.movement_traces, &mut zone.traces);
        std::mem::swap(&mut self.ground_effects, &mut zone.ground_effects);
    }

    fn tick_background(&mut self, zone: &mut ZoneState, previous_turn: u64) {
        let visible_events = std::mem::take(&mut self.events);
        let current_turn = self.turn;
        self.swap_zone(zone);
        self.turn = previous_turn;
        let actors: Vec<_> = self
            .actors
            .iter()
            .filter_map(|(id, actor)| {
                actor
                    .ai()
                    .filter(|ai| matches!(ai.behavior, AiBehavior::Hunter | AiBehavior::Skirmisher))
                    .map(|_| (id, actor.position()))
            })
            .collect();
        // No player position or remote target is available. Mobile profiles
        // patrol locally; sentries/idle actors stay put. Nobody crosses portals.
        for (id, origin) in actors {
            let choices: Vec<_> = [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .into_iter()
            .filter(|d| {
                let p = origin.step(*d);
                self.map.is_walkable(p)
                    && !self.map.is_protected(p)
                    && self.actors.entity_at(p).is_none()
            })
            .collect();
            if let Some(index) = self.rng.usize_inclusive(0, choices.len())
                && let Some(direction) = choices.get(index)
            {
                let _ = self.move_entity(id, *direction);
            }
        }
        self.resolve_status_trigger(StatusTrigger::TurnEnd);
        self.resolve_ground_effects();
        self.elapse_status_durations();
        self.turn = current_turn;
        self.movement_traces
            .prune(current_turn, self.rules.movement_traces);
        // Off-screen movement, death, positions and status events are not player
        // knowledge. Legitimate player XP from an ongoing effect is still global.
        let rewards: Vec<_> = self
            .events
            .drain(..)
            .filter(|event| {
                matches!(
                    event,
                    GameEvent::ExperienceAwarded { .. } | GameEvent::LevelGained { .. }
                )
            })
            .collect();
        self.swap_zone(zone);
        self.events = visible_events;
        self.events.extend(rewards);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::AiProfile;
    use crate::combat::{DamagePacket, DamageType};
    use crate::effects::GroundEffectSpec;
    use crate::facility::{
        FacilityBlueprint, InstallationBlueprint, InstallationCapability, RepairOrderBlueprint,
        RepairStatus, SecurityAlarmProfile, SecurityAlarmResponse, WorkerBlueprint, WorkerRole,
    };
    use crate::game::GameRules;
    use crate::item::{ItemDefinition, ItemEffect, ItemKind};
    use crate::status::{StatusDefinition, StatusEffectPrimitive, StatusHook, StatusStacking};
    use crate::world::{DoorState, Terrain};

    fn id(name: &str) -> ContentId {
        format!("test:{name}").parse().unwrap()
    }
    fn info(name: &str, depth: u16) -> ZoneInfo {
        ZoneInfo {
            id: id(name),
            name: name.into(),
            kind: id("industrial"),
            depth,
        }
    }
    fn map() -> Map {
        Map::from_ascii(
            "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
        )
        .unwrap()
    }
    fn world() -> WorldState {
        let mut rules = GameRules::default();
        rules
            .items
            .register(
                ItemDefinition::new(
                    id("repair"),
                    "repair".into(),
                    "repair".into(),
                    9,
                    ItemKind::Consumable,
                    vec![ItemEffect::RestoreIntegrity { amount: 1 }],
                )
                .unwrap(),
            )
            .unwrap();
        let mut game = GameState::new_with_rules(map(), GridPos::new(1, 1), 42, rules).unwrap();
        game.map
            .set_terrain(GridPos::new(1, 2), Terrain::Door(DoorState::Closed))
            .unwrap();
        game.spawn_actor(
            Actor::new(GridPos::new(7, 5), 10)
                .unwrap()
                .with_ai(AiProfile::hunter(0, 0)),
        )
        .unwrap();
        let mut world = WorldState::single(game);
        world.enable(info("a", 0)).unwrap();
        world
            .add_zone(ZoneBlueprint {
                info: info("b", 1),
                map: map(),
                entrance: GridPos::new(1, 1),
                seed: 888,
                actors: vec![
                    Actor::new(GridPos::new(7, 5), 10)
                        .unwrap()
                        .with_ai(AiProfile::idle()),
                ],
                loot: vec![(GridPos::new(2, 1), id("repair"), 1)],
            })
            .unwrap();
        world
            .connect(id("a"), GridPos::new(2, 1), id("b"), GridPos::new(1, 1))
            .unwrap();
        world.drain_events();
        world
    }
    fn apply(world: &mut WorldState, command: GameCommand) {
        assert_eq!(
            world.process_player_command(command),
            CommandOutcome::Applied
        );
        world.drain_events();
    }
    fn outward(world: &mut WorldState) {
        apply(
            world,
            GameCommand::Interact {
                target: GridPos::new(2, 1),
            },
        );
    }
    fn home(world: &mut WorldState) {
        apply(
            world,
            GameCommand::Interact {
                target: GridPos::new(1, 1),
            },
        );
    }

    #[test]
    fn zones_keep_doors_loot_player_and_global_ids_on_round_trips() {
        let mut world = world();
        let player = world.player_id();
        let initial_enemy = world.actors().iter().map(|(id, _)| id).max().unwrap();
        apply(
            &mut world,
            GameCommand::Interact {
                target: GridPos::new(1, 2),
            },
        );
        let explored: Vec<_> = world.player_visibility().explored_positions().collect();
        outward(&mut world);
        assert_eq!(world.current_zone().unwrap().depth, 1);
        assert_eq!(world.player_id(), player);
        assert!(
            world
                .actors()
                .iter()
                .all(|(id, _)| id == player || id > initial_enemy)
        );
        apply(&mut world, GameCommand::Move(Direction::East));
        apply(&mut world, GameCommand::PickUp);
        let item = world.player_inventory().iter().next().unwrap().instance();
        home(&mut world);
        assert_eq!(world.current_zone().unwrap().id, id("a"));
        assert_eq!(
            world.map().tile(GridPos::new(1, 2)).unwrap().terrain,
            Terrain::Door(DoorState::Open)
        );
        assert!(
            explored
                .iter()
                .all(|p| world.player_visibility().is_explored(*p))
        );
        outward(&mut world);
        assert!(world.ground_items().item_at(GridPos::new(2, 1)).is_none());
        assert!(world.player_inventory().get(item).is_some());
        assert_eq!(world.visited_zone_count(), 2);
        assert_eq!(world.turn(), 6);
    }

    #[test]
    fn offscreen_ai_statuses_and_deaths_evolve_without_visual_information() {
        let mut world = world();
        let status = StatusDefinition::new(
            id("decay"),
            Some(3),
            StatusStacking::Replace,
            vec![StatusHook::new(
                StatusTrigger::TurnEnd,
                vec![StatusEffectPrimitive::DealDamage {
                    packet: DamagePacket::new(4, DamageType::Chemical, 0),
                    multiply_by_stacks: false,
                }],
            )],
        )
        .unwrap();
        world
            .active
            .rules
            .statuses
            .register(status.clone())
            .unwrap();
        let enemy = world
            .actors()
            .iter()
            .find(|(id, _)| *id != world.player_id())
            .unwrap()
            .0;
        world
            .active
            .actors
            .get_mut(enemy)
            .unwrap()
            .apply_status(&status, 1, None);
        outward(&mut world); // A first advances without the player.
        assert_eq!(
            world.inactive[&id("a")]
                .actors
                .get(enemy)
                .unwrap()
                .integrity(),
            6
        );
        assert!(
            !world.inactive[&id("a")]
                .visibility
                .visible_positions()
                .any(|_| true)
        );
        let first_rng = world.inactive[&id("a")].rng.state();
        apply(&mut world, GameCommand::Wait);
        assert_eq!(
            world.inactive[&id("a")]
                .actors
                .get(enemy)
                .unwrap()
                .integrity(),
            2
        );
        assert_ne!(world.inactive[&id("a")].rng.state(), first_rng);
        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert!(world.inactive[&id("a")].actors.get(enemy).is_none());
        assert!(!world.events().iter().any(|event| matches!(event, GameEvent::EntityDied { entity } | GameEvent::EntityMoved { entity, .. } if *entity == enemy)));
        world.drain_events();
        home(&mut world);
        assert!(world.actors().get(enemy).is_none());
        outward(&mut world);
        home(&mut world);
        assert!(world.actors().get(enemy).is_none());
    }

    #[test]
    fn ground_effects_remain_with_their_zone_and_expire_offscreen() {
        let mut world = world();
        let position = GridPos::new(3, 3);
        let fire = GroundEffectSpec::new(
            id("ground_fire"),
            2,
            DamagePacket::new(1, DamageType::Thermal, 0),
        )
        .unwrap();
        let current_turn = world.turn();
        world
            .active
            .ground_effects
            .apply(position, None, &fire, current_turn);

        outward(&mut world);
        assert_eq!(
            world.inactive[&id("a")]
                .ground_effects
                .at(position)
                .next()
                .map(|effect| effect.remaining_turns()),
            Some(2)
        );
        apply(&mut world, GameCommand::Wait);
        assert_eq!(
            world.inactive[&id("a")]
                .ground_effects
                .at(position)
                .next()
                .map(|effect| effect.remaining_turns()),
            Some(1)
        );
        apply(&mut world, GameCommand::Wait);
        assert!(world.inactive[&id("a")].ground_effects.is_empty());
    }

    #[test]
    fn facility_repairs_continue_offscreen_without_leaking_events() {
        let mut world = world();
        world
            .active
            .map
            .set_terrain(GridPos::new(4, 3), Terrain::Door(DoorState::Closed))
            .unwrap();
        for position in [GridPos::new(1, 5), GridPos::new(2, 5)] {
            world
                .spawn_actor(Actor::new(position, 10).unwrap().with_ai(AiProfile::idle()))
                .unwrap();
        }
        world
            .spawn_ground_item(GridPos::new(3, 5), id("repair"), 1)
            .unwrap();
        world
            .register_facility(
                id("a"),
                FacilityBlueprint {
                    installations: vec![
                        InstallationBlueprint {
                            id: id("relay"),
                            position: GridPos::new(6, 5),
                            maximum_integrity: 10,
                            integrity: 0,
                            capabilities: vec![InstallationCapability::PowerRelay],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                        InstallationBlueprint {
                            id: id("actuator"),
                            position: GridPos::new(7, 3),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::DoorActuator {
                                door: GridPos::new(4, 3),
                            }],
                            dependencies: vec![id("relay")],
                            security_alarm_profile: None,
                        },
                        InstallationBlueprint {
                            id: id("depot"),
                            position: GridPos::new(6, 1),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::Storage],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                    ],
                    depot: id("depot"),
                    workers: vec![
                        WorkerBlueprint {
                            actor_position: GridPos::new(1, 5),
                            role: WorkerRole::Retriever,
                            maximum_integrity: 10,
                            affiliation: None,
                            witness_profile: None,
                            local_alert_profile: None,
                        },
                        WorkerBlueprint {
                            actor_position: GridPos::new(2, 5),
                            role: WorkerRole::Technician,
                            maximum_integrity: 10,
                            affiliation: None,
                            witness_profile: None,
                            local_alert_profile: None,
                        },
                    ],
                    repair_orders: vec![RepairOrderBlueprint {
                        id: id("restore"),
                        target: id("relay"),
                        required_item: id("repair"),
                        required_quantity: 1,
                        work_turns: 2,
                    }],
                    maximum_path_search: 256,
                    owner: None,
                },
            )
            .unwrap();
        world.drain_events();

        outward(&mut world);
        for _ in 0..24 {
            assert_eq!(
                world.process_player_command(GameCommand::Wait),
                CommandOutcome::Applied
            );
            assert!(
                !world
                    .events()
                    .iter()
                    .any(|event| matches!(event, GameEvent::Facility(_)))
            );
            world.drain_events();
        }

        let facility = world.facility_in_zone(&id("a")).unwrap();
        assert_eq!(
            facility.repair_status(&id("restore")),
            Some(RepairStatus::Completed)
        );
        assert!(facility.is_operational(&id("actuator")));
        assert_eq!(
            world.inactive[&id("a")]
                .map
                .tile(GridPos::new(4, 3))
                .unwrap()
                .terrain,
            Terrain::Door(DoorState::Closed)
        );
    }

    #[test]
    fn alarm_lockdown_expires_offscreen_without_leaking_events() {
        let mut world = world();
        let door = GridPos::new(4, 3);
        world
            .active
            .map
            .set_terrain(door, Terrain::Door(DoorState::Closed))
            .unwrap();
        world
            .spawn_ground_item_with_owner(
                GridPos::new(1, 1),
                id("repair"),
                1,
                Some(id("collective")),
            )
            .unwrap();
        world
            .register_facility(
                id("a"),
                FacilityBlueprint {
                    installations: vec![
                        InstallationBlueprint {
                            id: id("actuator"),
                            position: GridPos::new(6, 4),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::DoorActuator { door }],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                        InstallationBlueprint {
                            id: id("sensor"),
                            position: GridPos::new(2, 2),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::SecuritySensor],
                            dependencies: vec![],
                            security_alarm_profile: Some(
                                SecurityAlarmProfile::new(
                                    8,
                                    crate::world::DistanceMetric::Euclidean,
                                    true,
                                    4,
                                )
                                .unwrap()
                                .with_responses(vec![SecurityAlarmResponse::LockDoors {
                                    actuator: id("actuator"),
                                }])
                                .unwrap(),
                            ),
                        },
                        InstallationBlueprint {
                            id: id("depot"),
                            position: GridPos::new(7, 4),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::Storage],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                    ],
                    depot: id("depot"),
                    workers: vec![],
                    repair_orders: vec![],
                    maximum_path_search: 256,
                    owner: Some(id("collective")),
                },
            )
            .unwrap();
        world.drain_events();

        assert_eq!(
            world.process_player_command(GameCommand::PickUp),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.map().tile(door).unwrap().terrain,
            Terrain::Door(DoorState::Locked)
        );
        world.drain_events();
        outward(&mut world);
        for _ in 0..3 {
            assert_eq!(
                world.process_player_command(GameCommand::Wait),
                CommandOutcome::Applied
            );
            assert!(
                !world
                    .events()
                    .iter()
                    .any(|event| matches!(event, GameEvent::Facility(_)))
            );
            world.drain_events();
        }
        assert_eq!(
            world.inactive[&id("a")].map.tile(door).unwrap().terrain,
            Terrain::Door(DoorState::Closed)
        );
        assert!(
            world
                .facility_in_zone(&id("a"))
                .unwrap()
                .security_door_lockdown_at(door, world.turn())
                .is_none()
        );
    }

    #[test]
    fn blocked_and_remote_transitions_are_atomic_and_background_does_not_tick() {
        let mut world = world();
        outward(&mut world);
        world
            .inactive
            .get_mut(&id("a"))
            .unwrap()
            .map
            .set_terrain(GridPos::new(2, 1), Terrain::Wall)
            .unwrap();
        let before = format!("{world:?}");
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(1, 1)
            }),
            CommandOutcome::Rejected(CommandRejection::PassageUnavailable)
        );
        assert_eq!(format!("{world:?}"), before);
        apply(&mut world, GameCommand::Move(Direction::East));
        apply(&mut world, GameCommand::Move(Direction::East));
        let before = format!("{world:?}");
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(1, 1)
            }),
            CommandOutcome::Rejected(CommandRejection::InteractionOutOfReach)
        );
        assert_eq!(format!("{world:?}"), before);
    }

    #[test]
    fn all_world_updates_are_deterministic_and_unvisited_zones_do_not_spawn() {
        let mut a = world();
        let mut b = world();
        for command in [
            GameCommand::Wait,
            GameCommand::Interact {
                target: GridPos::new(2, 1),
            },
            GameCommand::Wait,
            GameCommand::Wait,
            GameCommand::Interact {
                target: GridPos::new(1, 1),
            },
            GameCommand::Wait,
        ] {
            assert_eq!(
                a.process_player_command(command.clone()),
                b.process_player_command(command)
            );
            assert_eq!(format!("{a:?}"), format!("{b:?}"));
            a.drain_events();
            b.drain_events();
        }
        assert_eq!(world().visited_zone_count(), 1);
    }

    #[test]
    fn invalid_connections_and_duplicate_zone_ids_do_not_replace_content() {
        let mut world = world();
        let before = format!("{world:?}");
        assert!(
            world
                .connect(id("a"), GridPos::new(0, 0), id("b"), GridPos::new(1, 2))
                .is_err()
        );
        assert!(
            world
                .connect(id("a"), GridPos::new(2, 1), id("b"), GridPos::new(1, 1))
                .is_err()
        );
        let duplicate = world.pending[&id("b")].clone();
        assert!(world.add_zone(duplicate).is_err());
        assert_eq!(format!("{world:?}"), before);
    }

    #[test]
    fn occupied_arrival_is_rejected_without_a_turn_or_mutation() {
        let mut world = world();
        outward(&mut world);
        let zone = world.inactive.get_mut(&id("a")).unwrap();
        let enemy = zone.actors.iter().next().unwrap().0;
        zone.actors.move_to(enemy, GridPos::new(2, 1)).unwrap();
        let before = format!("{world:?}");
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(1, 1)
            }),
            CommandOutcome::Rejected(CommandRejection::PassageObstructed)
        );
        assert_eq!(format!("{world:?}"), before);
    }

    #[test]
    fn rejected_first_visit_keeps_its_pending_facility_uninstantiated() {
        let mut world = world();
        world
            .register_facility(
                id("b"),
                FacilityBlueprint {
                    installations: vec![InstallationBlueprint {
                        id: id("remote_depot"),
                        position: GridPos::new(3, 3),
                        maximum_integrity: 10,
                        integrity: 10,
                        capabilities: vec![InstallationCapability::Storage],
                        dependencies: vec![],
                        security_alarm_profile: None,
                    }],
                    depot: id("remote_depot"),
                    workers: vec![],
                    repair_orders: vec![],
                    maximum_path_search: 64,
                    owner: None,
                },
            )
            .unwrap();
        world.pending.get_mut(&id("b")).unwrap().actors.push(
            Actor::new(GridPos::new(1, 1), 5)
                .unwrap()
                .with_ai(AiProfile::idle()),
        );
        let before_turn = world.turn();

        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(2, 1),
            }),
            CommandOutcome::Rejected(CommandRejection::PassageObstructed)
        );

        assert_eq!(world.turn(), before_turn);
        assert!(world.pending.contains_key(&id("b")));
        assert!(world.pending_facilities.contains_key(&id("b")));
        assert!(!world.facilities.contains_key(&id("b")));
        assert!(!world.inactive.contains_key(&id("b")));
    }

    #[test]
    fn player_status_ticks_once_and_offscreen_player_kills_award_xp_once() {
        let mut world = world();
        let status = StatusDefinition::new(
            id("decay"),
            Some(3),
            StatusStacking::Replace,
            vec![StatusHook::new(
                StatusTrigger::TurnEnd,
                vec![StatusEffectPrimitive::DealDamage {
                    packet: DamagePacket::new(4, DamageType::Chemical, 0),
                    multiply_by_stacks: false,
                }],
            )],
        )
        .unwrap();
        world
            .active
            .rules
            .statuses
            .register(status.clone())
            .unwrap();
        let player = world.player_id();
        let enemy = world
            .actors()
            .iter()
            .find(|(id, _)| *id != player)
            .unwrap()
            .0;
        let mut actor = world
            .active
            .actors
            .remove(enemy)
            .unwrap()
            .with_defeat_reward(crate::progression::DefeatReward::persistent(12, 1));
        actor.apply_status(&status, 1, Some(player));
        world.active.actors.insert_existing(enemy, actor);
        let actor = world.active.actors.get_mut(player).unwrap();
        let integrity = actor.integrity();
        actor.apply_status(&status, 1, None);
        outward(&mut world);
        assert_eq!(
            world.actors().get(player).unwrap().integrity(),
            integrity - 4
        );
        apply(&mut world, GameCommand::Wait);
        assert_eq!(
            world.actors().get(player).unwrap().integrity(),
            integrity - 8
        );
        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.actors().get(player).unwrap().integrity(),
            integrity - 12
        );
        assert_eq!(
            world
                .events()
                .iter()
                .filter(|e| matches!(e, GameEvent::ExperienceAwarded { .. }))
                .count(),
            1
        );
        world.drain_events();
        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert!(
            !world
                .events()
                .iter()
                .any(|e| matches!(e, GameEvent::ExperienceAwarded { .. }))
        );
    }
}
