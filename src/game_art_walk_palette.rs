//! Transfer only existing canonical color differences; alpha/geometry stay fixed.
use super::*;

#[derive(Clone, Copy, Default)]
struct Bin {
    count: u32,
    unchanged: u32,
    rgb: [i64; 3],
    delta: [i64; 3],
}
fn index(p: &[u8]) -> usize {
    ((p[0] as usize >> 3) << 10) | ((p[1] as usize >> 3) << 5) | (p[2] as usize >> 3)
}

pub(super) fn apply(number: u16, variant: u8, image: &mut Image) -> Option<()> {
    if variant == 0 {
        return Some(());
    }
    let base = super::super::campaign::reference_image(number, 0)?;
    let target = super::super::campaign::reference_image(number, variant)?;
    let mut bins = vec![Bin::default(); 32768];
    for y in 0..base.height as usize {
        for x in 0..base.width as usize {
            let i = (y * base.width as usize + x) * 4;
            let tx = x * target.width as usize / base.width as usize;
            let ty = y * target.height as usize / base.height as usize;
            let q = &target.bytes[(ty * target.width as usize + tx) * 4..][..4];
            let p = &base.bytes[i..i + 4];
            if p[3] < 220 || q[3] < 220 {
                continue;
            }
            let b = &mut bins[index(p)];
            b.count += 1;
            if (0..3).all(|k| p[k].abs_diff(q[k]) <= 3) {
                b.unchanged += 1;
            }
            for k in 0..3 {
                b.rgb[k] += p[k] as i64;
                b.delta[k] += q[k] as i64 - p[k] as i64;
            }
        }
    }
    let references: Vec<_> = bins
        .iter()
        .enumerate()
        .filter(|(_, b)| b.count > 0)
        .map(|(i, b)| {
            let n = b.count as i64;
            // Shared skin/steel/paint RGBs remain protected when the reference is ambiguous.
            let delta = if b.unchanged * 6 > b.count {
                [0; 3]
            } else {
                b.delta.map(|c| (c / n) as i16)
            };
            (i, b.rgb.map(|c| (c / n) as i16), delta)
        })
        .collect();
    if references.is_empty() {
        return Some(());
    }
    let mut lut = vec![None; 32768];
    for &(i, _, delta) in &references {
        lut[i] = Some(delta);
    }
    for pixel in image.bytes.chunks_exact_mut(4) {
        if pixel[3] == 0 {
            continue;
        }
        let bucket = index(pixel);
        let delta = if let Some(d) = lut[bucket] {
            d
        } else {
            let p = [pixel[0] as i32, pixel[1] as i32, pixel[2] as i32];
            let d = references
                .iter()
                .min_by_key(|(_, rgb, _)| {
                    (0..3).map(|k| (p[k] - rgb[k] as i32).pow(2)).sum::<i32>()
                })
                .unwrap()
                .2;
            lut[bucket] = Some(d);
            d
        };
        for k in 0..3 {
            pixel[k] = (pixel[k] as i16 + delta[k]).clamp(0, 255) as u8;
        }
    }
    Some(())
}
