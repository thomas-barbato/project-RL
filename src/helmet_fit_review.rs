//! C06-HELM01: fixed wearer scale and a neck anchor, isolated from normal play.
use super::*;
thread_local! {static FIT:Cell<bool>=const{Cell::new(false)};}
pub(crate) fn enabled() -> bool {
    FIT.get()
}
fn with_fit<T>(value: bool, f: impl FnOnce() -> T) -> T {
    struct Guard(bool);
    impl Drop for Guard {
        fn drop(&mut self) {
            FIT.set(self.0);
        }
    }
    let _guard = Guard(FIT.replace(value));
    f()
}
pub(crate) fn fit_scale(outfit: usize) -> f32 {
    // One proposal per physical wearer, independent of armor tier, helmet and
    // phase. Legacy Technomancien keeps its current size outside this pilot.
    [1.15, 1.0, 0.86, 0.96, 1.06]
        .get(outfit / 5)
        .copied()
        .unwrap_or(1.)
}
fn sample(source: &Image, x: f32, y: f32) -> [u8; 4] {
    let mut rgba = [0.; 4];
    let ix = x.floor() as i32;
    let iy = y.floor() as i32;
    for yy in iy..=iy + 1 {
        for xx in ix..=ix + 1 {
            if xx < 0 || yy < 0 || xx >= i32::from(source.width) || yy >= i32::from(source.height) {
                continue;
            }
            let w = (1. - (x - xx as f32).abs()) * (1. - (y - yy as f32).abs());
            let p = (yy as usize * usize::from(source.width) + xx as usize) * 4;
            let alpha = f32::from(source.bytes[p + 3]) * w;
            rgba[3] += alpha;
            for c in 0..3 {
                rgba[c] += f32::from(source.bytes[p + c]) * alpha;
            }
        }
    }
    if rgba[3] < 0.5 {
        return [0; 4];
    }
    [
        (rgba[0] / rgba[3]).round() as u8,
        (rgba[1] / rgba[3]).round() as u8,
        (rgba[2] / rgba[3]).round() as u8,
        rgba[3].round() as u8,
    ]
}
pub(crate) fn place(
    source: &Image,
    w: u16,
    h: u16,
    pivot: [f32; 2],
    target: [f32; 2],
    m: [f32; 4],
) -> Image {
    let mut out = Image::gen_image_color(w, h, BLANK);
    let mut bound = [i32::from(source.width), i32::from(source.height), 0, 0];
    for (i, p) in source.bytes.chunks_exact(4).enumerate() {
        if p[3] > 0 {
            let x = (i % usize::from(source.width)) as i32;
            let y = (i / usize::from(source.width)) as i32;
            bound[0] = bound[0].min(x);
            bound[1] = bound[1].min(y);
            bound[2] = bound[2].max(x + 1);
            bound[3] = bound[3].max(y + 1);
        }
    }
    if bound[2] == 0 {
        return out;
    }
    let mut extent = [
        f32::INFINITY,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NEG_INFINITY,
    ];
    for x in [bound[0] as f32 - 1., bound[2] as f32 + 1.] {
        for y in [bound[1] as f32 - 1., bound[3] as f32 + 1.] {
            let dx = x - pivot[0];
            let dy = y - pivot[1];
            let xx = target[0] + m[0] * dx + m[1] * dy;
            let yy = target[1] + m[2] * dx + m[3] * dy;
            extent[0] = extent[0].min(xx);
            extent[1] = extent[1].min(yy);
            extent[2] = extent[2].max(xx);
            extent[3] = extent[3].max(yy);
        }
    }
    let det = m[0] * m[3] - m[1] * m[2];
    for y in (extent[1].floor() as i32 - 1).clamp(0, i32::from(h))
        ..(extent[3].ceil() as i32 + 1).clamp(0, i32::from(h))
    {
        for x in (extent[0].floor() as i32 - 1).clamp(0, i32::from(w))
            ..(extent[2].ceil() as i32 + 1).clamp(0, i32::from(w))
        {
            let dx = x as f32 + 0.5 - target[0];
            let dy = y as f32 + 0.5 - target[1];
            let pixel = sample(
                source,
                pivot[0] + (m[3] * dx - m[1] * dy) / det - 0.5,
                pivot[1] + (-m[2] * dx + m[0] * dy) / det - 0.5,
            );
            let p = (y as usize * usize::from(w) + x as usize) * 4;
            out.bytes[p..p + 4].copy_from_slice(&pixel);
        }
    }
    out
}
fn blend(d: &mut [u8], s: &[u8]) {
    if s[3] == 0 {
        return;
    }
    let a = u32::from(s[3]);
    let old = u32::from(d[3]) * (255 - a);
    let total = a * 255 + old;
    for c in 0..3 {
        d[c] = ((u32::from(s[c]) * a * 255 + u32::from(d[c]) * old + total / 2) / total) as u8;
    }
    d[3] = ((total + 127) / 255) as u8;
}
pub(crate) fn vertical(
    body: &mut Image,
    files: [&str; 4],
    at: [f32; 2],
    back: bool,
    model: u16,
    outfit: usize,
) -> Result<(), String> {
    let atlas = crate::asset_preparation::image(files[usize::from(back)])?;
    let opening = crate::asset_preparation::image(files[2])?;
    let own = crate::asset_preparation::image(files[3])?;
    let rect = Rect::new(f32::from(model - 115) * 32., 0., 32., 32.);
    let cover = atlas.sub_image(rect);
    let holes = opening.sub_image(rect);
    let s = fit_scale(outfit);
    let target = [at[0], at[1] + 6.];
    let shell = place(&cover, 256, 256, [16., 22.], target, [s, 0., 0., s]);
    let holes = place(&holes, 256, 256, [16., 22.], target, [s, 0., 0., s]);
    let before = place(&own, 256, 256, [16., 22.], target, [1., 0., 0., 1.]);
    let after = place(&own, 256, 256, [16., 22.], target, [s, 0., 0., s]);
    for (i, p) in body.bytes.chunks_exact_mut(4).enumerate() {
        let offset = i * 4;
        let owned = before.bytes[offset + 3] > 32 || after.bytes[offset + 3] > 32;
        let restore = if back {
            matches!(model, 115 | 119) && owned && shell.bytes[offset + 3] < 220
        } else {
            holes.bytes[offset + 3] > 32
        };
        // The lower shell edge stays on the neck; its scale cannot erase the
        // first neck row or any armor below it.
        if owned && !restore && ((i / 256) as f32) < at[1] + 6. {
            p.fill(0);
        }
        blend(p, &shell.bytes[offset..offset + 4]);
    }
    Ok(())
}

struct Actor {
    vertical: Cycle,
    lateral: [Cycle; 2],
}
impl Actor {
    fn prepare(v: Visual, fit: bool) -> Result<Self, String> {
        let image = with_fit(fit, || {
            with_source(1, || game_art::vertical::review_image(v))
        })?;
        let head = game_art::vertical::review_heads(v).ok_or("HELM01 missing anchors")?;
        let lateral = with_fit(fit, || grenade::lateral(v, 1))?;
        Ok(Self {
            vertical: Cycle::new(&image.image, image.cell, image.anchor, roots(&head)),
            lateral,
        })
    }
    fn draw(&self, rect: Rect, direction: &str, tick: usize) {
        if tick == 6 {
            self.vertical.draw(rect, 12, true, false);
            return;
        }
        match direction {
            "front" => self.vertical.draw(rect, tick, true, false),
            "back" => self.vertical.draw(rect, 6 + tick, true, false),
            "left" => self.lateral[0].draw(rect, tick, true, false),
            _ => self.lateral[1].draw(rect, tick, true, false),
        }
    }
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(540., 470.);
    for _ in 0..3 {
        next_frame().await;
    }
    let bodies = [3, 5, 6, 7];
    let names = ["Bastion", "Traqueuse", "Arcaniste", "Metamorphe"];
    let mut saved = 0;
    for torso in [Some(103), None] {
        let outfit = if torso.is_some() { "heavy" } else { "base" };
        let bare = bodies
            .map(|b| Actor::prepare(player(b, torso, 31, None), false))
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
        for head in [120, 115, 116, 117, 118, 119] {
            let current = bodies
                .map(|b| Actor::prepare(player(b, torso, 31, Some(head)), false))
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;
            let fitted = bodies
                .map(|b| Actor::prepare(player(b, torso, 31, Some(head)), true))
                .into_iter()
                .collect::<Result<Vec<_>, _>>()?;
            for direction in ["front", "back", "left", "right"] {
                for tick in 0..7 {
                    for warm in 0..2 {
                        set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
                        clear_background(Color::from_rgba(30, 40, 49, 255));
                        draw_text(
                            &format!(
                                "C06-HELM01 / casque {head} / {outfit} / {direction} / natif64"
                            ),
                            10.,
                            23.,
                            14.,
                            WHITE,
                        );
                        for (col, name) in names.iter().enumerate() {
                            draw_text(name, 22. + col as f32 * 128., 55., 14., WHITE);
                        }
                        for (row, (title, actors)) in [
                            ("Tete de la tenue / temoin", &bare),
                            ("Taille actuelle", &current),
                            ("04 / taille adaptee au porteur", &fitted),
                        ]
                        .into_iter()
                        .enumerate()
                        {
                            draw_text(title, 12., 83. + row as f32 * 120., 14., LIGHTGRAY);
                            for (col, actor) in actors.iter().enumerate() {
                                actor.draw(
                                    Rect::new(
                                        28. + col as f32 * 128.,
                                        96. + row as f32 * 120.,
                                        64.,
                                        64.,
                                    ),
                                    direction,
                                    tick,
                                );
                            }
                        }
                        draw_text(
                            if tick == 6 {
                                "Retour au repos frontal"
                            } else {
                                "Meme casque, memes poses, tailles propres aux personnages"
                            },
                            12.,
                            458.,
                            12.,
                            LIGHTGRAY,
                        );
                        if warm == 0 {
                            next_frame().await;
                        } else {
                            crate::ui_capture::framebuffer()?.export_png(
                                output
                                    .join(format!("helmet-H{head}-{outfit}-{direction}-{tick}.png"))
                                    .to_str()
                                    .unwrap(),
                            );
                            saved += 1;
                        }
                    }
                }
            }
            eprintln!("[HELM01] {outfit} / H{head}");
            if torso.is_some() && head == 120 {
                camp(output, &current, &fitted).await?;
            }
        }
    }
    std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
        "review":"ANIM-POLISH01-C06-HELM01-r01","normal_game_enabled":false,"status":"unapproved_visual_candidate",
        "native_tile_px":64,"screenshots":saved,"player_profiles":8,"helmet_models":6,"directions":4,"cycle_poses":6,"frontal_return":true,
        "class_factors":{"Bastion":1.15,"Traqueuse":0.86,"Arcaniste":0.96,"Metamorphe":1.06},
        "same_factor_all_armor_tiers_and_phases":true,"anchor":"lower shell edge on neck","existing_source_art_modified":false,
        "weapon_scale":"accepted C02-A / 01 in all rows","whole_figure_registration":"accepted C01 in all rows",
        "legacy_technomancer_and_monsters":"unchanged; not part of this pilot"
    })).unwrap()).map_err(|e|e.to_string())?;
    Ok(())
}
async fn camp(output: &Path, current: &[Actor], fitted: &[Actor]) -> Result<(), String> {
    let scene = crate::cryogenic_camp::scene();
    let load = |path: &str| -> Result<Texture2D, String> {
        let im = crate::asset_preparation::image(path)?;
        let t = crate::asset_loading::from_image(&im);
        t.set_filter(FilterMode::Nearest);
        Ok(t)
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
            "HELM01 / Bastion - Traqueuse / actuel en haut, 04 en bas",
            12.,
            25.,
            14.,
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
            for (row, actors) in [current, fitted].into_iter().enumerate() {
                for col in 0..2 {
                    actors[col].draw(
                        Rect::new(
                            world.x + (2. + col as f32 * 3.) * 64.,
                            world.y + (8. + row as f32 * 2.) * 64.,
                            64.,
                            64.,
                        ),
                        "front",
                        6,
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
                .export_png(output.join("helmet-camp.png").to_str().unwrap());
        }
    }
    next_frame().await;
    request_new_screen_size(540., 470.);
    for _ in 0..3 {
        next_frame().await;
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    fn world_frame(p: &crate::asset_preparation::PreparedImage, n: usize) -> Image {
        let mut out = Image::gen_image_color(256, 256, BLANK);
        let [w, h] = p.cell;
        let ox = (128. - p.anchor[0]).round() as i32;
        let oy = (160. - p.anchor[1]).round() as i32;
        for y in 0..usize::from(h) {
            for x in 0..usize::from(w) {
                let xx = x as i32 + ox;
                let yy = y as i32 + oy;
                if !(0..256).contains(&xx) || !(0..256).contains(&yy) {
                    continue;
                }
                let from = (((n / 6 * usize::from(h) + y) * usize::from(p.image.width))
                    + n % 6 * usize::from(w)
                    + x)
                    * 4;
                let to = (yy as usize * 256 + xx as usize) * 4;
                out.bytes[to..to + 4].copy_from_slice(&p.image.bytes[from..from + 4]);
            }
        }
        out
    }
    #[test]
    fn wearer_size_is_constant_across_armor_tiers() {
        for class in 0..5 {
            let values = (0..5)
                .map(|rank| fit_scale(class * 5 + rank))
                .collect::<Vec<_>>();
            assert!(values.iter().all(|v| *v == values[0]));
            assert!((0.8..=1.2).contains(&values[0]));
        }
        assert!(fit_scale(0) > fit_scale(10));
        assert!(!enabled());
        with_fit(true, || assert!(enabled()));
        assert!(!enabled());
    }
    #[test]
    fn vertical_fit_preserves_necks_armor_weapons_and_normal_source() {
        for body in [3, 5, 6, 7] {
            for torso in [None, Some(103)] {
                for head in [115, 120] {
                    let v = player(body, torso, 31, Some(head));
                    let heads = game_art::vertical::review_heads(v).unwrap();
                    let original = with_source(1, || game_art::vertical::review_image(v)).unwrap();
                    let fitted = with_fit(true, || {
                        with_source(1, || game_art::vertical::review_image(v))
                    })
                    .unwrap();
                    let mut changed = 0;
                    for (n, at) in heads.iter().enumerate() {
                        let a = world_frame(&original, n);
                        let b = world_frame(&fitted, n);
                        for (i, (aa, bb)) in a
                            .bytes
                            .chunks_exact(4)
                            .zip(b.bytes.chunks_exact(4))
                            .enumerate()
                        {
                            if aa != bb {
                                changed += 1;
                                let x = (i % 256) as f32;
                                let y = (i / 256) as f32;
                                assert!(
                                    (x - at[0]).abs() < 20. && y < at[1] + 8.,
                                    "Helmet escaped head ownership: body{body}, torso{torso:?}, head{head}, frame{n}, {x},{y}"
                                );
                            }
                        }
                    }
                    assert!(changed > 0, "Fit did not change helmet");
                    assert!(
                        original.image.bytes
                            == with_source(1, || game_art::vertical::review_image(v))
                                .unwrap()
                                .image
                                .bytes,
                        "Review changed normal source"
                    );
                }
            }
        }
    }
    #[test]
    fn lateral_fit_keeps_lower_anatomy_and_normal_compositions_unchanged() {
        for body in [3, 5, 6, 7] {
            for facing in [-1, 1] {
                let v = player(body, Some(103), 31, Some(120));
                let pose = Pose { facing, frame: 1 };
                let a = with_source(1, || game_art::polish_review_lateral(v, pose)).unwrap();
                let b = with_fit(true, || {
                    with_source(1, || game_art::polish_review_lateral(v, pose))
                })
                .unwrap();
                assert!(a.bytes != b.bytes);
                assert!(
                    a.bytes[60 * 512..] == b.bytes[60 * 512..],
                    "Helmet modified torso or feet"
                );
                assert!(
                    a.bytes
                        == with_source(1, || game_art::polish_review_lateral(v, pose))
                            .unwrap()
                            .bytes,
                    "Helmet review polluted normal caches"
                );
            }
        }
    }
}
