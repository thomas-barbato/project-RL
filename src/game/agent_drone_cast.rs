//! Persistent agent invocations reuse ordinary drone actors and turn scheduling.
use super::*;
use crate::skills::agent;

pub(super) fn is_agent_drone(actor: &Actor) -> bool {
    actor.drone().is_some_and(|d| {
        matches!(
            d.profile().visual_profile().as_str(),
            "agent:d56" | "agent:d57" | "agent:d58" | "agent:d59" | "agent:d66"
        )
    })
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;
    use crate::content::ContentLoader;

    pub(crate) fn fixture() -> GameState {
        let content = ContentLoader::load(
            &[std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content")],
            &semver::Version::new(0, 1, 0),
        )
        .unwrap();
        let mut skills = crate::skills::SkillCatalog::default();
        skills
            .register_discipline(
                content
                    .skills()
                    .discipline(&"core:controle_drones".parse().unwrap())
                    .unwrap()
                    .clone(),
            )
            .unwrap();
        for (_, technique) in content
            .skills()
            .techniques()
            .filter(|(_, t)| t.discipline().as_str() == "core:controle_drones")
        {
            skills.register_technique(technique.clone()).unwrap();
        }
        let mut rules = GameRules {
            skills,
            items: content.items().clone(),
            weapons: content.weapons().clone(),
            statuses: content.statuses().clone(),
            player_weapon_slots: crate::game::WEAPON_HAND_SLOTS
                .map(|s| s.parse().unwrap())
                .into(),
            maintained_energy_reservations: true,
            player_companion_limit: Some(5),
            player_companion_behaviors: true,
            player_relation_targeting: true,
            ..Default::default()
        };
        let class = content
            .character_classes()
            .get(&"core:technomancien".parse().unwrap())
            .unwrap();
        class
            .apply_to_rules(&mut rules, class.recommended_attributes())
            .unwrap();
        rules.player_starting_weapons.clear();
        rules.player_starting_equipment.clear();
        let mut g=GameState::new_with_rules(Map::from_ascii(
            "############\n#..........#\n#..........#\n#..........#\n#..........#\n#..........#\n#..........#\n#..........#\n############"
        ).unwrap(),GridPos::new(4,4),13,rules).unwrap();
        g.player_skills.set_agent_skill_rank("D58", 1);
        g.player_energy = crate::resources::EnergyReserve::new(999, 999).unwrap();
        g.drain_events();
        g
    }
    fn cast(id: &str, at: GridPos) -> GameCommand {
        GameCommand::UseTechniqueAt {
            technique: agent::technique(id),
            target: at,
            weapon_slot: 0,
        }
    }
    #[test]
    fn drones_are_real_persistent_allies_and_act_once_per_turn_after_deployment() {
        for model in ["D58", "D59"] {
            let mut g = fixture();
            let at = GridPos::new(5, 4);
            let hostile = g
                .spawn_actor(
                    Actor::new(GridPos::new(7, 4), 500)
                        .unwrap()
                        .with_evasion_disabled()
                        .with_player_relation(crate::social::PlayerRelation::Hostile),
                )
                .unwrap();
            g.drain_events();
            let before = format!("{g:?}");
            g.player_weapon_technique_preview(&agent::technique(model), 0, at)
                .unwrap();
            assert_eq!(format!("{g:?}"), before);
            let energy = g.player_energy.available();
            assert_eq!(
                g.process_player_command(cast(model, at)),
                CommandOutcome::Applied
            );
            assert_eq!(g.turn, 1);
            assert_eq!(g.player_energy.available(), energy - 12);
            let drone = g.actors.entity_at(at).unwrap();
            assert!(g.is_player_controlled_companion(drone));
            assert_eq!(
                g.actors.get(drone).unwrap().drone().unwrap().lifecycle(),
                DroneLifecycle::Persistent
            );
            assert_eq!(g.actors.get(hostile).unwrap().integrity(), 500); // No birth attack.
            for _ in 0..3 {
                assert_eq!(
                    g.process_player_command(GameCommand::Wait),
                    CommandOutcome::Applied
                );
            }
            assert!(g.actors.get(hostile).unwrap().integrity() < 500);
            assert_eq!(
                g.actors
                    .get(drone)
                    .unwrap()
                    .drone()
                    .unwrap()
                    .energy()
                    .available(),
                30
            );
            g.drain_events();
            let world = crate::game::WorldState::single(g);
            let restored = crate::game::WorldState::from_recovery_snapshot_bytes(
                &world.recovery_snapshot_bytes().unwrap(),
                world.rules().clone(),
            )
            .unwrap();
            assert_eq!(restored.player_agent_drone_count(), 1);
            assert!(restored.actors().get(drone).unwrap().drone().is_some());
        }
    }
    #[test]
    fn army_capacity_counts_only_learned_branch_ranks_and_accepts_multiple_combat_models() {
        let mut g = fixture();
        assert_eq!(g.player_agent_drone_capacity(), 4);
        for at in [
            GridPos::new(5, 4),
            GridPos::new(4, 5),
            GridPos::new(3, 4),
            GridPos::new(4, 3),
        ] {
            assert_eq!(
                g.process_player_command(cast("D58", at)),
                CommandOutcome::Applied
            );
        }
        assert_eq!(g.player_agent_drone_count(), 4);
        let before = format!("{g:?}");
        assert!(matches!(
            g.process_player_command(cast("D59", GridPos::new(5, 5))),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(format!("{g:?}"), before);
        g.player_skills.set_agent_skill_rank("D58", 5);
        assert_eq!(g.player_agent_drone_capacity(), 5);
        // Free diagonal placement remains available even when four cardinals are occupied.
        assert_eq!(
            g.process_player_command(cast("D59", GridPos::new(5, 5))),
            CommandOutcome::Applied
        );
        assert_eq!(g.player_agent_drone_count(), 5);
    }
    #[test]
    fn invocation_refuses_bad_placement_unlearned_and_missing_energy_atomically() {
        let mut g = fixture();
        for at in [
            GridPos::new(4, 4),
            GridPos::new(0, 0),
            GridPos::new(8, 4),
            GridPos::new(-1, 0),
        ] {
            let before = format!("{g:?}");
            assert!(matches!(
                g.process_player_command(cast("D58", at)),
                CommandOutcome::Rejected(_)
            ));
            assert_eq!(format!("{g:?}"), before);
        }
        for rank in [0, 1] {
            g.player_skills.set_agent_skill_rank("D58", rank);
            if rank == 1 {
                let energy = g.player_energy.available();
                g.player_energy.spend(energy).unwrap();
            }
            let before = format!("{g:?}");
            assert!(matches!(
                g.process_player_command(cast("D58", GridPos::new(5, 4))),
                CommandOutcome::Rejected(_)
            ));
            assert_eq!(format!("{g:?}"), before);
        }
    }
    #[test]
    fn rank_increases_refresh_the_correct_drone_without_healing_or_refilling() {
        let mut g = fixture();
        g.process_player_command(cast("D58", GridPos::new(5, 4)));
        let drone = g.actors.entity_at(GridPos::new(5, 4)).unwrap();
        let before_max = g.actors.get(drone).unwrap().maximum_integrity();
        g.actors.get_mut(drone).unwrap().apply_damage(7);
        let current = g.actors.get(drone).unwrap().integrity();
        g.actors
            .get_mut(drone)
            .unwrap()
            .drone_mut()
            .unwrap()
            .spend_energy(5)
            .unwrap();
        g.player_skills.set_agent_skill_rank("D58", 10);
        g.refresh_agent_drone_statistics();
        let a = g.actors.get(drone).unwrap();
        assert!(a.maximum_integrity() > before_max);
        assert_eq!(a.integrity(), current);
        assert_eq!(a.drone().unwrap().energy().available(), 25);
    }
}

impl GameState {
    pub fn player_agent_drone_capacity(&self) -> u16 {
        let ranks: u16 = [
            "D56", "D57", "D58", "D59", "D66", "DP01", "DP02", "DR01", "O03",
        ]
        .into_iter()
        .map(|id| u16::from(self.player_skills.agent_skill_rank(id)))
        .sum();
        4 + ranks / 5
    }

    pub fn player_agent_drone_count(&self) -> usize {
        self.actors
            .iter()
            .filter(|(_, a)| {
                is_agent_drone(a) && a.drone().is_some_and(|d| d.controller() == self.player)
            })
            .count()
    }

    pub(super) fn agent_drone_skill_ready(
        &self,
        id: &str,
    ) -> Result<AgentSkillUseProfile, CommandRejection> {
        let start = self
            .rules
            .agent_class_start
            .as_ref()
            .ok_or(CommandRejection::AgentProgressionUnavailable)?;
        if self.player_agent_skill_rank(id) == 0 {
            return Err(CommandRejection::TechniqueNotLearned(agent::technique(id)));
        }
        if !matches!(id, "D56" | "D57" | "D58" | "D59" | "D66") {
            return Err(CommandRejection::TechniqueHasNoActiveAction(
                agent::technique(id),
            ));
        }
        if matches!(id, "D56" | "D57")
            && self.actors.iter().any(|(_, a)| {
                a.drone().is_some_and(|d| {
                    d.controller() == self.player
                        && d.profile().visual_profile().as_str()
                            == format!("agent:{}", id.to_ascii_lowercase())
                })
            })
        {
            return Err(TechniqueUseError::ManifestationLimitReached { maximum: 1 }.into());
        }
        if let Some(t) = self
            .actors
            .get(self.player)
            .and_then(Actor::recovery_remaining)
        {
            return Err(CommandRejection::OffensiveActionBlockedByRecovery {
                remaining_actions: t.get(),
            });
        }
        let active = self.player_agent_drone_count();
        if active >= usize::from(self.player_agent_drone_capacity()) {
            return Err(TechniqueUseError::ManifestationLimitReached {
                maximum: self.player_agent_drone_capacity().min(255) as u8,
            }
            .into());
        }
        let energy = (u32::from(
            agent::definition(id)
                .unwrap()
                .value("energy", self.player_agent_skill_rank(id)),
        ) * u32::from(start.human_skill_energy_percent))
        .div_ceil(100)
        .clamp(1, 65535) as u16;
        if self.player_energy.available() < energy {
            return Err(CommandRejection::InsufficientEnergy {
                required: energy,
                available: self.player_energy.available(),
            });
        }
        Ok(AgentSkillUseProfile { range: 1, energy })
    }

    pub(super) fn agent_drone_preview(
        &self,
        id: &str,
        target: GridPos,
    ) -> Result<AttackPreview, CommandRejection> {
        self.agent_drone_skill_ready(id)?;
        let origin = self
            .player_position()
            .ok_or(CommandRejection::AgentProgressionUnavailable)?;
        if !self.map.contains(target) {
            return Err(CommandRejection::AbilityTargetOutsideMap(target));
        }
        if grid_distance(origin, target) != 1 {
            return Err(CommandRejection::AbilityTargetOutOfRange(target));
        }
        if !self.player_visibility.is_visible(target) {
            return Err(CommandRejection::AbilityNoLineOfSight(target));
        }
        if !self.map.is_walkable(target) || self.actors.entity_at(target).is_some() {
            return Err(CommandRejection::TechniqueInvalidDroneRoutine);
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

    fn agent_combat_drone_statistics(
        &self,
        id: &str,
    ) -> (PrimaryAttributes, u16, Option<AttackProfile>, u16) {
        let level = self.player_progression.level().saturating_sub(1);
        let gunner = id == "D59";
        let major = 9u16.saturating_add(level.saturating_mul(3));
        let attrs = if id == "D56" {
            PrimaryAttributes::new(5, major, 7u16.saturating_add(level.saturating_mul(2)), 5, 5)
                .with_willpower(5)
        } else if id == "D57" {
            PrimaryAttributes::new(7u16.saturating_add(level.saturating_mul(2)), 5, major, 5, 5)
                .with_willpower(5)
        } else if id == "D66" {
            PrimaryAttributes::new(
                7u16.saturating_add(level),
                5u16.saturating_add(level),
                major,
                5,
                5,
            )
            .with_willpower(5)
        } else {
            PrimaryAttributes::new(
                if gunner { 5 } else { major },
                6u16.saturating_add(level),
                6u16.saturating_add(level),
                if gunner { major } else { 5 },
                5,
            )
            .with_willpower(5)
        };
        let rank = u16::from(self.player_agent_skill_rank(id).saturating_sub(1)) * 10;
        let owner = self.player_effective_primary_attributes().map_or(0, |a| {
            a.value(PrimaryAttribute::Processing).saturating_sub(5)
        });
        let start = self
            .rules
            .agent_class_start
            .as_ref()
            .expect("agent drone caster");
        let hp_base = 30u32
            + 4 * u32::from(attrs.value(PrimaryAttribute::Resilience).saturating_sub(5))
            + 2 * u32::from(level);
        let hp = (u64::from(hp_base)
            * (u64::from(start.drone_hp_percent) + u64::from(rank) + u64::from(owner))
            / 100)
            .clamp(1, 65535) as u16;
        let passive = self.player_agent_skill_rank("O03");
        let passive = if passive > 0 {
            agent::definition("O03")
                .unwrap()
                .value("drone_damage_bonus_percent", passive)
        } else {
            0
        };
        let damage_percent = start
            .drone_damage_percent
            .saturating_add(rank)
            .saturating_add(owner)
            .saturating_add(
                attrs
                    .value(if gunner {
                        PrimaryAttribute::Perception
                    } else {
                        PrimaryAttribute::Power
                    })
                    .saturating_sub(5),
            )
            .saturating_add(passive);
        // Accepted D58 contact attack. Rank/attribute bonuses use the shared additive pool.
        let attack = if gunner {
            AttackProfile::new(
                4,
                DistanceMetric::Chebyshev,
                true,
                crate::combat::DamageType::Kinetic,
                4,
                0,
            )
        } else {
            AttackProfile::melee(
                crate::combat::DamageType::Kinetic,
                if id == "D66" { 3 } else { 5 },
            )
        }
        .with_damage_output_percentage(damage_percent);
        let energy = 30u16.saturating_add(level.saturating_mul(2));
        (
            attrs,
            hp,
            (!matches!(id, "D56" | "D57")).then_some(attack),
            energy,
        )
    }

    pub(super) fn cast_agent_drone(
        &mut self,
        id: &str,
        target: GridPos,
    ) -> Result<(), CommandRejection> {
        self.agent_drone_preview(id, target)?;
        let cost = self.agent_drone_skill_ready(id)?.energy;
        let (attrs, hp, attack, energy) = self.agent_combat_drone_statistics(id);
        // Preserve current control-link hardware; no battery cost for walking.
        let Some(TechniqueAction::ManifestDrone {
            link_range,
            link_power,
            link_difficulty,
            link_attenuation_per_cell,
            link_wall_attenuation_multiplier,
            sensor_radius,
            ..
        }) = self
            .rules
            .skills
            .technique(&"core:drn_01".parse().unwrap())
            .and_then(|d| d.action())
        else {
            return Err(CommandRejection::TechniqueInvalidDroneRoutine);
        };
        let profile = DroneProfile::new(
            format!("agent:{}", id.to_ascii_lowercase())
                .parse()
                .unwrap(),
            link_range,
            link_power,
            link_difficulty,
            link_attenuation_per_cell,
            link_wall_attenuation_multiplier,
            if id == "D56" {
                agent::definition(id)
                    .unwrap()
                    .value("vision_radius_cells", self.player_agent_skill_rank(id))
            } else {
                sensor_radius
            },
            1,
            0,
            crate::drone::DroneCapabilities {
                manipulator_capacity_grams: (id == "D57")
                    .then_some(self.agent_mule_capacity_grams_for(attrs)),
                ..Default::default()
            },
        )
        .expect("existing drone control-link parameters");
        let mut actor = Actor::new(target, hp)
            .unwrap()
            .with_attacks(attack)
            .with_companion_origin(crate::companion::CompanionOrigin::Summoned);
        actor.delay_next_action_until(self.turn.saturating_add(1));
        let entity = self
            .spawn_player_drone(actor, profile, energy, energy)
            .map_err(|_| CommandRejection::TechniqueInvalidDroneRoutine)?;
        // Wide derived attributes are not restricted to the creation budget.
        self.actors
            .get_mut(entity)
            .unwrap()
            .set_primary_attributes(attrs);
        self.set_drone_order(
            entity,
            DroneOrder::Companion {
                controller: self.player,
                behavior: if matches!(id, "D56" | "D57") {
                    CompanionBehavior::Follow
                } else {
                    CompanionBehavior::Aggressive
                },
            },
        );
        self.player_energy
            .spend(cost)
            .expect("validated invocation cost");
        self.events.push(GameEvent::EnergySpent {
            entity: self.player,
            amount: cost,
            remaining: self.player_energy.available(),
        });
        self.events.push(GameEvent::TechniqueUsed {
            entity: self.player,
            technique: agent::technique(id),
            observed_on_turn: self.turn,
        });
        Ok(())
    }

    fn agent_mule_capacity_grams_for(&self, attrs: PrimaryAttributes) -> u32 {
        let kg = 40u32
            .saturating_add(2 * u32::from(attrs.value(PrimaryAttribute::Power).saturating_sub(5)))
            .saturating_add(8 * u32::from(self.player_agent_skill_rank("D57").saturating_sub(1)));
        kg.saturating_mul(1000)
    }

    pub fn agent_mule_capacity_grams(&self, drone: EntityId) -> Option<u32> {
        let actor = self.actors.get(drone)?;
        let state = actor.drone()?;
        (state.controller() == self.player
            && state.profile().visual_profile().as_str() == "agent:d57")
            .then(|| {
                self.agent_mule_capacity_grams_for(
                    actor
                        .primary_attributes()
                        .unwrap_or_else(|| PrimaryAttributes::new(5, 5, 5, 5, 5)),
                )
            })
    }

    /// Vision supplied by the one deployed, living Scout; actors own their positions.
    pub(super) fn agent_scout_vision_sources(&self) -> Vec<(GridPos, u16)> {
        self.actors
            .iter()
            .filter_map(|(_, a)| {
                let d = a.drone()?;
                (d.controller() == self.player
                    && d.profile().visual_profile().as_str() == "agent:d56")
                    .then_some((a.position(), d.profile().sensor_radius()))
            })
            .collect()
    }

    pub(crate) fn extend_agent_scout_visibility(&mut self) {
        for (origin, radius) in self.agent_scout_vision_sources() {
            let visible = compute_visible_tiles(
                &self.map,
                origin,
                FieldOfViewRules {
                    radius,
                    distance_metric: DistanceMetric::Euclidean,
                    block_closed_corners: true,
                },
            );
            self.player_visibility.merge_visible(visible);
        }
    }

    pub(super) fn refresh_agent_drone_statistics(&mut self) {
        if self.rules.agent_class_start.is_none() {
            return;
        }
        let drones: Vec<_> = self
            .actors
            .iter()
            .filter_map(|(id, a)| {
                let model = a
                    .drone()
                    .filter(|d| d.controller() == self.player)?
                    .profile()
                    .visual_profile()
                    .as_str();
                match model {
                    "agent:d56" => Some((id, "D56")),
                    "agent:d57" => Some((id, "D57")),
                    "agent:d58" => Some((id, "D58")),
                    "agent:d59" => Some((id, "D59")),
                    "agent:d66" => Some((id, "D66")),
                    _ => None,
                }
            })
            .collect();
        for (id, model) in drones {
            if self.player_agent_skill_rank(model) == 0 {
                continue;
            }
            let (attrs, hp, attack, energy) = self.agent_combat_drone_statistics(model);
            let vision = if model == "D56" {
                agent::definition(model)
                    .unwrap()
                    .value("vision_radius_cells", self.player_agent_skill_rank(model))
            } else {
                1
            };
            let cargo = self.agent_mule_capacity_grams_for(attrs);
            let actor = self.actors.get_mut(id).unwrap();
            let level_gain = actor.primary_attributes().is_some_and(|old| {
                attrs.value(PrimaryAttribute::Coordination)
                    > old.value(PrimaryAttribute::Coordination)
                    || attrs.value(PrimaryAttribute::Resilience)
                        > old.value(PrimaryAttribute::Resilience)
            });
            actor.set_primary_attributes(attrs);
            actor.resize_integrity_without_healing(hp);
            if level_gain {
                actor.restore_integrity(hp);
            }
            actor
                .drone_mut()
                .unwrap()
                .refresh_agent_energy_capacity(energy, level_gain);
            if model == "D56" {
                actor
                    .drone_mut()
                    .unwrap()
                    .profile_mut()
                    .set_agent_sensor_radius(vision);
            } else if model == "D57" {
                actor
                    .drone_mut()
                    .unwrap()
                    .profile_mut()
                    .set_agent_cargo_capacity(cargo);
            }
            actor.replace_attacks(attack);
        }
    }
}
