//! Approved upright player art, packed once into a small nearest-filtered atlas.
//! Facing and equipment are presentation only; the world and save schema stay separate.
use macroquad::prelude::*;
use project_rl::combat::AttackDelivery;

const SIDE: usize = 24;
const SOURCE: &[u8] = include_bytes!("../assets/sprites/player-upright-v1.png");

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

// Source poses share a baseline. Anchor each one at its feet so that the wider
// knife/rifle cannot displace the body when the active weapon changes.
fn atlas_image() -> Image {
    let source = Image::from_file_with_format(SOURCE, None).expect("embedded player sprite PNG");
    let width = source.width as usize;
    let height = source.height as usize;
    let pixel = |x: usize, y: usize| -> &[u8] {
        &source.bytes[(y * width + x) * 4..(y * width + x + 1) * 4]
    };
    let mut bytes = vec![0; SIDE * 3 * (20 + SIDE) * 4];
    for pose in 0..3 {
        let start = pose * width / 3;
        let end = (pose + 1) * width / 3;
        let (mut left, mut top, mut right, mut bottom) = (end, height, start, 0);
        for y in 0..height {
            for x in start..end {
                if pixel(x, y)[3] >= 128 {
                    left = left.min(x);
                    top = top.min(y);
                    right = right.max(x + 1);
                    bottom = bottom.max(y + 1);
                }
            }
        }
        assert!(right > left && bottom > top, "empty embedded player pose");
        let (mut foot_left, mut foot_right) = (right, left);
        for y in bottom - (bottom - top) / 10..bottom {
            for x in left..right {
                if pixel(x, y)[3] >= 128 {
                    foot_left = foot_left.min(x);
                    foot_right = foot_right.max(x + 1);
                }
            }
        }
        let foot_center = (foot_left + foot_right) as f32 * 0.5;
        // The default 20 px zoom has its own native row. Scaling down the 24 px
        // row could discard a one-pixel visor or toe during texture sampling.
        for (native, row) in [(20, 0), (SIDE, 20)] {
            let scale = ((native - 2) as f32 / (bottom - top) as f32)
                .min((native / 2 - 1) as f32 / (foot_center - left as f32))
                .min((native / 2 - 1) as f32 / (right as f32 - foot_center));
            for y in 1..native - 1 {
                for x in 1..native - 1 {
                    let sx = foot_center + (x as f32 + 0.5 - native as f32 * 0.5) / scale;
                    let sy = bottom as f32 - (native as f32 - 1.0 - y as f32 - 0.5) / scale;
                    if sx < left as f32 || sx >= right as f32 || sy < top as f32 {
                        continue;
                    }
                    let rgba = pixel(sx as usize, sy as usize);
                    if rgba[3] < 128 {
                        continue;
                    }
                    let [r, g, b, _] = <[u8; 4]>::try_from(rgba).unwrap();
                    let luma = (r as u16 * 3 + g as u16 * 6 + b as u16) / 10;
                    let color = if g.saturating_sub(r) > 20 && b.saturating_sub(r) > 20 {
                        [46, 225, 229, 255]
                    } else if luma >= 185 {
                        [238, 233, 221, 255]
                    } else if luma >= 100 {
                        [151, 155, 155, 255]
                    } else {
                        [76, 81, 85, 255]
                    };
                    let at = ((row + y) * SIDE * 3 + pose * SIDE + x) * 4;
                    bytes[at..at + 4].copy_from_slice(&color);
                }
            }
        }
    }
    Image {
        bytes,
        width: (SIDE * 3) as u16,
        height: (20 + SIDE) as u16,
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

pub(super) fn draw(rect: Rect, appearance: PlayerAppearance, opacity: f32) {
    let side = rect.w.min(rect.h);
    let (native, row) = if side <= 20.0 {
        (20.0, 0.0)
    } else {
        (SIDE as f32, 20.0)
    };
    ATLAS.with(|texture| {
        draw_texture_ex(
            texture,
            rect.x + (rect.w - side) * 0.5,
            rect.y + (rect.h - side) * 0.5,
            Color::new(1.0, 1.0, 1.0, opacity),
            DrawTextureParams {
                source: Some(Rect::new(
                    (appearance.pose as usize * SIDE) as f32,
                    row,
                    native,
                    native,
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
    draw_text_bold("JOUEUR · RENDU NATIF", 40.0, 48.0, 26.0, WHITE);
    draw_text(
        "Trois poses, retournement horizontal, pieds clairs",
        40.0,
        80.0,
        18.0,
        GRAY,
    );
    for (column, (pose, label)) in [
        (PlayerWeaponPose::Unarmed, "MAINS VIDES"),
        (PlayerWeaponPose::Melee, "MÊLÉE"),
        (PlayerWeaponPose::Ranged, "DISTANCE"),
    ]
    .into_iter()
    .enumerate()
    {
        let x = 155.0 + column as f32 * 330.0;
        draw_text_bold(label, x, 132.0, 20.0, WHITE);
        for (row, size) in [96.0, 24.0, 20.0].into_iter().enumerate() {
            let y = 170.0 + row as f32 * 155.0;
            draw_text(
                &format!("{} px", size as u16),
                40.0,
                y + size * 0.5,
                18.0,
                GRAY,
            );
            for (facing_left, dx) in [(true, 0.0), (false, 140.0)] {
                let rect = Rect::new(x + dx, y, size, size);
                draw_rectangle(
                    rect.x - 8.0,
                    rect.y - 8.0,
                    size + 16.0,
                    size + 16.0,
                    Color::from_rgba(15, 19, 24, 255),
                );
                draw(rect, PlayerAppearance { pose, facing_left }, 1.0);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_player_poses_fit_one_cell_with_a_visible_visor_and_light_feet() {
        let atlas = atlas_image();
        assert_eq!((atlas.width, atlas.height), (72, 44));
        for (native, row) in [(20, 0), (SIDE, 20)] {
            for pose in 0..3 {
                let mut cyan = 0;
                let mut bright_feet = 0;
                for y in 0..native {
                    for x in 0..native {
                        let at = ((row + y) * SIDE * 3 + pose * SIDE + x) * 4;
                        let pixel = &atlas.bytes[at..at + 4];
                        if x == 0 || x == native - 1 || y == 0 || y == native - 1 {
                            assert_eq!(pixel[3], 0, "pose {pose} exceeds its cell");
                        }
                        if y < native / 3 && pixel == [46, 225, 229, 255] {
                            cyan += 1;
                        }
                        if y >= native - 4 && pixel == [238, 233, 221, 255] {
                            bright_feet += 1;
                        }
                    }
                }
                assert!(cyan > 0, "pose {pose} lost its visor");
                assert!(bright_feet >= 2, "pose {pose} lost its light feet");
            }
        }
    }
}
