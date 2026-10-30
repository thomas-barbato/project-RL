//! Read-only CPU reference export for the complete VERT18 extension.
#[cfg(test)]
mod tests {
    use crate::game_art;
    use macroquad::prelude::*;
    use project_rl::entity::EquipmentPalette;
    use std::path::Path;

    fn save(mut image: Image, path: &Path) {
        let stride = usize::from(image.width) * 4;
        let source = image.bytes.clone();
        for y in 0..usize::from(image.height) {
            let from = (usize::from(image.height) - 1 - y) * stride;
            image.bytes[y * stride..(y + 1) * stride].copy_from_slice(&source[from..from + stride]);
        }
        image.export_png(path.to_str().unwrap());
    }

    #[test]
    #[ignore = "explicit CPU export; no campaign or preferences loaded"]
    fn export_vertical_catalog18() {
        let out = Path::new("artifacts/vertical-walk-review-18/current/extension-catalog");
        std::fs::create_dir_all(out.join("monsters")).unwrap();
        std::fs::create_dir_all(out.join("weapons")).unwrap();
        let manifest: serde_json::Value = serde_json::from_str(include_str!(
            "../assets/sprites/campaign-monsters/manifest.json"
        ))
        .unwrap();
        let mut monsters = Vec::new();
        for row in manifest["entries"].as_array().unwrap() {
            if row["variant"].as_u64() != Some(0) {
                continue;
            }
            let number = row["number"].as_u64().unwrap() as u16;
            let image = game_art::vertical_monster_reference(number, 0).unwrap();
            let dimensions = [image.width, image.height];
            let path = out.join(format!("monsters/MON-{number:03}.png"));
            save(image, &path);
            monsters.push(serde_json::json!({"number":number,"file":path,"image_px":dimensions,
                "native_side":row["side"],"foot":row["foot_anchor"],"name":row["display_name"],
                "role":row["role"],"variants":project_rl::monster_art::variant_ids(number,true,true)}));
        }
        let mut weapons = Vec::new();
        for model in 0..113u16 {
            let two = game_art::weapon_uses_two_hands(model);
            for left in if two { vec![false] } else { vec![false, true] } {
                let (image, anchor, support) =
                    game_art::vertical_review_weapon(model, left, EquipmentPalette::Original)
                        .unwrap();
                let path = out.join(format!("weapons/W-{model:03}-{}.png", u8::from(left)));
                save(image, &path);
                weapons.push(serde_json::json!({"model":model,"left":left,"file":path,"anchor":anchor,"support":support,"two_handed":two}));
            }
        }
        assert_eq!(monsters.len(), 129);
        assert_eq!(weapons.len(), 152);
        std::fs::write(out.join("catalog.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "review":"VERT18 extension","normal_game_enabled":false,"monsters":monsters,"weapons":weapons,
            "native_tile_px":64,"reference_export_only":true,"saved_game_loaded":false
        })).unwrap()).unwrap();
    }
}
