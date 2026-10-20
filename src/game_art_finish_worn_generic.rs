//! Source-authored finish limbs reattached to the existing weapon catalogue.
use super::super::held_geometry as geometry;
use super::*;
struct Rig {
    outfit: u16,
    number: u16,
    anchors: [[f32; 2]; 2],
    palms: [[f32; 2]; 2],
    shoulders: [[f32; 2]; 2],
}
struct WeaponBinding {
    model: u16,
    left: bool,
    anchor: [f32; 2],
    support: Option<[f32; 2]>,
    opposed: bool,
}
include!("game_art_finish_worn_catalog_06.rs");
thread_local! {
    static COMPONENTS: std::cell::RefCell<BTreeMap<String,Image>> = const {std::cell::RefCell::new(BTreeMap::new())};
}
fn component(number: u16, part: &str) -> Image {
    let name = format!("RIG-{number:02}-{part}");
    COMPONENTS.with(|c| {
        c.borrow_mut()
            .entry(name.clone())
            .or_insert_with(|| palettes::decoded(bytes_worn_06(&name).unwrap()))
            .clone()
    })
}
fn palette(p: EquipmentPalette) -> EquipmentPalette {
    match p {
        EquipmentPalette::ReviewedVariant1 => EquipmentPalette::Blue,
        EquipmentPalette::ReviewedVariant2 => EquipmentPalette::Red,
        _ => p,
    }
}
fn colored(number: u16, part: &str, color: EquipmentPalette) -> Image {
    let color = if (number == 18 && color == EquipmentPalette::ReviewedVariant1)
        || (matches!(number, 16 | 26) && color == EquipmentPalette::DeepRed)
    {
        EquipmentPalette::Original
    } else {
        color
    };
    let mut im = component(number, part);
    if color != EquipmentPalette::Original {
        let material = if number == 1 && color.is_torso_finish() && matches!(part, "base" | "core")
        {
            finish::image(&format!("TOR-RIG-01-{part}-material")).unwrap()
        } else {
            component(number, &format!("{part}-mask"))
        };
        palettes::recolor_finished(&mut im, &material, palette(color));
    }
    im
}
fn weapon_binding(model: u16, left: bool) -> Option<&'static WeaponBinding> {
    FINISHED_WEAPONS
        .iter()
        .find(|w| w.model == model && w.left == left)
        .or_else(|| {
            FINISHED_WEAPONS
                .iter()
                .find(|w| w.model == model && w.support.is_some())
        })
}
pub(super) fn supports(key: Key) -> bool {
    FINISHED_RIGS.iter().any(|r| r.outfit == key.0)
        && key
            .1
            .into_iter()
            .flatten()
            .all(|l| weapon_binding(l.model, l.left).is_some())
}
fn reflected(im: &Image) -> Image {
    let mut r = im.clone();
    for y in 0..128 {
        for x in 0..128 {
            let a = (y * 128 + x) * 4;
            let b = (y * 128 + 127 - x) * 4;
            r.bytes[a..a + 4].copy_from_slice(&im.bytes[b..b + 4]);
        }
    }
    r
}
fn weapon_image(model: u16, left: bool, color: EquipmentPalette) -> Option<Image> {
    let mut image = raw_weapon_image(model, left)?;
    if color != EquipmentPalette::Original {
        let mask_left = weapon_binding(model, left)?.left;
        palettes::recolor(
            &mut image,
            &component(0, &format!("weapon-{model}-{}-mask", u8::from(mask_left))),
            palette(color),
        );
    }
    Some(image)
}
fn raw_weapon_image(model: u16, left: bool) -> Option<Image> {
    if model == 2 {
        return finish::image("FIN-01-rifle-weapon");
    }
    if model == 101 {
        let im = finish::image("FIN-01-single-weapon")?;
        return Some(if left { reflected(&im) } else { im });
    }
    SOURCES.with(|s| s.flat.held_source(model, left))
}
fn arm_image(rig: &Rig, side: usize, color: EquipmentPalette) -> Image {
    let mut image = colored(rig.number, &format!("arm-{side}"), color);
    if !matches!(rig.number, 3..=5) {
        // The reused rifle masks extend into the trousers below the wrist.
        // Keep the accepted fingers' two-pixel margin, but never transform
        // those lower-body pixels along with the arm. The three separately
        // painted rigs already have isolated, bent elbows and need no trim.
        let bottom = (rig.anchors[side][1] + 3.).ceil() as usize;
        image.bytes[bottom * 512..].fill(0);
    }
    image
}
pub(super) fn layers(key: Key) -> Option<Vec<Image>> {
    if !supports(key) {
        return None;
    }
    let rig = FINISHED_RIGS.iter().find(|r| r.outfit == key.0)?;
    let mut body = colored(rig.number, "base", key.2[0]);
    let mut weapons = Image::gen_image_color(128, 128, BLANK);
    let mut fingers = weapons.clone();
    let held: Vec<_> = key.1.into_iter().flatten().collect();
    if let [layer] = held.as_slice()
        && let Some(w) = weapon_binding(layer.model, layer.left)
        && let Some(support) = w.support
    {
        body = colored(rig.number, "core", key.2[0]);
        for (side, at) in [w.anchor, support].into_iter().enumerate() {
            let arm = arm_image(rig, side, key.2[0]);
            geometry::over(
                &mut body,
                &geometry::posed_arm(&arm, rig.anchors[side], at, rig.shoulders[side]),
            );
            let hand = colored(rig.number, &format!("fingers-{side}"), key.2[0]);
            geometry::over(
                &mut fingers,
                &geometry::transformed(
                    &hand,
                    rig.anchors[side],
                    at,
                    1.,
                    if w.opposed { 0.694 } else { 0. },
                ),
            );
        }
        geometry::over(
            &mut weapons,
            &weapon_image(layer.model, layer.left, key.2[1 + usize::from(layer.left)])?,
        );
    } else {
        for layer in held {
            let side = usize::from(!layer.left);
            let w = weapon_binding(layer.model, layer.left)?;
            let im = weapon_image(layer.model, layer.left, key.2[1 + usize::from(layer.left)])?;
            geometry::over(
                &mut weapons,
                &geometry::transformed(&im, w.anchor, rig.palms[side], 1., 0.),
            );
            geometry::over(
                &mut fingers,
                &colored(rig.number, &format!("palm-{side}"), key.2[0]),
            );
        }
    }
    let fixed = colored(rig.number, "base", key.2[0]);
    for rows in [0..44usize, 80..128] {
        body.bytes[rows.start * 512..rows.end * 512]
            .copy_from_slice(&fixed.bytes[rows.start * 512..rows.end * 512]);
    }
    Some(vec![body, weapons, fingers])
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ranged_grip_has_no_transformed_trouser_limb_below_the_support_hand() {
        for rig in FINISHED_RIGS.iter().filter(|r| !matches!(r.number, 3..=5)) {
            for color in [
                EquipmentPalette::Original,
                EquipmentPalette::Blue,
                EquipmentPalette::Red,
            ] {
                let parts = layers((
                    rig.outfit,
                    selection(Some(38), None),
                    [
                        color,
                        EquipmentPalette::Original,
                        EquipmentPalette::Original,
                    ],
                ))
                .unwrap();
                // The patrol rifle and its hands are above this region. The
                // old leg fragment formed a diagonal third limb here.
                for y in 68..78 {
                    for x in 88..128 {
                        assert_eq!(
                            parts[0].bytes[(y * 128 + x) * 4 + 3],
                            0,
                            "outfit {}: phantom limb at {x},{y}",
                            rig.outfit
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn torso_finish_colors_match_accepted_outfits_and_preserve_weapon_layers() {
        let manifest: serde_json::Value = serde_json::from_str(include_str!(
            "../docs/reviews/refonte-histoire-2026-10-16/torso-icons-review-01/candidates.json"
        ))
        .unwrap();
        let supplement: serde_json::Value = serde_json::from_str(include_str!(
            "../docs/reviews/refonte-histoire-2026-10-16/torso-starting-review-02/candidates.json"
        ))
        .unwrap();
        let entries = manifest["entries"]
            .as_array()
            .unwrap()
            .iter()
            .chain(supplement["entries"].as_array().unwrap().iter());
        let mut checked = 0;
        for entry in entries {
            let outfit = entry["outfit"].as_u64().unwrap() as u16;
            let number = entry["finish_number"].as_u64().unwrap() as u16;
            for variant in entry["variants"].as_array().unwrap().iter().skip(1) {
                let color = match variant["code"].as_str().unwrap().as_bytes().last().unwrap() {
                    b'B' => EquipmentPalette::SlateBlue,
                    b'C' => EquipmentPalette::MossGreen,
                    b'D' => EquipmentPalette::Rust,
                    b'E' => EquipmentPalette::DeepRed,
                    _ => unreachable!(),
                };
                let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join(variant["worn"].as_str().unwrap());
                let expected = palettes::decoded(&std::fs::read(file).unwrap());
                let actual = colored(number, "base", color);
                assert!(
                    actual.bytes == expected.bytes,
                    "accepted outfit {outfit}/{color:?}"
                );
                for part in [
                    "base",
                    "core",
                    "arm-0",
                    "arm-1",
                    "fingers-0",
                    "fingers-1",
                    "palm-0",
                    "palm-1",
                ] {
                    let original = component(number, part);
                    let mask =
                        if let Some(file) = entry["new_palette_mask_overrides"][part].as_str() {
                            let file = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(file);
                            palettes::decoded(&std::fs::read(file).unwrap())
                        } else {
                            component(number, &format!("{part}-mask"))
                        };
                    let painted = colored(number, part, color);
                    assert!(
                        original
                            .bytes
                            .chunks_exact(4)
                            .zip(painted.bytes.chunks_exact(4))
                            .zip(mask.bytes.chunks_exact(4))
                            .all(|((a, b), m)| a[3] == b[3] && (m[0] == 1 || a == b)),
                        "protected {outfit}/{part}/{color:?}"
                    );
                }
                for hands in [
                    selection(Some(2), None),
                    selection(Some(88), None),
                    selection(None, Some([Some(101), Some(68)])),
                    selection(None, Some([Some(101), None])),
                ] {
                    let old = layers((outfit, hands, [EquipmentPalette::Original; 3])).unwrap();
                    let painted = layers((
                        outfit,
                        hands,
                        [
                            color,
                            EquipmentPalette::Original,
                            EquipmentPalette::Original,
                        ],
                    ))
                    .unwrap();
                    assert!(
                        old[1].bytes == painted[1].bytes,
                        "weapon changed {outfit}/{color:?}"
                    );
                    for rows in [0..44usize, 80..128] {
                        assert!(
                            painted[0].bytes[rows.start * 512..rows.end * 512]
                                == actual.bytes[rows.start * 512..rows.end * 512]
                        );
                    }
                }
                checked += 1;
            }
        }
        assert_eq!(checked, 74);
    }
    #[test]
    fn recoloring_preserves_bastion_skin_and_unpainted_weapon_materials() {
        let original = colored(1, "base", EquipmentPalette::Original);
        let blue = colored(1, "base", EquipmentPalette::Blue);
        assert!(original.bytes[..44 * 512] == blue.bytes[..44 * 512]);
        assert!(original.bytes != blue.bytes);
        for part in [
            "arm-0",
            "arm-1",
            "fingers-0",
            "fingers-1",
            "palm-0",
            "palm-1",
        ] {
            assert!(
                colored(1, part, EquipmentPalette::Original).bytes
                    == colored(1, part, EquipmentPalette::Blue).bytes,
                "skin changed in {part}"
            );
        }
        let original = weapon_image(0, false, EquipmentPalette::Original).unwrap();
        let red = weapon_image(0, false, EquipmentPalette::Red).unwrap();
        let mask = component(0, "weapon-0-0-mask");
        assert!(original.bytes != red.bytes);
        for ((a, b), m) in original
            .bytes
            .chunks_exact(4)
            .zip(red.bytes.chunks_exact(4))
            .zip(mask.bytes.chunks_exact(4))
        {
            assert_eq!(a[3], b[3]);
            if m[0] == 0 {
                assert!(a == b, "protected weapon material changed");
            }
        }
    }
    #[test]
    fn every_weapon_and_new_pose_keeps_the_finished_outfit_core() {
        for rig in FINISHED_RIGS {
            let key = |weapon, colors| (rig.outfit, selection(weapon, None), colors);
            for color in [
                EquipmentPalette::Original,
                EquipmentPalette::Blue,
                EquipmentPalette::Red,
            ] {
                let colors = [
                    color,
                    EquipmentPalette::Original,
                    EquipmentPalette::Original,
                ];
                let base = layers(key(None, colors)).unwrap();
                for weapon in [3, 15, 61, 70, 88, 101] {
                    let posed = layers(key(Some(weapon), colors)).unwrap();
                    for rows in [0..44usize, 80..128] {
                        assert!(
                            &base[0].bytes[rows.start * 512..rows.end * 512]
                                == &posed[0].bytes[rows.start * 512..rows.end * 512],
                            "{} / {weapon} / {}",
                            rig.outfit,
                            rows.start
                        );
                    }
                    assert!(posed[1].bytes.chunks_exact(4).any(|p| p[3] > 0));
                }
            }
        }
        for model in 0..HELD_WEAPON_COUNT {
            let key = (
                212,
                selection(Some(model), None),
                [EquipmentPalette::Original; 3],
            );
            let layers = layers(key).unwrap_or_else(|| panic!("Missing finished weapon {model}"));
            assert!(layers[1].bytes.chunks_exact(4).any(|p| p[3] > 0), "{model}");
            let left = (
                212,
                selection(None, Some([Some(model), None])),
                [
                    EquipmentPalette::Original,
                    EquipmentPalette::Original,
                    EquipmentPalette::Red,
                ],
            );
            let left = super::layers(left)
                .unwrap_or_else(|| panic!("Missing left-hand finished weapon {model}"));
            assert!(
                left[1].bytes.chunks_exact(4).any(|p| p[3] > 0),
                "left {model}"
            );
        }
    }
}
