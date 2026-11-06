//! Accepted BIPED-LATERAL01..04 and the street rifle cultists. No I/O in draw.
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
    walk_shadow: u8,
    front_shadow: u8,
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
            "../assets/animations/biped-lateral-01/manifest.json"
        ))
        .expect("accepted biped manifest");
        assert_eq!((manifest.format_version, manifest.native_tile_px), (1, 64));
        manifest
            .entries
            .into_iter()
            .map(|e| {
                assert_eq!(e.variant, 0);
                assert!(
                    e.file.starts_with("assets/animations/biped-lateral-01/")
                        && !e.file.contains("..")
                );
                assert!([128, 160].contains(&e.cell[0]) && e.cell[1] == 128);
                (e.number, e)
            })
            .collect()
    })
}
const RIFLE_WALK_MODEL: u16 = 164;

fn entry(visual: Visual) -> Option<(&'static Entry, Option<(u16, u8)>)> {
    match visual {
        Visual::CampaignMonster {
            number: number @ (22 | 24),
            variant: variant @ (0 | 1),
        } => Some((entries().get(&RIFLE_WALK_MODEL)?, Some((number, variant)))),
        Visual::CampaignMonster { number, variant: 0 } => Some((entries().get(&number)?, None)),
        _ => None,
    }
}
pub(crate) fn supports(visual: Option<Visual>) -> bool {
    visual.and_then(entry).is_some()
}
fn key(e: &Entry, street_palette: Option<(u16, u8)>) -> String {
    if let Some((number, 1)) = street_palette {
        return format!("biped-lateral01/{number}/1");
    }
    format!("biped-lateral01/{}", e.number)
}
fn request(visual: Visual) -> Option<Request> {
    let (e, street_palette) = entry(visual)?;
    Some((
        key(e, street_palette),
        Box::new(move || {
            let image = preparation::image(&e.file)?;
            if (image.width, image.height) != (e.cell[0] * 9, e.cell[1] * 2) {
                return Err(format!("Invalid biped atlas: {}", e.file));
            }
            let mut image = (*image).clone();
            if let Some((number, 1)) = street_palette {
                super::walking::apply_creature_palette(number, 1, &mut image)
                    .ok_or("Missing green street rifle cultist palette")?;
            }
            Ok(PreparedImage {
                image,
                cell: e.cell,
                anchor: e.anchor,
            })
        }),
    ))
}
pub(crate) fn prepare(visuals: &[Visual]) -> Result<std::time::Duration, String> {
    preparation::prepare(|| visuals.iter().filter_map(|v| request(*v)).collect())
}
pub(crate) fn anticipate(visuals: &[Visual]) {
    for visual in visuals.iter().take(4) {
        if let Some((key, build)) = request(*visual) {
            preparation::request(key, build);
        }
    }
}
fn selection(
    visual: Visual,
    pose: Option<Pose>,
) -> Option<(&'static Entry, Option<(u16, u8)>, u8, u16)> {
    let (e, street_palette) = entry(visual)?;
    let (phase, row) = match pose {
        None | Some(Pose { frame: 8, .. }) if street_palette.is_none() => (8, 0),
        Some(Pose { facing, frame }) if [-1, 1].contains(&facing) && frame < 8 => {
            (frame, u16::from(facing > 0))
        }
        _ => return None,
    };
    Some((e, street_palette, phase, row))
}
pub(crate) fn draw(cell: Rect, visual: Visual, opacity: f32, pose: Option<Pose>) -> bool {
    let Some((e, street_palette, phase, row)) = selection(visual, pose) else {
        return false;
    };
    preparation::with_texture(&key(e, street_palette), |p| {
        let scale = cell.w / 64.;
        let mut at = cell;
        at.y += f32::from(e.vertical_offset) * scale;
        super::draw_contact_shadow(
            at,
            [
                32,
                63,
                if phase == 8 {
                    e.front_shadow
                } else {
                    e.walk_shadow
                },
            ],
            64,
            opacity,
        );
        crate::world_readability::texture(
            &p.texture,
            at.center().x - p.anchor[0] * scale,
            at.bottom() - p.anchor[1] * scale,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                source: Some(Rect::new(
                    f32::from(phase) * f32::from(p.cell[0]),
                    f32::from(row * p.cell[1]),
                    f32::from(p.cell[0]),
                    f32::from(p.cell[1]),
                )),
                dest_size: Some(vec2(
                    f32::from(p.cell[0]) * scale,
                    f32::from(p.cell[1]) * scale,
                )),
                ..Default::default()
            },
        );
    })
    .is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepted_biped_scope_and_headings_do_not_expand_to_other_variants_or_vertical_walks() {
        assert_eq!(
            entries().keys().copied().collect::<Vec<_>>(),
            [5, 14, 39, 144, 145, 146, 151, 156, 164, 165]
        );
        for n in entries().keys() {
            let visual = Visual::CampaignMonster {
                number: *n,
                variant: 0,
            };
            assert!(supports(Some(visual)) && selection(visual, None).unwrap().2 == 8);
            assert!(!supports(Some(Visual::CampaignMonster {
                number: *n,
                variant: 1
            })));
            for facing in [-2, 2] {
                assert!(selection(visual, Some(Pose { facing, frame: 1 })).is_none());
            }
            for facing in [-1, 1] {
                for frame in 0..8 {
                    assert_eq!(
                        selection(visual, Some(Pose { facing, frame })).unwrap().2,
                        frame
                    );
                }
            }
        }
        assert!(!supports(Some(Visual::CampaignMonster {
            number: 25,
            variant: 0
        })));
    }

    #[test]
    fn street_rifles_share_the_approved_stride_and_keep_their_own_rest_and_vertical_art() {
        for (number, variant) in [(22, 0), (22, 1), (24, 0), (24, 1)] {
            let visual = Visual::CampaignMonster { number, variant };
            assert!(supports(Some(visual)));
            assert!(selection(visual, None).is_none());
            for facing in [-2, 2] {
                assert!(selection(visual, Some(Pose { facing, frame: 1 })).is_none());
            }
            for facing in [-1, 1] {
                for frame in 0..8 {
                    let (atlas, palette, phase, row) =
                        selection(visual, Some(Pose { facing, frame })).unwrap();
                    assert_eq!(atlas.number, RIFLE_WALK_MODEL);
                    assert_eq!(palette, Some((number, variant)));
                    assert_eq!((phase, row), (frame, u16::from(facing > 0)));
                }
            }
        }
    }
}
