//! FE05-A07 projectile/explosion and approved ANIM-OVER02 body-attached Burn.
use macroquad::prelude::*;
use project_rl::world::{GridPos, Map, has_line_of_sight};
use std::cell::RefCell;
#[derive(Clone, Debug)]
pub(super) struct Fireball {
    pub origin: GridPos,
    pub target: GridPos,
    pub radius: u16,
    pub started: f64,
}
struct Art {
    ball: Texture2D,
    blast: Texture2D,
    flame: Texture2D,
}
thread_local! {static ART:RefCell<Option<Art>>=const{RefCell::new(None)};}
impl Art {
    fn load() -> Self {
        let texture = |bytes: &[u8]| {
            let t = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        Self {
            ball: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-16/projectile-atlas.png"
            )),
            blast: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-20/burst-atlas.png"
            )),
            flame: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-06/flame-atlas.png"
            )),
        }
    }
}
pub(super) fn draw(
    flights: &[Fireball],
    map: &Map,
    now: f64,
    reduced: bool,
    background: bool,
    rect: impl Fn(GridPos) -> Option<Rect>,
) {
    if flights.is_empty() {
        return;
    }
    ART.with_borrow_mut(|cache| {
        let art = cache.get_or_insert_with(Art::load);
        for flight in flights {
            let age = (now - flight.started) as f32;
            if !(0.0..0.90).contains(&age) {
                continue;
            }
            let (Some(from), Some(to)) = (rect(flight.origin), rect(flight.target)) else {
                continue;
            };
            let scale = to.w / 64.;
            let age = if reduced { 0.55 } else { age };
            if age < 0.35 {
                if background {
                    continue;
                }
                let origin = from.center() - vec2(0., 8. * scale);
                let target = to.center() - vec2(0., 8. * scale);
                let heading = target - origin;
                let at = origin + heading * (age / 0.35);
                let size = 96. * scale;
                let anchor = vec2(190., 132.) * size / 256.;
                let frames = [0, 1, 2, 3, 2, 1, 0, 1];
                let frame = frames[((age * 8.) as usize) % 8];
                draw_texture_ex(
                    &art.ball,
                    at.x - anchor.x,
                    at.y - anchor.y,
                    Color::new(1., 1., 1., 0.9),
                    DrawTextureParams {
                        source: Some(Rect::new(frame as f32 * 256., 0., 256., 256.)),
                        dest_size: Some(vec2(size, size)),
                        rotation: heading.y.atan2(heading.x),
                        pivot: Some(at),
                        ..Default::default()
                    },
                );
            } else if background {
                let t = age - 0.35;
                let frame = ((t / 0.55 * 6.) as usize).min(5);
                let size =
                    320. * scale * f32::from(flight.radius.saturating_mul(2).saturating_add(1))
                        / 7.;
                let at = to.center();
                let whole = Rect::new(
                    at.x - 132. * size / 256.,
                    at.y - 146. * size / 256.,
                    size,
                    size,
                );
                let extent = i32::from(flight.radius) + 3;
                for y in (flight.target.y - extent).max(0)
                    ..=(flight.target.y + extent).min(map.height() as i32 - 1)
                {
                    for x in (flight.target.x - extent).max(0)
                        ..=(flight.target.x + extent).min(map.width() as i32 - 1)
                    {
                        let p = GridPos::new(x, y);
                        let Some(r) = rect(p) else {
                            continue;
                        };
                        if !map.is_walkable(p) || !has_line_of_sight(map, flight.target, p, true) {
                            continue;
                        }
                        let Some(crop) = whole.intersect(r) else {
                            continue;
                        };
                        if crop.w <= 0. || crop.h <= 0. {
                            continue;
                        }
                        draw_texture_ex(
                            &art.blast,
                            crop.x,
                            crop.y,
                            Color::new(1., 1., 1., 0.95 * ((0.90 - age) / 0.16).clamp(0., 1.)),
                            DrawTextureParams {
                                source: Some(Rect::new(
                                    frame as f32 * 256. + (crop.x - whole.x) * 256. / size,
                                    (crop.y - whole.y) * 256. / size,
                                    crop.w * 256. / size,
                                    crop.h * 256. / size,
                                )),
                                dest_size: Some(vec2(crop.w, crop.h)),
                                ..Default::default()
                            },
                        );
                    }
                }
            }
        }
    });
}
pub(super) fn burn(rect: Rect, now: f64, seed: u64, reduced: bool, background: bool) {
    ART.with_borrow_mut(|cache| {
        let art = cache.get_or_insert_with(Art::load);
        let at = rect.center();
        let tile = rect.w / 64.;
        let clock = if reduced { 0.2 } else { now as f32 * (2. / 3.) };
        let plumes = if background {
            [(-9., 19., 0.33, 0.90, 0u64), (10., 19., 0.27, 0.90, 3)]
        } else {
            [(-7., 20., 0.29, 0.85, 1u64), (7., 19., 0.24, 0.70, 4)]
        };
        for (x, y, scale, opacity, phase) in plumes {
            let frame = ((clock.max(0.) * 12.) as u64)
                .wrapping_add(seed)
                .wrapping_add(phase)
                % 8;
            let s = scale * tile;
            let base = at + vec2(x * tile, y * tile);
            draw_texture_ex(
                &art.flame,
                base.x - 64. * s,
                base.y - 120. * s,
                Color::new(1., 1., 1., opacity),
                DrawTextureParams {
                    source: Some(Rect::new(frame as f32 * 128., 0., 128., 128.)),
                    dest_size: Some(vec2(128. * s, 128. * s)),
                    ..Default::default()
                },
            );
        }
    });
}
