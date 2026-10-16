//! Accepted D66-TAUNT01-NUM01 / MULTI01. No new persistent fields.
use super::*;

fn provoked() -> StatusId {
    "agent:provoked".parse().unwrap()
}
fn taunt_clock() -> TechniqueId {
    "agent:guardian_taunt".parse().unwrap()
}

impl GameState {
    pub(super) fn agent_taunt_source(&self, target: EntityId) -> Option<EntityId> {
        let source = self.actors.get(target)?.status(&provoked())?.source?;
        self.actors
            .get(source)?
            .drone()
            .filter(|d| {
                d.controller() == self.player
                    && d.profile().visual_profile().as_str() == "agent:d66"
            })
            .map(|_| source)
    }
    pub(super) fn clear_invalid_agent_taunts(&mut self) {
        let invalid: Vec<_> = self
            .actors
            .iter()
            .filter_map(|(id, a)| {
                (a.status(&provoked()).is_some() && self.agent_taunt_source(id).is_none())
                    .then_some(id)
            })
            .collect();
        for target in invalid {
            self.actors
                .get_mut(target)
                .unwrap()
                .remove_status(&provoked());
            self.events.push(GameEvent::StatusRemoved {
                target,
                status: provoked(),
                reason: super::super::StatusRemovalReason::Consumed,
            });
        }
    }
    pub(super) fn resolve_agent_guardian_taunts(&mut self) -> BTreeSet<EntityId> {
        let guardians: Vec<_> = self
            .actors
            .iter()
            .filter_map(|(id, a)| {
                let d = a.drone()?;
                if d.controller() != self.player
                    || d.profile().visual_profile().as_str() != "agent:d66"
                    || a.action_is_delayed(self.turn)
                    || a.recovery_remaining().is_some()
                    || a.technique_cooldown_remaining(&taunt_clock()).is_some()
                    || !matches!(
                        d.order(),
                        DroneOrder::Companion {
                            behavior: CompanionBehavior::Aggressive | CompanionBehavior::Defensive,
                            ..
                        }
                    )
                {
                    return None;
                }
                if self.rules.player_drone_link_awareness
                    && !self.player_position().is_some_and(|p| {
                        d.profile().link_reaches(
                            &self.map,
                            p,
                            a.position(),
                            self.electronic_jamming_penalty_at(
                                crate::electronic_warfare::ElectronicChannel::ControlLink,
                                a.position(),
                            ),
                        )
                    })
                {
                    return None;
                }
                Some((id, a.position()))
            })
            .collect();
        let assignments: Vec<_> = self
            .actors
            .iter()
            .filter_map(|(target, a)| {
                if target == self.player
                    || a.player_relation() != crate::social::PlayerRelation::Hostile
                    || a.status(&provoked()).is_some()
                    || self.map.is_protected(a.position())
                    || !self.player_visibility.is_visible(a.position())
                {
                    return None;
                }
                let source = guardians
                    .iter()
                    .filter(|(_, p)| {
                        grid_distance(*p, a.position()) <= 2
                            && has_line_of_sight(&self.map, *p, a.position(), true)
                    })
                    .min_by_key(|(id, p)| (grid_distance(*p, a.position()), id.get()))?
                    .0;
                Some((target, source))
            })
            .collect();
        let mut casting = BTreeSet::new();
        for (target, source) in assignments {
            self.apply_status_to(
                Some(source),
                target,
                &ApplyStatusEffect::new(provoked(), 1).unwrap(),
            )
            .expect("agent status registered in run rules");
            casting.insert(source);
        }
        for &entity in &casting {
            let a = self.actors.get_mut(entity).unwrap();
            let _ = a.begin_normal_action();
            a.start_technique_cooldown(
                taunt_clock(),
                self.turn,
                // The casting phase arms the clock. Three blocked phases allow
                // the next provocation on phase N+4, not N+5.
                crate::time::TimeUnits::new(3).unwrap(),
            );
            self.events.push(GameEvent::AgentGuardianTaunted {
                entity,
                origin: a.position(),
            });
        }
        casting
    }
}

#[cfg(test)]
mod tests {
    use super::super::agent_drone_cast::tests::fixture;
    use super::*;
    use crate::ai::{AiBehavior, AiProfile};
    fn hostile(g: &mut GameState, at: GridPos) -> EntityId {
        g.spawn_actor(
            Actor::new(at, 500)
                .unwrap()
                .with_ai(AiProfile::new(AiBehavior::Hunter, 8, 0, 256, 0))
                .with_attack(AttackProfile::melee(crate::combat::DamageType::Kinetic, 1))
                .with_evasion_disabled()
                .with_player_relation(crate::social::PlayerRelation::Hostile),
        )
        .unwrap()
    }
    fn guardian(g: &mut GameState, at: GridPos) -> EntityId {
        g.player_skills.set_agent_skill_rank("D66", 1);
        g.cast_agent_drone("D66", at).unwrap();
        let id = g.actors.entity_at(at).unwrap();
        g.actors.get_mut(id).unwrap().delay_next_action_until(0);
        id
    }
    #[test]
    fn guardian_taunt_redirects_two_hostile_actions_and_replaces_its_attack() {
        let mut g = fixture();
        g.player_skills.set_agent_skill_rank("D66", 1);
        let enemy = hostile(&mut g, GridPos::new(5, 3));
        assert_eq!(
            g.process_player_command(GameCommand::UseTechniqueAt {
                technique: crate::skills::agent::technique("D66"),
                target: GridPos::new(5, 4),
                weapon_slot: 0
            }),
            CommandOutcome::Applied
        );
        let drone = g.actors.entity_at(GridPos::new(5, 4)).unwrap();
        assert!(g.actors.get(enemy).unwrap().status(&provoked()).is_none());
        let player_hp = g.actors.get(g.player).unwrap().integrity();
        g.drain_events();
        for remaining in [Some(1), None] {
            assert_eq!(
                g.process_player_command(GameCommand::Wait),
                CommandOutcome::Applied
            );
            let events = g.drain_events();
            assert!(events.iter().any(|e|matches!(e,GameEvent::AttackPerformed {attacker,target:Some(t),..} if *attacker==enemy && *t==drone)));
            assert_eq!(g.actors.get(g.player).unwrap().integrity(), player_hp);
            assert_eq!(
                g.actors
                    .get(enemy)
                    .unwrap()
                    .status(&provoked())
                    .and_then(|s| s.remaining_turns),
                remaining
            );
            if remaining.is_some() {
                assert!(!events.iter().any(
                    |e| matches!(e,GameEvent::AttackPerformed {attacker,..} if *attacker==drone)
                ));
            }
        }
        // Provocations are four phases apart, no refresh from repeated player actions.
        for _ in 0..2 {
            g.process_player_command(GameCommand::Wait);
            assert!(
                !g.drain_events()
                    .iter()
                    .any(|e| matches!(e, GameEvent::AgentGuardianTaunted { .. }))
            );
        }
        g.process_player_command(GameCommand::Wait);
        assert!(
            g.drain_events()
                .iter()
                .any(|e| matches!(e,GameEvent::AgentGuardianTaunted {entity,..} if *entity==drone))
        );
    }
    #[test]
    fn simultaneous_guardians_choose_the_nearest_source_once_and_survive_save_reload() {
        let mut g = fixture();
        let right = guardian(&mut g, GridPos::new(5, 4));
        let left = guardian(&mut g, GridPos::new(3, 4));
        let a = hostile(&mut g, GridPos::new(6, 3));
        let b = hostile(&mut g, GridPos::new(2, 3));
        let tie = hostile(&mut g, GridPos::new(4, 3));
        assert_eq!(g.resolve_agent_guardian_taunts().len(), 2);
        assert_eq!(g.agent_taunt_source(a), Some(right));
        assert_eq!(g.agent_taunt_source(b), Some(left));
        assert_eq!(g.agent_taunt_source(tie), Some(right));
        g.elapse_status_durations();
        assert!(g.resolve_agent_guardian_taunts().is_empty());
        assert_eq!(
            g.actors
                .get(tie)
                .unwrap()
                .status(&provoked())
                .unwrap()
                .remaining_turns,
            Some(1)
        );
        g.drain_events();
        let world = crate::game::WorldState::single(g);
        let mut restored = crate::game::WorldState::from_recovery_snapshot_bytes(
            &world.recovery_snapshot_bytes().unwrap(),
            world.rules().clone(),
        )
        .unwrap();
        assert_eq!(restored.agent_taunt_source(a), Some(right));
        assert!(
            restored
                .actors
                .get(right)
                .unwrap()
                .technique_cooldown_remaining(&taunt_clock())
                .is_some()
        );
        restored.actors.remove(right);
        restored.clear_invalid_agent_taunts();
        assert_eq!(restored.agent_taunt_source(a), None);
        assert_eq!(restored.agent_taunt_source(tie), None);
        assert_eq!(restored.agent_taunt_source(b), Some(left));
        assert!(restored.resolve_agent_guardian_taunts().is_empty());
    }
    #[test]
    fn taunt_respects_walls_range_allies_and_does_not_refresh_other_guardians() {
        let mut g = fixture();
        let drone = guardian(&mut g, GridPos::new(5, 4));
        g.map
            .set_terrain(GridPos::new(6, 4), crate::world::Terrain::Wall)
            .unwrap();
        let blocked = hostile(&mut g, GridPos::new(7, 4));
        let distant = hostile(&mut g, GridPos::new(8, 5));
        let ally = g
            .spawn_actor(
                Actor::new(GridPos::new(5, 3), 40)
                    .unwrap()
                    .with_player_relation(crate::social::PlayerRelation::Allied),
            )
            .unwrap();
        assert!(g.resolve_agent_guardian_taunts().is_empty());
        for id in [blocked, distant, ally, g.player] {
            assert_eq!(g.agent_taunt_source(id), None);
        }
        let near = hostile(&mut g, GridPos::new(5, 2));
        assert_eq!(g.resolve_agent_guardian_taunts(), BTreeSet::from([drone]));
        let second = guardian(&mut g, GridPos::new(4, 3));
        assert!(g.resolve_agent_guardian_taunts().is_empty());
        assert!(
            g.actors
                .get(second)
                .unwrap()
                .technique_cooldown_remaining(&taunt_clock())
                .is_none()
        );
        assert_eq!(g.agent_taunt_source(near), Some(drone));
    }
}
