//! Read-only audit: call the live CPU recoloring function without editing assets.
use macroquad::prelude::*;
use project_rl::entity::EquipmentPalette;
use std::collections::BTreeSet;

struct ItemArt { illustration: u16, icon: u8 }
#[path = "../../src/game_art_palette.rs"]
mod palettes;

fn orange(p: &[u8]) -> bool { p[3] > 0 && p[0] > 150 && p[1] > 90 && p[2] < 65 }
fn main() {
    let root = std::path::Path::new("assets/sprites/cyberpunk");
    let output = std::path::Path::new("artifacts/audit-sprites-2026-10-05");
    let poses = ["unarmed", "side", "two-handed", "raised", "melee-two-handed", "dual-melee", "dual-raised", "dual-mixed"];
    let mut records = Vec::new();
    for side in [48_u16, 128] {
        let sources: Vec<_> = poses.iter().map(|pose| palettes::decoded(&std::fs::read(root.join(format!("player-poses-{pose}-{side}.png"))).unwrap())).collect();
        for palette in EquipmentPalette::ALL {
            let mut sheet = Image::gen_image_color(side * 8, side * 17, BLANK);
            for (pose, source) in sources.iter().enumerate() {
                for outfit in 0..17 {
                    let colors = [palette, EquipmentPalette::Original, EquipmentPalette::Original];
                    let tile = palettes::figure(source, outfit, pose, colors);
                    let original = palettes::figure(source, outfit, pose, [EquipmentPalette::Original; 3]);
                    let mut retained_orange = Vec::new();
                    let mut colors = BTreeSet::new();
                    let mut visible = 0;
                    for (index, (p, old)) in tile.bytes.chunks_exact(4).zip(original.bytes.chunks_exact(4)).enumerate() {
                        if p[3] > 0 { visible += 1; colors.insert([p[0],p[1],p[2]]); }
                        if orange(p) && p == old {
                            retained_orange.push([index % usize::from(side), index / usize::from(side)]);
                        }
                    }
                    records.push(serde_json::json!({"side": side, "pose": poses[pose], "outfit": outfit,
                        "palette": palette.name(), "visible_pixels": visible, "unique_rgb": colors.len(),
                        "retained_orange": retained_orange}));
                    if side == 48 {
                        for y in 0..side { for x in 0..side {
                            sheet.set_pixel(pose as u32 * u32::from(side) + u32::from(x), u32::from(outfit) * u32::from(side) + u32::from(y), tile.get_pixel(u32::from(x), u32::from(y)));
                        }}
                    }
                }
            }
            // export_png flips framebuffer images. These CPU pixels are already
            // top-down; write them raw for lossless PNG encoding by the QA script.
            if side == 48 { std::fs::write(output.join(format!("figures-palette-{}.rgba", palette.index())), &sheet.bytes).unwrap(); }
        }
    }
    std::fs::write(output.join("mesures.json"), serde_json::to_vec(&records).unwrap()).unwrap();
    println!("1904 figures measured with the current recoloring code; 7 native sheets exported.");
}
