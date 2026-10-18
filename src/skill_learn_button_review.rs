//! LEARN-BTN-01..04: review-only drawings, with no engine or save mutation.
use super::{AsciiApp, agent_skill_window};
use crate::graphics::InterfaceStyle;
use crate::ui_theme::{UiTheme, draw_text as label, draw_text_bold, draw_text_bold_centered};
use macroquad::prelude::*;

const SIZE: (f32, f32) = (1280., 960.);
const NAMES: [&str; 4] = [
    "Plaque biseautée",
    "Sceau hexagonal",
    "Relais circulaire",
    "Languette intégrée",
];
const NOTES: [&str; 4] = [
    "Métal gravé, creux marqué et arêtes usées.",
    "Un petit module serti, entre technologie et occultisme.",
    "Une commande ronde, avec un anneau lumineux discret.",
    "Un prolongement du cadre, solidaire de l'icône.",
];
#[derive(Clone, Copy)]
enum State {
    Ready,
    Hover,
    Pressed,
    Disabled,
    Capped,
}
#[derive(Clone, Copy)]
struct Hotspot {
    candidate: usize,
    slot: usize,
    rect: Rect,
    enabled: bool,
}
fn alpha(mut c: Color, a: f32) -> Color {
    c.a *= a;
    c
}
fn polygon(points: &[Vec2], c: Color) {
    for i in 1..points.len() - 1 {
        draw_triangle(points[0], points[i], points[i + 1], c);
    }
}
fn outline(points: &[Vec2], width: f32, c: Color) {
    for i in 0..points.len() {
        let (a, b) = (points[i], points[(i + 1) % points.len()]);
        draw_line(a.x, a.y, b.x, b.y, width, c);
    }
}
fn shape(kind: usize, r: Rect) -> Vec<Vec2> {
    let (x, y, w, h) = (r.x, r.y, r.w, r.h);
    match kind {
        0 => vec![
            vec2(x + 5., y),
            vec2(x + w - 5., y),
            vec2(x + w, y + 5.),
            vec2(x + w, y + h - 5.),
            vec2(x + w - 5., y + h),
            vec2(x + 5., y + h),
            vec2(x, y + h - 5.),
            vec2(x, y + 5.),
        ],
        1 => vec![
            vec2(x + 8., y),
            vec2(x + w - 8., y),
            vec2(x + w, y + h / 2.),
            vec2(x + w - 8., y + h),
            vec2(x + 8., y + h),
            vec2(x, y + h / 2.),
        ],
        2 => (0..32)
            .map(|i| {
                let a = i as f32 / 32. * std::f32::consts::TAU;
                r.center() + vec2(a.cos() * w / 2., a.sin() * h / 2.)
            })
            .collect(),
        _ => vec![
            vec2(x, y + 4.),
            vec2(x + w - 6., y + 4.),
            vec2(x + w, y + 10.),
            vec2(x + w, y + h - 6.),
            vec2(x + w - 6., y + h),
            vec2(x, y + h),
        ],
    }
}
fn inset(r: Rect, amount: f32) -> Rect {
    Rect::new(
        r.x + amount,
        r.y + amount,
        r.w - amount * 2.,
        r.h - amount * 2.,
    )
}
fn button(kind: usize, r: Rect, state: State) {
    if kind == 0 {
        use crate::skill_learn_button::State as Accepted;
        let state = match state {
            State::Ready => Accepted::Ready,
            State::Hover => Accepted::Hover,
            State::Pressed => Accepted::Pressed,
            State::Disabled => Accepted::Disabled,
            State::Capped => Accepted::Capped,
        };
        crate::skill_learn_button::draw(r, state);
        return;
    }
    let disabled = matches!(state, State::Disabled | State::Capped);
    let hover = matches!(state, State::Hover);
    let pressed = matches!(state, State::Pressed);
    let terminal = crate::ui_theme::interface_style() == InterfaceStyle::Terminal;
    let accent = if disabled {
        Color::from_rgba(91, 99, 111, 255)
    } else {
        UiTheme.accent()
    };
    let edge = if hover || pressed {
        accent
    } else {
        Color::from_rgba(93, 101, 115, 255)
    };
    let outer = shape(kind, r);
    let shadow = outer.iter().map(|p| *p + vec2(0., 2.)).collect::<Vec<_>>();
    polygon(&shadow, Color::from_rgba(0, 1, 4, 205));
    polygon(&outer, Color::from_rgba(9, 12, 17, 255));
    outline(&outer, 1., edge);
    if kind == 3 {
        draw_rectangle(
            r.x - 8.,
            r.y + 4.,
            9.,
            r.h - 4.,
            Color::from_rgba(27, 33, 43, 255),
        );
        draw_line(r.x - 8., r.y + 4., r.x, r.y + 4., 1., edge);
        draw_line(r.x - 8., r.bottom(), r.x, r.bottom(), 1., edge);
    }
    let inner = shape(kind, inset(r, 3.));
    let fill = if pressed {
        Color::from_rgba(15, 18, 28, 255)
    } else if hover {
        Color::new(accent.r * 0.26, accent.g * 0.26, accent.b * 0.26, 1.)
    } else {
        Color::from_rgba(35, 41, 51, 255)
    };
    polygon(&inner, fill);
    outline(&inner, 1., Color::from_rgba(64, 71, 83, 255));
    if !terminal {
        // Low contrast machining marks stay inside the inset shape.
        for (i, y) in [9., 13., 17., 21., 25.].iter().enumerate() {
            let trim = if kind == 1 || kind == 2 { 8. } else { 6. };
            draw_line(
                r.x + trim,
                r.y + y,
                r.right() - trim,
                r.y + y,
                1.,
                Color::from_rgba(156, 169, 185, if i % 2 == 0 { 10 } else { 5 }),
            );
        }
        let a = inner[0];
        let b = inner[1];
        draw_line(
            a.x,
            a.y,
            b.x,
            b.y,
            1.,
            alpha(WHITE, if disabled { 0.07 } else { 0.18 }),
        );
    }
    if kind == 1 {
        for sign in [-1., 1.] {
            let x = r.center().x + sign * (r.w / 2. - 4.);
            draw_line(
                x,
                r.center().y - 2.,
                x,
                r.center().y + 2.,
                1.,
                alpha(accent, 0.85),
            );
        }
    } else if kind == 2 {
        for (a, b) in [(205f32, 255f32), (25., 75.)] {
            for step in 0..8 {
                let t = (a + (b - a) * step as f32 / 8.).to_radians();
                let u = (a + (b - a) * (step + 1) as f32 / 8.).to_radians();
                let p = r.center() + vec2(t.cos(), t.sin()) * 13.;
                let q = r.center() + vec2(u.cos(), u.sin()) * 13.;
                draw_line(p.x, p.y, q.x, q.y, 1., alpha(accent, 0.80));
            }
        }
    }
    if hover {
        outline(&shape(kind, inset(r, -1.)), 1., alpha(accent, 0.27));
    }
    let centre = r.center() + if pressed { vec2(0., 1.) } else { Vec2::ZERO };
    let glyph = if disabled {
        Color::from_rgba(112, 121, 135, 255)
    } else if hover {
        Color::from_rgba(236, 233, 255, 255)
    } else {
        accent
    };
    if matches!(state, State::Capped) {
        draw_line(
            centre.x - 5.,
            centre.y,
            centre.x - 1.,
            centre.y + 4.,
            2.,
            glyph,
        );
        draw_line(
            centre.x - 1.,
            centre.y + 4.,
            centre.x + 6.,
            centre.y - 4.,
            2.,
            glyph,
        );
    } else {
        draw_rectangle(centre.x - 6. + 1., centre.y - 1. + 1., 12., 3., BLACK);
        draw_rectangle(centre.x - 1. + 1., centre.y - 6. + 1., 3., 12., BLACK);
        draw_rectangle(centre.x - 6., centre.y - 1., 12., 3., glyph);
        draw_rectangle(centre.x - 1., centre.y - 6., 3., 12., glyph);
    }
}
fn board(pointer: Option<Vec2>, pressed: bool, ranks: &[[u8; 3]; 4]) -> Vec<Hotspot> {
    clear_background(Color::from_rgba(9, 12, 18, 255));
    let offset = vec2(
        (screen_width() - SIZE.0) / 2.,
        (screen_height() - SIZE.1) / 2.,
    );
    let text = UiTheme.text();
    let muted = UiTheme.muted();
    draw_text_bold(
        "APPRENDRE / AMÉLIORER · QUATRE BOUTONS",
        offset.x + 24.,
        offset.y + 36.,
        24.,
        text,
    );
    label(
        "Vraies icônes de 64 px · boutons à taille native · propositions LEARN-BTN-01 à 04",
        offset.x + 24.,
        offset.y + 63.,
        16.,
        muted,
    );
    let mut hotspots = Vec::new();
    for kind in 0..4 {
        let r = Rect::new(
            offset.x + 24. + (kind % 2) as f32 * 628.,
            offset.y + 88. + (kind / 2) as f32 * 370.,
            604.,
            350.,
        );
        UiTheme.card(r, false);
        draw_text_bold(
            &format!("0{} · {}", kind + 1, NAMES[kind]),
            r.x + 18.,
            r.y + 31.,
            20.,
            text,
        );
        label(NOTES[kind], r.x + 18., r.y + 56., 14., muted);
        for (slot, (id, title, heading)) in [
            ("FE01", "Braise", "Apprendre"),
            ("FE02", "Souffle de feu", "Améliorer"),
            ("FR01", "Aiguille de givre", "Sans point"),
        ]
        .iter()
        .enumerate()
        {
            let sx = r.x + 18. + slot as f32 * 188.;
            draw_text_bold_centered(heading, Rect::new(sx, r.y + 78., 160., 21.), 14, muted);
            let icon = Rect::new(sx + 18., r.y + 112., 64., 64.);
            draw_rectangle_lines(
                icon.x - 1.,
                icon.y - 1.,
                66.,
                66.,
                1.,
                Color::from_rgba(66, 75, 91, 255),
            );
            agent_skill_window::art(id, "compact", icon, None);
            let rect = Rect::new(
                icon.right() + 8.,
                icon.y + 16.,
                if kind == 3 { 36. } else { 32. },
                32.,
            );
            let enabled = slot < 2 && ranks[kind][slot] < 10;
            let hover = pointer.is_some_and(|p| rect.contains(p));
            let state = if ranks[kind][slot] >= 10 {
                State::Capped
            } else if !enabled {
                State::Disabled
            } else if hover && pressed {
                State::Pressed
            } else if hover {
                State::Hover
            } else {
                State::Ready
            };
            button(kind, rect, state);
            draw_text_bold_centered(title, Rect::new(sx, r.y + 185., 160., 19.), 13, text);
            draw_text_bold_centered(
                &format!("Rang {}", ranks[kind][slot]),
                Rect::new(sx, r.y + 207., 160., 18.),
                12,
                muted,
            );
            hotspots.push(Hotspot {
                candidate: kind,
                slot,
                rect,
                enabled,
            });
        }
        draw_line(
            r.x + 18.,
            r.y + 241.,
            r.right() - 18.,
            r.y + 241.,
            1.,
            Color::from_rgba(50, 57, 70, 255),
        );
        for (i, (state, name)) in [
            (State::Ready, "Repos"),
            (State::Hover, "Survol"),
            (State::Pressed, "Appui"),
            (State::Disabled, "Sans point"),
            (State::Capped, "Plafond"),
        ]
        .iter()
        .enumerate()
        {
            let cx = r.x + 57. + i as f32 * 119.;
            button(
                kind,
                Rect::new(cx - 16., r.y + 260., if kind == 3 { 36. } else { 32. }, 32.),
                *state,
            );
            draw_text_bold_centered(name, Rect::new(cx - 47., r.y + 303., 94., 20.), 12, muted);
        }
    }
    label(
        "Le bouton + apprend ou améliore. L'image de la compétence restera réservée à sa démonstration.",
        offset.x + 24.,
        offset.y + 866.,
        16.,
        text,
    );
    label(
        "Les boutons sont dessinés dans le moteur ; les icônes et les rangs servent ici de contexte de comparaison.",
        offset.x + 24.,
        offset.y + 894.,
        14.,
        muted,
    );
    label(
        "Revue uniquement · aucune compétence achetée dans une partie · T : métal / terminal · ESC : fermer l'aperçu",
        offset.x + 24.,
        offset.y + 925.,
        14.,
        muted,
    );
    hotspots
}
impl AsciiApp {
    pub async fn capture_skill_learn_buttons(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(SIZE.0, SIZE.1);
        for _ in 0..3 {
            next_frame().await;
        }
        if (screen_width(), screen_height()) != SIZE {
            return Err("Native 1280 × 960 surface unavailable".into());
        }
        for (style, name) in [
            (InterfaceStyle::Metal, "boutons-metal.png"),
            (InterfaceStyle::Terminal, "boutons-terminal.png"),
        ] {
            crate::ui_theme::set_interface_style(style);
            for _ in 0..3 {
                board(None, false, &[[0, 3, 3]; 4]);
                next_frame().await;
            }
            board(None, false, &[[0, 3, 3]; 4]);
            crate::ui_capture::framebuffer()?
                .export_png(output.join(name).to_str().ok_or("Capture path")?);
            next_frame().await;
        }
        std::fs::write(output.join("native-checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "review":"SKILL-LEARN-BUTTONS-01","status":"proposals_only","candidates":NAMES.iter().enumerate().map(|(i,n)|serde_json::json!({"id":format!("LEARN-BTN-{:02}",i+1),"name":n})).collect::<Vec<_>>(),
            "surface":[1280,960],"skill_icon_pixels":64,"button_pixels":[[32,32],[32,32],[32,32],[36,32]],
            "game_instance_created":false,"engine_commands":0,"normal_ui_changed":false,"styles":["metal","terminal"]
        })).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }
    pub async fn run_skill_learn_buttons_review() {
        request_new_screen_size(SIZE.0, SIZE.1);
        let mut ranks = [[0, 3, 3]; 4];
        loop {
            if is_key_pressed(KeyCode::Escape) {
                break;
            }
            if is_key_pressed(KeyCode::T) {
                let style = if crate::ui_theme::interface_style() == InterfaceStyle::Metal {
                    InterfaceStyle::Terminal
                } else {
                    InterfaceStyle::Metal
                };
                crate::ui_theme::set_interface_style(style);
            }
            let (x, y) = mouse_position();
            let hotspots = board(
                Some(vec2(x, y)),
                is_mouse_button_down(MouseButton::Left),
                &ranks,
            );
            if is_mouse_button_pressed(MouseButton::Left) {
                for h in hotspots {
                    if h.enabled && h.rect.contains(vec2(x, y)) {
                        ranks[h.candidate][h.slot] += 1;
                    }
                }
            }
            next_frame().await;
        }
    }
}
