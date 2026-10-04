use super::*;
use crate::combat::{ArmorRules, ConeAttack, DamageType, HitRules};
use crate::effects::DamageFalloff;
use crate::stats::BodyProfile;
use crate::weapon::{WeaponCatalog, WeaponEffectOrigin};
use crate::world::{NeighborMode, TerrainPropagationPolicy};

fn area_game(
    delivery: AttackDelivery,
    origin: WeaponEffectOrigin,
    trigger: WeaponEffectTrigger,
    affects_source: bool,
    cone: bool,
) -> GameState {
    let attack = AttackProfile::new(
        if delivery == AttackDelivery::Melee {
            1
        } else {
            10
        },
        DistanceMetric::Chebyshev,
        true,
        DamageType::Kinetic,
        3,
        0,
    )
    .with_delivery(delivery);
    let attack = if cone {
        attack.with_area(AttackArea::Cone(ConeAttack::new(1, 1, 1).unwrap()))
    } else {
        attack
    };
    let effect = WeaponEffect::radial_damage(
        RadialDamageEffect {
            maximum_cost: 1,
            neighbor_mode: NeighborMode::CardinalAndDiagonal,
            propagation_policy: TerrainPropagationPolicy::blocked_by_walls(1),
            damage: DamagePacket::new(7, DamageType::Electrical, 0),
            falloff: DamageFalloff::None,
        },
        trigger,
        origin,
        affects_source,
    )
    .unwrap();
    let weapon: WeaponId = "core:test_secondary_zone".parse().unwrap();
    let mut weapons = WeaponCatalog::default();
    weapons
        .register(
            WeaponDefinition::new(
                weapon.clone(),
                "test.name".into(),
                "test.description".into(),
                attack,
            )
            .unwrap()
            .with_effects([effect]),
        )
        .unwrap();
    GameState::new_with_rules(
        Map::filled(14, 7, Terrain::Floor).unwrap(),
        GridPos::new(2, 3),
        7,
        GameRules {
            player_base_attacks: vec![attack],
            player_weapon_slots: vec!["core:test_weapon".parse().unwrap()],
            player_starting_weapons: vec![weapon.clone()],
            player_starting_equipment: vec![Some(weapon)],
            weapons,
            ..GameRules::default()
        },
    )
    .unwrap()
}

fn actor(game: &mut GameState, position: GridPos, integrity: u16) -> EntityId {
    game.spawn_actor(Actor::new(position, integrity).unwrap())
        .unwrap()
}

fn centers(game: &GameState) -> Vec<GridPos> {
    game.events()
        .iter()
        .filter_map(|event| match event {
            GameEvent::PropagationResolved { origin, .. } => Some(*origin),
            _ => None,
        })
        .collect()
}

#[test]
fn three_different_dots_tick_independently_without_stacking_on_reapplication() {
    use crate::status::{
        StatusDefinition, StatusEffectPrimitive, StatusHook, StatusStacking, StatusTrigger,
    };
    let mut game = area_game(
        AttackDelivery::Melee,
        WeaponEffectOrigin::Impact,
        WeaponEffectTrigger::OnHit,
        false,
        false,
    );
    let target = actor(&mut game, GridPos::new(3, 3), 100);
    for (index, kind) in [
        DamageType::Thermal,
        DamageType::Chemical,
        DamageType::Radiation,
    ]
    .into_iter()
    .enumerate()
    {
        let id: crate::content::ContentId = format!("core:test_dot_{index}").parse().unwrap();
        game.rules
            .statuses
            .register(
                StatusDefinition::new(
                    id.clone(),
                    Some(3),
                    StatusStacking::RefreshDuration,
                    vec![StatusHook::new(
                        StatusTrigger::TurnEnd,
                        vec![StatusEffectPrimitive::DealDamage {
                            packet: DamagePacket::new(2, kind, 0),
                            multiply_by_stacks: true,
                        }],
                    )],
                )
                .unwrap(),
            )
            .unwrap();
        for _ in 0..5 {
            game.apply_status_to(
                Some(game.player),
                target,
                &ApplyStatusEffect::new(id.clone(), 7).unwrap(),
            )
            .unwrap();
        }
    }
    assert_eq!(game.actors.get(target).unwrap().statuses().count(), 3);
    assert!(
        game.actors
            .get(target)
            .unwrap()
            .statuses()
            .all(|status| status.stacks == 1)
    );
    for turn in 1..=3 {
        assert_eq!(
            game.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(game.actors.get(target).unwrap().integrity(), 100 - 6 * turn);
    }
    assert_eq!(game.actors.get(target).unwrap().statuses().count(), 0);
}

#[test]
fn melee_and_ranged_secondary_zones_share_the_impact_origin() {
    for (delivery, target_x) in [(AttackDelivery::Melee, 3), (AttackDelivery::Ranged, 8)] {
        let mut game = area_game(
            delivery,
            WeaponEffectOrigin::Impact,
            WeaponEffectTrigger::OnHit,
            false,
            false,
        );
        let target_position = GridPos::new(target_x, 3);
        let target = actor(&mut game, target_position, 30);
        let neighbor = actor(&mut game, GridPos::new(target_x + 1, 3), 30);
        let by_bearer = actor(&mut game, GridPos::new(1, 3), 30);
        let player_hp = game.actors.get(game.player).unwrap().integrity();
        game.drain_events();
        assert_eq!(
            game.process_player_command(GameCommand::Attack { slot: 0, target }),
            CommandOutcome::Applied
        );
        assert_eq!(centers(&game), vec![target_position]);
        assert_eq!(game.actors.get(target).unwrap().integrity(), 20);
        assert_eq!(game.actors.get(neighbor).unwrap().integrity(), 23);
        assert_eq!(game.actors.get(by_bearer).unwrap().integrity(), 30);
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), player_hp);
        // The secondary damage does not rewrite the underlying attack.
        assert_eq!(
            game.rules
                .weapons
                .iter()
                .next()
                .unwrap()
                .1
                .attack()
                .damage()
                .primary_component()
                .amount,
            3
        );
    }
}

#[test]
fn bearer_center_requires_an_explicit_origin_for_both_deliveries() {
    for (delivery, target_x) in [(AttackDelivery::Melee, 3), (AttackDelivery::Ranged, 8)] {
        let mut game = area_game(
            delivery,
            WeaponEffectOrigin::Bearer,
            WeaponEffectTrigger::OnHit,
            false,
            false,
        );
        let target = actor(&mut game, GridPos::new(target_x, 3), 30);
        let by_bearer = actor(&mut game, GridPos::new(1, 3), 30);
        let by_target = actor(&mut game, GridPos::new(target_x + 1, 3), 30);
        game.drain_events();
        game.process_player_command(GameCommand::Attack { slot: 0, target });
        assert_eq!(centers(&game), vec![GridPos::new(2, 3)]);
        assert_eq!(game.actors.get(by_bearer).unwrap().integrity(), 23);
        assert_eq!(game.actors.get(by_target).unwrap().integrity(), 30);
    }
}

#[test]
fn missed_attacks_never_create_a_secondary_zone_even_around_the_bearer() {
    for origin in [WeaponEffectOrigin::Impact, WeaponEffectOrigin::Bearer] {
        let mut game = area_game(
            AttackDelivery::Ranged,
            origin,
            WeaponEffectTrigger::OnHit,
            false,
            false,
        );
        game.rules.hit_rules = Some(HitRules {
            minimum_hit_chance: 0,
            maximum_hit_chance: 0,
            ..HitRules::default()
        });
        let target = actor(&mut game, GridPos::new(8, 3), 30);
        game.drain_events();
        assert_eq!(
            game.process_player_command(GameCommand::Attack { slot: 0, target }),
            CommandOutcome::Applied
        );
        assert!(centers(&game).is_empty());
        assert_eq!(game.actors.get(target).unwrap().integrity(), 30);
    }
}

#[test]
fn secondary_zone_keeps_the_impact_cell_after_a_lethal_hit() {
    let mut game = area_game(
        AttackDelivery::Ranged,
        WeaponEffectOrigin::Impact,
        WeaponEffectTrigger::OnHit,
        false,
        false,
    );
    let target = actor(&mut game, GridPos::new(8, 3), 3);
    let neighbor = actor(&mut game, GridPos::new(9, 3), 30);
    game.drain_events();
    game.process_player_command(GameCommand::Attack { slot: 0, target });
    assert!(game.actors.get(target).is_none());
    assert_eq!(centers(&game), vec![GridPos::new(8, 3)]);
    assert_eq!(game.actors.get(neighbor).unwrap().integrity(), 23);
}

#[test]
fn secondary_zone_respects_the_distinction_between_hit_and_damage() {
    for trigger in [WeaponEffectTrigger::OnHit, WeaponEffectTrigger::OnDamage] {
        let mut game = area_game(
            AttackDelivery::Ranged,
            WeaponEffectOrigin::Impact,
            trigger,
            false,
            false,
        );
        game.rules.armor_rules = Some(ArmorRules::default());
        let target = game
            .spawn_actor(
                Actor::new(GridPos::new(8, 3), 30)
                    .unwrap()
                    .with_body_profile(BodyProfile::new(30, 0).unwrap().with_base_armor(100)),
            )
            .unwrap();
        game.drain_events();
        game.process_player_command(GameCommand::Attack { slot: 0, target });
        assert_eq!(
            centers(&game).len(),
            usize::from(trigger == WeaponEffectTrigger::OnHit)
        );
    }
}

#[test]
fn a_multitarget_attack_emits_one_zone_at_the_qualifying_aimed_target() {
    let mut game = area_game(
        AttackDelivery::Ranged,
        WeaponEffectOrigin::Impact,
        WeaponEffectTrigger::OnHit,
        false,
        true,
    );
    let target = actor(&mut game, GridPos::new(8, 3), 30);
    actor(&mut game, GridPos::new(6, 3), 30);
    actor(&mut game, GridPos::new(7, 2), 30);
    game.drain_events();
    game.process_player_command(GameCommand::Attack { slot: 0, target });
    assert_eq!(centers(&game), vec![GridPos::new(8, 3)]);
}

#[test]
fn an_empty_aimed_cell_uses_a_real_hit_and_empty_attacks_emit_nothing() {
    for has_target in [false, true] {
        let mut game = area_game(
            AttackDelivery::Ranged,
            WeaponEffectOrigin::Impact,
            WeaponEffectTrigger::OnHit,
            false,
            true,
        );
        if has_target {
            actor(&mut game, GridPos::new(6, 3), 30);
        }
        game.drain_events();
        assert_eq!(
            game.process_player_command(GameCommand::AttackAt {
                slot: 0,
                target: GridPos::new(8, 3)
            }),
            CommandOutcome::Applied
        );
        assert_eq!(
            centers(&game),
            if has_target {
                vec![GridPos::new(6, 3)]
            } else {
                vec![]
            }
        );
    }
}

#[test]
fn reaction_attacks_do_not_trigger_secondary_zones() {
    let mut game = area_game(
        AttackDelivery::Melee,
        WeaponEffectOrigin::Impact,
        WeaponEffectTrigger::OnHit,
        false,
        false,
    );
    let target = actor(&mut game, GridPos::new(3, 3), 30);
    let prepared = game
        .prepare_targeted_attack(game.player, 0, target)
        .unwrap();
    game.drain_events();
    game.resolve_prepared_attack(prepared, ActionOrigin::Reaction)
        .unwrap();
    assert!(centers(&game).is_empty());
    assert_eq!(game.actors.get(target).unwrap().integrity(), 27);
}

#[test]
fn source_exposure_is_explicit_and_protected_ground_still_blocks_damage() {
    for affects_source in [false, true] {
        let mut game = area_game(
            AttackDelivery::Melee,
            WeaponEffectOrigin::Impact,
            WeaponEffectTrigger::OnHit,
            affects_source,
            false,
        );
        let target = actor(&mut game, GridPos::new(3, 3), 30);
        let protected_position = GridPos::new(4, 3);
        let protected = actor(&mut game, protected_position, 30);
        game.map.set_protected(protected_position, true).unwrap();
        let player_hp = game.actors.get(game.player).unwrap().integrity();
        game.process_player_command(GameCommand::Attack { slot: 0, target });
        assert_eq!(
            game.actors.get(game.player).unwrap().integrity(),
            player_hp - if affects_source { 7 } else { 0 }
        );
        assert_eq!(game.actors.get(protected).unwrap().integrity(), 30);
    }
}

#[test]
fn secondary_zone_replays_identically_after_a_snapshot_round_trip() {
    let mut game = area_game(
        AttackDelivery::Ranged,
        WeaponEffectOrigin::Impact,
        WeaponEffectTrigger::OnHit,
        false,
        false,
    );
    let target = actor(&mut game, GridPos::new(8, 3), 30);
    game.drain_events();
    let encoded = bincode::serialize(&game.snapshot().unwrap()).unwrap();
    let mut restored =
        GameState::from_snapshot(bincode::deserialize(&encoded).unwrap(), game.rules.clone());
    for run in [&mut game, &mut restored] {
        assert_eq!(
            run.process_player_command(GameCommand::Attack { slot: 0, target }),
            CommandOutcome::Applied
        );
    }
    assert_eq!(format!("{game:?}"), format!("{restored:?}"));
    assert_eq!(game.events(), restored.events());
}

#[test]
fn knockback_does_not_move_the_captured_secondary_impact() {
    let mut game = area_game(
        AttackDelivery::Melee,
        WeaponEffectOrigin::Impact,
        WeaponEffectTrigger::OnHit,
        false,
        false,
    );
    game.rules.physical_rules = Some(crate::stats::PhysicalRules::default());
    let target = game
        .spawn_actor(
            Actor::new(GridPos::new(3, 3), 100)
                .unwrap()
                .with_body_profile(BodyProfile::new(100, 0).unwrap().with_displacement_profile(
                    crate::stats::DisplacementProfile::new(1_000, 0).unwrap(),
                )),
        )
        .unwrap();
    let neighbor = actor(&mut game, GridPos::new(3, 2), 30);
    let mut prepared = game
        .prepare_targeted_attack(game.player, 0, target)
        .unwrap();
    prepared.attack = prepared
        .attack
        .with_melee_impact(crate::combat::MeleeImpactProfile::new(20, 0))
        .unwrap();
    prepared.forced_movement = Some(ForcedMovement::new(2, 0));
    game.drain_events();
    game.resolve_prepared_attack(prepared, ActionOrigin::Normal)
        .unwrap();
    assert_eq!(
        game.actors.get(target).unwrap().position(),
        GridPos::new(5, 3)
    );
    assert_eq!(centers(&game), vec![GridPos::new(3, 3)]);
    assert_eq!(game.actors.get(neighbor).unwrap().integrity(), 23);
}

#[test]
fn secondary_zone_does_not_cross_a_wall_to_reach_another_actor() {
    let mut game = area_game(
        AttackDelivery::Ranged,
        WeaponEffectOrigin::Impact,
        WeaponEffectTrigger::OnHit,
        false,
        false,
    );
    for y in 0..7 {
        game.map
            .set_terrain(GridPos::new(9, y), Terrain::Wall)
            .unwrap();
    }
    let target = actor(&mut game, GridPos::new(8, 3), 30);
    let behind_wall = actor(&mut game, GridPos::new(10, 3), 30);
    let mut prepared = game
        .prepare_targeted_attack(game.player, 0, target)
        .unwrap();
    prepared.weapon_effects = vec![
        WeaponEffect::radial_damage(
            RadialDamageEffect {
                maximum_cost: 2,
                neighbor_mode: NeighborMode::CardinalAndDiagonal,
                propagation_policy: TerrainPropagationPolicy::blocked_by_walls(1),
                damage: DamagePacket::new(7, DamageType::Electrical, 0),
                falloff: DamageFalloff::None,
            },
            WeaponEffectTrigger::OnHit,
            WeaponEffectOrigin::Impact,
            false,
        )
        .unwrap(),
    ];
    game.drain_events();
    game.resolve_prepared_attack(prepared, ActionOrigin::Normal)
        .unwrap();
    assert_eq!(game.actors.get(behind_wall).unwrap().integrity(), 30);
    assert!(game.events().iter().any(|event| matches!(event,
        GameEvent::PropagationResolved { cells, .. } if cells.iter().all(|cell| cell.position.x < 9)
    )));
}
