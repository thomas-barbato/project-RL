use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

use semver::Version;
use serde::Deserialize;

use crate::combat::{AttackArea, ConeAttack, DamagePacket, DamageType};
use crate::effects::{ApplyStatusEffect, GroundEffectSpec};
use crate::facility::{
    FacilityBlueprint, InstallationBlueprint, InstallationCapability, RepairOrderBlueprint,
    SecurityAlarmProfile, SecurityAlarmResponse, WorkerBlueprint, WorkerRole,
};
use crate::item::{
    ItemCatalog, ItemCatalogError, ItemDefinition, ItemDefinitionError, ItemEffect, ItemId,
    ItemKind,
};
use crate::localization::{TextCatalog, TextCatalogError};
use crate::loot::{LootCatalog, LootEntry, LootError, LootTable};
use crate::presentation::{
    TerminalCueStyle, TerminalCueStyleError, TerminalEffectGlyph, VisualCueCatalog,
    VisualCueCatalogError, VisualCueDefinition, VisualCueId,
};
use crate::skills::{
    DisciplineDefinition, SkillCatalog, SkillCatalogError, SkillDefinitionError, TechniqueAction,
    TechniqueDefinition, TechniqueKind,
};
use crate::social::{LocalAlertProfile, WitnessProfile};
use crate::status::{
    StatusCatalog, StatusCatalogError, StatusDefinition, StatusDefinitionError,
    StatusEffectPrimitive, StatusHook, StatusId, StatusStacking, StatusTrigger,
};
use crate::weapon::{
    WeaponCatalog, WeaponCatalogError, WeaponDefinition, WeaponDefinitionError, WeaponEffect,
    WeaponId,
};
use crate::world::DistanceMetric;
use crate::world::generation::{MapValidationRules, RoomsGeneratorConfig};

use super::{
    ContentIdError, ExpeditionCatalog, ExpeditionDefinition, ExpeditionDefinitionError,
    FacilityDefinition, FacilityMaterialSpawn, GeneratedZoneDefinition, ManifestError, PackageId,
    PackageManifest, PackageResolutionError, ZoneDefinition, resolve_package_order,
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
        let mut items = ItemCatalog::default();
        let mut skills = SkillCatalog::default();
        let mut texts = TextCatalog::default();
        let mut loot = LootCatalog::default();
        let mut expeditions = ExpeditionCatalog::default();
        let mut visual_cues = VisualCueCatalog::default();

        for package_id in &order {
            let package = packages
                .get(package_id)
                .ok_or_else(|| ContentLoadError::ResolvedPackageMissing(package_id.clone()))?;
            load_status_definitions(package, &mut statuses)?;
            load_weapon_definitions(package, &statuses, &mut weapons)?;
            load_skill_definitions(package, &mut skills)?;
            load_text_definitions(package, &mut texts)?;
            load_visual_cue_definitions(package, &mut visual_cues)?;
        }
        for package_id in &order {
            let package = packages
                .get(package_id)
                .ok_or_else(|| ContentLoadError::ResolvedPackageMissing(package_id.clone()))?;
            load_item_definitions(package, &weapons, &mut items)?;
        }
        skills
            .validate_structure()
            .map_err(|error| ContentLoadError::SkillCatalogValidation {
                error: Box::new(error),
            })?;

        // Resolve loot only once every package's weapon/item catalog is known.
        for package_id in &order {
            let package = packages
                .get(package_id)
                .ok_or_else(|| ContentLoadError::ResolvedPackageMissing(package_id.clone()))?;
            load_loot_definitions(package, &items, &weapons, &mut loot)?;
        }
        for package_id in &order {
            let package = packages
                .get(package_id)
                .ok_or_else(|| ContentLoadError::ResolvedPackageMissing(package_id.clone()))?;
            load_expedition_definitions(package, &loot, &items, &mut expeditions)?;
        }

        Ok(LoadedContent {
            package_order: order,
            statuses,
            weapons,
            items,
            skills,
            texts,
            loot,
            expeditions,
            visual_cues,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadedContent {
    package_order: Vec<PackageId>,
    statuses: StatusCatalog,
    weapons: WeaponCatalog,
    items: ItemCatalog,
    skills: SkillCatalog,
    texts: TextCatalog,
    loot: LootCatalog,
    expeditions: ExpeditionCatalog,
    visual_cues: VisualCueCatalog,
}

impl LoadedContent {
    pub const fn loot(&self) -> &LootCatalog {
        &self.loot
    }

    pub const fn expeditions(&self) -> &ExpeditionCatalog {
        &self.expeditions
    }

    pub const fn visual_cues(&self) -> &VisualCueCatalog {
        &self.visual_cues
    }

    pub fn package_order(&self) -> &[PackageId] {
        &self.package_order
    }

    pub const fn statuses(&self) -> &StatusCatalog {
        &self.statuses
    }

    pub const fn weapons(&self) -> &WeaponCatalog {
        &self.weapons
    }

    pub const fn items(&self) -> &ItemCatalog {
        &self.items
    }

    pub const fn skills(&self) -> &SkillCatalog {
        &self.skills
    }

    pub const fn texts(&self) -> &TextCatalog {
        &self.texts
    }

    pub fn into_statuses(self) -> StatusCatalog {
        self.statuses
    }

    pub fn into_registries(self) -> (StatusCatalog, WeaponCatalog, ItemCatalog, SkillCatalog) {
        (self.statuses, self.weapons, self.items, self.skills)
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
                content: Box::new(id.clone()),
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
    statuses: &StatusCatalog,
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
        let definition = raw.into_runtime(id.clone(), statuses).map_err(|error| {
            ContentLoadError::WeaponDefinition {
                package: package.manifest.id.clone(),
                path: path.clone(),
                content: Box::new(id),
                error: Box::new(error),
            }
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

fn load_visual_cue_definitions(
    package: &DiscoveredPackage,
    catalog: &mut VisualCueCatalog,
) -> Result<(), ContentLoadError> {
    for path in definition_paths(package, "visuals")? {
        let source = read_limited_utf8(&path, MAX_DEFINITION_BYTES)?;
        let raw: RawVisualCueDefinition =
            json5::from_str(&source).map_err(|error| ContentLoadError::DefinitionSyntax {
                package: package.manifest.id.clone(),
                path: path.clone(),
                definition_kind: "visual cue",
                explanation: error.to_string(),
            })?;
        let id: VisualCueId = parse_content_id(package, &path, &raw.id)?;
        ensure_local_namespace(package, &path, &id)?;
        let terminal =
            raw.terminal
                .into_runtime()
                .map_err(|error| ContentLoadError::VisualCueDefinition {
                    package: package.manifest.id.clone(),
                    path: path.clone(),
                    content: Box::new(id.clone()),
                    error,
                })?;
        catalog
            .register(VisualCueDefinition::new(id, terminal))
            .map_err(|error| ContentLoadError::VisualCueCatalog {
                package: package.manifest.id.clone(),
                path,
                error: Box::new(error),
            })?;
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawVisualCueDefinition {
    id: String,
    terminal: RawTerminalCueStyle,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTerminalCueStyle {
    frames: Vec<RawTerminalEffectGlyph>,
    color: [u8; 4],
    #[serde(default)]
    accent_colors: Vec<[u8; 4]>,
    frame_millis: u16,
    #[serde(default)]
    step_millis: u16,
    #[serde(default)]
    linger_millis: u16,
}

impl RawTerminalCueStyle {
    fn into_runtime(self) -> Result<TerminalCueStyle, TerminalCueStyleError> {
        let mut colors = Vec::with_capacity(1 + self.accent_colors.len());
        colors.push(self.color);
        colors.extend(self.accent_colors);
        TerminalCueStyle::new_with_palette(
            self.frames
                .into_iter()
                .map(RawTerminalEffectGlyph::into_runtime)
                .collect(),
            colors,
            self.frame_millis,
            self.step_millis,
            self.linger_millis,
        )
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawTerminalEffectGlyph {
    Dot,
    Projectile,
    Spark,
    Burst,
    FlameSmall,
    Flame,
    FlameLarge,
    Impact,
    Wave,
    Alarm,
}

impl RawTerminalEffectGlyph {
    const fn into_runtime(self) -> TerminalEffectGlyph {
        match self {
            Self::Dot => TerminalEffectGlyph::Dot,
            Self::Projectile => TerminalEffectGlyph::Projectile,
            Self::Spark => TerminalEffectGlyph::Spark,
            Self::Burst => TerminalEffectGlyph::Burst,
            Self::FlameSmall => TerminalEffectGlyph::FlameSmall,
            Self::Flame => TerminalEffectGlyph::Flame,
            Self::FlameLarge => TerminalEffectGlyph::FlameLarge,
            Self::Impact => TerminalEffectGlyph::Impact,
            Self::Wave => TerminalEffectGlyph::Wave,
            Self::Alarm => TerminalEffectGlyph::Alarm,
        }
    }
}

fn load_item_definitions(
    package: &DiscoveredPackage,
    weapons: &WeaponCatalog,
    catalog: &mut ItemCatalog,
) -> Result<(), ContentLoadError> {
    for path in definition_paths(package, "items")? {
        let source = read_limited_utf8(&path, MAX_DEFINITION_BYTES)?;
        let raw: RawItemDefinition =
            json5::from_str(&source).map_err(|error| ContentLoadError::DefinitionSyntax {
                package: package.manifest.id.clone(),
                path: path.clone(),
                definition_kind: "item",
                explanation: error.to_string(),
            })?;
        let id: ItemId = raw
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
        if weapons.get(&id).is_some() {
            return Err(ContentLoadError::ItemWeaponIdCollision {
                package: package.manifest.id.clone(),
                path,
                content: Box::new(id),
            });
        }
        let definition =
            raw.into_runtime(id.clone())
                .map_err(|error| ContentLoadError::ItemDefinition {
                    package: package.manifest.id.clone(),
                    path: path.clone(),
                    content: Box::new(id),
                    error,
                })?;
        catalog
            .register(definition)
            .map_err(|error| ContentLoadError::ItemCatalog {
                package: package.manifest.id.clone(),
                path,
                error: Box::new(error),
            })?;
    }
    Ok(())
}

fn load_loot_definitions(
    package: &DiscoveredPackage,
    items: &ItemCatalog,
    weapons: &WeaponCatalog,
    catalog: &mut LootCatalog,
) -> Result<(), ContentLoadError> {
    for path in definition_paths(package, "loot")? {
        let source = read_limited_utf8(&path, MAX_DEFINITION_BYTES)?;
        let raw: RawLootTable =
            json5::from_str(&source).map_err(|error| ContentLoadError::DefinitionSyntax {
                package: package.manifest.id.clone(),
                path: path.clone(),
                definition_kind: "loot",
                explanation: error.to_string(),
            })?;
        let parse = |value: &str| parse_content_id(package, &path, value);
        let id = parse(&raw.id)?;
        ensure_local_namespace(package, &path, &id)?;
        let entries = raw
            .entries
            .into_iter()
            .map(|entry| {
                Ok(LootEntry {
                    item: parse(&entry.item)?,
                    weight: entry.weight,
                    minimum_depth: entry.minimum_depth,
                    maximum_depth: entry.maximum_depth,
                    minimum_quantity: entry.quantity[0],
                    maximum_quantity: entry.quantity[1],
                    map_kinds: entry
                        .map_kinds
                        .iter()
                        .map(|value| parse(value))
                        .collect::<Result<_, _>>()?,
                    sources: entry
                        .sources
                        .iter()
                        .map(|value| parse(value))
                        .collect::<Result<_, _>>()?,
                })
            })
            .collect::<Result<Vec<_>, ContentLoadError>>()?;
        let failure = |error| ContentLoadError::LootDefinition {
            package: package.manifest.id.clone(),
            path: path.clone(),
            error: Box::new(error),
        };
        let table = LootTable::new(id, entries).map_err(&failure)?;
        table.validate_items(items, weapons).map_err(&failure)?;
        catalog.register(table).map_err(failure)?;
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawLootTable {
    id: String,
    entries: Vec<RawLootEntry>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawLootEntry {
    item: String,
    weight: u32,
    #[serde(default)]
    minimum_depth: u16,
    maximum_depth: Option<u16>,
    #[serde(default)]
    map_kinds: Vec<String>,
    #[serde(default)]
    sources: Vec<String>,
    #[serde(default = "one_loot_item")]
    quantity: [u16; 2],
}
fn one_loot_item() -> [u16; 2] {
    [1, 1]
}

fn load_expedition_definitions(
    package: &DiscoveredPackage,
    loot: &LootCatalog,
    items: &ItemCatalog,
    catalog: &mut ExpeditionCatalog,
) -> Result<(), ContentLoadError> {
    for path in definition_paths(package, "worlds")? {
        let source = read_limited_utf8(&path, MAX_DEFINITION_BYTES)?;
        let raw: RawExpeditionDefinition =
            json5::from_str(&source).map_err(|error| ContentLoadError::DefinitionSyntax {
                package: package.manifest.id.clone(),
                path: path.clone(),
                definition_kind: "expedition",
                explanation: error.to_string(),
            })?;
        let parse = |value: &str| parse_content_id(package, &path, value);
        let id = parse(&raw.id)?;
        ensure_local_namespace(package, &path, &id)?;
        let resolve_zone = |zone: RawZoneDefinition| -> Result<ZoneDefinition, ContentLoadError> {
            Ok(ZoneDefinition {
                id: parse(&zone.id)?,
                name: zone.name,
                kind: parse(&zone.kind)?,
                depth: zone.depth,
            })
        };
        let destination = GeneratedZoneDefinition {
            zone: resolve_zone(raw.destination)?,
            generator: RoomsGeneratorConfig {
                width: raw.width,
                height: raw.height,
                room_count: raw.rooms,
                minimum_room_width: raw.minimum_room_width,
                maximum_room_width: raw.maximum_room_width,
                minimum_room_height: raw.minimum_room_height,
                maximum_room_height: raw.maximum_room_height,
                placement_attempts: raw.placement_attempts,
                validation: MapValidationRules::default(),
            },
            seed_salt: raw.seed_salt,
            loot_table: raw.loot_table.as_deref().map(parse).transpose()?,
            loot_source: parse(&raw.loot_source)?,
            loot_draws: raw.loot_draws,
        };
        let failure = |error| ContentLoadError::ExpeditionDefinition {
            package: package.manifest.id.clone(),
            path: path.clone(),
            error: Box::new(error),
        };
        let mut definition = ExpeditionDefinition::new(
            id,
            resolve_zone(raw.hub)?,
            destination,
            crate::world::GridPos::new(raw.passage[0], raw.passage[1]),
        )
        .map_err(&failure)?;
        definition = definition
            .with_player_property_take_authorizations(
                raw.player_property_take_authorizations
                    .iter()
                    .map(|owner| parse(owner))
                    .collect::<Result<_, _>>()?,
            )
            .map_err(&failure)?;
        if let Some(facility) = raw.hub_facility {
            let installations = facility
                .installations
                .into_iter()
                .map(|installation| {
                    let id = parse(&installation.id)?;
                    ensure_local_namespace(package, &path, &id)?;
                    let security_alarm_profile = if let Some(alarm) = installation.security_alarm {
                        let responses = alarm
                            .responses
                            .iter()
                            .map(|response| match response {
                                RawSecurityAlarmResponse::LockDoors { actuator } => {
                                    Ok(SecurityAlarmResponse::LockDoors {
                                        actuator: parse(actuator)?,
                                    })
                                }
                            })
                            .collect::<Result<Vec<_>, ContentLoadError>>()?;
                        Some(
                            SecurityAlarmProfile::new(
                                alarm.radius,
                                alarm.distance_metric.into_runtime(),
                                alarm.block_closed_corners,
                                alarm.duration_turns,
                            )
                            .and_then(|profile| profile.with_responses(responses))
                            .map_err(|error| {
                                failure(ExpeditionDefinitionError::InvalidSecurityAlarmProfile(
                                    error,
                                ))
                            })?,
                        )
                    } else {
                        None
                    };
                    Ok(InstallationBlueprint {
                        id,
                        position: grid_position(installation.position),
                        maximum_integrity: installation.maximum_integrity,
                        integrity: installation.integrity,
                        capabilities: installation
                            .capabilities
                            .into_iter()
                            .map(RawInstallationCapability::into_runtime)
                            .collect(),
                        dependencies: installation
                            .dependencies
                            .iter()
                            .map(|dependency| parse(dependency))
                            .collect::<Result<_, _>>()?,
                        security_alarm_profile,
                    })
                })
                .collect::<Result<Vec<_>, ContentLoadError>>()?;
            let workers = facility
                .workers
                .into_iter()
                .map(|worker| {
                    let witness_profile = worker
                        .witness
                        .map(RawWitnessProfile::into_runtime)
                        .transpose()
                        .map_err(|error| {
                            failure(ExpeditionDefinitionError::InvalidWitnessProfile(error))
                        })?;
                    let local_alert_profile = worker
                        .local_alert
                        .map(RawLocalAlertProfile::into_runtime)
                        .transpose()
                        .map_err(|error| {
                            failure(ExpeditionDefinitionError::InvalidLocalAlertProfile(error))
                        })?;
                    Ok(WorkerBlueprint {
                        actor_position: grid_position(worker.position),
                        role: worker.role.into_runtime(),
                        maximum_integrity: worker.maximum_integrity,
                        affiliation: worker.affiliation.as_deref().map(parse).transpose()?,
                        witness_profile,
                        local_alert_profile,
                    })
                })
                .collect::<Result<Vec<_>, ContentLoadError>>()?;
            let repair_orders = facility
                .repair_orders
                .into_iter()
                .map(|order| {
                    let id = parse(&order.id)?;
                    ensure_local_namespace(package, &path, &id)?;
                    Ok(RepairOrderBlueprint {
                        id,
                        target: parse(&order.target)?,
                        required_item: parse(&order.required_item)?,
                        required_quantity: order.required_quantity,
                        work_turns: order.work_turns,
                    })
                })
                .collect::<Result<Vec<_>, ContentLoadError>>()?;
            let materials = facility
                .materials
                .into_iter()
                .map(|material| {
                    Ok(FacilityMaterialSpawn {
                        position: grid_position(material.position),
                        item: parse(&material.item)?,
                        quantity: material.quantity,
                        owner: material.owner.as_deref().map(parse).transpose()?,
                    })
                })
                .collect::<Result<Vec<_>, ContentLoadError>>()?;
            let blueprint = FacilityBlueprint {
                installations,
                depot: parse(&facility.depot)?,
                workers,
                repair_orders,
                maximum_path_search: facility.maximum_path_search,
                owner: facility.owner.as_deref().map(parse).transpose()?,
            };
            definition = definition.with_hub_facility(
                FacilityDefinition::new(blueprint, materials).map_err(&failure)?,
            );
        }
        definition
            .validate_references(loot, items)
            .map_err(&failure)?;
        catalog.register(definition).map_err(failure)?;
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawExpeditionDefinition {
    id: String,
    hub: RawZoneDefinition,
    destination: RawZoneDefinition,
    passage: [i32; 2],
    width: usize,
    height: usize,
    rooms: usize,
    #[serde(default = "default_minimum_room_width")]
    minimum_room_width: usize,
    #[serde(default = "default_maximum_room_width")]
    maximum_room_width: usize,
    #[serde(default = "default_minimum_room_height")]
    minimum_room_height: usize,
    #[serde(default = "default_maximum_room_height")]
    maximum_room_height: usize,
    #[serde(default = "default_placement_attempts")]
    placement_attempts: usize,
    seed_salt: u64,
    loot_table: Option<String>,
    loot_source: String,
    #[serde(default)]
    loot_draws: u16,
    #[serde(default)]
    player_property_take_authorizations: Vec<String>,
    hub_facility: Option<RawFacilityDefinition>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawFacilityDefinition {
    installations: Vec<RawInstallationDefinition>,
    depot: String,
    workers: Vec<RawWorkerDefinition>,
    repair_orders: Vec<RawRepairOrderDefinition>,
    #[serde(default)]
    materials: Vec<RawFacilityMaterial>,
    #[serde(default = "default_facility_path_search")]
    maximum_path_search: usize,
    owner: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawInstallationDefinition {
    id: String,
    position: [i32; 2],
    maximum_integrity: u16,
    integrity: u16,
    capabilities: Vec<RawInstallationCapability>,
    #[serde(default)]
    dependencies: Vec<String>,
    security_alarm: Option<RawSecurityAlarmProfile>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawInstallationCapability {
    PowerRelay,
    DoorActuator { door: [i32; 2] },
    SecuritySensor,
    Storage,
}

impl RawInstallationCapability {
    fn into_runtime(self) -> InstallationCapability {
        match self {
            Self::PowerRelay => InstallationCapability::PowerRelay,
            Self::DoorActuator { door } => InstallationCapability::DoorActuator {
                door: grid_position(door),
            },
            Self::SecuritySensor => InstallationCapability::SecuritySensor,
            Self::Storage => InstallationCapability::Storage,
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWorkerDefinition {
    position: [i32; 2],
    role: RawWorkerRole,
    maximum_integrity: u16,
    affiliation: Option<String>,
    witness: Option<RawWitnessProfile>,
    local_alert: Option<RawLocalAlertProfile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWitnessProfile {
    radius: u16,
    #[serde(default = "default_witness_distance_metric")]
    distance_metric: RawDistanceMetric,
    #[serde(default = "default_block_closed_corners")]
    block_closed_corners: bool,
    #[serde(default = "default_witness_memory_capacity")]
    memory_capacity: u16,
}

impl RawWitnessProfile {
    fn into_runtime(self) -> Result<WitnessProfile, crate::social::WitnessProfileError> {
        WitnessProfile::new(
            self.radius,
            self.distance_metric.into_runtime(),
            self.block_closed_corners,
            self.memory_capacity,
        )
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawLocalAlertProfile {
    duration_turns: u16,
}

impl RawLocalAlertProfile {
    fn into_runtime(self) -> Result<LocalAlertProfile, crate::social::LocalAlertProfileError> {
        LocalAlertProfile::new(self.duration_turns)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSecurityAlarmProfile {
    radius: u16,
    #[serde(default = "default_witness_distance_metric")]
    distance_metric: RawDistanceMetric,
    #[serde(default = "default_block_closed_corners")]
    block_closed_corners: bool,
    duration_turns: u16,
    #[serde(default)]
    responses: Vec<RawSecurityAlarmResponse>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawSecurityAlarmResponse {
    LockDoors { actuator: String },
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawWorkerRole {
    Retriever,
    Technician,
}

impl RawWorkerRole {
    const fn into_runtime(self) -> WorkerRole {
        match self {
            Self::Retriever => WorkerRole::Retriever,
            Self::Technician => WorkerRole::Technician,
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRepairOrderDefinition {
    id: String,
    target: String,
    required_item: String,
    required_quantity: u16,
    work_turns: u16,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawFacilityMaterial {
    position: [i32; 2],
    item: String,
    quantity: u16,
    owner: Option<String>,
}

const fn grid_position(position: [i32; 2]) -> crate::world::GridPos {
    crate::world::GridPos::new(position[0], position[1])
}

const fn default_facility_path_search() -> usize {
    15_000
}

const fn default_witness_distance_metric() -> RawDistanceMetric {
    RawDistanceMetric::Euclidean
}

const fn default_block_closed_corners() -> bool {
    true
}

const fn default_witness_memory_capacity() -> u16 {
    16
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawZoneDefinition {
    id: String,
    name: String,
    kind: String,
    depth: u16,
}

const fn default_minimum_room_width() -> usize {
    7
}
const fn default_maximum_room_width() -> usize {
    13
}
const fn default_minimum_room_height() -> usize {
    6
}
const fn default_maximum_room_height() -> usize {
    10
}
const fn default_placement_attempts() -> usize {
    3000
}

fn load_skill_definitions(
    package: &DiscoveredPackage,
    catalog: &mut SkillCatalog,
) -> Result<(), ContentLoadError> {
    for path in definition_paths(package, "skills")? {
        let source = read_limited_utf8(&path, MAX_DEFINITION_BYTES)?;
        let raw: RawSkillDefinitions =
            json5::from_str(&source).map_err(|error| ContentLoadError::DefinitionSyntax {
                package: package.manifest.id.clone(),
                path: path.clone(),
                definition_kind: "skill",
                explanation: error.to_string(),
            })?;

        for discipline in raw.disciplines {
            let id = parse_content_id(package, &path, &discipline.id)?;
            ensure_local_namespace(package, &path, &id)?;
            let definition = DisciplineDefinition::new(
                id.clone(),
                discipline.name_key,
                discipline.description_key,
            )
            .map_err(|error| ContentLoadError::SkillDefinition {
                package: package.manifest.id.clone(),
                path: path.clone(),
                content: Box::new(id.clone()),
                error,
            })?;
            catalog.register_discipline(definition).map_err(|error| {
                ContentLoadError::SkillCatalog {
                    package: package.manifest.id.clone(),
                    path: path.clone(),
                    error: Box::new(error),
                }
            })?;
        }

        for technique in raw.techniques {
            let id = parse_content_id(package, &path, &technique.id)?;
            ensure_local_namespace(package, &path, &id)?;
            let discipline = parse_content_id(package, &path, &technique.discipline)?;
            let prerequisite = technique
                .prerequisite
                .as_deref()
                .map(|value| parse_content_id(package, &path, value))
                .transpose()?;
            let required_features = technique
                .required_features
                .iter()
                .map(|value| parse_content_id(package, &path, value))
                .collect::<Result<Vec<_>, _>>()?;
            let mut definition = TechniqueDefinition::new(
                id.clone(),
                discipline,
                technique.name_key,
                technique.description_key,
                technique.minimum_rank,
                technique.kind.into_runtime(),
                prerequisite,
                required_features,
            )
            .map_err(|error| ContentLoadError::SkillDefinition {
                package: package.manifest.id.clone(),
                path: path.clone(),
                content: Box::new(id.clone()),
                error,
            })?;
            if let Some(action) = technique.action {
                definition = definition
                    .with_action(action.into_runtime())
                    .map_err(|error| ContentLoadError::SkillDefinition {
                        package: package.manifest.id.clone(),
                        path: path.clone(),
                        content: Box::new(id.clone()),
                        error,
                    })?;
            }
            catalog.register_technique(definition).map_err(|error| {
                ContentLoadError::SkillCatalog {
                    package: package.manifest.id.clone(),
                    path: path.clone(),
                    error: Box::new(error),
                }
            })?;
        }
    }
    Ok(())
}

fn load_text_definitions(
    package: &DiscoveredPackage,
    catalog: &mut TextCatalog,
) -> Result<(), ContentLoadError> {
    for path in definition_paths(package, "locales")? {
        let source = read_limited_utf8(&path, MAX_DEFINITION_BYTES)?;
        let raw: RawTextDefinitions =
            json5::from_str(&source).map_err(|error| ContentLoadError::DefinitionSyntax {
                package: package.manifest.id.clone(),
                path: path.clone(),
                definition_kind: "localized text",
                explanation: error.to_string(),
            })?;
        for (key, value) in raw.entries {
            catalog
                .register(raw.locale.clone(), key, value)
                .map_err(|error| ContentLoadError::TextCatalog {
                    package: package.manifest.id.clone(),
                    path: path.clone(),
                    error,
                })?;
        }
    }
    Ok(())
}

fn parse_content_id(
    package: &DiscoveredPackage,
    path: &Path,
    value: &str,
) -> Result<super::ContentId, ContentLoadError> {
    value.parse().map_err(|error| ContentLoadError::ContentId {
        package: package.manifest.id.clone(),
        path: path.to_path_buf(),
        value: value.to_owned(),
        error,
    })
}

fn ensure_local_namespace(
    package: &DiscoveredPackage,
    path: &Path,
    id: &super::ContentId,
) -> Result<(), ContentLoadError> {
    if id.namespace() != &package.manifest.id {
        return Err(ContentLoadError::NamespaceMismatch {
            package: package.manifest.id.clone(),
            path: path.to_path_buf(),
            content: Box::new(id.clone()),
        });
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
    #[serde(default)]
    effects: Vec<RawWeaponEffect>,
}

impl RawWeaponDefinition {
    fn into_runtime(
        self,
        id: WeaponId,
        statuses: &StatusCatalog,
    ) -> Result<WeaponDefinition, WeaponDefinitionError> {
        let attack = self.attack.into_runtime()?;
        let effects = self
            .effects
            .into_iter()
            .map(|effect| effect.into_runtime(statuses))
            .collect::<Result<Vec<_>, _>>()?;
        WeaponDefinition::new(id, self.name_key, self.description_key, attack)
            .map(|definition| definition.with_effects(effects))
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWeaponAttack {
    range: u16,
    distance_metric: RawDistanceMetric,
    requires_line_of_sight: bool,
    damage: RawWeaponDamage,
    #[serde(default)]
    area: RawAttackArea,
}

impl RawWeaponAttack {
    fn into_runtime(self) -> Result<crate::combat::AttackProfile, WeaponDefinitionError> {
        let attack = crate::combat::AttackProfile::new(
            self.range,
            self.distance_metric.into_runtime(),
            self.requires_line_of_sight,
            self.damage.damage_type.into_runtime(),
            self.damage.amount,
            self.damage.penetration,
        );
        Ok(match self.area {
            RawAttackArea::Single => attack,
            RawAttackArea::Cone {
                narrow_length,
                maximum_half_width,
                widen_every,
            } => attack.with_area(AttackArea::Cone(
                ConeAttack::new(narrow_length, maximum_half_width, widen_every)
                    .map_err(WeaponDefinitionError::InvalidCone)?,
            )),
        })
    }
}

#[derive(Default, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawAttackArea {
    #[default]
    Single,
    Cone {
        narrow_length: u16,
        maximum_half_width: u16,
        widen_every: u16,
    },
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawWeaponEffect {
    ApplyStatus {
        status: String,
        #[serde(default = "one_u16")]
        stacks: u16,
    },
    CreateGroundEffect {
        id: String,
        duration_turns: u16,
        damage_each_turn: RawWeaponDamage,
    },
}

impl RawWeaponEffect {
    fn into_runtime(self, statuses: &StatusCatalog) -> Result<WeaponEffect, WeaponDefinitionError> {
        match self {
            Self::ApplyStatus { status, stacks } => {
                let status: StatusId = status
                    .parse()
                    .map_err(WeaponDefinitionError::InvalidEffectId)?;
                if !statuses.contains(&status) {
                    return Err(WeaponDefinitionError::UnknownStatus(status));
                }
                ApplyStatusEffect::new(status, stacks)
                    .map(WeaponEffect::ApplyStatus)
                    .map_err(WeaponDefinitionError::InvalidStatusEffect)
            }
            Self::CreateGroundEffect {
                id,
                duration_turns,
                damage_each_turn,
            } => {
                let id = id.parse().map_err(WeaponDefinitionError::InvalidEffectId)?;
                GroundEffectSpec::new(
                    id,
                    duration_turns,
                    DamagePacket::new(
                        damage_each_turn.amount,
                        damage_each_turn.damage_type.into_runtime(),
                        damage_each_turn.penetration,
                    ),
                )
                .map(WeaponEffect::CreateGroundEffect)
                .map_err(WeaponDefinitionError::InvalidGroundEffect)
            }
        }
    }
}

const fn one_u16() -> u16 {
    1
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

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawItemDefinition {
    id: String,
    name_key: String,
    description_key: String,
    maximum_stack: u16,
    kind: RawItemKind,
    #[serde(default)]
    effects: Vec<RawItemEffect>,
}

impl RawItemDefinition {
    fn into_runtime(self, id: ItemId) -> Result<ItemDefinition, ItemDefinitionError> {
        ItemDefinition::new(
            id,
            self.name_key,
            self.description_key,
            self.maximum_stack,
            self.kind.into_runtime(),
            self.effects
                .into_iter()
                .map(RawItemEffect::into_runtime)
                .collect(),
        )
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawItemKind {
    Consumable,
    Material,
}

impl RawItemKind {
    const fn into_runtime(self) -> ItemKind {
        match self {
            Self::Consumable => ItemKind::Consumable,
            Self::Material => ItemKind::Material,
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawItemEffect {
    RestoreIntegrity { amount: u16 },
}

impl RawItemEffect {
    const fn into_runtime(self) -> ItemEffect {
        match self {
            Self::RestoreIntegrity { amount } => ItemEffect::RestoreIntegrity { amount },
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSkillDefinitions {
    #[serde(default)]
    disciplines: Vec<RawDisciplineDefinition>,
    #[serde(default)]
    techniques: Vec<RawTechniqueDefinition>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTextDefinitions {
    locale: String,
    entries: BTreeMap<String, String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawDisciplineDefinition {
    id: String,
    name_key: String,
    description_key: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTechniqueDefinition {
    id: String,
    discipline: String,
    name_key: String,
    description_key: String,
    minimum_rank: u8,
    kind: RawTechniqueKind,
    prerequisite: Option<String>,
    #[serde(default)]
    required_features: Vec<String>,
    action: Option<RawTechniqueAction>,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawTechniqueAction {
    AnalyzeTarget {
        range: u16,
    },
    AnalyzeMultipleTargets {
        maximum_targets: u8,
        energy_cost: u16,
    },
    ReadMovementTraces {
        radius: u16,
    },
    AnalyzeNearbyWalls {
        radius: u16,
        maximum_tiles: u8,
    },
    AnalyzeThreat {
        range: u16,
    },
}

impl RawTechniqueAction {
    const fn into_runtime(self) -> TechniqueAction {
        match self {
            Self::AnalyzeTarget { range } => TechniqueAction::AnalyzeTarget { range },
            Self::AnalyzeMultipleTargets {
                maximum_targets,
                energy_cost,
            } => TechniqueAction::AnalyzeMultipleTargets {
                maximum_targets,
                energy_cost,
            },
            Self::ReadMovementTraces { radius } => TechniqueAction::ReadMovementTraces { radius },
            Self::AnalyzeNearbyWalls {
                radius,
                maximum_tiles,
            } => TechniqueAction::AnalyzeNearbyWalls {
                radius,
                maximum_tiles,
            },
            Self::AnalyzeThreat { range } => TechniqueAction::AnalyzeThreat { range },
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawTechniqueKind {
    Action,
    Posture,
    Procedure,
    Behavior,
    Improvement,
}

impl RawTechniqueKind {
    const fn into_runtime(self) -> TechniqueKind {
        match self {
            Self::Action => TechniqueKind::Action,
            Self::Posture => TechniqueKind::Posture,
            Self::Procedure => TechniqueKind::Procedure,
            Self::Behavior => TechniqueKind::Behavior,
            Self::Improvement => TechniqueKind::Improvement,
        }
    }
}

#[derive(Debug)]
pub enum ContentLoadError {
    ExpeditionDefinition {
        package: PackageId,
        path: PathBuf,
        error: Box<ExpeditionDefinitionError>,
    },
    LootDefinition {
        package: PackageId,
        path: PathBuf,
        error: Box<LootError>,
    },
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
        error: Box<WeaponDefinitionError>,
    },
    WeaponCatalog {
        package: PackageId,
        path: PathBuf,
        error: Box<WeaponCatalogError>,
    },
    VisualCueDefinition {
        package: PackageId,
        path: PathBuf,
        content: Box<VisualCueId>,
        error: TerminalCueStyleError,
    },
    VisualCueCatalog {
        package: PackageId,
        path: PathBuf,
        error: Box<VisualCueCatalogError>,
    },
    ItemWeaponIdCollision {
        package: PackageId,
        path: PathBuf,
        content: Box<ItemId>,
    },
    ItemDefinition {
        package: PackageId,
        path: PathBuf,
        content: Box<ItemId>,
        error: ItemDefinitionError,
    },
    ItemCatalog {
        package: PackageId,
        path: PathBuf,
        error: Box<ItemCatalogError>,
    },
    SkillDefinition {
        package: PackageId,
        path: PathBuf,
        content: Box<super::ContentId>,
        error: SkillDefinitionError,
    },
    SkillCatalog {
        package: PackageId,
        path: PathBuf,
        error: Box<SkillCatalogError>,
    },
    SkillCatalogValidation {
        error: Box<SkillCatalogError>,
    },
    TextCatalog {
        package: PackageId,
        path: PathBuf,
        error: TextCatalogError,
    },
    PathEscapesPackage(PathBuf),
    PathEscapesPackageRoot(PathBuf),
}

impl Display for ContentLoadError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpeditionDefinition {
                package,
                path,
                error,
            } => write!(formatter, "[{package}]\n{}\n{error}", path.display()),
            Self::LootDefinition {
                package,
                path,
                error,
            } => write!(formatter, "[{package}]\n{}\n{error}", path.display()),
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
            Self::VisualCueDefinition {
                package,
                path,
                content,
                error,
            } => write!(
                formatter,
                "[{package}]\n{}\n{content}\ninvalid visual cue definition: {error}",
                path.display()
            ),
            Self::VisualCueCatalog {
                package,
                path,
                error,
            } => write!(formatter, "[{package}]\n{}\n{error}", path.display()),
            Self::ItemWeaponIdCollision {
                package,
                path,
                content,
            } => write!(
                formatter,
                "[{package}]\n{}\nitem ID '{content}' is already used by a weapon",
                path.display()
            ),
            Self::ItemDefinition {
                package,
                path,
                content,
                error,
            } => write!(
                formatter,
                "[{package}]\n{}\n{content}\ninvalid item definition: {error}",
                path.display()
            ),
            Self::ItemCatalog {
                package,
                path,
                error,
            } => write!(formatter, "[{package}]\n{}\n{error}", path.display()),
            Self::SkillDefinition {
                package,
                path,
                content,
                error,
            } => write!(
                formatter,
                "[{package}]\n{}\n{content}\ninvalid skill definition: {error}",
                path.display()
            ),
            Self::SkillCatalog {
                package,
                path,
                error,
            } => write!(formatter, "[{package}]\n{}\n{error}", path.display()),
            Self::SkillCatalogValidation { error } => {
                write!(formatter, "invalid resolved skill catalog: {error}")
            }
            Self::TextCatalog {
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
    use crate::content::{ContentId, ExpeditionId};

    #[test]
    fn core_package_loads_external_catalogs() {
        let content_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content");

        let loaded = ContentLoader::load(&[content_root], &Version::new(0, 1, 0))
            .unwrap_or_else(|error| panic!("core content failed to load: {error}"));
        let corrosion: StatusId = "core:corroded"
            .parse()
            .unwrap_or_else(|error| panic!("valid status ID rejected: {error}"));
        let blade: WeaponId = "core:integrity_blade"
            .parse()
            .unwrap_or_else(|error| panic!("valid weapon ID rejected: {error}"));
        let flamethrower: WeaponId = "core:flamethrower"
            .parse()
            .unwrap_or_else(|error| panic!("valid weapon ID rejected: {error}"));
        let repair_patch: ItemId = "core:repair_patch"
            .parse()
            .unwrap_or_else(|error| panic!("valid item ID rejected: {error}"));
        let power_regulator: ItemId = "core:power_regulator"
            .parse()
            .unwrap_or_else(|error| panic!("valid material ID rejected: {error}"));
        let reconnaissance: crate::skills::DisciplineId = "core:reconnaissance"
            .parse()
            .unwrap_or_else(|error| panic!("valid discipline ID rejected: {error}"));
        let multiple_analysis: crate::skills::TechniqueId = "core:rec_09"
            .parse()
            .unwrap_or_else(|error| panic!("valid technique ID rejected: {error}"));
        let target_analysis: crate::skills::TechniqueId = "core:rec_01"
            .parse()
            .unwrap_or_else(|error| panic!("valid technique ID rejected: {error}"));
        let starter_expedition: ExpeditionId = "core:starter_expedition"
            .parse()
            .unwrap_or_else(|error| panic!("valid expedition ID rejected: {error}"));

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
        let flame = loaded
            .weapons()
            .get(&flamethrower)
            .unwrap_or_else(|| panic!("flamethrower definition was not loaded"));
        assert!(matches!(
            flame.attack().area(),
            AttackArea::Cone(cone)
                if cone.narrow_length() == 2
                    && cone.maximum_half_width() == 2
                    && cone.widen_every() == 3
        ));
        assert!(flame.effects().iter().any(|effect| matches!(
            effect,
            WeaponEffect::ApplyStatus(effect) if effect.status().as_str() == "core:burning"
        )));
        assert!(flame.effects().iter().any(|effect| matches!(
            effect,
            WeaponEffect::CreateGroundEffect(effect)
                if effect.id().as_str() == "core:burning_ground"
                    && effect.duration_turns() == 3
        )));
        assert_eq!(
            loaded
                .visual_cues()
                .get(&blade)
                .map(|definition| definition.terminal().frames()),
            Some([TerminalEffectGlyph::Dot, TerminalEffectGlyph::Impact].as_slice())
        );
        assert_eq!(
            loaded
                .visual_cues()
                .get(&flamethrower)
                .map(|definition| definition.terminal().colors()),
            Some([[202, 53, 19, 220], [255, 118, 24, 235], [255, 231, 92, 255],].as_slice())
        );
        assert_eq!(
            loaded
                .visual_cues()
                .get(&flamethrower)
                .map(|definition| definition.terminal().frames()),
            Some(
                [
                    TerminalEffectGlyph::FlameSmall,
                    TerminalEffectGlyph::Flame,
                    TerminalEffectGlyph::FlameLarge,
                ]
                .as_slice()
            )
        );
        assert_eq!(
            loaded
                .items()
                .get(&repair_patch)
                .map(ItemDefinition::maximum_stack),
            Some(3)
        );
        assert_eq!(
            loaded
                .items()
                .get(&power_regulator)
                .map(ItemDefinition::kind),
            Some(ItemKind::Material)
        );
        assert!(loaded.skills().discipline(&reconnaissance).is_some());
        assert_eq!(
            loaded.texts().resolve("fr", "technique.rec_01.name"),
            Some("Analyse de cible")
        );
        assert_eq!(
            loaded
                .skills()
                .technique(&multiple_analysis)
                .and_then(TechniqueDefinition::prerequisite),
            Some(&target_analysis)
        );
        assert_eq!(
            loaded
                .skills()
                .technique(&target_analysis)
                .and_then(TechniqueDefinition::action),
            Some(TechniqueAction::AnalyzeTarget { range: 8 })
        );
        assert_eq!(
            loaded
                .skills()
                .validate(&crate::skills::SkillProgressionRules::default()),
            Ok(())
        );
        let prototype_availability = loaded
            .skills()
            .analyze_discipline(
                &reconnaissance,
                &crate::skills::SystemFeatureSet::default(),
                &[],
                &crate::skills::SkillProgressionRules::default(),
            )
            .unwrap_or_else(|error| panic!("core skill availability failed: {error}"));
        assert!(!prototype_availability.is_open());
        assert_eq!(prototype_availability.available.len(), 4);
        assert_eq!(prototype_availability.complete_paths, 0);
        let traces: crate::skills::SystemFeatureId = "core:traces"
            .parse()
            .unwrap_or_else(|error| panic!("valid feature ID rejected: {error}"));
        let trace_enabled_availability = loaded
            .skills()
            .analyze_discipline(
                &reconnaissance,
                &crate::skills::SystemFeatureSet::new([traces]),
                &[],
                &crate::skills::SkillProgressionRules::default(),
            )
            .unwrap_or_else(|error| panic!("trace-enabled availability failed: {error}"));
        assert!(trace_enabled_availability.is_open());
        assert_eq!(trace_enabled_availability.available.len(), 5);
        let expedition = loaded
            .expeditions()
            .get(&starter_expedition)
            .expect("core starter expedition was not loaded");
        assert_eq!(expedition.hub.id.as_str(), "core:starter_city");
        assert_eq!(
            expedition.destination.zone.id.as_str(),
            "core:industrial_sector"
        );
        assert_eq!(
            expedition
                .destination
                .loot_table
                .as_ref()
                .map(ContentId::as_str),
            Some("core:industrial_floor")
        );
        let facility = expedition
            .hub_facility
            .as_ref()
            .expect("core hub facility was not loaded");
        assert_eq!(facility.blueprint.installations.len(), 4);
        assert_eq!(
            facility
                .blueprint
                .installations
                .iter()
                .find(|installation| installation.id.as_str() == "core:checkpoint_sensor")
                .and_then(|installation| installation.security_alarm_profile.as_ref())
                .map(SecurityAlarmProfile::duration_turns),
            Some(8)
        );
        let alarm = facility
            .blueprint
            .installations
            .iter()
            .find(|installation| installation.id.as_str() == "core:checkpoint_sensor")
            .and_then(|installation| installation.security_alarm_profile.as_ref())
            .unwrap();
        assert!(matches!(
            alarm.responses(),
            [SecurityAlarmResponse::LockDoors { actuator }]
                if actuator.as_str() == "core:service_door_actuator"
        ));
        assert_eq!(facility.blueprint.workers.len(), 2);
        assert_eq!(facility.materials[0].item, power_regulator);
        assert_eq!(
            facility.blueprint.owner.as_ref().map(ContentId::as_str),
            Some("core:maintenance_collective")
        );
        assert_eq!(
            facility.blueprint.workers[0]
                .affiliation
                .as_ref()
                .map(ContentId::as_str),
            Some("core:maintenance_collective")
        );
        assert_eq!(
            facility.blueprint.workers[0]
                .witness_profile
                .map(WitnessProfile::memory_capacity),
            Some(16)
        );
        assert_eq!(
            facility.blueprint.workers[0]
                .local_alert_profile
                .map(LocalAlertProfile::duration_turns),
            Some(8)
        );
        assert_eq!(
            facility.materials[0].owner.as_ref().map(ContentId::as_str),
            Some("core:maintenance_collective")
        );
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
        assert!(loaded.visual_cues().get(&arc_lance).is_some());
        let table = loaded
            .loot()
            .get(&"example.arc_arsenal:industrial_floor".parse().unwrap())
            .unwrap();
        assert!(table.entries().iter().any(|entry| entry.item == arc_lance));
        assert!(
            loaded
                .loot()
                .get(&"core:industrial_floor".parse().unwrap())
                .is_some()
        );
        let arc_expedition: ExpeditionId = "example.arc_arsenal:arc_expedition"
            .parse()
            .expect("valid expedition ID");
        let expedition = loaded
            .expeditions()
            .get(&arc_expedition)
            .expect("mod expedition was not loaded");
        assert_eq!(
            expedition.destination.zone.id.as_str(),
            "example.arc_arsenal:arc_salvage_sector"
        );
        assert_eq!(
            expedition
                .destination
                .loot_table
                .as_ref()
                .map(ContentId::as_str),
            Some("example.arc_arsenal:industrial_floor")
        );
    }

    #[test]
    fn visual_loader_reports_bounded_style_errors_with_package_and_path() {
        let root = std::env::temp_dir().join(format!(
            "project-rl-visual-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let package_root = root.join("test.visuals");
        fs::create_dir_all(package_root.join("visuals")).unwrap();
        fs::write(
            package_root.join("manifest.toml"),
            r#"id = "test.visuals"
name = "Visual test"
version = "0.1.0"
author = "test"
game_version = ">=0.1.0, <0.2.0"
dependencies = ["core >=0.1.0, <0.2.0"]
optional_dependencies = []
incompatible = []
"#,
        )
        .unwrap();
        let path = package_root.join("visuals/bad.json5");
        fs::write(
            &path,
            r#"{
                id:'test.visuals:bad',
                terminal:{frames:[],color:[255,120,40,255],frame_millis:60}
            }"#,
        )
        .unwrap();
        let error = ContentLoader::load(
            &[
                PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content"),
                root.clone(),
            ],
            &Version::new(0, 1, 0),
        )
        .expect_err("empty animation should be rejected");

        assert!(matches!(
            error,
            ContentLoadError::VisualCueDefinition {
                package,
                path: error_path,
                error: TerminalCueStyleError::NoFrames,
                ..
            } if package.as_str() == "test.visuals" && error_path == path
        ));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn expedition_loader_reports_package_and_path_for_bad_content() {
        // Disposable content only; never alter installed definitions or a save.
        let root = std::env::temp_dir().join(format!(
            "project-rl-expedition-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let package_root = root.join("test.world");
        fs::create_dir_all(package_root.join("worlds")).unwrap();
        let manifest = r#"id = "test.world"
name = "World test"
version = "0.1.0"
author = "test"
game_version = ">=0.1.0, <0.2.0"
dependencies = ["core >=0.1.0, <0.2.0"]
optional_dependencies = []
incompatible = []
"#;
        fs::write(package_root.join("manifest.toml"), manifest).unwrap();
        let path = package_root.join("worlds/test.json5");
        let roots = [
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content"),
            root.clone(),
        ];
        let valid = r#"{
            id:'test.world:expedition',
            hub:{id:'core:starter_city',name:'Ville',kind:'core:surface',depth:0},
            destination:{id:'test.world:sector',name:'Secteur',kind:'core:industrial',depth:1},
            passage:[1,2],width:48,height:32,rooms:8,seed_salt:7,
            loot_table:'core:industrial_floor',loot_source:'core:floor',loot_draws:2,
            player_property_take_authorizations:['core:maintenance_collective']
        }"#;
        for (source, expected) in [
            (
                valid.replace("core:industrial_floor", "core:unknown"),
                "unknown loot table",
            ),
            (
                valid.replace("test.world:expedition", "core:foreign"),
                "namespace",
            ),
            (valid.replace("width:48", "width:257"), "budget"),
            (
                valid.replace("passage:[1,2]", "passage:[-1,2]"),
                "non-negative",
            ),
            (
                valid.replace(
                    "loot_table:'core:industrial_floor',loot_source:'core:floor',loot_draws:2",
                    "loot_source:'core:floor',loot_draws:2",
                ),
                "require a loot table",
            ),
            (
                valid.replace("rooms:8", "rooms:0"),
                "invalid rooms generator",
            ),
            (
                valid.replace(
                    "['core:maintenance_collective']",
                    "['core:maintenance_collective','core:maintenance_collective']",
                ),
                "authorizations contain duplicates",
            ),
            (valid.replace("seed_salt:7", "seed_slat:7"), "unknown field"),
        ] {
            fs::write(&path, source).unwrap();
            let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
                .unwrap_err()
                .to_string();
            assert!(
                error.contains("test.world") && error.contains("test.json5"),
                "{error}"
            );
            assert!(error.contains(expected), "Expected {expected}: {error}");
        }
        fs::write(&path, valid).unwrap();
        let loaded = ContentLoader::load(&roots, &Version::new(0, 1, 0)).unwrap();
        let expedition = loaded
            .expeditions()
            .get(&"test.world:expedition".parse().unwrap())
            .unwrap();
        assert_eq!(
            expedition.player_property_take_authorizations()[0].as_str(),
            "core:maintenance_collective"
        );
        fs::write(package_root.join("worlds/duplicate.json5"), valid).unwrap();
        assert!(
            ContentLoader::load(&roots, &Version::new(0, 1, 0))
                .unwrap_err()
                .to_string()
                .contains("duplicate expedition")
        );
        fs::remove_file(package_root.join("worlds/duplicate.json5")).unwrap();
        let facility = r#"{
            id:'test.world:expedition',
            hub:{id:'core:starter_city',name:'Ville',kind:'core:surface',depth:0},
            destination:{id:'test.world:sector',name:'Secteur',kind:'core:industrial',depth:1},
            passage:[1,2],width:48,height:32,rooms:8,seed_salt:7,
            loot_table:'core:industrial_floor',loot_source:'core:floor',loot_draws:2,
            hub_facility:{
                installations:[
                    {id:'test.world:relay',position:[2,2],maximum_integrity:5,integrity:0,capabilities:[{type:'power_relay'}]},
                    {id:'test.world:depot',position:[3,2],maximum_integrity:5,integrity:5,capabilities:[{type:'storage'}]}
                ],
                depot:'test.world:depot',
                workers:[],
                repair_orders:[{id:'test.world:repair',target:'test.world:relay',required_item:'core:unknown_material',required_quantity:1,work_turns:2}],
                materials:[]
            }
        }"#;
        fs::write(&path, facility).unwrap();
        let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
            .unwrap_err()
            .to_string();
        assert!(error.contains("unknown facility item"), "{error}");
        fs::write(
            &path,
            facility.replace("core:unknown_material", "core:repair_patch"),
        )
        .unwrap();
        let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
            .unwrap_err()
            .to_string();
        assert!(error.contains("is not a material"), "{error}");
        let invalid_witness = facility
            .replace("core:unknown_material", "core:power_regulator")
            .replace(
                "workers:[]",
                "workers:[{position:[4,2],role:'retriever',maximum_integrity:5,witness:{radius:0}}]",
            );
        fs::write(&path, invalid_witness).unwrap();
        let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
            .unwrap_err()
            .to_string();
        assert!(error.contains("witness range"), "{error}");
        let invalid_alert = facility
            .replace("core:unknown_material", "core:power_regulator")
            .replace(
                "workers:[]",
                "workers:[{position:[4,2],role:'retriever',maximum_integrity:5,local_alert:{duration_turns:0}}]",
            );
        fs::write(&path, invalid_alert).unwrap();
        let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
            .unwrap_err()
            .to_string();
        assert!(error.contains("local alert duration"), "{error}");
        let invalid_security_alarm = facility
            .replace("core:unknown_material", "core:power_regulator")
            .replace(
                "hub_facility:{",
                "hub_facility:{owner:'test.world:operators',",
            )
            .replacen(
                "capabilities:[{type:'power_relay'}]",
                "capabilities:[{type:'security_sensor'}],security_alarm:{radius:0,duration_turns:8}",
                1,
            );
        fs::write(&path, &invalid_security_alarm).unwrap();
        let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
            .unwrap_err()
            .to_string();
        assert!(error.contains("security sensor range"), "{error}");
        fs::write(
            &path,
            invalid_security_alarm.replace(
                "radius:0",
                "radius:8,responses:[{type:'lock_doors',actuator:'test.world:depot'}]",
            ),
        )
        .unwrap();
        let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("AlarmResponseTargetWithoutDoorActuator"),
            "{error}"
        );
        fs::write(&path, facility.replace("work_turns:2", "work_truns:2")).unwrap();
        let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
            .unwrap_err()
            .to_string();
        assert!(error.contains("unknown field"), "{error}");
        fs::remove_file(path).unwrap();
        fs::remove_file(package_root.join("manifest.toml")).unwrap();
        fs::remove_dir(package_root.join("worlds")).unwrap();
        fs::remove_dir(package_root).unwrap();
        fs::remove_dir(root).unwrap();
    }

    #[test]
    fn loot_loader_reports_package_and_path_for_bad_content() {
        // Disposable content only; never alter installed definitions or a save.
        let root = std::env::temp_dir().join(format!(
            "project-rl-loot-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let package_root = root.join("test.loot");
        fs::create_dir_all(package_root.join("loot")).unwrap();
        let manifest = r#"id = "test.loot"
name = "Loot test"
version = "0.1.0"
author = "test"
game_version = ">=0.1.0, <0.2.0"
dependencies = ["core >=0.1.0, <0.2.0"]
optional_dependencies = []
incompatible = []
"#;
        fs::write(package_root.join("manifest.toml"), manifest).unwrap();
        let path = package_root.join("loot/test.json5");
        let roots = [
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content"),
            root.clone(),
        ];
        for (source, expected) in [
            (
                r#"{id:'test.loot:table',entries:[{item:'core:typo',weight:1}]}"#,
                "unknown loot item",
            ),
            (
                r#"{id:'test.loot:table',entries:[{item:'core:repair_patch',weight:1,quantity:[1,4]}]}"#,
                "stack limit",
            ),
            (
                r#"{id:'test.loot:table',entries:[{item:'core:repair_patch',weight:1,minimum_depth:5,maximum_depth:2}]}"#,
                "range",
            ),
            (
                r#"{id:'test.loot:table',entries:[{item:'core:repair_patch',wieght:1}]}"#,
                "unknown field",
            ),
            (
                r#"{id:'core:foreign',entries:[{item:'core:repair_patch',weight:1}]}"#,
                "namespace",
            ),
            (r#"{id:'test.loot:table',entries:[]}"#, "entries"),
        ] {
            fs::write(&path, source).unwrap();
            let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
                .unwrap_err()
                .to_string();
            assert!(
                error.contains("test.loot") && error.contains("test.json5"),
                "{error}"
            );
            assert!(error.contains(expected), "Expected {expected}: {error}");
        }
        let valid = r#"{id:'test.loot:table',entries:[{item:'core:repair_patch',weight:1}]}"#;
        fs::write(&path, valid).unwrap();
        assert!(ContentLoader::load(&roots, &Version::new(0, 1, 0)).is_ok());
        fs::write(package_root.join("loot/duplicate.json5"), valid).unwrap();
        assert!(
            ContentLoader::load(&roots, &Version::new(0, 1, 0))
                .unwrap_err()
                .to_string()
                .contains("duplicate loot table")
        );
        fs::remove_file(package_root.join("loot/duplicate.json5")).unwrap();
        fs::remove_file(path).unwrap();
        fs::remove_file(package_root.join("manifest.toml")).unwrap();
        fs::remove_dir(package_root.join("loot")).unwrap();
        fs::remove_dir(package_root).unwrap();
        fs::remove_dir(root).unwrap();
    }
}
