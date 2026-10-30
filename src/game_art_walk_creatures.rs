//! Exact WALK07 creature poses authorized on 24 October 2026.
use super::*;
use project_rl::asset_cache::ByteCache;
use serde::Deserialize;
use std::{cell::RefCell, collections::BTreeMap, sync::OnceLock};
#[path = "game_art_walk_palette.rs"]
mod palette;

#[cfg(all(debug_assertions, test))]
pub(super) fn vertical_palette(number: u16, variant: u8, image: &mut Image) -> Option<()> {
    palette::apply(number, variant, image)
}

#[derive(Deserialize)]
struct Creature {
    id: String,
    number: u16,
    file: String,
    side: u16,
    foot: [u16; 2],
    vertical_offset: i16,
    shadow: bool,
    cycle_bounds: [u16; 4],
    front_vertical_offset: i16,
    front_bounds: [u16; 4],
    front_foot: [u16; 2],
}

fn entries() -> &'static BTreeMap<u16, Creature> {
    static ENTRIES: OnceLock<BTreeMap<u16, Creature>> = OnceLock::new();
    ENTRIES.get_or_init(|| {
        #[derive(Deserialize)]
        struct Manifest {
            format_version: u8,
            native_tile: u16,
            columns: u8,
            rows: u8,
            entries: Vec<Creature>,
        }
        let manifest: Manifest =
            serde_json::from_str(include_str!("../assets/animations/walk-07/manifest.json"))
                .expect("installed WALK07 manifest");
        assert_eq!(
            (
                manifest.format_version,
                manifest.native_tile,
                manifest.columns,
                manifest.rows
            ),
            (1, 64, 7, 2)
        );
        manifest
            .entries
            .into_iter()
            .map(|e| {
                assert_eq!(
                    e.id,
                    if e.number == 0 {
                        "DRONE-GUNNER".to_owned()
                    } else {
                        format!("MON-{:03}-0", e.number)
                    }
                );
                assert_eq!(e.side, 128);
                assert!(e.file.starts_with("assets/animations/walk-07/creatures/"));
                assert!(!e.file.contains(".."));
                let b = e.cycle_bounds;
                assert!(b[0] < b[2] && b[1] < b[3] && b[2] < 128 && b[3] < 128);
                let f = e.front_bounds;
                let expected = if f[3] - f[1] < 64 {
                    (f32::from(e.front_foot[1]) - 32. - f32::from(f[1] + f[3]) / 2. + 0.5).floor()
                        as i16
                } else {
                    0
                };
                assert_eq!(e.front_vertical_offset, expected);
                (e.number, e)
            })
            .collect()
    })
}

fn key(visual: Visual, original_palette_only: bool) -> Option<u16> {
    match visual {
        Visual::CampaignMonster { number, variant }
            if !original_palette_only
                || variant == 0
                || ((super::proportions_enabled() || super::corrections_enabled())
                    && project_rl::monster_art::art(number, variant).is_some()) =>
        {
            Some(number)
        }
        Visual::DroneGunner => Some(0),
        _ => None,
    }
}

fn selected_entry(number: u16) -> Option<&'static Creature> {
    if super::proportions_enabled() {
        static PROPOSED: OnceLock<BTreeMap<u16, Creature>> = OnceLock::new();
        let entries = PROPOSED.get_or_init(|| {
            let rows: Vec<Creature> = serde_json::from_str(include_str!(
                "../assets/animations/walk-11/creatures/manifest.json"
            ))
            .expect("WALK11 accepted creature proportions");
            rows.into_iter().map(|e| (e.number, e)).collect()
        });
        if let Some(entry) = entries.get(&number) {
            return Some(entry);
        }
    }
    entries().get(&number)
}

fn selection(visual: Visual, pose: Pose) -> Option<(&'static Creature, Rect)> {
    if ![-1, 1].contains(&pose.facing) || ![0, 1, 2, 3, 4, 5, 8].contains(&pose.frame) {
        return None;
    }
    // Approved color variants reuse the whole poses with their canonical color
    // differences, without changing alpha, geometry or movement timing.
    let entry = selected_entry(key(visual, true)?)?;
    let column = if pose.frame == 8 { 0 } else { pose.frame + 1 };
    let row = u16::from(pose.facing > 0);
    let side = f32::from(entry.side);
    Some((
        entry,
        Rect::new(f32::from(column) * side, f32::from(row) * side, side, side),
    ))
}

fn destination(cell: Rect, entry: &Creature) -> Rect {
    let scale = cell.w / 64.;
    Rect::new(
        cell.center().x - f32::from(entry.foot[0]) * scale,
        cell.bottom() + f32::from(entry.vertical_offset - entry.foot[1] as i16) * scale,
        f32::from(entry.side) * scale,
        f32::from(entry.side) * scale,
    )
}

pub(super) fn static_cell(mut cell: Rect, visual: Visual) -> Rect {
    if let Some(entry) = key(visual, false).and_then(selected_entry) {
        cell.y += f32::from(entry.front_vertical_offset) * cell.w / 64.;
    }
    cell
}

thread_local! {
    static CACHE: RefCell<ByteCache<(u16,u8,bool), Option<Texture2D>>> =
        const { RefCell::new(ByteCache::new(64 * 1024 * 1024)) };
}

pub(super) fn image(number: u16, variant: u8) -> Option<Image> {
    let entry = selected_entry(number)?;
    let mut image = Image::from_file_with_format(
        &project_rl::data_paths::read(&entry.file).ok()?,
        Some(ImageFormat::Png),
    )
    .ok()?;
    if (image.width, image.height) != (896, 256) {
        return None;
    }
    palette::apply(number, variant, &mut image)?;
    Some(image)
}

pub(super) fn draw(cell: Rect, visual: Visual, opacity: f32, pose: Option<Pose>) -> bool {
    let Some((entry, source)) = pose.and_then(|pose| selection(visual, pose)) else {
        return false;
    };
    let variant = if let Visual::CampaignMonster { variant, .. } = visual {
        variant
    } else {
        0
    };
    let key = (entry.number, variant, super::proportions_enabled());
    CACHE.with_borrow_mut(|cache| {
        if cache.get(&key).is_none() {
            let image = image(entry.number, variant);
            let bytes = image.as_ref().map_or(0, |image| image.bytes.len());
            let texture = image.map(|image| {
                let texture = Texture2D::from_image(&image);
                texture.set_filter(FilterMode::Nearest);
                texture
            });
            if texture.is_none() {
                eprintln!("[WALK07] Animation indisponible : {}", entry.file);
            }
            cache.insert(key, texture, bytes);
        }
        let Some(texture) = cache.get(&key).and_then(Option::as_ref) else {
            return false;
        };
        let target = destination(cell, entry);
        if entry.shadow {
            let mut shadow_cell = cell;
            shadow_cell.y += f32::from(entry.vertical_offset) * cell.w / 64.;
            super::super::draw_contact_shadow(shadow_cell, [32, 62, 17], 64, opacity);
        }
        crate::world_readability::texture(
            texture,
            target.x,
            target.y,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                source: Some(source),
                dest_size: Some(target.size()),
                ..Default::default()
            },
        );
        true
    })
}

#[cfg(debug_assertions)]
pub(super) fn diagnostic_selection(visual: Visual, pose: Pose) -> Option<(&'static str, Rect)> {
    selection(visual, pose).map(|(entry, source)| (entry.file.as_str(), source))
}

/// Read-only audit of the actual static renderer and normal walk bindings.
#[cfg(debug_assertions)]
pub(super) fn diagnostic_audit() -> Vec<serde_json::Value> {
    let pairs: std::collections::BTreeSet<_> = project_rl::monster_art::MONSTER_ART
        .iter()
        .map(|a| (a.number, a.variant))
        .chain(project_rl::monster_palettes::all_palettes().map(|p| (p.number, p.variant)))
        .collect();
    pairs.into_iter().filter_map(|(number, variant)| {
        let art = project_rl::monster_art::art(number, variant)?;
        let image = super::super::campaign::reference_image(number, variant)?;
        let mut bounds = [image.width, image.height, 0, 0];
        for y in 0..image.height {
            for x in 0..image.width {
                if image.bytes[(usize::from(y) * usize::from(image.width) + usize::from(x)) * 4 + 3] >= 96 {
                    bounds[0] = bounds[0].min(x); bounds[1] = bounds[1].min(y);
                    bounds[2] = bounds[2].max(x+1); bounds[3] = bounds[3].max(y+1);
                }
            }
        }
        let entry = selected_entry(number)?;
        let scale = f32::from(art.side) / f32::from(image.height);
        let static_height = f32::from(bounds[3] - bounds[1]) * scale;
        let front_center_y = 64. - f32::from(art.foot[1]) + f32::from(entry.front_vertical_offset)
            + f32::from(bounds[1] + bounds[3]) * scale / 2.;
        Some(serde_json::json!({
            "number":number,"variant":variant,"name":art.name,"role":format!("{:?}",art.role),
            "static_image_size":[image.width,image.height],"static_destination_side":art.side,
            "static_alpha_bounds":bounds,"static_height_native":static_height,
            "static_center_y_in_cell":front_center_y,
            "cycle_height_native":entry.cycle_bounds[3]-entry.cycle_bounds[1],
            "normal_lateral_animation":selection(Visual::CampaignMonster{number,variant},Pose{facing:1,frame:1}).is_some(),
            "approved_base_cycle":entry.file
        }))
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepted_proportions_preserve_all_471_palette_geometries_and_scope_comparison() {
        assert!(super::super::proportions_enabled());
        let original = selected_entry(22).unwrap().file.clone();
        {
            let _comparison = super::super::review_proportions(false);
            assert!(selected_entry(22).unwrap().file.contains("walk-07"));
        }
        assert!(super::super::proportions_enabled());
        assert_eq!(selected_entry(22).unwrap().file, original);
        assert!(original.contains("walk-11"));
        assert_eq!(
            selected_entry(22).unwrap().cycle_bounds[3]
                - selected_entry(22).unwrap().cycle_bounds[1],
            59
        );
        complete_review_catalogue_preserves_every_variant_alpha_and_every_base_pose();
    }
    #[test]
    fn complete_review_catalogue_preserves_every_variant_alpha_and_every_base_pose() {
        assert!(!super::super::corrections_enabled());
        assert!(super::super::proportions_enabled());
        let pairs: std::collections::BTreeSet<_> = project_rl::monster_art::MONSTER_ART
            .iter()
            .map(|a| (a.number, a.variant))
            .chain(project_rl::monster_palettes::all_palettes().map(|p| (p.number, p.variant)))
            .collect();
        assert_eq!(pairs.len(), 471);
        let mut bases = BTreeMap::new();
        let mut colored = 0;
        for (number, variant) in pairs {
            let base = bases
                .entry(number)
                .or_insert_with(|| image(number, 0).unwrap());
            let candidate = image(number, variant).unwrap();
            assert_eq!(base.bytes.len(), candidate.bytes.len());
            assert!(
                base.bytes
                    .chunks_exact(4)
                    .zip(candidate.bytes.chunks_exact(4))
                    .all(|(a, b)| a[3] == b[3])
            );
            if variant == 0 {
                assert_eq!(candidate.bytes, base.bytes);
            } else if candidate.bytes != base.bytes {
                colored += 1;
            }
            for facing in [-1, 1] {
                for frame in [8, 0, 1, 2, 3, 4, 5] {
                    assert!(
                        selection(
                            Visual::CampaignMonster { number, variant },
                            Pose { facing, frame }
                        )
                        .is_some()
                    );
                }
            }
        }
        assert_eq!(bases.len(), 129);
        assert!(colored >= 300, "Only {colored} actual color changes");
    }

    #[test]
    fn installed_bodies_match_the_authorized_native_sources() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let decision: serde_json::Value = serde_json::from_slice(
            &std::fs::read(root.join("docs/reviews/player-walk-07/integration/authorization.json"))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(decision["runtime_integration_authorized"], true);
        assert_eq!(entries().len(), 130);
        for entry in entries().values() {
            assert_eq!(entry.side, 128);
            assert!(
                entry
                    .file
                    .starts_with("assets/animations/walk-07/creatures/")
            );
            let image = Image::from_file_with_format(
                &std::fs::read(root.join(&entry.file)).unwrap(),
                Some(ImageFormat::Png),
            )
            .unwrap();
            let source = decision["corrected_pixels_and_scope"]
                .as_array()
                .unwrap()
                .iter()
                .find(|p| p["id"] == entry.id)
                .unwrap();
            let reference = Image::from_file_with_format(
                &std::fs::read(root.join(source["native_source"].as_str().unwrap())).unwrap(),
                Some(ImageFormat::Png),
            )
            .unwrap();
            assert_eq!((image.width, image.height), (896, 256));
            assert_eq!(image.bytes, reference.bytes, "{}", entry.id);
        }
    }

    #[test]
    fn lateral_directions_and_real_idle_use_separate_authored_rows() {
        for &number in entries().keys() {
            let visual = if number == 0 {
                Visual::DroneGunner
            } else {
                Visual::CampaignMonster { number, variant: 0 }
            };
            for facing in [-1, 1] {
                for frame in [0, 1, 2, 3, 4, 5, 8] {
                    let (_, rect) = selection(visual, Pose { facing, frame }).unwrap();
                    assert_eq!(rect.y, if facing < 0 { 0. } else { 128. });
                    assert_eq!(
                        rect.x,
                        if frame == 8 {
                            0.
                        } else {
                            f32::from(frame + 1) * 128.
                        }
                    );
                }
            }
            for facing in [0, -2, 2] {
                assert!(selection(visual, Pose { facing, frame: 1 }).is_none());
            }
            for frame in [6, 7, 9, 255] {
                assert!(selection(visual, Pose { facing: 1, frame }).is_none());
            }
        }
        for visual in [
            Visual::Actor(1),
            Visual::Bestiary {
                number: 1,
                corpse: true,
            },
            Visual::DroneScout,
        ] {
            assert!(
                selection(
                    visual,
                    Pose {
                        facing: 1,
                        frame: 2
                    }
                )
                .is_none()
            );
        }
    }

    #[test]
    fn small_monsters_and_their_front_sprite_stay_centered_at_every_cell_scale() {
        for entry in entries().values() {
            let b = entry.cycle_bounds;
            for scale in [1., 1.5, 2., 3.] {
                let cell = Rect::new(15., 23., 64. * scale, 64. * scale);
                let dest = destination(cell, entry);
                if b[3] - b[1] < 64 {
                    let center = dest.y + f32::from(b[1] + b[3]) * scale / 2.;
                    assert!(
                        (center - cell.center().y).abs() <= 0.501 * scale,
                        "{}",
                        entry.id
                    );
                } else {
                    assert_eq!(entry.vertical_offset, 0);
                }
                let f = entry.front_bounds;
                if f[3] - f[1] < 64 {
                    let center = cell.bottom()
                        + f32::from(entry.front_vertical_offset - entry.front_foot[1] as i16)
                            * scale
                        + f32::from(f[1] + f[3]) * scale / 2.;
                    assert!(
                        (center - cell.center().y).abs() <= 0.501 * scale,
                        "{} front",
                        entry.id
                    );
                } else {
                    assert_eq!(entry.front_vertical_offset, 0);
                }
            }
        }
    }

    #[test]
    fn reported_repairs_preserve_m100_right_and_only_swap_m104_complete_halves() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        for number in [99, 106] {
            let entry = &entries()[&number];
            let current = Image::from_file_with_format(
                &std::fs::read(root.join(&entry.file)).unwrap(),
                Some(ImageFormat::Png),
            )
            .unwrap();
            let before = Image::from_file_with_format(&std::fs::read(root.join(format!("assets/source-art/player-walk-review-07/integration-corrections-r07/MON-{number:03}-0-before-native.png"))).unwrap(), Some(ImageFormat::Png)).unwrap();
            let half = 896 * 128 * 4;
            if number == 99 {
                assert_eq!(current.bytes[half..], before.bytes[half..]);
                assert_ne!(current.bytes[..half], before.bytes[..half]);
            } else {
                assert_eq!(current.bytes[..half], before.bytes[half..]);
                assert_eq!(current.bytes[half..], before.bytes[..half]);
            }
        }
    }
}
