use crate::status::StatusId;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApplyStatusEffect {
    status: StatusId,
    stacks: u16,
}

impl ApplyStatusEffect {
    pub fn new(status: StatusId, stacks: u16) -> Result<Self, ApplyStatusEffectError> {
        if stacks == 0 {
            return Err(ApplyStatusEffectError::ZeroStacks);
        }
        Ok(Self { status, stacks })
    }

    pub const fn status(&self) -> &StatusId {
        &self.status
    }

    pub const fn stacks(&self) -> u16 {
        self.stacks
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApplyStatusEffectError {
    ZeroStacks,
}

impl Display for ApplyStatusEffectError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroStacks => write!(formatter, "applied status stacks must be positive"),
        }
    }
}

impl Error for ApplyStatusEffectError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_stack_application_is_rejected_as_invalid_content() {
        let status = "core:test"
            .parse()
            .unwrap_or_else(|error| panic!("valid status ID rejected: {error}"));

        assert_eq!(
            ApplyStatusEffect::new(status, 0),
            Err(ApplyStatusEffectError::ZeroStacks)
        );
    }
}
