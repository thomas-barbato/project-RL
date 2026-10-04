use super::*;
use project_rl::loot::{EquipmentQuality, EquipmentSource};
use std::collections::BTreeSet;

#[test]
fn reinforced_armor_models_keep_names_tradeoff_and_only_protection_progresses() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let loot = loot_for_generation_version(&loot, &rules.items, CURRENT_GENERATION_VERSION);
    let app = AsciiApp::from_seed(
        133,
        rules.clone(),
        texts.clone(),
        loot.clone(),
        expeditions.clone(),
    )
    .unwrap();
    let names = [
        "Brigandine Cognefer",
        "Plastron Bastion",
        "Armure Cerbère",
        "Carapace Chrysalide",
        "Manteau de mue",
        "Combinaison Fantôme",
    ];
    let mut profiles = Vec::new();
    for (index, id) in armor_model_ids().iter().enumerate() {
        let item = rules.items.get(id).unwrap();
        assert_eq!(
            texts.resolve(DISPLAY_LOCALE, item.name_key()),
            Some(names[index])
        );
        let profile = item.equipment().unwrap();
        assert_eq!(profile.armor(), index as u16 + 2);
        assert_eq!(profile.evasion_penalty(), 5);
        assert_eq!(profile.slot().as_str(), "core:body_armor");
        assert!(item.effects().is_empty());
        assert!(rules.weapons.get(id).is_none());
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("content/core/items")
            .join(format!(
                "{}.json5",
                id.as_str().strip_prefix("core:").unwrap()
            ));
        let mut data: serde_json::Value =
            json5::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        data.as_object_mut().unwrap().remove("id");
        data.as_object_mut().unwrap().remove("name_key");
        data["equipment"]["armor"] = 0.into();
        profiles.push(data);
        assert!(
            app.item_card_lines(id, None, false)
                .iter()
                .any(|line| line == "Esquive : -5")
        );
    }
    assert!(profiles.windows(2).all(|pair| pair[0] == pair[1]));
    assert_eq!(
        rules
            .items
            .get(&"core:veste_matelassee".parse().unwrap())
            .unwrap()
            .equipment()
            .unwrap()
            .evasion_penalty(),
        0
    );
}

#[test]
fn reinforced_armor_models_are_layer_bound_white_or_enchanted_and_tradeable() {
    let (rules, _, loot, expeditions) = ascii_game_content().unwrap();
    let loot = loot_for_generation_version(&loot, &rules.items, CURRENT_GENERATION_VERSION);
    let original = expeditions
        .get(&"core:starter_expedition".parse().unwrap())
        .unwrap()
        .hub_merchant
        .clone()
        .unwrap();
    for depth in 0..=7 {
        let expected = armor_model_ids()[usize::from(depth.min(5))].clone();
        let mut rng = GameRng::from_seed(133 + u64::from(depth));
        let mut qualities = BTreeSet::new();
        for index in 0..2000 {
            let magical = index % 2 == 0;
            let rolled = loot
                .equipment()
                .draw(
                    EquipmentSource::HumanoidSite,
                    depth,
                    if magical {
                        EquipmentQuality::Enchanted
                    } else {
                        EquipmentQuality::White
                    },
                    &mut rng,
                )
                .unwrap()
                .unwrap();
            if !ARMOR_MODEL_IDS.contains(&rolled.item.as_str()) {
                continue;
            }
            assert_eq!(rolled.item, expected);
            assert_eq!(rolled.modifiers.is_some(), magical);
            if let Some(bonus) = rolled.modifiers {
                assert!(bonus.effect_affix().is_none());
                assert_eq!(bonus.armor_bonus(), 0);
                assert!(bonus.named_affixes().is_some());
            }
            qualities.insert(magical);
        }
        assert_eq!(qualities.len(), 2);
        let (shop, quotes) =
            equipment_commerce::shop_definition(original.clone(), loot.equipment(), depth).unwrap();
        let offers: Vec<_> = shop
            .offers
            .iter()
            .filter(|offer| ARMOR_MODEL_IDS.contains(&offer.item.as_str()))
            .collect();
        assert_eq!(offers.len(), 1);
        assert_eq!(offers[0].item, expected);
        assert!(shop.gambles.iter().any(|gamble| gamble.item == expected));
        for id in armor_model_ids() {
            assert!(quotes.iter().any(|quote| quote.0 == id));
        }
        for source in [
            EquipmentSource::Robot,
            EquipmentSource::MechanicalSite,
            EquipmentSource::OrganicCreature,
            EquipmentSource::AnomalousBeing,
        ] {
            assert!(
                loot.equipment()
                    .draw(source, depth, EquipmentQuality::White, &mut rng)
                    .unwrap()
                    .is_none()
            );
        }
    }
}
