//! Raster preparation for the isolated preview. Source PNGs remain unchanged.
use macroquad::prelude::*;
#[path = "animation.rs"]
mod animation;

pub const TILE: usize = 64;
pub const BAND_START: usize = 8;
pub const BAND_END: usize = 56;
const OUTER_BORDER_END: usize = BAND_START + 20;
const INNER_BORDER_START: usize = BAND_END - 4;

#[derive(Clone)]
pub struct Raster {
    pub bytes: Vec<u8>,
}

impl Raster {
    fn blank() -> Self {
        Self {
            bytes: vec![0; TILE * TILE * 4],
        }
    }

    fn sample(image: &Image, columns: usize, rows: usize, index: usize) -> Self {
        let width = image.width as usize;
        let height = image.height as usize;
        let left = (index % columns) * width / columns;
        let top = (index / columns) * height / rows;
        let right = (index % columns + 1) * width / columns;
        let bottom = (index / columns + 1) * height / rows;
        let mut raster = Self::blank();
        for y in 0..TILE {
            for x in 0..TILE {
                let sx = left + (2 * x + 1) * (right - left) / (2 * TILE);
                let sy = top + (2 * y + 1) * (bottom - top) / (2 * TILE);
                raster.set(
                    x,
                    y,
                    image.bytes[(sy * width + sx) * 4..][..4]
                        .try_into()
                        .unwrap(),
                );
            }
        }
        raster
    }

    fn pixel(&self, x: usize, y: usize) -> [u8; 4] {
        self.bytes[(y * TILE + x) * 4..][..4].try_into().unwrap()
    }

    fn set(&mut self, x: usize, y: usize, pixel: [u8; 4]) {
        self.bytes[(y * TILE + x) * 4..][..4].copy_from_slice(&pixel);
    }

    pub fn rotated(&self, turns: usize) -> Self {
        let mut output = Self::blank();
        for y in 0..TILE {
            for x in 0..TILE {
                let (mut dx, mut dy) = (x, y);
                for _ in 0..turns % 4 {
                    (dx, dy) = (TILE - 1 - dy, dx);
                }
                output.set(dx, dy, self.pixel(x, y));
            }
        }
        output
    }

    fn sprite_cell(mut self) -> Self {
        // Keep the reserved three-pixel atlas gutter clear. The generated care
        // couch slightly spills into the adjacent cabinet's cell otherwise.
        for y in 0..TILE {
            for x in 0..TILE {
                if x < 3 || y < 3 || x >= TILE - 3 || y >= TILE - 3 {
                    self.set(x, y, [0; 4]);
                }
            }
        }
        self
    }

    fn without_fragments(mut self) -> Self {
        // Isolated slivers between generated atlas rows are not part of the
        // furniture. Keep substantial components (including floating screens).
        let mut visited = vec![false; TILE * TILE];
        for index in 0..TILE * TILE {
            if visited[index] || self.bytes[index * 4 + 3] < 16 {
                continue;
            }
            let mut component = vec![];
            let mut queue = vec![index];
            visited[index] = true;
            while let Some(pixel) = queue.pop() {
                component.push(pixel);
                let (x, y) = (pixel % TILE, pixel / TILE);
                for (nx, ny) in [
                    (x as i32 - 1, y as i32),
                    (x as i32 + 1, y as i32),
                    (x as i32, y as i32 - 1),
                    (x as i32, y as i32 + 1),
                ] {
                    if !(0..TILE as i32).contains(&nx) || !(0..TILE as i32).contains(&ny) {
                        continue;
                    }
                    let next = ny as usize * TILE + nx as usize;
                    if !visited[next] && self.bytes[next * 4 + 3] >= 16 {
                        visited[next] = true;
                        queue.push(next);
                    }
                }
            }
            if component.len() < 96 {
                for pixel in component {
                    self.bytes[pixel * 4..][..4].fill(0);
                }
            }
        }
        self
    }

    fn opaque_bounds(&self) -> (usize, usize, usize, usize) {
        let (mut left, mut top, mut right, mut bottom) = (TILE, TILE, 0, 0);
        for y in 0..TILE {
            for x in 0..TILE {
                if self.pixel(x, y)[3] >= 200 {
                    left = left.min(x);
                    top = top.min(y);
                    right = right.max(x + 1);
                    bottom = bottom.max(y + 1);
                }
            }
        }
        assert!(right > left && bottom > top, "empty source sprite");
        (left, top, right, bottom)
    }

    fn copy_rect(
        &mut self,
        source: &Self,
        from: (usize, usize, usize, usize),
        to: (usize, usize, usize, usize),
    ) {
        let (sx, sy, sw, sh) = from;
        let (dx, dy, dw, dh) = to;
        for y in 0..dh {
            for x in 0..dw {
                let mut p = source.pixel(
                    sx + (2 * x + 1) * sw / (2 * dw),
                    sy + (2 * y + 1) * sh / (2 * dh),
                );
                // Remove the very faint generated halo, retain real cutouts.
                p[3] = if p[3] >= 200 { 255 } else { 0 };
                self.set(dx + x, dy + y, p);
            }
        }
    }

    fn band(&self) -> Self {
        let (left, top, right, bottom) = self.opaque_bounds();
        let mut output = Self::blank();
        output.copy_rect(
            self,
            (left, top, right - left, bottom - top),
            (0, BAND_START, TILE, BAND_END - BAND_START),
        );
        output
    }

    fn solid_band(&self, vertical: bool) -> Self {
        let (left, top, right, bottom) = self.opaque_bounds();
        // Ignore the antialiased silhouette and scattered halo pixels when
        // finding the solid rectangular core of a straight piece.
        let row = (top + bottom) / 2;
        let column = (left + right) / 2;
        let xs: Vec<_> = (left..right)
            .filter(|&x| self.pixel(x, row)[3] >= 200)
            .collect();
        let ys: Vec<_> = (top..bottom)
            .filter(|&y| self.pixel(column, y)[3] >= 200)
            .collect();
        let left = xs[0] + 1;
        let right = xs[xs.len() - 1];
        let top = ys[0] + 1;
        let bottom = ys[ys.len() - 1];
        let mut output = Self::blank();
        output.copy_rect(
            self,
            (left, top, right - left, bottom - top),
            if vertical {
                (BAND_START, 0, BAND_END - BAND_START, TILE)
            } else {
                (0, BAND_START, TILE, BAND_END - BAND_START)
            },
        );
        output
    }

    fn corner(&self, mask: u8) -> Self {
        // Fit each separately drawn orientation as a whole. No arm overlays
        // or image rotation: the painted bend and lighting stay intact.
        let (left, top, right, bottom) = self.opaque_bounds();
        let row = if mask & 1 != 0 { top + 1 } else { bottom - 2 };
        let column = if mask & 8 != 0 { left + 1 } else { right - 2 };
        let vertical: Vec<_> = (0..TILE)
            .filter(|&x| self.pixel(x, row)[3] >= 200)
            .collect();
        let horizontal: Vec<_> = (0..TILE)
            .filter(|&y| self.pixel(column, y)[3] >= 200)
            .collect();
        let (vx, vr) = (*vertical.first().unwrap(), vertical.last().unwrap() + 1);
        let (hy, hb) = (*horizontal.first().unwrap(), horizontal.last().unwrap() + 1);
        let mut output = Self::blank();
        let axis = |i: usize,
                    low: usize,
                    band_low: usize,
                    band_high: usize,
                    high: usize,
                    before: bool,
                    after: bool| {
            if i < BAND_START {
                before.then(|| low + (2 * i + 1) * (band_low - low) / (2 * BAND_START))
            } else if i >= BAND_END {
                after.then(|| {
                    (band_high
                        + (2 * (i - BAND_END) + 1) * (high - band_high) / (2 * (TILE - BAND_END)))
                        .min(high - 1)
                })
            } else {
                Some(
                    band_low
                        + (2 * (i - BAND_START) + 1) * (band_high - band_low)
                            / (2 * (BAND_END - BAND_START)),
                )
            }
        };
        for y in 0..TILE {
            for x in 0..TILE {
                let Some(sx) = axis(x, left, vx, vr, right, mask & 8 != 0, mask & 2 != 0) else {
                    continue;
                };
                let Some(sy) = axis(y, top, hy, hb, bottom, mask & 1 != 0, mask & 4 != 0) else {
                    continue;
                };
                let mut pixel = self.pixel(sx, sy);
                pixel[3] = if pixel[3] >= 200 { 255 } else { 0 };
                output.set(x, y, pixel);
            }
        }
        output
    }

    fn texture(&self) -> Texture2D {
        let texture = Texture2D::from_rgba8(TILE as u16, TILE as u16, &self.bytes);
        texture.set_filter(FilterMode::Nearest);
        texture
    }

    fn use_corner_borders(&mut self, corner: &Self) {
        // The corner's straight arm is the reference for both outlines.
        // Its straight strip is outside the bevelled bend and the bolts.
        // Keep the straight wall's panel, but do not crop or stretch its rim
        // independently: that removed the dark outline and widened highlights.
        let strip_start = BAND_START + 24;
        let strip_width = BAND_END - strip_start;
        for y in BAND_START..BAND_END {
            for x in 0..TILE {
                if y < OUTER_BORDER_END || y >= INNER_BORDER_START {
                    let along = strip_start + x % strip_width;
                    let pixel_at = |across| corner.pixel(along, across);
                    let mut pixel = pixel_at(y);
                    // Extend the solid edge colour over a faint alpha notch
                    // in the generated outline, without changing its width.
                    if pixel[3] < 200 {
                        pixel = pixel_at(y.clamp(BAND_START + 1, BAND_END - 2));
                    }
                    self.set(x, y, pixel);
                }
            }
        }
    }

    fn use_top_wall_borders(&mut self, top: &Self) {
        // Use the already prepared upward-facing wall as the sole reference.
        // Rotate its borders exactly, retaining the vertical wall's panel.
        let left = top.rotated(3);
        for x in BAND_START..BAND_END {
            if x < OUTER_BORDER_END || x >= INNER_BORDER_START {
                for y in 0..TILE {
                    self.set(x, y, left.pixel(x, y));
                }
            }
        }
    }

    fn align_corner_border(&mut self, top: &Self, mask: u8) {
        // Work in the upper-left orientation. Treat the entire outer rim,
        // not just its last eight pixels: patching those endpoints left a
        // different bevel (and a white vertical stripe) inside each corner.
        let turns = match mask {
            6 => 0,
            12 => 1,
            3 => 3,
            9 => 2,
            _ => unreachable!(),
        };
        let mut corner = self.rotated((4 - turns) % 4);
        let left = top.rotated(3);
        for y in 0..TILE {
            for x in 0..TILE {
                let north = (BAND_START..OUTER_BORDER_END).contains(&y) && x >= BAND_START;
                let west = (BAND_START..OUTER_BORDER_END).contains(&x) && y >= BAND_START;
                let inner_north = x >= BAND_END && (INNER_BORDER_START..BAND_END).contains(&y);
                let inner_west = y >= BAND_END && (INNER_BORDER_START..BAND_END).contains(&x);
                let pixel = if north && west {
                    let original = corner.pixel(x, y);
                    // Retain a dark painted miter line in the bevel, while
                    // both of its sides follow the common border profile.
                    if x == y
                        && (BAND_START + 2..BAND_START + 11).contains(&x)
                        && original[0..3].iter().all(|&channel| channel < 65)
                    {
                        original
                    } else if y <= x {
                        top.pixel(x, y)
                    } else {
                        left.pixel(x, y)
                    }
                } else if north || inner_north {
                    top.pixel(x, y)
                } else if west || inner_west {
                    left.pixel(x, y)
                } else {
                    continue;
                };
                corner.set(x, y, pixel);
            }
        }
        *self = corner.rotated(turns);
    }

    fn connected(straight: &Self, vertical: &Self, corners: &[Self], mask: u8) -> Self {
        if let Some(index) = super::scene::CORNER_MASKS
            .iter()
            .position(|&corner| corner == mask)
        {
            return corners[index].clone();
        }
        if mask == 10 {
            return straight.clone();
        }
        if mask == 5 {
            return vertical.clone();
        }
        let mut result = Self::blank();
        for y in 0..TILE {
            for x in 0..TILE {
                let center =
                    (BAND_START..BAND_END).contains(&x) && (BAND_START..BAND_END).contains(&y);
                let north = mask & 1 != 0 && y < BAND_END && (BAND_START..BAND_END).contains(&x);
                let east = mask & 2 != 0 && x >= BAND_START && (BAND_START..BAND_END).contains(&y);
                let south = mask & 4 != 0 && y >= BAND_START && (BAND_START..BAND_END).contains(&x);
                let west = mask & 8 != 0 && x < BAND_END && (BAND_START..BAND_END).contains(&y);
                if center || north || east || south || west {
                    result.set(
                        x,
                        y,
                        if north || south {
                            vertical.pixel(x, y)
                        } else {
                            straight.pixel(x, y)
                        },
                    );
                }
            }
        }
        result
    }
}

pub struct WallKit {
    pub perforated: bool,
    pub horizontal: Raster,
    pub vertical: Raster,
    pub corners: Vec<Raster>,
}

impl WallKit {
    fn fence(bytes: &[u8]) -> Self {
        let mut kit = Self::load(bytes);
        kit.perforated = true;
        kit
    }
    fn load(bytes: &[u8]) -> Self {
        let source = Image::from_file_with_format(bytes, None).unwrap();
        let mut horizontal = Raster::sample(&source, 3, 2, 0).solid_band(false);
        let mut vertical = Raster::sample(&source, 3, 2, 3).solid_band(true);
        let mut corners: Vec<_> = [1, 2, 4, 5]
            .into_iter()
            .zip(super::scene::CORNER_MASKS)
            .map(|(index, mask)| Raster::sample(&source, 3, 2, index).corner(mask))
            .collect();
        horizontal.use_corner_borders(&corners[0]);
        vertical.use_top_wall_borders(&horizontal);
        for (corner, mask) in corners.iter_mut().zip(super::scene::CORNER_MASKS) {
            corner.align_corner_border(&horizontal, mask);
        }
        Self {
            perforated: false,
            horizontal,
            vertical,
            corners,
        }
    }
}

pub struct Prepared {
    pub terrain: Vec<Raster>,
    pub walls: Vec<Raster>,
    pub furniture: Vec<Raster>,
    pub corners: Vec<Raster>,
    pub vertical: Raster,
    pub city_walls: Vec<WallKit>,
    pub directions: std::collections::BTreeMap<usize, Vec<Raster>>,
}

impl Prepared {
    pub fn load() -> Self {
        let terrain = Image::from_file_with_format(
            include_bytes!("../../assets/prototypes/surface64/terrain-and-objects-source.png"),
            None,
        )
        .unwrap();
        let continuous_floors = Image::from_file_with_format(
            include_bytes!("../../assets/prototypes/surface64/floors-borderless-source-v2.png"),
            None,
        )
        .unwrap();
        let walls = Image::from_file_with_format(
            include_bytes!("../../assets/prototypes/surface64/walls-source.png"),
            None,
        )
        .unwrap();
        let furniture = Image::from_file_with_format(
            include_bytes!("../../assets/prototypes/surface64/furniture-source.png"),
            None,
        )
        .unwrap();
        let mut floors: Vec<_> = (0..16).map(|i| Raster::sample(&terrain, 4, 4, i)).collect();
        // Only adopt the edited ground cells. Objects and structural sources
        // continue to come from their original files and keep their indices.
        for (index, floor) in floors[..10].iter_mut().enumerate() {
            *floor = Raster::sample(&continuous_floors, 4, 4, index);
            for pixel in floor.bytes.chunks_exact_mut(4) {
                pixel[3] = 255;
            }
        }
        let city_floors = Image::from_file_with_format(
            include_bytes!("../../assets/prototypes/surface64/urban-review/floors-source-v1.png"),
            None,
        )
        .unwrap();
        for index in 0..12 {
            let mut floor = Raster::sample(&city_floors, 4, 3, index);
            for pixel in floor.bytes.chunks_exact_mut(4) {
                pixel[3] = 255;
            }
            floors.push(floor);
        }
        let mut furnishings: Vec<_> = (0..8)
            .map(|i| Raster::sample(&furniture, 4, 2, i))
            .collect();
        for bytes in [
            &include_bytes!(
                "../../assets/prototypes/surface64/urban-review/interiors-source-v1.png"
            )[..],
            &include_bytes!("../../assets/prototypes/surface64/urban-review/street-source-v1.png")
                [..],
        ] {
            let source = Image::from_file_with_format(bytes, None).unwrap();
            furnishings.extend((0..16).map(|i| Raster::sample(&source, 4, 4, i).sprite_cell()));
        }
        let technology = Image::from_file_with_format(
            include_bytes!(
                "../../assets/prototypes/surface64/urban-review/technology-source-v1.png"
            ),
            None,
        )
        .unwrap();
        furnishings.extend((0..16).map(|i| Raster::sample(&technology, 4, 4, i).sprite_cell()));
        for bytes in [
            &include_bytes!("../../assets/prototypes/surface64/sf/servers-source-v1.png")[..],
            &include_bytes!("../../assets/prototypes/surface64/sf/habitat-source-v1.png")[..],
            &include_bytes!("../../assets/prototypes/surface64/sf/armory-source-v1.png")[..],
        ] {
            let source = Image::from_file_with_format(bytes, None).unwrap();
            furnishings.extend((0..16).map(|i| {
                Raster::sample(&source, 4, 4, i)
                    .sprite_cell()
                    .without_fragments()
            }));
        }
        let nature = Image::from_file_with_format(
            include_bytes!("../../assets/prototypes/surface64/nature/nature-source-v1.png"),
            None,
        )
        .unwrap();
        furnishings.extend((0..16).map(|i| Raster::sample(&nature, 4, 4, i).sprite_cell()));
        let wall_sources: Vec<_> = (0..4).map(|i| Raster::sample(&walls, 2, 2, i)).collect();
        let coherent = Image::from_file_with_format(
            include_bytes!("../../assets/prototypes/surface64/walls-and-corners-source-v4.png"),
            None,
        )
        .unwrap();
        let mut horizontal = Raster::sample(&coherent, 3, 2, 0).solid_band(false);
        let mut vertical = Raster::sample(&coherent, 3, 2, 3).solid_band(true);
        let mut corners: Vec<_> = [1, 2, 4, 5]
            .into_iter()
            .zip(super::scene::CORNER_MASKS)
            .map(|(index, mask)| Raster::sample(&coherent, 3, 2, index).corner(mask))
            .collect();
        horizontal.use_corner_borders(&corners[0]);
        vertical.use_top_wall_borders(&horizontal);
        for (corner, mask) in corners.iter_mut().zip(super::scene::CORNER_MASKS) {
            corner.align_corner_border(&horizontal, mask);
        }
        Self {
            directions: {
                let objects=Image::from_file_with_format(include_bytes!("../../assets/prototypes/surface64/directions/furniture-four-directions-v1.png"),None).unwrap();
                let beds = Image::from_file_with_format(
                    include_bytes!(
                        "../../assets/prototypes/surface64/directions/beds-four-directions-v1.png"
                    ),
                    None,
                )
                .unwrap();
                let mut result = std::collections::BTreeMap::new();
                for (sprite, row) in [(4, 0), (1, 2), (65, 3)] {
                    result.insert(
                        sprite,
                        (0..4)
                            .map(|turn| {
                                Raster::sample(&objects, 4, 4, row * 4 + turn)
                                    .sprite_cell()
                                    .without_fragments()
                            })
                            .collect(),
                    );
                }
                for row in 0..4 {
                    result.insert(
                        72 + row,
                        (0..4)
                            .map(|turn| {
                                Raster::sample(&beds, 4, 4, row * 4 + turn)
                                    .sprite_cell()
                                    .without_fragments()
                            })
                            .collect(),
                    );
                }
                result
            },
            terrain: floors,
            walls: vec![
                horizontal,
                corners[3].clone(),
                wall_sources[2].band(),
                wall_sources[3].band(),
            ],
            furniture: furnishings,
            corners,
            vertical,
            city_walls: vec![
                WallKit::load(include_bytes!(
                    "../../assets/prototypes/surface64/urban-review/walls-brick-source-v1.png"
                )),
                WallKit::load(include_bytes!(
                    "../../assets/prototypes/surface64/urban-review/walls-plaster-source-v1.png"
                )),
                WallKit::load(include_bytes!(
                    "../../assets/prototypes/surface64/sf/walls-laser-source-v1.png"
                )),
                WallKit::load(include_bytes!(
                    "../../assets/prototypes/surface64/sf/walls-camera-source-v1.png"
                )),
                WallKit::load(include_bytes!(
                    "../../assets/prototypes/surface64/barriers-review/walls-concrete-source-v1.png"
                )),
                WallKit::load(include_bytes!(
                    "../../assets/prototypes/surface64/barriers-review/walls-stone-source-v1.png"
                )),
                WallKit::fence(include_bytes!(
                    "../../assets/prototypes/surface64/barriers-review/walls-fence-source-v1.png"
                )),
            ],
        }
    }

    pub fn upload(&self) -> Assets {
        Assets {
            animation: animation::Animation::new(),
            directions: self
                .directions
                .iter()
                .map(|(index, views)| (*index, views.iter().map(Raster::texture).collect()))
                .collect(),
            terrain: self.terrain.iter().map(Raster::texture).collect(),
            walls: self
                .walls
                .iter()
                .map(|r| (0..4).map(|turns| r.rotated(turns).texture()).collect())
                .collect(),
            furniture: self.furniture.iter().map(Raster::texture).collect(),
            connections: (0..16)
                .map(|mask| {
                    Raster::connected(&self.walls[0], &self.vertical, &self.corners, mask).texture()
                })
                .collect(),
            paint: std::cell::RefCell::new(super::paint::Cache::new(&self.terrain)),
            city_connections: self
                .city_walls
                .iter()
                .map(|kit| {
                    (0..16)
                        .map(|mask| {
                            Raster::connected(&kit.horizontal, &kit.vertical, &kit.corners, mask)
                                .texture()
                        })
                        .collect()
                })
                .collect(),
        }
    }

    pub fn ground(&self, tx: i32, ty: i32) -> Texture2D {
        if (1..=9).contains(&tx) && (1..=8).contains(&ty) {
            let index = if (tx, ty) == (3, 4) || (tx, ty) == (5, 6) {
                4
            } else if tx < 6 {
                usize::from(noise(tx, ty) > 0.78)
            } else {
                2 + usize::from(noise(tx, ty) > 0.78)
            };
            return self.terrain[index].texture();
        }
        let mut raster = Raster::blank();
        for y in 0..TILE {
            for x in 0..TILE {
                let wx = tx as f32 + x as f32 / TILE as f32;
                let wy = ty as f32 + y as f32 / TILE as f32;
                let grain = noise(tx * TILE as i32 + x as i32, ty * TILE as i32 + y as i32);
                let boundary = super::scene::shoreline(wy) + (grain - 0.5) * 0.12;
                let distance = wx - boundary;
                let path = (wx - (12.1 + (wy * 0.6).sin() * 0.6)).abs();
                let grass = (wx * 1.3).sin() * (wy * 0.8).cos() + (wy * 1.9 + wx * 0.4).sin() * 0.5;
                let index = if distance > 0.8 {
                    9
                } else if distance > 0.0 {
                    8
                } else if path < 0.85 + (grain - 0.5) * 0.3 {
                    5
                } else if grass > 0.1 + (grain - 0.5) * 0.35 && distance < -0.2 {
                    7
                } else {
                    6
                };
                // Sample at native 64, with mirrored variations. Material
                // boundaries are evaluated in world pixels, across cell edges.
                let sx = if tx % 2 == 0 { x } else { TILE - 1 - x };
                let sy = if ty % 2 == 0 { y } else { TILE - 1 - y };
                let mut pixel = self.terrain[index].pixel(sx, sy);
                if (-0.08..0.0).contains(&distance) {
                    pixel = self.terrain[8].pixel(sx, sy);
                    for value in &mut pixel[..3] {
                        *value = (*value as f32 * 0.78) as u8;
                    }
                }
                raster.set(x, y, pixel);
            }
        }
        raster.texture()
    }
}

fn noise(x: i32, y: i32) -> f32 {
    let n = (x as u32)
        .wrapping_mul(374761393)
        .wrapping_add((y as u32).wrapping_mul(668265263));
    let n = (n ^ (n >> 13)).wrapping_mul(1274126177);
    (n & 65535) as f32 / 65535.0
}

pub struct Assets {
    pub animation: animation::Animation,
    pub directions: std::collections::BTreeMap<usize, Vec<Texture2D>>,
    pub terrain: Vec<Texture2D>,
    pub walls: Vec<Vec<Texture2D>>,
    pub furniture: Vec<Texture2D>,
    pub connections: Vec<Texture2D>,
    pub paint: std::cell::RefCell<super::paint::Cache>,
    pub city_connections: Vec<Vec<Texture2D>>,
}

impl Assets {
    pub fn object_texture(&self, index: usize, turns: usize) -> (&Texture2D, usize) {
        self.directions
            .get(&index)
            .map_or((&self.furniture[index], turns), |views| {
                (&views[turns % 4], 0)
            })
    }
    pub fn connection(&self, style: usize, mask: usize) -> &Texture2D {
        if style == 0 {
            &self.connections[mask]
        } else {
            &self.city_connections[style - 1][mask]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn urban_kits_keep_all_connections_and_common_borders_at_native_size() {
        let prepared = Prepared::load();
        assert_eq!(prepared.directions.len(), 7);
        for (&index, views) in &prepared.directions {
            assert!(super::super::catalog::has_directions(index));
            assert_eq!(views.len(), 4);
            assert!(
                views
                    .iter()
                    .all(|view| view.bytes.chunks_exact(4).filter(|p| p[3] > 0).count() > 200)
            );
            assert!(views.windows(2).all(|pair| pair[0].bytes != pair[1].bytes));
        }
        assert_eq!(
            prepared.terrain.len(),
            super::super::catalog::MATERIAL_SLOTS
        );
        assert_eq!(
            prepared.furniture.len(),
            super::super::catalog::FURNITURE_COUNT
        );
        for floor in &prepared.terrain[16..] {
            assert!(floor.bytes.chunks_exact(4).all(|pixel| pixel[3] == 255));
        }
        for (index, sprite) in prepared.furniture[8..].iter().enumerate() {
            for side in 0..4 {
                assert!(
                    edge(sprite, side).is_empty(),
                    "sprite {} spills outside its cell",
                    index + 8
                );
            }
            assert!(
                sprite.bytes.chunks_exact(4).any(|pixel| pixel[3] == 0),
                "object must have real transparency"
            );
            let max_alpha = sprite
                .bytes
                .chunks_exact(4)
                .map(|pixel| pixel[3])
                .max()
                .unwrap();
            assert!(
                max_alpha >= 200,
                "sprite {} has no solid silhouette (maximum alpha {max_alpha})",
                8 + index
            );
        }
        for kit in &prepared.city_walls {
            for mask in 0..16 {
                let connected =
                    Raster::connected(&kit.horizontal, &kit.vertical, &kit.corners, mask);
                for side in 0..4 {
                    if kit.perforated {
                        let pixels = edge(&connected, side);
                        if mask & (1 << side) != 0 {
                            assert!(
                                !pixels.is_empty(),
                                "the fence must reach each connected edge"
                            );
                            assert!(pixels.iter().all(|p| (BAND_START..BAND_END).contains(p)));
                        } else {
                            assert!(pixels.is_empty());
                        }
                        continue;
                    }
                    assert_eq!(
                        edge(&connected, side),
                        if mask & (1 << side) != 0 {
                            (BAND_START..BAND_END).collect::<Vec<_>>()
                        } else {
                            vec![]
                        },
                        "urban mask {mask} side {side}"
                    );
                }
            }
            let left = kit.horizontal.rotated(3);
            for (corner, turns) in kit.corners.iter().zip([0, 1, 3, 2]) {
                let upright = corner.rotated((4 - turns) % 4);
                for across in BAND_START..OUTER_BORDER_END {
                    for along in OUTER_BORDER_END..TILE {
                        assert_eq!(
                            upright.pixel(along, across),
                            kit.horizontal.pixel(along, across)
                        );
                        assert_eq!(upright.pixel(across, along), left.pixel(across, along));
                    }
                }
            }
        }
    }

    fn edge(raster: &Raster, edge: usize) -> Vec<usize> {
        (0..TILE)
            .filter(|&i| {
                let (x, y) = match edge {
                    0 => (i, 0),
                    1 => (TILE - 1, i),
                    2 => (i, TILE - 1),
                    _ => (0, i),
                };
                raster.pixel(x, y)[3] == 255
            })
            .collect()
    }

    #[test]
    fn corner_borders_continue_the_wall_profile_along_both_arms() {
        let prepared = Prepared::load();
        let left = prepared.walls[0].rotated(3);
        let source = Image::from_file_with_format(
            include_bytes!("../../assets/prototypes/surface64/walls-and-corners-source-v4.png"),
            None,
        )
        .unwrap();
        for (index, (corner, turns)) in prepared.corners.iter().zip([0, 1, 3, 2]).enumerate() {
            let upright = corner.rotated((4 - turns) % 4);
            for across in BAND_START..OUTER_BORDER_END {
                for along in OUTER_BORDER_END..TILE {
                    assert_eq!(
                        upright.pixel(along, across),
                        prepared.walls[0].pixel(along, across),
                        "corner {turns}, top border ({along}, {across})"
                    );
                    assert_eq!(
                        upright.pixel(across, along),
                        left.pixel(across, along),
                        "corner {turns}, left border ({across}, {along})"
                    );
                }
            }
            let original = Raster::sample(&source, 3, 2, [1, 2, 4, 5][index])
                .corner(crate::scene::CORNER_MASKS[index])
                .rotated((4 - turns) % 4);
            for y in OUTER_BORDER_END..INNER_BORDER_START {
                for x in OUTER_BORDER_END..INNER_BORDER_START {
                    assert_eq!(
                        upright.pixel(x, y),
                        original.pixel(x, y),
                        "corner {turns}, preserved panel ({x}, {y})"
                    );
                }
            }
        }
    }

    #[test]
    fn every_straight_face_preserves_the_top_wall_border_pixels() {
        let prepared = Prepared::load();
        for face in 0..4 {
            let (mask, turns) = crate::scene::fixed_wall_sprite(10, face);
            let displayed = Raster::connected(
                &prepared.walls[0],
                &prepared.vertical,
                &prepared.corners,
                mask as u8,
            )
            .rotated(turns);
            let upright = displayed.rotated((4 - face) % 4);
            for y in BAND_START..BAND_END {
                if y < OUTER_BORDER_END || y >= INNER_BORDER_START {
                    for x in 0..TILE {
                        assert_eq!(
                            upright.pixel(x, y),
                            prepared.walls[0].pixel(x, y),
                            "face {face}, border pixel ({x}, {y})"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn decoded_wall_connections_have_matching_edges_in_all_orientations() {
        let prepared = Prepared::load();
        let band: Vec<_> = (BAND_START..BAND_END).collect();
        for (straight, mask) in [(&prepared.walls[0], 10), (&prepared.vertical, 5)] {
            for side in 0..4 {
                assert_eq!(
                    edge(straight, side),
                    if mask & (1 << side) != 0 {
                        band.clone()
                    } else {
                        vec![]
                    }
                );
            }
        }
        for (corner, mask) in prepared.corners.iter().zip(crate::scene::CORNER_MASKS) {
            for side in 0..4 {
                assert_eq!(
                    edge(corner, side),
                    if mask & (1 << side) != 0 {
                        band.clone()
                    } else {
                        vec![]
                    },
                    "corner {mask}, side {side}"
                );
            }
        }
    }

    #[test]
    fn door_edges_join_walls_and_opening_is_transparent() {
        let prepared = Prepared::load();
        for index in [2, 3] {
            assert_eq!(
                edge(&prepared.walls[index], 1),
                (BAND_START..BAND_END).collect::<Vec<_>>()
            );
            assert_eq!(
                edge(&prepared.walls[index], 3),
                (BAND_START..BAND_END).collect::<Vec<_>>()
            );
        }
        assert_eq!(prepared.walls[2].pixel(32, 32)[3], 255);
        assert_eq!(prepared.walls[3].pixel(32, 32)[3], 0);
    }

    #[test]
    fn editor_supports_every_wall_neighborhood_without_gaps() {
        let prepared = Prepared::load();
        for mask in 0..16 {
            let raster = Raster::connected(
                &prepared.walls[0],
                &prepared.vertical,
                &prepared.corners,
                mask,
            );
            for side in 0..4 {
                assert_eq!(
                    edge(&raster, side),
                    if mask & (1 << side) != 0 {
                        (BAND_START..BAND_END).collect::<Vec<_>>()
                    } else {
                        vec![]
                    },
                    "mask {mask}, side {side}"
                );
            }
        }
    }
}
