//! First authored equipment shops; prices are provisional, not rarity weights.
use super::*;
use project_rl::content::{MerchantDefinition, MerchantGambleDefinition, MerchantOfferDefinition};
use project_rl::loot::{EquipmentLootCatalog, EquipmentSource};

type EquipmentQuotes = Vec<(ItemId, u32, u32)>;

fn base_price(family: &str, tier: u8) -> Option<u32> {
    let prices = match family {
        "core:knives" => [50, 85, 130, 190, 280, 410],
        "core:rifles" => [85, 130, 200, 300, 440, 640],
        "core:protective_jackets" => [70, 105, 160, 240, 350, 510],
        _ => return None,
    };
    prices.get(usize::from(tier.checked_sub(1)?)).copied()
}

pub(super) fn shop_definition(
    mut definition: MerchantDefinition,
    catalog: &EquipmentLootCatalog,
    depth: u16,
) -> Result<(MerchantDefinition, EquipmentQuotes), String> {
    let tier = depth.saturating_add(1).min(6) as u8;
    // Keep ordinary supplies and their existing prices. The two old prototype
    // body armors are replaced; no rare affix is rolled into the white stock.
    definition.offers.retain(|offer| {
        !matches!(
            offer.item.as_str(),
            "core:patched_plating" | "core:composite_carapace"
        )
    });
    let mut gambles = Vec::new();
    let mut quotes = Vec::new();
    for (id, base) in catalog.iter() {
        if !base.sources.contains(&EquipmentSource::HumanoidSite) {
            continue;
        }
        let Some(price) = base_price(base.family.as_str(), base.tier) else {
            continue;
        };
        quotes.push((id.clone(), price / 3, price));
        if base.tier == tier || (tier > 1 && base.tier == tier - 1) {
            definition.offers.push(MerchantOfferDefinition {
                item: id.clone(),
                initial_stock: if base.tier == tier { 2 } else { 1 },
                buy_price: price,
                sell_price: price / 3,
                minimum_depth: 0,
                maximum_depth: None,
            });
        }
        if base.tier == tier {
            gambles.push(MerchantGambleDefinition {
                item: id.clone(),
                initial_stock: 2,
                price: price * 2,
            });
        }
    }
    let definition = MerchantDefinition::new(
        definition.position,
        definition.maximum_integrity,
        definition.initial_credits,
        definition.offers,
        gambles,
        definition.gamble_scaling,
    )
    .map_err(|error| error.to_string())?;
    Ok((definition, quotes))
}

impl AsciiApp {
    pub(super) fn register_local_equipment_merchant(
        &mut self,
        zone: ContentId,
        provider: EntityId,
        starting_credits: u32,
        definition: MerchantDefinition,
        seed: u64,
    ) -> Result<(), String> {
        if self.generation_version < EQUIPMENT_COMMERCE_GENERATION_VERSION {
            return self
                .game
                .register_merchant(zone, provider, starting_credits, definition, seed);
        }
        let depth = self
            .game
            .zone_info(&zone)
            .ok_or("Merchant zone absent")?
            .depth;
        let (definition, quotes) = shop_definition(definition, self.loot.equipment(), depth)?;
        self.game.register_equipment_merchant(
            zone,
            provider,
            starting_credits,
            definition,
            seed,
            self.loot.equipment(),
            &quotes,
        )
    }
}

#[cfg(test)]
#[path = "equipment_commerce_tests.rs"]
mod tests;
