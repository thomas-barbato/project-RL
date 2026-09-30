use super::*;
use crate::combat::{AttackDelivery, DamageType};
use crate::world::DistanceMetric;

fn arena() -> (GameState, EntityId) {
    let map = Map::from_ascii(
        "##########\n#........#\n#........#\n#........#\n#........#\n#........#\n##########",
    )
    .unwrap();
    let mut game = GameState::new(map, GridPos::new(2, 3), 17).unwrap();
    let spitter = game
        .spawn_actor(
            Actor::new(GridPos::new(6, 3), 6)
                .unwrap()
                .with_player_relation(PlayerRelation::Hostile)
                .with_ai(
                    AiProfile::new(AiBehavior::TelegraphedSpitter, 5, 0, 512, 0)
                        .with_maximum_pursuit_distance(std::num::NonZeroU16::new(6).unwrap()),
                )
                .with_attack(
                    AttackProfile::new(
                        4,
                        DistanceMetric::Chebyshev,
                        true,
                        DamageType::Chemical,
                        3,
                        0,
                    )
                    .with_delivery(AttackDelivery::Ranged)
                    .with_recovery_after_attack(crate::time::TimeUnits::new(2).unwrap()),
                ),
        )
        .unwrap();
    (game, spitter)
}

#[test]
fn spitter_announces_one_cell_hits_once_and_recovers_without_status_or_ground_effects() {
    let (mut game, spitter) = arena();
    let hp = game.actors.get(game.player).unwrap().integrity();
    game.process_player_command(GameCommand::Wait);
    let cells = game.telegraphed_attack_cells(spitter);
    assert_eq!(cells.len(), 1);
    assert_eq!(cells[0].position, GridPos::new(2, 3));
    assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
    let bytes = bincode::serialize(game.actors.get(spitter).unwrap()).unwrap();
    *game.actors.get_mut(spitter).unwrap() = bincode::deserialize(&bytes).unwrap();
    game.drain_events();
    game.process_player_command(GameCommand::Wait);
    assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp - 3);
    let attacks: Vec<_> = game
        .drain_events()
        .into_iter()
        .filter_map(|e| match e {
            GameEvent::AttackPerformed {
                attacker,
                affected_cells,
                ..
            } if attacker == spitter => Some(affected_cells),
            _ => None,
        })
        .collect();
    assert_eq!(attacks, vec![cells]);
    assert_eq!(game.actors.get(game.player).unwrap().statuses().count(), 0);
    for remaining in [2, 1] {
        assert_eq!(
            game.actors
                .get(spitter)
                .unwrap()
                .recovery_remaining()
                .unwrap()
                .get(),
            remaining
        );
        assert!(game.telegraphed_attack_cells(spitter).is_empty());
        game.process_player_command(GameCommand::Wait);
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp - 3);
        assert_eq!(
            game.actors.get(spitter).unwrap().position(),
            GridPos::new(6, 3)
        );
    }
}

#[test]
fn spitter_allows_dodging_and_cover_displacement_or_protection_cancel_the_jet() {
    for scenario in 0..4 {
        let (mut game, spitter) = arena();
        let hp = game.actors.get(game.player).unwrap().integrity();
        game.process_player_command(GameCommand::Wait);
        match scenario {
            0 => {
                game.process_player_command(GameCommand::Move(Direction::South));
            }
            1 => {
                game.map
                    .set_terrain(GridPos::new(4, 3), Terrain::Wall)
                    .unwrap();
                assert!(game.telegraphed_attack_cells(spitter).is_empty());
                game.process_player_command(GameCommand::Wait);
            }
            2 => {
                game.actors
                    .get_mut(spitter)
                    .unwrap()
                    .set_position(GridPos::new(6, 4));
                assert!(game.telegraphed_attack_cells(spitter).is_empty());
                game.process_player_command(GameCommand::Wait);
            }
            _ => {
                game.map.set_protected(GridPos::new(2, 3), true).unwrap();
                assert!(game.telegraphed_attack_cells(spitter).is_empty());
                game.process_player_command(GameCommand::Wait);
            }
        }
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
    }
}

#[test]
fn spitter_respects_territory_and_cannot_be_healed_by_a_humanoid_medic() {
    let (mut game, spitter) = arena();
    game.actors
        .get_mut(game.player)
        .unwrap()
        .set_position(GridPos::new(1, 1));
    for _ in 0..4 {
        game.process_player_command(GameCommand::Wait);
    }
    assert_eq!(
        game.actors.get(spitter).unwrap().position(),
        GridPos::new(6, 3)
    );
    game.apply_damage_to(
        Some(game.player),
        spitter,
        DamagePacket::new(2, DamageType::Kinetic, 0),
    )
    .unwrap();
    let hp = game.actors.get(spitter).unwrap().integrity();
    let medic = game
        .spawn_actor(
            Actor::new(GridPos::new(7, 3), 12)
                .unwrap()
                .with_ai(AiProfile::new(
                    AiBehavior::FieldMedic {
                        restoration: 3,
                        supplies: 3,
                    },
                    8,
                    0,
                    512,
                    0,
                ))
                .with_player_relation(PlayerRelation::Hostile),
        )
        .unwrap();
    game.resolve_tactical_ai_action(medic, AiAction::Wait, game.player, false);
    assert_eq!(game.actors.get(spitter).unwrap().integrity(), hp);
}
