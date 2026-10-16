//! Accepted drone repair, position exchange, self-destruction and passive upkeep.
use super::*;
use crate::skills::agent;

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct AgentDroneRuntime {
    pub quiet_turns: u16,
    pub last_attack_turn: Option<u64>,
    pub last_damage_turn: Option<u64>,
    pub last_maintenance_turn: Option<u64>,
    /// Hundredths of one HP; whole repair alone is sent to the actor.
    pub repair_remainders: BTreeMap<EntityId, u16>,
}

impl GameState {
    pub(super) fn agent_drone_utility_ready(
        &self,
        id: &str,
    ) -> Result<AgentSkillUseProfile, CommandRejection> {
        let start = self
            .rules
            .agent_class_start
            .as_ref()
            .ok_or(CommandRejection::AgentProgressionUnavailable)?;
        let rank = self.player_agent_skill_rank(id);
        if rank == 0 {
            return Err(CommandRejection::TechniqueNotLearned(agent::technique(id)));
        }
        if !matches!(id, "DR01" | "HY04" | "HY07") {
            return Err(CommandRejection::TechniqueHasNoActiveAction(
                agent::technique(id),
            ));
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
        let skill = agent::definition(id).unwrap();
        let range = skill.value(
            match id {
                "HY04" => "maximum_drone_distance_cells",
                "HY07" => "command_range_cells",
                _ => "range_cells",
            },
            rank,
        );
        let energy = (u32::from(skill.value("energy", rank))
            * u32::from(start.human_skill_energy_percent))
        .div_ceil(100)
        .clamp(1, 65535) as u16;
        if self.player_energy.available() < energy {
            return Err(CommandRejection::InsufficientEnergy {
                required: energy,
                available: self.player_energy.available(),
            });
        }
        Ok(AgentSkillUseProfile { range, energy })
    }

    fn agent_drone_utility_target(
        &self,
        id: &str,
        at: GridPos,
    ) -> Result<EntityId, CommandRejection> {
        let profile = self.agent_drone_utility_ready(id)?;
        let origin = self
            .player_position()
            .ok_or(CommandRejection::AgentProgressionUnavailable)?;
        if !self.map.contains(at) {
            return Err(CommandRejection::AbilityTargetOutsideMap(at));
        }
        if !crate::skills::spatial::in_cast_range(origin, at, profile.range) {
            return Err(CommandRejection::AbilityTargetOutOfRange(at));
        }
        if !self.player_visibility.is_visible(at) || !has_line_of_sight(&self.map, origin, at, true)
        {
            return Err(CommandRejection::AbilityNoLineOfSight(at));
        }
        if self.map.is_protected(origin) || self.map.is_protected(at) {
            return Err(CommandRejection::ProtectedZone);
        }
        if id == "HY04" && self.agent_magic_movement_blocked(self.player) {
            return Err(CommandRejection::AbilityTargetBlocked(at));
        }
        let entity = self
            .actors
            .entity_at(at)
            .ok_or(CommandRejection::TechniqueInvalidDroneRoutine)?;
        let actor = self.actors.get(entity).unwrap();
        let controlled = actor.drone().is_some_and(|d| d.controller() == self.player);
        let eligible = if id == "DR01" {
            (controlled || actor.player_relation() == crate::social::PlayerRelation::Allied)
                && (actor.drone().is_some() || actor.electronic_system().is_some())
                && actor.integrity() < actor.maximum_integrity()
                && !self.agent_healing_blocked(entity)
        } else if id == "HY07" {
            controlled
                && actor.drone().is_some_and(|d| {
                    matches!(
                        d.profile().visual_profile().as_str(),
                        "agent:d58" | "agent:d59" | "agent:d66"
                    )
                })
        } else {
            controlled && self.map.is_walkable(at) && self.map.is_walkable(origin)
        };
        if entity == self.player || !eligible {
            return Err(CommandRejection::TechniqueInvalidDroneRoutine);
        }
        Ok(entity)
    }

    pub(super) fn agent_drone_utility_footprint(
        &self,
        id: &str,
        at: GridPos,
    ) -> Result<AttackPreview, CommandRejection> {
        self.agent_drone_utility_target(id, at)?;
        let origin = self.player_position().unwrap();
        let cells = if id == "HY07" {
            let area = self.agent_spell_area(id, self.player_agent_skill_rank(id));
            crate::skills::spatial::affected_cells(&self.map, at, area, (1, 0))
                .into_iter()
                .filter(|p| self.player_visibility.is_visible(*p) && !self.map.is_protected(*p))
                .map(|position| AttackAreaCell {
                    position,
                    step: grid_distance(at, position),
                })
                .collect()
        } else {
            vec![AttackAreaCell {
                position: at,
                step: 0,
            }]
        };
        Ok(AttackPreview::new(origin, at, cells))
    }

    /// The same pre-defense amount is used by the repair card and real healing.
    pub fn agent_drone_repair_amount(&self, rank: u8) -> u16 {
        let processing = self
            .player_effective_primary_attributes()
            .map_or(5, |a| a.value(PrimaryAttribute::Processing));
        // DR01's accepted repair coefficient is +2% per Intelligence above five.
        (u64::from(
            agent::definition("DR01")
                .unwrap()
                .value("raw_robot_heal_HP", rank),
        ) * (100 + 2 * u64::from(processing.saturating_sub(5)))
            / 100)
            .min(65535) as u16
    }

    pub fn agent_drone_self_destruct_damage(&self, rank: u8) -> u16 {
        let base = agent::definition("HY07")
            .unwrap()
            .value("raw_fire_explosion_damage", rank);
        let intelligence = self
            .player_effective_primary_attributes()
            .map_or(5, |a| a.value(PrimaryAttribute::Processing));
        (u128::from(base)
            * (10 + u128::from(self.player_progression.level().saturating_sub(1)))
            * (100 + u128::from(intelligence.saturating_sub(5)))
            / 1000)
            .min(65535) as u16
    }

    pub(super) fn cast_agent_drone_utility(
        &mut self,
        id: &str,
        at: GridPos,
    ) -> Result<(), CommandRejection> {
        let preview = self.agent_drone_utility_footprint(id, at)?;
        let target = self.agent_drone_utility_target(id, at)?;
        let energy = self.agent_drone_utility_ready(id)?.energy;
        let rank = self.player_agent_skill_rank(id);
        self.player_energy
            .spend(energy)
            .expect("preflighted drone utility cost");
        self.events.push(GameEvent::EnergySpent {
            entity: self.player,
            amount: energy,
            remaining: self.player_energy.available(),
        });
        self.events.push(GameEvent::TechniqueUsed {
            entity: self.player,
            technique: agent::technique(id),
            observed_on_turn: self.turn,
        });
        match id {
            "DR01" => {
                let heal = self.agent_drone_repair_amount(rank);
                let amount = self.actors.get_mut(target).unwrap().restore_integrity(heal);
                if amount > 0 {
                    self.events.push(GameEvent::IntegrityRestored {
                        entity: target,
                        amount,
                    });
                    self.events.push(GameEvent::AllyHealed {
                        medic: self.player,
                        target,
                        amount,
                    });
                    self.events.push(GameEvent::AgentSpellCast {
                        entity: self.player,
                        victim: target,
                        skill: "DR01".into(),
                        origin: preview.origin(),
                        target: at,
                        critical: false,
                    });
                }
            }
            "HY04" => {
                let origin = preview.origin();
                self.actors.move_to(self.player, at).expect("living caster");
                self.actors
                    .move_to(target, origin)
                    .expect("living controlled drone");
                self.record_agent_movement(self.player, origin, at);
                self.record_agent_movement(target, at, origin);
                self.refresh_player_visibility();
            }
            "HY07" => self.resolve_agent_drone_self_destruct(target, preview, rank),
            _ => unreachable!(),
        }
        Ok(())
    }

    fn resolve_agent_drone_self_destruct(
        &mut self,
        drone: EntityId,
        preview: AttackPreview,
        rank: u8,
    ) {
        let skill = agent::definition("HY07").unwrap();
        let at = preview.target();
        let area = self.agent_spell_area("HY07", rank);
        let targets: Vec<_> = self
            .actors
            .iter()
            .filter_map(|(id, a)| {
                (id != drone
                    && id != self.player
                    && a.player_relation() == crate::social::PlayerRelation::Hostile
                    && !self.is_player_controlled_companion(id)
                    && preview.cells().iter().any(|c| c.position == a.position()))
                .then_some((id, a.position()))
            })
            .collect();
        let base = skill.value("raw_fire_explosion_damage", rank);
        let processing = self
            .player_effective_primary_attributes()
            .map_or(5, |a| a.value(PrimaryAttribute::Processing));
        let level = u128::from(self.player_progression.level().saturating_sub(1)) + 10;
        let multiplier = 100 + u128::from(processing.saturating_sub(5));
        // Sacrifice is unconditional once the legal command is confirmed; armor cannot save the drone.
        let dead = self.actors.remove(drone).expect("preflighted sacrifice");
        self.drop_agent_drone_cargo(drone, &dead);
        let state = dead.drone().unwrap();
        let bandwidth = state
            .profile()
            .bandwidth_required()
            .saturating_add(state.order_bandwidth());
        self.release_player_bandwidth(bandwidth);
        self.events
            .push(GameEvent::EntityDied { entity: drone, at });
        self.events.push(GameEvent::DroneControlReleased {
            entity: drone,
            controller: self.player,
            bandwidth_released: bandwidth,
        });
        self.electronic_warfare.remove_beacon(drone);
        self.clear_invalid_agent_taunts();
        self.events.push(GameEvent::AgentAreaSpellCast {
            entity: self.player,
            skill: "HY07".into(),
            origin: preview.origin(),
            target: at,
            radius: skill.value("radius_cells", rank),
        });
        self.note_agent_drone_attack(self.player);
        for (victim, position) in targets {
            if self.actors.get(victim).is_none() {
                continue;
            }
            let (n, d) = crate::skills::area_damage::retention(at, position, area)
                .expect("recipient in preview");
            let critical = self.rng.percentile() <= 5;
            self.events.push(GameEvent::AgentSpellCast {
                entity: self.player,
                victim,
                skill: "HY07".into(),
                origin: at,
                target: position,
                critical,
            });
            let _ = self.apply_damage_impact_to_scaled_spell(
                Some(self.player),
                victim,
                DamagePacket::new(base, crate::combat::DamageType::Thermal, 0),
                level * multiplier * u128::from(n) * if critical { 2 } else { 1 },
                1000 * u128::from(d),
            );
        }
        self.end_player_active_camouflage();
    }

    pub(super) fn drop_agent_drone_cargo(&mut self, _entity: EntityId, actor: &Actor) {
        let Some(cargo) = actor.drone().and_then(DroneState::cargo) else {
            return;
        };
        let ground_item = self
            .ground_items
            .spawn_cargo_remains(actor.position(), cargo)
            .expect("cargo already has a nonempty valid stack");
        self.events.push(GameEvent::GroundItemSpawned {
            ground_item,
            definition: cargo.item().clone(),
            quantity: cargo.quantity(),
            at: actor.position(),
        });
    }

    pub(super) fn note_agent_drone_attack(&mut self, attacker: EntityId) {
        if self.rules.agent_class_start.is_none() {
            return;
        }
        if attacker == self.player
            || self
                .actors
                .get(attacker)
                .is_some_and(|a| a.drone().is_some_and(|d| d.controller() == self.player))
        {
            let turn = self.turn;
            let runtime = &mut self.agent_runtime_mut().drones;
            runtime.last_attack_turn = Some(turn);
            runtime.quiet_turns = 0;
        }
    }

    pub(super) fn note_agent_drone_damage(&mut self, target: EntityId) {
        if self.rules.agent_class_start.is_none() {
            return;
        }
        if target == self.player
            || self
                .actors
                .get(target)
                .is_some_and(|a| a.drone().is_some_and(|d| d.controller() == self.player))
        {
            let turn = self.turn;
            let runtime = &mut self.agent_runtime_mut().drones;
            runtime.last_damage_turn = Some(turn);
            runtime.quiet_turns = 0;
        }
    }

    /// DP02 is applied to resolved damage after ordinary armor and resistance.
    pub(super) fn agent_drone_incoming_damage_percent(&self, target: EntityId) -> u16 {
        let rank = self.player_agent_skill_rank("DP02");
        if rank == 0
            || !self
                .actors
                .get(target)
                .is_some_and(|a| a.drone().is_some_and(|d| d.controller() == self.player))
        {
            return 100;
        }
        100u16.saturating_sub(
            agent::definition("DP02")
                .unwrap()
                .value("incoming_damage_reduction_percent", rank),
        )
    }

    pub(super) fn tick_agent_drone_maintenance(&mut self) {
        if self.rules.agent_class_start.is_none() {
            return;
        }
        let turn = self.turn;
        if self.agent_runtime().drones.last_maintenance_turn == Some(turn) {
            return;
        }
        let rank = self.player_agent_skill_rank("DP01");
        let interrupted = rank == 0
            || self.agent_runtime().drones.last_attack_turn == Some(turn)
            || self.agent_runtime().drones.last_damage_turn == Some(turn)
            || self.actors.iter().any(|(id, a)| {
                id != self.player
                    && a.player_relation() == crate::social::PlayerRelation::Hostile
                    && !self.is_player_controlled_companion(id)
                    && self.player_visibility.is_visible(a.position())
            });
        let runtime = &mut self.agent_runtime_mut().drones;
        runtime.last_maintenance_turn = Some(turn);
        if interrupted {
            runtime.quiet_turns = 0;
            return;
        }
        runtime.quiet_turns = runtime.quiet_turns.saturating_add(1).min(4);
        if runtime.quiet_turns < 4 {
            return;
        }
        let percent = agent::definition("DP01")
            .unwrap()
            .value("repair_percent_of_drone_maximum_HP", rank);
        let drones: Vec<_> = self
            .actors
            .iter()
            .filter_map(|(id, a)| {
                a.drone()
                    .filter(|d| d.controller() == self.player)
                    .map(|_| (id, a.integrity(), a.maximum_integrity()))
            })
            .collect();
        let live: BTreeSet<_> = drones.iter().map(|(id, _, _)| *id).collect();
        self.agent_runtime_mut()
            .drones
            .repair_remainders
            .retain(|id, _| live.contains(id));
        for (entity, current, maximum) in drones {
            if self.agent_healing_blocked(entity) {
                continue;
            }
            if current == maximum {
                self.agent_runtime_mut()
                    .drones
                    .repair_remainders
                    .remove(&entity);
                continue;
            }
            let remainder = self
                .agent_runtime()
                .drones
                .repair_remainders
                .get(&entity)
                .copied()
                .unwrap_or(0);
            let units = u32::from(maximum) * u32::from(percent) + u32::from(remainder);
            let amount = self
                .actors
                .get_mut(entity)
                .unwrap()
                .restore_integrity((units / 100).min(65535) as u16);
            let remaining = if self.actors.get(entity).unwrap().integrity() == maximum {
                0
            } else {
                (units % 100) as u16
            };
            self.agent_runtime_mut()
                .drones
                .repair_remainders
                .insert(entity, remaining);
            if amount > 0 {
                self.events
                    .push(GameEvent::IntegrityRestored { entity, amount });
                if let Some(at) = self.actors.get(entity).map(Actor::position) {
                    self.events.push(GameEvent::AgentSpellCast {
                        entity: self.player,
                        victim: entity,
                        skill: "DP01".into(),
                        origin: at,
                        target: at,
                        critical: false,
                    });
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::agent_drone_cast::tests::fixture;
    use super::*;

    fn game() -> GameState {
        let mut g = fixture();
        for id in ["D56", "D57", "D58", "D59", "D66", "DR01", "HY04", "HY07"] {
            g.player_skills.set_agent_skill_rank(id, 1);
        }
        g
    }
    fn summon(g: &mut GameState, id: &str, at: GridPos) -> EntityId {
        g.cast_agent_drone(id, at).unwrap();
        g.actors.entity_at(at).unwrap()
    }
    #[test]
    fn support_drones_are_unique_real_actors_without_an_attack_and_keep_their_hardware() {
        let mut g = game();
        let scout = summon(&mut g, "D56", GridPos::new(5, 4));
        let mule = summon(&mut g, "D57", GridPos::new(4, 5));
        assert!(g.actors.get(scout).unwrap().attacks().is_empty());
        assert!(g.actors.get(mule).unwrap().attacks().is_empty());
        assert_eq!(
            g.agent_scout_vision_sources(),
            vec![(GridPos::new(5, 4), 4)]
        );
        assert_eq!(g.agent_mule_capacity_grams(mule), Some(44000));
        assert_eq!(
            g.actors
                .get(mule)
                .unwrap()
                .drone()
                .unwrap()
                .profile()
                .capabilities()
                .manipulator_capacity_grams,
            Some(44000)
        );
        for id in ["D56", "D57"] {
            let before = format!("{g:?}");
            assert!(g.cast_agent_drone(id, GridPos::new(3, 4)).is_err());
            assert_eq!(format!("{g:?}"), before);
        }
        let hp = g.actors.get(mule).unwrap().maximum_integrity();
        g.actors.get_mut(mule).unwrap().apply_damage(10);
        g.player_skills.set_agent_skill_rank("D57", 10);
        g.player_skills.set_agent_skill_rank("D56", 10);
        g.refresh_agent_drone_statistics();
        assert_eq!(g.agent_mule_capacity_grams(mule), Some(116000));
        assert_eq!(
            g.agent_scout_vision_sources(),
            vec![(GridPos::new(5, 4), 7)]
        );
        assert_eq!(g.actors.get(mule).unwrap().integrity(), hp - 10);
        assert!(g.actors.get(mule).unwrap().attacks().is_empty());
    }
    #[test]
    fn targeted_repair_caps_at_missing_hp_and_refuses_full_hp_organic_or_hidden_targets_atomically()
    {
        let mut g = game();
        let at = GridPos::new(5, 4);
        let drone = summon(&mut g, "D58", at);
        let maximum = g.actors.get(drone).unwrap().maximum_integrity();
        g.actors.get_mut(drone).unwrap().apply_damage(3);
        let energy = g.player_energy.available();
        let cost = g.agent_drone_utility_ready("DR01").unwrap().energy;
        let before = format!("{g:?}");
        g.agent_drone_utility_footprint("DR01", at).unwrap();
        assert_eq!(format!("{g:?}"), before);
        g.cast_agent_drone_utility("DR01", at).unwrap();
        assert_eq!(g.actors.get(drone).unwrap().integrity(), maximum);
        assert_eq!(g.player_energy.available(), energy - cost);
        assert!(g.events.iter().any(
            |e| matches!(e, GameEvent::AllyHealed { target, amount: 3, .. } if *target == drone)
        ));
        let organic = GridPos::new(3, 4);
        g.spawn_actor(
            Actor::new(organic, 20)
                .unwrap()
                .with_player_relation(crate::social::PlayerRelation::Allied),
        )
        .unwrap();
        for bad in [at, organic, GridPos::new(2, 2), GridPos::new(-1, 4)] {
            let before = format!("{g:?}");
            assert!(g.cast_agent_drone_utility("DR01", bad).is_err());
            assert_eq!(format!("{g:?}"), before);
        }
    }
    #[test]
    fn position_exchange_swaps_real_cells_without_restoring_drone_resources() {
        let mut g = game();
        let at = GridPos::new(5, 4);
        let origin = g.player_position().unwrap();
        let drone = summon(&mut g, "D57", at);
        g.actors.get_mut(drone).unwrap().apply_damage(4);
        g.actors
            .get_mut(drone)
            .unwrap()
            .drone_mut()
            .unwrap()
            .spend_energy(5)
            .unwrap();
        let hp = g.actors.get(drone).unwrap().integrity();
        let battery = g
            .actors
            .get(drone)
            .unwrap()
            .drone()
            .unwrap()
            .energy()
            .available();
        g.cast_agent_drone_utility("HY04", at).unwrap();
        assert_eq!(g.player_position(), Some(at));
        assert_eq!(g.actors.get(drone).unwrap().position(), origin);
        assert_eq!(g.actors.get(drone).unwrap().integrity(), hp);
        assert_eq!(
            g.actors
                .get(drone)
                .unwrap()
                .drone()
                .unwrap()
                .energy()
                .available(),
            battery
        );
    }
    #[test]
    fn self_destruct_sacrifices_exactly_one_combat_drone_and_spares_allies() {
        let mut g = game();
        let at = GridPos::new(5, 4);
        let drone = summon(&mut g, "D58", at);
        let other = summon(&mut g, "D59", GridPos::new(4, 5));
        let hostile = g
            .spawn_actor(
                Actor::new(GridPos::new(6, 4), 1000)
                    .unwrap()
                    .with_player_relation(crate::social::PlayerRelation::Hostile),
            )
            .unwrap();
        let ally = g
            .spawn_actor(
                Actor::new(GridPos::new(5, 5), 1000)
                    .unwrap()
                    .with_player_relation(crate::social::PlayerRelation::Allied),
            )
            .unwrap();
        let player_hp = g.actors.get(g.player).unwrap().integrity();
        let other_hp = g.actors.get(other).unwrap().integrity();
        g.cast_agent_drone_utility("HY07", at).unwrap();
        assert!(g.actors.get(drone).is_none());
        assert_eq!(g.player_agent_drone_count(), 1);
        assert_eq!(g.actors.get(g.player).unwrap().integrity(), player_hp);
        assert_eq!(g.actors.get(other).unwrap().integrity(), other_hp);
        assert_eq!(g.actors.get(ally).unwrap().integrity(), 1000);
        assert!(g.actors.get(hostile).unwrap().integrity() < 1000);
        assert!(
            g.actors
                .get(hostile)
                .unwrap()
                .status(&"core:burning".parse().unwrap())
                .is_none()
        );
    }
    #[test]
    fn self_destruct_refuses_scout_mule_and_invalid_targets_without_spending_or_rng_changes() {
        let mut g = game();
        summon(&mut g, "D56", GridPos::new(5, 4));
        summon(&mut g, "D57", GridPos::new(4, 5));
        for at in [GridPos::new(5, 4), GridPos::new(4, 5), GridPos::new(3, 4)] {
            let before = format!("{g:?}");
            assert!(g.cast_agent_drone_utility("HY07", at).is_err());
            assert_eq!(format!("{g:?}"), before);
        }
    }
    #[test]
    fn maintenance_waits_three_complete_quiet_turns_and_interruption_restarts_the_wait() {
        let mut g = game();
        let drone = summon(&mut g, "D58", GridPos::new(5, 4));
        g.player_skills.set_agent_skill_rank("DP01", 1);
        g.actors.get_mut(drone).unwrap().apply_damage(20);
        let hp = g.actors.get(drone).unwrap().integrity();
        for turn in 1..=3 {
            g.turn = turn;
            g.tick_agent_drone_maintenance();
            assert_eq!(g.actors.get(drone).unwrap().integrity(), hp);
        }
        g.turn = 4;
        g.tick_agent_drone_maintenance();
        g.turn = 5;
        g.tick_agent_drone_maintenance();
        assert!(g.actors.get(drone).unwrap().integrity() > hp);
        let healed = g.actors.get(drone).unwrap().integrity();
        g.tick_agent_drone_maintenance(); // No extra tick when no reference turn elapsed.
        assert_eq!(g.actors.get(drone).unwrap().integrity(), healed);
        g.turn = 6;
        g.note_agent_drone_attack(g.player);
        g.tick_agent_drone_maintenance();
        for turn in 7..=9 {
            g.turn = turn;
            g.tick_agent_drone_maintenance();
            assert_eq!(g.actors.get(drone).unwrap().integrity(), healed);
        }
        assert_eq!(g.agent_runtime().drones.quiet_turns, 3);
    }
    #[test]
    fn armoring_and_algorithms_are_continuous_passives_not_map_animations() {
        let mut g = game();
        let drone = summon(&mut g, "D58", GridPos::new(5, 4));
        // The Technomancer fixture starts with O03 already learned. Exercise
        // an actual transition from no algorithms to its first rank.
        g.player_skills.set_agent_skill_rank("O03", 0);
        g.refresh_agent_drone_statistics();
        let damage = g
            .actors
            .get(drone)
            .unwrap()
            .attack(0)
            .unwrap()
            .damage_output_percentage();
        assert_eq!(g.agent_drone_incoming_damage_percent(drone), 100);
        g.player_skills.set_agent_skill_rank("DP02", 10);
        assert_eq!(g.agent_drone_incoming_damage_percent(drone), 72);
        assert_eq!(g.agent_drone_incoming_damage_percent(g.player), 100);
        g.player_skills.set_agent_skill_rank("O03", 1);
        g.refresh_agent_drone_statistics();
        assert_eq!(g.actors.get(drone).unwrap().attacks().len(), 1);
        assert!(
            g.actors
                .get(drone)
                .unwrap()
                .attack(0)
                .unwrap()
                .damage_output_percentage()
                > damage
        );
    }
    #[test]
    fn scout_sight_uses_native_walls_and_is_removed_when_the_actor_is_gone() {
        let mut g = game();
        let scout = summon(&mut g, "D56", GridPos::new(5, 4));
        g.rules.player_field_of_view.radius = 1;
        let origin = g.player_position().unwrap();
        g.player_visibility
            .recompute(&g.map, origin, g.rules.player_field_of_view);
        let distant = GridPos::new(8, 4);
        assert!(!g.player_visibility.is_visible(distant));
        g.extend_agent_scout_visibility();
        assert!(g.player_visibility.is_visible(distant));
        g.map
            .set_terrain(GridPos::new(6, 4), Terrain::Wall)
            .unwrap();
        g.player_visibility
            .recompute(&g.map, origin, g.rules.player_field_of_view);
        g.extend_agent_scout_visibility();
        assert!(!g.player_visibility.is_visible(distant));
        g.actors.remove(scout);
        g.player_visibility
            .recompute(&g.map, origin, g.rules.player_field_of_view);
        g.extend_agent_scout_visibility();
        assert!(!g.player_visibility.is_visible(distant));
        assert!(g.player_visibility.is_explored(distant));
    }
    #[test]
    fn losing_an_equipment_granted_invocation_removes_only_its_drone_and_drops_cargo_once() {
        let mut g = game();
        g.player_skills.set_agent_skill_rank("D57", 0);
        let slot: EquipmentSlotId = "core:body_armor".parse().unwrap();
        g.rules.player_armor_slots.push(slot.clone());
        let mut instances = Vec::new();
        for model in ["test:mule_source", "test:neutral_armor"] {
            let model: ItemId = model.parse().unwrap();
            g.rules
                .items
                .register(
                    crate::item::ItemDefinition::new(
                        model.clone(),
                        "name".into(),
                        "description".into(),
                        1,
                        crate::item::ItemKind::Armor,
                        Some(crate::item::EquipmentProfile::new(slot.clone(), 1).unwrap()),
                        vec![],
                    )
                    .unwrap(),
                )
                .unwrap();
            instances.push(g.player_inventory.add(model, 1, 1).unwrap()[0]);
        }
        g.rules.equipment_agent_skills.insert(
            "test:mule_source".parse().unwrap(),
            [("D57".into(), 2)].into(),
        );
        assert_eq!(
            g.process_player_command(GameCommand::EquipItem {
                slot: slot.clone(),
                item: instances[0],
            }),
            CommandOutcome::Applied
        );
        assert_eq!(g.player_agent_skill_rank("D57"), 2);
        assert_eq!(g.player_skills.agent_skill_rank("D57"), 0);
        assert_eq!(
            g.process_player_command(GameCommand::UseTechniqueAt {
                technique: agent::technique("D57"),
                target: GridPos::new(5, 4),
                weapon_slot: 0,
            }),
            CommandOutcome::Applied
        );
        let mule = g.actors.entity_at(GridPos::new(5, 4)).unwrap();
        let other = summon(&mut g, "D58", GridPos::new(4, 5));
        let at = g.actors.get(mule).unwrap().position();
        let cargo_item: ItemId = "core:repair_patch".parse().unwrap();
        let cargo_owner: crate::social::SocialGroupId = "test:owner".parse().unwrap();
        g.actors
            .get_mut(mule)
            .unwrap()
            .drone_mut()
            .unwrap()
            .load_cargo(DroneCargo::new(
                cargo_item.clone(),
                3,
                Some(cargo_owner.clone()),
                3000,
            ))
            .unwrap();
        let xp = g.player_progression.experience();
        g.drain_events();
        assert_eq!(
            g.process_player_command(GameCommand::EquipItem {
                slot,
                item: instances[1],
            }),
            CommandOutcome::Applied
        );
        assert_eq!(g.player_agent_skill_rank("D57"), 0);
        assert!(g.actors.get(mule).is_none());
        assert!(g.actors.get(other).is_some());
        assert!(g.player_inventory.get(instances[0]).is_some());
        let dropped: Vec<_> = g
            .ground_items
            .iter()
            .filter(|(_, stack)| stack.position() == at && stack.item() == &cargo_item)
            .collect();
        assert_eq!(dropped.len(), 1);
        assert_eq!(dropped[0].1.quantity(), 3);
        assert_eq!(dropped[0].1.owner(), Some(&cargo_owner));
        assert_eq!(g.events.iter().filter(|event| matches!(event,
            GameEvent::GroundItemSpawned {definition,quantity:3,..} if definition == &cargo_item
        )).count(), 1);
        assert!(!g.events.iter().any(|event| matches!(
            event,
            GameEvent::ExperienceAwarded { .. } | GameEvent::EntityDefeatedByPlayer { .. }
        )));
        assert_eq!(g.player_progression.experience(), xp);
        // Rechecking an already-lost source must neither respawn nor duplicate its cargo.
        g.refresh_agent_invocation_ranks();
        assert_eq!(
            g.ground_items
                .iter()
                .filter(|(_, stack)| stack.position() == at && stack.item() == &cargo_item)
                .count(),
            1
        );
        assert_eq!(g.player_progression.experience(), xp);
    }
    #[test]
    fn mule_cargo_persists_through_save_and_drops_exactly_once_beside_existing_loot() {
        let mut g = game();
        let at = GridPos::new(5, 4);
        let mule = summon(&mut g, "D57", at);
        let item: ItemId = "core:repair_patch".parse().unwrap();
        let owner: crate::social::SocialGroupId = "test:owner".parse().unwrap();
        let cargo = DroneCargo::new(item.clone(), 2, Some(owner.clone()), 2000);
        g.actors
            .get_mut(mule)
            .unwrap()
            .drone_mut()
            .unwrap()
            .load_cargo(cargo)
            .unwrap();
        let rules = g.rules.clone();
        g.drain_events();
        let bytes = bincode::serialize(&g.snapshot().unwrap()).unwrap();
        let mut restored = GameState::from_snapshot(bincode::deserialize(&bytes).unwrap(), rules);
        let cargo = restored
            .actors
            .get(mule)
            .unwrap()
            .drone()
            .unwrap()
            .cargo()
            .unwrap();
        assert_eq!(cargo.quantity(), 2);
        assert_eq!(cargo.owner(), Some(&owner));
        restored.spawn_ground_item(at, item.clone(), 1).unwrap();
        let dead = restored.actors.remove(mule).unwrap();
        // Test the exact death hook independently of final defense and turn ordering.
        restored.drop_agent_drone_cargo(mule, &dead);
        let stacks: Vec<_> = restored
            .ground_items
            .iter()
            .filter(|(_, i)| i.position() == at)
            .collect();
        assert_eq!(stacks.len(), 2);
        assert!(
            stacks
                .iter()
                .any(|(_, i)| i.item() == &item && i.quantity() == 2 && i.owner() == Some(&owner))
        );
    }
}
