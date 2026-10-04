use super::*;
use project_rl::loot::{EquipmentQuality, EquipmentSource};
use std::collections::BTreeSet;

#[test]
fn accessory_models_have_approved_names_slots_and_reduced_persistent_bonuses() {
    let (rules, texts, loot, _) = ascii_game_content().unwrap();
    let names = [
        "Calotte Cognefer",
        "Heaume Bastion",
        "Casque Cerbère",
        "Masque Chrysalide",
        "Capuche de mue",
        "Visière Fantôme",
        "Gants Cognefer",
        "Gantelets Bastion",
        "Mitaines Cerbère",
        "Gantelets Chrysalide",
        "Gants de mue",
        "Gants Fantôme",
        "Brodequins Cognefer",
        "Solerets Bastion",
        "Bottes Cerbère",
        "Bottines Chrysalide",
        "Bottes de mue",
        "Bottines Fantôme",
    ];
    for (index, id) in accessory_model_ids().iter().enumerate() {
        let item = rules.items.get(id).unwrap();
        let profile = item.equipment().unwrap();
        assert_eq!(
            texts.resolve(DISPLAY_LOCALE, item.name_key()),
            Some(names[index])
        );
        assert_eq!(profile.slot().as_str(), ACCESSORY_SLOTS[index / 6]);
        assert!(profile.reduced_affixes());
        assert_eq!(profile.evasion_penalty(), 0);
        assert_eq!(
            profile.armor(),
            if index < 6 && index % 6 >= 4 { 2 } else { 1 }
        );
        assert!(item.effects().is_empty());
        let mut rng = GameRng::from_seed(134);
        let mut counts = BTreeSet::new();
        for _ in 0..100 {
            let bonus = loot.equipment().enchant_base(id, 4, &mut rng).unwrap();
            assert!(bonus.effect_affix().is_none());
            let rolls: Vec<_> = bonus.named_affixes().unwrap().iter().collect();
            counts.insert(rolls.len());
            assert!(
                rolls
                    .iter()
                    .all(|roll| roll.tier() == ((index % 6 + 1) as u8).div_ceil(2))
            );
            let bytes = bincode::serialize(&bonus).unwrap();
            assert_eq!(
                bincode::deserialize::<project_rl::entity::MagicItemModifiers>(&bytes).unwrap(),
                bonus
            );
        }
        assert_eq!(counts, BTreeSet::from([1, 2]));
    }
}

#[test]
fn accessory_models_follow_layers_and_never_drop_from_robots_or_animals() {
    let (rules, _, loot, expeditions) = ascii_game_content().unwrap();
    let loot = loot_for_generation_version(&loot, &rules.items, CITY_DECOR_GENERATION_VERSION);
    let original = expeditions
        .get(&"core:starter_expedition".parse().unwrap())
        .unwrap()
        .hub_merchant
        .clone()
        .unwrap();
    for depth in 0..=7 {
        let tier = (depth + 1).min(6) as u8;
        let mut rng = GameRng::from_seed(134 + u64::from(depth));
        let mut white = BTreeSet::new();
        let mut magical = BTreeSet::new();
        for i in 0..2400 {
            let enchanted = i % 2 == 0;
            let rolled = loot
                .equipment()
                .draw(
                    EquipmentSource::HumanoidSite,
                    depth,
                    if enchanted {
                        EquipmentQuality::Enchanted
                    } else {
                        EquipmentQuality::White
                    },
                    &mut rng,
                )
                .unwrap()
                .unwrap();
            if !ACCESSORY_MODEL_IDS.contains(&rolled.item.as_str()) {
                continue;
            }
            assert_eq!(rolled.tier, tier);
            assert_eq!(rolled.modifiers.is_some(), enchanted);
            if enchanted {
                magical.insert(rolled.item);
            } else {
                white.insert(rolled.item);
            }
        }
        assert_eq!(white.len(), 3);
        assert_eq!(white, magical);
        let (shop, quotes) =
            equipment_commerce::shop_definition(original.clone(), loot.equipment(), depth).unwrap();
        for id in &white {
            assert!(shop.offers.iter().any(|offer| &offer.item == id));
            assert!(shop.gambles.iter().any(|offer| &offer.item == id));
        }
        for id in accessory_model_ids() {
            assert!(quotes.iter().any(|quote| quote.0 == id));
        }
        for source in [
            EquipmentSource::Robot,
            EquipmentSource::MechanicalSite,
            EquipmentSource::OrganicCreature,
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
