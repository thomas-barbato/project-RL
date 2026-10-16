//! Shared terminal presentation. Colors always resolve the player's active theme.
use crate::ui_theme::{self, UiTheme};
use macroquad::prelude::*;

/// Fill the two existing font atlases before the first rendered frame. Preparing
/// labels one at a time during inventory opening otherwise repeatedly uploads
/// an atlas that is still being used by the world/HUD from the same frame.
pub(crate) fn prepare_text() {
    const GLYPHS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789 ÀÂÄÇÉÈÊËÎÏÔÖÙÛÜàâäçéèêëîïôöùûüœŒ·…%+-/:()[]'’";
    for size in [12, 13, 14, 15, 20, 22] {
        let _ = ui_theme::measure_text(GLYPHS, None, size, 1.0);
    }
    for size in [12, 13, 14, 17] {
        let _ = ui_theme::measure_text_bold(GLYPHS, size);
    }
}

pub(crate) fn tint(alpha: f32) -> Color {
    let c = UiTheme.accent();
    Color::new(c.r, c.g, c.b, alpha)
}

pub(crate) fn ink() -> Color {
    let c = UiTheme.accent();
    Color::new(
        0.017 + c.r * 0.025,
        0.022 + c.g * 0.025,
        0.029 + c.b * 0.025,
        1.0,
    )
}

pub(crate) fn heading(text: &str, x: f32, y: f32, size: u16, color: Color) {
    // Prepare every glyph together. Uploading the full font atlas between each
    // previously unseen letter stalls the first inventory opening.
    let _ = ui_theme::measure_text_bold(text, size);
    let advance = f32::from(size) * 0.62;
    for (i, ch) in text.chars().enumerate() {
        ui_theme::draw_text_bold(
            ch.to_string(),
            x + i as f32 * advance,
            y,
            f32::from(size),
            color,
        );
    }
}

pub(crate) fn bezel(rect: Rect) {
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, tint(0.32));
    for (x, sx) in [(rect.x, 1.0), (rect.right(), -1.0)] {
        for (y, sy) in [(rect.y, 1.0), (rect.bottom(), -1.0)] {
            draw_line(
                x,
                y,
                x + sx * 12.0_f32.min(rect.w * 0.25),
                y,
                2.0,
                tint(0.9),
            );
            draw_line(x, y, x, y + sy * 9.0_f32.min(rect.h * 0.25), 2.0, tint(0.9));
        }
    }
}

pub(crate) fn module(rect: Rect, label: &str) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, ink());
    draw_rectangle(rect.x + 1.0, rect.y + 1.0, rect.w - 2.0, 26.0, tint(0.085));
    draw_line(
        rect.x + 1.0,
        rect.y + 27.0,
        rect.right() - 1.0,
        rect.y + 27.0,
        1.0,
        tint(0.35),
    );
    // Fit translated headings in narrow modules without crossing their frame.
    let size = ((rect.w - 20.0) / (label.chars().count().max(1) as f32 * 0.62))
        .floor()
        .clamp(10.0, 13.0) as u16;
    let capacity = ((rect.w - 20.0) / (f32::from(size) * 0.62)).floor() as usize;
    let label = if label.chars().count() > capacity {
        label
            .chars()
            .take(capacity.saturating_sub(1))
            .chain(['…'])
            .collect::<String>()
    } else {
        label.to_owned()
    };
    heading(&label, rect.x + 10.0, rect.y + 18.0, size, UiTheme.accent());
    bezel(rect);
}

pub(crate) fn segment(bar: Rect, divisions: usize) {
    for i in 1..divisions {
        let x = (bar.x + bar.w * i as f32 / divisions as f32).round();
        draw_line(x, bar.y, x, bar.bottom(), 1.0, ink());
    }
}

pub(crate) fn button(rect: Rect, focused: bool, active: bool, enabled: bool) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, ink());
    if enabled && (focused || active) {
        draw_rectangle(
            rect.x + 1.0,
            rect.y + 1.0,
            rect.w - 2.0,
            rect.h - 2.0,
            tint(0.13),
        );
    }
    bezel(rect);
    if !enabled {
        draw_rectangle(
            rect.x,
            rect.y,
            rect.w,
            rect.h,
            Color::new(0.0, 0.0, 0.0, 0.35),
        );
    }
}

pub(crate) fn command_rect(width: f32, height: f32, index: usize) -> Rect {
    let gap = 4.0;
    let w = (width - 40.0 - gap * 9.0) / 10.0;
    Rect::new(20.0 + index as f32 * (w + gap), height - 94.0, w, 36.0)
}

pub(crate) fn command(rect: Rect, label: &str, binding: &str, focused: bool) {
    button(rect, focused, false, true);
    ui_theme::draw_text_in_rect(
        label,
        Rect::new(rect.x + 7.0, rect.y + 1.0, rect.w - 14.0, 17.0),
        if rect.w < 80.0 { 12 } else { 13 },
        UiTheme.text(),
    );
    ui_theme::draw_text_in_rect(
        format!("[{binding}]"),
        Rect::new(rect.x + 7.0, rect.y + 17.0, rect.w - 14.0, 17.0),
        12,
        UiTheme.accent(),
    );
}
