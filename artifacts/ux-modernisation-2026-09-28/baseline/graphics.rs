//! Presentation-only settings. Window effects are dispatched by the native client,
//! while persistence, preview/revert and coordinate transforms are headless-testable.
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::controls::InputFrame;

pub fn ui_camera(width: f32, height: f32) -> macroquad::prelude::Camera2D {
    use macroquad::prelude::{Camera2D, Rect};
    let mut camera = Camera2D::from_display_rect(Rect::new(0.0, 0.0, width, height));
    // Camera2D::matrix inverts Y for an on-screen camera (no render target).
    // Keep GUI (0, 0) at the top left, matching the pointer coordinate transform.
    camera.zoom.y = -camera.zoom.y;
    camera
}

const RESOLUTIONS: &[[u32; 2]] = &[
    [960, 540],
    [1280, 720],
    [1280, 800],
    [1600, 900],
    [1920, 1080],
];
const UI_SCALES: &[u16] = &[75, 100, 125, 150, 175, 200];
const CELL_SIZES: &[u16] = &[24, 32, 40, 48];
fn default_cell_size() -> u16 {
    32
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowMode {
    Windowed,
    #[default]
    Borderless,
}

impl WindowMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Windowed => "Fenêtré",
            Self::Borderless => "Plein écran fenêtré",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GraphicsSettings {
    version: u8,
    pub mode: WindowMode,
    pub windowed_size: [u32; 2],
    pub ui_scale_percent: u16,
    #[serde(default = "default_cell_size")]
    pub world_cell_px: u16,
    #[serde(default)]
    pub high_contrast: bool,
    #[serde(default)]
    pub reduced_motion: bool,
}

impl Default for GraphicsSettings {
    fn default() -> Self {
        Self {
            version: 1,
            mode: WindowMode::Borderless,
            windowed_size: [1280, 800],
            ui_scale_percent: 100,
            world_cell_px: default_cell_size(),
            high_contrast: false,
            reduced_motion: false,
        }
    }
}

impl GraphicsSettings {
    fn validate(&self) -> Result<(), String> {
        if self.version != 1
            || !(640..=7680).contains(&self.windowed_size[0])
            || !(480..=4320).contains(&self.windowed_size[1])
            || !UI_SCALES.contains(&self.ui_scale_percent)
            || !CELL_SIZES.contains(&self.world_cell_px)
        {
            return Err(
                "Version, dimensions ou échelle d'interface non prises en charge.".to_owned(),
            );
        }
        Ok(())
    }

    fn decode(source: &str) -> Result<Self, String> {
        let settings: Self = serde_json::from_str(source).map_err(|e| e.to_string())?;
        settings.validate()?;
        Ok(settings)
    }

    pub fn load(path: &Path) -> (Self, String) {
        match std::fs::read_to_string(path) {
            Ok(source) => match Self::decode(&source) {
                Ok(settings) => (settings, "Réglages d'affichage chargés.".to_owned()),
                Err(error) => (
                    Self::default(),
                    format!("Affichage par défaut ; fichier invalide conservé : {error}"),
                ),
            },
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => (
                Self::default(),
                "Plein écran fenêtré par défaut, à la résolution du bureau.".to_owned(),
            ),
            Err(error) => (
                Self::default(),
                format!("Affichage par défaut ; lecture impossible : {error}"),
            ),
        }
    }

    pub fn save(&self, path: &Path) -> Result<(), String> {
        use std::io::Write;
        self.validate()?;
        let source = serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let temporary = path.with_extension("json.tmp");
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|e| e.to_string())?;
        let result = file.write_all(&source).and_then(|()| file.sync_all());
        drop(file);
        let result = result.and_then(|()| std::fs::rename(&temporary, path));
        if result.is_err() {
            let _ = std::fs::remove_file(&temporary);
        }
        result.map_err(|e| e.to_string())
    }

    /// Macroquad already exposes DPI-adjusted coordinates. The configured
    /// percentage is combined with a conservative automatic scale on large
    /// displays, then capped so every menu remains reachable after a resize.
    pub fn ui_scale(self, width: f32, height: f32) -> f32 {
        let automatic = (width.max(1.0) / 1280.0)
            .min(height.max(1.0) / 800.0)
            .clamp(1.0, 2.0);
        (self.ui_scale_percent as f32 / 100.0 * automatic)
            .min(width.max(1.0) / 700.0)
            .min(height.max(1.0) / 480.0)
    }

    pub fn transform_input(self, mut input: InputFrame) -> InputFrame {
        if let Some((width, height)) = input.viewport {
            let scale = self.ui_scale(width, height);
            input.viewport = Some((width / scale, height / scale));
            input.pointer = input.pointer.map(|(x, y)| (x / scale, y / scale));
        }
        input
    }

    pub fn cycle(&mut self, row: usize, forward: bool) {
        match row {
            0 => {
                self.mode = match self.mode {
                    WindowMode::Windowed => WindowMode::Borderless,
                    WindowMode::Borderless => WindowMode::Windowed,
                }
            }
            1 if self.mode == WindowMode::Windowed => {
                self.windowed_size = cycle_value(RESOLUTIONS, self.windowed_size, forward);
            }
            2 => self.ui_scale_percent = cycle_value(UI_SCALES, self.ui_scale_percent, forward),
            4 => self.world_cell_px = cycle_value(CELL_SIZES, self.world_cell_px, forward),
            5 => self.high_contrast = !self.high_contrast,
            6 => self.reduced_motion = !self.reduced_motion,
            _ => {}
        }
    }
}

fn cycle_value<T: Copy + PartialEq>(values: &[T], current: T, forward: bool) -> T {
    let Some(index) = values.iter().position(|value| *value == current) else {
        return values[0];
    };
    values[(index + if forward { 1 } else { values.len() - 1 }) % values.len()]
}

pub fn config_path() -> PathBuf {
    crate::controls::config_path().with_file_name("graphics.json")
}

/// Conf and the app must use the same startup read, including error messages.
pub fn startup() -> &'static (GraphicsSettings, String) {
    static SETTINGS: OnceLock<(GraphicsSettings, String)> = OnceLock::new();
    SETTINGS.get_or_init(|| {
        #[cfg(debug_assertions)]
        if ui_smoke_output().is_some() {
            let settings = GraphicsSettings {
                mode: WindowMode::Windowed,
                ..Default::default()
            };
            return (settings, "Diagnostic visuel isolé.".to_owned());
        }
        GraphicsSettings::load(&config_path())
    })
}

#[cfg(debug_assertions)]
pub fn ui_smoke_output() -> Option<PathBuf> {
    let mut args = std::env::args_os().skip(1);
    let mode = args.next()?;
    let mode = mode.to_str()?;
    if mode != "--ui-smoke" && mode != "--ui-cold" && !mode.starts_with("--ui-cold-") {
        return None;
    }
    Some(PathBuf::from(
        args.next()
            .expect("Usage : --ui-smoke <nouveau-dossier-de-captures>"),
    ))
}

#[derive(Clone)]
pub struct GraphicsState {
    pub active: GraphicsSettings,
    pub draft: GraphicsSettings,
    pub path: PathBuf,
    pub message: String,
    preview: Option<(GraphicsSettings, f64)>,
    pub pending: Option<(GraphicsSettings, GraphicsSettings)>,
    now: f64,
}

impl Default for GraphicsState {
    fn default() -> Self {
        Self {
            active: GraphicsSettings::default(),
            draft: GraphicsSettings::default(),
            path: config_path(),
            message: String::new(),
            preview: None,
            pending: None,
            now: 0.0,
        }
    }
}

impl GraphicsState {
    pub fn previewing(&self) -> bool {
        self.preview.is_some()
    }

    pub fn remaining_seconds(&self) -> u32 {
        self.preview
            .map_or(0, |(_, until)| (until - self.now).max(0.0).ceil() as u32)
    }

    pub fn begin_preview(&mut self) -> bool {
        if self.previewing() {
            return false;
        }
        if self.draft == self.active {
            self.message = "Aucune modification à appliquer.".to_owned();
            return false;
        }
        self.preview = Some((self.active, self.now + 15.0));
        self.pending = Some((self.active, self.draft));
        self.active = self.draft;
        true
    }

    pub fn revert(&mut self) {
        if let Some((previous, _)) = self.preview.take() {
            self.pending = Some((self.active, previous));
            self.active = previous;
            self.draft = previous;
            self.message = "Réglages précédents rétablis.".to_owned();
        }
    }

    pub fn confirm(&mut self) {
        if !self.previewing() {
            return;
        }
        match self.active.save(&self.path) {
            Ok(()) => {
                self.preview = None;
                self.message = "Réglages d'affichage enregistrés.".to_owned();
            }
            Err(error) => {
                self.revert();
                self.message =
                    format!("Enregistrement impossible ; anciens réglages rétablis : {error}");
            }
        }
    }

    pub fn tick(&mut self, now: f64) -> bool {
        self.now = now;
        if self.preview.is_some_and(|(_, until)| now >= until) {
            self.revert();
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pause_menu::MenuLayout;

    fn temporary_path(label: &str) -> PathBuf {
        std::env::temp_dir()
            .join(format!(
                "project-rl-graphics-{label}-{}-{}",
                std::process::id(),
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_nanos()
            ))
            .join("graphics.json")
    }

    #[test]
    fn first_launch_is_borderless_and_invalid_config_is_preserved() {
        let path = temporary_path("load");
        let (settings, _) = GraphicsSettings::load(&path);
        assert_eq!(settings.mode, WindowMode::Borderless);
        assert_eq!(settings.ui_scale_percent, 100);
        assert!(!path.exists()); // Merely loading cannot overwrite personal choices.
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, "invalid fixture").unwrap();
        let (settings, message) = GraphicsSettings::load(&path);
        assert_eq!(settings, GraphicsSettings::default());
        assert!(message.contains("invalide conservé"));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "invalid fixture");
        std::fs::remove_file(&path).unwrap();
        std::fs::remove_dir(path.parent().unwrap()).unwrap();
    }

    #[test]
    fn settings_reject_unsupported_modes_dimensions_scales_and_versions() {
        let valid = serde_json::to_value(GraphicsSettings::default()).unwrap();
        for (field, value) in [
            ("mode", serde_json::json!("exclusive")),
            ("windowed_size", serde_json::json!([0, 800])),
            ("windowed_size", serde_json::json!([1280, 999999])),
            ("ui_scale_percent", serde_json::json!(0)),
            ("version", serde_json::json!(2)),
            ("unexpected", serde_json::json!(true)),
        ] {
            let mut invalid = valid.clone();
            invalid[field] = value;
            assert!(
                GraphicsSettings::decode(&invalid.to_string()).is_err(),
                "{field}"
            );
        }
    }

    #[test]
    fn cycles_wrap_and_borderless_does_not_change_the_windowed_resolution() {
        let mut settings = GraphicsSettings::default();
        settings.cycle(1, true);
        assert_eq!(settings.windowed_size, [1280, 800]);
        settings.cycle(0, true);
        assert_eq!(settings.mode, WindowMode::Windowed);
        settings.cycle(1, true);
        assert_eq!(settings.windowed_size, [1600, 900]);
        settings.cycle(1, false);
        assert_eq!(settings.windowed_size, [1280, 800]);
        settings.ui_scale_percent = 75;
        settings.cycle(2, false);
        assert_eq!(settings.ui_scale_percent, 200);
        settings.cycle(2, true);
        assert_eq!(settings.ui_scale_percent, 75);
        settings.cycle(5, true);
        settings.cycle(6, true);
        assert!(settings.high_contrast);
        assert!(settings.reduced_motion);
    }

    #[test]
    fn default_interface_scales_up_on_large_displays_and_stays_reachable() {
        let settings = GraphicsSettings::default();
        assert_eq!(settings.ui_scale(960.0, 540.0), 1.0);
        assert!((settings.ui_scale(1920.0, 1080.0) - 1.35).abs() < 0.001);
        assert_eq!(settings.ui_scale(3840.0, 2160.0), 2.0);
        assert!(3840.0 / settings.ui_scale(3840.0, 2160.0) >= 700.0);
        assert!(2160.0 / settings.ui_scale(3840.0, 2160.0) >= 480.0);
    }

    #[test]
    fn tile_zoom_is_independent_and_legacy_graphics_keep_their_preferences() {
        let mut document = serde_json::to_value(GraphicsSettings::default()).unwrap();
        document.as_object_mut().unwrap().remove("world_cell_px");
        document.as_object_mut().unwrap().remove("high_contrast");
        document.as_object_mut().unwrap().remove("reduced_motion");
        document["ui_scale_percent"] = serde_json::json!(150);
        let mut settings = GraphicsSettings::decode(&document.to_string()).unwrap();
        assert_eq!(settings.world_cell_px, 32);
        assert!(!settings.high_contrast);
        assert!(!settings.reduced_motion);
        settings.cycle(4, true);
        assert_eq!(settings.world_cell_px, 40);
        assert_eq!(settings.ui_scale_percent, 150);
        document["world_cell_px"] = serde_json::json!(1);
        assert!(GraphicsSettings::decode(&document.to_string()).is_err());
    }

    #[test]
    fn scaled_pointer_and_drawing_agree_in_small_hidpi_and_large_viewports() {
        use macroquad::prelude::{Camera, vec3};
        for (width, height) in [
            (640.0, 480.0),
            (960.0, 540.0),
            (1280.0, 800.0),
            (3840.0, 2160.0),
        ] {
            for percent in UI_SCALES {
                let settings = GraphicsSettings {
                    ui_scale_percent: *percent,
                    ..Default::default()
                };
                let scale = settings.ui_scale(width, height);
                let matrix = ui_camera(width / scale, height / scale).matrix();
                let top_left = matrix.transform_point3(vec3(0.0, 0.0, 0.0));
                let bottom_right =
                    matrix.transform_point3(vec3(width / scale, height / scale, 0.0));
                assert!((top_left.x + 1.0).abs() < 0.0001 && (top_left.y - 1.0).abs() < 0.0001);
                assert!(
                    (bottom_right.x - 1.0).abs() < 0.0001 && (bottom_right.y + 1.0).abs() < 0.0001
                );
                let layout = MenuLayout::new(width / scale, height / scale, 7);
                for (index, rect) in layout.buttons.iter().enumerate() {
                    assert!(rect.x >= 0.0 && rect.y >= 0.0);
                    assert!((rect.x + rect.w) * scale <= width + 0.01);
                    assert!((rect.y + rect.h) * scale <= height + 0.01);
                    let input = settings.transform_input(InputFrame {
                        pointer: Some((
                            (rect.x + rect.w * 0.5) * scale,
                            (rect.y + rect.h * 0.5) * scale,
                        )),
                        viewport: Some((width, height)),
                        ..Default::default()
                    });
                    assert_eq!(layout.hit(input.pointer.unwrap()), Some(index));
                }
            }
        }
    }

    #[test]
    fn unconfirmed_preview_times_out_without_writing_settings() {
        let mut state = GraphicsState {
            path: temporary_path("timeout"),
            ..Default::default()
        };
        let before = state.active;
        state.tick(100.0);
        state.draft.mode = WindowMode::Windowed;
        assert!(state.begin_preview());
        assert!(!state.begin_preview()); // Cannot replace the rollback point.
        assert_eq!(state.remaining_seconds(), 15);
        assert_eq!(state.active.mode, WindowMode::Windowed);
        assert!(!state.tick(114.99));
        assert!(state.tick(115.0));
        assert_eq!(state.active, before);
        assert_eq!(state.draft, before);
        assert!(!state.previewing());
        assert!(!state.path.exists());
    }

    #[test]
    fn only_confirmed_choices_are_persisted_and_can_replace_previous_settings() {
        let mut state = GraphicsState {
            path: temporary_path("confirm"),
            ..Default::default()
        };
        for (mode, scale) in [(WindowMode::Windowed, 125), (WindowMode::Borderless, 150)] {
            state.draft.mode = mode;
            state.draft.ui_scale_percent = scale;
            assert!(state.begin_preview());
            state.confirm();
            assert!(!state.previewing());
            assert_eq!(GraphicsSettings::load(&state.path).0, state.active);
            assert!(!state.tick(30.0));
        }
        let committed = state.active;
        state.draft.ui_scale_percent = 75;
        state.begin_preview();
        state.revert();
        assert_eq!(state.active, committed);
        assert_eq!(GraphicsSettings::load(&state.path).0, committed);
        std::fs::remove_file(&state.path).unwrap();
        std::fs::remove_dir(state.path.parent().unwrap()).unwrap();
    }

    #[test]
    fn failed_save_rolls_back_without_truncating_saved_settings_or_existing_temporary() {
        let mut state = GraphicsState {
            path: temporary_path("failure"),
            ..Default::default()
        };
        state.active.save(&state.path).unwrap();
        let before = std::fs::read(&state.path).unwrap();
        let temporary = state.path.with_extension("json.tmp");
        std::fs::write(&temporary, "existing temporary fixture").unwrap();
        state.draft.mode = WindowMode::Windowed;
        state.begin_preview();
        state.confirm();
        assert_eq!(state.active, GraphicsSettings::default());
        assert!(state.message.contains("impossible"));
        assert_eq!(std::fs::read(&state.path).unwrap(), before);
        assert_eq!(
            std::fs::read_to_string(&temporary).unwrap(),
            "existing temporary fixture"
        );
        std::fs::remove_file(temporary).unwrap();
        std::fs::remove_file(&state.path).unwrap();
        std::fs::remove_dir(state.path.parent().unwrap()).unwrap();
    }
}
