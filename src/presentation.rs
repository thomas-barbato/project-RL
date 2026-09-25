use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::content::ContentId;
use crate::world::{GridPos, PropagationCell};

/// Stable semantic identifier interpreted by whichever renderer is active.
/// It deliberately names no texture, particle system, sound or screen timing.
pub type VisualCueId = ContentId;

/// A defensive ceiling for one presentation request. Visuals never influence
/// simulation results, but malformed mod output must not allocate without bound.
pub const MAX_VISUAL_CUE_CELLS: usize = 4_096;
pub const MAX_TERMINAL_CUE_FRAMES: usize = 16;
pub const MAX_TERMINAL_CUE_COLORS: usize = 3;
pub const MAX_TERMINAL_FRAME_MILLIS: u16 = 1_000;
pub const MAX_TERMINAL_STEP_MILLIS: u16 = 2_000;
pub const MAX_TERMINAL_LINGER_MILLIS: u16 = 10_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminalEffectGlyph {
    Dot,
    Projectile,
    Spark,
    Burst,
    FlameSmall,
    Flame,
    FlameLarge,
    Impact,
    Wave,
    Alarm,
    ShockSmall,
    ShockWide,
    RingSmall,
    RingWide,
    ArcFork,
    ArcSplit,
    AcidDrop,
    AcidSplash,
    AcidPool,
    Restoration,
    Piercing,
    Guard,
    GuardBreak,
    Impulse,
    ImpulseBlocked,
    Catalysis,
    Frost,
    Fracture,
    Alternation,
    Echo,
    Ricochet,
    FlameJet,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TerminalCueStyle {
    frames: Vec<TerminalEffectGlyph>,
    colors: Vec<[u8; 4]>,
    frame_millis: u16,
    step_millis: u16,
    linger_millis: u16,
}

impl TerminalCueStyle {
    pub fn new(
        frames: Vec<TerminalEffectGlyph>,
        color: [u8; 4],
        frame_millis: u16,
        step_millis: u16,
        linger_millis: u16,
    ) -> Result<Self, TerminalCueStyleError> {
        Self::new_with_palette(
            frames,
            vec![color],
            frame_millis,
            step_millis,
            linger_millis,
        )
    }

    pub fn new_with_palette(
        frames: Vec<TerminalEffectGlyph>,
        colors: Vec<[u8; 4]>,
        frame_millis: u16,
        step_millis: u16,
        linger_millis: u16,
    ) -> Result<Self, TerminalCueStyleError> {
        if frames.is_empty() {
            return Err(TerminalCueStyleError::NoFrames);
        }
        if frames.len() > MAX_TERMINAL_CUE_FRAMES {
            return Err(TerminalCueStyleError::TooManyFrames {
                found: frames.len(),
                maximum: MAX_TERMINAL_CUE_FRAMES,
            });
        }
        if colors.is_empty() {
            return Err(TerminalCueStyleError::NoColors);
        }
        if colors.len() > MAX_TERMINAL_CUE_COLORS {
            return Err(TerminalCueStyleError::TooManyColors {
                found: colors.len(),
                maximum: MAX_TERMINAL_CUE_COLORS,
            });
        }
        if frame_millis == 0 || frame_millis > MAX_TERMINAL_FRAME_MILLIS {
            return Err(TerminalCueStyleError::InvalidFrameMillis(frame_millis));
        }
        if step_millis > MAX_TERMINAL_STEP_MILLIS {
            return Err(TerminalCueStyleError::InvalidStepMillis(step_millis));
        }
        if linger_millis > MAX_TERMINAL_LINGER_MILLIS {
            return Err(TerminalCueStyleError::InvalidLingerMillis(linger_millis));
        }
        if colors.iter().any(|color| color[3] == 0) {
            return Err(TerminalCueStyleError::InvisibleColor);
        }
        Ok(Self {
            frames,
            colors,
            frame_millis,
            step_millis,
            linger_millis,
        })
    }

    pub fn frames(&self) -> &[TerminalEffectGlyph] {
        &self.frames
    }

    pub fn color(&self) -> [u8; 4] {
        self.colors[0]
    }

    pub fn colors(&self) -> &[[u8; 4]] {
        &self.colors
    }

    pub const fn frame_millis(&self) -> u16 {
        self.frame_millis
    }

    pub const fn step_millis(&self) -> u16 {
        self.step_millis
    }

    pub const fn linger_millis(&self) -> u16 {
        self.linger_millis
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminalCueStyleError {
    NoFrames,
    TooManyFrames { found: usize, maximum: usize },
    NoColors,
    TooManyColors { found: usize, maximum: usize },
    InvalidFrameMillis(u16),
    InvalidStepMillis(u16),
    InvalidLingerMillis(u16),
    InvisibleColor,
}

impl Display for TerminalCueStyleError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoFrames => write!(formatter, "terminal visual cue must contain a frame"),
            Self::TooManyFrames { found, maximum } => write!(
                formatter,
                "terminal visual cue has {found} frames, maximum is {maximum}"
            ),
            Self::NoColors => write!(formatter, "terminal visual cue must contain a color"),
            Self::TooManyColors { found, maximum } => write!(
                formatter,
                "terminal visual cue has {found} colors, maximum is {maximum}"
            ),
            Self::InvalidFrameMillis(value) => write!(
                formatter,
                "terminal visual cue frame duration must be 1..={MAX_TERMINAL_FRAME_MILLIS} ms, found {value}"
            ),
            Self::InvalidStepMillis(value) => write!(
                formatter,
                "terminal visual cue step delay must be 0..={MAX_TERMINAL_STEP_MILLIS} ms, found {value}"
            ),
            Self::InvalidLingerMillis(value) => write!(
                formatter,
                "terminal visual cue linger must be 0..={MAX_TERMINAL_LINGER_MILLIS} ms, found {value}"
            ),
            Self::InvisibleColor => write!(formatter, "terminal visual cue color is transparent"),
        }
    }
}

impl Error for TerminalCueStyleError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisualCueDefinition {
    id: VisualCueId,
    terminal: TerminalCueStyle,
}

impl VisualCueDefinition {
    pub const fn new(id: VisualCueId, terminal: TerminalCueStyle) -> Self {
        Self { id, terminal }
    }

    pub const fn id(&self) -> &VisualCueId {
        &self.id
    }

    pub const fn terminal(&self) -> &TerminalCueStyle {
        &self.terminal
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VisualCueCatalog {
    definitions: BTreeMap<VisualCueId, VisualCueDefinition>,
}

impl VisualCueCatalog {
    pub fn register(
        &mut self,
        definition: VisualCueDefinition,
    ) -> Result<(), VisualCueCatalogError> {
        let id = definition.id().clone();
        if self.definitions.contains_key(&id) {
            return Err(VisualCueCatalogError::DuplicateId(id));
        }
        self.definitions.insert(id, definition);
        Ok(())
    }

    pub fn get(&self, id: &VisualCueId) -> Option<&VisualCueDefinition> {
        self.definitions.get(id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&VisualCueId, &VisualCueDefinition)> {
        self.definitions.iter()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VisualCueCatalogError {
    DuplicateId(VisualCueId),
}

impl Display for VisualCueCatalogError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateId(id) => write!(formatter, "duplicate visual cue ID '{id}'"),
        }
    }
}

impl Error for VisualCueCatalogError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VisualCueCell {
    pub position: GridPos,
    /// Relative wave step. A renderer may animate it or ignore it entirely.
    pub delay_step: u16,
}

impl VisualCueCell {
    pub const fn new(position: GridPos, delay_step: u16) -> Self {
        Self {
            position,
            delay_step,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VisualCueTarget {
    /// A non-spatial signal for HUD, menu or full-screen feedback.
    Interface,
    /// A cue anchored in the world. Cells are unique and ordered by wave step,
    /// then by position, so every presentation backend sees stable input.
    World {
        origin: GridPos,
        cells: Vec<VisualCueCell>,
    },
}

/// Presentation request derived from simulation events by a client adapter.
/// Saving, replay and gameplay rules do not contain these transient requests.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VisualCue {
    id: VisualCueId,
    target: VisualCueTarget,
}

impl VisualCue {
    pub const fn interface(id: VisualCueId) -> Self {
        Self {
            id,
            target: VisualCueTarget::Interface,
        }
    }

    pub fn point(id: VisualCueId, position: GridPos) -> Self {
        Self {
            id,
            target: VisualCueTarget::World {
                origin: position,
                cells: vec![VisualCueCell::new(position, 0)],
            },
        }
    }

    pub fn world(
        id: VisualCueId,
        origin: GridPos,
        cells: impl IntoIterator<Item = VisualCueCell>,
    ) -> Result<Self, VisualCueError> {
        let cells = cells.into_iter().collect::<Vec<_>>();
        if cells.is_empty() {
            return Err(VisualCueError::EmptyWorldFootprint);
        }
        if cells.len() > MAX_VISUAL_CUE_CELLS {
            return Err(VisualCueError::TooManyCells {
                found: cells.len(),
                maximum: MAX_VISUAL_CUE_CELLS,
            });
        }

        let mut unique = BTreeMap::<GridPos, u16>::new();
        for cell in cells {
            unique
                .entry(cell.position)
                .and_modify(|step| *step = (*step).min(cell.delay_step))
                .or_insert(cell.delay_step);
        }
        let mut cells = unique
            .into_iter()
            .map(|(position, delay_step)| VisualCueCell::new(position, delay_step))
            .collect::<Vec<_>>();
        cells.sort_by_key(|cell| (cell.delay_step, cell.position));

        Ok(Self {
            id,
            target: VisualCueTarget::World { origin, cells },
        })
    }

    /// Builds a purely visual line. This never decides whether an attack hits
    /// and never checks collision: the simulation event has already resolved.
    pub fn line(id: VisualCueId, origin: GridPos, target: GridPos) -> Result<Self, VisualCueError> {
        if origin == target {
            return Ok(Self::point(id, target));
        }

        let mut x = i64::from(origin.x);
        let mut y = i64::from(origin.y);
        let target_x = i64::from(target.x);
        let target_y = i64::from(target.y);
        let delta_x = (target_x - x).abs();
        let step_x = if x < target_x { 1 } else { -1 };
        let delta_y = -(target_y - y).abs();
        let step_y = if y < target_y { 1 } else { -1 };
        let mut error = delta_x + delta_y;
        let mut cells = Vec::new();

        while x != target_x || y != target_y {
            let doubled_error = error.saturating_mul(2);
            if doubled_error >= delta_y {
                error += delta_y;
                x += step_x;
            }
            if doubled_error <= delta_x {
                error += delta_x;
                y += step_y;
            }
            if cells.len() == MAX_VISUAL_CUE_CELLS {
                return Err(VisualCueError::TooManyCells {
                    found: cells.len().saturating_add(1),
                    maximum: MAX_VISUAL_CUE_CELLS,
                });
            }
            cells.push(VisualCueCell::new(
                GridPos::new(x as i32, y as i32),
                u16::try_from(cells.len()).expect("visual cue ceiling fits in u16"),
            ));
        }

        Self::world(id, origin, cells)
    }

    pub fn from_propagation(
        id: VisualCueId,
        origin: GridPos,
        cells: &[PropagationCell],
    ) -> Result<Self, VisualCueError> {
        Self::world(
            id,
            origin,
            cells
                .iter()
                .map(|cell| VisualCueCell::new(cell.position, cell.step)),
        )
    }

    pub const fn id(&self) -> &VisualCueId {
        &self.id
    }

    pub const fn target(&self) -> &VisualCueTarget {
        &self.target
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VisualCueError {
    EmptyWorldFootprint,
    TooManyCells { found: usize, maximum: usize },
}

impl Display for VisualCueError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyWorldFootprint => write!(formatter, "world visual cue has no cells"),
            Self::TooManyCells { found, maximum } => write!(
                formatter,
                "world visual cue has {found} cells, maximum is {maximum}"
            ),
        }
    }
}

impl Error for VisualCueError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> VisualCueId {
        value.parse().expect("valid visual cue ID")
    }

    fn terminal_style() -> TerminalCueStyle {
        TerminalCueStyle::new(
            vec![TerminalEffectGlyph::Dot, TerminalEffectGlyph::Burst],
            [255, 120, 40, 255],
            60,
            40,
            80,
        )
        .expect("valid terminal style")
    }

    #[test]
    fn terminal_styles_are_bounded_and_cannot_be_invisible() {
        assert_eq!(
            TerminalCueStyle::new(vec![], [255, 255, 255, 255], 60, 0, 0),
            Err(TerminalCueStyleError::NoFrames)
        );
        assert_eq!(
            TerminalCueStyle::new(
                vec![TerminalEffectGlyph::Spark],
                [255, 255, 255, 0],
                60,
                0,
                0,
            ),
            Err(TerminalCueStyleError::InvisibleColor)
        );
        assert_eq!(
            TerminalCueStyle::new_with_palette(vec![TerminalEffectGlyph::Spark], vec![], 60, 0, 0,),
            Err(TerminalCueStyleError::NoColors)
        );
        assert!(matches!(
            TerminalCueStyle::new_with_palette(
                vec![TerminalEffectGlyph::Spark],
                vec![[255, 120, 40, 255]; MAX_TERMINAL_CUE_COLORS + 1],
                60,
                0,
                0,
            ),
            Err(TerminalCueStyleError::TooManyColors { .. })
        ));
        let palette = TerminalCueStyle::new_with_palette(
            vec![TerminalEffectGlyph::Spark],
            vec![[180, 35, 18, 220], [255, 112, 24, 235], [255, 232, 96, 255]],
            60,
            0,
            0,
        )
        .unwrap();
        assert_eq!(palette.colors().len(), 3);
        assert_eq!(palette.color(), [180, 35, 18, 220]);
        assert!(
            TerminalCueStyle::new(
                vec![TerminalEffectGlyph::Spark; MAX_TERMINAL_CUE_FRAMES + 1],
                [255, 255, 255, 255],
                60,
                0,
                0,
            )
            .is_err()
        );
    }

    #[test]
    fn cue_catalog_rejects_silent_style_overrides() {
        let mut catalog = VisualCueCatalog::default();
        let definition =
            VisualCueDefinition::new(id("example.effects:flame_cone"), terminal_style());

        assert_eq!(catalog.register(definition.clone()), Ok(()));
        assert!(matches!(
            catalog.register(definition),
            Err(VisualCueCatalogError::DuplicateId(_))
        ));
    }

    #[test]
    fn world_cue_deduplicates_cells_and_keeps_the_earliest_step() {
        let cue = VisualCue::world(
            id("example.effects:flame_cone"),
            GridPos::new(4, 4),
            [
                VisualCueCell::new(GridPos::new(5, 4), 2),
                VisualCueCell::new(GridPos::new(5, 4), 1),
                VisualCueCell::new(GridPos::new(6, 4), 2),
            ],
        )
        .expect("valid cue");

        assert_eq!(
            cue.target(),
            &VisualCueTarget::World {
                origin: GridPos::new(4, 4),
                cells: vec![
                    VisualCueCell::new(GridPos::new(5, 4), 1),
                    VisualCueCell::new(GridPos::new(6, 4), 2),
                ],
            }
        );
    }

    #[test]
    fn line_is_stable_excludes_the_source_and_reaches_the_target() {
        let cue = VisualCue::line(
            id("core:needle_launcher"),
            GridPos::new(1, 1),
            GridPos::new(4, 2),
        )
        .expect("valid line");
        let VisualCueTarget::World { cells, .. } = cue.target() else {
            panic!("line should target the world");
        };

        assert_eq!(cells.len(), 3);
        assert_ne!(cells[0].position, GridPos::new(1, 1));
        assert_eq!(
            cells.last().map(|cell| cell.position),
            Some(GridPos::new(4, 2))
        );
        assert!(
            cells
                .windows(2)
                .all(|pair| pair[0].delay_step < pair[1].delay_step)
        );
    }

    #[test]
    fn global_interface_cue_has_no_world_footprint() {
        let cue = VisualCue::interface(id("core:security_alarm"));

        assert_eq!(cue.target(), &VisualCueTarget::Interface);
    }
}
