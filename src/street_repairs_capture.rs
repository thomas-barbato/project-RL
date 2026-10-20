//! U01-REPAIRS-09: normal integrated scenery against the retained review candidate.
use super::*;

impl AsciiApp {
    pub async fn capture_street_repairs(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let (mut rules, texts, loot, expeditions) = ascii_game_content()?;
        rules.player_inventory_capacity = 96;
        let mut app = Self::from_seed_version(
            1872,
            rules,
            texts,
            loot,
            expeditions,
            CURRENT_GENERATION_VERSION,
        )?;
        app.suspension_path = output.join("isolated-run.json");
        app.crash_recovery_enabled = false;
        request_new_screen_size(1920., 1080.);
        for _ in 0..3 {
            next_frame().await;
        }
        let exit = pos(camp::scene().exits[0]);
        app.walk_fixture_to_unchecked(exit)?;
        if app.execute_command(GameCommand::Interact { target: exit }) != CommandOutcome::Applied {
            return Err("Street repair fixture could not enter street".into());
        }
        let torso = app.game.grant_colored_equipment_for_diagnostic(
            "core:tenue_assaut".parse().unwrap(),
            project_rl::entity::EquipmentPalette::Original,
        )?;
        if !matches!(
            app.execute_command(GameCommand::EquipItem {
                slot: "core:body_armor".parse().unwrap(),
                item: torso
            }),
            CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime
        ) {
            return Err("Street repair fixture could not equip accepted actor art".into());
        }
        for at in [
            [14, 16],
            [15, 40],
            [2, 30],
            [2, 31],
            [15, 32],
            [16, 32],
            [17, 32],
            [20, 49],
            [20, 50],
            [20, 51],
        ] {
            if !app.game.map().is_walkable(GridPos::new(at[0], at[1])) {
                return Err(format!("Reported contact remains blocked: {at:?}"));
            }
        }
        if app.game.map().is_walkable(GridPos::new(16, 33)) {
            return Err("Street repair removed the solid vehicle body".into());
        }
        for y in 7..=8 {
            for x in 9..=12 {
                if !app.game.map().is_walkable(GridPos::new(x, y)) {
                    return Err(format!("Removed prop left blocked ground: {x},{y}"));
                }
            }
        }
        let mut scenes = Vec::new();
        for (label, at) in [
            ("warehouse", [8, 8]),
            ("warehouse-cleared", [11, 7]),
            ("road14", [14, 14]),
            ("crater16", [14, 16]),
            ("van32", [16, 32]),
            ("rubble-west", [2, 30]),
            ("rubble40", [15, 40]),
            ("junction", [18, 49]),
            ("gate", [8, 59]),
        ] {
            app.walk_fixture_to_unchecked(GridPos::new(at[0], at[1]))?;
            app.selected_world_cell = None;
            app.selected_target = None;
            app.world_cursor = None;
            for (suffix, candidate) in [("current", false), ("candidate", true)] {
                crate::prison_street_render::review_repairs(candidate);
                app.world_fix_frame(output, &format!("{label}-{suffix}"))
                    .await?;
                if label == "junction" {
                    crate::prison_street_render::export_review_world(
                        &app.game,
                        &output.join(format!("map-{suffix}.png")),
                    )?;
                }
            }
            scenes.push(serde_json::json!({"id":label,"player":at,"render_read_only":true}));
        }
        crate::prison_street_render::review_repairs(false);
        let entry = pos(crate::prison_street::expanded_scene().entrance);
        app.walk_fixture_to_unchecked(entry)?;
        if app.execute_command(GameCommand::Interact { target: entry }) != CommandOutcome::Applied
            || app.game.current_zone().unwrap().id.as_str() != camp::ZONE_ID
        {
            return Err("Street repair camp round trip failed".into());
        }
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "review":"U01-REPAIRS-09","native_cell":64,"generation":CURRENT_GENERATION_VERSION,
            "collision_repairs":crate::prison_street::contact_repair_cells()?.len(),
            "reported_contacts_clear":true,"vehicle_body_still_solid":true,
            "camp_round_trip":true,"user_saves_touched":false,"normal_art_integrated":true,
            "removed_props":3,"former_prop_ground_clear":true,
            "additional_clearance_cells":crate::prison_street::warehouse_clearance_cells()?.len(),
            "scenes":scenes,"world_render_read_only":true,
            "note":"Collision status checked before return; full maps omit living actors and vision mask for review only"
        })).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }
}
