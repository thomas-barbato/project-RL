//! Approved cryogenic-agent starting profiles. Classes grant a start, not access restrictions.
use crate::stats::{PrimaryAttribute, PrimaryAttributes};

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AgentClassStart {
    pub skills: Vec<String>,
    pub maximum_hp_percent: u16,
    pub maximum_energy_percent: u16,
    pub armor: u16,
    pub evasion: i16,
    pub weapon_damage_percent: u16,
    pub melee_damage_percent: u16,
    pub ranged_damage_percent: u16,
    pub spell_damage_percent: u16,
    pub magic_energy_percent: u16,
    pub drone_hp_percent: u16,
    pub drone_damage_percent: u16,
    pub transformed_damage_percent: u16,
    pub transformation_duration_bonus: u16,
    pub weakening_duration_bonus: u16,
    pub human_skill_energy_percent: u16,
}
impl AgentClassStart {
    pub fn maximum_hp(&self, base: u16) -> u16 {
        percent(base, self.maximum_hp_percent).max(1)
    }
    pub fn maximum_energy(&self, attributes: PrimaryAttributes, level: u16, flat: u16) -> u16 {
        let intelligence = i32::from(attributes.value(PrimaryAttribute::Processing));
        let will = i32::from(attributes.value(PrimaryAttribute::Willpower));
        let base = (30
            + 2 * (intelligence - 5)
            + 2 * (will - 5)
            + 2 * i32::from(level.saturating_sub(1))
            + i32::from(flat))
        .clamp(1, i32::from(u16::MAX)) as u16;
        percent(base, self.maximum_energy_percent).max(1)
    }
}
pub fn percent(value: u16, percentage: u16) -> u16 {
    (u32::from(value) * u32::from(percentage) / 100).min(u32::from(u16::MAX)) as u16
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::content::{ContentId, ContentLoader};
    use crate::game::{GameRules, GameState};
    use crate::world::{GridPos, Map};
    use semver::Version;

    #[test]
    fn five_agent_starts_have_fixed_attributes_equipped_outfit_and_full_resources() {
        let loaded = ContentLoader::load(
            &[std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content")],
            &Version::new(0, 1, 0),
        )
        .unwrap();
        let expected = [
            ("bastion", 45, 27), // 41 base HP plus its learned B03 rank-1 passive.
            ("technomancien", 30, 38),
            ("traqueur", 30, 30),
            ("arcaniste", 30, 36),
            ("metamorphe", 34, 32),
        ];
        assert_eq!(
            loaded
                .character_classes()
                .iter()
                .map(|(id, _)| id.as_str())
                .collect::<Vec<_>>(),
            expected.map(|(id, _, _)| format!("core:{id}"))
        );
        for (slug, hp, energy) in expected {
            let id: ContentId = format!("core:{slug}").parse().unwrap();
            let class = loaded.character_classes().get(&id).unwrap();
            let mut rules = GameRules {
                statuses: loaded.statuses().clone(),
                weapons: loaded.weapons().clone(),
                items: loaded.items().clone(),
                player_weapon_slots: ["core:left_hand", "core:right_hand", "core:reserve"]
                    .map(|v| v.parse().unwrap())
                    .to_vec(),
                ..GameRules::default()
            };
            let attributes = class.recommended_attributes();
            assert_eq!(attributes.total(), 34);
            class.apply_to_rules(&mut rules, attributes).unwrap();
            let mut game = GameState::new_with_rules(
                Map::from_ascii("#####\n#...#\n#...#\n#...#\n#####").unwrap(),
                GridPos::new(2, 2),
                19,
                rules.clone(),
            )
            .unwrap();
            let actor = game.actors().get(game.player_id()).unwrap();
            assert_eq!(
                (actor.integrity(), actor.maximum_integrity()),
                (hp, hp),
                "{slug}"
            );
            assert_eq!(
                (
                    game.player_energy().available(),
                    game.player_energy().capacity()
                ),
                (energy, energy),
                "{slug}"
            );
            assert_eq!(game.player_effective_primary_attributes(), Some(attributes));
            let body: ContentId = "core:body_armor".parse().unwrap();
            let torso = game
                .player_equipment()
                .equipped(&body)
                .expect("starter torso equipped");
            assert!(
                game.player_inventory()
                    .get(torso)
                    .unwrap()
                    .item()
                    .as_str()
                    .starts_with("core:tenue_")
            );
            assert_eq!(game.player_skills().agent_skills().count(), 2);
            assert_eq!(game.player_progression().unspent_skill_points(), 0);
            let saved = game.export_player_progression().unwrap();
            game.restore_player_progression(&saved).unwrap();
            for skill in &class.agent_start().unwrap().skills {
                assert_eq!(game.player_skills().agent_skill_rank(skill), 1);
            }
            let wrong = attributes
                .with_value(
                    PrimaryAttribute::Power,
                    attributes.value(PrimaryAttribute::Power) + 1,
                )
                .with_value(
                    PrimaryAttribute::Coordination,
                    attributes.value(PrimaryAttribute::Coordination) - 1,
                );
            assert!(matches!(
                class.apply_to_rules(&mut rules, wrong),
                Err(
                    crate::character_class::CharacterClassApplicationError::FixedStartingAttributes
                )
            ));
        }
    }
}
