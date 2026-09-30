#[cfg(any(test, debug_assertions))]
use super::*;

#[cfg(any(test, debug_assertions))]
fn arena() -> project_rl::world::Map {
    project_rl::world::Map::from_ascii("#################\n#...............#\n#...............#\n#...............#\n#...............#\n#...............#\n#...............#\n#################").unwrap()
}

#[cfg(any(test, debug_assertions))]
fn specimen(catalog: &RegionalWorldCatalog) -> Actor {
    use project_rl::content::RegionTerrain;
    let world = catalog
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap();
    let fauna = world
        .biome(&"core:surface_wilds".parse().unwrap())
        .unwrap()
        .fauna()
        .unwrap();
    let at = GridPos::new(6, 4);
    let mut map = arena();
    for y in 1..7 {
        for x in 1..16 {
            if GridPos::new(x, y) != at {
                map.set_protected(GridPos::new(x, y), true).unwrap();
            }
        }
    }
    (0..128)
        .find_map(|seed| {
            project_rl::world::generation::generate_regional_fauna(
                &map,
                &[(at, RegionTerrain::Mud)].into(),
                &[],
                &Default::default(),
                fauna,
                seed,
            )
            .unwrap()
            .into_iter()
            .find(|a| a.tags().iter().any(|t| t.as_str() == "core:marsh_spitter"))
        })
        .expect("authored spitter can be generated on wet terrain")
}

#[cfg(any(test, debug_assertions))]
impl AsciiApp {
    pub(super) fn prepare_marsh_spitter_diagnostic(
        &mut self,
        recovery: bool,
    ) -> Result<(), String> {
        let mut actor = specimen(&self.regional_worlds);
        let world = self
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        project_rl::world::generation::balance_layer_encounters(
            std::slice::from_mut(&mut actor),
            world.encounter_balance().unwrap(),
            0,
            0,
            INITIAL_SEED,
        );
        let mut game = GameState::new_with_rules(
            arena(),
            GridPos::new(3, 4),
            INITIAL_SEED,
            self.rules.clone(),
        )
        .map_err(|e| e.to_string())?;
        let target = game.spawn_actor(actor).map_err(|e| e.to_string())?;
        game.process_player_command(GameCommand::Wait);
        if recovery {
            game.process_player_command(GameCommand::Wait);
        }
        self.terminal =
            TerminalView::new(SectorDecor::default(), game.map(), game.player_visibility());
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.selected_target = Some(target);
        self.intro_city_reached = true;
        self.log.clear();
        self.capture_events_at(Some(0.0));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spitter_generation_uses_wet_habitats_and_safe_passages_without_extra_groups() {
        use project_rl::content::{RegionCoord, RegionTerrain};
        use project_rl::world::generation::{RegionalMapGenerator, generate_regional_fauna};
        let catalog = ascii_regional_world_catalog().unwrap();
        let world = catalog
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        let biome = world.biome(&"core:surface_wilds".parse().unwrap()).unwrap();
        let mut observed = 0;
        for seed in 0..24 {
            let generated = RegionalMapGenerator::new(
                world.map_size_at(RegionCoord::new(3, -2, 0)),
                biome.terrain(),
            )
            .generate(seed)
            .unwrap();
            let passages = [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ]
            .map(|d| generated.passage(d));
            let actors = generate_regional_fauna(
                generated.map(),
                generated.terrain(),
                &passages,
                &Default::default(),
                biome.fauna().unwrap(),
                seed,
            )
            .unwrap();
            assert!(actors.len() <= 6);
            for actor in actors
                .iter()
                .filter(|a| a.tags().iter().any(|t| t.as_str() == "core:marsh_spitter"))
            {
                observed += 1;
                assert!(matches!(
                    generated.terrain().get(&actor.position()),
                    Some(RegionTerrain::Mud | RegionTerrain::ShallowWater)
                ));
                assert!(!generated.map().is_protected(actor.position()));
                for passage in passages {
                    assert!(
                        actor.position().x.abs_diff(passage.x)
                            + actor.position().y.abs_diff(passage.y)
                            >= 16
                    );
                }
            }
            if seed == 0 {
                let dry = generated
                    .terrain()
                    .keys()
                    .map(|at| (*at, RegionTerrain::Grass))
                    .collect();
                for draw in 0..32 {
                    let actors = generate_regional_fauna(
                        generated.map(),
                        &dry,
                        &passages,
                        &Default::default(),
                        biome.fauna().unwrap(),
                        draw,
                    )
                    .unwrap();
                    assert!(
                        actors
                            .iter()
                            .all(|a| a.tags().iter().all(|t| t.as_str() != "core:marsh_spitter"))
                    );
                }
            }
        }
        assert!(observed > 0);
        for name in [
            "core:human_habitat",
            "core:maintenance",
            "core:production",
            "core:research",
            "core:security",
            "core:network",
            "core:corrupted",
        ] {
            assert!(
                !format!("{:?}", world.biome(&name.parse().unwrap())).contains("marsh_spitter")
            );
        }
    }

    #[test]
    fn spitter_real_profile_is_fragile_without_equipment_or_electronic_parts() {
        let regions = ascii_regional_world_catalog().unwrap();
        let base = specimen(&regions);
        let world = regions
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        let (rules, _, _, _) = ascii_game_content().unwrap();
        for seed in 0..8 {
            let mut actor = base.clone();
            project_rl::world::generation::balance_layer_encounters(
                std::slice::from_mut(&mut actor),
                world.encounter_balance().unwrap(),
                0,
                0,
                seed,
            );
            assert!(actor.equipped_weapon().is_none());
            assert!(actor.electronic_system().is_none());
            assert_eq!(actor.body_components().count(), 0);
            assert!((1..=3).contains(&actor.defeat_reward().unwrap().threat_level));
            let mut rules = rules.clone();
            rules.hit_rules = None;
            rules.player_starting_weapons = vec!["core:couteau_de_camp".parse().unwrap()];
            rules.player_starting_equipment = vec![Some("core:couteau_de_camp".parse().unwrap())];
            let mut game =
                GameState::new_with_rules(arena(), GridPos::new(5, 4), seed, rules).unwrap();
            let target = game.spawn_actor(actor).unwrap();
            let hp = game.actors().get(target).unwrap().integrity();
            let attack = game.equipped_player_weapon(0).unwrap().attack();
            let damage = project_rl::combat::resolve_damage_impact_with_armor(
                game.resolved_attack_damage(game.player_id(), attack)
                    .unwrap(),
                game.actors().get(target).unwrap().resistances(),
                game.actor_armor_profile(target).unwrap(),
                game.rules().damage,
                game.rules().armor_rules.unwrap(),
            )
            .amount();
            assert!(damage > 0 && hp.div_ceil(damage) <= 3);
            assert_eq!(
                game.process_player_command(GameCommand::Attack { slot: 0, target }),
                project_rl::game::CommandOutcome::Applied
            );
            assert_eq!(
                game.actors().get(target).map_or(0, Actor::integrity),
                hp.saturating_sub(damage)
            );
        }
    }

    #[test]
    fn spitter_names_preview_and_recovery_survive_snapshot() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app =
            AsciiApp::from_seed(INITIAL_SEED, rules.clone(), texts, loot, expeditions).unwrap();
        for recovery in [false, true] {
            app.prepare_marsh_spitter_diagnostic(recovery).unwrap();
            let target = app.selected_target.unwrap();
            assert!(
                app.terminal_target_summary()
                    .unwrap()
                    .name
                    .starts_with("Cracheur des mares")
            );
            assert_eq!(app.hostile_glyph(target), 'O');
            assert_eq!(
                crate::terminal_view::visible_telegraphed_attack_cells(&app.game, target).len(),
                if recovery { 0 } else { 1 }
            );
            assert!(app.log.iter().any(|line| line.contains("prépare son jet")));
            app.game.drain_events();
            let bytes = app.game.recovery_snapshot_bytes().unwrap();
            let mut restored =
                WorldState::from_recovery_snapshot_bytes(&bytes, rules.clone()).unwrap();
            for _ in 0..4 {
                app.game.process_player_command(GameCommand::Wait);
                restored.process_player_command(GameCommand::Wait);
                assert_eq!(app.game.drain_events(), restored.drain_events());
                assert_eq!(
                    app.game.recovery_snapshot_bytes().unwrap(),
                    restored.recovery_snapshot_bytes().unwrap()
                );
            }
        }
    }

    #[test]
    fn spitter_preserves_previous_family_weights_and_v114_replay() {
        let regions = ascii_regional_world_catalog().unwrap();
        let old = regions.without_marsh_spitter_metadata();
        let id = "core:simulation_overworld".parse().unwrap();
        let biome = "core:surface_wilds".parse().unwrap();
        let new_fauna = regions
            .get(&id)
            .unwrap()
            .biome(&biome)
            .unwrap()
            .fauna()
            .unwrap();
        let old_fauna = old
            .get(&id)
            .unwrap()
            .biome(&biome)
            .unwrap()
            .fauna()
            .unwrap();
        assert_eq!(new_fauna.group_rolls, old_fauna.group_rolls);
        assert_eq!(new_fauna.danger_budget, old_fauna.danger_budget);
        assert_eq!(
            new_fauna
                .families
                .iter()
                .map(|f| f.weight)
                .collect::<Vec<_>>(),
            old_fauna
                .families
                .iter()
                .map(|f| f.weight)
                .collect::<Vec<_>>()
        );
        assert!(!format!("{old_fauna:?}").contains("marsh_spitter"));
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        assert_eq!(
            world_fingerprint_for_version(&expeditions, &regions, 114),
            world_fingerprint_for_version(&expeditions, &old, 114)
        );
        for version in [114, 115] {
            let mut app = AsciiApp::from_seed_version(
                INITIAL_SEED,
                rules.clone(),
                texts.clone(),
                loot.clone(),
                expeditions.clone(),
                version,
            )
            .unwrap();
            app.execute_command(GameCommand::Wait);
            app.game.drain_events();
            let bytes = app.game.recovery_snapshot_bytes().unwrap();
            let saved = app.suspension().unwrap();
            for replay in [false, true] {
                let mut saved = saved.clone();
                if replay {
                    saved.build = "0000000000000000".into();
                }
                let restored = AsciiApp::restore_suspension(
                    &saved,
                    rules.clone(),
                    texts.clone(),
                    loot.clone(),
                    expeditions.clone(),
                )
                .unwrap();
                assert_eq!(bytes, restored.game.recovery_snapshot_bytes().unwrap());
            }
        }
    }
}
