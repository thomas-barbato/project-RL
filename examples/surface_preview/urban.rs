//! Authored street for reviewing the proposed assets; never used by the campaign.
use super::scene::{Document, Prop, Scene, Structure};
use project_rl::world::{DoorState, GridPos};

pub(super) fn ground(doc: &mut Document, rect: (i32, i32, i32, i32), material: usize) {
    let (left, top, right, bottom) = rect;
    for y in top..=bottom {
        for x in left..=right {
            doc.floors[(y * doc.width + x) as usize] = Some(material);
        }
    }
}

pub(super) fn building(
    doc: &mut Document,
    rect: (i32, i32, i32, i32),
    floor: usize,
    style: usize,
    door: GridPos,
) {
    ground(doc, rect, floor);
    let (left, top, right, bottom) = rect;
    for y in top..=bottom {
        for x in left..=right {
            if x != left && x != right && y != top && y != bottom {
                continue;
            }
            let pos = GridPos::new(x, y);
            let mask = match (x == left, x == right, y == top, y == bottom) {
                (true, _, true, _) => 6,
                (_, true, true, _) => 12,
                (true, _, _, true) => 3,
                (_, true, _, true) => 9,
                _ => 10,
            };
            doc.structures.push(Structure {
                pos,
                style,
                door: (pos == door).then_some(DoorState::Open),
                rotation: if x == left {
                    3
                } else if x == right {
                    1
                } else if y == bottom {
                    2
                } else {
                    0
                },
                fixed_connections: if pos == door { None } else { Some(mask) },
            });
            if super::scene::CORNER_MASKS.contains(&mask) {
                doc.structures.last_mut().unwrap().rotation = 0;
            }
            if pos == door {
                doc.structures.last_mut().unwrap().rotation = usize::from(x == left || x == right);
            }
        }
    }
}

pub(super) fn object(doc: &mut Document, x: i32, y: i32, sprite: usize, rotation: usize) {
    doc.props.push(Prop {
        details: Default::default(),
        pos: GridPos::new(x, y),
        sprite,
        furniture: true,
        rotation,
        blocking: super::catalog::default_blocking(sprite, true),
    });
}

pub fn demo() -> Scene {
    let mut doc = Scene::empty(26, 16).unwrap().document;
    doc.floors.fill(Some(18));
    doc.spawn = GridPos::new(13, 8);
    ground(&mut doc, (0, 7, 25, 9), 16);
    ground(&mut doc, (13, 0, 14, 15), 17);
    ground(&mut doc, (11, 0, 12, 6), 19);
    ground(&mut doc, (15, 0, 15, 6), 19);
    building(&mut doc, (1, 1, 10, 6), 22, 1, GridPos::new(6, 6));
    building(&mut doc, (16, 1, 24, 6), 25, 2, GridPos::new(20, 6));
    building(&mut doc, (1, 10, 10, 14), 24, 2, GridPos::new(6, 10));
    building(&mut doc, (16, 10, 24, 14), 26, 0, GridPos::new(20, 10));
    // Grocery: shelving, fresh goods, service counter and storage.
    for (x, y, sprite) in [
        (2, 2, 11),
        (3, 2, 11),
        (5, 2, 10),
        (7, 2, 9),
        (9, 2, 11),
        (3, 4, 8),
        (4, 4, 8),
        (8, 4, 10),
        (9, 4, 34),
    ] {
        object(&mut doc, x, y, sprite, 0);
    }
    // Home: living room, kitchen and sleeping corner, with clear aisles.
    for (x, y, sprite) in [
        (17, 2, 15),
        (19, 2, 14),
        (22, 2, 18),
        (23, 2, 16),
        (22, 3, 17),
        (17, 4, 5),
        (18, 4, 2),
        (20, 4, 4),
        (23, 4, 19),
    ] {
        object(&mut doc, x, y, sprite, 0);
    }
    // Clinic / reception.
    for (x, y, sprite) in [
        (2, 11, 12),
        (3, 11, 13),
        (8, 11, 21),
        (2, 13, 13),
        (7, 13, 20),
        (9, 13, 20),
    ] {
        object(&mut doc, x, y, sprite, 0);
    }
    // Working workshop, tools and goods awaiting delivery.
    for (x, y, sprite) in [
        (17, 11, 0),
        (22, 11, 23),
        (23, 11, 22),
        (17, 13, 36),
        (19, 13, 37),
        (22, 13, 39),
    ] {
        object(&mut doc, x, y, sprite, 0);
    }
    // Small outdoor market and public street fixtures.
    for (x, y, sprite) in [
        (11, 1, 32),
        (11, 3, 34),
        (11, 5, 33),
        (15, 2, 35),
        (0, 0, 24),
        (10, 0, 27),
        (16, 0, 27),
        (25, 0, 24),
        (0, 6, 26),
        (12, 6, 24),
        (15, 6, 24),
        (25, 6, 26),
        (0, 10, 24),
        (12, 10, 24),
        (15, 10, 24),
        (25, 10, 24),
        (11, 12, 25),
        (11, 14, 28),
        (15, 12, 30),
        (25, 12, 29),
        (25, 15, 27),
        (0, 15, 27),
    ] {
        object(&mut doc, x, y, sprite, 0);
    }
    for (x, y, sprite) in [
        (0, 4, 45),
        (25, 4, 46),
        (15, 4, 47),
        (12, 0, 48),
        (15, 0, 42),
        (12, 5, 40),
        (15, 11, 44),
        (11, 11, 43),
        (12, 13, 50),
        (13, 10, 49),
        (23, 13, 55),
        (18, 11, 52),
        (21, 13, 53),
        (8, 13, 54),
        (25, 14, 41),
    ] {
        object(&mut doc, x, y, sprite, 0);
    }
    Scene::from_document(doc).expect("authored urban review map must be valid")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn review_street_roundtrips_with_walkable_access_to_all_four_buildings() {
        let scene = demo();
        let json = serde_json::to_vec(&scene.document).unwrap();
        let restored = Scene::from_document(serde_json::from_slice(&json).unwrap()).unwrap();
        assert_eq!(restored.document, scene.document);
        let mut visited = std::collections::BTreeSet::from([scene.document.spawn]);
        let mut queue = std::collections::VecDeque::from([scene.document.spawn]);
        while let Some(pos) = queue.pop_front() {
            for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
                let next = GridPos::new(pos.x + dx, pos.y + dy);
                if scene.game.map().tile(next).is_some_and(|tile| {
                    tile.terrain == project_rl::world::Terrain::Floor
                        || tile.terrain == project_rl::world::Terrain::Door(DoorState::Open)
                }) && visited.insert(next)
                {
                    queue.push_back(next);
                }
            }
        }
        for target in [(6, 5), (20, 5), (6, 11), (20, 11)] {
            assert!(
                visited.contains(&GridPos::new(target.0, target.1)),
                "inaccessible building {target:?}"
            );
        }
    }
}
