//! ACTOR-READABILITY01: world-only grading; effects and UI keep their colors.
use macroquad::{
    miniquad::{UniformDesc, UniformType},
    prelude::*,
};
use std::cell::{Cell, RefCell};

#[cfg(debug_assertions)]
#[path = "player_exposure_review.rs"]
pub(crate) mod player_exposure_review;

#[cfg(debug_assertions)]
#[derive(serde::Serialize)]
pub(crate) struct TextureAudit {
    pub world: bool,
    pub actor: bool,
    pub finish: bool,
    pub gain: f32,
    pub lift: f32,
    pub edge: f32,
    pub texture_size: [f32; 2],
}
#[cfg(debug_assertions)]
thread_local! {static AUDIT:RefCell<Option<Vec<TextureAudit>>>=const{RefCell::new(None)};}
#[cfg(debug_assertions)]
pub(crate) fn audit_textures(draw: impl FnOnce()) -> Vec<TextureAudit> {
    let previous = AUDIT.with_borrow_mut(|slot| slot.replace(Vec::new()));
    draw();
    AUDIT.with_borrow_mut(|slot| {
        let rows = slot.take().unwrap();
        *slot = previous;
        rows
    })
}

#[derive(Clone, Copy, Default)]
struct State {
    world: bool,
    gray: bool,
    actor: bool,
}
#[derive(Clone, Copy)]
struct Exposure {
    gain: f32,
    lift: f32,
    edge: f32,
}
const LEGACY_EXPOSURE: Exposure = Exposure {
    gain: 1.48,
    lift: 0.75,
    edge: 0.035,
};
// LIGHT01-r01 / 01 « Discret », selected on 1 November 2026.
const PLAYER_EXPOSURE: Exposure = Exposure {
    gain: 1.18,
    lift: 0.30,
    edge: 0.012,
};
thread_local! {
    static STATE: Cell<State> = const { Cell::new(State { world:false, gray:false, actor:false }) };
    static PLAYER: Cell<bool> = const { Cell::new(false) };
    static PLAIN: RefCell<Option<Material>> = const { RefCell::new(None) };
    static ACTOR: RefCell<Option<Material>> = const { RefCell::new(None) };
    static LIGHT: RefCell<Option<Lighting>> = const { RefCell::new(None) };
    static NEUTRAL: RefCell<Option<Texture2D>> = const { RefCell::new(None) };
}
pub(crate) struct PlayerScope(bool);
impl Drop for PlayerScope {
    fn drop(&mut self) {
        PLAYER.set(self.0);
    }
}
pub(crate) fn player_scope(player: bool) -> PlayerScope {
    PlayerScope(PLAYER.replace(player))
}
fn actor_exposure() -> Exposure {
    if PLAYER.get() && STATE.get().world && STATE.get().actor && crate::visual_polish::enabled() {
        #[cfg(debug_assertions)]
        if let Some(profile) = player_exposure_review::override_exposure() {
            return profile;
        }
        PLAYER_EXPOSURE
    } else {
        LEGACY_EXPOSURE
    }
}
fn actor_gain(exposure: Exposure) -> f32 {
    // Generated unlit maps already draw actors at their source exposure.
    LIGHT.with_borrow(|light| if light.is_some() { exposure.gain } else { 1. })
}
struct Lighting {
    texture: Texture2D,
    size: Vec2,
    ambient: [f32; 3],
    enabled: bool,
}
pub fn begin_lighting(texture: &Texture2D, size: Vec2, ambient: [f32; 3], enabled: bool) {
    LIGHT.with_borrow_mut(|slot| {
        *slot = Some(Lighting {
            texture: texture.clone(),
            size,
            ambient,
            enabled,
        })
    });
}
pub fn end_lighting() {
    LIGHT.with_borrow_mut(|slot| *slot = None);
}
pub fn lighting_enabled(enabled: bool) {
    LIGHT.with_borrow_mut(|slot| {
        if let Some(light) = slot {
            light.enabled = enabled;
        }
    });
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
uniform float PolishActor;
uniform float PolishActorGain;
uniform float ActorLift;
uniform float ActorEdge;
void main(){vec4 c=texture2D(Texture,uv);float l=dot(c.rgb,vec3(0.2126,0.7152,0.0722));
float rim=step(texture2D(Texture,uv-vec2(texel.x,0.0)).a,0.3765);
rim=max(rim,step(texture2D(Texture,uv-vec2(0.0,texel.y)).a,0.3765));
c.rgb=min(vec3(1.0),c.rgb+vec3((12.0/255.0)*(1.0-l+rim)*ActorLift));
if(PolishActor>0.5){
 float solid=smoothstep(0.04,0.15,dot(c.rgb,vec3(0.2126,0.7152,0.0722)))*c.a;
 vec3 pigment=c.rgb/max(max(c.r,max(c.g,c.b)),0.001);
 c.rgb=c.rgb*PolishActorGain+solid*rim*pigment*ActorEdge;
}
gl_FragColor=c*tint;}"#;

// Called on the graphics thread before the first displayed frame. One cached
// material serves all actors; only the player's approved coefficients differ.
pub(crate) fn prepare_actor_material() -> Result<(), macroquad::Error> {
    ACTOR.with_borrow_mut(|slot| {
        if slot.is_none() {
            *slot = Some(build(
                ShaderSource::Glsl {
                    vertex: VERTEX,
                    fragment: ACTOR_FRAGMENT,
                },
                MaterialParams {
                    uniforms: vec![
                        UniformDesc::new("texel", UniformType::Float2),
                        UniformDesc::new("PolishActor", UniformType::Float1),
                        UniformDesc::new("PolishActorGain", UniformType::Float1),
                        UniformDesc::new("ActorLift", UniformType::Float1),
                        UniformDesc::new("ActorEdge", UniformType::Float1),
                    ],
                    pipeline_params: crate::animation_policy::blend(),
                    ..Default::default()
                },
                true,
                false,
            )?);
        }
        Ok(())
    })
}

fn build(
    source: ShaderSource<'_>,
    mut params: MaterialParams,
    filter: bool,
    polish: bool,
) -> Result<Material, macroquad::Error> {
    let _loading = project_rl::loading_profile::Span::caller("shader_build_cpu", 0);
    let ShaderSource::Glsl { vertex, fragment } = source else {
        return load_material(source, params);
    };
    let mut fragment = fragment.to_owned();
    if polish {
        let main = fragment.find("void main()").expect("fragment main");
        fragment.insert_str(main, crate::visual_polish::SURFACE_GLSL);
        let end = fragment.rfind('}').expect("fragment main");
        fragment.insert_str(
            end,
            "if(PolishOn>0.5){gl_FragColor.rgb=polish_surface(gl_FragColor.rgb,uv);}",
        );
    }
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
    if polish {
        params.uniforms.extend([
            UniformDesc::new("PolishOn", UniformType::Float1),
            UniformDesc::new("PolishLit", UniformType::Float1),
            UniformDesc::new("PolishWorldSize", UniformType::Float2),
            UniformDesc::new("PolishAmbient", UniformType::Float3),
            UniformDesc::new("PolishLightOn", UniformType::Float1),
            UniformDesc::new("PolishTexel", UniformType::Float2),
        ]);
        params.textures.push("PolishLight".into());
    }
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
    build(source, params, true, true)
}
pub fn mask_material(
    source: ShaderSource<'_>,
    params: MaterialParams,
) -> Result<Material, macroquad::Error> {
    build(source, params, true, false)
}
// Lighting a framebuffer must preserve the already colored spell/marker layers.
pub fn lighting_material(
    source: ShaderSource<'_>,
    params: MaterialParams,
) -> Result<Material, macroquad::Error> {
    build(source, params, false, false)
}
pub fn use_material(material: &Material) {
    material.set_uniform("WorldGray", gray());
    gl_use_material(material);
}
pub fn use_surface_material(material: &Material, size: Option<Vec2>) {
    material.set_uniform(
        "PolishOn",
        if STATE.get().world && !STATE.get().actor && crate::visual_polish::enabled() {
            1.0_f32
        } else {
            0.0_f32
        },
    );
    let texel = size.map_or(Vec2::ZERO, |s| vec2(1. / s.x, 1. / s.y));
    material.set_uniform("PolishTexel", (texel.x, texel.y));
    LIGHT.with_borrow(|light| {
        if let Some(light) = light {
            material.set_texture("PolishLight", light.texture.clone());
            material.set_uniform("PolishLit", 1.0_f32);
            material.set_uniform("PolishWorldSize", (light.size.x, light.size.y));
            material.set_uniform("PolishAmbient", light.ambient);
            material.set_uniform(
                "PolishLightOn",
                if light.enabled { 1.0_f32 } else { 0.0_f32 },
            );
        } else {
            NEUTRAL.with_borrow_mut(|slot| {
                let texture = slot.get_or_insert_with(|| {
                    Texture2D::from_image(&Image::gen_image_color(1, 1, WHITE))
                });
                material.set_texture("PolishLight", texture.clone());
            });
            material.set_uniform("PolishLit", 0.0_f32);
            material.set_uniform("PolishWorldSize", (1.0_f32, 1.0_f32));
            material.set_uniform("PolishAmbient", [1.0_f32; 3]);
            material.set_uniform("PolishLightOn", 0.0_f32);
        }
    });
    use_material(material);
}
pub fn restore() {
    if !STATE.get().gray && !(STATE.get().world && crate::visual_polish::enabled()) {
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
        use_surface_material(m, None);
    });
}
pub fn texture(texture: &Texture2D, x: f32, y: f32, color: Color, params: DrawTextureParams) {
    #[cfg(debug_assertions)]
    AUDIT.with_borrow_mut(|slot| {
        if let Some(rows) = slot {
            let state = STATE.get();
            rows.push(TextureAudit {
                world: state.world,
                actor: state.actor,
                finish: crate::visual_polish::enabled(),
                gain: actor_gain(actor_exposure()),
                lift: actor_exposure().lift,
                edge: actor_exposure().edge,
                texture_size: [texture.width(), texture.height()],
            });
        }
    });
    if !STATE.get().actor {
        let polish = STATE.get().world && crate::visual_polish::enabled();
        if polish {
            // Plain world textures (including generated floors) know their
            // actual atlas size here; do not resample at a guessed tile size.
            restore();
            PLAIN.with_borrow(|slot| {
                if let Some(m) = slot {
                    use_surface_material(m, Some(texture.size()));
                }
            });
        }
        draw_texture_ex(texture, x, y, color, params);
        if polish {
            restore();
        }
        return;
    }
    let exposure = actor_exposure();
    ACTOR.with_borrow(|slot| {
        let m = slot
            .as_ref()
            .expect("actor material prepared before display");
        m.set_uniform("texel", (1. / texture.width(), 1. / texture.height()));
        m.set_uniform(
            "PolishActor",
            if STATE.get().world && crate::visual_polish::enabled() {
                1.0_f32
            } else {
                0.0_f32
            },
        );
        m.set_uniform("PolishActorGain", actor_gain(exposure));
        m.set_uniform("ActorLift", exposure.lift);
        m.set_uniform("ActorEdge", exposure.edge);
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
