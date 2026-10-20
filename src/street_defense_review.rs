//! DEFENSE-U01-10 visual proposal. Excluded from release; never adds live actors.
use crate::{camp_atmosphere::Atmosphere, prison_street::Street};
use macroquad::prelude::*;
use serde::Deserialize;
use std::{
    cell::{Cell, RefCell},
    sync::OnceLock,
};

#[derive(Deserialize)]
pub struct Guard {
    pub file: String,
    pub cell: [i32; 2],
    pub phase: f32,
    #[serde(default)]
    pub pixel_offset: [f32; 2],
    pub foot_anchor: [f32; 2],
}
#[derive(Deserialize)]
pub struct Cover {
    pub at_px: [f32; 2],
    pub mirror: bool,
}
#[derive(Deserialize)]
pub struct Fire {
    pub foot: [f32; 2],
    pub size: [f32; 2],
    pub phase: u32,
}
#[derive(Deserialize)]
pub struct Data {
    pub guards: Vec<Guard>,
    pub fires: Vec<Fire>,
    pub proposed_solid_cells: Vec<[i32; 2]>,
    pub central_free_cells: Vec<[i32; 2]>,
    #[serde(default)]
    pub covers: Vec<Cover>,
}
pub fn data() -> &'static Data {
    static DATA: OnceLock<Data> = OnceLock::new();
    DATA.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../assets/source-art/first-sector-review-01/u01-defense-r10/defense.json"
        ))
        .unwrap()
    })
}
pub fn scene() -> &'static Street {
    static SCENE: OnceLock<Street> = OnceLock::new();
    SCENE.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../assets/source-art/first-sector-review-01/u01-defense-r10/street.json"
        ))
        .unwrap()
    })
}
struct Bank {
    guards: Vec<Texture2D>,
    flame: Texture2D,
}
thread_local! { static ACTIVE:Cell<bool>=const{Cell::new(false)}; static BANK:RefCell<Option<Bank>>=const{RefCell::new(None)}; }
pub fn enabled() -> bool {
    ACTIVE.get()
}
pub fn enable(value: bool) {
    ACTIVE.set(value);
    crate::prison_street_render::review_repairs(false);
}
fn load(file: &str) -> Texture2D {
    let texture = Texture2D::from_file_with_format(
        &std::fs::read(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(file)).unwrap(),
        Some(ImageFormat::Png),
    );
    texture.set_filter(FilterMode::Nearest);
    texture
}
pub fn draw_cover_shadows(origin: Vec2, scale: f32) {
    gl_use_default_material();
    for cover in &data().covers {
        let at = origin + vec2(cover.at_px[0], cover.at_px[1]) * scale;
        let color = Color::new(0., 0., 0., 0.18);
        draw_rectangle(
            at.x + 4. * scale,
            at.y + 27. * scale,
            72. * scale,
            4. * scale,
            color,
        );
        let side = if cover.mirror { 61. } else { 15. };
        draw_rectangle(
            at.x + side * scale,
            at.y + 29. * scale,
            5. * scale,
            40. * scale,
            color,
        );
    }
}
/// Drawn into the existing world framebuffer, before the ordinary FOV and HUD.
pub fn draw(origin: Vec2, scale: f32, time: f32, atmosphere: &Atmosphere) {
    BANK.with(|bank| {
        let mut bank = bank.borrow_mut();
        let bank = bank.get_or_insert_with(|| Bank {
            guards: data().guards.iter().map(|g| load(&g.file)).collect(),
            flame: load("assets/animations/agent/skill-animation-review-06/flame-atlas.png"),
        });
        for (guard, texture) in data().guards.iter().zip(&bank.guards) {
            let at = origin
                + vec2(
                    guard.cell[0] as f32 * 64. + guard.pixel_offset[0],
                    guard.cell[1] as f32 * 64. + guard.pixel_offset[1],
                ) * scale;
            gl_use_default_material();
            draw_ellipse(
                at.x + guard.foot_anchor[0] * scale,
                at.y + guard.foot_anchor[1] * scale,
                12. * scale,
                2.5 * scale,
                0.,
                Color::new(0., 0., 0., 0.24),
            );
            atmosphere.begin_surface(texture.size());
            let breath = (time * 2.1 + guard.phase).sin() * 0.35;
            for (y, h, offset) in [(0., 48., breath), (48., 16., 0.)] {
                draw_texture_ex(
                    texture,
                    at.x,
                    at.y + (y + offset) * scale,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(0., y, 64., h)),
                        dest_size: Some(vec2(64., h) * scale),
                        ..Default::default()
                    },
                );
            }
        }
        gl_use_default_material();
        for fire in &data().fires {
            let foot = origin + vec2(fire.foot[0], fire.foot[1]) * scale;
            let frame = ((time.max(0.) * 8.) as u32 + fire.phase) % 8;
            for (dx, dy, w, h, phase) in [
                (0., 0., fire.size[0], fire.size[1], 0),
                (14., -3., fire.size[0] * 0.6, fire.size[1] * 0.72, 3),
            ] {
                draw_texture_ex(
                    &bank.flame,
                    foot.x + (dx - w / 2.) * scale,
                    foot.y + (dy - h) * scale,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(
                            ((frame + phase) % 8) as f32 * 128.,
                            0.,
                            128.,
                            128.,
                        )),
                        dest_size: Some(vec2(w, h) * scale),
                        ..Default::default()
                    },
                );
            }
            for i in 0..3 {
                let age = (time * 0.6 + i as f32 * 0.31 + fire.phase as f32 * 0.17).fract();
                draw_circle(
                    foot.x + (age * 4. + i as f32).sin() * 8. * scale,
                    foot.y - (14. + age * 55.) * scale,
                    0.75 * scale,
                    Color::new(1., 0.43, 0.08, (1. - age) * 0.7),
                );
            }
        }
    });
}
