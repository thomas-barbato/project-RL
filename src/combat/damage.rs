#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DamageType {
    Kinetic,
    Piercing,
    Explosive,
    Thermal,
    Electrical,
    Chemical,
    Radiation,
    Corruption,
}

impl DamageType {
    const COUNT: usize = 8;

    const fn index(self) -> usize {
        match self {
            Self::Kinetic => 0,
            Self::Piercing => 1,
            Self::Explosive => 2,
            Self::Thermal => 3,
            Self::Electrical => 4,
            Self::Chemical => 5,
            Self::Radiation => 6,
            Self::Corruption => 7,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DamagePacket {
    pub amount: u16,
    pub damage_type: DamageType,
    pub penetration: u16,
}

impl DamagePacket {
    pub const fn new(amount: u16, damage_type: DamageType, penetration: u16) -> Self {
        Self {
            amount,
            damage_type,
            penetration,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DamageRules {
    pub minimum_damage_after_resistance: u16,
    pub minimum_resistance_percent: i16,
    pub maximum_resistance_percent: i16,
}

impl Default for DamageRules {
    fn default() -> Self {
        Self {
            minimum_damage_after_resistance: 1,
            minimum_resistance_percent: -100,
            maximum_resistance_percent: 100,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResistanceProfile {
    percentages: [i16; DamageType::COUNT],
}

impl Default for ResistanceProfile {
    fn default() -> Self {
        Self {
            percentages: [0; DamageType::COUNT],
        }
    }
}

impl ResistanceProfile {
    pub fn with(mut self, damage_type: DamageType, percentage: i16) -> Self {
        self.percentages[damage_type.index()] = percentage;
        self
    }

    pub const fn get(self, damage_type: DamageType) -> i16 {
        self.percentages[damage_type.index()]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResolvedDamage {
    pub amount: u16,
    pub effective_resistance_percent: i16,
}

pub fn resolve_damage(
    packet: DamagePacket,
    resistances: ResistanceProfile,
    rules: DamageRules,
) -> ResolvedDamage {
    let raw_resistance = i32::from(resistances.get(packet.damage_type));
    let penetrated = raw_resistance - i32::from(packet.penetration);
    let configured_minimum = i32::from(rules.minimum_resistance_percent);
    let configured_maximum = i32::from(rules.maximum_resistance_percent);
    let minimum_resistance = configured_minimum.min(configured_maximum);
    let maximum_resistance = configured_minimum.max(configured_maximum);
    let effective_resistance = penetrated.clamp(minimum_resistance, maximum_resistance);
    let scaled = i32::from(packet.amount) * (100 - effective_resistance) / 100;
    let minimum_damage = if packet.amount == 0 {
        0
    } else {
        i32::from(rules.minimum_damage_after_resistance)
    };
    let amount = scaled.max(minimum_damage).clamp(0, i32::from(u16::MAX)) as u16;

    ResolvedDamage {
        amount,
        effective_resistance_percent: effective_resistance as i16,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resistance_and_penetration_are_resolved_centrally() {
        let resistances = ResistanceProfile::default().with(DamageType::Kinetic, 50);

        let result = resolve_damage(
            DamagePacket::new(20, DamageType::Kinetic, 10),
            resistances,
            DamageRules::default(),
        );

        assert_eq!(result.amount, 12);
        assert_eq!(result.effective_resistance_percent, 40);
    }

    #[test]
    fn vulnerability_increases_damage() {
        let resistances = ResistanceProfile::default().with(DamageType::Thermal, -50);

        let result = resolve_damage(
            DamagePacket::new(10, DamageType::Thermal, 0),
            resistances,
            DamageRules::default(),
        );

        assert_eq!(result.amount, 15);
    }
}
