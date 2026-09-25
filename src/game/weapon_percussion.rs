use super::*;

pub(super) fn continuation_cell(origin: GridPos, from: GridPos, step: u16) -> Option<GridPos> {
    let dx = i64::from(from.x) - i64::from(origin.x);
    let dy = i64::from(from.y) - i64::from(origin.y);
    let major = dx.abs().max(dy.abs());
    if major == 0 {
        return None;
    }
    let offset = |delta: i64| (delta.abs() * i64::from(step) + major / 2) / major * delta.signum();
    Some(GridPos::new(
        i32::try_from(i64::from(from.x) + offset(dx)).ok()?,
        i32::try_from(i64::from(from.y) + offset(dy)).ok()?,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::{AttackDelivery, ConeAttack, DamageType, HitRules};
    use crate::stats::{BodyProfile, DisplacementProfile};
    use crate::status::StatusStacking;
    use crate::weapon::WeaponCatalog;

    fn recovery() -> StatusId {
        "test:percussion_recovery".parse().unwrap()
    }

    fn fixture(delivery: AttackDelivery, cone: bool) -> GameState {
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
        let ids: Vec<WeaponId> = ["test:percussion", "test:percussion_other"]
            .map(|id| id.parse().unwrap())
            .into();
        for id in &ids {
            weapons
                .register(
                    WeaponDefinition::new(id.clone(), "name".into(), "description".into(), attack)
                        .unwrap()
                        .with_effects([WeaponEffect::percussion(4, recovery()).unwrap()]),
                )
                .unwrap();
        }
        let mut statuses = StatusCatalog::default();
        statuses
            .register(
                StatusDefinition::new(recovery(), Some(3), StatusStacking::KeepExisting, vec![])
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
                player_weapon_slots: vec![
                    "test:slot_a".parse().unwrap(),
                    "test:slot_b".parse().unwrap(),
                ],
                player_starting_weapons: ids.clone(),
                player_starting_equipment: ids.into_iter().map(Some).collect(),
                ..GameRules::default()
            },
        )
        .unwrap()
    }

    fn actor(
        game: &mut GameState,
        at: GridPos,
        mass: u32,
        anchoring: u16,
        fixed: bool,
        hp: u16,
    ) -> EntityId {
        let displacement = DisplacementProfile::new(mass, anchoring).unwrap();
        let body = BodyProfile::new(hp, 0)
            .unwrap()
            .with_displacement_profile(if fixed {
                displacement.fixed()
            } else {
                displacement
            });
        game.spawn_actor(Actor::new(at, hp).unwrap().with_body_profile(body))
            .unwrap()
    }

    fn pushes(game: &GameState) -> Vec<(EntityId, ForcedMovementOutcome)> {
        game.events()
            .iter()
            .filter_map(|event| match event {
                GameEvent::ForcedMovementResolved {
                    target, outcome, ..
                } => Some((*target, *outcome)),
                _ => None,
            })
            .collect()
    }

    fn remaining(game: &GameState) -> Option<u16> {
        game.actors
            .get(game.player)
            .unwrap()
            .status(&recovery())
            .and_then(|status| status.remaining_turns)
    }

    #[test]
    fn percussion_pushes_melee_and_ranged_without_extra_damage_or_base_changes() {
        for delivery in [AttackDelivery::Melee, AttackDelivery::Ranged] {
            let mut game = fixture(delivery, false);
            let from = GridPos::new(
                if delivery == AttackDelivery::Melee {
                    3
                } else {
                    7
                },
                3,
            );
            let target = actor(&mut game, from, 10_000, 0, false, 30);
            game.process_player_command(GameCommand::Attack { slot: 0, target });
            assert_eq!(
                game.actors.get(target).unwrap().position(),
                GridPos::new(from.x + 1, from.y)
            );
            assert_eq!(game.actors.get(target).unwrap().integrity(), 25);
            assert_eq!(pushes(&game), [(target, ForcedMovementOutcome::Moved)]);
            assert_eq!(remaining(&game), Some(2));
        }
    }

    #[test]
    fn percussion_uses_mass_anchoring_fixed_and_unknown_body_profiles() {
        for (mass, anchoring, fixed, unknown, outcome) in [
            (40_000, 0, false, false, ForcedMovementOutcome::Moved),
            (40_001, 0, false, false, ForcedMovementOutcome::Resisted),
            (10_000, 4, false, false, ForcedMovementOutcome::Resisted),
            (10_000, 0, true, false, ForcedMovementOutcome::Fixed),
            (10_000, 0, false, true, ForcedMovementOutcome::Incompatible),
        ] {
            let mut game = fixture(AttackDelivery::Ranged, false);
            let at = GridPos::new(7, 3);
            let target = if unknown {
                game.spawn_actor(Actor::new(at, 30).unwrap()).unwrap()
            } else {
                actor(&mut game, at, mass, anchoring, fixed, 30)
            };
            game.process_player_command(GameCommand::Attack { slot: 0, target });
            assert_eq!(pushes(&game), [(target, outcome)]);
            assert_eq!(remaining(&game), Some(2));
            assert_eq!(game.actors.get(target).unwrap().integrity(), 25);
        }
    }

    #[test]
    fn percussion_stops_at_walls_doors_occupants_protection_and_map_edges() {
        for obstacle in ["wall", "door", "actor", "protected", "edge"] {
            let mut game = fixture(AttackDelivery::Ranged, false);
            let from = GridPos::new(if obstacle == "edge" { 17 } else { 7 }, 3);
            let target = actor(&mut game, from, 10_000, 0, false, 30);
            let to = GridPos::new(from.x + 1, 3);
            match obstacle {
                "wall" => game.map.set_terrain(to, Terrain::Wall).unwrap(),
                "door" => game
                    .map
                    .set_terrain(to, Terrain::Door(DoorState::Closed))
                    .unwrap(),
                "actor" => {
                    actor(&mut game, to, 10_000, 0, false, 30);
                }
                "protected" => game.map.set_protected(to, true).unwrap(),
                _ => {}
            }
            game.apply_weapon_percussion(game.player, target, GridPos::new(2, 3), 4, &recovery());
            assert_eq!(
                game.actors.get(target).unwrap().position(),
                from,
                "{obstacle}"
            );
            assert_eq!(game.actors.get(target).unwrap().integrity(), 30);
            assert_eq!(pushes(&game), [(target, ForcedMovementOutcome::Blocked)]);
            assert_eq!(remaining(&game), Some(3));
        }
    }

    #[test]
    fn percussion_cannot_squeeze_through_closed_diagonal_corners() {
        for closed in [false, true] {
            let mut game = fixture(AttackDelivery::Ranged, false);
            let target = actor(&mut game, GridPos::new(3, 4), 10_000, 0, false, 30);
            game.map
                .set_terrain(GridPos::new(4, 4), Terrain::Wall)
                .unwrap();
            if closed {
                actor(&mut game, GridPos::new(3, 5), 10_000, 0, false, 30);
            }
            game.process_player_command(GameCommand::Attack { slot: 0, target });
            assert_eq!(
                game.actors.get(target).unwrap().position(),
                if closed {
                    GridPos::new(3, 4)
                } else {
                    GridPos::new(4, 5)
                }
            );
        }
    }

    #[test]
    fn percussion_oblique_direction_is_symmetric_and_not_snapped_to_a_diagonal() {
        for (dx, dy, expected) in [(5, 2, (1, 0)), (5, 3, (1, 1)), (2, 5, (0, 1))] {
            for sx in [-1, 1] {
                for sy in [-1, 1] {
                    let from = GridPos::new(dx * sx, dy * sy);
                    assert_eq!(
                        continuation_cell(GridPos::new(0, 0), from, 1),
                        Some(GridPos::new(
                            from.x + expected.0 * sx,
                            from.y + expected.1 * sy
                        ))
                    );
                }
            }
        }
        assert_eq!(
            continuation_cell(GridPos::new(0, 0), GridPos::new(i32::MAX, 0), 1),
            None
        );
        assert_eq!(
            continuation_cell(GridPos::new(0, 0), GridPos::new(0, 0), 1),
            None
        );
        let mut game = fixture(AttackDelivery::Ranged, false);
        let target = actor(&mut game, GridPos::new(7, 5), 10_000, 0, false, 30);
        game.process_player_command(GameCommand::Attack { slot: 0, target });
        assert_eq!(
            game.actors.get(target).unwrap().position(),
            GridPos::new(8, 5)
        );
    }

    #[test]
    fn percussion_recovery_is_shared_between_weapons_and_never_refreshed_by_hits() {
        let mut game = fixture(AttackDelivery::Ranged, false);
        let target = actor(&mut game, GridPos::new(7, 3), 10_000, 0, false, 100);
        for (slot, expected_x, turns) in [
            (0, 8, Some(2)),
            (1, 8, Some(1)),
            (0, 8, None),
            (1, 9, Some(2)),
        ] {
            game.drain_events();
            game.process_player_command(GameCommand::Attack { slot, target });
            assert_eq!(game.actors.get(target).unwrap().position().x, expected_x);
            assert_eq!(remaining(&game), turns);
            assert_eq!(pushes(&game).len(), usize::from(turns == Some(2)));
        }
    }

    #[test]
    fn percussion_requires_a_surviving_direct_hit_and_no_reaction_or_protected_actor() {
        for denial in [
            "miss",
            "kill",
            "reaction",
            "protected_target",
            "protected_source",
        ] {
            let mut game = fixture(AttackDelivery::Ranged, false);
            let target = actor(
                &mut game,
                GridPos::new(7, 3),
                10_000,
                0,
                false,
                if denial == "kill" { 5 } else { 30 },
            );
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
                let attack = game
                    .prepare_targeted_attack(game.player, 0, target)
                    .unwrap();
                game.resolve_prepared_attack(attack, ActionOrigin::Reaction)
                    .unwrap();
            } else {
                game.process_player_command(GameCommand::Attack { slot: 0, target });
            }
            assert!(pushes(&game).is_empty(), "{denial}");
            assert_eq!(remaining(&game), None);
        }
    }

    #[test]
    fn percussion_multitarget_action_pushes_only_the_aimed_survivor_or_first_real_hit() {
        for aim_x in [7, 9, 10] {
            let mut game = fixture(AttackDelivery::Ranged, true);
            let first = actor(&mut game, GridPos::new(5, 3), 10_000, 0, false, 30);
            let last = actor(&mut game, GridPos::new(7, 3), 10_000, 0, false, 30);
            game.process_player_command(GameCommand::AttackAt {
                slot: 0,
                target: GridPos::new(aim_x, 3),
            });
            assert_eq!(
                pushes(&game),
                [(
                    if aim_x == 7 { last } else { first },
                    ForcedMovementOutcome::Moved
                )]
            );
        }
        let mut empty = fixture(AttackDelivery::Ranged, true);
        empty.process_player_command(GameCommand::AttackAt {
            slot: 0,
            target: GridPos::new(9, 3),
        });
        assert!(pushes(&empty).is_empty());
        assert_eq!(remaining(&empty), None);
    }

    #[test]
    fn percussion_recovery_and_moved_actor_replay_after_snapshot() {
        let mut game = fixture(AttackDelivery::Ranged, false);
        let target = actor(&mut game, GridPos::new(7, 3), 10_000, 0, false, 100);
        game.process_player_command(GameCommand::Attack { slot: 0, target });
        game.drain_events();
        let bytes = bincode::serialize(&game.snapshot().unwrap()).unwrap();
        let mut restored =
            GameState::from_snapshot(bincode::deserialize(&bytes).unwrap(), game.rules.clone());
        for state in [&mut game, &mut restored] {
            for _ in 0..3 {
                state.process_player_command(GameCommand::Attack { slot: 1, target });
            }
        }
        assert_eq!(format!("{game:?}"), format!("{restored:?}"));
        assert_eq!(game.events(), restored.events());
    }
}

pub(super) fn push_step_open(
    map: &Map,
    actors: &ActorRegistry,
    from: GridPos,
    to: GridPos,
) -> bool {
    if from.x == to.x || from.y == to.y {
        return true;
    }
    // Never squeeze an actor through two blocked corners, including other
    // actors/protected cells; this is movement, not a projectile line of sight.
    let open = |at| map.is_walkable(at) && !map.is_protected(at) && actors.entity_at(at).is_none();
    open(GridPos::new(from.x, to.y)) || open(GridPos::new(to.x, from.y))
}

impl GameState {
    pub(super) fn apply_weapon_percussion(
        &mut self,
        source: EntityId,
        target: EntityId,
        origin: GridPos,
        force: u16,
        recovery: &StatusId,
    ) {
        let Some(bearer) = self.actors.get(source) else {
            return;
        };
        if !bearer.is_alive()
            || bearer.status(recovery).is_some()
            || self.map.is_protected(bearer.position())
        {
            return;
        }
        let Some(from) = self.actors.get(target).map(Actor::position) else {
            return;
        };
        if self.map.is_protected(from) || source == target {
            return;
        }
        // An attempted push spends readiness even when blocked or resisted.
        let status = ApplyStatusEffect::new(recovery.clone(), 1).expect("one recovery stack");
        let _ = self.apply_status_to(Some(source), source, &status);
        // A blocking status family must not grant repeated free attempts.
        if self
            .actors
            .get(source)
            .is_none_or(|actor| actor.status(recovery).is_none())
        {
            return;
        }
        if let Some(to) = self.resolve_displacement(source, target, origin, force, 1, true) {
            self.events.push(GameEvent::WeaponImpulseResolved {
                source,
                target,
                origin,
                from,
                to,
            });
        }
    }
}
