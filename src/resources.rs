use std::error::Error;
use std::fmt::{Display, Formatter};

/// A finite reserve. Spending cannot create energy, and changing capacity
/// never fills it. Sources of recharge will explicitly transfer stored energy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

/// Capacity occupied by live links and procedures. Unlike energy, bandwidth
/// is reserved and released; a failed reservation never changes occupancy.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BandwidthReserve {
    capacity: u16,
    occupied: u16,
}

impl BandwidthReserve {
    pub const fn new(capacity: u16) -> Self {
        Self {
            capacity,
            occupied: 0,
        }
    }

    pub const fn capacity(self) -> u16 {
        self.capacity
    }

    pub const fn occupied(self) -> u16 {
        self.occupied
    }

    pub const fn available(self) -> u16 {
        self.capacity.saturating_sub(self.occupied)
    }

    pub fn reserve(&mut self, amount: u16) -> Result<(), BandwidthReservationError> {
        if amount > self.available() {
            return Err(BandwidthReservationError {
                required: amount,
                available: self.available(),
            });
        }
        self.occupied += amount;
        Ok(())
    }

    pub fn release(&mut self, amount: u16) {
        self.occupied = self.occupied.saturating_sub(amount);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BandwidthReservationError {
    pub required: u16,
    pub available: u16,
}

/// Accumulated heat has no artificial storage capacity. The authored alert
/// and critical thresholds are presentation and consequence boundaries, not
/// clamps; cooling can only reduce the current value.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct HeatReserve {
    current: u16,
    alert_threshold: u16,
    critical_threshold: u16,
    dissipation_per_phase: u16,
}

impl HeatReserve {
    pub fn new(
        alert_threshold: u16,
        critical_threshold: u16,
        dissipation_per_phase: u16,
    ) -> Result<Self, HeatRulesError> {
        if alert_threshold == 0 || critical_threshold == 0 || alert_threshold > critical_threshold {
            return Err(HeatRulesError::InvalidThresholds {
                alert: alert_threshold,
                critical: critical_threshold,
            });
        }
        Ok(Self {
            current: 0,
            alert_threshold,
            critical_threshold,
            dissipation_per_phase,
        })
    }

    pub const fn current(self) -> u16 {
        self.current
    }

    pub const fn alert_threshold(self) -> u16 {
        self.alert_threshold
    }

    pub const fn critical_threshold(self) -> u16 {
        self.critical_threshold
    }

    pub fn add(&mut self, amount: u16) {
        self.current = self.current.saturating_add(amount);
    }

    pub fn dissipate(&mut self) -> u16 {
        let before = self.current;
        self.current = self.current.saturating_sub(self.dissipation_per_phase);
        before - self.current
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatRulesError {
    InvalidThresholds { alert: u16, critical: u16 },
}

impl Display for HeatRulesError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidThresholds { alert, critical } => write!(
                f,
                "heat thresholds must be positive and ordered, found alert {alert}, critical {critical}"
            ),
        }
    }
}

impl Error for HeatRulesError {}

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

    #[test]
    fn bandwidth_is_reserved_and_released_without_becoming_a_consumable() {
        let mut reserve = BandwidthReserve::new(3);
        reserve.reserve(2).unwrap();
        let before = reserve;
        assert_eq!(
            reserve.reserve(2),
            Err(BandwidthReservationError {
                required: 2,
                available: 1
            })
        );
        assert_eq!(reserve, before);
        reserve.release(1);
        reserve.reserve(2).unwrap();
        assert_eq!(reserve.occupied(), 3);
        reserve.release(20);
        assert_eq!(reserve.occupied(), 0);
    }

    #[test]
    fn heat_can_cross_critical_and_dissipation_never_underflows() {
        let mut heat = HeatReserve::new(40, 80, 6).unwrap();
        heat.add(90);
        assert_eq!(heat.current(), 90);
        assert_eq!(heat.dissipate(), 6);
        for _ in 0..20 {
            heat.dissipate();
        }
        assert_eq!(heat.current(), 0);
        assert!(HeatReserve::new(90, 80, 1).is_err());
    }
}
