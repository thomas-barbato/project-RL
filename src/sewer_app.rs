//! Entry/return test fixture for the accepted sewer integration.
use super::*;

#[cfg(any(test, debug_assertions))]
impl AsciiApp {
    pub fn new_sewer_trial() -> Result<Self, String> {
        let requested = std::env::args()
            .find_map(|a| {
                a.strip_prefix("--variante-egouts=")
                    .and_then(|v| v.parse::<usize>().ok())
            })
            .unwrap_or(1);
        Self::sewer_trial_start(requested, false)
    }
    fn sewer_trial_variant(index: usize) -> Result<Self, String> {
        Self::sewer_trial_start(index, true)
    }
    // Isolated near-hatch fixture: exercises the real current-version entry
    // without simulating a combat march through the populated city.
    pub(super) fn connected_sewer_fixture(index: usize) -> Result<Self, String> {
        Self::isolated_sewer_fixture(index, CURRENT_GENERATION_VERSION)
    }
    fn isolated_sewer_fixture(index: usize, version: u8) -> Result<Self, String> {
        let seed = (1872..2000)
            .find(|&seed| crate::sewer::variant(seed) == index)
            .ok_or("Unknown sewer index")?;
        let (mut rules, texts, loot, expeditions) = ascii_game_content()?;
        let classes = ascii_character_class_catalog()?;
        let id: CharacterClassId = "core:bastion".parse().unwrap();
        let class = classes.get(&id).ok_or("Bastion profile absent")?;
        class
            .apply_to_rules(&mut rules, class.recommended_attributes())
            .map_err(|e| e.to_string())?;
        weapon_supplies_app::ensure_class_weapon_supplies(&mut rules, version);
        let mut app = Self::from_seed_version(seed, rules, texts, loot, expeditions, version)?;
        app.character_class = Some(id);
        app.select_weapon_slot(app.preferred_starting_weapon_slot());
        app.crash_recovery_enabled = false;
        app.suspension_path =
            std::path::PathBuf::from("artifacts/sewer-water-review-01/isolated-resume.json");
        let info =
            crate::prison_street::info(app.game.current_zone().ok_or("Camp absent")?.clone());
        let mut street =
            crate::prison_street::blueprint(info.clone(), seed, true, true, true, true, true)?;
        crate::sewer::prepare_street(&mut street.map)?;
        let game = GameState::new_with_rules(
            street.map,
            GridPos::new(2, 15),
            app.seed,
            app.game.rules().clone(),
        )
        .map_err(|e| e.to_string())?;
        app.game = WorldState::single(game);
        app.game.enable(info.clone())?;
        let connection = if version >= CONNECTED_SEWER_GENERATION_VERSION {
            crate::sewer::connected_connection(app.seed, info.depth)
        } else {
            crate::sewer::connection(app.seed, info.depth)
        };
        app.game.declare_deferred_connection_at(
            info.id,
            connection.at,
            connection.destination,
            connection.arrival,
        )?;
        app.terminal = TerminalView::new(
            SectorDecor::default(),
            app.game.map(),
            app.game.player_visibility(),
        );
        app.zone_decor.clear();
        app.zone_views.clear();
        app.log.clear();
        app.refresh_zone_title();
        app.combat_motion = crate::combat_visuals::CombatMotion::default();
        app.open_menu(MenuScreen::Hidden);
        app.character_creation = None;
        app.graphics.active.world_cell_px = 64;
        app.game.drain_events();
        app.capture_events_at(Some(0.));
        Ok(app)
    }
    pub(super) fn sewer_trial_start(index: usize, near_hatch: bool) -> Result<Self, String> {
        if !(1..=3).contains(&index) {
            return Err("Sewer variant must be 1, 2 or 3".into());
        }
        let seed = (1872..2000)
            .find(|&seed| crate::sewer::variant(seed) == index)
            .unwrap();
        let (mut rules, texts, loot, expeditions) = ascii_game_content()?;
        let classes = ascii_character_class_catalog()?;
        let id: CharacterClassId = "core:bastion".parse().unwrap();
        let class = classes.get(&id).ok_or("Bastion profile absent")?;
        class
            .apply_to_rules(&mut rules, class.recommended_attributes())
            .map_err(|e| e.to_string())?;
        weapon_supplies_app::ensure_class_weapon_supplies(&mut rules, CURRENT_GENERATION_VERSION);
        // The historical integration fixture retains its original empty street.
        // The interactive trial uses today's populated city and starts at its
        // safe south gate; walking to the hatch now requires actual play.
        let mut app = Self::from_seed_version(
            seed,
            rules,
            texts,
            loot,
            expeditions,
            if near_hatch {
                SEWER_GENERATION_VERSION
            } else {
                CURRENT_GENERATION_VERSION
            },
        )?;
        app.character_class = Some(id);
        app.select_weapon_slot(app.preferred_starting_weapon_slot());
        app.crash_recovery_enabled = false;
        let folder =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("artifacts/sewer-playtest-04");
        std::fs::create_dir_all(&folder).map_err(|e| e.to_string())?;
        app.suspension_path = folder.join(format!("trial-{index}.json"));
        let exit = crate::cryogenic_camp::pos(crate::cryogenic_camp::scene().exits[0]);
        app.walk_fixture_to_unchecked(exit)?;
        if app.execute_command(GameCommand::Interact { target: exit }) != CommandOutcome::Applied {
            return Err("Sewer trial street entry failed".into());
        }
        if near_hatch {
            app.walk_fixture_to_unchecked(GridPos::new(2, 15))?;
        }
        app.open_menu(MenuScreen::Hidden);
        app.character_creation = None;
        app.selected_target = None;
        app.selected_world_cell = None;
        app.world_cursor = None;
        app.graphics.active.world_cell_px = 64;
        app.refresh_zone_title();
        app.terminal
            .observe(app.game.map(), app.game.player_visibility());
        app.game.drain_events();
        app.log.clear();
        Ok(app)
    }

    #[cfg(debug_assertions)]
    pub async fn capture_sewer_integration(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        macroquad::miniquad::window::set_fullscreen(false);
        request_new_screen_size(1920., 1080.);
        for _ in 0..3 {
            next_frame().await;
        }
        let mut records = Vec::new();
        for index in 1..=3 {
            let mut app = Self::sewer_trial_variant(index)?;
            app.world_fix_frame(output, &format!("variant-{index}-street-entry"))
                .await?;
            if app.execute_command(GameCommand::Interact {
                target: crate::sewer::ENTRY,
            }) != CommandOutcome::Applied
            {
                return Err("Sewer entry command failed".into());
            }
            app.log.clear();
            app.selected_world_cell = None;
            app.selected_target = None;
            app.world_cursor = None;
            app.world_fix_frame(output, &format!("variant-{index}-arrival"))
                .await?;
            let plan = crate::sewer::plan(index);
            let count = app
                .game
                .actors()
                .iter()
                .filter(|(id, _)| *id != app.game.player_id())
                .count();
            if count != plan.groups.iter().map(|g| g.members.len()).sum::<usize>() {
                return Err("Sewer population count mismatch".into());
            }
            let saved = app.suspension()?;
            let (rules, texts, loot, expeditions) = ascii_game_content()?;
            let restored = Self::restore_suspension(&saved, rules, texts, loot, expeditions)?;
            if suspension::fingerprint(&restored.game) != suspension::fingerprint(&app.game) {
                return Err("Sewer snapshot/replay mismatch".into());
            }
            app.walk_fixture_to_unchecked(crate::prison_street::pos(plan.spawn))?;
            if app.execute_command(GameCommand::Interact {
                target: crate::prison_street::pos(plan.spawn),
            }) != CommandOutcome::Applied
            {
                return Err("Sewer return failed".into());
            }
            app.log.clear();
            app.world_fix_frame(output, &format!("variant-{index}-return"))
                .await?;
            // Close-up visual fixtures use the shipped map and profiles,
            // while the real entry/return session above remains untouched.
            for name in ["refuge", "workshop", "encounter"] {
                if let Some(view) = plan.views.iter().find(|v| v.id == name) {
                    let mut sample = Self::sewer_trial_variant(index)?;
                    let info = crate::sewer::connection(sample.seed, 1).destination;
                    let blueprint = crate::sewer::blueprint(
                        info.clone(),
                        sample.seed,
                        &sample.regional_worlds,
                    )?;
                    let game = GameState::new_with_rules(
                        blueprint.map,
                        GridPos::new(view.at[0], view.at[1]),
                        blueprint.seed,
                        sample.game.rules().clone(),
                    )
                    .map_err(|e| e.to_string())?;
                    sample.terminal = TerminalView::new(
                        SectorDecor::default(),
                        game.map(),
                        game.player_visibility(),
                    );
                    sample.game = WorldState::single(game);
                    sample.game.enable(info)?;
                    sample.refresh_zone_title();
                    for actor in blueprint.actors {
                        sample.game.spawn_actor(actor).map_err(|e| e.to_string())?;
                    }
                    sample.game.drain_events();
                    sample.log.clear();
                    sample.world_cursor = None;
                    sample.selected_target = None;
                    sample.selected_world_cell = None;
                    sample
                        .world_fix_frame(output, &format!("variant-{index}-{name}"))
                        .await?;
                }
            }
            records.push(serde_json::json!({"variant":index,"monsters":count,"snapshot_restore":true,"entry_and_return":true}));
        }
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":"EG-COMP-04 / EG-POP-04","integration":true,"generation":SEWER_GENERATION_VERSION,"variants":records,"resurrection_active":false})).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }

    #[cfg(debug_assertions)]
    pub async fn capture_sewer_canals_integration(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1280., 800.);
        for _ in 0..3 {
            next_frame().await;
        }
        let mut rows = Vec::new();
        for index in 1..=3 {
            let mut app = Self::connected_sewer_fixture(index)?;
            if app.execute_command(GameCommand::Interact {
                target: crate::sewer::ENTRY,
            }) != CommandOutcome::Applied
            {
                return Err("Connected sewer entry failed".into());
            }
            if !crate::sewer::connected_id(app.game.current_zone().unwrap().id.as_str()) {
                return Err("New game used a legacy sewer".into());
            }
            app.world_fix_frame(output, &format!("variant-{index}-arrival"))
                .await?;
            app.capture_events_at(Some(0.));
            let saved = app.suspension()?;
            let (r, t, l, e) = ascii_game_content()?;
            let restored = Self::restore_suspension(&saved, r, t, l, e)?;
            if suspension::fingerprint(&restored.game) != suspension::fingerprint(&app.game) {
                return Err("Connected sewer resume changed world".into());
            }
            let exit = crate::prison_street::pos(crate::sewer::connected_plan(index).spawn);
            if app.execute_command(GameCommand::Interact { target: exit })
                != CommandOutcome::Applied
            {
                return Err("Connected sewer return failed".into());
            }
            app.world_fix_frame(output, &format!("variant-{index}-return"))
                .await?;
            let info = crate::sewer::connected_connection(app.seed, 1).destination;
            let blueprint = crate::sewer::blueprint(info.clone(), app.seed, &app.regional_worlds)?;
            let at = if index == 3 {
                GridPos::new(13, 35)
            } else {
                GridPos::new(22, 19)
            };
            let game = GameState::new_with_rules(
                blueprint.map,
                at,
                blueprint.seed,
                app.game.rules().clone(),
            )
            .map_err(|e| e.to_string())?;
            app.game = WorldState::single(game);
            app.game.enable(info)?;
            for actor in blueprint.actors {
                app.game.spawn_actor(actor).map_err(|e| e.to_string())?;
            }
            app.game.drain_events();
            app.terminal = TerminalView::new(
                SectorDecor::default(),
                app.game.map(),
                app.game.player_visibility(),
            );
            app.log.clear();
            app.refresh_zone_title();
            app.world_fix_frame(output, &format!("variant-{index}-channels"))
                .await?;
            app.graphics.active.reduced_motion = true;
            app.world_fix_frame(output, &format!("variant-{index}-reduced-motion"))
                .await?;
            rows.push(serde_json::json!({"variant":index,"normal_renderer":true,"connected_zone_id":true,"state_unchanged_during_render":true,"entry_return_and_resume":true,"monsters_and_player_walkable":app.game.actors().iter().all(|(_,a)|app.game.map().is_walkable(a.position()))}));
        }
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":"EG-CAN02-INTEGRATION01","generation":CURRENT_GENERATION_VERSION,"variants":rows,"saved_campaign_access":false,"settings_written":false,"scope":"Real entry/return from isolated near-hatch street fixture, resume and native production-renderer views; no city combat traversal or balance claim"})).map_err(|e|e.to_string())?).map_err(|e|e.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
#[path = "sewer_app_tests.rs"]
mod tests;
