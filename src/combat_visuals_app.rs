use super::*;

impl AsciiApp {
    pub(super) fn camera_glide_offset(&self) -> Vec2 {
        self.combat_motion.glide_offset(
            self.game.player_id(),
            self.world_presentation_time.get(),
            self.graphics.active.reduced_motion,
        )
    }

    pub(super) fn capture_combat_visual(
        &mut self,
        event: &GameEvent,
        origins: &mut BTreeMap<EntityId, GridPos>,
        now: f64,
    ) {
        match event {
            GameEvent::EntityMoved { entity, from, to } => {
                if self.game.player_visibility().is_visible(*from)
                    && self.game.player_visibility().is_visible(*to)
                {
                    self.combat_motion.glide_timed(
                        *entity,
                        *from,
                        *to,
                        now,
                        self.graphics.active.walk_duration_ms,
                    );
                } else {
                    self.combat_motion.stop_glide(*entity);
                }
            }
            GameEvent::AgentReanimated { at, .. } => {
                if self.game.player_visibility().is_visible(*at) {
                    self.combat_motion
                        .play_agent_magic("OC07-rise", *at, *at, 0, now);
                }
            }
            GameEvent::AgentSpellCast {
                entity,
                victim,
                skill,
                origin,
                target,
                ..
            } if matches!(
                skill.as_str(),
                "FR01"
                    | "FE06"
                    | "HY05"
                    | "OC05"
                    | "M02"
                    | "B02"
                    | "SU03"
                    | "SU04"
                    | "SU05"
                    | "SU06"
                    | "T02"
                    | "ES05"
                    | "DR01"
                    | "DP01"
                    | "TR64_bolt"
                    | "TR64_drain"
                    | "TR65_shadow_step"
                    | "M01_leap"
                    | "TR63_closure"
                    | "AF06"
            ) =>
            {
                origins.insert(*entity, *origin);
                if self.game.player_visibility().is_visible(*origin)
                    && self.game.player_visibility().is_visible(*target)
                {
                    self.combat_motion
                        .play_agent_magic(skill, *origin, *target, 0, now);
                    if matches!(
                        skill.as_str(),
                        "T02" | "ES05" | "M01_leap" | "TR65_shadow_step"
                    ) {
                        self.combat_motion
                            .play_agent_travel(*entity, *origin, *target, now);
                    }

                    if matches!(skill.as_str(), "FR01" | "M02" | "TR64_bolt") {
                        self.combat_motion.delay_agent_magic_impact(
                            *victim,
                            now + 0.18 * crate::animation_policy::duration_scale(skill),
                        );
                    }
                }
            }
            GameEvent::AgentAreaSpellCast {
                skill,
                origin,
                target,
                radius,
                ..
            } if matches!(
                skill.as_str(),
                "FE04"
                    | "FR02"
                    | "FR06"
                    | "SG00"
                    | "SG01"
                    | "SG02"
                    | "SM00"
                    | "SM02"
                    | "SM03"
                    | "HY07"
                    | "TR64_pulse"
                    | "M01_claw"
                    | "M01_laceration"
                    | "TR61_strike"
                    | "TR61_shockwave"
                    | "TR61_crushing"
                    | "TR63_heurt"
                    | "TR63_discharge"
                    | "TR65_reach"
                    | "TR65_line"
            ) || skill.starts_with("HL") =>
            {
                if self.game.player_visibility().is_visible(*target) {
                    self.combat_motion
                        .play_agent_magic(skill, *origin, *target, *radius, now);
                }
            }
            GameEvent::AgentSpellCast {
                entity,
                victim,
                origin,
                target,
                skill,
                ..
            } if skill == "FE01" => {
                origins.insert(*entity, *origin);
                if self.game.player_visibility().is_visible(*origin)
                    && self.game.player_visibility().is_visible(*target)
                {
                    self.combat_motion
                        .play_ember(*victim, *origin, *target, now);
                }
            }
            GameEvent::AgentAreaSpellCast {
                origin,
                target,
                radius,
                skill,
                ..
            } if skill == "FE05" => {
                if self.game.player_visibility().is_visible(*origin)
                    && self.game.player_visibility().is_visible(*target)
                {
                    self.combat_motion
                        .play_fireball(*origin, *target, *radius, now);
                }
            }
            GameEvent::AgentSpellCast {
                entity,
                victim,
                origin,
                skill,
                ..
            } if skill == "FE05" => {
                origins.insert(*entity, *origin);
                self.combat_motion.fireball_hit_after_flight(*victim, now);
            }
            GameEvent::AgentAreaSpellCast {
                target,
                radius,
                skill,
                ..
            } if skill == "A02" => {
                if self.game.player_visibility().is_visible(*target) {
                    self.combat_motion.play_blast(*target, *radius, now);
                }
            }
            GameEvent::AgentSpellCast {
                entity,
                origin,
                skill,
                ..
            } if skill == "A02" => {
                origins.insert(*entity, *origin);
            }
            GameEvent::AgentGuardianTaunted { origin, .. } => {
                if self.game.player_visibility().is_visible(*origin) {
                    self.combat_motion.play_taunt(*origin, now);
                }
            }
            GameEvent::AgentWeaponSkillUsed {
                skill,
                origin,
                target,
                ..
            } if skill == "B01" => {
                if self.game.player_visibility().is_visible(*origin)
                    && self.game.player_visibility().is_visible(*target)
                {
                    self.combat_motion.play_rupture(*origin, *target, now);
                }
            }
            GameEvent::AgentWeaponSkillUsed {
                skill,
                origin,
                target,
                ..
            } => {
                if self.game.player_visibility().is_visible(*origin)
                    && self.game.player_visibility().is_visible(*target)
                {
                    self.combat_motion
                        .play_agent_magic(skill, *origin, *target, 0, now);
                }
            }
            GameEvent::AgentSpellCast {
                skill,
                victim,
                origin,
                target,
                ..
            } if skill == "A01" => {
                if self.game.player_visibility().is_visible(*origin)
                    && self.game.player_visibility().is_visible(*target)
                {
                    self.combat_motion
                        .play_spell(*victim, *origin, *target, now);
                }
            }
            GameEvent::AttackPerformed {
                attacker,
                slot,
                origin,
                target_at,
                weapon,
                ..
            } => {
                origins.insert(*attacker, *origin);
                let attack = weapon
                    .as_ref()
                    .and_then(|id| self.game.rules().weapons.get(id))
                    .map(|weapon| weapon.attack())
                    .or_else(|| {
                        self.game
                            .actors()
                            .get(*attacker)
                            .and_then(|actor| actor.attack(*slot))
                    });
                if self.game.player_visibility().is_visible(*origin)
                    && attack.is_some_and(|attack| {
                        attack.delivery() == project_rl::combat::AttackDelivery::Melee
                    })
                {
                    self.combat_motion
                        .play(*attacker, *origin, *target_at, false, now);
                }
            }
            GameEvent::DamageApplied {
                source,
                target,
                at,
                amount,
                ..
            }
            | GameEvent::DamageImpactApplied {
                source,
                target,
                at,
                amount,
                ..
            } => {
                if *amount == 0 || !self.game.player_visibility().is_visible(*at) {
                    return;
                }
                let source_at = source
                    .and_then(|entity| {
                        origins
                            .get(&entity)
                            .copied()
                            .or_else(|| self.game.actors().get(entity).map(Actor::position))
                    })
                    .unwrap_or_else(|| at.step(Direction::North));
                let reaction_time = self.combat_motion.impact_time_for(*target, *at, now);
                self.combat_motion
                    .play(*target, *at, source_at, true, reaction_time);
                // Devices with no front character drawing retain their existing effects.
                let bleeds = self
                    .front_actor_visual(*target)
                    .map(crate::game_art::visual_bleeds)
                    .unwrap_or_else(|| {
                        ModuleKind::for_symbol(self.hostile_glyph(*target)).is_some()
                    });
                let physical = match event {
                    GameEvent::DamageApplied { damage_type, .. } => damage_type.is_physical(),
                    GameEvent::DamageImpactApplied { components, .. } => components
                        .iter()
                        .any(|c| c.damage_type.is_physical() && c.amount > 0),
                    _ => false,
                };
                if physical && (*target == self.game.player_id() || bleeds) {
                    self.terminal.combat_residue.hit(
                        *at,
                        *amount,
                        target.get() ^ self.game.turn(),
                        self.game.map(),
                        self.game.player_visibility(),
                    );
                }
            }
            GameEvent::StatusApplied {
                source,
                target,
                status,
                application,
                ..
            } => {
                if *application == project_rl::status::StatusApplyKind::Ignored
                    || !self
                        .game
                        .rules()
                        .statuses
                        .get(status)
                        .is_some_and(crate::combat_visuals::harmful_status)
                {
                    return;
                }
                let Some(at) = self.game.actors().get(*target).map(Actor::position) else {
                    return;
                };
                if !self.game.player_visibility().is_visible(at) {
                    return;
                }
                let source_at = source
                    .and_then(|entity| {
                        origins
                            .get(&entity)
                            .copied()
                            .or_else(|| self.game.actors().get(entity).map(Actor::position))
                    })
                    .unwrap_or_else(|| at.step(Direction::North));
                self.combat_motion.play(*target, at, source_at, true, now);
            }
            GameEvent::MonsterBodyFell {
                at,
                number,
                variant,
            } => {
                if self.game.player_visibility().is_visible(*at) {
                    let cell = self.terminal.combat_residue.cells.entry(*at).or_default();
                    cell.corpse = None;
                    cell.corpse_symbol =
                        Some(crate::game_art::campaign::corpse_code(*number, *variant));
                    cell.blood = if matches!(*number, 124..=127) { 0 } else { 6 };
                    cell.seed = self.game.turn() as u32 ^ (*number as u32);
                    cell.wall = false;
                }
            }
            GameEvent::EntityDied { entity, at } if *entity != self.game.player_id() => {
                self.combat_motion.stop_glide(*entity);
                if let Some(
                    crate::game_art::Visual::Actor(sprite)
                    | crate::game_art::Visual::ArmedActor { body: sprite, .. },
                ) = self.front_actor_visual(*entity)
                {
                    self.terminal.combat_residue.death(
                        *at,
                        sprite,
                        entity.get() ^ self.game.turn(),
                        self.game.player_visibility(),
                    );
                } else {
                    let symbol = self.hostile_glyph(*entity);
                    if ModuleKind::for_symbol(symbol).is_some() {
                        self.terminal.combat_residue.death_symbol(
                            *at,
                            symbol,
                            entity.get() ^ self.game.turn(),
                            self.game.player_visibility(),
                        );
                    }
                }
            }
            _ => {}
        }
    }
}

#[cfg(all(test, debug_assertions))]
mod tests {
    use super::*;

    fn fixture() -> AsciiApp {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.prepare_cyberpunk_diagnostic("combat-review").unwrap();
        app.select_weapon_slot(0); // These impact fixtures attack with the active knife.
        app
    }

    #[test]
    fn real_steps_only_change_presentation_and_hidden_moves_do_not_leave_glides() {
        let mut app = fixture();
        app.capture_events_at(Some(0.0));
        let id = app.game.player_id();
        let start = app.game.player_position().unwrap();
        for time in [1.0, 1.02] {
            assert_eq!(
                app.execute_command(GameCommand::Move(Direction::East)),
                CommandOutcome::Applied
            );
            let mut expected = app.game.clone();
            expected.drain_events();
            let snapshot = expected.recovery_snapshot_bytes().unwrap();
            app.capture_events_at(Some(time));
            assert_ne!(app.combat_motion.glide_offset(id, time, false), Vec2::ZERO);
            assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), snapshot);
        }
        assert_eq!(
            app.game.player_position(),
            Some(GridPos::new(start.x + 2, start.y))
        );
        assert_eq!(app.combat_motion.offset(id, 1.421, false), Vec2::ZERO);
        app.capture_combat_visual(
            &GameEvent::EntityMoved {
                entity: id,
                from: GridPos::new(-1, -1),
                to: app.game.player_position().unwrap(),
            },
            &mut BTreeMap::new(),
            1.03,
        );
        assert_eq!(app.combat_motion.glide_offset(id, 1.03, false), Vec2::ZERO);
    }

    #[test]
    fn walk_speed_does_not_gate_commands_or_modify_gameplay_or_saves() {
        let _review = crate::game_art::walking::review_corrections();
        let mut expected = None;
        for milliseconds in [120, 200, 400] {
            let mut app = fixture();
            app.graphics.active.walk_duration_ms = milliseconds;
            app.capture_events_at(Some(0.));
            for time in [1., 1.01] {
                assert_eq!(
                    app.execute_command(GameCommand::Move(Direction::East)),
                    CommandOutcome::Applied
                );
                app.capture_events_at(Some(time));
            }
            let snapshot = app.game.recovery_snapshot_bytes().unwrap();
            if let Some(previous) = &expected {
                assert_eq!(&snapshot, previous);
            } else {
                expected = Some(snapshot);
            }
        }
    }

    fn reaction_fixture() -> AsciiApp {
        let (mut rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        rules.player_base_abilities = vec![AbilityProfile::new(
            4,
            DistanceMetric::Euclidean,
            true,
            true,
            vec![EffectPrimitive::ApplyStatus(
                ApplyStatusEffect::new("core:armor_fragilized".parse().unwrap(), 1).unwrap(),
            )],
        )];
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.prepare_cyberpunk_diagnostic("combat-review").unwrap();
        app
    }

    #[test]
    fn applied_status_reaction_affects_both_player_and_monster_without_changing_the_simulation() {
        for affect_player in [false, true] {
            let mut app = reaction_fixture();
            app.capture_events_at(Some(0.0));
            let target = if affect_player {
                app.game.player_id()
            } else {
                app.game
                    .spawn_actor(
                        Actor::new(GridPos::new(11, 9), 20)
                            .unwrap()
                            .with_player_relation(PlayerRelation::Hostile),
                    )
                    .unwrap()
            };
            let at = app.game.actors().get(target).unwrap().position();
            assert!(app.game.player_visibility().is_visible(at));
            assert_eq!(
                app.game.process_player_command(GameCommand::UseAbility {
                    slot: 0,
                    target: at
                }),
                CommandOutcome::Applied
            );
            assert!(app.game.events().iter().any(|event| matches!(event,
                GameEvent::StatusApplied { target: id, .. } if *id == target)));
            assert!(!app.game.events().iter().any(|event| matches!(
                event,
                GameEvent::DamageApplied { .. } | GameEvent::DamageImpactApplied { .. }
            )));
            let mut expected = app.game.clone();
            expected.drain_events();
            let snapshot = expected.recovery_snapshot_bytes().unwrap();
            app.capture_events_at(Some(1.0));
            assert!(app.combat_motion.offset(target, 1.07056, false).length() > 0.16);
            assert_eq!(app.combat_motion.offset(target, 1.253, false), Vec2::ZERO);
            assert_eq!(app.combat_motion.offset(target, 1.07056, true), Vec2::ZERO);
            assert_eq!(app.game.actors().get(target).unwrap().position(), at);
            assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), snapshot);
        }
    }

    #[test]
    fn applied_status_reaction_ignores_rejected_existing_and_beneficial_effects() {
        let mut app = fixture();
        let target = app.game.player_id();
        let mut origins = BTreeMap::new();
        for (status, application) in [
            ("core:armor_fragilized", StatusApplyKind::Ignored),
            (
                "core:locomotion_hindrance_protection",
                StatusApplyKind::Applied,
            ),
        ] {
            app.capture_combat_visual(
                &GameEvent::StatusApplied {
                    source: None,
                    target,
                    status: status.parse().unwrap(),
                    stacks: 1,
                    remaining_turns: Some(3),
                    application,
                },
                &mut origins,
                1.0,
            );
            assert_eq!(app.combat_motion.offset(target, 1.05, false), Vec2::ZERO);
        }
        app.capture_combat_visual(
            &GameEvent::StatusApplicationBlocked {
                source: None,
                target,
                status: "core:locomotion_hindered".parse().unwrap(),
                blocking_status: "core:locomotion_hindrance_protection".parse().unwrap(),
            },
            &mut origins,
            2.0,
        );
        assert_eq!(app.combat_motion.offset(target, 2.05, false), Vec2::ZERO);
    }

    #[test]
    fn applied_status_reaction_does_not_reveal_hidden_targets() {
        let mut app = fixture();
        let at = (0..app.game.map().height())
            .flat_map(|y| {
                (0..app.game.map().width()).map(move |x| GridPos::new(x as i32, y as i32))
            })
            .find(|at| {
                app.game.map().is_walkable(*at)
                    && app.game.actors().entity_at(*at).is_none()
                    && !app.game.player_visibility().is_visible(*at)
            })
            .unwrap();
        let target = app.game.spawn_actor(Actor::new(at, 20).unwrap()).unwrap();
        app.capture_combat_visual(
            &GameEvent::StatusApplied {
                source: None,
                target,
                status: "core:armor_fragilized".parse().unwrap(),
                stacks: 1,
                remaining_turns: Some(3),
                application: StatusApplyKind::Applied,
            },
            &mut BTreeMap::new(),
            1.0,
        );
        assert_eq!(app.combat_motion.offset(target, 1.05, false), Vec2::ZERO);
    }

    #[test]
    fn equipped_pieces_reach_both_player_views_and_weapon_switch_keeps_them() {
        let mut app = fixture();
        let crate::game_art::Visual::Player { equipment, .. } = app.front_player_visual() else {
            panic!()
        };
        assert_eq!(equipment, [Some(107), Some(118), Some(121), Some(132)]);
        app.active_weapon_slot = 2;
        let crate::game_art::Visual::Player { equipment, .. } = app.front_player_visual() else {
            panic!()
        };
        assert_eq!(equipment, [Some(107), Some(118), Some(121), Some(132)]);
    }

    #[test]
    fn actual_combat_leaves_a_corpse_and_snapshot_preserves_it_without_changing_simulation() {
        let mut app = fixture();
        let at = GridPos::new(11, 9);
        let victim = app
            .game
            .spawn_actor(
                Actor::new(at, 1)
                    .unwrap()
                    .with_player_relation(PlayerRelation::Hostile),
            )
            .unwrap();
        app.actor_glyphs.insert(victim, 't');
        let Some(crate::game_art::Visual::Actor(sprite)) = app.front_actor_visual(victim) else {
            panic!()
        };
        for _ in 0..8 {
            if app.game.actors().get(victim).is_none() {
                break;
            }
            assert_eq!(
                app.game.process_player_command(GameCommand::Attack {
                    slot: 0,
                    target: victim
                }),
                CommandOutcome::Applied
            );
            let mut expected = app.game.clone();
            expected.drain_events();
            let before_feedback = expected.recovery_snapshot_bytes().unwrap();
            app.capture_events_at(Some(0.0));
            assert_eq!(before_feedback, app.game.recovery_snapshot_bytes().unwrap());
        }
        assert!(app.game.actors().get(victim).is_none());
        assert_eq!(app.terminal.combat_residue.cells[&at].corpse, Some(sprite));
        let snapshot = app.encode_recovery_snapshot().unwrap();
        let restored =
            AsciiApp::decode_recovery_snapshot(&snapshot, app.game.rules().clone()).unwrap();
        assert_eq!(
            restored.presentation.terminal.combat_residue,
            app.terminal.combat_residue
        );
    }

    #[test]
    fn damage_and_death_of_a_mechanical_actor_leave_only_a_persistent_dry_wreck() {
        let mut app = fixture();
        let at = GridPos::new(11, 9);
        let victim = app
            .game
            .spawn_actor(
                Actor::new(at, 1)
                    .unwrap()
                    .with_player_relation(PlayerRelation::Hostile),
            )
            .unwrap();
        app.actor_glyphs.insert(victim, 'P'); // Existing Sentinelle, actor index 12.
        for _ in 0..8 {
            if app.game.actors().get(victim).is_none() {
                break;
            }
            assert_eq!(
                app.game.process_player_command(GameCommand::Attack {
                    slot: 0,
                    target: victim
                }),
                CommandOutcome::Applied
            );
            app.capture_events_at(Some(0.0));
        }
        assert!(app.game.actors().get(victim).is_none());
        assert_eq!(app.terminal.combat_residue.cells[&at].corpse, Some(12));
        assert!(
            app.terminal
                .combat_residue
                .cells
                .values()
                .all(|cell| cell.blood == 0)
        );
        let encoded = app.encode_recovery_snapshot().unwrap();
        let restored =
            AsciiApp::decode_recovery_snapshot(&encoded, app.game.rules().clone()).unwrap();
        assert_eq!(
            restored.presentation.terminal.combat_residue,
            app.terminal.combat_residue
        );
    }

    #[test]
    fn combat_recovery_tail_preserves_cached_zones_and_rejects_corruption() {
        let mut app = fixture();
        app.capture_events_at(Some(0.0));
        let legacy = app.encode_recovery_snapshot().unwrap();
        let old = AsciiApp::decode_recovery_snapshot(&legacy, app.game.rules().clone()).unwrap();
        assert!(old.presentation.terminal.combat_residue.cells.is_empty());
        app.terminal.combat_residue.cells.insert(
            GridPos::new(10, 9),
            crate::combat_visuals::ResidueCell {
                blood: 6,
                corpse_symbol: Some('z'),
                ..Default::default()
            },
        );
        let zone: ContentId = "core:combat_memory_test".parse().unwrap();
        app.zone_views.insert(zone.clone(), app.terminal.clone());
        let encoded = app.encode_recovery_snapshot().unwrap();
        let restored =
            AsciiApp::decode_recovery_snapshot(&encoded, app.game.rules().clone()).unwrap();
        assert_eq!(
            restored.presentation.terminal.combat_residue,
            app.terminal.combat_residue
        );
        assert_eq!(
            restored.presentation.zone_views[&zone],
            app.zone_views[&zone]
        );
        let mut bytes = base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .unwrap();
        *bytes.last_mut().unwrap() ^= 1;
        assert!(
            AsciiApp::decode_recovery_snapshot(
                &base64::engine::general_purpose::STANDARD.encode(bytes),
                app.game.rules().clone()
            )
            .is_err()
        );
    }
}

#[cfg(test)]
mod agent_status_regression_tests {
    use super::*;
    #[test]
    fn agent_reanimation_mark_and_species_remain_visible_after_the_next_lab_turn() {
        let mut app = AsciiApp::new_agent_skill_laboratory().unwrap();
        let at = GridPos::new(6, 12);
        let original = app
            .game
            .spawn_actor(
                Actor::new(at, 1)
                    .unwrap()
                    .with_evasion_disabled()
                    .with_player_relation(PlayerRelation::Hostile),
            )
            .unwrap();
        app.actor_glyphs.insert(original, 'Q');
        let visual = app.front_actor_visual(original);
        assert_eq!(
            app.execute_command(GameCommand::UseTechniqueAt {
                technique: project_rl::skills::agent::technique("OC07"),
                target: at,
                weapon_slot: 0
            }),
            CommandOutcome::Applied
        );
        app.capture_events_at(Some(0.));
        assert!(app.game.agent_reanimation_marked(original));
        assert_eq!(
            app.execute_command(GameCommand::UseTechniqueAt {
                technique: project_rl::skills::agent::technique("A01"),
                target: at,
                weapon_slot: 0
            }),
            CommandOutcome::Applied
        );
        app.capture_events_at(Some(0.));
        let raised = app.game.actors().entity_at(at).unwrap();
        assert_ne!(original, raised);
        assert!(app.game.agent_reanimated_ally(raised));
        assert_eq!(app.front_actor_visual(raised), visual);
        assert_eq!(
            app.execute_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        app.capture_events_at(Some(1.));
        assert!(
            app.game.agent_reanimated_ally(raised),
            "a raised creature must remain after the next action"
        );
        assert_eq!(app.front_actor_visual(raised), visual);
        let position = app.game.actors().get(raised).unwrap().position();
        assert!(app.glyph_at(position).is_some());
    }
    #[test]
    fn agent_curse_icon_is_not_covered_by_a_generic_timer_badge() {
        let mut app = AsciiApp::new_agent_skill_laboratory().unwrap();
        let at = GridPos::new(8, 12);
        let victim = app.game.actors().entity_at(at).unwrap();
        let status: project_rl::status::StatusId = "agent:armor_breach".parse().unwrap();
        for _ in 0..20 {
            for _ in 0..10 {
                app.execute_command(GameCommand::Wait);
            }
            assert_eq!(
                app.execute_command(GameCommand::UseTechniqueAt {
                    technique: project_rl::skills::agent::technique("AF02"),
                    target: at,
                    weapon_slot: 0
                }),
                CommandOutcome::Applied
            );
            if app
                .game
                .actors()
                .get(victim)
                .unwrap()
                .status(&status)
                .is_some()
            {
                break;
            }
        }
        assert!(
            app.game
                .actors()
                .get(victim)
                .unwrap()
                .status(&status)
                .is_some()
        );
        assert!(crate::combat_visuals::has_agent_status_icon(
            status.as_str()
        ));
        assert!(
            !app.effect_badges_at(at)
                .iter()
                .any(|b| matches!(b, crate::terminal_view::TerminalEffectBadge::Timed))
        );
    }
}
