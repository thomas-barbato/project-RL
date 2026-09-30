use super::*;
use crate::ai::PursuitLifecycle;
use crate::combat::{AttackDelivery, DamageType};
use crate::world::{DistanceMetric, Terrain};
use std::num::NonZeroU16;

fn arena() -> GameState {
    let map = Map::from_ascii("#######################\n#.....................#\n#.....................#\n#.....................#\n#.....................#\n#.....................#\n#.....................#\n#######################").unwrap();
    GameState::new(map, GridPos::new(2, 3), 17).unwrap()
}

fn spawn(game: &mut GameState, at: GridPos, behavior: AiBehavior, sight: u16) -> EntityId {
    let nz = |n| NonZeroU16::new(n).unwrap();
    game.spawn_actor(
        Actor::new(at, 10)
            .unwrap()
            .with_player_relation(PlayerRelation::Hostile)
            .with_ai(
                AiProfile::new(behavior, sight, 0, 512, 0)
                    .with_pursuit_lifecycle(PursuitLifecycle::new(nz(6), nz(3), nz(4))),
            )
            .with_attack(
                AttackProfile::new(
                    4,
                    DistanceMetric::Chebyshev,
                    true,
                    DamageType::Kinetic,
                    3,
                    0,
                )
                .with_delivery(AttackDelivery::Ranged)
                .with_recovery_after_attack(crate::time::TimeUnits::new(2).unwrap()),
            ),
    )
    .unwrap()
}

#[test]
fn strange_fauna_watcher_reports_only_bounded_compatible_neighbors_and_does_not_refresh() {
    let mut game = arena();
    let source = spawn(&mut game, GridPos::new(4, 3), AiBehavior::Watcher, 4);
    let near = spawn(&mut game, GridPos::new(8, 3), AiBehavior::Watcher, 1);
    let far = spawn(&mut game, GridPos::new(12, 3), AiBehavior::Watcher, 1);
    let beyond = spawn(&mut game, GridPos::new(16, 3), AiBehavior::Watcher, 1);
    let unrelated = spawn(&mut game, GridPos::new(5, 2), AiBehavior::Hunter, 1);
    game.relay_watcher_alerts();
    for id in [near, far] {
        assert!(
            matches!(game.actors.get(id).unwrap().ai_state(), AiState::Responding {
            incident, remaining_turns: 6 } if incident == GridPos::new(2, 3))
        );
    }
    for id in [beyond, unrelated] {
        assert_eq!(game.actors.get(id).unwrap().ai_state(), AiState::Unaware);
    }
    assert!(game.drain_events().iter().any(|event| matches!(event,
        GameEvent::FaunaAlertRelayed {source: id, recipients, links, ..} if *id == source && recipients == &[near, far]
            && links == &[(GridPos::new(4, 3), GridPos::new(8, 3)), (GridPos::new(8, 3), GridPos::new(12, 3))])));
    game.actors
        .get_mut(near)
        .unwrap()
        .set_ai_state(AiState::Responding {
            remaining_turns: 2,
            incident: GridPos::new(2, 3),
        });
    game.relay_watcher_alerts();
    assert!(game.drain_events().is_empty());
    assert!(matches!(
        game.actors.get(near).unwrap().ai_state(),
        AiState::Responding {
            remaining_turns: 2,
            ..
        }
    ));
}

#[test]
fn strange_fauna_watcher_limits_recipients_and_respects_cover_protection_and_neutrality() {
    for blocked in [false, true] {
        let mut game = arena();
        spawn(&mut game, GridPos::new(4, 3), AiBehavior::Watcher, 4);
        let ids: Vec<_> = [(6, 1), (6, 2), (6, 3), (6, 4), (6, 5)]
            .into_iter()
            .map(|(x, y)| {
                spawn(
                    &mut game,
                    GridPos::new(x, y),
                    AiBehavior::TelegraphedEcho,
                    1,
                )
            })
            .collect();
        if blocked {
            for y in 1..7 {
                game.map
                    .set_terrain(GridPos::new(5, y), Terrain::Wall)
                    .unwrap();
            }
        }
        game.relay_watcher_alerts();
        assert_eq!(
            ids.iter()
                .filter(|id| matches!(
                    game.actors.get(**id).unwrap().ai_state(),
                    AiState::Responding { .. }
                ))
                .count(),
            if blocked { 0 } else { 3 }
        );
    }
    let mut game = arena();
    spawn(&mut game, GridPos::new(4, 3), AiBehavior::Watcher, 4);
    let protected = spawn(&mut game, GridPos::new(7, 3), AiBehavior::Watcher, 1);
    game.map.set_protected(GridPos::new(7, 3), true).unwrap();
    let neutral = spawn(&mut game, GridPos::new(6, 4), AiBehavior::Watcher, 1);
    let actor = game
        .actors
        .get(neutral)
        .unwrap()
        .clone()
        .with_player_relation(PlayerRelation::Neutral);
    *game.actors.get_mut(neutral).unwrap() = actor;
    game.relay_watcher_alerts();
    for id in [protected, neutral] {
        assert_eq!(game.actors.get(id).unwrap().ai_state(), AiState::Unaware);
    }
    game.map.set_protected(GridPos::new(2, 3), true).unwrap();
    game.relay_watcher_alerts();
    assert!(
        game.drain_events()
            .iter()
            .all(|e| !matches!(e, GameEvent::FaunaAlertRelayed { .. }))
    );
}

#[test]
fn strange_fauna_report_tracks_last_seen_cell_not_a_hidden_player() {
    let mut game = arena();
    spawn(&mut game, GridPos::new(4, 3), AiBehavior::Watcher, 4);
    let recipient = spawn(&mut game, GridPos::new(8, 3), AiBehavior::Watcher, 1);
    game.relay_watcher_alerts();
    game.actors
        .get_mut(game.player)
        .unwrap()
        .set_position(GridPos::new(20, 5));
    for _ in 0..3 {
        game.resolve_ai_turn();
        assert!(
            matches!(game.actors.get(recipient).unwrap().ai_state(), AiState::Responding {incident, ..} if incident == GridPos::new(2, 3))
        );
    }
    for _ in 0..12 {
        game.resolve_ai_turn();
    }
    assert!(!matches!(
        game.actors.get(recipient).unwrap().ai_state(),
        AiState::Responding { .. }
    ));
}

#[test]
fn strange_fauna_spectre_hits_exactly_the_two_announced_cells_once_then_recovers() {
    let mut game = arena();
    let spectre = spawn(
        &mut game,
        GridPos::new(5, 3),
        AiBehavior::TelegraphedEcho,
        6,
    );
    let echo_victim = game
        .spawn_actor(Actor::new(GridPos::new(2, 2), 20).unwrap())
        .unwrap();
    let hp = game.actors.get(game.player).unwrap().integrity();
    game.process_player_command(GameCommand::Wait);
    let cells = game.telegraphed_attack_cells(spectre);
    assert_eq!(
        cells.iter().map(|c| c.position).collect::<BTreeSet<_>>(),
        BTreeSet::from([GridPos::new(2, 3), GridPos::new(2, 2)])
    );
    assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
    let bytes = bincode::serialize(game.actors.get(spectre).unwrap()).unwrap();
    *game.actors.get_mut(spectre).unwrap() = bincode::deserialize(&bytes).unwrap();
    game.drain_events();
    game.process_player_command(GameCommand::Wait);
    assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp - 3);
    assert_eq!(game.actors.get(echo_victim).unwrap().integrity(), 17);
    let attacks: Vec<_> = game
        .drain_events()
        .into_iter()
        .filter_map(|e| match e {
            GameEvent::AttackPerformed {
                attacker,
                affected_cells,
                ..
            } if attacker == spectre => Some(affected_cells),
            _ => None,
        })
        .collect();
    assert_eq!(attacks, vec![cells]);
    for _ in 0..2 {
        assert!(game.telegraphed_attack_cells(spectre).is_empty());
        game.process_player_command(GameCommand::Wait);
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp - 3);
        assert_eq!(
            game.actors.get(spectre).unwrap().position(),
            GridPos::new(5, 3)
        );
    }
}

#[test]
fn strange_fauna_spectre_allows_escape_and_cancels_if_displaced() {
    for displaced in [false, true] {
        let mut game = arena();
        let spectre = spawn(
            &mut game,
            GridPos::new(5, 3),
            AiBehavior::TelegraphedEcho,
            6,
        );
        let hp = game.actors.get(game.player).unwrap().integrity();
        game.process_player_command(GameCommand::Wait);
        if displaced {
            game.actors
                .get_mut(spectre)
                .unwrap()
                .set_position(GridPos::new(5, 4));
            assert!(game.telegraphed_attack_cells(spectre).is_empty());
            game.process_player_command(GameCommand::Wait);
        } else {
            game.process_player_command(GameCommand::Move(Direction::South));
        }
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
    }
}

#[test]
fn strange_fauna_spectre_echo_cannot_hit_through_cover_or_into_protection() {
    for protected in [false, true] {
        let mut game = arena();
        let spectre = spawn(
            &mut game,
            GridPos::new(5, 3),
            AiBehavior::TelegraphedEcho,
            6,
        );
        let echo = GridPos::new(2, 2);
        if protected {
            game.map.set_protected(echo, true).unwrap();
        } else {
            game.map.set_terrain(echo, Terrain::Wall).unwrap();
        }
        game.process_player_command(GameCommand::Wait);
        assert_eq!(game.telegraphed_attack_cells(spectre).len(), 1);
        game.drain_events();
        game.process_player_command(GameCommand::Wait);
        assert!(game.drain_events().iter().all(|e| !matches!(e,
            GameEvent::AttackPerformed {affected_cells, ..} if affected_cells.iter().any(|c| c.position == echo))));
    }
}
