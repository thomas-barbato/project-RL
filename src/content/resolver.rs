use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Display, Formatter};

use semver::Version;

use super::{PackageId, PackageManifest};

pub fn resolve_package_order(
    manifests: &[PackageManifest],
    game_version: &Version,
) -> Result<Vec<PackageId>, PackageResolutionError> {
    let mut packages = BTreeMap::new();
    for manifest in manifests {
        if packages.insert(manifest.id.clone(), manifest).is_some() {
            return Err(PackageResolutionError::DuplicatePackage(
                manifest.id.clone(),
            ));
        }
        if !manifest.game_version.matches(game_version) {
            return Err(PackageResolutionError::UnsupportedGameVersion {
                package: manifest.id.clone(),
                required: manifest.game_version.to_string(),
                actual: game_version.clone(),
            });
        }
    }

    let mut dependencies: BTreeMap<PackageId, BTreeSet<PackageId>> = packages
        .keys()
        .cloned()
        .map(|id| (id, BTreeSet::new()))
        .collect();
    for manifest in manifests {
        for dependency in &manifest.dependencies {
            let Some(installed) = packages.get(&dependency.package) else {
                return Err(PackageResolutionError::MissingDependency {
                    package: manifest.id.clone(),
                    dependency: dependency.package.clone(),
                });
            };
            if !dependency.version.matches(&installed.version) {
                return Err(PackageResolutionError::DependencyVersionMismatch {
                    package: manifest.id.clone(),
                    dependency: dependency.package.clone(),
                    required: dependency.version.to_string(),
                    actual: installed.version.clone(),
                });
            }
            dependencies
                .entry(manifest.id.clone())
                .or_default()
                .insert(dependency.package.clone());
        }

        for dependency in &manifest.optional_dependencies {
            let Some(installed) = packages.get(&dependency.package) else {
                continue;
            };
            if !dependency.version.matches(&installed.version) {
                return Err(PackageResolutionError::DependencyVersionMismatch {
                    package: manifest.id.clone(),
                    dependency: dependency.package.clone(),
                    required: dependency.version.to_string(),
                    actual: installed.version.clone(),
                });
            }
            dependencies
                .entry(manifest.id.clone())
                .or_default()
                .insert(dependency.package.clone());
        }

        for incompatible in &manifest.incompatible {
            if let Some(installed) = packages.get(&incompatible.package)
                && incompatible.version.matches(&installed.version)
            {
                return Err(PackageResolutionError::IncompatiblePackages {
                    package: manifest.id.clone(),
                    incompatible: incompatible.package.clone(),
                    actual: installed.version.clone(),
                });
            }
        }
    }

    let mut order = Vec::with_capacity(packages.len());
    let mut resolved = BTreeSet::new();
    while order.len() < packages.len() {
        let next = dependencies.iter().find_map(|(package, required)| {
            (!resolved.contains(package) && required.is_subset(&resolved))
                .then_some(package.clone())
        });
        let Some(next) = next else {
            let packages = dependencies
                .keys()
                .filter(|package| !resolved.contains(*package))
                .cloned()
                .collect();
            return Err(PackageResolutionError::DependencyCycle { packages });
        };
        resolved.insert(next.clone());
        order.push(next);
    }

    Ok(order)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PackageResolutionError {
    DuplicatePackage(PackageId),
    UnsupportedGameVersion {
        package: PackageId,
        required: String,
        actual: Version,
    },
    MissingDependency {
        package: PackageId,
        dependency: PackageId,
    },
    DependencyVersionMismatch {
        package: PackageId,
        dependency: PackageId,
        required: String,
        actual: Version,
    },
    IncompatiblePackages {
        package: PackageId,
        incompatible: PackageId,
        actual: Version,
    },
    DependencyCycle {
        packages: Vec<PackageId>,
    },
}

impl Display for PackageResolutionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicatePackage(package) => {
                write!(formatter, "duplicate package ID '{package}'")
            }
            Self::UnsupportedGameVersion {
                package,
                required,
                actual,
            } => write!(
                formatter,
                "package '{package}' requires game version '{required}', current version is {actual}"
            ),
            Self::MissingDependency {
                package,
                dependency,
            } => write!(
                formatter,
                "package '{package}' requires missing package '{dependency}'"
            ),
            Self::DependencyVersionMismatch {
                package,
                dependency,
                required,
                actual,
            } => write!(
                formatter,
                "package '{package}' requires '{dependency} {required}', installed version is {actual}"
            ),
            Self::IncompatiblePackages {
                package,
                incompatible,
                actual,
            } => write!(
                formatter,
                "package '{package}' is incompatible with '{incompatible}' version {actual}"
            ),
            Self::DependencyCycle { packages } => {
                let ids: Vec<&str> = packages.iter().map(PackageId::as_str).collect();
                write!(
                    formatter,
                    "dependency cycle between packages: {}",
                    ids.join(", ")
                )
            }
        }
    }
}

impl Error for PackageResolutionError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(id: &str, version: &str, dependencies: &[&str]) -> PackageManifest {
        PackageManifest::from_toml(&format!(
            r#"
                id = "{id}"
                name = "{id}"
                version = "{version}"
                author = "test"
                game_version = ">=0.1.0"
                dependencies = [{}]
            "#,
            dependencies
                .iter()
                .map(|dependency| format!("\"{dependency}\""))
                .collect::<Vec<_>>()
                .join(", ")
        ))
        .unwrap_or_else(|error| panic!("valid test manifest rejected: {error}"))
    }

    #[test]
    fn dependencies_are_loaded_before_dependents() {
        let mod_manifest = manifest("alice.mod", "1.0.0", &["core >=0.1.0"]);
        let core = manifest("core", "0.1.0", &[]);

        let order = resolve_package_order(&[mod_manifest, core], &Version::new(0, 1, 0))
            .unwrap_or_else(|error| panic!("valid package graph rejected: {error}"));

        assert_eq!(
            order.iter().map(PackageId::as_str).collect::<Vec<_>>(),
            vec!["core", "alice.mod"]
        );
    }

    #[test]
    fn missing_dependencies_and_cycles_are_reported() {
        let missing = manifest("alice.mod", "1.0.0", &["core >=0.1.0"]);
        assert!(matches!(
            resolve_package_order(&[missing], &Version::new(0, 1, 0)),
            Err(PackageResolutionError::MissingDependency { .. })
        ));

        let first = manifest("first", "1.0.0", &["second"]);
        let second = manifest("second", "1.0.0", &["first"]);
        assert!(matches!(
            resolve_package_order(&[first, second], &Version::new(0, 1, 0)),
            Err(PackageResolutionError::DependencyCycle { .. })
        ));
    }
}
