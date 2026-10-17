//! Deterministic packaging of accepted source layers; no window or game state.
use macroquad::prelude::{BLANK, Image};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
#[path = "../src/game_art_held_geometry.rs"]
mod geometry;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}
fn load(path: impl AsRef<Path>) -> Image {
    Image::from_file_with_format(&std::fs::read(root().join(path)).unwrap(), None).unwrap()
}
fn crop(source: &Image, x: usize, y: usize, w: usize, h: usize) -> Image {
    let mut out = Image::gen_image_color(w as u16, h as u16, BLANK);
    for row in 0..h {
        let i = ((y + row) * source.width as usize + x) * 4;
        out.bytes[row * w * 4..(row + 1) * w * 4].copy_from_slice(&source.bytes[i..i + w * 4]);
    }
    out
}
fn cell(source: &Image, index: usize) -> Image {
    crop(source, index % 16 * 128, index / 16 * 128, 128, 128)
}
fn bounds(source: &Image) -> [usize; 4] {
    let mut out = [source.width as usize, source.height as usize, 0, 0];
    for (i, p) in source.bytes.chunks_exact(4).enumerate() {
        if p[3] >= 96 {
            let x = i % source.width as usize;
            let y = i / source.width as usize;
            out[0] = out[0].min(x);
            out[1] = out[1].min(y);
            out[2] = out[2].max(x + 1);
            out[3] = out[3].max(y + 1);
        }
    }
    out
}
fn main() {
    let folder = root().join("assets/sprites/held-native64");
    let input: Value =
        serde_json::from_slice(
            &std::fs::read(root().join(
                "docs/reviews/refonte-agent-2026-10-06/held-weapon-review-01/build-inputs.json",
            ))
            .unwrap(),
        )
        .unwrap();
    let entries = input["entries"].as_array().unwrap();
    let mut atlas = Image::gen_image_color(2048, (entries.len().div_ceil(16) * 128) as u16, BLANK);
    let original_bodies = load("assets/sprites/flat-agents/bodies-128.png");
    let original_fingers = load("assets/sprites/flat-agents/fingers-128.png");
    let extra_bodies = load("assets/sprites/equipment-native64/bodies-128.png");
    let extra_fingers = load("assets/sprites/equipment-native64/fingers-128.png");
    let mut manifest = Vec::new();
    let mut checked = 0;
    for (index, e) in entries.iter().enumerate() {
        let raw = load(e["source"].as_str().unwrap());
        let b = bounds(&raw);
        let raw = crop(&raw, b[0], b[1], b[2] - b[0], b[3] - b[1]);
        let grip = [
            e["grip"][0].as_f64().unwrap() as f32 * raw.width as f32,
            e["grip"][1].as_f64().unwrap() as f32 * raw.height as f32,
        ];
        let opposed = e["kind"] == 2;
        let (scale, angle) = if opposed {
            (48. / raw.height as f32, std::f32::consts::FRAC_PI_2)
        } else {
            (e["previous_scale"].as_f64().unwrap() as f32 * 2. * 1.55, 0.)
        };
        let mut target = geometry::HEAVY_HANDS[0];
        if !opposed {
            target[1] = target[1].max(49. + grip[1] * scale);
        }
        let support = if opposed {
            geometry::HEAVY_HANDS[1]
        } else {
            [
                target[0]
                    + (e["support"][0].as_f64().unwrap() as f32 * raw.width as f32 - grip[0])
                        * scale,
                target[1]
                    + (e["support"][1].as_f64().unwrap() as f32 * raw.height as f32 - grip[1])
                        * scale,
            ]
        };
        let hands = [target, support];
        let weapon = geometry::transformed(&raw, grip, target, scale, angle);
        let box_weapon = bounds(&weapon);
        assert!(
            box_weapon[0] > 0 && box_weapon[1] > 0 && box_weapon[2] < 128 && box_weapon[3] < 128,
            "Clipped weapon {}",
            e["id"]
        );
        for row in 0..128 {
            let a = ((index / 16 * 128 + row) * 2048 + index % 16 * 128) * 4;
            atlas.bytes[a..a + 512].copy_from_slice(&weapon.bytes[row * 512..(row + 1) * 512]);
        }
        // Check every armor/color, rather than just the three review examples.
        for outfit in 0..19 {
            for variant in 0..3 {
                let (n, bodies, fingers) = if outfit < 12 {
                    (outfit, &original_bodies, &original_fingers)
                } else {
                    (outfit - 12, &extra_bodies, &extra_fingers)
                };
                let i = (n * 3 + variant) * 4;
                let body = cell(bodies, i);
                let image = geometry::compose(
                    &body,
                    &cell(bodies, i + 2),
                    &[
                        cell(fingers, (i + if opposed { 3 } else { 2 }) * 2),
                        cell(fingers, (i + if opposed { 3 } else { 2 }) * 2 + 1),
                    ],
                    &weapon,
                    hands,
                    opposed,
                );
                assert!(
                    image.bytes[..48 * 128 * 4]
                        .chunks_exact(4)
                        .zip(body.bytes.chunks_exact(4))
                        .all(|(a, b)| a[3] == b[3] && (a[3] == 0 || a == b)),
                    "Changed head {} {outfit} {variant}",
                    e["id"]
                );
                checked += 1;
            }
        }
        manifest.push(json!({"id":e["id"],"model":e["model"],"review_id":e["review_id"],"layer":index,"hands":hands,"opposed":opposed,"source":e["source"],"source_sha256":e["source_sha256"],"angle_degrees":if opposed{90}else{0},"scale":scale}));
        println!("{} : 57 armor/color combinations verified", e["review_id"]);
    }
    std::fs::create_dir_all(&folder).unwrap();
    // Macroquad exports framebuffer order (bottom-up), whereas these CPU layers
    // are top-down. Reverse once before export to retain source pixel order.
    let original = atlas.bytes.clone();
    for y in 0..atlas.height as usize {
        let row = atlas.width as usize * 4;
        atlas.bytes[y * row..(y + 1) * row].copy_from_slice(
            &original[(atlas.height as usize - y - 1) * row..(atlas.height as usize - y) * row],
        );
    }
    atlas.export_png(folder.join("weapons-128.png").to_str().unwrap());
    std::fs::write(folder.join("manifest.json"),serde_json::to_vec_pretty(&json!({"review_id":"HELD-WEAPON-64-01-ACCEPT01","entries":manifest,"native_tile":64,"composition_canvas":128,"verified_armor_palette_combinations":checked,"ordinary_firearms_changed":false,"one_handed_weapons_changed":false})).unwrap()).unwrap();
    println!(
        "Accepted held layers packaged: {} models, {checked} combinations",
        entries.len()
    );
}
