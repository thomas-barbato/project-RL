use macroquad::prelude::Color;
use project_rl::presentation::{
    TerminalCueStyle, TerminalEffectGlyph, VisualCue, VisualCueCatalog, VisualCueTarget,
};
use project_rl::world::GridPos;

const MAX_ACTIVE_CUES: usize = 32;
const FALLBACK_PALETTE: [[u8; 4]; 1] = [[151, 229, 255, 255]];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TerminalEffectFamily {
    Glyph,
    ImpactWave,
    BearerWave,
    Conduction,
    Flame,
    Corrosion,
    Caustic,
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

// Clockwise: N, NE, E, SE, S, SW, W, NW. Local edges only: the
// renderer never queries terrain or invents cells outside the perceived cue.
pub const EFFECT_NEIGHBORS: [(i32, i32); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TerminalEffectSample {
    pub symbol: char,
    pub color: Color,
    pub accent_color: Option<Color>,
    pub highlight_color: Option<Color>,
    /// Top/right/bottom/left exposed edges of a single travelling wavefront.
    pub outline: Option<u8>,
    pub family: TerminalEffectFamily,
    /// Local animation progress. Reduced motion samples a fixed instant.
    pub progress: f32,
    /// A live simulation effect, looping without the transient fade-out.
    pub sustained: bool,
    pub variant: u8,
    /// Connected electrical branches, indexed by EFFECT_NEIGHBORS.
    pub links: u8,
    /// Local direction of a piercing trace, never a world-space destination.
    pub direction: (f32, f32),
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
        let steps: std::collections::BTreeMap<GridPos, u16> = match cue.target() {
            VisualCueTarget::World { cells, .. } => cells
                .iter()
                .map(|cell| (cell.position, cell.delay_step))
                .collect(),
            VisualCueTarget::Interface => std::collections::BTreeMap::new(),
        };
        let links = if matches!(
            terminal_style(&self.styles, cue.id()).family(),
            TerminalEffectFamily::Conduction
                | TerminalEffectFamily::ImpactWave
                | TerminalEffectFamily::BearerWave
        ) {
            conduction_links(&steps)
        } else {
            Default::default()
        };
        self.active.push(ActiveVisualCue {
            cue,
            started_at,
            steps,
            links,
        });
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

    pub fn sample_world_with_motion(
        &self,
        position: GridPos,
        visible: bool,
        now: f64,
        reduced: bool,
    ) -> Option<TerminalEffectSample> {
        if !reduced {
            return self.sample_world(position, visible, now);
        }
        if !visible {
            return None;
        }
        self.active.iter().rev().find_map(|active| {
            if !(0.0..0.35).contains(&(now - active.started_at)) {
                return None;
            }
            let style = terminal_style(&self.styles, active.cue.id());
            let last_step = active.steps.values().copied().max().unwrap_or(0);
            let sampled_step = if matches!(
                style.family(),
                TerminalEffectFamily::ImpactWave | TerminalEffectFamily::BearerWave
            ) {
                last_step
            } else {
                *active.steps.get(&position)?
            };
            active.sample_world(
                position,
                active.started_at
                    + f64::from(sampled_step) * style.step_seconds()
                    + style.frame_seconds() * 0.5,
                &self.styles,
            )
        })
    }

    pub fn palette_for(&self, id: &project_rl::presentation::VisualCueId) -> TerminalEffectPalette {
        terminal_style(&self.styles, id).palette()
    }

    /// Sample live fields, not past events. No wall-clock deadline, queue cap,
    /// save data or off-screen knowledge determines their presence.
    pub fn sample_ground(
        &self,
        ground: &project_rl::effects::GroundEffectMap,
        position: GridPos,
        visible: bool,
        now: f64,
        reduced: bool,
    ) -> Vec<TerminalEffectSample> {
        if !visible {
            return Vec::new();
        }
        ground
            .at(position)
            .map(|field| self.sample_sustained(field.definition(), position, now, reduced))
            .collect()
    }

    /// Only elemental status styles loop. Healing, guard activation, cooldowns
    /// and bursts remain one-shot feedback, not continuous attacks.
    pub fn sustains_status(&self, id: &project_rl::presentation::VisualCueId) -> bool {
        matches!(
            terminal_style(&self.styles, id).family(),
            TerminalEffectFamily::Flame
                | TerminalEffectFamily::Corrosion
                | TerminalEffectFamily::Caustic
                | TerminalEffectFamily::Conduction
                | TerminalEffectFamily::Frost
        )
    }

    /// Caller must supply a currently present and perceived field/status.
    pub fn sample_sustained(
        &self,
        id: &project_rl::presentation::VisualCueId,
        position: GridPos,
        now: f64,
        reduced: bool,
    ) -> TerminalEffectSample {
        let style = terminal_style(&self.styles, id);
        let variant = (position.x.wrapping_mul(31) ^ position.y.wrapping_mul(17)) as u8;
        // A quiet, seamless loop, offset per cell. Accessibility freezes its
        // pose, never its gameplay lifetime. Linger/fade belongs to bursts only.
        let duration = (style.frame_seconds() * style.frames().len() as f64).max(0.8);
        let progress = if reduced {
            0.42
        } else {
            (now / duration + f64::from(variant) / 256.0).rem_euclid(1.0) as f32
        };
        let frame =
            ((progress * style.frames().len() as f32) as usize).min(style.frames().len() - 1);
        let palette = self.palette_for(id);
        TerminalEffectSample {
            symbol: terminal_symbol(style.frames()[frame]),
            color: palette.color,
            accent_color: palette.accent_color,
            highlight_color: palette.highlight_color,
            outline: None,
            family: style.family(),
            progress,
            sustained: true,
            variant,
            links: 0,
            direction: (0.0, 0.0),
        }
    }

    #[cfg(any(test, debug_assertions))]
    pub(crate) fn active_count(&self) -> usize {
        self.active.len()
    }
}

struct ActiveVisualCue {
    cue: VisualCue,
    started_at: f64,
    steps: std::collections::BTreeMap<GridPos, u16>,
    links: std::collections::BTreeMap<GridPos, u8>,
}

impl ActiveVisualCue {
    fn sample_world(
        &self,
        position: GridPos,
        now: f64,
        styles: &VisualCueCatalog,
    ) -> Option<TerminalEffectSample> {
        let step = *self.steps.get(&position)?;
        let style = terminal_style(styles, self.cue.id());
        let elapsed = now - self.started_at;
        let local_time = elapsed - f64::from(step) * style.step_seconds();
        if local_time < 0.0 {
            return None;
        }
        let animation_duration = style.frame_seconds() * style.frames().len() as f64;
        if local_time >= animation_duration + style.linger_seconds() {
            return None;
        }
        let mut frame = ((local_time / style.frame_seconds()) as usize)
            .min(style.frames().len().saturating_sub(1));
        let outline = if matches!(
            style.family(),
            TerminalEffectFamily::ImpactWave | TerminalEffectFamily::BearerWave
        ) {
            let reach = (elapsed / style.step_seconds().max(0.001)).floor() as u16;
            let mut mask = 0;
            for (bit, (dx, dy)) in [(0, -1), (1, 0), (0, 1), (-1, 0)].into_iter().enumerate() {
                let neighbor =
                    GridPos::new(position.x.saturating_add(dx), position.y.saturating_add(dy));
                if self.steps.get(&neighbor).is_none_or(|step| *step > reach) {
                    mask |= 1 << bit;
                }
            }
            if mask == 0 && self.links.get(&position).copied().unwrap_or(0) == 0 {
                return None;
            }
            Some(mask)
        } else {
            None
        };
        if style.family() == TerminalEffectFamily::Conduction {
            let offset = (position.x.wrapping_mul(31) ^ position.y) as usize & 1;
            frame = (frame + offset) % style.frames().len();
        }
        let mut palette = style.palette();
        let progress = local_time / (animation_duration + style.linger_seconds());
        let alpha = (1.0 - ((progress - 0.6) / 0.4).max(0.0)).clamp(0.0, 1.0) as f32;
        palette.color.a *= alpha;
        palette.accent_color = palette.accent_color.map(|mut color| {
            color.a *= alpha;
            color
        });
        palette.highlight_color = palette.highlight_color.map(|mut color| {
            color.a *= alpha;
            color
        });
        Some(TerminalEffectSample {
            symbol: terminal_symbol(style.frames()[frame]),
            color: palette.color,
            accent_color: palette.accent_color,
            highlight_color: palette.highlight_color,
            outline,
            family: style.family(),
            progress: progress as f32,
            sustained: false,
            variant: (position.x.wrapping_mul(31) ^ position.y.wrapping_mul(17)) as u8,
            links: self.links.get(&position).copied().unwrap_or(0),
            direction: if style.family() == TerminalEffectFamily::FlameJet {
                // Cone rows have several possible predecessors. Use the
                // bearer-to-cell ray, not an arbitrary neighboring branch.
                match self.cue.target() {
                    VisualCueTarget::World { origin, .. } => {
                        let dx = (i64::from(position.x) - i64::from(origin.x)) as f32;
                        let dy = (i64::from(position.y) - i64::from(origin.y)) as f32;
                        let major = dx.abs().max(dy.abs()).max(1.0);
                        (dx / major, dy / major)
                    }
                    _ => (0.0, 0.0),
                }
            } else if matches!(
                style.family(),
                TerminalEffectFamily::Piercing
                    | TerminalEffectFamily::Impulse
                    | TerminalEffectFamily::ImpulseBlocked
                    | TerminalEffectFamily::Catalysis
                    | TerminalEffectFamily::Fracture
                    | TerminalEffectFamily::Alternation
                    | TerminalEffectFamily::Ricochet
            ) {
                self.piercing_direction(position, step)
            } else {
                (0.0, 0.0)
            },
        })
    }

    fn piercing_direction(&self, at: GridPos, step: u16) -> (f32, f32) {
        let previous = EFFECT_NEIGHBORS
            .iter()
            .find_map(|(dx, dy)| {
                let next = GridPos::new(at.x.saturating_add(*dx), at.y.saturating_add(*dy));
                self.steps
                    .get(&next)
                    .is_some_and(|previous| previous.checked_add(1) == Some(step))
                    .then_some(next)
            })
            .or_else(|| match self.cue.target() {
                VisualCueTarget::World { origin, .. } => Some(*origin),
                _ => None,
            })
            .unwrap_or(at);
        let dx = (i64::from(at.x) - i64::from(previous.x)) as f32;
        let dy = (i64::from(at.y) - i64::from(previous.y)) as f32;
        let major = dx.abs().max(dy.abs()).max(1.0);
        (dx / major, dy / major)
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

fn conduction_links(
    steps: &std::collections::BTreeMap<GridPos, u16>,
) -> std::collections::BTreeMap<GridPos, u8> {
    let mut links = std::collections::BTreeMap::new();
    for (&at, &step) in steps {
        let parent = EFFECT_NEIGHBORS
            .iter()
            .enumerate()
            .filter_map(|(index, &(dx, dy))| {
                let next = GridPos::new(at.x.saturating_add(dx), at.y.saturating_add(dy));
                let previous = *steps.get(&next)?;
                // Never bridge a missing cell, or cut a diagonal across an unknown
                // corner. Disconnected visible fragments remain local sparks.
                if previous >= step
                    || (dx != 0
                        && dy != 0
                        && (!steps.contains_key(&GridPos::new(next.x, at.y))
                            || !steps.contains_key(&GridPos::new(at.x, next.y))))
                {
                    return None;
                }
                Some((step - previous, index, next))
            })
            .min();
        if let Some((_, index, parent)) = parent {
            *links.entry(at).or_insert(0) |= 1 << index;
            *links.entry(parent).or_insert(0) |= 1 << ((index + 4) % 8);
        }
    }
    links
}

enum ResolvedTerminalCueStyle<'a> {
    Loaded(&'a TerminalCueStyle),
    Fallback,
}

impl ResolvedTerminalCueStyle<'_> {
    fn family(&self) -> TerminalEffectFamily {
        match self.frames()[0] {
            TerminalEffectGlyph::Fracture => TerminalEffectFamily::Fracture,
            TerminalEffectGlyph::Alternation => TerminalEffectFamily::Alternation,
            TerminalEffectGlyph::Echo => TerminalEffectFamily::Echo,
            TerminalEffectGlyph::Ricochet => TerminalEffectFamily::Ricochet,
            TerminalEffectGlyph::FlameJet => TerminalEffectFamily::FlameJet,
            TerminalEffectGlyph::Catalysis => TerminalEffectFamily::Catalysis,
            TerminalEffectGlyph::Frost => TerminalEffectFamily::Frost,
            TerminalEffectGlyph::Impulse => TerminalEffectFamily::Impulse,
            TerminalEffectGlyph::ImpulseBlocked => TerminalEffectFamily::ImpulseBlocked,
            TerminalEffectGlyph::Guard => TerminalEffectFamily::Guard,
            TerminalEffectGlyph::GuardBreak => TerminalEffectFamily::GuardBreak,
            TerminalEffectGlyph::Piercing => TerminalEffectFamily::Piercing,
            TerminalEffectGlyph::Restoration => TerminalEffectFamily::Restoration,
            TerminalEffectGlyph::ShockSmall | TerminalEffectGlyph::ShockWide => {
                TerminalEffectFamily::ImpactWave
            }
            TerminalEffectGlyph::RingSmall | TerminalEffectGlyph::RingWide => {
                TerminalEffectFamily::BearerWave
            }
            TerminalEffectGlyph::ArcFork | TerminalEffectGlyph::ArcSplit => {
                TerminalEffectFamily::Conduction
            }
            TerminalEffectGlyph::FlameSmall
            | TerminalEffectGlyph::Flame
            | TerminalEffectGlyph::FlameLarge => TerminalEffectFamily::Flame,
            TerminalEffectGlyph::AcidDrop
            | TerminalEffectGlyph::AcidSplash
            | TerminalEffectGlyph::AcidPool => {
                if self.frames().contains(&TerminalEffectGlyph::AcidPool) {
                    TerminalEffectFamily::Caustic
                } else {
                    TerminalEffectFamily::Corrosion
                }
            }
            _ => TerminalEffectFamily::Glyph,
        }
    }

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
        TerminalEffectGlyph::ShockSmall => '\u{e000}',
        TerminalEffectGlyph::ShockWide => '\u{e001}',
        TerminalEffectGlyph::RingSmall => '\u{e002}',
        TerminalEffectGlyph::RingWide => '\u{e003}',
        TerminalEffectGlyph::ArcFork => '\u{e004}',
        TerminalEffectGlyph::ArcSplit => '\u{e005}',
        TerminalEffectGlyph::AcidDrop => '\u{e006}',
        TerminalEffectGlyph::AcidSplash => '\u{e007}',
        TerminalEffectGlyph::AcidPool => '\u{e008}',
        TerminalEffectGlyph::Restoration => '+',
        TerminalEffectGlyph::Piercing => '-',
        TerminalEffectGlyph::Guard => '[',
        TerminalEffectGlyph::GuardBreak => ']',
        TerminalEffectGlyph::Impulse => '>',
        TerminalEffectGlyph::ImpulseBlocked => '|',
        TerminalEffectGlyph::Catalysis => '*',
        TerminalEffectGlyph::Fracture => '×',
        TerminalEffectGlyph::Alternation => '~',
        TerminalEffectGlyph::Echo => ':',
        TerminalEffectGlyph::Ricochet => '•',
        TerminalEffectGlyph::FlameJet => '^',
        TerminalEffectGlyph::Frost => '*',
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
    fn fracture_fragments_travel_from_the_impact_without_inventing_cells() {
        use project_rl::presentation::VisualCueCell;
        let id = cue_id("test:fracture");
        let mut catalog = VisualCueCatalog::default();
        catalog
            .register(VisualCueDefinition::new(
                id.clone(),
                TerminalCueStyle::new(
                    vec![TerminalEffectGlyph::Fracture],
                    [240, 220, 180, 255],
                    500,
                    45,
                    0,
                )
                .unwrap(),
            ))
            .unwrap();
        let mut player = VisualCuePlayer::with_catalog(catalog);
        let at = GridPos::new(3, 3);
        let east = GridPos::new(4, 3);
        player.play(
            VisualCue::world(
                id,
                at,
                [VisualCueCell::new(at, 0), VisualCueCell::new(east, 1)],
            )
            .unwrap(),
            0.0,
        );
        assert_eq!(
            player.sample_world(at, true, 0.2).unwrap().direction,
            (0.0, 0.0)
        );
        assert_eq!(
            player.sample_world(east, true, 0.2).unwrap().direction,
            (1.0, 0.0)
        );
        assert!(player.sample_world(east, false, 0.2).is_none());
        assert!(player.sample_world(GridPos::new(3, 4), true, 0.2).is_none());
    }

    #[test]
    fn sustained_ground_loops_without_fading_and_reduced_motion_is_stable() {
        use project_rl::combat::{DamagePacket, DamageType};
        use project_rl::effects::{GroundEffectMap, GroundEffectSpec};
        let mut catalog = VisualCueCatalog::default();
        for (id, glyph) in [
            ("core:burning_ground", TerminalEffectGlyph::Flame),
            ("core:caustic_ground", TerminalEffectGlyph::AcidPool),
            ("core:electrified_ground", TerminalEffectGlyph::ArcFork),
        ] {
            catalog
                .register(VisualCueDefinition::new(
                    cue_id(id),
                    TerminalCueStyle::new(vec![glyph], [180, 200, 230, 220], 120, 0, 80).unwrap(),
                ))
                .unwrap();
        }
        let player = VisualCuePlayer::with_catalog(catalog);
        let mut ground = GroundEffectMap::default();
        let at = GridPos::new(2, 2);
        for id in [
            "core:burning_ground",
            "core:caustic_ground",
            "core:electrified_ground",
            "mod:unknown",
        ] {
            ground.apply(
                at,
                None,
                &GroundEffectSpec::new(cue_id(id), 3, DamagePacket::new(1, DamageType::Thermal, 0))
                    .unwrap(),
                0,
            );
        }
        let original = ground.clone();
        let still = player.sample_ground(&ground, at, true, 0.0, true);
        assert_eq!(still.len(), 4);
        let animated = player.sample_ground(&ground, at, true, 0.0, false);
        for time in [0.1, 0.8, 10.0, 3600.0, 86_400.0] {
            let samples = player.sample_ground(&ground, at, true, time, false);
            assert_eq!(samples.len(), 4);
            for (sample, initial) in samples.iter().zip(&animated) {
                assert!(sample.sustained);
                assert_eq!(sample.color, initial.color);
                assert_eq!(sample.accent_color, initial.accent_color);
                assert!((0.0..1.0).contains(&sample.progress));
            }
            assert_eq!(player.sample_ground(&ground, at, true, time, true), still);
            assert!(
                player
                    .sample_ground(&ground, at, false, time, false)
                    .is_empty()
            );
        }
        assert_ne!(
            player.sample_ground(&ground, at, true, 0.15, false),
            animated
        );
        assert_eq!(ground, original);
        assert_eq!(player.active_count(), 0);
    }

    #[test]
    fn persistent_fields_are_not_limited_by_transient_queue_and_reconstruct_without_events() {
        use project_rl::combat::{DamagePacket, DamageType};
        use project_rl::effects::{GroundEffectMap, GroundEffectSpec};
        let mut player = VisualCuePlayer::default();
        let mut ground = GroundEffectMap::default();
        let id = cue_id("mod:ground");
        let spec =
            GroundEffectSpec::new(id.clone(), 3, DamagePacket::new(1, DamageType::Thermal, 0))
                .unwrap();
        for x in 0..64 {
            ground.apply(GridPos::new(x, 0), None, &spec, 0);
            player.play(VisualCue::point(id.clone(), GridPos::new(x, 0)), 0.0);
        }
        player.clear_world();
        let fresh = VisualCuePlayer::default();
        for x in 0..64 {
            let at = GridPos::new(x, 0);
            let sample = player.sample_ground(&ground, at, true, 900.0, false);
            assert_eq!(sample.len(), 1);
            assert_eq!(fresh.sample_ground(&ground, at, true, 900.0, false), sample);
        }
        let at = GridPos::new(2, 0);
        ground.elapse(at, &id);
        ground.apply(at, None, &spec, 1);
        assert_eq!(
            player.sample_ground(&ground, at, true, 900.0, false).len(),
            1
        );
        for _ in 0..3 {
            ground.elapse(at, &id);
        }
        assert!(
            player
                .sample_ground(&ground, at, true, 900.0, false)
                .is_empty()
        );
        assert_eq!(
            player
                .sample_ground(&ground, GridPos::new(3, 0), true, 900.0, false)
                .len(),
            1
        );
    }

    #[test]
    fn piercing_trace_has_direction_without_painting_the_impact_or_unseen_cells() {
        use project_rl::presentation::VisualCueCell;
        let id = cue_id("test:piercing");
        let mut catalog = VisualCueCatalog::default();
        catalog
            .register(VisualCueDefinition::new(
                id.clone(),
                TerminalCueStyle::new(
                    vec![TerminalEffectGlyph::Piercing],
                    [255, 245, 225, 255],
                    300,
                    75,
                    20,
                )
                .unwrap(),
            ))
            .unwrap();
        let mut player = VisualCuePlayer::with_catalog(catalog);
        let origin = GridPos::new(3, 3);
        player.play(
            VisualCue::world(
                id,
                origin,
                [
                    VisualCueCell::new(GridPos::new(4, 3), 1),
                    VisualCueCell::new(GridPos::new(5, 4), 2),
                ],
            )
            .unwrap(),
            10.0,
        );
        assert!(player.sample_world(origin, true, 10.2).is_none());
        assert!(
            player
                .sample_world(GridPos::new(4, 3), true, 10.01)
                .is_none()
        );
        let first = player.sample_world(GridPos::new(4, 3), true, 10.2).unwrap();
        assert_eq!(first.family, TerminalEffectFamily::Piercing);
        assert_eq!(first.direction, (1.0, 0.0));
        assert_eq!(
            player
                .sample_world(GridPos::new(5, 4), true, 10.2)
                .unwrap()
                .direction,
            (1.0, 1.0)
        );
        assert!(
            player
                .sample_world(GridPos::new(5, 4), false, 10.2)
                .is_none()
        );
        assert!(
            player
                .sample_world(GridPos::new(6, 4), true, 10.2)
                .is_none()
        );
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

    #[test]
    fn new_effect_silhouettes_are_distinct_animated_and_fade_without_exposing_hidden_cells() {
        let shapes = [
            TerminalEffectGlyph::ShockSmall,
            TerminalEffectGlyph::RingSmall,
            TerminalEffectGlyph::ArcFork,
            TerminalEffectGlyph::Flame,
            TerminalEffectGlyph::AcidDrop,
        ];
        let symbols = shapes
            .into_iter()
            .map(terminal_symbol)
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(symbols.len(), 5);
        let id = cue_id("core:test_acid");
        let mut catalog = VisualCueCatalog::default();
        catalog
            .register(VisualCueDefinition::new(
                id.clone(),
                TerminalCueStyle::new(
                    vec![
                        TerminalEffectGlyph::AcidDrop,
                        TerminalEffectGlyph::AcidSplash,
                        TerminalEffectGlyph::AcidPool,
                    ],
                    [120, 225, 95, 255],
                    100,
                    50,
                    0,
                )
                .unwrap(),
            ))
            .unwrap();
        let at = GridPos::new(4, 2);
        let mut player = VisualCuePlayer::with_catalog(catalog);
        player.play(VisualCue::point(id, at), 0.0);
        assert_ne!(
            player.sample_world(at, true, 0.02).unwrap().symbol,
            player.sample_world(at, true, 0.12).unwrap().symbol
        );
        assert!(player.sample_world(at, true, 0.28).unwrap().color.a < 0.3);
        assert!(player.sample_world(at, false, 0.02).is_none());
        assert!(player.sample_world(at, true, 0.31).is_none());
    }

    #[test]
    fn electrical_branches_are_reciprocal_and_never_bridge_missing_cells() {
        let at = GridPos::new;
        let cells = [
            (at(0, 0), 0),
            (at(1, 0), 1),
            (at(2, 0), 2),
            (at(2, 1), 3),
            (at(4, 0), 4),
            (at(3, 2), 4),
        ]
        .into();
        let links = conduction_links(&cells);
        assert_eq!(links[&at(0, 0)], 1 << 2);
        assert_eq!(links[&at(1, 0)], (1 << 6) | (1 << 2));
        assert_eq!(links[&at(2, 0)], (1 << 6) | (1 << 4));
        assert_eq!(links[&at(2, 1)], 1);
        assert!(!links.contains_key(&at(4, 0)), "missing cell was bridged");
        assert!(
            !links.contains_key(&at(3, 2)),
            "unknown diagonal corner was crossed"
        );
        for (&position, &mask) in &links {
            for (index, &(dx, dy)) in EFFECT_NEIGHBORS.iter().enumerate() {
                if mask & (1 << index) != 0 {
                    let next = at(position.x + dx, position.y + dy);
                    assert_ne!(links[&next] & (1 << ((index + 4) % 8)), 0);
                }
            }
        }
    }

    #[test]
    fn each_motion_family_progresses_but_reduced_motion_is_frozen_and_visibility_gated() {
        for (frames, family) in [
            (
                vec![TerminalEffectGlyph::Ricochet],
                TerminalEffectFamily::Ricochet,
            ),
            (
                vec![TerminalEffectGlyph::FlameJet],
                TerminalEffectFamily::FlameJet,
            ),
            (
                vec![TerminalEffectGlyph::Alternation],
                TerminalEffectFamily::Alternation,
            ),
            (vec![TerminalEffectGlyph::Echo], TerminalEffectFamily::Echo),
            (
                vec![TerminalEffectGlyph::Fracture],
                TerminalEffectFamily::Fracture,
            ),
            (
                vec![TerminalEffectGlyph::Frost],
                TerminalEffectFamily::Frost,
            ),
            (
                vec![TerminalEffectGlyph::Catalysis],
                TerminalEffectFamily::Catalysis,
            ),
            (
                vec![TerminalEffectGlyph::Impulse],
                TerminalEffectFamily::Impulse,
            ),
            (
                vec![TerminalEffectGlyph::ImpulseBlocked],
                TerminalEffectFamily::ImpulseBlocked,
            ),
            (
                vec![TerminalEffectGlyph::Guard],
                TerminalEffectFamily::Guard,
            ),
            (
                vec![TerminalEffectGlyph::GuardBreak],
                TerminalEffectFamily::GuardBreak,
            ),
            (
                vec![TerminalEffectGlyph::Piercing],
                TerminalEffectFamily::Piercing,
            ),
            (
                vec![TerminalEffectGlyph::Restoration],
                TerminalEffectFamily::Restoration,
            ),
            (
                vec![TerminalEffectGlyph::ShockSmall],
                TerminalEffectFamily::ImpactWave,
            ),
            (
                vec![TerminalEffectGlyph::RingWide],
                TerminalEffectFamily::BearerWave,
            ),
            (
                vec![TerminalEffectGlyph::ArcSplit],
                TerminalEffectFamily::Conduction,
            ),
            (
                vec![TerminalEffectGlyph::Flame],
                TerminalEffectFamily::Flame,
            ),
            (
                vec![TerminalEffectGlyph::AcidDrop],
                TerminalEffectFamily::Corrosion,
            ),
            (
                vec![TerminalEffectGlyph::AcidDrop, TerminalEffectGlyph::AcidPool],
                TerminalEffectFamily::Caustic,
            ),
        ] {
            let id = cue_id("test:motion");
            let mut catalog = VisualCueCatalog::default();
            catalog
                .register(VisualCueDefinition::new(
                    id.clone(),
                    TerminalCueStyle::new(frames, [120, 220, 250, 255], 300, 50, 20).unwrap(),
                ))
                .unwrap();
            let mut player = VisualCuePlayer::with_catalog(catalog);
            let at = GridPos::new(1, 2);
            player.play(VisualCue::point(id, at), 10.0);
            let first = player.sample_world(at, true, 10.02).unwrap();
            let later = player.sample_world(at, true, 10.2).unwrap();
            assert_eq!(first.family, family);
            assert!(later.progress > first.progress);
            assert_eq!(first.variant, later.variant);
            assert_eq!(
                player.sample_world_with_motion(at, true, 10.02, true),
                player.sample_world_with_motion(at, true, 10.25, true)
            );
            assert!(
                player
                    .sample_world_with_motion(at, false, 10.02, true)
                    .is_none()
            );
            assert!(
                player
                    .sample_world_with_motion(at, true, 10.36, true)
                    .is_none()
            );
            assert!(player.sample_world(at, false, 10.2).is_none());
            assert!(player.sample_world(at, true, 12.0).is_none());
        }
    }

    #[test]
    fn electrical_wave_keeps_internal_branches_without_tiling_outlines() {
        let id = cue_id("core:test_wave");
        let mut catalog = VisualCueCatalog::default();
        catalog
            .register(VisualCueDefinition::new(
                id.clone(),
                TerminalCueStyle::new(
                    vec![
                        TerminalEffectGlyph::RingSmall,
                        TerminalEffectGlyph::RingWide,
                    ],
                    [180, 160, 250, 255],
                    100,
                    100,
                    50,
                )
                .unwrap(),
            ))
            .unwrap();
        let mut player = VisualCuePlayer::with_catalog(catalog);
        let cells = (-1_i32..=1).flat_map(|x| {
            (-1_i32..=1).map(move |y| {
                project_rl::presentation::VisualCueCell::new(
                    GridPos::new(x, y),
                    x.abs().max(y.abs()) as u16,
                )
            })
        });
        player.play(
            VisualCue::world(id, GridPos::new(0, 0), cells).unwrap(),
            0.0,
        );
        assert_eq!(
            player
                .sample_world(GridPos::new(0, 0), true, 0.02)
                .unwrap()
                .outline,
            Some(15)
        );
        let interior = player.sample_world(GridPos::new(0, 0), true, 0.12).unwrap();
        assert_eq!(interior.outline, Some(0));
        assert_ne!(interior.links, 0);
        assert_eq!(
            player
                .sample_world(GridPos::new(1, 0), true, 0.12)
                .unwrap()
                .outline,
            Some(2)
        );
        assert!(
            player
                .sample_world(GridPos::new(1, 0), false, 0.12)
                .is_none()
        );
        assert_eq!(
            player.sample_world_with_motion(GridPos::new(1, 0), true, 0.02, true),
            player.sample_world_with_motion(GridPos::new(1, 0), true, 0.25, true)
        );
        assert!(
            player
                .sample_world_with_motion(GridPos::new(1, 0), true, 0.36, true)
                .is_none()
        );
    }
}
