//! ANIM-STYLE01: illustrated fire frames, isolated from campaign/combat rules.
use super::melee::actor;
use super::*;

pub(super) mod obstacles;
pub(crate) mod zones;

const TILE: f32 = 64.0;
const IDS: [&str; 3] = ["FE02", "FE03", "FE04"];
const TITLES: [&str; 3] = ["Souffle de feu", "Mur de feu", "Émanation soudaine"];
const VARIANTS: [&str; 3] = ["FE02-A02", "FE03-A02", "FE04-A02"];
const CROSS: [(i32, i32); 5] = [(0, 0), (1, 0), (-1, 0), (0, 1), (0, -1)];
const CAPTURE_TURN: f32 = 0.85;

struct DrawnFire {
    plume: Texture2D,
    cone: Texture2D,
}
impl DrawnFire {
    fn load() -> Self {
        let decode = |data: &[u8]| {
            let image = Image::from_file_with_format(data, Some(ImageFormat::Png)).unwrap();
            let texture = Texture2D::from_image(&image);
            texture.set_filter(FilterMode::Linear);
            texture
        };
        Self {
            plume: decode(include_bytes!(
                "../../artifacts/refonte-agent-2026-10-06/skill-animation-review-06/flame-atlas.png"
            )),
            cone: decode(include_bytes!(
                "../../artifacts/refonte-agent-2026-10-06/skill-animation-review-06/cone-atlas.png"
            )),
        }
    }
    fn plume(&self, base: Vec2, time: f32, phase: u32, scale: f32, opacity: f32) {
        let frame = (time.max(0.0) * 12.0) as u32 + phase;
        let width = 128.0 * scale;
        draw_texture_ex(
            &self.plume,
            base.x - 64.0 * scale,
            base.y - 120.0 * scale,
            alpha(WHITE, opacity),
            DrawTextureParams {
                source: Some(Rect::new((frame % 8) as f32 * 128.0, 0.0, 128.0, 128.0)),
                dest_size: Some(vec2(width, width)),
                ..Default::default()
            },
        );
    }
    fn torrent(&self, emitter: Vec2, time: f32, opacity: f32) {
        // Frame 0 illustrates the first instant; seven stable-size drawings loop thereafter.
        let frame = if time < 0.083 {
            0
        } else {
            1 + (time * 12.0) as u32 % 7
        };
        draw_texture_ex(
            &self.cone,
            emitter.x - 24.0,
            emitter.y - 192.0,
            alpha(WHITE, opacity),
            DrawTextureParams {
                source: Some(Rect::new(frame as f32 * 384.0, 0.0, 384.0, 384.0)),
                dest_size: Some(vec2(384.0, 384.0)),
                ..Default::default()
            },
        );
    }
    fn burn(&self, at: Vec2, time: f32, phase: u32, opacity: f32) {
        self.plume(at + vec2(-9.0, 19.0), time, phase, 0.33, opacity);
        self.plume(at + vec2(10.0, 19.0), time, phase + 3, 0.27, opacity);
    }
}

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
            let p = cell(rect, x as f32, y as f32);
            game_art::draw_world(
                Rect::new(p.x, p.y, TILE, TILE),
                Visual::Terrain {
                    kind: crate::test_sector::Decor::Deck,
                    position: GridPos::new(x, y),
                },
                1.0,
            );
            draw_rectangle_lines(
                p.x,
                p.y,
                TILE,
                TILE,
                0.7,
                Color::new(0.15, 0.18, 0.19, 0.75),
            );
        }
    }
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

fn breath(fire: &DrawnFire, rect: Rect, time: f32, active: bool) {
    let player = cell(rect, 1.5, 3.5);
    if active {
        fire.torrent(player + vec2(18.0, -5.0), time, 1.0);
    }
    // Draw targets over the ground-facing fire, then their small flame overlays.
    // This keeps their silhouettes legible without adding an outline/reveal mechanic.
    for (index, (x, y)) in [(3.5, 3.5), (4.5, 1.5), (4.5, 5.5), (6.5, 3.5)]
        .into_iter()
        .enumerate()
    {
        let at = cell(rect, x, y);
        enemy(at);
        if active && index < 3 {
            fire.burn(at, time, index as u32 * 2, 0.9);
        }
    }
    actor(player, caster());
    name(0, player, time);
}

fn wall(fire: &DrawnFire, rect: Rect, time: f32, active: bool) {
    if active {
        for x in 2..=6 {
            fire.plume(
                cell(rect, x as f32 + 0.5, 3.5) + vec2(0.0, 24.0),
                time,
                x * 3,
                1.0,
                1.0,
            );
        }
    }
    let leaving = cell(rect, 4.5, 3.5 + ((time - 1.5) / 0.55).clamp(0.0, 1.0));
    enemy(leaving);
    enemy(cell(rect, 6.5, 3.5));
    enemy(cell(rect, 4.5, 1.5));
    if active {
        fire.burn(leaving, time, 2, 0.95);
        fire.burn(cell(rect, 6.5, 3.5), time, 5, 0.95);
    }
    let player = cell(rect, 1.5, 5.5);
    actor(player, caster());
    name(1, player, time);
}

fn cracks(at: Vec2, time: f32, seed: u32, opacity: f32) {
    for n in 0..5 {
        let a = noise(seed + n) * std::f32::consts::TAU;
        let dir = vec2(a.cos(), a.sin());
        let normal = vec2(-dir.y, dir.x);
        let mut start = at;
        for step in 0..3 {
            let end = at
                + dir * (step as f32 + 1.0) * 9.0
                + normal * ((noise(seed + n + step * 30) - 0.5) * 12.0);
            draw_line(
                start.x,
                start.y,
                end.x,
                end.y,
                5.0,
                alpha(Color::new(0.18, 0.035, 0.006, 1.0), opacity),
            );
            draw_line(
                start.x,
                start.y,
                end.x,
                end.y,
                2.0,
                alpha(Color::new(0.95, 0.32, 0.035, 1.0), opacity),
            );
            draw_line(
                start.x,
                start.y,
                end.x,
                end.y,
                0.8,
                alpha(
                    Color::new(1.0, 0.76, 0.21, 1.0),
                    opacity * (0.85 + 0.15 * (time * 8.0 + n as f32).sin()),
                ),
            );
            start = end;
        }
    }
}

fn eruption(fire: &DrawnFire, rect: Rect, time: f32, burst: Option<f32>) {
    let center = cell(rect, 4.5, 3.5);
    if burst.is_none() {
        for (index, &(x, y)) in CROSS.iter().enumerate() {
            let at = center + vec2(x as f32, y as f32) * TILE;
            draw_rectangle(
                at.x - 31.0,
                at.y - 31.0,
                62.0,
                62.0,
                Color::new(0.6, 0.17, 0.02, 0.07),
            );
            cracks(at + vec2(0.0, 20.0), time, index as u32 * 27, 1.0);
        }
    }
    for &(x, y) in &CROSS {
        enemy(center + vec2(x as f32, y as f32) * TILE);
    }
    let player = cell(rect, 1.5, 3.5);
    actor(player, caster());
    name(2, player, time);
    if let Some(age) = burst {
        let life = (1.0 - age / 0.88).clamp(0.0, 1.0);
        // The rapid rise is presentation of the already-resolved blast, not another delay.
        let rise = (0.5 + age / 0.13).min(1.0);
        for (index, &(x, y)) in CROSS.iter().enumerate() {
            let at = center + vec2(x as f32, y as f32) * TILE;
            if life > 0.0 {
                fire.plume(
                    at + vec2(0.0, 23.0),
                    age,
                    index as u32 * 2,
                    1.15 * rise,
                    life.min(1.0),
                );
            }
            if age > 0.14 {
                fire.burn(at, time, index as u32, 0.9);
            }
        }
    }
}

fn scene(fire: &DrawnFire, skill: usize, rect: Rect, age: f32, active: bool, burst: Option<f32>) {
    base(rect);
    match skill {
        0 => breath(fire, rect, age, active),
        1 => wall(fire, rect, age, active),
        _ => eruption(fire, rect, age, burst),
    }
    let label = match skill {
        0 => "Cône · 5 cases · 3 tours",
        1 => "Rideau traversable · brûlure après la sortie",
        _ if burst.is_none() => "Préparation · 1 tour",
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
    let fire = DrawnFire::load();
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
        draw_text_bold("LABORATOIRE / FLAMMES DESSINÉES", 22.0, 35.0, 23.0, TEXT);
        draw_text(
            "ANIM-STYLE01 · proposition de style · cases natives de 64 px",
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
            &fire,
            selected,
            Rect::new(335.0, 95.0, 448.0, 448.0),
            age,
            remaining > 0,
            burst_start.map(|start| age - start),
        );
        draw_text(VARIANTS[selected], 335.0, 567.0, 14.0, DIM);
        draw_text(
            format!("Tour {turn} · recharge : {cooldown} tours · aperçu au rang 11"),
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
            remaining = match selected {
                0 => 3,
                1 => 5,
                _ => 1,
            };
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
            "Émanation attend +1 tour · rejouer ou ralentir ne fait pas avancer les tours",
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
    let fire = DrawnFire::load();
    let mut entries = Vec::new();
    for skill in 0..3 {
        let directory = output.join(VARIANTS[skill]);
        std::fs::create_dir(&directory).map_err(|e| e.to_string())?;
        for frame in 0..120 {
            clear_background(INK);
            let age = frame as f32 / 30.0;
            let burst = if skill == 2 && age >= CAPTURE_TURN {
                Some(age - CAPTURE_TURN)
            } else {
                None
            };
            scene(
                &fire,
                skill,
                Rect::new(24.0, 24.0, 448.0, 448.0),
                age,
                true,
                burst,
            );
            crate::ui_capture::framebuffer()?
                .export_png(directory.join(format!("{frame:03}.png")).to_str().unwrap());
            next_frame().await;
        }
        entries.push(serde_json::json!({"id":IDS[skill],"variant":VARIANTS[skill],"number":skill+10,"title":TITLES[skill],"tile":64,"folder":VARIANTS[skill],"frames":120,"fps":30,"framebuffer_size":[512,512],"crop":[24,24,448,448]}));
    }
    std::fs::write(
        output.join("capture.json"),
        serde_json::to_vec_pretty(&entries).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}
