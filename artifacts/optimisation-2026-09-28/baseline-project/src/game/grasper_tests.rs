use super::*;
use crate::combat::{AttackArea, AttackDelivery, DamageType};
use crate::stats::{BodyProfile, LocomotionProfile};
use crate::world::DistanceMetric;

fn grasp() -> AttackProfile {
    AttackProfile::new(
        1,
        DistanceMetric::Chebyshev,
        true,
        DamageType::Kinetic,
        3,
        0,
    )
    .with_delivery(AttackDelivery::Melee)
    .with_area(AttackArea::Adjacent)
    .with_recovery_after_attack(crate::time::TimeUnits::new(3).unwrap())
}

fn arena() -> (GameState, EntityId) {
    static STATUSES: std::sync::OnceLock<crate::status::StatusCatalog> = std::sync::OnceLock::new();
    let statuses = STATUSES
        .get_or_init(|| {
            crate::content::ContentLoader::load(
                &[std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content")],
                &semver::Version::new(0, 1, 0),
            )
            .unwrap()
            .statuses()
            .clone()
        })
        .clone();
    let map = Map::from_ascii(
        "###########\n#.........#\n#.........#\n#.........#\n#.........#\n#.........#\n#.........#\n#.........#\n###########"
    ).unwrap();
    let rules = GameRules {
        statuses,
        ..GameRules::default()
    };
    let mut game = GameState::new_with_rules(map, GridPos::new(4, 4), 17, rules).unwrap();
    let player = game.actors.get_mut(game.player).unwrap();
    *player = player.clone().with_evasion_disabled().with_body_profile(
        BodyProfile::new(20, 0)
            .unwrap()
            .with_locomotion_profile(LocomotionProfile::new(true)),
    );
    let animal = game
        .spawn_actor(
            Actor::new(GridPos::new(5, 4), 18)
                .unwrap()
                .with_ai(AiProfile::new(AiBehavior::TelegraphedGrasper, 4, 0, 256, 0))
                .with_attack(grasp())
                .with_player_relation(PlayerRelation::Hostile),
        )
        .unwrap();
    (game, animal)
}

#[test]
fn grasper_announces_eight_neighbors_and_only_entraves_real_surviving_hits() {
    let (mut game, animal) = arena();
    let bystander = game
        .spawn_actor(
            Actor::new(GridPos::new(5, 3), 20)
                .unwrap()
                .with_evasion_disabled()
                .with_body_profile(
                    BodyProfile::new(20, 0)
                        .unwrap()
                        .with_locomotion_profile(LocomotionProfile::new(true)),
                ),
        )
        .unwrap();
    let inert = game
        .spawn_actor(
            Actor::new(GridPos::new(6, 4), 20)
                .unwrap()
                .with_evasion_disabled(),
        )
        .unwrap();
    game.process_player_command(GameCommand::Wait);
    let preview = game.telegraphed_attack_cells(animal);
    assert_eq!(preview.len(), 8);
    game.drain_events();
    game.process_player_command(GameCommand::Wait);
    let status: StatusId = "core:locomotion_hindered".parse().unwrap();
    for target in [game.player, bystander] {
        assert_eq!(game.actors.get(target).unwrap().integrity(), 17);
        assert!(game.actors.get(target).unwrap().status(&status).is_some());
    }
    assert_eq!(game.actors.get(inert).unwrap().integrity(), 17);
    assert!(game.actors.get(inert).unwrap().status(&status).is_none());
    let attacks: Vec<_> = game
        .drain_events()
        .into_iter()
        .filter_map(|event| match event {
            GameEvent::AttackPerformed {
                attacker,
                affected_cells,
                ..
            } if attacker == animal => Some(affected_cells),
            _ => None,
        })
        .collect();
    assert_eq!(attacks, vec![preview]);
}

#[test]
fn grasper_can_be_avoided_and_never_chases_during_or_after_recovery() {
    let (mut game, animal) = arena();
    game.process_player_command(GameCommand::Wait);
    game.process_player_command(GameCommand::Move(Direction::West));
    assert_eq!(game.actors.get(game.player).unwrap().integrity(), 20);
    for remaining in [Some(3), Some(2), Some(1), None, None, None] {
        let actor = game.actors.get(animal).unwrap();
        assert_eq!(actor.position(), GridPos::new(5, 4));
        assert_eq!(actor.recovery_remaining().map(|time| time.get()), remaining);
        assert!(game.telegraphed_attack_cells(animal).is_empty());
        game.process_player_command(GameCommand::Wait);
    }
}

#[test]
fn grasper_does_not_stack_or_refresh_and_leaves_a_protection_window() {
    let (mut game, animal) = arena();
    game.process_player_command(GameCommand::Wait);
    game.process_player_command(GameCommand::Wait);
    let status: StatusId = "core:locomotion_hindered".parse().unwrap();
    let before = game
        .actors
        .get(game.player)
        .unwrap()
        .status(&status)
        .unwrap()
        .clone();
    // A second source's hit in the same phase must not stack or refresh it.
    game.perform_committed_attack(animal, 0, GridPos::new(4, 4))
        .unwrap();
    assert_eq!(
        game.actors
            .get(game.player)
            .unwrap()
            .status(&status)
            .unwrap(),
        &before
    );
    game.process_player_command(GameCommand::Wait);
    let player = game.actors.get(game.player).unwrap();
    assert!(player.status(&status).is_none());
    assert!(
        player
            .status(&"core:locomotion_hindrance_protection".parse().unwrap())
            .is_some()
    );
    game.perform_committed_attack(animal, 0, GridPos::new(4, 4))
        .unwrap();
    assert!(
        game.actors
            .get(game.player)
            .unwrap()
            .status(&status)
            .is_none()
    );
}

#[test]
fn grasper_cannot_reach_through_closed_corners_or_protected_cells() {
    let (mut game, animal) = arena();
    game.map
        .set_terrain(GridPos::new(5, 3), Terrain::Wall)
        .unwrap();
    game.map
        .set_terrain(GridPos::new(6, 4), Terrain::Wall)
        .unwrap();
    let diagonal = GridPos::new(6, 3);
    let protected = GridPos::new(5, 5);
    game.map.set_protected(protected, true).unwrap();
    let target = game
        .spawn_actor(Actor::new(protected, 20).unwrap())
        .unwrap();
    game.process_player_command(GameCommand::Wait);
    let cells = game.telegraphed_attack_cells(animal);
    assert!(
        !cells
            .iter()
            .any(|cell| [diagonal, protected].contains(&cell.position))
    );
    game.process_player_command(GameCommand::Wait);
    assert_eq!(game.actors.get(target).unwrap().integrity(), 20);
}

#[test]
fn grasper_commitment_is_cancelled_by_displacement_or_new_cover() {
    for displaced in [false, true] {
        let (mut game, animal) = arena();
        if !displaced {
            game.actors
                .get_mut(game.player)
                .unwrap()
                .set_position(GridPos::new(4, 3));
        }
        game.process_player_command(GameCommand::Wait);
        if displaced {
            game.actors
                .get_mut(animal)
                .unwrap()
                .set_position(GridPos::new(6, 4));
        } else {
            game.map
                .set_terrain(GridPos::new(4, 4), Terrain::Wall)
                .unwrap();
            game.map
                .set_terrain(GridPos::new(5, 3), Terrain::Wall)
                .unwrap();
        }
        assert!(game.telegraphed_attack_cells(animal).is_empty());
        game.process_player_command(GameCommand::Wait);
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), 20);
    }
}

#[test]
fn grasper_content_requires_neighboring_melee_and_a_release_window() {
    use crate::content::PopulationGroupDefinition;
    let valid = |attack, ai| PopulationGroupDefinition::new(1, 2, 18, attack, ai, None).is_ok();
    let ai = AiProfile::new(AiBehavior::TelegraphedGrasper, 4, 0, 256, 0);
    assert!(valid(grasp(), ai));
    assert!(!valid(grasp().with_area(AttackArea::Single), ai));
    assert!(!valid(grasp().with_delivery(AttackDelivery::Ranged), ai));
    assert!(!valid(
        grasp().with_recovery_after_attack(crate::time::TimeUnits::new(2).unwrap()),
        ai
    ));
    assert!(!valid(
        grasp(),
        ai.with_maximum_pursuit_distance(std::num::NonZeroU16::new(2).unwrap())
    ));
}

#[test]
fn grasper_hindrance_slows_escape_without_forbidding_it() {
    let (mut game, _) = arena();
    game.process_player_command(GameCommand::Wait);
    game.process_player_command(GameCommand::Wait);
    let turn = game.turn();
    assert_eq!(
        game.process_player_command(GameCommand::Move(Direction::West)),
        CommandOutcome::Applied
    );
    assert_eq!(game.player_position().unwrap(), GridPos::new(3, 4));
    assert_eq!(game.turn() - turn, 2);
}
