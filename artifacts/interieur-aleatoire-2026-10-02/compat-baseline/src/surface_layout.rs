//! Versioned urban blocks, with avenues, narrow streets and interior courtyards.
use crate::test_regional::GeneratedRegionalDestination;
use crate::test_sector::{Decor, SectorDecor, TestSector, Zone};
use project_rl::game::GameRng;
use project_rl::world::generation::{GeneratedMap, MapValidationRules, validate_interactive_map};
use project_rl::world::{DoorState, GridPos, Map, Terrain};
use std::collections::{BTreeSet, VecDeque};

const SALT: u64 = 0x5355_5246_494e_4649;
pub const BUILDING_NAMES: [&str; 4] = [
    "Atelier abandonné",
    "Entrepôt de surface",
    "Local technique",
    "Bloc urbain",
];

pub fn densify_hub(sector: TestSector, seed: u64) -> Result<TestSector, String> {
    densify_hub_layout(sector, seed, crate::hub_layout::HomePlacement::identity())
}

pub fn densify_hub_at_random_edge(sector: TestSector, seed: u64) -> Result<TestSector, String> {
    let home = crate::hub_layout::HomePlacement::for_seed(seed);
    densify_hub_layout(home.relocate(sector)?, seed, home)
}

fn densify_hub_layout(
    mut sector: TestSector,
    seed: u64,
    home: crate::hub_layout::HomePlacement,
) -> Result<TestSector, String> {
    let mut map = sector.level.map().clone();
    let passages = home.regional_passages();
    let required: Vec<_> = sector
        .enemies
        .iter()
        .chain(&sector.loot)
        .copied()
        .chain(passages.iter().map(|(_, p)| *p))
        .chain([
            sector.level.player_start(),
            sector.level.exit(),
            home.position(GridPos::new(68, 26)),
        ])
        .collect();
    let anchors: Vec<_> = required
        .iter()
        .copied()
        .chain([
            home.position(GridPos::new(63, 21)),
            home.position(GridPos::new(69, 11)),
            home.position(GridPos::new(88, 21)),
            home.position(GridPos::new(69, 32)),
            GridPos::new(175, 108),
        ])
        .collect();
    let editable = mask(&map, |p| {
        hub_editable(p, &map, home) && !required.contains(&p)
    });
    urbanize(
        &mut map,
        &mut sector.decor,
        &editable,
        &anchors,
        sector.level.player_start(),
        seed,
    )?;
    for (_, p) in passages {
        sector.decor.cells.insert(p, Decor::Passage);
    }
    sector.level = GeneratedMap::from_layout(
        map,
        sector.level.player_start(),
        sector.level.exit(),
        &required,
        MapValidationRules::default(),
    )
    .map_err(|e| format!("Surface urbaine inaccessible : {e}"))?;
    Ok(sector)
}

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
    // Keep a place to stand when using preserved wall-mounted installations.
    let interactions: Vec<_> = reserved
        .iter()
        .copied()
        .chain(positions(&destination.blueprint.map).filter(|p| {
            matches!(
                destination.blueprint.map.tile(*p).map(|t| t.terrain),
                Some(Terrain::ControlPanel { .. })
            )
        }))
        .collect();
    for p in interactions {
        if !destination.blueprint.map.is_walkable(p) {
            reserved.extend(
                p.cardinal_neighbors()
                    .into_iter()
                    .filter(|q| destination.blueprint.map.is_walkable(*q)),
            );
        }
    }
    let mut map = destination.blueprint.map.clone();
    let mut decor = destination.decor.clone();
    let editable = mask(&map, |p| {
        !map.is_protected(p)
            && !reserved.contains(&p)
            && matches!(
                decor.cells.get(&p),
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
    });
    let anchors: Vec<_> = reserved
        .iter()
        .copied()
        .filter(|p| map.is_walkable(*p))
        .collect();
    urbanize(
        &mut map,
        &mut decor,
        &editable,
        &anchors,
        destination.blueprint.entrance,
        seed,
    )?;
    validate_interactive_map(
        &map,
        destination.blueprint.entrance,
        destination.passages[0].1,
        &anchors,
        MapValidationRules::default(),
    )
    .map_err(|e| format!("Région urbaine inaccessible : {e}"))?;
    destination.blueprint.map = map;
    destination.decor = decor;
    Ok(())
}

fn urbanize(
    map: &mut Map,
    decor: &mut SectorDecor,
    editable: &[bool],
    anchors: &[GridPos],
    start: GridPos,
    seed: u64,
) -> Result<(), String> {
    let mut rng = GameRng::from_seed(seed ^ SALT);
    // Outside circulation remains distinct from the interiors of buildings.
    for p in positions(map).collect::<Vec<_>>() {
        if allowed(map, editable, p) {
            put(map, decor, p, Decor::Street)?;
        }
    }
    let bounds = [3, 3, map.width() as i32 - 6, map.height() as i32 - 6];
    let mut plots = Vec::new();
    subdivide(bounds, 0, &mut rng, &mut plots);
    let mut buildings = Vec::new();
    for plot in plots {
        // An internal alley divides some large blocks into adjoining buildings.
        let [x, y, w, h] = plot;
        if w >= 23 && h >= 12 && !rng.next_u64().is_multiple_of(3) {
            let cut = random(&mut rng, 9, w - 10);
            buildings.push([x, y, cut, h]);
            buildings.push([x + cut + 1, y, w - cut - 1, h]);
        } else if h >= 23 && w >= 12 && !rng.next_u64().is_multiple_of(3) {
            let cut = random(&mut rng, 9, h - 10);
            buildings.push([x, y, w, cut]);
            buildings.push([x, y + cut + 1, w, h - cut - 1]);
        } else {
            buildings.push(plot);
        }
    }
    let mut available = mask(map, |p| {
        allowed(map, editable, p) && !anchors.iter().any(|a| distance(*a, p) <= 1)
    });
    for plot in buildings {
        // A preserved encounter can interrupt a plot. Fill its remaining sides
        // with smaller buildings, leaving an alley between their facades.
        for _ in 0..3 {
            let Some(bounds) = fit_building(map, &available, plot) else {
                break;
            };
            build(map, decor, bounds, &mut rng)?;
            let [x, y, w, h] = bounds;
            for p in cells([x - 1, y - 1, w + 2, h + 2]) {
                if interior(map, p) {
                    available[index(map, p)] = false;
                }
            }
        }
    }
    // Protect the approach to every existing encounter and passage.
    for &p in anchors {
        for q in cells([p.x - 1, p.y - 1, 3, 3]) {
            if allowed(map, editable, q)
                && map
                    .tile(q)
                    .is_some_and(|t| !matches!(t.terrain, Terrain::Door(_)))
            {
                put(map, decor, q, Decor::Street)?;
            }
        }
    }
    join_components(map, decor, editable, start)?;
    decor.fallback_name = Some("Quartiers de surface".to_owned());
    Ok(())
}

// Large divisions make avenues; short subdivisions make streets and alleys.
// Their unequal lengths and T junctions avoid a uniform grid of square blocks.
fn subdivide([x, y, w, h]: [i32; 4], depth: usize, rng: &mut GameRng, plots: &mut Vec<[i32; 4]>) {
    if (w <= 31 && h <= 27) || (w < 23 && h < 23) {
        plots.push([x, y, w, h]);
        return;
    }
    let vertical = if w < 23 {
        false
    } else if h < 23 {
        true
    } else if w > h * 3 / 2 {
        true
    } else if h > w * 3 / 2 {
        false
    } else {
        rng.next_u64().is_multiple_of(2)
    };
    let street = if depth <= 1 { 5 } else { random(rng, 2, 3) };
    let length = if vertical { w } else { h };
    let cut = random(
        rng,
        9.max(length * 2 / 5),
        (length * 3 / 5).min(length - street - 9),
    );
    if vertical {
        subdivide([x, y, cut, h], depth + 1, rng, plots);
        subdivide(
            [x + cut + street, y, w - cut - street, h],
            depth + 1,
            rng,
            plots,
        );
    } else {
        subdivide([x, y, w, cut], depth + 1, rng, plots);
        subdivide(
            [x, y + cut + street, w, h - cut - street],
            depth + 1,
            rng,
            plots,
        );
    }
}

fn fit_building(map: &Map, editable: &[bool], [x, y, w, h]: [i32; 4]) -> Option<[i32; 4]> {
    // Find the largest rectangle within a plot without drawing over any authored
    // terrain or entity. The small exhaustive search runs only during generation.
    let mut best = None;
    let mut area = 0;
    for py in y..y + h - 6 {
        for px in x..x + w - 6 {
            let mut width = x + w - px;
            for bottom in py..y + h {
                let run = (px..px + width)
                    .take_while(|qx| {
                        let p = GridPos::new(*qx, bottom);
                        allowed(map, editable, p)
                    })
                    .count() as i32;
                width = width.min(run);
                if width < 7 {
                    break;
                }
                let height = bottom - py + 1;
                if height >= 7 && width * height > area {
                    best = Some([px, py, width, height]);
                    area = width * height;
                }
            }
        }
    }
    best
}

fn build(
    map: &mut Map,
    decor: &mut SectorDecor,
    [x, y, w, h]: [i32; 4],
    rng: &mut GameRng,
) -> Result<(), String> {
    let role = random(rng, 0, 3) as usize;
    let floor = if role == 2 { Decor::Grate } else { Decor::Deck };
    for p in cells([x, y, w, h]) {
        let edge = p.x == x || p.x == x + w - 1 || p.y == y || p.y == y + h - 1;
        put(map, decor, p, if edge { Decor::Wall } else { floor })?;
    }
    let cx = x + w / 2;
    let cy = y + h / 2;
    if w >= 15 && h >= 13 && rng.next_u64().is_multiple_of(3) {
        // A real exterior courtyard reached by an alley through the block.
        let court = [cx - 2, cy - 2, 5, 5];
        for p in cells(court) {
            let edge = p.x == cx - 2 || p.x == cx + 2 || p.y == cy - 2 || p.y == cy + 2;
            put(
                map,
                decor,
                p,
                if edge { Decor::Wall } else { Decor::Street },
            )?;
        }
        for p in cells([cx, cy + 1, 2, y + h - cy - 1]) {
            put(map, decor, p, Decor::Street)?;
        }
        for py in cy + 3..y + h {
            put(map, decor, GridPos::new(cx - 1, py), Decor::Wall)?;
            put(map, decor, GridPos::new(cx + 2, py), Decor::Wall)?;
        }
        ordinary_door(map, decor, GridPos::new(cx, cy - 2))?;
        decor.zones.push(Zone {
            name: "Cour intérieure".to_owned(),
            bounds: court,
        });
    } else if role != 1 && (w >= 13 || h >= 13) {
        // Larger urban buildings contain several rooms behind their facade.
        if w > h {
            for py in y + 1..y + h - 1 {
                put(map, decor, GridPos::new(cx, py), Decor::Wall)?;
            }
            ordinary_door(map, decor, GridPos::new(cx, cy))?;
        } else {
            for px in x + 1..x + w - 1 {
                put(map, decor, GridPos::new(px, cy), Decor::Wall)?;
            }
            ordinary_door(map, decor, GridPos::new(cx, cy))?;
        }
    }
    let candidates = [
        (
            GridPos::new(cx, y),
            GridPos::new(cx, y - 1),
            GridPos::new(cx, y + 1),
        ),
        (
            GridPos::new(x + w - 1, cy),
            GridPos::new(x + w, cy),
            GridPos::new(x + w - 2, cy),
        ),
        (
            GridPos::new(cx, y + h - 1),
            GridPos::new(cx, y + h),
            GridPos::new(cx, y + h - 2),
        ),
        (
            GridPos::new(x, cy),
            GridPos::new(x - 1, cy),
            GridPos::new(x + 1, cy),
        ),
    ];
    // Furnish away from entrances and the central circulation.
    for py in (y + 2..y + h - 2).step_by(4) {
        for px in (x + 2..x + w - 2).step_by(4) {
            let p = GridPos::new(px, py);
            if (px - cx).abs() <= 1 || (py - cy).abs() <= 1 || decor.cells.get(&p) != Some(&floor) {
                continue;
            }
            put(
                map,
                decor,
                p,
                [Decor::Console, Decor::Crate, Decor::Server, Decor::Pillar][role],
            )?;
        }
    }
    let first = random(rng, 0, 3) as usize;
    let mut opened = 0;
    for i in 0..4 {
        let (p, outside, inside) = candidates[(first + i) % 4];
        if map.is_walkable(outside)
            && map.is_walkable(inside)
            && map.tile(p).is_some_and(|t| t.terrain == Terrain::Wall)
        {
            ordinary_door(map, decor, p)?;
            opened += 1;
            if opened == 2 {
                break;
            }
        }
    }
    // A central partition can cover all four midpoints. Offset the entrance
    // along a facade instead of relying on a later connectivity repair.
    if opened == 0 {
        for px in x + 1..x + w - 1 {
            let p = GridPos::new(px, y);
            if map.is_walkable(GridPos::new(px, y - 1)) && map.is_walkable(GridPos::new(px, y + 1))
            {
                ordinary_door(map, decor, p)?;
                break;
            }
        }
    }
    decor.zones.push(Zone {
        name: BUILDING_NAMES[role].to_owned(),
        bounds: [x, y, w, h],
    });
    Ok(())
}

fn ordinary_door(map: &mut Map, decor: &mut SectorDecor, p: GridPos) -> Result<(), String> {
    map.set_terrain(p, Terrain::Door(DoorState::Closed))
        .map_err(|e| e.to_string())?;
    decor.cells.insert(p, Decor::DoorClosed);
    Ok(())
}

fn join_components(
    map: &mut Map,
    decor: &mut SectorDecor,
    editable: &[bool],
    start: GridPos,
) -> Result<(), String> {
    loop {
        let mut reachable = flood(map, start);
        let mut changed = false;
        for origin in positions(map).collect::<Vec<_>>() {
            if !ordinary_passable(map, origin) || reachable[index(map, origin)] {
                continue;
            }
            let mut parent = vec![None; map.width() * map.height()];
            let mut seen = vec![false; parent.len()];
            let mut queue = VecDeque::from([origin]);
            seen[index(map, origin)] = true;
            let mut target = None;
            while let Some(p) = queue.pop_front() {
                if reachable[index(map, p)] {
                    target = Some(p);
                    break;
                }
                for q in p.cardinal_neighbors() {
                    if !interior(map, q)
                        || seen[index(map, q)]
                        || (!allowed(map, editable, q)
                            && !ordinary_passable(map, q)
                            && !reachable[index(map, q)])
                    {
                        continue;
                    }
                    seen[index(map, q)] = true;
                    parent[index(map, q)] = Some(p);
                    queue.push_back(q);
                }
            }
            // A preserved locked site may need its exterior console connected
            // first. Revisit it after the surrounding components have joined.
            let Some(mut p) = target else {
                continue;
            };
            while p != origin {
                if allowed(map, editable, p) && !ordinary_passable(map, p) {
                    put(map, decor, p, Decor::Street)?;
                }
                p = parent[index(map, p)].ok_or("Raccord incomplet")?;
            }
            let next = flood(map, start);
            changed |= next != reachable;
            reachable = next;
        }
        let missing =
            positions(map).find(|p| ordinary_passable(map, *p) && !reachable[index(map, *p)]);
        if missing.is_none() {
            return Ok(());
        }
        if !changed {
            return Err(format!(
                "Aucun raccord urbain possible pour {:?}",
                missing.unwrap()
            ));
        }
    }
}

fn flood(map: &Map, start: GridPos) -> Vec<bool> {
    let controls: Vec<_> = positions(map)
        .filter_map(|p| match map.tile(p).map(|t| t.terrain) {
            Some(Terrain::ControlPanel {
                door,
                activated: false,
            }) => Some((p, door)),
            _ => None,
        })
        .collect();
    let mut unlocked = BTreeSet::new();
    loop {
        let mut seen = vec![false; map.width() * map.height()];
        let mut queue = VecDeque::from([start]);
        seen[index(map, start)] = true;
        while let Some(p) = queue.pop_front() {
            for q in p.cardinal_neighbors() {
                if interior(map, q)
                    && !seen[index(map, q)]
                    && (ordinary_passable(map, q) || unlocked.contains(&q))
                {
                    seen[index(map, q)] = true;
                    queue.push_back(q);
                }
            }
        }
        let mut changed = false;
        for (p, door) in &controls {
            if p.cardinal_neighbors()
                .iter()
                .any(|q| interior(map, *q) && seen[index(map, *q)])
            {
                changed |= unlocked.insert(*door);
            }
        }
        if !changed {
            return seen;
        }
    }
}

fn ordinary_passable(map: &Map, p: GridPos) -> bool {
    map.is_walkable(p)
        || matches!(
            map.tile(p).map(|t| t.terrain),
            Some(Terrain::Door(DoorState::Closed | DoorState::Open))
        )
}
fn positions(map: &Map) -> impl Iterator<Item = GridPos> + '_ {
    (1..map.height() as i32 - 1)
        .flat_map(|y| (1..map.width() as i32 - 1).map(move |x| GridPos::new(x, y)))
}
fn interior(map: &Map, p: GridPos) -> bool {
    p.x > 0 && p.y > 0 && p.x < map.width() as i32 - 1 && p.y < map.height() as i32 - 1
}
fn index(map: &Map, p: GridPos) -> usize {
    p.y as usize * map.width() + p.x as usize
}
fn allowed(map: &Map, editable: &[bool], p: GridPos) -> bool {
    interior(map, p) && editable[index(map, p)]
}
fn mask(map: &Map, predicate: impl Fn(GridPos) -> bool) -> Vec<bool> {
    (0..map.height() as i32)
        .flat_map(|y| (0..map.width() as i32).map(move |x| GridPos::new(x, y)))
        .map(|p| interior(map, p) && predicate(p))
        .collect()
}
fn hub_editable(p: GridPos, map: &Map, home: crate::hub_layout::HomePlacement) -> bool {
    interior(map, p) && !home.authored_contains(p) && !map.is_protected(p)
}
fn cells([x, y, w, h]: [i32; 4]) -> Vec<GridPos> {
    (y..y + h)
        .flat_map(|py| (x..x + w).map(move |px| GridPos::new(px, py)))
        .collect()
}
fn distance(a: GridPos, b: GridPos) -> u32 {
    a.x.abs_diff(b.x).max(a.y.abs_diff(b.y))
}
fn random(rng: &mut GameRng, min: i32, max: i32) -> i32 {
    rng.usize_inclusive(min as usize, max.max(min) as usize)
        .unwrap() as i32
}
fn put(map: &mut Map, decor: &mut SectorDecor, p: GridPos, kind: Decor) -> Result<(), String> {
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
                .filter(|z| BUILDING_NAMES.contains(&z.name.as_str()))
                .count();
            assert!(buildings >= 45, "seed {seed}: {buildings} urban buildings");
            assert!(dense.decor.cells.values().filter(|d| d.blocks()).count() > 3000);
            assert!(dense.decor.cells.values().any(|d| *d == Decor::Street));
            for y in 0..before.height() as i32 {
                for x in 0..before.width() as i32 {
                    let p = GridPos::new(x, y);
                    if !hub_editable(p, &before, crate::hub_layout::HomePlacement::identity())
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
                            "seed {seed}, {p:?}"
                        );
                    }
                }
            }
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
