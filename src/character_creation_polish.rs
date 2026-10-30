//! Scoped creation-screen finish and bounded, presentation-only transitions.
use super::*;

pub(super) fn enabled() -> bool {
    // CREATION-UI02-B is the normal screen in both Debug and Release.
    true
}

fn ease(t: f64) -> f32 {
    let t = t.clamp(0., 1.) as f32;
    t * t * (3. - 2. * t)
}

#[derive(Default)]
struct Tween {
    from: f32,
    to: f32,
    started: f64,
}
impl Tween {
    fn value(&self, now: f64) -> f32 {
        self.from + (self.to - self.from) * ease((now - self.started) / 0.14)
    }
    fn aim(&mut self, target: f32, now: f64, reduced: bool) -> f32 {
        if reduced {
            self.from = target;
            self.to = target;
            self.started = now;
            return target;
        }
        if self.to != target {
            self.from = self.value(now);
            self.to = target;
            self.started = now;
        }
        self.value(now)
    }
}

#[derive(Default)]
pub(super) struct State {
    class: Option<usize>,
    detail: Option<(usize, Option<CharacterCreationHover>)>,
    class_changed: f64,
    detail_changed: f64,
    values: BTreeMap<u16, Tween>,
}
impl State {
    fn observe(&mut self, class: usize, detail: Option<CharacterCreationHover>, now: f64) {
        if self.class != Some(class) {
            self.class = Some(class);
            self.class_changed = now;
        }
        if self.detail != Some((class, detail)) {
            self.detail = Some((class, detail));
            self.detail_changed = now;
        }
    }
    fn strength(&mut self, key: u16, target: bool, now: f64, reduced: bool) -> f32 {
        if self.values.len() >= 128 && !self.values.contains_key(&key) {
            self.values.clear();
        }
        self.values
            .entry(key)
            .or_default()
            .aim(if target { 1. } else { 0. }, now, reduced)
    }
}

pub(super) fn background() -> Color {
    if crate::ui_theme::interface_style() == crate::graphics::InterfaceStyle::Terminal {
        Color::from_rgba(8, 15, 10, 255)
    } else {
        Color::from_rgba(13, 19, 29, 255)
    }
}
fn mix(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}
fn alpha(mut c: Color, a: f32) -> Color {
    c.a = a;
    c
}
fn round(r: Rect, radius: f32, c: Color) {
    crate::ui_theme::rounded_rectangle(r, radius, c);
}
fn outline(r: Rect, radius: f32, border: Color, fill: Color) {
    round(r, radius, border);
    round(
        Rect::new(r.x + 1., r.y + 1., r.w - 2., r.h - 2.),
        radius - 1.,
        fill,
    );
}

pub(super) fn shell(r: Rect) {
    if super::character_creation_industrial::enabled() {
        super::character_creation_industrial::shell(r);
        return;
    }
    round(
        Rect::new(r.x, r.y + 6., r.w, r.h),
        14.,
        Color::new(0., 0., 0., 0.25),
    );
    outline(r, 14., alpha(UiTheme.muted(), 0.12), background());
    draw_line(
        r.x + 24.,
        r.y + 84.,
        r.right() - 24.,
        r.y + 84.,
        1.,
        alpha(UiTheme.muted(), 0.10),
    );
}
pub(super) fn panel(r: Rect) {
    if super::character_creation_industrial::enabled() {
        super::character_creation_industrial::panel(r);
        return;
    }
    round(
        Rect::new(r.x, r.y + 3., r.w, r.h),
        10.,
        Color::new(0., 0., 0., 0.15),
    );
    outline(r, 10., alpha(UiTheme.muted(), 0.17), UiTheme.surface());
}
pub(super) fn icon_frame(r: Rect) {
    if super::character_creation_industrial::enabled() {
        super::character_creation_industrial::icon_frame(r);
        return;
    }
    outline(
        r,
        8.,
        alpha(UiTheme.muted(), 0.12),
        Color::from_rgba(9, 14, 22, 255),
    );
}

impl AsciiApp {
    #[cfg(debug_assertions)]
    pub(super) async fn capture_creation_polish_animation(
        &mut self,
        output: &std::path::Path,
    ) -> Result<(), String> {
        let fingerprint = suspension::fingerprint(&self.game);
        let turn = self.game.turn();
        let rng = self.game.rng_state();
        let classes = self.character_classes.iter().count();
        let layout = CharacterCreationLayout::new(self.ui_width(), self.ui_height(), classes);
        let viewport = (self.ui_width(), self.ui_height());
        let click = |r: Rect| InputFrame {
            pointer: Some(r.center().into()),
            pressed: [controls::Binding::MouseLeft].into(),
            viewport: Some(viewport),
            ..Default::default()
        };
        // Warm the four nearby class previews before recording the motion itself.
        for i in 0..classes {
            self.update_character_creation(&click(layout.class_rows[i]));
            let _time = crate::effect_geometry::frame_time(1.);
            for _ in 0..2 {
                self.draw();
                next_frame().await;
            }
        }
        self.update_character_creation(&click(layout.class_rows[2]));
        *self.ux.creation_motion.borrow_mut() = State::default();
        let frames = output.join("frames");
        std::fs::create_dir_all(&frames).map_err(|e| e.to_string())?;
        let mut observations = Vec::new();
        for frame in 0..145 {
            if frame == 25 || frame == 105 {
                self.update_character_creation(&click(
                    layout.class_rows[if frame == 25 { 0 } else { 2 }],
                ));
            }
            if frame == 55 || frame == 80 {
                let p = self.ux.creation_details.as_ref().unwrap();
                let details = character_creation_details::DetailsLayout::new(
                    &layout,
                    p.items.len(),
                    p.skills.len(),
                    p.scroll.offset,
                );
                let target = if frame == 55 {
                    details.skills[1]
                } else {
                    details.items[0]
                };
                self.update_character_creation(&click(target));
            }
            let now = 10. + f64::from(frame) / 30.;
            let _time = crate::effect_geometry::frame_time(now);
            for _ in 0..2 {
                self.draw();
                next_frame().await;
            }
            self.draw();
            crate::ui_capture::framebuffer()?
                .export_png(frames.join(format!("{frame:03}.png")).to_str().unwrap());
            let s = self.ux.creation_motion.borrow();
            observations.push(serde_json::json!({"frame":frame,"class_progress":ease((now-s.class_changed)/0.18),"detail_progress":ease((now-s.detail_changed)/0.18),"cached_scalars":s.values.len()}));
        }
        if suspension::fingerprint(&self.game) != fingerprint
            || self.game.turn() != turn
            || self.game.rng_state() != rng
        {
            return Err("Creation animation modified the run".into());
        }
        std::fs::write(output.join("animation-checks.json"),serde_json::to_vec_pretty(&serde_json::json!({"review":"CREATION-UI02-B","frames":145,"sample_fps":30,"fade_ms":180,"selection_ms":140,"real_input_selection_used":true,"game_fingerprint_unchanged":true,"turn_and_rng_unchanged":true,"reduced_motion_supported":true,"normal_game_enabled":true,"observations":observations})).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }
    pub(super) fn creation_polish_begin(&self, creation: &CharacterCreation) {
        if enabled() {
            let now = crate::effect_geometry::presentation_time(get_time());
            let detail = self.ux.creation_details.as_ref().and_then(|p| p.tooltip);
            self.ux
                .creation_motion
                .borrow_mut()
                .observe(creation.selected_class, detail, now);
        }
    }
    pub(super) fn creation_card(&self, r: Rect, key: u16, selected: bool, hovered: bool) {
        if !enabled() {
            UiTheme.card(r, selected);
            return;
        }
        let now = crate::effect_geometry::presentation_time(get_time());
        let reduced = self.graphics.active.reduced_motion;
        let (select, hover) = {
            let mut s = self.ux.creation_motion.borrow_mut();
            (
                s.strength(key, selected, now, reduced),
                s.strength(key + 4000, hovered, now, reduced),
            )
        };
        if super::character_creation_industrial::enabled() {
            super::character_creation_industrial::card(r, key, select, hover);
            return;
        }
        let fill = mix(
            mix(
                UiTheme.surface(),
                UiTheme.surface_raised(),
                0.30 + hover * 0.35,
            ),
            UiTheme.surface_selected(),
            select * 0.85,
        );
        let border = mix(
            alpha(UiTheme.muted(), 0.14 + hover * 0.14),
            alpha(UiTheme.accent(), 0.65),
            select,
        );
        outline(r, 10., border, fill);
        round(
            Rect::new(r.x + 2., r.y + 13., 2., r.h - 26.),
            1.5,
            alpha(UiTheme.accent(), select),
        );
    }
    pub(super) fn creation_button(
        &self,
        r: Rect,
        label: &str,
        key: u16,
        focused: bool,
        enabled_button: bool,
        tone: ButtonTone,
    ) {
        if !enabled() {
            UiTheme.button(r, label, focused, false, enabled_button, tone);
            return;
        }
        let now = crate::effect_geometry::presentation_time(get_time());
        let hover = self.ux.creation_motion.borrow_mut().strength(
            key,
            focused,
            now,
            self.graphics.active.reduced_motion,
        );
        let primary = tone == ButtonTone::Primary && enabled_button;
        if super::character_creation_industrial::enabled() {
            super::character_creation_industrial::button(
                r,
                label,
                primary,
                hover,
                enabled_button,
                focused && is_mouse_button_down(MouseButton::Left),
            );
            return;
        }
        let fill = if primary {
            mix(UiTheme.accent(), UiTheme.text(), hover * 0.18)
        } else {
            mix(UiTheme.surface(), UiTheme.surface_raised(), hover)
        };
        outline(
            r,
            9.,
            if primary {
                fill
            } else {
                alpha(UiTheme.muted(), 0.20 + hover * 0.25)
            },
            fill,
        );
        let mut size = 16;
        while size > 12 && measure_text_bold(label, size).width > r.w - 20. {
            size -= 1;
        }
        draw_text_bold_centered(
            label,
            r,
            size,
            if primary {
                Color::from_rgba(6, 20, 29, 255)
            } else if enabled_button {
                UiTheme.text()
            } else {
                UiTheme.muted()
            },
        );
    }
    pub(super) fn creation_content_cover(&self, r: Rect, reader: bool) {
        if !enabled() || self.graphics.active.reduced_motion {
            return;
        }
        let now = crate::effect_geometry::presentation_time(get_time());
        let s = self.ux.creation_motion.borrow();
        let started = if reader {
            s.detail_changed
        } else {
            s.class_changed
        };
        // A gentle reveal keeps the new text readable, even during rapid clicks.
        let opacity = 0.30 * (1. - ease((now - started) / 0.18));
        if opacity > 0. {
            let r = if reader {
                Rect::new(r.x + 8., r.y + 8., r.w - 16., r.h - 16.)
            } else {
                r
            };
            draw_rectangle(
                r.x,
                r.y,
                r.w,
                r.h,
                alpha(
                    if reader {
                        UiTheme.surface()
                    } else {
                        background()
                    },
                    opacity,
                ),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interrupted_transitions_are_continuous_and_reduced_motion_is_immediate() {
        let mut t = Tween::default();
        assert_eq!(t.aim(1., 1., false), 0.);
        let mid = t.value(1.07);
        assert!((mid - 0.5).abs() < 0.001);
        assert!((t.aim(0., 1.07, false) - mid).abs() < 0.001);
        assert!(t.value(1.10) < mid);
        assert_eq!(t.value(2.), 0.);
        assert_eq!(t.aim(1., 2., true), 1.);
        assert_eq!(t.aim(0., 2.01, true), 0.);
    }
    #[test]
    fn transition_state_is_bounded_and_stable_on_repeated_observations() {
        let mut s = State::default();
        s.observe(1, None, 1.);
        s.observe(1, None, 2.);
        assert_eq!(s.class_changed, 1.);
        s.observe(2, None, 3.);
        assert_eq!(s.class_changed, 3.);
        for key in 0..600 {
            s.strength(key, true, 1., false);
        }
        assert!(s.values.len() <= 128);
    }
}
