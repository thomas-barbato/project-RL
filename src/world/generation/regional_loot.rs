use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::content::{ContentId, RegionLootProfile};
use crate::game::GameRng;
use crate::item::ItemId;
use crate::loot::{LootCatalog, LootContext};
use crate::world::{GridPos, Map};

const LOOT_SEED_SALT: u64 = 0x5245_4749_4f4e_4c54;
const SCATTER_SEED_SALT: u64 = 0x5343_4154_5445_5252;
const SALVAGE_SEED_SALT: u64 = 0x5341_4c56_4147_4552;

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
    generate_regional_loot_with_scatter(request, false)
}

pub fn generate_regional_loot_with_scatter(
    request: RegionalLootRequest<'_>,
    scatter: bool,
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
    let mut loose_positions: Vec<_> = (1..request.map.height() as i32 - 1)
        .flat_map(|y| (1..request.map.width() as i32 - 1).map(move |x| GridPos::new(x, y)))
        .filter(|position| {
            request.map.is_walkable(*position)
                && !request.map.is_protected(*position)
                && !request.passages.contains(position)
                && !request.reserved.contains(position)
                && !request.cache_positions.contains(position)
                && (!scatter
                    || (request.passages.iter().all(|passage| {
                        position.x.abs_diff(passage.x) + position.y.abs_diff(passage.y) >= 8
                    }) && request.cache_positions.iter().all(|cache| {
                        position.x.abs_diff(cache.x) + position.y.abs_diff(cache.y) >= 8
                    })))
        })
        .collect();
    if positions.len() + loose_positions.len() < drops.len() {
        return Err(RegionalLootError::InsufficientSpace);
    }
    if scatter {
        let mut placement_rng = GameRng::from_seed(request.region_seed ^ SCATTER_SEED_SALT);
        let extra_count = drops.len().saturating_sub(positions.len());
        for _ in 0..extra_count {
            let index = below(&mut placement_rng, loose_positions.len() as u64) as usize;
            positions.push(loose_positions.swap_remove(index));
        }
    } else {
        positions.extend(loose_positions);
    }
    Ok(positions
        .into_iter()
        .zip(drops)
        .map(|(position, drop)| (position, drop.item, drop.quantity))
        .collect())
}

/// Replaces the ordinary draw at a volatile optional cache. A separate stream
/// preserves every ordinary cache and loose-salvage roll.
pub fn generate_regional_salvage_reward(
    profile: &RegionLootProfile,
    catalog: &LootCatalog,
    biome: &ContentId,
    depth: u16,
    region_seed: u64,
) -> Result<(ItemId, u16), RegionalLootError> {
    let table = catalog
        .get(profile.table())
        .ok_or_else(|| RegionalLootError::UnknownTable(profile.table().clone()))?;
    let mut rng = GameRng::from_seed(region_seed ^ SALVAGE_SEED_SALT);
    let mut drops = table
        .draw(
            &LootContext {
                depth,
                map_kind: biome.clone(),
                source: profile.source().clone(),
            },
            1,
            &mut rng,
        )
        .map_err(|error| RegionalLootError::Draw(error.to_string()))?;
    let drop = drops.pop().expect("one salvage draw returns one item");
    Ok((drop.item, drop.quantity))
}

fn below(rng: &mut GameRng, bound: u64) -> u64 {
    let threshold = bound.wrapping_neg() % bound;
    loop {
        let value = rng.next_u64();
        if value >= threshold {
            return value % bound;
        }
    }
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

    #[test]
    fn scattered_salvage_is_seeded_and_stays_away_from_cache_and_passage() {
        let rows = std::iter::once("#".repeat(40))
            .chain(std::iter::repeat_n(format!("#{}#", ".".repeat(38)), 28))
            .chain(std::iter::once("#".repeat(40)))
            .collect::<Vec<_>>();
        let map = Map::from_ascii(&rows.join("\n")).unwrap();
        let caches = [GridPos::new(10, 10), GridPos::new(28, 20)];
        let passages = [GridPos::new(1, 15)];
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
        let profile = RegionLootProfile::new(id("cache_table"), id("cache"), 4, 4).unwrap();
        let generate = |seed| {
            generate_regional_loot_with_scatter(
                RegionalLootRequest {
                    map: &map,
                    passages: &passages,
                    reserved: &BTreeSet::from(caches),
                    cache_positions: &caches,
                    profile: &profile,
                    catalog: &catalog,
                    biome: &id("wilds"),
                    depth: 0,
                    region_seed: seed,
                },
                true,
            )
            .unwrap()
        };
        let first = generate(44);
        assert_eq!(first, generate(44));
        assert_ne!(first[2].0, generate(45)[2].0);
        assert_eq!(
            first[..2].iter().map(|drop| drop.0).collect::<Vec<_>>(),
            caches
        );
        assert!(first[2..].iter().all(|drop| {
            passages
                .iter()
                .chain(caches.iter())
                .all(|anchor| drop.0.x.abs_diff(anchor.x) + drop.0.y.abs_diff(anchor.y) >= 8)
        }));
    }
}
