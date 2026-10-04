//! Continuous, world-anchored floor grain. Generated once; no simulation RNG.
use macroquad::prelude::*;
use project_rl::world::GridPos;

use super::dim;
use crate::test_sector::Decor;

const PATCH: i32 = 256;
const CELL: i32 = 32;
const COLUMNS: i32 = 5;
const ATLAS: i32 = PATCH * COLUMNS;

#[derive(Clone, Copy)]
enum Surface {
    Deck,
    Street,
    Lane,
    Grate,
    Concrete,
    Gravel,
    Grass,
    Scrub,
    Mud,
    Sidewalk,
    ShallowWater,
    DeepWater,
    Foreign,
    Veined,
    Chitin,
    Pulse,
    Memory,
    Window,
    Fault,
    Network,
    Wall,
    Membrane,
    Void,
    DeadScreen,
    Rubble,
}

const SURFACES: [Surface; 25] = [
    Surface::Deck,
    Surface::Street,
    Surface::Lane,
    Surface::Grate,
    Surface::Concrete,
    Surface::Gravel,
    Surface::Grass,
    Surface::Scrub,
    Surface::Mud,
    Surface::Sidewalk,
    Surface::ShallowWater,
    Surface::DeepWater,
    Surface::Foreign,
    Surface::Veined,
    Surface::Chitin,
    Surface::Pulse,
    Surface::Memory,
    Surface::Window,
    Surface::Fault,
    Surface::Network,
    Surface::Wall,
    Surface::Membrane,
    Surface::Void,
    Surface::DeadScreen,
    Surface::Rubble,
];

impl Surface {
    fn of(kind: Decor) -> Self {
        match kind {
            Decor::Street
            | Decor::TransitSign
            | Decor::StreetDrain
            | Decor::Hydrant
            | Decor::RoadBollard => Self::Street,
            Decor::Lane | Decor::Threshold => Self::Lane,
            Decor::Grate => Self::Grate,
            Decor::RuinFloor => Self::Concrete,
            Decor::Gravel => Self::Gravel,
            Decor::Grass | Decor::Tree | Decor::UrbanTree | Decor::Planter => Self::Grass,
            Decor::Scrub => Self::Scrub,
            Decor::Mud => Self::Mud,
            Decor::Wall => Self::Wall,
            Decor::RuinWall | Decor::Barricade | Decor::Boulder => Self::Rubble,
            Decor::ShallowWater => Self::ShallowWater,
            Decor::DeepWater => Self::DeepWater,
            Decor::ForeignFloor => Self::Foreign,
            Decor::VeinedFloor => Self::Veined,
            Decor::MembraneWall => Self::Membrane,
            Decor::ChitinFloor => Self::Chitin,
            Decor::PulseChannel => Self::Pulse,
            Decor::VoidWall => Self::Void,
            Decor::MemoryFloor => Self::Memory,
            Decor::WindowFrame => Self::Window,
            Decor::FaultTrace => Self::Fault,
            Decor::DeadScreen => Self::DeadScreen,
            Decor::NetworkTrace(_) => Self::Network,
            _ => Self::Deck,
        }
    }

    fn base(self) -> [u8; 3] {
        match self {
            Self::Deck => [18, 31, 38],
            Self::Street => [29, 32, 33],
            Self::Lane => [29, 32, 33],
            Self::Grate => [20, 34, 39],
            Self::Concrete => [43, 42, 39],
            Self::Gravel => [27, 31, 30],
            Self::Grass => [24, 44, 33],
            Self::Scrub => [47, 45, 29],
            Self::Mud => [48, 37, 31],
            Self::Sidewalk => [55, 57, 54],
            Self::ShallowWater => [22, 61, 70],
            Self::DeepWater => [12, 34, 54],
            Self::Foreign => [25, 21, 43],
            Self::Veined => [24, 39, 48],
            Self::Chitin => [48, 39, 27],
            Self::Pulse => [57, 22, 30],
            Self::Memory => [8, 12, 13],
            Self::Window => [10, 15, 16],
            Self::Fault => [19, 8, 9],
            Self::Network => [23, 34, 35],
            Self::Wall => [61, 78, 84],
            Self::Membrane => [71, 43, 86],
            Self::Void => [52, 35, 28],
            Self::DeadScreen => [3, 5, 6],
            Self::Rubble => [50, 53, 50],
        }
    }
}

thread_local! {
    // One atlas keeps all floor cells in the same render batch, at every zoom.
    static FLOOR_ATLAS: Texture2D = build_atlas();
}

fn hash(x: i32, y: i32, seed: u32) -> f32 {
    let mut value =
        (x as u32).wrapping_mul(0x9e37_79b9) ^ (y as u32).wrapping_mul(0x85eb_ca6b) ^ seed;
    value = (value ^ (value >> 16)).wrapping_mul(0x7feb_352d);
    value = (value ^ (value >> 15)).wrapping_mul(0x846c_a68b);
    (value ^ (value >> 16)) as f32 / u32::MAX as f32
}

fn noise(x: i32, y: i32, spacing: i32, seed: u32) -> f32 {
    let period = PATCH / spacing;
    let cx = x / spacing;
    let cy = y / spacing;
    let smooth = |v: f32| v * v * (3.0 - 2.0 * v);
    let tx = smooth((x % spacing) as f32 / spacing as f32);
    let ty = smooth((y % spacing) as f32 / spacing as f32);
    let sample = |dx, dy| hash((cx + dx) % period, (cy + dy) % period, seed);
    let top = sample(0, 0) * (1.0 - tx) + sample(1, 0) * tx;
    let bottom = sample(0, 1) * (1.0 - tx) + sample(1, 1) * tx;
    top * (1.0 - ty) + bottom * ty
}

fn pixel(surface: Surface, x: i32, y: i32) -> [u8; 4] {
    let seed = (surface as u32 + 1) * 139;
    let broad = noise(x, y, 64, seed) - 0.5;
    let wear = noise(x, y, 16, seed + 1) - 0.5;
    let grain = hash(x, y, seed + 2) - 0.5;
    let mut shade = broad * 12.0 + wear * 8.0 + grain * 5.0;
    let mut tint = [0.0; 3];
    match surface {
        Surface::Deck => {
            // Sparse brushed scratches, with no plate border at cell edges.
            if hash(x / 16, y, seed + 3) > 0.988 && grain > -0.15 {
                shade += 9.0;
            }
        }
        Surface::Street
        | Surface::Lane
        | Surface::Concrete
        | Surface::Sidewalk
        | Surface::Rubble => {
            if matches!(surface, Surface::Street | Surface::Lane) {
                shade += grain * 6.0;
            }
            if grain > 0.485 {
                shade += 10.0;
            }
            // Broken, irregular fractures continue across cells. A second
            // field interrupts their branches instead of making closed loops.
            if matches!(
                surface,
                Surface::Concrete | Surface::Sidewalk | Surface::Rubble
            ) && (noise(x, y, 64, seed + 4) + (noise(x, y, 16, seed + 5) - 0.5) * 0.22 - 0.5)
                .abs()
                < 0.005
                && noise(x, y, 32, seed + 6) > 0.52
            {
                shade -= 13.0;
            }
            if matches!(surface, Surface::Sidewalk) {
                shade += wear * 7.0;
                if grain < -0.47 {
                    shade -= 8.0;
                }
            }
        }
        Surface::Grate => {
            let slot = y % 8;
            shade += if slot == 0 {
                -8.0
            } else if slot == 1 {
                7.0
            } else {
                0.0
            };
            if x % 64 == 0 {
                shade += 4.0;
            }
        }
        Surface::Gravel => {
            let pebble = hash(x / 2, y / 2, seed + 3);
            if pebble > 0.86 {
                shade += 15.0 + grain * 8.0;
            }
        }
        Surface::Grass | Surface::Scrub => {
            let tuft = hash(x, y / 3, seed + 3);
            if tuft > 0.94 {
                tint = if matches!(surface, Surface::Grass) {
                    [8.0, 18.0, 5.0]
                } else {
                    [19.0, 16.0, 5.0]
                };
            }
            shade += wear * 10.0;
        }
        Surface::Mud => {
            if noise(x, y, 16, seed + 3) > 0.68 {
                tint = [-6.0, 3.0, 5.0];
            }
            shade += broad * 8.0;
        }
        Surface::ShallowWater | Surface::DeepWater => {
            shade = broad * 20.0 + wear * 8.0 + grain * 2.0;
            // Ripples meander through the atlas; there is no per-cell water icon.
            let wave =
                (y as f32 * std::f32::consts::TAU / 32.0 + noise(x, y, 64, seed + 3) * 9.0).sin();
            if wave > 0.96 {
                tint = if matches!(surface, Surface::ShallowWater) {
                    [13.0, 34.0, 33.0]
                } else {
                    [4.0, 14.0, 23.0]
                };
            }
            if matches!(surface, Surface::ShallowWater) && hash(x / 3, y / 3, seed + 4) > 0.98 {
                tint = [6.0, 10.0, 5.0];
            }
        }
        Surface::Foreign
        | Surface::Veined
        | Surface::Chitin
        | Surface::Pulse
        | Surface::Membrane
        | Surface::Void => {
            let field = noise(x, y, 64, seed + 4);
            shade += wear * 14.0;
            let ridge = (field - 0.5).abs();
            if ridge < 0.018 {
                tint = match surface {
                    Surface::Foreign => [22.0, 11.0, 32.0],
                    Surface::Veined => [14.0, 54.0, 45.0],
                    Surface::Chitin => [24.0, 17.0, 6.0],
                    Surface::Pulse => [47.0, 11.0, 19.0],
                    Surface::Membrane => [22.0, 7.0, 31.0],
                    _ => [-15.0, -12.0, -9.0],
                };
            } else if ridge < 0.029 {
                shade -= 5.0;
            }
            if matches!(surface, Surface::Chitin) {
                shade += broad * 18.0;
            }
        }
        Surface::Memory
        | Surface::Window
        | Surface::Fault
        | Surface::Network
        | Surface::DeadScreen => {
            shade *= 0.4;
            // Short, sparse etched traces, independent of the game's cell boundaries.
            let trace = hash(x / 16, y / 4, seed + 3);
            if trace > 0.94 && y % 4 == 0 && x % 16 < 11 {
                tint = if matches!(surface, Surface::Fault) {
                    [48.0, 8.0, 6.0]
                } else {
                    [10.0, 25.0, 23.0]
                };
            }
        }
        Surface::Wall => {
            shade += wear * 6.0;
            if hash(x / 12, y, seed + 3) > 0.99 {
                shade += 9.0;
            }
            if grain < -0.48 {
                tint = [10.0, -1.0, -6.0];
            }
        }
    }
    let base = surface.base();
    let channel = |i: usize| (base[i] as f32 + shade + tint[i]).round().clamp(0.0, 255.0) as u8;
    [channel(0), channel(1), channel(2), 255]
}

fn build_atlas() -> Texture2D {
    let mut image = Image::gen_image_color(ATLAS as u16, ATLAS as u16, BLACK);
    for surface in SURFACES {
        let origin_x = surface as i32 % COLUMNS * PATCH;
        let origin_y = surface as i32 / COLUMNS * PATCH;
        for y in 0..PATCH {
            for x in 0..PATCH {
                let offset = ((origin_y + y) * ATLAS + origin_x + x) as usize * 4;
                image.bytes[offset..offset + 4].copy_from_slice(&pixel(surface, x, y));
            }
        }
    }
    let texture = Texture2D::from_image(&image);
    texture.set_filter(FilterMode::Nearest);
    texture
}

#[cfg(debug_assertions)]
pub(super) fn remembered_center_color(position: GridPos) -> [f32; 3] {
    let x = position.x.rem_euclid(PATCH / CELL) * CELL + CELL / 2;
    let y = position.y.rem_euclid(PATCH / CELL) * CELL + CELL / 2;
    let [r, g, b, a] = pixel(Surface::Deck, x, y);
    let color = dim(Color::from_rgba(r, g, b, a), false);
    [color.r, color.g, color.b]
}

#[cfg(debug_assertions)]
pub(super) fn visible_sample_color(position: GridPos, fraction: Vec2) -> [f32; 3] {
    let x = position.x.rem_euclid(PATCH / CELL) * CELL
        + (fraction.x * CELL as f32)
            .floor()
            .clamp(0.0, (CELL - 1) as f32) as i32;
    let y = position.y.rem_euclid(PATCH / CELL) * CELL
        + (fraction.y * CELL as f32)
            .floor()
            .clamp(0.0, (CELL - 1) as f32) as i32;
    let [r, g, b, _] = pixel(Surface::Deck, x, y);
    [
        f32::from(r) / 255.0,
        f32::from(g) / 255.0,
        f32::from(b) / 255.0,
    ]
}

pub(super) fn draw(rect: Rect, kind: Decor, visible: bool, position: GridPos) {
    draw_surface(rect, Surface::of(kind), visible, position);
}

pub(super) fn draw_sidewalk(rect: Rect, visible: bool, position: GridPos) {
    draw_surface(rect, Surface::Sidewalk, visible, position);
}

fn draw_surface(rect: Rect, surface: Surface, visible: bool, position: GridPos) {
    let origin_x = surface as i32 % COLUMNS * PATCH;
    let origin_y = surface as i32 / COLUMNS * PATCH;
    let x = position.x.rem_euclid(PATCH / CELL) * CELL;
    let y = position.y.rem_euclid(PATCH / CELL) * CELL;
    FLOOR_ATLAS.with(|texture| {
        draw_texture_ex(
            texture,
            rect.x,
            rect.y,
            dim(WHITE, visible),
            DrawTextureParams {
                dest_size: Some(rect.size()),
                source: Some(Rect::new(
                    (origin_x + x) as f32,
                    (origin_y + y) as f32,
                    CELL as f32,
                    CELL as f32,
                )),
                ..Default::default()
            },
        );
    });
}
