//! Cached material recoloring. Texture alpha, sprite coordinates, black outlines
//! and pale steel remain unchanged; paint/fabric use the instance's color ramp.
use super::*;
use std::collections::BTreeMap;

pub(super) fn decoded(bytes: &[u8]) -> Image {
    Image::from_file_with_format(bytes, Some(ImageFormat::Png)).expect("valid embedded sprite")
}

pub(super) fn family(index: u16) -> Option<u16> {
    Some(match index {
        0..=6 => 0,
        7..=12 => 1,
        13..=20 => 2,
        21..=27 => 3,
        28..=34 => 4,
        35..=46 => 5,
        47..=53 => 6,
        54..=60 => 7,
        150..=155 => 8,
        156..=161 => 9,
        _ => return None,
    })
}

pub(super) fn painted(pixel: [u8; 4], cloth: bool) -> bool {
    let [r, g, b, a] = pixel;
    let high = r.max(g).max(b);
    let low = r.min(g).min(b);
    a > 0 && high > 36 && (high - low > high / 6 || (cloth && high > 125))
}

pub(super) fn color(pixel: [u8; 4], palette: EquipmentPalette) -> [u8; 4] {
    let Some(ramp) = palette.ramp() else {
        return pixel;
    };
    let [r, g, b, a] = pixel;
    let value = f32::from(r.max(g).max(b)) / 255.0;
    let (low, high, t) = if value < 0.53 {
        (ramp[0], ramp[1], (value - 0.12).max(0.0) / 0.41)
    } else {
        (ramp[1], ramp[2], (value - 0.53) / 0.47)
    };
    let channel = |n| (f32::from(low[n]) * (1.0 - t) + f32::from(high[n]) * t).round() as u8;
    [channel(0), channel(1), channel(2), a]
}

/// The authored frames share the same 30px body and 48px canvas. These zones
/// identify held material, never modify positions or draw equipment overlays.
pub(super) fn held_material(pose: usize, x: f32, y: f32) -> Option<usize> {
    match pose {
        1 if x < 20.0 && y < 31.0 => Some(1),
        2 if (12.0..39.0).contains(&x) && (30.0..35.0).contains(&y) => Some(1),
        3 if x < 20.0 && y < 25.5 => Some(1),
        4 if (x < 20.0 || x > 29.0) && y < 31.0 => Some(1),
        5 if x < 20.0 && y < 32.0 => Some(1),
        5 if x > 29.0 && y < 32.0 => Some(2),
        6 if x < 20.0 && y < 25.5 => Some(1),
        6 if x > 29.0 && y < 25.5 => Some(2),
        7 if x < 20.0 && y < 32.0 => Some(1),
        7 if x > 29.0 && y < 25.5 => Some(2),
        _ => None,
    }
}

pub(super) fn face_pixel(outfit: u16, x: f32, y: f32, pixel: [u8; 4]) -> bool {
    let [r, g, b, _] = pixel;
    // Local material masks in the common 48px frame. The technician's face
    // reaches below its brim in armed poses. Yellow helmet paint is not skin;
    // the red full-face mask (outfit 13) has no exposed face to protect.
    let rows = match outfit {
        4 | 8 | 15 => 22.5..26.0,
        7 | 11 | 16 => 22.5..25.5,
        14 => 23.0..27.0,
        _ => return false,
    };
    (22.0..27.0).contains(&x)
        && rows.contains(&y)
        && r > 110
        && r.saturating_sub(g) > 12
        && g.saturating_sub(b) > 8
        && u16::from(g.saturating_sub(b)) < u16::from(r.saturating_sub(g)) + 30
}

pub(super) fn visor_pixel(outfit: u16, x: f32, y: f32, pixel: [u8; 4]) -> bool {
    let [r, g, b, _] = pixel;
    // Only authored visor/eye windows retain their light color. In particular
    // this never protects the technician's yellow shell or its wide brim.
    match outfit {
        0 | 2 => {
            (21.0..28.0).contains(&x) && (21.5..25.0).contains(&y) && r > 150 && g > 90 && b < 65
        }
        1 | 5 | 13 => {
            (21.0..28.0).contains(&x)
                && (21.5..25.0).contains(&y)
                && g > 125
                && b > 100
                && r < g / 2
        }
        15 => {
            (22.0..27.0).contains(&x)
                && (18.0..21.0).contains(&y)
                && (r.min(g).min(b) > 150 || (r > 150 && g > 90 && b < 65))
        }
        _ => false,
    }
}

// Return None for held steel/gloves, 0 for cloth/plates, 1/2 for weapon paint.
// In particular the rifle's shoulder/chest plates above its receiver belong
// to the outfit. A receiver color must never spread across the whole chest.
fn material(pose: usize, x: f32, y: f32, p: [u8; 4]) -> Option<usize> {
    let Some(hand) = held_material(pose, x, y) else {
        return Some(0);
    };
    let [r, g, b, _] = p;
    let neutral = r.max(g).max(b) - r.min(g).min(b) <= r.max(g).max(b) / 6;
    if neutral {
        return None;
    }
    let pale = r.min(g).min(b) > 150;
    if pale {
        return Some(0);
    }
    let receiver = pose != 2 || ((21.5..28.5).contains(&x) && y < 33.5);
    let rust = r > g && g > b && g as u16 * 100 > r as u16 * 22;
    if receiver && rust && painted(p, false) {
        Some(hand)
    } else {
        Some(0)
    }
}

fn figure_material(outfit: u16, pose: usize, x: f32, y: f32, p: [u8; 4]) -> Option<usize> {
    if outfit == 14 {
        // The hardhat brim crosses the old hand rectangle by one native pixel.
        // Ochre sleeves also cross it in sword poses; the authored gun/sword
        // receivers use red-brown paint, outside these local sleeve masks.
        let helmet = (19.0..30.0).contains(&x) && (17.0..24.0).contains(&y);
        let sleeves = ((16.0..21.0).contains(&x) || (29.0..33.0).contains(&x))
            && (26.0..32.0).contains(&y)
            && u16::from(p[1]) * 100 > u16::from(p[0]) * 42
            && u16::from(p[2]) * 100 < u16::from(p[1]) * 65;
        if helmet || sleeves {
            return Some(0);
        }
    }
    material(pose, x, y, p)
}

pub(super) fn figure(
    source: &Image,
    index: u16,
    pose: usize,
    palettes: [EquipmentPalette; 3],
) -> Image {
    let side = source.width / 16;
    let mut result = source.sub_image(Rect::new(
        f32::from(index % 16 * side),
        f32::from(index / 16 * side),
        f32::from(side),
        f32::from(side),
    ));
    for y in 0..side {
        for x in 0..side {
            let n = (usize::from(y) * usize::from(side) + usize::from(x)) * 4;
            let p: [u8; 4] = result.bytes[n..n + 4].try_into().unwrap();
            let (px, py) = (
                f32::from(x) * 48.0 / f32::from(side),
                f32::from(y) * 48.0 / f32::from(side),
            );
            let Some(slot) = figure_material(index, pose, px, py, p) else {
                continue;
            };
            let palette = palettes[slot];
            // Visor lights, exposed faces and neutral steel are retained.
            if palette != EquipmentPalette::Original
                && painted(p, slot == 0)
                && !(slot == 0 && (visor_pixel(index, px, py, p) || face_pixel(index, px, py, p)))
            {
                result.bytes[n..n + 4].copy_from_slice(&color(p, palette));
            }
        }
    }
    result
}

