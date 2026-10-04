#[cfg(any(test, debug_assertions))]
use super::*;

#[cfg(any(test, debug_assertions))]
fn specimens(spectre: bool, seed: u64, depth: u16) -> Result<Vec<Actor>, String> {
    use project_rl::content::RegionTerrain;
    let catalog = ascii_regional_world_catalog()?;
    let world = catalog
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap();
    let biome = world
        .biome(
            &if spectre {
                "core:corrupted"
            } else {
                "core:network"
            }
            .parse()
            .unwrap(),
        )
        .unwrap();
    let mut map = project_rl::world::Map::from_ascii("#################\n#...............#\n#...............#\n#...............#\n#...............#\n#...............#\n#...............#\n#################").unwrap();
    for y in 1..7 {
        for x in 1..16 {
            let at = GridPos::new(x, y);
            if at != GridPos::new(6, 4) && (spectre || at != GridPos::new(9, 4)) {
                map.set_protected(at, true).unwrap();
            }
        }
    }
    let terrain = if spectre {
        RegionTerrain::Mud
    } else {
        RegionTerrain::RuinFloor
    };
    let terrain = (1..7)
        .flat_map(|y| (1..16).map(move |x| (GridPos::new(x, y), terrain)))
        .collect();
    let mut profile = biome.fauna().unwrap().clone();
    let wanted = if spectre {
        "core:spectre"
    } else {
        "core:watcher"
    };
    for family in &mut profile.families {
        family.species.retain(|s| s.id.as_str() == wanted);
    }
    profile.families.retain(|f| !f.species.is_empty());
    let mut actors = project_rl::world::generation::generate_regional_fauna(
        &map,
        &terrain,
        &[],
        &Default::default(),
        &profile,
        seed,
    )
    .map_err(|e| e.to_string())?;
    project_rl::world::generation::balance_layer_encounters(
        &mut actors,
        world.encounter_balance().unwrap(),
        depth,
        if spectre { 5 } else { 4 },
        seed,
    );
    actors.sort_by_key(Actor::position);
    Ok(actors)
}

#[cfg(any(test, debug_assertions))]
impl AsciiApp {
    pub(super) fn prepare_strange_fauna_diagnostic(&mut self, scene: &str) -> Result<(), String> {
        let spectre = scene.starts_with("spectre");
        let habitat = scene.starts_with("watcher-habitat");
        let mut map = project_rl::world::Map::from_ascii("#################\n#...............#\n#...............#\n#...............#\n#...............#\n#...............#\n#...............#\n#################").unwrap();
        if habitat {
            for at in [
                GridPos::new(7, 2),
                GridPos::new(7, 6),
                GridPos::new(12, 2),
                GridPos::new(12, 6),
            ] {
                map.set_terrain(at, Terrain::Wall)
                    .map_err(|e| e.to_string())?;
            }
        }
        let mut game =
            GameState::new_with_rules(map, GridPos::new(3, 4), INITIAL_SEED, self.rules.clone())
                .map_err(|e| e.to_string())?;
        let mut actors = specimens(spectre, 17, if spectre { 5 } else { 4 })?;
        let mut decor = SectorDecor::default();
        if habitat {
            decor.fallback_name = Some("Réseaux · abords des Guetteurs".to_owned());
            for y in 0..game.map().height() as i32 {
                for x in 0..game.map().width() as i32 {
                    let at = GridPos::new(x, y);
                    decor.cells.insert(
                        at,
                        if game.map().is_walkable(at) {
                            crate::test_sector::Decor::RuinFloor
                        } else {
                            crate::test_sector::Decor::RuinWall
                        },
                    );
                }
            }
            crate::watcher_signals::decorate_ground(&mut decor, game.map(), &actors);
        }
        let mut selected = None;
        for (index, actor) in actors.iter_mut().enumerate() {
            if index > 0 {
                // Only this diagnostic narrows the neighbor's sight so the
                // reported position is visible independently of direct sight.
                let mut ai = actor.ai().unwrap();
                ai.perception_radius = 1;
                *actor = actor.clone().with_ai(ai);
            }
            let id = game.spawn_actor(actor.clone()).map_err(|e| e.to_string())?;
            if index == 0 {
                selected = Some(id);
            }
        }
        if selected.is_none() {
            return Err("Faune absente du diagnostic".into());
        }
        if !habitat || scene.ends_with("signal") {
            game.process_player_command(GameCommand::Wait);
        }
        if scene == "spectre-impact" || scene == "spectre-recovery" {
            game.process_player_command(GameCommand::Wait);
        }
        self.terminal = TerminalView::new(decor, game.map(), game.player_visibility());
        if habitat {
            self.terminal.title = "RÉSEAUX · RENCONTRE DE DIAGNOSTIC".to_owned();
        }
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.selected_target = selected;
        self.intro_city_reached = !habitat;
        self.log.clear();
        self.capture_events_at(Some(0.0));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn watcher_habitat_signal_uses_real_events_and_keeps_floor_memory_static() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.prepare_strange_fauna_diagnostic("watcher-habitat")
            .unwrap();
        let before = app.terminal.decor.clone();
        let positions: Vec<_> = app
            .game
            .actors()
            .iter()
            .filter(|(_, a)| a.ai().is_some())
            .map(|(_, a)| a.position())
            .collect();
        app.game.process_player_command(GameCommand::Wait);
        app.capture_events_at(Some(0.0));
        assert!(
            app.log
                .iter()
                .any(|line| line.contains("Le Guetteur alerte ses voisins."))
        );
        assert!((1..16).any(|x| {
            app.visual_cues
                .sample_world(GridPos::new(x, 4), true, 0.18)
                .is_some_and(|sample| {
                    sample.family == crate::visual_effects::TerminalEffectFamily::Signal
                })
        }));
        assert_eq!(app.terminal.decor, before);
        assert_ne!(
            positions,
            app.game
                .actors()
                .iter()
                .filter(|(_, a)| a.ai().is_some())
                .map(|(_, a)| a.position())
                .collect::<Vec<_>>()
        );
        let snapshot = app.game.recovery_snapshot_bytes().unwrap();
        let mut restored =
            WorldState::from_recovery_snapshot_bytes(&snapshot, app.rules.clone()).unwrap();
        for _ in 0..3 {
            app.game.process_player_command(GameCommand::Wait);
            restored.process_player_command(GameCommand::Wait);
            assert_eq!(app.game.drain_events(), restored.drain_events());
            assert_eq!(
                app.game.recovery_snapshot_bytes().unwrap(),
                restored.recovery_snapshot_bytes().unwrap()
            );
        }
        assert_eq!(app.terminal.decor, before);
    }

    #[test]
    fn strange_fauna_generated_regions_keep_habitats_and_safe_arrivals() {
        use crate::test_regional::{RegionalGenerationFeatures, generate, zone_info};
        use project_rl::content::{RegionCoord, RegionDescriptor};
        use project_rl::world::generation::cardinal_passage;
        let catalog = ascii_regional_world_catalog().unwrap();
        let world = catalog
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
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
        for (biome, depth, tag, maximum) in [
            ("core:network", 4, "core:watcher", 2),
            ("core:network", 7, "core:watcher", 2),
            ("core:corrupted", 5, "core:spectre", 1),
            ("core:corrupted", 7, "core:spectre", 1),
        ] {
            let mut observed = 0;
            for seed in 0..12 {
                let descriptor = RegionDescriptor {
                    coordinate: RegionCoord::new(3, -2, depth),
                    seed,
                    biome: biome.parse().unwrap(),
                };
                let entry = cardinal_passage(world.local_map_size(), Direction::West);
                let generated = generate(
                    world,
                    &descriptor,
                    zone_info(world, &descriptor).unwrap(),
                    entry,
                    Some(&loot),
                    features,
                )
                .unwrap();
                let animals: Vec<_> = generated
                    .blueprint
                    .actors
                    .iter()
                    .filter(|a| a.tags().iter().any(|t| t.as_str() == tag))
                    .collect();
                assert!(animals.len() <= maximum);
                if tag == "core:watcher" && !animals.is_empty() {
                    assert!(
                        generated.decor.cells.values().any(|decor| matches!(
                            decor,
                            crate::test_sector::Decor::NetworkTrace(_)
                        ))
                    );
                }
                for (at, decor) in &generated.decor.cells {
                    if matches!(decor, crate::test_sector::Decor::NetworkTrace(_)) {
                        assert!(generated.blueprint.map.is_walkable(*at));
                        assert!(!generated.blueprint.map.is_protected(*at));
                    }
                }
                observed += animals.len();
                for actor in animals {
                    assert!(generated.blueprint.map.is_walkable(actor.position()));
                    assert!(!generated.blueprint.map.is_protected(actor.position()));
                    assert!(
                        (depth * 4..=depth * 4 + 3)
                            .contains(&actor.defeat_reward().unwrap().threat_level)
                    );
                    for passage in generated
                        .passages
                        .iter()
                        .map(|(_, p)| p)
                        .chain(generated.vertical_passages.iter().map(|(_, p)| p))
                    {
                        assert!(
                            actor.position().x.abs_diff(passage.x)
                                + actor.position().y.abs_diff(passage.y)
                                >= 20
                        );
                    }
                }
            }
            assert!(observed > 0, "no {tag} generated at depth {depth}");
        }
    }

    #[test]
    fn strange_fauna_real_profiles_remain_fragile_at_all_authored_depths() {
        for spectre in [false, true] {
            for depth in if spectre { 5..=7 } else { 4..=7 } {
                for seed in 0..8 {
                    let actors = specimens(spectre, seed, depth).unwrap();
                    assert_eq!(actors.len(), if spectre { 1 } else { 2 });
                    for actor in actors {
                        assert!(actor.tags().iter().any(|t| t.as_str()
                            == if spectre {
                                "core:spectre"
                            } else {
                                "core:watcher"
                            }));
                        assert!(
                            actor.maximum_integrity() <= 14,
                            "depth {depth}: {} HP",
                            actor.maximum_integrity()
                        );
                        assert!(actor.equipped_weapon().is_none());
                        assert!(actor.electronic_system().is_none());
                        assert_eq!(actor.body_components().count(), 0);
                        let level = actor.defeat_reward().unwrap().threat_level;
                        assert!((depth * 4..=depth * 4 + 3).contains(&level));
                    }
                }
            }
        }
    }

    #[test]
    fn strange_fauna_names_preview_and_snapshot_continuation_match() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app =
            AsciiApp::from_seed(INITIAL_SEED, rules.clone(), texts, loot, expeditions).unwrap();
        for scene in ["watcher", "spectre", "spectre-recovery"] {
            app.prepare_strange_fauna_diagnostic(scene).unwrap();
            let entity = app.selected_target.unwrap();
            assert!(app.terminal_target_summary().unwrap().name.starts_with(
                if scene == "watcher" {
                    "Guetteur"
                } else {
                    "Spectre"
                }
            ));
            assert_eq!(
                app.game.telegraphed_attack_cells(entity).len(),
                if scene == "spectre" { 2 } else { 0 }
            );
            let snapshot = app.game.recovery_snapshot_bytes().unwrap();
            let mut restored =
                WorldState::from_recovery_snapshot_bytes(&snapshot, rules.clone()).unwrap();
            app.game.drain_events();
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
    fn strange_fauna_old_generations_strip_only_the_new_species() {
        let regions = ascii_regional_world_catalog()
            .unwrap()
            .without_bestiary_batch_metadata();
        let old = regions.without_strange_fauna_metadata();
        let id = "core:simulation_overworld".parse().unwrap();
        for biome in ["core:network", "core:corrupted"] {
            assert!(
                old.get(&id)
                    .unwrap()
                    .biome(&biome.parse().unwrap())
                    .unwrap()
                    .fauna()
                    .is_none()
            );
        }
        for biome in [
            "core:maintenance",
            "core:production",
            "core:research",
            "core:security",
        ] {
            let biome = biome.parse().unwrap();
            assert_eq!(
                format!("{:?}", old.get(&id).unwrap().biome(&biome)),
                format!("{:?}", regions.get(&id).unwrap().biome(&biome))
            );
        }
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        assert_eq!(
            world_fingerprint_for_version(&expeditions, &regions, 113),
            world_fingerprint_for_version(&expeditions, &old, 113)
        );
        for version in [113, 114] {
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
                assert_eq!(expected, restored.game.recovery_snapshot_bytes().unwrap());
            }
        }
    }
}
