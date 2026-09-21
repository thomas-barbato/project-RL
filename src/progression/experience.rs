use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Stable, namespaced identifier used to deduplicate one-time run rewards.
#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct RewardKey(String);

impl RewardKey {
    pub fn new(value: impl Into<String>) -> Result<Self, RewardKeyError> {
        let value = value.into();
        if value.is_empty() {
            return Err(RewardKeyError::Empty);
        }
        if !value.bytes().all(is_stable_id_byte) {
            return Err(RewardKeyError::InvalidCharacter);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_stable_id_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b':' | b'/')
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RewardKeyError {
    Empty,
    InvalidCharacter,
}

impl Display for RewardKeyError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(formatter, "reward key must not be empty"),
            Self::InvalidCharacter => write!(
                formatter,
                "reward key may only contain ASCII letters, digits, '_', '-', '.', ':', or '/'"
            ),
        }
    }
}

impl Error for RewardKeyError {}

/// Cumulative experience totals required to reach levels 2, 3, and so on.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExperienceCurve {
    cumulative_thresholds: Vec<u64>,
}

impl ExperienceCurve {
    pub fn new(cumulative_thresholds: Vec<u64>) -> Result<Self, ExperienceCurveError> {
        let curve = Self {
            cumulative_thresholds,
        };
        curve.validate()?;
        Ok(curve)
    }

    pub fn cumulative_thresholds(&self) -> &[u64] {
        &self.cumulative_thresholds
    }

    pub fn level_for_experience(&self, experience: u64) -> u16 {
        let reached_thresholds = self
            .cumulative_thresholds
            .partition_point(|threshold| *threshold <= experience);
        u16::try_from(reached_thresholds)
            .unwrap_or(u16::MAX - 1)
            .saturating_add(1)
    }

    pub fn next_threshold_after(&self, level: u16) -> Option<u64> {
        self.cumulative_thresholds
            .get(usize::from(level.saturating_sub(1)))
            .copied()
    }

    pub fn validate(&self) -> Result<(), ExperienceCurveError> {
        if self.cumulative_thresholds.is_empty() {
            return Err(ExperienceCurveError::Empty);
        }
        if self.cumulative_thresholds.len() >= usize::from(u16::MAX) {
            return Err(ExperienceCurveError::TooManyLevels);
        }
        if self.cumulative_thresholds[0] == 0 {
            return Err(ExperienceCurveError::ZeroThreshold);
        }
        if self
            .cumulative_thresholds
            .windows(2)
            .any(|thresholds| thresholds[0] >= thresholds[1])
        {
            return Err(ExperienceCurveError::NotStrictlyIncreasing);
        }
        Ok(())
    }
}

impl Default for ExperienceCurve {
    fn default() -> Self {
        Self {
            // Bootstrap values only. Content packs can replace the full curve.
            cumulative_thresholds: vec![10, 25, 45, 70, 100, 140, 190],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExperienceCurveError {
    Empty,
    TooManyLevels,
    ZeroThreshold,
    NotStrictlyIncreasing,
}

impl Display for ExperienceCurveError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(
                formatter,
                "experience curve must contain at least one threshold"
            ),
            Self::TooManyLevels => write!(formatter, "experience curve contains too many levels"),
            Self::ZeroThreshold => {
                write!(formatter, "the first experience threshold must be positive")
            }
            Self::NotStrictlyIncreasing => {
                write!(
                    formatter,
                    "experience thresholds must be strictly increasing"
                )
            }
        }
    }
}

impl Error for ExperienceCurveError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProgressionRules {
    pub curve: ExperienceCurve,
    pub starting_skill_points: u16,
    pub skill_points_per_level: u16,
    pub trivial_threat_level_gap: u16,
    pub trivial_reward_percent: u8,
    pub summoned_reward_percent: u8,
    pub fabricated_reward_percent: u8,
}

impl ProgressionRules {
    pub fn validate(&self) -> Result<(), ProgressionRulesError> {
        self.curve
            .validate()
            .map_err(ProgressionRulesError::Curve)?;
        if self.trivial_threat_level_gap == 0 {
            return Err(ProgressionRulesError::ZeroTrivialThreatGap);
        }
        if self.trivial_reward_percent > 100 {
            return Err(ProgressionRulesError::InvalidTrivialRewardPercent);
        }
        if self.summoned_reward_percent > 100 || self.fabricated_reward_percent > 100 {
            return Err(ProgressionRulesError::InvalidOriginRewardPercent);
        }
        Ok(())
    }

    pub fn experience_for_defeat(&self, player_level: u16, reward: DefeatReward) -> u64 {
        let origin_percent = match reward.origin {
            ExperienceRewardOrigin::Persistent => 100,
            ExperienceRewardOrigin::Summoned => self.summoned_reward_percent,
            ExperienceRewardOrigin::Fabricated => self.fabricated_reward_percent,
        };
        let origin_adjusted = reward
            .base_experience
            .saturating_mul(u64::from(origin_percent))
            / 100;

        let is_trivial = player_level
            >= reward
                .threat_level
                .saturating_add(self.trivial_threat_level_gap);
        if is_trivial {
            origin_adjusted.saturating_mul(u64::from(self.trivial_reward_percent)) / 100
        } else {
            origin_adjusted
        }
    }
}

impl Default for ProgressionRules {
    fn default() -> Self {
        Self {
            curve: ExperienceCurve::default(),
            starting_skill_points: 2,
            skill_points_per_level: 1,
            trivial_threat_level_gap: 3,
            trivial_reward_percent: 0,
            summoned_reward_percent: 0,
            fabricated_reward_percent: 0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProgressionRulesError {
    Curve(ExperienceCurveError),
    ZeroTrivialThreatGap,
    InvalidTrivialRewardPercent,
    InvalidOriginRewardPercent,
}

impl Display for ProgressionRulesError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Curve(error) => write!(formatter, "invalid experience curve: {error}"),
            Self::ZeroTrivialThreatGap => {
                write!(formatter, "trivial threat level gap must be positive")
            }
            Self::InvalidTrivialRewardPercent => {
                write!(formatter, "trivial reward percent must not exceed 100")
            }
            Self::InvalidOriginRewardPercent => write!(
                formatter,
                "summoned and fabricated reward percents must not exceed 100"
            ),
        }
    }
}

impl Error for ProgressionRulesError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ExperienceRewardOrigin {
    Persistent,
    Summoned,
    Fabricated,
}

/// Experience content attached to a defeated actor.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DefeatReward {
    pub base_experience: u64,
    pub threat_level: u16,
    pub origin: ExperienceRewardOrigin,
}

impl DefeatReward {
    pub const fn persistent(base_experience: u64, threat_level: u16) -> Self {
        Self {
            base_experience,
            threat_level,
            origin: ExperienceRewardOrigin::Persistent,
        }
    }

    pub const fn summoned(base_experience: u64, threat_level: u16) -> Self {
        Self {
            base_experience,
            threat_level,
            origin: ExperienceRewardOrigin::Summoned,
        }
    }

    pub const fn fabricated(base_experience: u64, threat_level: u16) -> Self {
        Self {
            base_experience,
            threat_level,
            origin: ExperienceRewardOrigin::Fabricated,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExperienceAward {
    amount: u64,
    one_time_key: Option<RewardKey>,
}

impl ExperienceAward {
    pub const fn repeatable(amount: u64) -> Self {
        Self {
            amount,
            one_time_key: None,
        }
    }

    pub const fn one_time(amount: u64, key: RewardKey) -> Self {
        Self {
            amount,
            one_time_key: Some(key),
        }
    }

    pub const fn amount(&self) -> u64 {
        self.amount
    }

    pub const fn one_time_key(&self) -> Option<&RewardKey> {
        self.one_time_key.as_ref()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LevelGain {
    pub level: u16,
    pub skill_points_awarded: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ExperienceAwardOutcome {
    pub awarded_experience: u64,
    pub previous_total: u64,
    pub total: u64,
    pub level_gains: Vec<LevelGain>,
    pub duplicate_one_time_reward: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct RunProgression {
    experience: u64,
    level: u16,
    skill_points_earned: u32,
    unspent_skill_points: u32,
    claimed_one_time_rewards: BTreeSet<RewardKey>,
}

impl Default for RunProgression {
    fn default() -> Self {
        Self::with_starting_skill_points(0)
    }
}

impl RunProgression {
    pub fn with_starting_skill_points(starting_skill_points: u16) -> Self {
        let starting_skill_points = u32::from(starting_skill_points);
        Self {
            experience: 0,
            level: 1,
            skill_points_earned: starting_skill_points,
            unspent_skill_points: starting_skill_points,
            claimed_one_time_rewards: BTreeSet::new(),
        }
    }

    pub const fn experience(&self) -> u64 {
        self.experience
    }

    pub const fn level(&self) -> u16 {
        self.level
    }

    pub const fn unspent_skill_points(&self) -> u32 {
        self.unspent_skill_points
    }

    pub const fn skill_points_earned(&self) -> u32 {
        self.skill_points_earned
    }

    pub fn claimed_one_time_rewards(&self) -> impl Iterator<Item = &RewardKey> {
        self.claimed_one_time_rewards.iter()
    }

    pub fn spend_skill_points(&mut self, amount: u16) -> Result<(), SkillPointSpendError> {
        let required = u32::from(amount);
        if self.unspent_skill_points < required {
            return Err(SkillPointSpendError {
                required: amount,
                available: self.unspent_skill_points,
            });
        }
        self.unspent_skill_points -= required;
        Ok(())
    }

    pub(crate) fn from_saved_parts(
        experience: u64,
        level: u16,
        skill_points_earned: u32,
        unspent_skill_points: u32,
        claimed_one_time_rewards: impl IntoIterator<Item = RewardKey>,
        rules: &ProgressionRules,
    ) -> Result<Self, RunProgressionRestoreError> {
        let expected_level = rules.curve.level_for_experience(experience);
        if level != expected_level {
            return Err(RunProgressionRestoreError::LevelMismatch {
                saved: level,
                expected: expected_level,
            });
        }
        if unspent_skill_points > skill_points_earned {
            return Err(RunProgressionRestoreError::UnspentPointsExceedEarned {
                unspent: unspent_skill_points,
                earned: skill_points_earned,
            });
        }
        let expected_skill_points = u32::from(rules.starting_skill_points).saturating_add(
            u32::from(level.saturating_sub(1))
                .saturating_mul(u32::from(rules.skill_points_per_level)),
        );
        if skill_points_earned != expected_skill_points {
            return Err(RunProgressionRestoreError::EarnedSkillPointsMismatch {
                saved: skill_points_earned,
                expected: expected_skill_points,
            });
        }
        let mut claimed = BTreeSet::new();
        for key in claimed_one_time_rewards {
            if !claimed.insert(key.clone()) {
                return Err(RunProgressionRestoreError::DuplicateRewardKey(key));
            }
        }
        Ok(Self {
            experience,
            level,
            skill_points_earned,
            unspent_skill_points,
            claimed_one_time_rewards: claimed,
        })
    }

    pub fn has_claimed(&self, key: &RewardKey) -> bool {
        self.claimed_one_time_rewards.contains(key)
    }

    pub fn award(
        &mut self,
        award: &ExperienceAward,
        rules: &ProgressionRules,
    ) -> ExperienceAwardOutcome {
        let previous_total = self.experience;
        if let Some(key) = award.one_time_key()
            && !self.claimed_one_time_rewards.insert(key.clone())
        {
            return ExperienceAwardOutcome {
                awarded_experience: 0,
                previous_total,
                total: previous_total,
                level_gains: Vec::new(),
                duplicate_one_time_reward: true,
            };
        }

        self.experience = self.experience.saturating_add(award.amount());
        let new_level = rules.curve.level_for_experience(self.experience);
        let level_gains: Vec<LevelGain> = (self.level.saturating_add(1)..=new_level)
            .map(|level| LevelGain {
                level,
                skill_points_awarded: rules.skill_points_per_level,
            })
            .collect();
        let total_skill_points = u32::from(rules.skill_points_per_level)
            .saturating_mul(u32::try_from(level_gains.len()).unwrap_or(u32::MAX));
        self.unspent_skill_points = self.unspent_skill_points.saturating_add(total_skill_points);
        self.skill_points_earned = self.skill_points_earned.saturating_add(total_skill_points);
        self.level = new_level;

        ExperienceAwardOutcome {
            awarded_experience: self.experience.saturating_sub(previous_total),
            previous_total,
            total: self.experience,
            level_gains,
            duplicate_one_time_reward: false,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SkillPointSpendError {
    pub required: u16,
    pub available: u32,
}

impl Display for SkillPointSpendError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "requires {} skill points but only {} are available",
            self.required, self.available
        )
    }
}

impl Error for SkillPointSpendError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RunProgressionRestoreError {
    LevelMismatch { saved: u16, expected: u16 },
    UnspentPointsExceedEarned { unspent: u32, earned: u32 },
    EarnedSkillPointsMismatch { saved: u32, expected: u32 },
    DuplicateRewardKey(RewardKey),
}

impl Display for RunProgressionRestoreError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LevelMismatch { saved, expected } => write!(
                formatter,
                "saved level {saved} does not match level {expected} derived from experience"
            ),
            Self::UnspentPointsExceedEarned { unspent, earned } => write!(
                formatter,
                "saved unspent skill points {unspent} exceed earned points {earned}"
            ),
            Self::EarnedSkillPointsMismatch { saved, expected } => write!(
                formatter,
                "saved earned skill points {saved} do not match {expected} expected from progression rules"
            ),
            Self::DuplicateRewardKey(key) => {
                write!(formatter, "duplicate claimed reward key '{}'", key.as_str())
            }
        }
    }
}

impl Error for RunProgressionRestoreError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_rules() -> ProgressionRules {
        ProgressionRules {
            curve: ExperienceCurve::new(vec![10, 25, 50])
                .unwrap_or_else(|error| panic!("valid curve rejected: {error}")),
            starting_skill_points: 0,
            skill_points_per_level: 2,
            trivial_threat_level_gap: 3,
            trivial_reward_percent: 0,
            summoned_reward_percent: 0,
            fabricated_reward_percent: 0,
        }
    }

    fn reward_key(value: &str) -> RewardKey {
        RewardKey::new(value).unwrap_or_else(|error| panic!("valid reward key rejected: {error}"))
    }

    #[test]
    fn one_award_can_cross_multiple_level_thresholds() {
        let mut progression = RunProgression::default();

        let outcome = progression.award(&ExperienceAward::repeatable(27), &test_rules());

        assert_eq!(progression.experience(), 27);
        assert_eq!(progression.level(), 3);
        assert_eq!(progression.unspent_skill_points(), 4);
        assert_eq!(
            outcome.level_gains,
            vec![
                LevelGain {
                    level: 2,
                    skill_points_awarded: 2,
                },
                LevelGain {
                    level: 3,
                    skill_points_awarded: 2,
                },
            ]
        );
    }

    #[test]
    fn configured_starting_points_and_failed_spending_preserve_the_budget() {
        let mut progression = RunProgression::with_starting_skill_points(2);

        assert_eq!(progression.skill_points_earned(), 2);
        assert_eq!(progression.unspent_skill_points(), 2);
        assert_eq!(
            progression.spend_skill_points(3),
            Err(SkillPointSpendError {
                required: 3,
                available: 2,
            })
        );
        assert_eq!(progression.unspent_skill_points(), 2);
        progression
            .spend_skill_points(1)
            .unwrap_or_else(|error| panic!("available point could not be spent: {error}"));
        assert_eq!(progression.skill_points_earned(), 2);
        assert_eq!(progression.unspent_skill_points(), 1);
    }

    #[test]
    fn stable_one_time_reward_cannot_be_claimed_twice() {
        let mut progression = RunProgression::default();
        let award = ExperienceAward::one_time(12, reward_key("location:archive/first-entry"));

        let first = progression.award(&award, &test_rules());
        let duplicate = progression.award(&award, &test_rules());

        assert_eq!(first.awarded_experience, 12);
        assert_eq!(duplicate.awarded_experience, 0);
        assert!(duplicate.duplicate_one_time_reward);
        assert_eq!(progression.experience(), 12);
        assert!(
            progression.has_claimed(
                award
                    .one_time_key()
                    .unwrap_or_else(|| { panic!("one-time test award unexpectedly has no key") })
            )
        );
    }

    #[test]
    fn trivial_and_non_persistent_enemies_cannot_be_farmed_by_default() {
        let rules = test_rules();

        assert_eq!(
            rules.experience_for_defeat(5, DefeatReward::persistent(20, 2)),
            0
        );
        assert_eq!(
            rules.experience_for_defeat(1, DefeatReward::summoned(20, 8)),
            0
        );
        assert_eq!(
            rules.experience_for_defeat(1, DefeatReward::fabricated(20, 8)),
            0
        );
        assert_eq!(
            rules.experience_for_defeat(1, DefeatReward::persistent(20, 2)),
            20
        );
    }

    #[test]
    fn a_mod_can_replace_origin_reward_rates_without_replacing_the_algorithm() {
        let rules = ProgressionRules {
            summoned_reward_percent: 25,
            fabricated_reward_percent: 50,
            ..test_rules()
        };

        assert_eq!(
            rules.experience_for_defeat(1, DefeatReward::summoned(20, 2)),
            5
        );
        assert_eq!(
            rules.experience_for_defeat(1, DefeatReward::fabricated(20, 2)),
            10
        );
    }

    #[test]
    fn invalid_curves_are_rejected_before_a_run_starts() {
        assert_eq!(
            ExperienceCurve::new(vec![10, 10]),
            Err(ExperienceCurveError::NotStrictlyIncreasing)
        );
        assert_eq!(
            ExperienceCurve::new(vec![0, 10]),
            Err(ExperienceCurveError::ZeroThreshold)
        );
    }
}
