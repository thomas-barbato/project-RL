//! ANIM-S04: native fire proposals; presentation fixtures, never combat state.
use super::melee::actor;
use super::*;

const TILE: f32 = 64.0;
const FPS: u32 = 30;
const FRAMES: u32 = 120;
const CAPTURE_TURN: f32 = 0.85;
const IDS: [&str; 3] = ["FE02", "FE03", "FE04"];
const TITLES: [&str; 3] = ["Souffle de feu", "Mur de feu", "Émanation soudaine"];
const VARIANTS: [&str; 3] = ["FE02-A01", "FE03-A01", "FE04-A01"];
const ORANGE: Color = Color::new(1.0, 0.31, 0.025, 1.0);
const GOLD: Color = Color::new(1.0, 0.73, 0.16, 1.0);
const HOT: Color = Color::new(1.0, 0.97, 0.72, 1.0);
const CROSS: [(i32, i32); 5] = [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)];

fn cell(rect: Rect, x: f32, y: f32) -> Vec2 {
    vec2(rect.x + x * TILE, rect.y + y * TILE)
}

fn caster() -> Visual {
    game_art::wear(
        game_art::player(Some("core:arcaniste"), PlayerWeaponPose::Unarmed, None),
        [("core:body_armor".into(), "core:tenue_rituelle".into())],
    )
}

fn enemy(at: Vec2) {
    actor(
        at,
        Visual::Bestiary {
            number: 12,
            corpse: false,
        },
    );
}

fn base(rect: Rect) {
    for y in 0..7 {
        for x in 0..7 {
            let at = cell(rect, x as f32, y as f32);
            game_art::draw_world(
                Rect::new(at.x, at.y, TILE, TILE),
                Visual::Terrain {
                    kind: crate::test_sector::Decor::Deck,
                    position: GridPos::new(x, y),
                },
                1.0,
            );
            draw_rectangle_lines(
                at.x,
                at.y,
                TILE,
                TILE,
                0.7,
                Color::new(0.15, 0.18, 0.19, 0.75),
            );
        }
    }
}

fn fire_glow(at: Vec2, radius: f32, color: Color, opacity: f32) {
    let mut mesh = Mesh {
        vertices: vec![Vertex::new(
            at.x,
            at.y,
            0.0,
            0.0,
            0.0,
            alpha(color, opacity * 0.28),
        )],
        indices: Vec::with_capacity(144),
        texture: None,
    };
    for n in 0..=48_u16 {
        let a = n as f32 / 48.0 * std::f32::consts::TAU;
        let p = at + vec2(a.cos(), a.sin()) * radius * 1.8;
        mesh.vertices
            .push(Vertex::new(p.x, p.y, 0.0, 0.0, 0.0, alpha(color, 0.0)));
        if n > 0 {
            mesh.indices.extend_from_slice(&[0, n, n + 1]);
        }
    }
    draw_mesh(&mesh);
}

// A continuous tapered tongue: dark edge, orange mantle, yellow body, pale core.
// Shared vertices avoid seams; the seeded sway has no gameplay RNG dependency.
fn tongue(at: Vec2, direction: Vec2, length: f32, width: f32, time: f32, seed: u32, opacity: f32) {
    let normal = vec2(-direction.y, direction.x);
    let mut mesh = Mesh {
        vertices: Vec::with_capacity(75),
        indices: Vec::with_capacity(336),
        texture: None,
    };
    for i in 0..=14_u16 {
        let k = i as f32 / 14.0;
        let wave = (time * 10.0 + k * 7.0 + noise(seed) * 9.0).sin();
        let center = at + direction * (k * length) + normal * (wave * width * k * 0.23);
        let spread = width * (1.0 - k).powf(0.7) * (0.68 + (k * 6.0 - time * 12.0).sin() * 0.16);
        let life = opacity * (1.0 - k * k);
        for (offset, color) in [
            (-1.0, alpha(Color::new(0.54, 0.06, 0.005, 1.0), life * 0.08)),
            (-0.60, alpha(ORANGE, life * 0.90)),
            (-0.06, alpha(if k < 0.38 { HOT } else { GOLD }, life)),
            (0.45, alpha(ORANGE, life * 0.80)),
            (1.0, alpha(ORANGE, 0.0)),
        ] {
            let p = center + normal * spread * offset;
            mesh.vertices
                .push(Vertex::new(p.x, p.y, 0.0, 0.0, 0.0, color));
        }
        if i > 0 {
            let prev = (i - 1) * 5;
            let curr = i * 5;
            for s in 0..4 {
                mesh.indices.extend_from_slice(&[
                    prev + s,
                    prev + s + 1,
                    curr + s,
                    curr + s,
                    prev + s + 1,
                    curr + s + 1,
                ]);
            }
        }
    }
    draw_mesh(&mesh);
}

fn smoke(at: Vec2, time: f32, seed: u32, spread: f32, opacity: f32) {
    for n in 0..12 {
        let phase = (time * 0.30 + noise(seed + n)).fract();
        let p = at
            + vec2(
                (noise(seed + n + 30) - 0.5) * spread + (phase * 5.0).sin() * 6.0,
                -phase * TILE * 1.05,
            );
        draw_circle(
            p.x,
            p.y,
            5.0 + phase * 16.0,
            alpha(Color::new(0.07, 0.055, 0.045, 1.0), (1.0 - phase) * opacity),
        );
    }
}

fn sparks(at: Vec2, time: f32, seed: u32, spread: f32, opacity: f32) {
    for n in 0..22 {
        let phase = (time * (0.55 + noise(seed + n) * 0.4) + noise(seed + n + 40)).fract();
        let drift = (noise(seed + n + 90) - 0.5) * spread;
        let p = at + vec2(drift * phase, -phase * TILE * (0.65 + noise(seed + n + 70)));
        luminous_line(
            p + vec2(-drift * 0.02, 4.0),
            p,
            1.4,
            GOLD,
            (1.0 - phase) * opacity,
        );
    }
}

fn burning(at: Vec2, time: f32, seed: u32, opacity: f32) {
    for n in 0..4 {
        let p = at + vec2((n as f32 - 1.5) * 10.0, 13.0 + noise(seed + n) * 8.0);
        tongue(
            p,
            vec2(0.0, -1.0),
            23.0 + noise(seed + n + 20) * 13.0,
            7.0,
            time,
            seed + n,
            opacity,
        );
    }
    sparks(at + vec2(0.0, 15.0), time, seed + 100, 42.0, opacity * 0.8);
}

fn name(skill: usize, at: Vec2, time: f32) {
    if time < 1.1 {
        draw_text_bold(
            TITLES[skill],
            at.x - 43.0,
            at.y - 43.0 - time * 7.0,
            15.0,
            alpha(TEXT, (1.0 - time / 1.1).sqrt()),
        );
    }
}

fn in_cone(dx: i32, dy: i32) -> bool {
    dx > 0 && dy.abs() <= dx && dx * dx + dy * dy <= 25
}

fn breath(rect: Rect, time: f32, active: bool) {
    let player = cell(rect, 1.5, 3.5);
    let strength = if active {
        (time / 0.15).clamp(0.0, 1.0)
    } else {
        0.0
    };
    for y in -3..=3 {
        for x in 1..=5 {
            if in_cone(x, y) {
                let at = player + vec2(x as f32, y as f32) * TILE;
                fire_glow(at, TILE * 0.43, ORANGE, strength * 0.7);
            }
        }
    }
    for (x, y) in [(3.5, 3.5), (4.5, 1.5), (4.5, 5.5), (6.5, 3.5)] {
        enemy(cell(rect, x, y));
    }
    actor(player, caster());
    name(0, player, time);
    if !active {
        return;
    }
    let start = player + vec2(TILE * 0.25, -TILE * 0.03);
    // Flames occupy the cone on activation. Apparent turbulence is not a moving damage front.
    let mut mesh = Mesh {
        vertices: Vec::with_capacity(287),
        indices: Vec::with_capacity(1440),
        texture: None,
    };
    for n in 0..=40_u16 {
        let u = n as f32 / 40.0;
        let reach = TILE * 4.55 * u;
        let width = (10.0 + reach * 0.61)
            * (1.0 - u.powi(5))
            * (1.0 + (time * 9.0 - u * 14.0).sin() * 0.07);
        for (side, color) in [
            (-1.0, alpha(ORANGE, 0.0)),
            (-0.70, alpha(ORANGE, strength * 0.48)),
            (-0.28, alpha(GOLD, strength * 0.55)),
            (0.0, alpha(HOT, strength * (1.0 - u) * 0.70)),
            (0.28, alpha(GOLD, strength * 0.55)),
            (0.70, alpha(ORANGE, strength * 0.48)),
            (1.0, alpha(ORANGE, 0.0)),
        ] {
            let p = start + vec2(reach, width * side);
            mesh.vertices
                .push(Vertex::new(p.x, p.y, 0.0, 0.0, 0.0, color));
        }
        if n > 0 {
            let a = (n - 1) * 7;
            let b = n * 7;
            for s in 0..6 {
                mesh.indices.extend_from_slice(&[
                    a + s,
                    a + s + 1,
                    b + s,
                    b + s,
                    a + s + 1,
                    b + s + 1,
                ]);
            }
        }
    }
    draw_mesh(&mesh);
    for n in 0..52 {
        let phase = (time * (0.6 + noise(n + 20) * 0.45) + noise(n + 1)).fract();
        let a = (noise(n + 100) - 0.5) * std::f32::consts::FRAC_PI_2;
        let direction = vec2(a.cos(), a.sin());
        let distance = 18.0 + phase * TILE * 3.95;
        let at = start + direction * distance;
        let life = (1.0 - phase).powf(0.5) * strength;
        tongue(
            at,
            direction,
            18.0 + noise(n + 160) * 25.0,
            9.0 + phase * 19.0,
            time,
            n + 400,
            life * 0.75,
        );
    }
    tongue(start, vec2(1.0, 0.0), TILE * 1.6, 15.0, time, 33, strength);
    fire_glow(start, 18.0, GOLD, strength);
    for (index, (x, y)) in [(3.5, 3.5), (4.5, 1.5), (4.5, 5.5)].into_iter().enumerate() {
        burning(
            cell(rect, x, y),
            time,
            300 + index as u32 * 31,
            strength * 0.85,
        );
        smoke(
            cell(rect, x, y),
            time,
            810 + index as u32 * 19,
            40.0,
            strength * 0.17,
        );
    }
    sparks(cell(rect, 5.0, 3.5), time, 530, TILE * 2.7, strength);
}

fn wall(rect: Rect, time: f32, active: bool) {
    let strength = if active {
        (time / 0.12).clamp(0.0, 1.0)
    } else {
        0.0
    };
    let caster_at = cell(rect, 1.5, 5.5);
    for x in 2..=6 {
        let at = cell(rect, x as f32 + 0.5, 3.5);
        fire_glow(at + vec2(0.0, 17.0), TILE * 0.52, ORANGE, strength);
        smoke(at, time, x * 29, TILE, strength * 0.5);
        for n in 0..6 {
            let base = at + vec2((n as f32 - 2.5) * TILE / 6.0, 18.0);
            let height =
                TILE * (0.85 + noise(n + x * 20) * 0.45) + (time * 8.0 + n as f32).sin() * 8.0;
            tongue(
                base,
                vec2(0.0, -1.0),
                height,
                14.0,
                time,
                n + x * 40,
                strength * 0.82,
            );
        }
    }
    let leaving_y = 3.5 + ((time - 1.5) / 0.55).clamp(0.0, 1.0);
    enemy(cell(rect, 4.5, leaving_y));
    enemy(cell(rect, 6.5, 3.5));
    enemy(cell(rect, 4.5, 1.5));
    actor(caster_at, caster());
    name(1, caster_at, time);
    for x in 2..=6 {
        let at = cell(rect, x as f32 + 0.5, 3.5);
        luminous_line(
            at - vec2(31.0, -22.0),
            at + vec2(31.0, 22.0),
            3.8,
            HOT,
            strength * 0.65,
        );
        for n in 0..5 {
            tongue(
                at + vec2((n as f32 - 2.0) * 12.0, 26.0),
                vec2(0.0, -1.0),
                35.0 + noise(n + x * 13) * 25.0,
                9.0,
                time,
                n + x * 12,
                strength * 0.55,
            );
        }
        sparks(at + vec2(0.0, 10.0), time, x * 30, TILE, strength);
    }
    burning(cell(rect, 4.5, leaving_y), time, 681, strength * 0.9);
    burning(cell(rect, 6.5, 3.5), time, 778, strength * 0.9);
}

fn cracks(at: Vec2, time: f32, seed: u32, opacity: f32) {
    for n in 0..5 {
        let a = noise(seed + n) * std::f32::consts::TAU;
        let direction = vec2(a.cos(), a.sin());
        let normal = vec2(-direction.y, direction.x);
        let mut start = at;
        for step in 0..3 {
            let end = at
                + direction * (step as f32 + 1.0) * 9.0
                + normal * ((noise(seed + n + step * 30) - 0.5) * 12.0);
            luminous_line(
                start,
                end,
                2.3,
                GOLD,
                opacity * (0.7 + 0.3 * (time * 8.0 + n as f32).sin()),
            );
            start = end;
        }
    }
}

fn eruption(rect: Rect, time: f32, blast_age: Option<f32>) {
    let center = cell(rect, 4.5, 3.5);
    let player = cell(rect, 1.5, 3.5);
    let marker_life = blast_age.map_or(1.0, |age| (1.0 - age / 0.23).max(0.0));
    for (index, &(x, y)) in CROSS.iter().enumerate() {
        let at = center + vec2(x as f32, y as f32) * TILE;
        draw_rectangle(
            at.x - 31.0,
            at.y - 31.0,
            62.0,
            62.0,
            alpha(ORANGE, marker_life * 0.09),
        );
        cracks(at + vec2(0.0, 20.0), time, index as u32 * 27, marker_life);
        fire_glow(at + vec2(0.0, 20.0), 22.0, ORANGE, marker_life * 0.6);
    }
    for &(x, y) in &CROSS {
        enemy(center + vec2(x as f32, y as f32) * TILE);
    }
    actor(player, caster());
    name(2, player, time);
    if let Some(age) = blast_age {
        let life = (1.0 - age / 0.95).clamp(0.0, 1.0);
        let rise = (age / 0.12).clamp(0.0, 1.0);
        fire_glow(center, 80.0, HOT, life * rise);
        if age < 0.55 {
            draw_circle_lines(
                center.x,
                center.y + 13.0,
                age * 175.0,
                5.0 * (1.0 - age / 0.55),
                alpha(GOLD, life),
            );
        }
        for (index, &(x, y)) in CROSS.iter().enumerate() {
            let at = center + vec2(x as f32, y as f32) * TILE;
            smoke(at, time, 910 + index as u32 * 17, TILE, life * 0.5);
            for n in 0..7 {
                let height = (70.0 + noise(n + index as u32 * 40) * 55.0) * rise;
                tongue(
                    at + vec2((n as f32 - 3.0) * 9.0, 20.0),
                    vec2(0.0, -1.0),
                    height,
                    14.0,
                    time,
                    n + index as u32 * 21,
                    life,
                );
            }
            for n in 0..10 {
                let a = noise(n + index as u32 * 110) * std::f32::consts::TAU;
                let dir = vec2(a.cos(), a.sin());
                let p = at + dir * (age * 50.0);
                luminous_line(p - dir * 10.0, p, 2.0, GOLD, life * 0.8);
            }
            if (0.22..2.3).contains(&age) {
                burning(
                    at,
                    time,
                    2000 + index as u32 * 37,
                    ((2.3 - age) / 0.5).min(1.0),
                );
            }
        }
    }
}

fn scene(skill: usize, rect: Rect, age: f32, active: bool, blast_age: Option<f32>) {
    base(rect);
    match skill {
        0 => breath(rect, age, active),
        1 => wall(rect, age, active),
        _ => eruption(rect, age, blast_age),
    }
    let label = match skill {
        0 => "Cône · 5 cases · 3 tours",
        1 => "Rideau traversable · brûlure après la sortie",
        _ if blast_age.is_none() => "Préparation · 1 tour",
        _ => "+1 tour · explosion",
    };
    draw_rectangle(
        rect.x + 7.0,
        rect.y + rect.h - 28.0,
        rect.w - 14.0,
        22.0,
        alpha(INK, 0.88),
    );
    draw_text(label, rect.x + 14.0, rect.y + rect.h - 12.0, 14.0, TEXT);
}

pub(super) async fn run() {
    request_new_screen_size(1100.0, 720.0);
    let mut selected = 0;
    let mut age = 0.0;
    let mut burst_start = None;
    let mut remaining = 3_u16;
    let mut cooldown = 0_u16;
    let mut turn = 0_u32;
    let mut paused = false;
    let mut speed = 1.0;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold("LABORATOIRE / ANIMATIONS · LOT 04", 22.0, 35.0, 23.0, TEXT);
        draw_text(
            "LAB-S04 · aperçu visuel · dégâts réels à raccorder",
            22.0,
            62.0,
            14.0,
            DIM,
        );
        for (i, title) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 96.0 + i as f32 * 44.0, 225.0, 35.0),
                &format!("{:02} · {title}", i + 10),
                true,
            ) {
                selected = i;
                age = 0.0;
                burst_start = None;
                remaining = match i {
                    0 => 3,
                    1 => 5,
                    _ => 1,
                };
                cooldown = 0;
                turn = 0;
            }
        }
        scene(
            selected,
            Rect::new(335.0, 95.0, 448.0, 448.0),
            age,
            remaining > 0,
            burst_start.map(|start| age - start),
        );
        draw_text(VARIANTS[selected], 335.0, 567.0, 14.0, DIM);
        draw_text(
            format!("Tour {turn} · recharge : {cooldown} tours · rang effectif 11"),
            22.0,
            604.0,
            15.0,
            TEXT,
        );
        if button(
            Rect::new(20.0, 625.0, 125.0, 34.0),
            if cooldown > 0 {
                "En recharge"
            } else {
                "Lancer"
            },
            cooldown == 0,
        ) || (is_key_pressed(KeyCode::Enter) && cooldown == 0)
        {
            age = 0.0;
            burst_start = None;
            paused = false;
            remaining = parameter_for(
                IDS[selected],
                if selected == 0 {
                    "maximum_duration_turns"
                } else if selected == 1 {
                    "duration_turns"
                } else {
                    "preparation_delay_complete_turns"
                },
                11,
            );
            cooldown = parameter_for(IDS[selected], "cooldown_turns", 11);
        }
        if button(Rect::new(154.0, 625.0, 118.0, 34.0), "+1 tour", true)
            || is_key_pressed(KeyCode::Space)
        {
            turn += 1;
            cooldown = cooldown.saturating_sub(1);
            remaining = remaining.saturating_sub(1);
            if selected == 2 && remaining == 0 && burst_start.is_none() {
                burst_start = Some(age);
            }
        }
        if button(Rect::new(281.0, 625.0, 118.0, 34.0), "Rejouer", true)
            || is_key_pressed(KeyCode::R)
        {
            age = 0.0;
            if burst_start.is_some() {
                burst_start = Some(0.0);
            }
        }
        if button(
            Rect::new(408.0, 625.0, 118.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if button(
            Rect::new(535.0, 625.0, 118.0, 34.0),
            if speed == 1.0 {
                "Vitesse ×1"
            } else {
                "Vitesse ×0,5"
            },
            true,
        ) {
            speed = if speed == 1.0 { 0.5 } else { 1.0 };
        }
        draw_text(
            "Émanation : +1 tour déclenche l'explosion · la lecture ne fait pas avancer les tours",
            22.0,
            694.0,
            14.0,
            DIM,
        );
        if !paused {
            age += get_frame_time() * speed;
        }
        next_frame().await;
    }
}

pub(super) async fn capture(output: &Path) -> Result<(), String> {
    if output.exists() {
        return Err("Le dossier de revue existe déjà.".into());
    }
    std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
    request_new_screen_size(512.0, 512.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let mut entries = Vec::new();
    for skill in 0..3 {
        let directory = output.join(VARIANTS[skill]);
        std::fs::create_dir(&directory).map_err(|e| e.to_string())?;
        for frame in 0..FRAMES {
            clear_background(INK);
            let age = frame as f32 / FPS as f32;
            // A scripted +1-turn event illustrates FE04's delay. Seconds never define it.
            let blast_age = if skill == 2 && age >= CAPTURE_TURN {
                Some(age - CAPTURE_TURN)
            } else {
                None
            };
            scene(
                skill,
                Rect::new(24.0, 24.0, 448.0, 448.0),
                age,
                true,
                blast_age,
            );
            crate::ui_capture::framebuffer()?
                .export_png(directory.join(format!("{frame:03}.png")).to_str().unwrap());
            next_frame().await;
        }
        entries.push(serde_json::json!({"id":IDS[skill],"variant":VARIANTS[skill],"number":skill+10,"title":TITLES[skill],
            "tile":64,"folder":VARIANTS[skill],"frames":FRAMES,"fps":FPS,"framebuffer_size":[512,512],"crop":[24,24,448,448]}));
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
