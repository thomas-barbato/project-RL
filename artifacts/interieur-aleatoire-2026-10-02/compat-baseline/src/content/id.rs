use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct PackageId(String);

impl PackageId {
    pub fn new(value: impl Into<String>) -> Result<Self, PackageIdError> {
        let value = value.into();
        validate_segment(&value).map_err(PackageIdError::InvalidSegment)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for PackageId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl FromStr for PackageId {
    type Err = PackageIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PackageIdError {
    InvalidSegment(IdSegmentError),
}

impl Display for PackageIdError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSegment(error) => write!(formatter, "invalid package ID: {error}"),
        }
    }
}

impl Error for PackageIdError {}

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct ContentId {
    raw: String,
    namespace: PackageId,
    name: String,
}

impl ContentId {
    pub fn new(namespace: PackageId, name: impl Into<String>) -> Result<Self, ContentIdError> {
        let name = name.into();
        validate_segment(&name).map_err(ContentIdError::InvalidName)?;
        let raw = format!("{}:{name}", namespace.as_str());
        Ok(Self {
            raw,
            namespace,
            name,
        })
    }

    pub fn as_str(&self) -> &str {
        &self.raw
    }

    pub const fn namespace(&self) -> &PackageId {
        &self.namespace
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Display for ContentId {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.raw)
    }
}

impl FromStr for ContentId {
    type Err = ContentIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let Some((namespace, name)) = value.split_once(':') else {
            return Err(ContentIdError::MissingNamespace);
        };
        if name.contains(':') {
            return Err(ContentIdError::TooManySeparators);
        }
        let namespace = PackageId::new(namespace).map_err(ContentIdError::InvalidNamespace)?;
        Self::new(namespace, name)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContentIdError {
    MissingNamespace,
    TooManySeparators,
    InvalidNamespace(PackageIdError),
    InvalidName(IdSegmentError),
}

impl Display for ContentIdError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingNamespace => write!(formatter, "content ID must use 'namespace:name'"),
            Self::TooManySeparators => {
                write!(formatter, "content ID must contain one ':' separator")
            }
            Self::InvalidNamespace(error) => {
                write!(formatter, "invalid content namespace: {error}")
            }
            Self::InvalidName(error) => write!(formatter, "invalid content name: {error}"),
        }
    }
}

impl Error for ContentIdError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdSegmentError {
    Empty,
    InvalidCharacter,
}

impl Display for IdSegmentError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(formatter, "segment must not be empty"),
            Self::InvalidCharacter => write!(
                formatter,
                "segments may only contain lowercase ASCII letters, digits, '_', '-', or '.'"
            ),
        }
    }
}

fn validate_segment(value: &str) -> Result<(), IdSegmentError> {
    if value.is_empty() {
        return Err(IdSegmentError::Empty);
    }
    if !value.bytes().all(|byte| {
        byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'_' | b'-' | b'.')
    }) {
        return Err(IdSegmentError::InvalidCharacter);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_id_exposes_typed_namespace_and_name() {
        let id: ContentId = "alice.expanded_arsenal:hellfire_m12"
            .parse()
            .unwrap_or_else(|error| panic!("valid content ID rejected: {error}"));

        assert_eq!(id.namespace().as_str(), "alice.expanded_arsenal");
        assert_eq!(id.name(), "hellfire_m12");
        assert_eq!(id.as_str(), "alice.expanded_arsenal:hellfire_m12");
    }

    #[test]
    fn malformed_or_non_canonical_ids_are_rejected() {
        assert_eq!(
            "missing_namespace".parse::<ContentId>(),
            Err(ContentIdError::MissingNamespace)
        );
        assert!("core:Uppercase".parse::<ContentId>().is_err());
        assert!("core:bad/path".parse::<ContentId>().is_err());
    }
}
