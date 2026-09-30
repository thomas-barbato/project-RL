//! Pixel silhouettes, driven only by presentation samples. No combat state,
//! RNG, texture or clock: the same sample always produces the same image.
use crate::visual_effects::{TerminalEffectFamily, TerminalEffectSample};
use macroquad::prelude::*;

const SIDE: usize = 16;
type Raster = [u8; SIDE * SIDE];

fn put(raster: &mut Raster, x: i32, y: i32, tone: u8) {
    if (0..SIDE as i32).contains(&x) && (0..SIDE as i32).contains(&y) {
        let pixel = &mut raster[y as usize * SIDE + x as usize];
        *pixel = (*pixel).max(tone);
    }
}

fn oval(raster: &mut Raster, center: Vec2, radius: Vec2, tone: u8) {
    for y in 0..SIDE {
        for x in 0..SIDE {
            let delta = (vec2(x as f32 + 0.5, y as f32 + 0.5) - center) / radius;
            if delta.length_squared() <= 1.0 {
                put(raster, x as i32, y as i32, tone);
            }
        }
    }
}

fn tongue(raster: &mut Raster, x: f32, base: f32, height: f32, width: f32, phase: f32) {
    for y in 0..SIDE {
        let rise = (base - y as f32 - 0.5) / height.max(0.1);
        if !(0.0..1.0).contains(&rise) {
            continue;
        }
        let center = x + (rise * 5.0 + phase).sin() * rise * 2.4;
        let half = width * (1.0 - rise).powf(0.75);
        for px in 0..SIDE {
            let distance = (px as f32 + 0.5 - center).abs();
            if distance > half {
                continue;
            }
            let tone = if distance < half * 0.36 && rise < 0.50 {
                4
            } else if distance < half * 0.76 && rise < 0.82 {
                3
            } else {
                2
            };
            put(raster, px as i32, y as i32, tone);
        }
    }
}

fn fire(effect: TerminalEffectSample) -> Raster {
    let mut raster = [0; SIDE * SIDE];
    let p = effect.progress;
    let seed = f32::from(effect.variant % 17);
    let phase = p * if effect.sustained {
        std::f32::consts::TAU
    } else {
        15.0
    } + seed;
    let burst = effect.family == TerminalEffectFamily::Catalysis;
    let life = if burst {
        ((1.0 - p) * 2.2).min(1.0)
    } else {
        0.83 + 0.17 * (phase * if effect.sustained { 1.0 } else { 0.7 }).sin()
    };
    let direction = vec2(effect.direction.0, effect.direction.1).normalize_or_zero();
    // A brief rounded ignition, then uneven rising tongues. The blast enters
    // neighboring cells from the impact side instead of stamping little rings.
    if burst && p < 0.34 {
        let center = vec2(8.0, 9.0) - direction * (1.0 - p * 2.0) * 3.0;
        let radius = 3.8 + (p * 12.0).min(3.6);
        oval(&mut raster, center, vec2(radius, radius * 0.76), 2);
        oval(
            &mut raster,
            center + vec2(0.0, 1.0),
            vec2(radius * 0.70, radius * 0.55),
            3,
        );
        oval(
            &mut raster,
            center + vec2(-1.0, 1.0),
            vec2(radius * 0.37, radius * 0.30),
            4,
        );
    }
    for (index, x) in [3.0, 8.0, 12.5].into_iter().enumerate() {
        let flicker = (phase + index as f32 * 2.2).sin();
        let height = (if index == 1 { 12.0 } else { 8.8 } + flicker * 2.1) * life;
        tongue(
            &mut raster,
            x + if burst { direction.x * p * 2.0 } else { 0.0 },
            15.0 - if burst { p * 3.0 } else { 0.0 },
            height,
            if burst { 4.4 } else { 3.2 },
            phase + index as f32,
        );
    }
    for index in 0..4 {
        let t = (p * if effect.sustained { 1.0 } else { 1.2 } + index as f32 * 0.27).fract();
        let x = 2.0 + ((seed * 3.0 + index as f32 * 4.2) % 12.0) + (t * 6.0 + seed).sin();
        let y = 12.0 - t * 13.0;
        put(
            &mut raster,
            x as i32,
            y as i32,
            if t < 0.45 { 4 } else { 3 },
        );
        if burst && p > 0.55 {
            put(&mut raster, x as i32 + 1, y as i32 - 1, 1);
        }
    }
    raster
}

fn acid(effect: TerminalEffectSample) -> Raster {
    let mut raster = [0; SIDE * SIDE];
    let p = effect.progress;
    let pool = effect.family == TerminalEffectFamily::Caustic;
    let seed = f32::from(effect.variant % 11);
    if pool {
        let spread = if effect.sustained {
            1.0
        } else {
            (p * 3.5).min(1.0)
        };
        oval(
            &mut raster,
            vec2(8.0, 13.0),
            vec2(3.5 + spread * 4.0, 2.4),
            2,
        );
        oval(&mut raster, vec2(4.0, 12.3), vec2(3.1, 1.8), 3);
        oval(&mut raster, vec2(11.0, 12.8), vec2(3.5, 1.7), 3);
        for index in 0..3 {
            let cycle = (p * 2.0 + index as f32 * 0.33).fract();
            let x = 3.0 + index as f32 * 4.6;
            if cycle < 0.68 {
                let radius = 0.65 + cycle * 1.5;
                oval(&mut raster, vec2(x, 11.5 - cycle), vec2(radius, radius), 4);
                // Dark bubble centers cut into the fluid, unlike fire sparks.
                let y = (11.5 - cycle) as usize;
                raster[y * SIDE + x as usize] = 2;
            } else {
                put(&mut raster, (x - 1.0) as i32, (9.5 - cycle) as i32, 4);
                put(&mut raster, (x + 1.0) as i32, (9.0 - cycle) as i32, 3);
            }
        }
    }
    // An established puddle bubbles in place; do not replay its initial fall
    // and spreading on every loop. Corrosion attached to an actor still drips.
    if pool && effect.sustained {
        return raster;
    }
    // Heavy rounded drops fall, squash and splash into separate satellite drops.
    for (index, x) in [2.5, 12.5, 7.0].into_iter().enumerate() {
        let t = (p + index as f32 * 0.19 + seed * 0.008).fract();
        if t < 0.58 {
            let y = 1.0 + (t / 0.58).powi(2) * 10.5;
            oval(&mut raster, vec2(x, y), vec2(1.65, 2.0), 3);
            put(&mut raster, x as i32, y as i32 - 1, 4);
            put(&mut raster, x as i32, y as i32 - 3, 2);
        } else {
            let t = (t - 0.58) / 0.42;
            for sign in [-1.0, 1.0] {
                oval(
                    &mut raster,
                    vec2(
                        x + sign * (1.0 + t * 3.2),
                        12.0 - (t * std::f32::consts::PI).sin() * 4.0,
                    ),
                    vec2(1.1, 1.1),
                    3,
                );
            }
            if !pool {
                oval(&mut raster, vec2(x, 13.5), vec2(2.0, 0.9), 2);
            }
        }
    }
    raster
}

fn frost(effect: TerminalEffectSample) -> Raster {
    let mut raster = [0; SIDE * SIDE];
    let p = effect.progress;
    let growth = if effect.sustained {
        1.0
    } else {
        (0.35 + p * 2.4).min(1.0)
    };
    for (index, (x, height, half)) in [(3.0, 8.0, 2.8), (8.0, 14.0, 3.4), (13.0, 10.0, 2.8)]
        .into_iter()
        .enumerate()
    {
        let height = height * growth;
        let lean = if index == 0 { -0.15 } else { 0.13 };
        for y in 0..SIDE {
            let rise = 14.0 - y as f32;
            if rise < 0.0 || rise > height {
                continue;
            }
            let center = x + lean * rise;
            let width = half * (1.0 - rise / height).min(0.75);
            for px in 0..SIDE {
                let delta = px as f32 - center;
                if delta.abs() <= width {
                    put(
                        &mut raster,
                        px as i32,
                        y as i32,
                        if delta.abs() < 0.6 {
                            4
                        } else if delta < 0.0 {
                            3
                        } else {
                            2
                        },
                    );
                }
            }
        }
    }
    for index in 0..7 {
        let x = (index * 7 + usize::from(effect.variant)) % 16;
        put(&mut raster, x as i32, 14 + (index % 2) as i32, 3);
        if p > 0.62 {
            put(
                &mut raster,
                x as i32,
                (12.0 - (p - 0.62) * 14.0 - (index % 3) as f32) as i32,
                4,
            );
        }
    }
    raster
}

fn flame_jet(effect: TerminalEffectSample) -> Raster {
    let upright = fire(effect);
    let mut rotated = [0; SIDE * SIDE];
    let mut axis = vec2(effect.direction.0, effect.direction.1).normalize_or_zero();
    if axis == Vec2::ZERO {
        axis = vec2(1.0, 0.0);
    }
    let tangent = vec2(-axis.y, axis.x);
    // Reorient the same authored fire tongues along the actual attack vector.
    for y in 0..SIDE {
        for x in 0..SIDE {
            let delta = vec2(x as f32 - 7.5, y as f32 - 7.5);
            let sx = (7.5 + delta.dot(tangent)).round() as i32;
            let sy = (7.5 - delta.dot(axis)).round() as i32;
            if (0..16).contains(&sx) && (0..16).contains(&sy) {
                rotated[y * SIDE + x] = upright[sy as usize * SIDE + sx as usize];
            }
        }
    }
    rotated
}

pub(super) fn raster(effect: TerminalEffectSample) -> Raster {
    match effect.family {
        TerminalEffectFamily::FlameJet => flame_jet(effect),
        TerminalEffectFamily::Flame | TerminalEffectFamily::Catalysis => fire(effect),
        TerminalEffectFamily::Corrosion | TerminalEffectFamily::Caustic => acid(effect),
        TerminalEffectFamily::Frost => frost(effect),
        _ => [0; SIDE * SIDE],
    }
}

pub(super) fn draw(rect: Rect, effect: TerminalEffectSample, occupied: bool) {
    let raster = raster(effect);
    let accent = effect.accent_color.unwrap_or(effect.color);
    let light = effect.highlight_color.unwrap_or(accent);
    for y in 0..SIDE {
        for x in 0..SIDE {
            let tone = raster[y * SIDE + x];
            let mut color = match tone {
                0 => continue,
                1 => Color::new(0.32, 0.29, 0.26, effect.color.a * 0.45),
                2 => effect.color,
                3 => accent,
                _ => light,
            };
            // The actual occupant is drawn afterwards. Quiet its glyph's core
            // without hollowing out unoccupied flames, pools or crystals.
            if occupied && (5..11).contains(&x) && (3..13).contains(&y) {
                color.a *= 0.22;
            }
            let left = rect.x + x as f32 * rect.w / SIDE as f32;
            let top = rect.y + y as f32 * rect.h / SIDE as f32;
            let right = (rect.x + (x + 1) as f32 * rect.w / SIDE as f32).min(rect.right());
            let bottom = (rect.y + (y + 1) as f32 * rect.h / SIDE as f32).min(rect.bottom());
            draw_rectangle(left, top, right - left, bottom - top, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample(family: TerminalEffectFamily, progress: f32) -> TerminalEffectSample {
        TerminalEffectSample {
            symbol: '*',
            color: WHITE,
            accent_color: Some(WHITE),
            highlight_color: Some(WHITE),
            outline: None,
            family,
            progress,
            sustained: false,
            variant: 7,
            links: 0,
            direction: (0.0, 0.0),
        }
    }

    #[test]
    fn elements_have_filled_distinct_silhouettes_even_without_color() {
        let fire = raster(sample(TerminalEffectFamily::Catalysis, 0.3));
        let acid = raster(sample(TerminalEffectFamily::Caustic, 0.3));
        let ice = raster(sample(TerminalEffectFamily::Frost, 0.3));
        let occupied = |r: Raster| r.map(|tone| tone != 0);
        assert_ne!(occupied(fire), occupied(acid));
        assert_ne!(occupied(fire), occupied(ice));
        assert_ne!(occupied(acid), occupied(ice));
        assert!(fire.into_iter().filter(|tone| *tone > 0).count() >= 65);
        assert!(acid[12 * SIDE..].iter().filter(|tone| **tone > 0).count() >= 22);
        assert!(ice[..8 * SIDE].iter().any(|tone| *tone > 0));
    }

    #[test]
    fn elemental_samples_animate_deterministically_with_bounded_palette() {
        for family in [
            TerminalEffectFamily::Flame,
            TerminalEffectFamily::FlameJet,
            TerminalEffectFamily::Catalysis,
            TerminalEffectFamily::Caustic,
            TerminalEffectFamily::Corrosion,
            TerminalEffectFamily::Frost,
        ] {
            let early = sample(family, 0.08);
            let late = sample(family, 0.68);
            assert_ne!(raster(early), raster(late), "{family:?}");
            assert_eq!(raster(early), raster(early));
            for step in 0..=100 {
                let frame = raster(sample(family, step as f32 / 100.0));
                assert!(frame.iter().all(|tone| *tone <= 4));
            }
        }
    }

    #[test]
    fn flame_jet_rotates_its_silhouette_with_the_actual_attack_direction() {
        let mut effect = sample(TerminalEffectFamily::FlameJet, 0.3);
        effect.direction = (1.0, 0.0);
        let right = raster(effect);
        effect.direction = (-1.0, 0.0);
        let left = raster(effect);
        effect.direction = (0.0, 1.0);
        let down = raster(effect);
        assert_ne!(right, left);
        assert_ne!(right, down);
        for frame in [right, left, down] {
            assert!(frame.iter().filter(|tone| **tone > 0).count() >= 30);
        }
    }

    #[test]
    fn sustained_flames_and_pools_never_vanish_or_replay_their_creation() {
        for family in [TerminalEffectFamily::Flame, TerminalEffectFamily::Caustic] {
            let mut effect = sample(family, 0.0);
            effect.sustained = true;
            let first = raster(effect);
            for step in 0..=100 {
                effect.progress = step as f32 / 100.0;
                let frame = raster(effect);
                assert!(frame.iter().filter(|tone| **tone > 0).count() >= 30);
                if family == TerminalEffectFamily::Caustic {
                    assert!(frame[..7 * SIDE].iter().all(|tone| *tone == 0));
                    assert!(frame[12 * SIDE..].iter().filter(|tone| **tone > 0).count() >= 22);
                }
            }
            // The seam is the same pose (allow subpixel rounding at 2π).
            let last = raster(effect);
            assert!(first.iter().zip(last).filter(|(a, b)| **a != *b).count() <= 4);
            effect.progress = 0.32;
            assert_ne!(raster(effect), first);
        }
    }
}
