//! Same Macroquad decoding/upload operations, with opt-in CPU measurements.
//! Texture creation measures the CPU/driver call, not GPU completion.
use macroquad::prelude::*;
use project_rl::loading_profile::Span;

#[track_caller]
pub fn image(bytes: &[u8], format: Option<ImageFormat>) -> Result<Image, macroquad::Error> {
    let _span = Span::caller("decode_image", bytes.len());
    Image::from_file_with_format(bytes, format)
}

#[track_caller]
pub fn from_image(image: &Image) -> Texture2D {
    let _span = Span::caller("texture_create_cpu", image.bytes.len());
    Texture2D::from_image(image)
}

#[track_caller]
pub fn texture(bytes: &[u8], format: Option<ImageFormat>) -> Texture2D {
    let image = image(bytes, format).unwrap_or_else(|error| panic!("{error}"));
    from_image(&image)
}
