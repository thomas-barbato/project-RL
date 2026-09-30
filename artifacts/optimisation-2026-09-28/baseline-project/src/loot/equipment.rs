//! One definition per base; weighted properties belong to each rolled instance.
use crate::content::ContentId;
use crate::entity::MagicItemModifiers;
use crate::game::GameRng;
use crate::item::{
    EquipmentAffixId, EquipmentNameGrammar, ItemCatalog, ItemId, ItemKind, NamedEquipmentAffixes,
    RolledEquipmentAffix,
};
use crate::weapon::WeaponCatalog;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EquipmentSource {
    HumanoidEquipped,
    HumanoidSite,
    Robot,
    MechanicalSite,
    OrganicCreature,
    AnomalousBeing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EquipmentForm {
    HumanoidEquipment,
    MachineModule,
}

impl EquipmentForm {
    fn admits(self, source: EquipmentSource) -> bool {
        matches!(
            (self, source),
            (
                Self::HumanoidEquipment,
                EquipmentSource::HumanoidEquipped | EquipmentSource::HumanoidSite
            ) | (
                Self::MachineModule,
                EquipmentSource::Robot | EquipmentSource::MechanicalSite
            )
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EquipmentStatChoice {
    pub affix: EquipmentAffixId,
    #[serde(deserialize_with = "deserialize_weight")]
    pub weight: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EquipmentEffectChoice {
    #[serde(deserialize_with = "deserialize_id")]
    pub effect: ContentId,
    #[serde(deserialize_with = "deserialize_tier")]
    pub minimum_tier: u8,
    #[serde(deserialize_with = "deserialize_weight")]
    pub weight: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EquipmentBaseDefinition {
    #[serde(deserialize_with = "deserialize_id")]
    pub item: ItemId,
    #[serde(deserialize_with = "deserialize_id")]
    pub family: ContentId,
    #[serde(deserialize_with = "deserialize_tier")]
    pub tier: u8,
    pub grammar: EquipmentNameGrammar,
    pub form: EquipmentForm,
    pub sources: Vec<EquipmentSource>,
    #[serde(default)]
    pub stats: Vec<EquipmentStatChoice>,
    #[serde(default)]
    pub effects: Vec<EquipmentEffectChoice>,
}

fn deserialize_id<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<ContentId, D::Error> {
    String::deserialize(deserializer)?
        .parse()
        .map_err(serde::de::Error::custom)
}

// JSON5's narrow integer readers coerce negatives/fractions. Validate the
// original value before narrowing; never silently alter a draw weight.
fn deserialize_weight<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
    serde_json::Value::deserialize(deserializer)?
        .as_u64()
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| serde::de::Error::custom("weight must be an integer in 0..=u32::MAX"))
}

fn deserialize_tier<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<u8, D::Error> {
    let value = deserialize_weight(deserializer)?;
    if !(1..=6).contains(&value) {
        return Err(serde::de::Error::custom("equipment tier must be in 1..=6"));
    }
    Ok(value as u8)
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct EquipmentLootCatalog {
    bases: BTreeMap<ItemId, EquipmentBaseDefinition>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EquipmentQuality {
    White,
    Enchanted,
    Random { enchanted_percent: u8 },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedEquipment {
    pub item: ItemId,
    pub tier: u8,
    pub modifiers: Option<MagicItemModifiers>,
}

// Prototype catalogue weights, not player levels. Missing tiers are excluded.
const DEPTH_WEIGHTS: [[u16; 6]; 6] = [
    [7000, 2200, 600, 160, 35, 5],
    [3000, 4000, 2000, 750, 200, 50],
    [1200, 2600, 3800, 1700, 550, 150],
    [500, 1300, 2300, 3600, 1700, 600],
    [200, 600, 1400, 2600, 3400, 1800],
    [100, 250, 650, 1700, 3300, 4000],
];

impl EquipmentLootCatalog {
    pub fn is_empty(&self) -> bool {
        self.bases.is_empty()
    }
    pub fn iter(&self) -> impl Iterator<Item = (&ItemId, &EquipmentBaseDefinition)> {
        self.bases.iter()
    }

    /// Roll properties on the advertised model, without drawing a new base.
    /// Better progression improves values only; property weights stay intact.
    pub fn enchant_base(
        &self,
        item: &ItemId,
        value_draws: u8,
        rng: &mut GameRng,
    ) -> Result<MagicItemModifiers, String> {
        if !(1..=4).contains(&value_draws) {
            return Err("Equipment value draws must be in 1..=4".into());
        }
        let base = self.bases.get(item).ok_or("Unknown equipment base")?;
        Ok(roll_properties(base, value_draws, rng))
    }

    pub fn register(
        &mut self,
        base: EquipmentBaseDefinition,
        items: &ItemCatalog,
        weapons: &WeaponCatalog,
    ) -> Result<(), String> {
        if self.bases.len() >= 1024 || self.bases.contains_key(&base.item) {
            return Err(format!(
                "Duplicate equipment base or catalogue limit: {}",
                base.item
            ));
        }
        if !(1..=6).contains(&base.tier)
            || base.sources.is_empty()
            || base.sources.iter().collect::<BTreeSet<_>>().len() != base.sources.len()
            || base.sources.iter().any(|source| !base.form.admits(*source))
        {
            return Err(format!("Invalid equipment tier or source: {}", base.item));
        }
        let weapon = weapons.get(&base.item);
        if weapon.is_none()
            && !items
                .get(&base.item)
                .is_some_and(|item| item.kind() == ItemKind::Armor)
        {
            return Err(format!("Unknown equipment base: {}", base.item));
        }
        if base.stats.len() > EquipmentAffixId::ALL.len()
            || base
                .stats
                .iter()
                .map(|choice| choice.affix)
                .collect::<BTreeSet<_>>()
                .len()
                != base.stats.len()
            || base.effects.len() > 64
            || base
                .effects
                .iter()
                .map(|choice| &choice.effect)
                .collect::<BTreeSet<_>>()
                .len()
                != base.effects.len()
        {
            return Err(format!(
                "Duplicate or excessive equipment properties: {}",
                base.item
            ));
        }
        for choice in &base.effects {
            if !(1..=6).contains(&choice.minimum_tier)
                || weapon.is_none()
                || weapons
                    .resolve_instance(&base.item, Some(&choice.effect))
                    .is_none()
            {
                return Err(format!(
                    "Unknown or incompatible effect {} on {}",
                    choice.effect, base.item
                ));
            }
        }
        if !base.stats.iter().any(|choice| choice.weight > 0)
            && !base
                .effects
                .iter()
                .any(|choice| choice.weight > 0 && base.tier >= choice.minimum_tier)
        {
            return Err(format!("No eligible weighted property: {}", base.item));
        }
        self.bases.insert(base.item.clone(), base);
        Ok(())
    }

    pub fn without_items(&self, excluded: &[ItemId]) -> Self {
        Self {
            bases: self
                .bases
                .iter()
                .filter(|(id, _)| !excluded.contains(id))
                .map(|(id, base)| (id.clone(), base.clone()))
                .collect(),
        }
    }

    /// Validation precedes RNG consumption. Family-first selection prevents
    /// adding many models from silently increasing a family's frequency.
    pub fn draw(
        &self,
        source: EquipmentSource,
        depth: u16,
        quality: EquipmentQuality,
        rng: &mut GameRng,
    ) -> Result<Option<GeneratedEquipment>, String> {
        if matches!(quality, EquipmentQuality::Random { enchanted_percent } if enchanted_percent > 100)
        {
            return Err("Enchanted equipment probability exceeds 100 percent".into());
        }
        let eligible: Vec<_> = self
            .bases
            .values()
            .filter(|base| base.sources.contains(&source))
            .collect();
        if eligible.is_empty() {
            return Ok(None);
        }
        let families: Vec<_> = eligible
            .iter()
            .map(|base| &base.family)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let family = families[super::below(rng, families.len() as u64) as usize];
        let family_bases: Vec<_> = eligible
            .into_iter()
            .filter(|base| &base.family == family)
            .collect();
        let tiers: Vec<_> = family_bases
            .iter()
            .map(|base| base.tier)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        let weights = DEPTH_WEIGHTS[usize::from(depth).min(DEPTH_WEIGHTS.len() - 1)];
        let tier = tiers[weighted_index(
            tiers
                .iter()
                .map(|tier| u32::from(weights[usize::from(*tier - 1)])),
            rng,
        )];
        let candidates: Vec<_> = family_bases
            .into_iter()
            .filter(|base| base.tier == tier)
            .collect();
        let base = candidates[super::below(rng, candidates.len() as u64) as usize];
        let enchanted = match quality {
            EquipmentQuality::White => false,
            EquipmentQuality::Enchanted => true,
            EquipmentQuality::Random { enchanted_percent } => rng.percentile() <= enchanted_percent,
        };
        let modifiers = enchanted.then(|| roll_properties(base, 1, rng));
        Ok(Some(GeneratedEquipment {
            item: base.item.clone(),
            tier,
            modifiers,
        }))
    }
}

fn weighted_index(weights: impl Iterator<Item = u32> + Clone, rng: &mut GameRng) -> usize {
    let total: u64 = weights.clone().map(u64::from).sum();
    let mut ticket = super::below(rng, total);
    weights
        .enumerate()
        .find_map(|(index, weight)| {
            if ticket < u64::from(weight) {
                Some(index)
            } else {
                ticket -= u64::from(weight);
                None
            }
        })
        .expect("positive bounded weights")
}

fn roll_properties(
    base: &EquipmentBaseDefinition,
    value_draws: u8,
    rng: &mut GameRng,
) -> MagicItemModifiers {
    let mut stats: Vec<_> = base
        .stats
        .iter()
        .filter(|choice| choice.weight > 0)
        .collect();
    let mut effects: Vec<_> = base
        .effects
        .iter()
        .filter(|choice| choice.weight > 0 && base.tier >= choice.minimum_tier)
        .collect();
    let maximum = (stats.len() + usize::from(!effects.is_empty())).min(3);
    let count = 1 + super::below(rng, maximum as u64) as usize;
    let mut rolls = Vec::new();
    let mut effect = None;
    for _ in 0..count {
        let index = weighted_index(
            stats
                .iter()
                .map(|choice| choice.weight)
                .chain(effects.iter().map(|choice| choice.weight)),
            rng,
        );
        if index < stats.len() {
            let id = stats.remove(index).affix;
            let (minimum, maximum) = id.range(base.tier).expect("validated tier");
            let value = (0..value_draws)
                .map(|_| minimum + super::below(rng, u64::from(maximum - minimum) + 1) as u16)
                .max()
                .expect("validated number of draws");
            rolls.push(RolledEquipmentAffix::new(id, base.tier, value).expect("bounded value"));
        } else {
            effect = Some(effects[index - stats.len()].effect.clone());
            effects.clear(); // At most one special effect, counted among 1..=3 properties.
        }
    }
    let mut modifiers = if rolls.is_empty() {
        MagicItemModifiers::effect_only(effect.take().expect("an enchanted item has a property"))
    } else {
        MagicItemModifiers::from_affixes(
            NamedEquipmentAffixes::new(base.grammar, &rolls).expect("distinct stats"),
        )
    };
    if let Some(effect) = effect {
        modifiers = modifiers.with_effect_affix(effect);
    }
    modifiers
}

#[cfg(test)]
#[path = "equipment_tests.rs"]
mod tests;
