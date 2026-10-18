//! Approved cosmetic light/ambience pass. No simulation mutation or per-frame uploads.
use macroquad::{miniquad as mq, prelude::*};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Deserialize)]
pub struct Config {
    correction: String,
    irradiance: String,
    ambient: [f32; 3],
}

pub struct Atmosphere {
    camera: Camera2D,
    frame: RenderTarget,
    background: Material,
    composite: Material,
    animated_prop: Material,
    steam: Texture2D,
    // Material::set_texture retains the raw GPU handle, not its owner.
    _correction: Texture2D,
    _irradiance: Texture2D,
}

fn load(path: &str, filter: FilterMode) -> Texture2D {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path);
    let im = Image::from_file_with_format(&std::fs::read(path).unwrap(), None).unwrap();
    let t = Texture2D::from_image(&im);
    t.set_filter(filter);
    t
}

fn material(fragment: &str, textures: &[&str], uniforms: &[(&str, mq::UniformType)]) -> Material {
    load_material(
        ShaderSource::Glsl {
            vertex: VERTEX,
            fragment,
        },
        MaterialParams {
            textures: textures.iter().map(|s| s.to_string()).collect(),
            uniforms: uniforms
                .iter()
                .map(|(name, ty)| mq::UniformDesc::new(name, *ty))
                .collect(),
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
    .expect("approved atmosphere shader must compile")
}

impl Atmosphere {
    pub fn new(config: &Config, width: u32, height: u32) -> Self {
        let frame = render_target(width, height);
        frame.texture.set_filter(FilterMode::Nearest);
        let mut camera =
            Camera2D::from_display_rect(Rect::new(0., 0., width as f32, height as f32));
        camera.render_target = Some(frame.clone());
        let background = material(
            BACKGROUND,
            &["Correction"],
            &[("LightOn", mq::UniformType::Float1)],
        );
        let correction = load(&config.correction, FilterMode::Nearest);
        background.set_texture("Correction", correction.clone());
        let composite = material(
            COMPOSITE,
            &["Irradiance"],
            &[
                ("LightOn", mq::UniformType::Float1),
                ("Ambient", mq::UniformType::Float3),
            ],
        );
        let irradiance = load(&config.irradiance, FilterMode::Linear);
        composite.set_texture("Irradiance", irradiance.clone());
        composite.set_uniform(
            "Ambient",
            (config.ambient[0], config.ambient[1], config.ambient[2]),
        );
        let animated_prop = material(
            PROP,
            &[],
            &[
                ("Clock", mq::UniformType::Float1),
                ("Kind", mq::UniformType::Float1),
                ("Phase", mq::UniformType::Float1),
                ("Panel", mq::UniformType::Float4),
            ],
        );
        let mut plume = Image::gen_image_color(32, 32, BLANK);
        for y in 0..32 {
            for x in 0..32 {
                let d2 = ((x as f32 - 15.5) / 8.).powi(2) + ((y as f32 - 15.5) / 8.).powi(2);
                let a = ((-d2).exp() * 255.) as u8;
                let i = (y * 32 + x) * 4;
                plume.bytes[i..i + 4].copy_from_slice(&[175, 187, 185, a]);
            }
        }
        let steam = Texture2D::from_image(&plume);
        steam.set_filter(FilterMode::Linear);
        Self {
            camera,
            frame,
            background,
            composite,
            animated_prop,
            steam,
            _correction: correction,
            _irradiance: irradiance,
        }
    }

    pub fn begin(&self) {
        push_camera_state();
        set_camera(&self.camera);
        clear_background(BLACK);
    }

    pub fn begin_background(&self, lights: bool) {
        self.background
            .set_uniform("LightOn", if lights { 1.0_f32 } else { 0.0_f32 });
        gl_use_material(&self.background);
    }

    pub fn end_material(&self) {
        gl_use_default_material();
    }

    pub fn begin_prop(&self, id: &str, time: f32, phase: f32) {
        let (kind, panel): (f32, (f32, f32, f32, f32)) = match id {
            "DEC-CAMP-15" | "DEC-CAMP-16" => (1., (61., 12., 6., 2.)),
            "DEC-CAMP-07" => (2., (52., 57., 12., 6.)),
            "DEC-CAMP-13" => (2., (86., 54., 8., 4.)),
            _ => return,
        };
        self.animated_prop.set_uniform("Kind", kind);
        self.animated_prop.set_uniform("Clock", time);
        self.animated_prop.set_uniform("Phase", phase);
        self.animated_prop.set_uniform("Panel", panel);
        gl_use_material(&self.animated_prop);
    }

    pub fn steam(&self, foot: Vec2, time: f32) {
        let cycle = time.rem_euclid(6.);
        for i in 0..6 {
            let age = cycle - i as f32 * 0.19;
            if !(0.0..2.2).contains(&age) {
                continue;
            }
            let opacity = (age / 2.2 * std::f32::consts::PI).sin() * 0.25;
            let size = vec2(22. + age * 10., 13. + age * 6.);
            let p = foot + vec2(10. + (age * 3. + i as f32).sin() * 4., -5. - age * 24.);
            draw_texture_ex(
                &self.steam,
                p.x - size.x / 2.,
                p.y - size.y / 2.,
                Color::new(1., 1., 1., opacity),
                DrawTextureParams {
                    dest_size: Some(size),
                    ..Default::default()
                },
            );
        }
    }

    pub fn finish(&self, at: Vec2, lights: bool) {
        gl_use_default_material();
        pop_camera_state();
        self.composite
            .set_uniform("LightOn", if lights { 1.0_f32 } else { 0.0_f32 });
        gl_use_material(&self.composite);
        draw_texture_ex(
            &self.frame.texture,
            at.x,
            at.y,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
}

const VERTEX: &str = r#"#version 100
attribute vec3 position;
attribute vec2 texcoord;
attribute vec4 color0;
uniform mat4 Model;
uniform mat4 Projection;
varying highp vec2 uv;
varying lowp vec4 tint;
void main() {
    gl_Position = Projection * Model * vec4(position,1.0);
    uv=texcoord; tint=color0/255.0;
}"#;

const BACKGROUND: &str = r#"#version 100
precision highp float;
varying highp vec2 uv;
varying lowp vec4 tint;
uniform sampler2D Texture;
uniform sampler2D Correction;
uniform float LightOn;
void main() {
    vec4 ink=texture2D(Texture,uv);
    vec4 data=texture2D(Correction,uv);
    vec3 neutral=ink.rgb*(data.rgb*2.0);
    vec3 filament=mix(ink.rgb*0.13,ink.rgb,LightOn);
    ink.rgb=mix(neutral,filament,data.a);
    gl_FragColor=ink*tint;
}"#;

const COMPOSITE: &str = r#"#version 100
precision highp float;
varying highp vec2 uv;
varying lowp vec4 tint;
uniform sampler2D Texture;
uniform sampler2D Irradiance;
uniform float LightOn;
uniform vec3 Ambient;
void main() {
    vec4 ink=texture2D(Texture,uv);
    // Framebuffer images are vertically inverted relative to uploaded maps.
    vec3 light=texture2D(Irradiance,vec2(uv.x,1.0-uv.y)).rgb*2.0;
    // Dark void around the architecture must not reveal the coarse wall grid.
    float surface=smoothstep(0.07,0.17,max(ink.r,max(ink.g,ink.b)));
    ink.rgb*=mix(Ambient,light,LightOn*surface);
    // The complete world has an opaque backdrop; do not blend its shadows or
    // antialiased actor edges a second time when compositing the framebuffer.
    gl_FragColor=vec4(ink.rgb*tint.rgb,tint.a);
}"#;

const PROP: &str = r#"#version 100
precision highp float;
varying highp vec2 uv;
varying lowp vec4 tint;
uniform sampler2D Texture;
uniform float Clock;
uniform float Phase;
uniform float Kind;
uniform vec4 Panel;
void main() {
    vec4 ink=texture2D(Texture,uv);
    vec2 p=uv*128.0;
    bool inside=p.x>=Panel.x&&p.x<Panel.x+Panel.z&&p.y>=Panel.y&&p.y<Panel.y+Panel.w;
    if(inside) {
        if(Kind<1.5) {
            ink.rgb=vec3(1.0,0.72,0.26)*(0.65+0.35*sin(Clock*1.8+Phase));
        } else {
            float row=floor(mod(Clock*2.0+Phase,Panel.w));
            float scan=1.0-step(1.05,abs(p.y-(Panel.y+row+0.5)));
            ink.rgb=mix(ink.rgb,vec3(0.60,0.85,0.62),scan*0.75);
        }
    }
    gl_FragColor=ink*tint;
}"#;
