use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

use semver::Version;
use serde::Deserialize;

use crate::combat::{DamagePacket, DamageType};
use crate::status::{
    StatusCatalog, StatusCatalogError, StatusDefinition, StatusDefinitionError,
    StatusEffectPrimitive, StatusHook, StatusId, StatusStacking, StatusTrigger,
};
use crate::weapon::{
    WeaponCatalog, WeaponCatalogError, WeaponDefinition, WeaponDefinitionError, WeaponId,
};
use crate::world::DistanceMetric;

use super::{
    ContentIdError, ManifestError, PackageId, PackageManifest, PackageResolutionError,
    resolve_package_order,
};

const MAX_MANIFEST_BYTES: u64 = 256 * 1024;
const MAX_DEFINITION_BYTES: u64 = 1024 * 1024;

pub struct ContentLoader;

impl ContentLoader {
    pub fn load(
        package_parent_directories: &[PathBuf],
        game_version: &Version,
    ) -> Result<LoadedContent, ContentLoadError> {
        let packages = discover_packages(package_parent_directories)?;
        if !packages.keys().any(|package| package.as_str() == "core") {
            return Err(ContentLoadError::MissingCorePackage);
        }
        let manifests: Vec<PackageManifest> = packages
            .values()
            .map(|package| package.manifest.clone())
            .collect();
        let order = resolve_package_order(&manifests, game_version)
            .map_err(ContentLoadError::PackageResolution)?;
        let mut statuses = StatusCatalog::default();
        let mut weapons = WeaponCatalog::default();

        for package_id in &order {
            let package = packages
                .get(package_id)
                .ok_or_else(|| ContentLoadError::ResolvedPackageMissing(package_id.clone()))?;
            load_status_definitions(package, &mut statuses)?;
            load_weapon_definitions(package, &mut weapons)?;
        }

        Ok(LoadedContent {
            package_order: order,
            statuses,
            weapons,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadedContent {
    package_order: Vec<PackageId>,
    statuses: StatusCatalog,
    weapons: WeaponCatalog,
}

impl LoadedContent {
    pub fn package_order(&self) -> &[PackageId] {
        &self.package_order
    }

    pub const fn statuses(&self) -> &StatusCatalog {
        &self.statuses
    }

    pub const fn weapons(&self) -> &WeaponCatalog {
        &self.weapons
    }

    pub fn into_statuses(self) -> StatusCatalog {
        self.statuses
    }

    pub fn into_registries(self) -> (StatusCatalog, WeaponCatalog) {
        (self.statuses, self.weapons)
    }
}

struct DiscoveredPackage {
    root: PathBuf,
    canonical_root: PathBuf,
    manifest: PackageManifest,
}

fn discover_packages(
    parent_directories: &[PathBuf],
) -> Result<BTreeMap<PackageId, DiscoveredPackage>, ContentLoadError> {
    let mut packages = BTreeMap::new();
    for parent in parent_directories {
        if !parent.exists() {
            continue;
        }
        let canonical_parent = parent
            .canonicalize()
            .map_err(|error| ContentLoadError::Io {
                path: parent.clone(),
                explanation: error.to_string(),
            })?;
        let entries = fs::read_dir(parent).map_err(|error| ContentLoadError::Io {
            path: parent.clone(),
            explanation: error.to_string(),
        })?;
        let mut roots = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|error| ContentLoadError::Io {
                path: parent.clone(),
                explanation: error.to_string(),
            })?;
            let path = entry.path();
            if path.is_dir() {
                roots.push(path);
            }
        }
        roots.sort();

        for root in roots {
            let manifest_path = root.join("manifest.toml");
            if !manifest_path.is_file() {
                continue;
            }
            let manifest_source = read_limited_utf8(&manifest_path, MAX_MANIFEST_BYTES)?;
            let manifest = PackageManifest::from_toml(&manifest_source).map_err(|error| {
                ContentLoadError::Manifest {
                    path: manifest_path,
                    error,
                }
            })?;
            let canonical_root = root.canonicalize().map_err(|error| ContentLoadError::Io {
                path: root.clone(),
                explanation: error.to_string(),
            })?;
            if !canonical_root.starts_with(&canonical_parent) {
                return Err(ContentLoadError::PathEscapesPackageRoot(root));
            }
            let package_id = manifest.id.clone();
            let discovered = DiscoveredPackage {
                root,
                canonical_root,
                manifest,
            };
            if packages.insert(package_id.clone(), discovered).is_some() {
                return Err(ContentLoadError::DuplicatePackage(package_id));
            }
        }
    }
    Ok(packages)
}

fn load_status_definitions(
    package: &DiscoveredPackage,
    catalog: &mut StatusCatalog,
) -> Result<(), ContentLoadError> {
    for path in definition_paths(package, "status")? {
        let source = read_limited_utf8(&path, MAX_DEFINITION_BYTES)?;
        let raw: RawStatusDefinition =
            json5::from_str(&source).map_err(|error| ContentLoadError::DefinitionSyntax {
                package: package.manifest.id.clone(),
                path: path.clone(),
                definition_kind: "status",
                explanation: error.to_string(),
            })?;
        let id: StatusId = raw
            .id
            .parse()
            .map_err(|error| ContentLoadError::ContentId {
                package: package.manifest.id.clone(),
                path: path.clone(),
                value: raw.id.clone(),
                error,
            })?;
        if id.namespace() != &package.manifest.id {
            return Err(ContentLoadError::NamespaceMismatch {
                package: package.manifest.id.clone(),
                path,
                content: Box::new(id),
            });
        }
        let definition =
            raw.into_runtime(id.clone())
                .map_err(|error| ContentLoadError::StatusDefinition {
                    package: package.manifest.id.clone(),
                    path: path.clone(),
                    content: Box::new(id),
                    error,
                })?;
        catalog
            .register(definition)
            .map_err(|error| ContentLoadError::StatusCatalog {
                package: package.manifest.id.clone(),
                path,
                error: Box::new(error),
            })?;
    }
    Ok(())
}

fn load_weapon_definitions(
    package: &DiscoveredPackage,
    catalog: &mut WeaponCatalog,
) -> Result<(), ContentLoadError> {
    for path in definition_paths(package, "weapons")? {
        let source = read_limited_utf8(&path, MAX_DEFINITION_BYTES)?;
        let raw: RawWeaponDefinition =
            json5::from_str(&source).map_err(|error| ContentLoadError::DefinitionSyntax {
                package: package.manifest.id.clone(),
                path: path.clone(),
                definition_kind: "weapon",
                explanation: error.to_string(),
            })?;
        let id: WeaponId = raw
            .id
            .parse()
            .map_err(|error| ContentLoadError::ContentId {
                package: package.manifest.id.clone(),
                path: path.clone(),
                value: raw.id.clone(),
                error,
            })?;
        if id.namespace() != &package.manifest.id {
            return Err(ContentLoadError::NamespaceMismatch {
                package: package.manifest.id.clone(),
                path,
                content: Box::new(id),
            });
        }
        let definition =
            raw.into_runtime(id.clone())
                .map_err(|error| ContentLoadError::WeaponDefinition {
                    package: package.manifest.id.clone(),
                    path: path.clone(),
                    content: Box::new(id),
                    error,
                })?;
        catalog
            .register(definition)
            .map_err(|error| ContentLoadError::WeaponCatalog {
                package: package.manifest.id.clone(),
                path,
                error: Box::new(error),
            })?;
    }
    Ok(())
}

fn definition_paths(
    package: &DiscoveredPackage,
    directory_name: &str,
) -> Result<Vec<PathBuf>, ContentLoadError> {
    let directory = package.root.join(directory_name);
    if !directory.exists() {
        return Ok(Vec::new());
    }
    let entries = fs::read_dir(&directory).map_err(|error| ContentLoadError::Io {
        path: directory.clone(),
        explanation: error.to_string(),
    })?;
    let mut paths = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|error| ContentLoadError::Io {
            path: directory.clone(),
            explanation: error.to_string(),
        })?;
        let path = entry.path();
        if path
            .extension()
            .is_some_and(|extension| extension == "json5")
        {
            ensure_path_inside_package(&path, &package.canonical_root)?;
            paths.push(path);
        }
    }
    paths.sort();
    Ok(paths)
}

fn ensure_path_inside_package(path: &Path, package_root: &Path) -> Result<(), ContentLoadError> {
    let canonical = path.canonicalize().map_err(|error| ContentLoadError::Io {
        path: path.to_path_buf(),
        explanation: error.to_string(),
    })?;
    if !canonical.starts_with(package_root) {
        return Err(ContentLoadError::PathEscapesPackage(path.to_path_buf()));
    }
    Ok(())
}

fn read_limited_utf8(path: &Path, maximum_bytes: u64) -> Result<String, ContentLoadError> {
    let metadata = path.metadata().map_err(|error| ContentLoadError::Io {
        path: path.to_path_buf(),
        explanation: error.to_string(),
    })?;
    if metadata.len() > maximum_bytes {
        return Err(ContentLoadError::FileTooLarge {
            path: path.to_path_buf(),
            maximum_bytes,
            actual_bytes: metadata.len(),
        });
    }
    fs::read_to_string(path).map_err(|error| ContentLoadError::Io {
        path: path.to_path_buf(),
        explanation: error.to_string(),
    })
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawStatusDefinition {
    id: String,
    duration_turns: Option<u16>,
    stacking: RawStatusStacking,
    #[serde(default)]
    hooks: Vec<RawStatusHook>,
}

impl RawStatusDefinition {
    fn into_runtime(self, id: StatusId) -> Result<StatusDefinition, StatusDefinitionError> {
        let hooks = self
            .hooks
            .into_iter()
            .map(RawStatusHook::into_runtime)
            .collect();
        StatusDefinition::new(id, self.duration_turns, self.stacking.into_runtime(), hooks)
    }
}

#[derive(Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
enum RawStatusStacking {
    Replace,
    RefreshDuration,
    AddStacks {
        maximum_stacks: u16,
        refresh_duration: bool,
    },
}

impl RawStatusStacking {
    const fn into_runtime(self) -> StatusStacking {
        match self {
            Self::Replace => StatusStacking::Replace,
            Self::RefreshDuration => StatusStacking::RefreshDuration,
            Self::AddStacks {
                maximum_stacks,
                refresh_duration,
            } => StatusStacking::AddStacks {
                maximum_stacks,
                refresh_duration,
            },
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawStatusHook {
    trigger: RawStatusTrigger,
    #[serde(default)]
    effects: Vec<RawStatusEffect>,
}

impl RawStatusHook {
    fn into_runtime(self) -> StatusHook {
        StatusHook::new(
            self.trigger.into_runtime(),
            self.effects
                .into_iter()
                .map(RawStatusEffect::into_runtime)
                .collect(),
        )
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawStatusTrigger {
    TurnStart,
    TurnEnd,
    DamageReceived,
    DamageDealt,
    Movement,
    Death,
}

impl RawStatusTrigger {
    const fn into_runtime(self) -> StatusTrigger {
        match self {
            Self::TurnStart => StatusTrigger::TurnStart,
            Self::TurnEnd => StatusTrigger::TurnEnd,
            Self::DamageReceived => StatusTrigger::DamageReceived,
            Self::DamageDealt => StatusTrigger::DamageDealt,
            Self::Movement => StatusTrigger::Movement,
            Self::Death => StatusTrigger::Death,
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawStatusEffect {
    DealDamage {
        amount: u16,
        damage_type: RawDamageType,
        #[serde(default)]
        penetration: u16,
        #[serde(default)]
        multiply_by_stacks: bool,
    },
}

impl RawStatusEffect {
    const fn into_runtime(self) -> StatusEffectPrimitive {
        match self {
            Self::DealDamage {
                amount,
                damage_type,
                penetration,
                multiply_by_stacks,
            } => StatusEffectPrimitive::DealDamage {
                packet: DamagePacket::new(amount, damage_type.into_runtime(), penetration),
                multiply_by_stacks,
            },
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawDamageType {
    Kinetic,
    Piercing,
    Explosive,
    Thermal,
    Electrical,
    Chemical,
    Radiation,
    Corruption,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWeaponDefinition {
    id: String,
    name_key: String,
    description_key: String,
    attack: RawWeaponAttack,
}

impl RawWeaponDefinition {
    fn into_runtime(self, id: WeaponId) -> Result<WeaponDefinition, WeaponDefinitionError> {
        WeaponDefinition::new(
            id,
            self.name_key,
            self.description_key,
            self.attack.into_runtime(),
        )
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWeaponAttack {
    range: u16,
    distance_metric: RawDistanceMetric,
    requires_line_of_sight: bool,
    damage: RawWeaponDamage,
}

impl RawWeaponAttack {
    const fn into_runtime(self) -> crate::combat::AttackProfile {
        crate::combat::AttackProfile::new(
            self.range,
            self.distance_metric.into_runtime(),
            self.requires_line_of_sight,
            self.damage.damage_type.into_runtime(),
            self.damage.amount,
            self.damage.penetration,
        )
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWeaponDamage {
    amount: u16,
    damage_type: RawDamageType,
    #[serde(default)]
    penetration: u16,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawDistanceMetric {
    Chebyshev,
    Euclidean,
}

impl RawDistanceMetric {
    const fn into_runtime(self) -> DistanceMetric {
        match self {
            Self::Chebyshev => DistanceMetric::Chebyshev,
            Self::Euclidean => DistanceMetric::Euclidean,
        }
    }
}

impl RawDamageType {
    const fn into_runtime(self) -> DamageType {
        match self {
            Self::Kinetic => DamageType::Kinetic,
            Self::Piercing => DamageType::Piercing,
            Self::Explosive => DamageType::Explosive,
            Self::Thermal => DamageType::Thermal,
            Self::Electrical => DamageType::Electrical,
            Self::Chemical => DamageType::Chemical,
            Self::Radiation => DamageType::Radiation,
            Self::Corruption => DamageType::Corruption,
        }
    }
}

#[derive(Debug)]
pub enum ContentLoadError {
    MissingCorePackage,
    Io {
        path: PathBuf,
        explanation: String,
    },
    FileTooLarge {
        path: PathBuf,
        maximum_bytes: u64,
        actual_bytes: u64,
    },
    Manifest {
        path: PathBuf,
        error: ManifestError,
    },
    DuplicatePackage(PackageId),
    PackageResolution(PackageResolutionError),
    ResolvedPackageMissing(PackageId),
    DefinitionSyntax {
        package: PackageId,
        path: PathBuf,
        definition_kind: &'static str,
        explanation: String,
    },
    ContentId {
        package: PackageId,
        path: PathBuf,
        value: String,
        error: ContentIdError,
    },
    NamespaceMismatch {
        package: PackageId,
        path: PathBuf,
        content: Box<StatusId>,
    },
    StatusDefinition {
        package: PackageId,
        path: PathBuf,
        content: Box<StatusId>,
        error: StatusDefinitionError,
    },
    StatusCatalog {
        package: PackageId,
        path: PathBuf,
        error: Box<StatusCatalogError>,
    },
    WeaponDefinition {
        package: PackageId,
        path: PathBuf,
        content: Box<WeaponId>,
        error: WeaponDefinitionError,
    },
    WeaponCatalog {
        package: PackageId,
        path: PathBuf,
        error: Box<WeaponCatalogError>,
    },
    PathEscapesPackage(PathBuf),
    PathEscapesPackageRoot(PathBuf),
}

impl Display for ContentLoadError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingCorePackage => write!(
                formatter,
                "core content package is missing; expected a package manifest with id = 'core'"
            ),
            Self::Io { path, explanation } => {
                write!(formatter, "{}: I/O error: {explanation}", path.display())
            }
            Self::FileTooLarge {
                path,
                maximum_bytes,
                actual_bytes,
            } => write!(
                formatter,
                "{}: file is {actual_bytes} bytes; maximum allowed is {maximum_bytes}",
                path.display()
            ),
            Self::Manifest { path, error } => write!(formatter, "{}: {error}", path.display()),
            Self::DuplicatePackage(package) => {
                write!(formatter, "duplicate discovered package '{package}'")
            }
            Self::PackageResolution(error) => write!(formatter, "package resolution: {error}"),
            Self::ResolvedPackageMissing(package) => write!(
                formatter,
                "internal content error: resolved package '{package}' was not discovered"
            ),
            Self::DefinitionSyntax {
                package,
                path,
                definition_kind,
                explanation,
            } => write!(
                formatter,
                "[{package}]\n{}\ninvalid {definition_kind} JSON5: {explanation}",
                path.display()
            ),
            Self::ContentId {
                package,
                path,
                value,
                error,
            } => write!(
                formatter,
                "[{package}]\n{}\ninvalid content ID '{value}': {error}",
                path.display()
            ),
            Self::NamespaceMismatch {
                package,
                path,
                content,
            } => write!(
                formatter,
                "[{package}]\n{}\ncontent ID '{content}' must use namespace '{package}'",
                path.display()
            ),
            Self::StatusDefinition {
                package,
                path,
                content,
                error,
            } => write!(
                formatter,
                "[{package}]\n{}\n{content}\ninvalid status definition: {error}",
                path.display()
            ),
            Self::StatusCatalog {
                package,
                path,
                error,
            } => write!(formatter, "[{package}]\n{}\n{error}", path.display()),
            Self::WeaponDefinition {
                package,
                path,
                content,
                error,
            } => write!(
                formatter,
                "[{package}]\n{}\n{content}\ninvalid weapon definition: {error}",
                path.display()
            ),
            Self::WeaponCatalog {
                package,
                path,
                error,
            } => write!(formatter, "[{package}]\n{}\n{error}", path.display()),
            Self::PathEscapesPackage(path) => write!(
                formatter,
                "{}: resolved path escapes its content package",
                path.display()
            ),
            Self::PathEscapesPackageRoot(path) => write!(
                formatter,
                "{}: package directory resolves outside its configured content/mod root",
                path.display()
            ),
        }
    }
}

impl Error for ContentLoadError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_package_loads_corrosion_from_json5() {
        let content_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content");

        let loaded = ContentLoader::load(&[content_root], &Version::new(0, 1, 0))
            .unwrap_or_else(|error| panic!("core content failed to load: {error}"));
        let corrosion: StatusId = "core:corroded"
            .parse()
            .unwrap_or_else(|error| panic!("valid status ID rejected: {error}"));
        let blade: WeaponId = "core:integrity_blade"
            .parse()
            .unwrap_or_else(|error| panic!("valid weapon ID rejected: {error}"));

        assert_eq!(
            loaded
                .package_order()
                .iter()
                .map(PackageId::as_str)
                .collect::<Vec<_>>(),
            vec!["core"]
        );
        assert!(loaded.statuses().contains(&corrosion));
        assert!(loaded.weapons().get(&blade).is_some());
    }

    #[test]
    fn local_mod_is_ordered_after_core_and_adds_its_weapon() {
        let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        let loaded = ContentLoader::load(
            &[project_root.join("content"), project_root.join("mods")],
            &Version::new(0, 1, 0),
        )
        .unwrap_or_else(|error| panic!("content and local mod failed to load: {error}"));
        let arc_lance: WeaponId = "example.arc_arsenal:arc_lance"
            .parse()
            .unwrap_or_else(|error| panic!("valid weapon ID rejected: {error}"));

        assert_eq!(
            loaded
                .package_order()
                .iter()
                .map(PackageId::as_str)
                .collect::<Vec<_>>(),
            vec!["core", "example.arc_arsenal"]
        );
        assert_eq!(
            loaded
                .weapons()
                .get(&arc_lance)
                .map(|weapon| weapon.attack().damage().damage_type),
            Some(DamageType::Electrical)
        );
    }
}
