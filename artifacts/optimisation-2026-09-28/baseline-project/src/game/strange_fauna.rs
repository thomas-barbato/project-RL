//! Bounded local information sharing and a two-cell committed attack.
use super::*;
use crate::combat::AttackAreaCell;
use std::collections::{BTreeSet, VecDeque};

#[cfg(test)]
#[path = "strange_fauna_tests.rs"]
mod tests;

impl GameState {
    pub(in crate::game) fn relay_watcher_alerts(&mut self) {
        let Some(incident) = self.player_position() else {
            return;
        };
        if self.map.is_protected(incident) {
            return;
        }
        // Sources are captured before delivering anything. A report is never
        // mistaken for a fresh observation and cannot recursively flood a zone.
        let sources: Vec<_> = self
            .actors
            .iter()
            .filter_map(|(id, actor)| {
                (actor.ai().is_some_and(|ai| {
                    ai.behavior == AiBehavior::Watcher && ai.pursuit_lifecycle().is_some()
                }) && actor.ai_state() == AiState::Unaware
                    && actor.player_relation() == PlayerRelation::Hostile
                    && !actor.action_is_delayed(self.turn)
                    && actor.recovery_remaining().is_none()
                    && !self.map.is_protected(actor.position())
                    && actor
                        .ai_home()
                        .zip(actor.ai().and_then(|ai| ai.maximum_pursuit_distance()))
                        .is_none_or(|(home, limit)| grid_distance(home, incident) <= limit)
                    && self.actor_optically_detects_player(id))
                .then_some((id, actor.position(), actor.affiliation().cloned()))
            })
            .collect();
        for (source, at, affiliation) in sources {
            let mut visited = BTreeSet::from([source]);
            let mut pending = VecDeque::from([(at, 0_u8)]);
            let mut recipients = Vec::new();
            let mut links = Vec::new();
            while let Some((from, hop)) = pending.pop_front() {
                if hop >= 2 || recipients.len() >= 3 {
                    continue;
                }
                let mut neighbors: Vec<_> = self
                    .actors
                    .iter()
                    .filter_map(|(id, actor)| {
                        let ai = actor.ai()?;
                        let lifecycle = ai.pursuit_lifecycle()?;
                        (!visited.contains(&id)
                            && matches!(
                                ai.behavior,
                                AiBehavior::Watcher | AiBehavior::TelegraphedEcho
                            )
                            && actor.player_relation() == PlayerRelation::Hostile
                            && actor.affiliation() == affiliation.as_ref()
                            && actor.drone().is_none()
                            && actor.ai_state() == AiState::Unaware
                            && !self.map.is_protected(actor.position())
                            && grid_distance(from, actor.position()) <= 4
                            && has_line_of_sight(&self.map, from, actor.position(), true)
                            && actor
                                .ai_home()
                                .zip(ai.maximum_pursuit_distance())
                                .is_none_or(|(home, limit)| grid_distance(home, incident) <= limit))
                        .then_some((
                            grid_distance(from, actor.position()),
                            id,
                            actor.position(),
                            ai.behavior,
                            lifecycle.maximum_pursuit_turns().min(6),
                        ))
                    })
                    .collect();
                neighbors.sort_by_key(|(distance, id, ..)| (*distance, *id));
                for (_, id, position, behavior, turns) in neighbors {
                    if recipients.len() >= 3 {
                        break;
                    }
                    visited.insert(id);
                    self.actors
                        .get_mut(id)
                        .unwrap()
                        .set_ai_state(AiState::Responding {
                            remaining_turns: turns,
                            incident,
                        });
                    recipients.push(id);
                    links.push((from, position));
                    if behavior == AiBehavior::Watcher {
                        pending.push_back((position, hop + 1));
                    }
                }
            }
            if !recipients.is_empty() {
                self.events.push(GameEvent::FaunaAlertRelayed {
                    source,
                    at,
                    incident,
                    recipients,
                    links,
                });
            }
        }
    }

    pub(super) fn committed_footprint(
        &self,
        actor: &Actor,
        attack: AttackProfile,
        origin: GridPos,
        target_at: GridPos,
    ) -> Vec<AttackAreaCell> {
        if let Some(cells) = self.encounter_footprint(actor, origin, target_at) {
            return cells;
        }
        let mut cells = attack.affected_cells(&self.map, origin, target_at);
        if actor
            .ai()
            .is_some_and(|ai| ai.behavior == AiBehavior::TelegraphedEcho)
        {
            // A clockwise lateral echo, fixed by the announced origin/aim.
            // Its shape never consults the target's current position.
            let dx = target_at.x - origin.x;
            let dy = target_at.y - origin.y;
            let echo = if dx.abs() >= dy.abs() {
                GridPos::new(target_at.x, target_at.y + dx.signum())
            } else {
                GridPos::new(target_at.x - dy.signum(), target_at.y)
            };
            if echo != target_at
                && echo != origin
                && self.map.is_walkable(echo)
                && !self.map.is_protected(echo)
                && attack.is_in_range(origin, echo)
                && has_line_of_sight(&self.map, origin, echo, true)
            {
                cells.extend(attack.affected_cells(&self.map, origin, echo));
            }
        }
        cells
    }
}
