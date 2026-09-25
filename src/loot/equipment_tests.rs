use super::*;
use crate::content::{ContentLoader, LoadedContent};

fn content() -> LoadedContent {
    ContentLoader::load(
        &[std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content")],
        &semver::Version::new(0, 1, 0),
    )
    .unwrap()
}

#[test]
fn real_bases_generate_distinct_persistent_instances_without_changing_definitions() {
    let content = content();
    let catalogue = content.loot().equipment();
    assert_eq!(catalogue.iter().count(), 18);
    let before = format!("{:?}{:?}", content.weapons(), content.items());
    let mut a = GameRng::from_seed(17);
    let mut b = a;
    let mut whites = 0;
    let mut magical = 0;
    let mut effects = 0;
    let mut models = BTreeSet::new();
    for index in 0..2000 {
        let item = catalogue
            .draw(
                EquipmentSource::HumanoidSite,
                index % 6,
                EquipmentQuality::Random {
                    enchanted_percent: 35,
                },
                &mut a,
            )
            .unwrap()
            .unwrap();
        assert_eq!(
            Some(item.clone()),
            catalogue
                .draw(
                    EquipmentSource::HumanoidSite,
                    index % 6,
                    EquipmentQuality::Random {
                        enchanted_percent: 35
                    },
                    &mut b
                )
                .unwrap()
        );
        models.insert(item.item.clone());
        if let Some(modifiers) = item.modifiers {
            magical += 1;
            assert_eq!(
                bincode::deserialize::<MagicItemModifiers>(
                    &bincode::serialize(&modifiers).unwrap()
                )
                .unwrap(),
                modifiers
            );
            let stats: Vec<_> = modifiers
                .named_affixes()
                .map(|affixes| affixes.iter().collect())
                .unwrap_or_default();
            assert!(
                (1..=3).contains(&(stats.len() + usize::from(modifiers.effect_affix().is_some())))
            );
            assert_eq!(
                stats
                    .iter()
                    .map(|roll| roll.id())
                    .collect::<BTreeSet<_>>()
                    .len(),
                stats.len()
            );
            assert!(stats.iter().all(|roll| roll.tier() == item.tier));
            if let Some(effect) = modifiers.effect_affix() {
                effects += 1;
                assert!(
                    content
                        .weapons()
                        .resolve_instance(&item.item, Some(effect))
                        .is_some()
                );
                assert!(content.items().get(&item.item).is_none());
            }
        } else {
            whites += 1;
        }
    }
    assert_eq!(models.len(), 18);
    assert!(whites > magical && magical > 400 && effects > 5);
    assert_eq!(a.state(), b.state());
    assert_eq!(
        format!("{:?}{:?}", content.weapons(), content.items()),
        before
    );
}

#[test]
fn absent_provenance_and_invalid_requests_never_consume_randomness() {
    let content = content();
    let catalogue = content.loot().equipment();
    let mut rng = GameRng::from_seed(42);
    for source in [
        EquipmentSource::Robot,
        EquipmentSource::MechanicalSite,
        EquipmentSource::OrganicCreature,
        EquipmentSource::AnomalousBeing,
    ] {
        assert_eq!(
            catalogue
                .draw(source, u16::MAX, EquipmentQuality::Enchanted, &mut rng)
                .unwrap(),
            None
        );
        assert_eq!(rng.state(), 42);
    }
    assert!(
        catalogue
            .draw(
                EquipmentSource::HumanoidSite,
                0,
                EquipmentQuality::Random {
                    enchanted_percent: 101
                },
                &mut rng
            )
            .is_err()
    );
    assert_eq!(rng.state(), 42);
}

#[test]
fn depth_favors_available_upper_tiers_but_surface_exceptions_remain_possible() {
    let content = content();
    let catalogue = content.loot().equipment();
    let mut rng = GameRng::from_seed(7);
    for depth in 0..6 {
        let mut counts = [0usize; 6];
        for _ in 0..10000 {
            let item = catalogue
                .draw(
                    EquipmentSource::HumanoidSite,
                    depth,
                    EquipmentQuality::White,
                    &mut rng,
                )
                .unwrap()
                .unwrap();
            assert!(item.modifiers.is_none());
            counts[usize::from(item.tier - 1)] += 1;
        }
        // Each layer predominantly draws its own tier, without making other
        // tiers impossible. All three families' models remain ordinary bases.
        let expected = DEPTH_WEIGHTS[usize::from(depth)];
        assert_eq!(
            counts.iter().position(|n| Some(n) == counts.iter().max()),
            Some(usize::from(depth))
        );
        for (count, weight) in counts.into_iter().zip(expected) {
            assert!(
                count.abs_diff(usize::from(weight)) < 250,
                "depth {depth}: {counts:?}"
            );
        }
        assert!(
            counts.iter().all(|count| *count > 0),
            "depth {depth}: {counts:?}"
        );
    }
}

#[test]
fn each_live_family_has_one_named_base_for_every_playable_layer() {
    let content = content();
    let design: serde_json::Value =
        serde_json::from_str(include_str!("../../docs/catalogues/equipements.json")).unwrap();
    let mut families = BTreeMap::<ContentId, BTreeSet<u8>>::new();
    for (_, base) in content.loot().equipment().iter() {
        assert!(
            families
                .entry(base.family.clone())
                .or_default()
                .insert(base.tier)
        );
        let authored = design["families"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|family| family["bases"].as_array().unwrap())
            .find(|item| item["id"] == base.item.as_str().replacen("core:", "catalog:", 1))
            .unwrap();
        assert_eq!(authored["tier"], base.tier);
        assert_eq!(
            authored["grammar"],
            if base.grammar == EquipmentNameGrammar::MasculineSingular {
                "ms"
            } else {
                "fs"
            }
        );
        if let Some(weapon) = content.weapons().get(&base.item) {
            assert!(
                weapon.effects().is_empty(),
                "specials must belong to instances"
            );
        } else {
            let armor = content
                .items()
                .get(&base.item)
                .unwrap()
                .equipment()
                .unwrap();
            assert_eq!(armor.slot().as_str(), "core:body_armor");
            assert_eq!(armor.armor(), u16::from(base.tier));
        }
    }
    assert_eq!(families.len(), 3);
    for tiers in families.values() {
        assert_eq!(tiers, &(1..=6).collect::<BTreeSet<_>>());
    }
}

#[test]
fn independent_affix_weights_make_specials_and_some_stats_rarer() {
    let content = content();
    let catalogue = content.loot().equipment();
    let mut rng = GameRng::from_seed(83);
    let (mut power, mut penetration, mut burning, mut discharge) = (0, 0, 0, 0);
    for _ in 0..30000 {
        let item = catalogue
            .draw(
                EquipmentSource::HumanoidSite,
                0,
                EquipmentQuality::Enchanted,
                &mut rng,
            )
            .unwrap()
            .unwrap();
        let modifiers = item.modifiers.unwrap();
        if let Some(stats) = modifiers.named_affixes() {
            for roll in stats.iter() {
                power += usize::from(roll.id() == EquipmentAffixId::Power);
                penetration += usize::from(roll.id() == EquipmentAffixId::ArmorPenetration);
            }
        }
        match modifiers.effect_affix().map(ContentId::as_str) {
            Some("core:affix_braise") => burning += 1,
            Some("core:affix_decharge") => discharge += 1,
            _ => {}
        }
    }
    assert!(power > penetration * 2, "{power} vs {penetration}");
    assert!(
        penetration > burning && burning > discharge * 2 && discharge > 100,
        "{penetration}, {burning}, {discharge}"
    );
}

#[test]
fn weights_support_disabled_entries_large_totals_and_exclusive_effect_groups() {
    let mut rng = GameRng::from_seed(6);
    for _ in 0..100 {
        assert_eq!(weighted_index([0, u32::MAX, 0].into_iter(), &mut rng), 1);
        assert!(weighted_index([u32::MAX, u32::MAX].into_iter(), &mut rng) < 2);
    }
    let content = content();
    let mut base = content
        .loot()
        .equipment()
        .iter()
        .find(|(_, base)| !base.effects.is_empty())
        .unwrap()
        .1
        .clone();
    base.stats.clear();
    let mut catalogue = EquipmentLootCatalog::default();
    catalogue
        .register(base, content.items(), content.weapons())
        .unwrap();
    for _ in 0..100 {
        let modifiers = catalogue
            .draw(
                EquipmentSource::HumanoidSite,
                0,
                EquipmentQuality::Enchanted,
                &mut rng,
            )
            .unwrap()
            .unwrap()
            .modifiers
            .unwrap();
        assert!(modifiers.named_affixes().is_none());
        assert!(modifiers.effect_affix().is_some());
    }
}

#[test]
fn malformed_or_incompatible_definitions_are_rejected_before_catalogue_mutation() {
    let content = content();
    let base = content.loot().equipment().iter().next().unwrap().1.clone();
    for case in 0..7 {
        let mut malformed = base.clone();
        match case {
            0 => malformed.sources = vec![EquipmentSource::Robot],
            1 => malformed.tier = 0,
            2 => malformed.stats.push(malformed.stats[0].clone()),
            3 => malformed.effects[0].effect = "core:missing_affix".parse().unwrap(),
            4 => {
                malformed.stats.clear();
                malformed.effects.clear();
            }
            5 => malformed.item = "core:veste_matelassee".parse().unwrap(),
            _ => malformed.sources.push(malformed.sources[0]),
        }
        let mut catalogue = EquipmentLootCatalog::default();
        assert!(
            catalogue
                .register(malformed, content.items(), content.weapons())
                .is_err(),
            "case {case}"
        );
        assert!(catalogue.is_empty());
    }
    let mut catalogue = EquipmentLootCatalog::default();
    catalogue
        .register(base.clone(), content.items(), content.weapons())
        .unwrap();
    assert!(
        catalogue
            .register(base, content.items(), content.weapons())
            .is_err()
    );
}

#[test]
fn runtime_stat_weights_match_the_shared_design_catalogue() {
    let design: serde_json::Value =
        serde_json::from_str(include_str!("../../docs/catalogues/equipements.json")).unwrap();
    let content = content();
    for (_, base) in content.loot().equipment().iter() {
        for choice in &base.stats {
            let authored = design["affixes"]
                .as_array()
                .unwrap()
                .iter()
                .find(|a| a["id"] == choice.affix.as_str())
                .unwrap();
            assert_eq!(authored["weight"], choice.weight);
        }
    }
    for (depth, row) in DEPTH_WEIGHTS.into_iter().enumerate() {
        assert_eq!(
            serde_json::to_value(row).unwrap(),
            design["distribution"]["depth_weights"][depth]["weights"]
        );
    }
}

#[test]
fn adding_models_does_not_make_their_family_more_frequent() {
    let content = content();
    let mut weapons = content.weapons().clone();
    let mut catalogue = content.loot().equipment().clone();
    let base = catalogue
        .iter()
        .find(|(_, base)| base.item.as_str() == "core:couteau_de_camp")
        .unwrap()
        .1
        .clone();
    for index in 0..8 {
        let mut variant = base.clone();
        variant.item = format!("test:extra_knife_{index}").parse().unwrap();
        weapons
            .register(
                crate::weapon::WeaponDefinition::new(
                    variant.item.clone(),
                    "name".into(),
                    "desc".into(),
                    weapons.get(&base.item).unwrap().attack(),
                )
                .unwrap(),
            )
            .unwrap();
        catalogue
            .register(variant, content.items(), &weapons)
            .unwrap();
    }
    let mut rng = GameRng::from_seed(76);
    let mut counts = BTreeMap::<ContentId, usize>::new();
    for _ in 0..6000 {
        let item = catalogue
            .draw(
                EquipmentSource::HumanoidSite,
                0,
                EquipmentQuality::White,
                &mut rng,
            )
            .unwrap()
            .unwrap();
        *counts
            .entry(catalogue.bases[&item.item].family.clone())
            .or_default() += 1;
    }
    assert_eq!(counts.len(), 3);
    assert!(
        counts.values().all(|count| (1700..2300).contains(count)),
        "{counts:?}"
    );
}
