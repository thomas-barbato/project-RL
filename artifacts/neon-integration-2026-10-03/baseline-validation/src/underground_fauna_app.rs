//! Authored underground fauna fixtures and compatibility checks.
use super::*;

impl AsciiApp {
    #[cfg(any(test, debug_assertions))]
    pub(super) fn prepare_armored_worm_diagnostic(
        &mut self,
        recovering: bool,
    ) -> Result<(), String> {
        use project_rl::content::RegionTerrain;
        let map = project_rl::world::Map::from_ascii(
            "#############\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#############"
        ).map_err(|error| error.to_string())?;
        let at = GridPos::new(7, 4);
        let profile = self
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .and_then(|world| world.biome(&"core:maintenance".parse().unwrap()))
            .and_then(|biome| biome.fauna())
            .ok_or("Ver cuirassé absent du catalogue")?;
        let terrain = [(at, RegionTerrain::Mud)].into();
        let mut animals = project_rl::world::generation::generate_regional_fauna(
            &map,
            &terrain,
            &[],
            &Default::default(),
            profile,
            INITIAL_SEED,
        )?;
        let mut game =
            GameState::new_with_rules(map, GridPos::new(5, 4), INITIAL_SEED, self.rules.clone())
                .map_err(|error| error.to_string())?;
        let worm = game
            .spawn_actor(animals.pop().ok_or("Ver cuirassé non généré")?)
            .map_err(|error| error.to_string())?;
        game.process_player_command(GameCommand::Wait);
        if !matches!(
            game.actors().get(worm).unwrap().ai_state(),
            project_rl::ai::AiState::Aiming { .. }
        ) {
            return Err("Le Ver cuirassé ne prépare pas son balayage".into());
        }
        if recovering {
            game.process_player_command(GameCommand::Move(Direction::West));
        }
        let mut decor = SectorDecor::default();
        decor.cells.insert(at, crate::test_sector::Decor::Mud);
        self.terminal = TerminalView::new(decor, game.map(), game.player_visibility());
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.selected_target = Some(worm);
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
    fn underground_fauna_preserves_complete_regions_and_stays_out_of_cities() {
        use crate::test_regional::{RegionalGenerationFeatures, generate, zone_info};
        use project_rl::content::{RegionCoord, RegionDescriptor};
        use project_rl::world::generation::cardinal_passage;
        let regions = ascii_regional_world_catalog()
            .unwrap()
            .without_deep_encounters_metadata();
        let legacy = regions
            .without_underground_fauna_metadata()
            .without_cave_anemone_metadata();
        let id = "core:simulation_overworld".parse().unwrap();
        let world = regions.get(&id).unwrap();
        let previous = legacy.get(&id).unwrap();
        let (_, _, loot, _) = ascii_game_content().unwrap();
        let features = RegionalGenerationFeatures {
            vertical_travel: true,
            population: true,
            encounters: true,
            exploration_variety: true,
            exploration_salvage: true,
            exploration_salvage_breaches: true,
            pursuit_lifecycle: true,
            primary_attributes: true,
            physical_profiles: true,
            loot: true,
            landmarks: true,
            sites: true,
            site_interactions: true,
            site_security: true,
            site_terminals: true,
            reinforcement_investigation: true,
            site_navigation_signals: true,
            site_terminal_navigation_signals: true,
            threat_renewal: true,
            destructibles: true,
            environmental_conduction: true,
            electronic_systems: true,
            player_relations: true,
        };
        let is_worm = |actor: &Actor| {
            actor
                .tags()
                .iter()
                .any(|tag| matches!(tag.as_str(), "core:armored_worm" | "core:cave_anemone"))
        };
        for biome in ["core:maintenance", "core:production", "core:research"] {
            let mut total = 0;
            for seed in 0..8 {
                let descriptor = RegionDescriptor {
                    coordinate: RegionCoord::new(
                        3,
                        -2,
                        if biome == "core:research" { 2 } else { 1 },
                    ),
                    seed,
                    biome: biome.parse().unwrap(),
                };
                let entrance = cardinal_passage(world.local_map_size(), Direction::West);
                let mut current = generate(
                    world,
                    &descriptor,
                    zone_info(world, &descriptor).unwrap(),
                    entrance,
                    Some(&loot),
                    features,
                )
                .unwrap();
                let old = generate(
                    previous,
                    &descriptor,
                    zone_info(previous, &descriptor).unwrap(),
                    entrance,
                    Some(&loot),
                    features,
                )
                .unwrap();
                total += current
                    .blueprint
                    .actors
                    .iter()
                    .filter(|actor| is_worm(actor))
                    .count();
                current.blueprint.actors.retain(|actor| !is_worm(actor));
                assert_eq!(
                    bincode::serialize(&current.blueprint).unwrap(),
                    bincode::serialize(&old.blueprint).unwrap()
                );
                assert_eq!(current.decor, old.decor);
                assert_eq!(current.facility, old.facility);
            }
            assert!(total > 0, "no new fauna in complete {biome} regions");
        }
        // Cities use the CURRENT catalogue, including the new machine population.
        let current_regions = ascii_regional_world_catalog().unwrap();
        let world = current_regions.get(&id).unwrap();
        for city in world.cities() {
            let descriptor = world.region(INITIAL_SEED, city.coordinate()).unwrap();
            let entrance = cardinal_passage(city.map_size(), Direction::West);
            let generated = generate(
                world,
                &descriptor,
                zone_info(world, &descriptor).unwrap(),
                entrance,
                Some(&loot),
                features,
            )
            .unwrap();
            assert!(
                !generated
                    .blueprint
                    .actors
                    .iter()
                    .any(|actor| actor.tags().iter().any(|tag| matches!(
                        tag.as_str(),
                        "core:armored_worm"
                            | "core:cave_anemone"
                            | "core:fault_howler"
                            | "core:sentinel_projector"
                    )))
            );
        }
    }

    #[test]
    fn underground_fauna_windup_and_recovery_survive_world_snapshots() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app =
            AsciiApp::from_seed(INITIAL_SEED, rules.clone(), texts, loot, expeditions).unwrap();
        for recovering in [false, true] {
            app.prepare_armored_worm_diagnostic(recovering).unwrap();
            app.game.drain_events();
            let bytes = app.game.recovery_snapshot_bytes().unwrap();
            let mut restored =
                WorldState::from_recovery_snapshot_bytes(&bytes, rules.clone()).unwrap();
            let worm = app.selected_target.unwrap();
            assert_eq!(
                restored.telegraphed_attack_cells(worm),
                app.game.telegraphed_attack_cells(worm)
            );
            for _ in 0..4 {
                assert_eq!(
                    restored.process_player_command(GameCommand::Wait),
                    app.game.process_player_command(GameCommand::Wait)
                );
                assert_eq!(restored.drain_events(), app.game.drain_events());
                assert_eq!(
                    restored.recovery_snapshot_bytes().unwrap(),
                    app.game.recovery_snapshot_bytes().unwrap()
                );
            }
        }
    }

    #[test]
    fn underground_fauna_uses_damp_habitats_safe_arrivals_and_bounded_biological_groups() {
        use project_rl::world::generation::{RegionalMapGenerator, generate_regional_fauna};
        let regions = ascii_regional_world_catalog().unwrap();
        let world = regions
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        for biome_id in ["core:maintenance", "core:production"] {
            let biome = world.biome(&biome_id.parse().unwrap()).unwrap();
            let profile = biome.fauna().unwrap();
            let mut total = 0;
            for seed in 0..32 {
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
                .map(|direction| region.passage(direction))
                .collect();
                let reserved: std::collections::BTreeSet<_> =
                    region.terrain().keys().copied().take(40).collect();
                let animals = generate_regional_fauna(
                    region.map(),
                    region.terrain(),
                    &passages,
                    &reserved,
                    profile,
                    seed,
                )
                .unwrap();
                assert_eq!(
                    animals,
                    generate_regional_fauna(
                        region.map(),
                        region.terrain(),
                        &passages,
                        &reserved,
                        profile,
                        seed
                    )
                    .unwrap()
                );
                assert!(animals.len() <= 2);
                total += animals.len();
                for animal in animals {
                    let at = animal.position();
                    assert!(
                        profile.families[0].species[0]
                            .habitats
                            .contains(&region.terrain()[&at])
                    );
                    assert!(!reserved.contains(&at) && !region.map().is_protected(at));
                    assert!(
                        passages
                            .iter()
                            .all(|entry| at.x.abs_diff(entry.x) + at.y.abs_diff(entry.y) >= 20)
                    );
                    assert!(
                        animal
                            .tags()
                            .contains(&"core:armored_worm".parse().unwrap())
                    );
                    assert_eq!(animal.ai().unwrap().maximum_pursuit_distance(), Some(5));
                    assert!(animal.electronic_system().is_none());
                    assert!(animal.equipped_weapon().is_none());
                }
            }
            assert!(total >= 16, "{biome_id}: {total} animals across 32 seeds");
        }
        let legacy = regions.without_underground_fauna_metadata();
        let legacy = legacy
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        for (_, biome) in [
            ("surface", "core:human_habitat"),
            ("wilds", "core:surface_wilds"),
            ("research", "core:research"),
        ] {
            assert_eq!(
                world.biome(&biome.parse().unwrap()),
                legacy.biome(&biome.parse().unwrap())
            );
        }
        assert!(
            legacy
                .biome(&"core:maintenance".parse().unwrap())
                .unwrap()
                .fauna()
                .is_none()
        );
        assert!(
            legacy
                .biome(&"core:production".parse().unwrap())
                .unwrap()
                .fauna()
                .is_none()
        );
    }

    #[test]
    fn underground_fauna_preview_never_reveals_hidden_actors_or_cells() {
        use project_rl::ai::{AiBehavior, AiState};
        let map = project_rl::world::Map::from_ascii(
            "#################\n#...............#\n#...............#\n#...............#\n#...............#\n#...............#\n#...............#\n#...............#\n#################"
        ).unwrap();
        for x in [8, 12] {
            let mut game = GameState::new(map.clone(), GridPos::new(1, 4), 19).unwrap();
            let origin = GridPos::new(x, 4);
            let actor = Actor::new(origin, 14)
                .unwrap()
                .with_ai(AiProfile::new(AiBehavior::TelegraphedSweeper, 7, 0, 256, 0))
                .with_attack(
                    AttackProfile::new(
                        2,
                        DistanceMetric::Chebyshev,
                        true,
                        DamageType::Kinetic,
                        4,
                        0,
                    )
                    .with_delivery(AttackDelivery::Melee)
                    .with_area(AttackArea::Cone(
                        project_rl::combat::ConeAttack::new(1, 1, 1).unwrap(),
                    )),
                );
            // Assemble a persisted wind-up without exposing a mutable engine
            // actor API just for this client visibility test.
            let mut persisted = serde_json::to_value(actor).unwrap();
            persisted["ai_state"] = serde_json::to_value(AiState::Aiming {
                origin,
                target_at: GridPos::new(x + 2, 4),
            })
            .unwrap();
            let actor: Actor = serde_json::from_value(persisted).unwrap();
            let worm = game.spawn_actor(actor).unwrap();
            let visible = crate::terminal_view::visible_telegraphed_attack_cells(&game, worm);
            let all = game.telegraphed_attack_cells(worm);
            assert_eq!(all.len(), 4);
            assert!(
                visible
                    .iter()
                    .all(|cell| game.player_visibility().is_visible(cell.position))
            );
            if x == 12 {
                assert!(visible.is_empty());
            } else {
                assert!(!visible.is_empty() && visible.len() < all.len());
            }
        }
    }

    #[test]
    fn underground_fauna_keeps_v109_v110_and_v111_suspensions_resumable() {
        for version in [
            109,
            UNDERGROUND_FAUNA_GENERATION_VERSION,
            CAVE_ANEMONE_GENERATION_VERSION,
            DEEP_ENCOUNTERS_GENERATION_VERSION,
        ] {
            let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
            let mut app = AsciiApp::from_seed_version(
                INITIAL_SEED,
                rules.clone(),
                texts.clone(),
                loot.clone(),
                expeditions.clone(),
                version,
            )
            .unwrap();
            for _ in 0..2 {
                app.execute_command(GameCommand::Wait);
            }
            app.game.drain_events();
            let expected = app.game.recovery_snapshot_bytes().unwrap();
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
                assert_eq!(restored.game.recovery_snapshot_bytes().unwrap(), expected);
                let has_worm = restored
                    .regional_worlds
                    .get(&"core:simulation_overworld".parse().unwrap())
                    .unwrap()
                    .biome(&"core:maintenance".parse().unwrap())
                    .unwrap()
                    .fauna()
                    .is_some();
                assert_eq!(has_worm, version >= UNDERGROUND_FAUNA_GENERATION_VERSION);
                let has_anemone = restored
                    .regional_worlds
                    .get(&"core:simulation_overworld".parse().unwrap())
                    .unwrap()
                    .biome(&"core:research".parse().unwrap())
                    .unwrap()
                    .fauna()
                    .is_some();
                assert_eq!(has_anemone, version >= CAVE_ANEMONE_GENERATION_VERSION);
            }
        }
    }

    #[test]
    fn underground_fauna_preserves_previous_world_fingerprints() {
        let (_, _, _, expeditions) = ascii_game_content().unwrap();
        let regions = ascii_regional_world_catalog().unwrap();
        // Captured on the v109 catalogue BEFORE adding the underground fauna.
        for (version, expected) in [
            (100, 12125727850193374453_u64),
            (101, 4939289738056939953),
            (102, 1190369058083372437),
            (103, 1879847749073525773),
            (108, 1578296249672886813),
            (109, 15810520955881226503),
            (110, 747664789216364421),
            (111, 3468351940251262990),
            (112, 8903987091305210840),
        ] {
            assert_eq!(
                world_fingerprint_for_version(&expeditions, &regions, version),
                expected,
                "v{version}"
            );
        }
    }

    #[test]
    fn underground_fauna_diagnostics_expose_the_whole_sweep_and_recovery() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.prepare_armored_worm_diagnostic(false).unwrap();
        let worm = app.selected_target.unwrap();
        let cells = crate::terminal_view::visible_telegraphed_attack_cells(&app.game, worm);
        assert_eq!(cells.len(), 4);
        assert_eq!(app.hostile_glyph(worm), 'W');
        assert!(
            app.terminal_target_summary()
                .unwrap()
                .name
                .starts_with("Ver cuirassé")
        );
        assert!(
            app.terminal_target_summary()
                .unwrap()
                .visible_state
                .contains("Balayage")
        );
        assert!(app.log.iter().any(|line| line.contains("zone marquée")));
        assert!(!app.log.iter().any(|line| line.contains("artilleur")));
        let player_hp = app
            .game
            .actors()
            .get(app.game.player_id())
            .unwrap()
            .integrity();
        app.prepare_armored_worm_diagnostic(true).unwrap();
        let worm = app.selected_target.unwrap();
        assert!(crate::terminal_view::visible_telegraphed_attack_cells(&app.game, worm).is_empty());
        assert!(
            app.terminal_target_summary()
                .unwrap()
                .visible_state
                .contains("Récupère")
        );
        assert_eq!(
            app.game
                .actors()
                .get(app.game.player_id())
                .unwrap()
                .integrity(),
            player_hp
        );
    }
}
