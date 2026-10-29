//! VISION-U01-04: display scenery in a soft circle, independently of actor LOS.
//! Reads existing terrain memory. Never discovers cells or changes simulation.
use super::GridCamera;
use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Equation, PipelineParams, UniformDesc, UniformType,
};
use macroquad::prelude::*;
use project_rl::{game::WorldState, world::GridPos};
use std::cell::RefCell;
#[cfg(debug_assertions)]
thread_local! { static REVIEW_RADIUS: std::cell::Cell<Option<u16>> = const {std::cell::Cell::new(None)}; }
#[cfg(debug_assertions)]
pub(crate) fn review_radius(radius: Option<u16>) {
    REVIEW_RADIUS.set(radius);
}

const VERTEX: &str = r#"#version 100
attribute vec3 position;
uniform mat4 Model;
uniform mat4 Projection;
varying highp vec2 point;
void main() {
    point=position.xy;
    gl_Position=Projection*Model*vec4(position,1.0);
}"#;
const FRAGMENT: &str = r#"#version 100
precision highp float;
varying highp vec2 point;
uniform sampler2D memory;
uniform vec2 origin;
uniform vec2 worldSize;
uniform vec2 focus;
uniform vec2 radii;
uniform float memoryStrength;
void main() {
    vec2 uv=(point-origin)/worldSize;
    float range=1.0-smoothstep(radii.x,radii.y,distance(point,focus));
    float remembered=smoothstep(0.5,1.0,texture2D(memory,uv).r)*memoryStrength;
    gl_FragColor=vec4(0.0,0.0,0.0,1.0-max(range,remembered));
}"#;

thread_local! {
    static MATERIAL: RefCell<Option<Material>> = const {RefCell::new(None)};
    static MEMORY: RefCell<Option<(Image,Texture2D)>> = const {RefCell::new(None)};
}

fn memory_image(camera: &GridCamera, known: impl Fn(GridPos) -> bool) -> Image {
    let w = (camera.columns + 4) as u16;
    let h = (camera.rows + 4) as u16;
    let mut image = Image::gen_image_color(w, h, BLACK);
    for y in 0..i32::from(h) {
        for x in 0..i32::from(w) {
            let at = GridPos::new(camera.first.x + x - 2, camera.first.y + y - 2);
            image.bytes[(y as usize * usize::from(w) + x as usize) * 4] =
                if known(at) { 255 } else { 0 };
        }
    }
    image
}

pub(super) fn draw(game: &WorldState, camera: &GridCamera, known: impl Fn(GridPos) -> bool) {
    let Some(focus) = game.player_position() else {
        draw_rectangle(
            camera.viewport.x,
            camera.viewport.y,
            camera.viewport.w,
            camera.viewport.h,
            BLACK,
        );
        return;
    };
    let image = memory_image(camera, known);
    let texture = MEMORY.with(|slot| {
        let mut slot = slot.borrow_mut();
        if slot
            .as_ref()
            .is_none_or(|(old, _)| old.width != image.width || old.height != image.height)
        {
            let texture = crate::asset_loading::from_image(&image);
            texture.set_filter(FilterMode::Linear);
            *slot = Some((image.clone(), texture));
        } else if let Some((old, texture)) = slot.as_mut() {
            if old.bytes != image.bytes {
                texture.update(&image);
                *old = image.clone();
            }
        }
        slot.as_ref().unwrap().1.clone()
    });
    MATERIAL.with(|slot| {
        let mut slot = slot.borrow_mut();
        let material = slot.get_or_insert_with(|| {
            crate::world_readability::mask_material(
                ShaderSource::Glsl {
                    vertex: VERTEX,
                    fragment: FRAGMENT,
                },
                MaterialParams {
                    uniforms: vec![
                        UniformDesc::new("origin", UniformType::Float2),
                        UniformDesc::new("worldSize", UniformType::Float2),
                        UniformDesc::new("focus", UniformType::Float2),
                        UniformDesc::new("radii", UniformType::Float2),
                        UniformDesc::new("memoryStrength", UniformType::Float1),
                    ],
                    textures: vec!["memory".to_owned()],
                    pipeline_params: PipelineParams {
                        color_blend: Some(BlendState::new(
                            Equation::Add,
                            BlendFactor::Value(BlendValue::SourceAlpha),
                            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                        )),
                        alpha_blend: Some(BlendState::new(
                            Equation::Add,
                            BlendFactor::One,
                            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                        )),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            )
            .expect("circular scenery shader")
        });
        let origin = camera.rect(GridPos::new(camera.first.x - 2, camera.first.y - 2));
        let center = camera.rect(focus).center();
        let radius = game.rules().player_field_of_view.radius;
        #[cfg(debug_assertions)]
        let radius = REVIEW_RADIUS.get().unwrap_or(radius);
        // The soft rim includes cells whose centres are at the real sight
        // boundary; otherwise perceived enemies on the sixth cell turn black.
        let margin = if radius == 6 {
            std::f32::consts::FRAC_1_SQRT_2
        } else {
            -std::f32::consts::FRAC_1_SQRT_2
        };
        let outer = (f32::from(radius) + margin).max(0.5) * camera.cell;
        material.set_uniform("origin", [origin.x, origin.y]);
        material.set_uniform(
            "worldSize",
            [
                f32::from(image.width) * camera.cell,
                f32::from(image.height) * camera.cell,
            ],
        );
        material.set_uniform("focus", [center.x, center.y]);
        let memory_strength = if game.rules().player_field_of_view.radius == 6 {
            0.0f32
        } else {
            0.45f32
        };
        #[cfg(debug_assertions)]
        let memory_strength = if REVIEW_RADIUS.get().is_some() {
            0.0f32
        } else {
            memory_strength
        };
        material.set_uniform("memoryStrength", memory_strength);
        material.set_uniform("radii", [(outer - camera.cell * 1.2).max(0.), outer]);
        material.set_texture("memory", texture);
        crate::world_readability::use_material(material);
        draw_rectangle(
            camera.viewport.x,
            camera.viewport.y,
            camera.viewport.w,
            camera.viewport.h,
            WHITE,
        );
        crate::world_readability::restore();
    });
}
