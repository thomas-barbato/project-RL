//! Illustrated r04 kit: fixed camera and painted orientations, no rotations.
use super::scene::{Scene, Structure};
use macroquad::prelude::*;
use project_rl::world::{DoorState, Terrain};
use std::path::PathBuf;

fn image(name: &str) -> Image {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("assets/prototypes/handcrafted-camp-01/r04")
        .join(name);
    Image::from_file_with_format(&std::fs::read(path).unwrap(), None).unwrap()
}
fn texture(name: &str) -> Texture2D {
    let result = Texture2D::from_image(&image(name));
    result.set_filter(FilterMode::Nearest);
    result
}
pub struct Kit {
    floors: Texture2D,
    walls: Texture2D,
    doors: Texture2D,
    lamp: Texture2D,
}
impl Kit {
    pub fn load(corrected_doors: bool) -> Self {
        Self {
            floors: texture("floors-native64.png"),
            walls: texture("walls-native64.png"),
            doors: texture(if corrected_doors {
                "../r05/doors-native64.png"
            } else {
                "doors-native64.png"
            }),
            lamp: texture("lamp-native.png"),
        }
    }
    pub fn ground(&self, x: i32, y: i32, w: i32, h: i32, origin: Vec2) {
        // Ground ends inside the exterior walls. Outside remains dark void.
        let left = if x == 0 { 32. } else { 0. };
        let top = if y == 0 { 32. } else { 0. };
        let right = if x == w - 1 { 32. } else { 64. };
        let bottom = if y == h - 1 { 32. } else { 64. };
        let index = (x * 3 + y * 5).rem_euclid(4);
        // Static preview illumination is evaluated in world coordinates;
        // shared vertices prevent a checkerboard of differently tinted tiles.
        let light = |dx: f32, dy: f32| {
            let wx = x as f32 * 64. + dx;
            let wy = y as f32 * 64. + dy;
            let warm = [2., 5., 10., 12.]
                .into_iter()
                .map(|lamp| {
                    let distance = (wx - (lamp + 0.5) * 64.).powi(2) + (wy - 48.).powi(2);
                    (-distance / 17000.).exp()
                })
                .sum::<f32>()
                .min(1.);
            Color::new(
                (0.72 + 0.28 * warm).min(1.),
                0.72 + 0.2 * warm,
                0.72 + 0.03 * warm,
                1.,
            )
        };
        let vertices = [(left, top), (right, top), (right, bottom), (left, bottom)]
            .into_iter()
            .map(|(dx, dy)| {
                Vertex::new(
                    origin.x + dx,
                    origin.y + dy,
                    0.,
                    ((index % 2) as f32 * 64. + dx) / 128.,
                    ((index / 2) as f32 * 64. + dy) / 128.,
                    light(dx, dy),
                )
            })
            .collect();
        draw_mesh(&Mesh {
            vertices,
            indices: vec![0, 1, 2, 0, 2, 3],
            texture: Some(self.floors.clone()),
        });
    }
    pub fn wall(&self, scene: &Scene, part: &Structure, origin: Vec2, shadow: bool) {
        let (texture, slot, columns) = if part.door.is_some() {
            let closed =
                scene.game.map().tile(part.pos).unwrap().terrain != Terrain::Door(DoorState::Open);
            (
                &self.doors,
                usize::from(part.rotation % 2 == 0) * 2 + usize::from(closed),
                2,
            )
        } else {
            (&self.walls, scene.neighbors(part.pos) as usize, 4)
        };
        let offset = if shadow { vec2(2., 3.) } else { Vec2::ZERO };
        let tint = if shadow {
            Color::new(0., 0., 0., 0.32)
        } else {
            WHITE
        };
        draw_texture_ex(
            texture,
            origin.x + offset.x,
            origin.y + offset.y,
            tint,
            DrawTextureParams {
                source: Some(Rect::new(
                    (slot % columns) as f32 * 64.,
                    (slot / columns) as f32 * 64.,
                    64.,
                    64.,
                )),
                ..Default::default()
            },
        );
    }
    pub fn lamps(&self, origin: Vec2) {
        for x in [2., 5., 10., 12.] {
            draw_texture(
                &self.lamp,
                origin.x + (x + 0.5) * 64. - 18.,
                origin.y + 36.,
                WHITE,
            );
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    fn at(image: &Image, slot: usize, columns: usize, x: usize, y: usize) -> [u8; 4] {
        let i =
            (((slot / columns) * 64 + y) * image.width as usize + (slot % columns) * 64 + x) * 4;
        image.bytes[i..i + 4].try_into().unwrap()
    }
    #[test]
    fn painted_t_pieces_have_three_matching_ports_and_connected_centers() {
        let source = image("walls-native64.png");
        assert_eq!((source.width, source.height), (256, 256));
        for mask in [7, 11, 13, 14] {
            assert_eq!(at(&source, mask, 4, 32, 32)[3], 255);
            for i in 0..64 {
                if mask & 1 != 0 {
                    assert_eq!(at(&source, mask, 4, i, 0), at(&source, 5, 4, i, 0));
                }
                if mask & 4 != 0 {
                    assert_eq!(at(&source, mask, 4, i, 63), at(&source, 5, 4, i, 0));
                }
                if mask & 8 != 0 {
                    assert_eq!(at(&source, mask, 4, 0, i), at(&source, 10, 4, 0, i));
                }
                if mask & 2 != 0 {
                    assert_eq!(at(&source, mask, 4, 63, i), at(&source, 10, 4, 0, i));
                }
            }
            let mut seen = std::collections::BTreeSet::new();
            let mut queue = vec![(32_i32, 32_i32)];
            while let Some((x, y)) = queue.pop() {
                if !(0..64).contains(&x)
                    || !(0..64).contains(&y)
                    || at(&source, mask, 4, x as usize, y as usize)[3] == 0
                    || !seen.insert((x, y))
                {
                    continue;
                }
                queue.extend([(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]);
            }
            for (bit, p) in [(1, (32, 0)), (2, (63, 32)), (4, (32, 63)), (8, (0, 32))] {
                if mask & bit != 0 {
                    assert!(seen.contains(&p), "T arm disconnected: {mask}/{bit}");
                }
            }
            let opaque = (0..64)
                .flat_map(|y| (0..64).map(move |x| (x, y)))
                .filter(|&(x, y)| at(&source, mask, 4, x, y)[3] > 0)
                .count();
            assert_eq!(seen.len(), opaque, "stray pixels in painted T: {mask}");
        }
    }
    #[test]
    fn both_open_door_axes_leave_the_entire_center_visible() {
        let source = image("../r05/doors-native64.png");
        for slot in [0, 2] {
            for y in 16..48 {
                for x in 16..48 {
                    assert_eq!(at(&source, slot, 2, x, y)[3], 0);
                }
            }
        }
    }
    #[test]
    fn corrected_door_parts_connect_to_the_neighboring_wall_edges() {
        let source = image("../r05/doors-native64.png");
        for slot in 0..4 {
            let mut remaining: std::collections::BTreeSet<_> = (0..64_i32)
                .flat_map(|y| (0..64_i32).map(move |x| (x, y)))
                .filter(|&(x, y)| at(&source, slot, 2, x as usize, y as usize)[3] > 0)
                .collect();
            let mut components = 0;
            while let Some(&start) = remaining.first() {
                remaining.remove(&start);
                let mut queue = vec![start];
                let mut attached = false;
                while let Some((x, y)) = queue.pop() {
                    attached |= if slot < 2 {
                        y == 0 || y == 63
                    } else {
                        x == 0 || x == 63
                    };
                    for p in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                        if remaining.remove(&p) {
                            queue.push(p);
                        }
                    }
                }
                assert!(attached, "floating part in door {slot}");
                components += 1;
            }
            assert_eq!(components, if slot % 2 == 0 { 2 } else { 1 });
        }
    }
}
