//! C03: heavy outfits with independently held single/dual one-handed weapons.
use super::*;

const CASES: [&str; 5] = ["unarmed", "pistol", "blades", "pistols", "mixed"];
pub(super) fn subjects(case: usize) -> [Visual; 4] {
    [3, 5, 6, 7].map(|body| {
        let (weapon, hands) = match case {
            0 => (None, None),
            1 => (Some(104), None),
            2 => (None, Some([Some(70), Some(70)])),
            3 => (None, Some([Some(104), Some(104)])),
            _ => (Some(70), Some([Some(104), Some(70)])),
        };
        Visual::Player {
            body,
            equipment: [Some(103), None, None, None],
            weapon,
            hands,
            palettes: [EquipmentPalette::Original; 3],
        }
    })
}

async fn contacts() -> Result<usize, String> {
    let mut checked = 0;
    for visual in subjects(0) {
        let (_, id) = key(visual).ok_or("C03 missing profile")?;
        let profile = manifest().profiles.iter().find(|p| p.id == id).unwrap();
        let sheet = image(&profile.file)?;
        for model in 0..113 {
            if game_art::weapon_uses_two_hands(model) {
                continue;
            }
            for left in [true, false] {
                let Visual::Player {
                    body,
                    equipment,
                    palettes,
                    ..
                } = visual
                else {
                    unreachable!()
                };
                let visual = Visual::Player {
                    body,
                    equipment,
                    palettes,
                    weapon: None,
                    hands: Some(if left {
                        [Some(model), None]
                    } else {
                        [None, Some(model)]
                    }),
                };
                for frame in &profile.frames {
                    let back = frame.direction == "back";
                    let original = sheet.sub_image(Rect::new(
                        f32::from(frame.phase) * 128.,
                        if back { 128. } else { 0. },
                        128.,
                        128.,
                    ));
                    if game_art::vertical_review_composition(&original, visual, frame.palms, back)
                        .is_none()
                    {
                        return Err(format!(
                            "C03 grip missing: {id}, weapon {model}, left {left}, back {back}, phase {}",
                            frame.phase
                        ));
                    }
                    checked += 1;
                }
                if profile.size_revision {
                    let idle = image(&profile.standing)?;
                    if game_art::vertical_review_composition(
                        &idle,
                        visual,
                        profile.standing_palms.ok_or("C05 idle palms missing")?,
                        false,
                    )
                    .is_none()
                    {
                        return Err(format!(
                            "C05 idle grip missing: {id}, weapon {model}, left {left}"
                        ));
                    }
                    checked += 1;
                }
            }
        }
        next_frame().await;
    }
    Ok(checked)
}

pub(super) async fn flat(
    output: &Path,
    visuals: &[Visual; 4],
    review: &str,
) -> Result<usize, String> {
    let mut motion = CombatMotion::default();
    let mut front_returns = 0;
    for step in 0..2 {
        for row in 0..2 {
            let dy = if row == 0 { -1 } else { 1 };
            for col in 0..4 {
                let id = serde_json::from_str(&(row * 4 + col + 1).to_string()).unwrap();
                motion.glide_timed(
                    id,
                    GridPos::new(0, step * dy),
                    GridPos::new(0, (step + 1) * dy),
                    1. + f64::from(step),
                    400,
                );
            }
        }
        for tick in 0..13 {
            let now = 1.
                + f64::from(step)
                + if tick == 12 {
                    0.401
                } else {
                    (f64::from(tick) + 0.5) / 12. * 0.4
                };
            for warm in 0..2 {
                set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
                clear_background(Color::from_rgba(30, 40, 49, 255));
                draw_text(
                    &format!("{review} / armures et armes independantes / natif64"),
                    16.,
                    27.,
                    21.,
                    WHITE,
                );
                for (col, label) in [
                    "01 Bastion",
                    "02 Traqueuse",
                    "03 Arcaniste",
                    "04 Metamorphe",
                ]
                .into_iter()
                .enumerate()
                {
                    draw_text(label, 154. + col as f32 * 288., 58., 17., WHITE);
                }
                let _world = world_readability::world(false);
                for row in 0..2 {
                    let dy = if row == 0 { -1 } else { 1 };
                    for (col, &visual) in visuals.iter().enumerate() {
                        let id = serde_json::from_str(&(row * 4 + col + 1).to_string()).unwrap();
                        let pose = motion.vertical_walk_pose(id, now, false);
                        if tick == 12 && pose.is_some() {
                            return Err("C03 arrival did not release pose".into());
                        }
                        if tick < 12 && pose.is_none_or(|p| p.facing != dy * 2) {
                            return Err("C03 Y heading mismatch".into());
                        }
                        let y = if row == 0 { 234. } else { 414. }
                            + (step + 1) as f32 * dy as f32 * 64.
                            + motion.glide_offset(id, now, false).y * 64.;
                        let draws = world_readability::audit_textures(|| {
                            assert!(render(
                                Rect::new(168. + col as f32 * 288., y, 64., 64.),
                                visual,
                                1.,
                                pose
                            ))
                        });
                        if draws.is_empty()
                            || draws
                                .iter()
                                .any(|d| !d.world || !d.actor || !d.finish || d.gain != 1.)
                        {
                            return Err("C03 actor exposure mismatch".into());
                        }
                        if warm == 1 {
                            front_returns += usize::from(tick == 12);
                        }
                    }
                }
                if warm == 0 {
                    next_frame().await;
                } else {
                    crate::ui_capture::framebuffer()?.export_png(
                        output
                            .join(format!("step{step}-tick{tick:02}.png"))
                            .to_str()
                            .unwrap(),
                    );
                }
            }
        }
    }
    Ok(front_returns)
}

pub(super) async fn capture(output: &Path) -> Result<(), String> {
    struct Guard(bool);
    impl Drop for Guard {
        fn drop(&mut self) {
            ACTIVE.set(self.0);
        }
    }
    let _guard = Guard(ACTIVE.replace(true));
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(1240., 760.);
    for _ in 0..3 {
        next_frame().await;
    }
    let all_contacts = contacts().await?;
    let mut samples = 0;
    let mut front_returns = 0;
    for (case, name) in CASES.into_iter().enumerate() {
        let path = output.join(name);
        std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
        let visuals = subjects(case);
        samples += prepare(&visuals, &path).await?;
        front_returns += flat(&path, &visuals, "VERT18-C03").await?;
    }
    let mixed = subjects(4);
    for (name, pair) in [("camp-west", &mixed[..2]), ("camp-east", &mixed[2..])] {
        let path = output.join(name);
        std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
        capture_camp(&path, "VERT18-C03", pair).await?;
    }
    std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
        "review":"VERT18-C03","status":"review_only","normal_game_enabled":false,"profiles":4,
        "authored_moving_figures":48,"cases":CASES,"one_handed_models":39,"one_handed_hand_contacts":all_contacts,
        "sampled_weapon_contacts":samples,"front_returns":front_returns,"neutral_actor_gain":1.,"camp_actor_gain":1.48,
        "added_helmet_models_reviewed":false,"true_unhelmeted_heavy_heads_reviewed":false,"two_handed_grips_reviewed":false,
        "head_scope":"original head protection belonging to current heavy outfit; independent helmet/unhelmeted variants still pending",
        "read_or_decode_during_draw":false,"source_cache_limit_bytes":4194304,"texture_cache_limit_bytes":8388608,
        "saved_game_loaded":false,"user_preferences_written":false
    })).unwrap()).map_err(|e|e.to_string())?;
    Ok(())
}
