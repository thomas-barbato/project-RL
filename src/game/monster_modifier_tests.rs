use super::*;
use crate::combat::{DamageComponent, DamageType};
use crate::monster_modifiers::PHYSICAL_RESISTANCE_TAG;
use crate::stats::BodyProfile;

#[test]
fn mm21_real_damage_snapshot_and_death_keep_the_native_identity() {
    let rules = GameRules {
        armor_rules: Some(crate::combat::ArmorRules::default()),
        damage: crate::combat::DamageRules::specialized(),
        ..Default::default()
    };
    let mut game = GameState::new_with_rules(
        Map::filled(8, 6, Terrain::Floor).unwrap(),
        GridPos::new(2, 2),
        15,
        rules,
    )
    .unwrap();
    let at = GridPos::new(3, 2);
    let target = game
        .spawn_actor(
            Actor::new(at, 100)
                .unwrap()
                .with_body_profile(BodyProfile::new(100, 0).unwrap().with_base_armor(20))
                .with_resistances(
                    crate::combat::ResistanceProfile::default().with(DamageType::Thermal, 50),
                )
                .with_tags([
                    PHYSICAL_RESISTANCE_TAG.parse().unwrap(),
                    "core:monster_art_124_1".parse().unwrap(),
                ]),
        )
        .unwrap();
    game.drain_events();
    let rng = game.rng;
    let impact = DamageImpact::mixed(
        [
            DamageComponent::new(100, DamageType::Kinetic),
            DamageComponent::new(20, DamageType::Thermal),
        ],
        0,
        [],
    )
    .unwrap();
    assert_eq!(
        game.apply_damage_impact_to(None, target, impact)
            .unwrap()
            .amount,
        66
    );
    assert_eq!(game.actors.get(target).unwrap().integrity(), 34);
    assert_eq!(game.rng, rng);
    game.drain_events();
    let bytes = bincode::serialize(&game.snapshot().unwrap()).unwrap();
    let mut restored =
        GameState::from_snapshot(bincode::deserialize(&bytes).unwrap(), game.rules.clone());
    assert_eq!(
        crate::monster_art::appearance(restored.actors.get(target).unwrap()),
        Some((124, 1))
    );
    assert_eq!(
        restored
            .actor_armor_profile(target)
            .unwrap()
            .physical_resistance_percent(),
        30
    );
    restored
        .apply_damage_impact_to(None, target, impact)
        .unwrap();
    assert!(restored.actors.get(target).is_none());
    assert!(restored.events.iter().any(|event| matches!(event,
        GameEvent::MonsterBodyFell { at: position, number: 124, variant: 1 } if *position == at)));
}
