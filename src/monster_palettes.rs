//! Explicit reviewed RGB maps, restricted by per-model material masks.
//! Palette identity is visual only; it grants no attack, resistance or status.

#[derive(Clone, Copy, Debug)]
pub struct MaterialPalette {
    pub number: u16,
    pub variant: u8,
    pub colors: &'static [(u32, u32)],
}

include!("../assets/sprites/campaign-monsters/material-palettes.rs");
include!("../assets/sprites/campaign-monsters/blue-material-palettes.rs");

pub fn all_palettes() -> impl Iterator<Item = &'static MaterialPalette> {
    MATERIAL_PALETTES.iter().chain(BLUE_MATERIAL_PALETTES)
}

pub fn palette(number: u16, variant: u8) -> Option<&'static MaterialPalette> {
    all_palettes().find(|p| p.number == number && p.variant == variant)
}

pub fn recolor_rgb(number: u16, variant: u8, rgb: [u8; 3]) -> [u8; 3] {
    palette(number, variant).map_or(rgb, |p| p.recolor(rgb))
}

impl MaterialPalette {
    pub fn recolor(&self, rgb: [u8; 3]) -> [u8; 3] {
        let key = u32::from(rgb[0]) << 16 | u32::from(rgb[1]) << 8 | u32::from(rgb[2]);
        self.colors
            .binary_search_by_key(&key, |p| p.0)
            .map_or(rgb, |i| {
                let value = self.colors[i].1;
                [(value >> 16) as u8, (value >> 8) as u8, value as u8]
            })
    }

    /// A shared RGB can belong to skin and metal. Only authored material pixels
    /// are tinted; alpha and all protected pixels remain byte-identical.
    pub fn apply_rgba(&self, image: &mut [u8], mask: &[u8]) {
        assert_eq!(image.len(), mask.len());
        assert_eq!(image.len() % 4, 0);
        for (pixel, material) in image.chunks_exact_mut(4).zip(mask.chunks_exact(4)) {
            if pixel[3] != 0 && material[0] != 0 {
                let color = self.recolor([pixel[0], pixel[1], pixel[2]]);
                pixel[..3].copy_from_slice(&color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_flesh_color_on_protected_cage_or_skull_stays_identical() {
        let p = palette(138, 3).unwrap();
        let source = p.colors[0].0;
        let rgb = [(source >> 16) as u8, (source >> 8) as u8, source as u8];
        let mut pixels = vec![
            rgb[0], rgb[1], rgb[2], 255, rgb[0], rgb[1], rgb[2], 255, 7, 8, 9, 0,
        ];
        let before = pixels.clone();
        p.apply_rgba(&mut pixels, &[255, 0, 0, 255, 0, 0, 0, 0, 255, 0, 0, 255]);
        assert_ne!(&pixels[..3], &before[..3]);
        assert_eq!(pixels[3], 255);
        assert_eq!(&pixels[4..], &before[4..]);
    }

    #[test]
    fn every_palette_has_unique_identity_and_sorted_exact_source_colors() {
        let mut seen = std::collections::BTreeSet::new();
        assert_eq!(MATERIAL_PALETTES.len(), 180);
        assert_eq!(BLUE_MATERIAL_PALETTES.len(), 51);
        assert!(EXTENDED_PALETTES_APPROVED);
        assert!(BLUE_PALETTES_APPROVED);
        for p in all_palettes() {
            assert!(seen.insert((p.number, p.variant)));
            assert!(matches!(p.variant, 2 | 3 | 4));
            assert!(!p.colors.is_empty());
            assert!(p.colors.windows(2).all(|pair| pair[0].0 < pair[1].0));
        }
        assert_eq!(recolor_rgb(150, 2, [213, 207, 174]), [213, 207, 174]);
        assert_eq!(recolor_rgb(156, 3, [65, 58, 55]), [65, 58, 55]);
    }
}
