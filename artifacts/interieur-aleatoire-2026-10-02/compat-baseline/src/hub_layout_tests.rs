use super::*;
use crate::hub_layout::{HomePlacement, contains};
use std::collections::BTreeSet;

#[test]
fn random_home_new_game_samples_once_and_rebuilds_with_the_chosen_seed() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
    app.begin_character_creation(false).unwrap();
    let first = app.character_creation.as_ref().unwrap().seed;
    app.begin_character_creation(false).unwrap();
    let creation = app.character_creation.as_ref().unwrap().clone();
    assert_ne!(creation.seed, first);
    let seed = creation.seed;
    app.rebuild_run_with_character_class(&creation).unwrap();
    assert_eq!(app.seed, seed);
    assert_eq!(app.home_layout(), HomePlacement::for_seed(seed));
    assert_eq!(app.suspension().unwrap().seed, seed);
}

#[test]
fn random_home_preserves_authored_terrain_controls_and_all_exits_on_every_edge() {
    let mut sides = BTreeSet::new();
    let mut positions = BTreeSet::new();
    for seed in 1..=48 {
        let original = TestSector::build(seed)
            .unwrap()
            .with_surface_contacts()
            .unwrap();
        let home = HomePlacement::for_seed(seed);
        assert_eq!(home, HomePlacement::for_seed(seed));
        sides.insert(format!("{:?}", home.side));
        positions.insert(home.origin);
        let relocated = crate::surface_layout::densify_hub_at_random_edge(original, seed)
            .unwrap_or_else(|e| panic!("seed {seed}, {home:?}: {e}"));
        let before = TestSector::build(seed)
            .unwrap()
            .with_surface_contacts()
            .unwrap();
        assert_eq!(
            relocated.level.player_start(),
            home.position(TestSector::RECYCLING_START)
        );
        assert_eq!(relocated.enemies.len(), before.enemies.len());
        assert_eq!(relocated.loot.len(), before.loot.len());
        let [x, y, w, h] = home.rectangle([1, 1, 62, 44]);
        assert!(
            x == 1
                || y == 1
                || x + w == TestSector::EXPANDED_WIDTH as i32 - 1
                || y + h == TestSector::EXPANDED_HEIGHT as i32 - 1
        );
        for py in 1..=46 {
            for px in 1..=102 {
                let p = GridPos::new(px, py);
                let to = home.position(p);
                if !home.authored_contains(to) {
                    continue;
                }
                let mut expected = *before.level.map().tile(p).unwrap();
                if let Terrain::ControlPanel { door, activated } = expected.terrain {
                    expected.terrain = Terrain::ControlPanel {
                        door: home.position(door),
                        activated,
                    };
                }
                assert_eq!(
                    relocated.level.map().tile(to),
                    Some(&expected),
                    "seed {seed}, {p:?}"
                );
            }
        }
        for (_, p) in home.regional_passages() {
            assert!(!contains(home.bounds(), p));
            assert_eq!(
                relocated.decor.cells.get(&p),
                Some(&crate::test_sector::Decor::Passage)
            );
        }
    }
    assert_eq!(sides.len(), 4);
    assert!(positions.len() > 16);
}

#[test]
fn random_home_services_quests_intro_and_resume_follow_the_physical_placement() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut covered = BTreeSet::new();
    for seed in 1..=24 {
        let home = HomePlacement::for_seed(seed);
        if !covered.insert(format!("{:?}", home.side)) {
            continue;
        }
        let mut app = AsciiApp::from_seed(
            seed,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
        )
        .unwrap_or_else(|e| panic!("seed {seed}, {home:?}: {e}"));
        app.capture_events_at(Some(0.0));
        assert!(app.recycling_intro_is_current_map());
        let definition = app.starter_hub_definition().unwrap();
        let merchant = definition.hub_merchant.as_ref().unwrap();
        let merchant_id = app.game.actors().entity_at(merchant.position).unwrap();
        assert!(app.game.active_merchant(merchant_id));
        let clinic = definition.hub_clinic.as_ref().unwrap();
        assert!(app.game.actors().entity_at(clinic.work_position).is_some());
        for character in &definition.narrative.as_ref().unwrap().characters {
            if let Some([x, y]) = character.hub_position {
                assert!(
                    app.game.actors().entity_at(GridPos::new(x, y)).is_some(),
                    "{}",
                    character.tag
                );
            }
        }
        for (_, p) in app.home_regional_passages() {
            assert!(
                app.game.passage(p).is_some()
                    || app.game.unmaterialized_passage_destination(p).is_some()
            );
        }
        app.walk_fixture_to(app.home_position(GridPos::new(27, 23)))
            .unwrap();
        assert!(app.intro_city_reached);
        let before = app.game.map().clone();
        let saved = app.suspension().unwrap();
        let restored = AsciiApp::restore_suspension(
            &saved,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
        )
        .unwrap();
        assert_eq!(restored.home_layout(), home);
        assert_eq!(restored.game.map(), &before);
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        assert!(restored.intro_city_reached);
    }
    assert_eq!(covered.len(), 4);
}
