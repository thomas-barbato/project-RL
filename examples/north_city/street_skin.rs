//! CITY-NORTH01-r03: proposed painted skin, with exact cardinal wall ports.
//! Geometry, placements, IDs, collisions and simulation never depend on this.
use crate::asset_preparation::{self, Builder, PreparedImage};
use macroquad::prelude::*;
use project_rl::world::GridPos;

pub const MATERIAL_SOURCE: &str = "assets/source-art/north-city-01/skin-r03/materials-source.png";
pub const FURNITURE_SOURCE: &str = "assets/source-art/north-city-01/skin-r03/furniture-source.png";
pub const SERVICE_SOURCE: &str = "assets/source-art/north-city-01/skin-r03/service-source.png";
pub const FIRST_WALL_SOURCE: &str =
    "assets/source-art/first-sector-review-01/u01-expansion-r06/house-a-body-native.png";
pub const PAVEMENT_SOURCE: &str = "assets/source-art/floor-continuous-01/material-source.png";
pub const ROAD_SOURCE: &str =
    "assets/source-art/first-sector-review-01/u01-repairs-r09/ROAD-U01-09-native.png";
const SERVICES: [&str; 9] = [
    "north-city-style03:service0",
    "north-city-style03:service1",
    "north-city-style03:service2",
    "north-city-style03:service3",
    "north-city-style03:service4",
    "north-city-style03:service5",
    "north-city-style03:service6",
    "north-city-style03:service7",
    "north-city-style03:service8",
];
pub const WALLS: &str = "north-city-style03:walls";
pub const FLOORS: [&str; 9] = [
    "north-city-style03:floor0",
    "north-city-style03:floor1",
    "north-city-style03:floor2",
    "north-city-style03:floor3",
    "north-city-style03:floor4",
    "north-city-style03:floor5",
    "north-city-style03:floor6",
    "north-city-style03:floor7",
    "north-city-style03:floor8",
];
pub const OBJECTS: [&str; 16] = [
    "north-city-style03:object0",
    "north-city-style03:object1",
    "north-city-style03:object2",
    "north-city-style03:object3",
    "north-city-style03:object4",
    "north-city-style03:object5",
    "north-city-style03:object6",
    "north-city-style03:object7",
    "north-city-style03:object8",
    "north-city-style03:object9",
    "north-city-style03:object10",
    "north-city-style03:object11",
    "north-city-style03:object12",
    "north-city-style03:object13",
    "north-city-style03:object14",
    "north-city-style03:object15",
];
const SIZES: [usize; 9] = [256, 256, 128, 192, 128, 256, 128, 128, 128];
pub const WALL_HEIGHT: usize = 88;
const DEPTH: usize = 18;
pub const PORT_START: usize = 18;
pub const PORT_END: usize = 46;

pub fn keys() -> impl Iterator<Item = &'static str> {
    FLOORS
        .into_iter()
        .chain(OBJECTS)
        .chain(SERVICES)
        .chain([WALLS])
}
fn rgba(image: &Image, x: usize, y: usize) -> [u8; 4] {
    image.bytes[(y * image.width as usize + x) * 4..][..4]
        .try_into()
        .unwrap()
}
fn set(image: &mut Image, x: usize, y: usize, value: [u8; 4]) {
    let index = (y * image.width as usize + x) * 4;
    image.bytes[index..index + 4].copy_from_slice(&value);
}
fn blank(w: usize, h: usize) -> Image {
    Image {
        width: w as u16,
        height: h as u16,
        bytes: vec![0; w * h * 4],
    }
}
fn scaled(pixel: [u8; 4], gain: [f32; 3]) -> [u8; 4] {
    [
        (pixel[0] as f32 * gain[0]).min(255.) as u8,
        (pixel[1] as f32 * gain[1]).min(255.) as u8,
        (pixel[2] as f32 * gain[2]).min(255.) as u8,
        pixel[3],
    ]
}
fn panel_sample(source: &Image, index: usize, native_x: usize, native_y: usize) -> [u8; 4] {
    let size = SIZES[index];
    let width = source.width as usize / 3;
    let height = source.height as usize / 3;
    // The original atlas remains intact. Ignore only its separating frame.
    let x = index % 3 * width + 6 + ((native_x % size) * 2 + 1) * (width - 12) / (size * 2);
    let y = index / 3 * height + 6 + ((native_y % size) * 2 + 1) * (height - 12) / (size * 2);
    let mut p = rgba(source, x, y);
    p[3] = 255;
    p
}
pub fn surface(source: &Image, index: usize) -> Image {
    if index == 4 {
        // Indoor slate: actual 64 px flagstones, with quiet centres and thin
        // joints. Repeating a mirrored painting of six huge damaged slabs
        // made checker-like stains and changed the apparent room scale.
        let mut image = blank(256, 256);
        for y in 0..256 {
            for x in 0..256 {
                let (lx, ly) = (x % 64, y % 64);
                let sample = panel_sample(
                    source,
                    5,
                    80 + lx / 2 + (x / 64) * 9,
                    70 + ly / 2 + (y / 64) * 11,
                );
                let edge = lx == 0 || ly == 0 || (lx < 3 && ly < 3 && lx + ly < 3);
                let bevel = lx == 1 || ly == 1;
                let mut color = scaled(sample, [0.68, 0.77, 0.84]);
                if edge {
                    color = [28, 33, 37, 255];
                } else if bevel {
                    for c in &mut color[..3] {
                        *c = c.saturating_add(9);
                    }
                }
                set(&mut image, x, y, color);
            }
        }
        return image;
    }
    let n = SIZES[index];
    let mut image = blank(n * 2, n * 2);
    for y in 0..n * 2 {
        for x in 0..n * 2 {
            let sx = if x < n { x } else { 2 * n - 1 - x };
            let sy = if y < n { y } else { 2 * n - 1 - y };
            set(&mut image, x, y, panel_sample(source, index, sx, sy));
        }
    }
    image
}
pub fn floor(index: usize) -> Result<Image, String> {
    if index > 1 {
        let source = asset_preparation::image(MATERIAL_SOURCE)?;
        return Ok(surface(&source, index));
    }
    // Exact first-city exterior materials at their accepted native scale.
    // Mirroring spans many logical cells and never draws a tile grid.
    let n = if index == 0 { 512 } else { 256 };
    let source = asset_preparation::image(if index == 0 {
        PAVEMENT_SOURCE
    } else {
        ROAD_SOURCE
    })?;
    let mut image = blank(n * 2, n * 2);
    for y in 0..n * 2 {
        for x in 0..n * 2 {
            let sx = if x < n { x } else { 2 * n - 1 - x };
            let sy = if y < n { y } else { 2 * n - 1 - y };
            set(
                &mut image,
                x,
                y,
                rgba(
                    &source,
                    (2 * sx + 1) * source.width as usize / (2 * n),
                    (2 * sy + 1) * source.height as usize / (2 * n),
                ),
            );
        }
    }
    Ok(image)
}
fn object_size(index: usize) -> (usize, usize) {
    match index {
        2 | 13 => (128, 64),
        5 | 10 => (64, 128),
        _ => (64, 64),
    }
}
fn object(source: &Image, index: usize, grid: usize) -> Result<Image, String> {
    let x0 = index % grid * source.width as usize / grid;
    let x1 = (index % grid + 1) * source.width as usize / grid;
    let y0 = index / grid * source.height as usize / grid;
    let y1 = (index / grid + 1) * source.height as usize / grid;
    let (mut l, mut t, mut r, mut b) = (x1, y1, x0, y0);
    for y in y0..y1 {
        for x in x0..x1 {
            if rgba(source, x, y)[3] > 96 {
                l = l.min(x);
                r = r.max(x + 1);
                t = t.min(y);
                b = b.max(y + 1);
            }
        }
    }
    if r <= l || b <= t {
        return Err(format!("Empty proposed object {index}"));
    }
    let (w, h) = if grid == 4 {
        object_size(index)
    } else {
        (64, 64)
    };
    let (max_width, max_height) = if grid == 4
        && index == 5
        && !std::env::args_os().any(|arg| arg == "--lits-avant-r01")
    {
        (44, 96)
    } else {
        (w - 6, h - 6)
    };
    let gain = (max_width as f32 / (r - l) as f32).min(max_height as f32 / (b - t) as f32);
    let rw = ((r - l) as f32 * gain).round() as usize;
    let rh = ((b - t) as f32 * gain).round() as usize;
    let mut result = blank(w, h);
    for y in 0..rh {
        for x in 0..rw {
            let sx = l + (2 * x + 1) * (r - l) / (2 * rw);
            let sy = t + (2 * y + 1) * (b - t) / (2 * rh);
            let mut p = rgba(source, sx, sy);
            if p[3] < 24 {
                p = [0; 4];
            }
            set(&mut result, x + (w - rw) / 2, y + h - rh - 3, p);
        }
    }
    Ok(result)
}

/// Every occupied port reaches its neighbour with exactly the same pixels.
/// N/E/S/W = bits 1/2/4/8. Corners are actual L-shaped top surfaces.
pub fn occupied(mask: u8, x: i32, y: i32) -> bool {
    let s = PORT_START as i32;
    let e = PORT_END as i32;
    if x < 0 {
        return mask & 8 != 0 && (s..e).contains(&y);
    }
    if x >= 64 {
        return mask & 2 != 0 && (s..e).contains(&y);
    }
    if y < 0 {
        return mask & 1 != 0 && (s..e).contains(&x);
    }
    if y >= 64 {
        return mask & 4 != 0 && (s..e).contains(&x);
    }
    let corner_post = matches!(mask, 3 | 6 | 9 | 12)
        && (12..52).contains(&x)
        && (12..52).contains(&y)
        && !((x < 15 && y < 15 && x + y < 27)
            || (x >= 49 && y < 15 && (63 - x) + y < 27)
            || (x < 15 && y >= 49 && x + (63 - y) < 27)
            || (x >= 49 && y >= 49 && (63 - x) + (63 - y) < 27));
    corner_post
        || ((s..e).contains(&x) && (s..e).contains(&y))
        || (mask & 1 != 0 && (s..e).contains(&x) && y < s)
        || (mask & 4 != 0 && (s..e).contains(&x) && y >= e)
        || (mask & 8 != 0 && (s..e).contains(&y) && x < s)
        || (mask & 2 != 0 && (s..e).contains(&y) && x >= e)
}
pub fn wall_atlas(source: &Image) -> Image {
    let accepted =
        asset_preparation::image(FIRST_WALL_SOURCE).expect("first-city native wall reference");
    let mut result = blank(16 * 64, 4 * 2 * WALL_HEIGHT);
    for family in 0..4 {
        for mask in 0..16u8 {
            let left = mask as usize * 64;
            let face_y = family * 2 * WALL_HEIGHT;
            let top_y = face_y + WALL_HEIGHT;
            // Front faces and shadows are separate from tops: neighbouring caps
            // cover their shared edges, without seams or a post at every tile.
            for y in 0..64 {
                for x in 0..64 {
                    if !occupied(mask, x, y) || occupied(mask, x, y + 1) {
                        continue;
                    }
                    for d in 1..=DEPTH + 4 {
                        let at_y = y as usize + d;
                        if at_y >= WALL_HEIGHT {
                            continue;
                        }
                        let p = if family == 3 && d <= DEPTH && d % 6 != 0 && x % 6 != 0 {
                            [0; 4]
                        } else if d > DEPTH {
                            [4, 7, 11, ((DEPTH + 5 - d) * 14) as u8]
                        } else if d == DEPTH {
                            [17, 20, 24, 255]
                        } else {
                            let panel = match family {
                                0 => 7,
                                1 => 5,
                                _ => 6,
                            };
                            let base = if family == 0 {
                                rgba(&accepted, 365 + x as usize % 64, 302 + (d - 1) % 24)
                            } else {
                                panel_sample(source, panel, x as usize, (d + 42) % SIZES[panel])
                            };
                            let vertical_gain = 0.80 - 0.25 * d as f32 / DEPTH as f32;
                            let mut color =
                                scaled(base, [vertical_gain, vertical_gain, vertical_gain]);
                            if x == 0 || x == 63 {
                                color = match family {
                                    0 => scaled([95, 65, 49, 255], [vertical_gain; 3]),
                                    1 => scaled([114, 110, 100, 255], [vertical_gain; 3]),
                                    _ => scaled([65, 75, 86, 255], [vertical_gain; 3]),
                                };
                            }
                            color
                        };
                        set(&mut result, left + x as usize, face_y + at_y, p);
                    }
                }
            }
            for y in 0..64 {
                for x in 0..64 {
                    if !occupied(mask, x, y) {
                        continue;
                    }
                    let edge = [
                        (-1, 0),
                        (1, 0),
                        (0, -1),
                        (0, 1),
                        (-2, 0),
                        (2, 0),
                        (0, -2),
                        (0, 2),
                    ]
                    .iter()
                    .any(|&(dx, dy)| !occupied(mask, x + dx, y + dy));
                    let light_edge = !occupied(mask, x, y - 3) || !occupied(mask, x - 3, y);
                    let panel = if family == 0 { 5 } else { 6 };
                    let mut color = if family <= 2 {
                        rgba(&accepted, 64 + x as usize % 64, 38 + y as usize % 6)
                    } else {
                        panel_sample(source, panel, x as usize + 20, y as usize + 22)
                    };
                    color = scaled(
                        color,
                        if family == 0 {
                            [1.04, 1.02, 0.96]
                        } else {
                            [1.18, 1.18, 1.16]
                        },
                    );
                    if family == 1 {
                        color = scaled(color, [1.15, 1.11, 1.04]);
                    }
                    if x == 0 || x == 63 || y == 0 || y == 63 {
                        color = match family {
                            0 => [113, 112, 104, 255],
                            1 => [89, 94, 98, 255],
                            _ => [74, 84, 95, 255],
                        };
                    }
                    let wire = (x + y) % 8 == 0 || (x - y + 64) % 8 == 0;
                    let p = if edge {
                        [19, 23, 27, 255]
                    } else if light_edge {
                        [
                            color[0].saturating_add(25),
                            color[1].saturating_add(25),
                            color[2].saturating_add(22),
                            255,
                        ]
                    } else if family == 3 {
                        let across = if x == 0 || x == 63 { y } else { x };
                        let port = x == 0 || x == 63 || y == 0 || y == 63;
                        if if port {
                            (across - PORT_START as i32) % 8 == 0
                        } else {
                            wire
                        } {
                            [62, 70, 77, 200]
                        } else {
                            [0; 4]
                        }
                    } else {
                        color
                    };
                    set(&mut result, left + x as usize, top_y + y as usize, p);
                }
            }
            if matches!(mask, 3 | 6 | 9 | 12) {
                for (bx, by) in [(17, 17), (46, 17), (17, 46), (46, 46)] {
                    set(&mut result, left + bx, top_y + by, [25, 29, 32, 255]);
                    set(&mut result, left + bx + 1, top_y + by, [48, 53, 57, 255]);
                    set(&mut result, left + bx, top_y + by + 1, [48, 53, 57, 255]);
                }
            }
        }
    }
    result
}
pub fn builder(key: &str) -> Option<Builder> {
    if key == WALLS {
        return Some(Box::new(|| {
            let source = asset_preparation::image(MATERIAL_SOURCE)?;
            Ok(PreparedImage {
                image: wall_atlas(&source),
                cell: [64, 64],
                anchor: [32., 32.],
            })
        }));
    }
    if let Some(index) = FLOORS.iter().position(|&k| k == key) {
        return Some(Box::new(move || {
            Ok(PreparedImage {
                image: floor(index)?,
                cell: [64, 64],
                anchor: [32., 32.],
            })
        }));
    }
    if let Some(index) = OBJECTS.iter().position(|&k| k == key) {
        return Some(Box::new(move || {
            let source = asset_preparation::image(FURNITURE_SOURCE)?;
            Ok(PreparedImage {
                image: object(&source, index, 4)?,
                cell: [64, 64],
                anchor: [32., 32.],
            })
        }));
    }
    if let Some(index) = SERVICES.iter().position(|&k| k == key) {
        return Some(Box::new(move || {
            let source = asset_preparation::image(SERVICE_SOURCE)?;
            Ok(PreparedImage {
                image: object(&source, index, 3)?,
                cell: [64, 64],
                anchor: [32., 32.],
            })
        }));
    }
    None
}
pub fn material_slot(material: usize) -> usize {
    match material {
        17 => 1,
        18 => 0,
        22 | 24 => 3,
        23 => 4,
        25 => 2,
        26 => 5,
        99 => 8,
        _ => 5,
    }
}
pub fn ground(material: usize, at: GridPos, origin: Vec2, cell: f32, patch: Rect) {
    let slot = material_slot(material);
    let n = SIZES[slot] * 2;
    let source = Rect::new(
        (at.x * 64).rem_euclid(n as i32) as f32 + patch.x,
        (at.y * 64).rem_euclid(n as i32) as f32 + patch.y,
        patch.w,
        patch.h,
    );
    let color = if material == 99 {
        Color::new(0.61, 0.67, 0.73, 1.)
    } else {
        Color::new(0.77, 0.81, 0.88, 1.)
    };
    asset_preparation::with_texture(FLOORS[slot], |p| {
        draw_texture_ex(
            &p.texture,
            origin.x + patch.x * cell / 64.,
            origin.y + patch.y * cell / 64.,
            color,
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(patch.size() * cell / 64.),
                ..Default::default()
            },
        )
    });
}
pub fn draw_wall(mask: u8, style: usize, at: Vec2, cell: f32, top: bool) {
    let family = wall_family(style);
    let source = Rect::new(
        mask as f32 * 64.,
        (family * 2 * WALL_HEIGHT + if top { WALL_HEIGHT } else { 0 }) as f32,
        64.,
        WALL_HEIGHT as f32,
    );
    asset_preparation::with_texture(WALLS, |p| {
        draw_texture_ex(
            &p.texture,
            at.x,
            at.y,
            Color::new(0.84, 0.88, 0.94, 1.),
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(vec2(cell, WALL_HEIGHT as f32 * cell / 64.)),
                ..Default::default()
            },
        )
    });
}
pub fn wall_family(style: usize) -> usize {
    match style {
        1 => 0,
        2 => 1,
        7 => 3,
        _ => 2,
    }
}
fn object_index(sprite: usize) -> Option<usize> {
    Some(match sprite {
        12 => 0,
        1 => 1,
        4 => 2,
        14 => 3,
        15 => 4,
        72..=75 => 5,
        16 => 6,
        17 | 86 => 7,
        18 => 8,
        19 | 85 => 9,
        24 => 10,
        25 => 11,
        26 => 12,
        65 | 8 => 13,
        23 => 14,
        111 | 34 => 15,
        _ => return None,
    })
}
fn service_index(sprite: usize) -> Option<usize> {
    Some(match sprite {
        11 => 0,
        28 => 1,
        31 => 2,
        80 => 3,
        81 => 4,
        92 => 5,
        94 => 6,
        97 => 7,
        119 => 8,
        _ => return None,
    })
}
pub fn supports(sprite: usize) -> bool {
    object_index(sprite).is_some() || service_index(sprite).is_some()
}
pub fn draw_prop(prop: &crate::scene::Prop, at: Vec2, cell: f32) -> bool {
    if !prop.furniture {
        return false;
    }
    let (key, w, h, lamp) = if let Some(index) = object_index(prop.sprite) {
        let (w, h) = object_size(index);
        (OBJECTS[index], w, h, index == 10)
    } else if let Some(index) = service_index(prop.sprite) {
        (SERVICES[index], 64, 64, false)
    } else {
        return false;
    };
    // One-cell lamp collision, two-cell painted height; its foot remains at
    // the same anchor. All actual prop footprints and rotations stay intact.
    let factor = if lamp {
        1.
    } else {
        (prop.details.width as f32 * 64. / w as f32)
            .min(prop.details.height as f32 * 64. / h as f32)
    } * prop.details.scale as f32
        / 100.;
    let size = vec2(w as f32, h as f32) * factor * cell / 64.;
    let (pw, ph) = prop.size();
    let center = at
        + vec2(pw as f32, ph as f32) * cell / 2.
        + vec2(prop.details.offset_x as f32, prop.details.offset_y as f32) * cell / 64.;
    let offset = if lamp {
        vec2(0., -cell / 2.)
    } else {
        Vec2::ZERO
    };
    let start = center + offset - size / 2.;
    asset_preparation::with_texture(key, |p| {
        draw_texture_ex(
            &p.texture,
            start.x,
            start.y,
            Color::new(0.80, 0.84, 0.92, 1.),
            DrawTextureParams {
                dest_size: Some(size),
                rotation: prop.rotation as f32 * std::f32::consts::FRAC_PI_2,
                ..Default::default()
            },
        )
    });
    true
}
pub fn draw_door(at: Vec2, cell: f32, vertical: bool, open: bool) {
    let s = cell / 64.;
    let rect = |x: f32, y: f32, w: f32, h: f32, color| {
        draw_rectangle(at.x + x * s, at.y + y * s, w * s, h * s, color)
    };
    let dark = Color::from_hex(0x151a20);
    let steel = Color::from_hex(0x404b56);
    let highlight = Color::from_hex(0x7a8185);
    if vertical {
        rect(17., 0., 30., 6., dark);
        rect(17., 58., 30., 6., dark);
        rect(19., 1., 26., 3., highlight);
        rect(19., 60., 26., 3., highlight);
        if !open {
            rect(18., 5., 28., 54., dark);
            rect(20., 7., 24., 50., steel);
            rect(21., 8., 2., 48., highlight);
            rect(36., 30., 4., 3., Color::from_hex(0xa58a4d));
        } else {
            rect(18., 6., 5., 20., dark);
            rect(19., 7., 2., 17., steel);
        }
    } else {
        // Front faces finish at the same depth as their neighbouring walls.
        rect(0., 46., 7., 18., dark);
        rect(57., 46., 7., 18., dark);
        rect(0., 17., 7., 31., dark);
        rect(57., 17., 7., 31., dark);
        rect(1., 19., 4., 25., highlight);
        rect(59., 19., 4., 25., highlight);
        if !open {
            rect(5., 46., 54., 16., dark);
            rect(7., 47., 50., 11., Color::from_hex(0x303943));
            rect(5., 18., 54., 28., dark);
            rect(7., 20., 50., 23., steel);
            rect(8., 21., 48., 2., highlight);
            rect(46., 35., 3., 4., Color::from_hex(0xa58a4d));
        } else {
            rect(7., 18., 20., 5., dark);
            rect(8., 19., 17., 2., steel);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_map_object_and_police_variant_has_proposed_artwork() {
        let mut variants = std::collections::BTreeSet::new();
        for seed in 1..=8 {
            let (review, scenes) = crate::model::load(seed).unwrap();
            for zone in &review.zones {
                if zone.selected_variant.is_some() {
                    variants.insert(zone.id.clone());
                }
            }
            for scene in scenes {
                for prop in scene.props {
                    assert!(
                        prop.furniture && supports(prop.sprite),
                        "uncovered object {}",
                        prop.sprite
                    );
                }
            }
        }
        assert_eq!(variants.len(), 6);
    }
    #[test]
    fn native_wall_ports_join_for_every_cardinal_pair_and_all_four_corners() {
        for a in 0..16 {
            for b in 0..16 {
                if a & 2 != 0 && b & 8 != 0 {
                    for y in 0..64 {
                        assert_eq!(occupied(a, 63, y), occupied(b, 0, y));
                    }
                }
                if a & 4 != 0 && b & 1 != 0 {
                    for x in 0..64 {
                        assert_eq!(occupied(a, x, 63), occupied(b, x, 0));
                    }
                }
            }
        }
        for mask in [3, 6, 9, 12] {
            assert_eq!(
                (0..64).filter(|&x| occupied(mask, x, -1)).count(),
                if mask & 1 != 0 { 28 } else { 0 }
            );
            assert!(!occupied(mask, 0, 0) && !occupied(mask, 63, 63));
        }
    }
    #[test]
    fn painted_caps_have_identical_pixels_at_matching_ports() {
        let mut source = blank(96, 96);
        for y in 0..96 {
            for x in 0..96 {
                set(&mut source, x, y, [40 + x as u8, 50 + y as u8, 70, 255]);
            }
        }
        let atlas = wall_atlas(&source);
        for family in 0..4 {
            for a in 0..16u8 {
                for b in 0..16u8 {
                    let top = family * 2 * WALL_HEIGHT + WALL_HEIGHT;
                    if a & 2 != 0 && b & 8 != 0 {
                        for y in 0..64 {
                            assert_eq!(
                                rgba(&atlas, a as usize * 64 + 63, top + y),
                                rgba(&atlas, b as usize * 64, top + y)
                            );
                        }
                    }
                    if a & 4 != 0 && b & 1 != 0 {
                        for x in 0..64 {
                            assert_eq!(
                                rgba(&atlas, a as usize * 64 + x, top + 63),
                                rgba(&atlas, b as usize * 64 + x, top)
                            );
                        }
                    }
                }
            }
        }
    }
}
