use super::*;

impl AsciiApp {
    pub(super) fn capture_combat_visual(
        &mut self,
        event: &GameEvent,
        origins: &mut BTreeMap<EntityId, GridPos>,
        now: f64,
    ) {
        match event {
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
                self.combat_motion.play(*target, *at, source_at, true, now);
                // Devices with no front character drawing retain their existing effects.
                let bleeds = self
                    .front_actor_visual(*target)
                    .map(crate::game_art::visual_bleeds)
                    .unwrap_or_else(|| {
                        ModuleKind::for_symbol(self.hostile_glyph(*target)).is_some()
                    });
                if *target == self.game.player_id() || bleeds {
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
            GameEvent::EntityDied { entity, at } if *entity != self.game.player_id() => {
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
        app
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
            assert!(app.combat_motion.offset(target, 1.0504, false).length() > 0.16);
            assert_eq!(app.combat_motion.offset(target, 1.18, false), Vec2::ZERO);
            assert_eq!(app.combat_motion.offset(target, 1.0504, true), Vec2::ZERO);
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
        let restored = AsciiApp::decode_recovery_snapshot(&encoded, app.rules.clone()).unwrap();
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
        let old = AsciiApp::decode_recovery_snapshot(&legacy, app.rules.clone()).unwrap();
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
        let restored = AsciiApp::decode_recovery_snapshot(&encoded, app.rules.clone()).unwrap();
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
                app.rules.clone()
            )
            .is_err()
        );
    }
}
