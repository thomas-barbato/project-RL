mod attack;
mod damage;
mod hit;

pub use attack::{
    AttackArea, AttackAreaCell, AttackDelivery, AttackImpactError, AttackPreview, AttackProfile,
    ConeAttack, ConeAttackError, MeleeArc, MeleeArcError, MeleeImpactProfile,
    PreparationDisruption, PreparationDisruptionFamily,
};
pub use damage::{
    ArmorProfile, ArmorRules, DamageComponent, DamageImpact, DamageImpactError, DamagePacket,
    DamageRules, DamageType, ResistanceProfile, ResolvedDamage, ResolvedDamageComponent,
    ResolvedDamageImpact, resolve_damage, resolve_damage_impact, resolve_damage_impact_with_armor,
    resolve_damage_with_armor,
};
pub use hit::{HitRules, HitRulesError};
