//! Accepted A01-A04 art and purple grading, driven only by presentation time.
use crate::effect_geometry::LazyTexture;
use macroquad::prelude::*;
use project_rl::{entity::EntityId, world::GridPos};
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub(super) struct SpellFlight {
    pub victim: EntityId,
    pub origin: GridPos,
    pub target: GridPos,
    pub started: f64,
}
struct Art {
    bolt: LazyTexture,
    burst: LazyTexture,
    purple: Material,
}
thread_local! { static ART: RefCell<Option<Art>> = const { RefCell::new(None) }; }
impl Art {
    fn load() -> Self {
        let bolt = LazyTexture::fitted(
            include_bytes!("../assets/animations/agent/skill-animation-review-29/bolt-atlas.png"),
            FilterMode::Linear,
        );
        let purple = load_material(
            ShaderSource::Glsl {
                vertex: r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;varying highp vec2 uv;varying lowp vec4 tint;
void main(){uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#,
                fragment: r#"#version 100
precision highp float;varying highp vec2 uv;varying lowp vec4 tint;uniform sampler2D Texture;
void main(){vec4 colour=texture2D(Texture,uv)*tint;
float violet=smoothstep(0.007,0.055,colour.b-colour.g)*step(colour.r*0.82,colour.b);
float visible=smoothstep(0.022,0.12,max(colour.r,colour.b));
float peak=smoothstep(0.35,0.75,max(colour.r,colour.b));
vec3 purple=mix(vec3(0.65,0.15,1.0),vec3(0.80,0.30,1.0),peak);
colour.rgb=mix(colour.rgb,purple,violet*visible*0.96);gl_FragColor=colour;}"#,
            },
            MaterialParams {
                pipeline_params: macroquad::miniquad::PipelineParams {
                    color_blend: Some(macroquad::miniquad::BlendState::new(
                        macroquad::miniquad::Equation::Add,
                        macroquad::miniquad::BlendFactor::Value(
                            macroquad::miniquad::BlendValue::SourceAlpha,
                        ),
                        macroquad::miniquad::BlendFactor::OneMinusValue(
                            macroquad::miniquad::BlendValue::SourceAlpha,
                        ),
                    )),
                    ..Default::default()
                },
                ..Default::default()
            },
        )
        .expect("approved occult purple shader");
        let burst = LazyTexture::fitted(
            include_bytes!("../assets/animations/agent/skill-animation-review-29/burst-atlas.png"),
            FilterMode::Linear,
        );
        Self {
            bolt,
            burst,
            purple,
        }
    }
}
pub(super) fn draw(
    flights: &[SpellFlight],
    now: f64,
    reduced: bool,
    rect: impl Fn(GridPos) -> Option<Rect>,
) {
    if flights.is_empty() {
        return;
    }
    ART.with(|cache| {
        let mut cache = cache.borrow_mut();
        let art = cache.get_or_insert_with(Art::load);
        gl_use_material(&art.purple);
        for flight in flights {
            let age = (now - flight.started) as f32;
            if !(0.0..0.4).contains(&age) {
                continue;
            }
            let (Some(from), Some(to)) = (rect(flight.origin), rect(flight.target)) else {
                continue;
            };
            let scale = from.w / 64.;
            let origin = from.center() - vec2(0., 8. * scale);
            let target = to.center() - vec2(0., 8. * scale);
            let heading = target - origin;
            let (frame, at, anchor, size, opacity) = if age < 0.18 && !reduced {
                (
                    ((age / 0.18 * 3.) as usize).min(2),
                    origin + heading * (age / 0.18),
                    vec2(232., 136.),
                    72.,
                    0.98,
                )
            } else {
                let t = if reduced { 0.05 } else { age - 0.18 };
                (
                    3 + ((t / 0.22 * 3.) as usize).min(2),
                    target,
                    vec2(128., 128.),
                    64.,
                    0.9 * ((0.22 - t) / 0.06).clamp(0., 1.),
                )
            };
            let size = size * scale;
            if crate::effect_geometry::enabled() && frame >= 3 {
                crate::effect_geometry::fitted_cells(
                    &art.bolt,
                    frame,
                    &[flight.target],
                    &rect,
                    opacity,
                );
                continue;
            }
            draw_texture_ex(
                &art.bolt,
                at.x - anchor.x * size / 256.,
                at.y - anchor.y * size / 256.,
                Color::new(1., 1., 1., opacity),
                DrawTextureParams {
                    source: Some(Rect::new(frame as f32 * 256., 0., 256., 256.)),
                    dest_size: Some(vec2(size, size)),
                    rotation: heading.y.atan2(heading.x),
                    pivot: Some(at),
                    ..Default::default()
                },
            );
        }
        gl_use_default_material();
    });
}

#[derive(Clone, Debug)]
pub(super) struct SpellBlast {
    pub centre: GridPos,
    pub radius: u16,
    pub started: f64,
    pub cells: Option<Vec<GridPos>>,
}
/// Floor/obstacle clipping only: the painted image can spill beyond nominal area edges.
pub(super) fn draw_blasts(
    blasts: &[SpellBlast],
    map: &project_rl::world::Map,
    now: f64,
    reduced: bool,
    background: bool,
    rect: impl Fn(GridPos) -> Option<Rect>,
) {
    if blasts.is_empty() {
        return;
    }
    ART.with(|cache| {
        let mut cache = cache.borrow_mut();
        let art = cache.get_or_insert_with(Art::load);
        gl_use_material(&art.purple);
        for blast in blasts {
            let age = (now - blast.started) as f32;
            if !(0.0..0.55).contains(&age) {
                continue;
            }
            let age = if reduced { 0.20 } else { age };
            let frame = ((age / 0.55 * 6.) as usize).min(5);
            if (frame == 0) != background {
                continue;
            }
            let Some(cell) = rect(blast.centre) else {
                continue;
            };
            if crate::effect_geometry::enabled() {
                let fallback;
                let cells = if let Some(cells) = &blast.cells {
                    cells.as_slice()
                } else {
                    fallback = project_rl::skills::spatial::affected_cells(
                        map,
                        blast.centre,
                        project_rl::skills::spatial::Area::Square {
                            radius: blast.radius,
                        },
                        (1, 0),
                    );
                    &fallback
                };
                crate::effect_geometry::fitted_cells(
                    &art.burst,
                    frame,
                    cells,
                    &rect,
                    0.83 * ((0.55 - age) / 0.10).clamp(0., 1.),
                );
                continue;
            }
            let size = 224. * cell.w / 64.
                * f32::from(blast.radius.saturating_mul(2).saturating_add(1))
                / 3.;
            let anchor = if frame < 2 {
                vec2(128., 170.)
            } else {
                vec2(128., 128.)
            };
            let rise = 18. * (1. - (age / 0.22).clamp(0., 1.)) * cell.w / 64.;
            let at = cell.center() + vec2(0., rise);
            let whole = Rect::new(
                at.x - anchor.x * size / 256.,
                at.y - anchor.y * size / 256.,
                size,
                size,
            );
            let extent = i32::from(blast.radius) + 2;
            for y in blast.centre.y.saturating_sub(extent).max(0)
                ..=blast
                    .centre
                    .y
                    .saturating_add(extent)
                    .min(map.height() as i32 - 1)
            {
                for x in blast.centre.x.saturating_sub(extent).max(0)
                    ..=blast
                        .centre
                        .x
                        .saturating_add(extent)
                        .min(map.width() as i32 - 1)
                {
                    let p = GridPos::new(x, y);
                    let Some(r) = rect(p) else {
                        continue;
                    };
                    if !map.is_walkable(p)
                        || !project_rl::world::has_line_of_sight(map, blast.centre, p, true)
                    {
                        continue;
                    }
                    let (left, top, right, bottom) = (
                        whole.x.max(r.x),
                        whole.y.max(r.y),
                        whole.right().min(r.right()),
                        whole.bottom().min(r.bottom()),
                    );
                    if right <= left || bottom <= top {
                        continue;
                    }
                    draw_texture_ex(
                        &art.burst,
                        left,
                        top,
                        Color::new(1., 1., 1., 0.83 * ((0.55 - age) / 0.10).clamp(0., 1.)),
                        DrawTextureParams {
                            source: Some(Rect::new(
                                frame as f32 * 256. + (left - whole.x) * 256. / size,
                                (top - whole.y) * 256. / size,
                                (right - left) * 256. / size,
                                (bottom - top) * 256. / size,
                            )),
                            dest_size: Some(vec2(right - left, bottom - top)),
                            ..Default::default()
                        },
                    );
                }
            }
        }
        gl_use_default_material();
    });
}
