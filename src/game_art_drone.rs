//! Accepted drone bodies: identical sources for the map and inspection.
use super::{Visual, portrait::Sheet, walking::Pose};
use crate::asset_preparation::{self as preparation, PreparedImage, Request};
use macroquad::prelude::*;
use serde::Deserialize;
use std::{cell::OnceCell, sync::OnceLock};

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Motion {
    Hover,
    Roll,
    Walk,
}

#[derive(Deserialize)]
struct Entry {
    id: String,
    file: String,
    motion: Motion,
    portrait: [f32; 4],
    shadow_y: f32,
    rest_shadow: f32,
    shadows: [[f32; 6]; 4],
}

const SIDE: u16 = 128;

fn entry(visual: Visual) -> Option<&'static Entry> {
    let id = match visual {
        Visual::DroneScout => "D56",
        Visual::Bestiary {
            number: 57,
            corpse: false,
        } => "D57",
        Visual::DroneTracker => "D58",
        Visual::DroneGunner => "D59",
        Visual::DroneGuardian => "D66",
        _ => return None,
    };
    static ENTRIES: OnceLock<Vec<Entry>> = OnceLock::new();
    ENTRIES
        .get_or_init(|| {
            #[derive(Deserialize)]
            struct Manifest {
                native_tile_px: u16,
                cell: u16,
                entries: Vec<Entry>,
            }
            let manifest: Manifest = serde_json::from_str(include_str!(
                "../assets/animations/drone-motion-01/manifest.json"
            ))
            .expect("approved drone motion manifest");
            assert_eq!((manifest.native_tile_px, manifest.cell), (64, SIDE));
            manifest.entries
        })
        .iter()
        .find(|entry| entry.id == id)
}

pub(crate) fn motion(visual: Visual) -> Option<Motion> {
    if super::walking::corrections_enabled() {
        return None;
    }
    Some(entry(visual)?.motion)
}

pub(super) fn request(visual: Visual) -> Option<Request> {
    motion(visual)?;
    let entry = entry(visual)?;
    Some((
        entry.file.clone(),
        Box::new(move || {
            let image = preparation::image(&entry.file)?;
            if (image.width, image.height) != (SIDE * 7, SIDE * 4) {
                return Err(format!("Invalid drone motion atlas: {}", entry.file));
            }
            Ok(PreparedImage {
                image: (*image).clone(),
                cell: [SIDE; 2],
                anchor: [64.; 2],
            })
        }),
    ))
}

pub(super) fn draw_motion(cell: Rect, visual: Visual, opacity: f32, pose: Option<Pose>) -> bool {
    let Some(entry) = entry(visual).filter(|_| !super::walking::corrections_enabled()) else {
        return false;
    };
    let row = pose.and_then(|p| [-1, 1, 2, -2].iter().position(|facing| *facing == p.facing));
    let (row, column, shadow) = match (row, pose) {
        (Some(row), Some(Pose { frame, .. })) if frame < 6 => (
            row,
            usize::from(frame) + 1,
            entry.shadows[row][usize::from(frame)],
        ),
        _ => (0, 0, entry.rest_shadow),
    };
    preparation::with_texture(&entry.file, |atlas| {
        let scale = cell.w / 64.;
        let center = cell.center();
        if !crate::target_outline::capturing_body_mask() {
            draw_ellipse(
                center.x,
                center.y + entry.shadow_y * scale,
                shadow * scale,
                2.5 * scale,
                0.,
                Color::new(0., 0., 0., 0.29 * opacity),
            );
        }
        crate::world_readability::texture(
            &atlas.texture,
            center.x - atlas.anchor[0] * scale,
            center.y - atlas.anchor[1] * scale,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                source: Some(Rect::new(
                    (column * usize::from(SIDE)) as f32,
                    (row * usize::from(SIDE)) as f32,
                    f32::from(SIDE),
                    f32::from(SIDE),
                )),
                dest_size: Some(vec2(cell.w * 2., cell.h * 2.)),
                ..Default::default()
            },
        );
    })
    .is_some()
}

pub(super) fn draw_motion_portrait(rect: Rect, visual: Visual) -> bool {
    let Some(entry) = entry(visual).filter(|_| !super::walking::corrections_enabled()) else {
        return false;
    };
    preparation::with_texture(&entry.file, |atlas| {
        let [x, y, w, h] = entry.portrait;
        let source = Rect::new(x, y, w, h);
        let target = super::portrait::contain(rect, source);
        crate::world_readability::texture(
            &atlas.texture,
            target.x,
            target.y,
            WHITE,
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(target.size()),
                ..Default::default()
            },
        );
    })
    .is_some()
}

thread_local! {
    static SCOUT: OnceCell<Sheet> = const { OnceCell::new() };
    static TRACKER: OnceCell<Sheet> = const { OnceCell::new() };
    static GUNNER: OnceCell<Sheet> = const { OnceCell::new() };
    static GUARDIAN: OnceCell<Sheet> = const { OnceCell::new() };
}

fn with_source(visual: Visual, draw: impl FnOnce(&Sheet)) {
    let (cache, bytes, side): (_, &[u8], u16) = match visual {
        Visual::DroneScout => (
            &SCOUT,
            include_bytes!(
                "../assets/animations/agent/skill-animation-review-97/D56-BODY01-SIZE02-native128.png"
            ),
            128,
        ),
        Visual::DroneTracker => (
            &TRACKER,
            include_bytes!(
                "../assets/animations/agent/skill-animation-review-96/D58-BODY01-native128.png"
            ),
            128,
        ),
        Visual::DroneGunner => (
            &GUNNER,
            include_bytes!(
                "../assets/animations/agent/skill-animation-review-92/artilleur-body03/D59-BODY03-native64.png"
            ),
            64,
        ),
        Visual::DroneGuardian => (
            &GUARDIAN,
            include_bytes!(
                "../assets/animations/agent/skill-animation-review-94/gardien-size01/D66-BODY02-SIZE01-native128.png"
            ),
            128,
        ),
        _ => return,
    };
    cache.with(|cache| draw(cache.get_or_init(|| Sheet::load(bytes, side))));
}

pub(super) fn draw_portrait(rect: Rect, visual: Visual) {
    with_source(visual, |source| source.draw(rect, 0, 1.));
}

fn draw_world(rect: Rect, visual: Visual, opacity: f32) {
    with_source(visual, |source| {
        if visual == Visual::DroneScout && !crate::target_outline::capturing_body_mask() {
            draw_ellipse(
                rect.center().x,
                rect.center().y + rect.h * 30. / 64.,
                rect.w * 22. / 64.,
                rect.h * 3. / 64.,
                0.,
                Color::new(0., 0., 0., opacity * 0.3),
            );
        }
        let destination = if visual == Visual::DroneGunner {
            rect
        } else {
            Rect::new(
                rect.center().x - rect.w,
                rect.center().y - rect.h,
                rect.w * 2.,
                rect.h * 2.,
            )
        };
        crate::world_readability::texture(
            &source.texture,
            destination.x,
            destination.y,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                dest_size: Some(destination.size()),
                ..Default::default()
            },
        );
    });
}
pub(super) fn draw_scout(rect: Rect, opacity: f32) {
    draw_world(rect, Visual::DroneScout, opacity);
}
pub(super) fn draw_tracker(rect: Rect, opacity: f32) {
    draw_world(rect, Visual::DroneTracker, opacity);
}
pub(super) fn draw_gunner(rect: Rect, opacity: f32) {
    draw_world(rect, Visual::DroneGunner, opacity);
}
pub(super) fn draw_guardian(rect: Rect, opacity: f32) {
    draw_world(rect, Visual::DroneGuardian, opacity);
}
