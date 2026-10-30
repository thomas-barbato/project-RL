//! Exact pixel ownership and rigid premultiplied weapon sampling for VERT18.
use super::{Gear, Transform};
use crate::asset_preparation::{self as preparation, PreparedImage};
use macroquad::prelude::*;

fn blend(d: &mut [u8], s: &[u8]) {
    if s[3] == 0 {
        return;
    }
    let a = u32::from(s[3]);
    let old = u32::from(d[3]) * (255 - a);
    let total = a * 255 + old;
    for c in 0..3 {
        d[c] = ((u32::from(s[c]) * a * 255 + u32::from(d[c]) * old + total / 2) / total) as u8;
    }
    d[3] = ((total + 127) / 255) as u8;
}
pub(super) fn over(dest: &mut Image, source: &Image, origin: [i32; 2]) {
    for y in 0..i32::from(source.height) {
        let yy = y + origin[1];
        if yy < 0 || yy >= i32::from(dest.height) {
            continue;
        }
        for x in 0..i32::from(source.width) {
            let xx = x + origin[0];
            if xx < 0 || xx >= i32::from(dest.width) {
                continue;
            }
            let a = (y as usize * usize::from(source.width) + x as usize) * 4;
            let b = (yy as usize * usize::from(dest.width) + xx as usize) * 4;
            blend(&mut dest.bytes[b..b + 4], &source.bytes[a..a + 4]);
        }
    }
}
pub(super) fn over_cell(
    dest: &mut Image,
    sheet: &Image,
    cell: [u16; 2],
    n: usize,
    origin: [i32; 2],
) {
    let [w, h] = cell;
    let source = sheet.sub_image(Rect::new(
        (n % 6) as f32 * f32::from(w),
        (n / 6) as f32 * f32::from(h),
        f32::from(w),
        f32::from(h),
    ));
    over(dest, &source, origin);
}
pub(super) fn helmet(
    body: &mut Image,
    gear: &Gear,
    anchor: [f32; 2],
    back: bool,
    model: u16,
) -> Result<(), String> {
    let cover = preparation::image(if back {
        &gear.back_file
    } else {
        &gear.front_file
    })?;
    let opening = preparation::image(&gear.opening_file)?;
    let ownership = preparation::image(&gear.ownership_file)?;
    let (ox, oy) = (
        anchor[0].round() as usize - 16,
        anchor[1].round() as usize - 16,
    );
    let col = usize::from(model - 115) * 32;
    for y in 0..32 {
        for x in 0..32 {
            let b = ((oy + y) * 256 + ox + x) * 4;
            let s = (y * usize::from(cover.width) + col + x) * 4;
            let o = (y * usize::from(opening.width) + col + x) * 4;
            let own = ownership.bytes[(y * 32 + x) * 4 + 3] > 0;
            let restore = if back {
                matches!(model, 115 | 119) && own && cover.bytes[s + 3] < 220
            } else {
                opening.bytes[o + 3] > 0
            };
            if own && !restore {
                body.bytes[b..b + 4].fill(0);
            }
            blend(&mut body.bytes[b..b + 4], &cover.bytes[s..s + 4]);
        }
    }
    Ok(())
}
pub(super) fn weapon(dest: &mut Image, source: &Image, t: &Transform) {
    let (sn, cs) = t.angle.sin_cos();
    let scale = t.scale;
    // Restrict expensive sampling to the transformed opaque source envelope.
    let mut bound = [128., 128., 0., 0.];
    for (i, p) in source.bytes.chunks_exact(4).enumerate() {
        if p[3] > 0 {
            let x = (i % usize::from(source.width)) as f32;
            let y = (i / usize::from(source.width)) as f32;
            let x = if t.flip { 127. - x } else { x };
            bound[0] = f32::min(bound[0], x);
            bound[1] = f32::min(bound[1], y);
            bound[2] = f32::max(bound[2], x + 1.);
            bound[3] = f32::max(bound[3], y + 1.);
        }
    }
    let mut extent = [
        f32::INFINITY,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NEG_INFINITY,
    ];
    for x in [bound[0] - 1., bound[2] + 1.] {
        for y in [bound[1] - 1., bound[3] + 1.] {
            let dx = x - t.pivot[0];
            let dy = y - t.pivot[1];
            let xx = t.target[0] + scale * (cs * dx - sn * dy);
            let yy = t.target[1] + scale * (sn * dx + cs * dy);
            extent[0] = extent[0].min(xx);
            extent[1] = extent[1].min(yy);
            extent[2] = extent[2].max(xx);
            extent[3] = extent[3].max(yy);
        }
    }
    for y in ((extent[1].floor() as i32 - 2).clamp(0, 256))
        ..((extent[3].ceil() as i32 + 2).clamp(0, 256))
    {
        for x in ((extent[0].floor() as i32 - 2).clamp(0, 256))
            ..((extent[2].ceil() as i32 + 2).clamp(0, 256))
        {
            let dx = x as f32 + 0.5 - t.target[0];
            let dy = y as f32 + 0.5 - t.target[1];
            let sx = t.pivot[0] + (cs * dx + sn * dy) / scale - 0.5;
            let sy = t.pivot[1] + (-sn * dx + cs * dy) / scale - 0.5;
            let ix = sx.floor() as i32;
            let iy = sy.floor() as i32;
            let fx = sx - ix as f32;
            let fy = sy - iy as f32;
            let mut sum = [0f32; 4];
            for (xx, yy, weight) in [
                (ix, iy, (1. - fx) * (1. - fy)),
                (ix + 1, iy, fx * (1. - fy)),
                (ix, iy + 1, (1. - fx) * fy),
                (ix + 1, iy + 1, fx * fy),
            ] {
                if !(0..i32::from(source.width)).contains(&xx)
                    || !(0..i32::from(source.height)).contains(&yy)
                {
                    continue;
                }
                let xx = if t.flip {
                    i32::from(source.width) - 1 - xx
                } else {
                    xx
                };
                let at = (yy as usize * usize::from(source.width) + xx as usize) * 4;
                let a = f32::from(source.bytes[at + 3]) * weight;
                sum[3] += a;
                for c in 0..3 {
                    sum[c] += f32::from(source.bytes[at + c]) * a;
                }
            }
            if sum[3] > 0. {
                let mut pixel = [0u8; 4];
                for c in 0..3 {
                    pixel[c] = (sum[c] / sum[3]).round().clamp(0., 255.) as u8;
                }
                pixel[3] = sum[3].round().clamp(0., 255.) as u8;
                let at = (y as usize * 256 + x as usize) * 4;
                blend(&mut dest.bytes[at..at + 4], &pixel);
            }
        }
    }
}
pub(super) fn compact(frames: &[Image], anchor: [f32; 2]) -> PreparedImage {
    let mut b = [256usize, 256, 0, 0];
    for frame in frames {
        for (i, p) in frame.bytes.chunks_exact(4).enumerate() {
            if p[3] > 0 {
                let x = i % 256;
                let y = i / 256;
                b[0] = b[0].min(x);
                b[1] = b[1].min(y);
                b[2] = b[2].max(x + 1);
                b[3] = b[3].max(y + 1);
            }
        }
    }
    b[0] = b[0].saturating_sub(1);
    b[1] = b[1].saturating_sub(1);
    b[2] = (b[2] + 1).min(256);
    b[3] = (b[3] + 1).min(256);
    let (w, h) = (b[2] - b[0], b[3] - b[1]);
    let mut image = Image::gen_image_color((w * 6) as u16, (h * 3) as u16, BLANK);
    for (n, frame) in frames.iter().enumerate() {
        for y in 0..h {
            let a = ((b[1] + y) * 256 + b[0]) * 4;
            let dest = ((n / 6 * h + y) * w * 6 + n % 6 * w) * 4;
            image.bytes[dest..dest + w * 4].copy_from_slice(&frame.bytes[a..a + w * 4]);
        }
    }
    PreparedImage {
        image,
        cell: [w as u16, h as u16],
        anchor: [anchor[0] - b[0] as f32, anchor[1] - b[1] as f32],
    }
}
