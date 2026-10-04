//! Canvas navigation; document coordinates and editing operations stay unchanged.
use super::{App, Camera};
use macroquad::prelude::*;

#[derive(Default)]
pub struct Navigation {
    drag: Option<(bool, f32)>,
    pan: Option<Vec2>,
}

struct Axis {
    track: Rect,
    thumb: Rect,
    overflow: f32,
    travel: f32,
    vertical: bool,
}
impl Axis {
    fn new(track: Rect, world: f32, visible: f32, offset: f32, vertical: bool) -> Self {
        let length = if vertical { track.h } else { track.w };
        let overflow = (world - visible).max(0.);
        let size = (length * visible / world.max(1.)).clamp(28f32.min(length), length);
        let travel = (length - size).max(0.);
        let at = if overflow == 0. {
            0.
        } else {
            offset.clamp(0., overflow) / overflow * travel
        };
        let thumb = if vertical {
            Rect::new(track.x, track.y + at, track.w, size)
        } else {
            Rect::new(track.x + at, track.y, size, track.h)
        };
        Self {
            track,
            thumb,
            overflow,
            travel,
            vertical,
        }
    }
    fn offset_at(&self, pointer: f32, grab: f32) -> f32 {
        let start = if self.vertical {
            self.track.y
        } else {
            self.track.x
        };
        if self.travel == 0. {
            0.
        } else {
            ((pointer - start - grab) / self.travel).clamp(0., 1.) * self.overflow
        }
    }
}

pub fn normalize(camera: &mut Camera, width: i32, height: i32, bounds: Rect) {
    let visible = bounds.size() / camera.cell;
    for (value, world, view) in [
        (&mut camera.center.x, width as f32, visible.x),
        (&mut camera.center.y, height as f32, visible.y),
    ] {
        *value = if world <= view {
            world / 2.
        } else {
            value.clamp(view / 2., world - view / 2.)
        };
    }
}

pub fn zoom_at(camera: &mut Camera, mouse: Vec2, cell: f32, width: i32, height: i32, bounds: Rect) {
    let anchor = (mouse - camera.origin(width, height, bounds)) / camera.cell;
    camera.cell = cell.clamp(8., 128.);
    camera.center = anchor + (bounds.point() + bounds.size() / 2. - mouse) / camera.cell;
    normalize(camera, width, height, bounds);
}

pub fn visible_region(app: &App) -> Rect {
    let doc = &app.scene.document;
    let b = app.bounds();
    let origin = app.camera.origin(doc.width, doc.height, b);
    let first = ((b.point() - origin) / app.camera.cell).max(Vec2::ZERO);
    let size = (b.size() / app.camera.cell).min(vec2(doc.width as f32, doc.height as f32));
    Rect::new(first.x, first.y, size.x, size.y)
}

fn axes(app: &App) -> [Axis; 2] {
    let b = app.bounds();
    let view = visible_region(app);
    [
        Axis::new(
            Rect::new(b.x, b.bottom() + 3., b.w, 14.),
            app.scene.document.width as f32,
            view.w,
            view.x,
            false,
        ),
        Axis::new(
            Rect::new(b.right() + 3., b.y, 14., b.h),
            app.scene.document.height as f32,
            view.h,
            view.y,
            true,
        ),
    ]
}

pub fn input(app: &mut App) -> bool {
    if app.modal.is_some() || app.workbench.panel.is_some() {
        return false;
    }
    let mouse: Vec2 = mouse_position().into();
    let b = app.bounds();
    let (width, height) = (app.scene.document.width, app.scene.document.height);
    if !is_mouse_button_down(MouseButton::Left) {
        app.navigation.drag = None;
    }
    if !app.testing {
        for axis in axes(app) {
            let pointer = if axis.vertical { mouse.y } else { mouse.x };
            if is_mouse_button_pressed(MouseButton::Left) && axis.track.contains(mouse) {
                let start = if axis.vertical {
                    axis.thumb.y
                } else {
                    axis.thumb.x
                };
                let length = if axis.vertical {
                    axis.thumb.h
                } else {
                    axis.thumb.w
                };
                let grab = if axis.thumb.contains(mouse) {
                    pointer - start
                } else {
                    length / 2.
                };
                app.navigation.drag = Some((axis.vertical, grab));
            }
            if let Some((vertical, grab)) = app.navigation.drag.filter(|(v, _)| *v == axis.vertical)
            {
                let offset = axis.offset_at(pointer, grab);
                if vertical {
                    app.camera.center.y = offset + b.h / app.camera.cell / 2.;
                } else {
                    app.camera.center.x = offset + b.w / app.camera.cell / 2.;
                }
                normalize(&mut app.camera, width, height, b);
                return true;
            }
        }
    }
    let shift = is_key_down(KeyCode::LeftShift) || is_key_down(KeyCode::RightShift);
    let space_pan = is_key_down(KeyCode::Space) && is_mouse_button_down(MouseButton::Left);
    let panning = (is_mouse_button_down(MouseButton::Middle) || space_pan)
        && (b.contains(mouse) || app.navigation.pan.is_some());
    if panning {
        if let Some(previous) = app.navigation.pan {
            app.camera.center -= (mouse - previous) / app.camera.cell;
        }
        app.navigation.pan = Some(mouse);
        app.editor.break_paint_path();
        normalize(&mut app.camera, width, height, b);
        return true;
    }
    app.navigation.pan = None;
    if b.contains(mouse) && !super::workbench::minimap_rect(b).contains(mouse) {
        let (horizontal, wheel) = mouse_wheel();
        if wheel != 0. {
            if shift && app.editor.round_ground() && !app.testing {
                app.editor.resize_brush(wheel > 0.);
            } else {
                let zooms = [8., 12., 16., 24., 32., 48., 64., 96., 128.];
                let current = zooms
                    .iter()
                    .position(|v| *v >= app.camera.cell)
                    .unwrap_or(8);
                let next = if wheel > 0. {
                    (current + 1).min(8)
                } else {
                    current.saturating_sub(1)
                };
                zoom_at(&mut app.camera, mouse, zooms[next], width, height, b);
            }
            return true;
        }
        if horizontal != 0. && !app.testing {
            app.camera.center.x -= horizontal * 96. / app.camera.cell;
            normalize(&mut app.camera, width, height, b);
            return true;
        }
    }
    if !app.testing {
        let mut delta = Vec2::ZERO;
        for (key, step) in [
            (KeyCode::Left, vec2(-1., 0.)),
            (KeyCode::Right, vec2(1., 0.)),
            (KeyCode::Up, vec2(0., -1.)),
            (KeyCode::Down, vec2(0., 1.)),
        ] {
            if is_key_down(key) {
                delta += step;
            }
        }
        if delta != Vec2::ZERO {
            app.camera.center += delta * get_frame_time().min(0.05) * 800. / app.camera.cell
                * if shift { 2. } else { 1. };
            normalize(&mut app.camera, width, height, b);
            return true;
        }
    }
    false
}

pub fn draw(app: &App) {
    if app.testing {
        return;
    }
    for axis in axes(app) {
        draw_rectangle(
            axis.track.x,
            axis.track.y,
            axis.track.w,
            axis.track.h,
            Color::from_rgba(24, 35, 43, 255),
        );
        let hover = axis.thumb.contains(mouse_position().into());
        let color = if axis.overflow == 0. {
            Color::from_rgba(44, 64, 72, 255)
        } else if hover || app.navigation.drag.is_some_and(|(v, _)| v == axis.vertical) {
            Color::from_rgba(105, 211, 187, 255)
        } else {
            Color::from_rgba(74, 116, 124, 255)
        };
        draw_rectangle(
            axis.thumb.x + 2.,
            axis.thumb.y + 2.,
            (axis.thumb.w - 4.).max(1.),
            (axis.thumb.h - 4.).max(1.),
            color,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scrollbar_reaches_both_edges_and_preserves_grab_offset() {
        let axis = Axis::new(Rect::new(100., 200., 600., 14.), 160., 32., 64., false);
        assert_eq!(axis.offset_at(100., 0.), 0.);
        assert_eq!(axis.offset_at(700., 0.), 128.);
        assert!((axis.offset_at(axis.thumb.x + 9., 9.) - 64.).abs() < 0.001);
        let vertical = Axis::new(Rect::new(20., 60., 14., 400.), 112., 24., 80., true);
        assert_eq!(vertical.offset_at(1000., 10.), 88.);
        let small = Axis::new(Rect::new(0., 0., 100., 14.), 8., 40., 0., false);
        assert_eq!(small.offset_at(500., 2.), 0.);
    }
    #[test]
    fn zoom_keeps_the_point_under_the_cursor_at_all_steps() {
        let bounds = Rect::new(304., 162., 800., 500.);
        let mouse = vec2(750., 380.);
        let mut camera = Camera {
            center: vec2(80., 56.),
            cell: 32.,
        };
        for zoom in [48., 64., 96., 128., 64., 32., 24., 16.] {
            let before = (mouse - camera.origin(160, 112, bounds)) / camera.cell;
            zoom_at(&mut camera, mouse, zoom, 160, 112, bounds);
            let after = (mouse - camera.origin(160, 112, bounds)) / camera.cell;
            assert!((before - after).length() < 0.1);
        }
    }
    #[test]
    fn small_maps_stay_centered_and_large_maps_do_not_scroll_past_the_edge() {
        let bounds = Rect::new(0., 0., 800., 500.);
        let mut camera = Camera {
            center: vec2(-500., 500.),
            cell: 32.,
        };
        normalize(&mut camera, 160, 112, bounds);
        assert_eq!(camera.center, vec2(12.5, 104.1875));
        normalize(&mut camera, 8, 6, bounds);
        assert_eq!(camera.center, vec2(4., 3.));
    }
}
