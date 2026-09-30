use super::*;
use crate::combat::{AttackDelivery, DamageType};
use crate::world::DistanceMetric;

fn arena(behavior: AiBehavior, position: GridPos, player: GridPos) -> (GameState, EntityId) {
    let map = Map::from_ascii("###########\n#.........#\n#.........#\n#.........#\n#.........#\n#.........#\n#.........#\n#.........#\n###########").unwrap();
    let mut game = GameState::new(map, player, 17).unwrap();
    let actor = Actor::new(position, 40)
        .unwrap()
        .with_player_relation(if behavior == AiBehavior::Riveter {
            PlayerRelation::Neutral
        } else {
            PlayerRelation::Hostile
        })
        .with_ai(
            AiProfile::new(behavior, 8, 0, 512, 0)
                .with_maximum_pursuit_distance(std::num::NonZeroU16::new(6).unwrap()),
        )
        .with_attack(
            AttackProfile::new(
                if behavior == AiBehavior::AlternatingStalker {
                    2
                } else {
                    3
                },
                DistanceMetric::Chebyshev,
                true,
                DamageType::Kinetic,
                3,
                0,
            )
            .with_delivery(AttackDelivery::Ranged)
            .with_recovery_after_attack(crate::time::TimeUnits::new(2).unwrap()),
        );
    let id = game.spawn_actor(actor).unwrap();
    (game, id)
}

fn release_cells(game: &mut GameState, id: EntityId) -> Vec<AttackAreaCell> {
    game.drain_events();
    game.process_player_command(GameCommand::Wait);
    game.drain_events()
        .into_iter()
        .filter_map(|event| match event {
            GameEvent::AttackPerformed {
                attacker,
                affected_cells,
                ..
            } if attacker == id => Some(affected_cells),
            _ => None,
        })
        .flatten()
        .collect()
}

#[test]
fn bestiary_ring_expands_over_two_announced_turns_leaves_safe_center_and_recovers() {
    let (mut game, id) = arena(
        AiBehavior::ExpandingRing,
        GridPos::new(5, 4),
        GridPos::new(4, 4),
    );
    let hp = game.actors.get(game.player).unwrap().integrity();
    game.process_player_command(GameCommand::Wait);
    for radius in [2, 3] {
        let preview = game.telegraphed_attack_cells(id);
        assert!(!preview.is_empty());
        assert!(preview.iter().all(|c| {
            let x = c.position.x - 5;
            let y = c.position.y - 4;
            let d = 4 * (x * x + y * y);
            d > (2 * radius - 1) * (2 * radius - 1) && d <= (2 * radius + 1) * (2 * radius + 1)
        }));
        assert_eq!(release_cells(&mut game, id), preview);
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
    }
    assert_eq!(
        game.actors
            .get(id)
            .unwrap()
            .recovery_remaining()
            .unwrap()
            .get(),
        3
    );
    assert!(game.telegraphed_attack_cells(id).is_empty());
}

#[test]
fn bestiary_guard_locks_facing_allows_flanking_and_does_not_track_hidden_player() {
    let (mut game, id) = arena(
        AiBehavior::DirectionalGuard,
        GridPos::new(5, 4),
        GridPos::new(3, 4),
    );
    game.resolve_bestiary_encounter(id, AiAction::Wait, game.player, false);
    assert!(game.directional_guard_blocks(id, GridPos::new(3, 4)));
    assert!(!game.directional_guard_blocks(id, GridPos::new(5, 2)));
    assert!(!game.directional_guard_blocks(id, GridPos::new(7, 4)));
    game.actors
        .get_mut(game.player)
        .unwrap()
        .set_position(GridPos::new(7, 4));
    for _ in 0..2 {
        game.resolve_bestiary_encounter(id, AiAction::Wait, game.player, false);
    }
    assert!(!game.directional_guard_blocks(id, GridPos::new(7, 4)));
    game.map
        .set_terrain(GridPos::new(6, 4), Terrain::Wall)
        .unwrap();
    game.resolve_bestiary_encounter(id, AiAction::Wait, game.player, false);
    assert!(!game.directional_guard_blocks(id, GridPos::new(7, 4)));
}

#[test]
fn bestiary_laggard_slows_announces_then_strikes_a_frozen_cell() {
    let (mut game, id) = arena(
        AiBehavior::AlternatingStalker,
        GridPos::new(5, 4),
        GridPos::new(3, 4),
    );
    for _ in 0..2 {
        game.process_player_command(GameCommand::Wait);
        assert!(game.telegraphed_attack_cells(id).is_empty());
    }
    game.process_player_command(GameCommand::Wait);
    let cells = game.telegraphed_attack_cells(id);
    assert_eq!(cells.len(), 1);
    assert_eq!(cells[0].position, GridPos::new(3, 4));
    let hp = game.actors.get(game.player).unwrap().integrity();
    game.process_player_command(GameCommand::Move(Direction::South));
    assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
    assert!(game.actors.get(id).unwrap().recovery_remaining().is_some());
}

#[test]
fn bestiary_guard_mitigates_real_frontal_impacts_not_flanks_or_status_damage() {
    for (from, expected) in [(GridPos::new(3, 3), 5), (GridPos::new(5, 2), 10)] {
        let (mut game, id) = arena(
            AiBehavior::DirectionalGuard,
            GridPos::new(5, 4),
            GridPos::new(3, 4),
        );
        game.resolve_bestiary_encounter(id, AiAction::Wait, game.player, false);
        let attacker = game
            .spawn_actor(
                Actor::new(from, 20)
                    .unwrap()
                    .with_attack(AttackProfile::new(
                        3,
                        DistanceMetric::Chebyshev,
                        true,
                        DamageType::Kinetic,
                        10,
                        0,
                    )),
            )
            .unwrap();
        game.perform_attack(attacker, 0, id).unwrap();
        assert_eq!(game.actors.get(id).unwrap().integrity(), 40 - expected);
        game.apply_damage_to(
            Some(attacker),
            id,
            DamagePacket::new(4, DamageType::Chemical, 0),
        )
        .unwrap();
        assert_eq!(game.actors.get(id).unwrap().integrity(), 36 - expected);
    }
}

#[test]
fn bestiary_worker_axis_stops_at_cover_and_never_hits_a_protected_cell() {
    let (mut game, id) = arena(AiBehavior::Riveter, GridPos::new(5, 4), GridPos::new(4, 4));
    game.process_player_command(GameCommand::Wait);
    game.map
        .set_terrain(GridPos::new(5, 2), Terrain::Wall)
        .unwrap();
    let preview = game.telegraphed_attack_cells(id);
    // The committed endpoint is now behind cover: the entire strike cancels.
    assert!(preview.is_empty());
    assert!(release_cells(&mut game, id).is_empty());
    game.map
        .set_terrain(GridPos::new(5, 2), Terrain::Floor)
        .unwrap();
    game.map.set_protected(GridPos::new(5, 3), true).unwrap();
    for _ in 0..4 {
        game.process_player_command(GameCommand::Wait);
    }
    assert!(
        game.telegraphed_attack_cells(id)
            .iter()
            .all(|c| !game.map.is_protected(c.position))
    );
}

#[test]
fn bestiary_bat_flies_over_water_not_walls_dives_then_withdraws_without_attacking() {
    let (mut game, id) = arena(
        AiBehavior::NestDiver,
        GridPos::new(6, 4),
        GridPos::new(3, 4),
    );
    game.map
        .set_terrain(GridPos::new(5, 4), Terrain::DeepWater)
        .unwrap();
    assert!(game.actor_can_traverse(id, GridPos::new(5, 4)));
    assert!(!game.actor_can_traverse(game.player, GridPos::new(5, 4)));
    assert!(!game.actor_can_traverse(id, GridPos::new(0, 4)));
    game.process_player_command(GameCommand::Wait);
    let cells = game.telegraphed_attack_cells(id);
    assert_eq!(cells.len(), 1);
    assert_eq!(release_cells(&mut game, id), cells);
    assert_eq!(game.actors.get(id).unwrap().position(), GridPos::new(4, 4));
    assert!(matches!(
        game.actors.get(id).unwrap().ai_state(),
        AiState::Encounter(EncounterState::Withdrawal { .. })
    ));
    for _ in 0..2 {
        assert!(release_cells(&mut game, id).is_empty());
    }
    assert_eq!(game.actors.get(id).unwrap().position(), GridPos::new(6, 4));
}

#[test]
fn bestiary_worker_is_neutral_announces_local_axis_and_interaction_stops_it_persistently() {
    let (mut game, id) = arena(AiBehavior::Riveter, GridPos::new(5, 4), GridPos::new(4, 4));
    game.process_player_command(GameCommand::Wait);
    let cells = game.telegraphed_attack_cells(id);
    assert_eq!(
        cells.iter().map(|c| c.position).collect::<Vec<_>>(),
        vec![GridPos::new(5, 3), GridPos::new(5, 2), GridPos::new(5, 1)]
    );
    assert_eq!(
        game.actors.get(id).unwrap().player_relation(),
        PlayerRelation::Neutral
    );
    assert_eq!(
        game.process_player_command(GameCommand::Interact {
            target: GridPos::new(5, 4)
        }),
        CommandOutcome::Applied
    );
    assert!(game.telegraphed_attack_cells(id).is_empty());
    let bytes = bincode::serialize(game.actors.get(id).unwrap()).unwrap();
    *game.actors.get_mut(id).unwrap() = bincode::deserialize(&bytes).unwrap();
    for _ in 0..8 {
        assert!(release_cells(&mut game, id).is_empty());
    }
    assert_eq!(game.actors.get(id).unwrap().position(), GridPos::new(5, 4));
}

#[test]
fn bestiary_commitments_survive_serialization_and_cancel_on_displacement_or_cover() {
    for behavior in [
        AiBehavior::ExpandingRing,
        AiBehavior::AlternatingStalker,
        AiBehavior::NestDiver,
        AiBehavior::Riveter,
    ] {
        let (mut game, id) = arena(behavior, GridPos::new(5, 4), GridPos::new(3, 4));
        for _ in 0..if behavior == AiBehavior::AlternatingStalker {
            3
        } else {
            1
        } {
            game.process_player_command(GameCommand::Wait);
        }
        let before = game.telegraphed_attack_cells(id);
        assert!(!before.is_empty(), "{behavior:?}");
        let bytes = bincode::serialize(game.actors.get(id).unwrap()).unwrap();
        *game.actors.get_mut(id).unwrap() = bincode::deserialize(&bytes).unwrap();
        assert_eq!(before, game.telegraphed_attack_cells(id));
        game.actors
            .get_mut(id)
            .unwrap()
            .set_position(GridPos::new(5, 5));
        assert!(game.telegraphed_attack_cells(id).is_empty());
        assert!(release_cells(&mut game, id).is_empty());
    }
}
