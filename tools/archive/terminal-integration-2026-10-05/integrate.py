from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
def read(name): return (ROOT / 'src' / name).read_text(encoding='utf-8')
def write(name, s): (ROOT / 'src' / name).write_text(s, encoding='utf-8', newline='\r\n')
def replace(s, old, new):
    assert old in s, old[:160]
    return s.replace(old, new, 1)
def between(s, start, end, new):
    a=s.index(start); b=s.index(end, a)
    return s[:a]+new+s[b:]

s=read('main.rs'); s=replace(s,'mod terminal_view;', 'mod terminal_chrome;\nmod terminal_view;'); write('main.rs',s)

s=read('hud_resources.rs')
s=replace(s, '        UiTheme.hud_chip(panel);', '        crate::terminal_chrome::module(panel, "07 / OBJECTIF");')
s=replace(s, '        draw_text_bold("OBJECTIF", x, panel.y + 22.0, 13.0, UiTheme.accent());\n', '')
s=replace(s, '            panel.y + 46.0,', '            panel.y + 49.0,')
s=replace(s, '        theme.hud_chip(rect);', '        crate::terminal_chrome::module(rect, "03 / DÉFENSES");')
s=between(s, '        crate::ui_theme::draw_text_in_rect(\n            "DÉFENSES",', '        let values = self.player_defense_values();', '')
# The HUD hides unused overclock heat; the inventory still shows its heat state.
s=between(s, '        if let Some(heat) = self.game.player_heat().filter', '        if let Some(maximum) = self.game.rules().player_companion_limit', '''        if (!self.game.rules().maintained_energy_reservations
            || !self.overclock_resource_lines().is_empty())
            && let Some(gauge) = self.heat_resource_gauge()
        {
            gauges.push(gauge);
        }
''')
pos=s.index('    pub(super) fn compact_resource_rects')
s=s[:pos]+'''    pub(super) fn heat_resource_gauge(&self) -> Option<ResourceGauge> {
        let heat = self.game.player_heat()?;
        Some(ResourceGauge {
            label: "Chaleur",
            value: format!("{} / {} critique", heat.current(), heat.critical_threshold()),
            ratio: normalized_ratio(heat.current(), heat.critical_threshold()),
            color: if heat.current() >= heat.critical_threshold() {
                UiTheme.danger()
            } else if heat.current() >= heat.alert_threshold() {
                UiTheme.attention()
            } else {
                UiTheme.muted()
            },
            reserved_ratio: 0.0,
            companion_slots: None,
        })
    }

    pub(super) fn experience_resource_gauge(&self) -> ResourceGauge {
        let progression = self.game.player_progression();
        let level = progression.level();
        let curve = &self.game.rules().progression.curve;
        let next = curve.next_threshold_after(level);
        let previous = if level > 1 {
            curve.next_threshold_after(level - 1).unwrap_or(0)
        } else { 0 };
        ResourceGauge {
            label: "Expérience",
            value: next.map_or_else(|| format!("Niveau {level} · MAX"),
                |next| format!("Niveau {level} · {} / {}",
                    progression.experience().saturating_sub(previous),
                    next.saturating_sub(previous))),
            ratio: next.map_or(1.0, |next| {
                progression.experience().saturating_sub(previous) as f32
                    / next.saturating_sub(previous).max(1) as f32
            }),
            color: UiTheme.accent(),
            reserved_ratio: 0.0,
            companion_slots: None,
        }
    }

'''+s[pos:]
s=replace(s, '56.0))', '76.0))')
# Segment the available portion; keep the reserved-energy hatching distinct.
s=replace(s, '        let reserved_width = bar.w * gauge.reserved_ratio.clamp(0.0, 1.0);',
    '        crate::terminal_chrome::segment(bar, 20);\n        let reserved_width = bar.w * gauge.reserved_ratio.clamp(0.0, 1.0);')
s=between(s, '    pub(super) fn draw_resource_sidebar', '    pub(super) fn draw_compact_resource_header', '''    pub(super) fn draw_resource_sidebar(&self, panel: Rect) {
        use crate::terminal_chrome::{module, button};
        let gauges = self.resource_gauges();
        let weapon_h = if panel.h < 640.0 { 80.0 } else { 94.0 };
        let progression_h = if panel.h < 640.0 { 88.0 } else { 92.0 };
        let pitch = ((panel.h - weapon_h - progression_h - 24.0 - 145.0 - 41.0)
            / gauges.len().max(1) as f32).clamp(38.0, 46.0);
        let state = Rect::new(panel.x, panel.y, panel.w, 41.0 + pitch * gauges.len() as f32);
        module(state, "01 / ÉTAT DU PERSONNAGE");
        for (i, gauge) in gauges.iter().enumerate() {
            self.draw_hud_resource_gauge(
                Rect::new(panel.x + 13.0, panel.y + 38.0 + i as f32 * pitch, panel.w - 26.0, 38.0), gauge);
        }
        let weapon = Rect::new(panel.x, state.bottom() + 8.0, panel.w, weapon_h);
        module(weapon, "02 / ARME ACTIVE");
        let bottom = draw_wrapped_text(&self.active_weapon_hud_label(), weapon.x + 13.0,
            weapon.y + 48.0, weapon.w - 26.0, 2, 15, UiTheme.text());
        crate::ui_theme::draw_text_in_rect(self.active_weapon_hud_cost(),
            Rect::new(weapon.x + 13.0, bottom, weapon.w - 26.0, (weapon.bottom() - bottom - 5.0).max(12.0)),
            12, UiTheme.muted());
        let progress = Rect::new(panel.x, panel.bottom() - progression_h, panel.w, progression_h);
        let defenses = Rect::new(panel.x, weapon.bottom() + 8.0, panel.w, progress.y - weapon.bottom() - 16.0);
        module(defenses, "03 / DÉFENSES");
        let values = self.player_defense_values();
        let row_h = ((defenses.h - 36.0) / values.len() as f32).min(20.0);
        let size = if row_h < 14.0 { 11 } else { 13 };
        for (i, (label, value)) in values.iter().enumerate() {
            let y = defenses.y + 33.0 + i as f32 * row_h;
            crate::ui_theme::draw_text_in_rect(label,
                Rect::new(defenses.x + 13.0, y, defenses.w - 77.0, row_h), size, UiTheme.text());
            crate::ui_theme::draw_text_in_rect(value,
                Rect::new(defenses.right() - 59.0, y, 46.0, row_h), size, UiTheme.accent());
        }
        module(progress, "04 / PROGRESSION");
        Self::draw_resource_gauge(Rect::new(progress.x + 13.0, progress.y + 33.0, progress.w - 26.0, 38.0),
            &self.experience_resource_gauge());
        let points = Rect::new(progress.x + 13.0, progress.bottom() - 18.0, progress.w - 26.0, 15.0);
        button(points, self.menu_focus.hovered == Some(50_020), false, true);
        crate::ui_theme::draw_text_in_rect(format!("{} point(s) · Compétences [{}]",
            self.game.player_progression().unspent_skill_points(), self.controls.label(Action::Skills)),
            Rect::new(points.x + 4.0, points.y, points.w - 8.0, points.h), 11, UiTheme.accent());
        self.ux.hud_points.set(Some(points));
    }

''')
# Preserve compact mode's 204px top reservation and its inspection summary.
s=replace(s, '            theme.hud_panel(rect);', '            crate::terminal_chrome::module(rect, &format!("{:02} / {}", gauges.iter().position(|g| g.label == gauge.label).unwrap_or(0) + 1, gauge.label.to_uppercase()));')
s=replace(s, 'Rect::new(rect.x + 9.0, rect.y + 6.0, rect.w - 18.0, 43.0)', 'Rect::new(rect.x + 9.0, rect.y + 32.0, rect.w - 18.0, 38.0)')
s=replace(s, 'Rect::new(12.0, 69.0, (self.ui_width() * 0.34).min(290.0), 30.0)', 'Rect::new(12.0, 89.0, (self.ui_width() * 0.34).min(290.0), 30.0)')
s=between(s, '        theme.button(\n            points,', '        self.ux.hud_points.set(Some(points));', '''        inventory_terminal::terminal_button(points,
            &format!("Niv. {} · {} point(s) [{}]", progression.level(),
                progression.unspent_skill_points(), self.controls.label(Action::Skills)),
            self.menu_focus.hovered == Some(50_020), false, true);
''')
s=replace(s, '        theme.hud_chip(weapon);', '        crate::terminal_chrome::button(weapon, false, false, true);')
s=replace(s, 'Rect::new(12.0, 105.0, defense_width, 88.0)', 'Rect::new(12.0, 125.0, defense_width, 68.0)')
s=replace(s, '            105.0,\n            self.ui_width() - defense_width - 30.0,\n            88.0,', '            125.0,\n            self.ui_width() - defense_width - 30.0,\n            68.0,')
s=replace(s, '        theme.hud_panel(target_rect);', '        crate::terminal_chrome::module(target_rect, "06 / CIBLE");')
s=replace(s, '                target_rect.y + 8.0,', '                target_rect.y + 29.0,')
s=replace(s, '                    target_rect.y + 32.0,', '                    target_rect.y + 47.0,')
s=replace(s, 'target_rect.bottom() - 18.0', 'target_rect.bottom() - 7.0')
s=s.replace('let pitch = ((rect.h - 30.0) / rows as f32).min(24.0);', 'let pitch = ((rect.h - 32.0) / rows as f32).min(24.0);')
s=s.replace('rect.y + 27.0 + (i / columns)', 'rect.y + 30.0 + (i / columns)')
write('hud_resources.rs',s)

s=read('inventory_terminal.rs')
s=between(s, 'const PHOSPHOR:', 'const SLOT_LABELS:', '''fn phosphor() -> Color { UiTheme.accent() }
fn dim() -> Color { UiTheme.muted() }
fn ink() -> Color { crate::terminal_chrome::ink() }
''')
s=s.replace('PHOSPHOR','phosphor()').replace('DIM','dim()').replace('INK','ink()')
s=between(s, '    draw_rectangle(\n        rect.x,', '    let color = if enabled', '    crate::terminal_chrome::button(rect, focused, active, enabled);\n')
s=between(s, '    pub(super) fn draw_terminal_hud_frame', '    pub(super) fn preview_selected_inventory_item', '')
s=between(s, '        crate::ui_theme::draw_text_in_rect(\n            format!("PERSONNAGE /', '        crate::game_art::draw_terminal_wireframe(', '''        crate::terminal_chrome::module(equipment.panel,
            &format!("02 / PERSONNAGE / {}", class.to_uppercase()));
''')
s=replace(s, '                    Color::new(0.065, 0.12, 0.095, 1.0)', '                    UiTheme.surface_selected()')
s=between(s, '    pub(super) fn draw_inventory_gauges', '    pub(super) fn draw_inventory_tooltip', '''    pub(super) fn inventory_resource_gauges(&self) -> Vec<hud_resources::ResourceGauge> {
        let mut gauges: Vec<_> = self.resource_gauges().into_iter()
            .filter(|g| matches!(g.label, "PV" | "Énergie")).collect();
        if let Some(heat) = self.heat_resource_gauge() { gauges.push(heat); }
        gauges.push(self.experience_resource_gauge());
        gauges
    }

    pub(super) fn draw_inventory_gauges(&self, panel: Rect) {
        let state = Rect::new(panel.x + 8.0, panel.y, panel.w - 16.0, 235.0);
        crate::terminal_chrome::module(state, "03 / ÉTAT DU PERSONNAGE");
        for (i, gauge) in self.inventory_resource_gauges().iter().enumerate() {
            Self::draw_resource_gauge(
                Rect::new(panel.x + 20.0, panel.y + 39.0 + i as f32 * 42.0, panel.w - 40.0, 38.0), gauge);
        }
        crate::ui_theme::draw_text_in_rect(format!("{} points de compétence",
            self.game.player_progression().unspent_skill_points()),
            Rect::new(panel.x + 20.0, panel.y + 211.0, panel.w - 40.0, 18.0), 13, dim());
    }

''')
s=replace(s, '        draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, phosphor());', '        crate::terminal_chrome::bezel(rect);')
s=replace(s, '40, 15, phosphor())', '40, 15, UiTheme.text())')
write('inventory_terminal.rs',s)

s=read('ascii_app.rs')
s=replace(s, '            if !self.inventory_open {\n                self.draw_terminal_hud_frame();\n            }\n', '')
a=s.index('    fn draw_inventory(&self)'); b=s.index('\n    fn ',a+10)
chunk=s[a:b]
chunk=replace(chunk,'let cyan = Color::from_rgba(125, 224, 178, 255);','let cyan = UiTheme.accent();')
chunk=replace(chunk,'let muted = Color::from_rgba(90, 143, 119, 255);','let muted = UiTheme.muted();')
chunk=replace(chunk,'let separator = Color::from_rgba(43, 76, 84, 255);','let separator = crate::terminal_chrome::tint(0.32);')
chunk=replace(chunk,'Color::from_rgba(4, 10, 9, 255)', 'crate::terminal_chrome::ink()')
chunk=replace(chunk,'        draw_rectangle_lines(margin, top, width, height, 1.0, muted);', '''        draw_rectangle(margin + 1.0, top + 1.0, width - 2.0, 57.0, crate::terminal_chrome::tint(0.085));
        crate::terminal_chrome::bezel(Rect::new(margin, top, width, height));''')
s=s[:a]+chunk+s[b:]
s=replace(s, '    UiTheme.hud_action(rect, hover_strength);', '    crate::terminal_chrome::button(rect, hover_strength > 0.0, false, true);')
s=replace(s, 'Color::new(0.025, 0.065, 0.075, 0.98)', 'crate::terminal_chrome::tint(0.085)')
s=replace(s, '                &message,\n                20.0,\n                y,', '                &message,\n                40.0,\n                y,')
s=replace(s, '            let y = self.ui_height() - 42.0 + index as f32 * 20.0;', '''            let y = self.ui_height() - 42.0 + index as f32 * 20.0;
            crate::terminal_chrome::heading(">", 22.0, y, 14, UiTheme.accent());''')
write('ascii_app.rs',s)

s=read('ux.rs'); s=replace(s,'            UiTheme.hud_action(rect, if hovered { 1.0 } else { 0.0 });', '            crate::terminal_chrome::button(rect, hovered, false, true);'); write('ux.rs',s)

# Keep the old review entry point, now using the production renderer without a second overlay.
s=read('terminal_console_capture.rs'); idle=s[s.index('    pub(super) fn draw_idle_integration_review'):]
write('terminal_console_capture.rs', '''//! Diagnostic scenes use the production terminal and idle renderers.
use super::*;
use crate::terminal_chrome::{heading, module};

impl AsciiApp {
    pub(super) fn draw_terminal_console_proposal(&self) { self.draw(); }

'''+idle)
