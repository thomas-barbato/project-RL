//! Persistent, deterministic zones. Presentation never advances this simulation.
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};

use crate::ai::{AiAction, AiBehavior, AiSituation, AiState, decide_known_action};
use crate::content::ContentId;
use crate::drone::DroneOrder;
use crate::effects::GroundEffectMap;
use crate::electronic_warfare::ElectronicWarfareState;
use crate::entity::{Actor, ActorRegistry, GroundItemRegistry};
use crate::explosive::ExplosiveDeviceMap;
use crate::facility::{
    FacilityBlueprint, FacilityEvent, FacilityState, ReinforcementRequestFailure,
    SecurityAlarmResponse, SecurityAlarmResponseContext, WorkerRole,
};
use crate::intrusion::IntrusionState;
use crate::item::ItemId;
use crate::social::{ObservedPropertyTake, SocialGroupId};
use crate::status::StatusTrigger;
use crate::world::generation::{MapValidationRules, validate_interactive_map};
use crate::world::{Direction, GridPos, Map, MovementTraceMap, VisibilityState, find_path};

use super::{
    CommandOutcome, CommandRejection, GameCommand, GameEvent, GameRng, GameState, RunStatus,
    ThreatReinforcementRequestError, ThreatSourceBlueprint, ThreatSourceState, TurnPhase,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZoneInfo {
    pub id: ContentId,
    pub name: String,
    pub kind: ContentId,
    pub depth: u16,
}

#[derive(Clone, PartialEq, Eq)]
pub struct GroundLootBlueprint {
    position: GridPos,
    item: ItemId,
    quantity: u16,
    owner: Option<SocialGroupId>,
}

impl GroundLootBlueprint {
    pub fn new(position: GridPos, item: ItemId, quantity: u16) -> Self {
        Self {
            position,
            item,
            quantity,
            owner: None,
        }
    }

    pub fn with_owner(mut self, owner: SocialGroupId) -> Self {
        self.owner = Some(owner);
        self
    }

    pub const fn position(&self) -> GridPos {
        self.position
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
}

impl From<(GridPos, ItemId, u16)> for GroundLootBlueprint {
    fn from((position, item, quantity): (GridPos, ItemId, u16)) -> Self {
        Self::new(position, item, quantity)
    }
}

impl Debug for GroundLootBlueprint {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        if self.owner.is_none() {
            return (&self.position, &self.item, self.quantity).fmt(formatter);
        }
        formatter
            .debug_struct("GroundLootBlueprint")
            .field("position", &self.position)
            .field("item", &self.item)
            .field("quantity", &self.quantity)
            .field("owner", &self.owner)
            .finish()
    }
}

/// Input from any authored/procedural content provider. Spawns are instantiated
/// once, on first entry, with global IDs and their own deterministic RNG stream.
#[derive(Clone)]
pub struct ZoneBlueprint {
    pub info: ZoneInfo,
    pub map: Map,
    pub entrance: GridPos,
    pub seed: u64,
    pub actors: Vec<Actor>,
    pub loot: Vec<GroundLootBlueprint>,
    pub threat_sources: Vec<ThreatSourceBlueprint>,
}

impl Debug for ZoneBlueprint {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut blueprint = formatter.debug_struct("ZoneBlueprint");
        blueprint
            .field("info", &self.info)
            .field("map", &self.map)
            .field("entrance", &self.entrance)
            .field("seed", &self.seed)
            .field("actors", &self.actors)
            .field("loot", &self.loot);
        if !self.threat_sources.is_empty() {
            blueprint.field("threat_sources", &self.threat_sources);
        }
        blueprint.finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ZoneConnectionBlueprint {
    pub at: GridPos,
    pub destination: ZoneInfo,
    pub arrival: GridPos,
}

#[derive(Clone, PartialEq, Eq)]
pub struct ZoneLink {
    pub destination: ContentId,
    pub arrival: Option<GridPos>,
}

// Keep resolved links byte-for-byte identical to their historical Debug form;
// only genuinely deferred links expose `arrival: None` in v14 state.
impl Debug for ZoneLink {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut link = formatter.debug_struct("ZoneLink");
        link.field("destination", &self.destination);
        match self.arrival {
            Some(arrival) => link.field("arrival", &arrival),
            None => link.field("arrival", &Option::<GridPos>::None),
        };
        link.finish()
    }
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
    explosive_devices: ExplosiveDeviceMap,
    intrusion: IntrusionState,
    electronic_warfare: ElectronicWarfareState,
    threat_sources: Vec<ThreatSourceState>,
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
        if !self.explosive_devices.is_empty() {
            state.field("explosive_devices", &self.explosive_devices);
        }
        if !self.intrusion.is_empty() {
            state.field("intrusion", &self.intrusion);
        }
        if !self.electronic_warfare.is_empty() {
            state.field("electronic_warfare", &self.electronic_warfare);
        }
        if !self.threat_sources.is_empty() {
            state.field("threat_sources", &self.threat_sources);
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
    pub fn zone_info(&self, id: &ContentId) -> Option<&ZoneInfo> {
        self.information.get(id)
    }
    pub fn visited_zone_count(&self) -> usize {
        1 + self.inactive.len()
    }
    pub fn passage(&self, at: GridPos) -> Option<&ZoneLink> {
        self.current
            .as_ref()
            .and_then(|id| self.links.get(&(id.clone(), at)))
    }

    pub fn passage_in(&self, zone: &ContentId, at: GridPos) -> Option<&ZoneLink> {
        self.links.get(&(zone.clone(), at))
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

    /// Records discovered anywhere in the instantiated world, including
    /// facilities that currently evolve off screen. Duplicate records are one
    /// discovery, even when several terminals contain the same text.
    pub fn discovered_data_terminal_records(&self) -> BTreeSet<ContentId> {
        self.facilities
            .values()
            .flat_map(FacilityState::accessed_data_terminal_records)
            .cloned()
            .collect()
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
        let threat_sources: Vec<_> = if self.current.as_ref() == Some(&zone) {
            self.active
                .threat_sources
                .iter()
                .map(|source| {
                    (
                        source.position(),
                        threat_actor_can_investigate(&source.actor),
                    )
                })
                .collect()
        } else if let Some(state) = self.inactive.get(&zone) {
            state
                .threat_sources
                .iter()
                .map(|source| {
                    (
                        source.position(),
                        threat_actor_can_investigate(&source.actor),
                    )
                })
                .collect()
        } else if let Some(pending) = self.pending.get(&zone) {
            pending
                .threat_sources
                .iter()
                .map(|source| (source.position, threat_actor_can_investigate(&source.actor)))
                .collect()
        } else {
            return Err("Unknown zone for facility simulation".into());
        };
        validate_facility_reinforcement_sources(&blueprint, &threat_sources)?;
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
        unreachable!("zone existence was checked before facility validation")
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

    fn map_for_anchor(&self, id: &ContentId, anchor: GridPos) -> Option<(&Map, GridPos)> {
        self.map_for(id)
            .or_else(|| self.inactive.get(id).map(|state| (&state.map, anchor)))
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
                arrival: Some(arrival),
            },
        );
        self.links.insert(
            (to, arrival),
            ZoneLink {
                destination: from,
                arrival: Some(at),
            },
        );
        Ok(())
    }

    /// Declares only the known end of a connection. The destination metadata
    /// is cheap, while its map remains absent until a content provider resolves
    /// the passage immediately before first travel.
    pub fn declare_deferred_connection(
        &mut self,
        from: ContentId,
        at: GridPos,
        destination: ZoneInfo,
    ) -> Result<(), String> {
        if from == destination.id
            || self.current.is_none()
            || self.information.contains_key(&destination.id)
            || self.links.contains_key(&(from.clone(), at))
        {
            return Err("Duplicate passage, zone ID or self connection".into());
        }
        let (map, start) = self
            .map_for(&from)
            .ok_or("Unknown/unavailable source zone")?;
        if !map.is_walkable(at) {
            return Err("Blocked passage".into());
        }
        validate_interactive_map(map, start, at, &[], MapValidationRules::default())
            .map_err(|error| error.to_string())?;
        if self
            .pending
            .get(&from)
            .is_some_and(|blueprint| blueprint.actors.iter().any(|actor| actor.position() == at))
        {
            return Err("Actor spawned on a passage".into());
        }
        self.links.insert(
            (from, at),
            ZoneLink {
                destination: destination.id.clone(),
                arrival: None,
            },
        );
        self.information.insert(destination.id.clone(), destination);
        Ok(())
    }

    /// Declares a reciprocal connection whose stable arrival tile is already
    /// known even though one or both local maps may not have been generated.
    /// Repeating the exact declaration is harmless, which lets independently
    /// discovered atlas paths close a cycle without replacing either link.
    pub fn declare_deferred_connection_at(
        &mut self,
        from: ContentId,
        at: GridPos,
        destination: ZoneInfo,
        arrival: GridPos,
    ) -> Result<(), String> {
        self.declare_deferred_connections_at(
            from,
            &[ZoneConnectionBlueprint {
                at,
                destination,
                arrival,
            }],
        )
    }

    /// Atomically registers several known-arrival passages leaving one zone.
    /// The source map is validated once with every requested anchor.
    pub fn declare_deferred_connections_at(
        &mut self,
        from: ContentId,
        connections: &[ZoneConnectionBlueprint],
    ) -> Result<(), String> {
        if connections.is_empty() {
            return Ok(());
        }
        if self.current.is_none() {
            return Err("World not initialized".into());
        }
        let source_anchors = connections
            .iter()
            .map(|connection| connection.at)
            .collect::<Vec<_>>();
        let (source_map, source_start) = self
            .map_for_anchor(&from, source_anchors[0])
            .ok_or("Unknown/unavailable source zone")?;
        validate_interactive_map(
            source_map,
            source_start,
            source_anchors[0],
            &source_anchors[1..],
            MapValidationRules::default(),
        )
        .map_err(|error| error.to_string())?;
        if self.pending.get(&from).is_some_and(|blueprint| {
            connections.iter().any(|connection| {
                blueprint
                    .actors
                    .iter()
                    .any(|actor| actor.position() == connection.at)
            })
        }) {
            return Err("Actor spawned on a passage".into());
        }

        let mut next_information = self.information.clone();
        let mut next_links = self.links.clone();
        for connection in connections {
            if from == connection.destination.id {
                return Err("Self connection".into());
            }
            if next_information
                .get(&connection.destination.id)
                .is_some_and(|known| known != &connection.destination)
            {
                return Err("Conflicting metadata for known destination".into());
            }
            let forward = ZoneLink {
                destination: connection.destination.id.clone(),
                arrival: Some(connection.arrival),
            };
            let reverse = ZoneLink {
                destination: from.clone(),
                arrival: Some(connection.at),
            };
            let forward_key = (from.clone(), connection.at);
            let reverse_key = (connection.destination.id.clone(), connection.arrival);
            match (next_links.get(&forward_key), next_links.get(&reverse_key)) {
                (Some(known_forward), Some(known_reverse))
                    if known_forward == &forward && known_reverse == &reverse =>
                {
                    continue;
                }
                (Some(_), _) | (_, Some(_)) => {
                    return Err("Conflicting passage declaration".into());
                }
                (None, None) => {}
            }
            if let Some((map, start)) =
                self.map_for_anchor(&connection.destination.id, connection.arrival)
            {
                if !map.is_walkable(connection.arrival) {
                    return Err("Blocked passage arrival".into());
                }
                validate_interactive_map(
                    map,
                    start,
                    connection.arrival,
                    &[],
                    MapValidationRules::default(),
                )
                .map_err(|error| error.to_string())?;
                if self
                    .pending
                    .get(&connection.destination.id)
                    .is_some_and(|blueprint| {
                        blueprint
                            .actors
                            .iter()
                            .any(|actor| actor.position() == connection.arrival)
                    })
                {
                    return Err("Actor spawned on a passage arrival".into());
                }
            }
            next_information
                .entry(connection.destination.id.clone())
                .or_insert_with(|| connection.destination.clone());
            next_links.insert(forward_key, forward);
            next_links.insert(reverse_key, reverse);
        }
        self.information = next_information;
        self.links = next_links;
        Ok(())
    }

    pub fn deferred_passage_destination(&self, at: GridPos) -> Option<&ContentId> {
        self.passage(at)
            .filter(|link| link.arrival.is_none())
            .map(|link| &link.destination)
    }

    pub fn unmaterialized_passage_destination(&self, at: GridPos) -> Option<&ContentId> {
        self.passage(at)
            .filter(|link| {
                self.current.as_ref() != Some(&link.destination)
                    && !self.pending.contains_key(&link.destination)
                    && !self.inactive.contains_key(&link.destination)
            })
            .map(|link| &link.destination)
    }

    /// A provider may perform expensive generation only after this preflight.
    /// The check is non-mutating and mirrors the travel interaction boundary.
    pub fn can_materialize_passage(&self, at: GridPos) -> bool {
        let Some(origin) = self.active.player_position() else {
            return false;
        };
        self.active.status == RunStatus::Active
            && self.active.phase == TurnPhase::AwaitingPlayer
            && (origin == at || origin.cardinal_neighbors().contains(&at))
            && self.active.player_visibility.is_visible(at)
            && self.active.map.is_walkable(at)
            && self.unmaterialized_passage_destination(at).is_some()
    }

    /// Installs a generated destination without advancing time. Validation is
    /// completed before any registry changes; the following ordinary Interact
    /// command performs the actual travel and remains the only recorded action.
    pub fn materialize_passage_destination(
        &mut self,
        at: GridPos,
        blueprint: ZoneBlueprint,
    ) -> Result<(), String> {
        self.materialize_passage_destination_with_connections(at, blueprint, &[])
    }

    /// Atomically installs a generated zone and every outward connection its
    /// provider derived from the same regional descriptor. A bad exit cannot
    /// leave behind a half-generated zone or a partially connected atlas.
    pub fn materialize_passage_destination_with_connections(
        &mut self,
        at: GridPos,
        blueprint: ZoneBlueprint,
        connections: &[ZoneConnectionBlueprint],
    ) -> Result<(), String> {
        self.materialize_passage_destination_with_connections_and_facility(
            at,
            blueprint,
            connections,
            None,
        )
    }

    /// Atomically validates and installs a generated zone, its atlas links and
    /// its optional local simulation. A malformed generated facility can never
    /// leave a half-materialized destination behind.
    pub fn materialize_passage_destination_with_connections_and_facility(
        &mut self,
        at: GridPos,
        blueprint: ZoneBlueprint,
        connections: &[ZoneConnectionBlueprint],
        facility: Option<FacilityBlueprint>,
    ) -> Result<(), String> {
        if !self.can_materialize_passage(at) {
            return Err("Deferred passage is not currently interactable".into());
        }
        let from = self.current.clone().ok_or("World is not initialized")?;
        let destination = self
            .unmaterialized_passage_destination(at)
            .cloned()
            .ok_or("Passage destination is already materialized")?;
        let declared_arrival = self
            .links
            .get(&(from.clone(), at))
            .and_then(|link| link.arrival);
        if blueprint.info.id != destination
            || self.information.get(&destination) != Some(&blueprint.info)
            || self.pending.contains_key(&destination)
            || self.inactive.contains_key(&destination)
            || declared_arrival.is_some_and(|arrival| arrival != blueprint.entrance)
        {
            return Err("Generated zone does not match its deferred destination".into());
        }
        let reciprocal_key = (destination.clone(), blueprint.entrance);
        if let Some(link) = self.links.get(&reciprocal_key)
            && (link.destination != from || link.arrival != Some(at))
        {
            return Err("Generated zone entrance already has a passage".into());
        }
        let mut probe = self.prepare_zone(&blueprint)?;
        if probe.actors.entity_at(blueprint.entrance).is_some() {
            return Err("Actor spawned on a passage".into());
        }
        if facility.is_some()
            && (self.facilities.contains_key(&destination)
                || self.pending_facilities.contains_key(&destination))
        {
            return Err("Generated zone already has a facility simulation".into());
        }
        if let Some(facility) = facility.as_ref() {
            validate_facility_reinforcement_sources(
                facility,
                &blueprint
                    .threat_sources
                    .iter()
                    .map(|source| (source.position, threat_actor_can_investigate(&source.actor)))
                    .collect::<Vec<_>>(),
            )?;
            FacilityState::instantiate(facility.clone(), &mut probe.map, &probe.actors)
                .map_err(|error| error.to_string())?;
        }
        let generated_passages = connections
            .iter()
            .map(|connection| connection.at)
            .collect::<Vec<_>>();
        let validation_exit = generated_passages
            .first()
            .copied()
            .unwrap_or(blueprint.entrance);
        validate_interactive_map(
            &blueprint.map,
            blueprint.entrance,
            validation_exit,
            generated_passages.get(1..).unwrap_or_default(),
            MapValidationRules::default(),
        )
        .map_err(|error| error.to_string())?;
        if connections.iter().any(|connection| {
            blueprint
                .actors
                .iter()
                .any(|actor| actor.position() == connection.at)
        }) {
            return Err("Actor spawned on a generated passage".into());
        }

        let mut next_information = self.information.clone();
        let mut next_links = self.links.clone();
        next_links
            .get_mut(&(from.clone(), at))
            .expect("checked deferred passage")
            .arrival = Some(blueprint.entrance);
        next_links.entry(reciprocal_key).or_insert(ZoneLink {
            destination: from,
            arrival: Some(at),
        });
        for connection in connections {
            if connection.destination.id == destination {
                return Err("Generated zone cannot connect to itself".into());
            }
            if !blueprint.map.is_walkable(connection.at) {
                return Err("Generated zone contains a blocked passage".into());
            }
            if next_information
                .get(&connection.destination.id)
                .is_some_and(|known| known != &connection.destination)
            {
                return Err("Conflicting metadata for generated destination".into());
            }
            if let Some((map, start)) =
                self.map_for_anchor(&connection.destination.id, connection.arrival)
            {
                if !map.is_walkable(connection.arrival) {
                    return Err("Generated connection has a blocked arrival".into());
                }
                validate_interactive_map(
                    map,
                    start,
                    connection.arrival,
                    &[],
                    MapValidationRules::default(),
                )
                .map_err(|error| error.to_string())?;
                if self
                    .pending
                    .get(&connection.destination.id)
                    .is_some_and(|target| {
                        target
                            .actors
                            .iter()
                            .any(|actor| actor.position() == connection.arrival)
                    })
                {
                    return Err("Actor spawned on a generated arrival".into());
                }
            }
            let forward_key = (destination.clone(), connection.at);
            let reverse_key = (connection.destination.id.clone(), connection.arrival);
            let forward = ZoneLink {
                destination: connection.destination.id.clone(),
                arrival: Some(connection.arrival),
            };
            let reverse = ZoneLink {
                destination: destination.clone(),
                arrival: Some(connection.at),
            };
            match (next_links.get(&forward_key), next_links.get(&reverse_key)) {
                (Some(known_forward), Some(known_reverse))
                    if known_forward == &forward && known_reverse == &reverse => {}
                (Some(_), _) | (_, Some(_)) => {
                    return Err("Conflicting generated passage declaration".into());
                }
                (None, None) => {
                    next_links.insert(forward_key, forward);
                    next_links.insert(reverse_key, reverse);
                }
            }
            next_information
                .entry(connection.destination.id.clone())
                .or_insert_with(|| connection.destination.clone());
        }

        self.information = next_information;
        self.links = next_links;
        if let Some(facility) = facility {
            self.pending_facilities
                .insert(destination.clone(), facility);
        }
        self.pending.insert(destination, blueprint);
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
        for loot in &blueprint.loot {
            probe
                .spawn_ground_item_with_owner(
                    loot.position(),
                    loot.item().clone(),
                    loot.quantity(),
                    loot.owner().cloned(),
                )
                .map_err(|e| e.to_string())?;
        }
        probe.install_threat_sources(blueprint.threat_sources.clone())?;
        Ok(ZoneState {
            map: probe.map,
            actors: probe.actors,
            exit: None,
            rng: probe.rng,
            visibility: VisibilityState::default(),
            ground: probe.ground_items,
            traces: MovementTraceMap::default(),
            ground_effects: GroundEffectMap::default(),
            explosive_devices: ExplosiveDeviceMap::default(),
            intrusion: IntrusionState::default(),
            electronic_warfare: ElectronicWarfareState::default(),
            threat_sources: probe.threat_sources,
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
                resolve_reinforcement_requests(&mut self.active, &mut events);
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
            for elapsed_turn in previous_turn..self.active.turn {
                if let Some(current) = self.current.clone()
                    && let Some(facility) = self.facilities.get_mut(&current)
                {
                    let result = (|| {
                        let mut events = facility.expire_security_alarm_responses(
                            &mut self.active.map,
                            elapsed_turn.saturating_add(1),
                        )?;
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
                    self.active.tick_background(zone, elapsed_turn);
                    if let Some(facility) = self.facilities.get_mut(id) {
                        let _ = facility.expire_security_alarm_responses(
                            &mut zone.map,
                            elapsed_turn.saturating_add(1),
                        );
                        let _ = facility.tick(&mut zone.map, &mut zone.actors, &mut zone.ground);
                    }
                }
            }
        }
        outcome
    }

    fn interact_with_facility(&mut self, target: GridPos) -> Option<CommandOutcome> {
        let zone = self.current.as_ref()?.clone();
        let facility = self.facilities.get(&zone)?;
        let data_terminal_record = facility.data_terminal_record_at(target).cloned();
        if !facility.is_player_interactive_at(target) {
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
        if let Some(record) = data_terminal_record {
            let first_discovery = !self.discovered_data_terminal_records().contains(&record);
            let facility = self
                .facilities
                .get_mut(&zone)
                .expect("facility checked before interaction");
            let Some(access) = facility.access_data_terminal(target) else {
                return reject(CommandRejection::FacilityUnavailable);
            };
            debug_assert_eq!(access.record, record);
            self.active
                .events
                .push(GameEvent::Facility(FacilityEvent::DataTerminalAccessed {
                    player,
                    installation: access.installation,
                    at: target,
                    record: access.record,
                    first_access: first_discovery,
                }));
            self.active.complete_turn();
            return Some(CommandOutcome::Applied);
        }
        let facility = self
            .facilities
            .get_mut(&zone)
            .expect("facility checked before interaction");
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
        let Some(arrival) = link.arrival else {
            return reject(CommandRejection::PassageUnavailable);
        };
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
            if zone.actors.entity_at(arrival).is_some() {
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
        if !zone.map.is_walkable(arrival) {
            return reject(CommandRejection::PassageUnavailable);
        }
        if zone.actors.entity_at(arrival).is_some() {
            return reject(CommandRejection::PassageObstructed);
        }
        let escorting_drones = self
            .active
            .actors
            .iter()
            .filter_map(|(entity, actor)| {
                let drone = actor.drone()?;
                let controller = match drone.order() {
                    DroneOrder::Escort { controller, .. } => controller,
                    DroneOrder::Companion { controller, .. }
                        if self.active.rules.player_drone_link_recovery =>
                    {
                        controller
                    }
                    _ => return None,
                };
                (*controller == self.active.player
                    && drone.controller() == self.active.player
                    && drone
                        .profile()
                        .link_reaches(&self.active.map, origin, actor.position(), 0))
                .then_some(entity)
            })
            .collect::<Vec<_>>();
        let mut occupied_arrivals = zone
            .actors
            .iter()
            .map(|(_, actor)| actor.position())
            .collect::<BTreeSet<_>>();
        occupied_arrivals.insert(arrival);
        let mut escort_arrivals = Vec::with_capacity(escorting_drones.len());
        for _ in &escorting_drones {
            let mut candidates = (0..zone.map.height())
                .flat_map(|y| (0..zone.map.width()).map(move |x| GridPos::new(x as i32, y as i32)))
                .filter(|position| {
                    zone.map.is_walkable(*position)
                        && !occupied_arrivals.contains(position)
                        && !zone.map.is_protected(*position)
                })
                .collect::<Vec<_>>();
            candidates.sort_by_key(|position| {
                (
                    position.x.abs_diff(arrival.x) + position.y.abs_diff(arrival.y),
                    position.y,
                    position.x,
                )
            });
            let Some(destination) = candidates.first().copied() else {
                return reject(CommandRejection::PassageObstructed);
            };
            occupied_arrivals.insert(destination);
            escort_arrivals.push(destination);
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
        let escorts = escorting_drones
            .iter()
            .map(|entity| {
                let actor = self
                    .active
                    .actors
                    .remove(*entity)
                    .expect("escorting drone was preflighted");
                (*entity, actor.position(), actor)
            })
            .collect::<Vec<_>>();
        next.actors.synchronize_ids(&self.active.actors);
        next.ground.synchronize_ids(&self.active.ground_items);
        self.active.swap_zone(&mut next);
        next.visibility.clear_visible();
        self.active
            .actors
            .insert_existing(self.active.player, player);
        self.active
            .actors
            .move_to(self.active.player, arrival)
            .expect("inserted player");
        for ((entity, origin, actor), destination) in escorts.into_iter().zip(escort_arrivals) {
            self.active.actors.insert_existing(entity, actor);
            self.active
                .actors
                .move_to(entity, destination)
                .expect("inserted escorting drone");
            if let Some(drone) = self
                .active
                .actors
                .get_mut(entity)
                .and_then(Actor::drone_mut)
            {
                drone.confirm_position(destination, self.active.turn);
            }
            self.active.events.push(GameEvent::EntityMoved {
                entity,
                from: origin,
                to: destination,
            });
        }
        self.inactive.insert(from.clone(), next);
        self.active.player_visibility.recompute(
            &self.active.map,
            arrival,
            self.active.rules.player_field_of_view,
        );
        self.active.events.push(GameEvent::ZoneChanged {
            from,
            to: link.destination,
            arrival,
        });
        self.active.events.push(GameEvent::VisibilityUpdated {
            observer: self.active.player,
            origin: arrival,
        });
        self.active.complete_turn();
        CommandOutcome::Applied
    }
}

fn validate_facility_reinforcement_sources(
    facility: &FacilityBlueprint,
    threat_sources: &[(GridPos, bool)],
) -> Result<(), String> {
    for response in facility
        .installations
        .iter()
        .filter_map(|installation| installation.security_alarm_profile.as_ref())
        .flat_map(|profile| profile.responses())
    {
        let (source, requires_investigation) = match response {
            SecurityAlarmResponse::CallReinforcements { source, .. } => (source, false),
            SecurityAlarmResponse::CallInvestigatingReinforcements { source, .. } => (source, true),
            SecurityAlarmResponse::LockDoors { .. } => continue,
        };
        let Some((_, supports_investigation)) = threat_sources
            .iter()
            .find(|(position, _)| position == source)
        else {
            return Err(format!(
                "Facility alarm references unknown threat source at {}, {}",
                source.x, source.y
            ));
        };
        if requires_investigation && !supports_investigation {
            return Err(format!(
                "Facility alarm investigation source at {}, {} requires mobile AI with a pursuit lifecycle",
                source.x, source.y
            ));
        }
    }
    Ok(())
}

fn threat_actor_can_investigate(actor: &Actor) -> bool {
    actor.ai().is_some_and(|profile| {
        matches!(
            profile.behavior,
            AiBehavior::Hunter | AiBehavior::Skirmisher
        ) && profile.pursuit_lifecycle().is_some()
    })
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
        FacilityEvent::DataTerminalAccessed { player, at, .. } => {
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
        FacilityEvent::ReinforcementsRequested {
            installation,
            source,
            ..
        }
        | FacilityEvent::InvestigatingReinforcementsRequested {
            installation,
            source,
            ..
        }
        | FacilityEvent::ReinforcementsUnavailable {
            installation,
            source,
            ..
        } => {
            visibility.is_visible(*source)
                || facility
                    .installation(installation)
                    .is_some_and(|state| visibility.is_visible(state.position()))
        }
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

fn resolve_reinforcement_requests(game: &mut GameState, events: &mut [FacilityEvent]) {
    for event in events {
        let (installation, source, result) = match event {
            FacilityEvent::ReinforcementsRequested {
                installation,
                source,
                delay_turns,
            } => (
                installation.clone(),
                *source,
                game.request_threat_reinforcement(*source, *delay_turns),
            ),
            FacilityEvent::InvestigatingReinforcementsRequested {
                installation,
                source,
                incident,
                delay_turns,
            } => (
                installation.clone(),
                *source,
                game.request_investigating_threat_reinforcement(*source, *incident, *delay_turns),
            ),
            _ => continue,
        };
        let failure = match result {
            Ok(()) => continue,
            Err(ThreatReinforcementRequestError::SourceInactive) => {
                Some(ReinforcementRequestFailure::SourceInactive)
            }
            Err(ThreatReinforcementRequestError::QuotaExhausted) => {
                Some(ReinforcementRequestFailure::QuotaExhausted)
            }
            Err(ThreatReinforcementRequestError::UnknownSource) => None,
        };
        *event = if let Some(reason) = failure {
            FacilityEvent::ReinforcementsUnavailable {
                installation,
                source,
                reason,
            }
        } else {
            FacilityEvent::SimulationFault(format!(
                "security alarm references unknown threat source at {}, {}",
                source.x, source.y
            ))
        };
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
        std::mem::swap(&mut self.explosive_devices, &mut zone.explosive_devices);
        std::mem::swap(&mut self.intrusion, &mut zone.intrusion);
        std::mem::swap(&mut self.electronic_warfare, &mut zone.electronic_warfare);
        std::mem::swap(&mut self.threat_sources, &mut zone.threat_sources);
    }

    fn tick_background(&mut self, zone: &mut ZoneState, previous_turn: u64) {
        let visible_events = std::mem::take(&mut self.events);
        let current_turn = self.turn;
        self.swap_zone(zone);
        self.turn = previous_turn;
        let actor_ids: Vec<_> = self.actors.iter().map(|(entity, _)| entity).collect();
        for entity in actor_ids {
            if let Some(actor) = self.actors.get_mut(entity) {
                actor.clear_elapsed_action_delay(self.turn);
            }
        }
        let actors: Vec<_> = self
            .actors
            .iter()
            .filter_map(|(id, actor)| {
                actor
                    .ai()
                    .filter(|ai| matches!(ai.behavior, AiBehavior::Hunter | AiBehavior::Skirmisher))
                    .map(|ai| (id, actor.position(), ai, actor.ai_home(), actor.ai_state()))
            })
            .collect();
        // No player position or remote target is available. Territorial actors
        // return home before resuming local patrols; sentries/idle actors stay
        // put. Nobody crosses portals.
        for (id, origin, profile, home, state) in actors {
            if self
                .actors
                .get(id)
                .is_some_and(|actor| actor.action_is_delayed(self.turn))
            {
                continue;
            }
            // Every off-screen actor still receives one normal opportunity.
            // Movement or local waiting is non-offensive and therefore spends
            // recovery, while the emitted transition remains hidden below.
            if self
                .actors
                .get(id)
                .is_some_and(|actor| actor.recovery_remaining().is_some())
            {
                self.advance_action_recovery(id);
            }
            if let Some(lifecycle) = profile.pursuit_lifecycle() {
                if let AiState::Responding {
                    remaining_turns,
                    incident,
                } = state
                {
                    let next_state = if remaining_turns <= 1 || origin == incident {
                        AiState::Returning
                    } else {
                        AiState::Responding {
                            remaining_turns: remaining_turns - 1,
                            incident,
                        }
                    };
                    if let Some(actor) = self.actors.get_mut(id) {
                        actor.set_ai_state(next_state);
                    }
                    if matches!(next_state, AiState::Returning) {
                        continue;
                    }
                    let occupied_positions = self
                        .actors
                        .iter()
                        .filter_map(|(other_id, actor)| {
                            (other_id != id).then_some(actor.position())
                        })
                        .collect();
                    if let AiAction::Move(direction) = decide_known_action(AiSituation {
                        map: &self.map,
                        actor_position: origin,
                        home_position: home,
                        target_position: incident,
                        occupied_positions: &occupied_positions,
                        profile,
                        preferred_attack: None,
                    }) {
                        let _ = self.move_ai_entity(id, direction);
                    }
                    continue;
                }
                let next_state = match state {
                    AiState::Pursuing { .. } | AiState::Searching { .. }
                        if home == Some(origin) =>
                    {
                        AiState::Cooldown {
                            remaining_turns: lifecycle.cooldown_turns(),
                        }
                    }
                    AiState::Pursuing { .. } | AiState::Searching { .. } => AiState::Returning,
                    AiState::Returning if home == Some(origin) => AiState::Cooldown {
                        remaining_turns: lifecycle.cooldown_turns(),
                    },
                    AiState::Cooldown { remaining_turns } if remaining_turns <= 1 => {
                        AiState::Unaware
                    }
                    AiState::Cooldown { remaining_turns } => AiState::Cooldown {
                        remaining_turns: remaining_turns - 1,
                    },
                    other => other,
                };
                if let Some(actor) = self.actors.get_mut(id) {
                    actor.set_ai_state(next_state);
                }
                if matches!(next_state, AiState::Cooldown { .. }) {
                    continue;
                }
            }
            if let Some(home) = home
                && origin != home
            {
                let path = find_path(
                    &self.map,
                    origin,
                    home,
                    profile.maximum_path_search,
                    |position| {
                        !self.map.is_protected(position)
                            && self.actors.entity_at(position).is_none()
                            && profile.maximum_pursuit_distance().is_none_or(|maximum| {
                                position.x.abs_diff(home.x).max(position.y.abs_diff(home.y))
                                    <= u32::from(maximum)
                            })
                    },
                );
                if let Some(next) = path.and_then(|path| path.get(1).copied())
                    && let Some(direction) =
                        Direction::from_delta(next.x - origin.x, next.y - origin.y)
                {
                    let _ = self.move_ai_entity(id, direction);
                }
                continue;
            }
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
                    && profile.maximum_pursuit_distance().is_none_or(|maximum| {
                        home.is_some_and(|home| {
                            p.x.abs_diff(home.x).max(p.y.abs_diff(home.y)) <= u32::from(maximum)
                        })
                    })
            })
            .collect();
            if let Some(index) = self.rng.usize_inclusive(0, choices.len())
                && let Some(direction) = choices.get(index)
            {
                let _ = self.move_ai_entity(id, *direction);
            }
        }
        self.resolve_status_trigger(StatusTrigger::TurnEnd);
        self.resolve_explosive_devices();
        self.resolve_ground_effects();
        self.elapse_status_durations();
        self.advance_technique_cooldowns();
        self.resolve_threat_sources();
        self.advance_intrusion_state();
        self.advance_electronic_warfare_state();
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
        // Off-screen reinforcements still consume IDs from the one global
        // namespace. Synchronize both registries before restoring the active
        // zone so a later spawn can never reuse an unseen entity ID.
        self.actors.synchronize_ids(&zone.actors);
        zone.actors.synchronize_ids(&self.actors);
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
    use crate::companion::CompanionBehavior;
    use crate::drone::{DroneCapabilities, DroneProfile, DroneState};
    use crate::effects::GroundEffectSpec;
    use crate::facility::{
        FacilityBlueprint, InstallationBlueprint, InstallationCapability, RepairOrderBlueprint,
        RepairStatus, SecurityAlarmProfile, SecurityAlarmResponse, WorkerBlueprint, WorkerRole,
    };
    use crate::game::GameRules;
    use crate::item::{ItemDefinition, ItemEffect, ItemKind};
    use crate::status::{
        StatusDefinition, StatusEffectPrimitive, StatusHook, StatusModifier, StatusStacking,
    };
    use crate::time::TimeUnits;
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
                    None,
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
                loot: vec![(GridPos::new(2, 1), id("repair"), 1).into()],
                threat_sources: vec![],
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
    fn offscreen_normal_opportunity_clears_recovery_without_leaking_its_event() {
        let mut world = world();
        outward(&mut world);
        home(&mut world);
        let inactive = world.inactive.get_mut(&id("b")).unwrap();
        let idle = inactive
            .actors
            .iter()
            .find_map(|(entity, actor)| (actor.ai().is_some()).then_some(entity))
            .unwrap();
        let position = inactive.actors.get(idle).unwrap().position();
        inactive.actors.remove(idle);
        let actor = inactive
            .actors
            .spawn(
                Actor::new(position, 10)
                    .unwrap()
                    .with_ai(AiProfile::hunter(0, 0)),
            )
            .unwrap();
        inactive
            .actors
            .get_mut(actor)
            .unwrap()
            .start_action_recovery(TimeUnits::ONE);

        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(
            world
                .inactive
                .get(&id("b"))
                .and_then(|zone| zone.actors.get(actor))
                .and_then(Actor::recovery_remaining),
            None
        );
        assert!(!world.events().iter().any(|event| matches!(
            event,
            GameEvent::ActionRecoveryAdvanced { entity, .. }
                | GameEvent::ActionRecoveryCompleted { entity }
                if *entity == actor
        )));
    }

    #[test]
    fn multi_ut_player_movement_advances_each_materialized_zone_once_per_unit() {
        let mut world = world();
        outward(&mut world);
        home(&mut world);
        let hindered = id("hindered");
        let definition = StatusDefinition::new(
            hindered.clone(),
            Some(2),
            StatusStacking::KeepExisting,
            Vec::new(),
        )
        .unwrap()
        .with_modifiers([StatusModifier::MovementTimeMinimum { time_units: 2 }])
        .unwrap();
        world
            .active
            .rules
            .statuses
            .register(definition.clone())
            .unwrap();
        world
            .active
            .actors
            .get_mut(world.active.player)
            .unwrap()
            .apply_status(&definition, 1, None);
        let remote = world.inactive[&id("b")]
            .actors
            .iter()
            .find_map(|(entity, actor)| (actor.ai().is_some()).then_some(entity))
            .unwrap();
        world
            .inactive
            .get_mut(&id("b"))
            .unwrap()
            .actors
            .get_mut(remote)
            .unwrap()
            .apply_status(&definition, 1, None);
        let before = world.turn();

        assert_eq!(
            world.process_player_command(GameCommand::Move(Direction::East)),
            CommandOutcome::Applied
        );
        assert_eq!(world.turn(), before + 2);
        assert!(
            world
                .actors()
                .get(world.player_id())
                .unwrap()
                .status(&hindered)
                .is_none()
        );
        assert!(
            world.inactive[&id("b")]
                .actors
                .get(remote)
                .unwrap()
                .status(&hindered)
                .is_none()
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
    fn following_companion_travels_with_the_player_between_zones() {
        let mut world = world();
        let profile = DroneProfile::new(
            id("travelling_companion"),
            6,
            50,
            40,
            1,
            3,
            4,
            1,
            1,
            DroneCapabilities::default(),
        )
        .unwrap();
        let player = world.player_id();
        let mut state =
            DroneState::new(profile, player, 10, 10, GridPos::new(1, 3), world.turn()).unwrap();
        state.replace_order(DroneOrder::Companion {
            controller: player,
            behavior: CompanionBehavior::Follow,
        });
        let drone = world
            .spawn_actor(
                Actor::new(GridPos::new(1, 3), 10)
                    .unwrap()
                    .with_drone(state),
            )
            .unwrap();
        assert!(matches!(
            world
                .actors()
                .get(drone)
                .and_then(Actor::drone)
                .map(|drone| drone.order()),
            Some(DroneOrder::Companion {
                behavior: CompanionBehavior::Follow,
                ..
            })
        ));

        outward(&mut world);

        assert_eq!(world.current_zone().unwrap().id, id("b"));
        assert!(world.actors().get(drone).is_some());
        assert!(world.inactive[&id("a")].actors.get(drone).is_none());
        assert!(world.player_companion_is_linked(drone));
    }

    #[test]
    fn deferred_zone_is_generated_only_at_an_interactable_first_passage() {
        let game = GameState::new(map(), GridPos::new(1, 1), 42).unwrap();
        let mut world = WorldState::single(game);
        world.enable(info("a", 0)).unwrap();
        let blueprint = ZoneBlueprint {
            info: info("b", 1),
            map: map(),
            entrance: GridPos::new(1, 1),
            seed: 888,
            actors: vec![],
            loot: vec![],
            threat_sources: vec![],
        };
        world
            .declare_deferred_connection(id("a"), GridPos::new(2, 1), blueprint.info.clone())
            .unwrap();

        assert!(world.pending.is_empty());
        assert!(world.inactive.is_empty());
        assert_eq!(world.visited_zone_count(), 1);
        assert_eq!(
            world.deferred_passage_destination(GridPos::new(2, 1)),
            Some(&id("b"))
        );
        assert!(world.can_materialize_passage(GridPos::new(2, 1)));

        world
            .materialize_passage_destination(GridPos::new(2, 1), blueprint)
            .unwrap();
        assert!(world.pending.contains_key(&id("b")));
        assert_eq!(world.visited_zone_count(), 1);
        assert!(
            world
                .passage(GridPos::new(2, 1))
                .is_some_and(|link| link.arrival == Some(GridPos::new(1, 1)))
        );

        apply(
            &mut world,
            GameCommand::Interact {
                target: GridPos::new(2, 1),
            },
        );
        assert_eq!(world.current_zone().unwrap().id, id("b"));
        assert_eq!(world.visited_zone_count(), 2);
        assert!(world.pending.is_empty());
        apply(
            &mut world,
            GameCommand::Interact {
                target: GridPos::new(1, 1),
            },
        );
        assert_eq!(world.current_zone().unwrap().id, id("a"));
    }

    #[test]
    fn known_atlas_arrival_can_materialize_lazily_and_be_declared_idempotently() {
        let game = GameState::new(map(), GridPos::new(1, 1), 42).unwrap();
        let mut world = WorldState::single(game);
        world.enable(info("a", 0)).unwrap();
        let destination = info("b", 0);
        for _ in 0..2 {
            world
                .declare_deferred_connection_at(
                    id("a"),
                    GridPos::new(2, 1),
                    destination.clone(),
                    GridPos::new(1, 1),
                )
                .unwrap();
        }

        assert!(
            world
                .deferred_passage_destination(GridPos::new(2, 1))
                .is_none()
        );
        assert_eq!(
            world.unmaterialized_passage_destination(GridPos::new(2, 1)),
            Some(&id("b"))
        );
        assert!(world.can_materialize_passage(GridPos::new(2, 1)));

        world
            .materialize_passage_destination(
                GridPos::new(2, 1),
                ZoneBlueprint {
                    info: destination,
                    map: map(),
                    entrance: GridPos::new(1, 1),
                    seed: 888,
                    actors: vec![],
                    loot: vec![],
                    threat_sources: vec![],
                },
            )
            .unwrap();
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(2, 1),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.current_zone().unwrap().id, id("b"));
        assert_eq!(world.player_position(), Some(GridPos::new(1, 1)));
    }

    #[test]
    fn rejected_deferred_materialization_is_atomic() {
        let game = GameState::new(map(), GridPos::new(1, 1), 42).unwrap();
        let mut world = WorldState::single(game);
        world.enable(info("a", 0)).unwrap();
        world
            .declare_deferred_connection(id("a"), GridPos::new(5, 1), info("b", 1))
            .unwrap();
        let before = format!("{world:?}");
        let blueprint = ZoneBlueprint {
            info: info("b", 1),
            map: map(),
            entrance: GridPos::new(1, 1),
            seed: 888,
            actors: vec![],
            loot: vec![],
            threat_sources: vec![],
        };

        assert!(!world.can_materialize_passage(GridPos::new(5, 1)));
        assert!(
            world
                .materialize_passage_destination(GridPos::new(5, 1), blueprint)
                .is_err()
        );
        assert_eq!(format!("{world:?}"), before);
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(5, 1),
            }),
            CommandOutcome::Rejected(CommandRejection::InteractionOutOfReach)
        );
        assert_eq!(format!("{world:?}"), before);
    }

    #[test]
    fn invalid_generated_outgoing_connection_rejects_the_whole_zone_atomically() {
        let game = GameState::new(map(), GridPos::new(1, 1), 42).unwrap();
        let mut world = WorldState::single(game);
        world.enable(info("a", 0)).unwrap();
        world
            .declare_deferred_connection_at(
                id("a"),
                GridPos::new(2, 1),
                info("b", 0),
                GridPos::new(1, 1),
            )
            .unwrap();
        let before = format!("{world:?}");

        assert!(
            world
                .materialize_passage_destination_with_connections(
                    GridPos::new(2, 1),
                    ZoneBlueprint {
                        info: info("b", 0),
                        map: map(),
                        entrance: GridPos::new(1, 1),
                        seed: 888,
                        actors: vec![],
                        loot: vec![],
                        threat_sources: vec![],
                    },
                    &[ZoneConnectionBlueprint {
                        at: GridPos::new(0, 0),
                        destination: info("c", 0),
                        arrival: GridPos::new(1, 1),
                    }],
                )
                .is_err()
        );
        assert_eq!(format!("{world:?}"), before);
        assert!(world.pending.is_empty());
        assert!(!world.information.contains_key(&id("c")));
    }

    #[test]
    fn bounded_threat_source_renews_offscreen_without_leaking_spawn_information() {
        let mut world = world();
        world
            .pending
            .get_mut(&id("b"))
            .unwrap()
            .threat_sources
            .push(ThreatSourceBlueprint {
                position: GridPos::new(6, 5),
                interval_turns: std::num::NonZeroU16::new(2).unwrap(),
                maximum_active: std::num::NonZeroU16::new(1).unwrap(),
                maximum_total: std::num::NonZeroU16::new(1).unwrap(),
                actor: Actor::new(GridPos::new(6, 5), 4)
                    .unwrap()
                    .with_ai(AiProfile::idle())
                    .with_defeat_reward(crate::progression::DefeatReward::summoned(0, 0)),
            });

        outward(&mut world);
        home(&mut world);

        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert!(
            !world
                .events()
                .iter()
                .any(|event| matches!(event, GameEvent::EntitySpawned { .. }))
        );
        world.drain_events();
        for _ in 0..3 {
            apply(&mut world, GameCommand::Wait);
        }
        outward(&mut world);

        assert_eq!(world.threat_sources()[0].spawned_total(), 1);
        assert_eq!(
            world
                .actors()
                .iter()
                .filter(|(_, actor)| actor.threat_source() == Some(0))
                .count(),
            1
        );
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
        assert!(!world.events().iter().any(|event| matches!(event, GameEvent::EntityDied { entity, .. } | GameEvent::EntityMoved { entity, .. } if *entity == enemy)));
        world.drain_events();
        home(&mut world);
        assert!(world.actors().get(enemy).is_none());
        outward(&mut world);
        home(&mut world);
        assert!(world.actors().get(enemy).is_none());
    }

    #[test]
    fn offscreen_leashed_actor_returns_to_its_home() {
        let mut world = world();
        let enemy = world
            .actors()
            .iter()
            .find(|(id, _)| *id != world.player_id())
            .unwrap()
            .0;
        let home = GridPos::new(7, 5);
        let profile = AiProfile::hunter(8, 0).with_maximum_pursuit_distance(
            std::num::NonZeroU16::new(4).expect("constant is non-zero"),
        );
        let actor = world.active.actors.remove(enemy).unwrap().with_ai(profile);
        world.active.actors.insert_existing(enemy, actor);
        world
            .active
            .actors
            .move_to(enemy, GridPos::new(5, 5))
            .unwrap();

        outward(&mut world);
        assert_eq!(
            world.inactive[&id("a")]
                .actors
                .get(enemy)
                .unwrap()
                .position(),
            GridPos::new(6, 5)
        );
        apply(&mut world, GameCommand::Wait);
        assert_eq!(
            world.inactive[&id("a")]
                .actors
                .get(enemy)
                .unwrap()
                .position(),
            home
        );
    }

    #[test]
    fn offscreen_alarm_responder_follows_only_its_recorded_incident() {
        let mut world = world();
        let enemy = world
            .actors()
            .iter()
            .find(|(id, _)| *id != world.player_id())
            .unwrap()
            .0;
        let profile = AiProfile::hunter(8, 0)
            .with_maximum_pursuit_distance(std::num::NonZeroU16::new(4).unwrap())
            .with_pursuit_lifecycle(crate::ai::PursuitLifecycle::new(
                std::num::NonZeroU16::new(6).unwrap(),
                std::num::NonZeroU16::new(2).unwrap(),
                std::num::NonZeroU16::new(2).unwrap(),
            ));
        let mut actor = world.active.actors.remove(enemy).unwrap().with_ai(profile);
        actor.set_ai_state(AiState::Responding {
            remaining_turns: 6,
            incident: GridPos::new(4, 5),
        });
        world.active.actors.insert_existing(enemy, actor);

        outward(&mut world);
        assert_eq!(
            world.inactive[&id("a")]
                .actors
                .get(enemy)
                .unwrap()
                .position(),
            GridPos::new(6, 5)
        );
        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(
            world.inactive[&id("a")]
                .actors
                .get(enemy)
                .unwrap()
                .position(),
            GridPos::new(5, 5)
        );
        assert!(!world.events().iter().any(
            |event| matches!(event, GameEvent::EntityMoved { entity, .. } if *entity == enemy)
        ));
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
    fn data_terminal_interaction_costs_time_and_discovers_each_record_only_once() {
        let game = GameState::new(map(), GridPos::new(1, 1), 42).unwrap();
        let mut world = WorldState::single(game);
        world.enable(info("a", 0)).unwrap();
        let terminal = id("terminal");
        let second_terminal = id("second_terminal");
        let record = id("record");
        world
            .register_facility(
                id("a"),
                FacilityBlueprint {
                    installations: vec![
                        InstallationBlueprint {
                            id: terminal.clone(),
                            position: GridPos::new(2, 1),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::DataTerminal {
                                record: record.clone(),
                            }],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                        InstallationBlueprint {
                            id: second_terminal.clone(),
                            position: GridPos::new(1, 2),
                            maximum_integrity: 10,
                            integrity: 10,
                            capabilities: vec![InstallationCapability::DataTerminal {
                                record: record.clone(),
                            }],
                            dependencies: vec![],
                            security_alarm_profile: None,
                        },
                        InstallationBlueprint {
                            id: id("depot"),
                            position: GridPos::new(3, 1),
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
                    owner: None,
                },
            )
            .unwrap();
        world.drain_events();

        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(2, 1),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(world.turn(), 1);
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::Facility(FacilityEvent::DataTerminalAccessed {
                installation,
                record: event_record,
                first_access: true,
                ..
            }) if installation == &terminal && event_record == &record
        )));
        assert!(
            world
                .active_facility()
                .unwrap()
                .data_terminal_was_accessed(&terminal)
        );
        assert_eq!(
            world.discovered_data_terminal_records(),
            BTreeSet::from([record.clone()])
        );
        world.drain_events();

        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(1, 2),
            }),
            CommandOutcome::Applied
        );
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::Facility(FacilityEvent::DataTerminalAccessed {
                installation,
                record: event_record,
                first_access: false,
                ..
            }) if installation == &second_terminal && event_record == &record
        )));
        assert!(
            world
                .active_facility()
                .unwrap()
                .data_terminal_was_accessed(&second_terminal)
        );
        assert_eq!(world.discovered_data_terminal_records().len(), 1);
    }

    #[test]
    fn data_terminal_discovery_is_shared_between_active_and_offscreen_facilities() {
        let mut world = world();
        let record = id("shared_record");
        let facility = |terminal: ContentId, terminal_position: GridPos| FacilityBlueprint {
            installations: vec![
                InstallationBlueprint {
                    id: terminal,
                    position: terminal_position,
                    maximum_integrity: 10,
                    integrity: 10,
                    capabilities: vec![InstallationCapability::DataTerminal {
                        record: record.clone(),
                    }],
                    dependencies: vec![],
                    security_alarm_profile: None,
                },
                InstallationBlueprint {
                    id: id(&format!(
                        "depot_{}_{}",
                        terminal_position.x, terminal_position.y
                    )),
                    position: GridPos::new(4, 1),
                    maximum_integrity: 10,
                    integrity: 10,
                    capabilities: vec![InstallationCapability::Storage],
                    dependencies: vec![],
                    security_alarm_profile: None,
                },
            ],
            depot: id(&format!(
                "depot_{}_{}",
                terminal_position.x, terminal_position.y
            )),
            workers: vec![],
            repair_orders: vec![],
            maximum_path_search: 256,
            owner: None,
        };
        let first_terminal = id("terminal_a");
        let second_terminal = id("terminal_b");
        world
            .register_facility(
                id("a"),
                facility(first_terminal.clone(), GridPos::new(3, 1)),
            )
            .unwrap();
        world
            .register_facility(
                id("b"),
                facility(second_terminal.clone(), GridPos::new(1, 2)),
            )
            .unwrap();

        apply(&mut world, GameCommand::Move(Direction::East));
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(3, 1),
            }),
            CommandOutcome::Applied
        );
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::Facility(FacilityEvent::DataTerminalAccessed {
                installation,
                first_access: true,
                ..
            }) if installation == &first_terminal
        )));
        world.drain_events();

        outward(&mut world);
        assert_eq!(world.current_zone().unwrap().id, id("b"));
        assert_eq!(
            world.process_player_command(GameCommand::Interact {
                target: GridPos::new(1, 2),
            }),
            CommandOutcome::Applied
        );
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::Facility(FacilityEvent::DataTerminalAccessed {
                installation,
                record: event_record,
                first_access: false,
                ..
            }) if installation == &second_terminal && event_record == &record
        )));
        assert_eq!(
            world.discovered_data_terminal_records(),
            BTreeSet::from([record])
        );
        assert!(
            world
                .facility_in_zone(&id("a"))
                .unwrap()
                .data_terminal_was_accessed(&first_terminal)
        );
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
    fn observed_owned_loot_accelerates_a_real_finite_threat_source() {
        let mut world = world();
        let source = GridPos::new(6, 1);
        world
            .spawn_ground_item_with_owner(
                GridPos::new(1, 1),
                id("repair"),
                1,
                Some(id("collective")),
            )
            .unwrap();
        world
            .active
            .install_threat_sources(vec![ThreatSourceBlueprint {
                position: source,
                interval_turns: std::num::NonZeroU16::new(20).unwrap(),
                maximum_active: std::num::NonZeroU16::new(1).unwrap(),
                maximum_total: std::num::NonZeroU16::new(1).unwrap(),
                actor: Actor::new(source, 4)
                    .unwrap()
                    .with_ai(
                        AiProfile::hunter(1, 0)
                            .with_maximum_pursuit_distance(std::num::NonZeroU16::new(6).unwrap())
                            .with_pursuit_lifecycle(crate::ai::PursuitLifecycle::new(
                                std::num::NonZeroU16::new(6).unwrap(),
                                std::num::NonZeroU16::new(2).unwrap(),
                                std::num::NonZeroU16::new(2).unwrap(),
                            )),
                    )
                    .with_defeat_reward(crate::progression::DefeatReward::summoned(0, 0)),
            }])
            .unwrap();
        world
            .register_facility(
                id("a"),
                FacilityBlueprint {
                    installations: vec![
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
                                    8,
                                )
                                .unwrap()
                                .with_responses(vec![
                                    SecurityAlarmResponse::CallInvestigatingReinforcements {
                                        source,
                                        delay_turns: 3,
                                    },
                                ])
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
        assert_eq!(world.threat_sources()[0].remaining_turns(), 3);
        assert!(world.events().iter().any(|event| matches!(
            event,
            GameEvent::Facility(FacilityEvent::InvestigatingReinforcementsRequested {
                source: event_source,
                incident: GridPos { x: 1, y: 1 },
                delay_turns: 3,
                ..
            }) if *event_source == source
        )));
        world.drain_events();
        for _ in 0..3 {
            apply(&mut world, GameCommand::Wait);
        }
        assert_eq!(world.threat_sources()[0].spawned_total(), 1);
        assert_eq!(
            world
                .actors()
                .iter()
                .filter(|(_, actor)| actor.threat_source() == Some(0))
                .count(),
            1
        );
        assert!(world.actors().iter().any(|(_, actor)| matches!(
            actor.ai_state(),
            AiState::Responding {
                incident: GridPos { x: 1, y: 1 },
                ..
            }
        )));
    }

    #[test]
    fn investigating_alarm_rejects_a_source_without_compatible_mobile_ai() {
        let mut world = world();
        let source = GridPos::new(6, 1);
        world
            .active
            .install_threat_sources(vec![ThreatSourceBlueprint {
                position: source,
                interval_turns: std::num::NonZeroU16::new(20).unwrap(),
                maximum_active: std::num::NonZeroU16::new(1).unwrap(),
                maximum_total: std::num::NonZeroU16::new(1).unwrap(),
                actor: Actor::new(source, 4)
                    .unwrap()
                    .with_ai(AiProfile::idle())
                    .with_defeat_reward(crate::progression::DefeatReward::summoned(0, 0)),
            }])
            .unwrap();
        let result = world.register_facility(
            id("a"),
            FacilityBlueprint {
                installations: vec![
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
                                8,
                            )
                            .unwrap()
                            .with_responses(vec![
                                SecurityAlarmResponse::CallInvestigatingReinforcements {
                                    source,
                                    delay_turns: 3,
                                },
                            ])
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
        );

        assert!(result.unwrap_err().contains("requires mobile AI"));
        assert!(world.active_facility().is_none());
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
