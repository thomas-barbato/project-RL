use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::ai::{AiAction, AiSituation, decide_action};
use crate::combat::{DamagePacket, resolve_damage};
use crate::effects::{ApplyStatusEffect, EffectPrimitive, RadialDamageEffect};
use crate::entity::{
    Actor, ActorBuildError, ActorRegistry, EntityId, Equipment, EquipmentError, EquipmentSlotId,
    Inventory, InventoryError, ItemInstanceId, RegistryError,
};
use crate::progression::{DefeatReward, ExperienceAward, ProgressionRulesError, RunProgression};
use crate::status::{
    StatusCatalog, StatusEffectPrimitive, StatusId, StatusInstance, StatusTrigger,
};
use crate::weapon::{WeaponDefinition, WeaponId};
use crate::world::generation::GeneratedMap;
use crate::world::{Direction, GridPos, Map, VisibilityState, has_line_of_sight};

use super::{ExperienceSource, GameCommand, GameEvent, GameRng, GameRules, TurnPhase};

pub struct GameState {
    map: Map,
    actors: ActorRegistry,
    player: EntityId,
    exit: Option<GridPos>,
    turn: u64,
    phase: TurnPhase,
    status: RunStatus,
    events: Vec<GameEvent>,
    rng: GameRng,
    rules: GameRules,
    player_visibility: VisibilityState,
    player_progression: RunProgression,
    player_inventory: Inventory,
    player_equipment: Equipment,
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
            .with_abilities(rules.player_base_abilities.iter().cloned());
        if let Some(status) = first_unknown_status(&player_actor, &rules.statuses) {
            return Err(GameInitError::UnknownStatusDefinition(status));
        }
        let mut actors = ActorRegistry::default();
        let player = actors
            .spawn(player_actor)
            .map_err(GameInitError::Registry)?;
        let mut player_visibility = VisibilityState::default();
        player_visibility.recompute(&map, player_start, rules.player_field_of_view);

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
            player_progression: RunProgression::default(),
            player_inventory,
            player_equipment,
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

    pub const fn player_inventory(&self) -> &Inventory {
        &self.player_inventory
    }

    pub const fn player_equipment(&self) -> &Equipment {
        &self.player_equipment
    }

    pub fn equipped_player_weapon(&self, slot: u8) -> Option<&WeaponDefinition> {
        let slot_id = self.rules.player_weapon_slots.get(usize::from(slot))?;
        let item = self.player_equipment.equipped(slot_id)?;
        let entry = self.player_inventory.get(item)?;
        self.rules.weapons.get(entry.item())
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

        let entity = self.actors.spawn(actor).map_err(SpawnError::Registry)?;
        self.events.push(GameEvent::EntitySpawned {
            entity,
            at: position,
        });
        Ok(entity)
    }

    pub fn process_player_command(&mut self, command: GameCommand) -> CommandOutcome {
        if self.status != RunStatus::Active {
            return CommandOutcome::Rejected(CommandRejection::RunEnded);
        }
        if self.phase != TurnPhase::AwaitingPlayer {
            return CommandOutcome::Rejected(CommandRejection::NotPlayersTurn);
        }

        let outcome = match command {
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
            GameCommand::EquipWeapon { slot, item } => match self.equip_player_weapon(slot, item) {
                Ok(()) => CommandOutcome::Applied,
                Err(error) => CommandOutcome::Rejected(error.into()),
            },
            GameCommand::UseAbility { slot, target } => {
                match self.perform_ability(self.player, slot, target) {
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

    fn move_entity(
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

        if !self.map.is_walkable(destination) {
            return Err(MovementError::BlockedByTerrain(destination));
        }
        if self.actors.entity_at(destination).is_some() {
            return Err(MovementError::Occupied(destination));
        }

        self.actors
            .move_to(entity, destination)
            .map_err(|_| MovementError::MissingEntity(entity))?;
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
        let attacker_state = self
            .actors
            .get(attacker)
            .cloned()
            .ok_or(AttackError::MissingAttacker(attacker))?;
        let target_state = self
            .actors
            .get(target)
            .cloned()
            .ok_or(AttackError::UnknownTarget(target))?;
        let attack = if attacker == self.player && !self.rules.player_weapon_slots.is_empty() {
            self.equipped_player_weapon(slot)
                .map(WeaponDefinition::attack)
                .ok_or(AttackError::MissingAttackSlot(slot))?
        } else {
            attacker_state
                .attack(slot)
                .ok_or(AttackError::MissingAttackSlot(slot))?
        };

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

        self.events.push(GameEvent::AttackPerformed {
            attacker,
            target,
            slot,
        });
        self.apply_damage_to(Some(attacker), target, attack.damage())
            .map_err(|_| AttackError::UnknownTarget(target))?;

        Ok(())
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

    fn complete_turn(&mut self) {
        self.phase = TurnPhase::ResolvingActors;
        self.resolve_ai_turn();
        self.phase = TurnPhase::ResolvingEnvironment;
        if self.status == RunStatus::Active {
            self.resolve_status_trigger(StatusTrigger::TurnEnd);
            self.elapse_status_durations();
        }
        self.turn = self.turn.saturating_add(1);
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

    fn resolve_status_trigger(&mut self, trigger: StatusTrigger) {
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

    fn elapse_status_durations(&mut self) {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunStatus {
    Active,
    PlayerDestroyed,
    Escaped,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandOutcome {
    Applied,
    Rejected(CommandRejection),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandRejection {
    RunEnded,
    NotPlayersTurn,
    MissingPlayer,
    BlockedByTerrain(GridPos),
    Occupied(GridPos),
    UnknownTarget(EntityId),
    MissingAttackSlot(u8),
    TargetOutOfRange(EntityId),
    NoLineOfSight(EntityId),
    MissingEquipmentSlot(u8),
    UnknownInventoryItem(ItemInstanceId),
    ItemIsNotWeapon(ItemInstanceId),
    ItemAlreadyEquippedInSlot { slot: u8, item: ItemInstanceId },
    MissingAbilitySlot(u8),
    AbilityTargetOutsideMap(GridPos),
    AbilityTargetBlocked(GridPos),
    AbilityTargetOutOfRange(GridPos),
    AbilityNoLineOfSight(GridPos),
    AbilityTargetHasNoActor(GridPos),
    AbilityUnknownStatusDefinition,
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
enum MovementError {
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
    MissingAttacker(EntityId),
    UnknownTarget(EntityId),
    MissingAttackSlot(u8),
    TargetOutOfRange(EntityId),
    NoLineOfSight(EntityId),
}

impl From<AttackError> for CommandRejection {
    fn from(error: AttackError) -> Self {
        match error {
            AttackError::MissingAttacker(_) => Self::MissingPlayer,
            AttackError::UnknownTarget(target) => Self::UnknownTarget(target),
            AttackError::MissingAttackSlot(slot) => Self::MissingAttackSlot(slot),
            AttackError::TargetOutOfRange(target) => Self::TargetOutOfRange(target),
            AttackError::NoLineOfSight(target) => Self::NoLineOfSight(target),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AbilityError {
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
        }
    }
}

impl Error for SpawnError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GameInitError {
    BlockedPlayerStart(GridPos),
    BlockedExit(GridPos),
    Actor(ActorBuildError),
    Registry(RegistryError),
    ProgressionRules(ProgressionRulesError),
    UnknownStatusDefinition(StatusId),
    DuplicateEquipmentSlot(EquipmentSlotId),
    UnknownStartingWeapon(WeaponId),
    TooManyStartingEquipmentEntries { equipment: usize, slots: usize },
    StartingEquipmentWeaponNotInInventory(WeaponId),
    Inventory(InventoryError),
    Equipment(EquipmentError),
}

impl Display for GameInitError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
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
            Self::UnknownStatusDefinition(status) => write!(
                formatter,
                "player ability references unknown status definition '{}'",
                status.as_str()
            ),
            Self::DuplicateEquipmentSlot(slot) => {
                write!(formatter, "duplicate player equipment slot '{slot}'")
            }
            Self::UnknownStartingWeapon(weapon) => {
                write!(formatter, "unknown player starting weapon '{weapon}'")
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
mod tests {
    use super::*;
    use crate::ai::AiProfile;
    use crate::combat::{AttackProfile, DamageType};
    use crate::effects::{AbilityProfile, ApplyStatusEffect};
    use crate::progression::{DefeatReward, ExperienceCurve, ProgressionRules};
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
    fn player_must_start_on_a_walkable_tile() {
        let result = GameState::new(parse_map("###\n#.#\n###"), GridPos::new(0, 0), 1);

        assert!(matches!(
            result,
            Err(GameInitError::BlockedPlayerStart(GridPos { x: 0, y: 0 }))
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
