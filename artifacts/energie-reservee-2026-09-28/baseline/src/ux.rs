//! Presentation-only state and scrollable readers. No game commands are issued by readers.
use super::*;
use std::cell::Cell;

#[derive(Default)]
pub(super) struct UxState {
    pub history_open: bool,
    pub history: Vec<(u64, String)>,
    pub history_scroll: ScrollState,
    pub help_tab: usize,
    pub help_scroll: ScrollState,
    pub dossier_scroll: ScrollState,
    pub skill_scroll: ScrollState,
    pub skill_filter: bool,
    pub skill_selection: Cell<(usize, usize)>,
    pub tracked_quest: Option<ContentId>,
    pub inspected_target: bool,
    pub moved: bool,
    pub interacted: bool,
    pub targeted: bool,
    pub item_card: Option<(String, Vec<String>)>,
    pub item_scroll: ScrollState,
    pub comparison_slot: Option<u8>,
    pub hud_points: Cell<Option<Rect>>,
    pub hud_defenses: Cell<Option<Rect>>,
    pub character_scroll: ScrollState,
    pub quest_scroll: ScrollState,
    pub end_selection: usize,
    pub controls_group: usize,
    pub lab_open: bool,
    pub lab_selection: usize,
    pub resume_error: String,
}

#[derive(Default)]
pub(super) struct ScrollState {
    pub offset: f32,
    pub maximum: Cell<f32>,
}

impl ScrollState {
    pub fn update(&mut self, input: &InputFrame, controls: &Controls) {
        let delta = if controls.pressed(Action::MenuDown, input) {
            28.0
        } else if controls.pressed(Action::MenuUp, input) {
            -28.0
        } else if input.pressed.contains(&controls::Binding::key("PageDown")) {
            240.0
        } else if input.pressed.contains(&controls::Binding::key("PageUp")) {
            -240.0
        } else {
            -input.wheel_y * 36.0
        };
        self.offset = (self.offset + delta).clamp(0.0, self.maximum.get());
    }

    pub fn finish(&self, rect: Rect, bottom: f32) {
        let maximum = (bottom - rect.bottom() + 12.0).max(0.0);
        self.maximum.set(maximum);
        if maximum > 0.0 {
            let track = Rect::new(rect.right() - 4.0, rect.y, 3.0, rect.h);
            let thumb = (track.h * rect.h / (maximum + rect.h)).max(20.0);
            draw_rectangle(track.x, track.y, track.w, track.h, UiTheme.surface_raised());
            draw_rectangle(
                track.x,
                track.y + (track.h - thumb) * self.offset.min(maximum) / maximum,
                track.w,
                thumb,
                UiTheme.accent(),
            );
        }
    }
}

pub(super) fn reader_panel(width: f32, height: f32) -> Rect {
    let w = (width - 40.0).min(1000.0);
    let h = (height - 40.0).min(740.0);
    Rect::new((width - w) * 0.5, (height - h) * 0.5, w, h)
}

pub(super) fn reader_close(panel: Rect) -> Rect {
    Rect::new(panel.right() - 116.0, panel.y + 18.0, 96.0, 34.0)
}

pub(super) fn reader_body(panel: Rect) -> Rect {
    Rect::new(
        panel.x + 24.0,
        panel.y + 116.0,
        panel.w - 48.0,
        panel.h - 161.0,
    )
}

pub(super) fn draw_reader(
    title: &str,
    subtitle: &str,
    panel: Rect,
    hovered: bool,
    controls: &Controls,
) {
    UiTheme.panel(panel);
    draw_wrapped_text(
        title,
        panel.x + 24.0,
        panel.y + 43.0,
        panel.w - 168.0,
        1,
        26,
        UiTheme.text(),
    );
    draw_wrapped_text(
        subtitle,
        panel.x + 24.0,
        panel.y + 76.0,
        panel.w - 48.0,
        1,
        15,
        UiTheme.muted(),
    );
    UiTheme.button(
        reader_close(panel),
        "Fermer",
        hovered,
        false,
        true,
        ButtonTone::Secondary,
    );
    draw_text(
        format!(
            "Molette / {} {} / Page préc. suiv. · Échap : fermer",
            controls.label(Action::MenuUp),
            controls.label(Action::MenuDown)
        ),
        panel.x + 24.0,
        panel.bottom() - 18.0,
        14.0,
        UiTheme.muted(),
    );
}

impl AsciiApp {
    pub(super) fn npc_layout(&self, width: f32, height: f32) -> NpcInteractionLayout {
        if let Some(provider) = self.npc_interaction {
            if let Some(dialogue) = self.game.dialogue_view(provider)
                && !self.npc_dialogue_services
            {
                return NpcInteractionLayout::bounded(
                    width,
                    height,
                    332.0 + 48.0 * dialogue.choices.len().clamp(1, 4) as f32,
                );
            }
            if self.game.npc_interaction(provider).is_some_and(|npc| {
                npc.quests.is_empty()
                    && matches!(npc.services.first(), Some(NpcService::Treatment { .. }))
            }) {
                return NpcInteractionLayout::bounded(width, height, 440.0);
            }
        }
        NpcInteractionLayout::new(width, height)
    }
    #[cfg(debug_assertions)]
    pub(super) fn prepare_end_diagnostic(&mut self) -> Result<(), String> {
        use project_rl::world::generation::{GeneratedMap, MapValidationRules};
        let map =
            project_rl::world::Map::from_ascii("#####\n#...#\n#####").map_err(|e| e.to_string())?;
        let generated = GeneratedMap::from_layout(
            map,
            GridPos::new(1, 1),
            GridPos::new(2, 1),
            &[],
            MapValidationRules::default(),
        )
        .map_err(|e| e.to_string())?;
        let game = GameState::from_generated(generated, 17, self.rules.clone())
            .map_err(|e| e.to_string())?;
        self.terminal = TerminalView::new(
            crate::test_sector::SectorDecor::default(),
            game.map(),
            game.player_visibility(),
        );
        self.game = WorldState::single(game);
        self.execute_command(GameCommand::Move(Direction::East));
        self.capture_events_at(Some(0.0));
        if self.game.status() == RunStatus::Active {
            return Err("Le scénario de bilan n'a pas atteint sa sortie.".to_owned());
        }
        Ok(())
    }

    fn inventory_card_is_weapon(&self) -> bool {
        self.inventory_open
            && self
                .inventory_entries()
                .get(self.inventory_selection)
                .is_some_and(|e| self.game.rules().weapons.get(e.item()).is_some())
    }
    pub(super) fn journal_entries(&self) -> Vec<project_rl::game::QuestJournalEntry> {
        let mut entries = self.game.quest_journal();
        entries.sort_by_key(|e| e.quest.status == QuestStatus::Completed);
        entries
    }
    // Only report blockers that can be established from the current visible state.
    // Placement, component selection and specialised conditions remain in the full card.
    pub(super) fn quick_technique_status(&self, id: &TechniqueId) -> (String, Option<String>) {
        let Some(definition) = self.game.rules().skills.technique(id) else {
            return (String::new(), Some("Technique indisponible".to_owned()));
        };
        let action = definition.action();
        let mut energy = match action {
            Some(
                TechniqueAction::DiagnoseEnergy { energy_cost, .. }
                | TechniqueAction::DiagnoseComponent { energy_cost, .. }
                | TechniqueAction::RepairComponent { energy_cost, .. }
                | TechniqueAction::BypassComponent { energy_cost, .. }
                | TechniqueAction::WeaponAttack { energy_cost, .. }
                | TechniqueAction::WeaponVolley { energy_cost, .. }
                | TechniqueAction::AnalyzeMultipleTargets { energy_cost, .. },
            ) => Some(energy_cost),
            Some(
                TechniqueAction::AnalyzeTarget { .. }
                | TechniqueAction::AnalyzeThreat { .. }
                | TechniqueAction::ReadMovementTraces { .. }
                | TechniqueAction::AnalyzeNearbyWalls { .. },
            ) => Some(0),
            _ => None,
        };
        let mut resource_text = String::new();
        let mut blocked = self
            .game
            .actors()
            .get(self.game.player_id())
            .and_then(|a| a.technique_cooldown_remaining(id))
            .map(|t| format!("Récupération : {} tour(s)", t.get()));
        if let Some(cost) = definition.activation_cost() {
            energy = Some(cost.energy());
            resource_text = format!(
                " · +{} chaleur · {} bande passante",
                cost.heat(),
                cost.persistent_bandwidth()
            );
            if cost.persistent_bandwidth()
                > self.game.player_bandwidth().map_or(0, |r| r.available())
            {
                blocked.get_or_insert("Bande passante insuffisante".to_owned());
            }
        }
        if energy.is_some_and(|e| e > self.game.player_energy().available()) {
            blocked.get_or_insert("Énergie insuffisante".to_owned());
        }
        let no_target = match action {
            Some(
                TechniqueAction::AnalyzeTarget { .. }
                | TechniqueAction::AnalyzeThreat { .. }
                | TechniqueAction::DiagnoseEnergy { .. }
                | TechniqueAction::AnalyzeMultipleTargets { .. },
            ) => self.game.player_technique_targets(id).is_empty(),
            Some(
                TechniqueAction::WeaponAttack {
                    melee_arc: None, ..
                }
                | TechniqueAction::WeaponVolley { .. }
                | TechniqueAction::AmbushAttack { .. }
                | TechniqueAction::ChargeAttack { .. }
                | TechniqueAction::Breakthrough { .. },
            ) => self
                .game
                .player_weapon_technique_targets(id, self.active_weapon_slot)
                .is_empty(),
            _ => false,
        };
        if no_target {
            blocked.get_or_insert("Aucune cible valide à portée".to_owned());
        }
        let cost = energy.map_or_else(
            || {
                format!(
                    "Coûts et conditions : fiche [{}]",
                    self.controls.label(Action::Inspect)
                )
            },
            |e| format!("{e} énergie{resource_text}"),
        );
        (cost, blocked)
    }

    pub(super) fn control_actions(&self) -> Vec<Action> {
        Action::ALL
            .iter()
            .copied()
            .filter(|a| {
                self.ux.controls_group == 0 || Self::control_group(*a) == self.ux.controls_group
            })
            .collect()
    }

    fn control_group(action: Action) -> usize {
        match action {
            Action::MoveNorth
            | Action::MoveSouth
            | Action::MoveEast
            | Action::MoveWest
            | Action::Wait => 1,
            Action::Inventory
            | Action::Character
            | Action::Skills
            | Action::QuickTechniques
            | Action::Report
            | Action::QuestJournal
            | Action::Legend
            | Action::EventHistory
            | Action::Inspect => 3,
            Action::MenuUp
            | Action::MenuDown
            | Action::MenuLeft
            | Action::MenuRight
            | Action::Learn
            | Action::Use
            | Action::Drop
            | Action::InventoryFilter
            | Action::InventorySort => 4,
            Action::Corrosion | Action::Pulse | Action::Laboratory => 5,
            _ => 2,
        }
    }

    pub(super) fn control_tab(width: f32, index: usize) -> Rect {
        Rect::new(
            30.0 + index as f32 * (width - 60.0) / 6.0,
            103.0,
            (width - 78.0) / 6.0,
            28.0,
        )
    }

    pub(super) fn lab_actions(width: f32, height: f32) -> [Rect; 6] {
        let p = reader_panel(width, height);
        std::array::from_fn(|i| {
            Rect::new(p.x + 24.0, p.y + 110.0 + i as f32 * 49.0, p.w - 48.0, 39.0)
        })
    }

    pub(super) fn update_lab_menu(&mut self, input: &InputFrame) {
        let (w, h) = input.viewport.unwrap_or((1280.0, 800.0));
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        if self.controls.pressed(Action::MenuUp, input) {
            self.ux.lab_selection = self.ux.lab_selection.saturating_sub(1);
        }
        if self.controls.pressed(Action::MenuDown, input) {
            self.ux.lab_selection = (self.ux.lab_selection + 1).min(5);
        }
        let hover = Self::lab_actions(w, h)
            .iter()
            .position(|r| input.pointer.is_some_and(|p| r.contains(p.into())));
        self.menu_focus.hovered = hover;
        if clicked && let Some(i) = hover {
            self.ux.lab_selection = i;
        }
        if self.controls.pressed(Action::Laboratory, input) {
            self.ux.lab_open = false;
            return;
        }
        if self.controls.pressed(Action::Learn, input) || clicked && hover.is_some() {
            self.ux.lab_open = false;
            match self.ux.lab_selection {
                0 => {
                    self.inventory_open = true;
                    self.inventory_filter = InventoryFilter::Weapons;
                    self.clamp_inventory_selection();
                }
                1 => {
                    self.inventory_open = true;
                    self.inventory_filter = InventoryFilter::Armor;
                    self.clamp_inventory_selection();
                }
                2 => {
                    self.skills_open = true;
                    self.clamp_skill_selection();
                }
                3 => self.cycle_target(),
                4 => {
                    if let Err(e) = self.reset_test_lab() {
                        self.push_log(e);
                    }
                }
                _ => {}
            }
        }
    }

    pub(super) fn draw_lab_menu(&self) {
        let p = reader_panel(self.ui_width(), self.ui_height());
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            UiTheme.backdrop(),
        );
        UiTheme.panel(p);
        draw_text_bold("Laboratoire", p.x + 24.0, p.y + 42.0, 28.0, UiTheme.text());
        let target = self
            .terminal_target_summary()
            .map_or("aucune".to_owned(), |t| t.name);
        draw_wrapped_text(
            &format!("Cible actuelle : {target} · Choisissez ce que vous voulez tester."),
            p.x + 24.0,
            p.y + 77.0,
            p.w - 48.0,
            1,
            16,
            UiTheme.muted(),
        );
        for (i, (rect, label)) in Self::lab_actions(self.ui_width(), self.ui_height())
            .into_iter()
            .zip([
                "Armes et effets · choisir dans l'inventaire",
                "Protections et bonus d'équipement",
                "Techniques et compétences",
                "Sélectionner la cible suivante",
                "Réinitialiser les cibles et les réserves",
                "Revenir à l'essai",
            ])
            .enumerate()
        {
            UiTheme.button(
                rect,
                label,
                self.menu_focus.hovered == Some(i),
                self.ux.lab_selection == i,
                true,
                if i == 5 {
                    ButtonTone::Primary
                } else {
                    ButtonTone::Secondary
                },
            );
        }
    }

    pub(super) fn cycle_companion_order(&mut self) {
        let current = self
            .game
            .player_controlled_companions()
            .first()
            .and_then(|id| self.game.actors().get(*id))
            .and_then(Actor::drone)
            .and_then(|d| match d.order() {
                DroneOrder::Companion { behavior, .. } => Some(*behavior),
                DroneOrder::Escort { .. } => Some(CompanionBehavior::Follow),
                _ => None,
            });
        let next = current
            .and_then(|c| CompanionBehavior::ALL.iter().position(|b| *b == c))
            .map_or(0, |i| (i + 1) % CompanionBehavior::ALL.len());
        let outcome = self.execute_command(GameCommand::SetCompanionBehavior {
            behavior: CompanionBehavior::ALL[next],
        });
        if let CommandOutcome::Rejected(reason) = outcome {
            self.push_log(command_rejection_message(reason).to_owned());
        }
        self.capture_events();
    }
    pub(super) fn item_card_lines(
        &self,
        id: &ItemId,
        bonus: Option<project_rl::entity::MagicItemModifiers>,
        hidden: bool,
    ) -> Vec<String> {
        if hidden {
            return vec!["Objet magique non identifié. Les propriétés seront révélées après le pari.".to_owned(), "Le prix et le type d'objet sont connus ; aucun bonus n'est garanti avant identification.".to_owned()];
        }
        let mut lines = Vec::new();
        if let Some(b) = bonus.clone() {
            lines.extend(equipment_affix_names::affix_detail_lines(b));
        }
        if let Some(weapon) = self.game.rules().weapons.get(id) {
            let attack = weapon.attack();
            lines.push(format!(
                "Arme · portée {} case(s) · précision de base {:+}",
                attack.range(),
                attack.accuracy_modifier()
            ));
            lines.push(format!(
                "Dégâts de base · {}",
                format_damage_impact(attack.damage())
            ));
            if let Some(impact) = attack.melee_impact() {
                lines.push(format!(
                    "Impact maximal du matériau · {}",
                    impact.material_cap
                ));
            }
            if let Some(cost) = self.weapon_supply_label(id) {
                lines.push(format!("Ressource · {cost}"));
            }
            lines.push(
                self.texts
                    .resolve(DISPLAY_LOCALE, weapon.description_key())
                    .unwrap_or("")
                    .to_owned(),
            );
            lines.push(self.weapon_effect_limits(weapon));
            if let Some(b) = bonus.as_ref() {
                if b.accuracy_bonus() > 0 {
                    lines.push(format!("Bonus de précision · +{}", b.accuracy_bonus()));
                }
                if let Some(effect) = b
                    .effect_affix()
                    .and_then(|id| self.game.rules().weapons.effect_affix(id))
                {
                    lines.push(
                        self.texts
                            .resolve(DISPLAY_LOCALE, effect.description_key())
                            .unwrap_or("")
                            .to_owned(),
                    );
                }
            }
        }
        if let Some(item) = self.game.rules().items.get(id) {
            lines.push(
                self.texts
                    .resolve(DISPLAY_LOCALE, item.description_key())
                    .unwrap_or("")
                    .to_owned(),
            );
            if let Some(equipment) = item.equipment() {
                lines.push(format!(
                    "Armure · {} · emplacement {}",
                    equipment.armor() + bonus.as_ref().map_or(0, |b| b.armor_bonus()),
                    self.equipment_slot_name(equipment.slot())
                ));
            }
            for effect in item.effects() {
                let ItemEffect::RestoreIntegrity { amount } = effect;
                lines.push(format!("Restaure {amount} PV"));
            }
        }
        if lines.is_empty() {
            lines.push("Matériau utilisable dans les interactions qui le demandent.".to_owned());
        }
        lines
    }

    pub(super) fn open_inventory_card(&mut self) {
        let entries = self.inventory_entries();
        if let Some(entry) = entries.get(self.inventory_selection) {
            self.ux.item_card = Some((
                self.inventory_entry_name(entry),
                self.item_card_lines(entry.item(), entry.magic_modifiers(), false),
            ));
            self.ux.item_scroll.offset = 0.0;
            self.ux.comparison_slot = None;
        }
    }

    pub(super) fn update_item_card(&mut self, input: &InputFrame) {
        let (w, h) = input.viewport.unwrap_or((1280.0, 800.0));
        let panel = reader_panel(w, h);
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        if clicked
            && input
                .pointer
                .is_some_and(|p| reader_close(panel).contains(p.into()))
        {
            self.ux.item_card = None;
            return;
        }
        if self.inventory_card_is_weapon() {
            for slot in 0..3 {
                let rect = Rect::new(
                    panel.x + 24.0 + slot as f32 * (panel.w - 48.0) / 3.0,
                    panel.y + 74.0,
                    (panel.w - 60.0) / 3.0,
                    32.0,
                );
                if clicked && input.pointer.is_some_and(|p| rect.contains(p.into()))
                    || pressed_weapon_slot(&self.controls, input) == Some(slot)
                {
                    self.ux.comparison_slot = Some(slot);
                    self.ux.item_scroll.offset = 0.0;
                }
            }
        }
        self.ux.item_scroll.update(input, &self.controls);
    }

    pub(super) fn draw_item_card(&self) {
        let Some((title, lines)) = &self.ux.item_card else {
            return;
        };
        let panel = reader_panel(self.ui_width(), self.ui_height());
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            UiTheme.backdrop(),
        );
        draw_reader(
            title,
            if self.inventory_card_is_weapon() {
                ""
            } else {
                "Informations connues · lecture complète"
            },
            panel,
            false,
            &self.controls,
        );
        if self.inventory_card_is_weapon() {
            for slot in 0..3 {
                let rect = Rect::new(
                    panel.x + 24.0 + slot as f32 * (panel.w - 48.0) / 3.0,
                    panel.y + 74.0,
                    (panel.w - 60.0) / 3.0,
                    32.0,
                );
                UiTheme.button(
                    rect,
                    &format!(
                        "Comparer au canal {} [{}]",
                        slot + 1,
                        self.controls
                            .label([Action::Slot1, Action::Slot2, Action::Slot3][slot as usize])
                    ),
                    false,
                    self.ux.comparison_slot == Some(slot),
                    true,
                    ButtonTone::Secondary,
                );
            }
        }
        let body = reader_body(panel);
        crate::ui_theme::begin_text_pane(body, self.ux.item_scroll.offset);
        let mut y = body.y + 24.0;
        for line in lines {
            y = draw_wrapped_text(line, body.x, y, body.w - 20.0, 4096, 17, UiTheme.text()) + 16.0;
        }
        if let Some(slot) = self.ux.comparison_slot {
            y += 16.0;
            draw_text_bold(
                format!("Actuellement au canal {}", slot + 1),
                body.x,
                y,
                20.0,
                UiTheme.accent(),
            );
            y += 34.0;
            let id = self
                .game
                .rules()
                .player_weapon_slots
                .get(slot as usize)
                .and_then(|s| self.game.player_equipment().equipped(s));
            if let Some(entry) = id.and_then(|id| self.game.player_inventory().get(id)) {
                y = draw_wrapped_text(
                    &self.inventory_entry_name(entry),
                    body.x,
                    y,
                    body.w - 20.0,
                    4096,
                    19,
                    UiTheme.accent(),
                ) + 12.0;
                for line in self.item_card_lines(entry.item(), entry.magic_modifiers(), false) {
                    y = draw_wrapped_text(
                        &line,
                        body.x,
                        y,
                        body.w - 20.0,
                        4096,
                        17,
                        UiTheme.text(),
                    ) + 16.0;
                }
            } else {
                draw_text("Emplacement vide", body.x, y, 17.0, UiTheme.muted());
                y += 26.0;
            }
        }
        crate::ui_theme::end_text_pane();
        self.ux.item_scroll.finish(body, y);
    }

    pub(super) fn end_actions(width: f32, height: f32) -> [Rect; 4] {
        let panel = reader_panel(width, height);
        let w = (panel.w - 60.0) / 2.0;
        std::array::from_fn(|i| {
            Rect::new(
                panel.x + 24.0 + (i % 2) as f32 * (w + 12.0),
                panel.bottom() - 106.0 + (i / 2) as f32 * 44.0,
                w,
                36.0,
            )
        })
    }

    pub(super) fn update_end_screen(&mut self, input: &InputFrame) {
        let (w, h) = input.viewport.unwrap_or((1280.0, 800.0));
        let hovered = Self::end_actions(w, h)
            .iter()
            .position(|r| input.pointer.is_some_and(|p| r.contains(p.into())));
        self.menu_focus.hovered = hovered;
        if self.controls.pressed(Action::MenuUp, input) {
            self.ux.end_selection = self.ux.end_selection.saturating_sub(2);
        }
        if self.controls.pressed(Action::MenuDown, input) {
            self.ux.end_selection = (self.ux.end_selection + 2).min(3);
        }
        if self.controls.pressed(Action::MenuLeft, input) {
            self.ux.end_selection = self.ux.end_selection.saturating_sub(1);
        }
        if self.controls.pressed(Action::MenuRight, input) {
            self.ux.end_selection = (self.ux.end_selection + 1).min(3);
        }
        let chosen = if input.pressed.contains(&controls::Binding::MouseLeft) {
            hovered
        } else if self.controls.pressed(Action::Learn, input) {
            Some(self.ux.end_selection)
        } else {
            None
        };
        if chosen.is_some() {
            match chosen {
                Some(0) => self.restart_current_profile(),
                Some(1) => {
                    if let Err(e) = self.begin_character_creation(false) {
                        self.push_log(e);
                    }
                }
                Some(2) => {
                    self.ux.history_open = true;
                    self.ux.history_scroll.offset = 0.0;
                }
                Some(3) => {
                    if let Err(e) = self.rebuild_run(MenuScreen::Main, false) {
                        self.push_log(e);
                    }
                }
                _ => {}
            }
        }
    }

    pub(super) fn draw_run_summary(&self) {
        if self.game.status() == RunStatus::Active {
            return;
        }
        let panel = reader_panel(self.ui_width(), self.ui_height());
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            UiTheme.backdrop(),
        );
        UiTheme.panel(panel);
        draw_text_bold(
            if self.game.status() == RunStatus::PlayerDestroyed {
                "Votre noyau a été détruit"
            } else {
                "Expédition terminée"
            },
            panel.x + 24.0,
            panel.y + 45.0,
            28.0,
            UiTheme.text(),
        );
        let p = self.game.player_progression();
        draw_text(
            format!(
                "{} tours · Niveau {} · {} XP · {} archives découvertes",
                self.game.turn(),
                p.level(),
                p.experience(),
                self.game.discovered_data_terminal_records().len()
            ),
            panel.x + 24.0,
            panel.y + 79.0,
            17.0,
            UiTheme.accent(),
        );
        draw_text_bold(
            "Derniers événements observés",
            panel.x + 24.0,
            panel.y + 124.0,
            18.0,
            UiTheme.muted(),
        );
        let body = Rect::new(
            panel.x + 24.0,
            panel.y + 136.0,
            panel.w - 48.0,
            panel.h - 257.0,
        );
        crate::ui_theme::begin_text_pane(body, 0.0);
        let mut y = body.y + 24.0;
        for m in self.log.iter().rev().take(4) {
            y = draw_wrapped_text(m, body.x, y, body.w, 4096, 17, UiTheme.text()) + 15.0;
        }
        crate::ui_theme::end_text_pane();
        for (i, (rect, label)) in Self::end_actions(self.ui_width(), self.ui_height())
            .into_iter()
            .zip([
                format!("Recommencer [{}]", self.controls.label(Action::Restart)),
                "Choisir un autre profil".to_owned(),
                format!(
                    "Derniers tours [{}]",
                    self.controls.label(Action::EventHistory)
                ),
                "Retour à l'accueil".to_owned(),
            ])
            .enumerate()
        {
            UiTheme.button(
                rect,
                &label,
                self.menu_focus.hovered == Some(i),
                self.ux.end_selection == i,
                true,
                if i == 0 {
                    ButtonTone::Primary
                } else {
                    ButtonTone::Secondary
                },
            );
        }
    }
    pub(super) fn skill_learning_cost(&self, id: &TechniqueId) -> Result<u16, String> {
        let definition = self
            .game
            .rules()
            .skills
            .technique(id)
            .ok_or("Technique indisponible")?;
        if self.game.player_skills().has_learned(id) {
            return Err("Déjà apprise".to_owned());
        }
        let availability = self
            .skill_availability_cache
            .get(definition.discipline())
            .ok_or("Discipline indisponible")?;
        if !availability.is_open() || !availability.available.contains(id) {
            return Err("Technique indisponible".to_owned());
        }
        if definition.minimum_level() > self.game.player_progression().level() {
            return Err(format!("Niveau {} requis", definition.minimum_level()));
        }
        if let Some(required) = definition.prerequisite()
            && !self.game.player_skills().has_learned(required)
        {
            return Err(format!(
                "Apprendre {} d'abord",
                self.technique_name(required)
            ));
        }
        if let Some(r) =
            definition.unmet_attribute_requirement(self.game.player_primary_attributes())
        {
            return Err(format!(
                "{} {} requis",
                primary_attribute_label(r.attribute()),
                r.minimum()
            ));
        }
        let learned = self
            .skill_techniques_cache
            .get(definition.discipline())
            .into_iter()
            .flatten()
            .filter(|id| self.game.player_skills().has_learned(id))
            .count();
        let cost = self
            .game
            .rules()
            .skill_progression
            .cost_for_choice_number(learned + 1)
            .ok_or("Coût indisponible")?;
        if self.game.player_progression().unspent_skill_points() < u32::from(cost) {
            return Err(format!("{cost} points nécessaires"));
        }
        Ok(cost)
    }

    pub(super) fn skill_filter_rect(panel: Rect) -> Rect {
        Rect::new(
            panel.x + 10.0,
            panel.bottom() - 44.0,
            (panel.w * 0.23).clamp(180.0, 260.0),
            32.0,
        )
    }

    pub(super) fn update_inspector(&mut self, input: &InputFrame) {
        let (w, h) = input.viewport.unwrap_or((1280.0, 800.0));
        let hovered = input
            .pointer
            .is_some_and(|p| reader_close(reader_panel(w, h)).contains(p.into()));
        self.menu_focus.hovered = hovered.then_some(0);
        if self.controls.pressed(Action::Inspect, input)
            || input.pressed.contains(&controls::Binding::MouseLeft) && hovered
        {
            self.ux.inspected_target = false;
        }
        self.ux.help_scroll.update(input, &self.controls);
    }

    pub(super) fn draw_inspector(&self) {
        let panel = reader_panel(self.ui_width(), self.ui_height());
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            UiTheme.backdrop(),
        );
        draw_reader(
            "État et cible",
            "Informations connues · aucune action ni aucun tour dépensé",
            panel,
            self.menu_focus.hovered == Some(0),
            &self.controls,
        );
        let body = reader_body(panel);
        crate::ui_theme::begin_text_pane(body, self.ux.help_scroll.offset);
        let mut y = body.y + 22.0;
        let id = self.game.player_id();
        let mut lines = Vec::new();
        if let Some(player) = self.game.actors().get(id) {
            lines.push(format!("Votre état · {} / {} PV · Armure {} · Esquive {} · Stabilité {} · Défense numérique {}",player.integrity(),player.maximum_integrity(),self.game.actor_armor_profile(id).map_or(0,ArmorProfile::after_fragilization),self.game.actor_evasion(id).unwrap_or(0),self.game.actor_stability(id).unwrap_or(0),self.game.actor_digital_defense(id).unwrap_or(0)));
            lines.push(format!(
                "Résistances · {}",
                [
                    ("Thermique", DamageType::Thermal),
                    ("Électrique", DamageType::Electrical),
                    ("Chimique", DamageType::Chemical),
                    ("Radiation", DamageType::Radiation),
                    ("Corruption", DamageType::Corruption)
                ]
                .into_iter()
                .map(|(label, t)| format!("{label} {} %", player.resistances().get(t)))
                .collect::<Vec<_>>()
                .join(" · ")
            ));
        }
        lines.push(format!(
            "Énergie {}/{} · Munitions {}",
            self.game.player_energy().available(),
            self.game.player_energy().capacity(),
            self.game.player_matter().unwrap_or(0)
        ));
        if let Some(b) = self.game.player_bandwidth() {
            lines.push(format!(
                "Bande passante disponible {}/{}",
                b.available(),
                b.capacity()
            ));
        }
        if let Some(h) = self.game.player_heat() {
            lines.push(format!(
                "Chaleur {} · alerte {} · seuil critique {}",
                h.current(),
                h.alert_threshold(),
                h.critical_threshold()
            ));
        }
        if let Some(target) = self.terminal_target_summary() {
            lines.push(format!(
                "Cible · {} · distance {} cases",
                target.name, target.distance
            ));
            lines.push(target.visible_state);
            if let Some(a) = target.analysis {
                lines.push(format!(
                    "Analyse · {} / {} PV · Armure {} · {}",
                    a.integrity, a.maximum_integrity, a.armor, a.resistances
                ));
            } else {
                lines.push(
                    "Statistiques inconnues. Une analyse de la cible peut compléter cette fiche."
                        .to_owned(),
                );
            }
        } else {
            lines.push(format!(
                "Aucune cible sélectionnée. {} ou clic sur une entité visible pour choisir.",
                self.controls.label(Action::CycleTarget)
            ));
        }
        for line in lines {
            y = draw_wrapped_text(&line, body.x, y, body.w - 20.0, 4096, 17, UiTheme.text()) + 24.0;
        }
        crate::ui_theme::end_text_pane();
        self.ux.help_scroll.finish(body, y);
    }

    pub(super) fn hud_shortcuts(
        width: f32,
        height: f32,
    ) -> Vec<(Rect, Option<Action>, &'static str, UiIcon)> {
        let start = quest_journal_button_rect(height).right() + 8.0;
        let stride = (width - start - 12.0) / 8.0;
        [
            (Some(Action::Interact), "Interagir", UiIcon::Interact),
            (Some(Action::Attack), "Attaquer", UiIcon::Attack),
            (
                Some(Action::QuickTechniques),
                "Techniques",
                UiIcon::Techniques,
            ),
            (Some(Action::Inventory), "Inventaire", UiIcon::Inventory),
            (Some(Action::Inspect), "État / cible", UiIcon::Target),
            (Some(Action::EventHistory), "Historique", UiIcon::Quest),
            (Some(Action::Legend), "Aide", UiIcon::Help),
            (None, "Menu", UiIcon::Menu),
        ]
        .into_iter()
        .enumerate()
        .map(|(i, (a, l, icon))| {
            (
                Rect::new(start + i as f32 * stride, height - 94.0, stride - 4.0, 34.0),
                a,
                l,
                icon,
            )
        })
        .collect()
    }

    pub(super) fn lab_button(width: f32, height: f32) -> Rect {
        Rect::new(width - 236.0, height - 144.0, 224.0, 36.0)
    }
    pub(super) fn draw_hud_shortcuts(&self) {
        if self.test_lab {
            UiTheme.button(
                Self::lab_button(self.ui_width(), self.ui_height()),
                &format!("Essais [{}]", self.controls.label(Action::Laboratory)),
                self.menu_focus.hovered == Some(50_030),
                false,
                true,
                ButtonTone::Secondary,
            );
        }
        for (index, (rect, action, label, icon)) in
            Self::hud_shortcuts(self.ui_width(), self.ui_height())
                .into_iter()
                .enumerate()
        {
            let hovered = self.menu_focus.hovered == Some(50_000 + index);
            UiTheme.hud_action(rect, if hovered { 1.0 } else { 0.0 });
            let key = action.map_or_else(|| "Esc".to_owned(), |a| self.controls.label(a));
            draw_ui_icon(
                icon,
                Rect::new(rect.x + 7.0, rect.y + 10.0, 15.0, 15.0),
                UiTheme.accent(),
            );
            if rect.w >= 100.0 {
                draw_wrapped_text(
                    label,
                    rect.x + 28.0,
                    rect.y + 14.0,
                    rect.w - 34.0,
                    1,
                    12,
                    UiTheme.text(),
                );
                draw_wrapped_text(
                    &key,
                    rect.x + 28.0,
                    rect.y + 28.0,
                    rect.w - 34.0,
                    1,
                    11,
                    UiTheme.accent(),
                );
            } else {
                draw_wrapped_text(
                    &key,
                    rect.x + 27.0,
                    rect.y + 23.0,
                    rect.w - 32.0,
                    1,
                    13,
                    UiTheme.text(),
                );
            }
            if hovered {
                draw_text_bold(
                    label,
                    rect.x.min(self.ui_width() - 130.0),
                    rect.y - 8.0,
                    14.0,
                    UiTheme.text(),
                );
            }
        }
    }

    pub(super) fn route_hud_click(&mut self, input: &InputFrame, captured_at: Option<f64>) -> bool {
        let (w, h) = input.viewport.unwrap_or((1280.0, 800.0));
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        if self.test_lab
            && input
                .pointer
                .is_some_and(|p| Self::lab_button(w, h).contains(p.into()))
        {
            self.menu_focus.hovered = Some(50_030);
            if clicked {
                self.ux.lab_open = true;
                return true;
            }
        }
        for (rect, action, focus) in [
            (self.ux.hud_points.get(), Action::Skills, 50_020),
            (self.ux.hud_defenses.get(), Action::Inspect, 50_021),
        ] {
            if rect.is_some_and(|r| input.pointer.is_some_and(|p| r.contains(p.into()))) {
                self.menu_focus.hovered = Some(focus);
                if clicked {
                    let forwarded = InputFrame {
                        pressed: [self.controls.binding(action).clone()].into(),
                        viewport: input.viewport,
                        ..Default::default()
                    };
                    self.dispatch_input_at(&forwarded, captured_at);
                    return true;
                }
            }
        }
        for (index, (rect, action, _, _)) in Self::hud_shortcuts(w, h).into_iter().enumerate() {
            if input.pointer.is_some_and(|p| rect.contains(p.into())) {
                self.menu_focus.hovered = Some(50_000 + index);
                if clicked {
                    let mut forwarded = InputFrame::default();
                    forwarded.viewport = input.viewport;
                    if let Some(action) = action {
                        forwarded
                            .pressed
                            .insert(self.controls.binding(action).clone());
                    } else {
                        forwarded.pause = true;
                    }
                    self.dispatch_input_at(&forwarded, captured_at);
                    return true;
                }
            }
        }
        false
    }
}

impl AsciiApp {
    pub(super) fn help_command_lines(&self) -> Vec<String> {
        let mut lines=vec![
                format!("Déplacement · {} {} {} {}. Un déplacement réussi fait avancer le temps.",self.controls.label(Action::MoveNorth),self.controls.label(Action::MoveWest),self.controls.label(Action::MoveSouth),self.controls.label(Action::MoveEast)),
                format!("Interagir · {} près d'une installation, d'un objet ou d'un habitant. Plusieurs possibilités ouvrent un choix.",self.controls.label(Action::Interact)),
                format!("Cibler · {} ou clic sur une entité visible. Attaquer · {}. Une attaque de zone présente sa zone avant confirmation ; Échap l'annule.",self.controls.label(Action::CycleTarget),self.controls.label(Action::Attack)),
                format!("Patienter · {}. Techniques actives · {}. Les coûts et conditions de chaque technique figurent dans les compétences.",self.controls.label(Action::Wait),self.controls.label(Action::QuickTechniques)),
                format!("Inventaire · {}. Personnage · {}. Compétences · {}. Ouvrir ces écrans ne dépense pas de tour.",self.controls.label(Action::Inventory),self.controls.label(Action::Character),self.controls.label(Action::Skills)),
                format!("Quêtes · {}. Archives découvertes · {}. Historique des tours · {}.",self.controls.label(Action::QuestJournal),self.controls.label(Action::Report),self.controls.label(Action::EventHistory)),
                "Échap ferme d'abord l'écran ouvert. Depuis le jeu, il ouvre la pause. Les commandes peuvent être réattribuées dans Options / Commandes.".to_owned(),
            ];
        lines.push(format!(
            "Aide : {} · Inspection de l'état et de la cible : {}.",
            self.controls.label(Action::Legend),
            self.controls.label(Action::Inspect)
        ));
        lines.push(format!("Navigation de l'aide : {} / {} pour les onglets ; {} / {} pour défiler. Molette et Page préc./suiv. sont également disponibles.",self.controls.label(Action::MenuLeft),self.controls.label(Action::MenuRight),self.controls.label(Action::MenuUp),self.controls.label(Action::MenuDown)));
        for action in [
            Action::Slot1,
            Action::Slot2,
            Action::Slot3,
            Action::Analyze,
            Action::Traces,
            Action::Walls,
            Action::Threat,
            Action::Multiple,
            Action::NpcVision,
            Action::CompanionOrder,
            Action::Restart,
            Action::MenuUp,
            Action::MenuDown,
            Action::MenuLeft,
            Action::MenuRight,
            Action::Learn,
            Action::Use,
            Action::Drop,
            Action::InventoryFilter,
            Action::InventorySort,
        ] {
            lines.push(format!(
                "{} · {}",
                action.name(),
                self.controls.label(action)
            ));
        }
        if self.test_lab {
            for action in [Action::Laboratory, Action::Corrosion, Action::Pulse] {
                lines.push(format!(
                    "{} · {}",
                    action.name(),
                    self.controls.label(action)
                ));
            }
        }
        lines
    }

    pub(super) fn update_help(&mut self, input: &InputFrame) {
        let (w, h) = input.viewport.unwrap_or((1280.0, 800.0));
        let panel = reader_panel(w, h);
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        let hovered = input
            .pointer
            .is_some_and(|p| reader_close(panel).contains(p.into()));
        self.menu_focus.hovered = hovered.then_some(0);
        if clicked && hovered {
            self.legend_open = false;
            return;
        }
        let mut tab = self.ux.help_tab;
        if self.controls.pressed(Action::MenuLeft, input) {
            tab = tab.saturating_sub(1);
        }
        if self.controls.pressed(Action::MenuRight, input) {
            tab = (tab + 1).min(2);
        }
        for index in 0..3 {
            let rect = Rect::new(
                panel.x + 24.0 + index as f32 * (panel.w - 48.0) / 3.0,
                panel.y + 74.0,
                (panel.w - 60.0) / 3.0,
                32.0,
            );
            if input.pointer.is_some_and(|p| rect.contains(p.into())) {
                self.menu_focus.hovered = Some(index + 1);
                if clicked {
                    tab = index;
                }
            }
        }
        if tab != self.ux.help_tab {
            self.ux.help_tab = tab;
            self.ux.help_scroll.offset = 0.0;
            return;
        }
        self.ux.help_scroll.update(input, &self.controls);
    }

    pub(super) fn draw_help(&self) {
        let panel = reader_panel(self.ui_width(), self.ui_height());
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            UiTheme.backdrop(),
        );
        draw_reader(
            "Aide",
            "",
            panel,
            self.menu_focus.hovered == Some(0),
            &self.controls,
        );
        for (index, label) in [
            "Commandes essentielles",
            "Symboles",
            "Statistiques et jauges",
        ]
        .into_iter()
        .enumerate()
        {
            let rect = Rect::new(
                panel.x + 24.0 + index as f32 * (panel.w - 48.0) / 3.0,
                panel.y + 74.0,
                (panel.w - 60.0) / 3.0,
                32.0,
            );
            UiTheme.button(
                rect,
                label,
                self.menu_focus.hovered == Some(index + 1),
                self.ux.help_tab == index,
                true,
                ButtonTone::Secondary,
            );
        }
        let body = reader_body(panel);
        if self.ux.help_tab == 1 {
            let bottom = crate::terminal_view::draw_symbol_guide(
                &self.game,
                body,
                self.ux.help_scroll.offset,
            );
            self.ux.help_scroll.finish(body, bottom);
            return;
        }
        if self.ux.help_tab == 2 {
            self.draw_statistics_help(body);
            return;
        }
        crate::ui_theme::begin_text_pane(body, self.ux.help_scroll.offset);
        let lines = self.help_command_lines();
        let mut y = body.y + 24.0;
        for line in lines {
            y = draw_wrapped_text(
                &line,
                body.x,
                y,
                body.w - 16.0,
                usize::MAX,
                17,
                UiTheme.text(),
            ) + 22.0;
        }
        crate::ui_theme::end_text_pane();
        self.ux.help_scroll.finish(body, y);
    }

    pub(super) fn update_history(&mut self, input: &InputFrame) {
        let (w, h) = input.viewport.unwrap_or((1280.0, 800.0));
        let close = reader_close(reader_panel(w, h));
        let hovered = input.pointer.is_some_and(|p| close.contains(p.into()));
        self.menu_focus.hovered = hovered.then_some(0);
        if self.controls.pressed(Action::EventHistory, input)
            || input.pressed.contains(&controls::Binding::MouseLeft) && hovered
        {
            self.ux.history_open = false;
        }
        self.ux.history_scroll.update(input, &self.controls);
    }

    pub(super) fn draw_history(&self) {
        let panel = reader_panel(self.ui_width(), self.ui_height());
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            UiTheme.backdrop(),
        );
        draw_reader(
            "Historique des tours",
            "Événements observés · les plus récents en premier",
            panel,
            self.menu_focus.hovered == Some(0),
            &self.controls,
        );
        let body = reader_body(panel);
        crate::ui_theme::begin_text_pane(body, self.ux.history_scroll.offset);
        let mut y = body.y + 24.0;
        let mut turn = None;
        let history = if self.ux.history.is_empty() {
            self.log
                .iter()
                .map(|m| (self.game.turn(), m.clone()))
                .collect()
        } else {
            self.ux.history.clone()
        };
        // Reverse turn groups, preserving event order within each turn.
        let mut end = history.len();
        while end > 0 {
            let t = history[end - 1].0;
            let start = history[..end]
                .iter()
                .rposition(|(a, _)| *a != t)
                .map_or(0, |i| i + 1);
            for (t, message) in &history[start..end] {
                if turn != Some(*t) {
                    draw_text_bold(format!("Tour {t}"), body.x, y, 18.0, UiTheme.accent());
                    y += 30.0;
                    turn = Some(*t);
                }
                y = draw_wrapped_text(
                    message,
                    body.x + 12.0,
                    y,
                    body.w - 28.0,
                    usize::MAX,
                    16,
                    UiTheme.text(),
                ) + 9.0;
            }
            y += 20.0;
            end = start;
        }
        crate::ui_theme::end_text_pane();
        self.ux.history_scroll.finish(body, y);
    }
}

impl AsciiApp {
    pub(super) fn technique_usage(
        &self,
        definition: &project_rl::skills::TechniqueDefinition,
    ) -> String {
        let action_timing = definition.preparation_steps().map_or_else(
            || "1 tour".to_owned(),
            |steps| format!("Préparation {} tour(s) + action 1 tour", steps.get()),
        );
        let usage = if definition.improvement() == Some(TechniqueImprovement::MeleeCounterattack) {
            "Passif. Après une Parade réussie, tente une frappe ordinaire avec la première arme de mêlée équipée si l'attaquant est encore au contact. Ne consomme pas une seconde réaction."
                    .to_owned()
        } else if definition.improvement() == Some(TechniqueImprovement::ExtendedRangedOverwatch) {
            "Passif. Remplace la ligne de Surveillance par le secteur visible de 90° montré dans l'aperçu. N'ajoute ni portée, ni tir, ni réaction."
                    .to_owned()
        } else if let Some(TechniqueImprovement::PersistentRangedAim {
            retained_accuracy_modifier,
        }) = definition.improvement()
        {
            format!(
                "Passif. Après le tir de la technique requise, conserve Précision {retained_accuracy_modifier:+} contre la même cible. Bouger, perdre la vue, changer de cible ou entreprendre une autre action qu'un tir simple ou attendre annule ce bonus."
            )
        } else if definition.improvement() == Some(TechniqueImprovement::ControlledChargeInertia) {
            "Passif. Une autre action permet d'arrêter volontairement la Charge en cours ; une Charge menée jusqu'à sa frappe finale ne provoque plus de récupération."
                    .to_owned()
        } else if let Some(TechniqueImprovement::CoveredApproach {
            optical_difficulty_bonus,
        }) = definition.improvement()
        {
            format!(
                "Passif. Après un déplacement, un couvert optique réel augmente de {optical_difficulty_bonus} la difficulté de détection pendant la résolution adverse. Ne crée jamais de couvert artificiel."
            )
        } else if let Some(TechniqueImprovement::SilentNeutralization {
            physical_damage_percentage,
            extra_energy_cost,
            noise_reduction,
        }) = definition.improvement()
        {
            format!(
                "Passif d'Embuscade au contact d'une vulnérabilité connue : {physical_damage_percentage} % des dégâts physiques, +{extra_energy_cost} E et bruit −{noise_reduction}."
            )
        } else if let Some(TechniqueImprovement::DroneAutonomousScout {
            maximum_unknown_steps,
            energy_cost_override,
            additional_bandwidth,
        }) = definition.improvement()
        {
            format!(
                "Passif de Patrouille bornée : autorise jusqu'à {maximum_unknown_steps} nouvelles cases avant retour, pour {energy_cost_override} E et +{additional_bandwidth} B durant la routine. Le rapport reste daté et n'accorde aucune vision directe."
            )
        } else {
            match definition.action() {
                Some(TechniqueAction::AnalyzeTarget { range }) => format!(
                    "{action_timing} / 0 E. Analyse une cible visible à portée {range} et révèle son intégrité, son armure et ses résistances observables."
                ),
                Some(TechniqueAction::AnalyzeMultipleTargets {
                    maximum_targets,
                    energy_cost,
                }) => format!(
                    "{action_timing} / {energy_cost} E. Jusqu'à {maximum_targets} cibles visibles : sélection d'abord, puis les plus proches. Même analyse que le prérequis."
                ),
                Some(TechniqueAction::ReadMovementTraces { radius }) => format!(
                    "{action_timing} / 0 E. Rayon {radius}. Indices datés ; aucun suivi de leur auteur."
                ),
                Some(TechniqueAction::InspectNearbySecrets {
                    radius,
                    detection_bonus,
                }) => format!(
                    "{action_timing} / 0 E. Inspecte les cases visibles dans un rayon de {radius}, avec Détection +{detection_bonus}. Découvre un dispositif réellement présent sans l'ouvrir ni le désarmer."
                ),
                Some(TechniqueAction::AnalyzeNearbyWalls {
                    maximum_tiles,
                    radius,
                }) => format!(
                    "{action_timing} / 0 E. Analyse jusqu'à {maximum_tiles} parois liées dans un rayon de {radius} et révèle si elles bloquent le passage ou la vision."
                ),
                Some(TechniqueAction::AnalyzeThreat { range }) => format!(
                    "{action_timing} / 0 E. Analyse une cible visible à portée {range} et révèle ses capacités offensives connues, sans prédire ses décisions."
                ),
                Some(TechniqueAction::DiagnoseEnergy {
                    range,
                    analysis_bonus,
                    energy_cost,
                }) => format!(
                    "{action_timing} / {energy_cost} E. Machine visible, portée {range}, Analyse +{analysis_bonus}. Ne révèle que ses réserves et canaux énergétiques réellement simulés."
                ),
                Some(TechniqueAction::RepairComponent {
                    durability_restored,
                    energy_cost,
                }) => format!(
                    "{action_timing} / {energy_cost} E de fonctionnement. Restaure jusqu'à {durability_restored} durabilité au composant choisi, sans soigner le corps ni recréer un composant détruit."
                ),
                Some(TechniqueAction::SalvageComponent) => format!(
                    "{action_timing}. Préserve dans son état réel un composant survivant choisi sur une carcasse adjacente ; il est retiré définitivement de cette carcasse."
                ),
                Some(TechniqueAction::DiagnoseComponent {
                    analysis_bonus,
                    energy_cost,
                }) => format!(
                    "{action_timing} / {energy_cost} E de fonctionnement. Analyse matérielle +{analysis_bonus} sur un composant accessible ; ne répare ni ne purge un logiciel."
                ),
                Some(TechniqueAction::TuneModule {
                    economy_output_percentage,
                    economy_energy_percentage,
                    power_output_percentage,
                    power_energy_percentage,
                }) => format!(
                    "{action_timing}. Économie : sortie {economy_output_percentage} %, énergie {economy_energy_percentage} %. Puissance : sortie {power_output_percentage} %, énergie {power_energy_percentage} %. Le nouveau réglage remplace l'ancien."
                ),
                Some(TechniqueAction::EmergencyRepairComponent {
                    durability_restored,
                }) => format!(
                    "{action_timing}. Restaure immédiatement {durability_restored} durabilité au plus ; un composant détruit reste détruit."
                ),
                Some(TechniqueAction::OverclockModule {
                    output_percentage,
                    usage_energy_percentage,
                    heat_per_use,
                    safe_heat_threshold,
                    maximum_heat_threshold,
                    duration_time_units,
                    activation_energy,
                    durability_damage_when_hot,
                }) => format!(
                    "{action_timing} / {activation_energy} E. Pendant {duration_time_units} UT : sortie {output_percentage} %, coût énergétique d'usage {usage_energy_percentage} %, +{heat_per_use} chaleur/usage, seuil volontaire {maximum_heat_threshold}. Au-dessus de {safe_heat_threshold}, −{durability_damage_when_hot} durabilité/usage."
                ),
                Some(TechniqueAction::BypassComponent {
                    restored_output_percentage,
                    energy_cost,
                }) => format!(
                    "{action_timing} / {energy_cost} E de fonctionnement. Rétablit une fonction électrique dégradée à {restored_output_percentage} % en suspendant un second composant réel du même corps."
                ),
                Some(TechniqueAction::ReconditionModule {
                    durability_restored,
                }) => format!(
                    "{action_timing} / atelier sûr. Restaure jusqu'à {durability_restored} durabilité propre sans dépasser le maximum réparable."
                ),
                Some(TechniqueAction::AssembleFieldBeacon {
                    integrity,
                    battery_energy,
                    energy_per_phase,
                    noise_intensity,
                }) => format!(
                    "{action_timing}. Manifeste sur une case libre une balise de {integrity} durabilité, batterie {battery_energy} E, consommation {energy_per_phase} E/phase, bruit {noise_intensity}."
                ),
                Some(TechniqueAction::WeaponAttack {
                    required_delivery,
                    physical_damage_percentage,
                    armor_penetration_bonus,
                    accuracy_modifier,
                    energy_cost,
                    recovery_time_units,
                    forced_movement,
                    melee_arc,
                }) => {
                    let delivery = match required_delivery {
                        project_rl::combat::AttackDelivery::Melee => "mêlée",
                        project_rl::combat::AttackDelivery::Ranged => "tir",
                    };
                    let recovery = recovery_time_units
                        .map_or_else(String::new, |duration| format!(" Puis R{duration}."));
                    let accuracy = match accuracy_modifier.cmp(&0) {
                        std::cmp::Ordering::Greater => {
                            format!(" Précision +{accuracy_modifier}.")
                        }
                        std::cmp::Ordering::Less => format!(" Précision {accuracy_modifier}."),
                        std::cmp::Ordering::Equal => String::new(),
                    };
                    let physical_damage = physical_damage_percentage
                        .map_or_else(String::new, |percentage| {
                            format!(" {percentage} % des dégâts physiques après Impact.")
                        });
                    let penetration = if armor_penetration_bonus > 0 {
                        format!(" Pénétration d'armure +{armor_penetration_bonus}.")
                    } else {
                        String::new()
                    };
                    let displacement = forced_movement.map_or_else(String::new, |movement| {
                        let modifier = match movement.impact_modifier().cmp(&0) {
                            std::cmp::Ordering::Greater => {
                                format!(" + {}", movement.impact_modifier())
                            }
                            std::cmp::Ordering::Less => {
                                format!(" − {}", movement.impact_modifier().unsigned_abs())
                            }
                            std::cmp::Ordering::Equal => String::new(),
                        };
                        format!(
                            " Sur une touche : poussée de {} case(s), Force = Impact{modifier}.",
                            movement.distance()
                        )
                    });
                    let arc = melee_arc.map_or_else(String::new, |arc| {
                        format!(
                            " Arc de {} cases adjacentes ; chaque occupant est exposé.",
                            arc.maximum_cells()
                        )
                    });
                    let on_hit_effect = definition.on_hit_effect().map_or_else(
                            String::new,
                            |effect| {
                                let target = match effect.target_requirement() {
                                    TechniqueTargetRequirement::HasArmor => {
                                        "une cible portant une armure"
                                    }
                                    TechniqueTargetRequirement::HasCompatibleLocomotion => {
                                        "une cible à locomotion compatible"
                                    }
                                    TechniqueTargetRequirement::HasCompatibleSuppressionResponse => {
                                        "une cible sensible à la suppression"
                                    }
                                };
                                let Some(status) = self
                                    .game
                                    .rules()
                                    .statuses
                                    .get(effect.application().status())
                                else {
                                    return format!(
                                        " Sur une touche contre {target} : applique {}.",
                                        status_display_name(effect.application().status())
                                    );
                                };
                                let duration = status.duration_turns().map_or_else(
                                    || "sans limite de durée".to_owned(),
                                    |turns| format!("pendant {turns} UT"),
                                );
                                let resistance = match effect.resistance() {
                                    Some(TechniqueEffectResistance::Stability { intensity }) => {
                                        format!(" Test passif de Stabilité contre intensité {intensity}.")
                                    }
                                    None => String::new(),
                                };
                                let description = status.modifiers().first().map_or_else(
                                    || {
                                        format!(
                                            " Sur une touche contre {target} : applique {} {duration}.",
                                            status_display_name(status.id())
                                        )
                                    },
                                    |modifier| match modifier {
                                        StatusModifier::DamageGuard { amount } => format!(
                                            " Protection de {amount} dégâts sur le prochain impact {duration}."
                                        ),
                                        StatusModifier::ArmorFragilization { amount } => format!(
                                            " Sur une touche contre {target} : armure fragilisée de {amount} {duration}, sans cumul ni rafraîchissement."
                                        ),
                                        StatusModifier::Stability { amount } => format!(
                                            " Sur une touche contre {target} : Stabilité {amount:+} {duration}."
                                        ),
                                        StatusModifier::MovementTimeMinimum { time_units } => format!(
                                            " Sur une touche contre {target} : déplacement ordinaire à {time_units} UT minimum {duration}."
                                        ),
                                        StatusModifier::Accuracy { amount } => format!(
                                            " Sur une touche contre {target} : Précision {amount:+} {duration}."
                                        ),
                                    },
                                );
                                format!("{description}{resistance}")
                            },
                        );
                    let engagement_requirement = definition
                            .engagement_requirement()
                            .map_or_else(String::new, |requirement| match requirement {
                                TechniqueEngagementRequirement::TargetHasAnyStatusFamily(_) => {
                                    " Requiert une cible déjà entravée ou immobilisée.".to_owned()
                                }
                                TechniqueEngagementRequirement::TargetHasKnownPhysicalWeakness => {
                                    " Requiert une faiblesse physique réellement identifiée sur cette cible."
                                        .to_owned()
                                }
                            });
                    let cost = if energy_cost == 0 {
                        "coût natif de l'arme".to_owned()
                    } else {
                        format!("coût natif de l'arme + {energy_cost} E")
                    };
                    let cooldown = definition.cooldown().map_or_else(String::new, |duration| {
                        format!(" Recharge : {} phases d'environnement.", duration.get())
                    });
                    format!(
                        "{action_timing} / {cost}. Attaque de {delivery}.{physical_damage}{penetration}{accuracy}{arc}{displacement}{on_hit_effect}{engagement_requirement}{recovery}{cooldown}"
                    )
                }
                Some(TechniqueAction::WeaponVolley {
                    projectiles,
                    maximum_targets,
                    maximum_target_separation,
                    accuracy_modifier,
                    energy_cost,
                    requires_automatic_fire,
                }) => {
                    let automatic = if requires_automatic_fire {
                        " Requiert un mode de tir automatique."
                    } else {
                        ""
                    };
                    let accuracy = match accuracy_modifier.cmp(&0) {
                        std::cmp::Ordering::Greater => {
                            format!(" Précision +{accuracy_modifier} par projectile.")
                        }
                        std::cmp::Ordering::Less => {
                            format!(" Précision {accuracy_modifier} par projectile.")
                        }
                        std::cmp::Ordering::Equal => String::new(),
                    };
                    let energy = if energy_cost == 0 {
                        String::new()
                    } else {
                        format!(" + {energy_cost} E")
                    };
                    let separation = maximum_target_separation.map_or_else(
                            String::new,
                            |maximum| {
                                format!(
                                    " Les cibles choisies doivent rester à {maximum} case(s) les unes des autres."
                                )
                            },
                        );
                    format!(
                        "{action_timing} / {projectiles} projectiles natifs{energy}. Jusqu'à {maximum_targets} cible(s) visible(s), une résolution indépendante par projectile.{accuracy}{automatic}{separation}"
                    )
                }
                Some(TechniqueAction::WeaponComponentAttack {
                    required_delivery,
                    accuracy_modifier,
                    energy_cost,
                }) => {
                    let delivery = match required_delivery {
                        project_rl::combat::AttackDelivery::Melee => "mêlée",
                        project_rl::combat::AttackDelivery::Ranged => "tir",
                    };
                    format!(
                        "{action_timing} / coût natif de l'arme + {energy_cost} E. Attaque de {delivery} contre la durabilité propre d'un composant identifié ; Précision {accuracy_modifier:+}. Les dégâts ne sont pas aussi appliqués aux PV du corps."
                    )
                }
                Some(TechniqueAction::WeaponBarrage {
                    stages,
                    cells,
                    accuracy_modifier,
                    energy_cost_per_stage,
                    requires_automatic_fire,
                }) => format!(
                    "{stages} étapes A1 / {cells} projectiles natifs par étape + {energy_cost_per_stage} E. Une balle par case de la ligne visée ; Précision {accuracy_modifier:+}.{} Toute autre action interrompt les étapes restantes sans annuler les tirs déjà résolus.",
                    if requires_automatic_fire {
                        " Requiert un mode automatique."
                    } else {
                        ""
                    }
                ),
                Some(TechniqueAction::PrepareRangedOverwatch { maximum_line_cells }) => {
                    format!(
                        "{action_timing} / un projectile natif au déclenchement. Désigne une ligne de {maximum_line_cells} case(s) dans la portée et la vision. Le premier ennemi perçu qui y entre déclenche un tir simple de réaction."
                    )
                }
                Some(TechniqueAction::PrepareMeleeParry {
                    physical_reduction_percentage,
                    trigger_energy_cost,
                }) => format!(
                    "{action_timing} / {trigger_energy_cost} E au déclenchement. Requiert une arme apte à parer. Réduit de {physical_reduction_percentage} % les dégâts physiques bruts de la prochaine touche de mêlée, avant l'armure."
                ),
                Some(TechniqueAction::PrepareMeleeInterception) => format!(
                    "{action_timing} / coût natif de l'arme au déclenchement. Prépare une frappe de mêlée ordinaire lorsqu'une cible au contact tente de s'éloigner. Une poussée ne la déclenche pas ; si la cible survit, elle parvient à s'éloigner."
                ),
                Some(TechniqueAction::PrepareControllingMeleeInterception {
                    stability_intensity,
                }) => format!(
                    "{action_timing} / coût natif de l'arme au déclenchement. Prépare une frappe de mêlée lorsqu'une cible au contact tente de s'éloigner. Sur une touche, sa Stabilité s'oppose à une intensité de {stability_intensity} : en cas d'échec, elle reste au contact. Une poussée ne déclenche jamais la garde."
                ),
                Some(TechniqueAction::DeployExplosive { deployment, .. }) => format!(
                    "{action_timing}. {} La zone d'effet est prévisualisée avant la manifestation.",
                    explosive_deployment_description(deployment)
                ),
                Some(TechniqueAction::NeutralizeExplosive {
                    range,
                    analysis_bonus,
                    energy_cost,
                }) => format!(
                    "{action_timing} / {energy_cost} E. Neutralise un dispositif identifié à portée {range}. Bonus d'analyse {analysis_bonus:+}."
                ),
                Some(TechniqueAction::RecoverNeutralizedExplosive { range }) => format!(
                    "{action_timing}. Récupère intacte la charge d'un dispositif déjà neutralisé à portée {range}, si l'inventaire peut la recevoir."
                ),
                Some(TechniqueAction::TriggerRemoteExplosive {
                    range,
                    energy_cost,
                    bandwidth_required,
                }) => format!(
                    "{action_timing} / {energy_cost} E · {bandwidth_required} B pendant la commande. Déclenche un récepteur identifié en liaison directe à portée {range}."
                ),
                Some(TechniqueAction::ProgramExplosives {
                    range,
                    maximum_devices,
                    minimum_delay,
                    maximum_delay,
                    energy_cost,
                    bandwidth_required,
                }) => format!(
                    "{action_timing} / {energy_cost} E · {bandwidth_required} B. Programme jusqu'à {maximum_devices} récepteurs connus à portée {range}, avec des délais de {minimum_delay} à {maximum_delay} UT."
                ),
                Some(TechniqueAction::CautiousMove {
                    interception_evasion_modifier,
                }) => format!(
                    "A1. Déplacement d'une case ; Esquive {interception_evasion_modifier:+} uniquement contre les interceptions provoquées lorsque vous quittez le corps à corps."
                ),
                Some(TechniqueAction::PrepareAnchor {
                    displacement_resistance_bonus,
                }) => format!(
                    "A1. Prépare un appui donnant Ancrage {displacement_resistance_bonus:+} tant que vous ne changez pas de case."
                ),
                Some(TechniqueAction::TraverseSingleObstacle {
                    maximum_distance,
                    energy_cost,
                }) => format!(
                    "{action_timing} / {energy_cost} E. Traverse un intervalle compatible d'une case vers une arrivée située à {maximum_distance} cases. Les murs et portes ne sont jamais franchissables."
                ),
                Some(TechniqueAction::ChargeAttack {
                    minimum_advance,
                    maximum_advance,
                    physical_damage_percentage,
                    energy_per_step,
                    recovery_time_units,
                }) => format!(
                    "Une avance par tour, de {minimum_advance} à {maximum_advance} cases, à {energy_per_step} E par case ; confirmez encore pour frapper à {physical_damage_percentage} % des dégâts physiques. Récupération {recovery_time_units} tour sans Inertie maîtrisée."
                ),
                Some(TechniqueAction::PrepareEvasiveStep {
                    trigger_energy_cost,
                }) => format!(
                    "A1 de garde. Choisissez une case adjacente ; la prochaine attaque perçue tente d'y déplacer le joueur pour {trigger_energy_cost} E et consomme la réaction commune."
                ),
                Some(TechniqueAction::PropelledMove {
                    distance,
                    energy_cost,
                    heat_generated,
                }) => format!(
                    "A1 / {energy_cost} E · +{heat_generated} H. Parcourt {distance} cases successives ; chaque entrée conserve ses dangers et tirs de Surveillance."
                ),
                Some(TechniqueAction::Breakthrough {
                    impact_modifier,
                    energy_cost,
                    recovery_time_units,
                }) => format!(
                    "{action_timing} / {energy_cost} E. Tente une poussée sans dégâts avec Impact {impact_modifier:+}, puis occupe la case libérée. Récupération {recovery_time_units} tour."
                ),
                Some(TechniqueAction::ExtractAlly { energy_cost }) => format!(
                    "{action_timing} / {energy_cost} E. Recule d'une case avec un allié adjacent coopératif et transportable ; aucun des deux ne reçoit d'action gratuite."
                ),
                Some(TechniqueAction::SilentMove {
                    noise_reduction,
                    minimum_time_units,
                }) => format!(
                    "A{minimum_time_units}. Avance d'une case avec une signature acoustique réduite de {noise_reduction}. Incompatible avec Profil réduit."
                ),
                Some(TechniqueAction::ToggleEmissionSilence { channel }) => format!(
                    "A1. Active ou désactive le silence des {}. Les actions qui en dépendent restent indisponibles tant que le silence est actif.",
                    signature_channel_label(channel)
                ),
                Some(TechniqueAction::ToggleLowProfile {
                    optical_difficulty_bonus,
                    minimum_movement_time_units,
                }) => format!(
                    "A1. Posture activable : difficulté optique +{optical_difficulty_bonus} sous couvert réel ; déplacements d'au moins {minimum_movement_time_units} UT."
                ),
                Some(TechniqueAction::AmbushAttack {
                    accuracy_modifier,
                    physical_damage_percentage,
                }) => format!(
                    "{action_timing}. Attaque simple contre une cible non alertée : Précision {accuracy_modifier:+}, {physical_damage_percentage} % des dégâts physiques. Si elle vous localise pendant la préparation, les bonus sont perdus."
                ),
                Some(TechniqueAction::DeploySoundDecoy {
                    range,
                    intensity,
                    duration_phases,
                    integrity,
                }) => format!(
                    "A1. Place à portée {range} un leurre physique (intensité {intensity}, durée {duration_phases} phases, intégrité {integrity}) qui attire les observateurs capables de l'entendre."
                ),
                Some(TechniqueAction::BreakTrail {
                    energy_cost,
                    maximum_steps,
                    maximum_duration,
                }) => format!(
                    "A1 / {energy_cost} E. Hors de toute détection optique, masque jusqu'à {maximum_steps} nouvelles traces pendant {maximum_duration} tours ; une nouvelle localisation interrompt l'effet."
                ),
                Some(TechniqueAction::CamouflageExplosive {
                    range,
                    optical_difficulty_bonus,
                }) => format!(
                    "{action_timing}. Génère une couverture donnant +{optical_difficulty_bonus} de difficulté optique à un explosif identifié et non déclenché à portée {range}."
                ),
                Some(TechniqueAction::ToggleActiveCamouflage {
                    channel,
                    optical_difficulty_bonus,
                    maximum_duration,
                    activation_energy,
                    upkeep_energy,
                    heat_per_phase,
                }) => format!(
                    "A1 / {activation_energy} E, puis {upkeep_energy} E et +{heat_per_phase} H par phase. Utilise les {} : difficulté optique +{optical_difficulty_bonus}, au plus {maximum_duration} phases ; une attaque l'interrompt.",
                    signature_channel_label(channel)
                ),
                Some(TechniqueAction::ManifestDrone {
                    integrity,
                    energy_capacity,
                    link_range,
                    sensor_radius,
                    bandwidth_required,
                    attack_range,
                    attack_damage,
                    ..
                }) => format!(
                    "A1. Manifeste sur une case adjacente un drone physique de {integrity} intégrité et {energy_capacity} E. Liaison ≤{link_range}, capteurs {sensor_radius}, {bandwidth_required} B réservée ; attaque électrique {attack_damage} à portée {attack_range}."
                ),
                Some(TechniqueAction::DroneEscort {
                    link_range,
                    minimum_distance,
                    maximum_distance,
                    energy_cost,
                }) => format!(
                    "A1 / {energy_cost} E. Donne à un drone physique en liaison ≤{link_range} un ordre d'escorte à une distance choisie de {minimum_distance} à {maximum_distance} cases."
                ),
                Some(TechniqueAction::DronePatrol {
                    link_range,
                    maximum_waypoints,
                    energy_cost,
                }) => format!(
                    "P1+A1 / {energy_cost} E. Programme jusqu'à {maximum_waypoints} points connus sur un drone en liaison ≤{link_range}. Le drone suit ensuite la route avec ses propres actions."
                ),
                Some(TechniqueAction::DroneMobileDecoy {
                    link_range,
                    controller_energy_cost,
                    drone_energy_per_phase,
                    intensity,
                    maximum_duration,
                }) => format!(
                    "A1 / {controller_energy_cost} E. Un drone équipé rejoint une destination connue en liaison ≤{link_range}, puis dépense {drone_energy_per_phase} E/phase pour émettre un leurre d'intensité {intensity}, au plus {maximum_duration} phases."
                ),
                Some(TechniqueAction::DroneCollect {
                    link_range,
                    energy_cost,
                }) => format!(
                    "A1 / {energy_cost} E. Ordonne à un drone en liaison ≤{link_range} de rejoindre un objet connu, de le charger réellement, puis de revenir pour une remise adjacente."
                ),
                Some(TechniqueAction::DroneCoordinateFire {
                    link_range,
                    maximum_drones,
                    energy_cost,
                    transmission_bandwidth,
                }) => format!(
                    "A1 / {energy_cost} E, +{transmission_bandwidth} B pendant l'émission. Jusqu'à {maximum_drones} drones en liaison ≤{link_range} viseront la cible lors de leur prochaine attaque ordinaire."
                ),
                Some(TechniqueAction::DroneInterpose {
                    link_range,
                    controller_energy_cost,
                    drone_trigger_energy_cost,
                }) => format!(
                    "A1 / {controller_energy_cost} E. Un drone compatible en liaison ≤{link_range} rejoint l'allié et peut intercepter un tir simple perçu pour {drone_trigger_energy_cost} E propres."
                ),
                Some(TechniqueAction::DroneConditionalRoutine {
                    link_range,
                    energy_cost,
                    additional_bandwidth,
                }) => format!(
                    "P1+A1 / {energy_cost} E, +{additional_bandwidth} B durant la routine. Programme une condition locale bornée sur un drone en liaison ≤{link_range}, sans boucle ni information globale."
                ),
                Some(TechniqueAction::DroneCoordinatedDeployment {
                    link_range,
                    maximum_drones,
                    energy_cost,
                    transmission_bandwidth,
                }) => format!(
                    "P1+A1 / {energy_cost} E, +{transmission_bandwidth} B pendant l'émission. Assigne à jusqu'à {maximum_drones} drones en liaison ≤{link_range} des destinations et rôles réels, sans déplacement instantané."
                ),
                Some(TechniqueAction::DroneEmergencyReturn {
                    link_range,
                    maximum_drones,
                    energy_cost,
                    transmission_bandwidth,
                    duration_phases,
                }) => format!(
                    "A1 / {energy_cost} E, +{transmission_bandwidth} B pendant l'émission. Jusqu'à {maximum_drones} drones en liaison ≤{link_range} reviennent durant {duration_phases} phases, puis attendent."
                ),
                Some(TechniqueAction::ProbeInterface {
                    range,
                    analysis_bonus,
                    energy_cost,
                    audit_delay,
                }) => format!(
                    "A1 / {energy_cost} E. Sonde une interface connue en liaison ≤{range}, Analyse +{analysis_bonus}. Crée une trace locale dont l'audit de référence arrive après {audit_delay} UT."
                ),
                Some(TechniqueAction::ForceElectronicLock {
                    range,
                    energy_cost,
                    bandwidth_required,
                    failure_hardening_duration,
                    ..
                }) => format!(
                    "P1+A1 / {energy_cost} E, {bandwidth_required} B pendant la procédure. Tente une ouverture électronique en liaison ≤{range}. Un échec durcit l'interface jusqu'à +20 pendant {failure_hardening_duration} UT."
                ),
                Some(TechniqueAction::ExtractData { range, energy_cost }) => format!(
                    "P1+A1 / {energy_cost} E. Extrait d'une session de lecture à portée {range} un lot persistant, daté et rattaché à sa source."
                ),
                Some(TechniqueAction::SpoofAuthorization {
                    range,
                    energy_cost,
                    duration_time_units,
                }) => format!(
                    "A1 / {energy_cost} E, 1 B de session. Utilise un identifiant déjà extrait à portée {range} et accorde seulement les droits locaux pendant {duration_time_units} UT."
                ),
                Some(TechniqueAction::DivertDevice {
                    range,
                    energy_cost,
                    additional_bandwidth,
                    duration_time_units,
                }) => format!(
                    "A1 / {energy_cost} E, +{additional_bandwidth} B. Maintient une consigne simple autorisée à portée {range} pendant {duration_time_units} UT ; une reprise adverse reste possible."
                ),
                Some(TechniqueAction::SuspendDigitalRoutine {
                    range,
                    energy_cost,
                    duration_time_units,
                    repeat_protection_time_units,
                }) => format!(
                    "A1 / {energy_cost} E, recharge 3. Suspend une routine nommée à portée {range} pendant {duration_time_units} UT, puis protège cette famille {repeat_protection_time_units} UT."
                ),
                Some(TechniqueAction::MaintainBackdoor {
                    range,
                    installation_energy_cost,
                    reconnection_energy_cost,
                    maximum_backdoors,
                    session_duration_time_units,
                }) => format!(
                    "P1+A1 / {installation_energy_cost} E à l'installation, {reconnection_energy_cost} E à la reconnexion et 1 B de session. Jusqu'à {maximum_backdoors} accès dormants, liaison ≤{range}, session {session_duration_time_units} UT."
                ),
                Some(TechniqueAction::FalsifySecurityTrace { range, energy_cost }) => format!(
                    "P1+A1 / {energy_cost} E. Avec droit de modification à portée {range}, falsifie une trace identifiée avant son audit ; copies, témoins et transmissions subsistent."
                ),
                Some(TechniqueAction::DivertSubnet {
                    range,
                    maximum_devices,
                    energy_cost,
                    bandwidth_per_device,
                    duration_time_units,
                }) => format!(
                    "P2+A1 / {energy_cost} E, {bandwidth_per_device} B par dispositif. Commande jusqu'à {maximum_devices} interfaces autorisées et joignables à portée {range} pendant {duration_time_units} UT."
                ),
                Some(TechniqueAction::LockDeviceControl {
                    range,
                    energy_cost,
                    additional_bandwidth,
                    duration_time_units,
                }) => format!(
                    "A1 / {energy_cost} E, +{additional_bandwidth} B, recharge 4. Bloque pendant {duration_time_units} UT les reprises ordinaires d'un dispositif déjà détourné à portée {range}."
                ),
                Some(TechniqueAction::ElectronicPulse {
                    radius,
                    damage,
                    energy_cost,
                    heat_generated,
                    disruption_intensity,
                    directional,
                    filter_identified_allies,
                    bandwidth_required,
                }) => format!(
                    "A1 / {energy_cost} E, +{heat_generated} H{}{}. {} de rayon {radius} : {damage} dégâts électriques aux systèmes compatibles ; interruption d'intensité {disruption_intensity}. Les parois bloquent la propagation.",
                    if bandwidth_required > 0 {
                        format!(", {bandwidth_required} B pendant l'émission")
                    } else {
                        String::new()
                    },
                    if filter_identified_allies {
                        ", alliés identifiés filtrés"
                    } else {
                        ""
                    },
                    if directional {
                        "Cône de 90°"
                    } else {
                        "Disque"
                    },
                ),
                Some(TechniqueAction::ImplantOverheat {
                    range,
                    energy_cost,
                    heat_generated,
                    bandwidth_required,
                    heat_per_tick,
                    dissipation_penalty,
                    duration_time_units,
                    ..
                }) => format!(
                    "A1 / {energy_cost} E, +{heat_generated} H, {bandwidth_required} B pendant la tentative. Machine visible à portée {range} : test logiciel, puis +{heat_per_tick} H et dissipation −{dissipation_penalty} durant {duration_time_units} UT."
                ),
                Some(TechniqueAction::MaintainJamming {
                    radius,
                    penalty,
                    activation_energy,
                    energy_per_phase,
                    heat_per_phase,
                    bandwidth_required,
                    maximum_duration,
                }) => format!(
                    "A1 / {activation_energy} E, puis {energy_per_phase} E et +{heat_per_phase} H par phase, {bandwidth_required} B. Brouille un canal choisi dans un rayon de {radius} avec un malus de {penalty}, au plus {maximum_duration} UT."
                ),
                Some(TechniqueAction::PurgeHostileProgram {
                    range,
                    energy_cost,
                    intrusion_bonus,
                }) => format!(
                    "A1 / {energy_cost} E. À portée {range}, tente de retirer un programme hostile précis avec un bonus logiciel de {intrusion_bonus:+}."
                ),
                Some(TechniqueAction::ElectronicCascade {
                    range,
                    jump_range,
                    maximum_targets,
                    damage_by_target,
                    energy_cost,
                    heat_generated,
                }) => format!(
                    "A1 / {energy_cost} E, +{heat_generated} H. Première machine visible à portée {range}, puis jusqu'à {maximum_targets} cibles distinctes séparées de {jump_range} cases : dégâts électriques successifs {}. Chaque saut exige une liaison libre.",
                    damage_sequence_label(
                        &damage_by_target
                            [..usize::from(maximum_targets).min(damage_by_target.len())]
                    )
                ),
                Some(TechniqueAction::ImplantInfection {
                    range,
                    propagation_range,
                    energy_cost,
                    heat_generated,
                    bandwidth_required,
                    thermal_damage_per_tick,
                    ticks_per_host,
                    maximum_hosts,
                    transmissions_per_host,
                    campaign_duration,
                    ..
                }) => format!(
                    "A1 / {energy_cost} E, +{heat_generated} H, {bandwidth_required} B pendant la tentative. Infection logicielle à portée {range} : {thermal_damage_per_tick} thermiques pendant {ticks_per_host} UT par hôte, jusqu'à {maximum_hosts} hôtes. Chaque hôte tente {transmissions_per_host} transmission(s) distincte(s) à portée {propagation_range}, campagne {campaign_duration} UT."
                ),
                Some(TechniqueAction::DeploySaturationBeacon {
                    radius,
                    damage,
                    duration_time_units,
                    integrity,
                    battery_energy,
                    energy_per_phase,
                    manual_activation,
                    activation_energy,
                    activation_bandwidth,
                    activation_link_range,
                }) => format!(
                    "P1+A1 / une balise. Acteur physique de {integrity} intégrité sur une case adjacente : disque bloqué par les murs, {damage} électriques, rayon {radius}, {duration_time_units} UT, batterie {battery_energy} E à {energy_per_phase} E/phase.{}",
                    if manual_activation {
                        format!(
                            " Pose inactive ; activation ultérieure à portée {activation_link_range} pour {activation_energy} E et {activation_bandwidth} B temporaire."
                        )
                    } else {
                        String::new()
                    }
                ),
                Some(TechniqueAction::ImplantImplosion {
                    range,
                    energy_cost,
                    heat_generated,
                    bandwidth_required,
                    minimum_stored_energy,
                    reserved_energy,
                    delay_time_units,
                    radius,
                    physical_damage,
                    thermal_damage,
                    ..
                }) => format!(
                    "P1+A1 / {energy_cost} E, +{heat_generated} H, {bandwidth_required} B pendant la tentative. Réserve {reserved_energy} E dans une machine possédant au moins {minimum_stored_energy} E à portée {range}, puis annonce une détonation après {delay_time_units} UT : rayon {radius}, {physical_damage} physiques + {thermal_damage} thermiques."
                ),
                None => "Fonction indisponible dans cette version.".to_owned(),
            }
        };
        let usage = definition.activation_cost().map_or(usage.clone(), |cost| {
            let mut resources = Vec::new();
            if cost.energy() > 0 {
                resources.push(format!("{} E", cost.energy()));
            }
            if cost.heat() > 0 {
                resources.push(format!("+{} H", cost.heat()));
            }
            if cost.persistent_bandwidth() > 0 {
                resources.push(format!("{} B réservée", cost.persistent_bandwidth()));
            }
            if let Some(limit) = cost.active_limit() {
                resources.push(format!("limite active {limit}"));
            }
            format!("COÛT INTRINSÈQUE : {}. {usage}", resources.join(" · "))
        });

        usage
            .replace("P1+A1", "Préparation 1 tour + action 1 tour")
            .replace("P2+A1", "Préparation 2 tours + action 1 tour")
            .replace("R1", "récupération 1 tour")
            .replace("R2", "récupération 2 tours")
    }
}
