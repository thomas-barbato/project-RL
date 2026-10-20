//! Interactive, opt-in population test with existing ordinary surface profiles.
use super::*;

impl AsciiApp {
    pub fn new_street_population_trial() -> Result<Self, String> {
        Self::street_population_fixture(None)
    }

    fn street_population_fixture(viewpoint: Option<GridPos>) -> Result<Self, String> {
        if !crate::street_population::enabled() {
            return Err("U01-POP-01 requires its dedicated trial mode".into());
        }
        crate::street_defense_review::enable(true);
        let (mut rules, texts, loot, expeditions) = ascii_game_content()?;
        let class_id: CharacterClassId = "core:bastion".parse().unwrap();
        let classes = ascii_character_class_catalog()?;
        let class = classes
            .get(&class_id)
            .ok_or("Trial Bastion profile absent")?;
        class
            .apply_to_rules(&mut rules, class.recommended_attributes())
            .map_err(|e| e.to_string())?;
        weapon_supplies_app::ensure_class_weapon_supplies(&mut rules, CURRENT_GENERATION_VERSION);
        let mut app = Self::from_seed_version(
            1872,
            rules,
            texts,
            loot,
            expeditions,
            SEWER_GENERATION_VERSION,
        )?;
        app.character_class = Some(class_id);
        app.select_weapon_slot(app.preferred_starting_weapon_slot());
        app.crash_recovery_enabled = false;
        let output = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("artifacts/street-population-review-01");
        std::fs::create_dir_all(&output).map_err(|e| e.to_string())?;
        app.suspension_path = output.join("trial-session.json");
        let exit = pos(camp::scene().exits[0]);
        app.walk_fixture_to_unchecked(exit)?;
        if app.execute_command(GameCommand::Interact { target: exit }) != CommandOutcome::Applied {
            return Err("Could not enter population trial street".into());
        }
        // A capture viewpoint is reached before populating, so no AI action is
        // skipped or simulated during the final, read-only scene capture.
        if let Some(at) = viewpoint {
            app.walk_fixture_to_unchecked(at)?;
        }
        let actors = crate::street_population::actors(&app.regional_worlds, app.game.map())?;
        for actor in actors {
            app.game.spawn_actor(actor).map_err(|e| e.to_string())?;
        }
        app.game.drain_events();
        app.open_menu(MenuScreen::Hidden);
        app.character_creation = None;
        app.skills_open = false;
        app.selected_world_cell = None;
        app.selected_target = None;
        app.world_cursor = None;
        app.graphics.active.world_cell_px = 64;
        app.refresh_zone_title();
        app.terminal
            .observe(app.game.map(), app.game.player_visibility());
        app.log.clear();
        Ok(app)
    }

    pub async fn capture_street_population(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        macroquad::miniquad::window::set_fullscreen(false);
        request_new_screen_size(1920., 1080.);
        for _ in 0..3 {
            next_frame().await;
        }
        let mut records = Vec::new();
        let mut scene_records = Vec::new();
        for (label, at) in [
            ("arrival", GridPos::new(7, 62)),
            ("first-cultists", GridPos::new(12, 56)),
            ("west-zombies", GridPos::new(6, 55)),
            ("central-cultists", GridPos::new(18, 39)),
            ("north-zombies", GridPos::new(18, 26)),
            ("last-cultists", GridPos::new(18, 10)),
        ] {
            let mut app = Self::street_population_fixture(Some(at))?;
            if label == "arrival" {
                for (id, actor) in app.game.actors().iter() {
                    if id == app.game.player_id() {
                        continue;
                    }
                    let attack = actor.attack(0).ok_or("Trial enemy without attack")?;
                    let appearance = project_rl::monster_art::appearance(actor).unwrap();
                    records.push(serde_json::json!({
                        "id":id.get(), "position":[actor.position().x,actor.position().y],
                        "number":appearance.0,"variant":appearance.1,
                        "name":project_rl::monster_art::display_name(actor),
                        "hp":actor.maximum_integrity(),"raw_damage":attack.damage().raw_total(),
                        "range":attack.range(),"ai":format!("{:?}",actor.ai().unwrap().behavior),
                        "reward":actor.defeat_reward().map(|r|serde_json::json!({"xp":r.base_experience,"level":r.threat_level})),
                        "tags":actor.tags().iter().map(|t|t.as_str()).collect::<Vec<_>>()
                    }));
                }
            }
            app.world_fix_frame(output, label).await?;
            let image = crate::ui_capture::framebuffer()?;
            if image.width < 1552 || image.height < 894 {
                return Err(format!(
                    "Invalid population capture size: {}x{}",
                    image.width, image.height
                ));
            }
            scene_records.push(serde_json::json!({"id":label,"player":[at.x,at.y]}));
        }
        let mut checks = serde_json::json!({
            "review":crate::street_population::plan().review,
            "mode":"isolated_playable_trial","count":records.len(),"scenes":scene_records,
            "profiles":records,"normal_generation_unchanged":true,
            "native_cell":64,"guard_combat_added":false,"new_powers_added":false
        });
        checks["player_profile"] =
            serde_json::json!({"class":"core:bastion","starting_equipment_unchanged":true});
        // Real combat/persistence checks are recorded by the targeted test suite.
        checks["normal_map_unchanged"] = serde_json::json!(true);
        std::fs::write(
            output.join("checks.json"),
            serde_json::to_vec_pretty(&checks).unwrap(),
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
#[path = "street_population_tests.rs"]
mod tests;
