//! Deterministic weighted draws of existing item definitions, independent of UI.
//! Power rolls/affixes are deliberately a later stage, not inferred from rarity.
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::content::ContentId;
use crate::game::GameRng;
use crate::item::{ItemCatalog, ItemId, ItemKind};
use crate::weapon::WeaponCatalog;

pub type LootTableId = ContentId;
pub const MAX_ENTRIES: usize = 1024;
pub const MAX_DRAWS: u16 = 64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LootContext {
    pub depth: u16,
    pub map_kind: ContentId,
    pub source: ContentId,
}

/// One ticket group. Matching rows add their weights, even for the same item.
/// Empty selectors match any kind/source; depth boundaries are inclusive.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LootEntry {
    pub item: ItemId,
    pub weight: u32,
    pub minimum_depth: u16,
    pub maximum_depth: Option<u16>,
    pub map_kinds: Vec<ContentId>,
    pub sources: Vec<ContentId>,
    pub minimum_quantity: u16,
    pub maximum_quantity: u16,
}

impl LootEntry {
    fn matches(&self, context: &LootContext) -> bool {
        context.depth >= self.minimum_depth
            && self.maximum_depth.is_none_or(|max| context.depth <= max)
            && (self.map_kinds.is_empty() || self.map_kinds.contains(&context.map_kind))
            && (self.sources.is_empty() || self.sources.contains(&context.source))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LootDrop {
    pub item: ItemId,
    pub quantity: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LootTable {
    id: LootTableId,
    entries: Vec<LootEntry>,
}

impl LootTable {
    pub fn new(id: LootTableId, entries: Vec<LootEntry>) -> Result<Self, LootError> {
        if entries.is_empty() || entries.len() > MAX_ENTRIES {
            return Err(LootError::InvalidEntryCount);
        }
        for entry in &entries {
            if entry
                .maximum_depth
                .is_some_and(|max| max < entry.minimum_depth)
                || entry.minimum_quantity == 0
                || entry.maximum_quantity < entry.minimum_quantity
            {
                return Err(LootError::InvalidRange(entry.item.clone()));
            }
            for selector in [&entry.map_kinds, &entry.sources] {
                if selector.len() > 64
                    || selector.iter().collect::<BTreeSet<_>>().len() != selector.len()
                {
                    return Err(LootError::InvalidSelector(entry.item.clone()));
                }
            }
        }
        if entries.iter().all(|entry| entry.weight == 0) {
            return Err(LootError::NoPositiveWeight);
        }
        Ok(Self { id, entries })
    }

    pub fn id(&self) -> &LootTableId {
        &self.id
    }
    pub fn entries(&self) -> &[LootEntry] {
        &self.entries
    }

    pub fn validate_items(
        &self,
        items: &ItemCatalog,
        weapons: &WeaponCatalog,
    ) -> Result<(), LootError> {
        for entry in &self.entries {
            let maximum = if weapons.get(&entry.item).is_some() {
                1
            } else {
                items
                    .get(&entry.item)
                    .ok_or_else(|| LootError::UnknownItem(entry.item.clone()))?
                    .maximum_stack()
            };
            if entry.maximum_quantity > maximum {
                return Err(LootError::QuantityExceedsStack(entry.item.clone()));
            }
        }
        Ok(())
    }

    /// No matching entry means no loot, never a fallback item from another tier.
    /// Rejections and empty draws preserve the RNG. Draws use replacement.
    pub fn draw(
        &self,
        context: &LootContext,
        count: u16,
        rng: &mut GameRng,
    ) -> Result<Vec<LootDrop>, LootError> {
        if count > MAX_DRAWS {
            return Err(LootError::TooManyDraws);
        }
        let entries: Vec<_> = self
            .entries
            .iter()
            .filter(|entry| entry.weight > 0 && entry.matches(context))
            .collect();
        // MAX_ENTRIES * u32::MAX fits in u64; never narrow or saturate weights.
        let total: u64 = entries.iter().map(|entry| u64::from(entry.weight)).sum();
        if total == 0 || count == 0 {
            return Ok(Vec::new());
        }
        let mut drops = Vec::with_capacity(usize::from(count));
        for _ in 0..count {
            let mut ticket = below(rng, total);
            let chosen = entries
                .iter()
                .find(|entry| {
                    if ticket < u64::from(entry.weight) {
                        true
                    } else {
                        ticket -= u64::from(entry.weight);
                        false
                    }
                })
                .expect("ticket is smaller than the total weight");
            let quantity = if chosen.minimum_quantity == chosen.maximum_quantity {
                chosen.minimum_quantity
            } else {
                let span = u64::from(chosen.maximum_quantity - chosen.minimum_quantity) + 1;
                chosen.minimum_quantity + below(rng, span) as u16
            };
            drops.push(LootDrop {
                item: chosen.item.clone(),
                quantity,
            });
        }
        Ok(drops)
    }
}

// Unbiased rejection sampling, without changing legacy GameRng range semantics
// (old replay suspensions must still reconstruct the same maps and actor turns).
fn below(rng: &mut GameRng, bound: u64) -> u64 {
    let threshold = bound.wrapping_neg() % bound;
    loop {
        let value = rng.next_u64();
        if value >= threshold {
            return value % bound;
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct LootCatalog {
    tables: BTreeMap<LootTableId, LootTable>,
}
impl LootCatalog {
    pub fn register(&mut self, table: LootTable) -> Result<(), LootError> {
        if self.tables.contains_key(table.id()) {
            return Err(LootError::DuplicateTable(table.id().clone()));
        }
        self.tables.insert(table.id().clone(), table);
        Ok(())
    }
    pub fn get(&self, id: &LootTableId) -> Option<&LootTable> {
        self.tables.get(id)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&LootTableId, &LootTable)> {
        self.tables.iter()
    }

    /// Removes a table introduced after an older run was created while
    /// leaving all historical table draws and their order untouched.
    pub fn without_table(&self, id: &LootTableId) -> Self {
        let mut compatible = self.clone();
        compatible.tables.remove(id);
        compatible
    }

    /// Compatibility projection used by versioned replay adapters when an
    /// item family did not exist yet. Empty, newly introduced tables vanish.
    pub fn without_item_kind(&self, items: &ItemCatalog, excluded: ItemKind) -> Self {
        let tables = self
            .tables
            .iter()
            .filter_map(|(id, table)| {
                let entries = table
                    .entries
                    .iter()
                    .filter(|entry| {
                        items
                            .get(&entry.item)
                            .is_none_or(|definition| definition.kind() != excluded)
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                (!entries.is_empty()).then(|| {
                    (
                        id.clone(),
                        LootTable {
                            id: id.clone(),
                            entries,
                        },
                    )
                })
            })
            .collect();
        Self { tables }
    }

    /// Compatibility projection for item definitions introduced after a
    /// recorded generation. Remaining rows keep their original order and
    /// weights, preserving the historical deterministic draw stream.
    pub fn without_items(&self, excluded: &[ItemId]) -> Self {
        let tables = self
            .tables
            .iter()
            .filter_map(|(id, table)| {
                let entries = table
                    .entries
                    .iter()
                    .filter(|entry| !excluded.contains(&entry.item))
                    .cloned()
                    .collect::<Vec<_>>();
                (!entries.is_empty()).then(|| {
                    (
                        id.clone(),
                        LootTable {
                            id: id.clone(),
                            entries,
                        },
                    )
                })
            })
            .collect();
        Self { tables }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LootError {
    InvalidEntryCount,
    NoPositiveWeight,
    InvalidRange(ItemId),
    InvalidSelector(ItemId),
    UnknownItem(ItemId),
    QuantityExceedsStack(ItemId),
    DuplicateTable(LootTableId),
    TooManyDraws,
}
impl Display for LootError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidEntryCount => write!(f, "loot table needs 1..={MAX_ENTRIES} entries"),
            Self::NoPositiveWeight => write!(f, "loot table needs at least one positive weight"),
            Self::InvalidRange(id) => write!(f, "invalid loot depth/quantity range for '{id}'"),
            Self::InvalidSelector(id) => {
                write!(f, "duplicate or oversized loot selector for '{id}'")
            }
            Self::UnknownItem(id) => write!(f, "unknown loot item '{id}'"),
            Self::QuantityExceedsStack(id) => {
                write!(f, "loot quantity exceeds stack limit for '{id}'")
            }
            Self::DuplicateTable(id) => write!(f, "duplicate loot table '{id}'"),
            Self::TooManyDraws => write!(f, "a loot request cannot exceed {MAX_DRAWS} draws"),
        }
    }
}
impl Error for LootError {}

#[cfg(test)]
mod tests {
    use super::*;
    fn id(name: &str) -> ContentId {
        format!("test:{name}").parse().unwrap()
    }
    fn context(depth: u16) -> LootContext {
        LootContext {
            depth,
            map_kind: id("industrial"),
            source: id("floor"),
        }
    }
    fn entry(name: &str, weight: u32) -> LootEntry {
        LootEntry {
            item: id(name),
            weight,
            minimum_depth: 0,
            maximum_depth: None,
            map_kinds: vec![],
            sources: vec![],
            minimum_quantity: 1,
            maximum_quantity: 1,
        }
    }
    fn table(entries: Vec<LootEntry>) -> LootTable {
        LootTable::new(id("table"), entries).unwrap()
    }

    #[test]
    fn equal_seed_and_context_produce_equal_items_quantities_and_rng() {
        let mut varying = entry("repair", 3);
        varying.maximum_quantity = 3;
        let table = table(vec![varying, entry("weapon", 1)]);
        for seed in 0..64 {
            let mut a = GameRng::from_seed(seed);
            let mut b = a;
            assert_eq!(
                table.draw(&context(1), 64, &mut a),
                table.draw(&context(1), 64, &mut b)
            );
            assert_eq!(a, b);
            let drops = table.draw(&context(1), 64, &mut a).unwrap();
            assert!(drops.iter().all(|drop| (1..=3).contains(&drop.quantity)));
        }
    }

    #[test]
    fn depth_map_source_and_zero_weight_filter_before_drawing() {
        let mut selected = entry("selected", u32::MAX);
        selected.minimum_depth = 2;
        selected.maximum_depth = Some(5);
        selected.map_kinds = vec![id("industrial")];
        selected.sources = vec![id("floor")];
        let table = table(vec![entry("disabled", 0), selected]);
        for depth in [2, 5] {
            let mut rng = GameRng::from_seed(1);
            assert!(
                table
                    .draw(&context(depth), 64, &mut rng)
                    .unwrap()
                    .iter()
                    .all(|d| d.item == id("selected"))
            );
        }
        let mut invalid = vec![context(1), context(6)];
        let mut other_map = context(2);
        other_map.map_kind = id("city");
        invalid.push(other_map);
        let mut other_source = context(2);
        other_source.source = id("chest");
        invalid.push(other_source);
        for context in invalid {
            let mut rng = GameRng::from_seed(42);
            let before = rng;
            assert!(table.draw(&context, 4, &mut rng).unwrap().is_empty());
            assert_eq!(rng, before);
        }
    }

    #[test]
    fn relative_weights_and_rare_shallow_exceptions_are_statistically_observable() {
        let mut shallow_common = entry("common", 999);
        shallow_common.maximum_depth = Some(2);
        let mut shallow_exception = entry("exception", 1);
        shallow_exception.maximum_depth = Some(2);
        let mut deep_common = entry("common", 700);
        deep_common.minimum_depth = 3;
        let mut deep_exception = entry("exception", 300);
        deep_exception.minimum_depth = 3;
        let table = table(vec![
            shallow_common,
            shallow_exception,
            deep_common,
            deep_exception,
        ]);
        let count = |depth| {
            let mut rng = GameRng::from_seed(77);
            (0..1000)
                .flat_map(|_| table.draw(&context(depth), 64, &mut rng).unwrap())
                .filter(|drop| drop.item == id("exception"))
                .count()
        };
        let shallow = count(0);
        let deep = count(10);
        assert!((30..100).contains(&shallow), "{shallow}/64000");
        assert!((18000..20500).contains(&deep), "{deep}/64000");
    }

    #[test]
    fn rejected_requests_zero_draws_and_duplicate_registration_are_atomic() {
        let table = table(vec![entry("a", u32::MAX), entry("b", u32::MAX)]);
        let mut rng = GameRng::from_seed(2);
        let before = rng;
        assert_eq!(
            table.draw(&context(0), MAX_DRAWS + 1, &mut rng),
            Err(LootError::TooManyDraws)
        );
        assert!(table.draw(&context(0), 0, &mut rng).unwrap().is_empty());
        assert_eq!(rng, before);
        assert_eq!(table.draw(&context(0), 64, &mut rng).unwrap().len(), 64);
        let mut catalog = LootCatalog::default();
        catalog.register(table.clone()).unwrap();
        let before = catalog.clone();
        assert!(catalog.register(table).is_err());
        assert_eq!(catalog, before);
    }

    #[test]
    fn malformed_ranges_selectors_and_empty_tables_are_rejected() {
        assert_eq!(
            LootTable::new(id("empty"), vec![]),
            Err(LootError::InvalidEntryCount)
        );
        assert!(LootTable::new(id("large"), vec![entry("a", 1); MAX_ENTRIES + 1]).is_err());
        assert_eq!(
            LootTable::new(id("disabled"), vec![entry("a", 0)]),
            Err(LootError::NoPositiveWeight)
        );
        let mut bad = entry("a", 1);
        bad.minimum_depth = 5;
        bad.maximum_depth = Some(4);
        assert!(LootTable::new(id("bad"), vec![bad]).is_err());
        let mut bad = entry("a", 1);
        bad.minimum_quantity = 0;
        assert!(LootTable::new(id("bad"), vec![bad]).is_err());
        let mut bad = entry("a", 1);
        bad.map_kinds = vec![id("city"), id("city")];
        assert!(LootTable::new(id("bad"), vec![bad]).is_err());
    }

    #[test]
    fn references_and_maximum_stack_are_validated_including_disabled_entries() {
        let loaded = crate::content::ContentLoader::load(
            &[std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content")],
            &semver::Version::new(0, 1, 0),
        )
        .unwrap();
        assert!(matches!(
            table(vec![entry("missing", 1)]).validate_items(loaded.items(), loaded.weapons()),
            Err(LootError::UnknownItem(_))
        ));
        let mut valid = entry("valid", 1);
        valid.item = "core:repair_patch".parse().unwrap();
        assert!(matches!(
            table(vec![valid, entry("disabled_missing", 0)])
                .validate_items(loaded.items(), loaded.weapons()),
            Err(LootError::UnknownItem(_))
        ));
        for (name, quantity) in [("core:needle_launcher", 2), ("core:repair_patch", 4)] {
            let mut bad = entry("bad", 1);
            bad.item = name.parse().unwrap();
            bad.maximum_quantity = quantity;
            assert!(matches!(
                table(vec![bad]).validate_items(loaded.items(), loaded.weapons()),
                Err(LootError::QuantityExceedsStack(_))
            ));
        }
    }
}
