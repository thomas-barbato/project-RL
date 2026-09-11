use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use crate::ai::{AiAction, AiSituation, decide_action};
use crate::combat::{
    AttackArea, AttackAreaCell, AttackPreview, AttackProfile, DamagePacket, resolve_damage,
};
use crate::effects::{
    ApplyStatusEffect, EffectPrimitive, GroundEffectMap, GroundEffectSpec, RadialDamageEffect,
};
use crate::entity::{
    Actor, ActorBuildError, ActorRegistry, EntityId, Equipment, EquipmentError, EquipmentSlotId,
    GroundItemId, GroundItemRegistry, GroundItemRegistryError, Inventory, InventoryError,
    ItemInstanceId, RegistryError,
};
use crate::item::{ItemEffect, ItemId, ItemKind};
use crate::progression::{
    DefeatReward, ExperienceAward, PlayerProgressionSaveError, ProgressionRulesError,
    RunProgression, SkillPointSpendError, decode_player_progression, encode_player_progression,
};
use crate::resources::{EnergyReserve, EnergyReserveError, EnergySpendError};
use crate::skills::{
    DisciplineAvailability, DisciplineId, InitialChoicesError, SkillCatalogError,
    SkillProgressionState, TechniqueAction, TechniqueId, TechniqueLearningError,
};
use crate::social::{ObservedPropertyTake, SocialGroupId};
use crate::stats::{PrimaryAttributes, PrimaryAttributesError};
use crate::status::{
    StatusCatalog, StatusEffectPrimitive, StatusId, StatusInstance, StatusTrigger,
};
use crate::weapon::{WeaponDefinition, WeaponEffect, WeaponId};
use crate::world::generation::GeneratedMap;
use crate::world::{
    Direction, DoorState, GridPos, Map, MovementTraceMap, MovementTraceRulesError, Terrain,
    VisibilityState, compute_visible_tiles, has_line_of_sight,
};

use super::{ExperienceSource, GameCommand, GameEvent, GameRng, GameRules, TurnPhase};

pub struct GameState {
    pub(super) map: Map,
    pub(super) actors: ActorRegistry,
    pub(super) player: EntityId,
    pub(super) exit: Option<GridPos>,
    pub(super) turn: u64,
    pub(super) phase: TurnPhase,
    pub(super) status: RunStatus,
    pub(super) events: Vec<GameEvent>,
    pub(super) rng: GameRng,
    pub(super) rules: GameRules,
    pub(super) player_visibility: VisibilityState,
    player_progression: RunProgression,
    player_skills: SkillProgressionState,
    player_inventory: Inventory,
    player_equipment: Equipment,
    pub(super) ground_items: GroundItemRegistry,
    pub(super) movement_traces: MovementTraceMap,
    player_energy: EnergyReserve,
    pub(super) ground_effects: GroundEffectMap,
}

// Empty ground effects are omitted so versions 1-9 retain the exact state
// fingerprint they had before persistent map effects existed.
impl Debug for GameState {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut state = formatter.debug_struct("GameState");
        state
            .field("map", &self.map)
            .field("actors", &self.actors)
            .field("player", &self.player)
            .field("exit", &self.exit)
            .field("turn", &self.turn)
            .field("phase", &self.phase)
            .field("status", &self.status)
            .field("events", &self.events)
            .field("rng", &self.rng)
            .field("rules", &self.rules)
            .field("player_visibility", &self.player_visibility)
            .field("player_progression", &self.player_progression)
            .field("player_skills", &self.player_skills)
            .field("player_inventory", &self.player_inventory)
            .field("player_equipment", &self.player_equipment)
            .field("ground_items", &self.ground_items)
            .field("movement_traces", &self.movement_traces)
            .field("player_energy", &self.player_energy);
        if !self.ground_effects.is_empty() {
            state.field("ground_effects", &self.ground_effects);
        }
        state.finish()
    }
}

impl GameState {
    pub fn new(map: Map, player_start: GridPos, seed: u64) -> Result<Self, GameInitError> {
        Self::new_with_rules(map, player_start, seed, GameRules::default())
    }

    pub fn new_with_rules(
        map: Map,
        player_start: GridPos,
        seed: u64,
        rules: GameRules,
    ) -> Result<Self, GameInitError> {
        Self::build(map, player_start, None, seed, rules)
    }

    pub fn from_generated(
        generated: GeneratedMap,
        rng_state: u64,
        rules: GameRules,
    ) -> Result<Self, GameInitError> {
        let (map, player_start, exit) = generated.into_parts();
        Self::build(map, player_start, Some(exit), rng_state, rules)
    }

    fn build(
        map: Map,
        player_start: GridPos,
        exit: Option<GridPos>,
        rng_state: u64,
        rules: GameRules,
    ) -> Result<Self, GameInitError> {
        rules
            .progression
            .validate()
            .map_err(GameInitError::ProgressionRules)?;
        rules
            .player_starting_attributes
            .validate_for_creation(rules.primary_attribute_rules)
            .map_err(GameInitError::PlayerAttributes)?;
        rules
            .skills
            .validate_runtime(&rules.enabled_system_features, &rules.skill_progression)
            .map_err(GameInitError::SkillCatalog)?;
        let player_energy =
            EnergyReserve::new(rules.player_energy_capacity, rules.player_starting_energy)
                .map_err(GameInitError::Energy)?;
        rules
            .movement_traces
            .validate()
            .map_err(GameInitError::MovementTraceRules)?;
        if let Some((weapon, status)) = first_unknown_weapon_status(&rules) {
            return Err(GameInitError::UnknownWeaponStatusDefinition {
                weapon: Box::new(weapon),
                status,
            });
        }
        if !map.is_walkable(player_start) {
            return Err(GameInitError::BlockedPlayerStart(player_start));
        }
        if let Some(exit) = exit
            && !map.is_walkable(exit)
        {
            return Err(GameInitError::BlockedExit(exit));
        }

        let mut unique_slots = BTreeSet::new();
        for slot in &rules.player_weapon_slots {
            if !unique_slots.insert(slot.clone()) {
                return Err(GameInitError::DuplicateEquipmentSlot(slot.clone()));
            }
        }
        let mut player_inventory = Inventory::new(rules.player_inventory_capacity);
        let mut player_equipment = Equipment::default();
        for weapon_id in &rules.player_starting_weapons {
            if rules.weapons.get(weapon_id).is_none() {
                return Err(GameInitError::UnknownStartingWeapon(weapon_id.clone()));
            }
            player_inventory
                .add(weapon_id.clone(), 1, 1)
                .map_err(GameInitError::Inventory)?;
        }
        for starting_item in &rules.player_starting_items {
            let definition = rules
                .items
                .get(&starting_item.item)
                .ok_or_else(|| GameInitError::UnknownStartingItem(starting_item.item.clone()))?;
            player_inventory
                .add(
                    starting_item.item.clone(),
                    starting_item.quantity,
                    definition.maximum_stack(),
                )
                .map_err(GameInitError::Inventory)?;
        }
        if rules.player_starting_equipment.len() > rules.player_weapon_slots.len() {
            return Err(GameInitError::TooManyStartingEquipmentEntries {
                equipment: rules.player_starting_equipment.len(),
                slots: rules.player_weapon_slots.len(),
            });
        }
        for (index, weapon_id) in rules.player_starting_equipment.iter().enumerate() {
            let Some(weapon_id) = weapon_id else {
                continue;
            };
            let item = player_inventory
                .iter()
                .find(|entry| {
                    entry.item() == weapon_id
                        && player_equipment.slot_of(entry.instance()).is_none()
                })
                .map(|entry| entry.instance())
                .ok_or_else(|| {
                    GameInitError::StartingEquipmentWeaponNotInInventory(weapon_id.clone())
                })?;
            player_equipment
                .equip(
                    rules.player_weapon_slots[index].clone(),
                    item,
                    &player_inventory,
                )
                .map_err(GameInitError::Equipment)?;
        }

        let player_actor = Actor::new(player_start, rules.player_maximum_integrity)
            .map_err(GameInitError::Actor)?
            .with_attacks(rules.player_base_attacks.iter().copied())
            .with_abilities(rules.player_base_abilities.iter().cloned())
            .with_primary_attributes(rules.player_starting_attributes);
        if let Some(status) = first_unknown_status(&player_actor, &rules.statuses) {
            return Err(GameInitError::UnknownStatusDefinition(status));
        }
        let mut actors = ActorRegistry::default();
        let player = actors
            .spawn(player_actor)
            .map_err(GameInitError::Registry)?;
        let mut player_visibility = VisibilityState::default();
        player_visibility.recompute(&map, player_start, rules.player_field_of_view);
        let player_progression =
            RunProgression::with_starting_skill_points(rules.progression.starting_skill_points);

        Ok(Self {
            map,
            actors,
            player,
            exit,
            turn: 0,
            phase: TurnPhase::AwaitingPlayer,
            status: RunStatus::Active,
            events: Vec::new(),
            rng: GameRng::from_seed(rng_state),
            rules,
            player_visibility,
            player_progression,
            player_skills: SkillProgressionState::default(),
            player_inventory,
            player_equipment,
            ground_items: GroundItemRegistry::default(),
            movement_traces: MovementTraceMap::default(),
            player_energy,
            ground_effects: GroundEffectMap::default(),
        })
    }

    pub const fn map(&self) -> &Map {
        &self.map
    }

    pub const fn actors(&self) -> &ActorRegistry {
        &self.actors
    }

    pub const fn player_id(&self) -> EntityId {
        self.player
    }

    pub const fn exit(&self) -> Option<GridPos> {
        self.exit
    }

    pub fn player_position(&self) -> Option<GridPos> {
        self.actors.get(self.player).map(Actor::position)
    }

    pub fn player_may_take_property_of(&self, owner: &SocialGroupId) -> bool {
        self.actors
            .get(self.player)
            .is_some_and(|player| player.may_take_property_of(owner))
    }

    /// Deterministic world-setup hook. Dynamic grants must eventually pass
    /// through a recorded game command before dialogue or quest systems use it.
    pub fn grant_player_property_take_authorization(&mut self, owner: SocialGroupId) -> bool {
        self.actors
            .get_mut(self.player)
            .expect("a valid game always retains its player")
            .grant_property_take_authorization(owner)
    }

    pub fn player_primary_attributes(&self) -> Option<PrimaryAttributes> {
        self.actors
            .get(self.player)
            .and_then(Actor::primary_attributes)
    }

    pub const fn turn(&self) -> u64 {
        self.turn
    }

    pub const fn phase(&self) -> TurnPhase {
        self.phase
    }

    pub const fn status(&self) -> RunStatus {
        self.status
    }

    pub const fn rng_state(&self) -> u64 {
        self.rng.state()
    }

    pub const fn rules(&self) -> &GameRules {
        &self.rules
    }

    pub const fn player_visibility(&self) -> &VisibilityState {
        &self.player_visibility
    }

    pub const fn player_progression(&self) -> &RunProgression {
        &self.player_progression
    }

    pub const fn player_skills(&self) -> &SkillProgressionState {
        &self.player_skills
    }

    pub const fn player_energy(&self) -> EnergyReserve {
        self.player_energy
    }

    pub const fn ground_effects(&self) -> &GroundEffectMap {
        &self.ground_effects
    }

    /// Visible, in-range candidates for a client preview, sorted by distance
    /// then stable ID. Neither explored tiles nor live hidden actors qualify.
    pub fn player_technique_targets(&self, technique: &TechniqueId) -> Vec<EntityId> {
        let Some(origin) = self.player_position() else {
            return Vec::new();
        };
        let Some(range) = self.rules.skills.target_range(technique) else {
            return Vec::new();
        };
        let mut candidates: Vec<_> = self
            .actors
            .iter()
            .filter(|(id, actor)| {
                *id != self.player
                    && self.player_visibility.is_visible(actor.position())
                    && is_within_chebyshev_range(origin, actor.position(), range)
            })
            .map(|(id, actor)| {
                let dx = (i64::from(actor.position().x) - i64::from(origin.x)).abs();
                let dy = (i64::from(actor.position().y) - i64::from(origin.y)).abs();
                (dx.max(dy), id)
            })
            .collect();
        candidates.sort_unstable();
        candidates.into_iter().map(|(_, id)| id).collect()
    }

    pub fn export_player_progression(&self) -> Result<String, PlayerProgressionSaveError> {
        encode_player_progression(&self.player_progression, &self.player_skills)
    }

    pub fn restore_player_progression(
        &mut self,
        source: &str,
    ) -> Result<(), PlayerProgressionSaveError> {
        let restored = decode_player_progression(
            source,
            &self.rules.progression,
            &self.rules.skills,
            &self.rules.enabled_system_features,
            &self.rules.skill_progression,
        )?;
        let (run, skills) = restored.into_parts();
        self.player_progression = run;
        self.player_skills = skills;
        Ok(())
    }

    pub const fn player_inventory(&self) -> &Inventory {
        &self.player_inventory
    }

    pub(super) fn player_inventory_mut(&mut self) -> &mut Inventory {
        &mut self.player_inventory
    }

    pub const fn player_equipment(&self) -> &Equipment {
        &self.player_equipment
    }

    pub fn discipline_availability(
        &self,
        discipline: &DisciplineId,
    ) -> Result<DisciplineAvailability, InitialChoicesError> {
        self.rules.skills.analyze_discipline(
            discipline,
            &self.rules.enabled_system_features,
            &[],
            &self.rules.skill_progression,
        )
    }

    pub const fn ground_items(&self) -> &GroundItemRegistry {
        &self.ground_items
    }

    pub const fn movement_traces(&self) -> &MovementTraceMap {
        &self.movement_traces
    }

    pub fn equipped_player_weapon(&self, slot: u8) -> Option<&WeaponDefinition> {
        let slot_id = self.rules.player_weapon_slots.get(usize::from(slot))?;
        let item = self.player_equipment.equipped(slot_id)?;
        let entry = self.player_inventory.get(item)?;
        self.rules.weapons.get(entry.item())
    }

    /// Resolves a freely aimed area footprint without spending a turn or
    /// mutating simulation state. Execution calls the same preparation path.
    pub fn player_attack_preview(
        &self,
        slot: u8,
        target: GridPos,
    ) -> Result<AttackPreview, CommandRejection> {
        self.prepare_player_area_attack(slot, target)
            .map(|prepared| {
                AttackPreview::new(prepared.origin, prepared.target_at, prepared.affected_cells)
            })
            .map_err(CommandRejection::from)
    }

    /// Returns the geometric footprint even when contextual rules such as a
    /// protected zone forbid confirmation. This lets clients explain an
    /// invalid aim without inventing or partially hiding the attack shape.
    pub fn player_attack_footprint(
        &self,
        slot: u8,
        target: GridPos,
    ) -> Result<AttackPreview, CommandRejection> {
        self.prepare_player_area_footprint(slot, target)
            .map(|prepared| {
                AttackPreview::new(prepared.origin, prepared.target_at, prepared.affected_cells)
            })
            .map_err(CommandRejection::from)
    }

    pub fn events(&self) -> &[GameEvent] {
        &self.events
    }

    pub fn drain_events(&mut self) -> Vec<GameEvent> {
        std::mem::take(&mut self.events)
    }

    pub fn spawn_actor(&mut self, actor: Actor) -> Result<EntityId, SpawnError> {
        let position = actor.position();
        if !self.map.is_walkable(position) {
            return Err(SpawnError::BlockedByTerrain(position));
        }
        if self.actors.entity_at(position).is_some() {
            return Err(SpawnError::Occupied(position));
        }
        if let Some(status) = first_unknown_status(&actor, &self.rules.statuses) {
            return Err(SpawnError::UnknownStatusDefinition(status));
        }
        if let Some(attributes) = actor.primary_attributes() {
            attributes
                .validate_absolute(self.rules.primary_attribute_rules)
                .map_err(SpawnError::InvalidPrimaryAttributes)?;
        }

        let entity = self.actors.spawn(actor).map_err(SpawnError::Registry)?;
        self.events.push(GameEvent::EntitySpawned {
            entity,
            at: position,
        });
        Ok(entity)
    }

    /// Places a non-blocking item stack in the world.
    ///
    /// Ground items deliberately do not participate in movement or map
    /// connectivity. Their definitions only control inventory stacking once
    /// they are picked up.
    pub fn spawn_ground_item(
        &mut self,
        position: GridPos,
        item: ItemId,
        quantity: u16,
    ) -> Result<GroundItemId, GroundItemSpawnError> {
        self.spawn_ground_item_with_owner(position, item, quantity, None)
    }

    pub fn spawn_ground_item_with_owner(
        &mut self,
        position: GridPos,
        item: ItemId,
        quantity: u16,
        owner: Option<SocialGroupId>,
    ) -> Result<GroundItemId, GroundItemSpawnError> {
        if !self.map.is_walkable(position) {
            return Err(GroundItemSpawnError::BlockedByTerrain(position));
        }
        if self.inventory_stack_limit(&item).is_none() {
            return Err(GroundItemSpawnError::UnknownItemDefinition(item));
        }

        let definition = item.clone();
        let ground_item = self
            .ground_items
            .spawn_with_owner(position, item, quantity, owner)
            .map_err(GroundItemSpawnError::Registry)?;
        self.events.push(GameEvent::GroundItemSpawned {
            ground_item,
            definition,
            quantity,
            at: position,
        });
        Ok(ground_item)
    }

    pub fn process_player_command(&mut self, command: GameCommand) -> CommandOutcome {
        if self.status != RunStatus::Active {
            return CommandOutcome::Rejected(CommandRejection::RunEnded);
        }
        if self.phase != TurnPhase::AwaitingPlayer {
            return CommandOutcome::Rejected(CommandRejection::NotPlayersTurn);
        }

        let outcome = match command {
            GameCommand::Interact { target } => match self.interact(target) {
                Ok(()) => CommandOutcome::Applied,
                Err(error) => CommandOutcome::Rejected(error),
            },
            GameCommand::Move(direction) => match self.move_entity(self.player, direction) {
                Ok(destination) => {
                    self.player_visibility.recompute(
                        &self.map,
                        destination,
                        self.rules.player_field_of_view,
                    );
                    self.events.push(GameEvent::VisibilityUpdated {
                        observer: self.player,
                        origin: destination,
                    });
                    if self.exit == Some(destination) {
                        self.status = RunStatus::Escaped;
                        self.events.push(GameEvent::ExitReached {
                            entity: self.player,
                            at: destination,
                        });
                    }
                    CommandOutcome::Applied
                }
                Err(error) => CommandOutcome::Rejected(error.into()),
            },
            GameCommand::Wait => {
                self.events.push(GameEvent::EntityWaited {
                    entity: self.player,
                });
                CommandOutcome::Applied
            }
            GameCommand::Attack { slot, target } => {
                match self.perform_attack(self.player, slot, target) {
                    Ok(()) => CommandOutcome::Applied,
                    Err(error) => CommandOutcome::Rejected(error.into()),
                }
            }
            GameCommand::AttackAt { slot, target } => {
                match self.perform_player_area_attack(slot, target) {
                    Ok(()) => CommandOutcome::Applied,
                    Err(error) => CommandOutcome::Rejected(error.into()),
                }
            }
            GameCommand::EquipWeapon { slot, item } => match self.equip_player_weapon(slot, item) {
                Ok(()) => CommandOutcome::Applied,
                Err(error) => CommandOutcome::Rejected(error.into()),
            },
            GameCommand::UseItem { item } => match self.use_player_item(item) {
                Ok(()) => CommandOutcome::Applied,
                Err(error) => CommandOutcome::Rejected(error.into()),
            },
            GameCommand::PickUp => match self.pick_up_at_player() {
                Ok(()) => CommandOutcome::Applied,
                Err(error) => CommandOutcome::Rejected(error.into()),
            },
            GameCommand::DropItem { item } => match self.drop_player_item(item) {
                Ok(()) => CommandOutcome::Applied,
                Err(error) => CommandOutcome::Rejected(error.into()),
            },
            GameCommand::UseAbility { slot, target } => {
                match self.perform_ability(self.player, slot, target) {
                    Ok(()) => CommandOutcome::Applied,
                    Err(error) => CommandOutcome::Rejected(error.into()),
                }
            }
            GameCommand::LearnTechnique { technique } => {
                match self.learn_player_technique(&technique) {
                    Ok(()) => CommandOutcome::AppliedWithoutTime,
                    Err(error) => CommandOutcome::Rejected(error.into()),
                }
            }
            GameCommand::UseTechnique { technique, targets } => {
                match self.use_player_technique(&technique, &targets) {
                    Ok(()) => CommandOutcome::Applied,
                    Err(error) => CommandOutcome::Rejected(error.into()),
                }
            }
        };

        if outcome == CommandOutcome::Applied {
            self.complete_turn();
        }

        outcome
    }

    fn interact(&mut self, target: GridPos) -> Result<(), CommandRejection> {
        let origin = self
            .player_position()
            .ok_or(CommandRejection::MissingPlayer)?;
        if !origin.cardinal_neighbors().contains(&target)
            || !self.player_visibility.is_visible(target)
        {
            return Err(CommandRejection::InteractionOutOfReach);
        }
        let terrain = self
            .map
            .tile(target)
            .ok_or(CommandRejection::NothingToInteract)?
            .terrain;
        let next = match terrain {
            Terrain::Door(DoorState::Locked) => return Err(CommandRejection::DoorLocked),
            Terrain::Door(DoorState::Unpowered) => return Err(CommandRejection::DoorUnpowered),
            Terrain::Door(DoorState::Closed) => Terrain::Door(DoorState::Open),
            Terrain::Door(DoorState::Open) => {
                if self.actors.entity_at(target).is_some()
                    || self.ground_items.item_at(target).is_some()
                {
                    return Err(CommandRejection::DoorObstructed);
                }
                Terrain::Door(DoorState::Closed)
            }
            Terrain::ControlPanel {
                door,
                activated: false,
            } => {
                if !matches!(
                    self.map.tile(door).map(|t| t.terrain),
                    Some(Terrain::Door(DoorState::Locked))
                ) {
                    return Err(CommandRejection::ControlUnavailable);
                }
                self.map
                    .set_terrain(door, Terrain::Door(DoorState::Closed))
                    .map_err(|_| CommandRejection::ControlUnavailable)?;
                // Report the local action only, not the remote door's live state.
                Terrain::ControlPanel {
                    door,
                    activated: true,
                }
            }
            Terrain::ControlPanel {
                activated: true, ..
            } => return Err(CommandRejection::ControlUnavailable),
            _ => return Err(CommandRejection::NothingToInteract),
        };
        self.map
            .set_terrain(target, next)
            .map_err(|_| CommandRejection::NothingToInteract)?;
        self.events.push(GameEvent::TerrainInteracted {
            entity: self.player,
            at: target,
            terrain: next,
        });
        self.player_visibility
            .recompute(&self.map, origin, self.rules.player_field_of_view);
        self.events.push(GameEvent::VisibilityUpdated {
            observer: self.player,
            origin,
        });
        Ok(())
    }

    fn learn_player_technique(
        &mut self,
        technique: &TechniqueId,
    ) -> Result<(), LearnTechniqueError> {
        let mut next_skills = self.player_skills.clone();
        let learned = next_skills
            .learn(
                technique,
                &self.rules.skills,
                &self.rules.enabled_system_features,
                &self.rules.skill_progression,
            )
            .map_err(LearnTechniqueError::Technique)?;
        let mut next_progression = self.player_progression.clone();
        next_progression
            .spend_skill_points(learned.cost)
            .map_err(LearnTechniqueError::Points)?;

        self.player_skills = next_skills;
        self.player_progression = next_progression;
        self.events.push(GameEvent::TechniqueLearned {
            entity: self.player,
            technique: learned.technique,
            discipline: learned.discipline,
            rank: learned.rank,
            skill_points_spent: learned.cost,
            skill_points_remaining: self.player_progression.unspent_skill_points(),
        });
        Ok(())
    }

    fn use_player_technique(
        &mut self,
        technique: &TechniqueId,
        targets: &[EntityId],
    ) -> Result<(), TechniqueUseError> {
        if !self.player_skills.has_learned(technique) {
            return Err(TechniqueUseError::NotLearned(technique.clone()));
        }
        let definition = self
            .rules
            .skills
            .technique(technique)
            .ok_or_else(|| TechniqueUseError::UnknownTechnique(technique.clone()))?;
        let action = definition
            .action()
            .ok_or_else(|| TechniqueUseError::NoActiveAction(technique.clone()))?;
        let player_position = self
            .player_position()
            .ok_or(TechniqueUseError::MissingPlayer)?;
        let (maximum_targets, energy_cost) = match action {
            TechniqueAction::AnalyzeTarget { .. } | TechniqueAction::AnalyzeThreat { .. } => (1, 0),
            TechniqueAction::AnalyzeMultipleTargets {
                maximum_targets,
                energy_cost,
            } => (usize::from(maximum_targets), energy_cost),
            _ => (0, 0),
        };
        if targets.is_empty() && maximum_targets > 0 {
            return Err(TechniqueUseError::MissingTarget);
        }
        if maximum_targets == 0 && !targets.is_empty() {
            return Err(TechniqueUseError::UnexpectedTarget);
        }
        if targets.len() > maximum_targets {
            return Err(TechniqueUseError::TooManyTargets {
                maximum: maximum_targets,
                actual: targets.len(),
            });
        }
        let mut unique = BTreeSet::new();
        for target in targets {
            if !unique.insert(*target) {
                return Err(TechniqueUseError::DuplicateTarget(*target));
            }
        }

        // Stage every observation before charging energy or publishing any result.
        // A hidden/invalid final target rejects the entire command atomically.
        let observations = match action {
            TechniqueAction::AnalyzeTarget { range } => {
                vec![self.target_analysis_event(targets[0], player_position, range)?]
            }
            TechniqueAction::AnalyzeMultipleTargets { .. } => {
                let parent = definition
                    .prerequisite()
                    .filter(|id| self.player_skills.has_learned(id))
                    .and_then(|id| self.rules.skills.technique(id))
                    .and_then(|definition| definition.action());
                let Some(TechniqueAction::AnalyzeTarget { range }) = parent else {
                    return Err(TechniqueUseError::NoActiveAction(technique.clone()));
                };
                targets
                    .iter()
                    .map(|target| self.target_analysis_event(*target, player_position, range))
                    .collect::<Result<Vec<_>, _>>()?
            }
            TechniqueAction::ReadMovementTraces { radius } => {
                let traces = self.movement_traces.observe(
                    player_position,
                    radius,
                    self.turn,
                    &self.player_visibility,
                    self.rules.movement_traces,
                );
                vec![GameEvent::MovementTracesRead {
                    observer: self.player,
                    traces,
                }]
            }
            TechniqueAction::AnalyzeNearbyWalls {
                radius,
                maximum_tiles,
            } => {
                let tiles = analyzed_wall_tiles(
                    &self.map,
                    &self.player_visibility,
                    player_position,
                    radius,
                    maximum_tiles,
                );
                vec![GameEvent::TerrainAnalyzed {
                    observer: self.player,
                    tiles,
                }]
            }
            TechniqueAction::AnalyzeThreat { range } => {
                let target = targets[0];
                let actor = self.visible_technique_target(target, player_position, range)?;
                vec![GameEvent::ThreatAnalyzed {
                    observer: self.player,
                    target,
                    attacks: actor.attacks().to_vec(),
                    resistances: actor.resistances(),
                }]
            }
        };
        self.player_energy
            .spend(energy_cost)
            .map_err(TechniqueUseError::Energy)?;
        self.events.push(GameEvent::TechniqueUsed {
            entity: self.player,
            technique: technique.clone(),
            observed_on_turn: self.turn,
        });
        if energy_cost > 0 {
            self.events.push(GameEvent::EnergySpent {
                entity: self.player,
                amount: energy_cost,
                remaining: self.player_energy.available(),
            });
        }
        self.events.extend(observations);
        Ok(())
    }

    fn visible_technique_target(
        &self,
        target: EntityId,
        origin: GridPos,
        range: u16,
    ) -> Result<&Actor, TechniqueUseError> {
        let actor = self
            .actors
            .get(target)
            .ok_or(TechniqueUseError::UnknownTarget(target))?;
        if !self.player_visibility.is_visible(actor.position()) {
            return Err(TechniqueUseError::TargetNotVisible(target));
        }
        if !is_within_chebyshev_range(origin, actor.position(), range) {
            return Err(TechniqueUseError::TargetOutOfRange(target));
        }
        Ok(actor)
    }

    fn target_analysis_event(
        &self,
        target: EntityId,
        origin: GridPos,
        range: u16,
    ) -> Result<GameEvent, TechniqueUseError> {
        let actor = self.visible_technique_target(target, origin, range)?;
        Ok(GameEvent::TargetAnalyzed {
            observer: self.player,
            target,
            at: actor.position(),
            integrity: actor.integrity(),
            maximum_integrity: actor.maximum_integrity(),
            resistances: actor.resistances(),
        })
    }

    pub(super) fn move_entity(
        &mut self,
        entity: EntityId,
        direction: Direction,
    ) -> Result<GridPos, MovementError> {
        let origin = self
            .actors
            .get(entity)
            .map(Actor::position)
            .ok_or(MovementError::MissingEntity(entity))?;
        let destination = origin.step(direction);

        if !self.map.is_walkable(destination)
            || (entity != self.player && self.map.is_protected(destination))
        {
            return Err(MovementError::BlockedByTerrain(destination));
        }
        if self.actors.entity_at(destination).is_some() {
            return Err(MovementError::Occupied(destination));
        }

        self.actors
            .move_to(entity, destination)
            .map_err(|_| MovementError::MissingEntity(entity))?;
        if self
            .rules
            .enabled_system_features
            .iter()
            .any(|feature| feature.as_str() == "core:traces")
        {
            self.movement_traces
                .record(origin, direction, self.turn, self.rules.movement_traces);
        }
        self.events.push(GameEvent::EntityMoved {
            entity,
            from: origin,
            to: destination,
        });
        Ok(destination)
    }

    fn perform_attack(
        &mut self,
        attacker: EntityId,
        slot: u8,
        target: EntityId,
    ) -> Result<(), AttackError> {
        let target_state = self
            .actors
            .get(target)
            .cloned()
            .ok_or(AttackError::UnknownTarget(target))?;
        let (attacker_state, attack, weapon, weapon_effects) =
            self.attack_details(attacker, slot)?;
        if self.map.is_protected(attacker_state.position())
            || self.map.is_protected(target_state.position())
        {
            return Err(AttackError::ProtectedZone);
        }
        if !attack.is_in_range(attacker_state.position(), target_state.position()) {
            return Err(AttackError::TargetOutOfRange(target));
        }
        if attack.requires_line_of_sight()
            && !has_line_of_sight(
                &self.map,
                attacker_state.position(),
                target_state.position(),
                true,
            )
        {
            return Err(AttackError::NoLineOfSight(target));
        }

        let origin = attacker_state.position();
        let target_at = target_state.position();
        let affected_cells = attack.affected_cells(&self.map, origin, target_at);
        self.resolve_prepared_attack(PreparedAttack {
            attacker,
            target: Some(target),
            slot,
            origin,
            target_at,
            attack,
            weapon,
            weapon_effects,
            affected_cells,
        })
    }

    fn perform_player_area_attack(&mut self, slot: u8, target: GridPos) -> Result<(), AttackError> {
        let prepared = self.prepare_player_area_attack(slot, target)?;
        self.resolve_prepared_attack(prepared)
    }

    fn prepare_player_area_attack(
        &self,
        slot: u8,
        target_at: GridPos,
    ) -> Result<PreparedAttack, AttackError> {
        let prepared = self.prepare_player_area_footprint(slot, target_at)?;
        let origin = prepared.origin;
        let attack = prepared.attack;
        if self.map.is_protected(origin) || self.map.is_protected(target_at) {
            return Err(AttackError::ProtectedZone);
        }
        if !attack.is_in_range(origin, target_at) {
            return Err(AttackError::PositionOutOfRange(target_at));
        }
        if attack.requires_line_of_sight() && !has_line_of_sight(&self.map, origin, target_at, true)
        {
            return Err(AttackError::NoLineOfSightAt(target_at));
        }
        Ok(prepared)
    }

    fn prepare_player_area_footprint(
        &self,
        slot: u8,
        target_at: GridPos,
    ) -> Result<PreparedAttack, AttackError> {
        let (attacker_state, attack, weapon, weapon_effects) =
            self.attack_details(self.player, slot)?;
        let origin = attacker_state.position();
        if matches!(attack.area(), AttackArea::Single) {
            return Err(AttackError::FreeAimRequiresAreaWeapon);
        }
        if !self.map.contains(target_at) {
            return Err(AttackError::TargetOutsideMap(target_at));
        }
        if target_at == origin {
            return Err(AttackError::TargetIsOrigin);
        }
        let affected_cells = attack.affected_cells(&self.map, origin, target_at);
        Ok(PreparedAttack {
            attacker: self.player,
            target: self
                .actors
                .entity_at(target_at)
                .filter(|entity| *entity != self.player),
            slot,
            origin,
            target_at,
            attack,
            weapon,
            weapon_effects,
            affected_cells,
        })
    }

    fn attack_details(
        &self,
        attacker: EntityId,
        slot: u8,
    ) -> Result<(Actor, AttackProfile, Option<WeaponId>, Vec<WeaponEffect>), AttackError> {
        let attacker_state = self
            .actors
            .get(attacker)
            .cloned()
            .ok_or(AttackError::MissingAttacker(attacker))?;
        if attacker == self.player && !self.rules.player_weapon_slots.is_empty() {
            let weapon = self
                .equipped_player_weapon(slot)
                .ok_or(AttackError::MissingAttackSlot(slot))?;
            Ok((
                attacker_state,
                weapon.attack(),
                Some(weapon.id().clone()),
                weapon.effects().to_vec(),
            ))
        } else {
            let attack = attacker_state
                .attack(slot)
                .ok_or(AttackError::MissingAttackSlot(slot))?;
            Ok((attacker_state, attack, None, Vec::new()))
        }
    }

    fn resolve_prepared_attack(&mut self, prepared: PreparedAttack) -> Result<(), AttackError> {
        let PreparedAttack {
            attacker,
            target,
            slot,
            origin,
            target_at,
            attack,
            weapon,
            weapon_effects,
            affected_cells,
        } = prepared;
        let affected_positions: BTreeSet<GridPos> =
            affected_cells.iter().map(|cell| cell.position).collect();
        let affected_targets: Vec<EntityId> = self
            .actors
            .iter()
            .filter_map(|(entity, actor)| {
                (entity != attacker
                    && affected_positions.contains(&actor.position())
                    && !self.map.is_protected(actor.position()))
                .then_some(entity)
            })
            .collect();
        self.events.push(GameEvent::AttackPerformed {
            attacker,
            target,
            slot,
            origin,
            target_at,
            weapon,
            damage_type: attack.damage().damage_type,
            affected_cells: affected_cells.clone(),
        });
        for affected_target in affected_targets {
            self.apply_damage_to(Some(attacker), affected_target, attack.damage())
                .map_err(|_| AttackError::UnknownTarget(affected_target))?;
            if self.actors.get(affected_target).is_none() {
                continue;
            }
            for effect in &weapon_effects {
                if let WeaponEffect::ApplyStatus(effect) = effect {
                    let _ = self.apply_status_to(Some(attacker), affected_target, effect);
                }
            }
        }
        for effect in &weapon_effects {
            if let WeaponEffect::CreateGroundEffect(effect) = effect {
                for cell in &affected_cells {
                    self.create_ground_effect(Some(attacker), cell.position, effect);
                }
            }
        }

        Ok(())
    }

    fn create_ground_effect(
        &mut self,
        source: Option<EntityId>,
        position: GridPos,
        effect: &GroundEffectSpec,
    ) {
        if !self.map.is_walkable(position) || self.map.is_protected(position) {
            return;
        }
        let remaining_turns = self
            .ground_effects
            .apply(position, source, effect, self.turn)
            .remaining_turns();
        self.events.push(GameEvent::GroundEffectCreated {
            source,
            effect: effect.id().clone(),
            at: position,
            remaining_turns,
        });
    }

    fn equip_player_weapon(
        &mut self,
        slot: u8,
        item: ItemInstanceId,
    ) -> Result<(), EquipWeaponError> {
        let equipment_slot = self
            .rules
            .player_weapon_slots
            .get(usize::from(slot))
            .cloned()
            .ok_or(EquipWeaponError::MissingEquipmentSlot(slot))?;
        if self.player_equipment.equipped(&equipment_slot) == Some(item) {
            return Err(EquipWeaponError::AlreadyEquipped { slot, item });
        }
        let entry = self
            .player_inventory
            .get(item)
            .ok_or(EquipWeaponError::UnknownInventoryItem(item))?;
        let weapon = self
            .rules
            .weapons
            .get(entry.item())
            .ok_or(EquipWeaponError::ItemIsNotWeapon(item))?;
        let weapon_id = weapon.id().clone();
        let outcome = self
            .player_equipment
            .equip(equipment_slot.clone(), item, &self.player_inventory)
            .map_err(EquipWeaponError::Equipment)?;
        self.events.push(GameEvent::WeaponEquipped {
            entity: self.player,
            slot,
            equipment_slot,
            item,
            weapon: weapon_id,
            displaced: outcome.displaced,
        });
        Ok(())
    }

    fn use_player_item(&mut self, item: ItemInstanceId) -> Result<(), UseItemError> {
        let entry = self
            .player_inventory
            .get(item)
            .cloned()
            .ok_or(UseItemError::UnknownInventoryItem(item))?;
        let definition = self
            .rules
            .items
            .get(entry.item())
            .cloned()
            .ok_or(UseItemError::ItemIsNotUsable(item))?;
        if definition.kind() != ItemKind::Consumable {
            return Err(UseItemError::ItemIsNotUsable(item));
        }
        let actor = self
            .actors
            .get(self.player)
            .ok_or(UseItemError::MissingPlayer)?;
        let missing_integrity = actor.maximum_integrity().saturating_sub(actor.integrity());
        let useful = definition.effects().iter().any(|effect| match effect {
            ItemEffect::RestoreIntegrity { amount } => *amount > 0 && missing_integrity > 0,
        });
        if !useful {
            return Err(UseItemError::NoUsefulEffect(item));
        }

        self.player_inventory
            .remove(item, 1)
            .map_err(|_| UseItemError::InventoryChanged(item))?;
        self.events.push(GameEvent::ItemUsed {
            entity: self.player,
            item,
            definition: definition.id().clone(),
        });
        for effect in definition.effects() {
            match effect {
                ItemEffect::RestoreIntegrity { amount } => {
                    let restored = self
                        .actors
                        .get_mut(self.player)
                        .ok_or(UseItemError::MissingPlayer)?
                        .restore_integrity(*amount);
                    if restored > 0 {
                        self.events.push(GameEvent::IntegrityRestored {
                            entity: self.player,
                            amount: restored,
                        });
                    }
                }
            }
        }
        Ok(())
    }

    fn pick_up_at_player(&mut self) -> Result<(), PickUpError> {
        let position = self
            .actors
            .get(self.player)
            .map(Actor::position)
            .ok_or(PickUpError::MissingPlayer)?;
        let ground_item = self
            .ground_items
            .item_at(position)
            .ok_or(PickUpError::NoItemAtPlayer)?;
        let stack = self
            .ground_items
            .get(ground_item)
            .cloned()
            .ok_or(PickUpError::NoItemAtPlayer)?;
        let maximum_stack = self
            .inventory_stack_limit(stack.item())
            .ok_or(PickUpError::UnknownItemDefinition)?;

        let mut next_inventory = self.player_inventory.clone();
        next_inventory
            .add_with_owner(
                stack.item().clone(),
                stack.quantity(),
                maximum_stack,
                stack.owner().cloned(),
            )
            .map_err(PickUpError::Inventory)?;
        let witnesses = self.property_take_witnesses(position, stack.owner());
        self.player_inventory = next_inventory;
        let _ = self.ground_items.remove(ground_item);
        self.events.push(GameEvent::ItemPickedUp {
            entity: self.player,
            ground_item,
            definition: stack.item().clone(),
            quantity: stack.quantity(),
        });
        if let Some(owner) = stack.owner() {
            for witness in witnesses {
                let visible_to_player = self
                    .actors
                    .get(witness)
                    .is_some_and(|actor| self.player_visibility.is_visible(actor.position()));
                let incident = ObservedPropertyTake {
                    turn: self.turn,
                    taker: self.player,
                    owner: owner.clone(),
                    item: stack.item().clone(),
                    quantity: stack.quantity(),
                    at: position,
                };
                let witness_actor = self
                    .actors
                    .get_mut(witness)
                    .expect("witness was selected from the actor registry");
                let alert_duration = witness_actor
                    .local_alert_profile()
                    .map(|profile| profile.duration_turns());
                witness_actor.remember_property_take(incident.clone());
                let raised_local_alert = witness_actor.raise_local_alert(incident);
                if visible_to_player {
                    self.events.push(GameEvent::PropertyTakeWitnessed {
                        taker: self.player,
                        witness,
                        owner: owner.clone(),
                        definition: stack.item().clone(),
                        quantity: stack.quantity(),
                        at: position,
                    });
                    if raised_local_alert {
                        self.events.push(GameEvent::LocalAlertRaised {
                            source: witness,
                            owner: owner.clone(),
                            at: position,
                            duration_turns: alert_duration
                                .expect("a raised local alert has a configured duration"),
                        });
                    }
                }
            }
        }
        Ok(())
    }

    fn property_take_witnesses(
        &self,
        position: GridPos,
        owner: Option<&SocialGroupId>,
    ) -> Vec<EntityId> {
        let Some(owner) = owner else {
            return Vec::new();
        };
        if self.player_may_take_property_of(owner) {
            return Vec::new();
        }
        self.actors
            .iter()
            .filter(|(entity, actor)| {
                *entity != self.player
                    && actor.affiliation() == Some(owner)
                    && actor.witness_profile().is_some_and(|profile| {
                        compute_visible_tiles(&self.map, actor.position(), profile.field_of_view())
                            .contains(&position)
                    })
            })
            .map(|(entity, _)| entity)
            .collect()
    }

    fn inventory_stack_limit(&self, item: &ItemId) -> Option<u16> {
        if self.rules.weapons.get(item).is_some() {
            Some(1)
        } else {
            self.rules
                .items
                .get(item)
                .map(|definition| definition.maximum_stack())
        }
    }

    fn drop_player_item(&mut self, item: ItemInstanceId) -> Result<(), DropItemError> {
        let position = self
            .actors
            .get(self.player)
            .map(Actor::position)
            .ok_or(DropItemError::MissingPlayer)?;
        let entry = self
            .player_inventory
            .get(item)
            .cloned()
            .ok_or(DropItemError::UnknownInventoryItem(item))?;

        // Stage every mutation first so a failed world insertion cannot remove
        // an inventory item or silently unequip it.
        let mut next_ground_items = self.ground_items.clone();
        let ground_item = next_ground_items
            .spawn_with_owner(
                position,
                entry.item().clone(),
                entry.quantity(),
                entry.owner().cloned(),
            )
            .map_err(DropItemError::Ground)?;
        let mut next_inventory = self.player_inventory.clone();
        next_inventory
            .remove(item, entry.quantity())
            .map_err(|_| DropItemError::InventoryChanged(item))?;
        let mut next_equipment = self.player_equipment.clone();
        if let Some(slot) = next_equipment.slot_of(item).cloned() {
            next_equipment.unequip(&slot);
        }

        self.ground_items = next_ground_items;
        self.player_inventory = next_inventory;
        self.player_equipment = next_equipment;
        self.events.push(GameEvent::GroundItemSpawned {
            ground_item,
            definition: entry.item().clone(),
            quantity: entry.quantity(),
            at: position,
        });
        self.events.push(GameEvent::ItemDropped {
            entity: self.player,
            item,
            ground_item,
            definition: entry.item().clone(),
            quantity: entry.quantity(),
        });
        Ok(())
    }

    fn perform_ability(
        &mut self,
        user: EntityId,
        slot: u8,
        target: GridPos,
    ) -> Result<(), AbilityError> {
        let user_state = self
            .actors
            .get(user)
            .cloned()
            .ok_or(AbilityError::MissingUser(user))?;
        if self.map.is_protected(user_state.position()) || self.map.is_protected(target) {
            return Err(AbilityError::ProtectedZone);
        }
        let ability = user_state
            .ability(slot)
            .cloned()
            .ok_or(AbilityError::MissingAbilitySlot(slot))?;

        if !self.map.contains(target) {
            return Err(AbilityError::TargetOutsideMap(target));
        }
        if ability.requires_walkable_target() && !self.map.is_walkable(target) {
            return Err(AbilityError::BlockedTarget(target));
        }
        if !ability.is_in_range(user_state.position(), target) {
            return Err(AbilityError::TargetOutOfRange(target));
        }
        if ability.requires_line_of_sight()
            && !has_line_of_sight(&self.map, user_state.position(), target, true)
        {
            return Err(AbilityError::NoLineOfSight(target));
        }
        for effect in ability.effects() {
            if let EffectPrimitive::ApplyStatus(effect) = effect {
                if !self.rules.statuses.contains(effect.status()) {
                    return Err(AbilityError::UnknownStatusDefinition);
                }
                if self.actors.entity_at(target).is_none() {
                    return Err(AbilityError::NoEntityAtTarget(target));
                }
            }
        }

        self.events
            .push(GameEvent::AbilityUsed { user, slot, target });
        for effect in ability.effects() {
            match effect {
                EffectPrimitive::RadialDamage(effect) => {
                    self.apply_radial_damage(Some(user), target, effect);
                }
                EffectPrimitive::ApplyStatus(effect) => {
                    let target_entity = self
                        .actors
                        .entity_at(target)
                        .ok_or(AbilityError::NoEntityAtTarget(target))?;
                    self.apply_status_to(Some(user), target_entity, effect)?;
                }
            }
        }

        Ok(())
    }

    fn apply_status_to(
        &mut self,
        source: Option<EntityId>,
        target: EntityId,
        effect: &ApplyStatusEffect,
    ) -> Result<(), AbilityError> {
        let definition = self
            .rules
            .statuses
            .get(effect.status())
            .cloned()
            .ok_or(AbilityError::UnknownStatusDefinition)?;
        let Some(target_actor) = self.actors.get_mut(target) else {
            // A previous primitive in the same ability may already have removed the target.
            return Ok(());
        };
        let outcome = target_actor.apply_status(&definition, effect.stacks(), source);
        self.events.push(GameEvent::StatusApplied {
            source,
            target,
            status: effect.status().clone(),
            stacks: outcome.stacks,
            remaining_turns: outcome.remaining_turns,
        });
        Ok(())
    }

    fn apply_radial_damage(
        &mut self,
        source: Option<EntityId>,
        origin: GridPos,
        effect: &RadialDamageEffect,
    ) {
        let cells = effect.affected_cells(&self.map, origin);
        let costs: std::collections::BTreeMap<GridPos, u16> = cells
            .iter()
            .map(|cell| (cell.position, cell.cost))
            .collect();
        self.events.push(GameEvent::PropagationResolved {
            source,
            origin,
            cells,
        });

        let targets: Vec<(EntityId, u16)> = self
            .actors
            .iter()
            .filter_map(|(entity, actor)| {
                costs
                    .get(&actor.position())
                    .copied()
                    .map(|cost| (entity, cost))
            })
            .collect();
        for (target, cost) in targets {
            let Some(packet) = effect.damage_at_cost(cost) else {
                continue;
            };
            let _ = self.apply_damage_to(source, target, packet);
        }
    }

    fn apply_damage_to(
        &mut self,
        source: Option<EntityId>,
        target: EntityId,
        packet: DamagePacket,
    ) -> Result<(), EntityId> {
        let position = self.actors.get(target).ok_or(target)?.position();
        if self.map.is_protected(position)
            || source
                .and_then(|id| self.actors.get(id))
                .is_some_and(|actor| self.map.is_protected(actor.position()))
        {
            return Ok(());
        }
        let (resistances, defeat_reward) = self
            .actors
            .get(target)
            .map(|actor| (actor.resistances(), actor.defeat_reward()))
            .ok_or(target)?;
        let resolved = resolve_damage(packet, resistances, self.rules.damage);
        let target_actor = self.actors.get_mut(target).ok_or(target)?;
        let applied_damage = target_actor.apply_damage(resolved.amount);
        let target_died = !target_actor.is_alive();

        self.events.push(GameEvent::DamageApplied {
            source,
            target,
            amount: applied_damage,
            damage_type: packet.damage_type,
        });

        if target_died {
            self.events.push(GameEvent::EntityDied { entity: target });
            self.actors.remove(target);
            if target == self.player {
                self.status = RunStatus::PlayerDestroyed;
            } else if source == Some(self.player)
                && let Some(reward) = defeat_reward
            {
                self.award_defeat_experience(target, reward);
            }
        }

        Ok(())
    }

    fn award_defeat_experience(&mut self, target: EntityId, reward: DefeatReward) {
        let amount = self
            .rules
            .progression
            .experience_for_defeat(self.player_progression.level(), reward);
        if amount == 0 {
            return;
        }

        self.apply_experience_award(
            &ExperienceAward::repeatable(amount),
            ExperienceSource::DefeatedEntity(target),
        );
    }

    fn apply_experience_award(&mut self, award: &ExperienceAward, source: ExperienceSource) {
        let outcome = self
            .player_progression
            .award(award, &self.rules.progression);
        if outcome.duplicate_one_time_reward || outcome.awarded_experience == 0 {
            return;
        }

        self.events.push(GameEvent::ExperienceAwarded {
            amount: outcome.awarded_experience,
            total: outcome.total,
            source,
        });
        self.events.extend(
            outcome
                .level_gains
                .into_iter()
                .map(|gain| GameEvent::LevelGained {
                    level: gain.level,
                    skill_points_awarded: gain.skill_points_awarded,
                }),
        );
    }

    pub(super) fn complete_turn(&mut self) {
        self.phase = TurnPhase::ResolvingActors;
        self.resolve_ai_turn();
        self.phase = TurnPhase::ResolvingEnvironment;
        if self.status == RunStatus::Active {
            self.resolve_status_trigger(StatusTrigger::TurnEnd);
            self.resolve_ground_effects();
            self.elapse_status_durations();
        }
        self.turn = self.turn.saturating_add(1);
        self.movement_traces
            .prune(self.turn, self.rules.movement_traces);
        self.events
            .push(GameEvent::TurnCompleted { turn: self.turn });
        self.phase = if self.status == RunStatus::Active {
            TurnPhase::AwaitingPlayer
        } else {
            TurnPhase::RunEnded
        };
    }

    fn resolve_ai_turn(&mut self) {
        let actors_to_resolve: Vec<EntityId> = self
            .actors
            .iter()
            .filter_map(|(id, actor)| (id != self.player && actor.ai().is_some()).then_some(id))
            .collect();

        for entity in actors_to_resolve {
            if self.status != RunStatus::Active {
                break;
            }

            let Some(actor) = self.actors.get(entity).cloned() else {
                continue;
            };
            let Some(profile) = actor.ai() else {
                continue;
            };
            let Some(player_position) = self.player_position() else {
                break;
            };
            let occupied_positions: BTreeSet<GridPos> = self
                .actors
                .iter()
                .filter_map(|(id, other)| (id != entity).then_some(other.position()))
                .collect();
            let action = decide_action(AiSituation {
                map: &self.map,
                actor_position: actor.position(),
                target_position: player_position,
                occupied_positions: &occupied_positions,
                profile,
                preferred_attack: actor.attack(profile.preferred_attack_slot),
            });

            match action {
                AiAction::Wait => {
                    self.events.push(GameEvent::EntityWaited { entity });
                }
                AiAction::Move(direction) => {
                    if self.move_entity(entity, direction).is_err() {
                        self.events.push(GameEvent::EntityWaited { entity });
                    }
                }
                AiAction::Attack { slot } => {
                    if self.perform_attack(entity, slot, self.player).is_err() {
                        self.events.push(GameEvent::EntityWaited { entity });
                    }
                }
            }
        }
    }

    pub(super) fn resolve_status_trigger(&mut self, trigger: StatusTrigger) {
        #[derive(Clone)]
        struct PendingStatus {
            target: EntityId,
            instance: StatusInstance,
            effects: Vec<StatusEffectPrimitive>,
        }

        let status_catalog = &self.rules.statuses;
        let pending: Vec<PendingStatus> = self
            .actors
            .iter()
            .flat_map(|(target, actor)| {
                actor.statuses().filter_map(move |instance| {
                    let definition = status_catalog.get(&instance.definition)?;
                    let effects: Vec<StatusEffectPrimitive> = definition
                        .hooks_for(trigger)
                        .flat_map(|hook| hook.effects().iter().copied())
                        .collect();
                    (!effects.is_empty()).then_some(PendingStatus {
                        target,
                        instance: instance.clone(),
                        effects,
                    })
                })
            })
            .collect();

        for pending_status in pending {
            if self.actors.get(pending_status.target).is_none() {
                continue;
            }
            self.events.push(GameEvent::StatusTriggered {
                target: pending_status.target,
                status: pending_status.instance.definition.clone(),
                trigger,
            });
            for effect in pending_status.effects {
                match effect {
                    StatusEffectPrimitive::DealDamage {
                        mut packet,
                        multiply_by_stacks,
                    } => {
                        if multiply_by_stacks {
                            packet.amount =
                                packet.amount.saturating_mul(pending_status.instance.stacks);
                        }
                        let _ = self.apply_damage_to(
                            pending_status.instance.source,
                            pending_status.target,
                            packet,
                        );
                    }
                }
            }
        }
    }

    pub(super) fn resolve_ground_effects(&mut self) {
        let ready = self.ground_effects.ready_on(self.turn);
        for effect in ready {
            let target = self.actors.entity_at(effect.position());
            self.events.push(GameEvent::GroundEffectTriggered {
                source: effect.source(),
                effect: effect.definition().clone(),
                at: effect.position(),
                target,
            });
            if let Some(target) = target {
                let _ = self.apply_damage_to(effect.source(), target, effect.damage_each_turn());
            }
            if self
                .ground_effects
                .elapse(effect.position(), effect.definition())
            {
                self.events.push(GameEvent::GroundEffectRemoved {
                    effect: effect.definition().clone(),
                    at: effect.position(),
                });
            }
        }
    }

    pub(super) fn elapse_status_durations(&mut self) {
        let active_statuses: Vec<(EntityId, StatusId)> = self
            .actors
            .iter()
            .flat_map(|(entity, actor)| {
                actor
                    .statuses()
                    .map(move |status| (entity, status.definition.clone()))
            })
            .collect();

        for (target, status) in active_statuses {
            let expired = self
                .actors
                .get_mut(target)
                .is_some_and(|actor| actor.elapse_status_turn(&status));
            if expired {
                self.events.push(GameEvent::StatusRemoved {
                    target,
                    status,
                    reason: super::StatusRemovalReason::Expired,
                });
            }
        }
    }
}

fn is_within_chebyshev_range(origin: GridPos, target: GridPos, range: u16) -> bool {
    let delta_x = (i64::from(target.x) - i64::from(origin.x)).abs();
    let delta_y = (i64::from(target.y) - i64::from(origin.y)).abs();
    delta_x.max(delta_y) <= i64::from(range)
}

fn analyzed_wall_tiles(
    map: &Map,
    visibility: &VisibilityState,
    origin: GridPos,
    radius: u16,
    maximum_tiles: u8,
) -> Vec<super::TerrainAnalysis> {
    let mut candidates: Vec<GridPos> = visibility
        .visible_positions()
        .filter(|position| is_within_chebyshev_range(origin, *position, radius))
        .filter(|position| {
            map.tile(*position)
                .is_some_and(|tile| tile.terrain.blocks_movement())
        })
        .collect();
    candidates.sort_by_key(|position| {
        let dx = (i64::from(position.x) - i64::from(origin.x)).abs();
        let dy = (i64::from(position.y) - i64::from(origin.y)).abs();
        (dx.max(dy), *position)
    });

    let mut selected = Vec::new();
    if let Some(first) = candidates.first().copied() {
        selected.push(first);
    }
    while selected.len() < usize::from(maximum_tiles) {
        let next = candidates.iter().copied().find(|candidate| {
            !selected.contains(candidate)
                && selected
                    .iter()
                    .any(|position| position.cardinal_neighbors().contains(candidate))
        });
        let Some(next) = next else {
            break;
        };
        selected.push(next);
    }

    selected
        .into_iter()
        .filter_map(|position| {
            map.tile(position).map(|tile| super::TerrainAnalysis {
                position,
                terrain: tile.terrain,
                blocks_movement: tile.terrain.blocks_movement(),
                blocks_vision: tile.terrain.blocks_vision(),
            })
        })
        .collect()
}

fn first_unknown_status(actor: &Actor, catalog: &StatusCatalog) -> Option<StatusId> {
    actor.abilities().iter().find_map(|ability| {
        ability.effects().iter().find_map(|effect| match effect {
            EffectPrimitive::ApplyStatus(effect) if !catalog.contains(effect.status()) => {
                Some(effect.status().clone())
            }
            _ => None,
        })
    })
}

fn first_unknown_weapon_status(rules: &GameRules) -> Option<(WeaponId, StatusId)> {
    rules.weapons.iter().find_map(|(weapon_id, weapon)| {
        weapon.effects().iter().find_map(|effect| match effect {
            WeaponEffect::ApplyStatus(effect) if !rules.statuses.contains(effect.status()) => {
                Some((weapon_id.clone(), effect.status().clone()))
            }
            _ => None,
        })
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunStatus {
    Active,
    PlayerDestroyed,
    Escaped,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandOutcome {
    Applied,
    AppliedWithoutTime,
    Rejected(CommandRejection),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandRejection {
    PassageUnavailable,
    PassageObstructed,
    InteractionOutOfReach,
    NothingToInteract,
    DoorLocked,
    DoorUnpowered,
    DoorObstructed,
    ControlUnavailable,
    NoMaterialForDepot,
    FacilityUnavailable,
    ProtectedZone,
    RunEnded,
    NotPlayersTurn,
    MissingPlayer,
    BlockedByTerrain(GridPos),
    Occupied(GridPos),
    UnknownTarget(EntityId),
    MissingAttackSlot(u8),
    TargetOutOfRange(EntityId),
    NoLineOfSight(EntityId),
    AttackTargetOutsideMap(GridPos),
    AttackTargetIsOrigin,
    AttackTargetOutOfRange(GridPos),
    AttackNoLineOfSight(GridPos),
    FreeAimRequiresAreaWeapon,
    MissingEquipmentSlot(u8),
    UnknownInventoryItem(ItemInstanceId),
    ItemIsNotWeapon(ItemInstanceId),
    ItemAlreadyEquippedInSlot { slot: u8, item: ItemInstanceId },
    ItemIsNotUsable(ItemInstanceId),
    ItemHasNoUsefulEffect(ItemInstanceId),
    InventoryChanged(ItemInstanceId),
    NoItemToPickUp,
    InventoryCannotFitItem,
    UnknownGroundItemDefinition,
    CannotDropItemHere,
    MissingAbilitySlot(u8),
    AbilityTargetOutsideMap(GridPos),
    AbilityTargetBlocked(GridPos),
    AbilityTargetOutOfRange(GridPos),
    AbilityNoLineOfSight(GridPos),
    AbilityTargetHasNoActor(GridPos),
    AbilityUnknownStatusDefinition,
    TechniqueLearning(Box<TechniqueLearningError>),
    InsufficientSkillPoints { required: u16, available: u32 },
    UnknownTechnique(TechniqueId),
    TechniqueNotLearned(TechniqueId),
    TechniqueHasNoActiveAction(TechniqueId),
    TechniqueMissingTarget,
    TechniqueUnexpectedTarget,
    TechniqueTargetNotVisible(EntityId),
    TechniqueTargetOutOfRange(EntityId),
    TechniqueTooManyTargets { maximum: usize, actual: usize },
    TechniqueDuplicateTarget(EntityId),
    InsufficientEnergy { required: u16, available: u16 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum TechniqueUseError {
    TooManyTargets { maximum: usize, actual: usize },
    DuplicateTarget(EntityId),
    Energy(EnergySpendError),
    MissingPlayer,
    UnknownTechnique(TechniqueId),
    NotLearned(TechniqueId),
    NoActiveAction(TechniqueId),
    MissingTarget,
    UnexpectedTarget,
    UnknownTarget(EntityId),
    TargetNotVisible(EntityId),
    TargetOutOfRange(EntityId),
}

impl From<TechniqueUseError> for CommandRejection {
    fn from(error: TechniqueUseError) -> Self {
        match error {
            TechniqueUseError::TooManyTargets { maximum, actual } => {
                Self::TechniqueTooManyTargets { maximum, actual }
            }
            TechniqueUseError::DuplicateTarget(target) => Self::TechniqueDuplicateTarget(target),
            TechniqueUseError::Energy(error) => Self::InsufficientEnergy {
                required: error.required,
                available: error.available,
            },
            TechniqueUseError::MissingPlayer => Self::MissingPlayer,
            TechniqueUseError::UnknownTechnique(technique) => Self::UnknownTechnique(technique),
            TechniqueUseError::NotLearned(technique) => Self::TechniqueNotLearned(technique),
            TechniqueUseError::NoActiveAction(technique) => {
                Self::TechniqueHasNoActiveAction(technique)
            }
            TechniqueUseError::MissingTarget => Self::TechniqueMissingTarget,
            TechniqueUseError::UnexpectedTarget => Self::TechniqueUnexpectedTarget,
            TechniqueUseError::UnknownTarget(target) => Self::UnknownTarget(target),
            TechniqueUseError::TargetNotVisible(target) => Self::TechniqueTargetNotVisible(target),
            TechniqueUseError::TargetOutOfRange(target) => Self::TechniqueTargetOutOfRange(target),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum LearnTechniqueError {
    Technique(TechniqueLearningError),
    Points(SkillPointSpendError),
}

impl From<LearnTechniqueError> for CommandRejection {
    fn from(error: LearnTechniqueError) -> Self {
        match error {
            LearnTechniqueError::Technique(error) => Self::TechniqueLearning(Box::new(error)),
            LearnTechniqueError::Points(error) => Self::InsufficientSkillPoints {
                required: error.required,
                available: error.available,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum PickUpError {
    MissingPlayer,
    NoItemAtPlayer,
    UnknownItemDefinition,
    Inventory(InventoryError),
}

impl From<PickUpError> for CommandRejection {
    fn from(error: PickUpError) -> Self {
        match error {
            PickUpError::MissingPlayer => Self::MissingPlayer,
            PickUpError::NoItemAtPlayer => Self::NoItemToPickUp,
            PickUpError::UnknownItemDefinition => Self::UnknownGroundItemDefinition,
            PickUpError::Inventory(_) => Self::InventoryCannotFitItem,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum DropItemError {
    MissingPlayer,
    UnknownInventoryItem(ItemInstanceId),
    Ground(GroundItemRegistryError),
    InventoryChanged(ItemInstanceId),
}

impl From<DropItemError> for CommandRejection {
    fn from(error: DropItemError) -> Self {
        match error {
            DropItemError::MissingPlayer => Self::MissingPlayer,
            DropItemError::UnknownInventoryItem(item) => Self::UnknownInventoryItem(item),
            DropItemError::Ground(_) => Self::CannotDropItemHere,
            DropItemError::InventoryChanged(item) => Self::InventoryChanged(item),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum UseItemError {
    MissingPlayer,
    UnknownInventoryItem(ItemInstanceId),
    ItemIsNotUsable(ItemInstanceId),
    NoUsefulEffect(ItemInstanceId),
    InventoryChanged(ItemInstanceId),
}

impl From<UseItemError> for CommandRejection {
    fn from(error: UseItemError) -> Self {
        match error {
            UseItemError::MissingPlayer => Self::MissingPlayer,
            UseItemError::UnknownInventoryItem(item) => Self::UnknownInventoryItem(item),
            UseItemError::ItemIsNotUsable(item) => Self::ItemIsNotUsable(item),
            UseItemError::NoUsefulEffect(item) => Self::ItemHasNoUsefulEffect(item),
            UseItemError::InventoryChanged(item) => Self::InventoryChanged(item),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum EquipWeaponError {
    MissingEquipmentSlot(u8),
    UnknownInventoryItem(ItemInstanceId),
    ItemIsNotWeapon(ItemInstanceId),
    AlreadyEquipped { slot: u8, item: ItemInstanceId },
    Equipment(EquipmentError),
}

impl From<EquipWeaponError> for CommandRejection {
    fn from(error: EquipWeaponError) -> Self {
        match error {
            EquipWeaponError::MissingEquipmentSlot(slot) => Self::MissingEquipmentSlot(slot),
            EquipWeaponError::UnknownInventoryItem(item) => Self::UnknownInventoryItem(item),
            EquipWeaponError::ItemIsNotWeapon(item) => Self::ItemIsNotWeapon(item),
            EquipWeaponError::AlreadyEquipped { slot, item } => {
                Self::ItemAlreadyEquippedInSlot { slot, item }
            }
            EquipWeaponError::Equipment(EquipmentError::ItemNotInInventory(item)) => {
                Self::UnknownInventoryItem(item)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum MovementError {
    MissingEntity(EntityId),
    BlockedByTerrain(GridPos),
    Occupied(GridPos),
}

impl From<MovementError> for CommandRejection {
    fn from(error: MovementError) -> Self {
        match error {
            MovementError::MissingEntity(_) => Self::MissingPlayer,
            MovementError::BlockedByTerrain(position) => Self::BlockedByTerrain(position),
            MovementError::Occupied(position) => Self::Occupied(position),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AttackError {
    ProtectedZone,
    MissingAttacker(EntityId),
    UnknownTarget(EntityId),
    MissingAttackSlot(u8),
    TargetOutOfRange(EntityId),
    NoLineOfSight(EntityId),
    TargetOutsideMap(GridPos),
    TargetIsOrigin,
    PositionOutOfRange(GridPos),
    NoLineOfSightAt(GridPos),
    FreeAimRequiresAreaWeapon,
}

struct PreparedAttack {
    attacker: EntityId,
    target: Option<EntityId>,
    slot: u8,
    origin: GridPos,
    target_at: GridPos,
    attack: AttackProfile,
    weapon: Option<WeaponId>,
    weapon_effects: Vec<WeaponEffect>,
    affected_cells: Vec<AttackAreaCell>,
}

impl From<AttackError> for CommandRejection {
    fn from(error: AttackError) -> Self {
        match error {
            AttackError::ProtectedZone => Self::ProtectedZone,
            AttackError::MissingAttacker(_) => Self::MissingPlayer,
            AttackError::UnknownTarget(target) => Self::UnknownTarget(target),
            AttackError::MissingAttackSlot(slot) => Self::MissingAttackSlot(slot),
            AttackError::TargetOutOfRange(target) => Self::TargetOutOfRange(target),
            AttackError::NoLineOfSight(target) => Self::NoLineOfSight(target),
            AttackError::TargetOutsideMap(target) => Self::AttackTargetOutsideMap(target),
            AttackError::TargetIsOrigin => Self::AttackTargetIsOrigin,
            AttackError::PositionOutOfRange(target) => Self::AttackTargetOutOfRange(target),
            AttackError::NoLineOfSightAt(target) => Self::AttackNoLineOfSight(target),
            AttackError::FreeAimRequiresAreaWeapon => Self::FreeAimRequiresAreaWeapon,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AbilityError {
    ProtectedZone,
    MissingUser(EntityId),
    MissingAbilitySlot(u8),
    TargetOutsideMap(GridPos),
    BlockedTarget(GridPos),
    TargetOutOfRange(GridPos),
    NoLineOfSight(GridPos),
    NoEntityAtTarget(GridPos),
    UnknownStatusDefinition,
}

impl From<AbilityError> for CommandRejection {
    fn from(error: AbilityError) -> Self {
        match error {
            AbilityError::ProtectedZone => Self::ProtectedZone,
            AbilityError::MissingUser(_) => Self::MissingPlayer,
            AbilityError::MissingAbilitySlot(slot) => Self::MissingAbilitySlot(slot),
            AbilityError::TargetOutsideMap(target) => Self::AbilityTargetOutsideMap(target),
            AbilityError::BlockedTarget(target) => Self::AbilityTargetBlocked(target),
            AbilityError::TargetOutOfRange(target) => Self::AbilityTargetOutOfRange(target),
            AbilityError::NoLineOfSight(target) => Self::AbilityNoLineOfSight(target),
            AbilityError::NoEntityAtTarget(target) => Self::AbilityTargetHasNoActor(target),
            AbilityError::UnknownStatusDefinition => Self::AbilityUnknownStatusDefinition,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpawnError {
    BlockedByTerrain(GridPos),
    Occupied(GridPos),
    Registry(RegistryError),
    UnknownStatusDefinition(StatusId),
    InvalidPrimaryAttributes(PrimaryAttributesError),
}

impl Display for SpawnError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BlockedByTerrain(position) => write!(
                formatter,
                "spawn position ({}, {}) is blocked or outside the map",
                position.x, position.y
            ),
            Self::Occupied(position) => write!(
                formatter,
                "spawn position ({}, {}) is already occupied",
                position.x, position.y
            ),
            Self::Registry(error) => write!(formatter, "could not register actor: {error}"),
            Self::UnknownStatusDefinition(status) => write!(
                formatter,
                "actor references unknown status definition '{}'",
                status.as_str()
            ),
            Self::InvalidPrimaryAttributes(error) => {
                write!(formatter, "actor has invalid primary attributes: {error}")
            }
        }
    }
}

impl Error for SpawnError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GroundItemSpawnError {
    BlockedByTerrain(GridPos),
    UnknownItemDefinition(ItemId),
    Registry(GroundItemRegistryError),
}

impl Display for GroundItemSpawnError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BlockedByTerrain(position) => write!(
                formatter,
                "ground item position ({}, {}) is blocked or outside the map",
                position.x, position.y
            ),
            Self::UnknownItemDefinition(item) => {
                write!(formatter, "unknown ground item definition '{item}'")
            }
            Self::Registry(error) => write!(formatter, "could not register ground item: {error}"),
        }
    }
}

impl Error for GroundItemSpawnError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameInitError {
    Energy(EnergyReserveError),
    BlockedPlayerStart(GridPos),
    BlockedExit(GridPos),
    Actor(ActorBuildError),
    Registry(RegistryError),
    ProgressionRules(ProgressionRulesError),
    PlayerAttributes(PrimaryAttributesError),
    SkillCatalog(SkillCatalogError),
    MovementTraceRules(MovementTraceRulesError),
    UnknownStatusDefinition(StatusId),
    UnknownWeaponStatusDefinition {
        weapon: Box<WeaponId>,
        status: StatusId,
    },
    DuplicateEquipmentSlot(EquipmentSlotId),
    UnknownStartingWeapon(WeaponId),
    UnknownStartingItem(ItemId),
    TooManyStartingEquipmentEntries {
        equipment: usize,
        slots: usize,
    },
    StartingEquipmentWeaponNotInInventory(WeaponId),
    Inventory(InventoryError),
    Equipment(EquipmentError),
}

impl Display for GameInitError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Energy(error) => write!(formatter, "invalid player energy: {error}"),
            Self::BlockedPlayerStart(position) => write!(
                formatter,
                "player start ({}, {}) is blocked or outside the map",
                position.x, position.y
            ),
            Self::BlockedExit(position) => write!(
                formatter,
                "exit ({}, {}) is blocked or outside the map",
                position.x, position.y
            ),
            Self::Actor(error) => write!(formatter, "could not build player actor: {error}"),
            Self::Registry(error) => write!(formatter, "could not create player actor: {error}"),
            Self::ProgressionRules(error) => {
                write!(formatter, "invalid progression rules: {error}")
            }
            Self::PlayerAttributes(error) => {
                write!(formatter, "invalid player primary attributes: {error}")
            }
            Self::SkillCatalog(error) => write!(formatter, "invalid skill catalog: {error}"),
            Self::MovementTraceRules(error) => {
                write!(formatter, "invalid movement trace rules: {error}")
            }
            Self::UnknownStatusDefinition(status) => write!(
                formatter,
                "player ability references unknown status definition '{}'",
                status.as_str()
            ),
            Self::UnknownWeaponStatusDefinition { weapon, status } => write!(
                formatter,
                "weapon '{weapon}' references unknown status definition '{status}'"
            ),
            Self::DuplicateEquipmentSlot(slot) => {
                write!(formatter, "duplicate player equipment slot '{slot}'")
            }
            Self::UnknownStartingWeapon(weapon) => {
                write!(formatter, "unknown player starting weapon '{weapon}'")
            }
            Self::UnknownStartingItem(item) => {
                write!(formatter, "unknown player starting item '{item}'")
            }
            Self::TooManyStartingEquipmentEntries { equipment, slots } => write!(
                formatter,
                "player has {equipment} starting equipment entries but only {slots} slots"
            ),
            Self::StartingEquipmentWeaponNotInInventory(weapon) => write!(
                formatter,
                "starting equipment weapon '{weapon}' is not available as a distinct inventory instance"
            ),
            Self::Inventory(error) => {
                write!(formatter, "could not build player inventory: {error}")
            }
            Self::Equipment(error) => {
                write!(formatter, "could not build player equipment: {error}")
            }
        }
    }
}

impl Error for GameInitError {}

#[cfg(test)]
mod interaction_tests {
    use super::*;
    use crate::ai::AiProfile;
    use crate::combat::{AttackProfile, DamageType};

    fn door_game(state: DoorState) -> GameState {
        let mut map =
            Map::from_ascii("#########\n#...#...#\n#...#...#\n#...#...#\n#########").unwrap();
        map.set_terrain(GridPos::new(4, 2), Terrain::Door(state))
            .unwrap();
        GameState::new(map, GridPos::new(3, 2), 42).unwrap()
    }

    #[test]
    fn doors_change_real_movement_los_and_turns() {
        let mut game = door_game(DoorState::Closed);
        let door = GridPos::new(4, 2);
        let beyond = GridPos::new(5, 2);
        assert!(!game.map.is_walkable(door));
        assert!(!game.player_visibility.is_visible(beyond));
        assert_eq!(
            game.process_player_command(GameCommand::Interact { target: door }),
            CommandOutcome::Applied
        );
        assert_eq!(game.turn(), 1);
        assert!(game.map.is_walkable(door));
        assert!(game.player_visibility.is_visible(beyond));
        assert_eq!(
            game.process_player_command(GameCommand::Interact { target: door }),
            CommandOutcome::Applied
        );
        assert_eq!(game.turn(), 2);
        assert!(!game.player_visibility.is_visible(beyond));
        assert!(game.player_visibility.is_explored(beyond));
    }

    #[test]
    fn locked_remote_and_repeated_interactions_are_atomic() {
        let mut game = door_game(DoorState::Locked);
        let door = GridPos::new(4, 2);
        let control = GridPos::new(3, 1);
        game.map
            .set_terrain(
                control,
                Terrain::ControlPanel {
                    door,
                    activated: false,
                },
            )
            .unwrap();
        for (target, reason) in [
            (door, CommandRejection::DoorLocked),
            (GridPos::new(7, 2), CommandRejection::InteractionOutOfReach),
            (GridPos::new(2, 2), CommandRejection::NothingToInteract),
        ] {
            let before = format!("{game:?}");
            assert_eq!(
                game.process_player_command(GameCommand::Interact { target }),
                CommandOutcome::Rejected(reason)
            );
            assert_eq!(format!("{game:?}"), before);
        }
        assert_eq!(
            game.process_player_command(GameCommand::Interact { target: control }),
            CommandOutcome::Applied
        );
        assert_eq!(
            game.map.tile(door).unwrap().terrain,
            Terrain::Door(DoorState::Closed)
        );
        assert!(!game.map.is_walkable(door)); // Unlocking is not opening.
        let before = format!("{game:?}");
        assert_eq!(
            game.process_player_command(GameCommand::Interact { target: control }),
            CommandOutcome::Rejected(CommandRejection::ControlUnavailable)
        );
        assert_eq!(format!("{game:?}"), before);
        assert_eq!(
            game.process_player_command(GameCommand::Interact { target: door }),
            CommandOutcome::Applied
        );
    }

    #[test]
    fn door_cannot_close_on_an_actor_or_ground_item() {
        let mut game = door_game(DoorState::Open);
        let door = GridPos::new(4, 2);
        let id = game.spawn_actor(Actor::new(door, 5).unwrap()).unwrap();
        let before = format!("{game:?}");
        assert_eq!(
            game.process_player_command(GameCommand::Interact { target: door }),
            CommandOutcome::Rejected(CommandRejection::DoorObstructed)
        );
        assert_eq!(format!("{game:?}"), before);
        game.actors.remove(id);
        game.ground_items
            .spawn(door, "core:test".parse().unwrap(), 1)
            .unwrap();
        assert_eq!(
            game.process_player_command(GameCommand::Interact { target: door }),
            CommandOutcome::Rejected(CommandRejection::DoorObstructed)
        );
    }

    #[test]
    fn protected_ground_blocks_hostiles_attacks_abilities_and_environmental_damage() {
        let mut map = Map::from_ascii("#######\n#.....#\n#######").unwrap();
        let origin = GridPos::new(2, 1);
        map.set_protected(origin, true).unwrap();
        let mut game = GameState::new(map, origin, 1).unwrap();
        let enemy = game
            .spawn_actor(
                Actor::new(GridPos::new(3, 1), 20)
                    .unwrap()
                    .with_attack(AttackProfile::melee(DamageType::Kinetic, 9))
                    .with_ai(AiProfile::hunter(8, 0)),
            )
            .unwrap();
        assert!(game.move_entity(enemy, Direction::West).is_err());
        for command in [
            GameCommand::Attack {
                slot: 0,
                target: enemy,
            },
            GameCommand::UseAbility {
                slot: 0,
                target: GridPos::new(3, 1),
            },
        ] {
            let before = format!("{game:?}");
            assert_eq!(
                game.process_player_command(command),
                CommandOutcome::Rejected(CommandRejection::ProtectedZone)
            );
            assert_eq!(format!("{game:?}"), before);
        }
        assert_eq!(
            game.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), 20);
        game.apply_damage_to(
            None,
            game.player,
            DamagePacket::new(100, DamageType::Thermal, 0),
        )
        .unwrap();
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), 20);
        game.process_player_command(GameCommand::Move(Direction::West));
        assert!(!game.map.is_protected(game.player_position().unwrap()));
        game.apply_damage_to(
            None,
            game.player,
            DamagePacket::new(1, DamageType::Thermal, 0),
        )
        .unwrap();
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), 19);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use semver::Version;

    use super::*;
    use crate::ai::AiProfile;
    use crate::combat::{AttackProfile, DamageType, ResistanceProfile};
    use crate::content::ContentLoader;
    use crate::effects::{AbilityProfile, ApplyStatusEffect};
    use crate::item::{ItemCatalog, ItemDefinition};
    use crate::progression::{DefeatReward, ExperienceCurve, ProgressionRules};
    use crate::skills::{SkillCatalog, SystemFeatureSet};
    use crate::social::{LocalAlertProfile, WitnessProfile};
    use crate::status::{StatusDefinition, StatusHook, StatusStacking};
    use crate::world::{DistanceMetric, FieldOfViewRules};

    fn parse_map(definition: &str) -> Map {
        match Map::from_ascii(definition) {
            Ok(map) => map,
            Err(error) => panic!("valid test map failed to parse: {error}"),
        }
    }

    fn small_game() -> GameState {
        match GameState::new(parse_map("#####\n#...#\n#####"), GridPos::new(2, 1), 1234) {
            Ok(game) => game,
            Err(error) => panic!("valid test game failed to initialize: {error}"),
        }
    }

    fn rules_with_reconnaissance(progression: ProgressionRules) -> GameRules {
        let content_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content");
        let content = ContentLoader::load(&[content_root], &Version::new(0, 1, 0))
            .unwrap_or_else(|error| panic!("core content failed to load: {error}"));
        GameRules {
            progression,
            skills: content.skills().clone(),
            enabled_system_features: SystemFeatureSet::new(["core:traces"
                .parse()
                .unwrap_or_else(|error| panic!("valid feature ID rejected: {error}"))]),
            ..GameRules::default()
        }
    }

    fn build_actor(position: GridPos, integrity: u16) -> Actor {
        Actor::new(position, integrity)
            .unwrap_or_else(|error| panic!("valid actor failed to build: {error}"))
    }

    fn corrosion_rules() -> (GameRules, StatusId) {
        let status: StatusId = "core:corroded"
            .parse()
            .unwrap_or_else(|error| panic!("valid status ID rejected: {error}"));
        let definition = StatusDefinition::new(
            status.clone(),
            Some(3),
            StatusStacking::AddStacks {
                maximum_stacks: 3,
                refresh_duration: true,
            },
            vec![StatusHook::new(
                StatusTrigger::TurnEnd,
                vec![StatusEffectPrimitive::DealDamage {
                    packet: DamagePacket::new(2, DamageType::Chemical, 0),
                    multiply_by_stacks: true,
                }],
            )],
        )
        .unwrap_or_else(|error| panic!("valid status definition rejected: {error}"));
        let application = ApplyStatusEffect::new(status.clone(), 1)
            .unwrap_or_else(|error| panic!("valid status application rejected: {error}"));
        let mut rules = GameRules {
            player_base_abilities: vec![AbilityProfile::new(
                5,
                DistanceMetric::Chebyshev,
                true,
                true,
                vec![EffectPrimitive::ApplyStatus(application)],
            )],
            ..GameRules::default()
        };
        rules
            .statuses
            .register(definition)
            .unwrap_or_else(|error| panic!("valid status registration rejected: {error}"));
        (rules, status)
    }

    fn equipment_rules() -> (GameRules, WeaponId, WeaponId) {
        let primary: EquipmentSlotId = "core:primary_channel"
            .parse()
            .unwrap_or_else(|error| panic!("valid slot ID rejected: {error}"));
        let blade: WeaponId = "core:test_blade"
            .parse()
            .unwrap_or_else(|error| panic!("valid weapon ID rejected: {error}"));
        let arc: WeaponId = "test.mod:arc_tool"
            .parse()
            .unwrap_or_else(|error| panic!("valid weapon ID rejected: {error}"));
        let mut weapons = crate::weapon::WeaponCatalog::default();
        weapons
            .register(
                WeaponDefinition::new(
                    blade.clone(),
                    "weapon.test_blade.name".to_owned(),
                    "weapon.test_blade.description".to_owned(),
                    AttackProfile::melee(DamageType::Kinetic, 2),
                )
                .unwrap_or_else(|error| panic!("valid weapon rejected: {error}")),
            )
            .unwrap_or_else(|error| panic!("valid weapon registration rejected: {error}"));
        weapons
            .register(
                WeaponDefinition::new(
                    arc.clone(),
                    "weapon.arc_tool.name".to_owned(),
                    "weapon.arc_tool.description".to_owned(),
                    AttackProfile::melee(DamageType::Electrical, 6),
                )
                .unwrap_or_else(|error| panic!("valid weapon rejected: {error}")),
            )
            .unwrap_or_else(|error| panic!("valid weapon registration rejected: {error}"));

        (
            GameRules {
                player_inventory_capacity: 4,
                player_weapon_slots: vec![primary],
                player_starting_weapons: vec![blade.clone(), arc.clone()],
                player_starting_equipment: vec![Some(blade.clone())],
                weapons,
                ..GameRules::default()
            },
            blade,
            arc,
        )
    }

    fn flamethrower_rules() -> (GameRules, WeaponId) {
        let content_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content");
        let loaded = ContentLoader::load(&[content_root], &Version::new(0, 1, 0))
            .unwrap_or_else(|error| panic!("core content failed to load: {error}"));
        let weapon: WeaponId = "core:flamethrower".parse().unwrap();
        let slot: EquipmentSlotId = "core:flame_channel".parse().unwrap();
        let rules = GameRules {
            player_base_attacks: vec![loaded.weapons().get(&weapon).unwrap().attack()],
            player_weapon_slots: vec![slot],
            player_starting_weapons: vec![weapon.clone()],
            player_starting_equipment: vec![Some(weapon.clone())],
            statuses: loaded.statuses().clone(),
            weapons: loaded.weapons().clone(),
            ..GameRules::default()
        };
        (rules, weapon)
    }

    fn consumable_rules() -> (GameRules, ItemId) {
        let repair: ItemId = "core:repair_patch"
            .parse()
            .unwrap_or_else(|error| panic!("valid item ID rejected: {error}"));
        let mut items = ItemCatalog::default();
        items
            .register(
                ItemDefinition::new(
                    repair.clone(),
                    "item.repair_patch.name".to_owned(),
                    "item.repair_patch.description".to_owned(),
                    3,
                    ItemKind::Consumable,
                    vec![ItemEffect::RestoreIntegrity { amount: 6 }],
                )
                .unwrap_or_else(|error| panic!("valid item rejected: {error}")),
            )
            .unwrap_or_else(|error| panic!("valid item registration rejected: {error}"));
        (
            GameRules {
                player_starting_items: vec![super::super::StartingItemStack::new(
                    repair.clone(),
                    2,
                )],
                items,
                ..GameRules::default()
            },
            repair,
        )
    }

    fn material_rules() -> (GameRules, ItemId) {
        let material: ItemId = "core:test_component".parse().unwrap();
        let mut items = ItemCatalog::default();
        items
            .register(
                ItemDefinition::new(
                    material.clone(),
                    "item.test_component.name".to_owned(),
                    "item.test_component.description".to_owned(),
                    4,
                    ItemKind::Material,
                    vec![],
                )
                .unwrap(),
            )
            .unwrap();
        (
            GameRules {
                player_inventory_capacity: 4,
                items,
                ..GameRules::default()
            },
            material,
        )
    }

    #[test]
    fn successful_movement_advances_turn_and_emits_ordered_events() {
        let mut game = small_game();

        let outcome = game.process_player_command(GameCommand::Move(Direction::East));

        assert_eq!(outcome, CommandOutcome::Applied);
        assert_eq!(game.player_position(), Some(GridPos::new(3, 1)));
        assert_eq!(game.turn(), 1);
        assert_eq!(game.phase(), TurnPhase::AwaitingPlayer);
        assert_eq!(
            game.events(),
            &[
                GameEvent::EntityMoved {
                    entity: game.player_id(),
                    from: GridPos::new(2, 1),
                    to: GridPos::new(3, 1),
                },
                GameEvent::VisibilityUpdated {
                    observer: game.player_id(),
                    origin: GridPos::new(3, 1),
                },
                GameEvent::TurnCompleted { turn: 1 },
            ]
        );
    }

    #[test]
    fn blocked_movement_does_not_advance_the_turn() {
        let mut game = small_game();

        let outcome = game.process_player_command(GameCommand::Move(Direction::North));

        assert_eq!(
            outcome,
            CommandOutcome::Rejected(CommandRejection::BlockedByTerrain(GridPos::new(2, 0)))
        );
        assert_eq!(game.player_position(), Some(GridPos::new(2, 1)));
        assert_eq!(game.turn(), 0);
        assert!(game.events().is_empty());
    }

    #[test]
    fn waiting_advances_the_turn_and_events_can_be_drained() {
        let mut game = small_game();

        assert_eq!(
            game.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(game.turn(), 1);
        assert_eq!(game.drain_events().len(), 2);
        assert!(game.events().is_empty());
    }

    #[test]
    fn starting_weapons_are_real_inventory_instances_and_only_available_slots_are_equipped() {
        let (rules, blade, arc) = equipment_rules();
        let game = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        )
        .unwrap_or_else(|error| panic!("valid equipped game rejected: {error}"));

        assert_eq!(game.player_inventory().len(), 2);
        assert_eq!(
            game.equipped_player_weapon(0).map(WeaponDefinition::id),
            Some(&blade)
        );
        assert!(
            game.player_inventory()
                .iter()
                .any(|entry| entry.item() == &arc
                    && game.player_equipment().slot_of(entry.instance()).is_none())
        );
    }

    #[test]
    fn equipment_command_switches_catalog_attack_and_costs_one_turn() {
        let (rules, _, arc) = equipment_rules();
        let mut game = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        )
        .unwrap_or_else(|error| panic!("valid equipped game rejected: {error}"));
        let arc_item = game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &arc)
            .map(|entry| entry.instance())
            .unwrap_or_else(|| panic!("arc weapon missing from inventory"));

        assert_eq!(
            game.process_player_command(GameCommand::EquipWeapon {
                slot: 0,
                item: arc_item,
            }),
            CommandOutcome::Applied
        );

        assert_eq!(game.turn(), 1);
        assert_eq!(
            game.equipped_player_weapon(0).map(WeaponDefinition::id),
            Some(&arc)
        );
        assert!(game.events().iter().any(|event| matches!(
            event,
            GameEvent::WeaponEquipped { slot: 0, item, weapon, .. }
                if *item == arc_item && weapon == &arc
        )));

        let target = game
            .spawn_actor(build_actor(GridPos::new(2, 1), 10))
            .unwrap_or_else(|error| panic!("valid target failed to spawn: {error}"));
        game.drain_events();
        assert_eq!(
            game.process_player_command(GameCommand::Attack { slot: 0, target }),
            CommandOutcome::Applied
        );
        assert_eq!(game.actors().get(target).map(Actor::integrity), Some(4));
    }

    #[test]
    fn flamethrower_hits_its_cone_burns_targets_and_leaves_expiring_ground_fire() {
        let (rules, weapon) = flamethrower_rules();
        let mut map = parse_map(
            "############\n#..........#\n#..........#\n#..........#\n#..........#\n#..........#\n############",
        );
        map.set_protected(GridPos::new(4, 4), true).unwrap();
        let mut game = GameState::new_with_rules(map, GridPos::new(1, 3), 7, rules).unwrap();
        let center = game
            .spawn_actor(build_actor(GridPos::new(6, 3), 30))
            .unwrap();
        let side = game
            .spawn_actor(build_actor(GridPos::new(4, 2), 30))
            .unwrap();
        let other_side = game
            .spawn_actor(build_actor(GridPos::new(5, 4), 30))
            .unwrap();
        let sheltered = game
            .spawn_actor(build_actor(GridPos::new(4, 4), 30))
            .unwrap();
        let outside = game
            .spawn_actor(build_actor(GridPos::new(4, 5), 30))
            .unwrap();
        game.drain_events();

        assert_eq!(
            game.process_player_command(GameCommand::Attack {
                slot: 0,
                target: center,
            }),
            CommandOutcome::Applied
        );

        for target in [center, side, other_side] {
            assert_eq!(game.actors().get(target).map(Actor::integrity), Some(24));
            assert!(game.actors().get(target).unwrap().statuses().any(|status| {
                status.definition.as_str() == "core:burning" && status.remaining_turns == Some(2)
            }));
        }
        assert_eq!(game.actors().get(sheltered).map(Actor::integrity), Some(30));
        assert!(
            game.actors()
                .get(sheltered)
                .unwrap()
                .statuses()
                .next()
                .is_none()
        );
        assert_eq!(game.actors().get(outside).map(Actor::integrity), Some(30));
        assert_eq!(game.ground_effects().iter().count(), 20);
        assert!(game.events().iter().any(|event| matches!(
            event,
            GameEvent::AttackPerformed {
                weapon: Some(id),
                affected_cells,
                ..
            } if id == &weapon
                && affected_cells.iter().filter(|cell| cell.position.x == 2).count() == 1
                && affected_cells.iter().filter(|cell| cell.position.x == 4).count() == 3
                && affected_cells.iter().filter(|cell| cell.position.x == 7).count() == 5
        )));

        game.move_entity(side, Direction::North).unwrap();
        assert_eq!(
            game.actors().get(side).map(Actor::position),
            Some(GridPos::new(4, 1))
        );
        assert!(
            game.ground_effects()
                .at(GridPos::new(4, 1))
                .next()
                .is_none()
        );
        game.drain_events();
        for turn in 0..3 {
            assert_eq!(
                game.process_player_command(GameCommand::Wait),
                CommandOutcome::Applied
            );
            if turn == 0 {
                assert_eq!(game.actors().get(side).map(Actor::integrity), Some(22));
                assert!(game.actors().get(side).unwrap().statuses().next().is_some());
            }
            game.drain_events();
        }
        for target in [center, other_side] {
            assert_eq!(game.actors().get(target).map(Actor::integrity), Some(17));
            assert!(
                game.actors()
                    .get(target)
                    .unwrap()
                    .statuses()
                    .next()
                    .is_none()
            );
        }
        assert_eq!(game.actors().get(side).map(Actor::integrity), Some(20));
        assert!(game.actors().get(side).unwrap().statuses().next().is_none());
        assert!(game.ground_effects().is_empty());
        assert_eq!(game.actors().get(outside).map(Actor::integrity), Some(30));
    }

    #[test]
    fn area_attack_preview_is_non_mutating_and_execution_uses_the_exact_same_cells() {
        let (rules, weapon) = flamethrower_rules();
        let mut game = GameState::new_with_rules(
            parse_map(
                "############\n#..........#\n#..........#\n#..........#\n#..........#\n#..........#\n############",
            ),
            GridPos::new(1, 3),
            7,
            rules,
        )
        .unwrap();
        let side = game
            .spawn_actor(build_actor(GridPos::new(4, 2), 30))
            .unwrap();
        game.drain_events();
        let aimed_at = GridPos::new(6, 3);

        let preview = game.player_attack_preview(0, aimed_at).unwrap();

        assert_eq!(preview.origin(), GridPos::new(1, 3));
        assert_eq!(preview.target(), aimed_at);
        assert!(
            preview
                .cells()
                .iter()
                .any(|cell| { cell.position == GridPos::new(2, 3) && cell.step == 0 })
        );
        assert!(
            preview
                .cells()
                .iter()
                .all(|cell| cell.position != preview.origin())
        );
        assert_eq!(game.turn(), 0);
        assert!(game.events().is_empty());

        assert_eq!(
            game.process_player_command(GameCommand::AttackAt {
                slot: 0,
                target: aimed_at,
            }),
            CommandOutcome::Applied
        );
        assert_eq!(game.actors().get(side).map(Actor::integrity), Some(24));
        assert!(game.events().iter().any(|event| matches!(
            event,
            GameEvent::AttackPerformed {
                target: None,
                target_at,
                weapon: Some(id),
                affected_cells,
                ..
            } if *target_at == aimed_at && id == &weapon && affected_cells == preview.cells()
        )));
    }

    #[test]
    fn invalid_protected_aim_still_exposes_its_complete_geometric_footprint() {
        let (rules, _) = flamethrower_rules();
        let origin = GridPos::new(1, 3);
        let target = GridPos::new(6, 3);
        let mut map = parse_map(
            "############\n#..........#\n#..........#\n#..........#\n#..........#\n#..........#\n############",
        );
        map.set_protected(origin, true).unwrap();
        let game = GameState::new_with_rules(map, origin, 7, rules).unwrap();

        let footprint = game.player_attack_footprint(0, target).unwrap();

        assert!(footprint.cells().len() > 1);
        assert!(footprint.cells().iter().any(|cell| {
            cell.position == origin.step(crate::world::Direction::East) && cell.step == 0
        }));
        assert_eq!(
            game.player_attack_preview(0, target),
            Err(CommandRejection::ProtectedZone)
        );
        assert_eq!(game.turn(), 0);
        assert!(game.events().is_empty());
    }

    #[test]
    fn free_tile_preview_rejects_single_target_attacks_without_mutation() {
        let game = small_game();

        assert_eq!(
            game.player_attack_preview(0, GridPos::new(3, 1)),
            Err(CommandRejection::FreeAimRequiresAreaWeapon)
        );
        assert_eq!(game.turn(), 0);
        assert!(game.events().is_empty());
    }

    #[test]
    fn a_weapon_cannot_reference_an_unknown_status_at_runtime_startup() {
        let weapon_id: WeaponId = "core:invalid_status_weapon".parse().unwrap();
        let status: StatusId = "missing.mod:burning".parse().unwrap();
        let weapon = WeaponDefinition::new(
            weapon_id.clone(),
            "weapon.invalid.name".into(),
            "weapon.invalid.description".into(),
            AttackProfile::melee(DamageType::Thermal, 1),
        )
        .unwrap()
        .with_effects([WeaponEffect::ApplyStatus(
            ApplyStatusEffect::new(status.clone(), 1).unwrap(),
        )]);
        let mut rules = GameRules::default();
        rules.weapons.register(weapon).unwrap();

        assert!(matches!(
            GameState::new_with_rules(
                parse_map("#####\n#...#\n#####"),
                GridPos::new(1, 1),
                1,
                rules,
            ),
            Err(GameInitError::UnknownWeaponStatusDefinition {
                weapon,
                status: missing,
            }) if *weapon == weapon_id && missing == status
        ));
    }

    #[test]
    fn unknown_starting_weapon_is_rejected_before_the_run() {
        let unknown: WeaponId = "missing.mod:ghost_weapon"
            .parse()
            .unwrap_or_else(|error| panic!("valid weapon ID rejected: {error}"));
        let rules = GameRules {
            player_starting_weapons: vec![unknown.clone()],
            ..GameRules::default()
        };

        let result = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        );

        assert!(matches!(
            result,
            Err(GameInitError::UnknownStartingWeapon(weapon)) if weapon == unknown
        ));
    }

    #[test]
    fn consumable_restores_integrity_and_removes_one_item() {
        let (rules, repair) = consumable_rules();
        let mut game = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        )
        .unwrap_or_else(|error| panic!("valid game rejected: {error}"));
        let player = game.player_id();
        game.actors
            .get_mut(player)
            .unwrap_or_else(|| panic!("player missing"))
            .apply_damage(8);
        let repair_instance = game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &repair)
            .map(|entry| entry.instance())
            .unwrap_or_else(|| panic!("repair item missing from inventory"));

        assert_eq!(
            game.process_player_command(GameCommand::UseItem {
                item: repair_instance,
            }),
            CommandOutcome::Applied
        );

        assert_eq!(game.turn(), 1);
        assert_eq!(game.actors().get(player).map(Actor::integrity), Some(18));
        assert_eq!(
            game.player_inventory()
                .get(repair_instance)
                .map(|entry| entry.quantity()),
            Some(1)
        );
        assert!(game.events().contains(&GameEvent::ItemUsed {
            entity: player,
            item: repair_instance,
            definition: repair,
        }));
        assert!(game.events().contains(&GameEvent::IntegrityRestored {
            entity: player,
            amount: 6,
        }));
    }

    #[test]
    fn owned_pickup_is_remembered_only_by_observers_who_can_see_it() {
        let (rules, material) = material_rules();
        let mut game = GameState::new_with_rules(
            parse_map("################\n#..............#\n################"),
            GridPos::new(1, 1),
            1,
            rules,
        )
        .unwrap();
        let owner: SocialGroupId = "core:maintainers".parse().unwrap();
        let profile = WitnessProfile::new(12, DistanceMetric::Euclidean, true, 4).unwrap();
        let alert_profile = LocalAlertProfile::new(8).unwrap();
        let visible_witness = game
            .spawn_actor(
                build_actor(GridPos::new(3, 1), 10)
                    .with_affiliation(owner.clone())
                    .with_witness_profile(profile)
                    .with_local_alert_profile(alert_profile),
            )
            .unwrap();
        let hidden_witness = game
            .spawn_actor(
                build_actor(GridPos::new(11, 1), 10)
                    .with_affiliation(owner.clone())
                    .with_witness_profile(profile)
                    .with_local_alert_profile(alert_profile),
            )
            .unwrap();
        let unaware_actor = game
            .spawn_actor(
                build_actor(GridPos::new(14, 1), 10)
                    .with_affiliation(owner.clone())
                    .with_witness_profile(profile),
            )
            .unwrap();
        game.spawn_ground_item_with_owner(
            GridPos::new(1, 1),
            material.clone(),
            1,
            Some(owner.clone()),
        )
        .unwrap();
        game.drain_events();

        assert_eq!(
            game.process_player_command(GameCommand::PickUp),
            CommandOutcome::Applied
        );
        for witness in [visible_witness, hidden_witness] {
            let memory = game
                .actors()
                .get(witness)
                .unwrap()
                .observed_property_takes();
            assert_eq!(memory.len(), 1);
            assert_eq!(memory[0].taker, game.player_id());
            assert_eq!(memory[0].owner, owner);
            assert_eq!(memory[0].item, material);
        }
        assert!(
            game.actors()
                .get(unaware_actor)
                .unwrap()
                .observed_property_takes()
                .is_empty()
        );
        let reported: Vec<_> = game
            .events()
            .iter()
            .filter_map(|event| match event {
                GameEvent::PropertyTakeWitnessed { witness, .. } => Some(*witness),
                _ => None,
            })
            .collect();
        assert_eq!(reported, vec![visible_witness]);
        for witness in [visible_witness, hidden_witness] {
            let alert = game.actors().get(witness).unwrap().local_alert().unwrap();
            assert!(alert.is_active(game.turn()));
            assert_eq!(alert.remaining_turns(game.turn()), 8);
        }
        let visible_alerts: Vec<_> = game
            .events()
            .iter()
            .filter_map(|event| match event {
                GameEvent::LocalAlertRaised { source, .. } => Some(*source),
                _ => None,
            })
            .collect();
        assert_eq!(visible_alerts, vec![visible_witness]);

        let carried = game.player_inventory().iter().next().unwrap();
        assert_eq!(carried.owner(), Some(&owner));
        let instance = carried.instance();
        assert_eq!(
            game.process_player_command(GameCommand::DropItem { item: instance }),
            CommandOutcome::Applied
        );
        let dropped = game
            .ground_items()
            .get(game.ground_items().item_at(GridPos::new(1, 1)).unwrap())
            .unwrap();
        assert_eq!(dropped.owner(), Some(&owner));
        for _ in 0..8 {
            assert_eq!(
                game.process_player_command(GameCommand::Wait),
                CommandOutcome::Applied
            );
        }
        assert!(
            game.actors()
                .get(visible_witness)
                .unwrap()
                .local_alert()
                .is_some_and(|alert| !alert.is_active(game.turn()))
        );
    }

    #[test]
    fn rejected_owned_pickup_creates_no_witness_memory() {
        let (mut rules, material) = material_rules();
        rules.player_inventory_capacity = 1;
        let mut game = GameState::new_with_rules(
            parse_map("#######\n#.....#\n#######"),
            GridPos::new(1, 1),
            1,
            rules,
        )
        .unwrap();
        game.player_inventory.add(material.clone(), 4, 4).unwrap();
        let owner: SocialGroupId = "core:maintainers".parse().unwrap();
        let witness = game
            .spawn_actor(
                build_actor(GridPos::new(3, 1), 10)
                    .with_affiliation(owner.clone())
                    .with_witness_profile(
                        WitnessProfile::new(4, DistanceMetric::Euclidean, true, 4).unwrap(),
                    )
                    .with_local_alert_profile(LocalAlertProfile::new(8).unwrap()),
            )
            .unwrap();
        let ground_item = game
            .spawn_ground_item_with_owner(GridPos::new(1, 1), material, 1, Some(owner))
            .unwrap();
        game.drain_events();

        assert_eq!(
            game.process_player_command(GameCommand::PickUp),
            CommandOutcome::Rejected(CommandRejection::InventoryCannotFitItem)
        );
        assert_eq!(game.turn(), 0);
        assert!(game.ground_items().get(ground_item).is_some());
        assert!(
            game.actors()
                .get(witness)
                .unwrap()
                .observed_property_takes()
                .is_empty()
        );
        assert!(game.actors().get(witness).unwrap().local_alert().is_none());
        assert!(game.events().is_empty());
    }

    #[test]
    fn explicit_property_authorization_prevents_witness_incidents() {
        let (rules, material) = material_rules();
        let mut game = GameState::new_with_rules(
            parse_map("#######\n#.....#\n#######"),
            GridPos::new(1, 1),
            1,
            rules,
        )
        .unwrap();
        let owner: SocialGroupId = "core:maintainers".parse().unwrap();
        assert!(game.grant_player_property_take_authorization(owner.clone()));
        assert!(!game.grant_player_property_take_authorization(owner.clone()));
        assert!(game.player_may_take_property_of(&owner));
        let witness = game
            .spawn_actor(
                build_actor(GridPos::new(3, 1), 10)
                    .with_affiliation(owner.clone())
                    .with_witness_profile(
                        WitnessProfile::new(4, DistanceMetric::Euclidean, true, 4).unwrap(),
                    )
                    .with_local_alert_profile(LocalAlertProfile::new(8).unwrap()),
            )
            .unwrap();
        game.spawn_ground_item_with_owner(GridPos::new(1, 1), material, 1, Some(owner.clone()))
            .unwrap();
        game.drain_events();

        assert_eq!(
            game.process_player_command(GameCommand::PickUp),
            CommandOutcome::Applied
        );
        let witness = game.actors().get(witness).unwrap();
        assert!(witness.observed_property_takes().is_empty());
        assert!(witness.local_alert().is_none());
        assert!(game.events().iter().all(|event| !matches!(
            event,
            GameEvent::PropertyTakeWitnessed { .. } | GameEvent::LocalAlertRaised { .. }
        )));
        assert_eq!(
            game.player_inventory()
                .iter()
                .next()
                .and_then(|entry| entry.owner()),
            Some(&owner)
        );
    }

    #[test]
    fn repair_at_full_integrity_consumes_neither_item_nor_turn() {
        let (rules, repair) = consumable_rules();
        let mut game = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        )
        .unwrap_or_else(|error| panic!("valid game rejected: {error}"));
        let repair_instance = game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &repair)
            .map(|entry| entry.instance())
            .unwrap_or_else(|| panic!("repair item missing from inventory"));

        assert_eq!(
            game.process_player_command(GameCommand::UseItem {
                item: repair_instance,
            }),
            CommandOutcome::Rejected(CommandRejection::ItemHasNoUsefulEffect(repair_instance))
        );
        assert_eq!(game.turn(), 0);
        assert_eq!(
            game.player_inventory()
                .get(repair_instance)
                .map(|entry| entry.quantity()),
            Some(2)
        );
        assert!(game.events().is_empty());
    }

    #[test]
    fn pickup_stacks_atomically_and_costs_one_turn() {
        let (rules, repair) = consumable_rules();
        let mut game = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        )
        .unwrap_or_else(|error| panic!("valid game rejected: {error}"));
        let ground_item = game
            .spawn_ground_item(GridPos::new(1, 1), repair.clone(), 1)
            .unwrap_or_else(|error| panic!("valid ground item rejected: {error}"));
        game.drain_events();

        assert_eq!(
            game.process_player_command(GameCommand::PickUp),
            CommandOutcome::Applied
        );

        assert_eq!(game.turn(), 1);
        assert!(game.ground_items().get(ground_item).is_none());
        assert_eq!(
            game.player_inventory()
                .iter()
                .find(|entry| entry.item() == &repair)
                .map(|entry| entry.quantity()),
            Some(3)
        );
        assert!(game.events().contains(&GameEvent::ItemPickedUp {
            entity: game.player_id(),
            ground_item,
            definition: repair,
            quantity: 1,
        }));
    }

    #[test]
    fn failed_pickup_keeps_the_entire_ground_stack_and_turn() {
        let (mut rules, repair) = consumable_rules();
        rules.player_inventory_capacity = 1;
        rules.player_starting_items[0].quantity = 3;
        let mut game = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        )
        .unwrap_or_else(|error| panic!("valid game rejected: {error}"));
        let ground_item = game
            .spawn_ground_item(GridPos::new(1, 1), repair, 1)
            .unwrap_or_else(|error| panic!("valid ground item rejected: {error}"));
        game.drain_events();

        assert_eq!(
            game.process_player_command(GameCommand::PickUp),
            CommandOutcome::Rejected(CommandRejection::InventoryCannotFitItem)
        );
        assert_eq!(game.turn(), 0);
        assert_eq!(
            game.ground_items()
                .get(ground_item)
                .map(|item| item.quantity()),
            Some(1)
        );
        assert!(game.events().is_empty());
    }

    #[test]
    fn ground_items_do_not_block_movement() {
        let (rules, repair) = consumable_rules();
        let mut game = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        )
        .unwrap_or_else(|error| panic!("valid game rejected: {error}"));
        game.spawn_ground_item(GridPos::new(2, 1), repair, 1)
            .unwrap_or_else(|error| panic!("valid ground item rejected: {error}"));
        game.drain_events();

        assert_eq!(
            game.process_player_command(GameCommand::Move(Direction::East)),
            CommandOutcome::Applied
        );
        assert_eq!(game.player_position(), Some(GridPos::new(2, 1)));
    }

    #[test]
    fn ground_item_spawn_rejects_walls_and_unknown_definitions() {
        let (rules, repair) = consumable_rules();
        let mut game = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        )
        .unwrap_or_else(|error| panic!("valid game rejected: {error}"));
        let unknown: ItemId = "missing.mod:ghost_item"
            .parse()
            .unwrap_or_else(|error| panic!("valid item ID rejected: {error}"));

        assert!(matches!(
            game.spawn_ground_item(GridPos::new(0, 0), repair, 1),
            Err(GroundItemSpawnError::BlockedByTerrain(_))
        ));
        assert_eq!(
            game.spawn_ground_item(GridPos::new(2, 1), unknown.clone(), 1),
            Err(GroundItemSpawnError::UnknownItemDefinition(unknown))
        );
    }

    #[test]
    fn dropping_an_equipped_weapon_moves_it_to_the_world_and_unequips_it() {
        let (rules, blade, _) = equipment_rules();
        let mut game = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        )
        .unwrap_or_else(|error| panic!("valid equipped game rejected: {error}"));
        let blade_item = game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &blade)
            .map(|entry| entry.instance())
            .unwrap_or_else(|| panic!("blade missing from inventory"));

        assert_eq!(
            game.process_player_command(GameCommand::DropItem { item: blade_item }),
            CommandOutcome::Applied
        );

        assert_eq!(game.turn(), 1);
        assert!(game.player_inventory().get(blade_item).is_none());
        assert!(game.equipped_player_weapon(0).is_none());
        let ground_item = game
            .ground_items()
            .item_at(GridPos::new(1, 1))
            .unwrap_or_else(|| panic!("dropped weapon missing from world"));
        assert_eq!(
            game.ground_items().get(ground_item).map(|item| item.item()),
            Some(&blade)
        );
        assert!(game.events().contains(&GameEvent::ItemDropped {
            entity: game.player_id(),
            item: blade_item,
            ground_item,
            definition: blade,
            quantity: 1,
        }));
    }

    #[test]
    fn failed_drop_preserves_inventory_equipment_and_turn() {
        let (rules, blade, arc) = equipment_rules();
        let mut game = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        )
        .unwrap_or_else(|error| panic!("valid equipped game rejected: {error}"));
        let arc_item = game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &arc)
            .map(|entry| entry.instance())
            .unwrap_or_else(|| panic!("arc weapon missing from inventory"));
        let existing_ground = game
            .spawn_ground_item(GridPos::new(1, 1), blade, 1)
            .unwrap_or_else(|error| panic!("valid ground item rejected: {error}"));
        game.drain_events();

        assert_eq!(
            game.process_player_command(GameCommand::DropItem { item: arc_item }),
            CommandOutcome::Rejected(CommandRejection::CannotDropItemHere)
        );
        assert_eq!(game.turn(), 0);
        assert!(game.player_inventory().get(arc_item).is_some());
        assert!(game.equipped_player_weapon(0).is_some());
        assert!(game.ground_items().get(existing_ground).is_some());
        assert!(game.events().is_empty());
    }

    #[test]
    fn player_must_start_on_a_walkable_tile() {
        let result = GameState::new(parse_map("###\n#.#\n###"), GridPos::new(0, 0), 1);

        assert!(matches!(
            result,
            Err(GameInitError::BlockedPlayerStart(GridPos { x: 0, y: 0 }))
        ));
    }

    #[test]
    fn player_starts_with_the_validated_primary_attribute_profile() {
        let game = small_game();

        assert_eq!(
            game.player_primary_attributes(),
            Some(PrimaryAttributes::prototype_default())
        );
    }

    #[test]
    fn invalid_player_creation_attributes_are_rejected_before_the_run() {
        let invalid = PrimaryAttributes::new(8, 8, 8, 8, 8);
        let rules = GameRules {
            player_starting_attributes: invalid,
            ..GameRules::default()
        };

        let result = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        );

        assert_eq!(
            result.err(),
            Some(GameInitError::PlayerAttributes(
                PrimaryAttributesError::WrongCreationTotal {
                    actual: 40,
                    expected: 28,
                }
            ))
        );
    }

    #[test]
    fn spawned_actor_attributes_may_ignore_creation_budget_but_not_absolute_bounds() {
        let mut game = small_game();
        let legal = build_actor(GridPos::new(1, 1), 5)
            .with_primary_attributes(PrimaryAttributes::new(10, 1, 1, 1, 1));
        game.spawn_actor(legal)
            .unwrap_or_else(|error| panic!("legal non-player profile rejected: {error}"));
        let illegal = build_actor(GridPos::new(3, 1), 5)
            .with_primary_attributes(PrimaryAttributes::new(11, 1, 1, 1, 1));

        assert!(matches!(
            game.spawn_actor(illegal),
            Err(SpawnError::InvalidPrimaryAttributes(
                PrimaryAttributesError::OutsideAbsoluteRange {
                    attribute: crate::stats::PrimaryAttribute::Power,
                    value: 11,
                    ..
                }
            ))
        ));
    }

    #[test]
    fn game_rules_control_visibility_without_changing_fov_code() {
        let rules = GameRules {
            player_field_of_view: FieldOfViewRules {
                radius: 1,
                distance_metric: DistanceMetric::Chebyshev,
                block_closed_corners: true,
            },
            ..GameRules::default()
        };
        let game = match GameState::new_with_rules(
            parse_map("#######\n#.....#\n#######"),
            GridPos::new(3, 1),
            7,
            rules,
        ) {
            Ok(game) => game,
            Err(error) => panic!("valid test game failed to initialize: {error}"),
        };

        assert!(game.player_visibility().is_visible(GridPos::new(4, 1)));
        assert!(!game.player_visibility().is_visible(GridPos::new(5, 1)));
    }

    #[test]
    fn attack_uses_profile_damage_and_removes_destroyed_target() {
        let mut game = small_game();
        let enemy = game
            .spawn_actor(build_actor(GridPos::new(3, 1), 5))
            .unwrap_or_else(|error| panic!("valid enemy failed to spawn: {error}"));
        game.drain_events();

        let outcome = game.process_player_command(GameCommand::Attack {
            slot: 0,
            target: enemy,
        });

        assert_eq!(outcome, CommandOutcome::Applied);
        assert!(game.actors().get(enemy).is_none());
        assert!(game.events().iter().any(|event| matches!(
            event,
            GameEvent::AttackPerformed {
                attacker,
                target,
                origin,
                target_at,
                weapon: None,
                damage_type: DamageType::Kinetic,
                ..
            } if *attacker == game.player_id()
                && *target == Some(enemy)
                && *origin == GridPos::new(2, 1)
                && *target_at == GridPos::new(3, 1)
        )));
        assert!(game.events().contains(&GameEvent::DamageApplied {
            source: Some(game.player_id()),
            target: enemy,
            amount: 5,
            damage_type: DamageType::Kinetic,
        }));
        assert!(
            game.events()
                .contains(&GameEvent::EntityDied { entity: enemy })
        );
    }

    #[test]
    fn hunter_ai_moves_toward_a_visible_player() {
        let mut game = match GameState::new(
            parse_map("#######\n#.....#\n#######"),
            GridPos::new(1, 1),
            1,
        ) {
            Ok(game) => game,
            Err(error) => panic!("valid test game failed to initialize: {error}"),
        };
        let hunter = build_actor(GridPos::new(4, 1), 10)
            .with_attack(AttackProfile::melee(DamageType::Kinetic, 2))
            .with_ai(AiProfile::hunter(8, 0));
        let hunter_id = game
            .spawn_actor(hunter)
            .unwrap_or_else(|error| panic!("valid hunter failed to spawn: {error}"));
        game.drain_events();

        game.process_player_command(GameCommand::Wait);

        assert_eq!(
            game.actors().get(hunter_id).map(Actor::position),
            Some(GridPos::new(3, 1))
        );
    }

    #[test]
    fn adjacent_hunter_can_destroy_player_and_end_run() {
        let rules = GameRules {
            player_maximum_integrity: 3,
            ..GameRules::default()
        };
        let mut game = match GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        ) {
            Ok(game) => game,
            Err(error) => panic!("valid test game failed to initialize: {error}"),
        };
        let hunter = build_actor(GridPos::new(2, 1), 10)
            .with_attack(AttackProfile::melee(DamageType::Kinetic, 5))
            .with_ai(AiProfile::hunter(8, 0));
        game.spawn_actor(hunter)
            .unwrap_or_else(|error| panic!("valid hunter failed to spawn: {error}"));
        game.drain_events();

        game.process_player_command(GameCommand::Wait);

        assert_eq!(game.status(), RunStatus::PlayerDestroyed);
        assert_eq!(game.phase(), TurnPhase::RunEnded);
        assert_eq!(game.player_position(), None);
        assert_eq!(
            game.process_player_command(GameCommand::Wait),
            CommandOutcome::Rejected(CommandRejection::RunEnded)
        );
    }

    #[test]
    fn ranged_attack_cannot_cross_an_opaque_wall() {
        let rules = GameRules {
            player_base_attacks: vec![AttackProfile::new(
                5,
                DistanceMetric::Chebyshev,
                true,
                DamageType::Kinetic,
                3,
                0,
            )],
            ..GameRules::default()
        };
        let mut game = match GameState::new_with_rules(
            parse_map("#######\n#.#...#\n#######"),
            GridPos::new(1, 1),
            1,
            rules,
        ) {
            Ok(game) => game,
            Err(error) => panic!("valid test game failed to initialize: {error}"),
        };
        let target = game
            .spawn_actor(build_actor(GridPos::new(3, 1), 10))
            .unwrap_or_else(|error| panic!("valid target failed to spawn: {error}"));
        game.drain_events();

        let outcome = game.process_player_command(GameCommand::Attack { slot: 0, target });

        assert_eq!(
            outcome,
            CommandOutcome::Rejected(CommandRejection::NoLineOfSight(target))
        );
        assert_eq!(game.turn(), 0);
    }

    #[test]
    fn default_secondary_attack_hits_a_visible_distant_target() {
        let mut game = match GameState::new(
            parse_map("#########\n#.......#\n#########"),
            GridPos::new(1, 1),
            1,
        ) {
            Ok(game) => game,
            Err(error) => panic!("valid test game failed to initialize: {error}"),
        };
        let target = game
            .spawn_actor(build_actor(GridPos::new(5, 1), 10))
            .unwrap_or_else(|error| panic!("valid target failed to spawn: {error}"));
        game.drain_events();

        let outcome = game.process_player_command(GameCommand::Attack { slot: 1, target });

        assert_eq!(outcome, CommandOutcome::Applied);
        assert_eq!(game.actors().get(target).map(Actor::integrity), Some(7));
        assert_eq!(game.turn(), 1);
    }

    #[test]
    fn radial_ability_applies_falloff_to_every_actor_in_propagation() {
        let mut game = match GameState::new(
            parse_map("#########\n#.......#\n#.......#\n#########"),
            GridPos::new(1, 1),
            1,
        ) {
            Ok(game) => game,
            Err(error) => panic!("valid test game failed to initialize: {error}"),
        };
        let center_target = game
            .spawn_actor(build_actor(GridPos::new(4, 1), 7))
            .unwrap_or_else(|error| panic!("valid target failed to spawn: {error}"));
        let adjacent_target = game
            .spawn_actor(build_actor(GridPos::new(4, 2), 10))
            .unwrap_or_else(|error| panic!("valid target failed to spawn: {error}"));
        game.drain_events();

        let outcome = game.process_player_command(GameCommand::UseAbility {
            slot: 0,
            target: GridPos::new(4, 1),
        });

        assert_eq!(outcome, CommandOutcome::Applied);
        assert!(game.actors().get(center_target).is_none());
        assert_eq!(
            game.actors().get(adjacent_target).map(Actor::integrity),
            Some(5)
        );
        assert!(game.events().iter().any(|event| matches!(
            event,
            GameEvent::PropagationResolved { origin, .. }
                if *origin == GridPos::new(4, 1)
        )));
    }

    #[test]
    fn wall_blocks_radial_damage_when_policy_forbids_crossing() {
        let mut game = match GameState::new(
            parse_map("#########\n#..#....#\n#.......#\n#########"),
            GridPos::new(1, 1),
            1,
        ) {
            Ok(game) => game,
            Err(error) => panic!("valid test game failed to initialize: {error}"),
        };
        let protected_target = game
            .spawn_actor(build_actor(GridPos::new(4, 1), 10))
            .unwrap_or_else(|error| panic!("valid target failed to spawn: {error}"));
        game.drain_events();

        let outcome = game.process_player_command(GameCommand::UseAbility {
            slot: 0,
            target: GridPos::new(2, 1),
        });

        assert_eq!(outcome, CommandOutcome::Applied);
        assert_eq!(
            game.actors().get(protected_target).map(Actor::integrity),
            Some(10)
        );
    }

    #[test]
    fn reaching_generated_exit_ends_the_run_before_ai_resolution() {
        let map = parse_map("#####\n#...#\n#####");
        let mut game = GameState::build(
            map,
            GridPos::new(1, 1),
            Some(GridPos::new(2, 1)),
            1,
            GameRules::default(),
        )
        .unwrap_or_else(|error| panic!("valid run failed to initialize: {error}"));

        let outcome = game.process_player_command(GameCommand::Move(Direction::East));

        assert_eq!(outcome, CommandOutcome::Applied);
        assert_eq!(game.status(), RunStatus::Escaped);
        assert_eq!(game.phase(), TurnPhase::RunEnded);
        assert!(game.events().contains(&GameEvent::ExitReached {
            entity: game.player_id(),
            at: GridPos::new(2, 1),
        }));
    }

    #[test]
    fn player_kill_awards_xp_and_emits_each_crossed_level() {
        let progression = ProgressionRules {
            curve: ExperienceCurve::new(vec![3, 6, 12])
                .unwrap_or_else(|error| panic!("valid curve rejected: {error}")),
            starting_skill_points: 0,
            skill_points_per_level: 1,
            trivial_threat_level_gap: 3,
            trivial_reward_percent: 0,
            summoned_reward_percent: 0,
            fabricated_reward_percent: 0,
        };
        let rules = GameRules {
            progression,
            ..GameRules::default()
        };
        let mut game = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        )
        .unwrap_or_else(|error| panic!("valid test game failed to initialize: {error}"));
        let enemy = game
            .spawn_actor(
                build_actor(GridPos::new(2, 1), 5)
                    .with_defeat_reward(DefeatReward::persistent(7, 1)),
            )
            .unwrap_or_else(|error| panic!("valid enemy failed to spawn: {error}"));
        game.drain_events();

        let outcome = game.process_player_command(GameCommand::Attack {
            slot: 0,
            target: enemy,
        });

        assert_eq!(outcome, CommandOutcome::Applied);
        assert_eq!(game.player_progression().experience(), 7);
        assert_eq!(game.player_progression().level(), 3);
        assert_eq!(game.player_progression().unspent_skill_points(), 2);
        assert!(game.events().contains(&GameEvent::ExperienceAwarded {
            amount: 7,
            total: 7,
            source: ExperienceSource::DefeatedEntity(enemy),
        }));
        assert!(game.events().contains(&GameEvent::LevelGained {
            level: 2,
            skill_points_awarded: 1,
        }));
        assert!(game.events().contains(&GameEvent::LevelGained {
            level: 3,
            skill_points_awarded: 1,
        }));
    }

    #[test]
    fn skill_purchase_spends_points_atomically_without_advancing_time() {
        let progression = ProgressionRules {
            curve: ExperienceCurve::new(vec![1])
                .unwrap_or_else(|error| panic!("valid curve rejected: {error}")),
            starting_skill_points: 0,
            skill_points_per_level: 1,
            trivial_threat_level_gap: 3,
            trivial_reward_percent: 0,
            summoned_reward_percent: 0,
            fabricated_reward_percent: 0,
        };
        let rules = rules_with_reconnaissance(progression);
        let mut game = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules.clone(),
        )
        .unwrap_or_else(|error| panic!("valid test game failed to initialize: {error}"));
        let enemy = game
            .spawn_actor(
                build_actor(GridPos::new(2, 1), 1)
                    .with_defeat_reward(DefeatReward::persistent(1, 1)),
            )
            .unwrap_or_else(|error| panic!("valid enemy failed to spawn: {error}"));
        assert_eq!(
            game.process_player_command(GameCommand::Attack {
                slot: 0,
                target: enemy,
            }),
            CommandOutcome::Applied
        );
        assert_eq!(game.player_progression().unspent_skill_points(), 1);
        let turn_before_learning = game.turn();
        game.drain_events();
        let technique: TechniqueId = "core:rec_01"
            .parse()
            .unwrap_or_else(|error| panic!("valid technique ID rejected: {error}"));
        let discipline: DisciplineId = "core:reconnaissance"
            .parse()
            .unwrap_or_else(|error| panic!("valid discipline ID rejected: {error}"));

        assert_eq!(
            game.process_player_command(GameCommand::LearnTechnique {
                technique: technique.clone(),
            }),
            CommandOutcome::AppliedWithoutTime
        );
        assert_eq!(game.turn(), turn_before_learning);
        assert_eq!(game.player_progression().unspent_skill_points(), 0);
        assert_eq!(game.player_skills().rank(&discipline), 1);
        assert!(game.player_skills().has_learned(&technique));
        assert_eq!(
            game.events(),
            &[GameEvent::TechniqueLearned {
                entity: game.player_id(),
                technique,
                discipline,
                rank: 1,
                skill_points_spent: 1,
                skill_points_remaining: 0,
            }]
        );

        let saved = game
            .export_player_progression()
            .unwrap_or_else(|error| panic!("valid progression failed to serialize: {error}"));
        let mut restored = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            99,
            rules,
        )
        .unwrap_or_else(|error| panic!("valid restore target failed to initialize: {error}"));
        restored
            .restore_player_progression(&saved)
            .unwrap_or_else(|error| panic!("valid progression failed to restore: {error}"));
        assert_eq!(restored.player_progression(), game.player_progression());
        assert_eq!(restored.player_skills(), game.player_skills());
    }

    #[test]
    fn failed_skill_purchase_changes_neither_points_choices_time_nor_events() {
        let mut game = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules_with_reconnaissance(ProgressionRules {
                starting_skill_points: 0,
                ..ProgressionRules::default()
            }),
        )
        .unwrap_or_else(|error| panic!("valid test game failed to initialize: {error}"));
        let technique: TechniqueId = "core:rec_01"
            .parse()
            .unwrap_or_else(|error| panic!("valid technique ID rejected: {error}"));

        assert_eq!(
            game.process_player_command(GameCommand::LearnTechnique {
                technique: technique.clone(),
            }),
            CommandOutcome::Rejected(CommandRejection::InsufficientSkillPoints {
                required: 1,
                available: 0,
            })
        );
        assert_eq!(game.turn(), 0);
        assert_eq!(game.player_progression().unspent_skill_points(), 0);
        assert!(!game.player_skills().has_learned(&technique));
        assert!(game.events().is_empty());
    }

    #[test]
    fn learned_target_analysis_reports_real_actor_state_and_costs_one_turn() {
        let mut game = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules_with_reconnaissance(ProgressionRules::default()),
        )
        .unwrap_or_else(|error| panic!("valid test game failed to initialize: {error}"));
        let target = game
            .spawn_actor(
                build_actor(GridPos::new(2, 1), 9)
                    .with_resistances(ResistanceProfile::default().with(DamageType::Kinetic, 25)),
            )
            .unwrap_or_else(|error| panic!("valid target failed to spawn: {error}"));
        let technique: TechniqueId = "core:rec_01"
            .parse()
            .unwrap_or_else(|error| panic!("valid technique ID rejected: {error}"));
        assert_eq!(
            game.process_player_command(GameCommand::LearnTechnique {
                technique: technique.clone(),
            }),
            CommandOutcome::AppliedWithoutTime
        );
        game.drain_events();

        assert_eq!(
            game.process_player_command(GameCommand::UseTechnique {
                technique: technique.clone(),
                targets: vec![target],
            }),
            CommandOutcome::Applied
        );
        assert_eq!(game.turn(), 1);
        assert_eq!(
            game.events(),
            &[
                GameEvent::TechniqueUsed {
                    entity: game.player_id(),
                    technique,
                    observed_on_turn: 0,
                },
                GameEvent::TargetAnalyzed {
                    observer: game.player_id(),
                    target,
                    at: GridPos::new(2, 1),
                    integrity: 9,
                    maximum_integrity: 9,
                    resistances: ResistanceProfile::default().with(DamageType::Kinetic, 25),
                },
                GameEvent::TurnCompleted { turn: 1 },
            ]
        );
    }

    #[test]
    fn trace_reading_returns_only_stored_local_evidence_not_an_entity_tracker() {
        let mut game = GameState::new_with_rules(
            parse_map("#######\n#.....#\n#######"),
            GridPos::new(1, 1),
            1,
            rules_with_reconnaissance(ProgressionRules::default()),
        )
        .unwrap_or_else(|error| panic!("valid test game failed to initialize: {error}"));
        let technique: TechniqueId = "core:rec_02"
            .parse()
            .unwrap_or_else(|error| panic!("valid technique ID rejected: {error}"));
        assert_eq!(
            game.process_player_command(GameCommand::LearnTechnique {
                technique: technique.clone(),
            }),
            CommandOutcome::AppliedWithoutTime
        );
        game.drain_events();
        assert_eq!(
            game.process_player_command(GameCommand::Move(Direction::East)),
            CommandOutcome::Applied
        );
        game.drain_events();

        assert_eq!(
            game.process_player_command(GameCommand::UseTechnique {
                technique: technique.clone(),
                targets: Vec::new(),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(
            game.events(),
            &[
                GameEvent::TechniqueUsed {
                    entity: game.player_id(),
                    technique,
                    observed_on_turn: 1,
                },
                GameEvent::MovementTracesRead {
                    observer: game.player_id(),
                    traces: vec![crate::world::ObservedMovementTrace {
                        position: GridPos::new(1, 1),
                        direction: Direction::East,
                        age_turns: 1,
                    }],
                },
                GameEvent::TurnCompleted { turn: 2 },
            ]
        );
    }

    #[test]
    fn defeating_a_summoned_enemy_does_not_award_experience() {
        let mut game = small_game();
        let enemy = game
            .spawn_actor(
                build_actor(GridPos::new(3, 1), 5)
                    .with_defeat_reward(DefeatReward::summoned(1_000, 20)),
            )
            .unwrap_or_else(|error| panic!("valid enemy failed to spawn: {error}"));
        game.drain_events();

        game.process_player_command(GameCommand::Attack {
            slot: 0,
            target: enemy,
        });

        assert_eq!(game.player_progression().experience(), 0);
        assert!(
            !game
                .events()
                .iter()
                .any(|event| matches!(event, GameEvent::ExperienceAwarded { .. }))
        );
    }

    fn learned_recon_game(energy: u16) -> GameState {
        let mut rules = rules_with_reconnaissance(ProgressionRules {
            starting_skill_points: 9,
            ..ProgressionRules::default()
        });
        rules.player_starting_energy = energy;
        rules.player_field_of_view.radius = 4;
        let mut game = GameState::new_with_rules(
            parse_map("#########\n#.......#\n#.......#\n#########"),
            GridPos::new(1, 1),
            7,
            rules,
        )
        .unwrap();
        for technique in [
            "core:rec_01",
            "core:rec_02",
            "core:rec_09",
            "core:rec_04",
            "core:rec_05",
        ] {
            assert_eq!(
                game.process_player_command(GameCommand::LearnTechnique {
                    technique: technique.parse().unwrap(),
                }),
                CommandOutcome::AppliedWithoutTime
            );
        }
        assert_eq!(game.player_progression().unspent_skill_points(), 0);
        assert_eq!(game.player_energy().available(), energy);
        assert_eq!(game.turn(), 0);
        game.drain_events();
        game
    }

    fn multiple_analysis_command(targets: Vec<EntityId>) -> GameCommand {
        GameCommand::UseTechnique {
            technique: "core:rec_09".parse().unwrap(),
            targets,
        }
    }

    #[test]
    fn complete_recon_progression_executes_multi_analysis_with_the_same_individual_results() {
        let mut game = learned_recon_game(100);
        let targets: Vec<_> = (2..=4)
            .map(|x| {
                game.spawn_actor(
                    build_actor(GridPos::new(x, 1), x as u16 * 3).with_resistances(
                        ResistanceProfile::default().with(DamageType::Thermal, x as i16 * 10),
                    ),
                )
                .unwrap()
            })
            .collect();
        game.drain_events();
        let rng = game.rng_state();
        assert_eq!(
            game.process_player_command(multiple_analysis_command(targets.clone())),
            CommandOutcome::Applied
        );
        assert_eq!(game.turn(), 1);
        assert_eq!(game.player_energy().available(), 98);
        assert_eq!(game.rng_state(), rng);
        let multi_events: Vec<_> = game
            .drain_events()
            .into_iter()
            .filter(|event| matches!(event, GameEvent::TargetAnalyzed { .. }))
            .collect();
        assert_eq!(multi_events.len(), 3);
        let mut single_events = Vec::new();
        for target in targets {
            assert_eq!(
                game.process_player_command(GameCommand::UseTechnique {
                    technique: "core:rec_01".parse().unwrap(),
                    targets: vec![target],
                }),
                CommandOutcome::Applied
            );
            single_events.extend(
                game.drain_events()
                    .into_iter()
                    .filter(|event| matches!(event, GameEvent::TargetAnalyzed { .. })),
            );
        }
        assert_eq!(single_events, multi_events);
        assert_eq!(game.turn(), 4);
        assert_eq!(game.player_energy().available(), 98);
    }

    #[test]
    fn invalid_multi_analysis_is_atomic_even_when_only_the_last_target_is_hidden() {
        let mut game = learned_recon_game(100);
        let visible = game
            .spawn_actor(build_actor(GridPos::new(2, 1), 12))
            .unwrap();
        let second = game
            .spawn_actor(build_actor(GridPos::new(3, 1), 13))
            .unwrap();
        let third = game
            .spawn_actor(build_actor(GridPos::new(4, 1), 14))
            .unwrap();
        let hidden = game
            .spawn_actor(build_actor(GridPos::new(7, 1), 15))
            .unwrap();
        game.drain_events();
        let before = (
            game.turn(),
            game.rng_state(),
            game.player_energy(),
            game.export_player_progression().unwrap(),
        );
        for (targets, error) in [
            (vec![], CommandRejection::TechniqueMissingTarget),
            (
                vec![visible, visible],
                CommandRejection::TechniqueDuplicateTarget(visible),
            ),
            (
                vec![visible, second, third, hidden],
                CommandRejection::TechniqueTooManyTargets {
                    maximum: 3,
                    actual: 4,
                },
            ),
            (
                vec![visible, hidden],
                CommandRejection::TechniqueTargetNotVisible(hidden),
            ),
        ] {
            assert_eq!(
                game.process_player_command(multiple_analysis_command(targets)),
                CommandOutcome::Rejected(error)
            );
            assert!(game.events().is_empty());
            assert_eq!(
                (
                    game.turn(),
                    game.rng_state(),
                    game.player_energy(),
                    game.export_player_progression().unwrap()
                ),
                before
            );
        }
    }

    #[test]
    fn modded_analysis_range_is_inherited_by_execution_and_client_candidates() {
        let mut game = learned_recon_game(100);
        let mut catalog = SkillCatalog::default();
        for (_, discipline) in game.rules.skills.disciplines() {
            catalog.register_discipline(discipline.clone()).unwrap();
        }
        for (id, definition) in game.rules.skills.techniques() {
            let definition = match id.as_str() {
                "core:rec_01" => definition
                    .clone()
                    .with_action(TechniqueAction::AnalyzeTarget { range: 2 })
                    .unwrap(),
                "core:rec_09" => definition
                    .clone()
                    .with_action(TechniqueAction::AnalyzeMultipleTargets {
                        maximum_targets: 2,
                        energy_cost: 7,
                    })
                    .unwrap(),
                _ => definition.clone(),
            };
            catalog.register_technique(definition).unwrap();
        }
        catalog
            .validate_runtime(
                &game.rules.enabled_system_features,
                &game.rules.skill_progression,
            )
            .unwrap();
        game.rules.skills = catalog;
        let targets: Vec<_> = (2..=4)
            .map(|x| {
                game.spawn_actor(build_actor(GridPos::new(x, 1), 12))
                    .unwrap()
            })
            .collect();
        let technique = "core:rec_09".parse().unwrap();
        assert!(game.player_visibility().is_visible(GridPos::new(4, 1)));
        assert_eq!(game.player_technique_targets(&technique), targets[..2]);
        game.drain_events();
        assert_eq!(
            game.process_player_command(multiple_analysis_command(vec![targets[0], targets[2]])),
            CommandOutcome::Rejected(CommandRejection::TechniqueTargetOutOfRange(targets[2]))
        );
        assert_eq!(game.turn(), 0);
        assert_eq!(game.player_energy().available(), 100);
        assert!(game.events().is_empty());
        assert_eq!(
            game.process_player_command(multiple_analysis_command(targets[..2].to_vec())),
            CommandOutcome::Applied
        );
        assert_eq!(game.turn(), 1);
        assert_eq!(game.player_energy().available(), 93);
    }

    #[test]
    fn empty_energy_blocks_only_costly_actions_and_progression_restore_cannot_refill_it() {
        let mut game = learned_recon_game(2);
        let target = game
            .spawn_actor(build_actor(GridPos::new(2, 1), 30))
            .unwrap();
        let saved = game.export_player_progression().unwrap();
        assert_eq!(
            game.process_player_command(multiple_analysis_command(vec![target])),
            CommandOutcome::Applied
        );
        assert_eq!(game.player_energy().available(), 0);
        game.restore_player_progression(&saved).unwrap();
        game.drain_events();
        assert_eq!(
            game.process_player_command(multiple_analysis_command(vec![target])),
            CommandOutcome::Rejected(CommandRejection::InsufficientEnergy {
                required: 2,
                available: 0
            })
        );
        assert_eq!(game.turn(), 1);
        assert!(game.events().is_empty());
        for command in [
            GameCommand::Wait,
            GameCommand::Attack { slot: 0, target },
            GameCommand::Move(Direction::South),
        ] {
            assert_eq!(
                game.process_player_command(command),
                CommandOutcome::Applied
            );
            assert_eq!(game.player_energy().available(), 0);
        }
    }

    #[test]
    fn wall_examination_is_bound_to_rec04_and_other_learned_actions_are_executable() {
        let mut game = learned_recon_game(0);
        let target = game
            .spawn_actor(
                build_actor(GridPos::new(2, 1), 12)
                    .with_attack(AttackProfile::melee(DamageType::Kinetic, 3)),
            )
            .unwrap();
        game.drain_events();
        assert_eq!(
            game.process_player_command(GameCommand::UseTechnique {
                technique: "core:rec_04".parse().unwrap(),
                targets: vec![],
            }),
            CommandOutcome::Applied
        );
        let events = game.drain_events();
        let tiles = events
            .iter()
            .find_map(|event| match event {
                GameEvent::TerrainAnalyzed { tiles, .. } => Some(tiles),
                _ => None,
            })
            .unwrap();
        assert!(!tiles.is_empty() && tiles.len() <= 3);
        for (index, tile) in tiles.iter().enumerate() {
            assert!(game.player_visibility().is_visible(tile.position));
            assert!(tile.blocks_movement && tile.blocks_vision);
            if index > 0 {
                assert!(
                    tiles[..index]
                        .iter()
                        .any(|other| other.position.cardinal_neighbors().contains(&tile.position))
                );
            }
        }
        assert_eq!(
            game.process_player_command(GameCommand::UseTechnique {
                technique: "core:rec_05".parse().unwrap(),
                targets: vec![target],
            }),
            CommandOutcome::Applied
        );
        assert!(game.events().iter().any(
            |event| matches!(event, GameEvent::ThreatAnalyzed { attacks, .. } if attacks.len() == 1)
        ));
    }

    #[test]
    fn startup_refuses_to_sell_a_technique_without_executable_behavior() {
        let mut rules = rules_with_reconnaissance(ProgressionRules::default());
        rules.enabled_system_features = SystemFeatureSet::new([
            "core:traces".parse().unwrap(),
            "core:secrets".parse().unwrap(),
        ]);
        let result =
            GameState::new_with_rules(parse_map("###\n#.#\n###"), GridPos::new(1, 1), 0, rules);
        assert!(
            matches!(result, Err(GameInitError::SkillCatalog(SkillCatalogError::MissingTechniqueBehavior(id))) if id.as_str() == "core:rec_03")
        );
    }

    #[test]
    fn turn_end_status_ticks_exact_duration_then_expires() {
        let (rules, corrosion) = corrosion_rules();
        let mut game = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        )
        .unwrap_or_else(|error| panic!("valid test game failed to initialize: {error}"));
        let target = game
            .spawn_actor(build_actor(GridPos::new(2, 1), 10))
            .unwrap_or_else(|error| panic!("valid target failed to spawn: {error}"));
        game.drain_events();

        assert_eq!(
            game.process_player_command(GameCommand::UseAbility {
                slot: 0,
                target: GridPos::new(2, 1),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(game.actors().get(target).map(Actor::integrity), Some(8));
        assert_eq!(
            game.actors()
                .get(target)
                .and_then(|actor| actor.status(&corrosion))
                .and_then(|status| status.remaining_turns),
            Some(2)
        );

        game.drain_events();
        game.process_player_command(GameCommand::Wait);
        game.drain_events();
        game.process_player_command(GameCommand::Wait);

        assert_eq!(game.actors().get(target).map(Actor::integrity), Some(4));
        assert!(
            game.actors()
                .get(target)
                .is_some_and(|actor| actor.status(&corrosion).is_none())
        );
        assert!(game.events().contains(&GameEvent::StatusRemoved {
            target,
            status: corrosion,
            reason: crate::game::StatusRemovalReason::Expired,
        }));
    }

    #[test]
    fn delayed_status_kill_keeps_its_original_source_for_xp_credit() {
        let (rules, _) = corrosion_rules();
        let mut game = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        )
        .unwrap_or_else(|error| panic!("valid test game failed to initialize: {error}"));
        let target = game
            .spawn_actor(
                build_actor(GridPos::new(2, 1), 4)
                    .with_defeat_reward(DefeatReward::persistent(6, 1)),
            )
            .unwrap_or_else(|error| panic!("valid target failed to spawn: {error}"));
        game.drain_events();

        game.process_player_command(GameCommand::UseAbility {
            slot: 0,
            target: GridPos::new(2, 1),
        });
        game.drain_events();
        game.process_player_command(GameCommand::Wait);

        assert!(game.actors().get(target).is_none());
        assert_eq!(game.player_progression().experience(), 6);
        assert!(game.events().contains(&GameEvent::ExperienceAwarded {
            amount: 6,
            total: 6,
            source: ExperienceSource::DefeatedEntity(target),
        }));
    }

    #[test]
    fn unknown_status_reference_is_rejected_before_run_initialization() {
        let unknown: StatusId = "missing:status"
            .parse()
            .unwrap_or_else(|error| panic!("valid status ID rejected: {error}"));
        let application = ApplyStatusEffect::new(unknown.clone(), 1)
            .unwrap_or_else(|error| panic!("valid status application rejected: {error}"));
        let rules = GameRules {
            player_base_abilities: vec![AbilityProfile::new(
                1,
                DistanceMetric::Chebyshev,
                false,
                true,
                vec![EffectPrimitive::ApplyStatus(application)],
            )],
            ..GameRules::default()
        };

        let result = GameState::new_with_rules(
            parse_map("#####\n#...#\n#####"),
            GridPos::new(1, 1),
            1,
            rules,
        );

        assert!(matches!(
            result,
            Err(GameInitError::UnknownStatusDefinition(status)) if status == unknown
        ));
    }
}
