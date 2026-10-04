//! Freehand strokes in native artwork pixels. Simulation coordinates stay intact.
use super::assets::{Raster, TILE};
use macroquad::prelude::*;
use project_rl::world::GridPos;
use std::collections::{BTreeMap, BTreeSet};

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
    /// Resizing clips existing strokes without changing their visible curve.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clip: Option<Clip>,
    /// None is the ground-paint eraser; indices include both water depths.
    pub material: Option<usize>,
    pub diameter: u16,
    pub points: Vec<Point>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Clip {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    /// Retain the original grain and material sampling after an anchored move.
    #[serde(default)]
    pub offset_x: i32,
    #[serde(default)]
    pub offset_y: i32,
    #[serde(default)]
    pub turns: u8,
}

impl Clip {
    fn contains(self, point: Vec2) -> bool {
        point.x >= self.left as f32
            && point.x < self.right as f32
            && point.y >= self.top as f32
            && point.y < self.bottom as f32
    }
}

impl Stroke {
    pub fn resized(
        &self,
        old_width: i32,
        old_height: i32,
        width: i32,
        height: i32,
        dx: i32,
        dy: i32,
    ) -> Option<Self> {
        let mut output = self.clone();
        let (dx, dy) = (dx * TILE as i32, dy * TILE as i32);
        for point in &mut output.points {
            point.x += dx;
            point.y += dy;
        }
        let mut clip = self.clip.unwrap_or(Clip {
            left: 0,
            top: 0,
            right: old_width * TILE as i32,
            bottom: old_height * TILE as i32,
            offset_x: 0,
            offset_y: 0,
            turns: 0,
        });
        clip.left = (clip.left + dx).max(0);
        clip.top = (clip.top + dy).max(0);
        clip.right = (clip.right + dx).min(width * TILE as i32);
        clip.bottom = (clip.bottom + dy).min(height * TILE as i32);
        clip.offset_x += dx;
        clip.offset_y += dy;
        if clip.right <= clip.left || clip.bottom <= clip.top {
            return None;
        }
        let paint_bounds = bounds(&output.points, output.diameter as f32 / 2.0);
        if !paint_bounds.overlaps(&Rect::new(
            clip.left as f32,
            clip.top as f32,
            (clip.right - clip.left) as f32,
            (clip.bottom - clip.top) as f32,
        )) {
            return None;
        }
        output.clip = Some(clip);
        Some(output)
    }
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
                || stroke.clip.is_some_and(|clip| {
                    clip.left < 0
                        || clip.top < 0
                        || clip.right > width * TILE as i32
                        || clip.bottom > height * TILE as i32
                        || clip.left >= clip.right
                        || clip.top >= clip.bottom
                        || clip.offset_x % TILE as i32 != 0
                        || clip.offset_y % TILE as i32 != 0
                        || clip.turns > 3
                        || !(-super::scene::MAX_SIDE * TILE as i32 * 3
                            ..=super::scene::MAX_SIDE * TILE as i32 * 3)
                            .contains(&clip.offset_x)
                        || !(-super::scene::MAX_SIDE * TILE as i32 * 3
                            ..=super::scene::MAX_SIDE * TILE as i32 * 3)
                            .contains(&clip.offset_y)
                })
                || stroke.points.iter().any(|point| {
                    if stroke.clip.is_some() {
                        let limit = super::scene::MAX_SIDE * TILE as i32;
                        !(-limit * 2..=limit * 3).contains(&point.x)
                            || !(-limit * 2..=limit * 3).contains(&point.y)
                    } else {
                        !(0..width * TILE as i32).contains(&point.x)
                            || !(0..height * TILE as i32).contains(&point.y)
                    }
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

fn source_point(world: Vec2, clip: Option<Clip>) -> Vec2 {
    let Some(clip) = clip else {
        return world;
    };
    let mut point = world - vec2(clip.offset_x as f32, clip.offset_y as f32);
    for _ in 0..clip.turns {
        point = vec2(point.y, -point.x);
    }
    point
}

fn segment_alpha(world: Vec2, from: Point, to: Point, radius: f32, clip: Option<Clip>) -> u8 {
    let feather = (radius * 0.16).clamp(2.0, 6.0);
    let d = distance(world, from.vector(), to.vector());
    let source = source_point(world, clip);
    let jitter = grain(source.x.floor() as i32, source.y.floor() as i32) * 1.5;
    let alpha = ((radius - d - jitter) / feather).clamp(0.0, 1.0);
    (alpha * 255.0).round() as u8
}

/// The dominant visible material at a native pixel, including the base floor.
/// Erasing returns to that base rather than silently leaving a water blocker.
pub fn material_at(point: Point, strokes: &[Stroke], base: usize) -> usize {
    let world = point.vector() + vec2(0.5, 0.5);
    let mut weights = [0.0_f32; GROUND_MATERIALS];
    for stroke in strokes {
        if stroke.clip.is_some_and(|clip| !clip.contains(world)) {
            continue;
        }
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
                    stroke.clip,
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
fn coverage(pos: GridPos, stroke: &Stroke, resolution: usize) -> Vec<u8> {
    let origin = vec2(pos.x as f32, pos.y as f32) * TILE as f32;
    let radius = stroke.diameter as f32 / 2.0;
    let step = TILE as f32 / resolution as f32;
    let mut result = vec![0; resolution * resolution];
    for index in 0..stroke.points.len().saturating_sub(1).max(1) {
        let from = stroke.points[index];
        let to = stroke.points[(index + 1).min(stroke.points.len() - 1)];
        let rect = bounds(&[from, to], radius);
        if !rect.overlaps(&Rect::new(origin.x, origin.y, TILE as f32, TILE as f32)) {
            continue;
        }
        let left =
            (((rect.x - origin.x) / step).floor() as i32).clamp(0, resolution as i32) as usize;
        let top =
            (((rect.y - origin.y) / step).floor() as i32).clamp(0, resolution as i32) as usize;
        let right = (((rect.x + rect.w - origin.x) / step).ceil() as i32)
            .clamp(0, resolution as i32) as usize;
        let bottom = (((rect.y + rect.h - origin.y) / step).ceil() as i32)
            .clamp(0, resolution as i32) as usize;
        for y in top..bottom {
            for x in left..right {
                let world = origin + vec2(x as f32 + 0.5, y as f32 + 0.5) * step;
                if stroke.clip.is_some_and(|clip| !clip.contains(world)) {
                    continue;
                }
                // A small stable grain keeps the edge consistent with pixel art.
                let target = &mut result[y * resolution + x];
                *target = (*target).max(segment_alpha(world, from, to, radius, stroke.clip));
            }
        }
    }
    result
}

/// Composite the saved strokes, including erasure, onto a transparent tile.
pub fn rasterize(pos: GridPos, strokes: &[Stroke], materials: &[Raster]) -> Vec<u8> {
    rasterize_scaled(pos, strokes, materials, TILE)
}
fn rasterize_scaled(
    pos: GridPos,
    strokes: &[Stroke],
    materials: &[Raster],
    resolution: usize,
) -> Vec<u8> {
    let origin = vec2(pos.x as f32, pos.y as f32) * TILE as f32;
    let tile = Rect::new(origin.x, origin.y, TILE as f32, TILE as f32);
    let mut bytes = vec![0; resolution * resolution * 4];
    for stroke in strokes {
        // Clearing paint that is already transparent cannot change any pixel.
        // Large authored floor regions often carry a clipped eraser; avoid
        // evaluating its many segments at every native pixel of an empty tile.
        if stroke.material.is_none() && bytes.chunks_exact(4).all(|pixel| pixel[3] == 0) {
            continue;
        }
        if !bounds(&stroke.points, stroke.diameter as f32 / 2.0).overlaps(&tile) {
            continue;
        }
        let mask = coverage(pos, stroke, resolution);
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
            let (x, y) = (index % resolution, index / resolution);
            let source_world = source_point(
                origin + vec2(x as f32 + 0.5, y as f32 + 0.5) * TILE as f32 / resolution as f32,
                stroke.clip,
            );
            let sample = |coordinate: f32| {
                let native = coordinate.floor() as i32;
                let local = native.rem_euclid(TILE as i32) as usize;
                if native.div_euclid(TILE as i32).rem_euclid(2) == 0 {
                    local
                } else {
                    TILE - 1 - local
                }
            };
            let (sx, sy) = (sample(source_world.x), sample(source_world.y));
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
    water_regions: Vec<Rect>,
    materials: Vec<Raster>,
    tiles: BTreeMap<GridPos, Option<Texture2D>>,
    last_used: BTreeMap<GridPos, u64>,
    recency: BTreeSet<(u64, GridPos)>,
    deadline: Option<std::time::Instant>,
    pub deferred: usize,
    clock: u64,
    capacity: usize,
    resolution: usize,
    pub rasterized: usize,
}

impl Cache {
    pub fn new(materials: &[Raster]) -> Self {
        Self {
            strokes: vec![],
            water_regions: vec![],
            materials: materials[..GROUND_MATERIALS].to_vec(),
            tiles: BTreeMap::new(),
            last_used: BTreeMap::new(),
            recency: BTreeSet::new(),
            deadline: None,
            deferred: 0,
            clock: 0,
            capacity: 1024,
            resolution: TILE,
            rasterized: 0,
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
        self.last_used.retain(|pos, _| self.tiles.contains_key(pos));
        self.recency.retain(|(_, pos)| self.tiles.contains_key(pos));
    }

    /// Independent preview cache: browsing another map never evicts the canvas.
    #[allow(dead_code)]
    pub fn preview(&self, resolution: usize) -> Self {
        let mut cache = Self::new(&self.materials);
        cache.resolution = resolution.clamp(1, TILE);
        cache.capacity = 512;
        cache
    }

    /// Only preparation is budgeted; existing textures remain immediately usable.
    #[allow(dead_code)]
    pub fn begin_frame(&mut self, budget: Option<std::time::Duration>) {
        self.deadline = budget.map(|time| std::time::Instant::now() + time);
        self.deferred = 0;
    }

    /// Hold the entire visible working set, plus a margin for camera movement.
    /// Keep the largest viewport budget seen; this is bounded by window size,
    /// rather than accumulating every visited cell of a large map.
    #[allow(dead_code)] // The fixed-size surface preview shares this cache.
    pub fn reserve_visible(&mut self, width: i32, height: i32) {
        let visible = width.max(0) as usize * height.max(0) as usize;
        self.capacity = self.capacity.max(visible + 512);
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
        self.water_regions = strokes
            .iter()
            .filter(|s| matches!(s.material, Some(8 | 9)))
            .filter_map(|s| {
                let region = bounds(&s.points, s.diameter as f32 / 2.);
                if let Some(clip) = s.clip {
                    let left = region.x.max(clip.left as f32);
                    let top = region.y.max(clip.top as f32);
                    let right = region.right().min(clip.right as f32);
                    let bottom = region.bottom().min(clip.bottom as f32);
                    (right > left && bottom > top).then_some(Rect::new(
                        left,
                        top,
                        right - left,
                        bottom - top,
                    ))
                } else {
                    Some(region)
                }
            })
            .collect();
        if self.strokes.is_empty() {
            self.tiles.clear();
            self.last_used.clear();
            self.recency.clear();
        }
    }

    pub fn texture(&mut self, pos: GridPos) -> Option<&Texture2D> {
        if self.strokes.is_empty() {
            return None;
        }
        self.clock += 1;
        if !self.tiles.contains_key(&pos) {
            if self
                .deadline
                .is_some_and(|limit| std::time::Instant::now() >= limit)
            {
                self.deferred += 1;
                return None;
            }
            if self.tiles.len() >= self.capacity {
                // Evict the least recently drawn cell, never the first world
                // coordinate (which kept evicting newly entered western cells).
                if let Some((_, victim)) = self.recency.pop_first() {
                    self.tiles.remove(&victim);
                    self.last_used.remove(&victim);
                }
            }
            let bytes = if self.resolution == TILE {
                rasterize(pos, &self.strokes, &self.materials)
            } else {
                rasterize_scaled(pos, &self.strokes, &self.materials, self.resolution)
            };
            self.rasterized += 1;
            let texture = bytes.chunks_exact(4).any(|pixel| pixel[3] != 0).then(|| {
                let image = Image {
                    bytes,
                    width: self.resolution as u16,
                    height: self.resolution as u16,
                };
                let texture = Texture2D::from_image(&image);
                texture.set_filter(FilterMode::Nearest);
                texture
            });
            self.tiles.insert(pos, texture);
        }
        if let Some(old) = self.last_used.insert(pos, self.clock) {
            self.recency.remove(&(old, pos));
        }
        self.recency.insert((self.clock, pos));
        self.tiles.get(&pos).and_then(Option::as_ref)
    }

    pub fn water_at(&self, pos: GridPos) -> bool {
        let tile = Rect::new(
            pos.x as f32 * TILE as f32,
            pos.y as f32 * TILE as f32,
            TILE as f32,
            TILE as f32,
        );
        self.water_regions
            .iter()
            .any(|region| region.overlaps(&tile))
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

    fn empty_paint_cache(capacity: usize) -> Cache {
        let mut cache = Cache::new(&materials());
        cache.capacity = capacity;
        cache.sync(&[Stroke {
            clip: None,
            material: None,
            diameter: 16,
            points: vec![Point::new(0, 0)],
        }]);
        cache
    }

    #[test]
    fn zoomed_out_view_larger_than_1024_cells_stays_cached_between_frames() {
        let mut cache = empty_paint_cache(1024);
        cache.reserve_visible(65, 40);
        for _ in 0..2 {
            for y in 0..40 {
                for x in 0..65 {
                    assert!(cache.texture(GridPos::new(x, y)).is_none());
                }
            }
        }
        assert_eq!(
            cache.rasterized, 2600,
            "stationary frames must not re-rasterize visible ground"
        );
        assert_eq!(cache.tiles.len(), 2600);
    }

    #[test]
    fn entering_western_cells_evicts_old_views_instead_of_new_low_coordinates() {
        let mut cache = empty_paint_cache(3);
        for x in [100, 101, 102, 1, 2] {
            cache.texture(GridPos::new(x, 0));
        }
        assert_eq!(cache.rasterized, 5);
        cache.texture(GridPos::new(1, 0));
        cache.texture(GridPos::new(2, 0));
        assert_eq!(cache.rasterized, 5);
        assert!(!cache.tiles.contains_key(&GridPos::new(100, 0)));
        assert!(!cache.tiles.contains_key(&GridPos::new(101, 0)));
    }

    #[test]
    fn visiting_many_views_keeps_the_cache_and_recency_metadata_bounded() {
        let mut cache = empty_paint_cache(64);
        for x in 0..200 {
            cache.texture(GridPos::new(x, 0));
        }
        assert_eq!(cache.tiles.len(), 64);
        assert_eq!(cache.last_used.len(), 64);
        assert_eq!(cache.recency.len(), 64);
        let before = cache.rasterized;
        cache.texture(GridPos::new(199, 0));
        assert_eq!(cache.rasterized, before);
        cache.sync(&[]);
        assert!(cache.tiles.is_empty() && cache.last_used.is_empty() && cache.recency.is_empty());
    }

    #[test]
    fn expired_preparation_budget_keeps_cached_cells_and_defers_only_new_cells() {
        let mut cache = empty_paint_cache(64);
        cache.texture(GridPos::new(1, 0));
        cache.begin_frame(Some(std::time::Duration::ZERO));
        cache.texture(GridPos::new(1, 0));
        cache.texture(GridPos::new(2, 0));
        assert_eq!(cache.rasterized, 1);
        assert_eq!(cache.deferred, 1);
        cache.begin_frame(None);
        cache.texture(GridPos::new(2, 0));
        assert_eq!(cache.rasterized, 2);
        assert_eq!(cache.deferred, 0);
    }

    #[test]
    fn small_preview_keeps_clipping_in_world_pixels_and_native_terrain_unchanged() {
        let stroke = Stroke {
            material: Some(8),
            diameter: 384,
            points: vec![Point::new(32, 32)],
            clip: Some(Clip {
                left: 16,
                top: 16,
                right: 48,
                bottom: 48,
                offset_x: 0,
                offset_y: 0,
                turns: 0,
            }),
        };
        for resolution in [4, 8, 16, 64] {
            let bytes = rasterize_scaled(
                GridPos::new(0, 0),
                std::slice::from_ref(&stroke),
                &materials(),
                resolution,
            );
            for y in 0..resolution {
                for x in 0..resolution {
                    let point = vec2(x as f32 + 0.5, y as f32 + 0.5) * 64. / resolution as f32;
                    assert_eq!(
                        bytes[(y * resolution + x) * 4 + 3],
                        if stroke.clip.unwrap().contains(point) {
                            255
                        } else {
                            0
                        }
                    );
                }
            }
        }
        assert_eq!(
            material_at(Point::new(32, 32), std::slice::from_ref(&stroke), 6),
            8
        );
        assert_eq!(material_at(Point::new(8, 8), &[stroke], 6), 6);
    }

    #[test]
    fn fast_diagonal_stroke_crosses_tile_edges_without_filling_whole_cells() {
        let stroke = Stroke {
            clip: None,
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
            clip: None,
            material: Some(5),
            diameter: 128,
            points: vec![Point::new(32, 32)],
        }];
        let covered = rasterize(GridPos::new(0, 0), &strokes, &materials());
        strokes.push(Stroke {
            clip: None,
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
            clip: None,
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
            clip: None,
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
            clip: None,
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
            clip: None,
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
            clip: None,
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
