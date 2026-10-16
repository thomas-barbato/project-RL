//! Already approved protection, healing, spectral and curse presentation.
use macroquad::prelude::*;
use project_rl::{game::GameState, world::GridPos};
use std::cell::RefCell;
struct Art {
    egide: Texture2D,
    thorns: Texture2D,
    regeneration: Texture2D,
    purge: Texture2D,
    wings: Texture2D,
    reanimation: Texture2D,
    icons: Vec<(&'static str, Texture2D)>,
    marker: Material,
    ghost: Material,
}
thread_local! {static ART:RefCell<Option<Art>>=const{RefCell::new(None)};}
impl Art {
    fn load() -> Self {
        let texture = |b: &[u8]| {
            let t = Texture2D::from_file_with_format(b, Some(ImageFormat::Png));
            t.set_filter(FilterMode::Linear);
            t
        };
        let shader = |fragment: &'static str| {
            load_material(ShaderSource::Glsl{vertex:r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;uniform mat4 Model;uniform mat4 Projection;varying highp vec2 uv;varying lowp vec4 tint;void main(){uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#,fragment},MaterialParams{pipeline_params:macroquad::miniquad::PipelineParams{color_blend:Some(macroquad::miniquad::BlendState::new(macroquad::miniquad::Equation::Add,macroquad::miniquad::BlendFactor::Value(macroquad::miniquad::BlendValue::SourceAlpha),macroquad::miniquad::BlendFactor::OneMinusValue(macroquad::miniquad::BlendValue::SourceAlpha))),..Default::default()},..Default::default()}).expect("approved support material")
        };
        let marker = shader(
            r#"#version 100
precision highp float;varying highp vec2 uv;varying lowp vec4 tint;uniform sampler2D Texture;float shape(vec2 p){vec4 c=texture2D(Texture,p);return c.a*smoothstep(0.035,0.075,max(c.r,max(c.g,c.b)));}void main(){vec4 c=texture2D(Texture,uv)*tint;float a=shape(uv)*tint.a;vec2 step=vec2(1.0/24.0);float rim=max(max(shape(uv+vec2(step.x,0.0)),shape(uv-vec2(step.x,0.0))),max(shape(uv+vec2(0.0,step.y)),shape(uv-vec2(0.0,step.y))))*0.82*tint.a;float opacity=a+rim*(1.0-a);vec3 rgb=(c.rgb*a+vec3(0.018,0.022,0.027)*rim*(1.0-a))/max(opacity,0.0001);gl_FragColor=vec4(rgb,opacity);}"#,
        );
        let ghost = shader(
            r#"#version 100
precision highp float;varying highp vec2 uv;varying lowp vec4 tint;uniform sampler2D Texture;void main(){float a=texture2D(Texture,uv).a;gl_FragColor=vec4(0.10,0.85,0.95,a*tint.a);}"#,
        );
        Self {
            egide: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-76/egide-source.png"
            )),
            thorns: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-76/epines-source.png"
            )),
            regeneration: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-77/regeneration-source.png"
            )),
            purge: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-78/purge-source.png"
            )),
            wings: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-83/SU06-C02-192.png"
            )),
            reanimation: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-38/reanimation-atlas.png"
            )),
            icons: vec![
                (
                    "agent:interference",
                    texture(include_bytes!("../assets/skills/agent/O02-compact.png")),
                ),
                (
                    "agent:armor_breach",
                    texture(include_bytes!("../assets/skills/agent/AF02-compact.png")),
                ),
                (
                    "agent:silence",
                    texture(include_bytes!("../assets/skills/agent/AF03-compact.png")),
                ),
                (
                    "agent:regeneration_break",
                    texture(include_bytes!("../assets/skills/agent/AF04-compact.png")),
                ),
                (
                    "agent:disorientation",
                    texture(include_bytes!("../assets/skills/agent/AF05-compact.png")),
                ),
                (
                    "agent:domination",
                    texture(include_bytes!("../assets/skills/agent/HY06-compact.png")),
                ),
            ],
            marker,
            ghost,
        }
    }
    fn picture(texture: &Texture2D, at: Vec2, size: Vec2, opacity: f32) {
        draw_texture_ex(
            texture,
            at.x - size.x / 2.,
            at.y - size.y / 2.,
            Color::new(1., 1., 1., opacity.clamp(0., 1.)),
            DrawTextureParams {
                dest_size: Some(size),
                ..Default::default()
            },
        );
    }
    fn ghost(&self, game: &GameState, at: Vec2, tile: f32, opacity: f32) {
        let mut visual =
            crate::game_art::player(None, crate::terminal_view::PlayerWeaponPose::Unarmed, None);
        if let Some(entry) = game
            .player_visible_body_item()
            .and_then(|i| game.player_inventory().get(i))
        {
            visual = crate::game_art::wear(
                visual,
                [("core:body_armor".into(), entry.item().as_str().to_owned())],
            );
            if let crate::game_art::Visual::Player { palettes, .. } = &mut visual {
                palettes[0] = entry.palette();
            }
        }
        gl_use_material(&self.ghost);
        crate::game_art::draw_world(
            Rect::new(at.x - tile / 2., at.y - tile / 2., tile, tile),
            visual,
            opacity,
        );
        gl_use_default_material();
    }
}
pub(super) fn has_status_icon(id: &str) -> bool {
    matches!(
        id,
        "agent:interference"
            | "agent:armor_breach"
            | "agent:silence"
            | "agent:regeneration_break"
            | "agent:disorientation"
            | "agent:domination"
    )
}

pub(super) fn draw(
    game: &GameState,
    flights: &[super::agent_magic::MagicFlight],
    now: f64,
    reduced: bool,
    background: bool,
    rect: impl Fn(GridPos) -> Option<Rect>,
) {
    let statuses = game.actors().iter().any(|(entity, a)| {
        game.agent_reanimation_marked(entity)
            || game.agent_reanimated_ally(entity)
            || a.statuses().any(|s| {
                matches!(
                    s.definition.as_str(),
                    "agent:regeneration"
                        | "agent:spectral_step"
                        | "agent:phase_traversal"
                        | "agent:interference"
                        | "agent:armor_breach"
                        | "agent:silence"
                        | "agent:regeneration_break"
                        | "agent:disorientation"
                        | "agent:domination"
                )
            })
    });
    if !statuses
        && !flights.iter().any(|f| {
            matches!(
                f.skill.as_str(),
                "B02" | "SU03" | "SU04" | "SU05" | "SU06" | "T02" | "ES05" | "OC07-rise"
            )
        })
    {
        return;
    }
    ART.with_borrow_mut(|cache| {
        let art = cache.get_or_insert_with(Art::load);
        let clock = if reduced { 0.2 } else { now as f32 };
        for (entity, actor) in game.actors().iter() {
            let Some(r) = rect(actor.position()) else {
                continue;
            };
            let p = r.center();
            let scale = r.w / 64.;
            if background && entity == game.player_id() && game.agent_regeneration_visual_active() {
                let pulse = (clock * 4.).sin() * 0.5 + 0.5;
                Art::picture(
                    &art.regeneration,
                    p + vec2(0., -2. + (clock * 2.8).sin() * 1.5) * scale,
                    vec2(49. + pulse * 4., 78.) * scale,
                    0.8 + pulse * 0.2,
                );
            }
            // The same six frames and placement as the accepted OC07 preview:
            // persistent mark at the feet, then the skull above the actual ally.
            if background && game.agent_reanimation_marked(entity) {
                let frame = (clock * 6.) as usize % 3;
                draw_texture_ex(
                    &art.reanimation,
                    p.x - 58. * scale,
                    p.y - 23. * scale,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(frame as f32 * 256., 0., 256., 256.)),
                        dest_size: Some(vec2(116., 84.) * scale),
                        ..Default::default()
                    },
                );
            }
            if !background && game.agent_reanimated_ally(entity) {
                draw_texture_ex(
                    &art.reanimation,
                    p.x + 11. * scale,
                    p.y - 37. * scale,
                    WHITE,
                    DrawTextureParams {
                        source: Some(Rect::new(5. * 256., 0., 256., 256.)),
                        dest_size: Some(vec2(24., 24.) * scale),
                        ..Default::default()
                    },
                );
            }
            if !background {
                let icons: Vec<_> = art
                    .icons
                    .iter()
                    .filter(|(id, _)| actor.statuses().any(|s| s.definition.as_str() == *id))
                    .collect();
                let total = icons.len() as f32;
                if !icons.is_empty() {
                    gl_use_material(&art.marker);
                    for (i, (_, t)) in icons.into_iter().enumerate() {
                        Art::picture(
                            t,
                            p + vec2((i as f32 - (total - 1.) / 2.) * 23., -45.) * scale,
                            vec2(24., 24.) * scale,
                            1.,
                        );
                    }
                    gl_use_default_material();
                }
                if entity == game.player_id()
                    && actor
                        .statuses()
                        .any(|s| s.definition.as_str() == "agent:spectral_step")
                {
                    for offset in [vec2(-1.5, 0.), vec2(1.5, 0.), vec2(0., -1.5)] {
                        art.ghost(
                            game,
                            p + offset * scale,
                            r.w,
                            0.16 + 0.025 * (clock * 3.2).sin(),
                        );
                    }
                }
            }
        }
        for flight in flights {
            if !matches!(
                flight.skill.as_str(),
                "B02" | "SU03" | "SU04" | "SU05" | "SU06" | "T02" | "ES05" | "OC07-rise"
            ) {
                continue;
            }
            let age = (now - flight.started) as f32;
            let Some(r) = rect(flight.target) else {
                continue;
            };
            let scale = r.w / 64.;
            let p = r.center();
            match flight.skill.as_str() {
                "OC07-rise" if !background && age < 0.36 => {
                    let frame = if age < 0.20 { 3 } else { 4 };
                    let opacity = ((0.36 - age) / 0.09).clamp(0., 1.);
                    draw_texture_ex(
                        &art.reanimation,
                        p.x - 73. * scale,
                        p.y - 88. * scale,
                        Color::new(1., 1., 1., opacity),
                        DrawTextureParams {
                            source: Some(Rect::new(frame as f32 * 256., 0., 256., 256.)),
                            dest_size: Some(vec2(146., 166.) * scale),
                            ..Default::default()
                        },
                    );
                }
                "B02" | "SU03" if background && age < 0.34 => {
                    let u = (age / 0.34).clamp(0., 1.);
                    draw_texture_ex(
                        if flight.skill == "B02" {
                            &art.egide
                        } else {
                            &art.thorns
                        },
                        p.x - (96. + 25. * u) * scale / 2.,
                        p.y - (96. + 25. * u) * scale / 2.,
                        Color::new(1., 1., 1., (1. - u).powf(0.6)),
                        DrawTextureParams {
                            dest_size: Some(vec2(96. + 25. * u, 96. + 25. * u) * scale),
                            rotation: u * 0.16,
                            pivot: Some(p),
                            ..Default::default()
                        },
                    );
                }
                "SU05" if background && age < 0.8 => {
                    let pulse = (clock * 4.).sin() * 0.5 + 0.5;
                    Art::picture(
                        &art.purge,
                        p + vec2(0., -2. + (clock * 2.8).sin() * 1.5) * scale,
                        vec2(49. + pulse * 4., 78.) * scale,
                        (1. - age / 0.8).powf(0.4),
                    );
                }
                "SU06" if background && age < 0.8 => {
                    let opacity = (age / 0.07).min(1.) * ((0.8 - age) / 0.23).clamp(0., 1.);
                    let pulse = (age / 0.16).min(1.) * ((0.8 - age) / 0.64).clamp(0., 1.);
                    let centre = p + vec2(0., -10.) * scale;
                    for ring in 0..32 {
                        let fraction = ring as f32 / 31.;
                        let radius = (68. - 43. * fraction) * scale;
                        draw_ellipse(
                            centre.x,
                            centre.y,
                            radius,
                            radius * 0.92,
                            0.,
                            Color::new(1., 1., 1., opacity * (0.016 + 0.024 * fraction)),
                        );
                    }
                    Art::picture(
                        &art.wings,
                        centre,
                        vec2(84. + pulse * 8., 84. + pulse * 8.) * scale,
                        opacity,
                    );
                }
                "T02" | "ES05" if !background && age < 0.28 => {
                    if let Some(from) = rect(flight.origin) {
                        for i in (1..=3).rev() {
                            let k = (age / 0.18 - i as f32 * 0.12).clamp(0., 1.);
                            art.ghost(
                                game,
                                from.center().lerp(p, k),
                                r.w,
                                0.27 * (1. - age / 0.28),
                            );
                        }
                    }
                }
                _ => {}
            }
            if !background && flight.skill == "SU05" && age < 0.8 {
                for i in 0..3 {
                    let phase = (clock * 0.65 + i as f32 / 3.).fract();
                    let y = p.y + (22. - phase * 45.) * scale;
                    let x = p.x + (phase * 6. + i as f32).sin() * 8. * scale;
                    let opacity =
                        (std::f32::consts::PI * phase).sin().max(0.) * 0.8 * (1. - age / 0.8);
                    draw_line(
                        x - 5. * scale,
                        y,
                        x + 5. * scale,
                        y,
                        2. * scale,
                        Color::new(0.76, 0.85, 1., opacity),
                    );
                }
                if age < 0.5 {
                    crate::ui_theme::draw_text_bold(
                        "Purge",
                        p.x - 22. * scale,
                        p.y - (48. + age * 12.) * scale,
                        16. * scale,
                        Color::new(0.85, 0.90, 1., 1. - age / 0.5),
                    );
                }
            }
        }
    });
}
