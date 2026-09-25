use super::*;
use crate::combat::DamageType;
use crate::content::{ContentId, ContentLoader};
use crate::entity::MagicItemModifiers;
use crate::stats::{BodyProfile, PhysicalRules};

fn id(value: &str) -> ContentId {
    format!("core:{value}").parse().unwrap()
}

fn rules() -> GameRules {
    static RULES: std::sync::OnceLock<GameRules> = std::sync::OnceLock::new();
    RULES
        .get_or_init(|| {
            let loaded = ContentLoader::load(
                &[std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content")],
                &semver::Version::new(0, 1, 0),
            )
            .unwrap();
            GameRules {
                items: loaded.items().clone(),
                weapons: loaded.weapons().clone(),
                statuses: loaded.statuses().clone(),
                player_maximum_integrity: 200,
                physical_rules: Some(PhysicalRules::default()),
                ..GameRules::default()
            }
        })
        .clone()
}

fn fixture() -> (GameState, EntityId, MagicItemModifiers) {
    let mut game = GameState::new_with_rules(
        Map::from_ascii("########\n#......#\n#......#\n########").unwrap(),
        GridPos::new(2, 1),
        108,
        rules(),
    )
    .unwrap();
    let bonus = MagicItemModifiers::rpg_bonuses([2, 0, 1, 0, 0], 12, 3, 11, 0, 0)
        .unwrap()
        .with_effect_affix(id("affix_braise"));
    let actor = Actor::new(GridPos::new(3, 1), 10)
        .unwrap()
        .with_primary_attributes(PrimaryAttributes::new(5, 5, 5, 5, 5))
        .with_body_profile(BodyProfile::new(10, 0).unwrap())
        .with_equipped_weapon(
            id("couteau_de_camp"),
            Some(bonus.clone()),
            &game.rules.weapons,
        )
        .unwrap();
    let enemy = game.spawn_actor(actor).unwrap();
    game.drain_events();
    (game, enemy, bonus)
}

#[test]
fn carried_weapon_drives_enemy_attack_passives_and_real_special_effect() {
    let (mut game, enemy, bonus) = fixture();
    let (_, attack, weapon, effects, _) = game.attack_details(enemy, 0).unwrap();
    let base = game.rules.weapons.get(&id("couteau_de_camp")).unwrap();
    assert_eq!(weapon, Some(base.id().clone()));
    assert_eq!(attack, bonus.modify_weapon_attack(base.attack()));
    assert!(!effects.is_empty());
    assert_eq!(
        game.actor_effective_primary_attributes(enemy)
            .unwrap()
            .value(PrimaryAttribute::Power),
        7
    );
    assert_eq!(
        game.actors
            .get(enemy)
            .unwrap()
            .primary_attributes()
            .unwrap()
            .value(PrimaryAttribute::Power),
        5
    );
    assert!(game.actors.get(enemy).unwrap().maximum_integrity() >= 21);
    game.perform_attack(enemy, 0, game.player).unwrap();
    assert!(
        game.actors
            .get(game.player)
            .unwrap()
            .statuses()
            .any(|status| status.definition == id("burning"))
    );
    assert!(game.events().iter().any(|event| matches!(event,
        GameEvent::AttackPerformed { attacker, weapon: Some(item), .. }
        if *attacker == enemy && item == &id("couteau_de_camp"))));
}

#[test]
fn death_drops_once_without_reroll_on_occupied_tile_and_both_objects_can_be_picked_up() {
    let (mut game, enemy, bonus) = fixture();
    let at = game.actors.get(enemy).unwrap().position();
    game.spawn_ground_item(at, id("repair_patch"), 1).unwrap();
    let rng = format!("{:?}", game.rng);
    game.apply_damage_to(
        Some(game.player),
        enemy,
        DamagePacket::new(500, DamageType::Kinetic, 0),
    )
    .unwrap();
    assert_eq!(format!("{:?}", game.rng), rng);
    assert_eq!(game.ground_items.count_at(at), 2);
    assert!(
        game.apply_damage_to(
            Some(game.player),
            enemy,
            DamagePacket::new(500, DamageType::Kinetic, 0)
        )
        .is_err()
    );
    assert_eq!(game.ground_items.count_at(at), 2);
    assert_eq!(
        game.events()
            .iter()
            .filter(|event| matches!(event, GameEvent::ActorEquipmentDropped { .. }))
            .count(),
        1
    );
    game.actors.move_to(game.player, at).unwrap();
    for _ in 0..2 {
        assert_eq!(
            game.process_player_command(GameCommand::PickUp),
            CommandOutcome::Applied
        );
    }
    assert_eq!(game.ground_items.count_at(at), 0);
    let found = game
        .player_inventory
        .iter()
        .find(|entry| entry.item() == &id("couteau_de_camp"))
        .unwrap();
    assert_eq!(found.magic_modifiers(), Some(bonus));
    assert!(
        game.player_inventory
            .iter()
            .any(|entry| entry.item() == &id("repair_patch"))
    );
}

#[test]
fn environmental_death_transfers_same_weapon_and_natural_attacks_never_become_loot() {
    let (mut game, enemy, bonus) = fixture();
    game.apply_status_damage_to(None, enemy, DamagePacket::new(500, DamageType::Thermal, 0))
        .unwrap();
    let ground = game.ground_items.iter().next().unwrap().1;
    assert_eq!(ground.magic_modifiers(), Some(bonus));
    assert_eq!(ground.item(), &id("couteau_de_camp"));
    let animal = game
        .spawn_actor(
            Actor::new(GridPos::new(5, 1), 5)
                .unwrap()
                .with_attack(AttackProfile::melee(DamageType::Kinetic, 2)),
        )
        .unwrap();
    game.apply_damage_to(
        Some(game.player),
        animal,
        DamagePacket::new(500, DamageType::Kinetic, 0),
    )
    .unwrap();
    assert_eq!(game.ground_items.iter().count(), 1);
}

#[test]
fn snapshot_preserves_wounded_carrier_and_refuses_falsified_weapon() {
    let (mut game, enemy, bonus) = fixture();
    game.apply_damage_to(None, enemy, DamagePacket::new(1, DamageType::Kinetic, 0))
        .unwrap();
    let hp = game.actors.get(enemy).unwrap().integrity();
    game.drain_events();
    let world = crate::game::WorldState::single(game);
    let bytes = world.recovery_snapshot_bytes().unwrap();
    let restored =
        crate::game::WorldState::from_recovery_snapshot_bytes(&bytes, world.rules().clone())
            .unwrap();
    assert_eq!(restored.actors().get(enemy).unwrap().integrity(), hp);
    assert_eq!(
        restored
            .actors()
            .get(enemy)
            .unwrap()
            .equipped_weapon()
            .unwrap()
            .modifiers(),
        Some(&bonus)
    );
    assert_eq!(format!("{world:?}"), format!("{restored:?}"));

    let (mut invalid, enemy, _) = fixture();
    let bad = invalid
        .actors
        .get(enemy)
        .unwrap()
        .clone()
        .with_attack(AttackProfile::melee(DamageType::Kinetic, 99));
    *invalid.actors.get_mut(enemy).unwrap() = bad;
    let world = crate::game::WorldState::single(invalid);
    assert!(
        crate::game::WorldState::from_recovery_snapshot_bytes(
            &world.recovery_snapshot_bytes().unwrap(),
            world.rules().clone()
        )
        .is_err()
    );
}

#[test]
fn recovered_weapon_can_be_sold_saved_and_bought_back_with_identical_properties() {
    use crate::content::{
        GambleScalingDefinition, MerchantDefinition, MerchantGambleDefinition,
        MerchantOfferDefinition,
    };
    use crate::game::{WorldState, ZoneInfo};
    let (mut game, enemy, bonus) = fixture();
    let at = game.actors.get(enemy).unwrap().position();
    game.apply_damage_to(
        Some(game.player),
        enemy,
        DamagePacket::new(500, DamageType::Kinetic, 0),
    )
    .unwrap();
    game.actors.move_to(game.player, at).unwrap();
    assert_eq!(
        game.process_player_command(GameCommand::PickUp),
        CommandOutcome::Applied
    );
    let instance = game
        .player_inventory
        .iter()
        .find(|item| item.item() == &id("couteau_de_camp"))
        .unwrap()
        .instance();
    let provider = game
        .spawn_actor(Actor::new(GridPos::new(3, 2), 10).unwrap())
        .unwrap();
    game.drain_events();
    let mut world = WorldState::single(game);
    world
        .enable(ZoneInfo {
            id: id("loot_trade"),
            name: "Marché".into(),
            kind: id("city"),
            depth: 0,
        })
        .unwrap();
    world
        .register_merchant(
            id("loot_trade"),
            provider,
            100,
            MerchantDefinition::new(
                GridPos::new(3, 2),
                10,
                1000,
                vec![MerchantOfferDefinition {
                    item: id("couteau_de_camp"),
                    initial_stock: 1,
                    buy_price: 60,
                    sell_price: 20,
                    minimum_depth: 0,
                    maximum_depth: None,
                }],
                vec![MerchantGambleDefinition {
                    item: id("veste_matelassee"),
                    initial_stock: 1,
                    price: 120,
                }],
                GambleScalingDefinition::new(3, 1, 12).unwrap(),
            )
            .unwrap(),
            108,
        )
        .unwrap();
    assert_eq!(
        world.process_player_command(GameCommand::SellItem {
            merchant: provider,
            item: instance
        }),
        CommandOutcome::Applied
    );
    world.drain_events();
    let mut world = WorldState::from_recovery_snapshot_bytes(
        &world.recovery_snapshot_bytes().unwrap(),
        world.rules().clone(),
    )
    .unwrap();
    let view = world.npc_interaction(provider).unwrap();
    let [crate::game::NpcService::Trade { resale, .. }] = view.services.as_slice() else {
        panic!("shop absent");
    };
    assert_eq!(resale.len(), 1);
    assert_eq!(resale[0].magic_modifiers, Some(bonus.clone()));
    assert_eq!(
        world.process_player_command(GameCommand::BuyResaleItem {
            merchant: provider,
            listing: resale[0].listing
        }),
        CommandOutcome::Applied
    );
    let item = world
        .player_inventory()
        .iter()
        .find(|item| item.item() == &id("couteau_de_camp"))
        .unwrap();
    assert_eq!(item.magic_modifiers(), Some(bonus));
}
