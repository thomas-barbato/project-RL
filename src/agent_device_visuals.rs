//! Native rendering of the already approved mine, grenade and residue artwork.
use crate::effect_geometry::LazyTexture;
use macroquad::miniquad::{
    BlendFactor, BlendState, BlendValue, Equation, PipelineParams, UniformDesc, UniformType,
};
use macroquad::prelude::*;
use project_rl::game::{AgentMineVariant, GameState};
use project_rl::world::{GridPos, Map};
use std::cell::RefCell;

thread_local! { static ART:RefCell<Option<Art>>=const{RefCell::new(None)}; }
struct Mask {
    cells: Vec<GridPos>,
    bounds: (i32, i32, u16, u16),
    texture: Texture2D,
}
struct Art {
    mine: LazyTexture,
    flame: LazyTexture,
    ice: LazyTexture,
    gas: LazyTexture,
    bursts: [LazyTexture; 3],
    fragment: LazyTexture,
    grenades: LazyTexture,
    clip: Material,
    lens: Material,
    masks: Vec<Mask>,
    born: std::collections::BTreeMap<u64, f64>,
}
fn load(bytes: &'static [u8]) -> LazyTexture {
    LazyTexture::plain(bytes, FilterMode::Linear)
}
const VERTEX: &str = r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;uniform mat4 Model;uniform mat4 Projection;
varying highp vec2 p;varying highp vec2 uv;varying lowp vec4 tint;
void main(){p=position.xy;uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#;
impl Art {
    fn load() -> Self {
        let blend = PipelineParams {
            color_blend: Some(BlendState::new(
                Equation::Add,
                BlendFactor::Value(BlendValue::SourceAlpha),
                BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
            )),
            ..Default::default()
        };
        let clip=load_material(ShaderSource::Glsl{vertex:VERTEX,fragment:r#"#version 100
precision highp float;varying highp vec2 p;varying highp vec2 uv;varying lowp vec4 tint;
uniform sampler2D Texture;uniform sampler2D zoneMask;uniform vec2 gridOrigin;uniform vec2 gridSize;uniform float tileSize;
void main(){vec2 q=(p-gridOrigin)/tileSize;if(q.x<0.0||q.y<0.0||q.x>=gridSize.x||q.y>=gridSize.y)discard;
if(texture2D(zoneMask,(floor(q)+vec2(0.5))/gridSize).r<0.5)discard;gl_FragColor=texture2D(Texture,uv)*tint;}"#},MaterialParams{uniforms:vec![UniformDesc::new("gridOrigin",UniformType::Float2),UniformDesc::new("gridSize",UniformType::Float2),UniformDesc::new("tileSize",UniformType::Float1)],textures:vec!["zoneMask".into()],pipeline_params:blend,..Default::default()}).expect("approved residue clip shader");
        let lens=load_material(ShaderSource::Glsl{vertex:VERTEX,fragment:r#"#version 100
precision highp float;varying highp vec2 uv;varying lowp vec4 tint;uniform sampler2D Texture;uniform vec3 lensTint;
void main(){vec4 c=texture2D(Texture,uv)*tint;if(uv.x>0.46&&uv.x<0.69&&uv.y>0.345&&uv.y<0.46&&c.r>c.g*1.45&&c.r>c.b*1.45)c.rgb=lensTint*c.r;gl_FragColor=c;}"#},MaterialParams{uniforms:vec![UniformDesc::new("lensTint",UniformType::Float3)],pipeline_params:blend,..Default::default()}).expect("approved elemental LED shader");
        Self {
            mine: load(include_bytes!(
                "../assets/animations/agent/skill-animation-review-60/mine-sprite.png"
            )),
            flame: load(include_bytes!(
                "../assets/animations/agent/skill-animation-review-06/flame-atlas.png"
            )),
            ice: load(include_bytes!(
                "../assets/animations/agent/skill-animation-review-68/snow-ice-source.png"
            )),
            gas: load(include_bytes!(
                "../assets/animations/agent/skill-animation-review-62/poison-burst-atlas.png"
            )),
            bursts: [
                load(include_bytes!(
                    "../assets/animations/agent/skill-animation-review-63/fire-burst-atlas.png"
                )),
                load(include_bytes!(
                    "../assets/animations/agent/skill-animation-review-62/poison-burst-atlas.png"
                )),
                load(include_bytes!(
                    "../assets/animations/agent/skill-animation-review-62/frost-burst-atlas.png"
                )),
            ],
            fragment: load(include_bytes!(
                "../assets/animations/agent/skill-animation-review-72/fragment-burst-source.png"
            )),
            grenades: load(include_bytes!(
                "../assets/animations/agent/skill-animation-review-75/grenades-simple-source.png"
            )),
            clip,
            lens,
            masks: vec![],
            born: Default::default(),
        }
    }
    fn mask(
        &mut self,
        cells: &[GridPos],
        rect: &impl Fn(GridPos) -> Option<Rect>,
    ) -> Option<(Rect, f32)> {
        let min_x = cells.iter().map(|p| p.x).min()?;
        let min_y = cells.iter().map(|p| p.y).min()?;
        let max_x = cells.iter().map(|p| p.x).max()?;
        let max_y = cells.iter().map(|p| p.y).max()?;
        let width = (max_x - min_x + 1).clamp(1, 65535) as u16;
        let height = (max_y - min_y + 1).clamp(1, 65535) as u16;
        let visible: Vec<_> = cells
            .iter()
            .copied()
            .filter(|p| rect(*p).is_some())
            .collect();
        let anchor = *visible.first()?;
        let tile = rect(anchor)?;
        let bounds = (min_x, min_y, width, height);
        let cache = self
            .masks
            .iter()
            .position(|m| m.cells == visible && m.bounds == bounds);
        let index = if let Some(index) = cache {
            index
        } else {
            if self.masks.len() >= 32 {
                self.masks.clear();
            }
            let mut image = Image::gen_image_color(width, height, BLACK);
            for p in &visible {
                image.set_pixel((p.x - min_x) as u32, (p.y - min_y) as u32, WHITE);
            }
            let texture = Texture2D::from_image(&image);
            texture.set_filter(FilterMode::Nearest);
            self.masks.push(Mask {
                cells: visible,
                bounds,
                texture,
            });
            self.masks.len() - 1
        };
        let bounds = Rect::new(
            tile.x + (min_x - anchor.x) as f32 * tile.w,
            tile.y + (min_y - anchor.y) as f32 * tile.h,
            width as f32 * tile.w,
            height as f32 * tile.h,
        );
        self.clip.set_uniform("gridOrigin", [bounds.x, bounds.y]);
        self.clip
            .set_uniform("gridSize", [width as f32, height as f32]);
        self.clip.set_uniform("tileSize", tile.w);
        self.clip
            .set_texture("zoneMask", self.masks[index].texture.clone());
        gl_use_material(&self.clip);
        Some((bounds, tile.w / 64.0))
    }
}
fn tint(mut color: Color, opacity: f32) -> Color {
    color.a *= opacity;
    color
}
fn hash(mut n: u32) -> u32 {
    n ^= n >> 16;
    n = n.wrapping_mul(0x7feb352d);
    n ^= n >> 15;
    n = n.wrapping_mul(0x846ca68b);
    n ^ (n >> 16)
}
fn seed(at: GridPos) -> u32 {
    hash((at.x as u32).wrapping_mul(73856093) ^ (at.y as u32).wrapping_mul(19349663))
}
fn unit(n: u32) -> f32 {
    (hash(n) & 0xffff) as f32 / 65535.0
}
fn gas_states(at: GridPos, time: f32) -> (usize, usize, f32) {
    let n = seed(at);
    let period = 0.72 + unit(n) * 0.83;
    let t = time / period + unit(n ^ 19) * 13.;
    let epoch = t.floor() as u32;
    let f = |e| 3 + (hash(n ^ hash(e)) % 3) as usize;
    let mix = (t.fract() * period / 0.25).min(1.);
    (
        f(epoch.saturating_sub(1)),
        f(epoch),
        mix * mix * (3. - 2. * mix),
    )
}
fn elemental(variant: AgentMineVariant) -> Color {
    match variant {
        AgentMineVariant::Fire => Color::new(0.95, 0.06, 0.04, 1.),
        AgentMineVariant::Poison => Color::new(0.38, 0.94, 0.12, 1.),
        AgentMineVariant::Cold => Color::new(0.28, 0.78, 1., 1.),
    }
}

pub(super) fn draw(
    game: &GameState,
    now: f64,
    reduced: bool,
    background: bool,
    rect: impl Fn(GridPos) -> Option<Rect>,
) {
    if !background
        || (game.player_agent_mines().is_empty() && game.player_agent_mine_residues().is_empty())
    {
        return;
    }
    ART.with(|cache| {
        let mut cache = cache.borrow_mut();
        let art = cache.get_or_insert_with(Art::load);
        let time = if reduced { 0.0 } else { now as f32 };
        art.born.retain(|serial, _| {
            game.player_agent_mine_residues()
                .iter()
                .any(|r| r.serial == *serial)
        });
        for zone in game.player_agent_mine_residues() {
            let born = *art.born.entry(zone.serial).or_insert(now);
            if now - born < 0.46 {
                continue;
            }
            let Some((bounds, scale)) = art.mask(&zone.cells, &rect) else {
                continue;
            };
            if zone.variant == AgentMineVariant::Cold {
                draw_texture_ex(
                    &art.ice,
                    bounds.x,
                    bounds.y,
                    tint(WHITE, 0.72 + (time * 0.65).sin() * 0.025),
                    DrawTextureParams {
                        dest_size: Some(bounds.size()),
                        ..Default::default()
                    },
                );
            } else {
                for (i, at) in zone.cells.iter().copied().enumerate() {
                    let Some(r) = rect(at) else {
                        continue;
                    };
                    let p = r.center();
                    if zone.variant == AgentMineVariant::Fire {
                        for (dx, dy) in [(-18., 6.), (15., 14.), (0., -13.)] {
                            let frame = ((time * 8.) as usize + i) % 8;
                            draw_texture_ex(
                                &art.flame,
                                p.x + (dx - 30.) * scale,
                                p.y + (dy - 38.) * scale,
                                tint(WHITE, 0.79),
                                DrawTextureParams {
                                    source: Some(Rect::new(frame as f32 * 128., 0., 128., 128.)),
                                    dest_size: Some(vec2(60., 48.) * scale),
                                    ..Default::default()
                                },
                            );
                        }
                    } else {
                        let n = seed(at);
                        let phase = unit(n ^ 31) * std::f32::consts::TAU;
                        let dx = (time * (0.7 + unit(n ^ 47)) + phase).sin() * 5.
                            + (time * 1.7 + phase * 2.3).sin() * 2.;
                        let dy = (time * (0.5 + unit(n ^ 61)) + phase * 1.9).sin() * 4.;
                        let w = 94. + (time * 0.8 + phase).sin() * 5.;
                        let h = 84. + (time * 1.1 + phase * 0.7).sin() * 3.;
                        let (from, to, mix) = gas_states(at, time);
                        for (frame, opacity) in [(from, 1. - mix), (to, mix)] {
                            draw_texture_ex(
                                &art.gas,
                                p.x + (-w * 0.5 + dx) * scale,
                                p.y + (-h * 0.5 + dy) * scale,
                                tint(WHITE, 0.62 * opacity),
                                DrawTextureParams {
                                    source: Some(Rect::new(frame as f32 * 256., 0., 256., 256.)),
                                    dest_size: Some(vec2(w, h) * scale),
                                    flip_x: n & 1 != 0,
                                    flip_y: n & 2 != 0,
                                    ..Default::default()
                                },
                            );
                        }
                    }
                }
            }
            gl_use_default_material();
        }
        for mine in game.player_agent_mines() {
            let Some(r) = rect(mine.at) else {
                continue;
            };
            let scale = r.w / 64.;
            let p = r.center();
            let blink = reduced || (now * 24.) as usize % 18 < 7;
            let color = elemental(mine.variant);
            let light = if blink { 1.0 } else { 0.13 };
            art.lens.set_uniform(
                "lensTint",
                [color.r * light, color.g * light, color.b * light],
            );
            gl_use_material(&art.lens);
            draw_texture_ex(
                &art.mine,
                p.x - 41.3 * scale,
                p.y - 16. * scale,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(72., 72.) * scale),
                    ..Default::default()
                },
            );
            gl_use_default_material();
            if blink {
                draw_ellipse(
                    p.x,
                    p.y + 13.25 * scale,
                    5.2 * scale,
                    2.3 * scale,
                    0.,
                    tint(
                        color,
                        if game.turn() >= mine.armed_at {
                            1.
                        } else {
                            0.76
                        },
                    ),
                );
            }
        }
    });
}

pub(super) fn burst(
    skill: &str,
    from: GridPos,
    to: GridPos,
    radius: u16,
    age: f32,
    map: &Map,
    background: bool,
    rect: impl Fn(GridPos) -> Option<Rect>,
    footprint: Option<&[GridPos]>,
) {
    if matches!(skill, "DR01" | "DP01") {
        if let Some(target) = rect(to) {
            repair(
                if skill == "DR01" { rect(from) } else { None },
                target,
                age,
                background,
                skill == "DR01",
            );
        }
        return;
    }
    if background
        || !matches!(
            skill,
            "SM00" | "SM02" | "SM03" | "SG00" | "SG01" | "SG02" | "HY07"
        )
    {
        return;
    }
    let Some(target) = rect(to) else {
        return;
    };
    let p = target.center();
    let scale = target.w / 64.;
    ART.with(|cache| {
        let mut cache = cache.borrow_mut();
        let art = cache.get_or_insert_with(Art::load);
        let grenade = skill.starts_with("SG");
        let flight = if grenade { 0.18 } else { 0. };
        if age < flight {
            if let Some(origin) = rect(from) {
                let t = (age / flight).clamp(0., 1.);
                let at = (origin.center() + vec2(12., -8.) * scale).lerp(p, t)
                    + vec2(0., -(t * std::f32::consts::PI).sin() * 30. * scale);
                let source = match skill {
                    "SG01" => Rect::new(232., 27., 385., 666.),
                    "SG02" => Rect::new(864., 27., 386., 665.),
                    _ => Rect::new(1545., 49., 446., 634.),
                };
                let size = vec2(18. * source.w / source.h, 18.) * scale;
                draw_texture_ex(
                    &art.grenades,
                    at.x - size.x * 0.5,
                    at.y - size.y * 0.5,
                    WHITE,
                    DrawTextureParams {
                        source: Some(source),
                        dest_size: Some(size),
                        rotation: t * 2.7,
                        ..Default::default()
                    },
                );
            }
            return;
        }
        let age = age - flight;
        let cells = project_rl::skills::spatial::affected_cells(
            map,
            to,
            project_rl::skills::spatial::Area::Square {
                radius: radius.saturating_add(1),
            },
            (1, 0),
        );
        let cells = if crate::effect_geometry::enabled() {
            footprint.unwrap_or(&cells)
        } else {
            &cells
        };
        if art.mask(cells, &rect).is_none() {
            return;
        }
        let scale = if crate::effect_geometry::enabled() {
            crate::effect_geometry::placement(cells, &rect, 0.)
                .map_or(scale, |p| p.size.x.min(p.size.y) / 168.)
        } else {
            scale
        };
        if skill == "SG01" && age < 0.32 {
            let u = age / 0.32;
            let opacity = (1. - u).powf(1.2);
            draw_circle(
                p.x,
                p.y,
                (16. + 68. * u) * scale,
                tint(WHITE, 0.55 * opacity),
            );
            for k in 0..12 {
                let theta = k as f32 * std::f32::consts::TAU / 12. + 0.12;
                let direction = vec2(theta.cos(), theta.sin());
                let start = p + direction * (14. + 23. * u) * scale;
                let end = p + direction * (40. + 66. * u) * scale;
                draw_line(
                    start.x,
                    start.y,
                    end.x,
                    end.y,
                    (5. * opacity + 1.) * scale,
                    tint(WHITE, opacity),
                );
            }
        } else if skill == "SG02" && age < 1.9 {
            let u = (age / 0.95).fract();
            draw_circle_lines(
                p.x,
                p.y,
                (28. + u * 155.) * scale,
                4. * scale,
                tint(Color::from_rgba(255, 194, 76, 255), 0.85 * (1. - u)),
            );
        } else if !matches!(skill, "SG01" | "SG02") {
            let index = match skill {
                "SM02" => 1,
                "SM03" => 2,
                _ => 0,
            };
            let duration = if index == 1 {
                0.46
            } else if index == 2 {
                0.40
            } else {
                0.44
            };
            if age < duration {
                let frame = ((age / duration * 6.) as usize).min(5);
                let place = crate::effect_geometry::enabled()
                    .then(|| crate::effect_geometry::placement(cells, &rect, 0.))
                    .flatten();
                let size = (radius as f32 * 2. + 1.) * 60. * scale;
                let (texture, source) = if skill == "SG00" {
                    (
                        &art.fragment,
                        Rect::new(
                            (frame % 3) as f32 * 512.,
                            (frame / 3) as f32 * 512.,
                            512.,
                            512.,
                        ),
                    )
                } else {
                    (
                        &art.bursts[index],
                        Rect::new(frame as f32 * 256., 0., 256., 256.),
                    )
                };
                draw_texture_ex(
                    texture,
                    place.map_or(p.x - size * 0.5, |p| p.center.x - p.size.x * 0.5),
                    place.map_or(p.y - size * 0.5 - 4. * scale, |p| {
                        p.center.y - p.size.y * 0.5
                    }),
                    tint(WHITE, 0.88),
                    DrawTextureParams {
                        source: Some(source),
                        dest_size: Some(place.map_or(vec2(size, size), |p| p.size)),
                        ..Default::default()
                    },
                );
            }
        }
        gl_use_default_material();
    });
}

fn repair(origin: Option<Rect>, target: Rect, age: f32, background: bool, active: bool) {
    let scale = target.w / 64.;
    let p = target.center();
    let blue = Color::new(0.12, 0.48, 0.96, 1.);
    let pale = Color::new(0.72, 0.88, 1., 1.);
    let ink = Color::new(0.015, 0.025, 0.035, 1.);
    if active
        && age < 0.10
        && !background
        && let Some(origin) = origin
    {
        let from = origin.center();
        let at = from.lerp(p, age / 0.10);
        let d = (p - from).normalize_or_zero();
        for (length, width, color) in [(22., 8., tint(ink, 0.8)), (20., 5., blue), (13., 2., pale)]
        {
            let start = at - d * length * scale;
            draw_line(start.x, start.y, at.x, at.y, width * scale, color);
        }
        draw_circle(at.x, at.y, 4. * scale, pale);
    }
    let age = age - if active { 0.10 } else { 0.0 };
    let duration = if active { 0.62 } else { 0.46 };
    if !(0.0..duration).contains(&age) {
        return;
    }
    let t = age / duration;
    let fade = (1. - t).sqrt();
    if background {
        let radius = if active { 30. } else { 24. };
        draw_ellipse(
            p.x,
            p.y + 27. * scale,
            (radius + t * 10.) * scale,
            (6. + t * 3.) * scale,
            0.,
            tint(blue, 0.20 * fade),
        );
        draw_ellipse_lines(
            p.x,
            p.y + 27. * scale,
            (radius + t * 10.) * scale,
            (6. + t * 3.) * scale,
            0.,
            2.4 * scale,
            tint(pale, 0.65 * fade),
        );
        for side in [-1., 1.] {
            let x = side * (20. + t * 10.);
            let points = [
                vec2(x, 23.),
                vec2(x + side * 6., 8. - t * 16.),
                vec2(x + side * 2., -13. - t * 18.),
            ];
            for line in points.windows(2) {
                let a = p + line[0] * scale;
                let b = p + line[1] * scale;
                for (width, color) in [
                    (7., tint(ink, fade * 0.7)),
                    (4., tint(blue, fade)),
                    (1.3, tint(pale, fade)),
                ] {
                    draw_line(a.x, a.y, b.x, b.y, width * scale, color);
                }
            }
        }
    } else {
        for i in 0..if active { 8 } else { 5 } {
            let angle = i as f32 * 2.399 + 0.2;
            let d = vec2(angle.cos(), angle.sin() * 0.8);
            let q = p + (vec2(0., -3.) + d * (8. + t * 23.) - vec2(0., t * 9.)) * scale;
            let end = q + d * (3. + t * 2.) * scale;
            draw_line(q.x, q.y, end.x, end.y, 4. * scale, tint(ink, fade));
            draw_line(q.x, q.y, end.x, end.y, 2.3 * scale, tint(pale, fade));
            draw_circle(q.x, q.y, 1.5 * scale, tint(blue, fade));
        }
        let y = p.y + (19. - t * 42.) * scale;
        draw_line(
            p.x - 11. * scale,
            y,
            p.x + 11. * scale,
            y,
            4. * scale,
            tint(ink, fade * 0.7),
        );
        draw_line(
            p.x - 10. * scale,
            y,
            p.x + 10. * scale,
            y,
            2. * scale,
            tint(pale, fade * 0.85),
        );
    }
}
