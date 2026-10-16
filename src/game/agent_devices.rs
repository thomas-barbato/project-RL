//! Reviewed grenades, armed mines and full-footprint persistent residues.
use super::*;
use crate::combat::DamageType;
use crate::skills::{
    agent,
    spatial::{self, Area},
};
use crate::status::StatusPower;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentMineVariant {
    #[default]
    Fire,
    Poison,
    Cold,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentGrenadeVariant {
    #[default]
    Explosive,
    Blinding,
    Decoy,
}
impl AgentMineVariant {
    pub fn skill(self) -> &'static str {
        match self {
            Self::Fire => "SM00",
            Self::Poison => "SM02",
            Self::Cold => "SM03",
        }
    }
    fn damage_type(self) -> DamageType {
        match self {
            Self::Fire => DamageType::Thermal,
            Self::Poison => DamageType::Chemical,
            Self::Cold => DamageType::Cold,
        }
    }
}
impl AgentGrenadeVariant {
    pub fn skill(self) -> &'static str {
        match self {
            Self::Explosive => "SG00",
            Self::Blinding => "SG01",
            Self::Decoy => "SG02",
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentMine {
    pub serial: u64,
    pub at: GridPos,
    pub variant: AgentMineVariant,
    pub armed_at: u64,
    pub radius: u16,
    pub power: StatusPower,
    pub poison: Option<(StatusPower, u16)>,
    pub freeze_rank: u8,
    pub residue_duration: u16,
    pub residue_power: StatusPower,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentMineResidue {
    pub serial: u64,
    pub center: GridPos,
    pub cells: Vec<GridPos>,
    pub variant: AgentMineVariant,
    pub remaining: u16,
    pub next_turn: u64,
    pub power: StatusPower,
    pub freeze_rank: u8,
    pub exposures: BTreeMap<EntityId, u16>,
    pub frozen_targets: BTreeSet<EntityId>,
    pub last_hit: BTreeMap<EntityId, u64>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AgentDecoy {
    pub at: GridPos,
    pub intensity: u16,
    pub expires: u64,
    pub attempted: BTreeSet<EntityId>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct AgentDevicesRuntime {
    pub mine_variant: AgentMineVariant,
    pub grenade_variant: AgentGrenadeVariant,
    pub serial: u64,
    pub mines: Vec<AgentMine>,
    pub residues: Vec<AgentMineResidue>,
    pub decoys: Vec<AgentDecoy>,
    pub lured: BTreeMap<EntityId, GridPos>,
}
fn zone_freeze() -> StatusId {
    "agent:mine_frozen".parse().unwrap()
}
fn blinded() -> StatusId {
    "agent:blind".parse().unwrap()
}
pub(crate) fn install_statuses(catalog: &mut crate::status::StatusCatalog) {
    for (id, duration) in [(zone_freeze(), 2), (blinded(), 2)] {
        if catalog.get(&id).is_none() {
            catalog
                .register(
                    StatusDefinition::new(
                        id,
                        Some(duration),
                        crate::status::StatusStacking::RefreshDuration,
                        vec![],
                    )
                    .unwrap(),
                )
                .unwrap();
        }
    }
}
impl GameState {
    pub fn player_agent_mine_variant(&self) -> AgentMineVariant {
        let variant = self.agent_runtime().devices.mine_variant;
        if variant == AgentMineVariant::Fire || self.player_agent_skill_rank(variant.skill()) > 0 {
            variant
        } else {
            AgentMineVariant::Fire
        }
    }
    pub fn player_agent_grenade_variant(&self) -> AgentGrenadeVariant {
        let variant = self.agent_runtime().devices.grenade_variant;
        if variant == AgentGrenadeVariant::Explosive
            || self.player_agent_skill_rank(variant.skill()) > 0
        {
            variant
        } else {
            AgentGrenadeVariant::Explosive
        }
    }
    pub fn set_player_agent_mine_variant(
        &mut self,
        variant: AgentMineVariant,
    ) -> Result<(), CommandRejection> {
        if self.player_agent_skill_rank("SM00") == 0
            || (variant != AgentMineVariant::Fire
                && self.player_agent_skill_rank(variant.skill()) == 0)
        {
            return Err(CommandRejection::TechniqueNotLearned(agent::technique(
                variant.skill(),
            )));
        }
        self.agent_runtime_mut().devices.mine_variant = variant;
        Ok(())
    }
    pub fn set_player_agent_grenade_variant(
        &mut self,
        variant: AgentGrenadeVariant,
    ) -> Result<(), CommandRejection> {
        if self.player_agent_skill_rank("SG00") == 0
            || (variant != AgentGrenadeVariant::Explosive
                && self.player_agent_skill_rank(variant.skill()) == 0)
        {
            return Err(CommandRejection::TechniqueNotLearned(agent::technique(
                variant.skill(),
            )));
        }
        self.agent_runtime_mut().devices.grenade_variant = variant;
        Ok(())
    }
    pub fn player_agent_mines(&self) -> &[AgentMine] {
        &self.agent_runtime().devices.mines
    }
    pub fn player_agent_mine_residues(&self) -> &[AgentMineResidue] {
        &self.agent_runtime().devices.residues
    }
    fn agent_device_bonus(&self, id: &str, key: &str) -> u16 {
        let rank = self.player_agent_skill_rank(id);
        if rank == 0 {
            0
        } else {
            agent::definition(id).unwrap().value(key, rank)
        }
    }
    pub(super) fn agent_device_ready(
        &self,
        id: &str,
    ) -> Result<AgentSkillUseProfile, CommandRejection> {
        if !matches!(id, "SM00" | "SG00") {
            return Err(CommandRejection::TechniqueHasNoActiveAction(
                agent::technique(id),
            ));
        }
        if self.player_agent_skill_rank(id) == 0 {
            return Err(CommandRejection::TechniqueNotLearned(agent::technique(id)));
        }
        let start = self
            .rules
            .agent_class_start
            .as_ref()
            .ok_or(CommandRejection::AgentProgressionUnavailable)?;
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
        let range = if id == "SG00" {
            skill
                .value("base_throw_range_cells", 1)
                .saturating_add(self.agent_device_bonus("SG03", "throw_range_bonus_cells"))
        } else {
            skill.value("placement_range_cells", 1)
        };
        let energy = (u32::from(skill.value("energy", 1))
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
    pub(super) fn agent_device_footprint(
        &self,
        id: &str,
        target: GridPos,
    ) -> Result<AttackPreview, CommandRejection> {
        let profile = self.agent_device_ready(id)?;
        let origin = self
            .player_position()
            .ok_or(CommandRejection::MissingPlayer)?;
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
        if !self.map.is_walkable(target) {
            return Err(CommandRejection::AbilityTargetBlocked(target));
        }
        if self.map.is_protected(origin) || self.map.is_protected(target) {
            return Err(CommandRejection::ProtectedZone);
        }
        if id == "SM00" {
            if self.actors.entity_at(target).is_some_and(|e| {
                e != self.player
                    && !self.is_player_controlled_companion(e)
                    && self.actors.get(e).is_some_and(|a| {
                        a.player_relation() == crate::social::PlayerRelation::Hostile
                    })
            }) || self
                .agent_runtime()
                .devices
                .mines
                .iter()
                .any(|m| m.at == target)
                || self.explosive_devices.at(target).next().is_some()
            {
                return Err(CommandRejection::AbilityTargetBlocked(target));
            }
        }
        let radius = if id == "SM00" {
            1 + self.agent_device_bonus("SM04", "added_explosion_radius_cells")
        } else {
            1
        };
        let affected =
            if id == "SG00" && self.player_agent_grenade_variant() == AgentGrenadeVariant::Decoy {
                let reach = self.agent_device_bonus("SG02", "open_terrain_sound_reach_cells");
                let rules = self.rules.stealth_rules.unwrap_or_default();
                rules
                    .sound_field_on_map(
                        &self.map,
                        1u16.saturating_add(reach.saturating_mul(rules.sound_attenuation_per_cell)),
                        target,
                    )
                    .collect()
            } else {
                spatial::affected_cells(&self.map, target, Area::Square { radius }, (1, 0))
            };
        let cells = affected
            .into_iter()
            .filter(|p| self.player_visibility.is_visible(*p) && !self.map.is_protected(*p))
            .map(|position| AttackAreaCell {
                position,
                step: grid_distance(target, position),
            })
            .collect();
        Ok(AttackPreview::new(origin, target, cells))
    }
    fn agent_device_power(&self, base: u16, bonus: u16) -> StatusPower {
        let intelligence = self
            .player_effective_primary_attributes()
            .map_or(5, |a| a.value(PrimaryAttribute::Processing));
        StatusPower {
            base_damage: base,
            numerator: (10 + u64::from(self.player_progression.level().saturating_sub(1)))
                * (100 + 2 * u64::from(intelligence.saturating_sub(5)) + u64::from(bonus)),
            denominator: 1000,
            applied_on_turn: self.turn,
        }
    }
    /// Card values reuse the same captured source calculation as actual devices.
    pub fn agent_device_parameter_value(&self, id: &str, key: &str, rank: u8) -> Option<u16> {
        let skill = agent::definition(id)?;
        if !matches!(id, "SG00" | "SM00" | "SM02" | "SM03" | "SM05") {
            return None;
        }
        if key == "energy" && matches!(id, "SG00" | "SM00") {
            let start = self.rules.agent_class_start.as_ref()?;
            return Some(
                (u32::from(skill.value(key, rank)) * u32::from(start.human_skill_energy_percent))
                    .div_ceil(100)
                    .clamp(1, 65535) as u16,
            );
        }
        if id == "SG00" && key == "base_throw_range_cells" {
            return Some(4 + self.agent_device_bonus("SG03", "throw_range_bonus_cells"));
        }
        if id == "SM00" && key == "radius_cells" {
            return Some(1 + self.agent_device_bonus("SM04", "added_explosion_radius_cells"));
        }
        if !key.starts_with("raw_") || !key.contains("damage") {
            return None;
        }
        let bonus = match id {
            "SG00" => self.agent_device_bonus("SG04", "grenade_damage_bonus_percent"),
            "SM02" | "SM03" => skill.value("variant_damage_bonus_percent", rank),
            "SM05" if key == "raw_poison_cloud_damage_per_turn" => {
                self.agent_device_bonus("SM02", "variant_damage_bonus_percent")
            }
            _ => 0,
        };
        let power = self.agent_device_power(skill.value(key, rank), bonus);
        Some(
            (u128::from(power.base_damage) * u128::from(power.numerator)
                / u128::from(power.denominator.max(1)))
            .min(65535) as u16,
        )
    }

    pub(super) fn cast_agent_device(
        &mut self,
        id: &str,
        target: GridPos,
    ) -> Result<(), CommandRejection> {
        let preview = self.agent_device_footprint(id, target)?;
        let cost = self.agent_device_ready(id)?.energy;
        self.player_energy
            .spend(cost)
            .expect("preflighted device cost");
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
        if id == "SM00" {
            self.place_agent_mine(target);
        } else {
            self.throw_agent_grenade(preview);
        }
        Ok(())
    }
    fn place_agent_mine(&mut self, at: GridPos) {
        let requested = self.player_agent_mine_variant();
        let variant = if requested == AgentMineVariant::Fire
            || self.player_agent_skill_rank(requested.skill()) > 0
        {
            requested
        } else {
            AgentMineVariant::Fire
        };
        let rank = self.player_agent_skill_rank(variant.skill()).max(1);
        let skill = agent::definition(variant.skill()).unwrap();
        let bonus = if variant == AgentMineVariant::Fire {
            0
        } else {
            skill.value("variant_damage_bonus_percent", rank)
        };
        let base = skill.value(
            match variant {
                AgentMineVariant::Fire => "raw_fire_explosion_damage",
                AgentMineVariant::Poison => "raw_poison_explosion_damage",
                AgentMineVariant::Cold => "raw_cold_explosion_damage",
            },
            rank,
        );
        let power = self.agent_device_power(base, bonus);
        let poison = (variant == AgentMineVariant::Poison).then(|| {
            (
                self.agent_device_power(skill.value("raw_poison_damage_per_turn", rank), bonus),
                skill.value("poison_duration_turns", rank),
            )
        });
        let radius = 1 + self.agent_device_bonus("SM04", "added_explosion_radius_cells");
        let duration = self.agent_device_bonus("SM05", "zone_duration_turns");
        let residual_base = if variant == AgentMineVariant::Cold {
            0
        } else {
            agent::definition("SM05").unwrap().value(
                if variant == AgentMineVariant::Fire {
                    "raw_fire_zone_damage_per_turn"
                } else {
                    "raw_poison_cloud_damage_per_turn"
                },
                self.player_agent_skill_rank("SM05").max(1),
            )
        };
        let residue_power = self.agent_device_power(residual_base, bonus);
        let capacity = (1 + self
            .agent_device_bonus("SM01", "maximum_simultaneous_mines")
            .saturating_sub(1))
        .max(1) as usize;
        let armed_at = self.turn.saturating_add(1);
        let runtime = &mut self.agent_runtime_mut().devices;
        runtime.serial = runtime.serial.saturating_add(1);
        while runtime.mines.len() >= capacity {
            runtime.mines.remove(0);
        }
        runtime.mines.push(AgentMine {
            serial: runtime.serial,
            at,
            variant,
            armed_at,
            radius,
            power,
            poison,
            freeze_rank: rank,
            residue_duration: duration,
            residue_power,
        });
    }
    fn throw_agent_grenade(&mut self, preview: AttackPreview) {
        let requested = self.player_agent_grenade_variant();
        let variant = if requested == AgentGrenadeVariant::Explosive
            || self.player_agent_skill_rank(requested.skill()) > 0
        {
            requested
        } else {
            AgentGrenadeVariant::Explosive
        };
        let center = preview.target();
        self.events.push(GameEvent::AgentAreaSpellCast {
            entity: self.player,
            skill: variant.skill().into(),
            origin: preview.origin(),
            target: center,
            radius: 1,
        });
        match variant {
            AgentGrenadeVariant::Explosive => {
                let power = self.agent_device_power(
                    8,
                    self.agent_device_bonus("SG04", "grenade_damage_bonus_percent"),
                );
                self.resolve_agent_device_explosion(
                    "SG00",
                    center,
                    Area::Square { radius: 1 },
                    DamageType::Explosive,
                    power,
                    None,
                    0,
                );
                self.note_agent_drone_attack(self.player);
            }
            AgentGrenadeVariant::Blinding => {
                let rank = self.player_agent_skill_rank("SG01");
                let duration = agent::definition("SG01")
                    .unwrap()
                    .value("blindness_duration_turns", rank);
                let targets: Vec<_> = self
                    .actors
                    .iter()
                    .filter_map(|(id, a)| {
                        (id != self.player
                            && preview.cells().iter().any(|c| c.position == a.position()))
                        .then_some(id)
                    })
                    .collect();
                for target in targets {
                    let chance = self.agent_grenade_blindness_chance(rank, target);
                    if chance == 0 || (chance < 100 && u16::from(self.rng.percentile()) > chance) {
                        continue;
                    }
                    if self.rules.statuses.get(&blinded()).is_some() {
                        let first = self.events.len();
                        self.apply_status_to(
                            Some(self.player),
                            target,
                            &ApplyStatusEffect::new(blinded(), 1).unwrap(),
                        )
                        .expect("registered blindness");
                        let mut applied = false;
                        for event in &mut self.events[first..] {
                            if let GameEvent::StatusApplied {
                                application,
                                remaining_turns,
                                ..
                            } = event
                            {
                                if *application != crate::status::StatusApplyKind::Ignored {
                                    *remaining_turns = Some(duration);
                                    applied = true;
                                }
                            }
                        }
                        if applied && let Some(a) = self.actors.get_mut(target) {
                            a.set_status_duration(&blinded(), duration);
                        }
                    }
                }
                self.note_agent_drone_attack(self.player);
            }
            AgentGrenadeVariant::Decoy => {
                let rank = self.player_agent_skill_rank("SG02");
                let reach = agent::definition("SG02")
                    .unwrap()
                    .value("open_terrain_sound_reach_cells", rank);
                let rules = self.rules.stealth_rules.unwrap_or_default();
                let intensity =
                    1u16.saturating_add(reach.saturating_mul(rules.sound_attenuation_per_cell));
                let duration = agent::definition("SG02")
                    .unwrap()
                    .value("emission_turns", rank);
                let expires = self.turn.saturating_add(u64::from(duration));
                self.agent_runtime_mut().devices.decoys.push(AgentDecoy {
                    at: center,
                    intensity,
                    expires,
                    attempted: Default::default(),
                });
                self.process_agent_decoys();
            }
        }
        self.end_player_active_camouflage();
    }
    pub fn agent_grenade_blindness_chance(&self, rank: u8, target: EntityId) -> u16 {
        let Some(a) = self.actors.get(target) else {
            return 0;
        };
        if a.tags()
            .iter()
            .any(|t| matches!(t.as_str(), "core:boss" | "agent:boss"))
        {
            return 0;
        }
        let intelligence = self
            .player_effective_primary_attributes()
            .map_or(5, |a| a.value(PrimaryAttribute::Processing));
        let power = u64::from(
            agent::definition("SG01")
                .unwrap()
                .value("application_power_rank_base", rank),
        ) + 2 * u64::from(intelligence.saturating_sub(5));
        let will = self
            .actor_effective_primary_attributes(target)
            .map_or(5, |a| a.value(PrimaryAttribute::Willpower));
        let resistance = (25 + 2 * (i32::from(will) - 5)).max(1) as u64;
        let chance = (100 * power.max(1) / (power.max(1) + resistance)).clamp(5, 95);
        let chance = if a
            .tags()
            .iter()
            .any(|t| matches!(t.as_str(), "core:elite" | "agent:elite"))
        {
            chance * 3 / 4
        } else {
            chance
        };
        chance as u16
    }
    pub fn agent_device_accuracy_percent(&self, entity: EntityId) -> u16 {
        if self
            .actors
            .get(entity)
            .is_some_and(|a| a.status(&blinded()).is_some())
        {
            70
        } else {
            100
        }
    }
    fn resolve_agent_device_explosion(
        &mut self,
        id: &str,
        center: GridPos,
        area: Area,
        damage_type: DamageType,
        power: StatusPower,
        poison: Option<(StatusPower, u16)>,
        freeze_rank: u8,
    ) {
        let cells = spatial::affected_cells(&self.map, center, area, (1, 0));
        let targets: Vec<_> = self
            .actors
            .iter()
            .filter_map(|(id, a)| {
                (id != self.player
                    && !self.map.is_protected(a.position())
                    && cells.contains(&a.position()))
                .then_some((id, a.position()))
            })
            .collect();
        for (victim, at) in targets {
            if self.actors.get(victim).is_none() {
                continue;
            }
            let (n, d) = crate::skills::area_damage::retention(center, at, area).unwrap();
            let critical = self.rng.percentile() <= 5;
            self.events.push(GameEvent::AgentSpellCast {
                entity: self.player,
                victim,
                skill: id.into(),
                origin: center,
                target: at,
                critical,
            });
            let _ = self.apply_damage_impact_to_scaled_spell(
                Some(self.player),
                victim,
                DamagePacket::new(power.base_damage, damage_type, 0),
                u128::from(power.numerator) * u128::from(n) * if critical { 2 } else { 1 },
                u128::from(power.denominator.max(1)) * u128::from(d),
            );
            if self.actors.get(victim).is_none() {
                continue;
            }
            if let Some((poison, duration)) = poison {
                self.apply_agent_device_poison(victim, poison, duration);
            }
            if freeze_rank > 0 {
                self.apply_agent_freeze("SM03", freeze_rank, victim, 1);
            }
        }
    }
    fn apply_agent_device_poison(
        &mut self,
        target: EntityId,
        mut power: StatusPower,
        duration: u16,
    ) {
        let status: StatusId = "core:poisoned".parse().unwrap();
        if self.rules.statuses.get(&status).is_none() {
            return;
        }
        let start = self.events.len();
        self.apply_status_to(
            Some(self.player),
            target,
            &ApplyStatusEffect::new(status.clone(), 1).unwrap(),
        )
        .expect("core poison registered");
        if !self.events[start..].iter().any(|e|matches!(e,GameEvent::StatusApplied{application,..}if *application!=crate::status::StatusApplyKind::Ignored)){return;}
        power.applied_on_turn = self.turn;
        for e in &mut self.events[start..] {
            if let GameEvent::StatusApplied {
                remaining_turns, ..
            } = e
            {
                *remaining_turns = Some(duration);
            }
        }
        if let Some(a) = self.actors.get_mut(target) {
            a.set_status_power(&status, power, duration);
        }
        self.events.push(GameEvent::StatusTriggered {
            target,
            status: status.clone(),
            trigger: StatusTrigger::TurnEnd,
        });
        let packet = DamagePacket::new(power.base_damage, DamageType::Chemical, 0);
        let _ = self.apply_damage_impact_to_resolved(
            Some(self.player),
            target,
            DamageImpact::single(packet),
            false,
            Some((
                packet,
                u128::from(power.numerator),
                u128::from(power.denominator.max(1)),
            )),
        );
    }
    pub(super) fn trigger_agent_mines_at(&mut self, at: GridPos) {
        if !self.actors.entity_at(at).is_some_and(|id| {
            id != self.player
                && !self.is_player_controlled_companion(id)
                && self
                    .actors
                    .get(id)
                    .is_some_and(|a| a.player_relation() == crate::social::PlayerRelation::Hostile)
        }) {
            return;
        }
        let turn = self.turn;
        let Some(index) = self
            .agent_runtime()
            .devices
            .mines
            .iter()
            .position(|m| m.at == at && m.armed_at <= turn)
        else {
            return;
        };
        let mine = self.agent_runtime_mut().devices.mines.remove(index);
        self.events.push(GameEvent::AgentAreaSpellCast {
            entity: self.player,
            skill: mine.variant.skill().into(),
            origin: at,
            target: at,
            radius: mine.radius,
        });
        self.resolve_agent_device_explosion(
            mine.variant.skill(),
            at,
            Area::Square {
                radius: mine.radius,
            },
            mine.variant.damage_type(),
            mine.power,
            mine.poison,
            if mine.variant == AgentMineVariant::Cold {
                mine.freeze_rank
            } else {
                0
            },
        );
        if mine.residue_duration > 0 {
            let cells = spatial::affected_cells(
                &self.map,
                at,
                Area::Square {
                    radius: mine.radius,
                },
                (1, 0),
            )
            .into_iter()
            .filter(|p| !self.map.is_protected(*p))
            .collect();
            let mut residue = AgentMineResidue {
                serial: mine.serial,
                center: at,
                cells,
                variant: mine.variant,
                remaining: mine.residue_duration,
                next_turn: turn.saturating_add(1),
                power: mine.residue_power,
                freeze_rank: mine.freeze_rank,
                exposures: Default::default(),
                frozen_targets: Default::default(),
                last_hit: Default::default(),
            };
            self.tick_agent_mine_residue(&mut residue);
            residue.remaining = residue.remaining.saturating_sub(1);
            if residue.remaining > 0 {
                self.agent_runtime_mut().devices.residues.push(residue);
            }
        }
        self.refresh_agent_zone_freeze_statuses();
    }
    pub fn agent_device_zone_freezes(&self, entity: EntityId) -> bool {
        self.agent_runtime()
            .devices
            .residues
            .iter()
            .any(|r| r.frozen_targets.contains(&entity))
    }
    pub(super) fn clear_agent_device_freeze(&mut self, entity: EntityId) {
        for residue in &mut self.agent_runtime_mut().devices.residues {
            residue.frozen_targets.remove(&entity);
        }
        if self
            .actors
            .get_mut(entity)
            .is_some_and(|a| a.remove_status(&zone_freeze()))
        {
            self.events.push(GameEvent::StatusRemoved {
                target: entity,
                status: zone_freeze(),
                reason: super::super::StatusRemovalReason::Consumed,
            });
        }
    }
    fn tick_agent_mine_residue(&mut self, residue: &mut AgentMineResidue) {
        let targets: Vec<_> = self
            .actors
            .iter()
            .filter_map(|(id, a)| {
                (id != self.player
                    && !self.map.is_protected(a.position())
                    && residue.cells.contains(&a.position()))
                .then_some(id)
            })
            .collect();
        let covered: BTreeSet<_> = targets.iter().copied().collect();
        residue.exposures.retain(|id, _| covered.contains(id));
        residue.frozen_targets.retain(|id| covered.contains(id));
        for target in targets {
            self.apply_agent_mine_residue_to(residue, target);
        }
    }
    fn apply_agent_mine_residue_to(&mut self, residue: &mut AgentMineResidue, target: EntityId) {
        if residue.last_hit.get(&target).copied() == Some(self.turn) {
            return;
        }
        residue.last_hit.insert(target, self.turn);
        let exposure = residue.exposures.entry(target).or_insert(0);
        *exposure = exposure.saturating_add(1);
        if residue.variant == AgentMineVariant::Cold {
            let chance = self.agent_freeze_chance("SM03", residue.freeze_rank, target, *exposure);
            if !residue.frozen_targets.contains(&target)
                && chance > 0
                && (chance >= 100 || u16::from(self.rng.percentile()) <= chance)
            {
                residue.frozen_targets.insert(target);
                let burning: StatusId = "core:burning".parse().unwrap();
                if self
                    .actors
                    .get_mut(target)
                    .is_some_and(|a| a.remove_status(&burning))
                {
                    self.events.push(GameEvent::StatusRemoved {
                        target,
                        status: burning,
                        reason: super::super::StatusRemovalReason::Consumed,
                    });
                }
            }
        } else {
            let p = residue.power;
            let _ = self.apply_damage_impact_to_scaled_spell(
                Some(self.player),
                target,
                DamagePacket::new(p.base_damage, residue.variant.damage_type(), 0),
                u128::from(p.numerator),
                u128::from(p.denominator.max(1)),
            );
        }
    }
    /// Immediate entry and exit share the ordinary zone's once-per-target-turn budget.
    pub(super) fn agent_device_actor_moved(&mut self, entity: EntityId) {
        let Some(at) = self.actors.get(entity).map(Actor::position) else {
            return;
        };
        self.trigger_agent_mines_at(at);
        if self.actors.get(entity).is_none() {
            return;
        }
        let mut residues = std::mem::take(&mut self.agent_runtime_mut().devices.residues);
        for residue in &mut residues {
            if !residue.cells.contains(&at) {
                residue.exposures.remove(&entity);
                residue.frozen_targets.remove(&entity);
            } else if entity != self.player && !self.map.is_protected(at) {
                self.apply_agent_mine_residue_to(residue, entity);
            }
        }
        self.agent_runtime_mut().devices.residues = residues;
        self.refresh_agent_zone_freeze_statuses();
    }
    fn refresh_agent_zone_freeze_statuses(&mut self) {
        let frozen: BTreeSet<_> = self
            .agent_runtime()
            .devices
            .residues
            .iter()
            .flat_map(|r| r.frozen_targets.iter().copied())
            .collect();
        let actors: Vec<_> = self
            .actors
            .iter()
            .map(|(id, a)| (id, a.status(&zone_freeze()).is_some()))
            .collect();
        for (entity, has_status) in actors {
            if frozen.contains(&entity) {
                if !has_status && self.rules.statuses.get(&zone_freeze()).is_some() {
                    self.apply_status_to(
                        Some(self.player),
                        entity,
                        &ApplyStatusEffect::new(zone_freeze(), 1).unwrap(),
                    )
                    .unwrap();
                }
                if let Some(a) = self.actors.get_mut(entity) {
                    a.set_status_duration(&zone_freeze(), 2);
                }
            } else if has_status {
                self.actors
                    .get_mut(entity)
                    .unwrap()
                    .remove_status(&zone_freeze());
                self.events.push(GameEvent::StatusRemoved {
                    target: entity,
                    status: zone_freeze(),
                    reason: super::super::StatusRemovalReason::Consumed,
                });
            }
        }
    }
    pub(super) fn process_agent_devices_turn(&mut self) {
        let turn = self.turn;
        let positions: Vec<_> = self
            .agent_runtime()
            .devices
            .mines
            .iter()
            .filter(|m| m.armed_at <= turn)
            .map(|m| m.at)
            .collect();
        for at in positions {
            self.trigger_agent_mines_at(at);
        }
        let mut residues = std::mem::take(&mut self.agent_runtime_mut().devices.residues);
        for residue in &mut residues {
            if residue.next_turn > turn {
                continue;
            }
            self.tick_agent_mine_residue(residue);
            residue.remaining = residue.remaining.saturating_sub(1);
            residue.next_turn = turn.saturating_add(1);
        }
        residues.retain(|r| r.remaining > 0);
        self.agent_runtime_mut().devices.residues = residues;
        self.refresh_agent_zone_freeze_statuses();
        self.process_agent_decoys();
    }
    pub(super) fn clear_agent_devices_on_zone_change(&mut self) {
        self.agent_runtime_mut().devices.mines.clear();
        self.agent_runtime_mut().devices.residues.clear();
        self.agent_runtime_mut().devices.decoys.clear();
        self.agent_runtime_mut().devices.lured.clear();
        self.refresh_agent_zone_freeze_statuses();
    }
    fn process_agent_decoys(&mut self) {
        let turn = self.turn;
        let rules = self.rules.stealth_rules.unwrap_or_default();
        let mut decoys = std::mem::take(&mut self.agent_runtime_mut().devices.decoys);
        decoys.retain(|d| d.expires > turn);
        for decoy in &mut decoys {
            self.events.push(GameEvent::NoiseEmitted {
                source: Some(self.player),
                at: decoy.at,
                intensity: decoy.intensity,
            });
            let targets: Vec<_> = self
                .actors
                .iter()
                .filter_map(|(id, a)| {
                    (id != self.player
                        && a.player_relation() == crate::social::PlayerRelation::Hostile
                        && !self.is_player_controlled_companion(id)
                        && !decoy.attempted.contains(&id)
                        && rules.sound_reaches_on_map(
                            &self.map,
                            decoy.intensity,
                            decoy.at,
                            a.position(),
                        ))
                    .then_some(id)
                })
                .collect();
            for target in targets {
                decoy.attempted.insert(target);
                let actor = self.actors.get(target).unwrap();
                if actor
                    .tags()
                    .iter()
                    .any(|t| matches!(t.as_str(), "core:boss" | "agent:boss"))
                {
                    continue;
                }
                let elite = actor
                    .tags()
                    .iter()
                    .any(|t| matches!(t.as_str(), "core:elite" | "agent:elite"));
                if elite && self.rng.percentile() > 75 {
                    continue;
                }
                self.agent_runtime_mut()
                    .devices
                    .lured
                    .insert(target, decoy.at);
            }
        }
        self.agent_runtime_mut().devices.decoys = decoys;
        let live: BTreeSet<_> = self
            .actors
            .iter()
            .filter_map(|(id, a)| {
                self.agent_runtime()
                    .devices
                    .lured
                    .get(&id)
                    .filter(|at| **at != a.position())
                    .map(|_| id)
            })
            .collect();
        self.agent_runtime_mut()
            .devices
            .lured
            .retain(|id, _| live.contains(id));
    }
    pub(super) fn agent_device_lure_target(&self, entity: EntityId) -> Option<GridPos> {
        let at = *self.agent_runtime().devices.lured.get(&entity)?;
        let actor = self.actors.get(entity)?;
        // A real attack in reach keeps priority over investigating a decoy.
        let occupied_target =
            std::iter::once(self.player).chain(self.actors.iter().filter_map(|(id, a)| {
                a.drone()
                    .filter(|d| d.controller() == self.player)
                    .map(|_| id)
            }));
        if occupied_target.into_iter().any(|victim| {
            actor.attacks().iter().enumerate().any(|(slot, _)| {
                self.prepare_targeted_attack(entity, slot as u8, victim)
                    .is_ok()
            })
        }) {
            return None;
        }
        (actor.position() != at).then_some(at)
    }
}

#[cfg(test)]
mod tests {
    use super::super::agent_drone_cast::tests::fixture;
    use super::*;
    fn game() -> GameState {
        let mut g = fixture();
        for id in ["SM00", "SG00"] {
            g.player_skills.set_agent_skill_rank(id, 1);
        }
        install_statuses(&mut g.rules.statuses);
        g
    }
    fn hostile(g: &mut GameState, at: GridPos) -> EntityId {
        g.spawn_actor(
            Actor::new(at, 1000)
                .unwrap()
                .with_tags(["core:humanoid_knife_carrier".parse().unwrap()])
                .with_player_relation(crate::social::PlayerRelation::Hostile),
        )
        .unwrap()
    }
    #[test]
    fn grenade_variants_require_passives_and_switch_without_time_energy_or_rng_cost() {
        let mut g = game();
        let before = format!("{g:?}");
        assert!(
            g.set_player_agent_grenade_variant(AgentGrenadeVariant::Blinding)
                .is_err()
        );
        assert!(
            g.set_player_agent_mine_variant(AgentMineVariant::Poison)
                .is_err()
        );
        assert_eq!(format!("{g:?}"), before);
        g.player_skills.set_agent_skill_rank("SG01", 1);
        g.player_skills.set_agent_skill_rank("SM02", 1);
        let turn = g.turn;
        let energy = g.player_energy.available();
        g.set_player_agent_grenade_variant(AgentGrenadeVariant::Blinding)
            .unwrap();
        g.set_player_agent_mine_variant(AgentMineVariant::Poison)
            .unwrap();
        assert_eq!(g.turn, turn);
        assert_eq!(g.player_energy.available(), energy);
    }
    #[test]
    fn explosive_grenade_uses_square_three_by_three_falloff_and_preserves_caster() {
        let mut g = game();
        let center = GridPos::new(5, 4);
        let enemy = hostile(&mut g, center);
        let diagonal = hostile(&mut g, GridPos::new(6, 5));
        let outside = hostile(&mut g, GridPos::new(7, 4));
        let before = g.actors.get(g.player).unwrap().integrity();
        let state = format!("{g:?}");
        assert_eq!(
            g.agent_device_footprint("SG00", center)
                .unwrap()
                .cells()
                .len(),
            9
        );
        assert_eq!(format!("{g:?}"), state);
        g.cast_agent_device("SG00", center).unwrap();
        assert_eq!(g.actors.get(g.player).unwrap().integrity(), before);
        assert_eq!(g.actors.get(outside).unwrap().integrity(), 1000);
        assert!(g.actors.get(enemy).unwrap().integrity() < 1000);
        assert!(g.actors.get(diagonal).unwrap().integrity() < 1000);
        let changes: Vec<_> = g
            .events
            .iter()
            .filter_map(|e| {
                if let GameEvent::DamageApplied { target, amount, .. } = e {
                    Some((*target, *amount))
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(changes.len(), 2);
    }
    #[test]
    fn mine_arms_next_turn_then_triggers_on_hostile_entry_and_replaces_oldest_at_capacity() {
        let mut g = game();
        let first = GridPos::new(5, 4);
        let second = GridPos::new(4, 5);
        g.cast_agent_device("SM00", first).unwrap();
        let victim = hostile(&mut g, first);
        g.trigger_agent_mines_at(first);
        assert_eq!(g.player_agent_mines().len(), 1);
        assert_eq!(g.actors.get(victim).unwrap().integrity(), 1000);
        g.turn = 1;
        g.trigger_agent_mines_at(first);
        assert!(g.player_agent_mines().is_empty());
        assert!(g.actors.get(victim).unwrap().integrity() < 1000);
        g.cast_agent_device("SM00", second).unwrap();
        g.cast_agent_device("SM00", GridPos::new(3, 4)).unwrap();
        assert_eq!(g.player_agent_mines().len(), 1);
        assert_eq!(g.player_agent_mines()[0].at, GridPos::new(3, 4));
    }
    #[test]
    fn residue_covers_every_square_cell_has_immediate_first_tick_and_expires_by_reference_turn() {
        let mut g = game();
        g.player_skills.set_agent_skill_rank("SM05", 1);
        let center = GridPos::new(5, 4);
        g.cast_agent_device("SM00", center).unwrap();
        let trigger = hostile(&mut g, center);
        g.turn = 1;
        g.trigger_agent_mines_at(center);
        assert_eq!(g.player_agent_mine_residues().len(), 1);
        assert_eq!(g.player_agent_mine_residues()[0].cells.len(), 9);
        assert_eq!(g.player_agent_mine_residues()[0].remaining, 2);
        let hp = g.actors.get(trigger).unwrap().integrity();
        g.process_agent_devices_turn();
        assert_eq!(g.actors.get(trigger).unwrap().integrity(), hp);
        let entrant = hostile(&mut g, GridPos::new(6, 5));
        g.turn = 2;
        g.process_agent_devices_turn();
        assert!(g.actors.get(entrant).unwrap().integrity() < 1000);
        assert_eq!(g.player_agent_mine_residues()[0].remaining, 1);
        g.turn = 3;
        g.process_agent_devices_turn();
        assert!(g.player_agent_mine_residues().is_empty());
    }
    #[test]
    fn poison_explosion_applies_shared_poison_immediately_and_preserves_its_full_power_in_save() {
        let mut g = game();
        g.player_skills.set_agent_skill_rank("SM02", 1);
        g.set_player_agent_mine_variant(AgentMineVariant::Poison)
            .unwrap();
        let at = GridPos::new(5, 4);
        g.cast_agent_device("SM00", at).unwrap();
        let victim = hostile(&mut g, at);
        g.turn = 1;
        g.trigger_agent_mines_at(at);
        let status = g
            .actors
            .get(victim)
            .unwrap()
            .status(&"core:poisoned".parse().unwrap())
            .unwrap()
            .clone();
        assert_eq!(status.remaining_turns, Some(3)); // Immediate before the normal turn duration pass.
        assert!(status.power.is_some());
        let hp = g.actors.get(victim).unwrap().integrity();
        g.drain_events();
        let bytes = bincode::serialize(&g.snapshot().unwrap()).unwrap();
        let restored =
            GameState::from_snapshot(bincode::deserialize(&bytes).unwrap(), g.rules.clone());
        assert_eq!(restored.actors.get(victim).unwrap().integrity(), hp);
        assert_eq!(
            restored
                .actors
                .get(victim)
                .unwrap()
                .status(&"core:poisoned".parse().unwrap()),
            Some(&status)
        );
    }
    #[test]
    fn walls_cut_blast_and_zone_and_invalid_device_casts_remain_atomic() {
        let mut g = game();
        let at = GridPos::new(5, 4);
        g.map
            .set_terrain(GridPos::new(6, 4), Terrain::Wall)
            .unwrap();
        let behind = hostile(&mut g, GridPos::new(7, 4));
        g.player_skills.set_agent_skill_rank("SM04", 4);
        g.player_skills.set_agent_skill_rank("SM05", 1);
        g.cast_agent_device("SM00", at).unwrap();
        hostile(&mut g, at);
        g.turn = 1;
        g.trigger_agent_mines_at(at);
        assert_eq!(g.actors.get(behind).unwrap().integrity(), 1000);
        assert!(
            !g.player_agent_mine_residues()[0]
                .cells
                .contains(&GridPos::new(7, 4))
        );
        for invalid in [GridPos::new(6, 4), GridPos::new(9, 7), GridPos::new(-1, 4)] {
            let state = format!("{g:?}");
            assert!(g.cast_agent_device("SM00", invalid).is_err());
            assert_eq!(format!("{g:?}"), state);
        }
    }
    #[test]
    fn decoy_lures_each_hearing_hostile_once_without_damage_and_remembers_destination_after_noise()
    {
        let mut g = game();
        g.player_skills.set_agent_skill_rank("SG02", 1);
        g.set_player_agent_grenade_variant(AgentGrenadeVariant::Decoy)
            .unwrap();
        let target = hostile(&mut g, GridPos::new(6, 4));
        let at = GridPos::new(5, 4);
        g.cast_agent_device("SG00", at).unwrap();
        assert_eq!(g.agent_device_lure_target(target), Some(at));
        assert_eq!(g.actors.get(target).unwrap().integrity(), 1000);
        assert_eq!(g.agent_runtime().devices.decoys[0].attempted.len(), 1);
        g.turn = 1;
        g.process_agent_devices_turn();
        assert_eq!(g.agent_runtime().devices.decoys[0].attempted.len(), 1);
        g.turn = 2;
        g.process_agent_devices_turn();
        assert!(g.agent_runtime().devices.decoys.is_empty());
        assert_eq!(g.agent_device_lure_target(target), Some(at));
    }
    #[test]
    fn cold_residue_uses_zone_owned_freeze_and_releases_its_hold_after_exit() {
        let mut g = game();
        g.player_skills.set_agent_skill_rank("SM03", 20);
        g.player_skills.set_agent_skill_rank("SM05", 1);
        g.set_player_agent_mine_variant(AgentMineVariant::Cold)
            .unwrap();
        let at = GridPos::new(5, 4);
        g.cast_agent_device("SM00", at).unwrap();
        let target = hostile(&mut g, at);
        g.turn = 1;
        g.trigger_agent_mines_at(at);
        assert!(g.agent_device_zone_freezes(target));
        assert!(
            g.actors
                .get(target)
                .unwrap()
                .status(&zone_freeze())
                .is_some()
        );
        g.actors.move_to(target, GridPos::new(8, 4)).unwrap();
        g.turn = 2;
        g.process_agent_devices_turn();
        assert!(!g.agent_device_zone_freezes(target));
        assert!(
            g.actors
                .get(target)
                .unwrap()
                .status(&zone_freeze())
                .is_none()
        );
    }
    #[test]
    fn residue_entry_is_immediate_but_exit_reentry_and_periodic_phase_share_one_turn_budget() {
        let mut g = game();
        g.player_skills.set_agent_skill_rank("SM05", 2);
        let center = GridPos::new(5, 4);
        g.cast_agent_device("SM00", center).unwrap();
        hostile(&mut g, center);
        g.turn = 1;
        g.trigger_agent_mines_at(center);
        let entrant = hostile(&mut g, GridPos::new(7, 5));
        g.actors.move_to(entrant, GridPos::new(6, 5)).unwrap();
        g.agent_device_actor_moved(entrant);
        let hp = g.actors.get(entrant).unwrap().integrity();
        assert!(hp < 1000);
        g.actors.move_to(entrant, GridPos::new(7, 5)).unwrap();
        g.agent_device_actor_moved(entrant);
        g.actors.move_to(entrant, GridPos::new(6, 5)).unwrap();
        g.agent_device_actor_moved(entrant);
        g.process_agent_devices_turn();
        assert_eq!(g.actors.get(entrant).unwrap().integrity(), hp);
        g.turn = 2;
        g.process_agent_devices_turn();
        assert!(g.actors.get(entrant).unwrap().integrity() < hp);
    }
}
