use crate::combat::{AttackProfile, DamageRules, DamageType};
use crate::effects::{AbilityProfile, DamageFalloff, EffectPrimitive, RadialDamageEffect};
use crate::entity::EquipmentSlotId;
use crate::progression::ProgressionRules;
use crate::status::StatusCatalog;
use crate::weapon::{WeaponCatalog, WeaponId};
use crate::world::{DistanceMetric, FieldOfViewRules, NeighborMode, TerrainPropagationPolicy};

/// Rules that shape a run independently from its mutable state.
///
/// The structure is intentionally data-shaped so the content/mod layer can
/// construct it later without replacing simulation code.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameRules {
    pub player_field_of_view: FieldOfViewRules,
    pub damage: DamageRules,
    pub player_maximum_integrity: u16,
    /// Intrinsic/fallback attacks used by actors that have no equipment model.
    pub player_base_attacks: Vec<AttackProfile>,
    pub player_base_abilities: Vec<AbilityProfile>,
    pub player_inventory_capacity: usize,
    /// Ordered attack channels. `GameCommand::Attack::slot` indexes this list.
    pub player_weapon_slots: Vec<EquipmentSlotId>,
    pub player_starting_weapons: Vec<WeaponId>,
    /// Weapon assigned to each ordered slot at run start. `None` leaves it empty.
    pub player_starting_equipment: Vec<Option<WeaponId>>,
    pub progression: ProgressionRules,
    pub statuses: StatusCatalog,
    pub weapons: WeaponCatalog,
}

impl Default for GameRules {
    fn default() -> Self {
        Self {
            player_field_of_view: FieldOfViewRules::default(),
            damage: DamageRules::default(),
            player_maximum_integrity: 20,
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
            player_starting_equipment: Vec::new(),
            progression: ProgressionRules::default(),
            statuses: StatusCatalog::default(),
            weapons: WeaponCatalog::default(),
        }
    }
}
