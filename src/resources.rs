use std::error::Error;
use std::fmt::{Display, Formatter};

/// A finite reserve. Spending cannot create energy, and changing capacity
/// never fills it. Sources of recharge will explicitly transfer stored energy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnergyReserve {
    capacity: u16,
    available: u16,
}

impl EnergyReserve {
    pub fn new(capacity: u16, available: u16) -> Result<Self, EnergyReserveError> {
        if available > capacity {
            return Err(EnergyReserveError {
                capacity,
                available,
            });
        }
        Ok(Self {
            capacity,
            available,
        })
    }

    pub const fn capacity(self) -> u16 {
        self.capacity
    }

    pub const fn available(self) -> u16 {
        self.available
    }

    pub fn set_capacity(&mut self, capacity: u16) {
        self.capacity = capacity;
        self.available = self.available.min(capacity);
    }

    pub fn spend(&mut self, amount: u16) -> Result<(), EnergySpendError> {
        if amount > self.available {
            return Err(EnergySpendError {
                required: amount,
                available: self.available,
            });
        }
        self.available -= amount;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnergyReserveError {
    pub capacity: u16,
    pub available: u16,
}

impl Display for EnergyReserveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "starting energy {} exceeds capacity {}",
            self.available, self.capacity
        )
    }
}

impl Error for EnergyReserveError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EnergySpendError {
    pub required: u16,
    pub available: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capacity_changes_and_failed_spends_cannot_refill_the_reserve() {
        let mut reserve = EnergyReserve::new(100, 40).unwrap();
        reserve.set_capacity(200);
        assert_eq!(reserve.available(), 40);
        reserve.set_capacity(20);
        reserve.set_capacity(200);
        assert_eq!(reserve.available(), 20);
        let before = reserve;
        assert!(reserve.spend(21).is_err());
        assert_eq!(reserve, before);
        reserve.spend(20).unwrap();
        reserve.spend(0).unwrap();
        assert_eq!(reserve.available(), 0);
        assert!(EnergyReserve::new(10, 11).is_err());
    }
}
