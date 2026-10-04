//! First range-sensitive families, using the ordinary campaign weapons.
use super::*;

pub(super) fn weapon_distance_damage_label(
    weapon: &project_rl::weapon::WeaponDefinition,
    damage: DamageImpact,
) -> String {
    weapon
        .distance_damage_percentages()
        .iter()
        .enumerate()
        .map(|(index, percent)| {
            format!(
                "{}c : {}",
                index + 1,
                damage.scaled_percentage(*percent).raw_total()
            )
        })
        .collect::<Vec<_>>()
        .join(" · ")
}

#[cfg(debug_assertions)]
impl AsciiApp {
    pub(super) fn prepare_weapon_family_diagnostic(&mut self, scene: &str) -> Result<(), String> {
        let id: ContentId = if scene.starts_with("spear") {
            "core:lance"
        } else {
            "core:fusil_a_pompe"
        }
        .parse()
        .unwrap();
        self.open_menu(MenuScreen::Main);
        self.enter_test_lab()?;
        let item = self
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &id)
            .ok_or("Arme absente du laboratoire")?
            .instance();
        self.execute_command(GameCommand::EquipWeapon { slot: 0, item });
        self.active_weapon_slot = 0;
        if scene.contains("inventory") {
            self.inventory_filter = InventoryFilter::Weapons;
            self.inventory_selection = self
                .inventory_entries()
                .iter()
                .position(|entry| entry.instance() == item)
                .ok_or("Arme masquée")?;
            self.inventory_open = true;
        } else {
            self.begin_attack_aim(None);
            let mut aim = self.attack_aim.ok_or("Prévisualisation absente")?;
            aim.cursor = GridPos::new(10, 12);
            self.game
                .player_attack_preview(aim.slot, aim.cursor)
                .map_err(|e| format!("{e:?}"))?;
            self.attack_aim = Some(aim);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use project_rl::game::{CommandOutcome, CommandRejection};
    use project_rl::loot::{EquipmentQuality, EquipmentSource};
    use project_rl::world::{Map, Terrain};

    fn game(id: &str, wall: bool) -> GameState {
        let (mut rules, _, _, _) = ascii_game_content().unwrap();
        let id: ContentId = id.parse().unwrap();
        rules.hit_rules = None;
        rules.physical_rules = None;
        rules.player_body_profile = None;
        rules.player_starting_weapons = vec![id.clone()];
        rules.player_weapon_slots.truncate(1);
        rules.player_starting_equipment = vec![Some(id)];
        let mut map = Map::filled(16, 14, Terrain::Floor).unwrap();
        if wall {
            map.set_terrain(GridPos::new(6, 6), Terrain::Wall).unwrap();
        }
        GameState::new_with_rules(map, GridPos::new(5, 6), 128, rules).unwrap()
    }

    #[test]
    fn weapon_families_spear_has_real_reach_contact_penalty_and_no_forced_wait() {
        let mut game = game("core:lance", false);
        let near = game
            .spawn_actor(Actor::new(GridPos::new(5, 5), 100).unwrap())
            .unwrap();
        let far = game
            .spawn_actor(Actor::new(GridPos::new(7, 6), 100).unwrap())
            .unwrap();
        let outside = game
            .spawn_actor(Actor::new(GridPos::new(8, 6), 100).unwrap())
            .unwrap();
        for target in [near, far, far] {
            assert_eq!(
                game.process_player_command(GameCommand::Attack { slot: 0, target }),
                CommandOutcome::Applied
            );
        }
        assert_eq!(game.actors().get(near).unwrap().integrity(), 98);
        assert_eq!(game.actors().get(far).unwrap().integrity(), 92);
        let before = suspension::fingerprint(&game);
        assert_eq!(
            game.process_player_command(GameCommand::Attack {
                slot: 0,
                target: outside
            }),
            CommandOutcome::Rejected(CommandRejection::TargetOutOfRange(outside))
        );
        assert_eq!(before, suspension::fingerprint(&game));
    }

    #[test]
    fn weapon_families_shotgun_falloff_matches_preview_and_spends_ammo_once() {
        let mut game = game("core:fusil_a_pompe", false);
        let mut targets = Vec::new();
        for distance in 1..=4 {
            let target = game
                .spawn_actor(Actor::new(GridPos::new(5 + distance, 6), 100).unwrap())
                .unwrap();
            targets.push(target);
        }
        let aim = GridPos::new(9, 6);
        let before = suspension::fingerprint(&game);
        let preview = game.player_attack_preview(0, aim).unwrap();
        assert_eq!(before, suspension::fingerprint(&game));
        assert!(targets.iter().all(|target| {
            preview
                .cells()
                .iter()
                .any(|cell| cell.position == game.actors().get(*target).unwrap().position())
        }));
        game.drain_events();
        let ammo = game.player_matter().unwrap();
        for shot in 1..=2 {
            assert_eq!(
                game.process_player_command(GameCommand::AttackAt {
                    slot: 0,
                    target: aim
                }),
                CommandOutcome::Applied
            );
            assert_eq!(game.player_matter(), Some(ammo - 2 * shot));
        }
        for (target, damage) in targets.into_iter().zip([8, 6, 4, 2]) {
            assert_eq!(
                game.actors().get(target).unwrap().integrity(),
                100 - 2 * damage
            );
        }
        let footprints: Vec<_> = game
            .events()
            .iter()
            .filter_map(|event| match event {
                GameEvent::AttackPerformed { affected_cells, .. } => Some(affected_cells),
                _ => None,
            })
            .collect();
        assert!(!footprints.is_empty());
        assert!(
            footprints
                .iter()
                .all(|cells| cells.as_slice() == preview.cells())
        );
    }

    #[test]
    fn weapon_families_respect_walls_and_oblique_range() {
        let mut spear = game("core:lance", true);
        let target = spear
            .spawn_actor(Actor::new(GridPos::new(7, 6), 100).unwrap())
            .unwrap();
        let before = suspension::fingerprint(&spear);
        assert_eq!(
            spear.process_player_command(GameCommand::Attack { slot: 0, target }),
            CommandOutcome::Rejected(CommandRejection::NoLineOfSight(target))
        );
        assert_eq!(before, suspension::fingerprint(&spear));
        let shotgun = game("core:fusil_a_pompe", true);
        assert!(
            !shotgun
                .player_attack_footprint(0, GridPos::new(9, 6))
                .unwrap()
                .cells()
                .iter()
                .any(|cell| cell.position == GridPos::new(7, 6))
        );
        let shotgun = game("core:fusil_a_pompe", false);
        for aim in [GridPos::new(9, 10), GridPos::new(9, 8), GridPos::new(1, 2)] {
            let preview = shotgun.player_attack_preview(0, aim).unwrap();
            assert!(
                preview.cells().iter().all(|cell| {
                    cell.position.x.abs_diff(5).max(cell.position.y.abs_diff(6)) <= 4
                })
            );
        }
    }

    #[test]
    fn weapon_families_shotgun_survives_resume_and_empty_ammo_is_non_mutating() {
        let mut game = game("core:fusil_a_pompe", false);
        let aim = GridPos::new(8, 6);
        game.spawn_actor(Actor::new(aim, 1000).unwrap()).unwrap();
        game.drain_events();
        let rules = game.rules().clone();
        let world = WorldState::single(game);
        let bytes = world.recovery_snapshot_bytes().unwrap();
        let mut restored = WorldState::from_recovery_snapshot_bytes(&bytes, rules).unwrap();
        assert_eq!(
            suspension::fingerprint(&world),
            suspension::fingerprint(&restored)
        );
        while restored.player_matter().unwrap() >= 2 {
            assert_eq!(
                restored.process_player_command(GameCommand::AttackAt {
                    slot: 0,
                    target: aim
                }),
                CommandOutcome::Applied
            );
        }
        let before = suspension::fingerprint(&restored);
        assert!(restored.player_attack_preview(0, aim).is_err());
        assert!(matches!(
            restored.process_player_command(GameCommand::AttackAt {
                slot: 0,
                target: aim
            }),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(before, suspension::fingerprint(&restored));
    }

    #[test]
    fn weapon_families_enter_early_loot_not_robot_or_deep_tables() {
        let (_, _, loot, _) = ascii_game_content().unwrap();
        let mut rng = project_rl::game::GameRng::from_seed(128);
        let mut seen = std::collections::BTreeSet::new();
        for depth in [0, 1, 2, 5] {
            for _ in 0..256 {
                let rolled = loot
                    .equipment()
                    .draw(
                        EquipmentSource::HumanoidSite,
                        depth,
                        EquipmentQuality::Enchanted,
                        &mut rng,
                    )
                    .unwrap()
                    .unwrap();
                if equipment_generation::TACTICAL_BASE_IDS.contains(&rolled.item.as_str()) {
                    assert!(depth <= 1);
                    assert!(rolled.modifiers.is_some());
                    seen.insert(rolled.item.to_string());
                }
            }
        }
        assert_eq!(seen.len(), 2);
        assert!(
            loot.equipment()
                .draw(EquipmentSource::Robot, 0, EquipmentQuality::White, &mut rng)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn weapon_families_lab_has_white_models_and_correct_inventory_icons() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(128, rules, texts, loot, expeditions).unwrap();
        app.open_menu(MenuScreen::Main);
        app.enter_test_lab().unwrap();
        for id in equipment_generation::TACTICAL_BASE_IDS {
            let entry = app
                .game
                .player_inventory()
                .iter()
                .find(|entry| entry.item().as_str() == id)
                .unwrap();
            assert!(entry.magic_modifiers().is_none());
            assert_eq!(
                app.inventory_glyph(entry),
                if id == "core:lance" {
                    InventoryGlyph::Blade
                } else {
                    InventoryGlyph::RangedWeapon
                }
            );
        }
        let shotgun = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item().as_str() == "core:fusil_a_pompe")
            .unwrap()
            .instance();
        assert_eq!(
            app.execute_command(GameCommand::EquipWeapon {
                slot: 0,
                item: shotgun
            }),
            CommandOutcome::Applied
        );
        app.active_weapon_slot = 0;
        app.begin_attack_aim(None);
        assert!(app.attack_aim.is_some());
        let fire = InputFrame {
            pressed: [controls::Binding::key("F")].into(),
            ..Default::default()
        };
        let turn = app.game.turn();
        let ammo = app.game.player_matter().unwrap();
        app.update_input(&fire);
        assert!(app.attack_aim.is_none());
        assert_eq!(app.game.turn(), turn + 1);
        assert_eq!(app.game.player_matter(), Some(ammo - 2));
        app.update_input(&fire);
        assert!(app.attack_aim.is_some());
        assert_eq!(app.game.turn(), turn + 1);
        app.update_input(&fire);
        assert_eq!(app.game.player_matter(), Some(ammo - 4));
    }
}
