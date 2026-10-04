//! Mouse context actions. Only perceived cells enter this UI; commands still
//! pass through the ordinary engine and replay history.
use super::*;
use project_rl::combat::AttackPreview;
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Choice {
    Move,
    Attack,
    Interact,
    PickUp,
    Wait,
    Inspect,
    Skills,
    Technique(TechniqueId),
    LearnSkills,
    EmptySkills,
    Back,
}

impl Choice {
    fn shortcut(&self) -> Option<Action> {
        match self {
            Self::Attack => Some(Action::Attack),
            Self::Interact | Self::PickUp => Some(Action::Interact),
            Self::Inspect => Some(Action::Inspect),
            Self::Skills => Some(Action::QuickTechniques),
            Self::Wait => Some(Action::Wait),
            _ => None,
        }
    }
}

#[cfg(debug_assertions)]
impl AsciiApp {
    pub(super) fn prepare_context_route_diagnostic(&mut self, scene: &str) -> Result<(), String> {
        use project_rl::world::{DoorState, Map, Terrain};
        let mut map = Map::filled(18, 15, Terrain::Floor).map_err(|e| e.to_string())?;
        if scene.contains("door-route") {
            map.set_terrain(GridPos::new(10, 10), Terrain::Door(DoorState::Closed))
                .map_err(|e| e.to_string())?;
            map.set_terrain(GridPos::new(6, 8), Terrain::Wall)
                .map_err(|e| e.to_string())?;
        }
        let mut rules = self.game.rules().clone();
        let weapon: ContentId = "core:fusil_d_assaut".parse().unwrap();
        rules.player_starting_weapons = vec![weapon.clone()];
        rules.player_weapon_slots.truncate(1);
        rules.player_starting_equipment = vec![Some(weapon)];
        let mut game = GameState::new_with_rules(map, GridPos::new(5, 6), 129, rules)
            .map_err(|e| e.to_string())?;
        self.actor_glyphs.clear();
        if scene.contains("attack-route") || scene.contains("examine-route") {
            let target = game
                .spawn_actor(
                    Actor::new(GridPos::new(13, 6), 120)
                        .unwrap()
                        .with_evasion_disabled()
                        .with_player_relation(PlayerRelation::Hostile),
                )
                .map_err(|e| e.to_string())?;
            self.actor_glyphs.insert(target, 'X');
        }
        if scene.contains("pickup-route") {
            game.spawn_ground_item(
                GridPos::new(10, 10),
                "core:weapon_matter".parse().unwrap(),
                3,
            )
            .map_err(|e| e.to_string())?;
        }
        self.terminal = TerminalView::new(
            crate::test_sector::SectorDecor::default(),
            game.map(),
            game.player_visibility(),
        );
        self.game = WorldState::single(game);
        self.active_weapon_slot = 0;
        Ok(())
    }

    pub(super) fn verify_world_context_pointer(&mut self, scene: &str) -> Result<(), String> {
        if scene.contains("empty") {
            let ammo = self
                .game
                .player_inventory()
                .iter()
                .find(|e| e.item().as_str() == "core:weapon_matter")
                .ok_or("Munitions de diagnostic absentes")?
                .instance();
            if let CommandOutcome::Rejected(reason) =
                self.execute_command(GameCommand::DropItem { item: ammo })
            {
                return Err(format!("Dépôt de diagnostic refusé : {reason:?}"));
            }
        }
        if scene.contains("skills") && !scene.contains("unlearned") {
            let names = if scene.contains("drone") {
                ["core:rec_01", "core:drn_01"]
            } else {
                ["core:rec_01", "core:rec_02"]
            };
            for name in names {
                let technique = name.parse().map_err(|e| format!("{e}"))?;
                if let CommandOutcome::Rejected(reason) =
                    self.execute_command(GameCommand::LearnTechnique { technique })
                {
                    return Err(format!("Compétence de diagnostic refusée : {reason:?}"));
                }
            }
        }
        let at = if scene.contains("door-route") || scene.contains("pickup-route") {
            GridPos::new(10, 10)
        } else if scene.contains("attack-route") || scene.contains("examine-route") {
            GridPos::new(13, 6)
        } else if scene.contains("npc") {
            GridPos::new(5, 3)
        } else if scene.contains("ground") {
            GridPos::new(11, 11)
        } else if scene.contains("skills") {
            self.game.player_position().ok_or("Joueur absent")?
        } else {
            GridPos::new(10, 12)
        };
        let cell = self
            .terminal
            .world_cell_rect(
                &self.game,
                self.terminal_bounds(),
                self.ui_scale(),
                self.graphics.active.world_cell_px,
                self.navigation_signal_summary().is_some(),
                at,
            )
            .ok_or("Case hors caméra")?;
        let scale = self.ui_scale();
        let before = suspension::fingerprint(&self.game);
        if scene.contains("selection") {
            let pointer = Some((
                (cell.x + cell.w * 0.5) / scale,
                (cell.y + cell.h * 0.5) / scale,
            ));
            self.select_world_target(None);
            let cells = self.visible_selection_cells();
            if !cells.contains(&at) {
                return Err("Élément absent du cycle de sélection".into());
            }
            for expected in &cells {
                self.update_input(&InputFrame {
                    pressed: [self.controls.binding(Action::CycleTarget).clone()].into(),
                    pointer,
                    ..Default::default()
                });
                if self.selected_visible_cell() != Some(*expected) {
                    return Err("Cycle de sélection incomplet".into());
                }
            }
            self.select_world_target(None);
            for expected in [Some(at), None, Some(at)] {
                self.update_input(&InputFrame {
                    pressed: [controls::Binding::MouseLeft].into(),
                    pointer,
                    ..Default::default()
                });
                if self.selected_visible_cell() != expected
                    || suspension::fingerprint(&self.game) != before
                {
                    return Err("Clic de sélection incorrect ou consommant un tour".into());
                }
            }
            if self.terminal_target_summary().is_none() {
                return Err("Présentation de la cible absente".into());
            }
            return Ok(());
        }
        if scene.contains("shortcut") {
            let action = if scene.contains("attack-route") {
                Action::Attack
            } else if scene.contains("examine-route") {
                Action::Inspect
            } else {
                Action::Interact
            };
            let key = self.controls.binding(action).clone();
            // No click, no keyboard cursor, no previously selected actor.
            self.update_input_at(
                &InputFrame {
                    pressed: [key.clone()].into(),
                    held: [key.clone()].into(),
                    pointer: Some((
                        (cell.x + cell.w * 0.5) / scale,
                        (cell.y + cell.h * 0.5) / scale,
                    )),
                    viewport: Some((self.ui_width(), self.ui_height())),
                    ..Default::default()
                },
                Some(0.0),
            );
            for tick in 1..=20 {
                self.update_input_at(
                    &InputFrame {
                        held: [key.clone()].into(),
                        ..Default::default()
                    },
                    Some(tick as f64),
                );
            }
            if self.mouse_walk.is_some() || suspension::fingerprint(&self.game) == before {
                return Err("Le raccourci survolé n'a pas terminé son approche".into());
            }
            let success = if scene.contains("door-route") {
                self.game.map().tile(at).unwrap().terrain
                    == project_rl::world::Terrain::Door(project_rl::world::DoorState::Open)
            } else if scene.contains("pickup-route") {
                self.game.player_position() == Some(at)
                    && self.game.ground_items().item_at(at).is_none()
            } else if scene.contains("examine-route") {
                self.ux.inspected_target
                    && self
                        .game
                        .player_position()
                        .is_some_and(|p| at.cardinal_neighbors().contains(&p))
            } else {
                self.game.player_position() == Some(GridPos::new(6, 6))
                    && self
                        .game
                        .actors()
                        .entity_at(at)
                        .and_then(|id| self.game.actors().get(id))
                        .is_some_and(|a| a.integrity() < 120)
            };
            return if success {
                Ok(())
            } else {
                Err(format!("Action {action:?} non accomplie après approche"))
            };
        }
        self.update_input(&InputFrame {
            pressed: [controls::Binding::MouseRight].into(),
            pointer: Some((
                (cell.x + cell.w * 0.5) / scale,
                (cell.y + cell.h * 0.5) / scale,
            )),
            viewport: Some((self.ui_width(), self.ui_height())),
            ..Default::default()
        });
        let menu = self
            .world_context
            .as_ref()
            .ok_or("Le clic droit n'a pas ouvert le menu")?;
        if menu.at != at || suspension::fingerprint(&self.game) != before {
            return Err("Le menu a modifié le jeu ou ciblé une autre case".into());
        }
        if scene.contains("-route") {
            let choice = if scene.contains("door-route") {
                Choice::Interact
            } else {
                Choice::Attack
            };
            let row = menu
                .rows
                .iter()
                .position(|r| r.choice == choice)
                .ok_or("Action de trajet absente")?;
            if let Some(reason) = &menu.rows[row].blocked {
                return Err(reason.clone());
            }
            let rect = menu.row_rect(row);
            self.update_input(&InputFrame {
                pressed: [controls::Binding::MouseLeft].into(),
                pointer: Some((rect.x + 20.0, rect.y + 10.0)),
                ..Default::default()
            });
            for tick in 1..=20 {
                self.update_input_at(&InputFrame::default(), Some(tick as f64));
            }
            if scene.contains("door-route") {
                if self.game.map().tile(at).unwrap().terrain
                    != project_rl::world::Terrain::Door(project_rl::world::DoorState::Open)
                {
                    return Err(format!(
                        "Porte non ouverte, arrêt {:?}",
                        self.game.player_position()
                    ));
                }
            } else if self.game.player_position() != Some(GridPos::new(6, 6))
                || self
                    .game
                    .actors()
                    .entity_at(at)
                    .and_then(|id| self.game.actors().get(id))
                    .is_none_or(|a| a.integrity() >= 120)
            {
                return Err("Approche d'attaque incorrecte".into());
            }
        } else if scene.contains("npc-approach") {
            let index = menu
                .rows
                .iter()
                .position(|r| r.choice == Choice::Interact)
                .ok_or("Parler absent")?;
            if menu.rows[index].blocked.is_some() {
                return Err("Approche grisée".into());
            }
            let rect = menu.row_rect(index);
            let turn = self.game.turn();
            self.update_input(&InputFrame {
                pressed: [controls::Binding::MouseLeft].into(),
                pointer: Some((rect.x + 20.0, rect.y + 10.0)),
                ..Default::default()
            });
            self.update_input_at(&InputFrame::default(), Some(1.0));
            self.update_input_at(&InputFrame::default(), Some(2.0));
            if self.npc_interaction.is_none()
                || self.mouse_walk.is_some()
                || self.game.turn() != turn + 2
            {
                return Err("Le clic n'a pas enchaîné approche et dialogue".into());
            }
        } else if scene.contains("skills") {
            let index = menu
                .rows
                .iter()
                .position(|r| r.choice == Choice::Skills)
                .ok_or("Sous-menu compétences absent")?;
            let rect = menu.row_rect(index);
            self.update_input(&InputFrame {
                pressed: [controls::Binding::MouseLeft].into(),
                pointer: Some((rect.x + 20.0, rect.y + 10.0)),
                ..Default::default()
            });
            if !self.world_context.as_ref().is_some_and(|m| m.skills) {
                return Err("Le clic n'a pas ouvert les compétences".into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use project_rl::world::{Map, Terrain};

    fn app(weapon: &str) -> AsciiApp {
        app_with_skill_points(weapon, None)
    }

    fn app_with_skill_points(weapon: &str, points: Option<u16>) -> AsciiApp {
        let (mut rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        if let Some(points) = points {
            rules.progression.starting_skill_points = points;
            rules.skill_progression = rules.skill_progression.without_authored_requirements();
        }
        rules.hit_rules = None;
        rules.physical_rules = None;
        rules.player_body_profile = None;
        let id: ContentId = format!("core:{weapon}").parse().unwrap();
        rules.player_starting_weapons = vec![id.clone()];
        rules.player_weapon_slots.truncate(1);
        rules.player_starting_equipment = vec![Some(id)];
        let mut app = AsciiApp::from_seed(129, rules.clone(), texts, loot, expeditions).unwrap();
        app.game = WorldState::single(
            GameState::new_with_rules(
                Map::filled(18, 15, Terrain::Floor).unwrap(),
                GridPos::new(5, 6),
                129,
                rules,
            )
            .unwrap(),
        );
        app.menu = MenuScreen::Hidden;
        app
    }

    fn open(app: &mut AsciiApp, at: GridPos) {
        app.open_world_context(at, (950.0, 530.0), (960.0, 540.0));
        assert!(app.world_context.is_some());
    }

    fn click(app: &mut AsciiApp, choice: Choice) {
        let menu = app.world_context.as_mut().unwrap();
        let index = menu.rows.iter().position(|r| r.choice == choice).unwrap();
        menu.selected = index;
        let rect = menu.row_rect(index);
        app.update_world_context(
            &InputFrame {
                pressed: [controls::Binding::MouseLeft].into(),
                pointer: Some((rect.x + 20.0, rect.y + 10.0)),
                ..Default::default()
            },
            0.0,
        );
    }

    #[test]
    fn world_context_open_close_and_disabled_action_spend_nothing() {
        let mut app = app("lance_grenades");
        let before = suspension::fingerprint(&app.game);
        open(&mut app, GridPos::new(7, 6));
        let panel = app.world_context.as_ref().unwrap().panel();
        assert!(panel.x >= 8.0 && panel.right() <= 952.0 && panel.bottom() <= 532.0);
        app.update_world_context(
            &InputFrame {
                pressed: [controls::Binding::MouseLeft].into(),
                pointer: Some((0.0, 0.0)),
                ..Default::default()
            },
            0.0,
        );
        assert!(app.world_context.is_none());
        assert_eq!(suspension::fingerprint(&app.game), before);
        let ammo = app
            .game
            .player_inventory()
            .iter()
            .find(|e| e.item().as_str() == "core:weapon_matter")
            .unwrap()
            .instance();
        assert_eq!(
            app.execute_command(GameCommand::DropItem { item: ammo }),
            CommandOutcome::Applied
        );
        let before = suspension::fingerprint(&app.game);
        open(&mut app, GridPos::new(7, 6));
        assert!(
            app.world_context
                .as_ref()
                .unwrap()
                .rows
                .iter()
                .find(|r| r.choice == Choice::Attack)
                .unwrap()
                .blocked
                .is_some()
        );
        click(&mut app, Choice::Attack);
        assert!(app.world_context.is_some());
        assert!(app.attack_aim.is_none());
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn world_context_area_attack_previews_then_explicit_confirmation_fires() {
        let mut app = app("lance_grenades");
        let at = GridPos::new(8, 6);
        let before = suspension::fingerprint(&app.game);
        open(&mut app, at);
        click(&mut app, Choice::Attack);
        assert!(app.world_context.is_none());
        assert_eq!(app.attack_aim.unwrap().cursor, at);
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert_eq!(
            app.execute_command(GameCommand::AttackAt {
                slot: 0,
                target: at
            }),
            CommandOutcome::Applied
        );
    }

    #[test]
    fn world_context_direct_attack_hits_clicked_actor() {
        let mut app = app("fusil_d_assaut");
        let at = GridPos::new(8, 6);
        let target = app
            .game
            .spawn_actor(Actor::new(at, 100).unwrap().with_evasion_disabled())
            .unwrap();
        let turn = app.game.turn();
        open(&mut app, at);
        click(&mut app, Choice::Attack);
        assert!(app.world_context.is_none());
        assert_eq!(app.selected_target, Some(target));
        assert_eq!(app.game.turn(), turn + 1);
        assert!(app.game.actors().get(target).unwrap().integrity() < 100);
    }

    #[test]
    fn world_context_mouse_walk_completes_visible_route() {
        let mut app = app("fusil_d_assaut");
        open(&mut app, GridPos::new(8, 6));
        click(&mut app, Choice::Move);
        assert_eq!(app.game.player_position(), Some(GridPos::new(6, 6)));
        app.tick_mouse_walk(1.0);
        app.tick_mouse_walk(2.0);
        assert_eq!(app.game.player_position(), Some(GridPos::new(8, 6)));
        assert!(app.mouse_walk.is_none());
    }

    #[test]
    fn world_context_held_mouse_does_not_cancel_walk_but_new_input_does() {
        let mut app = app("fusil_d_assaut");
        app.start_mouse_walk(GridPos::new(8, 6), 0.0);
        app.update_input_at(
            &InputFrame {
                held: [controls::Binding::MouseLeft].into(),
                ..Default::default()
            },
            Some(0.05),
        );
        assert!(app.mouse_walk.is_some());
        app.update_input_at(
            &InputFrame {
                wheel_y: 1.0,
                ..Default::default()
            },
            Some(0.06),
        );
        assert!(app.mouse_walk.is_none());
        assert_eq!(app.game.player_position(), Some(GridPos::new(6, 6)));
    }

    #[test]
    fn world_context_keyboard_navigation_and_escape_are_non_destructive() {
        let mut app = app("lance_grenades");
        let before = suspension::fingerprint(&app.game);
        open(&mut app, GridPos::new(8, 6));
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::MenuDown).clone()].into(),
            ..Default::default()
        });
        assert_eq!(app.world_context.as_ref().unwrap().selected, 1);
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Learn).clone()].into(),
            ..Default::default()
        });
        assert!(app.world_context.is_none());
        assert!(app.attack_aim.is_some());
        open(&mut app, GridPos::new(8, 6));
        app.update_input(&InputFrame {
            pause: true,
            ..Default::default()
        });
        assert!(app.world_context.is_none());
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn world_context_unknown_cells_and_occupied_destinations_are_rejected() {
        let mut app = app("fusil_d_assaut");
        app.open_world_context(GridPos::new(-1, -1), (50.0, 50.0), (960.0, 540.0));
        assert!(app.world_context.is_none());
        let at = GridPos::new(8, 6);
        app.game.spawn_actor(Actor::new(at, 100).unwrap()).unwrap();
        assert!(app.context_path(at).is_none());
        assert!(
            !app.world_context_rows(at, false)
                .iter()
                .any(|r| r.choice == Choice::Move)
        );
    }

    #[test]
    fn world_context_known_hostile_allows_walk_but_new_hostile_interrupts_it() {
        let mut app = app("fusil_d_assaut");
        app.start_mouse_walk(GridPos::new(8, 6), 0.0);
        app.game
            .spawn_actor(
                Actor::new(GridPos::new(8, 7), 100)
                    .unwrap()
                    .with_player_relation(PlayerRelation::Hostile),
            )
            .unwrap();
        app.tick_mouse_walk(1.0);
        assert_eq!(app.game.player_position(), Some(GridPos::new(6, 6)));
        assert!(app.mouse_walk.is_none());
        app.start_mouse_walk(GridPos::new(9, 6), 2.0);
        assert_eq!(app.game.player_position(), Some(GridPos::new(7, 6)));
        app.tick_mouse_walk(3.0);
        app.tick_mouse_walk(4.0);
        assert_eq!(app.game.player_position(), Some(GridPos::new(9, 6)));
        assert!(app.mouse_walk.is_none());
    }

    #[test]
    fn world_context_npc_talk_approaches_then_opens_without_extra_turn() {
        let mut app = app("fusil_d_assaut");
        app.prepare_clinic_diagnostic().unwrap();
        app.npc_interaction = None;
        open(&mut app, GridPos::new(5, 3));
        let before = suspension::fingerprint(&app.game);
        click(&mut app, Choice::Interact);
        assert!(app.npc_interaction.is_some());
        assert_eq!(suspension::fingerprint(&app.game), before);
        app.npc_interaction = None;
        assert_eq!(
            app.execute_command(GameCommand::Move(Direction::West)),
            CommandOutcome::Applied
        );
        open(&mut app, GridPos::new(5, 3));
        let turn = app.game.turn();
        click(&mut app, Choice::Interact);
        assert!(app.world_context.is_none());
        assert!(app.npc_interaction.is_none());
        assert_eq!(app.game.player_position(), Some(GridPos::new(4, 3)));
        app.tick_mouse_walk(1.0);
        assert!(app.npc_interaction.is_some());
        assert!(app.mouse_walk.is_none());
        assert_eq!(app.game.turn(), turn + 1);
    }

    #[test]
    fn world_context_remote_pickup_walks_then_takes_one_action() {
        let mut app = app("fusil_d_assaut");
        let at = GridPos::new(8, 6);
        app.game
            .spawn_ground_item(at, "core:repair_patch".parse().unwrap(), 1)
            .unwrap();
        let turn = app.game.turn();
        open(&mut app, at);
        click(&mut app, Choice::PickUp);
        for tick in 1..=3 {
            app.tick_mouse_walk(tick as f64);
        }
        assert_eq!(app.game.player_position(), Some(at));
        assert!(app.game.ground_items().item_at(at).is_none());
        assert!(app.mouse_walk.is_none());
        assert_eq!(app.game.turn(), turn + 4);
        app.tick_mouse_walk(5.0);
        assert_eq!(app.game.turn(), turn + 4);
    }

    #[test]
    fn world_context_cancel_at_arrival_never_picks_up_later() {
        let mut app = app("fusil_d_assaut");
        let at = GridPos::new(6, 6);
        app.game
            .spawn_ground_item(at, "core:repair_patch".parse().unwrap(), 1)
            .unwrap();
        app.start_approach(at, true, 0.0);
        let turn = app.game.turn();
        app.update_input_at(
            &InputFrame {
                wheel_y: 1.0,
                ..Default::default()
            },
            Some(0.1),
        );
        app.tick_mouse_walk(1.0);
        assert!(app.mouse_walk.is_none());
        assert!(app.game.ground_items().item_at(at).is_some());
        assert_eq!(app.game.turn(), turn);
    }

    #[test]
    fn world_context_approach_stops_on_enemy_and_does_not_resume() {
        let mut app = app("fusil_d_assaut");
        let at = GridPos::new(8, 6);
        app.game
            .spawn_ground_item(at, "core:repair_patch".parse().unwrap(), 1)
            .unwrap();
        app.start_approach(at, true, 0.0);
        app.game
            .spawn_actor(
                Actor::new(GridPos::new(8, 7), 100)
                    .unwrap()
                    .with_player_relation(PlayerRelation::Hostile),
            )
            .unwrap();
        let turn = app.game.turn();
        app.tick_mouse_walk(1.0);
        app.tick_mouse_walk(2.0);
        assert!(app.mouse_walk.is_none());
        assert_eq!(app.game.turn(), turn);
        assert!(app.game.ground_items().item_at(at).is_some());
        assert!(app.approach_blocked(at, true).is_none());
    }

    #[test]
    fn world_context_changed_item_cancels_pending_pickup() {
        let mut app = app("fusil_d_assaut");
        let at = GridPos::new(6, 6);
        app.game
            .spawn_ground_item(at, "core:repair_patch".parse().unwrap(), 1)
            .unwrap();
        app.start_approach(at, true, 0.0);
        assert_eq!(
            app.execute_command(GameCommand::PickUp),
            CommandOutcome::Applied
        );
        app.game
            .spawn_ground_item(at, "core:repair_patch".parse().unwrap(), 1)
            .unwrap();
        let turn = app.game.turn();
        app.tick_mouse_walk(1.0);
        assert!(app.mouse_walk.is_none());
        assert!(app.game.ground_items().item_at(at).is_some());
        assert_eq!(app.game.turn(), turn);
    }

    #[test]
    fn world_context_examine_approaches_without_spending_an_inspection_turn() {
        let mut app = app("fusil_d_assaut");
        let at = GridPos::new(8, 6);
        let actor = app.game.spawn_actor(Actor::new(at, 100).unwrap()).unwrap();
        let turn = app.game.turn();
        open(&mut app, at);
        click(&mut app, Choice::Inspect);
        assert!(!app.ux.inspected_target);
        app.tick_mouse_walk(1.0);
        assert!(!app.ux.inspected_target);
        app.tick_mouse_walk(2.0);
        assert!(app.ux.inspected_target);
        assert_eq!(app.selected_target, Some(actor));
        assert_eq!(app.game.player_position(), Some(GridPos::new(7, 6)));
        assert_eq!(app.game.turn(), turn + 2);
        assert!(app.mouse_walk.is_none());
    }

    #[test]
    fn world_context_cancel_examine_does_not_open_inspector() {
        let mut app = app("fusil_d_assaut");
        let at = GridPos::new(8, 6);
        app.game.spawn_actor(Actor::new(at, 100).unwrap()).unwrap();
        open(&mut app, at);
        click(&mut app, Choice::Inspect);
        app.update_input_at(
            &InputFrame {
                wheel_y: 1.0,
                ..Default::default()
            },
            Some(0.1),
        );
        app.tick_mouse_walk(10.0);
        assert!(!app.ux.inspected_target);
        assert!(app.mouse_walk.is_none());
    }

    fn door_app(blocked: bool) -> AsciiApp {
        let mut app = app("fusil_d_assaut");
        let at = GridPos::new(8, 6);
        let mut map = app.game.map().clone();
        map.set_terrain(at, Terrain::Door(project_rl::world::DoorState::Closed))
            .unwrap();
        if blocked {
            for p in at.cardinal_neighbors() {
                map.set_terrain(p, Terrain::DeepWater).unwrap();
            }
        }
        app.game = WorldState::single(
            GameState::new_with_rules(map, GridPos::new(5, 6), 129, app.game.rules().clone())
                .unwrap(),
        );
        app
    }

    #[test]
    fn world_context_door_approach_interacts_exactly_once() {
        let mut app = door_app(false);
        let at = GridPos::new(8, 6);
        open(&mut app, at);
        let turn = app.game.turn();
        click(&mut app, Choice::Interact);
        app.tick_mouse_walk(1.0);
        assert_eq!(
            app.game.map().tile(at).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Closed)
        );
        app.tick_mouse_walk(2.0);
        app.tick_mouse_walk(3.0);
        assert_eq!(
            app.game.map().tile(at).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Open)
        );
        assert_eq!(app.game.turn(), turn + 3);
        assert_eq!(app.game.player_position(), Some(GridPos::new(7, 6)));
        assert!(app.mouse_walk.is_none());
    }

    #[test]
    fn world_context_unreachable_interaction_does_not_move() {
        let mut app = door_app(true);
        let before = suspension::fingerprint(&app.game);
        open(&mut app, GridPos::new(8, 6));
        click(&mut app, Choice::Interact);
        assert!(app.world_context.is_some());
        assert!(app.mouse_walk.is_none());
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn world_context_door_detour_tracks_both_axes_behind_a_wall() {
        let mut app = app("fusil_d_assaut");
        let door = GridPos::new(8, 9);
        let mut map = app.game.map().clone();
        map.set_terrain(door, Terrain::Door(project_rl::world::DoorState::Closed))
            .unwrap();
        map.set_terrain(GridPos::new(7, 7), Terrain::Wall).unwrap();
        app.game = WorldState::single(
            GameState::new_with_rules(map, GridPos::new(5, 6), 129, app.game.rules().clone())
                .unwrap(),
        );
        assert!(app.game.player_visibility().is_visible(door));
        let action = app.arrival_action(door, false).unwrap();
        let path = app.approach_path(&action).unwrap();
        app.start_approach_action(action, 0.0);
        for tick in 1..=20 {
            app.tick_mouse_walk(tick as f64);
        }
        assert_eq!(
            app.game.map().tile(door).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Open),
            "stopped {:?}, visible={}, planned={path:?}",
            app.game.player_position(),
            app.game.player_visibility().is_visible(door)
        );
    }

    #[test]
    fn world_context_known_door_survives_temporary_occlusion() {
        let mut app = app("fusil_d_assaut");
        let door = GridPos::new(10, 10);
        let wall = GridPos::new(6, 8);
        let mut map = Map::filled(18, 15, Terrain::Floor).unwrap();
        map.set_terrain(door, Terrain::Door(project_rl::world::DoorState::Closed))
            .unwrap();
        map.set_terrain(wall, Terrain::Wall).unwrap();
        app.game = WorldState::single(
            GameState::new_with_rules(map, GridPos::new(5, 6), 129, app.game.rules().clone())
                .unwrap(),
        );
        let action = app.arrival_action(door, false).unwrap();
        let path = app.approach_path(&action).unwrap();
        assert!(
            path.iter()
                .skip(1)
                .any(|p| !project_rl::world::has_line_of_sight(app.game.map(), *p, door, true))
        );
        app.start_approach_action(action, 0.0);
        for tick in 1..=30 {
            app.tick_mouse_walk(tick as f64);
        }
        assert_eq!(
            app.game.map().tile(door).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Open),
            "wall={wall:?}, planned={path:?}, stopped={:?}",
            app.game.player_position()
        );
    }

    #[test]
    fn world_context_walk_reaches_vertical_and_oblique_destinations() {
        for at in [
            GridPos::new(5, 3),
            GridPos::new(5, 10),
            GridPos::new(8, 9),
            GridPos::new(2, 3),
        ] {
            let mut app = app("fusil_d_assaut");
            app.start_mouse_walk(at, 0.0);
            for tick in 1..=12 {
                app.tick_mouse_walk(tick as f64);
            }
            assert_eq!(app.game.player_position(), Some(at));
            assert!(app.mouse_walk.is_none());
        }
    }

    #[test]
    fn world_context_attack_approaches_at_weapon_maximum_range_on_both_axes() {
        for (weapon, at, destination) in [
            ("fusil_d_assaut", GridPos::new(13, 6), GridPos::new(6, 6)),
            ("lance", GridPos::new(5, 10), GridPos::new(5, 8)),
            ("hache_de_combat", GridPos::new(5, 2), GridPos::new(5, 3)),
        ] {
            let mut app = app(weapon);
            let target = app
                .game
                .spawn_actor(
                    Actor::new(at, 100)
                        .unwrap()
                        .with_evasion_disabled()
                        .with_player_relation(PlayerRelation::Hostile),
                )
                .unwrap();
            let before = suspension::fingerprint(&app.game);
            let action = app.attack_arrival(at).unwrap();
            assert!(app.attack_approach_path(&action).is_ok());
            assert_eq!(suspension::fingerprint(&app.game), before);
            open(&mut app, at);
            click(&mut app, Choice::Attack);
            for tick in 1..=12 {
                app.tick_mouse_walk(tick as f64);
            }
            assert_eq!(app.game.player_position(), Some(destination), "{weapon}");
            assert!(
                app.game.actors().get(target).unwrap().integrity() < 100,
                "{weapon}"
            );
            assert!(app.mouse_walk.is_none());
        }
    }

    #[test]
    fn world_context_area_attack_approaches_then_waits_for_confirmation() {
        let mut app = app("lance_grenades");
        let at = GridPos::new(5, 14);
        let turn = app.game.turn();
        let ammo = app.game.player_matter();
        open(&mut app, at);
        click(&mut app, Choice::Attack);
        for tick in 1..=10 {
            app.tick_mouse_walk(tick as f64);
        }
        assert_eq!(app.game.player_position(), Some(GridPos::new(5, 8)));
        assert_eq!(app.attack_aim.unwrap().cursor, at);
        assert_eq!(app.game.turn(), turn + 2);
        assert_eq!(app.game.player_matter(), ammo);
    }

    #[test]
    fn world_context_attack_without_ammo_never_walks() {
        let mut app = app("fusil_d_assaut");
        let at = GridPos::new(13, 6);
        app.game.spawn_actor(Actor::new(at, 100).unwrap()).unwrap();
        let ammo = app
            .game
            .player_inventory()
            .iter()
            .find(|e| e.item().as_str() == "core:weapon_matter")
            .unwrap()
            .instance();
        app.execute_command(GameCommand::DropItem { item: ammo });
        let before = suspension::fingerprint(&app.game);
        open(&mut app, at);
        click(&mut app, Choice::Attack);
        assert!(app.world_context.is_some());
        assert!(app.mouse_walk.is_none());
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn world_context_attack_tracks_same_moving_visible_actor() {
        let mut app = app("lance");
        let at = GridPos::new(5, 12);
        let target = app
            .game
            .spawn_actor(
                Actor::new(at, 100)
                    .unwrap()
                    .with_evasion_disabled()
                    .with_player_relation(PlayerRelation::Hostile)
                    .with_ai(AiProfile::hunter(8, 0)),
            )
            .unwrap();
        open(&mut app, at);
        click(&mut app, Choice::Attack);
        for tick in 1..=12 {
            app.tick_mouse_walk(tick as f64);
        }
        let actor = app.game.actors().get(target).unwrap();
        assert_ne!(actor.position(), at);
        assert!(actor.integrity() < 100);
        assert_eq!(app.selected_target, Some(target));
        assert!(app.mouse_walk.is_none());
    }

    #[test]
    fn world_context_cancel_attack_approach_never_fires_later() {
        let mut app = app("lance");
        let at = GridPos::new(5, 12);
        let target = app.game.spawn_actor(Actor::new(at, 100).unwrap()).unwrap();
        open(&mut app, at);
        click(&mut app, Choice::Attack);
        app.update_input_at(
            &InputFrame {
                wheel_y: 1.0,
                ..Default::default()
            },
            Some(0.1),
        );
        let before = suspension::fingerprint(&app.game);
        app.tick_mouse_walk(1.0);
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert_eq!(app.game.actors().get(target).unwrap().integrity(), 100);
        assert!(app.mouse_walk.is_none());
    }

    #[test]
    fn world_context_learned_skill_stays_visible_and_targets_clicked_actor() {
        let mut app = app("fusil_d_assaut");
        let at = GridPos::new(7, 6);
        let target = app.game.spawn_actor(Actor::new(at, 100).unwrap()).unwrap();
        let id: TechniqueId = "core:rec_01".parse().unwrap();
        assert!(!app.active_learned_techniques().contains(&id));
        assert_eq!(
            app.execute_command(GameCommand::LearnTechnique {
                technique: id.clone()
            }),
            CommandOutcome::AppliedWithoutTime
        );
        let ground = GridPos::new(7, 7);
        let ground_rows = app.world_context_rows(ground, true);
        let row = ground_rows
            .iter()
            .find(|row| row.choice == Choice::Technique(id.clone()))
            .unwrap();
        assert_eq!(row.blocked.as_deref(), Some("Choisissez une cible"));
        let before = suspension::fingerprint(&app.game);
        open(&mut app, ground);
        click(&mut app, Choice::Skills);
        click(&mut app, Choice::Technique(id.clone()));
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert!(app.world_context.is_some());
        open(&mut app, at);
        click(&mut app, Choice::Skills);
        let turn = app.game.turn();
        click(&mut app, Choice::Technique(id));
        assert!(app.world_context.is_none());
        assert_eq!(app.selected_target, Some(target));
        assert_eq!(app.game.turn(), turn + 1);
    }

    #[test]
    fn world_context_drone_is_named_visible_and_can_be_summoned_from_ground() {
        let mut app = app("fusil_d_assaut");
        let id: TechniqueId = "core:drn_01".parse().unwrap();
        assert_eq!(
            app.execute_command(GameCommand::LearnTechnique {
                technique: id.clone()
            }),
            CommandOutcome::AppliedWithoutTime
        );
        for at in [GridPos::new(5, 6), GridPos::new(7, 7)] {
            let rows = app.world_context_rows(at, true);
            let row = rows
                .iter()
                .find(|row| row.choice == Choice::Technique(id.clone()))
                .unwrap();
            assert_eq!(row.label, "Invoquer un drone");
            assert_eq!(row.blocked, None);
            assert_eq!(
                rows.iter()
                    .filter(|row| matches!(row.choice, Choice::Technique(_)))
                    .count(),
                app.active_learned_techniques().len()
            );
        }
        let before = suspension::fingerprint(&app.game);
        open(&mut app, GridPos::new(7, 7));
        click(&mut app, Choice::Skills);
        assert_eq!(suspension::fingerprint(&app.game), before);
        let turn = app.game.turn();
        let energy = app.game.player_energy().available();
        click(&mut app, Choice::Technique(id.clone()));
        assert!(app.world_context.is_none());
        assert_eq!(app.game.player_companion_count(), 1);
        assert_eq!(app.game.turn(), turn + 1);
        assert!(app.game.player_energy().available() < energy);
        assert!(
            app.context_technique_blocked(&id, GridPos::new(7, 7))
                .is_some()
        );
    }

    #[test]
    fn world_context_drone_without_free_adjacent_cell_is_disabled() {
        let mut app = app("lance");
        let id: TechniqueId = "core:drn_01".parse().unwrap();
        assert_eq!(
            app.execute_command(GameCommand::LearnTechnique {
                technique: id.clone()
            }),
            CommandOutcome::AppliedWithoutTime
        );
        let at = app.game.player_position().unwrap();
        for cell in at.cardinal_neighbors() {
            app.game
                .spawn_actor(Actor::new(cell, 100).unwrap())
                .unwrap();
        }
        assert_eq!(
            app.context_technique_blocked(&id, at).as_deref(),
            Some("Aucune case libre près de vous")
        );
        let before = suspension::fingerprint(&app.game);
        open(&mut app, at);
        click(&mut app, Choice::Skills);
        click(&mut app, Choice::Technique(id));
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert_eq!(app.game.player_companion_count(), 0);
    }

    #[test]
    fn world_context_empty_skills_open_and_offer_learning_without_a_turn() {
        let mut app = app("lance");
        let before = suspension::fingerprint(&app.game);
        open(&mut app, GridPos::new(7, 6));
        click(&mut app, Choice::Skills);
        assert!(app.world_context.as_ref().unwrap().skills);
        click(&mut app, Choice::LearnSkills);
        assert!(app.skills_open);
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    fn press(app: &mut AsciiApp, action: Action) {
        app.update_input_at(
            &InputFrame {
                pressed: [app.controls.binding(action).clone()].into(),
                viewport: Some((1280.0, 800.0)),
                ..Default::default()
            },
            Some(0.0),
        );
    }

    #[test]
    fn world_context_interact_can_leave_attack_preview_and_collect_selected_ground_item() {
        let mut app = app("lance_grenades");
        let at = GridPos::new(8, 6);
        app.game
            .spawn_ground_item(at, "core:weapon_matter".parse().unwrap(), 3)
            .unwrap();
        app.attack_aim = Some(AttackAim {
            slot: 0,
            cursor: at,
        });
        press(&mut app, Action::Interact);
        assert!(app.attack_aim.is_none());
        for tick in 1..=8 {
            app.update_input_at(&InputFrame::default(), Some(tick as f64));
        }
        assert_eq!(app.game.player_position(), Some(at));
        assert!(app.game.ground_items().item_at(at).is_none());
    }

    #[test]
    fn world_context_holding_trigger_does_not_abort_approach_but_other_input_does() {
        let mut app = app("lance");
        let at = GridPos::new(10, 6);
        app.game
            .spawn_ground_item(at, "core:weapon_matter".parse().unwrap(), 3)
            .unwrap();
        app.world_cursor = Some(at);
        let key = app.controls.binding(Action::Interact).clone();
        app.update_input_at(
            &InputFrame {
                pressed: [key.clone()].into(),
                held: [key.clone()].into(),
                ..Default::default()
            },
            Some(0.0),
        );
        for tick in 1..=8 {
            app.update_input_at(
                &InputFrame {
                    held: [key.clone()].into(),
                    ..Default::default()
                },
                Some(tick as f64),
            );
        }
        assert_eq!(app.game.player_position(), Some(at));
        assert!(app.game.ground_items().item_at(at).is_none());
        let back = GridPos::new(5, 6);
        app.game
            .spawn_ground_item(back, "core:weapon_matter".parse().unwrap(), 3)
            .unwrap();
        app.world_cursor = Some(back);
        press(&mut app, Action::Interact);
        assert!(app.mouse_walk.is_some());
        app.update_input_at(
            &InputFrame {
                pause: true,
                ..Default::default()
            },
            Some(9.0),
        );
        assert!(app.mouse_walk.is_none());
        assert!(app.game.ground_items().item_at(back).is_some());
    }

    #[test]
    fn world_context_held_menu_shortcut_completes_attack_once() {
        let mut app = app("fusil_d_assaut");
        let at = GridPos::new(13, 6);
        let id = app
            .game
            .spawn_actor(Actor::new(at, 100).unwrap().with_evasion_disabled())
            .unwrap();
        open(&mut app, at);
        let key = app.controls.binding(Action::Attack).clone();
        app.update_input_at(
            &InputFrame {
                pressed: [key.clone()].into(),
                held: [key.clone()].into(),
                ..Default::default()
            },
            Some(0.0),
        );
        for tick in 1..=8 {
            app.update_input_at(
                &InputFrame {
                    held: [key.clone()].into(),
                    ..Default::default()
                },
                Some(tick as f64),
            );
        }
        assert_eq!(app.game.turn(), 2);
        assert!(app.game.actors().get(id).unwrap().integrity() < 100);
    }

    #[test]
    fn world_context_tab_exits_cell_cursor_and_keeps_target_for_attack() {
        let mut app = app("fusil_d_assaut");
        let target = app
            .game
            .spawn_actor(
                Actor::new(GridPos::new(13, 6), 100)
                    .unwrap()
                    .with_evasion_disabled(),
            )
            .unwrap();
        press(&mut app, Action::WorldActions);
        press(&mut app, Action::CycleTarget);
        assert!(app.world_cursor.is_none());
        assert_eq!(app.selected_target, Some(target));
        // A stale pointer must not be queried or replace the explicit target.
        app.update_input_at(
            &InputFrame {
                pressed: [app.controls.binding(Action::Attack).clone()].into(),
                pointer: Some((1.0, 1.0)),
                ..Default::default()
            },
            Some(0.0),
        );
        for tick in 1..=8 {
            app.tick_mouse_walk(tick as f64);
        }
        assert!(app.game.actors().get(target).unwrap().integrity() < 100);
        assert_eq!(app.selected_target, Some(target));
    }

    #[test]
    fn world_context_click_selection_exits_cursor_and_toggles_without_turn() {
        let mut app = app("lance");
        let at = GridPos::new(7, 6);
        let target = app.game.spawn_actor(Actor::new(at, 100).unwrap()).unwrap();
        let before = suspension::fingerprint(&app.game);
        press(&mut app, Action::WorldActions);
        app.select_pointer_world_cell(at, Some((1.0, 1.0)));
        assert!(app.world_cursor.is_none());
        assert_eq!(app.selected_target, Some(target));
        assert!(app.terminal_target_summary().is_some());
        app.select_pointer_world_cell(at, Some((1.0, 1.0)));
        assert!(app.selected_target.is_none());
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn world_context_selection_cycles_interactive_cells_and_clicks_toggle_them() {
        let mut app = door_app(false);
        let door = GridPos::new(8, 6);
        let loot = GridPos::new(5, 7);
        let actor_at = GridPos::new(7, 6);
        app.game
            .spawn_ground_item(loot, "core:weapon_matter".parse().unwrap(), 3)
            .unwrap();
        app.game
            .spawn_ground_item(actor_at, "core:repair_patch".parse().unwrap(), 1)
            .unwrap();
        let actor = app
            .game
            .spawn_actor(Actor::new(actor_at, 100).unwrap())
            .unwrap();
        let before = suspension::fingerprint(&app.game);
        let cells = app.visible_selection_cells();
        assert_eq!(cells, vec![loot, actor_at, door]);
        for at in cells.iter().chain(cells.iter().take(1)) {
            press(&mut app, Action::CycleTarget);
            assert_eq!(app.selected_visible_cell(), Some(*at));
            assert!(app.is_selected_target_at(*at));
            assert_eq!(app.keyboard_action_cell(&InputFrame::default()), Some(*at));
            assert!(app.terminal_target_summary().is_some());
        }
        app.select_world_target(None);
        for at in cells {
            for expected in [Some(at), None] {
                app.select_pointer_world_cell(at, None);
                assert_eq!(app.selected_visible_cell(), expected);
                assert!(app.attack_aim.is_none());
            }
        }
        assert_eq!(app.visible_targets(), vec![actor]);
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn world_context_selected_door_shortcut_approaches_without_mouse() {
        let mut app = door_app(false);
        let at = GridPos::new(8, 6);
        press(&mut app, Action::CycleTarget);
        assert_eq!(app.selected_visible_cell(), Some(at));
        let summary = app.terminal_target_summary().unwrap();
        assert_eq!(summary.name, "Porte fermée");
        assert_eq!(summary.interaction.as_deref(), Some("E : ouvrir"));
        press(&mut app, Action::Interact);
        for tick in 1..=10 {
            app.tick_mouse_walk(tick as f64);
        }
        assert_eq!(
            app.game.map().tile(at).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Open)
        );
        assert_eq!(app.selected_visible_cell(), Some(at));
        assert_eq!(app.terminal_target_summary().unwrap().name, "Porte ouverte");
    }

    #[test]
    fn world_context_selected_loot_disappears_after_keyboard_pickup() {
        let mut app = app("lance");
        let at = GridPos::new(7, 6);
        app.game
            .spawn_ground_item(at, "core:weapon_matter".parse().unwrap(), 3)
            .unwrap();
        app.select_pointer_world_cell(at, None);
        press(&mut app, Action::Interact);
        for tick in 1..=10 {
            app.tick_mouse_walk(tick as f64);
        }
        assert!(app.game.ground_items().item_at(at).is_none());
        assert!(app.selected_visible_cell().is_none());
        assert!(app.terminal_target_summary().is_none());
    }

    #[test]
    fn world_context_selection_excludes_walls_floor_player_and_hidden_elements() {
        let mut app = door_app(false);
        let mut map = app.game.map().clone();
        let wall = GridPos::new(6, 5);
        map.set_terrain(wall, Terrain::Wall).unwrap();
        let console = GridPos::new(5, 8);
        map.set_terrain(
            console,
            Terrain::ControlPanel {
                door: GridPos::new(8, 6),
                activated: false,
            },
        )
        .unwrap();
        app.game = WorldState::single(
            GameState::new_with_rules(map, GridPos::new(5, 6), 129, app.game.rules().clone())
                .unwrap(),
        );
        let hidden = GridPos::new(17, 14);
        app.game
            .spawn_ground_item(hidden, "core:weapon_matter".parse().unwrap(), 1)
            .unwrap();
        assert!(!app.game.player_visibility().is_visible(hidden));
        for at in [wall, GridPos::new(5, 6), GridPos::new(4, 6), hidden] {
            assert!(!app.selectable_world_cell(at));
            assert!(!app.toggle_pointer_target_at(at));
        }
        assert!(app.visible_selection_cells().contains(&console));
        app.select_world_target(Some(console));
        assert_eq!(
            app.terminal_target_summary().unwrap().name,
            "Console active"
        );
    }

    #[test]
    fn world_context_tab_in_area_aim_can_select_interactive_terrain() {
        let mut app = door_app(false);
        let at = GridPos::new(8, 6);
        app.attack_aim = Some(AttackAim {
            slot: 0,
            cursor: GridPos::new(5, 7),
        });
        let before = suspension::fingerprint(&app.game);
        press(&mut app, Action::CycleTarget);
        assert_eq!(app.attack_aim.unwrap().cursor, at);
        assert_eq!(app.selected_visible_cell(), Some(at));
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn world_context_service_npc_selection_displays_talk_and_opens_dialogue() {
        let mut app = app("lance");
        app.prepare_clinic_diagnostic().unwrap();
        app.npc_interaction = None;
        let at = GridPos::new(5, 3);
        let before = suspension::fingerprint(&app.game);
        press(&mut app, Action::CycleTarget);
        assert_eq!(app.selected_visible_cell(), Some(at));
        let summary = app.terminal_target_summary().unwrap();
        assert_eq!(summary.name, "Soigneur de la clinique");
        assert_eq!(summary.interaction.as_deref(), Some("E : parler"));
        assert!(summary.analysis.is_none());
        assert_eq!(suspension::fingerprint(&app.game), before);
        app.select_pointer_world_cell(at, None);
        assert!(app.selected_visible_cell().is_none());
        app.select_pointer_world_cell(at, None);
        press(&mut app, Action::Interact);
        assert_eq!(app.npc_interaction, app.game.actors().entity_at(at));
    }

    #[test]
    fn world_context_noncombat_selection_never_attacks_an_unrelated_actor() {
        let mut app = door_app(false);
        let target = app
            .game
            .spawn_actor(Actor::new(GridPos::new(6, 7), 100).unwrap())
            .unwrap();
        app.select_world_target(Some(GridPos::new(8, 6)));
        let before = suspension::fingerprint(&app.game);
        press(&mut app, Action::Attack);
        assert!(app.mouse_walk.is_none());
        assert_eq!(app.game.actors().get(target).unwrap().integrity(), 100);
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn world_context_keyboard_attack_approaches_and_fires() {
        let mut app = app("fusil_d_assaut");
        let target = app
            .game
            .spawn_actor(
                Actor::new(GridPos::new(13, 6), 100)
                    .unwrap()
                    .with_evasion_disabled(),
            )
            .unwrap();
        app.selected_target = Some(target);
        press(&mut app, Action::Attack);
        for tick in 1..=10 {
            app.tick_mouse_walk(tick as f64);
        }
        assert_eq!(app.game.player_position(), Some(GridPos::new(6, 6)));
        assert!(app.game.actors().get(target).unwrap().integrity() < 100);
    }

    #[test]
    fn world_context_shortcut_uses_menu_cell_not_previous_target() {
        let mut app = app("fusil_d_assaut");
        let other = app
            .game
            .spawn_actor(Actor::new(GridPos::new(6, 7), 100).unwrap())
            .unwrap();
        let at = GridPos::new(13, 6);
        let target = app
            .game
            .spawn_actor(Actor::new(at, 100).unwrap().with_evasion_disabled())
            .unwrap();
        app.selected_target = Some(other);
        open(&mut app, at);
        press(&mut app, Action::Attack);
        for tick in 1..=10 {
            app.tick_mouse_walk(tick as f64);
        }
        assert!(app.world_context.is_none());
        assert!(app.game.actors().get(target).unwrap().integrity() < 100);
        assert_eq!(app.game.actors().get(other).unwrap().integrity(), 100);
    }

    #[test]
    fn world_context_keyboard_door_detours_around_wall() {
        let mut app = app("lance");
        app.prepare_context_route_diagnostic("door-route").unwrap();
        press(&mut app, Action::WorldActions);
        for _ in 0..5 {
            press(&mut app, Action::MenuRight);
        }
        for _ in 0..4 {
            press(&mut app, Action::MenuDown);
        }
        assert_eq!(app.world_cursor, Some(GridPos::new(10, 10)));
        press(&mut app, Action::Interact);
        for tick in 1..=20 {
            app.tick_mouse_walk(tick as f64);
        }
        assert_eq!(
            app.game.map().tile(GridPos::new(10, 10)).unwrap().terrain,
            Terrain::Door(project_rl::world::DoorState::Open)
        );
    }

    #[test]
    fn world_context_keyboard_cursor_pickup_and_cancel_are_safe() {
        let mut app = app("lance");
        let at = GridPos::new(7, 6);
        app.game
            .spawn_ground_item(at, "core:weapon_matter".parse().unwrap(), 3)
            .unwrap();
        let before = suspension::fingerprint(&app.game);
        press(&mut app, Action::WorldActions);
        press(&mut app, Action::MenuRight);
        press(&mut app, Action::MenuRight);
        assert_eq!(app.world_cursor, Some(at));
        assert_eq!(suspension::fingerprint(&app.game), before);
        app.update_input_at(
            &InputFrame {
                pause: true,
                ..Default::default()
            },
            Some(0.0),
        );
        assert!(app.world_cursor.is_none());
        assert_eq!(suspension::fingerprint(&app.game), before);
        press(&mut app, Action::WorldActions);
        press(&mut app, Action::MenuRight);
        press(&mut app, Action::MenuRight);
        press(&mut app, Action::Interact);
        for tick in 1..=5 {
            app.tick_mouse_walk(tick as f64);
        }
        assert_eq!(app.game.player_position(), Some(at));
        assert!(app.game.ground_items().item_at(at).is_none());
    }

    #[test]
    fn world_context_keyboard_examine_approaches_before_opening() {
        let mut app = app("lance");
        let at = GridPos::new(8, 6);
        app.selected_target = Some(app.game.spawn_actor(Actor::new(at, 100).unwrap()).unwrap());
        press(&mut app, Action::Inspect);
        assert!(!app.ux.inspected_target);
        for tick in 1..=5 {
            app.tick_mouse_walk(tick as f64);
        }
        assert!(app.ux.inspected_target);
        assert_eq!(app.game.player_position(), Some(GridPos::new(7, 6)));
    }

    #[test]
    fn world_context_keyboard_grenade_still_hits_selected_actor_directly() {
        let mut app = app("lance_grenades");
        let target = app
            .game
            .spawn_actor(
                Actor::new(GridPos::new(7, 6), 100)
                    .unwrap()
                    .with_evasion_disabled(),
            )
            .unwrap();
        app.selected_target = Some(target);
        press(&mut app, Action::Attack);
        assert!(app.attack_aim.is_none());
        assert!(app.game.actors().get(target).unwrap().integrity() < 100);
    }

    #[test]
    fn world_context_all_learned_actives_remain_accessible_with_scrolling() {
        let mut app = app_with_skill_points("fusil_d_assaut", Some(200));
        let ids: Vec<_> = app
            .game
            .rules()
            .skills
            .techniques()
            .filter(|(_, definition)| definition.minimum_level() == 1)
            .map(|(id, _)| id.clone())
            .collect();
        for id in ids {
            app.execute_command(GameCommand::LearnTechnique { technique: id });
        }
        let learned = app.active_learned_techniques();
        assert!(
            learned.len() > 8,
            "Fixture must exercise several menu pages"
        );
        for at in [GridPos::new(5, 6), GridPos::new(7, 7)] {
            let ids: Vec<_> = app
                .world_context_rows(at, true)
                .into_iter()
                .filter_map(|row| match row.choice {
                    Choice::Technique(id) => Some(id),
                    _ => None,
                })
                .collect();
            assert_eq!(ids, learned);
        }
        let before = suspension::fingerprint(&app.game);
        open(&mut app, GridPos::new(7, 7));
        click(&mut app, Choice::Skills);
        for _ in 0..learned.len() {
            app.update_world_context(
                &InputFrame {
                    wheel_y: -1.0,
                    ..Default::default()
                },
                0.0,
            );
        }
        let menu = app.world_context.as_ref().unwrap();
        assert_eq!(menu.selected, menu.rows.len() - 1);
        assert!(menu.start() > 0);
        assert!(menu.panel().contains(menu.row_rect(menu.selected).center()));
        for _ in 0..learned.len() {
            app.update_world_context(
                &InputFrame {
                    pressed: [controls::Binding::key("Up")].into(),
                    ..Default::default()
                },
                0.0,
            );
        }
        assert_eq!(app.world_context.as_ref().unwrap().selected, 0);
        assert_eq!(suspension::fingerprint(&app.game), before);
    }
}

#[derive(Clone)]
struct Row {
    label: String,
    blocked: Option<String>,
    choice: Choice,
}

pub(super) struct WorldContextMenu {
    pub(super) at: GridPos,
    anchor: (f32, f32),
    viewport: (f32, f32),
    skills: bool,
    selected: usize,
    rows: Vec<Row>,
    message: String,
}

pub(super) struct MouseWalk {
    pub(super) trigger_key: Option<controls::Binding>,
    cells: VecDeque<GridPos>,
    expected: GridPos,
    next_at: f64,
    arrival: Option<ArrivalAction>,
    known_hostiles: std::collections::BTreeSet<EntityId>,
    remaining_steps: usize,
}

#[derive(Clone)]
enum ArrivalAction {
    Attack {
        at: GridPos,
        actor: Option<EntityId>,
        slot: u8,
        weapon: ItemInstanceId,
    },
    Inspect {
        at: GridPos,
        actor: EntityId,
    },
    PickUp {
        at: GridPos,
        item: project_rl::entity::GroundItemId,
    },
    Interact {
        at: GridPos,
        actor: Option<EntityId>,
        terrain: project_rl::world::Terrain,
    },
}

impl ArrivalAction {
    fn at(&self) -> GridPos {
        match self {
            Self::PickUp { at, .. }
            | Self::Interact { at, .. }
            | Self::Inspect { at, .. }
            | Self::Attack { at, .. } => *at,
        }
    }
}

impl WorldContextMenu {
    fn row_step(&self) -> f32 {
        if self.rows.iter().any(|r| r.blocked.is_some()) {
            50.0
        } else {
            38.0
        }
    }
    fn page_size(&self) -> usize {
        (((self.viewport.1 - 122.0) / self.row_step()) as usize).clamp(1, 8)
    }
    fn start(&self) -> usize {
        self.selected / self.page_size() * self.page_size()
    }
    fn panel(&self) -> Rect {
        let width = 320.0_f32.min(self.viewport.0 - 16.0).max(1.0);
        let height = (102.0 + self.rows.len().min(self.page_size()) as f32 * self.row_step())
            .min(self.viewport.1 - 16.0);
        let x = if self.anchor.0 + 12.0 + width <= self.viewport.0 - 8.0 {
            self.anchor.0 + 12.0
        } else {
            self.anchor.0 - width - 12.0
        };
        Rect::new(
            x.clamp(8.0, (self.viewport.0 - width - 8.0).max(8.0)),
            (self.anchor.1 + 10.0).clamp(8.0, (self.viewport.1 - height - 8.0).max(8.0)),
            width,
            height,
        )
    }
    fn row_rect(&self, index: usize) -> Rect {
        let panel = self.panel();
        Rect::new(
            panel.x + 8.0,
            panel.y + 64.0 + (index - self.start()) as f32 * self.row_step(),
            panel.w - 16.0,
            self.row_step() - 4.0,
        )
    }
}

fn world_aim(action: TechniqueAction) -> bool {
    matches!(
        action,
        TechniqueAction::WeaponAttack {
            melee_arc: Some(_),
            ..
        } | TechniqueAction::PrepareRangedOverwatch { .. }
            | TechniqueAction::WeaponBarrage { .. }
    ) || action.is_explosive_action()
        || action.is_movement_aim_action()
        || action.is_stealth_world_aim_action()
}

fn actor_technique(action: TechniqueAction) -> bool {
    matches!(
        action,
        TechniqueAction::AnalyzeTarget { .. }
            | TechniqueAction::AnalyzeThreat { .. }
            | TechniqueAction::AnalyzeMultipleTargets { .. }
            | TechniqueAction::DiagnoseEnergy { .. }
            | TechniqueAction::WeaponAttack { .. }
            | TechniqueAction::WeaponVolley { .. }
            | TechniqueAction::WeaponComponentAttack { .. }
            | TechniqueAction::AmbushAttack { .. }
            | TechniqueAction::ChargeAttack { .. }
            | TechniqueAction::Breakthrough { .. }
            | TechniqueAction::ExtractAlly { .. }
    )
}

impl AsciiApp {
    pub(super) fn context_npc_at(&self, at: GridPos) -> bool {
        self.game.actors().entity_at(at).is_some_and(|id| {
            self.game.active_worker_role(id).is_some()
                || self.game.active_merchant(id)
                || self.game.active_artisan(id)
                || self.game.active_clinic(id)
                || self.game.active_resident(id)
                || self.game.active_quest_provider(id)
                || self.game.dialogue_view(id).is_some()
                || self.game.npc_interaction(id).is_some()
        })
    }

    fn context_interactive_at(&self, at: GridPos) -> bool {
        self.context_npc_at(at)
            || self.game.passage(at).is_some()
            || self
                .game
                .map()
                .tile(at)
                .is_some_and(|t| t.terrain.is_interactive())
            || self
                .game
                .active_facility()
                .is_some_and(|f| f.is_player_interactive_at(at))
            || self
                .game
                .threat_sources()
                .iter()
                .any(|s| s.position() == at && s.is_active())
    }

    pub(super) fn selectable_world_cell(&self, at: GridPos) -> bool {
        self.game.player_visibility().is_visible(at)
            && (self
                .game
                .actors()
                .entity_at(at)
                .is_some_and(|id| id != self.game.player_id())
                || self.game.ground_items().item_at(at).is_some()
                || self.context_interactive_at(at))
    }

    pub(super) fn visible_selection_cells(&self) -> Vec<GridPos> {
        let Some(origin) = self.game.player_position() else {
            return Vec::new();
        };
        let mut cells: Vec<_> = self
            .game
            .player_visibility()
            .visible_positions()
            .filter(|at| self.selectable_world_cell(*at))
            .collect();
        cells.sort_unstable_by_key(|at| (grid_distance(origin, *at), at.y, at.x));
        cells
    }

    pub(super) fn select_world_target(&mut self, at: Option<GridPos>) {
        let at = at.filter(|at| self.selectable_world_cell(*at));
        self.selected_target = at
            .and_then(|at| self.game.actors().entity_at(at))
            .filter(|id| *id != self.game.player_id());
        self.selected_world_cell = at.filter(|_| self.selected_target.is_none());
    }

    pub(super) fn selected_visible_cell(&self) -> Option<GridPos> {
        let at = if let Some(id) = self.selected_target {
            self.game.actors().get(id)?.position()
        } else {
            self.selected_world_cell
                .filter(|at| self.selectable_world_cell(*at))?
        };
        self.game.player_visibility().is_visible(at).then_some(at)
    }

    pub(super) fn keyboard_action_cell(&self, input: &InputFrame) -> Option<GridPos> {
        // Tab owns the selected element until the mouse actually moves again.
        if input.pointer != self.keyboard_target_pointer
            && let Some(at) = self.attack_pointer_cell(input)
        {
            return Some(at);
        }
        self.selected_visible_cell()
            .or_else(|| self.attack_aim.map(|aim| aim.cursor))
    }

    pub(super) fn run_keyboard_world_action(&mut self, action: Action, at: GridPos, now: f64) {
        if !self.keyboard_world_action(action, at, now) {
            self.push_log("Aucune action correspondante sur cette case.".into());
        }
        if let Some(walk) = self.mouse_walk.as_mut() {
            walk.trigger_key = Some(self.controls.binding(action).clone());
        }
    }

    pub(super) fn keyboard_world_action(&mut self, action: Action, at: GridPos, now: f64) -> bool {
        if self.game.player_technique_preparation().is_some() {
            return false;
        }
        let choice = match action {
            Action::Attack => Choice::Attack,
            Action::Interact if self.game.ground_items().item_at(at).is_some() => Choice::PickUp,
            Action::Interact => Choice::Interact,
            Action::Inspect => Choice::Inspect,
            _ => return false,
        };
        let Some(row) = self
            .world_context_rows(at, false)
            .into_iter()
            .find(|row| row.choice == choice)
        else {
            return false;
        };
        if let Some(reason) = row.blocked {
            self.push_log(reason);
            return true;
        }
        let arrival = match action {
            Action::Attack => self.attack_arrival(at),
            Action::Interact => {
                self.arrival_action(at, self.game.ground_items().item_at(at).is_some())
            }
            Action::Inspect => self
                .game
                .actors()
                .entity_at(at)
                .map(|actor| ArrivalAction::Inspect { at, actor }),
            _ => None,
        };
        let Some(arrival) = arrival else {
            return false;
        };
        if let Some(reason) = self.approach_action_blocked(&arrival) {
            self.push_log(reason);
        } else {
            self.world_cursor = None;
            self.attack_aim = None;
            self.attack_aim_technique = None;
            self.attack_aim_pointer = None;
            self.start_approach_action(arrival, now);
        }
        true
    }

    pub(super) fn update_world_cursor(&mut self, input: &InputFrame, now: f64) {
        let Some(mut at) = self.world_cursor else {
            return;
        };
        if input.pause || self.controls.pressed(Action::WorldActions, input) {
            self.world_cursor = None;
            return;
        }
        if self.controls.pressed(Action::CycleTarget, input) {
            self.world_cursor = None;
            self.cycle_target();
            self.keyboard_target_pointer = input.pointer;
            return;
        }
        if input.pressed.contains(&controls::Binding::MouseLeft)
            && let Some(cell) = self
                .attack_pointer_cell(input)
                .filter(|cell| self.game.player_visibility().is_visible(*cell))
        {
            self.select_pointer_world_cell(cell, input.pointer);
            return;
        }
        for (action, direction) in [
            (Action::MenuUp, Direction::North),
            (Action::MenuDown, Direction::South),
            (Action::MenuLeft, Direction::West),
            (Action::MenuRight, Direction::East),
        ] {
            if self.controls.pressed(action, input) {
                let next = at.step(direction);
                if self.game.player_visibility().is_visible(next) {
                    at = next;
                }
            }
        }
        self.world_cursor = Some(at);
        for action in [Action::Attack, Action::Interact, Action::Inspect] {
            if self.controls.pressed(action, input) {
                self.run_keyboard_world_action(action, at, now);
                return;
            }
        }
        if self.controls.pressed(Action::Learn, input) {
            let viewport = input.viewport.unwrap_or((1280.0, 800.0));
            // Use the same world-to-screen transform as mouse hit testing.
            let scale = self.ui_scale();
            let anchor = self
                .terminal
                .world_cell_rect(
                    &self.game,
                    self.terminal_bounds(),
                    scale,
                    self.graphics.active.world_cell_px,
                    self.navigation_signal_summary().is_some(),
                    at,
                )
                .map_or((viewport.0 * 0.5, viewport.1 * 0.5), |r| {
                    (r.right() / scale, r.y / scale)
                });
            self.world_cursor = None;
            self.open_world_context(at, anchor, viewport);
        }
    }

    pub(super) fn draw_world_cursor(&self) {
        let Some(at) = self.world_cursor else {
            return;
        };
        let scale = self.ui_scale();
        if let Some(r) = self.terminal.world_cell_rect(
            &self.game,
            self.terminal_bounds(),
            scale,
            self.graphics.active.world_cell_px,
            self.navigation_signal_summary().is_some(),
            at,
        ) {
            draw_rectangle_lines(
                r.x / scale - 2.0,
                r.y / scale - 2.0,
                r.w / scale + 4.0,
                r.h / scale + 4.0,
                2.0,
                UiTheme.accent(),
            );
        }
        let rect = Rect::new(20.0, self.ui_height() - 104.0, self.ui_width() - 40.0, 38.0);
        UiTheme.context_surface(rect);
        draw_wrapped_text(
            &format!(
                "Flèches : choisir une case · {} : actions · {} : attaquer · {} : interagir · Échap : annuler",
                self.controls.label(Action::Learn),
                self.controls.label(Action::Attack),
                self.controls.label(Action::Interact)
            ),
            rect.x + 14.0,
            rect.y + 25.0,
            rect.w - 28.0,
            1,
            16,
            UiTheme.text(),
        );
    }

    fn context_visible_hostiles(&self) -> std::collections::BTreeSet<EntityId> {
        self.game
            .actors()
            .iter()
            .filter_map(|(id, actor)| {
                (id != self.game.player_id()
                    && actor.is_alive()
                    && actor.player_relation() == PlayerRelation::Hostile
                    && self.game.player_visibility().is_visible(actor.position()))
                .then_some(id)
            })
            .collect()
    }

    fn attack_arrival(&self, at: GridPos) -> Option<ArrivalAction> {
        Some(ArrivalAction::Attack {
            at,
            actor: self.game.actors().entity_at(at),
            slot: self.active_weapon_slot,
            weapon: self
                .game
                .equipped_player_weapon_item(self.active_weapon_slot)?,
        })
    }

    fn context_attack_needs_aim(&self, slot: u8, actor: Option<EntityId>) -> bool {
        self.game
            .resolved_equipped_player_weapon(slot)
            .is_some_and(|weapon| {
                let direct_grenade = actor.is_some()
                    && weapon
                        .launcher()
                        .is_some_and(|launcher| launcher.delay_turns > 0)
                    && !weapon.effects().iter().any(|effect| {
                        matches!(
                            effect.kind(),
                            project_rl::weapon::WeaponEffectKind::CatalyticCone { .. }
                        )
                    });
                weapon.requires_area_aim() && !direct_grenade
            })
    }

    fn approach_attack_preview(
        &self,
        action: &ArrivalAction,
        origin: GridPos,
    ) -> Result<AttackPreview, CommandRejection> {
        let ArrivalAction::Attack {
            at, actor, slot, ..
        } = action
        else {
            unreachable!()
        };
        let area = self.context_attack_needs_aim(*slot, *actor);
        self.game.player_attack_approach_preview(
            *slot,
            origin,
            *at,
            if area { None } else { *actor },
        )
    }

    fn attack_approach_path(&self, action: &ArrivalAction) -> Result<Vec<GridPos>, String> {
        let origin = self.game.player_position().ok_or("Personnage absent")?;
        let spatial = |e: &CommandRejection| {
            matches!(
                e,
                CommandRejection::TargetOutOfRange(_)
                    | CommandRejection::AttackTargetOutOfRange(_)
                    | CommandRejection::NoLineOfSight(_)
                    | CommandRejection::AttackNoLineOfSight(_)
            )
        };
        match self.approach_attack_preview(action, origin) {
            Ok(_) => return Ok(vec![origin]),
            Err(e) if !spatial(&e) => return Err(attack_preview_rejection_label(&e).into()),
            _ => {}
        }
        // A* searches actual routes; Manhattan distance only orders candidate
        // firing positions and prunes those unable to beat the shortest route.
        let target = action.at();
        let mut candidates = Vec::new();
        for y in 0..self.game.map().height() {
            for x in 0..self.game.map().width() {
                let at = GridPos::new(x as i32, y as i32);
                if at == target || !self.context_safe_cell(at) {
                    continue;
                }
                match self.approach_attack_preview(action, at) {
                    Ok(_) => {
                        let steps = origin.x.abs_diff(at.x) + origin.y.abs_diff(at.y);
                        let dx = i64::from(target.x) - i64::from(at.x);
                        let dy = i64::from(target.y) - i64::from(at.y);
                        candidates.push((steps, std::cmp::Reverse(dx * dx + dy * dy), at));
                    }
                    Err(e) if spatial(&e) || matches!(e, CommandRejection::ProtectedZone) => {}
                    Err(e) => return Err(attack_preview_rejection_label(&e).into()),
                }
            }
        }
        candidates.sort();
        let mut best: Option<(Vec<GridPos>, i64)> = None;
        for (minimum, std::cmp::Reverse(distance), at) in candidates {
            if best
                .as_ref()
                .is_some_and(|(p, _)| minimum as usize > p.len() - 1)
            {
                break;
            }
            if let Some(path) = self.context_path(at)
                && best.as_ref().is_none_or(|(p, d)| {
                    path.len() < p.len() || path.len() == p.len() && distance > *d
                })
            {
                best = Some((path, distance));
            }
        }
        best.map(|(p, _)| p)
            .ok_or_else(|| "Aucune position de tir accessible".into())
    }

    fn context_safe_cell(&self, at: GridPos) -> bool {
        self.game.player_visibility().is_visible(at)
            && self.game.map().is_walkable(at)
            && self.game.actors().entity_at(at).is_none()
            && self.game.ground_effects().at(at).next().is_none()
            && !self
                .game
                .explosive_devices()
                .at(at)
                .any(|d| d.is_identified() && !d.is_neutralized())
    }

    fn context_path(&self, at: GridPos) -> Option<Vec<GridPos>> {
        let origin = self.game.player_position()?;
        if !self.context_safe_cell(at) {
            return None;
        }
        let path = project_rl::world::find_path(self.game.map(), origin, at, 4096, |p| {
            self.context_safe_cell(p)
        })?;
        path.iter()
            .skip(1)
            .all(|p| self.context_safe_cell(*p))
            .then_some(path)
    }

    fn arrival_action(&self, at: GridPos, pickup: bool) -> Option<ArrivalAction> {
        if pickup {
            Some(ArrivalAction::PickUp {
                at,
                item: self.game.ground_items().item_at(at)?,
            })
        } else {
            Some(ArrivalAction::Interact {
                at,
                actor: self.game.actors().entity_at(at),
                terrain: self.game.map().tile(at)?.terrain,
            })
        }
    }

    fn arrival_valid(&self, action: &ArrivalAction) -> bool {
        if !self.game.player_visibility().is_visible(action.at()) {
            // A known, stationary goal may disappear behind a wall during a
            // detour. Do not query unseen changes; revalidate on arrival.
            return self.game.player_visibility().is_explored(action.at())
                && matches!(
                    action,
                    ArrivalAction::PickUp { .. } | ArrivalAction::Interact { actor: None, .. }
                );
        }
        match action {
            ArrivalAction::Attack {
                at,
                actor,
                slot,
                weapon,
            } => {
                self.active_weapon_slot == *slot
                    && self.game.equipped_player_weapon_item(*slot) == Some(*weapon)
                    && actor.is_none_or(|id| {
                        self.game
                            .actors()
                            .get(id)
                            .is_some_and(|a| a.is_alive() && a.position() == *at)
                    })
            }
            ArrivalAction::Inspect { at, actor } => {
                self.game.actors().entity_at(*at) == Some(*actor)
            }
            ArrivalAction::PickUp { at, item } => {
                self.game.ground_items().item_at(*at) == Some(*item)
            }
            ArrivalAction::Interact { at, actor, terrain } => {
                self.game.actors().entity_at(*at) == *actor
                    && self
                        .game
                        .map()
                        .tile(*at)
                        .is_some_and(|tile| tile.terrain == *terrain)
            }
        }
    }

    fn arrival_ready(&self, action: &ArrivalAction) -> bool {
        match action {
            ArrivalAction::Attack { .. } => self
                .game
                .player_position()
                .is_some_and(|p| self.approach_attack_preview(action, p).is_ok()),
            ArrivalAction::Inspect { at, .. } => self
                .game
                .player_position()
                .is_some_and(|p| p == *at || p.cardinal_neighbors().contains(at)),
            ArrivalAction::PickUp { at, .. } => self.game.player_position() == Some(*at),
            ArrivalAction::Interact { at, .. } => self.interaction_candidates().contains(at),
        }
    }

    fn approach_path(&self, action: &ArrivalAction) -> Option<Vec<GridPos>> {
        if !self.arrival_valid(action) {
            return None;
        }
        if self.arrival_ready(action) {
            return Some(vec![self.game.player_position()?]);
        }
        match action {
            ArrivalAction::Attack { .. } => self.attack_approach_path(action).ok(),
            ArrivalAction::PickUp { at, .. } => self.context_path(*at),
            ArrivalAction::Interact { at, .. } | ArrivalAction::Inspect { at, .. } => at
                .cardinal_neighbors()
                .into_iter()
                .filter_map(|p| self.context_path(p))
                .min_by_key(Vec::len),
        }
    }

    fn approach_blocked(&self, at: GridPos, pickup: bool) -> Option<String> {
        let Some(action) = self.arrival_action(at, pickup) else {
            return Some("Action indisponible".into());
        };
        self.approach_action_blocked(&action)
    }

    fn approach_action_blocked(&self, action: &ArrivalAction) -> Option<String> {
        if matches!(action, ArrivalAction::Attack { .. }) {
            return self.attack_approach_path(action).err();
        }
        if self.arrival_ready(action) {
            return None;
        }
        self.approach_path(action)
            .is_none()
            .then(|| "Aucun chemin visible sûr".into())
    }

    fn finish_arrival(&mut self, action: ArrivalAction) {
        if !self.arrival_valid(&action) || !self.arrival_ready(&action) {
            self.push_log("Approche annulée : la cible a changé.".into());
            return;
        }
        let command = match action {
            ArrivalAction::Attack {
                at, actor, slot, ..
            } => {
                self.selected_world_cell = None;
                self.selected_target = actor;
                if self.context_attack_needs_aim(slot, actor) {
                    self.begin_attack_aim_at(Some(at), None, None);
                    None
                } else {
                    actor.map(|target| GameCommand::Attack { slot, target })
                }
            }
            ArrivalAction::Inspect { actor, .. } => {
                self.selected_world_cell = None;
                self.selected_target = (actor != self.game.player_id()).then_some(actor);
                self.ux.inspected_target = true;
                self.ux.help_scroll.offset = 0.0;
                None
            }
            ArrivalAction::PickUp { .. } => Some(GameCommand::PickUp),
            ArrivalAction::Interact { at, .. } => self.interaction_command_at(at),
        };
        if let Some(command) = command {
            if let CommandOutcome::Rejected(reason) = self.execute_command(command) {
                self.push_log(command_rejection_message(reason).to_owned());
            }
            self.capture_events();
        }
    }

    fn start_approach(&mut self, at: GridPos, pickup: bool, now: f64) {
        let Some(action) = self.arrival_action(at, pickup) else {
            return;
        };
        self.start_approach_action(action, now);
    }

    fn start_approach_action(&mut self, action: ArrivalAction, now: f64) {
        if self.arrival_ready(&action) {
            self.finish_arrival(action);
            return;
        }
        if self.approach_action_blocked(&action).is_some() {
            return;
        }
        let Some(path) = self.approach_path(&action) else {
            return;
        };
        self.mouse_walk = Some(MouseWalk {
            trigger_key: None,
            expected: self.game.player_position().unwrap(),
            remaining_steps: path.len().saturating_add(8).min(128),
            cells: path.into_iter().skip(1).collect(),
            next_at: now,
            arrival: Some(action),
            known_hostiles: self.context_visible_hostiles(),
        });
        self.step_mouse_walk(now);
    }

    fn context_technique_blocked(&self, id: &TechniqueId, at: GridPos) -> Option<String> {
        if let Some(reason) = self.quick_technique_status(id).1 {
            return Some(reason);
        }
        let action = self.game.rules().skills.technique(id)?.action()?;
        if matches!(action, TechniqueAction::ManifestDrone { .. }) {
            let origin = self.game.player_position()?;
            return (!origin.cardinal_neighbors().into_iter().any(|cell| {
                self.game.map().is_walkable(cell) && self.game.actors().entity_at(cell).is_none()
            }))
            .then(|| "Aucune case libre près de vous".into());
        }
        if world_aim(action) {
            return self
                .game
                .player_weapon_technique_preview(id, self.active_weapon_slot, at)
                .err()
                .map(|e| attack_preview_rejection_label(&e).to_owned());
        }
        if actor_technique(action) {
            let Some(entity) = self
                .game
                .actors()
                .entity_at(at)
                .filter(|id| *id != self.game.player_id())
            else {
                return Some("Choisissez une cible".into());
            };
            let ordinary = matches!(
                action,
                TechniqueAction::AnalyzeTarget { .. }
                    | TechniqueAction::AnalyzeThreat { .. }
                    | TechniqueAction::AnalyzeMultipleTargets { .. }
                    | TechniqueAction::DiagnoseEnergy { .. }
                    | TechniqueAction::ExtractAlly { .. }
            );
            let candidates = if ordinary {
                self.game.player_technique_targets(id)
            } else {
                self.game
                    .player_weapon_technique_targets(id, self.active_weapon_slot)
            };
            if !candidates.contains(&entity) {
                return Some("Cible incompatible ou hors de portée".into());
            }
        }
        // These adapters configure orders, components or nearby devices using
        // player-centred defaults, not the clicked cell. Keep them accessible
        // from the character, without silently acting on a different target.
        if (action.is_drone_action()
            || action.is_engineering_action()
            || action.is_intrusion_action()
            || action.is_electronic_warfare_action())
            && !matches!(action, TechniqueAction::MaintainJamming { .. })
            && Some(at) != self.game.player_position()
        {
            return Some("À utiliser depuis votre personnage".into());
        }
        if action.is_drone_action() && self.game.player_controlled_companions().is_empty() {
            return Some("Aucun drone contrôlé".into());
        }
        None
    }

    fn world_context_rows(&self, at: GridPos, skills: bool) -> Vec<Row> {
        if !self.game.player_visibility().is_visible(at) {
            return Vec::new();
        }
        let mut rows = Vec::new();
        let mut add = |choice, label: String, blocked| {
            rows.push(Row {
                choice,
                label,
                blocked,
            })
        };
        if skills {
            add(Choice::Back, "Retour".into(), None);
            if self.active_learned_techniques().is_empty() {
                add(
                    Choice::EmptySkills,
                    "Aucune compétence apprise".into(),
                    Some("Choisissez votre première compétence".into()),
                );
                add(Choice::LearnSkills, "Apprendre une compétence".into(), None);
            }
            // Context changes availability, never which learned skills exist.
            for id in self.active_learned_techniques() {
                add(
                    Choice::Technique(id.clone()),
                    self.technique_name(&id),
                    self.context_technique_blocked(&id, at),
                );
            }
            return rows;
        }
        let own = Some(at) == self.game.player_position();
        let entity = self.game.actors().entity_at(at);
        if !own && entity.is_none() && self.game.map().is_walkable(at) {
            add(
                Choice::Move,
                "Se déplacer ici".into(),
                self.context_path(at)
                    .is_none()
                    .then(|| "Aucun chemin visible sûr".into()),
            );
        }
        if own {
            add(Choice::Wait, "Attendre un tour".into(), None);
        }
        if self.game.ground_items().item_at(at).is_some() {
            add(
                Choice::PickUp,
                "Ramasser".into(),
                self.approach_blocked(at, true),
            );
        }
        let npc = self.context_npc_at(at);
        let interactive = self.context_interactive_at(at);
        if interactive {
            add(
                Choice::Interact,
                if npc {
                    "Parler"
                } else if self.game.passage(at).is_some() {
                    "Emprunter le passage"
                } else {
                    "Interagir"
                }
                .into(),
                self.approach_blocked(at, false),
            );
        }
        if !own
            && !npc
            && let Some(weapon) = self
                .game
                .resolved_equipped_player_weapon(self.active_weapon_slot)
            && (entity.is_some() || weapon.requires_area_aim())
        {
            add(
                Choice::Attack,
                "Attaquer".into(),
                self.attack_arrival(at)
                    .and_then(|action| self.approach_action_blocked(&action)),
            );
        }
        add(Choice::Skills, "Compétences".into(), None);
        if let Some(actor) = entity {
            add(
                Choice::Inspect,
                "Examiner".into(),
                self.approach_action_blocked(&ArrivalAction::Inspect { at, actor }),
            );
        }
        rows
    }

    pub(super) fn open_world_context(
        &mut self,
        at: GridPos,
        anchor: (f32, f32),
        viewport: (f32, f32),
    ) {
        if self.game.status() != RunStatus::Active || !self.game.player_visibility().is_visible(at)
        {
            return;
        }
        let rows = self.world_context_rows(at, false);
        if rows.is_empty() {
            self.push_log("Aucune action sur cette case.".into());
            return;
        }
        self.mouse_walk = None;
        self.world_context = Some(WorldContextMenu {
            at,
            anchor,
            viewport,
            skills: false,
            selected: 0,
            rows,
            message: String::new(),
        });
        self.menu_focus.reset();
    }

    pub(super) fn update_world_context(&mut self, input: &InputFrame, now: f64) {
        let Some(mut menu) = self.world_context.take() else {
            return;
        };
        self.menu_focus.begin_frame(input.pointer);
        menu.viewport = input.viewport.unwrap_or(menu.viewport);
        if input.pressed.contains(&controls::Binding::MouseRight) {
            return;
        }
        if self.controls.pressed(Action::MenuUp, input) || input.wheel_y > 0.0 {
            menu.selected = menu.selected.saturating_sub(1);
        }
        if self.controls.pressed(Action::MenuDown, input) || input.wheel_y < 0.0 {
            menu.selected = (menu.selected + 1).min(menu.rows.len() - 1);
        }
        let hovered = input.pointer.and_then(|p| {
            (menu.start()..(menu.start() + menu.page_size()).min(menu.rows.len()))
                .find(|i| menu.row_rect(*i).contains(p.into()))
        });
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        if clicked
            && !input
                .pointer
                .is_some_and(|p| menu.panel().contains(p.into()))
        {
            return;
        }
        let keyboard = self.controls.pressed(Action::MenuUp, input)
            || self.controls.pressed(Action::MenuDown, input)
            || self.controls.pressed(Action::Learn, input)
            || input.wheel_y != 0.0;
        self.menu_focus
            .update(hovered, &mut menu.selected, clicked, keyboard);
        let shortcut = menu.rows.iter().position(|row| {
            row.choice
                .shortcut()
                .is_some_and(|action| self.controls.pressed(action, input))
        });
        if let Some(index) = shortcut {
            menu.selected = index;
        }
        let confirmed = shortcut.is_some()
            || clicked && hovered.is_some()
            || self.controls.pressed(Action::Learn, input);
        if !confirmed {
            self.world_context = Some(menu);
            return;
        }
        let choice = menu.rows[menu.selected].choice.clone();
        let fresh = self.world_context_rows(menu.at, menu.skills);
        let Some(row) = fresh.iter().find(|r| r.choice == choice) else {
            return;
        };
        if let Some(reason) = &row.blocked {
            menu.message = reason.clone();
            menu.rows = fresh;
            self.world_context = Some(menu);
            return;
        }
        if matches!(choice, Choice::Skills | Choice::Back) {
            menu.skills = choice == Choice::Skills;
            menu.rows = self.world_context_rows(menu.at, menu.skills);
            menu.selected = 0;
            self.menu_focus.reset();
            menu.message.clear();
            self.world_context = Some(menu);
            return;
        }
        self.attack_aim = None;
        self.attack_aim_technique = None;
        self.attack_aim_pointer = None;
        self.select_world_target(Some(menu.at));
        let command = match choice {
            Choice::Attack => {
                if let Some(action) = self.attack_arrival(menu.at) {
                    self.start_approach_action(action, now);
                }
                None
            }
            Choice::Interact | Choice::PickUp => {
                self.start_approach(menu.at, choice == Choice::PickUp, now);
                None
            }
            Choice::Wait => Some(self.preparation_wait_command()),
            Choice::Inspect => {
                if let Some(actor) = self.game.actors().entity_at(menu.at) {
                    self.start_approach_action(ArrivalAction::Inspect { at: menu.at, actor }, now);
                }
                None
            }
            Choice::Move => {
                self.start_mouse_walk(menu.at, now);
                None
            }
            Choice::LearnSkills => {
                self.skills_open = true;
                self.clamp_skill_selection();
                None
            }
            Choice::Technique(id) => {
                let action = self
                    .game
                    .rules()
                    .skills
                    .technique(&id)
                    .and_then(|d| d.action())
                    .unwrap();
                if world_aim(action) {
                    self.begin_attack_aim_at(Some(menu.at), None, Some(id));
                    None
                } else {
                    self.technique_command(id)
                }
            }
            Choice::Skills | Choice::Back | Choice::EmptySkills => None,
        };
        if let Some(command) = command {
            if let CommandOutcome::Rejected(reason) = self.execute_command(command) {
                menu.message = command_rejection_message(reason).to_owned();
                menu.rows = self.world_context_rows(menu.at, menu.skills);
                self.world_context = Some(menu);
            }
            self.capture_events();
        }
        if let Some(walk) = self.mouse_walk.as_mut() {
            walk.trigger_key = input
                .pressed
                .iter()
                .find(|key| matches!(key, controls::Binding::Key(_)))
                .cloned();
        }
    }

    pub(super) fn draw_world_context(&self) {
        let Some(menu) = &self.world_context else {
            return;
        };
        let panel = menu.panel();
        let theme = UiTheme;
        theme.context_surface(panel);
        let entity = self.game.actors().entity_at(menu.at);
        let own = Some(menu.at) == self.game.player_position();
        let title = if own {
            "Votre personnage".to_owned()
        } else {
            self.interaction_display_name(menu.at)
                .or_else(|| {
                    entity
                        .and_then(|id| self.terminal_target_summary_for(id))
                        .map(|s| s.name)
                })
                .unwrap_or_else(|| "Case du terrain".to_owned())
        };
        let emblem = Rect::new(panel.x + 14.0, panel.y + 17.0, 30.0, 30.0);
        theme.hud_chip(emblem);
        draw_ui_icon(
            if menu.skills {
                UiIcon::Techniques
            } else if own {
                UiIcon::Character
            } else if entity.is_some() {
                UiIcon::Target
            } else {
                UiIcon::Grid
            },
            Rect::new(emblem.x + 6.0, emblem.y + 6.0, 18.0, 18.0),
            theme.accent(),
        );
        if menu.skills {
            draw_text(
                "COMPÉTENCES APPRISES",
                panel.x + 56.0,
                panel.y + 24.0,
                10.0,
                theme.muted(),
            );
        }
        let title = fit_inventory_label(&title, panel.w - 72.0, 17, true);
        draw_text_bold(
            title,
            panel.x + 56.0,
            panel.y + if menu.skills { 44.0 } else { 38.0 },
            17.0,
            theme.text(),
        );
        let divider = Color::new(theme.muted().r, theme.muted().g, theme.muted().b, 0.15);
        draw_line(
            panel.x + 14.0,
            panel.y + 57.0,
            panel.right() - 14.0,
            panel.y + 57.0,
            1.0,
            divider,
        );
        for index in menu.start()..(menu.start() + menu.page_size()).min(menu.rows.len()) {
            let row = &menu.rows[index];
            let rect = menu.row_rect(index);
            let highlighted = self.menu_focus.highlighted(index, menu.selected);
            let enabled = row.blocked.is_none();
            theme.context_row(rect, highlighted, enabled);
            let icon = match row.choice {
                Choice::Move => UiIcon::Play,
                Choice::Attack => UiIcon::Attack,
                Choice::Interact => UiIcon::Interact,
                Choice::PickUp => UiIcon::Inventory,
                Choice::Wait => UiIcon::Wait,
                Choice::Inspect => UiIcon::Help,
                Choice::Skills
                | Choice::Technique(_)
                | Choice::LearnSkills
                | Choice::EmptySkills => UiIcon::Techniques,
                Choice::Back => UiIcon::Back,
            };
            let icon_color = if !enabled {
                theme.muted()
            } else if row.choice == Choice::Attack {
                theme.danger()
            } else if highlighted {
                theme.accent()
            } else {
                theme.muted()
            };
            draw_ui_icon(
                icon,
                Rect::new(rect.x + 11.0, rect.y + (rect.h - 17.0) * 0.5, 17.0, 17.0),
                icon_color,
            );
            crate::ui_theme::draw_text_in_rect(
                &row.label,
                Rect::new(
                    rect.x + 40.0,
                    rect.y
                        + if row.blocked.is_some() {
                            3.0
                        } else {
                            (rect.h - 21.0) * 0.5
                        },
                    rect.w
                        - if row.choice.shortcut().is_some() {
                            102.0
                        } else {
                            65.0
                        },
                    21.0,
                ),
                16,
                if row.blocked.is_some() {
                    theme.muted()
                } else {
                    theme.text()
                },
            );
            if let Some(reason) = &row.blocked {
                crate::ui_theme::draw_text_in_rect(
                    reason,
                    Rect::new(rect.x + 40.0, rect.y + 25.0, rect.w - 50.0, 16.0),
                    12,
                    theme.muted(),
                );
            }
            if matches!(row.choice, Choice::Skills) {
                let x = rect.right() - 14.0;
                let y = rect.y + rect.h * 0.5;
                draw_line(x - 3.0, y - 4.0, x + 1.0, y, 1.5, theme.muted());
                draw_line(x + 1.0, y, x - 3.0, y + 4.0, 1.5, theme.muted());
            }
            if let Some(action) = row.choice.shortcut() {
                let key = Rect::new(rect.right() - 57.0, rect.y + 8.0, 34.0, 22.0);
                theme.hud_chip(key);
                draw_text_bold_centered(self.controls.label(action), key, 11, theme.muted());
            }
        }
        let footer = if !menu.message.is_empty() {
            menu.message.clone()
        } else if menu.rows.len() > menu.page_size() {
            format!(
                "{}/{} · Molette pour défiler",
                menu.selected + 1,
                menu.rows.len()
            )
        } else {
            "Clic extérieur : fermer".into()
        };
        draw_line(
            panel.x + 14.0,
            panel.bottom() - 34.0,
            panel.right() - 14.0,
            panel.bottom() - 34.0,
            1.0,
            divider,
        );
        crate::ui_theme::draw_text_in_rect(
            &footer,
            Rect::new(panel.x + 15.0, panel.bottom() - 29.0, panel.w - 72.0, 23.0),
            12,
            theme.muted(),
        );
        let key = Rect::new(panel.right() - 49.0, panel.bottom() - 27.0, 34.0, 20.0);
        theme.hud_chip(key);
        draw_text_bold_centered("Échap", key, 10, theme.muted());
    }

    fn start_mouse_walk(&mut self, at: GridPos, now: f64) {
        let Some(path) = self.context_path(at) else {
            return;
        };
        let origin = self.game.player_position().unwrap();
        let remaining_steps = path.len();
        let cells: VecDeque<_> = path.into_iter().skip(1).collect();
        self.mouse_walk = Some(MouseWalk {
            trigger_key: None,
            cells,
            expected: origin,
            next_at: now,
            arrival: None,
            known_hostiles: self.context_visible_hostiles(),
            remaining_steps,
        });
        self.step_mouse_walk(now);
    }

    pub(super) fn tick_mouse_walk(&mut self, now: f64) {
        if self.mouse_walk.is_none() {
            return;
        }
        if self.menu != MenuScreen::Hidden
            || self.menu_navigation_context().is_some()
            || self.attack_aim.is_some()
            || self.game.status() != RunStatus::Active
            || self.mouse_walk.as_ref().is_some_and(|walk| {
                !self
                    .context_visible_hostiles()
                    .is_subset(&walk.known_hostiles)
            })
        {
            self.mouse_walk = None;
            return;
        }
        self.step_mouse_walk(now);
    }

    fn step_mouse_walk(&mut self, now: f64) {
        let Some(mut walk) = self.mouse_walk.take() else {
            return;
        };
        if now < walk.next_at {
            self.mouse_walk = Some(walk);
            return;
        }
        if self.game.player_position() != Some(walk.expected) {
            return;
        }
        // Combat targets can move during their turns. Follow the same visible
        // actor, never another occupant, and stop chasing after a bounded route.
        if let Some(ArrivalAction::Attack {
            at,
            actor: Some(id),
            ..
        }) = &mut walk.arrival
        {
            let Some(actor) = self.game.actors().get(*id) else {
                return;
            };
            *at = actor.position();
        }
        if walk
            .arrival
            .as_ref()
            .is_some_and(|action| !self.arrival_valid(action))
        {
            self.push_log("Approche annulée : la cible a changé.".into());
            return;
        }
        if let Some(action @ ArrivalAction::Attack { .. }) = walk.arrival.as_ref() {
            if self.arrival_ready(action) {
                self.finish_arrival(action.clone());
                return;
            }
            match self.attack_approach_path(action) {
                Ok(path) => walk.cells = path.into_iter().skip(1).collect(),
                Err(reason) => {
                    self.push_log(reason);
                    return;
                }
            }
        }
        let Some(at) = walk.cells.pop_front() else {
            if let Some(action) = walk.arrival {
                self.finish_arrival(action);
            }
            return;
        };
        if walk.remaining_steps == 0 {
            self.push_log("Approche interrompue : cible trop mobile.".into());
            return;
        }
        walk.remaining_steps -= 1;
        if self.game.player_position() != Some(walk.expected) || !self.context_safe_cell(at) {
            return;
        }
        let direction = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
        .into_iter()
        .find(|d| walk.expected.step(*d) == at);
        let Some(direction) = direction else {
            return;
        };
        let hp = self
            .game
            .actors()
            .get(self.game.player_id())
            .unwrap()
            .integrity();
        let result = self.execute_command(GameCommand::Move(direction));
        self.capture_events();
        if result != CommandOutcome::Applied
            || self.game.player_position() != Some(at)
            || self
                .game
                .actors()
                .get(self.game.player_id())
                .is_none_or(|a| a.integrity() < hp)
            || !self
                .context_visible_hostiles()
                .is_subset(&walk.known_hostiles)
        {
            return;
        }
        if !walk.cells.is_empty() || walk.arrival.is_some() {
            walk.expected = at;
            walk.next_at = now + 0.16;
            self.mouse_walk = Some(walk);
        }
    }
}
