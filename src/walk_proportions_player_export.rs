//! Explicit read-only audit of front/walk body geometry, excluding held objects.
use super::*;
use serde_json::{Value, json};

#[test]
#[ignore = "Read-only arm projection review; RL_ARM14_OUTPUT required"]
fn export_arm_projection14_review() {
    let _legacy_dual = crate::game_art::walking::review_dual15(false);
    struct Restore(bool);
    impl Drop for Restore {
        fn drop(&mut self) {
            ARM_PROJECTION14.set(self.0);
        }
    }
    let _restore = Restore(ARM_PROJECTION14.replace(false));
    let output = std::path::PathBuf::from(std::env::var("RL_ARM14_OUTPUT").unwrap());
    std::fs::create_dir_all(&output).unwrap();
    let mut records = Vec::new();
    let mut failures = Vec::new();
    let mut checked = 0;
    for index in (0..5).chain(10..25) {
        for (name, held) in [
            ("rifle", selection(Some(2), None)),
            ("axe", selection(Some(67), None)),
            ("dual", selection(None, Some([Some(68), Some(101)]))),
        ] {
            let key = (BASE + index, held, [EquipmentPalette::Original; 3]);
            let front = compose(super::super::layers(key).unwrap(), true);
            let normal: Vec<_> = [-1, 1]
                .into_iter()
                .flat_map(|facing| (0..6).map(move |frame| Pose { facing, frame }))
                .map(|p| layers(key, p).unwrap())
                .collect();
            let mut variants = Vec::new();
            for proposed in [false, true] {
                ARM_PROJECTION14.set(proposed);
                let mut atlas = Image::gen_image_color(13 * 128, 128, BLANK);
                for y in 0..128usize {
                    atlas.bytes[(y * 1664) * 4..(y * 1664 + 128) * 4]
                        .copy_from_slice(&front.bytes[y * 512..(y + 1) * 512]);
                }
                let mut details = Vec::new();
                for (n, pose) in [-1, 1]
                    .into_iter()
                    .flat_map(|facing| (0..6).map(move |frame| Pose { facing, frame }))
                    .enumerate()
                {
                    let parts = layers(key, pose).unwrap();
                    assert_eq!(
                        parts[0].bytes, normal[n][0].bytes,
                        "body/helmet changed: {index} {name}"
                    );
                    if proposed {
                        let gear = &parts[3];
                        let fingers = &parts[4];
                        let mut todo: std::collections::BTreeSet<(i32, i32)> = fingers
                            .bytes
                            .chunks_exact(4)
                            .enumerate()
                            .filter(|(_, p)| p[3] >= 96)
                            .map(|(i, _)| ((i % 128) as i32, (i / 128) as i32))
                            .collect();
                        let mut islands = 0;
                        while let Some(seed) = todo.pop_first() {
                            let mut component = vec![seed];
                            let mut stack = vec![seed];
                            while let Some((x, y)) = stack.pop() {
                                for dx in -1..=1 {
                                    for dy in -1..=1 {
                                        let q = (x + dx, y + dy);
                                        if todo.remove(&q) {
                                            component.push(q);
                                            stack.push(q);
                                        }
                                    }
                                }
                            }
                            if component.len() < 2 {
                                continue;
                            }
                            islands += 1;
                            if !component.iter().any(|&(x, y)| {
                                gear.bytes[(y as usize * 128 + x as usize) * 4 + 3] >= 64
                            }) {
                                failures.push(json!({"index":index,"loadout":name,"facing":pose.facing,"frame":pose.frame,"island":component}));
                            }
                        }
                        if islands == 0 {
                            failures.push(json!({"index":index,"loadout":name,"facing":pose.facing,"frame":pose.frame,"missing_fingers":true}));
                        }
                        checked += 1;
                    }
                    details.push(json!({"facing":pose.facing,"frame":pose.frame,"arm_opaque_area":parts[1..3].iter().map(|a|a.bytes.chunks_exact(4).filter(|p|p[3]>=96).count()).sum::<usize>()}));
                    let image = compose(parts, true);
                    for y in 0..128usize {
                        let at = (y * 1664 + (n + 1) * 128) * 4;
                        atlas.bytes[at..at + 512]
                            .copy_from_slice(&image.bytes[y * 512..(y + 1) * 512]);
                    }
                }
                let file = format!(
                    "CLASS-{index:02}-{name}-{}.png",
                    if proposed { "candidate" } else { "current" }
                );
                assert!(!output.join(&file).exists(), "review output exists");
                atlas.export_png(output.join(&file).to_str().unwrap());
                variants.push(json!({"proposed":proposed,"file":file,"poses":details}));
            }
            ARM_PROJECTION14.set(true);
            records.push(json!({"index":index,"loadout":name,"reference_frames":[walking_arm_reference(key,Pose{facing:-1,frame:0}),walking_arm_reference(key,Pose{facing:1,frame:0})],"variants":variants}));
            ARM_PROJECTION14.set(false);
        }
        eprintln!("[ARM14] CLASS-{index:02}: three loadouts, both directions, six phases");
    }
    std::fs::write(output.join("review.json"),serde_json::to_vec_pretty(&json!({"id":"ARM14-r01","normal_game_changed":false,"native_tile_px":64,"exported_pngs_need_vertical_flip":true,"contact_cases":checked,"contact_failures":failures,"cases":records})).unwrap()).unwrap();
    assert!(!ARM_PROJECTION14.get());
}

#[test]
#[ignore = "Full ARM14 integration contact matrix; RL_ARM14_INTEGRATION_OUTPUT required"]
fn audit_arm14_all_outfits_and_weapons() {
    let output = std::path::PathBuf::from(std::env::var("RL_ARM14_INTEGRATION_OUTPUT").unwrap());
    std::fs::create_dir_all(&output).unwrap();
    assert!(ARM_PROJECTION14.get());
    assert!(crate::game_art::walking::gait12_enabled());
    let mut cases = 0;
    let mut failures = Vec::new();
    let mut references = Vec::new();
    for index in 0..25 {
        for facing in [-1, 1] {
            let empty = (BASE + index, [None; 2], [EquipmentPalette::Original; 3]);
            let frame = walking_arm_reference(empty, Pose { facing, frame: 0 });
            assert!(
                frame < 6,
                "missing current-cycle reference: {index} {facing}"
            );
            references.push(json!({"index":index,"facing":facing,"frame":frame}));
            for weapon in 0..HELD_WEAPON_COUNT {
                let key = (
                    BASE + index,
                    selection(Some(weapon), None),
                    [EquipmentPalette::Original; 3],
                );
                let parts = layers(key, Pose { facing, frame }).unwrap();
                let gear = &parts[parts.len() - 2];
                let fingers = &parts[parts.len() - 1];
                let mut todo: std::collections::BTreeSet<(i32, i32)> = fingers
                    .bytes
                    .chunks_exact(4)
                    .enumerate()
                    .filter(|(_, p)| p[3] >= 96)
                    .map(|(i, _)| ((i % 128) as i32, (i / 128) as i32))
                    .collect();
                let mut islands = 0;
                let mut gaps = 0;
                while let Some(seed) = todo.pop_first() {
                    let mut component = vec![seed];
                    let mut stack = vec![seed];
                    while let Some((x, y)) = stack.pop() {
                        for dx in -1..=1 {
                            for dy in -1..=1 {
                                let q = (x + dx, y + dy);
                                if todo.remove(&q) {
                                    component.push(q);
                                    stack.push(q);
                                }
                            }
                        }
                    }
                    if component.len() < 2 {
                        continue;
                    }
                    islands += 1;
                    if !component
                        .iter()
                        .any(|&(x, y)| gear.bytes[(y as usize * 128 + x as usize) * 4 + 3] >= 64)
                    {
                        gaps += 1;
                    }
                }
                if islands == 0 || gaps > 0 {
                    failures.push(json!({"index":index,"weapon":weapon,"facing":facing,"frame":frame,"hand_islands":islands,"gaps":gaps}));
                }
                cases += 1;
            }
        }
        eprintln!(
            "[ARM14-INTEGRATION] outfit{index:02}: all113 weapons, both directions; {} failures",
            failures.len()
        );
    }
    std::fs::write(output.join("all-weapons-contact.json"),serde_json::to_vec_pretty(&json!({"review_id":"ARM14-INTEGRATION01","outfits":25,"legacy_outfits_retained":5,"weapon_models":HELD_WEAPON_COUNT,"contact_cases":cases,"failures":failures,"reference_frames":references,"scope":"each outfit x each weapon x both directions at the stable assembly pose; six-phase stability and all helmets checked separately","normal_renderer_arm14_enabled":true})).unwrap()).unwrap();
    assert!(
        failures.is_empty(),
        "{} hand/weapon failures; see all-weapons-contact.json",
        failures.len()
    );
}

fn compose(parts: Vec<Image>, include_weapon: bool) -> Image {
    let weapon = parts.len() - 2;
    let mut result = Image::gen_image_color(128, 128, BLANK);
    for (index, part) in parts.into_iter().enumerate() {
        if include_weapon || index != weapon {
            geometry::over(&mut result, &part);
        }
    }
    result
}

fn metrics(image: &Image) -> Value {
    let (mut x0, mut y0, mut x1, mut y1, mut area) = (128usize, 128usize, 0usize, 0usize, 0usize);
    for (index, p) in image.bytes.chunks_exact(4).enumerate() {
        if p[3] >= 96 {
            let (x, y) = (index % 128, index / 128);
            x0 = x0.min(x);
            y0 = y0.min(y);
            x1 = x1.max(x + 1);
            y1 = y1.max(y + 1);
            area += 1;
        }
    }
    assert!(area > 0);
    let h = y1 - y0;
    let top = y0 + (h as f64 * 0.27).round() as usize;
    let end = y0 + (h as f64 * 0.55).round() as usize;
    let band = (top..end)
        .flat_map(|y| (0..128).map(move |x| (x, y)))
        .filter(|(x, y)| image.bytes[(y * 128 + x) * 4 + 3] >= 96)
        .count();
    json!({"bounds":[x0,y0,x1,y1],"height":h,"width":x1-x0,"opaque_area":area,
        "upper_body_mean_occupied_width":band as f64/(end-top) as f64})
}

fn one_case(
    output: &std::path::Path,
    index: u16,
    head: u16,
    weapon: Option<u16>,
    all_poses: bool,
    save: bool,
) -> Value {
    let key = (
        BASE + index + head * HEAD_STRIDE,
        selection(weapon, None),
        [EquipmentPalette::Original; 3],
    );
    let front_parts = super::super::layers(key).unwrap();
    let front = metrics(&compose(front_parts.clone(), false));
    let mut atlas = Image::gen_image_color(1920, 128, BLANK);
    let mut put = |column: usize, image: &Image| {
        for y in 0..128usize {
            let at = (y * 1920 + column * 128) * 4;
            atlas.bytes[at..at + 512].copy_from_slice(&image.bytes[y * 512..y * 512 + 512]);
        }
    };
    if save {
        put(0, &compose(front_parts, true));
    }
    let mut poses = Vec::new();
    let mut column = 1;
    let frames: &[u8] = if all_poses {
        &[8, 0, 1, 2, 3, 4, 5]
    } else {
        &[8, 1]
    };
    for facing in [-1, 1] {
        for &frame in frames {
            let parts = layers(key, Pose { facing, frame }).unwrap();
            poses.push(json!({"facing":facing,"frame":frame,"geometry":metrics(&compose(parts.clone(),false))}));
            if save {
                put(column, &compose(parts, true));
                column += 1;
            }
        }
    }
    let file = format!(
        "CLASS-{index:02}-head{head}-{}.png",
        weapon.map_or("unarmed".to_owned(), |w| format!("weapon{w:03}"))
    );
    if save {
        atlas.export_png(output.join(&file).to_str().unwrap());
    }
    json!({"id":format!("CLASS-{index:02}"),"outfit_index":index,"helmet":head,"weapon":weapon,
        "front":front,"poses":poses,"atlas":if save{Some(file)}else{None},"held_weapon_excluded_from_metrics":true})
}

#[test]
#[ignore = "Explicit all-outfit helmet review; RL_WALK_HELMET_OUTPUT required"]
fn walk_helmet_alignment_diagnostic() {
    let output = std::path::PathBuf::from(std::env::var("RL_WALK_HELMET_OUTPUT").unwrap());
    std::fs::create_dir_all(&output).unwrap();
    let mut cases = Vec::new();
    for index in 0..25 {
        for head in 0..=6 {
            cases.push(one_case(&output, index, head, None, true, true));
        }
        cases.push(one_case(&output, index, 3, Some(67), true, true));
    }
    std::fs::write(
        output.join("helmets.json"),
        serde_json::to_vec_pretty(&json!({
            "source": "normal Rust compositor", "native_tile_px": 64,
            "exported_pngs_need_vertical_flip": true, "cases": cases
        }))
        .unwrap(),
    )
    .unwrap();
}

#[test]
#[ignore = "Explicit full geometry audit; RL_PROPORTIONS_OUTPUT required"]
fn audit_all_player_proportions_diagnostic() {
    let output = std::path::PathBuf::from(
        std::env::var("RL_PROPORTIONS_OUTPUT").expect("audit output required"),
    )
    .join("players");
    std::fs::create_dir_all(&output).unwrap();
    assert!(!crate::game_art::walking::corrections_enabled());
    assert!(!PROPORTIONS10.get());
    let candidate = std::env::var_os("RL_PROPORTIONS_CANDIDATE").is_some();
    let _review = candidate.then(crate::game_art::walking::review_corrections);
    let _common = crate::game_art::walking::review_common(true);
    let _proportions = crate::game_art::walking::review_proportions(candidate);
    let mut bodies = Vec::new();
    let mut weapons = Vec::new();
    for index in 0..25 {
        for head in 0..=6 {
            bodies.push(one_case(&output, index, head, None, true, true));
        }
        bodies.push(one_case(&output, index, 0, Some(67), true, true));
        if index % 5 == 0 {
            for weapon in 0..HELD_WEAPON_COUNT {
                weapons.push(one_case(&output, index, 0, Some(weapon), false, false));
            }
        }
        eprintln!("[PROPORTIONS] CLASS-{index:02}: all helmets/14 poses, axe/14 poses");
    }
    assert_eq!(bodies.len(), 200);
    assert_eq!(weapons.len(), 565);
    std::fs::write(output.join("geometry.json"),serde_json::to_vec_pretty(&json!({
        "normal_game_changed":false,"normal_renderer_context":!candidate,"candidate_walk11":candidate,"exported_pngs_need_vertical_flip":true,
        "alpha_threshold":96,"body_cases":bodies,"weapon_cases":weapons,
        "weapon_sampling":"all113 models on each class first outfit, idle/passing in both directions; all outfits axe67 at all14 poses",
        "not_tested":"full armor x helmet x weapon x pose Cartesian product"
    })).unwrap()).unwrap();
}

#[test]
#[ignore = "Refresh only the two revised outfits in the existing WALK11 full export"]
fn audit_changed_player_proportions_diagnostic() {
    let output =
        std::path::PathBuf::from(std::env::var("RL_PROPORTIONS_OUTPUT").unwrap()).join("players");
    let file = output.join("geometry.json");
    let mut previous: Value = serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
    assert_eq!(previous["candidate_walk11"], true);
    let _review = crate::game_art::walking::review_corrections();
    let _common = crate::game_art::walking::review_common(true);
    let _proportions = crate::game_art::walking::review_proportions(true);
    let mut changed = Vec::new();
    for index in [21, 22] {
        for head in 0..=6 {
            changed.push(one_case(&output, index, head, None, true, true));
        }
        changed.push(one_case(&output, index, 0, Some(67), true, true));
    }
    let bodies = previous["body_cases"].as_array_mut().unwrap();
    for case in changed {
        let entry = bodies
            .iter_mut()
            .find(|old| {
                old["id"] == case["id"]
                    && old["helmet"] == case["helmet"]
                    && old["weapon"] == case["weapon"]
            })
            .unwrap();
        *entry = case;
    }
    assert_eq!(bodies.len(), 200);
    previous["refreshed_outfits"] = json!([21, 22]);
    std::fs::write(file, serde_json::to_vec_pretty(&previous).unwrap()).unwrap();
}

#[test]
#[ignore = "Explicit hand/weapon pixel-contact audit; RL_WALK11_GRIP_OUTPUT required"]
fn walk11_grip_contact_diagnostic() {
    let output = std::path::PathBuf::from(std::env::var("RL_WALK11_GRIP_OUTPUT").unwrap());
    std::fs::create_dir_all(&output).unwrap();
    let _review = crate::game_art::walking::review_corrections();
    let _common = crate::game_art::walking::review_common(true);
    let _proportions = crate::game_art::walking::review_proportions(true);
    let proposed = std::env::var_os("RL_WALK12_PROPOSED").is_some();
    let _gait = crate::game_art::walking::review_gait12(proposed);
    let mut records = Vec::new();
    let mut failures = Vec::new();
    for index in 0..25 {
        let key = (BASE + index, [None; 2], [EquipmentPalette::Original; 3]);
        for facing in [-1, 1] {
            for frame in [8, 0, 1, 2, 3, 4, 5] {
                let (_, parts, _) = direction_parts(key, Pose { facing, frame }).unwrap();
                for side in 0..2 {
                    assert!(
                        parts[2 + side * 2]
                            .bytes
                            .chunks_exact(4)
                            .filter(|p| p[3] >= 96)
                            .count()
                            >= 2,
                        "missing hand: outfit{index} facing{facing} frame{frame} side{side}"
                    );
                }
                let mut expected = parts[0].clone();
                geometry::over(&mut expected, &parts[1]);
                geometry::over(&mut expected, &parts[3]);
                assert_eq!(
                    compose(layers(key, Pose { facing, frame }).unwrap(), true).bytes,
                    expected.bytes,
                    "unarmed figure changed: {index} {facing} {frame}"
                );
            }
        }
    }
    let mut inspect = |index: u16,
                       weapon: Option<u16>,
                       hands: Option<[Option<u16>; 2]>,
                       frames: &[u8],
                       save: bool| {
        let key = (
            BASE + index,
            selection(weapon, hands),
            [EquipmentPalette::Original; 3],
        );
        if save && weapon == Some(67) {
            compose(front_layers(key).unwrap(), true).export_png(
                output
                    .join(format!("CLASS-{index:02}-weapon067-front.png"))
                    .to_str()
                    .unwrap(),
            );
        }
        let mut atlas = Image::gen_image_color((frames.len() * 2 * 128) as u16, 128, BLANK);
        let mut column = 0;
        for facing in [-1, 1] {
            for &frame in frames {
                let p = layers(key, Pose { facing, frame }).unwrap();
                let gear = &p[p.len() - 2];
                let fingers = &p[p.len() - 1];
                let mut todo: std::collections::BTreeSet<(i32, i32)> = fingers
                    .bytes
                    .chunks_exact(4)
                    .enumerate()
                    .filter(|(_, p)| p[3] >= 96)
                    .map(|(i, _)| ((i % 128) as i32, (i / 128) as i32))
                    .collect();
                let mut distances = Vec::new();
                while let Some(seed) = todo.pop_first() {
                    let mut component = vec![seed];
                    let mut stack = vec![seed];
                    while let Some((x, y)) = stack.pop() {
                        for dx in -1..=1 {
                            for dy in -1..=1 {
                                let q = (x + dx, y + dy);
                                if todo.remove(&q) {
                                    component.push(q);
                                    stack.push(q);
                                }
                            }
                        }
                    }
                    if component.len() < 2 {
                        continue;
                    }
                    let distance = component
                        .iter()
                        .flat_map(|&(x, y)| {
                            (-4..=4).flat_map(move |dx| {
                                (-4..=4).map(move |dy| (x + dx, y + dy, dx * dx + dy * dy))
                            })
                        })
                        .filter(|&(x, y, _)| {
                            (0..128).contains(&x)
                                && (0..128).contains(&y)
                                && gear.bytes[(y as usize * 128 + x as usize) * 4 + 3] >= 64
                        })
                        .map(|(_, _, d)| d)
                        .min()
                        .unwrap_or(100);
                    distances.push(distance);
                }
                let record = json!({"index":index,"weapon":weapon,"hands":hands,"facing":facing,"frame":frame,"hand_distance_squared":distances});
                if distances.is_empty() || distances.iter().any(|d| *d != 0) {
                    failures.push(record.clone());
                }
                records.push(record);
                if save {
                    let im = compose(p, true);
                    for y in 0..128usize {
                        let at = (y * atlas.width as usize + column * 128) * 4;
                        atlas.bytes[at..at + 512]
                            .copy_from_slice(&im.bytes[y * 512..y * 512 + 512]);
                    }
                }
                column += 1;
            }
        }
        if save {
            atlas.export_png(
                output
                    .join(format!(
                        "CLASS-{index:02}-{}.png",
                        weapon.map_or("dual".to_owned(), |w| format!("weapon{w:03}"))
                    ))
                    .to_str()
                    .unwrap(),
            );
        }
    };
    for index in 0..25 {
        inspect(index, Some(67), None, &[8, 0, 1, 2, 3, 4, 5], true);
        inspect(index, None, Some([Some(68), Some(101)]), &[8, 1], true);
        if index % 5 == 0 {
            for weapon in 0..HELD_WEAPON_COUNT {
                if weapon != 67 {
                    inspect(index, Some(weapon), None, &[8, 1], true);
                }
            }
        }
        eprintln!("[GRIPS] outfit{index:02}");
    }
    std::fs::write(output.join("contact.json"),serde_json::to_vec_pretty(&json!({"revision":if proposed{"WALK12-COMMON-r01"}else{"WALK11-GRIP-r04-FRONT"},"cases":records,"failures":failures,"threshold":"each painted hand island of at least2 pixels at alpha96 must overlap gear alpha64; zero pixel gap","normal_game_changed":false,"proposed_gait":proposed})).unwrap()).unwrap();
    eprintln!(
        "[GRIPS] {} sampled composites, {} contact failures",
        records.len(),
        failures.len()
    );
    assert!(
        failures.is_empty(),
        "hand/weapon contact failures: {}",
        failures.len()
    );
}

#[test]
#[ignore = "Refresh the25 equipped review atlases after hand ownership corrections"]
fn walk11_refresh_equipped_review_diagnostic() {
    let output =
        std::path::PathBuf::from(std::env::var("RL_PROPORTIONS_OUTPUT").unwrap()).join("players");
    let file = output.join("geometry.json");
    let mut previous: Value = serde_json::from_slice(&std::fs::read(&file).unwrap()).unwrap();
    let _review = crate::game_art::walking::review_corrections();
    let _common = crate::game_art::walking::review_common(true);
    let _proportions = crate::game_art::walking::review_proportions(true);
    let bodies = previous["body_cases"].as_array_mut().unwrap();
    for index in 0..25 {
        let case = one_case(&output, index, 0, Some(67), true, true);
        let old = bodies
            .iter_mut()
            .find(|old| {
                old["id"] == case["id"]
                    && old["helmet"] == case["helmet"]
                    && old["weapon"] == case["weapon"]
            })
            .unwrap();
        *old = case;
    }
    previous["grip_revision"] = json!("WALK11-GRIP-r03");
    std::fs::write(file, serde_json::to_vec_pretty(&previous).unwrap()).unwrap();
}
