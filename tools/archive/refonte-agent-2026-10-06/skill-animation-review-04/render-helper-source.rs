//! ANIM-S02: independent visual fixtures, never a combat or movement resolver.
use super::*;

const TILE: f32 = 64.0;
const FRAMES: u32 = 90;
const FPS: u32 = 30;
const HIT: f32 = 0.22;
const COLLISION: f32 = 0.76;
const STEEL: Color = Color::new(0.73, 0.86, 0.90, 1.0);
const WHITE: Color = Color::new(0.98, 0.98, 0.88, 1.0);
const BLOOD: Color = Color::new(0.58, 0.025, 0.045, 1.0);

#[derive(Clone, Copy, PartialEq, Eq)]
enum Action {
    Rupture,
    Pierce,
    PushWall,
    PushEnemy,
}
impl Action {
    fn id(self) -> &'static str {
        match self {
            Self::Rupture => "B01",
            Self::Pierce => "ME03",
            _ => "ME04",
        }
    }
    fn title(self) -> &'static str {
        match self {
            Self::Rupture => "Frappe de rupture",
            Self::Pierce => "Percée",
            _ => "Repousser",
        }
    }
    fn variant(self) -> &'static str {
        match self {
            Self::Rupture => "B01-A01",
            Self::Pierce => "ME03-A01",
            Self::PushWall => "ME04-A01-MUR",
            Self::PushEnemy => "ME04-A01-CIBLE",
        }
    }
    fn label(self) -> &'static str {
        match self {
            Self::Rupture => "04 · Frappe de rupture",
            Self::Pierce => "05 · Percée",
            Self::PushWall => "06 · Repousser / mur",
            Self::PushEnemy => "07 · Repousser / monstre",
        }
    }
    fn pushes(self) -> bool {
        matches!(self, Self::PushWall | Self::PushEnemy)
    }
}
const ACTIONS: [Action; 4] = [
    Action::Rupture,
    Action::Pierce,
    Action::PushWall,
    Action::PushEnemy,
];

fn progress(time: f32, start: f32, duration: f32) -> f32 {
    let t = ((time - start) / duration).clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

// The example stops on the last free cell before the wall/second creature.
fn target_column(action: Action, time: f32) -> f32 {
    2.5 + if action.pushes() {
        2.0 * progress(time, HIT + 0.06, COLLISION - HIT - 0.06)
    } else {
        0.0
    }
}

pub(super) fn actor(at: Vec2, visual: Visual) {
    game_art::draw_world(
        Rect::new(at.x - TILE * 0.5, at.y - TILE * 0.5, TILE, TILE),
        visual,
        1.0,
    );
}

pub(super) fn player_visual() -> Visual {
    game_art::wear(
        game_art::player(
            Some("core:bastion"),
            PlayerWeaponPose::Melee,
            Some("core:melee_epee_longue"),
        ),
        [("core:body_armor".into(), "core:tenue_assaut".into())],
    )
}

// Tapered steel stroke, with a continuous mesh instead of overlapping segments.
pub(super) fn cut(from: Vec2, to: Vec2, width: f32, life: f32) {
    let d = to - from;
    if d.length_squared() < 0.001 {
        return;
    }
    let normal = vec2(-d.y, d.x).normalize();
    let mut mesh = Mesh {
        vertices: Vec::with_capacity(100),
        indices: Vec::with_capacity(432),
        texture: None,
    };
    for i in 0..=24_u16 {
        let k = i as f32 / 24.0;
        let center = from + d * k;
        let spread = width * (k * std::f32::consts::PI).sin().sqrt();
        for (offset, color) in [
            (-spread, alpha(STEEL, 0.0)),
            (-spread * 0.22, alpha(STEEL, life * 0.65)),
            (spread * 0.08, alpha(WHITE, life)),
            (spread * 0.40, alpha(STEEL, 0.0)),
        ] {
            let point = center + normal * offset;
            mesh.vertices
                .push(Vertex::new(point.x, point.y, 0.0, 0.0, 0.0, color));
        }
        if i > 0 {
            let p = (i - 1) * 4;
            let c = i * 4;
            for s in 0..3 {
                mesh.indices.extend_from_slice(&[
                    p + s,
                    p + s + 1,
                    c + s,
                    c + s,
                    p + s + 1,
                    c + s + 1,
                ]);
            }
        }
    }
    draw_mesh(&mesh);
}

pub(super) fn impact(at: Vec2, age: f32, weight: f32, plates: bool) {
    if !(0.0..0.62).contains(&age) {
        return;
    }
    let life = 1.0 - age / 0.62;
    glow(at, TILE * 0.22 * weight, WHITE, life);
    if age < 0.21 {
        let r = TILE * (0.10 + age * 1.1) * weight;
        draw_circle_lines(at.x, at.y, r, TILE * 0.035 * life, alpha(STEEL, life * 0.7));
        for a in [0.1_f32, 1.45, 2.8, 4.4] {
            let direction = vec2(a.cos(), a.sin());
            luminous_line(
                at + direction * 5.0,
                at + direction * r * 1.7,
                2.1,
                WHITE,
                life,
            );
        }
    }
    for n in 0..24 {
        let angle = noise(n + 32) * std::f32::consts::TAU;
        let direction = vec2(angle.cos(), angle.sin());
        let velocity = TILE * (0.6 + noise(n + 110) * 1.8) * weight;
        let point = at + direction * age * velocity + vec2(0.0, age * age * TILE * 0.65);
        if plates && n % 3 == 0 {
            shard(
                point,
                direction,
                TILE * 0.15 * life,
                TILE * 0.043 * life,
                STEEL,
                life,
            );
        } else {
            luminous_line(point - direction * TILE * 0.08, point, 1.4, WHITE, life);
        }
    }
}

pub(super) fn blood_spray(at: Vec2, age: f32, seed: u32, strength: f32) {
    if !(0.0..1.6).contains(&age) {
        return;
    }
    for n in 0..22 {
        let angle = -1.55 + noise(n + seed) * 2.65;
        let direction = vec2(angle.cos(), angle.sin());
        let speed = TILE * (0.34 + noise(n + seed + 41) * 1.05) * strength;
        let falling = age.min(0.56 + noise(n + seed + 14) * 0.18);
        let point = at + direction * speed * falling + vec2(0.0, falling * falling * TILE * 1.85);
        let life = (1.0 - age / 1.6).clamp(0.0, 1.0);
        let radius = (1.1 + noise(n + seed + 80) * 2.8) * strength;
        if age < 0.45 {
            draw_line(
                point.x - direction.x * 3.0,
                point.y - direction.y * 3.0,
                point.x,
                point.y,
                radius,
                alpha(BLOOD, life),
            );
        }
        draw_circle(point.x, point.y, radius, alpha(BLOOD, life));
        draw_circle(
            point.x - 0.3,
            point.y - 0.5,
            radius * 0.32,
            alpha(Color::new(0.92, 0.11, 0.08, 1.0), life * 0.7),
        );
    }
}

pub(super) fn ground_marks(at: Vec2, time: f32, seed: u32) {
    if time <= HIT + 0.40 {
        return;
    }
    let opacity = ((time - HIT - 0.40) / 0.35).clamp(0.0, 0.70);
    for n in 0..11 {
        let p = at
            + vec2(
                (noise(n + seed) - 0.2) * TILE * 0.76,
                TILE * (0.26 + noise(n + seed + 55) * 0.20),
            );
        draw_ellipse(
            p.x,
            p.y,
            1.8 + noise(n + seed + 7) * 3.0,
            1.0 + noise(n + seed + 21) * 1.7,
            0.0,
            alpha(BLOOD, opacity),
        );
    }
}

fn scene(action: Action, rect: Rect, time: f32) {
    let origin = vec2(rect.x, rect.y);
    let player = origin + vec2(1.5, 2.5) * TILE;
    let initial = origin + vec2(2.5, 2.5) * TILE;
    let obstacle = origin + vec2(5.5, 2.5) * TILE;
    let target = origin + vec2(target_column(action, time), 2.5) * TILE;
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
                    kind: if action == Action::PushWall && x == 5 && (1..=3).contains(&y) {
                        crate::test_sector::Decor::Wall
                    } else {
                        crate::test_sector::Decor::Deck
                    },
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
    ground_marks(initial, time, 210);
    if action.pushes() {
        ground_marks(target, time - COLLISION + HIT, 320);
        if action == Action::PushEnemy {
            ground_marks(obstacle, time - COLLISION + HIT, 430);
        }
        let phase = ((time - HIT) / 0.54).clamp(0.0, 1.0);
        if (HIT..COLLISION).contains(&time) {
            for n in 0..12 {
                let p = initial
                    + vec2(
                        phase * TILE * 2.0 - noise(n + 49) * TILE * 0.6,
                        TILE * (0.31 + noise(n + 81) * 0.07),
                    );
                draw_circle(
                    p.x,
                    p.y,
                    3.0 + noise(n + 40) * 3.0,
                    alpha(DIM, (1.0 - phase) * 0.35),
                );
            }
        }
    }
    if action == Action::PushEnemy {
        let recoil = if (COLLISION..COLLISION + 0.3).contains(&time) {
            ((time - COLLISION) / 0.3 * std::f32::consts::PI).sin() * TILE * 0.06
        } else {
            0.0
        };
        actor(
            obstacle + vec2(recoil, 0.0),
            Visual::Bestiary {
                number: 12,
                corpse: false,
            },
        );
    }
    let bump = if (HIT..HIT + 0.24).contains(&time) && !action.pushes() {
        ((time - HIT) / 0.24 * std::f32::consts::PI).sin() * TILE * 0.07
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
    let lunge = if time < 0.38 {
        (time / 0.38 * std::f32::consts::PI).sin() * TILE * 0.12
    } else {
        0.0
    };
    actor(player + vec2(lunge, 0.0), player_visual());
    if time < 1.05 {
        draw_text_bold(
            action.title(),
            player.x - TILE * 0.68,
            player.y - TILE * 0.69 - time * 9.0,
            15.0,
            alpha(TEXT, (1.0 - time / 1.05).sqrt()),
        );
    }
    if (0.05..0.57).contains(&time) {
        let life = (1.0 - (time - HIT).max(0.0) / 0.35).clamp(0.0, 1.0);
        match action {
            Action::Rupture => {
                let phase = progress(time, 0.05, 0.20);
                let from = player + vec2(TILE * 0.34, -TILE * 0.62);
                let end = initial + vec2(TILE * 0.36, TILE * 0.42);
                cut(from, from.lerp(end, phase), TILE * 0.28, life);
            }
            Action::Pierce => {
                let start = player + vec2(TILE * 0.27, -TILE * 0.02);
                let tip = initial + vec2(TILE * 0.30, -TILE * 0.03);
                let phase = progress(time, 0.05, 0.17);
                if phase > 0.02 {
                    cut(start, start.lerp(tip, phase), TILE * 0.12, life);
                }
                luminous_line(start, start.lerp(tip, phase), 1.5, WHITE, life);
            }
            _ => {
                if time >= HIT {
                    let size = TILE * (0.13 + (time - HIT) * 0.4);
                    draw_rectangle_lines(
                        initial.x - size,
                        initial.y - size,
                        size * 2.0,
                        size * 2.0,
                        TILE * 0.05 * life,
                        alpha(WHITE, life),
                    );
                    luminous_line(
                        player + vec2(TILE * 0.29, 0.0),
                        initial,
                        TILE * 0.05,
                        STEEL,
                        life * 0.65,
                    );
                }
            }
        }
    }
    impact(
        initial,
        time - HIT,
        if action == Action::Rupture {
            1.15
        } else {
            0.75
        },
        action == Action::Pierce,
    );
    blood_spray(
        initial + vec2(0.0, -TILE * 0.08),
        time - HIT,
        100,
        if action == Action::Rupture { 1.2 } else { 0.8 },
    );
    if action.pushes() {
        let age = time - COLLISION;
        let contact = target + vec2(TILE * 0.42, 0.0);
        impact(contact, age, 1.0, action == Action::PushWall);
        blood_spray(target, age, 200, 0.65);
        if action == Action::PushEnemy {
            blood_spray(obstacle, age, 300, 0.8);
        }
    }
}

pub(super) async fn run() {
    request_new_screen_size(960.0, 650.0);
    let mut selected = 0;
    let mut age = 0.0;
    let mut paused = false;
    let mut speed = 1.0;
    let mut cooldown = 0_u16;
    let mut free = true;
    let mut turn = 0_u32;
    loop {
        if is_key_pressed(KeyCode::Escape) || is_quit_requested() {
            return;
        }
        clear_background(INK);
        draw_text_bold("LABORATOIRE / ANIMATIONS · LOT 02", 22.0, 35.0, 23.0, TEXT);
        draw_text(
            "ANIM-S02 · aperçu visuel · dégâts réels à raccorder",
            22.0,
            62.0,
            14.0,
            DIM,
        );
        for (i, action) in ACTIONS.iter().enumerate() {
            if button(
                Rect::new(20.0, 96.0 + i as f32 * 44.0, 238.0, 35.0),
                action.label(),
                true,
            ) {
                selected = i;
                age = 0.0;
                cooldown = 0;
                turn = 0;
            }
        }
        let action = ACTIONS[selected];
        scene(action, Rect::new(360.0, 100.0, TILE * 7.0, TILE * 5.0), age);
        draw_text(action.variant(), 360.0, 455.0, 14.0, DIM);
        draw_text(
            format!("Tour {turn} · recharge : {cooldown} tours · rang effectif 11"),
            22.0,
            515.0,
            15.0,
            TEXT,
        );
        let launch = button(
            Rect::new(20.0, 546.0, 125.0, 34.0),
            if cooldown > 0 && !free {
                "En recharge"
            } else {
                "Lancer"
            },
            free || cooldown == 0,
        );
        if (launch || is_key_pressed(KeyCode::Enter)) && (free || cooldown == 0) {
            age = 0.0;
            paused = false;
            cooldown = parameter_for(action.id(), "cooldown_turns", 11);
        }
        if button(Rect::new(154.0, 546.0, 118.0, 34.0), "+1 tour", true)
            || is_key_pressed(KeyCode::Space)
        {
            turn += 1;
            cooldown = cooldown.saturating_sub(1);
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
        if button(
            Rect::new(662.0, 546.0, 238.0, 34.0),
            if free {
                "Recharges libres : oui"
            } else {
                "Recharges libres : non"
            },
            true,
        ) {
            free = !free;
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
    request_new_screen_size(1280.0, 800.0);
    for _ in 0..4 {
        next_frame().await;
    }
    let mut entries = Vec::new();
    for (index, action) in ACTIONS.iter().enumerate() {
        let directory = output.join(action.variant());
        std::fs::create_dir(&directory).map_err(|e| e.to_string())?;
        for frame in 0..FRAMES {
            clear_background(INK);
            scene(
                *action,
                Rect::new(24.0, 24.0, 7.0 * TILE, 5.0 * TILE),
                frame as f32 / FPS as f32,
            );
            crate::ui_capture::framebuffer()?
                .export_png(directory.join(format!("{frame:03}.png")).to_str().unwrap());
            next_frame().await;
        }
        entries.push(
            serde_json::json!({"id": action.id(), "variant": action.variant(), "number": index + 4,
            "title": action.title(), "tile": 64, "folder": action.variant(), "frames": FRAMES,
            "fps": FPS, "crop": [24, 24, 448, 320]}),
        );
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
    fn push_preview_stops_on_the_free_cell_before_either_obstacle() {
        for action in [Action::PushWall, Action::PushEnemy] {
            assert_eq!(target_column(action, 0.0), 2.5);
            assert_eq!(target_column(action, COLLISION), 4.5);
            for frame in 0..FRAMES {
                assert!((2.5..=4.5).contains(&target_column(action, frame as f32 / FPS as f32)));
            }
        }
    }

    #[test]
    fn other_melee_previews_only_bump_and_do_not_push() {
        for action in [Action::Rupture, Action::Pierce] {
            assert_eq!(target_column(action, 2.0), 2.5);
        }
    }
}
