//! Cached, client-only projection; refreshing it never advances the simulation.
use super::*;
use crate::terminal_view;

#[derive(Default)]
pub(super) struct TacticalProjection {
    pub vision: Vec<project_rl::game::ActorObservationField>,
    pub sounds: Vec<terminal_view::TerminalSoundField>,
    pub signals: Vec<terminal_view::TerminalPerceptionSignal>,
}

#[derive(Default)]
pub(super) struct TacticalCache {
    key: Option<(Option<ContentId>, u64, Option<GridPos>, bool)>,
    projection: TacticalProjection,
    #[cfg(test)]
    rebuilds: usize,
}

impl TacticalCache {
    pub fn invalidate(&mut self) {
        self.key = None;
    }

    pub fn projection(&mut self, game: &WorldState, shown: bool) -> &TacticalProjection {
        let key = (
            game.current_zone().map(|z| z.id.clone()),
            game.turn(),
            game.player_position(),
            shown,
        );
        if self.key.as_ref() != Some(&key) {
            let signals = game
                .actors()
                .iter()
                .filter_map(|(entity, actor)| {
                    let (seen, heard) = game.actor_perception_signals(entity)?;
                    (seen || heard).then_some(terminal_view::TerminalPerceptionSignal {
                        position: actor.position(),
                        seen,
                        heard,
                    })
                })
                .collect();
            let vision = if shown {
                game.actor_observation_fields()
            } else {
                Vec::new()
            };
            let sounds = if shown {
                game.rules()
                    .stealth_rules
                    .map(|rules| {
                        game.sound_observation_sources()
                            .into_iter()
                            .map(|source| terminal_view::TerminalSoundField {
                                origin: source.position,
                                radius: source
                                    .intensity
                                    .saturating_sub(1)
                                    .checked_div(rules.sound_attenuation_per_cell)
                                    .unwrap_or(0),
                                circular: rules.circular_sound_fields,
                                positions: rules
                                    .sound_field_on_map(
                                        game.map(),
                                        source.intensity,
                                        source.position,
                                    )
                                    .filter(|&at| {
                                        game.player_visibility().is_visible(at)
                                            && game.map().is_walkable(at)
                                    })
                                    .collect(),
                            })
                            .collect()
                    })
                    .unwrap_or_default()
            } else {
                Vec::new()
            };
            self.projection = TacticalProjection {
                vision,
                sounds,
                signals,
            };
            self.key = Some(key);
            #[cfg(test)]
            {
                self.rebuilds += 1;
            }
        }
        &self.projection
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tactical_projection_reuses_frames_and_tracks_emitter_expiry_without_mutation() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.enter_installation_annex().unwrap();
        let source = GridPos::new(10, 7);
        app.walk_fixture_to_unchecked(source.step(Direction::East))
            .unwrap();
        app.execute_command(GameCommand::UseInstallation {
            target: source,
            action: project_rl::facility::InstallationAction::ActivateDiversion,
        });
        app.game.drain_events();
        let before = app.game.recovery_snapshot_bytes().unwrap();
        let mut cache = TacticalCache::default();
        let projection = cache.projection(&app.game, true);
        assert!(!projection.vision.is_empty());
        assert!(!projection.sounds.is_empty());
        assert!(projection.signals.iter().any(|s| s.heard));
        for field in &projection.sounds {
            assert!(app.game.player_visibility().is_visible(field.origin));
            assert!(
                field
                    .positions
                    .iter()
                    .all(|&at| app.game.player_visibility().is_visible(at)
                        && app.game.map().is_walkable(at))
            );
        }
        for _ in 0..20 {
            cache.projection(&app.game, true);
        }
        assert_eq!(cache.rebuilds, 1);
        assert_eq!(before, app.game.recovery_snapshot_bytes().unwrap());
        let hidden = cache.projection(&app.game, false);
        assert!(hidden.vision.is_empty() && hidden.sounds.is_empty());
        assert!(hidden.signals.iter().any(|s| s.heard));
        assert_eq!(before, app.game.recovery_snapshot_bytes().unwrap());
        app.execute_command(GameCommand::Wait);
        assert!(!cache.projection(&app.game, true).sounds.is_empty());
        app.execute_command(GameCommand::Wait);
        assert!(cache.projection(&app.game, true).sounds.is_empty());
    }
}
