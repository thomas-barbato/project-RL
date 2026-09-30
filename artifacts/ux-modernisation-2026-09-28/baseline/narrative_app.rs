//! Presentation adapter for the first optional investigation.
use super::*;
use project_rl::content::{
    DataRecordQuestDefinition, HubQuestDefinition, QuestDefinition, QuestWorldEffectDefinition,
    QuestWorldStateDefinition,
};
use project_rl::game::DialogueView;

impl AsciiApp {
    fn narrative_reward_experience(&self) -> u64 {
        self.expeditions
            .get(&"core:starter_expedition".parse().unwrap())
            .map_or(0, |definition| definition.narrative_reward_experience)
    }

    fn narrative_reward_summary(&self, provider: EntityId) -> Option<String> {
        let interaction = self.game.npc_interaction(provider)?;
        let quest = interaction.quests.first()?;
        Some(if quest.status == QuestStatus::Completed {
            format!(
                "QUÊTE TERMINÉE · +{} crédits · {} XP",
                quest.reward_credits, quest.reward_experience
            )
        } else {
            format!(
                "RÉCOMPENSE · {} crédits · {} XP",
                quest.reward_credits, quest.reward_experience
            )
        })
    }

    pub(super) fn narrative_text(&self, key: &str, fallback: &str) -> String {
        let text = self.texts.resolve(DISPLAY_LOCALE, key).unwrap_or(fallback);
        if !text.contains("{passage_coordinates}") {
            return text.to_owned();
        }
        let passage = self
            .expeditions
            .get(&"core:starter_expedition".parse().unwrap())
            .map(|expedition| {
                expedition.hub_passage_for_expanded_world(
                    self.generation_version >= EXPANDED_WORLD_GENERATION_VERSION,
                )
            });
        text.replace(
            "{passage_coordinates}",
            &crate::terminal_view::local_coordinates(passage),
        )
    }

    pub(super) fn narrative_navigation_signal_summary(&self) -> Option<String> {
        let expedition = self
            .expeditions
            .get(&"core:starter_expedition".parse().ok()?)?;
        let narrative = expedition.narrative.as_ref()?;
        if !self.game.quest_journal().iter().any(|entry| {
            entry.quest.id == narrative.investigation.id
                && entry.quest.status == QuestStatus::Active
        }) {
            return None;
        }
        let zone = self.game.current_zone()?;
        if zone.id == expedition.hub.id {
            // Elias supplies this known entrance; no unexplored tile is revealed.
            let passage = expedition.hub_passage_for_expanded_world(
                self.generation_version >= EXPANDED_WORLD_GENERATION_VERSION,
            );
            return Some(format!(
                "{} · DESCENDRE VERS LE SECTEUR INDUSTRIEL",
                crate::terminal_view::local_coordinates(Some(passage))
            ));
        }
        if zone.id == expedition.destination.zone.id {
            // A generated relay has no exact location until it is in sight.
            if let Some(facility) = self.game.active_facility()
                && let Some(position) = facility.installations().find_map(|(_, installation)| {
                    let position = installation.position();
                    (self.game.player_visibility().is_visible(position)
                        && facility.data_terminal_record_at(position)
                            == Some(&narrative.investigation.record))
                    .then_some(position)
                })
            {
                return Some(format!(
                    "{} · REGISTRE DU RELAIS EN VUE",
                    crate::terminal_view::local_coordinates(Some(position))
                ));
            }
            return Some("RELAIS À LOCALISER DANS LE SECTEUR INDUSTRIEL".to_owned());
        }
        None
    }

    #[cfg(any(test, debug_assertions))]
    pub(super) fn prepare_narrative_reward_diagnostic(&mut self) -> Result<(), String> {
        self.prepare_narrative_diagnostic()?;
        let giver = self.npc_interaction.unwrap();
        for command in [
            GameCommand::ChooseDialogue {
                speaker: giver,
                node: "ABS-D01".into(),
                choice: 0,
            },
            GameCommand::ChooseDialogue {
                speaker: giver,
                node: "ABS-D02".into(),
                choice: 0,
            },
            GameCommand::Move(Direction::West),
            GameCommand::Interact {
                target: GridPos::new(2, 3),
            },
            GameCommand::Move(Direction::East),
            GameCommand::ChooseDialogue {
                speaker: giver,
                node: "ABS-D01".into(),
                choice: 4,
            },
        ] {
            if let CommandOutcome::Rejected(reason) = self.execute_command(command) {
                return Err(format!("Rapport de diagnostic refusé : {reason:?}"));
            }
            self.capture_events_at(Some(0.0));
        }
        Ok(())
    }

    #[cfg(any(test, debug_assertions))]
    pub(super) fn prepare_narrative_directions_diagnostic(&mut self) -> Result<(), String> {
        let approach = if self.generation_version >= STATIONARY_QUEST_CONTACT_GENERATION_VERSION {
            GridPos::new(11, 26)
        } else {
            GridPos::new(40, 22)
        };
        self.walk_fixture_to(approach)?;
        let giver = self
            .game
            .actors()
            .iter()
            .find_map(|(entity, actor)| {
                actor
                    .tags()
                    .contains(&"core:orme".parse().unwrap())
                    .then_some(entity)
            })
            .ok_or("Elias absent")?;
        for _ in 0..120 {
            let position = self
                .game
                .actors()
                .get(giver)
                .ok_or("Elias absent")?
                .position();
            if self
                .game
                .player_position()
                .is_some_and(|p| p.cardinal_neighbors().contains(&position))
                && self.game.player_visibility().is_visible(position)
            {
                let outcome = self.execute_command(GameCommand::ChooseDialogue {
                    speaker: giver,
                    node: "ABS-D01".into(),
                    choice: 0,
                });
                if outcome != CommandOutcome::AppliedWithoutTime {
                    return Err(format!("Enquête refusée : {outcome:?}"));
                }
                self.npc_interaction = Some(giver);
                self.capture_events_at(Some(0.0));
                return Ok(());
            }
            self.execute_command(GameCommand::Wait);
            self.capture_events_at(Some(0.0));
        }
        Err("Elias est inaccessible".into())
    }

    #[cfg(any(test, debug_assertions))]
    pub(super) fn prepare_narrative_diagnostic(&mut self) -> Result<(), String> {
        use project_rl::facility::{
            FacilityBlueprint, InstallationBlueprint, InstallationCapability,
        };
        let definition = self
            .expeditions
            .get(&"core:starter_expedition".parse().unwrap())
            .and_then(|expedition| expedition.narrative.clone())
            .ok_or("Narration absente")?;
        let mut map = project_rl::world::Map::from_ascii("############\n#..........#\n#......###.#\n#......#.#.#\n#......###.#\n#..........#\n############").map_err(|e| e.to_string())?;
        map.set_terrain(GridPos::new(2, 3), Terrain::Wall)
            .map_err(|e| e.to_string())?;
        let mut game =
            GameState::new_with_rules(map, GridPos::new(4, 3), INITIAL_SEED, self.rules.clone())
                .map_err(|e| e.to_string())?;
        let orme = game
            .spawn_actor(
                Actor::new(GridPos::new(5, 3), 12)
                    .unwrap()
                    .with_tags(["core:orme".parse().unwrap()]),
            )
            .map_err(|e| e.to_string())?;
        game.spawn_actor(
            Actor::new(GridPos::new(4, 5), 18)
                .unwrap()
                .with_tags([definition.relay_character.clone()]),
        )
        .map_err(|e| e.to_string())?;
        let mut world = WorldState::single(game);
        let zone: ContentId = "core:narrative_diagnostic".parse().unwrap();
        world.enable(project_rl::game::ZoneInfo {
            id: zone.clone(),
            name: "Le chemin des absents".into(),
            kind: "core:city".parse().unwrap(),
            depth: 0,
        })?;
        let quest = &definition.investigation;
        let mut authored = HubQuestDefinition::new(
            HubQuestProviderDefinition::Existing {
                position: GridPos::new(5, 3),
            },
            QuestDefinition::KnownFact(
                DataRecordQuestDefinition::new(
                    quest.id.clone(),
                    quest.title_key.clone(),
                    quest.summary_key.clone(),
                    quest.record.clone(),
                    quest.reward_credits,
                )
                .map_err(|e| e.to_string())?,
            ),
        );
        authored.reward_experience = self.narrative_reward_experience();
        world.register_authored_quest(zone.clone(), orme, authored)?;
        let depot: ContentId = "core:narrative_depot".parse().unwrap();
        world.register_facility(
            zone,
            FacilityBlueprint {
                installations: vec![
                    InstallationBlueprint {
                        id: depot.clone(),
                        position: GridPos::new(2, 3),
                        maximum_integrity: 10,
                        integrity: 10,
                        capabilities: vec![
                            InstallationCapability::Storage,
                            InstallationCapability::DataTerminal {
                                record: quest.record.clone(),
                            },
                        ],
                        dependencies: vec![],
                        security_alarm_profile: None,
                    },
                    InstallationBlueprint {
                        id: "core:hidden_record".parse().unwrap(),
                        position: GridPos::new(8, 3),
                        maximum_integrity: 10,
                        integrity: 10,
                        capabilities: vec![InstallationCapability::DataTerminal {
                            record: quest.record.clone(),
                        }],
                        dependencies: vec![],
                        security_alarm_profile: None,
                    },
                ],
                depot,
                workers: vec![],
                repair_orders: vec![],
                maximum_path_search: 512,
                owner: None,
            },
        )?;
        world.register_narrative(definition)?;
        world.drain_events();
        self.game = world;
        let mut decor = crate::test_sector::SectorDecor::default();
        decor.cells.insert(
            GridPos::new(2, 3),
            crate::test_sector::Decor::DataTerminalOnline,
        );
        self.terminal = TerminalView::new(decor, self.game.map(), self.game.player_visibility());
        self.actor_glyphs.clear();
        self.refresh_zone_title();
        self.history.clear();
        self.npc_interaction = Some(orme);
        self.npc_dialogue_services = false;
        self.npc_quest_selection = 0;
        self.intro_city_reached = true;
        self.log.clear();
        Ok(())
    }

    pub(super) fn enable_narrative(&mut self) -> Result<(), String> {
        let id: ContentId = "core:starter_expedition".parse().unwrap();
        let Some(definition) = self
            .expeditions
            .get(&id)
            .and_then(|expedition| expedition.narrative.clone())
        else {
            return Ok(());
        };
        for key in definition
            .text_keys()
            .chain(std::iter::once(definition.investigation.record.as_str()))
        {
            if self.texts.resolve(DISPLAY_LOCALE, key).is_none() {
                return Err(format!("Texte narratif absent : {key}"));
            }
        }
        let zone = self
            .game
            .current_zone()
            .ok_or("Ville narrative absente")?
            .id
            .clone();
        self.game.register_narrative(definition.clone())?;
        for character in &definition.characters {
            let Some([x, y]) = character.hub_position else {
                continue;
            };
            let position = GridPos::new(x, y);
            let provider = if let Some(provider) = self.game.actors().entity_at(position) {
                provider
            } else if self.generation_version >= STATIONARY_QUEST_CONTACT_GENERATION_VERSION
                && (character.tag == definition.investigation.provider_tag
                    || self.generation_version >= SURFACE_CAST_GENERATION_VERSION)
                && self.game.map().is_walkable(position)
            {
                self.game
                    .spawn_actor(
                        Actor::new(position, 10)
                            .map_err(|error| error.to_string())?
                            .with_ai(AiProfile::idle()),
                    )
                    .map_err(|error| error.to_string())?
            } else {
                return Err(format!("Personnage narratif absent : {}", character.tag));
            };
            self.game
                .bind_narrative_character(provider, &character.tag)?;
            if character.tag == definition.investigation.provider_tag {
                let quest = &definition.investigation;
                let mut authored = HubQuestDefinition::new(
                    HubQuestProviderDefinition::Existing { position },
                    QuestDefinition::KnownFact(
                        DataRecordQuestDefinition::new(
                            quest.id.clone(),
                            quest.title_key.clone(),
                            quest.summary_key.clone(),
                            quest.record.clone(),
                            quest.reward_credits,
                        )
                        .map_err(|error| error.to_string())?,
                    ),
                );
                authored.reward_experience = self.narrative_reward_experience();
                authored.completion_world_states.push(
                    QuestWorldStateDefinition::new(
                        "core:relay_reported".parse().unwrap(),
                        "narrative.abs.reported".into(),
                    )
                    .map_err(|error| error.to_string())?,
                );
                if self.generation_version >= ORME_DIRECTION_BOARD_GENERATION_VERSION {
                    authored.completion_world_effects.push(
                        QuestWorldEffectDefinition::update_data_terminal(
                            "core:orme_direction_board".parse().unwrap(),
                            "core:orme_direction_board_updated".parse().unwrap(),
                            "world_effect.orme_direction_board_updated.summary".into(),
                        )
                        .map_err(|error| error.to_string())?,
                    );
                }
                self.game
                    .register_authored_quest(zone.clone(), provider, authored)?;
            }
        }
        self.game.drain_events();
        Ok(())
    }

    pub(super) fn update_narrative_dialogue(
        &mut self,
        input: &InputFrame,
        provider: EntityId,
        dialogue: &DialogueView,
        layout: &NpcInteractionLayout,
    ) {
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        if self.controls.pressed(Action::Interact, input)
            || (clicked
                && input
                    .pointer
                    .is_some_and(|point| layout.close.contains(point.into())))
        {
            self.npc_interaction = None;
            self.npc_interaction_message.clear();
            return;
        }
        let count = dialogue.choices.len();
        if count == 0 {
            self.menu_focus.hovered = input
                .pointer
                .filter(|point| layout.close.contains((*point).into()))
                .map(|_| 1);
            return;
        }
        self.npc_quest_selection = self.npc_quest_selection.min(count - 1);
        let up = self.controls.pressed(Action::MenuUp, input);
        let down = self.controls.pressed(Action::MenuDown, input);
        if up {
            self.npc_quest_selection = (self.npc_quest_selection + count - 1) % count;
        } else if down {
            self.npc_quest_selection = (self.npc_quest_selection + 1) % count;
        }
        let first = self.npc_quest_selection.saturating_sub(3);
        let hovered = layout
            .trade_rows
            .iter()
            .enumerate()
            .find_map(|(row, rect)| {
                let index = first + row;
                (index < count
                    && input
                        .pointer
                        .is_some_and(|point| rect.contains(point.into())))
                .then_some(index)
            });
        self.menu_focus
            .update(hovered, &mut self.npc_quest_selection, clicked, up || down);
        self.menu_focus.hovered = hovered.map(|index| 3 + index).or_else(|| {
            input.pointer.and_then(|point| {
                let point = point.into();
                if layout.service.contains(point) {
                    Some(0)
                } else if layout.close.contains(point) {
                    Some(1)
                } else if layout.mode_toggle.contains(point)
                    && self
                        .game
                        .npc_interaction(provider)
                        .is_some_and(|npc| !npc.services.is_empty())
                {
                    Some(2)
                } else {
                    None
                }
            })
        });
        let activate = self.controls.pressed(Action::Learn, input)
            || (clicked
                && (hovered.is_some()
                    || input
                        .pointer
                        .is_some_and(|point| layout.service.contains(point.into()))));
        if !activate {
            return;
        }
        let choice = &dialogue.choices[self.npc_quest_selection];
        match self.execute_command(GameCommand::ChooseDialogue {
            speaker: provider,
            node: dialogue.node.clone(),
            choice: choice.index,
        }) {
            CommandOutcome::Rejected(reason) => {
                self.npc_interaction_message = command_rejection_message(reason).into()
            }
            _ => {
                self.npc_quest_selection = 0;
                self.npc_interaction_message.clear();
                self.capture_events();
            }
        }
    }

    pub(super) fn draw_narrative_dialogue(&self, provider: EntityId, dialogue: &DialogueView) {
        let theme = UiTheme;
        let layout = NpcInteractionLayout::new(self.ui_width(), self.ui_height());
        let text = |key: &str| {
            self.texts
                .resolve(DISPLAY_LOCALE, key)
                .unwrap_or("Texte absent")
        };
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            theme.backdrop(),
        );
        theme.card(layout.panel, false);
        let x = layout.panel.x + 22.0;
        draw_text_bold(
            text(&dialogue.name_key),
            x,
            layout.panel.y + 47.0,
            25.0,
            theme.text(),
        );
        draw_text(
            text(&dialogue.role_key),
            x,
            layout.panel.y + 74.0,
            15.0,
            theme.accent(),
        );
        draw_wrapped_text(
            &self.narrative_text(&dialogue.text_key, "Texte absent"),
            x,
            layout.panel.y + 108.0,
            layout.panel.w - 44.0,
            6,
            16,
            theme.text(),
        );
        let selection = self
            .npc_quest_selection
            .min(dialogue.choices.len().saturating_sub(1));
        let first = selection.saturating_sub(3);
        for (row, rect) in layout.trade_rows.iter().enumerate() {
            let Some(choice) = dialogue.choices.get(first + row) else {
                break;
            };
            let index = first + row;
            let hovered = self.menu_focus.hovered == Some(3 + index);
            let selected = self.menu_focus.highlighted(index, selection);
            theme.dialogue_choice(*rect, selected, hovered);
            draw_wrapped_text(
                text(&choice.text_key),
                rect.x + 26.0,
                rect.y + 20.0,
                rect.w - 38.0,
                2,
                14,
                if selected || hovered {
                    theme.accent()
                } else {
                    theme.text()
                },
            );
        }
        if self
            .game
            .npc_interaction(provider)
            .is_some_and(|npc| !npc.services.is_empty())
        {
            theme.dialogue_action(
                layout.mode_toggle,
                "SOINS",
                self.menu_focus.hovered == Some(2),
                true,
            );
        }
        let reward_summary = self.narrative_reward_summary(provider).unwrap_or_default();
        draw_wrapped_text(
            if self.npc_interaction_message.is_empty() {
                &reward_summary
            } else {
                &self.npc_interaction_message
            },
            x,
            layout.service.y - 27.0,
            layout.panel.w - 44.0,
            1,
            14,
            theme.accent(),
        );
        theme.dialogue_action(
            layout.service,
            &format!("{} : répondre", self.controls.label(Action::Learn)),
            self.menu_focus.hovered == Some(0),
            !dialogue.choices.is_empty(),
        );
        theme.dialogue_action(
            layout.close,
            &format!("{} : partir", self.controls.label(Action::Interact)),
            self.menu_focus.hovered == Some(1),
            true,
        );
        let mut hint = format!(
            "{} / {} : choisir",
            self.controls.label(Action::MenuUp),
            self.controls.label(Action::MenuDown)
        );
        if self
            .game
            .npc_interaction(provider)
            .is_some_and(|npc| !npc.services.is_empty())
        {
            hint.push_str(&format!(
                " · {} / {} : conversation / soins",
                self.controls.label(Action::MenuLeft),
                self.controls.label(Action::MenuRight)
            ));
        }
        draw_text(&hint, x, layout.service.y - 12.0, 12.0, theme.muted());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> AsciiApp {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.prepare_narrative_diagnostic().unwrap();
        app
    }

    fn choose(
        world: &mut WorldState,
        speaker: EntityId,
        node: &str,
        choice: u16,
    ) -> CommandOutcome {
        world.process_player_command(GameCommand::ChooseDialogue {
            speaker,
            node: node.into(),
            choice,
        })
    }

    #[test]
    fn approved_names_match_dialogue_interaction_and_quest_giver() {
        let mut app = app();
        for (key, name) in [
            ("narrative.orme.name", "Elias"),
            ("narrative.seve.name", "Nora"),
            ("narrative.rivet.name", "Milo"),
            ("surface_cast.sorter.name", "Basile"),
            ("surface_cast.scout.name", "Lina"),
        ] {
            assert_eq!(app.texts.resolve(DISPLAY_LOCALE, key), Some(name));
        }
        let position = GridPos::new(5, 3);
        let giver = app.game.actors().entity_at(position).unwrap();
        let dialogue = app.game.dialogue_view(giver).unwrap();
        assert_eq!(
            app.texts.resolve(DISPLAY_LOCALE, &dialogue.name_key),
            Some("Elias")
        );
        assert_ne!(dialogue.name_key, dialogue.role_key);
        assert_eq!(
            app.interaction_display_name(position).as_deref(),
            Some("Elias")
        );
        assert_eq!(
            app.context_choice_label(ContextChoice::Interact(position)),
            "Interagir : Elias (5, 3)"
        );
        assert_eq!(
            choose(&mut app.game, giver, "ABS-D01", 0),
            CommandOutcome::AppliedWithoutTime
        );
        let journal = app.game.quest_journal();
        let entry = journal.iter().find(|entry| entry.giver == giver).unwrap();
        assert_eq!(app.quest_giver_name(entry), "Elias");
        for key in [
            "narrative.abs.summary",
            "narrative.abs.reported",
            "world_effect.orme_direction_board_updated.summary",
        ] {
            let text = app.narrative_text(key, "");
            assert!(text.contains("Elias"), "{key}: {text}");
            assert!(!text.contains("Orme"), "{key}: {text}");
        }
    }

    #[test]
    fn narrative_reward_versions_preserve_old_replays() {
        for (version, expected_xp) in [
            (NARRATIVE_GENERATION_VERSION, 0),
            (NARRATIVE_REWARD_GENERATION_VERSION, 12),
            (STATIONARY_QUEST_CONTACT_GENERATION_VERSION, 12),
            (ORME_DIRECTION_BOARD_GENERATION_VERSION, 12),
            (CURRENT_GENERATION_VERSION, 12),
        ] {
            let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
            let app = AsciiApp::from_seed_version(
                INITIAL_SEED,
                rules.clone(),
                texts.clone(),
                loot.clone(),
                expeditions.clone(),
                version,
            )
            .unwrap();
            let giver = app
                .game
                .actors()
                .iter()
                .find_map(|(id, actor)| {
                    actor
                        .tags()
                        .contains(&"core:orme".parse().unwrap())
                        .then_some(id)
                })
                .unwrap();
            let giver_position = app.game.actors().get(giver).unwrap().position();
            assert_eq!(
                giver_position,
                if version >= STATIONARY_QUEST_CONTACT_GENERATION_VERSION {
                    GridPos::new(11, 27)
                } else {
                    GridPos::new(12, 27)
                }
            );
            if version >= STATIONARY_QUEST_CONTACT_GENERATION_VERSION {
                assert!(!app.game.active_resident(giver));
                let resident = app.game.actors().entity_at(GridPos::new(12, 27)).unwrap();
                assert!(app.game.active_resident(resident));
                assert_ne!(resident, giver);
            }
            assert_eq!(
                app.game
                    .active_facility()
                    .and_then(|facility| facility
                        .installation(&"core:orme_direction_board".parse().unwrap()))
                    .is_some(),
                version >= ORME_DIRECTION_BOARD_GENERATION_VERSION
            );
            let mut saved = app.suspension().unwrap();
            saved.build = "0000000000000000".into();
            saved.recovery = None;
            let mut restored =
                AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
            assert_eq!(
                restored.game.recovery_snapshot_bytes().unwrap(),
                app.game.recovery_snapshot_bytes().unwrap()
            );
            assert_eq!(restored.generation_version, version);
            assert_eq!(restored.narrative_reward_experience(), expected_xp);
            restored.prepare_narrative_reward_diagnostic().unwrap();
            let giver = restored.npc_interaction.unwrap_or(giver);
            assert_eq!(restored.game.player_credits(), 30);
            assert_eq!(restored.game.player_progression().experience(), expected_xp);
            assert!(
                restored
                    .narrative_reward_summary(giver)
                    .unwrap()
                    .contains(&format!("{expected_xp} XP"))
            );
        }
    }

    #[test]
    fn accepted_investigation_points_to_the_authored_entrance_without_discovering_it() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        assert!(app.narrative_navigation_signal_summary().is_none());
        app.prepare_narrative_directions_diagnostic().unwrap();
        let entrance = app
            .expeditions
            .get(&"core:starter_expedition".parse().unwrap())
            .unwrap()
            .hub_passage_for_expanded_world(true);
        assert!(app.game.passage(entrance).is_some());
        assert!(app.terminal.known(entrance).is_none());
        let before = app.game.recovery_snapshot_bytes().unwrap();
        let coordinates = crate::terminal_view::local_coordinates(Some(entrance));
        for key in ["narrative.abs-d02", "narrative.abs.summary"] {
            let text = app.narrative_text(key, "");
            assert!(text.contains(&coordinates));
            assert!(!text.contains('{'));
            assert!(!text.contains("flèches"));
        }
        assert!(
            app.navigation_signal_summary()
                .unwrap()
                .starts_with(&coordinates)
        );
        assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
        assert!(app.terminal.known(entrance).is_none());
    }

    #[test]
    fn underground_passage_leads_to_the_record_and_a_completable_investigation() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(
            INITIAL_SEED,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
        )
        .unwrap();
        app.prepare_narrative_directions_diagnostic().unwrap();
        let board_id: ContentId = "core:orme_direction_board".parse().unwrap();
        let board_position = GridPos::new(10, 28);
        assert_eq!(
            app.game
                .active_facility()
                .unwrap()
                .data_terminal_record_at(board_position)
                .unwrap()
                .as_str(),
            "core:orme_direction_board_old"
        );
        assert_eq!(
            app.terminal.decor.cells.get(&board_position),
            Some(&crate::test_sector::Decor::DirectionBoard)
        );
        let giver = app.npc_interaction.take().unwrap();
        assert_eq!(
            app.execute_command(GameCommand::ChooseDialogue {
                speaker: giver,
                node: "ABS-D02".into(),
                choice: 0,
            }),
            CommandOutcome::AppliedWithoutTime
        );
        let passage = TestSector::EXPANDED_EXPEDITION_PASSAGE;
        app.walk_fixture_to(passage.step(Direction::West)).unwrap();
        assert_eq!(
            app.execute_command(GameCommand::Interact { target: passage }),
            CommandOutcome::Applied
        );
        app.capture_events_at(Some(0.0));
        assert_eq!(
            app.game.current_zone().unwrap().id.as_str(),
            "core:industrial_sector"
        );
        assert_eq!(app.game.current_zone().unwrap().depth, 1);
        let return_passage = app.game.player_position().unwrap();
        assert!(app.game.passage(return_passage).is_some());
        let register = app
            .game
            .active_facility()
            .unwrap()
            .installations()
            .find_map(|(id, installation)| {
                (id.as_str() == "core:relay_register").then_some(installation.position())
            })
            .unwrap();
        let service_door = GridPos::new(register.x, register.y + 2);
        let old_access = GridPos::new(register.x - 3, register.y);
        let service_plan = GridPos::new(register.x - 2, register.y + 1);
        assert_eq!(
            app.game.map().tile(service_door).map(|tile| tile.terrain),
            Some(Terrain::Door(project_rl::world::DoorState::Closed))
        );
        assert!(app.game.map().is_protected(old_access));
        assert_eq!(
            app.game
                .active_facility()
                .unwrap()
                .data_terminal_record_at(service_plan)
                .unwrap()
                .as_str(),
            "core:relay_service_route_verified"
        );
        let rivet = app
            .game
            .actors()
            .iter()
            .find_map(|(id, actor)| {
                actor
                    .tags()
                    .contains(&"core:rivet".parse().unwrap())
                    .then_some(id)
            })
            .unwrap();
        let rivet_position = app.game.actors().get(rivet).unwrap().position();
        app.walk_fixture_to_unchecked(rivet_position.step(Direction::North))
            .unwrap();
        assert_eq!(
            app.game.map().tile(service_door).map(|tile| tile.terrain),
            Some(Terrain::Door(project_rl::world::DoorState::Open))
        );
        assert!(app.game.dialogue_view(rivet).is_some());
        app.walk_fixture_to_unchecked(GridPos::new(register.x - 1, register.y + 1))
            .unwrap();
        assert_eq!(
            app.execute_command(GameCommand::Interact {
                target: service_plan
            }),
            CommandOutcome::Applied
        );
        app.capture_events_at(Some(0.0));
        assert!(
            app.game
                .discovered_data_terminal_records()
                .contains(&"core:relay_service_route_verified".parse().unwrap())
        );
        assert!(app.log.iter().any(|line| line.contains("PLAN LU")));
        // This fixture walks real generated corridors and uses the real terminal.
        let approach = register
            .cardinal_neighbors()
            .into_iter()
            .find(|position| {
                app.game.map().is_walkable(*position)
                    && app.game.actors().entity_at(*position).is_none()
            })
            .unwrap();
        app.walk_fixture_to_unchecked(approach).unwrap();
        assert_eq!(
            app.execute_command(GameCommand::Interact { target: register }),
            CommandOutcome::Applied
        );
        app.capture_events_at(Some(0.0));
        assert_eq!(
            app.game.quest_journal()[0].quest.status,
            QuestStatus::ReadyToComplete
        );
        app.walk_fixture_to_unchecked(return_passage).unwrap();
        let integrity_before_return = app
            .game
            .actors()
            .get(app.game.player_id())
            .map(Actor::integrity);
        assert_eq!(
            app.execute_command(GameCommand::Interact {
                target: return_passage
            }),
            CommandOutcome::Applied
        );
        app.capture_events_at(Some(0.0));
        assert_eq!(
            app.game.current_zone().unwrap().id.as_str(),
            "core:starter_city"
        );
        assert!(
            app.game.player_position().is_some(),
            "player lost on return, integrity before: {integrity_before_return:?}"
        );
        app.walk_fixture_to_unchecked_with_repairs(GridPos::new(11, 26), true)
            .unwrap();
        let credits = app.game.player_credits();
        let experience = app.game.player_progression().experience();
        assert_eq!(
            app.execute_command(GameCommand::ChooseDialogue {
                speaker: giver,
                node: "ABS-D01".into(),
                choice: 4,
            }),
            CommandOutcome::Applied
        );
        assert_eq!(
            app.game.quest_journal()[0].quest.status,
            QuestStatus::Completed
        );
        assert_eq!(app.game.player_credits(), credits + 30);
        assert_eq!(app.game.player_progression().experience(), experience + 12);
        app.capture_events_at(Some(0.0));
        let facility = app.game.active_facility().unwrap();
        assert!(facility.data_terminal_was_updated(&board_id));
        assert_eq!(
            facility
                .data_terminal_record_at(board_position)
                .unwrap()
                .as_str(),
            "core:orme_direction_board_updated"
        );
        assert_eq!(
            app.terminal.decor.cells.get(&board_position),
            Some(&crate::test_sector::Decor::DirectionBoardUpdated)
        );
        app.walk_fixture_to(GridPos::new(10, 27)).unwrap();
        assert_eq!(
            app.execute_command(GameCommand::Interact {
                target: board_position
            }),
            CommandOutcome::Applied
        );
        app.capture_events_at(Some(0.0));
        assert!(
            app.log
                .iter()
                .any(|line| line.contains("PANNEAU LU") && line.contains("Relais occupé"))
        );

        let mut saved = app.suspension().unwrap();
        saved.build = "0000000000000000".into();
        saved.recovery = None;
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(
            restored.game.recovery_snapshot_bytes().unwrap(),
            app.game.recovery_snapshot_bytes().unwrap()
        );
        assert!(
            restored
                .game
                .discovered_data_terminal_records()
                .contains(&"core:relay_service_route_verified".parse().unwrap())
        );
        assert!(
            restored
                .game
                .facility_in_zone(&"core:industrial_sector".parse().unwrap())
                .unwrap()
                .data_terminal_was_accessed(&"core:relay_service_plan".parse().unwrap())
        );
        assert_eq!(
            restored
                .game
                .active_facility()
                .unwrap()
                .data_terminal_record_at(board_position)
                .unwrap()
                .as_str(),
            "core:orme_direction_board_updated"
        );
        assert_eq!(
            restored.terminal.decor.cells.get(&board_position),
            Some(&crate::test_sector::Decor::DirectionBoardUpdated)
        );
    }

    #[test]
    fn relay_coordinates_require_current_sight_and_an_active_investigation() {
        let mut app = app();
        let mut expedition = app
            .expeditions
            .get(&"core:starter_expedition".parse().unwrap())
            .unwrap()
            .clone();
        expedition.destination.zone.id = app.game.current_zone().unwrap().id.clone();
        let mut catalog = project_rl::content::ExpeditionCatalog::default();
        catalog.register(expedition).unwrap();
        app.expeditions = catalog;
        assert!(app.narrative_navigation_signal_summary().is_none());
        let giver = app.npc_interaction.unwrap();
        assert_eq!(
            choose(&mut app.game, giver, "ABS-D01", 0),
            CommandOutcome::AppliedWithoutTime
        );
        assert_eq!(
            app.narrative_navigation_signal_summary().as_deref(),
            Some("X 2 · Y 3 · REGISTRE DU RELAIS EN VUE")
        );
        app.walk_fixture_to_unchecked(GridPos::new(10, 3)).unwrap();
        assert!(!app.game.player_visibility().is_visible(GridPos::new(2, 3)));
        assert!(!app.game.player_visibility().is_visible(GridPos::new(8, 3)));
        let before = app.game.recovery_snapshot_bytes().unwrap();
        assert_eq!(
            app.narrative_navigation_signal_summary().as_deref(),
            Some("RELAIS À LOCALISER DANS LE SECTEUR INDUSTRIEL")
        );
        assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
    }

    #[test]
    fn dialogue_hover_reveals_clickable_choices_without_advancing_time() {
        let mut app = app();
        let layout = NpcInteractionLayout::new(1280.0, 800.0);
        let row = layout.trade_rows[1];
        let turn = app.game.turn();
        app.update_input(&InputFrame {
            pointer: Some((row.x + 20.0, row.y + 20.0)),
            viewport: Some((1280.0, 800.0)),
            ..Default::default()
        });
        assert_eq!(app.menu_focus.hovered, Some(4));
        assert_eq!(app.npc_quest_selection, 1);
        assert_eq!(app.game.turn(), turn);

        app.update_input(&InputFrame {
            pressed: [controls::Binding::key("Up")].into(),
            ..Default::default()
        });
        assert_eq!(app.npc_quest_selection, 0);
        assert!(app.menu_focus.keyboard_mode());
    }

    #[test]
    fn dialogue_keyboard_mouse_and_clinic_service_remain_usable() {
        let mut keyboard = app();
        let mut mouse = app();
        let key = |name: &str| InputFrame {
            pressed: [controls::Binding::key(name)].into(),
            ..Default::default()
        };
        // Only two choices are visible, but the second has authored index 2.
        keyboard.update_npc_interaction(&key("Down"));
        keyboard.update_npc_interaction(&key("Enter"));
        let layout = NpcInteractionLayout::new(1280.0, 800.0);
        let row = layout.trade_rows[1];
        mouse.update_npc_interaction(&InputFrame {
            pressed: [controls::Binding::MouseLeft].into(),
            pointer: Some((row.x + 20.0, row.y + 20.0)),
            viewport: Some((1280.0, 800.0)),
            ..Default::default()
        });
        let giver = keyboard.npc_interaction.unwrap();
        assert_eq!(keyboard.game.dialogue_view(giver).unwrap().node, "ABS-D03");
        assert_eq!(
            keyboard.game.quest_marker(giver),
            Some(QuestMarker::Available)
        );
        assert_eq!(format!("{:?}", keyboard.game), format!("{:?}", mouse.game));

        let definition = keyboard
            .expeditions
            .get(&"core:starter_expedition".parse().unwrap())
            .unwrap()
            .narrative
            .clone()
            .unwrap();
        keyboard.prepare_clinic_diagnostic().unwrap();
        let healer = keyboard.npc_interaction.unwrap();
        keyboard.game.register_narrative(definition).unwrap();
        keyboard
            .game
            .bind_narrative_character(healer, &"core:seve".parse().unwrap())
            .unwrap();
        let before = format!("{:?}", keyboard.game);
        keyboard.update_npc_interaction(&key("Right"));
        assert!(keyboard.npc_dialogue_services);
        assert!(
            !keyboard
                .game
                .npc_interaction(healer)
                .unwrap()
                .services
                .is_empty()
        );
        keyboard.update_npc_interaction(&key("Left"));
        assert!(!keyboard.npc_dialogue_services);
        assert_eq!(format!("{:?}", keyboard.game), before);
    }

    #[test]
    fn early_discovery_dialogue_replay_snapshot_and_single_reward() {
        let mut original = app();
        let mut replay = app();
        let orme = original
            .game
            .actors()
            .entity_at(GridPos::new(5, 3))
            .unwrap();
        let rivet = original
            .game
            .actors()
            .entity_at(GridPos::new(4, 5))
            .unwrap();
        assert_eq!(
            choose(&mut original.game, rivet, "ABS-D04", 0),
            CommandOutcome::Rejected(CommandRejection::InteractionOutOfReach)
        );
        let commands = [
            GameCommand::Move(Direction::South),
            GameCommand::ChooseDialogue {
                speaker: rivet,
                node: "ABS-D04".into(),
                choice: 0,
            },
            GameCommand::ChooseDialogue {
                speaker: rivet,
                node: "ABS-D05".into(),
                choice: 0,
            },
            GameCommand::Move(Direction::North),
            GameCommand::ChooseDialogue {
                speaker: orme,
                node: "ABS-D01".into(),
                choice: 1,
            },
            GameCommand::ChooseDialogue {
                speaker: orme,
                node: "ABS-D17".into(),
                choice: 0,
            },
        ];
        for (index, command) in commands.into_iter().enumerate() {
            let saved = suspension::RecordedCommand::record(&command);
            let json = serde_json::to_string(&saved).unwrap();
            let loaded: suspension::RecordedCommand = serde_json::from_str(&json).unwrap();
            let replayed = loaded.command(&replay.game).unwrap();
            let outcome = original.game.process_player_command(command);
            assert!(
                matches!(
                    outcome,
                    CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime
                ),
                "{index}: {outcome:?}"
            );
            assert_eq!(replay.game.process_player_command(replayed), outcome);
            if index == 4 {
                assert_eq!(
                    original.game.quest_marker(orme),
                    Some(QuestMarker::ReadyToComplete)
                );
            }
            assert_eq!(original.game.drain_events(), replay.game.drain_events());
            let bytes = original.game.recovery_snapshot_bytes().unwrap();
            let restored =
                WorldState::from_recovery_snapshot_bytes(&bytes, original.rules.clone()).unwrap();
            assert_eq!(format!("{:?}", original.game), format!("{restored:?}"));
            assert_eq!(format!("{:?}", original.game), format!("{:?}", replay.game));
        }
        assert_eq!(original.game.player_credits(), 30);
        assert_eq!(original.game.player_progression().experience(), 12);
        assert_eq!(
            original.narrative_reward_summary(orme).as_deref(),
            Some("QUÊTE TERMINÉE · +30 crédits · 12 XP")
        );
        assert_eq!(
            original.game.quest_journal()[0].quest.status,
            QuestStatus::Completed
        );
        let before = format!("{:?}", original.game);
        assert_eq!(
            choose(&mut original.game, orme, "ABS-D17", 0),
            CommandOutcome::Rejected(CommandRejection::DialogueChoiceUnavailable)
        );
        assert_eq!(format!("{:?}", original.game), before);
    }

    #[test]
    fn markers_follow_active_objectives_hide_unseen_records_and_clear_on_discovery() {
        let mut app = app();
        let orme = app.game.actors().entity_at(GridPos::new(5, 3)).unwrap();
        let terminal = GridPos::new(2, 3);
        let hidden = GridPos::new(8, 3);
        assert_eq!(
            app.game.quest_marker_at(GridPos::new(5, 3)),
            Some(QuestMarker::Available)
        );
        assert_eq!(app.game.quest_marker_at(terminal), None);
        assert_eq!(
            choose(&mut app.game, orme, "ABS-D01", 0),
            CommandOutcome::AppliedWithoutTime
        );
        assert_eq!(
            app.game.quest_marker_at(GridPos::new(5, 3)),
            Some(QuestMarker::InProgress)
        );
        assert_eq!(
            app.game.quest_marker_at(terminal),
            Some(QuestMarker::Objective)
        );
        assert_eq!(
            app.game.quest_marker_at(GridPos::new(4, 5)),
            Some(QuestMarker::Objective)
        );
        assert!(!app.game.player_visibility().is_visible(hidden));
        assert_eq!(app.game.quest_marker_at(hidden), None);
        let before = format!("{:?}", app.game);
        for _ in 0..5 {
            app.game.quest_marker_at(terminal);
        }
        assert_eq!(format!("{:?}", app.game), before);
        assert_eq!(
            app.game
                .process_player_command(GameCommand::Move(Direction::West)),
            CommandOutcome::Applied
        );
        assert_eq!(
            app.game
                .process_player_command(GameCommand::Interact { target: terminal }),
            CommandOutcome::Applied
        );
        assert_eq!(app.game.quest_marker_at(terminal), None);
        assert_eq!(app.game.quest_marker_at(GridPos::new(4, 5)), None);
        assert_eq!(
            app.game.quest_marker(orme),
            Some(QuestMarker::ReadyToComplete)
        );
    }

    #[test]
    fn relay_is_guaranteed_and_reachable_in_seeded_expeditions() {
        let (rules, _, loot, expeditions) = ascii_game_content().unwrap();
        let definition = expeditions
            .get(&"core:starter_expedition".parse().unwrap())
            .unwrap();
        for seed in 0..24 {
            let mut destination = crate::test_expedition::generate_destination(
                &rules,
                seed,
                Some(&loot),
                definition,
                crate::test_expedition::ExpeditionGenerationFeatures {
                    defined_population: true,
                    expanded_world: true,
                    pursuit_limits: true,
                    pursuit_lifecycle: true,
                    primary_attributes: true,
                    physical_profiles: true,
                    electronic_systems: true,
                    preparation_disruption: true,
                    player_relations: true,
                },
            )
            .unwrap();
            let facility = crate::test_expedition::install_narrative_relay(
                &mut destination,
                definition.narrative.as_ref().unwrap(),
            )
            .unwrap();
            let terminal = facility.installations[0].position;
            let map = &destination.blueprint.map;
            assert_eq!(
                destination
                    .decor
                    .at(terminal, map.tile(terminal).unwrap().terrain),
                crate::test_sector::Decor::DataTerminalOnline
            );
            assert!(
                terminal.cardinal_neighbors().into_iter().any(|approach| {
                    project_rl::world::find_path(
                        map,
                        destination.blueprint.entrance,
                        approach,
                        map.width() * map.height(),
                        |_| true,
                    )
                    .is_some()
                }),
                "seed {seed}"
            );
            assert_eq!(
                destination
                    .blueprint
                    .actors
                    .iter()
                    .filter(|actor| actor.tags().contains(&"core:rivet".parse().unwrap()))
                    .count(),
                1
            );
        }
    }

    #[test]
    fn relay_service_courtyard_keeps_the_generated_map_connected() {
        let (rules, _, loot, expeditions) = ascii_game_content().unwrap();
        let definition = expeditions
            .get(&"core:starter_expedition".parse().unwrap())
            .unwrap();
        for seed in (0..24).chain(std::iter::once(INITIAL_SEED)) {
            let mut destination = crate::test_expedition::generate_destination(
                &rules,
                seed,
                Some(&loot),
                definition,
                crate::test_expedition::ExpeditionGenerationFeatures {
                    defined_population: true,
                    expanded_world: true,
                    pursuit_limits: true,
                    pursuit_lifecycle: true,
                    primary_attributes: true,
                    physical_profiles: true,
                    electronic_systems: true,
                    preparation_disruption: true,
                    player_relations: true,
                },
            )
            .unwrap();
            let facility = crate::test_expedition::install_narrative_relay_with_service_route(
                &mut destination,
                definition.narrative.as_ref().unwrap(),
            )
            .unwrap_or_else(|error| panic!("seed {seed}: {error}"));
            let center = facility.installations[0].position;
            let service_door = GridPos::new(center.x, center.y + 2);
            let old_access = GridPos::new(center.x - 3, center.y);
            let service_approach = service_door.step(Direction::South);
            let map = &destination.blueprint.map;
            assert_eq!(
                map.tile(service_door).map(|tile| tile.terrain),
                Some(Terrain::Door(project_rl::world::DoorState::Closed)),
                "seed {seed}"
            );
            assert_eq!(
                map.tile(old_access).map(|tile| tile.terrain),
                Some(Terrain::Wall)
            );
            assert!(map.is_protected(old_access));
            assert_eq!(
                destination.decor.at(old_access, Terrain::Wall),
                crate::test_sector::Decor::Barricade
            );
            assert!(
                project_rl::world::find_path(
                    map,
                    destination.blueprint.entrance,
                    service_approach,
                    map.width() * map.height(),
                    |_| true,
                )
                .is_some(),
                "seed {seed}"
            );
            assert_eq!(facility.installations.len(), 3);
        }
    }
}
