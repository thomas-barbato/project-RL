//! Reviewed encounter traits. Identity lives in the actor's existing tag set,
//! so traits survive snapshots without extending the actor's binary layout.
//! Unquantified proposals are not rolled or presented as functioning powers.
use crate::combat::DamageType;
use crate::entity::Actor;

pub const PHYSICAL_RESISTANCE_TAG: &str = "core:monster_modifier_mm21";
pub const PHYSICAL_RESISTANCE_PERCENT: u8 = 30;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MonsterModifier {
    FireDamage,
    ColdDamage,
    ElectricalDamage,
    FireResistance,
    ColdResistance,
    PoisonResistance,
    PhysicalResistance,
}

impl MonsterModifier {
    pub const fn review_id(self) -> &'static str {
        match self {
            Self::FireDamage => "MM01",
            Self::ColdDamage => "MM02",
            Self::ElectricalDamage => "MM04",
            Self::FireResistance => "MM15",
            Self::ColdResistance => "MM16",
            Self::PoisonResistance => "MM17",
            Self::PhysicalResistance => "MM21",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::FireDamage => "Dégâts de feu",
            Self::ColdDamage => "Dégâts de froid",
            Self::ElectricalDamage => "Dégâts électriques",
            Self::FireResistance => "Résistant au feu",
            Self::ColdResistance => "Résistant au froid",
            Self::PoisonResistance => "Résistant au poison",
            Self::PhysicalResistance => "Résistant aux dégâts physiques",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Self::FireDamage => "Ses attaques infligent des dégâts de feu.",
            Self::ColdDamage => "Ses attaques infligent des dégâts de froid.",
            Self::ElectricalDamage => "Ses attaques infligent des dégâts électriques.",
            Self::FireResistance => "Il subit moins de dégâts de feu.",
            Self::ColdResistance => "Il subit moins de dégâts de froid.",
            Self::PoisonResistance => "Il subit moins de dégâts de poison.",
            Self::PhysicalResistance => {
                "Résistance physique : +30 points, après l'armure. Plafond : 75 %."
            }
        }
    }
}

pub fn physical_resistance_percent(actor: &Actor) -> u8 {
    if actor
        .tags()
        .iter()
        .any(|tag| tag.as_str() == PHYSICAL_RESISTANCE_TAG)
    {
        PHYSICAL_RESISTANCE_PERCENT
    } else {
        0
    }
}

/// Derive descriptions from actual attacks/defenses, never from sprite colors.
/// A set prevents repeated native components and tags duplicating a keyword.
pub fn visible_modifiers(actor: &Actor) -> Vec<MonsterModifier> {
    use MonsterModifier::*;
    let mut result = std::collections::BTreeSet::new();
    for attack in actor.attacks() {
        for component in attack.damage().components() {
            if let Some(modifier) = match component.damage_type {
                DamageType::Thermal => Some(FireDamage),
                DamageType::Cold => Some(ColdDamage),
                DamageType::Electrical => Some(ElectricalDamage),
                _ => None,
            } {
                result.insert(modifier);
            }
        }
    }
    for (damage_type, modifier) in [
        (DamageType::Thermal, FireResistance),
        (DamageType::Cold, ColdResistance),
        (DamageType::Chemical, PoisonResistance),
    ] {
        if actor.resistances().get(damage_type) > 0 {
            result.insert(modifier);
        }
    }
    if physical_resistance_percent(actor) > 0 {
        result.insert(PhysicalResistance);
    }
    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::{AttackProfile, ResistanceProfile};
    use crate::world::GridPos;

    #[test]
    fn native_traits_are_unique_and_do_not_add_damage_or_change_actor_layout() {
        let actor = Actor::new(GridPos::new(2, 3), 20)
            .unwrap()
            .with_attack(AttackProfile::melee(DamageType::Thermal, 10))
            .with_attack(AttackProfile::melee(DamageType::Thermal, 30))
            .with_resistances(ResistanceProfile::default().with(DamageType::Cold, 20))
            .with_tags([
                PHYSICAL_RESISTANCE_TAG.parse().unwrap(),
                PHYSICAL_RESISTANCE_TAG.parse().unwrap(),
            ]);
        assert_eq!(
            visible_modifiers(&actor),
            vec![
                MonsterModifier::FireDamage,
                MonsterModifier::ColdResistance,
                MonsterModifier::PhysicalResistance
            ]
        );
        assert_eq!(physical_resistance_percent(&actor), 30);
        let bytes = bincode::serialize(&actor).unwrap();
        let restored: Actor = bincode::deserialize(&bytes).unwrap();
        assert_eq!(restored, actor);
        assert_eq!(visible_modifiers(&restored), visible_modifiers(&actor));
        assert_eq!(restored.attack(0).unwrap().damage().raw_total(), 10);
    }
}
