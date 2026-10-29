//! Approved cosmetic light/ambience pass. No simulation mutation or per-frame uploads.
use macroquad::{miniquad as mq, prelude::*};
use serde::Deserialize;
mod surface_wear;

#[derive(Deserialize)]
pub struct Config {
    correction: String,
    irradiance: String,
    ambient: [f32; 3],
    #[serde(default)]
    procedural_emitters: bool,
    #[serde(default)]
    colored_emitters: bool,
    #[serde(default)]
    fan: Option<[f32; 4]>,
    #[serde(default)]
    surface_wear: Option<String>,
}

impl Config {
    pub fn ambient_tint(&self) -> Color {
        Color::new(self.ambient[0], self.ambient[1], self.ambient[2], 1.)
    }
}

pub struct Atmosphere {
    frame: RenderTarget,
    background: Material,
    composite: Material,
    animated_prop: Material,
    steam: Texture2D,
    // Material::set_texture retains the raw GPU handle, not its owner.
    _correction: Texture2D,
    _irradiance: Texture2D,
    ambient: [f32; 3],
    surface: Option<surface_wear::Surface>,
    #[cfg(debug_assertions)]
    surface_override: std::cell::Cell<Option<bool>>,
}

fn load(path: &str, filter: FilterMode) -> Texture2D {
    let im =
        crate::asset_loading::image(&project_rl::data_paths::read(path).unwrap(), None).unwrap();
    let t = crate::asset_loading::from_image(&im);
    t.set_filter(filter);
    t
}

fn material(fragment: &str, textures: &[&str], uniforms: &[(&str, mq::UniformType)]) -> Material {
    let build = if fragment == COMPOSITE {
        crate::world_readability::lighting_material
    } else {
        crate::world_readability::material
    };
    build(
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
        let _loading =
            project_rl::loading_profile::Span::new("prepare_atmosphere", "atmosphere", 0);
        let target_loading = project_rl::loading_profile::Span::new(
            "render_target_create_cpu",
            "atmosphere framebuffer",
            width as usize * height as usize * 4,
        );
        let frame = render_target(width, height);
        drop(target_loading);
        frame.texture.set_filter(FilterMode::Nearest);
        let background = material(
            BACKGROUND,
            &["Correction"],
            &[
                ("LightOn", mq::UniformType::Float1),
                ("ProceduralEmitters", mq::UniformType::Float1),
                ("ColoredEmitters", mq::UniformType::Float1),
                ("Fan", mq::UniformType::Float4),
                ("Clock", mq::UniformType::Float1),
            ],
        );
        let correction = load(&config.correction, FilterMode::Nearest);
        background.set_texture("Correction", correction.clone());
        background.set_uniform(
            "ColoredEmitters",
            if config.colored_emitters {
                1.0_f32
            } else {
                0.0_f32
            },
        );
        let fan = config.fan.unwrap_or([0.; 4]);
        background.set_uniform("Fan", (fan[0], fan[1], fan[2], fan[3]));
        background.set_uniform("Clock", 0.0_f32);
        background.set_uniform(
            "ProceduralEmitters",
            if config.procedural_emitters {
                1.0_f32
            } else {
                0.0_f32
            },
        );
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
        let steam = crate::asset_loading::from_image(&plume);
        steam.set_filter(FilterMode::Linear);
        let surface = config.surface_wear.as_deref().map(|path| {
            surface_wear::Surface::new(
                correction.clone(),
                frame.texture.size(),
                surface_wear::Settings {
                    procedural: if config.procedural_emitters { 1. } else { 0. },
                    colored: if config.colored_emitters { 1. } else { 0. },
                    fan,
                },
                path,
            )
        });
        Self {
            frame,
            background,
            composite,
            animated_prop,
            steam,
            _correction: correction,
            _irradiance: irradiance,
            ambient: config.ambient,
            surface,
            #[cfg(debug_assertions)]
            surface_override: std::cell::Cell::new(None),
        }
    }

    pub fn begin(&self, world_rect: Rect) {
        push_camera_state();
        let mut camera = Camera2D::from_display_rect(world_rect);
        camera.render_target = Some(self.frame.clone());
        set_camera(&camera);
        clear_background(BLACK);
        crate::world_readability::begin_lighting(
            &self._irradiance,
            self.frame.texture.size(),
            self.ambient,
            true,
        );
    }

    pub fn begin_background(&self, lights: bool) {
        crate::world_readability::lighting_enabled(lights);
        if let Some(surface) = self.active_surface() {
            surface
                .background
                .set_uniform("LightOn", if lights { 1.0_f32 } else { 0.0_f32 });
            crate::world_readability::use_surface_material(
                &surface.background,
                Some(self.frame.texture.size()),
            );
            return;
        }
        self.background
            .set_uniform("LightOn", if lights { 1.0_f32 } else { 0.0_f32 });
        crate::world_readability::use_surface_material(
            &self.background,
            Some(self.frame.texture.size()),
        );
    }

    pub fn background_time(&self, time: f32) {
        self.background.set_uniform("Clock", time);
        if let Some(surface) = &self.surface {
            surface.background.set_uniform("Clock", time);
        }
    }

    #[cfg(debug_assertions)]
    pub fn review_surface(&self, enabled: Option<bool>) {
        self.surface_override.set(enabled);
    }

    fn active_surface(&self) -> Option<&surface_wear::Surface> {
        #[cfg(debug_assertions)]
        if self.surface_override.get() == Some(false) {
            return None;
        }
        self.surface.as_ref()
    }

    pub fn begin_surface(&self, size: Vec2) {
        if let Some(surface) = self.active_surface() {
            surface.isolated.set_uniform("ReviewSize", (size.x, size.y));
            crate::world_readability::use_surface_material(&surface.isolated, Some(size));
        }
    }

    /// A native-size overview from the same world framebuffer, before the UI's
    /// six-cell crop. Actors and simulation visibility are never expanded.
    #[cfg(debug_assertions)]
    pub fn export_review_world(
        &self,
        path: &std::path::Path,
        draw_exteriors: impl FnOnce(),
    ) -> Result<(), String> {
        let size = self.frame.texture.size();
        let target = render_target(size.x as u32, size.y as u32);
        target.texture.set_filter(FilterMode::Nearest);
        unsafe {
            macroquad::window::get_internal_gl().quad_gl.scissor(None);
        }
        push_camera_state();
        let mut camera = Camera2D::from_display_rect(Rect::new(0., 0., size.x, size.y));
        camera.render_target = Some(target.clone());
        set_camera(&camera);
        clear_background(BLACK);
        self.composite.set_uniform("LightOn", 1.0_f32);
        crate::world_readability::use_material(&self.composite);
        draw_texture_ex(
            &self.frame.texture,
            0.,
            0.,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()
            },
        );
        crate::world_readability::restore();
        draw_exteriors();
        unsafe {
            macroquad::window::get_internal_gl().flush();
        }
        let mut image = target.texture.get_texture_data();
        for pixel in image.bytes.as_chunks_mut::<4>().0 {
            pixel[3] = 255;
        }
        image.export_png(path.to_str().ok_or("Invalid overview path")?);
        pop_camera_state();
        Ok(())
    }

    pub fn end_material(&self) {
        crate::world_readability::restore();
    }
    #[cfg(debug_assertions)]
    pub fn review_ambient(&self, color: [f32; 3]) {
        self.composite.set_uniform("Ambient", color);
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
        crate::world_readability::use_surface_material(&self.animated_prop, None);
    }

    pub fn steam(&self, foot: Vec2, time: f32) {
        let _color = crate::world_readability::color();
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

    pub fn finish(&self, rect: Rect, clip: Rect, lights: bool) {
        crate::world_readability::restore();
        pop_camera_state();
        unsafe {
            macroquad::window::get_internal_gl().quad_gl.scissor(Some((
                clip.x as i32,
                clip.y as i32,
                clip.w as i32,
                clip.h as i32,
            )));
        }
        self.composite
            .set_uniform("LightOn", if lights { 1.0_f32 } else { 0.0_f32 });
        crate::world_readability::use_material(&self.composite);
        draw_texture_ex(
            &self.frame.texture,
            rect.x,
            rect.y,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                dest_size: Some(rect.size()),
                ..Default::default()
            },
        );
        crate::world_readability::end_lighting();
        crate::world_readability::restore();
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
uniform float ProceduralEmitters;
uniform float ColoredEmitters;
uniform vec4 Fan;
uniform float Clock;
void main() {
    vec2 sample_uv=uv;
    if(Fan.z>0.0&&Fan.w>0.0) {
        vec2 p=(uv-Fan.xy)/Fan.zw;
        if(dot(p,p)<1.0) {
            float a=Clock*0.45;
            p=mat2(cos(a),-sin(a),sin(a),cos(a))*p;
            sample_uv=Fan.xy+p*Fan.zw;
        }
    }
    vec4 ink=texture2D(Texture,sample_uv);
    vec4 data=texture2D(Correction,uv);
    vec3 neutral=ink.rgb*(data.rgb*2.0);
    vec3 filament=mix(ink.rgb*0.13,ink.rgb,LightOn);
    if(ProceduralEmitters>0.5) {
        filament=mix(ink.rgb*0.13,vec3(1.0,0.82,0.46),LightOn);
        if(ColoredEmitters>0.5) filament=mix(ink.rgb*0.13,data.rgb,LightOn);
    }
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
    vec3 illumination=mix(Ambient,light,LightOn*surface);
    illumination=mix(illumination,vec3(dot(illumination,vec3(0.2126,0.7152,0.0722))),WorldGray);
    ink.rgb*=illumination;
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
