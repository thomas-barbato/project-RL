//! FIREARM02 / 02: approved shared rifle preparation, before texture upload.
use macroquad::prelude::{BLANK, Image};
use std::cell::Cell;

pub(crate) fn rifle(model: u16) -> bool {
    matches!(model, 0..=12 | 35..=46 | 54..=60)
}
thread_local! { static ORIGINAL: Cell<bool> = const { Cell::new(false) }; }
pub(crate) struct OriginalScope(bool);
impl Drop for OriginalScope {
    fn drop(&mut self) {
        ORIGINAL.set(self.0);
    }
}
// NPC fallback rigs can reuse the player's front-layer compositor. Keep their
// existing drawing and use a separate cache entry for that original source.
pub(crate) fn original_scope(original: bool) -> OriginalScope {
    let old = ORIGINAL.get();
    ORIGINAL.set(old || original);
    OriginalScope(old)
}
pub(crate) fn variant() -> u8 {
    if ORIGINAL.get() {
        return 0;
    }
    #[cfg(debug_assertions)]
    if let Some(style) = crate::animation_polish_review::source_override() {
        return if matches!(style, 4 | 5) { style } else { 0 };
    }
    5
}
pub(crate) fn enabled(model: u16) -> bool {
    rifle(model) && variant() != 0
}
pub(crate) fn prepare(source: &Image, model: u16, a: [f32; 2], b: [f32; 2]) -> Image {
    if !enabled(model) {
        return source.clone();
    }
    prepare_profile(source, a, b, variant())
}
pub(crate) fn prepare_profile(source: &Image, a: [f32; 2], b: [f32; 2], profile: u8) -> Image {
    let (extension, thickness) = if profile == 4 {
        (0.85, 1.0)
    } else {
        (0.95, 1.08)
    };
    let inverse = |value: f32, low: f32, high: f32, factor: f32| {
        if value < low {
            low + (value - low) / factor
        } else if value > high {
            high + (value - high) / factor
        } else {
            value
        }
    };
    let mut out = Image::gen_image_color(source.width, source.height, BLANK);
    for y in 0..usize::from(source.height) {
        for x in 0..usize::from(source.width) {
            let sx = inverse(x as f32 + 0.5, a[0].min(b[0]), a[0].max(b[0]), extension) - 0.5;
            let sy = inverse(y as f32 + 0.5, a[1].min(b[1]), a[1].max(b[1]), thickness) - 0.5;
            let (xx, yy) = (sx.round() as i32, sy.round() as i32);
            if (0..i32::from(source.width)).contains(&xx)
                && (0..i32::from(source.height)).contains(&yy)
            {
                let from = (yy as usize * usize::from(source.width) + xx as usize) * 4;
                let to = (y * usize::from(source.width) + x) * 4;
                out.bytes[to..to + 4].copy_from_slice(&source.bytes[from..from + 4]);
            }
        }
    }
    clarify_metal(&mut out);
    out
}
pub(crate) fn clarify_metal(image: &mut Image) {
    let source = image.clone();
    for y in 0..usize::from(image.height) {
        for x in 0..usize::from(image.width) {
            let at = (y * usize::from(image.width) + x) * 4;
            let p = &source.bytes[at..at + 4];
            let low = *p[..3].iter().min().unwrap();
            let high = *p[..3].iter().max().unwrap();
            if p[3] < 128 || high - low > 30 || high < 45 || high > 185 {
                continue;
            }
            let upper_edge = y > 0 && source.bytes[at - usize::from(image.width) * 4 + 3] < 96;
            for c in 0..3 {
                let value =
                    (f32::from(p[c]) - 80.) * 1.15 + 90. + if upper_edge { 18. } else { 0. };
                image.bytes[at + c] = value.round().clamp(20., 205.) as u8;
            }
        }
    }
}
pub(crate) fn contact_shadow(body: &Image, weapon: &Image) -> Image {
    assert_eq!((body.width, body.height), (weapon.width, weapon.height));
    let mut out = Image::gen_image_color(body.width, body.height, BLANK);
    let (w, h) = (usize::from(body.width), usize::from(body.height));
    for y in 0..h {
        for x in 0..w {
            let at = (y * w + x) * 4;
            if body.bytes[at + 3] == 0 {
                continue;
            }
            let mut alpha = 0;
            for yy in y.saturating_sub(1)..=(y + 1).min(h - 1) {
                for xx in x.saturating_sub(1)..=(x + 1).min(w - 1) {
                    alpha = alpha.max(weapon.bytes[(yy * w + xx) * 4 + 3]);
                }
            }
            let ring = alpha.saturating_sub(weapon.bytes[at + 3]);
            out.bytes[at..at + 4].copy_from_slice(&[
                8,
                11,
                15,
                (u32::from(ring) * u32::from(body.bytes[at + 3]) * 9 / 255 / 10) as u8,
            ]);
        }
    }
    out
}
