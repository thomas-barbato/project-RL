/// High-level behavior shared by the companion command surface.
///
/// Drones are the first supported allies, but the player command and client
/// deliberately use companion terminology so later biological or mechanical
/// allies can adopt the same four readable behaviors.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum CompanionBehavior {
    /// Stay close and assist the target explicitly attacked by the player.
    Follow,
    /// Stay close and engage only immediate threats around the group.
    Defensive,
    /// Engage and pursue the closest locally perceived hostile within a leash.
    Aggressive,
    /// Stay close without initiating attacks.
    Passive,
}

impl CompanionBehavior {
    pub const ALL: [Self; 4] = [
        Self::Follow,
        Self::Defensive,
        Self::Aggressive,
        Self::Passive,
    ];
}
