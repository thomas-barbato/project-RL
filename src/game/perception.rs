//! Read-only sound presentation. This projection has no recovery schema.
use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SoundObservationSource {
    pub position: GridPos,
    pub intensity: u16,
    pub ongoing: bool,
}

impl GameState {
    /// Actual visible emissions, including the most recent turn's brief sounds.
    /// A hidden source is never disclosed by this optional tactical display.
    pub fn sound_observation_sources(&self) -> Vec<SoundObservationSource> {
        let mut sources = BTreeMap::<GridPos, SoundObservationSource>::new();
        for source in self
            .presentation_recent_noises
            .iter()
            .chain(&self.transient_noises)
            .map(|noise| SoundObservationSource {
                position: noise.at,
                intensity: noise.intensity,
                ongoing: false,
            })
            .chain(
                self.sound_emitters
                    .iter()
                    .map(|emitter| SoundObservationSource {
                        position: emitter.position(),
                        intensity: emitter.intensity(),
                        ongoing: true,
                    }),
            )
        {
            if source.intensity == 0 || !self.player_visibility.is_visible(source.position) {
                continue;
            }
            let entry = sources.entry(source.position).or_insert(source);
            if source.intensity > entry.intensity {
                *entry = source;
            } else if source.intensity == entry.intensity {
                entry.ongoing |= source.ongoing;
            }
        }
        sources.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::{AiProfile, PursuitLifecycle};
    use crate::social::PlayerRelation;
    use crate::stealth::StealthRules;
    use std::num::NonZeroU16;

    #[test]
    fn circular_player_vision_sees_actors_behind_walls_without_changing_npc_sight_or_trajectories()
    {
        let rules = GameRules {
            player_field_of_view: crate::world::FieldOfViewRules {
                radius: 6,
                ..Default::default()
            },
            ordinary_actor_field_of_view: Some(crate::world::FieldOfViewRules::default()),
            ..Default::default()
        };
        let mut map = Map::from_ascii(&vec!["................"; 13].join("\n")).unwrap();
        for y in 0..13 {
            map.set_terrain(GridPos::new(7, y), crate::world::Terrain::Wall)
                .unwrap();
        }
        let origin = GridPos::new(5, 6);
        let behind = GridPos::new(9, 6);
        let mut game = GameState::new_with_rules(map, origin, 42, rules).unwrap();
        let npc = game
            .spawn_actor(Actor::new(behind, 10).unwrap().with_ai(AiProfile::idle()))
            .unwrap();
        assert!(!game.player_visibility().is_visible(behind));
        let turn = game.turn();
        assert_eq!(
            game.process_player_command(GameCommand::EnableCircularPlayerVision),
            CommandOutcome::AppliedWithoutTime
        );
        assert_eq!(game.turn(), turn);
        assert!(game.player_visibility().is_visible(behind));
        assert!(game.player_visibility().is_visible(GridPos::new(11, 6)));
        assert!(!game.player_visibility().is_visible(GridPos::new(12, 6)));
        assert!(game.player_visibility().is_visible(GridPos::new(9, 10)));
        assert!(!game.player_visibility().is_visible(GridPos::new(10, 10)));
        assert!(!crate::world::has_line_of_sight(
            game.map(),
            origin,
            behind,
            true
        ));
        let fields = game.actor_observation_fields();
        let field = fields.iter().find(|field| field.observer() == npc).unwrap();
        assert_eq!(field.radius(), 8);
        assert!(!field.contains(origin));
    }

    #[test]
    fn six_cell_vision_adoption_hides_distant_actors_preserves_memory_and_spends_nothing() {
        let rules = GameRules {
            player_field_of_view: crate::world::FieldOfViewRules {
                radius: 12,
                ..Default::default()
            },
            ordinary_actor_field_of_view: Some(crate::world::FieldOfViewRules::default()),
            ..Default::default()
        };
        let map = Map::from_ascii(&vec!["..............................."; 25].join("\n")).unwrap();
        let mut game = GameState::new_with_rules(map, GridPos::new(12, 12), 42, rules).unwrap();
        let edge = GridPos::new(18, 12);
        let distant = GridPos::new(19, 12);
        for at in [edge, distant] {
            game.spawn_actor(Actor::new(at, 10).unwrap().with_ai(AiProfile::idle()))
                .unwrap();
        }
        assert!(game.player_visibility.is_visible(distant));
        let before = (
            game.turn,
            game.phase,
            format!("{:?}", game.rng),
            format!("{:?}", game.actors),
            format!("{:?}", game.events),
        );
        assert!(!GameCommand::UpgradePlayerVision.consumes_time_on_success());
        assert_eq!(
            game.process_player_command(GameCommand::UpgradePlayerVision),
            CommandOutcome::AppliedWithoutTime
        );
        assert!(game.player_visibility.is_visible(edge));
        assert!(!game.player_visibility.is_visible(distant));
        assert!(game.player_visibility.is_explored(distant));
        assert_eq!(game.rules.ordinary_actor_field_of_view.unwrap().radius, 8);
        assert_eq!(
            before,
            (
                game.turn,
                game.phase,
                format!("{:?}", game.rng),
                format!("{:?}", game.actors),
                format!("{:?}", game.events)
            )
        );
    }

    #[test]
    fn brief_sound_presentation_tracks_real_hearing_then_clears_without_changing_saves() {
        let mut rules = GameRules::default();
        rules.stealth_rules = Some(StealthRules {
            base_detection: 0,
            circular_sound_fields: true,
            ..StealthRules::default()
        });
        let mut game = GameState::new_with_rules(
            Map::from_ascii("########\n#......#\n#......#\n#......#\n########").unwrap(),
            GridPos::new(2, 2),
            17,
            rules,
        )
        .unwrap();
        let enemy = game
            .spawn_actor(
                Actor::new(GridPos::new(4, 2), 10)
                    .unwrap()
                    .with_player_relation(PlayerRelation::Hostile)
                    .with_ai(AiProfile::hunter(4, 128).with_pursuit_lifecycle(
                        PursuitLifecycle::new(
                            NonZeroU16::new(8).unwrap(),
                            NonZeroU16::new(2).unwrap(),
                            NonZeroU16::new(2).unwrap(),
                        ),
                    )),
            )
            .unwrap();
        let mut game = crate::game::WorldState::single(game);
        assert_eq!(
            game.process_player_command(GameCommand::Move(Direction::East)),
            CommandOutcome::Applied
        );
        assert!(game.transient_noises.is_empty());
        assert_eq!(game.actor_perception_signals(enemy), Some((false, true)));
        assert_eq!(
            game.sound_observation_sources(),
            vec![SoundObservationSource {
                position: GridPos::new(3, 2),
                intensity: 10,
                ongoing: false,
            }]
        );
        let before_sources = game.sound_observation_sources();
        let before_signals = game.actor_perception_signals(enemy);
        assert!(matches!(
            game.process_player_command(GameCommand::Attack {
                slot: 7,
                target: enemy
            }),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(game.sound_observation_sources(), before_sources);
        assert_eq!(game.actor_perception_signals(enemy), before_signals);
        game.drain_events();
        let bytes = game.recovery_snapshot_bytes().unwrap();
        let fingerprint = format!("{game:?}");
        let restored =
            crate::game::WorldState::from_recovery_snapshot_bytes(&bytes, game.rules().clone())
                .unwrap();
        assert_eq!(bytes, restored.recovery_snapshot_bytes().unwrap());
        assert_eq!(fingerprint, format!("{restored:?}"));
        game.process_player_command(GameCommand::Wait);
        assert!(game.sound_observation_sources().is_empty());
        assert_eq!(game.actor_perception_signals(enemy), Some((false, false)));
    }

    #[test]
    fn sound_projection_uses_actual_emitters_and_never_discloses_hidden_sources() {
        let mut rules = GameRules::default();
        rules.player_field_of_view.radius = 2;
        rules.stealth_rules = Some(StealthRules::default());
        let mut game = GameState::new_with_rules(
            Map::from_ascii("##########\n#........#\n#........#\n##########").unwrap(),
            GridPos::new(2, 1),
            17,
            rules,
        )
        .unwrap();
        game.sound_emitters
            .deploy(GridPos::new(3, 1), 30, 2, 1)
            .unwrap();
        game.sound_emitters
            .deploy(GridPos::new(7, 1), 60, 2, 1)
            .unwrap();
        let mut game = crate::game::WorldState::single(game);
        let bytes = game.recovery_snapshot_bytes().unwrap();
        let expected = vec![SoundObservationSource {
            position: GridPos::new(3, 1),
            intensity: 30,
            ongoing: true,
        }];
        assert_eq!(game.sound_observation_sources(), expected);
        assert_eq!(game.sound_observation_sources(), expected);
        assert_eq!(bytes, game.recovery_snapshot_bytes().unwrap());
        game.process_player_command(GameCommand::Wait);
        assert_eq!(game.sound_observation_sources(), expected);
        game.process_player_command(GameCommand::Wait);
        assert!(game.sound_observation_sources().is_empty());
    }
}
