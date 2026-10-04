//! Seeded placement of the authored interior and adjacent recycling start.
//! Coordinates remain derived from the generation revision and campaign seed.
use crate::test_sector::{Decor, SectorDecor, TestSector};
use project_rl::game::GameRng;
use project_rl::world::generation::{GeneratedMap, MapValidationRules};
use project_rl::world::{Direction, GridPos, Map, Terrain};

const WIDTH: i32 = 102;
const HEIGHT: i32 = 46;

/// Sample once per character creation. The chosen seed is then saved normally,
/// so replay and resume never depend on the clock.
pub fn fresh_campaign_seed() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};
    static SEQUENCE: AtomicU64 = AtomicU64::new(0);
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64;
    let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
    GameRng::from_seed(time ^ sequence.rotate_left(32) ^ u64::from(std::process::id())).next_u64()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HomePlacement {
    pub side: Direction,
    pub origin: GridPos,
    mirror_x: bool,
    mirror_y: bool,
    randomized: bool,
}

impl HomePlacement {
    pub fn identity() -> Self {
        Self {
            side: Direction::North,
            origin: GridPos::new(1, 1),
            mirror_x: false,
            mirror_y: false,
            randomized: false,
        }
    }
    pub fn for_seed(seed: u64) -> Self {
        let mut rng = GameRng::from_seed(seed ^ 0x484f_4d45_5f45_4447);
        let side = [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ][rng.usize_inclusive(0, 3).unwrap()];
        let horizontal = matches!(side, Direction::North | Direction::South);
        let limit = if horizontal {
            TestSector::EXPANDED_WIDTH as i32 - WIDTH - 1
        } else {
            TestSector::EXPANDED_HEIGHT as i32 - HEIGHT - 1
        };
        let candidates: Vec<_> = (1..=limit)
            .map(|offset| {
                let origin = match side {
                    Direction::North => GridPos::new(offset, 1),
                    Direction::South => {
                        GridPos::new(offset, TestSector::EXPANDED_HEIGHT as i32 - HEIGHT - 1)
                    }
                    Direction::West => GridPos::new(1, offset),
                    Direction::East => {
                        GridPos::new(TestSector::EXPANDED_WIDTH as i32 - WIDTH - 1, offset)
                    }
                };
                Self {
                    side,
                    origin,
                    mirror_x: false,
                    mirror_y: false,
                    randomized: true,
                }
            })
            .filter(|layout| {
                !contains(
                    expand(layout.bounds(), 2),
                    TestSector::EXPANDED_EXPEDITION_PASSAGE,
                )
            })
            .collect();
        let mut layout = candidates[rng.usize_inclusive(0, candidates.len() - 1).unwrap()];
        layout.mirror_x = match side {
            Direction::East => true,
            Direction::West => false,
            _ => rng.next_u64().is_multiple_of(2),
        };
        layout.mirror_y = match side {
            Direction::South => true,
            Direction::North => false,
            _ => rng.next_u64().is_multiple_of(2),
        };
        layout
    }
    pub fn position(self, p: GridPos) -> GridPos {
        if !contains([1, 1, WIDTH, HEIGHT], p) {
            return p;
        }
        let x = p.x - 1;
        let y = p.y - 1;
        GridPos::new(
            self.origin.x + if self.mirror_x { WIDTH - 1 - x } else { x },
            self.origin.y + if self.mirror_y { HEIGHT - 1 - y } else { y },
        )
    }
    pub fn rectangle(self, [x, y, w, h]: [i32; 4]) -> [i32; 4] {
        let a = self.position(GridPos::new(x, y));
        let b = self.position(GridPos::new(x + w - 1, y + h - 1));
        [a.x.min(b.x), a.y.min(b.y), w, h]
    }
    pub fn bounds(self) -> [i32; 4] {
        [self.origin.x, self.origin.y, WIDTH, HEIGHT]
    }
    pub fn town_contains(self, p: GridPos) -> bool {
        contains(self.rectangle([1, 1, 62, 44]), p)
    }
    pub fn authored_contains(self, p: GridPos) -> bool {
        self.town_contains(p)
            || contains(self.rectangle([70, 2, 28, 19]), p)
            || contains(self.rectangle([64, 22, 11, 10]), p)
    }
    pub fn regional_passages(self) -> [(Direction, GridPos); 4] {
        let mut passages = TestSector::EXPANDED_REGIONAL_PASSAGES;
        if !self.randomized {
            return passages;
        }
        let reserved = expand(self.bounds(), 2);
        for (side, p) in &mut passages {
            if !contains(reserved, *p) {
                continue;
            }
            let horizontal = matches!(side, Direction::North | Direction::South);
            let length = if horizontal {
                TestSector::EXPANDED_WIDTH
            } else {
                TestSector::EXPANDED_HEIGHT
            } as i32;
            *p = (2..length - 2)
                .map(|offset| match side {
                    Direction::North => GridPos::new(offset, 2),
                    Direction::South => {
                        GridPos::new(offset, TestSector::EXPANDED_HEIGHT as i32 - 3)
                    }
                    Direction::West => GridPos::new(2, offset),
                    Direction::East => GridPos::new(TestSector::EXPANDED_WIDTH as i32 - 3, offset),
                })
                .filter(|q| !contains(reserved, *q))
                .min_by_key(|q| (q.x.abs_diff(p.x) + q.y.abs_diff(p.y), *q))
                .unwrap();
        }
        passages
    }
    pub fn relocate(self, mut sector: TestSector) -> Result<TestSector, String> {
        let source = sector.level.map();
        let mut map = Map::filled(source.width(), source.height(), Terrain::Wall)
            .map_err(|e| e.to_string())?;
        let mut decor = SectorDecor::default();
        for y in 1..source.height() as i32 - 1 {
            for x in 1..source.width() as i32 - 1 {
                map.set_terrain(GridPos::new(x, y), Terrain::Floor)
                    .map_err(|e| e.to_string())?;
            }
        }
        for y in 1..=HEIGHT {
            for x in 1..=WIDTH {
                let from = GridPos::new(x, y);
                let to = self.position(from);
                let tile = source.tile(from).unwrap();
                let terrain = match tile.terrain {
                    Terrain::ControlPanel { door, activated } => Terrain::ControlPanel {
                        door: self.position(door),
                        activated,
                    },
                    terrain => terrain,
                };
                map.set_terrain(to, terrain).map_err(|e| e.to_string())?;
                map.set_protected(to, tile.protected)
                    .map_err(|e| e.to_string())?;
                if let Some(kind) = sector.decor.cells.get(&from) {
                    decor.cells.insert(
                        to,
                        if *kind == Decor::Passage {
                            Decor::Street
                        } else {
                            *kind
                        },
                    );
                }
                if let Some(biome) = sector.decor.biomes.get(&from) {
                    decor.biomes.insert(to, biome.clone());
                }
            }
        }
        for mut zone in sector.decor.zones {
            let [x, y, w, h] = zone.bounds;
            if contains([1, 1, WIDTH, HEIGHT], GridPos::new(x, y))
                && contains([1, 1, WIDTH, HEIGHT], GridPos::new(x + w - 1, y + h - 1))
            {
                zone.bounds = self.rectangle(zone.bounds);
                decor.zones.push(zone);
            }
        }
        let start = self.position(sector.level.player_start());
        let exit = sector.level.exit();
        let mut occupied: Vec<_> = sector
            .enemies
            .iter()
            .map(|p| self.position(*p))
            .chain([start, exit])
            .collect();
        for p in &mut sector.enemies {
            if contains([1, 1, WIDTH, HEIGHT], *p) {
                *p = self.position(*p);
            } else if contains(expand(self.bounds(), 3), *p) {
                let original = *p;
                *p = (2..map.height() as i32 - 2)
                    .flat_map(|y| (2..map.width() as i32 - 2).map(move |x| GridPos::new(x, y)))
                    .filter(|q| {
                        !contains(expand(self.bounds(), 3), *q)
                            && map.is_walkable(*q)
                            && !occupied.contains(q)
                    })
                    .min_by_key(|q| (q.x.abs_diff(original.x) + q.y.abs_diff(original.y), *q))
                    .ok_or("Aucune position extérieure pour un ennemi")?;
            }
            occupied.push(*p);
        }
        for p in &mut sector.loot {
            *p = self.position(*p);
        }
        let passages = self.regional_passages();
        let required: Vec<_> = sector
            .enemies
            .iter()
            .chain(&sector.loot)
            .copied()
            .chain(passages.iter().map(|(_, p)| *p))
            .chain([self.position(GridPos::new(68, 26))])
            .collect();
        sector.level =
            GeneratedMap::from_layout(map, start, exit, &required, MapValidationRules::default())
                .map_err(|e| format!("Placement de l'intérieur inaccessible : {e}"))?;
        sector.decor = decor;
        Ok(sector)
    }
}

pub fn contains(patch: [i32; 4], p: GridPos) -> bool {
    let [x, y, w, h] = patch;
    p.x >= x && p.y >= y && p.x < x + w && p.y < y + h
}
pub fn expand([x, y, w, h]: [i32; 4], margin: i32) -> [i32; 4] {
    [x - margin, y - margin, w + margin * 2, h + margin * 2]
}
