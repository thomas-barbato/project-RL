//! Accepted D66-TAUNT-A01/I01, shared between the review and the live renderer.
use macroquad::prelude::*;
use project_rl::world::{GridPos, Map, has_line_of_sight};
fn opacity_color(mut color: Color, opacity: f32) -> Color {
    color.a *= opacity;
    color
}
pub(crate) fn marker(p: Vec2, opacity: f32) {
    let dark = opacity_color(Color::from_rgba(24, 12, 9, 255), opacity);
    let red = opacity_color(Color::from_rgba(244, 95, 53, 255), opacity);
    let lit = opacity_color(Color::from_rgba(255, 207, 133, 255), opacity);
    let a = p + vec2(-2., -9.);
    let b = p + vec2(2., -9.);
    let c = p + vec2(1.2, 2.);
    let d = p + vec2(-1.2, 2.);
    draw_triangle(a, b, c, red);
    draw_triangle(a, c, d, red);
    for (from, to) in [(a, b), (b, c), (c, d), (d, a)] {
        draw_line(from.x, from.y, to.x, to.y, 1.3, dark);
    }
    draw_line(p.x - 0.6, p.y - 7.5, p.x - 0.6, p.y, 1.2, lit);
    draw_circle(p.x, p.y + 7., 3.1, dark);
    draw_circle(p.x, p.y + 7., 1.9, red);
    for sign in [-1., 1.] {
        let points = [
            p + vec2(sign * 10., -4.),
            p + vec2(sign * 6.5, 0.),
            p + vec2(sign * 10., 4.),
        ];
        for line in points.windows(2) {
            draw_line(line[0].x, line[0].y, line[1].x, line[1].y, 4., dark);
            draw_line(line[0].x, line[0].y, line[1].x, line[1].y, 2., red);
        }
    }
}
pub(crate) fn pulse(map: &Map, source: GridPos, age: f32, rect: impl Fn(GridPos) -> Option<Rect>) {
    if !(0.0..0.48).contains(&age) {
        return;
    }
    let Some(r) = rect(source) else {
        return;
    };
    let t = age / 0.48;
    let fade = (1. - t).powf(0.65);
    let scale = r.w / 64.;
    for ring in [0., 0.19] {
        let phase = (t - ring).max(0.) / (1. - ring);
        if phase <= 0. {
            continue;
        }
        let radius = 26. + phase * 128.;
        for n in 0..96 {
            let local = |n: usize| {
                let a = n as f32 * std::f32::consts::TAU / 96.;
                vec2(a.cos(), a.sin()) * radius
            };
            let (a, b) = (local(n), local(n + 1));
            let free = |v: Vec2| {
                let at = GridPos::new(
                    source.x + ((v.x + 32.) / 64.).floor() as i32,
                    source.y + ((v.y + 32.) / 64.).floor() as i32,
                );
                rect(at).is_some()
                    && map.is_walkable(at)
                    && has_line_of_sight(map, source, at, true)
            };
            if !free(a) || !free(b) {
                continue;
            }
            let (a, b) = (r.center() + a * scale, r.center() + b * scale);
            for (width, color) in [
                (8., Color::from_rgba(11, 13, 17, 255)),
                (4.2, Color::from_rgba(213, 99, 42, 255)),
                (1.4, Color::from_rgba(255, 210, 137, 255)),
            ] {
                draw_line(
                    a.x,
                    a.y,
                    b.x,
                    b.y,
                    width * scale,
                    opacity_color(color, fade * if width == 8. { 0.65 } else { 1. }),
                );
            }
        }
    }
}
