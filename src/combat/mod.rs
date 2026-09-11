mod attack;
mod damage;

pub use attack::{
    AttackArea, AttackAreaCell, AttackPreview, AttackProfile, ConeAttack, ConeAttackError,
};
pub use damage::{
    DamagePacket, DamageRules, DamageType, ResistanceProfile, ResolvedDamage, resolve_damage,
};
