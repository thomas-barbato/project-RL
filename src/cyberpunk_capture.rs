//! Isolated client scenes for checking the approved art in the real renderer.
use super::*;

impl AsciiApp {
    pub(super) fn prepare_cyberpunk_diagnostic(&mut self, scene: &str) -> Result<(), String> {
        let mut map =
            project_rl::world::Map::filled(22, 18, Terrain::Floor).map_err(|e| e.to_string())?;
        for y in 0..18 {
            for x in 0..22 {
                if x == 0 || y == 0 || x == 21 || y == 17 || (x == 16 && y < 14 && y != 9) {
                    map.set_terrain(GridPos::new(x, y), Terrain::Wall)
                        .map_err(|e| e.to_string())?;
                }
            }
        }
        let scenery = [
            (GridPos::new(4, 6), crate::test_sector::Decor::Server),
            (GridPos::new(5, 6), crate::test_sector::Decor::Crate),
            (GridPos::new(6, 6), crate::test_sector::Decor::VentUnit),
            (GridPos::new(7, 6), crate::test_sector::Decor::Console),
            (GridPos::new(8, 6), crate::test_sector::Decor::Barrel),
        ];
        for (at, _) in scenery {
            map.set_terrain(at, Terrain::Wall)
                .map_err(|e| e.to_string())?;
        }
        let mut rules = self.game.rules().clone();
        rules.player_inventory_capacity = rules.player_inventory_capacity.max(24);
        let rifle: ItemId = "core:modele_brenek_a80".parse().unwrap();
        rules.player_starting_weapons.push(rifle.clone());
        if rules.uses_weapon_hands() {
            let knife: WeaponId = "core:integrity_blade".parse().unwrap();
            rules.player_starting_weapons.push(knife.clone());
            rules.player_starting_equipment[1] = Some(knife);
        }
        let rifle_slot = if rules.uses_weapon_hands() { 2 } else { 0 };
        rules.player_starting_equipment[rifle_slot] = Some(rifle.clone());
        self.active_weapon_slot = rifle_slot as u8;
        rules.player_starting_items.push(StartingItemStack {
            item: "core:plastron_bastion".parse().unwrap(),
            quantity: 1,
        });
        let combat_review = scene.starts_with("combat-") || scene.starts_with("terminal-");
        if combat_review {
            for id in [
                "core:heaume_bastion",
                "core:gantelets_bastion",
                "core:solerets_bastion",
            ] {
                rules.player_starting_items.push(StartingItemStack {
                    item: id.parse().unwrap(),
                    quantity: 1,
                });
            }
        }
        let mut game = GameState::new_with_rules(map, GridPos::new(10, 9), INITIAL_SEED, rules)
            .map_err(|e| e.to_string())?;
        if combat_review {
            for (slot, id) in [
                ("core:body_armor", "core:plastron_bastion"),
                ("core:head_armor", "core:heaume_bastion"),
                ("core:hand_armor", "core:gantelets_bastion"),
                ("core:foot_armor", "core:solerets_bastion"),
            ] {
                let instance = game
                    .player_inventory()
                    .iter()
                    .find(|entry| entry.item().as_str() == id)
                    .unwrap()
                    .instance();
                if !matches!(
                    game.process_player_command(GameCommand::EquipItem {
                        slot: slot.parse().unwrap(),
                        item: instance
                    }),
                    CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime
                ) {
                    return Err(format!("Diagnostic équipement refusé : {id}"));
                }
            }
        }
        for (index, id) in [
            "core:modele_brenek_a80",
            "core:melee_epee_longue",
            "core:plastron_bastion",
            "core:repair_patch",
            "core:weapon_matter",
            "core:fragmentation_charge",
            "core:charged_battery",
            "core:saturation_beacon",
            "core:sound_decoy",
            "core:camouflage_bundle",
            "core:repair_parts",
            "core:salvaged_component",
            "core:tuning_parts",
            "core:engineering_tool",
            "core:power_regulator",
            "core:unstable_fragment",
        ]
        .into_iter()
        .enumerate()
        {
            if scene == "cyberpunk-scale" {
                break;
            }
            game.spawn_ground_item(
                GridPos::new(5 + index as i32 % 4, 8 + index as i32 / 4),
                id.parse().unwrap(),
                1,
            )
            .map_err(|e| e.to_string())?;
        }
        self.actor_glyphs.clear();
        let mut inspected_actor = None;
        for (at, glyph, tag) in [
            (GridPos::new(11, 6), 'N', Some("core:rubble_nibbler")),
            (GridPos::new(13, 7), 'B', Some("core:friche_biter")),
            (GridPos::new(14, 10), 'K', Some("core:bone_breaker")),
            (GridPos::new(12, 12), 'P', Some("core:sentinel_projector")),
            (GridPos::new(10, 12), 't', None),
            (GridPos::new(11, 11), 'w', None),
        ] {
            let actor = Actor::new(at, 30)
                .map_err(|e| e.to_string())?
                .with_player_relation(if glyph == 'N' || glyph == 'w' {
                    PlayerRelation::Neutral
                } else {
                    PlayerRelation::Hostile
                });
            let actor = if glyph == 't' {
                actor.with_equipped_weapon(rifle.clone(), None, &game.rules().weapons)?
            } else {
                actor
            };
            let actor = if let Some(tag) = tag {
                actor.with_tags([tag.parse().unwrap()])
            } else {
                actor
            };
            let entity = game.spawn_actor(actor).map_err(|e| e.to_string())?;
            self.actor_glyphs.insert(entity, glyph);
            if glyph == 'K' {
                inspected_actor = Some(entity);
            }
        }
        let mut decor = SectorDecor::default();
        for y in 0..18 {
            for x in 0..22 {
                let at = GridPos::new(x, y);
                let kind = if game
                    .map()
                    .tile(at)
                    .is_some_and(|tile| tile.terrain == Terrain::Wall)
                {
                    crate::test_sector::Decor::Wall
                } else if y < 7 {
                    crate::test_sector::Decor::RuinFloor
                } else if x < 10 {
                    crate::test_sector::Decor::Street
                } else {
                    crate::test_sector::Decor::Deck
                };
                decor.cells.insert(at, kind);
            }
        }
        for (at, kind) in scenery {
            decor.cells.insert(at, kind);
        }
        self.terminal = TerminalView::new(decor, game.map(), game.player_visibility());
        game.drain_events();
        self.game = WorldState::single(game);
        self.character_class = Some("core:breche".parse().unwrap());
        self.active_weapon_slot = rifle_slot as u8;
        self.selected_world_cell = Some(GridPos::new(5, 8));
        self.selected_target = None;
        if scene == "cyberpunk-actor" {
            self.selected_target = inspected_actor;
            self.selected_world_cell = Some(GridPos::new(14, 10));
            let technique = technique_id("core:rec_01").ok_or("Diagnostic analysis missing")?;
            if self
                .game
                .process_player_command(GameCommand::LearnTechnique {
                    technique: technique.clone(),
                })
                != CommandOutcome::AppliedWithoutTime
            {
                return Err("Diagnostic analysis learning refused".into());
            }
            if self.game.process_player_command(GameCommand::UseTechnique {
                technique,
                targets: inspected_actor.into_iter().collect(),
                weapon_slot: None,
            }) != CommandOutcome::Applied
            {
                return Err("Diagnostic analysis refused".into());
            }
            self.game.drain_events();
        }
        self.npc_art.clear();
        self.test_lab = false;
        self.intro_city_reached = true;
        self.menu = MenuScreen::Hidden;
        self.character_creation = None;
        self.inventory_filter = InventoryFilter::All;
        self.inventory_open = matches!(scene, "cyberpunk-inventory" | "cyberpunk-armour");
        let selected = if scene == "cyberpunk-armour" {
            "core:plastron_bastion"
        } else {
            "core:modele_brenek_a80"
        };
        self.inventory_selection = self
            .inventory_entries()
            .iter()
            .position(|entry| entry.item().as_str() == selected)
            .ok_or("Diagnostic item missing from inventory")?;
        self.graphics.active.world_cell_px = 32;
        self.log.clear();
        self.floating_messages.clear();
        self.level_up_notice = None;
        self.push_log(if scene == "cyberpunk-scale" {
            "ESSAI VISUEL ISOLÉ · cadrage des personnages et du bestiaire · cases de 32 pixels"
                .into()
        } else {
            "ESSAI VISUEL · tiles natives 32 px · 18 familles d'armes · portraits cohérents".into()
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approved_art_covers_every_loaded_core_item_and_weapon() {
        let (rules, _, _, _) = ascii_game_content().unwrap();
        let ids: std::collections::BTreeSet<_> = rules
            .items
            .iter()
            .map(|(id, _)| id)
            .chain(rules.weapons.iter().map(|(id, _)| id))
            .filter(|id| id.namespace().as_str() == "core")
            .collect();
        assert_eq!(ids.len(), 162);
        for id in ids {
            assert!(crate::game_art::item(id.as_str()).is_some(), "{id}");
        }
        for (id, _) in rules
            .weapons
            .iter()
            .filter(|(id, _)| id.namespace().as_str() == "core")
        {
            assert!(
                crate::game_art::held_weapon(id.as_str()).is_some(),
                "held layer missing: {id}"
            );
        }
    }

    #[test]
    fn changing_the_selected_weapon_slot_keeps_the_same_class_body() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.character_class = Some("core:creuset".parse().unwrap());
        let before = app.game.recovery_snapshot_bytes().unwrap();
        app.active_weapon_slot = 0;
        let crate::game_art::Visual::Player {
            body: melee_body,
            weapon: melee,
            ..
        } = app.front_player_visual()
        else {
            panic!("player visual missing");
        };
        app.active_weapon_slot = 2;
        let crate::game_art::Visual::Player {
            body: ranged_body,
            weapon: ranged,
            ..
        } = app.front_player_visual()
        else {
            panic!("player visual missing");
        };
        assert_eq!(melee_body, 2);
        assert_eq!(ranged_body, melee_body);
        assert!(melee.is_some() && ranged.is_some());
        assert_ne!(melee, ranged);
        assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
    }

    #[test]
    fn ground_preview_uses_the_exact_model_and_cannot_reveal_hidden_loot() {
        let (mut rules, _, _, _) = ascii_game_content().unwrap();
        rules.player_field_of_view.radius = 2;
        let map = project_rl::world::Map::filled(24, 24, Terrain::Floor).unwrap();
        let mut game =
            GameState::new_with_rules(map, GridPos::new(3, 3), INITIAL_SEED, rules).unwrap();
        let visible = GridPos::new(4, 3);
        let hidden = GridPos::new(15, 15);
        game.spawn_ground_item(visible, "core:modele_brenek_a80".parse().unwrap(), 1)
            .unwrap();
        game.spawn_ground_item(hidden, "core:plastron_bastion".parse().unwrap(), 1)
            .unwrap();
        game.drain_events();
        let game = WorldState::single(game);
        let before = game.recovery_snapshot_bytes().unwrap();
        let preview =
            crate::terminal_view::interactive_target_summary(&game, visible, None, "E").unwrap();
        assert_eq!(
            preview.visual,
            crate::game_art::item("core:modele_brenek_a80")
                .map(|art| crate::game_art::Visual::ColoredItem(art, Default::default()))
        );
        assert!(
            crate::terminal_view::interactive_target_summary(&game, hidden, None, "E").is_none()
        );
        assert_eq!(game.recovery_snapshot_bytes().unwrap(), before);
    }
}
