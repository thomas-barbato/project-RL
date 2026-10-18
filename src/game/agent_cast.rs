//! First executable catalogue spell. Uses existing commands and actor cooldowns,
//! preserving snapshot and replay layouts while later lots join this dispatch.
use super::*;
use crate::skills::agent;

#[derive(Clone, Copy, Debug)]
pub struct AgentSpellProfile {
    pub range: u16,
    pub energy: u16,
    pub cooldown: u16,
    pub base_damage: u16,
    pub multiplier_numerator: u64,
    pub multiplier_denominator: u64,
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
        let class = content
            .character_classes()
            .get(&"core:arcaniste".parse().unwrap())
            .unwrap();
        let mut rules = GameRules {
            weapons: content.weapons().clone(),
            items: content.items().clone(),
            statuses: content.statuses().clone(),
            player_weapon_slots: ["core:left_hand", "core:right_hand", "core:reserve"]
                .map(|s| s.parse().unwrap())
                .to_vec(),
            ..Default::default()
        };
        class
            .apply_to_rules(&mut rules, class.recommended_attributes())
            .unwrap();
        let mut game = GameState::new_with_rules(Map::from_ascii("###########\n#.........#\n#.........#\n#.........#\n#.........#\n#.........#\n###########").unwrap(),GridPos::new(2,3),19,rules).unwrap();
        game.player_skills.set_agent_skill_rank("A01", 1);
        game.drain_events();
        game
    }
    fn enemy(g: &mut GameState, p: GridPos) -> EntityId {
        let e = g
            .spawn_actor(
                Actor::new(p, 500)
                    .unwrap()
                    .with_player_relation(crate::social::PlayerRelation::Hostile),
            )
            .unwrap();
        g.drain_events();
        e
    }
    #[test]
    fn equipment_skill_grants_follow_equipped_sources_and_never_become_learned() {
        let mut g = fixture();
        g.player_skills = Default::default();
        let (slot, item) = g
            .player_equipment
            .iter()
            .next()
            .map(|(s, i)| (s.clone(), i))
            .unwrap();
        let model = g.player_inventory.get(item).unwrap().item().clone();
        g.rules
            .equipment_agent_skills
            .insert(model.clone(), [("A01".into(), 1)].into());
        let progression = g.export_player_progression().unwrap();
        assert_eq!(g.player_agent_skill_rank("A01"), 1);
        assert!(g.agent_skill_ready("A01").is_ok());
        assert_eq!(g.player_skills.agent_skill_rank("A01"), 0);
        let bytes = bincode::serialize(&g.snapshot().unwrap()).unwrap();
        let mut restored =
            GameState::from_snapshot(bincode::deserialize(&bytes).unwrap(), g.rules.clone());
        assert!(restored.agent_skill_ready("A01").is_ok());
        restored.player_equipment.unequip(&slot);
        assert_eq!(restored.player_agent_skill_rank("A01"), 0);
        assert!(matches!(
            restored.agent_skill_ready("A01"),
            Err(CommandRejection::TechniqueNotLearned(_))
        ));
        // Carrying the source is insufficient, including after snapshot restore.
        assert!(restored.player_inventory.get(item).is_some());
        assert_eq!(restored.export_player_progression().unwrap(), progression);
        let second = g.player_inventory.add(model, 1, 1).unwrap()[0];
        let second_slot = g
            .rules
            .player_weapon_slots
            .iter()
            .find(|s| **s != slot)
            .unwrap()
            .clone();
        g.player_equipment
            .equip(second_slot.clone(), second, &g.player_inventory)
            .unwrap();
        assert_eq!(
            g.process_player_command(GameCommand::DropItem { item }),
            CommandOutcome::Applied
        );
        assert_eq!(g.player_agent_skill_rank("A01"), 1); // Second source remains.
        g.player_equipment.unequip(&second_slot);
        assert_eq!(g.player_agent_skill_rank("A01"), 0);
        g.player_skills.set_agent_skill_rank("A01", 2);
        assert_eq!(g.player_agent_skill_rank("A01"), 2); // Learned rank survives removal.
    }

    #[test]
    fn appearance_slot_cannot_supply_an_equipment_skill() {
        let mut g = fixture();
        g.player_skills = Default::default();
        let (slot, item) = g
            .player_equipment
            .iter()
            .next()
            .map(|(s, i)| (s.clone(), i))
            .unwrap();
        let model = g.player_inventory.get(item).unwrap().item().clone();
        g.rules
            .equipment_agent_skills
            .insert(model, [("A01".into(), 1)].into());
        g.player_equipment.unequip(&slot);
        g.player_equipment
            .set_body_appearance(Some(item), &g.player_inventory)
            .unwrap();
        assert_eq!(g.player_agent_skill_rank("A01"), 0);
    }
    fn cast(p: GridPos) -> GameCommand {
        GameCommand::UseTechniqueAt {
            technique: agent::technique("A01"),
            target: p,
            weapon_slot: 0,
        }
    }
    #[test]
    fn learned_spell_can_repeat_next_action_without_a_ranged_weapon() {
        let mut g = fixture();
        let at = GridPos::new(5, 3);
        let e = enemy(&mut g, at);
        let profile = g.agent_skill_ready("A01").unwrap();
        let energy = g.player_energy.available();
        assert_eq!(g.process_player_command(cast(at)), CommandOutcome::Applied);
        let damage = g
            .events
            .iter()
            .find_map(|event| match event {
                GameEvent::DamageApplied { target, amount, .. } if *target == e => Some(*amount),
                _ => None,
            })
            .unwrap();
        assert!(damage >= profile.damage_before_defenses());
        assert_eq!(g.player_energy.available(), energy - profile.energy);
        assert!(
            g.actors
                .get(g.player)
                .unwrap()
                .technique_cooldown_remaining(&agent::technique("A01"))
                .is_none()
        );
        assert_eq!(profile.cooldown, 1);
        let turn = g.turn;
        g.drain_events();
        assert_eq!(g.process_player_command(cast(at)), CommandOutcome::Applied);
        assert_eq!(g.turn, turn + 1);
        assert_eq!(g.player_energy.available(), energy - 2 * profile.energy);
        assert!(g.actors.get(e).unwrap().integrity() < 500 - damage);
    }

    #[test]
    fn repeating_the_spell_still_gives_the_hostile_its_normal_action() {
        let mut g = fixture();
        let at = GridPos::new(3, 3);
        let hostile = g
            .spawn_actor(
                Actor::new(at, 500)
                    .unwrap()
                    .with_player_relation(crate::social::PlayerRelation::Hostile)
                    .with_attack(crate::combat::AttackProfile::melee(
                        crate::combat::DamageType::Kinetic,
                        1,
                    ))
                    .with_ai(crate::ai::AiProfile::hunter(8, 0)),
            )
            .unwrap();
        g.drain_events();
        for turn in 1..=2 {
            assert_eq!(g.process_player_command(cast(at)), CommandOutcome::Applied);
            assert_eq!(g.turn, turn);
            assert_eq!(g.drain_events().iter().filter(|event|
                matches!(event,GameEvent::AttackPerformed {attacker,..} if *attacker==hostile)
            ).count(),1);
        }
    }
    #[test]
    fn invalid_spell_targets_learning_and_cost_refuse_atomically() {
        let mut g = fixture();
        let at = GridPos::new(5, 3);
        enemy(&mut g, at);
        for p in [
            GridPos::new(1, 1),
            GridPos::new(9, 3),
            GridPos::new(-1, 0),
            g.player_position().unwrap(),
        ] {
            let before = format!("{g:?}");
            assert!(matches!(
                g.process_player_command(cast(p)),
                CommandOutcome::Rejected(_)
            ));
            assert_eq!(format!("{g:?}"), before);
        }
        g.player_skills.set_agent_skill_rank("A01", 0);
        let before = format!("{g:?}");
        assert!(matches!(
            g.process_player_command(cast(at)),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(format!("{g:?}"), before);
        g.player_skills.set_agent_skill_rank("A01", 1);
        let remaining = g.player_energy.available();
        g.player_energy.spend(remaining).unwrap();
        let before = format!("{g:?}");
        assert!(matches!(
            g.process_player_command(cast(at)),
            CommandOutcome::Rejected(CommandRejection::InsufficientEnergy { .. })
        ));
        assert_eq!(format!("{g:?}"), before);
    }
    #[test]
    fn spell_preview_is_free_and_next_action_stays_available_after_snapshot() {
        let mut g = fixture();
        let at = GridPos::new(5, 3);
        enemy(&mut g, at);
        let before = format!("{g:?}");
        for _ in 0..10 {
            g.agent_spell_preview("A01", at).unwrap();
        }
        assert_eq!(format!("{g:?}"), before);
        g.process_player_command(cast(at));
        g.drain_events();
        let rules = g.rules.clone();
        let world = crate::game::WorldState::single(g);
        let saved = world.recovery_snapshot_bytes().unwrap();
        let restored =
            crate::game::WorldState::from_recovery_snapshot_bytes(&saved, rules).unwrap();
        assert!(restored.agent_skill_ready("A01").is_ok());
        // Old save timers must not reinstate an obsolete multi-turn spell delay.
        let mut legacy = restored;
        let player = legacy.player;
        let turn = legacy.turn;
        legacy
            .actors
            .get_mut(player)
            .unwrap()
            .start_technique_cooldown(
                agent::technique("A01"),
                turn,
                crate::time::TimeUnits::new(6).unwrap(),
            );
        assert!(legacy.agent_skill_ready("A01").is_ok());
    }

    #[test]
    fn spell_targeting_respects_walls_protection_and_friendly_actors() {
        let mut g = fixture();
        let target = GridPos::new(5, 3);
        enemy(&mut g, target);
        g.map
            .set_terrain(GridPos::new(4, 3), Terrain::Wall)
            .unwrap();
        g.refresh_player_visibility();
        let before = format!("{g:?}");
        assert!(matches!(
            g.process_player_command(cast(target)),
            CommandOutcome::Rejected(CommandRejection::AbilityNoLineOfSight(_))
        ));
        assert_eq!(format!("{g:?}"), before);
        g.map
            .set_terrain(GridPos::new(4, 3), Terrain::Floor)
            .unwrap();
        g.refresh_player_visibility();
        g.map.set_protected(target, true).unwrap();
        assert!(matches!(
            g.process_player_command(cast(target)),
            CommandOutcome::Rejected(CommandRejection::ProtectedZone)
        ));
        g.map.set_protected(target, false).unwrap();
        let friend = GridPos::new(3, 2);
        g.spawn_actor(
            Actor::new(friend, 100)
                .unwrap()
                .with_player_relation(crate::social::PlayerRelation::Neutral),
        )
        .unwrap();
        assert!(matches!(
            g.process_player_command(cast(friend)),
            CommandOutcome::Rejected(
                CommandRejection::TechniqueTargetDoesNotMeetEngagementRequirement(_)
            )
        ));
        assert!(g.agent_spell_preview("A01", target).is_ok());
    }

    #[test]
    fn spell_profile_combines_current_level_will_class_and_channeling_cost() {
        let mut g = fixture();
        let first = g.agent_skill_ready("A01").unwrap();
        assert_eq!(first.damage_before_defenses(), 9); // 8 * (115% + 3% Willpower).
        assert_eq!(first.energy, 4); // ceil(6 * 80% * 80%).
        assert_eq!(first.range, 5);
        let ranks = g
            .player_skills
            .agent_skills()
            .filter(|(id, _)| *id != "A01")
            .map(|(id, rank)| (id.to_owned(), rank))
            .collect();
        g.player_skills.restore_agent_ranks(ranks).unwrap();
        let mut saved: serde_json::Value =
            serde_json::from_str(&g.export_player_progression().unwrap()).unwrap();
        saved["level"] = 2.into();
        saved["experience"] = g.rules.progression.curve.cumulative_thresholds()[0].into();
        saved["skill_points_earned"] = 1.into();
        saved["unspent_skill_points"] = 1.into();
        g.restore_player_progression(&saved.to_string()).unwrap();
        assert_eq!(
            g.agent_spell_profile("A01", 1)
                .unwrap()
                .damage_before_defenses(),
            10
        );
        assert_eq!(g.agent_spell_profile("A01", 10).unwrap().range, 8);
        // Buying the same rank later uses the same current-level calculation.
        assert_eq!(
            g.process_player_command(GameCommand::LearnAgentSkill {
                skill: "A01".into()
            }),
            CommandOutcome::AppliedWithoutTime
        );
        assert_eq!(
            g.agent_skill_ready("A01").unwrap().damage_before_defenses(),
            10
        );
    }
}
impl AgentSpellProfile {
    pub fn damage_before_defenses(self) -> u16 {
        (u128::from(self.base_damage) * u128::from(self.multiplier_numerator)
            / u128::from(self.multiplier_denominator.max(1)))
        .min(u128::from(u16::MAX)) as u16
    }
}

impl GameState {
    /// Equipment grants are derived from real equipped instances on every query.
    /// They are never inserted into learned ranks, so removal is immediate and
    /// cannot grant permanent progression or survive an absent source on restore.
    pub fn player_agent_equipment_skill_rank(&self, id: &str) -> u8 {
        if agent::definition(id).is_none() {
            return 0;
        }
        self.player_equipment
            .iter()
            .filter(|(slot, _)| self.equipment_slot_is_active(slot))
            .filter_map(|(_, instance)| self.player_inventory.get(instance))
            .filter_map(|entry| self.rules.equipment_agent_skills.get(entry.item()))
            .filter_map(|grants| grants.get(id).copied())
            .max()
            .unwrap_or(0)
    }
    pub fn player_agent_skill_rank(&self, id: &str) -> u8 {
        self.player_skills
            .agent_skill_rank(id)
            .max(self.player_agent_equipment_skill_rank(id))
    }
    pub fn agent_spell_profile(&self, id: &str, rank: u8) -> Option<AgentSpellProfile> {
        if matches!(id, "SF00" | "ES03") {
            return self.agent_stealth_profile(id, rank);
        }
        if matches!(id, "SG00" | "SM00") {
            return self.agent_device_ready(id).ok().map(|p| AgentSpellProfile {
                range: p.range,
                energy: p.energy,
                cooldown: 1,
                base_damage: 0,
                multiplier_numerator: 1,
                multiplier_denominator: 1,
            });
        }
        if super::agent_transform_cast::is_agent_transformation(id) {
            return self.agent_transformation_profile(id, rank);
        }
        if super::agent_support_cast::is_agent_support_action(id) {
            return self.agent_support_profile(id, rank);
        }
        if !matches!(id, "A01" | "A02" | "FE01" | "FE05")
            && !super::agent_magic_cast::is_agent_magic_skill(id)
        {
            return None;
        }
        let definition = agent::definition(id)?;
        let start = self.rules.agent_class_start.as_ref()?;
        let will = self
            .player_effective_primary_attributes()?
            .value(PrimaryAttribute::Willpower);
        let pool = u64::from(will.saturating_sub(5))
            + u64::from(start.spell_damage_percent)
            + u64::from(self.agent_transform_bonus_percent())
            + u64::from(self.agent_stealth_damage_bonus());
        let level_tenths = u64::from(self.player_progression.level().saturating_sub(1)) + 10;
        let channel_rank = self.player_agent_skill_rank("A03");
        let reduction = if channel_rank == 0 {
            0
        } else {
            agent::definition("A03")?
                .value("magical_energy_cost_reduction_percent", channel_rank)
                .min(99)
        };
        let magic = super::agent_magic_cast::is_agent_magic_skill(id);
        let energy_key = if magic {
            Self::agent_magic_energy_key(id)
        } else {
            "energy"
        };
        let energy = (u64::from(definition.value(energy_key, rank))
            * u64::from(start.magic_energy_percent)
            * u64::from(100 - reduction))
        .div_ceil(10_000);
        let (magic_base, magic_n, magic_d) = if magic {
            self.agent_magic_base_damage(id, rank)
        } else {
            (0, 1, 1)
        };
        Some(AgentSpellProfile {
            range: if magic {
                self.agent_magic_range(id, rank)
            } else {
                definition.value("range_cells", rank)
            },
            energy: energy.clamp(1, u64::from(u16::MAX)) as u16,
            cooldown: definition.value("cooldown_turns", rank),
            base_damage: if magic {
                magic_base
            } else {
                definition.value(
                    match id {
                        "FE01" => "raw_fire_damage",
                        "A02" => "raw_occult_damage_per_target",
                        "FE05" => "raw_explosion_fire_damage",
                        _ => "raw_occult_damage",
                    },
                    rank,
                )
            },
            multiplier_numerator: level_tenths * pool * magic_n,
            multiplier_denominator: 1000 * magic_d,
        })
    }
    pub fn agent_skill_ready(&self, id: &str) -> Result<AgentSpellProfile, CommandRejection> {
        let technique = agent::technique(id);
        if self.agent_transform_blocks_skill(id)
            || self.agent_support_effect(self.player, "AF03").is_some()
        {
            return Err(CommandRejection::TechniqueHasNoActiveAction(technique));
        }
        if self.rules.agent_class_start.is_none() {
            return Err(CommandRejection::AgentProgressionUnavailable);
        }
        let rank = self.player_agent_skill_rank(id);
        if rank == 0 {
            return Err(CommandRejection::TechniqueNotLearned(technique));
        }
        if self.agent_transform_blocks_skill(id) {
            return Err(CommandRejection::TechniqueHasNoActiveAction(
                technique.clone(),
            ));
        }
        if super::agent_support_cast::is_agent_support_action(id) {
            return self.agent_support_ready(id);
        }
        if self.agent_support_effect(self.player, "AF03").is_some() {
            return Err(CommandRejection::TechniqueHasNoActiveAction(
                technique.clone(),
            ));
        }
        let profile = self
            .agent_spell_profile(id, rank)
            .ok_or_else(|| CommandRejection::TechniqueHasNoActiveAction(technique.clone()))?;
        if profile.cooldown > 1
            && let Some(t) = self
                .actors
                .get(self.player)
                .and_then(|a| a.technique_cooldown_remaining(&technique))
        {
            return Err(CommandRejection::TechniqueOnCooldown {
                technique,
                remaining_phases: t.get(),
            });
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
        if self.player_energy.available() < profile.energy {
            return Err(CommandRejection::InsufficientEnergy {
                required: profile.energy,
                available: self.player_energy.available(),
            });
        }
        Ok(profile)
    }
    pub fn agent_spell_footprint(
        &self,
        id: &str,
        target: GridPos,
    ) -> Result<AttackPreview, CommandRejection> {
        if matches!(id, "SF00" | "ES03") {
            return self.agent_stealth_preview(id, target);
        }
        if super::agent_transform_cast::is_agent_transformation(id) {
            return self.agent_transformation_preview(id, target);
        }
        if super::agent_support_cast::is_agent_support_action(id) {
            return self.agent_support_preview(id, target);
        }
        if super::agent_magic_cast::is_agent_magic_skill(id) {
            return self.agent_magic_preview(id, target);
        }
        // A temporary inability to cast must not erase the visible footprint.
        // The executable preview below still requires readiness, and casts
        // still pay through the same validated path.
        let profile = match self.agent_skill_ready(id) {
            Ok(profile) => profile,
            Err(
                reason @ (CommandRejection::InsufficientEnergy { .. }
                | CommandRejection::TechniqueOnCooldown { .. }
                | CommandRejection::OffensiveActionBlockedByRecovery { .. }),
            ) => self
                .agent_spell_profile(id, self.player_agent_skill_rank(id))
                .ok_or(reason)?,
            Err(reason) => return Err(reason),
        };
        let origin = self
            .player_position()
            .ok_or(CommandRejection::AgentProgressionUnavailable)?;
        if !self.map.contains(target) {
            return Err(CommandRejection::AbilityTargetOutsideMap(target));
        }
        if !crate::skills::spatial::in_cast_range(origin, target, profile.range) {
            return Err(CommandRejection::AbilityTargetOutOfRange(target));
        }
        if !self.player_visibility.is_visible(target) {
            return Err(CommandRejection::AbilityNoLineOfSight(target));
        }
        if !has_line_of_sight(&self.map, origin, target, true) {
            return Err(CommandRejection::AbilityNoLineOfSight(target));
        }
        if self.map.is_protected(origin) || self.map.is_protected(target) {
            return Err(CommandRejection::ProtectedZone);
        }
        if matches!(id, "A01" | "FE01") {
            return Ok(AttackPreview::new(
                origin,
                target,
                vec![AttackAreaCell {
                    position: target,
                    step: 0,
                }],
            ));
        }
        let target = if id == "FE05" {
            self.agent_fireball_impact(origin, target)
        } else {
            target
        };
        if !self.map.is_walkable(target) {
            return Err(CommandRejection::AbilityTargetBlocked(target));
        }
        let area = self.agent_spell_area(id, self.player_agent_skill_rank(id));
        let cells = crate::skills::spatial::affected_cells(&self.map, target, area, (1, 0))
            .into_iter()
            .filter(|p| self.player_visibility.is_visible(*p) && !self.map.is_protected(*p))
            .map(|position| AttackAreaCell {
                position,
                step: grid_distance(target, position),
            })
            .collect();
        Ok(AttackPreview::new(origin, target, cells))
    }
    pub fn agent_spell_preview(
        &self,
        id: &str,
        target: GridPos,
    ) -> Result<AttackPreview, CommandRejection> {
        if matches!(id, "SF00" | "ES03") {
            return self.agent_stealth_preview(id, target);
        }

        if super::agent_transform_cast::is_agent_transformation(id) {
            return self.agent_transformation_preview(id, target);
        }
        if super::agent_support_cast::is_agent_support_action(id) {
            return self.agent_support_preview(id, target);
        }
        if super::agent_magic_cast::is_agent_magic_skill(id) {
            return self.agent_magic_preview(id, target);
        }
        self.agent_skill_ready(id)?;
        let preview = self.agent_spell_footprint(id, target)?;
        if matches!(id, "A02" | "FE05") {
            return Ok(preview);
        }
        let victim = self
            .actors
            .entity_at(target)
            .ok_or(CommandRejection::AbilityTargetHasNoActor(target))?;
        let actor = self
            .actors
            .get(victim)
            .ok_or(CommandRejection::AbilityTargetHasNoActor(target))?;
        if victim == self.player
            || actor.player_relation() != crate::social::PlayerRelation::Hostile
            || self.is_player_controlled_companion(victim)
        {
            return Err(CommandRejection::TechniqueTargetDoesNotMeetEngagementRequirement(victim));
        }
        Ok(preview)
    }
    pub(super) fn cast_agent_spell(
        &mut self,
        id: &str,
        target: GridPos,
    ) -> Result<(), CommandRejection> {
        if super::agent_transform_cast::is_agent_transformation(id) {
            return self.cast_agent_transformation(id, target, 0);
        }
        if super::agent_support_cast::is_agent_support_action(id) {
            return self.cast_agent_support(id, target);
        }
        if super::agent_magic_cast::is_agent_magic_skill(id) {
            return self.cast_agent_magic(id, target);
        }
        if matches!(id, "A02" | "FE05") {
            self.agent_spell_preview(id, target)?;
            self.cancel_agent_magic_channel();
            return self.cast_agent_area_spell(id, target);
        }
        let preview = self.agent_spell_preview(id, target)?;
        let profile = self.agent_skill_ready(id)?;
        let burn = self.agent_spell_burn_profile(id, self.player_agent_skill_rank(id));
        self.cancel_agent_magic_channel();
        let victim = self
            .actors
            .entity_at(target)
            .expect("validated spell recipient");
        self.player_energy
            .spend(profile.energy)
            .expect("validated spell cost");
        let technique = agent::technique(id);
        let critical = self.rng.percentile() <= 5;
        self.events.push(GameEvent::TechniqueUsed {
            entity: self.player,
            technique: technique.clone(),
            observed_on_turn: self.turn,
        });
        self.events.push(GameEvent::AgentSpellCast {
            entity: self.player,
            victim,
            skill: id.to_owned(),
            origin: preview.origin(),
            target,
            critical,
        });
        self.apply_damage_impact_to_scaled_spell(
            Some(self.player),
            victim,
            DamagePacket::new(
                profile.base_damage,
                if id == "FE01" {
                    crate::combat::DamageType::Thermal
                } else {
                    crate::combat::DamageType::Corruption
                },
                0,
            ),
            u128::from(profile.multiplier_numerator) * if critical { 2 } else { 1 },
            u128::from(profile.multiplier_denominator),
        )
        .expect("validated living spell recipient");
        if let Some((power, chance, duration)) = burn {
            if self.actors.get(victim).is_some()
                && (chance == 100 || u16::from(self.rng.percentile()) <= chance)
            {
                self.apply_agent_burn(victim, power, duration);
            }
        }
        // The casting action already advances one turn. Historical actor timers
        // count additional blocked actions, excluding their arming phase.
        if let Ok(cooldown) = crate::time::TimeUnits::new(profile.cooldown.saturating_sub(1)) {
            self.actors
                .get_mut(self.player)
                .expect("spell caster")
                .start_technique_cooldown(technique.clone(), self.turn, cooldown);
            self.events.push(GameEvent::TechniqueCooldownStarted {
                entity: self.player,
                technique,
                remaining_phases: cooldown.get(),
            });
        }
        self.end_player_active_camouflage();
        Ok(())
    }
}
