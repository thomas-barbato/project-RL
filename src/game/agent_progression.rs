//! Approved rank purchases and free attribute allocation. No turn or random draw.
use super::*;
use std::sync::OnceLock;

#[derive(serde::Deserialize)]
struct Catalogue {
    skills: Vec<Definition>,
}
#[derive(serde::Deserialize)]
struct Definition {
    id: String,
    learned_cap: u8,
    requires: Option<String>,
}
fn definitions() -> &'static [Definition] {
    static DATA: OnceLock<Catalogue> = OnceLock::new();
    &DATA
        .get_or_init(|| {
            serde_json::from_str(include_str!("../../assets/skills/agent/catalogue.json"))
                .expect("reviewed agent skills")
        })
        .skills
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::ContentLoader;
    use crate::game::{GameCommand, WorldState};
    use crate::progression::{ExperienceAward, ExperienceCurve};

    fn fixture() -> GameState {
        let content = ContentLoader::load(
            &[std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content")],
            &semver::Version::new(0, 1, 0),
        )
        .unwrap();
        let class = content
            .character_classes()
            .get(&"core:technomancien".parse().unwrap())
            .unwrap();
        let mut rules = GameRules {
            weapons: content.weapons().clone(),
            items: content.items().clone(),
            statuses: content.statuses().clone(),
            player_weapon_slots: ["core:left_hand", "core:right_hand", "core:reserve"]
                .map(|s| s.parse().unwrap())
                .to_vec(),
            ..GameRules::default()
        };
        class
            .apply_to_rules(&mut rules, class.recommended_attributes())
            .unwrap();
        rules.progression.curve =
            ExperienceCurve::new((1..=100).map(|n| n * 10).collect()).unwrap();
        GameState::new_with_rules(
            Map::from_ascii("#####\n#...#\n#...#\n#####").unwrap(),
            GridPos::new(2, 1),
            19,
            rules,
        )
        .unwrap()
    }
    fn award(game: &mut GameState, xp: u64) {
        game.apply_experience_award(
            &ExperienceAward::repeatable(xp),
            ExperienceSource::DefeatedEntity(EntityId::new(99)),
        );
        game.drain_events();
    }
    fn learn(id: &str) -> GameCommand {
        GameCommand::LearnAgentSkill {
            skill: id.to_owned(),
        }
    }

    #[test]
    fn levels_give_five_free_attributes_and_one_skill_point_even_when_multiple_are_gained() {
        let mut game = fixture();
        assert_eq!(game.unspent_attribute_points(), 0);
        award(&mut game, 30);
        assert_eq!(game.unspent_attribute_points(), 15);
        assert_eq!(game.player_progression.unspent_skill_points(), 3);
        let before = game.player_primary_attributes();
        game.refresh_equipment_resources();
        assert_eq!(game.unspent_attribute_points(), 15);
        assert_eq!(game.player_primary_attributes(), before);
    }

    #[test]
    fn skill_purchases_obey_cost_prerequisite_cap_and_atomic_refusals() {
        let mut game = fixture();
        award(&mut game, 100);
        let before = format!("{game:?}");
        assert!(matches!(
            game.process_player_command(learn("SF01")),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(format!("{game:?}"), before);
        assert!(matches!(
            game.process_player_command(learn("unknown")),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(format!("{game:?}"), before);
        for rank in 1..=10 {
            let turn = game.turn;
            let rng = game.rng;
            assert_eq!(
                game.process_player_command(learn("FE01")),
                CommandOutcome::AppliedWithoutTime
            );
            assert_eq!(game.player_skills.agent_skill_rank("FE01"), rank);
            assert_eq!(
                game.player_progression.unspent_skill_points(),
                10 - u32::from(rank)
            );
            assert_eq!(game.turn, turn);
            assert_eq!(game.rng, rng);
        }
        let before = format!("{game:?}");
        assert!(matches!(
            game.process_player_command(learn("FE01")),
            CommandOutcome::Rejected(_)
        ));
        assert!(matches!(
            game.process_player_command(learn("FE02")),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(format!("{game:?}"), before);
    }

    #[test]
    fn attributes_increase_capacity_without_healing_spending_energy_or_advancing_time() {
        let mut game = fixture();
        award(&mut game, 10);
        game.actors.get_mut(game.player).unwrap().apply_damage(5);
        game.player_energy.spend(5).unwrap();
        let hp = game.actors.get(game.player).unwrap().integrity();
        let energy = game.player_energy.available();
        let maximum_hp = game.actors.get(game.player).unwrap().maximum_integrity();
        let maximum_energy = game.player_energy.capacity();
        let turn = game.turn;
        let rng = game.rng;
        for attribute in [
            PrimaryAttribute::Resilience,
            PrimaryAttribute::Processing,
            PrimaryAttribute::Willpower,
            PrimaryAttribute::Power,
            PrimaryAttribute::Perception,
        ] {
            assert_eq!(
                game.process_player_command(GameCommand::AllocateAttribute { attribute }),
                CommandOutcome::AppliedWithoutTime
            );
        }
        assert_eq!(game.unspent_attribute_points(), 0);
        assert!(game.actors.get(game.player).unwrap().maximum_integrity() > maximum_hp);
        assert!(game.player_energy.capacity() > maximum_energy);
        assert_eq!(game.actors.get(game.player).unwrap().integrity(), hp);
        assert_eq!(game.player_energy.available(), energy);
        assert_eq!(game.turn, turn);
        assert_eq!(game.rng, rng);
        let before = format!("{game:?}");
        assert!(matches!(
            game.process_player_command(GameCommand::AllocateAttribute {
                attribute: PrimaryAttribute::Power
            }),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(format!("{game:?}"), before);
    }

    #[test]
    fn paid_ranks_and_attributes_round_trip_in_json_and_binary_including_over_255() {
        let mut game = fixture();
        award(&mut game, 600);
        for _ in 0..300 {
            assert_eq!(
                game.process_player_command(GameCommand::AllocateAttribute {
                    attribute: PrimaryAttribute::Power
                }),
                CommandOutcome::AppliedWithoutTime
            );
        }
        game.process_player_command(learn("FE01"));
        let expected = game.player_primary_attributes().unwrap();
        assert!(expected.value(PrimaryAttribute::Power) > 255);
        let json = game.export_player_progression().unwrap();
        let mut restored = fixture();
        restored.restore_player_progression(&json).unwrap();
        assert_eq!(restored.player_primary_attributes(), Some(expected));
        assert_eq!(restored.unspent_attribute_points(), 0);
        assert_eq!(restored.player_skills, game.player_skills);
        game.drain_events();
        let rules = game.rules.clone();
        let world = WorldState::single(game);
        let bytes = world.recovery_snapshot_bytes().unwrap();
        assert!(bytes.starts_with(b"RLWS\x0f"));
        let restored = WorldState::from_recovery_snapshot_bytes(&bytes, rules).unwrap();
        assert_eq!(restored.player_primary_attributes(), Some(expected));
        assert_eq!(restored.unspent_attribute_points(), 0);
        assert_eq!(restored.player_skills().agent_skill_rank("FE01"), 1);
    }

    #[test]
    fn version12_snapshots_and_old_agent_json_recover_owed_attribute_points() {
        let mut game = fixture();
        award(&mut game, 20);
        let mut json: serde_json::Value =
            serde_json::from_str(&game.export_player_progression().unwrap()).unwrap();
        json.as_object_mut().unwrap().remove("agent_granted_skills");
        json.as_object_mut().unwrap().remove("attributes");
        let mut restored = fixture();
        restored
            .restore_player_progression(&json.to_string())
            .unwrap();
        assert_eq!(restored.unspent_attribute_points(), 10);
        let rules = game.rules.clone();
        let world = WorldState::single(game);
        let mut bytes =
            crate::stats::with_legacy_attribute_encoding(|| world.recovery_snapshot_bytes())
                .unwrap();
        bytes[4] = 12;
        let restored = WorldState::from_recovery_snapshot_bytes(&bytes, rules).unwrap();
        assert_eq!(restored.unspent_attribute_points(), 10);
        assert_eq!(restored.player_progression().unspent_skill_points(), 2);
    }

    #[test]
    fn forged_paid_ranks_or_overspent_attributes_are_rejected_without_mutation() {
        let mut game = fixture();
        award(&mut game, 10);
        let before = format!("{game:?}");
        let mut saved: serde_json::Value =
            serde_json::from_str(&game.export_player_progression().unwrap()).unwrap();
        saved["agent_ranks"]["FE01"] = serde_json::json!(10);
        saved["agent_granted_skills"] = serde_json::json!(["FE01"]);
        assert!(game.restore_player_progression(&saved.to_string()).is_err());
        assert_eq!(format!("{game:?}"), before);
        let mut saved: serde_json::Value =
            serde_json::from_str(&game.export_player_progression().unwrap()).unwrap();
        saved["attributes"]["values"][0] = serde_json::json!(500);
        assert!(game.restore_player_progression(&saved.to_string()).is_err());
        assert_eq!(format!("{game:?}"), before);
    }
}

impl GameState {
    pub fn agent_skill_next_rank(&self, id: &str) -> Result<u8, CommandRejection> {
        if self.rules.agent_class_start.is_none() {
            return Err(CommandRejection::AgentProgressionUnavailable);
        }
        let definition = definitions()
            .iter()
            .find(|s| s.id == id)
            .ok_or(CommandRejection::AgentProgressionUnavailable)?;
        let current = self.player_skills.agent_skill_rank(id);
        if current >= definition.learned_cap
            || definition
                .requires
                .as_ref()
                .is_some_and(|id| self.player_skills.agent_skill_rank(id) == 0)
        {
            return Err(CommandRejection::AgentProgressionUnavailable);
        }
        if self.player_progression.unspent_skill_points() == 0 {
            return Err(CommandRejection::InsufficientSkillPoints {
                required: 1,
                available: 0,
            });
        }
        Ok(current + 1)
    }

    pub(super) fn learn_agent_skill(&mut self, id: &str) -> Result<(), CommandRejection> {
        let rank = self.agent_skill_next_rank(id)?;
        self.player_progression
            .spend_skill_points(1)
            .expect("quote checked available point");
        self.player_skills.set_agent_skill_rank(id, rank);
        self.refresh_equipment_resources();
        Ok(())
    }

    /// Intrinsic increases are stored on the actor in existing snapshots.
    /// Deriving the balance also restores all points owed to older agent runs.
    pub fn unspent_attribute_points(&self) -> u32 {
        if self.rules.agent_class_start.is_none() {
            return 0;
        }
        let spent = self.player_primary_attributes().map_or(0, |current| {
            PrimaryAttribute::ALL
                .into_iter()
                .map(|a| {
                    u32::from(
                        current
                            .value(a)
                            .saturating_sub(self.rules.player_starting_attributes.value(a)),
                    )
                })
                .sum::<u32>()
        });
        u32::from(self.player_progression.level().saturating_sub(1))
            .saturating_mul(5)
            .saturating_sub(spent)
    }

    pub fn can_allocate_attribute(&self, attribute: PrimaryAttribute) -> bool {
        self.unspent_attribute_points() > 0
            && self
                .player_primary_attributes()
                .is_some_and(|values| values.value(attribute) < u16::MAX)
    }

    pub(super) fn allocate_agent_attribute(
        &mut self,
        attribute: PrimaryAttribute,
    ) -> Result<(), CommandRejection> {
        if !self.can_allocate_attribute(attribute) {
            return Err(CommandRejection::AgentProgressionUnavailable);
        }
        let attributes = self
            .player_primary_attributes()
            .ok_or(CommandRejection::MissingPlayer)?;
        self.actors
            .get_mut(self.player)
            .ok_or(CommandRejection::MissingPlayer)?
            .set_primary_attributes(
                attributes.with_value(attribute, attributes.value(attribute) + 1),
            );
        self.refresh_equipment_resources();
        Ok(())
    }
}
