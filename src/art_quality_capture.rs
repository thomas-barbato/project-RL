//! Read-only comparison of current renderer paths, at one 64px world cell.
//! No candidate art, gameplay fixture, save or renderer override is introduced.
use crate::game_art::{self, Visual};
use crate::terminal_view::PlayerWeaponPose;
use macroquad::prelude::*;
use project_rl::entity::EquipmentPalette;

fn player(class: &str, weapon: &str, torso: Option<&str>) -> Visual {
    let visual = game_art::player(Some(class), PlayerWeaponPose::Ranged, Some(weapon));
    game_art::wear(
        visual,
        torso.map(|id| ("core:body_armor".to_owned(), id.to_owned())),
    )
}

pub(crate) async fn capture(output: &std::path::Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(1240., 900.);
    for _ in 0..3 {
        next_frame().await;
    }
    let corpse_dir = "assets/maps/prison-street-expanded-01/decor/civilians-08";
    let mut corpses = Vec::new();
    for number in 4..=8 {
        let texture = load_texture(&format!("{corpse_dir}/CIV-U01-{number:02}-native.png"))
            .await
            .map_err(|e| e.to_string())?;
        texture.set_filter(FilterMode::Nearest);
        corpses.push(texture);
    }
    let item = |id| Visual::ColoredItem(game_art::item(id).unwrap(), EquipmentPalette::Original);
    let visuals = [
        (
            "Bastion · pistolet",
            "tenue de classe / arme portée",
            player("core:bastion", "core:modele_cogneur_p01", None),
        ),
        (
            "Technomancien · fusil",
            "tenue de classe / arme portée",
            player("core:technomancien", "core:fusil_d_assaut", None),
        ),
        (
            "Armure Cerbère · fusil",
            "core:armure_cerbere",
            player(
                "core:bastion",
                "core:fusil_d_assaut",
                Some("core:armure_cerbere"),
            ),
        ),
        (
            "Plaques rapiécées · lame",
            "core:patched_plating",
            player(
                "core:bastion",
                "core:melee_epee_longue",
                Some("core:patched_plating"),
            ),
        ),
        ("PNJ du camp", "CampNpc(1)", Visual::CampNpc(1)),
        (
            "Cultiste · fusil",
            "164 A",
            Visual::CampaignMonster {
                number: 164,
                variant: 0,
            },
        ),
        (
            "Cultiste · hache",
            "165 A",
            Visual::CampaignMonster {
                number: 165,
                variant: 0,
            },
        ),
        (
            "Possédé",
            "005 A",
            Visual::CampaignMonster {
                number: 5,
                variant: 0,
            },
        ),
        (
            "Écorcheur écarlate",
            "144 A",
            Visual::CampaignMonster {
                number: 144,
                variant: 0,
            },
        ),
        (
            "Ailé d'obsidienne",
            "145 A",
            Visual::CampaignMonster {
                number: 145,
                variant: 0,
            },
        ),
        (
            "Icône du fusil",
            "core:fusil_d_assaut / case 64",
            item("core:fusil_d_assaut"),
        ),
        (
            "Icône du pistolet",
            "core:modele_cogneur_p01 / case 64",
            item("core:modele_cogneur_p01"),
        ),
        (
            "Icône de l'armure",
            "core:armure_cerbere / case 64",
            item("core:armure_cerbere"),
        ),
        (
            "Brigandine Cognefer",
            "core:brigandine_cognefer / ancien rig",
            player(
                "core:bastion",
                "core:modele_cogneur_p01",
                Some("core:brigandine_cognefer"),
            ),
        ),
        (
            "Invocation historique",
            "Bestiary 57 / drone",
            Visual::Bestiary {
                number: 57,
                corpse: false,
            },
        ),
    ];
    let mut samples = Vec::new();
    for frame in 0..3 {
        clear_background(Color::from_rgba(17, 25, 29, 255));
        draw_text(
            "AUDIT VISUEL · ressources actuelles · 19 octobre 2026",
            24.,
            30.,
            24.,
            Color::from_rgba(222, 204, 167, 255),
        );
        draw_text(
            "Même échelle : 64 px par case · sans éclairage de scène · aucun dessin modifié",
            24.,
            57.,
            19.,
            LIGHTGRAY,
        );
        for index in 0..20 {
            let x = 20. + (index % 5) as f32 * 240.;
            let y = 80. + (index / 5) as f32 * 200.;
            let cell = Rect::new(x + 88., y + 68., 64., 64.);
            draw_rectangle(
                x + 4.,
                y + 34.,
                232.,
                130.,
                Color::from_rgba(57, 61, 55, 255),
            );
            draw_rectangle_lines(
                cell.x,
                cell.y,
                64.,
                64.,
                1.,
                Color::new(0.5, 0.56, 0.52, 0.22),
            );
            let (label, code) = if index < 5 {
                let t = &corpses[index];
                draw_texture(
                    t,
                    cell.center().x - t.width() / 2.,
                    cell.bottom() - t.height(),
                    WHITE,
                );
                (
                    format!("Civil mort · {}", index + 4),
                    format!("CIV-U01-{:02} / 80 × 48", index + 4),
                )
            } else {
                let (label, code, visual) = visuals[index - 5];
                game_art::draw_world(cell, visual, 1.);
                (label.to_owned(), code.to_owned())
            };
            draw_text(
                &label,
                x + 9.,
                y + 22.,
                19.,
                Color::from_rgba(222, 204, 167, 255),
            );
            draw_text(&code, x + 9., y + 184., 15., LIGHTGRAY);
            if frame == 0 {
                samples.push(serde_json::json!({"label":label,"code":code,"panel":[x,y,240,200],"cell":[cell.x,cell.y,64,64]}));
            }
        }
        if frame == 2 {
            crate::ui_capture::framebuffer()?
                .export_png(output.join("native-comparison.png").to_str().unwrap());
        } else {
            next_frame().await;
        }
    }
    std::fs::write(
        output.join("checks.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "review":"ART-QUALITY-01","date":"2026-10-19","native_cell":64,
            "render":"game_art::draw_world, native corpse textures", "world_lighting":false,
            "gameplay_loaded":false,"art_modified":false,"samples":samples
        }))
        .unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
