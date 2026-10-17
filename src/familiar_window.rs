//! Floating companion reader. Ordering, folding and dragging never issue commands.
use super::*;

#[derive(Default)]
pub(super) struct State {
    pub open: bool,
    pub folded: bool,
    individual_orders: bool,
    pub order: Vec<EntityId>,
    pub selected: Option<EntityId>,
    pub position: Option<Vec2>,
    pub scroll: f32,
    pub pointer: Option<Vec2>,
    drag: Option<Drag>,
    confirm: Option<EntityId>,
}
enum Drag {
    Window(Vec2),
    Pet {
        entity: EntityId,
        start: Vec2,
        moved: bool,
    },
}

impl State {
    fn ordered(&self, live: &[EntityId]) -> Vec<EntityId> {
        let mut result: Vec<_> = self
            .order
            .iter()
            .copied()
            .filter(|id| live.contains(id))
            .collect();
        for id in live {
            if !result.contains(id) {
                result.push(*id);
            }
        }
        result
    }
    fn sync(&mut self, live: &[EntityId]) {
        self.order = self.ordered(live);
        if !self.selected.is_some_and(|id| live.contains(&id)) {
            self.selected = self.order.first().copied();
        }
        if self.confirm.is_some_and(|id| !live.contains(&id)) {
            self.confirm = None;
        }
    }
    fn move_before(&mut self, entity: EntityId, target: EntityId) {
        if entity == target {
            return;
        }
        let Some(from) = self.order.iter().position(|id| *id == entity) else {
            return;
        };
        let Some(to) = self.order.iter().position(|id| *id == target) else {
            return;
        };
        self.order.remove(from);
        self.order.insert(to, entity);
    }
}

pub(super) struct Layout {
    pub panel: Rect,
    header: Rect,
    fold: Rect,
    close: Rect,
    list: Rect,
    detail: Rect,
    pub behavior_buttons: [Rect; 4],
    dismiss: Rect,
    scope: Rect,
}
impl Layout {
    pub fn new(width: f32, height: f32, state: &State) -> Self {
        let w = 420f32.min(width - 32.);
        let h = 566f32.min(height - 36.);
        let p = state.position.unwrap_or(vec2(width - w - 38., 62.));
        let panel = Rect::new(
            p.x.clamp(8., (width - w - 8.).max(8.)),
            p.y.clamp(
                8.,
                (height - if state.folded { 36. } else { h } - 8.).max(8.),
            ),
            w,
            if state.folded { 36. } else { h },
        );
        let header = Rect::new(panel.x, panel.y, w, 36.);
        let detail = Rect::new(panel.x + 14., panel.bottom() - 141., w - 28., 83.);
        Self {
            panel,
            header,
            close: Rect::new(panel.right() - 34., panel.y + 5., 26., 26.),
            fold: Rect::new(panel.right() - 66., panel.y + 5., 26., 26.),
            list: Rect::new(
                panel.x + 10.,
                panel.y + 44.,
                w - 20.,
                (detail.y - panel.y - 54.).max(80.),
            ),
            detail,
            behavior_buttons: std::array::from_fn(|i| {
                Rect::new(
                    panel.x + 14. + i as f32 * 42.,
                    panel.bottom() - 43.,
                    38.,
                    32.,
                )
            }),
            scope: Rect::new(panel.x + 190., panel.bottom() - 43., 64., 30.),
            dismiss: Rect::new(panel.right() - 140., panel.bottom() - 43., 126., 30.),
        }
    }
}

struct Effect {
    detail: Option<String>,
    name: String,
    icon: Option<String>,
    status: String,
    turns: Option<u64>,
    stacks: u16,
}
fn status_art(id: &str) -> Option<&'static str> {
    Some(match id {
        "agent:aegis" | "lab:impact_aegis" => "B02",
        "agent:thorns" => "SU03",
        "agent:regeneration" => "SU04",
        "agent:armor_breach" => "AF02",
        "agent:silence" => "AF03",
        "agent:regeneration_break" => "AF04",
        "agent:disorientation" => "AF05",
        "agent:interference" => "O02",
        "agent:domination" => "HY06",
        "agent:spectral_step" => "T02",
        "agent:phase_traversal" => "ES05",
        "agent:provoked" => "D66",
        "agent:frozen" | "agent:mine_frozen" => "FR02",
        _ => return None,
    })
}

impl AsciiApp {
    fn familiar_effects(&self, entity: EntityId) -> Vec<Effect> {
        let Some(actor) = self.game.actors().get(entity) else {
            return vec![];
        };
        let mut effects: Vec<_> = actor
            .statuses()
            .map(|s| Effect {
                detail: self
                    .game
                    .agent_status_hud_value(entity, &s.definition)
                    .and_then(|(key, value)| {
                        agent_skill_window::parameter_label(key).map(|label| {
                            format!(
                                "{label} : {value}{}",
                                if key.ends_with("percent") { " %" } else { "" }
                            )
                        })
                    }),
                name: status_display_name(&s.definition),
                icon: status_art(s.definition.as_str()).map(str::to_owned),
                status: s.definition.to_string(),
                turns: s.remaining_turns.map(u64::from),
                stacks: s.stacks,
            })
            .collect();
        if actor
            .drone()
            .is_some_and(|d| d.controller() == self.game.player_id())
        {
            for id in ["DP02", "O03"] {
                if self.game.player_agent_skill_rank(id) > 0
                    && let Some(def) = project_rl::skills::agent::definition(id)
                {
                    let key = if id == "DP02" {
                        "incoming_damage_reduction_percent"
                    } else {
                        "drone_damage_bonus_percent"
                    };
                    let value = def.value(key, self.game.player_agent_skill_rank(id));
                    effects.push(Effect {
                        detail: agent_skill_window::parameter_label(key)
                            .map(|label| format!("{label} : {value} %")),
                        name: def.title.clone(),
                        icon: Some(id.into()),
                        status: String::new(),
                        turns: None,
                        stacks: 1,
                    });
                }
            }
        }
        effects
    }
    fn familiar_rows(&self, layout: &Layout, order: &[EntityId]) -> Vec<(EntityId, Rect)> {
        let mut y = layout.list.y - self.ux.familiars.scroll;
        order
            .iter()
            .map(|id| {
                let rows = self.familiar_effects(*id).len().div_ceil(11);
                let h = 76. + rows.max(1) as f32 * 29.;
                let r = Rect::new(layout.list.x, y, layout.list.w - 7., h);
                y += h + 6.;
                (*id, r)
            })
            .collect()
    }
    fn familiar_name(&self, entity: EntityId) -> String {
        if let Some(drone) = self.game.actors().get(entity).and_then(Actor::drone) {
            let id = drone
                .profile()
                .visual_profile()
                .as_str()
                .strip_prefix("agent:")
                .unwrap_or("")
                .to_ascii_uppercase();
            if let Some(def) = project_rl::skills::agent::definition(&id) {
                return def
                    .title
                    .strip_prefix("Invocation du drone ")
                    .or_else(|| def.title.strip_prefix("Invocation du "))
                    .or_else(|| def.title.strip_prefix("Invocation de l’"))
                    .or_else(|| def.title.strip_prefix("Invocation de l'"))
                    .unwrap_or(&def.title)
                    .to_owned();
            }
            return "Drone".into();
        }
        let name = self
            .terminal_target_summary_for(entity)
            .map(|s| s.name)
            .unwrap_or_else(|| "Allié".into());
        if self.game.agent_reanimated_ally(entity) {
            format!("Réanimé · {name}")
        } else {
            name
        }
    }
    pub(super) fn route_familiar_window(&mut self, input: &InputFrame) -> bool {
        self.ux.familiars.pointer = input.pointer.map(Into::into);
        if self.menu_navigation_context().is_some()
            || self.resume_requested
            || self.rebinding
            || self.quit_requested
            || self.attack_aim.is_some()
        {
            self.ux.familiars.drag = None;
            return false;
        }
        let (w, h) = input.viewport.unwrap_or((1280., 800.));
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        let pointer = input.pointer.map(Vec2::from);
        let button = crate::terminal_chrome::command_rect(w, h, 10);
        if clicked && pointer.is_some_and(|p| button.contains(p)) {
            self.ux.familiars.open = !self.ux.familiars.open;
            self.ux.familiars.drag = None;
            self.ux.familiars.confirm = None;
            self.mouse_walk = None;
            self.movement_repeat.clear();
            return true;
        }
        if !self.ux.familiars.open {
            return false;
        }
        self.ux
            .familiars
            .sync(&self.game.player_controlled_companions());
        if input.pause {
            if self.ux.familiars.confirm.take().is_none() {
                self.ux.familiars.open = false;
            }
            self.ux.familiars.drag = None;
            return true;
        }
        let layout = Layout::new(w, h, &self.ux.familiars);
        let rows = self.familiar_rows(&layout, &self.ux.familiars.order);
        let maximum = rows.last().map_or(0., |(_, r)| {
            (r.bottom() + self.ux.familiars.scroll - layout.list.bottom()).max(0.)
        });
        self.ux.familiars.scroll = self.ux.familiars.scroll.clamp(0., maximum);
        if let Some(drag) = self.ux.familiars.drag.take() {
            if input.mouse_left_down
                && let Some(p) = pointer
            {
                match drag {
                    Drag::Window(offset) => {
                        self.ux.familiars.position = Some(p - offset);
                        self.ux.familiars.drag = Some(Drag::Window(offset));
                    }
                    Drag::Pet {
                        entity,
                        start,
                        moved,
                    } => {
                        let moved = moved || p.distance(start) >= 5.;
                        if moved {
                            if p.y < layout.list.y + 20. {
                                self.ux.familiars.scroll = (self.ux.familiars.scroll - 7.).max(0.);
                            }
                            if p.y > layout.list.bottom() - 20. {
                                self.ux.familiars.scroll =
                                    (self.ux.familiars.scroll + 7.).min(maximum);
                            }
                            if let Some((target, _)) = rows
                                .iter()
                                .find(|(_, r)| r.contains(p) && layout.list.contains(p))
                            {
                                self.ux.familiars.move_before(entity, *target);
                            }
                        }
                        self.ux.familiars.drag = Some(Drag::Pet {
                            entity,
                            start,
                            moved,
                        });
                    }
                }
            }
            return true;
        }
        let inside = pointer.is_some_and(|p| layout.panel.contains(p));
        if !inside {
            return false;
        }
        if input.wheel_y != 0. && !self.ux.familiars.folded {
            self.ux.familiars.scroll =
                (self.ux.familiars.scroll - input.wheel_y * 42.).clamp(0., maximum);
            return true;
        }
        if !clicked {
            return input.pressed.iter().any(|b| {
                matches!(
                    b,
                    controls::Binding::MouseRight | controls::Binding::MouseMiddle
                )
            });
        }
        self.mouse_walk = None;
        self.movement_repeat.clear();
        let p = pointer.unwrap();
        if layout.close.contains(p) {
            self.ux.familiars.open = false;
            self.ux.familiars.confirm = None;
        } else if layout.fold.contains(p) {
            self.ux.familiars.folded = !self.ux.familiars.folded;
            self.ux.familiars.confirm = None;
        } else if layout.header.contains(p) {
            self.ux.familiars.drag = Some(Drag::Window(p - layout.panel.point()));
        } else if !self.ux.familiars.folded {
            if let Some(index) = layout.behavior_buttons.iter().position(|r| r.contains(p))
                && self
                    .ux
                    .familiars
                    .selected
                    .and_then(|id| self.game.actors().get(id))
                    .is_some_and(|a| a.drone().is_some())
            {
                let behavior = CompanionBehavior::ALL[index];
                let command = if self.ux.familiars.individual_orders {
                    GameCommand::SetCompanionBehaviorFor {
                        entity: self.ux.familiars.selected.unwrap(),
                        behavior,
                    }
                } else {
                    GameCommand::SetCompanionBehavior { behavior }
                };
                if let CommandOutcome::Rejected(reason) = self.execute_command(command) {
                    self.push_log(command_rejection_message(reason).to_owned());
                }
                self.capture_events();
            } else if layout.scope.contains(p) {
                self.ux.familiars.individual_orders = !self.ux.familiars.individual_orders;
            } else if layout.dismiss.contains(p)
                && let Some(entity) = self.ux.familiars.selected
            {
                if self.ux.familiars.confirm == Some(entity) {
                    self.ux.familiars.confirm = None;
                    if let CommandOutcome::Rejected(reason) =
                        self.execute_command(GameCommand::DismissCompanion { entity })
                    {
                        self.push_log(command_rejection_message(reason).to_owned());
                    }
                    self.capture_events();
                } else {
                    self.ux.familiars.confirm = Some(entity);
                }
            } else if layout.list.contains(p)
                && let Some((entity, _)) = rows.iter().find(|(_, r)| r.contains(p))
            {
                self.ux.familiars.selected = Some(*entity);
                self.ux.familiars.confirm = None;
                self.ux.familiars.drag = Some(Drag::Pet {
                    entity: *entity,
                    start: p,
                    moved: false,
                });
            } else {
                self.ux.familiars.confirm = None;
            }
        }
        true
    }
    pub(super) fn draw_familiar_window(&self) {
        if !self.ux.familiars.open {
            return;
        }
        let state = &self.ux.familiars;
        let layout = Layout::new(self.ui_width(), self.ui_height(), state);
        crate::ui_skin::frame(layout.panel);
        crate::ui_theme::draw_text_bold_in_rect(
            "Familiers",
            Rect::new(
                layout.header.x + 14.,
                layout.header.y + 1.,
                layout.header.w - 92.,
                layout.header.h - 2.,
            ),
            17,
            UiTheme.text(),
        );
        for rect in [layout.fold, layout.close] {
            if state.pointer.is_some_and(|p| rect.contains(p)) {
                crate::terminal_chrome::button(rect, true, false, true);
            }
        }
        let c = layout.fold.center();
        draw_line(c.x - 5., c.y, c.x + 5., c.y, 1.5, UiTheme.text());
        if state.folded {
            draw_line(c.x, c.y - 5., c.x, c.y + 5., 1.5, UiTheme.text());
        }
        let c = layout.close.center();
        draw_line(c.x - 4., c.y - 4., c.x + 4., c.y + 4., 1.5, UiTheme.muted());
        draw_line(c.x + 4., c.y - 4., c.x - 4., c.y + 4., 1.5, UiTheme.muted());
        if state.folded {
            return;
        }
        let order = state.ordered(&self.game.player_controlled_companions());
        let selected = state
            .selected
            .filter(|id| order.contains(id))
            .or_else(|| order.first().copied());
        let rows = self.familiar_rows(&layout, &order);
        let mut tooltip = None;
        // Clip partial rows, including their icons; pointer hits use the same viewport.
        let scale = self.ui_scale();
        let clip = layout.list;
        unsafe {
            get_internal_gl().quad_gl.scissor(Some((
                (clip.x * scale) as i32,
                (clip.y * scale) as i32,
                (clip.w * scale) as i32,
                (clip.h * scale) as i32,
            )));
        }
        for (entity, rect) in &rows {
            if rect.bottom() < clip.y || rect.y > clip.bottom() {
                continue;
            }
            let Some(actor) = self.game.actors().get(*entity) else {
                continue;
            };
            let active = selected == Some(*entity);
            if active {
                draw_rectangle(rect.x, rect.y, rect.w, rect.h, UiTheme.surface_raised());
            }
            for dx in [5., 10.] {
                for dy in [14., 19., 24.] {
                    draw_circle(rect.x + dx, rect.y + dy, 1., UiTheme.muted());
                }
            }
            if let Some(visual) = self.front_actor_visual(*entity) {
                crate::game_art::draw_portrait(
                    Rect::new(rect.x + 24., rect.y + 8., 40., 40.),
                    visual,
                );
            }
            crate::ui_theme::draw_text_in_rect(
                self.familiar_name(*entity),
                Rect::new(rect.x + 72., rect.y + 5., rect.w - 78., 22.),
                15,
                UiTheme.text(),
            );
            draw_text(
                format!("PV {} / {}", actor.integrity(), actor.maximum_integrity()),
                rect.x + 72.,
                rect.y + 43.,
                13.,
                UiTheme.text(),
            );
            let bar = Rect::new(rect.x + 72., rect.y + 49., rect.w - 84., 4.);
            draw_rectangle(bar.x, bar.y, bar.w, bar.h, UiTheme.surface());
            draw_rectangle(
                bar.x,
                bar.y,
                bar.w * f32::from(actor.integrity()) / f32::from(actor.maximum_integrity().max(1)),
                bar.h,
                UiTheme.success(),
            );
            let info = if let Some(drone) = actor.drone() {
                let behavior = match drone.order() {
                    DroneOrder::Companion { behavior, .. } => companion_behavior_label(*behavior),
                    DroneOrder::Escort { .. } => {
                        companion_behavior_label(CompanionBehavior::Follow)
                    }
                    _ => "Ordre spécial",
                };
                format!(
                    "{behavior} · {}",
                    if self.game.player_companion_is_linked(*entity) {
                        "LIÉ"
                    } else {
                        "RETOUR AUTO"
                    }
                )
            } else if let Some(turns) = self.game.agent_reanimated_remaining_turns(*entity) {
                format!("Autonome · {turns} tour(s)")
            } else {
                "Autonome".into()
            };
            draw_text(info, rect.x + 24., rect.y + 70., 12., UiTheme.muted());
            for (i, effect) in self.familiar_effects(*entity).iter().enumerate() {
                let icon = Rect::new(
                    rect.x + 24. + (i % 11) as f32 * 30.,
                    rect.y + 77. + (i / 11) as f32 * 29.,
                    26.,
                    26.,
                );
                self.draw_familiar_effect(effect, icon);
                if state
                    .pointer
                    .is_some_and(|p| clip.contains(p) && icon.contains(p))
                {
                    tooltip = Some((icon, effect_tooltip(effect)));
                }
            }
            draw_line(
                rect.x + 8.,
                rect.bottom(),
                rect.right() - 8.,
                rect.bottom(),
                1.,
                crate::terminal_chrome::tint(0.15),
            );
        }
        unsafe {
            get_internal_gl().quad_gl.scissor(None);
        }
        if let Some((_, last)) = rows.last() {
            let total = last.bottom() + state.scroll - clip.y;
            if total > clip.h {
                let thumb = clip.h * clip.h / total;
                draw_rectangle(
                    clip.right() - 3.,
                    clip.y + (clip.h - thumb) * (state.scroll / (total - clip.h)).clamp(0., 1.),
                    3.,
                    thumb,
                    UiTheme.muted(),
                );
            }
        }
        if order.is_empty() {
            draw_text(
                "Aucun familier actif.",
                clip.x + 12.,
                clip.y + 30.,
                15.,
                UiTheme.muted(),
            );
        }
        if let Some(entity) = selected
            && let Some(actor) = self.game.actors().get(entity)
        {
            let y = layout.detail.y;
            crate::ui_theme::draw_text_in_rect(
                self.familiar_name(entity),
                Rect::new(layout.detail.x, y, layout.detail.w, 23.),
                15,
                UiTheme.text(),
            );
            if let Some(drone) = actor.drone() {
                draw_text(
                    format!(
                        "Batterie {} / {}",
                        drone.energy().available(),
                        drone.energy().capacity()
                    ),
                    layout.detail.x,
                    y + 44.,
                    14.,
                    UiTheme.muted(),
                );
            } else if let Some(turns) = self.game.agent_reanimated_remaining_turns(entity) {
                draw_text(
                    format!("Autonome · {turns} tour(s)"),
                    layout.detail.x,
                    y + 44.,
                    14.,
                    UiTheme.muted(),
                );
            }
            if actor.drone().is_some() {
                crate::terminal_chrome::button(
                    layout.scope,
                    state.pointer.is_some_and(|p| layout.scope.contains(p)),
                    state.individual_orders,
                    true,
                );
                let offsets: &[f32] = if state.individual_orders {
                    &[0.]
                } else {
                    &[-13., 13.]
                };
                for &dx in offsets {
                    crate::ui_theme::draw_ui_icon(
                        UiIcon::Character,
                        Rect::new(
                            layout.scope.center().x - 10. + dx,
                            layout.scope.y + 5.,
                            20.,
                            20.,
                        ),
                        UiTheme.text(),
                    );
                }
                if state.pointer.is_some_and(|p| layout.scope.contains(p)) {
                    tooltip = Some((
                        layout.scope,
                        if state.individual_orders {
                            "Ordres individuels"
                        } else {
                            "Ordre des drones"
                        }
                        .into(),
                    ));
                }
                for (index, behavior) in CompanionBehavior::ALL.into_iter().enumerate() {
                    let r = layout.behavior_buttons[index];
                    let matches = |id| {
                        self.game.actors().get(id).and_then(Actor::drone).is_some_and(|d| matches!(d.order(), DroneOrder::Companion { behavior: current, .. } if *current == behavior))
                    };
                    let active = if state.individual_orders {
                        matches(entity)
                    } else {
                        self.game
                            .player_controlled_companions()
                            .iter()
                            .copied()
                            .filter(|id| {
                                self.game
                                    .actors()
                                    .get(*id)
                                    .is_some_and(|a| a.drone().is_some())
                            })
                            .all(matches)
                    };
                    crate::terminal_chrome::button(
                        r,
                        state.pointer.is_some_and(|p| r.contains(p)),
                        active,
                        true,
                    );
                    draw_companion_behavior_icon(r, behavior, UiTheme.text());
                    if state.pointer.is_some_and(|p| r.contains(p)) {
                        draw_companion_behavior_tooltip(r, behavior);
                    }
                }
            }
            let confirming = state.confirm == Some(entity);
            crate::terminal_chrome::button(
                layout.dismiss,
                state.pointer.is_some_and(|p| layout.dismiss.contains(p)),
                confirming,
                true,
            );
            crate::ui_theme::draw_text_bold_centered(
                if confirming {
                    "Confirmer ?"
                } else {
                    "Détruire"
                },
                layout.dismiss,
                13,
                if confirming {
                    UiTheme.danger()
                } else {
                    UiTheme.text()
                },
            );
        }
        if let Some((r, text)) = tooltip {
            effect_tip(r, &text, self.ui_width(), self.ui_height());
        }
    }
    fn draw_familiar_effect(&self, effect: &Effect, rect: Rect) {
        let pulse = effect.turns == Some(1) && !self.graphics.active.reduced_motion;
        let alpha = if pulse {
            0.7 + 0.3 * ((get_time() * std::f64::consts::TAU / 1.3).cos() as f32)
        } else {
            1.
        };
        draw_rectangle(rect.x, rect.y, rect.w, rect.h, UiTheme.surface());
        if !effect
            .icon
            .as_deref()
            .is_some_and(|id| agent_skill_window::art(id, "compact", rect, None))
        {
            crate::terminal_view::draw_hud_status_icon(rect, &effect.status);
        }
        if alpha < 1. {
            draw_rectangle(
                rect.x,
                rect.y,
                rect.w,
                rect.h,
                Color::new(0., 0., 0., 1. - alpha),
            );
        }
        if effect.turns == Some(1) {
            draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1., UiTheme.attention());
        }
    }
    pub(super) fn draw_actor_effect_icons(&self) {
        let scale = self.ui_scale();
        let physical = crate::terminal_view::terminal_playfield(
            &self.game,
            self.terminal_bounds(),
            scale,
            self.navigation_signal_summary().is_some(),
        );
        let map = Rect::new(
            physical.x / scale,
            physical.y / scale,
            physical.w / scale,
            physical.h / scale,
        );
        let per_column = ((map.h - 12.) / 35.).floor().max(1.) as usize;
        let mut tooltip = None;
        for (i, effect) in self
            .familiar_effects(self.game.player_id())
            .iter()
            .enumerate()
        {
            let rect = Rect::new(
                map.x + 8. + (i / per_column) as f32 * 35.,
                map.y + 8. + (i % per_column) as f32 * 35.,
                30.,
                30.,
            );
            self.draw_familiar_effect(effect, rect);
            if self.ux.familiars.pointer.is_some_and(|p| rect.contains(p)) {
                tooltip = Some((rect, effect_tooltip(effect)));
            }
        }
        if let Some((r, text)) = tooltip {
            effect_tip(r, &text, self.ui_width(), self.ui_height());
        }
    }
}
fn effect_tooltip(effect: &Effect) -> String {
    let mut text = effect.name.clone();
    if effect.stacks > 1 {
        text.push_str(&format!(" ×{}", effect.stacks));
    }
    if let Some(turns) = effect.turns {
        text.push_str(&format!(" · {turns} tour(s)"));
    }
    if let Some(detail) = &effect.detail {
        text.push_str(&format!(" · {detail}"));
    }
    text
}
fn effect_tip(anchor: Rect, text: &str, width: f32, height: f32) {
    let w = (measure_text(text, None, 14, 1.).width + 24.).min(width - 24.);
    let r = Rect::new(
        anchor.x.clamp(12., (width - w - 12.).max(12.)),
        (anchor.y - 38.).clamp(8., height - 40.),
        w,
        30.,
    );
    crate::ui_skin::frame(r);
    crate::ui_theme::draw_text_in_rect(
        text,
        Rect::new(r.x + 10., r.y, r.w - 20., r.h),
        14,
        UiTheme.text(),
    );
}

#[cfg(debug_assertions)]
impl AsciiApp {
    pub async fn capture_native_hud(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let mut app = Self::new_agent_drone_trial()?;
        app.suspension_path = output.join("isolated-run.json");
        app.graphics.active.ui_scale_percent = 100;
        // A disposable energy-rich fixture exercises several simultaneous
        // effects; these values are never written to gameplay content or saves.
        let player = app.game.player_position().unwrap();
        let mut rules = app.game.rules().clone();
        rules
            .agent_class_start
            .as_mut()
            .unwrap()
            .maximum_energy_percent = 1000;
        rules.player_energy_capacity = 600;
        rules.player_starting_energy = 600;
        rules
            .agent_class_start
            .as_mut()
            .unwrap()
            .skills
            .extend(["D58", "D59", "D66"].map(str::to_owned));
        app.game = WorldState::single(
            GameState::new_with_rules(app.game.map().clone(), player, 29, rules)
                .map_err(|e| e.to_string())?,
        );
        app.actor_glyphs.clear();
        for (id, dx, dy) in [("D58", -1, 0), ("D59", 0, -1), ("D66", -1, -1)] {
            let outcome = app.execute_command(GameCommand::UseTechniqueAt {
                technique: project_rl::skills::agent::technique(id),
                target: GridPos::new(player.x + dx, player.y + dy),
                weapon_slot: 0,
            });
            if outcome != CommandOutcome::Applied {
                return Err(format!("Drone de contrôle {id} : {outcome:?}"));
            }
        }
        if app.game.player_agent_drone_count() != 3 {
            return Err("Les trois drones de contrôle sont absents".into());
        }
        for skill in ["DP02", "SU03", "OC07", "A01"] {
            let outcome = app.execute_command(GameCommand::LearnAgentSkill {
                skill: skill.into(),
            });
            if app.game.player_agent_skill_rank(skill) == 0 {
                return Err(format!("Apprentissage de contrôle {skill} : {outcome:?}"));
            }
        }
        let player = app.game.player_position().unwrap();
        let outcome = app.execute_command(GameCommand::UseTechniqueAt {
            technique: project_rl::skills::agent::technique("SU03"),
            target: player,
            weapon_slot: 0,
        });
        if outcome != CommandOutcome::Applied {
            return Err(format!("Protection de contrôle : {outcome:?}"));
        }
        let at = GridPos::new(player.x - 1, player.y + 1);
        let target = app
            .game
            .spawn_actor(
                Actor::new(at, 1)
                    .unwrap()
                    .with_evasion_disabled()
                    .with_player_relation(PlayerRelation::Hostile),
            )
            .map_err(|e| e.to_string())?;
        app.actor_glyphs.insert(target, 'Q');
        for skill in ["OC07", "A01"] {
            let outcome = app.execute_command(GameCommand::UseTechniqueAt {
                technique: project_rl::skills::agent::technique(skill),
                target: at,
                weapon_slot: 0,
            });
            if outcome != CommandOutcome::Applied {
                return Err(format!("Réanimation de contrôle {skill} : {outcome:?}"));
            }
            if app
                .game
                .player_controlled_companions()
                .iter()
                .any(|id| app.game.agent_reanimated_ally(*id))
            {
                break;
            }
        }
        app.capture_events_at(Some(0.));
        if !app
            .game
            .player_controlled_companions()
            .iter()
            .any(|id| app.game.agent_reanimated_ally(*id))
        {
            return Err("Réanimé de contrôle absent".into());
        }
        for _ in 0..4 {
            let turns = app
                .game
                .actors()
                .get(app.game.player_id())
                .unwrap()
                .statuses()
                .find(|s| s.definition.as_str() == "agent:thorns")
                .and_then(|s| s.remaining_turns);
            if turns == Some(1) {
                break;
            }
            app.execute_command(GameCommand::Wait);
            app.capture_events_at(Some(0.));
        }
        app.ux.familiars.open = true;
        app.ux
            .familiars
            .sync(&app.game.player_controlled_companions());
        let before = suspension::fingerprint(&app.game);
        let order_before = app.ux.familiars.order.clone();
        let viewport = Some((1920. / 1.35, 1080. / 1.35));
        let layout = Layout::new(viewport.unwrap().0, viewport.unwrap().1, &app.ux.familiars);
        let rows = app.familiar_rows(&layout, &order_before);
        let a = rows[0].1.center();
        let b = rows[1].1.center();
        for input in [
            InputFrame {
                pressed: [controls::Binding::MouseLeft].into(),
                mouse_left_down: true,
                pointer: Some(a.into()),
                viewport,
                ..Default::default()
            },
            InputFrame {
                mouse_left_down: true,
                pointer: Some(b.into()),
                viewport,
                ..Default::default()
            },
            InputFrame {
                pointer: Some(b.into()),
                viewport,
                ..Default::default()
            },
        ] {
            app.update_input(&input);
        }
        if app.ux.familiars.order[0] != order_before[1]
            || suspension::fingerprint(&app.game) != before
        {
            return Err("Classement de contrôle incorrect ou action parasite".into());
        }
        set_fullscreen(true); // Borderless capture avoids desktop title-bar limits.
        let mut captures = vec![];
        for (width, height) in [(1920., 1080.), (2560., 1440.), (3840., 2160.)] {
            request_new_screen_size(width, height);
            for _ in 0..5 {
                app.draw();
                next_frame().await;
            }
            for style in crate::graphics::InterfaceStyle::ALL {
                app.graphics.active.interface_style = style;
                app.ux.familiars.open = true;
                app.ux.familiars.pointer = None;
                for _ in 0..4 {
                    app.draw();
                    next_frame().await;
                }
                app.draw();
                let name = format!(
                    "familiers-{}-{}x{}.png",
                    if style == crate::graphics::InterfaceStyle::Metal {
                        "metal"
                    } else {
                        "terminal"
                    },
                    width as u32,
                    height as u32
                );
                let image = crate::ui_capture::framebuffer()?;
                if (image.width as f32, image.height as f32) != (width, height) {
                    return Err(format!(
                        "Surface native incorrecte : {} × {}",
                        image.width, image.height
                    ));
                }
                image.export_png(output.join(&name).to_str().ok_or("Chemin PNG")?);
                let scale = app.ui_scale();
                let playfield = crate::terminal_view::terminal_playfield(
                    &app.game,
                    app.terminal_bounds(),
                    scale,
                    app.navigation_signal_summary().is_some(),
                );
                captures.push(serde_json::json!({"image":name,"viewport":[width,height],"playfield":[playfield.x,playfield.y,playfield.w,playfield.h]}));
                next_frame().await;
                if width == 1920. {
                    let prior = app.ux.familiars.selected;
                    app.ux.familiars.selected = app
                        .game
                        .player_controlled_companions()
                        .into_iter()
                        .find(|id| app.game.agent_reanimated_ally(*id));
                    let layout = Layout::new(app.ui_width(), app.ui_height(), &app.ux.familiars);
                    let order = app.ux.familiars.order.clone();
                    let rows = app.familiar_rows(&layout, &order);
                    app.ux.familiars.scroll = rows
                        .last()
                        .map_or(0., |(_, r)| (r.bottom() - layout.list.bottom()).max(0.));
                    app.draw();
                    next_frame().await;
                    app.draw();
                    let name = format!(
                        "reanime-{}.png",
                        if style == crate::graphics::InterfaceStyle::Metal {
                            "metal"
                        } else {
                            "terminal"
                        }
                    );
                    crate::ui_capture::framebuffer()?
                        .export_png(output.join(name).to_str().ok_or("Chemin PNG")?);
                    app.ux.familiars.scroll = 0.;
                    app.ux.familiars.selected = prior;
                    next_frame().await;
                }
            }
        }
        // Ordinary generated level: the playfield uses the whole available area.
        let (mut rules, texts, loot, expeditions) = ascii_game_content()?;
        let classes = ascii_character_class_catalog()?;
        let class = classes
            .get(&"core:technomancien".parse().unwrap())
            .ok_or("Classe de contrôle absente")?;
        class
            .apply_to_rules(&mut rules, class.recommended_attributes())
            .map_err(|e| e.to_string())?;
        let mut normal = Self::from_seed(INITIAL_SEED, rules, texts, loot, expeditions)?;
        normal.menu = MenuScreen::Hidden;
        normal.character_creation = None;
        normal.character_class = Some("core:technomancien".parse().unwrap());
        normal.ux.skill_bar = ["D58", "D59", "D66", "DR01", "HY04", "HY07"]
            .map(|id| normal.can_assign_agent_skill(id).then(|| id.into()));
        normal.graphics.active.ui_scale_percent = 100;
        request_new_screen_size(1920., 1080.);
        for style in crate::graphics::InterfaceStyle::ALL {
            normal.graphics.active.interface_style = style;
            for _ in 0..6 {
                normal.draw();
                next_frame().await;
            }
            normal.draw();
            let name = if style == crate::graphics::InterfaceStyle::Metal {
                "jeu-metal-1920x1080.png"
            } else {
                "jeu-terminal-1920x1080.png"
            };
            crate::ui_capture::framebuffer()?
                .export_png(output.join(name).to_str().ok_or("Chemin PNG")?);
            next_frame().await;
        }
        std::fs::write(output.join("native-checks.json"), serde_json::to_vec_pretty(&serde_json::json!({"review":"HUD-07 / UX-PETS-06","captures":captures,"drag_reordered":true,"presentation_preserves_engine":suspension::fingerprint(&app.game)==before,"drones":app.game.player_agent_drone_count(),"companions":app.game.player_controlled_companions().len(),"real_reanimation_deadline":true,"user_saves_untouched":true})).unwrap()).map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ordering_retains_survivors_and_appends_new_companions() {
        let id = |n: u64| serde_json::from_str::<EntityId>(&n.to_string()).unwrap();
        let mut state = State::default();
        state.sync(&[id(1), id(2), id(3)]);
        state.move_before(id(1), id(3));
        assert_eq!(state.order, [id(2), id(3), id(1)]);
        state.sync(&[id(1), id(3), id(4)]);
        assert_eq!(state.order, [id(3), id(1), id(4)]);
    }
    #[test]
    fn resized_or_folded_window_keeps_controls_on_screen() {
        for (w, h) in [
            (1280., 720.),
            (1920., 1080.),
            (2560., 1440.),
            (3840., 2160.),
        ] {
            for folded in [true, false] {
                let state = State {
                    position: Some(vec2(9000., 9000.)),
                    folded,
                    ..Default::default()
                };
                let layout = Layout::new(w, h, &state);
                assert!(layout.panel.right() <= w && layout.panel.bottom() <= h);
                assert!(layout.panel.contains(layout.close.center()));
                assert!(layout.panel.contains(layout.fold.center()));
            }
        }
    }
    fn with_drones() -> AsciiApp {
        let mut app = AsciiApp::new_agent_drone_trial().unwrap();
        let player = app.game.player_position().unwrap();
        for (id, dx, dy) in [("D58", -1, 0), ("D59", 0, -1), ("D66", -1, -1)] {
            assert_eq!(
                app.execute_command(GameCommand::UseTechniqueAt {
                    technique: project_rl::skills::agent::technique(id),
                    target: GridPos::new(player.x + dx, player.y + dy),
                    weapon_slot: 0,
                }),
                CommandOutcome::Applied
            );
        }
        app.capture_events();
        app
    }
    fn pointer(p: Vec2, pressed: bool, down: bool) -> InputFrame {
        InputFrame {
            pointer: Some(p.into()),
            viewport: Some((1280., 800.)),
            mouse_left_down: down,
            pressed: if pressed {
                [controls::Binding::MouseLeft].into()
            } else {
                Default::default()
            },
            ..Default::default()
        }
    }
    #[test]
    fn dragging_folding_closing_and_reopening_preserve_simulation_and_order() {
        let mut app = with_drones();
        let before = suspension::fingerprint(&app.game);
        let commands = app.history.len();
        app.update_input(&pointer(
            crate::terminal_chrome::command_rect(1280., 800., 10).center(),
            true,
            true,
        ));
        let layout = Layout::new(1280., 800., &app.ux.familiars);
        let order = app.game.player_controlled_companions();
        let rows = app.familiar_rows(&layout, &order);
        app.update_input(&pointer(rows[0].1.center(), true, true));
        app.update_input(&pointer(rows[1].1.center(), false, true));
        app.update_input(&pointer(rows[1].1.center(), false, false));
        assert_eq!(app.ux.familiars.order[0], order[1]);
        let reordered = app.ux.familiars.order.clone();
        app.update_input(&pointer(layout.fold.center(), true, false));
        assert!(app.ux.familiars.folded);
        app.update_input(&pointer(layout.fold.center(), true, false));
        assert!(!app.ux.familiars.folded);
        app.update_input(&pointer(layout.close.center(), true, false));
        assert!(!app.ux.familiars.open);
        app.update_input(&pointer(
            crate::terminal_chrome::command_rect(1280., 800., 10).center(),
            true,
            false,
        ));
        assert_eq!(app.ux.familiars.order, reordered);
        app.test_lab = false; // Metadata only: no file is saved by this test.
        let saved = app.suspension_metadata().unwrap();
        assert_eq!(
            saved.familiar_order,
            reordered.iter().map(|id| id.get()).collect::<Vec<_>>()
        );
        let mut json = serde_json::to_value(saved).unwrap();
        json.as_object_mut().unwrap().remove("familiar_order");
        assert!(
            serde_json::from_value::<Suspension>(json)
                .unwrap()
                .familiar_order
                .is_empty()
        );
        assert_eq!(app.history.len(), commands);
        assert_eq!(suspension::fingerprint(&app.game), before);
    }
    #[test]
    fn individual_order_is_timeless_recorded_and_leaves_other_drones_unchanged() {
        let mut app = with_drones();
        app.ux.familiars.open = true;
        app.ux
            .familiars
            .sync(&app.game.player_controlled_companions());
        let selected = app.ux.familiars.selected.unwrap();
        let before: Vec<_> = app
            .game
            .player_controlled_companions()
            .into_iter()
            .map(|id| {
                (
                    id,
                    app.game
                        .actors()
                        .get(id)
                        .unwrap()
                        .drone()
                        .unwrap()
                        .order()
                        .clone(),
                )
            })
            .collect();
        let turn = app.game.turn();
        let layout = Layout::new(1280., 800., &app.ux.familiars);
        app.update_input(&pointer(layout.scope.center(), true, false));
        assert!(app.ux.familiars.individual_orders);
        app.update_input(&pointer(layout.behavior_buttons[3].center(), true, false));
        assert_eq!(app.game.turn(), turn);
        for (id, order) in before {
            let actual = app.game.actors().get(id).unwrap().drone().unwrap().order();
            if id == selected {
                assert!(matches!(
                    actual,
                    DroneOrder::Companion {
                        behavior: CompanionBehavior::Passive,
                        ..
                    }
                ));
            } else {
                assert_eq!(actual, &order);
            }
        }
        let expected = GameCommand::SetCompanionBehaviorFor {
            entity: selected,
            behavior: CompanionBehavior::Passive,
        };
        let recorded = app.history.last().unwrap();
        assert_eq!(recorded.command(&app.game), Ok(expected));
        let json = serde_json::to_string(recorded).unwrap();
        let decoded: RecordedCommand = serde_json::from_str(&json).unwrap();
        assert_eq!(decoded.command(&app.game), recorded.command(&app.game));
    }
    #[test]
    fn first_destroy_click_only_asks_and_escape_cancels() {
        let mut app = with_drones();
        app.ux.familiars.open = true;
        app.ux
            .familiars
            .sync(&app.game.player_controlled_companions());
        let before = suspension::fingerprint(&app.game);
        let layout = Layout::new(1280., 800., &app.ux.familiars);
        app.update_input(&pointer(layout.dismiss.center(), true, false));
        assert!(app.ux.familiars.confirm.is_some());
        assert_eq!(suspension::fingerprint(&app.game), before);
        app.update_input(&InputFrame {
            pause: true,
            ..Default::default()
        });
        assert!(app.ux.familiars.confirm.is_none());
        assert!(app.ux.familiars.open);
        assert_eq!(suspension::fingerprint(&app.game), before);
    }
}
