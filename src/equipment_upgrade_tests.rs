use super::*;

fn app() -> AsciiApp {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    AsciiApp::from_seed_version(
        INITIAL_SEED,
        rules,
        texts,
        loot,
        expeditions,
        CURRENT_GENERATION_VERSION,
    )
    .unwrap()
}
fn fixture() -> (AsciiApp, EntityId, ItemInstanceId) {
    let mut app = app();
    app.prepare_equipment_upgrade_diagnostic().unwrap();
    let provider = app.npc_interaction.unwrap();
    let item = app.upgrade_items()[app.npc_trade_selection];
    (app, provider, item)
}
fn enter(app: &AsciiApp) -> InputFrame {
    InputFrame {
        pressed: [app.controls.binding(Action::Learn).clone()]
            .into_iter()
            .collect(),
        ..InputFrame::default()
    }
}

#[test]
fn upgrade_confirmation_is_free_and_keyboard_mouse_share_transaction() {
    let (mut app, provider, item) = fixture();
    let state = suspension::fingerprint(&app.game);
    let before = app.inventory_entry_name(app.game.player_inventory().get(item).unwrap());
    let quote = app.game.equipment_upgrade_quote(item).unwrap();
    let credits = app.game.player_credits();
    app.update_equipment_upgrade(&enter(&app), provider);
    assert_eq!(app.upgrade_confirmation, Some((provider, item)));
    assert_eq!(suspension::fingerprint(&app.game), state);
    let layout = UpgradeLayout::new(960.0, 540.0);
    let click = |r: Rect| InputFrame {
        viewport: Some((960.0, 540.0)),
        pointer: Some((r.x + r.w / 2.0, r.y + r.h / 2.0)),
        pressed: [controls::Binding::MouseLeft].into_iter().collect(),
        ..InputFrame::default()
    };
    app.update_equipment_upgrade(&click(layout.close), provider);
    assert!(app.upgrade_confirmation.is_none());
    assert_eq!(suspension::fingerprint(&app.game), state);
    app.update_equipment_upgrade(&click(layout.action), provider);
    assert_eq!(suspension::fingerprint(&app.game), state);
    app.update_equipment_upgrade(&enter(&app), provider);
    let entry = app.game.player_inventory().get(item).unwrap();
    assert!(entry.magic_modifiers().unwrap().improved());
    assert!((1..=6).contains(&entry.magic_modifiers().unwrap().bonus_count()));
    assert_eq!(app.game.player_credits(), credits - quote.credits);
    assert_eq!(
        app.npc_interaction_message,
        format!(
            "Votre {before} a été amélioré en {}.",
            app.inventory_entry_name(entry)
        )
    );
    assert_eq!(
        app.game.equipment_upgrade_quote(item),
        Err(CommandRejection::EquipmentAlreadyImproved)
    );
}

#[test]
fn upgrade_refusals_leave_inventory_credits_rng_and_turn_untouched() {
    let (mut app, provider, item) = fixture();
    let before = suspension::fingerprint(&app.game);
    assert!(matches!(
        app.execute_command(GameCommand::ImproveEquipment {
            artisan: app.game.player_id(),
            item
        }),
        CommandOutcome::Rejected(_)
    ));
    assert_eq!(suspension::fingerprint(&app.game), before);
    // A separate ordinary-shop fixture starts with only 120 credits.
    let mut poor = self::app();
    poor.rules.player_inventory_capacity = 32;
    poor.prepare_merchant_diagnostic().unwrap();
    let artisan = poor
        .game
        .spawn_actor(
            Actor::new(GridPos::new(4, 2), 20)
                .unwrap()
                .with_tags(["core:equipment_artisan".parse().unwrap()]),
        )
        .unwrap();
    poor.game
        .grant_player_item_for_diagnostic("core:unstable_fragment".parse().unwrap(), 3)
        .unwrap();
    poor.game
        .grant_player_item_for_diagnostic("core:visiere_fantome".parse().unwrap(), 1)
        .unwrap();
    let expensive = poor
        .game
        .player_inventory()
        .iter()
        .find(|e| e.item().as_str() == "core:visiere_fantome")
        .unwrap()
        .instance();
    let before = suspension::fingerprint(&poor.game);
    assert_eq!(
        poor.execute_command(GameCommand::ImproveEquipment {
            artisan,
            item: expensive
        }),
        CommandOutcome::Rejected(CommandRejection::InsufficientCredits)
    );
    assert_eq!(suspension::fingerprint(&poor.game), before);
    assert_eq!(
        app.execute_command(GameCommand::ImproveEquipment {
            artisan: provider,
            item
        }),
        CommandOutcome::Applied
    );
    let before = suspension::fingerprint(&app.game);
    assert_eq!(
        app.execute_command(GameCommand::ImproveEquipment {
            artisan: provider,
            item
        }),
        CommandOutcome::Rejected(CommandRejection::EquipmentAlreadyImproved)
    );
    assert_eq!(suspension::fingerprint(&app.game), before);
}

#[test]
fn upgrade_marker_survives_snapshot_sale_and_buyback() {
    let (mut app, provider, item) = fixture();
    assert_eq!(
        app.execute_command(GameCommand::ImproveEquipment {
            artisan: provider,
            item
        }),
        CommandOutcome::Applied
    );
    let bonus = app
        .game
        .player_inventory()
        .get(item)
        .unwrap()
        .magic_modifiers()
        .unwrap();
    let json = serde_json::to_string(&bonus).unwrap();
    assert_eq!(
        serde_json::from_str::<project_rl::entity::MagicItemModifiers>(&json).unwrap(),
        bonus
    );
    app.capture_events();
    let bytes = app.game.recovery_snapshot_bytes().unwrap();
    app.game = WorldState::from_recovery_snapshot_bytes(&bytes, app.game.rules().clone()).unwrap();
    assert_eq!(
        app.game.equipment_upgrade_quote(item),
        Err(CommandRejection::EquipmentAlreadyImproved)
    );
    let merchant = app
        .game
        .actors()
        .iter()
        .find_map(|(id, _)| app.game.active_merchant(id).then_some(id))
        .unwrap();
    assert_eq!(
        app.execute_command(GameCommand::SellItem { merchant, item }),
        CommandOutcome::Applied
    );
    let view = app.game.npc_interaction(merchant).unwrap();
    let NpcService::Trade { resale, .. } = &view.services[0] else {
        panic!("trade expected")
    };
    let listing = resale
        .iter()
        .find(|r| r.magic_modifiers.as_ref() == Some(&bonus))
        .unwrap()
        .listing;
    assert_eq!(
        app.execute_command(GameCommand::BuyResaleItem { merchant, listing }),
        CommandOutcome::Applied
    );
    let bought = app
        .game
        .player_inventory()
        .iter()
        .find(|e| e.magic_modifiers().as_ref() == Some(&bonus))
        .unwrap()
        .instance();
    assert_eq!(
        app.game.equipment_upgrade_quote(bought),
        Err(CommandRejection::EquipmentAlreadyImproved)
    );
}

#[test]
fn upgrade_uses_base_tier_and_replaces_instead_of_stacking() {
    let original = app();
    let rules = original.rules.clone();
    let upgrade = rules.equipment_upgrade.as_ref().unwrap();
    for tier in 1..=6 {
        let id = upgrade
            .catalog
            .iter()
            .find(|(_, b)| b.tier == tier)
            .unwrap()
            .0
            .clone();
        for seed in 0..12 {
            let mut r = rules.clone();
            r.player_starting_items = vec![project_rl::game::StartingItemStack::new(
                "core:unstable_fragment".parse().unwrap(),
                3,
            )];
            let mut rng = project_rl::game::GameRng::from_seed(seed + 1000);
            let old = upgrade.catalog.enchant_base(&id, 1, &mut rng).unwrap();
            let mut game = GameState::new_with_rules(
                project_rl::world::Map::from_ascii("#####\n#...#\n#####").unwrap(),
                GridPos::new(1, 1),
                seed,
                r,
            )
            .unwrap()
            .with_starting_magic_equipment([(id.clone(), old)])
            .unwrap();
            let item = game
                .player_inventory()
                .iter()
                .find(|e| e.item() == &id)
                .unwrap()
                .instance();
            let quote = game.equipment_upgrade_quote(item).unwrap();
            assert_eq!(
                (quote.fragments, quote.credits),
                upgrade.costs[usize::from(tier - 1)]
            );
            assert_eq!(
                game.process_player_command(GameCommand::ImproveEquipment {
                    artisan: game.player_id(),
                    item
                }),
                CommandOutcome::Applied
            );
            let entry = game.player_inventory().get(item).unwrap();
            assert_eq!(entry.item(), &id);
            let bonus = entry.magic_modifiers().unwrap();
            assert!(bonus.improved());
            assert!((1..=6).contains(&bonus.bonus_count()));
            let stats: Vec<_> = bonus
                .named_affixes()
                .into_iter()
                .flat_map(|a| a.iter().map(|r| r.id()).collect::<Vec<_>>())
                .collect();
            assert_eq!(
                stats.len(),
                stats
                    .iter()
                    .collect::<std::collections::BTreeSet<_>>()
                    .len()
            );
        }
    }
}

#[test]
fn upgrade_surface_artisan_is_protected_and_does_not_duplicate() {
    let mut app = app();
    app.walk_fixture_out_of_recycling().unwrap();
    app.ensure_local_artisan().unwrap();
    let artisans: Vec<_> = app
        .game
        .actors()
        .iter()
        .filter(|(id, _)| app.game.active_artisan(*id))
        .map(|(_, a)| a.position())
        .collect();
    assert_eq!(artisans.len(), 1);
    assert!(app.game.map().is_protected(artisans[0]));
    app.ensure_local_artisan().unwrap();
    assert_eq!(
        app.game
            .actors()
            .iter()
            .filter(|(id, _)| app.game.active_artisan(*id))
            .count(),
        1
    );
    let at = artisans[0]
        .cardinal_neighbors()
        .into_iter()
        .find(|p| app.game.map().is_walkable(*p) && app.game.actors().entity_at(*p).is_none())
        .unwrap();
    app.walk_fixture_to(at).unwrap();
    let provider = app.game.actors().entity_at(artisans[0]).unwrap();
    assert_eq!(
        app.game.npc_interaction(provider).unwrap().role,
        NpcRole::Artisan
    );
    assert!(app.context_npc_at(artisans[0]));
    let saved = app.suspension().unwrap();
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let restored = AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
    assert_eq!(
        suspension::fingerprint(&restored.game),
        suspension::fingerprint(&app.game)
    );
}

#[test]
fn upgrade_every_underground_city_has_an_accessible_artisan_and_limited_fragments() {
    let mut app = app();
    app.intro_city_reached = true;
    let atlas = app
        .regional_worlds
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap()
        .clone();
    assert_eq!(atlas.cities().len(), 5);
    for city in atlas.cities() {
        let generated =
            project_rl::world::generation::generate_regional_city(city.map_size(), city.layout())
                .unwrap();
        let entrance = generated.passages()[0];
        let mut game =
            GameState::new_with_rules(generated.map().clone(), entrance, 13, app.rules.clone())
                .unwrap();
        for at in std::iter::once(city.merchant().position)
            .chain(std::iter::once(city.clinic().work_position))
            .chain(city.residents().iter().map(|r| r.residence_position))
        {
            game.spawn_actor(Actor::new(at, 20).unwrap().with_ai(AiProfile::idle()))
                .unwrap();
        }
        let descriptor = atlas.region(app.seed, city.coordinate()).unwrap();
        let info = crate::test_regional::zone_info(&atlas, &descriptor).unwrap();
        app.game = WorldState::single(game);
        app.game.enable(info.clone()).unwrap();
        app.regional_zones.insert(info.id, city.coordinate());
        app.install_active_regional_city_services().unwrap();
        app.install_active_regional_city_services().unwrap();
        let artisans: Vec<_> = app
            .game
            .actors()
            .iter()
            .filter(|(id, _)| app.game.active_artisan(*id))
            .map(|(id, a)| (id, a.position()))
            .collect();
        assert_eq!(artisans.len(), 1, "{}", city.name());
        let (artisan, at) = artisans[0];
        assert!(app.game.map().is_protected(at));
        let neighbor = at
            .cardinal_neighbors()
            .into_iter()
            .find(|p| {
                app.game.map().is_walkable(*p)
                    && app.game.actors().entity_at(*p).is_none()
                    && project_rl::world::find_path(
                        app.game.map(),
                        app.game.player_position().unwrap(),
                        *p,
                        15000,
                        |_| true,
                    )
                    .is_some()
            })
            .unwrap();
        app.walk_fixture_to(neighbor)
            .unwrap_or_else(|e| panic!("{}: {e}", city.name()));
        assert!(app.game.npc_interaction(artisan).is_some());
        let at = city.merchant().position;
        let neighbor = at
            .cardinal_neighbors()
            .into_iter()
            .find(|p| {
                app.game.map().is_walkable(*p)
                    && app.game.actors().entity_at(*p).is_none()
                    && project_rl::world::find_path(
                        app.game.map(),
                        app.game.player_position().unwrap(),
                        *p,
                        15000,
                        |_| true,
                    )
                    .is_some()
            })
            .unwrap();
        app.walk_fixture_to(neighbor).unwrap();
        let merchant = app.game.actors().entity_at(at).unwrap();
        let view = app.game.npc_interaction(merchant).unwrap();
        let NpcService::Trade { offers, .. } = &view.services[0] else {
            panic!()
        };
        let offer = offers
            .iter()
            .find(|o| o.item.as_str() == "core:unstable_fragment")
            .unwrap();
        assert_eq!(offer.stock, 6);
        assert_eq!(offer.price, 15);
    }
}

#[test]
fn upgrade_without_fragments_is_rejected_before_any_reroll() {
    let mut app = app();
    app.prepare_merchant_diagnostic_at_depth(Some(0)).unwrap();
    let artisan = app
        .game
        .spawn_actor(
            Actor::new(GridPos::new(4, 2), 20)
                .unwrap()
                .with_tags(["core:equipment_artisan".parse().unwrap()]),
        )
        .unwrap();
    app.game
        .grant_player_item_for_diagnostic("core:modele_brask_p12".parse().unwrap(), 1)
        .unwrap();
    let item = app
        .game
        .player_inventory()
        .iter()
        .find(|e| e.item().as_str() == "core:modele_brask_p12")
        .unwrap()
        .instance();
    let state = suspension::fingerprint(&app.game);
    assert_eq!(
        app.execute_command(GameCommand::ImproveEquipment { artisan, item }),
        CommandOutcome::Rejected(CommandRejection::MissingUpgradeFragments)
    );
    assert_eq!(suspension::fingerprint(&app.game), state);
}

#[test]
fn upgrade_campaign_purchase_and_result_replay_without_binary_cache() {
    let mut app = app();
    app.walk_fixture_out_of_recycling().unwrap();
    while app.game.player_inventory().capacity() - app.game.player_inventory().len() < 2 {
        let item = app
            .game
            .player_inventory()
            .iter()
            .find(|e| {
                !app.game
                    .player_equipment()
                    .iter()
                    .any(|(_, id)| id == e.instance())
            })
            .unwrap()
            .instance();
        assert_eq!(
            app.execute_command(GameCommand::DropItem { item }),
            CommandOutcome::Applied
        );
        app.capture_events();
    }
    let merchant = app
        .game
        .actors()
        .iter()
        .find_map(|(id, _)| app.game.active_merchant(id).then_some(id))
        .unwrap();
    let at = app.game.actors().get(merchant).unwrap().position();
    let neighbor = at
        .cardinal_neighbors()
        .into_iter()
        .find(|p| app.game.map().is_walkable(*p) && app.game.actors().entity_at(*p).is_none())
        .unwrap();
    app.walk_fixture_to(neighbor).unwrap();
    for id in ["core:calotte_cognefer", "core:unstable_fragment"] {
        assert_eq!(
            app.execute_command(GameCommand::BuyItem {
                merchant,
                item: id.parse().unwrap()
            }),
            CommandOutcome::Applied
        );
        app.capture_events();
    }
    let item = app
        .game
        .player_inventory()
        .iter()
        .find(|e| e.item().as_str() == "core:calotte_cognefer")
        .unwrap()
        .instance();
    let artisan = app
        .game
        .actors()
        .iter()
        .find_map(|(id, _)| app.game.active_artisan(id).then_some(id))
        .unwrap();
    let at = app.game.actors().get(artisan).unwrap().position();
    let neighbor = at
        .cardinal_neighbors()
        .into_iter()
        .find(|p| app.game.map().is_walkable(*p) && app.game.actors().entity_at(*p).is_none())
        .unwrap();
    app.walk_fixture_to(neighbor).unwrap();
    assert_eq!(
        app.execute_command(GameCommand::ImproveEquipment { artisan, item }),
        CommandOutcome::Applied
    );
    app.capture_events();
    let mut saved = app.suspension().unwrap();
    saved.recovery = None;
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let restored = AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
    assert_eq!(
        suspension::fingerprint(&restored.game),
        suspension::fingerprint(&app.game)
    );
    assert_eq!(
        restored.game.equipment_upgrade_quote(item),
        Err(CommandRejection::EquipmentAlreadyImproved)
    );
}
