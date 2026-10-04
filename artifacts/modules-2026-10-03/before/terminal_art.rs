//! Orthographic 16-pixel symbols. Every drawing occupies one existing map cell.
//! The atlas contains only presentation data; it never changes terrain or actors.
use macroquad::prelude::*;
use project_rl::world::GridPos;

use super::dim;
use crate::test_sector::Decor;

const SIDE: usize = 16;
const ACTORS: &[char] = &[
    '@', 'c', 'm', 'v', 'h', 'i', 'u', 'd', 't', 'r', 'j', 'w', 'B', 'V', 'G', 'N', 'K', 'W', 'J',
    'Q', 'P', 'U', 'Z', 'O', 'C', 'D', 'F', 'H', 'I', 'X', 'Y', 'L', 'A', 'E',
];

#[derive(Clone, Copy)]
#[repr(usize)]
enum Prop {
    Server,
    Cabinet,
    Terminal,
    TerminalOff,
    TerminalUsed,
    Tank,
    Barrel,
    Crate,
    Vent,
    DoorOpen,
    DoorClosed,
    DoorLocked,
    DoorOff,
}
const PROPS: [Prop; 13] = [
    Prop::Server,
    Prop::Cabinet,
    Prop::Terminal,
    Prop::TerminalOff,
    Prop::TerminalUsed,
    Prop::Tank,
    Prop::Barrel,
    Prop::Crate,
    Prop::Vent,
    Prop::DoorOpen,
    Prop::DoorClosed,
    Prop::DoorLocked,
    Prop::DoorOff,
];

struct Raster {
    pixels: [[u8; 4]; SIDE * SIDE],
}
impl Raster {
    fn new() -> Self {
        Self {
            pixels: [[0; 4]; SIDE * SIDE],
        }
    }
    fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
        let rgba = [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255];
        for py in y.max(0)..(y + h).min(SIDE as i32) {
            for px in x.max(0)..(x + w).min(SIDE as i32) {
                self.pixels[py as usize * SIDE + px as usize] = rgba;
            }
        }
    }
    fn disk(&mut self, cx: i32, cy: i32, radius: i32, color: u32) {
        for y in -radius..=radius {
            let span = ((radius * radius - y * y) as f32).sqrt().floor() as i32;
            self.rect(cx - span, cy + y, span * 2 + 1, 1, color);
        }
    }
    fn roof(&mut self, x: i32, y: i32, w: i32, h: i32, base: u32, light: u32, shade: u32) {
        self.rect(x + 2, y, w - 4, h, base);
        self.rect(x, y + 2, w, h - 4, base);
        self.rect(x + 2, y, w - 4, 1, light);
        self.rect(x, y + 2, 1, h - 4, light);
        self.rect(x + 2, y + h - 1, w - 4, 1, shade);
        self.rect(x + w - 1, y + 2, 1, h - 4, shade);
    }
    fn fan(&mut self, cx: i32, cy: i32, radius: i32) {
        self.disk(cx, cy, radius, 0x718a92);
        self.disk(cx, cy, radius - 1, 0x15262f);
        for (x, y, w, h) in [(-3, -2, 3, 2), (1, -3, 2, 3), (1, 1, 3, 2), (-2, 1, 2, 3)] {
            self.rect(cx + x, cy + y, w, h, 0x71a5a2);
        }
        self.disk(cx, cy, 1, 0xbde0d7);
    }
}

fn actor(symbol: char) -> Raster {
    let mut p = Raster::new();
    if matches!(
        symbol,
        'B' | 'V'
            | 'G'
            | 'N'
            | 'K'
            | 'W'
            | 'J'
            | 'Q'
            | 'U'
            | 'Z'
            | 'O'
            | 'C'
            | 'D'
            | 'F'
            | 'H'
            | 'I'
    ) {
        let (base, light, shade) = match symbol {
            'G' | 'N' => (0xa39b7b, 0xd3cea5, 0x4d513f),
            'V' | 'H' | 'Z' => (0x908daa, 0xc4bbdd, 0x393747),
            'W' | 'C' => (0x8d9e72, 0xc1d49b, 0x3a5140),
            'J' | 'O' => (0xa07399, 0xdb9bbb, 0x463044),
            'I' => (0x80a9a1, 0xb9e3ce, 0x304a4b),
            _ => (0xb07672, 0xe5b799, 0x573841),
        };
        match symbol {
            'W' => {
                for (x, y) in [(6, 3), (9, 5), (8, 8), (5, 10), (7, 13)] {
                    p.disk(x, y, 2, shade);
                    p.disk(x, y - 1, 1, base);
                    p.rect(x, y - 2, 1, 1, light);
                }
            }
            'J' | 'Q' => {
                for (x, y, w, h) in [
                    (7, 1, 2, 14),
                    (1, 7, 14, 2),
                    (3, 3, 2, 2),
                    (11, 3, 2, 2),
                    (3, 11, 2, 2),
                    (11, 11, 2, 2),
                ] {
                    p.rect(x, y, w, h, base);
                }
                p.disk(8, 8, 4, shade);
                p.disk(8, 8, 3, base);
                p.disk(8, 7, 1, light);
            }
            'H' => {
                for (x, y, w, h) in [(1, 5, 5, 2), (0, 7, 6, 3), (10, 5, 5, 2), (10, 7, 6, 3)] {
                    p.rect(x, y, w, h, base);
                }
                p.roof(6, 4, 4, 9, base, light, shade);
                p.rect(7, 3, 2, 2, light);
            }
            _ => {
                let heavy = matches!(symbol, 'G' | 'K' | 'D');
                p.roof(3, 5, if heavy { 10 } else { 9 }, 8, base, light, shade);
                p.disk(8, 4, if heavy { 3 } else { 2 }, shade);
                p.disk(8, 3, 2, base);
                p.rect(7, 2, 2, 1, light);
                for (x, y) in [(1, 6), (12, 6), (2, 12), (11, 12)] {
                    p.rect(x, y, 3, 2, shade);
                    p.rect(x, y, 2, 1, light);
                }
                p.rect(7, 6, 2, 5, shade);
                if matches!(symbol, 'B' | 'K' | 'D') {
                    p.rect(5, 2, 1, 2, light);
                    p.rect(11, 2, 1, 2, light);
                }
                if matches!(symbol, 'N' | 'V' | 'F') {
                    p.rect(6, 13, 2, 2, base);
                    p.rect(5, 14, 2, 1, light);
                }
                if symbol == 'C' {
                    for y in [6, 9, 12] {
                        p.rect(3, y, 10, 1, light);
                    }
                }
                if matches!(symbol, 'O' | 'I') {
                    p.disk(10, 9, 2, 0x99d7b1);
                    p.rect(9, 8, 1, 1, 0xe0f0c2);
                }
                if symbol == 'Z' {
                    p.rect(4, 6, 1, 6, light);
                    p.rect(11, 6, 1, 6, light);
                }
            }
        }
        return p;
    }
    if matches!(symbol, 'u' | 'P' | 'Y') {
        p.disk(8, 8, 6, 0x26363e);
        p.disk(8, 8, 5, 0x99aab1);
        p.disk(8, 8, 3, 0x435c63);
        for (x, y) in [(1, 2), (12, 2), (1, 12), (12, 12)] {
            p.rect(x, y, 3, 2, 0x82939e);
            p.rect(x, y, 2, 1, 0xc9d9d7);
        }
        if symbol == 'P' {
            p.rect(7, 0, 2, 8, 0xb6b9aa);
            p.rect(7, 0, 1, 7, 0xe8ddbf);
        } else {
            p.rect(6, 6, 4, 3, 0x91d8d0);
        }
        return p;
    }
    let (base, light, shade) = match symbol {
        '@' => (0xbcc9cf, 0xeff8f6, 0x5f7180),
        'm' => (0xb2a377, 0xede2af, 0x655d46),
        'v' => (0xb9a47c, 0xf1ddb0, 0x675645),
        'h' | 'w' => (0x82ada7, 0xd4efe2, 0x385553),
        'c' | 'i' => (0x98ad85, 0xd0dcad, 0x4a604c),
        'X' | 'L' | 'A' | 'E' => (0xb49c82, 0xedddac, 0x63544b),
        _ => (0xb96177, 0xf9a4a3, 0x653247),
    };
    p.rect(2, 7, 4, 4, shade);
    p.rect(2, 7, 4, 1, light);
    p.rect(3, 8, 3, 2, base);
    p.rect(11, 7, 3, 4, shade);
    p.rect(11, 7, 3, 1, light);
    p.rect(11, 8, 2, 2, base);
    p.roof(5, 7, 7, 6, base, light, shade);
    p.disk(8, 5, 3, shade);
    p.disk(8, 4, 2, base);
    p.rect(7, 2, 2, 1, light);
    p.rect(6, 9, 4, 3, shade);
    p.rect(6, 9, 4, 1, light);
    p.rect(5, 13, 2, 1, 0x3e4c5a);
    p.rect(10, 12, 2, 2, 0x3e4c5a);
    match symbol {
        '@' => {
            p.rect(14, 2, 1, 7, 0x91ecf5);
            p.rect(13, 8, 2, 2, shade);
        }
        'm' => {
            p.rect(13, 5, 2, 5, 0x79858a);
            p.rect(12, 5, 4, 1, 0xd5dde1);
            p.rect(6, 10, 4, 2, 0x657d69);
        }
        'h' | 'w' => {
            p.rect(5, 10, 6, 2, 0x47665b);
            p.rect(7, 9, 2, 4, 0xdfefe1);
            p.rect(6, 10, 4, 2, 0xdfefe1);
        }
        'v' => {
            p.rect(2, 10, 3, 3, 0x736446);
            p.rect(2, 10, 3, 1, 0xe3ce98);
        }
        'c' => {
            p.rect(5, 9, 6, 3, 0x394b45);
            p.rect(6, 9, 4, 1, 0x9fc998);
        }
        'i' => {
            p.rect(4, 9, 8, 4, shade);
            p.rect(4, 9, 8, 1, light);
        }
        'd' => {
            p.rect(3, 5, 2, 7, 0x864559);
            p.rect(3, 5, 1, 6, light);
        }
        'r' => {
            p.rect(13, 1, 2, 10, 0x8e7c87);
            p.rect(13, 1, 1, 9, 0xe2ced0);
        }
        'j' => {
            p.roof(3, 9, 9, 5, 0x493745, 0xad748b, shade);
            p.rect(12, 0, 3, 10, 0x90858d);
            p.rect(12, 0, 1, 10, 0xe9c9c6);
        }
        't' | 'L' | 'A' => {
            p.rect(1, 7, 4, 6, base);
            p.rect(1, 7, 4, 1, light);
            p.rect(13, 2, 2, 8, 0xb8aeb0);
        }
        'E' => {
            p.rect(1, 8, 3, 2, light);
            p.rect(12, 11, 3, 2, light);
        }
        'X' => {
            p.rect(7, 7, 2, 6, 0xdccb93);
        }
        _ => {}
    }
    p
}

fn prop(kind: Prop) -> Raster {
    let mut p = Raster::new();
    match kind {
        Prop::Server | Prop::Cabinet => {
            p.roof(2, 1, 12, 14, 0x34464d, 0x9db9bc, 0x14232b);
            p.fan(8, 7, 5);
            p.rect(4, 12, 6, 1, 0x758f91);
            p.rect(11, 12, 1, 1, 0xbbf4b1);
            p.rect(12, 3, 1, 1, 0x90c9c9);
            if matches!(kind, Prop::Cabinet) {
                p.rect(5, 6, 6, 3, 0x889f92);
                p.rect(7, 7, 2, 1, 0xd3c799);
            }
        }
        Prop::Terminal | Prop::TerminalOff | Prop::TerminalUsed => {
            p.roof(1, 2, 14, 12, 0x3c474a, 0x99b0b2, 0x1d292e);
            p.rect(3, 3, 10, 3, 0x152326);
            p.rect(4, 3, 8, 1, 0x728d8e);
            p.rect(
                4,
                5,
                8,
                1,
                match kind {
                    Prop::TerminalOff => 0x52656a,
                    Prop::TerminalUsed => 0x8ea999,
                    _ => 0xa9d6bd,
                },
            );
            p.rect(3, 8, 7, 4, 0x1b2429);
            for x in [4, 6, 8] {
                p.rect(x, 8, 1, 1, 0xc1c6b7);
                p.rect(x, 10, 1, 1, 0x8d9995);
            }
            p.disk(12, 10, 1, 0xd3c695);
            p.rect(6, 12, 3, 1, 0x85918a);
        }
        Prop::Tank | Prop::Barrel => {
            let green = matches!(kind, Prop::Tank);
            p.disk(8, 8, 6, if green { 0x335045 } else { 0x67405c });
            p.disk(8, 8, 5, if green { 0x7e9b77 } else { 0xaa709b });
            p.disk(8, 8, 4, if green { 0x526e4b } else { 0x76516e });
            p.rect(5, 7, 6, 2, if green { 0xafc7a0 } else { 0xbf93b4 });
            p.disk(10, 5, 1, 0x324048);
            p.rect(10, 4, 1, 1, 0xd4dcd4);
        }
        Prop::Crate => {
            p.roof(2, 2, 12, 12, 0x716b4a, 0xc4b77b, 0x353527);
            p.rect(4, 4, 8, 8, 0x5a593e);
            p.rect(7, 3, 2, 10, 0xa7a06c);
            p.rect(3, 7, 10, 2, 0xa7a06c);
            p.rect(7, 7, 2, 2, 0x444837);
        }
        Prop::Vent => {
            p.roof(2, 2, 12, 12, 0x424751, 0xadb5be, 0x1e252e);
            for y in [4, 7, 10] {
                p.rect(4, y, 8, 2, 0x212b36);
                p.rect(4, y, 8, 1, 0x85919e);
            }
        }
        Prop::DoorOpen | Prop::DoorClosed | Prop::DoorLocked | Prop::DoorOff => {
            p.rect(0, 0, 4, 16, 0x353f49);
            p.rect(0, 0, 1, 16, 0xa0adb4);
            p.rect(3, 1, 1, 14, 0x10191f);
            p.rect(12, 0, 4, 16, 0x303a44);
            p.rect(12, 0, 1, 16, 0x7d919c);
            p.rect(15, 0, 1, 16, 0x111a22);
            for y in [2, 12] {
                p.rect(1, y, 2, 2, 0x141c24);
                p.rect(1, y, 1, 1, 0xacb9bd);
                p.rect(13, y, 2, 2, 0x141c24);
                p.rect(13, y, 1, 1, 0x83959e);
            }
            let indicator = match kind {
                Prop::DoorOpen => 0xa5d1b7,
                Prop::DoorLocked => 0xee9877,
                Prop::DoorOff => 0x63717a,
                _ => 0xd6b978,
            };
            if !matches!(kind, Prop::DoorOpen) {
                p.rect(4, 0, 8, 16, 0x5a646c);
                p.rect(4, 0, 8, 1, 0xb3bec2);
                p.rect(4, 15, 8, 1, 0x212c34);
                p.rect(7, 1, 2, 14, 0x202d35);
                p.rect(6, 2, 1, 11, 0x8c999e);
                p.rect(9, 2, 1, 11, 0x3c4b56);
                p.rect(4, 4, 3, 1, 0x73828c);
                p.rect(9, 4, 3, 1, 0x73828c);
                p.rect(4, 10, 3, 1, 0x394853);
                p.rect(9, 10, 3, 1, 0x394853);
                for x in [4, 6, 9, 11] {
                    p.rect(x, 13, 1, 2, indicator);
                }
                if matches!(kind, Prop::DoorLocked) {
                    p.rect(6, 6, 4, 4, 0x382b30);
                    p.rect(7, 7, 2, 2, indicator);
                }
                if matches!(kind, Prop::DoorOff) {
                    p.rect(5, 5, 6, 1, 0x87919a);
                    p.rect(5, 10, 6, 1, 0x87919a);
                }
            }
            p.rect(2, 6, 1, 3, indicator);
            p.rect(13, 6, 1, 3, indicator);
        }
    }
    p
}

thread_local! {
    static ATLAS: Texture2D = {
        let count = ACTORS.len() + PROPS.len();
        let mut image = Image::gen_image_color(
            (count * SIDE) as u16, SIDE as u16, Color::new(0., 0., 0., 0.),
        );
        for index in 0..count {
            let raster = if index < ACTORS.len() {
                actor(ACTORS[index])
            } else {
                prop(PROPS[index - ACTORS.len()])
            };
            for (offset, rgba) in raster.pixels.iter().enumerate() {
                let pixel = ((offset / SIDE) * count * SIDE + index * SIDE + offset % SIDE) * 4;
                image.bytes[pixel..pixel + 4].copy_from_slice(rgba);
            }
        }
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);
        texture
    };
}

fn draw(rect: Rect, index: usize, color: Color, rotation: f32) {
    // Fill the available cell at intermediate zooms too: rounding down to a
    // multiple of 16 made actors shrink abruptly on fitted 20–31 px cameras.
    let size = rect.w.min(rect.h);
    let x = (rect.x + (rect.w - size) * 0.5).floor();
    let y = (rect.y + (rect.h - size) * 0.5).floor();
    ATLAS.with(|texture| {
        draw_texture_ex(
            texture,
            x,
            y,
            color,
            DrawTextureParams {
                source: Some(Rect::new(
                    (index * SIDE) as f32,
                    0.,
                    SIDE as f32,
                    SIDE as f32,
                )),
                dest_size: Some(vec2(size, size)),
                rotation,
                pivot: Some(vec2(x + size * 0.5, y + size * 0.5)),
                ..Default::default()
            },
        )
    });
}

pub(super) fn draw_actor(rect: Rect, symbol: char, semantic: Color) -> bool {
    let Some(index) = ACTORS.iter().position(|&actor| actor == symbol) else {
        return false;
    };
    // Preserve observable colour cues without letting UI appearance recolour the world.
    let tint = Color::new(
        0.72 + semantic.r * 0.28,
        0.72 + semantic.g * 0.28,
        0.72 + semantic.b * 0.28,
        semantic.a,
    );
    draw(rect, index, tint, 0.);
    true
}

pub(super) fn draw_prop(rect: Rect, kind: Decor, joins: [bool; 4], visible: bool) -> bool {
    let art = match kind {
        Decor::Server => Prop::Server,
        Decor::ElectricalCabinet => Prop::Cabinet,
        Decor::Console | Decor::ControlReady | Decor::DataTerminalOnline => Prop::Terminal,
        Decor::DataTerminalOffline => Prop::TerminalOff,
        Decor::ControlUsed | Decor::DataTerminalUpdated => Prop::TerminalUsed,
        Decor::Coolant => Prop::Tank,
        Decor::Barrel => Prop::Barrel,
        Decor::Crate => Prop::Crate,
        Decor::VentUnit => Prop::Vent,
        Decor::DoorOpen => Prop::DoorOpen,
        Decor::DoorClosed => Prop::DoorClosed,
        Decor::DoorLocked => Prop::DoorLocked,
        Decor::DoorUnpowered => Prop::DoorOff,
        _ => return false,
    };
    let door = matches!(
        art,
        Prop::DoorOpen | Prop::DoorClosed | Prop::DoorLocked | Prop::DoorOff
    );
    let rotation = if door && joins[0] && joins[2] {
        std::f32::consts::FRAC_PI_2
    } else {
        0.
    };
    draw(
        rect,
        ACTORS.len() + art as usize,
        dim(WHITE, visible),
        rotation,
    );
    true
}

pub(super) fn draw_wall(
    rect: Rect,
    kind: Decor,
    joins: [bool; 4],
    visible: bool,
    at: GridPos,
) -> bool {
    if !matches!(
        kind,
        Decor::Wall | Decor::MembraneWall | Decor::VoidWall | Decor::DeadScreen
    ) {
        return false;
    }
    let [north, east, south, west] = joins;
    let (base, light, dark) = match kind {
        Decor::MembraneWall => (0x372c42, 0x8c759f, 0x18131e),
        Decor::VoidWall => (0x3b3430, 0x8b8072, 0x1c1716),
        Decor::DeadScreen => (0x15191d, 0x47535a, 0x090b0e),
        _ => (0x25272d, 0x848892, 0x15171c),
    };
    let color = |hex: u32| {
        dim(
            Color::from_rgba((hex >> 16) as u8, (hex >> 8) as u8, hex as u8, 255),
            visible,
        )
    };
    let paint = |x: f32, y: f32, w: f32, h: f32, ink: u32| {
        draw_rectangle(rect.x + x, rect.y + y, w.max(0.), h.max(0.), color(ink))
    };
    let s = rect.w;
    let l = if west { 0. } else { 1. };
    let t = if north { 0. } else { 1. };
    let r = if east { s } else { s - 1. };
    let b = if south { s } else { s - 1. };
    paint(l, t, r - l, b - t, base);
    if !north {
        paint(l, t, r - l, 2., 0x444851);
        paint(l + 1., t, r - l - 2., 1., light);
    }
    if !west {
        paint(l, t, 2., b - t, 0x444851);
        paint(l, t + 1., 1., b - t - 2., light);
    }
    if !south {
        paint(l, b - 4., r - l, 4., dark);
        paint(l + 1., b - 4., r - l - 2., 1., 0x5b606b);
    }
    if !east {
        paint(r - 3., t, 3., b - t, dark);
        paint(r - 3., t + 1., 1., b - t - 2., 0x5b606b);
    }
    let horizontal = east || west;
    if horizontal {
        paint(2., t + 4., s - 4., 1., 0x3e414c);
        paint(2., t + 7., s - 4., 1., 0x3e414c);
    } else {
        paint(l + 4., 2., 1., s - 4., 0x3e414c);
        paint(l + 7., 2., 1., s - 4., 0x3e414c);
    }
    if (if horizontal { at.x } else { at.y }).rem_euclid(3) == 0 {
        if horizontal {
            paint(1., t + 3., 1., b - t - 7., 0x0a0b0e);
        } else {
            paint(l + 3., 1., r - l - 6., 1., 0x0a0b0e);
        }
    }
    if (!north && !west) || (!south && !east) {
        paint(2., 2., 3., 3., 0x08090b);
        paint(3., 3., 1., 1., 0x8b909a);
    }
    if kind == Decor::Wall && (if horizontal { at.x } else { at.y }).rem_euclid(4) == 1 {
        // Fixed neutral lighting belongs to the wall material, never to the interface theme.
        if horizontal {
            paint(
                5.,
                if south { t + 3. } else { b - 3. },
                (s - 10.).max(3.),
                1.,
                0xadcbd0,
            );
        } else {
            paint(
                if east { l + 3. } else { r - 2. },
                5.,
                1.,
                (s - 10.).max(3.),
                0xadcbd0,
            );
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn open_door_preserves_a_clear_passage_and_closed_states_fill_it() {
        let open = prop(Prop::DoorOpen);
        for y in 0..SIDE {
            for x in 4..12 {
                assert_eq!(open.pixels[y * SIDE + x][3], 0);
            }
        }
        for kind in [Prop::DoorClosed, Prop::DoorLocked, Prop::DoorOff] {
            let closed = prop(kind);
            for y in 0..SIDE {
                for x in 4..12 {
                    assert_eq!(closed.pixels[y * SIDE + x][3], 255);
                }
            }
        }
    }
    #[test]
    fn every_actor_has_drawn_art_and_equipment_distinguishes_service_roles() {
        for &symbol in ACTORS {
            assert!(
                actor(symbol).pixels.iter().any(|pixel| pixel[3] > 0),
                "{symbol}"
            );
        }
        for a in ['@', 'm', 'v', 'h', 'c', 'i'] {
            for b in ['@', 'm', 'v', 'h', 'c', 'i'] {
                if a != b {
                    assert_ne!(actor(a).pixels, actor(b).pixels);
                }
            }
        }
    }
}
