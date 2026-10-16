//! Accepted D58-BODY01 and D59-BODY03, used by persistent drone actors.
use macroquad::prelude::*;
use std::cell::OnceCell;
thread_local! {
    static SCOUT: OnceCell<Texture2D> = const { OnceCell::new() };
    static TRACKER: OnceCell<Texture2D> = const { OnceCell::new() };
    static GUNNER: OnceCell<Texture2D> = const { OnceCell::new() };
    static GUARDIAN: OnceCell<Texture2D> = const { OnceCell::new() };
}

pub(super) fn draw_scout(rect: Rect, opacity: f32) {
    SCOUT.with(|cache| {
        let image = cache.get_or_init(|| {
            let image = Texture2D::from_file_with_format(include_bytes!(
                "../assets/animations/agent/skill-animation-review-97/D56-BODY01-SIZE02-native128.png"
            ), Some(ImageFormat::Png));
            image.set_filter(FilterMode::Nearest);
            image
        });
        draw_ellipse(rect.center().x, rect.center().y + rect.h * 30. / 64.,
            rect.w * 22. / 64., rect.h * 3. / 64., 0., Color::new(0.,0.,0.,opacity*0.3));
        draw_texture_ex(image, rect.center().x - rect.w, rect.center().y - rect.h,
            Color::new(1.,1.,1.,opacity), DrawTextureParams {
                dest_size:Some(vec2(rect.w*2.,rect.h*2.)), ..Default::default()
            });
    });
}

pub(super) fn draw_gunner(rect: Rect, opacity: f32) {
    GUNNER.with(|cache| {
        let image=cache.get_or_init(|| {
            let image=Texture2D::from_file_with_format(include_bytes!(
                "../assets/animations/agent/skill-animation-review-92/artilleur-body03/D59-BODY03-native64.png"
            ),Some(ImageFormat::Png));
            image.set_filter(FilterMode::Nearest);
            image
        });
        draw_texture_ex(image,rect.x,rect.y,Color::new(1.,1.,1.,opacity),
            DrawTextureParams { dest_size:Some(vec2(rect.w,rect.h)),..Default::default() });
    });
}
pub(super) fn draw_tracker(rect: Rect, opacity: f32) {
    TRACKER.with(|cache| {
        let image = cache.get_or_init(|| {
            let image = Texture2D::from_file_with_format(
                include_bytes!(
                    "../assets/animations/agent/skill-animation-review-96/D58-BODY01-native128.png"
                ),
                Some(ImageFormat::Png),
            );
            image.set_filter(FilterMode::Nearest);
            image
        });
        draw_texture_ex(
            image,
            rect.center().x - rect.w,
            rect.center().y - rect.h,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                dest_size: Some(vec2(rect.w * 2., rect.h * 2.)),
                ..Default::default()
            },
        );
    });
}

pub(super) fn draw_guardian(rect: Rect, opacity: f32) {
    GUARDIAN.with(|cache| {
        let image=cache.get_or_init(|| {
            let image=Texture2D::from_file_with_format(include_bytes!(
                "../assets/animations/agent/skill-animation-review-94/gardien-size01/D66-BODY02-SIZE01-native128.png"
            ),Some(ImageFormat::Png));
            image.set_filter(FilterMode::Nearest); image
        });
        draw_texture_ex(image,rect.center().x-rect.w,rect.center().y-rect.h,
            Color::new(1.,1.,1.,opacity),DrawTextureParams {
                dest_size:Some(vec2(rect.w*2.,rect.h*2.)),..Default::default()
            });
    });
}
