//! Arsenal acceptance scenarios use actual campaign content.
use super::*;

#[cfg(debug_assertions)]
impl AsciiApp {
    pub(super) fn verify_grenade_direct_pointer_input(&mut self) -> Result<(), String> {
        self.attack_aim = None;
        self.attack_aim_pointer = None;
        self.selected_target = None;
        let at = GridPos::new(10, 12);
        let target = self.game.actors().entity_at(at).ok_or("Cible absente")?;
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
            .ok_or("Cible hors caméra")?;
        let scale = self.ui_scale();
        let turn = self.game.turn();
        let ammo = self.game.player_matter().unwrap();
        self.update_input(&InputFrame {
            pressed: [controls::Binding::MouseLeft].into(),
            pointer: Some((
                (cell.x + cell.w * 0.5) / scale,
                (cell.y + cell.h * 0.5) / scale,
            )),
            viewport: Some((self.ui_width(), self.ui_height())),
            ..Default::default()
        });
        if self.selected_target != Some(target)
            || self.attack_aim.is_some()
            || self.game.turn() != turn
        {
            return Err("Le clic n'a pas sélectionné la cible sans tirer".into());
        }
        let button = if self.attack_aim.is_some() {
            Self::attack_aim_confirm_rect(self.ui_width(), self.ui_height())
        } else {
            Self::hud_shortcuts(self.ui_width(), self.ui_height())
                .into_iter()
                .find(|(_, action, _, _)| *action == Some(Action::Attack))
                .unwrap()
                .0
        };
        self.update_input(&InputFrame {
            pressed: [controls::Binding::MouseLeft].into(),
            pointer: Some((button.x + button.w * 0.5, button.y + button.h * 0.5)),
            viewport: Some((self.ui_width(), self.ui_height())),
            ..Default::default()
        });
        if self.attack_aim.is_some()
            || self.game.turn() != turn + 1
            || self.game.player_matter() != Some(ammo - 4)
            || self.game.explosive_devices().at(at).next().is_none()
        {
            return Err(
                "Le bouton Attaquer n'a pas lancé la grenade sur la cible sélectionnée".into(),
            );
        }
        Ok(())
    }

    pub(super) fn verify_grenade_pointer_input(&mut self) -> Result<(), String> {
        let empty = GridPos::new(11, 11);
        if self.game.actors().entity_at(empty).is_some() {
            return Err("Case d'essai occupée".into());
        }
        let controls = self.controls.clone();
        // Even a mouse-bound attack must not turn a map selection into a shot.
        self.controls
            .rebind(Action::Attack, controls::Binding::MouseLeft)?;
        self.attack_aim = None;
        self.selected_target = None;
        let turn = self.game.turn();
        let ammo = self.game.player_matter().unwrap();
        let before = suspension::fingerprint(&self.game);
        for (index, at) in [
            GridPos::new(8, 12),
            empty,
            GridPos::new(10, 12),
            empty,
            empty,
        ]
        .into_iter()
        .enumerate()
        {
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
                .ok_or("Case d'essai hors caméra")?;
            let scale = self.ui_scale();
            let pointer = Some((
                (cell.x + cell.w * 0.5) / scale,
                (cell.y + cell.h * 0.5) / scale,
            ));
            let previous = self.attack_aim.map(|aim| aim.cursor);
            self.update_input(&InputFrame {
                pointer,
                viewport: Some((self.ui_width(), self.ui_height())),
                ..Default::default()
            });
            if self.attack_aim.map(|aim| aim.cursor) != previous {
                return Err("Le survol a déplacé la sélection".into());
            }
            self.update_input(&InputFrame {
                pressed: [controls::Binding::MouseLeft].into(),
                pointer,
                viewport: Some((self.ui_width(), self.ui_height())),
                ..Default::default()
            });
            if suspension::fingerprint(&self.game) != before
                || self.selected_target != self.game.actors().entity_at(at)
                || (index > 0 && self.attack_aim.map(|aim| aim.cursor) != Some(at))
            {
                return Err("Le clic a tiré ou n'a pas changé la sélection".into());
            }
        }
        let button = Self::attack_aim_confirm_rect(self.ui_width(), self.ui_height());
        self.update_input(&InputFrame {
            pressed: [controls::Binding::MouseLeft].into(),
            pointer: Some((button.x + button.w * 0.5, button.y + button.h * 0.5)),
            viewport: Some((self.ui_width(), self.ui_height())),
            ..Default::default()
        });
        if self.attack_aim.is_some()
            || self.game.turn() != turn + 1
            || self.game.player_matter() != Some(ammo - 4)
            || self.game.explosive_devices().at(empty).next().is_none()
        {
            return Err(
                "Le bouton Attaquer n'a pas lancé la grenade sur la case sélectionnée".into(),
            );
        }
        self.controls = controls;
        Ok(())
    }

    pub(super) fn prepare_arsenal_diagnostic(&mut self, scene: &str) -> Result<(), String> {
        self.open_menu(MenuScreen::Main);
        self.enter_test_lab()?;
        let id = if scene.contains("rocket") {
            "core:lance_roquettes"
        } else if scene.contains("machine") {
            "core:mitrailleuse_lourde"
        } else if scene.contains("assault") {
            "core:fusil_d_assaut"
        } else {
            "core:lance_grenades"
        };
        let item = self
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item().as_str() == id)
            .ok_or("Arme absente")?
            .instance();
        self.execute_command(GameCommand::EquipWeapon { slot: 0, item });
        self.active_weapon_slot = 0;
        if scene.contains("inventory") {
            self.inventory_filter = InventoryFilter::Weapons;
            self.inventory_selection = self
                .inventory_entries()
                .iter()
                .position(|entry| entry.instance() == item)
                .unwrap();
            self.inventory_open = true;
        } else if id == "core:fusil_d_assaut" {
            self.selected_target = self.game.actors().entity_at(GridPos::new(10, 12));
        } else {
            let target = if scene.contains("diagonal") {
                GridPos::new(10, 9)
            } else if scene.contains("oblique") {
                GridPos::new(11, 10)
            } else {
                GridPos::new(10, 12)
            };
            self.begin_attack_aim_at(Some(target), None, None);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terminal_view::TerminalEffectBadge;
    use project_rl::game::{CommandOutcome, GameRng};
    use project_rl::loot::{EquipmentQuality, EquipmentSource};
    use project_rl::world::{Map, Terrain};

    fn game(name: &str) -> GameState {
        let (mut rules, _, _, _) = ascii_game_content().unwrap();
        let id: ContentId = format!("core:{name}").parse().unwrap();
        rules.hit_rules = None;
        rules.physical_rules = None;
        rules.player_body_profile = None;
        rules.player_maximum_integrity = 100;
        rules.player_starting_weapons = vec![id.clone()];
        rules.player_weapon_slots.truncate(1);
        rules.player_starting_equipment = vec![Some(id)];
        GameState::new_with_rules(
            Map::filled(18, 15, Terrain::Floor).unwrap(),
            GridPos::new(5, 6),
            129,
            rules,
        )
        .unwrap()
    }
    fn target(game: &mut GameState, at: GridPos) -> EntityId {
        game.spawn_actor(
            Actor::new(at, 1000)
                .unwrap()
                .with_evasion_disabled()
                .with_tags(["core:fauna_scavengers".parse().unwrap()]),
        )
        .unwrap()
    }
    fn fire(game: &mut GameState, at: GridPos) {
        assert_eq!(
            game.process_player_command(GameCommand::AttackAt {
                slot: 0,
                target: at
            }),
            CommandOutcome::Applied
        );
    }
    fn wait(game: &mut GameState) {
        assert_eq!(
            game.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
    }

    #[test]
    fn arsenal_burst_is_one_turn_three_bullets_one_affix_budget() {
        let mut game = game("fusil_d_assaut")
            .with_starting_weapon_effects([(
                "core:fusil_d_assaut".parse().unwrap(),
                "core:affix_poison".parse().unwrap(),
            )])
            .unwrap();
        let target = target(&mut game, GridPos::new(9, 6));
        game.drain_events();
        let ammo = game.player_matter().unwrap();
        assert_eq!(
            game.process_player_command(GameCommand::Attack { slot: 0, target }),
            CommandOutcome::Applied
        );
        assert_eq!(game.turn(), 1);
        assert_eq!(game.player_matter(), Some(ammo - 3));
        assert_eq!(game.actors().get(target).unwrap().integrity(), 990); // 3x3 + poison tick.
        let events = game.drain_events();
        assert_eq!(
            events
                .iter()
                .filter(|event| matches!(event, GameEvent::AttackPerformed { .. }))
                .count(),
            3
        );
        assert_eq!(events.iter().filter(|event| matches!(event, GameEvent::StatusApplied { status, .. } if status.as_str() == "core:poisoned")).count(), 1);
    }

    #[test]
    fn arsenal_grenade_contact_then_fixed_ground_blast_survives_snapshot() {
        let mut game = game("lance_grenades");
        let at = GridPos::new(9, 6);
        let center = target(&mut game, at);
        let neighbor = target(&mut game, GridPos::new(10, 6));
        let before = suspension::fingerprint(&game);
        assert_eq!(game.player_attack_preview(0, at).unwrap().cells().len(), 9);
        assert_eq!(before, suspension::fingerprint(&game));
        fire(&mut game, at);
        assert_eq!(game.actors().get(center).unwrap().integrity(), 998);
        assert_eq!(game.actors().get(neighbor).unwrap().integrity(), 1000);
        assert_eq!(game.explosive_devices().iter().count(), 1);
        game.drain_events();
        let rules = game.rules().clone();
        let world = WorldState::single(game);
        let mut restored = WorldState::from_recovery_snapshot_bytes(
            &world.recovery_snapshot_bytes().unwrap(),
            rules,
        )
        .unwrap();
        assert_eq!(
            suspension::fingerprint(&world),
            suspension::fingerprint(&restored)
        );
        wait(&mut restored);
        assert_eq!(restored.actors().get(center).unwrap().integrity(), 998);
        wait(&mut restored);
        assert!(restored.explosive_devices().is_empty());
        assert_eq!(restored.actors().get(center).unwrap().integrity(), 989);
        assert_eq!(restored.actors().get(neighbor).unwrap().integrity(), 991);
    }

    #[test]
    fn arsenal_empty_ground_grenade_and_rocket_have_real_area_damage() {
        let mut grenade = game("lance_grenades");
        let neighbor = target(&mut grenade, GridPos::new(10, 6));
        fire(&mut grenade, GridPos::new(9, 6));
        assert_eq!(grenade.actors().get(neighbor).unwrap().integrity(), 1000);
        wait(&mut grenade);
        wait(&mut grenade);
        assert_eq!(grenade.actors().get(neighbor).unwrap().integrity(), 991);
        let mut rocket = game("lance_roquettes");
        let center = target(&mut rocket, GridPos::new(6, 6));
        let neighbor = target(&mut rocket, GridPos::new(7, 6));
        fire(&mut rocket, GridPos::new(6, 6));
        for id in [center, neighbor] {
            assert_eq!(rocket.actors().get(id).unwrap().integrity(), 990);
        }
        assert_eq!(
            rocket.actors().get(rocket.player_id()).unwrap().integrity(),
            90
        );
        assert!(rocket.explosive_devices().is_empty());
    }

    #[test]
    fn arsenal_dots_refresh_without_stacking_and_do_not_affect_machines() {
        let mut game = game("fusil_d_assaut")
            .with_starting_weapon_effects([(
                "core:fusil_d_assaut".parse().unwrap(),
                "core:affix_saignement".parse().unwrap(),
            )])
            .unwrap();
        let organic = target(&mut game, GridPos::new(9, 6));
        let machine = game
            .spawn_actor(Actor::new(GridPos::new(9, 8), 1000).unwrap())
            .unwrap();
        let bleed: ContentId = "core:bleeding".parse().unwrap();
        for _ in 0..2 {
            assert_eq!(
                game.process_player_command(GameCommand::Attack {
                    slot: 0,
                    target: organic
                }),
                CommandOutcome::Applied
            );
            let status = game.actors().get(organic).unwrap().status(&bleed).unwrap();
            assert_eq!(status.stacks, 1);
            assert_eq!(status.remaining_turns, Some(2));
        }
        assert_eq!(game.actors().get(organic).unwrap().integrity(), 978);
        assert_eq!(
            game.process_player_command(GameCommand::Attack {
                slot: 0,
                target: machine
            }),
            CommandOutcome::Applied
        );
        assert!(game.actors().get(machine).unwrap().status(&bleed).is_none());
        assert_eq!(game.actors().get(machine).unwrap().integrity(), 991);
    }

    #[test]
    fn arsenal_machine_gun_sustains_resumes_and_cools_after_a_pause() {
        let mut game = game("mitrailleuse_lourde");
        let target = target(&mut game, GridPos::new(9, 6));
        let heat = game.player_heat().unwrap();
        assert_eq!(
            game.process_player_command(GameCommand::Attack { slot: 0, target }),
            CommandOutcome::Applied
        );
        assert_eq!(
            game.player_heat().unwrap().current(),
            8_u16.saturating_sub(heat.dissipation_per_phase())
        );
        game.drain_events();
        let rules = game.rules().clone();
        let world = WorldState::single(game);
        let restored = WorldState::from_recovery_snapshot_bytes(
            &world.recovery_snapshot_bytes().unwrap(),
            rules,
        )
        .unwrap();
        assert_eq!(
            suspension::fingerprint(&world),
            suspension::fingerprint(&restored)
        );
        let mut restored = restored;
        let previous = restored.player_heat().unwrap().current();
        wait(&mut restored);
        assert!(restored.player_heat().unwrap().current() < previous);
    }

    #[test]
    fn arsenal_grenade_direct_selected_target_fires_without_area_mode() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(129, rules, texts, loot, expeditions).unwrap();
        app.prepare_arsenal_diagnostic("arsenal-grenade-preview")
            .unwrap();
        app.attack_aim = None;
        let at = GridPos::new(10, 12);
        let target = app.game.actors().entity_at(at).unwrap();
        app.selected_target = Some(target);
        let turn = app.game.turn();
        let ammo = app.game.player_matter().unwrap();
        let hp = app.game.actors().get(target).unwrap().integrity();
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Attack).clone()].into(),
            ..Default::default()
        });
        assert!(
            app.attack_aim.is_none(),
            "A selected target should receive the direct shot"
        );
        assert_eq!(app.game.turn(), turn + 1);
        assert_eq!(app.game.player_matter(), Some(ammo - 4));
        assert!(app.game.actors().get(target).unwrap().integrity() < hp);
        assert!(
            app.effect_badges_at(at)
                .contains(&TerminalEffectBadge::Grenade { turns: 2 })
        );
        let hit_hp = app.game.actors().get(target).unwrap().integrity();
        app.execute_command(GameCommand::Wait);
        assert_eq!(app.game.actors().get(target).unwrap().integrity(), hit_hp);
        app.execute_command(GameCommand::Wait);
        assert!(app.game.actors().get(target).unwrap().integrity() < hit_hp);
    }

    #[test]
    fn arsenal_grenade_free_ground_aim_works_with_keyboard() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(129, rules, texts, loot, expeditions).unwrap();
        app.prepare_arsenal_diagnostic("arsenal-grenade-preview")
            .unwrap();
        app.attack_aim = None;
        app.controls
            .rebind(Action::AimGround, controls::Binding::key("F7"))
            .unwrap();
        app.selected_target = app.game.actors().entity_at(GridPos::new(10, 12));
        let opening_turn = app.game.turn();
        let ammo = app.game.player_matter();
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::AimGround).clone()].into(),
            ..Default::default()
        });
        assert!(app.attack_aim.is_some());
        assert_eq!(app.game.turn(), opening_turn);
        assert_eq!(app.game.player_matter(), ammo);
        assert_eq!(app.attack_aim.unwrap().cursor, GridPos::new(10, 12));
        let before = app.game.turn();
        let move_north = app.controls.binding(Action::MoveNorth).clone();
        app.update_input(&InputFrame {
            pressed: [move_north].into(),
            ..Default::default()
        });
        let empty = app.attack_aim.unwrap().cursor;
        assert_eq!(empty, GridPos::new(10, 11));
        assert!(app.game.actors().entity_at(empty).is_none());
        assert_eq!(app.selected_target, None);
        assert_eq!(app.game.turn(), before);
        let confirm = app.controls.binding(Action::Attack).clone();
        app.update_input(&InputFrame {
            pressed: [confirm].into(),
            ..Default::default()
        });
        assert!(app.attack_aim.is_none());
        assert_eq!(app.game.turn(), before + 1);
        assert!(app.game.explosive_devices().at(empty).next().is_some());
        assert!(
            app.effect_badges_at(empty)
                .contains(&TerminalEffectBadge::Grenade { turns: 2 })
        );
    }

    #[test]
    fn arsenal_lab_grenade_uses_shared_stock_after_other_weapons_and_reset() {
        for version in [127, 129] {
            let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
            let mut app =
                AsciiApp::from_seed_version(129, rules, texts, loot, expeditions, version).unwrap();
            let campaign = suspension::fingerprint(&app.game);
            app.open_menu(MenuScreen::Main);
            app.enter_test_lab().unwrap();
            let stock = app.game.player_matter().unwrap();
            assert!(stock >= 400);
            let mut remaining = stock;
            for (name, cost) in [
                ("core:fusil_d_assaut", 3),
                ("core:lance_roquettes", 5),
                ("core:lance_grenades", 4),
            ] {
                let item = app
                    .game
                    .player_inventory()
                    .iter()
                    .find(|entry| entry.item().as_str() == name)
                    .unwrap()
                    .instance();
                assert_eq!(
                    app.execute_command(GameCommand::EquipWeapon { slot: 0, item }),
                    CommandOutcome::Applied
                );
                app.active_weapon_slot = 0;
                assert!(
                    app.active_weapon_hud_cost()
                        .contains(&format!("{cost} munitions"))
                );
                let command = if name == "core:fusil_d_assaut" {
                    GameCommand::Attack {
                        slot: 0,
                        target: app.game.actors().entity_at(GridPos::new(10, 12)).unwrap(),
                    }
                } else {
                    GameCommand::AttackAt {
                        slot: 0,
                        target: GridPos::new(11, 11),
                    }
                };
                assert_eq!(app.execute_command(command), CommandOutcome::Applied);
                remaining -= cost;
                assert_eq!(app.game.player_matter(), Some(remaining));
                assert_eq!(
                    app.resource_gauges()
                        .iter()
                        .find(|g| g.label == "Munitions")
                        .unwrap()
                        .value,
                    remaining.to_string()
                );
            }
            assert!(
                app.game
                    .explosive_devices()
                    .at(GridPos::new(11, 11))
                    .next()
                    .is_some()
            );
            app.reset_test_lab().unwrap();
            assert_eq!(app.game.player_matter(), Some(stock));
            app.leave_test_lab();
            assert_eq!(suspension::fingerprint(&app.game), campaign);
        }
    }

    #[test]
    fn arsenal_ammo_rejection_is_atomic_and_grenades_stay_after_a_kill() {
        let mut burst = game("fusil_d_assaut");
        let victim = target(&mut burst, GridPos::new(9, 6));
        for _ in 0..13 {
            assert_eq!(
                burst.process_player_command(GameCommand::Attack {
                    slot: 0,
                    target: victim
                }),
                CommandOutcome::Applied
            );
        }
        assert_eq!(burst.player_matter(), Some(1));
        let before = suspension::fingerprint(&burst);
        assert!(matches!(
            burst.process_player_command(GameCommand::Attack {
                slot: 0,
                target: victim
            }),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(before, suspension::fingerprint(&burst));

        let mut grenade = game("lance_grenades");
        let at = GridPos::new(9, 6);
        let doomed = grenade
            .spawn_actor(Actor::new(at, 1).unwrap().with_evasion_disabled())
            .unwrap();
        let neighbor = target(&mut grenade, GridPos::new(10, 6));
        fire(&mut grenade, at);
        assert!(grenade.actors().get(doomed).is_none());
        assert_eq!(grenade.explosive_devices().iter().count(), 1);
        wait(&mut grenade);
        wait(&mut grenade);
        assert_eq!(grenade.actors().get(neighbor).unwrap().integrity(), 991);
    }

    #[test]
    fn arsenal_machine_gun_accuracy_grows_only_on_consecutive_stationary_fire() {
        let seed_game = game("mitrailleuse_lourde");
        let mut rules = seed_game.rules().clone();
        rules.hit_rules = Some(Default::default());
        let mut game =
            GameState::new_with_rules(seed_game.map().clone(), GridPos::new(5, 6), 129, rules)
                .unwrap();
        let target = game
            .spawn_actor(Actor::new(GridPos::new(9, 6), 1000).unwrap())
            .unwrap();
        let mut chances = Vec::new();
        for index in 0..3 {
            if index == 2 {
                wait(&mut game);
            }
            game.drain_events();
            assert_eq!(
                game.process_player_command(GameCommand::Attack { slot: 0, target }),
                CommandOutcome::Applied
            );
            chances.push(
                game.drain_events()
                    .into_iter()
                    .find_map(|event| match event {
                        GameEvent::AttackHitResolved { chance, .. } => Some(chance),
                        _ => None,
                    })
                    .unwrap(),
            );
        }
        assert!(chances[1] > chances[0], "{chances:?}");
        assert_eq!(chances[2], chances[0]);
    }

    #[test]
    fn arsenal_siphon_aggregates_real_burst_damage_and_caps_once() {
        let base = game("fusil_d_assaut");
        let mut rules = base.rules().clone();
        let affix: ContentId = "test:siphon".parse().unwrap();
        rules
            .weapons
            .register_effect_affix(
                project_rl::weapon::WeaponEffectAffixDefinition::new(
                    affix.clone(),
                    "suffix".into(),
                    "description".into(),
                    vec![
                        project_rl::weapon::WeaponEffect::life_steal(
                            50,
                            4,
                            "core:fauna_scavengers".parse().unwrap(),
                            project_rl::weapon::WeaponEffectTrigger::OnDamage,
                        )
                        .unwrap(),
                    ],
                )
                .unwrap(),
            )
            .unwrap();
        let mut game =
            GameState::new_with_rules(base.map().clone(), GridPos::new(5, 6), 129, rules)
                .unwrap()
                .with_starting_weapon_effects([("core:fusil_d_assaut".parse().unwrap(), affix)])
                .unwrap()
                .with_starting_player_integrity(40)
                .unwrap();
        let target = target(&mut game, GridPos::new(9, 6));
        game.drain_events();
        assert_eq!(
            game.process_player_command(GameCommand::Attack { slot: 0, target }),
            CommandOutcome::Applied
        );
        assert_eq!(game.actors().get(game.player_id()).unwrap().integrity(), 44);
        assert_eq!(
            game.drain_events()
                .iter()
                .filter(|event| matches!(event, GameEvent::IntegrityRestored { amount: 4, .. }))
                .count(),
            1
        );
    }

    #[test]
    fn arsenal_catalog_provenance_lab_badges_and_legacy_tables() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        for version in [127, 128] {
            let legacy = rules_for_generation_version(rules.clone(), version);
            for id in equipment_generation::ARSENAL_BASE_IDS {
                assert!(legacy.weapons.get(&id.parse().unwrap()).is_none());
            }
        }
        let mut seen = std::collections::BTreeSet::new();
        let mut rng = GameRng::from_seed(129);
        for depth in 0..=2 {
            for _ in 0..1000 {
                let rolled = loot
                    .equipment()
                    .draw(
                        EquipmentSource::HumanoidSite,
                        depth,
                        EquipmentQuality::Enchanted,
                        &mut rng,
                    )
                    .unwrap()
                    .unwrap();
                if equipment_generation::ARSENAL_BASE_IDS.contains(&rolled.item.as_str()) {
                    seen.insert(rolled.item.to_string());
                }
            }
        }
        assert_eq!(seen.len(), 6);
        assert!(
            loot.equipment()
                .draw(EquipmentSource::Robot, 1, EquipmentQuality::White, &mut rng)
                .unwrap()
                .is_none()
        );
        let mut app = AsciiApp::from_seed(129, rules, texts, loot, expeditions).unwrap();
        app.prepare_arsenal_diagnostic("arsenal-grenade-preview")
            .unwrap();
        for id in equipment_generation::ARSENAL_BASE_IDS {
            assert!(
                app.game
                    .player_inventory()
                    .iter()
                    .any(|entry| entry.item().as_str() == id && entry.magic_modifiers().is_none())
            );
        }
        let aim = app.attack_aim.unwrap();
        assert_eq!(
            app.execute_command(GameCommand::AttackAt {
                slot: 0,
                target: aim.cursor
            }),
            CommandOutcome::Applied
        );
        assert!(
            app.effect_badges_at(aim.cursor)
                .contains(&TerminalEffectBadge::Grenade { turns: 2 })
        );
        assert_eq!(
            app.execute_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert!(
            app.effect_badges_at(aim.cursor)
                .contains(&TerminalEffectBadge::Grenade { turns: 1 })
        );
        assert_eq!(
            app.execute_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert!(
            !app.effect_badges_at(aim.cursor)
                .iter()
                .any(|badge| matches!(badge, TerminalEffectBadge::Grenade { .. }))
        );
    }
}
