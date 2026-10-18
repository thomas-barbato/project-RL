//! AIM-VIS-02: approved blue hologram with circles and red invalid seal.
//! Original candidates remain available only to the isolated debug gallery.
use macroquad::prelude::*;
#[cfg(debug_assertions)]
use std::cell::Cell;

#[derive(Clone, Copy)]
pub enum Style {
    Hologram,
    Seal,
    #[cfg(debug_assertions)]
    Etched,
    Combined,
}

#[cfg(debug_assertions)]
thread_local! { static STYLE: Cell<Option<Style>> = const { Cell::new(Some(Style::Combined)) }; }
#[cfg(debug_assertions)]
pub struct Guard(Option<Style>);
#[cfg(debug_assertions)]
impl Drop for Guard {
    fn drop(&mut self) {
        STYLE.with(|s| s.set(self.0));
    }
}
#[cfg(debug_assertions)]
pub fn begin(style: Option<Style>) -> Guard {
    Guard(STYLE.with(|s| s.replace(style)))
}
pub fn active() -> bool {
    selected_style().is_some()
}
fn selected_style() -> Option<Style> {
    #[cfg(debug_assertions)]
    {
        STYLE.with(|s| s.get())
    }
    #[cfg(not(debug_assertions))]
    {
        Some(Style::Combined)
    }
}

fn ink(style: Style, valid: bool) -> Color {
    if !valid {
        return Color::from_rgba(255, 48, 38, 255);
    }
    match style {
        Style::Hologram | Style::Combined => Color::from_rgba(106, 236, 225, 255),
        Style::Seal => Color::from_rgba(246, 207, 125, 255),
        #[cfg(debug_assertions)]
        Style::Etched => Color::from_rgba(197, 178, 255, 255),
    }
}
fn alpha(c: Color, a: f32) -> Color {
    Color::new(c.r, c.g, c.b, a)
}
fn line(a: Vec2, b: Vec2, width: f32, color: Color) {
    let color = if color.r < 0.05 && color.g < 0.06 {
        crate::effect_geometry::underlay(color)
    } else {
        crate::effect_geometry::stroke(color)
    };
    draw_line(a.x, a.y, b.x, b.y, width, color);
}

pub fn area(rect: Rect, valid: bool, edge: [bool; 4]) {
    let Some(style) = selected_style() else {
        return;
    };
    let c = ink(style, valid);
    let combined_valid = valid && matches!(style, Style::Combined);
    let style = if matches!(style, Style::Combined) {
        if valid { Style::Hologram } else { Style::Seal }
    } else {
        style
    };
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        alpha(c, if valid { 0.105 } else { 0.15 }),
    );
    // Fine invalid hatching is distinct even without color discrimination.
    if !valid {
        let mut d = -rect.h;
        while d < rect.w {
            let a = vec2(rect.x + d.max(0.), rect.y + (-d).max(0.));
            let b = vec2(
                rect.x + (d + rect.h).min(rect.w),
                rect.y + (rect.w - d).min(rect.h),
            );
            line(a, b, 1., alpha(c, 0.28));
            d += 13.;
        }
    } else if matches!(style, Style::Hologram) {
        for y in [18., 42.] {
            line(
                vec2(rect.x + 3., rect.y + y),
                vec2(rect.right() - 3., rect.y + y),
                1.,
                alpha(c, 0.075),
            );
        }
    }
    let pad = if matches!(style, Style::Seal) {
        6.
    } else {
        1.5
    };
    let ends = [
        (
            vec2(rect.x + if edge[3] { pad } else { 0. }, rect.y + 1.5),
            vec2(rect.right() - if edge[1] { pad } else { 0. }, rect.y + 1.5),
        ),
        (
            vec2(rect.right() - 1.5, rect.y + if edge[0] { pad } else { 0. }),
            vec2(
                rect.right() - 1.5,
                rect.bottom() - if edge[2] { pad } else { 0. },
            ),
        ),
        (
            vec2(rect.x + if edge[3] { pad } else { 0. }, rect.bottom() - 1.5),
            vec2(
                rect.right() - if edge[1] { pad } else { 0. },
                rect.bottom() - 1.5,
            ),
        ),
        (
            vec2(rect.x + 1.5, rect.y + if edge[0] { pad } else { 0. }),
            vec2(rect.x + 1.5, rect.bottom() - if edge[2] { pad } else { 0. }),
        ),
    ];
    for (i, (a, b)) in ends.into_iter().enumerate() {
        if !edge[i] {
            continue;
        }
        line(a, b, 6., Color::from_rgba(5, 13, 18, 235));
        match style {
            Style::Hologram => {
                line(a, b, 9., alpha(c, 0.12));
                line(a, b, 4., alpha(c, 0.32));
                line(a, b, 1.7, alpha(c, 0.97));
            }
            Style::Seal => {
                line(a, b, 3.5, alpha(c, 0.30));
                line(a, b, 1.8, c);
                let mid = (a + b) * 0.5;
                crate::preview_markers::diamond(
                    mid,
                    3.5,
                    1.2,
                    Color::from_rgba(10, 13, 20, 245),
                    c,
                );
            }
            #[cfg(debug_assertions)]
            Style::Etched => {
                let dir = (b - a).normalize_or_zero();
                let len = a.distance(b);
                let mut at = 0.;
                while at < len {
                    line(a + dir * at, a + dir * (at + 11.).min(len), 2.2, c);
                    at += 15.;
                }
            }
            Style::Combined => unreachable!("combined style resolved above"),
        }
        if combined_valid {
            let mid = (a + b) * 0.5;
            crate::preview_markers::circle(mid, 3.5, 1.2, Color::from_rgba(10, 13, 20, 245), c);
        }
    }
    if matches!(style, Style::Seal) {
        for (one, two, a, b) in [
            (
                0,
                3,
                vec2(rect.x + 1.5, rect.y + 6.),
                vec2(rect.x + 6., rect.y + 1.5),
            ),
            (
                0,
                1,
                vec2(rect.right() - 6., rect.y + 1.5),
                vec2(rect.right() - 1.5, rect.y + 6.),
            ),
            (
                2,
                3,
                vec2(rect.x + 1.5, rect.bottom() - 6.),
                vec2(rect.x + 6., rect.bottom() - 1.5),
            ),
            (
                2,
                1,
                vec2(rect.right() - 6., rect.bottom() - 1.5),
                vec2(rect.right() - 1.5, rect.bottom() - 6.),
            ),
        ] {
            if edge[one] && edge[two] {
                line(a, b, 4., BLACK);
                line(a, b, 1.7, c);
            }
        }
    }
    #[cfg(debug_assertions)]
    if matches!(style, Style::Etched) {
        for p in [
            vec2(rect.x + 3., rect.y + 3.),
            vec2(rect.right() - 3., rect.bottom() - 3.),
        ] {
            draw_circle(p.x, p.y, 1., alpha(c, 0.6));
        }
    }
}

pub fn cursor(rect: Rect, valid: bool) {
    let Some(style) = selected_style() else {
        return;
    };
    let c = ink(style, valid);
    for (x, y, dx, dy) in [
        (rect.x + 4., rect.y + 4., 1., 1.),
        (rect.right() - 4., rect.y + 4., -1., 1.),
        (rect.x + 4., rect.bottom() - 4., 1., -1.),
        (rect.right() - 4., rect.bottom() - 4., -1., -1.),
    ] {
        let a = vec2(x + dx * 11., y);
        let b = vec2(x, y);
        let d = vec2(x, y + dy * 11.);
        line(a, b, 5., Color::from_rgba(3, 9, 15, 245));
        line(b, d, 5., Color::from_rgba(3, 9, 15, 245));
        line(a, b, 2.1, c);
        line(b, d, 2.1, c);
    }
    let p = vec2(rect.center().x, rect.y + 5.);
    draw_poly(p.x, p.y, 4, 4., 0., Color::from_rgba(5, 10, 16, 245));
    draw_poly_lines(p.x, p.y, 4, 4., 0., 1.4, c);
    if !valid {
        let p = vec2(rect.right() - 11., rect.y + 11.);
        for (a, b) in [
            (vec2(-3., -3.), vec2(3., 3.)),
            (vec2(-3., 3.), vec2(3., -3.)),
        ] {
            line(p + a, p + b, 4., Color::from_rgba(5, 10, 16, 240));
            line(p + a, p + b, 1.5, c);
        }
    }
}
