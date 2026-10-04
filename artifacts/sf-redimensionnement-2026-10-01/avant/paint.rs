//! Freehand strokes in native artwork pixels. Simulation coordinates stay intact.
use super::assets::{Raster, TILE};
use macroquad::prelude::*;
use project_rl::world::GridPos;
use std::collections::BTreeMap;

pub const MIN_DIAMETER: u16 = 16;
pub const MAX_DIAMETER: u16 = 384;
pub const GROUND_MATERIALS: usize = super::catalog::MATERIAL_SLOTS;

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    #[allow(dead_code)] // Also shared by the preview, which doesn't author strokes.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn vector(self) -> Vec2 {
        vec2(self.x as f32, self.y as f32)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Stroke {
    /// None is the ground-paint eraser; indices include both water depths.
    pub material: Option<usize>,
    pub diameter: u16,
    pub points: Vec<Point>,
}

pub fn validate(strokes: &[Stroke], width: i32, height: i32) -> Result<(), String> {
    if strokes.len() > 16_384
        || strokes
            .iter()
            .map(|stroke| stroke.points.len())
            .sum::<usize>()
            > 1_000_000
        || strokes.iter().any(|stroke| {
            !(MIN_DIAMETER..=MAX_DIAMETER).contains(&stroke.diameter)
                || stroke
                    .material
                    .is_some_and(|index| !super::catalog::is_floor(index))
                || stroke.points.is_empty()
                || stroke.points.iter().any(|point| {
                    !(0..width * TILE as i32).contains(&point.x)
                        || !(0..height * TILE as i32).contains(&point.y)
                })
        })
    {
        Err("Données de peinture invalides.".into())
    } else {
        Ok(())
    }
}

fn bounds(points: &[Point], radius: f32) -> Rect {
    let mut low = points[0].vector();
    let mut high = low;
    for point in &points[1..] {
        low = low.min(point.vector());
        high = high.max(point.vector());
    }
    Rect::new(
        low.x - radius,
        low.y - radius,
        high.x - low.x + radius * 2.0,
        high.y - low.y + radius * 2.0,
    )
}

/// Distance to a capsule connects fast pointer movements without gaps.
fn distance(point: Vec2, from: Vec2, to: Vec2) -> f32 {
    let delta = to - from;
    let length = delta.length_squared();
    let t = if length == 0.0 {
        0.0
    } else {
        ((point - from).dot(delta) / length).clamp(0.0, 1.0)
    };
    (point - (from + delta * t)).length()
}

fn grain(x: i32, y: i32) -> f32 {
    let n = (x as u32)
        .wrapping_mul(374_761_393)
        .wrapping_add((y as u32).wrapping_mul(668_265_263));
    let n = (n ^ (n >> 13)).wrapping_mul(1_274_126_177);
    (n & 255) as f32 / 255.0
}

fn segment_alpha(world: Vec2, from: Point, to: Point, radius: f32) -> u8 {
    let feather = (radius * 0.16).clamp(2.0, 6.0);
    let d = distance(world, from.vector(), to.vector());
    let jitter = grain(world.x as i32, world.y as i32) * 1.5;
    let alpha = ((radius - d - jitter) / feather).clamp(0.0, 1.0);
    (alpha * 255.0).round() as u8
}

/// The dominant visible material at a native pixel, including the base floor.
/// Erasing returns to that base rather than silently leaving a water blocker.
pub fn material_at(point: Point, strokes: &[Stroke], base: usize) -> usize {
    let world = point.vector() + vec2(0.5, 0.5);
    let mut weights = [0.0_f32; GROUND_MATERIALS];
    for stroke in strokes {
        let radius = stroke.diameter as f32 / 2.0;
        if !bounds(&stroke.points, radius).contains(world) {
            continue;
        }
        let alpha = (0..stroke.points.len().saturating_sub(1).max(1))
            .map(|index| {
                segment_alpha(
                    world,
                    stroke.points[index],
                    stroke.points[(index + 1).min(stroke.points.len() - 1)],
                    radius,
                )
            })
            .max()
            .unwrap_or(0) as f32
            / 255.0;
        for weight in &mut weights {
            *weight *= 1.0 - alpha;
        }
        if let Some(material) = stroke.material {
            weights[material] += alpha;
        }
    }
    weights[base] += (1.0 - weights.iter().sum::<f32>()).max(0.0);
    let mut material = base;
    for (index, weight) in weights.iter().enumerate() {
        if *weight > weights[material] {
            material = index;
        }
    }
    material
}

/// Coverage is accumulated with max, so joins within a stroke don't darken.
fn coverage(pos: GridPos, stroke: &Stroke) -> Vec<u8> {
    let origin = vec2(pos.x as f32, pos.y as f32) * TILE as f32;
    let radius = stroke.diameter as f32 / 2.0;
    let mut result = vec![0; TILE * TILE];
    for index in 0..stroke.points.len().saturating_sub(1).max(1) {
        let from = stroke.points[index];
        let to = stroke.points[(index + 1).min(stroke.points.len() - 1)];
        let rect = bounds(&[from, to], radius);
        if !rect.overlaps(&Rect::new(origin.x, origin.y, TILE as f32, TILE as f32)) {
            continue;
        }
        let left = ((rect.x - origin.x).floor() as i32).clamp(0, TILE as i32) as usize;
        let top = ((rect.y - origin.y).floor() as i32).clamp(0, TILE as i32) as usize;
        let right = ((rect.x + rect.w - origin.x).ceil() as i32).clamp(0, TILE as i32) as usize;
        let bottom = ((rect.y + rect.h - origin.y).ceil() as i32).clamp(0, TILE as i32) as usize;
        for y in top..bottom {
            for x in left..right {
                let world = origin + vec2(x as f32 + 0.5, y as f32 + 0.5);
                // A small stable grain keeps the edge consistent with pixel art.
                let target = &mut result[y * TILE + x];
                *target = (*target).max(segment_alpha(world, from, to, radius));
            }
        }
    }
    result
}

/// Composite the saved strokes, including erasure, onto a transparent tile.
pub fn rasterize(pos: GridPos, strokes: &[Stroke], materials: &[Raster]) -> Vec<u8> {
    let origin = vec2(pos.x as f32, pos.y as f32) * TILE as f32;
    let tile = Rect::new(origin.x, origin.y, TILE as f32, TILE as f32);
    let mut bytes = vec![0; TILE * TILE * 4];
    for stroke in strokes {
        if !bounds(&stroke.points, stroke.diameter as f32 / 2.0).overlaps(&tile) {
            continue;
        }
        let mask = coverage(pos, stroke);
        for (index, alpha) in mask
            .into_iter()
            .enumerate()
            .filter(|(_, alpha)| *alpha != 0)
        {
            let target = &mut bytes[index * 4..][..4];
            let remaining = target[3] as u32 * (255 - alpha as u32) / 255;
            let Some(material) = stroke.material else {
                target[3] = remaining as u8;
                continue;
            };
            let (x, y) = (index % TILE, index / TILE);
            let sx = if pos.x % 2 == 0 { x } else { TILE - 1 - x };
            let sy = if pos.y % 2 == 0 { y } else { TILE - 1 - y };
            let source = &materials[material].bytes[(sy * TILE + sx) * 4..][..4];
            let total = alpha as u32 + remaining;
            for channel in 0..3 {
                target[channel] = ((source[channel] as u32 * alpha as u32
                    + target[channel] as u32 * remaining)
                    / total) as u8;
            }
            target[3] = total as u8;
        }
    }
    bytes
}

/// GPU work is limited to visible tiles changed by a stroke or history action.
pub struct Cache {
    strokes: Vec<Stroke>,
    materials: Vec<Raster>,
    tiles: BTreeMap<GridPos, Option<Texture2D>>,
}

impl Cache {
    pub fn new(materials: &[Raster]) -> Self {
        Self {
            strokes: vec![],
            materials: materials[..GROUND_MATERIALS].to_vec(),
            tiles: BTreeMap::new(),
        }
    }

    fn invalidate(&mut self, rect: Rect) {
        self.tiles.retain(|pos, _| {
            !rect.overlaps(&Rect::new(
                pos.x as f32 * TILE as f32,
                pos.y as f32 * TILE as f32,
                TILE as f32,
                TILE as f32,
            ))
        });
    }

    pub fn sync(&mut self, strokes: &[Stroke]) {
        let common = self
            .strokes
            .iter()
            .zip(strokes)
            .take_while(|(old, new)| old == new)
            .count();
        if common == self.strokes.len() && common == strokes.len() {
            return;
        }
        // Extending the current drag only invalidates its new segment(s).
        let extension = if self.strokes.len() == strokes.len() && common + 1 == strokes.len() {
            let old = &self.strokes[common];
            let new = &strokes[common];
            (old.material == new.material
                && old.diameter == new.diameter
                && new.points.starts_with(&old.points))
            .then(|| {
                bounds(
                    &new.points[old.points.len() - 1..],
                    new.diameter as f32 / 2.0,
                )
            })
        } else {
            None
        };
        if let Some(rect) = extension {
            self.invalidate(rect);
        } else {
            let rectangles: Vec<_> = self.strokes[common..]
                .iter()
                .chain(&strokes[common..])
                .map(|stroke| bounds(&stroke.points, stroke.diameter as f32 / 2.0))
                .collect();
            for rect in rectangles {
                self.invalidate(rect);
            }
        }
        self.strokes = strokes.to_vec();
    }

    pub fn texture(&mut self, pos: GridPos) -> Option<&Texture2D> {
        if self.strokes.is_empty() {
            return None;
        }
        if !self.tiles.contains_key(&pos) {
            // Bound memory even when visiting a large map at native zoom.
            if self.tiles.len() >= 1024 {
                self.tiles.pop_first();
            }
            let bytes = rasterize(pos, &self.strokes, &self.materials);
            let texture = bytes.chunks_exact(4).any(|pixel| pixel[3] != 0).then(|| {
                let image = Image {
                    bytes,
                    width: TILE as u16,
                    height: TILE as u16,
                };
                let texture = Texture2D::from_image(&image);
                texture.set_filter(FilterMode::Nearest);
                texture
            });
            self.tiles.insert(pos, texture);
        }
        self.tiles.get(&pos).and_then(Option::as_ref)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn materials() -> Vec<Raster> {
        (0..GROUND_MATERIALS as u8)
            .map(|i| Raster {
                bytes: [(20 + i as u16 * 25).min(245) as u8, 80, 90, 255].repeat(TILE * TILE),
            })
            .collect()
    }

    #[test]
    fn fast_diagonal_stroke_crosses_tile_edges_without_filling_whole_cells() {
        let stroke = Stroke {
            material: Some(5),
            diameter: 32,
            points: vec![Point::new(16, 16), Point::new(176, 176)],
        };
        for pos in [GridPos::new(0, 0), GridPos::new(1, 1), GridPos::new(2, 2)] {
            let tile = rasterize(pos, std::slice::from_ref(&stroke), &materials());
            assert_eq!(tile[(32 * TILE + 32) * 4 + 3], 255);
            assert_eq!(tile[(4 * TILE + 60) * 4 + 3], 0);
        }
        let first = rasterize(
            GridPos::new(0, 0),
            std::slice::from_ref(&stroke),
            &materials(),
        );
        let second = rasterize(GridPos::new(1, 1), &[stroke], &materials());
        assert_eq!(first[(TILE * TILE - 1) * 4 + 3], 255);
        assert_eq!(second[3], 255);
    }

    #[test]
    fn overlaps_blend_materials_and_round_eraser_reveals_underlying_ground() {
        let mut strokes = vec![Stroke {
            material: Some(5),
            diameter: 128,
            points: vec![Point::new(32, 32)],
        }];
        let covered = rasterize(GridPos::new(0, 0), &strokes, &materials());
        strokes.push(Stroke {
            material: Some(7),
            diameter: 32,
            points: vec![Point::new(32, 32)],
        });
        let mixed = rasterize(GridPos::new(0, 0), &strokes, &materials());
        let center = (32 * TILE + 32) * 4;
        assert_eq!(mixed[center], 195);
        assert_eq!(mixed[0], covered[0]);
        // Both opaque and blended transition pixels remain opaque on this base.
        assert_eq!(mixed[center + 3], 255);
        assert!(
            mixed
                .chunks_exact(4)
                .any(|pixel| pixel[0] > 145 && pixel[0] < 195)
        );
        strokes.push(Stroke {
            material: None,
            diameter: 16,
            points: vec![Point::new(32, 32)],
        });
        let erased = rasterize(GridPos::new(0, 0), &strokes, &materials());
        assert_eq!(erased[center + 3], 0);
        assert_eq!(erased[3], covered[3]);
    }

    #[test]
    fn appending_a_stroke_only_invalidates_tiles_at_its_new_end() {
        let mut cache = Cache::new(&materials());
        let mut strokes = vec![Stroke {
            material: Some(5),
            diameter: 16,
            points: vec![Point::new(16, 32), Point::new(160, 32)],
        }];
        cache.sync(&strokes);
        for x in 0..4 {
            cache.tiles.insert(GridPos::new(x, 0), None);
        }
        strokes[0].points.push(Point::new(224, 32));
        cache.sync(&strokes);
        assert!(cache.tiles.contains_key(&GridPos::new(0, 0)));
        assert!(cache.tiles.contains_key(&GridPos::new(1, 0)));
        assert!(!cache.tiles.contains_key(&GridPos::new(2, 0)));
        assert!(!cache.tiles.contains_key(&GridPos::new(3, 0)));
        cache.sync(&[]);
        assert!(cache.tiles.is_empty());
    }

    #[test]
    fn both_water_depths_render_and_erasing_restores_the_base_material() {
        let center = Point::new(32, 32);
        let mut strokes = vec![Stroke {
            material: Some(8),
            diameter: 96,
            points: vec![center],
        }];
        let index = (32 * TILE + 32) * 4;
        assert_eq!(
            rasterize(GridPos::new(0, 0), &strokes, &materials())[index],
            220
        );
        assert_eq!(material_at(center, &strokes, 6), 8);
        strokes.push(Stroke {
            material: Some(9),
            diameter: 32,
            points: vec![center],
        });
        assert_eq!(
            rasterize(GridPos::new(0, 0), &strokes, &materials())[index],
            245
        );
        assert_eq!(material_at(center, &strokes, 6), 9);
        assert_eq!(material_at(Point::new(8, 32), &strokes, 6), 8);
        strokes.push(Stroke {
            material: None,
            diameter: 16,
            points: vec![center],
        });
        assert_eq!(
            rasterize(GridPos::new(0, 0), &strokes, &materials())[index + 3],
            0
        );
        assert_eq!(material_at(center, &strokes, 6), 6);
        assert_eq!(material_at(center, &strokes, 9), 9);
        let mut cache = Cache::new(&materials());
        cache.sync(&strokes);
        assert_eq!(cache.materials.len(), GROUND_MATERIALS);
    }
}
