//! Ground at structural margins follows the adjacent side without changing data.
use super::assets::{Assets, BAND_END, BAND_START, TILE};
use super::scene::{self, Scene, Structure};
use macroquad::prelude::*;
use project_rl::world::{DoorState, GridPos, Terrain};

pub fn draw_structure(scene: &Scene, assets: &Assets, part: &Structure, origin: Vec2, cell: f32) {
    let (texture, turns) = if part.door.is_some() {
        let open =
            scene.game.map().tile(part.pos).unwrap().terrain == Terrain::Door(DoorState::Open);
        (&assets.walls[if open { 3 } else { 2 }][part.rotation], 0)
    } else {
        let (mask, turns) = scene.wall_sprite(part);
        (&assets.connections[mask], turns)
    };
    for (offset, tint) in [
        (
            vec2(cell / 32.0, cell / 16.0),
            Color::new(0.0, 0.0, 0.0, 0.3),
        ),
        (Vec2::ZERO, WHITE),
    ] {
        draw_texture_ex(
            texture,
            origin.x + offset.x,
            origin.y + offset.y,
            tint,
            DrawTextureParams {
                dest_size: Some(vec2(cell, cell)),
                rotation: turns as f32 * std::f32::consts::FRAC_PI_2,
                ..Default::default()
            },
        );
    }
}

fn ground<'a>(
    scene: &Scene,
    assets: &'a Assets,
    background: &'a [Texture2D],
    pos: Option<GridPos>,
) -> &'a Texture2D {
    let Some(pos) = pos else {
        return &assets.terrain[6];
    };
    match scene.document.floors[(pos.y * scene.document.width + pos.x) as usize] {
        Some(index) => &assets.terrain[index],
        None if pos.x < scene::WIDTH && pos.y < scene::HEIGHT => {
            &background[(pos.y * scene::WIDTH + pos.x) as usize]
        }
        None => &assets.terrain[6],
    }
}

pub fn draw_ground(
    scene: &Scene,
    assets: &Assets,
    background: &[Texture2D],
    pos: GridPos,
    origin: Vec2,
    cell: f32,
) {
    draw_texture_ex(
        ground(scene, assets, background, Some(pos)),
        origin.x,
        origin.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(cell, cell)),
            ..Default::default()
        },
    );
    if !scene.is_structure(pos) {
        return;
    }
    let boundaries = [0, BAND_START, BAND_END, TILE];
    for row in 0..3 {
        for column in 0..3 {
            if (column, row) == (1, 1) {
                continue;
            }
            let x = boundaries[column] as f32;
            let y = boundaries[row] as f32;
            let w = (boundaries[column + 1] - boundaries[column]) as f32;
            let h = (boundaries[row + 1] - boundaries[row]) as f32;
            let source = scene.margin_ground(pos, column as i32 - 1, row as i32 - 1);
            let scale = cell / TILE as f32;
            draw_texture_ex(
                ground(scene, assets, background, source),
                origin.x + x * scale,
                origin.y + y * scale,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(x, y, w, h)),
                    dest_size: Some(vec2(w, h) * scale),
                    ..Default::default()
                },
            );
        }
    }
}
