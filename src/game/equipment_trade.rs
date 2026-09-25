//! Persistent, unrevealed property pools; no item is rolled by a shop view.
use super::*;
use crate::item::{EquipmentAffixId, EquipmentNameGrammar};
use crate::loot::{
    EquipmentBaseDefinition, EquipmentEffectChoice, EquipmentForm, EquipmentLootCatalog,
    EquipmentSource, EquipmentStatChoice,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct GambleProperties {
    tier: u8,
    grammar: EquipmentNameGrammar,
    stats: Vec<(EquipmentAffixId, u32)>,
    effects: Vec<(ContentId, u8, u32)>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(super) struct EquipmentTradeRules {
    properties: BTreeMap<ItemId, GambleProperties>,
    // Price paid to player, then price of the same item's resale listing.
    quotes: BTreeMap<ItemId, (u32, u32)>,
}

impl EquipmentTradeRules {
    pub(super) fn new(
        catalog: &EquipmentLootCatalog,
        gambles: &[crate::content::MerchantGambleDefinition],
        quotes: &[(ItemId, u32, u32)],
        rules: &GameRules,
    ) -> Result<Self, String> {
        let mut properties = BTreeMap::new();
        for gamble in gambles {
            let base = catalog
                .iter()
                .find(|(id, _)| *id == &gamble.item)
                .map(|(_, base)| base)
                .ok_or("Missing equipment gamble profile")?;
            if !base.sources.contains(&EquipmentSource::HumanoidSite) {
                return Err("Incompatible merchant equipment source".into());
            }
            if properties
                .insert(
                    gamble.item.clone(),
                    GambleProperties {
                        tier: base.tier,
                        grammar: base.grammar,
                        stats: base.stats.iter().map(|c| (c.affix, c.weight)).collect(),
                        effects: base
                            .effects
                            .iter()
                            .map(|c| (c.effect.clone(), c.minimum_tier, c.weight))
                            .collect(),
                    },
                )
                .is_some()
            {
                return Err("Duplicate equipment gamble".into());
            }
        }
        let mut prices = BTreeMap::new();
        for (id, sell, buy) in quotes {
            if prices.insert(id.clone(), (*sell, *buy)).is_some() {
                return Err("Duplicate equipment trade quote".into());
            }
        }
        let result = Self {
            properties,
            quotes: prices,
        };
        result.validate(rules)?;
        Ok(result)
    }

    fn catalog(&self, rules: &GameRules) -> Result<EquipmentLootCatalog, String> {
        let mut catalog = EquipmentLootCatalog::default();
        for (item, pool) in &self.properties {
            catalog.register(
                EquipmentBaseDefinition {
                    item: item.clone(),
                    family: "core:merchant_equipment".parse().unwrap(),
                    tier: pool.tier,
                    grammar: pool.grammar,
                    form: EquipmentForm::HumanoidEquipment,
                    sources: vec![EquipmentSource::HumanoidSite],
                    stats: pool
                        .stats
                        .iter()
                        .map(|&(affix, weight)| EquipmentStatChoice { affix, weight })
                        .collect(),
                    effects: pool
                        .effects
                        .iter()
                        .map(|(effect, minimum_tier, weight)| EquipmentEffectChoice {
                            effect: effect.clone(),
                            minimum_tier: *minimum_tier,
                            weight: *weight,
                        })
                        .collect(),
                },
                &rules.items,
                &rules.weapons,
            )?;
        }
        Ok(catalog)
    }

    pub(super) fn validate(&self, rules: &GameRules) -> Result<(), String> {
        if self.properties.is_empty() || self.properties.len() > 64 || self.quotes.len() > 1024 {
            return Err("Invalid equipment merchant size".into());
        }
        for (item, (sell, buy)) in &self.quotes {
            let equipment = rules.weapons.get(item).is_some()
                || rules
                    .items
                    .get(item)
                    .is_some_and(|definition| definition.kind() == crate::item::ItemKind::Armor);
            if !equipment || *sell == 0 || sell > buy {
                return Err("Invalid equipment purchase quote".into());
            }
        }
        self.catalog(rules)?;
        Ok(())
    }

    pub(super) fn admits(&self, item: &ItemId) -> bool {
        self.properties.contains_key(item)
    }

    pub(super) fn roll(
        &self,
        item: &ItemId,
        value_draws: u8,
        rules: &GameRules,
        rng: &mut GameRng,
    ) -> Result<MagicItemModifiers, String> {
        self.catalog(rules)?.enchant_base(item, value_draws, rng)
    }
}

impl MerchantState {
    pub(super) fn sale_quote(&self, item: &ItemId, rules: &GameRules) -> Option<(u32, u32, u16)> {
        if let Some((sell, buy)) = self
            .equipment
            .as_ref()
            .and_then(|equipment| equipment.quotes.get(item))
        {
            return Some((*sell, *buy, 1));
        }
        if let Some(offer) = self.offers.iter().find(|offer| &offer.item == item) {
            return Some((offer.sell_price, offer.buy_price, offer.maximum_stack));
        }
        self.gambles
            .iter()
            .find(|gamble| &gamble.item == item)
            .and_then(|gamble| {
                rules
                    .items
                    .get(item)
                    .map(|definition| (gamble.price / 2, gamble.price, definition.maximum_stack()))
            })
    }
}

impl WorldState {
    /// Explicit opt-in: legacy merchants retain their original armor rolls.
    #[allow(clippy::too_many_arguments)]
    pub fn register_equipment_merchant(
        &mut self,
        zone: ContentId,
        provider: crate::entity::EntityId,
        player_starting_credits: u32,
        definition: MerchantDefinition,
        gamble_seed: u64,
        catalog: &EquipmentLootCatalog,
        quotes: &[(ItemId, u32, u32)],
    ) -> Result<(), String> {
        let equipment =
            EquipmentTradeRules::new(catalog, &definition.gambles, quotes, self.rules())?;
        self.register_merchant_internal(
            zone,
            provider,
            player_starting_credits,
            definition,
            gamble_seed,
            Some(equipment),
        )
    }
}

#[cfg(test)]
#[path = "equipment_trade_tests.rs"]
mod tests;
