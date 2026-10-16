    fn draw_character_creation(&self) {
        let Some(creation) = &self.character_creation else { return; };
        let theme = UiTheme;
        let classes = self.character_classes.iter().collect::<Vec<_>>();
        let layout = CharacterCreationLayout::new(self.ui_width(), self.ui_height(), classes.len());
        let panel = layout.panel;
        let left_width = panel.w * 0.43;
        let detail_x = panel.x + left_width + 24.0;
        let detail_width = panel.w - left_width - 48.0;
        let rules = self.rules.primary_attribute_rules;
        let remaining = rules.creation_total.saturating_sub(creation.attributes.total());
        let customizing = creation.stage == CharacterCreationStage::Attributes;
        draw_rectangle(0.0, 0.0, self.ui_width(), self.ui_height(), theme.backdrop());
        theme.panel(panel);
        draw_text_bold(if customizing { "Personnaliser les attributs" } else { "Choisir votre profil" }, panel.x + 24.0, panel.y + 40.0, 28.0, theme.text());
        draw_text(if customizing { "Retirez un point avec − pour le réattribuer avec +." } else { "Un équipement de départ et une façon d'aborder l'expédition." }, panel.x + 24.0, panel.y + 66.0, 16.0, theme.muted());
        let selected = classes.get(creation.selected_class).map(|(_, class)| *class);
        if !customizing {
            for (index, ((_, class), row)) in classes.iter().zip(&layout.class_rows).enumerate() {
                theme.card(*row, index == creation.selected_class);
                let icon = [UiIcon::Attack, UiIcon::Techniques, UiIcon::Shield][index % 3];
                draw_ui_icon(icon, Rect::new(row.x + 14.0, row.y + 15.0, 24.0, 24.0), theme.accent());
                let name = self.texts.resolve(DISPLAY_LOCALE, class.name_key()).unwrap_or("Profil");
                let role = self.texts.resolve(DISPLAY_LOCALE, class.role_key()).unwrap_or("");
                draw_text_bold(name, row.x + 50.0, row.y + 25.0, 20.0, theme.text());
                draw_wrapped_text(role, row.x + 50.0, row.y + 46.0, row.w - 62.0, 1, 14, theme.muted());
            }
        } else {
            draw_text_bold(format!("{remaining} point{} à répartir", if remaining > 1 { "s" } else { "" }), panel.x + 28.0, panel.y + 98.0, 17.0, if remaining > 0 { theme.attention() } else { theme.accent() });
            for (index, attribute) in PrimaryAttribute::ALL.into_iter().enumerate() {
                let row = layout.attribute_rows[index];
                theme.card(row, index == creation.selected_attribute);
                let value = creation.attributes.value(attribute);
                draw_text(primary_attribute_label(attribute), row.x + 12.0, row.y + 28.0, 17.0, theme.text());
                draw_text_bold_centered(&value.to_string(), Rect::new(row.right() - 81.0, row.y, 43.0, row.h), 22, theme.text());
                for (button, symbol, hover, enabled) in [
                    (layout.attribute_minus[index], "−", CharacterCreationHover::AttributeMinus(index), value > rules.creation_minimum),
                    (layout.attribute_plus[index], "+", CharacterCreationHover::AttributePlus(index), value < rules.creation_maximum && remaining > 0),
                ] {
                    theme.button(button, symbol, creation.hovered == Some(hover), false, enabled, ButtonTone::Secondary);
                }
            }
            draw_wrapped_text(&format!("Valeurs de {} à {} · {} points au total", rules.creation_minimum, rules.creation_maximum, rules.creation_total), panel.x + 28.0, layout.attribute_rows.last().unwrap().bottom() + 24.0, left_width - 44.0, 2, 14, theme.muted());
        }
        if let Some(class) = selected {
            let name = self.texts.resolve(DISPLAY_LOCALE, class.name_key()).unwrap_or("Profil");
            draw_text_bold(name, detail_x, panel.y + 110.0, 25.0, theme.accent());
            let profile = creation.attributes;
            if customizing {
                let recommended = profile == class.recommended_attributes();
                draw_text(if recommended { "Profil recommandé" } else { "Répartition personnalisée" }, detail_x, panel.y + 139.0, 16.0, theme.muted());
                let attr = PrimaryAttribute::ALL[creation.selected_attribute];
                draw_text_bold(primary_attribute_label(attr), detail_x, panel.y + 180.0, 19.0, theme.text());
                let bottom = draw_wrapped_text(primary_attribute_description(attr), detail_x, panel.y + 206.0, detail_width, 6, 15, theme.text());
                let preview_y = (bottom + 26.0).min(panel.y + panel.h - 179.0);
                self.draw_creation_preview(profile, Rect::new(detail_x, preview_y, detail_width, 42.0));
            } else {
                let description = self.texts.resolve(DISPLAY_LOCALE, class.description_key()).unwrap_or("");
                let y = draw_wrapped_text(description, detail_x, panel.y + 143.0, detail_width, 4, 16, theme.text());
                let equipment = class.starting_weapons().iter().map(|id| self.item_name(id))
                    .chain(class.starting_items().iter().map(|item| format!("{} ×{}", self.item_name(item.item()), item.quantity())))
                    .collect::<Vec<_>>().join(" · ");
                draw_text_bold("Équipement de départ", detail_x, y + 26.0, 16.0, theme.muted());
                let y = draw_wrapped_text(&equipment, detail_x, y + 49.0, detail_width, 3, 15, theme.text());
                let profile_text = PrimaryAttribute::ALL.into_iter().map(|a| format!("{} {}", primary_attribute_label(a), profile.value(a))).collect::<Vec<_>>().join(" · ");
                draw_wrapped_text(&profile_text, detail_x, y + 28.0, detail_width, 2, 14, theme.accent());
            }
        }
        theme.button(layout.preset, if customizing { "Rétablir le profil recommandé" } else { "Personnaliser les attributs [Tab]" }, creation.hovered == Some(CharacterCreationHover::Preset), false, true, ButtonTone::Secondary);
        theme.button(layout.cancel, if customizing { "Retour aux profils" } else { "Retour" }, creation.hovered == Some(CharacterCreationHover::Cancel), false, true, ButtonTone::Secondary);
        theme.button(layout.continue_button, if customizing { "Commencer la partie" } else { "Commencer avec ce profil" }, creation.hovered == Some(CharacterCreationHover::Continue), false, remaining == 0, ButtonTone::Primary);
        if !creation.message.is_empty() || remaining > 0 {
            let message = if creation.message.is_empty() { "Répartissez tous les points pour commencer." } else { &creation.message };
            draw_wrapped_text(message, panel.x + 192.0, panel.bottom() - 38.0, panel.w - 480.0, 2, 14, theme.attention());
        }
    }

    fn draw_creation_preview(&self, attributes: PrimaryAttributes, rect: Rect) {
        let hp = self.rules.physical_rules.zip(self.rules.player_body_profile).map_or(self.rules.player_maximum_integrity, |(rules, body)| rules.hit_points.maximum_for_body(body, Some(attributes), 0));
        let evasion = self.rules.hit_rules.map_or(0, |rules| rules.evasion(Some(attributes), 0));
        UiTheme.card(rect, false);
        draw_text_bold(format!("Résultat · {hp} PV · Esquive {evasion}"), rect.x + 12.0, rect.y + 27.0, 16.0, UiTheme.accent());
    }
