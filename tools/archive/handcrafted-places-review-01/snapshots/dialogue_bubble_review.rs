//! DIALOGUE-BUBBLE-01: shared terminal r02 renderer and scoped notification state.
use crate::ui_theme as ui;
#[cfg(debug_assertions)]
use crate::ui_theme::UiTheme;
use macroquad::prelude::*;

#[derive(Clone, Copy, Default)]
struct ReviewState {
    speaker: Option<project_rl::world::GridPos>,
    open: bool,
    ready: bool,
}
thread_local! {static STATE: std::cell::Cell<ReviewState> = const {std::cell::Cell::new(ReviewState {speaker: None, open: false, ready: false})};}

pub fn active() -> bool {
    STATE.with(|state| state.get().open)
}
pub struct ReviewGuard(ReviewState);
impl Drop for ReviewGuard {
    fn drop(&mut self) {
        STATE.with(|state| state.set(self.0));
    }
}

pub fn begin(speaker: project_rl::world::GridPos) -> ReviewGuard {
    ReviewGuard(STATE.with(|state| {
        state.replace(ReviewState {
            speaker: Some(speaker),
            open: true,
            ready: false,
        })
    }))
}
pub fn set_open(open: bool) {
    STATE.with(|state| {
        state.set(ReviewState {
            open,
            ..state.get()
        })
    });
}
#[cfg(any(debug_assertions, test))]
pub fn set_ready() {
    STATE.with(|state| {
        state.set(ReviewState {
            ready: true,
            ..state.get()
        })
    });
}

pub fn reviewed_quest_marker(
    at: project_rl::world::GridPos,
) -> Option<project_rl::game::QuestMarker> {
    STATE.with(|state| {
        let state = state.get();
        (state.ready && state.speaker == Some(at))
            .then_some(project_rl::game::QuestMarker::ReadyToComplete)
    })
}
pub fn quest_notification_hidden(
    at: project_rl::world::GridPos,
    marker: project_rl::game::QuestMarker,
) -> bool {
    notification_hidden(at, marker != project_rl::game::QuestMarker::ReadyToComplete)
}
/// A read quest notification stays hidden. Service labels hide only while talking.
/// The caller scopes this presentation state to the current camp and frame.
pub fn notification_hidden(at: project_rl::world::GridPos, quest: bool) -> bool {
    STATE.with(|state| {
        let state = state.get();
        state.speaker == Some(at) && (state.open || quest)
    })
}

fn wrapped(text: &str, width: f32, measure: impl Fn(&str) -> f32) -> Vec<String> {
    let mut lines = Vec::new();
    for (p, paragraph) in text.split("\n\n").enumerate() {
        if p > 0 {
            lines.push(String::new());
        }
        let mut line = String::new();
        for word in paragraph.split_whitespace() {
            let next = if line.is_empty() {
                word.to_owned()
            } else {
                format!("{line} {word}")
            };
            if !line.is_empty() && measure(&next) > width {
                lines.push(std::mem::take(&mut line));
                line.push_str(word);
            } else {
                line = next;
            }
        }
        if !line.is_empty() {
            lines.push(line);
        }
    }
    lines
}

/// Consume a dismissal frame so its click cannot also move/attack in the world.
/// Shared by ordinary terminal dialogues and their isolated captures.
pub fn take_dismiss(input: &mut crate::controls::InputFrame) -> bool {
    use crate::controls::Binding;
    if !input.pause
        && !input.pressed.contains(&Binding::MouseLeft)
        && !input.pressed.contains(&Binding::key("Escape"))
    {
        return false;
    }
    input.pressed.clear();
    input.held.clear();
    input.pause = false;
    input.navigation_repeat = None;
    input.mouse_left_down = false;
    input.wheel_y = 0.;
    true
}

pub const CHARACTERS_PER_SECOND: f64 = 40.;
struct Reveal {
    text: String,
    cursor_on: bool,
}
fn reveal(display: &str, elapsed: Option<f64>, reduced_motion: bool) -> Reveal {
    if elapsed.is_none() || reduced_motion {
        return Reveal {
            text: display.to_owned(),
            cursor_on: true,
        };
    }
    let t = elapsed.filter(|t| t.is_finite() && *t >= 0.).unwrap_or(0.);
    let count = display.chars().count();
    let shown = ((t * CHARACTERS_PER_SECOND).floor() as usize).min(count);
    let finished_at = count as f64 / CHARACTERS_PER_SECOND;
    Reveal {
        text: display.chars().take(shown).collect(),
        cursor_on: shown < count || ((t - finished_at).max(0.) / 0.5).floor() as u64 % 2 == 0,
    }
}

#[cfg_attr(not(debug_assertions), allow(dead_code))]
pub struct TerminalFrame {
    pub bounds: Rect,
    pub cursor: Rect,
    pub characters: usize,
}

#[cfg(debug_assertions)]
#[allow(
    dead_code,
    reason = "Historical r01 renderer retained with its review sources"
)]
pub fn terminal(viewport: Rect, speaker: Rect, name: &str, text: &str) -> Result<Rect, String> {
    Ok(terminal_at(viewport, speaker, name, text, None, false)?.bounds)
}

pub fn terminal_at(
    viewport: Rect,
    speaker: Rect,
    name: &str,
    text: &str,
    elapsed: Option<f64>,
    reduced_motion: bool,
) -> Result<TerminalFrame, String> {
    let width = 660_f32.min(viewport.w - 36.);
    let lines = wrapped(text, width - 44., |s| {
        ui::measure_text(s, None, 18, 1.).width
    });
    let height = lines
        .iter()
        .map(|s| if s.is_empty() { 10. } else { 23. })
        .sum::<f32>()
        + 70.;
    let r = Rect::new(
        (viewport.x + 18.).round(),
        (speaker.y - height - 30.).max(viewport.y + 14.).round(),
        width,
        height,
    );
    if r.bottom() > viewport.bottom() || r.contains(speaker.center()) {
        return Err("La bulle masque son interlocuteur".into());
    }
    let tip = vec2(speaker.center().x, speaker.y + 5.);
    let base = tip.x.clamp(r.x + 30., r.right() - 30.);
    let left = vec2(base - 14., r.bottom());
    let right = vec2(base + 14., r.bottom());
    let fill = Color::from_rgba(10, 25, 19, 224);
    let accent = Color::from_rgba(154, 210, 158, 255);
    // These fills share an edge without overlap. In particular, there is no
    // second translucent layer or rectangle outline across the tail's root.
    draw_rectangle(r.x, r.y, r.w, r.h, fill);
    draw_triangle(left, right, tip, fill);
    let mut sy = r.y + 3.;
    while sy < tip.y - 2. {
        if (sy - r.bottom()).abs() > 2. {
            let (x0, x1) = if sy < r.bottom() {
                (r.x + 1., r.right() - 1.)
            } else {
                let t = (sy - r.bottom()) / (tip.y - r.bottom());
                (
                    left.x + (tip.x - left.x) * t + 1.,
                    right.x + (tip.x - right.x) * t - 1.,
                )
            };
            if x1 > x0 {
                draw_line(x0, sy, x1, sy, 1., Color::new(0., 0., 0., 0.10));
            }
        }
        sy += 3.;
    }
    let perimeter = [
        vec2(r.x, r.y),
        vec2(r.right(), r.y),
        vec2(r.right(), r.bottom()),
        right,
        tip,
        left,
        vec2(r.x, r.bottom()),
        vec2(r.x, r.y),
    ];
    for edge in perimeter.windows(2) {
        draw_line(edge[0].x, edge[0].y, edge[1].x, edge[1].y, 1., accent);
    }
    ui::draw_text_bold(name, r.x + 22., r.y + 29., 19., accent);
    draw_line(
        r.x + 20.,
        r.y + 42.,
        r.right() - 20.,
        r.y + 42.,
        1.,
        Color::new(accent.r, accent.g, accent.b, 0.23),
    );
    let display = lines.join("\n");
    let shown = reveal(&display, elapsed, reduced_motion);
    let mut by = r.y + 68.;
    let baselines: Vec<_> = lines
        .iter()
        .map(|line| {
            let y = by;
            by += if line.is_empty() { 10. } else { 23. };
            y
        })
        .collect();
    let visible: Vec<_> = shown.text.split('\n').collect();
    for (i, line) in visible.iter().enumerate() {
        if !line.is_empty() {
            ui::draw_text(
                line,
                r.x + 22.,
                baselines[i],
                18.,
                Color::from_rgba(202, 221, 201, 255),
            );
        }
    }
    let cursor = Rect::new(
        (r.x + 22. + ui::measure_text(visible.last().unwrap_or(&""), None, 18, 1.).width + 3.)
            .round(),
        (baselines
            .get(visible.len() - 1)
            .copied()
            .unwrap_or(r.y + 68.)
            - 15.)
            .round(),
        9.,
        17.,
    );
    if shown.cursor_on {
        draw_rectangle(cursor.x, cursor.y, cursor.w, cursor.h, accent);
    }
    if text.split_whitespace().collect::<Vec<_>>()
        != lines
            .iter()
            .flat_map(|s| s.split_whitespace())
            .collect::<Vec<_>>()
        || by > r.bottom()
    {
        return Err("Texte de dialogue tronqué".into());
    }
    Ok(TerminalFrame {
        bounds: r,
        cursor,
        characters: display.chars().count(),
    })
}

#[cfg(debug_assertions)]
#[allow(
    dead_code,
    reason = "Historical r01 candidates retained with their review sources"
)]
pub fn draw(
    style: usize,
    viewport: Rect,
    speaker: Rect,
    name: &str,
    text: &str,
) -> Result<Rect, String> {
    if style == 2 {
        return terminal(viewport, speaker, name, text);
    }
    let width = (if style == 1 { 820_f32 } else { 660_f32 }).min(viewport.w - 36.);
    let lines = wrapped(text, width - 44., |s| {
        ui::measure_text(s, None, 18, 1.).width
    });
    let height = lines
        .iter()
        .map(|s| if s.is_empty() { 10. } else { 23. })
        .sum::<f32>()
        + 111.;
    let x = if style == 2 {
        viewport.x + 18.
    } else {
        (speaker.center().x - width / 2.).clamp(viewport.x + 18., viewport.right() - width - 18.)
    };
    let y = if style == 1 {
        viewport.bottom() - height - 18.
    } else {
        (speaker.y - height - 30.).max(viewport.y + 14.)
    };
    let r = Rect::new(x.round(), y.round(), width, height);
    if r.bottom() > viewport.bottom() || r.contains(speaker.center()) {
        return Err("La bulle masque son interlocuteur".into());
    }
    let accent = match style {
        0 => Color::from_rgba(214, 191, 145, 255),
        1 => UiTheme.accent(),
        _ => Color::from_rgba(154, 210, 158, 255),
    };
    let fill = match style {
        0 => Color::from_rgba(22, 27, 30, 255),
        1 => UiTheme.surface(),
        _ => Color::from_rgba(10, 25, 19, 250),
    };
    let tip = vec2(speaker.center().x, speaker.y + 5.);
    if style == 1 {
        // Keep the connector beside the actor rather than across his body.
        let stem = (speaker.right() + 28.).min(r.right() - 28.);
        draw_line(stem, r.y, stem, tip.y, 2., accent);
        draw_line(stem, tip.y, tip.x + 18., tip.y, 2., accent);
        draw_triangle(
            vec2(stem - 11., r.y),
            vec2(stem + 11., r.y),
            vec2(stem, r.y - 17.),
            accent,
        );
    } else {
        let base = tip.x.clamp(r.x + 30., r.right() - 30.);
        draw_triangle(
            vec2(base - 14., r.bottom() - 1.),
            vec2(base + 14., r.bottom() - 1.),
            tip,
            accent,
        );
        draw_triangle(
            vec2(base - 11., r.bottom() - 2.),
            vec2(base + 11., r.bottom() - 2.),
            vec2(tip.x, tip.y - 4.),
            fill,
        );
    }
    match style {
        0 => {
            UiTheme.context_surface(r);
            draw_rectangle_lines(r.x, r.y, r.w, r.h, 1., accent);
        }
        1 => {
            UiTheme.panel(r);
            draw_rectangle_lines(
                r.x + 2.,
                r.y + 2.,
                r.w - 4.,
                r.h - 4.,
                2.,
                Color::from_rgba(65, 74, 82, 255),
            );
            for (dx, dy) in [
                (8., 8.),
                (r.w - 8., 8.),
                (8., r.h - 8.),
                (r.w - 8., r.h - 8.),
            ] {
                draw_circle(r.x + dx, r.y + dy, 2., Color::from_rgba(113, 123, 124, 255));
            }
        }
        _ => {
            draw_rectangle(r.x, r.y, r.w, r.h, fill);
            draw_rectangle_lines(r.x, r.y, r.w, r.h, 1., accent);
            let mut sy = r.y + 3.;
            while sy < r.bottom() {
                draw_line(
                    r.x + 1.,
                    sy,
                    r.right() - 1.,
                    sy,
                    1.,
                    Color::new(0., 0., 0., 0.15),
                );
                sy += 3.;
            }
            for (px, py, sx, sy) in [
                (r.x, r.y, 1., 1.),
                (r.right(), r.y, -1., 1.),
                (r.x, r.bottom(), 1., -1.),
                (r.right(), r.bottom(), -1., -1.),
            ] {
                draw_line(px, py, px + sx * 12., py, 2., accent);
                draw_line(px, py, px, py + sy * 12., 2., accent);
            }
        }
    }
    ui::draw_text_bold(name, r.x + 22., r.y + 29., 19., accent);
    draw_line(
        r.x + 20.,
        r.y + 42.,
        r.right() - 20.,
        r.y + 42.,
        1.,
        Color::new(accent.r, accent.g, accent.b, 0.23),
    );
    let mut by = r.y + 68.;
    for line in &lines {
        if line.is_empty() {
            by += 10.;
        } else {
            ui::draw_text(
                line,
                r.x + 22.,
                by,
                18.,
                if style == 2 {
                    Color::from_rgba(202, 221, 201, 255)
                } else {
                    UiTheme.text()
                },
            );
            by += 23.;
        }
    }
    let button = Rect::new(r.right() - 150., r.bottom() - 36., 128., 26.);
    UiTheme.dialogue_action(button, "Fermer  [Échap]", false, true);
    let original: Vec<_> = text.split_whitespace().collect();
    let rendered: Vec<_> = lines.iter().flat_map(|s| s.split_whitespace()).collect();
    if original != rendered || by > button.y + 2. {
        return Err("Texte de dialogue tronqué ou superposé".into());
    }
    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn consulted_quest_notification_stays_hidden_when_dialogue_closes() {
        use project_rl::world::GridPos;
        let officer = GridPos::new(6, 7);
        let other = GridPos::new(18, 13);
        assert!(!notification_hidden(officer, true));
        {
            let _review = begin(officer);
            assert!(active());
            assert!(notification_hidden(officer, true));
            assert!(notification_hidden(officer, false));
            assert!(!notification_hidden(other, true));
            set_open(false);
            assert!(!active());
            assert!(notification_hidden(officer, true));
            assert!(!notification_hidden(officer, false));
            assert!(quest_notification_hidden(
                officer,
                project_rl::game::QuestMarker::Available
            ));
            assert!(!quest_notification_hidden(
                officer,
                project_rl::game::QuestMarker::ReadyToComplete
            ));
            set_ready();
            assert_eq!(
                reviewed_quest_marker(officer),
                Some(project_rl::game::QuestMarker::ReadyToComplete)
            );
            assert_eq!(reviewed_quest_marker(other), None);
            set_open(true);
            assert!(notification_hidden(officer, true));
            assert!(quest_notification_hidden(
                officer,
                project_rl::game::QuestMarker::ReadyToComplete
            ));
        }
        assert!(!notification_hidden(officer, true));
    }
    #[test]
    fn wrapping_preserves_every_word_and_paragraph_without_ellipsis() {
        let text = crate::cryogenic_camp::OFFICER_DIALOGUE;
        for width in [20., 50., 80.] {
            let lines = wrapped(text, width, |s| s.chars().count() as f32);
            assert_eq!(
                lines
                    .iter()
                    .flat_map(|s| s.split_whitespace())
                    .collect::<Vec<_>>(),
                text.split_whitespace().collect::<Vec<_>>()
            );
            assert_eq!(lines.iter().filter(|s| s.is_empty()).count(), 2);
        }
    }

    #[test]
    fn dismissal_inside_outside_or_escape_consumes_the_whole_input_frame() {
        use crate::controls::{Binding, InputFrame};
        for (pointer, pause) in [
            (Some((600., 160.)), false),
            (Some((40., 800.)), false),
            (None, true),
        ] {
            let mut input = InputFrame {
                pointer,
                pause,
                pressed: if pause {
                    [Binding::key("Escape"), Binding::key("F")].into()
                } else {
                    [Binding::MouseLeft, Binding::key("F")].into()
                },
                held: [Binding::key("W")].into(),
                mouse_left_down: !pause,
                ..Default::default()
            };
            assert!(take_dismiss(&mut input));
            assert!(
                input.pressed.is_empty()
                    && input.held.is_empty()
                    && !input.pause
                    && !input.mouse_left_down
            );
            assert!(!take_dismiss(&mut input));
        }
        let mut input = InputFrame::default();
        assert!(!take_dismiss(&mut input));
    }

    #[test]
    fn writing_is_unicode_safe_and_blinks_only_after_completion() {
        let text = "Détenu.\nŒil";
        assert_eq!(reveal(text, Some(0.), false).text, "");
        assert_eq!(reveal(text, Some(0.05), false).text, "Dé");
        let end = text.chars().count() as f64 / CHARACTERS_PER_SECOND;
        let complete = reveal(text, Some(end + 0.1), false);
        assert_eq!(complete.text, text);
        assert!(complete.cursor_on);
        assert!(!reveal(text, Some(end + 0.6), false).cursor_on);
        assert!(reveal(text, Some(end + 1.1), false).cursor_on);
        let reduced = reveal(text, Some(0.), true);
        assert_eq!(reduced.text, text);
        assert!(reduced.cursor_on);
    }
}
