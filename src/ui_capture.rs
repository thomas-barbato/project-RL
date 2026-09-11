//! Debug-only screenshots with explicit opacity and label-contrast checks.
use macroquad::miniquad;
use macroquad::prelude::{Image, Rect};

pub fn framebuffer() -> Result<Image, String> {
    let (width, height) = miniquad::window::screen_size();
    if !(1.0..=16384.0).contains(&width) || !(1.0..=16384.0).contains(&height) {
        return Err("Dimensions de capture non prises en charge.".to_owned());
    }
    let mut image = macroquad::texture::get_screen_data();
    // A window is already composited. Framebuffer alpha is not PNG transparency.
    for pixel in image.bytes.as_chunks_mut::<4>().0 {
        pixel[3] = 255;
    }
    Ok(image)
}

/// Check the label's interior, excluding its bright border/selection marker.
/// Framebuffer rows start at the bottom; GUI rectangles start at the top.
pub fn text_is_visible(image: &Image, button: Rect, pixels_per_ui_unit: f32) -> bool {
    let left = ((button.x + 14.0) * pixels_per_ui_unit).max(0.0) as usize;
    let right = (((button.x + button.w - 14.0) * pixels_per_ui_unit).max(0.0) as usize)
        .min(usize::from(image.width));
    let top = ((button.y + 4.0) * pixels_per_ui_unit).max(0.0) as usize;
    let bottom = (((button.y + button.h - 4.0) * pixels_per_ui_unit).max(0.0) as usize)
        .min(usize::from(image.height));
    let mut readable = 0;
    for y in top..bottom {
        for x in left..right {
            let offset = ((usize::from(image.height) - 1 - y) * usize::from(image.width) + x) * 4;
            let pixel = &image.bytes[offset..offset + 4];
            if pixel[1] > 180 && (pixel[0] > 180 || pixel[2] > 180) && pixel[3] == 255 {
                readable += 1;
                if readable >= 10 {
                    return true;
                }
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readability_check_rejects_black_text_and_borders_but_accepts_light_labels() {
        let mut image = Image {
            width: 100,
            height: 60,
            bytes: vec![0; 100 * 60 * 4],
        };
        let button = Rect::new(0.0, 0.0, 100.0, 60.0);
        for pixel in image.bytes.as_chunks_mut::<4>().0 {
            pixel[3] = 255;
        }
        assert!(!text_is_visible(&image, button, 1.0));
        for x in 0..100 {
            image.bytes[x * 4..x * 4 + 4].copy_from_slice(&[255, 255, 0, 255]);
        }
        assert!(!text_is_visible(&image, button, 1.0)); // Only a yellow border.
        for x in 20..35 {
            let offset = (30 * 100 + x) * 4;
            image.bytes[offset..offset + 4].copy_from_slice(&[255, 255, 0, 255]);
        }
        assert!(text_is_visible(&image, button, 1.0));
        assert!(!text_is_visible(
            &image,
            Rect::new(200.0, 200.0, 100.0, 50.0),
            1.0
        ));
    }
}
