//! Reviewed weapon skills use the ordinary weapon resolver, including its
//! accuracy, defenses, item effects, resources and physical attribute rules.
use super::*;
use crate::skills::agent;

#[derive(Clone, Copy, Debug)]
pub struct AgentSkillUseProfile {
    pub range: u16,
    pub energy: u16,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::DamageType;
    use crate::content::ContentLoader;
    use crate::social::PlayerRelation;

    fn fixture() -> GameState {
        fixture_with_weapon(
            WeaponDefinition::new(
                "test:blade".parse().unwrap(),
                "blade".into(),
                "blade".into(),
                AttackProfile::melee(crate::combat::DamageType::Kinetic, 20),
            )
            .unwrap(),
        )
    }

    fn fixture_with_weapon(weapon: WeaponDefinition) -> GameState {
        let content = ContentLoader::load(
            &[std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content")],
            &semver::Version::new(0, 1, 0),
        )
        .unwrap();
        let mut rules = GameRules {
            weapons: content.weapons().clone(),
            items: content.items().clone(),
            statuses: content.statuses().clone(),
            player_weapon_slots: crate::game::WEAPON_HAND_SLOTS
                .map(|s| s.parse().unwrap())
                .into(),
            ..Default::default()
        };
        let class = content
            .character_classes()
            .get(&"core:bastion".parse().unwrap())
            .unwrap();
        class
            .apply_to_rules(&mut rules, class.recommended_attributes())
            .unwrap();
        let id = weapon.id().clone();
        let ranged = weapon.attack().delivery() == AttackDelivery::Ranged;
        rules.weapons.register(weapon).unwrap();
        rules.player_starting_weapons = vec![id.clone()];
        rules.player_starting_equipment = if ranged {
            vec![None, None, Some(id)]
        } else {
            vec![Some(id)]
        };
        let start = rules.agent_class_start.as_mut().unwrap();
        start.weapon_damage_percent = 100;
        start.melee_damage_percent = 110;
        let mut g = GameState::new_with_rules(
            Map::from_ascii("#########\n#.......#\n#.......#\n#.......#\n#.......#\n#########")
                .unwrap(),
            GridPos::new(3, 3),
            19,
            rules,
        )
        .unwrap();
        g.drain_events();
        g
    }
    fn enemy(g: &mut GameState, at: GridPos) -> EntityId {
        let id = g
            .spawn_actor(
                Actor::new(at, 500)
                    .unwrap()
                    .with_evasion_disabled()
                    .with_player_relation(PlayerRelation::Hostile),
            )
            .unwrap();
        g.drain_events();
        id
    }
    fn cast(at: GridPos) -> GameCommand {
        GameCommand::UseTechniqueAt {
            technique: agent::technique("B01"),
            target: at,
            weapon_slot: 0,
        }
    }

    #[test]
    fn rupture_uses_one_weapon_hit_additive_bonus_energy_and_next_action_cadence() {
        let mut g = fixture();
        let at = GridPos::new(4, 3);
        let target = enemy(&mut g, at);
        let untouched = enemy(&mut g, GridPos::new(4, 2));
        let before = format!("{g:?}");
        assert_eq!(g.agent_skill_ready_with_weapon("B01", 0).unwrap().energy, 6);
        for _ in 0..3 {
            g.player_weapon_technique_preview(&agent::technique("B01"), 0, at)
                .unwrap();
        }
        assert_eq!(format!("{g:?}"), before);
        let energy = g.player_energy.available();
        for turn in 1..=2 {
            assert_eq!(g.process_player_command(cast(at)), CommandOutcome::Applied);
            assert_eq!(g.turn, turn);
            assert_eq!(g.player_energy.available(), energy - 6 * turn as u16);
            assert_eq!(
                g.actors.get(target).unwrap().integrity(),
                500 - 30 * turn as u16
            );
            assert_eq!(g.actors.get(untouched).unwrap().integrity(), 500);
            let events = g.drain_events();
            assert_eq!(events.iter().filter(|e| matches!(e, GameEvent::AttackPerformed { attacker, .. } if *attacker==g.player)).count(), 1);
            assert!(events.iter().any(|e| matches!(e, GameEvent::TechniqueUsed { technique, .. } if *technique==agent::technique("B01"))));
            assert!(
                !events
                    .iter()
                    .any(|e| matches!(e, GameEvent::TechniqueCooldownStarted { .. }))
            );
        }
        let world = crate::game::WorldState::single(g);
        let restored = crate::game::WorldState::from_recovery_snapshot_bytes(
            &world.recovery_snapshot_bytes().unwrap(),
            world.rules().clone(),
        )
        .unwrap();
        assert!(restored.agent_skill_ready_with_weapon("B01", 0).is_ok());
    }

    #[test]
    fn rupture_rank_continuation_and_human_energy_use_the_catalogue() {
        let mut g = fixture();
        for (rank, coefficient, energy) in [(1, 140, 6), (10, 230, 9), (12, 250, 9)] {
            g.player_skills.set_agent_skill_rank("B01", rank);
            let (p, a) = g.agent_weapon_skill_attack("B01", 0).unwrap();
            assert_eq!(p.energy, energy);
            assert_eq!(a.damage_output_percentage(), coefficient + 10);
        }
        g.rules
            .agent_class_start
            .as_mut()
            .unwrap()
            .human_skill_energy_percent = 110;
        assert_eq!(
            g.agent_skill_ready_with_weapon("B01", 0).unwrap().energy,
            10
        );
    }

    #[test]
    fn rupture_refusals_are_atomic_for_range_friend_learning_weapon_and_energy() {
        let mut g = fixture();
        let far = GridPos::new(6, 3);
        enemy(&mut g, far);
        for at in [
            far,
            GridPos::new(3, 3),
            GridPos::new(2, 3),
            GridPos::new(-1, 0),
        ] {
            let before = format!("{g:?}");
            assert!(matches!(
                g.process_player_command(cast(at)),
                CommandOutcome::Rejected(_)
            ));
            assert_eq!(format!("{g:?}"), before);
        }
        let near = GridPos::new(4, 3);
        let victim = enemy(&mut g, near);
        g.map.set_protected(near, true).unwrap();
        let before = format!("{g:?}");
        assert!(matches!(
            g.process_player_command(cast(near)),
            CommandOutcome::Rejected(CommandRejection::ProtectedZone)
        ));
        assert_eq!(format!("{g:?}"), before);
        g.map.set_protected(near, false).unwrap();
        let actor = g.actors.get_mut(victim).unwrap();
        *actor = actor.clone().with_player_relation(PlayerRelation::Neutral);
        let before = format!("{g:?}");
        assert!(matches!(
            g.process_player_command(cast(near)),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(format!("{g:?}"), before);
        let actor = g.actors.get_mut(victim).unwrap();
        *actor = actor.clone().with_player_relation(PlayerRelation::Hostile);
        let before = format!("{g:?}");
        assert!(matches!(
            g.process_player_command(GameCommand::UseTechniqueAt {
                technique: agent::technique("B01"),
                target: near,
                weapon_slot: 1,
            }),
            CommandOutcome::Rejected(CommandRejection::MissingAttackSlot(1))
        ));
        assert_eq!(format!("{g:?}"), before);
        g.player_skills.set_agent_skill_rank("B01", 0);
        let before = format!("{g:?}");
        assert!(matches!(
            g.process_player_command(cast(near)),
            CommandOutcome::Rejected(CommandRejection::TechniqueNotLearned(_))
        ));
        assert_eq!(format!("{g:?}"), before);
        g.player_skills.set_agent_skill_rank("B01", 1);
        let energy = g.player_energy.available();
        g.player_energy.spend(energy).unwrap();
        let before = format!("{g:?}");
        assert!(matches!(
            g.process_player_command(cast(near)),
            CommandOutcome::Rejected(CommandRejection::InsufficientEnergy { .. })
        ));
        assert_eq!(format!("{g:?}"), before);
    }
    fn ranged_fixture(burst: bool, delayed: bool) -> GameState {
        let mut weapon = WeaponDefinition::new(
            "test:gun".parse().unwrap(),
            "gun".into(),
            "gun".into(),
            AttackProfile::new(
                6,
                DistanceMetric::Chebyshev,
                true,
                crate::combat::DamageType::Kinetic,
                20,
                0,
            ),
        )
        .unwrap();
        if burst {
            weapon = weapon
                .with_burst(crate::weapon::BurstFire {
                    shots: 3,
                    sustained_accuracy: 0,
                    heat: 0,
                })
                .unwrap();
        }
        if delayed {
            weapon = weapon
                .with_launcher(crate::weapon::Launcher {
                    radius: 1,
                    delay_turns: 2,
                    damage: DamagePacket::new(30, crate::combat::DamageType::Explosive, 0),
                })
                .unwrap();
        }
        fixture_with_weapon(weapon)
    }

    #[test]
    fn fauchage_is_three_adjacent_cells_and_tourbillon_stays_in_contact() {
        let mut g = fixture();
        for id in ["ME02", "ME06"] {
            g.player_skills.set_agent_skill_rank(id, 10);
        }
        let front = GridPos::new(4, 3);
        let preview = g.agent_weapon_skill_preview("ME02", 0, front).unwrap();
        let cells: BTreeSet<_> = preview.cells().iter().map(|c| c.position).collect();
        assert_eq!(
            cells,
            [front, GridPos::new(4, 2), GridPos::new(4, 4)].into()
        );
        let around = g
            .agent_weapon_skill_preview("ME06", 0, GridPos::new(3, 3))
            .unwrap();
        assert_eq!(around.cells().len(), 8);
        assert!(
            around
                .cells()
                .iter()
                .all(|c| grid_distance(GridPos::new(3, 3), c.position) == 1)
        );
    }

    #[test]
    fn piercing_preview_obeys_wall_and_target_count_and_retains_sequential_damage() {
        let mut g = ranged_fixture(false, false);
        g.player_skills.set_agent_skill_rank("DI01", 1);
        let first = enemy(&mut g, GridPos::new(4, 3));
        let second = enemy(&mut g, GridPos::new(5, 3));
        let third = enemy(&mut g, GridPos::new(6, 3));
        let plan = g
            .prepare_agent_weapon_skill("DI01", 2, GridPos::new(4, 3))
            .unwrap();
        let context = plan.attacks[0].agent_weapon.as_ref().unwrap();
        assert_eq!(context.retention.get(&first), Some(&100));
        assert_eq!(context.retention.get(&second), Some(&75));
        assert!(!context.retention.contains_key(&third));
        assert!(
            !plan.attacks[0]
                .affected_cells
                .iter()
                .any(|c| c.position == GridPos::new(6, 3))
        );
        assert_eq!(plan.attacks.len(), 1);
    }

    #[test]
    fn sustained_fire_keeps_two_complete_native_sequences_and_preflights_both() {
        let mut g = ranged_fixture(true, false);
        g.player_skills.set_agent_skill_rank("DI02", 1);
        enemy(&mut g, GridPos::new(5, 3));
        let plan = g
            .prepare_agent_weapon_skill("DI02", 2, GridPos::new(5, 3))
            .unwrap();
        assert_eq!(plan.attacks.len(), 2);
        assert!(plan.native_sequences);
        assert_eq!(
            g.rules
                .weapons
                .get(plan.attacks[0].weapon.as_ref().unwrap())
                .unwrap()
                .burst()
                .unwrap()
                .shots,
            3
        );
        assert_eq!(
            plan.attacks[0]
                .agent_weapon
                .as_ref()
                .unwrap()
                .accuracy_percent,
            90
        );
        let ordinary = g
            .attack_details(g.player, 2)
            .unwrap()
            .1
            .damage_output_percentage();
        assert_eq!(
            plan.attacks[0].attack.damage_output_percentage(),
            ordinary.saturating_sub(40)
        );
        g.player_energy.spend(g.player_energy.available()).unwrap();
        let before = format!("{g:?}");
        assert!(
            g.cast_agent_weapon_skill("DI02", 2, GridPos::new(5, 3))
                .is_err()
        );
        assert_eq!(format!("{g:?}"), before);
    }

    #[test]
    fn delayed_heavy_drain_waits_for_explosion_and_survives_recovery() {
        let mut g = ranged_fixture(false, true);
        g.player_skills.set_agent_skill_rank("HL06", 1);
        let victim = enemy(&mut g, GridPos::new(6, 3));
        let maximum = g.actors.get(g.player).unwrap().maximum_integrity();
        g.actors
            .get_mut(g.player)
            .unwrap()
            .apply_damage(maximum / 2);
        let before = g.actors.get(g.player).unwrap().integrity();
        assert_eq!(
            g.process_player_command(GameCommand::UseTechniqueAt {
                technique: agent::technique("HL06"),
                weapon_slot: 2,
                target: GridPos::new(6, 3),
            }),
            CommandOutcome::Applied
        );
        assert_eq!(g.actors.get(g.player).unwrap().integrity(), before);
        assert_eq!(g.agent_runtime().weapon.pending_explosions.len(), 1);
        g.drain_events();
        let world = crate::game::WorldState::single(g);
        let bytes = world.recovery_snapshot_bytes().unwrap();
        let mut world =
            crate::game::WorldState::from_recovery_snapshot_bytes(&bytes, world.rules().clone())
                .unwrap();
        assert_eq!(world.agent_runtime().weapon.pending_explosions.len(), 1);
        for _ in 0..3 {
            assert_eq!(
                world.process_player_command(GameCommand::Wait),
                CommandOutcome::Applied
            );
        }
        assert!(world.agent_runtime().weapon.pending_explosions.is_empty());
        assert!(world.actors().get(victim).unwrap().integrity() < 480);
        assert_eq!(
            world.actors().get(world.player_id()).unwrap().integrity(),
            before + maximum * 5 / 100
        );
    }

    #[test]
    fn charge_stops_before_striking_when_a_crossed_tile_kills_the_player() {
        use crate::status::{StatusDefinition, StatusEffectPrimitive, StatusHook, StatusStacking};
        let mut g = fixture();
        g.player_skills.set_agent_skill_rank("ME05", 1);
        let victim = enemy(&mut g, GridPos::new(5, 3));
        let hazard: StatusId = "test:lethal_movement".parse().unwrap();
        g.rules
            .statuses
            .register(
                StatusDefinition::new(
                    hazard.clone(),
                    None,
                    StatusStacking::Replace,
                    vec![StatusHook::new(
                        StatusTrigger::Movement,
                        vec![StatusEffectPrimitive::DealDamage {
                            packet: DamagePacket::new(5000, DamageType::Chemical, 0),
                            multiply_by_stacks: false,
                        }],
                    )],
                )
                .unwrap(),
            )
            .unwrap();
        g.apply_status_to(
            Some(g.player),
            g.player,
            &ApplyStatusEffect::new(hazard, 1).unwrap(),
        )
        .unwrap();
        let before = g.player_energy.available();
        g.cast_agent_weapon_skill("ME05", 0, GridPos::new(5, 3))
            .unwrap();
        assert!(g.actors.get(g.player).is_none_or(|a| !a.is_alive()));
        assert_eq!(g.actors.get(victim).unwrap().integrity(), 500);
        assert!(g.player_energy.available() < before);
        assert!(g.events.iter().any(|e|matches!(e,GameEvent::EntityMoved{entity,to,..}if *entity==g.player&&*to==GridPos::new(4,3))));
    }

    #[test]
    fn charge_rejects_occupied_path_before_spending_or_moving() {
        let mut g = fixture();
        g.player_skills.set_agent_skill_rank("ME05", 1);
        enemy(&mut g, GridPos::new(6, 3));
        enemy(&mut g, GridPos::new(4, 3));
        let before = format!("{g:?}");
        assert!(
            g.cast_agent_weapon_skill("ME05", 0, GridPos::new(6, 3))
                .is_err()
        );
        assert_eq!(format!("{g:?}"), before);
    }

    #[test]
    fn vampiric_healing_uses_real_hp_ceiling_and_shared_turn_budget() {
        let mut g = fixture();
        let missing = g.actors.get(g.player).unwrap().maximum_integrity() / 2;
        g.actors.get_mut(g.player).unwrap().apply_damage(missing);
        let before = g.actors.get(g.player).unwrap().integrity();
        let cap = g.actors.get(g.player).unwrap().maximum_integrity() / 10;
        g.agent_weapon_restore(1, 10, 10, true);
        assert_eq!(g.actors.get(g.player).unwrap().integrity(), before + 1);
        g.agent_weapon_restore(500, 100, 10, true);
        assert_eq!(g.actors.get(g.player).unwrap().integrity(), before + cap);
        g.agent_weapon_restore(500, 100, 10, true);
        assert_eq!(g.actors.get(g.player).unwrap().integrity(), before + cap);
    }

    #[test]
    fn blade_fire_is_one_added_component_without_automatic_burning() {
        let mut g = fixture();
        g.player_skills.set_agent_skill_rank("BM01", 1);
        let target = enemy(&mut g, GridPos::new(4, 3));
        g.cast_agent_weapon_skill("BM01", 0, GridPos::new(3, 3))
            .unwrap();
        let (_, attack, _, _, _) = g.attack_details(g.player, 0).unwrap();
        assert!(
            attack
                .damage()
                .raw_amount(crate::combat::DamageType::Thermal)
                > 0
        );
        assert!(
            g.actors
                .get(target)
                .unwrap()
                .status(&"core:burning".parse().unwrap())
                .is_none()
        );
    }
}

#[derive(Clone)]
struct AgentWeaponPlan {
    profile: AgentSkillUseProfile,
    attacks: Vec<PreparedAttack>,
    native_sequences: bool,
    charge_path: Vec<GridPos>,
}

pub(super) fn is_agent_weapon_action(id: &str) -> bool {
    matches!(
        id,
        "B01"
            | "ME02"
            | "ME03"
            | "ME04"
            | "ME05"
            | "ME06"
            | "DI01"
            | "DI02"
            | "DI03"
            | "DI04"
            | "DI05"
            | "DI07"
            | "HL01"
            | "HL02"
            | "HL03"
            | "HL06"
            | "BM01"
            | "BM02"
            | "BM05"
            | "T01"
    )
}

impl GameState {
    pub fn agent_skill_ready_with_weapon(
        &self,
        id: &str,
        slot: u8,
    ) -> Result<AgentSkillUseProfile, CommandRejection> {
        if self.agent_transform_blocks_skill(id) {
            return Err(CommandRejection::TechniqueHasNoActiveAction(
                crate::skills::agent::technique(id),
            ));
        }
        if matches!(id, "SM00" | "SG00") {
            return self.agent_device_ready(id);
        }
        if matches!(id, "D56" | "D57" | "D58" | "D59" | "D66") {
            return self.agent_drone_skill_ready(id);
        }
        if is_agent_weapon_action(id) {
            return self.agent_weapon_skill_attack(id, slot).map(|(p, _)| p);
        }
        if matches!(id, "DR01" | "HY04" | "HY07") {
            return self.agent_drone_utility_ready(id);
        }
        if super::agent_support_cast::is_agent_support_action(id) {
            return self.agent_support_ready(id).map(|p| AgentSkillUseProfile {
                range: p.range,
                energy: p.energy,
            });
        }
        self.agent_skill_ready(id).map(|p| AgentSkillUseProfile {
            range: p.range,
            energy: p.energy,
        })
    }

    fn agent_weapon_skill_attack(
        &self,
        id: &str,
        selected: u8,
    ) -> Result<(AgentSkillUseProfile, AttackProfile), CommandRejection> {
        let start = self
            .rules
            .agent_class_start
            .as_ref()
            .ok_or(CommandRejection::AgentProgressionUnavailable)?;
        let technique = agent::technique(id);
        let rank = self.player_agent_skill_rank(id);
        if rank == 0 {
            return Err(CommandRejection::TechniqueNotLearned(technique));
        }
        if !is_agent_weapon_action(id) {
            return Err(CommandRejection::TechniqueHasNoActiveAction(technique));
        }
        if let Some(remaining) = self
            .actors
            .get(self.player)
            .and_then(Actor::recovery_remaining)
        {
            return Err(CommandRejection::OffensiveActionBlockedByRecovery {
                remaining_actions: remaining.get(),
            });
        }
        let delivery = if id.starts_with("ME") || matches!(id, "B01" | "BM01" | "BM02") {
            AttackDelivery::Melee
        } else {
            AttackDelivery::Ranged
        };
        let slot = if id == "BM05" || self.equipped_player_weapon(selected).is_none() {
            selected
        } else {
            self.player_weapon_slot_for_delivery(selected, delivery)
                .unwrap_or(selected)
        };
        let (_, mut attack, weapon, _, usage) = match self.attack_details(self.player, slot) {
            Ok(details) => details,
            Err(_) if id == "BM05" => (
                self.actors
                    .get(self.player)
                    .cloned()
                    .ok_or(CommandRejection::MissingPlayer)?,
                AttackProfile::melee(crate::combat::DamageType::Kinetic, 1),
                None,
                vec![],
                None,
            ),
            Err(error) => return Err(CommandRejection::from(error)),
        };
        if id != "BM05" && attack.delivery() != delivery {
            return Err(CommandRejection::TechniqueWeaponDeliveryMismatch {
                required: delivery,
                actual: attack.delivery(),
            });
        }
        let heavy = weapon
            .as_ref()
            .and_then(|w| self.rules.weapons.get(w))
            .is_some_and(|w| self.agent_weapon_is_heavy(w));
        if (id.starts_with("DI") && heavy) || (id.starts_with("HL") && !heavy) {
            return Err(CommandRejection::TechniqueWeaponMustTargetSingleActor);
        }
        if attack.area() != AttackArea::Single
            && !id.starts_with("HL")
            && !matches!(id, "BM01" | "BM02" | "BM05" | "T01")
        {
            return Err(CommandRejection::TechniqueWeaponMustTargetSingleActor);
        }
        let definition = agent::definition(id).expect("reviewed weapon skill");
        if matches!(id, "BM01" | "BM02" | "BM05" | "T01")
            && self.agent_support_effect(self.player, "AF03").is_some()
        {
            return Err(CommandRejection::TechniqueHasNoActiveAction(technique));
        }
        let energy_key = if id == "HL03" {
            "energy_per_firing_turn"
        } else {
            "energy"
        };
        let energy = (u32::from(definition.value(energy_key, rank))
            * u32::from(start.human_skill_energy_percent))
        .div_ceil(100)
        .clamp(1, u32::from(u16::MAX)) as u16;
        let sequences = if id == "DI02" {
            2
        } else if matches!(id, "BM01" | "BM02" | "BM05" | "T01") {
            0
        } else {
            1
        };
        let required =
            u32::from(energy) + u32::from(usage.map_or(0, |u| u.energy_per_use)) * sequences;
        if u32::from(self.player_energy.available()) < required {
            return Err(CommandRejection::InsufficientEnergy {
                required: required.min(u32::from(u16::MAX)) as u16,
                available: self.player_energy.available(),
            });
        }
        let range = if id == "ME05" {
            definition.value("maximum_target_distance_cells", rank)
        } else if id == "T01" {
            definition.value("range_cells", rank)
        } else if matches!(id, "BM01" | "BM02" | "BM05") {
            0
        } else if id.starts_with("DI") || id.starts_with("HL") {
            let range = attack
                .range()
                .saturating_add(definition.value("weapon_range_bonus_cells", rank));
            if id == "DI03" {
                range.min(definition.value("range_skill_limit_cells", rank))
            } else {
                range
            }
        } else {
            attack.range()
        };
        attack = attack.with_range(range.max(1));
        if id == "ME05" {
            let tempo_rank = self.player_agent_skill_rank("BM03");
            if tempo_rank > 0 {
                let lost = agent::definition("BM03")
                    .unwrap()
                    .value("damage_bonus_percent_per_charge", tempo_rank)
                    .saturating_mul(self.agent_runtime().weapon.tempo);
                attack = attack.with_damage_output_percentage(
                    attack.damage_output_percentage().saturating_sub(lost),
                );
            }
        }
        let key = if id == "DI02" {
            "damage_coefficient_per_projectile_percent"
        } else {
            "damage_coefficient_percent"
        };
        if definition.parameters.contains_key(key) {
            let coefficient = definition.value(key, rank);
            // Reviewed coefficients below 100 are negative contributions to
            // the same additive pool, not zeroed-out bonuses.
            attack = attack.with_damage_output_percentage(
                (i32::from(attack.damage_output_percentage()) + i32::from(coefficient) - 100)
                    .clamp(0, i32::from(u16::MAX)) as u16,
            );
        }
        Ok((AgentSkillUseProfile { range, energy }, attack))
    }

    fn agent_weapon_target(&self, at: GridPos) -> Result<EntityId, CommandRejection> {
        let target = self
            .actors
            .entity_at(at)
            .ok_or(CommandRejection::AbilityTargetHasNoActor(at))?;
        if target == self.player
            || self.is_player_controlled_companion(target)
            || self
                .actors
                .get(target)
                .is_none_or(|a| a.player_relation() != crate::social::PlayerRelation::Hostile)
        {
            return Err(CommandRejection::TechniqueTargetDoesNotMeetEngagementRequirement(target));
        }
        Ok(target)
    }

    fn prepare_agent_weapon_skill(
        &self,
        id: &str,
        selected: u8,
        target: GridPos,
    ) -> Result<AgentWeaponPlan, CommandRejection> {
        use super::agent_weapon_effects::AgentWeaponHit;
        let (profile, attack) = self.agent_weapon_skill_attack(id, selected)?;
        let origin = self
            .player_position()
            .ok_or(CommandRejection::AgentProgressionUnavailable)?;
        if !self.map.contains(target) {
            return Err(CommandRejection::AbilityTargetOutsideMap(target));
        }
        if !self.player_visibility.is_visible(target)
            || !has_line_of_sight(&self.map, origin, target, true)
        {
            return Err(CommandRejection::AbilityNoLineOfSight(target));
        }
        if self.map.is_protected(origin) || self.map.is_protected(target) {
            return Err(CommandRejection::ProtectedZone);
        }
        if !crate::skills::spatial::in_cast_range(origin, target, profile.range)
            && !matches!(id, "BM01" | "BM02" | "BM05" | "ME06")
        {
            return Err(CommandRejection::AbilityTargetOutOfRange(target));
        }
        if matches!(id, "BM01" | "BM02" | "BM05") {
            if target != origin {
                return Err(CommandRejection::AbilityTargetOutOfRange(target));
            }
            return Ok(AgentWeaponPlan {
                profile,
                attacks: vec![],
                native_sequences: false,
                charge_path: vec![],
            });
        }
        let slot = if self.equipped_player_weapon(selected).is_none() {
            selected
        } else {
            self.player_weapon_slot_for_delivery(selected, attack.delivery())
                .unwrap_or(selected)
        };
        let (_, _, weapon, weapon_effects, module_use) = self
            .attack_details(self.player, slot)
            .map_err(CommandRejection::from)?;
        let rank = self.player_agent_skill_rank(id);
        let definition = agent::definition(id).unwrap();
        let mut context = AgentWeaponHit {
            skill: id.into(),
            rank,
            ..Default::default()
        };
        if matches!(id, "ME03" | "DI04") {
            context.armor_ignored_percent = definition.value("armor_ignored_percent", rank);
        }
        if id == "DI04" {
            context.accuracy_percent =
                100_u16.saturating_add(definition.value("accuracy_bonus_percent", rank));
            context.critical_bonus_points = definition.value("critical_chance_bonus_points", rank);
        }
        if id == "DI02" {
            context.accuracy_percent = definition.value("accuracy_multiplier_percent", rank);
        }
        let mut charge_path = Vec::new();
        let positions: Vec<GridPos> = match id {
            "ME02" => crate::combat::MeleeArc::new(3)
                .unwrap()
                .affected_cells(&self.map, origin, target)
                .into_iter()
                .map(|c| c.position)
                .collect(),
            "ME06" => (-1..=1)
                .flat_map(|dy| (-1..=1).map(move |dx| GridPos::new(origin.x + dx, origin.y + dy)))
                .filter(|p| {
                    *p != origin
                        && self.map.is_walkable(*p)
                        && has_line_of_sight(&self.map, origin, *p, true)
                        && !self.map.is_protected(*p)
                })
                .collect(),
            "DI01" => {
                let mut positions = vec![];
                let mut traversed = 0_u16;
                for at in agent_weapon_line(origin, target, profile.range) {
                    if !self.map.is_walkable(at)
                        || !has_line_of_sight(&self.map, origin, at, true)
                        || self.map.is_protected(at)
                    {
                        break;
                    }
                    positions.push(at);
                    if let Some(actor) = self.actors.entity_at(at) {
                        let mut retention = 100_u32;
                        for _ in 0..traversed {
                            retention = retention * 75 / 100;
                        }
                        context.retention.insert(actor, retention as u16);
                        traversed += 1;
                        if traversed >= definition.value("maximum_targets", rank) {
                            break;
                        }
                    }
                }
                positions
            }
            "DI03" => {
                let dx = i64::from(target.x - origin.x);
                let dy = i64::from(target.y - origin.y);
                if dx == 0 && dy == 0 {
                    return Err(CommandRejection::AbilityTargetBlocked(target));
                }
                let mut candidates: Vec<_> = self
                    .actors
                    .iter()
                    .filter_map(|(entity, actor)| {
                        let at = actor.position();
                        let x = i64::from(at.x - origin.x);
                        let y = i64::from(at.y - origin.y);
                        let dot = x * dx + y * dy;
                        let cross = (x * dy - y * dx).abs();
                        (entity != self.player
                            && actor.player_relation() == crate::social::PlayerRelation::Hostile
                            && !self.is_player_controlled_companion(entity)
                            && dot > 0
                            && cross <= dot
                            && crate::skills::spatial::in_cast_range(origin, at, profile.range)
                            && self.player_visibility.is_visible(at)
                            && !self.map.is_protected(at)
                            && has_line_of_sight(&self.map, origin, at, true))
                        .then_some((grid_distance(origin, at), entity, at))
                    })
                    .collect();
                candidates.sort_by_key(|(d, e, _)| (*d, *e));
                candidates
                    .into_iter()
                    .take(usize::from(definition.value("maximum_targets", rank)))
                    .map(|(_, _, at)| at)
                    .collect()
            }
            "HL01" | "HL03" => {
                let radius = if id == "HL01" {
                    definition
                        .parameters
                        .get("selection_zone_width_cells")
                        .map_or(2, |_| {
                            definition.value("selection_zone_width_cells", rank) / 2
                        })
                } else {
                    definition.value("aim_zone_radius_cells", rank)
                };
                let mut candidates: Vec<_> = self
                    .actors
                    .iter()
                    .filter_map(|(entity, actor)| {
                        let at = actor.position();
                        let distance = grid_distance(target, at);
                        let in_zone = if id == "HL01" {
                            distance <= radius
                        } else {
                            let dx = i64::from(at.x - target.x);
                            let dy = i64::from(at.y - target.y);
                            dx * dx + dy * dy <= i64::from(radius).pow(2)
                        };
                        (in_zone
                            && entity != self.player
                            && actor.player_relation() == crate::social::PlayerRelation::Hostile
                            && !self.is_player_controlled_companion(entity)
                            && self.player_visibility.is_visible(at)
                            && attack.is_in_range(origin, at)
                            && !self.map.is_protected(at)
                            && has_line_of_sight(&self.map, target, at, true)
                            && has_line_of_sight(&self.map, origin, at, true))
                        .then_some((distance, entity, at))
                    })
                    .collect();
                candidates.sort_by_key(|(d, e, _)| (*d, *e));
                candidates
                    .into_iter()
                    .take(if id == "HL01" { 3 } else { 1 })
                    .map(|(_, _, at)| at)
                    .collect()
            }
            _ => {
                context.primary = Some(self.agent_weapon_target(target)?);
                if id == "ME05" {
                    if grid_distance(origin, target)
                        < definition.value("minimum_target_distance_cells", rank)
                    {
                        return Err(CommandRejection::AbilityTargetOutOfRange(target));
                    }
                    let line = agent_weapon_line(origin, target, grid_distance(origin, target));
                    for at in line.into_iter().take_while(|p| *p != target) {
                        if !self.map.is_walkable(at)
                            || self.map.is_protected(at)
                            || self.actors.entity_at(at).is_some()
                            || !has_line_of_sight(&self.map, origin, at, true)
                        {
                            return Err(CommandRejection::AbilityTargetBlocked(at));
                        }
                        charge_path.push(at);
                    }
                    if charge_path
                        .last()
                        .is_none_or(|at| grid_distance(*at, target) != 1)
                    {
                        return Err(CommandRejection::AbilityTargetBlocked(target));
                    }
                }
                vec![target]
            }
        };
        if positions.is_empty() {
            return Err(CommandRejection::AbilityTargetHasNoActor(target));
        }
        if id == "T01" {
            return Ok(AgentWeaponPlan {
                profile,
                attacks: vec![],
                native_sequences: false,
                charge_path: vec![],
            });
        }
        let multi_selected = matches!(id, "DI03" | "HL01");
        let attack_origin = charge_path.last().copied().unwrap_or(origin);
        let make = |at: GridPos, cells: Vec<AttackAreaCell>| PreparedAttack {
            attacker: self.player,
            target: self.actors.entity_at(at),
            slot,
            origin: attack_origin,
            target_at: at,
            attack,
            weapon: weapon.clone(),
            weapon_effects: weapon_effects.clone(),
            module_use,
            affected_cells: cells,
            forced_movement: None,
            technique_on_hit_effect: None,
            damage_target: AttackDamageTarget::Body,
            agent_weapon: Some(context.clone()),
        };
        let mut attacks = if multi_selected {
            positions
                .iter()
                .map(|at| {
                    make(
                        *at,
                        if id == "HL01" {
                            self.weapon_attack_cells(attack, weapon.as_ref(), origin, *at)
                        } else {
                            vec![AttackAreaCell {
                                position: *at,
                                step: 0,
                            }]
                        },
                    )
                })
                .collect::<Vec<_>>()
        } else {
            let cells = if id.starts_with("HL") {
                self.weapon_attack_cells(attack, weapon.as_ref(), origin, target)
            } else {
                positions
                    .iter()
                    .map(|at| AttackAreaCell {
                        position: *at,
                        step: grid_distance(origin, *at),
                    })
                    .collect()
            };
            vec![make(target, cells)]
        };
        if id == "DI02" {
            attacks.push(attacks[0].clone());
        }
        let uses = if multi_selected || id == "DI02" {
            attacks.len() as u16
        } else {
            1
        };
        let first = &attacks[0];
        if id.starts_with("DI") && id != "DI02" {
            self.ensure_agent_projectile_usage(first, uses)
                .map_err(CommandRejection::from)?;
        } else {
            self.ensure_prepared_attack_usage(first, uses)
                .map_err(CommandRejection::from)?;
        }
        let native_heat = first
            .weapon
            .as_ref()
            .and_then(|w| self.rules.weapons.get(w))
            .and_then(WeaponDefinition::burst)
            .map_or(0, |b| b.heat);
        if let Some(heat) = self.player_heat
            && native_heat > 0
            && (id == "DI02" || id.starts_with("HL"))
        {
            let projected = u32::from(heat.current())
                + u32::from(native_heat.saturating_add(module_use.map_or(0, |u| u.heat_per_use)))
                    * u32::from(uses);
            if projected > u32::from(heat.critical_threshold()) {
                return Err(CommandRejection::from(
                    AttackError::EngineeringModuleHeatLimit {
                        module: self
                            .equipped_player_weapon_item(slot)
                            .expect("equipped weapon"),
                        projected: projected.min(65535) as u16,
                        maximum: heat.critical_threshold(),
                    },
                ));
            }
        }
        let required = u32::from(profile.energy)
            + u32::from(module_use.map_or(0, |u| u.energy_per_use)) * u32::from(uses);
        if required > u32::from(self.player_energy.available()) {
            return Err(CommandRejection::InsufficientEnergy {
                required: required.min(65535) as u16,
                available: self.player_energy.available(),
            });
        }
        Ok(AgentWeaponPlan {
            profile,
            attacks,
            native_sequences: matches!(
                id,
                "B01" | "ME03" | "ME04" | "ME05" | "DI02" | "HL01" | "HL02" | "HL03" | "HL06"
            ),
            charge_path,
        })
    }

    pub(super) fn agent_weapon_skill_preview(
        &self,
        id: &str,
        slot: u8,
        target: GridPos,
    ) -> Result<AttackPreview, CommandRejection> {
        let plan = self.prepare_agent_weapon_skill(id, slot, target)?;
        let origin = self
            .player_position()
            .ok_or(CommandRejection::AgentProgressionUnavailable)?;
        let cells = if matches!(id, "BM01" | "BM02" | "BM05") {
            vec![AttackAreaCell {
                position: origin,
                step: 0,
            }]
        } else if id == "T01" {
            vec![AttackAreaCell {
                position: target,
                step: 0,
            }]
        } else {
            plan.attacks
                .iter()
                .flat_map(|p| p.affected_cells.clone())
                .chain(plan.charge_path.iter().map(|p| AttackAreaCell {
                    position: *p,
                    step: 0,
                }))
                .collect()
        };
        Ok(AttackPreview::new(origin, target, cells))
    }

    pub(super) fn cast_agent_weapon_skill(
        &mut self,
        id: &str,
        slot: u8,
        target: GridPos,
    ) -> Result<(), CommandRejection> {
        let plan = self.prepare_agent_weapon_skill(id, slot, target)?;
        let origin = self.player_position().unwrap();
        self.player_energy
            .spend(plan.profile.energy)
            .expect("validated skill energy");
        self.events.push(GameEvent::EnergySpent {
            entity: self.player,
            amount: plan.profile.energy,
            remaining: self.player_energy.available(),
        });
        self.events.push(GameEvent::TechniqueUsed {
            entity: self.player,
            technique: agent::technique(id),
            observed_on_turn: self.turn,
        });
        if !id.starts_with("HL") && id != "DI03" {
            let visual_target = if id == "DI01" {
                plan.attacks
                    .first()
                    .and_then(|a| a.affected_cells.last())
                    .map_or(target, |c| c.position)
            } else {
                target
            };
            self.events.push(GameEvent::AgentWeaponSkillUsed {
                entity: self.player,
                skill: id.into(),
                origin,
                target: visual_target,
            });
        }
        let rank = self.player_agent_skill_rank(id);
        if id == "HL03" {
            let previous = self.agent_runtime().weapon.bombardment.clone().filter(|b| {
                b.center == target
                    && b.origin == origin
                    && b.slot == slot
                    && b.last_fired_on_turn.saturating_add(1) == self.turn
            });
            let remaining = previous.as_ref().map_or_else(
                || {
                    agent::definition("HL03")
                        .unwrap()
                        .value("maximum_firing_turns", rank)
                },
                |b| b.remaining,
            );
            let started_on_turn = previous.map_or(self.turn, |b| b.started_on_turn);
            self.agent_runtime_mut().weapon.bombardment =
                (remaining > 1).then_some(super::agent_weapon_effects::AgentBombardment {
                    center: target,
                    origin,
                    slot,
                    remaining: remaining.saturating_sub(1),
                    started_on_turn,
                    last_fired_on_turn: self.turn,
                });
        }
        if matches!(id, "BM01" | "BM02" | "BM05") {
            let duration = agent::definition(id).unwrap().value("duration_turns", rank);
            let fire = if id == "BM01" {
                let will = self
                    .player_effective_primary_attributes()
                    .map_or(5, |a| a.value(PrimaryAttribute::Willpower));
                let level = u64::from(self.player_progression.level().saturating_sub(1)) + 10;
                let raw = u64::from(
                    agent::definition(id)
                        .unwrap()
                        .value("raw_added_fire_damage", rank),
                );
                let pool = u64::from(will.saturating_sub(5))
                    + u64::from(
                        self.rules
                            .agent_class_start
                            .as_ref()
                            .unwrap()
                            .spell_damage_percent,
                    );
                (raw * level * pool / 1000).min(65535) as u16
            } else {
                0
            };
            let expires_on_turn = self.turn + u64::from(duration);
            self.agent_runtime_mut().weapon.buffs.insert(
                id.into(),
                super::agent_weapon_effects::AgentWeaponBuff {
                    rank,
                    expires_on_turn,
                    added_fire_damage: fire,
                },
            );
        } else if id == "T01" {
            let victim = self.actors.entity_at(target).unwrap();
            let expires_on_turn =
                self.turn + u64::from(agent::definition(id).unwrap().value("duration_turns", rank));
            self.agent_runtime_mut().weapon.mark =
                Some(super::agent_weapon_effects::AgentBallisticMark {
                    target: victim,
                    rank,
                    expires_on_turn,
                    used_on_turn: None,
                });
        } else {
            let mut from = origin;
            for to in plan.charge_path {
                // The path is admitted atomically, but reactions and hazards
                // still resolve on every crossed tile. A paid charge can be
                // interrupted; it cannot attack from a position never reached.
                if self.actors.get(self.player).is_none_or(|a| !a.is_alive())
                    || self
                        .move_entity(self.player, facing_toward(from, to))
                        .is_err()
                    || self.player_position() != Some(to)
                    || self.actors.get(self.player).is_none_or(|a| !a.is_alive())
                {
                    self.end_player_active_camouflage();
                    return Ok(());
                }
                from = to;
            }
            self.agent_runtime_mut().weapon.cast_healed = 0;
            let recovery = plan
                .attacks
                .first()
                .and_then(|p| p.attack.recovery_after_attack());
            let mut paid_single = false;
            let mut attacked = false;
            for prepared in plan.attacks {
                if self.actors.get(self.player).is_none_or(|a| !a.is_alive()) {
                    break;
                }
                if id == "DI03" {
                    self.events.push(GameEvent::AgentWeaponSkillUsed {
                        entity: self.player,
                        skill: id.into(),
                        origin: prepared.origin,
                        target: prepared.target_at,
                    });
                }
                if plan.native_sequences {
                    // Movement/reactions can spend resources after admission.
                    // An interrupted action must not panic or manufacture ammo.
                    if self.ensure_prepared_attack_usage(&prepared, 1).is_err()
                        || self.ensure_native_heat(&prepared).is_err()
                    {
                        break;
                    }
                    if self.execute_native_weapon(prepared).is_err() {
                        break;
                    }
                    attacked = true;
                } else {
                    // Multi-target single projectile skills spend one native
                    // resource unit per selected projectile, not per victim.
                    if !paid_single || id == "DI03" {
                        if id.starts_with("DI") {
                            self.spend_agent_projectile_usage(&prepared, 1)
                                .expect("preflighted projectile supply");
                        } else {
                            self.spend_prepared_attack_usage(&prepared, 1)
                                .expect("preflighted skill supply");
                        }
                        paid_single = true;
                    }
                    self.resolve_prepared_attack(prepared, ActionOrigin::Normal)
                        .expect("preflighted weapon skill");
                    attacked = true;
                }
            }
            if let Some(recovery) = recovery.filter(|_| attacked) {
                self.start_action_recovery(self.player, recovery);
            }
        }
        self.end_player_active_camouflage();
        Ok(())
    }

    fn agent_single_projectile_matter(&self, prepared: &PreparedAttack) -> Option<u16> {
        let weapon = prepared.weapon.as_ref()?;
        let shots = self.rules.weapons.get(weapon)?.burst()?.shots;
        let crate::weapon::WeaponSupply::Matter { amount } = self.weapon_supply(weapon)? else {
            return None;
        };
        // Existing authored burst supplies contain their complete bullet count.
        // Do not infer a new rounded unit for a future indivisible resource.
        (amount % shots == 0).then_some(amount / shots)
    }

    fn ensure_agent_projectile_usage(
        &self,
        prepared: &PreparedAttack,
        uses: u16,
    ) -> Result<(), AttackError> {
        if let Some(matter) = self.agent_single_projectile_matter(prepared) {
            self.ensure_weapon_matter(matter, uses)?;
            let mut unit = prepared.clone();
            unit.weapon = None;
            self.ensure_prepared_attack_usage(&unit, uses)
        } else {
            self.ensure_prepared_attack_usage(prepared, uses)
        }
    }

    fn spend_agent_projectile_usage(
        &mut self,
        prepared: &PreparedAttack,
        uses: u16,
    ) -> Result<(), AttackError> {
        self.ensure_agent_projectile_usage(prepared, uses)?;
        if let Some(matter) = self.agent_single_projectile_matter(prepared) {
            self.spend_weapon_matter(matter, uses)?;
            let mut unit = prepared.clone();
            unit.weapon = None;
            self.spend_prepared_attack_usage(&unit, uses)
        } else {
            self.spend_prepared_attack_usage(prepared, uses)
        }
    }

    /// Read-only replacement for Wait. Other actions are kept intact and will
    /// interrupt the channel only after they successfully consume a turn.
    pub(super) fn agent_bombardment_command(&self, command: &GameCommand) -> Option<GameCommand> {
        if !matches!(command, GameCommand::Wait) {
            return None;
        }
        let channel = self.agent_runtime().weapon.bombardment.as_ref()?;
        if self.player_position() != Some(channel.origin)
            || channel.remaining == 0
            || self
                .prepare_agent_weapon_skill("HL03", channel.slot, channel.center)
                .is_err()
        {
            return None;
        }
        Some(GameCommand::UseTechniqueAt {
            technique: agent::technique("HL03"),
            target: channel.center,
            weapon_slot: channel.slot,
        })
    }
}

fn agent_weapon_line(origin: GridPos, toward: GridPos, range: u16) -> Vec<GridPos> {
    let dx = i64::from(toward.x) - i64::from(origin.x);
    let dy = i64::from(toward.y) - i64::from(origin.y);
    let major = dx.abs().max(dy.abs());
    if major == 0 {
        return vec![];
    }
    (1..=range)
        .map(|step| {
            let offset =
                |delta: i64| ((delta.abs() * i64::from(step) + major / 2) / major) * delta.signum();
            GridPos::new(
                (i64::from(origin.x) + offset(dx)) as i32,
                (i64::from(origin.y) + offset(dy)) as i32,
            )
        })
        .collect()
}
