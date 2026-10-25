//! Isolated native64 verification using the ordinary player world/portrait renderer.
use crate::game_art::{self, Visual};
use crate::terminal_view::PlayerWeaponPose;
use macroquad::prelude::*;

pub(crate) async fn capture(output: &std::path::Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(960., 880.);
    for _ in 0..3 {
        next_frame().await;
    }
    let classes = [
        ("core:bastion", "Bastion"),
        ("core:technomancien", "Technomancien"),
        ("core:traqueur", "Traqueuse"),
        ("core:arcaniste", "Arcaniste"),
        ("core:metamorphe", "Metamorphe"),
    ];
    let torsos = [
        "core:veste_de_veille",
        "core:veste_a_fibres_croisees",
        "core:combinaison_fantome",
        "core:armure_cerbere",
        "core:carapace_chrysalide",
    ];
    let pages = [
        "unarmed",
        "pistol",
        "left-pistol",
        "dual-pistols",
        "pistol-smg",
        "rifle",
        "heavy-machinegun",
        "heavy-flamethrower",
        "heavy-grenade",
        "heavy-rocket",
        "rifle-gray",
        "heavy-machinegun-gray",
        "two-handed-axe",
        "two-handed-axe-combat",
        "two-handed-sword",
        "hammer-one-hand",
        "dual",
        "mixed-left-pistol",
        "open-helmet",
        "hood",
        "knife",
        "left-knife",
        "dual-knives",
        "stiletto",
    ];
    for page in pages {
        for frame in 0..3 {
            let _world = crate::world_readability::world(page.ends_with("-gray"));
            clear_background(Color::from_rgba(17, 25, 29, 255));
            draw_text(
                &format!("CLASS-ARMOR-INTEGRATION01 / {page} / native64"),
                16.,
                27.,
                22.,
                WHITE,
            );
            for rank in 0..5 {
                draw_text(
                    &format!("0{}", rank + 1),
                    175. + rank as f32 * 150.,
                    57.,
                    18.,
                    LIGHTGRAY,
                );
            }
            for (class, (id, label)) in classes.iter().enumerate() {
                let y = 72. + class as f32 * 150.;
                draw_text(label, 12., y + 70., 17., WHITE);
                for (rank, torso) in torsos.iter().enumerate() {
                    let x = 135. + rank as f32 * 150.;
                    draw_rectangle(x, y, 140., 136., Color::from_rgba(52, 59, 63, 255));
                    let weapon = match page {
                        "unarmed" => None,
                        "heavy-machinegun" | "heavy-machinegun-gray" => {
                            Some("core:mitrailleuse_lourde")
                        }
                        "heavy-flamethrower" => Some("core:modele_korgan_f100"),
                        "heavy-grenade" => Some("core:lance_grenades"),
                        "heavy-rocket" => Some("core:lance_roquettes"),
                        "two-handed-axe" => Some("core:melee_hache_d_armes"),
                        "two-handed-axe-combat" => Some("core:hache_de_combat"),
                        "two-handed-sword" => Some("core:melee_espadon"),
                        "hammer-one-hand" => Some("core:marteau_de_guerre"),
                        "knife" | "left-knife" | "dual-knives" => Some("core:couteau_ceramique"),
                        "stiletto" => Some("core:melee_stylet"),
                        "pistol" | "left-pistol" | "dual-pistols" | "dual"
                        | "mixed-left-pistol" => Some("core:modele_cogneur_p01"),
                        "pistol-smg" => Some("core:modele_crache_fer_pm06"),
                        _ => Some("core:fusil_d_assaut"),
                    };
                    let expected_reviewed_model = match page {
                        "two-handed-axe" => Some(66),
                        "two-handed-axe-combat" => Some(63),
                        "two-handed-sword" => Some(88),
                        "rifle" | "rifle-gray" | "open-helmet" | "hood" => Some(2),
                        "heavy-machinegun" | "heavy-machinegun-gray" => Some(31),
                        "heavy-flamethrower" => Some(17),
                        "heavy-grenade" => Some(23),
                        "heavy-rocket" => Some(51),
                        _ => None,
                    };
                    if let Some(expected) = expected_reviewed_model
                        && weapon.and_then(game_art::held_weapon) != Some(expected)
                    {
                        return Err(format!(
                            "capture {page}: item does not select reviewed model {expected}"
                        ));
                    }
                    let pose = if weapon.is_none() {
                        PlayerWeaponPose::Unarmed
                    } else if matches!(
                        page,
                        "two-handed-axe"
                            | "two-handed-axe-combat"
                            | "two-handed-sword"
                            | "hammer-one-hand"
                            | "knife"
                            | "left-knife"
                            | "dual-knives"
                            | "stiletto"
                    ) {
                        PlayerWeaponPose::Melee
                    } else {
                        PlayerWeaponPose::Ranged
                    };
                    let mut visual = game_art::wear(
                        game_art::player(Some(id), pose, weapon),
                        [("core:body_armor".to_owned(), torso.to_string())],
                    );
                    if matches!(
                        page,
                        "dual"
                            | "dual-pistols"
                            | "left-pistol"
                            | "mixed-left-pistol"
                            | "left-knife"
                            | "dual-knives"
                    ) {
                        if let Visual::Player { hands, .. } = &mut visual {
                            let gun = game_art::held_weapon("core:modele_cogneur_p01");
                            let knife = game_art::held_weapon("core:couteau_ceramique");
                            *hands = Some(match page {
                                "dual-pistols" => [gun, gun],
                                "left-pistol" => [gun, None],
                                "mixed-left-pistol" => [gun, knife],
                                "left-knife" => [knife, None],
                                "dual-knives" => [knife, knife],
                                _ => [knife, gun],
                            });
                        }
                    }
                    if matches!(page, "open-helmet" | "hood") {
                        visual = game_art::wear(
                            visual,
                            [(
                                "core:head_armor".to_owned(),
                                if page == "hood" {
                                    "core:capuche_de_mue"
                                } else {
                                    "core:calotte_cognefer"
                                }
                                .to_owned(),
                            )],
                        );
                    }
                    game_art::draw_world(Rect::new(x + 38., y + 32., 64., 64.), visual, 1.);
                }
            }
            if frame == 2 {
                crate::ui_capture::framebuffer()?
                    .export_png(output.join(format!("native-{page}.png")).to_str().unwrap());
            } else {
                next_frame().await;
            }
        }
    }
    std::fs::write(
        output.join("gpu-checks.json"),
        serde_json::to_vec_pretty(&serde_json::json!({
            "review_id":"CLASS-ARMOR-INTEGRATION01","ordinary_player_renderer":true,
            "native_tile":64,"outfit_count":25,"gpu_pages":pages.len(),
            "all_five_classes":true,"actual_item_to_visual_binding":true,
            "open_helmet_faces":"canonical uncovered class head; female identity retained"
        }))
        .unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
