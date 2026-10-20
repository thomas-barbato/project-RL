//! Draw-only lighting fixtures. Dormant in ordinary play and absent in release.
use super::*;
thread_local! {
    static FIGURES: std::cell::RefCell<Vec<(u16,u8)>> = const {std::cell::RefCell::new(Vec::new())};
}
pub(crate) fn keys() -> &'static [(u16, u8, u16)] {
    finish::COMPLETED_MONSTERS
}
pub(crate) fn set(figures: &[(u16, u8)]) {
    FIGURES.with(|f| *f.borrow_mut() = figures.to_vec());
}
pub(crate) fn normalized_visual(value: Visual) -> Visual {
    if FIGURES.with(|f| !f.borrow().is_empty()) && matches!(value, Visual::Player { .. }) {
        return player(
            Some("core:bastion"),
            PlayerWeaponPose::Ranged,
            Some("core:modele_cogneur_p01"),
        );
    }
    value
}
pub(crate) fn draw_worn(
    rect: Rect,
    outfit: u16,
    weapon: Option<u16>,
    hands: Option<[Option<u16>; 2]>,
    colors: [EquipmentPalette; 3],
) {
    super::player_layers::draw_finish_review(rect, outfit, weapon, hands, colors);
}
pub(crate) fn scene_actors(
    camera: &crate::terminal_view::GridCamera,
    game: &project_rl::game::WorldState,
) {
    FIGURES.with(|f| {
        for (at, &(number, variant)) in super::finish_review_05::positions(game)
            .into_iter()
            .zip(f.borrow().iter())
        {
            super::draw_world(
                camera.rect(at),
                Visual::CampaignMonster { number, variant },
                1.,
            );
        }
    });
}
