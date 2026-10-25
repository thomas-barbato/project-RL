//! Shared accepted sewer water and tiny ambient rats; presentation only.
use crate::{
    sewer::{Plan, RatRoute},
    terminal_view::GridCamera,
};
use macroquad::prelude::*;
use project_rl::world::GridPos;
use std::cell::RefCell;
thread_local! {
    static WATER:RefCell<Option<(Texture2D,Material)>>=const {RefCell::new(None)};
    static RATS:RefCell<Option<Texture2D>>=const {RefCell::new(None)};
}
pub fn water(camera: &GridCamera, plan: &Plan, time: f32) {
    WATER.with(|bank| {
        let mut bank = bank.borrow_mut();
        let (texture, material) = bank.get_or_insert_with(|| {
            let bytes =
                project_rl::data_paths::read("assets/maps/prison-sewer-01/water.png").unwrap();
            let t = Texture2D::from_file_with_format(&bytes, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Nearest);
            let material = crate::world_readability::material(
                ShaderSource::Glsl {
                    vertex: VERTEX,
                    fragment: FRAGMENT,
                },
                MaterialParams {
                    uniforms: vec![macroquad::miniquad::UniformDesc::new(
                        "Clock",
                        macroquad::miniquad::UniformType::Float1,
                    )],
                    ..Default::default()
                },
            )
            .unwrap();
            (t, material)
        });
        material.set_uniform("Clock", time);
        crate::world_readability::use_material(material);
        for (y, row) in plan.layout.iter().enumerate() {
            for (x, c) in row.bytes().enumerate() {
                if c != b'~' {
                    continue;
                }
                let rect = camera.rect(GridPos::new(x as i32, y as i32));
                draw_texture_ex(
                    texture,
                    rect.x + 4. * rect.w / 64.,
                    rect.y + 4. * rect.h / 64.,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(
                            (x % 4) as f32 * 64. + 4.,
                            (y % 4) as f32 * 64. + 4.,
                            56.,
                            56.,
                        )),
                        dest_size: Some(vec2(rect.w * 56. / 64., rect.h * 56. / 64.)),
                        ..Default::default()
                    },
                );
            }
        }
        crate::world_readability::restore();
    });
}
pub fn rats(camera: &GridCamera, plan: &Plan, time: f32) {
    RATS.with(|bank| {
        let mut bank = bank.borrow_mut();
        let texture = bank.get_or_insert_with(|| {
            let bytes =
                project_rl::data_paths::read("assets/maps/prison-sewer-01/rats.png").unwrap();
            let t = Texture2D::from_file_with_format(&bytes, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Nearest);
            t
        });
        let origin = camera.rect(GridPos::new(0, 0));
        let scale = origin.w / 64.;
        crate::world_readability::restore();
        for rat in &plan.rats {
            let Some((p, angle, running)) = rat_pose(rat, time) else {
                continue;
            };
            let at = vec2(origin.x + p[0] * scale, origin.y + p[1] * scale);
            draw_ellipse(
                at.x,
                at.y + 3. * scale,
                4. * scale,
                1.5 * scale,
                0.,
                Color::new(0., 0., 0., 0.22),
            );
            let frame = if running {
                ((time * 12. + rat.phase) as u32 % 4) as f32
            } else {
                0.
            };
            draw_texture_ex(
                texture,
                at.x - 10. * scale,
                at.y - 6. * scale,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect::new(frame * 20., 0., 20., 12.)),
                    dest_size: Some(vec2(20., 12.) * scale),
                    rotation: angle,
                    pivot: Some(at),
                    ..Default::default()
                },
            );
        }
    });
}
fn rat_pose(rat: &RatRoute, time: f32) -> Option<([f32; 2], f32, bool)> {
    let segments: Vec<_> = rat
        .path
        .windows(2)
        .map(|p| {
            let a = vec2(p[0][0], p[0][1]);
            let b = vec2(p[1][0], p[1][1]);
            (a, b, a.distance(b))
        })
        .collect();
    let total: f32 = segments.iter().map(|p| p.2).sum();
    if total <= 0. || rat.speed <= 0. {
        return None;
    }
    let pause = rat.speed * 0.7;
    let position = (time * rat.speed + rat.phase).rem_euclid(2. * total + 2. * pause);
    let (mut distance, reverse, running) = if position < total {
        (position, false, true)
    } else if position < total + pause {
        (total, false, false)
    } else if position < 2. * total + pause {
        (2. * total + pause - position, true, true)
    } else {
        (0., true, false)
    };
    for (i, (a, b, length)) in segments.iter().enumerate() {
        if distance <= *length || i + 1 == segments.len() {
            let p = a.lerp(*b, (distance / length.max(0.001)).clamp(0., 1.));
            let direction = if reverse { *a - *b } else { *b - *a };
            return Some(([p.x, p.y], direction.y.atan2(direction.x), running));
        }
        distance -= length;
    }
    None
}
const VERTEX: &str = r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;varying highp vec2 uv;
void main(){gl_Position=Projection*Model*vec4(position,1.0);uv=texcoord;}"#;
const FRAGMENT: &str = r#"#version 100
precision highp float;varying highp vec2 uv;uniform sampler2D Texture;uniform float Clock;
void main(){vec2 p=uv+vec2(sin(uv.y*35.0+Clock*0.8)*0.0015,Clock*0.003);
gl_FragColor=vec4(texture2D(Texture,fract(p)).rgb,1.0);}"#;
