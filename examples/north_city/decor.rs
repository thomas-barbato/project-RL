//! A continuous architectural backdrop, compiled from the playable plan.
//! Floor ownership and wall silhouettes share the same geometry. Native
//! chunks are only a bounded rendering detail: their edges do not alter art.
use crate::{
    asset_preparation::{self, Builder, PreparedImage},
    model, scene, street_skin, volumes,
};
use macroquad::prelude::*;
use project_rl::world::GridPos;
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

const TILE: i32 = 64;
const CHUNK: i32 = 8;
const PREFIX: &str = "north-city-decor07";
const PALETTE_BUDGET: usize = 12 * 1024 * 1024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Surface {
    pub material: usize,
    pub anchor: [i32; 2],
}
#[derive(Clone, Copy)]
struct Wall {
    mask: u8,
    style: usize,
    door: bool,
    rotation: usize,
}
pub struct Plan {
    width: i32,
    height: i32,
    surfaces: Vec<Surface>,
    walls: Vec<Option<Wall>>,
    sealed: Vec<bool>,
    volumes: volumes::Volumes,
    roofs: Vec<Roof>,
    scenery: Vec<(String, [i32; 2])>,
}
struct Roof {
    id: String,
    rect: [i32; 4],
    volume: volumes::Volumes,
}
impl Plan {
    pub fn new(zone: &model::Zone, scene: &scene::Scene) -> Self {
        let d = &scene.document;
        let sealed = zone.sealed.iter().copied().collect::<BTreeSet<_>>();
        let mut result = Self {
            width: d.width,
            height: d.height,
            surfaces: Vec::with_capacity(d.floors.len()),
            walls: vec![None; d.floors.len()],
            sealed: vec![false; d.floors.len()],
            volumes: volumes::Volumes::new(zone, scene),
            roofs: zone
                .buildings
                .iter()
                .filter(|b| b.accessible)
                .map(|b| Roof {
                    id: b.id.clone(),
                    rect: b.rect,
                    volume: volumes::Volumes::rectangle(b.rect).with_elevation(
                        if b.id == "south_west" {
                            volumes::BLOCK_HEIGHT
                        } else {
                            volumes::HEIGHT
                        },
                    ),
                })
                .collect(),
            scenery: zone
                .scenery
                .iter()
                .map(|s| (s.file.clone(), s.at_px))
                .collect(),
        };
        for y in 0..d.height {
            for x in 0..d.width {
                let material = if sealed.contains(&[x, y]) {
                    99
                } else {
                    d.floors[(y * d.width + x) as usize].unwrap_or(18)
                };
                let anchor = zone
                    .rooms
                    .iter()
                    .find(|r| {
                        r.material == material
                            && x >= r.rect[0]
                            && x <= r.rect[2]
                            && y >= r.rect[1]
                            && y <= r.rect[3]
                    })
                    .map_or([0, 0], |r| [r.rect[0] * TILE, r.rect[1] * TILE]);
                result.surfaces.push(Surface { material, anchor });
                result.sealed[(y * d.width + x) as usize] = sealed.contains(&[x, y]);
            }
        }
        for s in &d.structures {
            if sealed.contains(&[s.pos.x, s.pos.y]) {
                continue;
            }
            let mask = s
                .pos
                .cardinal_neighbors()
                .iter()
                .enumerate()
                .fold(0, |m, (i, p)| {
                    if scene.is_structure(*p) && !sealed.contains(&[p.x, p.y]) {
                        m | (1 << i)
                    } else {
                        m
                    }
                });
            result.walls[(s.pos.y * d.width + s.pos.x) as usize] = Some(Wall {
                mask,
                style: s.style,
                door: s.door.is_some(),
                rotation: s.rotation,
            });
        }
        result
    }
    fn index(&self, x: i32, y: i32) -> Option<usize> {
        (x >= 0 && y >= 0 && x < self.width && y < self.height)
            .then_some((y * self.width + x) as usize)
    }
    /// The nearest actual floor rectangle owns each exposed pixel. Material
    /// boundaries meet beneath the wall body, rather than at arbitrary 8 px
    /// tile margins. This also works at concave corners and T junctions.
    pub fn surface_at(&self, wx: i32, wy: i32) -> Surface {
        let tx = wx.div_euclid(TILE);
        let ty = wy.div_euclid(TILE);
        let Some(i) = self.index(tx, ty) else {
            return Surface {
                material: 18,
                anchor: [0, 0],
            };
        };
        if self.walls[i].is_none() || self.sealed[i] {
            return self.surfaces[i];
        }
        let mask = self.walls[i].unwrap().mask;
        // A straight partition also defines a straight material boundary at
        // its ends and thresholds. A nearest-diagonal choice would create a
        // triangular patch beside an open gate or the end of a fence.
        let axis = if matches!(mask, 1 | 4 | 5) {
            Some((if wx.rem_euclid(64) < 32 { -1 } else { 1 }, 0))
        } else if matches!(mask, 2 | 8 | 10) {
            Some((0, if wy.rem_euclid(64) < 32 { -1 } else { 1 }))
        } else {
            None
        };
        if let Some((dx, dy)) = axis {
            if let Some(j) = self.index(tx + dx, ty + dy) {
                if self.walls[j].is_none() {
                    return self.surfaces[j];
                }
            }
        }
        let mut best = None;
        for dy in -1..=1 {
            for dx in -1..=1 {
                let Some(j) = self.index(tx + dx, ty + dy) else {
                    continue;
                };
                // A solid neighbouring mass contributes its roof surface on
                // that side of a facade, even though it is not walkable.
                if self.walls[j].is_some() {
                    continue;
                }
                let l = (tx + dx) * TILE;
                let t = (ty + dy) * TILE;
                let distance =
                    (wx - wx.clamp(l, l + 63)).pow(2) + (wy - wy.clamp(t, t + 63)).pow(2);
                if best.is_none_or(|(old, _)| distance < old) {
                    best = Some((distance, self.surfaces[j]));
                }
            }
        }
        best.map_or(self.surfaces[i], |(_, surface)| surface)
    }
    fn raster(&self, palette: &Palette, cx: i32, cy: i32, step: i32) -> Image {
        let overview = step > 1;
        let (ox, oy, w, h) = if overview {
            (0, 0, self.width * TILE / step, self.height * TILE / step)
        } else {
            (
                cx * CHUNK * TILE,
                cy * CHUNK * TILE,
                CHUNK * TILE,
                CHUNK * TILE,
            )
        };
        self.raster_region(palette, ox, oy, w, h, step)
    }
    fn raster_region(
        &self,
        palette: &Palette,
        ox: i32,
        oy: i32,
        w: i32,
        h: i32,
        step: i32,
    ) -> Image {
        let mut image = Image {
            width: w as u16,
            height: h as u16,
            bytes: vec![0; (w * h * 4) as usize],
        };
        for y in 0..h {
            for x in 0..w {
                let wx = ox + x * step;
                let wy = oy + y * step;
                if let Some(p) = self.volumes.pixel(&palette.volumes, wx, wy) {
                    put(&mut image, x, y, p);
                    continue;
                }
                let surface = self.surface_at(wx, wy);
                let slot = street_skin::material_slot(surface.material);
                let source = &palette.floors[slot];
                let sx = (wx - surface.anchor[0]).rem_euclid(source.width as i32) as usize;
                let sy = (wy - surface.anchor[1]).rem_euclid(source.height as i32) as usize;
                let pixel = rgba(source, sx, sy);
                let tint = if surface.material == 99 {
                    [0.61, 0.67, 0.73]
                } else {
                    [0.77, 0.81, 0.88]
                };
                put(
                    &mut image,
                    x,
                    y,
                    [
                        (pixel[0] as f32 * tint[0]) as u8,
                        (pixel[1] as f32 * tint[1]) as u8,
                        (pixel[2] as f32 * tint[2]) as u8,
                        255,
                    ],
                );
            }
        }
        // Flat aftermath decals are fixed map layers. Roof overhangs occlude
        // their pixels naturally; a corpse cannot be painted onto a roof.
        for (file, [px, py]) in &self.scenery {
            let source = asset_preparation::image(file).expect("prepared review scenery");
            if *px >= ox + w * step
                || *py >= oy + h * step
                || *px + source.width as i32 <= ox
                || *py + source.height as i32 <= oy
            {
                continue;
            }
            let l = ((*px - ox).div_euclid(step)).max(0);
            let t = ((*py - oy).div_euclid(step)).max(0);
            let r = ((px + source.width as i32 - ox + step - 1).div_euclid(step)).min(w);
            let b = ((py + source.height as i32 - oy + step - 1).div_euclid(step)).min(h);
            for y in t..b {
                for x in l..r {
                    let wx = ox + x * step;
                    let wy = oy + y * step;
                    if self.volumes.surface_depth(wx, wy).is_some() {
                        continue;
                    }
                    let sx = wx - px;
                    let sy = wy - py;
                    if sx < 0 || sy < 0 || sx >= source.width as i32 || sy >= source.height as i32 {
                        continue;
                    }
                    let mut p = rgba(&source, sx as usize, sy as usize);
                    for (i, gain) in [0.82, 0.86, 0.94].into_iter().enumerate() {
                        p[i] = (p[i] as f32 * gain) as u8;
                    }
                    blend(&mut image, x, y, p);
                }
            }
        }
        // Floor and wall faces are painted together in world coordinates.
        // The same halo is sampled on both sides of a rendering chunk.
        for top in [false, true] {
            let l = (ox / TILE - 1).max(0);
            let t = (oy / TILE - 2).max(0);
            let r = ((ox + w * step) / TILE + 1).min(self.width);
            let b = ((oy + h * step) / TILE + 1).min(self.height);
            for ty in t..b {
                for tx in l..r {
                    let Some(wall) = self.walls[(ty * self.width + tx) as usize] else {
                        continue;
                    };
                    if wall.door || self.volumes.cell(tx, ty) {
                        continue;
                    }
                    let atlas_y =
                        (street_skin::wall_family(wall.style) * 2 * street_skin::WALL_HEIGHT
                            + if top { street_skin::WALL_HEIGHT } else { 0 })
                            as i32;
                    let start_x = (tx * TILE - ox).div_euclid(step);
                    let start_y = (ty * TILE - oy).div_euclid(step);
                    for dy in 0..(street_skin::WALL_HEIGHT as i32 + step - 1) / step {
                        for dx in 0..(TILE + step - 1) / step {
                            let x = start_x + dx;
                            let y = start_y + dy;
                            if x < 0 || y < 0 || x >= w || y >= h {
                                continue;
                            }
                            if self
                                .volumes
                                .surface_depth(ox + x * step, oy + y * step)
                                .is_some_and(|depth| depth > (ty + 1) * 64)
                            {
                                continue;
                            }
                            let mut p = rgba(
                                &palette.walls,
                                wall.mask as usize * 64 + (dx * step).min(63) as usize,
                                (atlas_y + (dy * step).min(street_skin::WALL_HEIGHT as i32 - 1))
                                    as usize,
                            );
                            for (i, tint) in [0.84, 0.88, 0.94].into_iter().enumerate() {
                                p[i] = (p[i] as f32 * tint) as u8;
                            }
                            blend(&mut image, x, y, p);
                        }
                    }
                }
            }
        }
        // Real thresholds occupy only the opening; no floor islands or broad
        // metallic square is painted under a door.
        for ty in (oy / TILE).max(0)..((oy + h * step) / TILE + 1).min(self.height) {
            for tx in (ox / TILE).max(0)..((ox + w * step) / TILE + 1).min(self.width) {
                let Some(wall) = self.walls[(ty * self.width + tx) as usize] else {
                    continue;
                };
                if !wall.door {
                    continue;
                }
                for dy in 0..64 / step {
                    for dx in 0..64 / step {
                        let px = dx * step;
                        let py = dy * step;
                        let threshold = if wall.rotation % 2 == 1 {
                            (29..35).contains(&px)
                        } else {
                            (29..35).contains(&py)
                        };
                        if !threshold {
                            continue;
                        }
                        let x = (tx * TILE - ox) / step + dx;
                        let y = (ty * TILE - oy) / step + dy;
                        if x >= 0 && y >= 0 && x < w && y < h {
                            put(
                                &mut image,
                                x,
                                y,
                                if px % 64 == 29 || py % 64 == 29 {
                                    [84, 84, 76, 255]
                                } else {
                                    [51, 53, 51, 255]
                                },
                            );
                        }
                    }
                }
            }
        }
        image
    }
}

struct Palette {
    floors: Vec<Image>,
    walls: Image,
    volumes: volumes::Paint,
}
thread_local! {
    // At most nine authored zones. This registry contains no game state.
    static PLANS:RefCell<BTreeMap<String,Arc<Plan>>>=const{RefCell::new(BTreeMap::new())};
    // One immutable painter palette, checked against a fixed 12 MiB bound.
    // Source PNG decoding still uses the shared 64 MiB source cache.
    static PALETTE:RefCell<Option<Palette>>=const{RefCell::new(None)};
}
fn name(zone: &model::Zone) -> String {
    std::path::Path::new(&zone.file)
        .file_stem()
        .unwrap()
        .to_string_lossy()
        .into_owned()
}
pub fn register(review: &model::Review, scenes: &[scene::Scene]) {
    PLANS.with_borrow_mut(|plans| {
        for (zone, scene) in review.zones.iter().zip(scenes) {
            plans
                .entry(name(zone))
                .or_insert_with(|| Arc::new(Plan::new(zone, scene)));
        }
        assert!(
            plans.len() <= 9,
            "Only the nine authored city/police plans belong in this review"
        );
    });
}
fn key(name: &str, cx: i32, cy: i32, step: i32) -> String {
    format!("{PREFIX}:{name}:{cx}:{cy}:{step}")
}
pub fn keys(zone: &model::Zone, at: GridPos) -> Vec<String> {
    let n = name(zone);
    let mut keys = vec![key(&n, 0, 0, 8)];
    PLANS.with_borrow(|plans| {
        let p = &plans[&n];
        for roof in &p.roofs {
            let [l, t, r, b] = roof.rect;
            if at.x + 18 >= l && at.x - 18 <= r && at.y + 16 >= t && at.y - 16 <= b {
                keys.push(format!("north-city-roof07:{n}:{}", roof.id));
            }
        }
        // Native viewport plus a nearby ring, prepared before it is revealed.
        for cy in ((at.y - 14).div_euclid(CHUNK)).max(0)
            ..=((at.y + 14).div_euclid(CHUNK)).min((p.height - 1) / CHUNK)
        {
            for cx in ((at.x - 18).div_euclid(CHUNK)).max(0)
                ..=((at.x + 18).div_euclid(CHUNK)).min((p.width - 1) / CHUNK)
            {
                keys.push(key(&n, cx, cy, 1));
            }
        }
    });
    keys
}
pub fn builder(path: &str) -> Option<Builder> {
    let mut fields = path.split(':');
    let kind = fields.next()?;
    if kind == "north-city-roof07" {
        let n = fields.next()?;
        let id = fields.next()?;
        let plan = PLANS.with_borrow(|p| p.get(n).cloned())?;
        let index = plan.roofs.iter().position(|r| r.id == id)?;
        return Some(Box::new(move || {
            let artwork = if plan.roofs[index].id == "south_west" {
                Some(volumes::block_image()?)
            } else {
                None
            };
            prepared(|palette| {
                let roof = &plan.roofs[index];
                let [l, t, r, b] = roof.rect;
                let (ox, oy, w, h) = (
                    l * 64,
                    t * 64 - roof.volume.elevation,
                    (r - l + 1) * 64,
                    (b - t + 1) * 64 + roof.volume.elevation,
                );
                let mut image = Image {
                    width: w as u16,
                    height: h as u16,
                    bytes: vec![0; (w * h * 4) as usize],
                };
                for y in 0..h {
                    for x in 0..w {
                        let wx = ox + x;
                        let wy = oy + y;
                        if let Some(depth) = roof.volume.surface_depth(wx, wy) {
                            if plan
                                .volumes
                                .surface_depth(wx, wy)
                                .is_some_and(|other| other > depth)
                            {
                                continue;
                            }
                            let pixel = if let Some(art) = &artwork {
                                Some(rgba(art, x as usize, y as usize))
                            } else {
                                roof.volume.pixel(&palette.volumes, wx, wy)
                            };
                            if let Some(pixel) = pixel {
                                put(&mut image, x, y, pixel);
                            }
                        }
                    }
                }
                image
            })
        }));
    }
    if kind == "north-city-actor07" {
        let n = fields.next()?;
        let x = fields.next()?.parse::<i32>().ok()?;
        let y = fields.next()?.parse::<i32>().ok()?;
        let flags = fields.next()?.parse::<u64>().ok()?;
        let plan = PLANS.with_borrow(|p| p.get(n).cloned())?;
        return Some(Box::new(move || {
            let source = asset_preparation::image(crate::PLAYER)?;
            let mut image = (*source).clone();
            let feet = y * 64 + 62;
            for sy in 0..64 {
                for sx in 0..64 {
                    let wx = x * 64 + sx;
                    let wy = y * 64 + sy;
                    let hidden = plan.volumes.surface_depth(wx, wy).is_some_and(|d| d > feet)
                        || plan.roofs.iter().enumerate().any(|(i, r)| {
                            flags & (1 << i) != 0
                                && r.volume.surface_depth(wx, wy).is_some_and(|d| d > feet)
                        });
                    if hidden {
                        image.bytes[((sy * 64 + sx) * 4 + 3) as usize] = 0;
                    }
                }
            }
            Ok(PreparedImage {
                image,
                cell: [64, 64],
                anchor: [32., 32.],
            })
        }));
    }
    if kind != PREFIX {
        return None;
    }
    let n = fields.next()?;
    let cx = fields.next()?.parse().ok()?;
    let cy = fields.next()?.parse().ok()?;
    let step = fields.next()?.parse().ok()?;
    let plan = PLANS.with_borrow(|p| p.get(n).cloned())?;
    Some(Box::new(move || {
        prepared(|palette| plan.raster(palette, cx, cy, step))
    }))
}
fn prepared(build: impl FnOnce(&Palette) -> Image) -> Result<PreparedImage, String> {
    PALETTE.with_borrow_mut(|cache| {
        if cache.is_none() {
            let source = asset_preparation::image(street_skin::MATERIAL_SOURCE)?;
            let floors = (0..9)
                .map(street_skin::floor)
                .collect::<Result<Vec<_>, _>>()?;
            let walls = street_skin::wall_atlas(&source);
            let volumes = volumes::Paint::load()?;
            if floors.iter().map(|i| i.bytes.len()).sum::<usize>()
                + walls.bytes.len()
                + volumes.bytes()
                > PALETTE_BUDGET
            {
                return Err("Decor painter exceeded its fixed memory bound".into());
            }
            *cache = Some(Palette {
                floors,
                walls,
                volumes,
            });
        }
        Ok(PreparedImage {
            image: build(cache.as_ref().unwrap()),
            cell: [64, 64],
            anchor: [32., 32.],
        })
    })
}
fn roof_flags(zone: &model::Zone, world: &project_rl::game::WorldState) -> u64 {
    zone.buildings
        .iter()
        .filter(|b| b.accessible)
        .enumerate()
        .fold(0, |flags, (i, b)| {
            if b.open(world) {
                flags
            } else {
                flags | (1 << i)
            }
        })
}
pub fn actor_key(zone: &model::Zone, world: &project_rl::game::WorldState, roofs: bool) -> String {
    let p = world.player_position().unwrap();
    format!(
        "north-city-actor07:{}:{}:{}:{}",
        name(zone),
        p.x,
        p.y,
        if roofs { roof_flags(zone, world) } else { 0 }
    )
}
pub fn actor_keys(zone: &model::Zone, world: &project_rl::game::WorldState) -> [String; 2] {
    [actor_key(zone, world, false), actor_key(zone, world, true)]
}
pub fn foot_hidden(zone: &model::Zone, pos: GridPos, height: i32) -> bool {
    PLANS.with_borrow(|plans| {
        plans[&name(zone)]
            .volumes
            .surface_depth(pos.x * 64 + 32, pos.y * 64 + 32)
            .is_some_and(|d| d > (pos.y + height) * 64)
    })
}
pub fn draw_roofs(
    zone: &model::Zone,
    world: &project_rl::game::WorldState,
    origin: Vec2,
    bounds: Rect,
    cell: f32,
) {
    let n = name(zone);
    PLANS.with_borrow(|plans| {
        for roof in &plans[&n].roofs {
            let b = zone.buildings.iter().find(|b| b.id == roof.id).unwrap();
            if b.open(world) {
                continue;
            }
            let [l, t, r, bb] = roof.rect;
            let at = origin + vec2(l as f32, t as f32 - roof.volume.elevation as f32 / 64.) * cell;
            let size = vec2(
                (r - l + 1) as f32,
                (bb - t + 1) as f32 + roof.volume.elevation as f32 / 64.,
            ) * cell;
            if !bounds.overlaps(&Rect::new(at.x, at.y, size.x, size.y)) {
                continue;
            }
            asset_preparation::with_texture(&format!("north-city-roof07:{n}:{}", roof.id), |p| {
                draw_texture_ex(
                    &p.texture,
                    at.x,
                    at.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(size),
                        ..Default::default()
                    },
                )
            });
        }
    });
}
pub fn draw(zone: &model::Zone, origin: Vec2, bounds: Rect, cell: f32) {
    let n = name(zone);
    PLANS.with_borrow(|plans| {
        let p = &plans[&n];
        if cell < 32. {
            asset_preparation::with_texture(&key(&n, 0, 0, 8), |t| {
                draw_texture_ex(
                    &t.texture,
                    origin.x,
                    origin.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(p.width as f32, p.height as f32) * cell),
                        ..Default::default()
                    },
                )
            });
            return;
        }
        let size = CHUNK as f32 * cell;
        let l = ((bounds.x - origin.x) / size).floor().max(0.) as i32;
        let t = ((bounds.y - origin.y) / size).floor().max(0.) as i32;
        let r =
            (((bounds.right() - origin.x) / size).ceil() as i32).min((p.width + CHUNK - 1) / CHUNK);
        let b = (((bounds.bottom() - origin.y) / size).ceil() as i32)
            .min((p.height + CHUNK - 1) / CHUNK);
        for cy in t..b {
            for cx in l..r {
                let width = ((p.width - cx * CHUNK).min(CHUNK) * 64) as f32;
                let height = ((p.height - cy * CHUNK).min(CHUNK) * 64) as f32;
                asset_preparation::with_texture(&key(&n, cx, cy, 1), |texture| {
                    draw_texture_ex(
                        &texture.texture,
                        origin.x + cx as f32 * size,
                        origin.y + cy as f32 * size,
                        WHITE,
                        DrawTextureParams {
                            source: Some(Rect::new(0., 0., width, height)),
                            dest_size: Some(vec2(width, height) * cell / 64.),
                            ..Default::default()
                        },
                    )
                });
            }
        }
    });
}
fn rgba(image: &Image, x: usize, y: usize) -> [u8; 4] {
    image.bytes[(y * image.width as usize + x) * 4..][..4]
        .try_into()
        .unwrap()
}
fn put(image: &mut Image, x: i32, y: i32, p: [u8; 4]) {
    let i = ((y * image.width as i32 + x) * 4) as usize;
    image.bytes[i..i + 4].copy_from_slice(&p);
}
fn blend(image: &mut Image, x: i32, y: i32, p: [u8; 4]) {
    if p[3] == 0 {
        return;
    }
    let old = rgba(image, x as usize, y as usize);
    let a = p[3] as u32;
    let mut out = [0, 0, 0, 255];
    for i in 0..3 {
        out[i] = ((p[i] as u32 * a + old[i] as u32 * (255 - a)) / 255) as u8;
    }
    put(image, x, y, out);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_accessible_building_facade_is_continuous_with_its_authored_doors() {
        let (r, s) = model::load(20261105).unwrap();
        let p = Plan::new(&r.zones[0], &s[0]);
        for b in r.zones[0].buildings.iter().filter(|b| b.accessible) {
            let [l, t, rr, bb] = b.rect;
            let mut doors = BTreeSet::new();
            for y in t..=bb {
                for x in l..=rr {
                    if x != l && x != rr && y != t && y != bb {
                        continue;
                    }
                    let wall = p.walls[p.index(x, y).unwrap()]
                        .unwrap_or_else(|| panic!("Missing facade at {x},{y} in {}", b.id));
                    if wall.door {
                        doors.insert([x, y]);
                    }
                }
            }
            assert_eq!(
                doors,
                b.doors.iter().copied().collect(),
                "Door metadata for {}",
                b.id
            );
        }
    }
    #[test]
    fn chunked_backdrop_is_pixel_identical_to_a_single_continuous_canvas() {
        let (r, s) = model::load(20261105).unwrap();
        let p = Plan::new(&r.zones[0], &s[0]);
        let dummy = Image {
            width: 96,
            height: 96,
            bytes: vec![160; 96 * 96 * 4],
        };
        let floors = (0..9)
            .map(|slot| {
                let mut i = Image {
                    width: 192,
                    height: 192,
                    bytes: vec![0; 192 * 192 * 4],
                };
                for y in 0..192 {
                    for x in 0..192 {
                        put(&mut i, x, y, [x as u8, y as u8, slot * 20, 255]);
                    }
                }
                i
            })
            .collect();
        let palette = Palette {
            floors,
            walls: street_skin::wall_atlas(&dummy),
            volumes: volumes::Paint::load().unwrap(),
        };
        for (cx, cy) in [(3, 12), (3, 2)] {
            let full = p.raster_region(&palette, cx * 512, cy * 512, 1024, 1024, 1);
            for dy in 0..2 {
                for dx in 0..2 {
                    let chunk = p.raster(&palette, cx + dx, cy + dy, 1);
                    for y in 0..512 {
                        let a = (y * 512 * 4) as usize;
                        let b = (((dy * 512 + y) * 1024 + dx * 512) * 4) as usize;
                        assert_eq!(
                            &chunk.bytes[a..a + 2048],
                            &full.bytes[b..b + 2048],
                            "Architecture seam at chunk {},{} row {}",
                            cx + dx,
                            cy + dy,
                            y
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn actual_partition_and_facade_exposed_pixels_use_the_floor_on_their_side() {
        let (r, s) = model::load(20261105).unwrap();
        let p = Plan::new(&r.zones[0], &s[0]);
        // Bathroom/hall divider: the old 8/56 margin leaked stored parquet.
        for y in 0..64 {
            for x in 0..18 {
                assert_eq!(p.surface_at(26 * 64 + x, 99 * 64 + y).material, 22);
            }
        }
        for y in 0..64 {
            for x in 46..64 {
                assert_eq!(p.surface_at(26 * 64 + x, 99 * 64 + y).material, 23);
            }
        }
        // Outer south facade: the pavement reaches the face, no indoor strip.
        for y in 46..64 {
            for x in 0..64 {
                assert_eq!(p.surface_at(27 * 64 + x, 102 * 64 + y).material, 18);
            }
        }
        assert_eq!(p.surface_at(27 * 64 + 32, 102 * 64 + 5).material, 23);
        for x in 0..64 {
            for y in 0..18 {
                assert_eq!(p.surface_at(28 * 64 + x, 92 * 64 + y).material, 99);
            }
        }
    }
    #[test]
    fn room_materials_continue_in_world_space_across_rendering_chunk_edges() {
        let (r, s) = model::load(20261105).unwrap();
        let p = Plan::new(&r.zones[0], &s[0]);
        let a = p.surface_at(24 * 64 - 1, 100 * 64 + 30);
        let b = p.surface_at(24 * 64, 100 * 64 + 30);
        assert_eq!(a, b);
        assert_eq!(a.material, 22);
        assert_eq!(a.anchor, [23 * 64, 98 * 64]);
        assert_eq!(
            p.walls.iter().flatten().count(),
            s[0].document.structures.len() - r.zones[0].sealed.len()
        );
    }
}
