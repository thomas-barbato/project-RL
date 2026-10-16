//! Ordinary weapon handling. A burst is one action; a grenade owns its timer.
use super::*;
use crate::explosive::{ExplosiveAreaProfile, ExplosivePayloadProfile};
use crate::weapon::{BurstFire, Launcher};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct SustainedFire {
    weapon: WeaponId,
    origin: GridPos,
    turn: u64,
    streak: u16,
}

pub(super) fn launcher_cells(map: &Map, at: GridPos, launcher: Launcher) -> Vec<AttackAreaCell> {
    launcher_payload(launcher)
        .into_payload()
        .footprint()
        .affected_cells(map, at, Direction::North)
}

fn launcher_payload(launcher: Launcher) -> ExplosivePayloadProfile {
    ExplosivePayloadProfile::new(
        ExplosiveAreaProfile::Radial {
            radius: launcher.radius,
            falloff_per_step: 0,
        },
        launcher.damage,
    )
}

impl GameState {
    pub(super) fn prepared_launcher(&self, prepared: &PreparedAttack) -> Option<Launcher> {
        prepared
            .weapon
            .as_ref()
            .and_then(|id| self.rules.weapons.get(id))
            .and_then(WeaponDefinition::launcher)
            .map(|launcher| self.agent_heavy_launcher(prepared.attacker, launcher))
    }

    pub(super) fn agent_heavy_launcher(
        &self,
        attacker: EntityId,
        mut launcher: Launcher,
    ) -> Launcher {
        if attacker == self.player && self.rules.agent_class_start.is_some() {
            let rank = self.player_agent_skill_rank("HL05");
            if rank > 0 {
                launcher.radius = launcher.radius.saturating_add(
                    crate::skills::agent::definition("HL05")
                        .unwrap()
                        .value("added_explosion_radius_cells", rank),
                );
            }
        }
        launcher
    }

    fn sustained_streak(&self, weapon: &WeaponId, origin: GridPos) -> u16 {
        self.sustained_fire
            .as_ref()
            .filter(|state| {
                state.weapon == *weapon
                    && state.origin == origin
                    && state.turn.saturating_add(1) == self.turn
            })
            .map_or(0, |state| state.streak)
    }

    pub(super) fn native_accuracy(&self, weapon: &WeaponDefinition, origin: GridPos) -> i16 {
        weapon.burst().map_or(0, |burst| {
            burst
                .sustained_accuracy
                .saturating_mul(self.sustained_streak(weapon.id(), origin).min(3) as i16)
        })
    }

    pub(super) fn ensure_native_heat(&self, prepared: &PreparedAttack) -> Result<(), AttackError> {
        if prepared.attacker != self.player {
            return Ok(());
        }
        let generated = prepared
            .weapon
            .as_ref()
            .and_then(|id| self.rules.weapons.get(id))
            .and_then(WeaponDefinition::burst)
            .map_or(0, |burst| burst.heat);
        if generated == 0 {
            return Ok(());
        }
        if let (Some(heat), Some(module)) = (
            self.player_heat,
            self.equipped_player_weapon_item(prepared.slot),
        ) {
            let projected = heat
                .current()
                .saturating_add(generated)
                .saturating_add(prepared.module_use.map_or(0, |usage| usage.heat_per_use));
            if projected > heat.critical_threshold() {
                return Err(AttackError::EngineeringModuleHeatLimit {
                    module,
                    projected,
                    maximum: heat.critical_threshold(),
                });
            }
        }
        Ok(())
    }

    pub(super) fn execute_native_weapon(
        &mut self,
        mut prepared: PreparedAttack,
    ) -> Result<(), AttackError> {
        let definition = prepared
            .weapon
            .as_ref()
            .and_then(|id| self.rules.weapons.get(id));
        let burst = definition.and_then(WeaponDefinition::burst);
        let launcher = definition
            .and_then(WeaponDefinition::launcher)
            .map(|launcher| self.agent_heavy_launcher(prepared.attacker, launcher));
        self.spend_prepared_attack_usage(&prepared, 1)?;
        if let Some(context) = prepared
            .agent_weapon
            .as_ref()
            .filter(|c| c.skill.starts_with("HL"))
        {
            if let Some(launcher) = launcher {
                self.events.push(GameEvent::AgentAreaSpellCast {
                    entity: prepared.attacker,
                    skill: format!(
                        "{}{}",
                        context.skill,
                        if launcher.delay_turns > 0 {
                            "-FLIGHT"
                        } else {
                            ""
                        }
                    ),
                    origin: prepared.origin,
                    target: prepared.target_at,
                    radius: launcher.radius,
                });
            } else {
                self.events.push(GameEvent::AgentWeaponSkillUsed {
                    entity: prepared.attacker,
                    skill: format!("{}-BULLET", context.skill),
                    origin: prepared.origin,
                    target: prepared.target_at,
                });
            }
        }
        if let Some(launcher) = launcher {
            if launcher.delay_turns > 0 {
                // Direct contact and the later blast are different events. The
                // delayed payload has no affix hooks and survives its victim.
                let at = prepared.target_at;
                let source = prepared.attacker;
                let material = prepared
                    .weapon
                    .clone()
                    .expect("launcher has an authored weapon");
                let facing = facing_toward(prepared.origin, at);
                let agent_payload =
                    (source == self.player && self.rules.agent_class_start.is_some()).then(|| {
                        let blast_attack = prepared
                            .attack
                            .with_damage(DamageImpact::single(launcher.damage));
                        super::agent_weapon_effects::AgentDelayedWeaponImpact {
                            context: prepared.agent_weapon.clone(),
                            attack: blast_attack,
                            damage: self
                                .resolved_attack_damage_or_error(source, blast_attack)
                                .expect("existing source"),
                            origin: prepared.origin,
                            source,
                        }
                    });
                prepared.affected_cells = vec![AttackAreaCell {
                    position: at,
                    step: 0,
                }];
                // The skill's stun and drain belong to the blast, not contact.
                prepared.agent_weapon = None;
                self.resolve_prepared_attack(prepared, ActionOrigin::Normal)?;
                let device = self
                    .explosive_devices
                    .deploy(
                        material.clone(),
                        Some(source),
                        at,
                        facing,
                        // complete_turn resolves the firing turn before incrementing
                        // it. A delay of two leaves two future actions to react.
                        ExplosiveActivation::Timed {
                            trigger_turn: self.turn.saturating_add(u64::from(launcher.delay_turns)),
                        },
                        true,
                        vec![ScheduledExplosivePayload::new(
                            0,
                            launcher_payload(launcher).into_payload(),
                        )],
                    )
                    .expect("validated launcher payload");
                if let Some(payload) = agent_payload {
                    self.agent_runtime_mut()
                        .weapon
                        .pending_explosions
                        .insert(device, payload);
                }
                self.events.push(GameEvent::ExplosiveDeployed {
                    entity: source,
                    device,
                    material,
                    at,
                });
                return Ok(());
            }
            // Rockets have one primary blast, not a direct hit plus another
            // identical center hit. Instance effects still resolve once.
            let self_hit = prepared
                .affected_cells
                .iter()
                .any(|cell| cell.position == prepared.origin);
            let source = prepared.attacker;
            if source == self.player && self.rules.agent_class_start.is_some() {
                prepared.attack = prepared
                    .attack
                    .with_damage(DamageImpact::single(launcher.damage));
            }
            let self_damage = self.resolved_attack_damage_or_error(source, prepared.attack)?;
            let stun_context = prepared.agent_weapon.clone();
            let impact = prepared.target_at;
            self.resolve_prepared_attack(prepared, ActionOrigin::Normal)?;
            self.agent_impact_stun(source, impact, stun_context.as_ref());
            if self_hit && self.actors.get(source).is_some_and(Actor::is_alive) {
                let _ = self.apply_damage_impact_to(Some(source), source, self_damage);
            }
            return Ok(());
        }
        if let Some(burst) = burst {
            let source = prepared.attacker;
            let at = prepared.target_at;
            let context = prepared.agent_weapon.clone();
            if self.resolve_native_burst(prepared, burst)? {
                self.agent_impact_stun(source, at, context.as_ref());
            }
            return Ok(());
        }
        self.resolve_prepared_attack(prepared, ActionOrigin::Normal)?;
        Ok(())
    }

    fn resolve_native_burst(
        &mut self,
        prepared: PreparedAttack,
        burst: BurstFire,
    ) -> Result<bool, AttackError> {
        let mut effects_used = false;
        let siphons: Vec<_> = prepared
            .weapon_effects
            .iter()
            .filter_map(|effect| match effect.kind() {
                WeaponEffectKind::LifeSteal {
                    percent,
                    maximum_per_attack,
                    required_target_tag,
                } => Some((
                    *percent,
                    *maximum_per_attack,
                    self.actors
                        .iter()
                        .filter_map(|(id, actor)| {
                            actor.tags().contains(required_target_tag).then_some(id)
                        })
                        .collect::<BTreeSet<_>>(),
                )),
                _ => None,
            })
            .collect();
        let mut siphoned_damage = vec![0_u32; siphons.len()];
        for shot in 0..burst.shots {
            if self
                .actors
                .get(prepared.attacker)
                .is_none_or(|actor| !actor.is_alive())
            {
                break;
            }
            let mut round = prepared.clone();
            if effects_used {
                round.weapon_effects.clear();
            } else if shot > 0 {
                round
                    .weapon_effects
                    .retain(|effect| effect.trigger() != Some(WeaponEffectTrigger::OnAttack));
            }
            round
                .weapon_effects
                .retain(|effect| !matches!(effect.kind(), WeaponEffectKind::LifeSteal { .. }));
            // Only the first projectile opens a reaction window. Secondary
            // equipment effects have a single successful-hit budget per burst.
            let outcome = self.resolve_prepared_attack(
                round,
                if shot == 0 || !effects_used {
                    ActionOrigin::Normal
                } else {
                    ActionOrigin::Reaction
                },
            )?;
            effects_used |= !outcome.hit_targets.is_empty();
            for (index, (_, _, admitted)) in siphons.iter().enumerate() {
                for (target, amount) in &outcome.primary_damage {
                    if admitted.contains(target) {
                        siphoned_damage[index] =
                            siphoned_damage[index].saturating_add(u32::from(*amount));
                    }
                }
            }
        }
        for (index, (percent, maximum, _)) in siphons.iter().enumerate() {
            if let Some(actor) = self
                .actors
                .get_mut(prepared.attacker)
                .filter(|actor| actor.is_alive())
            {
                let amount = ((u64::from(siphoned_damage[index]) * u64::from(*percent) / 100)
                    .min(u64::from(*maximum))) as u16;
                let restored = actor.restore_integrity(amount);
                if restored > 0 {
                    self.events.push(GameEvent::IntegrityRestored {
                        entity: prepared.attacker,
                        amount: restored,
                    });
                }
            }
        }
        if prepared.attacker == self.player {
            if burst.sustained_accuracy > 0 {
                let weapon = prepared.weapon.clone().expect("burst has a weapon");
                let streak = self
                    .sustained_streak(&weapon, prepared.origin)
                    .saturating_add(1)
                    .min(3);
                self.sustained_fire = Some(SustainedFire {
                    weapon,
                    origin: prepared.origin,
                    turn: self.turn,
                    streak,
                });
            }
            if burst.heat > 0
                && let Some(heat) = self.player_heat.as_mut()
            {
                heat.add(burst.heat);
                self.events.push(GameEvent::HeatGenerated {
                    entity: self.player,
                    amount: burst.heat,
                    current: heat.current(),
                });
            }
        }
        Ok(effects_used)
    }
}
