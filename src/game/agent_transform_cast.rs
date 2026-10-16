//! Autonomous form actions, shared catalogue numbers and equipment preservation.
#[path = "agent_transform_runtime.rs"]
mod runtime;
use super::*;
use crate::combat::{DamageComponent, DamageType};
use crate::skills::{
    agent,
    spatial::{self, Area},
};
use runtime::AgentActiveForm;
pub(crate) use runtime::AgentTransformRuntime;

#[derive(Clone, Debug)]
pub struct AgentFormActionInfo {
    pub technique: TechniqueId,
    pub title: &'static str,
    pub energy: u16,
    pub range: u16,
}
#[derive(Clone, Copy)]
struct FormAction {
    form: &'static str,
    suffix: &'static str,
    title: &'static str,
    damage: &'static str,
    energy: &'static str,
    range: &'static str,
    magic: bool,
    area: u8,
}
const FORM_ACTIONS: [FormAction; 15] = [
    FormAction {
        form: "M01",
        suffix: "claw",
        title: "Griffe",
        damage: "raw_claw_damage",
        energy: "claw_energy",
        range: "claw_range_cells",
        magic: false,
        area: 0,
    },
    FormAction {
        form: "M01",
        suffix: "leap",
        title: "Bond",
        damage: "",
        energy: "leap_energy",
        range: "leap_cells",
        magic: false,
        area: 3,
    },
    FormAction {
        form: "M01",
        suffix: "laceration",
        title: "Lacération",
        damage: "raw_laceration_damage_per_target",
        energy: "laceration_energy",
        range: "claw_range_cells",
        magic: false,
        area: 1,
    },
    FormAction {
        form: "TR61",
        suffix: "strike",
        title: "Coup lourd",
        damage: "raw_heavy_strike_damage",
        energy: "strike_energy",
        range: "strike_range_cells",
        magic: false,
        area: 0,
    },
    FormAction {
        form: "TR61",
        suffix: "shockwave",
        title: "Onde de choc",
        damage: "raw_shockwave_damage",
        energy: "shockwave_energy",
        range: "shockwave_radius_cells",
        magic: false,
        area: 2,
    },
    FormAction {
        form: "TR61",
        suffix: "crushing",
        title: "Écrasement",
        damage: "raw_heavy_strike_damage",
        energy: "crushing_energy",
        range: "strike_range_cells",
        magic: false,
        area: 0,
    },
    FormAction {
        form: "TR63",
        suffix: "heurt",
        title: "Heurt",
        damage: "raw_heurt_damage",
        energy: "heurt_energy",
        range: "heurt_range_cells",
        magic: false,
        area: 0,
    },
    FormAction {
        form: "TR63",
        suffix: "discharge",
        title: "Décharge de plaques",
        damage: "raw_discharge_damage_per_target",
        energy: "discharge_energy",
        range: "discharge_radius_cells",
        magic: false,
        area: 2,
    },
    FormAction {
        form: "TR63",
        suffix: "closure",
        title: "Fermeture des plaques",
        damage: "",
        energy: "closure_energy",
        range: "closure_duration_turns",
        magic: false,
        area: 4,
    },
    FormAction {
        form: "TR64",
        suffix: "bolt",
        title: "Trait corporel",
        damage: "raw_corporeal_bolt_damage",
        energy: "bolt_energy",
        range: "bolt_range_cells",
        magic: true,
        area: 0,
    },
    FormAction {
        form: "TR64",
        suffix: "pulse",
        title: "Pulsation occulte",
        damage: "raw_pulse_damage_per_target",
        energy: "pulse_energy",
        range: "pulse_radius_cells",
        magic: true,
        area: 2,
    },
    FormAction {
        form: "TR64",
        suffix: "drain",
        title: "Drain organique",
        damage: "raw_drain_damage",
        energy: "drain_energy",
        range: "drain_range_cells",
        magic: true,
        area: 0,
    },
    FormAction {
        form: "TR65",
        suffix: "reach",
        title: "Allonge",
        damage: "raw_reach_attack_damage",
        energy: "reach_attack_energy",
        range: "reach_attack_range_cells",
        magic: false,
        area: 0,
    },
    FormAction {
        form: "TR65",
        suffix: "line",
        title: "Fente abyssale",
        damage: "raw_line_strike_damage_per_target",
        energy: "line_strike_energy",
        range: "line_strike_length_cells",
        magic: false,
        area: 5,
    },
    FormAction {
        form: "TR65",
        suffix: "shadow_step",
        title: "Pas d’ombre",
        damage: "",
        energy: "shadow_step_energy",
        range: "shadow_step_range_cells",
        magic: true,
        area: 3,
    },
];
fn action(technique: &TechniqueId) -> Option<FormAction> {
    let name = technique.as_str().strip_prefix("agent:")?;
    FORM_ACTIONS
        .iter()
        .find(|a| name == format!("{}_{}", a.form.to_ascii_lowercase(), a.suffix))
        .copied()
}
pub(super) fn is_agent_transformation(id: &str) -> bool {
    matches!(id, "M01" | "TR61" | "TR63" | "TR64" | "TR65")
}
pub(super) fn is_agent_form_action(technique: &TechniqueId) -> bool {
    action(technique).is_some() || technique.as_str() == "agent:form_return"
}
impl GameState {
    pub fn player_agent_form(&self) -> Option<&str> {
        self.agent_runtime()
            .transform
            .active
            .as_ref()
            .map(|a| a.skill.as_str())
    }
    pub fn player_agent_form_remaining_turns(&self) -> u16 {
        self.agent_runtime()
            .transform
            .active
            .as_ref()
            .map_or(0, |f| f.expires.saturating_sub(self.turn).min(65535) as u16)
    }
    pub fn player_agent_form_actions(&self) -> Vec<AgentFormActionInfo> {
        FORM_ACTIONS
            .iter()
            .filter(|a| Some(a.form) == self.player_agent_form())
            .map(|a| {
                let technique: TechniqueId =
                    format!("agent:{}_{}", a.form.to_ascii_lowercase(), a.suffix)
                        .parse()
                        .unwrap();
                let p = self.agent_form_action_profile(&technique).unwrap();
                AgentFormActionInfo {
                    technique,
                    title: a.title,
                    energy: p.energy,
                    range: p.range,
                }
            })
            .collect()
    }
    pub fn agent_form_default_technique(&self) -> Option<TechniqueId> {
        self.player_agent_form_actions()
            .into_iter()
            .next()
            .map(|a| a.technique)
    }
    pub(super) fn agent_transform_blocks_skill(&self, id: &str) -> bool {
        self.player_agent_form()
            .is_some_and(|form| id != "M02" && id != form)
    }
    pub(super) fn agent_transform_bonus_percent(&self) -> u16 {
        if self.player_agent_form().is_none() {
            return 0;
        }
        let rank = self.player_agent_skill_rank("M03");
        self.rules
            .agent_class_start
            .as_ref()
            .map_or(0, |s| s.transformed_damage_percent.saturating_sub(100))
            .saturating_add(if rank > 0 {
                agent::definition("M03")
                    .unwrap()
                    .value("transformed_damage_bonus_percent", rank)
            } else {
                0
            })
    }
    pub(super) fn agent_transformation_profile(
        &self,
        id: &str,
        rank: u8,
    ) -> Option<AgentSpellProfile> {
        if !is_agent_transformation(id) {
            return None;
        }
        let s = agent::definition(id)?;
        let start = self.rules.agent_class_start.as_ref()?;
        let channel = self.player_agent_skill_rank("A03");
        let reduction = if channel > 0 {
            agent::definition("A03")?
                .value("magical_energy_cost_reduction_percent", channel)
                .min(99)
        } else {
            0
        };
        Some(AgentSpellProfile {
            range: 0,
            energy: (u64::from(s.value("energy", rank))
                * u64::from(start.magic_energy_percent)
                * u64::from(100 - reduction))
            .div_ceil(10000)
            .clamp(1, 65535) as u16,
            cooldown: 1,
            base_damage: 0,
            multiplier_numerator: 1,
            multiplier_denominator: 1,
        })
    }
    pub(super) fn agent_transformation_preview(
        &self,
        id: &str,
        target: GridPos,
    ) -> Result<AttackPreview, CommandRejection> {
        self.agent_skill_ready(id)?;
        let origin = self
            .player_position()
            .ok_or(CommandRejection::MissingPlayer)?;
        if target != origin {
            return Err(CommandRejection::AbilityTargetOutOfRange(target));
        }
        if self.map.is_protected(origin) {
            return Err(CommandRejection::ProtectedZone);
        }
        if self.player_agent_form().is_some_and(|f| f != id) {
            return Err(CommandRejection::TechniqueHasNoActiveAction(
                agent::technique(id),
            ));
        }
        Ok(AttackPreview::new(
            origin,
            origin,
            vec![AttackAreaCell {
                position: origin,
                step: 0,
            }],
        ))
    }
    pub(super) fn cast_agent_transformation(
        &mut self,
        id: &str,
        target: GridPos,
        weapon_slot: u8,
    ) -> Result<(), CommandRejection> {
        self.agent_transformation_preview(id, target)?;
        let rank = self.player_agent_skill_rank(id);
        let p = self.agent_skill_ready(id)?;
        let s = agent::definition(id).unwrap();
        let attrs = self.player_effective_primary_attributes();
        let duration = s
            .value("duration_rank_turns", rank)
            .saturating_add(attrs.map_or(0, |a| {
                (a.value(PrimaryAttribute::Willpower).saturating_sub(5) / 20).min(3)
            }))
            .saturating_add(
                self.rules
                    .agent_class_start
                    .as_ref()
                    .map_or(0, |s| s.transformation_duration_bonus),
            );
        let old = self
            .actors
            .get(self.player)
            .map(|a| (a.integrity(), a.maximum_integrity()))
            .unwrap();
        let expires = self.turn.saturating_add(u64::from(duration));
        if let Some(f) = &mut self.agent_runtime_mut().transform.active {
            f.rank = rank;
            f.expires = expires;
        } else {
            let slots = if weapon_slot < 2 && self.rules.uses_weapon_hands() {
                vec![0, 1]
            } else {
                vec![weapon_slot]
            };
            let mut types: BTreeMap<DamageType, u16> = BTreeMap::new();
            for slot in &slots {
                if let Some(w) = self.resolved_equipped_player_weapon(*slot) {
                    for c in w.attack().damage().components() {
                        let ty = if c.damage_type.is_physical() {
                            DamageType::Kinetic
                        } else {
                            c.damage_type
                        };
                        let n = types.entry(ty).or_default();
                        *n = n.saturating_add(c.amount);
                    }
                }
            }
            self.agent_runtime_mut().transform.active = Some(AgentActiveForm {
                skill: id.into(),
                rank,
                expires,
                extension_used: 0,
                reserve: 0,
                closure_until: 0,
                weapon_types: types.into_iter().collect(),
                original_weapon_slots: slots,
            });
        }
        self.player_energy
            .spend(p.energy)
            .expect("validated transformation energy");
        self.cancel_agent_magic_channel();
        self.refresh_equipment_resources();
        self.agent_form_restore_hp_ratio(old);
        self.events.push(GameEvent::TechniqueUsed {
            entity: self.player,
            technique: agent::technique(id),
            observed_on_turn: self.turn,
        });
        self.events.push(GameEvent::AgentSpellCast {
            entity: self.player,
            victim: self.player,
            skill: id.into(),
            origin: target,
            target,
            critical: false,
        });
        self.end_player_active_camouflage();
        Ok(())
    }
    fn agent_form_restore_hp_ratio(&mut self, (hp, max): (u16, u16)) {
        if let Some(a) = self.actors.get_mut(self.player) {
            let desired = ((u32::from(hp) * u32::from(a.maximum_integrity()))
                / u32::from(max.max(1)))
            .min(u32::from(a.maximum_integrity()))
            .max(u32::from(hp > 0)) as u16;
            if desired > a.integrity() {
                a.restore_integrity(desired - a.integrity());
            } else {
                a.apply_damage(a.integrity() - desired);
            }
        }
    }
    pub(super) fn end_agent_transformation(&mut self) {
        if self.agent_runtime().transform.active.is_none() {
            return;
        }
        let old = self
            .actors
            .get(self.player)
            .map(|a| (a.integrity(), a.maximum_integrity()));
        self.agent_runtime_mut().transform.active = None;
        self.refresh_equipment_resources();
        if let Some(old) = old {
            self.agent_form_restore_hp_ratio(old);
        }
    }
    pub(super) fn process_agent_transform_turn(&mut self) {
        if self
            .agent_runtime()
            .transform
            .active
            .as_ref()
            .is_some_and(|f| self.turn.saturating_add(1) >= f.expires)
        {
            self.end_agent_transformation();
        }
    }
    pub(super) fn agent_transform_maximum_hp(&self, base: u16) -> u16 {
        let Some(f) = &self.agent_runtime().transform.active else {
            return base;
        };
        if f.skill != "TR61" {
            return base;
        }
        let pct = agent::definition("TR61")
            .unwrap()
            .value("maximum_HP_bonus_percent", f.rank);
        (u32::from(base) * u32::from(100u16.saturating_add(pct)) / 100).clamp(1, 65535) as u16
    }
    pub(super) fn agent_transform_armor_bonus_percent(&self) -> u16 {
        self.agent_runtime()
            .transform
            .active
            .as_ref()
            .filter(|f| f.skill == "TR61")
            .map_or(0, |f| {
                agent::definition("TR61")
                    .unwrap()
                    .value("armor_bonus_percent", f.rank)
            })
    }
    pub(super) fn agent_transform_movement_time(&self, base: u16) -> u16 {
        self.agent_runtime()
            .transform
            .active
            .as_ref()
            .filter(|f| f.skill == "TR61")
            .map_or(base, |f| {
                (u32::from(base)
                    * u32::from(
                        100 + agent::definition("TR61")
                            .unwrap()
                            .value("movement_time_increase_percent", f.rank),
                    ))
                .div_ceil(100)
                .min(65535) as u16
            })
    }
    pub(super) fn agent_transform_reduce_direct(
        &self,
        target: EntityId,
        amount: u16,
        direct: bool,
    ) -> u16 {
        if target != self.player || !direct {
            return amount;
        }
        let Some(f) = self
            .agent_runtime()
            .transform
            .active
            .as_ref()
            .filter(|f| f.skill == "TR63")
        else {
            return amount;
        };
        let s = agent::definition("TR63").unwrap();
        let mut value = u32::from(amount)
            * u32::from(100 - s.value("direct_damage_reduction_percent", f.rank).min(100))
            / 100;
        if f.closure_until > self.turn {
            value = value
                * u32::from(
                    100 - s
                        .value("closure_additional_reduction_percent", f.rank)
                        .min(100),
                )
                / 100;
        }
        value as u16
    }
    pub(super) fn agent_transform_damage_received(
        &mut self,
        source: Option<EntityId>,
        target: EntityId,
        actual: u16,
        direct: bool,
    ) {
        if target != self.player
            || !direct
            || !source.is_some_and(|s| {
                self.actors
                    .get(s)
                    .is_some_and(|a| a.player_relation() == crate::social::PlayerRelation::Hostile)
            })
        {
            return;
        }
        let Some(f) = self
            .agent_runtime()
            .transform
            .active
            .as_ref()
            .filter(|f| f.skill == "TR63")
        else {
            return;
        };
        let rank = f.rank;
        let maximum = self
            .actors
            .get(self.player)
            .map_or(0, Actor::maximum_integrity);
        let cap = (u32::from(maximum)
            * u32::from(
                agent::definition("TR63")
                    .unwrap()
                    .value("reserve_cap_percent_of_maximum_HP", rank),
            )
            / 100) as u16;
        if let Some(f) = &mut self.agent_runtime_mut().transform.active {
            f.reserve = f.reserve.saturating_add(actual).min(cap);
        }
    }
    pub(super) fn extend_agent_transformation(&mut self, rank: u8, actual: u16) {
        if actual == 0 {
            return;
        }
        let s = agent::definition("M02").unwrap();
        let cap = s.value("maximum_added_turns", rank);
        let each = s.value("extension_turns_per_eligible_hit", rank);
        if let Some(f) = &mut self.agent_runtime_mut().transform.active {
            let gain = each.min(cap.saturating_sub(f.extension_used));
            f.extension_used = f.extension_used.saturating_add(gain);
            f.expires = f.expires.saturating_add(u64::from(gain));
        }
    }
    pub fn agent_form_action_profile(&self, technique: &TechniqueId) -> Option<AgentSpellProfile> {
        let a = action(technique)?;
        let f = self
            .agent_runtime()
            .transform
            .active
            .as_ref()
            .filter(|f| f.skill == a.form)?;
        let s = agent::definition(a.form)?;
        let start = self.rules.agent_class_start.as_ref()?;
        let attrs = self.player_effective_primary_attributes()?;
        let base = if a.damage.is_empty() {
            0
        } else {
            s.value(a.damage, f.rank)
        };
        let pool = if a.magic {
            u64::from(start.spell_damage_percent)
                + u64::from(attrs.value(PrimaryAttribute::Willpower).saturating_sub(5))
        } else {
            u64::from(start.melee_damage_percent)
                + u64::from(attrs.value(PrimaryAttribute::Power).saturating_sub(5))
        };
        let pool = pool + u64::from(self.agent_transform_bonus_percent());
        let coefficient = if a.suffix == "crushing" {
            s.value("crushing_damage_coefficient_percent", f.rank)
        } else {
            100
        };
        Some(AgentSpellProfile {
            range: if a.area == 4 {
                0
            } else {
                s.value(a.range, f.rank)
            },
            energy: s.value(a.energy, f.rank),
            cooldown: 1,
            base_damage: base,
            multiplier_numerator: (u64::from(self.player_progression.level().saturating_sub(1))
                + 10)
                * pool
                * u64::from(coefficient),
            multiplier_denominator: 100_000,
        })
    }
    pub fn agent_form_action_preview(
        &self,
        technique: &TechniqueId,
        target: GridPos,
    ) -> Result<AttackPreview, CommandRejection> {
        let origin = self
            .player_position()
            .ok_or(CommandRejection::MissingPlayer)?;
        if technique.as_str() == "agent:form_return" && self.player_agent_form().is_some() {
            return Ok(AttackPreview::new(
                origin,
                origin,
                vec![AttackAreaCell {
                    position: origin,
                    step: 0,
                }],
            ));
        }
        let a = action(technique)
            .ok_or_else(|| CommandRejection::TechniqueHasNoActiveAction(technique.clone()))?;
        let p = self
            .agent_form_action_profile(technique)
            .ok_or_else(|| CommandRejection::TechniqueHasNoActiveAction(technique.clone()))?;
        if self.player_energy.available() < p.energy {
            return Err(CommandRejection::InsufficientEnergy {
                required: p.energy,
                available: self.player_energy.available(),
            });
        }
        if self.agent_support_effect(self.player, "AF03").is_some()
            && matches!(a.suffix, "pulse" | "drain" | "shadow_step")
        {
            return Err(CommandRejection::TechniqueHasNoActiveAction(
                technique.clone(),
            ));
        }
        let target = if a.area == 2 || a.area == 4 {
            origin
        } else {
            target
        };
        if !self.map.contains(target) {
            return Err(CommandRejection::AbilityTargetOutsideMap(target));
        }
        if !spatial::in_cast_range(origin, target, p.range) {
            return Err(CommandRejection::AbilityTargetOutOfRange(target));
        }
        if !self.player_visibility.is_visible(target) {
            return Err(CommandRejection::AbilityNoLineOfSight(target));
        }
        if self.map.is_protected(origin) || self.map.is_protected(target) {
            return Err(CommandRejection::ProtectedZone);
        }
        let dx = target.x - origin.x;
        let dy = target.y - origin.y;
        let dir = if dx.abs() >= dy.abs() {
            (if dx < 0 { -1 } else { 1 }, 0)
        } else {
            (0, if dy < 0 { -1 } else { 1 })
        };
        let cells = match a.area {
            1 => spatial::affected_cells(
                &self.map,
                GridPos::new(origin.x + dir.0, origin.y + dir.1),
                Area::Wall { length: 3 },
                dir,
            ),
            2 => spatial::affected_cells(
                &self.map,
                origin,
                if a.form == "TR64" {
                    Area::Square { radius: p.range }
                } else {
                    Area::Disc { radius: p.range }
                },
                dir,
            ),
            5 => spatial::affected_cells(
                &self.map,
                GridPos::new(origin.x + dir.0, origin.y + dir.1),
                Area::Line {
                    length: p.range,
                    width: 1,
                },
                dir,
            ),
            _ => vec![target],
        };
        if a.area == 3 {
            if target == origin
                || !self.map.is_walkable(target)
                || self.actors.entity_at(target).is_some()
                || self.agent_magic_movement_blocked(self.player)
            {
                return Err(CommandRejection::AbilityTargetBlocked(target));
            }
            if a.suffix == "leap" {
                for at in form_line(origin, target).into_iter().skip(1) {
                    if !self.map.is_walkable(at) || self.actors.entity_at(at).is_some() {
                        return Err(CommandRejection::AbilityTargetBlocked(at));
                    }
                }
                if !has_line_of_sight(&self.map, origin, target, true) {
                    return Err(CommandRejection::AbilityNoLineOfSight(target));
                }
            } else if ![
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (1, -1),
                (-1, 1),
                (-1, -1),
            ]
            .iter()
            .any(|(x, y)| {
                !self
                    .map
                    .is_walkable(GridPos::new(target.x + x, target.y + y))
            }) {
                return Err(CommandRejection::AbilityTargetBlocked(target));
            }
        } else if a.area == 0 {
            if !has_line_of_sight(&self.map, origin, target, true) {
                return Err(CommandRejection::AbilityNoLineOfSight(target));
            }
            let victim = self
                .actors
                .entity_at(target)
                .ok_or(CommandRejection::AbilityTargetHasNoActor(target))?;
            if victim == self.player
                || self.is_player_controlled_companion(victim)
                || self
                    .actors
                    .get(victim)
                    .is_none_or(|x| x.player_relation() != crate::social::PlayerRelation::Hostile)
            {
                return Err(
                    CommandRejection::TechniqueTargetDoesNotMeetEngagementRequirement(victim),
                );
            }
        }
        Ok(AttackPreview::new(
            origin,
            target,
            cells
                .into_iter()
                .filter(|p| !self.map.is_protected(*p))
                .map(|position| AttackAreaCell {
                    position,
                    step: grid_distance(origin, position),
                })
                .collect(),
        ))
    }
    pub(super) fn cast_agent_form_action(
        &mut self,
        technique: &TechniqueId,
        target: GridPos,
    ) -> Result<(), CommandRejection> {
        let preview = self.agent_form_action_preview(technique, target)?;
        if technique.as_str() == "agent:form_return" {
            self.end_agent_transformation();
            return Ok(());
        }
        let a = action(technique).unwrap();
        let p = self.agent_form_action_profile(technique).unwrap();
        let f = self
            .agent_runtime()
            .transform
            .active
            .as_ref()
            .unwrap()
            .clone();
        self.player_energy
            .spend(p.energy)
            .expect("validated intrinsic action energy");
        self.cancel_agent_magic_channel();
        self.events.push(GameEvent::TechniqueUsed {
            entity: self.player,
            technique: technique.clone(),
            observed_on_turn: self.turn,
        });
        if a.area == 3 || a.area == 4 {
            self.events.push(GameEvent::AgentSpellCast {
                entity: self.player,
                victim: self.player,
                skill: format!("{}_{}", a.form, a.suffix),
                origin: preview.origin(),
                target: preview.target(),
                critical: false,
            });
        }
        if a.area == 3 {
            self.actors
                .get_mut(self.player)
                .unwrap()
                .set_position(preview.target());
            self.refresh_player_visibility();
            self.record_agent_movement(self.player, preview.origin(), preview.target());
        } else if a.area == 4 {
            let expires = self.turn.saturating_add(u64::from(
                agent::definition("TR63")
                    .unwrap()
                    .value("closure_duration_turns", f.rank),
            ));
            self.agent_runtime_mut()
                .transform
                .active
                .as_mut()
                .unwrap()
                .closure_until = expires;
        } else {
            let recipients: Vec<_> = self
                .actors
                .iter()
                .filter_map(|(e, b)| {
                    (e != self.player
                        && b.player_relation() == crate::social::PlayerRelation::Hostile
                        && !self.is_player_controlled_companion(e)
                        && preview.cells().iter().any(|c| c.position == b.position()))
                    .then_some((e, b.position()))
                })
                .collect();
            let extra = if a.suffix == "discharge" {
                f.reserve
            } else {
                0
            };
            if a.suffix == "discharge" {
                self.agent_runtime_mut()
                    .transform
                    .active
                    .as_mut()
                    .unwrap()
                    .reserve = 0;
            }
            self.events.push(GameEvent::AgentAreaSpellCast {
                entity: self.player,
                skill: format!("{}_{}", a.form, a.suffix),
                origin: preview.origin(),
                target: preview.target(),
                radius: if a.area == 2 { p.range } else { 0 },
            });
            if a.magic {
                for (victim, at) in recipients {
                    let before = self.actors.get(victim).unwrap().integrity();
                    let critical = self.rng.percentile() <= 5;
                    self.events.push(GameEvent::AgentSpellCast {
                        entity: self.player,
                        victim,
                        skill: format!("{}_{}", a.form, a.suffix),
                        origin: preview.origin(),
                        target: at,
                        critical,
                    });
                    let typed =
                        self.agent_form_typed_impact(p.base_damage, &f, DamageType::Corruption);
                    let (area_n, area_d) = if a.suffix == "pulse" {
                        crate::skills::area_damage::retention(
                            preview.origin(),
                            at,
                            Area::Square { radius: p.range },
                        )
                        .unwrap_or((1, 1))
                    } else {
                        (1, 1)
                    };
                    let parts: Vec<_> = typed.components().collect();
                    if parts.len() == 1 {
                        let c = parts[0];
                        let _ = self.apply_damage_impact_to_scaled_spell(
                            Some(self.player),
                            victim,
                            DamagePacket::new(c.amount, c.damage_type, 0),
                            u128::from(p.multiplier_numerator)
                                * u128::from(area_n)
                                * if critical { 2 } else { 1 },
                            u128::from(p.multiplier_denominator) * u128::from(area_d),
                        );
                    } else {
                        let typed = typed.map_amounts(|amount| {
                            (u128::from(amount)
                                * u128::from(p.multiplier_numerator)
                                * u128::from(area_n)
                                * if critical { 2 } else { 1 }
                                / (u128::from(p.multiplier_denominator) * u128::from(area_d)))
                            .min(65535) as u16
                        });
                        let _ = self.apply_damage_impact_to(Some(self.player), victim, typed);
                    }
                    let removed =
                        before.saturating_sub(self.actors.get(victim).map_or(0, Actor::integrity));
                    if a.suffix == "drain" {
                        let pct = agent::definition("TR64")
                            .unwrap()
                            .value("drain_life_return_percent", f.rank);
                        let healed = self.actors.get_mut(self.player).map_or(0, |actor| {
                            actor.restore_integrity(
                                (u32::from(removed) * u32::from(pct) / 100) as u16,
                            )
                        });
                        if healed > 0 {
                            self.events.push(GameEvent::IntegrityRestored {
                                entity: self.player,
                                amount: healed,
                            });
                        }
                    }
                }
            } else {
                let raw = p.damage_before_defenses().saturating_add(extra);
                let impact = self.agent_form_typed_impact(
                    raw,
                    &f,
                    if a.form == "M01" {
                        DamageType::Piercing
                    } else {
                        DamageType::Kinetic
                    },
                );
                let mut attack = AttackProfile::melee(DamageType::Kinetic, raw)
                    .with_range(p.range)
                    .with_damage(impact);
                attack = self.agent_weapon_passive_attack(attack, u8::MAX);
                let context = super::agent_weapon_effects::AgentWeaponHit {
                    skill: a.form.to_owned(),
                    rank: f.rank,
                    armor_ignored_percent: if a.suffix == "crushing" {
                        agent::definition("TR61")
                            .unwrap()
                            .value("crushing_armor_bypass_percent", f.rank)
                    } else {
                        0
                    },
                    ..Default::default()
                };
                let prepared = PreparedAttack {
                    agent_weapon: Some(context),
                    attacker: self.player,
                    target: self.actors.entity_at(preview.target()),
                    slot: 0,
                    origin: preview.origin(),
                    target_at: preview.target(),
                    attack,
                    weapon: None,
                    weapon_effects: Vec::new(),
                    module_use: None,
                    affected_cells: recipients
                        .iter()
                        .map(|(_, position)| AttackAreaCell {
                            position: *position,
                            step: grid_distance(preview.origin(), *position),
                        })
                        .collect(),
                    forced_movement: None,
                    technique_on_hit_effect: None,
                    damage_target: AttackDamageTarget::Body,
                };
                let resolution = self
                    .resolve_prepared_attack(prepared, ActionOrigin::Normal)
                    .map_err(CommandRejection::from)?;
                for (victim, actual) in resolution.primary_damage {
                    if self.actors.get(victim).is_none() {
                        continue;
                    }
                    if a.form == "M01"
                        && actual > 0
                        && u16::from(self.rng.percentile())
                            <= agent::definition("M01")
                                .unwrap()
                                .value("bleed_chance_percent", f.rank)
                    {
                        self.agent_form_bleed(victim, actual, f.rank);
                    }
                    if a.suffix == "shockwave" {
                        self.agent_form_stun(victim, f.rank);
                    }
                }
            }
        }
        self.end_player_active_camouflage();
        Ok(())
    }
    fn agent_form_typed_impact(
        &self,
        raw: u16,
        f: &AgentActiveForm,
        physical: DamageType,
    ) -> DamageImpact {
        let total: u32 = f.weapon_types.iter().map(|(_, v)| u32::from(*v)).sum();
        if total == 0 {
            return DamageImpact::single(DamagePacket::new(raw, physical, 0));
        }
        let mut amounts = BTreeMap::new();
        let mut allocated = 0u16;
        for (index, (ty, weight)) in f.weapon_types.iter().enumerate() {
            let value = if index + 1 == f.weapon_types.len() {
                raw.saturating_sub(allocated)
            } else {
                (u32::from(raw) * u32::from(*weight) / total) as u16
            };
            allocated = allocated.saturating_add(value);
            let ty = if ty.is_physical() { physical } else { *ty };
            if value > 0 {
                let n = amounts.entry(ty).or_insert(0u16);
                *n = n.saturating_add(value);
            }
        }
        if amounts.len() < 2 {
            let (ty, value) = amounts.into_iter().next().unwrap_or((physical, 0));
            DamageImpact::single(DamagePacket::new(value, ty, 0))
        } else {
            DamageImpact::mixed(
                amounts
                    .into_iter()
                    .map(|(ty, value)| DamageComponent::new(value, ty)),
                0,
                [],
            )
            .expect("inherited authored typed composition")
        }
    }
    fn agent_form_bleed(&mut self, target: EntityId, actual: u16, rank: u8) {
        let s = agent::definition("M01").unwrap();
        let amount = (u32::from(actual)
            * u32::from(s.value("bleed_percent_of_actual_hit_per_turn", rank))
            / 100) as u16;
        if amount == 0 {
            return;
        }
        let id: StatusId = "core:bleeding".parse().unwrap();
        let duration = s.value("bleed_duration_turns", rank);
        let before = self.events.len();
        let _ = self.apply_status_to(
            Some(self.player),
            target,
            &ApplyStatusEffect::new(id.clone(), 1).unwrap(),
        );
        if !self.events[before..].iter().any(|e|matches!(e,GameEvent::StatusApplied{target:t,application,..}if *t==target&&*application!=crate::status::StatusApplyKind::Ignored)){return}
        let turn = self.turn;
        if let Some(a) = self.actors.get_mut(target) {
            a.set_status_power(
                &id,
                crate::status::StatusPower {
                    base_damage: amount,
                    numerator: 1,
                    denominator: 1,
                    applied_on_turn: turn,
                },
                duration,
            );
        }
        self.events.push(GameEvent::StatusTriggered {
            target,
            status: id,
            trigger: StatusTrigger::TurnEnd,
        });
        let _ =
            self.apply_agent_net_damage(Some(self.player), target, amount, DamageType::Piercing);
    }
    fn agent_form_stun(&mut self, target: EntityId, rank: u8) {
        let Some(a) = self.actors.get(target) else {
            return;
        };
        if a.tags().iter().any(|t| t.as_str() == "core:boss") {
            return;
        }
        let elite = a.tags().iter().any(|t| t.as_str() == "core:elite");
        let s = agent::definition("TR61").unwrap();
        let mut chance = s.value("stun_chance_percent", rank);
        if elite {
            chance = chance * 3 / 4;
        }
        if u16::from(self.rng.percentile()) > chance {
            return;
        }
        let until = self
            .turn
            .saturating_add(u64::from(s.value("stun_duration_turns", rank)))
            .saturating_add(1);
        self.agent_runtime_mut().weapon.stuns.insert(target, until);
        let _ = self.apply_status_to(
            Some(self.player),
            target,
            &ApplyStatusEffect::new("agent:stunned".parse().unwrap(), 1).unwrap(),
        );
    }
}
fn form_line(a: GridPos, b: GridPos) -> Vec<GridPos> {
    let (mut x, mut y) = (a.x, a.y);
    let dx = (b.x - x).abs();
    let sx = if x < b.x { 1 } else { -1 };
    let dy = -(b.y - y).abs();
    let sy = if y < b.y { 1 } else { -1 };
    let mut err = dx + dy;
    let mut result = Vec::new();
    loop {
        result.push(GridPos::new(x, y));
        if x == b.x && y == b.y {
            break;
        }
        let twice = 2 * err;
        if twice >= dy {
            err += dy;
            x += sx
        }
        if twice <= dx {
            err += dx;
            y += sy;
        }
    }
    result
}

#[cfg(test)]
#[path = "agent_transform_tests.rs"]
mod tests;

impl GameState {
    pub fn player_agent_form_closure_active(&self) -> bool {
        self.agent_runtime()
            .transform
            .active
            .as_ref()
            .is_some_and(|f| f.skill == "TR63" && f.closure_until > self.turn)
    }
}
