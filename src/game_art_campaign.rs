//! Native campaign bodies, feet anchors and explicit material palette swaps.
//! Colors never select damage. A cached texture serves board and inspection.
use macroquad::prelude::*;
use project_rl::asset_cache::ByteCache;
use project_rl::monster_art::{MonsterArt, art};
#[cfg(test)]
use project_rl::monster_palettes::recolor_rgb as palette;

struct Bank {
    sources: [(u16, super::palettes::LazyImage); 4],
    materials: [(u16, super::palettes::LazyImage); 4],
    cells: ByteCache<(u16, u8, bool), super::portrait::Sheet>,
}

thread_local! {
    static BANK: std::cell::RefCell<Bank> = std::cell::RefCell::new(Bank {
        sources: [
            (64, decode(include_bytes!("../assets/sprites/campaign-monsters/living-64.png"))),
            (128, decode(include_bytes!("../assets/sprites/campaign-monsters/living-128.png"))),
            (192, decode(include_bytes!("../assets/sprites/campaign-monsters/living-192.png"))),
            (256, decode(include_bytes!("../assets/sprites/campaign-monsters/living-256.png"))),
        ],
        materials: [
            (64, decode(include_bytes!("../assets/sprites/campaign-monsters/materials-64.png"))),
            (128, decode(include_bytes!("../assets/sprites/campaign-monsters/materials-128.png"))),
            (192, decode(include_bytes!("../assets/sprites/campaign-monsters/materials-192.png"))),
            (256, decode(include_bytes!("../assets/sprites/campaign-monsters/materials-256.png"))),
        ],
        cells: ByteCache::new(32 * 1024 * 1024),
    });
}

fn decode(bytes: &'static [u8]) -> super::palettes::LazyImage {
    super::palettes::LazyImage::new(bytes)
}

pub(super) fn prewarm() {
    BANK.with(|_| {});
}

#[cfg(debug_assertions)]
pub(crate) fn export_variant(number: u16, variant: u8, path: &std::path::Path) {
    with_sheet(number, variant, |sheet, _| {
        let mut image = sheet.texture.get_texture_data();
        // Macroquad export_png flips rows for framebuffer captures; these
        // texture bytes are already top-down, so compensate before export.
        let stride = usize::from(image.width) * 4;
        let original = image.bytes.clone();
        for y in 0..usize::from(image.height) {
            let source = (usize::from(image.height) - 1 - y) * stride;
            image.bytes[y * stride..(y + 1) * stride]
                .copy_from_slice(&original[source..source + stride]);
        }
        image.export_png(path.to_str().unwrap());
    });
}

fn with_sheet(number: u16, variant: u8, draw: impl FnOnce(&super::portrait::Sheet, &MonsterArt)) {
    let Some(entry) = art(number, variant) else {
        return;
    };
    let key = (number, variant, super::walking::proportions_enabled());
    BANK.with(|bank| {
        let mut bank = bank.borrow_mut();
        if let Some(sheet) = bank.cells.get(&key) {
            draw(sheet, entry);
            return;
        }
        let image = if let Some(image) = proposed_front(number, variant)
            .or_else(|| super::finish::campaign_image(number, variant))
        {
            image
        } else {
            let Some((_, source)) = bank.sources.iter().find(|(side, _)| *side == entry.side)
            else {
                return;
            };
            let side = f32::from(entry.side);
            let mut image = source.sub_image(Rect::new(
                f32::from(entry.index % 8) * side,
                f32::from(entry.index / 8) * side,
                side,
                side,
            ));
            if let Some(palette) = project_rl::monster_palettes::palette(number, variant) {
                let (_, materials) = bank
                    .materials
                    .iter()
                    .find(|(side, _)| *side == entry.side)
                    .unwrap();
                let mask = materials.sub_image(Rect::new(
                    f32::from(entry.index % 8) * side,
                    f32::from(entry.index / 8) * side,
                    side,
                    side,
                ));
                palette.apply_rgba(&mut image.bytes, &mask.bytes);
            }
            image
        };
        let bytes = image.bytes.len();
        bank.cells.insert(
            key,
            super::portrait::Sheet::from_image(&image, entry.side),
            bytes,
        );
        draw(bank.cells.get(&key).unwrap(), entry);
    });
}

/// CPU reference used to transport the existing material colors to whole walk poses.
pub(super) fn reference_image(number: u16, variant: u8) -> Option<Image> {
    let entry = art(number, variant)?;
    if let Some(image) = proposed_front(number, variant) {
        return Some(image);
    }
    if let Some(image) = super::finish::campaign_image(number, variant) {
        return Some(image);
    }
    BANK.with_borrow(|bank| {
        let (_, source) = bank.sources.iter().find(|(side, _)| *side == entry.side)?;
        let side = f32::from(entry.side);
        let rect = Rect::new(
            f32::from(entry.index % 8) * side,
            f32::from(entry.index / 8) * side,
            side,
            side,
        );
        let mut image = source.sub_image(rect);
        if let Some(palette) = project_rl::monster_palettes::palette(number, variant) {
            let (_, materials) = bank.materials.iter().find(|(s, _)| *s == entry.side)?;
            palette.apply_rgba(&mut image.bytes, &materials.sub_image(rect).bytes);
        }
        Some(image)
    })
}

fn proposed_front(number: u16, variant: u8) -> Option<Image> {
    if !super::walking::proportions_enabled() || ![22, 24].contains(&number) {
        return None;
    }
    let path = format!("assets/animations/walk-11/fronts/MON-{number:03}-{variant}.png");
    Image::from_file_with_format(
        &project_rl::data_paths::read(&path).ok()?,
        Some(ImageFormat::Png),
    )
    .ok()
}

fn destination(cell: Rect, entry: &MonsterArt) -> Rect {
    let scale = cell.w / 64.;
    Rect::new(
        cell.center().x - f32::from(entry.foot[0]) * scale,
        cell.bottom() - f32::from(entry.foot[1]) * scale,
        f32::from(entry.side) * scale,
        f32::from(entry.side) * scale,
    )
}

pub(super) fn draw(cell: Rect, number: u16, variant: u8, opacity: f32, world: bool) {
    with_sheet(number, variant, |sheet, entry| {
        if !world {
            sheet.draw(cell, 0, opacity);
            return;
        }
        let rect = destination(cell, entry);
        let side = entry.side;
        super::draw_contact_shadow(
            rect,
            [
                entry.foot[0] as u8,
                (entry.foot[1] - 1) as u8,
                entry.shadow as u8,
            ],
            side,
            opacity,
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

/// Source-derived fallen body, using the existing rotated-corpse convention.
/// Encoded in the existing char slot: no residue/save-layout extension.
pub(crate) fn corpse_code(number: u16, variant: u8) -> char {
    char::from_u32(0xf0000 + u32::from(number) * 8 + u32::from(variant)).unwrap()
}

pub(crate) fn decode_corpse(code: char) -> Option<(u16, u8)> {
    let value = (code as u32).checked_sub(0xf0000)?;
    let key = ((value / 8).try_into().ok()?, (value % 8) as u8);
    art(key.0, key.1).map(|_| key)
}

pub(crate) fn draw_corpse(cell: Rect, number: u16, variant: u8, opacity: f32) {
    with_sheet(number, variant, |sheet, entry| {
        let rect = destination(cell, entry);
        let center = vec2(cell.center().x, cell.bottom() - 12. * cell.w / 64.);
        super::draw_contact_shadow(cell, [32, 52, entry.shadow.min(50) as u8], 64, opacity);
        let size = rect.size();
        crate::world_readability::texture(
            &sheet.texture,
            center.x - size.x / 2.,
            center.y - size.y / 2.,
            Color::new(0.68, 0.62, 0.57, opacity),
            DrawTextureParams {
                dest_size: Some(size),
                rotation: std::f32::consts::FRAC_PI_2,
                pivot: Some(center),
                ..Default::default()
            },
        );
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn feet_keep_native_scale_and_palettes_protect_other_materials() {
        let entry = art(156, 0).unwrap();
        let rect = destination(Rect::new(100., 200., 64., 64.), entry);
        assert_eq!(rect, Rect::new(68., 168., 128., 128.));
        assert_eq!(palette(150, 2, [213, 207, 174]), [213, 207, 174]);
        assert_eq!(palette(150, 3, [136, 94, 66]), [136, 94, 66]);
        assert_ne!(palette(144, 3, [181, 48, 45]), [181, 48, 45]);
        for variant in 0..4 {
            assert_eq!(
                decode_corpse(corpse_code(144, variant)),
                Some((144, variant))
            );
        }
        for entry in project_rl::monster_palettes::all_palettes() {
            assert_eq!(
                decode_corpse(corpse_code(entry.number, entry.variant)),
                Some((entry.number, entry.variant))
            );
        }
        assert_eq!(decode_corpse('X'), None);
        let mut residue = crate::combat_visuals::CombatResidue::default();
        residue.cells.insert(
            project_rl::world::GridPos::new(3, 4),
            crate::combat_visuals::ResidueCell {
                blood: 6,
                corpse_symbol: Some(corpse_code(144, 3)),
                ..Default::default()
            },
        );
        let bytes = bincode::serialize(&residue).unwrap();
        let restored: crate::combat_visuals::CombatResidue = bincode::deserialize(&bytes).unwrap();
        assert_eq!(restored, residue);
        let cell = residue.cells.values().next().unwrap();
        let json = serde_json::to_string(cell).unwrap();
        let restored_cell: crate::combat_visuals::ResidueCell =
            serde_json::from_str(&json).unwrap();
        assert_eq!(&restored_cell, cell);
    }
}
