//! Shared deterministic geometry frozen by HELD-WEAPON-64-01-ACCEPT01.
//! Used while packaging weapons and on a worn-texture cache miss, never per frame.
use macroquad::prelude::{BLANK, Image};
pub(crate) const SIDE: usize = 128;
pub(crate) const HEAVY_HANDS: [[f32; 2]; 2] = [[55.93273, 63.41818], [73.91273, 63.41818]];
pub(crate) const SHOULDERS: [[f32; 2]; 2] = [[53.45455, 54.19091], [74.80909, 54.19091]];
pub(crate) const POLE_HANDS: [[f32; 2]; 2] = [[63.20909, 62.78545], [72.01455, 55.45636]];
// Existing canonical sleeve masks, measured in the original paintings.
pub(crate) const INDEPENDENT: [&[[f32; 2]]; 2] = [
    &[
        [105., 435.],
        [482., 435.],
        [483., 473.],
        [463., 519.],
        [425., 556.],
        [390., 600.],
        [340., 660.],
        [100., 660.],
    ],
    &[
        [770., 435.],
        [1150., 435.],
        [1160., 660.],
        [910., 660.],
        [865., 600.],
        [825., 556.],
        [792., 519.],
        [770., 473.],
    ],
];
pub(crate) const HEAVY: [&[[f32; 2]]; 2] = [
    &[
        [340., 435.],
        [477., 435.],
        [485., 500.],
        [550., 537.],
        [550., 674.],
        [426., 674.],
        [284., 608.],
        [283., 526.],
    ],
    &[
        [778., 435.],
        [919., 435.],
        [960., 526.],
        [966., 608.],
        [894., 674.],
        [742., 674.],
        [742., 537.],
        [779., 500.],
    ],
];

fn blank() -> Image {
    Image::gen_image_color(128, 128, BLANK)
}
pub(crate) fn over(dest: &mut Image, source: &Image) {
    for (d, s) in dest
        .bytes
        .chunks_exact_mut(4)
        .zip(source.bytes.chunks_exact(4))
    {
        let a = u32::from(s[3]);
        let old = u32::from(d[3]) * (255 - a);
        let total = a * 255 + old;
        if total == 0 {
            continue;
        }
        for c in 0..3 {
            d[c] = ((u32::from(s[c]) * a * 255 + u32::from(d[c]) * old + total / 2) / total) as u8;
        }
        d[3] = ((total + 127) / 255) as u8;
    }
}
// Premultiplied bilinear sampling keeps translucent edges free of black fringes.
pub(crate) fn transformed(
    source: &Image,
    pivot: [f32; 2],
    target: [f32; 2],
    scale: f32,
    angle: f32,
) -> Image {
    let (sin, cos) = angle.sin_cos();
    affine(
        source,
        pivot,
        target,
        [cos * scale, -sin * scale, sin * scale, cos * scale],
    )
}
pub(crate) fn affine(source: &Image, pivot: [f32; 2], target: [f32; 2], matrix: [f32; 4]) -> Image {
    let mut dest = blank();
    let [a, b, c, d] = matrix;
    let det = a * d - b * c;
    let mut occupied = [source.width as usize, source.height as usize, 0, 0];
    for (i, pixel) in source.bytes.chunks_exact(4).enumerate() {
        if pixel[3] != 0 {
            let x = i % source.width as usize;
            let y = i / source.width as usize;
            occupied[0] = occupied[0].min(x);
            occupied[1] = occupied[1].min(y);
            occupied[2] = occupied[2].max(x + 1);
            occupied[3] = occupied[3].max(y + 1);
        }
    }
    if occupied[2] == 0 {
        return dest;
    }
    let mut extent = [
        f32::INFINITY,
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::NEG_INFINITY,
    ];
    for x in [occupied[0] as f32 - 1., occupied[2] as f32 + 1.] {
        for y in [occupied[1] as f32 - 1., occupied[3] as f32 + 1.] {
            let px = target[0] + a * (x - pivot[0]) + b * (y - pivot[1]);
            let py = target[1] + c * (x - pivot[0]) + d * (y - pivot[1]);
            extent[0] = extent[0].min(px);
            extent[1] = extent[1].min(py);
            extent[2] = extent[2].max(px);
            extent[3] = extent[3].max(py);
        }
    }
    let first_x = (extent[0].floor() as i32 - 1).clamp(0, SIDE as i32) as usize;
    let first_y = (extent[1].floor() as i32 - 1).clamp(0, SIDE as i32) as usize;
    let last_x = (extent[2].ceil() as i32 + 1).clamp(0, SIDE as i32) as usize;
    let last_y = (extent[3].ceil() as i32 + 1).clamp(0, SIDE as i32) as usize;
    for y in first_y..last_y {
        for x in first_x..last_x {
            let dx = x as f32 + 0.5 - target[0];
            let dy = y as f32 + 0.5 - target[1];
            let sx = pivot[0] + (d * dx - b * dy) / det - 0.5;
            let sy = pivot[1] + (-c * dx + a * dy) / det - 0.5;
            let ix = sx.floor() as i32;
            let iy = sy.floor() as i32;
            let fx = sx - ix as f32;
            let fy = sy - iy as f32;
            let mut sum = [0.; 4];
            for (xx, yy, w) in [
                (ix, iy, (1. - fx) * (1. - fy)),
                (ix + 1, iy, fx * (1. - fy)),
                (ix, iy + 1, (1. - fx) * fy),
                (ix + 1, iy + 1, fx * fy),
            ] {
                if xx < 0
                    || yy < 0
                    || xx >= i32::from(source.width)
                    || yy >= i32::from(source.height)
                {
                    continue;
                }
                let i = (yy as usize * usize::from(source.width) + xx as usize) * 4;
                let a = source.bytes[i + 3] as f32 * w;
                sum[3] += a;
                for c in 0..3 {
                    sum[c] += source.bytes[i + c] as f32 * a;
                }
            }
            let i = (y * SIDE + x) * 4;
            if sum[3] > 0. {
                for c in 0..3 {
                    dest.bytes[i + c] = (sum[c] / sum[3]).round() as u8;
                }
            }
            dest.bytes[i + 3] = sum[3].round() as u8;
        }
    }
    dest
}
fn inside(point: [f32; 2], original: &[[f32; 2]]) -> bool {
    let p = [
        625. + (point[0] / 2. - 32.) * 1100. / 29.,
        1190. + (point[1] / 2. - 47.) * 1100. / 29.,
    ];
    let mut found = false;
    for i in 0..original.len() {
        let a = original[i];
        let b = original[(i + 1) % original.len()];
        if (a[1] > p[1]) != (b[1] > p[1])
            && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0]
        {
            found = !found;
        }
    }
    found
}
pub(crate) fn selected(source: &Image, polygon: &[[f32; 2]], keep_inside: bool) -> Image {
    let mut dest = source.clone();
    for y in 0..SIDE {
        for x in 0..SIDE {
            if source.bytes[(y * SIDE + x) * 4 + 3] == 0 {
                continue;
            }
            if inside([x as f32 + 0.5, y as f32 + 0.5], polygon) != keep_inside {
                dest.bytes[(y * SIDE + x) * 4 + 3] = 0;
            }
        }
    }
    dest
}
pub(crate) fn posed_arm(source: &Image, old: [f32; 2], new: [f32; 2], shoulder: [f32; 2]) -> Image {
    let height = old[1] - shoulder[1];
    affine(
        source,
        shoulder,
        shoulder,
        [
            1.,
            (new[0] - old[0]) / height,
            0.,
            (new[1] - shoulder[1]) / height,
        ],
    )
}

pub(crate) fn compose(
    body: &Image,
    sleeves: &Image,
    fingers: &[Image; 2],
    weapon: &Image,
    hands: [[f32; 2]; 2],
    opposed: bool,
) -> Image {
    let mut result = body.clone();
    for polygon in INDEPENDENT {
        result = selected(&result, polygon, false);
    }
    for side in 0..2 {
        let sleeve = selected(sleeves, HEAVY[side], true);
        over(
            &mut result,
            &posed_arm(&sleeve, HEAVY_HANDS[side], hands[side], SHOULDERS[side]),
        );
    }
    over(&mut result, weapon);
    for side in 0..2 {
        let hand = transformed(
            &fingers[side],
            if opposed {
                POLE_HANDS[side]
            } else {
                HEAVY_HANDS[side]
            },
            hands[side],
            1.,
            if opposed { 0.694 } else { 0. },
        );
        over(&mut result, &hand);
    }
    result
}
