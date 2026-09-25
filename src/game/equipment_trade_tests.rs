use super::*;
use crate::content::{ContentLoader, MerchantGambleDefinition, MerchantOfferDefinition};

fn id(name: &str) -> ContentId {
    format!("core:{name}").parse().unwrap()
}

fn fixture() -> (WorldState, crate::entity::EntityId, EquipmentLootCatalog) {
    let loaded = ContentLoader::load(
        &[std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content")],
        &semver::Version::new(0, 1, 0),
    )
    .unwrap();
    let mut rules = GameRules::default();
    rules.items = loaded.items().clone();
    rules.weapons = loaded.weapons().clone();
    rules.statuses = loaded.statuses().clone();
    let map = Map::from_ascii("########\n#......#\n#......#\n#......#\n########").unwrap();
    let mut game = GameState::new_with_rules(map, GridPos::new(2, 2), 71, rules).unwrap();
    let provider = game
        .spawn_actor(Actor::new(GridPos::new(3, 2), 10).unwrap())
        .unwrap();
    let mut world = WorldState::single(game);
    world
        .enable(ZoneInfo {
            id: id("shop_probe"),
            name: "Marché".into(),
            kind: id("city"),
            depth: 0,
        })
        .unwrap();
    let bases = [
        id("couteau_de_camp"),
        id("fusil_de_patrouille"),
        id("veste_matelassee"),
    ];
    let definition = MerchantDefinition::new(
        GridPos::new(3, 2),
        10,
        20000,
        bases
            .iter()
            .map(|item| MerchantOfferDefinition {
                item: item.clone(),
                initial_stock: 2,
                buy_price: 60,
                sell_price: 20,
                minimum_depth: 0,
                maximum_depth: None,
            })
            .collect(),
        bases
            .iter()
            .map(|item| MerchantGambleDefinition {
                item: item.clone(),
                initial_stock: 4,
                price: 120,
            })
            .collect(),
        GambleScalingDefinition::new(3, 1, 12).unwrap(),
    )
    .unwrap();
    let catalog = loaded.loot().equipment().clone();
    let quotes: Vec<_> = catalog.iter().map(|(id, _)| (id.clone(), 20, 60)).collect();
    world
        .register_equipment_merchant(
            id("shop_probe"),
            provider,
            5000,
            definition,
            71,
            &catalog,
            &quotes,
        )
        .unwrap();
    world.drain_events();
    (world, provider, catalog)
}

fn resume(world: &WorldState) -> WorldState {
    WorldState::from_recovery_snapshot_bytes(
        &world.recovery_snapshot_bytes().unwrap(),
        world.rules().clone(),
    )
    .unwrap()
}

#[test]
fn weapon_and_armor_gambles_are_hidden_atomic_persistent_and_use_the_advertised_base() {
    let (mut world, provider, _) = fixture();
    let before = format!("{world:?}");
    for _ in 0..5 {
        let view = world.npc_interaction(provider).unwrap();
        let [NpcService::Trade { gambles, .. }] = view.services.as_slice() else {
            panic!("shop absent")
        };
        assert_eq!(gambles.len(), 3);
        let visible = format!("{gambles:?}");
        assert!(!visible.contains("affix") && !visible.contains("modifiers"));
    }
    assert_eq!(format!("{world:?}"), before);
    let mut resumed = resume(&world);
    for item in [
        id("couteau_de_camp"),
        id("fusil_de_patrouille"),
        id("veste_matelassee"),
    ] {
        let command = GameCommand::GambleItem {
            merchant: provider,
            item: item.clone(),
        };
        assert_eq!(
            world.process_player_command(command.clone()),
            CommandOutcome::Applied
        );
        assert_eq!(
            resumed.process_player_command(command),
            CommandOutcome::Applied
        );
        world.drain_events();
        resumed.drain_events();
        assert_eq!(format!("{world:?}"), format!("{resumed:?}"));
        let purchased = world
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &item)
            .unwrap();
        let bonus = purchased.magic_modifiers().unwrap();
        assert_eq!(bonus.armor_bonus(), 0);
        assert_eq!(bonus.mass_reduction_percent(), 0);
        if item == id("veste_matelassee") {
            assert!(bonus.effect_affix().is_none());
        }
        if let Some(rolls) = bonus.named_affixes() {
            assert!(rolls.iter().all(|roll| roll.tier() == 1));
        }
    }
    world.player_credits = 0;
    let before = format!("{world:?}");
    assert_eq!(
        world.process_player_command(GameCommand::GambleItem {
            merchant: provider,
            item: id("couteau_de_camp")
        }),
        CommandOutcome::Rejected(CommandRejection::InsufficientCredits)
    );
    assert_eq!(format!("{world:?}"), before);
}

#[test]
fn ordinary_purchases_are_white_and_unstocked_equipment_keeps_all_its_properties_on_resale() {
    let (mut world, provider, catalog) = fixture();
    let item = id("fusil_de_patrouille");
    assert_eq!(
        world.process_player_command(GameCommand::BuyItem {
            merchant: provider,
            item: item.clone()
        }),
        CommandOutcome::Applied
    );
    assert!(
        world
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &item)
            .unwrap()
            .magic_modifiers()
            .is_none()
    );
    let item = id("fusil_de_l_horizon_fendu");
    let mut rng = GameRng::from_seed(86);
    let bonus = (0..500)
        .map(|_| catalog.enchant_base(&item, 1, &mut rng).unwrap())
        .find(|bonus| bonus.effect_affix().is_some() && bonus.named_affixes().is_some())
        .unwrap();
    let sold = world
        .active
        .player_inventory_mut()
        .add_magic(item.clone(), None, bonus.clone())
        .unwrap();
    let interaction = world.npc_interaction(provider).unwrap();
    let [NpcService::Trade { sellable, .. }] = interaction.services.as_slice() else {
        panic!("shop absent")
    };
    assert!(
        sellable
            .iter()
            .any(|entry| entry.instance == sold && entry.price == 20)
    );
    assert_eq!(
        world.process_player_command(GameCommand::SellItem {
            merchant: provider,
            item: sold
        }),
        CommandOutcome::Applied
    );
    world.drain_events();
    world = resume(&world);
    let interaction = world.npc_interaction(provider).unwrap();
    let [NpcService::Trade { resale, .. }] = interaction.services.as_slice() else {
        panic!("shop absent")
    };
    assert_eq!(resale[0].item, item);
    assert_eq!(resale[0].magic_modifiers, Some(bonus.clone()));
    assert_eq!(
        world.process_player_command(GameCommand::BuyResaleItem {
            merchant: provider,
            listing: resale[0].listing
        }),
        CommandOutcome::Applied
    );
    let recovered = world
        .player_inventory()
        .iter()
        .find(|entry| entry.item() == &item)
        .unwrap();
    assert_eq!(recovered.magic_modifiers(), Some(bonus));
}

#[test]
fn full_inventory_out_of_stock_and_invalid_snapshot_do_not_reroll_or_charge() {
    let (mut world, provider, catalog) = fixture();
    while world.player_inventory().remaining_slots() > 0 {
        world
            .active
            .player_inventory_mut()
            .add(id("couteau_de_camp"), 1, 1)
            .unwrap();
    }
    let before = format!("{world:?}");
    assert_eq!(
        world.process_player_command(GameCommand::GambleItem {
            merchant: provider,
            item: id("couteau_de_camp")
        }),
        CommandOutcome::Rejected(CommandRejection::InventoryCannotFitItem)
    );
    assert_eq!(format!("{world:?}"), before);
    world.merchants.get_mut(&id("shop_probe")).unwrap().gambles[0].stock = 0;
    let before = format!("{world:?}");
    assert_eq!(
        world.process_player_command(GameCommand::GambleItem {
            merchant: provider,
            item: id("couteau_de_camp")
        }),
        CommandOutcome::Rejected(CommandRejection::MerchantOutOfStock)
    );
    assert_eq!(format!("{world:?}"), before);
    world
        .merchants
        .get_mut(&id("shop_probe"))
        .unwrap()
        .equipment
        .as_mut()
        .unwrap()
        .properties
        .get_mut(&id("couteau_de_camp"))
        .unwrap()
        .effects[0]
        .0 = id("missing_effect");
    assert!(
        WorldState::from_recovery_snapshot_bytes(
            &world.recovery_snapshot_bytes().unwrap(),
            world.rules().clone()
        )
        .is_err()
    );
    let mut rng = GameRng::from_seed(3);
    assert!(
        catalog
            .enchant_base(&id("missing_base"), 1, &mut rng)
            .is_err()
    );
    assert!(
        catalog
            .enchant_base(&id("couteau_de_camp"), 0, &mut rng)
            .is_err()
    );
    assert_eq!(rng.state(), 3);
}

#[test]
fn progression_improves_values_but_never_exceeds_the_base_tier_or_changes_it() {
    let (_, _, catalog) = fixture();
    let item = id("couteau_de_camp");
    let mean = |draws| {
        let mut rng = GameRng::from_seed(99);
        let mut total = 0u64;
        let mut count = 0u64;
        for _ in 0..4000 {
            let bonus = catalog.enchant_base(&item, draws, &mut rng).unwrap();
            if let Some(rolls) = bonus.named_affixes() {
                for roll in rolls.iter() {
                    assert_eq!(roll.tier(), 1);
                    total += u64::from(roll.value());
                    count += 1;
                }
            }
        }
        total as f64 / count as f64
    };
    assert!(mean(4) > mean(1));
    assert!(
        GambleRollProfile::for_progression(GambleScalingDefinition::new(3, 1, 12).unwrap(), 30, 0)
            .quality_draws
            > 1
    );
}
