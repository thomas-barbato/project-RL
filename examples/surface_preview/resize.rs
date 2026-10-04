//! Resize authored maps and keep their retained contents at the chosen anchor.
use super::scene::{Document, MAX_SIDE};
use project_rl::world::{DoorState, GridPos, Terrain};

pub const ANCHOR_NAMES: [&str; 9] = [
    "Haut gauche",
    "Haut centre",
    "Haut droite",
    "Milieu gauche",
    "Centre",
    "Milieu droite",
    "Bas gauche",
    "Bas centre",
    "Bas droite",
];

#[derive(Debug)]
pub struct Impact {
    pub shift: GridPos,
    pub removed_walls: usize,
    pub removed_objects: usize,
    pub moves_spawn: bool,
}

fn inside(pos: GridPos, width: i32, height: i32) -> bool {
    (0..width).contains(&pos.x) && (0..height).contains(&pos.y)
}

pub fn impact(doc: &Document, width: i32, height: i32, anchor: usize) -> Result<Impact, String> {
    if !(1..=MAX_SIDE).contains(&width) || !(1..=MAX_SIDE).contains(&height) || anchor > 8 {
        return Err(format!(
            "Chaque dimension doit être comprise entre 1 et {MAX_SIDE} cases."
        ));
    }
    let shift = GridPos::new(
        (width - doc.width) * (anchor % 3) as i32 / 2,
        (height - doc.height) * (anchor / 3) as i32 / 2,
    );
    let retained = |pos: GridPos| {
        inside(
            GridPos::new(pos.x + shift.x, pos.y + shift.y),
            width,
            height,
        )
    };
    Ok(Impact {
        shift,
        removed_walls: doc
            .structures
            .iter()
            .filter(|part| !retained(part.pos))
            .count(),
        removed_objects: doc
            .props
            .iter()
            .filter(|prop| prop.cells().any(|pos| !retained(pos)))
            .count(),
        moves_spawn: !retained(doc.spawn),
    })
}

impl Document {
    pub fn resized(&self, width: i32, height: i32, anchor: usize) -> Result<Self, String> {
        let plan = impact(self, width, height, anchor)?;
        if (width, height) == (self.width, self.height) {
            return Ok(self.clone());
        }
        let mut result = self.clone();
        result.width = width;
        result.height = height;
        result.background_offset = move_background(self.background_offset, plan.shift);
        result.floors = vec![Some(6); (width * height) as usize];
        let move_pos = |pos: GridPos| GridPos::new(pos.x + plan.shift.x, pos.y + plan.shift.y);
        result.background_tiles = self
            .background_tiles
            .iter()
            .filter_map(|(index, source)| {
                let next = move_pos(GridPos::new(
                    *index as i32 % self.width,
                    *index as i32 / self.width,
                ));
                inside(next, width, height).then_some(((next.y * width + next.x) as usize, *source))
            })
            .collect();
        for y in 0..self.height {
            for x in 0..self.width {
                let next = move_pos(GridPos::new(x, y));
                if inside(next, width, height) {
                    result.floors[(next.y * width + next.x) as usize] =
                        self.floors[(y * self.width + x) as usize];
                }
            }
        }
        if result.floors.iter().all(Option::is_some) {
            result.background_offset = GridPos::new(0, 0);
        }
        for part in &mut result.structures {
            part.pos = move_pos(part.pos);
        }
        result
            .structures
            .retain(|part| inside(part.pos, width, height));
        for prop in &mut result.props {
            prop.pos = move_pos(prop.pos);
        }
        result
            .props
            .retain(|prop| prop.cells().all(|pos| inside(pos, width, height)));
        result.markers = self
            .markers
            .iter()
            .filter_map(|marker| {
                let mut next = marker.clone();
                let left = (marker.pos.x + plan.shift.x).max(0);
                let top = (marker.pos.y + plan.shift.y).max(0);
                let right = (marker.pos.x + marker.width + plan.shift.x).min(width);
                let bottom = (marker.pos.y + marker.height + plan.shift.y).min(height);
                if left >= right || top >= bottom {
                    return None;
                }
                next.pos = GridPos::new(left, top);
                next.width = right - left;
                next.height = bottom - top;
                Some(next)
            })
            .collect();
        result.paint = self
            .paint
            .iter()
            .filter_map(|stroke| {
                stroke.resized(
                    self.width,
                    self.height,
                    width,
                    height,
                    plan.shift.x,
                    plan.shift.y,
                )
            })
            .collect();
        result.spawn = move_pos(self.spawn);
        let blocked: std::collections::BTreeSet<_> = result
            .structures
            .iter()
            .filter(|part| part.door != Some(DoorState::Open))
            .map(|part| part.pos)
            .chain(
                result
                    .props
                    .iter()
                    .filter(|prop| prop.blocking)
                    .flat_map(|prop| prop.cells()),
            )
            .collect();
        let passable = |pos: GridPos| {
            inside(pos, width, height)
                && !blocked.contains(&pos)
                && result.ground_terrain(pos) != Terrain::DeepWater
        };
        if !passable(result.spawn) {
            let target = GridPos::new(
                result.spawn.x.clamp(0, width - 1),
                result.spawn.y.clamp(0, height - 1),
            );
            let mut replacement = None;
            'search: for radius in 0..width + height {
                for dx in -radius..=radius {
                    let dy = radius - dx.abs();
                    for y in [target.y - dy, target.y + dy] {
                        let pos = GridPos::new(target.x + dx, y);
                        if passable(pos) {
                            replacement = Some(pos);
                            break 'search;
                        }
                    }
                }
            }
            result.spawn = replacement.ok_or_else(|| {
                "Il ne resterait aucune case accessible pour le départ du joueur.".to_owned()
            })?;
        }
        Ok(result)
    }
}

fn move_background(pos: GridPos, shift: GridPos) -> GridPos {
    GridPos::new(pos.x + shift.x, pos.y + shift.y)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paint::{Point, Stroke};
    use crate::scene::{Prop, Scene, Structure};

    fn materials() -> Vec<crate::assets::Raster> {
        (0..crate::catalog::MATERIAL_SLOTS)
            .map(|index| {
                let bytes = (0..64 * 64)
                    .flat_map(|pixel| {
                        [index as u8 * 5, (pixel % 64) as u8, (pixel / 64) as u8, 255]
                    })
                    .collect();
                crate::assets::Raster { bytes }
            })
            .collect()
    }

    #[test]
    fn every_growth_anchor_preserves_floors_props_and_exact_paint_pixels() {
        let mut doc = Scene::empty(4, 3).unwrap().document;
        for (i, floor) in doc.floors.iter_mut().enumerate() {
            *floor = Some(16 + i % 12);
        }
        doc.props.push(Prop {
            details: Default::default(),
            pos: GridPos::new(3, 2),
            sprite: 88,
            furniture: true,
            rotation: 3,
            blocking: true,
        });
        doc.structures.push(Structure {
            pos: GridPos::new(2, 1),
            door: None,
            rotation: 1,
            style: 4,
            fixed_connections: Some(10),
        });
        doc.paint.push(Stroke {
            clip: None,
            material: Some(5),
            diameter: 96,
            points: vec![Point::new(4, 8), Point::new(230, 175)],
        });
        let textures = materials();
        for anchor in 0..9 {
            let plan = impact(&doc, 6, 5, anchor).unwrap();
            let resized = doc.resized(6, 5, anchor).unwrap();
            let restored = Scene::from_document(
                serde_json::from_slice(&serde_json::to_vec(&resized).unwrap()).unwrap(),
            )
            .unwrap();
            assert_eq!(restored.document, resized);
            for y in 0..3 {
                for x in 0..4 {
                    let next = GridPos::new(x + plan.shift.x, y + plan.shift.y);
                    assert_eq!(
                        resized.floors[(next.y * 6 + next.x) as usize],
                        doc.floors[(y * 4 + x) as usize]
                    );
                    assert_eq!(
                        crate::paint::rasterize(next, &resized.paint, &textures),
                        crate::paint::rasterize(GridPos::new(x, y), &doc.paint, &textures),
                        "anchor {anchor}, ({x},{y})"
                    );
                }
            }
            assert_eq!(
                resized.props[0].pos,
                GridPos::new(3 + plan.shift.x, 2 + plan.shift.y)
            );
            assert_eq!(resized.props[0].rotation, 3);
            assert_eq!(resized.structures[0].style, 4);
            assert_eq!(resized.structures[0].rotation, 1);
            let new_empty = GridPos::new(
                if plan.shift.x == 0 { 5 } else { 0 },
                if plan.shift.y == 0 { 4 } else { 0 },
            );
            assert_eq!(
                resized.floors[(new_empty.y * 6 + new_empty.x) as usize],
                Some(6)
            );
            assert!(
                crate::paint::rasterize(new_empty, &resized.paint, &textures)
                    .iter()
                    .all(|&byte| byte == 0)
            );
        }
    }

    #[test]
    fn shrink_keeps_retained_paint_and_regrowth_does_not_restore_discarded_content() {
        let mut doc = Scene::empty(8, 5).unwrap().document;
        doc.spawn = GridPos::new(7, 4);
        doc.props.push(Prop {
            details: Default::default(),
            pos: GridPos::new(7, 3),
            sprite: 72,
            furniture: true,
            rotation: 2,
            blocking: true,
        });
        doc.props.push(Prop {
            details: Default::default(),
            pos: GridPos::new(3, 2),
            sprite: 56,
            furniture: true,
            rotation: 1,
            blocking: true,
        });
        doc.paint.push(Stroke {
            clip: None,
            material: Some(8),
            diameter: 96,
            points: vec![Point::new(0, 80), Point::new(490, 80)],
        });
        let small = doc.resized(4, 3, 0).unwrap();
        assert_eq!(small.props.len(), 1);
        assert_ne!(small.spawn, GridPos::new(3, 2));
        let scene = Scene::from_document(small.clone()).unwrap();
        assert!(scene.game.map().is_walkable(scene.document.spawn));
        let textures = materials();
        for y in 0..3 {
            for x in 0..4 {
                assert_eq!(
                    crate::paint::rasterize(GridPos::new(x, y), &doc.paint, &textures),
                    crate::paint::rasterize(GridPos::new(x, y), &small.paint, &textures)
                );
            }
        }
        let grown = small.resized(8, 5, 0).unwrap();
        assert_eq!(grown.props.len(), 1);
        assert!(
            crate::paint::rasterize(GridPos::new(6, 1), &grown.paint, &textures)
                .iter()
                .all(|&byte| byte == 0)
        );
        assert_eq!(grown.ground_terrain(GridPos::new(6, 1)), Terrain::Floor);
    }

    #[test]
    fn all_shrink_anchors_keep_the_matching_source_rectangle() {
        let mut doc = Scene::empty(5, 4).unwrap().document;
        for (i, floor) in doc.floors.iter_mut().enumerate() {
            *floor = Some(16 + i % 12);
        }
        for anchor in 0..9 {
            let plan = impact(&doc, 3, 2, anchor).unwrap();
            let resized = doc.resized(3, 2, anchor).unwrap();
            for y in 0..2 {
                for x in 0..3 {
                    assert_eq!(
                        resized.floors[(y * 3 + x) as usize],
                        doc.floors[((y - plan.shift.y) * 5 + x - plan.shift.x) as usize]
                    );
                }
            }
            Scene::from_document(resized).unwrap();
        }
    }

    #[test]
    fn moving_the_original_background_preserves_its_source_and_terrain() {
        let doc = Scene::new().document;
        let moved = doc.resized(24, 14, 8).unwrap();
        for y in 0..doc.height {
            for x in 0..doc.width {
                let next = GridPos::new(x + 4, y + 4);
                assert_eq!(moved.background_position(next), GridPos::new(x, y));
                assert_eq!(
                    moved.ground_terrain(next),
                    doc.ground_terrain(GridPos::new(x, y))
                );
                assert_eq!(
                    moved.floors[(next.y * 24 + next.x) as usize],
                    doc.floors[(y * doc.width + x) as usize]
                );
            }
        }
        Scene::from_document(moved).unwrap();
    }
}
