//! FX-FOOTPRINT-01. Isolated native captures, using actual casts and renderers.
use super::*;
use project_rl::world::Map;

struct Scene {
    app: AsciiApp,
    preview: project_rl::combat::AttackPreview,
    label: String,
    skill: &'static str,
    rank: u8,
    corridor: bool,
    before_cast: Vec<u8>,
}

impl Scene {
    fn new(skill: &'static str, rank: u8, delta: (i32, i32)) -> Result<Self, String> {
        let corridor = skill == "FE02-COULOIR";
        let skill = if corridor { "FE02" } else { skill };
        let mut app = AsciiApp::new_agent_blast_trial()?;
        let mut rules = app.game.rules().clone();
        rules.player_maximum_integrity = 50000;
        rules.progression.starting_skill_points = 1000;
        rules
            .agent_class_start
            .as_mut()
            .ok_or("Agent missing")?
            .skills = [
            "A01", "A02", "FE01", "FE02", "FE03", "FE04", "FE05", "FR01", "FR02", "FR03", "FR04",
            "FR05",
        ]
        .map(str::to_owned)
        .to_vec();
        eprintln!("[FX review] Preparing {skill} rank {rank}");
        let map =
            Map::from_ascii(&vec![".".repeat(33); 33].join("\n")).map_err(|e| e.to_string())?;
        let origin = GridPos::new(16, 16);
        let target = GridPos::new(origin.x + delta.0, origin.y + delta.1);
        let mut game =
            GameState::new_with_rules(map, origin, 724, rules).map_err(|e| e.to_string())?;
        if matches!(skill, "FE01" | "FR01" | "A01" | "M02") {
            game.spawn_actor(
                Actor::new(target, 50000)
                    .unwrap()
                    .with_evasion_disabled()
                    .with_player_relation(PlayerRelation::Hostile),
            )
            .map_err(|e| e.to_string())?;
        }
        for _ in 1..rank {
            let outcome = game.process_player_command(GameCommand::LearnAgentSkill {
                skill: skill.into(),
            });
            if outcome != CommandOutcome::AppliedWithoutTime {
                return Err(format!("Rank purchase {skill}: {outcome:?}"));
            }
        }
        game.drain_events();
        app.terminal =
            TerminalView::new(SectorDecor::default(), game.map(), game.player_visibility());
        app.game = WorldState::single(game);
        app.actor_glyphs.clear();
        app.history.clear();
        app.selected_target = None;
        app.select_world_target(None);
        app.crash_recovery_enabled = false;
        app.suspension_path = std::env::temp_dir().join("project-rl-fx-review-unused.json");
        app.graphics.active.world_cell_px = 64;
        app.graphics.active.reduced_motion = false;
        app.combat_motion.clear();
        app.visual_cues.clear_world();
        let technique = project_rl::skills::agent::technique(skill);
        let preview = app
            .game
            .player_weapon_technique_footprint(&technique, app.active_weapon_slot, target)
            .map_err(|e| format!("Footprint {skill}: {e:?}"))?;
        let before = app
            .game
            .recovery_snapshot_bytes()
            .map_err(|e| e.to_string())?;
        // Compare real engine results with and without the visual revision.
        let mut reference =
            WorldState::from_recovery_snapshot_bytes(&before, app.game.rules().clone())?;
        let command = GameCommand::UseTechniqueAt {
            technique,
            target,
            weapon_slot: app.active_weapon_slot,
        };
        eprintln!("[FX review] Casting {skill} rank {rank}");
        let expected = reference.process_player_command(command.clone());
        let outcome = app.execute_command(command);
        if outcome != expected || matches!(outcome, CommandOutcome::Rejected(_)) {
            return Err(format!("Cast {skill}: {outcome:?} / {expected:?}"));
        }
        let events = reference.drain_events();
        let actual_events = app.game.drain_events();
        if format!("{events:?}") != format!("{actual_events:?}") {
            return Err(format!("Visual revision changed events for {skill}"));
        }
        if app
            .game
            .recovery_snapshot_bytes()
            .map_err(|e| e.to_string())?
            == before
        {
            return Err(format!("Cast {skill} did not execute"));
        }
        if app.game.recovery_snapshot_bytes()? != reference.recovery_snapshot_bytes()? {
            return Err(format!("Visual revision changed state or RNG for {skill}"));
        }

        let mut origins = BTreeMap::new();
        for event in &actual_events {
            app.capture_combat_visual(event, &mut origins, get_time());
        }
        let title = &project_rl::skills::agent::definition(skill).unwrap().title;
        Ok(Self {
            app,
            preview,
            label: format!("{title} · rang {rank}"),
            skill,
            rank,
            corridor,
            before_cast: before,
        })
    }

    fn rect(&self, at: GridPos) -> Option<Rect> {
        self.app.terminal.world_cell_rect(
            &self.app.game,
            self.app.terminal_bounds(),
            self.app.ui_scale(),
            64,
            self.app.navigation_signal_summary().is_some(),
            at,
            Vec2::ZERO,
        )
    }

    fn verify_impact_pixels(&mut self) -> Result<usize, String> {
        // Test the production impact renderers against the real pre-cast cells.
        // Projectile travel is checked separately from its single-cell impact.
        let age = if self.skill == "FE05" { 0.48 } else { 0.24 };
        let now = get_time();
        self.app.combat_motion.review_age(now - age);

        let clip = Rect::new(0., 0., 1280., 1000.);
        let rect = |p: GridPos| {
            let r = Rect::new(
                608. + (p.x - 16) as f32 * 64.,
                468. + (p.y - 16) as f32 * 64.,
                64.,
                64.,
            );
            r.overlaps(&clip).then_some(r)
        };
        set_default_camera();
        gl_use_default_material();
        unsafe {
            macroquad::window::get_internal_gl().quad_gl.scissor(None);
        }
        clear_background(BLACK);
        for background in [true, false] {
            self.app
                .combat_motion
                .draw_blasts(self.app.game.map(), now, false, background, rect);
            self.app
                .combat_motion
                .draw_fireballs(&self.app.game, now, false, background, rect);
            self.app.combat_motion.draw_agent_world(
                &self.app.game,
                now,
                false,
                background,
                clip,
                rect,
            );
        }
        self.app
            .combat_motion
            .draw_spells(&self.app.game, now, false, clip, rect);
        unsafe {
            macroquad::window::get_internal_gl().quad_gl.scissor(None);
        }
        let image = crate::ui_capture::framebuffer()?;
        let cells = self
            .preview
            .cells()
            .iter()
            .map(|c| c.position)
            .collect::<Vec<_>>();
        let mut painted = 0;
        for y in 0..usize::from(image.height) {
            for x in 0..usize::from(image.width) {
                let offset =
                    ((usize::from(image.height) - 1 - y) * usize::from(image.width) + x) * 4;
                if image.bytes[offset..offset + 3].iter().any(|c| *c > 3) {
                    painted += 1;
                    let at = GridPos::new(
                        16 + ((x as f32 - 608.) / 64.).floor() as i32,
                        16 + ((y as f32 - 468.) / 64.).floor() as i32,
                    );
                    if !cells.contains(&at) {
                        return Err(format!(
                            "{} impact outside preview at {at:?} ({x},{y})",
                            self.skill
                        ));
                    }
                }
            }
        }
        if painted < 10 {
            image.export_png(
                std::env::temp_dir()
                    .join("project-rl-fx-invisible.png")
                    .to_str()
                    .unwrap(),
            );
            return Err(format!("{} impact is invisible", self.skill));
        }
        Ok(painted)
    }
    fn picture(
        &mut self,
        age: f64,
        size: Vec2,
        focus: GridPos,
    ) -> Result<(Texture2D, Rect), String> {
        let _frame = crate::effect_geometry::frame_time(10. + age);
        self.app.combat_motion.review_age(10.);
        let preview_phase = self.corridor && age < 0.05;
        let original = if preview_phase {
            let baseline = WorldState::from_recovery_snapshot_bytes(
                &self.before_cast,
                self.app.game.rules().clone(),
            )?;
            Some((
                std::mem::replace(&mut self.app.game, baseline),
                std::mem::take(&mut self.app.combat_motion),
            ))
        } else {
            None
        };
        self.app.draw();
        let cells = self
            .preview
            .cells()
            .iter()
            .map(|c| c.position)
            .collect::<Vec<_>>();
        for at in cells.iter().filter(|_| !self.corridor || preview_phase) {
            if let Some(r) = self.rect(*at) {
                let edge = [(0, -1), (1, 0), (0, 1), (-1, 0)]
                    .map(|(x, y)| !cells.contains(&GridPos::new(at.x + x, at.y + y)));
                crate::attack_preview_visuals::area(r, true, edge);
            }
        }
        let center = self.rect(focus).ok_or("Focus outside camera")?.center();
        let crop = Rect::new(
            (center.x - size.x * 0.5).floor(),
            (center.y - size.y * 0.5).floor(),
            size.x,
            size.y,
        );
        if crop.x < 0.
            || crop.y < 0.
            || crop.right() > screen_width()
            || crop.bottom() > screen_height()
        {
            return Err(format!("Native crop unavailable {crop:?}"));
        }
        let image = crate::ui_capture::framebuffer()?;
        if let Some((game, motion)) = original {
            self.app.game = game;
            self.app.combat_motion = motion;
        }
        let texture = Texture2D::from_image(&image);
        texture.set_filter(FilterMode::Nearest);
        Ok((
            texture,
            Rect::new(crop.x, image.height as f32 - crop.bottom(), size.x, size.y),
        ))
    }
}

impl AsciiApp {
    pub async fn capture_effect_footprints(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let corridor_only =
            std::env::args().nth(1).as_deref() == Some("--ui-cold-souffle-corridor");
        let fire_only = corridor_only
            || std::env::args().nth(1).as_deref() == Some("--ui-cold-effect-footprints-fire");
        let capture_size = if fire_only {
            (1920., 1050.)
        } else {
            (1280., 1000.)
        };
        request_new_screen_size(capture_size.0, capture_size.1);
        for _ in 0..3 {
            next_frame().await;
        }
        if (screen_width(), screen_height()) != capture_size {
            return Err("Native 1280x1000 surface unavailable".into());
        }
        let mut report = Vec::new();
        let groups = [
            (
                "souffle-couloir-1",
                vec![("FE02-COULOIR", 1, (-1, 0)), ("FE02-COULOIR", 1, (0, -1))],
            ),
            (
                "souffle-couloir-4",
                vec![("FE02-COULOIR", 4, (1, 0)), ("FE02-COULOIR", 4, (0, 1))],
            ),
            (
                "braise",
                vec![
                    ("FE01", 1, (-2, 0)),
                    ("FE01", 1, (0, -2)),
                    ("FE01", 1, (2, 0)),
                    ("FE01", 1, (0, 2)),
                ],
            ),
            ("occulte", vec![("A02", 1, (0, 0)), ("A02", 10, (0, 0))]),
            ("boule-feu", vec![("FE05", 1, (0, 0)), ("FE05", 10, (0, 0))]),
            ("nova", vec![("FR02", 1, (0, 0)), ("FR02", 10, (0, 0))]),
            ("blizzard", vec![("FR05", 1, (0, 0)), ("FR05", 10, (0, 0))]),
            ("souffle", vec![("FE02", 1, (1, 0)), ("FE02", 4, (1, 0))]),
            ("vague", vec![("FR04", 1, (1, 0)), ("FR04", 10, (0, -1))]),
            ("aiguille", vec![("FR01", 1, (-2, 0)), ("FR01", 1, (0, -2))]),
            ("fleche", vec![("A01", 1, (-2, 0)), ("A01", 1, (0, -2))]),
        ];
        for (name, specs) in groups {
            if corridor_only && !name.starts_with("souffle-couloir") && name != "braise" {
                continue;
            }
            if fire_only
                && !(name.starts_with("souffle-couloir")
                    || name == "braise"
                    || matches!(
                        name,
                        "occulte" | "boule-feu" | "nova" | "blizzard" | "souffle" | "vague"
                    ))
            {
                continue;
            }
            eprintln!("[FX review] Sheet {name}");
            let mut scenes = specs
                .into_iter()
                .map(|(id, rank, d)| Scene::new(id, rank, d))
                .collect::<Result<Vec<_>, _>>()?;
            let before = scenes
                .iter()
                .map(|s| suspension::fingerprint(&s.app.game))
                .collect::<Vec<_>>();
            let size = if scenes.len() == 4 {
                vec2(320., 320.)
            } else if fire_only && !name.starts_with("souffle-couloir") {
                vec2(832., 832.)
            } else {
                vec2(576., 640.)
            };
            for frame in 0..16 {
                let age = 0.012 + frame as f64 * 0.035;
                let mut pictures = Vec::new();
                for scene in &mut scenes {
                    let focus = if scene.corridor {
                        let origin = scene.preview.origin();
                        let target = scene.preview.target();
                        let half = (i32::from(scene.rank) + 1) / 2;
                        GridPos::new(
                            origin.x + (target.x - origin.x).signum() * half,
                            origin.y + (target.y - origin.y).signum() * half,
                        )
                    } else if name == "souffle" {
                        GridPos::new(18, 16)
                    } else if matches!(name, "braise" | "aiguille" | "fleche") {
                        let origin = scene.preview.origin();
                        let target = scene.preview.target();
                        GridPos::new((origin.x + target.x) / 2, (origin.y + target.y) / 2)
                    } else {
                        scene.preview.origin()
                    };
                    pictures.push(scene.picture(age, size, focus)?);
                    next_frame().await;
                }
                for pass in 0..2 {
                    set_default_camera();
                    clear_background(Color::from_rgba(11, 17, 23, 255));
                    crate::ui_theme::draw_text_bold(
                        if name.starts_with("souffle-couloir") {
                            "SOUFFLE-COULOIR-01 · prévisualisation puis animation · 64 px"
                        } else {
                            "FX-FOOTPRINT-01 · rendu natif 64 px · contours de contrôle"
                        },
                        20.,
                        30.,
                        20.,
                        WHITE,
                    );
                    let gap = if scenes.len() == 4 { 0. } else { 32. };
                    for (i, (texture, source)) in pictures.iter().enumerate() {
                        let x =
                            if scenes.len() == 4 { 0. } else { 32. } + i as f32 * (size.x + gap);
                        crate::ui_theme::draw_text_bold(
                            &scenes[i].label,
                            x + 8.,
                            56.,
                            17.,
                            Color::from_rgba(220, 217, 197, 255),
                        );
                        draw_texture_ex(
                            texture,
                            x,
                            72.,
                            WHITE,
                            DrawTextureParams {
                                source: Some(*source),
                                flip_y: true,
                                ..Default::default()
                            },
                        );
                    }
                    if pass == 0 {
                        next_frame().await;
                    }
                }
                crate::ui_capture::framebuffer()?.export_png(
                    output
                        .join(format!("{name}-{frame:02}.png"))
                        .to_str()
                        .ok_or("Capture path")?,
                );
                next_frame().await;
            }
            for (i, scene) in scenes.iter().enumerate() {
                if suspension::fingerprint(&scene.app.game) != before[i] {
                    return Err("Rendering changed game state".into());
                }
                report.push(serde_json::json!({"sheet":name,"skill":scene.skill,"rank":scene.rank,
                    "cells":scene.preview.cells().iter().map(|c|[c.position.x,c.position.y]).collect::<Vec<_>>(),
                    "rendering_state_rng_unchanged":true,"geometry_prototype_only":false,
                    "cast_matches_reference_events_state_rng":true}));
            }
            if matches!(
                name,
                "souffle-couloir-1"
                    | "souffle-couloir-4"
                    | "braise"
                    | "occulte"
                    | "boule-feu"
                    | "nova"
                    | "aiguille"
                    | "fleche"
            ) {
                for scene in &mut scenes {
                    scene.verify_impact_pixels()?;
                    next_frame().await;
                }
            }
        }
        if !fire_only {
            Self::capture_marker_pulse(&output.join("markers")).await?;
        }
        std::fs::write(output.join(if corridor_only {"souffle-corridor-native-checks.json"} else if fire_only {"fire-native-checks.json"} else {"native-checks.json"}),serde_json::to_vec_pretty(&serde_json::json!({
            "review":"FX-FOOTPRINT-01","production_enabled":true,"tile_pixels":64,"core_opacity":0.82,
            "underlay_opacity_multiplier":0.45,"scenes":report})).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }
}
