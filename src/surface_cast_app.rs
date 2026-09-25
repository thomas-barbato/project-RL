//! Isolated native-render diagnostics for the surface encounter roles.
use super::*;

impl AsciiApp {
    #[cfg(any(test, debug_assertions))]
    pub(super) fn prepare_surface_enemies_diagnostic(&mut self) -> Result<(), String> {
        let map = project_rl::world::Map::from_ascii(
            "#################\n#...............#\n#...............#\n#...............#\n#...............#\n#...............#\n#...............#\n#################",
        ).map_err(|e| e.to_string())?;
        let mut game =
            GameState::new_with_rules(map, GridPos::new(5, 4), INITIAL_SEED, self.rules.clone())
                .map_err(|e| e.to_string())?;
        let world = self
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .ok_or("Regional world missing")?;
        let biome = world
            .biome(&"core:human_habitat".parse().unwrap())
            .ok_or("Surface biome missing")?;
        let mut shooter = None;
        for (is_shooter, at) in [(true, GridPos::new(9, 4)), (false, GridPos::new(9, 3))] {
            let rule = biome
                .encounters()
                .rules()
                .iter()
                .find(|rule| match rule.ai().behavior {
                    project_rl::ai::AiBehavior::TelegraphedShooter => is_shooter,
                    project_rl::ai::AiBehavior::FieldMedic { .. } => !is_shooter,
                    _ => false,
                })
                .ok_or("Surface enemy profile missing")?;
            let mut actor = Actor::new(at, rule.maximum_integrity())
                .map_err(|e| e.to_string())?
                .with_ai(rule.ai())
                .with_attack(rule.attack())
                .with_player_relation(rule.player_relation());
            if let Some(body) = rule.body_profile() {
                actor = actor.with_body_profile(body);
            }
            if let Some(attributes) = rule.primary_attributes() {
                actor = actor.with_primary_attributes(attributes);
            }
            if let Some(system) = rule.electronic_system() {
                actor = actor.with_electronic_system(system);
            }
            let entity = game.spawn_actor(actor).map_err(|e| e.to_string())?;
            if is_shooter {
                shooter = Some(entity);
            }
        }
        game.process_player_command(GameCommand::Wait);
        let shooter = shooter.ok_or("Artilleur absent")?;
        if !matches!(
            game.actors().get(shooter).unwrap().ai_state(),
            project_rl::ai::AiState::Aiming { .. }
        ) {
            return Err("L'artilleur n'a pas annoncé son tir".into());
        }
        self.terminal = TerminalView::new(
            crate::test_sector::SectorDecor::default(),
            game.map(),
            game.player_visibility(),
        );
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.selected_target = Some(shooter);
        self.intro_city_reached = true;
        self.capture_events();
        Ok(())
    }
}
