use super::*;
use crate::game::StatusRemovalReason;

impl GameState {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn apply_weapon_catalysis(
        &mut self,
        source: EntityId,
        target: EntityId,
        at: GridPos,
        status: &StatusId,
        effect: &RadialDamageEffect,
        affects_source: bool,
        weapon_effect: Option<(WeaponId, usize)>,
    ) -> bool {
        if self.map.is_protected(at)
            || self
                .actors
                .get(source)
                .is_none_or(|actor| !actor.is_alive() || self.map.is_protected(actor.position()))
        {
            return false;
        }
        if let Some(actor) = self.actors.get_mut(target) {
            if self.map.is_protected(actor.position()) || !actor.remove_status(status) {
                return false;
            }
            self.events.push(GameEvent::StatusRemoved {
                target,
                status: status.clone(),
                reason: StatusRemovalReason::Consumed,
            });
        }
        // A killed target's fuel was captured by the attack resolver. Consume
        // without running natural-expiration transitions or cleansing hooks.
        self.events.push(GameEvent::StatusCatalyzed {
            source,
            target,
            at,
            status: status.clone(),
        });
        self.apply_radial_damage_excluding(
            Some(source),
            at,
            effect,
            (!affects_source).then_some(source),
            weapon_effect,
        );
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::{ConeAttack, DamageType, HitRules};
    use crate::effects::DamageFalloff;
    use crate::stats::{BodyProfile, DisplacementProfile};
    use crate::status::{StatusHook, StatusStacking, StatusTransition};
    use crate::weapon::WeaponCatalog;
    use crate::world::{NeighborMode, TerrainPropagationPolicy};

    fn id(name: &str) -> StatusId {
        format!("test:{name}").parse().unwrap()
    }

    fn catalyst(affects_source: bool) -> WeaponEffect {
        WeaponEffect::catalysis(
            id("burning"),
            RadialDamageEffect {
                maximum_cost: 1,
                neighbor_mode: NeighborMode::CardinalAndDiagonal,
                propagation_policy: TerrainPropagationPolicy::blocked_by_walls(1),
                damage: DamagePacket::new(6, DamageType::Thermal, 0),
                falloff: DamageFalloff::None,
            },
            affects_source,
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
                WeaponDefinition::new(id("catalyst"), "name".into(), "description".into(), attack)
                    .unwrap()
                    .with_effects(effects),
            )
            .unwrap();
        let mut statuses = StatusCatalog::default();
        for (name, damage, kind) in [
            ("burning", 2, DamageType::Thermal),
            ("corroded", 1, DamageType::Chemical),
        ] {
            let mut status = StatusDefinition::new(
                id(name),
                Some(4),
                StatusStacking::RefreshDuration,
                vec![StatusHook::new(
                    StatusTrigger::TurnEnd,
                    vec![StatusEffectPrimitive::DealDamage {
                        packet: DamagePacket::new(damage, kind, 0),
                        multiply_by_stacks: false,
                    }],
                )],
            )
            .unwrap();
            if name == "burning" {
                status = status
                    .with_expiration_transition(StatusTransition::new(id("afterburn"), 1).unwrap());
            }
            statuses.register(status).unwrap();
        }
        for name in ["afterburn", "recovery"] {
            statuses
                .register(
                    StatusDefinition::new(id(name), Some(3), StatusStacking::KeepExisting, vec![])
                        .unwrap(),
                )
                .unwrap();
        }
        GameState::new_with_rules(
            Map::filled(18, 12, Terrain::Floor).unwrap(),
            GridPos::new(2, 3),
            7,
            GameRules {
                weapons,
                statuses,
                player_base_attacks: vec![attack],
                player_weapon_slots: vec![id("slot")],
                player_starting_weapons: vec![id("catalyst")],
                player_starting_equipment: vec![Some(id("catalyst"))],
                ..GameRules::default()
            },
        )
        .unwrap()
    }

    fn actor(game: &mut GameState, at: GridPos, hp: u16) -> EntityId {
        game.spawn_actor(
            Actor::new(at, hp).unwrap().with_body_profile(
                BodyProfile::new(hp, 0)
                    .unwrap()
                    .with_displacement_profile(DisplacementProfile::new(10_000, 0).unwrap()),
            ),
        )
        .unwrap()
    }

    fn apply(game: &mut GameState, target: EntityId, name: &str) {
        game.apply_status_to(
            Some(game.player),
            target,
            &ApplyStatusEffect::new(id(name), 1).unwrap(),
        )
        .unwrap();
    }

    fn centers(game: &GameState) -> Vec<GridPos> {
        game.events()
            .iter()
            .filter_map(|event| match event {
                GameEvent::StatusCatalyzed { at, .. } => Some(*at),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn catalysis_consumes_only_its_fuel_in_melee_and_ranged_without_expiration_transition() {
        for delivery in [AttackDelivery::Melee, AttackDelivery::Ranged] {
            let mut game = fixture(delivery, false, vec![catalyst(false)]);
            let at = GridPos::new(
                if delivery == AttackDelivery::Melee {
                    3
                } else {
                    7
                },
                3,
            );
            let target = actor(&mut game, at, 30);
            let neighbor = actor(&mut game, GridPos::new(at.x, 4), 30);
            let beyond = actor(&mut game, GridPos::new(at.x, 5), 30);
            apply(&mut game, target, "burning");
            apply(&mut game, target, "corroded");
            game.drain_events();
            let hp = game.actors.get(game.player).unwrap().integrity();
            game.process_player_command(GameCommand::Attack { slot: 0, target });
            let victim = game.actors.get(target).unwrap();
            assert_eq!(victim.integrity(), 18); // 5 primary + 6 burst + 1 other DOT.
            assert!(victim.status(&id("burning")).is_none());
            assert!(victim.status(&id("afterburn")).is_none());
            assert_eq!(
                victim.status(&id("corroded")).unwrap().remaining_turns,
                Some(3)
            );
            assert_eq!(game.actors.get(neighbor).unwrap().integrity(), 24);
            assert_eq!(game.actors.get(beyond).unwrap().integrity(), 30);
            assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
            assert_eq!(centers(&game), [at]);
            assert_eq!(game.events().iter().filter(|e| matches!(e, GameEvent::StatusRemoved { status, reason: StatusRemovalReason::Consumed, .. } if status == &id("burning"))).count(), 1);
        }
    }

    #[test]
    fn catalysis_killing_hit_keeps_the_captured_impact() {
        let mut game = fixture(AttackDelivery::Ranged, false, vec![catalyst(false)]);
        let at = GridPos::new(7, 3);
        let target = actor(&mut game, at, 5);
        let neighbor = actor(&mut game, GridPos::new(7, 4), 30);
        apply(&mut game, target, "burning");
        game.process_player_command(GameCommand::Attack { slot: 0, target });
        assert!(game.actors.get(target).is_none());
        assert_eq!(centers(&game), [at]);
        assert_eq!(game.actors.get(neighbor).unwrap().integrity(), 24);
    }

    #[test]
    fn catalysis_requires_existing_fuel_and_a_normal_actual_hit() {
        for denial in [
            "unlit",
            "new_brand",
            "miss",
            "reaction",
            "protected_target",
            "protected_source",
        ] {
            let mut effects = vec![catalyst(false)];
            if denial == "new_brand" {
                effects.push(
                    WeaponEffect::apply_status(
                        ApplyStatusEffect::new(id("burning"), 1).unwrap(),
                        WeaponEffectTrigger::OnHit,
                    )
                    .unwrap(),
                );
            }
            let mut game = fixture(AttackDelivery::Ranged, false, effects);
            let target = actor(&mut game, GridPos::new(7, 3), 30);
            if !matches!(denial, "unlit" | "new_brand") {
                apply(&mut game, target, "burning");
            }
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
            assert!(centers(&game).is_empty(), "{denial}");
            if denial != "unlit" {
                assert!(
                    game.actors
                        .get(target)
                        .unwrap()
                        .status(&id("burning"))
                        .is_some(),
                    "{denial}"
                );
            }
        }
        let mut empty = fixture(AttackDelivery::Ranged, true, vec![catalyst(false)]);
        empty.process_player_command(GameCommand::AttackAt {
            slot: 0,
            target: GridPos::new(7, 3),
        });
        assert!(centers(&empty).is_empty());
    }

    #[test]
    fn catalysis_multitarget_and_duplicate_properties_still_burst_once() {
        for aim in [7, 9] {
            let mut game = fixture(
                AttackDelivery::Ranged,
                true,
                vec![catalyst(false), catalyst(false)],
            );
            let first = actor(&mut game, GridPos::new(5, 3), 100);
            let last = actor(&mut game, GridPos::new(7, 3), 100);
            for target in [first, last] {
                apply(&mut game, target, "burning");
            }
            game.process_player_command(GameCommand::AttackAt {
                slot: 0,
                target: GridPos::new(aim, 3),
            });
            assert_eq!(
                centers(&game),
                [GridPos::new(if aim == 7 { 7 } else { 5 }, 3)]
            );
            let untouched_fuel = if aim == 7 { first } else { last };
            assert!(
                game.actors
                    .get(untouched_fuel)
                    .unwrap()
                    .status(&id("burning"))
                    .is_some()
            );
        }
    }

    #[test]
    fn catalysis_obeys_walls_protection_and_explicit_bearer_exposure() {
        for expose in [false, true] {
            let mut game = fixture(AttackDelivery::Melee, false, vec![catalyst(expose)]);
            let target = actor(&mut game, GridPos::new(3, 3), 30);
            let protected = actor(&mut game, GridPos::new(3, 4), 30);
            game.map.set_protected(GridPos::new(3, 4), true).unwrap();
            game.map
                .set_terrain(GridPos::new(4, 3), Terrain::Wall)
                .unwrap();
            apply(&mut game, target, "burning");
            let hp = game.actors.get(game.player).unwrap().integrity();
            game.process_player_command(GameCommand::Attack { slot: 0, target });
            assert_eq!(game.actors.get(protected).unwrap().integrity(), 30);
            assert_eq!(
                game.actors.get(game.player).unwrap().integrity(),
                hp - if expose { 6 } else { 0 }
            );
            let cells = game
                .events()
                .iter()
                .find_map(|e| match e {
                    GameEvent::PropagationResolved { cells, .. } => Some(cells),
                    _ => None,
                })
                .unwrap();
            assert!(!cells.iter().any(|cell| cell.position == GridPos::new(4, 3)));
        }
    }

    #[test]
    fn catalysis_after_percussion_consumes_the_moved_target_but_keeps_the_impact_center() {
        let mut game = fixture(
            AttackDelivery::Ranged,
            false,
            vec![
                WeaponEffect::percussion(4, id("recovery")).unwrap(),
                catalyst(false),
            ],
        );
        let at = GridPos::new(7, 3);
        let target = actor(&mut game, at, 30);
        apply(&mut game, target, "burning");
        game.process_player_command(GameCommand::Attack { slot: 0, target });
        assert_eq!(
            game.actors.get(target).unwrap().position(),
            GridPos::new(8, 3)
        );
        assert!(
            game.actors
                .get(target)
                .unwrap()
                .status(&id("burning"))
                .is_none()
        );
        assert_eq!(centers(&game), [at]);
    }

    #[test]
    fn catalysis_absorbed_hit_still_uses_fuel_but_dead_bearer_cannot_activate() {
        use crate::combat::ArmorRules;
        let mut game = fixture(AttackDelivery::Ranged, false, vec![catalyst(false)]);
        game.rules.armor_rules = Some(ArmorRules::default());
        let target = game
            .spawn_actor(
                Actor::new(GridPos::new(7, 3), 30)
                    .unwrap()
                    .with_body_profile(BodyProfile::new(30, 0).unwrap().with_base_armor(100)),
            )
            .unwrap();
        apply(&mut game, target, "burning");
        game.process_player_command(GameCommand::Attack { slot: 0, target });
        assert_eq!(centers(&game), [GridPos::new(7, 3)]);
        assert!(
            game.actors
                .get(target)
                .unwrap()
                .status(&id("burning"))
                .is_none()
        );

        let mut game = fixture(AttackDelivery::Ranged, false, vec![catalyst(false)]);
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
        let target = actor(&mut game, GridPos::new(7, 3), 50);
        apply(&mut game, target, "burning");
        apply(&mut game, target, "riposte");
        game.process_player_command(GameCommand::Attack { slot: 0, target });
        assert!(game.actors.get(game.player).is_none());
        assert!(centers(&game).is_empty());
        assert!(
            game.actors
                .get(target)
                .unwrap()
                .status(&id("burning"))
                .is_some()
        );
    }

    #[test]
    fn catalysis_replays_with_prepared_fuel_without_new_saved_state() {
        let mut game = fixture(AttackDelivery::Ranged, false, vec![catalyst(false)]);
        let target = actor(&mut game, GridPos::new(7, 3), 50);
        apply(&mut game, target, "burning");
        game.drain_events();
        let bytes = bincode::serialize(&game.snapshot().unwrap()).unwrap();
        let mut restored =
            GameState::from_snapshot(bincode::deserialize(&bytes).unwrap(), game.rules.clone());
        for state in [&mut game, &mut restored] {
            state.process_player_command(GameCommand::Attack { slot: 0, target });
        }
        assert_eq!(format!("{game:?}"), format!("{restored:?}"));
        assert_eq!(game.events(), restored.events());
    }
}
