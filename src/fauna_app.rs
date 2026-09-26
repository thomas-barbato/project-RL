//! Surface fauna integration and isolated native-render fixture.
use super::*;
use crate::test_sector::Decor;
use project_rl::content::RegionTerrain;

impl AsciiApp {
    pub(super) fn populate_surface_fauna(&mut self) -> Result<(), String> {
        let profile = self
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .and_then(|world| world.biome(&"core:human_habitat".parse().unwrap()))
            .and_then(|biome| biome.fauna())
            .ok_or("Surface fauna profile missing")?;
        let terrain = self
            .terminal
            .decor
            .cells
            .iter()
            .filter_map(|(at, decor)| {
                Some((
                    *at,
                    match decor {
                        Decor::Gravel => RegionTerrain::Gravel,
                        Decor::Grass => RegionTerrain::Grass,
                        Decor::Scrub => RegionTerrain::Scrub,
                        Decor::Mud => RegionTerrain::Mud,
                        Decor::RuinFloor => RegionTerrain::RuinFloor,
                        _ => return None,
                    },
                ))
            })
            .collect();
        let reserved = self
            .game
            .actors()
            .iter()
            .map(|(_, actor)| actor.position())
            .chain(
                self.game
                    .ground_items()
                    .iter()
                    .map(|(_, item)| item.position()),
            )
            .collect();
        let passages = TestSector::EXPANDED_REGIONAL_PASSAGES
            .iter()
            .map(|(_, at)| *at)
            .chain([
                TestSector::GATE,
                TestSector::EXPANDED_EXPEDITION_PASSAGE,
                TestSector::RECYCLING_START,
            ])
            .collect::<Vec<_>>();
        let fauna = project_rl::world::generation::generate_regional_fauna(
            self.game.map(),
            &terrain,
            &passages,
            &reserved,
            profile,
            self.seed ^ 0x4855_425f_4641_554e,
        )?;
        for actor in fauna {
            self.game
                .spawn_actor(actor)
                .map_err(|error| error.to_string())?;
        }
        self.game.drain_events();
        Ok(())
    }

    #[cfg(any(test, debug_assertions))]
    pub(super) fn prepare_fauna_diagnostic(&mut self, sheltered: bool) -> Result<(), String> {
        let map = project_rl::world::Map::from_ascii(
            "#################\n#...............#\n#...............#\n#...............#\n#...............#\n#...............#\n#...............#\n#################"
        ).map_err(|error| error.to_string())?;
        let mut game =
            GameState::new_with_rules(map, GridPos::new(5, 4), INITIAL_SEED, self.rules.clone())
                .map_err(|error| error.to_string())?;
        let fauna = self
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap()
            .biome(&"core:human_habitat".parse().unwrap())
            .unwrap()
            .fauna()
            .unwrap();
        let mut selected = None;
        for (family, at) in
            fauna
                .families
                .iter()
                .zip([GridPos::new(9, 3), GridPos::new(9, 5), GridPos::new(5, 3)])
        {
            let species = family
                .species
                .iter()
                .find(|species| species.id.as_str() != "core:rubble_nibbler")
                .unwrap();
            let rule = &species.population;
            let mut actor = Actor::new(at, rule.maximum_integrity())
                .unwrap()
                .with_ai(rule.ai())
                .with_attack(rule.attack())
                .with_player_relation(rule.player_relation())
                .with_tags([
                    species.id.clone(),
                    family.id.clone(),
                    format!("core:fauna_level_{}", species.level)
                        .parse()
                        .unwrap(),
                ]);
            if sheltered && species.id.as_str() == "core:moss_grazer" {
                actor = actor.with_evasion_disabled();
            }
            if let Some(body) = rule.body_profile() {
                actor = actor.with_body_profile(body);
            }
            if let Some(attributes) = rule.primary_attributes() {
                actor = actor.with_primary_attributes(attributes);
            }
            let id = game.spawn_actor(actor).map_err(|error| error.to_string())?;
            if species.id.as_str() == "core:moss_grazer" {
                selected = Some(id);
            }
        }
        if sheltered {
            let target = selected.ok_or("Grazer missing")?;
            if game.process_player_command(GameCommand::Attack { slot: 0, target })
                != CommandOutcome::Applied
                || !matches!(
                    game.actors().get(target).map(Actor::ai_state),
                    Some(project_rl::ai::AiState::Sheltered { .. })
                )
            {
                return Err("Grazer failed to shelter after attack".into());
            }
        }
        self.terminal = TerminalView::new(
            crate::test_sector::SectorDecor::default(),
            game.map(),
            game.player_visibility(),
        );
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.selected_target = selected;
        self.intro_city_reached = true;
        self.log.clear();
        self.push_log(
            "FAUNE DE SURFACE · b : Mordeur · f : Fouisseur pâle · g : Dos-rond".to_owned(),
        );
        Ok(())
    }

    #[cfg(any(test, debug_assertions))]
    pub(super) fn prepare_bone_breaker_diagnostic(
        &mut self,
        recovering: bool,
    ) -> Result<(), String> {
        let map = project_rl::world::Map::from_ascii(
            "#############\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#############"
        ).map_err(|error| error.to_string())?;
        let mut game =
            GameState::new_with_rules(map, GridPos::new(5, 3), INITIAL_SEED, self.rules.clone())
                .map_err(|error| error.to_string())?;
        let species = self
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap()
            .biome(&"core:surface_wilds".parse().unwrap())
            .unwrap()
            .fauna()
            .unwrap()
            .families
            .iter()
            .flat_map(|family| &family.species)
            .find(|species| species.id.as_str() == "core:bone_breaker")
            .unwrap();
        let rule = &species.population;
        let mut actor = Actor::new(GridPos::new(6, 3), rule.maximum_integrity())
            .unwrap()
            .with_ai(rule.ai())
            .with_attack(rule.attack())
            .with_player_relation(rule.player_relation())
            .with_tags([
                species.id.clone(),
                format!("core:fauna_level_{}", species.level)
                    .parse()
                    .unwrap(),
            ]);
        if let Some(body) = rule.body_profile() {
            actor = actor.with_body_profile(body);
        }
        if let Some(attributes) = rule.primary_attributes() {
            actor = actor.with_primary_attributes(attributes);
        }
        let target = game.spawn_actor(actor).map_err(|error| error.to_string())?;
        game.process_player_command(GameCommand::Wait);
        if !matches!(
            game.actors().get(target).unwrap().ai_state(),
            project_rl::ai::AiState::Aiming { .. }
        ) {
            return Err("Bone-breaker did not telegraph its bite".into());
        }
        self.terminal = TerminalView::new(
            crate::test_sector::SectorDecor::default(),
            game.map(),
            game.player_visibility(),
        );
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.selected_target = Some(target);
        self.intro_city_reached = true;
        self.log.clear();
        self.floating_messages.clear();
        self.visual_cues.clear_world();
        self.capture_events();
        if recovering {
            // Capture each action while its positions are current, as the live client does.
            self.game
                .process_player_command(GameCommand::Move(Direction::North));
            if self
                .game
                .actors()
                .get(target)
                .unwrap()
                .recovery_remaining()
                .is_none()
            {
                return Err("Bone-breaker did not recover after the evaded bite".into());
            }
            self.terminal
                .observe(self.game.map(), self.game.player_visibility());
            self.capture_events();
            self.push_log("BRISE-OS · Morsure évitée ; il récupère sans se déplacer.".to_owned());
        }
        Ok(())
    }

    #[cfg(any(test, debug_assertions))]
    pub(super) fn prepare_forager_diagnostic(&mut self, cornered: bool) -> Result<(), String> {
        let map = project_rl::world::Map::from_ascii(if cornered {
            "#########\n#.......#\n#.......#\n#.......#\n####.####\n#########"
        } else {
            "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########"
        })
        .map_err(|error| error.to_string())?;
        let mut game =
            GameState::new_with_rules(map, GridPos::new(4, 3), INITIAL_SEED, self.rules.clone())
                .map_err(|error| error.to_string())?;
        let species = self
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap()
            .biome(&"core:human_habitat".parse().unwrap())
            .unwrap()
            .fauna()
            .unwrap()
            .families
            .iter()
            .flat_map(|family| &family.species)
            .find(|species| species.id.as_str() == "core:rubble_nibbler")
            .unwrap();
        let rule = &species.population;
        let mut actor = Actor::new(GridPos::new(4, 4), rule.maximum_integrity())
            .unwrap()
            .with_ai(rule.ai())
            .with_attack(rule.attack())
            .with_player_relation(rule.player_relation())
            .with_tags([
                species.id.clone(),
                format!("core:fauna_level_{}", species.level)
                    .parse()
                    .unwrap(),
            ]);
        if let Some(body) = rule.body_profile() {
            actor = actor.with_body_profile(body);
        }
        if let Some(attributes) = rule.primary_attributes() {
            actor = actor.with_primary_attributes(attributes);
        }
        let target = game.spawn_actor(actor).map_err(|error| error.to_string())?;
        game.process_player_command(GameCommand::Wait);
        let expected = if cornered {
            project_rl::ai::AiState::Cornered
        } else {
            project_rl::ai::AiState::Fleeing
        };
        if game.actors().get(target).unwrap().ai_state() != expected {
            return Err(format!(
                "Forager diagnostic expected {expected:?}, got {:?}",
                game.actors().get(target).unwrap().ai_state()
            ));
        }
        self.terminal = TerminalView::new(
            crate::test_sector::SectorDecor::default(),
            game.map(),
            game.player_visibility(),
        );
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.selected_target = Some(target);
        self.intro_city_reached = true;
        self.log.clear();
        self.push_log(
            if cornered {
                "GRIGNOTEUR · Acculé : laisser une issue lui permet de fuir."
            } else {
                "GRIGNOTEUR · Évite le contact ; aucune poursuite ni hostilité permanente."
            }
            .to_owned(),
        );
        Ok(())
    }
}
