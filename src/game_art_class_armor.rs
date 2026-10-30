//! Accepted class-specific native64 outfits, source limbs and unchanged held weapons.
use super::super::held_geometry as geometry;
use super::generic;
use super::*;
#[path = "game_art_weapon_readability.rs"]
mod readable;

const BASE: u16 = 512;
const HEAD_STRIDE: u16 = 32;
struct ArmorRig {
    palms: [[f32; 2]; 2],
    pivots: [[f32; 2]; 2],
    head_center: [f32; 2],
    raised_palms: [[f32; 2]; 2],
    corrected_arms: bool,
}
include!("game_art_class_armor_catalog.rs");
#[cfg(any(debug_assertions, test))]
#[path = "game_art_dual_review17.rs"]
mod dual_review17;
#[path = "game_art_walk_rig.rs"]
mod walk_rig;

pub(super) fn walking_layers(key: Key, pose: crate::game_art::walking::Pose) -> Option<Vec<Image>> {
    walk_rig::layers(key, pose)
}
pub(super) fn walking_npc(key: Key) -> bool {
    crate::game_art::walking::active() && (17..=22).contains(&key.0)
}

pub(super) fn outfit_index(body: u8, equipment: [Option<u16>; 4]) -> Option<u16> {
    let class = body.checked_sub(3).filter(|v| *v < 5)?;
    // Physical item models select a coverage stage; bonuses and rarity never
    // change an item's silhouette or any gameplay property.
    let rank = match equipment[0] {
        None | Some(108 | 112 | 113 | 200..=204) => 0,
        Some(110 | 114) => 1,
        Some(102 | 105 | 106 | 111) => 2,
        Some(101 | 104 | 107 | 109) => 3,
        Some(103) => 4,
        _ => 0,
    };
    let head = equipment[1]
        .filter(|v| (115..=120).contains(v))
        .map_or(0, |v| v - 114);
    Some(BASE + head * HEAD_STRIDE + u16::from(class) * 5 + rank)
}

fn decode(outfit: u16) -> Option<(usize, usize)> {
    let value = outfit.checked_sub(BASE)?;
    let index = value % HEAD_STRIDE;
    let helmet = value / HEAD_STRIDE;
    (index < 25 && helmet <= 6).then_some((usize::from(index), usize::from(helmet)))
}

thread_local! {
    static BANKS: BTreeMap<&'static str, palettes::LazyImage> = [
        "base", "core", "arm-0", "arm-1", "fingers-0", "fingers-1",
        "grip-0", "grip-1", "hand-mask-0", "hand-mask-1", "head-mask",
        "ranged-core", "ranged-arm-0", "ranged-arm-1",
        "helmets", "helmet-openings", "identity-heads",
        "raised-arm-0", "raised-arm-1", "raised-erase-0", "raised-erase-1",
        "raised-fingers-0", "raised-fingers-1",
    ].into_iter().map(|p| (p, palettes::LazyImage::new(armor_bank_bytes(p)))).collect();
}

fn part(index: usize, name: &str) -> Image {
    BANKS.with(|b| palettes::cell(&b[name], index as u16, 128))
}

fn attachment(model: u16, left: bool) -> Option<([f32; 2], Option<[f32; 2]>)> {
    if let Some(weapon) = readable::binding(model) {
        return Some((weapon.anchor, Some(weapon.support)));
    }
    let (mut anchor, mut support) = generic::attachment(model, left)?;
    if model == 66 {
        // This axe's shaft is painted at y=60, three pixels above the
        // catalogue's old fist-centre coordinates. Bind to the shaft itself.
        anchor[1] -= 3.;
        support.as_mut().unwrap()[1] -= 3.;
    }
    // These six upright swords retain the old generic pole support point,
    // which lies beside the blade. Both hands belong on the actual hilt.
    let support = if (88..=93).contains(&model) {
        Some([anchor[0], anchor[1] - 4.])
    } else {
        support
    };
    Some((anchor, support))
}

fn weapon_image(model: u16, left: bool, color: EquipmentPalette) -> Option<Image> {
    if readable::binding(model).is_some() {
        return readable::image(model, color);
    }
    let source = generic::weapon_image(model, left, color)?;
    if !matches!(model, 68 | 79) {
        return Some(source);
    }
    // These two single-edged sources put the sharpened bevel towards the
    // torso. Turn the weapon around its handle, after coloring, in either
    // hand. Reflecting the whole canvas would move it to the opposite hand.
    let axis = attachment(model, left)?.0[0].round() as i32;
    let mut corrected = Image::gen_image_color(128, 128, BLANK);
    for y in 0..128usize {
        for x in 0..128usize {
            let to_x = 2 * axis - x as i32;
            if (0..128).contains(&to_x) {
                let from = (y * 128 + x) * 4;
                let to = (y * 128 + to_x as usize) * 4;
                corrected.bytes[to..to + 4].copy_from_slice(&source.bytes[from..from + 4]);
            }
        }
    }
    Some(corrected)
}

#[cfg(all(debug_assertions, test))]
pub(super) fn vertical_review_weapon(
    model: u16,
    left: bool,
    color: EquipmentPalette,
) -> Option<(Image, [f32; 2], Option<[f32; 2]>)> {
    let (anchor, support) = attachment(model, left)?;
    Some((weapon_image(model, left, color)?, anchor, support))
}

#[cfg(debug_assertions)]
pub(super) fn vertical_review_composition(
    source: &Image,
    palms: [[f32; 2]; 2],
    back: bool,
    held: [Option<HeldLayer>; 2],
    colors: [EquipmentPalette; 3],
) -> Option<Image> {
    let mut body = source.clone();
    let mut weapons = Image::gen_image_color(128, 128, BLANK);
    let mut fingers = weapons.clone();
    for layer in held.into_iter().flatten() {
        if !matches!(
            weapon_hold(layer.model),
            WeaponHold::Side | WeaponHold::Raised
        ) {
            return None;
        }
        let logical_side = usize::from(!layer.left);
        let screen_side = if back { 1 - logical_side } else { logical_side };
        let palm = palms[screen_side];
        // The hand remains an exact part of this authored pose. Only its
        // foreground ownership changes; no wrist or sleeve is moved alone.
        for y in palm[1].round() as usize - 2..=palm[1].round() as usize + 2 {
            for x in palm[0].round() as usize - 2..=palm[0].round() as usize + 2 {
                let p = (y * 128 + x) * 4;
                fingers.bytes[p..p + 4].copy_from_slice(&source.bytes[p..p + 4]);
                body.bytes[p..p + 4].fill(0);
            }
        }
        let left = screen_side == 0;
        let (anchor, _) = attachment(layer.model, left)?;
        let weapon = weapon_image(layer.model, left, colors[1 + usize::from(layer.left)])?;
        let angle = if matches!(weapon_hold(layer.model), WeaponHold::Raised) {
            if left {
                -std::f32::consts::FRAC_PI_2
            } else {
                std::f32::consts::FRAC_PI_2
            }
        } else {
            0.
        };
        let posed = geometry::transformed(&weapon, anchor, palm, 1., angle);
        let mut contact = false;
        for y in palm[1].round() as usize - 2..=palm[1].round() as usize + 2 {
            for x in palm[0].round() as usize - 2..=palm[0].round() as usize + 2 {
                if fingers.bytes[(y * 128 + x) * 4 + 3] >= 32
                    && (y - 1..=y + 1).any(|yy| {
                        (x - 1..=x + 1).any(|xx| posed.bytes[(yy * 128 + xx) * 4 + 3] >= 32)
                    })
                {
                    contact = true;
                }
            }
        }
        if !contact {
            return None;
        }
        geometry::over(&mut weapons, &posed);
    }
    geometry::over(&mut body, &weapons);
    geometry::over(&mut body, &fingers);
    Some(body)
}

pub(super) fn supports(key: Key) -> bool {
    decode(key.0).is_some()
        && key
            .1
            .into_iter()
            .flatten()
            .all(|l| attachment(l.model, l.left).is_some())
}

fn moved(image: &Image, dx: i32, dy: i32) -> Image {
    let mut out = Image::gen_image_color(128, 128, BLANK);
    for y in 0..128i32 {
        for x in 0..128i32 {
            let (xx, yy) = (x + dx, y + dy);
            if (0..128).contains(&xx) && (0..128).contains(&yy) {
                let from = (y as usize * 128 + x as usize) * 4;
                let to = (yy as usize * 128 + xx as usize) * 4;
                out.bytes[to..to + 4].copy_from_slice(&image.bytes[from..from + 4]);
            }
        }
    }
    out
}

fn helmet(body: &mut Image, index: usize, head: usize) {
    if head == 0 {
        return;
    }
    let rig = &ARMOR_RIGS[index];
    let dy = (rig.head_center[1] - 40.).round() as i32;
    let cover = moved(&part(head - 1, "helmets"), 0, dy);
    let opening = moved(&part(head - 1, "helmet-openings"), 0, dy);
    let mask = part(index, "head-mask");
    for (pixel, m) in body
        .bytes
        .chunks_exact_mut(4)
        .zip(mask.bytes.chunks_exact(4))
    {
        if m[3] > 0 {
            pixel.fill(0);
        }
    }
    if head <= 2 {
        // The canonical uncovered class head supplies every visible face, even
        // when the torso's default head has a full helmet or an opaque mask.
        let identity = part(index / 5, "identity-heads");
        let top = ARMOR_RIGS[index / 5 * 5].head_center[1];
        let identity = moved(&identity, 0, (rig.head_center[1] - top).round() as i32);
        for ((pixel, src), hole) in body
            .bytes
            .chunks_exact_mut(4)
            .zip(identity.bytes.chunks_exact(4))
            .zip(opening.bytes.chunks_exact(4))
        {
            if hole[3] > 0 {
                pixel.copy_from_slice(src);
            }
        }
    }
    geometry::over(body, &cover);
}

fn pose(image: &Image, rig: &ArmorRig, side: usize, target: [f32; 2]) -> Image {
    pose_limb(image, rig.pivots[side], rig.palms[side], target)
}

fn pose_limb(image: &Image, pivot: [f32; 2], palm: [f32; 2], target: [f32; 2]) -> Image {
    geometry::affine(image, pivot, pivot, limb_matrix(pivot, palm, target))
}
fn limb_matrix(pivot: [f32; 2], palm: [f32; 2], target: [f32; 2]) -> [f32; 4] {
    let old = [palm[0] - pivot[0], palm[1] - pivot[1]];
    let new = [target[0] - pivot[0], target[1] - pivot[1]];
    let length = old[0].hypot(old[1]);
    let new_length = new[0].hypot(new[1]);
    let (ux, uy) = (old[0] / length, old[1] / length);
    let (vx, vy) = (new[0] / new_length, new[1] / new_length);
    let stretch = new_length / length;
    // Change reach along the limb, while keeping the original sleeve width.
    [
        stretch * vx * ux + vy * uy,
        stretch * vx * uy - vy * ux,
        stretch * vy * ux - vx * uy,
        stretch * vy * uy + vx * ux,
    ]
}

fn raised_palm(rig: &ArmorRig, side: usize) -> [f32; 2] {
    rig.raised_palms[side]
}

fn raised_forearm(index: usize, side: usize) -> (Image, [f32; 2]) {
    let rig = &ARMOR_RIGS[index];
    let mut arm = part(index, if side == 0 { "arm-0" } else { "arm-1" });
    let mut elbow = rig.pivots[side];
    if index / 5 == 3 {
        // The Arcanist's rifle rig owns the whole wide sleeve. For a bent
        // pistol arm retain the upper sleeve and move only its forearm.
        let palm = rig.palms[side];
        elbow = [palm[0] + if side == 0 { 4. } else { -4. }, palm[1] - 5.];
        let along = [palm[0] - elbow[0], palm[1] - elbow[1]];
        for (pixel, rgba) in arm.bytes.chunks_exact_mut(4).enumerate() {
            let x = (pixel % 128) as f32 + 0.5 - elbow[0];
            let y = (pixel / 128) as f32 + 0.5 - elbow[1];
            if x * along[0] + y * along[1] < -3. {
                rgba.fill(0);
            }
        }
    }
    (arm, elbow)
}

fn erase_owned(body: &mut Image, mask: &Image) {
    for (pixel, owned) in body
        .bytes
        .chunks_exact_mut(4)
        .zip(mask.bytes.chunks_exact(4))
    {
        if owned[3] > 0 {
            pixel.fill(0);
        }
    }
}

fn two_hand_offset(rig: &ArmorRig, anchor: [f32; 2], support: [f32; 2]) -> [i32; 2] {
    [
        ((rig.palms[0][0] + rig.palms[1][0] - anchor[0] - support[0]) / 2.).round() as i32,
        ((rig.palms[0][1] + rig.palms[1][1] - anchor[1] - support[1]) / 2. - 3.).round() as i32,
    ]
}

fn ranged_scale(index: usize, model: u16) -> f32 {
    if matches!(weapon_hold(model), WeaponHold::TwoHanded) {
        // Same weapon painting and proportions for every class; only physical
        // scale changes with its frame. Armor stages never change body height.
        [0.78, 0.70, 0.64, 0.68, 0.72][index / 5]
    } else {
        1.
    }
}
fn two_hand_targets(
    index: usize,
    model: u16,
    anchor: [f32; 2],
    support: [f32; 2],
) -> ([[f32; 2]; 2], f32) {
    if let Some(weapon) = readable::binding(model) {
        return weapon.targets(index);
    }
    let rig = &ARMOR_RIGS[index];
    let scale = ranged_scale(index, model);
    let offset = two_hand_offset(rig, anchor, support);
    let center = [(anchor[0] + support[0]) / 2., (anchor[1] + support[1]) / 2.];
    (
        [anchor, support].map(|p| {
            [
                center[0] + offset[0] as f32 + (p[0] - center[0]) * scale,
                center[1] + offset[1] as f32 + (p[1] - center[1]) * scale,
            ]
        }),
        scale,
    )
}

pub(super) fn render_layers(key: Key) -> Option<Vec<Image>> {
    let mut layers = if let Some(pose) = crate::game_art::walking::pose() {
        walking_layers(key, pose)?
    } else {
        layers(key)?
    };
    if let [Some(held), None] | [None, Some(held)] = key.1
        && readable::binding(held.model).is_some()
    {
        // Keep the contact shadow separate from both the painted body and
        // weapon. Alpha rounding before actor lighting otherwise changes a
        // few edge highlights compared with the approved four-layer review.
        let shadow = readable::contact_shadow(&layers[0], &layers[1]);
        layers.insert(1, shadow);
    }
    Some(layers)
}

pub(super) fn layers(key: Key) -> Option<Vec<Image>> {
    #[cfg(any(debug_assertions, test))]
    if crate::game_art::walking::dual17_enabled()
        && key.1.iter().all(|held| {
            held.is_some_and(|held| {
                matches!(
                    weapon_hold(held.model),
                    WeaponHold::Side | WeaponHold::Raised
                )
            })
        })
    {
        return dual_review17::front(key);
    }
    if !supports(key) {
        return None;
    }
    let (index, head) = decode(key.0)?;
    let rig = &ARMOR_RIGS[index];
    let held: Vec<_> = key.1.into_iter().flatten().collect();
    if let [layer] = held.as_slice()
        && readable::binding(layer.model).is_some()
    {
        return readable::layers(index, head, layer.model, key.2[1 + usize::from(layer.left)]);
    }
    let mut body = part(index, "base");
    let mut weapons = Image::gen_image_color(128, 128, BLANK);
    let mut fingers = weapons.clone();
    if let [layer] = held.as_slice()
        && let (anchor, Some(support)) = attachment(layer.model, layer.left)?
    {
        let ranged = matches!(weapon_hold(layer.model), WeaponHold::TwoHanded);
        // These ownership masks remove the whole original hand/cuff, including
        // pixels outside the old narrow arm cut. Two-handed melee needs the
        // same clean sleeve boundary as long guns before its closed grip moves.
        body = part(index, "ranged-core");
        let offset = two_hand_offset(rig, anchor, support);
        let (targets, scale) = two_hand_targets(index, layer.model, anchor, support);
        for (side, target) in targets.into_iter().enumerate() {
            geometry::over(
                &mut body,
                &pose(
                    &part(
                        index,
                        if side == 0 {
                            "ranged-arm-0"
                        } else {
                            "ranged-arm-1"
                        },
                    ),
                    rig,
                    side,
                    target,
                ),
            );
            let angle = if matches!(weapon_hold(layer.model), WeaponHold::MeleeTwoHanded)
                && (anchor[1] - support[1]).abs() < 1.
            {
                0.694
            } else {
                0.
            };
            geometry::over(
                &mut body,
                &geometry::transformed(
                    &part(index, if side == 0 { "grip-0" } else { "grip-1" }),
                    rig.palms[side],
                    target,
                    1.,
                    angle,
                ),
            );
            geometry::over(
                &mut fingers,
                &geometry::transformed(
                    &part(index, if side == 0 { "fingers-0" } else { "fingers-1" }),
                    rig.palms[side],
                    target,
                    1.,
                    angle,
                ),
            );
        }
        let weapon = weapon_image(layer.model, layer.left, key.2[1 + usize::from(layer.left)])?;
        weapons = if ranged {
            let center = [(anchor[0] + support[0]) / 2., (anchor[1] + support[1]) / 2.];
            geometry::transformed(
                &weapon,
                center,
                [center[0] + offset[0] as f32, center[1] + offset[1] as f32],
                scale,
                0.,
            )
        } else {
            moved(&weapon, offset[0], offset[1])
        };
    } else {
        for layer in held {
            let side = usize::from(!layer.left);
            let raised = matches!(weapon_hold(layer.model), WeaponHold::Raised);
            let target = if raised {
                raised_palm(rig, side)
            } else {
                rig.palms[side]
            };
            let angle = if raised {
                if layer.left {
                    std::f32::consts::FRAC_PI_2
                } else {
                    -std::f32::consts::FRAC_PI_2
                }
            } else {
                0.
            };
            let painted_arm = raised && rig.corrected_arms;
            if painted_arm {
                // Each corrected sleeve owns its complete old silhouette.
                // The replacement contains the bent arm and its closed hand;
                // no source limb is stretched or left hanging underneath it.
                erase_owned(
                    &mut body,
                    &part(
                        index,
                        if side == 0 {
                            "raised-erase-0"
                        } else {
                            "raised-erase-1"
                        },
                    ),
                );
                geometry::over(
                    &mut body,
                    &part(
                        index,
                        if side == 0 {
                            "raised-arm-0"
                        } else {
                            "raised-arm-1"
                        },
                    ),
                );
            } else if raised {
                let (arm, elbow) = raised_forearm(index, side);
                erase_owned(&mut body, &arm);
                geometry::over(&mut body, &pose_limb(&arm, elbow, rig.palms[side], target));
            }
            let mask = part(
                index,
                if side == 0 {
                    "hand-mask-0"
                } else {
                    "hand-mask-1"
                },
            );
            if !painted_arm {
                erase_owned(&mut body, &mask);
                geometry::over(
                    &mut body,
                    &geometry::transformed(
                        &part(index, if side == 0 { "grip-0" } else { "grip-1" }),
                        rig.palms[side],
                        target,
                        1.,
                        angle,
                    ),
                );
            }
            let (anchor, _) = attachment(layer.model, layer.left)?;
            let weapon = weapon_image(layer.model, layer.left, key.2[1 + usize::from(layer.left)])?;
            geometry::over(
                &mut weapons,
                &if raised {
                    geometry::transformed(&weapon, anchor, target, 1., angle)
                } else {
                    moved(
                        &weapon,
                        (target[0] - anchor[0]).round() as i32,
                        (target[1] - anchor[1]).round() as i32,
                    )
                },
            );
            if painted_arm {
                geometry::over(
                    &mut fingers,
                    &part(
                        index,
                        if side == 0 {
                            "raised-fingers-0"
                        } else {
                            "raised-fingers-1"
                        },
                    ),
                );
            } else {
                geometry::over(
                    &mut fingers,
                    &geometry::transformed(
                        &part(index, if side == 0 { "fingers-0" } else { "fingers-1" }),
                        rig.palms[side],
                        target,
                        1.,
                        angle,
                    ),
                );
            }
        }
    }
    helmet(&mut body, index, head);
    // Approved armor colors stay as drawn until their own color-variant review.
    Some(vec![body, weapons, fingers])
}

#[cfg(test)]
#[path = "movement_catalogue_review_export.rs"]
mod movement_catalogue_review_export;

#[cfg(test)]
mod tests {
    use super::*;
    fn key(index: u16, weapon: Option<u16>, hands: Option<[Option<u16>; 2]>) -> Key {
        (
            BASE + index,
            selection(weapon, hands),
            [EquipmentPalette::Original; 3],
        )
    }
    #[test]
    fn item_model_selects_the_same_stage_but_each_class_keeps_its_identity() {
        for (item, rank) in [(112, 0), (114, 1), (105, 2), (101, 3), (103, 4)] {
            let mut ids = std::collections::BTreeSet::new();
            for class in 0..5 {
                let id = outfit_index(class + 3, [Some(item), None, None, None]).unwrap();
                assert_eq!(decode(id).unwrap().0, usize::from(class) * 5 + rank);
                ids.insert(id);
                assert_eq!(
                    outfit_index(class + 3, [Some(item), Some(120), None, None]),
                    Some(id + 6 * HEAD_STRIDE)
                );
            }
            assert_eq!(ids.len(), 5);
        }
        assert!(outfit_index(0, [None; 4]).is_none());
    }

    #[test]
    fn active_single_edged_knife_bevels_face_away_from_the_body_in_both_hands() {
        for model in [68, 79] {
            for left in [false, true] {
                let source =
                    generic::weapon_image(model, left, EquipmentPalette::Original).unwrap();
                let corrected = weapon_image(model, left, EquipmentPalette::Original).unwrap();
                let axis = attachment(model, left).unwrap().0[0].round() as i32;
                let mut source_pixels: Vec<_> = source
                    .bytes
                    .chunks_exact(4)
                    .filter(|p| p[3] > 0)
                    .map(|p| p.to_vec())
                    .collect();
                let mut corrected_pixels: Vec<_> = corrected
                    .bytes
                    .chunks_exact(4)
                    .filter(|p| p[3] > 0)
                    .map(|p| p.to_vec())
                    .collect();
                source_pixels.sort();
                corrected_pixels.sort();
                assert_eq!(
                    source_pixels, corrected_pixels,
                    "orientation must preserve every painted pixel"
                );
                let mut tested = 0;
                let mut outward = 0;
                for y in 49..55 {
                    let steel: Vec<_> = (axis - 8..=axis + 8)
                        .filter(|&x| {
                            let p = corrected.get_pixel(x as u32, y);
                            p.a >= 0.5 && p.r > 0.3 && (p.r - p.b).abs() < 0.2
                        })
                        .collect();
                    if steel.len() < 2 {
                        continue;
                    }
                    let brightest = *steel
                        .iter()
                        .max_by(|&&a, &&b| {
                            corrected
                                .get_pixel(a as u32, y)
                                .r
                                .total_cmp(&corrected.get_pixel(b as u32, y).r)
                        })
                        .unwrap();
                    let midpoint = (steel[0] + steel[steel.len() - 1]) as f32 / 2.;
                    if if left {
                        (brightest as f32) <= midpoint
                    } else {
                        (brightest as f32) >= midpoint
                    } {
                        outward += 1;
                    }
                    tested += 1;
                }
                assert!(tested >= 4, "blade bevel probe missed model {model}");
                // A bevel is a bright band across the blade, not every single
                // highlight: the choil and curved tip can have a bright rim
                // on either side. The unchanged source fails this majority.
                assert!(
                    outward * 3 >= tested * 2,
                    "model {model}, left {left}: cutting bevel faces torso on most blade rows"
                );
            }
        }
        for model in 0..HELD_WEAPON_COUNT {
            if matches!(model, 68 | 79) || readable::binding(model).is_some() {
                continue;
            }
            for left in [false, true] {
                assert_eq!(
                    weapon_image(model, left, EquipmentPalette::Original)
                        .unwrap()
                        .bytes,
                    generic::weapon_image(model, left, EquipmentPalette::Original)
                        .unwrap()
                        .bytes
                );
            }
        }
    }

    #[test]
    #[ignore = "Explicit melee visual diagnostic; requires RL_CLASS_MELEE_OUTPUT"]
    fn class_melee_native_diagnostic() {
        let output = std::path::PathBuf::from(std::env::var("RL_CLASS_MELEE_OUTPUT").unwrap());
        std::fs::create_dir_all(&output).unwrap();
        let mut cases = vec![("unarmed".to_owned(), [None, None])];
        for model in (68..=87).chain(94..=100) {
            for left in [false, true] {
                cases.push((
                    format!("weapon-{model:03}-{}", if left { "left" } else { "right" }),
                    if left {
                        [Some(model), None]
                    } else {
                        [None, Some(model)]
                    },
                ));
            }
        }
        cases.extend([
            ("dual-knives".to_owned(), [Some(68), Some(68)]),
            ("dual-stilettos".to_owned(), [Some(79), Some(79)]),
            ("knife-pistol".to_owned(), [Some(68), Some(101)]),
            ("pistol-knife".to_owned(), [Some(101), Some(68)]),
        ]);
        for index in 0..25 {
            let mut sheet =
                Image::gen_image_color(1024, (cases.len().div_ceil(8) * 128) as u16, BLANK);
            for (cell, (_, hands)) in cases.iter().enumerate() {
                let im = super::super::compose(key(index, None, Some(*hands)), 0).unwrap();
                for y in 0..128 {
                    let at = ((cell / 8 * 128 + y) * 1024 + cell % 8 * 128) * 4;
                    sheet.bytes[at..at + 512].copy_from_slice(&im.bytes[y * 512..(y + 1) * 512]);
                }
            }
            sheet.export_png(
                output
                    .join(format!("outfit-{index:02}.png"))
                    .to_str()
                    .unwrap(),
            );
        }
        std::fs::write(output.join("capture.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "source":"ordinary finished-worn compositor","native_tile":64,"canvas":128,"columns":8,
            "outfits":25,"one_hand_melee_models":27,"configurations":cases.iter().map(|c|&c.0).collect::<Vec<_>>(),
            "exported_pngs_need_vertical_flip":true
        })).unwrap()).unwrap();
    }
    #[test]
    fn all_25_unarmed_compositions_preserve_the_approved_native_pixels_exactly() {
        for index in 0..25 {
            let layers = layers(key(index, None, None)).unwrap();
            assert_eq!(layers[0].bytes, part(index as usize, "base").bytes);
            assert!(layers[1].bytes.iter().all(|v| *v == 0));
            assert!(layers[2].bytes.iter().all(|v| *v == 0));
        }
    }
    #[test]
    fn long_gun_core_has_no_old_hands_or_outer_heavy_cuffs() {
        for index in 0..25 {
            let core = part(index, "ranged-core");
            for palm in ARMOR_RIGS[index].palms {
                for dy in 0..4 {
                    let at = (((palm[1] as usize + dy) * 128 + palm[0] as usize) * 4) + 3;
                    assert_eq!(
                        core.bytes[at], 0,
                        "old hanging fingers: outfit {index}, {palm:?}, dy {dy}"
                    );
                }
            }
        }
        // This outer copper cuff was outside the previous narrow arm polygon.
        let index = 4;
        let x = 44usize;
        let y = 67usize;
        let at = (y * 128 + x) * 4 + 3;
        assert!(part(index, "base").bytes[at] >= 64);
        assert_eq!(part(index, "ranged-core").bytes[at], 0);
    }
    #[test]
    fn every_weapon_and_supported_hand_has_a_drawable_attachment_on_every_outfit() {
        let mut failures = Vec::new();
        for index in 0..25 {
            for model in 0..HELD_WEAPON_COUNT {
                for left in [false, true] {
                    let key = key(
                        index,
                        None,
                        Some(if left {
                            [Some(model), None]
                        } else {
                            [None, Some(model)]
                        }),
                    );
                    let parts = layers(key)
                        .unwrap_or_else(|| panic!("outfit {index}, model {model}, left {left}"));
                    assert!(parts[1].bytes.chunks_exact(4).any(|p| p[3] > 0));
                    assert!(parts[2].bytes.chunks_exact(4).any(|p| p[3] > 0));
                    let rig = &ARMOR_RIGS[index as usize];
                    let (anchor, support) = attachment(model, left).unwrap();
                    let targets = if let Some(support) = support {
                        two_hand_targets(index as usize, model, anchor, support)
                            .0
                            .to_vec()
                    } else {
                        let side = usize::from(!left);
                        vec![if matches!(weapon_hold(model), WeaponHold::Raised) {
                            raised_palm(rig, side)
                        } else {
                            rig.palms[side]
                        }]
                    };
                    for target in targets {
                        let x = target[0].round() as usize;
                        let y = target[1].round() as usize;
                        // Adjacent visible edges are a valid contact too: a
                        // finger can wrap beside a thin shaft without hiding
                        // one of its pixels. Empty space between them is not.
                        let contact = (y - 3..=y + 3)
                            .flat_map(|yy| (x - 3..=x + 3).map(move |xx| (xx, yy)))
                            .filter(|&(xx, yy)| {
                                parts[2].bytes[(yy * 128 + xx) * 4 + 3] >= 16
                                    && (yy - 1..=yy + 1).any(|wy| {
                                        (xx - 1..=xx + 1)
                                            .any(|wx| parts[1].bytes[(wy * 128 + wx) * 4 + 3] >= 16)
                                    })
                            })
                            .count();
                        if contact == 0 {
                            failures.push(format!("outfit {index}, model {model}, left {left}: fingers miss grip {target:?}"));
                        }
                    }
                    // Raised forearms can share the head's rows, but must
                    // never change the head itself or any fixed boot pixels.
                    let base = part(index as usize, "base");
                    let head = part(index as usize, "head-mask");
                    for (pixel, rgba) in base.bytes.chunks_exact(4).enumerate() {
                        let y = pixel / 128;
                        if (head.bytes[pixel * 4 + 3] > 0 || (85..96).contains(&y))
                            && parts[0].bytes[pixel * 4..pixel * 4 + 4] != *rgba
                        {
                            failures.push(format!(
                                "outfit {index}, model {model}, fixed head/boot pixel {pixel}"
                            ));
                            break;
                        }
                        if matches!(weapon_hold(model), WeaponHold::Raised)
                            && head.bytes[pixel * 4 + 3] > 0
                            && rgba[3] >= 128
                            && (parts[1].bytes[pixel * 4 + 3] >= 16
                                || parts[2].bytes[pixel * 4 + 3] >= 16)
                        {
                            failures.push(format!("outfit {index}, model {model}, left {left}: pistol covers head pixel {pixel}"));
                            break;
                        }
                    }
                }
            }
        }
        assert!(
            failures.is_empty(),
            "{} attachment failures:\n{}",
            failures.len(),
            failures
                .iter()
                .take(40)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
    #[test]
    fn open_helmets_restore_the_same_female_class_identity_at_every_armor_stage() {
        for class in [2usize, 4] {
            for head in [1usize, 2] {
                let mut faces = Vec::new();
                for rank in 0..5 {
                    let index = class * 5 + rank;
                    let body = layers((
                        BASE + index as u16 + head as u16 * HEAD_STRIDE,
                        [None, None],
                        [EquipmentPalette::Original; 3],
                    ))
                    .unwrap()
                    .remove(0);
                    let dy = (ARMOR_RIGS[index].head_center[1] - 40.).round() as i32;
                    let hole = moved(&part(head - 1, "helmet-openings"), 0, dy);
                    let cover = moved(&part(head - 1, "helmets"), 0, dy);
                    let face: Vec<_> = body
                        .bytes
                        .chunks_exact(4)
                        .zip(hole.bytes.chunks_exact(4))
                        .zip(cover.bytes.chunks_exact(4))
                        .filter(|((_, h), c)| h[3] >= 128 && c[3] == 0)
                        .flat_map(|((p, _), _)| p.to_vec())
                        .collect();
                    assert!(!face.is_empty());
                    assert!(face.chunks_exact(4).any(|p| p[3] > 0));
                    faces.push(face);
                }
                assert!(
                    faces.windows(2).all(|w| w[0] == w[1]),
                    "class {class}, helmet {head}: changed face"
                );
            }
        }
    }

    #[test]
    fn all_one_hand_pairs_are_supported_without_changing_either_weapon_design() {
        let one: Vec<_> = (0..HELD_WEAPON_COUNT)
            .filter(|m| generic::attachment(*m, false).unwrap().1.is_none())
            .collect();
        for index in 0..25 {
            for &left in &one {
                for &right in &one {
                    assert!(supports(key(index, None, Some([Some(left), Some(right)]))));
                }
            }
        }
    }

    #[test]
    fn single_dual_and_mixed_pistols_fold_only_their_own_forearm_upward() {
        for index in 0..25 {
            let rig = &ARMOR_RIGS[index];
            for model in [101, 106, 112] {
                for hands in [
                    [None, Some(model)],
                    [Some(model), None],
                    [Some(model), Some(model)],
                    [Some(68), Some(model)],
                    [Some(model), Some(68)],
                ] {
                    let images = layers(key(index as u16, None, Some(hands))).unwrap();
                    let base = part(index, "base");
                    for side in 0..2 {
                        let slot = if side == 0 { 0 } else { 1 };
                        let palm = rig.palms[side];
                        let pixel = (palm[1] as usize * 128 + palm[0] as usize) * 4;
                        if hands[slot] == Some(model) {
                            let target = raised_palm(rig, side);
                            let (_, elbow) = raised_forearm(index, side);
                            assert!(target[1] < elbow[1], "forearm must rise from its elbow");
                            assert_eq!(images[0].bytes[pixel + 3], 0, "old hanging hand remains");
                            assert!(
                                images[1]
                                    .bytes
                                    .chunks_exact(4)
                                    .enumerate()
                                    .any(|(n, p)| p[3] >= 32
                                        && ((n / 128) as f32) < target[1] - 3.
                                        && ((n % 128) as f32 - target[0]).abs() < 5.),
                                "barrel must point up beside head"
                            );
                        } else if hands[slot].is_none() {
                            assert_eq!(
                                &images[0].bytes[pixel..pixel + 4],
                                &base.bytes[pixel..pixel + 4],
                                "empty-side hand changed"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn corrected_pistol_sleeves_leave_no_old_cuffs_or_armored_forearm_edges() {
        for index in (0..5).chain(15..20) {
            for side in 0..2 {
                let rig = &ARMOR_RIGS[index];
                assert!(rig.corrected_arms);
                let hands = if side == 0 {
                    [Some(101), None]
                } else {
                    [None, Some(101)]
                };
                let result = layers(key(index as u16, None, Some(hands))).unwrap();
                let erase = part(
                    index,
                    if side == 0 {
                        "raised-erase-0"
                    } else {
                        "raised-erase-1"
                    },
                );
                let replacement = part(
                    index,
                    if side == 0 {
                        "raised-arm-0"
                    } else {
                        "raised-arm-1"
                    },
                );
                let base = part(index, "base");
                // Inspect the complete lower cuff/forearm band, including the
                // outer edges missed by the former narrow centreline mask.
                let (x0, x1, y0, y1) = if index < 5 {
                    let x = rig.palms[side][0] as usize;
                    (x - 4, x + 4, 64, 70)
                } else if side == 0 {
                    (45, 54, 63, 68)
                } else {
                    (74, 83, 63, 68)
                };
                let mut removed = 0;
                for y in y0..y1 {
                    for x in x0..x1 {
                        let p = (y * 128 + x) * 4;
                        if base.bytes[p + 3] >= 16 {
                            assert!(
                                erase.bytes[p + 3] > 0,
                                "outfit {index}, side {side}: unowned old cuff {x},{y}"
                            );
                            assert_eq!(
                                replacement.bytes[p + 3],
                                0,
                                "replacement must not hang at old cuff"
                            );
                            assert_eq!(result[0].bytes[p + 3], 0, "old dangling cuff remains");
                            removed += 1;
                        }
                    }
                }
                // Tier 01 is sleeveless: its bare hands end above this band.
                if index != 15 {
                    assert!(
                        removed > 3,
                        "outfit {index}, side {side}: cuff probe missed source"
                    );
                }
                // No repaint, erasure or posing may escape this side's ownership.
                for (p, rgba) in base.bytes.chunks_exact(4).enumerate() {
                    if erase.bytes[p * 4 + 3] == 0 && replacement.bytes[p * 4 + 3] == 0 {
                        assert_eq!(&result[0].bytes[p * 4..p * 4 + 4], rgba);
                    }
                }
            }
        }
    }

    #[test]
    fn weapons_cover_armor_and_only_isolated_gripping_fingers_cover_weapons() {
        for index in 0..25 {
            for side in 0..2 {
                let fg = part(index, if side == 0 { "fingers-0" } else { "fingers-1" });
                // Include the source's translucent edge, but no sleeve or
                // armor outside the small source-authored closed hand.
                let palm = ARMOR_RIGS[index].palms[side];
                for (pixel, rgba) in fg.bytes.chunks_exact(4).enumerate() {
                    if rgba[3] > 0 {
                        assert!(
                            (pixel % 128) as f32 >= palm[0] - 5.
                                && (pixel % 128) as f32 <= palm[0] + 5.
                                && (pixel / 128) as f32 >= palm[1] - 5.
                                && (pixel / 128) as f32 <= palm[1] + 5.,
                            "outfit {index}: foreground extends beyond gripping fingers"
                        );
                    }
                }
            }
            for model in [2, 38, 88, 97, 101] {
                let key = key(index as u16, Some(model), None);
                let pieces = layers(key).unwrap();
                let composed = super::super::compose(key, 0).unwrap();
                for (i, pixel) in pieces[1].bytes.chunks_exact(4).enumerate() {
                    if pixel[3] == 255 && pieces[2].bytes[i * 4 + 3] == 0 {
                        assert_eq!(
                            &composed.bytes[i * 4..i * 4 + 4],
                            pixel,
                            "outfit {index}, model {model}: armor hides weapon"
                        );
                    }
                }
            }
        }
    }

    #[test]
    #[ignore = "Unintegrated weapon candidates; requires RL_WEAPON_REVIEW_OUTPUT"]
    fn weapon_readability_candidate_diagnostic() {
        let output = std::path::PathBuf::from(
            std::env::var("RL_WEAPON_REVIEW_OUTPUT").expect("review output required"),
        );
        let config: serde_json::Value =
            serde_json::from_slice(&std::fs::read(output.join("candidates.json")).unwrap())
                .unwrap();
        let target_dir = output.join("candidate-layers");
        std::fs::create_dir_all(&target_dir).unwrap();
        let pair = |value: &serde_json::Value| -> [f32; 2] {
            [
                value[0].as_f64().unwrap() as f32,
                value[1].as_f64().unwrap() as f32,
            ]
        };
        let mut checks = Vec::new();
        for candidate in config["candidates"].as_array().unwrap() {
            let name = candidate["id"].as_str().unwrap();
            let image = Image::from_file_with_format(
                &std::fs::read(candidate["native"].as_str().unwrap()).unwrap(),
                None,
            )
            .unwrap();
            let mut weapon = Image::gen_image_color(128, 128, BLANK);
            let origin = pair(&candidate["origin"]);
            for y in 0..usize::from(image.height) {
                let start = ((y + origin[1] as usize) * 128 + origin[0] as usize) * 4;
                let row = usize::from(image.width) * 4;
                weapon.bytes[start..start + row]
                    .copy_from_slice(&image.bytes[y * row..(y + 1) * row]);
            }
            let anchor = pair(&candidate["anchor"]);
            let support = pair(&candidate["support"]);
            let center = [(anchor[0] + support[0]) / 2., (anchor[1] + support[1]) / 2.];
            let angle = candidate["angle"].as_f64().unwrap() as f32;
            let grip_angle = candidate["grip_angle"].as_f64().unwrap() as f32;
            let (sin, cos) = angle.sin_cos();
            for (index, rig) in ARMOR_RIGS.iter().enumerate() {
                let scale = candidate["scales"][index / 5].as_f64().unwrap() as f32;
                let shift = pair(&candidate["shift_from_palms"]);
                let target_center = [
                    (rig.palms[0][0] + rig.palms[1][0]) / 2. + shift[0],
                    (rig.palms[0][1] + rig.palms[1][1]) / 2. + shift[1],
                ];
                let targets = [anchor, support].map(|p| {
                    let dx = (p[0] - center[0]) * scale;
                    let dy = (p[1] - center[1]) * scale;
                    [
                        target_center[0] + cos * dx - sin * dy,
                        target_center[1] + sin * dx + cos * dy,
                    ]
                });
                let mut body = part(index, "ranged-core");
                let mut fingers = Image::gen_image_color(128, 128, BLANK);
                for (side, target) in targets.into_iter().enumerate() {
                    geometry::over(
                        &mut body,
                        &pose(
                            &part(
                                index,
                                if side == 0 {
                                    "ranged-arm-0"
                                } else {
                                    "ranged-arm-1"
                                },
                            ),
                            rig,
                            side,
                            target,
                        ),
                    );
                    geometry::over(
                        &mut body,
                        &geometry::transformed(
                            &part(index, if side == 0 { "grip-0" } else { "grip-1" }),
                            rig.palms[side],
                            target,
                            1.,
                            grip_angle,
                        ),
                    );
                    geometry::over(
                        &mut fingers,
                        &geometry::transformed(
                            &part(index, if side == 0 { "fingers-0" } else { "fingers-1" }),
                            rig.palms[side],
                            target,
                            1.,
                            grip_angle,
                        ),
                    );
                }
                let held = geometry::transformed(&weapon, center, target_center, scale, angle);
                for target in targets {
                    let x = target[0].round() as usize;
                    let y = target[1].round() as usize;
                    assert!(
                        (y.saturating_sub(2)..=(y + 2).min(127)).any(|yy| {
                            (x.saturating_sub(2)..=(x + 2).min(127))
                                .any(|xx| held.bytes[(yy * 128 + xx) * 4 + 3] > 64)
                        }),
                        "{name}/{index}: grip misses weapon"
                    );
                }
                for (layer, image) in [body, held, fingers].iter().enumerate() {
                    image.export_png(
                        target_dir
                            .join(format!("{name}-{index:02}-layer-{layer}.png"))
                            .to_str()
                            .unwrap(),
                    );
                }
                checks.push(serde_json::json!({"candidate":name,"outfit":index,"scale":scale,"targets":targets}));
            }
        }
        std::fs::write(output.join("candidate-checks.json"), serde_json::to_vec_pretty(&serde_json::json!({
            "cases":checks,"grips_checked":checks.len()*2,"exported_pngs_need_vertical_flip":true,
            "production_compositor_used_for_arm_geometry":true,"runtime_integration":false
        })).unwrap()).unwrap();
    }

    #[test]
    #[ignore = "Read-only layer export for the weapon readability review; requires RL_WEAPON_REVIEW_OUTPUT"]
    fn weapon_readability_source_diagnostic() {
        let output = std::path::PathBuf::from(
            std::env::var("RL_WEAPON_REVIEW_OUTPUT").expect("review output required"),
        );
        std::fs::create_dir_all(&output).unwrap();
        let mut bindings = Vec::new();
        let models: Vec<u16> = std::env::var("RL_WEAPON_REVIEW_MODELS")
            .unwrap_or_else(|_| "2,66".to_owned())
            .split(',')
            .map(|value| value.parse().expect("numeric review model"))
            .collect();
        for model in models {
            weapon_image(model, false, EquipmentPalette::Original)
                .unwrap()
                .export_png(
                    output
                        .join(format!("weapon-{model:03}.png"))
                        .to_str()
                        .unwrap(),
                );
            let (anchor, support) = attachment(model, false).unwrap();
            bindings.push(serde_json::json!({"model":model,"anchor":anchor,"support":support}));
            for index in 0..25 {
                for (layer, image) in layers(key(index, Some(model), None))
                    .unwrap()
                    .iter()
                    .enumerate()
                {
                    image.export_png(
                        output
                            .join(format!(
                                "outfit-{index:02}-weapon-{model:03}-layer-{layer}.png"
                            ))
                            .to_str()
                            .unwrap(),
                    );
                }
            }
        }
        let rigs: Vec<_> = ARMOR_RIGS
            .iter()
            .map(|rig| {
                serde_json::json!({
                    "palms":rig.palms,"pivots":rig.pivots
                })
            })
            .collect();
        std::fs::write(
            output.join("layers.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
                "native_tile":64,"canvas":128,"bindings":bindings,"rigs":rigs,
                "exported_pngs_need_vertical_flip":true,
                "source":"production compositor, read-only diagnostic, no candidate integration"
            }))
            .unwrap(),
        )
        .unwrap();
    }

    #[test]
    #[ignore = "Explicit source-compositor visual diagnostic; requires RL_CLASS_ARMOR_OUTPUT"]
    fn class_armor_native_diagnostic() {
        let output = std::path::PathBuf::from(
            std::env::var("RL_CLASS_ARMOR_OUTPUT").expect("diagnostic output required"),
        );
        std::fs::create_dir_all(&output).unwrap();
        let mut labels = vec!["unarmed".to_owned()];
        labels.extend((0..HELD_WEAPON_COUNT).map(|m| format!("weapon-{m:03}")));
        labels.extend(
            [
                "dual-pistols",
                "dual-melee-ranged",
                "dual-melee",
                "helmet-1",
                "helmet-2",
                "helmet-3",
                "helmet-4",
                "helmet-5",
                "helmet-6",
                "breath-rifle",
            ]
            .map(str::to_owned),
        );
        let height = labels.len().div_ceil(16) * 128;
        for index in 0..25 {
            let mut sheet = Image::gen_image_color(2048, height as u16, BLANK);
            for (cell, label) in labels.iter().enumerate() {
                let (key, breath) = if let Some(model) = label.strip_prefix("weapon-") {
                    (key(index, Some(model.parse().unwrap()), None), 0)
                } else if let Some(head) = label.strip_prefix("helmet-") {
                    let mut k = key(index, Some(38), None);
                    k.0 += head.parse::<u16>().unwrap() * HEAD_STRIDE;
                    (k, 0)
                } else {
                    match label.as_str() {
                        "dual-pistols" => (key(index, None, Some([Some(101), Some(101)])), 0),
                        "dual-melee-ranged" => (key(index, None, Some([Some(68), Some(101)])), 0),
                        "dual-melee" => (key(index, None, Some([Some(68), Some(82)])), 0),
                        "breath-rifle" => (key(index, Some(38), None), 1),
                        _ => (key(index, None, None), 0),
                    }
                };
                let image = super::super::compose(key, breath).unwrap();
                for y in 0..128 {
                    let at = ((cell / 16 * 128 + y) * 2048 + cell % 16 * 128) * 4;
                    sheet.bytes[at..at + 512].copy_from_slice(&image.bytes[y * 512..(y + 1) * 512]);
                }
            }
            sheet.export_png(
                output
                    .join(format!("outfit-{index:02}.png"))
                    .to_str()
                    .unwrap(),
            );
        }
        std::fs::write(output.join("capture.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "source":"ordinary finished-worn compositor","outfits":25,"weapon_models":HELD_WEAPON_COUNT,
            "configurations":labels,"canvas":128,"native_tile":64,"columns":16,
            "exported_pngs_need_vertical_flip":true
        })).unwrap()).unwrap();
    }
}
