use super::*;

/// Class loadouts replace the generic inventory. Add the common reserve only
/// to new class-based runs, never when restoring a historical empty loadout.
pub(super) fn ensure_class_weapon_supplies(rules: &mut GameRules, version: u8) {
    if version < CLASS_WEAPON_SUPPLIES_GENERATION_VERSION {
        return;
    }
    if let Some(item) = rules.weapon_matter_item.clone()
        && !rules
            .player_starting_items
            .iter()
            .any(|stack| stack.item == item)
    {
        rules
            .player_starting_items
            .push(StartingItemStack::new(item, 40));
    }
}

impl AsciiApp {
    pub(super) fn weapon_supply_label(&self, weapon: &ContentId) -> Option<String> {
        use project_rl::weapon::WeaponSupply;
        match self.game.weapon_supply(weapon) {
            Some(WeaponSupply::Matter { amount }) => Some(
                self.game
                    .rules()
                    .weapons
                    .get(weapon)
                    .and_then(|weapon| weapon.power_draw())
                    .map_or_else(
                        || {
                            format!(
                                "{amount} munition{} par tir",
                                if amount > 1 { "s" } else { "" }
                            )
                        },
                        |energy| {
                            format!(
                                "{amount} munition{} + {energy} énergie par tir",
                                if amount > 1 { "s" } else { "" }
                            )
                        },
                    ),
            ),
            Some(WeaponSupply::Energy { amount }) => Some(format!("{amount} énergie par tir")),
            None => self
                .game
                .player_weapon_ammunition(weapon)
                .map(|(left, capacity)| format!("MUN. {left}/{capacity}")),
        }
    }

    #[cfg(debug_assertions)]
    pub(super) fn prepare_shared_supplies_diagnostic(
        &mut self,
        energy: bool,
    ) -> Result<(), String> {
        let mut rules = self.rules.clone();
        let weapon: ContentId = if energy {
            "core:fusil_de_parallaxe"
        } else {
            "core:fusil_de_patrouille"
        }
        .parse()
        .unwrap();
        rules.player_starting_weapons = vec![weapon.clone()];
        rules.player_starting_equipment = vec![Some(weapon)];
        rules.player_starting_energy = 20;
        let map =
            project_rl::world::Map::filled(24, 18, project_rl::world::Terrain::Floor).unwrap();
        let mut game = GameState::new_with_rules(map, GridPos::new(9, 9), 0, rules)
            .map_err(|e| e.to_string())?;
        game.spawn_actor(Actor::new(GridPos::new(13, 9), 100).unwrap())
            .map_err(|e| e.to_string())?;
        self.terminal = TerminalView::new(
            crate::test_sector::SectorDecor::default(),
            game.map(),
            game.player_visibility(),
        );
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.active_weapon_slot = 0;
        self.intro_city_reached = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use project_rl::weapon::WeaponSupply;

    #[test]
    fn class_weapon_supplies_cover_all_classes_without_duplication_or_legacy_refill() {
        let (base, texts, loot, expeditions) = ascii_game_content().unwrap();
        let classes = ascii_character_class_catalog().unwrap();
        for (_, class) in classes.iter() {
            let mut rules = base.clone();
            class
                .apply_to_rules(&mut rules, class.recommended_attributes())
                .unwrap();
            ensure_class_weapon_supplies(&mut rules, 126);
            assert!(
                !rules
                    .player_starting_items
                    .iter()
                    .any(|stack| Some(&stack.item) == rules.weapon_matter_item.as_ref())
            );
            ensure_class_weapon_supplies(&mut rules, CURRENT_GENERATION_VERSION);
            ensure_class_weapon_supplies(&mut rules, CURRENT_GENERATION_VERSION);
            let stacks: Vec<_> = rules
                .player_starting_items
                .iter()
                .filter(|stack| Some(&stack.item) == rules.weapon_matter_item.as_ref())
                .collect();
            assert_eq!(stacks.len(), 1);
            assert_eq!(stacks[0].quantity, 40);
            let game = GameState::new_with_rules(
                project_rl::world::Map::filled(3, 3, Terrain::Floor).unwrap(),
                GridPos::new(1, 1),
                0,
                rules,
            )
            .unwrap();
            assert_eq!(game.player_matter(), Some(40));
        }
        let class_id: CharacterClassId = "core:creuset".parse().unwrap();
        let class = classes.get(&class_id).unwrap();
        let mut rules = base.clone();
        class
            .apply_to_rules(&mut rules, class.recommended_attributes())
            .unwrap();
        let mut old = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules,
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
            126,
        )
        .unwrap();
        old.character_class = Some(class_id);
        let saved = old.suspension().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, base, texts, loot, expeditions).unwrap();
        assert_eq!(restored.game.player_matter(), Some(0));
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn creuset_creation_keeps_ammunition_and_confirms_flamethrower_preview() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(
            INITIAL_SEED,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
        )
        .unwrap();
        app.crash_recovery_enabled = false;
        app.controls = Controls::preset(crate::controls::Layout::Qwerty, KeySemantics::Physical);
        let (selected_class, attributes) = app
            .character_classes
            .iter()
            .enumerate()
            .find(|(_, (id, _))| id.as_str() == "core:creuset")
            .map(|(index, (_, class))| (index, class.recommended_attributes()))
            .unwrap();
        app.rebuild_run_with_character_class(&CharacterCreation {
            seed: INITIAL_SEED,
            stage: CharacterCreationStage::Attributes,
            selected_class,
            selected_attribute: 0,
            attributes,
            replace_suspension: false,
            message: String::new(),
            hovered: None,
        })
        .unwrap();
        assert_eq!(app.game.player_matter(), Some(40));
        assert_eq!(app.active_weapon_slot, 2);
        app.walk_fixture_to(app.home_position(GridPos::new(65, 21)))
            .unwrap();
        app.facing = Direction::East;
        let fire = InputFrame {
            pressed: [controls::Binding::key("F")].into(),
            ..Default::default()
        };
        let turn = app.game.turn();
        app.update_input(&fire);
        let aim = app.attack_aim.expect("flamethrower opens its cone preview");
        assert!(app.aimed_attack_preview(aim).is_ok());
        assert_eq!(app.game.turn(), turn);
        app.update_input(&InputFrame::default());
        app.update_input(&fire);
        assert!(app.attack_aim.is_none(), "{:?}", app.log);
        assert_eq!(app.game.turn(), turn + 1);
        assert_eq!(app.game.player_matter(), Some(37));
        assert!(app.visual_cues.active_count() > 0);
        let saved = app.suspension().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(restored.game.player_matter(), Some(37));
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn supply_gauges_follow_stocks_and_parallax_recovery_is_versioned() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let id: ContentId = "core:fusil_de_parallaxe".parse().unwrap();
        assert_eq!(
            rules_for_generation_version(rules.clone(), 123)
                .weapons
                .get(&id)
                .unwrap()
                .attack()
                .recovery_after_attack()
                .unwrap()
                .get(),
            40
        );
        assert_eq!(
            rules_for_generation_version(rules.clone(), 124)
                .weapons
                .get(&id)
                .unwrap()
                .attack()
                .recovery_after_attack()
                .unwrap()
                .get(),
            1
        );
        for weapon in ["core:fusil_de_parallaxe", "core:fusil_de_l_horizon_fendu"] {
            assert_eq!(
                rules
                    .weapons
                    .get(&weapon.parse().unwrap())
                    .unwrap()
                    .attack()
                    .recovery_after_attack(),
                None
            );
        }
        let mut app = AsciiApp::from_seed_version(0, rules, texts, loot, expeditions, 125).unwrap();
        app.prepare_shared_supplies_diagnostic(false).unwrap();
        let gauge = app.resource_gauges();
        let gauge = gauge.iter().find(|g| g.label == "Munitions").unwrap();
        assert_eq!(gauge.value, "40");
        assert_eq!(gauge.ratio, 1.0);
        let target = app.game.actors().entity_at(GridPos::new(13, 9)).unwrap();
        assert_eq!(
            app.execute_command(GameCommand::Attack { slot: 0, target }),
            CommandOutcome::Applied
        );
        assert_eq!(
            app.resource_gauges()
                .iter()
                .find(|g| g.label == "Munitions")
                .unwrap()
                .value,
            "39"
        );
        app.prepare_shared_supplies_diagnostic(true).unwrap();
        let gauge = app.resource_gauges();
        let gauge = gauge.iter().find(|g| g.label == "Énergie").unwrap();
        assert_eq!(gauge.value, "20 / 100");
        assert_eq!(gauge.ratio, 0.2);
        assert_eq!(app.active_weapon_hud_cost(), "6 énergie par tir");
        let target = app.game.actors().entity_at(GridPos::new(13, 9)).unwrap();
        assert_eq!(
            app.execute_command(GameCommand::Attack { slot: 0, target }),
            CommandOutcome::Applied
        );
        assert_eq!(
            app.resource_gauges()
                .iter()
                .find(|g| g.label == "Énergie")
                .unwrap()
                .value,
            "15 / 100"
        );
        assert!(!app.active_weapon_hud_cost().contains("Récupération"));
        for energy in [10, 5] {
            assert_eq!(
                app.execute_command(GameCommand::Attack { slot: 0, target }),
                CommandOutcome::Applied
            );
            assert_eq!(
                app.resource_gauges()
                    .iter()
                    .find(|g| g.label == "Énergie")
                    .unwrap()
                    .value,
                format!("{energy} / 100")
            );
        }
        let before = suspension::fingerprint(&app.game);
        assert_eq!(
            app.execute_command(GameCommand::Attack { slot: 0, target }),
            CommandOutcome::Rejected(CommandRejection::InsufficientEnergy {
                required: 6,
                available: 5
            })
        );
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn shared_supplies_current_content_and_legacy_generation_stay_distinct() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let matter: ContentId = "core:weapon_matter".parse().unwrap();
        let rifle: ContentId = "core:fusil_de_patrouille".parse().unwrap();
        let old = rules_for_generation_version(rules.clone(), 122);
        assert_eq!(old.weapon_matter_item, None);
        assert_eq!(old.player_energy_regeneration, 0);
        assert_eq!(old.player_inventory_capacity, 12);
        assert_eq!(
            rules_for_generation_version(old.clone(), 122).player_inventory_capacity,
            12
        );
        assert!(old.items.get(&matter).is_none());
        assert!(
            !old.player_starting_items
                .iter()
                .any(|stack| stack.item == matter)
        );
        assert_eq!(old.weapons.get(&rifle).unwrap().supply(), None);
        assert_eq!(
            old.weapons.get(&rifle).unwrap().ammunition_capacity(),
            Some(12)
        );
        assert_eq!(
            rules.weapons.get(&rifle).unwrap().supply(),
            Some(WeaponSupply::Matter { amount: 1 })
        );
        let mut app = AsciiApp::from_seed_version(0, rules, texts, loot, expeditions, 123).unwrap();
        assert_eq!(app.game.player_matter(), Some(40));
        assert!(app.game.player_inventory().remaining_slots() > 0);
        assert_eq!(app.game.player_weapon_ammunition(&rifle), None);
        let before = (app.game.player_matter(), app.game.player_energy());
        let saved = app.suspension().unwrap();
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        assert_eq!(
            (restored.game.player_matter(), restored.game.player_energy()),
            before
        );
        app.prepare_shared_supplies_diagnostic(false).unwrap();
        assert_eq!(
            app.weapon_supply_label(&rifle).unwrap(),
            "1 munition par tir"
        );
    }
}
