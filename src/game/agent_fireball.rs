//! FE05 projectile collision and shared FE01/FE05 per-cast Burn power.
use super::*;
use crate::status::StatusPower;

impl GameState {
    pub(super) fn agent_fireball_impact(&self, origin: GridPos, target: GridPos) -> GridPos {
        let mut at = origin;
        let dx = origin.x.abs_diff(target.x) as i64;
        let dy = -(origin.y.abs_diff(target.y) as i64);
        let sx = (target.x - origin.x).signum();
        let sy = (target.y - origin.y).signum();
        let mut error = dx + dy;
        while at != target {
            let mut next = at;
            let doubled = 2 * error;
            if doubled >= dy {
                error += dy;
                next.x += sx;
            }
            if doubled <= dx {
                error += dx;
                next.y += sy;
            }
            if !self.map.is_walkable(next)
                || (next.x != at.x
                    && next.y != at.y
                    && !self.map.is_walkable(GridPos::new(next.x, at.y))
                    && !self.map.is_walkable(GridPos::new(at.x, next.y)))
            {
                break;
            }
            at = next;
            if self.actors.entity_at(at).is_some() {
                break;
            }
        }
        at
    }
    pub fn agent_spell_burn_profile(&self, id: &str, rank: u8) -> Option<(StatusPower, u16, u16)> {
        if !matches!(id, "FE01" | "FE05") {
            return None;
        }
        let s = crate::skills::agent::definition(id)?;
        let profile = self.agent_spell_profile(id, rank)?;
        Some((
            StatusPower {
                base_damage: s.value("raw_burn_damage_per_turn", rank),
                numerator: profile.multiplier_numerator.saturating_sub(
                    (u64::from(self.player_progression.level().saturating_sub(1)) + 10)
                        * u64::from(self.agent_stealth_damage_bonus()),
                ),
                denominator: profile.multiplier_denominator,
                applied_on_turn: self.turn,
            },
            s.value(
                if id == "FE01" {
                    "ordinary_burn_chance_percent"
                } else {
                    "burn_chance_percent"
                },
                rank,
            ),
            s.value("burn_duration_turns", rank),
        ))
    }
    pub(super) fn apply_agent_burn(&mut self, target: EntityId, power: StatusPower, duration: u16) {
        self.apply_agent_burn_mode(target, power, duration, true);
    }
    pub(super) fn apply_agent_burn_mode(
        &mut self,
        target: EntityId,
        mut power: StatusPower,
        duration: u16,
        immediate: bool,
    ) {
        let previous = self
            .actors
            .get(target)
            .and_then(|a| a.status(&"core:burning".parse().unwrap()))
            .and_then(|s| s.power);
        let immediate = immediate || previous.is_none();
        if !immediate {
            power.applied_on_turn = previous.unwrap().applied_on_turn;
        }

        let id: StatusId = "core:burning".parse().unwrap();
        // Keep the shared Burn identity so cleansing, catalysis and ordinary fire interact.
        if self.rules.statuses.get(&id).is_none() {
            return;
        }
        let before = self.events.len();
        self.apply_status_to(
            Some(self.player),
            target,
            &ApplyStatusEffect::new(id.clone(), 1).unwrap(),
        )
        .expect("catalogue Burn");
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
        self.remove_agent_freeze(target);
        if let Some(actor) = self.actors.get_mut(target) {
            actor.set_status_power(&id, power, duration);
        }
        if !immediate {
            return;
        }

        // INTERACT-TIME01/02: first damage occurs at application/reapplication.
        // applied_on_turn suppresses a second automatic tick in this same turn;
        // the ordinary duration pass counts this first application exactly once.
        self.events.push(GameEvent::StatusTriggered {
            target,
            status: id,
            trigger: StatusTrigger::TurnEnd,
        });
        let packet = DamagePacket::new(power.base_damage, crate::combat::DamageType::Thermal, 0);
        let _ = self.apply_damage_impact_to_resolved(
            Some(self.player),
            target,
            DamageImpact::single(packet),
            false,
            Some((
                packet,
                u128::from(power.numerator),
                u128::from(power.denominator),
            )),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::super::agent_cast::tests::fixture;
    use super::*;
    use crate::combat::{DamageType, ResistanceProfile};
    fn arena() -> GameState {
        let mut g = fixture();
        g.player_skills.set_agent_skill_rank("FE05", 1);
        g.player_energy = crate::resources::EnergyReserve::new(60000, 60000).unwrap();
        g.drain_events();
        g
    }
    fn enemy(g: &mut GameState, p: GridPos) -> EntityId {
        g.spawn_actor(
            Actor::new(p, 60000)
                .unwrap()
                .with_player_relation(crate::social::PlayerRelation::Hostile),
        )
        .unwrap()
    }
    fn cast(p: GridPos) -> GameCommand {
        GameCommand::UseTechniqueAt {
            technique: crate::skills::agent::technique("FE05"),
            target: p,
            weapon_slot: 0,
        }
    }
    fn burn() -> StatusId {
        "core:burning".parse().unwrap()
    }
    #[test]
    fn fireball_intercepts_first_actor_and_matches_preview_with_falloff_and_caster_immunity() {
        let mut g = arena();
        let center = GridPos::new(4, 3);
        let first = enemy(&mut g, center);
        let edge = enemy(&mut g, GridPos::new(7, 3));
        let outside = enemy(&mut g, GridPos::new(9, 3));
        g.drain_events();
        let hp = g.actors.get(g.player).unwrap().integrity();
        let energy = g.player_energy.available();
        let profile = g.agent_skill_ready("FE05").unwrap();
        let before = format!("{g:?}");
        let preview = g.agent_spell_preview("FE05", GridPos::new(8, 3)).unwrap();
        assert_eq!(preview.target(), center);
        assert_eq!(format!("{g:?}"), before);
        assert!(
            preview
                .cells()
                .iter()
                .any(|c| c.position == GridPos::new(7, 3))
        );
        assert_eq!(
            g.process_player_command(cast(GridPos::new(8, 3))),
            CommandOutcome::Applied
        );
        assert_eq!(g.turn, 1);
        assert_eq!(g.player_energy.available(), energy - profile.energy);
        assert_eq!(g.actors.get(g.player).unwrap().integrity(), hp);
        assert_eq!(g.actors.get(outside).unwrap().integrity(), 60000);
        for (victim, factor) in [(first, 1.0), (edge, 0.5)] {
            let crit = g
                .events
                .iter()
                .find_map(|e| match e {
                    GameEvent::AgentSpellCast {
                        victim: v,
                        critical,
                        ..
                    } if *v == victim => Some(*critical),
                    _ => None,
                })
                .unwrap();
            let expected = (f64::from(profile.base_damage) * profile.multiplier_numerator as f64
                / profile.multiplier_denominator as f64
                * factor
                * if crit { 2.0 } else { 1.0 })
            .floor() as u16;
            let burned = g.actors.get(victim).unwrap().status(&burn());
            let tick = burned.and_then(|s| s.power).map_or(0, |p| {
                (u128::from(p.base_damage) * u128::from(p.numerator) / u128::from(p.denominator))
                    as u16
            });
            assert_eq!(
                60000 - g.actors.get(victim).unwrap().integrity(),
                expected + tick
            );
            if let Some(status) = burned {
                assert_eq!(status.remaining_turns, Some(2));
            }
        }
        assert_eq!(
            g.events
                .iter()
                .filter(|e| matches!(e,GameEvent::AgentAreaSpellCast {skill,..} if skill=="FE05"))
                .count(),
            1
        );
    }
    #[test]
    fn fireball_wall_impact_is_on_near_side_and_failed_aim_is_atomic() {
        let mut g = arena();
        let behind = enemy(&mut g, GridPos::new(7, 3));
        g.map
            .set_terrain(GridPos::new(6, 3), Terrain::Wall)
            .unwrap();
        g.refresh_player_visibility();
        g.drain_events();
        let preview = g.agent_spell_preview("FE05", GridPos::new(6, 3)).unwrap();
        assert_eq!(preview.target(), GridPos::new(5, 3));
        assert!(
            !preview
                .cells()
                .iter()
                .any(|c| c.position == GridPos::new(7, 3))
        );
        for p in [GridPos::new(-1, 0), GridPos::new(10, 5), GridPos::new(7, 3)] {
            let before = format!("{g:?}");
            assert!(matches!(
                g.process_player_command(cast(p)),
                CommandOutcome::Rejected(_)
            ));
            assert_eq!(format!("{g:?}"), before);
        }
        assert_eq!(
            g.process_player_command(cast(GridPos::new(6, 3))),
            CommandOutcome::Applied
        );
        assert_eq!(g.actors.get(behind).unwrap().integrity(), 60000);
        g.drain_events();
        let turn = g.turn;
        assert_eq!(
            g.process_player_command(cast(GridPos::new(5, 3))),
            CommandOutcome::Applied
        );
        assert_eq!(g.turn, turn + 1);
    }
    #[test]
    fn fireball_burn_keeps_cast_power_and_live_defenses_with_three_total_ticks_after_restore() {
        let mut g = arena();
        let target = enemy(&mut g, GridPos::new(5, 3));
        let mut power = g.agent_spell_burn_profile("FE05", 10).unwrap().0;
        power.base_damage = 18;
        power.numerator = 118;
        power.denominator = 100;
        g.apply_agent_burn(target, power, 3);
        assert_eq!(g.actors.get(target).unwrap().integrity(), 59979);
        g.complete_turn();
        assert_eq!(g.actors.get(target).unwrap().integrity(), 59979);
        assert_eq!(
            g.actors
                .get(target)
                .unwrap()
                .status(&burn())
                .unwrap()
                .remaining_turns,
            Some(2)
        );
        g.player_skills.set_agent_skill_rank("FE05", 1);
        g.drain_events();
        let rules = g.rules.clone();
        let world = crate::game::WorldState::single(g);
        let encoded = world.recovery_snapshot_bytes().unwrap();
        assert!(encoded.starts_with(b"RLWS\x0f"));
        let mut restored =
            crate::game::WorldState::from_recovery_snapshot_bytes(&encoded, rules).unwrap();
        assert_eq!(
            restored
                .actors
                .get(target)
                .unwrap()
                .status(&burn())
                .unwrap()
                .power,
            Some(power)
        );
        let actor = restored.actors.get_mut(target).unwrap();
        *actor = actor
            .clone()
            .with_resistances(ResistanceProfile::default().with(DamageType::Thermal, 50));
        restored.complete_turn();
        assert_eq!(restored.actors.get(target).unwrap().integrity(), 59969);
        restored.complete_turn();
        assert_eq!(restored.actors.get(target).unwrap().integrity(), 59959);
        assert!(
            restored
                .actors
                .get(target)
                .unwrap()
                .status(&burn())
                .is_none()
        );
        restored.complete_turn();
        assert_eq!(restored.actors.get(target).unwrap().integrity(), 59959);
    }
    #[test]
    fn fireball_latest_weaker_burn_replaces_power_and_restarts_duration_without_stacking() {
        let mut g = arena();
        let target = enemy(&mut g, GridPos::new(5, 3));
        let mut power = g.agent_spell_burn_profile("FE05", 10).unwrap().0;
        g.apply_agent_burn(target, power, 3);
        g.complete_turn();
        g.complete_turn();
        power.base_damage = 1;
        power.applied_on_turn = g.turn;
        g.apply_agent_burn(target, power, 3);
        let status = g.actors.get(target).unwrap().status(&burn()).unwrap();
        assert_eq!(status.stacks, 1);
        assert_eq!(status.remaining_turns, Some(3));
        assert_eq!(status.power, Some(power));
        let hp = g.actors.get(target).unwrap().integrity();
        g.complete_turn();
        assert_eq!(g.actors.get(target).unwrap().integrity(), hp);
        g.complete_turn();
        assert_eq!(g.actors.get(target).unwrap().integrity(), hp - 1);
        // A later ordinary weapon/status application must not reuse a former spell's power.
        g.apply_status_to(None, target, &ApplyStatusEffect::new(burn(), 1).unwrap())
            .unwrap();
        assert!(
            g.actors
                .get(target)
                .unwrap()
                .status(&burn())
                .unwrap()
                .power
                .is_none()
        );
    }
    #[test]
    fn fireball_guaranteed_burn_counts_first_tick_without_area_or_critical_power_scaling() {
        let mut g = arena();
        g.player_skills.set_agent_skill_rank("FE05", 25);
        let first = enemy(&mut g, GridPos::new(4, 3));
        let side = enemy(&mut g, GridPos::new(5, 4));
        let expected = g.agent_spell_burn_profile("FE05", 25).unwrap().0;
        assert_eq!(
            g.process_player_command(cast(GridPos::new(7, 3))),
            CommandOutcome::Applied
        );
        for id in [first, side] {
            let status = g.actors.get(id).unwrap().status(&burn()).unwrap();
            assert_eq!(status.power, Some(expected));
            assert_eq!(status.remaining_turns, Some(2));
            assert_eq!(status.stacks, 1);
        }
        assert!(g.actors.get(g.player).unwrap().status(&burn()).is_none());
    }
    #[test]
    fn fireball_collision_has_the_same_result_in_each_cardinal_direction() {
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let mut g = arena();
            let origin = GridPos::new(5, 3);
            g.actors.get_mut(g.player).unwrap().set_position(origin);
            let first = GridPos::new(origin.x + dx, origin.y + dy);
            enemy(&mut g, first);
            let aimed = GridPos::new(origin.x + 2 * dx, origin.y + 2 * dy);
            assert_eq!(g.agent_fireball_impact(origin, aimed), first);
        }
    }
    #[test]
    fn fireball_power_codec_preserves_v12_v13_nonempty_status_snapshots() {
        let mut g = arena();
        let target = enemy(&mut g, GridPos::new(5, 3));
        g.apply_status_to(None, target, &ApplyStatusEffect::new(burn(), 1).unwrap())
            .unwrap();
        g.agent_runtime = Default::default(); // data absent from the historical fixture
        g.drain_events();
        let rules = g.rules.clone();
        let world = crate::game::WorldState::single(g);
        for version in [12, 13] {
            let encode = || {
                crate::status::with_legacy_status_encoding(|| {
                    world.recovery_snapshot_bytes().unwrap()
                })
            };
            let mut bytes = if version == 12 {
                crate::stats::with_legacy_attribute_encoding(encode)
            } else {
                encode()
            };
            bytes[4] = version;
            let restored =
                crate::game::WorldState::from_recovery_snapshot_bytes(&bytes, rules.clone())
                    .unwrap();
            assert_eq!(format!("{restored:?}"), format!("{world:?}"));
            assert!(
                restored
                    .actors
                    .get(target)
                    .unwrap()
                    .status(&burn())
                    .unwrap()
                    .power
                    .is_none()
            );
        }
    }
}
