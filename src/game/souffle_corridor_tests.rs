//! SOUFFLE-COULOIR-01: shape, targeting, orientation and retained channel semantics.
use super::*;
fn fixture(rank: u8) -> GameState {
    let mut rules = agent_cast::tests::fixture().rules.clone();
    rules.player_field_of_view.radius = 20;
    let map = Map::from_ascii(&vec![".".repeat(35); 35].join("\n")).unwrap();
    let mut game = GameState::new_with_rules(map, GridPos::new(17, 17), 53, rules).unwrap();
    game.player_skills.set_agent_skill_rank("FE02", rank);
    game.player_energy = crate::resources::EnergyReserve::new(60000, 60000).unwrap();
    game.drain_events();
    game
}
fn cast(target: GridPos) -> GameCommand {
    GameCommand::UseTechniqueAt {
        technique: crate::skills::agent::technique("FE02"),
        target,
        weapon_slot: 0,
    }
}
fn hostile(game: &mut GameState, at: GridPos) -> EntityId {
    game.spawn_actor(
        Actor::new(at, 5000)
            .unwrap()
            .with_player_relation(crate::social::PlayerRelation::Hostile),
    )
    .unwrap()
}
#[test]
fn three_wide_two_long_then_one_more_per_rank_and_all_corners_are_targetable() {
    for rank in 1..=12 {
        let game = fixture(rank);
        let before = bincode::serialize(&game.snapshot().unwrap()).unwrap();
        for (x, y) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let target = GridPos::new(17 + x, 17 + y);
            let preview = game.agent_spell_preview("FE02", target).unwrap();
            assert_eq!(preview.target(), target);
            assert_eq!(preview.cells().len(), 3 * (usize::from(rank) + 1));
            for cell in preview.cells() {
                let (dx, dy) = (cell.position.x - 17, cell.position.y - 17);
                assert!((1..=i32::from(rank) + 1).contains(&(dx * x + dy * y)));
                assert!((-1..=1).contains(&(-dx * y + dy * x)));
                let aimed = game.agent_spell_preview("FE02", cell.position).unwrap();
                assert!(
                    aimed
                        .cells()
                        .iter()
                        .any(|hit| hit.position == cell.position)
                );
            }
        }
        assert_eq!(
            before,
            bincode::serialize(&game.snapshot().unwrap()).unwrap()
        );
    }
}
#[test]
fn channel_keeps_the_selected_direction_hits_all_lanes_and_pays_once_per_turn() {
    for (x, y) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
        let mut game = fixture(1);
        let at = |forward: i32, side: i32| {
            GridPos::new(17 + x * forward - y * side, 17 + y * forward + x * side)
        };
        let victims = [(1, 0), (2, 0), (2, -1), (2, 1)].map(|(f, s)| hostile(&mut game, at(f, s)));
        let outside = hostile(&mut game, at(3, 0));
        let target = at(1, 0);
        let cells = game
            .agent_spell_preview("FE02", target)
            .unwrap()
            .cells()
            .iter()
            .map(|c| c.position)
            .collect::<Vec<_>>();
        let hp = game.actors.get(game.player).unwrap().integrity();
        let energy = game.player_energy.available();
        let cost = game.agent_skill_ready("FE02").unwrap().energy;
        game.drain_events();
        assert_eq!(
            game.process_player_command(cast(target)),
            CommandOutcome::Applied
        );
        assert_eq!(game.turn, 1);
        assert_eq!(game.player_energy.available(), energy - cost);
        for v in victims {
            assert!(game.actors.get(v).unwrap().integrity() < 5000);
        }
        assert_eq!(game.actors.get(outside).unwrap().integrity(), 5000);
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
        assert_eq!(
            game.events
                .iter()
                .filter(|e| matches!(e, GameEvent::TechniqueUsed { .. }))
                .count(),
            1
        );
        let effect = game
            .agent_runtime()
            .magic
            .effects
            .iter()
            .find(|e| e.skill == "FE02")
            .unwrap();
        assert_eq!(effect.direction, (x, y));
        assert_eq!(effect.cells, cells);
        assert_eq!(effect.remaining, 2);
        game.drain_events();
        let saved = bincode::serialize(&game.snapshot().unwrap()).unwrap();
        let mut restored =
            GameState::from_snapshot(bincode::deserialize(&saved).unwrap(), game.rules.clone());
        for g in [&mut game, &mut restored] {
            assert_eq!(
                g.process_player_command(GameCommand::Wait),
                CommandOutcome::Applied
            );
            g.drain_events();
        }
        assert_eq!(game.player_energy.available(), energy - 2 * cost);
        assert_eq!(
            bincode::serialize(&game.snapshot().unwrap()).unwrap(),
            bincode::serialize(&restored.snapshot().unwrap()).unwrap()
        );
    }
}
#[test]
fn walls_stop_each_lane_and_a_first_enemy_does_not_stop_the_next() {
    let mut game = fixture(4);
    let front = hostile(&mut game, GridPos::new(19, 17));
    let rear = hostile(&mut game, GridPos::new(21, 17));
    for y in 16..=18 {
        game.map
            .set_terrain(GridPos::new(20, y), Terrain::Wall)
            .unwrap();
    }
    game.refresh_player_visibility();
    game.drain_events();
    let p = game
        .agent_spell_preview("FE02", GridPos::new(18, 17))
        .unwrap();
    assert!(p.cells().iter().any(|c| c.position == GridPos::new(19, 17)));
    assert!(p.cells().iter().all(|c| c.position.x < 20));
    assert_eq!(
        game.process_player_command(cast(GridPos::new(18, 17))),
        CommandOutcome::Applied
    );
    assert!(game.actors.get(front).unwrap().integrity() < 5000);
    assert_eq!(game.actors.get(rear).unwrap().integrity(), 5000);
}
#[test]
fn rejected_self_distant_or_unfunded_casts_leave_state_and_rng_unchanged() {
    let mut game = fixture(1);
    for target in [
        GridPos::new(17, 17),
        GridPos::new(20, 17),
        GridPos::new(19, 19),
    ] {
        let before = bincode::serialize(&game.snapshot().unwrap()).unwrap();
        assert!(matches!(
            game.process_player_command(cast(target)),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(
            before,
            bincode::serialize(&game.snapshot().unwrap()).unwrap()
        );
    }
    let available = game.player_energy.available();
    game.player_energy.spend(available).unwrap();
    let before = bincode::serialize(&game.snapshot().unwrap()).unwrap();
    assert!(matches!(
        game.process_player_command(cast(GridPos::new(18, 17))),
        CommandOutcome::Rejected(CommandRejection::InsufficientEnergy { .. })
    ));
    assert_eq!(
        before,
        bincode::serialize(&game.snapshot().unwrap()).unwrap()
    );
}
