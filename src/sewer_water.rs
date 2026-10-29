//! EG-EAU01 / EG-CAN02: accepted continuous water, banks and walkable grates.
use crate::{sewer::Plan, terminal_view::GridCamera};
use macroquad::{
    miniquad::{UniformDesc, UniformType},
    prelude::*,
};
use project_rl::world::GridPos;
use std::cell::RefCell;
thread_local! {
    static WATER: RefCell<Option<(Texture2D,Material)>> = const {RefCell::new(None)};
    static BANKS: RefCell<Option<(Texture2D,Texture2D)>> = const {RefCell::new(None)};
}
fn added_banks(camera: &GridCamera, base: &Plan, plan: &Plan, bridges: bool) {
    BANKS.with_borrow_mut(|slot| {
        let (bank, grate) = slot.get_or_insert_with(|| {
            let bank = crate::asset_loading::from_image(&Image::gen_image_color(
                1,
                1,
                Color::from_rgba(90, 88, 70, 255),
            ));
            let bytes =
                project_rl::data_paths::read("assets/maps/prison-sewer-canals-02/grate.png")
                    .unwrap();
            let grate = crate::asset_loading::texture(&bytes, Some(ImageFormat::Png));
            grate.set_filter(FilterMode::Nearest);
            (bank, grate)
        });
        for (y, row) in plan.layout.iter().enumerate() {
            for (x, c) in row.bytes().enumerate() {
                if base.layout[y].as_bytes()[x] == c
                    || !matches!(c, b'~' | b'=')
                    || bridges && c != b'='
                {
                    continue;
                }
                let r = camera.rect(GridPos::new(x as i32, y as i32));
                crate::world_readability::texture(
                    if bridges { grate } else { bank },
                    r.x,
                    r.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(r.size()),
                        ..Default::default()
                    },
                );
            }
        }
    });
}

fn channel(plan: &Plan, x: i32, y: i32) -> bool {
    usize::try_from(y)
        .ok()
        .and_then(|y| plan.layout.get(y))
        .and_then(|row| usize::try_from(x).ok().and_then(|x| row.as_bytes().get(x)))
        .is_some_and(|c| matches!(c, b'~' | b'='))
}

pub fn draw(camera: &GridCamera, base: &Plan, plan: &Plan, time: f32) {
    let expanded = base.layout != plan.layout;
    if expanded {
        added_banks(camera, base, plan, false);
    }
    WATER.with_borrow_mut(|bank| {
        let (texture, material) = bank.get_or_insert_with(|| {
            let bytes =
                project_rl::data_paths::read("assets/maps/prison-sewer-01/water.png").unwrap();
            let texture = crate::asset_loading::texture(&bytes, Some(ImageFormat::Png));
            texture.set_filter(FilterMode::Nearest);
            let material = crate::world_readability::material(
                ShaderSource::Glsl {
                    vertex: VERTEX,
                    fragment: FRAGMENT,
                },
                MaterialParams {
                    uniforms: vec![
                        UniformDesc::new("Clock", UniformType::Float1),
                        UniformDesc::new("Cell", UniformType::Float2),
                        UniformDesc::new("Banks", UniformType::Float4),
                    ],
                    ..Default::default()
                },
            )
            .expect("EG-EAU01 water shader");
            (texture, material)
        });
        material.set_uniform("Clock", time);
        for (y, row) in plan.layout.iter().enumerate() {
            for (x, c) in row.bytes().enumerate() {
                if c != b'~' {
                    continue;
                }
                let at = GridPos::new(x as i32, y as i32);
                let r = camera.rect(at);
                // Leave the accepted 4 px bank only at a real channel edge.
                // The bridge itself stays in the opaque authored background.
                let banks = [
                    !channel(plan, at.x - 1, at.y),
                    !channel(plan, at.x + 1, at.y),
                    !channel(plan, at.x, at.y - 1),
                    !channel(plan, at.x, at.y + 1),
                ];
                let [left, right, top, bottom] = banks.map(|bank| if bank { 4. } else { 0. });
                material.set_uniform("Cell", (x as f32, y as f32));
                material.set_uniform(
                    "Banks",
                    (
                        banks[0] as u8 as f32,
                        banks[1] as u8 as f32,
                        banks[2] as u8 as f32,
                        banks[3] as u8 as f32,
                    ),
                );
                crate::world_readability::use_surface_material(material, Some(texture.size()));
                draw_texture_ex(
                    texture,
                    r.x + left * r.w / 64.,
                    r.y + top * r.h / 64.,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(
                            (x % 4) as f32 * 64. + left,
                            (y % 4) as f32 * 64. + top,
                            64. - left - right,
                            64. - top - bottom,
                        )),
                        dest_size: Some(vec2(
                            (64. - left - right) * r.w / 64.,
                            (64. - top - bottom) * r.h / 64.,
                        )),
                        ..Default::default()
                    },
                );
            }
        }
        crate::world_readability::restore();
    });
    if expanded {
        added_banks(camera, base, plan, true);
    }
}

const VERTEX: &str = r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;varying highp vec2 uv;
void main(){uv=texcoord;gl_Position=Projection*Model*vec4(position,1.0);}"#;
const FRAGMENT: &str = r#"#version 100
precision highp float;varying highp vec2 uv;uniform sampler2D Texture;
uniform float Clock;uniform vec2 Cell;uniform vec4 Banks;
void main(){
 vec2 p=floor(uv*256.0)+floor(Cell/4.0)*256.0;
 vec2 local=p-Cell*64.0;
 // One shared phase across cells; the banks and bridge never move.
 vec2 flow=vec2(sin(p.y*0.065-Clock*0.65)*0.65,Clock*1.35);
 vec3 ink=texture2D(Texture,fract((p+flow)/256.0)).rgb;
 float l=dot(ink,vec3(0.2126,0.7152,0.0722));
 vec3 water=vec3(0.080,0.108,0.078)+mix(vec3(l),ink,0.65)*0.48;
 float crest=sin(p.x*0.19+sin(p.y*0.075)*1.7-Clock*0.95);
 float broken=0.35+0.65*smoothstep(-0.2,0.7,sin(p.y*0.22+p.x*0.043+Clock*0.37));
 water+=vec3(0.032,0.038,0.022)*smoothstep(0.90,1.0,crest)*broken;
 float shore=min(min(mix(1000.0,local.x-4.0,Banks.x),mix(1000.0,59.0-local.x,Banks.y)),
                 min(mix(1000.0,local.y-4.0,Banks.z),mix(1000.0,59.0-local.y,Banks.w)));
 water*=mix(0.58,1.0,smoothstep(0.0,4.0,shore));
 water+=vec3(0.008,0.014,0.003)*(1.0-smoothstep(1.0,2.0,shore));
 gl_FragColor=vec4(water,1.0);
}"#;
