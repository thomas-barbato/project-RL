//! Surface encounter density, kept separate from terrain and older replays.
use super::*;
use project_rl::content::RegionPopulationProfile;
use project_rl::world::generation::{
    RegionalEncounterLayout, RegionalPopulationFeatures, generate_regional_encounters_in_layout,
};
use std::collections::BTreeSet;

impl AsciiApp {
    pub(super) fn populate_starter_encounters(&mut self) -> Result<(), String> {
        let encounters = self
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .and_then(|world| world.biome(&"core:human_habitat".parse().unwrap()))
            .ok_or("Starter surface encounter profile missing")?
            .encounters();
        // Reuse the authored humanoids and their real weapons; do not infer a
        // species from the older prototypes or silently populate with robots.
        let rules = encounters
            .rules()
            .iter()
            .filter(|rule| {
                rule.tags().iter().any(|tag| {
                    matches!(
                        tag.as_str(),
                        "core:humanoid_rifle_carrier" | "core:humanoid_knife_carrier"
                    )
                })
            })
            .cloned()
            .collect();
        let profile = RegionPopulationProfile::new(20, 24, rules)
            .map_err(|e| e.to_string())?
            .with_spread_groups(true);
        let mut placement_map = self.game.map().clone();
        // A placement-only town buffer: no terrain, collision or safe-zone
        // semantics are changed in the live map.
        for y in 0..=47 {
            for x in 0..=65 {
                placement_map
                    .set_protected(GridPos::new(x, y), true)
                    .map_err(|e| e.to_string())?;
            }
        }
        let existing_actors: Vec<_> = self
            .game
            .actors()
            .iter()
            .map(|(_, actor)| actor.position())
            .collect();
        let reserved: BTreeSet<_> = existing_actors
            .iter()
            .copied()
            .chain(
                self.game
                    .ground_items()
                    .iter()
                    .map(|(_, item)| item.position()),
            )
            .collect();
        let passages: Vec<_> = TestSector::EXPANDED_REGIONAL_PASSAGES
            .iter()
            .map(|(_, at)| *at)
            .chain([
                TestSector::GATE,
                TestSector::EXPANDED_EXPEDITION_PASSAGE,
                TestSector::RECYCLING_START,
                TestSector::RECYCLING_MAIN_DOOR,
                TestSector::RECYCLING_CONDUIT,
            ])
            .collect();
        // First groups occupy the approaches to existing ruins and traversal
        // routes. The remaining draws fill quieter areas, independently seeded.
        let landmarks = [
            GridPos::new(110, 40),
            GridPos::new(155, 20),
            GridPos::new(143, 83),
            GridPos::new(74, 72),
            GridPos::new(103, 107),
            GridPos::new(39, 65),
            GridPos::new(50, 99),
            GridPos::new(125, 22),
            GridPos::new(164, 50),
            GridPos::new(166, 112),
        ];
        let mut actors = generate_regional_encounters_in_layout(
            &placement_map,
            &passages,
            RegionalEncounterLayout {
                landmarks: &landmarks,
                reserved: &reserved,
                existing_actors: &existing_actors,
            },
            &profile,
            self.seed ^ 0x4855_425f_454e_4354,
            RegionalPopulationFeatures {
                pursuit_lifecycle: true,
                primary_attributes: true,
                physical_profiles: true,
                electronic_systems: true,
                player_relations: true,
            },
            true,
        )
        .map_err(|e| e.to_string())?;
        self.equip_carried_weapons(&mut actors, 0, self.seed)?;
        for actor in actors {
            self.game.spawn_actor(actor).map_err(|e| e.to_string())?;
        }
        self.game.drain_events();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn app(version: u8) -> AsciiApp {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, version).unwrap()
    }

    #[test]
    fn starter_density_adds_equipped_separated_humanoids_without_changing_the_map() {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        for seed in [INITIAL_SEED, 0, 1, 17, 91, 255, 1024, u64::MAX] {
            let make = |version| {
                AsciiApp::from_seed_version(
                    seed,
                    rules.clone(),
                    texts.clone(),
                    loot.clone(),
                    expeditions.clone(),
                    version,
                )
                .unwrap()
            };
            let old = make(108);
            let current = make(109);
            assert_eq!(
                format!("{:?}", old.game.map()),
                format!("{:?}", current.game.map())
            );
            let added: Vec<_> = current
                .game
                .actors()
                .iter()
                .filter(|(_, actor)| actor.equipped_weapon().is_some())
                .collect();
            assert!(
                (20..=24).contains(&added.len()),
                "seed {seed}: {}",
                added.len()
            );
            assert_eq!(
                current.game.actors().iter().count(),
                old.game.actors().iter().count() + added.len()
            );
            for (id, actor) in &added {
                let at = actor.position();
                assert!(at.x > 65 || at.y > 47, "no extra enemy in town");
                assert!(!current.game.map().is_protected(at));
                assert!(current.game.map().is_walkable(at));
                assert!(actor.electronic_system().is_none());
                assert_eq!(actor.player_relation(), PlayerRelation::Hostile);
                for (other_id, other) in current.game.actors().iter() {
                    if *id != other_id {
                        let other = other.position();
                        assert!(
                            at.x.abs_diff(other.x).max(at.y.abs_diff(other.y)) >= 12,
                            "seed {seed}: {at:?} too close to {other:?}"
                        );
                    }
                }
                for entry in TestSector::EXPANDED_REGIONAL_PASSAGES
                    .iter()
                    .map(|(_, at)| *at)
                    .chain([
                        TestSector::RECYCLING_START,
                        TestSector::RECYCLING_MAIN_DOOR,
                        TestSector::RECYCLING_CONDUIT,
                        TestSector::GATE,
                        TestSector::EXPANDED_EXPEDITION_PASSAGE,
                    ])
                {
                    assert!(at.x.abs_diff(entry.x).max(at.y.abs_diff(entry.y)) >= 16);
                }
            }
            // Historical actors, fauna, items and their IDs remain untouched.
            for (id, actor) in old.game.actors().iter() {
                assert_eq!(current.game.actors().get(id), Some(actor));
            }
            assert_eq!(
                format!("{:?}", old.game.ground_items()),
                format!("{:?}", current.game.ground_items())
            );
        }
    }

    #[test]
    fn denser_surface_preserves_prologue_and_both_resume_paths() {
        let mut current = app(109);
        let count = current
            .game
            .actors()
            .iter()
            .filter(|(_, actor)| actor.equipped_weapon().is_some())
            .count();
        current.walk_fixture_out_of_recycling().unwrap();
        current.capture_events();
        assert!(current.intro_city_reached);
        let saved = current.suspension().unwrap();
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        assert_eq!(
            restored
                .game
                .actors()
                .iter()
                .filter(|(_, actor)| actor.equipped_weapon().is_some())
                .count(),
            count
        );
        let resumed = WorldState::from_recovery_snapshot_bytes(
            &current.game.recovery_snapshot_bytes().unwrap(),
            current.game.rules().clone(),
        )
        .unwrap();
        assert_eq!(format!("{:?}", current.game), format!("{resumed:?}"));
    }

    #[test]
    fn version_108_surface_fingerprint() {
        let mut old = app(108);
        let saved = old.suspension().unwrap();
        assert_eq!(saved.rules, 6198306336801966463);
        assert_eq!(saved.world_rules, Some(1578296249672886813));
        assert_eq!(saved.state, 7122100530405190243);
        let passage = TestSector::EXPANDED_REGIONAL_PASSAGES[3].1;
        old.walk_fixture_to(passage.step(Direction::East)).unwrap();
        old.execute_command(GameCommand::Interact { target: passage });
        old.capture_events();
        let saved = old.suspension().unwrap();
        assert_eq!(saved.state, 462931943177179174);
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }
}
