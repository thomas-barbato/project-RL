//! Accepted drone bodies: identical sources for the map and inspection.
use super::{Visual, portrait::Sheet};
use macroquad::prelude::*;
use std::cell::OnceCell;
thread_local! {
    static SCOUT: OnceCell<Sheet> = const { OnceCell::new() };
    static TRACKER: OnceCell<Sheet> = const { OnceCell::new() };
    static GUNNER: OnceCell<Sheet> = const { OnceCell::new() };
    static GUARDIAN: OnceCell<Sheet> = const { OnceCell::new() };
}

fn with_source(visual: Visual, draw: impl FnOnce(&Sheet)) {
    let (cache, bytes, side): (_, &[u8], u16) = match visual {
        Visual::DroneScout => (
            &SCOUT,
            include_bytes!(
                "../assets/animations/agent/skill-animation-review-97/D56-BODY01-SIZE02-native128.png"
            ),
            128,
        ),
        Visual::DroneTracker => (
            &TRACKER,
            include_bytes!(
                "../assets/animations/agent/skill-animation-review-96/D58-BODY01-native128.png"
            ),
            128,
        ),
        Visual::DroneGunner => (
            &GUNNER,
            include_bytes!(
                "../assets/animations/agent/skill-animation-review-92/artilleur-body03/D59-BODY03-native64.png"
            ),
            64,
        ),
        Visual::DroneGuardian => (
            &GUARDIAN,
            include_bytes!(
                "../assets/animations/agent/skill-animation-review-94/gardien-size01/D66-BODY02-SIZE01-native128.png"
            ),
            128,
        ),
        _ => return,
    };
    cache.with(|cache| draw(cache.get_or_init(|| Sheet::load(bytes, side))));
}

pub(super) fn draw_portrait(rect: Rect, visual: Visual) {
    with_source(visual, |source| source.draw(rect, 0, 1.));
}

fn draw_world(rect: Rect, visual: Visual, opacity: f32) {
    with_source(visual, |source| {
        if visual == Visual::DroneScout && !crate::target_outline::capturing_body_mask() {
            draw_ellipse(
                rect.center().x,
                rect.center().y + rect.h * 30. / 64.,
                rect.w * 22. / 64.,
                rect.h * 3. / 64.,
                0.,
                Color::new(0., 0., 0., opacity * 0.3),
            );
        }
        let destination = if visual == Visual::DroneGunner {
            rect
        } else {
            Rect::new(
                rect.center().x - rect.w,
                rect.center().y - rect.h,
                rect.w * 2.,
                rect.h * 2.,
            )
        };
        crate::world_readability::texture(
            &source.texture,
            destination.x,
            destination.y,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                dest_size: Some(destination.size()),
                ..Default::default()
            },
        );
    });
}
pub(super) fn draw_scout(rect: Rect, opacity: f32) {
    draw_world(rect, Visual::DroneScout, opacity);
}
pub(super) fn draw_tracker(rect: Rect, opacity: f32) {
    draw_world(rect, Visual::DroneTracker, opacity);
}
pub(super) fn draw_gunner(rect: Rect, opacity: f32) {
    draw_world(rect, Visual::DroneGunner, opacity);
}
pub(super) fn draw_guardian(rect: Rect, opacity: f32) {
    draw_world(rect, Visual::DroneGuardian, opacity);
}
