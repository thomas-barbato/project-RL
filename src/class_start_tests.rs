use super::*;

fn normal_start(index: usize) -> AsciiApp {
    let (r, t, l, e) = ascii_game_content().unwrap();
    let mut app = AsciiApp::from_seed(1872, r, t, l, e).unwrap();
    app.crash_recovery_enabled = false;
    app.suspension_path = std::env::temp_dir().join(format!(
        "rl-class-start-{}-{index}.json",
        std::process::id()
    ));
    app.begin_character_creation_with_seed(false, 1872).unwrap();
    let attributes = app
        .character_classes
        .iter()
        .nth(index)
        .unwrap()
        .1
        .recommended_attributes();
    let creation = app.character_creation.as_mut().unwrap();
    creation.selected_class = index;
    creation.attributes = attributes;
    let creation = app.character_creation.take().unwrap();
    app.rebuild_run_with_character_class(&creation).unwrap();
    app
}

#[test]
fn class_start_uses_class_skin_without_an_item_and_first_active_can_be_removed_and_restored() {
    for (index, skill) in ["B01", "D59", "T01", "FE01", "M01"].into_iter().enumerate() {
        let mut app = normal_start(index);
        assert_eq!(app.generation_version, 165);
        assert!(
            app.game
                .player_equipment()
                .equipped(&"core:body_armor".parse().unwrap())
                .is_none()
        );
        assert!(app.game.player_visible_body_item().is_none());
        let original = app.front_player_visual();
        let crate::game_art::Visual::Player {
            body, equipment, ..
        } = original
        else {
            panic!("missing class visual")
        };
        assert_eq!(body, 3 + index as u8);
        assert_eq!(equipment[0], None);
        assert_eq!(app.front_actor_visual(app.game.player_id()), Some(original));
        assert_eq!(
            app.ux.skill_bar,
            [Some(skill.into()), None, None, None, None, None]
        );
        let weapon = app.game.equipped_player_weapon_item(0).unwrap();
        assert!(
            app.game
                .player_inventory()
                .get(weapon)
                .unwrap()
                .magic_modifiers()
                .is_none()
        );

        // Use the ordinary drag-out path, not a direct mutation of the bar.
        app.skills_open = true;
        app.begin_agent_skill_drag(skill, Some(0), (300., 300.));
        app.route_agent_skill_bar(
            &InputFrame {
                pointer: Some((15., 15.)),
                mouse_left_down: true,
                viewport: Some((1280., 800.)),
                ..Default::default()
            },
            true,
        );
        app.route_agent_skill_bar(
            &InputFrame {
                pointer: Some((15., 15.)),
                viewport: Some((1280., 800.)),
                ..Default::default()
            },
            true,
        );
        assert!(app.ux.skill_bar.iter().all(Option::is_none));
        app.sync_agent_skill_bar();
        assert!(app.ux.skill_bar.iter().all(Option::is_none));
        assert_eq!(app.game.player_agent_skill_rank(skill), 1);
        app.game.drain_events();
        let saved = app.suspension().unwrap();
        assert_eq!(saved.agent_skill_bar, vec![None; 6]);
        let (r, t, l, e) = ascii_game_content().unwrap();
        let mut restored = AsciiApp::restore_suspension(&saved, r, t, l, e).unwrap();
        assert!(restored.ux.skill_bar.iter().all(Option::is_none));
        assert_eq!(restored.front_player_visual(), original);
        assert_eq!(restored.game.player_agent_skill_rank(skill), 1);
        assert_eq!(
            suspension::fingerprint(&restored.game),
            suspension::fingerprint(&app.game)
        );
        restored.sync_agent_skill_bar();
        assert!(restored.ux.skill_bar.iter().all(Option::is_none));
    }
}

#[test]
fn class_start_equipped_armor_overrides_the_class_skin_and_removal_restores_it() {
    for index in 0..5 {
        let mut app = normal_start(index);
        let original = app.front_player_visual();
        app.game
            .grant_player_item_for_diagnostic("core:armure_cerbere".parse().unwrap(), 1)
            .unwrap();
        let item = app
            .game
            .player_inventory()
            .iter()
            .find(|e| e.item().as_str() == "core:armure_cerbere")
            .unwrap()
            .instance();
        assert_eq!(
            app.execute_command(GameCommand::EquipItem {
                slot: "core:body_armor".parse().unwrap(),
                item
            }),
            CommandOutcome::Applied
        );
        assert_ne!(app.front_player_visual(), original);
        assert_eq!(
            app.front_actor_visual(app.game.player_id()),
            Some(app.front_player_visual())
        );
        assert_eq!(
            app.execute_command(GameCommand::DropItem { item }),
            CommandOutcome::Applied
        );
        assert!(app.game.player_visible_body_item().is_none());
        assert_eq!(app.front_player_visual(), original);
    }
}

#[test]
fn class_start_weapon_and_active_work_with_real_initial_resources() {
    use project_rl::{
        skills::agent,
        social::PlayerRelation,
        world::{Map, Terrain},
    };
    for (index, skill) in ["B01", "D59", "T01", "FE01", "M01"].into_iter().enumerate() {
        let app = normal_start(index);
        let mut game = GameState::new_with_rules(
            Map::filled(12, 10, Terrain::Floor).unwrap(),
            GridPos::new(3, 4),
            1872,
            app.game.rules().clone(),
        )
        .unwrap();
        let target = if skill == "M01" {
            GridPos::new(3, 4)
        } else {
            GridPos::new(4, 4)
        };
        if matches!(skill, "B01" | "T01" | "FE01") {
            game.spawn_actor(
                Actor::new(target, 1000)
                    .unwrap()
                    .with_player_relation(PlayerRelation::Hostile),
            )
            .unwrap();
        }
        assert!(game.equipped_player_weapon(0).is_some());
        assert_eq!(
            game.process_player_command(GameCommand::UseTechniqueAt {
                technique: agent::technique(skill),
                target,
                weapon_slot: 0
            }),
            CommandOutcome::Applied,
            "{skill}"
        );
    }
}

#[test]
fn class_start_historical_campaigns_keep_their_outfit_instances_and_saved_bar() {
    let classes = ascii_character_class_catalog().unwrap();
    for (id, class) in classes.iter() {
        let (mut r, t, l, e) = ascii_game_content().unwrap();
        class
            .apply_to_rules(&mut r, class.recommended_attributes())
            .unwrap();
        weapon_supplies_app::ensure_class_weapon_supplies(
            &mut r,
            COSMETIC_STARTING_OUTFITS_GENERATION_VERSION,
        );
        let mut app = AsciiApp::from_seed_version(
            1872,
            r,
            t,
            l,
            e,
            COSMETIC_STARTING_OUTFITS_GENERATION_VERSION,
        )
        .unwrap();
        app.character_class = Some(id.clone());
        let outfit = app
            .game
            .player_equipment()
            .equipped(&"core:body_armor".parse().unwrap())
            .unwrap();
        assert_eq!(outfit.get(), 2, "historical item order changed for {id}");
        assert_eq!(
            app.game
                .player_inventory()
                .get(outfit)
                .unwrap()
                .item()
                .as_str(),
            project_rl::agent_class::historical_outfit(class.agent_start().unwrap()).unwrap()
        );
        assert!(app.ux.skill_bar.iter().all(Option::is_none));
        app.game.drain_events();
        let saved = app.suspension().unwrap();
        let (r, t, l, e) = ascii_game_content().unwrap();
        let restored = AsciiApp::restore_suspension(&saved, r, t, l, e).unwrap();
        assert_eq!(
            restored
                .game
                .player_equipment()
                .equipped(&"core:body_armor".parse().unwrap()),
            Some(outfit)
        );
        assert_eq!(restored.ux.skill_bar, app.ux.skill_bar);
        assert_eq!(
            suspension::fingerprint(&restored.game),
            suspension::fingerprint(&app.game)
        );
    }
}
