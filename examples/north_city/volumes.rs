//! Native closed urban volumes, projected from actual blocked footprints.
//! Ground axes stay square; all roofs share one elevation, not a screen strip.
use crate::{asset_preparation, model, scene};
use macroquad::prelude::*;
pub const HEIGHT: i32 = 96;
pub const SOURCE: &str = "assets/source-art/north-city-01/style07/VOLUME07-01-r02-source.png";
pub const BLOCK_SOURCE: &str = "assets/source-art/north-city-01/style07/ILOT07-B01-r02-source.png";
pub const BLOCK_HEIGHT: i32 = 138;
const HOME: &str =
    "assets/source-art/first-sector-review-01/u01-expansion-r06/house-a-body-native.png";
const ROOF: &str = "assets/maps/prison-street-expanded-01/roof-0.png";
pub struct Volumes {
    pub elevation: i32,
    pub width: i32,
    pub height: i32,
    pub mask: Vec<bool>,
    fronts: Vec<Vec<i32>>,
    props: std::collections::BTreeSet<[i32; 2]>,
}
impl Volumes {
    pub fn new(zone: &model::Zone, scene: &scene::Scene) -> Self {
        let d = &scene.document;
        let mut mask = vec![false; d.floors.len()];
        for f in &zone.fabric {
            for &[x, y] in &f.cells {
                mask[(y * d.width + x) as usize] = true;
            }
        }
        for b in zone.buildings.iter().filter(|b| !b.accessible) {
            let [l, t, r, bb] = b.rect;
            for y in t..=bb {
                for x in l..=r {
                    mask[(y * d.width + x) as usize] = true;
                }
            }
        }
        Self::from_mask(d.width, d.height, mask)
    }
    pub fn rectangle(rect: [i32; 4]) -> Self {
        let [l, t, r, b] = rect;
        let mut p = Self::from_mask(r + 1, b + 1, vec![false; ((r + 1) * (b + 1)) as usize]);
        for y in t..=b {
            for x in l..=r {
                p.mask[(y * p.width + x) as usize] = true;
            }
        }
        Self::from_mask(p.width, p.height, p.mask)
    }
    pub fn with_elevation(mut self, elevation: i32) -> Self {
        self.elevation = elevation;
        self
    }
    fn from_mask(width: i32, height: i32, mask: Vec<bool>) -> Self {
        let mut p = Self {
            elevation: HEIGHT,
            width,
            height,
            mask,
            fronts: vec![vec![]; width as usize],
            props: std::collections::BTreeSet::new(),
        };
        for y in 0..height {
            for x in 0..width {
                if p.cell(x, y) && !p.cell(x, y + 1) {
                    p.fronts[x as usize].push((y + 1) * 64);
                }
                if x % 6 == 2
                    && y % 6 == 2
                    && (-1..=1).all(|dy| (-1..=2).all(|dx| p.cell(x + dx, y + dy)))
                {
                    p.props.insert([x * 64, y * 64 - HEIGHT]);
                }
            }
        }
        p
    }
    pub fn cell(&self, x: i32, y: i32) -> bool {
        x >= 0
            && y >= 0
            && x < self.width
            && y < self.height
            && self.mask[(y * self.width + x) as usize]
    }
    pub fn solid(&self, x: i32, y: i32) -> bool {
        self.cell(x.div_euclid(64), y.div_euclid(64))
    }
    pub fn surface_depth(&self, x: i32, y: i32) -> Option<i32> {
        if self.solid(x, y + self.elevation) {
            return Some(y + self.elevation);
        }
        if x < 0 || x >= self.width * 64 {
            return None;
        }
        self.fronts[x as usize / 64]
            .iter()
            .rev()
            .copied()
            .find(|&e| e > y && e <= y + self.elevation)
    }
    pub fn pixel(&self, paint: &Paint, x: i32, y: i32) -> Option<[u8; 4]> {
        if self.solid(x, y + HEIGHT) {
            let gy = y + HEIGHT;
            let distance = (1..=18).find(|&d| {
                !self.solid(x - d, gy)
                    || !self.solid(x + d, gy)
                    || !self.solid(x, gy - d)
                    || !self.solid(x, gy + d)
            });
            let mut p = match distance {
                Some(d) if d <= 3 || d == 18 => [11, 14, 18, 255],
                Some(4) => [175, 171, 153, 255],
                Some(d) => sample(&paint.cap, x, d + 2),
                None => {
                    let mut p = sample(&paint.roof, mirror(x, 64), mirror(gy, 32));
                    // Native roofing joints, never a tiled crop containing
                    // a coping strip or a roof appliance.
                    if x.rem_euclid(128) < 2 || gy.rem_euclid(96) < 2 {
                        p = [49, 50, 42, 255];
                    }
                    p
                }
            };
            // Complete native fan/duct units are stamped only where the
            // complete support is on a roof; never onto a vertical facade.
            let px = x.div_euclid(384) * 384 + 128;
            let py = (y + HEIGHT).div_euclid(384) * 384 + 128 - HEIGHT;
            if x >= px && x < px + 141 && y >= py && y < py + 78 && self.props.contains(&[px, py]) {
                p = sample(&paint.vent, x - px, y - py);
            }
            return Some(p);
        }
        let edge = self.surface_depth(x, y)?;
        let down = y - (edge - HEIGHT);
        let phase = x.rem_euclid(128);
        let p = if down < 3 || down > HEIGHT - 3 {
            [11, 14, 18, 255]
        } else if down < 9 {
            sample(&paint.cap, x, down)
        } else if down >= HEIGHT - 11 {
            [49, 51, 51, 255]
        } else if phase < 8 {
            if phase < 2 {
                [18, 21, 23, 255]
            } else {
                sample(&paint.cap, phase, 5)
            }
        } else if (32..96).contains(&phase) && (18..78).contains(&down) {
            sample(&paint.window, phase - 32, down - 18)
        } else {
            sample(&paint.brick, x, down)
        };
        Some(p)
    }
}
pub fn block_image() -> Result<Image, String> {
    let source = asset_preparation::image(BLOCK_SOURCE)?;
    // Complete whole-object conditioning. One uniform native scale, height
    // rounded to the nearest pixel. The facade baseline stays at ground.
    Ok(cut(&source, [57, 15, 1472, 968], [1280, 842]))
}
pub struct Paint {
    roof: Image,
    cap: Image,
    brick: Image,
    window: Image,
    vent: Image,
}
impl Paint {
    pub fn load() -> Result<Self, String> {
        let home = asset_preparation::image(HOME)?;
        let roof = asset_preparation::image(ROOF)?;
        let source = asset_preparation::image(SOURCE)?;
        Ok(Self {
            roof: cut(&roof, [312, 164, 64, 32], [64, 32]),
            cap: cut(&home, [64, 38, 64, 6], [64, 6]),
            brick: cut(&home, [365, 302, 64, 24], [64, 24]),
            window: cut(&source, [240, 810, 240, 225], [64, 60]),
            vent: cut(&roof, [293, 48, 141, 78], [141, 78]),
        })
    }
    pub fn bytes(&self) -> usize {
        [&self.roof, &self.cap, &self.brick, &self.window, &self.vent]
            .iter()
            .map(|i| i.bytes.len())
            .sum()
    }
}
fn mirror(x: i32, n: i32) -> i32 {
    let v = x.rem_euclid(n * 2);
    if v < n { v } else { n * 2 - 1 - v }
}
fn sample(i: &Image, x: i32, y: i32) -> [u8; 4] {
    let x = x.rem_euclid(i.width as i32) as usize;
    let y = y.rem_euclid(i.height as i32) as usize;
    i.bytes[(y * i.width as usize + x) * 4..][..4]
        .try_into()
        .unwrap()
}
fn cut(source: &Image, [l, t, w, h]: [usize; 4], [dw, dh]: [usize; 2]) -> Image {
    let mut out = Image {
        width: dw as u16,
        height: dh as u16,
        bytes: vec![0; dw * dh * 4],
    };
    for y in 0..dh {
        for x in 0..dw {
            let p = sample(
                source,
                (l + (2 * x + 1) * w / (2 * dw)) as i32,
                (t + (2 * y + 1) * h / (2 * dh)) as i32,
            );
            out.bytes[(y * dw + x) * 4..][..4].copy_from_slice(&p);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn whole_block_art_keeps_its_footprint_and_ground_baseline() {
        let image = block_image().unwrap();
        let rect = [12, 92, 31, 102];
        let p = Volumes::rectangle(rect).with_elevation(BLOCK_HEIGHT);
        assert_eq!(image.width as i32, (rect[2] - rect[0] + 1) * 64);
        assert_eq!(
            image.height as i32,
            (rect[3] - rect[1] + 1) * 64 + p.elevation
        );
        let ground = (rect[3] + 1) * 64;
        assert_eq!(p.surface_depth(20 * 64, ground - 1), Some(ground));
        assert_eq!(p.surface_depth(20 * 64, ground), None);
        assert_eq!(
            p.surface_depth(20 * 64, rect[1] * 64 - p.elevation),
            Some(rect[1] * 64)
        );
    }
    #[test]
    fn roof_front_and_ground_have_one_elevation() {
        let p = Volumes::rectangle([2, 3, 7, 6]);
        assert_eq!(p.surface_depth(3 * 64, 3 * 64 - HEIGHT), Some(3 * 64));
        assert_eq!(p.surface_depth(3 * 64, 7 * 64 - HEIGHT + 1), Some(7 * 64));
        assert_eq!(p.surface_depth(3 * 64, 7 * 64), None);
        assert_eq!(p.surface_depth(64, 4 * 64), None);
    }
    #[test]
    fn city_volume_footprints_are_blocked_and_do_not_swallow_rooms_or_passages() {
        let (r, s) = model::load(20261105).unwrap();
        let p = Volumes::new(&r.zones[0], &s[0]);
        for y in 0..p.height {
            for x in 0..p.width {
                if p.cell(x, y) {
                    assert!(
                        !s[0]
                            .game
                            .map()
                            .is_walkable(project_rl::world::GridPos::new(x, y))
                    );
                }
            }
        }
        for b in r.zones[0].buildings.iter().filter(|b| b.accessible) {
            for &[x, y] in &b.doors {
                assert!(!p.cell(x, y));
            }
        }
    }
    #[test]
    fn a_roof_occludes_a_ground_actor_behind_it_but_not_one_in_front() {
        let p = Volumes::rectangle([2, 3, 7, 6]);
        assert!(p.surface_depth(3 * 64, 3 * 64 - 32).unwrap() > 3 * 64 - 1);
        assert!(p.surface_depth(3 * 64, 6 * 64).unwrap() < 8 * 64);
    }
}
