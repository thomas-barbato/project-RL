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
    static DATA: std::sync::OnceLock<Catalogue> = std::sync::OnceLock::new();
    let data = DATA.get_or_init(|| {
        serde_json::from_str(include_str!("../assets/skills/agent/catalogue.json")).unwrap()
    });
    let p = &data
        .skills
        .iter()
        .find(|s| s.id == IDS[skill])
        .unwrap()
        .parameters[key];
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
fn draw_scene(skill: usize, rect: Rect, tile: f32, age: Option<f32>, status: u16) {
    let p = |x: f32, y: f32| vec2(rect.x + x * tile, rect.y + y * tile);
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
    let player = p(1.5, 2.5);
    let target = p(4.5, 2.5);
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
    let Some(t) = age else {
        return;
    };
    if t < 0.75 {
        draw_text_bold(
            TITLES[skill],
            player.x - tile * 0.65,
            player.y - tile * 0.67 - t * 9.0,
            if tile == 64.0 { 14.0 } else { 11.0 },
            alpha(TEXT, 1.0 - t / 0.75),
        );
    }
    match skill {
        0 => {
            let phase = (t / 0.34).clamp(0.0, 1.0);
            if t < 0.56 {
                // A single directed sweep; no full circle or persistent wind loop.
                let end = -1.15 + phase * 2.3;
                let start = (-1.15_f32).max(end - 0.70);
                let radius = tile * 1.30;
                for i in 0..24 {
                    let a = start + (end - start) * i as f32 / 24.0;
                    let b = start + (end - start) * (i + 1) as f32 / 24.0;
                    let from = player + vec2(a.cos(), a.sin()) * radius;
                    let to = player + vec2(b.cos(), b.sin()) * radius;
                    let opacity = (i as f32 / 24.0) * (1.0 - (t - 0.34).max(0.0) / 0.22);
                    draw_line(
                        from.x,
                        from.y,
                        to.x,
                        to.y,
                        tile * 0.045,
                        alpha(Color::new(0.68, 0.76, 0.77, 1.0), opacity),
                    );
                }
                let tip = player + vec2(end.cos(), end.sin()) * radius;
                draw_line(
                    tip.x,
                    tip.y,
                    tip.x + end.cos() * tile * 0.15,
                    tip.y + end.sin() * tile * 0.15,
                    tile * 0.025,
                    alpha(TEXT, 0.85),
                );
            }
            for (i, y) in [1.5, 2.5, 3.5].iter().enumerate() {
                let dt = t - (0.10 + i as f32 * 0.09);
                if (0.0..0.22).contains(&dt) {
                    let at = p(2.5, *y);
                    for n in 0..4 {
                        let a = n as f32 * 1.4 + 0.2;
                        let dir = vec2(a.cos(), a.sin());
                        let from = at + dir * tile * dt;
                        let to = from + dir * tile * 0.10;
                        draw_line(
                            from.x,
                            from.y,
                            to.x,
                            to.y,
                            tile * 0.023,
                            alpha(Color::new(0.76, 0.61, 0.40, 1.0), 1.0 - dt / 0.22),
                        );
                    }
                }
            }
        }
        1 | 2 => {
            let flight = 0.38;
            let start = player + vec2(tile * 0.23, -tile * 0.08);
            let hit = target + vec2(0.0, -tile * 0.07);
            let dir = (hit - start).normalize();
            let normal = vec2(-dir.y, dir.x);
            let hot = if skill == 1 {
                Color::new(0.97, 0.40, 0.11, 1.0)
            } else {
                Color::new(0.60, 0.85, 0.90, 1.0)
            };
            if t < flight {
                let at = start.lerp(hit, (t / flight).clamp(0.0, 1.0));
                for i in (0..6).rev() {
                    let tail = at - dir * tile * 0.065 * i as f32;
                    let color = if skill == 1 {
                        Color::new(0.32, 0.09, 0.05, 0.7)
                    } else {
                        Color::new(0.17, 0.35, 0.42, 0.55)
                    };
                    draw_circle(tail.x, tail.y, tile * (0.038 - i as f32 * 0.004), color);
                }
                if skill == 1 {
                    draw_triangle(
                        at + dir * tile * 0.11,
                        at - dir * tile * 0.08 + normal * tile * 0.055,
                        at - dir * tile * 0.08 - normal * tile * 0.055,
                        hot,
                    );
                    draw_circle(at.x, at.y, tile * 0.026, Color::new(1.0, 0.79, 0.39, 1.0));
                } else {
                    draw_triangle(
                        at + dir * tile * 0.16,
                        at - dir * tile * 0.09 + normal * tile * 0.032,
                        at - dir * tile * 0.09 - normal * tile * 0.032,
                        hot,
                    );
                    draw_line(
                        at.x - dir.x * tile * 0.08,
                        at.y - dir.y * tile * 0.08,
                        at.x + dir.x * tile * 0.12,
                        at.y + dir.y * tile * 0.12,
                        tile * 0.018,
                        Color::new(0.88, 0.96, 0.97, 1.0),
                    );
                }
            } else {
                let dt = t - flight;
                if dt < 0.30 {
                    for n in 0..8 {
                        let angle = n as f32 * std::f32::consts::TAU / 8.0;
                        let v = vec2(angle.cos(), angle.sin());
                        let at = hit + v * tile * (0.07 + dt * 0.50);
                        draw_line(
                            at.x,
                            at.y,
                            at.x + v.x * tile * 0.055,
                            at.y + v.y * tile * 0.055,
                            tile * 0.025,
                            alpha(hot, 1.0 - dt / 0.30),
                        );
                    }
                }
                if status > 0 {
                    if skill == 1 {
                        let at = hit + vec2(0.0, tile * 0.08);
                        draw_circle(at.x, at.y, tile * 0.065, Color::new(0.37, 0.11, 0.04, 0.70));
                        draw_circle(
                            at.x,
                            at.y,
                            tile * 0.026,
                            alpha(hot, 0.75 + 0.20 * (t * 8.0).sin()),
                        );
                        for n in 0..3 {
                            let phase = (t * 0.80 + n as f32 * 0.31).fract();
                            let x = at.x + (n as f32 - 1.0) * tile * 0.08;
                            let y = at.y - phase * tile * 0.25;
                            draw_circle(
                                x,
                                y,
                                tile * 0.015 * (1.0 - phase),
                                alpha(hot, 1.0 - phase),
                            );
                        }
                    } else {
                        let at = target + vec2(0.0, tile * 0.31);
                        draw_ellipse_lines(
                            at.x,
                            at.y,
                            tile * 0.25,
                            tile * 0.08,
                            0.0,
                            tile * 0.018,
                            hot,
                        );
                        for n in 0..5 {
                            let x = at.x + (n as f32 - 2.0) * tile * 0.09;
                            let h = tile * (0.13 + (n % 2) as f32 * 0.05);
                            draw_triangle(
                                vec2(x, at.y - h),
                                vec2(x - tile * 0.04, at.y + tile * 0.025),
                                vec2(x + tile * 0.04, at.y + tile * 0.025),
                                alpha(hot, 0.72),
                            );
                        }
                    }
                    let label = format!(
                        "{} · {}",
                        if skill == 1 { "Brûlure" } else { "Gel" },
                        status
                    );
                    draw_text(
                        label,
                        target.x - tile * 0.43,
                        target.y - tile * 0.57,
                        if tile == 64.0 { 13.0 } else { 10.0 },
                        hot,
                    );
                }
            }
        }
        _ => unreachable!(),
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
        let enabled = i != 0 || state.free_cooldowns || state.cooldown == 0;
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
                5 => state.tile = if state.tile == 64.0 { 32.0 } else { 64.0 },
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
    state.skill = 2;
    state.launch(0.0);
    state.animation_age = 0.46;
    state.status = 5;
    draw_review(&mut state, 0.0, false);
    crate::ui_capture::framebuffer()?
        .export_png(output.join("laboratoire-960.png").to_str().unwrap());
    request_new_screen_size(1280.0, 800.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let mut frames = Vec::new();
    for skill in 0..3 {
        for tile in [64.0, 32.0] {
            let name = format!("{}-{}", IDS[skill], tile as u16);
            let directory = output.join(&name);
            std::fs::create_dir(&directory).map_err(|e| e.to_string())?;
            for frame in 0..32 {
                clear_background(INK);
                let time = frame as f32 / 20.0;
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
                "frames": 32, "fps": 20, "crop": [24, 24, tile as u16 * 6, tile as u16 * 5]}),
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
