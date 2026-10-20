//! Accepted scale and horizontal grips. Source art and item identities stay fixed.
use super::super::held_geometry as geometry;
use super::*;

pub(super) struct Grip {
    pub model: u16,
    pub hands: [[f32; 2]; 2],
    pub opposed: bool,
}
include!("game_art_held_catalog.rs");

pub(super) struct HeldPoses {
    weapons: Image,
}
impl HeldPoses {
    pub(super) fn load() -> Self {
        Self {
            weapons: palettes::decoded(include_bytes!(
                "../assets/sprites/held-native64/weapons-128.png"
            )),
        }
    }
    pub(super) fn grip(model: u16) -> Option<(usize, &'static Grip)> {
        REVISED_GRIPS
            .iter()
            .enumerate()
            .find(|(_, grip)| grip.model == model)
    }
    pub(super) fn weapon(&self, model: u16) -> Option<Image> {
        let (index, _) = Self::grip(model)?;
        Some(palettes::cell(&self.weapons, index as u16, 128))
    }
    pub(super) fn compose(
        &self,
        index: usize,
        grip: &Grip,
        body: &Image,
        sleeves: &Image,
        fingers: &[Image; 2],
    ) -> Image {
        geometry::compose(
            body,
            sleeves,
            fingers,
            &palettes::cell(&self.weapons, index as u16, 128),
            grip.hands,
            grip.opposed,
        )
    }
}
