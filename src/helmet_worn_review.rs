//! HELM05: shared helmet drawings, fitted to wearer and armor; review only.
use super::*;
thread_local! { static MODEL: Cell<u8> = const { Cell::new(0) }; }
thread_local! { static COLLAR_MOTION: Cell<bool> = const { Cell::new(true) }; }
pub(crate) fn collar_motion() -> bool {
    COLLAR_MOTION.get()
}
#[cfg(test)]
fn with_collar_motion<T>(enabled: bool, f: impl FnOnce() -> T) -> T {
    struct Guard(bool);
    impl Drop for Guard {
        fn drop(&mut self) {
            COLLAR_MOTION.set(self.0);
        }
    }
    let _guard = Guard(COLLAR_MOTION.replace(enabled));
    f()
}

fn collar_target(outfit: usize, frame: usize, original: [f32; 2]) -> [f32; 2] {
    #[derive(serde::Deserialize)]
    struct Profile {
        outfit: usize,
        targets: Vec<[f32; 2]>,
    }
    #[derive(serde::Deserialize)]
    struct Anchors {
        profiles: Vec<Profile>,
    }
    static ANCHORS: std::sync::OnceLock<Anchors> = std::sync::OnceLock::new();
    if !collar_motion() {
        return original;
    }
    ANCHORS.get_or_init(|| serde_json::from_str(include_str!("../assets/source-art/animation-polish-01/helmet-worn-r01/HELM05-MOTION01-anchors.json")).unwrap())
        .profiles.iter().find(|p| p.outfit == outfit).and_then(|p| p.targets.get(frame)).copied().unwrap_or(original)
}

pub(crate) fn model() -> Option<usize> {
    MODEL
        .get()
        .checked_sub(1)
        .filter(|v| *v < 5)
        .map(usize::from)
}
fn with_model<T>(value: u8, f: impl FnOnce() -> T) -> T {
    struct Guard(u8);
    impl Drop for Guard {
        fn drop(&mut self) {
            MODEL.set(self.0);
        }
    }
    let _guard = Guard(MODEL.replace(value));
    f()
}
#[cfg(test)]
const FILES: [&[u8]; 5] = [
    include_bytes!("../assets/source-art/animation-polish-01/helmet-worn-r01/H05-W01-atlas.png"),
    include_bytes!("../assets/source-art/animation-polish-01/helmet-worn-r01/H05-W02-atlas.png"),
    include_bytes!("../assets/source-art/animation-polish-01/helmet-worn-r01/H05-W03-atlas.png"),
    include_bytes!("../assets/source-art/animation-polish-01/helmet-worn-r01/H05-W04-atlas.png"),
    include_bytes!("../assets/source-art/animation-polish-01/helmet-worn-r01/H05-W05-atlas.png"),
];
const HEAD_FILES: [&[u8]; 5] = [
    include_bytes!(
        "../assets/source-art/animation-polish-01/helmet-head-fit-r03/H03-W01-atlas.png"
    ),
    include_bytes!(
        "../assets/source-art/animation-polish-01/helmet-head-fit-r03/H03-W02-atlas.png"
    ),
    include_bytes!(
        "../assets/source-art/animation-polish-01/helmet-head-fit-r03/H03-W03-atlas.png"
    ),
    include_bytes!(
        "../assets/source-art/animation-polish-01/helmet-head-fit-r03/H03-W04-atlas.png"
    ),
    include_bytes!(
        "../assets/source-art/animation-polish-01/helmet-head-fit-r03/H03-W05-atlas.png"
    ),
];
fn head_unit(model: usize, outfit: usize, direction: usize) -> Image {
    thread_local! {
        static HEADS: [Image; 5] = HEAD_FILES.map(|bytes| Image::from_file_with_format(bytes, Some(ImageFormat::Png)).expect("MOTION03 native head sizes"));
    }
    let row = [0, 0, 1, 2, 3][outfit / 5] * 5 + outfit % 5;
    HEADS.with(|images| {
        images[model].sub_image(Rect::new(
            direction as f32 * 32.,
            row as f32 * 32.,
            32.,
            32.,
        ))
    })
}
#[cfg(test)]
fn shell(model: usize, direction: usize) -> Image {
    thread_local! {
        static SHELLS: [Image; 5] = FILES.map(|bytes| Image::from_file_with_format(bytes, Some(ImageFormat::Png)).expect("HELM05 source atlas"));
    }
    SHELLS.with(|images| images[model].sub_image(Rect::new(direction as f32 * 32., 0., 32., 32.)))
}
#[cfg(test)]
fn scale(outfit: usize) -> f32 {
    let class = [1.08, 1., 0.91, 0.97, 1.03]
        .get(outfit / 5)
        .copied()
        .unwrap_or(1.);
    class * [1., 1.035, 1.07, 1.105, 1.14][outfit % 5]
}

// The glyph already has its final wearer/armor dimensions. Copy exact pixels
// to the rounded neck joint; no affine filter, scaling or new RGB colors.
fn native_head(source: &Image, w: u16, h: u16, target: [f32; 2]) -> Image {
    let mut out = Image::gen_image_color(w, h, BLANK);
    let ox = target[0].round() as i32 - 16;
    let oy = target[1].round() as i32 - 24;
    for y in 0..i32::from(source.height) {
        for x in 0..i32::from(source.width) {
            let (xx, yy) = (x + ox, y + oy);
            if xx < 0 || yy < 0 || xx >= i32::from(w) || yy >= i32::from(h) {
                continue;
            }
            let from = (y as usize * usize::from(source.width) + x as usize) * 4;
            let to = (yy as usize * usize::from(w) + xx as usize) * 4;
            out.bytes[to..to + 4].copy_from_slice(&source.bytes[from..from + 4]);
        }
    }
    out
}
pub(crate) fn vertical(
    body: &mut Image,
    ownership_file: &str,
    at: [f32; 2],
    back: bool,
    model: usize,
    outfit: usize,
    frame: usize,
) -> Result<(), String> {
    let original = [at[0], at[1] + 6.];
    let collar = collar_target(outfit, frame, original);
    let target = [collar[0], collar[1] + 4.];
    let cover = native_head(
        &head_unit(model, outfit, usize::from(back)),
        256,
        256,
        target,
    );
    let ownership = crate::asset_preparation::image(ownership_file)?;
    let owned = helmets::place(&ownership, 256, 256, [16., 22.], original, [1., 0., 0., 1.]);
    // Every model replaces the painted head, including open helmets. There
    // is no animated old face, hair or class helmet underneath this module.
    for (i, p) in body.bytes.chunks_exact_mut(4).enumerate() {
        let x = (i % 256) as f32;
        let y = (i / 256) as f32;
        let neck_overlap =
            y >= original[1] && y < original[1] + 4. && (x - at[0]).abs() <= 8. - (y - original[1]);
        if (owned.bytes[i * 4 + 3] > 32 && y <= original[1]) || neck_overlap {
            p.fill(0);
        }
    }
    crate::game_art::vertical::review_over(body, &cover);
    Ok(())
}
pub(crate) fn lateral(
    body: &mut Image,
    at: [f32; 2],
    facing: i8,
    roll: f32,
    shoulder: f32,
    model: usize,
    outfit: usize,
    fitted: Option<([f32; 2], f32)>,
) {
    let _ = roll;
    // This point is the ARM14 torso joint, not the old cap's crown.
    let joint = fitted.map(|(joint, _)| joint).unwrap_or([at[0], shoulder]);
    let target = [joint[0] + f32::from(facing), joint[1] - 2.];
    let cover = native_head(
        &head_unit(model, outfit, if facing < 0 { 2 } else { 3 }),
        128,
        128,
        target,
    );
    let neck = shoulder - 2.;
    for y in 0..128usize {
        for x in 0..128usize {
            let half = if (y as f32) < neck - 3. { 12. } else { 5. };
            if (x as f32 - at[0]).abs() <= half && (y as f32) < neck {
                body.bytes[(y * 128 + x) * 4..(y * 128 + x) * 4 + 4].fill(0);
            }
        }
    }
    crate::game_art::vertical::review_over(body, &cover);
}
struct Actor {
    vertical: Cycle,
    lateral: [Cycle; 2],
}
impl Actor {
    fn prepare(v: Visual, model: u8) -> Result<Self, String> {
        with_model(model, || {
            with_source(1, || {
                let prepared = game_art::vertical::review_image(v)?;
                let heads =
                    game_art::vertical::review_heads(v).ok_or("HELM05 missing head anchors")?;
                Ok(Self {
                    vertical: Cycle::new(
                        &prepared.image,
                        prepared.cell,
                        prepared.anchor,
                        roots(&heads),
                    ),
                    lateral: grenade::lateral(v, 1)?,
                })
            })
        })
    }
    fn draw(&self, rect: Rect, direction: &str, frame: usize) {
        if frame == 6 {
            self.vertical.draw(rect, 12, true, false);
            return;
        }
        match direction {
            "front" => self.vertical.draw(rect, frame, true, false),
            "back" => self.vertical.draw(rect, 6 + frame, true, false),
            "left" => self.lateral[0].draw(rect, frame, true, false),
            _ => self.lateral[1].draw(rect, frame, true, false),
        }
    }
}
pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(600., 730.);
    for _ in 0..3 {
        next_frame().await;
    }
    let mut saved = 0;
    for (tier, torso) in [None, Some(110), Some(102), Some(101), Some(103)]
        .into_iter()
        .enumerate()
    {
        let bare = [3, 5, 6, 7]
            .map(|b| Actor::prepare(player(b, torso, 31, None), 0))
            .into_iter()
            .collect::<Result<Vec<_>, _>>()?;
        let mut worn = Vec::new();
        for model in 1..=5 {
            worn.push(
                [3, 5, 6, 7]
                    .map(|b| Actor::prepare(player(b, torso, 31, Some(120)), model))
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()?,
            );
        }
        for direction in ["front", "back", "left", "right"] {
            for frame in 0..7 {
                for warm in 0..2 {
                    set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
                    clear_background(Color::from_rgba(30, 40, 49, 255));
                    draw_text(
                        &format!("HELM05 / armure {} / {direction} / natif64", tier + 1),
                        12.,
                        24.,
                        15.,
                        WHITE,
                    );
                    for (col, name) in ["Bastion", "Traqueuse", "Arcaniste", "Metamorphe"]
                        .into_iter()
                        .enumerate()
                    {
                        draw_text(name, 24. + col as f32 * 128., 62., 14., WHITE);
                    }
                    for row in 0..6 {
                        let title = [
                            "Tete de la tenue / temoin",
                            "01 / tres leger",
                            "02 / leger",
                            "03 / intermediaire",
                            "04 / lourd",
                            "05 / tres lourd",
                        ][row];
                        draw_text(title, 12., 95. + row as f32 * 100., 13., LIGHTGRAY);
                        let actors = if row == 0 { &bare } else { &worn[row - 1] };
                        for (col, actor) in actors.iter().enumerate() {
                            actor.draw(
                                Rect::new(
                                    30. + col as f32 * 128.,
                                    108. + row as f32 * 100.,
                                    64.,
                                    64.,
                                ),
                                direction,
                                frame,
                            );
                        }
                    }
                    draw_text(
                        if frame == 6 {
                            "Retour au repos frontal"
                        } else {
                            "Meme casque sur tous les porteurs; volume ajuste a la carrure et a l'armure"
                        },
                        12.,
                        714.,
                        12.,
                        LIGHTGRAY,
                    );
                    if warm == 0 {
                        next_frame().await;
                    } else {
                        crate::ui_capture::framebuffer()?.export_png(
                            output
                                .join(format!("worn-tier{tier}-{direction}-{frame}.png"))
                                .to_str()
                                .unwrap(),
                        );
                        saved += 1;
                    }
                }
            }
        }
        if tier == 4 {
            camp(output, &worn).await?;
        }
        eprintln!("[HELM05] armor tier {}", tier + 1);
    }
    std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
        "review":"HELM05-WORN-r01","native_tile_px":64,"models":5,"classes":4,"armor_stages":5,
        "outfit_helmet_combinations":100,"directions":4,"moving_phases":6,"frontal_return":true,
        "screenshots":saved,"normal_game_enabled":false,"icon_acceptance":"HELM04-ACCEPTANCE.json",
        "fit_class_factors":[1.08,1.,0.91,0.97,1.03],"fit_armor_factors":[1.,1.035,1.07,1.105,1.14],
        "common_artwork_per_model":true,"pulse_in_this_worn_review":false
    })).unwrap()).map_err(|e|e.to_string())?;
    Ok(())
}
async fn camp(output: &Path, models: &[Vec<Actor>]) -> Result<(), String> {
    let scene = crate::cryogenic_camp::scene();
    let load = |file: &str| -> Result<Texture2D, String> {
        let image = crate::asset_preparation::image(file)?;
        let texture = crate::asset_loading::from_image(&image);
        texture.set_filter(FilterMode::Nearest);
        Ok(texture)
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
            "HELM05 / casques 01-05 / Bastion, Traqueuse, Arcaniste, Metamorphe",
            12.,
            24.,
            13.,
            WHITE,
        );
        {
            let _world = crate::world_readability::world(false);
            atmosphere.begin(world);
            atmosphere.background_time(0.);
            atmosphere.begin_background(true);
            for texture in [&floor, &structure] {
                draw_texture_ex(
                    texture,
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
            for row in 0..4 {
                for (col, model) in models.iter().enumerate() {
                    model[row].draw(
                        Rect::new(
                            world.x + (0.9 + col as f32 * 1.6) * 64.,
                            world.y + (6.5 + row as f32 * 1.6) * 64.,
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
                .export_png(output.join("worn-camp.png").to_str().unwrap());
        }
    }
    next_frame().await;
    request_new_screen_size(600., 730.);
    for _ in 0..3 {
        next_frame().await;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn final_sized_head_pixels_are_copied_without_interpolation_or_palette_changes() {
        for outfit in [0, 4, 10, 14, 15, 19, 20, 24] {
            for model in 0..5 {
                for direction in 0..4 {
                    let unit = head_unit(model, outfit, direction);
                    assert!(unit.bytes.chunks_exact(4).all(|p| p[3] == 0 || p[3] == 255));
                    let placed = native_head(&unit, 128, 128, [64.4, 50.3]);
                    for y in 0..32usize {
                        for x in 0..32usize {
                            let a = (y * 32 + x) * 4;
                            let b = ((y + 26) * 128 + x + 48) * 4;
                            assert_eq!(
                                &unit.bytes[a..a + 4],
                                &placed.bytes[b..b + 4],
                                "Filtered head: outfit{outfit} model{model} direction{direction}"
                            );
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn complete_heads_replace_baked_caps_and_do_not_depend_on_the_old_head_colors() {
        for outfit in [0, 10, 15, 20] {
            for model in 0..5 {
                for facing in [-1, 1] {
                    let fixture = |rgba: [u8; 4]| {
                        let mut image = Image::gen_image_color(128, 128, BLANK);
                        for y in 30..53usize {
                            let range = if y < 50 { 54..75 } else { 61..68 };
                            for x in range {
                                image.bytes[(y * 128 + x) * 4..(y * 128 + x) * 4 + 4]
                                    .copy_from_slice(&rgba);
                            }
                        }
                        image
                    };
                    let mut a = fixture([255, 0, 255, 255]);
                    let mut b = fixture([0, 255, 0, 255]);
                    for image in [&mut a, &mut b] {
                        lateral(
                            image,
                            [64., 44.],
                            facing,
                            0.,
                            55.,
                            model,
                            outfit,
                            Some(([64., 55.], 0.)),
                        );
                    }
                    assert_eq!(
                        a.bytes, b.bytes,
                        "Old painted cap leaked: outfit{outfit}, model{model}, facing{facing}"
                    );
                    assert!(a.bytes.chunks_exact(4).any(|p| p[3] > 96));
                }
            }
        }
    }
    fn eye_pixels(image: &Image, x: std::ops::Range<usize>, y: std::ops::Range<usize>) -> usize {
        let mut count = 0;
        for yy in y {
            for xx in x.clone() {
                let offset = (yy * usize::from(image.width) + xx) * 4;
                let p = &image.bytes[offset..offset + 4];
                if p[3] >= 96
                    && p[1] >= 60
                    && i16::from(p[1]) > i16::from(p[0]) + 15
                    && i16::from(p[2]) > i16::from(p[0]) + 15
                {
                    count += 1;
                }
            }
        }
        count
    }
    #[test]
    fn orbital_optic_remains_visible_in_every_front_angle_after_native_placement() {
        let before=Image::from_file_with_format(include_bytes!("../assets/source-art/animation-polish-01/helmet-worn-r01/H05-W02-before-eye01-atlas.png"),Some(ImageFormat::Png)).unwrap();
        let old_far = before.sub_image(Rect::new(64., 0., 32., 32.));
        assert_eq!(
            eye_pixels(&old_far, 0..32, 17..24),
            0,
            "Regression fixture must reproduce the missing eye"
        );
        for direction in [0, 2, 3] {
            let source = shell(1, direction);
            assert!(
                eye_pixels(&source, 0..32, 17..24) > 0,
                "Missing orbital glass, view {direction}"
            );
            for class in [0, 10, 15, 20] {
                for tier in 0..5 {
                    for roll in [-0.18_f32, 0., 0.18] {
                        let s = scale(class + tier);
                        let (sin, cos) = roll.sin_cos();
                        let placed = helmets::place(
                            &source,
                            128,
                            128,
                            [16., 24.],
                            [64., 50.],
                            [s * cos, -s * sin, s * sin, s * cos],
                        );
                        assert!(
                            eye_pixels(&placed, 48..80, 41..50) > 0,
                            "Glass lost at native scale: view{direction}, outfit{}, roll{roll}",
                            class + tier
                        );
                    }
                }
            }
        }
        assert_eq!(
            eye_pixels(&shell(1, 1), 0..32, 17..24),
            0,
            "No invented rear-facing orbital lens"
        );
    }
    fn frame_at_world(p: &crate::asset_preparation::PreparedImage, n: usize) -> Image {
        let mut out = Image::gen_image_color(256, 256, BLANK);
        let [w, h] = p.cell;
        let ox = (128. - p.anchor[0]).round() as i32;
        let oy = (160. - p.anchor[1]).round() as i32;
        for y in 0..usize::from(h) {
            for x in 0..usize::from(w) {
                let (xx, yy) = (x as i32 + ox, y as i32 + oy);
                if !(0..256).contains(&xx) || !(0..256).contains(&yy) {
                    continue;
                }
                let from = ((n / 6 * usize::from(h) + y) * usize::from(p.image.width)
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
    fn helmet_and_painted_head_follow_the_chest_instead_of_the_old_crown_bob() {
        use std::collections::BTreeSet;
        fn glass(image: &Image) -> BTreeSet<(i32, i32)> {
            let mut result = BTreeSet::new();
            for y in 96..116usize {
                for x in 112..144usize {
                    if eye_pixels(image, x..x + 1, y..y + 1) > 0 {
                        result.insert((x as i32, y as i32));
                    }
                }
            }
            assert!(!result.is_empty());
            result
        }
        // Traqueuse / heavy armor, front phase 1 -> phase 4: the chest moves
        // one pixel right, but the old crown marker moves one pixel down.
        // Exercise the actual composition, rather than just checking metadata.
        let visual = player(5, Some(101), 31, Some(120));
        let prepare = |stable| {
            with_model(2, || {
                with_collar_motion(stable, || {
                    with_source(1, || game_art::vertical::review_image(visual)).unwrap()
                })
            })
        };
        let old = prepare(false);
        let new = prepare(true);
        let shift = |points: BTreeSet<(i32, i32)>, dx, dy| {
            points
                .into_iter()
                .map(|(x, y)| (x + dx, y + dy))
                .collect::<BTreeSet<_>>()
        };
        assert_eq!(
            glass(&frame_at_world(&old, 3)),
            shift(glass(&frame_at_world(&old, 0)), 0, 1)
        );
        assert_eq!(
            glass(&frame_at_world(&new, 3)),
            shift(glass(&frame_at_world(&new, 0)), 1, 0)
        );
        for n in 0..13 {
            let before = frame_at_world(&old, n);
            let after = frame_at_world(&new, n);
            assert_eq!(
                &before.bytes[124 * 256 * 4..],
                &after.bytes[124 * 256 * 4..],
                "Torso/weapon/legs changed in phase {n}"
            );
        }
    }
    #[test]
    fn worn_selection_is_scoped_and_does_not_change_normal_source() {
        assert_eq!(model(), None);
        with_model(2, || {
            assert_eq!(model(), Some(1));
            with_model(5, || assert_eq!(model(), Some(4)));
            assert_eq!(model(), Some(1));
        });
        assert_eq!(model(), None);
        for body in [3, 5, 6, 7] {
            let visual = player(body, Some(103), 31, Some(120));
            let heads = game_art::vertical::review_heads(visual).unwrap();
            let old = with_source(1, || game_art::vertical::review_image(visual)).unwrap();
            let new = with_model(5, || {
                with_source(1, || game_art::vertical::review_image(visual))
            })
            .unwrap();
            let mut changed = 0;
            for (n, at) in heads.iter().enumerate() {
                let a = frame_at_world(&old, n);
                let b = frame_at_world(&new, n);
                // MOTION01 corrects the old crown-derived neck by at most
                // two pixels. The permitted region ends at the actual fitted
                // collar; the rest of the body remains outside ownership.
                let collar = collar_target(
                    usize::from(match body {
                        3 => 4u8,
                        5 => 14,
                        6 => 19,
                        _ => 24,
                    }),
                    n,
                    [at[0], at[1] + 6.],
                );
                let bottom = (at[1] + 10.).max(collar[1] + 4.) + 1.;
                for (i, (aa, bb)) in a
                    .bytes
                    .chunks_exact(4)
                    .zip(b.bytes.chunks_exact(4))
                    .enumerate()
                {
                    if aa != bb {
                        changed += 1;
                        assert!(
                            (i % 256) as f32 > at[0] - 24.
                                && ((i % 256) as f32) < at[0] + 24.
                                && ((i / 256) as f32) < bottom,
                            "HELM05 escaped head region: body{body} frame{n} pixel{i}"
                        );
                    }
                }
            }
            assert!(changed > 0);
            assert!(
                old.image.bytes
                    == with_source(1, || game_art::vertical::review_image(visual))
                        .unwrap()
                        .image
                        .bytes
            );
        }
    }
    #[test]
    fn lateral_worn_shell_preserves_lower_anatomy_weapons_and_normal_cache() {
        for body in [3, 5, 6, 7] {
            for facing in [-1, 1] {
                let visual = player(body, Some(103), 31, Some(120));
                let pose = Pose { facing, frame: 2 };
                let old = with_source(1, || game_art::polish_review_lateral(visual, pose)).unwrap();
                let new = with_model(3, || {
                    with_source(1, || game_art::polish_review_lateral(visual, pose))
                })
                .unwrap();
                assert!(old.bytes != new.bytes);
                assert!(old.bytes[64 * 512..] == new.bytes[64 * 512..]);
                assert!(
                    old.bytes
                        == with_source(1, || game_art::polish_review_lateral(visual, pose))
                            .unwrap()
                            .bytes
                );
            }
        }
    }
}
