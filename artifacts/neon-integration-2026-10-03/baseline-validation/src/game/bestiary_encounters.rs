//! Five bounded encounters. Commitments live in the simulation, never in VFX.
use super::*;
use crate::ai::EncounterState;
use crate::combat::AttackAreaCell;
use crate::world::find_path_with;

#[cfg(test)]
#[path = "bestiary_encounter_tests.rs"]
mod tests;

impl GameState {
    pub(in crate::game) fn actor_can_traverse(&self, entity: EntityId, at: GridPos) -> bool {
        self.map.is_walkable(at)
            || (self
                .actors
                .get(entity)
                .and_then(Actor::ai)
                .is_some_and(|ai| ai.behavior == AiBehavior::NestDiver)
                && self
                    .map
                    .tile(at)
                    .is_some_and(|tile| tile.terrain == Terrain::DeepWater))
    }

    pub(in crate::game) fn directional_guard_blocks(
        &self,
        target: EntityId,
        from: GridPos,
    ) -> bool {
        let Some(actor) = self.actors.get(target) else {
            return false;
        };
        if actor
            .ai()
            .is_none_or(|ai| ai.behavior != AiBehavior::DirectionalGuard)
        {
            return false;
        }
        let AiState::Encounter(EncounterState::Guard { dx, dy, .. }) = actor.ai_state() else {
            return false;
        };
        let x = from.x - actor.position().x;
        let y = from.y - actor.position().y;
        // A narrow front, not the entire forward half-plane. Sides remain open.
        let dot = x * dx + y * dy;
        dot > 0 && (x * dy - y * dx).abs() < dot
    }

    pub(in crate::game) fn stop_riveter_at(&mut self, at: GridPos) -> bool {
        let Some(id) = self.actors.entity_at(at) else {
            return false;
        };
        if self
            .actors
            .get(id)
            .and_then(Actor::ai)
            .is_none_or(|ai| ai.behavior != AiBehavior::Riveter)
        {
            return false;
        }
        self.actors
            .get_mut(id)
            .unwrap()
            .set_ai_state(AiState::Encounter(EncounterState::WorkStopped));
        self.events
            .push(GameEvent::RiveterStopped { entity: id, at });
        true
    }

    fn encounter_state(&mut self, id: EntityId, state: EncounterState) {
        if let Some(actor) = self.actors.get_mut(id) {
            actor.set_ai_state(AiState::Encounter(state));
        }
    }

    fn announce_encounter(&mut self, id: EntityId, target_at: GridPos) {
        let slot = self
            .actors
            .get(id)
            .and_then(Actor::ai)
            .unwrap()
            .preferred_attack_slot;
        if self
            .prepare_committed_attack(id, slot, target_at)
            .map_or(true, |prepared| {
                self.ensure_prepared_attack_usage(&prepared, 1).is_err()
            })
        {
            return;
        }
        let origin = self.actors.get(id).unwrap().position();
        self.actors
            .get_mut(id)
            .unwrap()
            .set_ai_state(AiState::Aiming { origin, target_at });
        self.events.push(GameEvent::AttackTelegraphed {
            attacker: id,
            origin,
            target_at,
        });
    }

    fn encounter_step(&mut self, id: EntityId, goal: GridPos) -> bool {
        let Some(actor) = self.actors.get(id) else {
            return false;
        };
        let from = actor.position();
        let Some(ai) = actor.ai() else {
            return false;
        };
        let path = find_path_with(
            &self.map,
            from,
            goal,
            ai.maximum_path_search,
            |_, at| self.actor_can_traverse(id, at),
            |at| {
                !self.map.is_protected(at)
                    && self.actors.entity_at(at).is_none()
                    && actor
                        .ai_home()
                        .zip(ai.maximum_pursuit_distance())
                        .is_none_or(|(home, limit)| grid_distance(home, at) <= limit)
            },
        );
        let Some(next) = path.and_then(|path| path.get(1).copied()) else {
            return false;
        };
        if self.actors.entity_at(next).is_some() || self.map.is_protected(next) {
            return false;
        }
        Direction::from_delta(next.x - from.x, next.y - from.y)
            .is_some_and(|direction| self.move_ai_entity(id, direction).is_ok())
    }

    pub(in crate::game) fn resolve_bestiary_encounter(
        &mut self,
        id: EntityId,
        action: AiAction,
        target: EntityId,
        recovering: bool,
    ) -> bool {
        let Some(actor) = self.actors.get(id).cloned() else {
            return false;
        };
        let Some(ai) = actor.ai() else {
            return false;
        };
        if !matches!(
            ai.behavior,
            AiBehavior::DirectionalGuard
                | AiBehavior::ExpandingRing
                | AiBehavior::AlternatingStalker
                | AiBehavior::NestDiver
                | AiBehavior::Riveter
        ) {
            return false;
        }
        if self.map.is_protected(actor.position()) {
            return true;
        }
        let seen = (target != id)
            .then(|| self.actors.get(target))
            .flatten()
            .filter(|a| {
                !self.map.is_protected(a.position())
                    && self.actor_perceives_position(id, a.position())
                    && (target != self.player || self.actor_optically_detects_player(id))
                    && actor
                        .ai_home()
                        .zip(ai.maximum_pursuit_distance())
                        .is_none_or(|(home, limit)| grid_distance(home, a.position()) <= limit)
            })
            .map(Actor::position);
        if ai.behavior == AiBehavior::DirectionalGuard {
            match actor.ai_state() {
                AiState::Encounter(EncounterState::Guard {
                    dx,
                    dy,
                    remaining_turns,
                }) if remaining_turns > 1 => self.encounter_state(
                    id,
                    EncounterState::Guard {
                        dx,
                        dy,
                        remaining_turns: remaining_turns - 1,
                    },
                ),
                _ => {
                    if let Some(at) = seen {
                        let x = at.x - actor.position().x;
                        let y = at.y - actor.position().y;
                        let (dx, dy) = if x.abs() >= y.abs() {
                            (x.signum(), 0)
                        } else {
                            (0, y.signum())
                        };
                        self.encounter_state(
                            id,
                            EncounterState::Guard {
                                dx,
                                dy,
                                remaining_turns: 3,
                            },
                        );
                    } else {
                        self.actors
                            .get_mut(id)
                            .unwrap()
                            .set_ai_state(AiState::Unaware);
                    }
                }
            }
            // The original pursuit action is still legal, but no remote target attack.
            if seen.is_none() {
                if let Some(home) = actor.ai_home() {
                    self.encounter_step(id, home);
                }
                return true;
            }
            return recovering;
        }
        if let AiState::Encounter(EncounterState::WorkStopped) = actor.ai_state() {
            return true;
        }
        if let AiState::Encounter(EncounterState::Withdrawal { remaining_turns }) = actor.ai_state()
        {
            if let Some(home) = actor.ai_home() {
                self.encounter_step(id, home);
                let at_home = self.actors.get(id).is_some_and(|a| {
                    a.position() == home
                        || (grid_distance(a.position(), home) <= 1
                            && self.actors.entity_at(home).is_some())
                });
                if remaining_turns <= 1 && at_home {
                    self.actors
                        .get_mut(id)
                        .unwrap()
                        .set_ai_state(AiState::Unaware);
                } else {
                    self.encounter_state(
                        id,
                        EncounterState::Withdrawal {
                            remaining_turns: remaining_turns.saturating_sub(1),
                        },
                    );
                }
            }
            return true;
        }
        if recovering {
            return true;
        }
        if let AiState::Encounter(EncounterState::Ring { origin, radius }) = actor.ai_state() {
            if actor.position() != origin {
                self.actors
                    .get_mut(id)
                    .unwrap()
                    .set_ai_state(AiState::Unaware);
                return true;
            }
            if let Ok(prepared) =
                self.prepare_committed_attack(id, ai.preferred_attack_slot, origin)
                && self.spend_prepared_attack_usage(&prepared, 1).is_ok()
            {
                let _ = self.resolve_prepared_attack(prepared, ActionOrigin::Normal);
            }
            if self.actors.get(id).is_none() {
                return true;
            }
            if radius < 3 {
                self.encounter_state(
                    id,
                    EncounterState::Ring {
                        origin,
                        radius: radius + 1,
                    },
                );
                self.events.push(GameEvent::AttackTelegraphed {
                    attacker: id,
                    origin,
                    target_at: origin,
                });
            } else {
                self.actors
                    .get_mut(id)
                    .unwrap()
                    .set_ai_state(AiState::Unaware);
                self.start_action_recovery(id, crate::time::TimeUnits::new(3).unwrap());
            }
            return true;
        }
        if let AiState::Aiming { origin, target_at } = actor.ai_state() {
            if origin == actor.position() {
                if ai.behavior == AiBehavior::NestDiver {
                    let prepared = self
                        .prepare_committed_attack(id, ai.preferred_attack_slot, target_at)
                        .ok();
                    // Fly to a neighboring landing cell along a real traversable route.
                    // No tracking and no wall-crossing, even if the victim moved.
                    for _ in 0..2 {
                        if self
                            .actors
                            .get(id)
                            .is_none_or(|a| grid_distance(a.position(), target_at) <= 1)
                        {
                            break;
                        }
                        if !self.encounter_step(id, target_at) {
                            break;
                        }
                    }
                    if self
                        .actors
                        .get(id)
                        .is_some_and(|a| grid_distance(a.position(), target_at) <= 1)
                    {
                        if let Some(prepared) = prepared
                            && self.spend_prepared_attack_usage(&prepared, 1).is_ok()
                        {
                            let recovery = prepared.attack.recovery_after_attack();
                            let _ = self.resolve_prepared_attack(prepared, ActionOrigin::Normal);
                            if let Some(recovery) = recovery {
                                self.start_action_recovery(id, recovery);
                            }
                        }
                    }
                } else {
                    let _ = self.perform_committed_attack(id, ai.preferred_attack_slot, target_at);
                }
            }
            if let Some(actor) = self.actors.get_mut(id) {
                actor.set_ai_state(AiState::Unaware);
            }
            if ai.behavior == AiBehavior::NestDiver {
                self.encounter_state(id, EncounterState::Withdrawal { remaining_turns: 2 });
            }
            return true;
        }
        if ai.behavior == AiBehavior::Riveter {
            // Fixed north/east/south/west axis chosen solely from the local worksite.
            // Damage is environmental: the machine never acquires or pursues a victim.
            for direction in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ] {
                let mut end = actor.position();
                for _ in 0..3 {
                    let next = end.step(direction);
                    if !self.map.is_walkable(next) || self.map.is_protected(next) {
                        break;
                    }
                    end = next;
                }
                if grid_distance(actor.position(), end) >= 2 {
                    self.announce_encounter(id, end);
                    break;
                }
            }
            return true;
        }
        let Some(at) = seen else {
            if let Some(home) = actor.ai_home() {
                self.encounter_step(id, home);
            }
            self.actors
                .get_mut(id)
                .unwrap()
                .set_ai_state(AiState::Unaware);
            return true;
        };
        match ai.behavior {
            AiBehavior::ExpandingRing if grid_distance(actor.position(), at) <= 3 => {
                let origin = actor.position();
                self.encounter_state(id, EncounterState::Ring { origin, radius: 2 });
                self.events.push(GameEvent::AttackTelegraphed {
                    attacker: id,
                    origin,
                    target_at: origin,
                });
            }
            AiBehavior::AlternatingStalker => {
                if let AiState::Encounter(EncounterState::Slow { remaining_turns }) =
                    actor.ai_state()
                {
                    if remaining_turns > 0 {
                        self.encounter_state(
                            id,
                            EncounterState::Slow {
                                remaining_turns: remaining_turns - 1,
                            },
                        );
                    } else if grid_distance(actor.position(), at) <= 2 {
                        self.announce_encounter(id, at);
                    } else {
                        self.encounter_step(id, at);
                        self.encounter_state(id, EncounterState::Slow { remaining_turns: 1 });
                    }
                } else {
                    self.encounter_state(id, EncounterState::Slow { remaining_turns: 1 });
                }
            }
            AiBehavior::NestDiver if grid_distance(actor.position(), at) <= 3 => {
                self.announce_encounter(id, at)
            }
            _ => {
                let _ = action;
                self.encounter_step(id, at);
            }
        }
        true
    }

    pub(super) fn encounter_footprint(
        &self,
        actor: &Actor,
        origin: GridPos,
        target: GridPos,
    ) -> Option<Vec<AttackAreaCell>> {
        let behavior = actor.ai()?.behavior;
        let mut cells = Vec::new();
        match behavior {
            AiBehavior::ExpandingRing => {
                let AiState::Encounter(EncounterState::Ring { radius, .. }) = actor.ai_state()
                else {
                    return Some(cells);
                };
                let radius = radius.clamp(2, 3);
                for y in -i32::from(radius)..=i32::from(radius) {
                    for x in -i32::from(radius)..=i32::from(radius) {
                        let distance4 = 4 * (x * x + y * y);
                        let inner = 2 * i32::from(radius) - 1;
                        let outer = 2 * i32::from(radius) + 1;
                        if distance4 > inner * inner && distance4 <= outer * outer {
                            cells.push(AttackAreaCell {
                                position: GridPos::new(origin.x + x, origin.y + y),
                                step: radius,
                            });
                        }
                    }
                }
            }
            AiBehavior::Riveter => {
                let dx = (target.x - origin.x).signum();
                let dy = (target.y - origin.y).signum();
                for step in 1..=grid_distance(origin, target).min(3) {
                    cells.push(AttackAreaCell {
                        position: GridPos::new(
                            origin.x + dx * i32::from(step),
                            origin.y + dy * i32::from(step),
                        ),
                        step,
                    });
                }
            }
            _ => return None,
        }
        cells.retain(|cell| {
            self.map.is_walkable(cell.position)
                && !self.map.is_protected(cell.position)
                && has_line_of_sight(&self.map, origin, cell.position, true)
        });
        Some(cells)
    }
}
