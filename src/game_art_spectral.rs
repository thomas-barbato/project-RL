//! ES03-A01: original player layers with the accepted holographic scan and opacity.
use super::*;
use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Equation, PipelineParams, UniformDesc, UniformType,
};
use std::cell::OnceCell;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Source {
    Player {
        body: u8,
        palettes: [EquipmentPalette; 3],
        weapon: Option<u16>,
        equipment: [Option<u16>; 4],
        hands: Option<[Option<u16>; 2]>,
    },
    Form(u8),
}

pub(super) fn hologram(source: Visual) -> Option<Visual> {
    Some(Visual::AgentHologram(match source {
        Visual::Player {
            body,
            palettes,
            weapon,
            equipment,
            hands,
        } => Source::Player {
            body,
            palettes,
            weapon,
            equipment,
            hands,
        },
        Visual::AgentForm(index) => Source::Form(index),
        Visual::AgentHologram(source) => source,
        _ => return None,
    }))
}

thread_local! {static SCAN:OnceCell<Material> = const {OnceCell::new()};}

pub(super) fn draw(rect: Rect, source: Source, opacity: f32, time: Option<f64>, side: u16) {
    let clock = time.unwrap_or(0.) as f32;
    // Identical persistent coefficients to fire_painted/evasion.rs::Art::ghost.
    let alpha = opacity * (0.64 + (clock * 4.).sin() * 0.06);
    SCAN.with(|cache| {
        let material=cache.get_or_init(||load_material(ShaderSource::Glsl {
            vertex:r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;uniform mat4 Model;uniform mat4 Projection;
varying highp vec2 p;varying highp vec2 uv;varying lowp vec4 tint;
void main(){p=position.xy;uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#,
            fragment:r#"#version 100
precision highp float;varying highp vec2 p;varying highp vec2 uv;varying lowp vec4 tint;
uniform sampler2D Texture;uniform float clock;uniform float origin;uniform float scale;
void main(){float lit=max(sin(((p.x-origin)/scale)*0.28-clock*6.0),0.0);
gl_FragColor=texture2D(Texture,uv)*tint*vec4(0.58+lit*0.20,0.78+lit*0.20,1.0,1.0);}"#,
        },MaterialParams {
            uniforms:vec![UniformDesc::new("clock",UniformType::Float1),UniformDesc::new("origin",UniformType::Float1),UniformDesc::new("scale",UniformType::Float1)],
            pipeline_params:PipelineParams { color_blend:Some(BlendState::new(Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),BlendFactor::OneMinusValue(BlendValue::SourceAlpha))),..Default::default()},
            ..Default::default()
        }).expect("approved hologram scan shader"));
        material.set_uniform("clock",clock);
        material.set_uniform("origin",if side==128 {rect.x} else {rect.center().x-rect.w});
        material.set_uniform("scale",rect.w/if side==128 {128.} else {64.});
        gl_use_material(material);
        match source {
            Source::Player{body,palettes,weapon,equipment,hands}=>
                player_layers::draw_player(rect,body,equipment,weapon,hands,palettes,side,alpha,None),
            Source::Form(index)=>draw_agent_form(rect,index,alpha),
        }
        gl_use_default_material();
    });
}
