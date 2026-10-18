//! Accepted WEAPON-SETS-01 row, backed by real equipment and active-set state.
use super::*;

fn hand_rects(equipment: &inventory_terminal::EquipmentLayout) -> ([Rect; 2], Rect) {
    let p = equipment.panel;
    let y = equipment.slots[4].y;
    let h = equipment.slots[4].h;
    let w = (p.w - 64.) * 0.5;
    (
        [Rect::new(p.x, y, w, h), Rect::new(p.right() - w, y, w, h)],
        Rect::new(p.center().x - 18., y + (h - 36.) * 0.5, 36., 36.),
    )
}

pub(super) fn draw_row(app: &AsciiApp, equipment: &inventory_terminal::EquipmentLayout) {
    if !app.game.rules().uses_weapon_sets() {
        return;
    }
    let first = app.game.player_active_weapon_set() * 2;
    let double = !app.active_single_hand_pair();
    let (hands, switch) = hand_rects(equipment);
    let accent = UiTheme.accent();
    for (side, rect) in hands.into_iter().enumerate() {
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            crate::terminal_chrome::ink(),
        );
        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., accent);
        draw_rectangle(rect.x + 2., rect.bottom() - 2., rect.w - 4., 2., accent);
        crate::ui_theme::draw_text_in_rect(
            if side == 0 {
                "Main droite"
            } else {
                "Main gauche"
            },
            Rect::new(rect.x + 5., rect.bottom() - 23., rect.w - 10., 20.),
            14,
            accent,
        );
        let image_side = (rect.h - 25.).min(56.);
        let image = Rect::new(
            rect.center().x - image_side * 0.5,
            rect.y + 3.,
            image_side,
            image_side,
        );
        let slot = if double {
            first
        } else if side == 0 {
            first + 1
        } else {
            first
        };
        if let Some(item) = app
            .game
            .equipped_player_weapon_item(slot)
            .and_then(|id| app.game.player_inventory().get(id))
        {
            // Both hands refer to one inventory instance. The reserved hand
            // reuses its exact sprite and palette, with only alpha changed.
            crate::game_art::draw_item_variant_opacity(
                image,
                item.item().as_str(),
                item.palette(),
                if double && side == 1 { 0.3 } else { 1. },
            );
        } else {
            draw_text_bold_centered("Vide", image, 12, UiTheme.muted());
        }
        let target = vec2(
            equipment.silhouette.x
                + (if side == 0 { 8. } else { 24. }) * equipment.silhouette.w / 32.,
            equipment.silhouette.y + 18.8 * equipment.silhouette.h / 32.,
        );
        let from = vec2(rect.center().x, rect.y - 4.);
        let route = if side == 0 {
            equipment.silhouette.x - 4.
        } else {
            equipment.silhouette.right() + 4.
        };
        draw_line(from.x, from.y, route, from.y, 1., UiTheme.muted());
        draw_line(route, from.y, route, target.y, 1., UiTheme.muted());
        draw_line(route, target.y, target.x, target.y, 1., UiTheme.muted());
    }
    if double {
        draw_line(
            hands[0].center().x,
            hands[0].y - 8.,
            hands[1].center().x,
            hands[1].y - 8.,
            1.,
            accent,
        );
        for r in hands {
            draw_line(r.center().x, r.y - 8., r.center().x, r.y - 2., 1., accent);
        }
    }
    UiTheme.button(switch, "", false, false, true, ButtonTone::Secondary);
    draw_ui_icon(
        UiIcon::Reset,
        Rect::new(switch.x + 8., switch.y + 8., 20., 20.),
        accent,
    );
    app.draw_weapon_set_badge(switch);
}

impl AsciiApp {
    pub(super) fn migrate_weapon_sets(&mut self) -> Result<(), String> {
        if self.generation_version < CRYOGENIC_CAMP_GENERATION_VERSION
            || !self.game.rules().uses_legacy_weapon_hands()
        {
            return Ok(());
        }
        let set = u8::from(self.active_weapon_slot == 2);
        if self.execute_command(GameCommand::UpgradeWeaponSets { set })
            != CommandOutcome::AppliedWithoutTime
        {
            return Err("Emplacement d’équipement enregistré invalide.".into());
        }
        // Installation writes one checkpoint after adopting its paths/settings.
        self.capture_events_at(Some(0.));
        Ok(())
    }
    pub(super) fn visible_hand_slot(&self, side: usize) -> u8 {
        if self.game.rules().uses_weapon_sets() {
            self.game.player_active_weapon_set() * 2 + side.min(1) as u8
        } else {
            side as u8
        }
    }
    pub(super) fn active_single_hand_pair(&self) -> bool {
        self.game
            .player_has_single_hand_pair(self.active_weapon_slot)
    }
    pub(super) fn draw_weapon_set_badge(&self, rect: Rect) {
        if !self.game.rules().uses_weapon_sets() {
            return;
        }
        let badge = Rect::new(rect.x + 2., rect.y + 2., 17., 13.);
        draw_rectangle(badge.x, badge.y, badge.w, badge.h, UiTheme.surface());
        draw_text_bold_centered(
            if self.game.player_active_weapon_set() == 0 {
                "I"
            } else {
                "II"
            },
            badge,
            11,
            UiTheme.accent(),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn fixture() -> AsciiApp {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.crash_recovery_enabled = false;
        app.suspension_path = std::env::temp_dir().join(format!(
            "project-rl-weapon-sets-{}-{}.json",
            std::process::id(),
            std::thread::current()
                .name()
                .unwrap_or("test")
                .replace(':', "_")
        ));
        assert!(app.game.rules().uses_weapon_sets());
        app
    }
    fn input(key: &str) -> InputFrame {
        InputFrame {
            pressed: [controls::Binding::key(key)].into(),
            viewport: Some((1280., 800.)),
            ..Default::default()
        }
    }
    fn click(at: Vec2) -> InputFrame {
        InputFrame {
            pointer: Some(at.into()),
            pressed: [controls::Binding::MouseLeft].into(),
            mouse_left_down: true,
            viewport: Some((1280., 800.)),
            ..Default::default()
        }
    }
    #[test]
    fn weapon_sets_inventory_and_bar_switch_the_same_real_set_without_spending_a_turn() {
        let mut app = fixture();
        app.begin_character_creation_with_seed(false, INITIAL_SEED)
            .unwrap();
        app.update_input(&input("Enter"));
        app.select_weapon_slot(0);
        app.history.clear();
        let turn = app.game.turn();
        let owned = app.game.player_inventory().clone();
        let before = app.game.recovery_snapshot_bytes().unwrap();
        let debug_before = format!("{:#?}", app.game);
        app.inventory_open = true;
        let layout = InventoryLayout::with_equipment(
            1280.,
            800.,
            0,
            app.inventory_entries().len(),
            app.inventory_equipped_count(),
        );
        let equipment = inventory_terminal::EquipmentLayout::new(&layout, 1280., 800.)
            .for_hands(true)
            .for_weapon_sets(true);
        app.update_input(&click(equipment.weapon_set_switch().center()));
        assert_eq!(app.game.player_active_weapon_set(), 1);
        assert_eq!(app.active_weapon_slot / 2, 1);
        assert_eq!(
            app.inventory_slot_instance(4),
            app.inventory_slot_instance(5)
        );
        app.inventory_open = false;
        app.update_input(&InputFrame::default());
        let switch = agent_skill_bar::bar_layout(
            app.agent_skill_bar_panel(false, 1280., 800.).unwrap(),
            false,
        )
        .1
        .unwrap();
        app.update_input(&click(switch.center()));
        assert_eq!(app.game.player_active_weapon_set(), 0);
        assert_eq!(app.game.turn(), turn);
        assert_eq!(app.game.player_inventory(), &owned);
        let after = app.game.recovery_snapshot_bytes().unwrap();
        let debug_after = format!("{:#?}", app.game);
        let difference = debug_before
            .lines()
            .zip(debug_after.lines())
            .enumerate()
            .find(|(_, (a, b))| a != b);
        assert!(after == before, "First changed state line: {difference:?}");
        assert_eq!(
            app.history
                .iter()
                .filter(|cmd| matches!(cmd, RecordedCommand::SelectWeaponSet { .. }))
                .count(),
            2
        );
        assert_eq!(equipment.hit(equipment.slots[4].center().into()), Some(4));
        assert_eq!(equipment.hit(equipment.slots[5].center().into()), Some(5));
        assert_eq!(
            equipment.hit(equipment.weapon_set_switch().center().into()),
            None
        );
    }
    #[test]
    fn weapon_sets_resume_and_replay_keep_the_selected_configuration_and_old_saves() {
        let mut app = fixture();
        app.cycle_bar_weapon();
        let saved = app.suspension().unwrap();
        for replay in [false, true] {
            let mut candidate = saved.clone();
            if replay {
                candidate.build = "0000000000000000".into();
            }
            let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
            let restored =
                AsciiApp::restore_suspension(&candidate, rules, texts, loot, expeditions).unwrap();
            assert_eq!(restored.game.player_active_weapon_set(), 1);
            assert_eq!(restored.active_weapon_slot, app.active_weapon_slot);
            assert_eq!(
                restored.game.recovery_snapshot_bytes().unwrap(),
                app.game.recovery_snapshot_bytes().unwrap()
            );
        }
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let old = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts,
            loot,
            expeditions,
            CRYOGENIC_CAMP_GENERATION_VERSION,
        )
        .unwrap();
        let saved = old.suspension().unwrap();
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert!(!restored.game.rules().uses_weapon_sets());
        assert_eq!(
            restored.game.recovery_snapshot_bytes().unwrap(),
            old.game.recovery_snapshot_bytes().unwrap()
        );
    }
    #[test]
    fn weapon_sets_keyboard_selects_hands_in_visible_set_and_switches_the_pair() {
        let mut app = fixture();
        app.update_input(&input("Key9"));
        assert_eq!(app.game.player_active_weapon_set(), 1);
        app.update_input(&input("Key8"));
        assert_eq!(app.game.player_active_weapon_set(), 1);
        assert_eq!(app.active_weapon_slot, 2); // Both hands refer to one two-handed weapon.
        app.update_input(&input("Key9"));
        assert_eq!(app.game.player_active_weapon_set(), 0);
    }
    #[test]
    fn weapon_sets_removed_third_equip_button_has_no_invisible_click_action() {
        let mut app = fixture();
        app.inventory_open = true;
        let heavy = app.game.equipped_player_weapon_item(2).unwrap();
        app.select_inventory_instance(heavy);
        let layout = InventoryLayout::with_equipment(
            1280.,
            800.,
            app.inventory_selection,
            app.inventory_entries().len(),
            app.inventory_equipped_count(),
        );
        let before = app.game.recovery_snapshot_bytes().unwrap();
        app.update_input(&click(layout.actions[2].center()));
        assert!(app.game.recovery_snapshot_bytes().unwrap() == before);
    }
    #[test]
    fn weapon_sets_migrate_existing_hands_once_and_replay_the_migration_after_old_commands() {
        for selected in [0, 2] {
            let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
            let mut old = AsciiApp::from_seed_version(
                INITIAL_SEED,
                rules,
                texts,
                loot,
                expeditions,
                CRYOGENIC_CAMP_GENERATION_VERSION,
            )
            .unwrap();
            old.crash_recovery_enabled = false;
            old.select_weapon_slot(selected);
            let inventory = old.game.player_inventory().clone();
            let turn = old.game.turn();
            old.migrate_weapon_sets().unwrap();
            assert_eq!(old.game.player_inventory(), &inventory);
            assert_eq!(old.game.turn(), turn);
            assert_eq!(old.game.player_active_weapon_set(), selected / 2);
            let count = old.history.len();
            old.migrate_weapon_sets().unwrap();
            assert_eq!(old.history.len(), count);
            let saved = old.suspension().unwrap();
            for replay in [false, true] {
                let mut saved = saved.clone();
                if replay {
                    saved.build = "0000000000000000".into();
                }
                let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
                let restored =
                    AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
                assert!(restored.game.rules().uses_weapon_sets());
                assert_eq!(restored.game.player_active_weapon_set(), selected / 2);
                assert_eq!(
                    restored.game.recovery_snapshot_bytes().unwrap(),
                    old.game.recovery_snapshot_bytes().unwrap()
                );
            }
        }
    }
}
