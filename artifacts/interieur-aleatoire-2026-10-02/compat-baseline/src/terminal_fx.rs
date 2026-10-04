//! Cell-clipped, code-native terminal effects. All motion comes from the cue
//! sample; this renderer has no clock, RNG, actor or simulation access.
use super::{TerminalGlyphPalette, draw_entity};
use crate::visual_effects::{EFFECT_NEIGHBORS, TerminalEffectFamily, TerminalEffectSample};
use macroquad::prelude::*;

#[path = "terminal_elements.rs"]
mod elements;

pub(super) fn draw(rect: Rect, effect: TerminalEffectSample, occupied: bool) {
    match effect.family {
        TerminalEffectFamily::Buckshot => buckshot(rect, effect, occupied),
        TerminalEffectFamily::Ballistic
        | TerminalEffectFamily::Rocket
        | TerminalEffectFamily::Grenade => ordnance(rect, effect),
        TerminalEffectFamily::Blast => blast(rect, effect),
        TerminalEffectFamily::Signal => signal(rect, effect),
        TerminalEffectFamily::Ricochet => ricochet(rect, effect),
        TerminalEffectFamily::Alternation => alternation(rect, effect),
        TerminalEffectFamily::Echo => echo(rect, effect),
        TerminalEffectFamily::Fracture => fracture(rect, effect),
        TerminalEffectFamily::Catalysis
        | TerminalEffectFamily::FlameJet
        | TerminalEffectFamily::Flame
        | TerminalEffectFamily::Corrosion
        | TerminalEffectFamily::Caustic
        | TerminalEffectFamily::Frost => elements::draw(rect, effect, occupied),
        TerminalEffectFamily::ImpactWave | TerminalEffectFamily::BearerWave => wave(rect, effect),
        TerminalEffectFamily::Conduction => conduction(rect, effect),
        TerminalEffectFamily::Resonance => resonance(rect, effect),
        TerminalEffectFamily::Restoration => restoration(rect, effect),
        TerminalEffectFamily::Piercing => piercing(rect, effect),
        TerminalEffectFamily::Impulse | TerminalEffectFamily::ImpulseBlocked => {
            impulse(rect, effect)
        }
        TerminalEffectFamily::Guard | TerminalEffectFamily::GuardBreak => guard(rect, effect),
        TerminalEffectFamily::Glyph => draw_entity(
            rect,
            effect.symbol,
            TerminalGlyphPalette::new(effect.color, effect.accent_color, effect.highlight_color),
            false,
            false,
            None,
        ),
    }
}

fn faded(mut color: Color, strength: f32) -> Color {
    color.a *= strength.clamp(0.0, 1.0);
    color
}

fn ordnance(rect: Rect, effect: TerminalEffectSample) {
    let mut axis = vec2(effect.direction.0, effect.direction.1).normalize_or_zero();
    if axis == Vec2::ZERO {
        axis = vec2(1.0, 0.0);
    }
    let tangent = vec2(-axis.y, axis.x);
    let (x, y) = effect.anchor.unwrap_or((0.5, 0.5));
    let head = vec2(x, y);
    let light = effect.highlight_color.unwrap_or(effect.color);
    let dark = effect.accent_color.unwrap_or(effect.color);
    match effect.family {
        TerminalEffectFamily::Ballistic => {
            stroke(rect, head - axis * 0.18, head, 1.4, dark);
            stroke(rect, head - axis * 0.06, head + axis * 0.06, 1.8, light);
        }
        TerminalEffectFamily::Rocket => {
            for index in (1..=9).rev() {
                let age = index as f32 / 9.0;
                pixel(
                    rect,
                    head - axis * (0.18 + age * 0.55)
                        + tangent * (age * 13.0 + effect.progress * 19.0).sin() * age * 0.09,
                    1.2 + age * 3.0,
                    faded(dark, (1.0 - age) * 0.6),
                );
            }
            // A single exhaust flame, behind a single metal body.
            stroke(
                rect,
                head - axis * 0.40,
                head - axis * 0.13,
                2.4,
                faded(light, 0.75),
            );
            stroke(
                rect,
                head - axis * 0.24,
                head - axis * 0.11,
                1.4,
                Color::from_rgba(255, 244, 195, 255),
            );
            stroke(
                rect,
                head - axis * 0.13,
                head + axis * 0.11,
                3.4,
                effect.color,
            );
            stroke(
                rect,
                head - axis * 0.07 + tangent * 0.09,
                head - axis * 0.07 - tangent * 0.09,
                1.2,
                dark,
            );
            stroke(
                rect,
                head + axis * 0.09,
                head + axis * 0.20,
                2.0,
                effect.color,
            );
            stroke(
                rect,
                head - tangent * 0.055 - axis * 0.08,
                head - tangent * 0.055 + axis * 0.10,
                0.8,
                WHITE,
            );
        }
        _ => {
            let spin = effect.progress * 12.0;
            let seam = vec2(spin.cos(), spin.sin()) * 0.1;
            pixel(rect, head + vec2(0.02, 0.02), 4.2, dark);
            pixel(rect, head, 3.2, effect.color);
            stroke(rect, head - seam, head + seam, 0.8, dark);
            pixel(rect, head - vec2(0.04, 0.055), 1.1, light);
            pixel(rect, head + axis * 0.13, 1.1, light);
        }
    }
}

fn blast(rect: Rect, effect: TerminalEffectSample) {
    let progress = effect.progress.clamp(0.0, 1.0);
    let light = effect.highlight_color.unwrap_or(effect.color);
    let dark = effect.accent_color.unwrap_or(effect.color);
    let (x, y) = effect.anchor.unwrap_or((0.5, 0.5));
    let center = vec2(x, y);
    let radius = effect.radius.max(0.5);
    let growth = (progress * 4.5).min(1.0);
    let fire_radius = radius * (0.15 + growth * 0.72) * (1.0 - progress * 0.30);
    // Rasterize one billowing fireball in world coordinates. Each tile clips
    // its part, so there is one epicenter even on diagonals or behind cover.
    let side = rect.w.min(rect.h).round().clamp(8.0, 24.0) as usize;
    for row in 0..side {
        for column in 0..side {
            let p = vec2(
                (column as f32 + 0.5) / side as f32,
                (row as f32 + 0.5) / side as f32,
            );
            let delta = p - center;
            let distance = delta.length();
            let angle = delta.y.atan2(delta.x);
            let billow = 1.0
                + (angle * 7.0 + progress * 4.0).sin() * 0.12
                + (angle * 11.0 - progress * 7.0).sin() * 0.07;
            let texture = (delta.x * 19.0 + (delta.y * 12.0).sin() + progress * 9.0).sin()
                * (delta.y * 17.0 - progress * 12.0).cos();
            let smoke_distance = (delta + vec2(0.0, progress * 0.22)).length();
            let smoke_edge = radius * (0.35 + progress * 0.75) * billow;
            let mut tone = None;
            if progress > 0.25 && smoke_distance < smoke_edge && texture > -0.55 {
                tone = Some(faded(dark, (0.35 + texture * 0.15) * (1.0 - progress)));
            }
            if progress < 0.72 && distance < fire_radius * billow {
                let heat =
                    1.0 - distance / (fire_radius * billow) + texture * 0.18 - progress * 0.65;
                let color = if heat > 0.48 {
                    light
                } else if heat > 0.12 {
                    effect.color
                } else {
                    Color::new(0.86, 0.23, 0.055, effect.color.a)
                };
                tone = Some(faded(color, (1.0 - progress / 0.78).min(0.85)));
            }
            let shock_radius = radius * (progress * 3.4).min(1.12);
            if progress < 0.34 && (distance - shock_radius).abs() < 0.035 {
                tone = Some(faded(light, (1.0 - progress / 0.34) * 0.8));
            }
            if let Some(color) = tone {
                draw_rectangle(
                    rect.x + column as f32 * rect.w / side as f32,
                    rect.y + row as f32 * rect.h / side as f32,
                    rect.w / side as f32,
                    rect.h / side as f32,
                    color,
                );
            }
        }
    }
    for i in 0..13 {
        let angle = i as f32 * 2.39996;
        let axis = vec2(angle.cos(), angle.sin());
        let reach = radius * (0.22 + progress * (1.05 + (i % 3) as f32 * 0.12));
        let head = center + axis * reach + vec2(0.0, progress * progress * 0.28);
        stroke(
            rect,
            head - axis * (0.04 + (1.0 - progress) * 0.13),
            head,
            1.3,
            faded(
                if i % 3 == 0 { light } else { effect.color },
                1.0 - progress,
            ),
        );
    }
}

fn buckshot(rect: Rect, effect: TerminalEffectSample, occupied: bool) {
    let mut axis = vec2(effect.direction.0, effect.direction.1).normalize_or_zero();
    if axis == Vec2::ZERO {
        axis = vec2(1.0, 0.0);
    }
    let tangent = vec2(-axis.y, axis.x);
    let light = effect.highlight_color.unwrap_or(effect.color);
    let dark = effect.accent_color.unwrap_or(effect.color);
    let progress = effect.progress.clamp(0.0, 1.0);
    let stagger = f32::from(effect.variant % 7) * 0.012;
    // Separate metal grains, never a solid trail, ray or flame. Each visible
    // cone cell receives the bearer-to-cell direction, so the volley fans out.
    for (index, lateral) in [-0.27_f32, 0.02, 0.29].into_iter().enumerate() {
        let travel = (progress * 1.45 - index as f32 * 0.07 - stagger).clamp(0.0, 1.0);
        let point = vec2(0.5, 0.5) + axis * (travel - 0.5) + tangent * (lateral + stagger);
        pixel(rect, point, 2.1, faded(dark, 0.7));
        pixel(
            rect,
            point,
            1.25,
            if index == 1 { light } else { effect.color },
        );
    }
    // Small peripheral impact flecks leave the actor's glyph readable.
    // Purely visual: the shot is still one hit per affected actor, not pellets
    // independently rolling damage, affixes or ammunition consumption.
    if occupied && progress > 0.55 {
        let spread = 0.18 + (progress - 0.55) * 0.35;
        for side in [-1.0, 1.0] {
            pixel(
                rect,
                vec2(0.5, 0.5) + tangent * side * spread - axis * 0.08,
                1.3,
                light,
            );
        }
    }
}

fn signal(rect: Rect, effect: TerminalEffectSample) {
    let axis = vec2(effect.direction.0, effect.direction.1).normalize_or_zero();
    let tangent = vec2(-axis.y, axis.x);
    let light = effect.highlight_color.unwrap_or(effect.color);
    let accent = effect.accent_color.unwrap_or(effect.color);
    // Three little packets, not lightning, an attack trail or an area blast.
    let head = vec2(0.5, 0.5) + axis * (effect.progress * 0.65 - 0.25);
    for index in 0..3 {
        let p = head - axis * (index as f32 * 0.22);
        pixel(
            rect,
            p,
            if index == 0 { 2.5 } else { 1.5 },
            if index == 0 { light } else { accent },
        );
    }
    stroke(
        rect,
        head + tangent * 0.13 - axis * 0.07,
        head + axis * 0.08,
        1.0,
        effect.color,
    );
    stroke(
        rect,
        head - tangent * 0.13 - axis * 0.07,
        head + axis * 0.08,
        1.0,
        effect.color,
    );
}

fn ricochet(rect: Rect, effect: TerminalEffectSample) {
    let mut axis = vec2(effect.direction.0, effect.direction.1).normalize_or_zero();
    if axis == Vec2::ZERO {
        axis = vec2(1.0, 0.0);
    }
    let tangent = vec2(-axis.y, axis.x);
    let head = vec2(0.5, 0.5) + axis * (effect.progress * 0.85 - 0.35);
    let light = effect.highlight_color.unwrap_or(effect.color);
    let accent = effect.accent_color.unwrap_or(effect.color);
    stroke(rect, head - axis * 0.40, head, 1.0, effect.color);
    stroke(rect, head - axis * 0.15, head, 2.0, accent);
    pixel(rect, head, 2.0, light);
    for side in [-1.0, 1.0] {
        pixel(
            rect,
            head - axis * 0.24 + tangent * side * (0.08 + effect.progress * 0.08),
            1.0,
            accent,
        );
    }
}

fn alternation(rect: Rect, effect: TerminalEffectSample) {
    let mut axis = vec2(effect.direction.0, effect.direction.1).normalize_or_zero();
    if axis == Vec2::ZERO {
        axis = vec2(1.0, 0.0);
    }
    let normal = vec2(-axis.y, axis.x);
    let light = effect.highlight_color.unwrap_or(effect.color);
    let accent = effect.accent_color.unwrap_or(effect.color);
    // Two interwoven, travelling ribbons, unlike the straight piercing bolt.
    for side in [-1.0, 1.0] {
        let mut previous = None;
        for step in 0..=12 {
            let t = step as f32 / 12.0;
            let wave = (t * std::f32::consts::TAU - effect.progress * std::f32::consts::TAU).sin();
            let point = vec2(0.5, 0.5) + axis * (t - 0.5) + normal * (side * wave * 0.14);
            if let Some(from) = previous {
                stroke(
                    rect,
                    from,
                    point,
                    1.0,
                    if side > 0.0 { accent } else { effect.color },
                );
            }
            previous = Some(point);
        }
    }
    let head = vec2(0.5, 0.5) + axis * (effect.progress - 0.5);
    pixel(rect, head, 2.0, light);
}

fn echo(rect: Rect, effect: TerminalEffectSample) {
    let light = effect.highlight_color.unwrap_or(effect.color);
    let accent = effect.accent_color.unwrap_or(effect.color);
    let p = effect.progress;
    // Quiet paired brackets breathe while waiting. At the deadline they fold
    // inward, then a crisp cross strikes the cell. No fire or false splash.
    let radius = if effect.sustained {
        0.29 + 0.035 * (p * std::f32::consts::TAU).sin()
    } else if p < 0.5 {
        0.36 * (1.0 - p * 1.65)
    } else {
        0.08 + (p - 0.5) * 0.55
    };
    for side in [-1.0, 1.0] {
        let x = 0.5 + side * radius;
        stroke(rect, vec2(x, 0.28), vec2(x, 0.72), 1.5, accent);
        for y in [0.28, 0.72] {
            stroke(rect, vec2(x, y), vec2(x - side * 0.10, y), 1.0, light);
        }
    }
    if !effect.sustained && p > 0.40 {
        let r = 0.12 + p * 0.30;
        for side in [-1.0, 1.0] {
            stroke(
                rect,
                vec2(0.5 - r, 0.5 - r * side),
                vec2(0.5 + r, 0.5 + r * side),
                1.5,
                light,
            );
        }
    }
}

fn fracture(rect: Rect, effect: TerminalEffectSample) {
    // Angular plates split and fly apart, not flames, rings or electric forks.
    // Everything stays on the shared cell-clipped pixel grid.
    let p = effect.progress;
    let light = effect.highlight_color.unwrap_or(effect.color);
    let accent = effect.accent_color.unwrap_or(effect.color);
    let outward = vec2(effect.direction.0, effect.direction.1).normalize_or_zero();
    let center_cell = outward == Vec2::ZERO;
    let count = if center_cell { 6 } else { 4 };
    for index in 0..count {
        let angle = if center_cell {
            (index as f32 + f32::from(effect.variant % 3) * 0.13) * std::f32::consts::TAU / 6.0
        } else {
            outward.y.atan2(outward.x) + (index as f32 - 1.5) * 0.52
        };
        let axis = vec2(angle.cos(), angle.sin());
        let tangent = vec2(-axis.y, axis.x);
        let center = vec2(0.5, 0.5) - outward * 0.30 + axis * (0.10 + p * 0.55);
        let length =
            (0.07 + ((usize::from(effect.variant) + index) % 3) as f32 * 0.035) * (1.0 - p * 0.45);
        stroke(
            rect,
            center - axis * length,
            center + axis * length,
            2.5,
            accent,
        );
        stroke(
            rect,
            center - axis * length + tangent * 0.06,
            center + tangent * 0.06,
            1.0,
            light,
        );
        pixel(
            rect,
            center + axis * length + tangent * 0.045,
            1.0,
            effect.color,
        );
        if p > 0.35 {
            pixel(rect, center - axis * 0.14 - tangent * 0.10, 1.0, accent);
        }
    }
}

fn impulse(rect: Rect, effect: TerminalEffectSample) {
    let direction = vec2(effect.direction.0, effect.direction.1).normalize_or_zero();
    let normal = vec2(-direction.y, direction.x);
    let blocked = effect.family == TerminalEffectFamily::ImpulseBlocked;
    let light = effect.highlight_color.unwrap_or(effect.color);
    let accent = effect.accent_color.unwrap_or(effect.color);
    let p = effect.progress;
    let center = vec2(0.5, 0.5);
    // Two broad, broken chevrons compress then travel along the push axis.
    // A stopped push meets a transverse bar: shape, not only color, differs.
    for trail in [0.0, 0.23] {
        let advance = if blocked {
            (p * 0.30).min(0.12)
        } else {
            p * 0.64 - 0.25
        };
        let tip = center + direction * (advance - trail);
        for side in [-1.0, 1.0] {
            stroke(
                rect,
                tip + normal * side * 0.15 - direction * 0.10,
                tip + normal * side * 0.36 - direction * 0.26,
                1.5,
                faded(accent, 1.0 - p * 0.7),
            );
            pixel(
                rect,
                tip + normal * side * 0.15 - direction * 0.10,
                1.0,
                faded(light, 1.0 - p),
            );
        }
    }
    if blocked {
        let barrier = center + direction * 0.38;
        stroke(
            rect,
            barrier - normal * 0.34,
            barrier + normal * 0.34,
            1.5,
            faded(light, 1.0 - p * 0.7),
        );
    }
}

fn guard(rect: Rect, effect: TerminalEffectSample) {
    let breaking = effect.family == TerminalEffectFamily::GuardBreak;
    let p = effect.progress;
    let light = effect.highlight_color.unwrap_or(effect.color);
    let accent = effect.accent_color.unwrap_or(effect.color);
    // An angular shield closes around the bearer, never across its glyph.
    // When consumed its six edges split outward; no offensive radial wave.
    let points = [
        vec2(0.18, 0.20),
        vec2(0.50, 0.08),
        vec2(0.82, 0.20),
        vec2(0.78, 0.65),
        vec2(0.50, 0.91),
        vec2(0.22, 0.65),
        vec2(0.18, 0.20),
    ];
    for (index, edge) in points.windows(2).enumerate() {
        let middle = (edge[0] + edge[1]) * 0.5;
        let offset = if breaking {
            (middle - vec2(0.5, 0.5)) * p * 0.45
        } else {
            Vec2::ZERO
        };
        let completion = if breaking {
            (1.0 - p).max(0.12)
        } else {
            (p * 4.0).min(1.0)
        };
        let from = middle.lerp(edge[0], completion) + offset;
        let to = middle.lerp(edge[1], completion) + offset;
        stroke(
            rect,
            from,
            to,
            1.0,
            faded(accent, (1.0 - p).min(0.75) / 0.75),
        );
        if index % 2 == 0 {
            pixel(rect, to, 1.0, faded(light, 1.0 - p));
        }
    }
}

fn restoration(rect: Rect, effect: TerminalEffectSample) {
    // Two rising medical crosses flank the restored actor. Keep its center
    // readable; any later transfer trail must respect perception separately.
    let p = effect.progress;
    let light = effect.highlight_color.unwrap_or(effect.color);
    let accent = effect.accent_color.unwrap_or(effect.color);
    let fade = (1.0 - p).min(0.65) / 0.65;
    for (x, offset) in [(0.16, 0.0), (0.84, 0.1)] {
        let y = 0.78 - p * 0.42 - offset;
        let center = vec2(x, y);
        stroke(
            rect,
            center - vec2(0.06, 0.0),
            center + vec2(0.06, 0.0),
            1.0,
            faded(accent, fade),
        );
        stroke(
            rect,
            center - vec2(0.0, 0.09),
            center + vec2(0.0, 0.09),
            1.0,
            faded(accent, fade),
        );
        pixel(rect, center, 1.0, faded(light, fade));
        pixel(
            rect,
            center + vec2(0.0, 0.21),
            1.0,
            faded(effect.color, fade * 0.5),
        );
    }
}

fn piercing(rect: Rect, effect: TerminalEffectSample) {
    let direction = vec2(effect.direction.0, effect.direction.1);
    let normal = vec2(-direction.y, direction.x);
    let center = vec2(0.5, 0.5);
    let from = center - direction * 0.5;
    let to = center + direction * 0.5;
    let accent = effect.accent_color.unwrap_or(effect.color);
    let light = effect.highlight_color.unwrap_or(accent);
    // Thin continuous afterimage, a moving sharp head and two tapered rails;
    // distinct from electric branches and radial waves, clipped to this cell.
    stroke(rect, from, to, 1.0, faded(effect.color, 0.5));
    let head = from.lerp(to, (effect.progress * 1.8).min(1.0));
    let tail = from.lerp(head, 0.2);
    stroke(rect, tail, head, 1.0, light);
    for side in [-1.0, 1.0] {
        stroke(
            rect,
            tail + normal * side * 0.13,
            head,
            1.0,
            faded(accent, 0.8),
        );
    }
}

/// A shared 16-unit pixel grid, clipped even on very small cells. No particle
/// can spill into a hidden neighboring tile or over an adjacent HUD panel.
fn pixel_rect(rect: Rect, point: Vec2, size: f32) -> Rect {
    let unit = (rect.w.min(rect.h) / 16.0).max(0.5);
    let side = (size * unit).max(1.0);
    let x = (rect.x + (point.x * 16.0).round() * unit - side * 0.5).floor();
    let y = (rect.y + (point.y * 16.0).round() * unit - side * 0.5).floor();
    let left = x.clamp(rect.x, rect.right());
    let top = y.clamp(rect.y, rect.bottom());
    Rect::new(
        left,
        top,
        ((x + side).min(rect.right()) - left).max(0.0),
        ((y + side).min(rect.bottom()) - top).max(0.0),
    )
}

fn pixel(rect: Rect, point: Vec2, size: f32, color: Color) {
    let r = pixel_rect(rect, point, size);
    draw_rectangle(r.x, r.y, r.w, r.h, color);
}

fn stroke(rect: Rect, from: Vec2, to: Vec2, size: f32, color: Color) {
    let delta = to - from;
    let steps = (delta.x.abs().max(delta.y.abs()) * 16.0).ceil().max(1.0) as usize;
    for index in 0..=steps {
        pixel(
            rect,
            from + delta * (index as f32 / steps as f32),
            size,
            color,
        );
    }
}

fn wave(rect: Rect, effect: TerminalEffectSample) {
    // These weapons inflict electricity: show an electrical discharge, not a
    // rectangular shockwave. The outline remains sampling metadata only.
    conduction(rect, effect);
}

fn resonance(rect: Rect, effect: TerminalEffectSample) {
    // Two smooth pressure fronts move away from the source. No electrical
    // forks; local clipping never paints a hidden or blocked neighboring cell.
    let axis = vec2(effect.direction.0, effect.direction.1).normalize_or_zero();
    let normal = vec2(-axis.y, axis.x);
    let light = effect.highlight_color.unwrap_or(effect.color);
    let accent = effect.accent_color.unwrap_or(effect.color);
    for (offset, color) in [(0.0, light), (-0.18, accent)] {
        let center = vec2(0.5, 0.5) + axis * (effect.progress * 0.85 - 0.32 + offset);
        let mut previous = None;
        for step in 0..=12 {
            let t = step as f32 / 12.0 * 2.0 - 1.0;
            let point = center + normal * t * 0.48 - axis * t * t * 0.18;
            if let Some(from) = previous {
                stroke(rect, from, point, 1.2, color);
            }
            previous = Some(point);
        }
    }
    // Sparse mineral dust trails behind the front rather than flashing stars.
    for index in 0..3 {
        let t = index as f32 - 1.0;
        let point = vec2(0.5, 0.5) + normal * t * 0.25 + axis * (effect.progress * 0.5 - 0.4);
        pixel(rect, point, 0.9, faded(effect.color, 0.5));
    }
}

fn bolt(rect: Rect, from: Vec2, to: Vec2, phase: f32, effect: TerminalEffectSample) {
    let delta = to - from;
    let normal = vec2(-delta.y, delta.x).normalize_or_zero();
    let accent = effect.accent_color.unwrap_or(effect.color);
    let light = effect.highlight_color.unwrap_or(accent);
    let mut previous = from;
    for index in 1..=6 {
        let t = index as f32 / 6.0;
        let offset = if index == 6 {
            0.0
        } else {
            (phase + index as f32 * 2.4).sin() * 0.11
        };
        let next = from + delta * t + normal * offset;
        stroke(rect, previous, next, 3.0, faded(effect.color, 0.18));
        stroke(rect, previous, next, 1.1, accent);
        if index % 2 == 0 {
            stroke(rect, previous, next, 0.65, light);
        }
        previous = next;
    }
    let fork = from + delta * 0.55;
    let tip = fork + delta * 0.15 + normal * if phase.sin() > 0.0 { 0.17 } else { -0.17 };
    stroke(rect, fork, tip, 0.8, light);
}

fn conduction(rect: Rect, effect: TerminalEffectSample) {
    let phase = (effect.progress * 8.0).floor() + f32::from(effect.variant % 7);
    let center = vec2(0.5, 0.5);
    for (index, (dx, dy)) in EFFECT_NEIGHBORS.into_iter().enumerate() {
        if effect.links & (1 << index) == 0 {
            continue;
        }
        let edge = center + vec2(dx as f32, dy as f32) * 0.5;
        bolt(rect, center, edge, phase + index as f32, effect);
    }
    if effect.links == 0 {
        // An isolated visible cell receives a forked bolt, never a fake link
        // through unseen terrain. No plus-shaped generic spark.
        bolt(rect, vec2(0.22, 0.10), vec2(0.78, 0.90), phase, effect);
        bolt(
            rect,
            vec2(0.56, 0.46),
            vec2(0.13, 0.68),
            phase + 2.0,
            effect,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn effect_pixels_remain_inside_their_visible_cell_at_every_zoom() {
        for size in [8.0, 12.0, 24.0, 32.0, 48.0] {
            let cell = Rect::new(13.0, 17.0, size, size);
            for x in [-0.2, 0.0, 0.5, 1.0, 1.2] {
                for y in [-0.2, 0.0, 0.5, 1.0, 1.2] {
                    for width in [0.8, 1.0, 2.0, 3.0] {
                        let pixel = pixel_rect(cell, vec2(x, y), width);
                        assert!(pixel.x >= cell.x && pixel.y >= cell.y);
                        assert!(pixel.right() <= cell.right() && pixel.bottom() <= cell.bottom());
                        assert!(pixel.w >= 0.0 && pixel.h >= 0.0);
                    }
                }
            }
        }
    }
}
