//! Editable review scene for new wall families; existing maps stay intact.
use super::{
    author::{self, Placement, Tool},
    editor::Brush,
    scene::{Prop, PropDetails, Scene},
};
use project_rl::world::GridPos;

pub fn demo() -> Scene {
    let mut doc = Scene::empty(38, 18).unwrap().document;
    doc.spawn = GridPos::new(18, 15);
    doc.floors.fill(Some(7));
    for (style, x) in [(5, 2), (6, 14), (7, 26)] {
        for y in 4..13 {
            for xx in x + 1..x + 9 {
                doc.floors[(y * doc.width + xx) as usize] = Some(18);
            }
        }
        let cells = author::shape(
            Tool::Rectangle,
            GridPos::new(x, 3),
            GridPos::new(x + 9, 13),
            Brush::Wall,
        );
        author::place(
            &mut doc,
            &cells,
            &Placement {
                brush: Brush::Wall,
                rotation: 0,
                style,
                fixed: None,
                blocking: true,
                details: &Default::default(),
                layers: &Default::default(),
            },
        )
        .unwrap();
        doc.structures.retain(|s| s.pos != GridPos::new(x + 4, 13));
        for (sprite, xx, y) in [(25, x + 2, 6), (27, x + 7, 8), (76, x + 2, 10)] {
            doc.props.push(Prop {
                pos: GridPos::new(xx, y),
                sprite,
                furniture: true,
                rotation: 0,
                blocking: true,
                details: PropDetails::default(),
            });
        }
    }
    Scene::from_document(doc).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn review_walls_keep_all_four_corners_and_walkable_access() {
        let scene = demo();
        assert!(super::super::author::validate(&scene.document).is_empty());
        for style in 5..8 {
            let wall = scene
                .document
                .structures
                .iter()
                .find(|s| s.style == style)
                .unwrap();
            assert!(!scene.game.map().is_walkable(wall.pos));
            assert_eq!(scene.game.map().blocks_vision(wall.pos), style != 7);
            for mask in [3, 6, 9, 12] {
                assert!(
                    scene
                        .document
                        .structures
                        .iter()
                        .filter(|s| s.style == style)
                        .any(|s| scene.wall_sprite(s).0 == mask)
                );
            }
        }
        let json = serde_json::to_vec(&scene.document).unwrap();
        assert_eq!(
            Scene::from_document(serde_json::from_slice(&json).unwrap())
                .unwrap()
                .document,
            scene.document
        );
    }
}
