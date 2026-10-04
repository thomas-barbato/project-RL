//! Optional small installations, using the existing perception and sound rules.
use super::*;
use project_rl::facility::{
    FacilityBlueprint, InstallationAction, InstallationBlueprint, InstallationIntel,
};
use project_rl::world::DoorState;
#[cfg(debug_assertions)]
use std::collections::BTreeSet;

pub(super) struct InstallationMenu {
    target: GridPos,
    selected: usize,
    message: String,
}

const PANEL: GridPos = GridPos::new(9, 3);
const TERMINAL: GridPos = GridPos::new(3, 12);
const POST: GridPos = GridPos::new(10, 7);
const GATE: GridPos = GridPos::new(10, 4);
const ENTRANCE: GridPos = GridPos::new(2, 10);

fn cid(value: &str) -> ContentId {
    value.parse().expect("authored installation id")
}
fn installation(
    name: &str,
    position: GridPos,
    capability: InstallationCapability,
) -> InstallationBlueprint {
    InstallationBlueprint {
        id: cid(name),
        position,
        integrity: 10,
        maximum_integrity: 10,
        capabilities: vec![capability],
        dependencies: vec![],
        security_alarm_profile: None,
    }
}

impl AsciiApp {
    pub fn new_installation_trial() -> Result<Self, String> {
        Self::new_with_session_path(
            controls::config_path().with_file_name("essai-interactions.json"),
        )
    }

    fn installation_annex_anchor(&self) -> GridPos {
        self.home_position(GridPos::new(44, 7))
    }

    pub(super) fn add_city_installation_intel(&self, blueprint: &mut FacilityBlueprint) {
        for terminal in blueprint.installations.iter_mut().filter(|installation| {
            installation
                .capabilities
                .iter()
                .any(|capability| matches!(capability, InstallationCapability::DataTerminal { .. }))
        }) {
            terminal
                .capabilities
                .push(InstallationCapability::IntelTerminal {
                    records: vec![InstallationIntel {
                        record: cid("core:intel_transit_annex"),
                        label: "Trouver l’annexe de transit".into(),
                        target: Some(self.installation_annex_anchor()),
                        consumes_turn: false,
                    }],
                });
        }
    }

    pub(super) fn enable_installation_interactions(&mut self) -> Result<(), String> {
        let zone = cid("core:transit_annex");
        let hub = self.starter_hub_definition()?.hub.id.clone();
        let mut map = project_rl::world::Map::from_ascii("#####################\n#.........#.........#\n#.........#.........#\n#.........#.........#\n#...................#\n#.........#.........#\n#.........#.........#\n#.........#.........#\n#.........#.........#\n#.........#.........#\n#...................#\n#...................#\n#####################").map_err(|e| e.to_string())?;
        map.set_terrain(PANEL, Terrain::Wall)
            .map_err(|e| e.to_string())?;
        map.set_terrain(GATE, Terrain::Door(DoorState::Closed))
            .map_err(|e| e.to_string())?;
        let lifecycle = project_rl::ai::PursuitLifecycle::new(
            std::num::NonZeroU16::new(10).unwrap(),
            std::num::NonZeroU16::new(3).unwrap(),
            std::num::NonZeroU16::new(3).unwrap(),
        );
        let observer = Actor::new(GridPos::new(15, 6), 14)
            .map_err(|e| e.to_string())?
            .with_ai(AiProfile::hunter(4, 0).with_pursuit_lifecycle(lifecycle))
            .with_player_relation(PlayerRelation::Hostile)
            .with_affiliation(cid("core:system_security"))
            .with_attack(project_rl::combat::AttackProfile::new(
                1,
                project_rl::world::DistanceMetric::Chebyshev,
                true,
                project_rl::combat::DamageType::Kinetic,
                3,
                0,
            ));
        self.game.add_zone(project_rl::game::ZoneBlueprint {
            info: project_rl::game::ZoneInfo {
                id: zone.clone(),
                name: "Annexe de transit".into(),
                kind: cid("core:industrial"),
                depth: 0,
            },
            map,
            entrance: ENTRANCE,
            seed: self.seed ^ 0x5452_414e_5349_54,
            actors: vec![observer],
            loot: vec![
                project_rl::game::GroundLootBlueprint::new(
                    GridPos::new(15, 4),
                    cid("core:power_regulator"),
                    1,
                )
                .with_owner(cid("core:system_security")),
            ],
            threat_sources: vec![],
        })?;
        self.game.connect_optional(
            hub.clone(),
            self.installation_annex_anchor(),
            zone.clone(),
            ENTRANCE,
        )?;
        let mut sensor = installation(
            "core:transit_sensor",
            GridPos::new(10, 5),
            InstallationCapability::SecuritySensor,
        );
        sensor.dependencies.push(cid("core:transit_relay"));
        sensor.security_alarm_profile = Some(
            project_rl::facility::SecurityAlarmProfile::new(
                6,
                project_rl::world::DistanceMetric::Euclidean,
                true,
                6,
            )
            .map_err(|e| e.to_string())?,
        );
        self.game.register_facility(
            zone.clone(),
            FacilityBlueprint {
                installations: vec![
                    installation(
                        "core:transit_depot",
                        GridPos::new(1, 0),
                        InstallationCapability::Storage,
                    ),
                    installation(
                        "core:transit_terminal",
                        TERMINAL,
                        InstallationCapability::IntelTerminal {
                            records: vec![
                                InstallationIntel {
                                    record: cid("core:intel_transit_return"),
                                    label: "Localiser le retour en ville".into(),
                                    target: Some(ENTRANCE),
                                    consumes_turn: true,
                                },
                                InstallationIntel {
                                    record: cid("core:intel_transit_power"),
                                    label: "Lire le plan d’alimentation".into(),
                                    target: Some(PANEL),
                                    consumes_turn: true,
                                },
                                InstallationIntel {
                                    record: cid("core:intel_transit_diversion"),
                                    label: "Repérer le poste sonore".into(),
                                    target: Some(POST),
                                    consumes_turn: true,
                                },
                            ],
                        },
                    ),
                    installation(
                        "core:transit_panel",
                        PANEL,
                        InstallationCapability::PowerControl {
                            relay: cid("core:transit_relay"),
                        },
                    ),
                    installation(
                        "core:transit_relay",
                        GridPos::new(10, 2),
                        InstallationCapability::SwitchableRelay {
                            door: GATE,
                            powered: true,
                        },
                    ),
                    sensor,
                    installation(
                        "core:transit_post",
                        POST,
                        InstallationCapability::DiversionPost {
                            outlet: POST.step(Direction::East),
                            charges: 2,
                            intensity: 30,
                            duration: 3,
                        },
                    ),
                ],
                depot: cid("core:transit_depot"),
                workers: vec![],
                repair_orders: vec![],
                maximum_path_search: 256,
                owner: Some(cid("core:system_security")),
            },
        )?;
        let mut decor = crate::test_sector::SectorDecor::default();
        decor.fallback_name = Some("Annexe de transit".into());
        for y in 1..12 {
            for x in 1..20 {
                if x != 10 || y >= 10 {
                    decor
                        .cells
                        .insert(GridPos::new(x, y), crate::test_sector::Decor::Deck);
                }
            }
        }
        decor
            .cells
            .insert(ENTRANCE, crate::test_sector::Decor::Passage);
        self.zone_decor.insert(zone, decor);
        let anchor = self.installation_annex_anchor();
        self.zone_decor
            .entry(hub)
            .or_default()
            .cells
            .insert(anchor, crate::test_sector::Decor::Passage);
        // The active town uses its own presentation until the next zone change.
        self.terminal.decor.cells.insert(
            self.installation_annex_anchor(),
            crate::test_sector::Decor::Passage,
        );
        Ok(())
    }

    pub(super) fn installation_name_at(&self, target: GridPos) -> Option<&'static str> {
        let facility = self.game.active_facility()?;
        if !facility.intel_at(target).is_empty() {
            Some("Terminal de renseignements")
        } else if facility.power_setting_at(target).is_some() {
            Some("Alimentation du passage")
        } else if facility.diversion_at(target).is_some() {
            Some("Poste de diversion")
        } else {
            None
        }
    }

    pub(super) fn open_installation_menu(&mut self, target: GridPos) -> bool {
        if !self.game.player_visibility().is_visible(target)
            || !self
                .game
                .player_position()
                .is_some_and(|p| p.cardinal_neighbors().contains(&target))
            || !self
                .game
                .active_facility()
                .is_some_and(|f| f.has_interaction_menu_at(target))
        {
            return false;
        }
        self.installation_menu = Some(InstallationMenu {
            target,
            selected: 0,
            message: String::new(),
        });
        self.movement_repeat.clear();
        self.mouse_walk = None;
        self.world_context = None;
        self.world_cursor = None;
        self.attack_aim = None;
        true
    }

    fn installation_choices(&self, target: GridPos) -> Vec<(String, Option<GameCommand>)> {
        let Some(facility) = self.game.active_facility() else {
            return vec![];
        };
        let mut choices = Vec::new();
        for (index, intel) in facility.intel_at(target).iter().enumerate() {
            let cost = if !intel.consumes_turn {
                "gratuit"
            } else if facility.intel_was_read(&intel.record) {
                "relire gratuitement"
            } else {
                "1 tour"
            };
            choices.push((
                format!("{} · {cost}", intel.label),
                Some(GameCommand::UseInstallation {
                    target,
                    action: InstallationAction::ReadIntel { index: index as u8 },
                }),
            ));
        }
        if let Some(powered) = facility.power_setting_at(target) {
            choices.push((
                format!(
                    "{} · 1 tour",
                    if powered {
                        "Couper l’alimentation"
                    } else {
                        "Rétablir l’alimentation"
                    }
                ),
                Some(GameCommand::UseInstallation {
                    target,
                    action: InstallationAction::SetPower { powered: !powered },
                }),
            ));
        }
        if let Some((charges, _, duration)) = facility.diversion_at(target) {
            choices.push((
                format!("Émettre un bruit · {duration} tours · {charges} charges · 1 tour"),
                Some(GameCommand::UseInstallation {
                    target,
                    action: InstallationAction::ActivateDiversion,
                }),
            ));
        }
        if facility
            .installation_at(target)
            .is_some_and(|installation| {
                installation.capabilities().iter().any(|capability| {
                    matches!(capability, InstallationCapability::DataTerminal { .. })
                })
            })
        {
            choices.push((
                "Consulter les archives · 1 tour".into(),
                Some(GameCommand::Interact { target }),
            ));
        }
        choices.push(("Fermer".into(), None));
        choices
    }

    fn installation_panel(width: f32, height: f32) -> Rect {
        let w = (width - 40.0).min(660.0);
        Rect::new(
            (width - w) * 0.5,
            (height - 420.0).max(20.0) * 0.5,
            w,
            420.0_f32.min(height - 40.0),
        )
    }
    fn installation_row(panel: Rect, index: usize) -> Rect {
        Rect::new(
            panel.x + 20.0,
            panel.y + 86.0 + index as f32 * 42.0,
            panel.w - 40.0,
            36.0,
        )
    }

    pub(super) fn update_installation_menu(&mut self, input: &InputFrame) {
        let Some(mut menu) = self.installation_menu.take() else {
            return;
        };
        if !self
            .game
            .player_position()
            .is_some_and(|p| p.cardinal_neighbors().contains(&menu.target))
        {
            return;
        }
        let choices = self.installation_choices(menu.target);
        if choices.is_empty() {
            return;
        }
        if input.pressed.contains(&controls::Binding::MouseRight) {
            return;
        }
        if self.controls.pressed(Action::MenuUp, input) {
            menu.selected = menu.selected.saturating_sub(1);
        }
        if self.controls.pressed(Action::MenuDown, input) {
            menu.selected = (menu.selected + 1).min(choices.len() - 1);
        }
        let (width, height) = input.viewport.unwrap_or((1280.0, 800.0));
        let panel = Self::installation_panel(width, height);
        let hovered = input.pointer.and_then(|p| {
            (0..choices.len()).find(|&i| Self::installation_row(panel, i).contains(p.into()))
        });
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        if clicked && let Some(index) = hovered {
            menu.selected = index;
        }
        let confirmed = self.controls.pressed(Action::Interact, input)
            || self.controls.pressed(Action::Learn, input)
            || clicked && hovered.is_some();
        if confirmed {
            let Some(command) = choices[menu.selected].1.clone() else {
                return;
            };
            let intel = match &command {
                GameCommand::UseInstallation {
                    action: InstallationAction::ReadIntel { index },
                    ..
                } => self
                    .game
                    .active_facility()
                    .and_then(|f| f.intel_at(menu.target).get(*index as usize))
                    .cloned(),
                _ => None,
            };
            let outcome = self.execute_command(command);
            self.capture_events();
            menu.message = if let CommandOutcome::Rejected(reason) = outcome {
                command_rejection_message(reason).into()
            } else if let Some(intel) = intel {
                let text = self
                    .texts
                    .resolve(DISPLAY_LOCALE, intel.record.as_str())
                    .unwrap_or("Renseignement enregistré dans le dossier.");
                let direction = intel
                    .target
                    .and_then(|target| {
                        self.game.player_position().map(|origin| {
                            format!(
                                " {}.",
                                crate::terminal_view::directional_signal_summary(
                                    "REPÈRE",
                                    origin,
                                    target,
                                    grid_distance(origin, target),
                                    1
                                )
                            )
                        })
                    })
                    .unwrap_or_default();
                format!("{text}{direction}")
            } else {
                "Commande exécutée. L’état de l’installation est conservé.".into()
            };
        }
        if self.game.status() == RunStatus::Active {
            self.installation_menu = Some(menu);
        }
    }

    pub(super) fn draw_installation_menu(&self) {
        let Some(menu) = &self.installation_menu else {
            return;
        };
        let panel = Self::installation_panel(self.ui_width(), self.ui_height());
        let theme = UiTheme;
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            theme.backdrop(),
        );
        theme.card(panel, false);
        draw_text_bold(
            self.installation_name_at(menu.target)
                .unwrap_or("Installation"),
            panel.x + 20.0,
            panel.y + 32.0,
            21.0,
            theme.text(),
        );
        let subtitle = if let Some(powered) = self
            .game
            .active_facility()
            .and_then(|f| f.power_setting_at(menu.target))
        {
            if powered {
                "Passage ouvert · capteur alimenté"
            } else {
                "Passage coupé · capteur hors tension · détour au sud"
            }
        } else if self
            .game
            .active_facility()
            .and_then(|f| f.diversion_at(menu.target))
            .is_some()
        {
            "Attire les ennemis qui l’entendent. S’ils vous voient, vous restez leur cible."
        } else {
            "Le coût est indiqué pour chaque lecture. Les renseignements restent dans le dossier."
        };
        draw_text(
            subtitle,
            panel.x + 20.0,
            panel.y + 62.0,
            15.0,
            theme.muted(),
        );
        let choices = self.installation_choices(menu.target);
        for (index, (label, _)) in choices.iter().enumerate() {
            let row = Self::installation_row(panel, index);
            if index == menu.selected {
                draw_rectangle(row.x, row.y, row.w, row.h, theme.surface_selected());
            }
            draw_text(label, row.x + 10.0, row.y + 24.0, 17.0, theme.text());
        }
        let message_y = panel.y + 86.0 + choices.len() as f32 * 42.0 + 12.0;
        draw_wrapped_text(
            &menu.message,
            panel.x + 20.0,
            message_y,
            panel.w - 40.0,
            4,
            15,
            theme.text(),
        );
        draw_text(
            "Haut/Bas choisir · E/Entrée valider · Échap fermer",
            panel.x + 20.0,
            panel.y + panel.h - 18.0,
            14.0,
            theme.muted(),
        );
    }
}

#[cfg(any(test, debug_assertions))]
impl AsciiApp {
    fn enter_installation_annex(&mut self) -> Result<(), String> {
        let anchor = self.installation_annex_anchor();
        self.walk_fixture_to(anchor)?;
        let outcome = self.execute_command(GameCommand::Interact { target: anchor });
        if outcome != CommandOutcome::Applied {
            return Err(format!("Annexe inaccessible : {outcome:?}"));
        }
        self.capture_events_at(Some(0.0));
        Ok(())
    }
}

#[cfg(debug_assertions)]
impl AsciiApp {
    pub async fn capture_installation_interactions(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(INITIAL_SEED, rules, texts, loot, expeditions)?;
        app.suspension_path = output.join("diagnostic.json");
        app.menu = MenuScreen::Hidden;
        app.enter_installation_annex()?;
        for (width, height) in [(1280.0, 800.0), (960.0, 600.0)] {
            request_new_screen_size(width, height);
            for _ in 0..5 {
                next_frame().await;
            }
            app.walk_fixture_to_unchecked(TERMINAL.step(Direction::North))?;
            app.open_installation_menu(TERMINAL);
            for _ in 0..3 {
                app.draw();
                next_frame().await;
            }
            app.draw();
            crate::ui_capture::framebuffer()?.export_png(
                output
                    .join(format!("renseignements-{width:.0}.png"))
                    .to_str()
                    .ok_or("Path")?,
            );
            app.installation_menu = None;
            app.walk_fixture_to_unchecked(PANEL.step(Direction::West))?;
            app.open_installation_menu(PANEL);
            for _ in 0..3 {
                app.draw();
                next_frame().await;
            }
            app.draw();
            crate::ui_capture::framebuffer()?.export_png(
                output
                    .join(format!("alimentation-{width:.0}.png"))
                    .to_str()
                    .ok_or("Path")?,
            );
            app.installation_menu = None;
        }
        app.walk_fixture_to_unchecked(POST.step(Direction::West))?;
        app.open_installation_menu(POST);
        for _ in 0..3 {
            app.draw();
            next_frame().await;
        }
        app.draw();
        crate::ui_capture::framebuffer()?
            .export_png(output.join("diversion-960.png").to_str().ok_or("Path")?);
        let before = app.game.recovery_snapshot_bytes()?;
        for _ in 0..8 {
            app.update_installation_menu(&InputFrame::default());
        }
        if before != app.game.recovery_snapshot_bytes()? {
            return Err("Menu altered the simulation".into());
        }
        app.installation_menu = None;
        app.walk_fixture_to_unchecked(PANEL.step(Direction::West))?;
        app.execute_command(GameCommand::UseInstallation {
            target: PANEL,
            action: InstallationAction::SetPower { powered: false },
        });
        app.capture_events_at(Some(0.0));
        for _ in 0..3 {
            app.draw();
            next_frame().await;
        }
        app.draw();
        crate::ui_capture::framebuffer()?.export_png(
            output
                .join("passage-coupe-960.png")
                .to_str()
                .ok_or("Path")?,
        );
        app.execute_command(GameCommand::UseInstallation {
            target: PANEL,
            action: InstallationAction::SetPower { powered: true },
        });
        app.capture_events_at(Some(0.0));
        request_new_screen_size(1600.0, 900.0);
        app.graphics.active.world_cell_px = 32;
        for _ in 0..5 {
            next_frame().await;
        }
        app.walk_fixture_to_unchecked(GridPos::new(11, 7))?;
        app.npc_vision_overlay_open = false; // Proposal capture only: no skill or setting is changed.
        let sounds = app.installation_sound_preview();
        if app.game.actor_observation_fields().is_empty() {
            return Err("No visible observer in range preview".into());
        }
        let before = app.game.recovery_snapshot_bytes()?;
        for _ in 0..3 {
            app.draw();
            app.draw_installation_sound_proposal(&sounds);
            next_frame().await;
        }
        app.draw();
        app.draw_installation_sound_proposal(&sounds);
        crate::ui_capture::framebuffer()?.export_png(
            output
                .join("proposition-portees-1600.png")
                .to_str()
                .ok_or("Path")?,
        );
        if before != app.game.recovery_snapshot_bytes()? {
            return Err("Range preview altered simulation".into());
        }
        app.execute_command(GameCommand::UseInstallation {
            target: POST,
            action: InstallationAction::ActivateDiversion,
        });
        app.capture_events_at(Some(0.0));
        if !app
            .game
            .actors()
            .iter()
            .any(|(entity, _)| app.game.actor_perception_signals(entity) == Some((false, true)))
        {
            return Err("Hearing pictogram has no real heard sound".into());
        }
        for _ in 0..3 {
            app.draw();
            app.draw_installation_sound_proposal(&sounds);
            next_frame().await;
        }
        app.draw();
        app.draw_installation_sound_proposal(&sounds);
        crate::ui_capture::framebuffer()?.export_png(
            output
                .join("proposition-entendu-1600.png")
                .to_str()
                .ok_or("Path")?,
        );
        for _ in 0..3 {
            app.execute_command(GameCommand::Wait);
            app.capture_events_at(Some(0.0));
        }
        // Leave the cover beside the post; the player's real concealment can
        // otherwise prevent detection even inside the geometric field.
        app.walk_fixture_to_unchecked(GridPos::new(12, 8))?;
        if !app.game.actors().iter().any(|(entity, _)| {
            app.game
                .actor_perception_signals(entity)
                .is_some_and(|(seen, _)| seen)
        }) {
            return Err(format!(
                "Eye pictogram has no real detection: player={:?}, actors={:?}",
                app.game.player_position(),
                app.game
                    .actors()
                    .iter()
                    .map(|(id, a)| (
                        a.position(),
                        a.ai_state(),
                        app.game.actor_perception_signals(id)
                    ))
                    .collect::<Vec<_>>()
            ));
        }
        for _ in 0..3 {
            app.draw();
            app.draw_installation_sound_proposal(&sounds);
            next_frame().await;
        }
        app.draw();
        app.draw_installation_sound_proposal(&sounds);
        crate::ui_capture::framebuffer()?.export_png(
            output
                .join("proposition-vu-1600.png")
                .to_str()
                .ok_or("Path")?,
        );
        std::fs::write(
            output.join("validation.json"),
            "{\"menu_without_turns\":true,\"range_preview_without_mutation\":true,\"hearing_from_real_sound\":true,\"eye_from_real_detection\":true}",
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}

// This rendering is a reviewable proposal, accessible only in native diagnostics.
// The game menu does not yet gain a sound overlay or bypass the existing vision skill.
#[cfg(debug_assertions)]
impl AsciiApp {
    fn installation_sound_preview(&self) -> BTreeSet<GridPos> {
        let Some(rules) = self.game.rules().stealth_rules else {
            return BTreeSet::new();
        };
        let Some((_, intensity, _)) = self
            .game
            .active_facility()
            .and_then(|f| f.diversion_at(POST))
        else {
            return BTreeSet::new();
        };
        (0..self.game.map().height() as i32)
            .flat_map(|y| (0..self.game.map().width() as i32).map(move |x| GridPos::new(x, y)))
            .filter(|&at| {
                self.game.player_visibility().is_visible(at)
                    && self.game.map().is_walkable(at)
                    && rules.sound_reaches_on_map(
                        self.game.map(),
                        intensity,
                        POST.step(Direction::East),
                        at,
                    )
            })
            .collect()
    }
    fn draw_installation_sound_proposal(&self, sounds: &BTreeSet<GridPos>) {
        for field in self.game.actor_observation_fields() {
            self.draw_circular_field_proposal(
                field.origin(),
                field.radius(),
                &field.positions().collect(),
                Color::from_rgba(55, 208, 174, 255),
            );
        }
        if let Some(rules) = self.game.rules().stealth_rules {
            if let Some((_, intensity, _)) = self
                .game
                .active_facility()
                .and_then(|f| f.diversion_at(POST))
            {
                let radius = (intensity - 1) / rules.sound_attenuation_per_cell;
                self.draw_circular_field_proposal(
                    POST.step(Direction::East),
                    radius,
                    sounds,
                    Color::from_rgba(240, 184, 79, 255),
                );
            }
        }
        for (entity, actor) in self.game.actors().iter() {
            let Some((seen, heard)) = self.game.actor_perception_signals(entity) else {
                continue;
            };
            if !seen && !heard {
                continue;
            }
            let Some(cell) = self.terminal.world_cell_rect(
                &self.game,
                self.terminal_bounds(),
                self.ui_scale(),
                self.graphics.active.world_cell_px,
                self.navigation_signal_summary().is_some(),
                actor.position(),
            ) else {
                continue;
            };
            let center = vec2(cell.x + cell.w * 0.5, cell.y - 13.0);
            let red = Color::from_rgba(255, 58, 73, 255);
            if seen {
                draw_ellipse_lines(center.x, center.y, 12.0, 7.0, 0.0, 2.6, red);
                draw_circle(center.x, center.y, 3.5, red);
            } else {
                draw_ellipse_lines(center.x + 1.0, center.y - 1.0, 7.0, 10.0, 0.0, 2.6, red);
                draw_line(
                    center.x + 2.0,
                    center.y - 5.0,
                    center.x - 2.0,
                    center.y + 1.0,
                    2.6,
                    red,
                );
                draw_line(
                    center.x - 2.0,
                    center.y + 1.0,
                    center.x + 2.0,
                    center.y + 5.0,
                    2.6,
                    red,
                );
                draw_line(
                    center.x - 11.0,
                    center.y - 4.0,
                    center.x - 11.0,
                    center.y + 4.0,
                    2.6,
                    red,
                );
            }
        }
        let y = screen_height() - 62.0;
        draw_rectangle(
            20.0,
            y,
            screen_width() - 40.0,
            44.0,
            Color::from_rgba(5, 7, 12, 255),
        );
        draw_text(
            "PROPOSITION · Vision : vert · Bruit du poste : ambre",
            32.0,
            y + 19.0,
            17.0,
            WHITE,
        );
        draw_text(
            "Ennemis visibles uniquement · Terrain visible uniquement · Aucune certitude de détection",
            32.0,
            y + 37.0,
            14.0,
            Color::from_rgba(172, 187, 200, 255),
        );
    }

    fn draw_circular_field_proposal(
        &self,
        origin: GridPos,
        radius: u16,
        positions: &BTreeSet<GridPos>,
        color: Color,
    ) {
        let rect_at = |at| {
            self.terminal.world_cell_rect(
                &self.game,
                self.terminal_bounds(),
                self.ui_scale(),
                self.graphics.active.world_cell_px,
                self.navigation_signal_summary().is_some(),
                at,
            )
        };
        let Some(origin_cell) = rect_at(origin) else {
            return;
        };
        let center = origin_cell.center();
        // Same conservative analytic edge as the player's visibility circle.
        let r = (f32::from(radius) - std::f32::consts::FRAC_1_SQRT_2).max(0.5) * origin_cell.w;
        let circle: Vec<_> = (0..96)
            .map(|i| {
                let angle = i as f32 * std::f32::consts::TAU / 96.0;
                center + vec2(angle.cos(), angle.sin()) * r
            })
            .collect();
        for &at in positions {
            if !self.game.player_visibility().is_visible(at) || !self.game.map().is_walkable(at) {
                continue;
            }
            let Some(cell) = rect_at(at) else {
                continue;
            };
            let mut polygon = vec![
                vec2(cell.x, cell.y),
                vec2(cell.x + cell.w, cell.y),
                vec2(cell.x + cell.w, cell.y + cell.h),
                vec2(cell.x, cell.y + cell.h),
            ];
            for edge in 0..circle.len() {
                if polygon.is_empty() {
                    break;
                }
                let a = circle[edge];
                let b = circle[(edge + 1) % circle.len()];
                let line = b - a;
                let side = |p: Vec2| line.perp_dot(p - a);
                let mut clipped = Vec::new();
                let mut previous = *polygon.last().unwrap();
                let mut previous_side = side(previous);
                for &current in &polygon {
                    let current_side = side(current);
                    if (previous_side >= 0.0) != (current_side >= 0.0) {
                        clipped.push(
                            previous
                                + (current - previous)
                                    * (previous_side / (previous_side - current_side)),
                        );
                    }
                    if current_side >= 0.0 {
                        clipped.push(current);
                    }
                    previous = current;
                    previous_side = current_side;
                }
                polygon = clipped;
            }
            for i in 1..polygon.len().saturating_sub(1) {
                draw_triangle(
                    polygon[0],
                    polygon[i],
                    polygon[i + 1],
                    Color::new(color.r, color.g, color.b, 0.15),
                );
            }
        }
        for i in 0..circle.len() {
            let a = circle[i];
            let b = circle[(i + 1) % circle.len()];
            let at = |p: Vec2| {
                GridPos::new(
                    origin.x + ((p.x - center.x) / origin_cell.w).round() as i32,
                    origin.y + ((p.y - center.y) / origin_cell.h).round() as i32,
                )
            };
            if [at(a), at(b)].into_iter().all(|p| {
                positions.contains(&p)
                    && self.game.player_visibility().is_visible(p)
                    && self.game.map().is_walkable(p)
            }) {
                draw_line(
                    a.x,
                    a.y,
                    b.x,
                    b.y,
                    1.4,
                    Color::new(color.r, color.g, color.b, 0.85),
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn app(version: u8) -> AsciiApp {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, version).unwrap()
    }
    #[test]
    fn installations_new_generation_is_optional_and_old_runs_keep_their_map() {
        let old = app(IRREVERSIBLE_LAYERS_GENERATION_VERSION);
        assert!(old.game.zone_info(&cid("core:transit_annex")).is_none());
        let mut new = app(CURRENT_GENERATION_VERSION);
        assert!(new.game.zone_info(&cid("core:transit_annex")).is_some());
        new.enter_installation_annex().unwrap();
        assert!(
            !new.game
                .player_may_take_property_of(&cid("core:system_security"))
        );
        assert!(
            !new.game
                .passage_is_irreversible(new.game.passage(ENTRANCE).unwrap())
        );
        new.walk_fixture_to_unchecked(TERMINAL.step(Direction::North))
            .unwrap();
        let before = new.game.recovery_snapshot_bytes().unwrap();
        assert!(new.open_installation_menu(TERMINAL));
        new.update_installation_menu(&InputFrame::default());
        assert_eq!(before, new.game.recovery_snapshot_bytes().unwrap());
        new.installation_menu = None;
        assert_eq!(
            new.execute_command(GameCommand::UseInstallation {
                target: TERMINAL,
                action: InstallationAction::ReadIntel { index: 0 }
            }),
            CommandOutcome::Applied
        );
        new.game.drain_events();
        let expected = new.game.recovery_snapshot_bytes().unwrap();
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut saved = new.suspension().unwrap();
        saved.build = "0000000000000000".into();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(expected, restored.game.recovery_snapshot_bytes().unwrap());
    }
}
