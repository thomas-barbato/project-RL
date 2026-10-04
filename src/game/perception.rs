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
