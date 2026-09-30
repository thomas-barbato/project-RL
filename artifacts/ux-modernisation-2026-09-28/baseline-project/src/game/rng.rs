/// Small deterministic generator owned by simulation state.
///
/// It uses SplitMix64, which is suitable for reproducible gameplay decisions
/// but is not a cryptographic generator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GameRng {
    state: u64,
}

impl GameRng {
    pub const fn from_seed(seed: u64) -> Self {
        Self { state: seed }
    }

    pub const fn state(self) -> u64 {
        self.state
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut value = self.state;
        value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        value ^ (value >> 31)
    }

    pub fn usize_inclusive(&mut self, minimum: usize, maximum: usize) -> Option<usize> {
        let span = maximum.checked_sub(minimum)?.checked_add(1)?;
        let span = u64::try_from(span).ok()?;
        let offset = self.next_u64() % span;
        minimum.checked_add(usize::try_from(offset).ok()?)
    }

    /// Draws an exactly uniform percentile in 1..=100. Rejection avoids the
    /// tiny modulo bias while remaining deterministic for replay.
    pub fn percentile(&mut self) -> u8 {
        const BOUND: u64 = 100;
        const THRESHOLD: u64 = BOUND.wrapping_neg() % BOUND;
        loop {
            let value = self.next_u64();
            if value >= THRESHOLD {
                return (value % BOUND + 1) as u8;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_seeds_produce_equal_sequences() {
        let mut first = GameRng::from_seed(42);
        let mut second = GameRng::from_seed(42);

        let first_values = [first.next_u64(), first.next_u64(), first.next_u64()];
        let second_values = [second.next_u64(), second.next_u64(), second.next_u64()];

        assert_eq!(first_values, second_values);
        assert_eq!(first.state(), second.state());
    }

    #[test]
    fn different_seeds_diverge() {
        let mut first = GameRng::from_seed(1);
        let mut second = GameRng::from_seed(2);

        assert_ne!(first.next_u64(), second.next_u64());
    }

    #[test]
    fn inclusive_range_honors_bounds_and_rejects_inverted_ranges() {
        let mut rng = GameRng::from_seed(99);

        for _ in 0..64 {
            let value = rng.usize_inclusive(3, 7);
            assert!(value.is_some_and(|sample| (3..=7).contains(&sample)));
        }
        assert_eq!(rng.usize_inclusive(8, 7), None);
    }

    #[test]
    fn percentile_is_deterministic_and_always_within_one_to_one_hundred() {
        let mut first = GameRng::from_seed(123);
        let mut second = GameRng::from_seed(123);

        for _ in 0..256 {
            let roll = first.percentile();
            assert!((1..=100).contains(&roll));
            assert_eq!(roll, second.percentile());
        }
    }
}
