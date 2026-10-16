//! Isolated animation/laboratory proposal. No GameState, campaign or save mutations.
//! Kept behind a debug entry point until the player approves each visual candidate.
use crate::game_art::{self, Visual};
use crate::terminal_view::PlayerWeaponPose;
use crate::ui_theme::{draw_text, draw_text_bold};
use macroquad::prelude::*;
use project_rl::world::GridPos;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::Path;

mod assault;
mod fire;
mod fire_painted;
mod melee;

const IDS: [&str; 3] = ["ME02", "FE01", "FR01"];
const TITLES: [&str; 3] = ["Fauchage", "Braise", "Aiguille de givre"];
const INK: Color = Color::new(0.025, 0.036, 0.045, 1.0);
const PANEL: Color = Color::new(0.050, 0.068, 0.079, 1.0);
const TEXT: Color = Color::new(0.82, 0.87, 0.87, 1.0);
const DIM: Color = Color::new(0.47, 0.55, 0.58, 1.0);

#[derive(Deserialize)]
struct Catalogue {
    skills: Vec<Skill>,
}
#[derive(Deserialize)]
struct Skill {
    id: String,
    parameters: BTreeMap<String, serde_json::Value>,
}

fn parameter(skill: usize, key: &str, rank: u16) -> u16 {
    parameter_for(IDS[skill], key, rank)
}

fn parameter_for(id: &str, key: &str, rank: u16) -> u16 {
    static DATA: std::sync::OnceLock<Catalogue> = std::sync::OnceLock::new();
    let data = DATA.get_or_init(|| {
        serde_json::from_str(include_str!("../assets/skills/agent/catalogue.json")).unwrap()
    });
    let p = &data.skills.iter().find(|s| s.id == id).unwrap().parameters[key];
    let n = f64::from(rank.saturating_sub(1));
    let number = |k: &str| p[k].as_f64().unwrap_or(0.0);
    match p["kind"].as_str().unwrap() {
        "constant" => number("value") as u16,
        "linear" => (number("base") + n * number("gain_per_rank")) as u16,
        "stepped" => {
            (number("base")
                + ((n + number("rank_offset")) / number("every_ranks")).floor()
                    * number("gain_per_step")) as u16
        }
        _ => unreachable!("only display cost and duration curves in this visual proposal"),
    }
}

struct Review {
    skill: usize,
    tile: f32,
    learned: u16,
    bonus: u16,
    level: u32,
    attributes: [u32; 6],
    started: Option<f64>,
    paused: bool,
    speed: f64,
    animation_age: f64,
    turn: u32,
    cooldown: u16,
    status: u16,
    free_cooldowns: bool,
}
impl Default for Review {
    fn default() -> Self {
        Self {
            skill: 0,
            tile: 64.0,
            learned: 9,
            bonus: 2,
            level: 1,
            attributes: [5; 6],
            started: None,
            paused: false,
            speed: 1.0,
            animation_age: 0.0,
            turn: 0,
            cooldown: 0,
            status: 0,
            free_cooldowns: true,
        }
    }
}
impl Review {
    fn rank(&self) -> u16 {
        self.learned + self.bonus
    }
    fn replay(&mut self, now: f64) {
        self.started = Some(now);
        self.animation_age = 0.0;
        self.paused = false;
    }
    fn launch(&mut self, now: f64) {
        if !self.free_cooldowns && self.cooldown > 0 {
            return;
        }
        self.replay(now);
        self.cooldown = parameter(self.skill, "cooldown_turns", self.rank());
        self.status = match self.skill {
            1 => parameter(1, "burn_duration_turns", self.rank()),
            2 => parameter(2, "freeze_duration_turns", self.rank()),
            _ => 0,
        };
    }
    fn next_turn(&mut self) {
        self.turn += 1;
        self.cooldown = self.cooldown.saturating_sub(1);
        self.status = self.status.saturating_sub(1);
    }
    fn reset(&mut self) {
        self.started = None;
        self.animation_age = 0.0;
        self.cooldown = 0;
        self.status = 0;
        self.turn = 0;
    }
}

fn button(rect: Rect, label: &str, enabled: bool) -> bool {
    let hover = rect.contains(vec2(mouse_position().0, mouse_position().1));
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if hover && enabled {
            Color::new(0.12, 0.13, 0.18, 1.0)
        } else {
            PANEL
        },
    );
    draw_rectangle_lines(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        1.0,
        if enabled { DIM } else { INK },
    );
    draw_text(
        label,
        rect.x + 9.0,
        rect.y + 21.0,
        15.0,
        if enabled { TEXT } else { DIM },
    );
    enabled && hover && is_mouse_button_pressed(MouseButton::Left)
}
fn number_row(label: &str, value: &mut u32, x: f32, y: f32, step: u32, minimum: u32, maximum: u32) {
    draw_text(label, x, y + 21.0, 14.0, DIM);
    draw_text(value.to_string(), x + 106.0, y + 21.0, 16.0, TEXT);
    if button(Rect::new(x + 150.0, y, 27.0, 29.0), "−", *value > minimum) {
        *value = value.saturating_sub(step).max(minimum);
    }
    if button(Rect::new(x + 181.0, y, 27.0, 29.0), "+", *value < maximum) {
        *value = value.saturating_add(step).min(maximum);
    }
}
fn alpha(mut c: Color, a: f32) -> Color {
    c.a = a.clamp(0.0, 1.0);
    c
}

fn stage_rect(width: f32, height: f32, tile: f32) -> Rect {
    let available = Rect::new(254.0, 104.0, width - 508.0, height - 211.0);
    Rect::new(
        available.x + (available.w - tile * 6.0) * 0.5,
        available.y + (available.h - tile * 5.0) * 0.5,
        tile * 6.0,
        tile * 5.0,
    )
}
fn draw_scene_base(skill: usize, rect: Rect, tile: f32) {
    for y in 0..5 {
        for x in 0..6 {
            game_art::draw_world(
                Rect::new(
                    rect.x + x as f32 * tile,
                    rect.y + y as f32 * tile,
                    tile,
                    tile,
                ),
                Visual::Terrain {
                    kind: crate::test_sector::Decor::Deck,
                    position: GridPos::new(x, y),
                },
                1.0,
            );
            draw_rectangle_lines(
                rect.x + x as f32 * tile,
                rect.y + y as f32 * tile,
                tile,
                tile,
                0.7,
                Color::new(0.15, 0.18, 0.19, 0.75),
            );
        }
    }
    for &(x, y) in if skill == 0 {
        &[(2.0, 1.0), (2.0, 2.0), (2.0, 3.0)][..]
    } else {
        &[(4.0, 2.0)][..]
    } {
        draw_rectangle(
            rect.x + x * tile,
            rect.y + y * tile,
            tile,
            tile,
            Color::new(0.50, 0.27, 0.17, 0.13),
        );
        game_art::draw_world(
            Rect::new(rect.x + x * tile, rect.y + y * tile, tile, tile),
            Visual::Bestiary {
                number: 12,
                corpse: false,
            },
            1.0,
        );
    }
    let visual = game_art::wear(
        game_art::player(
            Some(if skill == 0 {
                "core:bastion"
            } else {
                "core:arcaniste"
            }),
            if skill == 0 {
                PlayerWeaponPose::Melee
            } else {
                PlayerWeaponPose::Unarmed
            },
            if skill == 0 {
                Some("core:melee_epee_longue")
            } else {
                None
            },
        ),
        [(
            "core:body_armor".into(),
            if skill == 0 {
                "core:tenue_assaut"
            } else {
                "core:tenue_rituelle"
            }
            .into(),
        )],
    );
    game_art::draw_world(
        Rect::new(rect.x + tile, rect.y + 2.0 * tile, tile, tile),
        visual,
        1.0,
    );
}

// A02: visually stronger proposals designed for native 64 px cells.
// Particles are deterministic and never consume gameplay RNG or alter targeting.
fn noise(index: u32) -> f32 {
    let mut value = index.wrapping_add(19).wrapping_mul(2_654_435_761);
    value ^= value >> 16;
    value = value.wrapping_mul(2_246_822_519);
    value as f32 / u32::MAX as f32
}

fn glow(at: Vec2, radius: f32, color: Color, opacity: f32) {
    for (size, strength) in [(2.2, 0.05), (1.6, 0.08), (1.15, 0.13), (0.7, 0.22)] {
        draw_circle(at.x, at.y, radius * size, alpha(color, opacity * strength));
    }
}

fn luminous_line(from: Vec2, to: Vec2, width: f32, color: Color, opacity: f32) {
    for (size, strength) in [(5.0, 0.045), (2.8, 0.10), (1.4, 0.28), (0.6, 0.9)] {
        draw_line(
            from.x,
            from.y,
            to.x,
            to.y,
            width * size,
            alpha(color, opacity * strength),
        );
    }
}

fn shard(at: Vec2, direction: Vec2, length: f32, width: f32, color: Color, opacity: f32) {
    let normal = vec2(-direction.y, direction.x);
    let tip = at + direction * length * 0.65;
    let heel = at - direction * length * 0.35;
    let left = at + normal * width;
    let right = at - normal * width;
    draw_triangle(tip, left, heel, alpha(color, opacity));
    draw_triangle(
        tip,
        right,
        heel,
        alpha(
            Color::new(color.r * 0.45, color.g * 0.55, color.b * 0.60, 1.0),
            opacity,
        ),
    );
    draw_line(
        tip.x,
        tip.y,
        heel.x,
        heel.y,
        1.2,
        alpha(Color::new(0.9, 0.97, 1.0, 1.0), opacity * 0.8),
    );
}

fn crescent(player: Vec2, tile: f32, start: f32, end: f32, life: f32) {
    // Shared vertices and interpolated colours avoid seams between glowing segments.
    let mut mesh = Mesh {
        vertices: Vec::with_capacity(164),
        indices: Vec::with_capacity(720),
        texture: None,
    };
    for i in 0..=40 {
        let k = i as f32 / 40.0;
        let angle = start + (end - start) * k;
        let direction = vec2(angle.cos(), angle.sin());
        let width = tile * 0.34 * (k * std::f32::consts::PI).sin();
        let outer = tile * 1.52;
        let strength = k.sqrt() * life;
        for (radius, color) in [
            (outer - width, alpha(Color::new(0.32, 0.60, 0.74, 1.0), 0.0)),
            (
                outer - width * 0.50,
                alpha(Color::new(0.65, 0.88, 0.95, 1.0), strength * 0.75),
            ),
            (outer, alpha(Color::new(0.93, 0.99, 1.0, 1.0), strength)),
            (
                outer + tile * 0.12,
                alpha(Color::new(0.24, 0.65, 0.9, 1.0), 0.0),
            ),
        ] {
            let at = player + direction * radius;
            mesh.vertices
                .push(Vertex::new(at.x, at.y, 0.0, 0.0, 0.0, color));
        }
        if i > 0 {
            let previous = (i - 1) * 4;
            let current = i * 4;
            for strip in 0..3 {
                mesh.indices.extend_from_slice(&[
                    previous + strip,
                    previous + strip + 1,
                    current + strip,
                    current + strip,
                    previous + strip + 1,
                    current + strip + 1,
                ]);
            }
        }
    }
    draw_mesh(&mesh);
}

fn sweeping_blade(player: Vec2, tile: f32, time: f32, rect: Rect) {
    let phase = (time / 0.42).clamp(0.0, 1.0);
    let end = -1.10 + phase * 2.20;
    let life = (1.0 - (time - 0.42).max(0.0) / 0.38).clamp(0.0, 1.0);
    if time < 0.8 {
        // Filled, tapered steel crescent. The trailing edge remains a single sweep.
        let start = (-1.10_f32).max(end - 1.10);
        crescent(player, tile, start, end, life);
        let tip = player + vec2(end.cos(), end.sin()) * tile * 1.52;
        glow(tip, tile * 0.12, Color::new(0.56, 0.86, 1.0, 1.0), life);
        for n in 0..22 {
            let lag = noise(n) * 0.13;
            let dt = time - lag;
            if !(0.0..0.50).contains(&dt) {
                continue;
            }
            let a = -1.10 + (dt / 0.42).clamp(0.0, 1.0) * 2.2;
            let at = player + vec2(a.cos(), a.sin()) * tile * (1.52 + noise(n + 50) * dt * 0.9);
            draw_circle(
                at.x,
                at.y,
                tile * (0.01 + noise(n + 100) * 0.016),
                alpha(Color::new(0.82, 0.96, 1.0, 1.0), life * 0.85),
            );
        }
    }
    for (index, y) in [1.5, 2.5, 3.5].iter().enumerate() {
        let dt = time - (0.10 + index as f32 * 0.14);
        if !(0.0..0.58).contains(&dt) {
            continue;
        }
        let at = vec2(rect.x + tile * 2.5, rect.y + tile * *y);
        let life = 1.0 - dt / 0.58;
        glow(at, tile * 0.35, Color::new(0.95, 0.32, 0.19, 1.0), life);
        luminous_line(
            at - vec2(tile * 0.20, tile * 0.30),
            at + vec2(tile * 0.20, tile * 0.24),
            tile * 0.048,
            Color::new(1.0, 0.78, 0.55, 1.0),
            life,
        );
        for n in 0..20 {
            let angle = noise(n + index as u32 * 29) * std::f32::consts::TAU;
            let direction = vec2(angle.cos(), angle.sin());
            let speed = 0.35 + noise(n + 120) * 0.85;
            let point =
                at + direction * tile * (0.04 + dt * speed) + vec2(0.0, dt * dt * tile * 0.3);
            let blood = n % 3 == 0;
            let color = if blood {
                Color::new(0.52, 0.06, 0.04, 1.0)
            } else {
                Color::new(1.0, 0.63, 0.27, 1.0)
            };
            let radius = tile * if blood { 0.027 } else { 0.015 };
            draw_circle(point.x, point.y, radius * life, alpha(color, life));
            if !blood {
                luminous_line(
                    point - direction * tile * 0.1,
                    point,
                    tile * 0.024,
                    color,
                    life,
                );
            }
        }
    }
}

fn elemental_projectile(
    skill: usize,
    player: Vec2,
    target: Vec2,
    tile: f32,
    time: f32,
    status: u16,
) {
    let fire = skill == 1;
    let color = if fire {
        Color::new(1.0, 0.34, 0.055, 1.0)
    } else {
        Color::new(0.20, 0.76, 1.0, 1.0)
    };
    let core = if fire {
        Color::new(1.0, 0.93, 0.60, 1.0)
    } else {
        Color::new(0.90, 0.99, 1.0, 1.0)
    };
    let start = player + vec2(tile * 0.29, -tile * 0.06);
    let hit = target + vec2(0.0, -tile * 0.05);
    let direction = (hit - start).normalize();
    let normal = vec2(-direction.y, direction.x);
    let flight = 0.55;
    if time < 0.28 {
        let power = (time / 0.28 * std::f32::consts::PI).sin();
        glow(start, tile * 0.30, color, power);
        draw_circle_lines(
            start.x,
            start.y,
            tile * (0.08 + time * 0.3),
            tile * 0.022,
            alpha(core, power),
        );
        for n in 0..9 {
            let angle = n as f32 * std::f32::consts::TAU / 9.0 + time * 3.0;
            let point = start + vec2(angle.cos(), angle.sin()) * tile * (0.20 - time * 0.35);
            draw_circle(point.x, point.y, tile * 0.023 * power, alpha(core, power));
        }
    }
    if time < flight {
        let at = start.lerp(hit, time / flight);
        let tail_length = (tile * (0.55 + time * 0.70)).min((at - start).length() + tile * 0.15);
        for i in (0..28).rev() {
            let k = i as f32 / 28.0;
            let curl = (time * 20.0 - k * 6.0).sin() * tile * 0.035 * k;
            let point = at - direction * tail_length * k + normal * curl;
            let width = tile * (0.115 * (1.0 - k) + 0.01);
            glow(point, width, color, (1.0 - k) * 0.30);
            draw_circle(
                point.x,
                point.y,
                width * 0.65,
                alpha(color, (1.0 - k) * 0.60),
            );
            if i < 15 {
                draw_circle(
                    point.x,
                    point.y,
                    width * 0.23,
                    alpha(core, (1.0 - k) * 0.75),
                );
            }
        }
        for n in 0..26 {
            let k = noise(n);
            let cross = (noise(n + 31) - 0.5) * tile * 0.34 * k;
            let point = at - direction * tail_length * k + normal * cross;
            let opacity = (1.0 - k) * (0.4 + noise(n + 59) * 0.6);
            if fire {
                draw_circle(
                    point.x,
                    point.y,
                    tile * (0.018 + noise(n + 14) * 0.026),
                    alpha(color, opacity),
                );
            } else {
                shard(point, direction, tile * 0.12, tile * 0.018, color, opacity);
            }
        }
        glow(at, tile * 0.24, color, 1.0);
        if fire {
            draw_ellipse(at.x, at.y, tile * 0.18, tile * 0.09, -1.0, color);
            draw_ellipse(
                at.x + direction.x * tile * 0.035,
                at.y,
                tile * 0.125,
                tile * 0.045,
                -1.0,
                core,
            );
        } else {
            shard(at, direction, tile * 0.48, tile * 0.075, core, 1.0);
            luminous_line(
                at - direction * tile * 0.40,
                at + direction * tile * 0.28,
                tile * 0.042,
                core,
                1.0,
            );
        }
        return;
    }
    let impact = time - flight;
    let life = (1.0 - impact / 0.70).clamp(0.0, 1.0);
    if impact < 0.70 {
        glow(hit, tile * (0.34 + impact * 0.42), color, life);
        glow(hit, tile * 0.21, core, life * life);
        let ring = tile * (0.11 + impact * 0.82);
        draw_circle_lines(
            hit.x,
            hit.y,
            ring,
            tile * 0.034 * life,
            alpha(color, life * 0.6),
        );
        for n in 0..36 {
            let angle = noise(n + 220) * std::f32::consts::TAU;
            let vector = vec2(angle.cos(), angle.sin());
            let speed = 0.30 + noise(n + 90) * 0.70;
            let point = hit + vector * tile * (0.07 + impact * speed);
            if fire {
                let rise = vec2(0.0, -tile * impact * impact * 0.40);
                luminous_line(
                    point + rise - vector * tile * 0.09,
                    point + rise,
                    tile * 0.03,
                    color,
                    life,
                );
                draw_circle(
                    point.x + rise.x,
                    point.y + rise.y,
                    tile * 0.022 * life,
                    alpha(core, life),
                );
            } else {
                shard(
                    point,
                    vector,
                    tile * (0.15 + noise(n + 48) * 0.16) * life,
                    tile * 0.03 * life,
                    color,
                    life,
                );
            }
        }
        if !fire {
            for n in 0..7 {
                let angle = n as f32 * std::f32::consts::TAU / 7.0;
                let vector = vec2(angle.cos(), angle.sin());
                luminous_line(hit, hit + vector * tile * 0.42, tile * 0.018, core, life);
            }
        }
    }
    if status == 0 {
        return;
    }
    if fire {
        // Flames wrap the victim; these are status graphics, not a damage zone.
        glow(target, tile * 0.32, color, 0.45);
        for n in 0..11 {
            let phase = (time * 1.15 + noise(n + 160)).fract();
            let x = target.x + (noise(n + 33) - 0.5) * tile * 0.46;
            let base = vec2(x, target.y + tile * (0.27 - phase * 0.46));
            let height = tile * (0.16 + noise(n + 77) * 0.12) * (1.0 - phase);
            let sway = (time * 6.0 + n as f32).sin() * tile * 0.04;
            let top = base + vec2(sway, -height);
            let width = tile * (0.045 + noise(n + 49) * 0.028) * (1.0 - phase);
            draw_triangle(
                top,
                base - vec2(width, 0.0),
                base + vec2(width, 0.0),
                alpha(color, 0.65 * (1.0 - phase)),
            );
            draw_triangle(
                top + vec2(0.0, height * 0.27),
                base - vec2(width * 0.4, 0.0),
                base + vec2(width * 0.4, 0.0),
                alpha(core, 0.70 * (1.0 - phase)),
            );
        }
        for n in 0..10 {
            let phase = (time * 0.53 + noise(n + 80)).fract();
            let point = target + vec2((noise(n + 61) - 0.5) * tile * 0.54, -phase * tile * 0.64);
            draw_circle(
                point.x,
                point.y,
                tile * 0.019 * (1.0 - phase),
                alpha(core, 0.9 * (1.0 - phase)),
            );
        }
    } else {
        let feet = target + vec2(0.0, tile * 0.31);
        draw_ellipse(
            feet.x,
            feet.y,
            tile * 0.34,
            tile * 0.11,
            0.0,
            alpha(color, 0.22),
        );
        glow(feet, tile * 0.23, color, 0.28);
        for n in 0..7 {
            let x = feet.x + (n as f32 - 3.0) * tile * 0.09;
            let height = tile * (0.19 + noise(n + 24) * 0.18);
            shard(
                vec2(x, feet.y - height * 0.30),
                vec2((noise(n + 5) - 0.5) * 0.45, -1.0).normalize(),
                height,
                tile * 0.045,
                color,
                0.82,
            );
        }
        for n in 0..8 {
            let phase = (time * 0.4 + noise(n + 11)).fract();
            let point = feet + vec2((noise(n + 53) - 0.5) * tile * 0.55, -phase * tile * 0.45);
            draw_circle(
                point.x,
                point.y,
                tile * 0.013,
                alpha(core, (1.0 - phase) * 0.55),
            );
        }
    }
    draw_text(
        format!("{} · {}", if fire { "Brûlure" } else { "Gel" }, status),
        target.x - tile * 0.45,
        target.y - tile * 0.59,
        13.0,
        core,
    );
}

fn draw_scene(skill: usize, rect: Rect, tile: f32, age: Option<f32>, status: u16) {
    draw_scene_base(skill, rect, tile);
    let Some(time) = age else {
        return;
    };
    let player = vec2(rect.x + 1.5 * tile, rect.y + 2.5 * tile);
    let target = vec2(rect.x + 4.5 * tile, rect.y + 2.5 * tile);
    if time < 1.05 {
        draw_text_bold(
            TITLES[skill],
            player.x - tile * 0.65,
            player.y - tile * 0.68 - time * 10.0,
            15.0,
            alpha(TEXT, (1.0 - time / 1.05).sqrt()),
        );
    }
    if skill == 0 {
        sweeping_blade(player, tile, time, rect);
    } else {
        elemental_projectile(skill, player, target, tile, time, status);
    }
}

fn draw_review(state: &mut Review, dt: f64, interactive: bool) {
    clear_background(INK);
    let (w, h) = (screen_width(), screen_height());
    draw_text_bold("LABORATOIRE / COMPÉTENCES", 22.0, 35.0, 23.0, TEXT);
    draw_text(
        "LAB-S01 · prototype de revue · calcul des dégâts à raccorder au moteur",
        22.0,
        59.0,
        14.0,
        DIM,
    );
    draw_rectangle(16.0, 83.0, 224.0, h - 167.0, PANEL);
    draw_rectangle(w - 240.0, 83.0, 224.0, h - 167.0, PANEL);
    draw_text_bold("ANIMATIONS · LOT 01", 29.0, 110.0, 15.0, TEXT);
    for i in 0..3 {
        let rect = Rect::new(28.0, 127.0 + i as f32 * 43.0, 200.0, 34.0);
        if button(rect, &format!("0{}  {}", i + 1, TITLES[i]), true) && interactive {
            state.skill = i;
            state.reset();
        }
    }
    let mut learned = u32::from(state.learned);
    let mut bonus = u32::from(state.bonus);
    number_row("Rangs appris", &mut learned, 28.0, 278.0, 1, 1, 10);
    number_row("Bonus objet", &mut bonus, 28.0, 314.0, 1, 0, 30);
    state.learned = learned as u16;
    state.bonus = bonus as u16;
    draw_text_bold(
        format!("Rang effectif : {}", state.rank()),
        28.0,
        372.0,
        15.0,
        if state.bonus > 0 {
            Color::new(0.32, 0.76, 0.86, 1.0)
        } else {
            TEXT
        },
    );
    draw_text(
        format!(
            "Coût : {} énergie",
            parameter(state.skill, "energy", state.rank())
        ),
        28.0,
        399.0,
        14.0,
        DIM,
    );
    draw_text(
        format!(
            "Recharge : {} tours",
            parameter(state.skill, "cooldown_turns", state.rank())
        ),
        28.0,
        422.0,
        14.0,
        DIM,
    );
    let x = w - 226.0;
    draw_text_bold("AGENT · CONFIGURATION", x, 110.0, 15.0, TEXT);
    number_row("Niveau", &mut state.level, x, 127.0, 1, 1, 10000);
    for (i, name) in [
        "Force",
        "Agilité",
        "Vitalité",
        "Perception",
        "Intelligence",
        "Volonté",
    ]
    .iter()
    .enumerate()
    {
        number_row(
            name,
            &mut state.attributes[i],
            x,
            165.0 + i as f32 * 34.0,
            1,
            0,
            10000,
        );
    }
    if button(Rect::new(x, 378.0, 207.0, 31.0), "Respécialiser", true) && interactive {
        state.attributes = [5; 6];
        state.learned = 1;
        state.bonus = 0;
        state.reset();
    }
    if button(
        Rect::new(x, 418.0, 207.0, 31.0),
        if state.free_cooldowns {
            "Recharges libres : oui"
        } else {
            "Recharges libres : non"
        },
        true,
    ) && interactive
    {
        state.free_cooldowns = !state.free_cooldowns;
    }
    if !state.paused && state.started.is_some() {
        state.animation_age += dt * state.speed;
    }
    let rect = stage_rect(w, h, state.tile);
    draw_scene(
        state.skill,
        rect,
        state.tile,
        state.started.map(|_| state.animation_age as f32),
        state.status,
    );
    draw_text(
        format!(
            "Tour {} · recharge restante : {} tours",
            state.turn, state.cooldown
        ),
        260.0,
        100.0,
        14.0,
        DIM,
    );
    draw_text(
        if state.skill == 2 {
            "Aperçu du Gel déclenché · il n'est pas garanti en combat."
        } else {
            "Animation au temps réel · durées d'effets et recharges au tour."
        },
        260.0,
        h - 98.0,
        13.0,
        DIM,
    );
    let labels = [
        if state.cooldown > 0 && !state.free_cooldowns {
            "En recharge"
        } else {
            "Lancer"
        },
        "+1 tour",
        "Rejouer",
        if state.paused { "Lecture" } else { "Pause" },
        if state.speed == 0.5 {
            "Vitesse ×0,5"
        } else {
            "Vitesse ×1"
        },
        if state.tile == 64.0 {
            "Cases 64 px"
        } else {
            "Cases 32 px"
        },
        "Réinitialiser",
    ];
    let stride = (w - 40.0) / 7.0;
    for (i, label) in labels.iter().enumerate() {
        let enabled = i != 5 && (i != 0 || state.free_cooldowns || state.cooldown == 0);
        if button(
            Rect::new(20.0 + i as f32 * stride, h - 72.0, stride - 7.0, 32.0),
            label,
            enabled,
        ) && interactive
        {
            match i {
                0 => state.launch(get_time()),
                1 => state.next_turn(),
                2 => state.replay(get_time()),
                3 => state.paused = !state.paused,
                4 => state.speed = if state.speed == 0.5 { 1.0 } else { 0.5 },
                6 => state.reset(),
                _ => unreachable!(),
            }
        }
    }
    draw_text(
        "Entrée : lancer · Espace : +1 tour · R : rejouer · Échap : fermer",
        22.0,
        h - 16.0,
        13.0,
        DIM,
    );
}

pub(crate) async fn run() {
    if std::env::args().any(|arg| arg == "--lot-feu-final") {
        fire_painted::finale::run().await;
        return;
    }
    if std::env::args().any(|arg| arg == "--lot-feu-tours") {
        fire_painted::zones::run_held().await;
        return;
    }
    if std::env::args().any(|arg| arg == "--lot-feu-cadence") {
        fire_painted::zones::run(2.0 / 3.0).await;
        return;
    }
    if std::env::args().any(|arg| arg == "--lot-feu-zones") {
        fire_painted::zones::run(1.0).await;
        return;
    }
    if std::env::args().any(|arg| arg == "--lot-souffle-ouverture") {
        fire_painted::obstacles::overlay::stopped::expanded::run().await;
        return;
    }
    if std::env::args().any(|arg| arg == "--lot-souffle-arrete") {
        fire_painted::obstacles::overlay::stopped::run().await;
        return;
    }
    if std::env::args().any(|arg| arg == "--lot-souffle-superpose") {
        fire_painted::obstacles::overlay::run().await;
        return;
    }
    if std::env::args().any(|arg| arg == "--lot-souffle-particules") {
        fire_painted::obstacles::particles::run().await;
        return;
    }
    if std::env::args().any(|arg| arg == "--lot-souffle-continu") {
        fire_painted::obstacles::fluid::run().await;
        return;
    }
    if std::env::args().any(|arg| arg == "--lot-souffle-obstacles") {
        fire_painted::obstacles::run().await;
        return;
    }
    if std::env::args().any(|arg| arg == "--lot-feu-dessine") {
        fire_painted::run().await;
        return;
    }
    if std::env::args().any(|arg| arg == "--lot-feu") {
        fire::run().await;
        return;
    }
    if std::env::args().any(|arg| arg == "--lot-assaut") {
        assault::run().await;
        return;
    }
    if std::env::args().any(|arg| arg == "--lot-melee") {
        melee::run().await;
        return;
    }
    request_new_screen_size(1280.0, 800.0);
    let mut state = Review::default();
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        if is_key_pressed(KeyCode::Enter) {
            state.launch(get_time());
        }
        if is_key_pressed(KeyCode::Space) {
            state.next_turn();
        }
        if is_key_pressed(KeyCode::R) {
            state.replay(get_time());
        }
        draw_review(&mut state, f64::from(get_frame_time()), true);
        next_frame().await;
    }
}

pub(crate) async fn capture(output: &Path) -> Result<(), String> {
    if std::env::args().any(|arg| arg == "--lot-feu-final") {
        return fire_painted::finale::capture(output).await;
    }
    if std::env::args().any(|arg| arg == "--lot-feu-tours") {
        return fire_painted::zones::capture_held(output).await;
    }
    if std::env::args().any(|arg| arg == "--lot-feu-cadence") {
        return fire_painted::zones::capture(output, 2.0 / 3.0).await;
    }
    if std::env::args().any(|arg| arg == "--lot-feu-zones") {
        return fire_painted::zones::capture(output, 1.0).await;
    }
    if std::env::args().any(|arg| arg == "--lot-souffle-ouverture") {
        return fire_painted::obstacles::overlay::stopped::expanded::capture(output).await;
    }
    if std::env::args().any(|arg| arg == "--lot-souffle-arrete") {
        return fire_painted::obstacles::overlay::stopped::capture(output).await;
    }
    if std::env::args().any(|arg| arg == "--lot-souffle-superpose") {
        return fire_painted::obstacles::overlay::capture(output).await;
    }
    if std::env::args().any(|arg| arg == "--lot-souffle-particules") {
        return fire_painted::obstacles::particles::capture(output).await;
    }
    if std::env::args().any(|arg| arg == "--lot-souffle-continu") {
        return fire_painted::obstacles::fluid::capture(output).await;
    }
    if std::env::args().any(|arg| arg == "--lot-souffle-obstacles") {
        return fire_painted::obstacles::capture(output).await;
    }
    if std::env::args().any(|arg| arg == "--lot-feu-dessine") {
        return fire_painted::capture(output).await;
    }
    if std::env::args().any(|arg| arg == "--lot-feu") {
        return fire::capture(output).await;
    }
    if std::env::args().any(|arg| arg == "--lot-assaut") {
        return assault::capture(output).await;
    }
    if std::env::args().any(|arg| arg == "--lot-melee") {
        return melee::capture(output).await;
    }
    if output.exists() {
        return Err("Le dossier de revue existe déjà.".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(1280.0, 800.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let mut state = Review::default();
    state.launch(0.0);
    state.animation_age = 0.24;
    draw_review(&mut state, 0.0, false);
    crate::ui_capture::framebuffer()?
        .export_png(output.join("laboratoire-prototype.png").to_str().unwrap());
    state.free_cooldowns = false;
    draw_review(&mut state, 0.0, false);
    crate::ui_capture::framebuffer()?
        .export_png(output.join("laboratoire-recharge.png").to_str().unwrap());
    request_new_screen_size(960.0, 540.0);
    for _ in 0..4 {
        next_frame().await;
    }
    state.reset();
    state.skill = 2;
    state.launch(0.0);
    state.animation_age = 0.92;
    state.status = 5;
    draw_review(&mut state, 0.0, false);
    crate::ui_capture::framebuffer()?
        .export_png(output.join("laboratoire-960.png").to_str().unwrap());
    request_new_screen_size(1280.0, 800.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let mut frames = Vec::new();
    let range = if std::env::args().any(|arg| arg == "--melee-only") {
        0..1
    } else {
        0..3
    };
    for skill in range {
        for tile in [64.0] {
            let name = format!("{}-{}", IDS[skill], tile as u16);
            let directory = output.join(&name);
            std::fs::create_dir(&directory).map_err(|e| e.to_string())?;
            for frame in 0..60 {
                clear_background(INK);
                let time = frame as f32 / 30.0;
                draw_scene(
                    skill,
                    Rect::new(24.0, 24.0, tile * 6.0, tile * 5.0),
                    tile,
                    Some(time),
                    if skill == 1 {
                        5
                    } else if skill == 2 {
                        5
                    } else {
                        0
                    },
                );
                crate::ui_capture::framebuffer()?
                    .export_png(directory.join(format!("{frame:03}.png")).to_str().unwrap());
                next_frame().await;
            }
            frames.push(
                serde_json::json!({"id": IDS[skill], "tile": tile as u16, "folder": name,
                "frames": 60, "fps": 30, "crop": [24, 24, tile as u16 * 6, tile as u16 * 5]}),
            );
        }
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&frames).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn animation_replay_does_not_advance_turns_or_restart_status_timers() {
        let mut review = Review {
            skill: 1,
            ..Default::default()
        };
        review.launch(0.0);
        review.next_turn();
        let before = (review.turn, review.cooldown, review.status);
        review.replay(3.0);
        assert_eq!(before, (review.turn, review.cooldown, review.status));
        review.animation_age = 9.0;
        assert_eq!(before, (review.turn, review.cooldown, review.status));
    }
    #[test]
    fn cooldown_blocks_launch_until_turns_pass_unless_lab_override_is_enabled() {
        let mut review = Review {
            free_cooldowns: false,
            ..Default::default()
        };
        review.launch(0.0);
        review.animation_age = 1.0;
        review.launch(1.0);
        assert_eq!(review.animation_age, 1.0);
        for _ in 0..3 {
            review.next_turn();
        }
        review.launch(2.0);
        assert_eq!(review.animation_age, 0.0);
        assert_eq!(review.rank(), 11);
        assert_eq!(parameter(1, "energy", review.rank()), 11);
    }
}
