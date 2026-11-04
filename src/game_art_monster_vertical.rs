//! Approved eight-pose vertical cycles for the base monster models.
use super::{Visual, walking::Pose};
use crate::asset_preparation::{self as preparation, PreparedImage, Request};
use macroquad::prelude::*;
use serde::Deserialize;
use std::{collections::BTreeMap, sync::OnceLock};

#[derive(Deserialize)]
struct Entry {
    number: u16,
    variant: u8,
    file: String,
    cell: [u16; 2],
    anchor: [f32; 2],
    vertical_offset: i16,
    casts_shadow: bool,
}

fn entries() -> &'static BTreeMap<u16, Entry> {
    static ENTRIES: OnceLock<BTreeMap<u16, Entry>> = OnceLock::new();
    ENTRIES.get_or_init(|| {
        #[derive(Deserialize)]
        struct Manifest {
            format_version: u8,
            native_tile_px: u16,
            entries: Vec<Entry>,
        }
        let manifest: Manifest = serde_json::from_str(include_str!(
            "../assets/animations/monster-vertical-01/manifest.json"
        ))
        .expect("approved monster vertical manifest");
        assert_eq!((manifest.format_version, manifest.native_tile_px), (1, 64));
        let entries: BTreeMap<_, _> = manifest
            .entries
            .into_iter()
            .map(|entry| {
                assert_eq!(entry.variant, 0);
                assert_eq!(
                    entry.file,
                    format!(
                        "assets/animations/monster-vertical-01/MON-{:03}-0.png",
                        entry.number
                    )
                );
                assert!((1..=160).contains(&entry.cell[0]) && (1..=128).contains(&entry.cell[1]));
                (entry.number, entry)
            })
            .collect();
        assert_eq!(entries.len(), 129);
        entries
    })
}

fn entry(visual: Visual) -> Option<&'static Entry> {
    if let Visual::CampaignMonster { number, variant: 0 } = visual {
        entries().get(&number)
    } else {
        None
    }
}

pub(super) fn supports(visual: Option<Visual>) -> bool {
    visual.and_then(entry).is_some()
}

pub(super) fn request(visual: Visual) -> Option<Request> {
    let entry = entry(visual)?;
    Some((
        entry.file.clone(),
        Box::new(move || {
            let image = preparation::image(&entry.file)?;
            if (image.width, image.height) != (entry.cell[0] * 8, entry.cell[1] * 2) {
                return Err(format!("Invalid monster walk atlas: {}", entry.file));
            }
            Ok(PreparedImage {
                image: (*image).clone(),
                cell: entry.cell,
                anchor: entry.anchor,
            })
        }),
    ))
}

pub(super) fn draw(cell: Rect, visual: Visual, opacity: f32, pose: Option<Pose>) -> bool {
    let Some(Pose { facing, frame }) = pose else {
        return false;
    };
    if !matches!(facing, -2 | 2) || frame >= 8 {
        return false;
    }
    let Some(entry) = entry(visual) else {
        return false;
    };
    preparation::with_texture(&entry.file, |prepared| {
        let scale = cell.w / 64.;
        let mut at = cell;
        at.y += f32::from(entry.vertical_offset) * scale;
        if entry.casts_shadow {
            let shadow = project_rl::monster_art::art(entry.number, 0)
                .expect("approved monster identity")
                .shadow
                .min(255) as u8;
            super::draw_contact_shadow(at, [32, 63, shadow], 64, opacity);
        }
        let [width, height] = prepared.cell.map(f32::from);
        crate::world_readability::texture(
            &prepared.texture,
            at.center().x - prepared.anchor[0] * scale,
            at.bottom() - prepared.anchor[1] * scale,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                source: Some(Rect::new(
                    f32::from(frame) * width,
                    if facing < 0 { height } else { 0. },
                    width,
                    height,
                )),
                dest_size: Some(vec2(width * scale, height * scale)),
                ..Default::default()
            },
        );
    })
    .is_some()
}
