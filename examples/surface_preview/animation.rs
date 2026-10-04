//! Optional cosmetic GPU effects. No per-frame images, uploads or simulation work.
use macroquad::{miniquad as mq, prelude::*};
use std::cell::Cell;

pub struct Animation {
    clock: Cell<Option<f64>>,
    material: Option<Material>,
}
impl Animation {
    pub fn new() -> Self {
        let material = load_material(
            ShaderSource::Glsl {
                vertex: VERTEX,
                fragment: FRAGMENT,
            },
            MaterialParams {
                uniforms: vec![
                    mq::UniformDesc::new("Clock", mq::UniformType::Float1),
                    mq::UniformDesc::new("Kind", mq::UniformType::Float1),
                    mq::UniformDesc::new("Canvas", mq::UniformType::Float3),
                ],
                pipeline_params: mq::PipelineParams {
                    color_blend: Some(mq::BlendState::new(
                        mq::Equation::Add,
                        mq::BlendFactor::Value(mq::BlendValue::SourceAlpha),
                        mq::BlendFactor::OneMinusValue(mq::BlendValue::SourceAlpha),
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .map_err(|e| eprintln!("Animations visuelles indisponibles : {e:?}"))
        .ok();
        Self {
            clock: Cell::new(None),
            material,
        }
    }
    pub fn time(&self) -> Option<f64> {
        self.clock.get()
    }
    pub fn set_time(&self, value: Option<f64>, origin: Vec2, cell: f32) {
        self.clock.set(value);
        if let Some(material) = &self.material {
            material.set_uniform("Clock", value.unwrap_or(0.) as f32);
            material.set_uniform("Canvas", (origin.x, origin.y, cell));
        }
    }
    pub fn pause(&self) -> Option<f64> {
        self.clock.replace(None)
    }
    pub fn begin(&self, kind: f32) {
        if self.clock.get().is_some() {
            if let Some(material) = &self.material {
                material.set_uniform("Kind", kind);
                gl_use_material(material);
            }
        }
    }
    pub fn end(&self) {
        gl_use_default_material();
    }
}

const VERTEX: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
uniform mat4 Model;
uniform mat4 Projection;
uniform vec3 Canvas;
varying lowp vec2 uv;
varying lowp vec4 tint;
varying mediump vec2 world;
void main() {
    gl_Position = Projection * Model * vec4(position,1.0);
    uv = texcoord;
    tint = color0 / 255.0;
    world = (position.xy - Canvas.xy) / Canvas.z;
}"#;
const FRAGMENT: &str = r#"#version 100
precision mediump float;
varying lowp vec2 uv;
varying lowp vec4 tint;
varying mediump vec2 world;
uniform sampler2D Texture;
uniform float Clock;
uniform float Kind;
void main() {
    vec4 ink = texture2D(Texture,uv);
    if (Kind < 1.5) {
        // Only blue water pixels move in a composite paint tile.
        if (ink.b > ink.r * 1.4 && ink.g > ink.r * 1.2) {
            float ripple = sin(world.x * 2.1 + world.y * 3.7 - Clock * 1.8);
            ink.rgb *= 0.96 + 0.04 * ripple;
        }
    } else if (ink.g > ink.r * 1.5 && max(ink.g,ink.b) > 0.48) {
        ink.rgb *= 0.82 + 0.18 * sin(Clock * 2.5 + uv.y * 4.0);
    }
    gl_FragColor = ink * tint;
}"#;
