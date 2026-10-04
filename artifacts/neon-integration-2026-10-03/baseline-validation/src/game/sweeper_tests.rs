use super::*;
use crate::combat::{AttackArea, AttackDelivery, ConeAttack, DamageType};
use crate::world::DistanceMetric;

fn sweep() -> AttackProfile {
    AttackProfile::new(
        2,
        DistanceMetric::Chebyshev,
        true,
        DamageType::Kinetic,
        4,
        0,
    )
    .with_delivery(AttackDelivery::Melee)
    .with_area(AttackArea::Cone(ConeAttack::new(1, 1, 1).unwrap()))
    .with_recovery_after_attack(crate::time::TimeUnits::new(2).unwrap())
}

fn arena() -> (GameState, EntityId) {
    let map = Map::from_ascii(
        "###########\n#.........#\n#.........#\n#.........#\n#.........#\n#.........#\n#.........#\n#.........#\n###########"
    ).unwrap();
    let mut game = GameState::new(map, GridPos::new(3, 4), 17).unwrap();
    let player = game.actors.get_mut(game.player).unwrap();
    *player = player.clone().with_evasion_disabled();
    let worm = game
        .spawn_actor(
            Actor::new(GridPos::new(5, 4), 14)
                .unwrap()
                .with_ai(
                    AiProfile::new(AiBehavior::TelegraphedSweeper, 7, 0, 256, 0)
                        .with_maximum_pursuit_distance(std::num::NonZeroU16::new(5).unwrap()),
                )
                .with_attack(sweep())
                .with_player_relation(PlayerRelation::Hostile),
        )
        .unwrap();
    (game, worm)
}

#[test]
fn sweeper_announces_the_actual_cone_then_hits_each_occupant_once() {
    let (mut game, worm) = arena();
    let bystander = game
        .spawn_actor(
            Actor::new(GridPos::new(3, 3), 20)
                .unwrap()
                .with_evasion_disabled(),
        )
        .unwrap();
    let hp = game.actors.get(game.player).unwrap().integrity();
    game.process_player_command(GameCommand::Wait);
    let preview = game.telegraphed_attack_cells(worm);
    assert_eq!(preview.len(), 4);
    assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
    assert_eq!(game.actors.get(bystander).unwrap().integrity(), 20);
    let bytes = bincode::serialize(game.actors.get(worm).unwrap()).unwrap();
    *game.actors.get_mut(worm).unwrap() = bincode::deserialize(&bytes).unwrap();
    game.drain_events();
    game.process_player_command(GameCommand::Wait);
    assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp - 4);
    assert_eq!(game.actors.get(bystander).unwrap().integrity(), 16);
    let attacks: Vec<_> = game
        .drain_events()
        .into_iter()
        .filter_map(|event| match event {
            GameEvent::AttackPerformed {
                attacker,
                affected_cells,
                ..
            } if attacker == worm => Some(affected_cells),
            _ => None,
        })
        .collect();
    assert_eq!(attacks, vec![preview]);
}

#[test]
fn sweeper_does_not_retarget_an_escape_and_recovers_without_chasing() {
    let (mut game, worm) = arena();
    let hp = game.actors.get(game.player).unwrap().integrity();
    game.process_player_command(GameCommand::Wait);
    game.process_player_command(GameCommand::Move(Direction::West));
    assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
    for remaining in [Some(2), Some(1), None] {
        let actor = game.actors.get(worm).unwrap();
        assert_eq!(actor.position(), GridPos::new(5, 4));
        assert_eq!(actor.recovery_remaining().map(|time| time.get()), remaining);
        assert!(game.telegraphed_attack_cells(worm).is_empty());
        game.process_player_command(GameCommand::Wait);
    }
}

#[test]
fn sweeper_cancels_after_displacement_cover_or_a_protected_aim() {
    for mode in 0..3 {
        let (mut game, worm) = arena();
        let hp = game.actors.get(game.player).unwrap().integrity();
        game.process_player_command(GameCommand::Wait);
        match mode {
            0 => game
                .actors
                .get_mut(worm)
                .unwrap()
                .set_position(GridPos::new(6, 4)),
            1 => {
                for y in 1..8 {
                    game.map
                        .set_terrain(GridPos::new(4, y), Terrain::Wall)
                        .unwrap();
                }
            }
            _ => game.map.set_protected(GridPos::new(3, 4), true).unwrap(),
        }
        assert!(game.telegraphed_attack_cells(worm).is_empty());
        game.drain_events();
        game.process_player_command(GameCommand::Wait);
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
        assert!(!game.drain_events().iter().any(|event| matches!(event,
            GameEvent::AttackPerformed { attacker, .. } if *attacker == worm)));
    }
}

#[test]
fn sweeper_preview_excludes_protected_flanks_and_new_side_cover() {
    for protected in [false, true] {
        let (mut game, worm) = arena();
        let flank = GridPos::new(3, 3);
        let occupant = game
            .spawn_actor(Actor::new(flank, 20).unwrap().with_evasion_disabled())
            .unwrap();
        game.process_player_command(GameCommand::Wait);
        assert!(
            game.telegraphed_attack_cells(worm)
                .iter()
                .any(|cell| cell.position == flank)
        );
        if protected {
            game.map.set_protected(flank, true).unwrap();
        } else {
            game.map.set_terrain(flank, Terrain::Wall).unwrap();
        }
        assert!(
            !game
                .telegraphed_attack_cells(worm)
                .iter()
                .any(|cell| cell.position == flank)
        );
        game.process_player_command(GameCommand::Wait);
        assert_eq!(game.actors.get(occupant).unwrap().integrity(), 20);
    }
}

#[test]
fn sweeper_content_requires_a_short_melee_cone_with_sight_and_recovery() {
    use crate::content::PopulationGroupDefinition;
    let ai = AiProfile::new(AiBehavior::TelegraphedSweeper, 7, 0, 256, 0);
    let valid = |attack| PopulationGroupDefinition::new(1, 20, 14, attack, ai, None).is_ok();
    assert!(valid(sweep()));
    assert!(!valid(sweep().with_area(AttackArea::Single)));
    assert!(!valid(sweep().with_delivery(AttackDelivery::Ranged)));
    assert!(!valid(
        sweep().with_area(AttackArea::Cone(ConeAttack::new(1, 3, 1).unwrap()))
    ));
    assert!(!valid(
        AttackProfile::new(
            2,
            DistanceMetric::Chebyshev,
            true,
            DamageType::Kinetic,
            4,
            0
        )
        .with_delivery(AttackDelivery::Melee)
        .with_area(AttackArea::Cone(ConeAttack::new(1, 1, 1).unwrap()))
    ));
    assert!(!valid(sweep().with_recovery_after_attack(
        crate::time::TimeUnits::new(9).unwrap()
    )));
}
