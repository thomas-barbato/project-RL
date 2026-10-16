use super::agent_cast::tests::fixture;
use super::*;
use crate::combat::{DamageType, ResistanceProfile};

fn arena(rank: u8) -> GameState {
    let mut g = fixture();
    g.player_skills.set_agent_skill_rank("FE01", rank);
    g.player_energy = crate::resources::EnergyReserve::new(60000, 60000).unwrap();
    g
}
fn enemy(g: &mut GameState, at: GridPos, hp: u16) -> EntityId {
    let id = g
        .spawn_actor(
            Actor::new(at, hp)
                .unwrap()
                .with_player_relation(crate::social::PlayerRelation::Hostile),
        )
        .unwrap();
    g.drain_events();
    id
}
fn cast(at: GridPos) -> GameCommand {
    GameCommand::UseTechniqueAt {
        technique: crate::skills::agent::technique("FE01"),
        target: at,
        weapon_slot: 0,
    }
}
fn burn() -> StatusId {
    "core:burning".parse().unwrap()
}

#[test]
fn ember_is_a_single_fire_hit_with_immediate_burn_and_three_total_applications() {
    let mut g = arena(1);
    let at = GridPos::new(5, 3);
    let target = enemy(&mut g, at, 500);
    let other = enemy(&mut g, GridPos::new(5, 4), 500);
    let profile = g.agent_skill_ready("FE01").unwrap();
    let power = g.agent_spell_burn_profile("FE01", 1).unwrap().0;
    let tick = (f64::from(power.base_damage) * power.numerator as f64 / power.denominator as f64)
        .floor() as u16;
    let energy = g.player_energy.available();
    let hp = g.actors.get(g.player).unwrap().integrity();
    let before = format!("{g:?}");
    assert_eq!(g.agent_spell_preview("FE01", at).unwrap().cells().len(), 1);
    assert_eq!(format!("{g:?}"), before);
    assert_eq!(g.process_player_command(cast(at)), CommandOutcome::Applied);
    let critical = g
        .events
        .iter()
        .find_map(|e| match e {
            GameEvent::AgentSpellCast {
                skill, critical, ..
            } if skill == "FE01" => Some(*critical),
            _ => None,
        })
        .unwrap();
    let direct = (f64::from(profile.base_damage) * profile.multiplier_numerator as f64
        / profile.multiplier_denominator as f64
        * if critical { 2. } else { 1. })
    .floor() as u16;
    assert_eq!(
        g.actors.get(target).unwrap().integrity(),
        500 - direct - tick
    );
    assert_eq!(
        g.actors
            .get(target)
            .unwrap()
            .status(&burn())
            .unwrap()
            .remaining_turns,
        Some(2)
    );
    assert_eq!(g.actors.get(other).unwrap().integrity(), 500);
    assert_eq!(g.actors.get(g.player).unwrap().integrity(), hp);
    assert_eq!(g.player_energy.available(), energy - profile.energy);
    assert_eq!(g.turn, 1);
    for n in [2, 3] {
        assert_eq!(
            g.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(
            g.actors.get(target).unwrap().integrity(),
            500 - direct - tick * n
        );
    }
    assert!(g.actors.get(target).unwrap().status(&burn()).is_none());
    g.complete_turn();
    assert_eq!(
        g.actors.get(target).unwrap().integrity(),
        500 - direct - tick * 3
    );
    assert_eq!(g.events.iter().filter(|e| matches!(e, GameEvent::StatusTriggered { target: id, status, .. } if *id == target && status == &burn())).count(), 3);
}

#[test]
fn ember_target_and_energy_refusals_are_atomic() {
    let mut g = arena(1);
    let target = enemy(&mut g, GridPos::new(5, 3), 500);
    let neutral = g
        .spawn_actor(Actor::new(GridPos::new(4, 4), 500).unwrap())
        .unwrap();
    g.drain_events();
    for at in [
        GridPos::new(-1, 0),
        GridPos::new(9, 3),
        GridPos::new(4, 3),
        g.player_position().unwrap(),
        g.actors.get(neutral).unwrap().position(),
    ] {
        let before = format!("{g:?}");
        assert!(matches!(
            g.process_player_command(cast(at)),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(format!("{g:?}"), before);
    }
    g.map
        .set_terrain(GridPos::new(4, 3), Terrain::Wall)
        .unwrap();
    g.refresh_player_visibility();
    let before = format!("{g:?}");
    assert!(
        g.agent_spell_preview("FE01", g.actors.get(target).unwrap().position())
            .is_err()
    );
    assert_eq!(format!("{g:?}"), before);
    g.map
        .set_terrain(GridPos::new(4, 3), Terrain::Floor)
        .unwrap();
    g.refresh_player_visibility();
    let energy = g.player_energy.available();
    g.player_energy.spend(energy).unwrap();
    let before = format!("{g:?}");
    assert!(matches!(
        g.process_player_command(cast(GridPos::new(5, 3))),
        CommandOutcome::Rejected(CommandRejection::InsufficientEnergy { .. })
    ));
    assert_eq!(format!("{g:?}"), before);
}

#[test]
fn ember_variable_duration_and_cast_power_survive_save_and_live_defense_changes() {
    let mut g = arena(11);
    let at = GridPos::new(5, 3);
    let target = enemy(&mut g, at, 60000);
    let (power, chance, duration) = g.agent_spell_burn_profile("FE01", 11).unwrap();
    assert_eq!(chance, 100);
    assert_eq!(duration, 5);
    assert_eq!(g.process_player_command(cast(at)), CommandOutcome::Applied);
    assert_eq!(
        g.actors
            .get(target)
            .unwrap()
            .status(&burn())
            .unwrap()
            .remaining_turns,
        Some(4)
    );
    g.player_skills.set_agent_skill_rank("FE01", 1);
    g.drain_events();
    let rules = g.rules.clone();
    let world = crate::game::WorldState::single(g);
    let bytes = world.recovery_snapshot_bytes().unwrap();
    let mut restored =
        crate::game::WorldState::from_recovery_snapshot_bytes(&bytes, rules).unwrap();
    let a = restored.actors.get_mut(target).unwrap();
    *a = a
        .clone()
        .with_resistances(ResistanceProfile::default().with(DamageType::Thermal, 50));
    assert_eq!(
        restored
            .actors
            .get(target)
            .unwrap()
            .status(&burn())
            .unwrap()
            .power,
        Some(power)
    );
    let hp = restored.actors.get(target).unwrap().integrity();
    let tick = (f64::from(power.base_damage) * power.numerator as f64 / power.denominator as f64
        * 0.5)
        .floor() as u16;
    for n in 1..=4 {
        restored.complete_turn();
        assert_eq!(
            restored.actors.get(target).unwrap().integrity(),
            hp - tick * n
        );
    }
    assert!(
        restored
            .actors
            .get(target)
            .unwrap()
            .status(&burn())
            .is_none()
    );
    restored.complete_turn();
    assert_eq!(
        restored.actors.get(target).unwrap().integrity(),
        hp - tick * 4
    );
}

#[test]
fn ember_and_fireball_share_one_latest_power_burn_with_immediate_reapplication() {
    let mut g = arena(11);
    let at = GridPos::new(5, 3);
    let target = enemy(&mut g, at, 60000);
    assert_eq!(g.process_player_command(cast(at)), CommandOutcome::Applied);
    let power = g.agent_spell_burn_profile("FE05", 1).unwrap().0;
    let hp = g.actors.get(target).unwrap().integrity();
    g.apply_agent_burn(target, power, 3);
    let tick = (f64::from(power.base_damage) * power.numerator as f64 / power.denominator as f64)
        .floor() as u16;
    assert_eq!(g.actors.get(target).unwrap().integrity(), hp - tick);
    let status = g.actors.get(target).unwrap().status(&burn()).unwrap();
    assert_eq!(status.power, Some(power));
    assert_eq!(status.remaining_turns, Some(3));
    assert_eq!(status.stacks, 1);
    g.complete_turn();
    assert_eq!(g.actors.get(target).unwrap().integrity(), hp - tick);
    assert_eq!(
        g.actors
            .get(target)
            .unwrap()
            .status(&burn())
            .unwrap()
            .remaining_turns,
        Some(2)
    );
}

#[test]
fn ember_lethal_direct_hit_does_not_attach_a_status_to_a_dead_target() {
    let mut g = arena(1);
    let at = GridPos::new(5, 3);
    let target = enemy(&mut g, at, 1);
    assert_eq!(g.process_player_command(cast(at)), CommandOutcome::Applied);
    assert!(g.actors.get(target).is_none());
    assert!(
        !g.events
            .iter()
            .any(|e| matches!(e, GameEvent::StatusApplied { target: id, .. } if *id == target))
    );
}
