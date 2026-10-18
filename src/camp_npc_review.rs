//! CAMP-NPC-01: approved NPC markers and breathing; presentation only.
use macroquad::prelude::*;
use project_rl::{game::WorldState, world::GridPos};

#[cfg(any(debug_assertions, test))]
thread_local! {
    static CLOCK: std::cell::Cell<Option<f64>> = const {std::cell::Cell::new(None)};
}

#[cfg(any(debug_assertions, test))]
pub struct ReviewGuard(Option<f64>);
#[cfg(any(debug_assertions, test))]
impl Drop for ReviewGuard {
    fn drop(&mut self) {
        CLOCK.with(|r| r.set(self.0));
    }
}
#[cfg(any(debug_assertions, test))]
pub fn begin() -> ReviewGuard {
    ReviewGuard(CLOCK.with(|r| r.replace(None)))
}

#[cfg(any(debug_assertions, test))]
pub fn set_capture_time(time: f64) {
    CLOCK.with(|r| r.set(Some(time)));
}

pub fn animation_time(time: Option<f64>) -> Option<f64> {
    #[cfg(any(debug_assertions, test))]
    {
        time.map(|t| CLOCK.with(|r| r.get()).unwrap_or(t))
    }
    #[cfg(not(any(debug_assertions, test)))]
    {
        time
    }
}

// Same accepted two-pose cadence as humanoid breathing, at native 64 px.
pub fn breath_offset(time: Option<f64>) -> f32 {
    time.filter(|t| t.is_finite() && *t >= 0.)
        .map_or(0., |t| ((t / 1.5).floor() % 2.) as f32 * 2.)
}

pub fn marker_at(game: &WorldState, at: GridPos) -> Option<u8> {
    if !game.player_visibility().is_visible(at) {
        return None;
    }
    game.actors()
        .entity_at(at)
        .and_then(|id| game.actors().get(id))
        .and_then(crate::cryogenic_camp::actor_index)
}

pub fn draw_marker(cell: Rect, index: u8, rise: f32) {
    let size = 18. * cell.w / 64.;
    let r = Rect::new(
        (cell.center().x - size / 2. - if index == 1 { 11. } else { 0. }).round(),
        (cell.y - size - 5. - rise).round(),
        size,
        size,
    );
    let color = Color::from_rgba(248, 220, 150, 255);
    let shadow = Color::from_rgba(6, 10, 12, 255);
    if index == 1 {
        // A speech bubble avoids suggesting that a new quest is available.
        for (dy, c) in [(1., shadow), (0., color)] {
            draw_rectangle_lines(r.x + 1., r.y + 2. + dy, 16., 11., 2., c);
            draw_line(r.x + 4., r.y + 12. + dy, r.x + 4., r.y + 16. + dy, 2., c);
            draw_line(r.x + 4., r.y + 16. + dy, r.x + 8., r.y + 12. + dy, 2., c);
            for x in [5., 9., 13.] {
                draw_rectangle(r.x + x, r.y + 6. + dy, 1., 2., c);
            }
        }
    } else {
        use crate::ui_theme::{UiIcon, draw_ui_icon};
        let icon = match index {
            2 => UiIcon::Weapon,
            3 => UiIcon::Add,
            _ => UiIcon::Settings,
        };
        draw_ui_icon(icon, Rect::new(r.x, r.y + 1., r.w, r.h), shadow);
        draw_ui_icon(icon, r, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn breathing_uses_the_existing_cadence_and_disables_cleanly() {
        for time in [
            None,
            Some(-1.),
            Some(f64::NAN),
            Some(f64::INFINITY),
            Some(0.),
            Some(1.499),
            Some(3.),
        ] {
            assert_eq!(breath_offset(time), 0.);
        }
        for time in [1.5, 2.999, 4.5] {
            assert_eq!(breath_offset(Some(time)), 2.);
        }
    }
    #[test]
    fn capture_clock_restores_and_respects_disabled_animation() {
        {
            let _review = begin();
            set_capture_time(1.5);
            assert_eq!(animation_time(Some(0.)), Some(1.5));
            assert_eq!(animation_time(None), None);
        }
        assert_eq!(animation_time(Some(0.)), Some(0.));
    }
}
