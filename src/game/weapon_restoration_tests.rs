use super::*;
use crate::combat::{ArmorRules, ConeAttack, DamageType, HitRules};
use crate::content::ContentId;
use crate::effects::{DamageFalloff, DestructionEffect};
use crate::weapon::{WeaponCatalog, WeaponEffectOrigin};
use crate::world::{NeighborMode, TerrainPropagationPolicy};

fn tag() -> ContentId {
    "test:restoration_target".parse().unwrap()
}

fn radial() -> RadialDamageEffect {
    RadialDamageEffect {
        maximum_cost: 1,
        neighbor_mode: NeighborMode::CardinalAndDiagonal,
        propagation_policy: TerrainPropagationPolicy::blocked_by_walls(1),
        damage: DamagePacket::new(30, DamageType::Electrical, 0),
        falloff: DamageFalloff::None,
    }
}

fn fixture(delivery: AttackDelivery, cone: bool, secondary: bool) -> GameState {
    parameterized_fixture(delivery, cone, secondary, 100, 3, 3)
}

fn parameterized_fixture(
    delivery: AttackDelivery,
    cone: bool,
    secondary: bool,
    percent: u16,
    cap: u16,
    damage: u16,
) -> GameState {
    let mut attack = AttackProfile::new(
        10,
        DistanceMetric::Chebyshev,
        true,
        DamageType::Kinetic,
        damage,
        0,
    )
    .with_delivery(delivery);
    if cone {
        attack = attack.with_area(AttackArea::Cone(ConeAttack::new(1, 1, 1).unwrap()));
    }
    let mut effects =
        vec![WeaponEffect::life_steal(percent, cap, tag(), WeaponEffectTrigger::OnDamage).unwrap()];
    if secondary {
        effects.insert(
            0,
            WeaponEffect::radial_damage(
                radial(),
                WeaponEffectTrigger::OnHit,
                WeaponEffectOrigin::Impact,
                false,
            )
            .unwrap(),
        );
    }
    let id: WeaponId = "test:restoration_weapon".parse().unwrap();
    let mut weapons = WeaponCatalog::default();
    weapons
        .register(
            WeaponDefinition::new(id.clone(), "name".into(), "description".into(), attack)
                .unwrap()
                .with_effects(effects),
        )
        .unwrap();
    GameState::new_with_rules(
        Map::filled(14, 7, Terrain::Floor).unwrap(),
        GridPos::new(2, 3),
        7,
        GameRules {
            player_base_attacks: vec![attack],
            player_weapon_slots: vec!["test:weapon_slot".parse().unwrap()],
            player_starting_weapons: vec![id.clone()],
            player_starting_equipment: vec![Some(id)],
            weapons,
            ..GameRules::default()
        },
    )
    .unwrap()
    .with_starting_player_integrity(8)
    .unwrap()
}

fn target(game: &mut GameState, at: GridPos, hp: u16, eligible: bool) -> EntityId {
    let mut actor = Actor::new(at, hp).unwrap();
    if eligible {
        actor = actor.with_tags([tag()]);
    }
    game.spawn_actor(actor).unwrap()
}

fn heals(game: &GameState) -> Vec<(EntityId, u16)> {
    game.events()
        .iter()
        .filter_map(|event| match event {
            GameEvent::IntegrityRestored { entity, amount } => Some((*entity, *amount)),
            _ => None,
        })
        .collect()
}

#[test]
fn restoration_heals_only_the_bearer_for_melee_ranged_and_lethal_hits() {
    for delivery in [AttackDelivery::Melee, AttackDelivery::Ranged] {
        for lethal in [false, true] {
            let mut game = fixture(delivery, false, false);
            let at = GridPos::new(
                if delivery == AttackDelivery::Melee {
                    3
                } else {
                    8
                },
                3,
            );
            let target = target(&mut game, at, if lethal { 3 } else { 30 }, true);
            game.process_player_command(GameCommand::Attack { slot: 0, target });
            assert_eq!(heals(&game), [(game.player, 3)]);
            assert_eq!(game.actors.get(game.player).unwrap().integrity(), 11);
            assert_eq!(
                game.actors.get(target).map(Actor::integrity),
                if lethal { None } else { Some(27) }
            );
        }
    }
}

#[test]
fn restoration_caps_actual_healing_and_emits_nothing_at_full_health() {
    for missing in [0, 1, 2] {
        let mut game = fixture(AttackDelivery::Ranged, false, false);
        let bearer = game.actors.get_mut(game.player).unwrap();
        bearer.restore_integrity(u16::MAX);
        bearer.apply_damage(missing);
        let target = target(&mut game, GridPos::new(8, 3), 30, true);
        game.process_player_command(GameCommand::Attack { slot: 0, target });
        assert_eq!(
            heals(&game),
            if missing == 0 {
                vec![]
            } else {
                vec![(game.player, missing)]
            }
        );
        let bearer = game.actors.get(game.player).unwrap();
        assert_eq!(bearer.integrity(), bearer.maximum_integrity());
    }
}

#[test]
fn restoration_requires_admission_positive_damage_and_a_normal_attack() {
    for denial in ["unmarked", "miss", "armor", "protected", "reaction"] {
        let mut game = fixture(AttackDelivery::Ranged, false, false);
        let target = target(&mut game, GridPos::new(8, 3), 30, denial != "unmarked");
        match denial {
            "miss" => {
                game.rules.hit_rules = Some(HitRules {
                    minimum_hit_chance: 0,
                    maximum_hit_chance: 0,
                    ..HitRules::default()
                })
            }
            "armor" => {
                game.rules.armor_rules = Some(ArmorRules::default());
                let actor = game.actors.get(target).unwrap().clone().with_body_profile(
                    crate::stats::BodyProfile::new(30, 0)
                        .unwrap()
                        .with_base_armor(100),
                );
                *game.actors.get_mut(target).unwrap() = actor;
            }
            "protected" => game.map.set_protected(GridPos::new(8, 3), true).unwrap(),
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
        assert!(heals(&game).is_empty(), "{denial}");
        assert_eq!(
            game.actors.get(game.player).unwrap().integrity(),
            8,
            "{denial}"
        );
    }
}

#[test]
fn restoration_has_one_budget_for_a_multitarget_action_and_none_for_empty_attacks() {
    for count in [0, 1, 3] {
        let mut game = fixture(AttackDelivery::Ranged, true, false);
        for x in 5..5 + count {
            target(&mut game, GridPos::new(x, 3), 30, true);
        }
        game.process_player_command(GameCommand::AttackAt {
            slot: 0,
            target: GridPos::new(8, 3),
        });
        assert_eq!(
            heals(&game),
            if count == 0 {
                vec![]
            } else {
                vec![(game.player, 3)]
            }
        );
    }
}

#[test]
fn restoration_does_not_use_eligibility_of_secondary_victims() {
    for primary_eligible in [false, true] {
        let mut game = fixture(AttackDelivery::Ranged, false, true);
        let primary = target(&mut game, GridPos::new(8, 3), 30, primary_eligible);
        let secondary = target(&mut game, GridPos::new(9, 3), 30, true);
        game.process_player_command(GameCommand::Attack {
            slot: 0,
            target: primary,
        });
        assert!(game.actors.get(secondary).is_none());
        assert_eq!(
            heals(&game),
            if primary_eligible {
                vec![(game.player, 3)]
            } else {
                vec![]
            }
        );
    }
}

#[test]
fn restoration_cannot_resurrect_a_bearer_killed_by_the_targets_death_explosion() {
    let mut game = fixture(AttackDelivery::Melee, false, false);
    let target = game
        .spawn_actor(
            Actor::new(GridPos::new(3, 3), 3)
                .unwrap()
                .with_tags([tag()])
                .with_destruction_effect(DestructionEffect::new(radial())),
        )
        .unwrap();
    game.process_player_command(GameCommand::Attack { slot: 0, target });
    assert_eq!(game.status(), RunStatus::PlayerDestroyed);
    assert!(game.actors.get(game.player).is_none());
    assert!(heals(&game).is_empty());
}

#[test]
fn restoration_replays_after_a_snapshot_round_trip_without_new_state() {
    let mut game = fixture(AttackDelivery::Ranged, false, false);
    let target = target(&mut game, GridPos::new(8, 3), 30, true);
    game.drain_events();
    let encoded = bincode::serialize(&game.snapshot().unwrap()).unwrap();
    let mut restored =
        GameState::from_snapshot(bincode::deserialize(&encoded).unwrap(), game.rules.clone());
    for run in [&mut game, &mut restored] {
        run.process_player_command(GameCommand::Attack { slot: 0, target });
    }
    assert_eq!(format!("{game:?}"), format!("{restored:?}"));
    assert_eq!(game.events(), restored.events());
}

#[test]
fn starting_integrity_is_validated_and_cannot_be_used_as_a_runtime_cheat() {
    for hp in [0, 9, u16::MAX] {
        assert!(
            fixture(AttackDelivery::Ranged, false, false)
                .with_starting_player_integrity(hp)
                .is_err()
        );
    }
    let mut game = fixture(AttackDelivery::Ranged, false, false);
    game.process_player_command(GameCommand::Wait);
    assert!(game.with_starting_player_integrity(5).is_err());
}

#[test]
fn restoration_is_proportional_to_actual_damage_not_a_fixed_heal_or_overkill() {
    for (damage, hp, expected) in [
        (1, 30, 0),
        (2, 30, 1),
        (3, 30, 1),
        (4, 30, 2),
        (6, 30, 3),
        (20, 30, 3),
        (20, 2, 1),
    ] {
        let mut game = parameterized_fixture(AttackDelivery::Ranged, false, false, 50, 3, damage);
        let target = target(&mut game, GridPos::new(8, 3), hp, true);
        game.process_player_command(GameCommand::Attack { slot: 0, target });
        assert_eq!(
            heals(&game),
            if expected == 0 {
                vec![]
            } else {
                vec![(game.player, expected)]
            },
            "damage={damage}, hp={hp}"
        );
    }
}

#[test]
fn restoration_aggregates_direct_damage_then_rounds_and_caps_once() {
    for count in [1, 2, 3] {
        let mut game = parameterized_fixture(AttackDelivery::Ranged, true, false, 50, 2, 1);
        for x in 5..5 + count {
            target(&mut game, GridPos::new(x, 3), 30, true);
        }
        game.process_player_command(GameCommand::AttackAt {
            slot: 0,
            target: GridPos::new(8, 3),
        });
        let expected = (count / 2) as u16;
        assert_eq!(
            heals(&game),
            if expected == 0 {
                vec![]
            } else {
                vec![(game.player, expected)]
            }
        );
    }
    let mut game = parameterized_fixture(AttackDelivery::Ranged, true, false, 100, 2, 10);
    for x in 5..8 {
        target(&mut game, GridPos::new(x, 3), 30, true);
    }
    game.process_player_command(GameCommand::AttackAt {
        slot: 0,
        target: GridPos::new(8, 3),
    });
    assert_eq!(heals(&game), [(game.player, 2)]);
}
