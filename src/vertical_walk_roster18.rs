//! VERT18-C02: next authored class silhouettes, isolated native review only.
use super::*;

pub(super) async fn capture(output: &Path) -> Result<(), String> {
    struct Guard(bool);
    impl Drop for Guard {
        fn drop(&mut self) {
            ACTIVE.set(self.0);
        }
    }
    let _guard = Guard(ACTIVE.replace(true));
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(1040., 760.);
    for _ in 0..3 {
        next_frame().await;
    }
    let visuals = [3, 5, 7].map(|body| Visual::Player {
        body,
        equipment: [None; 4],
        weapon: None,
        hands: None,
        palettes: [EquipmentPalette::Original; 3],
    });
    if prepare(&visuals, output).await? != 0 {
        return Err("VERT18-C02 must remain an unarmed silhouette review".into());
    }
    let mut motion = CombatMotion::default();
    let mut records = Vec::new();
    let mut front_returns = 0;
    for step in 0..2 {
        for row in 0..2 {
            let dy = if row == 0 { -1 } else { 1 };
            for col in 0..3 {
                let id = serde_json::from_str(&(row * 3 + col + 1).to_string()).unwrap();
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
                    "VERT18-C02 / autres classes / natif64",
                    16.,
                    27.,
                    21.,
                    WHITE,
                );
                for (col, label) in ["01 Bastion", "02 Traqueuse", "03 Metamorphe"]
                    .into_iter()
                    .enumerate()
                {
                    draw_text(label, 154. + col as f32 * 288., 58., 17., WHITE);
                }
                draw_text("Vers le haut : dos", 12., 89., 17., SKYBLUE);
                draw_text("Vers le bas : face", 12., 398., 17., SKYBLUE);
                draw_text(
                    if tick == 12 {
                        if SIZES.get() {
                            "Arret : pose frontale du meme cycle / C05"
                        } else {
                            "Arret : dessin frontal existant"
                        }
                    } else {
                        "Trois phases par pas / appuis alternes / 400 ms"
                    },
                    16.,
                    724.,
                    17.,
                    WHITE,
                );
                let _world = world_readability::world(false);
                for row in 0..2 {
                    let dy = if row == 0 { -1 } else { 1 };
                    for (col, &visual) in visuals.iter().enumerate() {
                        let id = serde_json::from_str(&(row * 3 + col + 1).to_string()).unwrap();
                        let pose = motion.vertical_walk_pose(id, now, false);
                        if tick == 12 && pose.is_some() {
                            return Err("VERT18-C02 retained movement at arrival".into());
                        }
                        if tick < 12 && pose.is_none_or(|p| p.facing != dy * 2) {
                            return Err("VERT18-C02 Y direction mismatch".into());
                        }
                        let offset = motion.glide_offset(id, now, false);
                        let y = if row == 0 { 234. } else { 414. }
                            + (step + 1) as f32 * dy as f32 * 64.
                            + offset.y * 64.;
                        let cell = Rect::new(168. + col as f32 * 288., y, 64., 64.);
                        let draws = world_readability::audit_textures(|| {
                            assert!(render(cell, visual, 1., pose));
                        });
                        if draws.is_empty()
                            || draws
                                .iter()
                                .any(|d| !d.world || !d.actor || !d.finish || d.gain != 1.)
                        {
                            return Err("VERT18-C02 neutral exposure mismatch".into());
                        }
                        if warm == 1 {
                            front_returns += usize::from(tick == 12);
                            records.push(serde_json::json!({"step":step,"tick":tick,"row":row,"subject":col,"pose":pose.map(|p|[i16::from(p.facing),i16::from(p.frame)]),"actor_gain":draws[0].gain}));
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
    std::fs::write(output.join("checks.json"), serde_json::to_vec_pretty(&serde_json::json!({"review":"VERT18-C02","status":"review_only","normal_game_enabled":false,"authored_profiles":3,"authored_moving_figures":36,"equipment_scope":"unarmed_only","native_tile_px":64,"front_returns":front_returns,"neutral_actor_gain":1.0,"saved_game_loaded":false,"user_preferences_written":false,"records":records})).unwrap()).map_err(|e| e.to_string())?;
    capture_camp(output, "VERT18-C02", &visuals).await
}
