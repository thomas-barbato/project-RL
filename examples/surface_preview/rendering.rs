//! Ground at structural margins follows the adjacent side without changing data.
use super::assets::{Assets, BAND_END, BAND_START, TILE};
use super::scene::{self, Scene, Structure};
use macroquad::prelude::*;
use project_rl::world::{DoorState, GridPos, Terrain};

pub fn draw_prop(assets: &Assets, prop: &scene::Prop, origin: Vec2, cell: f32) {
    draw_prop_tinted(assets, prop, origin, cell, WHITE);
}

pub fn draw_prop_tinted(assets: &Assets, prop: &scene::Prop, origin: Vec2, cell: f32, tint: Color) {
    let (texture, turns) = if prop.furniture {
        assets.object_texture(prop.sprite, prop.rotation)
    } else {
        (&assets.terrain[prop.sprite], prop.rotation)
    };
    let (w, h) = prop.size();
    let center = origin
        + vec2(w as f32, h as f32) * cell / 2.0
        + vec2(prop.details.offset_x as f32, prop.details.offset_y as f32) * cell / 64.0;
    let size = (if prop.furniture && assets.directions.contains_key(&prop.sprite) {
        vec2(w as f32, h as f32) * cell
    } else {
        vec2(prop.details.width as f32, prop.details.height as f32) * cell
    }) * prop.details.scale as f32
        / 100.0;
    let wind = if prop.furniture && matches!(prop.sprite,104..=106 | 108..=109) {
        assets.animation.time().map_or(0., |time| {
            (time as f32 * 0.9 + prop.pos.x as f32 * 0.7).sin() * 0.012
        })
    } else {
        0.
    };
    let lights =
        prop.furniture && matches!(prop.sprite, 42 | 43 | 48..=71 | 84 | 85 | 97 | 101 | 103);
    if lights {
        assets.animation.begin(2.);
    }
    for (offset, color) in [
        (
            vec2(cell / 32.0, cell / 16.0),
            Color::new(0.0, 0.0, 0.0, 0.3),
        ),
        (Vec2::ZERO, WHITE),
    ] {
        let start = center - size / 2.0 + offset;
        draw_texture_ex(
            texture,
            start.x,
            start.y,
            Color::new(
                color.r * tint.r,
                color.g * tint.g,
                color.b * tint.b,
                color.a * tint.a,
            ),
            DrawTextureParams {
                dest_size: Some(size),
                rotation: turns as f32 * std::f32::consts::FRAC_PI_2 + wind,
                pivot: (wind != 0.).then_some(center + vec2(0., size.y * 0.32)),
                ..Default::default()
            },
        );
    }
    if lights {
        assets.animation.end();
    }
}

pub fn draw_structure(scene: &Scene, assets: &Assets, part: &Structure, origin: Vec2, cell: f32) {
    draw_structure_tinted(scene, assets, part, origin, cell, WHITE);
}
pub fn draw_structure_tinted(
    scene: &Scene,
    assets: &Assets,
    part: &Structure,
    origin: Vec2,
    cell: f32,
    tint: Color,
) {
    let (texture, turns) = if part.door.is_some() {
        let open =
            scene.game.map().tile(part.pos).unwrap().terrain == Terrain::Door(DoorState::Open);
        (&assets.walls[if open { 3 } else { 2 }][part.rotation], 0)
    } else {
        let (mask, turns) = scene.wall_sprite(part);
        (assets.connection(part.style, mask), turns)
    };
    let lights = matches!(part.style, 3 | 4) || part.door.is_some();
    if lights {
        assets.animation.begin(2.);
    }
    for (offset, color) in [
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
            Color::new(
                color.r * tint.r,
                color.g * tint.g,
                color.b * tint.b,
                color.a * tint.a,
            ),
            DrawTextureParams {
                dest_size: Some(vec2(cell, cell)),
                rotation: turns as f32 * std::f32::consts::FRAC_PI_2,
                ..Default::default()
            },
        );
    }
    if lights {
        assets.animation.end();
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
        None if (0..scene::WIDTH).contains(&scene.document.background_position(pos).x)
            && (0..scene::HEIGHT).contains(&scene.document.background_position(pos).y) =>
        {
            let source = scene.document.background_position(pos);
            &background[(source.y * scene::WIDTH + source.x) as usize]
        }
        None => &assets.terrain[6],
    }
}

fn draw_paint(assets: &Assets, pos: GridPos, origin: Vec2, cell: f32) {
    let mut cache = assets.paint.borrow_mut();
    let water = cache.water_at(pos);
    let Some(texture) = cache.texture(pos) else {
        return;
    };
    if water {
        assets.animation.begin(1.);
    }
    draw_texture_ex(
        texture,
        origin.x,
        origin.y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(cell, cell)),
            ..Default::default()
        },
    );
    if water {
        assets.animation.end();
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
    draw_floor(scene, assets, background, pos, origin, cell, true);
}

pub fn draw_floor(
    scene: &Scene,
    assets: &Assets,
    background: &[Texture2D],
    pos: GridPos,
    origin: Vec2,
    cell: f32,
    margins: bool,
) {
    let water = matches!(
        scene.document.floors[(pos.y * scene.document.width + pos.x) as usize],
        Some(8 | 9)
    );
    if water {
        assets.animation.begin(1.);
    }
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
    if water {
        assets.animation.end();
    }
    if !margins || !scene.is_structure(pos) {
        draw_paint(assets, pos, origin, cell);
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
    // Only the base floor borrows its neighbor's material at a wall margin.
    // Paint has absolute world coordinates; copying a neighbor's crop here
    // duplicated marks up to one whole cell away from the pointer.
    draw_paint(assets, pos, origin, cell);
}
