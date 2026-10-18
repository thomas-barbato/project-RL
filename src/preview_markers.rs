//! MARKER-PULSE-02: centered, progressive fills for small preview markers.
//! Numbered movement steps retain their static WALK-VIS-03-E rendering.
use macroquad::prelude::*;
use std::cell::Cell;

thread_local! {
    static BRIGHTNESS: Cell<f32> = const { Cell::new(1.) };
    #[cfg(debug_assertions)]
    static REVIEW_PHASE: Cell<Option<f32>> = const { Cell::new(None) };
}

pub fn begin_frame(now: f64, reduced_motion: bool) {
    let brightness = if reduced_motion {
        1.
    } else {
        ((1. - (now * std::f64::consts::TAU / 1.1).cos()) * 0.5) as f32
    };
    BRIGHTNESS.with(|value| value.set(brightness));
}

pub fn circle(center: Vec2, radius: f32, stroke: f32, base: Color, color: Color) {
    marker(center, radius, stroke, base, color, 64);
}

pub fn diamond(center: Vec2, radius: f32, stroke: f32, base: Color, color: Color) {
    marker(center, radius, stroke, base, color, 4);
}

fn marker(center: Vec2, radius: f32, stroke: f32, base: Color, color: Color, sides: u8) {
    let brightness = BRIGHTNESS.with(Cell::get);
    #[cfg(debug_assertions)]
    let brightness = REVIEW_PHASE.with(Cell::get).unwrap_or(brightness);
    // The hole, fill and outline share their center, radius and subdivision.
    let center = vec2(center.x.floor() + 0.5, center.y.floor() + 0.5);
    draw_poly(center.x, center.y, sides, radius, 0., base);
    draw_poly(
        center.x,
        center.y,
        sides,
        radius,
        0.,
        Color::new(color.r, color.g, color.b, 0.16 + brightness * 0.79),
    );
    draw_poly_lines(center.x, center.y, sides, radius, 0., stroke, color);
}

#[cfg(debug_assertions)]
pub struct ReviewGuard(Option<f32>);

#[cfg(debug_assertions)]
impl Drop for ReviewGuard {
    fn drop(&mut self) {
        REVIEW_PHASE.with(|phase| phase.set(self.0));
    }
}

#[cfg(debug_assertions)]
pub fn review_phase(value: Option<f32>) -> ReviewGuard {
    ReviewGuard(REVIEW_PHASE.with(|phase| phase.replace(value)))
}
