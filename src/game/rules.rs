use crate::combat::{AttackProfile, DamageRules, DamageType};
use crate::effects::{AbilityProfile, DamageFalloff, EffectPrimitive, RadialDamageEffect};
use crate::entity::EquipmentSlotId;
use crate::item::{ItemCatalog, ItemId};
use crate::progression::ProgressionRules;
use crate::skills::{SkillCatalog, SkillProgressionRules, SystemFeatureSet};
use crate::stats::{PrimaryAttributeRules, PrimaryAttributes};
use crate::status::StatusCatalog;
use crate::weapon::{WeaponCatalog, WeaponId};
use crate::world::{
    DistanceMetric, FieldOfViewRules, MovementTraceRules, NeighborMode, TerrainPropagationPolicy,
};

/// Rules that shape a run independently from its mutable state.
///
/// The structure is intentionally data-shaped so the content/mod layer can
/// construct it later without replacing simulation code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameRules {
    pub player_field_of_view: FieldOfViewRules,
    pub damage: DamageRules,
    pub player_maximum_integrity: u16,
    pub player_energy_capacity: u16,
    pub player_starting_energy: u16,
    pub primary_attribute_rules: PrimaryAttributeRules,
    pub player_starting_attributes: PrimaryAttributes,
    /// Intrinsic/fallback attacks used by actors that have no equipment model.
    pub player_base_attacks: Vec<AttackProfile>,
    pub player_base_abilities: Vec<AbilityProfile>,
    pub player_inventory_capacity: usize,
    /// Ordered attack channels. Entity-locked and freely aimed attack commands
    /// both index this list with their `slot` field.
    pub player_weapon_slots: Vec<EquipmentSlotId>,
    pub player_starting_weapons: Vec<WeaponId>,
    pub player_starting_items: Vec<StartingItemStack>,
    /// Weapon assigned to each ordered slot at run start. `None` leaves it empty.
    pub player_starting_equipment: Vec<Option<WeaponId>>,
    pub progression: ProgressionRules,
    pub skill_progression: SkillProgressionRules,
    pub enabled_system_features: SystemFeatureSet,
    pub movement_traces: MovementTraceRules,
    pub skills: SkillCatalog,
    pub statuses: StatusCatalog,
    pub weapons: WeaponCatalog,
    pub items: ItemCatalog,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StartingItemStack {
    pub item: ItemId,
    pub quantity: u16,
}

impl StartingItemStack {
    pub const fn new(item: ItemId, quantity: u16) -> Self {
        Self { item, quantity }
    }
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            player_field_of_view: FieldOfViewRules::default(),
            damage: DamageRules::default(),
            player_maximum_integrity: 20,
            player_energy_capacity: 100,
            player_starting_energy: 100,
            primary_attribute_rules: PrimaryAttributeRules::default(),
            player_starting_attributes: PrimaryAttributes::prototype_default(),
            player_base_attacks: vec![
                AttackProfile::melee(DamageType::Kinetic, 5),
                AttackProfile::new(
                    7,
                    DistanceMetric::Euclidean,
                    true,
                    DamageType::Piercing,
                    3,
                    1,
                ),
            ],
            player_base_abilities: vec![AbilityProfile::new(
                6,
                DistanceMetric::Euclidean,
                true,
                true,
                vec![EffectPrimitive::RadialDamage(RadialDamageEffect {
                    maximum_cost: 2,
                    neighbor_mode: NeighborMode::Cardinal,
                    propagation_policy: TerrainPropagationPolicy::blocked_by_walls(1),
                    damage: crate::combat::DamagePacket::new(7, DamageType::Explosive, 0),
                    falloff: DamageFalloff::PerPropagationCost(2),
                })],
            )],
            player_inventory_capacity: 12,
            player_weapon_slots: Vec::new(),
            player_starting_weapons: Vec::new(),
            player_starting_items: Vec::new(),
            player_starting_equipment: Vec::new(),
            progression: ProgressionRules::default(),
            skill_progression: SkillProgressionRules::default(),
            enabled_system_features: SystemFeatureSet::default(),
            movement_traces: MovementTraceRules::default(),
            skills: SkillCatalog::default(),
            statuses: StatusCatalog::default(),
            weapons: WeaponCatalog::default(),
            items: ItemCatalog::default(),
        }
    }
}
