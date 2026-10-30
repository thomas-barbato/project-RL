//! Verification of the normal, accepted renderer, without review activation.
use crate::{
    asset_preparation,
    combat_visuals::CombatMotion,
    game_art::{
        self, Visual,
        walking::{self, Pose},
    },
    world_readability,
};
use macroquad::prelude::*;
use project_rl::{entity::EquipmentPalette, world::GridPos};
use std::path::Path;

fn player(
    body: u8,
    heavy: bool,
    main: Option<u16>,
    left: Option<u16>,
    head: Option<u16>,
) -> Visual {
    Visual::Player {
        body,
        equipment: [heavy.then_some(103), head, None, None],
        weapon: main,
        hands: left.map(|l| [Some(l), main]),
        palettes: [EquipmentPalette::Original; 3],
    }
}
fn cases() -> Vec<(String, Visual, bool)> {
    let mut out = Vec::new();
    for (body, id) in [
        (6, "arcaniste"),
        (3, "bastion"),
        (5, "traqueuse"),
        (7, "metamorphe"),
    ] {
        for heavy in [false, true] {
            let id = if heavy {
                format!("{id}-{}", if body == 5 { "lourde" } else { "lourd" })
            } else {
                id.into()
            };
            for model in 0..113 {
                out.push((
                    format!("{id}-W{model:03}"),
                    player(body, heavy, Some(model), None, None),
                    matches!(model, 2 | 66 | 88 | 104),
                ));
            }
            for model in [2, 66, 88, 104] {
                for head in 115..=120 {
                    out.push((
                        format!("{id}-W{model:03}-H{head}"),
                        player(body, heavy, Some(model), None, Some(head)),
                        model == 104 && head == 118,
                    ));
                }
            }
            for (main, left) in [(104, 69), (104, 70), (104, 104), (68, 79)] {
                out.push((
                    format!("{id}-DUAL-{main}-{left}"),
                    player(body, heavy, Some(main), Some(left), None),
                    true,
                ));
            }
        }
    }
    let raw: serde_json::Value = serde_json::from_str(include_str!(
        "../assets/animations/vertical-18/manifest.json"
    ))
    .unwrap();
    for p in raw["profiles"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|p| p.get("fixed_loadouts").is_some())
    {
        let outfit = p["outfit"].as_u64().unwrap() as u8;
        let make = |main, left, head| {
            let mut v = player(outfit / 5 + 3, false, main, left, head);
            if let Visual::Player { equipment, .. } = &mut v {
                equipment[0] =
                    [None, Some(110), Some(102), Some(101), Some(103)][usize::from(outfit % 5)];
            }
            v
        };
        let id = p["id"].as_str().unwrap();
        for c in p["cases"].as_array().unwrap() {
            let model = c["model"].as_u64().unwrap() as u16;
            out.push((
                format!("{id}-W{model:03}"),
                make(Some(model), None, None),
                [2, 66, 88].contains(&model),
            ));
        }
        for model in [2, 66, 88] {
            for head in 115..=120 {
                out.push((
                    format!("{id}-W{model:03}-H{head}"),
                    make(Some(model), None, Some(head)),
                    head == 120,
                ));
            }
        }
        for c in p["fixed_loadouts"].as_array().unwrap() {
            let main = c["main"].as_u64().unwrap() as u16;
            let left = c["left"].as_u64().unwrap() as u16;
            let head = c["helmet"].as_u64().map(|n| n as u16);
            out.push((
                format!("{id}-C11-{main}-{left}-H{}", head.unwrap_or(0)),
                make(Some(main), Some(left), head),
                head.is_none(),
            ));
        }
    }
    for p in raw["monsters"].as_array().unwrap() {
        let number = p["number"].as_u64().unwrap() as u16;
        let variant = p["variant"].as_u64().unwrap() as u8;
        out.push((
            format!("MON-{number:03}-v{variant}"),
            Visual::CampaignMonster { number, variant },
            variant == 0 && [5, 22, 24, 71, 124, 141, 142, 148, 164].contains(&number),
        ));
    }
    out.push(("NPC-01".into(), Visual::CampNpc(1), true));
    out
}
pub async fn capture(output: &Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(960., 760.);
    for _ in 0..3 {
        next_frame().await;
    }
    let mut checked = 0;
    let mut returns = 0;
    let mut saved = 0;
    let mut draw_total = 0;
    let id = serde_json::from_str("1").unwrap();
    for (label, visual, save) in cases() {
        if !game_art::vertical::supports(Some(visual)) {
            return Err(format!("Normal VERT18 selector missing: {label}"));
        }
        game_art::vertical::prepare(&[visual])?;
        let before = asset_preparation::stats();
        let before_draws = game_art::vertical::draw_count();
        for warm in 0..2 {
            set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
            clear_background(Color::from_rgba(30, 40, 49, 255));
            draw_text(
                &format!("VERT18 / integration normale / {label} / natif64"),
                16.,
                24.,
                20.,
                WHITE,
            );
            draw_text(
                "Face 01-06 / dos 07-12 / repos frontal 13",
                16.,
                48.,
                16.,
                LIGHTGRAY,
            );
            let _world = world_readability::world(false);
            for n in 0..13 {
                let x = 16. + (n % 4) as f32 * 232.;
                let y = 62. + (n / 4) as f32 * 164.;
                draw_text(&format!("{:02}", n + 1), x, y + 16., 14., GRAY);
                let pose = if n == 12 {
                    None
                } else {
                    Some(Pose {
                        facing: if n < 6 { 2 } else { -2 },
                        frame: (n % 6) as u8,
                    })
                };
                let audit = world_readability::audit_textures(|| {
                    walking::with_pose(pose, || {
                        game_art::draw_world(Rect::new(x + 84., y + 84., 64., 64.), visual, 1.)
                    })
                });
                if audit.len() != 1
                    || audit
                        .iter()
                        .any(|a| !a.actor || !a.world || !a.finish || a.gain != 1.)
                {
                    return Err(format!("Normal actor finish mismatch: {label}/{n}"));
                }
            }
            if warm == 0 {
                next_frame().await;
            } else if save {
                crate::ui_capture::framebuffer()?
                    .export_png(output.join(format!("{label}.png")).to_str().unwrap());
                saved += 1;
            }
        }
        if game_art::vertical::draw_count() - before_draws != 26 {
            return Err(format!("Normal rendering fell back for {label}"));
        }
        let after = asset_preparation::stats();
        for field in ["jobs", "uploads", "source_reads"] {
            if before[field] != after[field] {
                return Err(format!("VERT18 resource work during draw: {label}/{field}"));
            }
        }
        let mut motion = CombatMotion::default();
        for milliseconds in [120, 200, 400] {
            for dy in [-1, 1] {
                for step in 0..2 {
                    motion.glide_timed(
                        id,
                        GridPos::new(0, step * dy),
                        GridPos::new(0, (step + 1) * dy),
                        10. + f64::from(step),
                        milliseconds,
                    );
                    for tick in 0..3 {
                        let pose = motion
                            .vertical_walk_pose(
                                id,
                                10. + f64::from(step)
                                    + (f64::from(tick) + 0.5) / 3. * f64::from(milliseconds)
                                        / 1000.,
                                false,
                            )
                            .ok_or("Missing normal vertical pose")?;
                        if pose.facing != dy as i8 * 2 || pose.frame != step as u8 * 3 + tick as u8
                        {
                            return Err("Normal vertical phase or direction mismatch".into());
                        }
                    }
                    if motion
                        .vertical_walk_pose(
                            id,
                            10. + f64::from(step) + f64::from(milliseconds) / 1000. + 0.001,
                            false,
                        )
                        .is_some()
                        || motion
                            .vertical_walk_pose(id, 10. + f64::from(step), true)
                            .is_some()
                    {
                        return Err("Normal front return/reduced motion mismatch".into());
                    }
                    returns += 1;
                }
            }
        }
        checked += 1;
        draw_total += 26;
        if checked % 100 == 0 {
            eprintln!("[VERT18 INTEGRATION] {checked}");
        }
        next_frame().await;
    }
    let a = player(6, false, Some(104), Some(70), None);
    let b = Visual::CampaignMonster {
        number: 148,
        variant: 0,
    };
    game_art::vertical::prepare(&[a])?;
    game_art::vertical::prepare(&[b])?;
    let before = asset_preparation::stats();
    game_art::vertical::prepare(&[a])?;
    let after = asset_preparation::stats();
    assert_eq!(before["jobs"], after["jobs"]);
    assert_eq!(before["uploads"], after["uploads"]);
    let report = serde_json::json!({"review":"VERT18-C06+C08-B+C09-B+C11-BACK-R02+C12","normal_game_enabled":true,"review_override_used":false,"intermediate_profiles":12,"C12_weapon_bindings":888,"C11_fixed_loadouts":168,"configurations":checked,"normal_native_draws":draw_total,"front_returns_at_120_200_400_ms":returns,"saved_boards":saved,"new_jobs_or_uploads_during_draw":0,"return_visit_reused_cache":true,"same_actor_finish":true,"resource_cache":after,"saved_game_loaded":false,"preferences_written":false});
    std::fs::write(
        output.join("checks.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    crate::ascii_app::AsciiApp::capture_cold_start(
        &output.join("camp"),
        "vertical18-integration-camp",
    )
    .await?;
    Ok(())
}
