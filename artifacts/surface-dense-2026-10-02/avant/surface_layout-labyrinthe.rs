//! Versioned, connected surface complexes with narrow, winding circulation.
use crate::test_regional::GeneratedRegionalDestination;
use crate::test_sector::{Decor, SectorDecor, TestSector, Zone};
use project_rl::game::GameRng;
use project_rl::world::generation::{GeneratedMap, MapValidationRules, validate_interactive_map};
use project_rl::world::{DoorState, GridPos, Map, Terrain};
use std::collections::{BTreeSet, VecDeque};

const SALT: u64 = 0x5355_5246_494e_4649;
const ROOM_NAMES: [&str; 4] = [
    "Atelier abandonné",
    "Entrepôt de surface",
    "Local technique",
    "Hall de transit",
];

pub fn densify_hub(mut sector: TestSector, seed: u64) -> Result<TestSector, String> {
    let mut map = sector.level.map().clone();
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
    let anchors: Vec<_> = required
        .iter()
        .copied()
        .chain([
            GridPos::new(63, 21),
            GridPos::new(69, 11),
            GridPos::new(88, 21),
            GridPos::new(69, 32),
            GridPos::new(175, 108),
        ])
        .collect();
    let editable = mask(&map, |p| hub_editable(p, &map) && !required.contains(&p));
    labyrinth(
        &mut map,
        &mut sector.decor,
        &editable,
        &anchors,
        sector.level.player_start(),
        seed,
        &[
            vec![
                GridPos::new(63, 21), GridPos::new(104, 21),
                GridPos::new(104, 40), GridPos::new(126, 40),
                GridPos::new(126, 77), GridPos::new(175, 77),
                GridPos::new(175, 108),
            ],
        ],
    )?;
    for (_, p) in TestSector::EXPANDED_REGIONAL_PASSAGES {
        sector.decor.cells.insert(p, Decor::Passage);
    }
    sector.level = GeneratedMap::from_layout(
        map,
        sector.level.player_start(),
        sector.level.exit(),
        &required,
        MapValidationRules::default(),
    )
    .map_err(|e| format!("Complexe de surface inaccessible : {e}"))?;
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
    // Retain a place to stand when using a wall-mounted control or terminal.
    let interactions: Vec<_> = reserved.iter().copied().chain(
        (0..destination.blueprint.map.height() as i32).flat_map(|y| {
            (0..destination.blueprint.map.width() as i32).map(move |x| GridPos::new(x, y))
        }).filter(|p| matches!(destination.blueprint.map.tile(*p).map(|t| t.terrain),
            Some(Terrain::ControlPanel { .. })))
    ).collect();
    for p in interactions {
        if !destination.blueprint.map.is_walkable(p) {
            reserved.extend(p.cardinal_neighbors().into_iter()
                .filter(|q| destination.blueprint.map.is_walkable(*q)));
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
    labyrinth(
        &mut map,
        &mut decor,
        &editable,
        &anchors,
        destination.blueprint.entrance,
        seed,
        &[vec![
            GridPos::new(2, map.height() as i32 / 2),
            GridPos::new(map.width() as i32 / 3, map.height() as i32 / 2),
            GridPos::new(map.width() as i32 / 3, map.height() as i32 / 3),
            GridPos::new(2 * map.width() as i32 / 3, map.height() as i32 / 3),
            GridPos::new(2 * map.width() as i32 / 3, 2 * map.height() as i32 / 3),
            GridPos::new(map.width() as i32 - 3, 2 * map.height() as i32 / 3),
        ]],
    )?;
    validate_interactive_map(
        &map,
        destination.blueprint.entrance,
        destination.passages[0].1,
        &anchors,
        MapValidationRules::default(),
    )
    .map_err(|e| format!("Région bâtie inaccessible : {e}"))?;
    destination.blueprint.map = map;
    destination.decor = decor;
    Ok(())
}

fn labyrinth(
    map: &mut Map,
    decor: &mut SectorDecor,
    editable: &[bool],
    anchors: &[GridPos],
    start: GridPos,
    seed: u64,
    avenues: &[Vec<GridPos>],
) -> Result<(), String> {
    let mut rng = GameRng::from_seed(seed ^ SALT);
    // Excavate solid construction instead of placing buildings in open fields.
    for y in 1..map.height() as i32 - 1 {
        for x in 1..map.width() as i32 - 1 {
            let p = GridPos::new(x, y);
            if allowed(map, editable, p) {
                put(map, decor, p, Decor::Wall)?;
            }
        }
    }
    let columns = (map.width() as i32 - 8) / 9;
    let rows = (map.height() as i32 - 8) / 9;
    let mut nodes = vec![None; (columns * rows) as usize];
    let mut rooms = Vec::new();
    for row in 0..rows {
        for col in 0..columns {
            let p = GridPos::new(
                6 + col * 9 + rng.usize_inclusive(0, 2).unwrap() as i32 - 1,
                6 + row * 9 + rng.usize_inclusive(0, 2).unwrap() as i32 - 1,
            );
            if !allowed(map, editable, p) {
                continue;
            }
            nodes[(row * columns + col) as usize] = Some(p);
            let w = rng.usize_inclusive(5, 7).unwrap() as i32;
            let h = rng.usize_inclusive(5, 7).unwrap() as i32;
            let b = [p.x - w / 2, p.y - h / 2, w, h];
            if cells(b).iter().all(|p| allowed(map, editable, *p)) {
                let role = rng.usize_inclusive(0, 3).unwrap();
                for p in cells(b) {
                    put(
                        map,
                        decor,
                        p,
                        if role == 2 { Decor::Grate } else { Decor::Deck },
                    )?;
                }
                decor.zones.push(Zone {
                    name: ROOM_NAMES[role].to_owned(),
                    bounds: b,
                });
                rooms.push((p, b, role));
            } else {
                put(map, decor, p, Decor::Lane)?;
            }
        }
    }
    // Branches and dead ends, plus extra links for alternative routes.
    let mut visited = vec![false; nodes.len()];
    for root in 0..nodes.len() {
        if nodes[root].is_none() || visited[root] {
            continue;
        }
        visited[root] = true;
        let mut stack = vec![root];
        while let Some(&i) = stack.last() {
            let candidates: Vec<_> = neighbors(i, columns as usize, rows as usize)
                .into_iter()
                .filter(|j| !visited[*j] && nodes[*j].is_some())
                .collect();
            if candidates.is_empty() {
                stack.pop();
                continue;
            }
            let j = candidates[rng.usize_inclusive(0, candidates.len() - 1).unwrap()];
            connect(
                map,
                decor,
                editable,
                nodes[i].unwrap(),
                nodes[j].unwrap(),
                &mut rng,
            )?;
            visited[j] = true;
            stack.push(j);
        }
    }
    for i in 0..nodes.len() {
        let Some(a) = nodes[i] else {
            continue;
        };
        for j in neighbors(i, columns as usize, rows as usize) {
            if j > i
                && let Some(b) = nodes[j]
                && rng.next_u64().is_multiple_of(6)
            {
                connect(map, decor, editable, a, b, &mut rng)?;
            }
        }
    }
    for &p in anchors {
        for q in cells([p.x - 1, p.y - 1, 3, 3]) {
            if allowed(map, editable, q) {
                put(map, decor, q, Decor::Deck)?;
            }
        }
    }
    // A few wider routes provide bearings among the narrow winding passages.
    for avenue in avenues {
        for pair in avenue.windows(2) {
            let mut p = pair[0];
            loop {
                for q in cells([p.x - 1, p.y - 1, 3, 3]) {
                    if allowed(map, editable, q) {
                        put(map, decor, q, Decor::Lane)?;
                    }
                }
                if p == pair[1] { break; }
                p.x += (pair[1].x - p.x).signum();
                p.y += (pair[1].y - p.y).signum();
            }
        }
    }
    // Leave central aisles clear; furnishing is validated with the terrain.
    for (center, [x, y, w, h], role) in &rooms {
        for py in (*y + 1..*y + *h - 1).step_by(3) {
            for px in (*x + 1..*x + *w - 1).step_by(3) {
                let p = GridPos::new(px, py);
                if px == center.x || py == center.y || anchors.iter().any(|a| distance(*a, p) <= 1)
                    || !matches!(decor.cells.get(&p), Some(Decor::Deck | Decor::Grate))
                {
                    continue;
                }
                put(
                    map,
                    decor,
                    p,
                    [Decor::Console, Decor::Crate, Decor::Server, Decor::Pillar][*role],
                )?;
            }
        }
    }
    join_components(map, decor, editable, start)?;
    for (center, [x, y, w, h], _) in rooms {
        if !rng.next_u64().is_multiple_of(5) {
            continue;
        }
        for (p, outside) in [
            (GridPos::new(center.x, y), GridPos::new(center.x, y - 1)),
            (
                GridPos::new(x + w - 1, center.y),
                GridPos::new(x + w, center.y),
            ),
            (
                GridPos::new(center.x, y + h - 1),
                GridPos::new(center.x, y + h),
            ),
            (GridPos::new(x, center.y), GridPos::new(x - 1, center.y)),
        ] {
            if map.is_walkable(p)
                && map.is_walkable(outside)
                && !anchors.iter().any(|a| distance(*a, p) <= 1)
            {
                map.set_terrain(p, Terrain::Door(DoorState::Closed))
                    .map_err(|e| e.to_string())?;
                decor.cells.insert(p, Decor::DoorClosed);
                break;
            }
        }
    }
    Ok(())
}

fn connect(
    map: &mut Map,
    decor: &mut SectorDecor,
    editable: &[bool],
    a: GridPos,
    b: GridPos,
    rng: &mut GameRng,
) -> Result<(), String> {
    let offset = rng.usize_inclusive(0, 4).unwrap() as i32 - 2;
    let width = if rng.next_u64().is_multiple_of(3) {
        2
    } else {
        1
    };
    let points = if a.x.abs_diff(b.x) > a.y.abs_diff(b.y) {
        [
            a,
            GridPos::new(a.x, a.y + offset),
            GridPos::new(b.x, a.y + offset),
            b,
        ]
    } else {
        [
            a,
            GridPos::new(a.x + offset, a.y),
            GridPos::new(a.x + offset, b.y),
            b,
        ]
    };
    for pair in points.windows(2) {
        let [from, to] = [pair[0], pair[1]];
        let mut p = from;
        loop {
            for dy in 0..width {
                for dx in 0..width {
                    let q = GridPos::new(p.x + dx, p.y + dy);
                    if allowed(map, editable, q)
                        && !matches!(decor.cells.get(&q), Some(Decor::Deck | Decor::Grate))
                    {
                        put(map, decor, q, Decor::Lane)?;
                    }
                }
            }
            if p == to {
                break;
            }
            p.x += (to.x - p.x).signum();
            p.y += (to.y - p.y).signum();
        }
    }
    Ok(())
}

fn join_components(
    map: &mut Map,
    decor: &mut SectorDecor,
    editable: &[bool],
    start: GridPos,
) -> Result<(), String> {
    let mut reachable = flood(map, start);
    for y in 1..map.height() as i32 - 1 {
        for x in 1..map.width() as i32 - 1 {
            let origin = GridPos::new(x, y);
            if !possible(map, origin) || reachable[index(map, origin)] {
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
                        || (!allowed(map, editable, q) && !possible(map, q))
                        || seen[index(map, q)]
                    {
                        continue;
                    }
                    parent[index(map, q)] = Some(p);
                    seen[index(map, q)] = true;
                    queue.push_back(q);
                }
            }
            let mut p = target.ok_or_else(|| format!("Aucun raccord possible pour {origin:?}"))?;
            while p != origin {
                if allowed(map, editable, p) && !possible(map, p) {
                    put(map, decor, p, Decor::Lane)?;
                }
                p = parent[index(map, p)].ok_or("Raccord incomplet")?;
            }
            reachable = flood(map, start);
        }
    }
    Ok(())
}

fn flood(map: &Map, start: GridPos) -> Vec<bool> {
    let mut seen = vec![false; map.width() * map.height()];
    let mut queue = VecDeque::from([start]);
    seen[index(map, start)] = true;
    while let Some(p) = queue.pop_front() {
        for q in p.cardinal_neighbors() {
            if interior(map, q) && possible(map, q) && !seen[index(map, q)] {
                seen[index(map, q)] = true;
                queue.push_back(q);
            }
        }
    }
    seen
}
fn possible(map: &Map, p: GridPos) -> bool {
    map.is_walkable(p) || matches!(map.tile(p).map(|t| t.terrain), Some(Terrain::Door(_)))
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
fn hub_editable(p: GridPos, map: &Map) -> bool {
    interior(map, p)
        && !(p.x <= 62 && p.y <= 44)
        && !inside(p, [70, 2, 28, 19])
        && !inside(p, [64, 22, 11, 10])
        && !map.is_protected(p)
}
fn inside(p: GridPos, [x, y, w, h]: [i32; 4]) -> bool {
    p.x >= x && p.y >= y && p.x < x + w && p.y < y + h
}
fn cells([x, y, w, h]: [i32; 4]) -> Vec<GridPos> {
    (y..y + h)
        .flat_map(|py| (x..x + w).map(move |px| GridPos::new(px, py)))
        .collect()
}
fn distance(a: GridPos, b: GridPos) -> u32 {
    a.x.abs_diff(b.x).max(a.y.abs_diff(b.y))
}
fn neighbors(i: usize, w: usize, h: usize) -> Vec<usize> {
    let mut result = Vec::new();
    if i % w > 0 {
        result.push(i - 1);
    }
    if i % w + 1 < w {
        result.push(i + 1);
    }
    if i / w > 0 {
        result.push(i - w);
    }
    if i / w + 1 < h {
        result.push(i + w);
    }
    result
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
            assert!(
                dense
                    .decor
                    .zones
                    .iter()
                    .filter(|z| ROOM_NAMES.contains(&z.name.as_str()))
                    .count()
                    > 100
            );
            assert!(
                dense.decor.cells.values().filter(|d| d.blocks()).count()
                    > before.width() * before.height() / 4
            );
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
