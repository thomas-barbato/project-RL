//! Previous accepted rifle bank retained while the complete correction is reviewed.
use super::*;
use project_rl::asset_cache::ByteCache;
use std::cell::RefCell;
const FILES: [&str; 5] = [
    "assets/animations/walk-06/bastion-rifle.png",
    "assets/animations/walk-06/technomancien-rifle.png",
    "assets/animations/walk-06/traqueuse-rifle.png",
    "assets/animations/walk-06/arcaniste-rifle.png",
    "assets/animations/walk-06/metamorphe-rifle.png",
];
thread_local! { static CACHE: RefCell<ByteCache<usize,Texture2D>> = const { RefCell::new(ByteCache::new(8*1024*1024)) }; }
pub(super) fn draw(cell: Rect, visual: Visual, opacity: f32, pose: Option<Pose>) -> bool {
    let Some(pose) = pose else {
        return false;
    };
    if ![-1, 1].contains(&pose.facing) || ![0, 1, 2, 3, 4, 5, 8].contains(&pose.frame) {
        return false;
    }
    let Visual::Player {
        body,
        equipment,
        weapon: Some(2),
        hands: None,
        palettes,
    } = visual
    else {
        return false;
    };
    if !(3..=7).contains(&body) || palettes != [EquipmentPalette::Original; 3] {
        return false;
    }
    let Some(outfit) = player_layers::outfit_index(body, equipment)
        .checked_sub(512)
        .filter(|o| *o < 25)
    else {
        return false;
    };
    let class = usize::from(outfit / 5);
    CACHE.with_borrow_mut(|cache| {
        if cache.get(&class).is_none() {
            let Some(image) = project_rl::data_paths::read(FILES[class])
                .ok()
                .and_then(|b| Image::from_file_with_format(&b, Some(ImageFormat::Png)).ok())
                .filter(|i| (i.width, i.height) == (448, 640))
            else {
                return false;
            };
            let texture = Texture2D::from_image(&image);
            texture.set_filter(FilterMode::Nearest);
            cache.insert(class, texture, image.bytes.len());
        }
        let texture = cache.get(&class).unwrap();
        draw_contact_shadow(cell, [32, 63, 17], 64, opacity);
        crate::world_readability::texture(
            texture,
            cell.x,
            cell.y,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                source: Some(Rect::new(
                    if pose.frame == 8 {
                        0.
                    } else {
                        f32::from(pose.frame + 1) * 64.
                    },
                    f32::from(outfit % 5 * 2 + u16::from(pose.facing > 0)) * 64.,
                    64.,
                    64.,
                )),
                dest_size: Some(cell.size()),
                ..Default::default()
            },
        );
        true
    })
}
