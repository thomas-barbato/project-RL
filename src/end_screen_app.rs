//! A finished run owns input and rendering until a menu or new profile takes over.
use super::*;

impl AsciiApp {
    pub(super) fn close_finished_run_overlays(&mut self) {
        if self.game.status() == RunStatus::Active
            || self.character_creation.is_some()
            || self.is_main_menu_flow()
        {
            return;
        }
        self.world_context = None;
        self.world_cursor = None;
        self.keyboard_target_pointer = None;
        self.mouse_walk = None;
        self.movement_repeat.clear();
        self.attack_aim = None;
        self.attack_aim_technique = None;
        self.attack_aim_pointer = None;
        self.inventory_open = false;
        self.inventory_scroll_drag = None;
        self.character_open = false;
        self.skills_open = false;
        self.level_up_notice = None;
        self.legend_open = false;
        self.report_open = false;
        self.quest_journal_open = false;
        if self.technique_menu_open {
            self.close_technique_menu();
        }
        self.component_selection = None;
        self.context_menu = None;
        self.installation_menu = None;
        self.npc_interaction = None;
        self.upgrade_confirmation = None;
        self.ux.resources_open = false;
        self.ux.inspected_target = false;
        self.ux.item_card = None;
        self.ux.lab_open = false;
        self.ux.inventory_tooltip = None;
    }

    #[cfg(debug_assertions)]
    pub(super) fn prepare_death_diagnostic(&mut self) -> Result<(), String> {
        let map =
            project_rl::world::Map::from_ascii("#####\n#...#\n#####").map_err(|e| e.to_string())?;
        // Minimal combat rules make the lethal hit deterministic. The app keeps
        // its normal content/catalogue for the subsequent menu and restart.
        let rules = GameRules {
            player_maximum_integrity: 3,
            ..GameRules::default()
        };
        let mut game = GameState::new_with_rules(map, GridPos::new(1, 1), 17, rules)
            .map_err(|e| e.to_string())?;
        game.spawn_actor(
            Actor::new(GridPos::new(2, 1), 10)
                .map_err(|e| e.to_string())?
                .with_player_relation(project_rl::social::PlayerRelation::Hostile)
                .with_attack(AttackProfile::melee(DamageType::Kinetic, 500))
                .with_ai(AiProfile::hunter(8, 0)),
        )
        .map_err(|e| e.to_string())?;
        self.terminal =
            TerminalView::new(SectorDecor::default(), game.map(), game.player_visibility());
        game.drain_events();
        self.game = WorldState::single(game);
        self.menu = MenuScreen::Hidden;
        self.character_creation = None;
        for turn in 0..20 {
            if self.game.status() != RunStatus::Active {
                break;
            }
            let outcome = self.game.process_player_command(GameCommand::Wait);
            self.capture_events_at(Some(turn as f64));
            if outcome != CommandOutcome::Applied {
                return Err(format!("Attente du diagnostic refusée : {outcome:?}"));
            }
        }
        if self.game.status() != RunStatus::PlayerDestroyed {
            return Err(format!(
                "Diagnostic de mort : état {:?}, tour {}, événements {:?}",
                self.game.status(),
                self.game.turn(),
                self.log
            ));
        }
        Ok(())
    }
}

#[cfg(all(test, debug_assertions))]
mod tests {
    use super::*;

    fn fixture() -> AsciiApp {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.suspension_path = std::env::temp_dir()
            .join(format!("project-rl-death-{}", std::process::id()))
            .join("diagnostic-run.json");
        app.prepare_death_diagnostic().unwrap();
        app
    }

    fn press(app: &mut AsciiApp, action: Action) {
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(action).clone()].into(),
            viewport: Some((1280.0, 800.0)),
            ..Default::default()
        });
    }

    #[test]
    fn death_dismisses_stale_game_panels_and_blocks_their_shortcuts() {
        let mut app = fixture();
        let before = app.game.recovery_snapshot_bytes().unwrap();
        app.inventory_open = true;
        app.character_open = true;
        app.skills_open = true;
        app.legend_open = true;
        app.technique_menu_open = true;
        app.ux.resources_open = true;
        app.ux.inspected_target = true;
        app.ux.item_card = Some(("Ancien objet".into(), vec![]));
        for action in [
            Action::Inventory,
            Action::Character,
            Action::Skills,
            Action::Legend,
            Action::QuestJournal,
            Action::Inspect,
        ] {
            press(&mut app, action);
            assert!(!app.inventory_open && !app.character_open && !app.skills_open);
            assert!(!app.legend_open && !app.technique_menu_open && !app.quest_journal_open);
            assert!(!app.ux.resources_open && !app.ux.inspected_target);
            assert!(app.ux.item_card.is_none());
            assert_eq!(
                app.menu_navigation_context(),
                Some(MenuNavigationContext::End)
            );
        }
        assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
    }

    #[test]
    fn death_history_works_with_mouse_and_keyboard_and_escape_returns_to_summary() {
        let mut app = fixture();
        let before = app.game.recovery_snapshot_bytes().unwrap();
        app.inventory_open = true;
        let rect = AsciiApp::end_actions(1280.0, 800.0)[2];
        app.update_input(&InputFrame {
            pressed: [controls::Binding::MouseLeft].into(),
            pointer: Some((rect.center().x, rect.center().y)),
            viewport: Some((1280.0, 800.0)),
            ..Default::default()
        });
        assert!(app.ux.history_open);
        app.update_input(&InputFrame {
            pause: true,
            ..Default::default()
        });
        assert!(!app.ux.history_open);
        assert_eq!(app.menu, MenuScreen::Hidden);
        press(&mut app, Action::MenuDown);
        press(&mut app, Action::Learn);
        assert!(app.ux.history_open);
        app.update_input(&InputFrame {
            pause: true,
            ..Default::default()
        });
        press(&mut app, Action::EventHistory);
        assert!(app.ux.history_open);
        assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
    }

    #[test]
    fn death_pause_menu_and_options_remain_accessible() {
        let mut app = fixture();
        let before = app.game.recovery_snapshot_bytes().unwrap();
        app.update_input(&InputFrame {
            pause: true,
            ..Default::default()
        });
        assert_eq!(app.menu, MenuScreen::Pause);
        press(&mut app, Action::MenuDown);
        press(&mut app, Action::Learn);
        assert_eq!(app.menu, MenuScreen::Options);
        app.update_input(&InputFrame {
            pause: true,
            ..Default::default()
        });
        assert_eq!(app.menu, MenuScreen::Pause);
        app.update_input(&InputFrame {
            pause: true,
            ..Default::default()
        });
        assert_eq!(app.menu, MenuScreen::Hidden);
        assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
    }

    fn click_end(app: &mut AsciiApp, index: usize) {
        let rect = AsciiApp::end_actions(1280.0, 800.0)[index];
        app.update_input(&InputFrame {
            pressed: [controls::Binding::MouseLeft].into(),
            pointer: Some((rect.center().x, rect.center().y)),
            viewport: Some((1280.0, 800.0)),
            ..Default::default()
        });
    }

    #[test]
    fn death_buttons_allow_profile_choice_and_return_to_main_menu() {
        let mut app = fixture();
        click_end(&mut app, 1);
        assert!(app.character_creation.is_some());
        let mut app = fixture();
        click_end(&mut app, 3);
        assert_eq!(app.menu, MenuScreen::Main);
    }

    #[test]
    fn death_restart_button_starts_a_new_run() {
        let mut app = fixture();
        click_end(&mut app, 0);
        assert_eq!(app.game.status(), RunStatus::Active);
        assert_eq!(app.game.turn(), 0);
        assert_eq!(app.menu, MenuScreen::Hidden);
    }
}
