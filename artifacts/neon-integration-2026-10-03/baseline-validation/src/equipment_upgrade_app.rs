//! Equipment improvement: a transactional service shared by all city artisans.
use super::*;
#[cfg(test)]
#[path = "equipment_upgrade_tests.rs"]
mod tests;

struct UpgradeLayout {
    panel: Rect,
    list: Rect,
    details: Rect,
    action: Rect,
    close: Rect,
    rows: usize,
}
impl UpgradeLayout {
    fn new(width: f32, height: f32) -> Self {
        let w = (width - 32.0).min(1080.0);
        let h = (height - 32.0).min(660.0);
        let panel = Rect::new((width - w) / 2.0, (height - h) / 2.0, w, h);
        let list = Rect::new(
            panel.x + 24.0,
            panel.y + 112.0,
            (w - 64.0) * 0.43,
            h - 250.0,
        );
        let details = Rect::new(list.x + list.w + 24.0, list.y, w - list.w - 72.0, list.h);
        let action = Rect::new(panel.x + w - 324.0, panel.y + h - 60.0, 180.0, 40.0);
        let close = Rect::new(panel.x + w - 132.0, action.y, 108.0, 40.0);
        Self {
            panel,
            list,
            details,
            action,
            close,
            rows: (list.h / 48.0).floor().max(1.0) as usize,
        }
    }
    fn first(&self, selected: usize) -> usize {
        selected.saturating_sub(self.rows - 1)
    }
    fn row(&self, index: usize) -> Rect {
        Rect::new(
            self.list.x,
            self.list.y + index as f32 * 48.0,
            self.list.w,
            44.0,
        )
    }
}

impl AsciiApp {
    #[cfg(debug_assertions)]
    pub(super) fn equipment_upgrade_capture_probes(&self) -> Vec<Rect> {
        let l = UpgradeLayout::new(self.ui_width(), self.ui_height());
        let mut probes = vec![
            Rect::new(l.panel.x + 24.0, l.panel.y + 12.0, l.panel.w - 48.0, 58.0),
            l.close,
            Rect::new(l.details.x, l.details.y, l.details.w, 80.0),
        ];
        if !self.npc_interaction_message.is_empty() {
            probes.push(Rect::new(
                l.panel.x + 24.0,
                l.action.y - 50.0,
                l.panel.w - 48.0,
                48.0,
            ));
        }
        probes
    }
    #[cfg(debug_assertions)]
    pub(super) fn prepare_equipment_upgrade_diagnostic(&mut self) -> Result<(), String> {
        let capacity = self.rules.player_inventory_capacity;
        self.rules.player_inventory_capacity = 32;
        let prepared = self.prepare_merchant_diagnostic_at_depth(Some(0));
        self.rules.player_inventory_capacity = capacity;
        prepared?;
        let provider = self
            .game
            .spawn_actor(
                Actor::new(GridPos::new(4, 2), 20)
                    .map_err(|e| e.to_string())?
                    .with_ai(AiProfile::idle())
                    .with_tags(["core:equipment_artisan".parse().unwrap()]),
            )
            .map_err(|e| e.to_string())?;
        for id in [
            "core:melee_hallebarde",
            "core:modele_brask_p12",
            "core:brigandine_cognefer",
            "core:calotte_cognefer",
        ] {
            self.game
                .grant_player_item_for_diagnostic(id.parse().unwrap(), 1)?;
        }
        self.game
            .grant_player_item_for_diagnostic("core:unstable_fragment".parse().unwrap(), 6)?;
        self.npc_interaction = Some(provider);
        self.npc_trade_selection = self
            .upgrade_items()
            .iter()
            .position(|id| {
                self.game
                    .player_inventory()
                    .get(*id)
                    .unwrap()
                    .item()
                    .as_str()
                    == "core:modele_brask_p12"
            })
            .unwrap();
        self.upgrade_confirmation = None;
        Ok(())
    }
    pub(super) fn ensure_local_artisan(&mut self) -> Result<(), String> {
        if self.generation_version < EQUIPMENT_UPGRADE_GENERATION_VERSION
            || self
                .game
                .actors()
                .iter()
                .any(|(id, _)| self.game.active_artisan(id))
        {
            return Ok(());
        }
        let Some(center) = self
            .game
            .actors()
            .iter()
            .find_map(|(id, a)| self.game.active_merchant(id).then_some(a.position()))
        else {
            return Ok(());
        };
        // An open, protected floor near the shop, outside doors and passage cells.
        let mut candidates = Vec::new();
        for y in 0..self.game.map().height() as i32 {
            for x in 0..self.game.map().width() as i32 {
                let at = GridPos::new(x, y);
                let distance = (x - center.x).abs() + (y - center.y).abs();
                if !(2..=8).contains(&distance)
                    || !self.game.map().is_protected(at)
                    || self.game.actors().entity_at(at).is_some()
                    || self.game.player_position() == Some(at)
                    || self.game.passage(at).is_some()
                    || !self
                        .game
                        .map()
                        .tile(at)
                        .is_some_and(|t| t.terrain == Terrain::Floor)
                {
                    continue;
                }
                let neighbors = [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
                    .into_iter()
                    .filter(|&(x, y)| self.game.map().is_walkable(GridPos::new(x, y)))
                    .count();
                if neighbors >= 3 {
                    candidates.push((distance, y, x));
                }
            }
        }
        candidates.sort();
        let mut navigation = self.game.map().clone();
        for y in 0..navigation.height() as i32 {
            for x in 0..navigation.width() as i32 {
                let at = GridPos::new(x, y);
                if navigation.tile(at).is_some_and(|t| {
                    matches!(
                        t.terrain,
                        Terrain::Door(project_rl::world::DoorState::Closed)
                    )
                }) {
                    navigation
                        .set_terrain(at, Terrain::Floor)
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        let origin = self.game.player_position().ok_or("Joueur absent")?;
        candidates.retain(|&(_, y, x)| {
            project_rl::world::find_path(&navigation, origin, GridPos::new(x, y), 15000, |_| true)
                .is_some()
        });
        let (_, y, x) = candidates
            .first()
            .copied()
            .ok_or("Aucune place protégée pour l'artisan")?;
        self.game
            .spawn_actor(
                Actor::new(GridPos::new(x, y), 20)
                    .map_err(|e| e.to_string())?
                    .with_ai(AiProfile::idle())
                    .with_tags(["core:equipment_artisan".parse().unwrap()]),
            )
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    fn upgrade_items(&self) -> Vec<ItemInstanceId> {
        let Some(rules) = &self.game.rules().equipment_upgrade else {
            return Vec::new();
        };
        self.game
            .player_inventory()
            .iter()
            .filter(|entry| {
                entry.quantity() == 1
                    && entry.owner().is_none()
                    && rules.catalog.iter().any(|(id, _)| id == entry.item())
            })
            .map(|entry| entry.instance())
            .collect()
    }

    pub(super) fn update_equipment_upgrade(&mut self, input: &InputFrame, provider: EntityId) {
        let (w, h) = input.viewport.unwrap_or((1280.0, 800.0));
        let layout = UpgradeLayout::new(w, h);
        let items = self.upgrade_items();
        self.npc_trade_selection = self.npc_trade_selection.min(items.len().saturating_sub(1));
        let hovered = |rect: Rect| input.pointer.is_some_and(|p| rect.contains(p.into()));
        self.menu_focus.hovered = if hovered(layout.action) {
            Some(0)
        } else if hovered(layout.close) {
            Some(1)
        } else {
            None
        };
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        if clicked && hovered(layout.close) || self.controls.pressed(Action::Interact, input) {
            if self.upgrade_confirmation.take().is_none() {
                self.npc_interaction = None;
            }
            return;
        }
        let previous = self.npc_trade_selection;
        if self.upgrade_confirmation.is_none() {
            if self.controls.pressed(Action::MenuUp, input) || input.wheel_y > 0.0 {
                self.npc_trade_selection = self.npc_trade_selection.saturating_sub(1);
            } else if self.controls.pressed(Action::MenuDown, input) || input.wheel_y < 0.0 {
                self.npc_trade_selection =
                    (self.npc_trade_selection + 1).min(items.len().saturating_sub(1));
            }
            let first = layout.first(self.npc_trade_selection);
            if clicked {
                for (row, index) in (first..items.len()).take(layout.rows).enumerate() {
                    if hovered(layout.row(row)) {
                        self.npc_trade_selection = index;
                    }
                }
            }
        }
        if previous != self.npc_trade_selection {
            self.npc_interaction_message.clear();
        }
        let Some(&item) = items.get(self.npc_trade_selection) else {
            return;
        };
        if !(self.controls.pressed(Action::Learn, input) || clicked && hovered(layout.action)) {
            return;
        }
        let quote = match self.game.equipment_upgrade_quote(item) {
            Ok(q) => q,
            Err(reason) => {
                self.npc_interaction_message = command_rejection_message(reason).to_owned();
                return;
            }
        };
        if quote.available_fragments < u32::from(quote.fragments) {
            self.npc_interaction_message = "Vous n’avez pas assez de fragments instables.".into();
            return;
        }
        if self.game.player_credits() < quote.credits {
            self.npc_interaction_message = "Vous n’avez pas assez de crédits.".into();
            return;
        }
        if self.upgrade_confirmation != Some((provider, item)) {
            self.upgrade_confirmation = Some((provider, item));
            self.npc_interaction_message.clear();
            return;
        }
        self.upgrade_confirmation = None;
        let before = self.inventory_entry_name(self.game.player_inventory().get(item).unwrap());
        let outcome = self.execute_command(GameCommand::ImproveEquipment {
            artisan: provider,
            item,
        });
        self.npc_interaction_message = match outcome {
            CommandOutcome::Applied => {
                let after =
                    self.inventory_entry_name(self.game.player_inventory().get(item).unwrap());
                format!("Votre {before} a été amélioré en {after}.")
            }
            CommandOutcome::Rejected(reason) => command_rejection_message(reason).to_owned(),
            _ => String::new(),
        };
        self.capture_events();
    }

    pub(super) fn draw_equipment_upgrade(&self, interaction: &NpcInteraction) {
        let theme = UiTheme;
        let l = UpgradeLayout::new(self.ui_width(), self.ui_height());
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            theme.backdrop(),
        );
        theme.card(l.panel, false);
        draw_text(
            "Amélioration",
            l.panel.x + 24.0,
            l.panel.y + 38.0,
            28.0,
            theme.text(),
        );
        draw_text(
            &format!(
                "{} : valider · Échap : retour",
                self.controls.label(Action::Learn)
            ),
            l.panel.x + l.panel.w - 306.0,
            l.panel.y + 36.0,
            16.0,
            theme.muted(),
        );
        draw_text(
            &self.npc_dialogue(interaction),
            l.panel.x + 24.0,
            l.panel.y + 66.0,
            18.0,
            theme.muted(),
        );
        draw_text(
            "Choisir un équipement",
            l.list.x,
            l.list.y - 14.0,
            18.0,
            theme.muted(),
        );
        let items = self.upgrade_items();
        let selected = self.npc_trade_selection.min(items.len().saturating_sub(1));
        let first = l.first(selected);
        for (row, index) in (first..items.len()).take(l.rows).enumerate() {
            let entry = self.game.player_inventory().get(items[index]).unwrap();
            let rect = l.row(row);
            theme.dialogue_choice(rect, index == selected, false);
            draw_wrapped_text(
                &self.inventory_entry_name(entry),
                rect.x + 28.0,
                rect.y + 18.0,
                rect.w - 40.0,
                2,
                17,
                equipment_affix_names::rarity_color(entry.magic_modifiers().as_ref()),
            );
        }
        if items.is_empty() {
            draw_text(
                "Aucun équipement",
                l.list.x + 12.0,
                l.list.y + 26.0,
                18.0,
                theme.muted(),
            );
        }
        draw_text(
            &format!(
                "{} / {}",
                if items.is_empty() { 0 } else { selected + 1 },
                items.len()
            ),
            l.list.x,
            l.list.y + l.list.h + 20.0,
            16.0,
            theme.muted(),
        );
        let mut enabled = false;
        let confirming = items
            .get(selected)
            .is_some_and(|&item| self.upgrade_confirmation == Some((interaction.provider, item)));
        if let Some(&item) = items.get(selected) {
            let entry = self.game.player_inventory().get(item).unwrap();
            let mut y = draw_wrapped_text(
                &self.inventory_entry_name(entry),
                l.details.x,
                l.details.y + 22.0,
                l.details.w,
                2,
                22,
                equipment_affix_names::rarity_color(entry.magic_modifiers().as_ref()),
            ) + 12.0;
            if confirming {
                draw_wrapped_text(
                    "Les bonus actuels seront perdus et remplacés au hasard. Cet objet ne pourra être amélioré qu’une seule fois.",
                    l.details.x,
                    y + 8.0,
                    l.details.w,
                    5,
                    20,
                    theme.text(),
                );
            } else if let Some(bonus) = entry.magic_modifiers() {
                let special = bonus
                    .effect_affix()
                    .and_then(|id| self.game.rules().weapons.effect_affix(id));
                for line in equipment_affix_names::affix_detail_lines(bonus) {
                    if y + 22.0 > l.details.y + l.details.h {
                        break;
                    }
                    draw_text(&line, l.details.x, y, 18.0, theme.accent());
                    y += 24.0;
                }
                if let Some(effect) = special {
                    let description = self
                        .texts
                        .resolve(DISPLAY_LOCALE, effect.description_key())
                        .unwrap_or("Effet spécial");
                    draw_wrapped_text(
                        description,
                        l.details.x,
                        y,
                        l.details.w,
                        2,
                        17,
                        theme.accent(),
                    );
                }
            } else {
                draw_text("Sans bonus", l.details.x, y, 18.0, theme.muted());
            }
            match self.game.equipment_upgrade_quote(item) {
                Ok(quote) => {
                    enabled = quote.available_fragments >= u32::from(quote.fragments)
                        && self.game.player_credits() >= quote.credits;
                    draw_text(
                        "1 à 6 nouveaux bonus",
                        l.details.x,
                        l.details.y + l.details.h + 20.0,
                        18.0,
                        theme.accent(),
                    );
                    draw_text(
                        &format!(
                            "{} fragment{} instable{} · {} crédits",
                            quote.fragments,
                            if quote.fragments > 1 { "s" } else { "" },
                            if quote.fragments > 1 { "s" } else { "" },
                            quote.credits
                        ),
                        l.panel.x + 24.0,
                        l.action.y + 12.0,
                        18.0,
                        theme.text(),
                    );
                    draw_text(
                        &format!(
                            "Vous avez {} fragments · {} crédits",
                            quote.available_fragments,
                            self.game.player_credits()
                        ),
                        l.panel.x + 24.0,
                        l.action.y + 34.0,
                        16.0,
                        theme.muted(),
                    );
                }
                Err(reason) => {
                    draw_text(
                        command_rejection_message(reason),
                        l.details.x,
                        l.details.y + l.details.h + 20.0,
                        18.0,
                        theme.muted(),
                    );
                }
            }
        }
        draw_wrapped_text(
            &self.npc_interaction_message,
            l.panel.x + 24.0,
            l.action.y - 30.0,
            l.panel.w - 48.0,
            2,
            18,
            theme.text(),
        );
        theme.button(
            l.action,
            if confirming {
                "Confirmer"
            } else {
                "Améliorer"
            },
            self.menu_focus.hovered == Some(0),
            confirming,
            enabled,
            ButtonTone::Primary,
        );
        theme.button(
            l.close,
            if confirming { "Annuler" } else { "Fermer" },
            self.menu_focus.hovered == Some(1),
            false,
            true,
            ButtonTone::Secondary,
        );
    }
}
