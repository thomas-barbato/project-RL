//! Read-only, cached starting-loadout cards. Never use or modify the current run.
use super::*;

pub(super) struct StartingItem {
    pub id: ContentId,
    pub title: String,
    pub quantity: u16,
    pub subtitle: String,
    pub palette: project_rl::entity::EquipmentPalette,
    pub lines: Vec<String>,
}

pub(super) struct Preview {
    pub class: CharacterClassId,
    pub attributes: PrimaryAttributes,
    seed: u64,
    pub items: Vec<StartingItem>,
    pub skills: Vec<agent_skill_window::StartingSkillCard>,
    pub scroll: ux::ScrollState,
    pub tooltip_scroll: ux::ScrollState,
    pub tooltip: Option<CharacterCreationHover>,
}

pub(super) struct DetailsLayout {
    pub body: Rect,
    pub items: Vec<Rect>,
    pub skills: Vec<Rect>,
    pub item_heading: f32,
    pub skill_heading: f32,
    pub attributes_y: f32,
    bottom: f32,
}

impl DetailsLayout {
    pub fn new(layout: &CharacterCreationLayout, items: usize, skills: usize, offset: f32) -> Self {
        let panel = layout.panel;
        let x = panel.x + layout.sidebar_width() + 24.;
        let body = Rect::new(
            x,
            panel.y + 198. + layout.body_offset,
            panel.right() - x - 24.,
            layout.continue_button.y - 14. - (panel.y + 198. + layout.body_offset),
        );
        let columns = ((body.w + 8.) / 204.).floor().clamp(1., 2.) as usize;
        let width = (body.w - (columns - 1) as f32 * 8.) / columns as f32;
        let height = if body.h >= 350. {
            96.
        } else if body.h >= 250. {
            72.
        } else {
            64.
        };
        let stride = height + 8.;
        let grid = |count: usize, y: f32| {
            (0..count)
                .map(|i| {
                    Rect::new(
                        body.x + (i % columns) as f32 * (width + 8.),
                        y + (i / columns) as f32 * stride - offset,
                        width,
                        height,
                    )
                })
                .collect()
        };
        let item_y = body.y + 24.;
        let skill_heading = item_y + items.div_ceil(columns) as f32 * stride + 12.;
        let skill_y = skill_heading + 12.;
        let footer_gap = if body.h >= 250. { 18. } else { 12. };
        let attributes_y = skill_y + skills.div_ceil(columns) as f32 * stride + footer_gap;
        Self {
            body,
            items: grid(items, item_y),
            skills: grid(skills, skill_y),
            item_heading: body.y + 16. - offset,
            skill_heading: skill_heading - offset,
            attributes_y: attributes_y - offset,
            bottom: attributes_y + 26.,
        }
    }

    fn hit(&self, point: Vec2) -> Option<CharacterCreationHover> {
        if !self.body.contains(point) {
            return None;
        }
        self.items
            .iter()
            .position(|r| r.contains(point))
            .map(CharacterCreationHover::StartingItem)
            .or_else(|| {
                self.skills
                    .iter()
                    .position(|r| r.contains(point))
                    .map(CharacterCreationHover::StartingSkill)
            })
    }

    fn anchor(&self, hover: CharacterCreationHover) -> Option<Rect> {
        match hover {
            CharacterCreationHover::StartingItem(i) => self.items.get(i).copied(),
            CharacterCreationHover::StartingSkill(i) => self.skills.get(i).copied(),
            _ => None,
        }
    }
}

pub(super) fn tooltip_rect(layout: &CharacterCreationLayout) -> Rect {
    Rect::new(
        layout.panel.x + 16.,
        layout.panel.y + 88.,
        layout.sidebar_width() - 8.,
        layout.continue_button.y - layout.panel.y - 100.,
    )
}

fn section_heading(title: &str, x: f32, y: f32, width: f32) {
    draw_text_bold(title, x, y, 16., UiTheme.muted());
    let start = x + measure_text_bold(title, 16).width + 16.;
    if start < x + width - 8. {
        let mut color = UiTheme.muted();
        color.a = 0.18;
        draw_line(start, y - 5., x + width - 6., y - 5., 1., color);
    }
}

fn frame_icon(rect: Rect) {
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        Color::from_rgba(7, 10, 14, 255),
    );
    let mut border = UiTheme.muted();
    border.a = 0.22;
    draw_rectangle_lines(
        rect.x - 1.,
        rect.y - 1.,
        rect.w + 2.,
        rect.h + 2.,
        1.,
        border,
    );
}

fn icon_frame(tile: Rect) -> Rect {
    let rect = Rect::new(tile.x + 5., tile.y + (tile.h - 64.) * 0.5, 64., 64.);
    frame_icon(rect);
    rect
}

// Keep an attribute label and its number together when the footer wraps.
fn attribute_summary(attributes: PrimaryAttributes, x: f32, mut y: f32, width: f32) -> f32 {
    let mut line = String::new();
    for attribute in PrimaryAttribute::ALL {
        let pair = format!(
            "{} {}",
            primary_attribute_label(attribute),
            attributes.value(attribute)
        );
        let next = if line.is_empty() {
            pair.clone()
        } else {
            format!("{line} · {pair}")
        };
        if !line.is_empty() && measure_text(&next, None, 13, 1.).width > width {
            draw_text(&line, x, y, 13., UiTheme.accent());
            y += 17.;
            line = pair;
        } else {
            line = next;
        }
    }
    if !line.is_empty() {
        draw_text(&line, x, y, 13., UiTheme.accent());
    }
    y + 4.
}

fn bold_wrapped(title: &str, x: f32, mut y: f32, width: f32, size: u16, maximum: usize) -> f32 {
    let mut line = String::new();
    let mut rows = 0;
    for word in title.split_whitespace() {
        let next = if line.is_empty() {
            word.to_owned()
        } else {
            format!("{line} {word}")
        };
        if !line.is_empty() && measure_text_bold(&next, size).width > width {
            draw_text_bold(&line, x, y, f32::from(size), UiTheme.text());
            y += f32::from(size) + 3.;
            rows += 1;
            line.clear();
            if rows >= maximum {
                return y;
            }
        }
        if !line.is_empty() {
            line.push(' ');
        }
        line.push_str(word);
    }
    if !line.is_empty() {
        draw_text_bold(&line, x, y, f32::from(size), UiTheme.text());
    }
    y
}

impl AsciiApp {
    pub(super) fn refresh_creation_details(
        &mut self,
        creation: &CharacterCreation,
    ) -> Result<(), String> {
        let (_, class) = self
            .character_classes
            .iter()
            .nth(creation.selected_class)
            .ok_or("Protocole de restauration sélectionné introuvable.")?;
        if self.ux.creation_details.as_ref().is_some_and(|p| {
            p.class == *class.id() && p.attributes == creation.attributes && p.seed == creation.seed
        }) {
            return Ok(());
        }
        let mut rules = self.rules.clone();
        class
            .apply_to_rules(&mut rules, creation.attributes)
            .map_err(|e| e.to_string())?;
        weapon_supplies_app::ensure_class_weapon_supplies(&mut rules, CURRENT_GENERATION_VERSION);
        let game = GameState::new_with_rules(
            project_rl::world::Map::filled(3, 3, project_rl::world::Terrain::Floor)
                .map_err(|e| e.to_string())?,
            GridPos::new(1, 1),
            creation.seed,
            rules_for_generation_version(rules, CURRENT_GENERATION_VERSION),
        )
        .map_err(|e| e.to_string())?;
        let items = game
            .player_inventory()
            .iter()
            .map(|entry| StartingItem {
                id: entry.item().clone(),
                title: self.inventory_entry_name(entry),
                quantity: entry.quantity(),
                subtitle: if entry.quantity() > 1 {
                    format!("×{}", entry.quantity())
                } else {
                    game.player_item_weapon(entry.instance())
                        .map(|weapon| Self::weapon_category_label_for_game(&game, &weapon))
                        .unwrap_or_default()
                },
                palette: entry.palette(),
                lines: self.inventory_item_lines_for_game(&game, entry.instance(), false),
            })
            .collect();
        let skills = class
            .agent_start()
            .into_iter()
            .flat_map(|start| &start.skills)
            .filter_map(|id| agent_skill_window::starting_skill_card(&game, id))
            .collect();
        self.ux.creation_details = Some(Preview {
            class: class.id().clone(),
            attributes: creation.attributes,
            seed: creation.seed,
            items,
            skills,
            scroll: Default::default(),
            tooltip_scroll: Default::default(),
            tooltip: None,
        });
        Ok(())
    }

    /// Returns true when the floating reader covers the class list under the pointer.
    pub(super) fn update_creation_details_hover(
        &mut self,
        creation: &mut CharacterCreation,
        input: &InputFrame,
        layout: &CharacterCreationLayout,
    ) -> bool {
        let Some(preview) = &mut self.ux.creation_details else {
            return false;
        };
        if creation.stage != CharacterCreationStage::Protocol {
            preview.tooltip = None;
            return false;
        }
        let popup = tooltip_rect(layout);
        let mut details = DetailsLayout::new(
            layout,
            preview.items.len(),
            preview.skills.len(),
            preview.scroll.offset,
        );
        let point = input.pointer.map(Vec2::from);
        let in_popup = preview.tooltip.is_some() && point.is_some_and(|p| popup.contains(p));
        if input.wheel_y != 0. {
            let scroll = if in_popup {
                Some(&mut preview.tooltip_scroll)
            } else if point.is_some_and(|p| details.body.contains(p)) {
                Some(&mut preview.scroll)
            } else {
                None
            };
            if let Some(scroll) = scroll {
                scroll.offset =
                    (scroll.offset - input.wheel_y * 36.).clamp(0., scroll.maximum.get());
                details = DetailsLayout::new(
                    layout,
                    preview.items.len(),
                    preview.skills.len(),
                    preview.scroll.offset,
                );
            }
        }
        let hit = point.and_then(|p| details.hit(p));
        let bridge = preview
            .tooltip
            .and_then(|h| details.anchor(h))
            .map(|r| Rect::new(popup.right(), r.y, (r.x - popup.right()).max(0.), r.h));
        let retained = in_popup || point.is_some_and(|p| bridge.is_some_and(|r| r.contains(p)));
        let next = hit.or_else(|| retained.then_some(preview.tooltip).flatten());
        if next != preview.tooltip {
            preview.tooltip_scroll = Default::default();
        }
        preview.tooltip = next;
        if let Some(h) = next {
            creation.hovered = Some(h);
        }
        in_popup
    }

    pub(super) fn draw_creation_starting_details(
        &self,
        creation: &CharacterCreation,
        main: &CharacterCreationLayout,
    ) {
        let Some(preview) = &self.ux.creation_details else {
            return;
        };
        let layout = DetailsLayout::new(
            main,
            preview.items.len(),
            preview.skills.len(),
            preview.scroll.offset,
        );
        let body = layout.body;
        // Same logical UI coordinates and clip transform as the familiar reader.
        let scale = self.ui_scale();
        unsafe {
            macroquad::window::get_internal_gl().quad_gl.scissor(Some((
                (body.x * scale) as i32,
                (body.y * scale) as i32,
                (body.w * scale) as i32,
                (body.h * scale) as i32,
            )));
        }
        crate::ui_theme::begin_text_pane(body, 0.);
        section_heading("Équipement de départ", body.x, layout.item_heading, body.w);
        for (i, (item, tile)) in preview.items.iter().zip(&layout.items).enumerate() {
            if !tile.overlaps(&body) {
                continue;
            }
            UiTheme.card(
                *tile,
                preview.tooltip == Some(CharacterCreationHover::StartingItem(i)),
            );
            let icon = icon_frame(*tile);
            crate::game_art::draw_item_variant(icon, item.id.as_str(), item.palette);
            let roomy = tile.h >= 88.;
            bold_wrapped(
                &item.title,
                tile.x + 80.,
                tile.y + if roomy { 32. } else { 24. },
                tile.w - 88.,
                if roomy { 16 } else { 14 },
                2,
            );
            draw_text(
                &item.subtitle,
                tile.x + 80.,
                tile.bottom() - if roomy { 16. } else { 10. },
                if roomy { 13. } else { 12. },
                UiTheme.muted(),
            );
        }
        section_heading(
            "Compétences de départ",
            body.x,
            layout.skill_heading,
            body.w,
        );
        for (i, (skill, tile)) in preview.skills.iter().zip(&layout.skills).enumerate() {
            if !tile.overlaps(&body) {
                continue;
            }
            UiTheme.card(
                *tile,
                preview.tooltip == Some(CharacterCreationHover::StartingSkill(i)),
            );
            let icon = icon_frame(*tile);
            agent_skill_window::art(&skill.id, "compact", icon, Some(body));
            let roomy = tile.h >= 88.;
            bold_wrapped(
                &skill.title,
                tile.x + 80.,
                tile.y + if roomy { 32. } else { 24. },
                tile.w - 88.,
                if roomy { 16 } else { 14 },
                2,
            );
            draw_text(
                format!("{} · Rang {}", skill.kind, skill.rank),
                tile.x + 80.,
                tile.bottom() - if roomy { 16. } else { 10. },
                if roomy { 13. } else { 12. },
                UiTheme.muted(),
            );
        }
        let mut divider = UiTheme.muted();
        divider.a = 0.2;
        draw_line(
            body.x,
            layout.attributes_y - 18.,
            body.right() - 6.,
            layout.attributes_y - 18.,
            1.,
            divider,
        );
        let attributes_bottom = attribute_summary(
            creation.attributes,
            body.x,
            layout.attributes_y,
            body.w - 8.,
        );
        crate::ui_theme::end_text_pane();
        unsafe {
            macroquad::window::get_internal_gl().quad_gl.scissor(None);
        }
        preview.scroll.finish(
            body,
            layout.bottom.max(attributes_bottom + preview.scroll.offset),
        );
    }

    pub(super) fn draw_creation_details_tooltip(&self, layout: &CharacterCreationLayout) {
        let Some(preview) = &self.ux.creation_details else {
            return;
        };
        let Some(hover) = preview.tooltip else {
            return;
        };
        let (title, subtitle, lines) = match hover {
            CharacterCreationHover::StartingItem(i) => {
                let Some(item) = preview.items.get(i) else {
                    return;
                };
                (
                    item.title.as_str(),
                    format!("Quantité : {}", item.quantity),
                    &item.lines,
                )
            }
            CharacterCreationHover::StartingSkill(i) => {
                let Some(skill) = preview.skills.get(i) else {
                    return;
                };
                (
                    skill.title.as_str(),
                    format!("{} · Rang {}", skill.kind, skill.rank),
                    &skill.lines,
                )
            }
            _ => return,
        };
        let rect = tooltip_rect(layout);
        UiTheme.panel(rect);
        let icon = Rect::new(rect.right() - 76., rect.y + 12., 64., 64.);
        frame_icon(icon);
        match hover {
            CharacterCreationHover::StartingItem(i) => {
                let item = &preview.items[i];
                crate::game_art::draw_item_variant(icon, item.id.as_str(), item.palette);
            }
            CharacterCreationHover::StartingSkill(i) => {
                agent_skill_window::art(&preview.skills[i].id, "compact", icon, None);
            }
            _ => {}
        }
        let title_end = bold_wrapped(title, rect.x + 14., rect.y + 28., rect.w - 104., 18, 3);
        draw_text(
            subtitle,
            rect.x + 12.,
            title_end + 18.,
            13.,
            UiTheme.muted(),
        );
        let top = (title_end + 36.).max(rect.y + 94.);
        let mut divider = UiTheme.muted();
        divider.a = 0.22;
        draw_line(
            rect.x + 14.,
            top - 12.,
            rect.right() - 14.,
            top - 12.,
            1.,
            divider,
        );
        let body = Rect::new(rect.x + 12., top, rect.w - 28., rect.bottom() - top - 26.);
        crate::ui_theme::begin_text_pane(body, preview.tooltip_scroll.offset);
        let mut y = body.y + 16.;
        for line in lines {
            if matches!(line.as_str(), "Effet" | "Prochain rang" | "Plafond atteint") {
                draw_text_bold(line, body.x, y, 15., UiTheme.accent());
                y += 24.;
            } else {
                y = draw_wrapped_text(line, body.x, y, body.w - 8., 40, 14, UiTheme.text()) + 8.;
            }
        }
        crate::ui_theme::end_text_pane();
        preview.tooltip_scroll.finish(body, y + 8.);
        if preview.tooltip_scroll.maximum.get() > 0. {
            draw_text(
                "Molette",
                rect.x + 12.,
                rect.bottom() - 9.,
                12.,
                UiTheme.muted(),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> AsciiApp {
        let (r, t, l, e) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(1872, r, t, l, e).unwrap();
        app.crash_recovery_enabled = false;
        app.suspension_path =
            std::env::temp_dir().join(format!("rl-creation-cards-{}.json", std::process::id()));
        app.controls = Controls::preset(controls::Layout::Qwerty, KeySemantics::Physical);
        app
    }

    fn choose(app: &mut AsciiApp, index: usize) {
        app.begin_character_creation_with_seed(false, 1872).unwrap();
        let mut creation = app.character_creation.take().unwrap();
        creation.selected_class = index;
        creation.attributes = app
            .character_classes
            .iter()
            .nth(index)
            .unwrap()
            .1
            .recommended_attributes();
        app.refresh_creation_details(&creation).unwrap();
        app.character_creation = Some(creation);
    }

    #[test]
    fn creation_cards_match_real_starting_items_skills_and_class_costs_without_touching_the_run() {
        let mut app = fixture();
        let before = (
            suspension::fingerprint(&app.game),
            app.game.turn(),
            app.game.rng_state(),
            app.history.len(),
        );
        for index in 0..4 {
            choose(&mut app, index);
            let preview = app.ux.creation_details.as_ref().unwrap();
            let mut actual = fixture();
            let creation = app.character_creation.as_ref().unwrap().clone();
            actual.rebuild_run_with_character_class(&creation).unwrap();
            let expected: Vec<_> = actual
                .game
                .player_inventory()
                .iter()
                .map(|e| (e.item().clone(), e.quantity()))
                .collect();
            assert_eq!(
                preview
                    .items
                    .iter()
                    .map(|e| (e.id.clone(), e.quantity))
                    .collect::<Vec<_>>(),
                expected
            );
            assert_eq!(preview.skills.len(), 2);
            for item in &preview.items {
                let entry = actual
                    .game
                    .player_inventory()
                    .iter()
                    .find(|e| *e.item() == item.id)
                    .unwrap();
                assert_eq!(
                    item.lines,
                    actual.inventory_item_lines_for_game(&actual.game, entry.instance(), false)
                );
            }
            for skill in &preview.skills {
                assert_eq!(skill.rank, actual.game.player_agent_skill_rank(&skill.id));
                if skill.kind == "Actif" {
                    let profile = actual
                        .game
                        .agent_skill_ready_with_weapon(
                            &skill.id,
                            actual.preferred_starting_weapon_slot(),
                        )
                        .unwrap();
                    let cost = format!(
                        "{} : {}",
                        agent_skill_window::parameter_label("energy").unwrap(),
                        profile.energy
                    );
                    assert!(
                        skill.lines.contains(&cost),
                        "{}: {:?}; expected {cost}",
                        skill.id,
                        skill.lines
                    );
                }
            }
        }
        assert_eq!(
            (
                suspension::fingerprint(&app.game),
                app.game.turn(),
                app.game.rng_state(),
                app.history.len()
            ),
            before
        );
    }

    #[test]
    fn creation_hover_clicks_and_long_tooltip_scroll_are_readers_and_clear_on_class_change() {
        let mut app = fixture();
        choose(&mut app, 3); // Mue prédatrice has a long description and several effects.
        let before = (suspension::fingerprint(&app.game), app.history.len());
        let main = CharacterCreationLayout::new(1280., 800., 4);
        let p = app.ux.creation_details.as_ref().unwrap();
        let layout = DetailsLayout::new(&main, p.items.len(), p.skills.len(), 0.);
        let at = |r: Rect| InputFrame {
            pointer: Some(r.center().into()),
            viewport: Some((1280., 800.)),
            ..Default::default()
        };
        app.update_character_creation(&at(layout.items[0]));
        assert_eq!(
            app.ux.creation_details.as_ref().unwrap().tooltip,
            Some(CharacterCreationHover::StartingItem(0))
        );
        let mut skill = at(layout.skills[0]);
        skill.pressed.insert(controls::Binding::MouseLeft);
        app.update_character_creation(&skill);
        assert_eq!(
            app.ux.creation_details.as_ref().unwrap().tooltip,
            Some(CharacterCreationHover::StartingSkill(0))
        );
        app.ux
            .creation_details
            .as_ref()
            .unwrap()
            .tooltip_scroll
            .maximum
            .set(400.);
        let mut wheel = at(tooltip_rect(&main));
        wheel.wheel_y = -1.;
        wheel.pressed.insert(controls::Binding::MouseLeft);
        app.update_character_creation(&wheel);
        assert_eq!(app.character_creation.as_ref().unwrap().selected_class, 3);
        assert_eq!(
            app.ux
                .creation_details
                .as_ref()
                .unwrap()
                .tooltip_scroll
                .offset,
            36.
        );
        app.update_character_creation(&InputFrame {
            pressed: [controls::Binding::key("Down")].into(),
            viewport: Some((1280., 800.)),
            ..Default::default()
        });
        assert_eq!(app.character_creation.as_ref().unwrap().selected_class, 0);
        let p = app.ux.creation_details.as_ref().unwrap();
        assert_eq!(p.class.as_str(), "core:bastion");
        assert!(p.tooltip.is_none());
        assert_eq!(
            (suspension::fingerprint(&app.game), app.history.len()),
            before
        );
    }

    #[test]
    fn creation_cards_clip_hits_to_the_reader_and_keep_actions_clear_in_compact_scaled_layouts() {
        for (width, height) in [(1280., 800.), (960., 540.), (853., 533.), (640., 360.)] {
            let main = CharacterCreationLayout::new(width, height, 4);
            let layout = DetailsLayout::new(&main, 8, 8, 140.);
            assert!(layout.body.bottom() < main.continue_button.y);
            assert!(tooltip_rect(&main).bottom() < main.continue_button.y);
            assert!(
                layout
                    .hit(vec2(layout.body.x + 10., layout.body.y - 1.))
                    .is_none()
            );
            assert!(layout.bottom > layout.body.bottom());
            for item in &layout.items {
                if layout.body.contains(item.center()) {
                    assert!(matches!(
                        layout.hit(item.center()),
                        Some(CharacterCreationHover::StartingItem(_))
                    ));
                }
            }
        }
    }
}
