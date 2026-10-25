//! Exact approved native weapon drawings, hand poses and local contact shadow.
use super::{ARMOR_RIGS, geometry, helmet, palettes, part, pose};
use macroquad::prelude::{BLANK, Image};
use project_rl::entity::EquipmentPalette;
use std::collections::BTreeMap;

pub(super) struct ReviewedWeapon {
    pub(super) model: u16,
    bytes: &'static [u8],
    material: &'static [u8],
    pub(super) anchor: [f32; 2],
    pub(super) support: [f32; 2],
    angle: f32,
    grip_angle: f32,
    scales: [f32; 5],
    shift: [f32; 2],
}
include!("game_art_weapon_readability_catalog.rs");

// Accepted with the WALK09 equipped-player review; distinct from model66.
const AXE67_REVIEW: ReviewedWeapon = ReviewedWeapon {
    model: 67,
    bytes: include_bytes!("../assets/animations/walk-08/weapons/AX67-r02.png"),
    material: include_bytes!("../assets/animations/walk-08/weapons/AX67-r02-materials.png"),
    anchor: [52., 60.],
    support: [68., 60.],
    angle: 0.,
    grip_angle: 0.694,
    scales: [1., 0.95, 0.88, 0.92, 0.96],
    shift: [2., 0.],
};

thread_local! {
    static IMAGES: BTreeMap<u16, (palettes::LazyImage, palettes::LazyImage)> = {
        let images = WEAPONS.iter()
            .map(|w| (w.model, (palettes::LazyImage::new(w.bytes), palettes::LazyImage::new(w.material))))
            .collect();
        {
            let mut images: BTreeMap<_, _> = images;
            images.insert(67, (palettes::LazyImage::new(AXE67_REVIEW.bytes), palettes::LazyImage::new(AXE67_REVIEW.material)));
            images
        }
    };
}

pub(super) fn binding(model: u16) -> Option<&'static ReviewedWeapon> {
    if model == 67 && crate::game_art::walking::player_enabled() {
        return Some(&AXE67_REVIEW);
    }
    WEAPONS.iter().find(|w| w.model == model)
}

pub(super) fn image(model: u16, color: EquipmentPalette) -> Option<Image> {
    binding(model)?;
    let color = match color {
        EquipmentPalette::ReviewedVariant1 => EquipmentPalette::Blue,
        EquipmentPalette::ReviewedVariant2 => EquipmentPalette::Red,
        color => color,
    };
    IMAGES.with(|images| {
        let (source, mask) = &images[&model];
        let mut out = (**source).clone();
        palettes::recolor(&mut out, mask, color);
        Some(out)
    })
}

impl ReviewedWeapon {
    fn center(&self) -> [f32; 2] {
        [
            (self.anchor[0] + self.support[0]) / 2.,
            (self.anchor[1] + self.support[1]) / 2.,
        ]
    }
    fn target_center(&self, index: usize) -> [f32; 2] {
        let rig = &ARMOR_RIGS[index];
        [
            (rig.palms[0][0] + rig.palms[1][0]) / 2. + self.shift[0],
            (rig.palms[0][1] + rig.palms[1][1]) / 2. + self.shift[1],
        ]
    }
    pub(super) fn targets(&self, index: usize) -> ([[f32; 2]; 2], f32) {
        let scale = self.scales[index / 5];
        let center = self.center();
        let target = self.target_center(index);
        let (sin, cos) = self.angle.sin_cos();
        (
            [self.anchor, self.support].map(|p| {
                let dx = (p[0] - center[0]) * scale;
                let dy = (p[1] - center[1]) * scale;
                [
                    target[0] + cos * dx - sin * dy,
                    target[1] + sin * dx + cos * dy,
                ]
            }),
            scale,
        )
    }
    /// Actual pixel-index locations after the frozen front renderer samples
    /// around pixel centres. Keep the normal/front placement unchanged.
    pub(super) fn raster_targets(&self, index: usize) -> [[f32; 2]; 2] {
        let (targets, scale) = self.targets(index);
        let (s, c) = self.angle.sin_cos();
        let mut shift = [0.5 * (scale * (c - s) - 1.), 0.5 * (scale * (s + c) - 1.)];
        if self.model == 67 {
            // AX67-r02's three-pixel wood shaft is centred on row59; row60
            // is its lower outline, rather than the centre of the grip.
            shift[0] += s * scale;
            shift[1] -= c * scale;
        }
        targets.map(|p| [p[0] + shift[0], p[1] + shift[1]])
    }
}

pub(super) fn contact_shadow(body: &Image, weapon: &Image) -> Image {
    let mut shadow = Image::gen_image_color(128, 128, BLANK);
    for y in 0usize..128 {
        for x in 0usize..128 {
            let at = (y * 128 + x) * 4;
            if body.bytes[at + 3] == 0 {
                continue;
            }
            let maximum = (y.saturating_sub(1)..=(y + 1).min(127))
                .flat_map(|yy| {
                    (x.saturating_sub(1)..=(x + 1).min(127)).map(move |xx| (yy * 128 + xx) * 4 + 3)
                })
                .map(|p| weapon.bytes[p])
                .max()
                .unwrap();
            let ring = maximum.saturating_sub(weapon.bytes[at + 3]);
            let clipped = u32::from(ring) * u32::from(body.bytes[at + 3]) / 255;
            // Match the reviewed Pillow mask: round(0.9*alpha), ties to even.
            let scaled = clipped * 9;
            let alpha = scaled / 10
                + u32::from(scaled % 10 > 5 || (scaled % 10 == 5 && (scaled / 10) % 2 == 1));
            shadow.bytes[at..at + 4].copy_from_slice(&[8, 11, 15, alpha as u8]);
        }
    }
    shadow
}

pub(super) fn layers(
    index: usize,
    head: usize,
    model: u16,
    color: EquipmentPalette,
) -> Option<Vec<Image>> {
    let weapon = binding(model)?;
    let rig = &ARMOR_RIGS[index];
    let (targets, scale) = weapon.targets(index);
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
                weapon.grip_angle,
            ),
        );
        geometry::over(
            &mut fingers,
            &geometry::transformed(
                &part(index, if side == 0 { "fingers-0" } else { "fingers-1" }),
                rig.palms[side],
                target,
                1.,
                weapon.grip_angle,
            ),
        );
    }
    let held = geometry::transformed(
        &image(model, color)?,
        weapon.center(),
        weapon.target_center(index),
        scale,
        weapon.angle,
    );
    helmet(&mut body, index, head);
    Some(vec![body, held, fingers])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accepted_slim_hachette_keeps_approved_axe_intact() {
        assert!(!crate::game_art::walking::corrections_enabled());
        let approved = image(66, EquipmentPalette::Original).unwrap();
        {
            assert_eq!(binding(67).unwrap().anchor, [52., 60.]);
            let candidate = image(67, EquipmentPalette::Original).unwrap();
            assert_ne!(candidate.bytes, approved.bytes);
            assert_eq!(
                image(66, EquipmentPalette::Original).unwrap().bytes,
                approved.bytes
            );
            // The wood shaft is substantially thinner than the old 6px band.
            for x in [55usize, 60, 65] {
                let ys: Vec<_> = (0..128)
                    .filter(|y| candidate.bytes[(y * 128 + x) * 4 + 3] >= 128)
                    .collect();
                assert!(!ys.is_empty());
                assert!(ys.last().unwrap() - ys[0] + 1 <= 4);
            }
        }
        assert!(binding(67).is_some());
    }

    #[test]
    fn existing_paints_preserve_steel_edges_and_geometry_on_all_six_weapons() {
        assert_eq!(WEAPONS.len(), 6);
        for w in WEAPONS {
            let original = image(w.model, EquipmentPalette::Original).unwrap();
            IMAGES.with(|images| {
                let mask = &images[&w.model].1;
                assert!(mask.bytes.chunks_exact(4).any(|m| m[0] == 1));
                for color in EquipmentPalette::ALL
                    .into_iter()
                    .chain([
                        EquipmentPalette::ReviewedVariant1,
                        EquipmentPalette::ReviewedVariant2,
                    ])
                    .chain(EquipmentPalette::TORSO_FINISH)
                {
                    let variant = image(w.model, color).unwrap();
                    for ((a, b), m) in original
                        .bytes
                        .chunks_exact(4)
                        .zip(variant.bytes.chunks_exact(4))
                        .zip(mask.bytes.chunks_exact(4))
                    {
                        assert_eq!(a[3], b[3]);
                        if m[0] == 0 {
                            assert_eq!(a, b, "neutral material changed: {}", w.model);
                        }
                    }
                }
                for (alias, color) in [
                    (EquipmentPalette::ReviewedVariant1, EquipmentPalette::Blue),
                    (EquipmentPalette::ReviewedVariant2, EquipmentPalette::Red),
                ] {
                    assert_eq!(
                        image(w.model, alias).unwrap().bytes,
                        image(w.model, color).unwrap().bytes
                    );
                }
            });
        }
    }
}
