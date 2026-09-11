use macroquad::prelude::Color;
use project_rl::presentation::{
    TerminalCueStyle, TerminalEffectGlyph, VisualCue, VisualCueCatalog, VisualCueTarget,
};
use project_rl::world::GridPos;

const MAX_ACTIVE_CUES: usize = 32;
const FALLBACK_PALETTE: [[u8; 4]; 1] = [[151, 229, 255, 255]];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TerminalEffectSample {
    pub symbol: char,
    pub color: Color,
    pub accent_color: Option<Color>,
    pub highlight_color: Option<Color>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TerminalEffectPalette {
    pub color: Color,
    pub accent_color: Option<Color>,
    pub highlight_color: Option<Color>,
}

pub struct VisualCuePlayer {
    active: Vec<ActiveVisualCue>,
    styles: VisualCueCatalog,
}

impl Default for VisualCuePlayer {
    fn default() -> Self {
        Self::with_catalog(VisualCueCatalog::default())
    }
}

impl VisualCuePlayer {
    pub const fn with_catalog(styles: VisualCueCatalog) -> Self {
        Self {
            active: Vec::new(),
            styles,
        }
    }

    pub fn play(&mut self, cue: VisualCue, started_at: f64) {
        self.active
            .retain(|active| !active.expired(started_at, &self.styles));
        self.active.push(ActiveVisualCue { cue, started_at });
        if self.active.len() > MAX_ACTIVE_CUES {
            self.active.remove(0);
        }
    }

    /// The caller supplies current perception explicitly. This second gate is
    /// intentional: a visual effect must never reveal a hidden simulation cell.
    pub fn sample_world(
        &self,
        position: GridPos,
        is_visible: bool,
        now: f64,
    ) -> Option<TerminalEffectSample> {
        if !is_visible {
            return None;
        }
        self.active
            .iter()
            .rev()
            .find_map(|active| active.sample_world(position, now, &self.styles))
    }

    pub fn clear_world(&mut self) {
        self.active
            .retain(|active| matches!(active.cue.target(), VisualCueTarget::Interface));
    }

    pub fn palette_for(&self, id: &project_rl::presentation::VisualCueId) -> TerminalEffectPalette {
        terminal_style(&self.styles, id).palette()
    }

    #[cfg(any(test, debug_assertions))]
    pub(crate) fn active_count(&self) -> usize {
        self.active.len()
    }
}

struct ActiveVisualCue {
    cue: VisualCue,
    started_at: f64,
}

impl ActiveVisualCue {
    fn sample_world(
        &self,
        position: GridPos,
        now: f64,
        styles: &VisualCueCatalog,
    ) -> Option<TerminalEffectSample> {
        let VisualCueTarget::World { cells, .. } = self.cue.target() else {
            return None;
        };
        let cell = cells.iter().find(|cell| cell.position == position)?;
        let style = terminal_style(styles, self.cue.id());
        let local_time = now - self.started_at - f64::from(cell.delay_step) * style.step_seconds();
        if local_time < 0.0 {
            return None;
        }
        let animation_duration = style.frame_seconds() * style.frames().len() as f64;
        if local_time >= animation_duration + style.linger_seconds() {
            return None;
        }
        let frame = ((local_time / style.frame_seconds()) as usize)
            .min(style.frames().len().saturating_sub(1));
        let palette = style.palette();
        Some(TerminalEffectSample {
            symbol: terminal_symbol(style.frames()[frame]),
            color: palette.color,
            accent_color: palette.accent_color,
            highlight_color: palette.highlight_color,
        })
    }

    fn expired(&self, now: f64, styles: &VisualCueCatalog) -> bool {
        let style = terminal_style(styles, self.cue.id());
        let maximum_step = match self.cue.target() {
            VisualCueTarget::Interface => 0,
            VisualCueTarget::World { cells, .. } => {
                cells.iter().map(|cell| cell.delay_step).max().unwrap_or(0)
            }
        };
        now - self.started_at
            >= f64::from(maximum_step) * style.step_seconds()
                + style.frame_seconds() * style.frames().len() as f64
                + style.linger_seconds()
    }
}

enum ResolvedTerminalCueStyle<'a> {
    Loaded(&'a TerminalCueStyle),
    Fallback,
}

impl ResolvedTerminalCueStyle<'_> {
    fn frames(&self) -> &[TerminalEffectGlyph] {
        match self {
            Self::Loaded(style) => style.frames(),
            Self::Fallback => &[TerminalEffectGlyph::Dot, TerminalEffectGlyph::Spark],
        }
    }

    fn palette(&self) -> TerminalEffectPalette {
        let colors = match self {
            Self::Loaded(style) => style.colors(),
            Self::Fallback => &FALLBACK_PALETTE,
        };
        TerminalEffectPalette {
            color: rgba(colors[0]),
            accent_color: colors.get(1).copied().map(rgba),
            highlight_color: colors.get(2).copied().map(rgba),
        }
    }

    fn frame_seconds(&self) -> f64 {
        f64::from(match self {
            Self::Loaded(style) => style.frame_millis(),
            Self::Fallback => 80,
        }) / 1_000.0
    }

    fn step_seconds(&self) -> f64 {
        f64::from(match self {
            Self::Loaded(style) => style.step_millis(),
            Self::Fallback => 45,
        }) / 1_000.0
    }

    fn linger_seconds(&self) -> f64 {
        f64::from(match self {
            Self::Loaded(style) => style.linger_millis(),
            Self::Fallback => 80,
        }) / 1_000.0
    }
}

fn rgba([red, green, blue, alpha]: [u8; 4]) -> Color {
    Color::from_rgba(red, green, blue, alpha)
}

fn terminal_style<'a>(
    catalog: &'a VisualCueCatalog,
    id: &project_rl::presentation::VisualCueId,
) -> ResolvedTerminalCueStyle<'a> {
    catalog
        .get(id)
        .map(|definition| ResolvedTerminalCueStyle::Loaded(definition.terminal()))
        .unwrap_or(ResolvedTerminalCueStyle::Fallback)
}

const fn terminal_symbol(glyph: TerminalEffectGlyph) -> char {
    match glyph {
        TerminalEffectGlyph::Dot => '.',
        TerminalEffectGlyph::Projectile => '-',
        TerminalEffectGlyph::Spark => '*',
        TerminalEffectGlyph::Burst => '+',
        TerminalEffectGlyph::FlameSmall => 'f',
        TerminalEffectGlyph::Flame => 'F',
        TerminalEffectGlyph::FlameLarge => '^',
        TerminalEffectGlyph::Impact => 'x',
        TerminalEffectGlyph::Wave => '~',
        TerminalEffectGlyph::Alarm => 'a',
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use project_rl::presentation::{TerminalCueStyle, VisualCueDefinition};

    fn cue_id(value: &str) -> project_rl::presentation::VisualCueId {
        value.parse().expect("valid cue ID")
    }

    #[test]
    fn delayed_line_animates_without_exposing_hidden_cells() {
        let mut player = VisualCuePlayer::default();
        let cue = VisualCue::line(
            cue_id("core:needle_launcher"),
            GridPos::new(1, 1),
            GridPos::new(3, 1),
        )
        .expect("valid line");
        player.play(cue, 10.0);

        assert!(
            player
                .sample_world(GridPos::new(2, 1), true, 10.0)
                .is_some()
        );
        assert!(
            player
                .sample_world(GridPos::new(3, 1), true, 10.0)
                .is_none()
        );
        assert!(
            player
                .sample_world(GridPos::new(3, 1), true, 10.08)
                .is_some()
        );
        assert!(
            player
                .sample_world(GridPos::new(2, 1), false, 10.08)
                .is_none()
        );
        assert!(
            player
                .sample_world(GridPos::new(3, 1), true, 11.0)
                .is_none()
        );
    }

    #[test]
    fn bounded_queue_drops_the_oldest_transient_cues() {
        let mut player = VisualCuePlayer::default();
        for index in 0..MAX_ACTIVE_CUES + 3 {
            player.play(
                VisualCue::point(cue_id("core:local_alert"), GridPos::new(index as i32, 0)),
                0.0,
            );
        }

        assert_eq!(player.active_count(), MAX_ACTIVE_CUES);
        assert!(player.sample_world(GridPos::new(0, 0), true, 0.0).is_none());
    }

    #[test]
    fn changing_animation_time_never_changes_the_cue_description() {
        let cue = VisualCue::point(cue_id("example.effects:flame_cone"), GridPos::new(2, 2));
        let before = format!("{cue:?}");
        let mut player = VisualCuePlayer::default();
        player.play(cue, 5.0);

        let _ = player.sample_world(GridPos::new(2, 2), true, 5.05);
        let _ = player.sample_world(GridPos::new(2, 2), true, 50.0);
        assert_eq!(player.active_count(), 1);
        assert_eq!(before, format!("{:?}", player.active[0].cue));
    }

    #[test]
    fn loaded_style_controls_frames_color_and_timing_without_code_branches() {
        let id = cue_id("example.effects:flame_cone");
        let style = TerminalCueStyle::new(
            vec![TerminalEffectGlyph::Wave],
            [12, 34, 56, 255],
            100,
            0,
            0,
        )
        .expect("valid style");
        let mut catalog = VisualCueCatalog::default();
        catalog
            .register(VisualCueDefinition::new(id.clone(), style))
            .expect("unique style");
        let mut player = VisualCuePlayer::with_catalog(catalog);
        player.play(VisualCue::point(id, GridPos::new(2, 2)), 5.0);

        let sample = player
            .sample_world(GridPos::new(2, 2), true, 5.05)
            .expect("loaded cue should be visible");
        assert_eq!(sample.symbol, '~');
        assert_eq!(sample.color, Color::from_rgba(12, 34, 56, 255));
        assert_eq!(sample.accent_color, None);
        assert_eq!(sample.highlight_color, None);
        assert!(
            player
                .sample_world(GridPos::new(2, 2), true, 5.11)
                .is_none()
        );
    }

    #[test]
    fn loaded_palette_exposes_three_colors_without_changing_cue_geometry() {
        let id = cue_id("example.effects:multicolor_flame");
        let style = TerminalCueStyle::new_with_palette(
            vec![TerminalEffectGlyph::Spark],
            vec![[180, 35, 18, 220], [255, 112, 24, 235], [255, 232, 96, 255]],
            100,
            0,
            0,
        )
        .expect("valid palette");
        let mut catalog = VisualCueCatalog::default();
        catalog
            .register(VisualCueDefinition::new(id.clone(), style))
            .expect("unique style");
        let mut player = VisualCuePlayer::with_catalog(catalog);
        player.play(VisualCue::point(id, GridPos::new(2, 2)), 5.0);

        let sample = player
            .sample_world(GridPos::new(2, 2), true, 5.05)
            .expect("loaded cue should be visible");
        assert_eq!(sample.color, Color::from_rgba(180, 35, 18, 220));
        assert_eq!(
            sample.accent_color,
            Some(Color::from_rgba(255, 112, 24, 235))
        );
        assert_eq!(
            sample.highlight_color,
            Some(Color::from_rgba(255, 232, 96, 255))
        );
    }
}
