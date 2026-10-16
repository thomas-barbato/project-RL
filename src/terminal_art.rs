//! Orthographic native 32-pixel scenery, preserving the original footprints.
//! The atlas contains only presentation data; it never changes terrain or actors.
use macroquad::prelude::*;
use project_rl::world::GridPos;

use super::dim;
use crate::test_sector::Decor;

const SIDE: usize = 32;
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
        // The established outlines use a 16px design grid. Fine material and
        // hardware details below occupy actual single pixels in the 32px tile.
        self.fine_rect(x * 2, y * 2, w * 2, h * 2, color);
    }
    fn fine_rect(&mut self, x: i32, y: i32, w: i32, h: i32, color: u32) {
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
    fn bolt(&mut self, x: i32, y: i32) {
        self.fine_rect(x, y, 2, 2, 0x17252b);
        self.fine_rect(x, y, 1, 1, 0xaab9b7);
    }
    fn finish(&mut self, kind: Prop) {
        match kind {
            Prop::Server | Prop::Cabinet => {
                for (x, y) in [(8, 4), (23, 4), (8, 26), (23, 26)] {
                    self.bolt(x, y);
                }
                for y in [9, 13, 17] {
                    self.fine_rect(7, y, 3, 1, 0x627f85);
                    self.fine_rect(23, y, 2, 1, 0x172a30);
                }
                self.fine_rect(11, 25, 8, 1, 0x394c53);
                self.fine_rect(22, 24, 1, 1, 0xe0edd2);
                if matches!(kind, Prop::Cabinet) {
                    self.fine_rect(11, 13, 10, 1, 0xb2c1ac);
                    self.fine_rect(20, 15, 2, 2, 0x34483f);
                } else {
                    self.fine_rect(14, 9, 3, 1, 0x8fc1bb);
                    self.fine_rect(10, 17, 1, 3, 0x8fc1bb);
                    self.fine_rect(19, 18, 3, 1, 0x8fc1bb);
                }
            }
            Prop::Terminal | Prop::TerminalOff | Prop::TerminalUsed => {
                self.fine_rect(8, 8, 5, 1, 0x657f7d);
                self.fine_rect(16, 8, 7, 1, 0x425b59);
                for y in [17, 20, 23] {
                    for x in [8, 11, 14, 17] {
                        self.fine_rect(x, y, 2, 1, 0x929f96);
                    }
                }
                self.fine_rect(23, 20, 2, 1, 0xf2deb0);
                self.bolt(5, 24);
                self.bolt(26, 5);
            }
            Prop::Tank | Prop::Barrel => {
                let green = matches!(kind, Prop::Tank);
                self.fine_rect(11, 6, 10, 1, if green { 0xa0b798 } else { 0xc798b8 });
                self.fine_rect(9, 23, 14, 1, if green { 0x334b35 } else { 0x54364e });
                self.fine_rect(11, 14, 8, 1, if green { 0xc5d8b4 } else { 0xd7b3c9 });
                self.fine_rect(20, 9, 1, 3, 0x121f25);
                self.fine_rect(21, 10, 1, 1, 0x849b99);
            }
            Prop::Crate => {
                for (x, y) in [(8, 8), (23, 8), (8, 23), (23, 23)] {
                    self.bolt(x, y);
                }
                for (x, y, w) in [(10, 10, 3), (19, 11, 4), (8, 21, 5), (20, 22, 3)] {
                    self.fine_rect(x, y, w, 1, 0x777347);
                }
                self.fine_rect(15, 7, 1, 18, 0xc2b87b);
                self.fine_rect(7, 15, 18, 1, 0xc2b87b);
                self.fine_rect(14, 14, 4, 4, 0x414333);
                self.fine_rect(15, 15, 2, 1, 0xaba66f);
            }
            Prop::Vent => {
                for y in [9, 15, 21] {
                    self.fine_rect(8, y, 16, 1, 0xc0c7cb);
                    self.fine_rect(9, y + 2, 14, 1, 0x111d28);
                }
                for (x, y) in [(6, 6), (24, 6), (6, 24), (24, 24)] {
                    self.bolt(x, y);
                }
            }
            Prop::DoorOpen | Prop::DoorClosed | Prop::DoorLocked | Prop::DoorOff => {
                for y in [5, 25] {
                    self.bolt(4, y);
                    self.bolt(27, y);
                }
                self.fine_rect(5, 13, 1, 2, 0xdddfbb);
                if !matches!(kind, Prop::DoorOpen) {
                    self.fine_rect(11, 6, 1, 5, 0xa3afb3);
                    self.fine_rect(20, 17, 1, 6, 0x25343e);
                    self.fine_rect(9, 25, 5, 1, 0x9a9990);
                    self.fine_rect(18, 25, 5, 1, 0x9a9990);
                    self.fine_rect(14, 14, 1, 5, 0x111e28);
                    self.fine_rect(17, 14, 1, 5, 0x85979e);
                }
            }
        }
    }
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
    p.finish(kind);
    p
}

thread_local! {
    static ATLAS: Texture2D = {
        let count = PROPS.len();
        let mut image = Image::gen_image_color(
            (count * SIDE) as u16, SIDE as u16, Color::new(0., 0., 0., 0.),
        );
        for index in 0..count {
            let raster = prop(PROPS[index]);
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
    // Native 32px pixels remain crisp; other chosen zooms fill the same cell.
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
    draw(rect, art as usize, dim(WHITE, visible), rotation);
    true
}

pub(super) fn draw_wall(
    rect: Rect,
    kind: Decor,
    joins: [bool; 4],
    visible: bool,
    at: GridPos,
) -> bool {
    if kind == Decor::Wall {
        super::terminal_materials::wall(rect, joins, visible, at);
        return true;
    }
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
        if s >= 28. {
            paint(2., t + 13., s - 4., 1., 0x333640);
            paint(2., t + 14., s - 4., 1., 0x1b1e25);
        }
    } else {
        paint(l + 4., 2., 1., s - 4., 0x3e414c);
        paint(l + 7., 2., 1., s - 4., 0x3e414c);
        if s >= 28. {
            paint(l + 13., 2., 1., s - 4., 0x333640);
            paint(l + 14., 2., 1., s - 4., 0x1b1e25);
        }
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
    if let Some(lamp) = wall_lamp_rect(s, kind, joins, at) {
        // Fixed neutral lighting belongs to the wall material, never to the interface theme.
        paint(lamp.x, lamp.y, lamp.w, lamp.h, 0xadcbd0);
    }
    true
}

pub(super) fn wall_lamp_rect(s: f32, kind: Decor, joins: [bool; 4], at: GridPos) -> Option<Rect> {
    let [north, east, south, west] = joins;
    let horizontal = east || west;
    if kind != Decor::Wall || (if horizontal { at.x } else { at.y }).rem_euclid(4) != 1 {
        return None;
    }
    Some(if horizontal {
        Rect::new(
            5.,
            if south {
                if north { 3. } else { 4. }
            } else {
                s - 4.
            },
            (s - 10.).max(3.),
            1.,
        )
    } else {
        Rect::new(
            if east {
                if west { 3. } else { 4. }
            } else {
                s - 3.
            },
            5.,
            1.,
            (s - 10.).max(3.),
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn open_door_preserves_a_clear_passage_and_closed_states_fill_it() {
        let open = prop(Prop::DoorOpen);
        for y in 0..SIDE {
            for x in 8..24 {
                assert_eq!(open.pixels[y * SIDE + x][3], 0);
            }
        }
        for kind in [Prop::DoorClosed, Prop::DoorLocked, Prop::DoorOff] {
            let closed = prop(kind);
            for y in 0..SIDE {
                for x in 8..24 {
                    assert_eq!(closed.pixels[y * SIDE + x][3], 255);
                }
            }
        }
    }
}
