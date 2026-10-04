//! Native captures of the ordinary renderer, real skill unlock and F2 input.
use super::*;
use project_rl::facility::InstallationAction;

impl AsciiApp {
    fn prepare_tactical_reading_trial(output: &std::path::Path) -> Result<Self, String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (mut rules, texts, loot, expeditions) = ascii_game_content()?;
        // Only this diagnostic speeds up learning; the actual technique and
        // production input/render paths are exercised without bypassing REC-06.
        rules.progression.curve =
            project_rl::progression::ExperienceCurve::new(vec![1]).map_err(|e| e.to_string())?;
        rules.progression.skill_points_per_level = 1;
        rules.hit_rules = None;
        let mut app = Self::from_seed(INITIAL_SEED, rules, texts, loot, expeditions)?;
        app.suspension_path = output.join("diagnostic.json");
        app.menu = MenuScreen::Hidden;
        let input = InputFrame {
            pressed: [app.controls.binding(Action::NpcVision).clone()]
                .into_iter()
                .collect(),
            ..InputFrame::default()
        };
        app.update_input(&input);
        if app.npc_vision_overlay_open {
            return Err("REC-06 n’a pas bloqué F2".into());
        }
        app.enter_installation_annex()?;
        let player = app.game.player_position().ok_or("Joueur absent")?;
        let at = player
            .cardinal_neighbors()
            .into_iter()
            .find(|&at| app.game.map().is_walkable(at) && app.game.actors().entity_at(at).is_none())
            .ok_or("Case de diagnostic absente")?;
        let target = app
            .game
            .spawn_actor(
                Actor::new(at, 1)
                    .map_err(|e| e.to_string())?
                    .with_player_relation(PlayerRelation::Hostile)
                    .with_defeat_reward(DefeatReward::persistent(10, 1)),
            )
            .map_err(|e| e.to_string())?;
        let outcome = app.execute_command(GameCommand::Attack { slot: 0, target });
        if outcome != CommandOutcome::Applied {
            return Err(format!(
                "Le gain de niveau du diagnostic a échoué : {outcome:?}"
            ));
        }
        for name in ["core:rec_01", "core:rec_06"] {
            if app.execute_command(GameCommand::LearnTechnique {
                technique: technique_id(name).ok_or("Technique absente")?,
            }) != CommandOutcome::AppliedWithoutTime
            {
                return Err(format!("Apprentissage du diagnostic impossible : {name}"));
            }
        }
        app.level_up_notice = None;
        let post = GridPos::new(10, 7);
        app.walk_fixture_to_unchecked(post.step(Direction::East))?;
        if app.execute_command(GameCommand::UseInstallation {
            target: post,
            action: InstallationAction::ActivateDiversion,
        }) != CommandOutcome::Applied
        {
            return Err("Diversion inactive".into());
        }
        app.capture_events_at(Some(0.0));
        app.level_up_notice = None;
        app.floating_messages.clear();
        app.graphics.active.world_cell_px = 32;
        Ok(app)
    }

    pub fn new_tactical_reading_trial() -> Result<Self, String> {
        let output =
            std::path::Path::new("artifacts/lecture-tactique-integration-2026-10-04/essai");
        let mut app = Self::prepare_tactical_reading_trial(output)?;
        app.npc_vision_overlay_open = true;
        app.push_log(
            "ESSAI · F2 : vision / bruit · E : installations · Attendre : expiration du bruit"
                .into(),
        );
        Ok(app)
    }

    pub async fn capture_tactical_reading_integration(
        output: &std::path::Path,
    ) -> Result<(), String> {
        let mut app = Self::prepare_tactical_reading_trial(output)?;
        let input = InputFrame {
            pressed: [app.controls.binding(Action::NpcVision).clone()]
                .into_iter()
                .collect(),
            ..InputFrame::default()
        };
        let before = app.game.recovery_snapshot_bytes()?;
        app.update_input(&input);
        if !app.npc_vision_overlay_open {
            return Err("F2 appris n’affiche pas les zones".into());
        }
        {
            let mut cache = app.tactical_projection.borrow_mut();
            let tactical = cache.projection(&app.game, true);
            if tactical.vision.is_empty()
                || tactical.sounds.is_empty()
                || !tactical.signals.iter().any(|s| s.heard)
            {
                return Err("Les perceptions réelles sont absentes de l’affichage".into());
            }
        }
        let mut render_performance = Vec::new();
        for (width, height) in [(1600.0, 900.0), (960.0, 600.0)] {
            request_new_screen_size(width, height);
            for _ in 0..5 {
                next_frame().await;
            }
            app.save_tactical_frame(output, &format!("vision-et-bruit-{width:.0}.png"))
                .await?;
            if width == 1600.0 {
                render_performance.push(app.measure_tactical_render(true).await);
            }
            app.update_input(&input);
            if app.npc_vision_overlay_open {
                return Err("F2 n’a pas masqué les zones".into());
            }
            {
                let mut cache = app.tactical_projection.borrow_mut();
                let tactical = cache.projection(&app.game, false);
                if !tactical.vision.is_empty()
                    || !tactical.sounds.is_empty()
                    || !tactical.signals.iter().any(|s| s.heard)
                {
                    return Err("L’alerte disparaît avec les zones".into());
                }
            }
            app.save_tactical_frame(output, &format!("zones-masquees-{width:.0}.png"))
                .await?;
            if width == 1600.0 {
                render_performance.push(app.measure_tactical_render(false).await);
            }
            app.update_input(&input);
        }
        if before != app.game.recovery_snapshot_bytes()? {
            return Err("F2 ou le rendu a modifié la simulation".into());
        }
        for _ in 0..3 {
            app.execute_command(GameCommand::Wait);
            app.capture_events_at(Some(0.0));
        }
        if !app.game.sound_observation_sources().is_empty() {
            return Err("Le bruit n’a pas expiré".into());
        }
        request_new_screen_size(1600.0, 900.0);
        for _ in 0..5 {
            next_frame().await;
        }
        app.floating_messages.clear();
        app.save_tactical_frame(output, "bruit-expire.png").await?;
        app.walk_fixture_to_unchecked(GridPos::new(12, 8))?;
        app.capture_events_at(Some(0.0));
        app.floating_messages.clear();
        if !app.game.actors().iter().any(|(entity, _)| {
            app.game
                .actor_perception_signals(entity)
                .is_some_and(|(seen, _)| seen)
        }) {
            return Err("L’alerte visuelle n’a pas de détection réelle".into());
        }
        app.save_tactical_frame(output, "joueur-repere.png").await?;
        std::fs::write(
            output.join("render-performance.json"),
            serde_json::to_vec_pretty(&render_performance).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        std::fs::write(output.join("validation.json"),
            "{\"ordinary_renderer\":true,\"rec06_gate_and_real_unlock\":true,\"configured_f2_toggles_both_fields\":true,\"red_cue_survives_hidden_fields\":true,\"display_preserves_snapshot\":true,\"emission_expires\":true,\"real_optical_detection\":true}")
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn save_tactical_frame(
        &self,
        output: &std::path::Path,
        name: &str,
    ) -> Result<(), String> {
        for _ in 0..3 {
            self.draw();
            next_frame().await;
        }
        self.draw();
        crate::ui_capture::framebuffer()?.export_png(output.join(name).to_str().ok_or("Path")?);
        Ok(())
    }

    async fn measure_tactical_render(&self, shown: bool) -> serde_json::Value {
        let mut samples = Vec::new();
        for _ in 0..120 {
            let start = std::time::Instant::now();
            self.draw();
            samples.push(start.elapsed().as_secs_f64() * 1000.0);
            next_frame().await;
        }
        let mut steady = samples[10..].to_vec();
        steady.sort_by(f64::total_cmp);
        serde_json::json!({"scene":"annexe-diversion", "zones_shown":shown,
            "build":"debug", "frames":samples.len(), "cpu_median_ms":steady[steady.len()/2],
            "cpu_p95_ms":steady[steady.len()*95/100], "cpu_max_ms":steady.last()})
    }
}
