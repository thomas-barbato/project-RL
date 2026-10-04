//! Road paint follows known corridor geometry, never hidden map cells.
use super::dim;
use crate::test_sector::Decor;
use macroquad::prelude::*;
use project_rl::world::GridPos;

fn road(kind: Decor) -> bool {
    matches!(
        kind,
        Decor::Lane
            | Decor::Street
            | Decor::TransitSign
            | Decor::StreetDrain
            | Decor::LampPost
            | Decor::Bin
            | Decor::Hydrant
            | Decor::RoadBollard
    )
}

/// Pavement is a presentation of already known ground beside a city road.
/// Doors and walls stop the search, so nearby rooms keep their interior floor.
pub(super) fn sidewalk(
    position: GridPos,
    kind: Decor,
    known: impl Fn(GridPos) -> Option<Decor>,
) -> bool {
    let paving = |d| {
        matches!(
            d,
            Decor::Deck
                | Decor::Bench
                | Decor::Bin
                | Decor::LampPost
                | Decor::CafeTable
                | Decor::EntranceMat
                | Decor::VendingMachine
        )
    };
    if !paving(kind) {
        return false;
    }
    for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
        for step in 1..=3 {
            match known(GridPos::new(position.x + dx * step, position.y + dy * step)) {
                Some(Decor::Street | Decor::StreetDrain | Decor::Hydrant | Decor::RoadBollard) => {
                    return true;
                }
                Some(d) if paving(d) && !d.blocks() => {}
                _ => break,
            }
        }
    }
    false
}

pub(super) fn draw_curb(
    rect: Rect,
    visible: bool,
    position: GridPos,
    known: impl Fn(GridPos) -> Option<Decor>,
) {
    let color = dim(Color::from_rgba(98, 102, 93, 255), visible);
    let width = (rect.w * 0.06).max(1.0);
    for (i, p) in position.cardinal_neighbors().into_iter().enumerate() {
        if !matches!(
            known(p),
            Some(Decor::Street | Decor::StreetDrain | Decor::Hydrant | Decor::RoadBollard)
        ) {
            continue;
        }
        let (x, y, w, h) = match i {
            0 => (rect.x, rect.y, rect.w, width),
            1 => (rect.right() - width, rect.y, width, rect.h),
            2 => (rect.x, rect.bottom() - width, rect.w, width),
            _ => (rect.x, rect.y, width, rect.h),
        };
        draw_rectangle(x, y, w, h, color);
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Marking {
    horizontal: bool,
    offset: f32,
}

fn marking(position: GridPos, known: impl Fn(GridPos) -> Option<Decor>) -> Option<Marking> {
    let run = |dx, dy| {
        for step in 1..=6 {
            match known(GridPos::new(position.x + dx * step, position.y + dy * step)) {
                Some(kind) if road(kind) => {}
                Some(kind) => {
                    // An isolated machine or planter is not a street edge.
                    let boundary = !kind.blocks()
                        || matches!(kind, Decor::Wall | Decor::RuinWall | Decor::Barricade);
                    return (step - 1, boundary);
                }
                None => return (step - 1, false),
            }
        }
        (6, false)
    };
    let (north, nb) = run(0, -1);
    let (south, sb) = run(0, 1);
    let (west, wb) = run(-1, 0);
    let (east, eb) = run(1, 0);
    let vertical_width = north + south + 1;
    let horizontal_width = west + east + 1;
    let minimum = if known(position) == Some(Decor::Lane) {
        3
    } else {
        4
    };
    let (horizontal, offset) = if nb
        && sb
        && (minimum..=7).contains(&vertical_width)
        && horizontal_width >= vertical_width + 2
    {
        (true, vertical_width as f32 * 0.5 - north as f32)
    } else if wb
        && eb
        && (minimum..=7).contains(&horizontal_width)
        && vertical_width >= horizontal_width + 2
    {
        (false, horizontal_width as f32 * 0.5 - west as f32)
    } else {
        return None;
    };
    (0.0..=1.0)
        .contains(&offset)
        .then_some(Marking { horizontal, offset })
}

pub(super) fn draw(
    rect: Rect,
    kind: Decor,
    visible: bool,
    position: GridPos,
    known: impl Fn(GridPos) -> Option<Decor>,
) {
    if !matches!(kind, Decor::Lane | Decor::Street) {
        return;
    }
    let Some(mark) = marking(position, known) else {
        return;
    };
    let along = if mark.horizontal {
        position.x
    } else {
        position.y
    };
    if along.rem_euclid(3) == 2 {
        return;
    }
    let thickness = (rect.w * 0.055).max(1.0);
    let color = dim(Color::from_rgba(152, 150, 130, 255), visible);
    // Clip both halves of an even-width avenue's central line to their own
    // cells. Paint cannot leak into an unobserved or neighbouring floor.
    if mark.horizontal {
        let center = rect.y + rect.h * mark.offset;
        let top = (center - thickness * 0.5).max(rect.y);
        let bottom = (center + thickness * 0.5).min(rect.bottom());
        draw_rectangle(rect.x, top, rect.w, bottom - top, color);
    } else {
        let center = rect.x + rect.w * mark.offset;
        let left = (center - thickness * 0.5).max(rect.x);
        let right = (center + thickness * 0.5).min(rect.right());
        draw_rectangle(left, rect.y, right - left, rect.h, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pavement_uses_known_streets_and_stops_at_building_entrances() {
        let p = GridPos::new(10, 10);
        for separator in [
            Decor::Wall,
            Decor::DoorOpen,
            Decor::DoorClosed,
            Decor::Threshold,
        ] {
            let room = |q: GridPos| {
                Some(if q.x == 11 {
                    separator
                } else if q.x == 12 {
                    Decor::Street
                } else {
                    Decor::Deck
                })
            };
            assert!(!sidewalk(p, Decor::Deck, room), "{separator:?}");
        }
        let street = |q: GridPos| {
            Some(if q.x == 12 {
                Decor::Street
            } else {
                Decor::Deck
            })
        };
        assert!(sidewalk(p, Decor::Deck, street));
        assert!(sidewalk(p, Decor::Bench, street));
        assert!(!sidewalk(GridPos::new(8, 10), Decor::Deck, street));
        assert!(!sidewalk(p, Decor::Grate, street));
        assert!(!sidewalk(p, Decor::Deck, |q| (q.x != 11).then(|| {
            if q.x == 12 {
                Decor::Street
            } else {
                Decor::Deck
            }
        })));
    }

    #[test]
    fn road_paint_follows_both_axes_and_stops_at_crossroads() {
        let horizontal = |p: GridPos| {
            Some(if (2..8).contains(&p.y) {
                Decor::Street
            } else {
                Decor::Wall
            })
        };
        assert_eq!(
            marking(GridPos::new(20, 4), horizontal),
            Some(Marking {
                horizontal: true,
                offset: 1.0
            })
        );
        assert_eq!(
            marking(GridPos::new(20, 5), horizontal),
            Some(Marking {
                horizontal: true,
                offset: 0.0
            })
        );
        assert_eq!(marking(GridPos::new(20, 3), horizontal), None);
        let vertical = |p: GridPos| horizontal(GridPos::new(p.y, p.x));
        assert_eq!(
            marking(GridPos::new(5, 20), vertical),
            Some(Marking {
                horizontal: false,
                offset: 0.0
            })
        );
        let crossing = |p: GridPos| {
            Some(if (2..8).contains(&p.y) || (17..23).contains(&p.x) {
                Decor::Street
            } else {
                Decor::Wall
            })
        };
        assert_eq!(marking(GridPos::new(20, 5), crossing), None);
    }

    #[test]
    fn unknown_cells_cannot_define_road_boundaries() {
        let partial = |p: GridPos| (2..8).contains(&p.y).then_some(Decor::Street);
        assert_eq!(marking(GridPos::new(20, 5), partial), None);
    }
}
