//! Playable, reviewed Feu, Froid and occult spells. Values come from the catalogue.
use super::agent_magic_runtime::{AgentFrozen, AgentMagicBarrier, AgentMagicEffect};
use super::*;
use crate::combat::DamageType;
use crate::skills::{
    agent,
    spatial::{self, Area},
};
use crate::status::StatusPower;

pub(super) fn is_agent_magic_skill(id: &str) -> bool {
    matches!(
        id,
        "FE02"
            | "FE03"
            | "FE04"
            | "FE06"
            | "FR01"
            | "FR02"
            | "FR03"
            | "FR04"
            | "FR05"
            | "FR06"
            | "HY05"
            | "M02"
            | "OC04"
            | "OC05"
    )
}
fn frozen() -> StatusId {
    "agent:frozen".parse().unwrap()
}
fn burn() -> StatusId {
    "core:burning".parse().unwrap()
}
fn direction(from: GridPos, to: GridPos) -> (i32, i32) {
    let (x, y) = (to.x - from.x, to.y - from.y);
    if x.abs() >= y.abs() {
        (if x < 0 { -1 } else { 1 }, 0)
    } else {
        (0, if y < 0 { -1 } else { 1 })
    }
}
fn damage_key(id: &str) -> &'static str {
    match id {
        "FE02" | "FE03" => "raw_fire_damage_per_target_per_turn",
        "FE04" => "raw_center_fire_damage",
        "FR01" => "raw_cold_damage",
        "FR02" | "FR04" => "raw_cold_damage_per_target",
        "FR05" => "raw_cold_damage_per_impact",
        "FR06" => "raw_cold_damage_on_frozen_source",
        "OC05" => "raw_occult_damage_per_resolution",
        "HY05" | "M02" => "raw_occult_damage",
        _ => "",
    }
}
impl GameState {
    pub(super) fn agent_magic_base_damage(&self, id: &str, rank: u8) -> (u16, u64, u64) {
        let s = agent::definition(id).unwrap();
        let key = damage_key(id);
        if key.is_empty() {
            return (0, 1, 1);
        }
        let p = &s.parameters[key];
        if p["kind"] == "scaled_linear_damage" {
            let base = p["base"].as_u64().unwrap_or(0)
                + u64::from(rank.saturating_sub(1)) * p["gain_per_rank"].as_u64().unwrap_or(0);
            (
                base.min(u64::from(u16::MAX)) as u16,
                p["numerator"].as_u64().unwrap_or(1),
                p["denominator"].as_u64().unwrap_or(1).max(1),
            )
        } else {
            (s.value(key, rank), 1, 1)
        }
    }
    pub(super) fn agent_magic_range(&self, id: &str, rank: u8) -> u16 {
        let s = agent::definition(id).unwrap();
        let key = match id {
            "FE02" => "cone_range_cells",
            "FE03" => "placement_range_cells",
            "FR04" => "maximum_wave_travel_cells",
            "OC04" => "center_range_cells",
            "FR02" | "FR06" => return 0,
            _ => "range_cells",
        };
        s.value(key, rank)
    }
    pub(super) fn agent_magic_energy_key(id: &str) -> &'static str {
        match id {
            "FE02" => "energy_per_turn",
            "OC05" => "energy_per_resolution",
            _ => "energy",
        }
    }
    pub fn agent_magic_movement_blocked(&self, entity: EntityId) -> bool {
        if self.agent_device_zone_freezes(entity) {
            return true;
        }
        self.actors
            .get(entity)
            .is_some_and(|a| a.status(&frozen()).is_some())
    }
    pub(super) fn remove_agent_freeze(&mut self, target: EntityId) {
        self.agent_runtime_mut().magic.frozen.remove(&target);
        if self
            .actors
            .get_mut(target)
            .is_some_and(|a| a.remove_status(&frozen()))
        {
            self.events.push(GameEvent::StatusRemoved {
                target,
                status: frozen(),
                reason: super::super::StatusRemovalReason::Consumed,
            });
        }
    }
    /// R03-GEL02 + RESIST-LOT03: Cold resistance, then a single importance penalty.
    pub fn agent_freeze_chance(&self, id: &str, rank: u8, target: EntityId, exposure: u16) -> u16 {
        let Some(a) = self.actors.get(target) else {
            return 0;
        };
        let Some(s) = agent::definition(id) else {
            return 0;
        };
        let key = "freeze_skill_chance_factor_percent";
        let Some(_) = s.parameters.get(key) else {
            return 0;
        };
        let resistance = a.resistances().get(DamageType::Cold).clamp(-50, 75);
        let chance = (u64::from(s.value(key, rank))
            * u64::from(exposure.max(1))
            * u64::try_from(100 - i32::from(resistance)).unwrap())
        .min(10_000);
        let boss = a
            .tags()
            .iter()
            .any(|t| matches!(t.as_str(), "core:boss" | "agent:boss"));
        let elite = a
            .tags()
            .iter()
            .any(|t| matches!(t.as_str(), "core:elite" | "agent:elite"));
        let (n, d) = if boss {
            (1, 2)
        } else if elite {
            (3, 4)
        } else {
            (1, 1)
        };
        (chance * n / d / 100) as u16
    }
    pub(super) fn apply_agent_freeze(
        &mut self,
        id: &str,
        rank: u8,
        target: EntityId,
        exposure: u16,
    ) {
        let chance = self.agent_freeze_chance(id, rank, target, exposure);
        if chance == 0 || (chance < 100 && u16::from(self.rng.percentile()) > chance) {
            return;
        }
        let duration = agent::definition(id)
            .unwrap()
            .value("freeze_duration_turns", rank);
        if self.rules.statuses.get(&frozen()).is_none() {
            return;
        }
        let before = self.events.len();
        self.apply_status_to(
            Some(self.player),
            target,
            &ApplyStatusEffect::new(frozen(), 1).unwrap(),
        )
        .expect("registered Gel");
        let mut applied = false;
        for event in &mut self.events[before..] {
            if let GameEvent::StatusApplied {
                target: recipient,
                application,
                remaining_turns,
                ..
            } = event
            {
                if *recipient == target && *application != crate::status::StatusApplyKind::Ignored {
                    *remaining_turns = Some(duration);
                    applied = true;
                }
            }
        }
        if !applied {
            return;
        }
        let turn = self.turn;
        if let Some(a) = self.actors.get_mut(target) {
            a.set_status_duration(&frozen(), duration);
            if a.remove_status(&burn()) {
                self.events.push(GameEvent::StatusRemoved {
                    target,
                    status: burn(),
                    reason: super::super::StatusRemovalReason::Consumed,
                });
            }
        }
        self.agent_runtime_mut().magic.frozen.insert(
            target,
            AgentFrozen {
                expires_at: turn.saturating_add(u64::from(duration)),
            },
        );
    }
    fn agent_magic_hostile(&self, entity: EntityId) -> bool {
        entity != self.player
            && !self.is_player_controlled_companion(entity)
            && self
                .actors
                .get(entity)
                .is_some_and(|a| a.player_relation() == crate::social::PlayerRelation::Hostile)
    }
    pub(super) fn agent_magic_preview(
        &self,
        id: &str,
        selected: GridPos,
    ) -> Result<AttackPreview, CommandRejection> {
        let profile = self.agent_skill_ready(id)?;
        let rank = self.player_agent_skill_rank(id);
        let s = agent::definition(id).unwrap();
        let origin = self
            .player_position()
            .ok_or(CommandRejection::AgentProgressionUnavailable)?;
        let target = if matches!(id, "FR02" | "FR06") {
            origin
        } else {
            selected
        };
        if !self.map.contains(target) {
            return Err(CommandRejection::AbilityTargetOutsideMap(target));
        }
        if !spatial::in_cast_range(origin, target, profile.range) {
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
        if !self.map.is_walkable(target) {
            return Err(CommandRejection::AbilityTargetBlocked(target));
        }
        let dir = direction(origin, target);
        let area = match id {
            "FE02" => Area::Cone {
                length: s.value("cone_range_cells", rank),
            },
            "FE03" | "FR03" => Area::Wall {
                length: s.value("maximum_wall_length_cells", rank),
            },
            "FE04" => Area::Square {
                radius: s.value("radius_cells", rank),
            },
            "FR02" | "FR05" => Area::Disc {
                radius: s.value("radius_cells", rank),
            },
            "FR04" => Area::Wall {
                length: s.value("wave_width_cells", rank),
            },
            "OC04" => Area::Ring {
                radius: s.value("ritual_radius_cells", rank),
            },
            _ => Area::Point,
        };
        let centre = match id {
            "FE02" => origin,
            "FR04" => GridPos::new(origin.x + dir.0, origin.y + dir.1),
            _ => target,
        };
        let mut cells = spatial::affected_cells(&self.map, centre, area, dir);
        if id == "FR06" {
            let sources = self.agent_frozen_sources();
            if sources.is_empty() {
                return Err(CommandRejection::AbilityTargetHasNoActor(origin));
            }
            cells.clear();
            for (_, at) in sources {
                cells.extend(spatial::affected_cells(
                    &self.map,
                    at,
                    Area::Diamond {
                        radius: s.value("radius_cells", rank),
                    },
                    (1, 0),
                ));
            }
            cells.sort_by_key(|p| (p.x, p.y));
            cells.dedup();
        }
        if matches!(id, "FR01" | "HY05" | "M02" | "OC05" | "FE06") {
            let victim = self
                .actors
                .entity_at(target)
                .ok_or(CommandRejection::AbilityTargetHasNoActor(target))?;
            if !self.agent_magic_hostile(victim) {
                return Err(
                    CommandRejection::TechniqueTargetDoesNotMeetEngagementRequirement(victim),
                );
            }
            if id == "FE06"
                && !self
                    .actors
                    .get(victim)
                    .and_then(|a| a.status(&burn()))
                    .is_some_and(|b| b.source == Some(self.player) && b.power.is_some())
            {
                return Err(
                    CommandRejection::TechniqueTargetDoesNotMeetEngagementRequirement(victim),
                );
            }
        }
        if matches!(id, "FR03" | "OC04") {
            if cells.is_empty() {
                return Err(CommandRejection::AbilityTargetBlocked(target));
            }
            for &at in &cells {
                if !self.player_visibility.is_visible(at) || self.map.is_protected(at) {
                    return Err(CommandRejection::AbilityNoLineOfSight(at));
                }
                if self.actors.entity_at(at).is_some() {
                    return Err(CommandRejection::AbilityTargetBlocked(at));
                }
            }
        }
        cells.retain(|p| self.player_visibility.is_visible(*p) && !self.map.is_protected(*p));
        Ok(AttackPreview::new(
            origin,
            centre,
            cells
                .into_iter()
                .map(|position| AttackAreaCell {
                    position,
                    step: grid_distance(centre, position),
                })
                .collect(),
        ))
    }
    fn agent_frozen_sources(&self) -> Vec<(EntityId, GridPos)> {
        self.actors
            .iter()
            .filter_map(|(e, a)| {
                (self.agent_magic_hostile(e)
                    && a.status(&frozen()).is_some()
                    && self.player_visibility.is_visible(a.position()))
                .then_some((e, a.position()))
            })
            .collect()
    }
    fn agent_magic_effect(&self, id: &str, rank: u8, preview: &AttackPreview) -> AgentMagicEffect {
        let profile = self.agent_spell_profile(id, rank).unwrap();
        let plain = self.agent_magic_plain_multiplier();
        AgentMagicEffect {
            skill: id.into(),
            rank,
            origin: preview.origin(),
            target: preview.target(),
            direction: direction(preview.origin(), preview.target()),
            cells: preview.cells().iter().map(|c| c.position).collect(),
            remaining: 1,
            next_turn: self.turn,
            stage: 0,
            base_damage: profile.base_damage,
            numerator: profile.multiplier_numerator,
            denominator: profile.multiplier_denominator,
            status_numerator: plain.0,
            status_denominator: plain.1,
            energy: profile.energy,
            victim: self.actors.entity_at(preview.target()),
            exposure: BTreeMap::new(),
            last_hit: BTreeMap::new(),
        }
    }
    pub(super) fn cast_agent_magic(
        &mut self,
        id: &str,
        target: GridPos,
    ) -> Result<(), CommandRejection> {
        let preview = self.agent_magic_preview(id, target)?;
        let rank = self.player_agent_skill_rank(id);
        let s = agent::definition(id).unwrap();
        let mut effect = self.agent_magic_effect(id, rank, &preview);
        self.player_energy
            .spend(effect.energy)
            .expect("validated spell energy");
        self.cancel_agent_magic_channel();
        self.events.push(GameEvent::TechniqueUsed {
            entity: self.player,
            technique: agent::technique(id),
            observed_on_turn: self.turn,
        });
        match id {
            "FR03" | "OC04" => {
                if id == "FR03" {
                    self.remove_agent_magic_barriers("FR03");
                }
                let hp_key = if id == "FR03" {
                    "raw_HP_per_block"
                } else {
                    "raw_HP_per_ritual_segment"
                };
                let hp = s.value(hp_key, rank).max(1);
                let expiry = self.turn.saturating_add(u64::from(s.value(
                    if id == "FR03" {
                        "wall_duration_turns"
                    } else {
                        "duration_turns"
                    },
                    rank,
                )));
                for cell in &effect.cells {
                    let actor = Actor::new(*cell, hp).unwrap().with_tags([
                        format!("agent:{}", id.to_ascii_lowercase())
                            .parse()
                            .unwrap(),
                        "agent:spell_barrier".parse().unwrap(),
                    ]);
                    let entity = self
                        .spawn_actor(actor)
                        .expect("validated free spell barrier cell");
                    self.agent_runtime_mut().magic.barriers.insert(
                        entity,
                        AgentMagicBarrier {
                            centre: preview.target(),
                            skill: id.into(),
                            rank,
                            expires_at: expiry,
                            last_contact: BTreeMap::new(),
                        },
                    );
                }
            }
            "FE06" => self.resolve_agent_combustion(&effect),
            "FR06" => self.resolve_agent_crystalline(&effect),
            "FE02" | "OC05" => {
                effect.remaining = s.value(
                    if id == "FE02" {
                        "maximum_duration_turns"
                    } else {
                        "maximum_resolutions"
                    },
                    rank,
                );
                self.resolve_agent_magic_effect(&mut effect, false, None);
                effect.remaining = effect.remaining.saturating_sub(1);
                effect.next_turn = self.turn.saturating_add(1);
                if id == "FE02" {
                    effect.numerator = plain_multiplier_for_effect(self, &effect).0;
                    effect.denominator = plain_multiplier_for_effect(self, &effect).1;
                }
                if effect.remaining > 0 {
                    self.agent_runtime_mut().magic.effects.push(effect);
                }
            }
            "FE03" | "FE04" | "FR04" | "FR05" => {
                if id == "FE03" {
                    self.agent_runtime_mut()
                        .magic
                        .effects
                        .retain(|e| e.skill != "FE03");
                }
                effect.remaining = s
                    .value(
                        match id {
                            "FE03" | "FR05" => "duration_turns",
                            "FR04" => "maximum_wave_travel_cells",
                            _ => "preparation_delay_complete_turns",
                        },
                        rank,
                    )
                    .max(1);
                if matches!(id, "FE03" | "FR04") {
                    self.resolve_agent_magic_effect(&mut effect, false, None);
                    effect.remaining = effect.remaining.saturating_sub(1);
                }
                // FE04 and FR05 retain a complete warning turn before the first impact.
                effect.next_turn = self.turn.saturating_add(1);
                if matches!(id, "FE03" | "FR05") {
                    let (n, d) = plain_multiplier_for_effect(self, &effect);
                    effect.numerator = n;
                    effect.denominator = d;
                }
                if effect.remaining > 0 {
                    self.agent_runtime_mut().magic.effects.push(effect);
                }
            }
            _ => self.resolve_agent_magic_effect(&mut effect, false, None),
        }
        self.end_player_active_camouflage();
        Ok(())
    }
    pub(super) fn cancel_agent_magic_channel(&mut self) {
        self.agent_runtime_mut()
            .magic
            .effects
            .retain(|e| !matches!(e.skill.as_str(), "FE02" | "OC05"));
    }
    pub(crate) fn process_agent_magic_turn(&mut self) {
        let turn = self.turn;
        let expired: Vec<_> = self
            .agent_runtime()
            .magic
            .barriers
            .iter()
            .filter_map(|(entity, b)| {
                (turn.saturating_add(1) >= b.expires_at || self.actors.get(*entity).is_none())
                    .then_some(*entity)
            })
            .collect();
        for entity in expired {
            self.actors.remove(entity);
            self.agent_runtime_mut().magic.barriers.remove(&entity);
        }
        let invalid: Vec<_> = self
            .agent_runtime()
            .magic
            .frozen
            .keys()
            .filter(|e| {
                !self
                    .actors
                    .get(**e)
                    .is_some_and(|a| a.status(&frozen()).is_some())
            })
            .copied()
            .collect();
        for e in invalid {
            self.agent_runtime_mut().magic.frozen.remove(&e);
        }
        let mut effects = std::mem::take(&mut self.agent_runtime_mut().magic.effects);
        for mut e in effects.drain(..) {
            if e.next_turn > turn {
                self.agent_runtime_mut().magic.effects.push(e);
                continue;
            }
            if matches!(e.skill.as_str(), "FE02" | "OC05") {
                if self.player_position() != Some(e.origin)
                    || self.player_energy.available() < e.energy
                {
                    continue;
                }
                if e.skill == "OC05" {
                    let Some(target) = e
                        .victim
                        .and_then(|v| self.actors.get(v))
                        .map(Actor::position)
                    else {
                        continue;
                    };
                    if !self.player_visibility.is_visible(target)
                        || !has_line_of_sight(&self.map, e.origin, target, true)
                        || !spatial::in_cast_range(
                            e.origin,
                            target,
                            self.agent_magic_range("OC05", e.rank),
                        )
                    {
                        continue;
                    }
                    e.target = target;
                    e.cells = vec![target];
                }
                self.player_energy
                    .spend(e.energy)
                    .expect("checked continuation energy");
            }
            if e.skill == "FR04" {
                e.stage = e.stage.saturating_add(1);
                // Keep only lanes that survived every previous wall collision.
                e.cells = e
                    .cells
                    .iter()
                    .map(|at| GridPos::new(at.x + e.direction.0, at.y + e.direction.1))
                    .filter(|at| self.map.is_walkable(*at))
                    .collect();
                if e.cells.is_empty() {
                    continue;
                }
                e.target = GridPos::new(e.target.x + e.direction.0, e.target.y + e.direction.1);
            }
            self.resolve_agent_magic_effect(&mut e, true, None);
            e.remaining = e.remaining.saturating_sub(1);
            e.next_turn = turn.saturating_add(1);
            if e.remaining > 0 {
                self.agent_runtime_mut().magic.effects.push(e);
            }
        }
    }
    fn resolve_agent_magic_effect(
        &mut self,
        e: &mut AgentMagicEffect,
        automatic: bool,
        only: Option<EntityId>,
    ) {
        let id = e.skill.as_str();
        let s = agent::definition(id).unwrap();
        let mut cells = e.cells.clone();
        if id == "FR05" {
            // FR05-HITS02: every admissible cell receives its own roll each
            // turn, including empty cells. A new obstacle/protected cell must
            // not consume an impact roll or keep exposure alive behind it.
            cells.retain(|at| {
                self.map.is_walkable(*at)
                    && !self.map.is_protected(*at)
                    && has_line_of_sight(&self.map, e.target, *at, true)
            });
            e.stage = e.stage.saturating_add(1);
            let occupants: Vec<_> = self
                .actors
                .iter()
                .filter_map(|(a, b)| cells.contains(&b.position()).then_some(a))
                .collect();
            e.exposure.retain(|a, _| occupants.contains(a));
            for a in occupants {
                let n = e.exposure.entry(a).or_default();
                *n = n.saturating_add(1);
            }
            let chance = s.value("impact_chance_per_cell_percent", e.rank).min(100);
            cells.retain(|_| u16::from(self.rng.percentile()) <= chance);
        }
        let recipients: Vec<_> = self
            .actors
            .iter()
            .filter_map(|(victim, a)| {
                let eligible = if matches!(id, "FR01" | "FR02" | "FR04" | "HY05" | "M02" | "OC05") {
                    self.agent_magic_hostile(victim)
                } else {
                    victim != self.player
                        && !self.agent_runtime().magic.barriers.contains_key(&victim)
                };
                (eligible && only.is_none_or(|v| v == victim) && cells.contains(&a.position()))
                    .then_some((victim, a.position()))
            })
            .collect();
        if id != "FR05" {
            e.exposure.retain(|a, _| {
                self.actors
                    .get(*a)
                    .is_some_and(|actor| e.cells.contains(&actor.position()))
            });
        }
        let area = match id {
            "FE04" => Area::Square {
                radius: s.value("radius_cells", e.rank),
            },
            "FR02" | "FR05" => Area::Disc {
                radius: s.value("radius_cells", e.rank),
            },
            _ => Area::Point,
        };
        let radius = match area {
            Area::Square { radius } | Area::Disc { radius } | Area::Diamond { radius } => radius,
            _ => 0,
        };
        self.events.push(GameEvent::AgentAreaSpellCast {
            entity: self.player,
            skill: e.skill.clone(),
            origin: e.origin,
            target: e.target,
            radius,
        });
        for (victim, at) in recipients {
            if e.last_hit.get(&victim) == Some(&self.turn) {
                continue;
            }
            e.last_hit.insert(victim, self.turn);
            let exposure = if id == "FR05" {
                *e.exposure.get(&victim).unwrap_or(&1)
            } else {
                let value = e.exposure.entry(victim).or_default();
                *value = value.saturating_add(1);
                *value
            };
            let critical = !matches!(id, "FE03" | "FE02" | "FR05") && self.rng.percentile() <= 5;
            self.events.push(GameEvent::AgentSpellCast {
                entity: self.player,
                victim,
                skill: e.skill.clone(),
                origin: e.origin,
                target: at,
                critical,
            });
            let (n, d) = if radius > 0 {
                crate::skills::area_damage::retention(e.target, at, area).unwrap_or((1, 1))
            } else {
                (1, 1)
            };
            let hp_before = self.actors.get(victim).map_or(0, Actor::integrity);
            let damage_type = if id.starts_with("FR") {
                DamageType::Cold
            } else if id.starts_with("FE") {
                DamageType::Thermal
            } else {
                DamageType::Corruption
            };
            let _ = self.apply_damage_impact_to_scaled_spell(
                Some(self.player),
                victim,
                DamagePacket::new(e.base_damage, damage_type, 0),
                u128::from(e.numerator) * u128::from(n) * if critical { 2 } else { 1 },
                u128::from(e.denominator) * u128::from(d),
            );
            let removed =
                hp_before.saturating_sub(self.actors.get(victim).map_or(0, Actor::integrity));
            if id == "M02" {
                self.extend_agent_transformation(e.rank, removed);
            }
            if id == "HY05" {
                let pct = s.value("energy_return_percent_of_actual_HP_removed", e.rank);
                self.player_energy.restore(
                    (u32::from(removed) * u32::from(pct) / 100).min(u32::from(u16::MAX)) as u16,
                );
            }
            if id == "OC05" {
                let pct = s.value("life_return_percent_of_actual_HP_removed", e.rank);
                let restored = self.actors.get_mut(self.player).map_or(0, |a| {
                    a.restore_integrity(
                        ((u32::from(removed) * u32::from(pct)).div_ceil(100))
                            .min(u32::from(removed)) as u16,
                    )
                });
                if restored > 0 {
                    self.events.push(GameEvent::IntegrityRestored {
                        entity: self.player,
                        amount: restored,
                    });
                }
            }
            if self.actors.get(victim).is_none() {
                continue;
            }
            if id.starts_with("FR") {
                self.apply_agent_freeze(id, e.rank, victim, exposure);
            }
            if id.starts_with("FE") && s.parameters.contains_key("raw_burn_damage_per_turn") {
                let chance = s
                    .value("burn_chance_percent", e.rank)
                    .saturating_mul(if matches!(id, "FE02" | "FE03") {
                        exposure
                    } else {
                        1
                    })
                    .min(100);
                if chance == 100 || u16::from(self.rng.percentile()) <= chance {
                    let power = StatusPower {
                        base_damage: s.value("raw_burn_damage_per_turn", e.rank),
                        numerator: e.status_numerator,
                        denominator: e.status_denominator,
                        applied_on_turn: self.turn,
                    };
                    let duration = s.value("burn_duration_turns", e.rank);
                    // Automatic zone refresh must not create a second immediate periodic hit.
                    self.apply_agent_burn_mode(
                        victim,
                        power,
                        duration,
                        !automatic || !matches!(id, "FE02" | "FE03"),
                    );
                }
            }
        }
    }
    fn resolve_agent_combustion(&mut self, e: &AgentMagicEffect) {
        let Some(victim) = e.victim else { return };
        let Some(old) = self
            .actors
            .get(victim)
            .and_then(|a| a.status(&burn()))
            .cloned()
        else {
            return;
        };
        let Some(power) = old.power else { return };
        let s = agent::definition("FE06").unwrap();
        self.actors.get_mut(victim).unwrap().remove_status(&burn());
        self.events.push(GameEvent::StatusRemoved {
            target: victim,
            status: burn(),
            reason: super::super::StatusRemovalReason::Consumed,
        });
        let critical = self.rng.percentile() <= 5;
        self.events.push(GameEvent::AgentSpellCast {
            entity: self.player,
            victim,
            skill: "FE06".into(),
            origin: e.origin,
            target: e.target,
            critical,
        });
        // The stored burn already includes its original spell power. Only the
        // direct ambush portion is added here; periodic damage never receives it.
        let ambush = (u128::from(self.player_progression.level().saturating_sub(1)) + 10)
            * u128::from(self.agent_stealth_damage_bonus())
            * u128::from(power.denominator)
            / 1000;
        let n = (u128::from(power.numerator) + ambush)
            * u128::from(old.remaining_turns.unwrap_or(0))
            * u128::from(s.value("remaining_burn_conversion_percent", e.rank))
            * if critical { 2 } else { 1 };
        let _ = self.apply_damage_impact_to_scaled_spell(
            Some(self.player),
            victim,
            DamagePacket::new(power.base_damage, DamageType::Thermal, 0),
            n,
            u128::from(power.denominator) * 100,
        );
        if self.actors.get(victim).is_some()
            && u16::from(self.rng.percentile()) <= s.value("fresh_burn_chance_percent", e.rank)
        {
            self.apply_agent_burn(
                victim,
                StatusPower {
                    base_damage: s.value("raw_fresh_burn_damage_per_turn", e.rank),
                    numerator: e.status_numerator,
                    denominator: e.status_denominator,
                    applied_on_turn: self.turn,
                },
                s.value("fresh_burn_duration_turns", e.rank),
            );
        }
    }
    fn resolve_agent_crystalline(&mut self, e: &AgentMagicEffect) {
        let sources = self.agent_frozen_sources();
        let radius = agent::definition("FR06")
            .unwrap()
            .value("radius_cells", e.rank);
        for (source, centre) in sources {
            let area = Area::Diamond { radius };
            let cells = spatial::affected_cells(&self.map, centre, area, (1, 0));
            let recipients: Vec<_> = self
                .actors
                .iter()
                .filter_map(|(v, a)| {
                    (self.agent_magic_hostile(v) && cells.contains(&a.position()))
                        .then_some((v, a.position()))
                })
                .collect();
            self.events.push(GameEvent::AgentAreaSpellCast {
                entity: self.player,
                skill: "FR06".into(),
                origin: e.origin,
                target: centre,
                radius,
            });
            for (victim, at) in recipients {
                let critical = self.rng.percentile() <= 5;
                self.events.push(GameEvent::AgentSpellCast {
                    entity: self.player,
                    victim,
                    skill: "FR06".into(),
                    origin: centre,
                    target: at,
                    critical,
                });
                let (n, d) = crate::skills::area_damage::retention(centre, at, area).unwrap();
                let _ = self.apply_damage_impact_to_scaled_spell(
                    Some(self.player),
                    victim,
                    DamagePacket::new(e.base_damage, DamageType::Cold, 0),
                    u128::from(e.numerator) * u128::from(n) * if critical { 2 } else { 1 },
                    u128::from(e.denominator) * u128::from(d),
                );
            }
            self.remove_agent_freeze(source);
        }
    }
    fn remove_agent_magic_barriers(&mut self, id: &str) {
        let entities: Vec<_> = self
            .agent_runtime()
            .magic
            .barriers
            .iter()
            .filter_map(|(e, b)| (b.skill == id).then_some(*e))
            .collect();
        for e in entities {
            self.actors.remove(e);
            self.agent_runtime_mut().magic.barriers.remove(&e);
        }
    }
    pub(super) fn agent_magic_barrier_melee_contact(
        &mut self,
        attacker: EntityId,
        barrier: EntityId,
    ) {
        let Some(b) = self.agent_runtime().magic.barriers.get(&barrier) else {
            return;
        };
        if b.skill != "FR03"
            || !self.agent_magic_hostile(attacker)
            || self.agent_runtime().magic.barriers.values().any(|other| {
                other.skill == "FR03" && other.last_contact.get(&attacker) == Some(&self.turn)
            })
        {
            return;
        }
        let rank = b.rank;
        let turn = self.turn;
        self.agent_runtime_mut()
            .magic
            .barriers
            .get_mut(&barrier)
            .unwrap()
            .last_contact
            .insert(attacker, turn);
        self.apply_agent_freeze("FR03", rank, attacker, 1);
    }
    pub fn agent_magic_barrier_at(&self, at: GridPos) -> Option<EntityId> {
        self.actors
            .entity_at(at)
            .filter(|e| self.agent_runtime().magic.barriers.contains_key(e))
    }
}

#[cfg(test)]
#[path = "agent_magic_tests.rs"]
mod tests;

/// Presentation-only copy of authoritative effects. Never advances their turns.
#[derive(Clone, Debug)]
pub struct AgentMagicVisualEffect {
    pub skill: String,
    pub rank: u8,
    pub origin: GridPos,
    pub target: GridPos,
    pub direction: (i32, i32),
    pub cells: Vec<GridPos>,
    pub stage: u16,
}
impl GameState {
    pub fn agent_magic_visual_effects(&self) -> Vec<AgentMagicVisualEffect> {
        self.agent_runtime()
            .magic
            .effects
            .iter()
            .map(|e| AgentMagicVisualEffect {
                skill: e.skill.clone(),
                rank: e.rank,
                origin: e.origin,
                target: e.target,
                direction: e.direction,
                cells: e.cells.clone(),
                stage: e.stage,
            })
            .collect()
    }
    pub fn agent_magic_visual_barriers(&self) -> Vec<(EntityId, String, GridPos, u8)> {
        self.agent_runtime()
            .magic
            .barriers
            .iter()
            .filter(|(e, _)| self.actors.get(**e).is_some())
            .map(|(e, b)| (*e, b.skill.clone(), b.centre, b.rank))
            .collect()
    }
}

impl GameState {
    fn agent_magic_plain_multiplier(&self) -> (u64, u64) {
        let will = self
            .player_effective_primary_attributes()
            .map_or(5, |a| a.value(PrimaryAttribute::Willpower));
        let pool = u64::from(will.saturating_sub(5))
            + u64::from(
                self.rules
                    .agent_class_start
                    .as_ref()
                    .map_or(100, |s| s.spell_damage_percent),
            )
            + u64::from(self.agent_transform_bonus_percent());
        (
            (u64::from(self.player_progression.level().saturating_sub(1)) + 10) * pool,
            1000,
        )
    }
}
fn plain_multiplier_for_effect(game: &GameState, e: &AgentMagicEffect) -> (u64, u64) {
    let (_, n, d) = game.agent_magic_base_damage(&e.skill, e.rank);
    (e.status_numerator * n, e.status_denominator * d)
}

impl GameState {
    pub fn agent_regeneration_visual_active(&self) -> bool {
        self.agent_runtime().support.regeneration.is_some()
    }
}

impl GameState {
    pub(super) fn agent_magic_actor_moved(&mut self, entity: EntityId) {
        let Some(position) = self.actors.get(entity).map(Actor::position) else {
            return;
        };
        let mut effects = std::mem::take(&mut self.agent_runtime_mut().magic.effects);
        for effect in &mut effects {
            if matches!(effect.skill.as_str(), "FE02" | "FE03" | "FR05")
                && !effect.cells.contains(&position)
            {
                effect.exposure.remove(&entity);
            }
            if effect.skill == "FE03" && effect.cells.contains(&position) {
                self.resolve_agent_magic_effect(effect, true, Some(entity));
            }
        }
        self.agent_runtime_mut().magic.effects.extend(effects);
    }
}
