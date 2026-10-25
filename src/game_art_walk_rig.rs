//! Articulated walking of the accepted painted three-quarter bodies.
//! Carried objects are rigid independent layers, never part of a leg mask.
use super::*;
use crate::game_art::walking::Pose;
use serde::Deserialize;
use std::sync::OnceLock;

#[derive(Deserialize)]
struct Rig {
    id: String,
    facing: i8,
    file: String,
    palms: [[f32; 2]; 2],
    pivots: [[f32; 2]; 2],
    hip: u16,
    bottom: u16,
    center: f32,
    #[cfg(test)]
    #[serde(default)]
    leg_guard: Option<[u16; 4]>,
}
fn rigs() -> &'static [Rig] {
    static RIGS: OnceLock<Vec<Rig>> = OnceLock::new();
    RIGS.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../assets/animations/walk-04/rigs/manifest.json"
        ))
        .unwrap()
    })
}
#[derive(Deserialize)]
struct DrawnCycle {
    id: String,
    facing: i8,
    file: String,
    waist: u16,
}
fn drawn(source: &Image, key: Key, pose: Pose) -> Option<Image> {
    if pose.frame >= 6 {
        return None;
    }
    static CYCLES: OnceLock<Vec<DrawnCycle>> = OnceLock::new();
    let cycles = CYCLES.get_or_init(|| {
        let text =
            project_rl::data_paths::read_to_string("assets/animations/walk-04/drawn/manifest.json")
                .expect("drawn walk manifest");
        serde_json::from_str(&text).unwrap()
    });
    let id = id(key)?;
    let cycle = cycles
        .iter()
        .find(|c| c.id == id && c.facing == pose.facing)?;
    let frames = PARTS.with_borrow_mut(|cache| {
        if let Some(frames) = cache.get(&cycle.file) {
            return Some(frames.clone());
        }
        let bytes = project_rl::data_paths::read(&cycle.file).ok()?;
        let sheet = Image::from_file_with_format(&bytes, Some(ImageFormat::Png)).ok()?;
        let frames: Vec<_> = (0..6)
            .map(|frame| sheet.sub_image(Rect::new(frame as f32 * 128., 0., 128., 128.)))
            .collect();
        cache.insert(cycle.file.clone(), frames.clone(), 6 * 128 * 128 * 4);
        Some(frames)
    })?;
    let mut out = frames[usize::from(pose.frame)].clone();
    let mut upper = source.clone();
    upper.bytes[(usize::from(cycle.waist) + 1) * 512..].fill(0);
    geometry::over(&mut out, &upper);
    Some(out)
}
thread_local! {
    static PARTS: std::cell::RefCell<project_rl::asset_cache::ByteCache<String,Vec<Image>>> =
        const {std::cell::RefCell::new(project_rl::asset_cache::ByteCache::new(12*1024*1024))};
}
// Only the explicit, ignored source-compositor diagnostic reads this proposal.
#[cfg(test)]
thread_local! { static PROPORTIONS10: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
fn parts(rig: &Rig) -> Option<Vec<Image>> {
    PARTS.with_borrow_mut(|cache| {
        if let Some(parts) = cache.get(&rig.file) {
            return Some(parts.clone());
        }
        let bytes = project_rl::data_paths::read(&rig.file).ok()?;
        let sheet = Image::from_file_with_format(&bytes, Some(ImageFormat::Png)).ok()?;
        if (sheet.width, sheet.height) != (640, 128) {
            return None;
        }
        let parts: Vec<_> = (0..5)
            .map(|n| {
                let mut im = Image::gen_image_color(128, 128, BLANK);
                for y in 0..128usize {
                    let at = (y * 640 + n * 128) * 4;
                    im.bytes[y * 512..(y + 1) * 512].copy_from_slice(&sheet.bytes[at..at + 512]);
                }
                im
            })
            .collect();
        cache.insert(rig.file.clone(), parts.clone(), 5 * 128 * 128 * 4);
        Some(parts)
    })
}
// Six contact/passing poses, three per grid step. The next step changes the
// supporting leg just as data-scene="1" of the original canvas does.
fn phase(frame: u8) -> f32 {
    [-1., -0.15, 0.85, 1., 0.15, -0.85, 0., 0., 0.][usize::from(frame.min(8))]
}

pub(super) fn gait(source: &Image, hip: u16, bottom: u16, center: f32, pose: Pose) -> Image {
    if pose.frame == 8 {
        return source.clone();
    }
    let hip = usize::from(hip);
    let bottom = usize::from(bottom);
    let mut out = Image::gen_image_color(128, 128, BLANK);
    let stride = phase(pose.frame);
    let leg_height = bottom.saturating_sub(hip).max(8) as f32;
    let order = if stride < 0. { [1, 0] } else { [0, 1] };
    for limb in order {
        let wave = if limb == 0 { stride } else { -stride };
        let mut sum = 0.;
        let mut weight = 0.;
        for y in bottom.saturating_sub(6)..=(bottom.min(127)) {
            for x in 0..128usize {
                let a = source.bytes[(y * 128 + x) * 4 + 3] as f32;
                if ((x as f32) < center) == (limb == 0) && a >= 64. {
                    sum += x as f32 * a;
                    weight += a;
                }
            }
        }
        let foot = if weight > 0. {
            sum / weight
        } else {
            center + if limb == 0 { -8. } else { 8. }
        };
        // A standing sprite already has an open stance. Walking first brings
        // each foot under its hip; adding a stride to the idle spread was wrong.
        let advance = if pose.facing == 0 {
            (center - foot) * 0.10
        } else {
            (center - foot) * 0.65 + wave * leg_height * 0.18
        };
        let lift = wave.max(0.) * leg_height * 0.20;
        let knee = hip as f32 + leg_height * 0.49;
        let knee_x = advance * 0.47;
        let knee_lift = lift * 0.45;
        // Two overlapping bones keep the knee connected. Inverse sampling
        // changes the joint angle, rather than sliding a strip of shin pixels.
        for upper in [true, false] {
            let (y0, y1, dy0, dy1, dx0, dx1) = if upper {
                (hip as f32 - 2., knee + 2., 0., -knee_lift, 0., knee_x)
            } else {
                (
                    knee - 2.,
                    bottom as f32 + 2.,
                    -knee_lift,
                    -lift,
                    knee_x,
                    advance,
                )
            };
            let mut segment = Image::gen_image_color(128, 128, BLANK);
            for y in (y0.max(0.) as usize)..(y1.ceil() as usize).min(128) {
                for x in 0..128usize {
                    if ((x as f32) < center) == (limb == 0) {
                        let p = (y * 128 + x) * 4;
                        segment.bytes[p..p + 4].copy_from_slice(&source.bytes[p..p + 4]);
                    }
                }
            }
            let sy = ((y1 + dy1) - (y0 + dy0)) / (y1 - y0);
            let sh = (dx1 - dx0) / (y1 - y0);
            let segment = geometry::affine(&segment, [0., y0], [dx0, y0 + dy0], [1., sh, 0., sy]);
            geometry::over(&mut out, &segment);
        }
    }
    // The complete torso closes the hip attachment. Arms/gear are added later.
    let mut upper = source.clone();
    upper.bytes[(hip + 1).min(128) * 512..].fill(0);
    geometry::over(&mut out, &upper);
    out
}

fn id(key: Key) -> Option<String> {
    if (17..=22).contains(&key.0) {
        return Some(format!("ACTOR-{:02}", key.0 + 3));
    }
    if let Some((index, _)) = decode(key.0) {
        return Some(format!("CLASS-{index:02}"));
    }
    let index = key.0.checked_sub(200).filter(|i| *i < 19)?;
    let p = palettes::authored_variant(key.2[0]);
    Some(if index < 12 {
        format!("ARMOR-flat-agents-{index:02}-{p}")
    } else {
        format!("ARMOR-equipment-native64-{:02}-{p}", index - 12)
    })
}

// WALK07 is a review-only source. It never enables WALK04 in the game.
#[cfg(test)]
thread_local! { static REVIEW07: std::cell::Cell<bool> = const { std::cell::Cell::new(false) }; }
#[cfg(test)]
#[derive(Clone)]
struct Recipe07 {
    image: Image,
    pivot: [f32; 2],
    target: [f32; 2],
    matrix: [f32; 4],
}
#[cfg(test)]
thread_local! {
    static RECIPE07: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static RECIPES07: std::cell::RefCell<Vec<Vec<Recipe07>>> = const { std::cell::RefCell::new(Vec::new()) };
}
fn record07(_group: usize, _image: &Image, _pivot: [f32; 2], _target: [f32; 2], _matrix: [f32; 4]) {
    #[cfg(test)]
    if RECIPE07.get() {
        RECIPES07.with_borrow_mut(|groups| {
            groups[_group].push(Recipe07 {
                image: _image.clone(),
                pivot: _pivot,
                target: _target,
                matrix: _matrix,
            })
        });
    }
}

fn direction_parts(key: Key, pose: Pose) -> Option<(&'static Rig, Vec<Image>, bool)> {
    let id = id(key)?;
    #[cfg(test)]
    if REVIEW07.get() {
        #[derive(Deserialize)]
        struct Proposed {
            frame: u8,
            sheet_row: u16,
            #[serde(flatten)]
            rig: Rig,
        }
        static PROPOSED: OnceLock<Vec<Proposed>> = OnceLock::new();
        let entries = PROPOSED.get_or_init(|| {
            serde_json::from_str(
                &std::fs::read_to_string(
                    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                        .join("assets/source-art/player-walk-review-07/rigs/manifest.json"),
                )
                .expect("WALK07 proposed rigs"),
            )
            .unwrap()
        });
        let entry = entries
            .iter()
            .find(|e| e.rig.id == id && e.rig.facing == pose.facing && e.frame == pose.frame)?;
        let cache_key = format!("{}:{}", entry.rig.file, entry.sheet_row);
        let parts = PARTS.with_borrow_mut(|cache| {
            if let Some(parts) = cache.get(&cache_key) {
                return Some(parts.clone());
            }
            let bytes = std::fs::read(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(&entry.rig.file),
            )
            .ok()?;
            let image = Image::from_file_with_format(&bytes, Some(ImageFormat::Png)).ok()?;
            let parts: Vec<_> = (0..5)
                .map(|n| {
                    image.sub_image(Rect::new(
                        n as f32 * 128.,
                        f32::from(entry.sheet_row) * 128.,
                        128.,
                        128.,
                    ))
                })
                .collect();
            cache.insert(cache_key, parts.clone(), 5 * 128 * 128 * 4);
            Some(parts)
        })?;
        return Some((&entry.rig, parts, true));
    }
    if decode(key.0).is_some() && [-1, 1].contains(&pose.facing) {
        #[derive(Deserialize)]
        struct Approved {
            frame: u8,
            sheet_row: u16,
            #[serde(flatten)]
            rig: Rig,
        }
        static APPROVED: OnceLock<Vec<Approved>> = OnceLock::new();
        static COMMON: OnceLock<Vec<Approved>> = OnceLock::new();
        let common = crate::game_art::walking::common_cycle_enabled();
        let entries = if common { &COMMON } else { &APPROVED }.get_or_init(|| {
            serde_json::from_str(if common {
                include_str!("../assets/animations/walk-09/player/manifest.json")
            } else {
                include_str!("../assets/animations/walk-08/player/manifest.json")
            })
            .expect("whole player walk poses")
        });
        let entries = if crate::game_art::walking::proportions_enabled() {
            static PROPOSED: OnceLock<Vec<Approved>> = OnceLock::new();
            PROPOSED.get_or_init(|| {
                serde_json::from_str(include_str!(
                    "../assets/animations/walk-11/player/manifest.json"
                ))
                .expect("WALK11 accepted player proportions and GRIP-r03")
            })
        } else {
            entries
        };
        let entries = if crate::game_art::walking::gait12_enabled() {
            static ACCEPTED12: OnceLock<Vec<Approved>> = OnceLock::new();
            let accepted = ACCEPTED12.get_or_init(|| {
                serde_json::from_str(include_str!(
                    "../assets/animations/walk-12/player/manifest.json"
                ))
                .expect("WALK12 accepted complete figures")
            });
            if accepted.iter().any(|e| e.rig.id == id) {
                accepted
            } else {
                entries
            }
        } else {
            entries
        };
        #[cfg(test)]
        let entries = if PROPORTIONS10.get() && id == "CLASS-00" {
            static PROPORTIONS: OnceLock<Vec<Approved>> = OnceLock::new();
            PROPORTIONS.get_or_init(|| {
                serde_json::from_str(include_str!(
                    "../assets/source-art/player-walk-review-10/candidate/manifest.json"
                ))
                .expect("Bastion proportions proposal")
            })
        } else {
            entries
        };
        let entry = entries
            .iter()
            .find(|e| e.rig.id == id && e.rig.facing == pose.facing && e.frame == pose.frame)?;
        let cache_key = format!("{}:{}", entry.rig.file, entry.sheet_row);
        let parts = PARTS.with_borrow_mut(|cache| {
            if let Some(parts) = cache.get(&cache_key) {
                return Some(parts.clone());
            }
            let image = Image::from_file_with_format(
                &project_rl::data_paths::read(&entry.rig.file).ok()?,
                Some(ImageFormat::Png),
            )
            .ok()?;
            if (image.width, image.height) != (640, 896) {
                return None;
            }
            let parts: Vec<_> = (0..5)
                .map(|n| {
                    image.sub_image(Rect::new(
                        n as f32 * 128.,
                        entry.sheet_row as f32 * 128.,
                        128.,
                        128.,
                    ))
                })
                .collect();
            cache.insert(cache_key, parts.clone(), 5 * 128 * 128 * 4);
            Some(parts)
        })?;
        return Some((&entry.rig, parts, true));
    }
    let rig = rigs()
        .iter()
        .find(|r| r.id == id && r.facing == pose.facing)?;
    Some((rig, parts(rig)?, false))
}

fn front_layers(key: Key) -> Option<Vec<Image>> {
    {
        thread_local! { static FRONTS: std::cell::RefCell<project_rl::asset_cache::ByteCache<(bool,Key),Vec<Image>>> = const {
            std::cell::RefCell::new(project_rl::asset_cache::ByteCache::new(24*1024*1024))
        }; }
        return FRONTS.with_borrow_mut(|cache| {
            let cache_key = (crate::game_art::walking::corrections_enabled(), key);
            if let Some(images) = cache.get(&cache_key) {
                return Some(images.clone());
            }
            let images = super::layers(key)?;
            cache.insert(
                cache_key,
                images.clone(),
                images.iter().map(|i| i.bytes.len()).sum(),
            );
            Some(images)
        });
    }
}

// Rig landmarks are measured as pixel indices. The shared affine sampler
// expects pixel-centre coordinates; without this conversion a reflection
// places the weapon one pixel away from the hand.
fn affine_pixels(source: &Image, pivot: [f32; 2], target: [f32; 2], matrix: [f32; 4]) -> Image {
    geometry::affine(
        source,
        pivot.map(|v| v + 0.5),
        target.map(|v| v + 0.5),
        matrix,
    )
}

fn front_two_hand_pixels(
    index: usize,
    model: u16,
    anchor: [f32; 2],
    support: [f32; 2],
) -> [[f32; 2]; 2] {
    if let Some(weapon) = readable::binding(model) {
        return weapon.raster_targets(index);
    }
    let (points, scale) = two_hand_targets(index, model, anchor, support);
    let shift = if matches!(weapon_hold(model), WeaponHold::TwoHanded) {
        0.5 * (scale - 1.)
    } else {
        0.
    };
    points.map(|p| [p[0] + shift, p[1] + shift])
}

fn front_single_pixel(index: usize, model: u16, left: bool, raised: bool) -> Option<[f32; 2]> {
    let side = usize::from(!left);
    let rig = &ARMOR_RIGS[index];
    if raised {
        let p = raised_palm(rig, side);
        Some(if left {
            [p[0] - 1., p[1]]
        } else {
            [p[0], p[1] - 1.]
        })
    } else {
        let source = attachment(model, left)?.0;
        let p = rig.palms[side];
        Some([
            source[0] + (p[0] - source[0]).round(),
            source[1] + (p[1] - source[1]).round(),
        ])
    }
}

fn walking_head_anchor(body: &Image, rig: &Rig) -> Option<[f32; 2]> {
    // The torso centre and the front outfit's head height are not landmarks
    // of the painted walking head. Measure the crown before adding equipment;
    // the upper half of the head excludes the neck and shoulder silhouette.
    let top = body.bytes.chunks_exact(4).position(|p| p[3] >= 96)? / 128;
    let shoulders = rig.pivots[0][1].min(rig.pivots[1][1]);
    let end = (top as f32 + (shoulders - top as f32) * 0.5).round() as usize;
    let mut left = 128;
    let mut right = 0;
    for y in top..end.max(top + 1).min(128) {
        for x in 0..128 {
            if body.bytes[(y * 128 + x) * 4 + 3] >= 96 {
                left = left.min(x);
                right = right.max(x);
            }
        }
    }
    // Every existing helmet has its crown at y35 and its attachment at y40.
    Some([(left + right) as f32 / 2., top as f32 + 5.])
}

fn walking_head_roll(body: &Image, at: [f32; 2]) -> f32 {
    let neck_y = (at[1] + 5.).round().clamp(0., 127.) as usize;
    let mut left = 128;
    let mut right = 0;
    for x in 0..128 {
        if body.bytes[(neck_y * 128 + x) * 4 + 3] >= 96 {
            left = left.min(x);
            right = right.max(x);
        }
    }
    if left == 128 {
        return 0.;
    }
    (at[0] - (left + right) as f32 / 2.)
        .atan2(10.)
        .clamp(-0.18, 0.18)
}

fn turned_helmet(source: &Image, facing: i8) -> Image {
    // Wrap the existing frontal shell around a rounded head, then turn that
    // surface. Facial openings stay transparent: the actual painted walking
    // face underneath must not be replaced by the static frontal identity.
    let mut turned = Image::gen_image_color(128, 128, BLANK);
    let yaw = f32::from(facing) * 0.65;
    for y in 0..128 {
        let row = &source.bytes[y * 512..(y + 1) * 512];
        let Some(left) = row.chunks_exact(4).position(|p| p[3] >= 96) else {
            continue;
        };
        let right = row.chunks_exact(4).rposition(|p| p[3] >= 96).unwrap();
        let center = (left + right) as f32 / 2.;
        let radius = (right - left + 1) as f32 / 2.;
        for x in left..=right {
            let angle = ((x as f32 - center) / radius).clamp(-1., 1.).asin() - yaw;
            let back = angle.abs() > std::f32::consts::FRAC_PI_2;
            let sx = if back {
                if angle < 0. { left } else { right }
            } else {
                (center + radius * angle.sin())
                    .round()
                    .clamp(left as f32, right as f32) as usize
            };
            let from = sx * 4;
            let to = (y * 128 + x) * 4;
            turned.bytes[to..to + 4].copy_from_slice(&row[from..from + 4]);
            if back {
                // The newly exposed temple keeps the shell's existing edge
                // material and dark outline, rather than repeating a visor.
                for c in 0..3 {
                    turned.bytes[to + c] = (u16::from(turned.bytes[to + c]) * 4 / 5) as u8;
                }
            }
        }
    }
    turned
}

fn walk_helmet(body: &mut Image, key: Key, pose: Pose, rig: &Rig) {
    if let Some((_, head)) = decode(key.0)
        && head > 0
        && let Some(at) = walking_head_anchor(body, rig)
    {
        let head_image = turned_helmet(&part(head - 1, "helmets"), pose.facing);
        let (sin, cos) = walking_head_roll(body, at).sin_cos();
        geometry::over(
            body,
            &affine_pixels(
                &head_image,
                [64., 40.],
                at,
                [cos * 0.88, -sin, sin * 0.88, cos],
            ),
        );
    }
}

pub(super) fn layers(key: Key, pose: Pose) -> Option<Vec<Image>> {
    if pose.facing == 0 && !(17..=22).contains(&key.0) {
        // Real front body/arms, real selected weapons and fingers. Only the
        // owned lower body is articulated; no carried shaft enters this mask.
        let mut result = if decode(key.0).is_some() {
            super::layers(key)?
        } else {
            generic::layers(key)?
        };
        let top = result[0]
            .bytes
            .chunks_exact(4)
            .enumerate()
            .filter(|(_, p)| p[3] >= 128)
            .map(|(i, _)| i / 128)
            .min()?;
        let bottom = result[0]
            .bytes
            .chunks_exact(4)
            .enumerate()
            .filter(|(_, p)| p[3] >= 128)
            .map(|(i, _)| i / 128)
            .max()?;
        let hip = (top as f32 + (bottom - top) as f32 * 0.60).round() as u16;
        result[0] = drawn(&result[0], key, pose)
            .unwrap_or_else(|| gait(&result[0], hip, bottom as u16, 64., pose));
        return Some(result);
    }
    let (rig, p, authored_pose) = direction_parts(key, pose)?;
    let fixed_grips = authored_pose && crate::game_art::walking::proportions_enabled();
    #[cfg(test)]
    if authored_pose && RECIPE07.get() {
        RECIPES07.with_borrow_mut(|groups| *groups = vec![Vec::new(); 5]);
        record07(0, &p[0], [0., 0.], [0., 0.], [1., 0., 0., 1.]);
    }
    let mut body = if authored_pose {
        p[0].clone()
    } else {
        drawn(&p[0], key, pose)
            .unwrap_or_else(|| gait(&p[0], rig.hip, rig.bottom, rig.center, pose))
    };
    if fixed_grips && pose.frame != 8 && key.1.iter().any(Option::is_some) {
        // The carried assembly keeps one authored arm pose for this exact
        // outfit, direction and loadout. Only the body's small root motion is
        // shared; elbows and grips no longer change with each leg drawing.
        let idle = Pose { frame: 8, ..pose };
        let (reference, _, _) = direction_parts(key, idle)?;
        let delta = [
            (rig.center - reference.center).round(),
            (rig.pivots[0][1] - reference.pivots[0][1]).round(),
        ];
        let carried = layers(key, idle)?;
        walk_helmet(&mut body, key, pose, rig);
        let mut result = vec![body];
        result.extend(carried.into_iter().skip(1).map(|image| {
            if delta == [0., 0.] {
                image
            } else {
                affine_pixels(&image, [0., 0.], delta, [1., 0., 0., 1.])
            }
        }));
        return Some(result);
    }
    let mut weapons = Image::gen_image_color(128, 128, BLANK);
    let mut fingers = weapons.clone();
    let held: Vec<_> = key.1.into_iter().flatten().collect();
    let mut targets = rig.palms;
    let mut occupied = [false; 2];
    if let [layer] = held.as_slice()
        && let (anchor, Some(support)) = attachment(layer.model, layer.left)?
    {
        // Start from the production weapon layer, which already owns the
        // model-specific angle and reviewed scale (especially rifle model 2).
        let (front, grips) = if let Some((index, _)) = decode(key.0) {
            (
                front_layers(key)?,
                if fixed_grips {
                    front_two_hand_pixels(index, layer.model, anchor, support)
                } else {
                    two_hand_targets(index, layer.model, anchor, support).0
                },
            )
        } else if (17..=22).contains(&key.0) {
            let mut native = key;
            native.0 = BASE;
            (
                front_layers(native)?,
                two_hand_targets(0, layer.model, anchor, support).0,
            )
        } else {
            (generic::layers(key)?, [anchor, support])
        };
        let source = &front[1];
        let [anchor, support] = grips;
        let scale = 1.;
        let center = [(anchor[0] + support[0]) / 2., (anchor[1] + support[1]) / 2.];
        let at = [rig.center, rig.pivots[0][1] + 11.];
        let mirror = if pose.facing < 0 { -1. } else { 1. };
        let matrix = [scale * mirror, 0., 0., scale];
        record07(3, source, center, at, matrix);
        weapons = if fixed_grips {
            affine_pixels(source, center, at, matrix)
        } else {
            geometry::affine(source, center, at, matrix)
        };
        targets = [anchor, support].map(|a| {
            [
                at[0] + (a[0] - center[0]) * scale * mirror,
                at[1] + (a[1] - center[1]) * scale,
            ]
        });
        // Assign the two grips to the corresponding arm in this projection.
        if targets[0][0] > targets[1][0] {
            targets.swap(0, 1);
        }
        occupied = [true, true];
    } else {
        for layer in held {
            let side = usize::from(!layer.left);
            occupied[side] = true;
            let raised = matches!(weapon_hold(layer.model), WeaponHold::Raised);
            let angle = if raised && pose.facing != 0 && !authored_pose {
                if pose.facing < 0 {
                    -std::f32::consts::FRAC_PI_2
                } else {
                    std::f32::consts::FRAC_PI_2
                }
            } else {
                0.
            };
            let at = if raised {
                [
                    rig.pivots[side][0] + f32::from(pose.facing) * 11.,
                    rig.pivots[side][1] + 3.,
                ]
            } else {
                rig.palms[side]
            };
            targets[side] = at;
            let mut single = key;
            single.1 = if layer.left {
                [Some(layer), None]
            } else {
                [None, Some(layer)]
            };
            let (front, anchor) = if let Some((index, _)) = decode(key.0) {
                (
                    front_layers(single)?,
                    if fixed_grips {
                        front_single_pixel(index, layer.model, layer.left, raised)?
                    } else if raised {
                        raised_palm(&ARMOR_RIGS[index], side)
                    } else {
                        ARMOR_RIGS[index].palms[side]
                    },
                )
            } else if (17..=22).contains(&key.0) {
                let mut native = single;
                native.0 = BASE;
                (
                    front_layers(native)?,
                    if raised {
                        raised_palm(&ARMOR_RIGS[0], side)
                    } else {
                        ARMOR_RIGS[0].palms[side]
                    },
                )
            } else {
                (
                    generic::layers(single)?,
                    attachment(layer.model, layer.left)?.0,
                )
            };
            let source = &front[1];
            let (s, c) = angle.sin_cos();
            let source_faces_left = raised && layer.left;
            let mirror = if fixed_grips && (pose.facing < 0) != source_faces_left {
                -1.
            } else {
                1.
            };
            let matrix = [c * mirror, -s, s * mirror, c];
            record07(3, source, anchor, at, matrix);
            geometry::over(
                &mut weapons,
                &if fixed_grips {
                    affine_pixels(source, anchor, at, matrix)
                } else {
                    geometry::affine(source, anchor, at, matrix)
                },
            );
        }
    }
    // Both sleeves and fists are owned pixels from this exact painted view.
    // They remain attached at the shoulder through every support phase.
    let mut authored_arms = Vec::new();
    for side in 0..2 {
        let mut at = targets[side];
        if !occupied[side] && pose.frame != 8 && !authored_pose {
            at[0] += phase(pose.frame) * if side == 0 { 1.8 } else { -1.8 };
            at[1] -= phase(pose.frame).abs();
        }
        let mut arm_source = p[1 + side * 2].clone();
        if fixed_grips && occupied[side] {
            erase_owned(&mut arm_source, &p[2 + side * 2]);
        }
        let arm = if fixed_grips && at == rig.palms[side] {
            arm_source.clone()
        } else if fixed_grips {
            affine_pixels(
                &arm_source,
                rig.pivots[side],
                rig.pivots[side],
                limb_matrix(rig.pivots[side], rig.palms[side], at),
            )
        } else {
            pose_limb(&arm_source, rig.pivots[side], rig.palms[side], at)
        };
        let old = [
            rig.palms[side][0] - rig.pivots[side][0],
            rig.palms[side][1] - rig.pivots[side][1],
        ];
        let new = [at[0] - rig.pivots[side][0], at[1] - rig.pivots[side][1]];
        let hand_angle = new[1].atan2(new[0]) - old[1].atan2(old[0]);
        let rigid_hand = || {
            let (s, c) = hand_angle.sin_cos();
            affine_pixels(&p[2 + side * 2], rig.palms[side], at, [c, -s, s, c])
        };
        // The rigid glove is drawn once, above the weapon in `fingers` below.
        // Drawing it both here and there darkens translucent edge pixels and
        // leaves a second outline around the grip.
        record07(
            1 + side,
            &p[1 + side * 2],
            rig.pivots[side],
            rig.pivots[side],
            limb_matrix(rig.pivots[side], rig.palms[side], at),
        );
        if authored_pose {
            authored_arms.push(arm);
        } else {
            geometry::over(&mut body, &arm);
        }
        if occupied[side] {
            record07(
                4,
                &p[2 + side * 2],
                rig.pivots[side],
                rig.pivots[side],
                limb_matrix(rig.pivots[side], rig.palms[side], at),
            );
            geometry::over(
                &mut fingers,
                &if fixed_grips {
                    rigid_hand()
                } else {
                    pose_limb(&p[2 + side * 2], rig.pivots[side], rig.palms[side], at)
                },
            );
        }
    }
    walk_helmet(&mut body, key, pose, rig);
    if authored_pose {
        let mut result = vec![body];
        result.extend(authored_arms);
        result.extend([weapons, fingers]);
        Some(result)
    } else {
        Some(vec![body, weapons, fingers])
    }
}

#[cfg(test)]
#[path = "walk_proportions_player_export.rs"]
mod proportions_export;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equipped_helmets_stay_on_the_painted_head_through_every_pose() {
        for head in 0..6 {
            let source = part(head, "helmets");
            let left = turned_helmet(&source, -1);
            let right = turned_helmet(&source, 1);
            assert_ne!(
                left.bytes, right.bytes,
                "helmet{head} still faces front in both directions"
            );
            if head >= 4 {
                for (facing, image) in [(-1i8, left), (1i8, right)] {
                    let visor: Vec<_> = image
                        .bytes
                        .chunks_exact(4)
                        .enumerate()
                        .filter(|(_, p)| {
                            p[3] >= 96
                                && p[1] > 80
                                && u16::from(p[1]) * 4 > u16::from(p[0]) * 5
                                && u16::from(p[2]) * 10 > u16::from(p[0]) * 11
                        })
                        .map(|(n, _)| (n % 128) as f32)
                        .collect();
                    assert!(!visor.is_empty());
                    let x = visor.iter().sum::<f32>() / visor.len() as f32;
                    assert!(
                        (x - 64.) * f32::from(facing) > 0.5,
                        "visor faces opposite the walking head: helmet{head} facing{facing} x{x}"
                    );
                }
            }
        }
        for index in 0..25 {
            let bare = (BASE + index, [None; 2], [EquipmentPalette::Original; 3]);
            for facing in [-1, 1] {
                for frame in [8, 0, 1, 2, 3, 4, 5] {
                    let pose = Pose { facing, frame };
                    let (rig, _, _) = direction_parts(bare, pose).unwrap();
                    let original = layers(bare, pose).unwrap().remove(0);
                    let top = original
                        .bytes
                        .chunks_exact(4)
                        .position(|p| p[3] >= 96)
                        .unwrap()
                        / 128;
                    let shoulders = rig.pivots[0][1].min(rig.pivots[1][1]).round() as usize;
                    for head in 1..=6 {
                        let key = (BASE + index + head * HEAD_STRIDE, bare.1, bare.2);
                        let actual = layers(key, pose).unwrap().remove(0);
                        let changed: Vec<_> = actual
                            .bytes
                            .chunks_exact(4)
                            .zip(original.bytes.chunks_exact(4))
                            .enumerate()
                            .filter(|(_, (a, b))| a[3] >= 96 && a != b)
                            .map(|(n, _)| (n % 128, n / 128))
                            .collect();
                        assert!(!changed.is_empty());
                        let crown = changed.iter().map(|p| p.1).min().unwrap();
                        assert!(
                            crown.abs_diff(top) <= 1,
                            "floating helmet: outfit{index} head{head} facing{facing} frame{frame}, crown{crown} head top{top}"
                        );
                        let touching = changed
                            .iter()
                            .filter(|&&(x, y)| {
                                (y.saturating_sub(1)..=(y + 1).min(127)).any(|yy| {
                                    (x.saturating_sub(1)..=(x + 1).min(127))
                                        .any(|xx| original.bytes[(yy * 128 + xx) * 4 + 3] >= 96)
                                })
                            })
                            .count();
                        assert!(
                            touching * 100 >= changed.len() * 80,
                            "detached helmet: outfit{index} head{head} facing{facing} frame{frame}, contact{touching}/{}",
                            changed.len()
                        );
                        assert_eq!(
                            &actual.bytes[shoulders * 512..],
                            &original.bytes[shoulders * 512..],
                            "helmet changed torso/legs: outfit{index} head{head} facing{facing} frame{frame}"
                        );
                        if head <= 2 {
                            let at = walking_head_anchor(&original, rig).unwrap();
                            let (sin, cos) = walking_head_roll(&original, at).sin_cos();
                            let shell = affine_pixels(
                                &turned_helmet(&part(usize::from(head - 1), "helmets"), facing),
                                [64., 40.],
                                at,
                                [cos * 0.88, -sin, sin * 0.88, cos],
                            );
                            for (n, pixel) in shell.bytes.chunks_exact(4).enumerate() {
                                if pixel[3] == 0 {
                                    assert_eq!(
                                        &actual.bytes[n * 4..n * 4 + 4],
                                        &original.bytes[n * 4..n * 4 + 4],
                                        "frontal face substituted in an open helmet: outfit{index} head{head} facing{facing} frame{frame}"
                                    );
                                }
                            }
                        }
                        if frame == 1 {
                            for held in [
                                selection(Some(67), None),
                                selection(None, Some([Some(68), Some(101)])),
                            ] {
                                let armed = layers((key.0, held, key.2), pose).unwrap();
                                assert_eq!(
                                    actual.bytes, armed[0].bytes,
                                    "weapon moved helmet: outfit{index} head{head} facing{facing}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn equipped_arms_and_weapons_keep_the_same_pose_through_all_steps() {
        for index in 0..25 {
            let mut loadouts = vec![selection(Some(67), None)];
            if index % 5 == 0 {
                loadouts.push(selection(Some(101), None));
                loadouts.push(selection(None, Some([Some(68), Some(101)])));
            }
            for held in loadouts {
                let key = (BASE + index, held, [EquipmentPalette::Original; 3]);
                for facing in [-1, 1] {
                    let idle = Pose { facing, frame: 8 };
                    let reference = layers(key, idle).unwrap();
                    let (base, _, _) = direction_parts(key, idle).unwrap();
                    let mut bodies = std::collections::BTreeSet::new();
                    for frame in 0..6 {
                        let pose = Pose { facing, frame };
                        let actual = layers(key, pose).unwrap();
                        let (rig, _, _) = direction_parts(key, pose).unwrap();
                        let delta = [
                            (rig.center - base.center).round(),
                            (rig.pivots[0][1] - base.pivots[0][1]).round(),
                        ];
                        for part in 1..5 {
                            let expected =
                                affine_pixels(&reference[part], [0., 0.], delta, [1., 0., 0., 1.]);
                            let differences: Vec<_> = actual[part]
                                .bytes
                                .chunks_exact(4)
                                .zip(expected.bytes.chunks_exact(4))
                                .enumerate()
                                .filter(|(_, (a, b))| a[3] != b[3] || (a[3] != 0 && a != b))
                                .map(|(i, (a, b))| (i, a.to_vec(), b.to_vec()))
                                .take(8)
                                .collect();
                            assert!(
                                differences.is_empty(),
                                "outfit{index}, facing{facing}, frame{frame}, layer{part}: {differences:?}"
                            );
                        }
                        bodies.insert(actual[0].bytes.clone());
                    }
                    assert_eq!(
                        bodies.len(),
                        6,
                        "leg cycle changed: outfit{index} facing{facing}"
                    );
                }
            }
        }
    }

    #[test]
    fn accepted_common_gait_keeps_whole_legs_rigid_grips_and_scoped_legacy_comparison() {
        let key = (
            BASE,
            selection(Some(67), None),
            [EquipmentPalette::Original; 3],
        );
        let pose = Pose {
            facing: 1,
            frame: 1,
        };
        assert!(crate::game_art::walking::gait12_enabled());
        let ordinary = layers(key, pose).unwrap();
        {
            let _legacy = crate::game_art::walking::review_gait12(false);
            assert!(!crate::game_art::walking::gait12_enabled());
            assert!(
                direction_parts(key, pose)
                    .unwrap()
                    .0
                    .file
                    .contains("walk-11/player/")
            );
            assert_ne!(layers(key, pose).unwrap()[0].bytes, ordinary[0].bytes);
        }
        {
            let _proposal = crate::game_art::walking::review_gait12(true);
            for index in 0..25 {
                let key = (BASE + index, [None; 2], [EquipmentPalette::Original; 3]);
                for facing in [-1, 1] {
                    let mut heights = Vec::new();
                    for frame in [8, 0, 1, 2, 3, 4, 5] {
                        let (rig, parts, authored) =
                            direction_parts(key, Pose { facing, frame }).unwrap();
                        assert!(
                            authored && rig.file.contains("walk-12/player/"),
                            "missing proposed outfit{index}"
                        );
                        let [x0, y0, x1, y1] = rig.leg_guard.unwrap();
                        for y in y0..y1 {
                            for x in x0..x1 {
                                let at = (usize::from(y) * 128 + usize::from(x)) * 4 + 3;
                                assert!(
                                    parts[1..].iter().all(|p| p.bytes[at] == 0),
                                    "leg ownership: outfit{index} frame{frame}"
                                );
                            }
                        }
                        let top = parts[0]
                            .bytes
                            .chunks_exact(4)
                            .position(|p| p[3] >= 96)
                            .unwrap()
                            / 128;
                        let bottom = parts[0]
                            .bytes
                            .chunks_exact(4)
                            .rposition(|p| p[3] >= 96)
                            .unwrap()
                            / 128;
                        assert_eq!(bottom, 95);
                        heights.push(bottom - top + 1);
                    }
                    assert!(
                        heights.iter().max().unwrap() - heights.iter().min().unwrap() <= 2,
                        "scale jump: outfit{index}, {heights:?}"
                    );
                }
            }
            equipped_arms_and_weapons_keep_the_same_pose_through_all_steps();
        }
        assert!(crate::game_art::walking::gait12_enabled());
        let restored = layers(key, pose).unwrap();
        assert!(
            restored
                .iter()
                .zip(ordinary.iter())
                .all(|(a, b)| a.bytes == b.bytes)
        );
    }

    #[test]
    fn grip_pixel_centres_stay_on_the_target_under_mirrors_and_turns() {
        let mut source = Image::gen_image_color(128, 128, BLANK);
        let at = (43 * 128 + 31) * 4;
        source.bytes[at..at + 4].copy_from_slice(&[120, 180, 220, 255]);
        for matrix in [
            [1., 0., 0., 1.],
            [-1., 0., 0., 1.],
            [0., -1., 1., 0.],
            [0., 1., -1., 0.],
        ] {
            let result = affine_pixels(&source, [31., 43.], [73., 62.], matrix);
            let pixels: Vec<_> = result
                .bytes
                .chunks_exact(4)
                .enumerate()
                .filter(|(_, p)| p[3] != 0)
                .collect();
            assert_eq!(pixels.len(), 1);
            assert_eq!(pixels[0].0, 62 * 128 + 73);
            assert_eq!(pixels[0].1, &[120, 180, 220, 255]);
        }
    }

    #[test]
    #[ignore = "Read-only WALK10 before/after export; RL_WALK10_OUTPUT required"]
    fn walk10_proportions_diagnostic() {
        let output =
            std::path::PathBuf::from(std::env::var("RL_WALK10_OUTPUT").expect("output required"));
        std::fs::create_dir_all(&output).unwrap();
        assert!(!PROPORTIONS10.get());
        for weapon in [None, Some(67)] {
            let label = if weapon.is_some() { "axe" } else { "unarmed" };
            let key = (
                BASE,
                selection(weapon, None),
                [EquipmentPalette::Original; 3],
            );
            let compose = |layers: Vec<Image>| {
                let mut image = Image::gen_image_color(128, 128, BLANK);
                for layer in layers {
                    geometry::over(&mut image, &layer);
                }
                image
            };
            compose(super::super::layers(key).unwrap())
                .export_png(output.join(format!("{label}-front.png")).to_str().unwrap());
            for candidate in [false, true] {
                PROPORTIONS10.set(candidate);
                for facing in [-1, 1] {
                    for frame in [8, 0, 1, 2, 3, 4, 5] {
                        compose(layers(key, Pose { facing, frame }).unwrap()).export_png(
                            output
                                .join(format!(
                                    "{label}-{}-{facing}-{frame}.png",
                                    if candidate { "candidate" } else { "current" }
                                ))
                                .to_str()
                                .unwrap(),
                        );
                    }
                }
            }
            PROPORTIONS10.set(false);
        }
        std::fs::write(output.join("capture.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "normal_game_changed":false,"source":"ordinary outfit/weapon compositor with explicit test-only candidate body selection",
            "class":"Bastion","outfit":1,"weapon_models":[67],"unarmed":true,"poses":14,
            "exported_pngs_need_vertical_flip":true
        })).unwrap()).unwrap();
    }

    #[test]
    fn common_cycle_keeps_all_350_whole_poses_and_leg_armor_out_of_moving_arms() {
        let _legacy = crate::game_art::walking::review_gait12(false);
        let _review = crate::game_art::walking::review_corrections();
        let _common = crate::game_art::walking::review_common(true);
        for index in 0..25 {
            let key = (BASE + index, [None; 2], [EquipmentPalette::Original; 3]);
            for facing in [-1, 1] {
                let mut heights = Vec::new();
                for frame in [8, 0, 1, 2, 3, 4, 5] {
                    let (rig, parts, authored) =
                        direction_parts(key, Pose { facing, frame }).unwrap();
                    assert!(authored && rig.file.contains("walk-11/player/"));
                    let [x0, y0, x1, y1] = rig.leg_guard.expect("whole leg armor guard");
                    let mut core_pixels = 0;
                    for y in y0..y1 {
                        for x in x0..x1 {
                            let at = (usize::from(y) * 128 + usize::from(x)) * 4 + 3;
                            core_pixels += usize::from(parts[0].bytes[at] > 0);
                            for arm in [1, 2, 3, 4] {
                                assert_eq!(
                                    parts[arm].bytes[at], 0,
                                    "leg armor owned by arm: outfit{index} frame{frame}"
                                );
                            }
                        }
                    }
                    assert!(core_pixels > 0);
                    let ys: Vec<_> = parts[0]
                        .bytes
                        .chunks_exact(4)
                        .enumerate()
                        .filter(|(_, p)| p[3] >= 96)
                        .map(|(i, _)| i / 128)
                        .collect();
                    assert_eq!(*ys.last().unwrap(), 95);
                    heights.push(ys.last().unwrap() - ys[0] + 1);
                }
                assert!(
                    heights.iter().max().unwrap() - heights.iter().min().unwrap() <= 2,
                    "scale jump: outfit{index} {heights:?}"
                );
            }
        }
    }

    #[test]
    fn technomancer_outfit_five_rightward_cycle_keeps_size_and_ground_contact() {
        let _review = crate::game_art::walking::review_corrections();
        let _historical = crate::game_art::walking::review_proportions(false);
        let key = (BASE + 9, [None; 2], [EquipmentPalette::Original; 3]);
        let mut heights = Vec::new();
        for frame in [8, 0, 1, 2, 3, 4, 5] {
            let (rig, parts, authored) = direction_parts(key, Pose { facing: 1, frame }).unwrap();
            assert!(authored);
            assert!(rig.file.ends_with(if frame < 3 || frame == 8 {
                "CLASS-09-1.png"
            } else {
                "CLASS-09-1-r02.png"
            }));
            let ys: Vec<_> = parts[0]
                .bytes
                .chunks_exact(4)
                .enumerate()
                .filter(|(_, p)| p[3] >= 96)
                .map(|(i, _)| i / 128)
                .collect();
            let top = *ys.first().unwrap();
            let bottom = *ys.last().unwrap();
            assert_eq!(bottom, 95, "floating feet in phase {frame}");
            heights.push(bottom - top + 1);
        }
        assert!(
            *heights.iter().max().unwrap() - *heights.iter().min().unwrap() <= 1,
            "body scale jumps: {heights:?}"
        );
    }

    #[test]
    fn technomancer_outfit_one_optic_faces_the_requested_direction_in_every_pose() {
        let _review = crate::game_art::walking::review_corrections();
        let _historical = crate::game_art::walking::review_proportions(false);
        let key = (BASE + 5, [None; 2], [EquipmentPalette::Original; 3]);
        for facing in [-1, 1] {
            for frame in [8, 0, 1, 2, 3, 4, 5] {
                let (rig, parts, authored) = direction_parts(key, Pose { facing, frame }).unwrap();
                assert!(authored);
                assert!(rig.file.ends_with(if facing < 0 {
                    "CLASS-05-0.png"
                } else {
                    "CLASS-05-1-r03.png"
                }));
                let body = &parts[0];
                let top = body
                    .bytes
                    .chunks_exact(4)
                    .position(|p| p[3] >= 128)
                    .unwrap()
                    / 128;
                let (mut head_x, mut head_weight, mut optic_x, mut optic_weight) = (0., 0., 0., 0.);
                for y in top..top + 12 {
                    for x in 0..128 {
                        let p = &body.bytes[(y * 128 + x) * 4..][..4];
                        if p[3] < 128 {
                            continue;
                        }
                        let a = f32::from(p[3]);
                        head_x += x as f32 * a;
                        head_weight += a;
                        if p[0] >= 100
                            && f32::from(p[0]) > f32::from(p[1]) * 1.12
                            && f32::from(p[1]) > f32::from(p[2]) * 1.4
                        {
                            optic_x += x as f32 * a;
                            optic_weight += a;
                        }
                    }
                }
                assert!(optic_weight > 0.);
                assert!(
                    (optic_x / optic_weight - head_x / head_weight) * f32::from(facing) > 0.,
                    "optic faces backwards: {facing}, frame {frame}"
                );
            }
        }
    }

    fn review_sprite(
        image: &Image,
        output: &std::path::Path,
        unique: &mut std::collections::BTreeMap<Vec<u8>, usize>,
        sprites: &mut Vec<serde_json::Value>,
    ) -> Option<[usize; 3]> {
        let (mut x0, mut y0, mut x1, mut y1) = (128usize, 128usize, 0usize, 0usize);
        for y in 0..128 {
            for x in 0..128 {
                if image.bytes[(y * 128 + x) * 4 + 3] > 0 {
                    x0 = x0.min(x);
                    y0 = y0.min(y);
                    x1 = x1.max(x + 1);
                    y1 = y1.max(y + 1);
                }
            }
        }
        if x1 == 0 {
            return None;
        }
        let (w, h) = (x1 - x0, y1 - y0);
        let mut key = vec![w as u8, h as u8];
        for y in y0..y1 {
            key.extend_from_slice(&image.bytes[(y * 128 + x0) * 4..(y * 128 + x1) * 4]);
        }
        let id = if let Some(id) = unique.get(&key) {
            *id
        } else {
            let id = sprites.len();
            let file = format!("sprite-{id:05}.png");
            Image {
                width: w as u16,
                height: h as u16,
                bytes: key[2..].to_vec(),
            }
            .export_png(output.join(&file).to_str().unwrap());
            sprites.push(serde_json::json!({"file":file,"w":w,"h":h}));
            unique.insert(key, id);
            id
        };
        Some([id, x0, y0])
    }
    #[test]
    #[ignore = "Explicit WALK07 equipment proposal export; RL_WALK07_OUTPUT required"]
    fn walk07_equipment_proposal_diagnostic() {
        let output =
            std::path::PathBuf::from(std::env::var("RL_WALK07_OUTPUT").expect("output required"));
        std::fs::create_dir_all(&output).unwrap();
        REVIEW07.set(true);
        let configured: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("assets/source-art/player-walk-review-07/rigs/manifest.json"),
            )
            .unwrap(),
        )
        .unwrap();
        let mut indices: std::collections::BTreeSet<_> = configured
            .as_array()
            .unwrap()
            .iter()
            .map(|e| {
                e["id"]
                    .as_str()
                    .unwrap()
                    .strip_prefix("CLASS-")
                    .unwrap()
                    .parse::<u16>()
                    .unwrap()
            })
            .collect();
        if let Ok(requested) = std::env::var("RL_WALK07_INDICES") {
            let requested: std::collections::BTreeSet<u16> = requested
                .split(',')
                .map(|n| n.parse().expect("WALK07 class index"))
                .collect();
            assert!(requested.is_subset(&indices));
            indices.retain(|n| requested.contains(n));
        }
        let mut configurations = vec![
            ("unarmed", None, None),
            ("pistol", Some(101), None),
            ("hammer", Some(97), None),
            ("rifle", Some(2), None),
            ("axe", Some(66), None),
            ("sword", Some(88), None),
            ("heavy", Some(31), None),
            ("dual", None, Some([Some(101), Some(101)])),
            ("mixed", None, Some([Some(68), Some(101)])),
        ];
        configurations.push(("blades", None, Some([Some(68), Some(82)])));
        let mut configurations: Vec<_> = configurations
            .into_iter()
            .map(|(label, weapon, hands)| (label.to_owned(), weapon, hands))
            .collect();
        for model in 0..HELD_WEAPON_COUNT {
            if ![101, 97, 2, 66, 88, 31].contains(&model) {
                configurations.push((format!("model-{model:03}"), Some(model), None));
            }
        }
        let mut exported = Vec::new();
        let layered = std::env::var_os("RL_WALK07_LAYERED").is_some();
        let recipe = std::env::var_os("RL_WALK07_RECIPE").is_some();
        RECIPE07.set(recipe);
        for index in indices {
            let class_output = output.join(format!("CLASS-{index:02}"));
            let mut unique = std::collections::BTreeMap::new();
            let mut sprites = Vec::new();
            let mut configs = Vec::new();
            if layered {
                std::fs::create_dir_all(&class_output).unwrap();
            }
            for (label, weapon, hands) in configurations.clone() {
                let mut frames = Vec::new();
                let mut sheet = Image::gen_image_color(896, 256, BLANK);
                for (row, facing) in [-1, 1].into_iter().enumerate() {
                    for (column, frame) in [8, 0, 1, 2, 3, 4, 5].into_iter().enumerate() {
                        let key = (
                            BASE + index,
                            selection(weapon, hands),
                            [EquipmentPalette::Original; 3],
                        );
                        let parts = layers(key, Pose { facing, frame })
                            .expect("whole authored pose must support actual equipment");
                        if recipe {
                            let groups = RECIPES07.with_borrow(|groups| groups.clone());
                            let mut encoded = Vec::new();
                            for group in groups {
                                let mut draws = Vec::new();
                                for r in group {
                                    if let Some([id, ox, oy]) = review_sprite(
                                        &r.image,
                                        &class_output,
                                        &mut unique,
                                        &mut sprites,
                                    ) {
                                        let [a, b, c, d] = r.matrix;
                                        let e = r.target[0]
                                            + a * (ox as f32 - r.pivot[0])
                                            + b * (oy as f32 - r.pivot[1]);
                                        let f = r.target[1]
                                            + c * (ox as f32 - r.pivot[0])
                                            + d * (oy as f32 - r.pivot[1]);
                                        draws.push(serde_json::json!([id, a, b, c, d, e, f]));
                                    }
                                }
                                encoded.push(draws);
                            }
                            frames.push(serde_json::json!(encoded));
                            continue;
                        }
                        if layered {
                            frames.push(serde_json::json!(
                                parts
                                    .iter()
                                    .filter_map(|p| {
                                        review_sprite(p, &class_output, &mut unique, &mut sprites)
                                    })
                                    .collect::<Vec<_>>()
                            ));
                            continue;
                        }
                        let mut image = Image::gen_image_color(128, 128, BLANK);
                        for part in parts {
                            geometry::over(&mut image, &part);
                        }
                        for y in 0..128usize {
                            let to = ((row * 128 + y) * 896 + column * 128) * 4;
                            sheet.bytes[to..to + 512]
                                .copy_from_slice(&image.bytes[y * 512..(y + 1) * 512]);
                        }
                    }
                }
                if layered {
                    configs.push(serde_json::json!({"label":label,"weapon":weapon,"hands":hands,"frames":frames}));
                    continue;
                }
                let file = format!("CLASS-{index:02}-{label}.png");
                sheet.export_png(output.join(&file).to_str().unwrap());
                exported.push(serde_json::json!({"id":format!("CLASS-{index:02}"),"configuration":label,"weapon":weapon,"hands":hands,"file":file,"side":128,"foot":[64,96],"frames":14}));
            }
            if layered {
                std::fs::write(class_output.join("layers.json"),serde_json::to_vec(&serde_json::json!({
                    "id":format!("CLASS-{index:02}"),"index":index,"side":128,"foot":[64,96],"sprites":sprites,"configs":configs,
                    "exported_pngs_need_vertical_flip":true,"whole_drawn_gait":true,"runtime_integration":false,"recipe":recipe
                })).unwrap()).unwrap();
                println!(
                    "WALK07 CLASS-{index:02}: {} configurations, {} unique pixel layers",
                    configurations.len(),
                    unique.len()
                );
            }
        }
        REVIEW07.set(false);
        std::fs::write(output.join("capture.json"),serde_json::to_vec_pretty(&serde_json::json!({"entries":exported,"exported_pngs_need_vertical_flip":true,"whole_drawn_gait":true,"runtime_integration":false})).unwrap()).unwrap();
    }
    #[test]
    fn every_weapon_uses_real_rigid_layers_and_every_outfit_has_two_rigs() {
        for index in 0..25 {
            for facing in [-1, 1] {
                assert!(
                    rigs()
                        .iter()
                        .any(|r| r.id == format!("CLASS-{index:02}") && r.facing == facing)
                );
            }
        }
        for model in 0..HELD_WEAPON_COUNT {
            let key = (
                BASE,
                selection(Some(model), None),
                [EquipmentPalette::Original; 3],
            );
            for facing in [-1, 1] {
                let a = layers(key, Pose { facing, frame: 0 }).unwrap();
                let b = layers(key, Pose { facing, frame: 2 }).unwrap();
                assert!(
                    a[1].bytes.chunks_exact(4).any(|p| p[3] > 0),
                    "empty weapon {model}"
                );
                assert_eq!(a[1].bytes, b[1].bytes, "walking bent weapon {model}");
                assert_ne!(
                    a[0].bytes[78 * 512..],
                    b[0].bytes[78 * 512..],
                    "frozen legs {model}"
                );
            }
        }
    }
    #[test]
    #[ignore = "Explicit six-phase native comparison of the actual modular renderer"]
    fn walk02_native_diagnostic() {
        let output = std::path::PathBuf::from(std::env::var("RL_WALK02_OUTPUT").unwrap());
        std::fs::create_dir_all(&output).unwrap();
        for (index, weapon, hands, head) in [
            (0, Some(67), None, 0),
            (4, None, None, 0),
            (4, Some(66), None, 0),
            (5, Some(101), None, 0),
            (10, Some(2), None, 0),
            (15, Some(67), None, 0),
            (20, None, Some([Some(68), Some(101)]), 0),
            (0, Some(67), None, 3),
        ] {
            let key = (
                BASE + index + head * HEAD_STRIDE,
                selection(weapon, hands),
                [EquipmentPalette::Original; 3],
            );
            let mut sheet = Image::gen_image_color(128 * 7, 128 * 3, BLANK);
            for (row, facing) in [-1, 0, 1].into_iter().enumerate() {
                let mut weapon_pixels = None;
                let mut leg_hashes = std::collections::BTreeSet::new();
                for (col, frame) in [0, 1, 2, 3, 4, 5, 8].into_iter().enumerate() {
                    let layers = layers(key, Pose { facing, frame }).unwrap();
                    // Whole weapons, including diagonal shafts, are identical
                    // in all phases: a leg cannot erase or bend any gear pixel.
                    if let Some(bytes) = &weapon_pixels {
                        assert_eq!(bytes, &layers[1].bytes);
                    }
                    weapon_pixels = Some(layers[1].bytes.clone());
                    leg_hashes.insert(layers[0].bytes[78 * 512..].to_vec());
                    let mut composed = Image::gen_image_color(128, 128, BLANK);
                    for layer in layers {
                        geometry::over(&mut composed, &layer);
                    }
                    for y in 0..128usize {
                        let at = ((row * 128 + y) * 896 + col * 128) * 4;
                        sheet.bytes[at..at + 512]
                            .copy_from_slice(&composed.bytes[y * 512..(y + 1) * 512]);
                    }
                }
                assert!(leg_hashes.len() >= 5, "legs did not articulate");
            }
            sheet.export_png(
                output
                    .join(format!(
                        "class-{index:02}-weapon-{}-head-{head}.png",
                        weapon.unwrap_or(999)
                    ))
                    .to_str()
                    .unwrap(),
            );
        }
    }
}
