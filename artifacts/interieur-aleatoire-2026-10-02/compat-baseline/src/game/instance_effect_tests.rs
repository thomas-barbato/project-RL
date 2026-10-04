use super::*;
use crate::combat::DamageType;
use crate::content::ContentId;
use crate::effects::DamageFalloff;
use crate::entity::MagicItemModifiers;
use crate::weapon::{WeaponCatalog, WeaponEffectAffixDefinition, WeaponEffectOrigin};
use crate::world::{NeighborMode, TerrainPropagationPolicy};

fn id(name: &str) -> ContentId {
    format!("test:{name}").parse().unwrap()
}

fn fixture() -> (GameState, Vec<ItemInstanceId>, EntityId, EntityId) {
    let mut weapons = WeaponCatalog::default();
    weapons
        .register(
            WeaponDefinition::new(
                id("blade"),
                "blade.name".into(),
                "blade.desc".into(),
                AttackProfile::melee(DamageType::Kinetic, 6),
            )
            .unwrap(),
        )
        .unwrap();
    for (name, effect) in [
        (
            "siphon",
            WeaponEffect::life_steal(50, 3, id("living"), WeaponEffectTrigger::OnDamage).unwrap(),
        ),
        (
            "burst",
            WeaponEffect::radial_damage(
                RadialDamageEffect {
                    maximum_cost: 1,
                    neighbor_mode: NeighborMode::CardinalAndDiagonal,
                    propagation_policy: TerrainPropagationPolicy::blocked_by_walls(1),
                    damage: DamagePacket::new(2, DamageType::Electrical, 0),
                    falloff: DamageFalloff::None,
                },
                WeaponEffectTrigger::OnHit,
                WeaponEffectOrigin::Impact,
                false,
            )
            .unwrap(),
        ),
    ] {
        weapons
            .register_effect_affix(
                WeaponEffectAffixDefinition::new(
                    id(name),
                    format!("{name}.suffix"),
                    format!("{name}.description"),
                    vec![effect],
                )
                .unwrap(),
            )
            .unwrap();
    }
    let mut game = GameState::new_with_rules(
        Map::from_ascii("#########\n#.......#\n#.......#\n#.......#\n#.......#\n#########")
            .unwrap(),
        GridPos::new(2, 2),
        14,
        GameRules {
            weapons,
            player_maximum_integrity: 30,
            player_weapon_slots: vec![id("hand"), id("secondary")],
            player_starting_weapons: vec![id("blade")],
            player_starting_equipment: vec![Some(id("blade"))],
            ..Default::default()
        },
    )
    .unwrap()
    .with_starting_magic_equipment([
        (id("blade"), MagicItemModifiers::effect_only(id("siphon"))),
        (id("blade"), MagicItemModifiers::effect_only(id("burst"))),
    ])
    .unwrap()
    .with_starting_player_integrity(10)
    .unwrap();
    let items = game
        .player_inventory()
        .iter()
        .map(|entry| entry.instance())
        .collect();
    let primary = game
        .spawn_actor(
            Actor::new(GridPos::new(3, 2), 100)
                .unwrap()
                .with_tags([id("living")])
                .with_evasion_disabled(),
        )
        .unwrap();
    let neighbor = game
        .spawn_actor(
            Actor::new(GridPos::new(4, 2), 100)
                .unwrap()
                .with_evasion_disabled(),
        )
        .unwrap();
    game.drain_events();
    (game, items, primary, neighbor)
}

#[test]
fn same_base_instances_use_only_their_own_effect_without_changing_base_damage() {
    for selected in 0..3 {
        let (mut game, items, primary, neighbor) = fixture();
        let base = game.rules.weapons.get(&id("blade")).unwrap().clone();
        game.equip_player_weapon(1, items[if selected == 1 { 2 } else { 1 }])
            .unwrap();
        if selected != 0 {
            game.equip_player_weapon(0, items[selected]).unwrap();
        }
        game.drain_events();
        assert_eq!(
            game.process_player_command(GameCommand::Attack {
                slot: 0,
                target: primary
            }),
            CommandOutcome::Applied
        );
        assert_eq!(
            game.actors.get(game.player).unwrap().integrity(),
            if selected == 1 { 13 } else { 10 }
        );
        assert_eq!(
            game.actors.get(primary).unwrap().integrity(),
            if selected == 2 { 92 } else { 94 }
        );
        assert_eq!(
            game.actors.get(neighbor).unwrap().integrity(),
            if selected == 2 { 98 } else { 100 }
        );
        assert_eq!(game.rules.weapons.get(&id("blade")), Some(&base));
        assert_eq!(
            game.player_item_weapon(items[selected]).unwrap().attack(),
            base.attack()
        );
        let sources: Vec<_> = game
            .events()
            .iter()
            .filter_map(|event| match event {
                GameEvent::PropagationResolved { weapon_effect, .. } => weapon_effect.clone(),
                _ => None,
            })
            .collect();
        assert_eq!(
            sources,
            if selected == 2 {
                vec![(id("burst"), 0)]
            } else {
                vec![]
            }
        );
    }
}

#[test]
fn effect_only_instance_survives_ground_snapshot_and_pickup() {
    let (game, items, _, _) = fixture();
    let mut world = crate::game::WorldState::single(game);
    let expected = world
        .player_inventory()
        .get(items[1])
        .unwrap()
        .magic_modifiers();
    assert_eq!(
        world.process_player_command(GameCommand::DropItem { item: items[1] }),
        CommandOutcome::Applied
    );
    world.drain_events();
    let bytes = world.recovery_snapshot_bytes().unwrap();
    world = crate::game::WorldState::from_recovery_snapshot_bytes(&bytes, world.rules().clone())
        .unwrap();
    assert_eq!(
        world
            .ground_items()
            .iter()
            .next()
            .unwrap()
            .1
            .magic_modifiers(),
        expected
    );
    assert_eq!(
        world.process_player_command(GameCommand::PickUp),
        CommandOutcome::Applied
    );
    let recovered = world
        .player_inventory()
        .iter()
        .find(|entry| entry.magic_modifiers() == expected)
        .unwrap();
    assert_eq!(recovered.item(), &id("blade"));
    assert_ne!(recovered.instance(), items[1]);
    assert_eq!(
        world
            .player_inventory()
            .iter()
            .filter(|entry| entry.item() == &id("blade"))
            .count(),
        3
    );
    assert!(
        world
            .player_inventory()
            .get(items[0])
            .unwrap()
            .magic_modifiers()
            .is_none()
    );
}

#[test]
fn unknown_effect_references_are_rejected_on_setup_and_snapshot_load() {
    let (game, _, _, _) = fixture();
    assert!(
        game.with_starting_magic_equipment([(
            id("blade"),
            MagicItemModifiers::effect_only(id("missing"))
        )])
        .is_err()
    );
    let (mut invalid, _, _, _) = fixture();
    invalid
        .player_inventory
        .add_magic(
            id("blade"),
            None,
            MagicItemModifiers::effect_only(id("missing")),
        )
        .unwrap();
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
fn intrinsic_and_instance_effects_keep_distinct_animation_sources() {
    let (game, _, _, _) = fixture();
    let mut catalog = game.rules.weapons.clone();
    let intrinsic = game
        .rules
        .weapons
        .effect_affix(&id("burst"))
        .unwrap()
        .effects()[0]
        .clone();
    catalog
        .register(
            WeaponDefinition::new(
                id("charged_blade"),
                "name".into(),
                "desc".into(),
                AttackProfile::melee(DamageType::Kinetic, 6),
            )
            .unwrap()
            .with_effects([intrinsic]),
        )
        .unwrap();
    let resolved = catalog
        .resolve_instance(&id("charged_blade"), Some(&id("siphon")))
        .unwrap();
    assert_eq!(resolved.effects().len(), 2);
    assert!(resolved.effects()[0].source().is_none());
    assert_eq!(resolved.effects()[1].source(), Some(&(id("siphon"), 0)));
    assert_eq!(
        catalog.effect_at_source(&id("siphon"), 0).unwrap().kind(),
        resolved.effects()[1].kind()
    );
}

#[test]
fn effect_and_numeric_affixes_survive_sale_snapshot_and_repurchase() {
    use crate::content::{
        GambleScalingDefinition, MerchantDefinition, MerchantGambleDefinition,
        MerchantOfferDefinition,
    };
    use crate::game::{NpcService, WorldState, ZoneInfo};
    use crate::item::{
        EquipmentAffixId, EquipmentNameGrammar, EquipmentProfile, ItemDefinition, ItemKind,
        NamedEquipmentAffixes, RolledEquipmentAffix,
    };
    let (mut game, _, _, _) = fixture();
    game.rules
        .items
        .register(
            ItemDefinition::new(
                id("armor"),
                "name".into(),
                "desc".into(),
                1,
                ItemKind::Armor,
                Some(EquipmentProfile::new(id("body"), 1).unwrap()),
                vec![],
            )
            .unwrap(),
        )
        .unwrap();
    let bonus = MagicItemModifiers::from_affixes(
        NamedEquipmentAffixes::new(
            EquipmentNameGrammar::FeminineSingular,
            &[RolledEquipmentAffix::new(EquipmentAffixId::Vitality, 1, 15).unwrap()],
        )
        .unwrap(),
    )
    .with_effect_affix(id("siphon"));
    let instance = game
        .player_inventory
        .add_magic(id("blade"), None, bonus.clone())
        .unwrap();
    let merchant = game
        .spawn_actor(Actor::new(GridPos::new(2, 3), 10).unwrap())
        .unwrap();
    let mut world = WorldState::single(game);
    world
        .enable(ZoneInfo {
            id: id("market"),
            name: "Marché".into(),
            kind: id("city"),
            depth: 0,
        })
        .unwrap();
    world
        .register_merchant(
            id("market"),
            merchant,
            200,
            MerchantDefinition::new(
                GridPos::new(2, 3),
                10,
                500,
                vec![MerchantOfferDefinition {
                    item: id("blade"),
                    initial_stock: 1,
                    buy_price: 30,
                    sell_price: 15,
                    minimum_depth: 0,
                    maximum_depth: None,
                }],
                vec![MerchantGambleDefinition {
                    item: id("armor"),
                    initial_stock: 1,
                    price: 60,
                }],
                GambleScalingDefinition::new(3, 1, 12).unwrap(),
            )
            .unwrap(),
            7,
        )
        .unwrap();
    assert_eq!(
        world.process_player_command(GameCommand::SellItem {
            merchant,
            item: instance
        }),
        CommandOutcome::Applied
    );
    world.drain_events();
    world = WorldState::from_recovery_snapshot_bytes(
        &world.recovery_snapshot_bytes().unwrap(),
        world.rules().clone(),
    )
    .unwrap();
    let interaction = world.npc_interaction(merchant).unwrap();
    let [NpcService::Trade { resale, .. }] = interaction.services.as_slice() else {
        panic!("missing trade")
    };
    assert_eq!(resale[0].magic_modifiers, Some(bonus.clone()));
    let listing = resale[0].listing;
    assert_eq!(
        world.process_player_command(GameCommand::BuyResaleItem { merchant, listing }),
        CommandOutcome::Applied
    );
    let bought = world
        .player_inventory()
        .iter()
        .find(|entry| entry.magic_modifiers() == Some(bonus.clone()))
        .unwrap();
    assert_ne!(bought.instance(), instance);
    assert_eq!(
        world
            .player_item_weapon(bought.instance())
            .unwrap()
            .effects()
            .len(),
        1
    );
    assert!(
        world
            .rules()
            .weapons
            .get(&id("blade"))
            .unwrap()
            .effects()
            .is_empty()
    );
}

#[test]
fn ranged_only_affix_cannot_be_attached_to_a_melee_instance() {
    let (game, _, _, _) = fixture();
    let mut catalog = game.rules.weapons.clone();
    catalog
        .register_effect_affix(
            WeaponEffectAffixDefinition::new(
                id("ricochet"),
                "suffix".into(),
                "description".into(),
                vec![
                    WeaponEffect::ricochet(3, DamagePacket::new(2, DamageType::Kinetic, 0))
                        .unwrap(),
                ],
            )
            .unwrap(),
        )
        .unwrap();
    assert!(
        catalog
            .resolve_instance(&id("blade"), Some(&id("ricochet")))
            .is_none()
    );
}

#[test]
fn generated_zone_loot_keeps_its_properties_through_pending_snapshot_travel_and_pickup() {
    use crate::game::{GroundLootBlueprint, WorldState, ZoneBlueprint, ZoneInfo};
    use crate::item::{
        EquipmentAffixId, EquipmentNameGrammar, NamedEquipmentAffixes, RolledEquipmentAffix,
    };
    let (game, _, _, _) = fixture();
    let modifiers = MagicItemModifiers::from_affixes(
        NamedEquipmentAffixes::new(
            EquipmentNameGrammar::FeminineSingular,
            &[RolledEquipmentAffix::new(EquipmentAffixId::Power, 1, 2).unwrap()],
        )
        .unwrap(),
    )
    .with_effect_affix(id("siphon"));
    let info = |name| ZoneInfo {
        id: id(name),
        name: name.into(),
        kind: id("site"),
        depth: 0,
    };
    let mut world = WorldState::single(game);
    world.enable(info("a")).unwrap();
    world
        .add_zone(ZoneBlueprint {
            info: info("b"),
            map: Map::from_ascii(
                "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
            )
            .unwrap(),
            entrance: GridPos::new(2, 2),
            seed: 78,
            actors: vec![],
            threat_sources: vec![],
            loot: vec![
                GroundLootBlueprint::new(GridPos::new(2, 2), id("blade"), 1)
                    .with_owner(id("owners"))
                    .with_magic_modifiers(modifiers.clone())
                    .unwrap(),
            ],
        })
        .unwrap();
    world
        .connect(id("a"), GridPos::new(2, 3), id("b"), GridPos::new(2, 2))
        .unwrap();
    let expected = format!("{world:?}");
    world = WorldState::from_recovery_snapshot_bytes(
        &world.recovery_snapshot_bytes().unwrap(),
        world.rules().clone(),
    )
    .unwrap();
    assert_eq!(format!("{world:?}"), expected);
    assert_eq!(
        world.process_player_command(GameCommand::Interact {
            target: GridPos::new(2, 3)
        }),
        CommandOutcome::Applied
    );
    let ground = world.ground_items().iter().next().unwrap().1;
    assert_eq!(ground.magic_modifiers(), Some(modifiers.clone()));
    assert_eq!(ground.owner(), Some(&id("owners")));
    assert_eq!(
        world.process_player_command(GameCommand::PickUp),
        CommandOutcome::Applied
    );
    assert!(
        world
            .player_inventory()
            .iter()
            .any(|entry| entry.magic_modifiers() == Some(modifiers.clone()))
    );
    assert!(
        GroundLootBlueprint::new(GridPos::new(1, 1), id("blade"), 2)
            .with_magic_modifiers(modifiers)
            .is_err()
    );
}
