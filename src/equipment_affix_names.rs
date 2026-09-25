//! French instance names and property lines. Reading a name never rolls loot.
use super::*;
use project_rl::entity::MagicItemModifiers;
use project_rl::item::{
    EquipmentAffixId as Id, EquipmentNameGrammar as Grammar, NamedEquipmentAffixes,
};

fn affix_form(id: Id, grammar: Grammar) -> &'static str {
    let index = match grammar {
        Grammar::MasculineSingular => 0,
        Grammar::FeminineSingular => 1,
        Grammar::MasculinePlural => 2,
        Grammar::FemininePlural => 3,
    };
    match id {
        Id::Power => ["puissant", "puissante", "puissants", "puissantes"][index],
        Id::Coordination => ["adroit", "adroite", "adroits", "adroites"][index],
        Id::Resilience => ["robuste", "robuste", "robustes", "robustes"][index],
        Id::Perception => "de vigilance",
        Id::Processing => ["lucide", "lucide", "lucides", "lucides"][index],
        Id::Accuracy => ["précis", "précise", "précis", "précises"][index],
        Id::ArmorPenetration => ["pénétrant", "pénétrante", "pénétrants", "pénétrantes"][index],
        Id::Vitality => "de vitalité",
        Id::EnergyReserve => "de réserve",
        Id::HeatDissipation => "de dissipation",
    }
}

fn is_suffix(id: Id) -> bool {
    matches!(
        id,
        Id::Perception | Id::Vitality | Id::EnergyReserve | Id::HeatDissipation
    )
}

fn priority(id: Id) -> u8 {
    match id {
        Id::Accuracy => 30,
        Id::ArmorPenetration => 25,
        Id::Vitality => 20,
        Id::EnergyReserve | Id::HeatDissipation => 15,
        _ => 10,
    }
}

fn compose_name(
    base: &str,
    affixes: Option<NamedEquipmentAffixes>,
    effect_suffix: Option<&str>,
) -> String {
    let choose = |suffix| {
        affixes.and_then(|affixes| {
            affixes
                .iter()
                .map(|roll| roll.id())
                .filter(|id| is_suffix(*id) == suffix)
                .min_by_key(|id| (std::cmp::Reverse(priority(*id)), id.as_str()))
                .map(|id| affix_form(id, affixes.grammar()))
        })
    };
    [
        Some(base),
        choose(false),
        effect_suffix.or_else(|| choose(true)),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join(" ")
}

/// These remain the laboratory's authored effects, NOT rolled instance effects.
pub(super) fn lab_effect_suffix(kind: &str) -> Option<&'static str> {
    Some(match kind {
        "impact" => "de décharge",
        "bearer" => "de couronne",
        "conduction" => "de conduction",
        "burning" => "de braise",
        "caustic" => "d'acide",
        "healing" => "de siphon",
        "piercing" => "de perforation",
        "aegis" => "d'égide",
        "percussion" => "de percussion",
        "fracture" => "de marquage",
        "ricochet" => "de ricochet",
        "catalysis" => "de fournaise",
        _ => return None,
    })
}

pub(super) fn affix_detail_lines(bonus: MagicItemModifiers) -> Vec<String> {
    let Some(affixes) = bonus.named_affixes() else {
        return Vec::new();
    };
    affixes
        .iter()
        .map(|roll| {
            let property = match roll.id() {
                Id::Power => "Puissance",
                Id::Coordination => "Coordination",
                Id::Resilience => "Résilience",
                Id::Perception => "Perception",
                Id::Processing => "Traitement",
                Id::Accuracy => "Précision",
                Id::ArmorPenetration => "Pénétration d'armure",
                Id::Vitality => "PV maximum",
                Id::EnergyReserve => "Énergie maximale",
                Id::HeatDissipation => "Dissipation",
            };
            let unit = if roll.id() == Id::HeatDissipation {
                "/tour"
            } else {
                ""
            };
            format!("{property} +{}{unit}", roll.value())
        })
        .collect()
}

impl AsciiApp {
    /// A gamble never uses an instance's identified name, even if its caller
    /// accidentally supplies properties intended for a resale row.
    pub(super) fn trade_item_name(
        &self,
        mode: NpcTradeMode,
        id: &ItemId,
        bonus: Option<MagicItemModifiers>,
    ) -> String {
        if mode == NpcTradeMode::Gamble {
            self.item_name(id)
        } else {
            self.equipment_name(id, bonus)
        }
    }

    pub(super) fn equipment_name(&self, id: &ItemId, bonus: Option<MagicItemModifiers>) -> String {
        let original = self.item_name(id);
        let affixes = bonus.as_ref().and_then(MagicItemModifiers::named_affixes);
        let effect_suffix = bonus
            .as_ref()
            .and_then(|bonus| bonus.effect_affix())
            .and_then(|id| self.game.rules().weapons.effect_affix(id))
            .and_then(|effect| self.texts.resolve(DISPLAY_LOCALE, effect.suffix_key()));
        // No heuristic based on a rendered glyph, material or translated name.
        let lab = id
            .as_str()
            .strip_prefix("lab:")
            .and_then(|name| name.split_once('_'));
        if let Some((family @ ("blade" | "rifle"), _)) = lab
            && let Some(suffix) = effect_suffix
        {
            return compose_name(
                if family == "blade" { "Lame" } else { "Fusil" },
                affixes,
                Some(suffix),
            );
        }
        if let Some((family @ ("blade" | "rifle"), "reference")) = lab {
            return compose_name(
                if family == "blade" {
                    "Lame témoin"
                } else {
                    "Fusil témoin"
                },
                affixes,
                effect_suffix,
            );
        }
        if let Some((family @ ("blade" | "rifle"), kind)) = lab
            && let Some(suffix) = lab_effect_suffix(kind)
        {
            return compose_name(
                if family == "blade" { "Lame" } else { "Fusil" },
                affixes,
                Some(suffix),
            );
        }
        compose_name(&original, affixes, effect_suffix)
    }

    pub(super) fn inventory_entry_name(&self, entry: &InventoryEntry) -> String {
        self.equipment_name(entry.item(), entry.magic_modifiers())
    }

    pub(super) fn equipped_instance_name(&self, slot: u8) -> Option<String> {
        let item = self.game.equipped_player_weapon_item(slot)?;
        self.game
            .player_inventory()
            .get(item)
            .map(|entry| self.inventory_entry_name(entry))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use project_rl::item::RolledEquipmentAffix as Roll;

    #[test]
    fn names_agree_and_titles_never_hide_properties_from_the_details() {
        for (grammar, base, expected) in [
            (
                Grammar::MasculineSingular,
                "Fusil",
                "Fusil précis de siphon",
            ),
            (Grammar::FeminineSingular, "Lame", "Lame précise de siphon"),
            (
                Grammar::MasculinePlural,
                "Gantelets",
                "Gantelets précis de siphon",
            ),
            (
                Grammar::FemininePlural,
                "Bottes",
                "Bottes précises de siphon",
            ),
        ] {
            let affixes = NamedEquipmentAffixes::new(
                grammar,
                &[
                    Roll::new(Id::Accuracy, 1, 10).unwrap(),
                    Roll::new(Id::Vitality, 1, 15).unwrap(),
                    Roll::new(Id::Power, 1, 2).unwrap(),
                ],
            )
            .unwrap();
            assert_eq!(
                compose_name(base, Some(affixes), Some("de siphon")),
                expected
            );
            let details = affix_detail_lines(MagicItemModifiers::from_affixes(affixes));
            assert_eq!(details.len(), 3);
            assert!(details.iter().any(|line| line == "PV maximum +15"));
            assert!(details.iter().any(|line| line == "Puissance +2"));
            assert!(compose_name(base, Some(affixes), None).ends_with("de vitalité"));
        }
        assert_eq!(compose_name("Armure témoin", None, None), "Armure témoin");
        assert!(affix_detail_lines(MagicItemModifiers::new(2, 10).unwrap()).is_empty());
    }

    #[test]
    fn bonus_details_show_only_the_property_and_value_without_affix_labels_or_dashes() {
        let expected = [
            "Puissance +2",
            "Coordination +2",
            "Résilience +2",
            "Perception +2",
            "Traitement +2",
            "Précision +10",
            "Pénétration d'armure +1",
            "PV maximum +15",
            "Énergie maximale +15",
            "Dissipation +1/tour",
        ];
        for (id, expected) in Id::ALL.into_iter().zip(expected) {
            for grammar in [Grammar::MasculineSingular, Grammar::FemininePlural] {
                let affixes = NamedEquipmentAffixes::new(
                    grammar,
                    &[Roll::new(id, 1, id.range(1).unwrap().1).unwrap()],
                )
                .unwrap();
                let lines = affix_detail_lines(MagicItemModifiers::from_affixes(affixes));
                assert_eq!(lines, [expected]);
                assert!(!lines[0].contains(['-', '—', '–']));
                assert!(!lines[0].contains(affix_form(id, grammar)));
            }
        }
    }

    #[test]
    fn affix_forms_and_priority_stay_in_sync_with_the_catalogue() {
        let catalogue: serde_json::Value =
            serde_json::from_str(include_str!("../docs/catalogues/equipements.json")).unwrap();
        for id in Id::ALL {
            let a = catalogue["affixes"]
                .as_array()
                .unwrap()
                .iter()
                .find(|a| a["id"] == id.as_str())
                .unwrap();
            assert_eq!(a["priority"], priority(id));
            assert_eq!(
                a["placement"],
                if is_suffix(id) { "suffix" } else { "prefix" }
            );
            for (grammar, key) in [
                (Grammar::MasculineSingular, "ms"),
                (Grammar::FeminineSingular, "fs"),
                (Grammar::MasculinePlural, "mp"),
                (Grammar::FemininePlural, "fp"),
            ] {
                assert_eq!(
                    a["forms"][if is_suffix(id) { "all" } else { key }],
                    affix_form(id, grammar)
                );
            }
        }
        for a in catalogue["affixes"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|a| a["category"] == "special_effect")
        {
            assert_eq!(
                lab_effect_suffix(a["lab_profile"].as_str().unwrap()),
                a["forms"]["all"].as_str()
            );
        }
    }
}
