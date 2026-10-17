//! Accepted frontal outfits, with independent model-specific held layers.
use super::*;
include!("game_art_flat_catalog.rs");
#[path = "game_art_held.rs"]
mod revised_hands;

pub(super) struct FlatAgents {
    held: revised_hands::HeldPoses,
    reviewed: Image,
    reviewed64: Image,
    bodies: Image,
    fingers: Image,
    weapons: Image,
    extended_reviewed: Image,
    extended_reviewed64: Image,
    extended_bodies: Image,
    extended_fingers: Image,
}
impl FlatAgents {
    pub(super) fn load() -> Self {
        Self {
            held: revised_hands::HeldPoses::load(),
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
            extended_reviewed: palettes::decoded(include_bytes!(
                "../assets/sprites/equipment-native64/outfits-64.png"
            )),
            extended_reviewed64: palettes::decoded(include_bytes!(
                "../assets/sprites/equipment-native64/outfits-128.png"
            )),
            extended_bodies: palettes::decoded(include_bytes!(
                "../assets/sprites/equipment-native64/bodies-128.png"
            )),
            extended_fingers: palettes::decoded(include_bytes!(
                "../assets/sprites/equipment-native64/fingers-128.png"
            )),
        }
    }
    fn variant(palette: EquipmentPalette) -> u16 {
        palettes::authored_variant(palette)
    }
    pub(super) fn compose_pose(&self, key: Key, resolution: u16, breath: u8) -> Option<Image> {
        let still = self.compose(key, resolution)?;
        if breath == 0 {
            return Some(still);
        }
        let (body, layers, colors) = key;
        let outfit = body - 200;
        let (outfit, bodies, fingers) = if outfit < 12 {
            (outfit, &self.bodies, &self.fingers)
        } else {
            (outfit - 12, &self.extended_bodies, &self.extended_fingers)
        };
        let base = (outfit * 3 + Self::variant(colors[0])) * 4;
        let mut lower = palettes::cell(bodies, base, 128);
        let mut foreground = Image::gen_image_color(128, 128, BLANK);
        let held: Vec<_> = layers.into_iter().flatten().collect();
        if let [weapon] = held.as_slice()
            && let Some((index, grip)) = revised_hands::HeldPoses::grip(weapon.model)
        {
            let pose = if grip.opposed { 3 } else { 2 };
            foreground = self.held.compose(
                index,
                grip,
                &foreground,
                &palettes::cell(bodies, base + 2, 128),
                &[
                    palettes::cell(fingers, (base + pose) * 2, 128),
                    palettes::cell(fingers, (base + pose) * 2 + 1, 128),
                ],
            );
        } else {
            let held: Vec<_> = held
                .into_iter()
                .filter_map(|layer| {
                    COMIC_WEAPONS
                        .iter()
                        .find(|(model, left, _, _)| *model == layer.model && *left == layer.left)
                        .or_else(|| {
                            COMIC_WEAPONS
                                .iter()
                                .find(|(model, _, pose, _)| *model == layer.model && *pose != 0)
                        })
                        .map(|&(_, _, pose, index)| (layer, pose, index))
                })
                .collect();
            let pose = held.iter().map(|(_, pose, _)| *pose).max().unwrap_or(0);
            for &(_, _, index) in &held {
                alpha_over(&mut foreground, &palettes::cell(&self.weapons, index, 128));
            }
            for side in 0..2 {
                if held
                    .iter()
                    .any(|(layer, pose, _)| *pose != 0 || usize::from(!layer.left) == side)
                {
                    alpha_over(
                        &mut foreground,
                        &palettes::cell(fingers, (base + pose) * 2 + side as u16, 128),
                    );
                }
            }
        }
        if resolution == 32 {
            lower = downsample(&lower);
            foreground = downsample(&foreground);
        }
        Some(breathe(&still, &lower, &foreground, resolution))
    }
    pub(super) fn compose(&self, key: Key, resolution: u16) -> Option<Image> {
        let (body, layers, colors) = key;
        let outfit = body
            .checked_sub(200)
            .filter(|i| *i < palettes::FLAT_OUTFIT_COUNT)?;
        let (outfit, reviewed, reviewed64, bodies, fingers) = if outfit < 12 {
            (
                outfit,
                &self.reviewed,
                &self.reviewed64,
                &self.bodies,
                &self.fingers,
            )
        } else {
            (
                outfit - 12,
                &self.extended_reviewed,
                &self.extended_reviewed64,
                &self.extended_bodies,
                &self.extended_fingers,
            )
        };
        let variant = Self::variant(colors[0]);
        let selected: Vec<_> = layers
            .into_iter()
            .flatten()
            .map(|l| (l.model, l.left))
            .collect();
        if let [(model, _)] = selected.as_slice()
            && let Some((layer, grip)) = revised_hands::HeldPoses::grip(*model)
        {
            let base = (outfit * 3 + variant) * 4;
            let pose = if grip.opposed { 3 } else { 2 };
            let result = self.held.compose(
                layer,
                grip,
                &palettes::cell(bodies, base, 128),
                &palettes::cell(bodies, base + 2, 128),
                &[
                    palettes::cell(fingers, (base + pose) * 2, 128),
                    palettes::cell(fingers, (base + pose) * 2 + 1, 128),
                ],
            );
            return Some(if resolution == 64 {
                result
            } else {
                downsample(&result)
            });
        }
        if colors[1..].iter().all(|p| *p == EquipmentPalette::Original)
            && let Some(config) = COMIC_CONFIGS
                .iter()
                .position(|items| *items == selected.as_slice())
        {
            let (sheet, canvas) = if resolution == 64 {
                (reviewed64, 128)
            } else {
                (reviewed, 64)
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
        let mut result = palettes::cell(bodies, index, 128);
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
                    &palettes::cell(fingers, index * 2 + side as u16, 128),
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
fn breathe(still: &Image, lower: &Image, foreground: &Image, resolution: u16) -> Image {
    let canvas = usize::from(still.width);
    let waist = canvas / 4 + usize::from(resolution) * 23 / 32;
    let shift = usize::from(resolution) / 32;
    let mut result = still.clone();
    let mut moving = Image::gen_image_color(still.width, still.height, BLANK);
    for y in 0..canvas {
        for x in 0..canvas {
            let i = (y * canvas + x) * 4;
            let held = foreground.bytes[i + 3] > 0;
            if y < waist {
                result.bytes[i..i + 4].fill(0);
            } else if held {
                result.bytes[i..i + 4].copy_from_slice(&lower.bytes[i..i + 4]);
            }
            let next = y + shift;
            if (y < waist || held) && next < canvas && (next < waist || held) {
                let dest = (next * canvas + x) * 4;
                moving.bytes[dest..dest + 4].copy_from_slice(&still.bytes[i..i + 4]);
            }
        }
    }
    alpha_over(&mut result, &moving);
    result
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
    fn every_outfit_breathes_with_fixed_feet_in_each_palette_and_resolution() {
        let bank = FlatAgents::load();
        for outfit in 200..219 {
            for palette in [
                EquipmentPalette::Original,
                EquipmentPalette::ReviewedVariant1,
                EquipmentPalette::ReviewedVariant2,
            ] {
                for resolution in [32, 64] {
                    let key = (
                        outfit,
                        [None, None],
                        [
                            palette,
                            EquipmentPalette::Original,
                            EquipmentPalette::Original,
                        ],
                    );
                    let still = bank.compose_pose(key, resolution, 0).unwrap();
                    let breath = bank.compose_pose(key, resolution, 1).unwrap();
                    let canvas = still.width as usize;
                    let waist = canvas / 4 + resolution as usize * 23 / 32;
                    let shift = resolution as usize / 32;
                    assert_eq!(
                        &still.bytes[waist * canvas * 4..],
                        &breath.bytes[waist * canvas * 4..],
                        "feet changed: {outfit}/{palette:?}/{resolution}"
                    );
                    assert_ne!(still.bytes, breath.bytes, "outfit does not breathe");
                    for y in 0..waist - shift {
                        for x in 0..canvas {
                            let src = (y * canvas + x) * 4;
                            let dst = ((y + shift) * canvas + x) * 4;
                            if still.bytes[src + 3] > 0 {
                                assert_eq!(&still.bytes[src..src + 4], &breath.bytes[dst..dst + 4]);
                            }
                        }
                    }
                }
            }
        }
    }
    #[test]
    fn breathing_moves_a_foreground_part_below_the_waist_without_dragging_the_legs() {
        let mut lower = Image::gen_image_color(128, 128, BLANK);
        lower.set_pixel(60, 84, BLUE);
        lower.set_pixel(60, 86, BLUE);
        lower.set_pixel(60, 93, WHITE);
        let mut still = lower.clone();
        still.set_pixel(60, 84, RED);
        let mut foreground = Image::gen_image_color(128, 128, BLANK);
        foreground.set_pixel(60, 84, RED);
        let breath = breathe(&still, &lower, &foreground, 64);
        assert_eq!(breath.get_pixel(60, 84), lower.get_pixel(60, 84));
        assert_eq!(breath.get_pixel(60, 86), still.get_pixel(60, 84));
        assert_eq!(breath.get_pixel(60, 93), lower.get_pixel(60, 93));
    }
    #[test]
    fn revised_heavy_weapons_and_axe_match_the_approved_pixels() {
        let bank = FlatAgents::load();
        let approved = palettes::decoded(include_bytes!(
            "../assets/source-art/refonte-agent-2026-10-06/held-weapon-review-01/approved-128.png"
        ));
        for (weapon_index, weapon) in [31, 23, 15, 63].into_iter().enumerate() {
            for (outfit_index, outfit) in [212, 203, 213].into_iter().enumerate() {
                let key = (
                    outfit,
                    [None, layer(weapon, false)],
                    [EquipmentPalette::Original; 3],
                );
                let image = bank.compose(key, 64).unwrap();
                let expected =
                    palettes::cell(&approved, (weapon_index * 3 + outfit_index) as u16, 128);
                assert_eq!(
                    image.bytes, expected.bytes,
                    "model {weapon} / outfit {outfit}"
                );
                assert_eq!(
                    bank.compose(key, 32).unwrap().bytes,
                    downsample(&expected).bytes
                );
            }
        }
    }
    #[test]
    fn revised_grips_target_only_heavy_firearms_and_two_handed_axes() {
        let manifest: serde_json::Value = serde_json::from_slice(include_bytes!(
            "../assets/sprites/held-native64/manifest.json"
        ))
        .unwrap();
        assert_eq!(manifest["entries"].as_array().unwrap().len(), 34);
        for entry in manifest["entries"].as_array().unwrap() {
            let model = entry["model"].as_u64().unwrap() as u16;
            let (_, grip) = revised_hands::HeldPoses::grip(model).unwrap();
            assert!(matches!(
                weapon_hold(model),
                WeaponHold::TwoHanded | WeaponHold::MeleeTwoHanded
            ));
            assert_eq!(grip.opposed, entry["opposed"].as_bool().unwrap());
            assert_eq!(grip.opposed, WEAPON_FAMILIES[model as usize] == 3);
        }
        for model in 0..HELD_WEAPON_COUNT {
            if matches!(WEAPON_FAMILIES[model as usize], 6 | 7 | 8 | 9 | 11 | 17)
                || matches!(weapon_hold(model), WeaponHold::Side | WeaponHold::Raised)
            {
                assert!(
                    revised_hands::HeldPoses::grip(model).is_none(),
                    "ordinary firearm or one-handed model {model}"
                );
            }
        }
    }
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
        for outfit in [200, 203, 207, 211, 212, 213, 214, 215, 216, 217, 218] {
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
    #[test]
    fn extended_outfits_copy_all_approved_configurations_and_palettes_exactly() {
        let bank = FlatAgents::load();
        for outfit in 0..7 {
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
                    let image = bank.compose((212 + outfit, held, colors), 64).unwrap();
                    let expected = palettes::cell(
                        &bank.extended_reviewed64,
                        (outfit * 3 + variant) * 16 + config as u16,
                        128,
                    );
                    assert_eq!(image.bytes, expected.bytes, "{outfit}/{variant}/{config}");
                }
            }
        }
    }
    #[test]
    fn chrysalide_default_keeps_olive_colours_for_presets_and_independent_layers() {
        let bank = FlatAgents::load();
        for layers in [
            [None, None],
            [
                None,
                layer(held_weapon("core:modele_brenek_a80").unwrap(), false),
            ],
            selection(
                None,
                Some([
                    held_weapon("core:melee_epee_courte"),
                    held_weapon("core:modele_cogneur_p01"),
                ]),
            ),
        ] {
            for resolution in [32, 64] {
                let default = bank
                    .compose((213, layers, [EquipmentPalette::Original; 3]), resolution)
                    .unwrap();
                let olive = bank
                    .compose(
                        (
                            213,
                            layers,
                            [
                                EquipmentPalette::ReviewedVariant1,
                                EquipmentPalette::Original,
                                EquipmentPalette::Original,
                            ],
                        ),
                        resolution,
                    )
                    .unwrap();
                assert_eq!(default.bytes, olive.bytes);
            }
        }
    }
}
