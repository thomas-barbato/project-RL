use super::*;

fn combat_rules(class_id: &str) -> GameRules {
    let (mut rules, _, _, _) = ascii_game_content().unwrap();
    let classes = ascii_character_class_catalog().unwrap();
    let class = classes.get(&class_id.parse().unwrap()).unwrap();
    class
        .apply_to_rules(&mut rules, class.recommended_attributes())
        .unwrap();
    rules_for_generation_version(rules, CURRENT_GENERATION_VERSION)
}

fn sewer_enemy(catalog: &RegionalWorldCatalog, number: u16, level: u16) -> Actor {
    let plan: crate::sewer::Plan = serde_json::from_value(serde_json::json!({
        "seed":1,"width":17,"height":13,"spawn":[2,6],
        "layout":vec![".................";13],"views":[],"groups":[{
            "id":1,"family":"diagnostic","members":[{"at":[8,6],"number":number,"variant":0,"level":level}]
        }]
    }))
    .unwrap();
    crate::sewer::actors(&plan, catalog).unwrap().remove(0)
}

fn attempts(game: &mut GameState, enemy: EntityId) -> usize {
    let player = game.player_id();
    game.drain_events()
        .into_iter()
        .filter(|event| {
            matches!(event, GameEvent::AttackHitResolved { attacker, target, .. }
            if *attacker == enemy && *target == player)
        })
        .count()
}

#[test]
fn campaign_aggression_real_monsters_detect_and_attack_every_unhidden_starting_class() {
    let catalog = ascii_regional_world_catalog().unwrap();
    let mut failures = Vec::new();
    for class in [
        "core:bastion",
        "core:technomancien",
        "core:traqueur",
        "core:arcaniste",
        "core:metamorphe",
    ] {
        let rules = combat_rules(class);
        for level in [1, 2] {
            for number in [22, 24] {
                for distance in [1, 3, 6] {
                    let actor = sewer_enemy(&catalog, number, level);
                    let mut game = GameState::new_with_rules(
                        project_rl::world::Map::filled(17, 13, project_rl::world::Terrain::Floor)
                            .unwrap(),
                        GridPos::new(8 - distance, 6),
                        1872,
                        rules.clone(),
                    )
                    .unwrap();
                    let enemy = game.spawn_actor(actor).unwrap();
                    let detected = game.actor_perception_signals(enemy).unwrap().0;
                    let mut count = 0;
                    for _ in 0..8 {
                        game.process_player_command(GameCommand::Wait);
                        count += attempts(&mut game, enemy);
                        if count > 0 {
                            break;
                        }
                    }
                    if !detected || count == 0 {
                        failures.push(format!(
                            "{class}, enemy {number}, level {level}, distance {distance}: detected={detected}, attacks={count}"
                        ));
                    }
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn campaign_aggression_actual_city_groups_attack_the_traqueur_at_six_cells() {
    let _scope = crate::street_population::Scope::new();
    let app = AsciiApp::new_street_population_trial().unwrap();
    let city = crate::street_population::actors(&app.regional_worlds, app.game.map()).unwrap();
    let rules = combat_rules("core:traqueur");
    for actor in city {
        let at = actor.position();
        let mut game = GameState::new_with_rules(
            project_rl::world::Map::filled(40, 80, project_rl::world::Terrain::Floor).unwrap(),
            GridPos::new(at.x + 6, at.y),
            1872,
            rules.clone(),
        )
        .unwrap();
        let enemy = game.spawn_actor(actor).unwrap();
        assert!(game.actor_perception_signals(enemy).unwrap().0);
        let mut count = 0;
        for _ in 0..8 {
            game.process_player_command(GameCommand::Wait);
            count += attempts(&mut game, enemy);
            if count > 0 {
                break;
            }
        }
        assert!(count > 0, "city enemy at {at:?} stayed passive");
    }
}

#[test]
fn campaign_aggression_visible_ranged_enemy_keeps_fighting_past_old_pursuit_timeout() {
    let catalog = ascii_regional_world_catalog().unwrap();
    let mut game = GameState::new_with_rules(
        project_rl::world::Map::filled(17, 13, project_rl::world::Terrain::Floor).unwrap(),
        GridPos::new(5, 6),
        1872,
        combat_rules("core:bastion"),
    )
    .unwrap();
    let enemy = game.spawn_actor(sewer_enemy(&catalog, 24, 1)).unwrap();
    game.drain_events();
    for turn in 1..=24 {
        assert_eq!(game.status(), RunStatus::Active);
        game.process_player_command(GameCommand::Wait);
        assert_eq!(
            attempts(&mut game, enemy),
            1,
            "stopped attacking on turn {turn}"
        );
    }
}
