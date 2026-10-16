//! The selected weapon remains the attack command; both singles form one visible set.
use super::*;

impl AsciiApp {
    pub(super) fn hand_slot_label(&self, index: usize) -> String {
        if self.game.rules().uses_weapon_hands() {
            ["Main gauche", "Main droite", "Deux mains"]
                .get(index)
                .unwrap_or(&"Arme")
                .to_string()
        } else {
            weapon_slot_label(index)
        }
    }
    pub(super) fn weapon_configuration_active(&self, index: usize) -> bool {
        if self.game.rules().uses_weapon_hands() {
            (index == 2) == (self.active_weapon_slot == 2)
        } else {
            index == usize::from(self.active_weapon_slot)
        }
    }
    pub(super) fn weapon_category_label(
        &self,
        weapon: &project_rl::weapon::WeaponDefinition,
    ) -> String {
        let delivery = match weapon.attack().delivery() {
            AttackDelivery::Melee => "Mêlée",
            AttackDelivery::Ranged => "Tir",
        };
        format!(
            "{delivery} · {}",
            self.game.rules().weapon_hands(weapon).label()
        )
    }
    pub(super) fn preferred_starting_weapon_slot(&self) -> u8 {
        self.game
            .rules()
            .player_starting_weapons
            .iter()
            .find_map(|id| {
                (0..self.game.rules().player_weapon_slots.len()).find(|&slot| {
                    self.game
                        .equipped_player_weapon(slot as u8)
                        .is_some_and(|w| w.id() == id)
                })
            })
            .unwrap_or(0) as u8
    }

    #[cfg(any(test, debug_assertions))]
    pub(super) fn equip_diagnostic_weapon(
        &mut self,
        item: ItemInstanceId,
        preferred: u8,
    ) -> CommandOutcome {
        let slot = if self.game.rules().uses_weapon_hands() {
            self.game
                .player_item_weapon(item)
                .map_or(preferred, |weapon| {
                    match self.game.rules().weapon_hands(&weapon) {
                        project_rl::game::WeaponHands::Two => 2,
                        project_rl::game::WeaponHands::One => preferred.min(1),
                    }
                })
        } else {
            preferred
        };
        if self.game.equipped_player_weapon_item(slot) == Some(item) {
            self.active_weapon_slot = slot;
            return CommandOutcome::AppliedWithoutTime;
        }
        let outcome = self.execute_command(GameCommand::EquipWeapon { slot, item });
        if matches!(
            outcome,
            CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime
        ) {
            self.active_weapon_slot = slot;
        }
        outcome
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> AsciiApp {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap()
    }
    #[test]
    fn equipment_colors_are_gated_and_carried_into_the_visible_configuration() {
        use project_rl::entity::EquipmentPalette;
        let (rules, _, _, _) = ascii_game_content().unwrap();
        let old =
            rules_for_generation_version(rules.clone(), EQUIPMENT_COLORS_GENERATION_VERSION - 1);
        assert!(!old.equipment_color_variants);
        assert!(!format!("{old:?}").contains("equipment_color_variants"));
        let current = rules_for_generation_version(rules, EQUIPMENT_COLORS_GENERATION_VERSION);
        assert!(current.equipment_color_variants);
        let mut app = fixture();
        for slot in [0, 2] {
            app.select_weapon_slot(slot);
            let crate::game_art::Visual::Player { palettes, .. } = app.front_player_visual() else {
                panic!("player figure");
            };
            let expected = app
                .game
                .equipped_player_weapon_item(slot)
                .and_then(|id| app.game.player_inventory().get(id))
                .unwrap()
                .palette();
            assert_eq!(palettes[if slot == 0 { 2 } else { 1 }], expected);
            if slot == 2 {
                assert_eq!(palettes[1], palettes[2]);
            }
            let saved = app.suspension().unwrap();
            let restored = AsciiApp::restore_suspension(
                &saved,
                app.rules.clone(),
                app.texts.clone(),
                app.loot.clone(),
                app.expeditions.clone(),
            )
            .unwrap();
            assert_eq!(restored.front_player_visual(), app.front_player_visual());
        }
        assert!(
            app.game
                .player_inventory()
                .iter()
                .any(|entry| entry.palette() != EquipmentPalette::Original)
        );
    }

    #[test]
    fn weapon_hands_core_categories_match_the_carried_figures() {
        let app = fixture();
        assert!(app.game.rules().uses_weapon_hands());
        for (id, weapon) in app.game.rules().weapons.iter() {
            let Some(index) = crate::game_art::held_weapon(id.as_str()) else {
                continue;
            };
            let hold = crate::game_art::weapon_hold(index);
            let expected = match hold {
                crate::game_art::WeaponHold::Side | crate::game_art::WeaponHold::Raised => {
                    project_rl::game::WeaponHands::One
                }
                crate::game_art::WeaponHold::TwoHanded
                | crate::game_art::WeaponHold::MeleeTwoHanded => project_rl::game::WeaponHands::Two,
            };
            assert_eq!(app.game.rules().weapon_hands(weapon), expected, "{id}");
        }
    }
    #[test]
    fn weapon_hands_switch_the_visible_set_without_unequipping_or_spending_a_turn() {
        let mut app = fixture();
        let before = app.game.recovery_snapshot_bytes().unwrap();
        assert_eq!(
            app.game.equipped_player_weapon(0).unwrap().id().as_str(),
            "core:integrity_blade"
        );
        assert!(app.game.equipped_player_weapon(2).is_some());
        app.select_weapon_slot(0);
        assert!(app.weapon_configuration_active(0) && app.weapon_configuration_active(1));
        assert!(!app.weapon_configuration_active(2));
        let crate::game_art::Visual::Player { hands, .. } = app.front_player_visual() else {
            panic!("player visual missing");
        };
        assert_eq!(
            hands,
            Some([crate::game_art::held_weapon("core:integrity_blade"), None])
        );
        app.select_weapon_slot(2);
        assert!(!app.weapon_configuration_active(0) && !app.weapon_configuration_active(1));
        assert!(app.weapon_configuration_active(2));
        assert!(matches!(
            app.front_player_visual(),
            crate::game_art::Visual::Player { hands: None, .. }
        ));
        assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
        assert_eq!(app.hand_slot_label(0), "Main gauche");
        assert_eq!(app.hand_slot_label(1), "Main droite");
        assert_eq!(app.hand_slot_label(2), "Deux mains");
    }
    #[test]
    fn weapon_hands_version_gate_preserves_old_rule_fingerprints() {
        let (rules, _, _, _) = ascii_game_content().unwrap();
        let old = rules_for_generation_version(rules.clone(), WEAPON_HANDS_GENERATION_VERSION - 1);
        assert!(!old.uses_weapon_hands());
        assert_eq!(
            old.player_weapon_slots[0].as_str(),
            "core:close_combat_channel"
        );
        assert_eq!(
            old.player_starting_equipment,
            rules.player_starting_equipment
        );
        let current = rules_for_generation_version(rules, CURRENT_GENERATION_VERSION);
        assert!(current.uses_weapon_hands());
        assert!(
            current
                .player_starting_equipment
                .iter()
                .enumerate()
                .all(|(slot, id)| id.as_ref().is_none_or(
                    |id| current.weapon_fits_slot(current.weapons.get(id).unwrap(), slot)
                ))
        );
    }

    #[test]
    fn weapon_hands_suspension_round_trip_keeps_the_active_configuration_and_legacy_resume() {
        let mut app = fixture();
        for slot in [0, 2] {
            app.select_weapon_slot(slot);
            let saved = app.suspension().unwrap();
            let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
            let restored =
                AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
            assert_eq!(restored.active_weapon_slot, slot);
            assert_eq!(
                restored.game.recovery_snapshot_bytes().unwrap(),
                app.game.recovery_snapshot_bytes().unwrap()
            );
        }
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let legacy = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            loot,
            expeditions,
            WEAPON_HANDS_GENERATION_VERSION - 1,
        )
        .unwrap();
        let saved = legacy.suspension().unwrap();
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert!(!restored.game.rules().uses_weapon_hands());
        assert_eq!(
            restored.game.recovery_snapshot_bytes().unwrap(),
            legacy.game.recovery_snapshot_bytes().unwrap()
        );
    }
}
