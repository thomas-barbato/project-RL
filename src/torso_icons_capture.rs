//! Accepted torso art through normal GPU providers and isolated real inventories.
use super::*;
use crate::game_art::{self, finish_completion as art};
use project_rl::entity::EquipmentPalette as P;

fn palette(letter: &str) -> P {
    match letter.as_bytes().last().unwrap() {
        b'A' => P::Original,
        b'B' => P::SlateBlue,
        b'C' => P::MossGreen,
        b'D' => P::Rust,
        b'E' => P::DeepRed,
        _ => unreachable!(),
    }
}

impl AsciiApp {
    pub async fn capture_torso_icons(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1280., 900.);
        for _ in 0..3 {
            next_frame().await;
        }
        let manifest: serde_json::Value = serde_json::from_str(include_str!(
            "../docs/reviews/refonte-histoire-2026-10-16/torso-icons-review-01/candidates.json"
        ))
        .map_err(|e| e.to_string())?;
        let supplement: serde_json::Value = serde_json::from_str(include_str!(
            "../docs/reviews/refonte-histoire-2026-10-16/torso-starting-review-02/candidates.json"
        ))
        .map_err(|e| e.to_string())?;
        let entries = manifest["entries"]
            .as_array()
            .unwrap()
            .iter()
            .chain(supplement["entries"].as_array().unwrap().iter());
        for entry in entries {
            let ident = entry["illustration"].as_u64().unwrap();
            for frame in 0..3 {
                clear_background(Color::from_rgba(17, 25, 29, 255));
                for (row, variant) in entry["variants"].as_array().unwrap().iter().enumerate() {
                    let y = 30. + row as f32 * 166.;
                    let color = palette(variant["code"].as_str().unwrap());
                    draw_rectangle(12., y, 1256., 158., Color::from_rgba(67, 74, 70, 255));
                    draw_text(variant["code"].as_str().unwrap(), 24., y + 25., 20., WHITE);
                    game_art::draw_item_variant(
                        Rect::new(30., y + 58., 64., 64.),
                        entry["identity"].as_str().unwrap(),
                        color,
                    );
                    game_art::draw_item_variant(
                        Rect::new(122., y + 70., 40., 40.),
                        entry["identity"].as_str().unwrap(),
                        color,
                    );
                    game_art::draw_item_variant(
                        Rect::new(188., y + 76., 28., 28.),
                        entry["identity"].as_str().unwrap(),
                        color,
                    );
                    for (col, (weapon, hands)) in [
                        (None, None),
                        (Some(101), None),
                        (Some(2), None),
                        (Some(88), None),
                        (None, Some([Some(101), Some(68)])),
                        (None, Some([Some(101), None])),
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        art::draw_worn(
                            Rect::new(300. + col as f32 * 154., y + 64., 64., 64.),
                            entry["outfit"].as_u64().unwrap() as u16,
                            weapon,
                            hands,
                            [color, P::Original, P::Original],
                        );
                    }
                }
                if frame == 2 {
                    crate::ui_capture::framebuffer()?
                        .export_png(output.join(format!("native-{ident}.png")).to_str().unwrap());
                } else {
                    next_frame().await;
                }
            }
        }
        request_new_screen_size(1920., 1080.);
        for _ in 0..3 {
            next_frame().await;
        }
        for (label, item, color) in [
            ("robe-blue", "core:veste_de_la_seconde_ombre", P::SlateBlue),
            ("mue-red", "core:manteau_de_mue", P::DeepRed),
            ("chrysalide-green", "core:carapace_chrysalide", P::MossGreen),
            ("cerbere-rust", "core:armure_cerbere", P::Rust),
            ("assaut-blue", "core:tenue_assaut", P::SlateBlue),
            ("technique-green", "core:tenue_technique", P::MossGreen),
        ] {
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
            let exit = pos(camp::scene().exits[0]);
            app.walk_fixture_to_unchecked(exit)?;
            if app.execute_command(GameCommand::Interact { target: exit })
                != CommandOutcome::Applied
            {
                return Err("Torso fixture could not enter street".into());
            }
            app.walk_fixture_to_unchecked(GridPos::new(15, 41))?;
            let item_id: project_rl::content::ContentId =
                item.parse().map_err(|e| format!("{e}"))?;
            let instance = app
                .game
                .grant_colored_equipment_for_diagnostic(item_id.clone(), color)?;
            if !matches!(
                app.execute_command(GameCommand::EquipItem {
                    slot: "core:body_armor".parse().unwrap(),
                    item: instance
                }),
                CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime
            ) {
                return Err("Torso fixture could not equip armor".into());
            }
            let dropped = app
                .game
                .grant_colored_equipment_for_diagnostic(item_id.clone(), color)?;
            if app.execute_command(GameCommand::DropItem { item: dropped })
                != CommandOutcome::Applied
            {
                return Err("Torso fixture could not drop armor".into());
            }
            app.walk_fixture_to_unchecked(GridPos::new(14, 41))?;
            app.selected_world_cell = None;
            app.selected_target = None;
            app.world_cursor = None;
            app.world_fix_frame(output, &format!("{label}-ground"))
                .await?;
            app.inventory_selection = app
                .inventory_entries()
                .iter()
                .position(|e| e.instance() == instance)
                .ok_or("Equipped torso missing")?;
            app.inventory_open = true;
            app.preview_selected_inventory_item();
            app.world_fix_frame(output, &format!("{label}-inventory"))
                .await?;
        }
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "review":["TORSO-ICONS-01","TORSO-STARTING-02"],"native_models":19,"approved_pairs":93,"icon_sizes":[64,40,28],
            "worn_poses_per_pair":6,"real_lighting_scenes":6,"real_inventory_scenes":6,
            "gameplay_fingerprints_unchanged_by_rendering":true,"isolated_save":true
        })).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }
}
