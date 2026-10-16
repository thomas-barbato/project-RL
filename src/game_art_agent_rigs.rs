//! Approved native attachments for the five agent silhouettes.
//! Reuses family IDs and independent body/weapon material palettes.
use super::*;

const BODY_BINDINGS: [u16; 5] = [9, 2, 1, 11, 5];
const LEFT_FAMILIES: [u8; 7] = [0, 1, 4, 5, 6, 7, 16];

pub(super) struct AgentRigs {
    bodies: Image,
    body_maps: Image,
    weapons: Image,
    weapon_maps: Image,
    arms: Image,
    arm_maps: Image,
    fingers: Image,
}

impl AgentRigs {
    pub(super) fn load() -> Self {
        Self {
            bodies: palettes::decoded(include_bytes!(
                "../assets/sprites/cyberpunk/agent-rigs/bodies-32.png"
            )),
            body_maps: palettes::decoded(include_bytes!(
                "../assets/sprites/cyberpunk/agent-rigs/body-materials-32.png"
            )),
            weapons: palettes::decoded(include_bytes!(
                "../assets/sprites/cyberpunk/agent-rigs/weapons-64.png"
            )),
            weapon_maps: palettes::decoded(include_bytes!(
                "../assets/sprites/cyberpunk/agent-rigs/weapon-materials-64.png"
            )),
            arms: palettes::decoded(include_bytes!(
                "../assets/sprites/cyberpunk/agent-rigs/arms-64.png"
            )),
            arm_maps: palettes::decoded(include_bytes!(
                "../assets/sprites/cyberpunk/agent-rigs/arm-materials-64.png"
            )),
            fingers: palettes::decoded(include_bytes!(
                "../assets/sprites/cyberpunk/agent-rigs/fingers-64.png"
            )),
        }
    }

    pub(super) fn profile(body: u16) -> Option<u16> {
        BODY_BINDINGS
            .iter()
            .position(|&id| id == body)
            .map(|p| p as u16)
    }

    #[cfg(test)]
    pub(super) fn body(&self, body: u16) -> Option<Image> {
        Some(palettes::cell(&self.bodies, Self::profile(body)?, 32))
    }

    fn index(profile: u16, layer: HeldLayer) -> Option<u16> {
        let family = if layer.left {
            18 + LEFT_FAMILIES.iter().position(|&f| f == layer.family)? as u16
        } else {
            u16::from(layer.family)
        };
        Some(profile * 25 + family)
    }

    fn weapon(
        &self,
        profile: u16,
        layer: HeldLayer,
        colors: [EquipmentPalette; 3],
    ) -> Option<Image> {
        let index = Self::index(profile, layer)?;
        let mut image = palettes::cell(&self.weapons, index, 64);
        palettes::recolor(
            &mut image,
            &palettes::cell(&self.weapon_maps, index, 64),
            colors[if layer.left { 2 } else { 1 }],
        );
        Some(image)
    }

    fn body_pose(result: &mut Image, body: &Image, breath: u8) {
        for (start, end, offset) in [(0, 23, usize::from(breath)), (23, 32, 0)] {
            for y in start..end {
                for x in 0..32 {
                    let source = (y * 32 + x) * 4;
                    if body.bytes[source + 3] != 0 {
                        let dest = ((16 + y + offset) * 64 + 16 + x) * 4;
                        result.bytes[dest..dest + 4]
                            .copy_from_slice(&body.bytes[source..source + 4]);
                    }
                }
            }
        }
    }

    pub(super) fn compose(&self, key: Key, breath: u8) -> Option<Image> {
        let (body, layers, colors) = key;
        let profile = Self::profile(body)?;
        let mut fixed = palettes::cell(&self.bodies, profile, 32);
        palettes::recolor(
            &mut fixed,
            &palettes::cell(&self.body_maps, profile, 32),
            colors[0],
        );
        let mut result = Image::gen_image_color(64, 64, BLANK);
        if let Some(heavy) = layers.into_iter().flatten().find(|l| l.pose >= 2) {
            // These configurations are already constrained by the active hand group.
            let index = Self::index(profile, heavy)?;
            let cuts = if profile == 2 {
                [(8, 16, 11, 20), (21, 16, 25, 20)]
            } else {
                [(3, 17, 10, 23), (22, 17, 30, 23)]
            };
            for (x0, y0, x1, y1) in cuts {
                for y in y0..y1 {
                    for x in x0..x1 {
                        fixed.bytes[(y * 32 + x) * 4..(y * 32 + x) * 4 + 4].fill(0);
                    }
                }
            }
            Self::body_pose(&mut result, &fixed, breath);
            let mut arms = palettes::cell(&self.arms, index, 64);
            palettes::recolor(
                &mut arms,
                &palettes::cell(&self.arm_maps, index, 64),
                colors[0],
            );
            paste(&mut result, &arms, 0, usize::from(breath), false);
            paste(
                &mut result,
                &self.weapon(profile, heavy, colors)?,
                0,
                usize::from(breath),
                false,
            );
            paste(
                &mut result,
                &palettes::cell(&self.fingers, index, 64),
                0,
                usize::from(breath),
                false,
            );
        } else {
            // One-hand weapons stay behind the authored gloves; no forearm is added.
            for held in layers.into_iter().flatten() {
                paste(
                    &mut result,
                    &self.weapon(profile, held, colors)?,
                    0,
                    usize::from(breath),
                    false,
                );
            }
            Self::body_pose(&mut result, &fixed, breath);
        }
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_composer_matches_all_375_approved_configurations_exactly() {
        let source = Sources::load();
        let review: serde_json::Value = serde_json::from_str(include_str!("../docs/reviews/refonte-agent-2026-10-06/weapon-silhouette-review-03/review-weapon-silhouettes-WEAPON-SIL01-03.json")).unwrap();
        let atlas = palettes::decoded(include_bytes!(
            "../assets/animations/agent/weapon-silhouette-review-03/class-weapons-review-atlas64.png"
        ));
        let weapon = |family: &serde_json::Value| {
            family.as_u64().map(|family| {
                WEAPON_FAMILIES
                    .iter()
                    .position(|&f| u64::from(f) == family)
                    .unwrap() as u16
            })
        };
        let frames = review["frames"].as_array().unwrap();
        assert_eq!(frames.len(), 375);
        for row in frames {
            let profile = row["class_number"].as_u64().unwrap() as usize - 1;
            let left = weapon(&row["left_family"]);
            let right = weapon(&row["right_family"]);
            let index = row["sprite_index"].as_u64().unwrap() as u32;
            let expected = atlas.sub_image(Rect::new(
                (index % 25 * 64) as f32,
                (index / 25 * 64) as f32,
                64.0,
                64.0,
            ));
            let actual = compose_pose(
                &source,
                (
                    BODY_BINDINGS[profile],
                    selection(None, Some([left, right])),
                    [EquipmentPalette::Original; 3],
                ),
                0,
            );
            assert_eq!(actual.bytes, expected.bytes, "{}", row["review_id"]);
        }
    }

    #[test]
    fn agent_palettes_are_independent_and_preserve_approved_geometry() {
        let source = Sources::load();
        let pistol = held_weapon("core:modele_cogneur_p01").unwrap();
        let knife = held_weapon("core:melee_couteau_de_camp").unwrap();
        let layers = selection(None, Some([Some(pistol), Some(knife)]));
        for body in BODY_BINDINGS {
            let original =
                compose_pose(&source, (body, layers, [EquipmentPalette::Original; 3]), 0);
            for part in 0..3 {
                for palette in EquipmentPalette::ALL {
                    let mut colors = [EquipmentPalette::Original; 3];
                    colors[part] = palette;
                    let colored = compose_pose(&source, (body, layers, colors), 0);
                    assert_eq!(original.width, colored.width);
                    for (a, b) in original
                        .bytes
                        .chunks_exact(4)
                        .zip(colored.bytes.chunks_exact(4))
                    {
                        assert_eq!(a[3], b[3]);
                    }
                }
            }
        }
    }
}
