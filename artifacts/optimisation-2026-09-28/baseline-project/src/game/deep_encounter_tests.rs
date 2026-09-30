use super::*;
use crate::combat::{AttackDelivery, ConeAttack, DamageType, PulseAttack};
use crate::entity::{BodyComponentProfile, ComponentFailureEffect};
use crate::world::DistanceMetric;

fn attack(howler: bool) -> AttackProfile {
    AttackProfile::new(
        if howler { 3 } else { 5 },
        if howler {
            DistanceMetric::Euclidean
        } else {
            DistanceMetric::Chebyshev
        },
        true,
        DamageType::Kinetic,
        5,
        0,
    )
    .with_delivery(AttackDelivery::Ranged)
    .with_area(if howler {
        AttackArea::Pulse(PulseAttack::new(3).unwrap())
    } else {
        AttackArea::Cone(ConeAttack::new(1, 2, 2).unwrap())
    })
    .with_recovery_after_attack(crate::time::TimeUnits::new(3).unwrap())
}

fn arena(howler: bool) -> (GameState, EntityId) {
    let map = Map::from_ascii("#############\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#############").unwrap();
    let mut game = GameState::new(map, GridPos::new(5, 5), 17).unwrap();
    let player = game.actors.get_mut(game.player).unwrap();
    *player = player.clone().with_evasion_disabled();
    let actor = Actor::new(GridPos::new(6, 5), 24)
        .unwrap()
        .with_ai(AiProfile::new(
            if howler {
                AiBehavior::TelegraphedResonator
            } else {
                AiBehavior::TelegraphedProjector
            },
            8,
            0,
            512,
            0,
        ))
        .with_attack(attack(howler))
        .with_player_relation(PlayerRelation::Hostile)
        .with_body_components([BodyComponentProfile::new(
            "core:sentinel_projector".parse().unwrap(),
            "test".into(),
            10,
            0,
            ComponentFailureEffect::DisableAttackSlot(0),
        )
        .unwrap()]);
    let entity = game.spawn_actor(actor).unwrap();
    (game, entity)
}

#[test]
fn deep_attacks_count_down_then_hit_the_advertised_cells_once_and_recover() {
    for howler in [false, true] {
        let (mut game, entity) = arena(howler);
        let hp = game.actors.get(game.player).unwrap().integrity();
        game.process_player_command(GameCommand::Wait);
        let preview = game.telegraphed_attack_cells(entity);
        assert!(!preview.is_empty());
        if howler {
            assert_eq!(preview.len(), 28);
        }
        let delay = if howler { 2 } else { 1 };
        for remaining in (0..=delay).rev() {
            assert!(
                matches!(game.actors.get(entity).unwrap().ai_state(), AiState::ChargingAttack {remaining_turns, ..} if remaining_turns == remaining)
            );
            assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
            assert_eq!(game.telegraphed_attack_cells(entity), preview);
            let bytes = bincode::serialize(game.actors.get(entity).unwrap()).unwrap();
            *game.actors.get_mut(entity).unwrap() = bincode::deserialize(&bytes).unwrap();
            game.drain_events();
            game.process_player_command(GameCommand::Wait);
        }
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp - 5);
        let attacks: Vec<_> = game
            .drain_events()
            .into_iter()
            .filter_map(|event| match event {
                GameEvent::AttackPerformed {
                    attacker,
                    affected_cells,
                    ..
                } if attacker == entity => Some(affected_cells),
                _ => None,
            })
            .collect();
        assert_eq!(attacks, vec![preview]);
        for remaining in [Some(3), Some(2), Some(1)] {
            assert_eq!(
                game.actors
                    .get(entity)
                    .unwrap()
                    .recovery_remaining()
                    .map(|t| t.get()),
                remaining
            );
            assert!(game.telegraphed_attack_cells(entity).is_empty());
            game.process_player_command(GameCommand::Wait);
            assert_eq!(
                game.actors.get(entity).unwrap().position(),
                GridPos::new(6, 5)
            );
        }
    }
}

#[test]
fn deep_attacks_allow_escape_from_adjacent_contact_without_tracking() {
    for howler in [false, true] {
        let (mut game, entity) = arena(howler);
        let hp = game.actors.get(game.player).unwrap().integrity();
        game.process_player_command(GameCommand::Wait);
        for _ in 0..if howler { 3 } else { 2 } {
            assert_eq!(
                game.process_player_command(GameCommand::Move(if howler {
                    Direction::West
                } else {
                    Direction::North
                })),
                CommandOutcome::Applied
            );
        }
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
        assert!(
            game.actors
                .get(entity)
                .unwrap()
                .recovery_remaining()
                .is_some()
        );
    }
}

#[test]
fn deep_attack_displacement_or_broken_projector_cancels_pending_damage() {
    for howler in [false, true] {
        for displaced in [false, true] {
            let (mut game, entity) = arena(howler);
            let hp = game.actors.get(game.player).unwrap().integrity();
            game.process_player_command(GameCommand::Wait);
            if displaced {
                game.actors
                    .get_mut(entity)
                    .unwrap()
                    .set_position(GridPos::new(7, 5));
            } else {
                game.actors
                    .get_mut(entity)
                    .unwrap()
                    .body_component_mut(&"core:sentinel_projector".parse().unwrap())
                    .unwrap()
                    .apply_damage(10);
            }
            assert!(game.telegraphed_attack_cells(entity).is_empty());
            game.drain_events();
            for _ in 0..3 {
                game.process_player_command(GameCommand::Wait);
            }
            assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
            assert!(!game.drain_events().iter().any(|event| matches!(event, GameEvent::AttackPerformed {attacker, ..} if *attacker == entity)));
        }
    }
}

#[test]
fn howler_cover_clips_rays_without_cancelling_uncovered_side_of_wave() {
    let (mut game, entity) = arena(true);
    game.actors
        .get_mut(game.player)
        .unwrap()
        .set_position(GridPos::new(4, 5));
    let exposed = game
        .spawn_actor(
            Actor::new(GridPos::new(7, 5), 20)
                .unwrap()
                .with_evasion_disabled(),
        )
        .unwrap();
    let protected = game
        .spawn_actor(
            Actor::new(GridPos::new(6, 6), 20)
                .unwrap()
                .with_evasion_disabled(),
        )
        .unwrap();
    let hp = game.actors.get(game.player).unwrap().integrity();
    game.process_player_command(GameCommand::Wait);
    game.map
        .set_terrain(GridPos::new(5, 5), Terrain::Wall)
        .unwrap();
    game.map.set_protected(GridPos::new(6, 6), true).unwrap();
    let preview = game.telegraphed_attack_cells(entity);
    assert!(
        !preview
            .iter()
            .any(|cell| [GridPos::new(4, 5), GridPos::new(6, 6)].contains(&cell.position))
    );
    assert!(
        preview
            .iter()
            .any(|cell| cell.position == GridPos::new(7, 5))
    );
    for _ in 0..3 {
        game.process_player_command(GameCommand::Wait);
    }
    assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
    assert_eq!(game.actors.get(exposed).unwrap().integrity(), 15);
    assert_eq!(game.actors.get(protected).unwrap().integrity(), 20);
}

#[test]
fn deep_attack_content_rejects_unbounded_or_unavoidable_profiles() {
    use crate::content::PopulationGroupDefinition;
    for howler in [false, true] {
        let ai = AiProfile::new(
            if howler {
                AiBehavior::TelegraphedResonator
            } else {
                AiBehavior::TelegraphedProjector
            },
            8,
            0,
            512,
            0,
        );
        let valid = |profile| PopulationGroupDefinition::new(1, 20, 24, profile, ai, None).is_ok();
        assert!(valid(attack(howler)));
        assert!(!valid(attack(howler).with_area(AttackArea::Single)));
        assert!(!valid(attack(howler).with_delivery(AttackDelivery::Melee)));
        assert!(!valid(attack(howler).with_recovery_after_attack(
            crate::time::TimeUnits::new(1).unwrap()
        )));
    }
    assert!(PulseAttack::new(0).is_err());
    assert!(PulseAttack::new(u16::MAX).is_err());
}
