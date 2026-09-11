use std::error::Error;
use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::content::ContentId;
use crate::skills::{
    SkillCatalog, SkillProgressionRules, SkillProgressionState, SkillProgressionStateError,
    SystemFeatureSet,
};

use super::{ProgressionRules, RewardKey, RunProgression, RunProgressionRestoreError};

pub const PLAYER_PROGRESSION_SAVE_VERSION: u16 = 1;
const MAX_PLAYER_PROGRESSION_SAVE_BYTES: usize = 1024 * 1024;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RestoredPlayerProgression {
    run: RunProgression,
    skills: SkillProgressionState,
}

impl RestoredPlayerProgression {
    pub const fn run(&self) -> &RunProgression {
        &self.run
    }

    pub const fn skills(&self) -> &SkillProgressionState {
        &self.skills
    }

    pub fn into_parts(self) -> (RunProgression, SkillProgressionState) {
        (self.run, self.skills)
    }
}

pub fn encode_player_progression(
    run: &RunProgression,
    skills: &SkillProgressionState,
) -> Result<String, PlayerProgressionSaveError> {
    let document = PlayerProgressionDocument {
        schema_version: PLAYER_PROGRESSION_SAVE_VERSION,
        experience: run.experience(),
        level: run.level(),
        skill_points_earned: run.skill_points_earned(),
        unspent_skill_points: run.unspent_skill_points(),
        claimed_one_time_rewards: run
            .claimed_one_time_rewards()
            .map(|key| key.as_str().to_owned())
            .collect(),
        disciplines: skills
            .disciplines()
            .map(|(discipline, choices)| SavedDisciplineProgression {
                discipline: discipline.as_str().to_owned(),
                choices: choices
                    .iter()
                    .map(|technique| technique.as_str().to_owned())
                    .collect(),
            })
            .collect(),
    };
    serde_json::to_string_pretty(&document)
        .map_err(|error| PlayerProgressionSaveError::Serialization(error.to_string()))
}

pub fn decode_player_progression(
    source: &str,
    progression_rules: &ProgressionRules,
    skill_catalog: &SkillCatalog,
    system_features: &SystemFeatureSet,
    skill_rules: &SkillProgressionRules,
) -> Result<RestoredPlayerProgression, PlayerProgressionSaveError> {
    if source.len() > MAX_PLAYER_PROGRESSION_SAVE_BYTES {
        return Err(PlayerProgressionSaveError::TooLarge {
            bytes: source.len(),
            maximum: MAX_PLAYER_PROGRESSION_SAVE_BYTES,
        });
    }
    let document: PlayerProgressionDocument = serde_json::from_str(source)
        .map_err(|error| PlayerProgressionSaveError::Syntax(error.to_string()))?;
    if document.schema_version != PLAYER_PROGRESSION_SAVE_VERSION {
        return Err(PlayerProgressionSaveError::UnsupportedVersion {
            found: document.schema_version,
            supported: PLAYER_PROGRESSION_SAVE_VERSION,
        });
    }

    let rewards = document
        .claimed_one_time_rewards
        .into_iter()
        .map(|value| {
            RewardKey::new(value.clone()).map_err(|error| {
                PlayerProgressionSaveError::InvalidRewardKey {
                    value,
                    explanation: error.to_string(),
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let run = RunProgression::from_saved_parts(
        document.experience,
        document.level,
        document.skill_points_earned,
        document.unspent_skill_points,
        rewards,
        progression_rules,
    )
    .map_err(|error| PlayerProgressionSaveError::Run(Box::new(error)))?;

    let entries = document
        .disciplines
        .into_iter()
        .map(|entry| {
            let discipline = parse_content_id(entry.discipline)?;
            let choices = entry
                .choices
                .into_iter()
                .map(parse_content_id)
                .collect::<Result<Vec<_>, _>>()?;
            Ok((discipline, choices))
        })
        .collect::<Result<Vec<_>, PlayerProgressionSaveError>>()?;
    let skills = SkillProgressionState::from_ordered_choices(
        entries,
        skill_catalog,
        system_features,
        skill_rules,
    )
    .map_err(|error| PlayerProgressionSaveError::Skills(Box::new(error)))?;
    let spent_skill_points = skills.disciplines().fold(0_u32, |total, (_, choices)| {
        let discipline_cost = skill_rules
            .rank_costs()
            .iter()
            .take(choices.len())
            .fold(0_u32, |cost, rank_cost| {
                cost.saturating_add(u32::from(*rank_cost))
            });
        total.saturating_add(discipline_cost)
    });
    let accounted_skill_points = run
        .unspent_skill_points()
        .saturating_add(spent_skill_points);
    if accounted_skill_points != run.skill_points_earned() {
        return Err(PlayerProgressionSaveError::SkillPointAccounting {
            earned: run.skill_points_earned(),
            spent: spent_skill_points,
            unspent: run.unspent_skill_points(),
        });
    }

    Ok(RestoredPlayerProgression { run, skills })
}

fn parse_content_id(value: String) -> Result<ContentId, PlayerProgressionSaveError> {
    value
        .parse()
        .map_err(|error: crate::content::ContentIdError| {
            PlayerProgressionSaveError::InvalidContentId {
                value,
                explanation: error.to_string(),
            }
        })
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PlayerProgressionDocument {
    schema_version: u16,
    experience: u64,
    level: u16,
    skill_points_earned: u32,
    unspent_skill_points: u32,
    claimed_one_time_rewards: Vec<String>,
    disciplines: Vec<SavedDisciplineProgression>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SavedDisciplineProgression {
    discipline: String,
    choices: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PlayerProgressionSaveError {
    TooLarge {
        bytes: usize,
        maximum: usize,
    },
    Syntax(String),
    Serialization(String),
    UnsupportedVersion {
        found: u16,
        supported: u16,
    },
    InvalidRewardKey {
        value: String,
        explanation: String,
    },
    InvalidContentId {
        value: String,
        explanation: String,
    },
    Run(Box<RunProgressionRestoreError>),
    Skills(Box<SkillProgressionStateError>),
    SkillPointAccounting {
        earned: u32,
        spent: u32,
        unspent: u32,
    },
}

impl Display for PlayerProgressionSaveError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooLarge { bytes, maximum } => write!(
                formatter,
                "player progression save contains {bytes} bytes; maximum is {maximum}"
            ),
            Self::Syntax(explanation) => {
                write!(formatter, "invalid player progression save: {explanation}")
            }
            Self::Serialization(explanation) => {
                write!(
                    formatter,
                    "could not serialize player progression: {explanation}"
                )
            }
            Self::UnsupportedVersion { found, supported } => write!(
                formatter,
                "unsupported player progression save version {found}; this build supports {supported}"
            ),
            Self::InvalidRewardKey { value, explanation } => {
                write!(
                    formatter,
                    "invalid saved reward key '{value}': {explanation}"
                )
            }
            Self::InvalidContentId { value, explanation } => {
                write!(
                    formatter,
                    "invalid saved content ID '{value}': {explanation}"
                )
            }
            Self::Run(error) => write!(formatter, "invalid saved run progression: {error}"),
            Self::Skills(error) => write!(formatter, "invalid saved skill progression: {error}"),
            Self::SkillPointAccounting {
                earned,
                spent,
                unspent,
            } => write!(
                formatter,
                "saved skill points do not balance: {earned} earned, {spent} spent, {unspent} unspent"
            ),
        }
    }
}

impl Error for PlayerProgressionSaveError {}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use semver::Version;

    use super::*;
    use crate::content::ContentLoader;
    use crate::progression::ExperienceAward;

    fn id(value: &str) -> ContentId {
        value
            .parse()
            .unwrap_or_else(|error| panic!("valid content ID rejected: {error}"))
    }

    fn core_skills() -> SkillCatalog {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content");
        ContentLoader::load(&[root], &Version::new(0, 1, 0))
            .unwrap_or_else(|error| panic!("core content failed to load: {error}"))
            .skills()
            .clone()
    }

    fn recon_features() -> SystemFeatureSet {
        SystemFeatureSet::new([id("core:traces"), id("core:secrets")])
    }

    fn learned_recon(catalog: &SkillCatalog) -> SkillProgressionState {
        SkillProgressionState::from_ordered_choices(
            [(
                id("core:reconnaissance"),
                vec![id("core:rec_01"), id("core:rec_04")],
            )],
            catalog,
            &recon_features(),
            &SkillProgressionRules::default(),
        )
        .unwrap_or_else(|error| panic!("valid skill state rejected: {error}"))
    }

    #[test]
    fn player_progression_round_trip_preserves_ordered_choices_and_rewards() {
        let progression_rules = ProgressionRules::default();
        let mut run =
            RunProgression::with_starting_skill_points(progression_rules.starting_skill_points);
        run.award(
            &ExperienceAward::one_time(
                25,
                RewardKey::new("core:test_reward")
                    .unwrap_or_else(|error| panic!("valid reward key rejected: {error}")),
            ),
            &progression_rules,
        );
        run.spend_skill_points(2)
            .unwrap_or_else(|error| panic!("earned skill points could not be spent: {error}"));
        let catalog = core_skills();
        let skills = learned_recon(&catalog);

        let encoded = encode_player_progression(&run, &skills)
            .unwrap_or_else(|error| panic!("valid progression failed to serialize: {error}"));
        let restored = decode_player_progression(
            &encoded,
            &progression_rules,
            &catalog,
            &recon_features(),
            &SkillProgressionRules::default(),
        )
        .unwrap_or_else(|error| panic!("valid progression failed to restore: {error}"));

        assert_eq!(restored.run(), &run);
        assert_eq!(restored.skills(), &skills);
        assert_eq!(
            encode_player_progression(restored.run(), restored.skills()),
            Ok(encoded)
        );
    }

    #[test]
    fn changed_experience_curve_requires_an_explicit_migration() {
        let catalog = core_skills();
        let source = r#"{
            "schema_version": 1,
            "experience": 10,
            "level": 1,
            "skill_points_earned": 2,
            "unspent_skill_points": 2,
            "claimed_one_time_rewards": [],
            "disciplines": []
        }"#;

        assert!(matches!(
            decode_player_progression(
                source,
                &ProgressionRules::default(),
                &catalog,
                &recon_features(),
                &SkillProgressionRules::default(),
            ),
            Err(PlayerProgressionSaveError::Run(error))
                if matches!(*error, RunProgressionRestoreError::LevelMismatch { .. })
        ));
    }

    #[test]
    fn removed_technique_is_not_silently_replaced() {
        let catalog = core_skills();
        let source = r#"{
            "schema_version": 1,
            "experience": 0,
            "level": 1,
            "skill_points_earned": 2,
            "unspent_skill_points": 2,
            "claimed_one_time_rewards": [],
            "disciplines": [{
                "discipline": "core:reconnaissance",
                "choices": ["core:removed_technique"]
            }]
        }"#;

        assert!(matches!(
            decode_player_progression(
                source,
                &ProgressionRules::default(),
                &catalog,
                &recon_features(),
                &SkillProgressionRules::default(),
            ),
            Err(PlayerProgressionSaveError::Skills(_))
        ));
    }

    #[test]
    fn learned_discipline_is_rejected_when_its_version_is_deferred() {
        let catalog = core_skills();
        let skills = learned_recon(&catalog);
        let source = encode_player_progression(
            &RunProgression::with_starting_skill_points(
                ProgressionRules::default().starting_skill_points,
            ),
            &skills,
        )
        .unwrap_or_else(|error| panic!("valid progression failed to serialize: {error}"));

        assert!(matches!(
            decode_player_progression(
                &source,
                &ProgressionRules::default(),
                &catalog,
                &SystemFeatureSet::default(),
                &SkillProgressionRules::default(),
            ),
            Err(PlayerProgressionSaveError::Skills(error))
                if matches!(*error, SkillProgressionStateError::DeferredDiscipline(_))
        ));
    }

    #[test]
    fn duplicate_saved_discipline_is_rejected_instead_of_overwritten() {
        let catalog = core_skills();
        let source = r#"{
            "schema_version": 1,
            "experience": 0,
            "level": 1,
            "skill_points_earned": 2,
            "unspent_skill_points": 0,
            "claimed_one_time_rewards": [],
            "disciplines": [
                {"discipline": "core:reconnaissance", "choices": ["core:rec_01"]},
                {"discipline": "core:reconnaissance", "choices": ["core:rec_02"]}
            ]
        }"#;

        assert!(matches!(
            decode_player_progression(
                source,
                &ProgressionRules::default(),
                &catalog,
                &recon_features(),
                &SkillProgressionRules::default(),
            ),
            Err(PlayerProgressionSaveError::Skills(error))
                if matches!(*error, SkillProgressionStateError::DuplicateDiscipline(_))
        ));
    }

    #[test]
    fn saved_improvement_without_its_prerequisite_is_rejected() {
        let catalog = core_skills();
        let source = r#"{
            "schema_version": 1,
            "experience": 25,
            "level": 3,
            "skill_points_earned": 4,
            "unspent_skill_points": 0,
            "claimed_one_time_rewards": [],
            "disciplines": [{
                "discipline": "core:reconnaissance",
                "choices": ["core:rec_02", "core:rec_04", "core:rec_09"]
            }]
        }"#;

        assert!(matches!(
            decode_player_progression(
                source,
                &ProgressionRules::default(),
                &catalog,
                &recon_features(),
                &SkillProgressionRules::default(),
            ),
            Err(PlayerProgressionSaveError::Skills(error))
                if matches!(
                    error.as_ref(),
                    SkillProgressionStateError::InvalidChoices { error, .. }
                        if matches!(
                            error.as_ref(),
                            crate::skills::InitialChoicesError::MissingPrerequisite(_)
                        )
                )
        ));
    }

    #[test]
    fn saved_skill_points_must_balance_earned_spent_and_unspent() {
        let catalog = core_skills();
        let source = r#"{
            "schema_version": 1,
            "experience": 0,
            "level": 1,
            "skill_points_earned": 2,
            "unspent_skill_points": 2,
            "claimed_one_time_rewards": [],
            "disciplines": [{
                "discipline": "core:reconnaissance",
                "choices": ["core:rec_01"]
            }]
        }"#;

        assert!(matches!(
            decode_player_progression(
                source,
                &ProgressionRules::default(),
                &catalog,
                &recon_features(),
                &SkillProgressionRules::default(),
            ),
            Err(PlayerProgressionSaveError::SkillPointAccounting {
                earned: 2,
                spent: 1,
                unspent: 2,
            })
        ));
    }
}
