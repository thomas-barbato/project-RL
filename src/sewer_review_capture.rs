//! EG01 native proposal only. No gameplay flag or campaign transition is added.
use super::*;

impl AsciiApp {
    fn sewer_review_actor(&self, member: &crate::sewer_review::Member) -> Result<Actor, String> {
        let world = self
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .ok_or("Surface profiles absent")?;
        let biome = world
            .biome(
                &if member.level == 1 {
                    "core:human_habitat"
                } else {
                    "core:surface_wilds"
                }
                .parse()
                .unwrap(),
            )
            .ok_or("Ordinary profiles absent")?;
        let source = &biome.encounters().rules()[usize::from(member.number == 24)];
        let mut actor = Actor::new(
            GridPos::new(member.at[0], member.at[1]),
            source.maximum_integrity(),
        )
        .map_err(|e| e.to_string())?
        .with_attack(source.attack())
        .with_ai(source.ai())
        .with_player_relation(source.player_relation())
        .with_tags(source.tags().iter().cloned());
        if let Some(a) = source.primary_attributes() {
            actor = actor.with_primary_attributes(a);
        }
        if let Some(b) = source.body_profile() {
            actor = actor.with_body_profile(b);
        }
        if let Some(mut r) = source.defeat_reward() {
            r.threat_level = member.level;
            actor = actor.with_defeat_reward(r);
        }
        Ok(actor.with_tags([
            format!("core:monster_art_{}_{}", member.number, member.variant)
                .parse()
                .unwrap(),
            format!("core:encounter_level_{}", member.level)
                .parse()
                .unwrap(),
            "core:sewer_visual_candidate".parse().unwrap(),
        ]))
    }
    fn sewer_review_base() -> Result<Self, String> {
        let (mut rules, texts, loot, expeditions) = ascii_game_content()?;
        let classes = ascii_character_class_catalog()?;
        let id: CharacterClassId = "core:bastion".parse().unwrap();
        let class = classes.get(&id).ok_or("Bastion absent")?;
        class
            .apply_to_rules(&mut rules, class.recommended_attributes())
            .map_err(|e| e.to_string())?;
        weapon_supplies_app::ensure_class_weapon_supplies(&mut rules, CURRENT_GENERATION_VERSION);
        let mut app = Self::from_seed_version(
            1872,
            rules,
            texts,
            loot,
            expeditions,
            CURRENT_GENERATION_VERSION,
        )?;
        app.character_class = Some(id);
        app.crash_recovery_enabled = false;
        app.suspension_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
            "artifacts/sewer-review-{:02}/isolated-capture.json",
            crate::sewer_review::revision()
        ));
        app.select_weapon_slot(app.preferred_starting_weapon_slot());
        app.open_menu(MenuScreen::Hidden);
        app.character_creation = None;
        app.selected_world_cell = None;
        app.selected_target = None;
        app.world_cursor = None;
        app.graphics.active.world_cell_px = 64;
        Ok(app)
    }
    pub async fn capture_sewer_review(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        macroquad::miniquad::window::set_fullscreen(false);
        request_new_screen_size(1920., 1080.);
        for _ in 0..3 {
            next_frame().await;
        }
        let mut scenes = Vec::new();
        let mut street_scenes = Vec::new();
        if crate::sewer_review::revision() < 4 {
            let mut app = Self::sewer_review_base()?;
            let exit = pos(camp::scene().exits[0]);
            app.walk_fixture_to_unchecked(exit)?;
            if app.execute_command(GameCommand::Interact { target: exit })
                != CommandOutcome::Applied
            {
                return Err("Street entry failed".into());
            }
            app.walk_fixture_to_unchecked(GridPos::new(3, 15))?;
            app.world_cursor = None;
            app.selected_target = None;
            app.selected_world_cell = None;
            app.log.clear();
            crate::sewer_review::select(None);
            app.world_fix_frame(output, "entrance-before").await?;
            crate::sewer_review::select(Some(0));
            app.world_fix_frame(output, "entrance-proposal").await?;
            for index in 1..=2 {
                let plan = crate::sewer_review::street_plan(index);
                for view in &plan.views {
                    let mut app = Self::sewer_review_base()?;
                    let exit = pos(camp::scene().exits[0]);
                    app.walk_fixture_to_unchecked(exit)?;
                    if app.execute_command(GameCommand::Interact { target: exit })
                        != CommandOutcome::Applied
                    {
                        return Err("Street population entry failed".into());
                    }
                    app.walk_fixture_to_unchecked(GridPos::new(view.at[0], view.at[1]))?;
                    for group in &plan.groups {
                        for member in &group.members {
                            let actor = app.sewer_review_actor(member)?;
                            app.game
                                .spawn_actor(actor)
                                .map_err(|e| format!("POP-D2-02 {:?}: {e}", member.at))?;
                        }
                    }
                    app.game.drain_events();
                    app.log.clear();
                    app.world_cursor = None;
                    app.selected_world_cell = None;
                    app.selected_target = None;
                    let label = format!("street-{index}-{}", view.id);
                    app.world_fix_frame(output, &label).await?;
                    street_scenes.push(serde_json::json!({"id":label,"seed":plan.seed,"groups":plan.groups.len(),"monsters":plan.groups.iter().map(|g|g.members.len()).sum::<usize>()}));
                }
            }
        }
        for index in 1..=3 {
            let plan = crate::sewer_review::plan(index);
            for view in &plan.views {
                let mut app = Self::sewer_review_base()?;
                let mut map = project_rl::world::Map::filled(
                    plan.width,
                    plan.height,
                    project_rl::world::Terrain::Wall,
                )
                .map_err(|e| e.to_string())?;
                for (y, row) in plan.layout.iter().enumerate() {
                    for (x, c) in row.bytes().enumerate() {
                        let at = GridPos::new(x as i32, y as i32);
                        if c != b'#' {
                            map.set_terrain(at, project_rl::world::Terrain::Floor)
                                .map_err(|e| e.to_string())?;
                        }
                        if c == b'~' {
                            map.set_obstruction(at, true, false)
                                .map_err(|e| e.to_string())?;
                        }
                    }
                }
                for at in &plan.blocked {
                    map.set_obstruction(GridPos::new(at[0], at[1]), true, false)
                        .map_err(|e| e.to_string())?;
                }
                let player = GridPos::new(view.at[0], view.at[1]);
                let game =
                    GameState::new_with_rules(map, player, plan.seed, app.game.rules().clone())
                        .map_err(|e| e.to_string())?;
                app.terminal =
                    TerminalView::new(SectorDecor::default(), game.map(), game.player_visibility());
                app.terminal.observe(game.map(), game.player_visibility());
                app.game = WorldState::single(game);
                app.game
                    .enable(project_rl::game::ZoneInfo {
                        id: "core:sewer_review".parse().unwrap(),
                        name: "Égouts · aperçu".into(),
                        kind: "core:dungeon".parse().unwrap(),
                        depth: 1,
                    })
                    .map_err(|e| e.to_string())?;
                let mut count = 0;
                for group in &plan.groups {
                    for member in &group.members {
                        let at = GridPos::new(member.at[0], member.at[1]);
                        if at == player {
                            return Err("Capture player overlaps a candidate enemy".into());
                        }
                        if !app.game.map().is_walkable(at) {
                            return Err(format!("Invalid sewer monster at {at:?}"));
                        }
                        let actor = app.sewer_review_actor(member)?;
                        app.game.spawn_actor(actor).map_err(|e| e.to_string())?;
                        count += 1;
                    }
                }
                app.game.drain_events();
                app.log.clear();
                app.actor_glyphs.clear();
                app.zone_views.clear();
                app.zone_decor.clear();
                app.terminal.title = "Égouts · aperçu".into();
                crate::sewer_review::select(Some(index));
                let label = format!("seed-{index}-{}", view.id);
                app.world_fix_frame(output, &label).await?;
                if view.id == "access" {
                    if crate::sewer_review::revision() == 4 {
                        crate::prison_street_render::export_review_world(
                            &app.game,
                            &output.join(format!("seed-{index}-scenery.png")),
                        )?;
                    }
                    crate::prison_street_render::export_review_world(
                        &app.game,
                        &output.join(format!("seed-{index}-map.png")),
                    )?;
                }
                if index == 1 && view.id == "collector" {
                    for frame in 0..12 {
                        let _time = crate::effect_geometry::frame_time(f64::from(frame) * 0.22);
                        app.world_presentation_time.set(f64::from(frame) * 0.22);
                        app.draw();
                        next_frame().await;
                        app.draw();
                        crate::ui_capture::framebuffer()?.export_png(
                            output
                                .join(format!("water-{frame:02}.png"))
                                .to_str()
                                .unwrap(),
                        );
                    }
                }
                scenes.push(serde_json::json!({"id":label,"seed":plan.seed,"monsters":count,"groups":plan.groups.iter().map(|g|serde_json::json!({"id":g.id,"family":g.family})).collect::<Vec<_>>(),"player":view.at,"arrival":plan.spawn}));
            }
        }
        crate::sewer_review::select(None);
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "review":crate::sewer_review::review_id(),
            "status":"candidate_not_integrated","approved_base":crate::sewer_review::enriched(),
            "resurrection_ai_active":false,"native_cell":64,"scenes":scenes,"street_population":street_scenes,
            "real_engine_lighting":true,"existing_actor_art_unchanged":true,"simulation_not_advanced_for_capture":true,
            "balance_or_xp_changed":false,"normal_map_changed":false,"entry":[2,14],"interactive_transition_added":false
        })).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }
}
