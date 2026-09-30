use super::*;

#[test]
fn level_health_legacy_rules_fingerprint() {
    let (rules, _, _, _) = ascii_game_content().unwrap();
    assert_eq!(
        rules_fingerprint_for_version(&rules, 121),
        6_137_039_013_056_757_832
    );
    assert!(!rules_for_generation_version(rules.clone(), 121).player_full_heal_on_level_up);
    assert!(rules_for_generation_version(rules.clone(), 122).player_full_heal_on_level_up);
    assert_ne!(
        rules_fingerprint_for_version(&rules, 121),
        rules_fingerprint_for_version(&rules, 122)
    );
    assert_eq!(
        rules_fingerprint_for_version(&rules, 120),
        6_198_306_336_801_966_463
    );
    assert_eq!(
        rules_for_generation_version(rules.clone(), 120).player_hit_points_per_level,
        0
    );
    assert_eq!(
        rules_for_generation_version(rules.clone(), 121).player_hit_points_per_level,
        3
    );
    assert_ne!(
        rules_fingerprint_for_version(&rules, 120),
        rules_fingerprint_for_version(&rules, 121)
    );
}

#[test]
fn level_health_current_content_and_feedback_use_the_new_rule() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut app =
        AsciiApp::from_seed_version(INITIAL_SEED, rules.clone(), texts, loot, expeditions, 122)
            .unwrap();
    let mut rules = rules;
    rules.hit_rules = None;
    rules.progression.curve = project_rl::progression::ExperienceCurve::new(vec![1]).unwrap();
    let mut game = GameState::new_with_rules(
        project_rl::world::Map::from_ascii("#####\n#...#\n#####").unwrap(),
        GridPos::new(1, 1),
        0,
        rules.clone(),
    )
    .unwrap();
    let enemy = game
        .spawn_actor(
            Actor::new(GridPos::new(2, 1), 1)
                .unwrap()
                .with_defeat_reward(project_rl::progression::DefeatReward::persistent(10, 1)),
        )
        .unwrap();
    game.drain_events();
    assert_eq!(
        game.process_player_command(GameCommand::Attack {
            slot: 0,
            target: enemy
        }),
        CommandOutcome::Applied
    );
    let actor = game.actors().get(game.player_id()).unwrap();
    assert_eq!((actor.integrity(), actor.maximum_integrity()), (23, 23));
    app.rules = rules;
    app.game = WorldState::single(game);
    app.capture_events();
    assert!(app.log.iter().any(|line| line.contains("PV maximum +3")));
}
