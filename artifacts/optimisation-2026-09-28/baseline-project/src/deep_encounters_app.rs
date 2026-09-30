//! Isolated fixtures use the actual regional generators and authored profiles.
#[cfg(any(test, debug_assertions))]
use super::*;
#[cfg(any(test, debug_assertions))]
use project_rl::world::generation::{
    RegionalPopulationFeatures, generate_regional_fauna, generate_regional_population,
};

#[cfg(any(test, debug_assertions))]
fn population_features() -> RegionalPopulationFeatures {
    RegionalPopulationFeatures {
        pursuit_lifecycle: true,
        primary_attributes: true,
        physical_profiles: true,
        electronic_systems: true,
        player_relations: true,
    }
}

#[cfg(any(test, debug_assertions))]
impl AsciiApp {
    pub(super) fn prepare_deep_encounter_diagnostic(
        &mut self,
        howler: bool,
        recovery: bool,
    ) -> Result<(), String> {
        self.prepare_deep_encounter_at_depth(howler, recovery, None)
    }

    pub(super) fn prepare_layer_balance_diagnostic(&mut self, depth: u16) -> Result<(), String> {
        self.prepare_deep_encounter_at_depth(false, false, Some(depth))
    }

    fn prepare_deep_encounter_at_depth(
        &mut self,
        howler: bool,
        recovery: bool,
        depth: Option<u16>,
    ) -> Result<(), String> {
        use project_rl::content::RegionTerrain;
        let map = project_rl::world::Map::from_ascii("###############\n#.............#\n#.............#\n#.............#\n#.............#\n#.............#\n#.............#\n#.............#\n#.............#\n#.............#\n###############").map_err(|error| error.to_string())?;
        let at = GridPos::new(8, 5);
        let world = self
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        let mut spawn_map = map.clone();
        for y in 1..10 {
            for x in 1..14 {
                if GridPos::new(x, y) != at {
                    spawn_map.set_protected(GridPos::new(x, y), true).unwrap();
                }
            }
        }
        let mut actor = (0..64)
            .find_map(|seed| {
                if howler {
                    let biome = world.biome(&"core:research".parse().unwrap()).unwrap();
                    generate_regional_fauna(
                        &spawn_map,
                        &[(at, RegionTerrain::Gravel)].into(),
                        &[],
                        &Default::default(),
                        biome.fauna().unwrap(),
                        seed,
                    )
                    .ok()?
                    .into_iter()
                    .find(|a| a.tags().iter().any(|t| t.as_str() == "core:fault_howler"))
                } else {
                    let biome = world.biome(&"core:security".parse().unwrap()).unwrap();
                    generate_regional_population(
                        &spawn_map,
                        &[],
                        biome.population(),
                        seed,
                        population_features(),
                    )
                    .ok()?
                    .pop()
                }
            })
            .ok_or("Rencontre absente du catalogue")?;
        if let Some(depth) = depth {
            project_rl::world::generation::balance_layer_encounters(
                std::slice::from_mut(&mut actor),
                world
                    .encounter_balance()
                    .ok_or("Équilibrage par couche absent")?,
                depth,
                if howler { 2 } else { 3 },
                INITIAL_SEED,
            );
        }
        let mut game =
            GameState::new_with_rules(map, GridPos::new(6, 5), INITIAL_SEED, self.rules.clone())
                .map_err(|error| error.to_string())?;
        let target = game.spawn_actor(actor).map_err(|error| error.to_string())?;
        game.process_player_command(GameCommand::Wait);
        if recovery {
            for _ in 0..if howler { 3 } else { 2 } {
                game.process_player_command(GameCommand::Wait);
            }
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
    fn deep_encounter_fixtures_show_real_names_charge_and_snapshot_continuation() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app =
            AsciiApp::from_seed(INITIAL_SEED, rules.clone(), texts, loot, expeditions).unwrap();
        for howler in [false, true] {
            for recovery in [false, true] {
                app.prepare_deep_encounter_diagnostic(howler, recovery)
                    .unwrap();
                let target = app.selected_target.unwrap();
                let summary = app.terminal_target_summary().unwrap();
                assert!(summary.name.starts_with(if howler {
                    "Hurleur des failles"
                } else {
                    "Sentinelle · machine"
                }));
                assert_eq!(app.hostile_glyph(target), if howler { 'Q' } else { 'P' });
                let actor = app.game.actors().get(target).unwrap();
                assert_eq!(actor.electronic_system().is_some(), !howler);
                assert_eq!(actor.body_components().count(), if howler { 0 } else { 3 });
                assert!(actor.equipped_weapon().is_none());
                assert_eq!(
                    crate::terminal_view::visible_telegraphed_attack_cells(&app.game, target)
                        .is_empty(),
                    recovery
                );
                assert!(app.log.iter().any(|line| line.contains(if howler {
                    "Hurleur des failles"
                } else {
                    "Sentinelle charge"
                })));
                let bytes = app.game.recovery_snapshot_bytes().unwrap();
                let mut restored =
                    WorldState::from_recovery_snapshot_bytes(&bytes, rules.clone()).unwrap();
                for _ in 0..5 {
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
    }

    #[test]
    fn deep_encounters_are_repeatable_rare_and_respect_habitats_and_safe_arrivals() {
        use project_rl::content::RegionTerrain;
        use project_rl::world::generation::RegionalMapGenerator;
        let catalog = ascii_regional_world_catalog().unwrap();
        let world = catalog
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        let mut howlers = 0;
        let mut anemones = 0;
        let mut machines = 0;
        for id in ["core:research", "core:security"] {
            let biome = world.biome(&id.parse().unwrap()).unwrap();
            for seed in 0..64 {
                let region = RegionalMapGenerator::new(world.local_map_size(), biome.terrain())
                    .generate(seed)
                    .unwrap();
                let passages: Vec<_> = [
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ]
                .into_iter()
                .map(|dir| region.passage(dir))
                .collect();
                let generate = || {
                    if let Some(profile) = biome.fauna() {
                        generate_regional_fauna(
                            region.map(),
                            region.terrain(),
                            &passages,
                            &Default::default(),
                            profile,
                            seed,
                        )
                        .unwrap()
                    } else {
                        generate_regional_population(
                            region.map(),
                            &passages,
                            biome.population(),
                            seed,
                            population_features(),
                        )
                        .unwrap()
                    }
                };
                let actors = generate();
                assert_eq!(actors, generate());
                assert!(actors.len() <= 1);
                for actor in actors {
                    let machine = id == "core:security";
                    assert!(!region.map().is_protected(actor.position()));
                    assert!(passages.iter().all(|at| at.x.abs_diff(actor.position().x)
                        + at.y.abs_diff(actor.position().y)
                        >= if machine { 24 } else { 20 }));
                    assert!(actor.equipped_weapon().is_none());
                    assert_eq!(actor.electronic_system().is_some(), machine);
                    if machine {
                        machines += 1;
                        assert_eq!(
                            actor.ai().unwrap().behavior,
                            project_rl::ai::AiBehavior::TelegraphedProjector
                        );
                        assert_eq!(actor.body_components().count(), 3);
                        assert!(
                            !actor
                                .tags()
                                .iter()
                                .any(|tag| tag.as_str().contains("humanoid"))
                        );
                    } else if actor.ai().unwrap().behavior
                        == project_rl::ai::AiBehavior::TelegraphedResonator
                    {
                        howlers += 1;
                        assert!(
                            [RegionTerrain::Gravel, RegionTerrain::RuinFloor]
                                .contains(&region.terrain()[&actor.position()])
                        );
                    } else if actor.ai().unwrap().behavior
                        == project_rl::ai::AiBehavior::DirectionalGuard
                    {
                        assert!(
                            [RegionTerrain::Gravel, RegionTerrain::RuinFloor]
                                .contains(&region.terrain()[&actor.position()])
                        );
                    } else {
                        anemones += 1;
                        assert_eq!(
                            region.terrain()[&actor.position()],
                            RegionTerrain::ShallowWater
                        );
                    }
                }
            }
        }
        assert!(
            howlers > 0 && howlers < anemones,
            "howlers={howlers}, anemones={anemones}"
        );
        assert!((16..48).contains(&machines), "machines={machines}");
    }

    #[test]
    fn deep_encounter_compatibility_preserves_unrelated_profiles_and_anemones() {
        let catalog = ascii_regional_world_catalog()
            .unwrap()
            .without_bestiary_batch_metadata();
        let previous = catalog.without_deep_encounters_metadata();
        let id = "core:simulation_overworld".parse().unwrap();
        let world = catalog.get(&id).unwrap();
        let old = previous.get(&id).unwrap();
        for biome in [
            "core:human_habitat",
            "core:surface_wilds",
            "core:maintenance",
            "core:production",
            "core:network",
            "core:corrupted",
        ] {
            assert_eq!(
                world.biome(&biome.parse().unwrap()),
                old.biome(&biome.parse().unwrap())
            );
        }
        let fauna = old
            .biome(&"core:research".parse().unwrap())
            .unwrap()
            .fauna()
            .unwrap();
        assert_eq!(fauna.level_range, [8, 12]);
        assert_eq!(fauna.danger_budget, 8);
        assert_eq!(fauna.families.len(), 1);
        assert_eq!(
            fauna.families[0].species[0].id.as_str(),
            "core:cave_anemone"
        );
        assert!(
            old.biome(&"core:security".parse().unwrap())
                .unwrap()
                .population()
                .rules()
                .is_empty()
        );
        // Removing anemones alone must leave the howler, not erase all research fauna.
        let no_anemones = catalog.without_cave_anemone_metadata();
        let fauna = no_anemones
            .get(&id)
            .unwrap()
            .biome(&"core:research".parse().unwrap())
            .unwrap()
            .fauna()
            .unwrap();
        assert_eq!(fauna.families.len(), 1);
        assert_eq!(
            fauna.families[0].species[0].id.as_str(),
            "core:fault_howler"
        );
    }
}
