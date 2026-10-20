//! Approved STYLE-MAP-U01-04 finish, applied to scenery before actors and light.
use super::*;

#[derive(Clone, Copy)]
pub(super) struct Settings {
    pub procedural: f32,
    pub colored: f32,
    pub fan: [f32; 4],
}

pub(super) struct Surface {
    pub background: Material,
    pub isolated: Material,
    _wear: Texture2D,
}

impl Surface {
    pub fn new(correction: Texture2D, size: Vec2, settings: Settings, wear_path: &str) -> Self {
        let fragment = BACKGROUND
            .replacen("void main() {", &format!("{WEAR}\nvoid main() {{"), 1)
            .replacen(
                "vec3 neutral=",
                "ink.rgb=worn(ink.rgb,uv*ReviewSize,1.0-data.a);\n    vec3 neutral=",
                1,
            );
        let background = material(
            &fragment,
            &["Correction", "Wear"],
            &[
                ("LightOn", mq::UniformType::Float1),
                ("ProceduralEmitters", mq::UniformType::Float1),
                ("ColoredEmitters", mq::UniformType::Float1),
                ("Fan", mq::UniformType::Float4),
                ("Clock", mq::UniformType::Float1),
                ("ReviewSize", mq::UniformType::Float2),
            ],
        );
        background.set_texture("Correction", correction);
        background.set_uniform("ProceduralEmitters", settings.procedural);
        background.set_uniform("ColoredEmitters", settings.colored);
        background.set_uniform(
            "Fan",
            (
                settings.fan[0],
                settings.fan[1],
                settings.fan[2],
                settings.fan[3],
            ),
        );
        background.set_uniform("Clock", 0.0_f32);
        background.set_uniform("ReviewSize", (size.x, size.y));
        let isolated = material(
            &format!("{ISOLATED_HEAD}\n{WEAR}\n{ISOLATED_MAIN}"),
            &["Wear"],
            &[("ReviewSize", mq::UniformType::Float2)],
        );
        let wear = load(wear_path, FilterMode::Linear);
        background.set_texture("Wear", wear.clone());
        isolated.set_texture("Wear", wear.clone());
        Self {
            background,
            isolated,
            _wear: wear,
        }
    }
}

// Neutral mask values leave broad quiet areas. Dark outlines, black void and
// live emitters are protected; material hues and recognizable forms survive.
const WEAR: &str = r#"
uniform sampler2D Wear;
uniform vec2 ReviewSize;
vec3 worn(vec3 color,vec2 px,float allowed) {
    float a=texture2D(Wear,fract(px/512.0)).r;
    float b=texture2D(Wear,fract(vec2(px.y,-px.x)/1013.0+vec2(0.31,0.71))).r;
    float value=max(color.r,max(color.g,color.b));
    float surface=smoothstep(0.10,0.32,value)*allowed;
    float plant=max(0.0,color.g-max(color.r,color.b));
    surface*=1.0-clamp(plant*4.0,0.0,0.75);
    float grime=1.0-smoothstep(0.33,0.50,a);
    float broad=1.0-smoothstep(0.36,0.51,b);
    float chip=smoothstep(0.55,0.67,a);
    float gray=dot(color,vec3(0.30,0.59,0.11));
    vec3 matte=mix(color,gray*vec3(1.02,1.0,0.94),0.14*surface);
    matte*=1.0-(0.40*grime+0.13*broad)*surface;
    matte=mix(matte,vec3(0.39,0.36,0.29),chip*surface*0.47);
    return clamp(matte,0.0,1.0);
}
"#;

const ISOLATED_HEAD: &str = r#"#version 100
precision highp float;
varying highp vec2 uv;
varying lowp vec4 tint;
uniform sampler2D Texture;
"#;
const ISOLATED_MAIN: &str = r#"
void main() {
    vec4 ink=texture2D(Texture,uv);
    ink.rgb=worn(ink.rgb,uv*ReviewSize,1.0);
    gl_FragColor=ink*tint;
}
"#;
