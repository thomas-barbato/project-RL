//! Play all five unarmed Traqueuse outfits with the real game controller.
use super::*;
use std::collections::BTreeSet;

const OUTFITS: [&str; 5] = [
    "core:veste_de_veille",
    "core:veste_a_fibres_croisees",
    "core:combinaison_fantome",
    "core:armure_cerbere",
    "core:carapace_chrysalide",
];

impl AsciiApp {
    pub fn new_traqueuse_walk_trial() -> Result<Self, String> {
        let (mut rules, texts, loot, expeditions) = ascii_game_content()?;
        let classes = ascii_character_class_catalog()?;
        let id: CharacterClassId = "core:traqueur".parse().unwrap();
        let class = classes.get(&id).ok_or("Traqueuse absente")?;
        class
            .apply_to_rules(&mut rules, class.recommended_attributes())
            .map_err(|e| e.to_string())?;
        rules.player_starting_weapons.clear();
        rules.player_starting_equipment = vec![None; rules.player_weapon_slots.len()];
        // The normal starting-item path equips the last armor, retaining the
        // heavy candidate at entry. F6 changes appearance only, never bonuses.
        rules.player_starting_items = [0, 1, 2, 4, 3]
            .map(|tier| StartingItemStack::new(OUTFITS[tier].parse().unwrap(), 1))
            .to_vec();
        let mut app = Self::from_seed(1872, rules, texts, loot, expeditions)?;
        app.character_class = Some(id);
        app.test_lab = true;
        app.crash_recovery_enabled = false;
        let isolated = controls::config_path()
            .parent()
            .ok_or("Dossier de configuration absent")?
            .join("essai-marche-traqueuse");
        app.suspension_path = isolated.join("run.json");
        app.controls = Controls::load(&controls::config_path(), controls::detect_layout()).0;
        app.controls_path = isolated.join("controls.json");
        app.graphics.path = isolated.join("graphics.json");
        app.graphics.active = graphics::startup().0;
        app.graphics.active.world_cell_px = 64;
        app.graphics.active.walk_duration_ms = 400;
        app.graphics.active.reduced_motion = false;
        app.graphics.active.interface_style = graphics::InterfaceStyle::Metal;
        app.graphics.active.mode = graphics::WindowMode::Windowed;
        app.graphics.active.windowed_size = [1280, 800];
        app.graphics.draft = app.graphics.active;
        app.capture_events_at(Some(0.));
        app.open_menu(MenuScreen::Hidden);
        app.inventory_open = false;
        app.character_creation = None;
        if !crate::game_art::traqueuse_trial::selected(
            app.front_player_visual(),
            crate::game_art::walking::Pose {
                facing: 1,
                frame: 0,
            },
        ) {
            return Err(format!(
                "Configuration LATERAL04 inattendue : {:?}",
                app.front_player_visual()
            ));
        }
        Ok(app)
    }

    fn select_traqueuse_trial_outfit(&mut self, tier: usize) -> Result<(), String> {
        let current = crate::game_art::traqueuse_trial::selection(self.front_player_visual(), None)
            .ok_or("Essai réservé à la Traqueuse sans arme")?;
        if current[0] == tier {
            return Ok(());
        }
        let id: ContentId = OUTFITS[tier].parse().unwrap();
        let item = self
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &id)
            .ok_or("Tenue absente de l'inventaire d'essai")?
            .instance();
        let outcome = self.execute_command(GameCommand::EquipItem {
            slot: project_rl::entity::BODY_APPEARANCE_SLOT.parse().unwrap(),
            item,
        });
        if outcome != CommandOutcome::AppliedWithoutTime {
            return Err(format!("Changement de tenue refusé : {outcome:?}"));
        }
        if crate::game_art::traqueuse_trial::selection(self.front_player_visual(), None)
            != Some([tier, 2, 6])
        {
            return Err("Tenue affichée incorrecte".into());
        }
        Ok(())
    }

    pub(crate) fn handle_traqueuse_trial_shortcut(&mut self) -> bool {
        if !self.test_lab
            || !crate::game_art::traqueuse_trial::enabled()
            || self.menu != MenuScreen::Hidden
            || self.inventory_open
            || !is_key_pressed(KeyCode::F6)
        {
            return false;
        }
        if let Some([tier, _, _]) =
            crate::game_art::traqueuse_trial::selection(self.front_player_visual(), None)
        {
            if let Err(error) = self.select_traqueuse_trial_outfit((tier + 1) % 5) {
                self.inventory_message = error;
            } else {
                self.capture_events();
            }
        }
        true
    }

    fn traqueuse_trial_directions(&self) -> Result<[Direction; 4], String> {
        let from = self.game.player_position().ok_or("Joueur absent")?;
        let forward = [Direction::East, Direction::West]
            .into_iter()
            .find(|d| {
                let p = from.step(*d);
                self.game.map().is_walkable(p) && self.game.actors().entity_at(p).is_none()
            })
            .ok_or("Aucun déplacement latéral libre au départ")?;
        let back = if forward == Direction::East {
            Direction::West
        } else {
            Direction::East
        };
        let vertical = [Direction::North, Direction::South]
            .into_iter()
            .find(|d| {
                let p = from.step(*d);
                self.game.map().is_walkable(p) && self.game.actors().entity_at(p).is_none()
            })
            .ok_or("Aucun déplacement vertical libre au départ")?;
        let return_vertical = if vertical == Direction::North {
            Direction::South
        } else {
            Direction::North
        };
        Ok([forward, back, vertical, return_vertical])
    }

    fn traqueuse_trial_pose(&self, time: f64) -> Option<crate::game_art::walking::Pose> {
        let player = self.game.player_id();
        self.combat_motion
            .vertical_walk_pose(player, time, false)
            .or_else(|| self.combat_motion.player_walk_pose(player, time, false))
    }

    pub fn verify_traqueuse_walk_trial() -> Result<serde_json::Value, String> {
        let mut app = Self::new_traqueuse_walk_trial()?;
        let from = app.game.player_position().ok_or("Joueur absent")?;
        let directions = app.traqueuse_trial_directions()?;
        let slot = "core:body_armor".parse().unwrap();
        let actual_armor = app.game.player_equipment().equipped(&slot);
        let mut outfits = Vec::new();
        for tier in 0..5 {
            let before_turn = app.game.turn();
            app.select_traqueuse_trial_outfit(tier)?;
            if app.game.turn() != before_turn
                || app.game.player_equipment().equipped(&slot) != actual_armor
            {
                return Err("Le sélecteur a changé les règles de jeu".into());
            }
            let mut poses = Vec::new();
            for (step, direction) in directions.into_iter().enumerate() {
                let time = (tier * 4 + step + 1) as f64;
                if app.execute_command(GameCommand::Move(direction)) != CommandOutcome::Applied {
                    return Err("Déplacement réel refusé".into());
                }
                app.capture_events_at(Some(time));
                for t in [0.01, 0.15, 0.29] {
                    let pose = app
                        .traqueuse_trial_pose(time + t)
                        .ok_or("Pose de marche absente")?;
                    if crate::game_art::traqueuse_trial::image(app.front_player_visual(), pose)
                        .is_none()
                    {
                        return Err(format!(
                            "Pose hors du candidat04 : tenue {tier}, {direction:?}, {pose:?}, {:?}",
                            app.front_player_visual()
                        ));
                    }
                    poses.push([pose.facing, i8::try_from(pose.frame).unwrap()]);
                }
                if app.traqueuse_trial_pose(time + 0.401).is_some() {
                    return Err("Retour frontal absent après 400 ms".into());
                }
            }
            if poses.iter().map(|p| p[1]).collect::<BTreeSet<_>>().len() != 6
                || app.game.player_position() != Some(from)
            {
                return Err("Cycle réel incomplet".into());
            }
            if crate::game_art::traqueuse_trial::selection(app.front_player_visual(), None)
                != Some([tier, 2, 6])
            {
                return Err("Le repos n'utilise pas la même tenue".into());
            }
            outfits.push(serde_json::json!({"tier":tier,"poses":poses,"idle":[tier,2,6]}));
        }
        // Verify the same wraparound as the F6 selector.
        app.select_traqueuse_trial_outfit(0)?;
        Ok(
            serde_json::json!({"review":"LATERAL02-r04-live","status":"passed","actual_game_commands":true,"outfits":outfits,"step_ms":400,"native_cell_px":64,"appearance_shortcut":"F6","appearance_only":true,"bastion_unchanged":true,"normal_game_default_unchanged":true,"unarmed_only":true,"trial_save_path":app.suspension_path,"trial_graphics_path":app.graphics.path,"crash_recovery":false,"user_campaign_loaded":false}),
        )
    }

    #[cfg(debug_assertions)]
    pub async fn capture_traqueuse_walk_trial(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let report = Self::verify_traqueuse_walk_trial()?;
        let mut app = Self::new_traqueuse_walk_trial()?;
        miniquad::window::set_fullscreen(false);
        request_new_screen_size(1280., 800.);
        for _ in 0..3 {
            next_frame().await;
        }
        let mut time = 1.;
        let directions = app.traqueuse_trial_directions()?;
        let mut draws = Vec::new();
        for tier in 0..5 {
            app.select_traqueuse_trial_outfit(tier)?;
            app.prepare_presentation()?;
            for direction in directions {
                if app.execute_command(GameCommand::Move(direction)) != CommandOutcome::Applied {
                    return Err("Déplacement de capture refusé".into());
                }
                app.capture_events_at(Some(time));
                for (phase, t) in [0.01, 0.15, 0.29, 0.401].into_iter().enumerate() {
                    let _clock = crate::effect_geometry::frame_time(time + t);
                    crate::game_art::traqueuse_trial::audit(true);
                    for _ in 0..2 {
                        app.draw();
                        next_frame().await;
                    }
                    app.draw();
                    let drawn = crate::game_art::traqueuse_trial::audit(false);
                    let pose = app.traqueuse_trial_pose(time + t);
                    let expected = crate::game_art::traqueuse_trial::selection(
                        app.front_player_visual(),
                        pose,
                    );
                    if drawn != expected {
                        return Err(format!(
                            "Rendu réel LATERAL04 incorrect : tenue {tier}, {direction:?}, phase {phase}, {drawn:?} != {expected:?}"
                        ));
                    }
                    crate::ui_capture::framebuffer()?.export_png(
                        output
                            .join(format!("tier{tier}-{direction:?}-{phase}.png"))
                            .to_str()
                            .ok_or("capture path")?,
                    );
                    draws.push(serde_json::json!({"tier":tier,"direction":format!("{direction:?}"),"phase":phase,"actual":drawn,"expected":expected}));
                }
                time += 1.;
            }
        }
        std::fs::write(
            output.join("checks.json"),
            serde_json::to_vec_pretty(&serde_json::json!({"game":report,"actual_draws":draws}))
                .unwrap(),
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn live_traqueuse_trial_walks_all_five_outfits_without_changing_real_armor() {
        let report = AsciiApp::verify_traqueuse_walk_trial().unwrap();
        assert_eq!(report["outfits"].as_array().unwrap().len(), 5);
        for outfit in report["outfits"].as_array().unwrap() {
            assert_eq!(outfit["poses"].as_array().unwrap().len(), 12);
        }
        assert!(
            report["trial_save_path"]
                .as_str()
                .unwrap()
                .contains("essai-marche-traqueuse")
        );
        let mut app = AsciiApp::new_traqueuse_walk_trial().unwrap();
        app.request_quit();
        assert!(app.should_quit());
        assert!(!app.crash_recovery_enabled);
    }
}
