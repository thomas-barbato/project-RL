//! Recolor authored material labels; geometry, steel, skin and outlines stay fixed.
use super::*;
use std::collections::BTreeMap;
pub(super) fn decoded(bytes: &[u8]) -> Image {
    Image::from_file_with_format(bytes, Some(ImageFormat::Png)).expect("valid embedded sprite")
}
pub(super) fn cell(source: &Image, index: u16, side: u16) -> Image {
    source.sub_image(Rect::new(
        f32::from(index % 16 * side),
        f32::from(index / 16 * side),
        f32::from(side),
        f32::from(side),
    ))
}
fn secondary(palette: EquipmentPalette) -> EquipmentPalette {
    use EquipmentPalette::*;
    match palette {
        Original | ReviewedVariant1 | ReviewedVariant2 => Original,
        Red => Turquoise,
        Blue => Yellow,
        Violet => Turquoise,
        Turquoise => Violet,
        Yellow => Violet,
        Gray => Gray,
    }
}
pub(super) fn recolor(image: &mut Image, mask: &Image, palette: EquipmentPalette) {
    assert_eq!((image.width, image.height), (mask.width, mask.height));
    if palette == EquipmentPalette::Original {
        return;
    }
    recolor_ramps(image, mask, palette.ramp(), secondary(palette).ramp());
}
fn recolor_ramps(
    image: &mut Image,
    mask: &Image,
    primary: Option<[[u8; 3]; 3]>,
    secondary: Option<[[u8; 3]; 3]>,
) {
    assert_eq!((image.width, image.height), (mask.width, mask.height));
    for (pixel, label) in image
        .bytes
        .chunks_exact_mut(4)
        .zip(mask.bytes.chunks_exact(4))
    {
        let ramp = match label[0] {
            1 => primary,
            2 => secondary,
            _ => continue,
        };
        let Some(ramp) = ramp else { continue };
        let value = f32::from(pixel[0].max(pixel[1]).max(pixel[2])) / 255.0;
        let (low, high, t) = if value < 0.53 {
            (ramp[0], ramp[1], ((value - 0.12) / 0.41).clamp(0.0, 1.0))
        } else {
            (ramp[1], ramp[2], ((value - 0.53) / 0.47).clamp(0.0, 1.0))
        };
        for n in 0..3 {
            pixel[n] = (f32::from(low[n]) * (1.0 - t) + f32::from(high[n]) * t).round() as u8;
        }
    }
}
fn recolor_default_item(image: &mut Image, mask: &Image, illustration: u16) {
    // Current catalogue bindings: the same approved default materials as the
    // held pistol and precision-rifle families, independent of the wearer.
    match illustration {
        150..=155 => recolor_ramps(image, mask, EquipmentPalette::Gray.ramp(), None),
        35..=46 => recolor_ramps(
            image,
            mask,
            Some([[8, 9, 10], [56, 54, 58], [146, 157, 169]]),
            Some([[0, 0, 1], [56, 54, 58], [67, 84, 118]]),
        ),
        _ => {}
    }
}
struct ItemSources {
    items: Image,
    flat_outfits: Image,
    materials: Image,
    cache: BTreeMap<(u16, EquipmentPalette), (Texture2D, f64)>,
}
thread_local! {
    #[cfg(debug_assertions)]
    static CREATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static ITEMS: std::cell::RefCell<ItemSources> = std::cell::RefCell::new(ItemSources {
        items: decoded(include_bytes!("../assets/sprites/cyberpunk/crawl-items-32.png")),
        flat_outfits: decoded(include_bytes!("../assets/sprites/flat-agents/outfit-icons-32.png")),
        materials: decoded(include_bytes!("../assets/sprites/cyberpunk/crawl-item-materials-32.png")),cache: BTreeMap::new(),
    });
}
pub(super) fn prewarm() {
    ITEMS.with(|_| {});
}
pub(super) fn draw_item(
    rect: Rect,
    art: ItemArt,
    palette: EquipmentPalette,
    _side: u16,
    opacity: f32,
) {
    ITEMS.with(|sources| {
        let mut sources = sources.borrow_mut();
        let key = (art.illustration, palette);
        if !sources.cache.contains_key(&key) {
            let image = if let Some(outfit) = flat_outfit(art.illustration) {
                let variant = match palette {
                    EquipmentPalette::Original => 0,
                    EquipmentPalette::ReviewedVariant1
                    | EquipmentPalette::Blue
                    | EquipmentPalette::Violet
                    | EquipmentPalette::Turquoise => 1,
                    EquipmentPalette::ReviewedVariant2 => 2,
                    _ => 2,
                };
                cell(&sources.flat_outfits, outfit * 3 + variant, 32)
            } else {
                let mut image = cell(&sources.items, art.illustration, 32);
                let mask = cell(&sources.materials, art.illustration, 32);
                recolor_default_item(&mut image, &mask, art.illustration);
                recolor(&mut image, &mask, palette);
                image
            };
            let sprite = Texture2D::from_image(&image);
            sprite.set_filter(FilterMode::Nearest);
            evict(&mut sources.cache, 256);
            sources.cache.insert(key, (sprite, get_time()));
            #[cfg(debug_assertions)]
            CREATIONS.with(|count| count.set(count.get() + 1));
        }
        let (sprite, time) = sources.cache.get_mut(&key).unwrap();
        *time = get_time();
        draw_texture_ex(
            sprite,
            rect.x,
            rect.y,
            Color::new(1.0, 1.0, 1.0, opacity),
            DrawTextureParams {
                dest_size: Some(rect.size()),
                ..Default::default()
            },
        );
    });
}

fn flat_outfit(illustration: u16) -> Option<u16> {
    match illustration {
        200..=211 => Some(illustration - 200),
        108 => Some(8),
        110 => Some(9),
        111 => Some(10),
        112 => Some(11),
        _ => None,
    }
}

#[cfg(debug_assertions)]
pub(super) fn cache_stats() -> (usize, usize) {
    (
        ITEMS.with(|s| s.borrow().cache.len()),
        CREATIONS.with(|c| c.get()),
    )
}
pub(super) fn evict<K: Copy + Ord>(cache: &mut BTreeMap<K, (Texture2D, f64)>, limit: usize) {
    if cache.len() >= limit {
        if let Some(key) = cache
            .iter()
            .min_by(|a, b| a.1.1.total_cmp(&b.1.1))
            .map(|(key, _)| *key)
        {
            cache.remove(&key);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn approved_default_weapon_materials_keep_item_alpha_and_protected_parts() {
        let items = decoded(include_bytes!(
            "../assets/sprites/cyberpunk/crawl-items-32.png"
        ));
        let materials = decoded(include_bytes!(
            "../assets/sprites/cyberpunk/crawl-item-materials-32.png"
        ));
        for index in (35..=46).chain(150..=155) {
            let original = cell(&items, index, 32);
            let mut image = original.clone();
            let mask = cell(&materials, index, 32);
            recolor_default_item(&mut image, &mask, index);
            assert_ne!(original.bytes, image.bytes);
            for ((a, b), label) in original
                .bytes
                .chunks_exact(4)
                .zip(image.bytes.chunks_exact(4))
                .zip(mask.bytes.chunks_exact(4))
            {
                assert_eq!(a[3], b[3]);
                if label[0] == 0 {
                    assert_eq!(a, b);
                }
            }
        }
    }
    #[test]
    fn every_palette_preserves_alpha_and_protected_pixels_on_all_outfits() {
        let source = decoded(include_bytes!(
            "../assets/sprites/cyberpunk/crawl-bodies-32.png"
        ));
        let maps = decoded(include_bytes!(
            "../assets/sprites/cyberpunk/crawl-body-materials-32.png"
        ));
        for outfit in 0..17 {
            let original = cell(&source, outfit, 32);
            let mask = cell(&maps, outfit, 32);
            assert!(
                mask.bytes.chunks_exact(4).any(|p| p[0] == 1),
                "primary {outfit}"
            );
            assert!(
                mask.bytes.chunks_exact(4).any(|p| p[0] == 2),
                "secondary {outfit}"
            );
            for palette in EquipmentPalette::ALL {
                let mut colored = original.clone();
                recolor(&mut colored, &mask, palette);
                for ((a, b), m) in original
                    .bytes
                    .chunks_exact(4)
                    .zip(colored.bytes.chunks_exact(4))
                    .zip(mask.bytes.chunks_exact(4))
                {
                    assert_eq!(a[3], b[3]);
                    if m[0] == 0 {
                        assert_eq!(a, b, "protected {outfit}/{palette:?}");
                    }
                }
            }
        }
    }
    #[test]
    fn all_item_frames_are_nonempty_and_match_their_material_maps() {
        let items = decoded(include_bytes!(
            "../assets/sprites/cyberpunk/crawl-items-32.png"
        ));
        let mask = decoded(include_bytes!(
            "../assets/sprites/cyberpunk/crawl-item-materials-32.png"
        ));
        assert_eq!((items.width, items.height), (mask.width, mask.height));
        for index in 0..ITEM_COUNT as u16 {
            assert!(
                cell(&items, index, 32)
                    .bytes
                    .chunks_exact(4)
                    .any(|p| p[3] > 0),
                "{index}"
            );
        }
    }
}
