use super::*;
use crate::game::StatusRemovalReason;

impl GameState {
    /// A mark is target-owned and inert: charges do not multiply periodic damage.
    /// Its existing status instance supplies persistence, expiry and identity.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn apply_weapon_fracture(
        &mut self,
        source: EntityId,
        target: EntityId,
        impact: GridPos,
        previous: u16,
        mark_status: &StatusId,
        threshold: u16,
        effect: &RadialDamageEffect,
        affects_source: bool,
        weapon_effect: Option<(WeaponId, usize)>,
    ) -> bool {
        if self.map.is_protected(impact)
            || self
                .actors
                .get(source)
                .is_none_or(|actor| !actor.is_alive() || self.map.is_protected(actor.position()))
        {
            return false;
        }
        let charges = previous.saturating_add(1).min(threshold);
        let at = if let Some(actor) = self.actors.get(target) {
            if !actor.is_alive()
                || self.map.is_protected(actor.position())
                || actor.status(mark_status).map_or(0, |mark| mark.stacks) != previous
            {
                return false;
            }
            actor.position()
        } else if charges == threshold {
            impact // A lethal final hit keeps its captured mark and impact.
        } else {
            return false; // Never store a mark on a dead target or the floor.
        };
        if charges < threshold {
            let mark = ApplyStatusEffect::new(mark_status.clone(), 1).expect("one charge");
            if self.apply_status_to(Some(source), target, &mark).is_err() {
                return false;
            }
        } else if let Some(actor) = self.actors.get_mut(target) {
            actor.remove_status(mark_status);
            self.events.push(GameEvent::StatusRemoved {
                target,
                status: mark_status.clone(),
                reason: StatusRemovalReason::Consumed,
            });
        }
        self.events.push(GameEvent::WeaponFractureResolved {
            source,
            target,
            at: if charges == threshold { impact } else { at },
            status: mark_status.clone(),
            charges,
            threshold,
        });
        if charges == threshold {
            self.apply_radial_damage_excluding(
                Some(source),
                impact,
                effect,
                (!affects_source).then_some(source),
                weapon_effect,
            );
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::{ArmorRules, ConeAttack, DamageType, HitRules};
    use crate::effects::DamageFalloff;
    use crate::stats::{BodyProfile, DisplacementProfile};
    use crate::status::{StatusHook, StatusStacking, StatusTransition};
    use crate::weapon::WeaponCatalog;
    use crate::world::{NeighborMode, TerrainPropagationPolicy};

    fn id(name: &str) -> StatusId {
        format!("test:{name}").parse().unwrap()
    }
    fn fracture(expose: bool) -> WeaponEffect {
        WeaponEffect::accumulated_fracture(
            id("mark"),
            3,
            RadialDamageEffect {
                maximum_cost: 1,
                neighbor_mode: NeighborMode::CardinalAndDiagonal,
                propagation_policy: TerrainPropagationPolicy::blocked_by_walls(1),
                damage: DamagePacket::new(6, DamageType::Kinetic, 0),
                falloff: DamageFalloff::None,
            },
            expose,
        )
        .unwrap()
    }
    fn marker() -> StatusDefinition {
        StatusDefinition::new(
            id("mark"),
            Some(5),
            StatusStacking::AddStacks {
                maximum_stacks: 3,
                refresh_duration: true,
            },
            vec![],
        )
        .unwrap()
    }
    fn fixture(delivery: AttackDelivery, cone: bool, effects: Vec<WeaponEffect>) -> GameState {
        let mut attack = AttackProfile::new(
            10,
            DistanceMetric::Chebyshev,
            true,
            DamageType::Kinetic,
            5,
            0,
        )
        .with_delivery(delivery);
        if cone {
            attack = attack.with_area(AttackArea::Cone(ConeAttack::new(1, 1, 1).unwrap()));
        }
        let mut weapons = WeaponCatalog::default();
        weapons
            .register(
                WeaponDefinition::new(id("weapon"), "name".into(), "description".into(), attack)
                    .unwrap()
                    .with_effects(effects),
            )
            .unwrap();
        let mut statuses = StatusCatalog::default();
        statuses.register(marker()).unwrap();
        statuses
            .register(
                StatusDefinition::new(
                    id("other"),
                    Some(9),
                    StatusStacking::RefreshDuration,
                    vec![],
                )
                .unwrap(),
            )
            .unwrap();
        statuses
            .register(
                StatusDefinition::new(
                    id("recovery"),
                    Some(3),
                    StatusStacking::KeepExisting,
                    vec![],
                )
                .unwrap(),
            )
            .unwrap();
        GameState::new_with_rules(
            Map::filled(18, 12, Terrain::Floor).unwrap(),
            GridPos::new(2, 3),
            7,
            GameRules {
                weapons,
                statuses,
                player_base_attacks: vec![attack],
                player_weapon_slots: vec![id("slot")],
                player_starting_weapons: vec![id("weapon")],
                player_starting_equipment: vec![Some(id("weapon"))],
                ..GameRules::default()
            },
        )
        .unwrap()
    }
    fn actor(game: &mut GameState, at: GridPos, hp: u16) -> EntityId {
        game.spawn_actor(
            Actor::new(at, hp)
                .unwrap()
                .with_evasion_disabled()
                .with_body_profile(
                    BodyProfile::new(hp, 0)
                        .unwrap()
                        .with_displacement_profile(DisplacementProfile::new(10_000, 0).unwrap()),
                ),
        )
        .unwrap()
    }
    fn prime(game: &mut GameState, target: EntityId, charges: u16) {
        game.apply_status_to(
            Some(game.player),
            target,
            &ApplyStatusEffect::new(id("mark"), charges).unwrap(),
        )
        .unwrap();
        game.drain_events();
    }
    fn marks(game: &GameState, target: EntityId) -> u16 {
        game.actors
            .get(target)
            .and_then(|actor| actor.status(&id("mark")))
            .map_or(0, |mark| mark.stacks)
    }
    fn bursts(game: &GameState) -> Vec<GridPos> {
        game.events()
            .iter()
            .filter_map(|event| match event {
                GameEvent::WeaponFractureResolved {
                    at,
                    charges,
                    threshold,
                    ..
                } if charges == threshold => Some(*at),
                _ => None,
            })
            .collect()
    }
    fn hit(game: &mut GameState, target: EntityId) {
        assert_eq!(
            game.process_player_command(GameCommand::Attack { slot: 0, target }),
            CommandOutcome::Applied
        );
    }

    #[test]
    fn fracture_third_hit_bursts_at_impact_in_melee_and_ranged_and_preserves_other_statuses() {
        for delivery in [AttackDelivery::Melee, AttackDelivery::Ranged] {
            let mut game = fixture(delivery, false, vec![fracture(false)]);
            let at = GridPos::new(
                if delivery == AttackDelivery::Melee {
                    3
                } else {
                    7
                },
                3,
            );
            let target = actor(&mut game, at, 100);
            let neighbor = actor(&mut game, GridPos::new(at.x, 4), 100);
            let outside = actor(&mut game, GridPos::new(at.x, 5), 100);
            game.apply_status_to(
                None,
                target,
                &ApplyStatusEffect::new(id("other"), 1).unwrap(),
            )
            .unwrap();
            let player_hp = game.actors.get(game.player).unwrap().integrity();
            for expected in [1, 2] {
                hit(&mut game, target);
                assert_eq!(marks(&game, target), expected);
                assert!(bursts(&game).is_empty());
                assert_eq!(
                    game.actors
                        .get(target)
                        .unwrap()
                        .status(&id("mark"))
                        .unwrap()
                        .remaining_turns,
                    Some(4)
                );
                assert_eq!(game.actors.get(neighbor).unwrap().integrity(), 100);
            }
            hit(&mut game, target);
            assert_eq!(marks(&game, target), 0);
            assert_eq!(bursts(&game), [at]);
            assert_eq!(game.actors.get(target).unwrap().integrity(), 79);
            assert_eq!(game.actors.get(neighbor).unwrap().integrity(), 94);
            assert_eq!(game.actors.get(outside).unwrap().integrity(), 100);
            assert_eq!(game.actors.get(game.player).unwrap().integrity(), player_hp);
            assert!(
                game.actors
                    .get(target)
                    .unwrap()
                    .status(&id("other"))
                    .is_some()
            );
            hit(&mut game, target);
            assert_eq!(marks(&game, target), 1);
        }
    }

    #[test]
    fn fracture_marks_expire_without_detonating_and_are_independent_per_target() {
        let mut game = fixture(AttackDelivery::Ranged, false, vec![fracture(false)]);
        let first = actor(&mut game, GridPos::new(7, 3), 100);
        let second = actor(&mut game, GridPos::new(7, 5), 100);
        hit(&mut game, first);
        hit(&mut game, second);
        hit(&mut game, first);
        assert_eq!(marks(&game, first), 2);
        assert_eq!(marks(&game, second), 1);
        for _ in 0..4 {
            game.process_player_command(GameCommand::Wait);
        }
        assert_eq!(marks(&game, first), 0);
        assert_eq!(marks(&game, second), 0);
        assert!(bursts(&game).is_empty());
        hit(&mut game, first);
        assert_eq!(marks(&game, first), 1);
    }

    #[test]
    fn fracture_lethal_threshold_bursts_but_early_kills_leave_no_mark_or_burst() {
        for previous in [0, 1, 2] {
            let mut game = fixture(AttackDelivery::Ranged, false, vec![fracture(false)]);
            let at = GridPos::new(7, 3);
            let target = actor(&mut game, at, 5);
            let neighbor = actor(&mut game, GridPos::new(7, 4), 100);
            if previous > 0 {
                prime(&mut game, target, previous);
            }
            hit(&mut game, target);
            assert!(game.actors.get(target).is_none());
            assert_eq!(bursts(&game).len(), usize::from(previous == 2));
            assert_eq!(
                game.actors.get(neighbor).unwrap().integrity(),
                if previous == 2 { 94 } else { 100 }
            );
            assert!(game.ground_effects.is_empty());
        }
    }

    #[test]
    fn fracture_multitarget_duplicate_properties_and_secondary_damage_never_multiply_charges() {
        for aim in [7, 9] {
            let mut game = fixture(
                AttackDelivery::Ranged,
                true,
                vec![fracture(false), fracture(false)],
            );
            let first = actor(&mut game, GridPos::new(5, 3), 100);
            let last = actor(&mut game, GridPos::new(7, 3), 100);
            for turn in 1..=3 {
                game.process_player_command(GameCommand::AttackAt {
                    slot: 0,
                    target: GridPos::new(aim, 3),
                });
                let charged = if aim == 7 { last } else { first };
                let other = if aim == 7 { first } else { last };
                assert_eq!(marks(&game, other), 0);
                assert_eq!(marks(&game, charged), if turn == 3 { 0 } else { turn });
            }
            assert_eq!(
                bursts(&game),
                [GridPos::new(if aim == 7 { 7 } else { 5 }, 3)]
            );
        }
        let mut game = fixture(AttackDelivery::Ranged, false, vec![fracture(false)]);
        let target = actor(&mut game, GridPos::new(7, 3), 100);
        let neighbor = actor(&mut game, GridPos::new(7, 4), 100);
        prime(&mut game, target, 2);
        prime(&mut game, neighbor, 2);
        hit(&mut game, target);
        assert_eq!(marks(&game, neighbor), 2); // Splash does not charge/detonate a neighbor.
        assert_eq!(bursts(&game), [GridPos::new(7, 3)]);
    }

    #[test]
    fn fracture_requires_normal_hit_and_never_charges_on_a_miss_reaction_or_empty_ground() {
        for denial in ["miss", "reaction", "protected_target", "protected_source"] {
            let mut game = fixture(AttackDelivery::Ranged, false, vec![fracture(false)]);
            let target = game
                .spawn_actor(Actor::new(GridPos::new(7, 3), 100).unwrap())
                .unwrap();
            match denial {
                "miss" => {
                    game.rules.hit_rules = Some(HitRules {
                        minimum_hit_chance: 0,
                        maximum_hit_chance: 0,
                        ..HitRules::default()
                    })
                }
                "protected_target" => game.map.set_protected(GridPos::new(7, 3), true).unwrap(),
                "protected_source" => game.map.set_protected(GridPos::new(2, 3), true).unwrap(),
                _ => {}
            }
            if denial == "reaction" {
                let prepared = game
                    .prepare_targeted_attack(game.player, 0, target)
                    .unwrap();
                game.resolve_prepared_attack(prepared, ActionOrigin::Reaction)
                    .unwrap();
            } else {
                game.process_player_command(GameCommand::Attack { slot: 0, target });
            }
            assert_eq!(marks(&game, target), 0, "{denial}");
            assert!(bursts(&game).is_empty());
        }
        let mut game = fixture(AttackDelivery::Ranged, true, vec![fracture(false)]);
        game.process_player_command(GameCommand::AttackAt {
            slot: 0,
            target: GridPos::new(7, 3),
        });
        assert!(
            !game
                .events()
                .iter()
                .any(|e| matches!(e, GameEvent::WeaponFractureResolved { .. }))
        );
    }

    #[test]
    fn fracture_obeys_walls_protected_occupants_and_explicit_source_exposure() {
        for expose in [false, true] {
            let mut game = fixture(AttackDelivery::Melee, false, vec![fracture(expose)]);
            let target = actor(&mut game, GridPos::new(3, 3), 100);
            let protected = actor(&mut game, GridPos::new(3, 4), 100);
            game.map.set_protected(GridPos::new(3, 4), true).unwrap();
            game.map
                .set_terrain(GridPos::new(4, 3), Terrain::Wall)
                .unwrap();
            prime(&mut game, target, 2);
            let hp = game.actors.get(game.player).unwrap().integrity();
            hit(&mut game, target);
            assert_eq!(game.actors.get(protected).unwrap().integrity(), 100);
            assert_eq!(
                game.actors.get(game.player).unwrap().integrity(),
                hp - if expose { 6 } else { 0 }
            );
            assert!(
                game.events()
                    .iter()
                    .filter_map(|e| match e {
                        GameEvent::PropagationResolved { cells, .. } => Some(cells),
                        _ => None,
                    })
                    .all(|cells| !cells.iter().any(|cell| cell.position == GridPos::new(4, 3)))
            );
        }
    }

    #[test]
    fn fracture_mark_follows_pushed_actor_but_detonation_keeps_actual_hit_position() {
        let mut game = fixture(
            AttackDelivery::Ranged,
            false,
            vec![
                WeaponEffect::percussion(4, id("recovery")).unwrap(),
                fracture(false),
            ],
        );
        let at = GridPos::new(7, 3);
        let target = actor(&mut game, at, 100);
        hit(&mut game, target);
        assert_eq!(
            game.actors.get(target).unwrap().position(),
            GridPos::new(8, 3)
        );
        assert_eq!(marks(&game, target), 1);
        // Recharge expires while the mark is still live.
        for _ in 0..2 {
            game.process_player_command(GameCommand::Wait);
        }
        prime(&mut game, target, 1);
        hit(&mut game, target);
        assert_eq!(
            game.actors.get(target).unwrap().position(),
            GridPos::new(9, 3)
        );
        assert_eq!(bursts(&game), [GridPos::new(8, 3)]);
        assert_eq!(marks(&game, target), 0);
    }

    #[test]
    fn fracture_inert_marker_validation_rejects_damage_hooks_modifiers_and_wrong_counters() {
        let mut game = fixture(AttackDelivery::Ranged, false, vec![fracture(false)]);
        let valid = marker();
        assert!(valid.is_weapon_charge_marker(3));
        let invalid = [
            StatusDefinition::new(id("mark"), None, valid.stacking(), vec![]).unwrap(),
            StatusDefinition::new(id("mark"), Some(1), valid.stacking(), vec![]).unwrap(),
            StatusDefinition::new(id("mark"), Some(5), StatusStacking::RefreshDuration, vec![])
                .unwrap(),
            StatusDefinition::new(
                id("mark"),
                Some(5),
                StatusStacking::AddStacks {
                    maximum_stacks: 4,
                    refresh_duration: true,
                },
                vec![],
            )
            .unwrap(),
            StatusDefinition::new(
                id("mark"),
                Some(5),
                valid.stacking(),
                vec![StatusHook::new(
                    StatusTrigger::TurnEnd,
                    vec![StatusEffectPrimitive::DealDamage {
                        packet: DamagePacket::new(1, DamageType::Thermal, 0),
                        multiply_by_stacks: true,
                    }],
                )],
            )
            .unwrap(),
            valid
                .clone()
                .with_expiration_transition(StatusTransition::new(id("other"), 1).unwrap()),
            valid
                .clone()
                .with_modifiers([crate::status::StatusModifier::Accuracy { amount: 1 }])
                .unwrap(),
        ];
        for marker in invalid {
            assert!(!marker.is_weapon_charge_marker(3));
            game.rules.statuses = game.rules.statuses.without_id(&id("mark"));
            game.rules.statuses.register(marker).unwrap();
            assert!(matches!(
                GameState::new_with_rules(
                    game.map.clone(),
                    GridPos::new(2, 3),
                    7,
                    game.rules.clone()
                ),
                Err(GameInitError::InvalidWeaponChargeMarker { .. })
            ));
        }
    }

    #[test]
    fn fracture_charges_and_expiry_replay_after_snapshot_without_client_state() {
        let mut game = fixture(AttackDelivery::Ranged, false, vec![fracture(false)]);
        let target = actor(&mut game, GridPos::new(7, 3), 100);
        hit(&mut game, target);
        hit(&mut game, target);
        game.drain_events();
        let bytes = bincode::serialize(&game.snapshot().unwrap()).unwrap();
        let mut restored =
            GameState::from_snapshot(bincode::deserialize(&bytes).unwrap(), game.rules.clone());
        for state in [&mut game, &mut restored] {
            hit(state, target);
        }
        assert_eq!(format!("{game:?}"), format!("{restored:?}"));
        assert_eq!(game.events(), restored.events());
    }

    #[test]
    fn fracture_absorbed_hit_charges_but_a_dead_bearer_cannot_activate() {
        let mut game = fixture(AttackDelivery::Ranged, false, vec![fracture(false)]);
        game.rules.armor_rules = Some(ArmorRules::default());
        let target = game
            .spawn_actor(
                Actor::new(GridPos::new(7, 3), 100)
                    .unwrap()
                    .with_body_profile(BodyProfile::new(100, 0).unwrap().with_base_armor(100)),
            )
            .unwrap();
        hit(&mut game, target);
        assert_eq!(marks(&game, target), 1);
        assert_eq!(game.actors.get(target).unwrap().integrity(), 100);
        let mut game = fixture(AttackDelivery::Ranged, false, vec![fracture(false)]);
        game.rules
            .statuses
            .register(
                StatusDefinition::new(
                    id("riposte"),
                    Some(4),
                    StatusStacking::KeepExisting,
                    vec![StatusHook::new(
                        StatusTrigger::DamageReceived,
                        vec![StatusEffectPrimitive::DealDamageToCounterpart {
                            packet: DamagePacket::new(1000, DamageType::Thermal, 0),
                            multiply_by_stacks: false,
                        }],
                    )],
                )
                .unwrap(),
            )
            .unwrap();
        let target = actor(&mut game, GridPos::new(7, 3), 100);
        prime(&mut game, target, 2);
        game.apply_status_to(
            None,
            target,
            &ApplyStatusEffect::new(id("riposte"), 1).unwrap(),
        )
        .unwrap();
        hit(&mut game, target);
        assert!(bursts(&game).is_empty());
    }
}
