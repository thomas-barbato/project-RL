//! Explicit native preparation tools; invoked only by ignored export tests.
use crate::game_art;
use macroquad::prelude::*;
use project_rl::entity::EquipmentPalette;
use std::path::Path;

fn save(mut image: Image, path: &Path) {
    let stride = usize::from(image.width) * 4;
    let bytes = image.bytes.clone();
    for y in 0..usize::from(image.height) {
        let from = (usize::from(image.height) - 1 - y) * stride;
        image.bytes[y * stride..(y + 1) * stride].copy_from_slice(&bytes[from..from + stride]);
    }
    image.export_png(path.to_str().unwrap());
}

#[test]
#[ignore = "explicit canonical weapon palette export; no gameplay changes"]
fn export_vertical_weapon_palettes18() {
    let out =
        Path::new("artifacts/vertical-walk-review-18/current/extension-catalog/weapon-palettes");
    std::fs::create_dir_all(out).unwrap();
    let mut rows = Vec::new();
    for model in 0..113u16 {
        let two = game_art::weapon_uses_two_hands(model);
        for left in if two { vec![false] } else { vec![false, true] } {
            let (base, anchor, support) =
                game_art::vertical_review_weapon(model, left, EquipmentPalette::Original).unwrap();
            let alpha: Vec<_> = base.bytes.chunks_exact(4).map(|p| p[3]).collect();
            let mut palettes = Vec::new();
            for color in EquipmentPalette::ALL {
                let (image, a, s) = game_art::vertical_review_weapon(model, left, color).unwrap();
                assert_eq!((anchor, support), (a, s));
                assert_eq!(
                    alpha,
                    image
                        .bytes
                        .chunks_exact(4)
                        .map(|p| p[3])
                        .collect::<Vec<_>>()
                );
                let file = out.join(format!(
                    "W-{model:03}-{}-p{}.png",
                    u8::from(left),
                    color.index()
                ));
                save(image, &file);
                palettes.push(
                    serde_json::json!({"index":color.index(),"name":color.name(),"file":file}),
                );
            }
            rows.push(serde_json::json!({"model":model,"left":left,"two_handed":two,"anchor":anchor,"support":support,"palettes":palettes}));
        }
    }
    assert_eq!(rows.len(), 152);
    std::fs::write(out.join("manifest.json"),serde_json::to_vec_pretty(&serde_json::json!({"bindings":rows,"models":113,"palettes":7,"alpha_geometry_unchanged":true})).unwrap()).unwrap();
}

#[test]
#[ignore = "explicit C06 preparation after every creature source is conditioned"]
fn prepare_vertical_monster_palettes18() {
    let root =
        Path::new("assets/source-art/vertical-walk-review-18/extension-C06/conditioned-monsters");
    let manifest: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join("manifest.json")).unwrap()).unwrap();
    assert_eq!(manifest["conditioned_models"].as_u64(), Some(129));
    let out = root.join("palettes");
    std::fs::create_dir_all(&out).unwrap();
    let only = std::env::var("RL_VERTICAL_ONLY_MODEL")
        .ok()
        .map(|v| v.parse::<u16>().expect("numeric model"));
    let mut entries = if let Some(number) = only {
        let old: serde_json::Value = serde_json::from_slice(
            &std::fs::read(out.join("manifest.json")).expect("prior complete palette preparation"),
        )
        .unwrap();
        assert_eq!(old["alpha_geometry_unchanged"], true);
        old["entries"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|e| e["number"].as_u64() != Some(u64::from(number)))
            .cloned()
            .collect()
    } else {
        Vec::new()
    };
    for p in manifest["profiles"].as_array().unwrap() {
        let number = p["number"].as_u64().unwrap() as u16;
        if only.is_some_and(|n| n != number) {
            continue;
        }
        let side = p["side"].as_u64().unwrap() as u16;
        let source = Image::from_file_with_format(
            &std::fs::read(p["file"].as_str().unwrap()).unwrap(),
            Some(ImageFormat::Png),
        )
        .unwrap();
        let idle = Image::from_file_with_format(
            &std::fs::read(p["standing"].as_str().unwrap()).unwrap(),
            Some(ImageFormat::Png),
        )
        .unwrap();
        let mut atlas = Image::gen_image_color(side * 6, side * 3, BLANK);
        let stride = usize::from(side) * 6 * 4;
        atlas.bytes[..source.bytes.len()].copy_from_slice(&source.bytes);
        for y in 0..usize::from(side) {
            let at = (usize::from(side) * 2 + y) * stride;
            atlas.bytes[at..at + usize::from(side) * 4].copy_from_slice(
                &idle.bytes[y * usize::from(side) * 4..(y + 1) * usize::from(side) * 4],
            );
        }
        let alpha: Vec<_> = atlas.bytes.chunks_exact(4).map(|p| p[3]).collect();
        for variant in project_rl::monster_art::variant_ids(number, true, true) {
            let mut colored = atlas.clone();
            game_art::vertical_monster_palette(number, variant, &mut colored).unwrap();
            assert_eq!(
                alpha,
                colored
                    .bytes
                    .chunks_exact(4)
                    .map(|p| p[3])
                    .collect::<Vec<_>>()
            );
            let file = out.join(format!("MON-{number:03}-v{variant}.png"));
            save(colored, &file);
            entries.push(serde_json::json!({"number":number,"variant":variant,"file":file,"side":side,"foot":p["foot"],"name":p["name"]}));
        }
    }
    assert_eq!(entries.len(), 471);
    std::fs::write(out.join("manifest.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":"VERT18-C06-MON","entries":entries,"alpha_geometry_unchanged":true,"normal_game_enabled":false})).unwrap()).unwrap();
}
