use super::*;
use std::path::Path;

fn player(p: &Profile, main: Option<u16>, left: Option<u16>, helmet: Option<u16>) -> Visual {
    Visual::Player {
        body: (p.outfit / 5) as u8 + 3,
        equipment: [
            [None, Some(110), Some(102), Some(101), Some(103)][usize::from(p.outfit % 5)],
            helmet,
            None,
            None,
        ],
        weapon: main,
        hands: left.map(|l| [Some(l), main]),
        palettes: [EquipmentPalette::Original; 3],
    }
}
fn load(path: &Path) -> Image {
    Image::from_file_with_format(&std::fs::read(path).unwrap(), Some(ImageFormat::Png)).unwrap()
}
fn cell(source: &PreparedImage, n: usize, side: u16, anchor: [f32; 2]) -> Image {
    let [w, h] = source.cell;
    let part = source.image.sub_image(Rect::new(
        (n % 6) as f32 * f32::from(w),
        (n / 6) as f32 * f32::from(h),
        f32::from(w),
        f32::from(h),
    ));
    let mut out = Image::gen_image_color(side, side, BLANK);
    let x = (anchor[0] - source.anchor[0]).round() as usize;
    let y = (anchor[1] - source.anchor[1]).round() as usize;
    assert!(x + usize::from(w) <= usize::from(side) && y + usize::from(h) <= usize::from(side));
    for row in 0..usize::from(h) {
        let at = ((row + y) * usize::from(side) + x) * 4;
        out.bytes[at..at + usize::from(w) * 4]
            .copy_from_slice(&part.bytes[row * usize::from(w) * 4..(row + 1) * usize::from(w) * 4]);
    }
    out
}
fn compare(
    prepared: &PreparedImage,
    expected: &Image,
    side: u16,
    anchor: [f32; 2],
    label: &str,
) -> u8 {
    let mut max = 0;
    for n in 0..13 {
        let actual = cell(prepared, n, side, anchor);
        let reference = expected.sub_image(Rect::new(
            (n % 6) as f32 * f32::from(side),
            (n / 6) as f32 * f32::from(side),
            f32::from(side),
            f32::from(side),
        ));
        for (i, (a, b)) in actual
            .bytes
            .chunks_exact(4)
            .zip(reference.bytes.chunks_exact(4))
            .enumerate()
        {
            if a[3] == 0 && b[3] == 0 {
                continue;
            }
            for c in 0..4 {
                // RGB on an alpha-zero fringe has no visible contribution.
                // Retain a separate one-unit limit on the sampled alpha.
                let delta = if c == 3 {
                    a[3].abs_diff(b[3])
                } else {
                    (u32::from(a[c]) * u32::from(a[3]))
                        .abs_diff(u32::from(b[c]) * u32::from(b[3]))
                        .div_ceil(255) as u8
                };
                assert!(
                    delta <= if c == 3 { 1 } else { 2 },
                    "{label} frame{n} pixel{i} channel{c}: {} vs {}; rgba {a:?} / {b:?}",
                    a[c],
                    b[c]
                );
                max = max.max(delta);
            }
        }
    }
    max
}

#[test]
fn accepted_scope_covers_weapons_colors_heads_and_pairs_without_changing_other_bodies() {
    let m = manifest();
    assert_eq!(m.profiles.len(), 20);
    assert_eq!(m.monsters.len(), 471);
    for p in m.profiles.iter().filter(|p| matches!(p.outfit % 5, 0 | 4)) {
        for case in p.cases.iter().filter(|c| !c.left) {
            for helmet in [
                None,
                Some(115),
                Some(116),
                Some(117),
                Some(118),
                Some(119),
                Some(120),
            ] {
                for color in EquipmentPalette::ALL {
                    let mut v = player(p, Some(case.model), None, helmet);
                    if let Visual::Player { palettes, .. } = &mut v {
                        palettes[1] = color;
                    }
                    assert!(selection(v).is_some());
                }
            }
        }
        let one: Vec<_> = p.cases.iter().filter(|c| c.left).map(|c| c.model).collect();
        assert_eq!(one.len(), 39);
        for &main in &one {
            for &left in &one {
                assert!(selection(player(p, Some(main), Some(left), None)).is_some());
            }
        }
        assert!(selection(player(p, Some(2), Some(104), None)).is_none());
        let mut v = player(p, None, None, None);
        if let Visual::Player { palettes, .. } = &mut v {
            palettes[0] = EquipmentPalette::SlateBlue;
        }
        assert!(selection(v).is_none());
    }
    for p in &m.monsters {
        assert!(supports(Some(Visual::CampaignMonster {
            number: p.number,
            variant: p.variant
        })));
    }
    for body in [4] {
        for torso in [110, 102, 101] {
            assert!(!supports(Some(Visual::Player {
                body,
                equipment: [Some(torso), None, None, None],
                weapon: Some(2),
                hands: None,
                palettes: [EquipmentPalette::Original; 3]
            })));
        }
    }
    assert!(!supports(Some(Visual::DroneGunner)));
    assert!(!supports(Some(Visual::CampNpc(2))));
    assert!(!supports(Some(Visual::Bestiary {
        number: 22,
        corpse: true
    })));
}

#[test]
fn representative_runtime_compositions_match_the_accepted_pixels() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("assets/source-art/vertical-walk-review-18/extension-C06");
    for p in manifest()
        .profiles
        .iter()
        .filter(|p| matches!(p.outfit % 5, 0 | 4))
    {
        for model in [2, 51, 66, 88, 104] {
            let v = player(p, Some(model), None, None);
            let image = prepare_image(&selection(v).unwrap()).unwrap();
            let expected = load(
                &reference_dir(&root, model, None, false).join(format!("{}-W{model:03}.png", p.id)),
            );
            compare(
                &image,
                &expected,
                256,
                [128., 160.],
                &format!("{}/{model}", p.id),
            );
        }
    }
}

#[test]
fn accepted_c08_c09_angles_preserve_every_other_attachment_field() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let original: serde_json::Value = serde_json::from_slice(&std::fs::read(root.join("assets/source-art/vertical-walk-review-18/extension-C06/conditioned-weapons/recipes.json")).unwrap()).unwrap();
    let current: serde_json::Value = serde_json::from_str(include_str!(
        "../assets/animations/vertical-18/manifest.json"
    ))
    .unwrap();
    let approved: serde_json::Value = serde_json::from_slice(
        &std::fs::read(
            root.join("assets/source-art/vertical-walk-review-18/candidate-C08/knife-angles.json"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(current["accepted_overrides"][0], "VERT18-C08-B");
    assert_eq!(current["accepted_overrides"][1], "VERT18-C09-B");
    let pistols: serde_json::Value = serde_json::from_slice(
        &std::fs::read(
            root.join("assets/source-art/vertical-walk-review-18/candidate-C09/pistol-angles.json"),
        )
        .unwrap(),
    )
    .unwrap();
    assert_eq!(pistols["outward_degrees"], 10);
    let mut changed = 0;
    let mut changed_pistols = 0;
    for p in current["profiles"].as_array().unwrap() {
        if p.get("fixed_loadouts").is_some() {
            continue;
        }
        let source = original["profiles"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["profile"] == p["id"])
            .unwrap();
        for c in p["cases"].as_array().unwrap() {
            let old = source["cases"]
                .as_array()
                .unwrap()
                .iter()
                .find(|s| s["model"] == c["model"] && s["left"] == c["left"])
                .unwrap();
            let model = c["model"].as_u64().unwrap();
            for (frame, reference) in c["frames"]
                .as_array()
                .unwrap()
                .iter()
                .zip(old["frames"].as_array().unwrap())
            {
                if (68..=79).contains(&model) {
                    let axis = approved["knife_axes"][format!(
                        "{model}-{}",
                        u8::from(frame["left_source"].as_bool().unwrap())
                    )]
                    .as_f64()
                    .unwrap();
                    let tilt = 20_f64.to_radians();
                    let expected = if frame["left_source"] == true {
                        std::f64::consts::PI - tilt
                    } else {
                        tilt
                    } - axis;
                    assert!((frame["angle"].as_f64().unwrap() - expected).abs() < 1e-12);
                    changed += 1;
                } else if (101..=112).contains(&model) {
                    let left = frame["left_source"].as_bool().unwrap();
                    let baseline = pistols["baseline_angles"][u8::from(left).to_string()]
                        .as_f64()
                        .unwrap();
                    assert_eq!(baseline, reference["angle"].as_f64().unwrap());
                    let expected = baseline
                        + if left {
                            10_f64.to_radians()
                        } else {
                            -10_f64.to_radians()
                        };
                    assert!((frame["angle"].as_f64().unwrap() - expected).abs() < 1e-12);
                    changed_pistols += 1;
                } else {
                    assert_eq!(
                        frame, reference,
                        "Accepted tilt changed another weapon: {model}"
                    );
                }
                let mut a = frame.clone();
                let mut b = reference.clone();
                a.as_object_mut().unwrap().remove("angle");
                b.as_object_mut().unwrap().remove("angle");
                assert_eq!(
                    a, b,
                    "Accepted tilt changed a grip, scale or layer: {model}"
                );
            }
        }
    }
    assert_eq!(changed, 2496);
    assert_eq!(changed_pistols, 2496);
}

fn reference_dir(art: &Path, main: u16, left: Option<u16>, variant: bool) -> std::path::PathBuf {
    if (101..=112).contains(&main) || left.is_some_and(|m| (101..=112).contains(&m)) {
        art.parent()
            .unwrap()
            .join("candidate-C09/composites-original")
    } else if (68..=79).contains(&main) || left.is_some_and(|m| (68..=79).contains(&m)) {
        art.parent()
            .unwrap()
            .join("candidate-C08/composites-original")
    } else {
        art.join(if variant {
            "conditioned-weapons/composites-variants"
        } else {
            "conditioned-weapons/composites-original"
        })
    }
}

#[test]
fn intermediate_outfits_select_only_accepted_two_hand_and_c11_loadouts() {
    let profiles: Vec<_> = manifest()
        .profiles
        .iter()
        .filter(|p| !p.fixed_loadouts.is_empty())
        .collect();
    assert_eq!(profiles.len(), 12);
    for p in profiles {
        assert_eq!(p.cases.len(), 74);
        assert_eq!(p.fixed_loadouts.len(), 14);
        for c in &p.cases {
            assert!(c.two_handed && !c.left);
            for head in [
                None,
                Some(115),
                Some(116),
                Some(117),
                Some(118),
                Some(119),
                Some(120),
            ] {
                for color in EquipmentPalette::ALL {
                    let mut v = player(p, Some(c.model), None, head);
                    if let Visual::Player { palettes, .. } = &mut v {
                        palettes[1] = color;
                    }
                    assert!(selection(v).is_some(), "{}/{}/{head:?}", p.id, c.model);
                }
            }
        }
        for fixed in &p.fixed_loadouts {
            assert!(matches!(
                selection(player(p, Some(fixed.main), Some(fixed.left), fixed.helmet)),
                Some(Selection::FixedPlayer { .. })
            ));
        }
        for (main, left) in [
            (None, None),
            (Some(104), None),
            (Some(104), Some(104)),
            (Some(2), Some(70)),
            (Some(70), Some(104)),
        ] {
            assert!(selection(player(p, main, left, None)).is_none());
        }
        let mut colored = player(p, Some(104), Some(70), None);
        if let Visual::Player { palettes, .. } = &mut colored {
            palettes[1] = EquipmentPalette::SlateBlue;
        }
        assert!(selection(colored).is_none());
    }
}

#[test]
#[ignore = "complete C11/C12 approved pixel bank; explicit optimized validation"]
fn intermediate_runtime_matches_every_approved_composite() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let art = root.join("assets/source-art/vertical-walk-review-18");
    let mut c11 = 0;
    let mut c12 = 0;
    let mut max = 0;
    for p in manifest()
        .profiles
        .iter()
        .filter(|p| !p.fixed_loadouts.is_empty())
    {
        for case in &p.cases {
            let heads: &[Option<u16>] = if [2, 66, 88].contains(&case.model) {
                &[
                    None,
                    Some(115),
                    Some(116),
                    Some(117),
                    Some(118),
                    Some(119),
                    Some(120),
                ]
            } else {
                &[None]
            };
            for &head in heads {
                let image =
                    prepare_image(&selection(player(p, Some(case.model), None, head)).unwrap())
                        .unwrap();
                let name = format!("{}-W{:03}-H{}", p.id, case.model, head.unwrap_or(0));
                let expected = load(&art.join("grips-C12/composites").join(format!("{name}.png")));
                max = max.max(compare(&image, &expected, 256, [128., 159.], &name));
                c12 += 1;
            }
        }
        for fixed in &p.fixed_loadouts {
            let image = prepare_image(
                &selection(player(p, Some(fixed.main), Some(fixed.left), fixed.helmet)).unwrap(),
            )
            .unwrap();
            let loadout = if fixed.main == 70 {
                "dual-knives"
            } else {
                "pistol-knife"
            };
            let name = format!(
                "{}-{loadout}-H{}-B-back-r02",
                p.id,
                fixed.helmet.unwrap_or(0)
            );
            let expected = load(
                &art.join("knife-contact-C11/back-r02/composites")
                    .join(format!("{name}.png")),
            );
            assert_eq!(compare(&image, &expected, 256, [128., 159.], &name), 0);
            c11 += 1;
        }
    }
    assert_eq!((c11, c12), (168, 1104));
    let out = root.join("artifacts/vertical-walk-review-18/integration-C12");
    std::fs::create_dir_all(&out).unwrap();
    std::fs::write(out.join("pixel-parity-C12.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":"VERT18-C11-BACK-R02+C12","C11_exact_configurations":c11,"C12_configurations":c12,"whole_pose_comparisons":(c11+c12)*13,"max_premultiplied_rgba_delta":max,"color_limit":2,"alpha_limit":1})).unwrap()).unwrap();
}

#[test]
#[ignore = "complete accepted C06/C08/C09 bank comparison; explicit optimized validation"]
fn complete_runtime_bank_matches_approved_weapons_helmets_pairs_and_monsters() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let art = root.join("assets/source-art/vertical-walk-review-18/extension-C06");
    let mut count = 0;
    let mut max = 0;
    for p in manifest()
        .profiles
        .iter()
        .filter(|p| matches!(p.outfit % 5, 0 | 4))
    {
        for case in p.cases.iter().filter(|c| !c.left) {
            let image = prepare_image(&selection(player(p, Some(case.model), None, None)).unwrap())
                .unwrap();
            let name = format!("{}-W{:03}", p.id, case.model);
            let reference = reference_dir(&art, case.model, None, false);
            let expected = load(&reference.join(format!("{name}.png")));
            max = max.max(compare(&image, &expected, 256, [128., 160.], &name));
            count += 1;
        }
        for model in [2, 66, 88, 104] {
            for helmet in 115..=120 {
                let image =
                    prepare_image(&selection(player(p, Some(model), None, Some(helmet))).unwrap())
                        .unwrap();
                let name = format!("{}-W{model:03}-H{helmet}", p.id);
                let expected =
                    load(&reference_dir(&art, model, None, true).join(format!("{name}.png")));
                max = max.max(compare(&image, &expected, 256, [128., 160.], &name));
                count += 1;
            }
        }
        for (main, left) in [(104, 69), (104, 70), (104, 104), (68, 79)] {
            let image = prepare_image(&selection(player(p, Some(main), Some(left), None)).unwrap())
                .unwrap();
            let name = format!("{}-DUAL-{main}-{left}", p.id);
            let reference = reference_dir(&art, main, Some(left), true);
            let expected = load(&reference.join(format!("{name}.png")));
            max = max.max(compare(&image, &expected, 256, [128., 160.], &name));
            count += 1;
        }
    }
    let profiles: serde_json::Value = serde_json::from_slice(
        &std::fs::read(art.join("conditioned-monsters/manifest.json")).unwrap(),
    )
    .unwrap();
    for p in &manifest().monsters {
        let source = profiles["profiles"]
            .as_array()
            .unwrap()
            .iter()
            .find(|r| r["number"].as_u64() == Some(u64::from(p.number)))
            .unwrap();
        let side = source["side"].as_u64().unwrap() as u16;
        let foot = source["foot"].as_array().unwrap();
        let anchor = [
            foot[0].as_f64().unwrap() as f32,
            foot[1].as_f64().unwrap() as f32,
        ];
        let expected = load(&art.join(format!(
            "conditioned-monsters/palettes/MON-{:03}-v{}.png",
            p.number, p.variant
        )));
        let image = static_image(p).unwrap();
        max = max.max(compare(
            &image,
            &expected,
            side,
            anchor,
            &format!("MON-{}-{}", p.number, p.variant),
        ));
        count += 1;
    }
    assert_eq!(count, 1599);
    let out = root.join("artifacts/vertical-walk-review-18/integration");
    std::fs::create_dir_all(&out).unwrap();
    std::fs::write(out.join("pixel-parity.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":"VERT18-C06+C08-B+C09-B","configurations":count,"c09_configurations":168,"c08_configurations":104,"c06_unchanged_configurations":1327,"whole_pose_comparisons":count*13,"max_premultiplied_rgba_delta":max,"color_limit":2,"alpha_limit":1,"source_art_unchanged":true})).unwrap()).unwrap();
}
