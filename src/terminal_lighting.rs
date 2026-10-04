//! Local presentation lights. No live map access, simulation state or save data.
use std::cell::RefCell;
use std::collections::BTreeMap;

use macroquad::prelude::*;
use project_rl::world::{GridPos, VisibilityState};

use super::{GridCamera, KnownTile, terminal_art};
use crate::test_sector::Decor;

const MARGIN: i32 = 3;
const TEXTURE_SIDE: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Kind {
    Wall([bool; 4]),
    Lamp,
    Server,
    Terminal,
    UsedTerminal,
    Control,
}

impl Kind {
    fn ray_origin(self, at: GridPos) -> GridPos {
        match self {
            Self::Wall([_, east, south, west]) if east || west => {
                GridPos::new(at.x, at.y + if south { -1 } else { 1 })
            }
            Self::Wall([_, east, _, _]) => GridPos::new(at.x + if east { -1 } else { 1 }, at.y),
            _ => at,
        }
    }
    fn faces(self, from: GridPos, to: GridPos) -> bool {
        match self {
            Self::Wall([_, east, south, west]) if east || west => {
                if south {
                    to.y < from.y
                } else {
                    to.y > from.y
                }
            }
            Self::Wall([_, east, _, _]) => {
                if east {
                    to.x < from.x
                } else {
                    to.x > from.x
                }
            }
            _ => true,
        }
    }
    fn radius(self) -> f32 {
        match self {
            Self::Wall(_) | Self::Lamp => 2.3,
            Self::Control => 1.5,
            _ => 1.8,
        }
    }
    fn ink(self) -> Color {
        match self {
            Self::Wall(_) | Self::Lamp => Color::from_rgba(174, 203, 207, 32),
            Self::Control => Color::from_rgba(219, 169, 80, 40),
            Self::UsedTerminal => Color::from_rgba(143, 174, 153, 23),
            _ => Color::from_rgba(125, 231, 157, 44),
        }
    }
    fn pulse(self, at: GridPos, time: f64, fixed: bool) -> f32 {
        if fixed || !matches!(self, Self::Server | Self::Terminal) {
            return 1.;
        }
        let phase = f64::from(at.x.rem_euclid(7) + at.y.rem_euclid(5)) * 0.73;
        0.88 + 0.12 * (time * 1.3 + phase).sin() as f32
    }
    fn indicator(self, at: GridPos, cell: f32) -> Rect {
        let pixel = cell / 16.;
        match self {
            Self::Wall(joins) => {
                terminal_art::wall_lamp_rect(cell, Decor::Wall, joins, at).unwrap()
            }
            Self::Server => Rect::new(11. * pixel, 12. * pixel, pixel, pixel),
            Self::Terminal | Self::UsedTerminal | Self::Control => {
                Rect::new(4. * pixel, 5. * pixel, 8. * pixel, pixel)
            }
            Self::Lamp => Rect::new(cell * 0.4, cell * 0.4, cell * 0.2, cell * 0.2),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Light {
    at: GridPos,
    kind: Kind,
    receivers: Vec<GridPos>,
}

type Sample = (GridPos, KnownTile, [bool; 4]);

#[derive(Default)]
struct Cache {
    first: Option<GridPos>,
    size: (i32, i32),
    cells: Vec<Sample>,
    lights: Vec<Light>,
    scratch: Vec<Sample>,
}

impl Cache {
    fn refresh(
        &mut self,
        camera: &GridCamera,
        visibility: &VisibilityState,
        known_at: impl Fn(GridPos) -> Option<KnownTile>,
    ) -> bool {
        self.scratch.clear();
        sample_visible(camera, visibility, known_at, &mut self.scratch);
        let size = (camera.columns, camera.rows);
        if self.first == Some(camera.first) && self.size == size && self.cells == self.scratch {
            return false;
        }
        self.lights = build_plan(&self.scratch);
        std::mem::swap(&mut self.cells, &mut self.scratch);
        self.first = Some(camera.first);
        self.size = size;
        true
    }
}

thread_local! {
    static CACHE: RefCell<Cache> = RefCell::new(Cache::default());
    static HALO: Texture2D = {
        let mut image = Image::gen_image_color(TEXTURE_SIDE as u16, TEXTURE_SIDE as u16, WHITE);
        for y in 0..TEXTURE_SIDE {
            for x in 0..TEXTURE_SIDE {
                let dx = (x as f32 + 0.5) / TEXTURE_SIDE as f32 * 2. - 1.;
                let dy = (y as f32 + 0.5) / TEXTURE_SIDE as f32 * 2. - 1.;
                let alpha = (1. - (dx * dx + dy * dy).sqrt()).max(0.).powi(2);
                image.bytes[(y * TEXTURE_SIDE + x) * 4 + 3] = (alpha * 255.) as u8;
            }
        }
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Linear);
        texture
    };
    #[cfg(debug_assertions)]
    static DIAGNOSTIC: std::cell::Cell<(bool, bool, Option<f64>)> = const { std::cell::Cell::new((true, false, None)) };
    #[cfg(debug_assertions)]
    static LAST_COST: std::cell::Cell<(f64, bool, usize, usize)> = const { std::cell::Cell::new((0., false, 0, 0)) };
}

pub(super) fn prewarm() {
    HALO.with(|_| {});
}

#[cfg(debug_assertions)]
pub(super) fn set_diagnostic(enabled: bool, fixed: bool, time: Option<f64>) {
    DIAGNOSTIC.with(|mode| mode.set((enabled, fixed, time)));
}

#[cfg(debug_assertions)]
pub(super) fn diagnostic_cost() -> (f64, bool, usize, usize) {
    LAST_COST.with(|cost| cost.get())
}

fn light_kind(at: GridPos, tile: KnownTile, joins: [bool; 4]) -> Option<Kind> {
    Some(match tile.decor {
        Decor::Wall => {
            terminal_art::wall_lamp_rect(20., tile.decor, joins, at)?;
            Kind::Wall(joins)
        }
        Decor::LampPost => Kind::Lamp,
        Decor::Server | Decor::ElectricalCabinet => Kind::Server,
        Decor::DataTerminalOnline => Kind::Terminal,
        Decor::DataTerminalUpdated => Kind::UsedTerminal,
        Decor::ControlReady => Kind::Control,
        _ => return None,
    })
}

// Supercover between cell centres, rejecting unknown cells and diagonal cracks.
// The emitter can occupy a wall; every other crossed cell must be transparent.
fn clear_path(from: GridPos, to: GridPos, cells: &BTreeMap<GridPos, KnownTile>) -> bool {
    let transparent = |at| cells.get(&at).is_some_and(|t| !t.terrain.blocks_vision());
    let dx = (to.x - from.x).abs();
    let dy = (to.y - from.y).abs();
    let sx = (to.x - from.x).signum();
    let sy = (to.y - from.y).signum();
    let (mut ix, mut iy) = (0, 0);
    let mut at = from;
    while ix < dx || iy < dy {
        let decision = (1 + 2 * ix) * dy - (1 + 2 * iy) * dx;
        if decision == 0 {
            if !transparent(GridPos::new(at.x + sx, at.y))
                || !transparent(GridPos::new(at.x, at.y + sy))
            {
                return false;
            }
            at.x += sx;
            at.y += sy;
            ix += 1;
            iy += 1;
        } else if decision < 0 {
            at.x += sx;
            ix += 1;
        } else {
            at.y += sy;
            iy += 1;
        }
        if !transparent(at) {
            return false;
        }
    }
    true
}

fn build_plan(entries: &[Sample]) -> Vec<Light> {
    let cells: BTreeMap<_, _> = entries.iter().map(|&(at, tile, _)| (at, tile)).collect();
    let mut lights = Vec::new();
    for &(at, tile, joins) in entries {
        let Some(kind) = light_kind(at, tile, joins) else {
            continue;
        };
        // A wall lamp shines from its exposed face. Casting from the middle
        // of its solid tile would incorrectly reject every diagonal floor cell.
        let origin = kind.ray_origin(at);
        let face_clear = origin == at
            || cells
                .get(&origin)
                .is_some_and(|t| !t.terrain.blocks_vision());
        let mut receivers = Vec::new();
        for y in -MARGIN..=MARGIN {
            for x in -MARGIN..=MARGIN {
                // Include the edge cells touched by the small circular halo.
                let near_x = (x.abs() as f32 - 0.5).max(0.);
                let near_y = (y.abs() as f32 - 0.5).max(0.);
                if near_x * near_x + near_y * near_y > kind.radius().powi(2) {
                    continue;
                }
                let target = GridPos::new(at.x + x, at.y + y);
                if face_clear
                    && kind.faces(at, target)
                    && cells
                        .get(&target)
                        .is_some_and(|t| !t.terrain.blocks_movement())
                    && clear_path(origin, target, &cells)
                {
                    receivers.push(target);
                }
            }
        }
        lights.push(Light {
            at,
            kind,
            receivers,
        });
    }
    lights
}

fn sample_visible(
    camera: &GridCamera,
    visibility: &VisibilityState,
    known_at: impl Fn(GridPos) -> Option<KnownTile>,
    output: &mut Vec<Sample>,
) {
    for at in visibility.visible_positions() {
        if at.x >= camera.first.x - MARGIN
            && at.x < camera.first.x + camera.columns + MARGIN
            && at.y >= camera.first.y - MARGIN
            && at.y < camera.first.y + camera.rows + MARGIN
            && let Some(tile) = known_at(at)
        {
            // Match the wall drawing's remembered joins, even at the FOV edge.
            let joins = if tile.decor == Decor::Wall {
                at.cardinal_neighbors().map(|n| {
                    known_at(n).is_some_and(|t| t.terrain == project_rl::world::Terrain::Wall)
                })
            } else {
                [false; 4]
            };
            output.push((at, tile, joins));
        }
    }
}

pub(super) fn draw(
    camera: &GridCamera,
    visibility: &VisibilityState,
    known_at: impl Fn(GridPos) -> Option<KnownTile>,
    reduced_motion: bool,
) {
    #[cfg(debug_assertions)]
    let (enabled, fixed, override_time) = DIAGNOSTIC.with(|mode| mode.get());
    #[cfg(not(debug_assertions))]
    let (enabled, fixed, override_time) = (true, false, None);
    if !enabled {
        #[cfg(debug_assertions)]
        LAST_COST.with(|cost| cost.set((0., false, 0, 0)));
        return;
    }
    #[cfg(debug_assertions)]
    let start = std::time::Instant::now();
    let fixed = fixed || reduced_motion;
    let time = override_time.unwrap_or_else(get_time);
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        let rebuilt = cache.refresh(camera, visibility, &known_at);
        #[cfg(not(debug_assertions))]
        let _ = rebuilt;
        HALO.with(|texture| {
            for light in &cache.lights {
                let rect = camera.rect(light.at);
                let indicator = light.kind.indicator(light.at, camera.cell);
                let centre = vec2(
                    rect.x + indicator.x + indicator.w * 0.5,
                    rect.y + indicator.y + indicator.h * 0.5,
                );
                let diameter = light.kind.radius() * camera.cell * 2.;
                let pulse = light.kind.pulse(light.at, time, fixed);
                let mut ink = light.kind.ink();
                ink.a *= pulse;
                for &at in &light.receivers {
                    if !camera.contains(at) {
                        continue;
                    }
                    let target = camera.rect(at);
                    // One cropped texture per visible floor cell: no spill into walls or fog.
                    draw_texture_ex(
                        texture,
                        target.x,
                        target.y,
                        ink,
                        DrawTextureParams {
                            source: Some(Rect::new(
                                ((target.x - centre.x) / diameter + 0.5) * TEXTURE_SIDE as f32,
                                ((target.y - centre.y) / diameter + 0.5) * TEXTURE_SIDE as f32,
                                target.w / diameter * TEXTURE_SIDE as f32,
                                target.h / diameter * TEXTURE_SIDE as f32,
                            )),
                            dest_size: Some(vec2(target.w, target.h)),
                            ..Default::default()
                        },
                    );
                }
                if camera.contains(light.at) && matches!(light.kind, Kind::Server | Kind::Terminal)
                {
                    // Small live LED only; the approved prop silhouette stays untouched.
                    draw_rectangle(
                        rect.x + indicator.x,
                        rect.y + indicator.y,
                        indicator.w,
                        indicator.h,
                        Color::new(0.65 * pulse, 0.95 * pulse, 0.71 * pulse, 0.8),
                    );
                }
            }
        });
        #[cfg(debug_assertions)]
        LAST_COST.with(|cost| {
            cost.set((
                start.elapsed().as_secs_f64() * 1000.,
                rebuilt,
                cache.lights.len(),
                cache
                    .lights
                    .iter()
                    .map(|light| {
                        light
                            .receivers
                            .iter()
                            .filter(|at| camera.contains(**at))
                            .count()
                    })
                    .sum(),
            ))
        });
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use project_rl::world::{DoorState, Terrain};

    fn room() -> BTreeMap<GridPos, KnownTile> {
        (-4..=4)
            .flat_map(|y| {
                (-4..=4).map(move |x| {
                    (
                        GridPos::new(x, y),
                        KnownTile {
                            terrain: Terrain::Floor,
                            decor: Decor::Deck,
                        },
                    )
                })
            })
            .collect()
    }
    fn plan(cells: &BTreeMap<GridPos, KnownTile>) -> Vec<Light> {
        build_plan(
            &cells
                .iter()
                .map(|(&at, &tile)| {
                    (
                        at,
                        tile,
                        at.cardinal_neighbors()
                            .map(|n| cells.get(&n).is_some_and(|t| t.terrain == Terrain::Wall)),
                    )
                })
                .collect::<Vec<_>>(),
        )
    }
    #[test]
    fn closed_partition_and_doors_stop_a_local_light() {
        let mut cells = room();
        let source = GridPos::new(-1, 0);
        cells.insert(
            source,
            KnownTile {
                terrain: Terrain::Wall,
                decor: Decor::Server,
            },
        );
        for y in -4..=4 {
            cells.insert(
                GridPos::new(0, y),
                KnownTile {
                    terrain: Terrain::Wall,
                    decor: Decor::DeadScreen,
                },
            );
        }
        for state in [
            DoorState::Closed,
            DoorState::Locked,
            DoorState::Unpowered,
            DoorState::Open,
        ] {
            cells.insert(
                GridPos::new(0, 0),
                KnownTile {
                    terrain: Terrain::Door(state),
                    decor: Decor::DoorClosed,
                },
            );
            let lights = plan(&cells);
            let light = lights.iter().find(|light| light.at == source).unwrap();
            assert_eq!(
                light.receivers.contains(&GridPos::new(1, 0)),
                state == DoorState::Open
            );
            assert!(
                light
                    .receivers
                    .iter()
                    .all(|at| !cells[at].terrain.blocks_movement())
            );
        }
    }
    #[test]
    fn unknown_cells_and_closed_diagonal_corners_block_light() {
        let mut cells = room();
        cells.insert(
            GridPos::new(0, 0),
            KnownTile {
                terrain: Terrain::Wall,
                decor: Decor::Server,
            },
        );
        cells.remove(&GridPos::new(1, 0));
        assert!(!clear_path(GridPos::new(0, 0), GridPos::new(2, 0), &cells));
        assert!(!clear_path(GridPos::new(0, 0), GridPos::new(1, 1), &cells));
        cells.insert(
            GridPos::new(1, 0),
            KnownTile {
                terrain: Terrain::Wall,
                decor: Decor::DeadScreen,
            },
        );
        assert!(!clear_path(GridPos::new(0, 0), GridPos::new(1, 1), &cells));
    }
    #[test]
    fn hidden_and_remembered_sources_cannot_enter_the_render_plan() {
        use project_rl::world::{DistanceMetric, FieldOfViewRules, Map};
        let map = Map::filled(20, 20, Terrain::Floor).unwrap();
        let mut visibility = VisibilityState::default();
        let rules = FieldOfViewRules {
            radius: 2,
            distance_metric: DistanceMetric::Euclidean,
            block_closed_corners: true,
        };
        let source = GridPos::new(5, 5);
        let camera = GridCamera {
            first: GridPos::new(0, 0),
            columns: 20,
            rows: 20,
            origin: vec2(0., 0.),
            cell: 20.,
        };
        let mut remembered: BTreeMap<_, _> = (0..20)
            .flat_map(|y| {
                (0..20).map(move |x| {
                    (
                        GridPos::new(x, y),
                        KnownTile {
                            terrain: Terrain::Floor,
                            decor: Decor::Deck,
                        },
                    )
                })
            })
            .collect();
        remembered.insert(
            source,
            KnownTile {
                terrain: Terrain::Wall,
                decor: Decor::Server,
            },
        );
        visibility.recompute(&map, source, rules);
        let mut cache = Cache::default();
        assert!(cache.refresh(&camera, &visibility, |at| remembered.get(&at).copied()));
        assert_eq!(cache.lights.len(), 1);
        assert!(!cache.refresh(&camera, &visibility, |at| remembered.get(&at).copied()));
        let mut samples = Vec::new();
        sample_visible(
            &camera,
            &visibility,
            |at| remembered.get(&at).copied(),
            &mut samples,
        );
        assert_eq!(build_plan(&samples).len(), 1);
        visibility.recompute(&map, GridPos::new(12, 12), rules);
        assert!(cache.refresh(&camera, &visibility, |at| remembered.get(&at).copied()));
        assert!(cache.lights.is_empty());
        assert!(visibility.is_explored(source));
        assert!(!visibility.is_visible(source));
        samples.clear();
        sample_visible(
            &camera,
            &visibility,
            |at| remembered.get(&at).copied(),
            &mut samples,
        );
        assert!(build_plan(&samples).is_empty());
        let before = samples.clone();
        remembered.insert(
            source,
            KnownTile {
                terrain: Terrain::Floor,
                decor: Decor::DataTerminalOnline,
            },
        );
        samples.clear();
        sample_visible(
            &camera,
            &visibility,
            |at| remembered.get(&at).copied(),
            &mut samples,
        );
        assert_eq!(before, samples);
        assert!(build_plan(&samples).is_empty());
        assert!(!cache.refresh(&camera, &visibility, |at| remembered.get(&at).copied()));
    }
    #[test]
    fn reduced_motion_is_constant_and_dynamic_variation_is_gentle() {
        for kind in [Kind::Server, Kind::Terminal] {
            let at = GridPos::new(-5, 8);
            assert_eq!(kind.pulse(at, 0., true), kind.pulse(at, 99., true));
            for step in 0..200 {
                assert!((0.76..=1.).contains(&kind.pulse(at, step as f64 * 0.1, false)));
            }
            assert_ne!(kind.pulse(at, 0., false), kind.pulse(at, 2., false));
        }
        assert_eq!(Kind::Lamp.pulse(GridPos::new(0, 0), 10., false), 1.);
    }
    #[test]
    fn offline_and_inactive_terminals_do_not_emit() {
        let mut cells = room();
        for decor in [
            Decor::DataTerminalOffline,
            Decor::Console,
            Decor::ControlUsed,
        ] {
            cells.insert(
                GridPos::new(0, 0),
                KnownTile {
                    terrain: Terrain::Wall,
                    decor,
                },
            );
            assert!(plan(&cells).is_empty());
        }
        cells.insert(
            GridPos::new(0, 0),
            KnownTile {
                terrain: Terrain::Wall,
                decor: Decor::DataTerminalOnline,
            },
        );
        assert_eq!(plan(&cells).len(), 1);
    }

    #[test]
    fn wall_lamps_only_light_the_side_where_their_strip_is_drawn() {
        let mut cells = room();
        let at = GridPos::new(1, 0);
        for x in -4..=4 {
            cells.insert(
                GridPos::new(x, 0),
                KnownTile {
                    terrain: Terrain::Wall,
                    decor: Decor::Wall,
                },
            );
        }
        let lights = plan(&cells);
        let lamp = lights.iter().find(|light| light.at == at).unwrap();
        assert!(lamp.receivers.contains(&GridPos::new(1, 1)));
        assert!(lamp.receivers.contains(&GridPos::new(2, 1)));
        assert!(lamp.receivers.iter().all(|target| target.y > at.y));
        assert!(!lamp.receivers.contains(&GridPos::new(1, -1)));
    }
}
