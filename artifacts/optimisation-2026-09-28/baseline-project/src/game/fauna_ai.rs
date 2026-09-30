//! Fauna uses ordinary perception, pathfinding, damage and persistent actor state.
use super::*;
use crate::world::find_path;

#[path = "skittish_ai.rs"]
mod skittish;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::{ArmorRules, DamageType};
    use crate::stealth::StealthRules;

    fn arena() -> GameState {
        GameState::new_with_rules(Map::from_ascii(
            "###############\n#.............#\n#.............#\n#.............#\n#.............#\n#.............#\n###############"
        ).unwrap(), GridPos::new(2, 3), 19,
            GameRules { stealth_rules: Some(StealthRules::default()), armor_rules: Some(ArmorRules::default()), ..GameRules::default() }).unwrap()
    }

    fn animal(
        game: &mut GameState,
        at: GridPos,
        behavior: AiBehavior,
        sight: u16,
        relation: PlayerRelation,
    ) -> EntityId {
        game.spawn_actor(
            Actor::new(at, 20)
                .unwrap()
                .with_ai(
                    AiProfile::new(behavior, sight, 0, 256, 0)
                        .with_maximum_pursuit_distance(std::num::NonZeroU16::new(8).unwrap()),
                )
                .with_attack(AttackProfile::new(
                    1,
                    crate::world::DistanceMetric::Chebyshev,
                    true,
                    DamageType::Kinetic,
                    2,
                    0,
                ))
                .with_player_relation(relation),
        )
        .unwrap()
    }

    fn burrower(game: &mut GameState) -> EntityId {
        animal(
            game,
            GridPos::new(7, 3),
            AiBehavior::VibrationHunter {
                hearing_radius: 8,
                hearing_gain: 4,
                memory_turns: 4,
            },
            1,
            PlayerRelation::Hostile,
        )
    }

    #[test]
    fn fauna_burrower_waits_in_silence_then_follows_a_heard_cell_not_the_hidden_player() {
        let mut game = arena();
        let id = burrower(&mut game);
        game.process_player_command(GameCommand::Wait);
        assert_eq!(game.actors.get(id).unwrap().position(), GridPos::new(7, 3));
        game.process_player_command(GameCommand::Move(Direction::North));
        assert!(
            matches!(game.actors.get(id).unwrap().ai_state(), AiState::Listening { last_heard, .. } if last_heard == GridPos::new(2, 2))
        );
        // A silent reposition must not update the remembered destination.
        game.actors
            .get_mut(game.player)
            .unwrap()
            .set_position(GridPos::new(2, 5));
        game.process_player_command(GameCommand::Wait);
        assert!(
            matches!(game.actors.get(id).unwrap().ai_state(), AiState::Listening { last_heard, .. } if last_heard == GridPos::new(2, 2))
        );
        for _ in 0..5 {
            game.process_player_command(GameCommand::Wait);
        }
        assert_eq!(game.actors.get(id).unwrap().ai_state(), AiState::Unaware);
    }

    #[test]
    fn fauna_burrower_respects_walls_hearing_range_protection_and_sound_diversions() {
        let mut game = arena();
        let id = burrower(&mut game);
        let decoy = GridPos::new(10, 3);
        game.emit_noise(None, decoy, 10);
        game.resolve_ai_turn();
        assert_eq!(game.actors.get(id).unwrap().position(), GridPos::new(8, 3));
        assert!(
            matches!(game.actors.get(id).unwrap().ai_state(), AiState::Listening { last_heard, .. } if last_heard == decoy)
        );
        game.transient_noises.clear();
        game.map.set_protected(decoy, true).unwrap();
        game.emit_noise(None, decoy, 100);
        assert!(game.fauna_heard_position(id, 8, 4).is_none());
        game.transient_noises.clear();
        game.emit_noise(None, GridPos::new(1, 3), 100);
        assert!(game.fauna_heard_position(id, 3, 4).is_none());
        game.transient_noises.clear();
        for y in 1..6 {
            game.map
                .set_terrain(GridPos::new(5, y), Terrain::Wall)
                .unwrap();
        }
        game.actors
            .get_mut(game.player)
            .unwrap()
            .set_position(GridPos::new(4, 3));
        game.emit_noise(Some(game.player), GridPos::new(4, 3), 30);
        let integrity = game.actors.get(game.player).unwrap().integrity();
        for _ in 0..4 {
            game.resolve_ai_turn();
        }
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), integrity);
        assert!(game.actors.get(id).unwrap().position().x > 5);
    }

    #[test]
    fn fauna_grazer_never_attacks_and_its_shell_expires_and_survives_serialization() {
        let mut game = arena();
        let id = animal(
            &mut game,
            GridPos::new(3, 3),
            AiBehavior::TimidGrazer {
                shell_armor: 3,
                shelter_turns: 4,
            },
            4,
            PlayerRelation::Neutral,
        );
        let player_hp = game.actors.get(game.player).unwrap().integrity();
        game.apply_damage_to(
            Some(game.player),
            id,
            DamagePacket::new(5, DamageType::Kinetic, 0),
        )
        .unwrap();
        assert_eq!(game.actors.get(id).unwrap().integrity(), 15);
        assert_eq!(game.fauna_shell_armor(id), 3);
        let serialized = bincode::serialize(game.actors.get(id).unwrap()).unwrap();
        *game.actors.get_mut(id).unwrap() = bincode::deserialize(&serialized).unwrap();
        game.apply_damage_to(
            Some(game.player),
            id,
            DamagePacket::new(5, DamageType::Kinetic, 0),
        )
        .unwrap();
        assert_eq!(game.actors.get(id).unwrap().integrity(), 13);
        for _ in 0..8 {
            game.process_player_command(GameCommand::Wait);
        }
        assert_eq!(game.fauna_shell_armor(id), 0);
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), player_hp);
        assert_eq!(
            game.actors.get(id).unwrap().player_relation(),
            PlayerRelation::Neutral
        );
        assert!(!game.drain_events().iter().any(
            |event| matches!(event, GameEvent::AttackPerformed { attacker, .. } if *attacker == id)
        ));
    }

    #[test]
    fn fauna_pack_members_use_distinct_attack_positions_without_entering_protected_cells() {
        let mut game = arena();
        game.actors
            .get_mut(game.player)
            .unwrap()
            .set_position(GridPos::new(6, 3));
        let first = animal(
            &mut game,
            GridPos::new(3, 3),
            AiBehavior::PackHunter,
            9,
            PlayerRelation::Hostile,
        );
        let second = animal(
            &mut game,
            GridPos::new(3, 2),
            AiBehavior::PackHunter,
            9,
            PlayerRelation::Hostile,
        );
        for _ in 0..4 {
            game.resolve_ai_turn();
        }
        let a = game.actors.get(first).unwrap().position();
        let b = game.actors.get(second).unwrap().position();
        assert_ne!(a, b);
        assert!(grid_distance(a, GridPos::new(6, 3)) <= 1);
        assert!(grid_distance(b, GridPos::new(6, 3)) <= 1);
        game.map
            .set_protected(game.player_position().unwrap(), true)
            .unwrap();
        game.drain_events();
        game.resolve_ai_turn();
        assert!(
            !game
                .drain_events()
                .iter()
                .any(|event| matches!(event, GameEvent::AttackPerformed { .. }))
        );
    }
}

impl GameState {
    pub(in crate::game) fn fauna_shell_armor(&self, entity: EntityId) -> u16 {
        self.actors
            .get(entity)
            .and_then(|actor| match (actor.ai()?.behavior, actor.ai_state()) {
                (
                    AiBehavior::TimidGrazer { shell_armor, .. },
                    AiState::Sheltered {
                        remaining_turns: 1..,
                    },
                ) => Some(shell_armor),
                _ => None,
            })
            .unwrap_or(0)
    }

    pub(in crate::game) fn shelter_wounded_fauna(&mut self, entity: EntityId) {
        let Some(actor) = self.actors.get_mut(entity) else {
            return;
        };
        if let Some(AiProfile {
            behavior: AiBehavior::TimidGrazer { shelter_turns, .. },
            ..
        }) = actor.ai()
        {
            actor.set_ai_state(AiState::Sheltered {
                remaining_turns: shelter_turns,
            });
        }
    }

    pub(super) fn resolve_fauna_ai_action(
        &mut self,
        entity: EntityId,
        action: AiAction,
        target: EntityId,
        recovering: bool,
    ) -> bool {
        let Some(actor) = self.actors.get(entity).cloned() else {
            return false;
        };
        let Some(profile) = actor.ai() else {
            return false;
        };
        match profile.behavior {
            AiBehavior::SkittishForager => {
                self.resolve_skittish_fauna(entity, &actor, profile, recovering);
                true
            }
            AiBehavior::PackHunter if matches!(action, AiAction::Move(_)) => {
                let Some(at) = self
                    .actors
                    .get(target)
                    .map(Actor::position)
                    .filter(|at| self.actor_perceives_position(entity, *at))
                else {
                    return false;
                };
                let occupied: BTreeSet<_> = self
                    .actors
                    .iter()
                    .filter_map(|(id, other)| (id != entity).then_some(other.position()))
                    .collect();
                // Different reachable attack positions, not a shared omniscient pack target.
                let next = at
                    .cardinal_neighbors()
                    .into_iter()
                    .filter(|position| {
                        self.map.is_walkable(*position)
                            && !self.map.is_protected(*position)
                            && !occupied.contains(position)
                            && actor
                                .ai_home()
                                .zip(profile.maximum_pursuit_distance())
                                .is_none_or(|(home, limit)| grid_distance(home, *position) <= limit)
                    })
                    .filter_map(|position| {
                        let path = find_path(
                            &self.map,
                            actor.position(),
                            position,
                            profile.maximum_path_search,
                            |cell| {
                                !occupied.contains(&cell)
                                    && !self.map.is_protected(cell)
                                    && actor
                                        .ai_home()
                                        .zip(profile.maximum_pursuit_distance())
                                        .is_none_or(|(home, limit)| {
                                            grid_distance(home, cell) <= limit
                                        })
                            },
                        )?;
                        Some((path.len(), position, path.get(1).copied()?))
                    })
                    .min()
                    .map(|(_, _, next)| next);
                if let Some(next) = next
                    && let Some(direction) = Direction::from_delta(
                        next.x - actor.position().x,
                        next.y - actor.position().y,
                    )
                {
                    return self.move_ai_entity(entity, direction).is_ok();
                }
                false
            }
            AiBehavior::VibrationHunter {
                hearing_radius,
                hearing_gain,
                memory_turns,
            } => {
                let heard = self.fauna_heard_position(entity, hearing_radius, hearing_gain);
                let memory =
                    heard
                        .map(|at| (at, memory_turns))
                        .or_else(|| match actor.ai_state() {
                            AiState::Listening {
                                remaining_turns,
                                last_heard,
                            } if remaining_turns > 0 => Some((last_heard, remaining_turns - 1)),
                            _ => None,
                        });
                self.actors
                    .get_mut(entity)
                    .unwrap()
                    .set_ai_state(match memory {
                        Some((last_heard, remaining_turns)) if remaining_turns > 0 => {
                            AiState::Listening {
                                last_heard,
                                remaining_turns,
                            }
                        }
                        _ => AiState::Unaware,
                    });
                // Only immediate optical contact authorizes a bite. Sound alone never
                // authorizes attacks through walls or reveals a moving target's position.
                let contact = self
                    .actors
                    .iter()
                    .filter(|(id, other)| {
                        (*id == self.player
                            || other
                                .drone()
                                .is_some_and(|drone| drone.controller() == self.player))
                            && !self.map.is_protected(other.position())
                            && self.actor_perceives_position(entity, other.position())
                    })
                    .map(|(id, _)| id)
                    .next();
                if !recovering
                    && let Some(contact) = contact
                    && self
                        .perform_attack(entity, profile.preferred_attack_slot, contact)
                        .is_ok()
                {
                    return true;
                }
                if let Some((at, _)) = memory.filter(|(_, turns)| *turns > 0) {
                    self.fauna_move_toward(entity, &actor, profile, at);
                }
                true
            }
            AiBehavior::TimidGrazer { .. } => {
                if let AiState::Sheltered { remaining_turns } = actor.ai_state() {
                    self.actors
                        .get_mut(entity)
                        .unwrap()
                        .set_ai_state(if remaining_turns > 1 {
                            AiState::Sheltered {
                                remaining_turns: remaining_turns - 1,
                            }
                        } else {
                            AiState::Unaware
                        });
                } else if self.turn % 3 == 0 {
                    // Small, deterministic grazing steps; no target or player position.
                    let directions = [
                        Direction::North,
                        Direction::East,
                        Direction::South,
                        Direction::West,
                    ];
                    let offset =
                        ((self.turn / 3) as usize + actor.position().x.unsigned_abs() as usize) % 4;
                    for step in 0..4 {
                        let direction = directions[(offset + step) % 4];
                        let at = actor.position().step(direction);
                        if !self.map.is_protected(at)
                            && actor
                                .ai_home()
                                .zip(profile.maximum_pursuit_distance())
                                .is_none_or(|(home, limit)| grid_distance(home, at) <= limit)
                            && self.move_ai_entity(entity, direction).is_ok()
                        {
                            break;
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }

    fn fauna_move_toward(
        &mut self,
        entity: EntityId,
        actor: &Actor,
        profile: AiProfile,
        at: GridPos,
    ) {
        let occupied = self
            .actors
            .iter()
            .filter_map(|(id, other)| (id != entity).then_some(other.position()))
            .collect();
        if let AiAction::Move(direction) = decide_known_action(AiSituation {
            map: &self.map,
            actor_position: actor.position(),
            home_position: actor.ai_home(),
            target_position: at,
            occupied_positions: &occupied,
            profile,
            preferred_attack: None,
        }) {
            let _ = self.move_ai_entity(entity, direction);
        }
    }

    fn fauna_heard_position(&self, entity: EntityId, radius: u16, gain: u16) -> Option<GridPos> {
        let rules = self.rules.stealth_rules?;
        let listener = self.actors.get(entity)?.position();
        self.transient_noises
            .iter()
            .map(|noise| (noise.intensity, noise.at))
            .chain(
                self.sound_emitters
                    .iter()
                    .map(|emitter| (emitter.intensity(), emitter.position())),
            )
            .filter(|(intensity, at)| {
                *at != listener
                    && !self.map.is_protected(*at)
                    && grid_distance(listener, *at) <= radius
                    && rules.sound_reaches_on_map(
                        &self.map,
                        intensity.saturating_mul(gain),
                        *at,
                        listener,
                    )
            })
            .min_by_key(|(intensity, at)| {
                (
                    std::cmp::Reverse(*intensity),
                    grid_distance(listener, *at),
                    *at,
                )
            })
            .map(|(_, at)| at)
    }
}
