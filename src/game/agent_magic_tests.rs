//! Integration checks for real magic damage, autonomous turns and persistence.
use super::super::agent_cast::tests::fixture;
use super::super::*;
use crate::combat::{DamageType, ResistanceProfile};
use crate::game::WorldState;
use crate::social::PlayerRelation;
fn arena(ids: &[&str]) -> GameState {
    let mut g = fixture();
    for id in ids {
        g.player_skills.set_agent_skill_rank(id, 1);
    }
    g.player_energy = crate::resources::EnergyReserve::new(60000, 60000).unwrap();
    g.drain_events();
    g
}
fn enemy(g: &mut GameState, p: GridPos, hp: u16) -> EntityId {
    let e = g
        .spawn_actor(
            Actor::new(p, hp)
                .unwrap()
                .with_player_relation(PlayerRelation::Hostile),
        )
        .unwrap();
    g.drain_events();
    e
}
fn cast(id: &str, p: GridPos) -> GameCommand {
    GameCommand::UseTechniqueAt {
        technique: crate::skills::agent::technique(id),
        target: p,
        weapon_slot: 0,
    }
}
fn hits(g: &GameState, e: EntityId) -> usize {
    g.events
        .iter()
        .filter(|ev| matches!(ev,GameEvent::DamageApplied{target,..}if *target==e))
        .count()
}

#[test]
fn agent_magic_dense_areas_damage_every_recipient_and_apply_guaranteed_statuses() {
    let template = arena(&[]);
    for id in ["A02", "FE02", "FE03", "FE04", "FE05", "FR02", "FR04"] {
        let mut game = GameState::new_with_rules(
            template.map.clone(),
            template.player_position().unwrap(),
            19,
            template.rules.clone(),
        )
        .unwrap();
        // Effective ranks above ten are supported by equipment. At this rank
        // these status chances reach their authored 100% cap, exposing quotas.
        game.player_skills.set_agent_skill_rank(id, 31);
        game.player_energy = crate::resources::EnergyReserve::new(60000, 60000).unwrap();
        let selected = GridPos::new(6, 3);
        let cells: Vec<_> = game
            .agent_spell_preview(id, selected)
            .unwrap()
            .cells()
            .iter()
            .map(|c| c.position)
            .collect();
        let mut victims = Vec::new();
        for at in cells {
            if game.actors.entity_at(at).is_none() {
                victims.push(enemy(&mut game, at, 60000));
            }
        }
        // A projectile can intercept an earlier occupant. Use its actual
        // committed preview, rather than assuming the cursor is its epicenter.
        let preview = game.agent_spell_preview(id, selected).unwrap();
        let expected: Vec<_> = victims
            .into_iter()
            .filter(|v| {
                let at = game.actors.get(*v).unwrap().position();
                preview.cells().iter().any(|c| c.position == at)
            })
            .collect();
        assert!(expected.len() > 3, "{id}: dense area fixture");
        assert_eq!(
            game.process_player_command(cast(id, selected)),
            CommandOutcome::Applied,
            "{id}"
        );
        for _ in 0..3 {
            if expected
                .iter()
                .all(|v| game.actors.get(*v).unwrap().integrity() < 60000)
            {
                break;
            }
            game.process_player_command(GameCommand::Wait);
        }
        for victim in expected {
            let actor = game.actors.get(victim).unwrap();
            assert!(actor.integrity() < 60000, "{id}: no damage on {victim:?}");
            let status = if id.starts_with("FE") {
                Some("core:burning")
            } else if id.starts_with("FR") {
                Some("agent:frozen")
            } else {
                None
            };
            if let Some(status) = status {
                assert!(
                    actor.status(&status.parse().unwrap()).is_some(),
                    "{id}: missing {status} on {victim:?}"
                );
            }
        }
    }
}

#[test]
fn agent_magic_breath_rolls_burn_for_each_victim_without_gating_their_damage() {
    let mut game = arena(&["FE02"]);
    game.player_skills.set_agent_skill_rank("FE02", 10);
    let selected = GridPos::new(8, 3);
    let cells: Vec<_> = game
        .agent_spell_preview("FE02", selected)
        .unwrap()
        .cells()
        .iter()
        .map(|c| c.position)
        .collect();
    let origin = game.player_position().unwrap();
    let victims: Vec<_> = cells
        .into_iter()
        .filter(|at| *at != origin)
        .map(|at| enemy(&mut game, at, 60000))
        .collect();
    game.rng = GameRng::from_seed(14);
    let mut expected_rng = game.rng;
    let chance = crate::skills::agent::definition("FE02")
        .unwrap()
        .value("burn_chance_percent", 10);
    let expected: BTreeSet<_> = victims
        .iter()
        .copied()
        .filter(|_| u16::from(expected_rng.percentile()) <= chance)
        .collect();
    assert!(expected.len() > 3 && expected.len() < victims.len());
    assert_eq!(
        game.process_player_command(cast("FE02", selected)),
        CommandOutcome::Applied
    );
    let actual: BTreeSet<_> = victims
        .iter()
        .copied()
        .filter(|v| {
            game.actors
                .get(*v)
                .unwrap()
                .status(&"core:burning".parse().unwrap())
                .is_some()
        })
        .collect();
    assert_eq!(
        actual, expected,
        "one burn roll per victim, not one shared roll"
    );
    assert!(
        victims
            .iter()
            .all(|v| game.actors.get(*v).unwrap().integrity() < 60000)
    );
}
#[test]
fn agent_magic_invalid_targets_and_previews_are_atomic() {
    for id in [
        "FE02", "FE03", "FE04", "FR01", "FR03", "FR04", "FR05", "HY05", "OC04", "OC05",
    ] {
        let mut g = arena(&[id]);
        enemy(&mut g, GridPos::new(5, 3), 5000);
        let before = format!("{g:?}");
        assert!(
            g.agent_magic_preview(id, GridPos::new(-1, 0)).is_err(),
            "{id}"
        );
        assert_eq!(before, format!("{g:?}"), "{id}");
        let energy = g.player_energy.available();
        g.player_energy.spend(energy).unwrap();
        let before = format!("{g:?}");
        assert!(g.cast_agent_magic(id, GridPos::new(5, 3)).is_err(), "{id}");
        assert_eq!(before, format!("{g:?}"), "{id}");
    }
}
#[test]
fn agent_magic_breath_and_beam_resolve_one_paid_application_per_turn() {
    for id in ["FE02", "OC05"] {
        let mut g = arena(&[id]);
        let target = GridPos::new(4, 3);
        let victim = enemy(&mut g, target, 5000);
        let cost = g.agent_skill_ready(id).unwrap().energy;
        let energy = g.player_energy.available();
        assert_eq!(
            g.process_player_command(cast(id, target)),
            CommandOutcome::Applied,
            "{id}"
        );
        assert_eq!(g.player_energy.available(), energy - cost, "{id}");
        assert_eq!(g.events.iter().filter(|ev|matches!(ev,GameEvent::AgentSpellCast{skill,victim:v,..}if skill==id&&*v==victim)).count(),1);
        g.drain_events();
        assert_eq!(
            g.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert_eq!(g.player_energy.available(), energy - 2 * cost, "{id}");
        assert_eq!(g.events.iter().filter(|ev|matches!(ev,GameEvent::AgentSpellCast{skill,victim:v,..}if skill==id&&*v==victim)).count(),1);
        g.drain_events();
        g.process_player_command(GameCommand::Wait);
        assert_eq!(g.player_energy.available(), energy - 3 * cost);
        assert!(
            !g.agent_runtime()
                .magic
                .effects
                .iter()
                .any(|e| e.skill == id)
        );
        let hp = g.actors.get(victim).unwrap().integrity();
        g.process_player_command(GameCommand::Wait);
        if id == "OC05" {
            assert_eq!(g.actors.get(victim).unwrap().integrity(), hp);
        }
    }
}
#[test]
fn agent_magic_delayed_blast_allows_escape_and_uses_actual_occupants() {
    let mut g = arena(&["FE04"]);
    let centre = GridPos::new(5, 3);
    let escaping = enemy(&mut g, centre, 5000);
    let entering = enemy(&mut g, GridPos::new(8, 3), 5000);
    assert_eq!(
        g.process_player_command(cast("FE04", centre)),
        CommandOutcome::Applied
    );
    assert_eq!(hits(&g, escaping), 0);
    g.actors
        .get_mut(escaping)
        .unwrap()
        .set_position(GridPos::new(8, 2));
    g.actors.get_mut(entering).unwrap().set_position(centre);
    g.drain_events();
    g.process_player_command(GameCommand::Wait);
    assert_eq!(hits(&g, escaping), 0);
    assert!(hits(&g, entering) > 0);
    assert!(g.agent_runtime().magic.effects.is_empty());
}
#[test]
fn agent_magic_gel_resistance_quenching_and_actions_are_separate() {
    let mut g = arena(&["FE01", "FR01"]);
    g.player_skills.set_agent_skill_rank("FR01", 20);
    let at = GridPos::new(5, 3);
    let victim = enemy(&mut g, at, 5000);
    *g.actors.get_mut(victim).unwrap() = g
        .actors
        .get(victim)
        .unwrap()
        .clone()
        .with_resistances(ResistanceProfile::default().with(DamageType::Cold, -50));
    g.process_player_command(cast("FE01", at));
    assert!(
        g.actors
            .get(victim)
            .unwrap()
            .status(&"core:burning".parse().unwrap())
            .is_some()
    );
    g.process_player_command(cast("FR01", at));
    assert!(g.agent_magic_movement_blocked(victim));
    assert!(
        g.actors
            .get(victim)
            .unwrap()
            .status(&"core:burning".parse().unwrap())
            .is_none()
    );
    // Gel never adds recovery or disables attacks.
    assert!(g.actors.get(victim).unwrap().recovery_remaining().is_none());
    g.process_player_command(cast("FE01", at));
    assert!(!g.agent_magic_movement_blocked(victim));
    assert!(
        g.actors
            .get(victim)
            .unwrap()
            .status(&"core:burning".parse().unwrap())
            .is_some()
    );
}
#[test]
fn agent_magic_cold_scaling_keeps_fraction_until_final_defenses() {
    let g = arena(&["FR01", "FR05"]);
    let frost = g.agent_spell_profile("FR01", 1).unwrap();
    let ember = g.agent_spell_profile("FE01", 1).unwrap();
    assert_eq!(frost.base_damage, 8);
    assert_eq!(
        frost.multiplier_numerator * ember.multiplier_denominator * 100,
        ember.multiplier_numerator * frost.multiplier_denominator * 85
    );
    let blizzard = g.agent_spell_profile("FR05", 1).unwrap();
    assert_eq!(blizzard.base_damage, 6);
}
#[test]
fn agent_magic_crystalline_snapshots_all_sources_and_consumes_gel() {
    let mut g = arena(&["FR01", "FR06"]);
    g.player_skills.set_agent_skill_rank("FR01", 20);
    let first = enemy(&mut g, GridPos::new(4, 3), 5000);
    let second = enemy(&mut g, GridPos::new(5, 3), 5000);
    g.apply_agent_freeze("FR01", 20, first, 10);
    g.apply_agent_freeze("FR01", 20, second, 10);
    g.drain_events();
    assert_eq!(
        g.process_player_command(cast("FR06", GridPos::new(2, 3))),
        CommandOutcome::Applied
    );
    assert_eq!(hits(&g, first), 2);
    assert_eq!(hits(&g, second), 2);
    assert!(!g.agent_magic_movement_blocked(first));
    assert!(!g.agent_magic_movement_blocked(second));
    let before = format!("{g:?}");
    assert!(matches!(
        g.process_player_command(cast("FR06", GridPos::new(2, 3))),
        CommandOutcome::Rejected(_)
    ));
    assert_eq!(before, format!("{g:?}"));
}
#[test]
fn agent_magic_spirit_drain_returns_only_actual_removed_hp() {
    let mut g = arena(&["HY05"]);
    let at = GridPos::new(5, 3);
    enemy(&mut g, at, 4);
    g.player_energy.spend(100).unwrap();
    let before = g.player_energy.available();
    let cost = g.agent_skill_ready("HY05").unwrap().energy;
    g.process_player_command(cast("HY05", at));
    assert_eq!(g.player_energy.available(), before - cost + 1);
}
#[test]
fn agent_magic_wave_keeps_blocked_lanes_stopped_after_the_obstacle() {
    let mut g = arena(&["FR04"]);
    g.map
        .set_terrain(GridPos::new(4, 3), Terrain::Wall)
        .unwrap();
    g.refresh_player_visibility();
    let neighbour = enemy(&mut g, GridPos::new(3, 3), 5000);
    let shadow = enemy(&mut g, GridPos::new(5, 3), 5000);
    let side = enemy(&mut g, GridPos::new(5, 2), 5000);
    g.process_player_command(cast("FR04", GridPos::new(3, 3)));
    assert_eq!(hits(&g, neighbour), 1);
    for _ in 0..4 {
        g.process_player_command(GameCommand::Wait);
    }
    assert_eq!(g.actors.get(shadow).unwrap().integrity(), 5000);
    assert!(g.actors.get(side).unwrap().integrity() < 5000);
}
#[test]
fn agent_magic_barriers_are_separate_breakable_actors_and_expire_without_loot() {
    let mut g = arena(&["FR03", "OC04"]);
    let at = GridPos::new(5, 3);
    g.process_player_command(cast("FR03", at));
    let first: Vec<_> = g.agent_runtime().magic.barriers.keys().copied().collect();
    assert_eq!(first.len(), 3);
    let victim = first[0];
    let hp = g.actors.get(victim).unwrap().integrity();
    g.apply_damage_impact_to_scaled_spell(
        Some(g.player),
        victim,
        DamagePacket::new(hp, DamageType::Corruption, 0),
        1,
        1,
    )
    .unwrap();
    assert!(g.actors.get(victim).is_none());
    assert!(g.actors.get(first[1]).is_some());
    g.process_player_command(cast("FR03", GridPos::new(6, 3)));
    assert!(first.iter().all(|e| g.actors.get(*e).is_none()));
    for _ in 0..3 {
        g.process_player_command(GameCommand::Wait);
    }
    assert!(g.agent_runtime().magic.barriers.is_empty());
    assert_eq!(g.player_progression.level(), 1);
}
#[test]
fn agent_magic_runtime_nonempty_roundtrip_continues_same_damage_and_rng() {
    let mut g = arena(&["FR04", "FE04"]);
    enemy(&mut g, GridPos::new(5, 3), 5000);
    g.process_player_command(cast("FR04", GridPos::new(3, 3)));
    g.process_player_command(cast("FE04", GridPos::new(5, 3)));
    g.drain_events();
    let bytes = bincode::serialize(&g.snapshot().unwrap()).unwrap();
    let mut restored =
        GameState::from_snapshot(bincode::deserialize(&bytes).unwrap(), g.rules.clone());
    g.drain_events();
    restored.drain_events();
    for _ in 0..4 {
        g.process_player_command(GameCommand::Wait);
        restored.process_player_command(GameCommand::Wait);
        assert_eq!(g.rng_state(), restored.rng_state());
        assert_eq!(g.agent_runtime(), restored.agent_runtime());
        assert_eq!(
            g.actors
                .iter()
                .map(|(e, a)| (e, a.integrity()))
                .collect::<Vec<_>>(),
            restored
                .actors
                .iter()
                .map(|(e, a)| (e, a.integrity()))
                .collect::<Vec<_>>()
        );
        assert_eq!(g.drain_events(), restored.drain_events());
    }
}

#[test]
fn agent_magic_fire_wall_entry_and_stay_share_one_damage_per_actor_per_turn() {
    let mut g = arena(&["FE03"]);
    let outside = GridPos::new(6, 3);
    let inside = GridPos::new(5, 3);
    let victim = enemy(&mut g, outside, 5000);
    g.cast_agent_magic("FE03", inside).unwrap();
    g.drain_events();
    g.actors.get_mut(victim).unwrap().set_position(inside);
    g.agent_magic_actor_moved(victim);
    let casts = |g: &GameState| {
        g.events.iter().filter(|e|matches!(e,GameEvent::AgentSpellCast{skill,victim:v,..}if skill=="FE03"&&*v==victim)).count()
    };
    assert_eq!(casts(&g), 1);
    g.actors.get_mut(victim).unwrap().set_position(outside);
    g.agent_magic_actor_moved(victim);
    g.actors.get_mut(victim).unwrap().set_position(inside);
    g.agent_magic_actor_moved(victim);
    assert_eq!(casts(&g), 1);
    g.complete_turn();
    assert_eq!(casts(&g), 1);
    g.drain_events();
    g.complete_turn();
    assert_eq!(casts(&g), 1);
}

fn barrier_corridor() -> GameState {
    let fixture = arena(&[]);
    let mut game = GameState::new_with_rules(
        Map::from_ascii("#########\n#.......#\n#########").unwrap(),
        GridPos::new(1, 1),
        19,
        fixture.rules.clone(),
    )
    .unwrap();
    game.player_energy = crate::resources::EnergyReserve::new(60000, 60000).unwrap();
    game
}
#[test]
fn agent_magic_ai_breaches_only_an_adjacent_spell_segment_blocking_its_route() {
    for skill in ["FR03", "OC04"] {
        let mut g = barrier_corridor();
        g.player_skills.set_agent_skill_rank(skill, 1);
        let (center, at) = if skill == "FR03" {
            (GridPos::new(2, 1), GridPos::new(3, 1))
        } else {
            (GridPos::new(4, 1), GridPos::new(4, 1))
        };
        let hunter = g
            .spawn_actor(
                Actor::new(at, 5000)
                    .unwrap()
                    .with_player_relation(PlayerRelation::Hostile)
                    .with_ai(crate::ai::AiProfile::hunter(8, 0))
                    .with_attack(AttackProfile::melee(DamageType::Kinetic, 5)),
            )
            .unwrap();
        g.cast_agent_magic(skill, center).unwrap();
        let barrier = g
            .agent_magic_barrier_at(GridPos::new(at.x - 1, at.y))
            .unwrap();
        g.drain_events();
        g.resolve_ai_turn();
        assert!(g.events.iter().any(|e|matches!(e,GameEvent::AttackPerformed{attacker,target:Some(victim),..}if *attacker==hunter&&*victim==barrier)),"{skill}: {:?}",g.events);
        assert_eq!(g.actors.get(hunter).unwrap().position(), at);
    }
}
#[test]
fn agent_magic_ai_does_not_attack_a_plain_occupant_when_its_route_is_blocked() {
    let mut g = barrier_corridor();
    let at = GridPos::new(3, 1);
    let hunter = g
        .spawn_actor(
            Actor::new(at, 5000)
                .unwrap()
                .with_player_relation(PlayerRelation::Hostile)
                .with_ai(crate::ai::AiProfile::hunter(8, 0))
                .with_attack(AttackProfile::melee(DamageType::Kinetic, 5)),
        )
        .unwrap();
    g.spawn_actor(Actor::new(GridPos::new(2, 1), 5000).unwrap())
        .unwrap();
    g.drain_events();
    g.resolve_ai_turn();
    assert!(
        !g.events
            .iter()
            .any(|e| matches!(e,GameEvent::AttackPerformed{attacker,..}if *attacker==hunter))
    );
    assert_eq!(g.actors.get(hunter).unwrap().position(), at);
}
#[test]
fn agent_magic_walls_expire_on_local_turns_with_no_player_in_the_zone() {
    let mut g = barrier_corridor();
    g.player_skills.set_agent_skill_rank("FR03", 1);
    g.cast_agent_magic("FR03", GridPos::new(2, 1)).unwrap();
    let barriers = g
        .agent_runtime()
        .magic
        .barriers
        .keys()
        .copied()
        .collect::<Vec<_>>();
    assert!(!barriers.is_empty());
    g.actors.remove(g.player);
    for turn in 0..5 {
        g.turn = turn;
        g.process_agent_magic_turn();
    }
    assert!(g.agent_runtime().magic.barriers.is_empty());
    assert!(barriers.iter().all(|e| g.actors.get(*e).is_none()));
}

#[test]
fn agent_magic_blizzard_learned_through_commands_resolves_after_warning_in_real_world_turns() {
    let mut game = fixture();
    let xp = game.rules.progression.curve.cumulative_thresholds()[0];
    game.apply_experience_award(
        &crate::progression::ExperienceAward::repeatable(xp),
        ExperienceSource::DefeatedEntity(EntityId::new(9000)),
    );
    assert_eq!(
        game.process_player_command(GameCommand::LearnAgentSkill {
            skill: "FR05".into()
        }),
        CommandOutcome::AppliedWithoutTime
    );
    assert_eq!(game.player_agent_skill_rank("FR05"), 1);
    let center = GridPos::new(6, 3);
    let area = crate::skills::spatial::affected_cells(
        &game.map,
        center,
        crate::skills::spatial::Area::Disc { radius: 2 },
        (1, 0),
    );
    assert_eq!(area.len(), 13);
    let mut victims = Vec::new();
    for at in area {
        victims.push(
            game.spawn_actor(
                Actor::new(at, 5000)
                    .unwrap()
                    .with_player_relation(PlayerRelation::Hostile)
                    .with_resistances(ResistanceProfile::default().with(DamageType::Cold, -50))
                    .with_ai(crate::ai::AiProfile::sentry(8, 0))
                    .with_attack(AttackProfile::melee(DamageType::Kinetic, 1)),
            )
            .unwrap(),
        );
    }
    game.drain_events();
    let mut world = WorldState::single(game);
    let preview = world.agent_spell_preview("FR05", center).unwrap();
    assert_eq!(preview.cells().len(), 13);
    assert_eq!(
        world.process_player_command(cast("FR05", center)),
        CommandOutcome::Applied
    );
    let cast_events = world.drain_events();
    assert!(cast_events.iter().any(|e|matches!(e,GameEvent::TechniqueUsed{technique,..}if *technique==crate::skills::agent::technique("FR05"))));
    assert!(!cast_events.iter().any(|e| matches!(
        e,
        GameEvent::DamageApplied {
            damage_type: DamageType::Cold,
            ..
        }
    )));
    assert!(
        victims
            .iter()
            .all(|e| world.actors.get(*e).unwrap().integrity() == 5000)
    );
    world.rng = GameRng::from_seed(14);
    let chance = crate::skills::agent::definition("FR05")
        .unwrap()
        .value("impact_chance_per_cell_percent", 1);
    assert_eq!(chance, 50);
    let mut impacts = 0;
    let mut gel = 0;
    let mut guaranteed_gel_impacts = 0;
    for turn in 1..=3 {
        let mut expected_rng = world.rng;
        let expected_impacts = preview
            .cells()
            .iter()
            .filter(|_| u16::from(expected_rng.percentile()) <= chance)
            .count();
        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        let events = world.drain_events();
        let actual=events.iter().filter(|e|matches!(e,GameEvent::DamageApplied{damage_type:DamageType::Cold,amount,..}if *amount>0)).count();
        assert_eq!(
            actual, expected_impacts,
            "each cell is independently rolled and every cell here is occupied: {events:?}"
        );
        impacts += actual;
        if turn == 3 {
            guaranteed_gel_impacts = actual;
        }
        gel+=events.iter().filter(|e|matches!(e,GameEvent::StatusApplied{status,application,..}if status.as_str()=="agent:frozen"&&*application!=crate::status::StatusApplyKind::Ignored)).count();
        assert!(
            events
                .iter()
                .any(|e| matches!(e, GameEvent::EntityWaited { .. })),
            "real sentry AI resolves each round"
        );
    }
    assert!(impacts > 6, "no former two-impact quota remains");
    assert!(guaranteed_gel_impacts > 0);
    assert!(
        gel >= guaranteed_gel_impacts,
        "third continuous exposure guarantees Gel on each struck target"
    );
    assert!(world.agent_runtime().magic.effects.is_empty());
    assert!(
        victims
            .iter()
            .any(|e| world.actors.get(*e).unwrap().integrity() < 5000)
    );
}

#[test]
fn agent_magic_blizzard_inflicts_cold_damage_when_every_freeze_roll_fails() {
    let mut game = arena(&["FR05"]);
    let center = GridPos::new(6, 3);
    let cells = crate::skills::spatial::affected_cells(
        game.map(),
        center,
        crate::skills::spatial::Area::Disc { radius: 2 },
        (1, 0),
    );
    let victims: Vec<_> = cells
        .into_iter()
        .map(|at| enemy(&mut game, at, 5000))
        .collect();
    assert_eq!(
        game.process_player_command(cast("FR05", center)),
        CommandOutcome::Applied
    );
    game.drain_events();
    // Every cell is rolled, including empty ones in ordinary play. This seed
    // selects five of these thirteen occupied cells, then yields freeze rolls
    // 99, 51, 56, 90 and 69: all above the first-exposure chance of 30%.
    game.rng = GameRng::from_seed(14);
    assert_eq!(
        game.process_player_command(GameCommand::Wait),
        CommandOutcome::Applied
    );
    let events = game.drain_events();
    let damage: Vec<_> = events
        .iter()
        .filter_map(|event| match event {
            GameEvent::DamageApplied {
                target,
                amount,
                damage_type: DamageType::Cold,
                ..
            } if *amount > 0 => Some((*target, *amount)),
            _ => None,
        })
        .collect();
    assert_eq!(damage.len(), 5, "{events:?}");
    assert!(
        !events.iter().any(|event| matches!(event,
            GameEvent::StatusApplied { status, .. } if status.as_str() == "agent:frozen"
        )),
        "{events:?}"
    );
    for (victim, amount) in damage {
        assert!(victims.contains(&victim));
        let actor = game.actors().get(victim).unwrap();
        assert_eq!(actor.integrity(), 5000 - amount);
        assert!(actor.status(&"agent:frozen".parse().unwrap()).is_none());
        assert_eq!(game.agent_freeze_chance("FR05", 1, victim, 1), 30);
    }
}

#[test]
fn agent_magic_blizzard_rolls_every_valid_cell_without_a_rank_ten_target_quota() {
    let mut game = arena(&["FR05"]);
    game.player_skills.set_agent_skill_rank("FR05", 10);
    let center = GridPos::new(6, 3);
    let before = format!("{game:?}");
    let preview = game.agent_spell_preview("FR05", center).unwrap();
    assert_eq!(
        format!("{game:?}"),
        before,
        "preview consumes no RNG or state"
    );
    let cells: Vec<_> = preview.cells().iter().map(|cell| cell.position).collect();
    for (i, at) in cells.iter().enumerate() {
        if i % 3 != 0 && game.actors.entity_at(*at).is_none() {
            enemy(&mut game, *at, 5000);
        }
    }
    assert_eq!(
        game.process_player_command(cast("FR05", center)),
        CommandOutcome::Applied
    );
    game.drain_events();
    // Runtime changes also remain excluded from the lottery and its damage.
    let protected = cells
        .iter()
        .copied()
        .find(|at| *at != center && game.actors.entity_at(*at).is_some())
        .unwrap();
    game.map.set_protected(protected, true).unwrap();
    let blocked = cells
        .iter()
        .copied()
        .find(|at| *at != center && game.actors.entity_at(*at).is_none())
        .unwrap();
    game.map.set_terrain(blocked, Terrain::Wall).unwrap();
    game.rng = GameRng::from_seed(14);
    let mut expected_rng = game.rng;
    let expected: BTreeSet<_> = cells
        .iter()
        .copied()
        .filter(|at| {
            game.map.is_walkable(*at)
                && !game.map.is_protected(*at)
                && has_line_of_sight(&game.map, center, *at, true)
        })
        .filter(|_| expected_rng.percentile() <= 50)
        .filter_map(|at| game.actors.entity_at(at))
        .filter(|id| *id != game.player_id())
        .collect();
    assert!(
        expected.len() > 3,
        "more than the former rank-ten quota: {expected:?}"
    );
    assert_eq!(
        game.process_player_command(GameCommand::Wait),
        CommandOutcome::Applied
    );
    let events = game.drain_events();
    let actual: BTreeSet<_> = events
        .iter()
        .filter_map(|event| match event {
            GameEvent::DamageApplied {
                target,
                amount,
                damage_type: DamageType::Cold,
                ..
            } if *amount > 0 => Some(*target),
            _ => None,
        })
        .collect();
    assert_eq!(
        actual, expected,
        "one independent roll per cell, occupied or empty; no target quota"
    );
    assert!(!actual.contains(&game.actors.entity_at(protected).unwrap()));
}
