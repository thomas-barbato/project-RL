use super::*;
use project_rl::loot::EquipmentQuality;
use std::collections::{BTreeMap, BTreeSet};

fn projected_content() -> (GameRules, TextCatalog, LootCatalog, ExpeditionCatalog) {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    // Historical layer-bound draws before the expanded model pool (v140).
    let loot = loot_for_generation_version(&loot, &rules.items, CITY_DECOR_GENERATION_VERSION);
    (rules, texts, loot, expeditions)
}

#[test]
fn firearm_models_have_six_tiers_and_only_damage_changes_within_a_family() {
    let (rules, texts, loot, _) = projected_content();
    let mut families = BTreeMap::<String, Vec<_>>::new();
    let visuals = ascii_visual_cue_catalog().unwrap();
    for (_, base) in loot
        .equipment()
        .iter()
        .filter(|(id, _)| FIREARM_MODEL_IDS.contains(&id.as_str()))
    {
        let weapon = rules.weapons.get(&base.item).unwrap();
        assert!(texts.resolve(DISPLAY_LOCALE, weapon.name_key()).is_some());
        assert!(
            texts
                .resolve(DISPLAY_LOCALE, weapon.description_key())
                .is_some()
        );
        assert!(
            visuals.get(&base.item).is_some(),
            "missing animation: {}",
            base.item
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
        let blast = data["launcher"]["damage"]["amount"].as_u64();
        data.as_object_mut().unwrap().remove("id");
        data.as_object_mut().unwrap().remove("name_key");
        data["attack"]["damage"]["amount"] = 0.into();
        if blast.is_some() {
            data["launcher"]["damage"]["amount"] = 0.into();
        }
        families
            .entry(base.family.to_string())
            .or_default()
            .push((base.tier, damage, blast, data));
    }
    assert_eq!(families.len(), 8);
    for (family, mut models) in families {
        models.sort_by_key(|m| m.0);
        assert_eq!(
            models.iter().map(|m| m.0).collect::<Vec<_>>(),
            (1..=6).collect::<Vec<_>>()
        );
        for pair in models.windows(2) {
            assert!(pair[1].1 > pair[0].1, "{family}");
            if let (Some(a), Some(b)) = (pair[0].2, pair[1].2) {
                assert!(b > a);
            }
            assert_eq!(pair[0].3, pair[1].3, "non-damage change: {family}");
        }
    }
}

#[test]
fn firearm_models_are_ordinary_layer_loot_with_independent_affixes_and_provenance() {
    let (rules, _, loot, expeditions) = projected_content();
    assert_eq!(loot.equipment().iter().count(), 108);
    let original = expeditions
        .get(&"core:starter_expedition".parse().unwrap())
        .unwrap()
        .hub_merchant
        .clone()
        .unwrap();
    for depth in 0..=6 {
        let tier = (depth + 1).min(6) as u8;
        let mut rng = GameRng::from_seed(700 + u64::from(depth));
        let mut white = BTreeMap::<ContentId, usize>::new();
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
            assert!(!LEGACY_FIREARM_IDS.contains(&rolled.item.as_str()));
            if FIREARM_MODEL_IDS.contains(&rolled.item.as_str()) {
                assert_eq!(rolled.tier, tier);
                if quality == EquipmentQuality::White {
                    assert!(rolled.modifiers.is_none());
                    *white.entry(rolled.item).or_default() += 1;
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
        }
        assert_eq!(white.len(), 8, "depth {depth}");
        assert_eq!(enchanted.len(), 8);
        assert!(
            white.values().all(|count| *count >= 40),
            "no artificially rare family: {white:?}"
        );
        for source in [
            EquipmentSource::Robot,
            EquipmentSource::OrganicCreature,
            EquipmentSource::MechanicalSite,
        ] {
            assert!(
                loot.equipment()
                    .draw(source, depth, EquipmentQuality::White, &mut rng)
                    .unwrap()
                    .is_none()
            );
        }
        let (shop, _) =
            equipment_commerce::shop_definition(original.clone(), loot.equipment(), depth).unwrap();
        let offers: Vec<_> = shop
            .offers
            .iter()
            .filter(|offer| FIREARM_MODEL_IDS.contains(&offer.item.as_str()))
            .collect();
        assert_eq!(offers.len(), 8);
        for offer in offers {
            assert!(white.contains_key(&offer.item));
            assert!(shop.gambles.iter().any(|gamble| gamble.item == offer.item));
        }
    }
}

#[test]
fn firearm_models_all_fire_with_native_resources_and_current_runs_resume() {
    let (base, texts, loot, expeditions) = projected_content();
    for id in firearm_model_ids() {
        let mut rules = base.clone();
        rules.hit_rules = None;
        rules.player_starting_weapons = vec![id.clone()];
        rules.player_starting_equipment = vec![Some(id.clone())];
        rules.player_starting_energy = 30;
        rules.player_starting_items = vec![StartingItemStack::new(
            "core:weapon_matter".parse().unwrap(),
            100,
        )];
        let map =
            project_rl::world::Map::filled(20, 15, project_rl::world::Terrain::Floor).unwrap();
        let mut game = GameState::new_with_rules(map, GridPos::new(6, 6), 130, rules).unwrap();
        let target = game
            .spawn_actor(
                Actor::new(GridPos::new(9, 6), 1000)
                    .unwrap()
                    .with_evasion_disabled(),
            )
            .unwrap();
        let before = game.player_matter().unwrap();
        assert_eq!(
            game.process_player_command(GameCommand::Attack { slot: 0, target }),
            CommandOutcome::Applied,
            "{id}"
        );
        assert!(
            game.actors().get(target).unwrap().integrity() < 1000,
            "{id}"
        );
        if let Some(project_rl::weapon::WeaponSupply::Matter { amount }) =
            game.rules().weapons.get(&id).unwrap().supply()
        {
            assert_eq!(
                before - game.player_matter().unwrap(),
                u32::from(amount),
                "{id}"
            );
        }
    }
    let app = AsciiApp::from_seed(130, base, texts, loot, expeditions).unwrap();
    let saved = app.suspension().unwrap();
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let restored = AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
    assert_eq!(saved.state, suspension::fingerprint(&restored.game));
}
