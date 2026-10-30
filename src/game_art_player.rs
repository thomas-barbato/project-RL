//! Human wireframe and the posed, interchangeable player artwork.
use super::*;
#[cfg(all(debug_assertions, test))]
pub(super) fn vertical_review_weapon(
    model: u16,
    left: bool,
    color: EquipmentPalette,
) -> Option<(Image, [f32; 2], Option<[f32; 2]>)> {
    worn::vertical_review_weapon(model, left, color)
}

#[cfg(debug_assertions)]
pub(super) fn vertical_review_composition(
    source: &Image,
    visual: Visual,
    palms: [[f32; 2]; 2],
    back: bool,
) -> Option<Image> {
    worn::vertical_review_composition(source, visual, palms, back)
}
#[cfg(debug_assertions)]
pub(super) fn vertical_review_standing(visual: Visual) -> Option<Image> {
    worn::vertical_review_standing(visual)
}
#[cfg(debug_assertions)]
pub(super) fn vertical_review_standing_head(
    visual: Visual,
    apply: impl FnOnce(&mut Image),
) -> Option<Image> {
    worn::vertical_review_standing_head(visual, apply)
}
#[path = "game_art_worn.rs"]
mod worn;
pub(super) fn approved_walking_image(
    body: u8,
    equipment: [Option<u16>; 4],
    weapon: Option<u16>,
    hands: Option<[Option<u16>; 2]>,
    colors: [EquipmentPalette; 3],
    pose: crate::game_art::walking::Pose,
) -> Option<Image> {
    worn::approved_walking_image(body, equipment, weapon, hands, colors, pose)
}
pub(super) fn outfit_index(body: u8, equipment: [Option<u16>; 4]) -> u16 {
    worn::outfit_index(body, equipment)
}
#[cfg(debug_assertions)]
pub(super) fn draw_finish_review(
    rect: Rect,
    body: u16,
    weapon: Option<u16>,
    hands: Option<[Option<u16>; 2]>,
    colors: [EquipmentPalette; 3],
) {
    worn::draw_finish_review(rect, body, weapon, hands, colors);
}

pub(super) fn prewarm() {
    worn::prewarm();
}

#[cfg(debug_assertions)]
pub(super) fn cache_stats() -> (usize, usize) {
    worn::cache_stats()
}

#[cfg(debug_assertions)]
pub(super) fn draw_pistol_reference(rect: Rect, body: u8, equipment: [Option<u16>; 4], side: u16) {
    worn::draw_pistol_reference(rect, body, equipment, side);
}
#[cfg(debug_assertions)]
pub(super) fn draw_grip_review(
    rect: Rect,
    body: u16,
    weapon: Option<u16>,
    hands: Option<[Option<u16>; 2]>,
) {
    worn::draw_grip_review(rect, body, weapon, hands);
}
#[cfg(debug_assertions)]
pub(super) fn draw_grip_review_at_time(
    rect: Rect,
    body: u16,
    weapon: Option<u16>,
    hands: Option<[Option<u16>; 2]>,
    time: Option<f64>,
) {
    worn::draw_grip_review_at_time(rect, body, weapon, hands, time);
}

pub(super) fn draw_player(
    rect: Rect,
    body: u8,
    equipment: [Option<u16>; 4],
    weapon: Option<u16>,
    hands: Option<[Option<u16>; 2]>,
    palettes: [EquipmentPalette; 3],
    side: u16,
    opacity: f32,
    idle_time: Option<f64>,
) {
    worn::draw(
        rect, body, equipment, weapon, hands, palettes, side, opacity, idle_time,
    );
}

pub(super) fn draw_npc(
    rect: Rect,
    body: u8,
    weapon: Option<u16>,
    side: u16,
    opacity: f32,
    idle_time: Option<f64>,
) {
    worn::draw_npc(rect, body, weapon, side, opacity, idle_time);
}

pub(super) fn draw_wireframe(rect: Rect, equipment: [Option<u16>; 4], color: Color) {
    let point = |x: f32, y: f32| vec2(rect.x + x * rect.w / 32.0, rect.y + y * rect.h / 32.0);
    let edge = |from: (f32, f32), to: (f32, f32), opacity: f32| {
        let a = point(from.0, from.1);
        let b = point(to.0, to.1);
        draw_line(
            a.x,
            a.y,
            b.x,
            b.y,
            1.2,
            Color::new(color.r, color.g, color.b, opacity),
        );
    };
    // Human proportions, soft contours and surface rings; every face stays hollow.
    let contour = |points: &[(f32, f32)], opacity: f32| {
        let n = points.len();
        for i in 0..n {
            let p0 = vec2(points[(i + n - 1) % n].0, points[(i + n - 1) % n].1);
            let p1 = vec2(points[i].0, points[i].1);
            let p2 = vec2(points[(i + 1) % n].0, points[(i + 1) % n].1);
            let p3 = vec2(points[(i + 2) % n].0, points[(i + 2) % n].1);
            let mut previous = p1;
            for step in 1..=6 {
                let t = step as f32 / 6.0;
                let next = 0.5
                    * ((2.0 * p1)
                        + (-p0 + p2) * t
                        + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * t * t
                        + (-p0 + 3.0 * p1 - 3.0 * p2 + p3) * t * t * t);
                edge((previous.x, previous.y), (next.x, next.y), opacity);
                previous = next;
            }
        }
    };
    contour(
        &[
            (16.0, 0.8),
            (17.5, 1.2),
            (18.1, 2.5),
            (17.8, 3.8),
            (17.1, 4.8),
            (16.0, 5.2),
            (14.9, 4.8),
            (14.2, 3.8),
            (13.9, 2.5),
            (14.5, 1.2),
        ],
        0.94,
    );
    contour(
        &[
            (14.9, 4.8),
            (14.4, 6.1),
            (12.2, 6.9),
            (10.4, 7.3),
            (9.3, 8.3),
            (8.7, 10.3),
            (8.3, 12.5),
            (7.5, 15.2),
            (7.3, 17.3),
            (6.9, 18.5),
            (7.4, 19.8),
            (8.2, 20.0),
            (8.9, 18.5),
            (9.0, 17.4),
            (9.8, 15.2),
            (10.2, 13.3),
            (10.8, 10.7),
            (11.5, 11.8),
            (12.3, 14.2),
            (12.0, 15.8),
            (11.3, 17.8),
            (11.4, 20.3),
            (11.3, 22.4),
            (11.5, 24.5),
            (11.3, 26.6),
            (11.8, 29.2),
            (10.7, 30.3),
            (10.5, 31.0),
            (14.1, 31.0),
            (14.4, 30.4),
            (14.0, 29.3),
            (14.5, 26.4),
            (14.4, 24.5),
            (14.8, 22.0),
            (15.5, 19.2),
            (16.0, 18.6),
            (16.5, 19.2),
            (17.2, 22.0),
            (17.6, 24.5),
            (17.5, 26.4),
            (18.0, 29.3),
            (17.6, 30.4),
            (17.9, 31.0),
            (21.5, 31.0),
            (21.3, 30.3),
            (20.2, 29.2),
            (20.7, 26.6),
            (20.5, 24.5),
            (20.7, 22.4),
            (20.6, 20.3),
            (20.7, 17.8),
            (20.0, 15.8),
            (19.7, 14.2),
            (20.5, 11.8),
            (21.2, 10.7),
            (21.8, 13.3),
            (22.2, 15.2),
            (23.0, 17.4),
            (23.1, 18.5),
            (23.8, 20.0),
            (24.6, 19.8),
            (25.1, 18.5),
            (24.7, 17.3),
            (24.5, 15.2),
            (23.7, 12.5),
            (23.3, 10.3),
            (22.7, 8.3),
            (21.6, 7.3),
            (19.8, 6.9),
            (17.6, 6.1),
            (17.1, 4.8),
        ],
        0.94,
    );
    // Curved cross-sections show the chest, waist, pelvis and joints.
    for points in [
        vec![(14.1, 2.5), (16.0, 2.1), (17.9, 2.5), (16.0, 2.9)],
        vec![(14.5, 3.7), (16.0, 3.4), (17.5, 3.7), (16.0, 4.0)],
        vec![(12.1, 7.2), (16.0, 7.6), (19.9, 7.2), (16.0, 6.5)],
        vec![(11.2, 10.2), (16.0, 11.0), (20.8, 10.2), (16.0, 9.1)],
        vec![(12.3, 14.2), (16.0, 14.7), (19.7, 14.2), (16.0, 13.6)],
        vec![(11.7, 16.7), (16.0, 17.5), (20.3, 16.7), (16.0, 15.6)],
        vec![(8.3, 12.5), (9.2, 13.0), (10.2, 13.3), (9.3, 12.1)],
        vec![(21.8, 13.3), (22.8, 13.0), (23.7, 12.5), (22.7, 12.1)],
        vec![(11.5, 24.5), (13.0, 24.9), (14.4, 24.5), (13.0, 24.1)],
        vec![(17.6, 24.5), (19.0, 24.9), (20.5, 24.5), (19.0, 24.1)],
    ] {
        contour(&points, 0.34);
    }
    for (a, b) in [
        ((16.0, 0.8), (16.0, 5.2)),
        ((12.2, 6.9), (16.0, 9.1)),
        ((19.8, 6.9), (16.0, 9.1)),
        ((16.0, 7.6), (16.0, 17.5)),
        ((11.2, 10.2), (16.0, 14.7)),
        ((20.8, 10.2), (16.0, 14.7)),
        ((12.3, 14.2), (16.0, 17.5)),
        ((19.7, 14.2), (16.0, 17.5)),
        ((11.7, 16.7), (15.5, 19.2)),
        ((20.3, 16.7), (16.5, 19.2)),
        ((10.4, 7.3), (9.3, 12.1)),
        ((9.3, 12.1), (8.2, 18.8)),
        ((21.6, 7.3), (22.7, 12.1)),
        ((22.7, 12.1), (23.8, 18.8)),
        ((11.3, 17.8), (13.0, 24.1)),
        ((15.5, 19.2), (13.0, 24.1)),
        ((13.0, 24.9), (12.8, 29.5)),
        ((20.7, 17.8), (19.0, 24.1)),
        ((16.5, 19.2), (19.0, 24.1)),
        ((19.0, 24.9), (19.2, 29.5)),
    ] {
        edge(a, b, 0.36);
    }
    if equipment[1].is_some() {
        edge((14.1, 2.7), (17.9, 2.7), 0.82);
    }
    if equipment[0].is_some() {
        contour(
            &[
                (12.1, 8.0),
                (16.0, 8.8),
                (19.9, 8.0),
                (19.3, 12.8),
                (16.0, 13.7),
                (12.7, 12.8),
            ],
            0.72,
        );
    }
    if equipment[2].is_some() {
        edge((7.3, 17.3), (9.0, 17.4), 0.82);
        edge((23.0, 17.4), (24.7, 17.3), 0.82);
    }
    if equipment[3].is_some() {
        edge((11.4, 28.0), (14.2, 28.0), 0.82);
        edge((17.8, 28.0), (20.6, 28.0), 0.82);
    }
}
