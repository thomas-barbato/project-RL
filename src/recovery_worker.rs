//! Immutable recovery capture and a single bounded background writer.
use super::*;
use std::cell::RefCell;
use std::path::PathBuf;
use std::thread::JoinHandle;

pub(super) struct SaveContent<'a> {
    pub rules: &'a GameRules,
    pub loot: &'a LootCatalog,
    pub expeditions: &'a ExpeditionCatalog,
    pub regional_worlds: &'a RegionalWorldCatalog,
}

pub(super) fn encode_snapshot(
    game: &WorldState,
    presentation: RecoveryPresentationRef<'_>,
) -> Result<String, String> {
    let engine = game.recovery_snapshot_bytes()?;
    let snapshot = AppRecoverySnapshotRef {
        engine: &engine,
        presentation_fingerprint: suspension::fingerprint(&presentation),
        presentation,
    };
    let mut bytes = Vec::new();
    bincode::serialize_into(&mut bytes, &snapshot)
        .map_err(|error| format!("Impossible d'encoder la reprise : {error}"))?;
    if bytes.len() > MAX_APP_RECOVERY_SNAPSHOT_BYTES {
        return Err("Instantané de reprise trop volumineux.".to_owned());
    }
    Ok(base64::engine::general_purpose::STANDARD.encode(bytes))
}

pub(super) fn finish_suspension(
    mut saved: Suspension,
    game: &WorldState,
    presentation: RecoveryPresentationRef<'_>,
    content: SaveContent<'_>,
) -> Result<Suspension, String> {
    let version = saved.version;
    saved.rules = rules_fingerprint_for_version(content.rules, version);
    saved.loot_rules = (version >= 3).then(|| suspension::fingerprint(content.loot));
    saved.world_rules = (version >= 4).then(|| {
        world_fingerprint_for_version(content.expeditions, content.regional_worlds, version)
    });
    saved.recovery = Some(RecoveryPayload::snapshot(
        saved.commands.len(),
        encode_snapshot(game, presentation)?,
    )?);
    saved.state = if version == 1 {
        suspension::fingerprint(game.active_game())
    } else {
        suspension::fingerprint(game)
    };
    saved.validate()?;
    Ok(saved)
}

pub(super) struct CheckpointCapture {
    game: WorldState,
    presentation: RecoveryPresentation,
    rules: GameRules,
    loot: LootCatalog,
    expeditions: ExpeditionCatalog,
    regional_worlds: RegionalWorldCatalog,
    metadata: Suspension,
}

impl CheckpointCapture {
    pub(super) fn from_app(app: &AsciiApp) -> Result<Self, String> {
        let metadata = app.suspension_metadata()?;
        Ok(Self {
            game: app.game.clone(),
            presentation: RecoveryPresentation {
                terminal: app.terminal.clone(),
                zone_views: app.zone_views.clone(),
                zone_decor: app.zone_decor.clone(),
                facing: app.facing,
                regional_zones: app.regional_zones.clone(),
                actor_glyphs: app.actor_glyphs.clone(),
                intro_city_reached: app.intro_city_reached,
            },
            rules: app.rules.clone(),
            loot: app.loot.clone(),
            expeditions: app.expeditions.clone(),
            regional_worlds: app.regional_worlds.clone(),
            metadata,
        })
    }

    pub(super) fn encode(self) -> Result<Suspension, String> {
        let p = &self.presentation;
        finish_suspension(
            self.metadata,
            &self.game,
            RecoveryPresentationRef {
                terminal: &p.terminal,
                zone_views: &p.zone_views,
                zone_decor: &p.zone_decor,
                facing: p.facing,
                regional_zones: &p.regional_zones,
                actor_glyphs: &p.actor_glyphs,
                intro_city_reached: p.intro_city_reached,
            },
            SaveContent {
                rules: &self.rules,
                loot: &self.loot,
                expeditions: &self.expeditions,
                regional_worlds: &self.regional_worlds,
            },
        )
    }
}

pub(super) struct Completion {
    pub sequence: u64,
    pub command_count: usize,
}

#[derive(Default)]
pub(super) struct RecoveryWorker {
    pending: RefCell<Option<JoinHandle<Result<Completion, String>>>>,
    completed: RefCell<Option<Result<Completion, String>>>,
}

impl RecoveryWorker {
    pub(super) fn has_work(&self) -> bool {
        self.pending.borrow().is_some() || self.completed.borrow().is_some()
    }

    pub(super) fn start(
        &self,
        capture: CheckpointCapture,
        sequence: u64,
        path: PathBuf,
    ) -> Result<(), String> {
        if self.has_work() {
            return Err("Une récupération est déjà en cours.".into());
        }
        let worker = std::thread::Builder::new()
            .name("rl-recovery".into())
            .spawn(move || {
                let saved = capture.encode()?;
                let command_count = saved.commands.len();
                CrashRecovery::new(sequence, saved)?.write_replacing(&path)?;
                Ok(Completion {
                    sequence,
                    command_count,
                })
            })
            .map_err(|error| format!("Impossible de préparer la récupération : {error}"))?;
        *self.pending.borrow_mut() = Some(worker);
        Ok(())
    }

    fn finish(&self, wait: bool) {
        let ready = self
            .pending
            .borrow()
            .as_ref()
            .is_some_and(|worker| wait || worker.is_finished());
        if ready {
            let worker = self.pending.borrow_mut().take().unwrap();
            let result = worker
                .join()
                .unwrap_or_else(|_| Err("Le travailleur de récupération a été interrompu.".into()));
            *self.completed.borrow_mut() = Some(result);
        }
    }

    pub(super) fn take_completion(&self, wait: bool) -> Option<Result<Completion, String>> {
        self.finish(wait);
        self.completed.borrow_mut().take()
    }

    // Keep the completion until the app can update its counters, even if a
    // voluntary suspension fails and the player continues the current run.
    pub(super) fn barrier(&self) {
        self.finish(true);
    }
}

impl Drop for RecoveryWorker {
    fn drop(&mut self) {
        self.finish(true);
        if let Some(Err(error)) = self.completed.get_mut().take() {
            eprintln!("[RECOVERY] Final checkpoint failed: {error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    fn app(prefix: &str) -> AsciiApp {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        let folder = std::env::temp_dir().join(format!(
            "rl-recovery-{prefix}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir(&folder).unwrap();
        app.suspension_path = folder.join("suspended-run.json");
        app.session_lock = Some(suspension::session_lock(&folder.join("session.lock")).unwrap());
        app.crash_recovery_enabled = true;
        app.start_crash_recovery();
        assert_eq!(app.crash_recovery_sequence, 1);
        app
    }

    fn clean(mut app: AsciiApp) {
        app.remove_crash_recovery_files_except(None).unwrap();
        let folder = app.suspension_path.parent().unwrap().to_owned();
        if app.suspension_path.exists() {
            std::fs::remove_file(&app.suspension_path).unwrap();
        }
        drop(app.session_lock.take());
        drop(app);
        std::fs::remove_file(folder.join("session.lock")).unwrap();
        std::fs::remove_dir(folder).unwrap();
    }

    fn step(app: &mut AsciiApp) {
        assert_eq!(
            app.execute_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        app.capture_events_at(Some(0.));
    }

    // A channel holds the writer before encoding, making lifecycle assertions
    // independent of CPU and disk speed, without sleeps or timing races.
    fn gated_writer(app: &AsciiApp) -> mpsc::Sender<()> {
        let (send, receive) = mpsc::channel();
        let capture = CheckpointCapture::from_app(app).unwrap();
        let sequence = app.crash_recovery_sequence + 1;
        let path = app.crash_recovery_paths()[sequence as usize % 2].clone();
        *app.crash_recovery_worker.pending.borrow_mut() = Some(std::thread::spawn(move || {
            receive.recv().unwrap();
            let saved = capture.encode()?;
            let command_count = saved.commands.len();
            CrashRecovery::new(sequence, saved)?.write_replacing(&path)?;
            Ok(Completion {
                sequence,
                command_count,
            })
        }));
        send
    }

    #[test]
    fn recovery_capture_preserves_exact_save_bytes_after_live_changes() {
        let mut app = app("immutable");
        step(&mut app);
        let before = serde_json::to_vec(&app.suspension().unwrap()).unwrap();
        let capture = CheckpointCapture::from_app(&app).unwrap();
        step(&mut app);
        app.facing = Direction::West;
        app.log.push("Nouvel événement après la capture".into());
        let saved = std::thread::spawn(move || capture.encode().unwrap())
            .join()
            .unwrap();
        assert_eq!(serde_json::to_vec(&saved).unwrap(), before);
        assert_ne!(saved.state, suspension::fingerprint(&app.game));
        let restored = AsciiApp::restore_suspension(
            &saved,
            app.rules.clone(),
            app.texts.clone(),
            app.loot.clone(),
            app.expeditions.clone(),
        )
        .unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        clean(app);
    }

    #[test]
    fn recovery_queue_is_bounded_and_catches_up_to_the_latest_commands() {
        let mut app = app("bounded");
        let release = gated_writer(&app);
        for _ in 0..12 {
            step(&mut app);
            app.update_crash_recovery();
        }
        assert_eq!(app.crash_recovery_sequence, 1);
        assert!(app.crash_recovery_worker.pending.borrow().is_some());
        assert!(
            app.crash_recovery_worker
                .start(
                    CheckpointCapture::from_app(&app).unwrap(),
                    3,
                    app.crash_recovery_paths()[1].clone()
                )
                .is_err()
        );
        release.send(()).unwrap();
        app.poll_crash_recovery_worker(true);
        assert_eq!(app.crash_recovery_sequence, 2);
        app.update_crash_recovery();
        app.poll_crash_recovery_worker(true);
        let (saved, source) = AsciiApp::read_resume_candidate_from(&app.suspension_path).unwrap();
        assert_eq!(source, ResumeSource::CrashRecovery { sequence: 3 });
        assert_eq!(saved.commands.len(), 12);
        assert_eq!(saved.state, suspension::fingerprint(&app.game));
        clean(app);
    }

    #[test]
    fn recovery_failure_keeps_the_other_slot_and_does_not_retry_while_idle() {
        let mut app = app("failure");
        let blocked = app.crash_recovery_paths()[0].clone();
        std::fs::create_dir(&blocked).unwrap();
        for _ in 0..5 {
            step(&mut app);
            app.update_crash_recovery();
        }
        app.poll_crash_recovery_worker(true);
        assert_eq!(app.crash_recovery_sequence, 1);
        assert_eq!(
            AsciiApp::read_resume_candidate_from(&app.suspension_path)
                .unwrap()
                .1,
            ResumeSource::CrashRecovery { sequence: 1 }
        );
        app.update_crash_recovery();
        assert!(!app.crash_recovery_worker.has_work());
        std::fs::remove_dir(&blocked).unwrap();
        step(&mut app);
        app.update_crash_recovery();
        app.poll_crash_recovery_worker(true);
        assert_eq!(app.crash_recovery_sequence, 2);
        assert_eq!(app.last_crash_recovery_command_count, 6);
        clean(app);
    }

    #[test]
    fn recovery_cleanup_and_restart_wait_for_the_previous_writer() {
        let mut app = app("restart");
        let release = gated_writer(&app);
        release.send(()).unwrap();
        app.remove_crash_recovery_files_except(None).unwrap();
        assert!(app.crash_recovery_worker.pending.borrow().is_none());
        assert!(app.crash_recovery_paths().iter().all(|path| !path.exists()));
        app.poll_crash_recovery_worker(false);
        let release = gated_writer(&app);
        release.send(()).unwrap();
        let old_seed = app.seed;
        app.restart_current_profile();
        assert_eq!(app.seed, old_seed + 1);
        let (saved, source) = AsciiApp::read_resume_candidate_from(&app.suspension_path).unwrap();
        assert_eq!(source, ResumeSource::CrashRecovery { sequence: 1 });
        assert_eq!(saved.seed, app.seed);
        assert_eq!(saved.state, suspension::fingerprint(&app.game));
        clean(app);
    }

    #[test]
    fn recovery_resume_reads_the_completed_writer_and_rearms_the_latest_state() {
        let mut app = app("resume");
        step(&mut app);
        let expected = suspension::fingerprint(&app.game);
        let release = gated_writer(&app);
        step(&mut app);
        release.send(()).unwrap();
        app.resume_run().unwrap();
        assert_eq!(app.history.len(), 1);
        assert_eq!(suspension::fingerprint(&app.game), expected);
        let (saved, source) = AsciiApp::read_resume_candidate_from(&app.suspension_path).unwrap();
        assert_eq!(source, ResumeSource::CrashRecovery { sequence: 3 });
        assert_eq!(saved.state, expected);
        clean(app);
    }

    #[test]
    fn recovery_character_creation_waits_and_requires_replacement_before_discarding() {
        let mut app = app("creation");
        step(&mut app);
        let release = gated_writer(&app);
        release.send(()).unwrap();
        let attributes = app
            .character_classes
            .iter()
            .next()
            .unwrap()
            .1
            .recommended_attributes();
        let mut creation = CharacterCreation {
            seed: INITIAL_SEED + 5,
            stage: CharacterCreationStage::Attributes,
            selected_class: 0,
            selected_attribute: 0,
            attributes,
            replace_suspension: false,
            message: String::new(),
            hovered: None,
        };
        assert!(app.rebuild_run_with_character_class(&creation).is_err());
        let (saved, source) = AsciiApp::read_resume_candidate_from(&app.suspension_path).unwrap();
        assert_eq!(source, ResumeSource::CrashRecovery { sequence: 2 });
        assert_eq!(saved.seed, INITIAL_SEED);
        let release = gated_writer(&app);
        release.send(()).unwrap();
        creation.replace_suspension = true;
        app.rebuild_run_with_character_class(&creation).unwrap();
        let (saved, source) = AsciiApp::read_resume_candidate_from(&app.suspension_path).unwrap();
        assert_eq!(source, ResumeSource::CrashRecovery { sequence: 1 });
        assert_eq!(saved.seed, creation.seed);
        assert_eq!(saved.state, suspension::fingerprint(&app.game));
        clean(app);
    }

    #[test]
    fn recovery_drop_joins_the_writer_before_releasing_the_session() {
        let app = app("drop");
        let path = app.suspension_path.clone();
        let release = gated_writer(&app);
        release.send(()).unwrap();
        drop(app);
        let (saved, source) = AsciiApp::read_resume_candidate_from(&path).unwrap();
        assert_eq!(source, ResumeSource::CrashRecovery { sequence: 2 });
        assert_eq!(saved.seed, INITIAL_SEED);
        for slot in AsciiApp::crash_recovery_paths_for(&path) {
            std::fs::remove_file(slot).unwrap();
        }
        let folder = path.parent().unwrap();
        let lock = suspension::session_lock(&folder.join("session.lock")).unwrap();
        drop(lock);
        std::fs::remove_file(folder.join("session.lock")).unwrap();
        std::fs::remove_dir(folder).unwrap();
    }
}
