//! Escape is a local, bounded search, not a pursuit with its sign reversed.
use super::*;
use std::collections::VecDeque;

enum Escape {
    Step(Direction),
    Trapped,
    SearchLimit,
}

impl GameState {
    pub(super) fn resolve_skittish_fauna(
        &mut self,
        entity: EntityId,
        actor: &Actor,
        profile: AiProfile,
        recovering: bool,
    ) {
        let threats = self
            .actors
            .iter()
            .filter_map(|(id, other)| {
                ((id == self.player
                    || other
                        .drone()
                        .is_some_and(|drone| drone.controller() == self.player))
                    && !self.map.is_protected(other.position())
                    && self.actor_perceives_position(entity, other.position()))
                .then_some((id, other.position()))
            })
            .collect::<Vec<_>>();
        if threats.is_empty() || self.map.is_protected(actor.position()) {
            self.actors
                .get_mut(entity)
                .unwrap()
                .set_ai_state(AiState::Unaware);
            return;
        }
        match self.fauna_escape(
            entity,
            actor.position(),
            profile.maximum_path_search,
            &threats,
        ) {
            Escape::Step(direction) => {
                self.actors
                    .get_mut(entity)
                    .unwrap()
                    .set_ai_state(AiState::Fleeing);
                // A failed move (for example immobilization) is not permission to bite.
                let _ = self.move_ai_entity(entity, direction);
            }
            Escape::SearchLimit => {
                // An incomplete search must never be interpreted as proof of a trap.
                self.actors
                    .get_mut(entity)
                    .unwrap()
                    .set_ai_state(AiState::Fleeing);
            }
            Escape::Trapped => {
                self.actors
                    .get_mut(entity)
                    .unwrap()
                    .set_ai_state(AiState::Cornered);
                if !recovering {
                    let contact = threats
                        .iter()
                        .filter(|(_, at)| grid_distance(actor.position(), *at) <= 1)
                        .min_by_key(|(id, at)| (grid_distance(actor.position(), *at), *id));
                    if let Some((target, _)) = contact {
                        let _ = self.perform_attack(entity, profile.preferred_attack_slot, *target);
                    }
                }
            }
        }
    }

    fn fauna_escape(
        &self,
        entity: EntityId,
        origin: GridPos,
        maximum_search: usize,
        threats: &[(EntityId, GridPos)],
    ) -> Escape {
        let distance = |at| {
            threats
                .iter()
                .map(|(_, threat)| grid_distance(at, *threat))
                .min()
                .unwrap()
        };
        let initial_distance = distance(origin);
        let occupied: BTreeSet<_> = self
            .actors
            .iter()
            .filter_map(|(id, actor)| (id != entity).then_some(actor.position()))
            .collect();
        let mut visited = BTreeSet::from([origin]);
        let mut queue = VecDeque::from([(origin, None)]);
        // Stable cardinal order and no RNG; no live hidden target enters this search.
        for _ in 0..maximum_search.min(4096) {
            let Some((at, first_step)) = queue.pop_front() else {
                return Escape::Trapped;
            };
            if distance(at) > initial_distance {
                return Escape::Step(first_step.unwrap());
            }
            for direction in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ] {
                let next = at.step(direction);
                if self.map.is_walkable(next)
                    && !self.map.is_protected(next)
                    && !occupied.contains(&next)
                    && distance(next) >= initial_distance
                    && visited.insert(next)
                {
                    queue.push_back((next, first_step.or(Some(direction))));
                }
            }
        }
        if queue.is_empty() {
            Escape::Trapped
        } else {
            Escape::SearchLimit
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::DamageType;

    fn arena(source: &str, player: GridPos, animal: GridPos) -> (GameState, EntityId) {
        let mut game = GameState::new(Map::from_ascii(source).unwrap(), player, 19).unwrap();
        let id = game
            .spawn_actor(
                Actor::new(animal, 10)
                    .unwrap()
                    .with_ai(AiProfile::new(AiBehavior::SkittishForager, 5, 0, 128, 0))
                    .with_attack(AttackProfile::new(
                        1,
                        crate::world::DistanceMetric::Chebyshev,
                        true,
                        DamageType::Piercing,
                        2,
                        0,
                    ))
                    .with_player_relation(PlayerRelation::Neutral),
            )
            .unwrap();
        let player = game.actors.get_mut(game.player).unwrap();
        *player = player.clone().with_evasion_disabled();
        (game, id)
    }

    fn attacks(game: &mut GameState, id: EntityId) -> bool {
        game.drain_events().iter().any(|event| {
            matches!(event,
            GameEvent::AttackPerformed { attacker, .. } if *attacker == id)
        })
    }

    #[test]
    fn fauna_forager_prefers_escape_and_uses_a_sideways_detour() {
        for blocked in [false, true] {
            let (mut game, id) = arena(
                "#######\n#.....#\n#.....#\n#.....#\n#######",
                GridPos::new(2, 2),
                GridPos::new(3, 2),
            );
            if blocked {
                game.map
                    .set_terrain(GridPos::new(4, 2), Terrain::Wall)
                    .unwrap();
            }
            game.resolve_ai_turn();
            assert_eq!(game.actors.get(id).unwrap().ai_state(), AiState::Fleeing);
            assert_eq!(
                game.actors.get(id).unwrap().position(),
                if blocked {
                    GridPos::new(3, 1)
                } else {
                    GridPos::new(4, 2)
                }
            );
            assert!(!attacks(&mut game, id));
            // State survives serialization without becoming permanently hostile.
            let bytes = bincode::serialize(game.actors.get(id).unwrap()).unwrap();
            let restored: Actor = bincode::deserialize(&bytes).unwrap();
            assert_eq!(&restored, game.actors.get(id).unwrap());
            assert_eq!(restored.player_relation(), PlayerRelation::Neutral);
        }
    }

    #[test]
    fn fauna_forager_bites_only_when_trapped_then_uses_the_reopened_exit() {
        let (mut game, id) = arena(
            "#######\n#.....#\n###.###\n#######",
            GridPos::new(3, 1),
            GridPos::new(3, 2),
        );
        let before = game.actors.get(game.player).unwrap().integrity();
        game.resolve_ai_turn();
        assert_eq!(game.actors.get(id).unwrap().ai_state(), AiState::Cornered);
        assert!(attacks(&mut game, id));
        assert!(game.actors.get(game.player).unwrap().integrity() < before);
        game.process_player_command(GameCommand::Move(Direction::West));
        assert_eq!(game.actors.get(id).unwrap().position(), GridPos::new(3, 1));
        assert_eq!(game.actors.get(id).unwrap().ai_state(), AiState::Fleeing);
        assert!(!attacks(&mut game, id));
    }

    #[test]
    fn fauna_forager_ignores_hidden_or_protected_threats_and_never_uses_a_safe_shelter() {
        for protected in [false, true] {
            let (mut game, id) = arena(
                "#######\n#.....#\n#.....#\n#.....#\n#######",
                GridPos::new(1, 2),
                GridPos::new(3, 2),
            );
            if protected {
                game.map.set_protected(GridPos::new(1, 2), true).unwrap();
            } else {
                for y in 1..4 {
                    game.map
                        .set_terrain(GridPos::new(2, y), Terrain::Wall)
                        .unwrap();
                }
            }
            game.actors
                .get_mut(id)
                .unwrap()
                .set_ai_state(AiState::Cornered);
            game.resolve_ai_turn();
            assert_eq!(game.actors.get(id).unwrap().position(), GridPos::new(3, 2));
            assert_eq!(game.actors.get(id).unwrap().ai_state(), AiState::Unaware);
            assert!(!attacks(&mut game, id));
        }
        let (mut game, id) = arena(
            "#######\n###.###\n#.....#\n###.###\n#######",
            GridPos::new(2, 2),
            GridPos::new(3, 2),
        );
        game.map.set_protected(GridPos::new(4, 2), true).unwrap();
        game.resolve_ai_turn();
        assert_ne!(game.actors.get(id).unwrap().position(), GridPos::new(4, 2));
    }

    #[test]
    fn fauna_forager_respects_optical_concealment_even_inside_its_geometric_view() {
        let (mut game, id) = arena(
            "#######\n#.....#\n#.....#\n#.....#\n#######",
            GridPos::new(2, 1),
            GridPos::new(3, 2),
        );
        game.rules.stealth_rules = Some(crate::stealth::StealthRules::default());
        game.resolve_ai_turn();
        assert_eq!(game.actors.get(id).unwrap().ai_state(), AiState::Unaware);
        assert_eq!(game.actors.get(id).unwrap().position(), GridPos::new(3, 2));
        assert!(!attacks(&mut game, id));
        // Moving out of cover makes the same animal react, without changing its senses.
        game.actors
            .get_mut(game.player)
            .unwrap()
            .set_position(GridPos::new(2, 2));
        game.resolve_ai_turn();
        assert_eq!(game.actors.get(id).unwrap().ai_state(), AiState::Fleeing);
    }

    #[test]
    fn fauna_forager_does_not_treat_exhausted_search_budget_as_a_trap() {
        let (mut game, id) = arena(
            "#######\n#.....#\n#.....#\n#.....#\n#######",
            GridPos::new(2, 2),
            GridPos::new(3, 2),
        );
        let actor = game.actors.get(id).unwrap().clone();
        *game.actors.get_mut(id).unwrap() =
            actor.with_ai(AiProfile::new(AiBehavior::SkittishForager, 5, 0, 1, 0));
        game.resolve_ai_turn();
        assert_eq!(game.actors.get(id).unwrap().ai_state(), AiState::Fleeing);
        assert!(!attacks(&mut game, id));
    }
}
