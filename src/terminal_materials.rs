//! Reviewed materials only. Layout, decor placement and loot belong to the world.
use super::{dim, terminal_art};
use crate::test_sector::Decor;
use macroquad::prelude::*;
use project_rl::world::GridPos;
use std::sync::OnceLock;

const CELL: i32 = 32;

fn image() -> &'static Image {
    static IMAGE: OnceLock<Image> = OnceLock::new();
    IMAGE.get_or_init(|| {
        Image::from_file_with_format(
            include_bytes!("../assets/sprites/cyberpunk/terrain-materials-32.png"),
            Some(ImageFormat::Png),
        )
        .expect("embedded terrain materials")
    })
}

thread_local! {
    static TEXTURE: Texture2D = {
        let texture = Texture2D::from_image(image());
        texture.set_filter(FilterMode::Nearest);
        texture
    };
}

pub(super) fn prewarm() {
    TEXTURE.with(|_| {});
}

pub(super) fn floor_pixel(family: u8, x: i32, y: i32) -> [u8; 4] {
    let cx = x / CELL;
    let cy = y / CELL;
    let variant = (cx * 17 + cy * 23 + (cx * cy) % 7).rem_euclid(4) as u8;
    let image = image();
    let column = i32::from(family * 4 + variant) * CELL + x.rem_euclid(CELL);
    let offset = (y.rem_euclid(CELL) * i32::from(image.width) + column) as usize * 4;
    image.bytes[offset..offset + 4].try_into().unwrap()
}

pub(super) fn wall(rect: Rect, joins: [bool; 4], visible: bool, at: GridPos) {
    TEXTURE.with(|texture| {
        draw_texture_ex(
            texture,
            rect.x,
            rect.y,
            dim(WHITE, visible),
            DrawTextureParams {
                source: Some(Rect::new(16.0 * 32.0, 0.0, 32.0, 32.0)),
                dest_size: Some(rect.size()),
                ..Default::default()
            },
        )
    });
    let scale = rect.w / 32.0;
    let paint = |x: f32, y: f32, w: f32, h: f32, color: Color| {
        draw_rectangle(
            rect.x + x * scale,
            rect.y + y * scale,
            w * scale,
            h * scale,
            dim(color, visible),
        );
    };
    let border = Color::from_rgba(21, 28, 35, 255);
    let light = Color::from_rgba(159, 168, 175, 255);
    let shade = Color::from_rgba(42, 49, 55, 255);
    if !joins[0] {
        paint(0., 0., 32., 1., border);
        paint(1., 1., 30., 1., light);
    }
    if !joins[3] {
        paint(0., 0., 1., 32., border);
        paint(1., 1., 1., 30., light);
    }
    if !joins[2] {
        paint(0., 31., 32., 1., border);
        paint(1., 30., 30., 1., shade);
    }
    if !joins[1] {
        paint(31., 0., 1., 32., border);
        paint(30., 1., 1., 30., shade);
    }
    if let Some(lamp) = terminal_art::wall_lamp_rect(rect.w, Decor::Wall, joins, at) {
        draw_rectangle(
            rect.x + lamp.x,
            rect.y + lamp.y,
            lamp.w,
            lamp.h,
            dim(Color::from_rgba(173, 203, 208, 255), visible),
        );
    }
}
