/// Stable runtime identity. IDs, rather than references, are used by events and
/// will later be used by saves.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EntityId(u64);

impl EntityId {
    pub const fn get(self) -> u64 {
        self.0
    }

    pub(super) const fn new(value: u64) -> Self {
        Self(value)
    }
}
