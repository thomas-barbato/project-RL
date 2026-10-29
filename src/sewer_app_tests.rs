use super::*;
use crate::{cryogenic_camp as camp, sewer};

#[test]
fn sewer_connected_generation_keeps_all_characters_mobile_after_entry_return_and_resume() {
    for index in 1..=3 {
        let mut app = AsciiApp::connected_sewer_fixture(index).unwrap();
        let mut zone_id = None;
        let mut occupants = Vec::new();
        for stage in 0..3 {
            if stage == 1 {
                enter(&mut app);
                assert!(sewer::connected_id(
                    app.game.current_zone().unwrap().id.as_str()
                ));
                zone_id = Some(app.game.current_zone().unwrap().id.clone());
                occupants = app
                    .game
                    .actors()
                    .iter()
                    .map(|(_, a)| a.position())
                    .collect();
                assert!(occupants.iter().all(|&at| app.game.map().is_walkable(at)));
            }
            if stage == 2 {
                let exit = crate::prison_street::pos(sewer::connected_plan(index).spawn);
                assert_eq!(
                    app.execute_command(GameCommand::Interact { target: exit }),
                    CommandOutcome::Applied
                );
            }
            app.capture_events_at(Some(0.));
            let saved = app.suspension().unwrap();
            let (r, t, l, e) = ascii_game_content().unwrap();
            let restored = AsciiApp::restore_suspension(&saved, r, t, l, e).unwrap();
            assert_eq!(
                suspension::fingerprint(&app.game),
                suspension::fingerprint(&restored.game),
                "variant {index}, stage {stage}"
            );
            if let Some(id) = &zone_id {
                let map = restored.game.map_in_zone(id).unwrap();
                assert!(occupants.iter().all(|&at| map.is_walkable(at)));
            }
        }
    }
}

#[test]
fn sewer_legacy_resume_never_floods_the_player_or_a_monster() {
    let mut app =
        AsciiApp::isolated_sewer_fixture(1, CONNECTED_SEWER_GENERATION_VERSION - 1).unwrap();
    enter(&mut app);
    let old = sewer::plan(1);
    let new = sewer::connected_plan(1);
    let additions: Vec<_> = new
        .layout
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.bytes().enumerate().filter_map(move |(x, c)| {
                (c == b'~' && old.layout[y].as_bytes()[x] == b'.')
                    .then_some(GridPos::new(x as i32, y as i32))
            })
        })
        .collect();
    let info = app.game.current_zone().unwrap().clone();
    let game = GameState::new_with_rules(
        app.game.map().clone(),
        additions[0],
        app.seed,
        app.game.rules().clone(),
    )
    .unwrap();
    app.game = WorldState::single(game);
    app.game.enable(info).unwrap();
    let monster = app
        .game
        .spawn_actor(
            Actor::new(additions[1], 40)
                .unwrap()
                .with_player_relation(PlayerRelation::Neutral),
        )
        .unwrap();
    app.capture_events_at(Some(0.));
    let saved = app.suspension().unwrap();
    let (r, t, l, e) = ascii_game_content().unwrap();
    let restored = AsciiApp::restore_suspension(&saved, r, t, l, e).unwrap();
    assert!(!sewer::connected_id(
        restored.game.current_zone().unwrap().id.as_str()
    ));
    assert_eq!(restored.game.player_position(), Some(additions[0]));
    assert_eq!(
        restored.game.actors().get(monster).unwrap().position(),
        additions[1]
    );
    assert!(restored.game.map().is_walkable(additions[0]));
    assert!(restored.game.map().is_walkable(additions[1]));
    assert_eq!(
        suspension::fingerprint(&app.game),
        suspension::fingerprint(&restored.game)
    );
}

#[test]
fn sewer_hostile_combat_diagnostic_uses_real_starting_defenses() {
    let app = AsciiApp::sewer_trial_variant(1).unwrap();
    let (mut rules, _, _, _) = ascii_game_content().unwrap();
    let classes = ascii_character_class_catalog().unwrap();
    let class = classes.get(&"core:bastion".parse().unwrap()).unwrap();
    class
        .apply_to_rules(&mut rules, class.recommended_attributes())
        .unwrap();
    let mut rows = Vec::new();
    for (version, level, number) in [
        (163, 1, 22),
        (163, 1, 24),
        (163, 2, 22),
        (163, 2, 24),
        (164, 1, 22),
        (164, 1, 24),
        (164, 2, 22),
        (164, 2, 24),
    ] {
        let plan: sewer::Plan = serde_json::from_value(serde_json::json!({
            "seed":1,"width":11,"height":9,"spawn":[3,4],
            "layout":vec!["...........";9],"views":[],"groups":[{
                "id":1,"family":"diagnostic","members":[{"at":[4,4],"number":number,"variant":0,"level":level}]
            }]
        })).unwrap();
        let mut game = GameState::new_with_rules(
            project_rl::world::Map::filled(11, 9, project_rl::world::Terrain::Floor).unwrap(),
            GridPos::new(3, 4),
            1872,
            rules_for_generation_version(rules.clone(), version),
        )
        .unwrap();
        let actor = sewer::actors(&plan, &app.regional_worlds)
            .unwrap()
            .remove(0);
        let raw = actor.attack(0).unwrap().damage().raw_total();
        let armor = game
            .actor_armor_profile(game.player_id())
            .unwrap()
            .effective_against(0);
        game.spawn_actor(actor).unwrap();
        game.drain_events();
        let mut attempts = 0;
        let mut hits = 0;
        let mut damage = 0;
        let mut absorbed = 0;
        let mut chances = std::collections::BTreeSet::new();
        let initial_hp = game.actors().get(game.player_id()).unwrap().integrity();
        for _ in 0..32 {
            if game.status() != RunStatus::Active {
                break;
            }
            game.process_player_command(GameCommand::Wait);
            for event in game.drain_events() {
                match event {
                    GameEvent::AttackHitResolved {
                        target,
                        chance,
                        hit,
                        ..
                    } if target == game.player_id() => {
                        attempts += 1;
                        hits += u32::from(hit);
                        chances.insert(chance);
                    }
                    GameEvent::DamageApplied {
                        target,
                        amount,
                        absorbed_by_armor,
                        ..
                    } if target == game.player_id() => {
                        damage += u32::from(amount);
                        absorbed += u32::from(absorbed_by_armor);
                    }
                    _ => {}
                }
            }
        }
        assert!(
            attempts > 0 && hits > 0,
            "enemy {number} never attacked or hit"
        );
        let hp_lost = initial_hp
            - game
                .actors()
                .get(game.player_id())
                .map_or(0, Actor::integrity);
        if version == 164 && number == 22 {
            assert!(
                hp_lost > 0,
                "original melee enemy {number} level {level} still does no damage"
            );
        }
        rows.push(serde_json::json!({"generation":version,"armor":armor,"level":level,"monster":number,"raw_damage":raw,"attempts":attempts,"hits":hits,
            "chance_percent":chances,"hp_lost":hp_lost,"damage_sum":damage,"armor_absorbed":absorbed}));
    }
    let folder = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("artifacts/combat-fixes-01");
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::write(
        folder.join("starter-skin-02.json"),
        serde_json::to_vec_pretty(&rows).unwrap(),
    )
    .unwrap();
    eprintln!("{rows:?}");
}

fn enter(app: &mut AsciiApp) {
    assert_eq!(
        app.game.current_zone().unwrap().id.as_str(),
        crate::prison_street::ZONE_ID
    );
    assert!(app.interaction_candidates().contains(&sewer::ENTRY));
    assert!(
        matches!(app.interaction_command_at(sewer::ENTRY),Some(GameCommand::Interact{target}) if target==sewer::ENTRY)
    );
    assert_eq!(
        app.execute_command(GameCommand::Interact {
            target: sewer::ENTRY
        }),
        CommandOutcome::Applied
    );
}

#[test]
fn sewer_all_three_playable_variants_have_real_population_and_bidirectional_return() {
    for (index, count) in [(1, 85), (2, 104), (3, 78)] {
        let mut app = AsciiApp::sewer_trial_variant(index).unwrap();
        assert_eq!(app.generation_version, 162);
        enter(&mut app);
        assert_eq!(sewer::index(&app.game), Some(index));
        assert_eq!(app.game.actors().iter().count(), count + 1);
        assert_eq!(
            app.game.player_position(),
            Some(crate::prison_street::pos(sewer::plan(index).spawn))
        );
        let ids: Vec<_> = app.game.actors().iter().map(|(id, _)| id).collect();
        let exit = crate::prison_street::pos(sewer::plan(index).spawn);
        assert!(
            !app.game
                .passage_is_irreversible(app.game.passage(exit).unwrap())
        );
        assert_eq!(
            app.execute_command(GameCommand::Interact { target: exit }),
            CommandOutcome::Applied
        );
        assert_eq!(app.game.player_position(), Some(sewer::ENTRY));
        enter(&mut app);
        assert_eq!(
            app.game
                .actors()
                .iter()
                .map(|(id, _)| id)
                .collect::<Vec<_>>(),
            ids
        );
    }
}

#[test]
fn sewer_restore_preserves_water_collisions_and_pending_or_inactive_zones() {
    let mut app = AsciiApp::sewer_trial_variant(1).unwrap();
    for stage in 0..3 {
        if stage == 1 {
            enter(&mut app);
        }
        if stage == 2 {
            let exit = crate::prison_street::pos(sewer::plan(1).spawn);
            assert_eq!(
                app.execute_command(GameCommand::Interact { target: exit }),
                CommandOutcome::Applied
            );
        }
        app.capture_events_at(Some(0.));
        let saved = app.suspension().unwrap();
        let (r, t, l, e) = ascii_game_content().unwrap();
        let restored = AsciiApp::restore_suspension(&saved, r, t, l, e).unwrap();
        assert_eq!(
            suspension::fingerprint(&app.game),
            suspension::fingerprint(&restored.game),
            "stage {stage}"
        );
        assert!(
            restored
                .game
                .map_in_zone(&crate::prison_street::ZONE_ID.parse().unwrap())
                .unwrap()
                .is_walkable(sewer::ENTRY)
        );
    }
}

#[test]
fn sewer_legacy_generation_keeps_the_original_street_and_no_passage() {
    let (r, t, l, e) = ascii_game_content().unwrap();
    let mut app = AsciiApp::from_seed_version(
        1872,
        r,
        t,
        l,
        e,
        STREET_WAREHOUSE_CLEARANCE_GENERATION_VERSION,
    )
    .unwrap();
    app.crash_recovery_enabled = false;
    let exit = crate::cryogenic_camp::pos(camp::scene().exits[0]);
    app.walk_fixture_to_unchecked(exit).unwrap();
    assert_eq!(
        app.execute_command(GameCommand::Interact { target: exit }),
        CommandOutcome::Applied
    );
    assert!(!sewer::has_street_entry(&app.game));
    assert!(sewer::scene(&app.game).is_none());
}

#[test]
fn sewer_invalid_remote_entry_does_not_materialize_or_consume_a_turn() {
    let mut app = AsciiApp::sewer_trial_variant(1).unwrap();
    app.walk_fixture_to_unchecked(GridPos::new(12, 16)).unwrap();
    let before = suspension::fingerprint(&app.game);
    assert_ne!(
        app.execute_command(GameCommand::Interact {
            target: sewer::ENTRY
        }),
        CommandOutcome::Applied
    );
    assert_eq!(before, suspension::fingerprint(&app.game));
}

#[test]
fn sewer_combat_and_loot_state_survive_snapshot() {
    let mut app = AsciiApp::sewer_trial_variant(1).unwrap();
    enter(&mut app);
    let origin = app.game.player_position().unwrap();
    let target = app
        .game
        .actors()
        .iter()
        .filter(|(id, _)| *id != app.game.player_id())
        .min_by_key(|(_, a)| a.position().x.abs_diff(origin.x) + a.position().y.abs_diff(origin.y))
        .unwrap()
        .0;
    let xp = app.game.player_progression().experience();
    for _ in 0..60 {
        let Some(actor) = app.game.actors().get(target).filter(|a| a.is_alive()) else {
            break;
        };
        let at = actor.position();
        let command = if app.game.player_targeted_attack_preview(0, target).is_ok() {
            GameCommand::Attack { slot: 0, target }
        } else {
            let start = app.game.player_position().unwrap();
            let path = at
                .cardinal_neighbors()
                .into_iter()
                .filter(|p| {
                    app.game.map().is_walkable(*p) && app.game.actors().entity_at(*p).is_none()
                })
                .filter_map(|goal| {
                    project_rl::world::find_path(app.game.map(), start, goal, 15000, |p| {
                        app.game.actors().entity_at(p).is_none()
                    })
                })
                .min_by_key(|p| p.len());
            path.and_then(|p| p.get(1).copied())
                .map_or(GameCommand::Wait, |next| {
                    GameCommand::Move(
                        Direction::from_delta(next.x - start.x, next.y - start.y).unwrap(),
                    )
                })
        };
        let result = app.execute_command(command);
        assert_eq!(app.game.status(), RunStatus::Active, "{result:?}");
        app.capture_events_at(Some(0.));
    }
    assert!(app.game.actors().get(target).is_none_or(|a| !a.is_alive()));
    assert!(app.game.player_progression().experience() > xp);
    let saved = app.suspension().unwrap();
    let (r, t, l, e) = ascii_game_content().unwrap();
    let restored = AsciiApp::restore_suspension(&saved, r, t, l, e).unwrap();
    assert_eq!(
        suspension::fingerprint(&app.game),
        suspension::fingerprint(&restored.game)
    );
    assert!(
        restored
            .game
            .actors()
            .get(target)
            .is_none_or(|a| !a.is_alive())
    );
}
