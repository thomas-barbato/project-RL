use super::*;
use crate::combat::DamageType;
use crate::entity::MagicItemModifiers;
use crate::progression::ExperienceCurve;
use crate::stats::{BodyProfile, PhysicalRules};

fn fixture(physical: bool) -> GameState {
    let mut rules = GameRules {
        player_hit_points_per_level: 3,
        player_full_heal_on_level_up: true,
        ..GameRules::default()
    };
    rules.progression.curve = ExperienceCurve::new(vec![10, 20, 30]).unwrap();
    if physical {
        rules.physical_rules = Some(PhysicalRules::default());
        rules.player_body_profile = Some(BodyProfile::new(15, 0).unwrap());
    }
    GameState::new_with_rules(
        Map::from_ascii("#####\n#...#\n#####").unwrap(),
        GridPos::new(1, 1),
        42,
        rules,
    )
    .unwrap()
}

fn health(game: &GameState) -> (u16, u16) {
    let actor = game.actors.get(game.player).unwrap();
    (actor.integrity(), actor.maximum_integrity())
}

fn award(game: &mut GameState, amount: u64) {
    game.apply_experience_award(
        &ExperienceAward::repeatable(amount),
        ExperienceSource::DefeatedEntity(EntityId::new(99)),
    );
}

#[test]
fn level_health_growth_fully_heals_and_only_triggers_on_levels() {
    for physical in [false, true] {
        let mut game = fixture(physical);
        game.actors.get_mut(game.player).unwrap().apply_damage(8);
        let attributes = game.player_primary_attributes();
        assert_eq!(health(&game), (12, 20));
        award(&mut game, 9);
        assert_eq!(health(&game), (12, 20));
        game.drain_events();
        award(&mut game, 1);
        assert_eq!(health(&game), (23, 23));
        assert_eq!(game.player_level_hit_point_bonus(), 3);
        assert_eq!(game.player_primary_attributes(), attributes);
        assert!(game.events().contains(&GameEvent::IntegrityRestored {
            entity: game.player,
            amount: 11
        }));
        game.actors.get_mut(game.player).unwrap().apply_damage(4);
        game.refresh_equipment_resources();
        award(&mut game, 1);
        assert_eq!(health(&game), (19, 23));
    }
}

#[test]
fn level_health_multiple_levels_and_duplicate_rewards_are_exact() {
    let mut game = fixture(true);
    game.actors.get_mut(game.player).unwrap().apply_damage(8);
    let key = RewardKey::new("test:level_health").unwrap();
    game.award_one_time_experience(20, key.clone());
    assert_eq!(game.player_progression.level(), 3);
    assert_eq!(health(&game), (26, 26));
    game.actors.get_mut(game.player).unwrap().apply_damage(5);
    game.drain_events();
    game.award_one_time_experience(20, key);
    assert_eq!(health(&game), (21, 26));
    assert!(game.events().is_empty());
    game.refresh_equipment_resources();
    assert_eq!(health(&game), (21, 26));
}

#[test]
fn level_health_legacy_partial_reward_is_preserved() {
    let mut game = fixture(true);
    game.rules.player_full_heal_on_level_up = false;
    game.actors.get_mut(game.player).unwrap().apply_damage(8);
    award(&mut game, 10);
    assert_eq!(health(&game), (15, 23));
    assert!(!format!("{:?}", game.rules).contains("player_full_heal_on_level_up"));
}

#[test]
fn level_health_full_heal_also_works_without_capacity_growth() {
    let mut game = fixture(false);
    game.rules.player_hit_points_per_level = 0;
    game.actors.get_mut(game.player).unwrap().apply_damage(8);
    award(&mut game, 10);
    assert_eq!(health(&game), (20, 20));
}

#[test]
fn level_health_does_not_resurrect_but_heals_at_the_numeric_cap() {
    let mut game = fixture(false);
    game.actors.get_mut(game.player).unwrap().apply_damage(20);
    game.status = RunStatus::PlayerDestroyed;
    award(&mut game, 10);
    assert_eq!(health(&game), (0, 23));
    assert_eq!(game.status, RunStatus::PlayerDestroyed);
    assert!(
        !game
            .events()
            .iter()
            .any(|e| matches!(e, GameEvent::IntegrityRestored { .. }))
    );

    let mut game = fixture(false);
    game.rules.player_maximum_integrity = u16::MAX - 1;
    game.rules.player_hit_points_per_level = u16::MAX;
    game.refresh_equipment_resources();
    assert_eq!(health(&game), (20, u16::MAX - 1));
    award(&mut game, 10);
    assert_eq!(health(&game), (u16::MAX, u16::MAX));
    game.actors.get_mut(game.player).unwrap().apply_damage(7);
    award(&mut game, 10);
    assert_eq!(health(&game), (u16::MAX, u16::MAX));
}

#[test]
fn level_health_disabled_rules_keep_historical_behavior() {
    let mut game = fixture(true);
    game.rules.player_hit_points_per_level = 0;
    game.rules.player_full_heal_on_level_up = false;
    game.actors.get_mut(game.player).unwrap().apply_damage(8);
    award(&mut game, 30);
    assert_eq!(health(&game), (12, 20));
    assert_eq!(game.player_level_hit_point_bonus(), 0);
    assert!(!format!("{:?}", game.rules).contains("player_hit_points_per_level"));
}

#[test]
fn level_health_progression_import_recalculates_capacity_without_healing() {
    let mut advanced = fixture(true);
    award(&mut advanced, 20);
    let saved = advanced.export_player_progression().unwrap();
    let mut game = fixture(true);
    let initial = game.export_player_progression().unwrap();
    game.actors.get_mut(game.player).unwrap().apply_damage(8);
    game.drain_events();
    for _ in 0..2 {
        game.restore_player_progression(&saved).unwrap();
        assert_eq!(health(&game), (12, 26));
        assert!(game.events().is_empty());
    }
    assert!(game.restore_player_progression("invalid").is_err());
    assert_eq!(health(&game), (12, 26));
    game.restore_player_progression(&initial).unwrap();
    assert_eq!(health(&game), (12, 20));
}

#[test]
fn level_health_real_kill_replays_with_identical_health_and_events() {
    let make = || {
        let mut game = fixture(true);
        game.actors.get_mut(game.player).unwrap().apply_damage(8);
        let enemy = game
            .spawn_actor(
                Actor::new(GridPos::new(2, 1), 1)
                    .unwrap()
                    .with_defeat_reward(DefeatReward::persistent(10, 1)),
            )
            .unwrap();
        game.drain_events();
        (game, enemy)
    };
    let (mut live, enemy) = make();
    let (mut replay, replay_enemy) = make();
    assert_eq!(enemy, replay_enemy);
    let command = GameCommand::Attack {
        slot: 0,
        target: enemy,
    };
    assert_eq!(
        live.process_player_command(command.clone()),
        CommandOutcome::Applied
    );
    assert_eq!(
        replay.process_player_command(command),
        CommandOutcome::Applied
    );
    assert_eq!(health(&live), (23, 23));
    assert_eq!(health(&replay), health(&live));
    assert_eq!(live.drain_events(), replay.drain_events());
}

#[test]
fn level_health_stacks_with_resilience_and_gear_without_swap_healing() {
    let mut rules = fixture(true).rules.clone();
    let weapon: WeaponId = "test:health_blade".parse().unwrap();
    rules
        .weapons
        .register(
            WeaponDefinition::new(
                weapon.clone(),
                "name".into(),
                "description".into(),
                AttackProfile::melee(DamageType::Kinetic, 3),
            )
            .unwrap(),
        )
        .unwrap();
    rules.player_weapon_slots = vec!["test:hand".parse().unwrap()];
    rules.player_starting_weapons = vec![weapon.clone()];
    rules.player_starting_equipment = vec![Some(weapon.clone())];
    let mut game = GameState::new_with_rules(
        Map::filled(5, 3, Terrain::Floor).unwrap(),
        GridPos::new(1, 1),
        0,
        rules,
    )
    .unwrap()
    .with_starting_magic_equipment([(
        weapon,
        MagicItemModifiers::rpg_bonuses([0, 0, 1, 0, 0], 0, 0, 7, 0, 0).unwrap(),
    )])
    .unwrap();
    let plain = game
        .player_inventory
        .iter()
        .find(|e| e.magic_modifiers().is_none())
        .unwrap()
        .instance();
    let magic = game
        .player_inventory
        .iter()
        .find(|e| e.magic_modifiers().is_some())
        .unwrap()
        .instance();
    game.equip_player_weapon(0, magic).unwrap();
    assert_eq!(health(&game), (20, 32));
    game.actors.get_mut(game.player).unwrap().apply_damage(8);
    award(&mut game, 10);
    assert_eq!(health(&game), (35, 35));
    game.actors.get_mut(game.player).unwrap().apply_damage(20);
    game.equip_player_weapon(0, plain).unwrap();
    assert_eq!(health(&game), (15, 23));
    game.equip_player_weapon(0, magic).unwrap();
    assert_eq!(health(&game), (15, 35));
    game.drain_events();
    let snapshot = game.snapshot().unwrap();
    let mut restored = GameState::from_snapshot(snapshot, game.rules.clone());
    restored.refresh_equipment_resources();
    assert_eq!(health(&restored), (15, 35));
    award(&mut game, 10);
    award(&mut restored, 10);
    assert_eq!(health(&restored), (38, 38));
    assert_eq!(health(&game), health(&restored));
    assert_eq!(game.drain_events(), restored.drain_events());
}
