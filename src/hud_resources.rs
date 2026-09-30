//! The same resolved resource values feed both HUD layouts.
use super::*;

pub(super) struct ResourceGauge {
    pub label: &'static str,
    pub value: String,
    pub ratio: f32,
    pub color: Color,
    pub reserved_ratio: f32,
    pub companion_slots: Option<(usize, u16)>,
}

impl AsciiApp {
    pub(super) fn objective_panel_text(&self) -> Option<String> {
        let objective = self.primary_objective()?;
        if objective == "OBJECTIF · REJOINDRE LE SECTEUR HABITÉ" {
            return Some("Rejoindre le secteur habité.".into());
        }
        if objective == "OBJECTIF · PARLER À L'HABITANT DE LA PLACE" {
            return Some("Parler à l'habitant de la place.".into());
        }
        let journal = self.game.quest_journal();
        let entry = self
            .ux
            .tracked_quest
            .as_ref()
            .and_then(|id| {
                journal
                    .iter()
                    .find(|e| &e.quest.id == id && e.quest.status != QuestStatus::Completed)
            })
            .or_else(|| {
                journal
                    .iter()
                    .find(|e| e.quest.status == QuestStatus::ReadyToComplete)
            })
            .or_else(|| {
                journal
                    .iter()
                    .find(|e| e.quest.status == QuestStatus::Active)
            });
        if !self.test_lab
            && let Some(entry) = entry
        {
            return Some(if entry.quest.status == QuestStatus::ReadyToComplete {
                format!("Retourner parler à {}.", self.quest_giver_name(entry))
            } else {
                self.quest_objective_text(&entry.quest.objective)
            });
        }
        Some(
            objective
                .trim_start_matches("LABORATOIRE · ")
                .trim_start_matches("OBJECTIF · ")
                .trim_start_matches("Objectif · ")
                .to_owned(),
        )
    }

    pub(super) fn draw_objective_panel(&self) {
        let scale = self.ui_scale();
        let physical =
            crate::terminal_view::terminal_objective_panel(self.terminal_bounds(), scale);
        let panel = Rect::new(
            physical.x / scale,
            physical.y / scale,
            physical.w / scale,
            physical.h / scale,
        );
        self.ux.hud_objective.set(Some(panel));
        UiTheme.hud_chip(panel);
        let x = panel.x + 12.0;
        let width = panel.w - 24.0;
        draw_text_bold("OBJECTIF", x, panel.y + 22.0, 13.0, UiTheme.accent());
        draw_wrapped_text(
            &self
                .objective_panel_text()
                .unwrap_or_else(|| "Aucun objectif suivi.".into()),
            x,
            panel.y + 46.0,
            width,
            4,
            15,
            UiTheme.text(),
        );
        if !self.test_lab {
            crate::ui_theme::draw_text_in_rect(
                format!("Journal [{}]", self.controls.label(Action::QuestJournal)),
                Rect::new(x, panel.bottom() - 26.0, width, 18.0),
                12,
                if self.menu_focus.hovered == Some(50_023) {
                    UiTheme.accent()
                } else {
                    UiTheme.muted()
                },
            );
        }
    }

    pub(super) fn visible_health_bars(&self, now: f64) -> Vec<(EntityId, GridPos, u16, u16)> {
        self.game
            .actors()
            .iter()
            .filter_map(|(id, actor)| {
                let recent = self
                    .ux
                    .recent_damage
                    .get(&id)
                    .is_some_and(|time| (0.0..3.0).contains(&(now - time)));
                (self.game.player_visibility().is_visible(actor.position())
                    && (self.selected_target == Some(id) || recent))
                    .then_some((
                        id,
                        actor.position(),
                        actor.integrity(),
                        actor.maximum_integrity(),
                    ))
            })
            .collect()
    }

    pub(super) fn draw_world_health_bars(&self, navigation: bool) {
        for (id, at, hp, maximum) in self.visible_health_bars(get_time()) {
            let Some(cell) = self.terminal.world_cell_rect(
                &self.game,
                self.terminal_bounds(),
                self.ui_scale(),
                self.graphics.active.world_cell_px,
                navigation,
                at,
            ) else {
                continue;
            };
            let width = (cell.w * 0.82).clamp(8.0, 34.0);
            let badge_rows = self.effect_badges_at(at).len().div_ceil(4) as f32;
            let bar = Rect::new(
                cell.x + (cell.w - width) * 0.5,
                cell.y - badge_rows * 15.0 - 7.0,
                width,
                4.0,
            );
            draw_rectangle(
                bar.x - 1.0,
                bar.y - 1.0,
                bar.w + 2.0,
                bar.h + 2.0,
                Color::from_rgba(3, 9, 13, 240),
            );
            draw_rectangle(
                bar.x,
                bar.y,
                bar.w,
                bar.h,
                Color::from_rgba(39, 52, 59, 255),
            );
            let color = if id == self.game.player_id() {
                UiTheme.success()
            } else if self
                .game
                .actors()
                .get(id)
                .is_some_and(|actor| actor.player_relation() != PlayerRelation::Hostile)
            {
                UiTheme.accent()
            } else {
                UiTheme.danger()
            };
            draw_rectangle(
                bar.x,
                bar.y,
                bar.w * normalized_ratio(hp, maximum),
                bar.h,
                color,
            );
            draw_line(
                bar.x,
                bar.y,
                bar.x + bar.w * normalized_ratio(hp, maximum),
                bar.y,
                1.0,
                Color::new(1.0, 1.0, 1.0, 0.28),
            );
        }
    }

    pub(super) fn player_defense_values(&self) -> Vec<(&'static str, String)> {
        let id = self.game.player_id();
        let mut values = vec![
            (
                "Armure",
                self.game
                    .actor_armor_profile(id)
                    .map_or(0, ArmorProfile::after_fragilization)
                    .to_string(),
            ),
            (
                "Esquive",
                self.game
                    .actor_evasion(id)
                    .map_or("n/d".into(), |v| v.to_string()),
            ),
            (
                "Stabilité",
                self.game
                    .actor_stability(id)
                    .map_or("n/d".into(), |v| v.to_string()),
            ),
            (
                "Défense numérique",
                self.game
                    .actor_digital_defense(id)
                    .map_or("n/d".into(), |v| v.to_string()),
            ),
        ];
        if let Some(player) = self.game.actors().get(id) {
            for (label, kind) in [
                ("Thermique", DamageType::Thermal),
                ("Électrique", DamageType::Electrical),
                ("Chimique", DamageType::Chemical),
                ("Radiation", DamageType::Radiation),
                ("Corruption", DamageType::Corruption),
            ] {
                values.push((label, format!("{} %", player.resistances().get(kind))));
            }
        }
        values
    }

    fn draw_defense_values(&self, rect: Rect, compact: bool) {
        let theme = UiTheme;
        theme.hud_chip(rect);
        crate::ui_theme::draw_text_in_rect(
            "DÉFENSES",
            Rect::new(rect.x + 10.0, rect.y + 7.0, rect.w - 20.0, 16.0),
            12,
            theme.muted(),
        );
        let values = self.player_defense_values();
        let columns = if compact { 3 } else { 1 };
        let rows = values.len().div_ceil(columns);
        let pitch = ((rect.h - 30.0) / rows as f32).min(24.0);
        let width = (rect.w - 20.0) / columns as f32;
        for (i, (label, value)) in values.iter().enumerate() {
            let x = rect.x + 10.0 + (i % columns) as f32 * width;
            let y = rect.y + 27.0 + (i / columns) as f32 * pitch;
            crate::ui_theme::draw_text_in_rect(
                label,
                Rect::new(x, y, width - 44.0, pitch - 2.0),
                if compact { 12 } else { 14 },
                theme.text(),
            );
            crate::ui_theme::draw_text_in_rect(
                value,
                Rect::new(x + width - 42.0, y, 38.0, pitch - 2.0),
                if compact { 12 } else { 14 },
                theme.accent(),
            );
        }
    }

    pub(super) fn resource_gauges(&self) -> Vec<ResourceGauge> {
        let theme = UiTheme;
        let mut gauges = Vec::new();
        if let Some(player) = self.game.actors().get(self.game.player_id()) {
            gauges.push(ResourceGauge {
                reserved_ratio: 0.0,
                companion_slots: None,
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
            reserved_ratio: normalized_ratio(energy.reserved(), energy.capacity()),
            companion_slots: None,
            label: "Énergie",
            value: if self.game.rules().maintained_energy_reservations {
                format!(
                    "{} libres · {} réservés",
                    energy.available(),
                    energy.reserved()
                )
            } else {
                format!("{} / {}", energy.available(), energy.capacity())
            },
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
                reserved_ratio: 0.0,
                companion_slots: None,
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
                reserved_ratio: 0.0,
                companion_slots: None,
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
                reserved_ratio: 0.0,
                companion_slots: None,
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
        if let Some(heat) = self.game.player_heat().filter(|_| {
            !self.game.rules().maintained_energy_reservations
                || !self.overclock_resource_lines().is_empty()
        }) {
            gauges.push(ResourceGauge {
                reserved_ratio: 0.0,
                companion_slots: None,
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
        if let Some(maximum) = self.game.rules().player_companion_limit {
            let count = self.game.player_companion_count();
            gauges.push(ResourceGauge {
                label: "Compagnons",
                value: format!("{count} / {maximum}"),
                ratio: count as f32 / f32::from(maximum.max(1)),
                color: UiTheme.accent(),
                reserved_ratio: 0.0,
                companion_slots: Some((count, maximum)),
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
        let bar = Rect::new(rect.x, rect.bottom() - 7.0, rect.w, 7.0);
        if let Some((occupied, maximum)) = gauge.companion_slots {
            let gap = 4.0;
            let width =
                (bar.w - gap * f32::from(maximum.saturating_sub(1))) / f32::from(maximum.max(1));
            for index in 0..maximum {
                draw_rectangle(
                    bar.x + f32::from(index) * (width + gap),
                    bar.y,
                    width,
                    bar.h,
                    if usize::from(index) < occupied {
                        gauge.color
                    } else {
                        Color::from_rgba(44, 49, 59, 255)
                    },
                );
            }
            return;
        }
        draw_rectangle(
            bar.x,
            bar.y,
            bar.w,
            bar.h,
            Color::from_rgba(44, 49, 59, 255),
        );
        draw_rectangle(
            bar.x,
            bar.y,
            bar.w * gauge.ratio.clamp(0.0, 1.0),
            bar.h,
            gauge.color,
        );
        let reserved_width = bar.w * gauge.reserved_ratio.clamp(0.0, 1.0);
        if reserved_width > 0.0 {
            let left = bar.right() - reserved_width;
            draw_rectangle(
                left,
                bar.y,
                reserved_width,
                bar.h,
                Color::from_rgba(179, 132, 235, 255),
            );
            let mut stripe = left + 2.0;
            while stripe < bar.right() {
                let end = (stripe + bar.h).min(bar.right());
                draw_line(
                    stripe,
                    bar.bottom(),
                    end,
                    bar.bottom() - (end - stripe),
                    1.0,
                    Color::from_rgba(76, 45, 116, 255),
                );
                stripe += 8.0;
            }
        }
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

    fn draw_hud_resource_gauge(&self, rect: Rect, gauge: &ResourceGauge) {
        match gauge.label {
            "Énergie" => self.ux.hud_energy.set(Some(rect)),
            "Compagnons" => self.ux.hud_companions.set(Some(rect)),
            _ => {}
        }
        Self::draw_resource_gauge(rect, gauge);
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
            self.draw_hud_resource_gauge(Rect::new(x, y, width, 38.0), &gauge);
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
        let defense_top = bottom + 29.0;
        self.draw_defense_values(
            Rect::new(x, defense_top, width, panel.bottom() - 108.0 - defense_top),
            false,
        );
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
            self.draw_hud_resource_gauge(
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
        let defense_width = (self.ui_width() - 30.0) * 0.66;
        self.draw_defense_values(Rect::new(12.0, 105.0, defense_width, 88.0), true);
        let target_rect = Rect::new(
            18.0 + defense_width,
            105.0,
            self.ui_width() - defense_width - 30.0,
            88.0,
        );
        theme.hud_panel(target_rect);
        let summary = self.terminal_target_summary();
        crate::ui_theme::draw_text_in_rect(
            summary.as_ref().map_or("Aucune cible", |s| s.name.as_str()),
            Rect::new(
                target_rect.x + 10.0,
                target_rect.y + 8.0,
                target_rect.w - 20.0,
                20.0,
            ),
            14,
            theme.text(),
        );
        if let Some(summary) = summary {
            let details = summary.interaction.clone().unwrap_or_else(|| {
                if let Some(analysis) = &summary.analysis {
                    format!(
                        "PV {} / {} · Armure {}",
                        analysis.integrity, analysis.maximum_integrity, analysis.armor
                    )
                } else if !summary.visible_state.is_empty()
                    && summary.visible_state != "Aucun état visible"
                {
                    summary.visible_state.clone()
                } else {
                    format!(
                        "À {} case{}",
                        summary.distance,
                        if summary.distance > 1 { "s" } else { "" }
                    )
                }
            });
            crate::ui_theme::draw_text_in_rect(
                details,
                Rect::new(
                    target_rect.x + 10.0,
                    target_rect.y + 32.0,
                    target_rect.w - 20.0,
                    20.0,
                ),
                12,
                theme.muted(),
            );
            if let Some((hp, max)) = summary.health {
                draw_rectangle(
                    target_rect.x + 10.0,
                    target_rect.bottom() - 18.0,
                    target_rect.w - 20.0,
                    5.0,
                    theme.surface(),
                );
                draw_rectangle(
                    target_rect.x + 10.0,
                    target_rect.bottom() - 18.0,
                    (target_rect.w - 20.0) * normalized_ratio(hp, max),
                    5.0,
                    theme.success(),
                );
            }
        }
    }
}
