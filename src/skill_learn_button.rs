//! LEARN-BTN-01: the accepted beveled skill purchase button.
use crate::{graphics::InterfaceStyle, ui_theme::UiTheme};
use macroquad::prelude::*;
#[derive(Clone, Copy)]
pub(crate) enum State {
    Ready,
    Hover,
    Pressed,
    Disabled,
    Capped,
}
fn alpha(mut c: Color, a: f32) -> Color {
    c.a *= a;
    c
}
fn polygon(points: &[Vec2], c: Color) {
    for i in 1..points.len() - 1 {
        draw_triangle(points[0], points[i], points[i + 1], c);
    }
}
fn outline(points: &[Vec2], width: f32, c: Color) {
    for i in 0..points.len() {
        let (a, b) = (points[i], points[(i + 1) % points.len()]);
        draw_line(a.x, a.y, b.x, b.y, width, c);
    }
}
fn shape(r: Rect) -> [Vec2; 8] {
    let (x, y, w, h) = (r.x, r.y, r.w, r.h);
    [
        vec2(x + 5., y),
        vec2(x + w - 5., y),
        vec2(x + w, y + 5.),
        vec2(x + w, y + h - 5.),
        vec2(x + w - 5., y + h),
        vec2(x + 5., y + h),
        vec2(x, y + h - 5.),
        vec2(x, y + 5.),
    ]
}
fn inset(r: Rect, amount: f32) -> Rect {
    Rect::new(
        r.x + amount,
        r.y + amount,
        r.w - amount * 2.,
        r.h - amount * 2.,
    )
}
pub(crate) fn draw(r: Rect, state: State) {
    let disabled = matches!(state, State::Disabled | State::Capped);
    let hover = matches!(state, State::Hover);
    let pressed = matches!(state, State::Pressed);
    let terminal = crate::ui_theme::interface_style() == InterfaceStyle::Terminal;
    let accent = if disabled {
        Color::from_rgba(91, 99, 111, 255)
    } else {
        UiTheme.accent()
    };
    let edge = if hover || pressed {
        accent
    } else {
        Color::from_rgba(93, 101, 115, 255)
    };
    let outer = shape(r);
    let shadow = outer.map(|p| p + vec2(0., 2.));
    polygon(&shadow, Color::from_rgba(0, 1, 4, 205));
    polygon(&outer, Color::from_rgba(9, 12, 17, 255));
    outline(&outer, 1., edge);
    let inner = shape(inset(r, 3.));
    let fill = if pressed {
        Color::from_rgba(15, 18, 28, 255)
    } else if hover {
        Color::new(accent.r * 0.26, accent.g * 0.26, accent.b * 0.26, 1.)
    } else {
        Color::from_rgba(35, 41, 51, 255)
    };
    polygon(&inner, fill);
    outline(&inner, 1., Color::from_rgba(64, 71, 83, 255));
    if !terminal {
        // Low contrast machining marks stay inside the inset shape.
        for (i, y) in [9., 13., 17., 21., 25.].iter().enumerate() {
            let trim = 6.;
            draw_line(
                r.x + trim,
                r.y + y,
                r.right() - trim,
                r.y + y,
                1.,
                Color::from_rgba(156, 169, 185, if i % 2 == 0 { 10 } else { 5 }),
            );
        }
        let a = inner[0];
        let b = inner[1];
        draw_line(
            a.x,
            a.y,
            b.x,
            b.y,
            1.,
            alpha(WHITE, if disabled { 0.07 } else { 0.18 }),
        );
    }
    if hover {
        outline(&shape(inset(r, -1.)), 1., alpha(accent, 0.27));
    }
    let centre = r.center() + if pressed { vec2(0., 1.) } else { Vec2::ZERO };
    let glyph = if disabled {
        Color::from_rgba(112, 121, 135, 255)
    } else if hover {
        Color::from_rgba(236, 233, 255, 255)
    } else {
        accent
    };
    if matches!(state, State::Capped) {
        draw_line(
            centre.x - 5.,
            centre.y,
            centre.x - 1.,
            centre.y + 4.,
            2.,
            glyph,
        );
        draw_line(
            centre.x - 1.,
            centre.y + 4.,
            centre.x + 6.,
            centre.y - 4.,
            2.,
            glyph,
        );
    } else {
        draw_rectangle(centre.x - 6. + 1., centre.y - 1. + 1., 12., 3., BLACK);
        draw_rectangle(centre.x - 1. + 1., centre.y - 6. + 1., 3., 12., BLACK);
        draw_rectangle(centre.x - 6., centre.y - 1., 12., 3., glyph);
        draw_rectangle(centre.x - 1., centre.y - 6., 3., 12., glyph);
    }
}
