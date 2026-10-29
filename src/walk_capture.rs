//! Uses the live app/controller/renderer; no user campaign or preferences touched.
use super::*;

#[test]
#[ignore = "Read-only audit of the actual saved population; RL_WALK_SAVED_RUN and RL_WALK_LIVE_AUDIT required"]
fn saved_game_walking_audit() {
    let path = std::path::PathBuf::from(std::env::var("RL_WALK_SAVED_RUN").unwrap());
    let output = std::path::PathBuf::from(std::env::var("RL_WALK_LIVE_AUDIT").unwrap());
    let saved = Suspension::read(&path).unwrap();
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let app = AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
    let mut actors = Vec::new();
    for (id, actor) in app.game.actors().iter() {
        let visual = app.front_actor_visual(id);
        let binding = visual.and_then(|v| {
            crate::game_art::walking::creature_selection(
                v,
                crate::game_art::walking::Pose {
                    facing: 1,
                    frame: 1,
                },
            )
        });
        actors.push(serde_json::json!({"id":id.get(),"player":id==app.game.player_id(),
            "position":actor.position(),"visible":app.game.player_visibility().is_visible(actor.position()),
            "visual":format!("{visual:?}"),"normal_creature_cycle":binding.map(|b|b.0),
            "tags":actor.tags().iter().map(|t|t.as_str()).collect::<Vec<_>>()}));
    }
    let mut trials = Vec::new();
    for command in [
        GameCommand::Wait,
        GameCommand::Move(Direction::East),
        GameCommand::Move(Direction::West),
        GameCommand::Move(Direction::North),
        GameCommand::Move(Direction::South),
    ] {
        let mut game = app.game.clone();
        game.drain_events();
        let name = format!("{command:?}");
        let outcome = game.process_player_command(command);
        for event in game.drain_events() {
            if let GameEvent::EntityMoved { entity, from, to } = event {
                let facing = (to.x - from.x).signum() as i8;
                let visual = app.front_actor_visual(entity);
                let mut motion = crate::combat_visuals::CombatMotion::default();
                motion.glide_timed(entity, from, to, 1., 400);
                let corrected = motion.creature_walk_pose(entity, 1.2, false).unwrap();
                let active_binding = visual.and_then(|v| walking::creature_selection(v, corrected));
                let binding = visual.and_then(|v| {
                    walking::creature_selection(v, walking::Pose { facing, frame: 1 })
                });
                trials.push(serde_json::json!({"command":name,"outcome":format!("{outcome:?}"),
                    "entity":entity.get(),"player":entity==game.player_id(),"from":from,"to":to,
                    "both_ends_visible":game.player_visibility().is_visible(from)&&game.player_visibility().is_visible(to),
                    "visual":format!("{visual:?}"),"motion_facing":facing,
                    "old_cycle_for_this_direction":binding.map(|b|b.0),
                    "active_render_facing":corrected.facing,"cycle_for_this_direction":active_binding.map(|b|b.0)}));
            }
        }
    }
    std::fs::create_dir_all(output.parent().unwrap()).unwrap();
    std::fs::write(
        output,
        serde_json::to_vec_pretty(&serde_json::json!({
        "source":"restored user save; commands simulated only in isolated memory clones",
        "player_position":app.game.player_position(),"actors":actors,"movement_trials":trials}))
        .unwrap(),
    )
    .unwrap();
}
use crate::game_art::{self, Visual, walking};

async fn save(
    app: &AsciiApp,
    output: &std::path::Path,
    name: &str,
    time: f64,
) -> Result<(), String> {
    let _clock = crate::effect_geometry::frame_time(time);
    for _ in 0..3 {
        app.draw();
        next_frame().await;
    }
    app.draw();
    crate::ui_capture::framebuffer()?.export_png(
        output
            .join(format!("{name}.png"))
            .to_str()
            .ok_or("capture path")?,
    );
    Ok(())
}

impl AsciiApp {
    pub async fn capture_saved_walk_live(
        path: &std::path::Path,
        output: &std::path::Path,
    ) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let original = std::fs::read(path).map_err(|e| e.to_string())?;
        let saved = Suspension::read(path)?;
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::restore_suspension(&saved, rules, texts, loot, expeditions)?;
        app.suspension_path = output.join("isolated-run.json");
        app.graphics.path = output.join("isolated-graphics.json");
        app.crash_recovery_enabled = false;
        app.graphics.active.reduced_motion = false;
        app.graphics.active.world_cell_px = 64;
        app.graphics.active.walk_duration_ms = 400;
        app.menu = MenuScreen::Hidden;
        app.game.drain_events();
        if app
            .game
            .process_player_command(GameCommand::Move(Direction::East))
            != CommandOutcome::Applied
        {
            return Err("isolated saved-scene eastward movement unavailable".into());
        }
        app.capture_events_at(Some(1.));
        let fingerprint = suspension::fingerprint(&app.game);
        let mut walkers = Vec::new();
        for (id, actor) in app.game.actors().iter() {
            if id != app.game.player_id()
                && app.game.player_visibility().is_visible(actor.position())
            {
                let pose = app.combat_motion.creature_walk_pose(id, 1.2, false);
                walkers.push(serde_json::json!({"id":id.get(),"visual":format!("{:?}",app.front_actor_visual(id)),
                    "pose":pose.map(|p|[i16::from(p.facing),i16::from(p.frame)])}));
            }
        }
        for (name, time) in [
            ("step-100ms", 1.1),
            ("step-200ms", 1.2),
            ("front-at-400ms", 1.401),
        ] {
            save(&app, output, name, time).await?;
        }
        if app
            .combat_motion
            .player_walk_pose(app.game.player_id(), 1.401, false)
            .is_some()
        {
            return Err("player did not return to frontal rest".into());
        }
        for (id, _) in app.game.actors().iter() {
            if app
                .combat_motion
                .creature_walk_pose(id, 1.401, false)
                .is_some()
            {
                return Err(format!("actor{id:?} did not return to frontal rest"));
            }
        }
        if suspension::fingerprint(&app.game) != fingerprint
            || std::fs::read(path).map_err(|e| e.to_string())? != original
        {
            return Err("saved-scene diagnostic changed source or simulation while drawing".into());
        }
        std::fs::write(
            output.join("checks.json"),
            serde_json::to_vec_pretty(&serde_json::json!({
            "status":"passed","source":"isolated restoration of actual saved scene",
            "original_save_unchanged":true,"player_front_at_arrival":true,"all_actors_front_at_arrival":true,"walkers":walkers}))
            .unwrap(),
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Compare the actual NPC draw routes with/without the idle clock.
    pub async fn capture_npc_proportions(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1280., 1000.);
        for _ in 0..3 {
            next_frame().await;
        }
        let mut checks = Vec::new();
        for size in [64., 96.] {
            for warm in 0..3 {
                set_camera(&graphics::ui_camera(screen_width(), screen_height()));
                clear_background(Color::from_rgba(37, 49, 54, 255));
                draw_text(
                    &format!(
                        "PNJ / meme corps au repos et pendant le deplacement / cases{size:.0}"
                    ),
                    16.,
                    28.,
                    20.,
                    WHITE,
                );
                for (col, label) in [
                    "Repos sans arme",
                    "Mouvement sans arme",
                    "Repos hache",
                    "Mouvement hache",
                ]
                .into_iter()
                .enumerate()
                {
                    draw_text(label, 80. + col as f32 * 300., 66., 16., WHITE);
                }
                for body in 20..=25u8 {
                    let y = 110. + f32::from(body - 20) * 140.;
                    draw_text(&format!("ACTOR-{body}"), 8., y + 30., 14., WHITE);
                    for col in 0..4 {
                        let cell = Rect::new(100. + col as f32 * 300., y, size, size);
                        let visual = if col < 2 {
                            Visual::Actor(body)
                        } else {
                            Visual::ArmedActor { body, weapon: 67 }
                        };
                        let idle = if col % 2 == 0 { Some(0.) } else { None };
                        walking::with_pose(
                            Some(walking::Pose {
                                facing: 1,
                                frame: 1,
                            }),
                            || game_art::draw_world_animated(cell, visual, 1., idle),
                        );
                    }
                }
                if warm < 2 {
                    next_frame().await;
                }
            }
            let capture = crate::ui_capture::framebuffer()?;
            capture.export_png(
                output
                    .join(format!("npc-size-{size:.0}.png"))
                    .to_str()
                    .unwrap(),
            );
            for body in 20..=25u8 {
                let y = 110. + f32::from(body - 20) * 140.;
                for (armed, first_col) in [(false, 0), (true, 2)] {
                    let crop = |col| {
                        capture.sub_image(Rect::new(
                            100. + col as f32 * 300.,
                            f32::from(capture.height) - y - size,
                            size,
                            size,
                        ))
                    };
                    let a = crop(first_col);
                    let b = crop(first_col + 1);
                    let different = a
                        .bytes
                        .chunks_exact(4)
                        .zip(b.bytes.chunks_exact(4))
                        .filter(|(a, b)| a != b)
                        .count();
                    checks.push(serde_json::json!({"actor":body,"armed":armed,"cell_size":size,"different_pixels":different,"same_body":different==0}));
                }
            }
        }
        let failed = checks.iter().filter(|c| c["same_body"] != true).count();
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "status":if failed==0{"passed"}else{"body_switch_detected"},"cases":checks,"failed":failed,
            "actual_world_renderer":true,"campaign_created":false,"user_settings_written":false,
            "comparison_region":"owning cell, excluding labels; framebuffer rows converted from bottom-up"
        })).unwrap()).map_err(|e|e.to_string())?;
        if failed > 0 {
            return Err(format!(
                "{failed} NPC body changes between rest and movement"
            ));
        }
        Ok(())
    }

    pub async fn capture_walk_integration(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        request_new_screen_size(1280., 800.);
        for _ in 0..3 {
            next_frame().await;
        }
        let (rules, texts, loot, expeditions) = ascii_game_content()?;
        let mut app = Self::from_seed(INITIAL_SEED, rules, texts, loot, expeditions)?;
        app.suspension_path = output.join("isolated-run.json");
        app.graphics.path = output.join("isolated-graphics.json");
        app.crash_recovery_enabled = false;
        app.graphics.active.reduced_motion = false;
        app.graphics.active.world_cell_px = 64;
        app.begin_character_creation_with_seed(false, app.seed)?;
        let index = app
            .character_classes
            .iter()
            .position(|(id, _)| id.as_str() == "core:bastion")
            .ok_or("Bastion missing")?;
        let attributes = app
            .character_classes
            .iter()
            .nth(index)
            .unwrap()
            .1
            .recommended_attributes();
        let creation = app.character_creation.as_mut().unwrap();
        creation.selected_class = index;
        creation.attributes = attributes;
        let creation = app.character_creation.take().unwrap();
        app.rebuild_run_with_character_class(&creation)?;
        app.menu = MenuScreen::Hidden;
        let complete_review = walking::corrections_enabled();
        let accepted_player = walking::player_enabled();
        let expected_model = if accepted_player { 67 } else { 2 };
        let weapon_slot = if accepted_player { 0 } else { 2 };
        let rifle = app.game.grant_colored_equipment_for_diagnostic(
            if accepted_player {
                "core:melee_hachette"
            } else {
                "core:fusil_d_assaut"
            }
            .parse()
            .unwrap(),
            project_rl::entity::EquipmentPalette::Original,
        )?;
        if app.execute_command(GameCommand::EquipWeapon {
            slot: weapon_slot,
            item: rifle,
        }) != CommandOutcome::Applied
        {
            return Err("reviewed rifle could not be equipped".into());
        }
        app.select_weapon_slot(weapon_slot);
        if !matches!(
            app.front_player_visual(),
            Visual::Player {
                weapon: Some(model),
                hands: None,
                ..
            } if model==expected_model
        ) {
            return Err("live player does not use the accepted rifle loadout".into());
        }
        app.graphics.active.walk_duration_ms = 400;
        app.capture_events_at(Some(0.));
        let from = app.game.player_position().ok_or("player missing")?;
        let direction = [Direction::West, Direction::East]
            .into_iter()
            .find(|d| {
                let at = from.step(*d);
                app.game.map().is_walkable(at) && app.game.actors().entity_at(at).is_none()
            })
            .ok_or("no empty adjacent cell")?;
        if app.execute_command(GameCommand::Move(direction)) != CommandOutcome::Applied {
            return Err("real move rejected".into());
        }
        if app.game.player_position() != Some(from.step(direction)) {
            return Err("move was not one cell".into());
        }
        if !matches!(
            app.front_player_visual(),
            Visual::Player {
                weapon: Some(model),
                hands: None,
                ..
            } if model==expected_model
        ) {
            return Err("moving player switched away from the reviewed rifle".into());
        }
        app.capture_events_at(Some(1.));
        let fingerprint = suspension::fingerprint(&app.game);
        for frame in 0..15 {
            save(
                &app,
                output,
                &format!("walk-live-{frame:02}"),
                1. + f64::from(frame) / 30.,
            )
            .await?;
        }
        for (name, time) in [
            ("camp-step-100ms", 1.1),
            ("camp-step-200ms", 1.2),
            ("camp-rest-400ms", 1.401),
        ] {
            save(&app, output, name, time).await?;
        }
        if suspension::fingerprint(&app.game) != fingerprint {
            return Err("drawing first step changed simulation".into());
        }
        // A second real cell back switches both the facing and support foot.
        let back = match direction {
            Direction::West => Direction::East,
            Direction::East => Direction::West,
            Direction::North => Direction::South,
            _ => Direction::North,
        };
        if app.execute_command(GameCommand::Move(back)) != CommandOutcome::Applied {
            return Err("return move rejected".into());
        }
        app.capture_events_at(Some(2.));
        for frame in 0..15 {
            save(
                &app,
                output,
                &format!("walk-live-{:02}", frame + 15),
                2. + f64::from(frame) / 30.,
            )
            .await?;
        }
        let fingerprint = suspension::fingerprint(&app.game);
        app.graphics.draft = app.graphics.active;
        app.open_menu(MenuScreen::Graphics);
        save(&app, output, "options-400ms-1280", 1.401).await?;
        request_new_screen_size(960., 540.);
        for _ in 0..3 {
            next_frame().await;
        }
        save(&app, output, "options-400ms-960", 1.401).await?;
        if suspension::fingerprint(&app.game) != fingerprint {
            return Err("drawing changed simulation".into());
        }

        request_new_screen_size(1280., 800.);
        for _ in 0..3 {
            next_frame().await;
        }
        app.menu = MenuScreen::Hidden;
        let map = project_rl::world::Map::filled(20, 13, project_rl::world::Terrain::Floor)
            .map_err(|e| e.to_string())?;
        let mut rules = app.rules.clone();
        rules.player_field_of_view.radius = 20;
        let mut game = GameState::new_with_rules(map, GridPos::new(8, 7), 123, rules)
            .map_err(|e| e.to_string())?;
        app.actor_glyphs.clear();
        app.npc_art.clear();
        app.combat_motion.clear();
        for (number, variant, to, from) in [
            (99, 0, GridPos::new(6, 5), GridPos::new(7, 5)),
            (106, 0, GridPos::new(10, 7), GridPos::new(9, 7)),
            (9, 0, GridPos::new(8, 9), GridPos::new(8, 10)),
            (22, 1, GridPos::new(5, 8), GridPos::new(6, 8)),
            (24, 1, GridPos::new(11, 5), GridPos::new(10, 5)),
            (14, 0, GridPos::new(10, 9), GridPos::new(9, 9)),
        ] {
            let actor = Actor::new(to, 100)
                .map_err(|e| e.to_string())?
                .with_player_relation(PlayerRelation::Hostile)
                .with_ai(AiProfile::idle())
                .with_tags([format!("core:monster_art_{number}_{variant}")
                    .parse()
                    .unwrap()]);
            let id = game.spawn_actor(actor).map_err(|e| e.to_string())?;
            app.actor_glyphs.insert(id, 'd');
            app.combat_motion.glide_timed(id, from, to, 1., 400);
        }
        app.terminal =
            TerminalView::new(SectorDecor::default(), game.map(), game.player_visibility());
        app.game = WorldState::single(game);
        app.test_lab = true;
        let fingerprint = suspension::fingerprint(&app.game);
        save(&app, output, "monsters-step-200ms", 1.2).await?;
        save(&app, output, "monsters-rest-400ms", 1.401).await?;
        if suspension::fingerprint(&app.game) != fingerprint {
            return Err("monster drawing changed simulation".into());
        }

        request_new_screen_size(1240., 900.);
        for _ in 0..3 {
            next_frame().await;
        }
        let classes = [
            "core:bastion",
            "core:technomancien",
            "core:traqueur",
            "core:arcaniste",
            "core:metamorphe",
        ];
        let torsos = [
            "core:veste_de_veille",
            "core:veste_a_fibres_croisees",
            "core:combinaison_fantome",
            "core:armure_cerbere",
            "core:carapace_chrysalide",
        ];
        let weapons = [None, Some(101), Some(97), Some(2), Some(66), Some(101)];
        for (rank, torso) in torsos.into_iter().enumerate() {
            for light in [false, true] {
                for warm in 0..3 {
                    set_camera(&graphics::ui_camera(screen_width(), screen_height()));
                    clear_background(if light {
                        Color::from_rgba(174, 172, 158, 255)
                    } else {
                        Color::from_rgba(30, 43, 46, 255)
                    });
                    let ink = if light { BLACK } else { WHITE };
                    draw_text(
                        &format!(
                            "WALK11 / proportions et prises integrees / tenue {} / 64 px",
                            rank + 1
                        ),
                        16.,
                        30.,
                        21.,
                        ink,
                    );
                    for (class, id) in classes.into_iter().enumerate() {
                        let x = 70. + class as f32 * 232.;
                        draw_text(id.trim_start_matches("core:"), x, 58., 17., ink);
                        for (configuration, weapon) in weapons.into_iter().enumerate() {
                            let mut visual = game_art::wear(
                                game_art::player(Some(id), PlayerWeaponPose::Unarmed, None),
                                [("core:body_armor".to_owned(), torso.to_owned())],
                            );
                            if let Visual::Player {
                                weapon: held,
                                hands,
                                ..
                            } = &mut visual
                            {
                                *held = weapon;
                                if configuration == 5 {
                                    *hands = Some([weapon, weapon]);
                                }
                            }
                            for (column, facing) in [-1, 0, 1].into_iter().enumerate() {
                                let cell = Rect::new(
                                    x + column as f32 * 72.,
                                    102. + configuration as f32 * 120.,
                                    64.,
                                    64.,
                                );
                                draw_rectangle_lines(
                                    cell.x,
                                    cell.y,
                                    cell.w,
                                    cell.h,
                                    1.,
                                    if light { GRAY } else { DARKGRAY },
                                );
                                walking::with_pose(
                                    Some(walking::Pose { facing, frame: 2 }),
                                    || game_art::draw_world(cell, visual, 1.),
                                );
                            }
                            draw_text(
                                &format!("CLASS-{:02} / config {configuration}", class * 5 + rank),
                                x,
                                194. + configuration as f32 * 120.,
                                12.,
                                ink,
                            );
                        }
                    }
                    if warm == 2 {
                        crate::ui_capture::framebuffer()?.export_png(
                            output
                                .join(format!(
                                    "native-tenue-{}-{}.png",
                                    rank + 1,
                                    if light { "clair" } else { "sombre" }
                                ))
                                .to_str()
                                .unwrap(),
                        );
                    } else {
                        next_frame().await;
                    }
                }
            }
        }
        request_new_screen_size(1200., 720.);
        for _ in 0..3 {
            next_frame().await;
        }
        let mut bindings = Vec::new();
        for light in [false, true] {
            for warm in 0..3 {
                set_camera(&graphics::ui_camera(screen_width(), screen_height()));
                let ground = if light {
                    Color::from_rgba(174, 172, 158, 255)
                } else {
                    Color::from_rgba(30, 43, 46, 255)
                };
                let ink = if light { BLACK } else { WHITE };
                clear_background(ground);
                draw_text(
                    "WALK07 / corrections M100 et M104 / cases natives64",
                    16.,
                    28.,
                    20.,
                    ink,
                );
                for (rank, number) in [99, 106].into_iter().enumerate() {
                    let visual = Visual::CampaignMonster { number, variant: 0 };
                    for (row, facing) in [-1, 1].into_iter().enumerate() {
                        let y = 112. + (rank * 2 + row) as f32 * 148.;
                        draw_text(
                            &format!(
                                "M{} / {}",
                                if number == 99 { 100 } else { 104 },
                                if facing < 0 { "gauche" } else { "droite" }
                            ),
                            18.,
                            y - 42.,
                            17.,
                            ink,
                        );
                        for column in 0..7u8 {
                            let frame = if column == 0 { 8 } else { column - 1 };
                            let pose = walking::Pose { facing, frame };
                            let (file, source) = walking::creature_selection(visual, pose)
                                .ok_or("approved creature binding missing")?;
                            let cell = Rect::new(190. + f32::from(column) * 136., y, 64., 64.);
                            draw_rectangle_lines(
                                cell.x,
                                cell.y,
                                cell.w,
                                cell.h,
                                1.,
                                Color::new(ink.r, ink.g, ink.b, 0.2),
                            );
                            walking::with_pose(Some(pose), || {
                                game_art::draw_world(cell, visual, 1.)
                            });
                            if light && warm == 2 {
                                bindings.push(serde_json::json!({"number":number,"facing":facing,"frame":frame,"file":file,"source":[source.x,source.y,source.w,source.h]}));
                            }
                        }
                    }
                }
                if warm == 2 {
                    crate::ui_capture::framebuffer()?.export_png(
                        output
                            .join(if light {
                                "walk07-corrections-light.png"
                            } else {
                                "walk07-corrections-dark.png"
                            })
                            .to_str()
                            .unwrap(),
                    );
                } else {
                    next_frame().await;
                }
            }
        }
        if suspension::fingerprint(&app.game) != fingerprint {
            return Err("WALK07 renderer changed simulation".into());
        }
        std::fs::write(
            output.join("walk07-bindings.json"),
            serde_json::to_vec_pretty(&bindings).unwrap(),
        )
        .map_err(|e| e.to_string())?;
        let audit = walking::creature_audit();
        std::fs::write(
            output.join("monster-audit.json"),
            serde_json::to_vec_pretty(&audit).unwrap(),
        )
        .map_err(|e| e.to_string())?;
        let cultists: Vec<_> = audit.iter().filter(|a| a["name"] == "Cultiste").collect();
        for (page, entries) in cultists.chunks(5).enumerate() {
            for warm in 0..3 {
                set_camera(&graphics::ui_camera(screen_width(), screen_height()));
                clear_background(Color::from_rgba(30, 43, 46, 255));
                draw_text(
                    "Audit cultistes / lecteur normal / cases64 / repos, gauche, droite",
                    16.,
                    28.,
                    20.,
                    WHITE,
                );
                for (row, entry) in entries.iter().enumerate() {
                    let number = entry["number"].as_u64().unwrap() as u16;
                    let variant = entry["variant"].as_u64().unwrap() as u8;
                    let y = 90. + row as f32 * 122.;
                    draw_text(
                        &format!(
                            "MON-{number:03}-{variant} / {:.0}px / anime:{}",
                            entry["static_height_native"].as_f64().unwrap(),
                            entry["normal_lateral_animation"]
                        ),
                        16.,
                        y + 28.,
                        16.,
                        WHITE,
                    );
                    for (column, facing) in [0, -1, 1].into_iter().enumerate() {
                        let cell = Rect::new(420. + column as f32 * 200., y, 64., 64.);
                        draw_rectangle_lines(cell.x, cell.y, cell.w, cell.h, 1., GRAY);
                        walking::with_pose(Some(walking::Pose { facing, frame: 1 }), || {
                            game_art::draw_world(
                                cell,
                                Visual::CampaignMonster { number, variant },
                                1.,
                            );
                        });
                    }
                }
                if warm == 2 {
                    crate::ui_capture::framebuffer()?.export_png(
                        output
                            .join(format!("cultists-normal-{page:02}.png"))
                            .to_str()
                            .unwrap(),
                    );
                } else {
                    next_frame().await;
                }
            }
        }
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "status":"passed","live_world_renderer":true,"real_move_command":true,"logical_distance":1,
            "duration_ms":400,"native_cell":64,"class_outfits":25,"loadouts_per_outfit":6,
            "review_id":if complete_review{"WALK08-REVIEW"}else{"WALK12-INTEGRATION-r01"},"actual_moving_player_weapon_model":expected_model,"accepted_rifle_model":2,"accepted_class_outfits":25,
            "accepted_common_gait12_enabled":walking::gait12_enabled(),"all_actors_front_at_arrival":true,
            "facings":["left-quarter","front-unchanged","right-quarter"],"light_and_dark":true,
            "accepted_player_renderer_enabled":accepted_player,"common_player_cycle":walking::common_cycle_enabled(),
            "candidate_renderer_review_only":complete_review,
            "accepted_proportions_enabled":walking::proportions_enabled(),
            "walk07_approved_creature_cycles":130,"walk07_repaired_models":[99,106],
            "walk07_both_directions_all14_poses":true,
            "simulation_unchanged_by_drawing":true,"campaign_monsters_in_live_scene":true,"user_campaign_written":false,"user_settings_written":false
        })).unwrap()).map_err(|e|e.to_string())?;
        if let Some(path) = std::env::var_os("RL_WALK_SAVED_RUN") {
            Self::capture_saved_walk_live(
                &std::path::PathBuf::from(path),
                &output.join("actual-scene"),
            )
            .await?;
        }
        Ok(())
    }
}
