//! Reviewed stealth mode and non-blocking holographic diversion.
use super::*;
use crate::skills::agent;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AgentStealthRuntime {
    pub hologram: Option<AgentHologram>,
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> GameState {
        let mut game = super::super::agent_cast::tests::fixture();
        game.player_skills.set_agent_skill_rank("SF00", 10);
        game.player_skills.set_agent_skill_rank("ES03", 1);
        game
    }

    #[test]
    fn stealth_has_no_stationary_upkeep_and_ranked_direct_bonus() {
        let mut game = fixture();
        let at = game.player_position().unwrap();
        let before = game.player_energy.available();
        game.cast_agent_stealth("SF00", at).unwrap();
        assert!(game.agent_stealth_active());
        assert_eq!(game.agent_stealth_damage_bonus(), 50);
        assert_eq!(game.player_energy.available(), before);
        assert_eq!(game.agent_stealth_step_energy(), 11);
        for (rank, cost) in [(1, 10), (5, 6), (9, 2), (10, 1), (15, 1)] {
            game.player_skills.set_agent_skill_rank("SF01", rank);
            assert_eq!(game.agent_stealth_step_energy(), cost);
        }
        game.cast_agent_stealth("SF00", at).unwrap();
        assert!(!game.agent_stealth_active());
        assert_eq!(game.agent_stealth_damage_bonus(), 0);
    }

    #[test]
    fn stealth_step_cannot_spend_unavailable_energy_and_detection_has_one_cell_floor() {
        let mut game = fixture();
        game.cast_agent_stealth("SF00", game.player_position().unwrap())
            .unwrap();
        assert_eq!(game.agent_stealth_detection_radius(16), 4);
        game.player_skills.set_agent_skill_rank("SF02", 10);
        assert_eq!(game.agent_stealth_detection_radius(16), 1);
        game.player_energy
            .spend(game.player_energy.available() - 1)
            .unwrap();
        game.agent_stealth_successful_step(game.player);
        assert!(!game.agent_stealth_active());
        assert_eq!(game.player_energy.available(), 1);
    }

    #[test]
    fn hologram_refuses_occupied_cell_atomically_then_replaces_one_double() {
        let mut game = fixture();
        let origin = game.player_position().unwrap();
        let before = format!("{game:?}");
        assert!(game.cast_agent_stealth("ES03", origin).is_err());
        assert_eq!(format!("{game:?}"), before);
        let first = GridPos::new(origin.x + 1, origin.y);
        let second = GridPos::new(origin.x, origin.y + 1);
        game.cast_agent_stealth("ES03", first).unwrap();
        let original = game
            .agent_runtime()
            .weapon
            .stealth
            .hologram
            .as_ref()
            .unwrap()
            .entity;
        game.cast_agent_stealth("ES03", second).unwrap();
        assert!(game.actors.get(original).is_none());
        let current = game
            .agent_runtime()
            .weapon
            .stealth
            .hologram
            .as_ref()
            .unwrap()
            .entity;
        assert_ne!(original, current);
        assert_eq!(game.actors.get(current).unwrap().integrity(), 1);
        assert!(game.agent_is_hologram(current));
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AgentHologram {
    pub entity: EntityId,
    pub expires: u64,
    pub overridden: BTreeSet<EntityId>,
    pub diverted: BTreeSet<EntityId>,
}

impl GameState {
    pub(super) fn agent_stealth_profile(&self, id: &str, rank: u8) -> Option<AgentSpellProfile> {
        if rank == 0 || !matches!(id, "SF00" | "ES03") {
            return None;
        }
        let definition = agent::definition(id)?;
        let start = self.rules.agent_class_start.as_ref()?;
        Some(AgentSpellProfile {
            range: if id == "SF00" {
                0
            } else {
                definition.value("range_cells", rank)
            },
            energy: if id == "SF00" {
                0
            } else {
                (u32::from(definition.value("energy", rank))
                    * u32::from(start.human_skill_energy_percent))
                .div_ceil(100)
                .clamp(1, 65535) as u16
            },
            cooldown: if id == "SF00" {
                0
            } else {
                definition.value("cooldown_turns", rank)
            },
            base_damage: 0,
            multiplier_numerator: 1,
            multiplier_denominator: 1,
        })
    }

    pub fn agent_stealth_active(&self) -> bool {
        self.player_active_camouflage
            .as_ref()
            .is_some_and(|c| c.technique == agent::technique("SF00"))
    }

    pub fn agent_stealth_step_energy(&self) -> u16 {
        let rank = self.player_agent_skill_rank("SF01");
        if rank == 0 {
            11
        } else {
            agent::definition("SF01")
                .unwrap()
                .value("energy_per_successful_stealth_step", rank)
        }
    }

    pub fn agent_stealth_damage_bonus(&self) -> u16 {
        if self.agent_stealth_active() {
            u16::from(self.player_agent_skill_rank("SF00")) * 5
        } else {
            0
        }
    }

    pub(super) fn agent_stealth_preview(
        &self,
        id: &str,
        target: GridPos,
    ) -> Result<AttackPreview, CommandRejection> {
        let profile = self.agent_skill_ready(id)?;
        let origin = self
            .player_position()
            .ok_or(CommandRejection::MissingPlayer)?;
        if !self.map.contains(target) {
            return Err(CommandRejection::AbilityTargetOutsideMap(target));
        }
        if id == "SF00" {
            if target != origin {
                return Err(CommandRejection::AbilityTargetOutOfRange(target));
            }
        } else {
            if !crate::skills::spatial::in_cast_range(origin, target, profile.range) {
                return Err(CommandRejection::AbilityTargetOutOfRange(target));
            }
            if !self.player_visibility.is_visible(target)
                || !has_line_of_sight(&self.map, origin, target, true)
            {
                return Err(CommandRejection::AbilityNoLineOfSight(target));
            }
            if self.map.is_protected(origin) || self.map.is_protected(target) {
                return Err(CommandRejection::ProtectedZone);
            }
            if !self.map.is_walkable(target) || self.actors.entity_at(target).is_some() {
                return Err(CommandRejection::AbilityTargetBlocked(target));
            }
        }
        Ok(AttackPreview::new(
            origin,
            target,
            vec![AttackAreaCell {
                position: target,
                step: 0,
            }],
        ))
    }

    pub(super) fn cast_agent_stealth(
        &mut self,
        id: &str,
        target: GridPos,
    ) -> Result<(), CommandRejection> {
        self.agent_stealth_preview(id, target)?;
        let profile = self.agent_skill_ready(id)?;
        if id == "SF00" {
            if self.agent_stealth_active() {
                self.end_player_active_camouflage();
            } else {
                self.player_active_camouflage = Some(ActiveCamouflage {
                    technique: agent::technique("SF00"),
                    channel: SignatureChannel::Optical,
                    optical_difficulty_bonus: 0,
                    remaining_phases: u16::MAX,
                    upkeep_energy: 0,
                    heat_per_phase: 0,
                });
                self.events.push(GameEvent::ActiveCamouflageChanged {
                    entity: self.player,
                    technique: agent::technique(id),
                    channel: SignatureChannel::Optical,
                    active: true,
                });
            }
        } else {
            let actor = Actor::new(target, 1)
                .unwrap()
                .with_evasion_disabled()
                .with_player_relation(crate::social::PlayerRelation::Allied)
                .with_tags(["agent:hologram".parse().unwrap()]);
            // A successful admission must not destroy the previous double if a
            // new entity cannot be allocated.
            let entity = self
                .spawn_actor(actor)
                .map_err(|_| CommandRejection::AbilityTargetBlocked(target))?;
            if let Some(previous) = self.agent_runtime_mut().weapon.stealth.hologram.take() {
                self.actors.remove(previous.entity);
            }
            let rank = self.player_agent_skill_rank(id);
            self.agent_runtime_mut().weapon.stealth.hologram = Some(AgentHologram {
                entity,
                expires: self.turn
                    + u64::from(agent::definition(id).unwrap().value("duration_turns", rank)),
                overridden: BTreeSet::new(),
                diverted: BTreeSet::new(),
            });
            self.player_energy
                .spend(profile.energy)
                .expect("preflight hologram energy");
            self.events.push(GameEvent::EnergySpent {
                entity: self.player,
                amount: profile.energy,
                remaining: self.player_energy.available(),
            });
        }
        self.events.push(GameEvent::TechniqueUsed {
            entity: self.player,
            technique: agent::technique(id),
            observed_on_turn: self.turn,
        });
        self.events.push(GameEvent::AgentWeaponSkillUsed {
            entity: self.player,
            skill: id.into(),
            origin: self.player_position().unwrap(),
            target,
        });
        Ok(())
    }

    /// Apply only after path/occupancy/interception admission succeeded.
    pub(super) fn agent_stealth_successful_step(&mut self, entity: EntityId) {
        if entity != self.player || !self.agent_stealth_active() {
            return;
        }
        let amount = self.agent_stealth_step_energy();
        if self.player_energy.available() < amount {
            self.end_player_active_camouflage();
            return;
        }
        self.player_energy
            .spend(amount)
            .expect("affordable stealth step");
        self.events.push(GameEvent::EnergySpent {
            entity: self.player,
            amount,
            remaining: self.player_energy.available(),
        });
    }

    pub(super) fn agent_stealth_step_noise(&self, amount: u16) -> u16 {
        if self.agent_stealth_active() {
            amount / 2
        } else {
            amount
        }
    }

    pub fn agent_stealth_detection_radius(&self, ordinary: u16) -> u16 {
        if !self.agent_stealth_active() || ordinary == 0 {
            return ordinary;
        }
        let rank = self.player_agent_skill_rank("SF02");
        ((f64::from(ordinary) / 4.0) * (4_f64 / 5_f64).powi(i32::from(rank)))
            .floor()
            .max(1.0) as u16
    }

    pub fn agent_is_hologram(&self, entity: EntityId) -> bool {
        self.agent_runtime()
            .weapon
            .stealth
            .hologram
            .as_ref()
            .is_some_and(|h| h.entity == entity)
    }

    pub(super) fn agent_hologram_target(&mut self, observer: EntityId) -> Option<EntityId> {
        let hologram = self.agent_runtime().weapon.stealth.hologram.as_ref()?;
        if hologram.expires <= self.turn || hologram.overridden.contains(&observer) {
            return None;
        }
        let entity = hologram.entity;
        let already_diverted = hologram.diverted.contains(&observer);
        let source = self.actors.get(observer)?;
        if source.player_relation() != crate::social::PlayerRelation::Hostile
            || source.tags().iter().any(|t| t.as_str() == "core:boss")
        {
            return None;
        }
        let elite = source.tags().iter().any(|t| t.as_str() == "core:elite");
        let target = self.actors.get(entity)?;
        let radius = source.ai()?.perception_radius;
        let seen = crate::world::is_tile_visible(
            &self.map,
            source.position(),
            target.position(),
            FieldOfViewRules {
                radius,
                distance_metric: DistanceMetric::Euclidean,
                block_closed_corners: true,
            },
        );
        if !seen {
            return None;
        }
        if !already_diverted {
            let succeeds = !elite || self.rng.percentile() <= 75;
            let hologram = self
                .agent_runtime_mut()
                .weapon
                .stealth
                .hologram
                .as_mut()
                .unwrap();
            if succeeds {
                hologram.diverted.insert(observer);
            } else {
                hologram.overridden.insert(observer);
                return None;
            }
        }
        Some(entity)
    }

    pub(super) fn agent_hologram_reveal_attacker(
        &mut self,
        source: EntityId,
        target: Option<EntityId>,
    ) {
        if source != self.player {
            return;
        }
        if let (Some(target), Some(hologram)) = (
            target,
            &mut self.agent_runtime_mut().weapon.stealth.hologram,
        ) {
            hologram.overridden.insert(target);
        }
    }

    pub(super) fn advance_agent_hologram(&mut self) {
        let gone = self
            .agent_runtime()
            .weapon
            .stealth
            .hologram
            .as_ref()
            .is_some_and(|h| h.expires <= self.turn + 1 || self.actors.get(h.entity).is_none());
        if gone && let Some(hologram) = self.agent_runtime_mut().weapon.stealth.hologram.take() {
            self.actors.remove(hologram.entity);
        }
    }
}
