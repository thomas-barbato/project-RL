use super::*;
use project_rl::loot::EquipmentQuality;
use std::collections::{BTreeMap, BTreeSet};

fn is_current_melee(id: &ContentId) -> bool {
    MELEE_MODEL_IDS.contains(&id.as_str()) || SWORD_MODEL_IDS.contains(&id.as_str())
}

fn content() -> (GameRules, TextCatalog, LootCatalog, ExpeditionCatalog) {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let loot = loot_for_generation_version(&loot, &rules.items, CITY_DECOR_GENERATION_VERSION);
    (rules, texts, loot, expeditions)
}

#[test]
fn melee_models_have_six_tiers_with_only_damage_changes_and_complete_presentation() {
    let (rules, texts, loot, _) = content();
    let visuals = ascii_visual_cue_catalog().unwrap();
    let mut families = BTreeMap::<ContentId, Vec<_>>::new();
    let mut names = BTreeSet::new();
    for (_, base) in loot
        .equipment()
        .iter()
        .filter(|(id, _)| is_current_melee(id))
    {
        let weapon = rules.weapons.get(&base.item).unwrap();
        assert_eq!(
            weapon.attack().delivery(),
            project_rl::combat::AttackDelivery::Melee
        );
        assert!(weapon.supply().is_none());
        assert!(weapon.effects().is_empty());
        assert!(visuals.get(&base.item).is_some());
        assert!(
            texts
                .resolve(DISPLAY_LOCALE, weapon.description_key())
                .is_some()
        );
        assert!(
            names.insert(
                texts
                    .resolve(DISPLAY_LOCALE, weapon.name_key())
                    .unwrap()
                    .to_owned()
            )
        );
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("content/core/weapons")
            .join(format!(
                "{}.json5",
                base.item.as_str().strip_prefix("core:").unwrap()
            ));
        let mut data: serde_json::Value =
            json5::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        let damage = data["attack"]["damage"]["amount"].as_u64().unwrap();
        if base.family.as_str() == "core:swords" {
            let names = [
                "Épée courte",
                "Glaive",
                "Fauchon",
                "Épée longue",
                "Épée bâtarde",
                "Espadon",
            ];
            assert_eq!(
                texts.resolve(DISPLAY_LOCALE, weapon.name_key()),
                Some(names[usize::from(base.tier - 1)])
            );
            assert_eq!(damage, 3 + 2 * u64::from(base.tier));
            assert_eq!(weapon.attack().accuracy_modifier(), 0);
            assert_eq!(data["attack"]["damage"]["penetration"], 0);
            assert!(data.get("capabilities").is_none());
            assert!(data.get("launcher").is_none());
        }
        data.as_object_mut().unwrap().remove("id");
        data.as_object_mut().unwrap().remove("name_key");
        data["attack"]["damage"]["amount"] = 0.into();
        families
            .entry(base.family.clone())
            .or_default()
            .push((base.tier, damage, data));
    }
    assert_eq!(names.len(), 30);
    assert_eq!(families.len(), 5);
    for (_, mut models) in families {
        models.sort_by_key(|model| model.0);
        assert_eq!(
            models.iter().map(|model| model.0).collect::<Vec<_>>(),
            (1..=6).collect::<Vec<_>>()
        );
        for pair in models.windows(2) {
            assert!(pair[1].1 > pair[0].1);
            assert_eq!(pair[0].2, pair[1].2);
        }
    }
}

#[test]
fn melee_models_remain_common_white_or_enchanted_at_every_layer_and_in_shops() {
    let (rules, _, loot, expeditions) = content();
    assert_eq!(loot.equipment().iter().count(), 108);
    let original = expeditions
        .get(&"core:starter_expedition".parse().unwrap())
        .unwrap()
        .hub_merchant
        .clone()
        .unwrap();
    for depth in 0..=6 {
        let tier = (depth + 1).min(6) as u8;
        let mut rng = GameRng::from_seed(900 + u64::from(depth));
        let mut whites = BTreeMap::<ContentId, usize>::new();
        let mut enchanted = BTreeSet::new();
        for i in 0..2500 {
            let quality = if i % 2 == 0 {
                EquipmentQuality::White
            } else {
                EquipmentQuality::Enchanted
            };
            let rolled = loot
                .equipment()
                .draw(EquipmentSource::HumanoidSite, depth, quality, &mut rng)
                .unwrap()
                .unwrap();
            assert!(!LEGACY_MELEE_IDS.contains(&rolled.item.as_str()));
            if !is_current_melee(&rolled.item) {
                continue;
            }
            assert_eq!(rolled.tier, tier);
            if quality == EquipmentQuality::White {
                assert!(rolled.modifiers.is_none());
                *whites.entry(rolled.item).or_default() += 1;
            } else {
                let bonus = rolled.modifiers.unwrap();
                assert!(
                    rules
                        .weapons
                        .resolve_instance(&rolled.item, bonus.effect_affix())
                        .is_some()
                );
                enchanted.insert(rolled.item);
            }
        }
        assert_eq!(whites.len(), 5);
        assert_eq!(enchanted.len(), 5);
        assert!(whites.values().all(|count| *count >= 40));
        let (shop, quotes) =
            equipment_commerce::shop_definition(original.clone(), loot.equipment(), depth).unwrap();
        let offers: Vec<_> = shop
            .offers
            .iter()
            .filter(|offer| is_current_melee(&offer.item))
            .collect();
        assert_eq!(offers.len(), 5);
        for offer in offers {
            assert!(whites.contains_key(&offer.item));
            assert!(shop.gambles.iter().any(|gamble| gamble.item == offer.item));
        }
        // Every model keeps a resale quote even outside its native layer.
        for id in melee_model_ids().into_iter().chain(sword_model_ids()) {
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

#[test]
fn melee_models_increase_actual_damage_without_changing_reach_or_spending_ammunition() {
    let (base_rules, _, loot, _) = content();
    let mut damages = BTreeMap::<ContentId, Vec<_>>::new();
    for (_, base) in loot
        .equipment()
        .iter()
        .filter(|(id, _)| is_current_melee(id))
    {
        let mut rules = base_rules.clone();
        rules.hit_rules = None;
        rules.player_starting_weapons = vec![base.item.clone()];
        rules.player_weapon_slots.truncate(1);
        rules.player_starting_equipment = vec![Some(base.item.clone())];
        let range = rules.weapons.get(&base.item).unwrap().attack().range();
        assert_eq!(
            range,
            if base.family.as_str() == "core:spears" {
                2
            } else {
                1
            }
        );
        let map =
            project_rl::world::Map::filled(18, 15, project_rl::world::Terrain::Floor).unwrap();
        let mut game = GameState::new_with_rules(map, GridPos::new(5, 6), 131, rules).unwrap();
        let target = game
            .spawn_actor(
                Actor::new(GridPos::new(5 + i32::from(range), 6), 1000)
                    .unwrap()
                    .with_evasion_disabled(),
            )
            .unwrap();
        let outside = game
            .spawn_actor(
                Actor::new(GridPos::new(5 + i32::from(range) + 1, 7), 1000)
                    .unwrap()
                    .with_evasion_disabled(),
            )
            .unwrap();
        let ammo = game.player_matter();
        let turn = game.turn();
        assert!(matches!(
            game.process_player_command(GameCommand::Attack {
                slot: 0,
                target: outside
            }),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(game.turn(), turn);
        for _ in 0..3 {
            assert_eq!(
                game.process_player_command(GameCommand::Attack { slot: 0, target }),
                CommandOutcome::Applied,
                "{}",
                base.item
            );
        }
        assert_eq!(game.player_matter(), ammo);
        assert_eq!(game.turn(), turn + 3);
        assert_eq!(game.actors().get(outside).unwrap().integrity(), 1000);
        damages.entry(base.family.clone()).or_default().push((
            base.tier,
            1000 - game.actors().get(target).unwrap().integrity(),
        ));
    }
    for (family, mut tiers) in damages {
        tiers.sort();
        for pair in tiers.windows(2) {
            assert!(
                pair[1].1 > pair[0].1,
                "actual damage must increase: {family}: {tiers:?}"
            );
        }
    }
}
