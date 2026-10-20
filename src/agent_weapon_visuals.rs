//! Reviewed weapon drawings shared with the animation laboratory.
//! Presentation time never changes skill counters or simulation state.
use crate::effect_geometry::LazyTexture;
use macroquad::prelude::*;
use project_rl::{
    game::GameState,
    world::{GridPos, Map},
};
use std::{cell::RefCell, collections::BTreeMap};

const INK: Color = Color::new(0.025, 0.03, 0.035, 1.0);
struct Art {
    icons: BTreeMap<&'static str, LazyTexture>,
    jaw: LazyTexture,
    rocket: LazyTexture,
    blast: LazyTexture,
    claws: LazyTexture,
    impact: LazyTexture,
    plates: LazyTexture,
    reach: LazyTexture,
    line: LazyTexture,
}
thread_local! {static ART:RefCell<Option<Art>>=const {RefCell::new(None)};}
impl Art {
    fn load() -> Self {
        fn image(bytes: &'static [u8]) -> LazyTexture {
            LazyTexture::fitted(bytes, FilterMode::Linear)
        }
        Self {
            icons: [
                (
                    "BM01",
                    image(include_bytes!("../assets/skills/agent/BM01-compact.png")),
                ),
                (
                    "BM02",
                    image(include_bytes!("../assets/skills/agent/BM02-compact.png")),
                ),
                (
                    "BM03",
                    image(include_bytes!("../assets/skills/agent/BM03-compact.png")),
                ),
                (
                    "BM05",
                    image(include_bytes!("../assets/skills/agent/BM05-compact.png")),
                ),
                (
                    "BD04",
                    image(include_bytes!("../assets/skills/agent/BD04-compact.png")),
                ),
                (
                    "T01",
                    image(include_bytes!("../assets/skills/agent/T01-compact.png")),
                ),
                (
                    "SF00",
                    image(include_bytes!("../assets/skills/agent/SF00-compact.png")),
                ),
                (
                    "HL02",
                    image(include_bytes!("../assets/skills/agent/HL02-compact.png")),
                ),
                (
                    "HL03",
                    image(include_bytes!("../assets/skills/agent/HL03-compact.png")),
                ),
            ]
            .into(),
            jaw: image(include_bytes!(
                "../assets/animations/agent/skill-animation-review-50/jaw-atlas.png"
            )),
            rocket: image(include_bytes!(
                "../assets/animations/agent/skill-animation-review-55/rocket-sprite.png"
            )),
            blast: image(include_bytes!(
                "../assets/animations/agent/skill-animation-review-20/burst-atlas.png"
            )),
            claws: image(include_bytes!(
                "../assets/animations/agent/skill-animation-review-40/claws-atlas.png"
            )),
            impact: image(include_bytes!(
                "../assets/animations/agent/skill-animation-review-40/impact-atlas.png"
            )),
            plates: image(include_bytes!(
                "../assets/animations/agent/skill-animation-review-42/plates-atlas.png"
            )),
            reach: image(include_bytes!(
                "../assets/animations/agent/skill-animation-review-45/allonge-atlas.png"
            )),
            line: image(include_bytes!(
                "../assets/animations/agent/skill-animation-review-45/fente-atlas.png"
            )),
        }
    }
}
fn alpha(mut color: Color, opacity: f32) -> Color {
    color.a = opacity.clamp(0.0, 1.0);
    color
}

pub(super) fn draw(
    game: &GameState,
    now: f64,
    reduced: bool,
    background: bool,
    rect: impl Fn(project_rl::entity::EntityId, GridPos) -> Option<Rect>,
) {
    if background {
        return;
    }
    let states = game.agent_weapon_statuses();
    if states.is_empty() {
        return;
    }
    ART.with(|cache| {
        let mut cache = cache.borrow_mut();
        let art = cache.get_or_insert_with(Art::load);
        let player_count = states
            .iter()
            .filter(|(e, _, _, _)| *e == game.player_id())
            .count();
        let mut columns = BTreeMap::new();
        for (entity, skill, remaining, charges) in states {
            let Some(actor) = game.actors().get(entity) else {
                continue;
            };
            if !game.player_visibility().is_visible(actor.position()) {
                continue;
            }
            let Some(r) = rect(entity, actor.position()) else {
                continue;
            };
            if skill == "HL02" {
                let at = r.center() + vec2(0.0, -45.0 * r.w / 64.0);
                for i in 0..3 {
                    let angle = if reduced {
                        0.0
                    } else {
                        crate::animation_policy::clock(now) as f32 * 4.0
                    } + i as f32 * std::f32::consts::TAU / 3.0;
                    let p = at + vec2(angle.cos() * 14.0, angle.sin() * 5.0) * (r.w / 64.0);
                    for (width, length, color) in [
                        (4.0, 4.0, INK),
                        (2.4, 3.0, Color::new(0.90, 0.72, 0.40, 1.0)),
                    ] {
                        let length = length * r.w / 64.0;
                        draw_line(
                            p.x - length,
                            p.y,
                            p.x + length,
                            p.y,
                            width * r.w / 64.0,
                            color,
                        );
                        draw_line(
                            p.x,
                            p.y - length,
                            p.x,
                            p.y + length,
                            width * r.w / 64.0,
                            color,
                        );
                    }
                }
                continue;
            }
            let Some(icon) = art.icons.get(skill.as_str()) else {
                continue;
            };
            let index = columns.entry(entity).or_insert(0_u16);
            let size = r.w * 0.375;
            let at = if entity == game.player_id() {
                let count = player_count;
                vec2(
                    r.center().x - (count as f32 * (size + 2.0) - 2.0) / 2.0
                        + f32::from(*index) * (size + 2.0),
                    r.y - size - 4.0,
                )
            } else {
                vec2(
                    r.center().x - size / 2.0 + f32::from(*index) * (size + 2.0),
                    r.y - size * 0.8,
                )
            };
            *index += 1;
            draw_texture_ex(
                icon,
                at.x,
                at.y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(size, size)),
                    ..Default::default()
                },
            );
            if entity == game.player_id() && (remaining > 0 || charges > 0) {
                let text = if charges > 0 {
                    charges.to_string()
                } else {
                    remaining.to_string()
                };
                draw_rectangle(at.x + size - 11.0, at.y + size - 12.0, 12.0, 13.0, INK);
                draw_text(&text, at.x + size - 10.0, at.y + size - 1.0, 13.0, WHITE);
            }
            let icon_rect = Rect::new(at.x, at.y, size, size);
            if icon_rect.contains(vec2(mouse_position().0, mouse_position().1)) {
                let title = project_rl::skills::agent::definition(&skill)
                    .map_or(skill.as_str(), |s| s.title.as_str());
                let text = if charges > 0 {
                    format!("{title} · {charges}")
                } else if remaining > 0 {
                    format!("{title} · {remaining}")
                } else {
                    title.into()
                };
                let width = measure_text(&text, None, 16, 1.0).width + 14.0;
                draw_rectangle(at.x, at.y - 26.0, width, 23.0, alpha(INK, 0.97));
                draw_text(&text, at.x + 7.0, at.y - 9.0, 16.0, WHITE);
            }
        }
    });
}

/// Copy of the accepted fast physical tracer from ANIM-S22/S23, scaled to tile.
fn tracer(from: Vec2, to: Vec2, age: f32, piercing: bool, scale: f32) {
    if !(0.0..0.12).contains(&age) {
        return;
    }
    let delta = to - from;
    let direction = delta.normalize_or_zero();
    let head = from.lerp(to, age / 0.12);
    let length = if piercing { 50_f32 } else { 25_f32 } * scale;
    let tail = head - direction * length.min((head - from).length());
    for (width, color) in [
        (8.0, INK),
        (5.0, Color::new(0.88, 0.63, 0.29, 1.0)),
        (2.4, Color::new(1.0, 0.96, 0.79, 1.0)),
    ] {
        draw_line(tail.x, tail.y, head.x, head.y, width * scale, color);
    }
    draw_circle(head.x, head.y, 2.0 * scale, WHITE);
}

pub(super) fn burst(
    skill: &str,
    origin: GridPos,
    target: GridPos,
    radius: u16,
    age: f32,
    map: &Map,
    background: bool,
    rect: impl Fn(GridPos) -> Option<Rect>,
    clip: Rect,
    cells: Option<&[GridPos]>,
) {
    if !(0.0..0.8).contains(&age) {
        return;
    }
    let (from, to) = match (rect(origin), rect(target)) {
        (Some(from), to) => (
            from,
            to.unwrap_or_else(|| {
                Rect::new(
                    from.x + (target.x - origin.x) as f32 * from.w,
                    from.y + (target.y - origin.y) as f32 * from.h,
                    from.w,
                    from.h,
                )
            }),
        ),
        (None, Some(to)) if skill.ends_with("-BLAST") => (
            Rect::new(
                to.x + (origin.x - target.x) as f32 * to.w,
                to.y + (origin.y - target.y) as f32 * to.h,
                to.w,
                to.h,
            ),
            to,
        ),
        _ => return,
    };
    let scale = from.w / 64.0;
    let source = from.center() - vec2(0.0, 8.0 * scale);
    let impact = to.center() - vec2(0.0, 8.0 * scale);
    let direction = (impact - source).normalize_or_zero();
    let side = vec2(-direction.y, direction.x);
    if matches!(
        skill,
        "M01_claw"
            | "M01_laceration"
            | "TR61_strike"
            | "TR61_shockwave"
            | "TR61_crushing"
            | "TR63_heurt"
            | "TR63_discharge"
            | "TR65_reach"
            | "TR65_line"
    ) {
        ART.with(|cache| {
            let mut cache = cache.borrow_mut();
            let art = cache.get_or_insert_with(Art::load);
            let angle = direction.y.atan2(direction.x);
            let ground = matches!(
                skill,
                "TR61_strike" | "TR61_shockwave" | "TR61_crushing" | "TR63_discharge"
            );
            if ground != background {
                return;
            }
            let (texture, at, size, start, duration, opacity, rotation) = match skill {
                "M01_claw" => (
                    &art.claws,
                    source.lerp(impact, 0.75),
                    vec2(98.0, 90.0),
                    0.03,
                    0.30,
                    1.0,
                    angle,
                ),
                "M01_laceration" => (
                    &art.claws,
                    source.lerp(impact, 0.75),
                    vec2(126.0, 200.0),
                    0.03,
                    0.30,
                    1.0,
                    angle,
                ),
                "TR61_strike" => (
                    &art.impact,
                    to.center() + vec2(0.0, 9.0 * scale),
                    vec2(96.0, 86.0),
                    0.06,
                    0.30,
                    0.95,
                    0.0,
                ),
                "TR61_crushing" => (
                    &art.impact,
                    to.center(),
                    vec2(132.0, 132.0),
                    0.07,
                    0.36,
                    1.0,
                    0.0,
                ),
                "TR61_shockwave" => (
                    &art.impact,
                    from.center() + vec2(0.0, 17.0 * scale),
                    vec2(230.0, 210.0),
                    0.03,
                    0.40,
                    1.0,
                    0.0,
                ),
                "TR63_heurt" => (
                    &art.impact,
                    to.center(),
                    vec2(96.0, 88.0),
                    0.02,
                    0.32,
                    1.0,
                    0.0,
                ),
                "TR63_discharge" => (
                    &art.plates,
                    from.center() + vec2(0.0, 12.0 * scale),
                    vec2(230.0, 206.0),
                    0.01,
                    0.38,
                    1.0,
                    0.0,
                ),
                "TR65_reach" => {
                    let start = from.center()
                        + vec2(if direction.x < 0.0 { -18.0 } else { 18.0 }, 12.0) * scale;
                    let axis = (to.center() - start).normalize_or_zero();
                    let width = start.distance(to.center()) / 0.9;
                    (
                        &art.reach,
                        start + axis * width * 0.45,
                        vec2(width / scale, 64.0),
                        0.0,
                        0.32,
                        if age > 0.27 { (0.32 - age) / 0.05 } else { 1.0 },
                        axis.y.atan2(axis.x),
                    )
                }
                _ => {
                    let start = from.center() + direction * 18.0 * scale;
                    let finish = to.center() + direction * 20.0 * scale;
                    (
                        &art.line,
                        (start + finish) * 0.5,
                        vec2(start.distance(finish) / scale, 100.0),
                        0.0,
                        0.32,
                        0.95,
                        angle,
                    )
                }
            };
            if (start..start + duration).contains(&age) {
                masked_sheet(
                    texture,
                    at,
                    size * scale,
                    ((age - start) / duration * 6.0) as usize,
                    opacity,
                    rotation,
                    origin,
                    map,
                    &rect,
                    from,
                    clip,
                    cells,
                );
            }
        });
        return;
    }
    if background {
        return;
    }
    match skill {
        "B01" => {} // Existing B01-A01 renderer remains the sole owner.
        "ME02" | "ME03" | "ME04" | "ME05" => {
            if age < 0.2 {
                let k = age / 0.2;
                if skill == "ME03" {
                    crate::melee_strike_visuals::cut(
                        source,
                        source.lerp(impact + direction * 25.0 * scale, k),
                        6.0 * scale,
                        1.0 - k,
                    );
                } else {
                    crate::melee_strike_visuals::cut(
                        impact - side * 27.0 * scale - direction * 12.0 * scale,
                        impact + side * (20.0 + k * 16.0) * scale + direction * 9.0 * scale,
                        7.0 * scale,
                        (1.0 - k) * 0.85,
                    );
                }
            }
            if skill != "ME02" {
                crate::melee_strike_visuals::impact(
                    impact,
                    age - 0.1,
                    if skill == "ME04" { 1.3 } else { 0.85 },
                    skill == "ME03",
                    from.w,
                );
            }
        }
        "ME06" => {
            if age < 0.25 {
                let k = age / 0.25;
                for n in 0..8 {
                    let angle = n as f32 * std::f32::consts::TAU / 8.0 + k * 1.6;
                    let a = source + vec2(angle.cos(), angle.sin()) * 42.0 * scale;
                    let b = source + vec2((angle + 0.6).cos(), (angle + 0.6).sin()) * 48.0 * scale;
                    crate::melee_strike_visuals::cut(a, b, 6.0 * scale, 1.0 - k);
                }
            }
        }
        "DI01" | "DI03" | "DI04" | "DI07" => {
            tracer(
                source + direction * 24.0 * scale,
                impact,
                age,
                skill == "DI01",
                scale,
            );
            if age >= 0.12 {
                crate::melee_strike_visuals::impact(impact, age - 0.12, 0.45, false, from.w);
            }
        }
        "DI02" => {
            for sequence in 0..2 {
                let a = age - sequence as f32 * 0.18;
                tracer(source + direction * 24.0 * scale, impact, a, false, scale);
            }
        }
        "DI05" => {
            if age < 0.4 {
                let head = source.lerp(impact, (age / 0.12).min(1.0));
                for (width, color) in [
                    (5.0, INK),
                    (2.3, Color::new(0.55, 0.62, 0.64, 1.0)),
                    (0.8, Color::new(0.92, 0.94, 0.92, 1.0)),
                ] {
                    draw_line(
                        source.x,
                        source.y,
                        head.x,
                        head.y,
                        width * scale,
                        alpha(color, (0.4 - age).min(0.1) * 10.0),
                    );
                }
                draw_triangle(
                    head + direction * 4.0 * scale,
                    head - direction * 18.0 * scale + side * 5.0 * scale,
                    head - direction * 18.0 * scale - side * 5.0 * scale,
                    WHITE,
                );
            }
        }
        "T01" => {
            if age < 0.22 {
                let k = (age / 0.22).clamp(0.0, 1.0);
                let r = (18.0 + 22.0 * (1.0 - k).powi(2)) * scale;
                for (dx, dy) in [(-1.0, -1.0), (-1.0, 1.0), (1.0, -1.0), (1.0, 1.0)] {
                    let q = impact + vec2(dx, dy) * r;
                    for (width, color) in [
                        (6.0, INK),
                        (3.3, Color::new(1.0, 0.22, 0.19, 1.0)),
                        (0.9, Color::new(1.0, 0.78, 0.55, 1.0)),
                    ] {
                        draw_line(q.x, q.y, q.x - dx * 10.0 * scale, q.y, width * scale, color);
                        draw_line(q.x, q.y, q.x, q.y - dy * 10.0 * scale, width * scale, color);
                    }
                }
            }
        }
        "BM02-HIT" => {
            ART.with(|cache| {
                let mut cache = cache.borrow_mut();
                let art = cache.get_or_insert_with(Art::load);
                if age < 0.4 {
                    let frame = ((age / 0.4 * 6.0) as usize).min(5);
                    draw_texture_ex(
                        &art.jaw,
                        impact.x - 46.0 * scale,
                        impact.y - 46.0 * scale,
                        alpha(WHITE, 0.90),
                        DrawTextureParams {
                            source: Some(Rect::new(frame as f32 * 256.0, 0.0, 256.0, 256.0)),
                            dest_size: Some(vec2(92.0 * scale, 92.0 * scale)),
                            ..Default::default()
                        },
                    );
                }
            });
        }
        id if id.starts_with("HL") && id.ends_with("-BULLET") => {
            tracer(source + direction * 24.0 * scale, impact, age, false, scale);
            if age >= 0.12 {
                crate::melee_strike_visuals::impact(impact, age - 0.12, 0.45, false, from.w);
            }
        }
        id if id.starts_with("HL") => {
            ART.with(|cache| {
                let mut cache = cache.borrow_mut();
                let art = cache.get_or_insert_with(Art::load);
                let blast_only = id.ends_with("-BLAST");
                let flight = if blast_only { 0.0 } else { 0.18 };
                if age < flight {
                    let p = (source + direction * 30.0 * scale).lerp(impact, age / flight);
                    let width = 50.0 * scale;
                    let height = width * art.rocket.height() / art.rocket.width();
                    draw_texture_ex(
                        &art.rocket,
                        p.x - width / 2.0,
                        p.y - height / 2.0,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(width, height)),
                            rotation: direction.y.atan2(direction.x),
                            pivot: Some(p),
                            ..Default::default()
                        },
                    );
                }
                if id.ends_with("-FLIGHT") {
                    return;
                }
                let t = age - flight;
                if !(0.0..0.55).contains(&t) {
                    return;
                }
                let frame = ((t / 0.55 * 6.0) as usize).min(5);
                if crate::effect_geometry::enabled() && cells.is_some() {
                    crate::natural_effect::sheet(
                        &art.blast, frame, target, radius, 0.9, false, map, &rect,
                    );
                    return;
                }
                let size = 134.0 * scale * (f32::from(radius.max(1)) + 0.5) / 1.5;
                let dest = Rect::new(
                    impact.x - size * 132.0 / 256.0,
                    impact.y - size * 146.0 / 256.0,
                    size,
                    size,
                );
                // Clip only to visible, unobstructed scenery; the painted edge may
                // naturally overflow the mechanical damage footprint.
                let span = (size / from.w).ceil() as i32 + 2;
                for y in target.y - span..=target.y + span {
                    for x in target.x - span..=target.x + span {
                        let at = GridPos::new(x, y);
                        let Some(cell) = rect(at) else {
                            continue;
                        };
                        if !map.is_walkable(at)
                            || !project_rl::world::has_line_of_sight(map, target, at, true)
                        {
                            continue;
                        }
                        let x = dest.x.max(cell.x);
                        let y = dest.y.max(cell.y);
                        let w = dest.right().min(cell.right()) - x;
                        let h = dest.bottom().min(cell.bottom()) - y;
                        if w <= 0.0 || h <= 0.0 {
                            continue;
                        }
                        draw_texture_ex(
                            &art.blast,
                            x,
                            y,
                            alpha(WHITE, 0.9),
                            DrawTextureParams {
                                source: Some(Rect::new(
                                    frame as f32 * 256.0 + (x - dest.x) * 256.0 / size,
                                    (y - dest.y) * 256.0 / size,
                                    w * 256.0 / size,
                                    h * 256.0 / size,
                                )),
                                dest_size: Some(vec2(w, h)),
                                ..Default::default()
                            },
                        );
                    }
                }
            });
        }
        _ => {}
    }
}

/// Paint keeps its organic outline, while visible terrain alone cuts it.
/// Scissors use top-left screen coordinates in Macroquad's quad_gl API.
fn masked_sheet(
    texture: &Texture2D,
    at: Vec2,
    size: Vec2,
    frame: usize,
    opacity: f32,
    rotation: f32,
    origin: GridPos,
    map: &Map,
    rect: &impl Fn(GridPos) -> Option<Rect>,
    origin_rect: Rect,
    clip: Rect,
    _cells: Option<&[GridPos]>,
) {
    let dest = Rect::new(at.x - size.x * 0.5, at.y - size.y * 0.5, size.x, size.y);
    crate::effect_geometry::clipped(clip, clip, || {
        crate::natural_effect::draw(
            texture,
            Rect::new(frame.min(5) as f32 * 256., 0., 256., 256.),
            dest,
            rotation,
            opacity,
            false,
            origin,
            origin_rect,
            map,
            rect,
        )
    });
}
