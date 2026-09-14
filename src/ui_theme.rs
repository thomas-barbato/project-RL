//! Shared presentation shell for menus and overlays.
//!
//! World glyphs deliberately keep their terminal renderer.  This module owns
//! the readable interface layer that can be reused by terminal and textured
//! world renderers alike.
use std::sync::OnceLock;

use macroquad::prelude::{
    Color, Font, Rect, TextDimensions, TextParams, draw_circle, draw_rectangle, draw_text_ex,
    load_ttf_font_from_bytes,
};

static REGULAR_FONT: OnceLock<Font> = OnceLock::new();
static BOLD_FONT: OnceLock<Font> = OnceLock::new();

const REGULAR_BYTES: &[u8] = include_bytes!("../assets/fonts/AtkinsonHyperlegible-Regular.ttf");
const BOLD_BYTES: &[u8] = include_bytes!("../assets/fonts/AtkinsonHyperlegible-Bold.ttf");

/// Must be called once after Macroquad has created its graphics context.
/// Headless tests intentionally use the built-in fallback instead.
pub fn initialize_fonts() -> Result<(), String> {
    if REGULAR_FONT.get().is_none() {
        let font = load_ttf_font_from_bytes(REGULAR_BYTES).map_err(|error| error.to_string())?;
        let _ = REGULAR_FONT.set(font);
    }
    if BOLD_FONT.get().is_none() {
        let font = load_ttf_font_from_bytes(BOLD_BYTES).map_err(|error| error.to_string())?;
        let _ = BOLD_FONT.set(font);
    }
    Ok(())
}

pub fn regular_font() -> Option<&'static Font> {
    REGULAR_FONT.get()
}

pub fn bold_font() -> Option<&'static Font> {
    BOLD_FONT.get()
}

/// Drop-in UI replacement for Macroquad's default-font drawing helper.
pub fn draw_text(
    text: impl AsRef<str>,
    x: f32,
    y: f32,
    font_size: f32,
    color: Color,
) -> TextDimensions {
    draw_with_font(text, x, y, font_size, color, regular_font())
}

pub fn draw_text_bold(
    text: impl AsRef<str>,
    x: f32,
    y: f32,
    font_size: f32,
    color: Color,
) -> TextDimensions {
    draw_with_font(
        text,
        x,
        y,
        font_size,
        color,
        bold_font().or_else(regular_font),
    )
}

fn draw_with_font(
    text: impl AsRef<str>,
    x: f32,
    y: f32,
    font_size: f32,
    color: Color,
    font: Option<&Font>,
) -> TextDimensions {
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font,
            font_size: font_size.round().clamp(1.0, u16::MAX as f32) as u16,
            color,
            ..Default::default()
        },
    )
}

/// Drop-in UI replacement for `measure_text`; an explicitly supplied font
/// still wins, which keeps the helper useful for future icon fonts.
pub fn measure_text(
    text: impl AsRef<str>,
    font: Option<&Font>,
    font_size: u16,
    font_scale: f32,
) -> TextDimensions {
    let resolved_font = match font {
        Some(font) => Some(font),
        None => regular_font(),
    };
    macroquad::text::measure_text(text.as_ref(), resolved_font, font_size, font_scale)
}

pub fn measure_text_bold(text: impl AsRef<str>, font_size: u16) -> TextDimensions {
    macroquad::text::measure_text(
        text.as_ref(),
        bold_font().or_else(regular_font),
        font_size,
        1.0,
    )
}

/// Positionne la ligne de base à partir des limites réellement rasterisées de
/// la police. Une proportion fixe de la hauteur du bouton donne une impression
/// de décalage différente selon les accents, les capitales et les symboles.
pub fn centered_text_origin(rect: Rect, dimensions: TextDimensions) -> (f32, f32) {
    (
        rect.x + (rect.w - dimensions.width) * 0.5,
        rect.y + (rect.h - dimensions.height) * 0.5 + dimensions.offset_y,
    )
}

pub fn draw_text_bold_centered(
    text: impl AsRef<str>,
    rect: Rect,
    font_size: u16,
    color: Color,
) -> TextDimensions {
    let text = text.as_ref();
    let dimensions = measure_text_bold(text, font_size);
    let (x, y) = centered_text_origin(rect, dimensions);
    draw_text_bold(text, x, y, f32::from(font_size), color)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonTone {
    #[default]
    Secondary,
    Primary,
    Danger,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct UiTheme;

impl UiTheme {
    pub const fn backdrop(self) -> Color {
        Color::new(0.008, 0.027, 0.043, 0.94)
    }

    pub const fn surface(self) -> Color {
        Color::new(0.024, 0.055, 0.075, 0.98)
    }

    pub const fn surface_raised(self) -> Color {
        Color::new(0.045, 0.105, 0.13, 1.0)
    }

    pub const fn surface_selected(self) -> Color {
        Color::new(0.07, 0.23, 0.25, 0.98)
    }

    pub const fn text(self) -> Color {
        Color::new(0.88, 0.94, 0.94, 1.0)
    }

    pub const fn muted(self) -> Color {
        Color::new(0.58, 0.7, 0.72, 1.0)
    }

    pub const fn accent(self) -> Color {
        Color::new(0.39, 0.95, 0.82, 1.0)
    }

    pub const fn focus(self) -> Color {
        Color::new(1.0, 0.83, 0.36, 1.0)
    }

    pub const fn danger(self) -> Color {
        Color::new(1.0, 0.48, 0.39, 1.0)
    }

    pub const fn success(self) -> Color {
        Color::new(0.48, 0.88, 0.59, 1.0)
    }

    pub fn panel(self, rect: Rect) {
        rounded_rectangle(
            Rect::new(rect.x + 4.0, rect.y + 6.0, rect.w, rect.h),
            11.0,
            Color::new(0.0, 0.0, 0.0, 0.42),
        );
        rounded_outline(
            rect,
            11.0,
            1.0,
            subdued(self.accent(), 0.34),
            self.surface(),
        );
    }

    pub fn card(self, rect: Rect, selected: bool) {
        let fill = if selected {
            self.surface_selected()
        } else {
            self.surface_raised()
        };
        rounded_outline(
            rect,
            7.0,
            if selected { 2.0 } else { 1.0 },
            if selected {
                self.focus()
            } else {
                subdued(self.muted(), 0.28)
            },
            fill,
        );
        if selected {
            rounded_rectangle(
                Rect::new(rect.x + 7.0, rect.y + 7.0, 3.0, (rect.h - 14.0).max(3.0)),
                1.5,
                self.focus(),
            );
        }
    }

    pub fn button(
        self,
        rect: Rect,
        label: &str,
        focused: bool,
        active: bool,
        enabled: bool,
        tone: ButtonTone,
    ) {
        let semantic = match tone {
            ButtonTone::Secondary => self.accent(),
            ButtonTone::Primary => self.focus(),
            ButtonTone::Danger => self.danger(),
        };
        let fill = if !enabled {
            Color::new(0.035, 0.06, 0.07, 0.9)
        } else if focused || active {
            match tone {
                ButtonTone::Secondary => self.surface_selected(),
                ButtonTone::Primary => Color::new(0.20, 0.18, 0.075, 1.0),
                ButtonTone::Danger => Color::new(0.20, 0.075, 0.07, 1.0),
            }
        } else {
            self.surface_raised()
        };
        let outline = if !enabled {
            Color::new(0.22, 0.27, 0.28, 0.55)
        } else if focused || active {
            semantic
        } else {
            subdued(self.muted(), 0.32)
        };
        let radius = (rect.h * 0.2).clamp(4.0, 8.0);

        rounded_rectangle(
            Rect::new(rect.x + 1.0, rect.y + 3.0, rect.w, rect.h),
            radius,
            Color::new(0.0, 0.0, 0.0, if enabled { 0.38 } else { 0.22 }),
        );
        rounded_outline(
            rect,
            radius,
            if focused || active { 2.0 } else { 1.0 },
            outline,
            fill,
        );

        if enabled && (focused || active || tone != ButtonTone::Secondary) {
            let rail_width = if active || tone != ButtonTone::Secondary {
                34.0
            } else {
                22.0
            };
            rounded_rectangle(
                Rect::new(
                    rect.x + (rect.w - rail_width) * 0.5,
                    rect.y + rect.h - 4.0,
                    rail_width,
                    2.0,
                ),
                1.0,
                semantic,
            );
        }

        let mut font_size = 16_u16;
        while font_size > 12 && measure_text_bold(label, font_size).width > rect.w - 20.0 {
            font_size -= 1;
        }
        draw_text_bold_centered(
            label,
            Rect::new(rect.x + 8.0, rect.y, rect.w - 16.0, rect.h - 1.0),
            font_size,
            if !enabled {
                Color::new(0.42, 0.49, 0.50, 1.0)
            } else if focused || active {
                semantic
            } else {
                self.text()
            },
        );
    }
}

fn subdued(color: Color, alpha: f32) -> Color {
    Color::new(color.r, color.g, color.b, alpha)
}

fn rounded_outline(rect: Rect, radius: f32, thickness: f32, outline: Color, fill: Color) {
    rounded_rectangle(rect, radius, outline);
    let inset = thickness.max(0.0).min(rect.w * 0.5).min(rect.h * 0.5);
    if inset > 0.0 {
        rounded_rectangle(
            Rect::new(
                rect.x + inset,
                rect.y + inset,
                (rect.w - inset * 2.0).max(0.0),
                (rect.h - inset * 2.0).max(0.0),
            ),
            (radius - inset).max(0.0),
            fill,
        );
    }
}

fn rounded_rectangle(rect: Rect, radius: f32, color: Color) {
    let radius = radius.max(0.0).min(rect.w * 0.5).min(rect.h * 0.5);
    if radius <= 0.0 {
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, color);
        return;
    }
    draw_rectangle(
        rect.x + radius,
        rect.y,
        rect.w - radius * 2.0,
        rect.h,
        color,
    );
    draw_rectangle(
        rect.x,
        rect.y + radius,
        rect.w,
        rect.h - radius * 2.0,
        color,
    );
    draw_circle(rect.x + radius, rect.y + radius, radius, color);
    draw_circle(rect.x + rect.w - radius, rect.y + radius, radius, color);
    draw_circle(rect.x + radius, rect.y + rect.h - radius, radius, color);
    draw_circle(
        rect.x + rect.w - radius,
        rect.y + rect.h - radius,
        radius,
        color,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn centered_origin_uses_the_rasterized_bounds_in_both_axes() {
        let rect = Rect::new(20.0, 30.0, 100.0, 40.0);
        let dimensions = TextDimensions {
            width: 48.0,
            height: 18.0,
            offset_y: 15.0,
        };
        let (x, baseline) = centered_text_origin(rect, dimensions);
        assert_eq!(x, 46.0);
        assert_eq!(baseline, 56.0);
        assert_eq!(baseline - dimensions.offset_y, 41.0);
    }
}
