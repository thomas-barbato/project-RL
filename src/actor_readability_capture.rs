//! Explicit production-renderer checks, isolated from the user's campaigns/settings.
use super::*;
use project_rl::world::Map;

async fn save(
    app: &AsciiApp,
    output: &std::path::Path,
    name: &str,
    time: f64,
) -> Result<(), String> {
    let _frame = crate::effect_geometry::frame_time(time);
    crate::camp_npc_review::set_capture_time(time);
    for _ in 0..3 {
        app.draw();
        next_frame().await;
    }
    app.draw();
    crate::ui_capture::framebuffer()?.export_png(
        output
            .join(format!("{name}.png"))
            .to_str()
            .ok_or("capture path")?,
    );
    Ok(())
}

impl AsciiApp {
    pub async fn capture_actor_readability(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1280., 800.);
        for _ in 0..3 {
            next_frame().await;
        }
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(INITIAL_SEED, rules, texts, loot, expeditions)?;
        app.suspension_path = output.join("isolated-run.json");
        app.begin_character_creation_with_seed(false, app.seed)?;
        let index = app
            .character_classes
            .iter()
            .position(|(id, _)| id.as_str() == "core:bastion")
            .ok_or("Bastion missing")?;
        let attributes = app
            .character_classes
            .iter()
            .nth(index)
            .unwrap()
            .1
            .recommended_attributes();
        let creation = app.character_creation.as_mut().unwrap();
        creation.selected_class = index;
        creation.attributes = attributes;
        let creation = app.character_creation.take().unwrap();
        app.rebuild_run_with_character_class(&creation)?;
        app.menu = MenuScreen::Hidden;
        let fingerprint = suspension::fingerprint(&app.game);
        save(&app, output, "camp-color", 0.62).await?;
        app.graphics.active.world_grayscale = true;
        save(&app, output, "camp-gray", 0.62).await?;
        app.graphics.active.actor_ground_markers = false;
        save(&app, output, "camp-gray-no-markers", 0.62).await?;
        if suspension::fingerprint(&app.game) != fingerprint {
            return Err("presentation changed camp simulation".into());
        }
        app.graphics.active.actor_ground_markers = true;
        app.graphics.draft = app.graphics.active;
        app.open_menu(MenuScreen::Graphics);
        save(&app, output, "options-1280", 0.62).await?;
        request_new_screen_size(960., 540.);
        for _ in 0..3 {
            next_frame().await;
        }
        save(&app, output, "options-960", 0.62).await?;
        request_new_screen_size(1280., 800.);
        for _ in 0..3 {
            next_frame().await;
        }
        app.menu = MenuScreen::Hidden;
        let map=Map::from_ascii("####################\n#..................#\n#..................#\n#..................#\n#..................#\n#..................#\n#..................#\n#..................#\n#..................#\n#..................#\n#..................#\n#..................#\n####################").map_err(|e|e.to_string())?;
        let mut rules = app.rules.clone();
        rules.player_field_of_view.radius = 20;
        let player = GridPos::new(8, 7);
        let mut game =
            GameState::new_with_rules(map, player, 123, rules).map_err(|e| e.to_string())?;
        for (n, x, y) in [(7, 7, 5), (8, 10, 7), (10, 8, 9)] {
            let actor = Actor::new(GridPos::new(x, y), 100)
                .map_err(|e| e.to_string())?
                .with_player_relation(PlayerRelation::Hostile)
                .with_ai(AiProfile::idle())
                .with_tags([format!("core:monster_art_{n}_0").parse().unwrap()]);
            let id = game.spawn_actor(actor).map_err(|e| e.to_string())?;
            app.actor_glyphs.insert(id, 'd');
            if n == 8 {
                app.selected_target = Some(id);
            }
        }
        app.terminal =
            TerminalView::new(SectorDecor::default(), game.map(), game.player_visibility());
        app.game = WorldState::single(game);
        app.test_lab = true;
        let fingerprint = suspension::fingerprint(&app.game);
        for gray in [false, true] {
            app.graphics.active.world_grayscale = gray;
            let suffix = if gray { "gray" } else { "color" };
            app.combat_motion = crate::combat_visuals::CombatMotion::default();
            save(&app, output, &format!("monsters-{suffix}"), 0.62).await?;
            app.combat_motion
                .play_fireball(player, GridPos::new(10, 7), 1, 0.);
            save(&app, output, &format!("projectile-{suffix}"), 0.20).await?;
            save(&app, output, &format!("explosion-{suffix}"), 0.62).await?;
        }
        if suspension::fingerprint(&app.game) != fingerprint {
            return Err("presentation changed monster simulation".into());
        }
        std::fs::write(output.join("runtime-checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "native_cell":64,"ordinary_world_renderer":true,"camp_atmosphere_composite":true,
            "gray_and_color":true,"markers_on_and_off":true,"projectile_and_explosion":true,
            "simulation_fingerprints_unchanged":true,"settings_file_written":false,"saved_campaign_read_or_written":false,
            "options_resolutions":[[1280,800],[960,540]],"screenshots":11
        })).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }
}
