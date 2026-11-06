//! VISION-U01-05: accepted real sight of six cells, including existing agent runs.
use super::*;

impl AsciiApp {
    pub(super) fn migrate_circular_player_vision(&mut self) -> Result<(), String> {
        if self.generation_version >= CRYOGENIC_CAMP_GENERATION_VERSION
            && !self.game.rules().circular_player_vision
        {
            if self.execute_command(GameCommand::EnableCircularPlayerVision)
                != CommandOutcome::AppliedWithoutTime
            {
                return Err("Circular player vision adoption failed".into());
            }
        }
        Ok(())
    }

    pub(super) fn migrate_street_collisions(&mut self) -> Result<(), String> {
        self.street_collision_adoption_enabled = true;
        let zone = crate::prison_street::ZONE_ID.parse().unwrap();
        if self
            .game
            .passage_in(
                &zone,
                crate::prison_street::pos(crate::prison_street::expanded_scene().entrance),
            )
            .is_none()
        {
            return Ok(());
        }
        if self.generation_version < STREET_WAREHOUSE_CLEARANCE_GENERATION_VERSION {
            for (version, cells) in [
                (
                    CIRCULAR_VISION_REPAIRS_GENERATION_VERSION,
                    crate::prison_street::collision_repair_cells()?,
                ),
                (
                    STREET_CONTACT_REPAIRS_GENERATION_VERSION,
                    crate::prison_street::contact_repair_cells()?,
                ),
                (
                    STREET_WAREHOUSE_CLEARANCE_GENERATION_VERSION,
                    crate::prison_street::warehouse_clearance_cells()?,
                ),
            ] {
                if self.generation_version < version {
                    self.repair_unrecorded_street_cells(&zone, cells)?;
                }
            }
        }
        self.repair_unrecorded_street_cells(
            &zone,
            crate::prison_street::expanded_scene()
                .courtyard_clearance
                .iter()
                .copied()
                .map(crate::prison_street::pos),
        )
    }

    fn repair_unrecorded_street_cells(
        &mut self,
        zone: &ContentId,
        cells: impl IntoIterator<Item = GridPos>,
    ) -> Result<(), String> {
        let cells: Vec<_> = cells
            .into_iter()
            .filter(|cell| !Self::street_repair_recorded(&self.history, [cell.x, cell.y]))
            .collect();
        if !cells.is_empty()
            && self.execute_command(GameCommand::RepairStaticZoneFloors {
                zone: zone.clone(),
                cells,
            }) != CommandOutcome::AppliedWithoutTime
        {
            return Err("Street collision adoption failed".into());
        }
        Ok(())
    }

    fn street_repair_recorded(history: &[RecordedCommand], cell: [i32; 2]) -> bool {
        history.iter().any(|command| {
            matches!(command, RecordedCommand::RepairStaticZoneFloors { zone, cells }
                if zone == crate::prison_street::ZONE_ID && cells.contains(&cell))
        })
    }

    pub(super) fn restore_courtyard_clearance(
        world: &mut WorldState,
        history: &[RecordedCommand],
    ) -> Result<(), String> {
        // The map snapshot omits derived obstructions. Restore the recorded
        // removal after the historical obstacles have been reconstructed.
        let cells: Vec<_> = crate::prison_street::expanded_scene()
            .courtyard_clearance
            .iter()
            .copied()
            .filter(|&cell| Self::street_repair_recorded(history, cell))
            .map(|cell| (crate::prison_street::pos(cell), false, false))
            .collect();
        if !cells.is_empty() {
            let zone = crate::prison_street::ZONE_ID.parse().unwrap();
            world.restore_static_zone_obstructions(&zone, &cells)?;
        }
        Ok(())
    }

    pub(super) fn migrate_player_vision(&mut self) -> Result<(), String> {
        // Older campaigns retain their historical world and perception. Agent
        // runs adopt the new sight only after complete suspension verification.
        if self.generation_version < CRYOGENIC_CAMP_GENERATION_VERSION
            || self.game.rules().player_field_of_view.radius == 6
        {
            return Ok(());
        }
        if self.execute_command(GameCommand::UpgradePlayerVision)
            != CommandOutcome::AppliedWithoutTime
        {
            return Err("Player vision migration was rejected".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cryogenic_camp::{self as camp, pos};

    #[test]
    fn courtyard_clearance_survives_old_and_new_street_suspensions() {
        let cells = [[11, 39], [11, 40], [12, 39], [12, 40]].map(pos);
        for historical in [false, true] {
            let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
            let mut app = AsciiApp::from_seed(1872, rules, texts, loot, expeditions).unwrap();
            app.crash_recovery_enabled = false;
            if historical {
                app.street_collision_adoption_enabled = false;
            }
            let exit = pos(camp::scene().exits[0]);
            app.walk_fixture_to_unchecked(exit).unwrap();
            assert_eq!(
                app.execute_command(GameCommand::Interact { target: exit }),
                CommandOutcome::Applied
            );
            assert!(cells
                .iter()
                .all(|&cell| app.game.map().is_walkable(cell) != historical));
            app.capture_events_at(Some(0.));
            let saved = app.suspension().unwrap();
            for replay in [false, true] {
                let mut source = saved.clone();
                if replay {
                    source.recovery = Some(RecoveryPayload::replay(source.commands.len()).unwrap());
                }
                let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
                let mut restored =
                    AsciiApp::restore_suspension(&source, rules, texts, loot, expeditions).unwrap();
                assert_eq!(suspension::fingerprint(&restored.game), saved.state);
                let turn = restored.game.turn();
                restored.migrate_street_collisions().unwrap();
                assert_eq!(restored.game.turn(), turn);
                assert!(cells.iter().all(|&cell| restored.game.map().is_walkable(cell)));
                assert!(!restored.game.map().is_walkable(pos([10, 42])));
                let commands = restored.history.len();
                restored.migrate_street_collisions().unwrap();
                assert_eq!(restored.history.len(), commands);
                let after = restored.suspension().unwrap();
                let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
                let reopened =
                    AsciiApp::restore_suspension(&after, rules, texts, loot, expeditions).unwrap();
                assert_eq!(suspension::fingerprint(&reopened.game), after.state);
                assert!(cells.iter().all(|&cell| reopened.game.map().is_walkable(cell)));
            }
        }
    }

    #[test]
    fn contact_repairs_adopt_existing_streets_once_and_survive_snapshot_and_replay() {
        for version in [
            SIX_CELL_VISION_GENERATION_VERSION,
            CIRCULAR_VISION_REPAIRS_GENERATION_VERSION,
            STREET_CONTACT_REPAIRS_GENERATION_VERSION,
        ] {
            for visited in [false, true] {
                let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
                let mut old =
                    AsciiApp::from_seed_version(1872, rules, texts, loot, expeditions, version)
                        .unwrap();
                old.crash_recovery_enabled = false;
                let exit = pos(camp::scene().exits[0]);
                if visited {
                    old.walk_fixture_to_unchecked(exit).unwrap();
                    assert_eq!(
                        old.execute_command(GameCommand::Interact { target: exit }),
                        CommandOutcome::Applied
                    );
                    assert!(!old.game.map().is_walkable(GridPos::new(11, 7)));
                }
                old.capture_events_at(Some(0.));
                let saved = old.suspension().unwrap();
                for replay in [false, true] {
                    let mut source = saved.clone();
                    if replay {
                        source.recovery =
                            Some(RecoveryPayload::replay(source.commands.len()).unwrap());
                    }
                    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
                    let mut app =
                        AsciiApp::restore_suspension(&source, rules, texts, loot, expeditions)
                            .unwrap();
                    assert_eq!(suspension::fingerprint(&app.game), source.state);
                    let turn = app.game.turn();
                    app.migrate_circular_player_vision().unwrap();
                    app.migrate_street_collisions().unwrap();
                    assert_eq!(app.game.turn(), turn);
                    let history = app.history.clone();
                    app.migrate_street_collisions().unwrap();
                    assert_eq!(app.history, history);
                    if !visited {
                        app.walk_fixture_to_unchecked(exit).unwrap();
                        assert_eq!(
                            app.execute_command(GameCommand::Interact { target: exit }),
                            CommandOutcome::Applied
                        );
                    }
                    for at in [
                        [11, 7],
                        [14, 16],
                        [15, 40],
                        [2, 30],
                        [2, 31],
                        [15, 32],
                        [16, 32],
                        [17, 32],
                    ] {
                        assert!(
                            app.game.map().is_walkable(GridPos::new(at[0], at[1])),
                            "{version}/{visited}/{replay}: {at:?}"
                        );
                    }
                    assert!(!app.game.map().is_walkable(GridPos::new(16, 33)));
                    app.capture_events_at(Some(0.));
                    let after = app.suspension().unwrap();
                    for replay_after in [false, true] {
                        let mut source = after.clone();
                        if replay_after {
                            source.recovery =
                                Some(RecoveryPayload::replay(source.commands.len()).unwrap());
                        }
                        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
                        let restored =
                            AsciiApp::restore_suspension(&source, rules, texts, loot, expeditions)
                                .unwrap();
                        assert_eq!(
                            restored.game.recovery_snapshot_bytes().unwrap(),
                            app.game.recovery_snapshot_bytes().unwrap()
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn reported_row_31_is_repaired_in_existing_visited_and_deferred_streets_and_reloads() {
        for visited in [false, true] {
            let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
            let mut old = AsciiApp::from_seed_version(
                1872,
                rules,
                texts,
                loot,
                expeditions,
                SIX_CELL_VISION_GENERATION_VERSION,
            )
            .unwrap();
            old.crash_recovery_enabled = false;
            let exit = pos(camp::scene().exits[0]);
            if visited {
                old.walk_fixture_to_unchecked(exit).unwrap();
                assert_eq!(
                    old.execute_command(GameCommand::Interact { target: exit }),
                    CommandOutcome::Applied
                );
                old.walk_fixture_to_unchecked(GridPos::new(16, 31)).unwrap();
                assert!(!old.game.map().is_walkable(GridPos::new(15, 31)));
            }
            let saved = old.suspension().unwrap();
            for replay in [false, true] {
                let mut source = saved.clone();
                if replay {
                    source.recovery = Some(RecoveryPayload::replay(source.commands.len()).unwrap());
                }
                let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
                let mut app =
                    AsciiApp::restore_suspension(&source, rules, texts, loot, expeditions).unwrap();
                assert_eq!(suspension::fingerprint(&app.game), source.state);
                let turn = app.game.turn();
                app.migrate_player_vision().unwrap();
                app.migrate_circular_player_vision().unwrap();
                app.migrate_street_collisions().unwrap();
                assert_eq!(app.game.turn(), turn);
                let history = app.history.clone();
                app.migrate_circular_player_vision().unwrap();
                app.migrate_street_collisions().unwrap();
                assert_eq!(app.history, history);
                if !visited {
                    app.walk_fixture_to_unchecked(exit).unwrap();
                    assert_eq!(
                        app.execute_command(GameCommand::Interact { target: exit }),
                        CommandOutcome::Applied
                    );
                    app.walk_fixture_to_unchecked(GridPos::new(16, 31)).unwrap();
                }
                let before = app.game.recovery_snapshot_bytes().unwrap();
                let zone = crate::prison_street::ZONE_ID.parse().unwrap();
                assert!(matches!(
                    app.execute_command(GameCommand::RepairStaticZoneFloors {
                        zone,
                        cells: vec![GridPos::new(15, 31), GridPos::new(999, 999)]
                    }),
                    CommandOutcome::Rejected(_)
                ));
                assert_eq!(app.game.recovery_snapshot_bytes().unwrap(), before);
                let turn = app.game.turn();
                for x in (7..16).rev() {
                    assert!(app.game.map().is_walkable(GridPos::new(x, 31)));
                    assert_eq!(
                        app.execute_command(GameCommand::Move(Direction::West)),
                        CommandOutcome::Applied
                    );
                    assert_eq!(app.game.player_position(), Some(GridPos::new(x, 31)));
                }
                assert_eq!(app.game.turn(), turn + 9);
                app.capture_events_at(Some(0.));
                let saved = app.suspension().unwrap();
                let expected = app.game.recovery_snapshot_bytes().unwrap();
                for replay_after in [false, true] {
                    let mut source = saved.clone();
                    if replay_after {
                        source.recovery =
                            Some(RecoveryPayload::replay(source.commands.len()).unwrap());
                    }
                    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
                    let restored =
                        AsciiApp::restore_suspension(&source, rules, texts, loot, expeditions)
                            .unwrap();
                    assert!(restored.game.rules().circular_player_vision);
                    assert_eq!(restored.game.recovery_snapshot_bytes().unwrap(), expected);
                }
            }
        }
    }

    #[test]
    fn legacy_vision_is_verified_then_adopted_and_survives_snapshot_and_replay() {
        for version in [
            CRYOGENIC_CAMP_GENERATION_VERSION,
            STREET_EXPANSION_GENERATION_VERSION,
        ] {
            let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
            let old =
                AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, version)
                    .unwrap();
            let saved = old.suspension().unwrap();
            let original = serde_json::to_vec(&saved).unwrap();
            assert_eq!(old.game.rules().player_field_of_view.radius, 12);
            for replay in [false, true] {
                let mut candidate = saved.clone();
                if replay {
                    candidate.recovery =
                        Some(RecoveryPayload::replay(candidate.commands.len()).unwrap());
                }
                let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
                let mut restored =
                    AsciiApp::restore_suspension(&candidate, rules, texts, loot, expeditions)
                        .unwrap();
                assert_eq!(suspension::fingerprint(&restored.game), saved.state);
                restored.migrate_weapon_sets().unwrap();
                let position = restored.game.player_position();
                let turn = restored.game.turn();
                let inventory = format!("{:?}", restored.game.player_inventory());
                restored.migrate_player_vision().unwrap();
                assert_eq!(restored.game.rules().player_field_of_view.radius, 6);
                assert_eq!(
                    restored
                        .game
                        .rules()
                        .ordinary_actor_field_of_view
                        .unwrap()
                        .radius,
                    8
                );
                assert_eq!(restored.game.player_position(), position);
                assert_eq!(restored.game.turn(), turn);
                assert_eq!(format!("{:?}", restored.game.player_inventory()), inventory);
                let history = restored.history.clone();
                restored.migrate_player_vision().unwrap();
                assert_eq!(restored.history, history, "adoption must be idempotent");
                assert_eq!(history.last(), Some(&RecordedCommand::UpgradePlayerVision));
                let next = restored.suspension().unwrap();
                let expected = restored.game.recovery_snapshot_bytes().unwrap();
                for replay_next in [false, true] {
                    let mut next = next.clone();
                    if replay_next {
                        next.recovery = Some(RecoveryPayload::replay(next.commands.len()).unwrap());
                    }
                    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
                    let resumed =
                        AsciiApp::restore_suspension(&next, rules, texts, loot, expeditions)
                            .unwrap();
                    assert_eq!(resumed.game.rules().player_field_of_view.radius, 6);
                    assert_eq!(resumed.game.recovery_snapshot_bytes().unwrap(), expected);
                }
            }
            assert_eq!(serde_json::to_vec(&saved).unwrap(), original);
            let mut damaged = saved;
            damaged.state ^= 1;
            let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
            assert!(
                AsciiApp::restore_suspension(&damaged, rules, texts, loot, expeditions).is_err()
            );
        }
    }

    #[test]
    fn new_runs_use_six_and_old_rule_fingerprints_remain_unchanged() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        for version in [
            ONE_HAND_FIREARMS_GENERATION_VERSION,
            PLAYER_VISION_GENERATION_VERSION,
            CRYOGENIC_CAMP_GENERATION_VERSION,
            STREET_EXPANSION_GENERATION_VERSION,
        ] {
            let mut historical = rules.clone();
            historical.player_field_of_view.radius = 12;
            assert_eq!(
                rules_fingerprint_for_version(&rules, version),
                rules_fingerprint_for_version(&historical, version)
            );
        }
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        assert_eq!(app.generation_version, CURRENT_GENERATION_VERSION);
        assert_eq!(app.game.rules().player_field_of_view.radius, 6);
        let history = app.history.clone();
        app.migrate_player_vision().unwrap();
        assert_eq!(app.history, history);
    }
}
