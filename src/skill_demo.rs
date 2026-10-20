//! SKILL-DEMO-01. A compact, isolated base-rank example for every active skill.
use super::*;
use std::cell::{Cell, RefCell};

#[derive(Default)]
pub(super) struct State {
    scene: RefCell<Option<Scene>>,
    error: RefCell<Option<String>>,
    surface: RefCell<Option<RenderTarget>>,
    clock: Cell<Option<f64>>,
}
struct Scene {
    app: Box<AsciiApp>,
    template: GameRules,
    baseline: Vec<u8>,
    skill: String,
    started: f64,
    step: usize,
    actions: Vec<GameCommand>,
    camera: (i32, i32, i32, i32),
}
impl Scene {
    fn new(
        host: &AsciiApp,
        skill: &str,
        now: f64,
        reuse: Option<(Box<AsciiApp>, GameRules)>,
    ) -> Result<Self, String> {
        let (mut app, template) = match reuse {
            Some(pair) => pair,
            None => {
                // Reuse the loaded, unmodified rules/catalogues rather than
                // reading and parsing all content/mod files on the first click.
                // Apply the canonical class again: learned ranks and live
                // equipment come from neither the host game nor its class.
                let mut template = host.rules.clone();
                let class = host
                    .character_classes
                    .get(&"core:arcaniste".parse().unwrap())
                    .ok_or("Missing demo class")?;
                class
                    .apply_to_rules(&mut template, class.recommended_attributes())
                    .map_err(|e| e.to_string())?;
                let app = Box::new(AsciiApp::from_seed(
                    724,
                    template.clone(),
                    host.texts.clone(),
                    host.loot.clone(),
                    host.expeditions.clone(),
                )?);
                (app, template)
            }
        };
        let fixture = project_rl::game::AgentSkillDemo::new(template.clone(), skill)?;
        app.character_class = host.character_class.clone();
        app.active_weapon_slot = fixture.weapon_slot;
        app.terminal = TerminalView::new(
            SectorDecor::default(),
            fixture.game.map(),
            fixture.game.player_visibility(),
        );
        app.game = WorldState::single(fixture.game);
        app.crash_recovery_enabled = false;
        app.suspension_path = std::env::temp_dir().join("project-rl-skill-demo-unused.json");
        app.history.clear();
        app.actor_glyphs.clear();
        app.combat_motion.clear();
        app.visual_cues.clear_world();
        let baseline = app.game.recovery_snapshot_bytes()?;
        Ok(Self {
            app,
            template,
            baseline,
            skill: skill.into(),
            started: now,
            step: 0,
            actions: fixture.actions,
            camera: fixture.bounds,
        })
    }
    fn tick(&mut self, now: f64) -> Result<(), String> {
        if now < self.started || now - self.started >= 3.4 {
            self.app.game = WorldState::from_recovery_snapshot_bytes(
                &self.baseline,
                self.app.game.rules().clone(),
            )?;
            self.app.history.clear();
            self.app.combat_motion.clear();
            self.app.visual_cues.clear_world();
            self.started = now;
            self.step = 0;
        }
        for (step, at) in [0.30, 1.10, 1.95].into_iter().enumerate() {
            if self.step == step
                && now - self.started >= at
                && let Some(command) = self.actions.get(step)
            {
                let outcome = self.app.execute_command(command.clone());
                if matches!(outcome, CommandOutcome::Rejected(_)) {
                    return Err(format!("{} example: {outcome:?}", self.skill));
                }
                let mut origins = BTreeMap::new();
                for event in self.app.game.drain_events() {
                    self.app
                        .capture_combat_visual(&event, &mut origins, self.started + at);
                }
                self.step += 1;
            }
        }
        self.app.combat_motion.prune(now);
        Ok(())
    }
    fn draw(&self, stage: Rect, now: f64, reduced: bool) {
        let (min_x, min_y, max_x, max_y) = self.camera;
        let width = (max_x - min_x + 1) as f32;
        let height = (max_y - min_y + 1) as f32;
        let cell = 64_f32
            .min(stage.w / width)
            .min(stage.h / height)
            .floor()
            .max(1.);
        let left = (stage.center().x - width * cell * 0.5).floor();
        let top = (stage.center().y - height * cell * 0.5).floor();
        let rect = |at: GridPos| {
            let r = Rect::new(
                left + (at.x - min_x) as f32 * cell,
                top + (at.y - min_y) as f32 * cell,
                cell,
                cell,
            );
            r.overlaps(&stage).then_some(r)
        };
        #[cfg(debug_assertions)]
        let _time = crate::effect_geometry::frame_time(now);
        for y in min_y + ((stage.y - top) / cell).floor() as i32
            ..min_y + ((stage.bottom() - top) / cell).ceil() as i32
        {
            for x in min_x + ((stage.x - left) / cell).floor() as i32
                ..min_x + ((stage.right() - left) / cell).ceil() as i32
            {
                let at = GridPos::new(x, y);
                if let Some(r) = rect(at) {
                    crate::terminal_view::draw_scenery_preview(
                        r,
                        crate::test_sector::Decor::Deck,
                        at,
                    );
                }
            }
        }
        let motion = &self.app.combat_motion;
        let world = &self.app.game;
        motion.draw_blasts(world.map(), now, reduced, true, rect);
        motion.draw_fireballs(world, now, reduced, true, rect);
        motion.draw_agent_world(world, now, reduced, true, stage, rect);
        for (id, actor) in world.actors().iter() {
            let Some(mut r) = rect(actor.position()) else {
                continue;
            };
            let Some(visual) = self.app.front_actor_visual(id) else {
                continue;
            };
            let offset = motion.offset(id, now, reduced) * cell;
            r.x += offset.x;
            r.y += offset.y;
            let burning = crate::terminal_view::burning_actor(world, actor.position());
            if let Some(id) = burning {
                motion.draw_burn(id, r, now, reduced, true);
            }
            let frozen = actor
                .statuses()
                .any(|s| matches!(s.definition.as_str(), "agent:frozen" | "agent:mine_frozen"));
            if frozen {
                motion.draw_gel(id, r, now, reduced, true);
            }
            let opacity = if id == world.player_id() && world.agent_stealth_active() {
                0.35
            } else {
                1.
            };
            crate::game_art::draw_world_animated(r, visual, opacity, (!reduced).then_some(now));
            if frozen {
                motion.draw_gel(id, r, now, reduced, false);
            }
            if let Some(id) = burning {
                motion.draw_burn(id, r, now, reduced, false);
            }
            if actor.integrity() < actor.maximum_integrity() && !world.agent_is_hologram(id) {
                let allied =
                    id == world.player_id() || actor.player_relation() == PlayerRelation::Allied;
                crate::terminal_view::draw_hud_progress(
                    Rect::new(r.x + cell * 0.15, r.bottom() - 2., cell * 0.7, 3.),
                    actor.integrity(),
                    actor.maximum_integrity(),
                    if allied {
                        Color::from_rgba(88, 190, 127, 255)
                    } else {
                        Color::from_rgba(215, 69, 66, 255)
                    },
                );
            }
        }
        motion.draw_blasts(world.map(), now, reduced, false, rect);
        motion.draw_fireballs(world, now, reduced, false, rect);
        motion.draw_agent_world(world, now, reduced, false, stage, rect);
        motion.draw_taunts(world.map(), now, reduced, rect);
        motion.draw_spells(world, now, reduced, stage, rect);
    }
}
impl State {
    fn prepare(&self, host: &AsciiApp, id: &str, now: f64) -> Result<(), String> {
        if self.scene.borrow().as_ref().is_some_and(|s| s.skill == id) {
            return Ok(());
        }
        let reuse = self.scene.borrow_mut().take().map(|s| (s.app, s.template));
        *self.scene.borrow_mut() = Some(Scene::new(host, id, now, reuse)?);
        *self.error.borrow_mut() = None;
        Ok(())
    }
    pub(super) fn draw(&self, host: &AsciiApp, bounds: Rect) {
        let Some(id) = host.ux.skill_demo_request.as_deref() else {
            return;
        };
        let title = project_rl::skills::agent::definition(id).map_or(id, |s| s.title.as_str());
        draw_text_bold_centered(
            title,
            Rect::new(bounds.x + 8., bounds.y + 3., bounds.w - 16., 24.),
            14,
            UiTheme.muted(),
        );
        let now = self.clock.get().unwrap_or_else(get_time);
        if let Err(error) = self
            .prepare(host, id, now)
            .and_then(|()| self.scene.borrow_mut().as_mut().unwrap().tick(now))
        {
            if self.error.borrow().as_ref() != Some(&error) {
                eprintln!("[SKILL-DEMO-01] {error}");
            }
            *self.error.borrow_mut() = Some(error);
            return;
        }
        let width = (bounds.w - 4.).floor().max(1.) as u32;
        let height = (bounds.h - 30.).floor().max(1.) as u32;
        let mut surface = self.surface.borrow_mut();
        if surface.as_ref().is_none_or(|s| {
            s.texture.width() != width as f32 || s.texture.height() != height as f32
        }) {
            let target = render_target(width, height);
            target.texture.set_filter(FilterMode::Nearest);
            *surface = Some(target);
        }
        let target = surface.as_ref().unwrap();
        macroquad::camera::push_camera_state();
        let mut camera =
            Camera2D::from_display_rect(Rect::new(0., 0., width as f32, height as f32));
        camera.render_target = Some(target.clone());
        set_camera(&camera);
        unsafe {
            macroquad::window::get_internal_gl().quad_gl.scissor(None);
        }
        clear_background(Color::from_rgba(11, 15, 20, 255));
        self.scene.borrow().as_ref().unwrap().draw(
            Rect::new(0., 0., width as f32, height as f32),
            now,
            host.graphics.active.reduced_motion,
        );
        gl_use_default_material();
        unsafe {
            macroquad::window::get_internal_gl().quad_gl.scissor(None);
        }
        macroquad::camera::pop_camera_state();
        draw_texture_ex(
            &target.texture,
            bounds.x + 2.,
            bounds.y + 28.,
            WHITE,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()
            },
        );
        draw_rectangle_lines(
            bounds.x + 1.,
            bounds.y + 27.,
            width as f32 + 2.,
            height as f32 + 2.,
            1.,
            Color {
                a: 0.65,
                ..UiTheme.accent()
            },
        );
    }
    #[cfg(debug_assertions)]
    pub(super) fn capture_time(&self, time: Option<f64>) {
        self.clock.set(time);
    }
    #[cfg(debug_assertions)]
    pub(super) fn restart(&self) {
        if let Some(scene) = self.scene.borrow_mut().as_mut() {
            scene.skill.clear();
        }
    }
    #[cfg(debug_assertions)]
    pub(super) fn failure(&self) -> Option<String> {
        self.error.borrow().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_active_replays_in_the_normal_window_without_touching_the_host() {
        let mut host = AsciiApp::new_agent_skill_laboratory().unwrap();
        host.game.drain_events();
        let before = host.game.recovery_snapshot_bytes().unwrap();
        let bar = host.ux.skill_bar.clone();
        let history = host.history.len();
        let state = State::default();
        let mut previous_app = None;
        for skill in project_rl::skills::agent::definitions()
            .iter()
            .filter(|s| s.kind == "active")
        {
            state
                .prepare(&host, &skill.id, 10.)
                .unwrap_or_else(|e| panic!("{}: {e}", skill.id));
            let mut scene = state.scene.borrow_mut();
            let scene = scene.as_mut().unwrap();
            let pointer = (&*scene.app) as *const AsciiApp;
            if let Some(previous) = previous_app {
                assert_eq!(pointer, previous, "one reusable isolated simulation");
            }
            previous_app = Some(pointer);
            assert_eq!(scene.app.game.player_agent_skill_rank(&skill.id), 1);
            for time in [10.5, 11.3, 12.3, 14.] {
                scene
                    .tick(time)
                    .unwrap_or_else(|e| panic!("{}: {e}", skill.id));
                if time == 10.5 {
                    let status = match skill.id.as_str() {
                        "AF02" => Some("agent:armor_breach"),
                        "AF03" => Some("agent:silence"),
                        "AF04" => Some("agent:regeneration_break"),
                        "AF05" => Some("agent:disorientation"),
                        "O02" => Some("agent:interference"),
                        _ => None,
                    };
                    if let Some(status) = status {
                        assert!(
                            scene.app.game.actors().iter().any(|(_, a)| a
                                .statuses()
                                .any(|s| s.definition.as_str() == status)),
                            "{} successful illustrative affliction",
                            skill.id
                        );
                    }
                    if skill.id == "HY06" {
                        assert!(
                            scene
                                .app
                                .game
                                .actors()
                                .iter()
                                .any(|(_, a)| a.player_relation() == PlayerRelation::Allied)
                        );
                    }
                }
            }
            assert_eq!(scene.step, 0);
            assert!(!scene.app.crash_recovery_enabled);
        }
        assert_eq!(host.game.recovery_snapshot_bytes().unwrap(), before);
        assert_eq!(host.ux.skill_bar, bar);
        assert_eq!(host.history.len(), history);
        state.prepare(&host, "FE02", 20.).unwrap();
        let baseline = state.scene.borrow().as_ref().unwrap().baseline.clone();
        for _ in 0..4 {
            assert_eq!(
                host.execute_command(GameCommand::LearnAgentSkill {
                    skill: "FE02".into()
                }),
                CommandOutcome::AppliedWithoutTime
            );
        }
        state.scene.borrow_mut().as_mut().unwrap().skill.clear();
        state.prepare(&host, "FE02", 20.).unwrap();
        assert_eq!(state.scene.borrow().as_ref().unwrap().baseline, baseline);
    }
}
