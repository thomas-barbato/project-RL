//! C05-GL01: proposed shared grenade-launcher drawing; lateral grip review.
use super::*;

pub(super) fn lateral(visual: Visual, style: u8) -> Result<[Cycle; 2], String> {
    let mut sides = Vec::new();
    for facing in [-1, 1] {
        let mut sheet = Image::gen_image_color(768, 128, BLANK);
        let mut landmarks = Vec::new();
        for frame in 0..6 {
            let image = with_source(style, || {
                game_art::polish_review_lateral(visual, Pose { facing, frame })
            })
            .ok_or("GL01 missing lateral pose")?;
            for y in 0..128usize {
                let at = (y * 768 + usize::from(frame) * 128) * 4;
                sheet.bytes[at..at + 512].copy_from_slice(&image.bytes[y * 512..y * 512 + 512]);
            }
            let Visual::Player {
                body,
                equipment,
                palettes,
                ..
            } = visual
            else {
                unreachable!()
            };
            let bare = game_art::polish_review_lateral(
                Visual::Player {
                    body,
                    equipment: [equipment[0], None, None, None],
                    weapon: None,
                    hands: None,
                    palettes,
                },
                Pose { facing, frame },
            )
            .unwrap();
            landmarks.push(crown(&bare));
        }
        let mut heads = landmarks.clone();
        heads.extend(&landmarks);
        heads.push(landmarks[0]);
        sides.push(Cycle::new(
            &sheet,
            [128, 128],
            [64., 96.],
            roots(&heads)[..6].to_vec(),
        ));
    }
    Ok(sides.try_into().ok().unwrap())
}
fn source(style: u8) -> Image {
    with_source(style, || {
        game_art::vertical_review_weapon(23, false, EquipmentPalette::Original)
    })
    .unwrap()
    .0
}
fn texture(image: &Image) -> Texture2D {
    let t = crate::asset_loading::from_image(image);
    t.set_filter(FilterMode::Nearest);
    t
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(540., 490.);
    for _ in 0..3 {
        next_frame().await;
    }
    let styles = [0, 2, 3];
    let weapons = styles.map(|style| texture(&source(style)));
    let mut cases = 0;
    for (body, name) in [
        (3, "Bastion"),
        (5, "Traqueuse"),
        (6, "Arcaniste"),
        (7, "Metamorphe"),
    ] {
        for head in [None, Some(120)] {
            let visual = player(body, Some(103), 23, head);
            let cycles = styles
                .map(|s| lateral(visual, s))
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;
            let references = styles
                .map(|s| lateral(player(body, Some(103), 31, head), s.min(2)))
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;
            for tick in 0..6u8 {
                for warm in 0..2 {
                    set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
                    clear_background(Color::from_rgba(30, 40, 49, 255));
                    draw_text(
                        &format!("C05-GL01 / lance-grenades / {name} / natif64"),
                        12.,
                        24.,
                        16.,
                        WHITE,
                    );
                    for (col, title) in ["Actuel", "02 precedente", "03 / GL01 redessine"]
                        .into_iter()
                        .enumerate()
                    {
                        let x = 12. + col as f32 * 180.;
                        draw_text(title, x, 52., 15., WHITE);
                        draw_text("Gauche", x, 79., 13., LIGHTGRAY);
                        cycles[col][0].draw(
                            Rect::new(x + 40., 89., 64., 64.),
                            usize::from(tick),
                            col > 0,
                            false,
                        );
                        draw_text("Droite", x, 183., 13., LIGHTGRAY);
                        cycles[col][1].draw(
                            Rect::new(x + 40., 193., 64., 64.),
                            usize::from(tick),
                            col > 0,
                            false,
                        );
                        draw_text("Mitrailleuse temoin", x, 286., 12., LIGHTGRAY);
                        references[col][1].draw(
                            Rect::new(x + 40., 295., 64., 64.),
                            usize::from(tick),
                            col > 0,
                            false,
                        );
                        draw_text("Dessin source x2", x, 387., 13., LIGHTGRAY);
                        crate::world_readability::texture(
                            &weapons[col],
                            x + 2.,
                            397.,
                            WHITE,
                            DrawTextureParams {
                                source: Some(Rect::new(30., 40., 70., 40.)),
                                dest_size: Some(vec2(140., 80.)),
                                ..Default::default()
                            },
                        );
                    }
                    if warm == 0 {
                        next_frame().await;
                    } else {
                        crate::ui_capture::framebuffer()?.export_png(
                            output
                                .join(format!("gl-{body}-H{}-{tick}.png", head.unwrap_or(0)))
                                .to_str()
                                .unwrap(),
                        );
                    }
                }
            }
            cases += 1;
            eprintln!("[GL01] {name} / H{}", head.unwrap_or(0));
            if body == 3 && head == Some(120) {
                camp(output, &cycles).await?;
            }
        }
    }
    std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
        "review":"ANIM-POLISH01-C05-GL01-r01","status":"unapproved_drawing_candidate","normal_game_enabled":false,
        "configurations":cases,"poses_per_direction":6,"directions":2,"native_tile_px":64,"source_bounds":[44,53,88,71],
        "rear_grip":[59,68],"front_support_grip":[77,68],"source_dimensions":[44,18],"previous_dimensions":[52,24],
        "weapon_colors":"original only","front_back_poses":"not changed or proposed; require adaptation to the new grip spacing",
        "saved_game_loaded":false,"preferences_written":false
    })).unwrap()).map_err(|e|e.to_string())?;
    Ok(())
}
async fn camp(output: &Path, cycles: &[[Cycle; 2]]) -> Result<(), String> {
    let scene = crate::cryogenic_camp::scene();
    let load = |path: &str| -> Result<Texture2D, String> {
        let image = crate::asset_preparation::image(path)?;
        Ok(texture(image.as_ref()))
    };
    let floor = load(&scene.floor)?;
    let structure = load(&scene.structure)?;
    let atmosphere = crate::camp_atmosphere::Atmosphere::new(
        &scene.atmosphere,
        scene.width as u32 * 64,
        scene.height as u32 * 64,
    );
    request_new_screen_size(640., 610.);
    for _ in 0..3 {
        next_frame().await;
    }
    let world = Rect::new(
        16.,
        -260.,
        scene.width as f32 * 64.,
        scene.height as f32 * 64.,
    );
    let clip = Rect::new(16., 60., 576., 512.);
    for warm in 0..2 {
        set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
        clear_background(Color::from_rgba(30, 40, 49, 255));
        draw_text(
            "C05-GL01 / actuel - 02 precedente - 03 redessine / natif64",
            12.,
            25.,
            15.,
            WHITE,
        );
        {
            let _world = crate::world_readability::world(false);
            atmosphere.begin(world);
            atmosphere.background_time(0.);
            atmosphere.begin_background(true);
            for t in [&floor, &structure] {
                draw_texture_ex(
                    t,
                    world.x,
                    world.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(world.size()),
                        ..Default::default()
                    },
                );
            }
            atmosphere.end_material();
            for (col, cycle) in cycles.iter().enumerate() {
                for direction in 0..2 {
                    cycle[direction].draw(
                        Rect::new(
                            world.x + (2. + col as f32 * 2.) * 64.,
                            world.y + (8. + direction as f32 * 2.) * 64.,
                            64.,
                            64.,
                        ),
                        1,
                        col > 0,
                        false,
                    );
                }
            }
            atmosphere.finish(world, clip, true);
            unsafe {
                macroquad::window::get_internal_gl().quad_gl.scissor(None);
            }
        }
        if warm == 0 {
            next_frame().await;
        } else {
            crate::ui_capture::framebuffer()?
                .export_png(output.join("gl-camp.png").to_str().unwrap());
        }
    }
    next_frame().await;
    request_new_screen_size(540., 490.);
    for _ in 0..3 {
        next_frame().await;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn redraw_changes_only_the_scoped_grenade_source_and_keeps_two_visible_grips() {
        let old = source(0);
        let new = source(3);
        assert!(old.bytes != new.bytes);
        for [x, y] in [[59usize, 68usize], [77, 68]] {
            assert!(
                (y - 1..=y + 1)
                    .any(|yy| (x - 1..=x + 1).any(|xx| new.bytes[(yy * 128 + xx) * 4 + 3] >= 64)),
                "GL01 grip has no painted surface"
            );
        }
        assert!(
            source(0).bytes == old.bytes,
            "GL01 contaminated the accepted source"
        );
        let v = player(3, Some(103), 23, Some(120));
        let pose = Pose {
            facing: 1,
            frame: 1,
        };
        let baseline = game_art::polish_review_lateral(v, pose).unwrap();
        let proposal = with_source(3, || game_art::polish_review_lateral(v, pose)).unwrap();
        assert!(baseline.bytes != proposal.bytes);
        assert!(baseline.bytes == game_art::polish_review_lateral(v, pose).unwrap().bytes);
        assert_eq!(source_style(), 0);
    }
}
