//! Review-only export of the production body, held weapon and finger layers.
use super::*;

#[test]
#[ignore = "Explicit movement review source export; requires RL_MOVEMENT_REVIEW_OUTPUT"]
fn movement_catalogue_source_diagnostic() {
    let output = std::path::PathBuf::from(
        std::env::var("RL_MOVEMENT_REVIEW_OUTPUT").expect("review output required"),
    );
    std::fs::create_dir_all(&output).unwrap();
    let mut configurations = vec![("Sans arme".to_owned(), None, None, 0u16)];
    configurations
        .extend((0..HELD_WEAPON_COUNT).map(|m| (format!("Arme {m:03}"), Some(m), None, 0)));
    configurations.extend([
        (
            "Deux pistolets".to_owned(),
            None,
            Some([Some(101), Some(101)]),
            0,
        ),
        (
            "Lame et pistolet".to_owned(),
            None,
            Some([Some(68), Some(101)]),
            0,
        ),
        ("Deux lames".to_owned(), None, Some([Some(68), Some(82)]), 0),
    ]);
    configurations.extend((1..=6).map(|head| (format!("Casque {head}"), None, None, head)));
    let height = configurations.len().div_ceil(16) * 128;
    for index in 0..25 {
        let mut sheets: Vec<_> = (0..3)
            .map(|_| Image::gen_image_color(2048, height as u16, BLANK))
            .collect();
        for (cell, (_, weapon, hands, head)) in configurations.iter().enumerate() {
            let key = (
                BASE + index as u16 + head * HEAD_STRIDE,
                selection(*weapon, *hands),
                [EquipmentPalette::Original; 3],
            );
            let layers = layers(key).expect("production outfit supports this loadout");
            assert_eq!(layers.len(), 3);
            assert!(layers[0].bytes.chunks_exact(4).any(|p| p[3] > 0));
            for (sheet, image) in sheets.iter_mut().zip(&layers) {
                for y in 0..128 {
                    let at = ((cell / 16 * 128 + y) * 2048 + cell % 16 * 128) * 4;
                    sheet.bytes[at..at + 512].copy_from_slice(&image.bytes[y * 512..(y + 1) * 512]);
                }
            }
        }
        for (layer, sheet) in sheets.iter().enumerate() {
            sheet.export_png(
                output
                    .join(format!("class-{index:02}-layer-{layer}.png"))
                    .to_str()
                    .unwrap(),
            );
        }
        part(index, "ranged-core").export_png(
            output
                .join(format!("class-{index:02}-core.png"))
                .to_str()
                .unwrap(),
        );
        part(index, "head-mask").export_png(
            output
                .join(format!("class-{index:02}-head-mask.png"))
                .to_str()
                .unwrap(),
        );
    }
    std::fs::write(output.join("capture.json"), serde_json::to_vec_pretty(&serde_json::json!({
        "native_tile":64,"canvas":128,"columns":16,"outfits":25,
        "weapon_models":HELD_WEAPON_COUNT,
        "configurations":configurations.iter().map(|(label,weapon,hands,head)|serde_json::json!({"label":label,"weapon":weapon,"hands":hands,"head":head})).collect::<Vec<_>>(),
        "exported_pngs_need_vertical_flip":true,
        "production_compositor_used":true,"runtime_animation_integration":false
    })).unwrap()).unwrap();
}
