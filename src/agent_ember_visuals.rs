//! Shared elemental drawing primitives and the accepted laboratory; no gameplay RNG.
use macroquad::prelude::*;
use project_rl::world::GridPos;

#[derive(Clone, Debug)]
pub(super) struct Flight {
    pub origin: GridPos,
    pub target: GridPos,
    pub started: f64,
}
pub(super) const FLIGHT_SECONDS: f64 = 0.18;
pub(super) const LIFETIME_SECONDS: f64 = 0.42;
pub(super) fn draw(
    flights: &[Flight],
    map: &project_rl::world::Map,
    now: f64,
    reduced: bool,
    clip: Rect,
    rect: impl Fn(GridPos) -> Option<Rect>,
) {
    for flight in flights {
        let age = crate::animation_policy::age("FE01", now, flight.started);
        if !(0.0..LIFETIME_SECONDS).contains(&age) {
            continue;
        }
        let (Some(from), Some(to)) = (rect(flight.origin), rect(flight.target)) else {
            continue;
        };
        // Keep FE01-A02's shapes and deterministic sparks; common fast cadence
        // shortens presentation only. Body-attached Burn is drawn separately.
        let age = if reduced { 0.20 } else { age };
        let draw = || {
            elemental_projectile(
                1,
                from.center(),
                to.center(),
                to.w,
                (age * 0.55 / FLIGHT_SECONDS) as f32,
                0,
            )
        };
        if age >= FLIGHT_SECONDS {
            crate::effect_geometry::clipped(clip, clip, || {
                crate::natural_effect::on_floor(
                    Rect::new(to.x - to.w, to.y - to.h, to.w * 3., to.h * 3.),
                    flight.target,
                    to,
                    map,
                    &rect,
                    false,
                    draw,
                )
            });
        } else {
            draw();
        }
    }
}
pub(crate) fn alpha(mut c: Color, a: f32) -> Color {
    c.a = a.clamp(0.0, 1.0);
    c
}

pub(crate) fn noise(index: u32) -> f32 {
    let mut value = index.wrapping_add(19).wrapping_mul(2_654_435_761);
    value ^= value >> 16;
    value = value.wrapping_mul(2_246_822_519);
    value as f32 / u32::MAX as f32
}

pub(crate) fn glow(at: Vec2, radius: f32, color: Color, opacity: f32) {
    for (size, strength) in [(2.2, 0.05), (1.6, 0.08), (1.15, 0.13), (0.7, 0.22)] {
        draw_circle(at.x, at.y, radius * size, alpha(color, opacity * strength));
    }
}

pub(crate) fn luminous_line(from: Vec2, to: Vec2, width: f32, color: Color, opacity: f32) {
    for (size, strength) in [(5.0, 0.045), (2.8, 0.10), (1.4, 0.28), (0.6, 0.9)] {
        draw_line(
            from.x,
            from.y,
            to.x,
            to.y,
            width * size,
            alpha(color, opacity * strength),
        );
    }
}

pub(crate) fn shard(
    at: Vec2,
    direction: Vec2,
    length: f32,
    width: f32,
    color: Color,
    opacity: f32,
) {
    let normal = vec2(-direction.y, direction.x);
    let tip = at + direction * length * 0.65;
    let heel = at - direction * length * 0.35;
    let left = at + normal * width;
    let right = at - normal * width;
    draw_triangle(tip, left, heel, alpha(color, opacity));
    draw_triangle(
        tip,
        right,
        heel,
        alpha(
            Color::new(color.r * 0.45, color.g * 0.55, color.b * 0.60, 1.0),
            opacity,
        ),
    );
    draw_line(
        tip.x,
        tip.y,
        heel.x,
        heel.y,
        1.2,
        alpha(Color::new(0.9, 0.97, 1.0, 1.0), opacity * 0.8),
    );
}

pub(crate) fn elemental_projectile(
    skill: usize,
    player: Vec2,
    target: Vec2,
    tile: f32,
    time: f32,
    status: u16,
) {
    let fire = skill == 1;
    let color = if fire {
        Color::new(1.0, 0.34, 0.055, 1.0)
    } else {
        Color::new(0.20, 0.76, 1.0, 1.0)
    };
    let core = if fire {
        Color::new(1.0, 0.93, 0.60, 1.0)
    } else {
        Color::new(0.90, 0.99, 1.0, 1.0)
    };
    let (start, hit, direction) = if crate::effect_geometry::enabled() {
        crate::effect_geometry::projectile(player, target, tile)
    } else {
        let start = player + vec2(tile * 0.29, -tile * 0.06);
        let hit = target + vec2(0., -tile * 0.05);
        (start, hit, (hit - start).normalize_or_zero())
    };
    let normal = vec2(-direction.y, direction.x);
    let flight = 0.55;
    if time < 0.28 {
        let power = (time / 0.28 * std::f32::consts::PI).sin();
        glow(start, tile * 0.30, color, power);
        draw_circle_lines(
            start.x,
            start.y,
            tile * (0.08 + time * 0.3),
            tile * 0.022,
            alpha(core, power),
        );
        for n in 0..9 {
            let angle = n as f32 * std::f32::consts::TAU / 9.0 + time * 3.0;
            let point = start + vec2(angle.cos(), angle.sin()) * tile * (0.20 - time * 0.35);
            draw_circle(point.x, point.y, tile * 0.023 * power, alpha(core, power));
        }
    }
    if time < flight {
        let at = start.lerp(hit, time / flight);
        let tail_length = (tile * (0.55 + time * 0.70)).min((at - start).length() + tile * 0.15);
        for i in (0..28).rev() {
            let k = i as f32 / 28.0;
            let curl = (time * 20.0 - k * 6.0).sin() * tile * 0.035 * k;
            let point = at - direction * tail_length * k + normal * curl;
            let width = tile * (0.115 * (1.0 - k) + 0.01);
            glow(point, width, color, (1.0 - k) * 0.30);
            draw_circle(
                point.x,
                point.y,
                width * 0.65,
                alpha(color, (1.0 - k) * 0.60),
            );
            if i < 15 {
                draw_circle(
                    point.x,
                    point.y,
                    width * 0.23,
                    alpha(core, (1.0 - k) * 0.75),
                );
            }
        }
        for n in 0..26 {
            let k = noise(n);
            let cross = (noise(n + 31) - 0.5) * tile * 0.34 * k;
            let point = at - direction * tail_length * k + normal * cross;
            let opacity = (1.0 - k) * (0.4 + noise(n + 59) * 0.6);
            if fire {
                draw_circle(
                    point.x,
                    point.y,
                    tile * (0.018 + noise(n + 14) * 0.026),
                    alpha(color, opacity),
                );
            } else {
                shard(point, direction, tile * 0.12, tile * 0.018, color, opacity);
            }
        }
        glow(at, tile * 0.24, color, 1.0);
        if fire {
            let angle = if crate::effect_geometry::enabled() {
                direction.y.atan2(direction.x).to_degrees()
            } else {
                -1.
            };
            draw_ellipse(at.x, at.y, tile * 0.18, tile * 0.09, angle, color);
            let core_at = if crate::effect_geometry::enabled() {
                at + direction * tile * 0.035
            } else {
                vec2(at.x + direction.x * tile * 0.035, at.y)
            };
            draw_ellipse(
                core_at.x,
                core_at.y,
                tile * 0.125,
                tile * 0.045,
                angle,
                core,
            );
        } else {
            shard(at, direction, tile * 0.48, tile * 0.075, core, 1.0);
            luminous_line(
                at - direction * tile * 0.40,
                at + direction * tile * 0.28,
                tile * 0.042,
                core,
                1.0,
            );
        }
        return;
    }
    let impact = time - flight;
    let life = (1.0 - impact / 0.70).clamp(0.0, 1.0);
    if impact < 0.70 {
        glow(hit, tile * (0.34 + impact * 0.42), color, life);
        glow(hit, tile * 0.21, core, life * life);
        let ring = tile * (0.11 + impact * 0.82);
        draw_circle_lines(
            hit.x,
            hit.y,
            ring,
            tile * 0.034 * life,
            alpha(color, life * 0.6),
        );
        for n in 0..36 {
            let angle = noise(n + 220) * std::f32::consts::TAU;
            let vector = vec2(angle.cos(), angle.sin());
            let speed = 0.30 + noise(n + 90) * 0.70;
            let point = hit + vector * tile * (0.07 + impact * speed);
            if fire {
                let rise = vec2(0.0, -tile * impact * impact * 0.40);
                luminous_line(
                    point + rise - vector * tile * 0.09,
                    point + rise,
                    tile * 0.03,
                    color,
                    life,
                );
                draw_circle(
                    point.x + rise.x,
                    point.y + rise.y,
                    tile * 0.022 * life,
                    alpha(core, life),
                );
            } else {
                shard(
                    point,
                    vector,
                    tile * (0.15 + noise(n + 48) * 0.16) * life,
                    tile * 0.03 * life,
                    color,
                    life,
                );
            }
        }
        if !fire {
            for n in 0..7 {
                let angle = n as f32 * std::f32::consts::TAU / 7.0;
                let vector = vec2(angle.cos(), angle.sin());
                luminous_line(hit, hit + vector * tile * 0.42, tile * 0.018, core, life);
            }
        }
    }
    if status == 0 {
        return;
    }
    if fire {
        // Flames wrap the victim; these are status graphics, not a damage zone.
        glow(target, tile * 0.32, color, 0.45);
        for n in 0..11 {
            let phase = (time * 1.15 + noise(n + 160)).fract();
            let x = target.x + (noise(n + 33) - 0.5) * tile * 0.46;
            let base = vec2(x, target.y + tile * (0.27 - phase * 0.46));
            let height = tile * (0.16 + noise(n + 77) * 0.12) * (1.0 - phase);
            let sway = (time * 6.0 + n as f32).sin() * tile * 0.04;
            let top = base + vec2(sway, -height);
            let width = tile * (0.045 + noise(n + 49) * 0.028) * (1.0 - phase);
            draw_triangle(
                top,
                base - vec2(width, 0.0),
                base + vec2(width, 0.0),
                alpha(color, 0.65 * (1.0 - phase)),
            );
            draw_triangle(
                top + vec2(0.0, height * 0.27),
                base - vec2(width * 0.4, 0.0),
                base + vec2(width * 0.4, 0.0),
                alpha(core, 0.70 * (1.0 - phase)),
            );
        }
        for n in 0..10 {
            let phase = (time * 0.53 + noise(n + 80)).fract();
            let point = target + vec2((noise(n + 61) - 0.5) * tile * 0.54, -phase * tile * 0.64);
            draw_circle(
                point.x,
                point.y,
                tile * 0.019 * (1.0 - phase),
                alpha(core, 0.9 * (1.0 - phase)),
            );
        }
    } else {
        let feet = target + vec2(0.0, tile * 0.31);
        draw_ellipse(
            feet.x,
            feet.y,
            tile * 0.34,
            tile * 0.11,
            0.0,
            alpha(color, 0.22),
        );
        glow(feet, tile * 0.23, color, 0.28);
        for n in 0..7 {
            let x = feet.x + (n as f32 - 3.0) * tile * 0.09;
            let height = tile * (0.19 + noise(n + 24) * 0.18);
            shard(
                vec2(x, feet.y - height * 0.30),
                vec2((noise(n + 5) - 0.5) * 0.45, -1.0).normalize(),
                height,
                tile * 0.045,
                color,
                0.82,
            );
        }
        for n in 0..8 {
            let phase = (time * 0.4 + noise(n + 11)).fract();
            let point = feet + vec2((noise(n + 53) - 0.5) * tile * 0.55, -phase * tile * 0.45);
            draw_circle(
                point.x,
                point.y,
                tile * 0.013,
                alpha(core, (1.0 - phase) * 0.55),
            );
        }
    }
    draw_text(
        format!("{} · {}", if fire { "Brûlure" } else { "Gel" }, status),
        target.x - tile * 0.45,
        target.y - tile * 0.59,
        13.0,
        core,
    );
}
