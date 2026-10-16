//! ANIM-S03: Charge and Tourbillon visual fixtures, without combat state.
use super::melee::{actor, blood_spray, cut, ground_marks, impact, player_visual};
use super::*;

const TILE: f32 = 64.0;
const FPS: u32 = 30;
const FRAMES: u32 = 90;
const CHARGE_HIT: f32 = 0.60;
const IDS: [&str; 2] = ["ME05", "ME06"];
const TITLES: [&str; 2] = ["Charge", "Tourbillon"];
const VARIANTS: [&str; 2] = ["ME05-A01", "ME06-A01"];
const NEIGHBORS: [(i32, i32); 8] = [
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
];

fn charge_column(time: f32) -> f32 {
    let t = ((time - 0.06) / 0.51).clamp(0.0, 1.0);
    1.5 + 3.0 * t * t * (3.0 - 2.0 * t)
}

fn base(rect: Rect) {
    for y in 0..5 {
        for x in 0..7 {
            game_art::draw_world(
                Rect::new(
                    rect.x + x as f32 * TILE,
                    rect.y + y as f32 * TILE,
                    TILE,
                    TILE,
                ),
                Visual::Terrain {
                    kind: crate::test_sector::Decor::Deck,
                    position: GridPos::new(x, y),
                },
                1.0,
            );
            draw_rectangle_lines(
                rect.x + x as f32 * TILE,
                rect.y + y as f32 * TILE,
                TILE,
                TILE,
                0.7,
                Color::new(0.15, 0.18, 0.19, 0.75),
            );
        }
    }
}

fn name(skill: usize, at: Vec2, time: f32) {
    if time < 1.05 {
        draw_text_bold(
            TITLES[skill],
            at.x - TILE * 0.6,
            at.y - TILE * 0.69 - time * 9.0,
            15.0,
            alpha(TEXT, (1.0 - time / 1.05).sqrt()),
        );
    }
}

fn charge(rect: Rect, time: f32) {
    let player = vec2(rect.x + charge_column(time) * TILE, rect.y + 2.5 * TILE);
    let target = vec2(rect.x + 5.5 * TILE, rect.y + 2.5 * TILE);
    let steel = Color::new(0.71, 0.88, 0.94, 1.0);
    ground_marks(target, time - CHARGE_HIT + 0.22, 710);
    if (0.07..CHARGE_HIT).contains(&time) {
        for n in 1..=3 {
            let previous = charge_column(time - n as f32 * 0.055);
            let at = vec2(rect.x + previous * TILE, player.y);
            game_art::draw_world(
                Rect::new(at.x - TILE * 0.5, at.y - TILE * 0.5, TILE, TILE),
                player_visual(),
                0.11 * (4 - n) as f32,
            );
        }
        for n in 0..15 {
            let lag = noise(n + 90) * 0.12;
            let at = vec2(
                rect.x + charge_column(time - lag) * TILE - TILE * 0.2,
                player.y + TILE * (0.31 + noise(n + 70) * 0.10),
            );
            draw_ellipse(
                at.x,
                at.y,
                8.0 + noise(n + 130) * 12.0,
                2.0,
                0.0,
                alpha(DIM, 0.16),
            );
        }
        luminous_line(
            player - vec2(TILE * 0.85, -TILE * 0.30),
            player + vec2(TILE * 0.27, TILE * 0.30),
            2.5,
            steel,
            0.75,
        );
    }
    let bump = if (CHARGE_HIT..CHARGE_HIT + 0.26).contains(&time) {
        ((time - CHARGE_HIT) / 0.26 * std::f32::consts::PI).sin() * TILE * 0.07
    } else {
        0.0
    };
    actor(
        target + vec2(bump, 0.0),
        Visual::Bestiary {
            number: 12,
            corpse: false,
        },
    );
    actor(player, player_visual());
    name(0, player, time);
    if (0.08..0.88).contains(&time) {
        let life = (1.0 - (time - CHARGE_HIT).max(0.0) / 0.28).clamp(0.0, 1.0);
        let front = player + vec2(TILE * 0.28, -TILE * 0.10);
        let end = front + vec2(TILE * 0.76, 0.03 * TILE);
        cut(front, end, TILE * 0.16, life);
        glow(front, TILE * 0.15, steel, life * 0.75);
    }
    impact(target, time - CHARGE_HIT, 1.2, false);
    blood_spray(target - vec2(0.0, TILE * 0.08), time - CHARGE_HIT, 600, 1.0);
}

fn whirl(rect: Rect, time: f32, status: u16) {
    let center = vec2(rect.x + 3.5 * TILE, rect.y + 2.5 * TILE);
    for (index, &(x, y)) in NEIGHBORS.iter().enumerate() {
        let at = center + vec2(x as f32, y as f32) * TILE;
        let hit = 0.12 + index as f32 * 0.0975;
        ground_marks(at, time - hit + 0.22, 1000 + index as u32 * 29);
    }
    for (index, &(x, y)) in NEIGHBORS.iter().enumerate() {
        let at = center + vec2(x as f32, y as f32) * TILE;
        let age = time - (0.12 + index as f32 * 0.0975);
        let direction = vec2(x as f32, y as f32).normalize();
        let bump = if (0.0..0.22).contains(&age) {
            (age / 0.22 * std::f32::consts::PI).sin() * TILE * 0.045
        } else {
            0.0
        };
        actor(
            at + direction * bump,
            Visual::Bestiary {
                number: 12,
                corpse: false,
            },
        );
    }
    actor(center, player_visual());
    name(1, center, time);
    if (0.04..1.12).contains(&time) {
        let phase = ((time - 0.04) / 0.78).clamp(0.0, 1.0);
        let end = -std::f32::consts::FRAC_PI_2 + phase * std::f32::consts::TAU;
        let start = (end - 2.0).max(-std::f32::consts::FRAC_PI_2);
        let life = (1.0 - (time - 0.82).max(0.0) / 0.30).clamp(0.0, 1.0);
        crescent(center, TILE, start, end, life);
        let tip = center + vec2(end.cos(), end.sin()) * TILE * 1.52;
        glow(tip, TILE * 0.16, Color::new(0.60, 0.84, 1.0, 1.0), life);
        for n in 0..20 {
            let a = end - noise(n + 160) * 1.8;
            let r = TILE * (1.47 + noise(n + 50) * 0.13);
            let at = center + vec2(a.cos(), a.sin()) * r;
            draw_circle(
                at.x,
                at.y,
                1.2 + noise(n + 73) * 1.3,
                alpha(Color::new(0.92, 0.96, 0.92, 1.0), life * 0.75),
            );
        }
    }
    for (index, &(x, y)) in NEIGHBORS.iter().enumerate() {
        let at = center + vec2(x as f32, y as f32) * TILE;
        let age = time - (0.12 + index as f32 * 0.0975);
        impact(at, age, 0.50, false);
        blood_spray(at, age, 900 + index as u32 * 43, 0.52);
        if status > 0 && age > 0.95 {
            for n in 0..3 {
                let phase = (age * 0.72 + noise(n + index as u32 * 29)).fract();
                let point = at
                    + vec2(
                        (noise(n + index as u32 * 43 + 22) - 0.5) * TILE * 0.22,
                        phase * TILE * 0.42,
                    );
                draw_ellipse(
                    point.x,
                    point.y,
                    1.4,
                    2.3,
                    0.0,
                    alpha(Color::new(0.63, 0.02, 0.04, 1.0), 1.0 - phase),
                );
            }
        }
    }
    if time > 0.85 && status > 0 {
        draw_text(
            format!("Saignement · {status} tours"),
            rect.x + 16.0,
            rect.y + TILE * 4.74,
            13.0,
            Color::new(0.88, 0.33, 0.32, 1.0),
        );
    }
}

fn scene(skill: usize, rect: Rect, time: f32, status: u16) {
    base(rect);
    match skill {
        0 => charge(rect, time),
        1 => whirl(rect, time, status),
        _ => unreachable!(),
    }
}

pub(super) async fn run() {
    request_new_screen_size(960.0, 650.0);
    let mut selected = 0;
    let mut age = 0.0;
    let mut speed = 1.0;
    let mut paused = false;
    let mut turn = 0_u32;
    let mut status = 0_u16;
    let mut cooldown = 0_u16;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold("LABORATOIRE / ANIMATIONS · LOT 03", 22.0, 35.0, 23.0, TEXT);
        draw_text(
            "LAB-S03 · aperçu visuel · dégâts réels à raccorder",
            22.0,
            62.0,
            14.0,
            DIM,
        );
        for (i, title) in TITLES.iter().enumerate() {
            if button(
                Rect::new(20.0, 96.0 + i as f32 * 44.0, 220.0, 35.0),
                &format!("{:02} · {title}", i + 8),
                true,
            ) {
                selected = i;
                age = 0.0;
                turn = 0;
                status = 0;
                cooldown = 0;
            }
        }
        scene(selected, Rect::new(360.0, 100.0, 448.0, 320.0), age, status);
        draw_text(VARIANTS[selected], 360.0, 455.0, 14.0, DIM);
        draw_text(
            format!("Tour {turn} · recharge : {cooldown} tours · rang effectif 11"),
            22.0,
            515.0,
            15.0,
            TEXT,
        );
        if button(
            Rect::new(20.0, 546.0, 125.0, 34.0),
            if cooldown > 0 {
                "En recharge"
            } else {
                "Lancer"
            },
            cooldown == 0,
        ) || (is_key_pressed(KeyCode::Enter) && cooldown == 0)
        {
            age = 0.0;
            paused = false;
            cooldown = parameter_for(IDS[selected], "cooldown_turns", 11);
            status = if selected == 1 {
                parameter_for("ME06", "bleed_duration_turns", 11)
            } else {
                0
            };
        }
        if button(Rect::new(154.0, 546.0, 118.0, 34.0), "+1 tour", true)
            || is_key_pressed(KeyCode::Space)
        {
            turn += 1;
            cooldown = cooldown.saturating_sub(1);
            status = status.saturating_sub(1);
        }
        if button(Rect::new(281.0, 546.0, 118.0, 34.0), "Rejouer", true)
            || is_key_pressed(KeyCode::R)
        {
            age = 0.0;
        }
        if button(
            Rect::new(408.0, 546.0, 118.0, 34.0),
            if paused { "Lecture" } else { "Pause" },
            true,
        ) {
            paused = !paused;
        }
        if button(
            Rect::new(535.0, 546.0, 118.0, 34.0),
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
            "64 px natifs · Entrée : lancer · Espace : +1 tour · R : rejouer · Échap : fermer",
            22.0,
            618.0,
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
    // Capture only the native scene and margin, avoiding an unused full UI framebuffer.
    request_new_screen_size(512.0, 384.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let mut entries = Vec::new();
    for skill in 0..2 {
        let directory = output.join(VARIANTS[skill]);
        std::fs::create_dir(&directory).map_err(|e| e.to_string())?;
        for frame in 0..FRAMES {
            clear_background(INK);
            scene(
                skill,
                Rect::new(24.0, 24.0, 448.0, 320.0),
                frame as f32 / FPS as f32,
                if skill == 1 { 3 } else { 0 },
            );
            crate::ui_capture::framebuffer()?
                .export_png(directory.join(format!("{frame:03}.png")).to_str().unwrap());
            next_frame().await;
        }
        entries.push(serde_json::json!({"id":IDS[skill],"variant":VARIANTS[skill],"number":skill+8,"title":TITLES[skill],
            "tile":64,"folder":VARIANTS[skill],"frames":FRAMES,"fps":FPS,"framebuffer_size":[512,384],"crop":[24,24,448,320]}));
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn charge_ends_adjacent_to_the_target_without_crossing_its_cell() {
        assert_eq!(charge_column(0.0), 1.5);
        assert_eq!(charge_column(CHARGE_HIT), 4.5);
        for frame in 0..FRAMES {
            assert!((1.5..=4.5).contains(&charge_column(frame as f32 / FPS as f32)));
        }
    }
    #[test]
    fn whirl_shows_each_adjacent_cell_once() {
        let cells: std::collections::BTreeSet<_> = NEIGHBORS.into_iter().collect();
        assert_eq!(cells.len(), 8);
        assert!(!cells.contains(&(0, 0)));
        assert!(cells.iter().all(|&(x, y)| x.abs() <= 1 && y.abs() <= 1));
    }
}
