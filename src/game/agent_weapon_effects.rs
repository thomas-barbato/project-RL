//! Ephemeral attack context and saved state for the reviewed weapon branches.
//! Never adds fields to the serialized weapon/content AttackProfile.
use super::*;
use crate::combat::DamageType;

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct AgentWeaponHit {
    pub skill: String,
    pub rank: u8,
    pub accuracy_percent: u16,
    pub armor_ignored_percent: u16,
    pub critical_bonus_points: u16,
    pub retention: BTreeMap<EntityId, u16>,
    pub primary: Option<EntityId>,
}

impl Default for AgentWeaponHit {
    fn default() -> Self {
        Self {
            skill: String::new(),
            rank: 0,
            accuracy_percent: 100,
            armor_ignored_percent: 0,
            critical_bonus_points: 0,
            retention: BTreeMap::new(),
            primary: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct AgentWeaponBuff {
    pub rank: u8,
    pub expires_on_turn: u64,
    pub added_fire_damage: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct AgentBallisticMark {
    pub target: EntityId,
    pub rank: u8,
    pub expires_on_turn: u64,
    // Consumption is delayed to the end of the action so every projectile of
    // the same native sequence receives its mark bonus, including a miss.
    pub used_on_turn: Option<u64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct AgentWeaponRuntime {
    pub buffs: BTreeMap<String, AgentWeaponBuff>,
    pub mark: Option<AgentBallisticMark>,
    pub tempo: u16,
    pub stationary_turns: u16,
    pub previous_position: Option<GridPos>,
    pub moved_on_turn: Option<u64>,
    pub melee_action_on_turn: Option<u64>,
    pub melee_action_damaged: bool,
    pub melee_action_missed: bool,
    pub drain_turn: u64,
    pub hunger_healed: u16,
    pub cast_healed: u16,
    pub pending_explosions: BTreeMap<crate::explosive::ExplosiveDeviceId, AgentDelayedWeaponImpact>,
    pub bombardment: Option<AgentBombardment>,
    pub stuns: BTreeMap<EntityId, u64>,
    pub stealth: super::agent_stealth_cast::AgentStealthRuntime,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct AgentDelayedWeaponImpact {
    pub context: Option<AgentWeaponHit>,
    pub attack: AttackProfile,
    pub damage: DamageImpact,
    pub origin: GridPos,
    pub source: EntityId,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct AgentBombardment {
    pub center: GridPos,
    pub origin: GridPos,
    pub slot: u8,
    pub remaining: u16,
    pub started_on_turn: u64,
    pub last_fired_on_turn: u64,
}

impl GameState {
    /// (target, accepted icon id, remaining turns, charge count). Zero turns
    /// denotes an untimed passive/mode; presentation time never mutates these.
    pub fn agent_weapon_statuses(&self) -> Vec<(EntityId, String, u16, u16)> {
        let mut effects = Vec::new();
        let state = &self.agent_runtime().weapon;
        for (id, buff) in &state.buffs {
            if buff.expires_on_turn > self.turn {
                effects.push((
                    self.player,
                    id.clone(),
                    buff.expires_on_turn.saturating_sub(self.turn).min(65535) as u16,
                    0,
                ));
            }
        }
        if let Some(mark) = &state.mark
            && mark.expires_on_turn > self.turn
            && self.actors.get(mark.target).is_some()
        {
            effects.push((
                mark.target,
                "T01".into(),
                mark.expires_on_turn.saturating_sub(self.turn).min(65535) as u16,
                0,
            ));
        }
        if state.tempo > 0 {
            effects.push((self.player, "BM03".into(), 0, state.tempo));
        }
        if state.stationary_turns > 0 {
            effects.push((self.player, "BD04".into(), 0, state.stationary_turns));
        }
        if self.agent_stealth_active() {
            effects.push((self.player, "SF00".into(), 0, 0));
        }
        if let Some(channel) = &state.bombardment {
            effects.push((self.player, "HL03".into(), channel.remaining, 0));
        }
        for (entity, until) in &state.stuns {
            if *until > self.turn && self.actors.get(*entity).is_some() {
                effects.push((
                    *entity,
                    "HL02".into(),
                    until.saturating_sub(self.turn).min(65535) as u16,
                    0,
                ));
            }
        }
        effects
    }

    pub(super) fn agent_weapon_is_heavy(&self, weapon: &WeaponDefinition) -> bool {
        weapon.launcher().is_some()
            || self
                .rules
                .monster_equipment_loot
                .as_ref()
                .and_then(|catalog| catalog.family(weapon.id()))
                .is_some_and(|family| {
                    matches!(
                        family.as_str(),
                        "core:machine_guns" | "core:rocket_launchers" | "core:grenade_launchers"
                    )
                })
    }

    pub(super) fn agent_weapon_passive_attack(
        &self,
        attack: AttackProfile,
        slot: u8,
    ) -> AttackProfile {
        if self.rules.agent_class_start.is_none() {
            return attack;
        }
        let mut attack = attack;
        let heavy = self
            .resolved_equipped_player_weapon(slot)
            .is_some_and(|w| self.agent_weapon_is_heavy(&w));
        let mut extra = self.agent_stealth_damage_bonus();
        for (id, key, eligible) in [
            (
                "T03",
                "ranged_attack_damage_bonus_percent",
                attack.delivery() == AttackDelivery::Ranged && !heavy,
            ),
            ("HL04", "heavy_weapon_damage_bonus_percent", heavy),
            (
                "BM03",
                "damage_bonus_percent_per_charge",
                attack.delivery() == AttackDelivery::Melee,
            ),
        ] {
            let rank = self.player_agent_skill_rank(id);
            if eligible && rank > 0 {
                let value = crate::skills::agent::definition(id)
                    .unwrap()
                    .value(key, rank);
                extra = u16::saturating_add(
                    extra,
                    if id == "BM03" {
                        value.saturating_mul(self.agent_runtime().weapon.tempo)
                    } else {
                        value
                    },
                );
            }
        }
        attack = attack
            .with_damage_output_percentage(attack.damage_output_percentage().saturating_add(extra));
        if attack.delivery() == AttackDelivery::Melee
            && let Some(buff) = self.agent_weapon_buff("BM01")
            && buff.added_fire_damage > 0
        {
            let impact = attack.damage();
            let components: Vec<_> = DamageType::ALL
                .into_iter()
                .filter_map(|ty| {
                    let amount =
                        impact
                            .raw_amount(ty)
                            .saturating_add(if ty == DamageType::Thermal {
                                buff.added_fire_damage
                            } else {
                                0
                            });
                    (amount > 0).then_some(crate::combat::DamageComponent::new(amount, ty))
                })
                .collect();
            if components.len() > 1 {
                let penetrations = DamageType::ALL.into_iter().filter_map(|ty| {
                    let p = impact.resistance_penetration(ty);
                    (p > 0).then_some((ty, p))
                });
                if let Ok(mixed) =
                    DamageImpact::mixed(components, impact.armor_penetration(), penetrations)
                {
                    attack = attack.with_damage(mixed);
                }
            } else {
                attack = attack.with_damage(DamageImpact::single(DamagePacket::new(
                    impact
                        .raw_amount(DamageType::Thermal)
                        .saturating_add(buff.added_fire_damage),
                    DamageType::Thermal,
                    impact.resistance_penetration(DamageType::Thermal),
                )));
            }
        }
        attack
    }

    pub(super) fn agent_weapon_buff(&self, id: &str) -> Option<&AgentWeaponBuff> {
        self.agent_runtime()
            .weapon
            .buffs
            .get(id)
            .filter(|b| b.expires_on_turn > self.turn)
    }

    pub(super) fn agent_weapon_accuracy_attack(
        &self,
        attacker: EntityId,
        target: EntityId,
        attack: AttackProfile,
        context: Option<&AgentWeaponHit>,
    ) -> AttackProfile {
        if attacker != self.player || self.rules.agent_class_start.is_none() {
            return attack;
        }
        let mut percentage = u32::from(context.map_or(100, |c| c.accuracy_percent));
        if attack.delivery() == AttackDelivery::Ranged {
            let rank = self.player_agent_skill_rank("BD04");
            if rank > 0 {
                let per = crate::skills::agent::definition("BD04")
                    .unwrap()
                    .value("accuracy_bonus_percent_per_stationary_turn", rank);
                percentage = percentage.saturating_add(
                    u32::from(per) * u32::from(self.agent_runtime().weapon.stationary_turns.min(3)),
                );
            }
            if let Some(mark) = &self.agent_runtime().weapon.mark
                && mark.target == target
                && mark.expires_on_turn > self.turn
            {
                percentage = percentage.saturating_add(u32::from(
                    crate::skills::agent::definition("T01")
                        .unwrap()
                        .value("accuracy_bonus_percent", mark.rank),
                ));
            }
        }
        let Some(rules) = self.rules.hit_rules else {
            return attack;
        };
        let ordinary = rules.accuracy(
            attack.delivery(),
            self.actor_effective_primary_attributes(attacker),
            attack
                .accuracy_modifier()
                .saturating_add(self.actor_accuracy_modifier(attacker)),
        );
        let extra = (i64::from(ordinary) * (i64::from(percentage) - 100) / 100)
            .clamp(i64::from(i16::MIN), i64::from(i16::MAX)) as i16;
        attack.with_accuracy_modifier(attack.accuracy_modifier().saturating_add(extra))
    }

    /// Called only after the accuracy roll succeeded, before target defenses.
    pub(super) fn agent_weapon_hit_damage(
        &mut self,
        attacker: EntityId,
        target: EntityId,
        impact: DamageImpact,
        context: Option<&AgentWeaponHit>,
    ) -> DamageImpact {
        if attacker != self.player || self.rules.agent_class_start.is_none() {
            return impact;
        }
        let mut impact = impact;
        if let Some(context) = context {
            if let Some(retained) = context.retention.get(&target) {
                impact = impact.scaled_percentage(*retained);
            }
            if context.armor_ignored_percent > 0 {
                let ignored = self
                    .actor_armor_profile(target)
                    .map_or(0, ArmorProfile::after_fragilization);
                impact = impact.with_additional_armor_penetration(
                    (u32::from(ignored) * u32::from(context.armor_ignored_percent) / 100) as u16,
                );
            }
        }
        let chance = 5_u16
            .saturating_add(context.map_or(0, |c| c.critical_bonus_points))
            .min(100);
        if u16::from(self.rng.percentile()) <= chance {
            impact = impact.scaled_percentage(200);
        }
        impact
    }

    /// Invoked for every primary hit (not secondary equipment/status damage).
    /// `actual` must be actual HP removed, already capped against overkill.
    pub(super) fn agent_weapon_hit_effects(
        &mut self,
        attacker: EntityId,
        target: EntityId,
        origin: GridPos,
        attack: AttackProfile,
        context: Option<&AgentWeaponHit>,
        actual: u16,
    ) {
        if attacker != self.player || self.rules.agent_class_start.is_none() {
            return;
        }
        if attack.delivery() == AttackDelivery::Melee {
            let turn = self.turn;
            let state = &mut self.agent_runtime_mut().weapon;
            state.melee_action_on_turn = Some(turn);
            state.melee_action_damaged |= actual > 0;
            if actual > 0
                && let Some(buff) = self.agent_weapon_buff("BM02").cloned()
            {
                if let Some(at) = self.actors.get(target).map(Actor::position) {
                    self.events.push(GameEvent::AgentWeaponSkillUsed {
                        entity: attacker,
                        skill: "BM02-HIT".into(),
                        origin,
                        target: at,
                    });
                }
                let percent = crate::skills::agent::definition("BM02").unwrap().value(
                    "life_return_percent_of_actual_eligible_HP_removed",
                    buff.rank,
                );
                self.agent_weapon_restore(actual, percent, 10, true);
            }
        }
        let Some(context) = context else {
            return;
        };
        let definition = crate::skills::agent::definition(&context.skill).unwrap();
        match context.skill.as_str() {
            "DI07" | "HL06" if context.primary == Some(target) && actual > 0 => {
                let percent = definition.value(
                    "life_return_percent_of_actual_primary_HP_removed",
                    context.rank,
                );
                let cap = definition.value(
                    "heal_cap_percent_of_player_maximum_HP_per_use",
                    context.rank,
                );
                self.agent_weapon_restore(actual, percent, cap, false);
            }
            "ME06" if actual > 0 => {
                let amount = u32::from(actual)
                    * u32::from(
                        definition.value("bleed_percent_of_actual_hit_per_turn", context.rank),
                    )
                    / 100;
                let id: StatusId = "core:bleeding".parse().unwrap();
                let duration = definition.value("bleed_duration_turns", context.rank);
                if amount > 0 && self.actors.get(target).is_some() {
                    let _ = self.apply_status_to(
                        Some(attacker),
                        target,
                        &ApplyStatusEffect::new(id.clone(), 1).unwrap(),
                    );
                    if let Some(actor) = self.actors.get_mut(target)
                        && actor.status(&id).is_some()
                    {
                        actor.set_status_power(
                            &id,
                            crate::status::StatusPower {
                                base_damage: amount as u16,
                                numerator: 1,
                                denominator: 1,
                                applied_on_turn: self.turn,
                            },
                            duration,
                        );
                    }
                    if let Some(GameEvent::StatusApplied{remaining_turns,..})=self.events.iter_mut().rev().find(|e|matches!(e,GameEvent::StatusApplied{target:t,status:s,..}if *t==target&&s==&id)){*remaining_turns=Some(duration);}
                    if self
                        .actors
                        .get(target)
                        .is_some_and(|a| a.status(&id).is_some())
                    {
                        self.events.push(GameEvent::StatusTriggered {
                            target,
                            status: id.clone(),
                            trigger: StatusTrigger::TurnEnd,
                        });
                        let _ = self.apply_agent_net_damage(
                            Some(attacker),
                            target,
                            amount as u16,
                            DamageType::Piercing,
                        );
                    }
                }
            }
            "ME04" if self.actors.get(target).is_some() => {
                let distance = definition.value("push_cells", context.rank);
                self.agent_weapon_displace(target, origin, distance, false, actual);
            }
            "DI05" if self.actors.get(target).is_some() => {
                let distance = definition.value("maximum_pull_cells", context.rank);
                self.agent_weapon_displace(target, origin, distance, true, 0);
            }
            _ => {}
        }
    }

    pub(super) fn agent_weapon_miss(&mut self, attacker: EntityId, attack: AttackProfile) {
        if attacker == self.player
            && self.rules.agent_class_start.is_some()
            && attack.delivery() == AttackDelivery::Melee
        {
            let turn = self.turn;
            let state = &mut self.agent_runtime_mut().weapon;
            state.melee_action_on_turn = Some(turn);
            state.melee_action_missed = true;
        }
    }

    pub(super) fn agent_weapon_restore(
        &mut self,
        removed: u16,
        percent: u16,
        cap_percent: u16,
        hunger: bool,
    ) {
        if self.agent_healing_blocked(self.player) {
            return;
        }
        let Some(actor) = self.actors.get(self.player).filter(|a| a.is_alive()) else {
            return;
        };
        let budget = (u32::from(actor.maximum_integrity()) * u32::from(cap_percent) / 100) as u16;
        let turn = self.turn;
        let state = &mut self.agent_runtime_mut().weapon;
        if state.drain_turn != turn {
            state.drain_turn = turn;
            state.hunger_healed = 0;
        }
        let used = if hunger {
            state.hunger_healed
        } else {
            state.cast_healed
        };
        let heal = (u32::from(removed) * u32::from(percent))
            .div_ceil(100)
            .min(u32::from(removed))
            .min(u32::from(budget.saturating_sub(used))) as u16;
        let restored = self
            .actors
            .get_mut(self.player)
            .unwrap()
            .restore_integrity(heal);
        let state = &mut self.agent_runtime_mut().weapon;
        if hunger {
            state.hunger_healed = state.hunger_healed.saturating_add(restored);
        } else {
            state.cast_healed = state.cast_healed.saturating_add(restored);
        }
        if restored > 0 {
            self.events.push(GameEvent::IntegrityRestored {
                entity: self.player,
                amount: restored,
            });
        }
    }

    fn agent_weapon_displace(
        &mut self,
        target: EntityId,
        source_at: GridPos,
        cells: u16,
        pull: bool,
        actual: u16,
    ) {
        let Some(actor) = self.actors.get(target) else {
            return;
        };
        // Content-authored fixed bodies remain anchored; a missing mass profile
        // is not used to invent a resistance roll for these reviewed skills.
        if actor.displacement_profile().is_some_and(|p| p.is_fixed())
            || actor.tags().iter().any(|t| t.as_str() == "core:boss")
        {
            return;
        }
        let elite = actor.tags().iter().any(|t| t.as_str() == "core:elite");
        let mut at = actor.position();
        if elite && self.rng.percentile() > 75 {
            return;
        }
        let step = ((at.x - source_at.x).signum(), (at.y - source_at.y).signum());
        for _ in 0..cells {
            if pull && grid_distance(at, source_at) <= 1 {
                break;
            }
            let direction = if pull { -1 } else { 1 };
            let next = GridPos::new(at.x + step.0 * direction, at.y + step.1 * direction);
            let occupied = self.actors.entity_at(next);
            let blocked = !self.map.is_walkable(next)
                || self.map.is_protected(next)
                || occupied.is_some()
                || !has_line_of_sight(&self.map, at, next, true);
            if blocked {
                if !pull && actual > 0 {
                    let bonus = actual / 4;
                    if bonus > 0 {
                        // Collision is a derivative of net damage, never a new
                        // weapon hit, critical roll, or armor/resistance pass.
                        self.agent_weapon_collision(target, bonus);
                        if let Some(other) =
                            occupied.filter(|id| *id != self.player && *id != target)
                        {
                            self.agent_weapon_collision(other, bonus);
                        }
                    }
                }
                break;
            }
            self.actors.move_to(target, next).expect("existing target");
            self.record_agent_movement(target, at, next);
            if self.actors.get(target).is_none_or(|a| !a.is_alive()) {
                break;
            }
            self.resolve_status_trigger_for(target, StatusTrigger::Movement);
            if self.actors.get(target).is_none_or(|a| !a.is_alive()) {
                break;
            }
            self.resolve_ranged_overwatch_entries(target, next);
            if self.actors.get(target).is_none_or(|a| !a.is_alive())
                || self
                    .actors
                    .get(target)
                    .is_some_and(|a| a.position() != next)
            {
                break;
            }
            at = next;
        }
    }

    fn agent_weapon_collision(&mut self, target: EntityId, amount: u16) {
        let _ = self.apply_agent_net_damage(Some(self.player), target, amount, DamageType::Kinetic);
    }

    /// Reset on every movement event, including a forced out-and-back path.
    pub(super) fn agent_weapon_moved(&mut self, entity: EntityId) {
        if entity != self.player {
            return;
        }
        let turn = self.turn;
        let state = &mut self.agent_runtime_mut().weapon;
        state.moved_on_turn = Some(turn);
        state.tempo = 0;
        state.stationary_turns = 0;
    }

    /// Called once after the player's elapsed action, before turn increments.
    pub(super) fn finish_agent_weapon_action(&mut self) {
        self.advance_agent_hologram();
        if self.rules.agent_class_start.is_none() {
            return;
        }
        let position = self.player_position();
        let turn = self.turn;
        let tempo_rank = self.player_agent_skill_rank("BM03");
        let aim_rank = self.player_agent_skill_rank("BD04");
        let state = &mut self.agent_runtime_mut().weapon;
        let moved = state.moved_on_turn == Some(turn)
            || state.previous_position.is_some_and(|p| Some(p) != position);
        state.stationary_turns = if moved || aim_rank == 0 {
            0
        } else {
            (state.stationary_turns + 1).min(3)
        };
        state.tempo = if tempo_rank > 0
            && !moved
            && state.melee_action_on_turn == Some(turn)
            && state.melee_action_damaged
            && !state.melee_action_missed
        {
            (state.tempo + 1).min(3)
        } else {
            0
        };
        state.previous_position = position;
        state.melee_action_on_turn = None;
        state.melee_action_damaged = false;
        state.melee_action_missed = false;
        if state
            .mark
            .as_ref()
            .is_some_and(|m| m.used_on_turn.is_some() || m.expires_on_turn <= turn + 1)
        {
            state.mark = None;
        }
        state.buffs.retain(|_, b| b.expires_on_turn > turn + 1);
        state.stuns.retain(|_, until| *until > turn + 1);
        if state
            .bombardment
            .as_ref()
            .is_some_and(|b| b.last_fired_on_turn != turn || Some(b.origin) != position)
        {
            state.bombardment = None;
        }
        state.cast_healed = 0;
    }

    /// Before the first projectile; a failed accuracy roll still spends a mark.
    pub(super) fn consume_agent_ballistic_mark(
        &mut self,
        attacker: EntityId,
        target: Option<EntityId>,
        attack: AttackProfile,
    ) {
        if attacker == self.player && attack.delivery() == AttackDelivery::Ranged {
            let turn = self.turn;
            if let Some(mark) = &mut self.agent_runtime_mut().weapon.mark
                && Some(mark.target) == target
            {
                mark.used_on_turn = Some(turn);
            }
        }
    }

    /// Returns true only for a player launch whose authored native delay kept
    /// a full skill context. Ordinary legacy devices retain their old path.
    pub(super) fn resolve_agent_delayed_weapon_payload(
        &mut self,
        resolved: &crate::explosive::ResolvedExplosivePayload,
        cells: &[AttackAreaCell],
    ) -> bool {
        let Some(payload) = self
            .agent_runtime()
            .weapon
            .pending_explosions
            .get(&resolved.device)
            .cloned()
        else {
            return false;
        };
        if resolved.final_payload {
            self.agent_runtime_mut()
                .weapon
                .pending_explosions
                .remove(&resolved.device);
        }
        self.agent_runtime_mut().weapon.cast_healed = 0;
        if let Some(context) = &payload.context {
            self.events.push(GameEvent::AgentAreaSpellCast {
                entity: payload.source,
                skill: format!("{}-BLAST", context.skill),
                origin: payload.origin,
                target: resolved.position,
                radius: cells
                    .iter()
                    .map(|c| grid_distance(resolved.position, c.position))
                    .max()
                    .unwrap_or(0),
            });
        }
        for cell in cells {
            if self.map.is_protected(cell.position) {
                continue;
            }
            let Some(target) = self.actors.entity_at(cell.position) else {
                continue;
            };
            let damage = self.agent_weapon_hit_damage(
                payload.source,
                target,
                payload.damage,
                payload.context.as_ref(),
            );
            if let Ok(application) =
                self.apply_damage_impact_to(Some(payload.source), target, damage)
            {
                self.agent_weapon_hit_effects(
                    payload.source,
                    target,
                    payload.origin,
                    payload.attack,
                    payload.context.as_ref(),
                    application.amount,
                );
            }
        }
        self.agent_impact_stun(payload.source, resolved.position, payload.context.as_ref());
        true
    }

    pub(super) fn agent_impact_stun(
        &mut self,
        source: EntityId,
        center: GridPos,
        context: Option<&AgentWeaponHit>,
    ) {
        let Some(context) = context.filter(|c| c.skill == "HL02") else {
            return;
        };
        let definition = crate::skills::agent::definition("HL02").unwrap();
        let radius = definition.value("stun_radius_cells", context.rank);
        let targets: Vec<_> = self
            .actors
            .iter()
            .filter_map(|(target, actor)| {
                let at = actor.position();
                (target != source
                    && grid_distance(center, at) <= radius
                    && !self.map.is_protected(at)
                    && has_line_of_sight(&self.map, center, at, true))
                .then_some(target)
            })
            .collect();
        for target in targets {
            let Some(actor) = self.actors.get(target) else {
                continue;
            };
            if actor.tags().iter().any(|t| t.as_str() == "core:boss") {
                continue;
            }
            let chance = definition.value("ordinary_stun_chance_percent", context.rank);
            let chance = if actor.tags().iter().any(|t| t.as_str() == "core:elite") {
                chance * 3 / 4
            } else {
                chance
            };
            if u16::from(self.rng.percentile()) <= chance {
                let until =
                    self.turn + u64::from(definition.value("stun_duration_turns", context.rank));
                self.agent_runtime_mut().weapon.stuns.insert(target, until);
                let id: StatusId = "agent:stunned".parse().unwrap();
                if self.rules.statuses.get(&id).is_some() {
                    let _ = self.apply_status_to(
                        Some(source),
                        target,
                        &ApplyStatusEffect::new(id.clone(), 1).unwrap(),
                    );
                    let duration = definition.value("stun_duration_turns", context.rank);
                    if let Some(actor) = self.actors.get_mut(target) {
                        actor.set_status_duration(&id, duration);
                    }
                    if let Some(GameEvent::StatusApplied{remaining_turns,..})=self.events.iter_mut().rev().find(|e|matches!(e,GameEvent::StatusApplied{target:t,status:s,..}if *t==target&&s==&id)){*remaining_turns=Some(duration);}
                }
            }
        }
    }

    pub(super) fn agent_weapon_stunned(&self, target: EntityId) -> bool {
        if self
            .actors
            .get(target)
            .is_some_and(|a| a.status(&"agent:stunned".parse().unwrap()).is_some())
        {
            return true;
        }
        self.agent_runtime()
            .weapon
            .stuns
            .get(&target)
            .is_some_and(|until| *until > self.turn)
    }
}
