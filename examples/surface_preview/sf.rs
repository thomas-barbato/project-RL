//! Authored facility for reviewing the SF catalogue, independent of the campaign.
use super::scene::Scene;
use super::urban::{building, ground, object};
use project_rl::world::GridPos;

pub fn demo() -> Scene {
    let mut doc = Scene::empty(24, 16).unwrap().document;
    doc.floors.fill(Some(3));
    doc.spawn = GridPos::new(11, 8);
    ground(&mut doc, (0, 8, 23, 9), 23);
    building(&mut doc, (1, 1, 10, 7), 0, 0, GridPos::new(6, 7));
    building(&mut doc, (13, 1, 22, 7), 1, 3, GridPos::new(17, 7));
    building(&mut doc, (1, 10, 10, 14), 22, 0, GridPos::new(6, 10));
    building(&mut doc, (13, 10, 22, 14), 26, 0, GridPos::new(17, 10));
    for part in &mut doc.structures {
        if [(5, 1), (1, 4), (10, 4), (18, 10), (22, 12)].contains(&(part.pos.x, part.pos.y)) {
            part.style = 4;
        }
    }
    // Server aisles, controls, cooling and power supply.
    for (x, y, sprite) in [
        (2, 2, 56),
        (3, 2, 57),
        (4, 2, 58),
        (5, 2, 59),
        (8, 2, 60),
        (9, 2, 61),
        (2, 4, 62),
        (3, 4, 63),
        (8, 4, 70),
        (2, 6, 64),
        (4, 6, 65),
        (9, 6, 66),
    ] {
        object(&mut doc, x, y, sprite, 0);
    }
    // Secure armoury behind a laser perimeter, with storage and workspaces.
    for (x, y, sprite) in [
        (14, 2, 88),
        (15, 2, 89),
        (17, 2, 90),
        (19, 2, 91),
        (21, 2, 98),
        (14, 4, 92),
        (15, 4, 93),
        (18, 4, 96),
        (19, 4, 97),
        (21, 4, 99),
        (14, 6, 94),
        (15, 6, 95),
        (19, 6, 100),
        (20, 6, 101),
        (21, 6, 103),
    ] {
        object(&mut doc, x, y, sprite, 0);
    }
    // Crew quarters with beds, washroom equipment and underground access.
    for (x, y, sprite) in [
        (2, 11, 72),
        (4, 11, 73),
        (6, 11, 74),
        (8, 11, 75),
        (2, 13, 80),
        (4, 13, 85),
        (6, 13, 83),
        (8, 13, 84),
        (9, 12, 86),
        (9, 13, 87),
    ] {
        object(&mut doc, x, y, sprite, 0);
    }
    // Storage, laboratory, barrel variants and a second access to the basement.
    for (x, y, sprite) in [
        (14, 11, 76),
        (15, 11, 77),
        (16, 11, 78),
        (19, 11, 79),
        (21, 11, 82),
        (14, 13, 68),
        (16, 13, 69),
        (19, 13, 71),
        (21, 13, 81),
    ] {
        object(&mut doc, x, y, sprite, 0);
    }
    Scene::from_document(doc).expect("authored SF facility must be valid")
}
