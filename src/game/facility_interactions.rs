use super::*;
use crate::facility::InstallationAction;

impl WorldState {
    pub(super) fn use_installation(
        &mut self,
        target: GridPos,
        action: &InstallationAction,
    ) -> CommandOutcome {
        let reject = CommandOutcome::Rejected;
        if self.active.status != RunStatus::Active {
            return reject(CommandRejection::RunEnded);
        }
        if self.active.phase != TurnPhase::AwaitingPlayer {
            return reject(CommandRejection::NotPlayersTurn);
        }
        let Some(origin) = self.active.player_position() else {
            return reject(CommandRejection::MissingPlayer);
        };
        if !origin.cardinal_neighbors().contains(&target)
            || !self.active.player_visibility.is_visible(target)
        {
            return reject(CommandRejection::InteractionOutOfReach);
        }
        let Some(zone) = self.current.clone() else {
            return reject(CommandRejection::FacilityUnavailable);
        };
        let Some(facility) = self.facilities.get(&zone) else {
            return reject(CommandRejection::FacilityUnavailable);
        };
        let Some(installation) = facility.installation_at(target) else {
            return reject(CommandRejection::NothingToInteract);
        };
        if !facility.is_operational(installation.id()) {
            return reject(CommandRejection::FacilityUnavailable);
        }
        match action {
            InstallationAction::ReadIntel { index } => {
                let Some(intel) = facility.intel_at(target).get(usize::from(*index)).cloned()
                else {
                    return reject(CommandRejection::NothingToInteract);
                };
                if facility.intel_was_read(&intel.record) {
                    return CommandOutcome::AppliedWithoutTime;
                }
                let first_access = !self
                    .discovered_data_terminal_records()
                    .contains(&intel.record);
                let installation = installation.id().clone();
                self.facilities
                    .get_mut(&zone)
                    .unwrap()
                    .remember_intel(intel.record.clone());
                self.active
                    .events
                    .push(GameEvent::Facility(FacilityEvent::DataTerminalAccessed {
                        player: self.active.player,
                        installation,
                        at: target,
                        record: intel.record.clone(),
                        first_access,
                    }));
                self.record_data_record_quest_progress(&intel.record);
                if !intel.consumes_turn {
                    return CommandOutcome::AppliedWithoutTime;
                }
            }
            InstallationAction::SetPower { powered } => {
                let Some(current) = facility.power_setting_at(target) else {
                    return reject(CommandRejection::NothingToInteract);
                };
                if current == *powered {
                    return CommandOutcome::AppliedWithoutTime;
                }
                let facility = self.facilities.get_mut(&zone).unwrap();
                // A validated branch controls only real doors. Preflight on a copy
                // prevents malformed state from spending a turn or partly switching.
                let mut next = facility.clone();
                let mut map = self.active.map.clone();
                next.set_power_at(target, *powered);
                if next
                    .synchronize_interactive_outputs(
                        &mut map,
                        &self.active.actors,
                        Some(&self.active.ground_items),
                    )
                    .is_err()
                {
                    return reject(CommandRejection::FacilityUnavailable);
                }
                *facility = next;
                self.active.map = map;
                self.active.player_visibility.recompute(
                    &self.active.map,
                    origin,
                    self.active.rules.player_field_of_view,
                );
            }
            InstallationAction::ActivateDiversion => {
                let Some((charges, intensity, duration)) = facility.diversion_at(target) else {
                    return reject(CommandRejection::NothingToInteract);
                };
                if charges == 0 {
                    return reject(CommandRejection::FacilityUnavailable);
                }
                let Some(outlet) = facility.diversion_outlet_at(target) else {
                    return reject(CommandRejection::FacilityUnavailable);
                };
                if !self.active.map.is_walkable(outlet)
                    || self.active.rules.stealth_rules.is_none()
                    || self.active.sound_emitters.at(outlet).next().is_some()
                {
                    return reject(CommandRejection::FacilityUnavailable);
                }
                let Ok(emitter) = self
                    .active
                    .sound_emitters
                    .deploy(outlet, intensity, duration, 1)
                else {
                    return reject(CommandRejection::FacilityUnavailable);
                };
                self.facilities
                    .get_mut(&zone)
                    .unwrap()
                    .spend_diversion_charge(target);
                self.active.events.push(GameEvent::SoundEmitterDeployed {
                    entity: self.active.player,
                    emitter,
                    at: outlet,
                    intensity,
                    remaining_phases: duration,
                });
            }
        }
        self.active.complete_turn();
        CommandOutcome::Applied
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::{AiProfile, AiState, PursuitLifecycle};
    use crate::facility::{
        FacilityBlueprint, InstallationBlueprint, InstallationCapability, InstallationIntel,
        SecurityAlarmProfile,
    };
    use crate::game::GameRules;
    use crate::world::{DistanceMetric, DoorState, Terrain};
    const ORIGIN: GridPos = GridPos::new(2, 2);
    const PANEL: GridPos = GridPos::new(3, 2);
    const TERMINAL: GridPos = GridPos::new(2, 1);
    const POST: GridPos = GridPos::new(1, 2);
    const DOOR: GridPos = GridPos::new(4, 2);
    fn id(value: &str) -> ContentId {
        format!("test:{value}").parse().unwrap()
    }
    fn fixture(powered: bool) -> WorldState {
        let mut map = Map::from_ascii(
            "#########\n#...#...#\n#.......#\n#...#...#\n#...#...#\n#.......#\n#########",
        )
        .unwrap();
        map.set_terrain(DOOR, Terrain::Door(DoorState::Closed))
            .unwrap();
        let mut rules = GameRules {
            stealth_rules: Some(crate::stealth::StealthRules::default()),
            ..GameRules::default()
        };
        for name in ["material", "other_material"] {
            rules
                .items
                .register(
                    crate::item::ItemDefinition::new(
                        id(name),
                        "Material".into(),
                        "Material".into(),
                        4,
                        crate::item::ItemKind::Material,
                        None,
                        vec![],
                    )
                    .unwrap(),
                )
                .unwrap();
        }
        let mut world =
            WorldState::single(GameState::new_with_rules(map, ORIGIN, 42, rules).unwrap());
        world
            .enable(ZoneInfo {
                id: id("zone"),
                name: "Interactions".into(),
                kind: id("industrial"),
                depth: 0,
            })
            .unwrap();
        let make = |name, position, capability| InstallationBlueprint {
            id: id(name),
            position,
            integrity: 10,
            maximum_integrity: 10,
            capabilities: vec![capability],
            dependencies: vec![],
            security_alarm_profile: None,
        };
        let mut sensor = make(
            "sensor",
            GridPos::new(6, 2),
            InstallationCapability::SecuritySensor,
        );
        sensor.dependencies = vec![id("relay")];
        sensor.security_alarm_profile =
            Some(SecurityAlarmProfile::new(6, DistanceMetric::Euclidean, false, 10).unwrap());
        world
            .register_facility(
                id("zone"),
                FacilityBlueprint {
                    installations: vec![
                        make("depot", GridPos::new(1, 1), InstallationCapability::Storage),
                        make(
                            "terminal",
                            TERMINAL,
                            InstallationCapability::IntelTerminal {
                                records: vec![InstallationIntel {
                                    record: id("record"),
                                    label: "Issue".into(),
                                    target: Some(GridPos::new(1, 5)),
                                    consumes_turn: true,
                                }],
                            },
                        ),
                        make(
                            "panel",
                            PANEL,
                            InstallationCapability::PowerControl { relay: id("relay") },
                        ),
                        make(
                            "relay",
                            GridPos::new(3, 1),
                            InstallationCapability::SwitchableRelay {
                                door: DOOR,
                                powered,
                            },
                        ),
                        make(
                            "post",
                            POST,
                            InstallationCapability::DiversionPost {
                                outlet: POST,
                                charges: 2,
                                intensity: 60,
                                duration: 3,
                            },
                        ),
                        sensor,
                    ],
                    depot: id("depot"),
                    workers: vec![],
                    repair_orders: vec![],
                    maximum_path_search: 128,
                    owner: Some(id("owner")),
                },
            )
            .unwrap();
        world.drain_events();
        world
    }
    fn command(target: GridPos, action: InstallationAction) -> GameCommand {
        GameCommand::UseInstallation { target, action }
    }
    fn observer(world: &mut WorldState, position: GridPos) -> crate::entity::EntityId {
        world
            .spawn_actor(
                Actor::new(position, 20)
                    .unwrap()
                    .with_player_relation(crate::social::PlayerRelation::Hostile)
                    .with_ai(AiProfile::hunter(8, 0).with_pursuit_lifecycle(
                        PursuitLifecycle::new(
                            std::num::NonZeroU16::new(6).unwrap(),
                            std::num::NonZeroU16::new(2).unwrap(),
                            std::num::NonZeroU16::new(2).unwrap(),
                        ),
                    )),
            )
            .unwrap()
    }
    #[test]
    fn installation_intel_is_durable_and_rereading_is_free_without_revealing_terrain() {
        let mut world = fixture(false);
        let known = world.active.player_visibility.clone();
        assert_eq!(
            world.process_player_command(command(
                TERMINAL,
                InstallationAction::ReadIntel { index: 0 }
            )),
            CommandOutcome::Applied
        );
        assert!(
            world
                .discovered_data_terminal_records()
                .contains(&id("record"))
        );
        assert_eq!(world.active.player_visibility, known);
        world.drain_events();
        let bytes = world.recovery_snapshot_bytes().unwrap();
        assert_eq!(
            world.process_player_command(command(
                TERMINAL,
                InstallationAction::ReadIntel { index: 0 }
            )),
            CommandOutcome::AppliedWithoutTime
        );
        assert_eq!(bytes, world.recovery_snapshot_bytes().unwrap());
        let restored =
            WorldState::from_recovery_snapshot_bytes(&bytes, world.rules().clone()).unwrap();
        assert_eq!(bytes, restored.recovery_snapshot_bytes().unwrap());
        assert!(
            restored
                .active_facility()
                .unwrap()
                .intel_was_read(&id("record"))
        );
    }
    #[test]
    fn installation_invalid_queries_and_distance_refuse_atomically() {
        let mut world = fixture(false);
        for cmd in [
            command(TERMINAL, InstallationAction::ReadIntel { index: 8 }),
            command(GridPos::new(6, 2), InstallationAction::ActivateDiversion),
            command(PANEL, InstallationAction::ReadIntel { index: 0 }),
        ] {
            let before = world.recovery_snapshot_bytes().unwrap();
            assert!(matches!(
                world.process_player_command(cmd),
                CommandOutcome::Rejected(_)
            ));
            assert_eq!(before, world.recovery_snapshot_bytes().unwrap());
        }
    }
    #[test]
    fn installation_power_preserves_existing_alarms_and_defers_occupied_door() {
        let mut world = fixture(true);
        // A real unauthorized property event activates the existing security system.
        world
            .active
            .spawn_ground_item_with_owner(ORIGIN, id("material"), 1, Some(id("owner")))
            .unwrap();
        world.process_player_command(GameCommand::PickUp);
        assert!(
            world
                .active_facility()
                .unwrap()
                .security_alarm_at(GridPos::new(6, 2), world.turn())
                .is_some()
        );
        let occupant = world.spawn_actor(Actor::new(DOOR, 10).unwrap()).unwrap();
        assert_eq!(
            world.process_player_command(command(
                PANEL,
                InstallationAction::SetPower { powered: false }
            )),
            CommandOutcome::Applied
        );
        assert!(
            !world
                .active_facility()
                .unwrap()
                .is_operational(&id("sensor"))
        );
        assert!(
            world
                .active_facility()
                .unwrap()
                .security_alarm_at(GridPos::new(6, 2), world.turn())
                .is_some()
        );
        assert_eq!(
            world.map().tile(DOOR).unwrap().terrain,
            Terrain::Door(DoorState::Open)
        );
        world
            .active
            .actors
            .move_to(occupant, GridPos::new(5, 2))
            .unwrap();
        world.process_player_command(GameCommand::Wait);
        assert_eq!(
            world.map().tile(DOOR).unwrap().terrain,
            Terrain::Door(DoorState::Unpowered)
        );
        assert_eq!(
            world.process_player_command(command(
                PANEL,
                InstallationAction::SetPower { powered: true }
            )),
            CommandOutcome::Applied
        );
        assert!(
            world
                .active_facility()
                .unwrap()
                .is_operational(&id("sensor"))
        );
        assert_eq!(
            world.map().tile(DOOR).unwrap().terrain,
            Terrain::Door(DoorState::Open)
        );
    }
    #[test]
    fn installation_cut_keeps_ground_items_accessible_and_does_not_detect_new_theft() {
        let mut world = fixture(true);
        world
            .active
            .spawn_ground_item(DOOR, id("material"), 1)
            .unwrap();
        world.process_player_command(command(
            PANEL,
            InstallationAction::SetPower { powered: false },
        ));
        assert_eq!(
            world.map().tile(DOOR).unwrap().terrain,
            Terrain::Door(DoorState::Open)
        );
        world
            .active
            .spawn_ground_item_with_owner(ORIGIN, id("other_material"), 1, Some(id("owner")))
            .unwrap();
        world.process_player_command(GameCommand::PickUp);
        assert!(
            world
                .active_facility()
                .unwrap()
                .security_alarm_at(GridPos::new(6, 2), world.turn())
                .is_none()
        );
    }
    #[test]
    fn installation_diversion_investigates_sound_and_expires_without_refilling() {
        let mut world = fixture(false);
        let enemy = observer(&mut world, GridPos::new(6, 2));
        assert_eq!(
            world.process_player_command(command(POST, InstallationAction::ActivateDiversion)),
            CommandOutcome::Applied
        );
        assert!(
            matches!(world.actors().get(enemy).unwrap().ai_state(), AiState::Responding { incident, .. } if incident == POST)
        );
        world.drain_events();
        let bytes = world.recovery_snapshot_bytes().unwrap();
        let mut world =
            WorldState::from_recovery_snapshot_bytes(&bytes, world.rules().clone()).unwrap();
        assert_eq!(
            world
                .active_facility()
                .unwrap()
                .diversion_at(POST)
                .unwrap()
                .0,
            1
        );
        let before = world.recovery_snapshot_bytes().unwrap();
        assert!(matches!(
            world.process_player_command(command(POST, InstallationAction::ActivateDiversion)),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(before, world.recovery_snapshot_bytes().unwrap());
        world.process_player_command(GameCommand::Wait);
        world.process_player_command(GameCommand::Wait);
        assert!(world.active.sound_emitters.is_empty());
        world.process_player_command(command(POST, InstallationAction::ActivateDiversion));
        world.process_player_command(GameCommand::Wait);
        world.process_player_command(GameCommand::Wait);
        world.drain_events();
        let before = world.recovery_snapshot_bytes().unwrap();
        assert!(matches!(
            world.process_player_command(command(POST, InstallationAction::ActivateDiversion)),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(before, world.recovery_snapshot_bytes().unwrap());
    }
    #[test]
    fn installation_diversion_never_overrides_visible_player() {
        let mut world = fixture(true);
        let enemy = observer(&mut world, GridPos::new(5, 2));
        world.process_player_command(command(POST, InstallationAction::ActivateDiversion));
        assert!(
            matches!(world.actors().get(enemy).unwrap().ai_state(), AiState::Pursuing { last_seen, .. } if last_seen == ORIGIN)
        );
        assert_eq!(world.actor_perception_signals(enemy), Some((true, true)));
    }
}
