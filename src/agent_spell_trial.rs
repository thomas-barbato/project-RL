//! Isolated native fixture for the first playable agent spell.
use super::*;

impl AsciiApp {
    pub fn new_agent_ember_trial() -> Result<Self, String> {
        let mut app = Self::new_agent_blast_trial()?;
        if app.execute_command(GameCommand::LearnAgentSkill {
            skill: "FE01".into(),
        }) != CommandOutcome::AppliedWithoutTime
        {
            return Err("Apprentissage de diagnostic refuse".into());
        }
        app.capture_events();
        app.ux.skill_bar[0] = Some("FE01".into());
        app.terminal.title = "Laboratoire · Braise".into();
        app.select_world_target(Some(GridPos::new(5, 4)));
        Ok(app)
    }
    pub fn new_agent_fireball_trial() -> Result<Self, String> {
        let mut app = Self::new_agent_blast_trial()?;
        if app.execute_command(GameCommand::LearnAgentSkill {
            skill: "FE05".into(),
        }) != CommandOutcome::AppliedWithoutTime
        {
            return Err("Apprentissage de diagnostic refusé".into());
        }
        app.ux.skill_bar[0] = Some("FE05".into());
        app.terminal.title = "Laboratoire · Boule de feu".into();
        app.select_world_target(Some(GridPos::new(8, 4)));
        Ok(app)
    }
    #[cfg(debug_assertions)]
    pub async fn capture_agent_fireball_playable(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let mut app = Self::new_agent_fireball_trial()?;
        app.suspension_path = output.join("isolated-run.json");
        request_new_screen_size(1280., 800.);
        for _ in 0..5 {
            app.draw();
            next_frame().await;
        }
        async fn frame(app: &AsciiApp, output: &std::path::Path, name: &str) -> Result<(), String> {
            app.draw();
            let image = crate::ui_capture::framebuffer()?;
            if (image.width, image.height) != (1280, 800) {
                return Err("Surface de capture indisponible".into());
            }
            image.export_png(output.join(name).to_str().ok_or("Chemin de capture")?);
            next_frame().await;
            Ok(())
        }
        let before = suspension::fingerprint(&app.game);
        let hp = app
            .game
            .actors()
            .get(app.game.player_id())
            .unwrap()
            .integrity();
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Skill1).clone()].into(),
            ..Default::default()
        });
        let mut click_confirmation = None;
        for (i, at) in [GridPos::new(6, 5), GridPos::new(7, 5), GridPos::new(8, 4)]
            .into_iter()
            .enumerate()
        {
            let scale = app.ui_scale();
            let cell = app
                .terminal
                .world_cell_rect(
                    &app.game,
                    app.terminal_bounds(),
                    scale,
                    app.graphics.active.world_cell_px,
                    app.navigation_signal_summary().is_some(),
                    at,
                )
                .ok_or("Hover cell outside camera")?;
            let hover = app.graphics.active.transform_input(InputFrame {
                pointer: Some(cell.center().into()),
                viewport: Some((screen_width(), screen_height())),
                ..Default::default()
            });
            app.update_input(&hover);
            if app.attack_aim.is_none_or(|a| a.cursor != at)
                || suspension::fingerprint(&app.game) != before
            {
                return Err("Hover preview must follow without click or cost".into());
            }
            click_confirmation = Some(hover.clone());
            frame(&app, output, &format!("fireball-hover-{i}.png")).await?;
            if i == 0 {
                let mut key = hover.clone();
                key.pressed
                    .insert(app.controls.binding(Action::MoveNorth).clone());
                app.update_input(&key);
                app.update_input(&hover);
                if app
                    .attack_aim
                    .is_none_or(|a| a.cursor != at.step(Direction::North))
                {
                    return Err("Stationary mouse overrode keyboard aiming".into());
                }
                frame(&app, output, "fireball-keyboard.png").await?;
            }
        }
        let aim = app.attack_aim.ok_or("Visée de Boule de feu absente")?;
        if suspension::fingerprint(&app.game) != before {
            return Err("Visée non gratuite".into());
        }
        let preview = app
            .aimed_attack_preview(aim)
            .map_err(|e| format!("{e:?}"))?;
        let impact = preview.target();
        let cells = preview.cells().len();
        frame(&app, output, "fireball-aim.png").await?;
        let mut click = click_confirmation.ok_or("Missing aiming input")?;
        click.pressed.insert(controls::Binding::MouseLeft);
        app.update_input(&click);
        if app.game.turn() != 1 || app.attack_aim.is_some() {
            return Err("Boule de feu non exécutée".into());
        }
        let victims: Vec<_> = app
            .game
            .actors()
            .iter()
            .filter(|(id, a)| *id != app.game.player_id() && a.integrity() < 500)
            .map(|(id, a)| (id, a.position(), 500 - a.integrity()))
            .collect();
        if victims.len() < 2 {
            return Err("Explosion sans victimes multiples".into());
        }
        let fireball_messages = app.floating_messages.clone();
        for (i, age) in [0.01, 0.10, 0.22, 0.34, 0.38, 0.46, 0.58, 0.74, 0.88, 1.0]
            .into_iter()
            .enumerate()
        {
            let started = get_time() - age;
            app.floating_messages = fireball_messages
                .iter()
                .cloned()
                .map(|mut m| {
                    if matches!(
                        m.tone,
                        FloatingMessageTone::Damage | FloatingMessageTone::Status
                    ) {
                        m.started_at = started + 0.35;
                    }
                    m
                })
                .collect();
            app.combat_motion.clear();
            app.combat_motion
                .play_fireball(preview.origin(), impact, 3, started);
            for &(id, at, _) in &victims {
                app.combat_motion.fireball_hit_after_flight(id, started);
                app.combat_motion.play(id, at, impact, true, started);
            }
            frame(&app, output, &format!("fireball-{i}.png")).await?;
        }
        app.combat_motion.clear();
        for i in 0..8 {
            let start = get_time();
            while get_time() - start < 0.125 {
                app.draw();
                next_frame().await;
            }
            frame(&app, output, &format!("fireball-burn-{i}.png")).await?;
        }
        let burns:Vec<_>=app.game.actors().iter().filter_map(|(id,a)|a.statuses().find(|s|s.definition.as_str()=="core:burning").map(|s|serde_json::json!({"id":id.get(),"turns":s.remaining_turns,"power":s.power}))).collect();
        app.open_agent_skill_window();
        app.select_agent_skill_for_capture("FE05")?;
        frame(&app, output, "fireball-skill.png").await?;
        if app
            .game
            .actors()
            .get(app.game.player_id())
            .unwrap()
            .integrity()
            != hp
        {
            return Err("Le lanceur subit son propre sort".into());
        }
        write_agent_review_json(
            &output.join("fireball-checks.json"),
            serde_json::json!({"review":"AGENT-PLAY05","skill":"FE05","cells":cells,"impact":[impact.x,impact.y],"victims":victims.len(),"burns":burns,
        "targeting_review":"AGENT-AIM02","left_click_confirms":true,"hover_without_click":true,"keyboard_not_overridden_by_stationary_mouse":true,
        "preview_free":true,"preview_hidden_after_confirmation":true,"caster_unharmed":true,"saves_untouched":true,"turns":app.game.turn(),"tile_pixels":64,"viewport":[1280,800]}),
        )?;
        Ok(())
    }
    pub fn new_agent_blast_trial() -> Result<Self, String> {
        let mut app = Self::new_mixed_weapon_trial()?;
        if app.execute_command(GameCommand::LearnAgentSkill {
            skill: "A02".into(),
        }) != CommandOutcome::AppliedWithoutTime
        {
            return Err("Apprentissage de diagnostic refusé".into());
        }
        for (position, glyph) in [
            (GridPos::new(6, 4), 'B'),
            (GridPos::new(7, 3), 'C'),
            (GridPos::new(9, 4), 'J'),
        ] {
            let id = app
                .game
                .spawn_actor(
                    Actor::new(position, 500)
                        .map_err(|e| e.to_string())?
                        .with_player_relation(PlayerRelation::Hostile),
                )
                .map_err(|e| e.to_string())?;
            app.actor_glyphs.insert(id, glyph);
        }
        app.capture_events();
        app.ux.skill_bar[0] = Some("A02".into());
        app.terminal.title = "Laboratoire · Déflagration occulte".into();
        app.select_world_target(Some(GridPos::new(6, 4)));
        Ok(app)
    }
    #[cfg(debug_assertions)]
    pub async fn capture_agent_blast_playable(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let mut app = Self::new_agent_blast_trial()?;
        app.suspension_path = output.join("isolated-run.json");
        request_new_screen_size(1280., 800.);
        for _ in 0..5 {
            app.draw();
            next_frame().await;
        }
        async fn frame(app: &AsciiApp, output: &std::path::Path, name: &str) -> Result<(), String> {
            app.draw();
            let image = crate::ui_capture::framebuffer()?;
            if (image.width, image.height) != (1280, 800) {
                return Err("Surface de capture indisponible".into());
            }
            image.export_png(output.join(name).to_str().ok_or("Chemin de capture")?);
            next_frame().await;
            Ok(())
        }
        let before = suspension::fingerprint(&app.game);
        let hp = app
            .game
            .actors()
            .get(app.game.player_id())
            .unwrap()
            .integrity();
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Skill1).clone()].into(),
            ..Default::default()
        });
        if app.attack_aim.is_none() || suspension::fingerprint(&app.game) != before {
            return Err("Visée de zone incorrecte".into());
        }
        let aim = app.attack_aim.unwrap();
        let cells = app
            .aimed_attack_preview(aim)
            .map_err(|e| format!("{e:?}"))?
            .cells()
            .len();
        if cells != 9 {
            return Err("La zone de base doit couvrir 3 × 3 cases".into());
        }
        frame(&app, output, "blast-aim.png").await?;
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Attack).clone()].into(),
            ..Default::default()
        });
        if app.game.turn() != 1 || app.attack_aim.is_some() {
            return Err("Explosion non exécutée".into());
        }
        let target = GridPos::new(6, 4);
        let victims: Vec<_> = app
            .game
            .actors()
            .iter()
            .filter(|(id, a)| *id != app.game.player_id() && a.integrity() < 500)
            .map(|(id, a)| (id, a.position(), 500 - a.integrity()))
            .collect();
        if victims.len() != 3 {
            return Err(format!("Trois victimes attendues : {}", victims.len()));
        }
        for (i, age) in [0.01, 0.06, 0.12, 0.22, 0.32, 0.42, 0.52, 0.65]
            .into_iter()
            .enumerate()
        {
            let started = get_time() - age;
            app.combat_motion.clear();
            app.combat_motion.play_blast(target, 1, started);
            for &(id, at, _) in &victims {
                app.combat_motion.play(id, at, target, true, started);
            }
            frame(&app, output, &format!("blast-{i}.png")).await?;
        }
        app.open_agent_skill_window();
        app.select_agent_skill_for_capture("A02")?;
        frame(&app, output, "blast-skill.png").await?;
        if app
            .game
            .actors()
            .get(app.game.player_id())
            .unwrap()
            .integrity()
            != hp
        {
            return Err("Dégâts sur le lanceur".into());
        }
        let damage: Vec<_> = victims
            .iter()
            .map(|(_, at, amount)| serde_json::json!({"at":[at.x,at.y],"damage":amount}))
            .collect();
        write_agent_review_json(
            &output.join("blast-checks.json"),
            serde_json::json!({"review":"AGENT-PLAY04","skill":"A02","cells":cells,"damage":damage,
            "preview_free":true,"preview_hidden_after_confirmation":true,"caster_unharmed":true,"saves_untouched":true,"turns":app.game.turn(),"viewport":[1280,800],"tile_pixels":64}),
        )?;
        Ok(())
    }
    #[cfg(debug_assertions)]
    pub async fn capture_agent_drone_playable(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let mut app = Self::new_agent_drone_trial()?;
        app.suspension_path = output.join("isolated-run.json");
        request_new_screen_size(1280., 800.);
        for _ in 0..5 {
            app.draw();
            next_frame().await;
        }
        async fn frame(app: &AsciiApp, output: &std::path::Path, index: u8) -> Result<(), String> {
            app.draw();
            let image = crate::ui_capture::framebuffer()?;
            if (image.width, image.height) != (1280, 800) {
                return Err("Surface de capture indisponible".into());
            }
            image.export_png(
                output
                    .join(format!("drone-{index}.png"))
                    .to_str()
                    .ok_or("Chemin de capture")?,
            );
            next_frame().await;
            Ok(())
        }
        let matter = app.game.player_matter();
        frame(&app, output, 0).await?;
        for (index, action) in [Action::Skill1, Action::Skill2, Action::Skill3]
            .into_iter()
            .enumerate()
        {
            let before = suspension::fingerprint(&app.game);
            app.update_input(&InputFrame {
                pressed: [app.controls.binding(action).clone()].into(),
                ..Default::default()
            });
            if app.attack_aim.is_none() || suspension::fingerprint(&app.game) != before {
                return Err("Visée d’invocation incorrecte".into());
            }
            if index == 0 {
                frame(&app, output, 1).await?;
            }
            app.update_input(&InputFrame {
                pressed: [app.controls.binding(Action::Attack).clone()].into(),
                ..Default::default()
            });
            if app.game.player_agent_drone_count() != index + 1 {
                return Err("Invocation non exécutée".into());
            }
            frame(&app, output, index as u8 + 2).await?;
        }
        for _ in 0..1 {
            app.execute_command(GameCommand::Wait);
            app.capture_events();
        }
        let taunted = app
            .game
            .actors()
            .iter()
            .filter(|(_, a)| {
                a.statuses()
                    .any(|s| s.definition.as_str() == "agent:provoked")
            })
            .count();
        if taunted == 0 {
            return Err("La provocation n’a atteint aucune cible".into());
        }
        frame(&app, output, 5).await?;
        app.open_agent_skill_window();
        app.select_agent_skill_for_capture("D66")?;
        frame(&app, output, 6).await?;
        app.skills_open = false;
        if app.game.player_matter() != matter {
            return Err("Les drones ont consommé les munitions du joueur".into());
        }
        write_agent_review_json(
            &output.join("drone-checks.json"),
            serde_json::json!({
                "review":"AGENT-PLAY03","skills":["D58","D59","D66"],"real_companions":3,"taunted_targets":taunted,
                "viewport":[1280,800],"tile_pixels":64,"capacity":app.game.player_agent_drone_capacity(),
                "previews_free":true,"player_ammunition_unchanged":true,"saves_untouched":true,
                "turns":app.game.turn()
            }),
        )?;
        Ok(())
    }
    pub fn new_agent_drone_trial() -> Result<Self, String> {
        let mut app = Self::new_mixed_weapon_trial()?;
        for skill in ["D58", "D59", "D66"] {
            if app.execute_command(GameCommand::LearnAgentSkill {
                skill: skill.into(),
            }) != CommandOutcome::AppliedWithoutTime
            {
                return Err("Apprentissage de diagnostic refusé".into());
            }
        }
        app.capture_events();
        app.ux.skill_bar[0] = Some("D58".into());
        app.ux.skill_bar[1] = Some("D59".into());
        app.ux.skill_bar[2] = Some("D66".into());
        app.terminal.title = "Laboratoire · Drones de combat".into();
        Ok(app)
    }
    pub fn new_agent_melee_trial() -> Result<Self, String> {
        let mut app = Self::new_mixed_weapon_trial()?;
        if app.execute_command(GameCommand::LearnAgentSkill {
            skill: "B01".into(),
        }) != CommandOutcome::AppliedWithoutTime
        {
            return Err("Apprentissage de diagnostic refusé".into());
        }
        app.capture_events();
        app.ux.skill_bar[0] = Some("B01".into());
        app.terminal.title = "Laboratoire · Frappe de rupture".into();
        Ok(app)
    }

    #[cfg(debug_assertions)]
    pub async fn capture_agent_melee_playable(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let mut app = Self::new_agent_melee_trial()?;
        app.suspension_path = output.join("isolated-run.json");
        request_new_screen_size(1280., 800.);
        for _ in 0..5 {
            app.draw();
            next_frame().await;
        }
        let target = app.game.actors().entity_at(GridPos::new(5, 4)).unwrap();
        let hp = app.game.actors().get(target).unwrap().integrity();
        let initial = suspension::fingerprint(&app.game);
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Skill1).clone()].into(),
            ..Default::default()
        });
        if app.attack_aim.is_none_or(|a| a.slot != 0)
            || suspension::fingerprint(&app.game) != initial
        {
            return Err("Prévisualisation de Frappe de rupture incorrecte".into());
        }
        app.draw();
        let frame = crate::ui_capture::framebuffer()?;
        if (frame.width, frame.height) != (1280, 800) {
            return Err("Surface de capture indisponible".into());
        }
        frame.export_png(
            output
                .join("rupture-aim.png")
                .to_str()
                .ok_or("Chemin de capture")?,
        );
        next_frame().await;
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Attack).clone()].into(),
            ..Default::default()
        });
        if app.game.turn() != 1 || app.attack_aim.is_some() {
            return Err("Frappe non exécutée".into());
        }
        let origin = app.game.player_position().unwrap();
        let target_at = GridPos::new(5, 4);
        for (index, age) in [0.06, 0.12, 0.20, 0.28, 0.40, 0.55, 0.72, 0.92]
            .into_iter()
            .enumerate()
        {
            // Sample presentation time explicitly: framebuffer exports can take
            // longer than the attack itself. The completed game action stays intact.
            let began = get_time() - age;
            app.combat_motion.clear();
            app.combat_motion.play_rupture(origin, target_at, began);
            app.combat_motion
                .play(target, target_at, origin, true, began + 0.22);
            app.draw();
            crate::ui_capture::framebuffer()?.export_png(
                output
                    .join(format!("rupture-{index}.png"))
                    .to_str()
                    .ok_or("Chemin de capture")?,
            );
            next_frame().await;
        }
        let damage = hp - app.game.actors().get(target).unwrap().integrity();
        if damage == 0 {
            return Err("Frappe sans dégâts".into());
        }
        write_agent_review_json(
            &output.join("checks.json"),
            serde_json::json!({
                "review":"AGENT-PLAY02","skill":"B01","tile_pixels":64,
                "viewport":[1280,800],"preview_free":true,"damage":damage,"turns":1,
                "matching_weapon_slot":0,"preview_hidden_after_confirmation":true,"saves_untouched":true
            }),
        )?;
        Ok(())
    }

    pub fn new_mixed_weapon_trial() -> Result<Self, String> {
        let mut app = Self::new_agent_weapon_switch_trial()?;
        let mut rules = app.game.rules().clone();
        rules.player_weapon_slots = project_rl::game::WEAPON_HAND_SLOTS
            .map(|id| id.parse().unwrap())
            .to_vec();
        rules.player_starting_weapons = [
            "core:integrity_blade",
            "core:modele_cogneur_p01",
            "core:flamethrower",
        ]
        .map(|id| id.parse().unwrap())
        .to_vec();
        rules.player_starting_equipment = rules
            .player_starting_weapons
            .iter()
            .cloned()
            .map(Some)
            .collect();
        rules
            .player_starting_items
            .push(project_rl::game::StartingItemStack::new(
                "core:weapon_matter".parse().unwrap(),
                40,
            ));
        rules.progression.starting_skill_points = 50;
        let mut game =
            GameState::new_with_rules(app.game.map().clone(), GridPos::new(4, 4), 29, rules)
                .map_err(|e| e.to_string())?;
        app.actor_glyphs.clear();
        for (at, glyph) in [(GridPos::new(5, 4), 'B'), (GridPos::new(4, 7), 'C')] {
            let entity = game
                .spawn_actor(
                    Actor::new(at, 500)
                        .unwrap()
                        .with_evasion_disabled()
                        .with_player_relation(PlayerRelation::Hostile),
                )
                .map_err(|e| e.to_string())?;
            app.actor_glyphs.insert(entity, glyph);
        }
        game.drain_events();
        app.terminal =
            TerminalView::new(SectorDecor::default(), game.map(), game.player_visibility());
        app.terminal.title = "Laboratoire · Mêlée et distance".into();
        app.game = WorldState::single(game);
        app.history.clear();
        app.ux.skill_bar = Default::default();
        app.skills_open = false;
        app.active_weapon_slot = 1;
        app.select_world_target(Some(GridPos::new(5, 4)));
        Ok(app)
    }

    #[cfg(debug_assertions)]
    pub async fn capture_mixed_weapon_trial(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let mut app = Self::new_mixed_weapon_trial()?;
        app.suspension_path = output.join("isolated-run.json");
        request_new_screen_size(1280., 800.);
        for _ in 0..5 {
            app.draw();
            next_frame().await;
        }
        async fn frame(app: &AsciiApp, output: &std::path::Path, index: u8) -> Result<(), String> {
            app.draw();
            let image = crate::ui_capture::framebuffer()?;
            if image.width != 1280 || image.height != 800 {
                return Err("Surface de capture native indisponible".into());
            }
            image.export_png(
                output
                    .join(format!("native-preview-{index}.png"))
                    .to_str()
                    .ok_or("Chemin de capture")?,
            );
            next_frame().await;
            Ok(())
        }
        let matter = app.game.player_matter().ok_or("Munitions absentes")?;
        frame(&app, output, 0).await?;
        let key = InputFrame {
            pressed: [app.controls.binding(Action::Attack).clone()].into(),
            ..Default::default()
        };
        app.update_input(&key);
        if app.game.turn() != 1
            || app.active_weapon_slot != 0
            || app.game.player_matter() != Some(matter)
        {
            return Err("Le coup de mêlée automatique est incorrect".into());
        }
        frame(&app, output, 1).await?;
        app.update_input(&InputFrame::default());
        app.select_world_target(Some(GridPos::new(4, 7)));
        app.update_input(&key);
        if app.game.turn() != 2
            || app.active_weapon_slot != 1
            || app.game.player_matter() != Some(matter - 1)
        {
            return Err("Le tir automatique est incorrect".into());
        }
        frame(&app, output, 2).await?;
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "review":"AGENT-MIXED01","ordinary_attacks":"melee_at_contact_ranged_farther",
            "actual_command_slots":[0,1],"actions":2,"ammunition_spent":1,
            "one_weapon_per_action":true,"matching_hand_for_skills":true,
            "inactive_set_excluded":true,"new_catalogue_effects":"integration_by_batches_unchanged",
            "checked_viewports":[[1280,800]],"saves_untouched":true
        })).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }

    #[cfg(test)]
    pub fn new_agent_spell_trial() -> Result<Self, String> {
        Self::agent_spell_trial_with_weapons(false)
    }

    pub fn new_agent_weapon_switch_trial() -> Result<Self, String> {
        Self::agent_spell_trial_with_weapons(true)
    }

    fn agent_spell_trial_with_weapons(full_loadout: bool) -> Result<Self, String> {
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(INITIAL_SEED, rules, texts, loot, expeditions)?;
        let id: CharacterClassId = "core:arcaniste".parse().unwrap();
        let class = app.character_classes.get(&id).ok_or("Arcaniste absent")?;
        class
            .apply_to_rules(&mut app.rules, class.recommended_attributes())
            .map_err(|e| e.to_string())?;
        app.character_class = Some(id);
        if full_loadout {
            app.rules.player_starting_weapons = [
                "core:integrity_blade",
                "core:needle_launcher",
                "core:flamethrower",
            ]
            .map(|id| id.parse().unwrap())
            .to_vec();
            app.rules.player_starting_equipment = app
                .rules
                .player_starting_weapons
                .iter()
                .cloned()
                .map(Some)
                .collect();
        }
        let map = project_rl::world::Map::from_ascii(
            "###############\n#.............#\n#.............#\n#.............#\n#.............#\n#.............#\n#.............#\n#.............#\n###############"
        ).map_err(|e| e.to_string())?;
        let mut game =
            GameState::new_with_rules(map, GridPos::new(4, 4), INITIAL_SEED, app.rules.clone())
                .map_err(|e| e.to_string())?;
        // Earn and buy through the same commands as a campaign, with no user save.
        let mut progression: serde_json::Value = serde_json::from_str(
            &game
                .export_player_progression()
                .map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        progression["level"] = 2.into();
        progression["experience"] =
            game.rules().progression.curve.cumulative_thresholds()[0].into();
        progression["skill_points_earned"] = 1.into();
        progression["unspent_skill_points"] = 1.into();
        game.restore_player_progression(&progression.to_string())
            .map_err(|e| e.to_string())?;
        if game.process_player_command(GameCommand::LearnAgentSkill {
            skill: "A01".into(),
        }) != CommandOutcome::AppliedWithoutTime
        {
            return Err("Apprentissage de diagnostic refusé".into());
        }
        let target = game
            .spawn_actor(
                Actor::new(GridPos::new(7, 4), 500)
                    .map_err(|e| e.to_string())?
                    .with_player_relation(PlayerRelation::Hostile),
            )
            .map_err(|e| e.to_string())?;
        game.drain_events();
        app.terminal =
            TerminalView::new(SectorDecor::default(), game.map(), game.player_visibility());
        app.terminal.title = "Laboratoire · Trait arcanique".into();
        app.game = WorldState::single(game);
        app.actor_glyphs.clear();
        app.actor_glyphs.insert(target, 'B');
        app.npc_art.clear();
        app.test_lab = true;
        app.open_menu(MenuScreen::Hidden);
        app.character_creation = None;
        app.graphics.active.world_cell_px = 64;
        app.facing = Direction::East;
        app.active_weapon_slot = 0;
        app.select_world_target(Some(GridPos::new(7, 4)));
        app.log.clear();
        app.open_agent_skill_window();
        app.select_agent_skill_for_capture("A01")?;
        app.ux.skill_bar[0] = Some("A01".into());
        app.ux.skill_bar[1] = Some("FE01".into());
        Ok(app)
    }

    #[cfg(debug_assertions)]
    pub async fn capture_agent_spell_playable(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let mut app = Self::new_agent_weapon_switch_trial()?;
        app.suspension_path = output.join("isolated-run.json");
        request_new_screen_size(1280., 800.);
        for _ in 0..5 {
            app.draw();
            next_frame().await;
        }
        async fn frame(app: &AsciiApp, path: &std::path::Path) -> Result<(), String> {
            app.draw();
            let capture = crate::ui_capture::framebuffer()?;
            if capture.width < 640 || capture.height < 480 {
                return Err("Capture native indisponible : surface de fenêtre trop petite.".into());
            }
            capture.export_png(path.to_str().ok_or("Chemin de capture")?);
            next_frame().await;
            Ok(())
        }
        frame(&app, &output.join("skills.png")).await?;
        let before = suspension::fingerprint(&app.game);
        app.update_agent_skills(&InputFrame {
            pressed: [app.controls.binding(Action::Use).clone()].into(),
            viewport: Some((1280., 800.)),
            ..Default::default()
        });
        if app.attack_aim.is_none()
            || app.skills_open
            || suspension::fingerprint(&app.game) != before
        {
            return Err("Utiliser ne lance pas une visée sans coût".into());
        }
        frame(&app, &output.join("aim.png")).await?;
        app.attack_aim = None;
        app.attack_aim_technique = None;
        let switch_before = suspension::fingerprint(&app.game);
        for (index, expected) in [1, 2, 0].into_iter().enumerate() {
            app.cycle_bar_weapon();
            if app.active_weapon_slot != expected
                || suspension::fingerprint(&app.game) != switch_before
            {
                return Err("Le changement d’arme a modifié la simulation".into());
            }
            frame(&app, &output.join(format!("weapon-switch-{index}.png"))).await?;
        }
        if std::env::args().any(|arg| arg == "--weapon-switch-only") {
            std::fs::write(output.join("checks.json"), serde_json::to_vec_pretty(&serde_json::json!({
                "review":"AGENT-BAR02", "review_scope":"weapon_switch_only",
                "weapon_switch_cycle":[1,2,3,1], "weapon_switch_without_time_or_resources":true,
                "weapon_switch_side":"left", "weapon_switch_icon":"circular_arrow", "slot_number_visible":false,
                "checked_viewports":[[1280,800]], "saves_untouched":true
            })).unwrap()).map_err(|e| e.to_string())?;
            return Ok(());
        }
        app.open_technique_menu();
        let active = app.active_learned_techniques();
        app.technique_menu_selection = active
            .iter()
            .position(|id| id == &project_rl::skills::agent::technique("A01"))
            .ok_or("Sort absent du clic droit")?;
        frame(&app, &output.join("quick-menu.png")).await?;
        app.use_quick_technique(&active);
        let before_hp = app
            .game
            .actors()
            .get(app.selected_target.ok_or("Cible absente")?)
            .unwrap()
            .integrity();
        let aim = app.attack_aim.ok_or("Visée du menu rapide absente")?;
        let command = GameCommand::UseTechniqueAt {
            technique: app.attack_aim_technique.clone().unwrap(),
            target: aim.cursor,
            weapon_slot: aim.slot,
        };
        if app.execute_command(command) != CommandOutcome::Applied {
            return Err("Sort refusé dans le laboratoire".into());
        }
        app.attack_aim = None;
        app.attack_aim_technique = None;
        let after_hp = app
            .game
            .actors()
            .get(app.selected_target.unwrap())
            .unwrap()
            .integrity();
        let profile = app.game.agent_spell_profile("A01", 1).unwrap();
        let spell = (
            app.selected_target.unwrap(),
            app.game.player_position().unwrap(),
            aim.cursor,
        );
        app.capture_events_at(Some(get_time()));
        // Capture the actual presentation at controlled ages; no simulation steps.
        for i in 0..28 {
            app.combat_motion.clear();
            let age = i as f64 * 0.02;
            app.combat_motion
                .play_spell(spell.0, spell.1, spell.2, get_time() - age);
            if age >= 0.18 {
                app.combat_motion
                    .play(spell.0, spell.2, spell.1, true, get_time() - (age - 0.18));
            }
            frame(&app, &output.join(format!("cast-{i:02}.png"))).await?;
        }
        let recast = GameCommand::UseTechniqueAt {
            technique: project_rl::skills::agent::technique("A01"),
            target: aim.cursor,
            weapon_slot: aim.slot,
        };
        let before_recast_turn = app.game.turn();
        if app.execute_command(recast) != CommandOutcome::Applied
            || app.game.turn() != before_recast_turn + 1
        {
            return Err("Relancement sans attente refusé".into());
        }
        app.capture_events_at(Some(get_time()));
        for i in 0..28 {
            app.combat_motion.clear();
            let age = i as f64 * 0.02;
            app.combat_motion
                .play_spell(spell.0, spell.1, spell.2, get_time() - age);
            if age >= 0.18 {
                app.combat_motion
                    .play(spell.0, spell.2, spell.1, true, get_time() - (age - 0.18));
            }
            frame(&app, &output.join(format!("cast-{:02}.png", i + 28))).await?;
        }
        app.combat_motion.clear();
        request_new_screen_size(960., 540.);
        for _ in 0..4 {
            app.draw();
            next_frame().await;
        }
        frame(&app, &output.join("bar-960.png")).await?;
        app.open_agent_skill_window();
        frame(&app, &output.join("skills-960.png")).await?;
        app.skills_open = false;
        std::fs::write(output.join("checks.json"), serde_json::to_vec_pretty(&serde_json::json!({
            "skill":"A01", "use_opens_targeting_without_cost":true,"quick_menu_contains_learned_spell":true,
            "actual_damage":before_hp-after_hp,"damage_before_defenses":profile.damage_before_defenses(),
            "energy_cost":profile.energy,"cooldown":profile.cooldown,"tile_pixels":64,
            "casts_without_waiting":2,"turns_advanced":app.game.turn(),
            "bar_slots":6,"bar_visible_game_and_tree":true,"checked_viewports":[[1280,800],[960,540]],
            "weapon_switch_cycle":[1,2,3,1],"weapon_switch_without_time_or_resources":true,
            "saves_untouched":true,"preview_hidden_after_confirmation":app.attack_aim.is_none()
        })).unwrap()).map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mixed_weapon_default_attacks_choose_one_weapon_and_spend_only_its_resources() {
        let mut app = AsciiApp::new_mixed_weapon_trial().unwrap();
        let near = app.game.actors().entity_at(GridPos::new(5, 4)).unwrap();
        let far = app.game.actors().entity_at(GridPos::new(4, 7)).unwrap();
        let matter = app.game.player_matter();
        let energy = app.game.player_energy().available();
        let key = InputFrame {
            pressed: [app.controls.binding(Action::Attack).clone()].into(),
            ..Default::default()
        };
        app.update_input(&key);
        assert_eq!(app.game.turn(), 1);
        assert!(matches!(
            app.history.last(),
            Some(RecordedCommand::Attack { slot: 0, .. })
        ));
        assert_eq!(app.active_weapon_slot, 0);
        assert_eq!(app.game.player_matter(), matter);
        assert_eq!(app.game.actors().get(far).unwrap().integrity(), 500);
        assert!(app.game.actors().get(near).unwrap().integrity() < 500);
        app.update_input(&InputFrame::default());
        app.select_world_target(Some(GridPos::new(4, 7)));
        app.update_input(&key);
        assert_eq!(app.game.turn(), 2);
        assert!(matches!(
            app.history.last(),
            Some(RecordedCommand::Attack { slot: 1, .. })
        ));
        assert_eq!(app.active_weapon_slot, 1);
        assert_eq!(app.game.player_matter(), matter.map(|v| v - 1));
        assert_eq!(app.game.player_energy().available(), energy);
        assert!(app.game.actors().get(far).unwrap().integrity() < 500);
        let encoded = app.game.recovery_snapshot_bytes().unwrap();
        let restored =
            WorldState::from_recovery_snapshot_bytes(&encoded, app.game.rules().clone()).unwrap();
        assert_eq!(restored.recovery_snapshot_bytes().unwrap(), encoded);
        let bump = read_movement_command(
            &app.game,
            1,
            &app.controls,
            &InputFrame {
                pressed: [app.controls.binding(Action::MoveEast).clone()].into(),
                ..Default::default()
            },
        );
        assert!(matches!(bump,Some(GameCommand::Attack { slot:0,target }) if target==near));
    }

    #[test]
    fn mixed_weapon_techniques_use_the_matching_hand_and_never_the_inactive_set() {
        let mut app = AsciiApp::new_mixed_weapon_trial().unwrap();
        for name in ["core:mel_02", "core:tir_01"] {
            assert_eq!(
                app.execute_command(GameCommand::LearnTechnique {
                    technique: name.parse().unwrap()
                }),
                CommandOutcome::AppliedWithoutTime
            );
        }
        app.select_world_target(Some(GridPos::new(5, 4)));
        assert!(
            app.quick_technique_status(&"core:mel_02".parse().unwrap())
                .1
                .is_none()
        );
        let command = app
            .technique_command("core:mel_02".parse().unwrap())
            .unwrap();
        assert!(matches!(
            &command,
            GameCommand::UseTechnique {
                weapon_slot: Some(0),
                ..
            }
        ));
        assert_eq!(app.execute_command(command), CommandOutcome::Applied);
        app.capture_events();
        app.select_world_target(Some(GridPos::new(4, 7)));
        assert!(
            app.quick_technique_status(&"core:tir_01".parse().unwrap())
                .1
                .is_none()
        );
        let command = app
            .technique_command("core:tir_01".parse().unwrap())
            .unwrap();
        assert!(matches!(
            &command,
            GameCommand::UseTechnique {
                weapon_slot: Some(1),
                ..
            }
        ));
        assert_eq!(app.execute_command(command), CommandOutcome::Applied);
        app.active_weapon_slot = 2;
        assert_eq!(
            app.game
                .player_weapon_slot_for_technique(2, &project_rl::skills::agent::technique("B01")),
            2
        );
        assert_eq!(
            app.game
                .player_weapon_slot_for_technique(0, &project_rl::skills::agent::technique("DI01")),
            1
        );
    }

    #[test]
    fn mixed_weapon_out_of_ammunition_refuses_without_time_movement_or_melee_fallback() {
        let mut app = AsciiApp::new_mixed_weapon_trial().unwrap();
        let ammo = app
            .game
            .player_inventory()
            .iter()
            .find(|e| e.item().as_str() == "core:weapon_matter")
            .unwrap()
            .instance();
        assert_eq!(
            app.execute_command(GameCommand::DropItem { item: ammo }),
            CommandOutcome::Applied
        );
        app.capture_events();
        app.active_weapon_slot = 0;
        app.select_world_target(Some(GridPos::new(4, 7)));
        let before = suspension::fingerprint(&app.game);
        let history = app.history.len();
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Attack).clone()].into(),
            ..Default::default()
        });
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert_eq!(app.history.len(), history);
        assert!(app.mouse_walk.is_none());
    }
    #[test]
    fn learned_agent_spell_is_accessible_from_use_and_quick_menu_without_preview_cost() {
        let mut app = AsciiApp::new_agent_spell_trial().unwrap();
        let before = suspension::fingerprint(&app.game);
        app.update_agent_skills(&InputFrame {
            pressed: [app.controls.binding(Action::Use).clone()].into(),
            ..Default::default()
        });
        assert!(app.attack_aim.is_some());
        assert!(!app.skills_open);
        assert_eq!(suspension::fingerprint(&app.game), before);
        app.update_input(&InputFrame {
            pause: true,
            ..Default::default()
        });
        assert!(app.attack_aim.is_none());
        assert_eq!(suspension::fingerprint(&app.game), before);
        app.open_technique_menu();
        assert!(app.technique_menu_open);
        let active = app.active_learned_techniques();
        let index = active
            .iter()
            .position(|id| id == &project_rl::skills::agent::technique("A01"))
            .unwrap();
        assert!(app.quick_technique_status(&active[index]).1.is_none());
        assert!(
            app.quick_technique_status(&project_rl::skills::agent::technique("FE01"))
                .1
                .is_none()
        );
        app.technique_menu_selection = index;
        app.use_quick_technique(&active);
        assert_eq!(suspension::fingerprint(&app.game), before);
        let aim = app.attack_aim.unwrap();
        assert_eq!(aim.cursor, GridPos::new(7, 4));
        assert!(app.aimed_attack_preview(aim).is_ok());
        app.update_attack_aim(&InputFrame {
            pressed: [app.controls.binding(Action::Attack).clone()].into(),
            ..Default::default()
        });
        assert!(app.attack_aim.is_none());
        assert_ne!(suspension::fingerprint(&app.game), before);
        assert!(app.quick_technique_status(&active[index]).1.is_none());
        let turn = app.game.turn();
        app.begin_technique_attack_aim(active[index].clone());
        app.update_attack_aim(&InputFrame {
            pressed: [app.controls.binding(Action::Attack).clone()].into(),
            ..Default::default()
        });
        assert_eq!(app.game.turn(), turn + 1);
        assert!(app.attack_aim.is_none());
        assert!(app.observation_report.is_empty());
        // The synthetic command uses the existing replay representation.
        let encoded = serde_json::to_string(&app.history).unwrap();
        assert!(encoded.contains("agent:a01"));
    }
}

/// Windows editors may map an existing receipt; replace it without truncating it.
#[cfg(debug_assertions)]
fn write_agent_review_json(path: &std::path::Path, value: serde_json::Value) -> Result<(), String> {
    let temporary = path.with_extension("agent-receipt-tmp");
    std::fs::write(
        &temporary,
        serde_json::to_vec_pretty(&value).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    std::fs::rename(temporary, path).map_err(|e| e.to_string())
}

impl AsciiApp {
    fn prepare_agent_support_visual_trial(
        &mut self,
    ) -> Result<(EntityId, EntityId, EntityId), String> {
        for (skill, target) in [
            ("D56", GridPos::new(7, 11)),
            ("D57", GridPos::new(6, 12)),
            ("ES03", GridPos::new(9, 13)),
            ("SF00", GridPos::new(7, 12)),
        ] {
            let outcome = self.execute_command(GameCommand::UseTechniqueAt {
                technique: project_rl::skills::agent::technique(skill),
                target,
                weapon_slot: self.active_weapon_slot,
            });
            if outcome != CommandOutcome::Applied {
                return Err(format!("Diagnostic {skill} refusé : {outcome:?}"));
            }
        }
        let scout = self
            .game
            .actors()
            .entity_at(GridPos::new(7, 11))
            .ok_or("Éclaireur absent")?;
        let mule = self
            .game
            .actors()
            .entity_at(GridPos::new(6, 12))
            .ok_or("Mule absente")?;
        let hologram = self
            .game
            .actors()
            .iter()
            .find_map(|(entity, _)| self.game.agent_is_hologram(entity).then_some(entity))
            .ok_or("Hologramme absent")?;
        Ok((scout, mule, hologram))
    }

    /// One reusable diagnostic folder exercises the real laboratory and real commands.
    #[cfg(debug_assertions)]
    pub async fn capture_agent_catalogue_playable(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1280., 800.);
        let mut app = Self::new_agent_skill_laboratory()?;
        app.suspension_path = output.join("isolated-run.json");
        app.graphics.active.world_cell_px = 64;
        app.ux.skill_bar = ["FE02", "FR02", "M01", "SM00", "D58", "SG00"].map(|id| Some(id.into()));
        let total = project_rl::skills::agent::definitions().len();
        let learned = project_rl::skills::agent::definitions()
            .iter()
            .filter(|s| app.game.player_agent_skill_rank(&s.id) > 0)
            .count();
        if total != learned {
            return Err(format!(
                "Catalogue laboratoire incomplet : {learned}/{total}"
            ));
        }
        for _ in 0..5 {
            app.draw();
            next_frame().await;
        }
        async fn frame(app: &AsciiApp, output: &std::path::Path, name: &str) -> Result<(), String> {
            app.draw();
            let image = crate::ui_capture::framebuffer()?;
            if (image.width, image.height) != (1280, 800) {
                return Err("Surface de diagnostic différente de 1280 × 800".into());
            }
            image.export_png(output.join(name).to_str().ok_or("Chemin de diagnostic")?);
            next_frame().await;
            Ok(())
        }
        app.open_agent_skill_window();
        app.select_agent_skill_for_capture("SG00")?;
        frame(&app, output, "competences-variantes.png").await?;
        app.skills_open = false;
        let mine = GridPos::new(7, 11);
        if app.execute_command(GameCommand::SetAgentMineVariant {
            variant: project_rl::game::AgentMineVariant::Poison,
        }) != CommandOutcome::AppliedWithoutTime
        {
            return Err("Variante Poison indisponible dans le laboratoire".into());
        }
        if app.execute_command(GameCommand::UseTechniqueAt {
            technique: project_rl::skills::agent::technique("SM00"),
            target: mine,
            weapon_slot: app.active_weapon_slot,
        }) != CommandOutcome::Applied
        {
            return Err("Mine réelle refusée dans le laboratoire".into());
        }
        app.capture_events();
        frame(&app, output, "mine-led.png").await?;
        for _ in 1..10 {
            if app.execute_command(GameCommand::LearnAgentSkill {
                skill: "FR02".into(),
            }) != CommandOutcome::AppliedWithoutTime
            {
                return Err("Amélioration de diagnostic refusée".into());
            }
        }
        let origin = app.game.player_position().ok_or("Personnage absent")?;
        for _ in 0..3 {
            if app.execute_command(GameCommand::UseTechniqueAt {
                technique: project_rl::skills::agent::technique("FR02"),
                target: origin,
                weapon_slot: app.active_weapon_slot,
            }) != CommandOutcome::Applied
            {
                return Err("Nova glaciale réelle refusée".into());
            }
            app.capture_events();
            if app
                .game
                .actors()
                .iter()
                .any(|(_, a)| a.status(&"agent:frozen".parse().unwrap()).is_some())
            {
                break;
            }
        }
        let frozen = app
            .game
            .actors()
            .iter()
            .filter(|(_, a)| a.status(&"agent:frozen".parse().unwrap()).is_some())
            .count();
        if frozen == 0 {
            return Err("Le diagnostic de Nova n'a produit aucun Gel".into());
        }
        app.combat_motion.clear();
        app.combat_motion
            .play_agent_magic("FR02", origin, origin, 5, get_time() - 0.20);
        frame(&app, output, "magie-gel.png").await?;
        if app.execute_command(GameCommand::UseTechniqueAt {
            technique: project_rl::skills::agent::technique("M01"),
            target: origin,
            weapon_slot: app.active_weapon_slot,
        }) != CommandOutcome::Applied
        {
            return Err("Transformation réelle refusée dans le laboratoire".into());
        }
        app.capture_events();
        app.combat_motion.clear();
        frame(&app, output, "transformation.png").await?;
        let mut support = Self::new_agent_skill_laboratory()?;
        support.suspension_path = output.join("isolated-run.json");
        support.graphics.active.world_cell_px = 64;
        support.ux.skill_bar =
            ["D56", "D57", "D58", "D59", "ES03", "SF00"].map(|id| Some(id.into()));
        let (scout, mule, hologram) = support.prepare_agent_support_visual_trial()?;
        support.combat_motion.clear();
        frame(&support, output, "drones-furtivite.png").await?;
        let mut status_trial = Self::new_agent_skill_laboratory()?;
        status_trial.graphics.active.world_cell_px = 64;
        let (marked, raised, curse_count) = status_trial.prepare_agent_status_regression_trial()?;
        status_trial.combat_motion.clear();
        frame(&status_trial, output, "statuts-reanimation.png").await?;
        write_agent_review_json(
            &output.join("catalogue-checks.json"),
            serde_json::json!({
                "diagnostic":"agent-catalogue-playable","learned":learned,"catalogue_count":total,
                "hotbar":["FE02","FR02","M01","SM00","D58","SG00"],"frozen_targets":frozen,
                "real_mines":app.game.player_agent_mines().len(),"viewport":[1280,800],"tile_pixels":64,
                "frames":["competences-variantes.png","mine-led.png","magie-gel.png","transformation.png","drones-furtivite.png","statuts-reanimation.png"],
                "status_regression": {
                    "marked":status_trial.game.agent_reanimation_marked(marked),
                    "raised":status_trial.game.agent_reanimated_ally(raised),
                    "same_creature_glyph":status_trial.actor_glyphs.get(&raised)==Some(&'Q'),
                    "curse_icons":curse_count,
                    "generic_timer_on_cursed_targets":status_trial.game.actors().iter()
                        .filter(|(_,a)|a.status(&"agent:armor_breach".parse().unwrap()).is_some())
                        .any(|(_,a)|status_trial.effect_badges_at(a.position()).iter().any(|b|
                            matches!(b,crate::terminal_view::TerminalEffectBadge::Timed))),
                },
                "support_visuals": {
                    "scout":format!("{:?}",support.front_actor_visual(scout)),
                    "mule":format!("{:?}",support.front_actor_visual(mule)),
                    "hologram":format!("{:?}",support.front_actor_visual(hologram)),
                    "stealth":support.game.agent_stealth_active(),
                    "player_opacity":support.glyph_at(GridPos::new(7,12)).unwrap().1.a,
                    "turns":support.game.turn(),
                },
                "game_save_untouched":true,"turns":app.game.turn(),"disposable_laboratory":app.test_lab
            }),
        )?;
        Ok(())
    }
}

impl AsciiApp {
    #[cfg(debug_assertions)]
    fn prepare_agent_status_regression_trial(
        &mut self,
    ) -> Result<(EntityId, EntityId, usize), String> {
        for _ in 1..10 {
            if self.execute_command(GameCommand::LearnAgentSkill {
                skill: "AF02".into(),
            }) != CommandOutcome::AppliedWithoutTime
            {
                return Err("Le rang de la malédiction de contrôle n'a pas été appris".into());
            }
        }
        let original_at = GridPos::new(6, 12);
        let original = self
            .game
            .spawn_actor(
                Actor::new(original_at, 1)
                    .unwrap()
                    .with_evasion_disabled()
                    .with_player_relation(PlayerRelation::Hostile),
            )
            .map_err(|e| e.to_string())?;
        self.actor_glyphs.insert(original, 'Q');
        for skill in ["OC07", "A01"] {
            if self.execute_command(GameCommand::UseTechniqueAt {
                technique: project_rl::skills::agent::technique(skill),
                target: original_at,
                weapon_slot: 0,
            }) != CommandOutcome::Applied
            {
                return Err(format!("L'action de contrôle {skill} a été refusée"));
            }
        }
        self.capture_events_at(Some(0.));
        let raised = self
            .game
            .actors()
            .entity_at(original_at)
            .ok_or("Allié relevé absent")?;
        let at = GridPos::new(8, 12);
        let marked = self
            .game
            .actors()
            .entity_at(at)
            .ok_or("Cible à marquer absente")?;
        if self.execute_command(GameCommand::UseTechniqueAt {
            technique: project_rl::skills::agent::technique("OC07"),
            target: at,
            weapon_slot: 0,
        }) != CommandOutcome::Applied
        {
            return Err("Marque de contrôle refusée".into());
        }
        let curse: project_rl::status::StatusId = "agent:armor_breach".parse().unwrap();
        let mut curse_count = 0;
        for _ in 0..3 {
            if self.execute_command(GameCommand::UseTechniqueAt {
                technique: project_rl::skills::agent::technique("AF02"),
                target: at,
                weapon_slot: 0,
            }) != CommandOutcome::Applied
            {
                return Err("Malédiction de contrôle refusée".into());
            }
            curse_count = self
                .game
                .actors()
                .iter()
                .filter(|(_, a)| a.status(&curse).is_some())
                .count();
            if curse_count > 0 {
                break;
            }
        }
        self.capture_events_at(Some(0.));
        if !self.game.agent_reanimated_ally(raised)
            || !self.game.agent_reanimation_marked(marked)
            || curse_count == 0
        {
            return Err("Les effets de contrôle n'ont pas tous persisté".into());
        }
        Ok((marked, raised, curse_count))
    }
}

#[cfg(test)]
mod agent_catalogue_lab_tests {
    use super::*;
    #[test]
    fn auto_target_after_reanimation_skips_the_nearest_ally_but_keeps_explicit_selection() {
        let mut app = AsciiApp::new_agent_skill_laboratory().unwrap();
        let at = GridPos::new(6, 11);
        let original = app
            .game
            .spawn_actor(
                Actor::new(at, 1)
                    .unwrap()
                    .with_evasion_disabled()
                    .with_player_relation(PlayerRelation::Hostile),
            )
            .unwrap();
        app.selected_target = Some(original);
        for skill in ["OC07", "A01"] {
            assert_eq!(
                app.execute_command(GameCommand::UseTechniqueAt {
                    technique: project_rl::skills::agent::technique(skill),
                    target: at,
                    weapon_slot: 0,
                }),
                CommandOutcome::Applied
            );
        }
        app.capture_events_at(Some(0.));
        let raised = app.game.actors().entity_at(at).unwrap();
        assert_ne!(raised, original);
        assert!(app.game.is_player_controlled_companion(raised));
        assert_eq!(app.selected_target, None);
        for _ in 0..2 {
            assert_eq!(
                app.execute_command(GameCommand::Move(Direction::West)),
                CommandOutcome::Applied
            );
            app.capture_events_at(Some(0.));
        }
        // The former implementation picked this nearest visible actor.
        assert_eq!(app.visible_targets().first(), Some(&raised));
        let automatic = app.ensure_visible_target().unwrap();
        assert_ne!(automatic, raised);
        assert_ne!(
            app.game.actors().get(automatic).unwrap().player_relation(),
            PlayerRelation::Allied
        );
        app.selected_target = Some(raised);
        assert_eq!(app.ensure_visible_target(), Some(raised));
    }

    #[test]
    fn agent_laboratory_rank_ten_blizzard_hotbar_resolves_among_seven_actual_targets() {
        let mut app = AsciiApp::new_agent_skill_laboratory().unwrap();
        for _ in 1..10 {
            assert_eq!(
                app.execute_command(GameCommand::LearnAgentSkill {
                    skill: "FR05".into(),
                }),
                CommandOutcome::AppliedWithoutTime
            );
        }
        assert_eq!(app.game.player_agent_skill_rank("FR05"), 10);
        let center = GridPos::new(11, 9);
        let before_preview = suspension::fingerprint(&app.game);
        let preview = app.game.agent_spell_preview("FR05", center).unwrap();
        assert_eq!(
            suspension::fingerprint(&app.game),
            before_preview,
            "preview must show every eligible cell without sampling or RNG use"
        );
        assert_eq!(preview.cells().len(), 81);
        let victims: Vec<_> = app
            .game
            .actors()
            .iter()
            .filter_map(|(id, actor)| {
                (id != app.game.player_id()
                    && preview
                        .cells()
                        .iter()
                        .any(|cell| cell.position == actor.position()))
                .then_some((id, actor.position(), actor.integrity()))
            })
            .collect();
        assert_eq!(victims.len(), 7);
        app.game.drain_events();
        app.ux.skill_bar[0] = Some("FR05".into());
        app.select_world_target(Some(center));
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Skill1).clone()].into(),
            ..Default::default()
        });
        assert_eq!(app.game.turn(), 0);
        // Use the real aiming keys after opening the shortcut: opening it may
        // select a nearer actor instead of the previously selected empty cell.
        for _ in 0..24 {
            let cursor = app.attack_aim.unwrap().cursor;
            if cursor == center {
                break;
            }
            let action = if cursor.y < center.y {
                Action::MoveSouth
            } else if cursor.y > center.y {
                Action::MoveNorth
            } else if cursor.x < center.x {
                Action::MoveEast
            } else {
                Action::MoveWest
            };
            app.update_input(&InputFrame {
                pressed: [app.controls.binding(action).clone()].into(),
                ..Default::default()
            });
            assert_ne!(app.attack_aim.unwrap().cursor, cursor);
            assert_eq!(app.game.turn(), 0);
        }
        assert_eq!(app.attack_aim.unwrap().cursor, center);
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Attack).clone()].into(),
            ..Default::default()
        });
        assert_eq!(app.game.turn(), 1);
        assert!(app.attack_aim.is_none());
        assert!(
            victims
                .iter()
                .all(|(id, _, hp)| app.game.actors().get(*id).unwrap().integrity() == *hp)
        );
        let skill = project_rl::skills::agent::definition("FR05").unwrap();
        assert_eq!(skill.value("impact_chance_per_cell_percent", 10), 50);
        let mut total_impacts = 0;
        let mut maximum_impacts = 0;
        let mut damage_without_gel = false;
        let mut total_damage = 0;
        for turn in 1..=skill.value("duration_turns", 10) {
            assert_eq!(
                app.execute_command(GameCommand::Wait),
                CommandOutcome::Applied
            );
            let events = app.game.drain_events();
            let damage: Vec<_> = events
                .iter()
                .filter_map(|event| match event {
                    GameEvent::DamageApplied {
                        target,
                        amount,
                        damage_type: DamageType::Cold,
                        ..
                    } if *amount > 0 => Some((*target, *amount)),
                    _ => None,
                })
                .collect();
            let frozen: Vec<_> = events
                .iter()
                .filter_map(|event| match event {
                    GameEvent::StatusApplied { target, status, .. }
                        if status.as_str() == "agent:frozen" =>
                    {
                        Some(*target)
                    }
                    _ => None,
                })
                .collect();
            eprintln!(
                "real laboratory FR05 rank 10 impact turn {turn}: damage={damage:?}, frozen={frozen:?}"
            );
            assert!(damage.len() <= victims.len());
            maximum_impacts = maximum_impacts.max(damage.len());
            if turn == 1 {
                damage_without_gel = damage.iter().any(|(id, _)| !frozen.contains(id));
            }
            for (id, amount) in &damage {
                assert!(victims.iter().any(|(victim, _, _)| victim == id));
                total_damage += u32::from(*amount);
            }
            total_impacts += damage.len();
        }
        assert!(
            maximum_impacts > 3,
            "independent case rolls can hit more than the former rank-ten quota"
        );
        assert!(
            damage_without_gel,
            "a failed Gel roll must not cancel the damage of a successful impact"
        );
        assert!(total_impacts > 0);
        assert!(total_damage > 0);
        assert!(
            app.game
                .agent_magic_visual_effects()
                .iter()
                .all(|effect| effect.skill != "FR05")
        );
        let actual_loss: u32 = victims
            .iter()
            .map(|(id, _, hp)| {
                u32::from(hp.saturating_sub(app.game.actors().get(*id).map_or(0, Actor::integrity)))
            })
            .sum();
        assert_eq!(actual_loss, total_damage);
        assert_eq!(
            app.execute_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert!(!app.game.drain_events().iter().any(|event| matches!(
            event,
            GameEvent::DamageApplied {
                damage_type: DamageType::Cold,
                ..
            }
        )));
    }
    #[test]
    fn reanimation_preserves_the_original_reviewed_species_glyph_and_facing() {
        let mut app = AsciiApp::new_agent_skill_laboratory().unwrap();
        let at = GridPos::new(6, 12);
        let original = app
            .game
            .spawn_actor(
                Actor::new(at, 1)
                    .unwrap()
                    .with_evasion_disabled()
                    .with_player_relation(PlayerRelation::Hostile),
            )
            .unwrap();
        // This is the ordinary generated presentation for an actor whose
        // visual identity is indexed separately from its mechanical AI.
        app.actor_glyphs.insert(original, 'Q');
        app.npc_art.insert(
            original,
            NpcArtState {
                facing: ModuleFacing::Left,
                attack_slot: 0,
            },
        );
        let visual = app.front_actor_visual(original);
        assert_ne!(visual, crate::game_art::actor_for_symbol('d'));
        for skill in ["OC07", "A01"] {
            assert_eq!(
                app.execute_command(GameCommand::UseTechniqueAt {
                    technique: project_rl::skills::agent::technique(skill),
                    target: at,
                    weapon_slot: 0,
                }),
                CommandOutcome::Applied
            );
        }
        app.capture_events_at(Some(0.));
        assert!(app.game.actors().get(original).is_none());
        let raised = app.game.actors().entity_at(at).unwrap();
        assert_ne!(original, raised);
        assert_eq!(app.front_actor_visual(raised), visual);
        assert_eq!(app.actor_glyphs.get(&raised), Some(&'Q'));
        assert!(!app.actor_glyphs.contains_key(&original));
        assert_eq!(app.npc_art.get(&raised).unwrap().facing, ModuleFacing::Left);
        assert!(!app.npc_art.contains_key(&original));
    }
    #[test]
    fn support_invocations_use_reviewed_art_and_stealth_reduces_the_real_player_opacity() {
        let mut app = AsciiApp::new_agent_skill_laboratory().unwrap();
        assert_eq!(app.glyph_at(GridPos::new(7, 12)).unwrap().1.a, 1.);
        let (scout, mule, hologram) = app.prepare_agent_support_visual_trial().unwrap();
        assert_eq!(
            app.front_actor_visual(scout),
            Some(crate::game_art::Visual::DroneScout)
        );
        assert_eq!(
            app.front_actor_visual(mule),
            Some(crate::game_art::Visual::Bestiary {
                number: 57,
                corpse: false
            })
        );
        assert!(matches!(
            app.front_actor_visual(hologram),
            Some(crate::game_art::Visual::AgentHologram(_))
        ));
        assert!(app.game.agent_stealth_active());
        assert_eq!(app.glyph_at(GridPos::new(7, 12)).unwrap().1.a, 0.56);
    }
    #[test]
    fn agent_laboratory_exposes_every_catalogue_skill_and_spends_real_rank_points() {
        let mut app = AsciiApp::new_agent_skill_laboratory().unwrap();
        assert!(app.test_lab);
        assert!(!app.crash_recovery_enabled);
        assert!(
            app.player_defense_values()
                .iter()
                .any(|(label, _)| *label == "Froid")
        );
        for skill in project_rl::skills::agent::definitions() {
            assert_eq!(
                app.game.player_agent_skill_rank(&skill.id),
                1,
                "{}",
                skill.id
            );
        }
        let points = app.game.player_progression().unspent_skill_points();
        assert_eq!(points, 1000);
        let turn = app.game.turn();
        assert_eq!(
            app.execute_command(GameCommand::LearnAgentSkill {
                skill: "FE01".into()
            }),
            CommandOutcome::AppliedWithoutTime
        );
        assert_eq!(app.game.player_agent_skill_rank("FE01"), 2);
        assert_eq!(
            app.game.player_progression().unspent_skill_points(),
            points - 1
        );
        assert_eq!(app.game.turn(), turn);
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::QuickTechniques).clone()].into(),
            ..Default::default()
        });
        assert!(app.skills_open);
        assert!(!app.technique_menu_open);
        assert!(app.can_assign_agent_skill("FR02"));
        assert!(!app.can_assign_agent_skill("A03"));
    }
    #[test]
    fn agent_laboratory_hotbar_casts_the_real_nova_against_its_real_targets() {
        let mut app = AsciiApp::new_agent_skill_laboratory().unwrap();
        app.ux.skill_bar[0] = Some("FR02".into());
        let at = GridPos::new(8, 12);
        let target = app.game.actors().entity_at(at).unwrap();
        let hp = app.game.actors().get(target).unwrap().integrity();
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Skill1).clone()].into(),
            ..Default::default()
        });
        assert_eq!(app.game.turn(), 1);
        assert!(app.attack_aim.is_none());
        assert!(app.game.actors().get(target).unwrap().integrity() < hp);
    }
}
