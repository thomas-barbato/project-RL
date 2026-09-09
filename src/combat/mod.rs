mod attack;
mod damage;

pub use attack::AttackProfile;
pub use damage::{
    DamagePacket, DamageRules, DamageType, ResistanceProfile, ResolvedDamage, resolve_damage,
};
