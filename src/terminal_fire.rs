//! Native painted replacement proposed for the old 16-pixel terminal flames.
//! Uses the accepted 128-pixel flame atlas, reduced to native64, never enlarged.
use crate::{
    effect_geometry::LazyTexture,
    visual_effects::{TerminalEffectFamily, TerminalEffectSample},
};
use macroquad::prelude::*;
use std::cell::RefCell;

thread_local! { static ART: RefCell<Option<LazyTexture>> = const { RefCell::new(None) }; }

// FIRE-NATIVE-01: explicitly accepted by the player on 20 October 2026.
pub fn enabled() -> bool {
    true
}

pub fn draw(rect: Rect, effect: TerminalEffectSample) {
    ART.with_borrow_mut(|cache| {
        let art = cache.get_or_insert_with(|| {
            LazyTexture::fitted(
                include_bytes!(
                    "../assets/animations/agent/skill-animation-review-06/flame-atlas.png"
                ),
                FilterMode::Linear,
            )
        });
        let phase = if effect.family == TerminalEffectFamily::Flame {
            effect.motion_progress
        } else {
            effect.progress
        };
        let frame = ((phase * 8.).floor() as usize + usize::from(effect.variant % 8)) % 8;
        crate::effect_geometry::fitted_sheet(
            art,
            frame,
            &[project_rl::world::GridPos::new(0, 0)],
            &|_| Some(rect),
            0.,
            effect.color.a,
        );
    });
}

#[cfg(test)]
mod tests {
    #[test]
    fn flame_source_is_native_detail_and_not_a_doubled_small_sprite() {
        let bytes =
            include_bytes!("../assets/animations/agent/skill-animation-review-06/flame-atlas.png");
        let width = u32::from_be_bytes(bytes[16..20].try_into().unwrap());
        let height = u32::from_be_bytes(bytes[20..24].try_into().unwrap());
        assert_eq!((width, height), (1024, 128));
    }
}
