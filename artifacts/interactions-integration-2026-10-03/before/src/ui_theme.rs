//! Shared presentation shell for menus and overlays.
//!
//! Interface accents are independent of the world's material and actor colors.
use std::collections::{BTreeMap, HashMap};
use std::sync::OnceLock;

use macroquad::prelude::{
    Color, Font, Rect, TextDimensions, TextParams, draw_circle, draw_circle_lines, draw_line,
    draw_rectangle, draw_rectangle_lines, draw_text_ex, draw_triangle, load_ttf_font_from_bytes,
    vec2,
};

static REGULAR_FONT: OnceLock<Font> = OnceLock::new();
static BOLD_FONT: OnceLock<Font> = OnceLock::new();

thread_local! {
    static TEXT_PANE: std::cell::Cell<Option<(Rect, f32)>> = const { std::cell::Cell::new(None) };
    static HIGH_CONTRAST: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static INTERFACE_THEME: std::cell::Cell<crate::graphics::InterfaceTheme> = const { std::cell::Cell::new(crate::graphics::InterfaceTheme::Violet) };
    static TEXT_MEASUREMENTS: std::cell::RefCell<TextMeasurements> = std::cell::RefCell::new(TextMeasurements::default());
}

/// Fonts are immutable after startup. Bound both entry count and string length
/// so dynamic logs/mod text cannot turn this presentation cache into a leak.
#[derive(Default)]
struct TextMeasurements {
    fonts: BTreeMap<(bool, u16, u32), HashMap<String, TextDimensions>>,
    count: usize,
    dpi: u32,
}

impl TextMeasurements {
    fn set_dpi(&mut self, dpi: f32) {
        if self.dpi != dpi.to_bits() {
            self.fonts.clear();
            self.count = 0;
            self.dpi = dpi.to_bits();
        }
    }
    fn get_or_measure(
        &mut self,
        text: &str,
        bold: bool,
        size: u16,
        scale: f32,
        measure: impl FnOnce() -> TextDimensions,
    ) -> TextDimensions {
        if text.len() > 1024 {
            return measure();
        }
        let key = (bold, size, scale.to_bits());
        if let Some(value) = self.fonts.get(&key).and_then(|entries| entries.get(text)) {
            return *value;
        }
        let value = measure();
        if self.count >= 8192 {
            self.fonts.clear();
            self.count = 0;
        }
        self.fonts
            .entry(key)
            .or_default()
            .insert(text.to_owned(), value);
        self.count += 1;
        value
    }
}

fn measure_builtin(text: &str, bold: bool, size: u16, scale: f32) -> TextDimensions {
    let font = if bold {
        bold_font().or_else(regular_font)
    } else {
        regular_font()
    };
    let measure = || macroquad::text::measure_text(text, font, size, scale);
    if font.is_none() {
        return measure();
    }
    TEXT_MEASUREMENTS.with_borrow_mut(|cache| {
        cache.set_dpi(macroquad::window::screen_dpi_scale());
        cache.get_or_measure(text, bold, size, scale, measure)
    })
}

pub fn set_high_contrast(enabled: bool) {
    HIGH_CONTRAST.set(enabled);
}
pub fn set_interface_theme(theme: crate::graphics::InterfaceTheme) {
    INTERFACE_THEME.set(theme);
}
pub fn begin_text_pane(rect: Rect, offset: f32) {
    TEXT_PANE.set(Some((rect, offset)));
}
pub fn end_text_pane() {
    TEXT_PANE.set(None);
}
pub fn text_pane_active() -> bool {
    TEXT_PANE.get().is_some()
}

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
    let bold = font.is_some_and(|font| bold_font().is_some_and(|bold| std::ptr::eq(font, bold)));
    let size = font_size.round().clamp(1.0, u16::MAX as f32) as u16;
    // Prepare the entire string before drawing its first glyph. Otherwise
    // Macroquad can upload the complete atlas again for EACH missing glyph.
    // Cache hits avoid repeating the glyph-by-glyph measurement on later frames.
    let dimensions = measure_builtin(text.as_ref(), bold, size, 1.0);
    let mut y = y;
    if let Some((rect, offset)) = TEXT_PANE.get() {
        y -= offset;
        // Draw complete lines only; text never leaks into headers or actions.
        if y - font_size < rect.y || y + 4.0 > rect.bottom() {
            return dimensions;
        }
    }
    draw_text_ex(
        text,
        x,
        y,
        TextParams {
            font,
            font_size: size,
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
    if let Some(font) = font {
        return macroquad::text::measure_text(text.as_ref(), Some(font), font_size, font_scale);
    }
    measure_builtin(text.as_ref(), false, font_size, font_scale)
}

pub fn measure_text_bold(text: impl AsRef<str>, font_size: u16) -> TextDimensions {
    measure_builtin(text.as_ref(), true, font_size, 1.0)
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
    let (text, font_size, dimensions) = fitted_label(text.as_ref(), rect, font_size, true);
    let (x, y) = centered_text_origin(rect, dimensions);
    draw_text_bold(text, x, y, f32::from(font_size), color)
}

/// A single label stays inside its allotted content box, including descenders.
/// Long labels retain a readable size and use an ellipsis instead of overflowing.
fn fitted_label(text: &str, rect: Rect, size: u16, bold: bool) -> (String, u16, TextDimensions) {
    let measure = |text: &str, size| {
        if bold {
            measure_text_bold(text, size)
        } else {
            measure_text(text, None, size, 1.0)
        }
    };
    let mut size = size.max(1);
    let mut label = text.to_owned();
    let minimum = size.min(12);
    while size > minimum {
        let dimensions = measure(&label, size);
        if dimensions.width <= rect.w && dimensions.height <= rect.h {
            break;
        }
        size -= 1;
    }
    while size > 1 && measure(&label, size).height > rect.h.max(1.0) {
        size -= 1;
    }
    if measure(&label, size).width > rect.w.max(0.0) {
        while !label.is_empty() && measure(&format!("{label}…"), size).width > rect.w.max(0.0) {
            label.pop();
        }
        if measure("…", size).width <= rect.w.max(0.0) {
            label.push('…');
        }
    }
    let dimensions = measure(&label, size);
    (label, size, dimensions)
}

pub fn draw_text_in_rect(
    text: impl AsRef<str>,
    rect: Rect,
    size: u16,
    color: Color,
) -> TextDimensions {
    let (text, size, dimensions) = fitted_label(text.as_ref(), rect, size, false);
    let (_, y) = centered_text_origin(rect, dimensions);
    draw_text(text, rect.x, y, f32::from(size), color)
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
#[allow(dead_code)] // Shared icon family, including variants used by alternate HUD layouts.
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
    Quest,
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
        Color::new(0.008, 0.008, 0.012, 0.96)
    }

    pub const fn surface(self) -> Color {
        Color::from_rgba(8, 9, 11, 255)
    }

    pub const fn surface_raised(self) -> Color {
        Color::from_rgba(17, 19, 22, 255)
    }

    pub fn surface_selected(self) -> Color {
        let accent = self.accent();
        Color::new(
            0.035 + accent.r * 0.10,
            0.035 + accent.g * 0.10,
            0.04 + accent.b * 0.10,
            1.0,
        )
    }

    pub const fn text(self) -> Color {
        Color::new(0.88, 0.94, 0.94, 1.0)
    }

    pub fn muted(self) -> Color {
        if HIGH_CONTRAST.get() {
            Color::new(0.80, 0.87, 0.88, 1.0)
        } else {
            Color::from_rgba(164, 170, 178, 255)
        }
    }

    pub fn accent(self) -> Color {
        use crate::graphics::InterfaceTheme;
        match INTERFACE_THEME.get() {
            InterfaceTheme::Blue => Color::from_rgba(54, 185, 255, 255),
            InterfaceTheme::Violet => Color::from_rgba(151, 128, 255, 255),
            InterfaceTheme::Green => Color::from_rgba(135, 232, 117, 255),
            InterfaceTheme::Red => Color::from_rgba(255, 110, 138, 255),
        }
    }

    pub fn focus(self) -> Color {
        self.accent()
    }

    pub const fn attention(self) -> Color {
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
        rounded_outline(rect, 11.0, 1.0, subdued(self.muted(), 0.20), self.surface());
        surface_relief(rect, 11.0);
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
                subdued(self.muted(), 0.12)
            },
            fill,
        );
        surface_relief(rect, 7.0);
    }

    pub fn hud_panel(self, rect: Rect) {
        rounded_outline(
            rect,
            3.0,
            1.0,
            Color::from_rgba(44, 48, 55, 255),
            self.surface(),
        );
    }

    pub fn hud_chip(self, rect: Rect) {
        rounded_rectangle(rect, 3.0, self.surface_raised());
    }

    /// Compact floating menus have a quiet rim, soft shadow and opaque content.
    pub fn context_surface(self, rect: Rect) {
        for (spread, alpha) in [(9.0, 0.035), (6.0, 0.06), (3.0, 0.12)] {
            rounded_rectangle(
                Rect::new(
                    rect.x - spread,
                    rect.y + 4.0 - spread,
                    rect.w + spread * 2.0,
                    rect.h + spread * 2.0,
                ),
                12.0 + spread,
                Color::new(0.0, 0.0, 0.0, alpha),
            );
        }
        rounded_outline(
            rect,
            12.0,
            1.0,
            subdued(self.muted(), 0.25),
            self.surface_raised(),
        );
        rounded_rectangle(
            Rect::new(rect.x + 1.0, rect.y + 1.0, rect.w - 2.0, rect.h - 2.0),
            11.0,
            self.surface(),
        );
    }

    pub fn context_row(self, rect: Rect, highlighted: bool, enabled: bool) {
        if highlighted {
            rounded_outline(
                rect,
                6.0,
                1.0,
                subdued(if enabled { self.accent() } else { self.muted() }, 0.20),
                if enabled {
                    self.surface_selected()
                } else {
                    self.surface_raised()
                },
            );
        }
    }

    pub fn hud_action(self, rect: Rect, hover: f32) {
        let hover = hover.clamp(0.0, 1.0);
        surface_shadow(rect, 5.0);
        rounded_rectangle(
            rect,
            5.0,
            if hover > 0.0 {
                self.surface_selected()
            } else {
                self.surface_raised()
            },
        );
    }

    pub fn hud_alert(self, rect: Rect, pulse: f32) {
        let pulse = pulse.clamp(0.0, 1.0);
        rounded_outline(
            rect,
            8.0,
            1.0 + pulse * 0.6,
            Color::new(0.88, 0.32 + pulse * 0.08, 0.22, 0.78),
            Color::new(0.28, 0.075, 0.055, 0.97),
        );
    }

    /// Dialogue choices use a filled state and a small chevron instead of rails or box outlines.
    pub fn dialogue_choice(self, rect: Rect, selected: bool, hovered: bool) {
        let fill = if hovered || selected {
            self.surface_selected()
        } else {
            self.surface_raised()
        };
        rounded_rectangle(rect, 6.0, fill);
        if selected || hovered {
            let x = rect.x + 11.0;
            let y = rect.y + rect.h * 0.5;
            draw_line(x, y - 4.0, x + 4.0, y, 1.5, self.accent());
            draw_line(x + 4.0, y, x, y + 4.0, 1.5, self.accent());
        }
    }

    pub fn dialogue_action(self, rect: Rect, label: &str, hovered: bool, enabled: bool) {
        let fill = if !enabled {
            self.surface()
        } else if hovered {
            self.surface_selected()
        } else {
            self.surface_raised()
        };
        rounded_rectangle(rect, 6.0, fill);
        let color = if !enabled {
            self.muted()
        } else if hovered {
            self.accent()
        } else {
            self.text()
        };
        let mut font_size = 16_u16;
        while font_size > 12 && measure_text_bold(label, font_size).width > rect.w - 20.0 {
            font_size -= 1;
        }
        draw_text_bold_centered(label, rect, font_size, color);
    }

    pub fn dialogue_action_with_icon(
        self,
        rect: Rect,
        label: &str,
        icon: UiIcon,
        hovered: bool,
        enabled: bool,
    ) {
        self.dialogue_action(rect, "", hovered, enabled);
        let color = if !enabled {
            self.muted()
        } else if hovered {
            self.accent()
        } else {
            self.text()
        };
        let icon_size = (rect.h - 14.0).clamp(12.0, 21.0);
        draw_ui_icon(
            icon,
            Rect::new(
                rect.x + 12.0,
                rect.y + (rect.h - icon_size) * 0.5,
                icon_size,
                icon_size,
            ),
            color,
        );
        let label_rect = Rect::new(
            rect.x + icon_size + 20.0,
            rect.y,
            rect.w - icon_size - 28.0,
            rect.h,
        );
        let mut font_size = 15_u16;
        while font_size > 12 && measure_text_bold(label, font_size).width > label_rect.w - 8.0 {
            font_size -= 1;
        }
        draw_text_bold_centered(label, label_rect, font_size, color);
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
            ButtonTone::Secondary | ButtonTone::Primary => self.accent(),
            ButtonTone::Danger => self.danger(),
        };
        let fill = if !enabled {
            self.surface()
        } else {
            match tone {
                ButtonTone::Primary => {
                    if focused {
                        let accent = self.accent();
                        Color::new(
                            (accent.r + 0.12).min(1.0),
                            (accent.g + 0.12).min(1.0),
                            (accent.b + 0.12).min(1.0),
                            1.0,
                        )
                    } else {
                        self.accent()
                    }
                }
                ButtonTone::Danger => Color::new(0.25, 0.085, 0.08, 1.0),
                ButtonTone::Secondary if focused || active => self.surface_selected(),
                ButtonTone::Secondary => self.surface_raised(),
            }
        };
        let outline = if !enabled {
            subdued(self.muted(), 0.15)
        } else if focused {
            self.text()
        } else if active || tone != ButtonTone::Secondary {
            semantic
        } else {
            subdued(self.muted(), 0.15)
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
        surface_relief(rect, radius);

        let mut font_size = 16_u16;
        while font_size > 12 && measure_text_bold(label, font_size).width > rect.w - 20.0 {
            font_size -= 1;
        }
        draw_text_bold_centered(
            label,
            Rect::new(rect.x + 8.0, rect.y, rect.w - 16.0, rect.h - 1.0),
            font_size,
            button_foreground(self, focused, active, enabled, tone),
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
        // On narrow filters, keep the full name before the redundant icon.
        let label_start = if compact {
            icon_size + 9.0
        } else {
            icon_size + 24.0
        };
        let label_end_margin = if compact { 4.0 } else { 12.0 };
        if measure_text_bold(label, 12).width > rect.w - label_start - label_end_margin {
            draw_text_bold_centered(
                label,
                Rect::new(
                    rect.x + 6.0,
                    rect.y + 3.0,
                    (rect.w - 12.0).max(0.0),
                    rect.h - 6.0,
                ),
                16,
                color,
            );
            return;
        }
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

    /// The title screen uses quiet surfaces and the chosen focus accent.
    /// Keeping the label centered leaves the icon and chevron as secondary cues.
    pub fn main_menu_action(
        self,
        rect: Rect,
        label: &str,
        icon: UiIcon,
        focused: bool,
        enabled: bool,
        primary: bool,
    ) {
        let fill = if !enabled {
            self.surface()
        } else if primary {
            self.accent()
        } else if focused {
            self.surface_selected()
        } else {
            self.surface_raised()
        };
        rounded_rectangle(rect, 7.0, fill);
        let foreground = if !enabled {
            Color::new(0.45, 0.53, 0.55, 1.0)
        } else if primary {
            self.surface()
        } else if focused {
            self.accent()
        } else {
            self.text()
        };
        draw_ui_icon(
            icon,
            Rect::new(rect.x + 18.0, rect.y + (rect.h - 19.0) * 0.5, 19.0, 19.0),
            foreground,
        );
        let label_area = Rect::new(rect.x + 49.0, rect.y, rect.w - 98.0, rect.h);
        let mut font_size = 17_u16;
        while font_size > 12 && measure_text_bold(label, font_size).width > label_area.w - 8.0 {
            font_size -= 1;
        }
        draw_text_bold_centered(label, label_area, font_size, foreground);
        if enabled {
            let mid_y = rect.y + rect.h * 0.5;
            let right = rect.x + rect.w - 20.0;
            let chevron = foreground;
            draw_line(right - 5.0, mid_y - 4.0, right, mid_y, 1.5, chevron);
            draw_line(right, mid_y, right - 5.0, mid_y + 4.0, 1.5, chevron);
        }
    }

    pub fn setting_row(
        self,
        rect: Rect,
        label: &str,
        focused: bool,
        enabled: bool,
        toggle: Option<bool>,
    ) {
        self.card(rect, focused);
        let (name, value) = label.split_once(':').unwrap_or((label, ""));
        draw_text(
            name,
            rect.x + 14.0,
            rect.y + rect.h * 0.7,
            15.0,
            if enabled { self.text() } else { self.muted() },
        );
        if let Some(on) = toggle {
            let switch = Rect::new(rect.right() - 69.0, rect.y + rect.h * 0.5 - 9.0, 48.0, 18.0);
            rounded_rectangle(
                switch,
                9.0,
                if on {
                    self.accent()
                } else {
                    self.surface_raised()
                },
            );
            draw_circle(
                switch.x + if on { 38.0 } else { 10.0 },
                switch.y + 9.0,
                6.0,
                if on { self.surface() } else { self.muted() },
            );
            draw_text(
                if on { "Activé" } else { "Désactivé" },
                rect.right() - 157.0,
                rect.y + rect.h * 0.7,
                14.0,
                self.muted(),
            );
        } else {
            let area = Rect::new(rect.x + rect.w * 0.43, rect.y, rect.w * 0.57 - 16.0, rect.h);
            draw_text_bold_centered(
                value.trim(),
                area,
                14,
                if enabled { self.accent() } else { self.muted() },
            );
            if enabled {
                draw_text(
                    "<",
                    rect.right() - 62.0,
                    rect.y + rect.h * 0.7,
                    16.0,
                    self.accent(),
                );
                draw_text(
                    ">",
                    rect.right() - 23.0,
                    rect.y + rect.h * 0.7,
                    16.0,
                    self.accent(),
                );
            }
        }
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
        theme.muted()
    } else if tone == ButtonTone::Primary {
        theme.surface()
    } else if tone == ButtonTone::Danger {
        theme.danger()
    } else if focused || active {
        theme.accent()
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
        UiIcon::Quest => {
            draw_rectangle_lines(
                left + radius * 0.22,
                top,
                radius * 1.56,
                radius * 2.0,
                line,
                color,
            );
            for offset in [0.65, 1.05, 1.45] {
                let y = top + radius * offset;
                draw_line(
                    left + radius * 0.52,
                    y,
                    right - radius * 0.45,
                    y,
                    line,
                    color,
                );
            }
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

fn surface_shadow(rect: Rect, radius: f32) {
    for (offset, alpha) in [(6.0, 0.10), (3.0, 0.20)] {
        rounded_rectangle(
            Rect::new(rect.x + 1.0, rect.y + offset, rect.w, rect.h),
            radius,
            Color::new(0.0, 0.0, 0.0, alpha),
        );
    }
}

/// Quiet directional lighting, confined to the rim so text keeps its contrast.
fn surface_relief(rect: Rect, radius: f32) {
    for i in 0..6 {
        let inset = 1.0 + i as f32;
        draw_line(
            rect.x + radius,
            rect.y + inset,
            rect.right() - radius,
            rect.y + inset,
            1.0,
            Color::new(0.65, 0.88, 0.92, 0.07 * (1.0 - i as f32 / 6.0)),
        );
    }
    draw_line(
        rect.x + radius,
        rect.bottom() - 1.0,
        rect.right() - radius,
        rect.bottom() - 1.0,
        1.0,
        Color::new(0.0, 0.0, 0.0, 0.48),
    );
    draw_line(
        rect.x + 1.0,
        rect.y + radius,
        rect.x + 1.0,
        rect.bottom() - radius,
        1.0,
        Color::new(0.48, 0.75, 0.80, 0.13),
    );
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
    fn text_cache_separates_fonts_sizes_scales_and_stays_bounded() {
        let mut cache = TextMeasurements::default();
        let value = TextDimensions {
            width: 12.0,
            height: 8.0,
            offset_y: 6.0,
        };
        assert_eq!(
            cache
                .get_or_measure("Énergie", false, 16, 1.0, || value)
                .width,
            12.0
        );
        cache.get_or_measure("Énergie", false, 16, 1.0, || panic!("cache miss"));
        for (bold, size, scale) in [(true, 16, 1.0), (false, 18, 1.0), (false, 16, 1.5)] {
            assert_eq!(
                cache
                    .get_or_measure("Énergie", bold, size, scale, || TextDimensions {
                        width: 30.0,
                        ..value
                    })
                    .width,
                30.0
            );
        }
        for index in 0..9000 {
            cache.get_or_measure(&index.to_string(), false, 16, 1.0, || value);
        }
        assert!(cache.count <= 8192);
        let before = cache.count;
        cache.get_or_measure(&"x".repeat(1025), false, 16, 1.0, || value);
        assert_eq!(cache.count, before);
        cache.set_dpi(1.5);
        assert_eq!(cache.count, 0);
        cache.get_or_measure("Énergie", false, 16, 1.0, || value);
        cache.set_dpi(1.5);
        assert_eq!(cache.count, 1);
        cache.set_dpi(2.0);
        assert_eq!(cache.count, 0);
    }

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
