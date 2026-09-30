use std::error::Error;
use std::fmt::{Display, Formatter};

use semver::{Version, VersionReq};
use serde::Deserialize;

use super::{PackageId, PackageIdError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DependencySpec {
    pub package: PackageId,
    pub version: VersionReq,
}

impl DependencySpec {
    pub fn parse(value: &str) -> Result<Self, ManifestError> {
        let value = value.trim();
        let boundary = value.find(char::is_whitespace).unwrap_or(value.len());
        let (package, requirement) = value.split_at(boundary);
        let package = PackageId::new(package).map_err(ManifestError::PackageId)?;
        let requirement = requirement.trim();
        let version = if requirement.is_empty() {
            VersionReq::STAR
        } else {
            VersionReq::parse(requirement).map_err(|error| {
                ManifestError::DependencyVersionRequirement {
                    value: value.to_owned(),
                    explanation: error.to_string(),
                }
            })?
        };
        Ok(Self { package, version })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackageManifest {
    pub id: PackageId,
    pub name: String,
    pub version: Version,
    pub author: String,
    pub game_version: VersionReq,
    pub dependencies: Vec<DependencySpec>,
    pub optional_dependencies: Vec<DependencySpec>,
    pub incompatible: Vec<DependencySpec>,
}

impl PackageManifest {
    pub fn from_toml(source: &str) -> Result<Self, ManifestError> {
        let raw: RawPackageManifest =
            toml::from_str(source).map_err(|error| ManifestError::Toml(error.to_string()))?;
        let id = PackageId::new(raw.id).map_err(ManifestError::PackageId)?;
        let version = Version::parse(&raw.version).map_err(|error| ManifestError::Version {
            field: "version",
            value: raw.version,
            explanation: error.to_string(),
        })?;
        let game_version =
            VersionReq::parse(&raw.game_version).map_err(|error| ManifestError::Version {
                field: "game_version",
                value: raw.game_version,
                explanation: error.to_string(),
            })?;
        Ok(Self {
            id,
            name: raw.name,
            version,
            author: raw.author,
            game_version,
            dependencies: parse_dependencies(raw.dependencies)?,
            optional_dependencies: parse_dependencies(raw.optional_dependencies)?,
            incompatible: parse_dependencies(raw.incompatible)?,
        })
    }
}

fn parse_dependencies(values: Vec<String>) -> Result<Vec<DependencySpec>, ManifestError> {
    values
        .iter()
        .map(|dependency| DependencySpec::parse(dependency))
        .collect()
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPackageManifest {
    id: String,
    name: String,
    version: String,
    author: String,
    game_version: String,
    #[serde(default)]
    dependencies: Vec<String>,
    #[serde(default)]
    optional_dependencies: Vec<String>,
    #[serde(default)]
    incompatible: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ManifestError {
    Toml(String),
    PackageId(PackageIdError),
    Version {
        field: &'static str,
        value: String,
        explanation: String,
    },
    DependencyVersionRequirement {
        value: String,
        explanation: String,
    },
}

impl Display for ManifestError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Toml(explanation) => write!(formatter, "invalid TOML manifest: {explanation}"),
            Self::PackageId(error) => write!(formatter, "{error}"),
            Self::Version {
                field,
                value,
                explanation,
            } => write!(
                formatter,
                "invalid semantic version in '{field}' ('{value}'): {explanation}"
            ),
            Self::DependencyVersionRequirement { value, explanation } => write!(
                formatter,
                "invalid dependency requirement '{value}': {explanation}"
            ),
        }
    }
}

impl Error for ManifestError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manifest_uses_semantic_versions_and_dependency_requirements() {
        let manifest = PackageManifest::from_toml(
            r#"
                id = "alice.expanded_arsenal"
                name = "Expanded Arsenal"
                version = "1.2.0"
                author = "Alice"
                game_version = ">=0.1.0, <0.2.0"
                dependencies = ["core >=0.1.0"]
            "#,
        )
        .unwrap_or_else(|error| panic!("valid manifest rejected: {error}"));

        assert_eq!(manifest.id.as_str(), "alice.expanded_arsenal");
        assert_eq!(manifest.version, Version::new(1, 2, 0));
        assert!(manifest.game_version.matches(&Version::new(0, 1, 0)));
        assert_eq!(manifest.dependencies[0].package.as_str(), "core");
        assert!(
            manifest.dependencies[0]
                .version
                .matches(&Version::new(0, 1, 0))
        );
    }
}
