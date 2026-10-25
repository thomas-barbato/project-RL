//! ACTOR-READABILITY01: world-only grading; effects and UI keep their colors.
use macroquad::{
    miniquad::{UniformDesc, UniformType},
    prelude::*,
};
use std::cell::{Cell, RefCell};

#[derive(Clone, Copy, Default)]
struct State {
    world: bool,
    gray: bool,
    actor: bool,
}
thread_local! {
    static STATE: Cell<State> = const { Cell::new(State { world:false, gray:false, actor:false }) };
    static PLAIN: RefCell<Option<Material>> = const { RefCell::new(None) };
    static ACTOR: RefCell<Option<Material>> = const { RefCell::new(None) };
}
pub struct Scope(State);
impl Drop for Scope {
    fn drop(&mut self) {
        STATE.set(self.0);
        restore();
    }
}
pub fn world(gray: bool) -> Scope {
    let old = STATE.replace(State {
        world: true,
        gray,
        actor: false,
    });
    restore();
    Scope(old)
}
pub fn color() -> Scope {
    let old = STATE.replace(State::default());
    gl_use_default_material();
    Scope(old)
}
pub fn actor(enabled: bool) -> Scope {
    let old = STATE.get();
    STATE.set(State {
        actor: enabled && old.world,
        ..old
    });
    Scope(old)
}
fn gray() -> f32 {
    if STATE.get().gray { 1. } else { 0. }
}
const VERTEX: &str = r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;varying highp vec2 uv;varying lowp vec4 tint;
void main(){uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#;
const PLAIN_FRAGMENT: &str = r#"#version 100
precision highp float;varying highp vec2 uv;varying lowp vec4 tint;uniform sampler2D Texture;
void main(){gl_FragColor=texture2D(Texture,uv)*tint;}"#;
const ACTOR_FRAGMENT: &str = r#"#version 100
precision highp float;varying highp vec2 uv;varying lowp vec4 tint;uniform sampler2D Texture;
uniform vec2 texel;
void main(){vec4 c=texture2D(Texture,uv);float l=dot(c.rgb,vec3(0.2126,0.7152,0.0722));
float rim=step(texture2D(Texture,uv-vec2(texel.x,0.0)).a,0.3765);
rim=max(rim,step(texture2D(Texture,uv-vec2(0.0,texel.y)).a,0.3765));
c.rgb=min(vec3(1.0),c.rgb+vec3((12.0/255.0)*(1.0-l+rim)*0.75));gl_FragColor=c*tint;}"#;

fn build(
    source: ShaderSource<'_>,
    mut params: MaterialParams,
    filter: bool,
) -> Result<Material, macroquad::Error> {
    let ShaderSource::Glsl { vertex, fragment } = source else {
        return load_material(source, params);
    };
    let mut fragment = fragment.to_owned();
    if filter {
        let end = fragment.rfind('}').expect("fragment main");
        fragment.insert_str(end,"gl_FragColor.rgb=mix(gl_FragColor.rgb,vec3(dot(gl_FragColor.rgb,vec3(0.2126,0.7152,0.0722))),WorldGray);");
    }
    let precision = fragment
        .find("precision ")
        .and_then(|p| fragment[p..].find(';').map(|i| p + i + 1))
        .expect("fragment precision");
    fragment.insert_str(precision, "uniform float WorldGray;");
    params
        .uniforms
        .push(UniformDesc::new("WorldGray", UniformType::Float1));
    load_material(
        ShaderSource::Glsl {
            vertex,
            fragment: &fragment,
        },
        params,
    )
}
pub fn material(
    source: ShaderSource<'_>,
    params: MaterialParams,
) -> Result<Material, macroquad::Error> {
    build(source, params, true)
}
// Lighting a framebuffer must preserve the already colored spell/marker layers.
pub fn lighting_material(
    source: ShaderSource<'_>,
    params: MaterialParams,
) -> Result<Material, macroquad::Error> {
    build(source, params, false)
}
pub fn use_material(material: &Material) {
    material.set_uniform("WorldGray", gray());
    gl_use_material(material);
}
pub fn restore() {
    if !STATE.get().gray {
        gl_use_default_material();
        return;
    }
    PLAIN.with_borrow_mut(|slot| {
        let m = slot.get_or_insert_with(|| {
            material(
                ShaderSource::Glsl {
                    vertex: VERTEX,
                    fragment: PLAIN_FRAGMENT,
                },
                MaterialParams {
                    pipeline_params: crate::animation_policy::blend(),
                    ..Default::default()
                },
            )
            .expect("world grayscale shader")
        });
        use_material(m);
    });
}
pub fn texture(texture: &Texture2D, x: f32, y: f32, color: Color, params: DrawTextureParams) {
    if !STATE.get().actor {
        draw_texture_ex(texture, x, y, color, params);
        return;
    }
    ACTOR.with_borrow_mut(|slot| {
        let m = slot.get_or_insert_with(|| {
            material(
                ShaderSource::Glsl {
                    vertex: VERTEX,
                    fragment: ACTOR_FRAGMENT,
                },
                MaterialParams {
                    uniforms: vec![UniformDesc::new("texel", UniformType::Float2)],
                    pipeline_params: crate::animation_policy::blend(),
                    ..Default::default()
                },
            )
            .expect("actor readability shader")
        });
        m.set_uniform("texel", (1. / texture.width(), 1. / texture.height()));
        use_material(m);
        draw_texture_ex(texture, x, y, color, params);
    });
    restore();
}
pub fn ground_marker(rect: Rect, player: bool) {
    let _color = color();
    let scale = rect.w / 64.;
    let center = vec2(rect.center().x, rect.bottom() - scale);
    let ink = if player {
        Color::from_rgba(80, 205, 220, 163)
    } else {
        Color::from_rgba(218, 86, 73, 172)
    };
    for start in [0.12, 1.12] {
        for i in 0..24 {
            let a = (start + i as f32 * 0.76 / 24.) * std::f32::consts::PI;
            let b = (start + (i + 1) as f32 * 0.76 / 24.) * std::f32::consts::PI;
            draw_line(
                center.x + 17. * scale * a.cos(),
                center.y + 5. * scale * a.sin(),
                center.x + 17. * scale * b.cos(),
                center.y + 5. * scale * b.sin(),
                scale,
                ink,
            );
        }
    }
}
