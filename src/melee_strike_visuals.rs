//! Shared B01-A01 stroke and impact, extracted unchanged from the approved
//! ANIM-S02 laboratory. Presentation only; no game state or simulation RNG.
use macroquad::prelude::*;
const STEEL: Color = Color::new(0.73, 0.86, 0.90, 1.0);
const WHITE: Color = Color::new(0.98, 0.98, 0.88, 1.0);
fn alpha(mut c: Color, a: f32) -> Color {
    c.a = a.clamp(0.0, 1.0);
    c
}
fn noise(index: u32) -> f32 {
    let mut value = index.wrapping_add(19).wrapping_mul(2_654_435_761);
    value ^= value >> 16;
    value = value.wrapping_mul(2_246_822_519);
    value as f32 / u32::MAX as f32
}

fn glow(at: Vec2, radius: f32, color: Color, opacity: f32) {
    for (size, strength) in [(2.2, 0.05), (1.6, 0.08), (1.15, 0.13), (0.7, 0.22)] {
        draw_circle(at.x, at.y, radius * size, alpha(color, opacity * strength));
    }
}

fn luminous_line(from: Vec2, to: Vec2, width: f32, color: Color, opacity: f32) {
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

fn shard(at: Vec2, direction: Vec2, length: f32, width: f32, color: Color, opacity: f32) {
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

// Tapered steel stroke, with a continuous mesh instead of overlapping segments.
pub(crate) fn cut(from: Vec2, to: Vec2, width: f32, life: f32) {
    let d = to - from;
    if d.length_squared() < 0.001 {
        return;
    }
    let normal = vec2(-d.y, d.x).normalize();
    let mut mesh = Mesh {
        vertices: Vec::with_capacity(100),
        indices: Vec::with_capacity(432),
        texture: None,
    };
    for i in 0..=24_u16 {
        let k = i as f32 / 24.0;
        let center = from + d * k;
        let spread = width * (k * std::f32::consts::PI).sin().sqrt();
        for (offset, color) in [
            (-spread, alpha(STEEL, 0.0)),
            (-spread * 0.22, alpha(STEEL, life * 0.65)),
            (spread * 0.08, alpha(WHITE, life)),
            (spread * 0.40, alpha(STEEL, 0.0)),
        ] {
            let point = center + normal * offset;
            mesh.vertices
                .push(Vertex::new(point.x, point.y, 0.0, 0.0, 0.0, color));
        }
        if i > 0 {
            let p = (i - 1) * 4;
            let c = i * 4;
            for s in 0..3 {
                mesh.indices.extend_from_slice(&[
                    p + s,
                    p + s + 1,
                    c + s,
                    c + s,
                    p + s + 1,
                    c + s + 1,
                ]);
            }
        }
    }
    draw_mesh(&mesh);
}

pub(crate) fn impact(at: Vec2, age: f32, weight: f32, plates: bool, tile: f32) {
    if !(0.0..0.62).contains(&age) {
        return;
    }
    let life = 1.0 - age / 0.62;
    glow(at, tile * 0.22 * weight, WHITE, life);
    if age < 0.21 {
        let r = tile * (0.10 + age * 1.1) * weight;
        draw_circle_lines(at.x, at.y, r, tile * 0.035 * life, alpha(STEEL, life * 0.7));
        for a in [0.1_f32, 1.45, 2.8, 4.4] {
            let direction = vec2(a.cos(), a.sin());
            luminous_line(
                at + direction * 5.0,
                at + direction * r * 1.7,
                2.1,
                WHITE,
                life,
            );
        }
    }
    for n in 0..24 {
        let angle = noise(n + 32) * std::f32::consts::TAU;
        let direction = vec2(angle.cos(), angle.sin());
        let velocity = tile * (0.6 + noise(n + 110) * 1.8) * weight;
        let point = at + direction * age * velocity + vec2(0.0, age * age * tile * 0.65);
        if plates && n % 3 == 0 {
            shard(
                point,
                direction,
                tile * 0.15 * life,
                tile * 0.043 * life,
                STEEL,
                life,
            );
        } else {
            luminous_line(point - direction * tile * 0.08, point, 1.4, WHITE, life);
        }
    }
}
