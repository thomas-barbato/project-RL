use super::{CommandRejection, GameState};
use crate::entity::ItemInstanceId;
use crate::item::ItemId;
use crate::loot::EquipmentLootCatalog;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EquipmentUpgradeRules {
    pub catalog: EquipmentLootCatalog,
    pub material: ItemId,
    /// Material quantity and credits, indexed by the base's tier, never the town.
    pub costs: [(u16, u32); 6],
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EquipmentUpgradeQuote {
    pub fragments: u16,
    pub credits: u32,
    pub available_fragments: u32,
}

impl GameState {
    pub fn equipment_upgrade_quote(
        &self,
        item: ItemInstanceId,
    ) -> Result<EquipmentUpgradeQuote, CommandRejection> {
        let rules = self
            .rules
            .equipment_upgrade
            .as_ref()
            .ok_or(CommandRejection::UpgradeUnavailable)?;
        let entry = self
            .player_inventory()
            .get(item)
            .ok_or(CommandRejection::UnknownInventoryItem(item))?;
        if entry
            .magic_modifiers()
            .is_some_and(|bonus| bonus.improved())
        {
            return Err(CommandRejection::EquipmentAlreadyImproved);
        }
        if entry.quantity() != 1 || entry.owner().is_some() {
            return Err(CommandRejection::UpgradeUnavailable);
        }
        let base = rules
            .catalog
            .iter()
            .find(|(id, _)| *id == entry.item())
            .map(|(_, base)| base)
            .ok_or(CommandRejection::UpgradeUnavailable)?;
        let (fragments, credits) = rules.costs[usize::from(base.tier - 1)];
        let available_fragments = self
            .player_inventory()
            .iter()
            .filter(|e| e.item() == &rules.material && e.owner().is_none())
            .map(|e| u32::from(e.quantity()))
            .sum();
        Ok(EquipmentUpgradeQuote {
            fragments,
            credits,
            available_fragments,
        })
    }

    pub(super) fn improve_player_equipment(
        &mut self,
        item: ItemInstanceId,
    ) -> Result<(), CommandRejection> {
        let quote = self.equipment_upgrade_quote(item)?;
        if quote.available_fragments < u32::from(quote.fragments) {
            return Err(CommandRejection::MissingUpgradeFragments);
        }
        let rules = self.rules.equipment_upgrade.as_ref().unwrap();
        let mut inventory = self.player_inventory().clone();
        let id = inventory.get(item).unwrap().item().clone();
        let stacks: Vec<_> = inventory
            .iter()
            .filter(|e| e.item() == &rules.material && e.owner().is_none())
            .map(|e| (e.instance(), e.quantity()))
            .collect();
        let mut remaining = quote.fragments;
        for (instance, quantity) in stacks {
            let used = remaining.min(quantity);
            if used > 0 {
                inventory
                    .remove(instance, used)
                    .map_err(|_| CommandRejection::InventoryChanged(instance))?;
            }
            remaining -= used;
        }
        let mut rng = self.rng;
        let bonus = rules
            .catalog
            .enchant_base(&id, 1, &mut rng)
            .map_err(|_| CommandRejection::UpgradeUnavailable)?
            .with_improvement_marker(true);
        inventory
            .replace_equipment_modifiers(item, bonus)
            .map_err(|_| CommandRejection::InventoryChanged(item))?;
        *self.player_inventory_mut() = inventory;
        self.rng = rng;
        self.refresh_equipment_resources();
        Ok(())
    }
}
