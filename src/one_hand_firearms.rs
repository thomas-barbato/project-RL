//! Pistols and submachine guns use the ordinary equipment/attack contracts.
use super::*;

const MODEL_IDS: [&str; 12] = [
    "core:modele_cogneur_p01",
    "core:modele_cogneur_p02",
    "core:modele_cogneur_p03",
    "core:modele_cogneur_p04",
    "core:modele_cogneur_p05",
    "core:modele_cogneur_p06",
    "core:modele_crache_fer_pm01",
    "core:modele_crache_fer_pm02",
    "core:modele_crache_fer_pm03",
    "core:modele_crache_fer_pm04",
    "core:modele_crache_fer_pm05",
    "core:modele_crache_fer_pm06",
];

pub(super) fn model_ids() -> Vec<ItemId> {
    MODEL_IDS.iter().map(|id| id.parse().unwrap()).collect()
}

#[cfg(debug_assertions)]
impl AsciiApp {
    pub(super) fn prepare_one_hand_firearms_diagnostic(
        &mut self,
        scene: &str,
    ) -> Result<(), String> {
        self.prepare_cyberpunk_diagnostic("terminal-hud-live")?;
        let position = self.game.player_position().ok_or("Position absente")?;
        for id in [MODEL_IDS[0], MODEL_IDS[6]] {
            self.game
                .spawn_ground_item(position, id.parse().unwrap(), 1)
                .map_err(|e| e.to_string())?;
            if self.execute_command(GameCommand::PickUp) != CommandOutcome::Applied {
                return Err("Ramassage de diagnostic refusé".into());
            }
        }
        let knife = self
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item().as_str() == "core:integrity_blade")
            .unwrap()
            .instance();
        for (slot, id) in [(0, MODEL_IDS[0]), (1, MODEL_IDS[6])] {
            let item = self
                .game
                .player_inventory()
                .iter()
                .find(|entry| entry.item().as_str() == id)
                .unwrap()
                .instance();
            if !matches!(
                self.execute_command(GameCommand::EquipWeapon { slot, item }),
                CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime
            ) {
                return Err("Équipement de diagnostic refusé".into());
            }
        }
        if scene.contains("mixed") {
            let slot = if scene.contains("inverse") { 0 } else { 1 };
            if !matches!(
                self.execute_command(GameCommand::EquipWeapon { slot, item: knife }),
                CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime
            ) {
                return Err("Équipement mixte de diagnostic refusé".into());
            }
        }
        self.select_weapon_slot(if scene.ends_with("two-hands") {
            2
        } else if scene.contains("inverse") {
            1
        } else {
            0
        });
        self.selected_target = None;
        self.selected_world_cell = None;
        self.inventory_open = scene.contains("inventory");
        self.inventory_filter = InventoryFilter::Weapons;
        if self.inventory_open {
            let slot = u8::from(scene.ends_with("pm"));
            self.ux.inventory_tooltip = self.game.equipped_player_weapon_item(slot);
            self.ux.inventory_keyboard_preview = true;
            self.inventory_selection = self
                .inventory_entries()
                .iter()
                .position(|entry| Some(entry.instance()) == self.ux.inventory_tooltip)
                .unwrap_or(0);
        }
        self.capture_events_at(Some(0.0));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use project_rl::game::{GameEvent, WeaponHands};
    use project_rl::loot::EquipmentQuality;
    use std::collections::{BTreeMap, BTreeSet};

    #[test]
    fn one_hand_firearms_have_six_tiers_art_names_and_one_hand_slots() {
        let (raw, texts, loot, _) = ascii_game_content().unwrap();
        let rules = rules_for_generation_version(raw, CURRENT_GENERATION_VERSION);
        let visuals = ascii_visual_cue_catalog().unwrap();
        let mut families = BTreeMap::<ContentId, BTreeSet<u8>>::new();
        for id in model_ids() {
            let weapon = rules.weapons.get(&id).unwrap();
            let base = loot
                .equipment()
                .iter()
                .find(|(key, _)| **key == id)
                .unwrap()
                .1;
            families
                .entry(base.family.clone())
                .or_default()
                .insert(base.tier);
            assert!(texts.resolve(DISPLAY_LOCALE, weapon.name_key()).is_some());
            assert!(
                texts
                    .resolve(DISPLAY_LOCALE, weapon.description_key())
                    .is_some()
            );
            assert!(visuals.get(&id).is_some());
            assert_eq!(rules.weapon_hands(weapon), WeaponHands::One);
            assert!(rules.weapon_fits_slot(weapon, 0) && rules.weapon_fits_slot(weapon, 1));
            assert!(rules.weapon_fits_slot(weapon, 2) && rules.weapon_fits_slot(weapon, 3));
            let index = crate::game_art::held_weapon(id.as_str()).unwrap();
            assert_eq!(
                crate::game_art::weapon_hold(index),
                crate::game_art::WeaponHold::Raised
            );
            assert!(crate::game_art::item(id.as_str()).is_some());
        }
        assert_eq!(families.len(), 2);
        assert!(families.values().all(|tiers| *tiers == (1..=6).collect()));
    }

    #[test]
    fn one_hand_firearms_are_in_monster_rewards_white_stock_and_enchanted_gambles() {
        let (rules, _, raw, expeditions) = ascii_game_content().unwrap();
        let loot = loot_for_generation_version(&raw, &rules.items, CURRENT_GENERATION_VERSION);
        let catalog = loot.equipment();
        let ids = model_ids();
        let mut found = BTreeSet::new();
        let mut qualities = BTreeMap::<ContentId, BTreeSet<bool>>::new();
        let mut rng = project_rl::game::GameRng::from_seed(146);
        for _ in 0..4000 {
            let roll = catalog.draw_monster_drop(None, &mut rng).unwrap();
            if ids.contains(&roll.item) {
                found.insert(roll.item.clone());
                qualities
                    .entry(catalog.family(&roll.item).unwrap().clone())
                    .or_default()
                    .insert(roll.modifiers.is_some());
            }
        }
        assert_eq!(found, ids.into_iter().collect());
        assert!(
            qualities
                .values()
                .all(|set| *set == [false, true].into_iter().collect())
        );
        let merchant = expeditions
            .get(&"core:starter_expedition".parse().unwrap())
            .unwrap()
            .hub_merchant
            .clone()
            .unwrap();
        for depth in 0..6 {
            let (shop, quotes) =
                equipment_commerce::shop_definition(merchant.clone(), catalog, depth).unwrap();
            for index in [depth as usize, depth as usize + 6] {
                let id: ItemId = MODEL_IDS[index].parse().unwrap();
                assert!(quotes.iter().any(|(item, _, _)| item == &id));
                let offer = shop.offers.iter().find(|offer| offer.item == id).unwrap();
                let gamble = shop
                    .gambles
                    .iter()
                    .find(|gamble| gamble.item == id)
                    .unwrap();
                assert_eq!(gamble.price, offer.buy_price * 2);
                assert!(
                    catalog
                        .enchant_base(&id, 1, &mut rng)
                        .unwrap()
                        .bonus_count()
                        > 0
                );
                let white = catalog
                    .draw(
                        project_rl::loot::EquipmentSource::HumanoidSite,
                        depth,
                        EquipmentQuality::White,
                        &mut rng,
                    )
                    .unwrap()
                    .unwrap();
                assert!(white.modifiers.is_none());
            }
        }
    }

    fn shooting_game(id: ItemId, blocked: bool, ammunition: u16) -> (GameState, EntityId) {
        let (rules, _, _, _) = ascii_game_content().unwrap();
        let mut rules = rules_for_generation_version(rules, CURRENT_GENERATION_VERSION);
        rules.hit_rules = None;
        rules.physical_rules = None;
        rules.player_body_profile = None;
        let other: ItemId = if id.as_str().contains("cogneur") {
            MODEL_IDS[6]
        } else {
            MODEL_IDS[0]
        }
        .parse()
        .unwrap();
        let rifle: ItemId = "core:modele_brenek_a80".parse().unwrap();
        rules.player_starting_weapons = vec![id.clone(), other.clone(), rifle.clone()];
        rules.player_starting_equipment = vec![Some(id), Some(other), Some(rifle)];
        rules.player_starting_items = if ammunition > 0 {
            vec![StartingItemStack::new(
                "core:weapon_matter".parse().unwrap(),
                ammunition,
            )]
        } else {
            vec![]
        };
        let mut map = project_rl::world::Map::filled(16, 12, Terrain::Floor).unwrap();
        if blocked {
            map.set_terrain(GridPos::new(3, 2), Terrain::Wall).unwrap();
        }
        let mut game = GameState::new_with_rules(map, GridPos::new(2, 2), 146, rules).unwrap();
        let target = game
            .spawn_actor(Actor::new(GridPos::new(4, 2), 1000).unwrap())
            .unwrap();
        game.drain_events();
        (game, target)
    }

    #[test]
    fn one_hand_firearms_fire_only_the_selected_weapon_and_pay_once_per_trigger() {
        for id in model_ids() {
            let (mut game, target) = shooting_game(id.clone(), false, 100);
            let weapon = game.rules().weapons.get(&id).unwrap();
            let shots = if weapon.burst().is_some() { 3 } else { 1 };
            let expected = weapon.attack().damage().raw_total() * shots;
            let before = game.actors().get(target).unwrap().integrity();
            assert_eq!(
                game.process_player_command(GameCommand::Attack { slot: 0, target }),
                CommandOutcome::Applied
            );
            assert_eq!(
                before - game.actors().get(target).unwrap().integrity(),
                expected,
                "{id}"
            );
            assert_eq!(game.player_matter(), Some(100 - u32::from(shots)));
            let spent: Vec<_> = game
                .events()
                .iter()
                .filter(|event| matches!(event, GameEvent::MatterSpent { .. }))
                .collect();
            assert_eq!(spent.len(), 1);
            assert!(
                game.events()
                    .iter()
                    .filter_map(
                        |event| if let GameEvent::AttackPerformed { weapon, .. } = event {
                            Some(weapon)
                        } else {
                            None
                        }
                    )
                    .all(|weapon| weapon.as_ref() == Some(&id))
            );
            assert!(
                game.equipped_player_weapon(1).is_some()
                    && game.equipped_player_weapon(2).is_some()
            );
        }
    }

    #[test]
    fn one_hand_firearms_refuse_walls_and_missing_ammunition_without_mutation() {
        for id in [MODEL_IDS[0], MODEL_IDS[6]] {
            for (wall, ammo) in [(true, 100), (false, 0)] {
                let (mut game, target) = shooting_game(id.parse().unwrap(), wall, ammo);
                let before = format!("{game:?}");
                assert!(matches!(
                    game.process_player_command(GameCommand::Attack { slot: 0, target }),
                    CommandOutcome::Rejected(_)
                ));
                assert_eq!(format!("{game:?}"), before);
            }
        }
    }

    #[test]
    fn one_hand_firearms_real_equipment_pairs_and_two_hand_switch_survive_resume() {
        let (mut rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        // Leave space for the two finds without removing the normal loadout.
        rules.player_inventory_capacity = 24;
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        let both_hands = app.game.equipped_player_weapon_item(2);
        for (slot, id) in [(0, MODEL_IDS[0]), (1, MODEL_IDS[6])] {
            let position = app.game.player_position().unwrap();
            app.game
                .spawn_ground_item(position, id.parse().unwrap(), 1)
                .unwrap();
            assert_eq!(
                app.execute_command(GameCommand::PickUp),
                CommandOutcome::Applied
            );
            let item = app
                .game
                .player_inventory()
                .iter()
                .find(|entry| entry.item().as_str() == id)
                .unwrap()
                .instance();
            assert!(matches!(
                app.execute_command(GameCommand::EquipWeapon { slot, item }),
                CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime
            ));
        }
        app.capture_events_at(Some(0.0));
        assert_eq!(app.game.equipped_player_weapon_item(2), both_hands);
        app.select_weapon_slot(0);
        let crate::game_art::Visual::Player { hands, .. } = app.front_player_visual() else {
            panic!("player visual missing");
        };
        assert_eq!(
            hands,
            Some([
                crate::game_art::held_weapon(MODEL_IDS[0]),
                crate::game_art::held_weapon(MODEL_IDS[6]),
            ])
        );
        for slot in [0, 2] {
            let before = app.game.turn();
            let owned = app.game.player_inventory().clone();
            app.select_weapon_slot(slot);
            assert_eq!(app.game.turn(), before);
            assert_eq!(app.game.player_inventory(), &owned);
            let engine = app.game.recovery_snapshot_bytes().unwrap();
            let saved = app.suspension().unwrap();
            let restored = AsciiApp::restore_suspension(
                &saved,
                app.rules.clone(),
                app.texts.clone(),
                app.loot.clone(),
                app.expeditions.clone(),
            )
            .unwrap();
            assert_eq!(restored.front_player_visual(), app.front_player_visual());
            assert_eq!(restored.game.recovery_snapshot_bytes().unwrap(), engine);
        }
    }

    #[test]
    fn one_hand_firearms_historical_saves_keep_the_pre_change_fingerprints() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/one-hand-firearms");
        for version in [143, 144, 145] {
            let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
            let baseline = Suspension::read(&dir.join(format!("partie-{version}.json"))).unwrap();
            let app = AsciiApp::from_seed_version(
                INITIAL_SEED,
                rules.clone(),
                texts.clone(),
                loot.clone(),
                expeditions.clone(),
                version,
            )
            .unwrap();
            // These checked-in fixtures were written during the interim Map
            // Debug format. Preserve their exact stored hashes as well.
            let saved =
                project_rl::world::with_explicit_empty_obstructions(|| app.suspension().unwrap());
            assert_eq!(
                (saved.rules, saved.loot_rules, saved.state),
                (baseline.rules, baseline.loot_rules, baseline.state),
                "version {version}"
            );
            let restored =
                AsciiApp::restore_suspension(&baseline, rules, texts, loot, expeditions).unwrap();
            assert_eq!(
                project_rl::world::with_explicit_empty_obstructions(|| {
                    suspension::fingerprint(&restored.game)
                }),
                baseline.state
            );
            for id in model_ids() {
                assert!(app.game.rules().weapons.get(&id).is_none());
                assert!(app.loot.equipment().family(&id).is_none());
            }
        }
    }
}
