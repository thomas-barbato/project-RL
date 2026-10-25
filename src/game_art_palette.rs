//! Recolor authored material labels; geometry, steel, skin and outlines stay fixed.
use super::*;

/// Compressed bytes stay embedded; decode only the source banks actually sampled.
pub(super) struct LazyImage {
    bytes: &'static [u8],
    image: std::cell::OnceCell<Image>,
}
impl LazyImage {
    pub(super) const fn new(bytes: &'static [u8]) -> Self {
        Self {
            bytes,
            image: std::cell::OnceCell::new(),
        }
    }
}
impl std::ops::Deref for LazyImage {
    type Target = Image;
    fn deref(&self) -> &Image {
        self.image.get_or_init(|| decoded(self.bytes))
    }
}
use std::collections::BTreeMap;
include!("game_art_equipment_catalog.rs");
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
        Original | ReviewedVariant1 | ReviewedVariant2 | SlateBlue | MossGreen | Rust | DeepRed => {
            Original
        }
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
/// Finished cloth/paint keeps some of its original worn pigment. The material
/// label still protects skin, steel, secondary cloth and transparent edges.
pub(super) fn recolor_finished(image: &mut Image, mask: &Image, palette: EquipmentPalette) {
    if palette == EquipmentPalette::Original {
        return;
    }
    if palette.is_torso_finish() {
        recolor_torso_finish(image, mask, palette);
        return;
    }
    let original = image.bytes.clone();
    recolor(image, mask, palette);
    for ((pixel, before), label) in image
        .bytes
        .chunks_exact_mut(4)
        .zip(original.chunks_exact(4))
        .zip(mask.bytes.chunks_exact(4))
    {
        if label[0] != 1 {
            continue;
        }
        for c in 0..3 {
            pixel[c] = ((u16::from(pixel[c]) * 7 + u16::from(before[c]) * 3 + 5) / 10) as u8;
        }
    }
}
/// Match the accepted authoring ramps exactly. Only primary material labels
/// change; neutral fastenings, secondary cloth and alpha retain source pixels.
fn recolor_torso_finish(image: &mut Image, mask: &Image, palette: EquipmentPalette) {
    assert_eq!((image.width, image.height), (mask.width, mask.height));
    let ramp = palette.ramp().expect("torso finish ramp");
    for (pixel, label) in image
        .bytes
        .chunks_exact_mut(4)
        .zip(mask.bytes.chunks_exact(4))
    {
        if label[0] != 1 {
            continue;
        }
        let value = f64::from(pixel[0].max(pixel[1]).max(pixel[2])) / 255.;
        let (low, high, t) = if value < 0.53 {
            (ramp[0], ramp[1], ((value - 0.12) / 0.41).clamp(0., 1.))
        } else {
            (ramp[1], ramp[2], ((value - 0.53) / 0.47).clamp(0., 1.))
        };
        for c in 0..3 {
            let mapped = (f64::from(low[c]) * (1. - t) + f64::from(high[c]) * t).round() as u16;
            pixel[c] = ((mapped * 9 + u16::from(pixel[c]) + 5) / 10) as u8;
        }
    }
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
    items: LazyImage,
    flat_outfits: LazyImage,
    materials: LazyImage,
    native: NativeItems,
    cache: BTreeMap<(u16, EquipmentPalette, u16), (Texture2D, f64)>,
}
struct NativeItems {
    large: LazyImage,
    small: LazyImage,
    large_maps: LazyImage,
    small_maps: LazyImage,
}
impl NativeItems {
    fn load() -> Self {
        Self {
            large: LazyImage::new(include_bytes!(
                "../assets/sprites/equipment-native64/icons-64.png"
            )),
            small: LazyImage::new(include_bytes!(
                "../assets/sprites/equipment-native64/icons-32.png"
            )),
            large_maps: LazyImage::new(include_bytes!(
                "../assets/sprites/equipment-native64/materials-64.png"
            )),
            small_maps: LazyImage::new(include_bytes!(
                "../assets/sprites/equipment-native64/materials-32.png"
            )),
        }
    }
    fn image(
        &self,
        illustration: u16,
        palette: EquipmentPalette,
        resolution: u16,
    ) -> Option<Image> {
        if resolution == 64
            && let Some(image) = super::finish::item_image(illustration, palette)
        {
            return Some(image);
        }
        let &(_, index, variants) = NATIVE_ITEMS.iter().find(|(i, _, _)| *i == illustration)?;
        let index = index
            + if variants {
                authored_variant(palette)
            } else {
                0
            };
        let (sheet, maps) = if resolution == 64 {
            (&self.large, &self.large_maps)
        } else {
            (&self.small, &self.small_maps)
        };
        let mut image = cell(sheet, index, resolution);
        if !variants {
            recolor(&mut image, &cell(maps, index, resolution), palette);
        }
        Some(image)
    }
}
pub(super) fn authored_variant(palette: EquipmentPalette) -> u16 {
    match palette {
        EquipmentPalette::Original => 0,
        EquipmentPalette::ReviewedVariant1
        | EquipmentPalette::Blue
        | EquipmentPalette::Violet
        | EquipmentPalette::Turquoise => 1,
        _ => 2,
    }
}
fn item_resolution(rect: Rect) -> u16 {
    // Ground items can be inset inside a 64px tile. Select by their actual
    // destination size so a 56px icon never enlarges a 32px source.
    if rect.w.max(rect.h) > 32.0 { 64 } else { 32 }
}
thread_local! {
    #[cfg(debug_assertions)]
    static CREATIONS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    static ITEMS: std::cell::RefCell<ItemSources> = std::cell::RefCell::new(ItemSources {
        items: LazyImage::new(include_bytes!("../assets/sprites/cyberpunk/crawl-items-32.png")),
        flat_outfits: LazyImage::new(include_bytes!("../assets/sprites/flat-agents/outfit-icons-32.png")),
        materials: LazyImage::new(include_bytes!("../assets/sprites/cyberpunk/crawl-item-materials-32.png")),
        native: NativeItems::load(), cache: BTreeMap::new(),
    });
}
pub(super) fn prewarm() {
    ITEMS.with(|_| {});
}
pub(super) fn draw_item(rect: Rect, art: ItemArt, palette: EquipmentPalette, opacity: f32) {
    // Even compact UI slots sample the approved native icon, rather than
    // switching back to its historical small painting.
    let resolution = if super::finish::has_torso_icon(art.illustration)
        || (palette == EquipmentPalette::Original && matches!(art.illustration, 2 | 150))
    {
        64
    } else {
        item_resolution(rect)
    };
    ITEMS.with(|sources| {
        let mut sources = sources.borrow_mut();
        let key = (art.illustration, palette, resolution);
        if !sources.cache.contains_key(&key) {
            let image =
                if let Some(image) = sources.native.image(art.illustration, palette, resolution) {
                    image
                } else if let Some(outfit) = flat_outfit(art.illustration) {
                    // Unbound reference outfits and retired save content retain
                    // their historical source; current equipment uses NativeItems.
                    let variant = authored_variant(palette);
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
        crate::world_readability::texture(
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

pub(super) fn reviewed_outfit(illustration: u16) -> Option<u16> {
    flat_outfit(illustration).map(|i| 200 + i).or_else(|| {
        EXTENDED_TORSOS
            .iter()
            .find_map(|&(item, outfit)| (item == illustration).then_some(outfit))
    })
}

#[cfg(debug_assertions)]
pub(super) fn cache_stats() -> (usize, usize) {
    (
        ITEMS.with(|s| s.borrow().cache.len()),
        CREATIONS.with(|c| c.get()),
    )
}
pub(super) fn evict<K: Copy + Ord, V>(cache: &mut BTreeMap<K, (V, f64)>, limit: usize) {
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
    fn source_bank_stays_compressed_until_sampled_and_decodes_once() {
        let source = LazyImage::new(include_bytes!(
            "../assets/sprites/cyberpunk/crawl-bodies-32.png"
        ));
        assert!(source.image.get().is_none());
        let expected = decoded(source.bytes);
        assert_eq!(
            source.bytes,
            include_bytes!("../assets/sprites/cyberpunk/crawl-bodies-32.png")
        );
        let first: *const Image = &*source;
        assert_eq!((&*source).bytes, expected.bytes);
        assert!(std::ptr::eq(first, &*source));
    }
    #[test]
    fn all_current_weapon_and_armor_ids_use_the_reviewed_bank() {
        let catalog: serde_json::Value =
            serde_json::from_str(include_str!("../assets/sprites/cyberpunk/manifest.json"))
                .unwrap();
        let review: serde_json::Value = serde_json::from_str(include_str!(
            "../docs/reviews/refonte-agent-2026-10-06/equipment-native64-01/review.json"
        ))
        .unwrap();
        let bank = NativeItems::load();
        let mut checked = 0;
        for entry in catalog["items"].as_array().unwrap() {
            let id = entry["id"].as_str().unwrap();
            if review["excluded_ids"]
                .as_array()
                .unwrap()
                .iter()
                .any(|e| e == id)
                || !(entry["icon"] == 2 || held_weapon(id).is_some())
            {
                continue;
            }
            let art = item(id).unwrap();
            for resolution in [32, 64] {
                let image = bank
                    .image(art.illustration, EquipmentPalette::Original, resolution)
                    .unwrap_or_else(|| panic!("missing equipment {id} at {resolution}px"));
                assert_eq!((image.width, image.height), (resolution, resolution));
                assert!(image.bytes.chunks_exact(4).any(|p| p[3] > 0));
            }
            checked += 1;
        }
        assert_eq!(checked, 139);
        for id in [
            "core:tenue_assaut",
            "core:tenue_technique",
            "core:tenue_traque",
            "core:tenue_rituelle",
            "core:tenue_metamorphe",
        ] {
            assert!(
                bank.image(
                    item(id).unwrap().illustration,
                    EquipmentPalette::Original,
                    64
                )
                .is_some()
            );
        }
    }
    #[test]
    fn native64_equipment_never_reuses_an_enlarged_32px_frame() {
        let bank = NativeItems::load();
        for &(illustration, _, variants) in NATIVE_ITEMS {
            for palette in [
                EquipmentPalette::Original,
                EquipmentPalette::ReviewedVariant1,
                EquipmentPalette::ReviewedVariant2,
            ]
            .into_iter()
            .take(if variants { 3 } else { 1 })
            {
                let large = bank.image(illustration, palette, 64).unwrap();
                let small = bank.image(illustration, palette, 32).unwrap();
                assert!(
                    (0..64).any(|y| (0..64)
                        .any(|x| { large.get_pixel(x, y) != small.get_pixel(x / 2, y / 2) })),
                    "enlarged source for {illustration}/{palette:?}"
                );
            }
        }
        assert_eq!(item_resolution(Rect::new(0.0, 0.0, 56.0, 56.0)), 64);
        assert_eq!(item_resolution(Rect::new(0.0, 0.0, 64.0, 64.0)), 64);
        assert_eq!(item_resolution(Rect::new(0.0, 0.0, 128.0, 128.0)), 64);
        assert_eq!(item_resolution(Rect::new(0.0, 0.0, 32.0, 32.0)), 32);
    }
    #[test]
    fn chrysalide_icons_use_the_existing_olive_palette_as_the_default() {
        let bank = NativeItems::load();
        let illustration = item("core:carapace_chrysalide").unwrap().illustration;
        for side in [32, 64] {
            let default = bank
                .image(illustration, EquipmentPalette::Original, side)
                .unwrap();
            let olive = bank
                .image(illustration, EquipmentPalette::ReviewedVariant1, side)
                .unwrap();
            let red = bank
                .image(illustration, EquipmentPalette::ReviewedVariant2, side)
                .unwrap();
            assert_eq!(default.bytes, olive.bytes);
            assert_ne!(default.bytes, red.bytes);
        }
    }
    #[test]
    fn equipment_repainting_preserves_transparency_and_protected_materials() {
        let bank = NativeItems::load();
        for &(illustration, index, variants) in NATIVE_ITEMS.iter().filter(|(_, _, v)| !v) {
            assert!(!variants);
            // The pilot revises only the original icon. Other equipment paints
            // retain their separately accepted source, checked against that bank.
            let revised =
                super::super::finish::item_image(illustration, EquipmentPalette::Original);
            let original = bank
                .image(illustration, EquipmentPalette::Original, 64)
                .unwrap();
            let mask = cell(&bank.large_maps, index, 64);
            for palette in EquipmentPalette::ALL {
                let image = bank.image(illustration, palette, 64).unwrap();
                if (101..=114).contains(&illustration) {
                    let material =
                        super::super::finish::image(&format!("TOR-{illustration}-material"))
                            .unwrap();
                    assert!(
                        original
                            .bytes
                            .chunks_exact(4)
                            .zip(image.bytes.chunks_exact(4))
                            .zip(material.bytes.chunks_exact(4))
                            .all(|((a, b), m)| a[3] == b[3] && (m[0] == 1 || a == b)),
                        "torso material {illustration}/{palette:?}"
                    );
                    continue;
                }
                if let Some(ref accepted) = revised {
                    if palette == EquipmentPalette::Original {
                        assert_eq!(image.bytes, accepted.bytes);
                    } else {
                        let mut unchanged = cell(&bank.large, index, 64);
                        recolor(&mut unchanged, &mask, palette);
                        assert_eq!(
                            image.bytes, unchanged.bytes,
                            "unreviewed paint changed: {illustration}/{palette:?}"
                        );
                    }
                    continue;
                }
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
    }
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
