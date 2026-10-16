//! Accepted frontal outfits, with independent model-specific held layers.
use super::*;
include!("game_art_flat_catalog.rs");

pub(super) struct FlatAgents {
    reviewed: Image,
    reviewed64: Image,
    bodies: Image,
    fingers: Image,
    weapons: Image,
}
impl FlatAgents {
    pub(super) fn load() -> Self {
        Self {
            reviewed: palettes::decoded(include_bytes!(
                "../assets/sprites/flat-agents/reviewed-64.png"
            )),
            reviewed64: palettes::decoded(include_bytes!(
                "../assets/sprites/flat-agents/reviewed-128.png"
            )),
            bodies: palettes::decoded(include_bytes!(
                "../assets/sprites/flat-agents/bodies-128.png"
            )),
            fingers: palettes::decoded(include_bytes!(
                "../assets/sprites/flat-agents/fingers-128.png"
            )),
            weapons: palettes::decoded(include_bytes!(
                "../assets/sprites/flat-agents/weapons-128.png"
            )),
        }
    }
    fn variant(palette: EquipmentPalette) -> u16 {
        match palette {
            EquipmentPalette::Original => 0,
            EquipmentPalette::ReviewedVariant1
            | EquipmentPalette::Blue
            | EquipmentPalette::Violet
            | EquipmentPalette::Turquoise => 1,
            EquipmentPalette::ReviewedVariant2 => 2,
            _ => 2,
        }
    }
    pub(super) fn compose(&self, key: Key, resolution: u16) -> Option<Image> {
        let (body, layers, colors) = key;
        let outfit = body.checked_sub(200).filter(|i| *i < 12)?;
        let variant = Self::variant(colors[0]);
        let selected: Vec<_> = layers
            .into_iter()
            .flatten()
            .map(|l| (l.model, l.left))
            .collect();
        if colors[1..].iter().all(|p| *p == EquipmentPalette::Original)
            && let Some(config) = COMIC_CONFIGS
                .iter()
                .position(|items| *items == selected.as_slice())
        {
            let (sheet, canvas) = if resolution == 64 {
                (&self.reviewed64, 128)
            } else {
                (&self.reviewed, 64)
            };
            return Some(palettes::cell(
                sheet,
                (outfit * 3 + variant) * 16 + config as u16,
                canvas,
            ));
        }
        let held: Vec<_> = layers
            .into_iter()
            .flatten()
            .filter_map(|l| {
                COMIC_WEAPONS
                    .iter()
                    .find(|(model, left, _, _)| *model == l.model && *left == l.left)
                    .or_else(|| {
                        COMIC_WEAPONS
                            .iter()
                            .find(|(model, _, pose, _)| *model == l.model && *pose != 0)
                    })
                    .map(|&(_, _, pose, index)| (l, pose, index))
            })
            .collect();
        let pose = held.iter().map(|(_, p, _)| *p).max().unwrap_or(0);
        let index = (outfit * 3 + variant) * 4 + pose;
        let mut result = palettes::cell(&self.bodies, index, 128);
        for &(_, _, index) in &held {
            alpha_over(&mut result, &palettes::cell(&self.weapons, index, 128));
        }
        for side in 0..2 {
            if held
                .iter()
                .any(|(l, p, _)| *p != 0 || usize::from(!l.left) == side)
            {
                alpha_over(
                    &mut result,
                    &palettes::cell(&self.fingers, index * 2 + side as u16, 128),
                );
            }
        }
        // Keep authored 64px detail for large tiles; the canvas spans two tiles.
        Some(if resolution == 64 {
            result
        } else {
            downsample(&result)
        })
    }
}
fn alpha_over(dest: &mut Image, source: &Image) {
    for (d, s) in dest
        .bytes
        .chunks_exact_mut(4)
        .zip(source.bytes.chunks_exact(4))
    {
        let a = u32::from(s[3]);
        let old = u32::from(d[3]) * (255 - a);
        let total = a * 255 + old;
        if total == 0 {
            continue;
        }
        for c in 0..3 {
            d[c] = ((u32::from(s[c]) * a * 255 + u32::from(d[c]) * old + total / 2) / total) as u8;
        }
        d[3] = ((total + 127) / 255) as u8;
    }
}
fn downsample(source: &Image) -> Image {
    let mut dest = Image::gen_image_color(64, 64, BLANK);
    for y in 0..64 {
        for x in 0..64 {
            let mut sum = [0u32; 4];
            for dy in 0..2 {
                for dx in 0..2 {
                    let i = ((y * 2 + dy) * 128 + x * 2 + dx) * 4;
                    let a = u32::from(source.bytes[i + 3]);
                    sum[3] += a;
                    for c in 0..3 {
                        sum[c] += u32::from(source.bytes[i + c]) * a;
                    }
                }
            }
            let i = (y * 64 + x) * 4;
            if sum[3] > 0 {
                for c in 0..3 {
                    dest.bytes[i + c] = ((sum[c] + sum[3] / 2) / sum[3]) as u8;
                }
            }
            dest.bytes[i + 3] = ((sum[3] + 2) / 4) as u8;
        }
    }
    dest
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepted_compositions_are_copied_exactly_for_every_outfit_and_palette() {
        let bank = FlatAgents::load();
        for outfit in 0..12 {
            for variant in 0..3 {
                for (config, models) in COMIC_CONFIGS.iter().enumerate() {
                    if models.iter().any(|(m, _)| *m == u16::MAX) {
                        continue;
                    }
                    let mut held = [None, None];
                    for &(model, left) in *models {
                        held[usize::from(!left)] = Some(layer(model, left).unwrap());
                    }
                    let colors = [
                        match variant {
                            0 => EquipmentPalette::Original,
                            1 => EquipmentPalette::ReviewedVariant1,
                            _ => EquipmentPalette::ReviewedVariant2,
                        },
                        EquipmentPalette::Original,
                        EquipmentPalette::Original,
                    ];
                    let actual = bank.compose((200 + outfit, held, colors), 32).unwrap();
                    let expected = palettes::cell(
                        &bank.reviewed,
                        (outfit * 3 + variant) * 16 + config as u16,
                        64,
                    );
                    assert_eq!(actual.bytes, expected.bytes);
                    let direct = bank.compose((200 + outfit, held, colors), 64).unwrap();
                    let expected64 = palettes::cell(
                        &bank.reviewed64,
                        (outfit * 3 + variant) * 16 + config as u16,
                        128,
                    );
                    assert_eq!((direct.width, direct.height), (128, 128));
                    assert_eq!(direct.bytes, expected64.bytes);
                    assert_ne!(direct.bytes, nearest_double(&actual).bytes);
                }
            }
        }
    }
    fn nearest_double(source: &Image) -> Image {
        let mut result = Image::gen_image_color(128, 128, BLANK);
        for y in 0..128 {
            for x in 0..128 {
                result.set_pixel(x, y, source.get_pixel(x / 2, y / 2));
            }
        }
        result
    }
    #[test]
    fn every_existing_model_has_its_own_layer_at_the_common_grip() {
        for model in 0..WEAPON_FAMILIES.len() as u16 {
            assert!(
                COMIC_WEAPONS.iter().any(|(m, _, _, _)| *m == model),
                "missing model {model}"
            );
        }
    }
    #[test]
    fn non_preset_weapons_keep_detail_and_placement_at_both_resolutions() {
        let bank = FlatAgents::load();
        for outfit in [200, 203, 207, 211] {
            for weapon in 0..WEAPON_FAMILIES.len() as u16 {
                // A non-original weapon palette takes the independent-layer path.
                let colors = [
                    EquipmentPalette::Original,
                    EquipmentPalette::Blue,
                    EquipmentPalette::Original,
                ];
                let key = (outfit, [None, layer(weapon, false)], colors);
                let small = bank.compose(key, 32).unwrap();
                let large = bank.compose(key, 64).unwrap();
                assert_eq!((small.width, small.height), (64, 64));
                assert_eq!((large.width, large.height), (128, 128));
                assert_eq!(small.bytes, downsample(&large).bytes);
            }
        }
    }
}
