//! Isolated native captures and integration checks for the cave anemone.
use super::*;

impl AsciiApp {
    #[cfg(any(test, debug_assertions))]
    pub(super) fn prepare_cave_anemone_diagnostic(&mut self, grasped: bool) -> Result<(), String> {
        use project_rl::content::RegionTerrain;
        let map = project_rl::world::Map::from_ascii(
            "#############\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#############"
        ).map_err(|error| error.to_string())?;
        let at = GridPos::new(7, 4);
        let profile = self
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .and_then(|world| world.biome(&"core:research".parse().unwrap()))
            .and_then(|biome| biome.fauna())
            .ok_or("Anémone absente du catalogue")?;
        let terrain = [(at, RegionTerrain::ShallowWater)].into();
        let mut animals = project_rl::world::generation::generate_regional_fauna(
            &map,
            &terrain,
            &[],
            &Default::default(),
            profile,
            INITIAL_SEED,
        )?;
        let mut game =
            GameState::new_with_rules(map, GridPos::new(6, 4), INITIAL_SEED, self.rules.clone())
                .map_err(|error| error.to_string())?;
        let anemone = game
            .spawn_actor(animals.pop().ok_or("Anémone non générée")?)
            .map_err(|error| error.to_string())?;
        game.process_player_command(GameCommand::Wait);
        if grasped {
            // A real successful hit, not a manually applied presentation status.
            // Bounded retries account for the player's ordinary evasion.
            for _ in 0..24 {
                game.process_player_command(GameCommand::Wait);
                if game.actors().get(game.player_id()).is_some_and(|player| {
                    player
                        .status(&"core:locomotion_hindered".parse().unwrap())
                        .is_some()
                }) {
                    break;
                }
            }
            if !game.actors().get(game.player_id()).is_some_and(|player| {
                player
                    .status(&"core:locomotion_hindered".parse().unwrap())
                    .is_some()
            }) {
                return Err("Le diagnostic n'a pas produit de prise réelle".into());
            }
        }
        let mut decor = SectorDecor::default();
        decor
            .cells
            .insert(at, crate::test_sector::Decor::ShallowWater);
        self.terminal = TerminalView::new(decor, game.map(), game.player_visibility());
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.selected_target = Some(anemone);
        self.intro_city_reached = true;
        self.log.clear();
        self.capture_events_at(Some(0.0));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terminal_view::TerminalEffectBadge;

    #[test]
    fn cave_anemone_diagnostics_show_real_neighbors_hindrance_and_persistent_recovery() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app =
            AsciiApp::from_seed(INITIAL_SEED, rules.clone(), texts, loot, expeditions).unwrap();
        for grasped in [false, true] {
            app.prepare_cave_anemone_diagnostic(grasped).unwrap();
            let animal = app.selected_target.unwrap();
            assert_eq!(app.hostile_glyph(animal), 'J');
            assert!(
                app.terminal_target_summary()
                    .unwrap()
                    .name
                    .starts_with("Anémone des caves")
            );
            let cells = crate::terminal_view::visible_telegraphed_attack_cells(&app.game, animal);
            assert_eq!(cells.len(), if grasped { 0 } else { 8 });
            assert_eq!(
                app.effect_badges_at(app.game.player_position().unwrap())
                    .contains(&TerminalEffectBadge::Slowed),
                grasped
            );
            if grasped {
                assert!(
                    app.log
                        .iter()
                        .any(|line| line.contains("Les vrilles vous entravent"))
                );
                assert!(
                    !app.log
                        .iter()
                        .any(|line| line.contains("LOCOMOTION HINDERED"))
                );
            }
            let bytes = app.game.recovery_snapshot_bytes().unwrap();
            let mut restored =
                WorldState::from_recovery_snapshot_bytes(&bytes, rules.clone()).unwrap();
            app.game.drain_events();
            restored.drain_events();
            for _ in 0..4 {
                assert_eq!(
                    app.game.process_player_command(GameCommand::Wait),
                    restored.process_player_command(GameCommand::Wait)
                );
                assert_eq!(app.game.drain_events(), restored.drain_events());
                assert_eq!(
                    app.game.recovery_snapshot_bytes().unwrap(),
                    restored.recovery_snapshot_bytes().unwrap()
                );
            }
        }
    }

    #[test]
    fn cave_anemone_generation_respects_water_safe_arrivals_and_biological_identity() {
        use project_rl::world::generation::{RegionalMapGenerator, generate_regional_fauna};
        let regions = ascii_regional_world_catalog()
            .unwrap()
            .without_bestiary_batch_metadata()
            .without_deep_encounters_metadata();
        let world = regions
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        let biome = world.biome(&"core:research".parse().unwrap()).unwrap();
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
            let reserved = region.terrain().keys().copied().take(40).collect();
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
            assert!(animals.len() <= 1);
            total += animals.len();
            for actor in animals {
                assert_eq!(
                    region.terrain()[&actor.position()],
                    project_rl::content::RegionTerrain::ShallowWater
                );
                assert!(!reserved.contains(&actor.position()));
                assert!(passages.iter().all(|at| at.x.abs_diff(actor.position().x)
                    + at.y.abs_diff(actor.position().y)
                    >= 20));
                assert!(actor.equipped_weapon().is_none() && actor.electronic_system().is_none());
                assert!(
                    actor
                        .body_profile()
                        .unwrap()
                        .displacement_profile()
                        .unwrap()
                        .is_fixed()
                );
                assert_eq!(
                    actor.ai().unwrap().behavior,
                    project_rl::ai::AiBehavior::TelegraphedGrasper
                );
            }
        }
        assert!(total >= 16, "too few wet habitat encounters: {total}");
        let previous = regions.without_cave_anemone_metadata();
        let previous = previous.get(world.id()).unwrap();
        assert!(
            previous
                .biome(&"core:research".parse().unwrap())
                .unwrap()
                .fauna()
                .is_none()
        );
        for biome in [
            "core:human_habitat",
            "core:surface_wilds",
            "core:maintenance",
            "core:production",
            "core:security",
        ] {
            assert_eq!(
                world.biome(&biome.parse().unwrap()),
                previous.biome(&biome.parse().unwrap())
            );
        }
    }

    #[test]
    fn cave_anemone_feedback_uses_short_french_text_and_removes_the_slow_badge() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.prepare_cave_anemone_diagnostic(true).unwrap();
        app.game.process_player_command(GameCommand::Wait);
        app.capture_events_at(Some(0.0));
        assert!(
            app.log
                .iter()
                .any(|line| line == "Vous n'êtes plus entravé.")
        );
        assert!(
            app.log
                .iter()
                .any(|line| line == "Vous résistez brièvement aux entraves.")
        );
        assert!(!app.log.iter().any(|line| line.contains("LOCOMOTION")));
        assert!(
            !app.effect_badges_at(app.game.player_position().unwrap())
                .contains(&TerminalEffectBadge::Slowed)
        );
    }
}
