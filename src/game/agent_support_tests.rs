use super::*;
use crate::skills::agent;

fn arena() -> GameState {
    agent_cast::tests::fixture()
}
fn learn(g: &mut GameState, id: &str) {
    g.player_skills.set_agent_skill_rank(id, 1);
}
fn enemy(g: &mut GameState, p: GridPos) -> EntityId {
    g.spawn_actor(
        Actor::new(p, 500)
            .unwrap()
            .with_evasion_disabled()
            .with_player_relation(crate::social::PlayerRelation::Hostile),
    )
    .unwrap()
}
fn cast(g: &mut GameState, id: &str, at: GridPos) -> CommandOutcome {
    g.process_player_command(GameCommand::UseTechniqueAt {
        technique: agent::technique(id),
        target: at,
        weapon_slot: 0,
    })
}

#[test]
fn every_catalogue_active_has_a_real_use_profile_or_a_concrete_weapon_condition() {
    let mut g = arena();
    for s in agent::definitions() {
        g.player_skills.set_agent_skill_rank(&s.id, 1);
    }
    for s in agent::definitions().iter().filter(|s| s.kind == "active") {
        let result = g.agent_skill_ready_with_weapon(&s.id, 0);
        assert!(
            !matches!(result, Err(CommandRejection::TechniqueHasNoActiveAction(_))),
            "unrouted {}: {result:?}",
            s.id
        );
    }
}

#[test]
fn aegis_mitigates_direct_hits_but_not_periodic_damage_and_expires() {
    let mut g = arena();
    learn(&mut g, "B02");
    let at = g.player_position().unwrap();
    assert_eq!(cast(&mut g, "B02", at), CommandOutcome::Applied);
    let hp = g.actors.get(g.player).unwrap().integrity();
    let packet = DamagePacket::new(8, crate::combat::DamageType::Corruption, 0);
    assert_eq!(g.apply_damage_to(None, g.player, packet).unwrap().amount, 6);
    assert_eq!(
        g.apply_status_damage_to(None, g.player, packet)
            .unwrap()
            .amount,
        8
    );
    assert_eq!(g.actors.get(g.player).unwrap().integrity(), hp - 14);
    for _ in 0..3 {
        g.process_player_command(GameCommand::Wait);
    }
    assert!(g.agent_support_effect(g.player, "B02").is_none());
}

#[test]
fn purge_removes_burning_and_its_saved_power_without_healing() {
    let mut g = arena();
    learn(&mut g, "SU05");
    learn(&mut g, "FE01");
    let power = g.agent_spell_burn_profile("FE01", 1).unwrap().0;
    g.apply_agent_burn(g.player, power, 3);
    let hp = g.actors.get(g.player).unwrap().integrity();
    let at = g.player_position().unwrap();
    assert_eq!(cast(&mut g, "SU05", at), CommandOutcome::Applied);
    assert!(
        g.actors
            .get(g.player)
            .unwrap()
            .status(&"core:burning".parse().unwrap())
            .is_none()
    );
    assert_eq!(g.actors.get(g.player).unwrap().integrity(), hp);
    let state = format!("{g:?}");
    assert!(matches!(
        cast(&mut g, "SU05", at),
        CommandOutcome::Rejected(_)
    ));
    assert_eq!(state, format!("{g:?}"));
}

#[test]
fn regeneration_has_three_paid_heals_and_interrupts_when_hit() {
    let mut g = arena();
    learn(&mut g, "SU04");
    g.actors.get_mut(g.player).unwrap().apply_damage(20);
    let hp = g.actors.get(g.player).unwrap().integrity();
    let energy = g.player_energy.available();
    let cost = g.agent_support_profile("SU04", 1).unwrap().energy;
    let at = g.player_position().unwrap();
    assert_eq!(cast(&mut g, "SU04", at), CommandOutcome::Applied);
    g.process_player_command(GameCommand::Wait);
    g.process_player_command(GameCommand::Wait);
    assert_eq!(g.player_energy.available(), energy - cost * 3);
    assert!(g.actors.get(g.player).unwrap().integrity() > hp);
    assert!(g.agent_runtime().support.regeneration.is_none());
    cast(&mut g, "SU04", at);
    g.apply_damage_to(
        None,
        g.player,
        DamagePacket::new(1, crate::combat::DamageType::Corruption, 0),
    )
    .unwrap();
    assert!(g.agent_runtime().support.regeneration.is_none());
}

#[test]
fn spectral_step_accepts_shorter_distances_and_refuses_blocked_or_longer_paths_atomically() {
    let mut g = arena();
    learn(&mut g, "T02");
    let start = g.player_position().unwrap();
    let short = GridPos::new(start.x + 1, start.y);
    assert_eq!(cast(&mut g, "T02", short), CommandOutcome::Applied);
    assert_eq!(g.player_position(), Some(short));
    let before = format!("{g:?}");
    assert!(matches!(
        cast(&mut g, "T02", GridPos::new(8, 3)),
        CommandOutcome::Rejected(_)
    ));
    assert_eq!(before, format!("{g:?}"));
}

#[test]
fn last_surge_saves_one_fatal_direct_hit_then_cannot_save_a_second() {
    let mut g = arena();
    learn(&mut g, "SU06");
    let packet = DamagePacket::new(600, crate::combat::DamageType::Corruption, 0);
    g.apply_damage_to(None, g.player, packet).unwrap();
    assert_eq!(g.actors.get(g.player).unwrap().integrity(), 1);
    g.apply_damage_to(None, g.player, packet).unwrap();
    assert!(g.actors.get(g.player).is_none());
    let mut g = arena();
    learn(&mut g, "SU06");
    g.apply_status_damage_to(None, g.player, packet).unwrap();
    assert!(g.actors.get(g.player).is_none());
}

#[test]
fn reanimation_marks_a_living_hostile_then_raises_it_only_after_death() {
    let mut g = arena();
    learn(&mut g, "OC07");
    let at = GridPos::new(4, 3);
    let victim = enemy(&mut g, at);
    assert_eq!(cast(&mut g, "OC07", at), CommandOutcome::Applied);
    assert_eq!(g.actors.get(victim).unwrap().integrity(), 500);
    assert!(g.agent_runtime().support.raised.is_empty());
    g.apply_damage_to(
        Some(g.player),
        victim,
        DamagePacket::new(600, crate::combat::DamageType::Corruption, 0),
    )
    .unwrap();
    assert!(g.actors.get(victim).is_none());
    let raised = g.actors.entity_at(at).unwrap();
    assert!(g.is_player_controlled_companion(raised));
    assert!(g.actors.get(raised).unwrap().defeat_reward().is_none());
    assert_eq!(
        agent::definition("OC07")
            .unwrap()
            .value("maximum_raised_allies", 1),
        5
    );
}

#[test]
fn regeneration_break_blocks_potions_and_all_other_integrity_restoration() {
    let mut g = arena();
    g.apply_agent_support_status("AF04", 1, g.player, 3);
    let player = g.player;
    g.actors.get_mut(player).unwrap().apply_damage(10);
    let before = g.actors.get(player).unwrap().integrity();
    assert_eq!(g.actors.get_mut(player).unwrap().restore_integrity(10), 0);
    assert_eq!(g.actors.get(player).unwrap().integrity(), before);
}

#[test]
fn support_nonempty_snapshot_roundtrip_preserves_mitigation_and_curse_timer() {
    let mut g = arena();
    learn(&mut g, "B02");
    let at = g.player_position().unwrap();
    cast(&mut g, "B02", at);
    g.drain_events();
    let world = crate::game::WorldState::single(g);
    let bytes = world.recovery_snapshot_bytes().unwrap();
    assert!(bytes.starts_with(b"RLWS\x0f"));
    let mut restored =
        crate::game::WorldState::from_recovery_snapshot_bytes(&bytes, world.rules().clone())
            .unwrap();
    assert!(
        restored
            .agent_support_effect(restored.player, "B02")
            .is_some()
    );
    let player = restored.player;
    assert_eq!(
        restored
            .apply_damage_to(
                None,
                player,
                DamagePacket::new(8, crate::combat::DamageType::Corruption, 0)
            )
            .unwrap()
            .amount,
        6
    );
}

#[test]
fn last_surge_waits_until_after_cuirassier_mitigation() {
    let mut g = arena();
    learn(&mut g, "SU06");
    learn(&mut g, "TR63");
    g.player_energy = crate::resources::EnergyReserve::new(60000, 60000).unwrap();
    let at = g.player_position().unwrap();
    g.cast_agent_transformation("TR63", at, 0).unwrap();
    let hp = g.actors.get(g.player).unwrap().integrity();
    let reduced = g.agent_transform_reduce_direct(g.player, hp, true);
    assert!(reduced > 0 && reduced < hp);
    g.apply_damage_to(
        None,
        g.player,
        DamagePacket::new(hp, crate::combat::DamageType::Corruption, 0),
    )
    .unwrap();
    assert_eq!(g.actors.get(g.player).unwrap().integrity(), hp - reduced);
    assert_eq!(g.agent_runtime().support.surge_ready, 0);
    g.apply_damage_to(
        None,
        g.player,
        DamagePacket::new(60000, crate::combat::DamageType::Corruption, 0),
    )
    .unwrap();
    assert_eq!(g.actors.get(g.player).unwrap().integrity(), 1);
    assert!(g.agent_runtime().support.surge_ready > g.turn);
}

#[test]
fn support_power_survives_an_absent_target_until_its_absolute_expiry() {
    let mut g = arena();
    let victim = enemy(&mut g, GridPos::new(4, 3));
    g.apply_agent_support_status("AF02", 1, victim, 3);
    let actor = g.actors.remove(victim).unwrap();
    g.process_agent_support_turn();
    assert!(
        g.agent_runtime()
            .support
            .effects
            .iter()
            .any(|e| e.target == victim)
    );
    // World travel restores the actor with the same globally unique ID.
    let expires = g
        .agent_runtime()
        .support
        .effects
        .iter()
        .find(|e| e.target == victim)
        .unwrap()
        .expires;
    assert!(actor.statuses().next().is_some());
    g.turn = expires;
    g.process_agent_support_turn();
    assert!(
        !g.agent_runtime()
            .support
            .effects
            .iter()
            .any(|e| e.target == victim)
    );
}

#[test]
fn starting_vitality_is_applied_before_the_first_action() {
    let base = arena();
    let maximum = base.actors.get(base.player).unwrap().maximum_integrity();
    let mut rules = base.rules.clone();
    rules
        .agent_class_start
        .as_mut()
        .unwrap()
        .skills
        .push("B03".into());
    let mut game =
        GameState::new_with_rules(base.map.clone(), base.player_position().unwrap(), 19, rules)
            .unwrap();
    let boosted = game.actors.get(game.player).unwrap().maximum_integrity();
    assert!(boosted > maximum);
    assert_eq!(game.actors.get(game.player).unwrap().integrity(), boosted);
    game.refresh_equipment_resources();
    assert_eq!(
        game.actors.get(game.player).unwrap().maximum_integrity(),
        boosted
    );
    assert_eq!(game.turn, 0);
}

#[test]
fn reanimation_keeps_the_original_body_health_share_and_remains_an_active_ally() {
    let mut g = arena();
    g.rules.physical_rules = Some(Default::default());
    learn(&mut g, "OC07");
    let at = GridPos::new(4, 3);
    let victim = g
        .spawn_actor(
            Actor::new(at, 100)
                .unwrap()
                .with_body_profile(crate::stats::BodyProfile::new(100, 0).unwrap())
                .with_player_relation(crate::social::PlayerRelation::Hostile)
                .with_ai(crate::ai::AiProfile::sentry(8, 0))
                .with_attacks([AttackProfile::melee(crate::combat::DamageType::Kinetic, 20)]),
        )
        .unwrap();
    let original_maximum = g.actors.get(victim).unwrap().maximum_integrity();
    assert_eq!(cast(&mut g, "OC07", at), CommandOutcome::Applied);
    assert!(g.agent_reanimation_marked(victim));
    let adversary = enemy(&mut g, GridPos::new(5, 3));
    g.apply_damage_to(
        Some(g.player),
        victim,
        DamagePacket::new(600, crate::combat::DamageType::Corruption, 0),
    )
    .unwrap();
    let raised = g.actors.entity_at(at).unwrap();
    assert_ne!(victim, raised);
    assert!(!g.agent_reanimation_marked(victim));
    assert!(g.agent_reanimated_ally(raised));
    let expected = (u32::from(original_maximum) * 45 / 100).max(1) as u16;
    assert_eq!(g.actors.get(raised).unwrap().maximum_integrity(), expected);
    assert_eq!(g.actors.get(raised).unwrap().integrity(), expected);
    assert!(g.events.iter().any(|event| matches!(event,
        GameEvent::AgentReanimated {original,raised:new,at:position}
        if *original==victim && *new==raised && *position==at)));
    g.drain_events();
    for _ in 0..3 {
        assert_eq!(
            g.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert!(
            g.agent_reanimated_ally(raised),
            "the raised ally must not disappear on the next turn"
        );
    }
    assert!(
        g.actors.get(adversary).unwrap().integrity() < 500,
        "the same creature must actually attack its new adversary"
    );
    assert!(g.events.iter().any(|event| matches!(event,
        GameEvent::DamageApplied{source:Some(source),target,amount,..}
        if *source==raised && *target==adversary && *amount>0)));
    let until = g.agent_runtime().support.raised[&raised].1;
    while g.turn <= until {
        assert_eq!(
            g.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
    }
    assert!(!g.agent_reanimated_ally(raised));
    assert!(g.actors.get(raised).is_none());
}
