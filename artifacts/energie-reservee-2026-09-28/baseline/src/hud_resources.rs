//! The same resolved resource values feed both HUD layouts.
use super::*;

pub(super) struct ResourceGauge {
    pub label: &'static str,
    pub value: String,
    pub ratio: f32,
    pub color: Color,
}

impl AsciiApp {
    pub(super) fn resource_gauges(&self) -> Vec<ResourceGauge> {
        let theme = UiTheme;
        let mut gauges = Vec::new();
        if let Some(player) = self.game.actors().get(self.game.player_id()) {
            gauges.push(ResourceGauge {
                label: "PV",
                value: format!("{} / {}", player.integrity(), player.maximum_integrity()),
                ratio: normalized_ratio(player.integrity(), player.maximum_integrity()),
                color: if player.integrity().saturating_mul(3) < player.maximum_integrity() {
                    theme.danger()
                } else {
                    theme.success()
                },
            });
        }
        let energy = self.game.player_energy();
        gauges.push(ResourceGauge {
            label: "Énergie",
            value: format!("{} / {}", energy.available(), energy.capacity()),
            ratio: normalized_ratio(energy.available(), energy.capacity()),
            color: if energy.available() == 0 {
                theme.danger()
            } else {
                Color::from_rgba(109, 186, 255, 255)
            },
        });
        if let Some(stock) = self.game.player_matter() {
            let scale = stock.div_ceil(40).max(1).saturating_mul(40);
            gauges.push(ResourceGauge {
                label: "Munitions",
                value: stock.to_string(),
                ratio: stock as f32 / scale as f32,
                color: if stock == 0 {
                    theme.danger()
                } else {
                    theme.accent()
                },
            });
        } else if let Some((left, capacity)) = self
            .game
            .equipped_player_weapon(self.active_weapon_slot)
            .and_then(|weapon| self.game.player_weapon_ammunition(weapon.id()))
        {
            gauges.push(ResourceGauge {
                label: "Munitions",
                value: format!("{left} / {capacity}"),
                ratio: normalized_ratio(left, capacity),
                color: if left == 0 {
                    theme.danger()
                } else {
                    theme.accent()
                },
            });
        }
        if let Some(bandwidth) = self.game.player_bandwidth() {
            gauges.push(ResourceGauge {
                label: "Bande passante",
                value: format!(
                    "{} / {} occupés",
                    bandwidth.occupied(),
                    bandwidth.capacity()
                ),
                ratio: normalized_ratio(bandwidth.occupied(), bandwidth.capacity()),
                color: if bandwidth.available() == 0 {
                    theme.attention()
                } else {
                    theme.accent()
                },
            });
        }
        if let Some(heat) = self.game.player_heat() {
            gauges.push(ResourceGauge {
                label: "Chaleur",
                value: format!(
                    "{} / {} critique",
                    heat.current(),
                    heat.critical_threshold()
                ),
                ratio: normalized_ratio(heat.current(), heat.critical_threshold()),
                color: if heat.current() >= heat.critical_threshold() {
                    theme.danger()
                } else if heat.current() >= heat.alert_threshold() {
                    theme.attention()
                } else {
                    theme.muted()
                },
            });
        }
        gauges
    }

    pub(super) fn compact_resource_rects(width: f32, count: usize) -> Vec<Rect> {
        let gap = 6.0;
        let w = (width - 24.0 - gap * count.saturating_sub(1) as f32) / count.max(1) as f32;
        (0..count)
            .map(|i| Rect::new(12.0 + i as f32 * (w + gap), 7.0, w, 56.0))
            .collect()
    }

    pub(super) fn draw_resource_gauge(rect: Rect, gauge: &ResourceGauge) {
        crate::ui_theme::draw_text_in_rect(
            gauge.label,
            Rect::new(rect.x, rect.y, rect.w, 16.0),
            14,
            UiTheme.text(),
        );
        crate::ui_theme::draw_text_in_rect(
            &gauge.value,
            Rect::new(rect.x, rect.y + 17.0, rect.w, 15.0),
            13,
            gauge.color,
        );
        let bar = Rect::new(rect.x, rect.bottom() - 5.0, rect.w, 5.0);
        draw_rectangle(
            bar.x,
            bar.y,
            bar.w,
            bar.h,
            Color::from_rgba(32, 59, 66, 255),
        );
        draw_rectangle(
            bar.x,
            bar.y,
            bar.w * gauge.ratio.clamp(0.0, 1.0),
            bar.h,
            gauge.color,
        );
    }

    pub(super) fn active_weapon_hud_label(&self) -> String {
        self.game
            .equipped_player_weapon(self.active_weapon_slot)
            .map(|weapon| {
                self.equipped_instance_name(self.active_weapon_slot)
                    .unwrap_or_else(|| self.item_name(weapon.id()))
            })
            .unwrap_or_else(|| "Aucune arme".to_owned())
    }

    pub(super) fn active_weapon_hud_cost(&self) -> String {
        let mut parts = Vec::new();
        if let Some(cost) = self
            .game
            .equipped_player_weapon(self.active_weapon_slot)
            .and_then(|w| self.weapon_supply_label(w.id()))
        {
            parts.push(cost);
        }
        if let Some(ready) = self.active_effect_readiness() {
            parts.push(ready);
        }
        if let Some(turns) = self
            .game
            .actors()
            .get(self.game.player_id())
            .and_then(Actor::recovery_remaining)
        {
            parts.push(format!("Récupération : {}", turns.get()));
        }
        parts.join(" · ")
    }

    pub(super) fn draw_resource_sidebar(&self, panel: Rect) {
        let theme = UiTheme;
        theme.hud_panel(panel);
        let x = panel.x + 14.0;
        let width = panel.w - 28.0;
        let mut y = panel.y + 12.0;
        for gauge in self.resource_gauges() {
            Self::draw_resource_gauge(Rect::new(x, y, width, 38.0), &gauge);
            y += 44.0;
        }
        draw_text_bold("Arme en main", x, y + 14.0, 14.0, theme.muted());
        let bottom = draw_wrapped_text(
            &self.active_weapon_hud_label(),
            x,
            y + 37.0,
            width,
            2,
            16,
            theme.text(),
        );
        crate::ui_theme::draw_text_in_rect(
            self.active_weapon_hud_cost(),
            Rect::new(x, bottom, width, 17.0),
            12,
            theme.muted(),
        );
        let defenses = Rect::new(x, panel.bottom() - 130.0, width, 32.0);
        theme.button(
            defenses,
            "Défenses et réserves",
            self.menu_focus.hovered == Some(50_021),
            false,
            true,
            ButtonTone::Secondary,
        );
        self.ux.hud_defenses.set(Some(defenses));
        let progression = self.game.player_progression();
        let available = progression.unspent_skill_points();
        let points = Rect::new(x, panel.bottom() - 88.0, width, 32.0);
        theme.button(
            points,
            &format!(
                "{available} point{} de compétence",
                if available > 1 { "s" } else { "" }
            ),
            self.menu_focus.hovered == Some(50_020),
            false,
            true,
            if available > 0 {
                ButtonTone::Primary
            } else {
                ButtonTone::Secondary
            },
        );
        self.ux.hud_points.set(Some(points));
        let level = progression.level();
        let curve = &self.game.rules().progression.curve;
        let next = curve.next_threshold_after(level);
        let previous = curve
            .cumulative_thresholds()
            .get(usize::from(level.saturating_sub(2)))
            .copied()
            .filter(|_| level > 1)
            .unwrap_or(0);
        let ratio = next.map_or(1.0, |next| {
            progression.experience().saturating_sub(previous) as f32
                / next.saturating_sub(previous).max(1) as f32
        });
        draw_status_bar(
            Rect::new(x, panel.bottom() - 43.0, width, 31.0),
            &format!("XP · niveau {level}"),
            &next.map_or_else(
                || "MAX".to_owned(),
                |next| format!("{}/{next}", progression.experience()),
            ),
            ratio,
            UiIcon::Level,
            theme.accent(),
        );
    }

    pub(super) fn draw_compact_resource_header(&self) {
        let theme = UiTheme;
        let gauges = self.resource_gauges();
        for (rect, gauge) in Self::compact_resource_rects(self.ui_width(), gauges.len())
            .into_iter()
            .zip(&gauges)
        {
            theme.hud_panel(rect);
            Self::draw_resource_gauge(
                Rect::new(rect.x + 9.0, rect.y + 6.0, rect.w - 18.0, 43.0),
                gauge,
            );
        }
        let points = Rect::new(12.0, 69.0, (self.ui_width() * 0.34).min(290.0), 30.0);
        let progression = self.game.player_progression();
        theme.button(
            points,
            &format!(
                "Niv. {} · {} point(s) [{}]",
                progression.level(),
                progression.unspent_skill_points(),
                self.controls.label(Action::Skills)
            ),
            self.menu_focus.hovered == Some(50_020),
            false,
            true,
            ButtonTone::Secondary,
        );
        self.ux.hud_points.set(Some(points));
        let weapon = Rect::new(
            points.right() + 8.0,
            points.y,
            self.ui_width() - points.right() - 20.0,
            points.h,
        );
        theme.hud_chip(weapon);
        let cost = self.active_weapon_hud_cost();
        let name = self.active_weapon_hud_label();
        crate::ui_theme::draw_text_in_rect(
            if cost.is_empty() {
                name
            } else {
                format!("{name} · {cost}")
            },
            Rect::new(
                weapon.x + 10.0,
                weapon.y + 4.0,
                weapon.w - 20.0,
                weapon.h - 8.0,
            ),
            14,
            theme.text(),
        );
    }
}
