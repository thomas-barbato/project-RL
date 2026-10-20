//! Exact accepted poses and completed source-authored rigs for current outfits.
use super::*;
#[path = "game_art_finish_worn_generic.rs"]
mod generic;

fn binding(key: Key) -> Option<(&'static str, Option<&'static str>)> {
    let (outfit, hands, colors) = key;
    if colors[0] != EquipmentPalette::Original {
        return None;
    }
    let code = match outfit {
        200 => "FIN-01",
        212 => "FIN-02",
        4 => "FIN-03",
        6 => "FIN-04",
        15 => "FIN-05",
        201 => "FIN-14",
        202 => "FIN-15",
        203 => "FIN-16",
        204 => "FIN-17",
        213 => "FIN-18",
        214 => "FIN-19",
        215 => "FIN-20",
        216 => "FIN-21",
        208 => "FIN-22",
        209 => "FIN-23",
        210 => "FIN-24",
        211 => "FIN-25",
        217 => "FIN-26",
        218 => "FIN-27",
        _ => return None,
    };
    let pose = match hands {
        [None, None] => return Some((code, None)),
        [None, Some(layer)]
            if !layer.left
                && colors[1] == EquipmentPalette::Original
                && matches!(outfit, 200..=204 | 208..=218) =>
        {
            match layer.model {
                101 => "single",
                2 => "rifle",
                _ => return None,
            }
        }
        _ => return None,
    };
    Some((code, Some(pose)))
}
pub(super) fn supports(key: Key) -> bool {
    binding(key).is_some() || generic::supports(key)
}
fn files(key: Key) -> Option<Vec<String>> {
    let (code, pose) = binding(key)?;
    Some(if let Some(pose) = pose {
        ["body", "weapon", "fingers"]
            .map(|part| format!("{code}-{pose}-{part}"))
            .to_vec()
    } else {
        vec![code.to_owned()]
    })
}
fn layers(key: Key, breath: u8) -> Option<Vec<Image>> {
    if binding(key).is_none() {
        return Some(
            generic::layers(key)?
                .into_iter()
                .enumerate()
                .map(|(index, image)| breathed(image, index, breath))
                .collect(),
        );
    }
    Some(
        files(key)?
            .iter()
            .enumerate()
            .map(|(index, file)| {
                let image = finish::image(file).unwrap();
                if breath == 0 {
                    return image;
                }
                let mut posed = Image::gen_image_color(128, 128, BLANK);
                // The existing native breath moves the upper body and held layers by
                // two pixels. Feet remain fixed; no interpolation or new animation.
                for y in 0..128usize {
                    let dest = if index != 0 || y < 78 { y + 2 } else { y };
                    if dest < 128 && (index != 0 || y >= 78 || dest < 78) {
                        posed.bytes[dest * 512..(dest + 1) * 512]
                            .copy_from_slice(&image.bytes[y * 512..(y + 1) * 512]);
                    }
                }
                posed
            })
            .collect(),
    )
}
fn breathed(image: Image, index: usize, breath: u8) -> Image {
    if breath == 0 {
        return image;
    }
    let mut posed = Image::gen_image_color(128, 128, BLANK);
    for y in 0..128usize {
        let dest = if index != 0 || y < 78 { y + 2 } else { y };
        if dest < 128 && (index != 0 || y >= 78 || dest < 78) {
            posed.bytes[dest * 512..(dest + 1) * 512]
                .copy_from_slice(&image.bytes[y * 512..(y + 1) * 512]);
        }
    }
    posed
}
pub(super) fn compose(key: Key, breath: u8) -> Option<Image> {
    let layers = layers(key, breath)?;
    let mut composed = Image::gen_image_color(128, 128, BLANK);
    for layer in layers {
        flat_agents::alpha_over(&mut composed, &layer);
    }
    Some(composed)
}
struct Pack {
    layers: Vec<Texture2D>,
    portrait: portrait::Sheet,
}
thread_local! {
    static CACHE: std::cell::RefCell<BTreeMap<(Key,u8),(Pack,f64)>> = const {std::cell::RefCell::new(BTreeMap::new())};
}
#[cfg(debug_assertions)]
pub(super) fn cache_len() -> usize {
    CACHE.with(|cache| cache.borrow().len())
}
pub(super) fn draw(rect: Rect, key: Key, side: u16, opacity: f32, breath: u8) -> bool {
    if !supports(key) {
        return false;
    }
    CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        let cache_key = (key, breath);
        if !cache.contains_key(&cache_key) {
            let images = layers(key, breath).unwrap();
            let pack = Pack {
                layers: images
                    .iter()
                    .map(|im| {
                        let texture = Texture2D::from_image(im);
                        texture.set_filter(FilterMode::Nearest);
                        texture
                    })
                    .collect(),
                portrait: portrait::Sheet::from_image(&compose(key, breath).unwrap(), 128),
            };
            palettes::evict(&mut cache, 192);
            cache.insert(cache_key, (pack, get_time()));
            #[cfg(debug_assertions)]
            CREATIONS.with(|count| count.set(count.get() + 1));
        }
        let (pack, time) = cache.get_mut(&cache_key).unwrap();
        *time = get_time();
        if side == 128 {
            pack.portrait.draw(rect, 0, opacity);
            return;
        }
        let dest = Rect::new(
            rect.x - rect.w / 2.,
            rect.y - rect.h / 2.,
            rect.w * 2.,
            rect.h * 2.,
        );
        for texture in &pack.layers {
            draw_texture_ex(
                texture,
                dest.x,
                dest.y,
                Color::new(1., 1., 1., opacity),
                DrawTextureParams {
                    dest_size: Some(dest.size()),
                    ..Default::default()
                },
            );
        }
    });
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "Explicit native visual diagnostic; writes only to the supplied review directory"]
    fn ranged_grip_native_diagnostic() {
        let output = std::path::PathBuf::from(
            std::env::var("RL_GRIP_CAPTURE_OUTPUT").expect("review output directory required"),
        );
        std::fs::create_dir_all(&output).unwrap();
        for outfit in [200, 201, 202, 203, 204] {
            for (label, color) in [
                ("original", EquipmentPalette::Original),
                ("blue", EquipmentPalette::Blue),
            ] {
                for weapon in [None, Some(101), Some(38), Some(15)] {
                    let key = (
                        outfit,
                        selection(weapon, None),
                        [
                            color,
                            EquipmentPalette::Original,
                            EquipmentPalette::Original,
                        ],
                    );
                    let name = format!(
                        "{outfit}-{}-{label}.png",
                        weapon.map_or_else(|| "none".into(), |m| m.to_string())
                    );
                    compose(key, 0)
                        .unwrap()
                        .export_png(output.join(name).to_str().unwrap());
                }
            }
        }
    }
    #[test]
    fn exact_approved_poses_preserve_core_and_completed_hands_are_available() {
        for outfit in [
            200, 212, 201, 202, 203, 204, 213, 214, 215, 216, 208, 209, 210, 211, 217, 218,
        ] {
            let key = |weapon, hands| {
                (
                    outfit,
                    selection(weapon, hands),
                    [EquipmentPalette::Original; 3],
                )
            };
            let base = compose(key(None, None), 0).unwrap();
            for weapon in [2, 101] {
                let composed = compose(key(Some(weapon), None), 0).unwrap();
                let fixed_legs = if matches!(outfit, 200 | 212) { 72 } else { 80 };
                for rows in [0..44usize, fixed_legs..128] {
                    assert_eq!(
                        &base.bytes[rows.start * 512..rows.end * 512],
                        &composed.bytes[rows.start * 512..rows.end * 512]
                    );
                }
                assert_ne!(base.bytes, composed.bytes);
            }
            assert!(compose(key(Some(3), None), 0).is_some());
            assert!(compose(key(None, Some([Some(101), Some(101)])), 0).is_some());
            let mut recolored = key(Some(101), None);
            recolored.2[0] = EquipmentPalette::Blue;
            assert!(supports(recolored));
            recolored = key(Some(101), None);
            recolored.2[1] = EquipmentPalette::Blue;
            assert!(supports(recolored));
            let breath = compose(key(None, None), 1).unwrap();
            assert_eq!(&base.bytes[78 * 512..], &breath.bytes[78 * 512..]);
        }
    }
}
