//! Deliberately placed encounters using authored profiles, not campaign spawns.
//! These probes test counterplay; they do not simulate equipment progression.
use super::*;
use project_rl::content::RegionTerrain;
use project_rl::world::{Map, generation::*};

#[cfg(test)]
const SCENES: [&str; 3] = ["mixed-surface", "mixed-research", "mixed-corrupted"];

fn arena() -> Map {
    Map::from_ascii("#############\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#############").unwrap()
}

pub(super) fn encounter(
    rules: &GameRules,
    catalog: &RegionalWorldCatalog,
    scene: &str,
    seed: u64,
) -> Result<(WorldState, Vec<EntityId>, SectorDecor), String> {
    encounter_on_map(rules, catalog, scene, seed, arena())
}

fn encounter_on_map(
    rules: &GameRules,
    catalog: &RegionalWorldCatalog,
    scene: &str,
    seed: u64,
    map: Map,
) -> Result<(WorldState, Vec<EntityId>, SectorDecor), String> {
    use RegionTerrain::{Gravel, Mud, RuinFloor};
    let (biome_id, depth, player, groups): (_, _, _, Vec<(&str, _, Vec<GridPos>)>) = match scene {
        "mixed-surface" => (
            "core:surface_wilds",
            0,
            GridPos::new(3, 4),
            vec![
                (
                    "core:ruin_bat",
                    RuinFloor,
                    vec![GridPos::new(6, 4), GridPos::new(6, 5)],
                ),
                ("core:marsh_spitter", Mud, vec![GridPos::new(5, 2)]),
            ],
        ),
        "mixed-research" => (
            "core:research",
            2,
            GridPos::new(4, 4),
            vec![
                ("core:cave_scaled", Gravel, vec![GridPos::new(6, 4)]),
                ("core:fault_howler", Gravel, vec![GridPos::new(6, 6)]),
            ],
        ),
        "mixed-corrupted" => (
            "core:corrupted",
            5,
            GridPos::new(4, 4),
            vec![
                ("core:void_maw", Mud, vec![GridPos::new(6, 4)]),
                ("core:spectre", Mud, vec![GridPos::new(7, 6)]),
            ],
        ),
        _ => return Err(format!("Scène inconnue : {scene}")),
    };
    let world = catalog
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap();
    let biome = world.biome(&biome_id.parse().unwrap()).unwrap();
    let mut actors = Vec::new();
    let mut decor = SectorDecor::default();
    for (species, habitat, cells) in groups {
        let mut spawn_map = arena();
        for y in 1..9 {
            for x in 1..12 {
                let at = GridPos::new(x, y);
                spawn_map.set_protected(at, !cells.contains(&at)).unwrap();
            }
        }
        // Only the draw is narrowed. Count, attacks, senses, body, loot and AI
        // remain those of the real species, including the bats' shared nest.
        let mut profile = biome.fauna().unwrap().clone();
        for family in &mut profile.families {
            family.species.retain(|s| s.id.as_str() == species);
        }
        profile.families.retain(|family| !family.species.is_empty());
        let terrain = cells.iter().map(|&at| (at, habitat)).collect();
        let mut group = generate_regional_fauna(
            &spawn_map,
            &terrain,
            &[],
            &Default::default(),
            &profile,
            seed,
        )?;
        if group.len() != cells.len() {
            return Err(format!(
                "{species} : {} individus au lieu de {}",
                group.len(),
                cells.len()
            ));
        }
        balance_layer_encounters(
            &mut group,
            world.encounter_balance().unwrap(),
            depth,
            depth,
            seed,
        );
        for actor in &group {
            if actor.ai().unwrap().behavior == project_rl::ai::AiBehavior::NestDiver {
                decor
                    .cells
                    .insert(actor.ai_home().unwrap(), crate::test_sector::Decor::Nest);
            }
        }
        actors.extend(group);
    }
    let mut game =
        GameState::new_with_rules(map, player, seed, rules.clone()).map_err(|e| e.to_string())?;
    let ids = actors
        .into_iter()
        .map(|actor| game.spawn_actor(actor).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    Ok((WorldState::single(game), ids, decor))
}

impl AsciiApp {
    pub(super) fn prepare_mixed_encounter_diagnostic(&mut self, scene: &str) -> Result<(), String> {
        let (mut game, ids, decor) =
            encounter(&self.rules, &self.regional_worlds, scene, INITIAL_SEED)?;
        game.process_player_command(GameCommand::Wait);
        self.terminal = TerminalView::new(decor, game.map(), game.player_visibility());
        self.game = game;
        self.actor_glyphs.clear();
        self.selected_target = ids.last().copied();
        self.intro_city_reached = true;
        self.log.clear();
        self.capture_events_at(Some(0.0));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use project_rl::ai::{AiBehavior, AiState, EncounterState};

    fn hp(game: &WorldState) -> u16 {
        game.actors()
            .get(game.player_id())
            .map_or(0, Actor::integrity)
    }

    #[test]
    fn mixed_encounters_allow_an_announced_retreat_with_real_profiles() {
        let (rules, ..) = ascii_game_content().unwrap();
        let catalog = ascii_regional_world_catalog().unwrap();
        for scene in SCENES {
            for seed in 0..16 {
                let (mut game, ids, _) = encounter(&rules, &catalog, scene, seed).unwrap();
                let before = hp(&game);
                game.process_player_command(GameCommand::Wait);
                assert_eq!(
                    hp(&game),
                    before,
                    "{scene}, seed {seed}: no surprise hit on approach"
                );
                let warnings: Vec<_> = ids
                    .iter()
                    .filter(|&&id| !game.telegraphed_attack_cells(id).is_empty())
                    .collect();
                assert!(
                    warnings.len() >= if scene == "mixed-research" { 1 } else { 2 },
                    "{scene}: missing simultaneous warnings"
                );
                game.drain_events();
                let mut performed = 0;
                // A real sequence of player commands, not position teleportation.
                // The second westward step clears the expanding outer ring.
                for _ in 0..2 {
                    let destination = game.player_position().unwrap().step(Direction::West);
                    assert!(
                        ids.iter().all(|&id| game
                            .telegraphed_attack_cells(id)
                            .iter()
                            .all(|c| c.position != destination)),
                        "{scene}: retreat enters a warning"
                    );
                    assert_eq!(
                        game.process_player_command(GameCommand::Move(Direction::West)),
                        CommandOutcome::Applied
                    );
                    assert_eq!(
                        hp(&game),
                        before,
                        "{scene}, seed {seed}: advertised retreat damaged player"
                    );
                    performed += game
                        .drain_events()
                        .iter()
                        .filter(|e| matches!(e, GameEvent::AttackPerformed { .. }))
                        .count();
                }
                if scene != "mixed-research" {
                    assert!(
                        performed >= 2,
                        "{scene}: enemies must execute, not merely cancel every attack"
                    );
                }
            }
        }
    }

    #[test]
    fn mixed_encounters_standing_in_warnings_is_worse_than_retreating() {
        let (rules, ..) = ascii_game_content().unwrap();
        let catalog = ascii_regional_world_catalog().unwrap();
        for scene in SCENES {
            let mut losses = Vec::new();
            for seed in 0..16 {
                let (mut game, _, _) = encounter(&rules, &catalog, scene, seed).unwrap();
                let before = hp(&game);
                for _ in 0..4 {
                    game.process_player_command(GameCommand::Wait);
                }
                let events = game.drain_events();
                let attempts = events
                    .iter()
                    .filter(|e| {
                        matches!(e,
                            GameEvent::AttackHitResolved { target, chance, .. }
                            if *target == game.player_id() && *chance > 0
                        )
                    })
                    .count();
                assert!(
                    attempts >= 2,
                    "{scene}, seed {seed}: overlapping attacks must threaten the stationary player"
                );
                losses.push(before - hp(&game));
            }
            assert!(
                losses.iter().any(|&n| n > 0),
                "{scene}: the counterexample must be genuinely dangerous"
            );
            // Real attacks retain their hit rolls: surviving by two misses is
            // legitimate, unlike guaranteed safety from leaving the footprint.
            println!(
                "{scene}: staying still for four turns costs {}–{} HP (starting body, no equipment)",
                losses.iter().min().unwrap(),
                losses.iter().max().unwrap()
            );
        }
    }

    #[test]
    fn mixed_encounters_cover_and_protected_arrivals_block_combined_threats() {
        let (rules, ..) = ascii_game_content().unwrap();
        let catalog = ascii_regional_world_catalog().unwrap();
        for scene in SCENES {
            let mut map = arena();
            let at = GridPos::new(if scene == "mixed-surface" { 3 } else { 4 }, 4);
            map.set_protected(at, true).unwrap();
            let (mut game, ids, _) = encounter_on_map(&rules, &catalog, scene, 17, map).unwrap();
            let before = hp(&game);
            for _ in 0..8 {
                game.process_player_command(GameCommand::Wait);
                assert_eq!(hp(&game), before);
                assert!(
                    ids.iter()
                        .all(|&id| game.telegraphed_attack_cells(id).is_empty())
                );
            }
        }
        let mut map = arena();
        for y in 1..9 {
            map.set_terrain(GridPos::new(5, y), project_rl::world::Terrain::Wall)
                .unwrap();
        }
        let (mut game, ids, _) =
            encounter_on_map(&rules, &catalog, "mixed-corrupted", 17, map).unwrap();
        let before = hp(&game);
        for _ in 0..8 {
            game.process_player_command(GameCommand::Wait);
            assert_eq!(hp(&game), before);
            assert!(
                ids.iter()
                    .all(|&id| game.telegraphed_attack_cells(id).is_empty())
            );
        }
    }

    #[test]
    fn mixed_encounters_concurrent_commitments_restore_identical_turns() {
        let (rules, ..) = ascii_game_content().unwrap();
        let catalog = ascii_regional_world_catalog().unwrap();
        for scene in SCENES {
            let (mut game, _, _) = encounter(&rules, &catalog, scene, 17).unwrap();
            game.process_player_command(GameCommand::Wait);
            game.drain_events();
            let bytes = game.recovery_snapshot_bytes().unwrap();
            let mut restored =
                WorldState::from_recovery_snapshot_bytes(&bytes, rules.clone()).unwrap();
            for command in [
                GameCommand::Move(Direction::West),
                GameCommand::Move(Direction::West),
                GameCommand::Wait,
                GameCommand::Wait,
                GameCommand::Wait,
                GameCommand::Wait,
            ] {
                assert_eq!(
                    game.process_player_command(command.clone()),
                    restored.process_player_command(command)
                );
                assert_eq!(game.drain_events(), restored.drain_events(), "{scene}");
                assert_eq!(
                    game.recovery_snapshot_bytes().unwrap(),
                    restored.recovery_snapshot_bytes().unwrap(),
                    "{scene}"
                );
            }
        }
    }

    #[test]
    fn mixed_encounters_bat_pair_keeps_distinct_cells_and_completes_shared_nest_retreat() {
        let (rules, ..) = ascii_game_content().unwrap();
        let catalog = ascii_regional_world_catalog().unwrap();
        for seed in 0..16 {
            let (mut game, ids, _) = encounter(&rules, &catalog, SCENES[0], seed).unwrap();
            let bats: Vec<_> = ids
                .into_iter()
                .filter(|&id| {
                    game.actors().get(id).unwrap().ai().unwrap().behavior == AiBehavior::NestDiver
                })
                .collect();
            assert_eq!(bats.len(), 2);
            assert_eq!(
                game.actors().get(bats[0]).unwrap().ai_home(),
                game.actors().get(bats[1]).unwrap().ai_home()
            );
            game.process_player_command(GameCommand::Wait);
            game.process_player_command(GameCommand::Move(Direction::West));
            for &id in &bats {
                assert!(matches!(
                    game.actors().get(id).unwrap().ai_state(),
                    AiState::Encounter(EncounterState::Withdrawal { .. })
                ));
            }
            for _ in 0..2 {
                game.drain_events();
                game.process_player_command(GameCommand::Wait);
                assert!(game.drain_events().iter().all(|e| !matches!(e, GameEvent::AttackPerformed { attacker, .. } if bats.contains(attacker))));
                assert_ne!(
                    game.actors().get(bats[0]).unwrap().position(),
                    game.actors().get(bats[1]).unwrap().position()
                );
            }
            for &id in &bats {
                let bat = game.actors().get(id).unwrap();
                assert_eq!(
                    bat.ai_state(),
                    AiState::Unaware,
                    "seed {seed}: pair stuck returning to shared nest"
                );
                let home = bat.ai_home().unwrap();
                assert!(
                    bat.position()
                        .x
                        .abs_diff(home.x)
                        .max(bat.position().y.abs_diff(home.y))
                        <= 1
                );
            }
        }
    }
}
