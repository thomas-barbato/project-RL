//! Approved self protections, movement and area afflictions.
use super::*;
use crate::game::StatusRemovalReason;
use crate::skills::agent;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AgentTimedSupport {
    pub skill: String,
    pub source: EntityId,
    pub target: EntityId,
    pub rank: u8,
    pub expires: u64,
    pub applied: u64,
    pub potency: u16,
    pub transmitted: bool,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AgentSupportRuntime {
    pub effects: Vec<AgentTimedSupport>,
    pub regeneration: Option<(u8, u16, u64)>,
    pub surge_ready: u64,
    pub energy_return_ready: u64,
    pub necromancy_marks: BTreeMap<EntityId, u8>,
    pub raised: BTreeMap<EntityId, (u8, u64)>,
    pub status_sequence: u64,
    pub status_recency: BTreeMap<EntityId, BTreeMap<StatusId, u64>>,
}

pub(super) fn is_agent_support_action(id: &str) -> bool {
    matches!(
        id,
        "B02"
            | "SU03"
            | "SU04"
            | "SU05"
            | "T02"
            | "ES05"
            | "AF02"
            | "AF03"
            | "AF04"
            | "AF05"
            | "O02"
            | "HY06"
            | "OC07"
    )
}
fn status_name(id: &str) -> &'static str {
    match id {
        "B02" => "aegis",
        "SU03" => "thorns",
        "SU04" => "regeneration",
        "T02" => "spectral_step",
        "ES05" => "phase_traversal",
        "AF02" => "armor_breach",
        "AF03" => "silence",
        "AF04" => "regeneration_break",
        "AF05" => "disorientation",
        "O02" => "interference",
        "HY06" => "domination",
        _ => "last_surge",
    }
}

impl GameState {
    pub(super) fn agent_support_profile(&self, id: &str, rank: u8) -> Option<AgentSpellProfile> {
        if !is_agent_support_action(id) || rank == 0 {
            return None;
        }
        let def = agent::definition(id)?;
        let start = self.rules.agent_class_start.as_ref()?;
        let key = if id == "SU04" {
            "energy_per_turn"
        } else {
            "energy"
        };
        let channel = self.player_agent_skill_rank("A03");
        let reduction = if channel > 0 {
            agent::definition("A03")?
                .value("magical_energy_cost_reduction_percent", channel)
                .min(99)
        } else {
            0
        };
        let energy = (u64::from(def.value(key, rank))
            * u64::from(start.magic_energy_percent)
            * u64::from(100 - reduction))
        .div_ceil(10000)
        .clamp(1, 65535) as u16;
        let range = if matches!(id, "T02" | "ES05") {
            def.value("movement_cells", rank)
        } else if def.parameters.contains_key("range_cells") {
            def.value("range_cells", rank)
        } else {
            0
        };
        Some(AgentSpellProfile {
            range,
            energy,
            cooldown: 1,
            base_damage: 0,
            multiplier_numerator: 1,
            multiplier_denominator: 1,
        })
    }
    pub(super) fn agent_support_ready(
        &self,
        id: &str,
    ) -> Result<AgentSpellProfile, CommandRejection> {
        let rank = self.player_agent_skill_rank(id);
        if rank == 0 {
            return Err(CommandRejection::TechniqueNotLearned(agent::technique(id)));
        }
        if self.agent_support_effect(self.player, "AF03").is_some() {
            return Err(CommandRejection::TechniqueHasNoActiveAction(
                agent::technique(id),
            ));
        }
        let profile = self
            .agent_support_profile(id, rank)
            .ok_or(CommandRejection::AgentProgressionUnavailable)?;
        if self.player_energy.available() < profile.energy {
            return Err(CommandRejection::InsufficientEnergy {
                required: profile.energy,
                available: self.player_energy.available(),
            });
        }
        Ok(profile)
    }
    pub(super) fn agent_support_preview(
        &self,
        id: &str,
        target: GridPos,
    ) -> Result<AttackPreview, CommandRejection> {
        let profile = self.agent_support_ready(id)?;
        let origin = self
            .player_position()
            .ok_or(CommandRejection::MissingPlayer)?;
        if !self.map.contains(target) {
            return Err(CommandRejection::AbilityTargetOutsideMap(target));
        }
        if self.map.is_protected(origin) || self.map.is_protected(target) {
            return Err(CommandRejection::ProtectedZone);
        }
        if !self.player_visibility.is_visible(target) {
            return Err(CommandRejection::AbilityNoLineOfSight(target));
        }
        if matches!(id, "HY06" | "OC07") {
            if !crate::skills::spatial::in_cast_range(origin, target, profile.range) {
                return Err(CommandRejection::AbilityTargetOutOfRange(target));
            }
            if !has_line_of_sight(&self.map, origin, target, true) {
                return Err(CommandRejection::AbilityNoLineOfSight(target));
            }
            let victim = self
                .actors
                .entity_at(target)
                .ok_or(CommandRejection::AbilityTargetHasNoActor(target))?;
            if victim == self.player
                || self.actors.get(victim).is_none_or(|a| {
                    a.player_relation() != crate::social::PlayerRelation::Hostile
                        || a.tags().iter().any(|t| t.as_str() == "core:boss")
                })
            {
                return Err(
                    CommandRejection::TechniqueTargetDoesNotMeetEngagementRequirement(victim),
                );
            }
            return Ok(AttackPreview::new(
                origin,
                target,
                vec![AttackAreaCell {
                    position: target,
                    step: 0,
                }],
            ));
        }
        let affliction = matches!(id, "AF02" | "AF03" | "AF04" | "AF05" | "O02");
        if affliction {
            if !crate::skills::spatial::in_cast_range(origin, target, profile.range) {
                return Err(CommandRejection::AbilityTargetOutOfRange(target));
            }
            if !has_line_of_sight(&self.map, origin, target, true) {
                return Err(CommandRejection::AbilityNoLineOfSight(target));
            }
            let width = agent::definition(id)
                .unwrap()
                .value("zone_width_cells", self.player_agent_skill_rank(id));
            let cells = crate::skills::spatial::affected_cells(
                &self.map,
                target,
                crate::skills::spatial::Area::Square { radius: width / 2 },
                (1, 0),
            )
            .into_iter()
            .filter(|p| self.player_visibility.is_visible(*p) && !self.map.is_protected(*p))
            .map(|position| AttackAreaCell {
                position,
                step: grid_distance(target, position),
            })
            .collect();
            return Ok(AttackPreview::new(origin, target, cells));
        }
        if matches!(id, "T02" | "ES05") {
            if target == origin
                || !crate::skills::spatial::in_cast_range(origin, target, profile.range)
            {
                return Err(CommandRejection::AbilityTargetOutOfRange(target));
            }
            if self.agent_magic_movement_blocked(self.player) {
                return Err(CommandRejection::AbilityTargetBlocked(target));
            }
            if !self.map.is_walkable(target) || self.actors.entity_at(target).is_some() {
                return Err(CommandRejection::AbilityTargetBlocked(target));
            }
            let trace = agent_movement_line(origin, target);
            for position in trace.into_iter().skip(1) {
                if !self.map.is_walkable(position)
                    || !has_line_of_sight(&self.map, origin, position, true)
                {
                    return Err(CommandRejection::AbilityTargetBlocked(position));
                }
                if let Some(actor) = self.actors.entity_at(position) {
                    if id != "ES05"
                        || self.actors.get(actor).is_none_or(|a| {
                            a.player_relation() != crate::social::PlayerRelation::Hostile
                        })
                    {
                        return Err(CommandRejection::AbilityTargetBlocked(position));
                    }
                }
            }
        } else if target != origin {
            return Err(CommandRejection::AbilityTargetOutOfRange(target));
        }
        if id == "SU05" && self.agent_purge_candidates().is_empty() {
            return Err(CommandRejection::TechniqueHasNoActiveAction(
                agent::technique(id),
            ));
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
    pub(super) fn cast_agent_support(
        &mut self,
        id: &str,
        target: GridPos,
    ) -> Result<(), CommandRejection> {
        let preview = self.agent_support_preview(id, target)?;
        let profile = self.agent_support_ready(id)?;
        let rank = self.player_agent_skill_rank(id);
        let def = agent::definition(id).unwrap();
        self.player_energy
            .spend(profile.energy)
            .expect("preflight energy");
        self.cancel_agent_magic_channel();
        self.agent_runtime_mut().support.regeneration = None;
        self.events.push(GameEvent::TechniqueUsed {
            entity: self.player,
            technique: agent::technique(id),
            observed_on_turn: self.turn,
        });
        self.events.push(GameEvent::AgentSpellCast {
            entity: self.player,
            victim: self.player,
            skill: id.into(),
            origin: preview.origin(),
            target,
            critical: false,
        });
        match id {
            "HY06" => {
                let victim = self.actors.entity_at(target).unwrap();
                if self.agent_occult_application_succeeds(id, rank, victim) {
                    self.end_agent_domination();
                    self.apply_agent_support_status(
                        id,
                        rank,
                        victim,
                        self.agent_affliction_duration(id, rank),
                    );
                }
            }
            "OC07" => {
                let victim = self.actors.entity_at(target).unwrap();
                self.agent_runtime_mut()
                    .support
                    .necromancy_marks
                    .insert(victim, rank);
            }
            "AF02" | "AF03" | "AF04" | "AF05" | "O02" => {
                let targets: Vec<_> = self
                    .actors
                    .iter()
                    .filter_map(|(entity, actor)| {
                        (entity != self.player
                            && actor.player_relation() == crate::social::PlayerRelation::Hostile
                            && preview
                                .cells()
                                .iter()
                                .any(|p| p.position == actor.position()))
                        .then_some(entity)
                    })
                    .collect();
                for victim in targets {
                    if self.agent_occult_application_succeeds(id, rank, victim) {
                        self.apply_agent_support_status(
                            id,
                            rank,
                            victim,
                            self.agent_affliction_duration(id, rank),
                        );
                    }
                }
            }
            "SU05" => {
                let candidates = self.agent_purge_candidates();
                for status in candidates
                    .into_iter()
                    .take(usize::from(def.value("maximum_cleansed_statuses", rank)))
                {
                    self.actors
                        .get_mut(self.player)
                        .unwrap()
                        .remove_status(&status);
                    let player = self.player;
                    if matches!(status.as_str(), "agent:frozen" | "agent:mine_frozen") {
                        self.agent_runtime_mut().magic.frozen.remove(&player);
                        self.clear_agent_device_freeze(player);
                    }
                    self.events.push(GameEvent::StatusRemoved {
                        target: self.player,
                        status,
                        reason: StatusRemovalReason::Cleansed,
                    });
                }
            }
            "SU04" => {
                self.agent_regeneration_heal(rank);
                let remaining = def.value("maximum_channel_turns", rank).saturating_sub(1);
                self.agent_runtime_mut().support.regeneration =
                    Some((rank, remaining, self.turn.saturating_add(1)));
                self.apply_agent_support_status(id, rank, self.player, remaining.saturating_add(1));
            }
            "T02" | "ES05" => {
                self.apply_agent_support_status(
                    id,
                    rank,
                    self.player,
                    def.value("duration_turns", rank),
                );
                let origin = preview.origin();
                let line = agent_movement_line(origin, target);
                // Crossing an actor is exclusive to Traversée de phase. Empty
                // steps retain leave interception and ordinary movement hazards.
                for next in line.into_iter().skip(1) {
                    if self.player_position().is_none() {
                        break;
                    }
                    if self.actors.entity_at(next).is_some() {
                        continue;
                    }
                    let from = self.player_position().unwrap();
                    if !self.resolve_voluntary_leave_interceptions(self.player, from, next, 0) {
                        break;
                    }
                    if self.actors.get(self.player).is_none() {
                        break;
                    }
                    self.actors
                        .move_to(self.player, next)
                        .expect("validated landing");
                    self.record_agent_movement(self.player, from, next);
                    self.resolve_status_trigger_for(self.player, StatusTrigger::Movement);
                }
                self.refresh_player_visibility();
            }
            _ => self.apply_agent_support_status(
                id,
                rank,
                self.player,
                def.value("duration_turns", rank),
            ),
        }
        Ok(())
    }
    fn agent_occult_application_succeeds(&mut self, id: &str, rank: u8, target: EntityId) -> bool {
        self.agent_occult_application_with_potency(id, rank, target, 100)
    }
    fn agent_occult_application_with_potency(
        &mut self,
        id: &str,
        rank: u8,
        target: EntityId,
        potency: u16,
    ) -> bool {
        let Some(actor) = self.actors.get(target) else {
            return false;
        };
        let control = matches!(id, "AF03" | "AF05" | "HY06");
        if control && actor.tags().iter().any(|t| t.as_str() == "core:boss") {
            return false;
        }
        let elite = control && actor.tags().iter().any(|t| t.as_str() == "core:elite");
        let processing = self
            .player_effective_primary_attributes()
            .map_or(5, |a| a.value(PrimaryAttribute::Processing));
        let power = u64::from(
            agent::definition(id)
                .unwrap()
                .value("application_power_rank_base", rank),
        ) + 2 * u64::from(processing.saturating_sub(5));
        let will = self
            .actor_effective_primary_attributes(target)
            .map_or(5, |a| a.value(PrimaryAttribute::Willpower));
        let resistance = (25 + 2 * (i32::from(will) - 5)).max(1) as u64;
        let power = power * u64::from(potency) / 100;
        let chance = (100 * power / (power + resistance)).clamp(5, 95);
        let chance = if elite { chance * 3 / 4 } else { chance };
        u64::from(self.rng.percentile()) <= chance
    }
    pub(super) fn apply_agent_support_status(
        &mut self,
        id: &str,
        rank: u8,
        target: EntityId,
        duration: u16,
    ) {
        let status: StatusId = format!("agent:{}", status_name(id)).parse().unwrap();
        let turn = self.turn;
        let player = self.player;
        if id == "HY06"
            && let Some(actor) = self.actors.get_mut(target)
        {
            *actor = actor
                .clone()
                .with_player_relation(crate::social::PlayerRelation::Allied);
        }
        self.agent_runtime_mut()
            .support
            .effects
            .retain(|e| e.skill != id || e.target != target);
        self.agent_runtime_mut()
            .support
            .effects
            .push(AgentTimedSupport {
                skill: id.into(),
                source: player,
                target,
                rank,
                expires: turn.saturating_add(u64::from(duration)),
                applied: turn,
                potency: 100,
                transmitted: false,
            });
        self.apply_status_to(
            Some(player),
            target,
            &ApplyStatusEffect::new(status.clone(), 1).unwrap(),
        )
        .expect("installed agent status");
        if let Some(actor) = self.actors.get_mut(target) {
            actor.set_status_duration(&status, duration);
        }
        if let Some(GameEvent::StatusApplied {
            remaining_turns, ..
        }) = self.events.iter_mut().rev().find(
            |e| matches!(e,GameEvent::StatusApplied{target:t,status:s,..}if *t==target&&s==&status),
        ) {
            *remaining_turns = Some(duration);
        }
    }
    pub(super) fn agent_support_effect(
        &self,
        target: EntityId,
        id: &str,
    ) -> Option<&AgentTimedSupport> {
        self.agent_runtime().support.effects.iter().find(|e| {
            e.target == target
                && e.skill == id
                && e.expires > self.turn
                && self.actors.get(target).is_some_and(|a| {
                    a.status(&format!("agent:{}", status_name(id)).parse().unwrap())
                        .is_some()
                })
        })
    }
    fn agent_purge_candidates(&self) -> Vec<StatusId> {
        let Some(actor) = self.actors.get(self.player) else {
            return Vec::new();
        };
        let mut statuses: Vec<_> = actor
            .statuses()
            .filter(|s| {
                matches!(
                    s.definition.as_str(),
                    "core:poisoned"
                        | "core:burning"
                        | "core:slowed"
                        | "core:blind"
                        | "agent:blind"
                        | "agent:frozen"
                        | "agent:mine_frozen"
                )
            })
            .map(|s| s.definition.clone())
            .collect();
        statuses.sort_by_key(|s| {
            std::cmp::Reverse(
                self.agent_runtime()
                    .support
                    .status_recency
                    .get(&self.player)
                    .and_then(|m| m.get(s))
                    .copied()
                    .unwrap_or(0),
            )
        });
        statuses
    }
    pub(super) fn agent_regeneration_heal(&mut self, rank: u8) {
        if self.agent_support_effect(self.player, "AF04").is_some() {
            return;
        }
        let percent = agent::definition("SU04")
            .unwrap()
            .value("heal_percent_of_max_HP_per_turn", rank);
        let actor = self.actors.get_mut(self.player).unwrap();
        let amount =
            (u32::from(actor.maximum_integrity()) * u32::from(percent) / 100).min(65535) as u16;
        let amount = actor.restore_integrity(amount);
        self.events.push(GameEvent::AllyHealed {
            medic: self.player,
            target: self.player,
            amount,
        });
    }
    pub(super) fn process_agent_support_turn(&mut self) {
        if let Some((rank, remaining, next)) = self.agent_runtime().support.regeneration {
            if remaining > 0 && next <= self.turn {
                let profile = self.agent_support_profile("SU04", rank).unwrap();
                let wounded = self
                    .actors
                    .get(self.player)
                    .is_some_and(|a| a.integrity() < a.maximum_integrity());
                if wounded
                    && self.player_energy.available() >= profile.energy
                    && self.agent_support_effect(self.player, "AF03").is_none()
                {
                    self.player_energy.spend(profile.energy).unwrap();
                    self.agent_regeneration_heal(rank);
                    self.agent_runtime_mut().support.regeneration = if remaining > 1 {
                        Some((rank, remaining - 1, self.turn + 1))
                    } else {
                        None
                    };
                } else {
                    self.agent_runtime_mut().support.regeneration = None;
                }
            }
        }
        let expired_domination: Vec<_> = self
            .agent_runtime()
            .support
            .effects
            .iter()
            .filter_map(|e| (e.skill == "HY06" && e.expires <= self.turn + 1).then_some(e.target))
            .collect();
        for entity in expired_domination {
            self.restore_agent_domination_target(entity);
        }
        let expired: Vec<_> = self
            .agent_runtime()
            .support
            .raised
            .iter()
            .filter_map(|(e, (_, expires))| (*expires <= self.turn).then_some(*e))
            .collect();
        for entity in expired {
            self.actors.remove(entity);
            self.agent_runtime_mut().support.raised.remove(&entity);
        }
        let turn = self.turn;
        // Targets in an inactive zone still own these effects. Their absolute
        // expiration bounds storage without discarding their power on travel.
        self.agent_runtime_mut()
            .support
            .effects
            .retain(|e| e.expires > turn);
    }
    /// Mitigation is after ordinary defenses; periodic/reflective damage is
    /// excluded. Shared by spells and ordinary physical attacks.
    pub(super) fn agent_support_incoming_damage(
        &mut self,
        target: EntityId,
        amount: u16,
        direct: bool,
    ) -> u16 {
        if !direct {
            return amount;
        }
        let mut amount = amount;
        if let Some(effect) = self.agent_support_effect(target, "B02") {
            let reduction = agent::definition("B02")
                .unwrap()
                .value("direct_damage_reduction_percent", effect.rank)
                .min(100);
            amount = (u32::from(amount) * u32::from(100 - reduction) / 100) as u16;
        }
        if target == self.player
            && amount >= self.actors.get(target).map_or(0, Actor::integrity)
            && self.turn >= self.agent_runtime().support.surge_ready
        {
            let rank = self.player_agent_skill_rank("SU06");
            if rank > 0 && self.actors.get(target).is_some_and(|a| a.is_alive()) {
                self.agent_runtime_mut().support.surge_ready = self.turn
                    + u64::from(
                        agent::definition("SU06")
                            .unwrap()
                            .value("cooldown_turns", rank),
                    );
                amount = self.actors.get(target).unwrap().integrity() - 1;
                self.events.push(GameEvent::AgentSpellCast {
                    entity: self.player,
                    victim: self.player,
                    skill: "SU06".into(),
                    origin: self.player_position().unwrap(),
                    target: self.player_position().unwrap(),
                    critical: false,
                });
            }
        }
        amount
    }
    pub(super) fn agent_support_evasion_bonus(&self, target: EntityId, ranged: bool) -> i16 {
        let mut result = 0u16;
        for id in ["T02", "ES05"] {
            if let Some(e) = self.agent_support_effect(target, id) {
                result = result.saturating_add(
                    agent::definition(id)
                        .unwrap()
                        .value("evasion_bonus_units", e.rank),
                );
            }
        }
        if target == self.player && ranged {
            let rank = self.player_agent_skill_rank("ES06");
            if rank > 0 {
                result = result.saturating_add(
                    agent::definition("ES06")
                        .unwrap()
                        .value("projectile_evasion_bonus_units", rank),
                );
            }
        }
        result.min(i16::MAX as u16) as i16
    }
}

fn agent_movement_line(a: GridPos, b: GridPos) -> Vec<GridPos> {
    let (mut x, mut y) = (a.x, a.y);
    let dx = (b.x - x).abs();
    let sx = if x < b.x { 1 } else { -1 };
    let dy = -(b.y - y).abs();
    let sy = if y < b.y { 1 } else { -1 };
    let mut err = dx + dy;
    let mut line = Vec::new();
    loop {
        line.push(GridPos::new(x, y));
        if x == b.x && y == b.y {
            break;
        }
        let twice = 2 * err;
        if twice >= dy {
            err += dy;
            x += sx;
        }
        if twice <= dx {
            err += dx;
            y += sy;
        }
    }
    line
}

impl GameState {
    pub(super) fn raise_agent_marked_actor(&mut self, entity: EntityId, dead: &Actor) {
        let Some(rank) = self
            .agent_runtime_mut()
            .support
            .necromancy_marks
            .remove(&entity)
        else {
            return;
        };
        let def = agent::definition("OC07").unwrap();
        let limit = usize::from(def.value("maximum_raised_allies", rank));
        if limit == 0 {
            return;
        }
        while self.agent_runtime().support.raised.len() >= limit {
            let Some(oldest) = self.agent_runtime().support.raised.keys().next().copied() else {
                break;
            };
            self.actors.remove(oldest);
            self.agent_runtime_mut().support.raised.remove(&oldest);
        }
        let percent = def.value("origin_HP_and_damage_power_rank_percent", rank);
        let mut raised = dead.clone().into_agent_raised(percent);
        raised.delay_next_action_until(self.turn.saturating_add(1));
        if let Ok(new) = self.spawn_actor(raised) {
            let until = self.turn + u64::from(def.value("raised_duration_turns", rank));
            self.agent_runtime_mut()
                .support
                .raised
                .insert(new, (rank, until));
            // Ordinary spawn initializes body HP. Reapply the accepted share of
            // the original creature after that initialization, without healing
            // or scaling the copied attacks a second time.
            let maximum = (u32::from(dead.maximum_integrity()) * u32::from(percent) / 100)
                .clamp(1, u32::from(u16::MAX)) as u16;
            self.actors
                .get_mut(new)
                .unwrap()
                .resize_integrity_without_healing(maximum);
            self.events.push(GameEvent::AgentReanimated {
                original: entity,
                raised: new,
                at: dead.position(),
            });
        }
    }
    /// Presentation reads the actual persistent mark, not a timed cast animation.
    pub fn agent_reanimation_marked(&self, entity: EntityId) -> bool {
        self.agent_runtime()
            .support
            .necromancy_marks
            .contains_key(&entity)
            && self.actors.get(entity).is_some_and(Actor::is_alive)
    }
    pub fn agent_reanimated_ally(&self, entity: EntityId) -> bool {
        self.agent_runtime().support.raised.contains_key(&entity)
            && self.actors.get(entity).is_some_and(Actor::is_alive)
    }
    /// Read-only HUD projection of the actual resurrection deadline.
    pub fn agent_reanimated_remaining_turns(&self, entity: EntityId) -> Option<u64> {
        self.agent_runtime()
            .support
            .raised
            .get(&entity)
            .filter(|_| self.actors.get(entity).is_some_and(Actor::is_alive))
            .map(|(_, expires)| expires.saturating_sub(self.turn))
    }
    /// Effective support values for a live actor's HUD; the stored application
    /// rank and curse potency may differ from the caster's current equipment.
    pub fn agent_status_hud_value(
        &self,
        target: EntityId,
        status: &StatusId,
    ) -> Option<(&'static str, u16)> {
        let (id, key, use_potency) = match status.as_str() {
            "agent:aegis" => ("B02", "direct_damage_reduction_percent", false),
            "agent:thorns" => ("SU03", "reflected_direct_melee_damage_percent", false),
            "agent:armor_breach" => ("AF02", "physical_armor_reduction_percent", true),
            "agent:interference" => ("O02", "direct_damage_reduction_percent", true),
            "agent:spectral_step" => ("T02", "evasion_bonus_units", false),
            "agent:phase_traversal" => ("ES05", "evasion_bonus_units", false),
            _ => return None,
        };
        let effect = self.agent_support_effect(target, id)?;
        let value = agent::definition(id)?.value(key, effect.rank);
        Some((
            key,
            if use_potency {
                (u32::from(value) * u32::from(effect.potency) / 100).min(65535) as u16
            } else {
                value
            },
        ))
    }
    pub(super) fn agent_support_owned(&self, entity: EntityId) -> bool {
        self.agent_runtime().support.raised.contains_key(&entity)
            || self.agent_support_effect(entity, "HY06").is_some()
    }
    pub(super) fn agent_support_ai_target(&self, entity: EntityId) -> Option<EntityId> {
        let actor = self.actors.get(entity)?;
        let confused = self.agent_support_effect(entity, "AF05").is_some();
        if !confused && !self.agent_support_owned(entity) {
            return None;
        }
        self.actors
            .iter()
            .filter(|(id, a)| {
                *id != entity
                    && (*id != self.player || confused)
                    && a.is_alive()
                    && !self.map.is_protected(a.position())
                    && self.actor_perceives_position(entity, a.position())
                    && (confused
                        || (a.player_relation() == crate::social::PlayerRelation::Hostile
                            && !self.agent_support_owned(*id)))
            })
            .min_by_key(|(id, a)| (grid_distance(actor.position(), a.position()), *id))
            .map(|(id, _)| id)
    }
    pub(super) fn agent_support_damage_feedback(
        &mut self,
        source: Option<EntityId>,
        target: EntityId,
        amount: u16,
        direct: bool,
        physical: bool,
    ) {
        if amount == 0 {
            return;
        }
        if self.agent_support_effect(target, "AF05").is_some() {
            self.agent_runtime_mut()
                .support
                .effects
                .retain(|e| e.target != target || e.skill != "AF05");
            let status: StatusId = "agent:disorientation".parse().unwrap();
            if let Some(a) = self.actors.get_mut(target) {
                a.remove_status(&status);
            }
            self.events.push(GameEvent::StatusRemoved {
                target,
                status,
                reason: StatusRemovalReason::Consumed,
            });
        }
        if !direct {
            return;
        }
        if let Some(source) = source.filter(|s| *s != target) {
            if !physical
                && let Some(buff) = self
                    .agent_weapon_buff("BM05")
                    .filter(|_| target == self.player)
            {
                let value = agent::definition("BM05")
                    .unwrap()
                    .value("reflected_direct_magic_damage_percent", buff.rank);
                let reflected = (u32::from(amount) * u32::from(value) / 100).min(65535) as u16;
                if reflected > 0 {
                    let _ = self.apply_agent_net_damage(
                        Some(target),
                        source,
                        reflected,
                        crate::combat::DamageType::Corruption,
                    );
                }
            }
        }
    }
    pub(super) fn agent_thorns_after_hit(
        &mut self,
        attacker: EntityId,
        target: EntityId,
        attack: AttackProfile,
        actual: u16,
    ) {
        if actual == 0 || attack.delivery() != AttackDelivery::Melee {
            return;
        }
        if let Some(effect) = self.agent_support_effect(target, "SU03") {
            let value = agent::definition("SU03")
                .unwrap()
                .value("reflected_direct_melee_damage_percent", effect.rank);
            let reflected = (u32::from(actual) * u32::from(value) / 100).min(65535) as u16;
            if reflected > 0 {
                let _ = self.apply_agent_net_damage(
                    Some(target),
                    attacker,
                    reflected,
                    crate::combat::DamageType::Piercing,
                );
            }
        }
    }
    pub(super) fn agent_support_outgoing_percent(&self, source: EntityId) -> u16 {
        self.agent_support_effect(source, "O02").map_or(100, |e| {
            100 - ((u32::from(
                agent::definition("O02")
                    .unwrap()
                    .value("direct_damage_reduction_percent", e.rank),
            ) * u32::from(e.potency)
                / 100)
                .min(100) as u16)
        })
    }
    pub(super) fn agent_armor_after_affliction(
        &self,
        target: EntityId,
        armor: ArmorProfile,
    ) -> ArmorProfile {
        let armor = if target == self.player {
            let bonus = self.agent_transform_armor_bonus_percent();
            let added = (u32::from(armor.total_before_fragilization()) * u32::from(bonus) / 100)
                .min(65535) as u16;
            ArmorProfile::new(
                armor.body(),
                armor.equipment(),
                armor.reinforcement().saturating_add(added),
                armor.fragilization(),
            )
        } else {
            armor
        };
        let reduction = self.agent_support_effect(target, "AF02").map_or(0, |e| {
            (u32::from(
                agent::definition("AF02")
                    .unwrap()
                    .value("physical_armor_reduction_percent", e.rank),
            ) * u32::from(e.potency)
                / 100)
                .min(65535) as u16
        });
        let removed =
            (u32::from(armor.after_fragilization()) * u32::from(reduction) / 100).min(65535) as u16;
        ArmorProfile::new(
            armor.body(),
            armor.equipment(),
            armor.reinforcement(),
            armor.fragilization().saturating_add(removed),
        )
    }
    pub(super) fn agent_support_direct_kill(
        &mut self,
        source: Option<EntityId>,
        target: EntityId,
        eligible: bool,
    ) {
        if source != Some(self.player)
            || !eligible
            || self.turn < self.agent_runtime().support.energy_return_ready
        {
            return;
        }
        let rank = self.player_agent_skill_rank("BD06");
        if rank == 0 {
            return;
        }
        let def = agent::definition("BD06").unwrap();
        let amount = (u32::from(self.player_energy.capacity())
            * u32::from(def.value("energy_return_percent_of_maximum", rank))
            / 100)
            .min(65535) as u16;
        self.player_energy.restore(amount);
        self.agent_runtime_mut().support.energy_return_ready =
            self.turn + u64::from(def.value("minimum_interval_turns", rank));
        let _ = target;
    }
}

impl GameState {
    pub(super) fn agent_contagion_snapshot(&self, target: EntityId) -> Option<AgentTimedSupport> {
        if self.player_agent_skill_rank("AF06") == 0 {
            return None;
        }
        self.agent_runtime()
            .support
            .effects
            .iter()
            .filter(|e| {
                e.target == target
                    && !e.transmitted
                    && e.expires > self.turn
                    && matches!(e.skill.as_str(), "AF02" | "AF03" | "AF04" | "AF05" | "O02")
            })
            .max_by_key(|e| {
                self.agent_runtime()
                    .support
                    .status_recency
                    .get(&target)
                    .and_then(|m| {
                        m.get(&format!("agent:{}", status_name(&e.skill)).parse().unwrap())
                    })
                    .copied()
                    .unwrap_or(0)
            })
            .cloned()
    }
    pub(super) fn spread_agent_contagion(
        &mut self,
        from: GridPos,
        effect: Option<AgentTimedSupport>,
    ) {
        let Some(effect) = effect else {
            return;
        };
        let rank = self.player_agent_skill_rank("AF06");
        if rank == 0 {
            return;
        }
        let def = agent::definition("AF06").unwrap();
        let retained = def.value("transmitted_potency_retained_percent", rank);
        let radius = def.value("zone_width_cells", rank) / 2;
        let area = crate::skills::spatial::affected_cells(
            &self.map,
            from,
            crate::skills::spatial::Area::Square { radius },
            (1, 0),
        );
        let targets: Vec<_> = self
            .actors
            .iter()
            .filter_map(|(id, a)| {
                (a.player_relation() == crate::social::PlayerRelation::Hostile
                    && id != self.player
                    && self.player_visibility.is_visible(a.position())
                    && area.contains(&a.position())
                    && !self.map.is_protected(a.position()))
                .then_some(id)
            })
            .collect();
        let duration = (effect.expires.saturating_sub(self.turn) * u64::from(retained) / 100)
            .clamp(1, 65535) as u16;
        for target in targets {
            if self.agent_occult_application_with_potency(
                &effect.skill,
                effect.rank,
                target,
                retained,
            ) {
                let at = self.actors.get(target).unwrap().position();
                self.events.push(GameEvent::AgentSpellCast {
                    entity: self.player,
                    victim: target,
                    skill: "AF06".into(),
                    origin: from,
                    target: at,
                    critical: false,
                });
                self.apply_agent_support_status(&effect.skill, effect.rank, target, duration);
                if let Some(last) = self.agent_runtime_mut().support.effects.last_mut() {
                    last.potency = retained;
                    last.transmitted = true;
                }
            }
        }
    }
}

impl GameState {
    pub(super) fn agent_healing_blocked(&self, target: EntityId) -> bool {
        self.agent_support_effect(target, "AF04").is_some()
    }
}

impl GameState {
    fn restore_agent_domination_target(&mut self, entity: EntityId) {
        if let Some(actor) = self.actors.get_mut(entity) {
            *actor = actor
                .clone()
                .with_player_relation(crate::social::PlayerRelation::Hostile);
            actor.remove_status(&"agent:domination".parse().unwrap());
        }
        self.agent_runtime_mut()
            .support
            .effects
            .retain(|e| e.skill != "HY06" || e.target != entity);
    }
    pub(super) fn end_agent_domination(&mut self) {
        let controlled: Vec<_> = self
            .agent_runtime()
            .support
            .effects
            .iter()
            .filter_map(|e| (e.skill == "HY06").then_some(e.target))
            .collect();
        for entity in controlled {
            self.restore_agent_domination_target(entity);
        }
    }
    pub(crate) fn end_agent_zone_effects(&mut self) {
        self.clear_agent_devices_on_zone_change();
        self.cancel_agent_magic_channel();
        self.end_agent_domination();
        if let Some(hologram) = self.agent_runtime_mut().weapon.stealth.hologram.take() {
            self.actors.remove(hologram.entity);
        }
        self.agent_runtime_mut().support.regeneration = None;
    }
    pub(super) fn follow_agent_companion(&mut self, entity: EntityId) {
        let Some(origin) = self.actors.get(entity).map(Actor::position) else {
            return;
        };
        let Some(player) = self.player_position() else {
            return;
        };
        if grid_distance(origin, player) <= 1 {
            return;
        }
        let mut goals: Vec<_> = player
            .cardinal_neighbors()
            .into_iter()
            .filter(|p| self.map.is_walkable(*p) && self.actors.entity_at(*p).is_none())
            .collect();
        goals.sort_by_key(|p| grid_distance(origin, *p));
        for goal in goals {
            let path = find_path_with(
                &self.map,
                origin,
                goal,
                self.map.width() * self.map.height(),
                |m, p| m.is_walkable(p),
                |p| self.actors.entity_at(p).is_none() || p == origin,
            );
            if let Some(path) = path
                && let Some(next) = path.get(1)
            {
                let direction = match (next.x - origin.x, next.y - origin.y) {
                    (1, 0) => Direction::East,
                    (-1, 0) => Direction::West,
                    (0, 1) => Direction::South,
                    (0, -1) => Direction::North,
                    _ => return,
                };
                let _ = self.move_entity(entity, direction);
                return;
            }
        }
    }
}

impl GameState {
    fn agent_affliction_duration(&self, id: &str, rank: u8) -> u16 {
        let processing = self
            .player_effective_primary_attributes()
            .map_or(5, |a| a.value(PrimaryAttribute::Processing));
        agent::definition(id)
            .unwrap()
            .value("duration_rank_turns", rank)
            .saturating_add((processing.saturating_sub(5) / 20).min(3))
            .saturating_add(
                self.rules
                    .agent_class_start
                    .as_ref()
                    .map_or(0, |s| s.weakening_duration_bonus),
            )
    }
}
