//! Accepted A02 / FE05 instantaneous areas, AOE-FALLOFF01 and SELF-DMG01.
use super::*;
use crate::skills::{agent, spatial::Area};

impl GameState {
    pub(super) fn agent_spell_area(&self, id: &str, rank: u8) -> Area {
        agent::definition(id)
            .and_then(|s| {
                s.centred_damage
                    .as_ref()
                    .map(|b| b.area(s.value(&b.extent_key, rank)))
            })
            .unwrap_or(Area::Point)
    }
    pub(super) fn cast_agent_area_spell(
        &mut self,
        id: &str,
        target: GridPos,
    ) -> Result<(), CommandRejection> {
        let preview = self.agent_spell_preview(id, target)?;
        let profile = self.agent_skill_ready(id)?;
        let area = self.agent_spell_area(id, self.player_agent_skill_rank(id));
        let radius = match area {
            Area::Square { radius } | Area::Disc { radius } => radius,
            _ => {
                return Err(CommandRejection::TechniqueHasNoActiveAction(
                    agent::technique(id),
                ));
            }
        };
        let target = preview.target();
        let burn = self.agent_spell_burn_profile(id, self.player_agent_skill_rank(id));
        // Recipients and power are captured once, before death hooks or level-up changes.
        // SELF-DMG01 protects the caster only. The A02 ally/drone rule is preserved.
        let recipients: Vec<_> = self
            .actors
            .iter()
            .filter_map(|(id, a)| {
                (id != self.player && preview.cells().iter().any(|c| c.position == a.position()))
                    .then_some((id, a.position()))
            })
            .collect();
        self.player_energy
            .spend(profile.energy)
            .expect("validated spell cost");
        self.events.push(GameEvent::TechniqueUsed {
            entity: self.player,
            technique: agent::technique(id),
            observed_on_turn: self.turn,
        });
        self.events.push(GameEvent::AgentAreaSpellCast {
            entity: self.player,
            skill: id.into(),
            origin: preview.origin(),
            target,
            radius,
        });
        for (victim, at) in recipients {
            if self.actors.get(victim).is_none() {
                continue;
            }
            let critical = self.rng.percentile() <= 5;
            self.events.push(GameEvent::AgentSpellCast {
                entity: self.player,
                victim,
                skill: id.into(),
                origin: target,
                target: at,
                critical,
            });
            let (numerator, denominator) = crate::skills::area_damage::retention(target, at, area)
                .expect("recipient in footprint");
            self.apply_damage_impact_to_scaled_spell(
                Some(self.player),
                victim,
                DamagePacket::new(
                    profile.base_damage,
                    if id == "FE05" {
                        crate::combat::DamageType::Thermal
                    } else {
                        crate::combat::DamageType::Corruption
                    },
                    0,
                ),
                u128::from(profile.multiplier_numerator)
                    * u128::from(numerator)
                    * if critical { 2 } else { 1 },
                u128::from(profile.multiplier_denominator) * u128::from(denominator),
            )
            .expect("living snapshotted recipient");
            if let Some((power, chance, duration)) = burn {
                if self.actors.get(victim).is_some() && u16::from(self.rng.percentile()) <= chance {
                    self.apply_agent_burn(victim, power, duration);
                }
            }
        }
        self.end_player_active_camouflage();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::agent_cast::tests::fixture;
    use super::*;
    use crate::combat::{DamageType, ResistanceProfile};
    fn arena() -> GameState {
        let mut g = fixture();
        g.player_skills.set_agent_skill_rank("A02", 1);
        g.player_energy = crate::resources::EnergyReserve::new(60000, 60000).unwrap();
        g.drain_events();
        g
    }
    fn cast(p: GridPos) -> GameCommand {
        GameCommand::UseTechniqueAt {
            technique: agent::technique("A02"),
            target: p,
            weapon_slot: 0,
        }
    }
    fn actor(g: &mut GameState, p: GridPos, relation: crate::social::PlayerRelation) -> EntityId {
        g.spawn_actor(
            Actor::new(p, 60000)
                .unwrap()
                .with_player_relation(relation)
                .with_resistances(ResistanceProfile::default().with(DamageType::Corruption, 10)),
        )
        .unwrap()
    }
    #[test]
    fn occult_blast_uses_square_falloff_one_final_rounding_and_keeps_caster_safe() {
        let mut g = arena();
        let c = GridPos::new(3, 3);
        let center = actor(&mut g, c, crate::social::PlayerRelation::Hostile);
        let edge = actor(
            &mut g,
            GridPos::new(4, 3),
            crate::social::PlayerRelation::Allied,
        );
        let outside = actor(
            &mut g,
            GridPos::new(5, 3),
            crate::social::PlayerRelation::Hostile,
        );
        let profile = DroneProfile::new(
            "agent:d58".parse().unwrap(),
            8,
            20,
            5,
            1,
            3,
            4,
            1,
            0,
            Default::default(),
        )
        .unwrap();
        let drone = DroneState::new(profile, g.player, 30, 30, GridPos::new(4, 4), g.turn)
            .unwrap()
            .with_lifecycle(DroneLifecycle::Persistent);
        let pet = g
            .spawn_actor(
                Actor::new(GridPos::new(4, 4), 60000)
                    .unwrap()
                    .with_drone(drone)
                    .with_player_relation(crate::social::PlayerRelation::Allied)
                    .with_resistances(
                        ResistanceProfile::default().with(DamageType::Corruption, 10),
                    ),
            )
            .unwrap();
        g.drain_events();
        let state = format!("{g:?}");
        let preview = g.agent_spell_preview("A02", c).unwrap();
        assert_eq!(preview.cells().len(), 9);
        assert_eq!(format!("{g:?}"), state);
        let hp = g.actors.get(g.player).unwrap().integrity();
        let energy = g.player_energy.available();
        let cost = g.agent_skill_ready("A02").unwrap().energy;
        assert_eq!(g.process_player_command(cast(c)), CommandOutcome::Applied);
        assert_eq!(g.actors.get(g.player).unwrap().integrity(), hp);
        assert_eq!(g.actors.get(outside).unwrap().integrity(), 60000);
        assert_eq!(g.player_energy.available(), energy - cost);
        assert_eq!(g.turn, 1);
        for (id, normal, critical) in [(center, 6, 12), (edge, 3, 6), (pet, 3, 6)] {
            let crit = g
                .events
                .iter()
                .find_map(|e| match e {
                    GameEvent::AgentSpellCast {
                        victim, critical, ..
                    } if *victim == id => Some(*critical),
                    _ => None,
                })
                .unwrap();
            assert_eq!(
                60000 - g.actors.get(id).unwrap().integrity(),
                if crit { critical } else { normal }
            );
        }
        assert_eq!(
            g.events
                .iter()
                .filter(|e| matches!(e, GameEvent::AgentAreaSpellCast { .. }))
                .count(),
            1
        );
    }
    #[test]
    fn occult_blast_can_target_empty_floor_and_rejects_invalid_aim_without_spending_anything() {
        let mut g = arena();
        let target = GridPos::new(5, 3);
        let before = format!("{g:?}");
        assert_eq!(
            g.agent_spell_preview("A02", target).unwrap().cells().len(),
            9
        );
        assert_eq!(format!("{g:?}"), before);
        for at in [GridPos::new(-1, 3), GridPos::new(9, 3), GridPos::new(2, 0)] {
            let before = format!("{g:?}");
            assert!(matches!(
                g.process_player_command(cast(at)),
                CommandOutcome::Rejected(_)
            ));
            assert_eq!(format!("{g:?}"), before);
        }
        g.map.set_protected(target, true).unwrap();
        let before = format!("{g:?}");
        assert_eq!(
            g.process_player_command(cast(target)),
            CommandOutcome::Rejected(CommandRejection::ProtectedZone)
        );
        assert_eq!(format!("{g:?}"), before);
        g.map.set_protected(target, false).unwrap();
        g.player_skills.set_agent_skill_rank("A02", 0);
        let before = format!("{g:?}");
        assert!(matches!(
            g.process_player_command(cast(target)),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(format!("{g:?}"), before);
        g.player_skills.set_agent_skill_rank("A02", 1);
        g.player_energy.spend(g.player_energy.available()).unwrap();
        let before = format!("{g:?}");
        assert!(matches!(
            g.process_player_command(cast(target)),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(format!("{g:?}"), before);
        g.player_energy.restore(60000);
        let rng = g.rng.clone();
        assert_eq!(
            g.process_player_command(cast(target)),
            CommandOutcome::Applied
        );
        assert_eq!(format!("{:?}", g.rng), format!("{rng:?}"));
        assert_eq!(g.turn, 1);
    }
    #[test]
    fn occult_blast_rank_growth_walls_protected_cells_and_hidden_actors_match_the_preview() {
        let mut g = arena();
        g.player_skills.set_agent_skill_rank("A02", 4);
        let c = GridPos::new(4, 3);
        assert_eq!(g.agent_skill_ready("A02").unwrap().range, 5);
        assert_eq!(g.agent_spell_preview("A02", c).unwrap().cells().len(), 25);
        g.map
            .set_terrain(GridPos::new(5, 3), crate::world::Terrain::Wall)
            .unwrap();
        g.refresh_player_visibility();
        let shadow = actor(
            &mut g,
            GridPos::new(6, 3),
            crate::social::PlayerRelation::Hostile,
        );
        g.map.set_protected(GridPos::new(4, 2), true).unwrap();
        let safe = actor(
            &mut g,
            GridPos::new(4, 2),
            crate::social::PlayerRelation::Allied,
        );
        let cells = g.agent_spell_preview("A02", c).unwrap();
        assert!(!cells.cells().iter().any(|cell| {
            [GridPos::new(5, 3), GridPos::new(6, 3), GridPos::new(4, 2)].contains(&cell.position)
        }));
        assert_eq!(g.process_player_command(cast(c)), CommandOutcome::Applied);
        for id in [shadow, safe] {
            assert_eq!(g.actors.get(id).unwrap().integrity(), 60000);
        }
        let mut hidden = arena();
        hidden.player_skills.set_agent_skill_rank("A02", 4);
        hidden.rules.player_field_of_view.radius = 3;
        hidden.refresh_player_visibility();
        let far = actor(
            &mut hidden,
            GridPos::new(6, 3),
            crate::social::PlayerRelation::Hostile,
        );
        assert!(!hidden.player_visibility.is_visible(GridPos::new(6, 3)));
        assert_eq!(
            hidden.process_player_command(cast(c)),
            CommandOutcome::Applied
        );
        assert_eq!(hidden.actors.get(far).unwrap().integrity(), 60000);
    }
    #[test]
    fn occult_blast_rolls_each_recipient_independently_repeats_next_turn_and_restores() {
        let mut g = arena();
        let c = GridPos::new(4, 3);
        let a = actor(&mut g, c, crate::social::PlayerRelation::Hostile);
        let b = actor(
            &mut g,
            GridPos::new(4, 4),
            crate::social::PlayerRelation::Hostile,
        );
        let mut different = false;
        for _ in 0..100 {
            g.drain_events();
            assert_eq!(g.process_player_command(cast(c)), CommandOutcome::Applied);
            let crit = |id| {
                g.events
                    .iter()
                    .find_map(|e| match e {
                        GameEvent::AgentSpellCast {
                            victim, critical, ..
                        } if *victim == id => Some(*critical),
                        _ => None,
                    })
                    .unwrap()
            };
            different |= crit(a) != crit(b);
        }
        assert!(different);
        assert_eq!(g.turn, 100);
        g.drain_events();
        let world = crate::game::WorldState::single(g);
        let mut restored = crate::game::WorldState::from_recovery_snapshot_bytes(
            &world.recovery_snapshot_bytes().unwrap(),
            world.rules().clone(),
        )
        .unwrap();
        assert_eq!(restored.player_agent_skill_rank("A02"), 1);
        assert_eq!(
            restored.process_player_command(cast(c)),
            CommandOutcome::Applied
        );
        assert_eq!(restored.turn(), 101);
    }
}
