//! Terminal dialogue presenter shared by normal launches and isolated captures.
//! Sound DIALOGUE-AUDIO-01-03 remains a provisional listening choice.
use super::*;
use crate::terminal_dialogue as bubble;
use macroquad::audio::{PlaySoundParams, Sound, load_sound_from_bytes, play_sound, stop_sound};

const AUDIO: &[u8] = include_bytes!("../assets/audio/ui/dialogue-writing-03.wav");

#[derive(Debug, PartialEq, Eq)]
enum AudioAction {
    None,
    Start,
    Stop,
}

#[derive(Default)]
struct Playback {
    opened_at: Option<f64>,
    started: bool,
    playing: bool,
}

impl Playback {
    fn sync_open(&mut self, open: bool, now: f64) {
        if open && self.opened_at.is_none() {
            self.opened_at = Some(now);
            self.started = false;
        } else if !open {
            self.opened_at = None;
        }
    }

    fn elapsed(&self, now: f64) -> f64 {
        self.opened_at.map_or(0., |start| (now - start).max(0.))
    }

    fn audio_step(&mut self, now: f64, characters: usize, reduced: bool) -> AudioAction {
        let writing = self.opened_at.is_some()
            && !reduced
            && self.elapsed(now) < characters as f64 / bubble::CHARACTERS_PER_SECOND;
        if !writing {
            self.started = true;
            if std::mem::take(&mut self.playing) {
                AudioAction::Stop
            } else {
                AudioAction::None
            }
        } else if !self.started {
            self.started = true;
            self.playing = true;
            AudioAction::Start
        } else {
            AudioAction::None
        }
    }
}

pub struct TerminalDialogue {
    sound: Option<Sound>,
    playback: Playback,
    consulted: bool,
    starts: usize,
    stops: usize,
    run_seed: Option<u64>,
}

impl Drop for TerminalDialogue {
    fn drop(&mut self) {
        self.stop();
    }
}

impl TerminalDialogue {
    pub async fn load() -> Result<Self, String> {
        let mut presenter = Self::silent();
        if cfg!(feature = "dialogue-audio") {
            presenter.sound = Some(
                load_sound_from_bytes(AUDIO)
                    .await
                    .map_err(|e| e.to_string())?,
            );
        }
        Ok(presenter)
    }

    pub fn silent() -> Self {
        Self {
            sound: None,
            playback: Playback::default(),
            consulted: false,
            starts: 0,
            stops: 0,
            run_seed: None,
        }
    }

    pub fn officer(app: &AsciiApp) -> Option<(EntityId, GridPos)> {
        if app.menu != MenuScreen::Hidden || app.character_creation.is_some() {
            return None;
        }
        if app.game.current_zone()?.id.as_str() != crate::cryogenic_camp::ZONE_ID {
            return None;
        }
        let entity = app.npc_interaction?;
        let actor = app.game.actors().get(entity)?;
        (crate::cryogenic_camp::actor_index(actor) == Some(1)).then_some((entity, actor.position()))
    }

    fn audio(&mut self, action: AudioAction) {
        match action {
            AudioAction::Start => {
                // Keep exactly the selected sample and its reviewed level.
                if let Some(sound) = &self.sound {
                    play_sound(
                        sound,
                        PlaySoundParams {
                            looped: false,
                            volume: 1.,
                        },
                    );
                }
                self.starts += 1;
            }
            AudioAction::Stop => {
                if let Some(sound) = &self.sound {
                    stop_sound(sound);
                }
                self.stops += 1;
            }
            AudioAction::None => {}
        }
    }

    pub fn stop(&mut self) {
        if let Some(sound) = &self.sound {
            stop_sound(sound);
        }
        self.playback.playing = false;
    }

    pub fn dismiss(&mut self, app: &mut AsciiApp, input: &mut InputFrame, now: f64) -> bool {
        if Self::officer(app).is_none() || !bubble::take_dismiss(input) {
            return false;
        }
        app.npc_interaction = None;
        app.npc_interaction_message.clear();
        app.mouse_walk = None;
        app.movement_repeat.clear();
        self.playback.sync_open(false, now);
        let action = self.playback.audio_step(now, 0, false);
        self.audio(action);
        true
    }

    pub fn draw(
        &mut self,
        app: &mut AsciiApp,
        now: f64,
    ) -> Result<Option<bubble::TerminalFrame>, String> {
        if self.run_seed != Some(app.seed) || app.character_creation.is_some() {
            self.stop();
            self.playback = Playback::default();
            self.consulted = false;
            self.run_seed = Some(app.seed);
        }
        let officer = Self::officer(app);
        self.playback.sync_open(officer.is_some(), now);
        self.consulted |= officer.is_some();
        // The review guard is scoped to this camp/frame, so another map's NPC
        // at the same coordinates never inherits the officer's read state.
        let _review = (self.consulted
            && app
                .game
                .current_zone()
                .is_some_and(|z| z.id.as_str() == crate::cryogenic_camp::ZONE_ID))
        .then(|| {
            let guard = bubble::begin(GridPos::new(6, 7));
            bubble::set_open(officer.is_some());
            guard
        });
        if let Some((provider, at)) = officer {
            app.mouse_walk = None;
            app.movement_repeat.clear();
            // Paint the real world and HUD, then the revised bubble instead of
            // the old modal. Services and other NPCs keep their complete UI.
            app.npc_interaction = None;
            app.draw();
            app.npc_interaction = Some(provider);
            let navigation = app.navigation_signal_summary().is_some();
            let viewport = crate::terminal_view::terminal_playfield(
                &app.game,
                app.terminal_bounds(),
                app.ui_scale(),
                navigation,
            );
            let speaker = app
                .terminal
                .world_cell_rect(
                    &app.game,
                    app.terminal_bounds(),
                    app.ui_scale(),
                    app.graphics.active.world_cell_px,
                    navigation,
                    at,
                    Vec2::ZERO,
                )
                .ok_or("Officier hors caméra")?;
            // The terminal playfield and actor rectangles already use framebuffer
            // pixels. Applying the logical HUD camera would scale them twice.
            set_default_camera();
            let frame = bubble::terminal_at(
                viewport,
                speaker,
                "Officier",
                crate::cryogenic_camp::OFFICER_DIALOGUE,
                Some(self.playback.elapsed(now)),
                app.graphics.active.reduced_motion,
            );
            set_default_camera();
            let frame = frame?;
            let action =
                self.playback
                    .audio_step(now, frame.characters, app.graphics.active.reduced_motion);
            self.audio(action);
            Ok(Some(frame))
        } else {
            let action = self.playback.audio_step(now, 0, false);
            self.audio(action);
            app.draw();
            Ok(None)
        }
    }
}

impl AsciiApp {
    #[cfg(debug_assertions)]
    fn dialogue_capture_app(path: std::path::PathBuf) -> Result<Self, String> {
        let mut app = Self::new_with_session_path(path)?;
        app.crash_recovery_enabled = false;
        app.begin_character_creation_with_seed(false, 1810)?;
        app.update_input(&InputFrame {
            pressed: [controls::Binding::key("Enter")].into(),
            ..Default::default()
        });
        app.walk_fixture_to_unchecked(GridPos::new(5, 7))?;
        Ok(app)
    }

    #[cfg(debug_assertions)]
    pub async fn capture_terminal_dialogue(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let mut app = Self::dialogue_capture_app(output.join("isolated-run.json"))?;
        let mut trial = TerminalDialogue::load().await?;
        request_new_screen_size(1600., 1000.);
        for _ in 0..3 {
            next_frame().await;
        }
        let before = suspension::fingerprint(&app.game);
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Interact).clone()].into(),
            pointer: Some((1., 1.)),
            viewport: Some((app.ui_width(), app.ui_height())),
            ui_scale: Some(app.ui_scale()),
            ..Default::default()
        });
        if TerminalDialogue::officer(&app).is_none() {
            return Err("Le raccourci sans survol n'ouvre pas le dialogue".into());
        }
        app.npc_interaction = None;
        // Exercise the same pointer dispatch as a normal game, including when
        // the notification is gone and the NPC is already selected.
        let click_officer = |app: &mut Self| -> Result<(), String> {
            let scale = app.ui_scale();
            let cell = app
                .terminal
                .world_cell_rect(
                    &app.game,
                    app.terminal_bounds(),
                    scale,
                    app.graphics.active.world_cell_px,
                    app.navigation_signal_summary().is_some(),
                    GridPos::new(6, 7),
                    app.camera_glide_offset(),
                )
                .ok_or("Officier hors caméra")?;
            app.update_input(&InputFrame {
                pressed: [controls::Binding::MouseLeft].into(),
                pointer: Some((
                    (cell.x + cell.w * 0.5) / scale,
                    (cell.y + cell.h * 0.5) / scale,
                )),
                viewport: Some((app.ui_width(), app.ui_height())),
                ui_scale: Some(scale),
                ..Default::default()
            });
            if TerminalDialogue::officer(app).is_none() {
                return Err("Le clic sur l'officier ne rouvre pas son dialogue".into());
            }
            Ok(())
        };
        click_officer(&mut app)?;
        let start = get_time();
        let mut mid = false;
        loop {
            let now = get_time();
            let frame = trial.draw(&mut app, now)?.ok_or("Bulle absente")?;
            if frame.cursor.right() > frame.bounds.right()
                || frame.cursor.bottom() > frame.bounds.bottom()
            {
                return Err("La frappe déborde de la bulle".into());
            }
            if !mid && now - start >= 1.2 {
                crate::ui_capture::framebuffer()?
                    .export_png(output.join("typing.png").to_str().unwrap());
                mid = true;
            }
            if now - start > 8.9 {
                crate::ui_capture::framebuffer()?
                    .export_png(output.join("complete.png").to_str().unwrap());
                break;
            }
            next_frame().await;
        }
        if trial.starts != 1 || trial.stops != 1 {
            return Err("Lecture sonore répétée ou poursuivie après le texte".into());
        }
        let mut closures = Vec::new();
        for (name, pointer, pause) in [
            ("inside", Some((650., 220.)), false),
            ("outside", Some((20., 900.)), false),
            ("escape", None, true),
        ] {
            app.npc_interaction = None;
            trial.draw(&mut app, get_time())?;
            click_officer(&mut app)?;
            trial.draw(&mut app, get_time())?;
            let stops = trial.stops;
            let mut input = InputFrame {
                pointer,
                pause,
                pressed: if pause {
                    [
                        controls::Binding::key("Escape"),
                        controls::Binding::key("F"),
                    ]
                    .into()
                } else {
                    [controls::Binding::MouseLeft, controls::Binding::key("F")].into()
                },
                held: [controls::Binding::key("W")].into(),
                ..Default::default()
            };
            if !trial.dismiss(&mut app, &mut input, get_time()) || trial.stops != stops + 1 {
                return Err("La fermeture ne coupe pas le son".into());
            }
            // The consumed frame is safe even if the outer loop receives it.
            app.update_input(&input);
            if app.npc_interaction.is_some() || suspension::fingerprint(&app.game) != before {
                return Err("La fermeture a modifié la partie".into());
            }
            closures.push(name);
        }
        app.graphics.active.reduced_motion = true;
        click_officer(&mut app)?;
        let starts = trial.starts;
        trial.draw(&mut app, get_time())?;
        if trial.starts != starts {
            return Err("Son actif en lecture sans animation".into());
        }
        app.npc_interaction = None;
        trial.draw(&mut app, get_time())?;
        crate::ui_capture::framebuffer()?.export_png(output.join("closed.png").to_str().unwrap());
        std::fs::write(output.join("checks.json"), serde_json::to_vec_pretty(&serde_json::json!({
            "review":"DIALOGUE-AUDIO-01-03", "selection":"normal_launch_shared_presenter",
            "normal_runs_changed":true, "audio_feature":cfg!(feature="dialogue-audio"),
            "embedded_asset":"assets/audio/ui/dialogue-writing-03.wav", "audio_loaded":true,
            "starts":trial.starts,"stops":trial.stops,"ends_with_writing":true,
            "closures_stop_audio":closures, "reduced_motion_silent":true,
            "npc_click_opens_and_reopens_dialogue":true,
            "keyboard_interacts_without_hover":true,
            "read_notification_remains_consulted":trial.consulted,
            "game_fingerprint_preserved":suspension::fingerprint(&app.game)==before,
            "save_path":app.suspension_path,
            "audibility":"Native playback requested; final listening judgement belongs to player"
        })).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(debug_assertions)]
    fn read_officer_dialogue_reopens_on_npc_click_after_each_dismissal_without_a_turn() {
        let path = std::env::temp_dir().join(format!(
            "project-rl-dialogue-reopen-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let mut app = AsciiApp::dialogue_capture_app(path).unwrap();
        let mut presenter = TerminalDialogue::silent();
        let at = GridPos::new(6, 7);
        let before = suspension::fingerprint(&app.game);
        // A read notification is hidden, while the actor remains interactive.
        let _read = bubble::begin(at);
        bubble::set_open(false);
        assert!(bubble::notification_hidden(at, true));
        for (index, pause) in [false, true, false].into_iter().enumerate() {
            let scale = 1.5;
            let viewport = (1600. / scale, 1000. / scale);
            let cell = app
                .terminal
                .world_cell_rect(
                    &app.game,
                    app.terminal_bounds_for(1600., 1000., scale),
                    scale,
                    app.graphics.active.world_cell_px,
                    app.navigation_signal_summary().is_some(),
                    at,
                    app.camera_glide_offset(),
                )
                .unwrap();
            app.update_input(&InputFrame {
                pressed: [controls::Binding::MouseLeft].into(),
                pointer: Some((
                    (cell.x + cell.w * 0.5) / scale,
                    (cell.y + cell.h * 0.5) / scale,
                )),
                viewport: Some(viewport),
                ui_scale: Some(scale),
                ..Default::default()
            });
            assert_eq!(TerminalDialogue::officer(&app).unwrap().1, at);
            assert!(app.attack_aim.is_none());
            assert!(app.mouse_walk.is_none());
            let mut close = InputFrame {
                pause,
                pressed: if pause {
                    [controls::Binding::key("Escape")].into()
                } else {
                    [controls::Binding::MouseLeft].into()
                },
                ..Default::default()
            };
            assert!(presenter.dismiss(&mut app, &mut close, index as f64));
            app.update_input(&close);
            assert!(TerminalDialogue::officer(&app).is_none());
            assert!(bubble::notification_hidden(at, true));
            assert_eq!(suspension::fingerprint(&app.game), before);
        }
    }

    #[test]
    fn audio_starts_once_stops_with_text_and_restarts_on_reopening() {
        let mut p = Playback::default();
        p.sync_open(true, 10.);
        assert_eq!(p.audio_step(10., 40, false), AudioAction::Start);
        assert_eq!(p.audio_step(10.5, 40, false), AudioAction::None);
        assert_eq!(p.audio_step(11., 40, false), AudioAction::Stop);
        assert_eq!(p.audio_step(12., 40, false), AudioAction::None);
        p.sync_open(false, 12.);
        p.sync_open(true, 13.);
        assert_eq!(p.audio_step(13., 40, false), AudioAction::Start);
        p.sync_open(false, 13.1);
        assert_eq!(p.audio_step(13.1, 40, false), AudioAction::Stop);
    }

    #[test]
    fn static_reading_and_empty_text_are_silent() {
        for (characters, reduced) in [(345, true), (0, false)] {
            let mut p = Playback::default();
            p.sync_open(true, 0.);
            assert_eq!(p.audio_step(0., characters, reduced), AudioAction::None);
        }
    }
}
