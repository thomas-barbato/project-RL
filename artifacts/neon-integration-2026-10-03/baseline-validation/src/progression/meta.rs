use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StartingProtocolId(String);

impl StartingProtocolId {
    pub fn new(value: impl Into<String>) -> Result<Self, StartingProtocolIdError> {
        let value = value.into();
        if value.is_empty() {
            return Err(StartingProtocolIdError::Empty);
        }
        if !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b':'))
        {
            return Err(StartingProtocolIdError::InvalidCharacter);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartingProtocolIdError {
    Empty,
    InvalidCharacter,
}

impl Display for StartingProtocolIdError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(formatter, "starting protocol ID must not be empty"),
            Self::InvalidCharacter => write!(
                formatter,
                "starting protocol ID contains a character that is not stable across content files"
            ),
        }
    }
}

impl Error for StartingProtocolIdError {}

/// Persistent horizontal unlocks. This object deliberately does not own XP or levels.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MetaProgression {
    unlocked_starting_protocols: BTreeSet<StartingProtocolId>,
}

impl MetaProgression {
    pub fn unlock_starting_protocol(&mut self, protocol: StartingProtocolId) -> bool {
        self.unlocked_starting_protocols.insert(protocol)
    }

    pub fn is_starting_protocol_unlocked(&self, protocol: &StartingProtocolId) -> bool {
        self.unlocked_starting_protocols.contains(protocol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::progression::{ExperienceAward, ProgressionRules, RunProgression};

    #[test]
    fn restarting_a_run_resets_xp_but_preserves_separate_meta_unlocks() {
        let protocol = StartingProtocolId::new("core:infiltrator")
            .unwrap_or_else(|error| panic!("valid protocol ID rejected: {error}"));
        let mut meta = MetaProgression::default();
        assert!(meta.unlock_starting_protocol(protocol.clone()));

        let mut first_run = RunProgression::default();
        first_run.award(
            &ExperienceAward::repeatable(50),
            &ProgressionRules::default(),
        );
        let restarted_run = RunProgression::default();

        assert!(first_run.experience() > 0);
        assert_eq!(restarted_run.experience(), 0);
        assert_eq!(restarted_run.level(), 1);
        assert!(meta.is_starting_protocol_unlocked(&protocol));
    }
}
