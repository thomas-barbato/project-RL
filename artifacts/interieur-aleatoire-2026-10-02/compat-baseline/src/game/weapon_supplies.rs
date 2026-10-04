//! Shared ranged supplies use an ordinary material stack, so pickups, zone
//! travel, trading and snapshots preserve it without a second hidden balance.
use super::*;
use crate::weapon::WeaponSupply;

impl GameState {
    pub fn player_matter(&self) -> Option<u32> {
        let item = self.rules.weapon_matter_item.as_ref()?;
        Some(
            self.player_inventory
                .iter()
                .filter(|entry| entry.item() == item)
                .map(|entry| u32::from(entry.quantity()))
                .sum(),
        )
    }

    pub fn weapon_supply(&self, weapon: &WeaponId) -> Option<WeaponSupply> {
        self.rules.weapon_matter_item.as_ref()?;
        self.rules.weapons.get(weapon)?.supply()
    }

    pub(super) fn ensure_weapon_matter(&self, amount: u16, uses: u16) -> Result<(), AttackError> {
        let required = u32::from(amount) * u32::from(uses);
        let available = self.player_matter().unwrap_or(0);
        if required > available {
            return Err(AttackError::InsufficientMatter {
                required,
                available,
            });
        }
        Ok(())
    }

    pub(super) fn spend_weapon_matter(
        &mut self,
        amount: u16,
        uses: u16,
    ) -> Result<(), AttackError> {
        self.ensure_weapon_matter(amount, uses)?;
        let item = self
            .rules
            .weapon_matter_item
            .as_ref()
            .expect("shared supplies enabled");
        let stacks: Vec<_> = self
            .player_inventory
            .iter()
            .filter(|entry| entry.item() == item)
            .map(|entry| (entry.instance(), entry.quantity()))
            .collect();
        let amount = u32::from(amount) * u32::from(uses);
        let mut remaining = amount;
        for (instance, quantity) in stacks {
            let consumed = remaining.min(u32::from(quantity)) as u16;
            if consumed > 0 {
                self.player_inventory
                    .remove(instance, consumed)
                    .expect("preflighted matter stack");
                remaining -= u32::from(consumed);
            }
            if remaining == 0 {
                break;
            }
        }
        self.events.push(GameEvent::MatterSpent {
            entity: self.player,
            amount,
            remaining: self.player_matter().unwrap_or(0),
        });
        Ok(())
    }

    pub(super) fn drop_actor_matter(&mut self, actor: &Actor) {
        let Some(item) = self.rules.weapon_matter_item.clone() else {
            return;
        };
        if actor.player_relation() != crate::social::PlayerRelation::Hostile {
            return;
        }
        let carries_projectiles = actor.equipped_weapon().is_some_and(|weapon| {
            matches!(
                self.weapon_supply(weapon.item()),
                Some(WeaponSupply::Matter { .. })
            )
        });
        if actor.electronic_system().is_none() && !carries_projectiles {
            return;
        }
        // First balance pass: a small, finite salvage stack, never equipment
        // from a different creature family and never a second roll on pickup.
        let quantity = self.rng.usize_inclusive(2, 6).unwrap() as u16;
        let at = actor.position();
        let ground_item = self
            .ground_items
            .spawn_material_remains(at, item.clone(), quantity)
            .expect("material remains fit the ground item ID space");
        self.events.push(GameEvent::GroundItemSpawned {
            ground_item,
            definition: item,
            quantity,
            at,
        });
    }
}
