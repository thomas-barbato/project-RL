use super::super::agent_cast::tests::fixture;
use super::*;
fn arena(form: &str) -> GameState {
    let mut g = fixture();
    g.player_skills.set_agent_skill_rank(form, 1);
    g.player_energy = crate::resources::EnergyReserve::new(60000, 60000).unwrap();
    g.drain_events();
    g
}
fn target(g: &mut GameState, at: GridPos, hp: u16) -> EntityId {
    g.spawn_actor(
        Actor::new(at, hp)
            .unwrap()
            .with_evasion_disabled()
            .with_player_relation(crate::social::PlayerRelation::Hostile),
    )
    .unwrap()
}
fn technique(id: &str) -> TechniqueId {
    format!("agent:{id}").parse().unwrap()
}

#[test]
fn agent_transform_parameter_reader_matches_execution_with_class_and_passive_bonuses() {
    for rank in [1, 4, 10] {
        let mut g = arena("M01");
        g.player_skills.set_agent_skill_rank("M01", rank);
        g.player_skills.set_agent_skill_rank("M03", 3);
        let start = g.rules.agent_class_start.as_mut().unwrap();
        start.transformation_duration_bonus = 2;
        start.transformed_damage_percent = 140;
        let before = format!("{g:?}");
        let duration = g
            .agent_transformation_parameter_value("M01", "duration_rank_turns", rank)
            .unwrap();
        let claw = g
            .agent_transformation_parameter_value("M01", "raw_claw_damage", rank)
            .unwrap();
        let laceration = g
            .agent_transformation_parameter_value("M01", "raw_laceration_damage_per_target", rank)
            .unwrap();
        assert_eq!(format!("{g:?}"), before);
        let origin = g.player_position().unwrap();
        g.cast_agent_transformation("M01", origin, 0).unwrap();
        assert_eq!(g.player_agent_form_remaining_turns(), duration);
        assert_eq!(
            g.agent_form_action_profile(&technique("m01_claw"))
                .unwrap()
                .damage_before_defenses(),
            claw
        );
        assert_eq!(
            g.agent_form_action_profile(&technique("m01_laceration"))
                .unwrap()
                .damage_before_defenses(),
            laceration
        );
    }
}

#[test]
fn agent_transform_forms_offer_distinct_actions_and_reject_other_skills() {
    for form in ["M01", "TR61", "TR63", "TR64", "TR65"] {
        let mut g = arena(form);
        let origin = g.player_position().unwrap();
        g.cast_agent_transformation(form, origin, 0).unwrap();
        assert_eq!(g.player_agent_form(), Some(form));
        assert_eq!(g.player_agent_form_actions().len(), 3);
        assert!(g.agent_transform_blocks_skill("FE01"));
        assert!(!g.agent_transform_blocks_skill("M02"));
        assert!(!g.agent_transform_blocks_skill(form));
        assert!(g.agent_skill_ready("A01").is_err());
        for info in g.player_agent_form_actions() {
            assert!(g.agent_form_action_profile(&info.technique).is_some());
        }
    }
}
#[test]
fn agent_transform_colossus_preserves_hp_fraction_and_does_not_heal_on_renewal() {
    let mut g = arena("TR61");
    let origin = g.player_position().unwrap();
    let maximum = g.actors.get(g.player).unwrap().maximum_integrity();
    g.actors
        .get_mut(g.player)
        .unwrap()
        .apply_damage(maximum * 2 / 3);
    let before = g.actors.get(g.player).unwrap().integrity();
    g.cast_agent_transformation("TR61", origin, 0).unwrap();
    let newmax = g.actors.get(g.player).unwrap().maximum_integrity();
    assert!(newmax > maximum);
    assert_eq!(
        g.actors.get(g.player).unwrap().integrity(),
        (u32::from(before) * u32::from(newmax) / u32::from(maximum)) as u16
    );
    let hp = g.actors.get(g.player).unwrap().integrity();
    g.cast_agent_transformation("TR61", origin, 0).unwrap();
    assert_eq!(g.actors.get(g.player).unwrap().integrity(), hp);
    g.end_agent_transformation();
    assert_eq!(g.actors.get(g.player).unwrap().maximum_integrity(), maximum);
    assert!(g.actors.get(g.player).unwrap().integrity() <= before);
}
#[test]
fn agent_transform_predatory_damage_is_natural_and_does_not_consume_weapon_supply() {
    let mut g = arena("M01");
    let at = GridPos::new(3, 3);
    let victim = target(&mut g, at, 5000);
    let origin = g.player_position().unwrap();
    let equipment = g.player_equipment.clone();
    g.cast_agent_transformation("M01", origin, 0).unwrap();
    let energy = g.player_energy.available();
    g.cast_agent_form_action(&technique("m01_claw"), at)
        .unwrap();
    assert!(g.actors.get(victim).unwrap().integrity() < 5000);
    assert_eq!(energy, g.player_energy.available());
    assert_eq!(equipment, g.player_equipment);
}
#[test]
fn agent_transform_movements_preflight_free_path_and_shadow_destination() {
    let mut g = arena("M01");
    let origin = g.player_position().unwrap();
    g.cast_agent_transformation("M01", origin, 0).unwrap();
    target(&mut g, GridPos::new(3, 3), 5000);
    let before = format!("{g:?}");
    assert!(
        g.cast_agent_form_action(&technique("m01_leap"), GridPos::new(4, 3))
            .is_err()
    );
    assert_eq!(format!("{g:?}"), before);
    g.end_agent_transformation();
    g.player_skills.set_agent_skill_rank("TR65", 1);
    g.cast_agent_transformation("TR65", origin, 0).unwrap();
    let before = format!("{g:?}");
    assert!(
        g.cast_agent_form_action(&technique("tr65_shadow_step"), GridPos::new(4, 3))
            .is_err()
    );
    assert_eq!(format!("{g:?}"), before);
    g.cast_agent_form_action(&technique("tr65_shadow_step"), GridPos::new(4, 1))
        .unwrap();
    assert_eq!(g.player_position(), Some(GridPos::new(4, 1)));
}
#[test]
fn agent_transform_mutagenic_extension_is_capped_and_not_reset_by_renewal() {
    let mut g = arena("M01");
    let origin = g.player_position().unwrap();
    g.cast_agent_transformation("M01", origin, 0).unwrap();
    let expiry = g.agent_runtime().transform.active.as_ref().unwrap().expires;
    g.extend_agent_transformation(1, 0);
    assert_eq!(
        g.agent_runtime().transform.active.as_ref().unwrap().expires,
        expiry
    );
    for _ in 0..10 {
        g.extend_agent_transformation(1, 10);
    }
    assert_eq!(
        g.agent_runtime()
            .transform
            .active
            .as_ref()
            .unwrap()
            .extension_used,
        2
    );
    assert_eq!(
        g.agent_runtime().transform.active.as_ref().unwrap().expires,
        expiry + 2
    );
    g.cast_agent_transformation("M01", origin, 0).unwrap();
    let renewed = g.agent_runtime().transform.active.as_ref().unwrap().expires;
    g.extend_agent_transformation(1, 10);
    assert_eq!(
        g.agent_runtime().transform.active.as_ref().unwrap().expires,
        renewed
    );
}
#[test]
fn agent_transform_cuirasse_reserve_and_closure_only_apply_to_direct_hostile_hp_loss() {
    let mut g = arena("TR63");
    let origin = g.player_position().unwrap();
    let victim = target(&mut g, GridPos::new(3, 3), 5000);
    g.cast_agent_transformation("TR63", origin, 0).unwrap();
    assert_eq!(g.agent_transform_reduce_direct(g.player, 100, false), 100);
    assert_eq!(g.agent_transform_reduce_direct(g.player, 100, true), 90);
    g.agent_transform_damage_received(Some(victim), g.player, 100, false);
    assert_eq!(
        g.agent_runtime().transform.active.as_ref().unwrap().reserve,
        0
    );
    g.agent_transform_damage_received(Some(victim), g.player, 500, true);
    let cap = g.actors.get(g.player).unwrap().maximum_integrity() / 10;
    assert_eq!(
        g.agent_runtime().transform.active.as_ref().unwrap().reserve,
        cap
    );
    g.cast_agent_form_action(&technique("tr63_closure"), origin)
        .unwrap();
    assert_eq!(g.agent_transform_reduce_direct(g.player, 100, true), 63);
    g.cast_agent_transformation("TR63", origin, 0).unwrap();
    assert_eq!(
        g.agent_runtime().transform.active.as_ref().unwrap().reserve,
        cap
    );
    g.cast_agent_form_action(&technique("tr63_discharge"), origin)
        .unwrap();
    assert_eq!(
        g.agent_runtime().transform.active.as_ref().unwrap().reserve,
        0
    );
}
#[test]
fn agent_transform_nonempty_runtime_roundtrip_preserves_form_and_intrinsic_state() {
    let mut g = arena("TR63");
    let origin = g.player_position().unwrap();
    g.cast_agent_transformation("TR63", origin, 0).unwrap();
    g.cast_agent_form_action(&technique("tr63_closure"), origin)
        .unwrap();
    g.drain_events();
    let bytes = bincode::serialize(&g.snapshot().unwrap()).unwrap();
    let restored = GameState::from_snapshot(bincode::deserialize(&bytes).unwrap(), g.rules.clone());
    assert_eq!(
        g.agent_runtime().transform,
        restored.agent_runtime().transform
    );
    assert_eq!(
        g.player_agent_form_actions()
            .iter()
            .map(|a| a.technique.clone())
            .collect::<Vec<_>>(),
        restored
            .player_agent_form_actions()
            .iter()
            .map(|a| a.technique.clone())
            .collect::<Vec<_>>()
    );
}

#[test]
fn agent_transform_hp_ratio_never_revives_a_dead_actor() {
    let mut g = arena("TR61");
    let max = g.actors.get(g.player).unwrap().maximum_integrity();
    g.actors.get_mut(g.player).unwrap().apply_damage(max);
    g.agent_form_restore_hp_ratio((0, max));
    assert_eq!(g.actors.get(g.player).unwrap().integrity(), 0);
}
