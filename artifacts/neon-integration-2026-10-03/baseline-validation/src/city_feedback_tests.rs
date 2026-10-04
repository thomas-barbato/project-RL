use super::*;
use project_rl::game::{GameRng, ZoneBlueprint};

#[test]
fn monster_reward_current_client_uses_new_pool_and_legacy_runs_keep_the_old_rules() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    assert!(
        rules_for_generation_version(rules.clone(), 140)
            .monster_equipment_loot
            .is_none()
    );
    assert!(
        rules_for_generation_version(rules.clone(), 141)
            .monster_equipment_loot
            .is_some()
    );
    let mut app = AsciiApp::from_seed(8, rules, texts, loot, expeditions).unwrap();
    assert_eq!(app.generation_version, 141);
    app.prepare_monster_reward_diagnostic().unwrap();
    let items: std::collections::BTreeSet<_> = app
        .game
        .player_inventory()
        .iter()
        .filter(|entry| {
            app.game.rules().weapons.get(entry.item()).is_some()
                || app
                    .game
                    .rules()
                    .items
                    .get(entry.item())
                    .is_some_and(|d| d.kind() == ItemKind::Armor)
        })
        .map(|entry| entry.item().clone())
        .collect();
    assert!(
        items.len() >= 8,
        "10 identical enemies must give varied models: {items:?}"
    );
    assert!(
        items
            .iter()
            .any(|item| app.game.rules().weapons.get(item).is_some())
    );
    assert!(items.iter().any(|item| {
        app.game
            .rules()
            .items
            .get(item)
            .is_some_and(|d| d.kind() == ItemKind::Armor)
    }));
    assert!(app.game.ground_items().iter().next().is_none());
}

/// Inspect a copied save without consuming or writing the user's suspension.
#[test]
#[ignore]
fn city_feedback_saved_run_diagnostic() {
    let source = std::env::var("RL_DIAGNOSTIC_SAVE").expect("copied save path");
    let saved = Suspension::read(std::path::Path::new(&source)).unwrap();
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let app = AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
    println!(
        "version={} seed={} zone={:?} player={:?} interior={:?} intro={} objective={:?} nav={:?}",
        saved.version,
        saved.seed,
        app.game.current_zone(),
        app.game.player_position(),
        app.home_layout().rectangle([1, 1, 62, 44]),
        app.intro_city_reached,
        app.primary_objective(),
        app.navigation_signal_summary()
    );
    println!(
        "gate={:?} terrain={:?}",
        app.home_position(TestSector::GATE),
        app.game.map().tile(app.home_position(TestSector::GATE))
    );
    for (id, actor) in app.game.actors().iter() {
        if app.game.active_resident(id)
            || app.game.active_clinic(id)
            || app.game.active_merchant(id)
            || app.game.active_quest_provider(id)
        {
            println!(
                "npc position={:?} resident={} healer={} merchant={} quests={} tags={:?}",
                actor.position(),
                app.game.active_resident(id),
                app.game.active_clinic(id),
                app.game.active_merchant(id),
                app.game.active_quest_provider(id),
                actor.tags()
            );
        }
    }
    for item in app.game.player_inventory().iter() {
        println!("inventory={}", item.item());
    }
}

#[test]
fn city_feedback_resident_and_clinic_move_on_all_four_edges() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut covered = std::collections::BTreeSet::new();
    for seed in 1..=24 {
        if !covered.insert(format!(
            "{:?}",
            crate::hub_layout::HomePlacement::for_seed(seed).side
        )) {
            continue;
        }
        let mut app = AsciiApp::from_seed(
            seed,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
        )
        .unwrap();
        app.walk_fixture_to(app.home_position(GridPos::new(27, 23)))
            .unwrap();
        let npcs: Vec<_> = app
            .game
            .actors()
            .iter()
            .filter_map(|(id, actor)| {
                (app.game.active_resident(id) || app.game.active_clinic(id))
                    .then_some((id, actor.position()))
            })
            .collect();
        assert_eq!(npcs.len(), 6);
        let mut moved = std::collections::BTreeSet::new();
        for _ in 0..80 {
            assert_eq!(
                app.game.process_player_command(GameCommand::Wait),
                CommandOutcome::Applied
            );
            for (id, origin) in &npcs {
                if app.game.actors().get(*id).unwrap().position() != *origin {
                    moved.insert(*id);
                }
            }
        }
        assert_eq!(moved.len(), npcs.len(), "seed {seed}");
    }
    assert_eq!(covered.len(), 4);
}

#[test]
fn city_feedback_new_starts_see_elias_inside_the_randomly_placed_town() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut covered = std::collections::BTreeSet::new();
    for seed in 1..=24 {
        if !covered.insert(format!(
            "{:?}",
            crate::hub_layout::HomePlacement::for_seed(seed).side
        )) {
            continue;
        }
        let mut app = AsciiApp::from_seed(
            seed,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
        )
        .unwrap();
        app.capture_events_at(Some(0.0));
        let start = app.game.player_position().unwrap();
        assert!(app.home_layout().town_contains(start));
        assert!(app.game.map().is_protected(start));
        assert!(app.intro_city_reached);
        let elias = app.home_first_contact().unwrap();
        assert!(app.game.player_visibility().is_visible(elias));
        assert!(grid_distance(start, elias) <= 3);
        assert!(app.navigation_signal_summary().unwrap().contains("ELIAS"));
        app.prepare_narrative_directions_diagnostic().unwrap();
        assert!(!app.game.quest_journal().is_empty());
        assert!(app.home_first_contact().is_none());
        let saved = app.suspension().unwrap();
        let restored = AsciiApp::restore_suspension(
            &saved,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
        )
        .unwrap();
        assert_eq!(saved.state, suspension::fingerprint(&restored.game));
    }
    assert_eq!(covered.len(), 4);
}

#[test]
fn city_feedback_old_starts_and_loot_remain_unchanged() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut app = AsciiApp::from_seed_version(
        8,
        rules.clone(),
        texts.clone(),
        loot.clone(),
        expeditions.clone(),
        CITY_DECOR_GENERATION_VERSION,
    )
    .unwrap();
    assert_eq!(
        app.game.player_position(),
        Some(app.home_position(TestSector::RECYCLING_START))
    );
    assert!(!app.intro_city_reached);
    assert!(app.navigation_signal_summary().unwrap().contains(
        &crate::terminal_view::local_coordinates(Some(app.home_position(TestSector::GATE)))
    ));
    assert_eq!(app.starter_hub_definition().unwrap().hub_residents.len(), 1);
    app.capture_events_at(Some(0.0));
    let saved = app.suspension().unwrap();
    let restored =
        AsciiApp::restore_suspension(&saved, rules.clone(), texts, loot.clone(), expeditions)
            .unwrap();
    assert_eq!(saved.state, suspension::fingerprint(&restored.game));
    assert_eq!(
        suspension::fingerprint(&app.loot),
        suspension::fingerprint(&loot_for_generation_version(
            &loot,
            &rules.items,
            CITY_DECOR_GENERATION_VERSION
        ))
    );
}

#[test]
fn city_feedback_real_carriers_and_caches_roll_different_models_and_independent_bonuses() {
    use project_rl::loot::{EquipmentQuality, EquipmentSource};
    use std::collections::{BTreeMap, BTreeSet};
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut app = AsciiApp::from_seed(8, rules.clone(), texts, loot.clone(), expeditions).unwrap();
    let mut by_family: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut bonus_counts = BTreeSet::new();
    let mut rng = GameRng::from_seed(140);
    let mut local_tiers = [0usize; 6];
    for _ in 0..12000 {
        let rolled = app
            .loot
            .equipment()
            .draw(
                EquipmentSource::HumanoidSite,
                0,
                EquipmentQuality::Random {
                    enchanted_percent: project_rl::loot::ENCHANTED_EQUIPMENT_PERCENT,
                },
                &mut rng,
            )
            .unwrap()
            .unwrap();
        let base = app
            .loot
            .equipment()
            .iter()
            .find(|(id, _)| **id == rolled.item)
            .unwrap()
            .1;
        by_family
            .entry(base.family.to_string())
            .or_default()
            .insert(rolled.item.to_string());
        bonus_counts.insert(rolled.modifiers.as_ref().map_or(0, |m| m.bonus_count()));
        if equipment_generation::current_model_ids().contains(&rolled.item) {
            local_tiers[usize::from(rolled.tier - 1)] += 1;
        }
    }
    for family in [
        "core:rifles",
        "core:knives",
        "core:reinforced_armor",
        "core:helmets",
        "core:gauntlets",
        "core:boots",
    ] {
        assert_eq!(by_family[family].len(), 3, "{family}");
    }
    assert_eq!(bonus_counts, (0..=6).collect());
    assert!(local_tiers[0] > local_tiers[1] && local_tiers[1] > local_tiers[2]);
    assert_eq!(&local_tiers[3..], &[0, 0, 0]);
    let mut carriers = BTreeSet::new();
    for seed in 1..=96 {
        let mut actors = vec![
            Actor::new(GridPos::new(4, 4), 10)
                .unwrap()
                .with_tags(["core:humanoid_rifle_carrier".parse().unwrap()]),
        ];
        app.equip_carried_weapons(&mut actors, 0, seed).unwrap();
        carriers.insert(actors[0].equipped_weapon().unwrap().item().to_string());
    }
    assert_eq!(carriers.len(), 3);
    for source in [
        EquipmentSource::Robot,
        EquipmentSource::OrganicCreature,
        EquipmentSource::MechanicalSite,
    ] {
        assert!(
            app.loot
                .equipment()
                .draw(source, 0, EquipmentQuality::White, &mut rng)
                .unwrap()
                .is_none()
        );
    }
    // The actual ground-cache adapter replaces both weapon and armor prototypes.
    let mut drops = BTreeSet::new();
    for seed in 1..=96 {
        let mut blueprint = ZoneBlueprint {
            info: project_rl::game::ZoneInfo {
                id: "core:variety_probe".parse().unwrap(),
                name: "Probe".into(),
                kind: "core:human_habitat".parse().unwrap(),
                depth: 0,
            },
            map: project_rl::world::Map::filled(12, 12, Terrain::Floor).unwrap(),
            entrance: GridPos::new(2, 2),
            seed,
            actors: vec![],
            loot: vec![project_rl::game::GroundLootBlueprint::new(
                GridPos::new(5, 5),
                "core:needle_launcher".parse().unwrap(),
                1,
            )],
            threat_sources: vec![],
        };
        app.populate_regional_equipment(
            &mut blueprint,
            &"core:human_habitat".parse().unwrap(),
            seed,
        )
        .unwrap();
        drops.insert(blueprint.loot[0].item().to_string());
    }
    assert!(drops.len() >= 16);
    app.capture_events_at(Some(0.0));
    let saved = app.suspension().unwrap();
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let restored = AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
    assert_eq!(saved.state, suspension::fingerprint(&restored.game));
}
