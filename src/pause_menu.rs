//! Pause-menu navigation and shared drawing/hit-test geometry.
use macroquad::prelude::Rect;

/// Last input device owns the highlight. A stationary mouse must not steal
/// keyboard focus, nor preselect a destructive choice in a newly opened dialog.
#[derive(Default)]
pub struct MenuFocus {
    pointer: Option<(f32, f32)>,
    moved: bool,
    pointer_mode: bool,
    pub hovered: Option<usize>,
}

impl MenuFocus {
    pub fn begin_frame(&mut self, pointer: Option<(f32, f32)>) {
        self.moved = pointer.is_some() && pointer != self.pointer;
        self.pointer = pointer;
        self.hovered = None;
    }

    pub fn reset(&mut self) {
        self.moved = false;
        self.pointer_mode = false;
        self.hovered = None;
    }

    pub fn scroll(&mut self) {
        self.moved = true;
    }

    pub fn keyboard_mode(&self) -> bool {
        !self.pointer_mode
    }

    pub fn update(
        &mut self,
        hovered: Option<usize>,
        selected: &mut usize,
        clicked: bool,
        keyboard: bool,
    ) {
        self.hovered = hovered;
        if clicked {
            self.pointer_mode = true;
            if let Some(index) = hovered {
                *selected = index;
            }
        } else if keyboard {
            self.pointer_mode = false;
        } else if self.moved {
            self.pointer_mode = true;
            if let Some(index) = hovered {
                *selected = index;
            }
        }
    }

    pub fn highlighted(&self, index: usize, selected: usize) -> bool {
        if self.pointer_mode {
            self.hovered == Some(index)
        } else {
            selected == index
        }
    }
}

/// The controls list uses one geometry for paint, hover, clicks and scrolling.
pub struct ControlsLayout {
    pub bounds: Rect,
    pub back: Rect,
    pub first: usize,
    pub visible: usize,
    pub count: usize,
}

impl ControlsLayout {
    pub fn new(width: f32, height: f32, first: usize, count: usize) -> Self {
        let visible = (((height - 283.0) / 30.0).floor().max(1.0) as usize).min(count);
        Self {
            back: Rect::new((width - 130.0).max(30.0), 35.0, 100.0, 32.0),
            bounds: Rect::new(
                30.0,
                139.0,
                (width - 60.0).max(200.0),
                visible as f32 * 30.0,
            ),
            first: first.min(count.saturating_sub(visible)),
            visible,
            count,
        }
    }

    pub fn row(&self, index: usize) -> Option<Rect> {
        (index >= self.first && index < self.first + self.visible).then(|| {
            Rect::new(
                self.bounds.x,
                self.bounds.y + (index - self.first) as f32 * 30.0,
                self.bounds.w,
                29.0,
            )
        })
    }

    pub fn hit(&self, pointer: (f32, f32)) -> Option<usize> {
        (self.first..self.first + self.visible).find(|index| {
            self.row(*index)
                .is_some_and(|rect| rect.contains(pointer.into()))
        })
    }

    pub fn scroll(&mut self, wheel: f32) {
        let steps = wheel_steps(wheel);
        self.first = if wheel > 0.0 {
            self.first.saturating_sub(steps)
        } else {
            self.first
                .saturating_add(steps)
                .min(self.count.saturating_sub(self.visible))
        };
    }

    pub fn reveal(&mut self, selected: usize) {
        if selected < self.first {
            self.first = selected;
        } else if selected >= self.first + self.visible {
            self.first = selected.saturating_sub(self.visible.saturating_sub(1));
        }
        self.first = self.first.min(self.count.saturating_sub(self.visible));
    }
}

pub fn wheel_steps(wheel: f32) -> usize {
    if wheel.is_finite() && wheel != 0.0 {
        wheel.abs().ceil().min(10.0) as usize
    } else {
        0
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MenuScreen {
    #[default]
    Hidden,
    Main,
    Pause,
    Options,
    Controls,
    Graphics,
    ConfirmGraphics,
    ConfirmAbandon,
    ConfirmNewRun,
}

impl MenuScreen {
    pub fn back(self) -> Self {
        match self {
            Self::Hidden => Self::Pause,
            Self::Main => Self::Main,
            Self::Pause => Self::Hidden,
            Self::Options | Self::ConfirmAbandon => Self::Pause,
            Self::Controls | Self::Graphics => Self::Options,
            Self::ConfirmGraphics => Self::Graphics,
            Self::ConfirmNewRun => Self::Main,
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::Main => "PROJECT RL",
            Self::Pause => "PAUSE",
            Self::Options | Self::Controls => "OPTIONS",
            Self::Graphics => "AFFICHAGE",
            Self::ConfirmGraphics => "CONSERVER L'AFFICHAGE ?",
            Self::ConfirmAbandon => "ABANDONNER LA PARTIE ?",
            Self::ConfirmNewRun => "COMMENCER UNE NOUVELLE PARTIE ?",
            Self::Hidden => "",
        }
    }

    pub fn buttons(self) -> &'static [&'static str] {
        match self {
            Self::Main => &[
                "Reprendre la partie",
                "Nouvelle partie",
                "Options",
                "Quitter",
            ],
            Self::Pause => &[
                "Reprendre",
                "Options",
                "Sauvegarder et quitter",
                "Abandonner la partie",
            ],
            Self::Options => &["Commandes et clavier", "Affichage", "Retour"],
            Self::Graphics => &[
                "Mode",
                "Résolution",
                "Interface",
                "Rendu",
                "Taille des cases",
                "Contraste renforcé",
                "Animations réduites",
                "Valeurs par défaut",
                "Appliquer",
                "Retour",
            ],
            Self::ConfirmGraphics => &["Rétablir les anciens réglages", "Conserver"],
            Self::ConfirmAbandon => &["Annuler", "Confirmer"],
            Self::ConfirmNewRun => &["Annuler", "Confirmer"],
            Self::Hidden | Self::Controls => &[],
        }
    }
}

pub struct MenuLayout {
    pub panel: Rect,
    pub buttons: Vec<Rect>,
}

impl MenuLayout {
    pub fn new(width: f32, height: f32, count: usize) -> Self {
        let panel_width = (width - 32.0).clamp(360.0, 680.0);
        let desired_height: f32 = match count {
            0..=2 => 430.0,
            3 => 445.0,
            4 => 470.0,
            5..=7 => 500.0,
            _ => 540.0,
        };
        let panel_height = (height - 32.0).clamp(400.0, desired_height);
        let panel = Rect::new(
            (width - panel_width) * 0.5,
            (height - panel_height) * 0.5,
            panel_width,
            panel_height,
        );
        let row_height = ((panel_height - 190.0) / count.max(1) as f32).clamp(25.0, 58.0);
        let button_width = if count <= 4 {
            (panel.w - 120.0).max(300.0)
        } else {
            panel.w - 48.0
        };
        let buttons = (0..count)
            .map(|index| {
                Rect::new(
                    panel.x + (panel.w - button_width) * 0.5,
                    panel.y + 88.0 + index as f32 * row_height,
                    button_width,
                    row_height - 7.0,
                )
            })
            .collect();
        Self { panel, buttons }
    }

    pub fn hit(&self, pointer: (f32, f32)) -> Option<usize> {
        self.buttons
            .iter()
            .position(|rect| rect.contains(pointer.into()))
    }

    /// Dedicated, prominent feedback between the two resume buttons and footer.
    pub fn resume_error(&self) -> Rect {
        let last = self.buttons.last().expect("resume menu has buttons");
        let top = last.y + last.h + 16.0;
        Rect::new(
            self.panel.x + 24.0,
            top,
            self.panel.w - 48.0,
            self.panel.y + self.panel.h - 88.0 - top,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escape_follows_the_menu_hierarchy() {
        assert_eq!(MenuScreen::Hidden.back(), MenuScreen::Pause);
        assert_eq!(MenuScreen::Main.back(), MenuScreen::Main);
        assert_eq!(MenuScreen::Pause.back(), MenuScreen::Hidden);
        assert_eq!(
            MenuScreen::Controls.back().back().back(),
            MenuScreen::Hidden
        );
        assert_eq!(MenuScreen::ConfirmAbandon.back(), MenuScreen::Pause);
        assert_eq!(MenuScreen::ConfirmNewRun.back(), MenuScreen::Main);
    }

    #[test]
    fn drawing_and_pointer_share_non_overlapping_button_rectangles() {
        for (width, height) in [(1280.0, 800.0), (640.0, 480.0), (3840.0, 2160.0)] {
            let layout = MenuLayout::new(width, height, 4);
            assert!(layout.panel.x >= 0.0 && layout.panel.y >= 0.0);
            for (index, rect) in layout.buttons.iter().enumerate() {
                assert!(
                    rect.x >= layout.panel.x && rect.y + rect.h <= layout.panel.y + layout.panel.h
                );
                assert_eq!(
                    layout.hit((rect.x + rect.w / 2.0, rect.y + rect.h / 2.0)),
                    Some(index)
                );
                assert_eq!(layout.hit((rect.x - 1.0, rect.y)), None);
            }
        }
    }

    #[test]
    fn short_game_menus_use_a_compact_centered_action_column() {
        let short = MenuLayout::new(1280.0, 800.0, 4);
        let dense = MenuLayout::new(1280.0, 800.0, 10);
        assert!(short.panel.h < dense.panel.h);
        assert!(short.buttons[0].w < dense.buttons[0].w);
        assert_eq!(
            short.buttons[0].x + short.buttons[0].w * 0.5,
            short.panel.x + short.panel.w * 0.5
        );
    }

    #[test]
    fn resume_error_fits_between_buttons_and_footer() {
        for (width, height) in [(640.0, 480.0), (1280.0, 800.0), (1920.0, 1080.0)] {
            let layout = MenuLayout::new(width, height, 2);
            let error = layout.resume_error();
            let last = layout.buttons.last().unwrap();
            assert!(error.y > last.y + last.h);
            assert!(error.x > layout.panel.x);
            assert!(error.x + error.w < layout.panel.x + layout.panel.w);
            assert!(error.h >= 120.0);
            assert!(error.y + error.h < layout.panel.y + layout.panel.h - 70.0);
        }
    }

    #[test]
    fn hover_is_immediate_but_stationary_mouse_does_not_override_keyboard() {
        let mut focus = MenuFocus::default();
        let mut selected = 0;
        focus.begin_frame(Some((10.0, 20.0)));
        focus.update(Some(2), &mut selected, false, false);
        assert_eq!(selected, 2);
        assert!(focus.highlighted(2, selected));
        focus.begin_frame(Some((10.0, 20.0)));
        selected = 3; // Down was pressed by the caller.
        focus.update(Some(2), &mut selected, false, true);
        assert!(focus.highlighted(3, selected));
        focus.begin_frame(Some((10.0, 20.0)));
        focus.update(Some(2), &mut selected, false, false);
        assert_eq!(selected, 3);
        assert!(focus.highlighted(3, selected));
        focus.begin_frame(Some((11.0, 20.0)));
        focus.update(Some(2), &mut selected, false, false);
        assert_eq!(selected, 2);
        focus.begin_frame(Some((1000.0, 20.0)));
        focus.update(None, &mut selected, false, false);
        assert!(!(0..4).any(|index| focus.highlighted(index, selected)));
    }

    #[test]
    fn opening_a_confirmation_preserves_safe_default_until_pointer_moves_or_clicks() {
        let mut focus = MenuFocus::default();
        let mut selected = 3;
        focus.begin_frame(Some((10.0, 20.0)));
        focus.update(Some(3), &mut selected, true, false);
        focus.reset();
        selected = 0;
        focus.begin_frame(Some((10.0, 20.0)));
        focus.update(Some(1), &mut selected, false, false);
        assert_eq!(selected, 0);
        assert!(focus.highlighted(0, selected));
        focus.update(Some(1), &mut selected, true, false);
        assert_eq!(selected, 1);
    }

    #[test]
    fn controls_scroll_and_hit_test_only_visible_rows_and_reveal_keyboard_selection() {
        let mut layout = ControlsLayout::new(700.0, 480.0, 0, 30);
        assert_eq!(layout.visible, 6);
        assert_eq!(layout.hit((40.0, 150.0)), Some(0));
        layout.scroll(-3.0);
        assert_eq!(layout.hit((40.0, 150.0)), Some(3));
        assert!(layout.row(0).is_none());
        assert_eq!(
            layout.hit((40.0, layout.bounds.y + layout.bounds.h + 1.0)),
            None
        );
        layout.reveal(29);
        assert_eq!(layout.first, 24);
        layout.scroll(-100.0);
        assert_eq!(layout.first, 24);
        layout.reveal(0);
        layout.scroll(100.0);
        assert_eq!(layout.first, 0);
        assert_eq!(wheel_steps(f32::NAN), 0);
        assert_eq!(wheel_steps(f32::INFINITY), 0);
    }

    #[test]
    fn graphics_rows_leave_room_for_status_and_footer_at_every_ui_scale() {
        for height in [480.0, 540.0, 800.0, 2160.0] {
            let layout = MenuLayout::new(700.0, height, MenuScreen::Graphics.buttons().len());
            let last = layout.buttons.last().unwrap();
            assert!(last.y + last.h < layout.panel.y + layout.panel.h - 85.0);
        }
        assert_eq!(MenuScreen::ConfirmGraphics.back(), MenuScreen::Graphics);
        assert_eq!(MenuScreen::Graphics.back(), MenuScreen::Options);
    }
}
