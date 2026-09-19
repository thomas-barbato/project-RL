//! Shared presentation shell for menus and overlays.
//!
//! World glyphs deliberately keep their terminal renderer.  This module owns
//! the readable interface layer that can be reused by terminal and textured
//! world renderers alike.
use std::sync::OnceLock;

use macroquad::prelude::{
    Color, Font, Rect, TextDimensions, TextParams, draw_circle, draw_circle_lines, draw_line,
    draw_rectangle, draw_rectangle_lines, draw_text_ex, draw_triangle, load_ttf_font_from_bytes,
    vec2,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ButtonState {
    focused: bool,
    active: bool,
    enabled: bool,
    tone: ButtonTone,
}

impl ButtonState {
    pub const fn new(focused: bool, active: bool, enabled: bool, tone: ButtonTone) -> Self {
        Self {
            focused,
            active,
            enabled,
            tone,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiIcon {
    Play,
    Add,
    Settings,
    Power,
    Save,
    Abandon,
    Controls,
    Display,
    Back,
    Confirm,
    Cancel,
    Reset,
    Interact,
    Attack,
    Techniques,
    Inventory,
    Help,
    Menu,
    Sort,
    All,
    Weapon,
    Armor,
    Consumable,
    Material,
    Equip,
    Use,
    Drop,
    Character,
    Level,
    Health,
    Energy,
    Bandwidth,
    Heat,
    Target,
    Window,
    Grid,
    Contrast,
    Motion,
    Wait,
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
            if tone == ButtonTone::Danger {
                Color::new(0.16, 0.045, 0.045, 0.94)
            } else {
                Color::new(0.035, 0.06, 0.07, 0.9)
            }
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
            if tone == ButtonTone::Danger {
                subdued(self.danger(), 0.78)
            } else {
                Color::new(0.22, 0.27, 0.28, 0.55)
            }
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
                if tone == ButtonTone::Danger {
                    self.danger()
                } else {
                    Color::new(0.42, 0.49, 0.50, 1.0)
                }
            } else if focused || active {
                semantic
            } else {
                self.text()
            },
        );
    }

    pub fn button_with_icon(self, rect: Rect, label: &str, icon: UiIcon, state: ButtonState) {
        self.button(
            rect,
            "",
            state.focused,
            state.active,
            state.enabled,
            state.tone,
        );
        let color = button_foreground(self, state.focused, state.active, state.enabled, state.tone);
        let compact = rect.w < 140.0;
        let icon_size = if compact {
            (rect.h - 14.0).clamp(12.0, 15.0)
        } else {
            (rect.h - 13.0).clamp(13.0, 22.0)
        };
        let icon_margin = if compact { 5.0 } else { 12.0 };
        draw_ui_icon(
            icon,
            Rect::new(
                rect.x + icon_margin,
                rect.y + (rect.h - icon_size) * 0.5,
                icon_size,
                icon_size,
            ),
            color,
        );
        let label_start = if compact {
            icon_size + 9.0
        } else {
            icon_size + 24.0
        };
        let label_end_margin = if compact { 4.0 } else { 12.0 };
        let label_rect = Rect::new(
            rect.x + label_start,
            rect.y,
            (rect.w - label_start - label_end_margin).max(20.0),
            rect.h - 1.0,
        );
        let mut font_size = 16_u16;
        while font_size > 12 && measure_text_bold(label, font_size).width > label_rect.w - 8.0 {
            font_size -= 1;
        }
        draw_text_bold_centered(label, label_rect, font_size, color);
    }
}

fn button_foreground(
    theme: UiTheme,
    focused: bool,
    active: bool,
    enabled: bool,
    tone: ButtonTone,
) -> Color {
    if !enabled {
        if tone == ButtonTone::Danger {
            theme.danger()
        } else {
            Color::new(0.42, 0.49, 0.50, 1.0)
        }
    } else if focused || active {
        match tone {
            ButtonTone::Secondary => theme.accent(),
            ButtonTone::Primary => theme.focus(),
            ButtonTone::Danger => theme.danger(),
        }
    } else {
        theme.text()
    }
}

pub fn draw_ui_icon(icon: UiIcon, rect: Rect, color: Color) {
    let size = rect.w.min(rect.h);
    let cx = rect.x + rect.w * 0.5;
    let cy = rect.y + rect.h * 0.5;
    let radius = size * 0.34;
    let line = (size * 0.095).clamp(1.2, 2.2);
    let left = cx - radius;
    let right = cx + radius;
    let top = cy - radius;
    let bottom = cy + radius;
    match icon {
        UiIcon::Play | UiIcon::Use => draw_triangle(
            vec2(cx - radius * 0.55, top),
            vec2(right, cy),
            vec2(cx - radius * 0.55, bottom),
            color,
        ),
        UiIcon::Add => {
            draw_circle_lines(cx, cy, radius, line, color);
            draw_line(
                left + radius * 0.45,
                cy,
                right - radius * 0.45,
                cy,
                line,
                color,
            );
            draw_line(
                cx,
                top + radius * 0.45,
                cx,
                bottom - radius * 0.45,
                line,
                color,
            );
        }
        UiIcon::Settings => {
            draw_circle_lines(cx, cy, radius * 0.48, line, color);
            for (dx, dy) in [(0.0, -1.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 0.0)] {
                draw_line(
                    cx + dx * radius * 0.58,
                    cy + dy * radius * 0.58,
                    cx + dx * radius,
                    cy + dy * radius,
                    line,
                    color,
                );
            }
        }
        UiIcon::Power => {
            draw_circle_lines(cx, cy + radius * 0.12, radius * 0.82, line, color);
            draw_line(cx, top, cx, cy, line, color);
        }
        UiIcon::Save => {
            draw_rectangle_lines(left, top, radius * 2.0, radius * 2.0, line, color);
            draw_rectangle_lines(
                cx - radius * 0.45,
                top,
                radius * 0.9,
                radius * 0.65,
                line,
                color,
            );
            draw_rectangle_lines(
                cx - radius * 0.5,
                cy + radius * 0.18,
                radius,
                radius * 0.62,
                line,
                color,
            );
        }
        UiIcon::Abandon | UiIcon::Cancel => {
            draw_line(left, top, right, bottom, line, color);
            draw_line(right, top, left, bottom, line, color);
        }
        UiIcon::Controls => {
            draw_rectangle_lines(
                left,
                cy - radius * 0.72,
                radius * 2.0,
                radius * 1.44,
                line,
                color,
            );
            for column in 0..3 {
                let x = left + radius * (0.45 + column as f32 * 0.55);
                draw_circle(x, cy - radius * 0.25, line * 0.72, color);
            }
            draw_line(
                left + radius * 0.35,
                cy + radius * 0.28,
                right - radius * 0.35,
                cy + radius * 0.28,
                line,
                color,
            );
        }
        UiIcon::Display | UiIcon::Window => {
            draw_rectangle_lines(left, top, radius * 2.0, radius * 1.45, line, color);
            draw_line(cx, cy + radius * 0.45, cx, bottom, line, color);
            draw_line(
                cx - radius * 0.5,
                bottom,
                cx + radius * 0.5,
                bottom,
                line,
                color,
            );
        }
        UiIcon::Back => {
            draw_line(right, cy, left, cy, line, color);
            draw_line(left, cy, cx - radius * 0.25, top, line, color);
            draw_line(left, cy, cx - radius * 0.25, bottom, line, color);
        }
        UiIcon::Confirm | UiIcon::Equip => {
            draw_line(left, cy, cx - radius * 0.15, bottom, line, color);
            draw_line(cx - radius * 0.15, bottom, right, top, line, color);
        }
        UiIcon::Reset => {
            draw_circle_lines(cx, cy, radius * 0.82, line, color);
            draw_triangle(
                vec2(left - line, cy - radius * 0.2),
                vec2(left + radius * 0.55, cy - radius * 0.55),
                vec2(left + radius * 0.45, cy + radius * 0.1),
                color,
            );
        }
        UiIcon::Interact => {
            draw_circle_lines(cx, cy, radius * 0.34, line, color);
            for (dx, dy) in [(0.0, -1.0), (1.0, 0.0), (0.0, 1.0), (-1.0, 0.0)] {
                draw_line(
                    cx + dx * radius * 0.55,
                    cy + dy * radius * 0.55,
                    cx + dx * radius,
                    cy + dy * radius,
                    line,
                    color,
                );
            }
        }
        UiIcon::Attack => {
            draw_circle_lines(cx, cy, radius * 0.55, line, color);
            draw_line(left, cy, cx - radius * 0.25, cy, line, color);
            draw_line(cx + radius * 0.25, cy, right, cy, line, color);
            draw_line(cx, top, cx, cy - radius * 0.25, line, color);
            draw_line(cx, cy + radius * 0.25, cx, bottom, line, color);
        }
        UiIcon::Techniques => {
            let points = [
                vec2(cx + radius * 0.05, top),
                vec2(left + radius * 0.35, cy + radius * 0.08),
                vec2(cx - radius * 0.05, cy + radius * 0.08),
                vec2(cx - radius * 0.18, bottom),
                vec2(right - radius * 0.2, cy - radius * 0.12),
                vec2(cx + radius * 0.15, cy - radius * 0.12),
            ];
            for edge in points.windows(2) {
                draw_line(edge[0].x, edge[0].y, edge[1].x, edge[1].y, line, color);
            }
        }
        UiIcon::Inventory => {
            draw_rectangle_lines(
                left,
                cy - radius * 0.52,
                radius * 2.0,
                radius * 1.5,
                line,
                color,
            );
            draw_line(
                cx - radius * 0.4,
                cy - radius * 0.52,
                cx - radius * 0.18,
                top,
                line,
                color,
            );
            draw_line(
                cx + radius * 0.4,
                cy - radius * 0.52,
                cx + radius * 0.18,
                top,
                line,
                color,
            );
        }
        UiIcon::Help => {
            draw_circle_lines(cx, cy, radius, line, color);
            draw_line(cx, top + radius * 0.38, cx, cy + radius * 0.1, line, color);
            draw_circle(cx, bottom - radius * 0.28, line * 0.72, color);
        }
        UiIcon::Menu => {
            for offset in [-0.62, 0.0, 0.62] {
                draw_line(
                    left,
                    cy + radius * offset,
                    right,
                    cy + radius * offset,
                    line,
                    color,
                );
            }
        }
        UiIcon::Sort => {
            for (index, width) in [1.0, 0.72, 0.44].into_iter().enumerate() {
                let y = top + radius * (0.25 + index as f32 * 0.72);
                draw_line(left, y, left + radius * 2.0 * width, y, line, color);
            }
        }
        UiIcon::All | UiIcon::Grid => {
            let cell = radius * 0.72;
            for row in 0..2 {
                for column in 0..2 {
                    draw_rectangle_lines(
                        cx - radius + column as f32 * radius * 1.08,
                        cy - radius + row as f32 * radius * 1.08,
                        cell,
                        cell,
                        line,
                        color,
                    );
                }
            }
        }
        UiIcon::Weapon => {
            draw_line(left, bottom, right, top, line * 1.25, color);
            draw_line(
                left,
                bottom - radius * 0.55,
                left + radius * 0.55,
                bottom,
                line,
                color,
            );
            draw_line(
                left,
                bottom,
                left + radius * 0.25,
                bottom - radius * 0.25,
                line,
                color,
            );
        }
        UiIcon::Armor => {
            let points = [
                vec2(cx, top),
                vec2(right, top + radius * 0.42),
                vec2(right - radius * 0.2, bottom - radius * 0.25),
                vec2(cx, bottom),
                vec2(left + radius * 0.2, bottom - radius * 0.25),
                vec2(left, top + radius * 0.42),
                vec2(cx, top),
            ];
            for edge in points.windows(2) {
                draw_line(edge[0].x, edge[0].y, edge[1].x, edge[1].y, line, color);
            }
        }
        UiIcon::Consumable => {
            draw_line(
                cx - radius * 0.34,
                top,
                cx + radius * 0.34,
                top,
                line,
                color,
            );
            draw_rectangle_lines(
                cx - radius * 0.52,
                top + radius * 0.38,
                radius * 1.04,
                radius * 1.52,
                line,
                color,
            );
            draw_line(
                cx - radius * 0.45,
                cy + radius * 0.25,
                cx + radius * 0.45,
                cy + radius * 0.25,
                line,
                color,
            );
        }
        UiIcon::Material => {
            draw_circle_lines(cx, cy, radius, line, color);
            draw_circle_lines(cx, cy, radius * 0.38, line, color);
        }
        UiIcon::Drop => {
            draw_line(cx, top, cx, bottom - radius * 0.35, line, color);
            draw_line(
                cx,
                bottom,
                left + radius * 0.25,
                cy + radius * 0.2,
                line,
                color,
            );
            draw_line(
                cx,
                bottom,
                right - radius * 0.25,
                cy + radius * 0.2,
                line,
                color,
            );
        }
        UiIcon::Character => {
            draw_circle(cx, top + radius * 0.48, radius * 0.36, color);
            draw_circle_lines(cx, bottom, radius * 0.78, line, color);
        }
        UiIcon::Level => {
            draw_line(left, cy + radius * 0.25, cx, top, line, color);
            draw_line(cx, top, right, cy + radius * 0.25, line, color);
            draw_line(cx, top, cx, bottom, line, color);
            draw_line(left, bottom, right, bottom, line, color);
        }
        UiIcon::Health => {
            draw_circle(cx - radius * 0.38, cy - radius * 0.22, radius * 0.5, color);
            draw_circle(cx + radius * 0.38, cy - radius * 0.22, radius * 0.5, color);
            draw_triangle(
                vec2(left - radius * 0.02, cy - radius * 0.05),
                vec2(right + radius * 0.02, cy - radius * 0.05),
                vec2(cx, bottom),
                color,
            );
        }
        UiIcon::Energy => {
            let points = [
                vec2(cx + radius * 0.05, top),
                vec2(left + radius * 0.35, cy + radius * 0.08),
                vec2(cx - radius * 0.05, cy + radius * 0.08),
                vec2(cx - radius * 0.18, bottom),
                vec2(right - radius * 0.2, cy - radius * 0.12),
                vec2(cx + radius * 0.15, cy - radius * 0.12),
            ];
            for edge in points.windows(2) {
                draw_line(edge[0].x, edge[0].y, edge[1].x, edge[1].y, line, color);
            }
        }
        UiIcon::Bandwidth => {
            for (index, height) in [0.45, 0.72, 1.0].into_iter().enumerate() {
                let x = left + radius * (0.28 + index as f32 * 0.72);
                draw_line(x, bottom, x, bottom - radius * 2.0 * height, line, color);
            }
        }
        UiIcon::Heat => {
            draw_circle_lines(cx, bottom - radius * 0.25, radius * 0.42, line, color);
            draw_line(cx, top, cx, bottom - radius * 0.55, line * 1.4, color);
            draw_circle(cx, bottom - radius * 0.25, radius * 0.18, color);
        }
        UiIcon::Target => {
            draw_circle_lines(cx, cy, radius * 0.62, line, color);
            draw_line(left, cy, cx - radius * 0.3, cy, line, color);
            draw_line(cx + radius * 0.3, cy, right, cy, line, color);
            draw_line(cx, top, cx, cy - radius * 0.3, line, color);
            draw_line(cx, cy + radius * 0.3, cx, bottom, line, color);
        }
        UiIcon::Contrast => {
            draw_circle_lines(cx, cy, radius, line, color);
            draw_line(cx, top, cx, bottom, line, color);
            draw_circle(cx - radius * 0.42, cy, radius * 0.23, color);
        }
        UiIcon::Motion => {
            draw_line(
                left,
                cy - radius * 0.38,
                right,
                cy - radius * 0.38,
                line,
                color,
            );
            draw_line(
                left,
                cy + radius * 0.38,
                right,
                cy + radius * 0.38,
                line,
                color,
            );
            draw_triangle(
                vec2(right, cy - radius * 0.38),
                vec2(right - radius * 0.48, top),
                vec2(right - radius * 0.48, cy),
                color,
            );
            draw_triangle(
                vec2(left, cy + radius * 0.38),
                vec2(left + radius * 0.48, cy),
                vec2(left + radius * 0.48, bottom),
                color,
            );
        }
        UiIcon::Wait => {
            draw_line(left, top, right, top, line, color);
            draw_line(left, bottom, right, bottom, line, color);
            draw_line(
                left + line,
                top + line,
                right - line,
                bottom - line,
                line,
                color,
            );
            draw_line(
                right - line,
                top + line,
                left + line,
                bottom - line,
                line,
                color,
            );
        }
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
