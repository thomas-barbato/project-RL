//! Versioned surface infill. Buildings are real movement/vision obstacles;
//! authored services, encounters, passages and earlier generations are retained.
use crate::test_regional::GeneratedRegionalDestination;
use crate::test_sector::{Decor, SectorDecor, TestSector, Zone};
use project_rl::game::GameRng;
use project_rl::world::generation::{GeneratedMap, MapValidationRules, validate_interactive_map};
use project_rl::world::{DoorState, GridPos, Map, Terrain};
use std::collections::BTreeSet;

const LAYOUT_SALT: u64 = 0x5355_5246_494e_4649;
const ROOM_NAMES: [&str; 4] = [
    "Atelier abandonné",
    "Entrepôt de surface",
    "Local technique",
    "Habitation abandonnée",
];

pub fn densify_hub(mut sector: TestSector, seed: u64) -> Result<TestSector, String> {
    let mut map = sector.level.map().clone();
    let mut rng = GameRng::from_seed(seed ^ LAYOUT_SALT);
    let required: Vec<_> = sector
        .enemies
        .iter()
        .chain(&sector.loot)
        .copied()
        .chain(
            TestSector::EXPANDED_REGIONAL_PASSAGES
                .iter()
                .map(|(_, p)| *p),
        )
        .chain([
            sector.level.player_start(),
            sector.level.exit(),
            GridPos::new(68, 26),
        ])
        .collect();
    // Streets form loops around blocks. Infill cannot overwrite the original
    // service road, the town wall, the prologue, or the roadside contact.
    for x in (3..190).step_by(22) {
        for y in 1..127 {
            for dx in 0..3 {
                let p = GridPos::new(x + dx, y);
                if hub_editable(p, &map) && is_natural(sector.decor.cells.get(&p).copied()) {
                    set(&mut map, &mut sector.decor, p, Decor::Lane)?;
                }
            }
        }
    }
    for y in (3..125).step_by(20) {
        for x in 1..191 {
            for dy in 0..3 {
                let p = GridPos::new(x, y + dy);
                if hub_editable(p, &map) && is_natural(sector.decor.cells.get(&p).copied()) {
                    set(&mut map, &mut sector.decor, p, Decor::Lane)?;
                }
            }
        }
    }
    for y in (3..108).step_by(20) {
        for x in (3..170).step_by(22) {
            let role = rng.usize_inclusive(0, 3).unwrap();
            let layout = rng.usize_inclusive(0, 5).unwrap();
            // Large workshops alternate with pairs or clusters of small rooms.
            // Water and existing content, rather than empty random parcels,
            // determine where construction must give way.
            let offset = rng.usize_inclusive(0, 1).unwrap() as i32;
            let mut bounds = if layout < 2 {
                vec![[x + 5, y + 5 + offset, 16, 13]]
            } else if layout < 4 {
                vec![[x + 5, y + 5, 7, 13], [x + 14, y + 5 + offset, 7, 12]]
            } else if layout == 4 {
                vec![[x + 5, y + 5, 16, 6], [x + 5 + offset, y + 13, 15, 6]]
            } else {
                Vec::new()
            };
            // A large footprint may be excluded by a road or a single actor.
            // Smaller rooms can still use the remainder of that same block.
            bounds.extend([
                [x + 5, y + 5, 7, 7],
                [x + 14, y + 5, 7, 7],
                [x + 5, y + 13, 7, 6],
                [x + 14, y + 13, 7, 6],
            ]);
            for (index, bounds) in bounds.into_iter().enumerate() {
                let pad = padded(bounds);
                let positions = cells(pad);
                if positions.iter().any(|p| {
                    !hub_editable(*p, &map)
                        || required.iter().any(|r| distance(*r, *p) <= 2)
                        || !is_natural(sector.decor.cells.get(p).copied())
                }) {
                    continue;
                }
                let deep_water = positions
                    .iter()
                    .filter(|p| sector.decor.cells.get(p) == Some(&Decor::DeepWater))
                    .count();
                if deep_water * 8 > positions.len() {
                    continue;
                }
                clear_yard(&mut map, &mut sector.decor, pad)?;
                building(
                    &mut map,
                    &mut sector.decor,
                    bounds,
                    (role + index) % 4,
                    layout,
                )?;
            }
        }
    }
    sector.level = GeneratedMap::from_layout(
        map,
        sector.level.player_start(),
        sector.level.exit(),
        &required,
        MapValidationRules::default(),
    )
    .map_err(|e| format!("Surface bâtie inaccessible : {e}"))?;
    Ok(sector)
}

/// Supplement ordinary surface regions after all their interactive content has
/// been placed. Each proposed block must preserve every navigable cell and all
/// existing actor/loot/installation positions. Authored regional cities already
/// have their own layout and are excluded by the caller.
pub fn densify_regional(
    destination: &mut GeneratedRegionalDestination,
    seed: u64,
) -> Result<(), String> {
    if destination.blueprint.info.depth != 0 {
        return Ok(());
    }
    let mut reserved: BTreeSet<_> = destination
        .blueprint
        .actors
        .iter()
        .flat_map(|a| std::iter::once(a.position()).chain(a.ai_home()))
        .chain(destination.blueprint.loot.iter().map(|l| l.position()))
        .chain(destination.passages.iter().map(|(_, p)| *p))
        .chain(destination.vertical_passages.iter().map(|(_, p)| *p))
        .chain(
            destination
                .blueprint
                .threat_sources
                .iter()
                .map(|s| s.position),
        )
        .collect();
    if let Some(facility) = &destination.facility {
        reserved.extend(facility.installations.iter().map(|i| i.position));
    }
    let mut rng = GameRng::from_seed(seed ^ LAYOUT_SALT);
    let mut map = destination.blueprint.map.clone();
    let mut decor = destination.decor.clone();
    let required: Vec<_> = reserved
        .iter()
        .copied()
        .filter(|p| map.is_walkable(*p))
        .collect();
    for y in (5..map.height() as i32 - 12).step_by(12) {
        for x in (5..map.width() as i32 - 12).step_by(14) {
            let role = rng.usize_inclusive(0, 3).unwrap();
            let variant = rng.usize_inclusive(0, 4).unwrap();
            let bounds = [
                x + rng.usize_inclusive(0, 2).unwrap() as i32,
                y + rng.usize_inclusive(0, 1).unwrap() as i32,
                if variant == 0 { 7 } else { 10 },
                if variant == 0 { 7 } else { 9 },
            ];
            let pad = padded(bounds);
            if variant == 4
                || cells(pad).iter().any(|p| {
                    map.is_protected(*p)
                        || reserved.iter().any(|r| distance(*r, *p) <= 2)
                        || !matches!(
                            decor.cells.get(p),
                            Some(
                                Decor::Gravel
                                    | Decor::Grass
                                    | Decor::Scrub
                                    | Decor::Mud
                                    | Decor::Tree
                                    | Decor::Boulder
                                    | Decor::ShallowWater
                                    | Decor::DeepWater
                            )
                        )
                        || !matches!(
                            map.tile(*p).map(|t| t.terrain),
                            Some(
                                Terrain::Floor
                                    | Terrain::Wall
                                    | Terrain::ShallowWater
                                    | Terrain::DeepWater
                            )
                        )
                })
            {
                continue;
            }
            let footprint = cells(pad);
            if footprint
                .iter()
                .filter(|p| decor.cells.get(p) == Some(&Decor::DeepWater))
                .count()
                * 4
                > footprint.len()
            {
                continue;
            }
            let mut candidate = map.clone();
            let mut candidate_decor = decor.clone();
            clear_yard(&mut candidate, &mut candidate_decor, pad)?;
            building(&mut candidate, &mut candidate_decor, bounds, role, variant)?;
            if validate_interactive_map(
                &candidate,
                destination.blueprint.entrance,
                destination.passages[0].1,
                &required,
                MapValidationRules::default(),
            )
            .is_ok()
            {
                map = candidate;
                decor = candidate_decor;
            }
        }
    }
    destination.blueprint.map = map;
    destination.decor = decor;
    Ok(())
}

fn hub_editable(p: GridPos, map: &Map) -> bool {
    p.x > 0
        && p.y > 0
        && p.x < map.width() as i32 - 1
        && p.y < map.height() as i32 - 1
        && !(p.x <= 62 && p.y <= 44)
        && !inside(p, [70, 2, 28, 19])
        && !inside(p, [64, 22, 11, 10])
        && !map.is_protected(p)
}

fn is_natural(kind: Option<Decor>) -> bool {
    matches!(
        kind,
        Some(
            Decor::Gravel
                | Decor::Grass
                | Decor::Scrub
                | Decor::Mud
                | Decor::ShallowWater
                | Decor::DeepWater
                | Decor::Tree
                | Decor::Boulder
        )
    )
}

fn building(
    map: &mut Map,
    decor: &mut SectorDecor,
    b: [i32; 4],
    role: usize,
    variant: usize,
) -> Result<(), String> {
    let [x, y, w, h] = b;
    for p in cells(b) {
        let edge = p.x == x || p.y == y || p.x == x + w - 1 || p.y == y + h - 1;
        set(
            map,
            decor,
            p,
            if edge {
                Decor::Wall
            } else if role == 2 {
                Decor::Grate
            } else {
                Decor::Deck
            },
        )?;
    }
    // Two opposing entrances offer a way through each building. Half are
    // ordinary doors; the others are broken/open entrances, never new locks.
    let vertical = variant.is_multiple_of(2);
    let entrances = if vertical {
        [
            GridPos::new(x + w / 2, y),
            GridPos::new(x + w / 2, y + h - 1),
        ]
    } else {
        [
            GridPos::new(x, y + h / 2),
            GridPos::new(x + w - 1, y + h / 2),
        ]
    };
    for (i, p) in entrances.into_iter().enumerate() {
        map.set_terrain(
            p,
            if i == 0 && variant != 4 {
                Terrain::Door(DoorState::Closed)
            } else {
                Terrain::Floor
            },
        )
        .map_err(|e| e.to_string())?;
        decor.cells.insert(p, Decor::Threshold);
    }
    // Furniture occupies isolated islands with walkable aisles. It genuinely
    // interrupts sight/fire and movement, using existing semantic props.
    let prop = [Decor::Console, Decor::Crate, Decor::Server, Decor::Pillar][role];
    for py in (y + 2..y + h - 2).step_by(3) {
        for px in (x + 2..x + w - 2).step_by(3) {
            if (vertical && (px - (x + w / 2)).abs() <= 1)
                || (!vertical && (py - (y + h / 2)).abs() <= 1)
            {
                continue;
            }
            set(map, decor, GridPos::new(px, py), prop)?;
        }
    }
    if w >= 12 && h >= 9 {
        // A partition adds an alcove and a second room without closing the
        // route between entrances or creating inaccessible floor islands.
        let partition_x = x + 4;
        for py in y + 1..y + h - 1 {
            let p = GridPos::new(partition_x, py);
            set(
                map,
                decor,
                p,
                if py == y + h / 2 {
                    Decor::Threshold
                } else {
                    Decor::Wall
                },
            )?;
        }
    }
    decor.zones.push(Zone {
        name: ROOM_NAMES[role].to_owned(),
        bounds: b,
    });
    Ok(())
}

fn clear_yard(map: &mut Map, decor: &mut SectorDecor, bounds: [i32; 4]) -> Result<(), String> {
    for p in cells(bounds) {
        set(map, decor, p, Decor::Lane)?;
    }
    Ok(())
}

fn set(map: &mut Map, decor: &mut SectorDecor, p: GridPos, kind: Decor) -> Result<(), String> {
    map.set_terrain(
        p,
        if kind.blocks() {
            Terrain::Wall
        } else {
            Terrain::Floor
        },
    )
    .map_err(|e| e.to_string())?;
    decor.cells.insert(p, kind);
    decor.biomes.remove(&p);
    Ok(())
}

fn inside(p: GridPos, [x, y, w, h]: [i32; 4]) -> bool {
    p.x >= x && p.y >= y && p.x < x + w && p.y < y + h
}
fn padded([x, y, w, h]: [i32; 4]) -> [i32; 4] {
    [x - 1, y - 1, w + 2, h + 2]
}
fn cells([x, y, w, h]: [i32; 4]) -> Vec<GridPos> {
    (y..y + h)
        .flat_map(|py| (x..x + w).map(move |px| GridPos::new(px, py)))
        .collect()
}
fn distance(a: GridPos, b: GridPos) -> u32 {
    a.x.abs_diff(b.x).max(a.y.abs_diff(b.y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dense_surface_has_connected_furnished_buildings_and_preserves_authored_content() {
        for seed in 1..=32 {
            let original = TestSector::build(seed)
                .unwrap()
                .with_surface_contacts()
                .unwrap();
            let before = original.level.map().clone();
            let actors = original.enemies.clone();
            let loot = original.loot.clone();
            let dense = densify_hub(original, seed).unwrap();
            assert_eq!(dense.enemies, actors);
            assert_eq!(dense.loot, loot);
            let buildings = dense
                .decor
                .zones
                .iter()
                .filter(|z| ROOM_NAMES.contains(&z.name.as_str()))
                .count();
            assert!(buildings >= 20, "seed {seed}: only {buildings} buildings");
            for y in 0..before.height() as i32 {
                for x in 0..before.width() as i32 {
                    let p = GridPos::new(x, y);
                    if !hub_editable(p, &before)
                        || dense.enemies.contains(&p)
                        || dense.loot.contains(&p)
                        || TestSector::EXPANDED_REGIONAL_PASSAGES
                            .iter()
                            .any(|(_, r)| *r == p)
                        || p == TestSector::EXPANDED_EXPEDITION_PASSAGE
                    {
                        assert_eq!(
                            before.tile(p),
                            dense.level.map().tile(p),
                            "seed {seed} at {p:?}"
                        );
                    }
                }
            }
            assert!(dense.decor.cells.values().any(|d| *d == Decor::DeepWater));
            assert!(dense.decor.cells.values().any(|d| *d == Decor::Tree));
        }
    }

    #[test]
    fn dense_surface_is_repeatable_and_varies_between_seeds() {
        let build = |seed| {
            densify_hub(
                TestSector::build(seed)
                    .unwrap()
                    .with_surface_contacts()
                    .unwrap(),
                seed,
            )
            .unwrap()
        };
        let a = build(17);
        let b = build(17);
        assert_eq!(a.level, b.level);
        assert_eq!(a.decor, b.decor);
        assert_ne!(a.decor.zones, build(18).decor.zones);
    }
}
