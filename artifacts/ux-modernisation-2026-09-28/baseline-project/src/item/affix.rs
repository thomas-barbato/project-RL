//! Stable, rolled identities for numeric equipment properties. The catalogue's
//! base families and special-effect affixes are deliberately not generated here.
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum EquipmentAffixId {
    #[serde(rename = "affix:power")]
    Power,
    #[serde(rename = "affix:coordination")]
    Coordination,
    #[serde(rename = "affix:resilience")]
    Resilience,
    #[serde(rename = "affix:perception")]
    Perception,
    #[serde(rename = "affix:processing")]
    Processing,
    #[serde(rename = "affix:accuracy")]
    Accuracy,
    #[serde(rename = "affix:armor_penetration")]
    ArmorPenetration,
    #[serde(rename = "affix:vitality")]
    Vitality,
    #[serde(rename = "affix:energy_reserve")]
    EnergyReserve,
    #[serde(rename = "affix:heat_dissipation")]
    HeatDissipation,
}

impl EquipmentAffixId {
    pub const ALL: [Self; 10] = [
        Self::Power,
        Self::Coordination,
        Self::Resilience,
        Self::Perception,
        Self::Processing,
        Self::Accuracy,
        Self::ArmorPenetration,
        Self::Vitality,
        Self::EnergyReserve,
        Self::HeatDissipation,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Power => "affix:power",
            Self::Coordination => "affix:coordination",
            Self::Resilience => "affix:resilience",
            Self::Perception => "affix:perception",
            Self::Processing => "affix:processing",
            Self::Accuracy => "affix:accuracy",
            Self::ArmorPenetration => "affix:armor_penetration",
            Self::Vitality => "affix:vitality",
            Self::EnergyReserve => "affix:energy_reserve",
            Self::HeatDissipation => "affix:heat_dissipation",
        }
    }

    /// Prototype power tiers, never character levels or equipment requirements.
    pub fn range(self, tier: u8) -> Option<(u16, u16)> {
        let ranges = match self {
            Self::Power
            | Self::Coordination
            | Self::Resilience
            | Self::Perception
            | Self::Processing => [(1, 2), (1, 3), (2, 4), (3, 5), (4, 7), (5, 9)],
            Self::Accuracy => [(5, 10), (7, 14), (10, 18), (14, 23), (18, 28), (23, 35)],
            Self::ArmorPenetration | Self::HeatDissipation => {
                [(1, 1), (1, 2), (2, 3), (3, 4), (4, 5), (5, 7)]
            }
            Self::Vitality | Self::EnergyReserve => {
                [(5, 15), (10, 20), (15, 30), (20, 40), (30, 55), (40, 75)]
            }
        };
        tier.checked_sub(1)
            .and_then(|index| ranges.get(usize::from(index)).copied())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "UncheckedRoll")]
pub struct RolledEquipmentAffix {
    id: EquipmentAffixId,
    tier: u8,
    value: u16,
}

#[derive(Deserialize)]
struct UncheckedRoll {
    id: EquipmentAffixId,
    tier: u8,
    value: u16,
}

impl TryFrom<UncheckedRoll> for RolledEquipmentAffix {
    type Error = &'static str;
    fn try_from(raw: UncheckedRoll) -> Result<Self, Self::Error> {
        Self::new(raw.id, raw.tier, raw.value).ok_or("invalid equipment affix tier or value")
    }
}

impl RolledEquipmentAffix {
    pub fn new(id: EquipmentAffixId, tier: u8, value: u16) -> Option<Self> {
        let (minimum, maximum) = id.range(tier)?;
        (minimum..=maximum)
            .contains(&value)
            .then_some(Self { id, tier, value })
    }
    pub const fn id(self) -> EquipmentAffixId {
        self.id
    }
    pub const fn tier(self) -> u8 {
        self.tier
    }
    pub const fn value(self) -> u16 {
        self.value
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum EquipmentNameGrammar {
    #[serde(rename = "ms")]
    MasculineSingular,
    #[serde(rename = "fs")]
    FeminineSingular,
    #[serde(rename = "mp")]
    MasculinePlural,
    #[serde(rename = "fp")]
    FemininePlural,
}

/// Three unique properties at most in this first laboratory prototype. White
/// items and legacy anonymous bonuses have no such metadata; never infer it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "UncheckedAffixes")]
pub struct NamedEquipmentAffixes {
    grammar: EquipmentNameGrammar,
    rolls: [Option<RolledEquipmentAffix>; 3],
}

#[derive(Deserialize)]
struct UncheckedAffixes {
    grammar: EquipmentNameGrammar,
    rolls: [Option<RolledEquipmentAffix>; 3],
}

impl TryFrom<UncheckedAffixes> for NamedEquipmentAffixes {
    type Error = &'static str;
    fn try_from(raw: UncheckedAffixes) -> Result<Self, Self::Error> {
        let rolls: Vec<_> = raw.rolls.into_iter().flatten().collect();
        Self::new(raw.grammar, &rolls).ok_or("empty or duplicate equipment affixes")
    }
}

impl NamedEquipmentAffixes {
    pub fn new(grammar: EquipmentNameGrammar, rolls: &[RolledEquipmentAffix]) -> Option<Self> {
        if !(1..=3).contains(&rolls.len()) {
            return None;
        }
        let mut sorted = rolls.to_vec();
        sorted.sort_by_key(|roll| roll.id());
        if sorted.windows(2).any(|pair| pair[0].id() == pair[1].id()) {
            return None;
        }
        let mut stored = [None; 3];
        for (slot, roll) in stored.iter_mut().zip(sorted) {
            *slot = Some(roll);
        }
        Some(Self {
            grammar,
            rolls: stored,
        })
    }
    pub const fn grammar(self) -> EquipmentNameGrammar {
        self.grammar
    }
    pub fn iter(&self) -> impl Iterator<Item = RolledEquipmentAffix> + '_ {
        self.rolls.iter().copied().flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_affixes_match_the_design_catalogue_and_reject_out_of_range_values() {
        let catalogue: serde_json::Value =
            serde_json::from_str(include_str!("../../docs/catalogues/equipements.json")).unwrap();
        for id in EquipmentAffixId::ALL {
            let definition = catalogue["affixes"]
                .as_array()
                .unwrap()
                .iter()
                .find(|entry| entry["id"] == id.as_str())
                .unwrap();
            for tier in 1..=6 {
                let (minimum, maximum) = id.range(tier).unwrap();
                let range = &definition["ranges"][usize::from(tier - 1)];
                assert_eq!(range["minimum"], minimum);
                assert_eq!(range["maximum"], maximum);
                for value in [minimum, maximum] {
                    let roll = RolledEquipmentAffix::new(id, tier, value).unwrap();
                    assert_eq!(serde_json::to_value(roll).unwrap()["id"], id.as_str());
                    assert_eq!(
                        bincode::deserialize::<RolledEquipmentAffix>(
                            &bincode::serialize(&roll).unwrap()
                        )
                        .unwrap(),
                        roll
                    );
                }
                assert!(RolledEquipmentAffix::new(id, tier, minimum - 1).is_none());
                assert!(RolledEquipmentAffix::new(id, tier, maximum + 1).is_none());
            }
            assert!(id.range(0).is_none());
            assert!(id.range(7).is_none());
        }
    }

    #[test]
    fn named_affixes_are_canonical_and_validate_deserialized_values_and_duplicates() {
        let power = RolledEquipmentAffix::new(EquipmentAffixId::Power, 1, 2).unwrap();
        let hp = RolledEquipmentAffix::new(EquipmentAffixId::Vitality, 1, 15).unwrap();
        let grammar = EquipmentNameGrammar::FeminineSingular;
        assert_eq!(
            NamedEquipmentAffixes::new(grammar, &[power, hp]),
            NamedEquipmentAffixes::new(grammar, &[hp, power])
        );
        assert!(NamedEquipmentAffixes::new(grammar, &[power, power]).is_none());
        assert!(NamedEquipmentAffixes::new(grammar, &[]).is_none());
        assert!(NamedEquipmentAffixes::new(grammar, &[power; 4]).is_none());
        let valid = NamedEquipmentAffixes::new(grammar, &[power, hp]).unwrap();
        assert_eq!(
            bincode::deserialize::<NamedEquipmentAffixes>(&bincode::serialize(&valid).unwrap())
                .unwrap(),
            valid
        );
        let mut raw = serde_json::to_value(valid).unwrap();
        raw["rolls"][1] = raw["rolls"][0].clone();
        assert!(serde_json::from_value::<NamedEquipmentAffixes>(raw).is_err());
        for raw in [
            r#"{"id":"affix:power","tier":0,"value":1}"#,
            r#"{"id":"affix:power","tier":1,"value":255}"#,
            r#"{"id":"affix:unknown","tier":1,"value":1}"#,
        ] {
            assert!(serde_json::from_str::<RolledEquipmentAffix>(raw).is_err());
        }
    }
}
