//! Real authored profiles in isolated native diagnostics and regression probes.
#[cfg(any(test, debug_assertions))]
use super::*;
#[cfg(any(test, debug_assertions))]
use project_rl::content::RegionTerrain;
#[cfg(any(test, debug_assertions))]
use project_rl::world::{Map, generation::*};

#[cfg(any(test, debug_assertions))]
const KINDS: [(&str, &str, RegionTerrain, u16); 5] = [
    ("cave_scaled", "research", RegionTerrain::Gravel, 2),
    ("void_maw", "corrupted", RegionTerrain::Mud, 5),
    ("laggard", "network", RegionTerrain::RuinFloor, 4),
    ("ruin_bat", "surface_wilds", RegionTerrain::RuinFloor, 0),
    ("riveter", "maintenance", RegionTerrain::RuinFloor, 1),
];

#[cfg(any(test, debug_assertions))]
fn arena() -> Map {
    Map::from_ascii("#############\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#############").unwrap()
}

#[cfg(any(test, debug_assertions))]
fn specimen(catalog: &RegionalWorldCatalog, index: usize) -> Actor {
    let (name, biome, habitat, _) = KINDS[index];
    let world = catalog
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap();
    let biome = world
        .biome(&format!("core:{biome}").parse().unwrap())
        .unwrap();
    let at = GridPos::new(6, 4);
    let mut map = arena();
    for y in 1..9 {
        for x in 1..12 {
            if GridPos::new(x, y) != at && !(index == 3 && GridPos::new(x, y) == GridPos::new(6, 5))
            {
                map.set_protected(GridPos::new(x, y), true).unwrap();
            }
        }
    }
    for seed in 0..256 {
        let actors = if index == 4 {
            let profile = project_rl::content::RegionPopulationProfile::new(
                1,
                1,
                biome
                    .population()
                    .rules()
                    .iter()
                    .filter(|rule| rule.ai().behavior == project_rl::ai::AiBehavior::Riveter)
                    .cloned()
                    .collect(),
            )
            .unwrap();
            generate_regional_population(
                &map,
                &[],
                &profile,
                seed,
                RegionalPopulationFeatures {
                    pursuit_lifecycle: true,
                    primary_attributes: true,
                    physical_profiles: true,
                    electronic_systems: true,
                    player_relations: true,
                },
            )
            .unwrap()
        } else {
            generate_regional_fauna(
                &map,
                &[(at, habitat), (GridPos::new(6, 5), habitat)].into(),
                &[],
                &Default::default(),
                biome.fauna().unwrap(),
                seed,
            )
            .unwrap()
        };
        if let Some(actor) = actors.into_iter().find(|a| {
            a.position() == at
                && a.tags()
                    .iter()
                    .any(|t| t.as_str() == format!("core:{name}"))
        }) {
            return actor;
        }
    }
    panic!("Missing authored specimen: {name}");
}

#[cfg(any(test, debug_assertions))]
impl AsciiApp {
    pub(super) fn prepare_bestiary_batch_diagnostic(&mut self, scene: &str) -> Result<(), String> {
        let index = match scene {
            "bestiary-scaled" => 0,
            "bestiary-maw" => 1,
            "bestiary-laggard" => 2,
            "bestiary-bat" => 3,
            "bestiary-riveter" => 4,
            _ => return Err("Scène inconnue".into()),
        };
        let mut actor = specimen(&self.regional_worlds, index);
        let world = self
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        balance_layer_encounters(
            std::slice::from_mut(&mut actor),
            world.encounter_balance().unwrap(),
            KINDS[index].3,
            KINDS[index].3,
            INITIAL_SEED,
        );
        let mut map = arena();
        let mut decor = SectorDecor::default();
        if index == 3 {
            map.set_terrain(GridPos::new(5, 4), project_rl::world::Terrain::DeepWater)
                .unwrap();
            decor
                .cells
                .insert(actor.ai_home().unwrap(), crate::test_sector::Decor::Nest);
        }
        let mut game = GameState::new_with_rules(
            map,
            GridPos::new(if index == 3 { 3 } else { 4 }, 4),
            INITIAL_SEED,
            self.rules.clone(),
        )
        .map_err(|e| e.to_string())?;
        let id = game.spawn_actor(actor).map_err(|e| e.to_string())?;
        for _ in 0..if index == 2 { 3 } else { 1 } {
            game.process_player_command(GameCommand::Wait);
        }
        self.terminal = TerminalView::new(decor, game.map(), game.player_visibility());
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.selected_target = Some(id);
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
    fn bestiary_batch_riveter_uses_the_existing_keyboard_and_mouse_context_choice() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.prepare_bestiary_batch_diagnostic("bestiary-riveter")
            .unwrap();
        assert_eq!(
            app.execute_command(GameCommand::Move(Direction::East)),
            CommandOutcome::Applied
        );
        let at = GridPos::new(6, 4);
        assert!(app.interaction_candidates().contains(&at));
        assert_eq!(app.interaction_display_name(at).as_deref(), Some("Riveuse"));
        assert_eq!(
            app.context_choice_label(ContextChoice::Interact(at)),
            "Arrêter le chantier : Riveuse"
        );
        let command = app.interaction_command_at(at).unwrap();
        assert_eq!(app.execute_command(command), CommandOutcome::Applied);
        assert!(matches!(
            app.game
                .actors()
                .get(app.selected_target.unwrap())
                .unwrap()
                .ai_state(),
            project_rl::ai::AiState::Encounter(project_rl::ai::EncounterState::WorkStopped)
        ));
    }
    #[test]
    fn bestiary_batch_white_weapon_hits_remain_in_role_bands_at_every_authored_depth() {
        let catalog = ascii_regional_world_catalog().unwrap();
        let world = catalog
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        let (rules, ..) = ascii_game_content().unwrap();
        let knives = [
            "core:couteau_de_camp",
            "core:couteau_de_sapeur",
            "core:couteau_ceramique",
            "core:couteau_de_chitine",
            "core:couteau_a_dent_vivante",
            "core:couteau_du_dernier_seuil",
        ];
        for (index, (_, biome, _, reference)) in KINDS.into_iter().enumerate() {
            let base = specimen(&catalog, index);
            let definition = world
                .biome(&format!("core:{biome}").parse().unwrap())
                .unwrap();
            let maximum_depth = if index == 4 {
                let production = world.biome(&"core:production".parse().unwrap()).unwrap();
                let is_riveter = |rule: &&project_rl::content::RegionPopulationRule| {
                    rule.ai().behavior == project_rl::ai::AiBehavior::Riveter
                };
                assert_eq!(
                    definition.population().rules().iter().find(is_riveter),
                    production.population().rules().iter().find(is_riveter)
                );
                production.maximum_depth().unwrap()
            } else {
                definition.maximum_depth().unwrap()
            };
            for depth in reference..=maximum_depth {
                for seed in 0..8 {
                    let mut actor = base.clone();
                    balance_layer_encounters(
                        std::slice::from_mut(&mut actor),
                        world.encounter_balance().unwrap(),
                        depth,
                        reference,
                        seed,
                    );
                    let mut rules = rules.clone();
                    let knife = knives[usize::from(depth.min(5))];
                    rules.player_starting_weapons = vec![knife.parse().unwrap()];
                    rules.player_starting_equipment = vec![Some(knife.parse().unwrap())];
                    let mut game =
                        GameState::new_with_rules(arena(), GridPos::new(5, 4), seed, rules)
                            .unwrap();
                    let id = game.spawn_actor(actor).unwrap();
                    let actor = game.actors().get(id).unwrap();
                    let level = actor.defeat_reward().unwrap().threat_level;
                    let band = world.encounter_balance().unwrap().layers[usize::from(depth)].levels;
                    assert!((band[0]..=band[1]).contains(&level));
                    let attack = game.equipped_player_weapon(0).unwrap().attack();
                    let damage = project_rl::combat::resolve_damage_impact_with_armor(
                        game.resolved_attack_damage(game.player_id(), attack)
                            .unwrap(),
                        actor.resistances(),
                        game.actor_armor_profile(id).unwrap(),
                        game.rules().damage,
                        game.rules().armor_rules.unwrap(),
                    )
                    .amount();
                    let hits = actor.integrity().div_ceil(damage.max(1));
                    let band = if index == 3 { 1..=3 } else { 3..=5 };
                    assert!(
                        band.contains(&hits),
                        "{} depth {depth} seed {seed}: {hits} hits (HP {}, damage {damage})",
                        KINDS[index].0,
                        actor.integrity()
                    );
                }
            }
        }
    }
    #[test]
    fn bestiary_batch_names_hints_and_committed_states_survive_world_snapshots() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app =
            AsciiApp::from_seed(INITIAL_SEED, rules.clone(), texts, loot, expeditions).unwrap();
        for (scene, name, glyph) in [
            ("bestiary-scaled", "Écailleux des cavernes", 'C'),
            ("bestiary-maw", "Gueule du vide", 'D'),
            ("bestiary-laggard", "Traînard", 'F'),
            ("bestiary-bat", "Chauve-souris des ruines", 'H'),
            ("bestiary-riveter", "Riveuse", 'I'),
        ] {
            app.prepare_bestiary_batch_diagnostic(scene).unwrap();
            let target = app.selected_target.unwrap();
            assert_eq!(app.hostile_glyph(target), glyph);
            assert!(
                app.terminal_target_summary()
                    .unwrap()
                    .name
                    .starts_with(name)
            );
            if glyph != 'C' {
                assert!(
                    !crate::terminal_view::visible_telegraphed_attack_cells(&app.game, target)
                        .is_empty()
                );
            }
            app.game.drain_events();
            let bytes = app.game.recovery_snapshot_bytes().unwrap();
            let mut restored =
                WorldState::from_recovery_snapshot_bytes(&bytes, rules.clone()).unwrap();
            for _ in 0..8 {
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
    fn bestiary_batch_preserves_v115_catalog_and_snapshot_or_replay_restore() {
        let regions = ascii_regional_world_catalog().unwrap();
        let old = regions.without_bestiary_batch_metadata();
        let text = format!(
            "{:?}",
            old.get(&"core:simulation_overworld".parse().unwrap())
                .unwrap()
        );
        for (name, ..) in KINDS {
            assert!(!text.contains(&format!("core:{name}")));
        }
        assert!(
            text.contains("core:marsh_spitter")
                && text.contains("core:watcher")
                && text.contains("core:spectre")
        );
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        assert_eq!(
            world_fingerprint_for_version(&expeditions, &regions, 115),
            world_fingerprint_for_version(&expeditions, &old, 115)
        );
        for version in [115, 116] {
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
                let actual = AsciiApp::restore_suspension(
                    &saved,
                    rules.clone(),
                    texts.clone(),
                    loot.clone(),
                    expeditions.clone(),
                )
                .unwrap();
                assert_eq!(expected, actual.game.recovery_snapshot_bytes().unwrap());
            }
        }
    }

    #[test]
    fn bestiary_batch_real_regions_produce_all_five_in_safe_habitats_without_extra_draws() {
        use project_rl::content::RegionCoord;
        let catalog = ascii_regional_world_catalog().unwrap();
        let old = catalog.without_bestiary_batch_metadata();
        let id = "core:simulation_overworld".parse().unwrap();
        let world = catalog.get(&id).unwrap();
        for (index, (name, biome, habitat, depth)) in KINDS.into_iter().enumerate() {
            let biome_id = format!("core:{biome}").parse().unwrap();
            let biome = world.biome(&biome_id).unwrap();
            let before = old.get(&id).unwrap().biome(&biome_id).unwrap();
            if index == 4 {
                assert_eq!(
                    biome.population().maximum_group_rolls(),
                    before.population().maximum_group_rolls()
                );
            } else {
                assert_eq!(
                    biome.fauna().unwrap().group_rolls,
                    before.fauna().unwrap().group_rolls
                );
                assert_eq!(
                    biome.fauna().unwrap().danger_budget,
                    before.fauna().unwrap().danger_budget
                );
            }
            let mut observed = 0;
            for seed in 0..32 {
                let generated = RegionalMapGenerator::new(
                    world.map_size_at(RegionCoord::new(3, -2, depth)),
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
                let actors = if index == 4 {
                    generate_regional_population(
                        generated.map(),
                        &passages,
                        biome.population(),
                        seed,
                        RegionalPopulationFeatures {
                            pursuit_lifecycle: true,
                            primary_attributes: true,
                            physical_profiles: true,
                            electronic_systems: true,
                            player_relations: true,
                        },
                    )
                    .unwrap()
                } else {
                    generate_regional_fauna(
                        generated.map(),
                        generated.terrain(),
                        &passages,
                        &Default::default(),
                        biome.fauna().unwrap(),
                        seed,
                    )
                    .unwrap()
                };
                for actor in actors.iter().filter(|a| {
                    a.tags()
                        .iter()
                        .any(|t| t.as_str() == format!("core:{name}"))
                }) {
                    observed += 1;
                    assert!(!generated.map().is_protected(actor.position()));
                    for passage in passages {
                        assert!(
                            actor.position().x.abs_diff(passage.x)
                                + actor.position().y.abs_diff(passage.y)
                                >= if index == 3 { 16 } else { 20 }
                        );
                    }
                    if index == 3 {
                        assert_eq!(generated.terrain().get(&actor.position()), Some(&habitat));
                        assert!(actor.ai_home().is_some());
                    }
                    if index < 4 {
                        let species = biome
                            .fauna()
                            .unwrap()
                            .families
                            .iter()
                            .flat_map(|f| &f.species)
                            .find(|s| s.id.as_str() == format!("core:{name}"))
                            .unwrap();
                        assert!(
                            species
                                .habitats
                                .contains(&generated.terrain()[&actor.position()])
                        );
                    }
                    if index == 4 {
                        assert_eq!(
                            actor.player_relation(),
                            project_rl::social::PlayerRelation::Neutral
                        );
                        assert_eq!(actor.body_components().count(), 2);
                    } else {
                        assert!(actor.electronic_system().is_none());
                        assert_eq!(actor.body_components().count(), 0);
                    }
                    assert!(actor.equipped_weapon().is_none());
                }
            }
            assert!(observed > 0, "not generated: {name}");
        }
    }
}
