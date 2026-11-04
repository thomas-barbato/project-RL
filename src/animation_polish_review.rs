//! ANIM-POLISH01: isolated comparisons of current rendering and proposed fixes.
use crate::game_art::{self, Visual, walking::Pose};
use macroquad::prelude::*;
use project_rl::entity::EquipmentPalette;
use std::cell::Cell;
use std::path::Path;
#[path = "grenade_launcher_review.rs"]
mod grenade;
pub(crate) use grenade::capture as capture_grenade;
thread_local! { static SOURCE_STYLE: Cell<Option<u8>> = const { Cell::new(None) }; }
pub(crate) fn source_style() -> u8 {
    SOURCE_STYLE.get().unwrap_or(0)
}
pub(crate) fn source_override() -> Option<u8> {
    SOURCE_STYLE.get()
}
pub(crate) fn with_source<T>(style: u8, f: impl FnOnce() -> T) -> T {
    struct Guard(Option<u8>);
    impl Drop for Guard {
        fn drop(&mut self) {
            SOURCE_STYLE.set(self.0);
        }
    }
    let _guard = Guard(SOURCE_STYLE.replace(Some(style)));
    f()
}
pub(crate) fn heavy(model: u16) -> bool {
    matches!(model, 13 | 15..=34 | 47..=53)
}
pub(crate) fn rifle(model: u16) -> bool {
    crate::rifle_visuals::rifle(model)
}
pub(crate) fn revising(model: u16) -> bool {
    match source_style() {
        4 => model <= 60,
        5 => rifle(model),
        _ => heavy(model),
    }
}
pub(crate) fn local_separation(model: u16) -> bool {
    matches!(source_style(), 4 | 5) && revising(model)
}

/// Inverse mapping outside the axis-aligned box of the two authored grips.
/// The grip box stays exact; horizontal barrels and vertical ribs stay straight.
/// This deliberately does not pretend to be a uniform percentage reduction.
pub(crate) fn compact_weapon(
    source: &Image,
    model: u16,
    a: [f32; 2],
    b: [f32; 2],
    style: u8,
) -> Image {
    if rifle(model) && matches!(style, 4 | 5) {
        return crate::rifle_visuals::prepare_profile(source, a, b, style);
    }
    if style == 0 || (style == 5 && !rifle(model)) || (style < 4 && !heavy(model)) || model > 60 {
        return source.clone();
    }
    let (extension, thickness) = if style == 5 {
        (0.95, 1.08)
    } else if style == 4 {
        if heavy(model) {
            (0.60, 0.70)
        } else {
            (0.85, 1.0)
        }
    } else if style == 1 {
        (0.80, 0.85)
    } else {
        (0.65, 0.75)
    };
    let mut out = Image::gen_image_color(source.width, source.height, BLANK);
    let inverse = |value: f32, low: f32, high: f32, factor: f32| {
        if value < low {
            low + (value - low) / factor
        } else if value > high {
            high + (value - high) / factor
        } else {
            value
        }
    };
    for y in 0..usize::from(source.height) {
        for x in 0..usize::from(source.width) {
            let sx = inverse(x as f32 + 0.5, a[0].min(b[0]), a[0].max(b[0]), extension) - 0.5;
            let sy = inverse(y as f32 + 0.5, a[1].min(b[1]), a[1].max(b[1]), thickness) - 0.5;
            if matches!(style, 4 | 5) {
                // Preserve hard source pixels; the normal compositor performs
                // the sole orientation/scale sampling after this preparation.
                let (xx, yy) = (sx.round() as i32, sy.round() as i32);
                if (0..i32::from(source.width)).contains(&xx)
                    && (0..i32::from(source.height)).contains(&yy)
                {
                    let from = (yy as usize * usize::from(source.width) + xx as usize) * 4;
                    let to = (y * usize::from(source.width) + x) * 4;
                    out.bytes[to..to + 4].copy_from_slice(&source.bytes[from..from + 4]);
                }
                continue;
            }
            let mut rgba = [0.; 4];
            let ix = sx.floor() as i32;
            let iy = sy.floor() as i32;
            for yy in iy..=iy + 1 {
                for xx in ix..=ix + 1 {
                    if xx < 0
                        || yy < 0
                        || xx >= i32::from(source.width)
                        || yy >= i32::from(source.height)
                    {
                        continue;
                    }
                    let w = (1. - (sx - xx as f32).abs()) * (1. - (sy - yy as f32).abs());
                    let i = (yy as usize * usize::from(source.width) + xx as usize) * 4;
                    let alpha = f32::from(source.bytes[i + 3]) * w;
                    rgba[3] += alpha;
                    for (c, value) in rgba.iter_mut().enumerate().take(3) {
                        *value += f32::from(source.bytes[i + c]) * alpha;
                    }
                }
            }
            let i = (y * usize::from(source.width) + x) * 4;
            if rgba[3] >= 0.5 {
                for c in 0..3 {
                    out.bytes[i + c] = (rgba[c] / rgba[3]).round().clamp(0., 255.) as u8;
                }
                out.bytes[i + 3] = rgba[3].round().clamp(0., 255.) as u8;
            }
        }
    }
    if matches!(style, 4 | 5) {
        crate::firearm_polish_review::clarify_metal(&mut out);
    }
    out
}
pub(crate) fn vertical_weapon(source: &Image, model: u16) -> Image {
    let style = source_style();
    if style == 0 || !revising(model) {
        return source.clone();
    }
    let (_, a, b) = with_source(0, || {
        game_art::vertical_review_weapon(model, false, EquipmentPalette::Original)
    })
    .unwrap();
    compact_weapon(source, model, a, b.unwrap(), style)
}

struct Cycle {
    texture: Texture2D,
    cell: [u16; 2],
    anchor: [f32; 2],
    root: Vec<[f32; 2]>,
}
impl Cycle {
    fn new(image: &Image, cell: [u16; 2], anchor: [f32; 2], root: Vec<[f32; 2]>) -> Self {
        let texture = crate::asset_loading::from_image(image);
        texture.set_filter(FilterMode::Nearest);
        Self {
            texture,
            cell,
            anchor,
            root,
        }
    }
    fn draw(&self, rect: Rect, n: usize, stable: bool, portrait: bool) {
        let [w, h] = self.cell;
        let source = Rect::new(
            (n % 6) as f32 * f32::from(w),
            (n / 6) as f32 * f32::from(h),
            f32::from(w),
            f32::from(h),
        );
        let offset = if stable && !portrait {
            self.root[n]
        } else {
            [0., 0.]
        };
        let scale = if portrait {
            (rect.w / f32::from(w)).min(rect.h / f32::from(h))
        } else {
            rect.w / 64.
        };
        let at = if portrait {
            vec2(
                rect.center().x - f32::from(w) * scale / 2.,
                rect.center().y - f32::from(h) * scale / 2.,
            )
        } else {
            vec2(
                rect.center().x - (self.anchor[0] - offset[0]) * scale,
                rect.bottom() - (self.anchor[1] - offset[1]) * scale,
            )
        };
        let _actor = crate::world_readability::actor(true);
        if !portrait {
            game_art::polish_review_shadow(rect);
        }
        crate::world_readability::texture(
            &self.texture,
            at.x,
            at.y,
            WHITE,
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(vec2(f32::from(w) * scale, f32::from(h) * scale)),
                ..Default::default()
            },
        );
    }
}
fn roots(heads: &[[f32; 2]]) -> Vec<[f32; 2]> {
    heads
        .iter()
        .enumerate()
        .map(|(n, h)| {
            if n == 12 {
                return [0., 0.];
            }
            let group = if n < 6 { &heads[..6] } else { &heads[6..12] };
            let mut xs: Vec<_> = group.iter().map(|h| h[0]).collect();
            let mut ys: Vec<_> = group.iter().map(|h| h[1]).collect();
            xs.sort_by(f32::total_cmp);
            ys.sort_by(f32::total_cmp);
            [
                ((xs[2] + xs[3]) / 2. - h[0]).round().clamp(-2., 2.),
                ((ys[2] + ys[3]) / 2. - h[1]).round().clamp(-1., 1.),
            ]
        })
        .collect()
}
fn crown(image: &Image) -> [f32; 2] {
    let top = image
        .bytes
        .chunks_exact(4)
        .position(|p| p[3] >= 96)
        .unwrap()
        / usize::from(image.width);
    let (mut sum, mut weight) = (0., 0.);
    for y in top..top + 6 {
        for x in 0..usize::from(image.width) {
            let a = f32::from(image.bytes[(y * usize::from(image.width) + x) * 4 + 3]);
            if a >= 96. {
                sum += x as f32 * a;
                weight += a;
            }
        }
    }
    [sum / weight, top as f32]
}
struct Subject {
    vertical: Vec<Cycle>,
    lateral: Vec<[Cycle; 2]>,
    report: serde_json::Value,
}
impl Subject {
    fn prepare(visual: Visual) -> Result<Self, String> {
        let heads =
            game_art::vertical::review_heads(visual).ok_or("Review missing head anchors")?;
        let vertical_roots = roots(&heads);
        let mut vertical = Vec::new();
        let mut lateral = Vec::new();
        let mut lateral_heads = Vec::new();
        for style in 0..3 {
            let p = with_source(style, || game_art::vertical::review_image(visual))?;
            vertical.push(Cycle::new(
                &p.image,
                p.cell,
                p.anchor,
                vertical_roots.clone(),
            ));
            let mut sides = Vec::new();
            for facing in [-1, 1] {
                let mut sheet = Image::gen_image_color(768, 128, BLANK);
                let mut landmarks = Vec::new();
                for frame in 0..6 {
                    let pose = Pose { facing, frame };
                    let im = with_source(style, || game_art::polish_review_lateral(visual, pose))
                        .ok_or("Review missing lateral composition")?;
                    for y in 0..128usize {
                        let at = (y * 768 + usize::from(frame) * 128) * 4;
                        sheet.bytes[at..at + 512]
                            .copy_from_slice(&im.bytes[y * 512..y * 512 + 512]);
                    }
                    // The bare figure supplies anatomy; an elevated weapon or
                    // an equipment fringe must never be treated as the crown.
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
                        pose,
                    )
                    .unwrap();
                    landmarks.push(crown(&bare));
                }
                let mut same = landmarks.clone();
                same.extend(&landmarks);
                same.push(landmarks[0]);
                let root = roots(&same)[..6].to_vec();
                if style == 0 {
                    lateral_heads.push(serde_json::json!({"facing":facing,"heads":landmarks,"whole_figure_offsets":root}));
                }
                sides.push(Cycle::new(&sheet, [128, 128], [64., 96.], root));
            }
            lateral.push(sides.try_into().ok().unwrap());
        }
        Ok(Self {
            vertical,
            lateral,
            report: serde_json::json!({"vertical_heads":heads,"vertical_whole_figure_offsets":vertical_roots,"lateral":lateral_heads}),
        })
    }
    fn draw(&self, rect: Rect, pose: Option<Pose>, style: usize) {
        match pose {
            None => self.vertical[style].draw(rect, 12, style > 0, false),
            Some(Pose { facing, frame }) if facing.abs() == 2 => self.vertical[style].draw(
                rect,
                usize::from(frame) + if facing < 0 { 6 } else { 0 },
                style > 0,
                false,
            ),
            Some(Pose { facing, frame }) => self.lateral[style][usize::from(facing > 0)].draw(
                rect,
                usize::from(frame),
                style > 0,
                false,
            ),
        }
    }
}

async fn ground_scene(output: &Path, subject: &Subject) -> Result<(), String> {
    let camp = crate::cryogenic_camp::scene();
    let upload = |path: &str| -> Result<Texture2D, String> {
        let image = crate::asset_preparation::image(path)?;
        let texture = crate::asset_loading::from_image(&image);
        texture.set_filter(FilterMode::Nearest);
        Ok(texture)
    };
    let floor = upload(&camp.floor)?;
    let structure = upload(&camp.structure)?;
    let atmosphere = crate::camp_atmosphere::Atmosphere::new(
        &camp.atmosphere,
        camp.width as u32 * 64,
        camp.height as u32 * 64,
    );
    request_new_screen_size(640., 610.);
    for _ in 0..3 {
        next_frame().await;
    }
    let world = Rect::new(
        16.,
        -260.,
        camp.width as f32 * 64.,
        camp.height as f32 * 64.,
    );
    let clip = Rect::new(16., 60., 576., 512.);
    for warm in 0..2 {
        set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
        clear_background(Color::from_rgba(30, 40, 49, 255));
        draw_text(
            "C03 / extrait du sol accepte / objets et acteurs natifs64",
            14.,
            25.,
            16.,
            WHITE,
        );
        draw_text("Actuel : icones 64 px", 142., 51., 15., WHITE);
        draw_text("Proposition : 40 px", 366., 51., 15., WHITE);
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
            for style in 0..2 {
                let x = world.x + if style == 0 { 2. } else { 5. } * 64.;
                subject.draw(
                    Rect::new(x + 32., world.y + 7. * 64., 64., 64.),
                    None,
                    style,
                );
                for (row, id) in [
                    "core:mitrailleuse_lourde",
                    "core:lance_roquettes",
                    "core:tenue_assaut",
                ]
                .into_iter()
                .enumerate()
                {
                    let side = if style == 0 { 64. } else { 40. };
                    game_art::draw_item(
                        Rect::new(
                            x + 32. + (64. - side) / 2.,
                            world.y + (8. + row as f32) * 64. + (64. - side) / 2.,
                            side,
                            side,
                        ),
                        id,
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
                .export_png(output.join("ground-native.png").to_str().unwrap());
        }
    }
    next_frame().await;
    Ok(())
}

/// GPU comparison of the normal world path against accepted C03 and UI icons.
pub async fn capture_ground_icons(output: &Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(640., 420.);
    for _ in 0..3 {
        next_frame().await;
    }
    let ids = [
        "core:mitrailleuse_lourde",
        "core:lance_roquettes",
        "core:tenue_assaut",
        "core:power_regulator",
    ];
    let mut checks = Vec::new();
    for (palette_index, palette) in EquipmentPalette::ALL.into_iter().enumerate() {
        let mut actual = None;
        // Compare successive passes at the same coordinates: nearest sampling
        // at half-texel boundaries can differ between distant screen columns.
        for warm in 0..3 {
            set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
            clear_background(Color::from_rgba(30, 40, 49, 255));
            for (col, label) in ["Avant / 64", "Jeu / 40", "Reference / 40", "Interface / 64"]
                .into_iter()
                .enumerate()
            {
                draw_text(label, 16. + col as f32 * 144., 30., 17., WHITE);
            }
            for (row, id) in ids.into_iter().enumerate() {
                let art = game_art::item(id).ok_or_else(|| format!("C03 missing icon: {id}"))?;
                for col in 0..4 {
                    let cell = Rect::new(16. + col as f32 * 144., 72. + row as f32 * 80., 64., 64.);
                    match col {
                        1 if warm < 2 => {
                            game_art::draw_world(cell, Visual::ColoredItem(art, palette), 1.)
                        }
                        1 | 2 => {
                            game_art::draw_item_variant(
                                Rect::new(cell.x + 12., cell.y + 12., 40., 40.),
                                id,
                                palette,
                            );
                        }
                        3 if warm < 2 => {
                            game_art::draw_portrait(cell, Visual::ColoredItem(art, palette))
                        }
                        _ => {
                            game_art::draw_item_variant(cell, id, palette);
                        }
                    }
                }
            }
            if warm == 0 {
                next_frame().await;
                continue;
            }
            let image = crate::ui_capture::framebuffer()?;
            if warm == 1 {
                image.export_png(
                    output
                        .join(format!("ground-icons-{palette_index}.png"))
                        .to_str()
                        .unwrap(),
                );
                actual = Some(image);
                next_frame().await;
                continue;
            }
            let actual = actual.as_ref().unwrap();
            let scale = f32::from(image.width) / screen_width();
            let tile = |image: &Image, col: usize, row: usize| {
                let left = ((16. + col as f32 * 144.) * scale).round() as usize;
                let top = ((72. + row as f32 * 80.) * scale).round() as usize;
                let side = (64. * scale).round() as usize;
                let mut pixels = Vec::new();
                for y in top..top + side {
                    let offset =
                        ((usize::from(image.height) - 1 - y) * usize::from(image.width) + left) * 4;
                    pixels.extend_from_slice(&image.bytes[offset..offset + side * 4]);
                }
                pixels
            };
            for (row, id) in ids.into_iter().enumerate() {
                if tile(actual, 1, row) != tile(&image, 1, row) {
                    return Err(format!(
                        "C03 normal world differs from accepted 40 px: {id} {palette:?}"
                    ));
                }
                if tile(actual, 3, row) != tile(&image, 3, row) {
                    return Err(format!("C03 interface changed: {id} {palette:?}"));
                }
                if tile(actual, 1, row) == tile(actual, 0, row) {
                    return Err(format!("C03 ground icon was not reduced: {id} {palette:?}"));
                }
                checks.push(serde_json::json!({"item":id,"palette":format!("{palette:?}"),"world_matches_accepted_40":true,"interface_matches_original_64":true}));
            }
        }
    }
    std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":"ANIM-POLISH01-C03","normal_game_enabled":true,"native_cell_px":64,"ground_icon_px":40,"unchanged_interface_icon_px":64,"checks":checks})).unwrap()).map_err(|e|e.to_string())?;
    Ok(())
}

pub(crate) fn player(body: u8, torso: Option<u16>, weapon: u16, head: Option<u16>) -> Visual {
    Visual::Player {
        body,
        equipment: [torso, head, None, None],
        weapon: Some(weapon),
        hands: None,
        palettes: [EquipmentPalette::Original; 3],
    }
}

pub async fn capture(output: &Path) -> Result<(), String> {
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(1100., 760.);
    for _ in 0..3 {
        next_frame().await;
    }
    let mut reports = Vec::new();
    for (body, name) in [
        (3, "Bastion"),
        (5, "Traqueuse"),
        (6, "Arcaniste"),
        (7, "Metamorphe"),
    ] {
        for (weapon, label) in [
            (2, "fusil temoin"),
            (13, "lance-flammes"),
            (23, "lance-grenades"),
            (31, "mitrailleuse"),
            (51, "roquettes"),
        ] {
            for head in [None] {
                let visual = player(body, Some(103), weapon, head);
                let subject = Subject::prepare(visual)?;
                reports.push(serde_json::json!({"body":body,"weapon":weapon,"helmet":head,"registration":subject.report}));
                for tick in 0..7 {
                    for warm in 0..2 {
                        set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
                        clear_background(Color::from_rgba(30, 40, 49, 255));
                        draw_text(
                            &format!("ANIM-POLISH01 / {name} / {label} / casque {head:?}"),
                            16.,
                            26.,
                            20.,
                            WHITE,
                        );
                        for (col, title) in
                            ["Actuel", "01 / compact modere", "02 / compact accentue"]
                                .into_iter()
                                .enumerate()
                        {
                            draw_text(title, 145. + col as f32 * 310., 64., 18., WHITE);
                        }
                        for (row, (facing, title)) in
                            [(2, "Face"), (-2, "Dos"), (-1, "Gauche"), (1, "Droite")]
                                .into_iter()
                                .enumerate()
                        {
                            draw_text(title, 16., 120. + row as f32 * 126., 18., WHITE);
                            for style in 0..3 {
                                let rect = Rect::new(
                                    170. + style as f32 * 310.,
                                    104. + row as f32 * 126.,
                                    64.,
                                    64.,
                                );
                                let pose = (tick < 6).then_some(Pose {
                                    facing,
                                    frame: tick,
                                });
                                subject.draw(rect, pose, style);
                                // Integer zoom accompanies, never replaces, native64.
                                subject.draw(
                                    Rect::new(rect.x + 100., rect.y - 26., 128., 128.),
                                    pose,
                                    style,
                                );
                            }
                        }
                        draw_text("Portrait Visee", 16., 637., 18., WHITE);
                        game_art::draw_portrait(Rect::new(170., 596., 96., 96.), visual);
                        for style in 1..3 {
                            subject.vertical[style].draw(
                                Rect::new(170. + style as f32 * 310., 596., 96., 96.),
                                12,
                                false,
                                true,
                            );
                        }
                        draw_text("Objets au sol", 16., 724., 18., WHITE);
                        let ids = [
                            "core:mitrailleuse_lourde",
                            "core:lance_roquettes",
                            "core:tenue_assaut",
                        ];
                        for style in 0..3 {
                            for (i, id) in ids.iter().enumerate() {
                                let side = if style == 0 { 64. } else { 40. };
                                game_art::draw_item(
                                    Rect::new(
                                        158. + style as f32 * 310.
                                            + i as f32 * 78.
                                            + (64. - side) / 2.,
                                        695. + (64. - side) / 2.,
                                        side,
                                        side,
                                    ),
                                    id,
                                );
                            }
                        }
                        if warm == 0 {
                            next_frame().await;
                        } else {
                            crate::ui_capture::framebuffer()?.export_png(
                                output
                                    .join(format!(
                                        "review-{body}-{weapon}-H{}-{tick}.png",
                                        head.unwrap_or(0)
                                    ))
                                    .to_str()
                                    .unwrap(),
                            );
                        }
                    }
                }
                eprintln!(
                    "[ANIM-POLISH01] {name} / W{weapon:03} / H{}",
                    head.unwrap_or(0)
                );
            }
        }
    }
    let subject = Subject::prepare(player(3, Some(103), 31, None))?;
    let camp = output.join("camp");
    std::fs::create_dir_all(&camp).map_err(|e| e.to_string())?;
    crate::vertical_walk_review18::capture_prepared_camp(
        &camp,
        "ANIM-POLISH01 / actuel - 01 - 02",
        3,
        |rect, style, pose| {
            subject.draw(rect, pose, style);
            true
        },
    )
    .await?;
    ground_scene(output, &subject).await?;
    request_new_screen_size(540., 360.);
    for _ in 0..3 {
        next_frame().await;
    }
    for tick in 0..7 {
        for warm in 0..2 {
            set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
            clear_background(Color::from_rgba(30, 40, 49, 255));
            draw_text(
                "ANIM-POLISH01 / mitrailleuse / natif 64 px",
                14.,
                23.,
                17.,
                WHITE,
            );
            for (style, label) in ["Actuel", "01 / modere", "02 / accentue"]
                .into_iter()
                .enumerate()
            {
                let x = 14. + style as f32 * 180.;
                draw_text(label, x, 54., 17., WHITE);
                subject.draw(
                    Rect::new(x, 80., 64., 64.),
                    (tick < 6).then_some(Pose {
                        facing: 2,
                        frame: tick,
                    }),
                    style,
                );
                subject.draw(
                    Rect::new(x + 86., 80., 64., 64.),
                    (tick < 6).then_some(Pose {
                        facing: 1,
                        frame: tick,
                    }),
                    style,
                );
                if style == 0 {
                    game_art::draw_portrait(
                        Rect::new(x + 22., 165., 96., 96.),
                        player(3, Some(103), 31, None),
                    );
                } else {
                    subject.vertical[style].draw(
                        Rect::new(x + 22., 165., 96., 96.),
                        12,
                        false,
                        true,
                    );
                }
                let side = if style == 0 { 64. } else { 40. };
                for (i, id) in ["core:mitrailleuse_lourde", "core:lance_roquettes"]
                    .into_iter()
                    .enumerate()
                {
                    game_art::draw_item(
                        Rect::new(
                            x + i as f32 * 84. + (64. - side) / 2.,
                            280. + (64. - side) / 2.,
                            side,
                            side,
                        ),
                        id,
                    );
                }
            }
            if warm == 0 {
                next_frame().await;
            } else {
                crate::ui_capture::framebuffer()?.export_png(
                    output
                        .join(format!("native-summary-{tick}.png"))
                        .to_str()
                        .unwrap(),
                );
            }
        }
    }
    std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":"ANIM-POLISH01","status":"unapproved_candidates","normal_game_enabled":false,"native_tile_px":64,"ground_icon_px":{"current":64,"candidate":40},"heads_are_not_cut_or_moved_separately":true,"heavy_models":(0..113).filter(|m|heavy(*m)).collect::<Vec<_>>(),"portrait":"same whole frontal rest as world sprite","reports":reports})).unwrap()).map_err(|e|e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn compaction_scope_preserves_regular_rifles_pistols_and_melee() {
        let mut source = Image::gen_image_color(128, 128, BLANK);
        source.set_pixel(30, 64, WHITE);
        for model in [2, 14, 38, 54, 66, 70, 88, 104] {
            for style in 0..3 {
                assert_eq!(
                    source.bytes,
                    compact_weapon(&source, model, [60., 60.], [80., 60.], style).bytes
                );
            }
        }
        assert_eq!((0..113).filter(|m| heavy(*m)).count(), 28);
        assert_eq!(source_style(), 0);
        with_source(2, || assert_eq!(source_style(), 2));
        assert_eq!(source_style(), 0);
    }
    #[test]
    fn head_registration_moves_whole_figures_with_small_bounded_offsets() {
        let mut heads = vec![[64., 40.]; 13];
        heads[1] = [66., 42.];
        let r = roots(&heads);
        assert_eq!(r[1], [-2., -1.]);
        assert_eq!(r[12], [0., 0.]);
    }
    #[test]
    fn compaction_keeps_barrels_straight_when_grips_have_different_heights() {
        let mut source = Image::gen_image_color(128, 128, BLANK);
        for x in 10..110 {
            for y in 44..47 {
                source.set_pixel(x, y, WHITE);
            }
        }
        for style in [1, 2] {
            let image = compact_weapon(&source, 31, [48., 57.], [61., 66.], style);
            let mut centers = Vec::new();
            for x in 25..90usize {
                let mut sum = 0.;
                let mut weight = 0.;
                for y in 0..128usize {
                    let alpha = f32::from(image.bytes[(y * 128 + x) * 4 + 3]);
                    sum += y as f32 * alpha;
                    weight += alpha;
                }
                if weight > 100. {
                    centers.push(sum / weight);
                }
            }
            let low = centers.iter().copied().fold(f32::INFINITY, f32::min);
            let high = centers.iter().copied().fold(f32::NEG_INFINITY, f32::max);
            assert!(
                centers.len() > 40 && high - low < 0.01,
                "Compaction bent a straight barrel"
            );
        }
    }
    #[test]
    fn all_heavy_weapon_grips_keep_opaque_contact_after_compaction() {
        fn distance(image: &Image, p: [f32; 2]) -> f32 {
            image
                .bytes
                .chunks_exact(4)
                .enumerate()
                .filter(|(_, v)| v[3] >= 64)
                .map(|(i, _)| ((i % 128) as f32 + 0.5 - p[0]).hypot((i / 128) as f32 + 0.5 - p[1]))
                .fold(f32::INFINITY, f32::min)
        }
        for model in (0..113).filter(|m| heavy(*m)) {
            let (source, a, b) =
                game_art::vertical_review_weapon(model, false, EquipmentPalette::Original).unwrap();
            for style in [1, 2] {
                let candidate = compact_weapon(&source, model, a, b.unwrap(), style);
                for point in [a, b.unwrap()] {
                    let before = distance(&source, point);
                    let after = distance(&candidate, point);
                    assert!(
                        after <= before.max(1.) + 0.5,
                        "W{model:03} style {style} grip {point:?}: {before} -> {after}"
                    );
                }
            }
        }
    }
    #[test]
    fn candidate_compositions_leave_normal_images_and_source_caches_unchanged() {
        let visual = player(3, Some(103), 31, None);
        let pose = Pose {
            facing: 1,
            frame: 1,
        };
        let normal = game_art::polish_review_lateral(visual, pose).unwrap();
        let candidate = with_source(2, || game_art::polish_review_lateral(visual, pose)).unwrap();
        assert!(
            normal.bytes != candidate.bytes,
            "Heavy lateral candidate did not change the weapon"
        );
        assert!(
            normal.bytes == game_art::polish_review_lateral(visual, pose).unwrap().bytes,
            "Review modified the normal lateral cache"
        );
        let normal = game_art::vertical::review_image(visual).unwrap();
        let candidate = with_source(2, || game_art::vertical::review_image(visual)).unwrap();
        assert!(normal.cell != candidate.cell || normal.image.bytes != candidate.image.bytes);
        assert!(
            normal.image.bytes
                == game_art::vertical::review_image(visual)
                    .unwrap()
                    .image
                    .bytes,
            "Review modified the normal vertical cache"
        );
        assert_eq!(source_style(), 0);
    }
}
