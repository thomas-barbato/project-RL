//! Opt-in LATERAL02-r04: whole outfits and canonical front/back/idle poses.
use super::*;
use std::cell::OnceCell;
use std::sync::OnceLock;

const PNGS: [&[u8]; 5] = [
    include_bytes!("../assets/animations/lateral-04/traqueuse-tier0.png"),
    include_bytes!("../assets/animations/lateral-04/traqueuse-tier1.png"),
    include_bytes!("../assets/animations/lateral-04/traqueuse-tier2.png"),
    include_bytes!("../assets/animations/lateral-04/traqueuse-tier3.png"),
    include_bytes!("../assets/animations/lateral-04/traqueuse-tier4.png"),
];
// Fixed bounded bank. Drawing never decodes or uploads an image.
static SHEETS: OnceLock<[Image; 5]> = OnceLock::new();
static ENABLED: OnceLock<bool> = OnceLock::new();
thread_local! { static TEXTURES: OnceCell<[Texture2D; 5]> = const { OnceCell::new() }; }
#[cfg(debug_assertions)]
thread_local! { static LAST_DRAW: std::cell::Cell<Option<[usize; 3]>> = const { std::cell::Cell::new(None) }; }
#[cfg(debug_assertions)]
pub(crate) fn audit(reset: bool) -> Option<[usize; 3]> {
    if reset {
        LAST_DRAW.replace(None)
    } else {
        LAST_DRAW.get()
    }
}

pub(crate) fn enabled() -> bool {
    *ENABLED.get_or_init(|| {
        std::env::args().any(|a| {
            matches!(
                a.as_str(),
                "--essai-marche-traqueuse"
                    | "--ui-cold-lateral-trial"
                    | "--verify-lateral03-trial"
                    | "--verify-lateral04-trial"
            )
        })
    })
}

pub(crate) fn selection(visual: Visual, pose: Option<walking::Pose>) -> Option<[usize; 3]> {
    let Visual::Player {
        body: 5,
        equipment,
        weapon: None,
        hands: None | Some([None, None]),
        palettes: [EquipmentPalette::Original, _, _],
    } = visual
    else {
        return None;
    };
    let outfit = player_layers::outfit_index(5, equipment).checked_sub(512)? % 32;
    let tier = usize::from(outfit.checked_sub(10)?);
    if tier >= 5 {
        return None;
    }
    let (row, col) = match pose {
        None => (2, 6),
        Some(walking::Pose { facing, frame }) if [0, 1, 2, 3, 4, 5, 8].contains(&frame) => {
            let row = match facing {
                -1 => 0,
                1 => 1,
                2 => 2,
                -2 => 3,
                _ => return None,
            };
            (row, if frame == 8 { 6 } else { usize::from(frame) })
        }
        _ => return None,
    };
    Some([tier, row, col])
}

pub(crate) fn selected(visual: Visual, pose: walking::Pose) -> bool {
    selection(visual, Some(pose)).is_some()
}
fn sheets() -> &'static [Image; 5] {
    SHEETS.get_or_init(|| {
        PNGS.map(|png| {
            crate::asset_loading::image(png, Some(ImageFormat::Png))
                .expect("embedded LATERAL04 atlas")
        })
    })
}
pub(crate) fn prepare() {
    if enabled() && crate::asset_preparation::graphics_thread_ready() {
        TEXTURES.with(|cell| {
            cell.get_or_init(|| {
                std::array::from_fn(|tier| {
                    let texture = crate::asset_loading::from_image(&sheets()[tier]);
                    texture.set_filter(FilterMode::Nearest);
                    texture
                })
            });
        });
    }
}
fn source([_, row, col]: [usize; 3]) -> Rect {
    Rect::new(col as f32 * 96., row as f32 * 96., 96., 96.)
}
pub(crate) fn image(visual: Visual, pose: walking::Pose) -> Option<Image> {
    image_at(visual, Some(pose))
}
pub(crate) fn image_at(visual: Visual, pose: Option<walking::Pose>) -> Option<Image> {
    let selected = selection(visual, pose)?;
    let frame = sheets()[selected[0]].sub_image(source(selected));
    let mut image = Image::gen_image_color(128, 128, BLANK);
    for y in 0..96usize {
        let to = ((y + 15) * 128 + 16) * 4;
        image.bytes[to..to + 96 * 4].copy_from_slice(&frame.bytes[y * 96 * 4..(y + 1) * 96 * 4]);
    }
    Some(image)
}
pub(crate) fn draw(cell: Rect, visual: Visual, opacity: f32, pose: Option<walking::Pose>) -> bool {
    if !enabled() {
        return false;
    }
    let Some(selected) = selection(visual, pose) else {
        return false;
    };
    TEXTURES.with(|ready| {
        let Some(textures) = ready.get() else {
            return false;
        };
        #[cfg(debug_assertions)]
        LAST_DRAW.set(Some(selected));
        super::walking::trial_contact_shadow(cell, opacity);
        let scale = cell.w / 64.;
        crate::world_readability::texture(
            &textures[selected[0]],
            cell.center().x - 48. * scale,
            cell.bottom() - 81. * scale,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                source: Some(source(selected)),
                dest_size: Some(vec2(96. * scale, 96. * scale)),
                ..Default::default()
            },
        );
        true
    })
}
pub(crate) fn draw_portrait(rect: Rect, visual: Visual) -> bool {
    if !enabled() {
        return false;
    }
    let Some(selected) = selection(visual, None) else {
        return false;
    };
    TEXTURES.with(|ready| {
        let Some(textures) = ready.get() else {
            return false;
        };
        draw_texture_ex(
            &textures[selected[0]],
            rect.x + rect.w / 8.,
            rect.y + rect.h * (15. / 128.),
            WHITE,
            DrawTextureParams {
                source: Some(source(selected)),
                dest_size: Some(vec2(rect.w * 0.75, rect.h * 0.75)),
                ..Default::default()
            },
        );
        true
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn player(torso: Option<u16>) -> Visual {
        Visual::Player {
            body: 5,
            equipment: [torso, None, None, None],
            weapon: None,
            hands: None,
            palettes: [EquipmentPalette::Original; 3],
        }
    }
    #[test]
    fn trial_selects_five_traqueuse_outfits_in_all_directions_and_at_rest() {
        for (tier, torso) in [Some(108), Some(110), Some(102), Some(101), Some(103)]
            .into_iter()
            .enumerate()
        {
            let visual = player(torso);
            assert_eq!(selection(visual, None), Some([tier, 2, 6]));
            assert!(image_at(visual, None).is_some());
            for facing in [-1, 1, 2, -2] {
                let frames = (0..6)
                    .map(|frame| {
                        image(visual, walking::Pose { facing, frame })
                            .unwrap()
                            .bytes
                    })
                    .collect::<std::collections::BTreeSet<_>>();
                assert_eq!(frames.len(), 6);
                if facing.abs() == 1 {
                    assert!(
                        !frames.contains(
                            &image(visual, walking::Pose { facing, frame: 8 })
                                .unwrap()
                                .bytes
                        )
                    );
                }
            }
        }
        for body in [3, 4, 6, 7] {
            let mut v = player(Some(101));
            if let Visual::Player { body: b, .. } = &mut v {
                *b = body;
            }
            assert!(selection(v, None).is_none());
        }
        let mut armed = player(Some(101));
        if let Visual::Player { weapon, .. } = &mut armed {
            *weapon = Some(2);
        }
        assert!(selection(armed, None).is_none());
        let mut dual = player(Some(101));
        if let Visual::Player { hands, .. } = &mut dual {
            *hands = Some([Some(101), None]);
        }
        assert!(selection(dual, None).is_none());
    }
}
