//! Approved WALK-VIS-03-E: free route preview, then normal turn-by-turn moves.
use super::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct PlannedRoute {
    pub(super) cells: Vec<GridPos>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> AsciiApp {
        let mut app = AsciiApp::new_agent_blast_trial().unwrap();
        app.graphics.active.world_cell_px = 64;
        app.graphics.active.reduced_motion = true;
        app
    }

    fn click(app: &AsciiApp, at: GridPos) -> InputFrame {
        let cell = app
            .terminal
            .world_cell_rect(
                &app.game,
                app.terminal_bounds_for(1280., 800., 1.),
                1.,
                64,
                app.navigation_signal_summary().is_some(),
                at,
                app.camera_glide_offset(),
            )
            .unwrap();
        app.graphics.active.transform_input(InputFrame {
            pointer: Some(cell.center().into()),
            viewport: Some((1280., 800.)),
            pressed: [controls::Binding::MouseLeft].into(),
            ..Default::default()
        })
    }

    #[test]
    fn distant_preview_is_free_and_cancellation_does_not_open_a_menu() {
        for escape in [false, true] {
            let mut app = app();
            let before = app.game.recovery_snapshot_bytes().unwrap();
            let history = app.history.len();
            app.update_input_at(&click(&app, GridPos::new(8, 4)), Some(0.));
            assert!(app.walk_preview.is_some());
            assert!(app.mouse_walk.is_none());
            assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
            let cancel = if escape {
                InputFrame {
                    pause: true,
                    ..Default::default()
                }
            } else {
                InputFrame {
                    pressed: [controls::Binding::MouseRight].into(),
                    ..Default::default()
                }
            };
            app.update_input_at(&cancel, Some(1.));
            assert!(app.walk_preview.is_none());
            assert!(app.world_context.is_none());
            assert_eq!(app.menu, MenuScreen::Hidden);
            assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
            assert_eq!(app.history.len(), history);
        }
    }

    #[test]
    fn adjacent_click_and_confirmed_route_cost_one_turn_per_cell() {
        let mut app = app();
        let adjacent = GridPos::new(4, 5);
        let turn = app.game.turn();
        app.update_input_at(&click(&app, adjacent), Some(0.));
        assert_eq!(app.game.player_position(), Some(adjacent));
        assert_eq!(app.game.turn(), turn + 1);
        let target = GridPos::new(8, 5);
        let goal = click(&app, target);
        app.update_input_at(&goal, Some(1.));
        assert_eq!(
            app.walk_preview.as_ref().unwrap().cells.last(),
            Some(&target)
        );
        let steps = app.walk_preview.as_ref().unwrap().cells.len() - 1;
        let turn = app.game.turn();
        let history = app.history.len();
        app.update_input_at(&goal, Some(2.));
        assert_eq!(app.game.turn(), turn + 1);
        for tick in 3..20 {
            let previous = app.game.turn();
            app.update_input_at(&InputFrame::default(), Some(tick as f64));
            assert!(app.game.turn() <= previous + 1);
        }
        assert_eq!(app.game.player_position(), Some(target));
        assert_eq!(app.game.turn(), turn + steps as u64);
        assert_eq!(app.history.len(), history + steps);
        assert!(app.walk_preview.is_none());
        assert!(app.mouse_walk.is_none());
    }

    #[test]
    fn another_destination_replaces_preview_and_hover_or_held_click_never_confirms() {
        let mut app = app();
        let before = app.game.recovery_snapshot_bytes().unwrap();
        for target in [GridPos::new(8, 4), GridPos::new(8, 5)] {
            let mut frame = click(&app, target);
            app.update_input_at(&frame, Some(0.));
            assert_eq!(
                app.walk_preview.as_ref().unwrap().cells.last(),
                Some(&target)
            );
            frame.pressed.clear();
            frame.mouse_left_down = true;
            frame.held.insert(controls::Binding::MouseLeft);
            for tick in 1..4 {
                app.update_input_at(&frame, Some(tick as f64));
            }
            assert!(app.walk_preview.is_some());
            assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
        }
    }

    #[test]
    fn native64_pointer_navigation_matches_camera_in_2k_4k_and_ui_scales() {
        let mut app = app();
        let target = GridPos::new(5, 3);
        let before = app.game.recovery_snapshot_bytes().unwrap();
        for viewport in [(1280., 800.), (2560., 1440.), (3840., 2160.)] {
            for percent in [75, 100, 150] {
                app.graphics.active.ui_scale_percent = percent;
                let scale = app.graphics.active.ui_scale(viewport.0, viewport.1);
                let cell = app
                    .terminal
                    .world_cell_rect(
                        &app.game,
                        app.terminal_bounds_for(viewport.0, viewport.1, scale),
                        scale,
                        64,
                        app.navigation_signal_summary().is_some(),
                        target,
                        Vec2::ZERO,
                    )
                    .unwrap_or_else(|| {
                        panic!("Target outside camera: {viewport:?} scale {percent}")
                    });
                let frame = app.graphics.active.transform_input(InputFrame {
                    pointer: Some(cell.center().into()),
                    viewport: Some(viewport),
                    pressed: [controls::Binding::MouseLeft].into(),
                    ..Default::default()
                });
                app.walk_preview = None;
                app.update_input_at(&frame, Some(0.));
                assert_eq!(
                    app.walk_preview.as_ref().unwrap().cells.last(),
                    Some(&target),
                    "{viewport:?} scale {percent}"
                );
                assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
            }
        }
    }

    #[test]
    fn right_click_or_escape_stops_confirmed_walk_before_opening_any_menu() {
        for escape in [false, true] {
            let mut app = app();
            let target = click(&app, GridPos::new(8, 4));
            app.update_input_at(&target, Some(0.));
            app.update_input_at(&target, Some(1.));
            assert!(app.mouse_walk.is_some());
            let before = app.game.recovery_snapshot_bytes().unwrap();
            let cancel = InputFrame {
                pause: escape,
                pressed: if escape {
                    Default::default()
                } else {
                    [controls::Binding::MouseRight].into()
                },
                pointer: target.pointer,
                viewport: target.viewport,
                ..Default::default()
            };
            app.update_input_at(&cancel, Some(2.));
            app.update_input_at(&InputFrame::default(), Some(3.));
            assert!(app.mouse_walk.is_none());
            assert!(app.world_context.is_none());
            assert_eq!(app.menu, MenuScreen::Hidden);
            assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
        }
    }

    #[test]
    fn skill_selection_clears_route_and_right_click_still_cancels_the_skill() {
        let mut app = app();
        let before = app.game.recovery_snapshot_bytes().unwrap();
        app.update_input_at(&click(&app, GridPos::new(8, 4)), Some(0.));
        app.update_input_at(
            &InputFrame {
                pressed: [app.controls.binding(Action::Skill1).clone()].into(),
                ..Default::default()
            },
            Some(1.),
        );
        assert!(app.walk_preview.is_none());
        assert!(app.attack_aim_technique.is_some());
        app.update_input_at(
            &InputFrame {
                pressed: [controls::Binding::MouseRight].into(),
                ..Default::default()
            },
            Some(2.),
        );
        assert!(app.attack_aim.is_none());
        assert!(app.world_context.is_none());
        assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
    }

    #[test]
    fn keyboard_movement_and_opening_a_panel_clear_the_route() {
        for action in [Action::MoveSouth, Action::Inventory] {
            let mut app = app();
            app.update_input_at(&click(&app, GridPos::new(8, 4)), Some(0.));
            app.update_input_at(
                &InputFrame {
                    pressed: [app.controls.binding(action).clone()].into(),
                    ..Default::default()
                },
                Some(1.),
            );
            assert!(app.walk_preview.is_none());
            assert!(app.mouse_walk.is_none());
        }
    }

    #[test]
    fn a_changed_path_requires_a_new_confirmation_and_an_occupied_goal_never_moves() {
        let mut app = app();
        let target = GridPos::new(8, 4);
        app.update_input_at(&click(&app, target), Some(0.));
        let path = app.walk_preview.as_ref().unwrap().cells.clone();
        app.game
            .spawn_actor(Actor::new(path[2], 100).unwrap())
            .unwrap();
        app.game.drain_events();
        let before = app.game.recovery_snapshot_bytes().unwrap();
        app.update_input_at(&click(&app, target), Some(1.));
        assert!(app.mouse_walk.is_none());
        assert_ne!(app.walk_preview.as_ref().unwrap().cells, path);
        assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
        app.game
            .spawn_actor(Actor::new(target, 100).unwrap())
            .unwrap();
        app.game.drain_events();
        // Revalidate through the ground handler: an occupied goal cannot start
        // a route. A real actor click instead uses the separate attack branch.
        app.preview_or_walk_to(target, 2.);
        assert!(app.walk_preview.is_none());
        assert!(app.mouse_walk.is_none());
    }
}

impl AsciiApp {
    pub(super) fn preview_or_walk_to(&mut self, at: GridPos, now: f64) {
        let Some(cells) = self.context_path(at) else {
            self.walk_preview = None;
            return;
        };
        let route = PlannedRoute { cells };
        let steps = route.cells.len().saturating_sub(1);
        if steps == 1 || (steps > 1 && self.walk_preview.as_ref() == Some(&route)) {
            self.walk_preview = None;
            self.movement_repeat.clear();
            self.start_mouse_walk(at, now);
        } else {
            self.walk_preview = (steps > 1).then_some(route);
        }
    }

    pub(super) fn draw_walk_preview(&self) {
        let Some(route) = &self.walk_preview else {
            return;
        };
        if self.menu_navigation_context().is_some()
            || self.attack_aim.is_some()
            || self.game.status() != RunStatus::Active
            || route.cells.first().copied() != self.game.player_position()
        {
            return;
        }
        let rect = |at| {
            self.terminal.world_cell_rect(
                &self.game,
                self.terminal_bounds(),
                self.ui_scale(),
                self.graphics.active.world_cell_px,
                self.navigation_signal_summary().is_some(),
                at,
                self.camera_glide_offset(),
            )
        };
        let blue = Color::from_rgba(143, 204, 243, 255);
        let ink = Color::from_rgba(7, 18, 27, 240);
        let stroke = crate::effect_geometry::stroke(blue);
        let shadow = crate::effect_geometry::underlay(ink);
        for pair in route.cells.windows(2) {
            if !pair
                .iter()
                .all(|at| self.game.player_visibility().is_visible(*at))
            {
                continue;
            }
            let (Some(a), Some(b)) = (rect(pair[0]), rect(pair[1])) else {
                continue;
            };
            let direction = (b.center() - a.center()).normalize_or_zero();
            let start = a.center()
                + if pair[0] == route.cells[0] {
                    direction * 22.
                } else {
                    Vec2::ZERO
                };
            // The last line and its chevron stop outside the whole arrival
            // cell, leaving the approved E tile and its number unobstructed.
            let inset = if Some(&pair[1]) == route.cells.last() {
                if pair[0].x != pair[1].x {
                    b.w * 0.5
                } else {
                    b.h * 0.5
                }
            } else {
                0.
            };
            let end = b.center() - direction * inset;
            draw_line(start.x, start.y, end.x, end.y, 5., shadow);
            draw_line(
                start.x,
                start.y,
                end.x,
                end.y,
                8.,
                Color::new(blue.r, blue.g, blue.b, 0.10),
            );
            draw_line(start.x, start.y, end.x, end.y, 1.6, stroke);
            let mid = (start + end) * 0.5;
            let side = vec2(-direction.y, direction.x);
            for sign in [-1., 1.] {
                let tail = mid - direction * 4. + side * 3. * sign;
                draw_line(
                    tail.x,
                    tail.y,
                    mid.x + direction.x * 2.,
                    mid.y + direction.y * 2.,
                    1.3,
                    stroke,
                );
            }
        }
        for (step, &at) in route.cells.iter().enumerate().skip(1) {
            if !self.game.player_visibility().is_visible(at) {
                continue;
            }
            let Some(cell) = rect(at) else { continue };
            let center = cell.center();
            let destination = step + 1 == route.cells.len();
            if destination {
                let corners = [
                    vec2(-17., -24.),
                    vec2(17., -24.),
                    vec2(24., -17.),
                    vec2(24., 17.),
                    vec2(17., 24.),
                    vec2(-17., 24.),
                    vec2(-24., 17.),
                    vec2(-24., -17.),
                ];
                for i in 0..8 {
                    let a = center + corners[i];
                    let b = center + corners[(i + 1) % 8];
                    draw_triangle(center, a, b, Color::new(blue.r, blue.g, blue.b, 0.12));
                    draw_line(a.x, a.y, b.x, b.y, 4.5, shadow);
                    draw_line(a.x, a.y, b.x, b.y, 1.5, stroke);
                }
                for p in [center + vec2(0., -24.), center + vec2(0., 24.)] {
                    crate::preview_markers::circle(p, 3., 1., ink, blue);
                }
            }
            let radius = if destination { 11. } else { 10. };
            draw_circle(center.x, center.y, radius, ink);
            draw_circle_lines(center.x, center.y, radius, 1., blue);
            crate::ui_theme::draw_text_bold_centered(
                step.to_string(),
                Rect::new(center.x - 12., center.y - 12., 24., 24.),
                14,
                blue,
            );
        }
    }
}
