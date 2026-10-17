#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum DamageType {
    Kinetic,
    Piercing,
    Explosive,
    Thermal,
    Electrical,
    Chemical,
    Radiation,
    Corruption,
    Cold,
}

impl DamageType {
    pub const COUNT: usize = 9;

    pub const ALL: [Self; Self::COUNT] = [
        Self::Kinetic,
        Self::Piercing,
        Self::Explosive,
        Self::Thermal,
        Self::Electrical,
        Self::Chemical,
        Self::Radiation,
        Self::Corruption,
        Self::Cold,
    ];

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
            Self::Cold => 8,
        }
    }

    /// Physical families share one fixed Armor reduction in the current
    /// model. Specialized resistances remain separate percentage defenses.
    pub const fn is_physical(self) -> bool {
        matches!(self, Self::Kinetic | Self::Piercing | Self::Explosive)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DamagePacket {
    pub amount: u16,
    pub damage_type: DamageType,
    /// Defense penetration whose unit follows the defended family: Armor
    /// points for physical damage, percentage points for legacy/specialized
    /// resistance resolution. It is never applied to both for one packet.
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
pub struct DamageComponent {
    pub amount: u16,
    pub damage_type: DamageType,
}

impl DamageComponent {
    pub const fn new(amount: u16, damage_type: DamageType) -> Self {
        Self {
            amount,
            damage_type,
        }
    }
}

/// All damage delivered by one impact. Amounts are stored once per family so
/// Armor and specialized resistances cannot be applied repeatedly because a
/// content file split one impact into several rows.
#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DamageImpact {
    #[serde(with = "damage_array")]
    amounts: [u16; DamageType::COUNT],
    primary_damage_type: DamageType,
    armor_penetration: u16,
    #[serde(with = "damage_array")]
    resistance_penetrations: [u16; DamageType::COUNT],
    legacy_penetration: Option<u16>,
}

impl DamageImpact {
    /// Compatibility constructor for historical single-component packets.
    /// Their untyped penetration follows whichever defense the run assigns to
    /// the component, exactly as it did before mixed impacts existed.
    pub const fn single(packet: DamagePacket) -> Self {
        let mut amounts = [0; DamageType::COUNT];
        amounts[packet.damage_type.index()] = packet.amount;
        let mut resistance_penetrations = [0; DamageType::COUNT];
        resistance_penetrations[packet.damage_type.index()] = packet.penetration;
        Self {
            amounts,
            primary_damage_type: packet.damage_type,
            armor_penetration: packet.penetration,
            resistance_penetrations,
            legacy_penetration: Some(packet.penetration),
        }
    }

    pub fn mixed(
        components: impl IntoIterator<Item = DamageComponent>,
        armor_penetration: u16,
        resistance_penetrations: impl IntoIterator<Item = (DamageType, u16)>,
    ) -> Result<Self, DamageImpactError> {
        let mut amounts = [0; DamageType::COUNT];
        let mut primary_damage_type = None;
        let mut component_count = 0_usize;
        let mut total = 0_u32;
        for component in components {
            if component.amount == 0 {
                return Err(DamageImpactError::ZeroComponent(component.damage_type));
            }
            let index = component.damage_type.index();
            if amounts[index] != 0 {
                return Err(DamageImpactError::DuplicateComponent(component.damage_type));
            }
            primary_damage_type.get_or_insert(component.damage_type);
            amounts[index] = component.amount;
            component_count += 1;
            total = total.saturating_add(u32::from(component.amount));
        }
        if component_count < 2 {
            return Err(DamageImpactError::RequiresSeveralComponents);
        }
        if total > u32::from(u16::MAX) {
            return Err(DamageImpactError::TotalOverflow);
        }

        let mut typed_penetrations = [0; DamageType::COUNT];
        let mut penetration_seen = [false; DamageType::COUNT];
        for (damage_type, percentage_points) in resistance_penetrations {
            let index = damage_type.index();
            if amounts[index] == 0 {
                return Err(DamageImpactError::PenetrationWithoutComponent(damage_type));
            }
            if percentage_points == 0 {
                return Err(DamageImpactError::ZeroResistancePenetration(damage_type));
            }
            if penetration_seen[index] {
                return Err(DamageImpactError::DuplicateResistancePenetration(
                    damage_type,
                ));
            }
            penetration_seen[index] = true;
            typed_penetrations[index] = percentage_points;
        }

        Ok(Self {
            amounts,
            primary_damage_type: primary_damage_type
                .expect("a mixed impact was validated with at least two components"),
            armor_penetration,
            resistance_penetrations: typed_penetrations,
            legacy_penetration: None,
        })
    }

    pub const fn primary_damage_type(self) -> DamageType {
        self.primary_damage_type
    }

    /// Scales every authored damage family once while preserving its defense
    /// metadata. A non-zero component remains at least one point so an
    /// economy tuning cannot silently erase a damage family through integer
    /// rounding.
    pub fn scaled_percentage(mut self, percentage: u16) -> Self {
        for amount in &mut self.amounts {
            if *amount == 0 {
                continue;
            }
            let product = u32::from(*amount).saturating_mul(u32::from(percentage));
            let scaled = if percentage > 100 {
                product.div_ceil(100)
            } else {
                product / 100
            };
            *amount = u16::try_from(scaled.clamp(1, u32::from(u16::MAX))).unwrap_or(u16::MAX);
        }
        self
    }

    /// Spatial retention may round tiny components to zero. All defense metadata
    /// stays attached to the original impact, without a second scale or a floor.
    pub(crate) fn map_amounts(mut self, mut scale: impl FnMut(u16) -> u16) -> Self {
        for amount in &mut self.amounts {
            *amount = scale(*amount);
        }
        if self.raw_amount(self.primary_damage_type) == 0
            && let Some(component) = self.components().next()
        {
            self.primary_damage_type = component.damage_type;
        }
        self
    }

    pub fn primary_component(self) -> DamagePacket {
        let penetration = if self.primary_damage_type.is_physical() {
            self.armor_penetration
        } else {
            self.resistance_penetration(self.primary_damage_type)
        };
        DamagePacket::new(
            self.raw_amount(self.primary_damage_type),
            self.primary_damage_type,
            self.legacy_penetration.unwrap_or(penetration),
        )
    }

    pub fn components(self) -> impl Iterator<Item = DamageComponent> {
        DamageType::ALL.into_iter().filter_map(move |damage_type| {
            let amount = self.raw_amount(damage_type);
            (amount > 0).then_some(DamageComponent::new(amount, damage_type))
        })
    }

    pub const fn raw_amount(self, damage_type: DamageType) -> u16 {
        self.amounts[damage_type.index()]
    }

    pub fn raw_total(self) -> u16 {
        self.components().fold(0_u16, |total, component| {
            total.saturating_add(component.amount)
        })
    }

    pub fn component_count(self) -> usize {
        self.components().count()
    }

    pub const fn armor_penetration(self) -> u16 {
        self.armor_penetration
    }

    /// Adds a temporary physical Armor penetration contribution while keeping
    /// every authored damage component unchanged. Legacy single-component
    /// packets retain their historical shared penetration representation.
    pub const fn with_additional_armor_penetration(mut self, amount: u16) -> Self {
        self.armor_penetration = self.armor_penetration.saturating_add(amount);
        if let Some(legacy) = self.legacy_penetration
            && self.primary_damage_type.is_physical()
        {
            let penetration = legacy.saturating_add(amount);
            self.legacy_penetration = Some(penetration);
            self.resistance_penetrations[self.primary_damage_type.index()] = penetration;
        }
        self
    }

    pub const fn resistance_penetration(self, damage_type: DamageType) -> u16 {
        self.resistance_penetrations[damage_type.index()]
    }

    pub const fn is_legacy_single(self) -> bool {
        self.legacy_penetration.is_some()
    }

    pub fn contains(self, damage_type: DamageType) -> bool {
        self.raw_amount(damage_type) > 0
    }

    pub fn has_physical_component(self) -> bool {
        self.components()
            .any(|component| component.damage_type.is_physical())
    }

    pub fn raw_physical_total(self) -> u16 {
        self.components()
            .filter(|component| component.damage_type.is_physical())
            .fold(0_u16, |total, component| {
                total.saturating_add(component.amount)
            })
    }

    /// Applies the authored melee Impact result to the complete physical part
    /// while preserving its deterministic family proportions. The Armor
    /// resolver will still group these families before applying protection.
    pub fn with_physical_total(mut self, new_total: u16) -> Self {
        let original_total = u32::from(self.raw_physical_total());
        if original_total == 0 {
            return self;
        }

        let original_amounts = self.amounts;
        let physical_count = DamageType::ALL
            .into_iter()
            .filter(|damage_type| {
                damage_type.is_physical() && original_amounts[damage_type.index()] > 0
            })
            .count();
        let mut raw_remaining = original_total;
        let mut resolved_remaining = u32::from(new_total);
        let mut processed = 0_usize;
        for damage_type in DamageType::ALL.into_iter().filter(|damage_type| {
            damage_type.is_physical() && original_amounts[damage_type.index()] > 0
        }) {
            processed += 1;
            let raw = u32::from(original_amounts[damage_type.index()]);
            let amount = if processed == physical_count {
                resolved_remaining
            } else {
                resolved_remaining.saturating_mul(raw) / raw_remaining
            };
            self.amounts[damage_type.index()] = amount.min(u32::from(u16::MAX)) as u16;
            raw_remaining = raw_remaining.saturating_sub(raw);
            resolved_remaining = resolved_remaining.saturating_sub(amount);
        }
        self
    }
}

impl std::fmt::Debug for DamageImpact {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_legacy_single() {
            return self.primary_component().fmt(formatter);
        }
        formatter
            .debug_struct("DamageImpact")
            .field("components", &self.components().collect::<Vec<_>>())
            .field("armor_penetration", &self.armor_penetration)
            .field(
                "resistance_penetrations",
                &DamageType::ALL
                    .into_iter()
                    .filter_map(|damage_type| {
                        let penetration = self.resistance_penetration(damage_type);
                        (penetration > 0).then_some((damage_type, penetration))
                    })
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DamageImpactError {
    RequiresSeveralComponents,
    ZeroComponent(DamageType),
    DuplicateComponent(DamageType),
    TotalOverflow,
    PenetrationWithoutComponent(DamageType),
    ZeroResistancePenetration(DamageType),
    DuplicateResistancePenetration(DamageType),
}

impl std::fmt::Display for DamageImpactError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequiresSeveralComponents => {
                write!(formatter, "a mixed impact requires at least two components")
            }
            Self::ZeroComponent(damage_type) => {
                write!(
                    formatter,
                    "{damage_type:?} damage component must be positive"
                )
            }
            Self::DuplicateComponent(damage_type) => write!(
                formatter,
                "{damage_type:?} damage components must be grouped in one entry"
            ),
            Self::TotalOverflow => write!(formatter, "mixed impact total exceeds u16"),
            Self::PenetrationWithoutComponent(damage_type) => write!(
                formatter,
                "{damage_type:?} resistance penetration has no matching component"
            ),
            Self::ZeroResistancePenetration(damage_type) => write!(
                formatter,
                "{damage_type:?} resistance penetration must be positive when declared"
            ),
            Self::DuplicateResistancePenetration(damage_type) => write!(
                formatter,
                "{damage_type:?} resistance penetration is declared more than once"
            ),
        }
    }
}

impl std::error::Error for DamageImpactError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DamageRules {
    pub minimum_damage_after_resistance: u16,
    pub minimum_resistance_percent: i16,
    pub maximum_resistance_percent: i16,
}

impl DamageRules {
    /// Current specialized-resistance model: vulnerabilities stop at -50%,
    /// resistances at 75%, and rounding may reduce a small packet to zero.
    pub const fn specialized() -> Self {
        Self {
            minimum_damage_after_resistance: 0,
            minimum_resistance_percent: -50,
            maximum_resistance_percent: 75,
        }
    }
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

/// Selects which damage families meet fixed Armor rather than a percentage
/// resistance. Keeping this in run rules lets a total-conversion mod change
/// the classification without branching in combat resolution.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct ArmorRules {
    protected_damage_types: [bool; DamageType::COUNT],
}

impl ArmorRules {
    pub fn with(mut self, damage_type: DamageType, protected: bool) -> Self {
        self.protected_damage_types[damage_type.index()] = protected;
        self
    }

    pub const fn protects(self, damage_type: DamageType) -> bool {
        self.protected_damage_types[damage_type.index()]
    }
}

impl Default for ArmorRules {
    fn default() -> Self {
        let mut protected_damage_types = [false; DamageType::COUNT];
        protected_damage_types[DamageType::Kinetic.index()] = true;
        protected_damage_types[DamageType::Piercing.index()] = true;
        protected_damage_types[DamageType::Explosive.index()] = true;
        Self {
            protected_damage_types,
        }
    }
}

/// Explicit Armor sources for one target at the instant of an impact. Keeping
/// each source separate lets rulesets add equipment, reinforcement or
/// fragilization without rewriting the damage formula or storing a duplicated
/// total.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ArmorProfile {
    body: u16,
    equipment: u16,
    reinforcement: u16,
    fragilization: u16,
    physical_resistance_percent: u8,
}

impl ArmorProfile {
    pub const fn new(body: u16, equipment: u16, reinforcement: u16, fragilization: u16) -> Self {
        Self {
            body,
            equipment,
            reinforcement,
            fragilization,
            physical_resistance_percent: 0,
        }
    }

    /// MM21: a separate percentage defense, applied after physical Armor.
    /// Kept on this computed profile so actor/save layouts remain unchanged.
    pub const fn with_physical_resistance(mut self, percentage: u8) -> Self {
        self.physical_resistance_percent = if percentage > 75 { 75 } else { percentage };
        self
    }

    pub const fn physical_resistance_percent(self) -> u8 {
        self.physical_resistance_percent
    }

    pub const fn body(self) -> u16 {
        self.body
    }

    pub const fn equipment(self) -> u16 {
        self.equipment
    }

    pub const fn reinforcement(self) -> u16 {
        self.reinforcement
    }

    pub const fn fragilization(self) -> u16 {
        self.fragilization
    }

    pub fn total_before_fragilization(self) -> u16 {
        u32::from(self.body)
            .saturating_add(u32::from(self.equipment))
            .saturating_add(u32::from(self.reinforcement))
            .min(u32::from(u16::MAX)) as u16
    }

    pub fn after_fragilization(self) -> u16 {
        self.total_before_fragilization()
            .saturating_sub(self.fragilization)
    }

    pub fn effective_against(self, penetration: u16) -> u16 {
        self.after_fragilization().saturating_sub(penetration)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ResistanceProfile {
    #[serde(with = "damage_array")]
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
    pub effective_armor: u16,
    pub absorbed_by_armor: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResolvedDamageComponent {
    pub damage_type: DamageType,
    pub raw_amount: u16,
    pub amount: u16,
    pub effective_resistance_percent: i16,
    pub armor_protected: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResolvedDamageImpact {
    amount: u16,
    components: [Option<ResolvedDamageComponent>; DamageType::COUNT],
    pub effective_armor: u16,
    pub absorbed_by_armor: u16,
}

impl ResolvedDamageImpact {
    pub const fn amount(self) -> u16 {
        self.amount
    }

    pub fn components(self) -> impl Iterator<Item = ResolvedDamageComponent> {
        self.components.into_iter().flatten()
    }

    pub const fn component(self, damage_type: DamageType) -> Option<ResolvedDamageComponent> {
        self.components[damage_type.index()]
    }
}

pub fn resolve_damage(
    packet: DamagePacket,
    resistances: ResistanceProfile,
    rules: DamageRules,
) -> ResolvedDamage {
    let impact = resolve_damage_impact(DamageImpact::single(packet), resistances, rules);
    let component = impact
        .component(packet.damage_type)
        .expect("a single packet always produces one resolved component");
    ResolvedDamage {
        amount: impact.amount(),
        effective_resistance_percent: component.effective_resistance_percent,
        effective_armor: impact.effective_armor,
        absorbed_by_armor: impact.absorbed_by_armor,
    }
}

/// Resolves the current defense model. A physical packet meets Armor once and
/// then receives the separately authored physical percentage resistance.
/// Other types ignore Armor and use their specialized resistance. Physical
/// penetration is consumed by Armor only, never a second time by MM21.
pub fn resolve_damage_with_armor(
    packet: DamagePacket,
    resistances: ResistanceProfile,
    armor: ArmorProfile,
    damage_rules: DamageRules,
    armor_rules: ArmorRules,
) -> ResolvedDamage {
    let impact = resolve_damage_impact_with_armor(
        DamageImpact::single(packet),
        resistances,
        armor,
        damage_rules,
        armor_rules,
    );
    let component = impact
        .component(packet.damage_type)
        .expect("a single packet always produces one resolved component");
    ResolvedDamage {
        amount: impact.amount(),
        effective_resistance_percent: component.effective_resistance_percent,
        effective_armor: impact.effective_armor,
        absorbed_by_armor: impact.absorbed_by_armor,
    }
}

pub fn resolve_damage_impact(
    impact: DamageImpact,
    resistances: ResistanceProfile,
    damage_rules: DamageRules,
) -> ResolvedDamageImpact {
    resolve_damage_impact_internal(impact, resistances, None, damage_rules)
}

/// Resolves every component of one impact. All families selected by
/// `ArmorRules` are summed first, Armor is subtracted exactly once, and every
/// remaining family meets its own percentage resistance once.
pub fn resolve_damage_impact_with_armor(
    impact: DamageImpact,
    resistances: ResistanceProfile,
    armor: ArmorProfile,
    damage_rules: DamageRules,
    armor_rules: ArmorRules,
) -> ResolvedDamageImpact {
    resolve_damage_impact_internal(
        impact,
        resistances,
        Some((armor, armor_rules)),
        damage_rules,
    )
}

/// A single spell component keeps its source multiplier fractional until its
/// elemental defense is applied. Ordinary weapon resolution remains unchanged.
pub fn resolve_scaled_spell_damage(
    packet: DamagePacket,
    numerator: u128,
    denominator: u128,
    resistances: ResistanceProfile,
    armor: ArmorProfile,
    rules: DamageRules,
    armor_rules: Option<ArmorRules>,
) -> ResolvedDamageImpact {
    let denominator = u128::from(denominator.max(1));
    let raw = u128::from(packet.amount) * u128::from(numerator);
    let whole = (raw / denominator).min(u128::from(u16::MAX)) as u16;
    let impact = DamageImpact::single(DamagePacket::new(
        whole,
        packet.damage_type,
        packet.penetration,
    ));
    let mut resolved = armor_rules.map_or_else(
        || resolve_damage_impact(impact, resistances, rules),
        |profile| resolve_damage_impact_with_armor(impact, resistances, armor, rules, profile),
    );
    if !armor_rules.is_some_and(|profile| profile.protects(packet.damage_type)) {
        let resistance =
            (i32::from(resistances.get(packet.damage_type)) - i32::from(packet.penetration)).clamp(
                i32::from(
                    rules
                        .minimum_resistance_percent
                        .min(rules.maximum_resistance_percent),
                ),
                i32::from(
                    rules
                        .minimum_resistance_percent
                        .max(rules.maximum_resistance_percent),
                ),
            );
        let factor = (100 - resistance).max(0) as u128;
        let capped_raw = raw.min(u128::from(u16::MAX) * denominator);
        let amount = (capped_raw * factor / (denominator * 100))
            .max(if raw == 0 {
                0
            } else {
                u128::from(rules.minimum_damage_after_resistance)
            })
            .min(u128::from(u16::MAX)) as u16;
        resolved.amount = amount;
        if let Some(component) = &mut resolved.components[packet.damage_type.index()] {
            component.amount = amount;
            component.effective_resistance_percent = resistance as i16;
        }
    }
    resolved
}

fn resolve_damage_impact_internal(
    impact: DamageImpact,
    resistances: ResistanceProfile,
    armor: Option<(ArmorProfile, ArmorRules)>,
    damage_rules: DamageRules,
) -> ResolvedDamageImpact {
    let mut components = [None; DamageType::COUNT];
    let protected_raw = armor.map_or(0_u16, |(_, armor_rules)| {
        impact
            .components()
            .filter(|component| armor_rules.protects(component.damage_type))
            .fold(0_u16, |total, component| {
                total.saturating_add(component.amount)
            })
    });
    let (effective_armor, absorbed_by_armor, protected_remaining) =
        armor.map_or((0, 0, 0), |(armor, _)| {
            if protected_raw == 0 {
                return (0, 0, 0);
            }
            let effective = armor.effective_against(impact.armor_penetration());
            (
                effective,
                protected_raw.min(effective),
                protected_raw.saturating_sub(effective),
            )
        });

    let physical_resistance = armor.map_or(0, |(profile, _)| profile.physical_resistance_percent());
    // Round the whole physical group once, before distributing its remainder
    // over components. Splitting an impact must not bypass or multiply MM21.
    let protected_remaining =
        (u32::from(protected_remaining) * u32::from(100 - physical_resistance) / 100) as u16;

    let mut protected_raw_remaining = u32::from(protected_raw);
    let mut protected_damage_remaining = u32::from(protected_remaining);
    for component in impact.components() {
        let armor_protected =
            armor.is_some_and(|(_, armor_rules)| armor_rules.protects(component.damage_type));
        let resolved = if armor_protected {
            let raw = u32::from(component.amount);
            let amount = if raw == protected_raw_remaining {
                protected_damage_remaining
            } else {
                protected_damage_remaining.saturating_mul(raw) / protected_raw_remaining
            };
            protected_raw_remaining = protected_raw_remaining.saturating_sub(raw);
            protected_damage_remaining = protected_damage_remaining.saturating_sub(amount);
            ResolvedDamageComponent {
                damage_type: component.damage_type,
                raw_amount: component.amount,
                amount: amount.min(u32::from(u16::MAX)) as u16,
                effective_resistance_percent: i16::from(physical_resistance),
                armor_protected: true,
            }
        } else {
            let packet = DamagePacket::new(
                component.amount,
                component.damage_type,
                impact.resistance_penetration(component.damage_type),
            );
            let (amount, effective_resistance_percent) =
                resolve_percentage_resistance(packet, resistances, damage_rules);
            ResolvedDamageComponent {
                damage_type: component.damage_type,
                raw_amount: component.amount,
                amount,
                effective_resistance_percent,
                armor_protected: false,
            }
        };
        components[component.damage_type.index()] = Some(resolved);
    }

    let amount = components
        .into_iter()
        .flatten()
        .fold(0_u16, |total, component| {
            total.saturating_add(component.amount)
        });
    ResolvedDamageImpact {
        amount,
        components,
        effective_armor,
        absorbed_by_armor,
    }
}

fn resolve_percentage_resistance(
    packet: DamagePacket,
    resistances: ResistanceProfile,
    rules: DamageRules,
) -> (u16, i16) {
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

    (amount, effective_resistance as i16)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mm21_reduces_only_physical_damage_after_armor() {
        let impact = DamageImpact::mixed(
            [
                DamageComponent::new(100, DamageType::Kinetic),
                DamageComponent::new(20, DamageType::Thermal),
            ],
            0,
            [],
        )
        .unwrap();
        let resolved = resolve_damage_impact_with_armor(
            impact,
            ResistanceProfile::default().with(DamageType::Thermal, 50),
            ArmorProfile::new(20, 0, 0, 0).with_physical_resistance(30),
            DamageRules::specialized(),
            ArmorRules::default(),
        );
        assert_eq!(resolved.component(DamageType::Kinetic).unwrap().amount, 56);
        assert_eq!(resolved.component(DamageType::Thermal).unwrap().amount, 10);
        assert_eq!(resolved.absorbed_by_armor, 20);
        assert_eq!(resolved.amount(), 66);
    }

    #[test]
    fn mm21_caps_and_does_not_reuse_armor_penetration() {
        let resolved = resolve_damage_with_armor(
            DamagePacket::new(100, DamageType::Kinetic, 10),
            ResistanceProfile::default(),
            ArmorProfile::new(20, 0, 0, 0).with_physical_resistance(30),
            DamageRules::specialized(),
            ArmorRules::default(),
        );
        assert_eq!(resolved.effective_armor, 10);
        assert_eq!(resolved.effective_resistance_percent, 30);
        assert_eq!(resolved.amount, 63);
        let capped = resolve_damage_with_armor(
            DamagePacket::new(100, DamageType::Piercing, 0),
            ResistanceProfile::default(),
            ArmorProfile::default().with_physical_resistance(100),
            DamageRules::specialized(),
            ArmorRules::default(),
        );
        assert_eq!(capped.amount, 25);
        assert_eq!(capped.effective_resistance_percent, 75);
    }

    #[test]
    fn mm21_rounds_the_physical_group_once_and_keeps_full_armor_absorption() {
        let impact = DamageImpact::mixed(
            [
                DamageComponent::new(60, DamageType::Kinetic),
                DamageComponent::new(40, DamageType::Piercing),
            ],
            0,
            [],
        )
        .unwrap();
        let resolve = |armor| {
            resolve_damage_impact_with_armor(
                impact,
                ResistanceProfile::default(),
                armor,
                DamageRules::specialized(),
                ArmorRules::default(),
            )
        };
        assert_eq!(
            resolve(ArmorProfile::new(20, 0, 0, 0).with_physical_resistance(30)).amount(),
            56
        );
        assert_eq!(
            resolve(ArmorProfile::new(100, 0, 0, 0).with_physical_resistance(30)).amount(),
            0
        );
    }

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

    #[test]
    fn armor_uses_explicit_sources_fragilization_and_point_penetration_once() {
        let armor = ArmorProfile::new(5, 3, 2, 4);
        let resistances = ResistanceProfile::default().with(DamageType::Kinetic, 50);

        let result = resolve_damage_with_armor(
            DamagePacket::new(12, DamageType::Kinetic, 2),
            resistances,
            armor,
            DamageRules::specialized(),
            ArmorRules::default(),
        );

        assert_eq!(armor.total_before_fragilization(), 10);
        assert_eq!(armor.after_fragilization(), 6);
        assert_eq!(result.effective_armor, 4);
        assert_eq!(result.absorbed_by_armor, 4);
        assert_eq!(result.amount, 8);
        assert_eq!(result.effective_resistance_percent, 0);
    }

    #[test]
    fn armor_can_absorb_every_physical_damage_point_without_a_minimum() {
        let result = resolve_damage_with_armor(
            DamagePacket::new(6, DamageType::Piercing, 0),
            ResistanceProfile::default(),
            ArmorProfile::new(10, 0, 0, 0),
            DamageRules::specialized(),
            ArmorRules::default(),
        );

        assert_eq!(result.amount, 0);
        assert_eq!(result.absorbed_by_armor, 6);
        assert_eq!(result.effective_armor, 10);
    }

    #[test]
    fn specialized_damage_ignores_armor_and_can_round_to_zero() {
        let armor = ArmorProfile::new(50, 0, 0, 0);
        let resistances = ResistanceProfile::default().with(DamageType::Thermal, 75);

        let ordinary = resolve_damage_with_armor(
            DamagePacket::new(20, DamageType::Thermal, 0),
            resistances,
            armor,
            DamageRules::specialized(),
            ArmorRules::default(),
        );
        let small = resolve_damage_with_armor(
            DamagePacket::new(3, DamageType::Thermal, 0),
            resistances,
            armor,
            DamageRules::specialized(),
            ArmorRules::default(),
        );

        assert_eq!(ordinary.amount, 5);
        assert_eq!(ordinary.effective_armor, 0);
        assert_eq!(small.amount, 0);
    }

    #[test]
    fn mixed_impact_groups_all_physical_components_before_armor() {
        let impact = DamageImpact::mixed(
            [
                DamageComponent::new(6, DamageType::Kinetic),
                DamageComponent::new(4, DamageType::Piercing),
                DamageComponent::new(8, DamageType::Electrical),
            ],
            0,
            [],
        )
        .unwrap();
        let resolved = resolve_damage_impact_with_armor(
            impact,
            ResistanceProfile::default().with(DamageType::Electrical, 50),
            ArmorProfile::new(4, 0, 0, 0),
            DamageRules::specialized(),
            ArmorRules::default(),
        );

        assert_eq!(resolved.amount(), 10);
        assert_eq!(resolved.effective_armor, 4);
        assert_eq!(resolved.absorbed_by_armor, 4);
        assert_eq!(
            resolved
                .components()
                .filter(|component| component.armor_protected)
                .map(|component| component.amount)
                .sum::<u16>(),
            6
        );
        assert_eq!(
            resolved.component(DamageType::Electrical),
            Some(ResolvedDamageComponent {
                damage_type: DamageType::Electrical,
                raw_amount: 8,
                amount: 4,
                effective_resistance_percent: 50,
                armor_protected: false,
            })
        );
    }

    #[test]
    fn typed_penetrations_only_change_their_declared_defense() {
        let impact = DamageImpact::mixed(
            [
                DamageComponent::new(4, DamageType::Kinetic),
                DamageComponent::new(8, DamageType::Electrical),
            ],
            3,
            [(DamageType::Electrical, 10)],
        )
        .unwrap();
        let resolved = resolve_damage_impact_with_armor(
            impact,
            ResistanceProfile::default().with(DamageType::Electrical, 50),
            ArmorProfile::new(10, 0, 0, 0),
            DamageRules::specialized(),
            ArmorRules::default(),
        );

        assert_eq!(resolved.amount(), 4);
        assert_eq!(resolved.effective_armor, 7);
        assert_eq!(resolved.absorbed_by_armor, 4);
        assert_eq!(
            resolved
                .component(DamageType::Electrical)
                .map(|component| component.effective_resistance_percent),
            Some(40)
        );
    }

    #[test]
    fn melee_impact_rescales_the_whole_physical_group_once() {
        let impact = DamageImpact::mixed(
            [
                DamageComponent::new(6, DamageType::Kinetic),
                DamageComponent::new(4, DamageType::Piercing),
                DamageComponent::new(8, DamageType::Thermal),
            ],
            0,
            [],
        )
        .unwrap()
        .with_physical_total(16);

        assert_eq!(impact.raw_amount(DamageType::Kinetic), 9);
        assert_eq!(impact.raw_amount(DamageType::Piercing), 7);
        assert_eq!(impact.raw_amount(DamageType::Thermal), 8);
        assert_eq!(impact.raw_total(), 24);
    }

    #[test]
    fn scaled_spells_round_after_elemental_resistance_and_critical() {
        let result = resolve_scaled_spell_damage(
            DamagePacket::new(8, DamageType::Corruption, 0),
            22,
            10,
            ResistanceProfile::default().with(DamageType::Corruption, 25),
            ArmorProfile::new(100, 0, 0, 0),
            DamageRules::specialized(),
            Some(ArmorRules::default()),
        );
        assert_eq!(result.amount(), 13); // floor(8 * 1.1 * 2 * 0.75), not floor(8 * 1.1) * 2.
        assert_eq!(result.effective_armor, 0);
        assert_eq!(result.component(DamageType::Corruption).unwrap().amount, 13);
        for resistance in [-50, 0, 25, 75, 100] {
            let defenses = ResistanceProfile::default().with(DamageType::Corruption, resistance);
            let ordinary = resolve_damage_impact_with_armor(
                DamageImpact::single(DamagePacket::new(19, DamageType::Corruption, 0)),
                defenses,
                ArmorProfile::new(100, 0, 0, 0),
                DamageRules::specialized(),
                ArmorRules::default(),
            );
            assert_eq!(
                resolve_scaled_spell_damage(
                    DamagePacket::new(19, DamageType::Corruption, 0),
                    1,
                    1,
                    defenses,
                    ArmorProfile::new(100, 0, 0, 0),
                    DamageRules::specialized(),
                    Some(ArmorRules::default()),
                ),
                ordinary
            );
        }
    }

    #[test]
    fn mixed_impact_rejects_ambiguous_or_ungrouped_content() {
        assert_eq!(
            DamageImpact::mixed([DamageComponent::new(3, DamageType::Kinetic)], 0, [],),
            Err(DamageImpactError::RequiresSeveralComponents)
        );
        assert_eq!(
            DamageImpact::mixed(
                [
                    DamageComponent::new(3, DamageType::Kinetic),
                    DamageComponent::new(2, DamageType::Kinetic),
                ],
                0,
                [],
            ),
            Err(DamageImpactError::DuplicateComponent(DamageType::Kinetic))
        );
        assert_eq!(
            DamageImpact::mixed(
                [
                    DamageComponent::new(3, DamageType::Kinetic),
                    DamageComponent::new(2, DamageType::Thermal),
                ],
                0,
                [(DamageType::Electrical, 4)],
            ),
            Err(DamageImpactError::PenetrationWithoutComponent(
                DamageType::Electrical
            ))
        );
    }
}

// Nine current damage families; binary saves through v14 carried eight.
mod damage_array {
    use serde::{Deserialize, Serialize};
    pub fn serialize<T: Serialize + Copy, S: serde::Serializer>(
        value: &[T; 9],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if !serializer.is_human_readable()
            && crate::game::game_state::agent_runtime::legacy_encoding()
        {
            let old: [T; 8] = std::array::from_fn(|i| value[i]);
            old.serialize(serializer)
        } else {
            value.serialize(serializer)
        }
    }
    pub fn deserialize<'de, T: Deserialize<'de> + Copy + Default, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<[T; 9], D::Error> {
        if deserializer.is_human_readable() {
            let values = Vec::<T>::deserialize(deserializer)?;
            if values.len() != 8 && values.len() != 9 {
                return Err(serde::de::Error::custom(
                    "expected eight or nine damage families",
                ));
            }
            let mut result = [T::default(); 9];
            result[..values.len()].copy_from_slice(&values);
            Ok(result)
        } else if crate::game::game_state::agent_runtime::legacy_encoding() {
            let old = <[T; 8]>::deserialize(deserializer)?;
            let mut result = [T::default(); 9];
            result[..8].copy_from_slice(&old);
            Ok(result)
        } else {
            <[T; 9]>::deserialize(deserializer)
        }
    }
}
impl std::fmt::Debug for ResistanceProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values: &[i16] = if self.percentages[8] == 0 {
            &self.percentages[..8]
        } else {
            &self.percentages
        };
        f.debug_struct("ResistanceProfile")
            .field("percentages", &values)
            .finish()
    }
}

// Existing run fingerprints must keep their original eight-family rules when
// the appended Cold family does not use Armor.
impl std::fmt::Debug for ArmorRules {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values: &[bool] = if !self.protected_damage_types[DamageType::Cold.index()] {
            &self.protected_damage_types[..8]
        } else {
            &self.protected_damage_types
        };
        f.debug_struct("ArmorRules")
            .field("protected_damage_types", &values)
            .finish()
    }
}
