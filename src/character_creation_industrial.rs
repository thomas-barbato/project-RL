//! Accepted CREATION-UI02-D r03 finish, shared by normal Debug and Release.
use super::*;
use crate::asset_preparation::{self as preparation, PreparedImage, Request};

pub(super) fn enabled() -> bool {
    true
}

const FLOOR: &str = "creation-ui02D-r01-floor";
const LOCKER: &str = "creation-ui02D-r01-locker";
const CRATE: &str = "creation-ui02D-r01-crate";

fn requests() -> Vec<Request> {
    [
        (FLOOR, "assets/maps/cryogenic-camp-01/floor.png"),
        (LOCKER, "assets/maps/cryogenic-camp-01/DEC-CAMP-11-r05.png"),
        (CRATE, "assets/maps/cryogenic-camp-01/DEC-CAMP-14-r05.png"),
    ]
    .into_iter()
    .map(|(key, path)| {
        (
            key.to_owned(),
            Box::new(move || {
                let source = preparation::image(path)?;
                let image = if key == FLOOR {
                    source.sub_image(Rect::new(128., 384., 192., 192.))
                } else {
                    (*source).clone()
                };
                Ok(PreparedImage {
                    image,
                    cell: [64, 64],
                    anchor: [32., 64.],
                })
            }) as preparation::Builder,
        )
    })
    .collect()
}

/// Reuses the common bounded CPU/GPU caches, before the first creation draw.
fn wanted(app: &AsciiApp) -> bool {
    let Some(preview) = &app.ux.creation_details else {
        return false;
    };
    let layout = CharacterCreationLayout::new(
        app.ui_width(),
        app.ui_height(),
        app.character_classes.iter().count(),
    );
    let details = super::character_creation_details::DetailsLayout::new(
        &layout,
        preview.items.len(),
        preview.skills.len(),
        0.,
    );
    scene_rect(details.body, details.attributes_y + 21.).is_some()
}
fn ready() -> bool {
    [FLOOR, LOCKER, CRATE]
        .into_iter()
        .all(|key| preparation::ready(key).unwrap_or(false))
}
pub(super) fn prepare(app: &AsciiApp) -> Result<(), String> {
    if enabled() && preparation::graphics_thread_ready() && wanted(app) {
        preparation::prepare(requests)?;
    }
    Ok(())
}

/// Resizing may reveal the scene. Queue it during input, then upload on the GL
/// thread; drawing waits for the complete scene rather than loading resources.
pub(super) fn tick(app: &AsciiApp) {
    if !enabled() || !preparation::graphics_thread_ready() || !wanted(app) || ready() {
        return;
    }
    for (key, build) in requests() {
        preparation::request(key, build);
    }
    preparation::pump();
}

fn alpha(mut c: Color, a: f32) -> Color {
    c.a = a;
    c
}
fn mix(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}
fn steel() -> Color {
    if crate::ui_theme::interface_style() == crate::graphics::InterfaceStyle::Terminal {
        Color::from_rgba(22, 30, 24, 255)
    } else {
        Color::from_rgba(25, 32, 38, 255)
    }
}
fn inset(r: Rect, n: f32) -> Rect {
    Rect::new(r.x + n, r.y + n, r.w - 2. * n, r.h - 2. * n)
}
fn cut(r: Rect, c: f32, color: Color) {
    let c = c.min(r.h * 0.25).min(r.w * 0.25);
    let points = [
        vec2(r.x + c, r.y),
        vec2(r.right() - c, r.y),
        vec2(r.right(), r.y + c),
        vec2(r.right(), r.bottom() - c),
        vec2(r.right() - c, r.bottom()),
        vec2(r.x + c, r.bottom()),
        vec2(r.x, r.bottom() - c),
        vec2(r.x, r.y + c),
    ];
    for i in 0..8 {
        draw_triangle(r.center(), points[i], points[(i + 1) % 8], color);
    }
}
fn plate(r: Rect, c: f32, fill: Color, edge: Color) {
    // One contour: highlights share its pixels, without an offset outline.
    cut(r, c, edge);
    cut(inset(r, 1.), (c - 1.).max(1.), fill);
    draw_line(
        r.x + c,
        r.y + 0.5,
        r.right() - c,
        r.y + 0.5,
        1.,
        alpha(WHITE, 0.10),
    );
    draw_line(
        r.x + 0.5,
        r.y + c,
        r.x + 0.5,
        r.bottom() - c,
        1.,
        alpha(WHITE, 0.035),
    );
    draw_line(
        r.x + c,
        r.bottom() - 0.5,
        r.right() - c,
        r.bottom() - 0.5,
        1.,
        alpha(BLACK, 0.25),
    );
}
fn bolt(x: f32, y: f32) {
    draw_circle(x, y, 2.7, Color::from_rgba(6, 10, 12, 255));
    draw_line(x - 1.5, y - 0.5, x + 1., y - 0.5, 1., alpha(WHITE, 0.22));
}

pub(super) fn shell(r: Rect) {
    plate(
        r,
        12.,
        mix(steel(), BLACK, 0.45),
        mix(steel(), UiTheme.muted(), 0.30),
    );
    cut(
        inset(r, 4.),
        8.,
        super::character_creation_polish::background(),
    );
    // Brushed upper and lower rails; the content surfaces stay quiet.
    for n in 0..6 {
        draw_line(
            r.x + 14.,
            r.y + 8. + n as f32,
            r.right() - 14.,
            r.y + 8. + n as f32,
            1.,
            alpha(WHITE, 0.025 - n as f32 * 0.003),
        );
    }
    for (x, y) in [
        (r.x + 13., r.y + 18.),
        (r.right() - 13., r.y + 18.),
        (r.x + 13., r.bottom() - 18.),
        (r.right() - 13., r.bottom() - 18.),
    ] {
        bolt(x, y);
    }
}

pub(super) fn class_well(rows: &[Rect]) {
    if let (Some(a), Some(b)) = (rows.first(), rows.last()) {
        cut(
            Rect::new(a.x - 7., a.y - 7., a.w + 14., b.bottom() - a.y + 14.),
            7.,
            mix(steel(), BLACK, 0.70),
        );
    }
}

pub(super) fn panel(r: Rect) {
    plate(
        r,
        8.,
        mix(steel(), BLACK, 0.50),
        alpha(UiTheme.muted(), 0.33),
    );
    draw_line(
        r.x + 9.,
        r.y + 4.,
        r.right() - 9.,
        r.y + 4.,
        1.,
        alpha(UiTheme.accent(), 0.24),
    );
    for y in [r.y + 15., r.bottom() - 15.] {
        draw_line(
            r.right() - 5.,
            y,
            r.right() - 5.,
            y + 5.,
            1.,
            alpha(WHITE, 0.17),
        );
    }
}

pub(super) fn icon_frame(r: Rect) {
    plate(
        r,
        4.,
        mix(steel(), BLACK, 0.76),
        alpha(UiTheme.muted(), 0.25),
    );
}

pub(super) fn card(r: Rect, key: u16, selected: f32, hover: f32) {
    let is_class = key < 1000;
    let fill = mix(
        mix(steel(), BLACK, if is_class { 0.16 } else { 0.38 }),
        UiTheme.surface_selected(),
        selected * 0.34 + hover * 0.1,
    );
    let edge = mix(
        alpha(UiTheme.muted(), 0.28 + hover * 0.18),
        alpha(UiTheme.accent(), 0.78),
        selected,
    );
    plate(r, if is_class { 8. } else { 5. }, fill, edge);
    if is_class {
        // Low-contrast horizontal brushing gives the slots a material without
        // a texture decode, random simulation state, or noise over the labels.
        for n in 0..7 {
            let y = r.y + 9. + n as f32 * (r.h - 18.) / 7.;
            draw_line(r.x + 56., y, r.right() - 9., y, 1., alpha(WHITE, 0.012));
        }
    }
    if is_class {
        cut(
            Rect::new(r.x + 3., r.y + 5., 50., r.h - 10.),
            4.,
            mix(fill, BLACK, 0.35),
        );
        draw_rectangle(
            r.x + 1.,
            r.y + 14.,
            1.,
            r.h - 28.,
            alpha(UiTheme.accent(), selected),
        );
    } else {
        draw_line(
            r.right() - 14.,
            r.bottom() - 3.,
            r.right() - 6.,
            r.bottom() - 3.,
            1.,
            alpha(UiTheme.accent(), selected * 0.8),
        );
    }
}

pub(super) fn button(
    r: Rect,
    label: &str,
    primary: bool,
    hover: f32,
    enabled_button: bool,
    pressed: bool,
) {
    let tint = if primary {
        UiTheme.accent()
    } else {
        UiTheme.muted()
    };
    let fill = mix(
        steel(),
        tint,
        if pressed { 0.09 } else { 0.15 + hover * 0.14 },
    );
    plate(
        r,
        6.,
        fill,
        alpha(tint, if primary { 0.75 } else { 0.3 + hover * 0.2 }),
    );
    if primary {
        draw_rectangle(r.x + 10., r.bottom() - 3., r.w - 20., 1., alpha(tint, 0.7));
    }
    if pressed {
        cut(inset(r, 3.), 3., alpha(BLACK, 0.15));
    }
    let mut size = 16;
    while size > 12 && measure_text_bold(label, size).width > r.w - 24. {
        size -= 1;
    }
    draw_text_bold_centered(
        label,
        r,
        size,
        if enabled_button {
            UiTheme.text()
        } else {
            UiTheme.muted()
        },
    );
}

pub(super) fn attributes(attributes: PrimaryAttributes, x: f32, y: f32, width: f32) -> f32 {
    let cell = width / 3.;
    for (i, attribute) in PrimaryAttribute::ALL.into_iter().enumerate() {
        let at = x + (i % 3) as f32 * cell;
        let baseline = y + (i / 3) as f32 * 17.;
        draw_text(
            primary_attribute_label(attribute),
            at,
            baseline,
            12.,
            UiTheme.muted(),
        );
        let value = attributes.value(attribute).to_string();
        draw_text_bold(
            &value,
            at + cell - 12. - measure_text_bold(&value, 14).width,
            baseline,
            14.,
            UiTheme.text(),
        );
    }
    y + 21.
}

pub(super) fn scene_rect(body: Rect, after: f32) -> Option<Rect> {
    let top = after + 18.;
    let free = body.bottom() - top - 4.;
    (body.w >= 240. && free >= 216.).then(|| {
        Rect::new(
            (body.x + (body.w - 216.) * 0.5).floor(),
            (top + (free - 216.) * 0.5).floor(),
            216.,
            216.,
        )
    })
}

pub(super) fn scene(body: Rect, after: f32, visual: crate::game_art::Visual) {
    let Some(r) = scene_rect(body, after) else {
        return;
    };
    if !ready() {
        return;
    }
    panel(r);
    let at = vec2(r.x + 12., r.y + 12.);
    preparation::with_texture(FLOOR, |p| draw_texture(&p.texture, at.x, at.y, WHITE));
    preparation::with_texture(LOCKER, |p| {
        draw_texture(&p.texture, at.x - 8., at.y - 24., WHITE)
    });
    preparation::with_texture(CRATE, |p| {
        draw_texture(&p.texture, at.x + 72., at.y + 24., WHITE)
    });
    draw_ellipse(
        at.x + 96.,
        at.y + 129.,
        20.,
        5.,
        0.,
        Color::new(0., 0., 0., 0.35),
    );
    crate::game_art::draw_portrait(Rect::new(at.x + 64., at.y + 68., 64., 64.), visual);
    draw_rectangle_lines(at.x - 1., at.y - 1., 194., 194., 1., alpha(BLACK, 0.7));
}

#[cfg(debug_assertions)]
pub(super) async fn audit_lifecycle(app: &mut AsciiApp) -> Result<serde_json::Value, String> {
    let original = (screen_width(), screen_height());
    let first = preparation::stats();
    app.begin_character_creation_with_seed(false, app.seed)?;
    let reopened = preparation::stats();
    for key in ["jobs", "uploads", "source_reads"] {
        if first[key] != reopened[key] {
            return Err(format!("Creation reopen repeated {key}"));
        }
    }
    let mut resized = None;
    if original.0 < 1100. {
        request_new_screen_size(1280., 800.);
        for _ in 0..3 {
            next_frame().await;
        }
        for n in 0..300 {
            tick(app);
            if ready() {
                break;
            }
            if n == 299 {
                return Err("Resize preparation timed out".into());
            }
            next_frame().await;
        }
        let prepared = preparation::stats();
        app.draw();
        next_frame().await;
        let drawn = preparation::stats();
        if prepared["source_reads"] != drawn["source_reads"]
            || prepared["uploads"] != drawn["uploads"]
        {
            return Err("Resize drawing performed resource work".into());
        }
        request_new_screen_size(original.0, original.1);
        for _ in 0..3 {
            next_frame().await;
        }
        resized = Some(
            serde_json::json!({"small_to_large_then_back":true,"resource_work_in_draw":false,"prepared":prepared}),
        );
    }
    Ok(
        serde_json::json!({"before_reopen":first,"after_reopen":reopened,"reuse_verified":true,"resize":resized}),
    )
}
