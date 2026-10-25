//! Accepted ART-FINISH-01 through 06 resources shared by board and inspection.
//! Embedded assets only; neither captures nor authoring files are runtime inputs.
use super::*;
use project_rl::asset_cache::ByteCache;
use std::cell::RefCell;
include!("game_art_finish_catalog.rs");
include!("game_art_finish_catalog_02.rs");
include!("game_art_finish_catalog_03.rs");
include!("game_art_finish_catalog_04.rs");
include!("game_art_finish_catalog_05.rs");
include!("game_art_finish_catalog_06.rs");
include!("game_art_finish_icons_catalog_06.rs");
include!("game_art_torso_catalog_01.rs");
include!("game_art_readability_catalog.rs");

thread_local! {
    static IMAGES: RefCell<ByteCache<String, Image>> = const { RefCell::new(ByteCache::new(16 * 1024 * 1024)) };
    static DRONE: portrait::Sheet = portrait::Sheet::from_image(&image("FIN-11").unwrap(), 128);
}
pub(super) fn image(name: &str) -> Option<Image> {
    let bytes = readability_bytes(name)
        .or_else(|| bytes(name))
        .or_else(|| bytes_02(name))
        .or_else(|| bytes_03(name))
        .or_else(|| bytes_04(name))
        .or_else(|| bytes_05(name))
        .or_else(|| bytes_06(name))
        .or_else(|| bytes_outfit_icons_06(name))
        .or_else(|| bytes_torso_icons_01(name))?;
    Some(IMAGES.with(|images| {
        let mut images = images.borrow_mut();
        let key = name.to_owned();
        if let Some(image) = images.get(&key) {
            return image.clone();
        }
        let image = palettes::decoded(bytes);
        let size = image.bytes.len();
        images.insert(key.clone(), image, size);
        images.get(&key).unwrap().clone()
    }))
}
pub(super) fn campaign_image(number: u16, variant: u8) -> Option<Image> {
    if let Some(image) = image(&format!("VIS-{number:03}-{variant}")) {
        return Some(image);
    }
    image(match (number, variant) {
        (164, 0) => "FIN-06",
        (165, 0) => "FIN-07",
        (5, 0) => "FIN-08",
        (144, 0) => "FIN-09",
        (144, 1) => "FIN-09-blue",
        (144, 2) => "FIN-09-orange",
        (144, 3) => "FIN-09-green",
        (145, 0) => "FIN-10",
        (145, 2) => "FIN-10-orange",
        (145, 3) => "FIN-10-green",
        (14, 0) => "FIN-28",
        (16, 0) => "FIN-29",
        (6, 0) => "FIN-30",
        (120, 0) => "FIN-31",
        _ => return image(&format!("MON-{number:03}-{variant}")),
    })
}
pub(super) fn has_torso_icon(illustration: u16) -> bool {
    matches!(illustration, 101..=114 | 200..=204)
}
pub(super) fn item_image(illustration: u16, palette: EquipmentPalette) -> Option<Image> {
    if has_torso_icon(illustration) {
        let variant = match palette {
            EquipmentPalette::Original => Some('A'),
            EquipmentPalette::SlateBlue => Some('B'),
            EquipmentPalette::MossGreen => Some('C'),
            EquipmentPalette::Rust => Some('D'),
            EquipmentPalette::DeepRed if matches!(illustration, 109 | 203) => Some('A'),
            EquipmentPalette::DeepRed => Some('E'),
            _ => None,
        };
        if let Some(variant) = variant {
            return image(&format!("TOR-{illustration}-{variant}"));
        }
        let mut icon = image(&format!("TOR-{illustration}-A"))?;
        let selected = match palette {
            EquipmentPalette::ReviewedVariant1 if illustration == 103 => EquipmentPalette::Original,
            EquipmentPalette::ReviewedVariant1 => EquipmentPalette::Blue,
            EquipmentPalette::ReviewedVariant2 => EquipmentPalette::Red,
            _ => palette,
        };
        palettes::recolor_finished(
            &mut icon,
            &image(&format!("TOR-{illustration}-material"))?,
            selected,
        );
        return Some(icon);
    }
    if finished_outfit_icon(illustration) {
        let mut icon = image(&format!("ICON-FIN-{illustration}"))?;
        let selected = match palette {
            EquipmentPalette::ReviewedVariant1 if illustration == 103 => EquipmentPalette::Original,
            EquipmentPalette::ReviewedVariant1 => EquipmentPalette::Blue,
            EquipmentPalette::ReviewedVariant2 => EquipmentPalette::Red,
            _ => palette,
        };
        palettes::recolor_finished(
            &mut icon,
            &image(&format!("ICON-FIN-{illustration}-mask"))?,
            selected,
        );
        return Some(icon);
    }
    // The pilot approved original materials, not new equipment recolorings.
    if palette != EquipmentPalette::Original {
        return None;
    }
    image(match illustration {
        2 => "FIN-12",
        150 => "FIN-13",
        _ => return None,
    })
}
pub(super) fn draw_drone(cell: Rect, opacity: f32, world: bool) {
    DRONE.with(|sheet| {
        if !world {
            sheet.draw(cell, 0, opacity);
            return;
        }
        super::draw_contact_shadow(cell, [16, 31, 23], 32, opacity);
        let rect = Rect::new(
            cell.x - cell.w / 2.,
            cell.y - cell.h / 2.,
            cell.w * 2.,
            cell.h * 2.,
        );
        crate::world_readability::texture(
            &sheet.texture,
            rect.x,
            rect.y,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                dest_size: Some(rect.size()),
                ..Default::default()
            },
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepted_torso_icons_match_every_approved_native_file() {
        let manifest: serde_json::Value = serde_json::from_str(include_str!(
            "../docs/reviews/refonte-histoire-2026-10-16/torso-icons-review-01/candidates.json"
        ))
        .unwrap();
        let supplement: serde_json::Value = serde_json::from_str(include_str!(
            "../docs/reviews/refonte-histoire-2026-10-16/torso-starting-review-02/candidates.json"
        ))
        .unwrap();
        let entries = manifest["entries"]
            .as_array()
            .unwrap()
            .iter()
            .chain(supplement["entries"].as_array().unwrap().iter());
        let mut checked = 0;
        for entry in entries {
            let illustration = entry["illustration"].as_u64().unwrap() as u16;
            for variant in entry["variants"].as_array().unwrap() {
                let code = variant["code"].as_str().unwrap();
                let palette = match code.as_bytes().last().unwrap() {
                    b'A' => EquipmentPalette::Original,
                    b'B' => EquipmentPalette::SlateBlue,
                    b'C' => EquipmentPalette::MossGreen,
                    b'D' => EquipmentPalette::Rust,
                    b'E' => EquipmentPalette::DeepRed,
                    _ => unreachable!(),
                };
                let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join(variant["icon"].as_str().unwrap());
                let expected = palettes::decoded(&std::fs::read(file).unwrap());
                let actual = item_image(illustration, palette).unwrap();
                assert_eq!((actual.width, actual.height), (64, 64));
                assert!(actual.bytes == expected.bytes, "accepted icon {code}");
                checked += 1;
            }
        }
        assert_eq!(checked, 93);
        for illustration in [109, 203] {
            assert!(
                item_image(illustration, EquipmentPalette::DeepRed)
                    .unwrap()
                    .bytes
                    == item_image(illustration, EquipmentPalette::Original)
                        .unwrap()
                        .bytes
            );
        }
    }
    #[test]
    fn approved_exports_have_native_canvases_and_stable_original_bindings() {
        for number in 1..=31 {
            let image = image(&format!("FIN-{number:02}")).unwrap();
            let side = if (12..=13).contains(&number) { 64 } else { 128 };
            assert_eq!((image.width, image.height), (side, side));
            assert!(image.bytes.chunks_exact(4).any(|p| p[3] != 0));
        }
        assert!(bytes("FIN-10-blue").is_none());
        assert!(campaign_image(145, 4).is_none());
        assert!(campaign_image(164, 1).is_none());
        assert!(item_image(150, EquipmentPalette::Blue).is_none());
        for (monster, drawing) in [(14, 28), (16, 29), (6, 30), (120, 31)] {
            assert_eq!(
                campaign_image(monster, 0).unwrap().bytes,
                image(&format!("FIN-{drawing}")).unwrap().bytes
            );
            assert_eq!(
                campaign_image(monster, 1).is_some(),
                project_rl::monster_art::art(monster, 1).is_some()
            );
        }
    }
    #[test]
    fn completed_bestiary_covers_every_existing_appearance_without_changing_geometry() {
        let mut actual = 0;
        for number in 0..=165 {
            for variant in 0..=7 {
                let expected = project_rl::monster_art::art(number, variant);
                let image = campaign_image(number, variant);
                assert_eq!(image.is_some(), expected.is_some(), "{number}:{variant}");
                let (Some(image), Some(entry)) = (image, expected) else {
                    continue;
                };
                actual += 1;
                assert_eq!((image.width, image.height), (entry.side, entry.side));
                let base = campaign_image(number, 0).unwrap();
                assert!(image.bytes.chunks_exact(4).any(|p| p[3] > 128));
                assert!(
                    image
                        .bytes
                        .chunks_exact(4)
                        .zip(base.bytes.chunks_exact(4))
                        .all(|(a, b)| a[3] == b[3]),
                    "{number}:{variant} silhouette"
                );
            }
        }
        assert_eq!(actual, 471);
        assert_eq!(COMPLETED_MONSTERS.len(), actual);
    }
}
