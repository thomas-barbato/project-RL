// Included in the engine test module to share its authored-skill fixtures.
fn modern_energy_game() -> GameState {
    let mut game = game_with_core_electronic_warfare(
        "#########\n#.......#\n#.......#\n#.......#\n#.......#\n#########",
        GridPos::new(2, 2), &["gel_01", "gel_03"],
    );
    game.rules.maintained_energy_reservations = true;
    game.rules.player_companion_limit = Some(5);
    game.player_bandwidth = None;
    game.player_energy = EnergyReserve::new(100, 100).unwrap();
    game
}

fn reserve_test_jamming(game: &mut GameState) {
    assert_eq!(use_electronic_warfare(game, "gel_03", ElectronicDirective::Jam {
        channel: ElectronicChannel::ControlLink,
    }), CommandOutcome::Applied);
}

#[test]
fn maintained_energy_multiple_effects_use_authored_costs_and_end_independently() {
    let mut game = modern_energy_game();
    game.player_skills = SkillProgressionState::from_ordered_choices(
        [("core:guerre_electronique".parse().unwrap(), vec!["core:gel_01".parse().unwrap(), "core:gel_03".parse().unwrap()]),
         ("core:furtivite".parse().unwrap(), ["fur_01", "fur_03", "fur_05", "fur_07", "fur_09"].into_iter().map(|s| format!("core:{s}").parse().unwrap()).collect())],
        &game.rules.skills, &game.rules.enabled_system_features, &game.rules.skill_progression,
    ).unwrap();
    reserve_test_jamming(&mut game);
    let jamming = game.player_energy.reserved();
    assert!(jamming > 0);
    assert_eq!(game.player_energy.available() + jamming, 100);
    let technique = "core:fur_09".parse().unwrap();
    assert_eq!(game.process_player_command(GameCommand::UseTechnique { technique, targets: vec![], weapon_slot: None }), CommandOutcome::Applied);
    assert_eq!(game.player_maintained_energy().len(), 2);
    assert_eq!(game.player_energy.reserved(), jamming + 10);
    assert_eq!(game.player_energy.available() + game.player_energy.reserved(), 100);
    assert_eq!(game.player_heat.unwrap().current(), 0);
    game.player_energy.spend(20).unwrap();
    assert_eq!(game.process_player_command(GameCommand::EndMaintainedEffect { effect: MaintainedEffectId::Camouflage }), CommandOutcome::Applied);
    assert_eq!(game.player_energy.reserved(), jamming);
    assert_eq!(game.player_energy.available() + jamming, 80);
    assert!(game.electronic_warfare.jamming().is_some());
    assert_eq!(game.process_player_command(GameCommand::EndMaintainedEffect { effect: MaintainedEffectId::Jamming }), CommandOutcome::Applied);
    assert_eq!((game.player_energy.available(), game.player_energy.reserved()), (80, 0));
    let before = (game.turn, game.player_energy);
    assert!(matches!(game.process_player_command(GameCommand::EndMaintainedEffect { effect: MaintainedEffectId::Jamming }), CommandOutcome::Rejected(_)));
    assert_eq!((game.turn, game.player_energy), before);
}

#[test]
fn maintained_energy_survives_zero_free_energy_and_refunds_once_on_expiry() {
    let mut game = modern_energy_game();
    reserve_test_jamming(&mut game);
    let reserved = game.player_energy.reserved();
    game.player_energy.spend(game.player_energy.available()).unwrap();
    game.process_player_command(GameCommand::Wait);
    assert!(game.electronic_warfare.jamming().is_some());
    assert_eq!(game.player_energy.available(), 0);
    for _ in 0..20 { game.process_player_command(GameCommand::Wait); }
    assert!(game.electronic_warfare.jamming().is_none());
    assert_eq!((game.player_energy.available(), game.player_energy.reserved()), (reserved, 0));
}

#[test]
fn maintained_energy_regenerates_only_free_capacity_and_rejects_unaffordable_activation() {
    let mut game = modern_energy_game();
    game.player_energy.spend(99).unwrap();
    let before = (game.turn, game.player_energy);
    assert!(matches!(use_electronic_warfare(&mut game, "gel_03", ElectronicDirective::Jam { channel: ElectronicChannel::ControlLink }), CommandOutcome::Rejected(_)));
    assert_eq!((game.turn, game.player_energy), before);
    assert!(game.player_maintained_energy().is_empty());
    game.player_energy.restore(100);
    game.rules.player_energy_regeneration = 1;
    reserve_test_jamming(&mut game);
    assert_eq!(game.player_energy.available(), game.player_energy.usable_capacity());
    game.player_energy.spend(2).unwrap();
    for _ in 0..2 { game.process_player_command(GameCommand::Wait); }
    assert!(game.player_energy.reserved() > 0);
    assert_eq!(game.player_energy.available(), game.player_energy.usable_capacity());
}

#[test]
fn maintained_companion_slots_count_origins_exclude_quest_escorts_and_can_be_reused() {
    use crate::companion::CompanionOrigin;
    let mut game = modern_energy_game();
    let mut first = None;
    for (i, origin) in [CompanionOrigin::Summoned, CompanionOrigin::Recruited, CompanionOrigin::Purchased, CompanionOrigin::Summoned, CompanionOrigin::Recruited].into_iter().enumerate() {
        let entity = game.spawn_actor(Actor::new(GridPos::new(1 + i as i32, 1), 10).unwrap().with_companion_origin(origin)).unwrap();
        first.get_or_insert(entity);
    }
    let escort = game.spawn_actor(Actor::new(GridPos::new(7, 1), 10).unwrap().with_companion_origin(CompanionOrigin::QuestEscort)).unwrap();
    assert_eq!(game.player_companion_count(), 5);
    let sixth = Actor::new(GridPos::new(7, 2), 10).unwrap().with_companion_origin(CompanionOrigin::Purchased);
    assert!(matches!(game.spawn_actor(sixth.clone()), Err(SpawnError::CompanionLimit { maximum: 5 })));
    assert!(matches!(game.process_player_command(GameCommand::DismissCompanion { entity: escort }), CommandOutcome::Rejected(_)));
    assert_eq!(game.process_player_command(GameCommand::DismissCompanion { entity: first.unwrap() }), CommandOutcome::Applied);
    assert_eq!(game.player_companion_count(), 4);
    game.spawn_actor(sixth).unwrap();
    assert_eq!(game.player_companion_count(), 5);
    game.rules.player_companion_limit = Some(6);
    game.spawn_actor(Actor::new(GridPos::new(7, 3), 10).unwrap().with_companion_origin(CompanionOrigin::Summoned)).unwrap();
    assert_eq!(game.player_companion_count(), 6);
}

#[test]
fn maintained_energy_capacity_reduction_stops_effects_without_orphaned_reservations() {
    let mut game = modern_energy_game();
    reserve_test_jamming(&mut game);
    game.fit_maintained_energy_capacity(0);
    game.player_energy.set_capacity(0);
    assert!(game.electronic_warfare.jamming().is_none());
    assert!(game.player_maintained_energy().is_empty());
    assert_eq!((game.player_energy.available(), game.player_energy.reserved()), (0, 0));
}

#[test]
fn maintained_energy_native_snapshot_preserves_reservation_and_refund() {
    let mut game = modern_energy_game();
    reserve_test_jamming(&mut game);
    let rules = game.rules.clone();
    let mut world = crate::game::WorldState::single(game);
    world.drain_events();
    let bytes = world.recovery_snapshot_bytes().unwrap();
    let mut restored = crate::game::WorldState::from_recovery_snapshot_bytes(&bytes, rules).unwrap();
    assert_eq!(bytes, restored.recovery_snapshot_bytes().unwrap());
    assert!(restored.player_energy().reserved() > 0);
    assert_eq!(restored.process_player_command(GameCommand::EndMaintainedEffect { effect: MaintainedEffectId::Jamming }), CommandOutcome::Applied);
    assert_eq!((restored.player_energy().available(), restored.player_energy().reserved()), (100, 0));
}

#[test]
fn maintained_companion_sixth_invocation_is_rejected_without_cost_or_time() {
    use crate::companion::CompanionOrigin;
    let mut game = game_with_core_drones("#########\n#.......#\n#.......#\n#.......#\n#########", GridPos::new(2, 2), &["drn_01"]);
    game.rules.maintained_energy_reservations = true;
    game.rules.player_companion_limit = Some(5);
    game.player_bandwidth = None;
    for x in 1..=5 {
        game.spawn_actor(Actor::new(GridPos::new(x, 1), 10).unwrap().with_companion_origin(CompanionOrigin::Recruited)).unwrap();
    }
    let before = (game.turn, game.player_energy);
    assert!(matches!(use_drone_technique(&mut game, "drn_01", DroneDirective::Manifest { position: GridPos::new(3, 2) }), CommandOutcome::Rejected(_)));
    assert_eq!((game.turn, game.player_energy), before);
    let dismissed = game.actors.entity_at(GridPos::new(1, 1)).unwrap();
    assert_eq!(game.process_player_command(GameCommand::DismissCompanion { entity: dismissed }), CommandOutcome::Applied);
    assert_eq!(use_drone_technique(&mut game, "drn_01", DroneDirective::Manifest { position: GridPos::new(3, 2) }), CommandOutcome::Applied);
    assert_eq!(game.player_companion_count(), 5);
    assert!(game.player_energy.available() < 100);
    assert_eq!(game.player_energy.reserved(), 0);
}

#[test]
fn maintained_energy_subnet_reserves_one_total_and_releases_each_device_share() {
    let mut game = game_with_core_intrusion(
        "########\n#......#\n#......#\n#......#\n########", GridPos::new(1, 2),
        &["int_01", "int_03", "int_05", "int_07", "int_09"],
    );
    game.rules.maintained_energy_reservations = true;
    game.player_bandwidth = None;
    let panels = [GridPos::new(2, 1), GridPos::new(2, 2), GridPos::new(2, 3)];
    for (panel, door) in panels.into_iter().zip([GridPos::new(3, 1), GridPos::new(3, 2), GridPos::new(3, 3)]) {
        install_intrusion_panel(&mut game, panel, door);
        grant_intrusion_access(&mut game, panel);
    }
    let before = game.player_energy.available();
    for _ in 0..3 {
        assert_eq!(use_intrusion(&mut game, "int_09", IntrusionDirective::Subnet { positions: panels.to_vec(), command: DeviceCommand::Open }), CommandOutcome::Applied);
    }
    let reserved = game.player_energy.reserved();
    assert!(reserved > 0);
    assert_eq!(game.player_maintained_energy().len(), 3);
    assert_eq!(game.player_energy.available() + reserved, before);
    let reservation = game.player_maintained_energy().into_iter().find(|r| r.effect == MaintainedEffectId::Device(panels[0])).unwrap();
    game.end_maintained_effect(reservation.effect).unwrap();
    assert_eq!(game.player_energy.reserved(), reserved - reservation.amount);
    assert!(game.intrusion.control(panels[0]).is_none());
    assert_eq!(game.intrusion.controls().count(), 2);
    game.end_local_maintained_effects_for_travel();
    assert_eq!((game.player_energy.available(), game.player_energy.reserved()), (before, 0));
    assert_eq!(game.intrusion.controls().count(), 0);
}
