//! Shared portrait framing: ignore transparent canvas padding, keep every layer.
//! Bounds are measured once on CPU when a source/composition enters its cache.
use macroquad::prelude::*;

pub(super) struct Sheet {
    pub texture: Texture2D,
    bounds: Vec<Option<Rect>>,
}

impl Sheet {
    pub fn load(bytes: &[u8], side: u16) -> Self {
        let image = Image::from_file_with_format(bytes, Some(ImageFormat::Png)).unwrap();
        Self::from_image(&image, side)
    }

    pub fn from_image(image: &Image, side: u16) -> Self {
        let columns = image.width / side;
        let bounds = (0..columns * (image.height / side))
            .map(|i| {
                opaque_bounds(
                    image,
                    Rect::new(
                        f32::from(i % columns * side),
                        f32::from(i / columns * side),
                        f32::from(side),
                        f32::from(side),
                    ),
                )
            })
            .collect();
        let texture = Texture2D::from_image(image);
        texture.set_filter(FilterMode::Nearest);
        Self { texture, bounds }
    }

    pub fn draw(&self, frame: Rect, index: u16, opacity: f32) {
        let Some(Some(source)) = self.bounds.get(usize::from(index)) else {
            return;
        };
        let destination = contain(frame, *source);
        crate::world_readability::texture(
            &self.texture,
            destination.x,
            destination.y,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                source: Some(*source),
                dest_size: Some(destination.size()),
                ..Default::default()
            },
        );
    }
}

fn opaque_bounds(image: &Image, cell: Rect) -> Option<Rect> {
    let (mut left, mut top, mut right, mut bottom) =
        (cell.right() as u32, cell.bottom() as u32, 0, 0);
    for y in cell.y as u32..cell.bottom() as u32 {
        for x in cell.x as u32..cell.right() as u32 {
            if image.bytes[((y * u32::from(image.width) + x) * 4 + 3) as usize] != 0 {
                left = left.min(x);
                top = top.min(y);
                right = right.max(x + 1);
                bottom = bottom.max(y + 1);
            }
        }
    }
    (right > left && bottom > top).then(|| {
        Rect::new(
            left as f32,
            top as f32,
            (right - left) as f32,
            (bottom - top) as f32,
        )
    })
}

fn contain(frame: Rect, source: Rect) -> Rect {
    let scale = (frame.w / source.w).min(frame.h / source.h);
    let size = source.size() * scale;
    Rect::new(
        frame.center().x - size.x * 0.5,
        frame.center().y - size.y * 0.5,
        size.x,
        size.y,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transparent_padding_does_not_shrink_a_portrait_or_crop_equipment() {
        let mut image = Image::gen_image_color(128, 128, BLANK);
        for (x, y) in [(20, 40), (102, 98), (65, 34)] {
            image.set_pixel(x, y, WHITE);
        }
        let bounds = opaque_bounds(&image, Rect::new(0., 0., 128., 128.)).unwrap();
        assert_eq!(bounds, Rect::new(20., 34., 83., 65.));
        let frame = Rect::new(500., 200., 96., 96.);
        let fitted = contain(frame, bounds);
        assert_eq!(fitted.w, 96.);
        assert!(fitted.h <= frame.h && fitted.y >= frame.y && fitted.bottom() <= frame.bottom());
        assert_eq!(fitted.center(), frame.center());
        assert!(opaque_bounds(&image, Rect::new(0., 0., 16., 16.)).is_none());
    }

    #[test]
    fn atlas_bounds_stay_inside_the_requested_cell() {
        let mut image = Image::gen_image_color(64, 32, BLANK);
        image.set_pixel(31, 31, WHITE);
        image.set_pixel(32, 0, WHITE);
        assert_eq!(
            opaque_bounds(&image, Rect::new(0., 0., 32., 32.)),
            Some(Rect::new(31., 31., 1., 1.))
        );
        assert_eq!(
            opaque_bounds(&image, Rect::new(32., 0., 32., 32.)),
            Some(Rect::new(32., 0., 1., 1.))
        );
    }
}
