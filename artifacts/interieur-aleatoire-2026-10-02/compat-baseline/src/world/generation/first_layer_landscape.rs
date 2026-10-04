use super::GeneratedRegionalMap;
use crate::content::{FirstLayerPlace, RegionTerrain};
use crate::game::GameRng;
use crate::world::{DoorState, GridPos, Terrain};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FirstLayerLandscape {
    pub role: FirstLayerPlace,
    pub annex: Option<[i32; 4]>,
    pub cache: Option<GridPos>,
    pub shortcut: Option<(GridPos, GridPos)>,
}

impl FirstLayerLandscape {
    /// Soft tactical anchors, not mandatory fights or additional spawns.
    pub fn encounter_anchors(&self, width: usize, height: usize, seed: u64) -> Vec<GridPos> {
        use FirstLayerPlace::*;
        let (w, h) = (width as i32, height as i32);
        let fractions = match self.role {
            Workshops => [(3, 3), (8, 3), (3, 8), (8, 8), (6, 2), (6, 9)],
            Conduits => [(3, 2), (7, 4), (3, 7), (8, 9), (8, 2), (6, 8)],
            Pumps => [(8, 3), (3, 8), (7, 8), (4, 3), (9, 7), (2, 4)],
            Gallery => [(3, 3), (8, 8), (7, 3), (3, 8), (9, 5), (5, 9)],
            Descent => [(3, 4), (8, 7), (7, 3), (4, 8), (2, 7), (9, 3)],
            City => return vec![],
        };
        let mut rng = GameRng::from_seed(seed ^ 0x5441_4354_4943_5331);
        let mut anchors: Vec<_> = fractions
            .into_iter()
            .map(|(x, y)| {
                GridPos::new(
                    w * x / 11 + (rng.next_u64() % 5) as i32 - 2,
                    h * y / 11 + (rng.next_u64() % 5) as i32 - 2,
                )
            })
            .collect();
        // Avoid assigning the first group systematically to the same corner.
        anchors.rotate_left((rng.next_u64() % 6) as usize);
        anchors
    }

    pub fn reserved(&self) -> Vec<GridPos> {
        self.cache
            .into_iter()
            .chain(
                self.shortcut
                    .into_iter()
                    .flat_map(|(door, control)| [door, control]),
            )
            .collect()
    }
}

/// Functional silhouettes with seeded dimensions/interruptions. Applied before
/// occupants and caches, without increasing population or loot budgets.
pub fn shape_first_layer_landscape(
    generated: &mut GeneratedRegionalMap,
    role: FirstLayerPlace,
    seed: u64,
) -> Result<FirstLayerLandscape, String> {
    use FirstLayerPlace::*;
    use RegionTerrain::*;
    let w = generated.map().width() as i32;
    let h = generated.map().height() as i32;
    if w < 64 || h < 48 || role == City {
        return Err("Unsupported first-layer landscape dimensions or role".into());
    }
    let mut rng = GameRng::from_seed(seed ^ 0x4c41_4e44_5343_5031);
    let shift = (rng.next_u64() % 5) as i32 - 2;
    let gap = (rng.next_u64() % 5) as i32;
    let ground = if matches!(role, Conduits | Gallery) {
        Gravel
    } else {
        RuinFloor
    };
    let mut cells = BTreeMap::new();
    // Account for the entire interior rather than trap old random floor
    // pockets behind newly painted walls. Border and passage anchors stay put.
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            let terrain = match role {
                Workshops => {
                    let column = (x + shift).rem_euclid(19);
                    let row = y.rem_euclid(18);
                    if x > 5
                        && x < w - 6
                        && y > 5
                        && y < h - 6
                        && column < 2
                        && !(7..=11).contains(&row)
                    {
                        RuinWall
                    } else if column == 7 && (4..=6).contains(&row) {
                        Boulder
                    } else {
                        ground
                    }
                }
                Conduits => {
                    let bend = ((y / 11) % 2) * 4 + shift;
                    let channel = (x - bend).rem_euclid(17);
                    if x > 5
                        && x < w - 6
                        && y > 5
                        && y < h - 6
                        && channel == 0
                        && (y + gap) % 13 > 3
                    {
                        RuinWall
                    } else if (1..=3).contains(&channel) {
                        ShallowWater
                    } else if channel == 4 {
                        Mud
                    } else {
                        ground
                    }
                }
                Pumps => {
                    let dx = x - (w * 2 / 3 + shift);
                    let dy = y - h / 4;
                    let radius = (w.min(h) / 6).max(6);
                    let r2 = dx * dx + dy * dy;
                    if r2 < (radius - 2) * (radius - 2) {
                        DeepWater
                    } else if r2 < (radius + 2) * (radius + 2) {
                        ShallowWater
                    } else if y > h / 2 + 6 && y < h - 6 && x > w / 5 && x < w * 4 / 5 {
                        if (x + shift).rem_euclid(18) < 3 {
                            Boulder
                        } else {
                            ShallowWater
                        }
                    } else if (x - w / 3).abs() < 2 && y > 6 && y < h - 6 {
                        Mud
                    } else {
                        ground
                    }
                }
                Gallery => {
                    let pillar_x = (x + shift).rem_euclid(15);
                    let row = y.rem_euclid(14);
                    if x > 5 && x < w - 6 && y > 5 && y < h - 6 && pillar_x < 3 && row < 5 {
                        RuinWall
                    } else if row == 7 && pillar_x > 10 && y > h / 2 {
                        Mud
                    } else {
                        ground
                    }
                }
                Descent => {
                    let dx = (x - w / 2).abs();
                    let dy = (y - h / 2).abs();
                    let ring = dx.max(dy * 2);
                    if (ring == 12 + shift || ring == 24 + shift) && dx > 4 && dy > 3 {
                        RuinWall
                    } else if ring < 10 {
                        RuinFloor
                    } else if ring % 9 == 0 {
                        Gravel
                    } else {
                        ground
                    }
                }
                City => unreachable!(),
            };
            cells.insert(GridPos::new(x, y), terrain);
        }
    }
    // Ordinary floor, not protected/invulnerable cells.
    for y in 1..h - 1 {
        for x in 1..w - 1 {
            if (x - w / 2).abs() <= 2
                || (y - h / 2).abs() <= 2
                || x <= 3
                || x >= w - 4
                || y <= 3
                || y >= h - 4
            {
                cells.insert(GridPos::new(x, y), ground);
            }
        }
    }
    let mut result = FirstLayerLandscape {
        role,
        annex: None,
        cache: None,
        shortcut: None,
    };
    let mut interactions = BTreeMap::new();
    if matches!(role, Workshops | Pumps) {
        let ax = w / 10 + shift;
        let ay = h / 8;
        let aw = w / 4;
        let ah = h / 4;
        for y in ay - 2..=ay + ah + 2 {
            for x in ax - 2..=ax + aw + 2 {
                let wall = (x == ax || x == ax + aw) && (ay..=ay + ah).contains(&y)
                    || (y == ay || y == ay + ah) && (ax..=ax + aw).contains(&x);
                cells.insert(GridPos::new(x, y), if wall { RuinWall } else { RuinFloor });
            }
        }
        let entrance = GridPos::new(ax + aw, ay + ah - 3);
        cells.insert(entrance, RuinFloor);
        cells.insert(GridPos::new(entrance.x, entrance.y - 1), RuinFloor);
        result.annex = Some([ax, ay, aw + 1, ah + 1]);
        result.cache = Some(GridPos::new(ax + 4, ay + 4));
        if role == Workshops {
            let door = GridPos::new(ax, ay + 4);
            let control = GridPos::new(ax + 2, ay + 4);
            interactions.insert(door, Terrain::Door(DoorState::Locked));
            interactions.insert(
                control,
                Terrain::ControlPanel {
                    door,
                    activated: false,
                },
            );
            result.shortcut = Some((door, control));
        }
    }
    let required = result.cache.into_iter().collect::<Vec<_>>();
    generated
        .apply_site_overlay(&cells, &interactions, &required)
        .map_err(|e| e.to_string())?;
    Ok(result)
}
