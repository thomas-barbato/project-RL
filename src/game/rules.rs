use std::fmt::{Debug, Formatter};

use crate::combat::{ArmorRules, AttackProfile, DamageRules, DamageType, HitRules};
use crate::effects::{AbilityProfile, DamageFalloff, EffectPrimitive, RadialDamageEffect};
use crate::entity::EquipmentSlotId;
use crate::item::{ItemCatalog, ItemId};
use crate::progression::ProgressionRules;
use crate::skills::{SkillCatalog, SkillProgressionRules, SystemFeatureSet};
use crate::stats::{
    BodyProfile, PhysicalRules, PrimaryAttributeRules, PrimaryAttributes, StabilityRules,
};
use crate::status::StatusCatalog;
use crate::stealth::StealthRules;
use crate::weapon::{WeaponCatalog, WeaponId};
use crate::world::{
    DistanceMetric, FieldOfViewRules, MovementTraceRules, NeighborMode, TerrainPropagationPolicy,
};

/// Rules that shape a run independently from its mutable state.
///
/// The structure is intentionally data-shaped so the content/mod layer can
/// construct it later without replacing simulation code.
#[derive(Clone, PartialEq, Eq)]
pub struct GameRules {
    pub player_field_of_view: FieldOfViewRules,
    pub damage: DamageRules,
    /// `None` retains the legacy percentage-resistance resolver. Current runs
    /// use fixed Armor for the configured physical damage families.
    pub armor_rules: Option<ArmorRules>,
    /// `None` retains the historical automatic-hit resolver for legacy
    /// suspensions and focused engine tests. New playable runs opt into an
    /// explicit, versioned coefficient set.
    pub hit_rules: Option<HitRules>,
    /// `None` retains legacy damage and maximum-PV behavior.
    pub physical_rules: Option<PhysicalRules>,
    /// `None` keeps rulesets predating passive Stability checks valid. Content
    /// which declares a Stability-resisted effect requires this coefficient set.
    pub stability_rules: Option<StabilityRules>,
    /// Optional authored body used with `physical_rules`; the legacy maximum
    /// remains available for old suspensions and rulesets.
    pub player_body_profile: Option<BodyProfile>,
    pub player_maximum_integrity: u16,
    pub player_energy_capacity: u16,
    pub player_starting_energy: u16,
    /// Optional programmable resource buses. `None` preserves rulesets from
    /// before heat and bandwidth existed.
    pub player_system_resources: Option<SystemResourceRules>,
    /// Optional deterministic signature and sound coefficients. `None` keeps
    /// pre-stealth generations on their historical binary visibility rules.
    pub stealth_rules: Option<StealthRules>,
    pub primary_attribute_rules: PrimaryAttributeRules,
    pub player_starting_attributes: PrimaryAttributes,
    /// Intrinsic/fallback attacks used by actors that have no equipment model.
    pub player_base_attacks: Vec<AttackProfile>,
    pub player_base_abilities: Vec<AbilityProfile>,
    pub player_inventory_capacity: usize,
    /// Ordered weapon slots. Entity-locked and freely aimed attack commands
    /// both index this list with their `slot` field.
    pub player_weapon_slots: Vec<EquipmentSlotId>,
    /// Ordered non-weapon protection slots accepted by equippable item
    /// profiles. Empty in rulesets predating equippable armor.
    pub player_armor_slots: Vec<EquipmentSlotId>,
    pub player_starting_weapons: Vec<WeaponId>,
    pub player_starting_items: Vec<StartingItemStack>,
    /// Weapon assigned to each ordered slot at run start. `None` leaves it empty.
    pub player_starting_equipment: Vec<Option<WeaponId>>,
    pub progression: ProgressionRules,
    pub skill_progression: SkillProgressionRules,
    /// When enabled, an explicit wait resumes the exact technique payload
    /// currently being prepared. Older suspension generations keep the
    /// historical contract which required the client to submit the technique
    /// command again.
    pub wait_continues_technique_preparation: bool,
    /// Gives newly controlled drones a close escort order and lets that order
    /// opportunistically attack the player's explicit target during the same
    /// action. Advanced authored orders still replace this baseline doctrine.
    pub player_drone_default_support: bool,
    /// Removes a controlled drone as soon as its finite energy reserve reaches
    /// zero. This turns an exhausted manifested unit into an explicit end of
    /// life instead of leaving an inert actor on the map.
    pub player_drone_expires_without_energy: bool,
    /// Enables the timeless, replayable four-behavior companion command.
    /// Drones are the first actor type implementing this generic contract.
    pub player_companion_behaviors: bool,
    pub enabled_system_features: SystemFeatureSet,
    pub movement_traces: MovementTraceRules,
    pub skills: SkillCatalog,
    pub statuses: StatusCatalog,
    pub weapons: WeaponCatalog,
    pub items: ItemCatalog,
}

// Omit disabled hit rules to preserve exact rules fingerprints for suspension
// versions created before accuracy and evasion entered the simulation.
impl Debug for GameRules {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        let mut rules = formatter.debug_struct("GameRules");
        rules
            .field("player_field_of_view", &self.player_field_of_view)
            .field("damage", &self.damage)
            .field("player_maximum_integrity", &self.player_maximum_integrity)
            .field("player_energy_capacity", &self.player_energy_capacity)
            .field("player_starting_energy", &self.player_starting_energy)
            .field("primary_attribute_rules", &self.primary_attribute_rules)
            .field(
                "player_starting_attributes",
                &self.player_starting_attributes,
            )
            .field("player_base_attacks", &self.player_base_attacks)
            .field("player_base_abilities", &self.player_base_abilities)
            .field("player_inventory_capacity", &self.player_inventory_capacity)
            .field("player_weapon_slots", &self.player_weapon_slots)
            .field("player_starting_weapons", &self.player_starting_weapons)
            .field("player_starting_items", &self.player_starting_items)
            .field("player_starting_equipment", &self.player_starting_equipment)
            .field("progression", &self.progression)
            .field("skill_progression", &self.skill_progression)
            .field("enabled_system_features", &self.enabled_system_features)
            .field("movement_traces", &self.movement_traces)
            .field("skills", &self.skills)
            .field("statuses", &self.statuses)
            .field("weapons", &self.weapons)
            .field("items", &self.items);
        if self.wait_continues_technique_preparation {
            rules.field(
                "wait_continues_technique_preparation",
                &self.wait_continues_technique_preparation,
            );
        }
        if self.player_drone_default_support {
            rules.field(
                "player_drone_default_support",
                &self.player_drone_default_support,
            );
        }
        if self.player_drone_expires_without_energy {
            rules.field(
                "player_drone_expires_without_energy",
                &self.player_drone_expires_without_energy,
            );
        }
        if self.player_companion_behaviors {
            rules.field(
                "player_companion_behaviors",
                &self.player_companion_behaviors,
            );
        }
        if let Some(resources) = self.player_system_resources {
            rules.field("player_system_resources", &resources);
        }
        if let Some(stealth) = self.stealth_rules {
            rules.field("stealth_rules", &stealth);
        }
        if !self.player_armor_slots.is_empty() {
            rules.field("player_armor_slots", &self.player_armor_slots);
        }
        if let Some(armor_rules) = self.armor_rules {
            rules.field("armor_rules", &armor_rules);
        }
        if let Some(hit_rules) = self.hit_rules {
            rules.field("hit_rules", &hit_rules);
        }
        if let Some(physical_rules) = self.physical_rules {
            rules.field("physical_rules", &physical_rules);
        }
        if let Some(stability_rules) = self.stability_rules {
            rules.field("stability_rules", &stability_rules);
        }
        if let Some(body_profile) = self.player_body_profile {
            rules.field("player_body_profile", &body_profile);
        }
        rules.finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StartingItemStack {
    pub item: ItemId,
    pub quantity: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SystemResourceRules {
    pub bandwidth_capacity: u16,
    pub heat_alert_threshold: u16,
    pub heat_critical_threshold: u16,
    pub heat_dissipation_per_phase: u16,
}

impl Default for SystemResourceRules {
    fn default() -> Self {
        Self {
            bandwidth_capacity: 4,
            heat_alert_threshold: 40,
            heat_critical_threshold: 80,
            heat_dissipation_per_phase: 4,
        }
    }
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
            armor_rules: None,
            hit_rules: None,
            physical_rules: None,
            stability_rules: None,
            player_body_profile: None,
            player_maximum_integrity: 20,
            player_energy_capacity: 100,
            player_starting_energy: 100,
            player_system_resources: None,
            stealth_rules: None,
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
            player_armor_slots: Vec::new(),
            player_starting_weapons: Vec::new(),
            player_starting_items: Vec::new(),
            player_starting_equipment: Vec::new(),
            progression: ProgressionRules::default(),
            skill_progression: SkillProgressionRules::default(),
            wait_continues_technique_preparation: true,
            player_drone_default_support: true,
            player_drone_expires_without_energy: true,
            player_companion_behaviors: true,
            enabled_system_features: SystemFeatureSet::default(),
            movement_traces: MovementTraceRules::default(),
            skills: SkillCatalog::default(),
            statuses: StatusCatalog::default(),
            weapons: WeaponCatalog::default(),
            items: ItemCatalog::default(),
        }
    }
}
