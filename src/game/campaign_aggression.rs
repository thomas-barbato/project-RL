//! Aggressive combat policy for the accepted city and sewer populations.
//! Existing population tags also identify actors in already saved campaigns;
//! no actor replacement, save migration, or combat-stat change is needed.
use super::*;

impl GameState {
    pub(super) fn campaign_monster_is_aggressive(&self, entity: EntityId) -> bool {
        self.actors.get(entity).is_some_and(|actor| {
            actor.player_relation() == crate::social::PlayerRelation::Hostile
                && !self.agent_support_owned(entity)
                && actor.tags().iter().any(|tag| {
                    matches!(
                        tag.as_str(),
                        "core:street_population_01" | "core:sewer_population_04"
                    )
                })
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::AiProfile;
    use crate::social::PlayerRelation;
    use crate::stealth::StealthRules;
    use std::num::NonZeroU16;

    fn fixture(tag: &str, at: GridPos) -> (GameState, EntityId) {
        let rules = GameRules {
            player_maximum_integrity: 100,
            player_starting_attributes: PrimaryAttributes::new(5, 8, 5, 5, 5),
            stealth_rules: Some(StealthRules::default()),
            ..GameRules::default()
        };
        let mut game = GameState::new_with_rules(
            Map::filled(20, 13, Terrain::Floor).unwrap(),
            GridPos::new(3, 6),
            42,
            rules,
        )
        .unwrap();
        let short = NonZeroU16::new(2).unwrap();
        let enemy = game
            .spawn_actor(
                Actor::new(at, 10)
                    .unwrap()
                    .with_attack(AttackProfile::melee(crate::combat::DamageType::Kinetic, 1))
                    .with_ai(
                        AiProfile::hunter(9, 0)
                            .with_maximum_pursuit_distance(short)
                            .with_pursuit_lifecycle(PursuitLifecycle::new(short, short, short)),
                    )
                    .with_player_relation(PlayerRelation::Hostile)
                    .with_tags([tag.parse().unwrap()]),
            )
            .unwrap();
        game.drain_events();
        (game, enemy)
    }

    #[test]
    fn campaign_aggression_only_affects_hostile_campaign_monsters() {
        let (mut game, enemy) = fixture("core:street_population_01", GridPos::new(9, 6));
        assert!(game.campaign_monster_is_aggressive(enemy));
        assert!(game.actor_optically_detects_player(enemy));
        // A neighboring wall is partial cover, not an invisibility ability.
        game.map
            .set_terrain(GridPos::new(3, 5), Terrain::Wall)
            .unwrap();
        assert!(game.actor_optically_detects_player(enemy));
        for relation in [PlayerRelation::Allied, PlayerRelation::Neutral] {
            let actor = game
                .actors
                .get(enemy)
                .unwrap()
                .clone()
                .with_player_relation(relation);
            *game.actors.get_mut(enemy).unwrap() = actor;
            assert!(!game.campaign_monster_is_aggressive(enemy));
        }
        let (legacy, enemy) = fixture("core:legacy_enemy", GridPos::new(9, 6));
        assert!(!legacy.campaign_monster_is_aggressive(enemy));
        assert!(!legacy.actor_optically_detects_player(enemy));
    }

    #[test]
    fn campaign_aggression_respects_walls_range_protection_and_active_camouflage() {
        let (mut game, enemy) = fixture("core:sewer_population_04", GridPos::new(9, 6));
        for y in 0..13 {
            game.map
                .set_terrain(GridPos::new(6, y), Terrain::Wall)
                .unwrap();
        }
        assert!(!game.actor_optically_detects_player(enemy));
        game.process_player_command(GameCommand::Wait);
        assert!(
            !game
                .events
                .iter()
                .any(|event| matches!(event, GameEvent::AttackPerformed { .. }))
        );
        assert_eq!(
            game.actors.get(enemy).unwrap().position(),
            GridPos::new(9, 6)
        );

        let (mut game, enemy) = fixture("core:sewer_population_04", GridPos::new(4, 6));
        game.map.set_protected(GridPos::new(3, 6), true).unwrap();
        assert!(!game.actor_optically_detects_player(enemy));
        game.process_player_command(GameCommand::Wait);
        assert!(
            !game
                .events
                .iter()
                .any(|event| matches!(event, GameEvent::AttackPerformed { .. }))
        );

        let (mut game, enemy) = fixture("core:sewer_population_04", GridPos::new(9, 6));
        game.player_active_camouflage = Some(ActiveCamouflage {
            technique: "core:test_camouflage".parse().unwrap(),
            channel: SignatureChannel::Optical,
            optical_difficulty_bonus: 200,
            remaining_phases: 20,
            upkeep_energy: 0,
            heat_per_phase: 0,
        });
        assert!(!game.actor_optically_detects_player(enemy));
        let (game, enemy) = fixture("core:sewer_population_04", GridPos::new(17, 6));
        assert!(!game.actor_optically_detects_player(enemy));
    }

    #[test]
    fn campaign_aggression_chases_visible_player_beyond_old_home_leash() {
        let (mut game, enemy) = fixture("core:street_population_01", GridPos::new(6, 6));
        for _ in 0..3 {
            game.process_player_command(GameCommand::Wait);
        }
        assert_eq!(
            game.actors.get(enemy).unwrap().position(),
            GridPos::new(4, 6)
        );
        assert!(game.events.iter().any(|event| matches!(event,
            GameEvent::AttackPerformed { attacker, .. } if *attacker == enemy)));
    }

    #[test]
    fn campaign_aggression_reengages_saved_returning_and_cooling_down_enemies() {
        for tag in ["core:street_population_01", "core:sewer_population_04"] {
            for state in [AiState::Returning, AiState::Cooldown { remaining_turns: 6 }] {
                let (mut game, enemy) = fixture(tag, GridPos::new(4, 6));
                game.actors.get_mut(enemy).unwrap().set_ai_state(state);
                let world = crate::game::WorldState::single(game);
                let bytes = world.recovery_snapshot_bytes().unwrap();
                let mut restored = crate::game::WorldState::from_recovery_snapshot_bytes(
                    &bytes,
                    world.rules().clone(),
                )
                .unwrap();
                assert_eq!(restored.recovery_snapshot_bytes().unwrap(), bytes);
                restored.process_player_command(GameCommand::Wait);
                assert!(
                    restored.drain_events().iter().any(|event| matches!(event,
                    GameEvent::AttackPerformed { attacker, .. } if *attacker == enemy)),
                    "{tag}: {state:?}"
                );
                assert!(matches!(
                    restored.actors().get(enemy).unwrap().ai_state(),
                    AiState::Pursuing { .. }
                ));
            }
        }
    }
}
