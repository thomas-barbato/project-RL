//! Presentation policy accepted on 20 October 2026. No simulation clock changes.
use macroquad::{
    miniquad::{UniformDesc, UniformType},
    prelude::*,
};
use std::cell::{Cell, RefCell};

pub fn duration_scale(skill: &str) -> f64 {
    match skill {
        "FE05" => 1.30,
        "B01" => 1.35,
        "reaction" => 1.40,
        _ => 1.50,
    }
}
pub fn age(skill: &str, now: f64, started: f64) -> f64 {
    (now - started) / duration_scale(skill)
}
pub fn clock(now: f64) -> f64 {
    now / 1.30
}
pub fn gain(skill: &str) -> f32 {
    match skill {
        "A01" => 1.30,
        "OC05" => 1.40,
        "FR05" | "FE05" => 1.0,
        "FR06" => 1.12,
        _ => 1.18,
    }
}
thread_local! {
    static GAIN: Cell<Option<f32>> = const { Cell::new(None) };
    static PLAIN: RefCell<Option<Material>> = const { RefCell::new(None) };
}
pub fn with_gain<T>(gain: f32, draw: impl FnOnce() -> T) -> T {
    let _color = crate::world_readability::color();
    struct Guard(Option<f32>);
    impl Drop for Guard {
        fn drop(&mut self) {
            GAIN.set(self.0);
            restore_material();
        }
    }
    let _guard = Guard(GAIN.replace(Some(gain)));
    restore_material();
    draw()
}
pub fn material(
    source: ShaderSource<'_>,
    mut params: MaterialParams,
) -> Result<Material, macroquad::Error> {
    let ShaderSource::Glsl { vertex, fragment } = source else {
        return load_material(source, params);
    };
    let mut fragment = fragment.to_owned();
    let end = fragment.rfind('}').expect("fragment main");
    fragment.insert_str(
        end,
        "gl_FragColor.rgb=clamp(gl_FragColor.rgb*readabilityGain,0.0,1.0);",
    );
    let precision = fragment
        .find("precision ")
        .and_then(|p| fragment[p..].find(';').map(|i| p + i + 1))
        .unwrap();
    fragment.insert_str(precision, "uniform float readabilityGain;");
    params
        .uniforms
        .push(UniformDesc::new("readabilityGain", UniformType::Float1));
    load_material(
        ShaderSource::Glsl {
            vertex,
            fragment: &fragment,
        },
        params,
    )
}
pub fn use_material(material: &Material) {
    material.set_uniform("readabilityGain", GAIN.get().unwrap_or(1.0));
    gl_use_material(material);
}
pub fn restore_material() {
    if GAIN.get().is_none() {
        gl_use_default_material();
        return;
    }
    PLAIN.with_borrow_mut(|plain| {
        let material = plain.get_or_insert_with(|| {
            material(
                ShaderSource::Glsl {
                    vertex: r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;varying highp vec2 uv;varying lowp vec4 tint;
void main(){uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#,
                    fragment: r#"#version 100
precision highp float;varying highp vec2 uv;varying lowp vec4 tint;uniform sampler2D Texture;
void main(){gl_FragColor=texture2D(Texture,uv)*tint;}"#,
                },
                MaterialParams {
                    pipeline_params: blend(),
                    ..Default::default()
                },
            )
            .expect("effect readability material")
        });
        use_material(material);
    });
}
pub fn blend() -> macroquad::miniquad::PipelineParams {
    use macroquad::miniquad::*;
    PipelineParams {
        color_blend: Some(BlendState::new(
            Equation::Add,
            BlendFactor::Value(BlendValue::SourceAlpha),
            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
        )),
        ..Default::default()
    }
}
