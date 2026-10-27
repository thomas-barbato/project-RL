//! Equipped slots and contextual item details in the ordinary inventory.
use super::*;

fn phosphor() -> Color {
    UiTheme.accent()
}
fn dim() -> Color {
    UiTheme.muted()
}
fn ink() -> Color {
    crate::terminal_chrome::ink()
}
const SLOT_LABELS: [&str; 7] = [
    "TÊTE",
    "CORPS",
    "MAINS",
    "PIEDS",
    "Arme primaire",
    "Arme secondaire",
    "Arme de secours",
];
const BODY_POINTS: [Vec2; 4] = [
    vec2(16.0, 2.7),
    vec2(20.0, 12.0),
    vec2(8.0, 18.8),
    vec2(19.2, 29.5),
];
pub(super) const BODY_APPEARANCE_INDEX: usize = 7;
const BODY_APPEARANCE_LABEL: &str = "Transmogrification";

pub(super) struct EquipmentLayout {
    pub panel: Rect,
    pub silhouette: Rect,
    pub slots: [Rect; 8],
}

impl EquipmentLayout {
    pub(super) fn new(layout: &InventoryLayout, width: f32, height: f32) -> Self {
        let right = layout.stats_panel.map_or(width - 32.0, |panel| panel.x);
        let x = 32.0 + layout.left_width + 12.0;
        let panel = Rect::new(x, 100.0, right - x - 12.0, (height - 214.0).max(216.0));
        let weapon_h = if panel.h > 400.0 { 74.0 } else { 56.0 };
        let body = Rect::new(panel.x, panel.y + 38.0, panel.w, panel.h - weapon_h - 52.0);
        let slot_w = (panel.w * 0.19).clamp(64.0, 108.0);
        let slot_h = (body.h * 0.23).clamp(36.0, 86.0);
        let silhouette_h = (body.h - 12.0).min(380.0);
        let silhouette_w = (silhouette_h * 280.0 / 380.0).min(panel.w - 2.0 * (slot_w + 9.0));
        let silhouette = Rect::new(
            body.x + (body.w - silhouette_w) * 0.5,
            body.y + (body.h - silhouette_h) * 0.5,
            silhouette_w,
            silhouette_h,
        );
        let slots = std::array::from_fn(|index| {
            if index == BODY_APPEARANCE_INDEX {
                Rect::new(
                    panel.right() - slot_w,
                    body.y + body.h * 0.01,
                    slot_w,
                    slot_h,
                )
            } else if index < 4 {
                let right = index == 1 || index == 3;
                let offset = [0.01, 0.28, 0.57, 0.80][index];
                Rect::new(
                    if right {
                        panel.right() - slot_w
                    } else {
                        panel.x
                    },
                    (body.y + body.h * offset).min(body.bottom() - slot_h),
                    slot_w,
                    slot_h,
                )
            } else {
                let w = (panel.w - 16.0) / 3.0;
                Rect::new(
                    panel.x + (index - 4) as f32 * (w + 8.0),
                    panel.bottom() - weapon_h,
                    w,
                    weapon_h,
                )
            }
        });
        Self {
            panel,
            silhouette,
            slots,
        }
    }

    pub(super) fn hit(&self, pointer: (f32, f32)) -> Option<usize> {
        self.slots
            .iter()
            .position(|slot| slot.contains(pointer.into()))
    }

    pub(super) fn weapon_set_switch(&self) -> Rect {
        let r = self.slots[4];
        Rect::new(
            self.panel.center().x - 18.,
            r.y + (r.h - 36.) * 0.5,
            36.,
            36.,
        )
    }
    pub(super) fn for_weapon_sets(mut self, enabled: bool) -> Self {
        if enabled {
            let w = (self.panel.w - 64.) * 0.5;
            let y = self.slots[4].y;
            let h = self.slots[4].h;
            self.slots[4] = Rect::new(self.panel.right() - w, y, w, h);
            self.slots[5] = Rect::new(self.panel.x, y, w, h);
            self.slots[6] = Rect::new(-100., -100., 0., 0.);
        }
        self
    }

    pub(super) fn appearance_clear(&self) -> Rect {
        let slot = self.slots[BODY_APPEARANCE_INDEX];
        Rect::new(slot.right() - 18.0, slot.y + 3.0, 15.0, 15.0)
    }

    pub(super) fn for_hands(mut self, enabled: bool) -> Self {
        if enabled {
            let [left, middle, right] = [self.slots[4], self.slots[5], self.slots[6]];
            self.slots[4] = right; // Character's left hand is on the screen's right.
            self.slots[5] = left;
            self.slots[6] = middle;
        }
        self
    }
}

pub(super) fn terminal_button(rect: Rect, label: &str, focused: bool, active: bool, enabled: bool) {
    crate::terminal_chrome::button(rect, focused, active, enabled);
    let color = if enabled { phosphor() } else { dim() };
    if measure_text_bold(label, 14).width > rect.w - 8.0
        && let Some((title, binding)) = label.rsplit_once(" [")
    {
        draw_text_bold_centered(
            title,
            Rect::new(rect.x + 2.0, rect.y + 1.0, rect.w - 4.0, 15.0),
            12,
            color,
        );
        draw_text_bold_centered(
            format!("[{binding}"),
            Rect::new(rect.x + 2.0, rect.y + 16.0, rect.w - 4.0, rect.h - 17.0),
            11,
            color,
        );
    } else {
        draw_text_bold_centered(
            label,
            Rect::new(rect.x + 2.0, rect.y, rect.w - 4.0, rect.h),
            if rect.w < 75.0 { 11 } else { 14 },
            color,
        );
    }
}

impl AsciiApp {
    pub(super) fn preview_selected_inventory_item(&mut self) {
        self.ux.inventory_keyboard_preview = true;
        self.ux.inventory_tooltip = self
            .inventory_entries()
            .get(self.inventory_selection)
            .map(|entry| entry.instance());
        self.ux.inventory_tooltip_scroll.offset = 0.0;
        self.ux.inventory_tooltip_scroll.maximum.set(0.0);
    }
    pub(super) fn inventory_slot_instance(&self, index: usize) -> Option<ItemInstanceId> {
        if index == 6 && self.game.rules().uses_weapon_sets() {
            return None;
        }
        if index == BODY_APPEARANCE_INDEX {
            self.game.player_equipment().body_appearance()
        } else if index < 4 {
            let id: ContentId = [
                "core:head_armor",
                "core:body_armor",
                "core:hand_armor",
                "core:foot_armor",
            ][index]
                .parse()
                .unwrap();
            self.game.player_equipment().equipped(&id)
        } else {
            self.game
                .equipped_player_weapon_item(self.visible_hand_slot(index - 4))
        }
    }

    pub(super) fn inventory_item_is_torso(&self, item: ItemInstanceId) -> bool {
        self.game
            .player_inventory()
            .get(item)
            .and_then(|entry| self.game.rules().items.get(entry.item()))
            .and_then(project_rl::item::ItemDefinition::equipment)
            .is_some_and(|profile| profile.slot().as_str() == "core:body_armor")
    }

    pub(super) fn update_inventory_body_appearance(&mut self, item: ItemInstanceId) {
        let outcome = self.execute_command(GameCommand::EquipItem {
            slot: project_rl::entity::BODY_APPEARANCE_SLOT
                .parse()
                .expect("static appearance slot"),
            item,
        });
        self.inventory_message = match outcome {
            CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime => {
                BODY_APPEARANCE_LABEL.to_owned()
            }
            CommandOutcome::Rejected(reason) => command_rejection_message(reason).to_owned(),
        };
        self.capture_events();
        self.select_inventory_instance(item);
    }

    pub(super) fn inventory_tooltip_rect(
        layout: &InventoryLayout,
        width: f32,
        height: f32,
    ) -> Rect {
        if let Some(stats) = layout.stats_panel {
            Rect::new(
                stats.x + 8.0,
                stats.y + 245.0,
                stats.w - 16.0,
                stats.h - 294.0,
            )
        } else {
            let equipment = EquipmentLayout::new(layout, width, height);
            let top = (equipment.slots[BODY_APPEARANCE_INDEX].bottom() + 8.0)
                .max(equipment.panel.y + 32.0);
            Rect::new(
                equipment.panel.x + 8.0,
                top,
                equipment.panel.w - 16.0,
                equipment.panel.bottom() - 8.0 - top,
            )
        }
    }

    // Only presentation state changes here. Merely visiting a slot cannot equip an item.
    pub(super) fn update_inventory_tooltip(
        &mut self,
        input: &InputFrame,
        layout: &InventoryLayout,
    ) -> bool {
        let (width, height) = input.viewport.unwrap_or((1280.0, 800.0));
        let equipment = EquipmentLayout::new(layout, width, height)
            .for_hands(self.game.rules().uses_weapon_hands())
            .for_weapon_sets(self.game.rules().uses_weapon_sets());
        let previous = self.ux.inventory_tooltip;
        let pointer_changed = input.pointer != self.ux.inventory_pointer;
        self.ux.inventory_pointer = input.pointer;
        if pointer_changed {
            self.ux.inventory_keyboard_preview = false;
        }
        if !self.ux.inventory_keyboard_preview {
            let in_tooltip = input.pointer.is_some_and(|pointer| {
                Self::inventory_tooltip_rect(layout, width, height).contains(pointer.into())
            });
            let item = input.pointer.and_then(|pointer| {
                if in_tooltip && previous.is_some() && layout.stats_panel.is_none() {
                    return previous;
                }
                layout
                    .rows
                    .iter()
                    .find(|(_, row)| row.contains(pointer.into()))
                    .and_then(|(index, _)| {
                        self.inventory_entries()
                            .get(*index)
                            .map(|entry| entry.instance())
                    })
                    .or_else(|| {
                        equipment
                            .hit(pointer)
                            .and_then(|slot| self.inventory_slot_instance(slot))
                    })
            });
            self.ux.inventory_tooltip = item.or_else(|| in_tooltip.then_some(previous).flatten());
        }
        if self.ux.inventory_tooltip != previous {
            self.ux.inventory_tooltip_scroll.offset = 0.0;
            self.ux.inventory_tooltip_scroll.maximum.set(0.0);
        }
        let scroll_tooltip = self.ux.inventory_tooltip.is_some()
            && input.pointer.is_some_and(|pointer| {
                !layout.list.contains(pointer.into())
                    && (Self::inventory_tooltip_rect(layout, width, height)
                        .contains(pointer.into())
                        || equipment.hit(pointer).is_some())
            })
            && input.wheel_y != 0.0;
        if scroll_tooltip {
            let scroll = &mut self.ux.inventory_tooltip_scroll;
            scroll.offset = (scroll.offset - input.wheel_y * 36.0).clamp(0.0, scroll.maximum.get());
        }
        scroll_tooltip
    }

    pub(super) fn draw_inventory_equipment(&self, equipment: &EquipmentLayout) {
        let class = self
            .character_class
            .as_ref()
            .and_then(|id| self.character_classes.get(id))
            .and_then(|class| self.texts.resolve(DISPLAY_LOCALE, class.name_key()))
            .unwrap_or("Personnage");
        crate::terminal_chrome::module(
            equipment.panel,
            &format!("02 / PERSONNAGE / {}", class.to_uppercase()),
        );
        crate::game_art::draw_terminal_wireframe(
            equipment.silhouette,
            self.front_player_visual(),
            phosphor(),
        );
        for (index, rect) in equipment.slots.iter().copied().enumerate() {
            if (4..=6).contains(&index) && self.game.rules().uses_weapon_sets() {
                continue;
            }
            let instance = self.inventory_slot_instance(index);
            let entry = instance.and_then(|id| self.game.player_inventory().get(id));
            let hovered = instance.is_some() && instance == self.ux.inventory_tooltip;
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                if hovered {
                    UiTheme.surface_selected()
                } else {
                    ink()
                },
            );
            draw_rectangle_lines(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                1.0,
                if hovered { phosphor() } else { dim() },
            );
            let side = (rect.h - 25.0).min(56.0);
            let image = Rect::new(rect.x + (rect.w - side) * 0.5, rect.y + 3.0, side, side);
            if let Some(entry) = entry {
                if !crate::game_art::draw_item_variant(
                    image,
                    entry.item().as_str(),
                    entry.palette(),
                ) {
                    draw_inventory_glyph(image, self.inventory_glyph(entry), phosphor());
                }
            } else {
                crate::ui_theme::draw_text_bold_centered("Vide", image, 12, dim());
            }
            if index == BODY_APPEARANCE_INDEX {
                let width = measure_text(BODY_APPEARANCE_LABEL, None, 12, 1.0).width;
                draw_text(
                    BODY_APPEARANCE_LABEL,
                    rect.right() - width,
                    rect.y - 5.0,
                    12.0,
                    phosphor(),
                );
                if instance.is_some() {
                    draw_ui_icon(UiIcon::Cancel, equipment.appearance_clear(), dim());
                }
                continue;
            }
            crate::ui_theme::draw_text_in_rect(
                if index < 4 {
                    SLOT_LABELS[index].to_owned()
                } else {
                    self.hand_slot_label(index - 4)
                },
                Rect::new(rect.x + 5.0, rect.bottom() - 23.0, rect.w - 10.0, 20.0),
                14,
                phosphor(),
            );
            if index >= 4 && self.weapon_configuration_active(index - 4) {
                draw_rectangle(
                    rect.x + 2.0,
                    rect.bottom() - 2.0,
                    rect.w - 4.0,
                    2.0,
                    phosphor(),
                );
            }
            if index < 4 {
                let point = BODY_POINTS[index];
                let target = vec2(
                    equipment.silhouette.x + point.x * equipment.silhouette.w / 32.0,
                    equipment.silhouette.y + point.y * equipment.silhouette.h / 32.0,
                );
                let start = vec2(
                    if index == 0 || index == 2 {
                        rect.right()
                    } else {
                        rect.x
                    },
                    rect.y + rect.h * 0.47,
                );
                let elbow = (start.x + target.x) * 0.5;
                draw_line(start.x, start.y, elbow, start.y, 1.0, dim());
                draw_line(elbow, start.y, elbow, target.y, 1.0, dim());
                draw_line(elbow, target.y, target.x, target.y, 1.0, dim());
                draw_circle(target.x, target.y, 2.0, phosphor());
            }
            if index >= 4 && self.game.rules().uses_weapon_hands() {
                let line_color = if self.weapon_configuration_active(index - 4) {
                    phosphor()
                } else {
                    dim()
                };
                let point = |x: f32, y: f32| {
                    vec2(
                        equipment.silhouette.x + x * equipment.silhouette.w / 32.0,
                        equipment.silhouette.y + y * equipment.silhouette.h / 32.0,
                    )
                };
                let start = vec2(rect.x + rect.w * 0.5, rect.y - 4.0);
                let targets: &[f32] = match index {
                    4 => &[24.0],
                    5 => &[8.0],
                    _ => &[8.0, 24.0],
                };
                for &x in targets {
                    let target = point(x, 18.8);
                    let route_x = if index == 6 {
                        start.x
                    } else if index == 4 {
                        equipment.silhouette.right() + 4.0
                    } else {
                        equipment.silhouette.x - 4.0
                    };
                    draw_line(start.x, start.y, route_x, start.y, 1.0, line_color);
                    draw_line(route_x, start.y, route_x, target.y, 1.0, line_color);
                    draw_line(route_x, target.y, target.x, target.y, 1.0, line_color);
                    draw_circle(target.x, target.y, 2.0, line_color);
                }
            }
        }
        super::weapon_sets_app::draw_row(self, equipment);
    }

    pub(super) fn inventory_resource_gauges(&self) -> Vec<hud_resources::ResourceGauge> {
        let mut gauges: Vec<_> = self
            .resource_gauges()
            .into_iter()
            .filter(|g| matches!(g.label, "PV" | "Énergie"))
            .collect();
        if let Some(heat) = self.heat_resource_gauge() {
            gauges.push(heat);
        }
        gauges.push(self.experience_resource_gauge());
        gauges
    }

    pub(super) fn draw_inventory_gauges(&self, panel: Rect) {
        let state = Rect::new(panel.x + 8.0, panel.y, panel.w - 16.0, 235.0);
        crate::terminal_chrome::module(state, "03 / ÉTAT DU PERSONNAGE");
        for (i, gauge) in self.inventory_resource_gauges().iter().enumerate() {
            Self::draw_resource_gauge(
                Rect::new(
                    panel.x + 20.0,
                    panel.y + 39.0 + i as f32 * 42.0,
                    panel.w - 40.0,
                    38.0,
                ),
                gauge,
            );
        }
        crate::ui_theme::draw_text_in_rect(
            format!(
                "{} points de compétence",
                self.game.player_progression().unspent_skill_points()
            ),
            Rect::new(panel.x + 20.0, panel.y + 211.0, panel.w - 40.0, 18.0),
            13,
            dim(),
        );
    }

    pub(super) fn draw_inventory_tooltip(&self, layout: &InventoryLayout) {
        let Some(instance) = self.ux.inventory_tooltip else {
            return;
        };
        let Some(entry) = self.game.player_inventory().get(instance) else {
            return;
        };
        let rect = Self::inventory_tooltip_rect(layout, self.ui_width(), self.ui_height());
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, ink());
        crate::terminal_chrome::bezel(rect);
        let glyph = Rect::new(rect.right() - 55.0, rect.y + 8.0, 46.0, 46.0);
        crate::game_art::draw_item_variant(glyph, entry.item().as_str(), entry.palette());
        let title = self.inventory_entry_name(entry);
        let title_end = draw_wrapped_text(
            &title,
            rect.x + 10.0,
            rect.y + 23.0,
            rect.w - 74.0,
            3,
            17,
            equipment_affix_names::rarity_color(entry.magic_modifiers().as_ref()),
        );
        let body_y = (rect.y + 75.0).max(title_end + 8.0);
        let body = Rect::new(
            rect.x + 10.0,
            body_y,
            rect.w - 22.0,
            rect.bottom() - 27.0 - body_y,
        );
        draw_line(
            rect.x + 8.0,
            body.y - 8.0,
            rect.right() - 8.0,
            body.y - 8.0,
            1.0,
            dim(),
        );
        crate::ui_theme::begin_text_pane(body, self.ux.inventory_tooltip_scroll.offset);
        let mut y = body.y + 17.0;
        for line in self.inventory_item_lines(instance) {
            y = draw_wrapped_text(&line, body.x, y, body.w - 5.0, 40, 15, UiTheme.text()) + 8.0;
        }
        crate::ui_theme::end_text_pane();
        let scroll = &self.ux.inventory_tooltip_scroll;
        let maximum = (y - body.bottom() + 12.0).max(0.0);
        scroll.maximum.set(maximum);
        if maximum > 0.0 {
            let track = Rect::new(body.right() - 3.0, body.y, 2.0, body.h);
            let thumb = (track.h * body.h / (maximum + body.h)).max(20.0);
            draw_rectangle(track.x, track.y, track.w, track.h, dim());
            draw_rectangle(
                track.x,
                track.y + (track.h - thumb) * scroll.offset.min(maximum) / maximum,
                track.w,
                thumb,
                phosphor(),
            );
        }
        crate::ui_theme::draw_text_in_rect(
            format!(
                "Molette · {} : détails / comparer",
                self.controls.label(Action::Inspect)
            ),
            Rect::new(rect.x + 8.0, rect.bottom() - 24.0, rect.w - 16.0, 20.0),
            12,
            dim(),
        );
    }
}

#[cfg(all(test, debug_assertions))]
mod tests {
    use super::*;

    fn fixture() -> AsciiApp {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.prepare_cyberpunk_diagnostic("terminal-inventory-live")
            .unwrap();
        app.controls = Controls::preset(controls::Layout::Qwerty, controls::KeySemantics::Physical);
        app.inventory_open = true;
        app
    }

    #[test]
    fn inventory_gauges_keep_resource_colors_in_both_interface_styles() {
        let app = fixture();
        let before = suspension::fingerprint(&app.game);
        for style in graphics::InterfaceStyle::ALL {
            crate::ui_theme::set_interface_style(style);
            let inventory = app.inventory_resource_gauges();
            let hud = app.resource_gauges();
            for label in ["PV", "Énergie"] {
                let a = inventory.iter().find(|g| g.label == label).unwrap();
                let b = hud.iter().find(|g| g.label == label).unwrap();
                assert_eq!(
                    (a.value.as_str(), a.ratio, a.reserved_ratio, a.color),
                    (b.value.as_str(), b.ratio, b.reserved_ratio, b.color)
                );
            }
            assert_eq!(inventory[0].color, UiTheme.success());
            assert_eq!(inventory[1].color, Color::from_rgba(109, 186, 255, 255));
            assert_eq!(inventory.last().unwrap().color, UiTheme.accent());
            if let Some(heat) = app.heat_resource_gauge() {
                let actual = inventory.iter().find(|g| g.label == "Chaleur").unwrap();
                assert_eq!(
                    (actual.value.as_str(), actual.ratio, actual.color),
                    (heat.value.as_str(), heat.ratio, heat.color)
                );
            }
            assert_eq!(suspension::fingerprint(&app.game), before);
        }
        crate::ui_theme::set_interface_style(graphics::InterfaceStyle::default());
    }

    fn layout(app: &AsciiApp, width: f32, height: f32) -> InventoryLayout {
        InventoryLayout::with_scroll(
            width,
            height,
            app.inventory_selection,
            app.inventory_entries().len(),
            app.inventory_equipped_count(),
            app.inventory_scroll,
        )
    }

    fn pointer(rect: Rect, clicked: bool) -> InputFrame {
        InputFrame {
            pointer: Some((rect.center().x, rect.center().y)),
            viewport: Some((1280.0, 800.0)),
            pressed: if clicked {
                [controls::Binding::MouseLeft].into()
            } else {
                Default::default()
            },
            ..Default::default()
        }
    }

    #[test]
    fn body_appearance_slot_selects_owned_torso_and_preserves_real_loadout_and_weapon_pose() {
        let mut app = fixture();
        let real = app
            .inventory_slot_instance(1)
            .expect("diagnostic has a torso equipped");
        app.game
            .grant_player_item_for_diagnostic("core:veste_de_la_seconde_ombre".parse().unwrap(), 1)
            .unwrap();
        let look = app
            .game
            .player_inventory()
            .iter()
            .find(|e| e.instance() != real && app.inventory_item_is_torso(e.instance()))
            .map(|e| e.instance())
            .expect("diagnostic carries another torso");
        app.select_inventory_instance(look);
        let before = app.front_player_visual();
        let equipped = app
            .game
            .player_equipment()
            .iter()
            .map(|(s, i)| (s.clone(), i))
            .collect::<Vec<_>>();
        let turn = app.game.turn();
        let layout = layout(&app, 1280.0, 800.0);
        let equipment = EquipmentLayout::new(&layout, 1280.0, 800.0);
        let history = app.history.len();
        app.update_input(&pointer(equipment.slots[BODY_APPEARANCE_INDEX], true));
        assert_eq!(app.game.player_equipment().body_appearance(), Some(look));
        assert_eq!(
            app.inventory_entries()[app.inventory_selection].instance(),
            look
        );
        assert_eq!(app.game.turn(), turn);
        assert_eq!(app.inventory_slot_instance(1), Some(real));
        assert_eq!(
            app.inventory_slot_instance(BODY_APPEARANCE_INDEX),
            Some(look)
        );
        assert_eq!(app.history.len(), history + 1);
        assert_eq!(
            app.game
                .player_equipment()
                .iter()
                .map(|(s, i)| (s.clone(), i))
                .collect::<Vec<_>>(),
            equipped
        );
        if let (
            crate::game_art::Visual::Player {
                weapon: a,
                hands: b,
                ..
            },
            crate::game_art::Visual::Player {
                weapon: c,
                hands: d,
                equipment: shown,
                palettes,
                ..
            },
        ) = (before, app.front_player_visual())
        {
            assert_eq!((a, b), (c, d));
            let entry = app.game.player_inventory().get(look).unwrap();
            assert_eq!(
                shown[0],
                crate::game_art::item(entry.item().as_str()).map(|i| i.illustration)
            );
            assert_eq!(palettes[0], entry.palette());
        } else {
            panic!("expected player visuals");
        }
        let bytes = app.game.recovery_snapshot_bytes().unwrap();
        let restored =
            WorldState::from_recovery_snapshot_bytes(&bytes, app.game.rules().clone()).unwrap();
        assert_eq!(restored.player_equipment().body_appearance(), Some(look));
        app.update_input(&pointer(equipment.appearance_clear(), true));
        assert_eq!(app.game.player_equipment().body_appearance(), None);
        assert_eq!(
            app.inventory_entries()[app.inventory_selection].instance(),
            look
        );
        assert_eq!(app.front_player_visual(), before);
        assert_eq!(app.game.turn(), turn);
    }

    #[test]
    fn hovering_bag_and_equipped_slots_reads_the_exact_instance_without_changing_the_run() {
        let mut app = fixture();
        let layout = layout(&app, 1280.0, 800.0);
        let equipment = EquipmentLayout::new(&layout, 1280.0, 800.0)
            .for_hands(app.game.rules().uses_weapon_hands());
        let before = (
            suspension::fingerprint(&app.game),
            app.history.len(),
            app.inventory_selection,
            app.active_weapon_slot,
        );
        for (index, row) in &layout.rows {
            let expected = app.inventory_entries()[*index].instance();
            app.update_input(&pointer(*row, false));
            assert_eq!(app.ux.inventory_tooltip, Some(expected));
            assert_eq!(
                (
                    suspension::fingerprint(&app.game),
                    app.history.len(),
                    app.inventory_selection,
                    app.active_weapon_slot
                ),
                before
            );
        }
        for (index, slot) in equipment.slots.into_iter().enumerate() {
            let expected = app.inventory_slot_instance(index);
            app.update_input(&pointer(slot, false));
            assert_eq!(app.ux.inventory_tooltip, expected);
            assert_eq!(
                (
                    suspension::fingerprint(&app.game),
                    app.history.len(),
                    app.inventory_selection,
                    app.active_weapon_slot
                ),
                before
            );
        }
    }

    #[test]
    fn equipped_slots_remain_inspectable_under_another_filter_and_do_not_equip_on_click() {
        let mut app = fixture();
        app.inventory_filter = InventoryFilter::Weapons;
        let expected = app.inventory_slot_instance(0).unwrap();
        let head = EquipmentLayout::new(&layout(&app, 1280.0, 800.0), 1280.0, 800.0).slots[0];
        let before = (
            suspension::fingerprint(&app.game),
            app.history.len(),
            app.active_weapon_slot,
        );
        app.update_input(&pointer(head, false));
        assert_eq!(app.ux.inventory_tooltip, Some(expected));
        app.update_input(&pointer(head, true));
        assert_eq!(
            app.inventory_entries()[app.inventory_selection].instance(),
            expected
        );
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Inspect).clone()].into(),
            ..Default::default()
        });
        let name = app.inventory_entry_name(app.game.player_inventory().get(expected).unwrap());
        assert_eq!(app.ux.item_card.as_ref().unwrap().0, name);
        assert_eq!(
            (
                suspension::fingerprint(&app.game),
                app.history.len(),
                app.active_weapon_slot
            ),
            before
        );
    }

    #[test]
    fn item_tooltip_uses_real_armor_and_mass_and_keyboard_navigation_updates_it() {
        let mut app = fixture();
        let body = app.inventory_slot_instance(1).unwrap();
        let lines = app.inventory_item_lines(body).join("\n");
        assert!(lines.contains("Armure : +3"));
        assert!(lines.contains("Masse : 2.60 kg"));
        let before = suspension::fingerprint(&app.game);
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::MenuDown).clone()].into(),
            ..Default::default()
        });
        assert_eq!(
            app.ux.inventory_tooltip,
            Some(app.inventory_entries()[app.inventory_selection].instance())
        );
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn equipment_and_tooltip_bounds_survive_compact_windows_and_large_interface_scale() {
        for (width, height) in [
            (684.0, 460.0),
            (960.0, 540.0),
            (1280.0, 800.0),
            (853.0, 533.0),
            (1920.0, 1080.0),
        ] {
            let list = InventoryLayout::with_scroll(width, height, 0, 16, 7, None);
            let equipment = EquipmentLayout::new(&list, width, height);
            assert!(equipment.silhouette.w > 0.0 && equipment.silhouette.h > 0.0);
            for (i, slot) in equipment.slots.iter().enumerate() {
                assert!(slot.x >= list.list.right());
                assert!(slot.right() <= width - 32.0);
                assert!(slot.bottom() < list.actions[0].y);
                assert_eq!(equipment.hit((slot.center().x, slot.center().y)), Some(i));
                assert!(
                    equipment
                        .slots
                        .iter()
                        .enumerate()
                        .all(|(other, rect)| i == other || !slot.overlaps(rect))
                );
            }
            let tooltip = AsciiApp::inventory_tooltip_rect(&list, width, height);
            assert!(tooltip.w >= 200.0 && tooltip.h > 130.0);
            assert!(tooltip.bottom() < list.actions[0].y);
            if list.stats_panel.is_none() {
                assert!(!tooltip.overlaps(&equipment.slots[BODY_APPEARANCE_INDEX]));
            }
        }
    }
}

impl AsciiApp {
    pub(super) fn inventory_item_lines(&self, instance: ItemInstanceId) -> Vec<String> {
        self.inventory_item_lines_for_game(&self.game, instance, true)
    }

    pub(super) fn inventory_item_lines_for_game(
        &self,
        game: &GameState,
        instance: ItemInstanceId,
        compare_equipped: bool,
    ) -> Vec<String> {
        let Some(entry) = game.player_inventory().get(instance) else {
            return Vec::new();
        };
        let resolved_weapon = game.player_item_weapon(entry.instance());
        let weapon = resolved_weapon.as_ref();
        let item_definition = game.rules().items.get(entry.item());
        let mut lines = Vec::new();
        if let Some(weapon) = weapon {
            lines.push(Self::weapon_category_label_for_game(game, weapon));
        }
        if let Some(label) = compare_equipped
            .then(|| self.inventory_equipped_label(instance))
            .flatten()
        {
            let slot = game
                .player_equipment()
                .slot_of(instance)
                .map(ContentId::as_str);
            lines.push(match slot {
                Some("core:head_armor") => "Équipé · Tête".to_owned(),
                Some("core:body_armor") => "Équipé · Corps".to_owned(),
                Some("core:hand_armor") => "Équipé · Mains".to_owned(),
                Some("core:foot_armor") => "Équipé · Pieds".to_owned(),
                _ => label,
            });
        }
        let bonuses = entry.magic_modifiers();
        if weapon.is_some() || item_definition.is_some_and(|item| item.kind() == ItemKind::Armor) {
            lines.push(equipment_affix_names::bonus_count_label(bonuses.as_ref()));
        }
        if let Some(bonuses) = bonuses.clone() {
            let details = equipment_affix_names::affix_detail_lines(bonuses.clone());
            if details.is_empty() {
                lines.push(effect_feedback::weapon_bonus_label(bonuses));
            } else {
                lines.extend(details);
            }
        }
        if let Some(mass) = weapon
            .and_then(|w| w.mass_grams())
            .or_else(|| item_definition.and_then(|item| item.mass_grams()))
        {
            let effective = mass.saturating_mul(
                100_u32.saturating_sub(
                    bonuses
                        .as_ref()
                        .map_or(0, |b| u32::from(b.mass_reduction_percent())),
                ),
            ) / 100;
            lines.push(format!("Masse : {:.2} kg", effective as f64 / 1000.0));
        }
        let mut rows = Vec::new();
        if let Some(weapon) = weapon {
            let attack = game
                .player_item_attack_profile(entry.instance())
                .unwrap_or_else(|| weapon.attack());
            let authored_damage = attack.damage();
            let damage = game
                .resolved_attack_damage(game.player_id(), attack)
                .unwrap_or(authored_damage);
            let damage_label = attack.melee_impact().map_or_else(
                || damage.raw_total().to_string(),
                |impact| {
                    format!(
                        "{} (base {} · max. {})",
                        damage.raw_total(),
                        physical_damage_total(authored_damage),
                        impact.material_cap
                    )
                },
            );
            let area = if let Some(launcher) = weapon.launcher() {
                format!(
                    "Explosion · rayon {} · {}",
                    launcher.radius,
                    if launcher.delay_turns == 0 {
                        "à l’impact".into()
                    } else {
                        format!("{} tours", launcher.delay_turns)
                    }
                )
            } else {
                match attack.area() {
                    AttackArea::Single => "Cible unique".to_owned(),
                    AttackArea::Adjacent => "Cases voisines".to_owned(),
                    AttackArea::Pulse(pulse) => format!("Onde · rayon {}", pulse.radius()),
                    AttackArea::Cone(cone) => format!(
                        "Cône · largeur 1 à {}",
                        cone.maximum_half_width()
                            .saturating_mul(2)
                            .saturating_add(1)
                    ),
                }
            };
            rows.extend([
                (
                    "Dégâts",
                    format!("{damage_label} · {}", damage_component_types_label(damage)),
                ),
                (
                    "Maniement",
                    format!(
                        "{:+} précision · {} pénétration",
                        attack.accuracy_modifier(),
                        format_damage_penetration(damage)
                    ),
                ),
                (
                    "Portée",
                    format!(
                        "{} case{} · {}",
                        attack.range(),
                        if attack.range() > 1 { "s" } else { "" },
                        if attack.requires_line_of_sight() {
                            "vue requise"
                        } else {
                            "sans ligne de vue"
                        }
                    ),
                ),
                ("Zone", area),
            ]);
            if !weapon.distance_damage_percentages().is_empty() {
                rows.push(("Distance", weapon_distance_damage_label(weapon, damage)));
            }
            if let Some(burst) = weapon.burst() {
                rows.push((
                    "Rafale",
                    format!(
                        "{} balles · {} dégâts chacune",
                        burst.shots,
                        damage.raw_total()
                    ),
                ));
                if burst.sustained_accuracy > 0 {
                    rows.push((
                        "Tir soutenu",
                        format!(
                            "+{} précision par rafale · max. +{}",
                            burst.sustained_accuracy,
                            burst.sustained_accuracy * 3
                        ),
                    ));
                    rows.push(("Chaleur", format!("+{} par rafale", burst.heat)));
                }
            }
            if let Some(launcher) = weapon
                .launcher()
                .filter(|launcher| launcher.delay_turns > 0)
            {
                rows.push((
                    "Explosion",
                    format!(
                        "{} dégâts · après {} tours",
                        launcher.damage.amount, launcher.delay_turns
                    ),
                ));
            }
            if let Some(supply) = game
                .weapon_supply(weapon.id())
                .and_then(|_| Self::weapon_supply_label_for_game(game, weapon.id()))
            {
                rows.push(("Coût du tir", supply));
            }
        } else if let Some(definition) = item_definition {
            if let Some(equipment) = definition.equipment() {
                let magic = entry.magic_modifiers();
                rows.push((
                    "Armure",
                    magic
                        .as_ref()
                        .filter(|bonus| bonus.armor_bonus() > 0)
                        .map_or_else(
                            || format!("+{}", equipment.armor()),
                            |bonus| {
                                format!(
                                    "+{} (base {} + bonus {})",
                                    equipment.armor().saturating_add(bonus.armor_bonus()),
                                    equipment.armor(),
                                    bonus.armor_bonus()
                                )
                            },
                        ),
                ));
                rows.push((
                    "Emplacement",
                    self.equipment_slot_name(equipment.slot()).to_owned(),
                ));
                if equipment.evasion_penalty() > 0 {
                    rows.push(("Esquive", format!("-{}", equipment.evasion_penalty())));
                }
                if let Some(bonus) = magic
                    .as_ref()
                    .filter(|bonus| bonus.mass_reduction_percent() > 0)
                {
                    rows.push((
                        "Allègement",
                        format!("Masse −{} %", bonus.mass_reduction_percent()),
                    ));
                }
                rows.push((
                    "Bonus",
                    magic
                        .as_ref()
                        .map_or(0, project_rl::entity::MagicItemModifiers::bonus_count)
                        .to_string(),
                ));
            } else {
                rows.push(("Pile maximale", definition.maximum_stack().to_string()));
            }
            rows.push(("Quantité", entry.quantity().to_string()));
            for effect in definition.effects() {
                rows.push((
                    "Restaure",
                    match effect {
                        ItemEffect::RestoreIntegrity { amount } => format!("{amount} PV"),
                        ItemEffect::RestoreEnergy { amount } => format!("{amount} énergie"),
                    },
                ));
            }
        }
        if let Some(owner) = entry.owner() {
            rows.push((
                "Propriété",
                if game.player_may_take_property_of(owner) {
                    "Attribué · autorisé"
                } else {
                    "Attribué · non autorisé"
                }
                .to_owned(),
            ));
        }
        lines.extend(
            rows.into_iter()
                .map(|(label, value)| format!("{label} : {value}")),
        );
        if let Some(weapon) = weapon {
            let effect_affix = bonuses
                .as_ref()
                .and_then(|b| b.effect_affix())
                .and_then(|id| game.rules().weapons.effect_affix(id));
            if let Some(description) = self.texts.resolve(
                DISPLAY_LOCALE,
                effect_affix.map_or(weapon.description_key(), |effect| effect.description_key()),
            ) {
                lines.push(description.to_owned());
            }
            let limits = Self::weapon_effect_limits_for_game(game, weapon);
            if !limits.is_empty() {
                lines.push(limits);
            }
        } else if let Some(item) = item_definition {
            if let Some(description) = self.texts.resolve(DISPLAY_LOCALE, item.description_key()) {
                lines.push(description.to_owned());
            }
        }
        if !compare_equipped {
            lines.retain(|line| !line.is_empty());
            return lines;
        }
        let slot = item_definition
            .and_then(|item| item.equipment())
            .map(|profile| profile.slot())
            .or_else(|| {
                weapon.and_then(|_| {
                    game.rules()
                        .player_weapon_slots
                        .get(usize::from(self.active_weapon_slot))
                })
            });
        if let Some(current) = slot
            .and_then(|slot| game.player_equipment().equipped(slot))
            .filter(|current| *current != instance)
            .and_then(|current| game.player_inventory().get(current))
        {
            lines.push(format!(
                "Actuellement : {}",
                self.inventory_entry_name(current)
            ));
            if let Some(profile) = item_definition.and_then(|item| item.equipment()) {
                let new_armor = profile.armor() + bonuses.as_ref().map_or(0, |b| b.armor_bonus());
                if let Some(current_profile) = game
                    .rules()
                    .items
                    .get(current.item())
                    .and_then(|item| item.equipment())
                {
                    let current_armor = current_profile.armor()
                        + current
                            .magic_modifiers()
                            .as_ref()
                            .map_or(0, |b| b.armor_bonus());
                    lines.push(format!(
                        "Écart d'armure : {:+}",
                        i32::from(new_armor) - i32::from(current_armor)
                    ));
                }
            }
        }
        lines.retain(|line| !line.is_empty());
        lines
    }
}
