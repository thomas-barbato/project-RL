use super::*;

#[test]
fn shops_cover_every_layer_with_white_stock_and_current_tier_gambles() {
    let (_, _, loot, expeditions) = ascii_game_content().unwrap();
    let original = expeditions
        .get(&"core:starter_expedition".parse().unwrap())
        .unwrap()
        .hub_merchant
        .clone()
        .unwrap();
    for depth in 0..=7 {
        let (shop, quotes) = shop_definition(original.clone(), loot.equipment(), depth).unwrap();
        let tier = depth.saturating_add(1).min(6) as u8;
        assert_eq!(quotes.len(), 18);
        let equipment: Vec<_> = shop
            .offers
            .iter()
            .filter_map(|offer| {
                loot.equipment()
                    .iter()
                    .find(|(id, _)| *id == &offer.item)
                    .map(|(_, base)| base)
            })
            .collect();
        assert_eq!(equipment.len(), if tier == 1 { 3 } else { 6 });
        assert!(
            equipment
                .iter()
                .all(|base| base.tier == tier || base.tier + 1 == tier)
        );
        assert_eq!(shop.gambles.len(), 3);
        for gamble in &shop.gambles {
            let base = loot
                .equipment()
                .iter()
                .find(|(id, _)| *id == &gamble.item)
                .unwrap()
                .1;
            assert_eq!(base.tier, tier);
            let offer = shop
                .offers
                .iter()
                .find(|offer| offer.item == gamble.item)
                .unwrap();
            assert_eq!(gamble.price, offer.buy_price * 2);
            assert!(offer.sell_price < offer.buy_price && offer.buy_price < gamble.price);
        }
        for old in original
            .offers
            .iter()
            .filter(|offer| offer.item.as_str() == "core:repair_patch")
        {
            assert!(shop.offers.contains(old));
        }
    }
}
