//! Read-only DUAL15 source-compositor audit and native exports.
use super::*;
use serde_json::json;

#[test]
#[ignore = "DUAL17-C02 complete wrist review, RL_DUAL17_OUTPUT required"]
fn export_dual17_review() {
    let output = std::path::PathBuf::from(std::env::var("RL_DUAL17_OUTPUT").unwrap());
    std::fs::create_dir_all(&output).unwrap();
    assert!(!crate::game_art::walking::dual17_enabled());
    let mut records = Vec::new();
    for index in [0, 4, 10, 14, 15, 19, 20, 24] {
        for (label, hands) in [
            ("actual", [Some(104), Some(70)]),
            ("reverse", [Some(70), Some(104)]),
            ("pistols", [Some(101); 2]),
            ("blades", [Some(68), Some(79)]),
        ] {
            let key = (
                BASE + index,
                selection(None, Some(hands)),
                [EquipmentPalette::Original; 3],
            );
            let baseline = dual_source_atlas(key).bytes;
            for candidate in [false, true] {
                let _guard = crate::game_art::walking::review_dual17(candidate);
                let tag = if candidate { "C02" } else { "baseline" };
                let parts = super::super::layers(key).unwrap();
                let atlas = dual_source_atlas(key);
                let file = format!("CLASS-{index:02}-{label}-{tag}.png");
                atlas.export_png(output.join(&file).to_str().unwrap());
                if index == 15 && label == "actual" {
                    for (part, image) in parts.iter().enumerate() {
                        image.export_png(
                            output
                                .join(format!("front-{tag}-layer{part}.png"))
                                .to_str()
                                .unwrap(),
                        );
                    }
                    for facing in [-1, 1] {
                        for side in 0..2 {
                            for (part, image) in dual_arm15(key, Pose { facing, frame: 0 }, side)
                                .unwrap()
                                .iter()
                                .enumerate()
                            {
                                image.export_png(
                                    output
                                        .join(format!(
                                            "walk-{tag}-dir{facing}-side{side}-part{part}.png"
                                        ))
                                        .to_str()
                                        .unwrap(),
                                );
                            }
                        }
                    }
                }
                records.push(json!({"index":index,"hands":hands,"loadout":label,"candidate":candidate,"file":file}));
            }
            assert_eq!(
                dual_source_atlas(key).bytes,
                baseline,
                "review leaked into normal drawing"
            );
        }
    }
    assert!(!crate::game_art::walking::dual17_enabled());
    std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&json!({"review":"DUAL17-C02","status":"review_only","normal_game_enabled":false,"native_tile_px":64,"source_drawings_changed":false,"shared_weapon_drawing_preserved":true,"weapon_pose_changed":true,"pngs_need_vertical_flip":true,"records":records})).unwrap()).unwrap();
}

fn dual_source_atlas(key: Key) -> Image {
    let mut atlas = Image::gen_image_color(13 * 128, 128, BLANK);
    for (column, pose) in std::iter::once(None)
        .chain(
            [-1, 1]
                .into_iter()
                .flat_map(|facing| (0..6).map(move |frame| Some(Pose { facing, frame }))),
        )
        .enumerate()
    {
        let parts = if let Some(pose) = pose {
            layers(key, pose).unwrap()
        } else {
            super::super::layers(key).unwrap()
        };
        let mut image = Image::gen_image_color(128, 128, BLANK);
        for part in parts {
            geometry::over(&mut image, &part);
        }
        for y in 0..128usize {
            let at = (y * 1664 + column * 128) * 4;
            atlas.bytes[at..at + 512].copy_from_slice(&image.bytes[y * 512..(y + 1) * 512]);
        }
    }
    atlas
}

#[test]
#[ignore = "Complete outfit/hand/weapon connection audit; RL_DUAL16_OUTPUT required"]
fn validate_dual_wrist_connections() {
    let output = std::path::PathBuf::from(std::env::var("RL_DUAL16_OUTPUT").unwrap());
    std::fs::create_dir_all(&output).unwrap();
    let mut failures = Vec::new();
    let mut sleeve_gaps = Vec::new();
    let mut cases = 0;
    for index in 0..25 {
        for facing in [-1, 1] {
            for model in [70, 104] {
                for side in 0..2 {
                    let mut hands = [Some(101); 2];
                    hands[side] = Some(model);
                    let key = (
                        BASE + index,
                        selection(None, Some(hands)),
                        [EquipmentPalette::Original; 3],
                    );
                    for frame in 0..6 {
                        let parts = dual_arm15(key, Pose { facing, frame }, side).unwrap();
                        let contact = parts[2]
                            .bytes
                            .chunks_exact(4)
                            .zip(parts[1].bytes.chunks_exact(4))
                            .any(|(hand, weapon)| hand[3] >= 96 && weapon[3] >= 64);
                        if !contact {
                            failures.push(json!({"index":index,"facing":facing,"model":model,"side":side,"frame":frame}));
                        }
                        let points = |im: &Image| {
                            im.bytes
                                .chunks_exact(4)
                                .enumerate()
                                // Keep the painted antialiasing at the wrist:
                                // its edge can be below 25% opacity.
                                .filter(|(_, p)| p[3] >= 16)
                                .map(|(n, _)| [(n % 128) as i32, (n / 128) as i32])
                                .collect::<Vec<_>>()
                        };
                        let arms = points(&parts[0]);
                        let gloves = points(&parts[2]);
                        let gap = arms
                            .iter()
                            .flat_map(|arm| {
                                gloves.iter().map(move |glove| {
                                    (arm[0] - glove[0]).abs().max((arm[1] - glove[1]).abs())
                                })
                            })
                            .min()
                            .unwrap();
                        if gap > 1 {
                            sleeve_gaps.push(json!({"index":index,"facing":facing,"model":model,"side":side,"frame":frame,"gap_px":gap}));
                        }
                        cases += 1;
                    }
                }
            }
        }
    }
    std::fs::write(output.join("connections.json"), serde_json::to_vec_pretty(&json!({"review":if crate::game_art::walking::dual17_enabled() {"DUAL17-C02"} else {"DUAL15-r01"},"cases":cases,"weapon_contact_failures":failures,"sleeve_gaps_over_one_pixel":sleeve_gaps})).unwrap()).unwrap();
    assert!(failures.is_empty());
    assert!(sleeve_gaps.is_empty(), "{} sleeve gaps", sleeve_gaps.len());
}

#[test]
#[ignore = "DUAL17-C02 connection audit across all 25 outfits; RL_DUAL16_OUTPUT required"]
fn validate_dual17_connections() {
    assert!(!crate::game_art::walking::dual17_enabled());
    {
        let _review = crate::game_art::walking::review_dual17(true);
        validate_dual_wrist_connections();
        for index in 0..25 {
            for model in 0..HELD_WEAPON_COUNT {
                if !matches!(weapon_hold(model), WeaponHold::Side | WeaponHold::Raised) {
                    continue;
                }
                for side in 0..2 {
                    let mut hands = [Some(101); 2];
                    hands[side] = Some(model);
                    let key = (
                        BASE + index,
                        selection(None, Some(hands)),
                        [EquipmentPalette::Original; 3],
                    );
                    let parts = super::super::layers(key).unwrap();
                    let palm = ARMOR_RIGS[index as usize].palms[side];
                    let touches = parts[2]
                        .bytes
                        .chunks_exact(4)
                        .zip(parts[1].bytes.chunks_exact(4))
                        .enumerate()
                        .any(|(n, (hand, weapon))| {
                            hand[3] >= 16
                                && weapon[3] >= 16
                                && ((n % 128) as f32 - palm[0]).abs() <= 5.
                                && ((n / 128) as f32 - palm[1]).abs() <= 5.
                        });
                    assert!(
                        touches,
                        "DUAL17 front grip outfit{index} model{model} side{side}"
                    );
                }
            }
        }
    }
    assert!(!crate::game_art::walking::dual17_enabled());
}

#[test]
#[ignore = "Read-only DUAL16 texture diagnostic; RL_DUAL16_OUTPUT required"]
fn diagnose_dual_texture16() {
    let output = std::path::PathBuf::from(std::env::var("RL_DUAL16_OUTPUT").unwrap());
    std::fs::create_dir_all(&output).unwrap();
    let mut weapon_rows = Vec::new();
    let models: Vec<_> = (0..HELD_WEAPON_COUNT)
        .filter(|&model| matches!(weapon_hold(model), WeaponHold::Side | WeaponHold::Raised))
        .collect();
    let mut weapons = Image::gen_image_color(
        10 * 128,
        (models.len() * 2).div_ceil(10) as u16 * 128,
        BLANK,
    );
    for (n, (model, left)) in models
        .iter()
        .flat_map(|&model| [true, false].map(move |left| (model, left)))
        .enumerate()
    {
        let im = weapon_image(model, left, EquipmentPalette::Original).unwrap();
        for y in 0..128usize {
            let at = ((n / 10 * 128 + y) * 1280 + n % 10 * 128) * 4;
            weapons.bytes[at..at + 512].copy_from_slice(&im.bytes[y * 512..(y + 1) * 512]);
        }
        weapon_rows.push(json!({"cell":n,"model":model,"left":left}));
    }
    weapons.export_png(output.join("one-handed-weapons.png").to_str().unwrap());
    std::fs::write(
        output.join("one-handed-weapons.json"),
        serde_json::to_vec_pretty(&weapon_rows).unwrap(),
    )
    .unwrap();
    let mut rows = Vec::new();
    for index in 0..25 {
        for (label, hands) in [
            ("actual", [Some(104), Some(70)]),
            ("reverse", [Some(70), Some(104)]),
            ("pistols", [Some(101), Some(101)]),
        ] {
            for gray in [false, true] {
                let colors = [
                    EquipmentPalette::Original,
                    EquipmentPalette::Original,
                    if gray {
                        EquipmentPalette::Gray
                    } else {
                        EquipmentPalette::Original
                    },
                ];
                let key = (BASE + index, selection(None, Some(hands)), colors);
                let mut atlas = Image::gen_image_color(5 * 128, 128, BLANK);
                for (n, pose) in [
                    None,
                    Some(Pose {
                        facing: -1,
                        frame: 0,
                    }),
                    Some(Pose {
                        facing: -1,
                        frame: 2,
                    }),
                    Some(Pose {
                        facing: 1,
                        frame: 0,
                    }),
                    Some(Pose {
                        facing: 1,
                        frame: 2,
                    }),
                ]
                .into_iter()
                .enumerate()
                {
                    let parts = if let Some(pose) = pose {
                        layers(key, pose).unwrap()
                    } else {
                        super::super::layers(key).unwrap()
                    };
                    let mut im = Image::gen_image_color(128, 128, BLANK);
                    for (layer, part) in parts.iter().enumerate() {
                        geometry::over(&mut im, part);
                        if index == 15 && label == "actual" && gray {
                            part.export_png(
                                output
                                    .join(format!("actual-pose{n}-layer{layer}.png"))
                                    .to_str()
                                    .unwrap(),
                            );
                        }
                    }
                    for y in 0..128usize {
                        let at = (y * 640 + n * 128) * 4;
                        atlas.bytes[at..at + 512]
                            .copy_from_slice(&im.bytes[y * 512..(y + 1) * 512]);
                    }
                }
                let file = format!(
                    "CLASS-{index:02}-{label}-{}.png",
                    if gray { "gray" } else { "original" }
                );
                atlas.export_png(output.join(&file).to_str().unwrap());
                rows.push(json!({"index":index,"loadout":label,"gray_weapon":gray,"file":file}));
            }
        }
    }
    std::fs::write(output.join("source-checks.json"),serde_json::to_vec_pretty(&json!({"review":"DUAL16","diagnostic_only":true,"source_drawings_changed":false,"pngs_need_vertical_flip":true,"rows":rows})).unwrap()).unwrap();
}

#[test]
#[ignore = "Explicit DUAL15 visual review; RL_DUAL15_OUTPUT required"]
fn export_dual15_review() {
    let _legacy = crate::game_art::walking::review_dual15(false);
    let output = std::path::PathBuf::from(std::env::var("RL_DUAL15_OUTPUT").unwrap());
    std::fs::create_dir_all(&output).unwrap();
    assert!(
        !std::path::Path::new("docs/reviews/dual-walk-15/acceptance.json").exists(),
        "accepted review is frozen"
    );
    assert!(!crate::game_art::walking::dual15_enabled());
    let mut cases = Vec::new();
    let mut contacts = 0;
    let mut failures = Vec::new();
    for index in [0, 4, 10, 11, 12, 13, 14, 15, 20] {
        for (label, hands) in [
            ("pistols", [Some(101), Some(101)]),
            ("blades", [Some(68), Some(79)]),
            ("mixed", [Some(68), Some(101)]),
            ("mixed-reversed", [Some(101), Some(68)]),
        ] {
            let key = (
                BASE + index,
                selection(None, Some(hands)),
                [EquipmentPalette::Original; 3],
            );
            let poses: Vec<_> = [-1, 1]
                .into_iter()
                .flat_map(|facing| (0..6).map(move |frame| Pose { facing, frame }))
                .collect();
            let normal: Vec<_> = poses.iter().map(|&p| layers(key, p).unwrap()).collect();
            let mut variants = Vec::new();
            for proposed in [false, true] {
                let _guard = crate::game_art::walking::review_dual15(proposed);
                let mut atlas = Image::gen_image_color(13 * 128, 128, BLANK);
                let front = super::super::layers(key).unwrap();
                let mut front_image = Image::gen_image_color(128, 128, BLANK);
                for part in front {
                    geometry::over(&mut front_image, &part);
                }
                for y in 0..128usize {
                    atlas.bytes[(y * 1664) * 4..(y * 1664 + 128) * 4]
                        .copy_from_slice(&front_image.bytes[y * 512..(y + 1) * 512]);
                }
                for (n, &pose) in poses.iter().enumerate() {
                    let parts = layers(key, pose).unwrap();
                    let body = if proposed { &parts[1] } else { &parts[0] };
                    assert_eq!(
                        body.bytes, normal[n][0].bytes,
                        "body/helmet changed: {index} {label} {pose:?}"
                    );
                    if proposed {
                        for side in 0..2 {
                            let hand = dual_arm15(key, pose, side).unwrap();
                            let touching = hand[2]
                                .bytes
                                .chunks_exact(4)
                                .zip(hand[1].bytes.chunks_exact(4))
                                .any(|(h, w)| h[3] >= 96 && w[3] >= 64);
                            contacts += 1;
                            if !touching {
                                failures.push(json!({"index":index,"loadout":label,"facing":pose.facing,"frame":pose.frame,"side":side}));
                            }
                        }
                    }
                    let mut image = Image::gen_image_color(128, 128, BLANK);
                    for part in parts {
                        geometry::over(&mut image, &part);
                    }
                    for y in 0..128usize {
                        let at = (y * 1664 + (n + 1) * 128) * 4;
                        atlas.bytes[at..at + 512]
                            .copy_from_slice(&image.bytes[y * 512..(y + 1) * 512]);
                    }
                }
                let file = format!(
                    "CLASS-{index:02}-{label}-{}.png",
                    if proposed { "candidate" } else { "current" }
                );
                atlas.export_png(output.join(&file).to_str().unwrap());
                variants.push(json!({"proposed":proposed,"file":file}));
            }
            assert!(!crate::game_art::walking::dual15_enabled());
            for (n, &pose) in poses.iter().enumerate() {
                assert_eq!(layers(key, pose).unwrap()[0].bytes, normal[n][0].bytes);
            }
            cases.push(json!({"index":index,"loadout":label,"hands":hands,"variants":variants}));
        }
        eprintln!("[DUAL15] CLASS-{index:02}: four loadouts, both directions, six phases");
    }
    // Scope guard: single weapons, two-handed weapons and standing/vertical
    // presentation must remain byte-identical when the proposal is enabled.
    for (weapon, hands) in [
        (Some(2), None),
        (Some(67), None),
        (Some(101), None),
        (None, Some([Some(101), None])),
    ] {
        let key = (
            BASE + 10,
            selection(weapon, hands),
            [EquipmentPalette::Original; 3],
        );
        for pose in [
            Pose {
                facing: 1,
                frame: 2,
            },
            Pose {
                facing: 1,
                frame: 8,
            },
            Pose {
                facing: 0,
                frame: 2,
            },
        ] {
            let normal = layers(key, pose).unwrap();
            let _guard = crate::game_art::walking::review_dual15(true);
            let proposed = layers(key, pose).unwrap();
            assert_eq!(normal.len(), proposed.len());
            for (a, b) in normal.iter().zip(proposed) {
                assert_eq!(a.bytes, b.bytes);
            }
        }
    }
    let report = json!({"review":"DUAL15-r01","status":"awaiting_visual_approval","normal_game_changed":false,"native_tile_px":64,"exported_pngs_need_vertical_flip":true,"body_and_legs_unchanged":true,"rigid_arm_weapon_glove_transform":true,"contacts":contacts,"contact_failures":failures,"cases":cases});
    std::fs::write(
        output.join("review.json"),
        serde_json::to_vec_pretty(&report).unwrap(),
    )
    .unwrap();
    assert!(failures.is_empty(), "{} contact failures", failures.len());
}

#[test]
#[ignore = "Accepted DUAL15 integration audit; RL_DUAL15_INTEGRATION_OUTPUT required"]
fn validate_dual15_integration() {
    assert!(crate::game_art::walking::dual15_enabled());
    let output = std::path::PathBuf::from(std::env::var("RL_DUAL15_INTEGRATION_OUTPUT").unwrap());
    std::fs::create_dir_all(&output).unwrap();
    let accepted: serde_json::Value = serde_json::from_slice(
        &std::fs::read("docs/reviews/dual-walk-15/acceptance.json").unwrap(),
    )
    .unwrap();
    let mut frozen_matches = 0;
    for case in accepted["source_cases"].as_array().unwrap() {
        let index = case["index"].as_u64().unwrap() as u16;
        let hands: [Option<u16>; 2] = serde_json::from_value(case["hands"].clone()).unwrap();
        let key = (
            BASE + index,
            selection(None, Some(hands)),
            [EquipmentPalette::Original; 3],
        );
        let atlas = dual_source_atlas(key);
        let path = std::path::Path::new(case["path"].as_str().unwrap());
        let target = output.join(path.file_name().unwrap());
        atlas.export_png(target.to_str().unwrap());
        assert_eq!(
            std::fs::read(&target).unwrap(),
            std::fs::read(path).unwrap(),
            "accepted DUAL15 changed: outfit{index}"
        );
        frozen_matches += 1;
    }
    let models: Vec<_> = (0..HELD_WEAPON_COUNT)
        .filter(|&m| matches!(weapon_hold(m), WeaponHold::Side | WeaponHold::Raised))
        .collect();
    let mut contacts = 0;
    let mut failures = Vec::new();
    for index in 0..25 {
        for facing in [-1, 1] {
            for &model in &models {
                for side in 0..2 {
                    let hands = if side == 0 {
                        [Some(model), Some(101)]
                    } else {
                        [Some(101), Some(model)]
                    };
                    let key = (
                        BASE + index,
                        selection(None, Some(hands)),
                        [EquipmentPalette::Original; 3],
                    );
                    for frame in 0..6 {
                        let parts = dual_arm15(key, Pose { facing, frame }, side).unwrap();
                        let touches = parts[2]
                            .bytes
                            .chunks_exact(4)
                            .zip(parts[1].bytes.chunks_exact(4))
                            .any(|(h, w)| h[3] >= 96 && w[3] >= 64);
                        if !touches {
                            failures.push(json!({"index":index,"model":model,"side":side,"facing":facing,"frame":frame}));
                        }
                        contacts += 1;
                    }
                }
            }
        }
        eprintln!(
            "[DUAL15-INTEGRATION] outfit{index:02}: {} one-handed models, both hands/directions, six phases",
            models.len()
        );
    }
    let mut unchanged = 0;
    for index in 0..25 {
        for held in [
            selection(Some(2), None),
            selection(Some(67), None),
            selection(Some(101), None),
            selection(None, Some([Some(101), None])),
        ] {
            let key = (BASE + index, held, [EquipmentPalette::Original; 3]);
            for pose in [
                Pose {
                    facing: -1,
                    frame: 2,
                },
                Pose {
                    facing: 1,
                    frame: 2,
                },
                Pose {
                    facing: 1,
                    frame: 8,
                },
                Pose {
                    facing: 0,
                    frame: 2,
                },
            ] {
                let normal = layers(key, pose).unwrap();
                let _legacy = crate::game_art::walking::review_dual15(false);
                let old = layers(key, pose).unwrap();
                assert_eq!(normal.len(), old.len());
                for (a, b) in normal.iter().zip(old) {
                    assert_eq!(a.bytes, b.bytes);
                }
                unchanged += 1;
            }
        }
        for pose in [
            Pose {
                facing: 0,
                frame: 2,
            },
            Pose {
                facing: -1,
                frame: 8,
            },
            Pose {
                facing: 1,
                frame: 8,
            },
        ] {
            let key = (
                BASE + index,
                selection(None, Some([Some(68), Some(101)])),
                [EquipmentPalette::Original; 3],
            );
            let normal = layers(key, pose).unwrap();
            let _legacy = crate::game_art::walking::review_dual15(false);
            let old = layers(key, pose).unwrap();
            for (a, b) in normal.iter().zip(old) {
                assert_eq!(a.bytes, b.bytes);
            }
            unchanged += 1;
        }
    }
    assert!(crate::game_art::walking::dual15_enabled());
    std::fs::write(output.join("source-checks.json"),serde_json::to_vec_pretty(&json!({"review":"DUAL15-r01","status":"integrated","normal_game_activation":true,"accepted_source_atlases_byte_identical":frozen_matches,"technical_outfits":25,"one_handed_models":models,"contact_cases":contacts,"contact_failures":failures,"unaffected_single_two_handed_front_and_vertical_cases":unchanged,"bounded_owned_arm_cache_bytes":8*1024*1024})).unwrap()).unwrap();
    assert!(
        failures.is_empty(),
        "{} DUAL15 contact failures",
        failures.len()
    );
}
