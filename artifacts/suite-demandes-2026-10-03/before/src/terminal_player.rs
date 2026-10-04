//! Approved top-down modules, rasterized once into one nearest-filtered atlas.
//! Equipment and horizontal facing are presentation data, outside the save schema.
use macroquad::prelude::*;
use project_rl::combat::AttackDelivery;

const SIDE: usize = 20;
const POSES: [PlayerWeaponPose; 3] = [
    PlayerWeaponPose::Unarmed,
    PlayerWeaponPose::Melee,
    PlayerWeaponPose::Ranged,
];
const KINDS: [ModuleKind; 3] = [ModuleKind::Player, ModuleKind::Service, ModuleKind::Hostile];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum PlayerWeaponPose {
    Unarmed,
    Melee,
    Ranged,
}

impl PlayerWeaponPose {
    pub const fn from_delivery(delivery: Option<AttackDelivery>) -> Self {
        match delivery {
            None => Self::Unarmed,
            Some(AttackDelivery::Melee) => Self::Melee,
            Some(AttackDelivery::Ranged) => Self::Ranged,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PlayerAppearance {
    pub pose: PlayerWeaponPose,
    pub facing_left: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum ModuleKind {
    Player,
    Service,
    Hostile,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ActorAppearance {
    pub kind: ModuleKind,
    pub pose: PlayerWeaponPose,
    pub facing_left: bool,
}

impl From<PlayerAppearance> for ActorAppearance {
    fn from(player: PlayerAppearance) -> Self {
        Self {
            kind: ModuleKind::Player,
            pose: player.pose,
            facing_left: player.facing_left,
        }
    }
}

impl ModuleKind {
    pub const fn for_symbol(symbol: char) -> Option<Self> {
        match symbol {
            '@' => Some(ModuleKind::Player),
            'c' | 'm' | 'v' | 'h' | 'i' | 'u' | 'G' | 'N' | 'I' | 'E' => Some(ModuleKind::Service),
            'd' | 't' | 'r' | 'j' | 'w' | 'B' | 'V' | 'K' | 'W' | 'J' | 'Q' | 'P' | 'U' | 'Z'
            | 'O' | 'C' | 'D' | 'F' | 'H' | 'X' | 'Y' | 'L' | 'A' => Some(ModuleKind::Hostile),
            _ => None,
        }
    }
}

// Integer coordinates and colours reproduce the approved 20 px mockup.
// Transparent padding keeps the body and its attachment inside one cell.
fn module_pixels(kind: ModuleKind, pose: PlayerWeaponPose) -> [[u8; 4]; SIDE * SIDE] {
    let [light, middle, dark, core] = match kind {
        ModuleKind::Player => [0xeef0e6, 0x94a2ad, 0x41505c, 0x61e3e9],
        ModuleKind::Service => [0xb7e7ba, 0x6aa987, 0x3f6057, 0xdceeb3],
        ModuleKind::Hostile => [0xffa99a, 0xd66365, 0x723b49, 0xffd47c],
    };
    let mut pixels = [[0; 4]; SIDE * SIDE];
    let mut rect = |x: usize, y: usize, w: usize, h: usize, color: u32| {
        let rgba = [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255];
        for py in y..y + h {
            for px in x..x + w {
                pixels[py * SIDE + px] = rgba;
            }
        }
    };
    match kind {
        ModuleKind::Player => {
            rect(4, 4, 9, 13, light);
            rect(2, 6, 13, 9, light);
            rect(5, 7, 7, 7, dark);
            rect(7, 9, 3, 3, core);
            rect(4, 5, 3, 1, middle);
            rect(3, 6, 1, 3, middle);
        }
        ModuleKind::Service => {
            rect(3, 6, 12, 10, light);
            rect(5, 8, 8, 6, dark);
            rect(6, 9, 2, 4, core);
            rect(10, 9, 2, 4, core);
            rect(14, 10, 2, 2, core);
        }
        ModuleKind::Hostile => {
            for x in 3..=15 {
                let half = (15 - x) * 52 / 100;
                rect(x, 10 - half, 1, half * 2 + 1, light);
            }
            rect(6, 8, 2, 5, dark);
        }
    }
    match pose {
        PlayerWeaponPose::Unarmed => {}
        PlayerWeaponPose::Melee => {
            rect(14, 5, 4, 3, light);
            rect(16, 8, 2, 1, middle);
            rect(14, 13, 4, 3, light);
            rect(16, 12, 2, 1, middle);
        }
        PlayerWeaponPose::Ranged => {
            rect(14, 8, 5, 5, dark);
            rect(15, 9, 4, 3, light);
            rect(17, 10, 2, 1, core);
        }
    }
    pixels
}

fn atlas_image() -> Image {
    let width = SIDE * POSES.len() * KINDS.len();
    let mut bytes = vec![0; width * SIDE * 4];
    for kind in KINDS {
        for pose in POSES {
            let index = kind as usize * POSES.len() + pose as usize;
            for (offset, rgba) in module_pixels(kind, pose).iter().enumerate() {
                let at = ((offset / SIDE) * width + index * SIDE + offset % SIDE) * 4;
                bytes[at..at + 4].copy_from_slice(rgba);
            }
        }
    }
    Image {
        bytes,
        width: width as u16,
        height: SIDE as u16,
    }
}

thread_local! {
    static ATLAS: Texture2D = {
        let texture = Texture2D::from_image(&atlas_image());
        texture.set_filter(FilterMode::Nearest);
        texture
    };
}

pub(super) fn prewarm() {
    ATLAS.with(|_| {});
}

pub(super) fn draw(rect: Rect, appearance: ActorAppearance, opacity: f32) {
    let side = rect.w.min(rect.h);
    let index = appearance.kind as usize * POSES.len() + appearance.pose as usize;
    ATLAS.with(|texture| {
        draw_texture_ex(
            texture,
            (rect.x + (rect.w - side) * 0.5).floor(),
            (rect.y + (rect.h - side) * 0.5).floor(),
            Color::new(1.0, 1.0, 1.0, opacity),
            DrawTextureParams {
                source: Some(Rect::new(
                    (index * SIDE) as f32,
                    0.0,
                    SIDE as f32,
                    SIDE as f32,
                )),
                dest_size: Some(vec2(side, side)),
                flip_x: appearance.facing_left,
                ..Default::default()
            },
        );
    });
}

#[cfg(debug_assertions)]
pub(super) fn draw_gallery() {
    use crate::ui_theme::{draw_text, draw_text_bold};
    clear_background(Color::from_rgba(3, 4, 7, 255));
    draw_text_bold("MODULES · VUE DE DESSUS", 40.0, 40.0, 26.0, WHITE);
    draw_text(
        "Gauche / droite · pince et émetteur suivent l'orientation",
        40.0,
        68.0,
        18.0,
        GRAY,
    );
    for (column, (kind, label)) in [
        (ModuleKind::Player, "JOUEUR"),
        (ModuleKind::Service, "NPC DE SERVICE"),
        (ModuleKind::Hostile, "NPC HOSTILE"),
    ]
    .into_iter()
    .enumerate()
    {
        let x = 135.0 + column as f32 * 375.0;
        draw_text_bold(label, x, 105.0, 20.0, WHITE);
        for (row, (pose, label)) in [
            (PlayerWeaponPose::Unarmed, "NEUTRE"),
            (PlayerWeaponPose::Melee, "MÊLÉE"),
            (PlayerWeaponPose::Ranged, "DISTANCE"),
        ]
        .into_iter()
        .enumerate()
        {
            let y = 145.0 + row as f32 * 150.0;
            draw_text(label, x, y - 12.0, 17.0, GRAY);
            for (facing_left, dx) in [(true, 0.0), (false, 100.0)] {
                for (size, sx, sy) in [(80.0, 0.0, 0.0), (24.0, 0.0, 92.0), (20.0, 36.0, 92.0)] {
                    let rect = Rect::new(x + dx + sx, y + sy, size, size);
                    draw_rectangle(
                        rect.x,
                        rect.y,
                        size,
                        size,
                        Color::from_rgba(15, 19, 24, 255),
                    );
                    draw(
                        rect,
                        ActorAppearance {
                            kind,
                            pose,
                            facing_left,
                        },
                        1.0,
                    );
                }
            }
        }
    }
    draw_text(
        "Agrandissement ×4, puis tailles réelles : 24 et 20 px",
        40.0,
        610.0,
        16.0,
        GRAY,
    );
}
