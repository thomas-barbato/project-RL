//! Isolated renderer proposal: explicit clothing areas and paired color ramps.
//! Does not replace sprites or modify the game. Uses the existing full figures.
use super::*;

type Ramp = [[u8; 3]; 3];
const IVORY: Ramp = [[71, 72, 82], [175, 183, 190], [241, 237, 218]];
const NAVY: Ramp = [[23, 30, 58], [55, 76, 114], [104, 142, 173]];
const CHARCOAL: Ramp = [[25, 29, 37], [62, 71, 89], [118, 134, 151]];
const COPPER: Ramp = [[72, 41, 37], [168, 100, 64], [228, 170, 110]];
const OLIVE: Ramp = [[48, 48, 25], [125, 129, 65], [209, 217, 139]];
const GREEN: Ramp = [[18, 53, 41], [45, 161, 81], [144, 239, 162]];
const TEAL: Ramp = [[19, 49, 56], [46, 133, 146], [160, 221, 206]];
const RED: Ramp = [[77, 27, 38], [176, 49, 67], [239, 130, 112]];

fn ramps(outfit: u16, palette: EquipmentPalette) -> (Ramp, Ramp) {
    use EquipmentPalette::*;
    let secondary = match palette {
        Red | Blue | Turquoise => IVORY,
        Violet => TEAL,
        Yellow | Pink => NAVY,
        Original => match outfit {
            0 | 1 | 6 | 8 | 10 | 11 | 15 | 16 => IVORY,
            2 | 4 | 12 => NAVY,
            3 | 9 => Blue.ramp().unwrap(),
            5 | 7 => TEAL,
            13 => CHARCOAL,
            14 => Violet.ramp().unwrap(),
            _ => IVORY,
        },
    };
    let primary = palette.ramp().unwrap_or_else(|| match outfit {
        0 | 13 | 16 => RED,
        1 | 6 | 15 => Blue.ramp().unwrap(),
        2 => GREEN,
        3 | 9 => IVORY,
        4 | 8 => COPPER,
        5 | 11 => Violet.ramp().unwrap(),
        7 => CHARCOAL,
        10 => TEAL,
        12 => OLIVE,
        14 => Yellow.ramp().unwrap(),
        _ => IVORY,
    });
    (primary, secondary)
}

/// Stable areas across all eight poses. Large connected surfaces rather than
/// tiny colored ornaments. Weapon pixels are removed before this mask is used.
pub(super) fn secondary(outfit: u16, x: f32, y: f32) -> bool {
    let arms = (x < 20.5 || x >= 28.0) && (25.5..34.0).contains(&y);
    let knees = (37.5..42.0).contains(&y);
    let front = (21.0..27.5).contains(&x) && (27.0..35.5).contains(&y);
    match outfit {
        0..=3 | 5 | 6 | 9 => arms || knees,
        4 => front || knees,
        7 => arms || front,
        8 | 11 => front || ((22.0..26.5).contains(&x) && (35.5..44.5).contains(&y)),
        10 | 12 => front || arms,
        13 => arms || ((21.0..27.5).contains(&x) && (34.0..42.5).contains(&y)),
        14 => arms || (y >= 35.5 && y < 44.0),
        15 => front || knees,
        16 => arms || (y >= 36.0 && y < 43.0),
        _ => false,
    }
}

/// The old rust classifier accepted tan/copper sleeves as weapon paint.
/// Constrain weapon paint to its silhouette; the chest/hat/forearms outside
/// those areas stay with the outfit even when their original hue is warm.
pub(super) fn material(outfit: u16, pose: usize, x: f32, y: f32, p: [u8; 4]) -> Option<usize> {
    if (19.0..30.0).contains(&x) && (16.0..25.5).contains(&y) {
        return Some(0);
    }
    let hand = current::held_material(pose, x, y);
    let weapon = match pose {
        1 | 5 | 7 if x < 20.0 => x < 16.5 || (y >= 29.5 && x < 18.5),
        5 if x > 29.0 => x > 32.0 || (y >= 29.5 && x > 30.5),
        2 => (20.0..30.0).contains(&x) && (30.0..33.5).contains(&y),
        3 | 6 | 7 => y < 25.5,
        4 => y < 26.5 || (x < 17.0 || x > 31.5),
        _ => false,
    };
    if let Some(slot) = hand.filter(|_| weapon) {
        // Pale and neutral held surfaces are steel/gloves, not outfit paint.
        let hi = p[0].max(p[1]).max(p[2]);
        let lo = p[0].min(p[1]).min(p[2]);
        if hi - lo <= hi / 6 { return None; }
        if lo > 150 { return None; }
        return Some(slot);
    }
    let _ = outfit;
    Some(0)
}

fn flat_color(p: [u8; 4], ramp: Ramp) -> [u8; 4] {
    // Three values per colored material. Preserve shape/alpha exactly.
    let value = p[0].max(p[1]).max(p[2]);
    let c = ramp[if value < 96 { 0 } else if value < 190 { 1 } else { 2 }];
    [c[0], c[1], c[2], p[3]]
}

pub fn figure(source: &Image, outfit: u16, pose: usize, palettes: [EquipmentPalette; 3]) -> Image {
    let side = source.width / 16;
    let mut result = source.sub_image(Rect::new(
        f32::from(outfit % 16 * side), f32::from(outfit / 16 * side),
        f32::from(side), f32::from(side)));
    let (primary, accent) = ramps(outfit, palettes[0]);
    for y in 0..side {
        for x in 0..side {
            let n = (usize::from(y) * usize::from(side) + usize::from(x)) * 4;
            let p: [u8; 4] = result.bytes[n..n+4].try_into().unwrap();
            if p[3] == 0 { continue; }
            let px = f32::from(x) * 48.0 / f32::from(side);
            let py = f32::from(y) * 48.0 / f32::from(side);
            let Some(slot) = material(outfit, pose, px, py, p) else { continue; };
            if slot == 0 {
                if current::visor_pixel(outfit, px, py, p) || current::face_pixel(outfit, px, py, p) { continue; }
                let is_accent = secondary(outfit, px, py);
                let high = p[0].max(p[1]).max(p[2]);
                if current::painted(p, true) || (is_accent && high >= 65) {
                    result.bytes[n..n+4].copy_from_slice(&flat_color(p, if is_accent {accent} else {primary}));
                }
            } else if palettes[slot] != EquipmentPalette::Original && current::painted(p, false) {
                result.bytes[n..n+4].copy_from_slice(&current::color(p, palettes[slot]));
            }
        }
    }
    result
}
