use super::*;
use project_rl::game::ZoneInfo;
use project_rl::world::{Map, Terrain};

fn app() -> AsciiApp {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap()
}

fn blueprint() -> ZoneBlueprint {
    ZoneBlueprint {
        info: ZoneInfo {
            id: "core:loot_probe".parse().unwrap(),
            name: "Essai".into(),
            kind: "core:human_habitat".parse().unwrap(),
            depth: 0,
        },
        map: Map::filled(12, 8, Terrain::Floor).unwrap(),
        entrance: GridPos::new(1, 1),
        seed: 108,
        actors: [
            ("core:humanoid_rifle_carrier", 3),
            ("core:humanoid_knife_carrier", 5),
        ]
        .into_iter()
        .map(|(tag, x)| {
            Actor::new(GridPos::new(x, 3), 8)
                .unwrap()
                .with_tags([tag.parse().unwrap()])
        })
        .chain([Actor::new(GridPos::new(7, 3), 8).unwrap().with_attack(
            project_rl::combat::AttackProfile::melee(project_rl::combat::DamageType::Kinetic, 2),
        )])
        .collect(),
        loot: vec![],
        threat_sources: vec![],
    }
}

#[test]
fn carried_weapons_are_explicit_bounded_varied_and_independent_of_actor_order() {
    let app = app();
    let mut enchanted = 0;
    for seed in 0..192 {
        let mut zone = blueprint();
        let natural = zone.actors[2].clone();
        app.populate_carried_weapons(&mut zone, seed).unwrap();
        assert_eq!(zone.actors[2], natural);
        for (actor, expected) in zone.actors[..2]
            .iter()
            .zip(["core:fusil_de_patrouille", "core:couteau_de_camp"])
        {
            let weapon = actor.equipped_weapon().unwrap();
            assert_eq!(weapon.item().as_str(), expected);
            actor.validate_equipped_weapon(&app.rules.weapons).unwrap();
            if let Some(bonus) = weapon.modifiers() {
                enchanted += 1;
                assert_eq!(bonus.energy_capacity_bonus(), 0);
                assert_eq!(bonus.heat_dissipation_bonus(), 0);
                assert!(
                    bonus
                        .named_affixes()
                        .is_none_or(|rolls| rolls.iter().all(|roll| roll.tier() == 1))
                );
            }
        }
        let mut reversed = blueprint();
        reversed.actors.reverse();
        app.populate_carried_weapons(&mut reversed, seed).unwrap();
        reversed.actors.reverse();
        assert_eq!(zone.actors, reversed.actors);
    }
    assert!(
        (90..190).contains(&enchanted),
        "white and enchanted weapons: {enchanted}/384"
    );
    let mut invalid = blueprint();
    invalid.actors[0] = invalid.actors[0]
        .clone()
        .with_tags(["core:humanoid_knife_carrier".parse().unwrap()]);
    assert!(app.populate_carried_weapons(&mut invalid, 8).is_err());
}

#[test]
fn authored_carriers_are_only_the_two_humanoid_surface_roles() {
    let app = app();
    let world = app
        .regional_worlds
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap();
    let mut carriers = 0;
    for biome in ["core:human_habitat", "core:surface_wilds"] {
        let biome = world.biome(&biome.parse().unwrap()).unwrap();
        for rule in biome.encounters().rules() {
            let tagged = rule
                .tags()
                .iter()
                .any(|tag| tag.as_str().starts_with("core:humanoid_"));
            let expected = matches!(
                rule.ai().behavior,
                project_rl::ai::AiBehavior::TelegraphedShooter
                    | project_rl::ai::AiBehavior::FieldMedic { .. }
            );
            assert_eq!(tagged, expected);
            if tagged {
                carriers += 1;
                assert!(rule.electronic_system().is_none());
            }
        }
    }
    assert_eq!(carriers, 4);
}

#[test]
fn actual_kill_feedback_and_pickup_show_the_same_generated_weapon() {
    let mut app = app();
    app.prepare_enemy_loot_diagnostic(false).unwrap();
    let item = app.game.ground_items().iter().next().unwrap().1;
    let expected_item = item.item().clone();
    let expected_bonus = item.magic_modifiers();
    assert!(expected_bonus.is_some());
    assert!(app.log.iter().any(|line| line.starts_with("Butin : ")));
    app.execute_command(GameCommand::Move(Direction::East));
    app.execute_command(GameCommand::PickUp);
    assert_eq!(app.game.ground_items().iter().count(), 0);
    let found = app
        .game
        .player_inventory()
        .iter()
        .find(|item| item.item() == &expected_item)
        .unwrap();
    assert_eq!(found.magic_modifiers(), expected_bonus);
}

#[test]
fn regional_carriers_survive_real_travel_snapshot_and_journal_replay() {
    let mut app = app();
    let passage = TestSector::EXPANDED_REGIONAL_PASSAGES
        .into_iter()
        .find_map(|(direction, at)| (direction == Direction::West).then_some(at))
        .unwrap();
    app.walk_fixture_to(passage.step(Direction::East)).unwrap();
    app.execute_command(GameCommand::Interact { target: passage });
    app.capture_events();
    let carriers: Vec<_> = app
        .game
        .actors()
        .iter()
        .filter_map(|(id, actor)| actor.equipped_weapon().map(|weapon| (id, weapon.clone())))
        .collect();
    assert!(
        !carriers.is_empty(),
        "the initial western region should contain an authored carrier"
    );
    let saved = app.suspension().unwrap();
    let restored = AsciiApp::restore_suspension(
        &saved,
        app.rules.clone(),
        app.texts.clone(),
        app.loot.clone(),
        app.expeditions.clone(),
    )
    .unwrap();
    assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    for (id, weapon) in carriers {
        assert_eq!(
            restored.game.actors().get(id).unwrap().equipped_weapon(),
            Some(&weapon)
        );
    }
    let mut world = app.game;
    world.drain_events();
    let resumed = WorldState::from_recovery_snapshot_bytes(
        &world.recovery_snapshot_bytes().unwrap(),
        world.rules().clone(),
    )
    .unwrap();
    assert_eq!(format!("{world:?}"), format!("{resumed:?}"));
}
