use super::*;
use crate::combat::DamageType;
use crate::weapon::WeaponSupply;

fn id(name: &str) -> crate::content::ContentId {
    format!("test:{name}").parse().unwrap()
}

fn fixture() -> (GameState, EntityId) {
    let mut rules = GameRules {
        weapon_matter_item: Some(id("matter")),
        player_energy_capacity: 20,
        player_starting_energy: 10,
        player_energy_regeneration: 1,
        player_weapon_slots: vec![id("slot0"), id("slot1"), id("slot2")],
        player_starting_weapons: vec![id("rifle"), id("cannon"), id("beam")],
        player_starting_equipment: vec![Some(id("rifle")), Some(id("cannon")), Some(id("beam"))],
        player_starting_items: vec![super::super::StartingItemStack::new(id("matter"), 5)],
        ..GameRules::default()
    };
    rules
        .items
        .register(
            ItemDefinition::new(
                id("matter"),
                "matter".into(),
                "matter".into(),
                999,
                crate::item::ItemKind::Material,
                None,
                vec![],
            )
            .unwrap(),
        )
        .unwrap();
    for (name, supply) in [
        ("rifle", WeaponSupply::Matter { amount: 1 }),
        ("cannon", WeaponSupply::Matter { amount: 3 }),
        ("beam", WeaponSupply::Energy { amount: 6 }),
    ] {
        let attack = AttackProfile::new(
            6,
            DistanceMetric::Euclidean,
            true,
            DamageType::Piercing,
            1,
            0,
        );
        rules
            .weapons
            .register(
                WeaponDefinition::new(id(name), name.into(), name.into(), attack)
                    .unwrap()
                    .with_ammunition_capacity(10)
                    .unwrap()
                    .with_supply(supply)
                    .unwrap(),
            )
            .unwrap();
    }
    let map = Map::from_ascii("#########\n#.......#\n#.......#\n#########").unwrap();
    let mut game = GameState::new_with_rules(map, GridPos::new(2, 1), 123, rules).unwrap();
    let target = game
        .spawn_actor(Actor::new(GridPos::new(5, 1), 100).unwrap())
        .unwrap();
    game.drain_events();
    (game, target)
}

#[test]
fn shared_supplies_draw_from_one_stock_and_failed_shots_are_free() {
    let (mut game, target) = fixture();
    assert_eq!(game.player_matter(), Some(5));
    for (slot, left) in [(0, 4), (1, 1)] {
        assert_eq!(
            game.process_player_command(GameCommand::Attack { slot, target }),
            CommandOutcome::Applied
        );
        assert_eq!(game.player_matter(), Some(left));
    }
    let before = format!("{game:?}");
    assert_eq!(
        game.process_player_command(GameCommand::Attack { slot: 1, target }),
        CommandOutcome::Rejected(CommandRejection::InsufficientMatter {
            required: 3,
            available: 1
        })
    );
    assert_eq!(format!("{game:?}"), before);
    assert_eq!(game.player_weapon_ammunition(&id("rifle")), None);
    // An unequipped copy cannot refill the shared stock.
    let copy = game.player_inventory.add(id("rifle"), 1, 1).unwrap()[0];
    assert_eq!(
        game.process_player_command(GameCommand::EquipWeapon {
            slot: 0,
            item: copy
        }),
        CommandOutcome::Applied
    );
    assert_eq!(game.player_matter(), Some(1));
}

#[test]
fn shared_supplies_energy_recovers_per_real_turn_not_on_failed_actions_or_loading() {
    let (mut game, target) = fixture();
    assert_eq!(
        game.process_player_command(GameCommand::Attack { slot: 2, target }),
        CommandOutcome::Applied
    );
    assert_eq!(game.player_energy().available(), 5); // 10 - 6 + 1
    assert_eq!(game.player_matter(), Some(5));
    let before = format!("{game:?}");
    assert_eq!(
        game.process_player_command(GameCommand::Attack { slot: 2, target }),
        CommandOutcome::Rejected(CommandRejection::InsufficientEnergy {
            required: 6,
            available: 5
        })
    );
    assert_eq!(format!("{game:?}"), before);
    assert_eq!(
        game.process_player_command(GameCommand::Wait),
        CommandOutcome::Applied
    );
    assert_eq!(game.player_energy().available(), 6);
    game.drain_events();
    let world = super::super::WorldState::single(game);
    let bytes = world.recovery_snapshot_bytes().unwrap();
    let restored =
        super::super::WorldState::from_recovery_snapshot_bytes(&bytes, world.rules().clone())
            .unwrap();
    assert_eq!(restored.recovery_snapshot_bytes().unwrap(), bytes);
    assert_eq!(restored.player_energy().available(), 6);
    assert_eq!(restored.player_matter(), Some(5));
}

#[test]
fn shared_supplies_compatible_enemies_drop_once_and_fauna_does_not() {
    let (mut game, organic) = fixture();
    let at = GridPos::new(4, 2);
    let robot = game
        .spawn_actor(
            Actor::new(at, 2)
                .unwrap()
                .with_electronic_system(ElectronicSystemProfile::new(0, 10, 10, 0, 0).unwrap())
                .with_player_relation(crate::social::PlayerRelation::Hostile),
        )
        .unwrap();
    game.spawn_ground_item(at, id("matter"), 1).unwrap();
    let hit = DamagePacket::new(200, DamageType::Kinetic, 0);
    game.apply_damage_to(Some(game.player), robot, hit).unwrap();
    assert_eq!(game.ground_items.count_at(at), 2);
    let dropped = game
        .ground_items
        .iter()
        .find(|(_, entry)| entry.quantity() > 1)
        .unwrap()
        .1
        .quantity();
    assert!((2..=6).contains(&dropped));
    assert!(game.apply_damage_to(Some(game.player), robot, hit).is_err());
    game.apply_damage_to(Some(game.player), organic, hit)
        .unwrap();
    assert_eq!(game.ground_items.count_at(at), 2);
    assert_eq!(game.ground_items.count_at(GridPos::new(5, 1)), 0);
    game.actors.move_to(game.player, at).unwrap();
    for _ in 0..2 {
        assert_eq!(
            game.process_player_command(GameCommand::PickUp),
            CommandOutcome::Applied
        );
    }
    assert_eq!(game.player_matter(), Some(6 + u32::from(dropped)));
    assert_eq!(game.ground_items.count_at(at), 0);
}

#[test]
fn shared_supplies_energy_regeneration_is_capped_and_stops_on_death() {
    let (mut game, _) = fixture();
    for _ in 0..30 {
        assert_eq!(
            game.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
    }
    assert_eq!(game.player_energy().available(), 20);
    game.player_energy.spend(5).unwrap();
    game.apply_damage_to(
        None,
        game.player,
        DamagePacket::new(1000, DamageType::Kinetic, 0),
    )
    .unwrap();
    assert!(matches!(
        game.process_player_command(GameCommand::Wait),
        CommandOutcome::Rejected(_)
    ));
    assert_eq!(game.player_energy().available(), 15);
}

#[test]
fn shared_supplies_armed_hostile_drops_matter_alongside_its_weapon() {
    let (mut game, _) = fixture();
    let at = GridPos::new(4, 2);
    let enemy = Actor::new(at, 2)
        .unwrap()
        .with_player_relation(crate::social::PlayerRelation::Hostile)
        .with_equipped_weapon(id("rifle"), Default::default(), &game.rules.weapons)
        .unwrap();
    let target = game.spawn_actor(enemy).unwrap();
    game.apply_damage_to(
        Some(game.player),
        target,
        DamagePacket::new(100, DamageType::Kinetic, 0),
    )
    .unwrap();
    let items: Vec<_> = game
        .ground_items
        .iter()
        .filter(|(_, entry)| entry.position() == at)
        .collect();
    assert_eq!(items.len(), 2);
    assert!(items.iter().any(|(_, entry)| entry.item() == &id("rifle")));
    assert!(
        items
            .iter()
            .any(|(_, entry)| entry.item() == &id("matter") && (2..=6).contains(&entry.quantity()))
    );
}
