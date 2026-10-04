//! Approved minor protection models, with separate head, hand and foot slots.
use super::*;

pub(in crate::ascii_app) const ACCESSORY_SLOTS: [&str; 3] =
    ["core:head_armor", "core:hand_armor", "core:foot_armor"];
pub(in crate::ascii_app) fn accessory_model_ids() -> Vec<ContentId> {
    ACCESSORY_MODEL_IDS
        .iter()
        .map(|id| id.parse().unwrap())
        .collect()
}
pub(super) const ACCESSORY_MODEL_IDS: [&str; 18] = [
    "core:calotte_cognefer",
    "core:heaume_bastion",
    "core:casque_cerbere",
    "core:masque_chrysalide",
    "core:capuche_de_mue",
    "core:visiere_fantome",
    "core:gants_cognefer",
    "core:gantelets_bastion",
    "core:mitaines_cerbere",
    "core:gantelets_chrysalide",
    "core:gants_de_mue",
    "core:gants_fantome",
    "core:brodequins_cognefer",
    "core:solerets_bastion",
    "core:bottes_cerbere",
    "core:bottines_chrysalide",
    "core:bottes_de_mue",
    "core:bottines_fantome",
];

#[cfg(test)]
#[path = "accessory_models_tests.rs"]
mod tests;

#[cfg(debug_assertions)]
impl AsciiApp {
    pub(in crate::ascii_app) fn prepare_accessory_equipment_diagnostic(
        &mut self,
    ) -> Result<(), String> {
        self.open_menu(MenuScreen::Main);
        self.enter_test_lab()?;
        let mut rng = GameRng::from_seed(134);
        let ids: Vec<ContentId> = [
            ACCESSORY_MODEL_IDS[5],
            ACCESSORY_MODEL_IDS[11],
            ACCESSORY_MODEL_IDS[17],
        ]
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
        let equipment = ids
            .iter()
            .map(|id| {
                self.loot
                    .equipment()
                    .enchant_base(id, 1, &mut rng)
                    .map(|bonus| (id.clone(), bonus))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut game = GameState::new_with_rules(
            self.game.map().clone(),
            self.game.player_position().ok_or("Position absente")?,
            134,
            self.game.rules().clone(),
        )
        .map_err(|e| e.to_string())?
        .with_starting_magic_equipment(equipment)?;
        let mut selected = None;
        for id in &ids {
            let item = game
                .player_inventory()
                .iter()
                .find(|entry| entry.item() == id)
                .ok_or("Objet absent")?
                .instance();
            let slot = game
                .rules()
                .items
                .get(id)
                .unwrap()
                .equipment()
                .unwrap()
                .slot()
                .clone();
            if game.process_player_command(GameCommand::EquipItem { slot, item })
                != CommandOutcome::Applied
            {
                return Err("Équipement de diagnostic impossible".into());
            }
            selected.get_or_insert(item);
        }
        game.drain_events();
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.selected_target = None;
        self.inventory_filter = InventoryFilter::Armor;
        self.select_inventory_instance(selected.unwrap());
        self.inventory_open = true;
        Ok(())
    }
}
