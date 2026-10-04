//! Isolated, reviewable F2 proposal. Ordinary controls and REC-06 are unchanged.
use super::*;
use project_rl::facility::InstallationAction;
use std::collections::BTreeSet;

const POST: GridPos = GridPos::new(10, 7);

impl AsciiApp {
    fn prepare_tactical_reading_proposal() -> Result<(Self, BTreeSet<GridPos>), String> {
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(INITIAL_SEED, rules, texts, loot, expeditions)?;
        app.menu = MenuScreen::Hidden;
        app.enter_installation_annex()?;
        app.walk_fixture_to_unchecked(POST.step(Direction::East))?;
        app.npc_vision_overlay_open = false;
        app.graphics.active.world_cell_px = 32;
        if app.execute_command(GameCommand::UseInstallation {
            target: POST,
            action: InstallationAction::ActivateDiversion,
        }) != CommandOutcome::Applied
        {
            return Err("Le poste de l’aperçu n’a pas émis de bruit.".into());
        }
        app.capture_events_at(Some(0.0));
        if !app
            .game
            .actors()
            .iter()
            .any(|(entity, _)| app.game.actor_perception_signals(entity) == Some((false, true)))
        {
            return Err("L’aperçu ne contient pas d’hostile ayant entendu le poste.".into());
        }
        let sounds = app.installation_sound_preview();
        if sounds.is_empty() || app.game.actor_observation_fields().is_empty() {
            return Err("Les portées de l’aperçu sont absentes.".into());
        }
        Ok((app, sounds))
    }

    fn tactical_proposal_button() -> Rect {
        Rect::new(24.0, screen_height() - 66.0, 214.0, 32.0)
    }

    fn draw_tactical_reading_proposal(&self, sounds: &BTreeSet<GridPos>, shown: bool) {
        self.draw();
        if shown {
            for field in self.game.actor_observation_fields() {
                self.draw_circular_field_proposal(
                    field.origin(),
                    field.radius(),
                    &field.positions().collect(),
                    Color::from_rgba(55, 208, 174, 255),
                );
            }
            let intensity = self
                .game
                .active_facility()
                .unwrap()
                .diversion_at(POST)
                .unwrap()
                .1;
            let attenuation = self
                .game
                .rules()
                .stealth_rules
                .unwrap()
                .sound_attenuation_per_cell;
            self.draw_circular_field_proposal(
                POST.step(Direction::East),
                (intensity - 1) / attenuation,
                sounds,
                Color::from_rgba(240, 184, 79, 255),
            );
        }
        // The warnings stay visible when the optional tactical ranges are hidden.
        self.draw_perception_icons_proposal();
        let y = screen_height() - 72.0;
        draw_rectangle(20.0, y, screen_width() - 40.0, 62.0, UiTheme.surface());
        let button = Self::tactical_proposal_button();
        let action_label = format!(
            "{} · {} les zones",
            self.controls.label(Action::NpcVision),
            if shown { "Masquer" } else { "Afficher" },
        );
        UiTheme.button(
            button,
            &action_label,
            false,
            false,
            true,
            ButtonTone::Secondary,
        );
        draw_text(
            if shown {
                "LECTURE TACTIQUE · Vision : vert · Bruit actif : ambre"
            } else {
                "LECTURE TACTIQUE · Zones masquées · Alertes rouges conservées"
            },
            250.0,
            y + 25.0,
            17.0,
            UiTheme.text(),
        );
        draw_text(
            "APERÇU · Scène figée · Après REC-06 · Le bruit part du poste ; les murs l’atténuent. · Échap : fermer",
            32.0,
            y + 51.0,
            14.0,
            UiTheme.muted(),
        );
    }

    pub async fn run_tactical_reading_proposal() -> Result<(), String> {
        request_new_screen_size(1600.0, 900.0);
        for _ in 0..5 {
            next_frame().await;
        }
        let (app, sounds) = Self::prepare_tactical_reading_proposal()?;
        let before = app.game.recovery_snapshot_bytes()?;
        let mut shown = true;
        loop {
            let input = InputFrame::capture();
            if is_quit_requested() || is_key_pressed(KeyCode::Escape) {
                break;
            }
            if app.controls.pressed(Action::NpcVision, &input)
                || (is_mouse_button_pressed(MouseButton::Left)
                    && Self::tactical_proposal_button()
                        .contains(vec2(mouse_position().0, mouse_position().1)))
            {
                shown = !shown;
                if before != app.game.recovery_snapshot_bytes()? {
                    return Err("L’affichage a modifié la scène d’aperçu.".into());
                }
            }
            app.draw_tactical_reading_proposal(&sounds, shown);
            next_frame().await;
        }
        Ok(())
    }

    pub async fn capture_tactical_reading_proposal(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1600.0, 900.0);
        for _ in 0..5 {
            next_frame().await;
        }
        let (app, sounds) = Self::prepare_tactical_reading_proposal()?;
        let before = app.game.recovery_snapshot_bytes()?;
        for (shown, name) in [(true, "zones-affichees.png"), (false, "zones-masquees.png")] {
            for _ in 0..3 {
                app.draw_tactical_reading_proposal(&sounds, shown);
                next_frame().await;
            }
            app.draw_tactical_reading_proposal(&sounds, shown);
            crate::ui_capture::framebuffer()?.export_png(output.join(name).to_str().ok_or("Path")?);
            if before != app.game.recovery_snapshot_bytes()? {
                return Err("L’affichage a modifié la scène d’aperçu.".into());
            }
        }
        std::fs::write(output.join("validation.json"),
            "{\"ranges_from_engine\":true,\"hostile_heard_real_sound\":true,\"display_without_simulation_mutation\":true,\"ordinary_f2_unchanged\":true}")
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
