//! Native captures of the integrated equipment UI; no proposal-only rendering.
use super::*;
impl AsciiApp {
    pub async fn capture_weapon_sets_review(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(1810, rules, texts, loot, expeditions)?;
        app.crash_recovery_enabled = false;
        app.suspension_path = output.join("unused-review-save.json");
        app.begin_character_creation_with_seed(false, 1810)?;
        app.update_input(&InputFrame {
            pressed: [controls::Binding::key("Enter")].into(),
            ..Default::default()
        });
        for (slot, model) in [
            (0, "core:integrity_blade"),
            (1, "core:modele_cogneur_p01"),
            (2, "core:flamethrower"),
        ] {
            if !app
                .game
                .player_inventory()
                .iter()
                .any(|e| e.item().as_str() == model)
            {
                app.game
                    .grant_player_item_for_diagnostic(model.parse().unwrap(), 1)?;
            }
            let item = app
                .game
                .player_inventory()
                .iter()
                .find(|e| e.item().as_str() == model)
                .unwrap()
                .instance();
            if !matches!(
                app.equip_diagnostic_weapon(item, slot),
                CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime
            ) {
                return Err(format!("Review equipment failed: {model}"));
            }
        }
        app.history.clear();
        app.log.clear();
        app.game.drain_events();
        app.select_weapon_slot(0);
        app.inventory_open = true;
        app.graphics.active.interface_style = crate::graphics::InterfaceStyle::Metal;
        app.graphics.active.interface_theme = crate::graphics::InterfaceTheme::Violet;
        app.graphics.active.ui_scale_percent = 100;
        app.graphics.active.reduced_motion = true;
        app.ux.skill_bar = [
            Some("B01".into()),
            Some("A01".into()),
            None,
            None,
            None,
            None,
        ];
        let selected = app.game.equipped_player_weapon_item(0).unwrap();
        app.select_inventory_instance(selected);
        set_fullscreen(true);
        request_new_screen_size(1920., 1080.);
        for _ in 0..5 {
            app.draw();
            next_frame().await;
        }
        if (screen_width(), screen_height()) != (1920., 1080.) {
            return Err(format!(
                "Native 1920x1080 surface unavailable: {}x{}",
                screen_width(),
                screen_height()
            ));
        }
        let _clock = crate::effect_geometry::frame_time(10.);
        let items = app.game.player_inventory().len();
        let mut captures = Vec::new();
        for (theme, style, color) in [
            (
                "metal",
                crate::graphics::InterfaceStyle::Metal,
                crate::graphics::InterfaceTheme::Violet,
            ),
            (
                "terminal",
                crate::graphics::InterfaceStyle::Terminal,
                crate::graphics::InterfaceTheme::Green,
            ),
        ] {
            app.graphics.active.interface_style = style;
            app.graphics.active.interface_theme = color;
            for reversed in [false, true] {
                let pair = if reversed { 2 } else { 0 };
                for (slot, model) in [
                    (pair, "core:integrity_blade"),
                    (pair + 1, "core:modele_cogneur_p01"),
                    (2 - pair, "core:flamethrower"),
                ] {
                    let item = app
                        .game
                        .player_inventory()
                        .iter()
                        .find(|entry| entry.item().as_str() == model)
                        .unwrap()
                        .instance();
                    if !matches!(
                        app.equip_diagnostic_weapon(item, slot),
                        CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime
                    ) {
                        return Err("Equipment fixture failed".into());
                    }
                }
                for set in 0..2 {
                    app.select_weapon_slot(set * 2);
                    let double = (set == 1) != reversed;
                    for (screen, inventory) in [("inventory", true), ("hud", false)] {
                        app.inventory_open = inventory;
                        app.game.drain_events();
                        let before = app.game.recovery_snapshot_bytes()?;
                        app.draw();
                        next_frame().await;
                        app.draw();
                        let file = format!(
                            "{theme}-{}-{}-{screen}.png",
                            if reversed { "b" } else { "a" },
                            set + 1
                        );
                        crate::ui_capture::framebuffer()?
                            .export_png(output.join(&file).to_str().ok_or("Review image path")?);
                        if app.game.recovery_snapshot_bytes()? != before {
                            return Err("Drawing altered the game".into());
                        }
                        captures.push(serde_json::json!({"file":file,"theme":theme,"scenario":if reversed {"b"}else{"a"},"set":set+1,"two_handed":double,"screen":screen}));
                        next_frame().await;
                    }
                }
            }
        }
        if app.game.player_inventory().len() != items {
            return Err("Review changed inventory instances".into());
        }
        std::fs::write(output.join("native.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "review":"WEAPON-SETS-01","status":"integrated","normal_game_changed":true,
            "visual_revision":"WEAPON-SETS-01-GHOST-02","reserved_hand_opacity":0.3,
            "actual_engine_uses_two_sets":true,"native_size":[1920,1080],"existing_art_only":true,
            "draws_preserve_engine_snapshot":true,"inventory_instances_preserved":items,"history_commands_added":app.history.len(),
            "captures":captures,
        })).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }
}
