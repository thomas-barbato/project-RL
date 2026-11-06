use super::{Decor, GridCamera, GridPos, KnownTile, TerminalView, Terrain, terminal_roads};

#[derive(Default)]
pub(super) struct PresentationCache {
    pub viewport: Option<TerrainViewport>,
    pub radar: Option<RadarTerrain>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct TerrainWindow {
    first: GridPos,
    columns: i32,
    rows: i32,
    tiled: bool,
    surface_city: bool,
}

impl TerrainWindow {
    pub fn new(camera: &GridCamera, tiled: bool, surface_city: bool) -> Self {
        Self {
            first: camera.first,
            columns: camera.columns,
            rows: camera.rows,
            tiled,
            surface_city,
        }
    }
}

pub(super) struct TerrainCell {
    pub position: GridPos,
    pub decor: Decor,
    pub joins: [bool; 4],
    pub sidewalk: bool,
    pub curb: [bool; 4],
    pub marking: Option<terminal_roads::Marking>,
}

pub(super) struct TerrainViewport {
    pub window: TerrainWindow,
    pub tiles: Vec<TerrainCell>,
    first: GridPos,
    columns: i32,
    rows: i32,
    known: Vec<Option<KnownTile>>,
}

impl TerrainViewport {
    pub fn new(view: &TerminalView, window: TerrainWindow) -> Self {
        // Road markings inspect up to six cells outside the drawable window.
        const ROAD_MARGIN: i32 = 6;
        let first = GridPos::new(window.first.x - ROAD_MARGIN, window.first.y - ROAD_MARGIN);
        let columns = window.columns + 2 * ROAD_MARGIN;
        let rows = window.rows + 2 * ROAD_MARGIN;
        let known = (0..rows)
            .flat_map(|y| {
                (0..columns).map(move |x| view.known(GridPos::new(first.x + x, first.y + y)))
            })
            .collect();
        let mut viewport = Self {
            window,
            tiles: Vec::new(),
            first,
            columns,
            rows,
            known,
        };
        if window.tiled {
            let known_at = |at| viewport.known_at(view, at);
            let street_decor = |at| {
                known_at(at).map(|tile| match tile.decor {
                    Decor::Lane if window.surface_city => Decor::Street,
                    decor => decor,
                })
            };
            let positions = (0..window.rows).flat_map(|y| {
                (0..window.columns)
                    .map(move |x| GridPos::new(window.first.x + x, window.first.y + y))
            });
            viewport.tiles = positions
                .filter_map(|position| {
                    let decor = known_at(position)?.decor;
                    let sidewalk = terminal_roads::sidewalk(position, decor, street_decor);
                    let joins = position
                        .cardinal_neighbors()
                        .map(|at| known_at(at).is_some_and(|tile| tile.terrain == Terrain::Wall));
                    Some(TerrainCell {
                        position,
                        decor,
                        joins,
                        sidewalk,
                        curb: if sidewalk {
                            terminal_roads::curb_edges(position, street_decor)
                        } else {
                            [false; 4]
                        },
                        marking: terminal_roads::prepare_marking(decor, position, |at| {
                            known_at(at).map(|tile| tile.decor)
                        }),
                    })
                })
                .collect();
        }
        viewport
    }

    pub fn known_at(&self, view: &TerminalView, at: GridPos) -> Option<KnownTile> {
        let x = at.x - self.first.x;
        let y = at.y - self.first.y;
        if (0..self.columns).contains(&x) && (0..self.rows).contains(&y) {
            self.known[(y * self.columns + x) as usize]
        } else {
            view.known(at)
        }
    }
}

pub(super) enum RadarTile {
    Wall([bool; 4]),
    Floor { show_in_memory: bool },
}

pub(super) struct RadarTerrain {
    pub corners: Option<[GridPos; 2]>,
    pub tiles: Vec<(GridPos, RadarTile)>,
}

impl RadarTerrain {
    pub fn new(view: &TerminalView) -> Self {
        let mut corners: Option<[GridPos; 2]> = None;
        let tiles = view
            .remembered
            .iter()
            .filter_map(|(&position, tile)| {
                let [min, max] = corners.get_or_insert([position, position]);
                *min = GridPos::new(min.x.min(position.x), min.y.min(position.y));
                *max = GridPos::new(max.x.max(position.x), max.y.max(position.y));
                let tile = if tile.terrain.blocks_movement() {
                    let edges = position.cardinal_neighbors().map(|at| {
                        !view
                            .remembered
                            .get(&at)
                            .is_some_and(|known| known.terrain.blocks_movement())
                    });
                    if !edges.contains(&true) {
                        return None;
                    }
                    RadarTile::Wall(edges)
                } else {
                    RadarTile::Floor {
                        show_in_memory: super::sensor_memory_sample(position),
                    }
                };
                Some((position, tile))
            })
            .collect();
        Self { corners, tiles }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_sector::SectorDecor;
    use macroquad::prelude::Rect;
    use project_rl::world::{Map, VisibilityState};

    #[test]
    fn terrain_cache_is_reused_until_memory_changes_and_stays_out_of_saves() {
        let open = Map::from_ascii("#####\n#...#\n#####").unwrap();
        let player = GridPos::new(1, 1);
        let door = GridPos::new(2, 1);
        let mut closed = open.clone();
        closed
            .set_terrain(door, Terrain::Door(project_rl::world::DoorState::Closed))
            .unwrap();
        let mut sight = VisibilityState::default();
        sight.recompute(&closed, player, Default::default());
        let mut view = TerminalView::new(SectorDecor::default(), &closed, &sight);
        let bytes = bincode::serialize(&view).unwrap();
        let camera = GridCamera::new(Rect::new(0., 0., 320., 192.), 64., player);
        let window = TerrainWindow::new(&camera, true, false);
        let viewport = TerrainViewport::new(&view, window);
        assert!(
            viewport
                .known_at(&view, door)
                .unwrap()
                .terrain
                .blocks_movement()
        );
        assert!(viewport.known_at(&view, GridPos::new(3, 1)).is_none());
        let radar = RadarTerrain::new(&view);
        assert!(
            radar
                .tiles
                .iter()
                .any(|(at, tile)| *at == door && matches!(tile, RadarTile::Wall(_)))
        );
        view.presentation.get_mut().viewport = Some(viewport);
        view.presentation.get_mut().radar = Some(radar);

        view.observe(&closed, &sight);
        assert!(view.presentation.get_mut().viewport.is_some());
        assert!(view.presentation.get_mut().radar.is_some());
        assert_eq!(bincode::serialize(&view).unwrap(), bytes);
        assert_eq!(view.clone(), view);
        assert!(view.clone().presentation.into_inner().viewport.is_none());

        sight.recompute(&open, player, Default::default());
        view.observe(&open, &sight);
        assert!(view.presentation.get_mut().viewport.is_none());
        assert!(view.presentation.get_mut().radar.is_none());
        let viewport = TerrainViewport::new(&view, window);
        assert_eq!(
            viewport.known_at(&view, door).unwrap().terrain,
            Terrain::Floor
        );
        assert!(viewport.known_at(&view, GridPos::new(3, 1)).is_some());
    }
}
