use super::*;
use crate::content::ContentId;
use crate::entity::{BODY_APPEARANCE_SLOT, MagicItemModifiers};
use crate::item::{EquipmentProfile, ItemCatalog, ItemDefinition, ItemKind};
use crate::stats::{BodyProfile, PhysicalRules};

fn fixture() -> (GameState, ItemInstanceId, ItemInstanceId, ItemInstanceId) {
    let mut items = ItemCatalog::default();
    let body: ContentId = "core:body_armor".parse().unwrap();
    let ids: Vec<ContentId> = ["test:real_armor", "test:appearance", "test:helmet"]
        .into_iter()
        .map(|id| id.parse().unwrap())
        .collect();
    for (i, id) in ids.iter().enumerate() {
        let slot = if i == 2 {
            "core:head_armor".parse().unwrap()
        } else {
            body.clone()
        };
        items
            .register(
                ItemDefinition::new(
                    id.clone(),
                    "name".into(),
                    "description".into(),
                    1,
                    ItemKind::Armor,
                    Some(
                        EquipmentProfile::new(slot, [3, 99, 5][i])
                            .unwrap()
                            .with_evasion_penalty(i as u16 * 10),
                    ),
                    vec![],
                )
                .unwrap()
                .with_mass_grams(1000)
                .unwrap(),
            )
            .unwrap();
    }
    let mut game = GameState::new_with_rules(
        Map::filled(8, 6, Terrain::Floor).unwrap(),
        GridPos::new(2, 2),
        412,
        GameRules {
            items,
            player_armor_slots: vec![body.clone(), "core:head_armor".parse().unwrap()],
            player_starting_items: vec![
                super::super::StartingItemStack::new(ids[0].clone(), 1),
                super::super::StartingItemStack::new(ids[2].clone(), 1),
            ],
            player_inventory_capacity: 8,
            player_body_profile: Some(BodyProfile::new(20, 0).unwrap().with_base_armor(1)),
            physical_rules: Some(PhysicalRules::default()),
            armor_rules: Some(crate::combat::ArmorRules::default()),
            ..GameRules::default()
        },
    )
    .unwrap();
    let real = game
        .player_inventory
        .iter()
        .find(|e| e.item() == &ids[0])
        .unwrap()
        .instance();
    let helmet = game
        .player_inventory
        .iter()
        .find(|e| e.item() == &ids[2])
        .unwrap()
        .instance();
    let bonus = MagicItemModifiers::rpg_bonuses([9; 5], 30, 8, 100, 80, 9).unwrap();
    let look = game
        .player_inventory
        .add_magic(ids[1].clone(), None, bonus)
        .unwrap();
    game.equip_player_item(body, real).unwrap();
    game.drain_events();
    (game, real, look, helmet)
}

fn appearance(item: ItemInstanceId) -> GameCommand {
    GameCommand::EquipItem {
        slot: BODY_APPEARANCE_SLOT.parse().unwrap(),
        item,
    }
}

#[test]
fn body_appearance_changes_only_rendering_and_round_trips_in_the_existing_snapshot() {
    let (mut game, real, look, _) = fixture();
    let before = (
        game.turn,
        game.rng,
        game.player_energy,
        game.player_heat,
        format!("{:?}", game.actors),
        game.actor_armor_profile(game.player),
        game.actor_equipped_evasion_modifier(game.player),
        game.player_effective_primary_attributes(),
        game.player_inventory.clone(),
    );
    assert!(!appearance(look).consumes_time_on_success());
    assert_eq!(
        game.process_player_command(appearance(look)),
        CommandOutcome::AppliedWithoutTime
    );
    assert_eq!(game.player_visible_body_item(), Some(look));
    assert_eq!(
        game.player_equipment
            .equipped(&"core:body_armor".parse().unwrap()),
        Some(real)
    );
    assert_eq!(
        (
            game.turn,
            game.rng,
            game.player_energy,
            game.player_heat,
            format!("{:?}", game.actors),
            game.actor_armor_profile(game.player),
            game.actor_equipped_evasion_modifier(game.player),
            game.player_effective_primary_attributes(),
            game.player_inventory.clone()
        ),
        before
    );
    assert_eq!(game.player_attribute_bonus(PrimaryAttribute::Power), 0);
    assert!(!game.player_equipment.iter().any(|(_, item)| item == look));
    assert!(game.events().is_empty());
    let bytes = bincode::serialize(&game.snapshot().unwrap()).unwrap();
    let mut restored =
        GameState::from_snapshot(bincode::deserialize(&bytes).unwrap(), game.rules.clone());
    assert_eq!(restored.player_visible_body_item(), Some(look));
    assert_eq!(restored.player_equipment, game.player_equipment);
    assert_eq!(
        restored.process_player_command(appearance(look)),
        CommandOutcome::AppliedWithoutTime
    );
    assert_eq!(restored.player_visible_body_item(), Some(real));
    assert_eq!(restored.player_equipment.body_appearance(), None);
}

#[test]
fn body_appearance_rejections_are_atomic_and_only_accept_owned_torsos() {
    let (mut game, _, look, helmet) = fixture();
    game.process_player_command(appearance(look));
    let before = format!("{game:?}");
    for item in [helmet, serde_json::from_str("9999").unwrap()] {
        assert!(matches!(
            game.process_player_command(appearance(item)),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(format!("{game:?}"), before);
    }
}

#[test]
fn body_appearance_can_reference_real_equipment_and_clears_when_its_item_is_dropped() {
    let (mut game, real, look, _) = fixture();
    game.process_player_command(appearance(real));
    assert_eq!(
        game.player_equipment
            .equipped(&"core:body_armor".parse().unwrap()),
        Some(real)
    );
    assert_eq!(game.player_equipment.iter().count(), 1);
    game.equip_player_item("core:body_armor".parse().unwrap(), look)
        .unwrap();
    assert_eq!(game.player_visible_body_item(), Some(real));
    assert_eq!(
        game.process_player_command(GameCommand::DropItem { item: real }),
        CommandOutcome::Applied
    );
    assert_eq!(game.player_equipment.body_appearance(), None);
    assert_eq!(game.player_visible_body_item(), Some(look));
    game.process_player_command(appearance(look));
    let before_refused_drop = format!("{game:?}");
    assert!(matches!(
        game.process_player_command(GameCommand::DropItem { item: look }),
        CommandOutcome::Rejected(CommandRejection::CannotDropItemHere)
    ));
    assert_eq!(format!("{game:?}"), before_refused_drop);
    game.process_player_command(GameCommand::Move(Direction::East));
    assert_eq!(
        game.process_player_command(GameCommand::DropItem { item: look }),
        CommandOutcome::Applied
    );
    assert_eq!(game.player_equipment.body_appearance(), None);
    assert_eq!(game.player_visible_body_item(), None);
}
