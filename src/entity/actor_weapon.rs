//! One persistent, actually wielded weapon. Natural attacks are not possessions.
use super::Actor;
use crate::entity::{ItemId, MagicItemModifiers};
use crate::stats::{HitPointRules, PrimaryAttribute, PrimaryAttributes};
use crate::weapon::{WeaponCatalog, WeaponDefinition};

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ActorWeapon {
    item: ItemId,
    modifiers: Option<MagicItemModifiers>,
}

impl ActorWeapon {
    pub fn item(&self) -> &ItemId {
        &self.item
    }

    pub fn modifiers(&self) -> Option<&MagicItemModifiers> {
        self.modifiers.as_ref()
    }

    pub fn resolve(&self, catalog: &WeaponCatalog) -> Option<WeaponDefinition> {
        catalog.resolve_instance(
            &self.item,
            self.modifiers
                .as_ref()
                .and_then(|bonus| bonus.effect_affix()),
        )
    }
}

impl Actor {
    /// Replace the prototype attack with the actual weapon, including handling
    /// bonuses. Only slot zero is supported by this first NPC loadout.
    pub fn with_equipped_weapon(
        mut self,
        item: ItemId,
        modifiers: Option<MagicItemModifiers>,
        catalog: &WeaponCatalog,
    ) -> Result<Self, String> {
        let carried = ActorWeapon { item, modifiers };
        let weapon = carried
            .resolve(catalog)
            .ok_or("Unknown NPC weapon or effect")?;
        // NPC resource expenditure is not modelled yet: do not grant powered
        // weapons a free bypass of their energy cost.
        if weapon.power_draw().is_some() {
            return Err("Powered NPC equipment is not supported yet".into());
        }
        let attack = carried.modifiers.as_ref().map_or(weapon.attack(), |bonus| {
            bonus.modify_weapon_attack(weapon.attack())
        });
        self.attacks = vec![attack];
        self.equipped_weapon = Some(carried);
        Ok(self)
    }

    pub fn equipped_weapon(&self) -> Option<&ActorWeapon> {
        self.equipped_weapon.as_ref()
    }

    pub fn validate_equipped_weapon(&self, catalog: &WeaponCatalog) -> Result<(), String> {
        if let Some(carried) = &self.equipped_weapon {
            let weapon = carried
                .resolve(catalog)
                .ok_or("Unknown NPC weapon or effect")?;
            let attack = carried.modifiers.as_ref().map_or(weapon.attack(), |bonus| {
                bonus.modify_weapon_attack(weapon.attack())
            });
            if weapon.power_draw().is_some() || self.attacks.as_slice() != [attack] {
                return Err("NPC attacks disagree with its equipped weapon".into());
            }
        }
        Ok(())
    }

    pub(crate) fn equipment_primary_attributes(&self) -> Option<PrimaryAttributes> {
        let mut attributes = self.primary_attributes?;
        if let Some(bonus) = self
            .equipped_weapon
            .as_ref()
            .and_then(ActorWeapon::modifiers)
        {
            for attribute in PrimaryAttribute::ALL {
                attributes = attributes.with_value(
                    attribute,
                    attributes
                        .value(attribute)
                        .saturating_add(bonus.attribute_bonus(attribute)),
                );
            }
        }
        Some(attributes)
    }

    /// Called once on initial spawn, never while restoring a wounded actor.
    pub(crate) fn initialize_weapon_hit_points(&mut self, rules: Option<HitPointRules>) {
        let Some(bonus) = self
            .equipped_weapon
            .as_ref()
            .and_then(ActorWeapon::modifiers)
        else {
            return;
        };
        let maximum = match (rules, self.body_profile) {
            (Some(rules), Some(body)) => {
                rules.maximum_for_body(body, self.equipment_primary_attributes(), 0)
            }
            _ => self.maximum_integrity,
        }
        .saturating_add(bonus.maximum_hit_points_bonus());
        self.maximum_integrity = maximum;
        self.integrity = maximum;
    }
}
