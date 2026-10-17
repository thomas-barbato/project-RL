//! Cached, presentation-only surface textures. Never consumes simulation RNG.
use crate::{
    graphics::InterfaceStyle,
    ui_theme::{self, UiTheme},
};
use macroquad::prelude::*;

thread_local! { static GRAIN: std::cell::OnceCell<Texture2D> = const { std::cell::OnceCell::new() }; }

pub fn surface(rect: Rect, color: Color) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
    if ui_theme::interface_style() == InterfaceStyle::Terminal {
        let mut y = rect.y + 2.;
        while y < rect.bottom() {
            draw_line(rect.x, y, rect.right(), y, 1., Color::new(0., 0., 0., 0.12));
            y += 3.;
        }
    } else {
        GRAIN.with(|cache| {
            let texture = cache.get_or_init(|| {
                let mut seed = 4016u32;
                let mut pixels = Vec::with_capacity(128 * 128 * 4);
                for _ in 0..128 * 128 {
                    seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                    let light = (seed >> 16) & 1 != 0;
                    pixels.extend_from_slice(&[
                        if light { 182 } else { 0 },
                        if light { 190 } else { 0 },
                        if light { 194 } else { 0 },
                        ((seed >> 24) % 15) as u8,
                    ]);
                }
                let t = Texture2D::from_rgba8(128, 128, &pixels);
                t.set_filter(FilterMode::Nearest);
                t
            });
            let mut y = rect.y;
            while y < rect.bottom() {
                let mut x = rect.x;
                while x < rect.right() {
                    let w = 128f32.min(rect.right() - x);
                    let h = 128f32.min(rect.bottom() - y);
                    draw_texture_ex(
                        texture,
                        x,
                        y,
                        WHITE,
                        DrawTextureParams {
                            source: Some(Rect::new(0., 0., w, h)),
                            ..Default::default()
                        },
                    );
                    x += 128.;
                }
                y += 128.;
            }
        });
    }
}

pub fn frame(rect: Rect) {
    surface(rect, UiTheme.surface());
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.,
        Color::new(0.45, 0.55, 0.55, 0.27),
    );
}
