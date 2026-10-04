//! Small, stateful combat roles sharing the ordinary attack and movement rules.
use super::*;
use crate::ai::{AiBehavior, AiProfile};
use crate::social::PlayerRelation;

#[path = "fauna_ai.rs"]
mod fauna;

#[path = "strange_fauna.rs"]
mod strange_fauna;

#[path = "bestiary_encounters.rs"]
mod bestiary_encounters;

#[cfg(test)]
#[path = "deep_encounter_tests.rs"]
mod deep_encounter_tests;
#[cfg(test)]
#[path = "grasper_tests.rs"]
mod grasper_tests;
#[cfg(test)]
#[path = "spitter_tests.rs"]
mod spitter_tests;
#[cfg(test)]
#[path = "sweeper_tests.rs"]
mod sweeper_tests;

impl GameState {
    pub(in crate::game) fn resolve_tactical_ai_action(
        &mut self,
        entity: EntityId,
        action: AiAction,
        target: EntityId,
        recovering: bool,
    ) -> bool {
        if self.resolve_bestiary_encounter(entity, action, target, recovering) {
            return true;
        }
        if self.resolve_fauna_ai_action(entity, action, target, recovering) {
            return true;
        }
        let Some(actor) = self.actors.get(entity).cloned() else {
            return false;
        };
        let Some(profile) = actor.ai() else {
            return false;
        };
        if profile.behavior == AiBehavior::TelegraphedEcho
            && target == entity
            && !matches!(actor.ai_state(), AiState::Aiming { .. })
        {
            // Inactive zones have no remote player. Expire reports without
            // starting attacks or preserving an investigation indefinitely.
            let state = match actor.ai_state() {
                AiState::Responding {
                    remaining_turns,
                    incident,
                } if remaining_turns > 1 => AiState::Responding {
                    remaining_turns: remaining_turns - 1,
                    incident,
                },
                _ => AiState::Unaware,
            };
            self.actors.get_mut(entity).unwrap().set_ai_state(state);
            return true;
        }
        if let AiState::ChargingAttack {
            origin,
            target_at,
            remaining_turns,
        } = actor.ai_state()
        {
            if recovering || actor.position() != origin {
                self.actors
                    .get_mut(entity)
                    .unwrap()
                    .set_ai_state(AiState::Unaware);
            } else if remaining_turns > 0 {
                self.actors
                    .get_mut(entity)
                    .unwrap()
                    .set_ai_state(AiState::ChargingAttack {
                        origin,
                        target_at,
                        remaining_turns: remaining_turns - 1,
                    });
            } else {
                self.actors
                    .get_mut(entity)
                    .unwrap()
                    .set_ai_state(AiState::Unaware);
                let _ =
                    self.perform_committed_attack(entity, profile.preferred_attack_slot, target_at);
            }
            return true;
        }
        if let AiState::Aiming { origin, target_at } = actor.ai_state() {
            self.actors
                .get_mut(entity)
                .unwrap()
                .set_ai_state(AiState::Unaware);
            if !recovering && actor.position() == origin {
                // No live target is consulted: moving away really evades the shot.
                let _ =
                    self.perform_committed_attack(entity, profile.preferred_attack_slot, target_at);
            }
            return true;
        }
        if recovering {
            // Heavy fauna exposes a stationary recovery window, not another chase.
            return matches!(
                profile.behavior,
                AiBehavior::TelegraphedBiter
                    | AiBehavior::TelegraphedSweeper
                    | AiBehavior::TelegraphedGrasper
                    | AiBehavior::TelegraphedResonator
                    | AiBehavior::TelegraphedProjector
                    | AiBehavior::TelegraphedEcho
                    | AiBehavior::TelegraphedSpitter
            );
        }
        if matches!(
            profile.behavior,
            AiBehavior::TelegraphedShooter
                | AiBehavior::TelegraphedBiter
                | AiBehavior::TelegraphedSweeper
                | AiBehavior::TelegraphedGrasper
                | AiBehavior::TelegraphedResonator
                | AiBehavior::TelegraphedProjector
                | AiBehavior::TelegraphedEcho
                | AiBehavior::TelegraphedSpitter
        ) {
            if let AiAction::Attack { slot } = action
                && let Ok(prepared) = self.prepare_targeted_attack(entity, slot, target)
                && self.ensure_prepared_attack_usage(&prepared, 1).is_ok()
            {
                self.actors
                    .get_mut(entity)
                    .unwrap()
                    .set_ai_state(match profile.behavior {
                        AiBehavior::TelegraphedResonator | AiBehavior::TelegraphedProjector => {
                            AiState::ChargingAttack {
                                origin: prepared.origin,
                                target_at: prepared.target_at,
                                remaining_turns: if profile.behavior
                                    == AiBehavior::TelegraphedResonator
                                {
                                    2
                                } else {
                                    1
                                },
                            }
                        }
                        _ => AiState::Aiming {
                            origin: prepared.origin,
                            target_at: prepared.target_at,
                        },
                    });
                self.events.push(GameEvent::AttackTelegraphed {
                    attacker: entity,
                    origin: prepared.origin,
                    target_at: prepared.target_at,
                });
                return true;
            }
            if matches!(
                profile.behavior,
                AiBehavior::TelegraphedBiter
                    | AiBehavior::TelegraphedSweeper
                    | AiBehavior::TelegraphedGrasper
                    | AiBehavior::TelegraphedResonator
                    | AiBehavior::TelegraphedProjector
                    | AiBehavior::TelegraphedEcho
                    | AiBehavior::TelegraphedSpitter
            ) && matches!(action, AiAction::Attack { .. })
            {
                // A rejected wind-up cannot fall back to an unannounced bite.
                return true;
            }
        }
        if let AiBehavior::FieldMedic {
            restoration,
            supplies,
        } = profile.behavior
        {
            return self.try_ally_care(entity, &actor, profile, restoration, supplies);
        }
        false
    }

    /// The committed footprint, recomputed against current cover without
    /// following the target. Presentation must still filter by visibility.
    pub fn telegraphed_attack_cells(
        &self,
        attacker: EntityId,
    ) -> Vec<crate::combat::AttackAreaCell> {
        let Some(actor) = self.actors.get(attacker) else {
            return Vec::new();
        };
        let (origin, target_at) = match actor.ai_state() {
            AiState::Encounter(crate::ai::EncounterState::Ring { origin, .. }) => (origin, origin),
            AiState::Aiming { origin, target_at }
            | AiState::ChargingAttack {
                origin, target_at, ..
            } => (origin, target_at),
            _ => return Vec::new(),
        };
        let Some(profile) = actor.ai() else {
            return Vec::new();
        };
        if actor.position() != origin || actor.recovery_remaining().is_some() {
            return Vec::new();
        }
        self.prepare_committed_attack(attacker, profile.preferred_attack_slot, target_at)
            .map(|prepared| {
                prepared
                    .affected_cells
                    .into_iter()
                    .filter(|cell| !self.map.is_protected(cell.position))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn prepare_committed_attack(
        &self,
        attacker: EntityId,
        slot: u8,
        target_at: GridPos,
    ) -> Result<PreparedAttack, AttackError> {
        let (actor, attack, weapon, weapon_effects, module_use) =
            self.attack_details(attacker, slot)?;
        let origin = actor.position();
        let pulse = matches!(attack.area(), AttackArea::Pulse(_))
            || actor
                .ai()
                .is_some_and(|ai| ai.behavior == AiBehavior::ExpandingRing);
        if self.map.is_protected(origin) || (!pulse && self.map.is_protected(target_at)) {
            return Err(AttackError::ProtectedZone);
        }
        if !attack.is_in_range(origin, target_at) {
            return Err(AttackError::PositionOutOfRange(target_at));
        }
        if !pulse && !has_line_of_sight(&self.map, origin, target_at, true) {
            return Err(AttackError::NoLineOfSightAt(target_at));
        }
        Ok(PreparedAttack {
            attacker,
            target: self
                .actors
                .entity_at(target_at)
                .filter(|id| *id != attacker),
            slot,
            origin,
            target_at,
            attack,
            weapon,
            weapon_effects,
            module_use,
            affected_cells: self.committed_footprint(&actor, attack, origin, target_at),
            forced_movement: None,
            technique_on_hit_effect: None,
            damage_target: AttackDamageTarget::Body,
        })
    }

    fn perform_committed_attack(
        &mut self,
        attacker: EntityId,
        slot: u8,
        target_at: GridPos,
    ) -> Result<(), AttackError> {
        let prepared = self.prepare_committed_attack(attacker, slot, target_at)?;
        let recovery = prepared.attack.recovery_after_attack();
        let grasp = self
            .actors
            .get(attacker)
            .and_then(Actor::ai)
            .is_some_and(|ai| ai.behavior == AiBehavior::TelegraphedGrasper);
        self.spend_prepared_attack_usage(&prepared, 1)?;
        let resolution = self.resolve_prepared_attack(prepared, ActionOrigin::Normal)?;
        if grasp {
            // Reuse the ordinary non-refreshing hindrance and its brief immunity.
            // This is a natural attack: no hidden weapon or duplicated loot.
            let status: StatusId = "core:locomotion_hindered".parse().unwrap();
            if self.rules.statuses.get(&status).is_some() {
                for target in resolution.hit_targets {
                    let can_grasp = self
                        .actors
                        .get(attacker)
                        .zip(self.actors.get(target))
                        .is_some_and(|(source, victim)| {
                            victim
                                .locomotion_profile()
                                .is_some_and(|profile| profile.hindrance_compatible())
                                && source.position().x.abs_diff(victim.position().x) <= 1
                                && source.position().y.abs_diff(victim.position().y) <= 1
                                && has_line_of_sight(
                                    &self.map,
                                    source.position(),
                                    victim.position(),
                                    true,
                                )
                        });
                    if can_grasp {
                        let _ = self.apply_status_to(
                            Some(attacker),
                            target,
                            &ApplyStatusEffect::new(status.clone(), 1).unwrap(),
                        );
                    }
                }
            }
        }
        if let Some(recovery) = recovery {
            self.start_action_recovery(attacker, recovery);
        }
        Ok(())
    }

    fn try_ally_care(
        &mut self,
        medic: EntityId,
        actor: &Actor,
        profile: AiProfile,
        restoration: u16,
        supplies: u16,
    ) -> bool {
        let remaining = match actor.ai_state() {
            AiState::SupportStock { remaining } => remaining,
            _ => supplies,
        };
        if restoration == 0
            || remaining == 0
            || actor.player_relation() != PlayerRelation::Hostile
            || self.map.is_protected(actor.position())
            || matches!(
                actor.ai_state(),
                AiState::Returning | AiState::Cooldown { .. }
            )
        {
            return false;
        }
        let target = self
            .actors
            .iter()
            .filter(|(id, ally)| {
                *id != medic
                    && *id != self.player
                    && ally.player_relation() == PlayerRelation::Hostile
                    && ally.affiliation() == actor.affiliation()
                    && ally.drone().is_none()
                    && !ally.ai().is_some_and(|ai| {
                        matches!(
                            ai.behavior,
                            AiBehavior::PackHunter
                                | AiBehavior::VibrationHunter { .. }
                                | AiBehavior::TimidGrazer { .. }
                                | AiBehavior::SkittishForager
                                | AiBehavior::TelegraphedBiter
                                | AiBehavior::TelegraphedSweeper
                                | AiBehavior::TelegraphedGrasper
                                | AiBehavior::TelegraphedResonator
                                | AiBehavior::TelegraphedProjector
                                | AiBehavior::Watcher
                                | AiBehavior::TelegraphedEcho
                                | AiBehavior::TelegraphedSpitter
                                | AiBehavior::DirectionalGuard
                                | AiBehavior::ExpandingRing
                                | AiBehavior::AlternatingStalker
                                | AiBehavior::NestDiver
                                | AiBehavior::Riveter
                        )
                    })
                    && ally.integrity() < ally.maximum_integrity()
                    && !self.map.is_protected(ally.position())
                    && self.actor_perceives_position(medic, ally.position())
                    && actor
                        .ai_home()
                        .zip(profile.maximum_pursuit_distance())
                        .is_none_or(|(home, limit)| grid_distance(home, ally.position()) <= limit)
            })
            .min_by_key(|(id, ally)| (grid_distance(actor.position(), ally.position()), *id))
            .map(|(id, ally)| (id, ally.position()));
        let Some((target, position)) = target else {
            return false;
        };
        if grid_distance(actor.position(), position) <= 1 {
            self.actors
                .get_mut(medic)
                .unwrap()
                .set_ai_state(AiState::SupportStock {
                    remaining: remaining - 1,
                });
            let amount = self
                .actors
                .get_mut(target)
                .unwrap()
                .restore_integrity(restoration);
            self.events.push(GameEvent::AllyHealed {
                medic,
                target,
                amount,
            });
            return true;
        }
        let occupied_positions = self
            .actors
            .iter()
            .filter_map(|(id, other)| (id != medic).then_some(other.position()))
            .collect();
        if let AiAction::Move(direction) = decide_known_action(AiSituation {
            map: &self.map,
            actor_position: actor.position(),
            home_position: actor.ai_home(),
            target_position: position,
            occupied_positions: &occupied_positions,
            profile,
            preferred_attack: None,
        }) {
            return self.move_ai_entity(medic, direction).is_ok();
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::DamageType;
    use crate::world::DistanceMetric;

    fn arena() -> GameState {
        GameState::new(
            Map::from_ascii("#########\n#.......#\n#.......#\n#.......#\n#########").unwrap(),
            GridPos::new(2, 2),
            17,
        )
        .unwrap()
    }

    fn shooter(game: &mut GameState) -> EntityId {
        game.spawn_actor(
            Actor::new(GridPos::new(6, 2), 10)
                .unwrap()
                .with_ai(AiProfile::new(AiBehavior::TelegraphedShooter, 8, 0, 256, 0))
                .with_attack(AttackProfile::new(
                    6,
                    DistanceMetric::Euclidean,
                    true,
                    DamageType::Piercing,
                    4,
                    0,
                ))
                .with_player_relation(PlayerRelation::Hostile),
        )
        .unwrap()
    }

    fn heavy_biter(game: &mut GameState, at: GridPos) -> EntityId {
        game.spawn_actor(
            Actor::new(at, 20)
                .unwrap()
                .with_ai(AiProfile::new(AiBehavior::TelegraphedBiter, 7, 0, 256, 0))
                .with_attack(
                    AttackProfile::new(
                        1,
                        DistanceMetric::Chebyshev,
                        true,
                        DamageType::Piercing,
                        6,
                        0,
                    )
                    .with_recovery_after_attack(crate::time::TimeUnits::new(2).unwrap()),
                )
                .with_player_relation(PlayerRelation::Hostile),
        )
        .unwrap()
    }

    #[test]
    fn heavy_fauna_bite_commits_to_a_cell_and_leaves_two_stationary_recovery_actions() {
        let mut game = arena();
        let enemy = heavy_biter(&mut game, GridPos::new(3, 2));
        let hp = game.actors.get(game.player).unwrap().integrity();
        game.process_player_command(GameCommand::Wait);
        assert!(
            matches!(game.actors.get(enemy).unwrap().ai_state(), AiState::Aiming { target_at, .. } if target_at == GridPos::new(2, 2))
        );
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
        game.drain_events();
        // Still in melee range diagonally, but no longer on the committed cell.
        game.process_player_command(GameCommand::Move(Direction::North));
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
        assert!(game.drain_events().iter().any(|event| matches!(event,
            GameEvent::AttackPerformed { attacker, target: None, target_at, .. }
            if *attacker == enemy && *target_at == GridPos::new(2, 2))));
        assert_eq!(
            game.actors
                .get(enemy)
                .unwrap()
                .recovery_remaining()
                .map(|time| time.get()),
            Some(2)
        );
        for remaining in [Some(1), None] {
            let bytes = bincode::serialize(game.actors.get(enemy).unwrap()).unwrap();
            *game.actors.get_mut(enemy).unwrap() = bincode::deserialize(&bytes).unwrap();
            game.process_player_command(GameCommand::Wait);
            assert_eq!(
                game.actors.get(enemy).unwrap().position(),
                GridPos::new(3, 2)
            );
            assert_eq!(
                game.actors
                    .get(enemy)
                    .unwrap()
                    .recovery_remaining()
                    .map(|time| time.get()),
                remaining
            );
            assert!(!game.drain_events().iter().any(|event| matches!(event,
                GameEvent::AttackPerformed { attacker, .. } | GameEvent::AttackTelegraphed { attacker, .. } if *attacker == enemy)));
        }
        game.process_player_command(GameCommand::Wait);
        assert!(matches!(
            game.actors.get(enemy).unwrap().ai_state(),
            AiState::Aiming { .. }
        ));
    }

    #[test]
    fn heavy_fauna_windup_survives_serialization_and_hits_only_after_the_warning() {
        let mut game = arena();
        let enemy = heavy_biter(&mut game, GridPos::new(3, 2));
        let player = game.actors.get_mut(game.player).unwrap();
        *player = player.clone().with_evasion_disabled();
        let hp = player.integrity();
        game.process_player_command(GameCommand::Wait);
        let bytes = bincode::serialize(game.actors.get(enemy).unwrap()).unwrap();
        *game.actors.get_mut(enemy).unwrap() = bincode::deserialize(&bytes).unwrap();
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
        game.process_player_command(GameCommand::Wait);
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp - 6);
        assert!(
            game.actors
                .get(enemy)
                .unwrap()
                .recovery_remaining()
                .is_some()
        );
    }

    #[test]
    fn heavy_fauna_committed_bite_respects_displacement_new_cover_and_protection() {
        for mode in 0..3 {
            let mut game = arena();
            let enemy = heavy_biter(&mut game, GridPos::new(3, 3));
            game.process_player_command(GameCommand::Wait);
            assert!(matches!(
                game.actors.get(enemy).unwrap().ai_state(),
                AiState::Aiming { .. }
            ));
            match mode {
                0 => game
                    .actors
                    .get_mut(enemy)
                    .unwrap()
                    .set_position(GridPos::new(4, 3)),
                1 => {
                    game.map
                        .set_terrain(GridPos::new(3, 2), Terrain::Wall)
                        .unwrap();
                    game.map
                        .set_terrain(GridPos::new(2, 3), Terrain::Wall)
                        .unwrap();
                }
                _ => game.map.set_protected(GridPos::new(2, 2), true).unwrap(),
            }
            game.drain_events();
            game.process_player_command(GameCommand::Wait);
            assert!(!game.drain_events().iter().any(|event| matches!(event,
                GameEvent::AttackPerformed { attacker, .. } if *attacker == enemy)));
            assert_eq!(game.actors.get(enemy).unwrap().ai_state(), AiState::Unaware);
        }
    }

    #[test]
    fn telegraphed_shot_allows_one_action_and_does_not_follow_the_player() {
        let mut game = arena();
        let enemy = shooter(&mut game);
        let before = game.actors.get(game.player).unwrap().integrity();
        game.process_player_command(GameCommand::Wait);
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), before);
        assert!(
            matches!(game.actors.get(enemy).unwrap().ai_state(), AiState::Aiming { target_at, .. } if target_at == GridPos::new(2, 2))
        );
        assert!(game.drain_events().iter().any(|event| matches!(event, GameEvent::AttackTelegraphed { attacker, .. } if *attacker == enemy)));
        game.process_player_command(GameCommand::Move(Direction::North));
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), before);
        assert!(game.drain_events().iter().any(|event| matches!(event, GameEvent::AttackPerformed { attacker, target: None, target_at, .. } if *attacker == enemy && *target_at == GridPos::new(2, 2))));
    }

    #[test]
    fn telegraphed_shot_hits_an_occupant_who_stays() {
        let mut game = arena();
        shooter(&mut game);
        let player = game.actors.get_mut(game.player).unwrap();
        *player = player.clone().with_evasion_disabled();
        let before = player.integrity();
        game.process_player_command(GameCommand::Wait);
        game.process_player_command(GameCommand::Wait);
        assert!(game.actors.get(game.player).unwrap().integrity() < before);
    }

    #[test]
    fn telegraph_is_serialized_and_blocked_by_new_cover_or_displacement() {
        for displaced in [false, true] {
            let mut game = arena();
            let enemy = shooter(&mut game);
            game.process_player_command(GameCommand::Wait);
            let bytes = bincode::serialize(game.actors.get(enemy).unwrap()).unwrap();
            let actor: Actor = bincode::deserialize(&bytes).unwrap();
            assert_eq!(&actor, game.actors.get(enemy).unwrap());
            if displaced {
                game.actors
                    .get_mut(enemy)
                    .unwrap()
                    .set_position(GridPos::new(6, 3));
            } else {
                game.map
                    .set_terrain(GridPos::new(4, 2), Terrain::Wall)
                    .unwrap();
            }
            game.drain_events();
            game.process_player_command(GameCommand::Wait);
            assert!(!game.drain_events().iter().any(|event| matches!(event, GameEvent::AttackPerformed { attacker, .. } if *attacker == enemy)));
            assert_eq!(game.actors.get(enemy).unwrap().ai_state(), AiState::Unaware);
        }
    }

    fn person(position: GridPos) -> Actor {
        Actor::new(position, 12)
            .unwrap()
            .with_player_relation(PlayerRelation::Hostile)
            .with_ai(AiProfile::idle())
    }

    #[test]
    fn medic_spends_a_turn_and_finite_supplies_without_overhealing() {
        let mut game = arena();
        let medic = game
            .spawn_actor(person(GridPos::new(6, 2)).with_ai(AiProfile::new(
                AiBehavior::FieldMedic {
                    restoration: 3,
                    supplies: 2,
                },
                8,
                0,
                256,
                0,
            )))
            .unwrap();
        let ally = game.spawn_actor(person(GridPos::new(6, 3))).unwrap();
        game.actors.get_mut(ally).unwrap().apply_damage(8);
        for _ in 0..3 {
            game.process_player_command(GameCommand::Wait);
            // Suspending after each action must never refill the stock.
            let bytes = bincode::serialize(game.actors.get(medic).unwrap()).unwrap();
            *game.actors.get_mut(medic).unwrap() = bincode::deserialize(&bytes).unwrap();
        }
        assert_eq!(game.actors.get(ally).unwrap().integrity(), 10);
        assert_eq!(
            game.actors.get(medic).unwrap().ai_state(),
            AiState::SupportStock { remaining: 0 }
        );
        assert_eq!(
            game.drain_events()
                .iter()
                .filter(|event| matches!(event, GameEvent::AllyHealed { .. }))
                .count(),
            2
        );
    }

    #[test]
    fn medic_does_not_heal_neutrals_hidden_or_protected_people() {
        for mode in 0..6 {
            let mut game = arena();
            game.map
                .set_protected(game.player_position().unwrap(), true)
                .unwrap();
            let medic = game
                .spawn_actor(person(GridPos::new(6, 2)).with_ai(AiProfile::new(
                    AiBehavior::FieldMedic {
                        restoration: 3,
                        supplies: 2,
                    },
                    8,
                    0,
                    256,
                    0,
                )))
                .unwrap();
            let ally = game.spawn_actor(person(GridPos::new(4, 2))).unwrap();
            let target = game.actors.get_mut(ally).unwrap();
            target.apply_damage(5);
            match mode {
                0 => *target = target.clone().with_player_relation(PlayerRelation::Neutral),
                1 => {
                    for y in 1..4 {
                        game.map
                            .set_terrain(GridPos::new(5, y), Terrain::Wall)
                            .unwrap();
                    }
                }
                2 => game.map.set_protected(GridPos::new(4, 2), true).unwrap(),
                _ => {
                    let animal = game.actors.get(ally).unwrap().clone();
                    let behavior = if mode == 3 {
                        AiBehavior::TelegraphedBiter
                    } else if mode == 4 {
                        AiBehavior::TelegraphedSweeper
                    } else {
                        AiBehavior::TelegraphedGrasper
                    };
                    *game.actors.get_mut(ally).unwrap() =
                        animal.with_ai(AiProfile::new(behavior, 7, 0, 256, 0));
                }
            }
            game.process_player_command(GameCommand::Wait);
            assert_eq!(game.actors.get(ally).unwrap().integrity(), 7);
            assert_eq!(
                game.actors.get(medic).unwrap().position(),
                GridPos::new(6, 2)
            );
            assert_eq!(game.actors.get(medic).unwrap().ai_state(), AiState::Unaware);
        }
    }

    #[test]
    fn medic_moves_to_a_visible_wounded_ally_before_healing() {
        let mut game = arena();
        let medic = game
            .spawn_actor(person(GridPos::new(6, 2)).with_ai(AiProfile::new(
                AiBehavior::FieldMedic {
                    restoration: 3,
                    supplies: 2,
                },
                8,
                0,
                256,
                0,
            )))
            .unwrap();
        let ally = game.spawn_actor(person(GridPos::new(4, 2))).unwrap();
        game.actors.get_mut(ally).unwrap().apply_damage(1);
        game.process_player_command(GameCommand::Wait);
        assert_eq!(
            game.actors.get(medic).unwrap().position(),
            GridPos::new(5, 2)
        );
        assert_eq!(game.actors.get(ally).unwrap().integrity(), 11);
        game.process_player_command(GameCommand::Wait);
        assert_eq!(game.actors.get(ally).unwrap().integrity(), 12);
        assert_eq!(
            game.actors.get(medic).unwrap().ai_state(),
            AiState::SupportStock { remaining: 1 }
        );
    }
}
