use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::content::{ContentId, RegionLootProfile};
use crate::game::GameRng;
use crate::item::ItemId;
use crate::loot::{LootCatalog, LootContext};
use crate::world::{GridPos, Map};

const LOOT_SEED_SALT: u64 = 0x5245_4749_4f4e_4c54;

pub struct RegionalLootRequest<'a> {
    pub map: &'a Map,
    pub passages: &'a [GridPos],
    pub reserved: &'a BTreeSet<GridPos>,
    pub cache_positions: &'a [GridPos],
    pub profile: &'a RegionLootProfile,
    pub catalog: &'a LootCatalog,
    pub biome: &'a ContentId,
    pub depth: u16,
    pub region_seed: u64,
}

pub fn generate_regional_loot(
    request: RegionalLootRequest<'_>,
) -> Result<Vec<(GridPos, ItemId, u16)>, RegionalLootError> {
    let table = request
        .catalog
        .get(request.profile.table())
        .ok_or_else(|| RegionalLootError::UnknownTable(request.profile.table().clone()))?;
    let mut rng = GameRng::from_seed(request.region_seed ^ LOOT_SEED_SALT);
    let draws = inclusive(
        &mut rng,
        request.profile.minimum_draws(),
        request.profile.maximum_draws(),
    );
    let drops = table
        .draw(
            &LootContext {
                depth: request.depth,
                map_kind: request.biome.clone(),
                source: request.profile.source().clone(),
            },
            draws,
            &mut rng,
        )
        .map_err(|error| RegionalLootError::Draw(error.to_string()))?;
    let mut positions: Vec<_> = request.cache_positions.to_vec();
    positions.extend(
        (1..request.map.height() as i32 - 1)
            .flat_map(|y| (1..request.map.width() as i32 - 1).map(move |x| GridPos::new(x, y)))
            .filter(|position| {
                request.map.is_walkable(*position)
                    && !request.map.is_protected(*position)
                    && !request.passages.contains(position)
                    && !request.reserved.contains(position)
                    && !request.cache_positions.contains(position)
            }),
    );
    if positions.len() < drops.len() {
        return Err(RegionalLootError::InsufficientSpace);
    }
    Ok(positions
        .into_iter()
        .zip(drops)
        .map(|(position, drop)| (position, drop.item, drop.quantity))
        .collect())
}

fn inclusive(rng: &mut GameRng, minimum: u16, maximum: u16) -> u16 {
    let bound = u64::from(maximum - minimum) + 1;
    let threshold = bound.wrapping_neg() % bound;
    loop {
        let value = rng.next_u64();
        if value >= threshold {
            return minimum + (value % bound) as u16;
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegionalLootError {
    UnknownTable(ContentId),
    Draw(String),
    InsufficientSpace,
}

impl Display for RegionalLootError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownTable(table) => write!(formatter, "unknown regional loot table '{table}'"),
            Self::Draw(error) => write!(formatter, "regional loot draw failed: {error}"),
            Self::InsufficientSpace => {
                write!(formatter, "regional loot has no available floor cell")
            }
        }
    }
}

impl Error for RegionalLootError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::loot::{LootEntry, LootTable};

    fn id(name: &str) -> ContentId {
        format!("test:{name}").parse().unwrap()
    }

    #[test]
    fn regional_loot_is_deterministic_and_fills_real_cache_positions_first() {
        let map = Map::from_ascii("########\n#......#\n#......#\n########").unwrap();
        let caches = [GridPos::new(3, 1), GridPos::new(4, 2)];
        let reserved = BTreeSet::from(caches);
        let mut catalog = LootCatalog::default();
        catalog
            .register(
                LootTable::new(
                    id("cache_table"),
                    vec![LootEntry {
                        item: id("repair"),
                        weight: 1,
                        minimum_depth: 0,
                        maximum_depth: None,
                        map_kinds: vec![id("wilds")],
                        sources: vec![id("cache")],
                        minimum_quantity: 1,
                        maximum_quantity: 1,
                    }],
                )
                .unwrap(),
            )
            .unwrap();
        let profile = RegionLootProfile::new(id("cache_table"), id("cache"), 2, 2).unwrap();
        let generate = || {
            generate_regional_loot(RegionalLootRequest {
                map: &map,
                passages: &[],
                reserved: &reserved,
                cache_positions: &caches,
                profile: &profile,
                catalog: &catalog,
                biome: &id("wilds"),
                depth: 0,
                region_seed: 44,
            })
            .unwrap()
        };

        let first = generate();
        assert_eq!(first, generate());
        assert_eq!(first.iter().map(|drop| drop.0).collect::<Vec<_>>(), caches);
    }
}
