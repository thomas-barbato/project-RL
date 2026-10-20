//! Production adapters for approved painted spell atlases (no simulation state changes).
use crate::animation_policy::{
    material as load_material, restore_material as gl_use_default_material,
    use_material as gl_use_material,
};
use crate::effect_geometry::LazyTexture;
use macroquad::miniquad::{UniformDesc, UniformType};
use macroquad::prelude::*;
use project_rl::{
    game::GameState,
    world::{GridPos, has_line_of_sight},
};
use std::cell::RefCell;

#[derive(Clone, Debug)]
pub(super) struct MagicFlight {
    pub skill: String,
    pub origin: GridPos,
    pub target: GridPos,
    pub radius: u16,
    pub started: f64,
    pub cells: Option<Vec<GridPos>>,
}
struct Art {
    flame: LazyTexture,
    nova: LazyTexture,
    wall: LazyTexture,
    wave: LazyTexture,
    storm: LazyTexture,
    shatter: LazyTexture,
    gel_rear: LazyTexture,
    gel_front: LazyTexture,
    spirit: LazyTexture,
    ray: LazyTexture,
    prison: LazyTexture,
    shot: LazyTexture,
    mutagen: LazyTexture,
    form_bolt: LazyTexture,
    form_pulse: LazyTexture,
    shadows: LazyTexture,
    tentacle: LazyTexture,
    clip: Material,
    masks: Vec<((GridPos, (i32, i32), u64, usize, usize, u64), Texture2D)>,
}
thread_local! {static ART:RefCell<Option<Art>>=const{RefCell::new(None)};}
impl Art {
    fn load() -> Self {
        let texture = |bytes: &'static [u8]| LazyTexture::fitted(bytes, FilterMode::Linear);
        let clip=load_material(ShaderSource::Glsl{vertex:r#"#version 100
attribute vec3 position;attribute vec2 texcoord;attribute vec4 color0;
uniform mat4 Model;uniform mat4 Projection;varying highp vec2 point;varying highp vec2 uv;varying lowp vec4 tint;
void main(){point=position.xy;uv=texcoord;tint=color0/255.0;gl_Position=Projection*Model*vec4(position,1.0);}"#,
fragment:r#"#version 100
precision highp float;varying highp vec2 point;varying highp vec2 uv;varying lowp vec4 tint;
uniform sampler2D Texture;uniform sampler2D worldMask;uniform vec2 stageOrigin;uniform vec2 stageSize;uniform float grade;
void main(){vec2 local=(point-stageOrigin)/stageSize;if(local.x<0.0||local.y<0.0||local.x>=1.0||local.y>=1.0)discard;
float visibility=texture2D(worldMask,local).r;vec4 colour=texture2D(Texture,uv)*tint;
if(grade>2.0&&grade<4.0){float violet=smoothstep(0.007,0.055,colour.b-colour.g)*step(colour.r*0.82,colour.b);float visible=smoothstep(0.022,0.12,max(colour.r,colour.b));float peak=smoothstep(0.35,0.75,max(colour.r,colour.b));vec3 purple=mix(vec3(0.65,0.15,1.0),vec3(0.80,0.30,1.0),peak);colour.rgb=mix(colour.rgb,purple,violet*visible*0.96);}
if(grade>4.5)colour.rgb=min(vec3(0.97),colour.rgb*1.5);
gl_FragColor=vec4(colour.rgb,colour.a*visibility);}"#},MaterialParams{uniforms:vec![UniformDesc::new("stageOrigin",UniformType::Float2),UniformDesc::new("stageSize",UniformType::Float2),UniformDesc::new("grade",UniformType::Float1)],textures:vec!["worldMask".into()],pipeline_params:macroquad::miniquad::PipelineParams{color_blend:Some(macroquad::miniquad::BlendState::new(macroquad::miniquad::Equation::Add,macroquad::miniquad::BlendFactor::Value(macroquad::miniquad::BlendValue::SourceAlpha),macroquad::miniquad::BlendFactor::OneMinusValue(macroquad::miniquad::BlendValue::SourceAlpha))),..Default::default()},..Default::default()}).expect("approved spell clipping material");
        Self {
            flame: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-06/flame-atlas.png"
            )),
            nova: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-23/nova-atlas.png"
            )),
            wall: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-24/wall-atlas.png"
            )),
            wave: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-25/wave-atlas.png"
            )),
            storm: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-25/storm-atlas.png"
            )),
            shatter: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-26/burst-atlas.png"
            )),
            gel_rear: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-27/gel-atlas.png"
            )),
            gel_front: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-28/gel-atlas.png"
            )),
            spirit: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-33/spirit-atlas.png"
            )),
            ray: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-33/ray-atlas.png"
            )),
            prison: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-38/prison-atlas.png"
            )),
            shot: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-48/shot-atlas.png"
            )),
            mutagen: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-48/impact-atlas.png"
            )),
            form_bolt: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-42/bolt-atlas.png"
            )),
            form_pulse: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-42/pulse-atlas.png"
            )),
            shadows: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-44/abysse_fx-atlas.png"
            )),
            tentacle: texture(include_bytes!(
                "../assets/animations/agent/skill-animation-review-102/AF06-TENTACLE02-source.png"
            )),
            clip,
            masks: Vec::new(),
        }
    }
    fn bind(
        &mut self,
        game: &GameState,
        source: GridPos,
        direction: (i32, i32),
        anchor: GridPos,
        r: Rect,
        grade: f32,
        cells: Option<&[GridPos]>,
    ) {
        let map = game.map();
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        cells.hash(&mut hasher);
        let key = (
            source,
            direction,
            game.turn(),
            map.width(),
            map.height(),
            hasher.finish(),
        );
        if self
            .masks
            .first()
            .is_some_and(|(k, _)| k.2 != game.turn() || k.3 != map.width() || k.4 != map.height())
        {
            self.masks.clear();
        }
        let index = if let Some(i) = self.masks.iter().position(|(k, _)| *k == key) {
            i
        } else {
            let mut mask = Image::gen_image_color(map.width() as u16, map.height() as u16, BLACK);
            let exit = corridor_exit(game, source, direction);
            for at in game.player_visibility().visible_positions() {
                let from_exit = exit.is_some_and(|exit| {
                    let f = (at.x - exit.x) * direction.0 + (at.y - exit.y) * direction.1;
                    f >= 0 && has_line_of_sight(map, exit, at, true)
                });
                if map.is_walkable(at)
                    && cells.map_or_else(
                        || has_line_of_sight(map, source, at, true) || from_exit,
                        |cells| cells.contains(&at),
                    )
                {
                    mask.set_pixel(at.x as u32, at.y as u32, WHITE);
                }
            }
            if self.masks.len() == 16 {
                self.masks.remove(0);
            }
            let texture = Texture2D::from_image(&mask);
            texture.set_filter(FilterMode::Nearest);
            self.masks.push((key, texture));
            self.masks.len() - 1
        };
        self.clip
            .set_texture("worldMask", self.masks[index].1.clone());
        self.clip.set_uniform(
            "stageOrigin",
            (r.x - anchor.x as f32 * r.w, r.y - anchor.y as f32 * r.h),
        );
        self.clip.set_uniform(
            "stageSize",
            (map.width() as f32 * r.w, map.height() as f32 * r.h),
        );
        self.clip.set_uniform("grade", grade);
        gl_use_material(&self.clip);
    }
    fn sheet(texture: &Texture2D, frame: usize, at: Vec2, size: Vec2, angle: f32, opacity: f32) {
        draw_texture_ex(
            texture,
            at.x - size.x / 2.,
            at.y - size.y / 2.,
            Color::new(1., 1., 1., opacity.clamp(0., 1.)),
            DrawTextureParams {
                source: Some(Rect::new(frame as f32 * 256., 0., 256., 256.)),
                dest_size: Some(size),
                rotation: angle,
                pivot: Some(at),
                ..Default::default()
            },
        );
    }
    fn plume(&self, base: Vec2, clock: f32, phase: u32, scale: f32, opacity: f32) {
        let frame = ((clock.max(0.) * 8.) as u32 + phase) % 8;
        draw_texture_ex(
            &self.flame,
            base.x - 64. * scale,
            base.y - 120. * scale,
            Color::new(1., 1., 1., opacity),
            DrawTextureParams {
                source: Some(Rect::new(frame as f32 * 128., 0., 128., 128.)),
                dest_size: Some(vec2(128. * scale, 128. * scale)),
                ..Default::default()
            },
        );
    }
}
/// ANIM-OVER03: recover painted breadth only once an emitting corridor opens.
fn corridor_exit(game: &GameState, origin: GridPos, direction: (i32, i32)) -> Option<GridPos> {
    if direction == (0, 0) {
        return None;
    }
    let map = game.map();
    let side = (-direction.1, direction.0);
    let narrow = |at: GridPos| {
        map.blocks_vision(GridPos::new(at.x + side.0, at.y + side.1))
            && map.blocks_vision(GridPos::new(at.x - side.0, at.y - side.1))
    };
    if !narrow(origin) {
        return None;
    }
    let mut at = origin;
    for _ in 0..map.width().max(map.height()) {
        at = GridPos::new(at.x + direction.0, at.y + direction.1);
        if !map.contains(at) || !map.is_walkable(at) {
            return None;
        }
        if !narrow(at) {
            return Some(at);
        }
    }
    None
}
pub(super) fn draw_gel(rect: Rect, now: f64, seed: u64, reduced: bool, background: bool) {
    ART.with_borrow_mut(|cache| {
        let art = cache.get_or_insert_with(Art::load);
        let p = if reduced {
            seed as f32
        } else {
            crate::animation_policy::clock(now) as f32 * 3. + seed as f32
        };
        let f = p.floor() as usize % 6;
        let blend = p.fract();
        let size = 72. * rect.w / 64.;
        let at = rect.center();
        let (texture, y, h, opacity) = if background {
            (&art.gel_rear, 192., 64., 0.26)
        } else {
            (&art.gel_front, 64., 192., 0.70)
        };
        for (frame, weight) in [(f, 1. - blend), ((f + 1) % 6, blend)] {
            draw_texture_ex(
                texture,
                at.x - size / 2.,
                at.y - size / 2. + y * size / 256.,
                Color::new(1., 1., 1., weight * opacity),
                DrawTextureParams {
                    source: Some(Rect::new(frame as f32 * 256., y, 256., h)),
                    dest_size: Some(vec2(size, size * h / 256.)),
                    ..Default::default()
                },
            );
        }
    });
}
fn fire_corridor(
    art: &Art,
    cells: &[GridPos],
    source: GridPos,
    direction: (i32, i32),
    clock: f32,
    opacity: f32,
    rect: &impl Fn(GridPos) -> Option<Rect>,
) {
    let angle = (direction.1 as f32).atan2(direction.0 as f32) + std::f32::consts::FRAC_PI_2;
    for side in -1..=1 {
        let lane = cells
            .iter()
            .copied()
            .filter(|at| -(at.x - source.x) * direction.1 + (at.y - source.y) * direction.0 == side)
            .collect::<Vec<_>>();
        let frame = ((clock * 16.) as usize + (side + 1) as usize * 3) % 8;
        crate::effect_geometry::fitted_sheet(&art.flame, frame, &lane, rect, angle, opacity);
    }
}
pub(super) fn draw(
    game: &GameState,
    flights: &[MagicFlight],
    now: f64,
    reduced: bool,
    background: bool,
    rect: impl Fn(GridPos) -> Option<Rect>,
) {
    let effects = game.agent_magic_visual_effects();
    let barriers = game.agent_magic_visual_barriers();
    if effects.is_empty() && barriers.is_empty() && flights.is_empty() {
        return;
    }
    ART.with_borrow_mut(|cache| {
        let art = cache.get_or_insert_with(Art::load);
        let clock = if reduced {
            0.2
        } else {
            crate::animation_policy::clock(now) as f32
        };
        for e in &effects {
            let anchor = e
                .cells
                .iter()
                .copied()
                .chain([e.target, e.origin])
                .find_map(|p| rect(p).map(|r| (p, r)));
            let Some((anchor, r)) = anchor else { continue };
            let scale = r.w / 64.;
            if e.skill == "FR05" {
                if background {
                    let radius = project_rl::skills::agent::definition("FR05")
                        .unwrap()
                        .value("radius_cells", e.rank);
                    crate::animation_policy::with_gain(1.0, || {
                        crate::natural_effect::sheet(
                            &art.storm,
                            (clock * 6.) as usize % 6,
                            e.target,
                            radius,
                            if e.stage == 0 { 0.42 } else { 0.94 },
                            true,
                            game.map(),
                            &rect,
                        )
                    });
                }
                continue;
            }
            art.bind(
                game,
                if e.skill == "FE02" {
                    e.origin
                } else {
                    e.target
                },
                if e.skill == "FE02" {
                    e.direction
                } else {
                    (0, 0)
                },
                anchor,
                r,
                if e.skill == "OC05" { 3. } else { 0. },
                None,
            );
            if e.skill == "FE02" {
                if background {
                    fire_corridor(art, &e.cells, e.origin, e.direction, clock, 0.94, &rect);
                }
                gl_use_default_material();
                continue;
            }
            if crate::effect_geometry::enabled() && background {
                let fitted = match e.skill.as_str() {
                    "FR04" => Some((
                        &art.wave,
                        (clock * 6.) as usize % 6,
                        (e.direction.1 as f32).atan2(e.direction.0 as f32)
                            + std::f32::consts::FRAC_PI_2,
                        0.92,
                    )),
                    _ => None,
                };
                if let Some((texture, frame, angle, opacity)) = fitted {
                    crate::effect_geometry::fitted_sheet(
                        texture, frame, &e.cells, &rect, angle, opacity,
                    );
                    gl_use_default_material();
                    continue;
                }
            }
            match e.skill.as_str() {
                "FE03" if background => {
                    for (i, at) in e.cells.iter().enumerate() {
                        if let Some(cell) = rect(*at) {
                            art.plume(
                                cell.center() + vec2(0., 24. * scale),
                                clock,
                                i as u32 * 3,
                                scale,
                                1.,
                            );
                        }
                    }
                    for (i, pair) in e.cells.windows(2).enumerate() {
                        if pair[0].x.abs_diff(pair[1].x) + pair[0].y.abs_diff(pair[1].y) == 1 {
                            if let (Some(a), Some(b)) = (rect(pair[0]), rect(pair[1])) {
                                art.plume(
                                    (a.center() + b.center()) * 0.5 + vec2(0., 24. * scale),
                                    clock,
                                    i as u32 * 3 + 1,
                                    scale,
                                    1.,
                                );
                            }
                        }
                    }
                }
                "FE04" if background => {
                    for (i, at) in e.cells.iter().enumerate() {
                        if let Some(cell) = rect(*at) {
                            cracks(
                                cell.center() + vec2(0., 19. * scale),
                                clock,
                                i as u32,
                                scale,
                            );
                        }
                    }
                }
                "FR04" if background => {
                    if let Some(cell) = rect(e.target) {
                        let width = project_rl::skills::agent::definition("FR04")
                            .unwrap()
                            .value("wave_width_cells", e.rank);
                        let size = f32::from(width) * r.w;
                        Art::sheet(
                            &art.wave,
                            (clock * 6.) as usize % 6,
                            cell.center(),
                            vec2(size, size),
                            (e.direction.1 as f32).atan2(e.direction.0 as f32)
                                + std::f32::consts::FRAC_PI_2,
                            0.92,
                        );
                    }
                }
                "OC05" if !background => {
                    if let (Some(a), Some(b)) = (rect(e.origin), rect(e.target)) {
                        draw_ray(
                            art,
                            a.center() - vec2(0., 8. * scale),
                            b.center() - vec2(0., 8. * scale),
                            clock,
                            scale,
                            0.9,
                        );
                    }
                }
                _ => {}
            }
            gl_use_default_material();
        }
        for (entity, skill, centre, _) in barriers {
            let Some(actor) = game.actors().get(entity) else {
                continue;
            };
            let Some(r) = rect(actor.position()) else {
                continue;
            };
            let scale = r.w / 64.;
            let at = r.center() + vec2(0., 26. * scale);
            if skill == "FR03" && !background {
                let variant = (actor.position().x + actor.position().y).rem_euclid(3) as usize;
                let damaged = actor.integrity() * 2 < actor.maximum_integrity();
                let size = 76. * scale;
                Art::sheet(
                    &art.wall,
                    variant + if damaged { 3 } else { 0 },
                    at - vec2(0., 0.4 * size),
                    vec2(size, size),
                    0.,
                    0.94,
                );
            } else if skill == "OC04" && background {
                art.bind(game, centre, (0, 0), actor.position(), r, 5., None);
                let delta = vec2(
                    (actor.position().x - centre.x) as f32,
                    (actor.position().y - centre.y) as f32,
                );
                let angle = delta.y.atan2(delta.x) + std::f32::consts::FRAC_PI_2;
                for offset in [-1.2, 0., 1.2] {
                    Art::sheet(
                        &art.prison,
                        (clock * 6.) as usize % 3,
                        r.center() + vec2(angle.sin(), -angle.cos()) * offset * scale,
                        vec2(76., 48.) * scale,
                        angle,
                        1.,
                    );
                }
                gl_use_default_material();
            }
        }
        for flight in flights {
            let age = crate::animation_policy::age(&flight.skill, now, flight.started) as f32;
            if !(0.0..0.85).contains(&age) {
                continue;
            }
            let Some(r) = rect(flight.target) else {
                continue;
            };
            let scale = r.w / 64.;
            let target = r.center();
            let origin = rect(flight.origin).map_or(target, |r| r.center());
            let age = if reduced { 0.20 } else { age };
            art.bind(
                game,
                if matches!(
                    flight.skill.as_str(),
                    "HY05" | "OC05" | "M02" | "FR01" | "AF06"
                ) {
                    flight.origin
                } else {
                    flight.target
                },
                (0, 0),
                flight.target,
                r,
                if matches!(
                    flight.skill.as_str(),
                    "HY05" | "OC05" | "TR64_bolt" | "TR64_pulse" | "TR64_drain"
                ) {
                    3.
                } else {
                    0.
                },
                None,
            );
            art.clip.set_uniform(
                "readabilityGain",
                crate::animation_policy::gain(&flight.skill),
            );
            if crate::effect_geometry::enabled() {
                let fitted = match flight.skill.as_str() {
                    "FR02" if background && age < 0.45 => Some((
                        &art.nova,
                        ((age / 0.45 * 6.) as usize).min(5),
                        0.88 * ((0.45 - age) / 0.12).clamp(0., 1.),
                    )),
                    "FR06" if background && age < 0.42 => Some((
                        &art.shatter,
                        ((age / 0.42 * 6.) as usize).min(5),
                        0.88 * ((0.42 - age) / 0.12).clamp(0., 1.),
                    )),
                    "TR64_pulse" if background && age < 0.41 => Some((
                        &art.form_pulse,
                        (((age - 0.01).max(0.) / 0.40 * 6.) as usize).min(5),
                        1.,
                    )),
                    "M02" if !background && (0.18..0.46).contains(&age) => Some((
                        &art.mutagen,
                        (((age - 0.18) / 0.28 * 6.) as usize).min(5),
                        0.94,
                    )),
                    "TR64_bolt" if !background && (0.16..0.36).contains(&age) => Some((
                        &art.form_bolt,
                        3 + (((age - 0.16) / 0.20 * 3.) as usize).min(2),
                        1.,
                    )),
                    _ => None,
                };
                if let Some((texture, frame, opacity)) = fitted {
                    crate::animation_policy::with_gain(
                        crate::animation_policy::gain(&flight.skill),
                        || {
                            crate::natural_effect::sheet(
                                texture,
                                frame,
                                flight.target,
                                flight.radius,
                                opacity,
                                false,
                                game.map(),
                                &rect,
                            )
                        },
                    );
                    gl_use_default_material();
                    continue;
                }
            }
            match flight.skill.as_str() {
                "AF06" if !background => {
                    draw_contagion(&art.tentacle, game, flight, origin, target, r, age, scale);
                }
                "FR01" if !background => {
                    super::agent_ember::elemental_projectile(
                        2,
                        origin,
                        target,
                        r.w,
                        age * 0.55 / 0.18,
                        0,
                    );
                }
                "FR02" | "FR06" if background => {
                    let life = if flight.skill == "FR02" { 0.45 } else { 0.42 };
                    if age < life {
                        let frame = (age / life * 6.) as usize;
                        let size = if flight.skill == "FR02" {
                            (f32::from(flight.radius) * 2. + 1.) * r.w * 0.95
                        } else {
                            (f32::from(flight.radius) * 2. + 1.) * r.w * 0.84
                        };
                        Art::sheet(
                            if flight.skill == "FR02" {
                                &art.nova
                            } else {
                                &art.shatter
                            },
                            frame.min(5),
                            target,
                            vec2(size, size),
                            0.,
                            0.88 * ((life - age) / 0.12).clamp(0., 1.),
                        );
                    }
                }
                "FE04" if background => {
                    let s = project_rl::skills::spatial::Area::Square {
                        radius: flight.radius,
                    };
                    for (i, at) in project_rl::skills::spatial::affected_cells(
                        game.map(),
                        flight.target,
                        s,
                        (1, 0),
                    )
                    .iter()
                    .enumerate()
                    {
                        if let Some(r) = rect(*at) {
                            art.plume(
                                r.center() + vec2(0., 24. * scale),
                                age,
                                i as u32 * 2,
                                scale * if *at == flight.target { 1.45 } else { 1.10 },
                                (1. - age / 0.8).clamp(0., 1.).sqrt(),
                            );
                        }
                    }
                }
                "FE06" if background => {
                    let closing = (age / 0.20).clamp(0., 1.);
                    for (phase, sign) in [(0, -1.), (3, 1.)] {
                        art.plume(
                            target + vec2(sign * (13. - 8. * closing), 21.) * scale,
                            age,
                            phase,
                            scale * if phase == 0 { 0.52 } else { 0.42 },
                            0.65 * (1. - age / 0.6).clamp(0., 1.),
                        );
                    }
                }
                "HY05" | "OC05" if !background => {
                    draw_drain(
                        art,
                        flight.skill == "HY05",
                        origin - vec2(0., 8. * scale),
                        target - vec2(0., 8. * scale),
                        age,
                        scale,
                    );
                }
                "TR64_bolt" if !background => {
                    let delta = target - origin;
                    let axis = delta.normalize_or_zero();
                    if age < 0.16 {
                        Art::sheet(
                            &art.form_bolt,
                            ((age / 0.16 * 3.) as usize).min(2),
                            origin.lerp(target, age / 0.16) - axis * 28. * scale,
                            vec2(64., 48.) * scale,
                            axis.y.atan2(axis.x),
                            1.,
                        );
                    } else if age < 0.36 {
                        Art::sheet(
                            &art.form_bolt,
                            3 + (((age - 0.16) / 0.20 * 3.) as usize).min(2),
                            target,
                            vec2(96., 80.) * scale,
                            0.,
                            1.,
                        );
                    }
                }
                "TR64_pulse" if background && age < 0.41 => {
                    let size = (f32::from(flight.radius) * 2. + 1.) * r.w;
                    Art::sheet(
                        &art.form_pulse,
                        (((age - 0.01).max(0.) / 0.40 * 6.) as usize).min(5),
                        target,
                        vec2(size * 1.0625, size * 1.0125),
                        0.,
                        1.,
                    );
                }
                "TR64_drain" if !background && age < 0.29 => {
                    let delta = origin - target;
                    let centre = (origin + target) * 0.5;
                    let angle = delta.y.atan2(delta.x);
                    let frame = ((age * 11.) as usize) % 3;
                    let fade = ((0.29 - age) / 0.09).clamp(0., 1.);
                    Art::sheet(
                        &art.ray,
                        frame,
                        centre,
                        vec2(delta.length() * 256. / 224., 150. * scale),
                        angle,
                        fade * 0.35,
                    );
                    Art::sheet(
                        &art.ray,
                        frame,
                        centre,
                        vec2(delta.length() * 256. / 224., 112. * scale),
                        angle,
                        fade,
                    );
                }
                "TR65_shadow_step" if background && age < 0.32 => {
                    let frame = ((age / 0.32 * 6.) as usize).min(5);
                    for p in [origin, target] {
                        Art::sheet(
                            &art.shadows,
                            frame,
                            p,
                            vec2(104., 104.) * scale,
                            0.,
                            0.85 * (1. - age / 0.32),
                        );
                    }
                }
                "M02" if !background => {
                    let delta = target - origin;
                    let axis = delta.normalize_or_zero();
                    if age < 0.18 {
                        Art::sheet(
                            &art.shot,
                            ((age / 0.18 * 6.) as usize).min(5),
                            origin.lerp(target, age / 0.18) - axis * 18. * scale,
                            vec2(96., 96.) * scale,
                            axis.y.atan2(axis.x),
                            1.,
                        );
                    } else if age < 0.46 {
                        Art::sheet(
                            &art.mutagen,
                            (((age - 0.18) / 0.28 * 6.) as usize).min(5),
                            target,
                            vec2(112., 112.) * scale,
                            0.,
                            0.94,
                        );
                    }
                }
                _ => {}
            }
            gl_use_default_material();
        }
    });
}
fn draw_ray(art: &Art, origin: Vec2, victim: Vec2, clock: f32, scale: f32, opacity: f32) {
    let d = origin - victim;
    let centre = (origin + victim) * 0.5;
    let angle = d.y.atan2(d.x);
    let length = d.length() * 256. / 224.;
    let frame = (clock * 9.) as usize % 3;
    for (height, a, next) in [
        (240., opacity * 0.36, 0),
        (184., 1., 0),
        (120., opacity * 0.72, 1),
    ] {
        Art::sheet(
            &art.ray,
            (frame + next) % 3,
            centre,
            vec2(length, height * scale),
            angle,
            a,
        );
    }
}
fn draw_drain(art: &Art, spirit: bool, origin: Vec2, victim: Vec2, age: f32, scale: f32) {
    let t = age - if spirit { 0.16 } else { 0. };
    if !(0.0..0.30).contains(&t) {
        return;
    }
    let u = t / 0.30;
    let delta = origin - victim;
    let angle = delta.y.atan2(delta.x);
    let count = if spirit { 6 } else { 4 };
    let fade = (u / 0.08).clamp(0., 1.) * ((1. - u) / 0.18).clamp(0., 1.);
    for i in (0..count).rev() {
        let lag = i as f32 * 0.105;
        if u < lag {
            continue;
        }
        let progress = u - lag;
        let size = if spirit {
            vec2(120. - 22. * u, 132. - 20. * u)
        } else {
            vec2(102. - 18. * u, 116. - 18. * u)
        };
        let side = vec2(-delta.y, delta.x).normalize_or_zero()
            * (u * std::f32::consts::PI + i as f32 * 1.7).sin()
            * 5.
            * scale;
        Art::sheet(
            if spirit { &art.spirit } else { &art.ray },
            3 + ((u * 3. + i as f32 * 0.5) as usize % 3),
            victim + delta * progress + side,
            size * scale * (1. - i as f32 * 0.07),
            angle,
            fade * (1. - i as f32 * 0.07),
        );
    }
}
fn cracks(at: Vec2, time: f32, seed: u32, scale: f32) {
    for n in 0..5 {
        let a = super::agent_ember::noise(seed + n) * std::f32::consts::TAU;
        let d = vec2(a.cos(), a.sin());
        let normal = vec2(-d.y, d.x);
        let mut from = at;
        for step in 0..3 {
            let to = from
                + d * 9. * scale
                + normal * (super::agent_ember::noise(seed + n * 5 + step) - 0.5) * 9. * scale;
            draw_line(
                from.x,
                from.y,
                to.x,
                to.y,
                2.2 * scale,
                Color::new(0.92, 0.24, 0.015, 0.5 + 0.3 * (time * 4.).sin().abs()),
            );
            from = to;
        }
    }
}

// AF06-A02: the approved painted organic strip follows the same curved spine,
// contact timing and retraction as its review, clipped by the shared terrain mask.
fn draw_contagion(
    texture: &Texture2D,
    game: &GameState,
    flight: &MagicFlight,
    source: Vec2,
    target: Vec2,
    r: Rect,
    age: f32,
    scale: f32,
) {
    const CONTACT: f32 = 0.22;
    const LIFE: f32 = 0.72;
    if !(0.0..LIFE).contains(&age) {
        return;
    }
    let reach = if age < CONTACT {
        1.0 - (1.0 - age / CONTACT).powi(3)
    } else if age < 0.43 {
        1.0
    } else {
        let t = ((age - 0.43) / (LIFE - 0.43)).clamp(0.0, 1.0);
        1.0 - t * t * (3.0 - 2.0 * t)
    };
    let from = source + vec2(0.0, 3.0 * scale);
    let to = target - vec2(0.0, 5.0 * scale);
    let seed = (flight.target.x * 17 + flight.target.y * 31).unsigned_abs() as f32 * 2.31;
    let axis = (to - from).normalize_or_zero();
    let side = vec2(-axis.y, axis.x);
    let spine = |u: f32, bend: f32| {
        from.lerp(to, u)
            + side
                * (u * std::f32::consts::PI).sin()
                * (bend + (u * 8.0 + age * 14.0 + seed).sin() * 3.0 * scale)
    };
    let requested = (if (flight.target.x + flight.target.y) % 2 == 0 {
        1.0
    } else {
        -1.0
    }) * ((to - from).length() * 0.17).min(23.0 * scale);
    let bend = if (0..=48).all(|i| {
        let p = spine(i as f32 / 48.0, requested);
        let at = GridPos::new(
            flight.target.x + ((p.x - r.x) / r.w).floor() as i32,
            flight.target.y + ((p.y - r.y) / r.h).floor() as i32,
        );
        game.map().is_walkable(at)
    }) {
        requested
    } else {
        0.0
    };
    let opacity = if age < 0.56 {
        1.0
    } else {
        (LIFE - age) / (LIFE - 0.56)
    };
    let mut mesh = Mesh {
        vertices: Vec::with_capacity(98),
        indices: Vec::with_capacity(288),
        texture: Some(texture.clone()),
    };
    for j in 0..=48 {
        let t = j as f32 / 48.0;
        let u = t * reach;
        let at = spine(u, bend);
        let tangent = (spine((u + 0.002).min(1.0), bend) - spine((u - 0.002).max(0.0), bend))
            .normalize_or_zero();
        let side = vec2(-tangent.y, tangent.x);
        let half = 10.5 * scale * (0.90 + 0.10 * (t * 16.0 - age * 17.0 + seed).sin());
        for (sign, v) in [(-1.0, 222.0 / 748.0), (1.0, 514.0 / 748.0)] {
            let p = at + side * half * sign;
            mesh.vertices.push(Vertex::new(
                p.x,
                p.y,
                0.0,
                (20.0 + 2063.0 * t) / 2103.0,
                v,
                Color::new(1.0, 1.0, 1.0, opacity),
            ));
        }
        if j > 0 {
            let k = (j * 2) as u16;
            mesh.indices
                .extend_from_slice(&[k - 2, k - 1, k, k, k - 1, k + 1]);
        }
    }
    draw_mesh(&mesh);
}
