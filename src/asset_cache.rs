//! Small byte-budgeted LRU, independent of graphics contexts.
use std::collections::BTreeMap;

pub struct ByteCache<K, V> {
    entries: BTreeMap<K, (V, usize, u64)>,
    bytes: usize,
    budget: usize,
    tick: u64,
}

impl<K: Ord + Clone, V> ByteCache<K, V> {
    pub const fn new(budget: usize) -> Self {
        Self {
            entries: BTreeMap::new(),
            bytes: 0,
            budget,
            tick: 0,
        }
    }
    pub fn get(&mut self, key: &K) -> Option<&V> {
        self.tick += 1;
        self.entries.get_mut(key).map(|(value, _, used)| {
            *used = self.tick;
            &*value
        })
    }
    pub fn bytes_used(&self) -> usize {
        self.bytes
    }
    pub fn insert(&mut self, key: K, value: V, bytes: usize) {
        if let Some((_, old, _)) = self.entries.remove(&key) {
            self.bytes -= old;
        }
        // One oversized value may live alone so callers can still draw it.
        while !self.entries.is_empty() && self.bytes.saturating_add(bytes) > self.budget {
            let oldest = self
                .entries
                .iter()
                .min_by_key(|(_, (_, _, used))| used)
                .unwrap()
                .0
                .clone();
            let (_, size, _) = self.entries.remove(&oldest).unwrap();
            self.bytes -= size;
        }
        self.tick += 1;
        self.entries.insert(key, (value, bytes, self.tick));
        self.bytes += bytes;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn recently_used_assets_survive_and_replacement_releases_its_budget() {
        let mut cache = ByteCache::new(10);
        cache.insert(1, "one", 4);
        cache.insert(2, "two", 4);
        assert_eq!(cache.get(&1), Some(&"one"));
        cache.insert(3, "three", 4);
        assert_eq!(cache.get(&2), None);
        assert_eq!(cache.get(&1), Some(&"one"));
        cache.insert(1, "replacement", 1);
        cache.insert(4, "four", 5);
        assert_eq!(cache.bytes, 10);
        assert_eq!(cache.get(&1), Some(&"replacement"));
        assert_eq!(cache.get(&3), Some(&"three"));
    }
    #[test]
    fn oversized_asset_is_kept_alone_then_evicted_normally() {
        let mut cache = ByteCache::new(10);
        cache.insert(1, (), 3);
        cache.insert(2, (), 11);
        assert_eq!(cache.bytes, 11);
        assert_eq!(cache.entries.len(), 1);
        cache.insert(3, (), 2);
        assert_eq!(cache.bytes, 2);
        assert!(cache.get(&2).is_none());
    }
}
