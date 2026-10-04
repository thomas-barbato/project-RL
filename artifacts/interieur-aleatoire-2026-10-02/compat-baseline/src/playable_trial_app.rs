//! A suggested outing in the existing campaign, with an independent save slot.
//! No map, RNG, inventory or combat rule is changed by this presentation profile.
use super::*;

pub(super) const SAVE_NAME: &str = "essai-expedition.json";

impl AsciiApp {
    pub fn new_playable_trial() -> Result<Self, String> {
        Self::new_with_session_path(controls::config_path().with_file_name(SAVE_NAME))
    }

    pub(super) fn is_playable_trial(&self) -> bool {
        self.suspension_path.file_name() == Some(std::ffi::OsStr::new(SAVE_NAME))
    }

    pub(super) fn playable_trial_objective(&self) -> Option<&'static str> {
        if !self.is_playable_trial() || self.test_lab {
            return None;
        }
        let expedition = self
            .expeditions
            .get(&"core:starter_expedition".parse().ok()?)?;
        let current = &self.game.current_zone()?.id;
        if current == &expedition.destination.zone.id {
            return Some("ESSAI · Explorer le souterrain, puis rentrer en ville.");
        }
        if self
            .zone_views
            .contains_key(&expedition.destination.zone.id)
        {
            let in_town = current == &expedition.hub.id
                && self
                    .game
                    .player_position()
                    .is_some_and(|p| (1..=62).contains(&p.x) && (1..=44).contains(&p.y));
            return Some(if in_town {
                "ESSAI · Retour en ville effectué. Tu peux poursuivre librement."
            } else {
                "ESSAI · Rentrer en ville avec ce que tu as trouvé."
            });
        }
        Some("ESSAI · Explorer les friches et trouver l'accès souterrain au sud-est.")
    }

    pub(super) fn playable_trial_navigation_signal_summary(&self) -> Option<String> {
        let expedition = self
            .expeditions
            .get(&"core:starter_expedition".parse().ok()?)?;
        let current = &self.game.current_zone()?.id;
        let observer = self.game.player_position()?;
        let (label, target) = if current == &expedition.destination.zone.id {
            (
                "RETOUR SURFACE",
                self.game.next_visited_passage_towards(&expedition.hub.id)?,
            )
        } else if current == &expedition.hub.id {
            if self
                .zone_views
                .contains_key(&expedition.destination.zone.id)
            {
                if (1..=62).contains(&observer.x) && (1..=44).contains(&observer.y) {
                    return None;
                }
                ("RETOUR EN VILLE", TestSector::GATE)
            } else {
                let passage = expedition.hub_passage_for_expanded_world(
                    self.generation_version >= EXPANDED_WORLD_GENERATION_VERSION,
                );
                if self.game.passage(passage).is_none()
                    && self.game.unmaterialized_passage_destination(passage)
                        != Some(&expedition.destination.zone.id)
                {
                    return None;
                }
                ("ACCÈS DU SECTEUR INDUSTRIEL", passage)
            }
        } else {
            (
                "RETOUR VERS LES FRICHES",
                self.game.next_visited_passage_towards(&expedition.hub.id)?,
            )
        };
        Some(crate::terminal_view::directional_signal_summary(
            label,
            observer,
            target,
            grid_distance(observer, target),
            1,
        ))
    }

    #[cfg(any(test, debug_assertions))]
    pub(super) fn walk_playable_trial_outbound(&mut self) -> Result<(), String> {
        if !self.intro_city_reached {
            self.walk_fixture_to(GridPos::new(27, 23))?;
        }
        self.walk_fixture_to_unchecked_near(
            TestSector::EXPANDED_EXPEDITION_PASSAGE.step(Direction::West),
            true,
            0,
            true,
        )
    }

    #[cfg(any(test, debug_assertions))]
    fn walk_playable_trial_return(&mut self) -> Result<(), String> {
        // Follow the generated circulation. All moves, doors, fights and
        // repairs use the normal commands and the finite carried supplies.
        self.walk_fixture_to_unchecked_near(GridPos::new(27, 23), true, 0, true)
    }

    #[cfg(debug_assertions)]
    pub(super) fn prepare_playable_trial_capture(&mut self, scene: &str) -> Result<(), String> {
        match scene {
            "playtest-main" => self.open_menu(MenuScreen::Main),
            "playtest-creation" => self.begin_character_creation(false)?,
            _ => {
                self.begin_character_creation(false)?;
                let creation = self
                    .character_creation
                    .clone()
                    .ok_or("Création de diagnostic absente")?;
                self.rebuild_run_with_character_class(&creation)?;
                self.open_menu(MenuScreen::Hidden);
                if scene == "playtest-town" {
                    self.walk_fixture_to(GridPos::new(27, 23))?;
                } else if scene == "playtest-outside" {
                    self.walk_fixture_to(GridPos::new(65, 21))?;
                } else {
                    self.walk_expedition_fixture(u8::from(scene == "playtest-return"))?;
                    if scene == "playtest-return" {
                        self.walk_playable_trial_return()?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app() -> AsciiApp {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.capture_events_at(Some(0.0));
        app
    }

    #[test]
    fn playable_trial_profile_preserves_the_complete_campaign_and_save_format() {
        let mut app = app();
        let before = serde_json::to_value(app.suspension().unwrap()).unwrap();
        let normal_path = app.suspension_path.clone();
        app.suspension_path = normal_path.with_file_name(SAVE_NAME);
        assert_ne!(normal_path, app.suspension_path);
        assert_ne!(
            normal_path.with_extension("lock"),
            app.suspension_path.with_extension("lock")
        );
        assert_ne!(
            AsciiApp::crash_recovery_paths_for(&normal_path),
            app.crash_recovery_paths()
        );
        app.open_menu(MenuScreen::Main);
        assert_eq!(app.menu_labels()[1], "Nouvel essai d'expédition");
        assert!(app.playable_trial_objective().is_some());
        assert_eq!(
            serde_json::to_value(app.suspension().unwrap()).unwrap(),
            before
        );
    }

    #[test]
    fn playable_trial_survives_verified_resume_and_restart() {
        let mut app = app();
        let folder = std::env::temp_dir().join(format!(
            "project-rl-trial-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        app.suspension_path = folder.join(SAVE_NAME);
        let saved = app.suspension().unwrap();
        saved.write(&app.suspension_path).unwrap();
        app.resume_run().unwrap();
        assert!(app.is_playable_trial());
        assert!(!app.suspension_path.exists());
        assert_eq!(suspension::fingerprint(&app.game), saved.state);
        app.restart_current_profile();
        assert!(app.is_playable_trial());
        assert_eq!(app.game.turn(), 0);
        assert_eq!(app.seed, INITIAL_SEED.wrapping_add(1));
        std::fs::remove_dir(folder).unwrap();
    }

    #[test]
    fn playable_trial_keeps_normal_character_creation() {
        let mut app = app();
        app.suspension_path = app.suspension_path.with_file_name(SAVE_NAME);
        app.begin_character_creation(false).unwrap();
        let creation = app.character_creation.clone().unwrap();
        let class_id = app.character_classes.iter().next().unwrap().0.clone();
        app.rebuild_run_with_character_class(&creation).unwrap();
        assert!(app.is_playable_trial());
        assert_eq!(app.character_class, Some(class_id));
        assert_eq!(
            app.game.player_primary_attributes(),
            Some(creation.attributes)
        );
        assert_eq!(app.game.turn(), 0);
        assert!(app.character_creation.is_none());
    }

    #[test]
    fn playable_trial_guidance_uses_real_travel_and_keeps_returned_loot() {
        let mut app = app();
        app.suspension_path = app.suspension_path.with_file_name(SAVE_NAME);
        // Use the normal starter protocol, as an actual new trial does, rather
        // than the unclassified engine fixture used by presentation tests.
        app.begin_character_creation(false).unwrap();
        let creation = app.character_creation.clone().unwrap();
        app.rebuild_run_with_character_class(&creation).unwrap();
        app.walk_fixture_to(GridPos::new(27, 23)).unwrap();
        assert!(
            app.navigation_signal_summary()
                .unwrap()
                .starts_with("ACCÈS DU SECTEUR INDUSTRIEL · SUD-EST")
        );
        app.walk_expedition_fixture(0).unwrap();
        assert!(
            app.playable_trial_objective()
                .unwrap()
                .contains("Explorer le souterrain")
        );
        assert!(
            app.navigation_signal_summary()
                .unwrap()
                .starts_with("RETOUR SURFACE")
        );
        let inventory = suspension::fingerprint(&app.game.player_inventory());
        let hub = app
            .expeditions
            .get(&"core:starter_expedition".parse().unwrap())
            .unwrap()
            .hub
            .id
            .clone();
        let entrance = app.game.next_visited_passage_towards(&hub).unwrap();
        app.walk_fixture_to(entrance).unwrap();
        assert_eq!(
            app.execute_command(GameCommand::Interact { target: entrance }),
            CommandOutcome::Applied
        );
        app.capture_events_at(Some(0.0));
        assert!(
            app.playable_trial_objective()
                .unwrap()
                .contains("Rentrer en ville")
        );
        assert!(
            app.navigation_signal_summary()
                .unwrap()
                .starts_with("RETOUR EN VILLE")
        );
        assert_eq!(
            suspension::fingerprint(&app.game.player_inventory()),
            inventory
        );
        // The labyrinth changes sight lines and the automatic shortest route.
        // Return using only repair patches carried or actually found;
        // this tests guidance/persistence without imposing a no-healing run.
        app.walk_playable_trial_return().unwrap();
        assert!(
            app.playable_trial_objective()
                .unwrap()
                .contains("Retour en ville effectué")
        );
        assert_eq!(app.navigation_signal_summary(), None);
        let saved = app.suspension().unwrap();
        let mut restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        restored.suspension_path = app.suspension_path.clone();
        assert_eq!(
            restored.playable_trial_objective(),
            app.playable_trial_objective()
        );
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }
}
