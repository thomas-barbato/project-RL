use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::{Path, PathBuf};

use semver::Version;
use serde::Deserialize;

use crate::ai::{AiBehavior, AiProfile};
use crate::character_class::{
    CharacterClassCatalog, CharacterClassCatalogError, CharacterClassDefinition,
    CharacterClassDefinitionError, ClassStartingItem,
};
use crate::combat::{
    AttackArea, AttackDelivery, ConeAttack, DamageComponent, DamageImpact, DamagePacket,
    DamageType, MeleeArc, MeleeArcError,
};
use crate::effects::{
    ApplyStatusEffect, DamageFalloff, DestructionEffect, GroundEffectSpec, RadialDamageEffect,
};
use crate::explosive::{ExplosiveAreaProfile, ExplosivePayloadProfile};
use crate::facility::{
    FacilityBlueprint, InstallationBlueprint, InstallationCapability, RepairOrderBlueprint,
    SecurityAlarmProfile, SecurityAlarmResponse, WorkerBlueprint,
    WorkerInstalledPropertyReportBlueprint, WorkerPropertyReportBlueprint,
    WorkerReportedIncidentResponseBlueprint, WorkerRole,
};
use crate::item::{
    EquipmentProfile, ItemCatalog, ItemCatalogError, ItemDefinition, ItemDefinitionError,
    ItemEffect, ItemId, ItemKind,
};
use crate::localization::{TextCatalog, TextCatalogError};
use crate::loot::{LootCatalog, LootEntry, LootError, LootTable};
use crate::presentation::{
    TerminalCueStyle, TerminalCueStyleError, TerminalEffectGlyph, VisualCueCatalog,
    VisualCueCatalogError, VisualCueDefinition, VisualCueId,
};
use crate::progression::{DefeatReward, ExperienceRewardOrigin};
use crate::skills::{
    DisciplineDefinition, ExplosiveDeployment, ExplosivePlacementTarget, ForcedMovement,
    SecondaryExplosivePayload, SkillCatalog, SkillCatalogError, SkillDefinitionError,
    TechniqueAction, TechniqueActivationCost, TechniqueAttributeRequirement, TechniqueDefinition,
    TechniqueEngagementRequirement, TechniqueImprovement, TechniqueKind, TechniqueMaterialCost,
    TechniqueOnHitEffect, TechniqueTargetRequirement,
};
use crate::social::{LocalAlertProfile, PlayerRelation, PropertyReportChannel, WitnessProfile};
use crate::stats::{PrimaryAttribute, PrimaryAttributes};
use crate::status::{
    StatusCatalog, StatusCatalogError, StatusDefinition, StatusDefinitionError,
    StatusEffectPrimitive, StatusHook, StatusId, StatusModifier, StatusStacking, StatusTransition,
    StatusTrigger,
};
use crate::stealth::SignatureChannel;
use crate::time::{ActionKind, TimeUnits};
use crate::weapon::{
    WeaponCatalog, WeaponCatalogError, WeaponDefinition, WeaponDefinitionError, WeaponEffect,
    WeaponEffectTrigger, WeaponId,
};
use crate::world::generation::{MapValidationRules, RoomsGeneratorConfig};
use crate::world::{DistanceMetric, GridPos, NeighborMode, TerrainPropagationPolicy};

use super::{
    ContentId, ContentIdError, ExpandedWorldDefinition, ExpeditionCatalog, ExpeditionDefinition,
    ExpeditionDefinitionError, FacilityDefinition, FacilityMaterialSpawn, GeneratedZoneDefinition,
    ManifestError, PackageId, PackageManifest, PackageResolutionError, PopulationGroupDefinition,
    RegionBiomeRule, RegionBounds, RegionCoord, RegionDestructibleProfile, RegionLandmarkProfile,
    RegionLootProfile, RegionMapSize, RegionPopulationProfile, RegionPopulationRule,
    RegionSiteEntranceProfile, RegionSiteProfile, RegionSiteSecurityProfile,
    RegionSiteTerminalProfile, RegionTerrain, RegionTerrainProfile, RegionTerrainRule,
    RegionThreatProfile, RegionVerticalLink, RegionalWorldCatalog, RegionalWorldDefinition,
    RegionalWorldError, ZoneDefinition, resolve_package_order,
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
        let package_manifests = order
            .iter()
            .map(|package_id| {
                packages
                    .get(package_id)
                    .map(|package| package.manifest.clone())
                    .ok_or_else(|| ContentLoadError::ResolvedPackageMissing(package_id.clone()))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let mut statuses = StatusCatalog::default();
        let mut weapons = WeaponCatalog::default();
        let mut items = ItemCatalog::default();
        let mut character_classes = CharacterClassCatalog::default();
        let mut skills = SkillCatalog::default();
        let mut texts = TextCatalog::default();
        let mut loot = LootCatalog::default();
        let mut expeditions = ExpeditionCatalog::default();
        let mut regional_worlds = RegionalWorldCatalog::default();
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
        statuses.validate_references().map_err(|error| {
            ContentLoadError::StatusCatalogValidation {
                error: Box::new(error),
            }
        })?;
        for package_id in &order {
            let package = packages
                .get(package_id)
                .ok_or_else(|| ContentLoadError::ResolvedPackageMissing(package_id.clone()))?;
            load_item_definitions(package, &weapons, &mut items)?;
        }
        for package_id in &order {
            let package = packages
                .get(package_id)
                .ok_or_else(|| ContentLoadError::ResolvedPackageMissing(package_id.clone()))?;
            load_character_class_definitions(package, &weapons, &items, &mut character_classes)?;
        }
        skills
            .validate_structure()
            .map_err(|error| ContentLoadError::SkillCatalogValidation {
                error: Box::new(error),
            })?;
        skills
            .validate_status_references(&statuses)
            .map_err(|error| ContentLoadError::SkillCatalogValidation {
                error: Box::new(error),
            })?;
        skills.validate_item_references(&items).map_err(|error| {
            ContentLoadError::SkillCatalogValidation {
                error: Box::new(error),
            }
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
            load_regional_world_definitions(package, &loot, &items, &mut regional_worlds)?;
        }

        Ok(LoadedContent {
            package_order: order,
            package_manifests,
            statuses,
            weapons,
            items,
            character_classes,
            skills,
            texts,
            loot,
            expeditions,
            regional_worlds,
            visual_cues,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LoadedContent {
    package_order: Vec<PackageId>,
    package_manifests: Vec<PackageManifest>,
    statuses: StatusCatalog,
    weapons: WeaponCatalog,
    items: ItemCatalog,
    character_classes: CharacterClassCatalog,
    skills: SkillCatalog,
    texts: TextCatalog,
    loot: LootCatalog,
    expeditions: ExpeditionCatalog,
    regional_worlds: RegionalWorldCatalog,
    visual_cues: VisualCueCatalog,
}

impl LoadedContent {
    pub const fn loot(&self) -> &LootCatalog {
        &self.loot
    }

    pub const fn expeditions(&self) -> &ExpeditionCatalog {
        &self.expeditions
    }

    pub const fn regional_worlds(&self) -> &RegionalWorldCatalog {
        &self.regional_worlds
    }

    pub const fn visual_cues(&self) -> &VisualCueCatalog {
        &self.visual_cues
    }

    pub fn package_order(&self) -> &[PackageId] {
        &self.package_order
    }

    /// Resolved package identities in the exact dependency order used to load
    /// this content set. Save systems can persist this human-readable contract
    /// in addition to their full rules fingerprints.
    pub fn package_manifests(&self) -> &[PackageManifest] {
        &self.package_manifests
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

    pub const fn character_classes(&self) -> &CharacterClassCatalog {
        &self.character_classes
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
        let family = raw
            .family
            .as_deref()
            .map(|value| parse_content_id(package, &path, value))
            .transpose()?;
        let blocked_families = raw
            .blocked_families
            .iter()
            .map(|value| parse_content_id(package, &path, value))
            .collect::<Result<Vec<_>, _>>()?;
        let expiration_transition = if let Some(transition) = &raw.expiration_transition {
            let target = parse_content_id(package, &path, &transition.status)?;
            Some(
                StatusTransition::new(target, transition.stacks).map_err(|error| {
                    ContentLoadError::StatusDefinition {
                        package: package.manifest.id.clone(),
                        path: path.clone(),
                        content: Box::new(id.clone()),
                        error,
                    }
                })?,
            )
        } else {
            None
        };
        let definition = raw
            .into_runtime(id.clone(), family, blocked_families, expiration_transition)
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
        let equipment = raw
            .equipment
            .as_ref()
            .map(|equipment| {
                let slot = parse_content_id(package, &path, &equipment.slot)?;
                EquipmentProfile::new(slot, equipment.armor).map_err(|error| {
                    ContentLoadError::ItemDefinition {
                        package: package.manifest.id.clone(),
                        path: path.clone(),
                        content: Box::new(id.clone()),
                        error,
                    }
                })
            })
            .transpose()?;
        let definition = raw.into_runtime(id.clone(), equipment).map_err(|error| {
            ContentLoadError::ItemDefinition {
                package: package.manifest.id.clone(),
                path: path.clone(),
                content: Box::new(id),
                error,
            }
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

fn load_character_class_definitions(
    package: &DiscoveredPackage,
    weapons: &WeaponCatalog,
    items: &ItemCatalog,
    catalog: &mut CharacterClassCatalog,
) -> Result<(), ContentLoadError> {
    for path in definition_paths(package, "classes")? {
        let source = read_limited_utf8(&path, MAX_DEFINITION_BYTES)?;
        let raw: RawCharacterClassDefinition =
            json5::from_str(&source).map_err(|error| ContentLoadError::DefinitionSyntax {
                package: package.manifest.id.clone(),
                path: path.clone(),
                definition_kind: "character class",
                explanation: error.to_string(),
            })?;
        let id = parse_content_id(package, &path, &raw.id)?;
        ensure_local_namespace(package, &path, &id)?;
        let starting_weapons = raw
            .starting_weapons
            .iter()
            .map(|value| parse_content_id(package, &path, value))
            .collect::<Result<Vec<_>, _>>()?;
        let starting_items = raw
            .starting_items
            .iter()
            .map(|entry| {
                parse_content_id(package, &path, &entry.item)
                    .map(|item| ClassStartingItem::new(item, entry.quantity))
            })
            .collect::<Result<Vec<_>, _>>()?;
        let starting_equipment = raw
            .starting_equipment
            .iter()
            .map(|entry| {
                entry
                    .as_deref()
                    .map(|value| parse_content_id(package, &path, value))
                    .transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;
        let definition = CharacterClassDefinition::new(
            id.clone(),
            raw.name_key,
            raw.role_key,
            raw.description_key,
            raw.recommended_attributes.into_runtime(),
            starting_weapons,
            starting_items,
            starting_equipment,
        )
        .and_then(|definition| {
            definition
                .validate_references(weapons, items)
                .map(|()| definition)
        })
        .map_err(|error| ContentLoadError::CharacterClassDefinition {
            package: package.manifest.id.clone(),
            path: path.clone(),
            content: Box::new(id),
            error: Box::new(error),
        })?;
        catalog
            .register(definition)
            .map_err(|error| ContentLoadError::CharacterClassCatalog {
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

fn load_regional_world_definitions(
    package: &DiscoveredPackage,
    loot_catalog: &LootCatalog,
    item_catalog: &ItemCatalog,
    catalog: &mut RegionalWorldCatalog,
) -> Result<(), ContentLoadError> {
    for path in definition_paths(package, "regional_worlds")? {
        let source = read_limited_utf8(&path, MAX_DEFINITION_BYTES)?;
        let raw: RawRegionalWorld =
            json5::from_str(&source).map_err(|error| ContentLoadError::DefinitionSyntax {
                package: package.manifest.id.clone(),
                path: path.clone(),
                definition_kind: "regional world",
                explanation: error.to_string(),
            })?;
        let parse = |value: &str| parse_content_id(package, &path, value);
        let id = parse(&raw.id)?;
        ensure_local_namespace(package, &path, &id)?;
        let failure = |error| ContentLoadError::RegionalWorldDefinition {
            package: package.manifest.id.clone(),
            path: path.clone(),
            error: Box::new(error),
        };
        let bounds = RegionBounds::new(
            raw.bounds.minimum_x,
            raw.bounds.maximum_x,
            raw.bounds.minimum_y,
            raw.bounds.maximum_y,
            raw.bounds.maximum_depth,
        )
        .map_err(&failure)?;
        let local_map_size =
            RegionMapSize::new(raw.local_map.width, raw.local_map.height).map_err(&failure)?;
        let vertical_links = raw
            .vertical_links
            .iter()
            .map(|link| {
                let coordinate = |raw: [i32; 3]| {
                    let depth = u16::try_from(raw[2])
                        .map_err(|_| failure(RegionalWorldError::InvalidVerticalLink))?;
                    Ok(RegionCoord::new(raw[0], raw[1], depth))
                };
                RegionVerticalLink::new(coordinate(link.upper)?, coordinate(link.lower)?)
                    .map_err(&failure)
            })
            .collect::<Result<Vec<_>, ContentLoadError>>()?;
        let biomes = raw
            .biomes
            .into_iter()
            .map(|biome| {
                let id = parse(&biome.id)?;
                let features = biome
                    .terrain
                    .features
                    .into_iter()
                    .map(|feature| {
                        RegionTerrainRule::new(feature.kind.into_runtime(), feature.weight)
                            .map_err(&failure)
                    })
                    .collect::<Result<Vec<_>, ContentLoadError>>()?;
                let terrain = RegionTerrainProfile::new(
                    biome.terrain.ground.into_runtime(),
                    biome.terrain.patch_count,
                    biome.terrain.minimum_patch_radius,
                    biome.terrain.maximum_patch_radius,
                    features,
                )
                .map_err(&failure)?;
                let population_rules = biome
                    .population
                    .groups
                    .into_iter()
                    .map(|rule| {
                        let tags = rule.tags.iter().map(|tag| parse(tag)).collect::<Result<
                            Vec<_>,
                            ContentLoadError,
                        >>(
                        )?;
                        rule.into_runtime(tags).map_err(&failure)
                    })
                    .collect::<Result<Vec<_>, ContentLoadError>>()?;
                let population = RegionPopulationProfile::new(
                    biome.population.group_rolls[0],
                    biome.population.group_rolls[1],
                    population_rules,
                )
                .map_err(&failure)?;
                let encounter_rules = biome
                    .encounters
                    .groups
                    .into_iter()
                    .map(|rule| {
                        let tags = rule.tags.iter().map(|tag| parse(tag)).collect::<Result<
                            Vec<_>,
                            ContentLoadError,
                        >>(
                        )?;
                        rule.into_runtime(tags).map_err(&failure)
                    })
                    .collect::<Result<Vec<_>, ContentLoadError>>()?;
                let encounters = RegionPopulationProfile::new(
                    biome.encounters.group_rolls[0],
                    biome.encounters.group_rolls[1],
                    encounter_rules,
                )
                .map_err(&failure)?;
                let loot = biome
                    .loot
                    .map(|loot| {
                        let table = parse(&loot.table)?;
                        if loot_catalog.get(&table).is_none() {
                            return Err(failure(RegionalWorldError::UnknownLootTable(table)));
                        }
                        RegionLootProfile::new(
                            table,
                            parse(&loot.source)?,
                            loot.draws[0],
                            loot.draws[1],
                        )
                        .map_err(&failure)
                    })
                    .transpose()?;
                let landmarks = RegionLandmarkProfile::new(
                    biome.landmarks.caches[0],
                    biome.landmarks.caches[1],
                    biome.landmarks.threat_camps[0],
                    biome.landmarks.threat_camps[1],
                    biome.landmarks.minimum_passage_distance,
                )
                .map_err(&failure)?;
                let sites = RegionSiteProfile::new(
                    biome.sites.compounds[0],
                    biome.sites.compounds[1],
                    biome.sites.width,
                    biome.sites.height,
                )
                .map_err(&failure)?
                .with_entrances(RegionSiteEntranceProfile::new(
                    biome.sites.entrances.open,
                    biome.sites.entrances.closed_door,
                    biome.sites.entrances.locked_console,
                ))
                .map_err(&failure)?;
                let site_security = biome
                    .sites
                    .security
                    .map(|security| {
                        let mut profile = RegionSiteSecurityProfile::new(
                            parse(&security.owner)?,
                            security.radius,
                            security.distance_metric.into_runtime(),
                            security.block_closed_corners,
                            security.duration_turns,
                            security.reinforcement_delay_turns,
                        )
                        .map_err(&failure)?;
                        if let Some(range) = security.navigation_signal_range {
                            profile = profile
                                .with_navigation_signal_range(range)
                                .map_err(&failure)?;
                        }
                        Ok(profile)
                    })
                    .transpose()?;
                let site_terminals = biome
                    .sites
                    .terminals
                    .map(|terminals| {
                        let records = terminals
                            .records
                            .iter()
                            .map(|record| parse(record))
                            .collect::<Result<Vec<_>, ContentLoadError>>()?;
                        RegionSiteTerminalProfile::new(
                            terminals.count[0],
                            terminals.count[1],
                            records,
                        )
                        .map_err(&failure)
                    })
                    .transpose()?;
                let threats = biome
                    .threats
                    .map(|threats| {
                        let tags = threats.tags.iter().map(|tag| parse(tag)).collect::<Result<
                            Vec<_>,
                            ContentLoadError,
                        >>(
                        )?;
                        threats.into_runtime(tags).map_err(&failure)
                    })
                    .transpose()?;
                let destructibles = biome
                    .destructibles
                    .map(|raw| {
                        let ground_effect = raw
                            .ground_effect
                            .as_ref()
                            .map(|effect| {
                                GroundEffectSpec::new(
                                    parse(&effect.id)?,
                                    effect.duration_turns,
                                    effect.damage_each_turn.into_runtime(),
                                )
                                .map_err(|_| failure(RegionalWorldError::InvalidDestructionEffect))
                            })
                            .transpose()?;
                        raw.into_runtime(ground_effect).map_err(&failure)
                    })
                    .transpose()?;
                let mut runtime = RegionBiomeRule::new(
                    id,
                    biome.weight,
                    biome.minimum_depth,
                    biome.maximum_depth,
                    terrain,
                )
                .map_err(&failure)?
                .with_population(population)
                .with_encounters(encounters)
                .with_landmarks(landmarks)
                .with_sites(sites);
                if let Some(loot) = loot {
                    runtime = runtime.with_loot(loot);
                }
                if let Some(threats) = threats {
                    runtime = runtime.with_threats(threats);
                }
                if let Some(site_security) = site_security {
                    runtime = runtime.with_site_security(site_security);
                }
                if let Some(site_terminals) = site_terminals {
                    runtime = runtime.with_site_terminals(site_terminals);
                }
                if let Some(destructibles) = destructibles {
                    runtime = runtime.with_destructibles(destructibles);
                }
                Ok(runtime)
            })
            .collect::<Result<Vec<_>, ContentLoadError>>()?;
        let cities = raw
            .cities
            .into_iter()
            .map(|city| {
                let city_id = parse(&city.id)?;
                let coordinate_depth = u16::try_from(city.coordinate[2])
                    .map_err(|_| failure(RegionalWorldError::CityOutsideWorld(city_id.clone())))?;
                let city_map_size = city
                    .map
                    .map(|map| RegionMapSize::new(map.width, map.height).map_err(&failure))
                    .transpose()?
                    .unwrap_or(local_map_size);
                let merchant_offers = city
                    .merchant
                    .offers
                    .into_iter()
                    .map(|offer| {
                        Ok(crate::content::MerchantOfferDefinition {
                            item: parse(&offer.item)?,
                            initial_stock: offer.stock,
                            buy_price: offer.buy_price,
                            sell_price: offer.sell_price,
                            minimum_depth: offer.minimum_depth,
                            maximum_depth: offer.maximum_depth,
                        })
                    })
                    .collect::<Result<Vec<_>, ContentLoadError>>()?;
                let merchant_gambles = city
                    .merchant
                    .gambles
                    .into_iter()
                    .map(|gamble| {
                        Ok(crate::content::MerchantGambleDefinition {
                            item: parse(&gamble.item)?,
                            initial_stock: gamble.stock,
                            price: gamble.price,
                        })
                    })
                    .collect::<Result<Vec<_>, ContentLoadError>>()?;
                let merchant = crate::content::MerchantDefinition::new(
                    grid_position(city.merchant.position),
                    city.merchant.maximum_integrity,
                    city.merchant.credits,
                    merchant_offers,
                    merchant_gambles,
                    crate::content::GambleScalingDefinition::new(
                        city.merchant.gamble_scaling.player_levels_per_rank,
                        city.merchant.gamble_scaling.zone_depths_per_rank,
                        city.merchant.gamble_scaling.maximum_rank,
                    )
                    .ok_or_else(|| {
                        failure(RegionalWorldError::InvalidCity(Box::new(
                            ExpeditionDefinitionError::InvalidMerchant,
                        )))
                    })?,
                )
                .map_err(|error| failure(RegionalWorldError::InvalidCity(Box::new(error))))?;
                merchant
                    .validate_references(item_catalog)
                    .map_err(|error| failure(RegionalWorldError::InvalidCity(Box::new(error))))?;
                let clinic = crate::content::ClinicDefinition::new(
                    grid_position(city.clinic.work_position),
                    grid_position(city.clinic.break_position),
                    city.clinic.maximum_integrity,
                    city.clinic.credits,
                    city.clinic.maximum_restoration,
                    city.clinic.price_per_point,
                    city.clinic.work_turns,
                    city.clinic.break_turns,
                    city.clinic.maximum_path_search,
                )
                .map_err(|error| failure(RegionalWorldError::InvalidCity(Box::new(error))))?;
                let residents = city
                    .residents
                    .into_iter()
                    .map(|resident| {
                        crate::content::ResidentDefinition::new(
                            grid_position(resident.residence_position),
                            grid_position(resident.gathering_position),
                            resident.maximum_integrity,
                            resident.residence_turns,
                            resident.gathering_turns,
                            resident.maximum_path_search,
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|error| failure(RegionalWorldError::InvalidCity(Box::new(error))))?;
                crate::content::RegionCityDefinition::new(
                    city_id,
                    RegionCoord::new(city.coordinate[0], city.coordinate[1], coordinate_depth),
                    city.name,
                    parse(&city.kind)?,
                    city_map_size,
                    city.layout.into_runtime(),
                    merchant,
                    clinic,
                    residents,
                )
                .map_err(&failure)
            })
            .collect::<Result<Vec<_>, ContentLoadError>>()?;
        let definition =
            RegionalWorldDefinition::new(id, bounds, raw.province_size, local_map_size, biomes)
                .map_err(&failure)?
                .with_vertical_links(vertical_links)
                .map_err(&failure)?
                .with_cities(cities)
                .map_err(&failure)?;
        catalog.register(definition).map_err(failure)?;
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionalWorld {
    id: String,
    bounds: RawRegionBounds,
    province_size: u16,
    local_map: RawRegionMap,
    #[serde(default)]
    vertical_links: Vec<RawRegionVerticalLink>,
    #[serde(default)]
    cities: Vec<RawRegionCityDefinition>,
    biomes: Vec<RawRegionBiomeRule>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionCityDefinition {
    id: String,
    coordinate: [i32; 3],
    name: String,
    kind: String,
    #[serde(default)]
    map: Option<RawRegionMap>,
    layout: RawRegionCityLayout,
    merchant: RawMerchantDefinition,
    clinic: RawClinicDefinition,
    residents: Vec<RawResidentDefinition>,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawRegionCityLayout {
    MaintenanceSpine,
    CoolantRings,
    DissonantLattice,
    RecursiveBloom,
    ProcessRuin,
}

impl RawRegionCityLayout {
    const fn into_runtime(self) -> crate::content::RegionCityLayout {
        match self {
            Self::MaintenanceSpine => crate::content::RegionCityLayout::MaintenanceSpine,
            Self::CoolantRings => crate::content::RegionCityLayout::CoolantRings,
            Self::DissonantLattice => crate::content::RegionCityLayout::DissonantLattice,
            Self::RecursiveBloom => crate::content::RegionCityLayout::RecursiveBloom,
            Self::ProcessRuin => crate::content::RegionCityLayout::ProcessRuin,
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionVerticalLink {
    upper: [i32; 3],
    lower: [i32; 3],
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionMap {
    width: u16,
    height: u16,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionBounds {
    minimum_x: i32,
    maximum_x: i32,
    minimum_y: i32,
    maximum_y: i32,
    maximum_depth: u16,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionBiomeRule {
    id: String,
    weight: u32,
    #[serde(default)]
    minimum_depth: u16,
    maximum_depth: Option<u16>,
    terrain: RawRegionTerrainProfile,
    #[serde(default)]
    population: RawRegionPopulationProfile,
    #[serde(default)]
    encounters: RawRegionPopulationProfile,
    loot: Option<RawRegionLootProfile>,
    #[serde(default)]
    landmarks: RawRegionLandmarkProfile,
    #[serde(default)]
    sites: RawRegionSiteProfile,
    threats: Option<RawRegionThreatProfile>,
    destructibles: Option<RawRegionDestructibleProfile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionDestructibleProfile {
    count: [u16; 2],
    #[serde(default)]
    minimum_passage_distance: u16,
    maximum_integrity: u16,
    explosion: RawRegionExplosion,
    ground_effect: Option<RawRegionGroundEffect>,
}

impl RawRegionDestructibleProfile {
    fn into_runtime(
        self,
        ground_effect: Option<GroundEffectSpec>,
    ) -> Result<RegionDestructibleProfile, RegionalWorldError> {
        let mut destruction_effect = DestructionEffect::new(RadialDamageEffect {
            maximum_cost: self.explosion.radius,
            neighbor_mode: NeighborMode::CardinalAndDiagonal,
            propagation_policy: TerrainPropagationPolicy {
                floor_cost: Some(self.explosion.floor_cost),
                shallow_water_cost: Some(
                    self.explosion
                        .shallow_water_cost
                        .unwrap_or(self.explosion.floor_cost),
                ),
                deep_water_cost: self.explosion.deep_water_cost,
                wall_cost: None,
            },
            damage: self.explosion.damage.into_runtime(),
            falloff: self
                .explosion
                .falloff_per_cost
                .map_or(DamageFalloff::None, DamageFalloff::PerPropagationCost),
        });
        if let Some(ground_effect) = ground_effect {
            destruction_effect = destruction_effect.with_ground_effect(ground_effect);
        }
        RegionDestructibleProfile::new(
            self.count[0],
            self.count[1],
            self.minimum_passage_distance,
            self.maximum_integrity,
            destruction_effect,
        )
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionExplosion {
    radius: u16,
    #[serde(default = "one_u16")]
    floor_cost: u16,
    shallow_water_cost: Option<u16>,
    deep_water_cost: Option<u16>,
    damage: RawWeaponDamage,
    falloff_per_cost: Option<u16>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionGroundEffect {
    id: String,
    duration_turns: u16,
    damage_each_turn: RawWeaponDamage,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionLootProfile {
    table: String,
    source: String,
    draws: [u16; 2],
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionLandmarkProfile {
    #[serde(default)]
    caches: [u16; 2],
    #[serde(default)]
    threat_camps: [u16; 2],
    #[serde(default)]
    minimum_passage_distance: u16,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionSiteProfile {
    #[serde(default)]
    compounds: [u16; 2],
    #[serde(default)]
    width: u16,
    #[serde(default)]
    height: u16,
    #[serde(default)]
    entrances: RawRegionSiteEntranceProfile,
    terminals: Option<RawRegionSiteTerminalProfile>,
    security: Option<RawRegionSiteSecurityProfile>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionSiteTerminalProfile {
    count: [u16; 2],
    records: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionSiteSecurityProfile {
    owner: String,
    radius: u16,
    #[serde(default = "default_witness_distance_metric")]
    distance_metric: RawDistanceMetric,
    #[serde(default = "default_block_closed_corners")]
    block_closed_corners: bool,
    duration_turns: u16,
    reinforcement_delay_turns: u16,
    #[serde(default)]
    navigation_signal_range: Option<u16>,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionSiteEntranceProfile {
    #[serde(default)]
    open: u16,
    #[serde(default)]
    closed_door: u16,
    #[serde(default)]
    locked_console: u16,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionThreatProfile {
    interval_turns: u16,
    maximum_active: u16,
    maximum_total: u16,
    maximum_integrity: u16,
    attack: RawWeaponAttack,
    ai: RawAiProfile,
    primary_attributes: Option<RawPrimaryAttributes>,
    body: Option<RawBodyProfile>,
    #[serde(default)]
    components: Vec<RawBodyComponentProfile>,
    electronic_system: Option<RawElectronicSystemProfile>,
    #[serde(default)]
    player_relation: RawPlayerRelation,
    #[serde(default)]
    tags: Vec<String>,
}

impl RawRegionThreatProfile {
    fn into_runtime(self, tags: Vec<ContentId>) -> Result<RegionThreatProfile, RegionalWorldError> {
        let interval_turns = std::num::NonZeroU16::new(self.interval_turns)
            .ok_or(RegionalWorldError::InvalidThreatLimits)?;
        let maximum_active = std::num::NonZeroU16::new(self.maximum_active)
            .ok_or(RegionalWorldError::InvalidThreatLimits)?;
        let maximum_total = std::num::NonZeroU16::new(self.maximum_total)
            .ok_or(RegionalWorldError::InvalidThreatLimits)?;
        let attack = self.attack.into_runtime().map_err(|error| {
            RegionalWorldError::InvalidPopulation(Box::new(
                ExpeditionDefinitionError::InvalidPopulationAttackDefinition(error),
            ))
        })?;
        let ai = self
            .ai
            .into_runtime()
            .map_err(|error| RegionalWorldError::InvalidPopulation(Box::new(error)))?;
        let mut profile = RegionThreatProfile::new(
            interval_turns,
            maximum_active,
            maximum_total,
            self.maximum_integrity,
            attack,
            ai,
        )?;
        profile = profile.with_player_relation(self.player_relation.into_runtime());
        profile = profile.with_tags(tags)?;
        if let Some(attributes) = self.primary_attributes {
            profile = profile.with_primary_attributes(attributes.into_runtime())?;
        }
        if let Some(body) = self.body {
            profile = profile.with_body_profile(body.into_runtime().map_err(|error| {
                RegionalWorldError::InvalidPopulation(Box::new(
                    ExpeditionDefinitionError::InvalidPopulationBody(error),
                ))
            })?);
        }
        if !self.components.is_empty() {
            let components = self
                .components
                .into_iter()
                .map(RawBodyComponentProfile::into_runtime)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| {
                    RegionalWorldError::InvalidPopulation(Box::new(
                        ExpeditionDefinitionError::InvalidPopulationComponent(error),
                    ))
                })?;
            profile = profile.with_body_components(components);
        }
        if let Some(electronic_system) = self.electronic_system {
            profile = profile.with_electronic_system(electronic_system.into_runtime().map_err(
                |error| {
                    RegionalWorldError::InvalidPopulation(Box::new(
                        ExpeditionDefinitionError::InvalidPopulationElectronicSystem(error),
                    ))
                },
            )?);
        }
        Ok(profile)
    }
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionPopulationProfile {
    #[serde(default)]
    group_rolls: [u16; 2],
    #[serde(default)]
    groups: Vec<RawRegionPopulationRule>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionPopulationRule {
    weight: u32,
    count: [u16; 2],
    #[serde(default)]
    minimum_passage_distance: u16,
    maximum_integrity: u16,
    attack: RawWeaponAttack,
    ai: RawAiProfile,
    defeat_reward: Option<RawDefeatReward>,
    primary_attributes: Option<RawPrimaryAttributes>,
    body: Option<RawBodyProfile>,
    #[serde(default)]
    components: Vec<RawBodyComponentProfile>,
    electronic_system: Option<RawElectronicSystemProfile>,
    #[serde(default)]
    player_relation: RawPlayerRelation,
    #[serde(default)]
    tags: Vec<String>,
}

impl RawRegionPopulationRule {
    fn into_runtime(
        self,
        tags: Vec<ContentId>,
    ) -> Result<RegionPopulationRule, RegionalWorldError> {
        let attack = self.attack.into_runtime().map_err(|error| {
            RegionalWorldError::InvalidPopulation(Box::new(
                ExpeditionDefinitionError::InvalidPopulationAttackDefinition(error),
            ))
        })?;
        let ai = self
            .ai
            .into_runtime()
            .map_err(|error| RegionalWorldError::InvalidPopulation(Box::new(error)))?;
        let mut rule = RegionPopulationRule::new(
            self.weight,
            self.count[0],
            self.count[1],
            self.minimum_passage_distance,
            self.maximum_integrity,
            attack,
            ai,
            self.defeat_reward.map(RawDefeatReward::into_runtime),
        )?;
        rule = rule.with_player_relation(self.player_relation.into_runtime());
        rule = rule.with_tags(tags)?;
        if let Some(attributes) = self.primary_attributes {
            rule = rule.with_primary_attributes(attributes.into_runtime())?;
        }
        if let Some(body) = self.body {
            rule = rule.with_body_profile(body.into_runtime().map_err(|error| {
                RegionalWorldError::InvalidPopulation(Box::new(
                    ExpeditionDefinitionError::InvalidPopulationBody(error),
                ))
            })?);
        }
        if !self.components.is_empty() {
            let components = self
                .components
                .into_iter()
                .map(RawBodyComponentProfile::into_runtime)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|error| {
                    RegionalWorldError::InvalidPopulation(Box::new(
                        ExpeditionDefinitionError::InvalidPopulationComponent(error),
                    ))
                })?;
            rule = rule.with_body_components(components);
        }
        if let Some(electronic_system) = self.electronic_system {
            rule =
                rule.with_electronic_system(electronic_system.into_runtime().map_err(|error| {
                    RegionalWorldError::InvalidPopulation(Box::new(
                        ExpeditionDefinitionError::InvalidPopulationElectronicSystem(error),
                    ))
                })?);
        }
        Ok(rule)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionTerrainProfile {
    ground: RawRegionTerrain,
    patch_count: u16,
    minimum_patch_radius: u16,
    maximum_patch_radius: u16,
    features: Vec<RawRegionTerrainRule>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRegionTerrainRule {
    kind: RawRegionTerrain,
    weight: u32,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawRegionTerrain {
    Gravel,
    Grass,
    Scrub,
    Mud,
    ShallowWater,
    DeepWater,
    Tree,
    Boulder,
    RuinFloor,
    RuinWall,
}

impl RawRegionTerrain {
    const fn into_runtime(self) -> RegionTerrain {
        match self {
            Self::Gravel => RegionTerrain::Gravel,
            Self::Grass => RegionTerrain::Grass,
            Self::Scrub => RegionTerrain::Scrub,
            Self::Mud => RegionTerrain::Mud,
            Self::ShallowWater => RegionTerrain::ShallowWater,
            Self::DeepWater => RegionTerrain::DeepWater,
            Self::Tree => RegionTerrain::Tree,
            Self::Boulder => RegionTerrain::Boulder,
            Self::RuinFloor => RegionTerrain::RuinFloor,
            Self::RuinWall => RegionTerrain::RuinWall,
        }
    }
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
        let failure = |error| ContentLoadError::ExpeditionDefinition {
            package: package.manifest.id.clone(),
            path: path.clone(),
            error: Box::new(error),
        };
        let resolve_zone = |zone: RawZoneDefinition| -> Result<ZoneDefinition, ContentLoadError> {
            Ok(ZoneDefinition {
                id: parse(&zone.id)?,
                name: zone.name,
                kind: parse(&zone.kind)?,
                depth: zone.depth,
            })
        };
        let population = raw
            .population
            .into_iter()
            .map(|group| {
                let tags = group
                    .tags
                    .iter()
                    .map(|tag| parse(tag))
                    .collect::<Result<Vec<_>, ContentLoadError>>()?;
                group.into_runtime(tags).map_err(&failure)
            })
            .collect::<Result<Vec<_>, ContentLoadError>>()?;
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
            population,
        };
        let mut definition = ExpeditionDefinition::new(
            id,
            resolve_zone(raw.hub)?,
            destination,
            crate::world::GridPos::new(raw.passage[0], raw.passage[1]),
        )
        .map_err(&failure)?;
        if let Some(expanded) = raw.expanded_world {
            definition = definition
                .with_expanded_world(ExpandedWorldDefinition {
                    hub_passage: grid_position(expanded.passage),
                    destination_generator: expanded.generator(),
                })
                .map_err(&failure)?;
        }
        definition = definition
            .with_player_property_take_authorizations(
                raw.player_property_take_authorizations
                    .iter()
                    .map(|owner| parse(owner))
                    .collect::<Result<_, _>>()?,
            )
            .map_err(&failure)?;
        if let Some(merchant) = raw.hub_merchant {
            let offers = merchant
                .offers
                .into_iter()
                .map(|offer| {
                    Ok(crate::content::MerchantOfferDefinition {
                        item: parse(&offer.item)?,
                        initial_stock: offer.stock,
                        buy_price: offer.buy_price,
                        sell_price: offer.sell_price,
                        minimum_depth: offer.minimum_depth,
                        maximum_depth: offer.maximum_depth,
                    })
                })
                .collect::<Result<Vec<_>, ContentLoadError>>()?;
            let gambles = merchant
                .gambles
                .into_iter()
                .map(|gamble| {
                    Ok(crate::content::MerchantGambleDefinition {
                        item: parse(&gamble.item)?,
                        initial_stock: gamble.stock,
                        price: gamble.price,
                    })
                })
                .collect::<Result<Vec<_>, ContentLoadError>>()?;
            definition = definition.with_hub_merchant(
                crate::content::MerchantDefinition::new(
                    grid_position(merchant.position),
                    merchant.maximum_integrity,
                    merchant.credits,
                    offers,
                    gambles,
                    crate::content::GambleScalingDefinition::new(
                        merchant.gamble_scaling.player_levels_per_rank,
                        merchant.gamble_scaling.zone_depths_per_rank,
                        merchant.gamble_scaling.maximum_rank,
                    )
                    .ok_or_else(|| failure(ExpeditionDefinitionError::InvalidMerchant))?,
                )
                .map_err(&failure)?,
                raw.player_starting_credits,
            );
        }
        if let Some(clinic) = raw.hub_clinic {
            definition = definition.with_hub_clinic(
                crate::content::ClinicDefinition::new(
                    grid_position(clinic.work_position),
                    grid_position(clinic.break_position),
                    clinic.maximum_integrity,
                    clinic.credits,
                    clinic.maximum_restoration,
                    clinic.price_per_point,
                    clinic.work_turns,
                    clinic.break_turns,
                    clinic.maximum_path_search,
                )
                .map_err(&failure)?,
            );
        }
        let residents = raw
            .hub_residents
            .into_iter()
            .map(|resident| {
                crate::content::ResidentDefinition::new(
                    grid_position(resident.residence_position),
                    grid_position(resident.gathering_position),
                    resident.maximum_integrity,
                    resident.residence_turns,
                    resident.gathering_turns,
                    resident.maximum_path_search,
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(&failure)?;
        definition = definition.with_hub_residents(residents).map_err(&failure)?;
        let quests = raw
            .hub_quests
            .into_iter()
            .map(|quest| {
                let quest_id = parse(&quest.id)?;
                ensure_local_namespace(package, &path, &quest_id)?;
                let provider = match quest.provider {
                    RawHubQuestProviderDefinition::Existing { position } => {
                        crate::content::HubQuestProviderDefinition::existing(grid_position(
                            position,
                        ))
                        .map_err(&failure)?
                    }
                    RawHubQuestProviderDefinition::Contact {
                        position,
                        maximum_integrity,
                    } => crate::content::HubQuestProviderDefinition::contact(
                        grid_position(position),
                        maximum_integrity,
                    )
                    .map_err(&failure)?,
                };
                let objective = match quest.objective {
                    RawQuestObjectiveDefinition::Delivery {
                        required_item,
                        required_quantity,
                    } => crate::content::DeliveryQuestDefinition::new(
                        quest_id,
                        quest.title_key,
                        quest.summary_key,
                        parse(&required_item)?,
                        required_quantity,
                        quest.reward_credits,
                    )
                    .map(crate::content::QuestDefinition::from)
                    .map_err(&failure)?,
                    RawQuestObjectiveDefinition::ExploreZones {
                        required_zones,
                        qualifying_records,
                    } => {
                        let mut definition = crate::content::ExplorationQuestDefinition::new(
                            quest_id,
                            quest.title_key,
                            quest.summary_key,
                            required_zones,
                            quest.reward_credits,
                        )
                        .map_err(&failure)?;
                        if !qualifying_records.is_empty() {
                            definition = definition
                                .with_qualifying_records(
                                    qualifying_records
                                        .iter()
                                        .map(|record| parse(record))
                                        .collect::<Result<_, _>>()?,
                                )
                                .map_err(&failure)?;
                        }
                        definition.into()
                    }
                    RawQuestObjectiveDefinition::AccessDataRecord { record } => {
                        crate::content::DataRecordQuestDefinition::new(
                            quest_id,
                            quest.title_key,
                            quest.summary_key,
                            parse(&record)?,
                            quest.reward_credits,
                        )
                        .map(crate::content::QuestDefinition::from)
                        .map_err(&failure)?
                    }
                    RawQuestObjectiveDefinition::DefeatTargets {
                        target_tag,
                        required_quantity,
                    } => crate::content::DefeatTargetsQuestDefinition::new(
                        quest_id,
                        quest.title_key,
                        quest.summary_key,
                        parse(&target_tag)?,
                        required_quantity,
                        quest.reward_credits,
                    )
                    .map(crate::content::QuestDefinition::from)
                    .map_err(&failure)?,
                };
                let prerequisites = quest
                    .prerequisites
                    .iter()
                    .map(|id| parse(id))
                    .collect::<Result<Vec<_>, _>>()?;
                let reward_items = quest
                    .reward_items
                    .into_iter()
                    .map(|reward| {
                        crate::content::QuestItemRewardDefinition::new(
                            parse(&reward.item)?,
                            reward.quantity,
                        )
                        .map_err(&failure)
                    })
                    .collect::<Result<Vec<_>, ContentLoadError>>()?;
                let required_world_states = quest
                    .required_world_states
                    .iter()
                    .map(|id| parse(id))
                    .collect::<Result<Vec<_>, _>>()?;
                let completion_world_states = quest
                    .completion_world_states
                    .into_iter()
                    .map(|state| {
                        let state_id = parse(&state.id)?;
                        ensure_local_namespace(package, &path, &state_id)?;
                        let mut definition = crate::content::QuestWorldStateDefinition::new(
                            state_id,
                            state.summary_key,
                        )
                        .map_err(&failure)?;
                        if let Some(dialogue_key) = state.provider_dialogue_key {
                            definition = definition
                                .with_provider_dialogue(dialogue_key)
                                .map_err(&failure)?;
                        }
                        Ok(definition)
                    })
                    .collect::<Result<Vec<_>, ContentLoadError>>()?;
                let completion_world_effects = quest
                    .completion_world_effects
                    .into_iter()
                    .map(|effect| match effect {
                        RawQuestWorldEffectDefinition::UnlockDoor {
                            position,
                            summary_key,
                        } => crate::content::QuestWorldEffectDefinition::unlock_door(
                            GridPos::new(position[0], position[1]),
                            summary_key,
                        )
                        .map_err(&failure),
                        RawQuestWorldEffectDefinition::UpdateDataTerminal {
                            installation,
                            record,
                            summary_key,
                        } => {
                            let installation = parse(&installation)?;
                            ensure_local_namespace(package, &path, &installation)?;
                            crate::content::QuestWorldEffectDefinition::update_data_terminal(
                                installation,
                                parse(&record)?,
                                summary_key,
                            )
                            .map_err(&failure)
                        }
                        RawQuestWorldEffectDefinition::GrantPropertyTakeAuthorization {
                            owner,
                            summary_key,
                        } => crate::content::QuestWorldEffectDefinition::grant_property_take_authorization(
                            parse(&owner)?,
                            summary_key,
                        )
                        .map_err(&failure),
                    })
                    .collect::<Result<Vec<_>, ContentLoadError>>()?;
                let mut definition = crate::content::HubQuestDefinition::new(provider, objective)
                    .with_prerequisites(prerequisites)
                    .map_err(&failure)?
                    .with_world_states(required_world_states, completion_world_states)
                    .map_err(&failure)?
                    .with_world_effects(completion_world_effects)
                    .map_err(&failure)?
                    .with_additional_rewards(quest.reward_experience, reward_items)
                    .map_err(&failure)?;
                match (quest.choice_group, quest.choice_prompt_key) {
                    (Some(group), Some(prompt)) => {
                        let group = parse(&group)?;
                        ensure_local_namespace(package, &path, &group)?;
                        definition = definition.with_choice(group, prompt).map_err(&failure)?;
                    }
                    (None, None) => {}
                    _ => return Err(failure(ExpeditionDefinitionError::InvalidQuestChoice)),
                }
                Ok(definition)
            })
            .collect::<Result<Vec<_>, ContentLoadError>>()?;
        definition = definition.with_hub_quests(quests).map_err(&failure)?;
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
                                RawSecurityAlarmResponse::CallReinforcements {
                                    source,
                                    delay_turns,
                                } => Ok(SecurityAlarmResponse::CallReinforcements {
                                    source: GridPos::new(source[0], source[1]),
                                    delay_turns: *delay_turns,
                                }),
                                RawSecurityAlarmResponse::CallInvestigatingReinforcements {
                                    source,
                                    delay_turns,
                                } => Ok(SecurityAlarmResponse::CallInvestigatingReinforcements {
                                    source: GridPos::new(source[0], source[1]),
                                    delay_turns: *delay_turns,
                                }),
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
                            .map(|capability| capability.into_runtime(package, &path))
                            .collect::<Result<_, _>>()?,
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
                    let property_report = worker
                        .property_report
                        .map(RawPropertyReportProfile::into_runtime)
                        .transpose()
                        .map_err(|error| {
                            failure(ExpeditionDefinitionError::InvalidPropertyReportProfile(
                                error,
                            ))
                        })?;
                    let installed_property_report = worker
                        .installed_property_report
                        .map(|report| {
                            let installation = parse(&report.installation)?;
                            let channel = report.into_channel().map_err(|error| {
                                failure(ExpeditionDefinitionError::InvalidPropertyReportProfile(
                                    error,
                                ))
                            })?;
                            Ok::<_, ContentLoadError>(WorkerInstalledPropertyReportBlueprint {
                                installation,
                                channel,
                            })
                        })
                        .transpose()?;
                    Ok(WorkerBlueprint {
                        actor_position: grid_position(worker.position),
                        role: worker.role.into_runtime(),
                        maximum_integrity: worker.maximum_integrity,
                        affiliation: worker.affiliation.as_deref().map(parse).transpose()?,
                        witness_profile,
                        local_alert_profile,
                        property_report,
                        installed_property_report,
                        reported_incident_response: worker
                            .reported_incident_response
                            .map(RawReportedIncidentResponse::into_runtime),
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
        if let Some(narrative) = raw.narrative {
            narrative
                .validate()
                .map_err(|explanation| ContentLoadError::DefinitionSyntax {
                    package: package.manifest.id.clone(),
                    path: path.clone(),
                    definition_kind: "narrative",
                    explanation,
                })?;
            definition.narrative = Some(narrative);
            definition.narrative_reward_experience = raw.narrative_reward_experience;
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
    narrative: Option<super::NarrativeDefinition>,
    #[serde(default)]
    narrative_reward_experience: u64,
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
    population: Vec<RawPopulationGroup>,
    expanded_world: Option<RawExpandedWorldDefinition>,
    #[serde(default)]
    player_property_take_authorizations: Vec<String>,
    #[serde(default)]
    player_starting_credits: u32,
    hub_merchant: Option<RawMerchantDefinition>,
    hub_clinic: Option<RawClinicDefinition>,
    #[serde(default)]
    hub_residents: Vec<RawResidentDefinition>,
    #[serde(default)]
    hub_quests: Vec<RawHubQuestDefinition>,
    hub_facility: Option<RawFacilityDefinition>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawHubQuestDefinition {
    id: String,
    title_key: String,
    summary_key: String,
    reward_credits: u32,
    #[serde(default)]
    reward_experience: u64,
    #[serde(default)]
    reward_items: Vec<RawQuestItemRewardDefinition>,
    #[serde(default)]
    prerequisites: Vec<String>,
    #[serde(default)]
    required_world_states: Vec<String>,
    #[serde(default)]
    completion_world_states: Vec<RawQuestWorldStateDefinition>,
    #[serde(default)]
    completion_world_effects: Vec<RawQuestWorldEffectDefinition>,
    choice_group: Option<String>,
    choice_prompt_key: Option<String>,
    objective: RawQuestObjectiveDefinition,
    provider: RawHubQuestProviderDefinition,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawQuestItemRewardDefinition {
    item: String,
    quantity: u16,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawQuestWorldStateDefinition {
    id: String,
    summary_key: String,
    provider_dialogue_key: Option<String>,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawQuestWorldEffectDefinition {
    UnlockDoor {
        position: [i32; 2],
        summary_key: String,
    },
    UpdateDataTerminal {
        installation: String,
        record: String,
        summary_key: String,
    },
    GrantPropertyTakeAuthorization {
        owner: String,
        summary_key: String,
    },
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawQuestObjectiveDefinition {
    Delivery {
        required_item: String,
        required_quantity: u16,
    },
    ExploreZones {
        required_zones: u16,
        #[serde(default)]
        qualifying_records: Vec<String>,
    },
    AccessDataRecord {
        record: String,
    },
    DefeatTargets {
        target_tag: String,
        required_quantity: u16,
    },
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawHubQuestProviderDefinition {
    Existing {
        position: [i32; 2],
    },
    Contact {
        position: [i32; 2],
        maximum_integrity: u16,
    },
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawResidentDefinition {
    residence_position: [i32; 2],
    gathering_position: [i32; 2],
    maximum_integrity: u16,
    residence_turns: u16,
    gathering_turns: u16,
    maximum_path_search: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawClinicDefinition {
    work_position: [i32; 2],
    break_position: [i32; 2],
    maximum_integrity: u16,
    credits: u32,
    maximum_restoration: u16,
    price_per_point: u32,
    work_turns: u16,
    break_turns: u16,
    maximum_path_search: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMerchantDefinition {
    position: [i32; 2],
    maximum_integrity: u16,
    credits: u32,
    offers: Vec<RawMerchantOfferDefinition>,
    gambles: Vec<RawMerchantGambleDefinition>,
    gamble_scaling: RawGambleScalingDefinition,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawGambleScalingDefinition {
    player_levels_per_rank: u16,
    zone_depths_per_rank: u16,
    maximum_rank: u16,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMerchantOfferDefinition {
    item: String,
    stock: u16,
    buy_price: u32,
    sell_price: u32,
    #[serde(default)]
    minimum_depth: u16,
    maximum_depth: Option<u16>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMerchantGambleDefinition {
    item: String,
    stock: u16,
    price: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawExpandedWorldDefinition {
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
}

impl RawExpandedWorldDefinition {
    const fn generator(&self) -> RoomsGeneratorConfig {
        RoomsGeneratorConfig {
            width: self.width,
            height: self.height,
            room_count: self.rooms,
            minimum_room_width: self.minimum_room_width,
            maximum_room_width: self.maximum_room_width,
            minimum_room_height: self.minimum_room_height,
            maximum_room_height: self.maximum_room_height,
            placement_attempts: self.placement_attempts,
            validation: MapValidationRules {
                require_sealed_border: true,
                require_all_walkable_connected: true,
            },
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPopulationGroup {
    count: u16,
    #[serde(default)]
    minimum_entrance_distance: u16,
    maximum_integrity: u16,
    attack: RawWeaponAttack,
    ai: RawAiProfile,
    defeat_reward: Option<RawDefeatReward>,
    primary_attributes: Option<RawPrimaryAttributes>,
    body: Option<RawBodyProfile>,
    #[serde(default)]
    components: Vec<RawBodyComponentProfile>,
    electronic_system: Option<RawElectronicSystemProfile>,
    #[serde(default)]
    player_relation: RawPlayerRelation,
    #[serde(default)]
    tags: Vec<String>,
}

impl RawPopulationGroup {
    fn into_runtime(
        self,
        tags: Vec<ContentId>,
    ) -> Result<PopulationGroupDefinition, ExpeditionDefinitionError> {
        let attack = self
            .attack
            .into_runtime()
            .map_err(ExpeditionDefinitionError::InvalidPopulationAttackDefinition)?;
        let ai = self.ai.into_runtime()?;
        let mut group = PopulationGroupDefinition::new(
            self.count,
            self.minimum_entrance_distance,
            self.maximum_integrity,
            attack,
            ai,
            self.defeat_reward.map(RawDefeatReward::into_runtime),
        )?;
        group = group.with_player_relation(self.player_relation.into_runtime());
        group = group.with_tags(tags)?;
        if let Some(attributes) = self.primary_attributes {
            group = group.with_primary_attributes(attributes.into_runtime())?;
        }
        if let Some(body) = self.body {
            group = group.with_body_profile(
                body.into_runtime()
                    .map_err(ExpeditionDefinitionError::InvalidPopulationBody)?,
            );
        }
        if !self.components.is_empty() {
            let components = self
                .components
                .into_iter()
                .map(RawBodyComponentProfile::into_runtime)
                .collect::<Result<Vec<_>, _>>()
                .map_err(ExpeditionDefinitionError::InvalidPopulationComponent)?;
            group = group.with_body_components(components);
        }
        if let Some(electronic_system) = self.electronic_system {
            group = group.with_electronic_system(
                electronic_system
                    .into_runtime()
                    .map_err(ExpeditionDefinitionError::InvalidPopulationElectronicSystem)?,
            );
        }
        Ok(group)
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawPlayerRelation {
    Allied,
    #[default]
    Neutral,
    Hostile,
}

impl RawPlayerRelation {
    const fn into_runtime(self) -> PlayerRelation {
        match self {
            Self::Allied => PlayerRelation::Allied,
            Self::Neutral => PlayerRelation::Neutral,
            Self::Hostile => PlayerRelation::Hostile,
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawElectronicSystemProfile {
    digital_defense: u16,
    heat_alert_threshold: u16,
    heat_critical_threshold: u16,
    heat_dissipation_per_phase: u16,
    stored_energy: u16,
}

impl RawElectronicSystemProfile {
    fn into_runtime(self) -> Result<crate::electronic_warfare::ElectronicSystemProfile, String> {
        crate::electronic_warfare::ElectronicSystemProfile::new(
            self.digital_defense,
            self.heat_alert_threshold,
            self.heat_critical_threshold,
            self.heat_dissipation_per_phase,
            self.stored_energy,
        )
        .map_err(|error| error.to_string())
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawBodyComponentProfile {
    id: String,
    name_key: String,
    maximum_durability: u16,
    #[serde(default)]
    failure_threshold: u16,
    failure_effect: RawComponentFailureEffect,
}

impl RawBodyComponentProfile {
    fn into_runtime(self) -> Result<crate::entity::BodyComponentProfile, String> {
        crate::entity::BodyComponentProfile::new(
            self.id
                .parse()
                .map_err(|error| format!("invalid component ID '{}': {error}", self.id))?,
            self.name_key,
            self.maximum_durability,
            self.failure_threshold,
            self.failure_effect.into_runtime(),
        )
        .map_err(|error| error.to_string())
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawComponentFailureEffect {
    DisableMovement,
    DisableAttackSlot { slot: u8 },
    ReduceArmor { amount: u16 },
    ReducePerception { amount: u16 },
}

impl RawComponentFailureEffect {
    const fn into_runtime(self) -> crate::entity::ComponentFailureEffect {
        match self {
            Self::DisableMovement => crate::entity::ComponentFailureEffect::DisableMovement,
            Self::DisableAttackSlot { slot } => {
                crate::entity::ComponentFailureEffect::DisableAttackSlot(slot)
            }
            Self::ReduceArmor { amount } => {
                crate::entity::ComponentFailureEffect::ReduceArmor(amount)
            }
            Self::ReducePerception { amount } => {
                crate::entity::ComponentFailureEffect::ReducePerception(amount)
            }
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawBodyProfile {
    base_hit_points: u16,
    #[serde(default)]
    material_bonus: i16,
    #[serde(default)]
    base_armor: u16,
    mass_grams: Option<u32>,
    #[serde(default)]
    anchoring: u16,
    #[serde(default)]
    fixed: bool,
    locomotion: Option<RawLocomotionProfile>,
    #[serde(default)]
    suppression_compatible: bool,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawLocomotionProfile {
    hindrance_compatible: bool,
}

impl RawBodyProfile {
    fn into_runtime(self) -> Result<crate::stats::BodyProfile, crate::stats::PhysicalRulesError> {
        let mut body = crate::stats::BodyProfile::new(self.base_hit_points, self.material_bonus)?
            .with_base_armor(self.base_armor);
        match self.mass_grams {
            Some(mass_grams) => {
                let mut displacement =
                    crate::stats::DisplacementProfile::new(mass_grams, self.anchoring)?;
                if self.fixed {
                    displacement = displacement.fixed();
                }
                body = body.with_displacement_profile(displacement);
            }
            None if self.anchoring > 0 || self.fixed => {
                return Err(crate::stats::PhysicalRulesError::DisplacementPropertiesWithoutMass);
            }
            None => {}
        }
        if let Some(locomotion) = self.locomotion {
            body = body.with_locomotion_profile(crate::stats::LocomotionProfile::new(
                locomotion.hindrance_compatible,
            ));
        }
        body = body.with_suppression_compatibility(self.suppression_compatible);
        Ok(body)
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPrimaryAttributes {
    power: u8,
    coordination: u8,
    resilience: u8,
    perception: u8,
    processing: u8,
}

impl RawPrimaryAttributes {
    const fn into_runtime(self) -> PrimaryAttributes {
        PrimaryAttributes::new(
            self.power,
            self.coordination,
            self.resilience,
            self.perception,
            self.processing,
        )
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawAiProfile {
    behavior: RawAiBehavior,
    perception_radius: u16,
    #[serde(default)]
    preferred_attack_slot: u8,
    #[serde(default = "default_ai_path_search")]
    maximum_path_search: usize,
    #[serde(default)]
    preferred_minimum_distance: u16,
    maximum_pursuit_distance: Option<u16>,
    pursuit_lifecycle: Option<RawPursuitLifecycle>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPursuitLifecycle {
    maximum_turns: u16,
    search_turns: u16,
    cooldown_turns: u16,
}

impl RawAiProfile {
    fn into_runtime(self) -> Result<AiProfile, ExpeditionDefinitionError> {
        let mut profile = AiProfile::new(
            self.behavior.into_runtime(),
            self.perception_radius,
            self.preferred_attack_slot,
            self.maximum_path_search,
            self.preferred_minimum_distance,
        );
        if let Some(distance) = self.maximum_pursuit_distance {
            let distance = std::num::NonZeroU16::new(distance)
                .ok_or(ExpeditionDefinitionError::ZeroPopulationPursuitDistance)?;
            profile = profile.with_maximum_pursuit_distance(distance);
        }
        if let Some(lifecycle) = self.pursuit_lifecycle {
            let maximum_turns = std::num::NonZeroU16::new(lifecycle.maximum_turns)
                .ok_or(ExpeditionDefinitionError::InvalidPopulationPursuitLifecycle)?;
            let search_turns = std::num::NonZeroU16::new(lifecycle.search_turns)
                .ok_or(ExpeditionDefinitionError::InvalidPopulationPursuitLifecycle)?;
            let cooldown_turns = std::num::NonZeroU16::new(lifecycle.cooldown_turns)
                .ok_or(ExpeditionDefinitionError::InvalidPopulationPursuitLifecycle)?;
            profile = profile.with_pursuit_lifecycle(crate::ai::PursuitLifecycle::new(
                maximum_turns,
                search_turns,
                cooldown_turns,
            ));
        }
        Ok(profile)
    }
}

const fn default_ai_path_search() -> usize {
    2_048
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawAiBehavior {
    Idle,
    Hunter,
    Sentry,
    Skirmisher,
}

impl RawAiBehavior {
    const fn into_runtime(self) -> AiBehavior {
        match self {
            Self::Idle => AiBehavior::Idle,
            Self::Hunter => AiBehavior::Hunter,
            Self::Sentry => AiBehavior::Sentry,
            Self::Skirmisher => AiBehavior::Skirmisher,
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawDefeatReward {
    base_experience: u64,
    threat_level: u16,
    #[serde(default)]
    origin: RawExperienceRewardOrigin,
}

impl RawDefeatReward {
    const fn into_runtime(self) -> DefeatReward {
        DefeatReward {
            base_experience: self.base_experience,
            threat_level: self.threat_level,
            origin: self.origin.into_runtime(),
        }
    }
}

#[derive(Clone, Copy, Default, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawExperienceRewardOrigin {
    #[default]
    Persistent,
    Summoned,
    Fabricated,
}

impl RawExperienceRewardOrigin {
    const fn into_runtime(self) -> ExperienceRewardOrigin {
        match self {
            Self::Persistent => ExperienceRewardOrigin::Persistent,
            Self::Summoned => ExperienceRewardOrigin::Summoned,
            Self::Fabricated => ExperienceRewardOrigin::Fabricated,
        }
    }
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
    NavigationBeacon { range: u16 },
    DataTerminal { record: String },
    Storage,
}

impl RawInstallationCapability {
    fn into_runtime(
        self,
        package: &DiscoveredPackage,
        path: &Path,
    ) -> Result<InstallationCapability, ContentLoadError> {
        Ok(match self {
            Self::PowerRelay => InstallationCapability::PowerRelay,
            Self::DoorActuator { door } => InstallationCapability::DoorActuator {
                door: grid_position(door),
            },
            Self::SecuritySensor => InstallationCapability::SecuritySensor,
            Self::NavigationBeacon { range } => InstallationCapability::NavigationBeacon { range },
            Self::DataTerminal { record } => InstallationCapability::DataTerminal {
                record: parse_content_id(package, path, &record)?,
            },
            Self::Storage => InstallationCapability::Storage,
        })
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
    property_report: Option<RawPropertyReportProfile>,
    installed_property_report: Option<RawInstalledPropertyReportProfile>,
    reported_incident_response: Option<RawReportedIncidentResponse>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawInstalledPropertyReportProfile {
    installation: String,
    radius: u16,
    #[serde(default = "default_witness_distance_metric")]
    distance_metric: RawDistanceMetric,
    #[serde(default = "default_block_closed_corners")]
    block_closed_corners: bool,
}

impl RawInstalledPropertyReportProfile {
    fn into_channel(
        self,
    ) -> Result<PropertyReportChannel, crate::social::PropertyReportProfileError> {
        PropertyReportChannel::new(
            self.radius,
            self.distance_metric.into_runtime(),
            self.block_closed_corners,
        )
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawReportedIncidentResponse {
    inspection_turns: u16,
    maximum_response_turns: u16,
}

impl RawReportedIncidentResponse {
    const fn into_runtime(self) -> WorkerReportedIncidentResponseBlueprint {
        WorkerReportedIncidentResponseBlueprint {
            inspection_turns: self.inspection_turns,
            maximum_response_turns: self.maximum_response_turns,
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPropertyReportProfile {
    recipient_position: [i32; 2],
    radius: u16,
    #[serde(default = "default_witness_distance_metric")]
    distance_metric: RawDistanceMetric,
    #[serde(default = "default_block_closed_corners")]
    block_closed_corners: bool,
}

impl RawPropertyReportProfile {
    fn into_runtime(
        self,
    ) -> Result<WorkerPropertyReportBlueprint, crate::social::PropertyReportProfileError> {
        Ok(WorkerPropertyReportBlueprint {
            recipient_position: grid_position(self.recipient_position),
            channel: PropertyReportChannel::new(
                self.radius,
                self.distance_metric.into_runtime(),
                self.block_closed_corners,
            )?,
        })
    }
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
    CallReinforcements { source: [i32; 2], delay_turns: u16 },
    CallInvestigatingReinforcements { source: [i32; 2], delay_turns: u16 },
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
            let engagement_requirement = match technique.engagement_requirement.as_ref() {
                Some(RawTechniqueEngagementRequirement::TargetHasAnyStatusFamily { families }) => {
                    let families = families
                        .iter()
                        .map(|value| parse_content_id(package, &path, value))
                        .collect::<Result<Vec<_>, _>>()?;
                    Some(
                        TechniqueEngagementRequirement::target_has_any_status_family(families)
                            .map_err(|error| ContentLoadError::SkillDefinition {
                                package: package.manifest.id.clone(),
                                path: path.clone(),
                                content: Box::new(id.clone()),
                                error,
                            })?,
                    )
                }
                Some(RawTechniqueEngagementRequirement::TargetHasKnownPhysicalWeakness) => {
                    Some(TechniqueEngagementRequirement::TargetHasKnownPhysicalWeakness)
                }
                None => None,
            };
            let minimum_attributes =
                technique
                    .minimum_attributes
                    .into_runtime()
                    .map_err(|error| ContentLoadError::SkillDefinition {
                        package: package.manifest.id.clone(),
                        path: path.clone(),
                        content: Box::new(id.clone()),
                        error,
                    })?;
            let mut definition = TechniqueDefinition::new(
                id.clone(),
                discipline,
                technique.name_key,
                technique.description_key,
                technique.minimum_level,
                technique.kind.into_runtime(),
                prerequisite,
                required_features,
            )
            .map_err(|error| ContentLoadError::SkillDefinition {
                package: package.manifest.id.clone(),
                path: path.clone(),
                content: Box::new(id.clone()),
                error,
            })?
            .with_attribute_requirements(minimum_attributes)
            .map_err(|error| ContentLoadError::SkillDefinition {
                package: package.manifest.id.clone(),
                path: path.clone(),
                content: Box::new(id.clone()),
                error,
            })?;
            if let Some(action) = technique.action {
                definition = definition
                    .with_action(action.into_runtime().map_err(|error| {
                        ContentLoadError::SkillDefinition {
                            package: package.manifest.id.clone(),
                            path: path.clone(),
                            content: Box::new(id.clone()),
                            error,
                        }
                    })?)
                    .map_err(|error| ContentLoadError::SkillDefinition {
                        package: package.manifest.id.clone(),
                        path: path.clone(),
                        content: Box::new(id.clone()),
                        error,
                    })?;
            }
            if let Some(cost) = technique.activation_cost {
                definition = definition.with_activation_cost(
                    TechniqueActivationCost::new(
                        cost.energy,
                        cost.heat,
                        cost.persistent_bandwidth,
                        cost.active_limit,
                    )
                    .map_err(|error| ContentLoadError::SkillDefinition {
                        package: package.manifest.id.clone(),
                        path: path.clone(),
                        content: Box::new(id.clone()),
                        error,
                    })?,
                );
            }
            if let Some(item) = technique.manifestation_item {
                definition =
                    definition.with_manifestation_item(parse_content_id(package, &path, &item)?);
            }
            if let Some(profile) = technique.manifestation_profile {
                definition = definition
                    .with_manifestation_profile(parse_content_id(package, &path, &profile)?);
            }
            if let Some(cost) = technique.material_cost {
                let item = parse_content_id(package, &path, &cost.item)?;
                definition = definition
                    .with_material_cost(TechniqueMaterialCost::new(item, cost.quantity).map_err(
                        |error| ContentLoadError::SkillDefinition {
                            package: package.manifest.id.clone(),
                            path: path.clone(),
                            content: Box::new(id.clone()),
                            error,
                        },
                    )?)
                    .map_err(|error| ContentLoadError::SkillDefinition {
                        package: package.manifest.id.clone(),
                        path: path.clone(),
                        content: Box::new(id.clone()),
                        error,
                    })?;
            }
            for cost in technique.additional_material_costs {
                let item = parse_content_id(package, &path, &cost.item)?;
                definition = definition
                    .with_additional_material_cost(
                        TechniqueMaterialCost::new(item, cost.quantity).map_err(|error| {
                            ContentLoadError::SkillDefinition {
                                package: package.manifest.id.clone(),
                                path: path.clone(),
                                content: Box::new(id.clone()),
                                error,
                            }
                        })?,
                    )
                    .map_err(|error| ContentLoadError::SkillDefinition {
                        package: package.manifest.id.clone(),
                        path: path.clone(),
                        content: Box::new(id.clone()),
                        error,
                    })?;
            }
            if let Some(tool) = technique.required_tool {
                definition =
                    definition.with_required_tool(parse_content_id(package, &path, &tool)?);
            }
            if let Some(item) = technique.produced_item {
                definition =
                    definition.with_produced_item(parse_content_id(package, &path, &item)?);
            }
            if let Some(requirement) = engagement_requirement {
                definition = definition
                    .with_engagement_requirement(requirement)
                    .map_err(|error| ContentLoadError::SkillDefinition {
                        package: package.manifest.id.clone(),
                        path: path.clone(),
                        content: Box::new(id.clone()),
                        error,
                    })?;
            }
            if let Some(effect) = technique.on_hit_effect {
                let status = parse_content_id(package, &path, effect.status())?;
                if effect.stacks() == 0 {
                    return Err(ContentLoadError::SkillDefinition {
                        package: package.manifest.id.clone(),
                        path: path.clone(),
                        content: Box::new(id.clone()),
                        error: SkillDefinitionError::ZeroOnHitStatusStacks,
                    });
                }
                definition = definition
                    .with_on_hit_effect(effect.into_runtime(status).map_err(|error| {
                        ContentLoadError::SkillDefinition {
                            package: package.manifest.id.clone(),
                            path: path.clone(),
                            content: Box::new(id.clone()),
                            error,
                        }
                    })?)
                    .map_err(|error| ContentLoadError::SkillDefinition {
                        package: package.manifest.id.clone(),
                        path: path.clone(),
                        content: Box::new(id.clone()),
                        error,
                    })?;
            }
            if let Some(improvement) = technique.improvement {
                definition = definition
                    .with_improvement(improvement.into_runtime())
                    .map_err(|error| ContentLoadError::SkillDefinition {
                        package: package.manifest.id.clone(),
                        path: path.clone(),
                        content: Box::new(id.clone()),
                        error,
                    })?;
            }
            if let Some(action_kind) = technique.action_kind {
                definition = definition
                    .with_action_kind(action_kind.into_runtime())
                    .map_err(|error| ContentLoadError::SkillDefinition {
                        package: package.manifest.id.clone(),
                        path: path.clone(),
                        content: Box::new(id.clone()),
                        error,
                    })?;
            }
            if let Some(preparation_steps) = technique.preparation_time_units {
                definition = definition
                    .with_preparation_steps(preparation_steps)
                    .map_err(|error| ContentLoadError::SkillDefinition {
                        package: package.manifest.id.clone(),
                        path: path.clone(),
                        content: Box::new(id.clone()),
                        error,
                    })?;
            }
            if let Some(cooldown_turns) = technique.cooldown_turns {
                definition = definition.with_cooldown(cooldown_turns).map_err(|error| {
                    ContentLoadError::SkillDefinition {
                        package: package.manifest.id.clone(),
                        path: path.clone(),
                        content: Box::new(id.clone()),
                        error,
                    }
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
    #[serde(default)]
    modifiers: Vec<RawStatusModifier>,
    family: Option<String>,
    #[serde(default)]
    blocked_families: Vec<String>,
    expiration_transition: Option<RawStatusTransition>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawStatusTransition {
    status: String,
    #[serde(default = "one_u16")]
    stacks: u16,
}

impl RawStatusDefinition {
    fn into_runtime(
        self,
        id: StatusId,
        family: Option<ContentId>,
        blocked_families: Vec<ContentId>,
        expiration_transition: Option<StatusTransition>,
    ) -> Result<StatusDefinition, StatusDefinitionError> {
        let hooks = self
            .hooks
            .into_iter()
            .map(RawStatusHook::into_runtime)
            .collect();
        let mut definition =
            StatusDefinition::new(id, self.duration_turns, self.stacking.into_runtime(), hooks)?
                .with_modifiers(
                    self.modifiers
                        .into_iter()
                        .map(RawStatusModifier::into_runtime),
                )?;
        if let Some(family) = family {
            definition = definition.with_family(family);
        }
        definition = definition.with_blocked_families(blocked_families);
        if let Some(transition) = expiration_transition {
            definition = definition.with_expiration_transition(transition);
        }
        Ok(definition)
    }
}

#[derive(Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case", deny_unknown_fields)]
enum RawStatusStacking {
    KeepExisting,
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
            Self::KeepExisting => StatusStacking::KeepExisting,
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

#[derive(Clone, Copy, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawStatusModifier {
    ArmorFragilization { amount: u16 },
    Stability { amount: i16 },
    MovementTimeMinimum { time_units: u16 },
    Accuracy { amount: i16 },
}

impl RawStatusModifier {
    const fn into_runtime(self) -> StatusModifier {
        match self {
            Self::ArmorFragilization { amount } => StatusModifier::ArmorFragilization { amount },
            Self::Stability { amount } => StatusModifier::Stability { amount },
            Self::MovementTimeMinimum { time_units } => {
                StatusModifier::MovementTimeMinimum { time_units }
            }
            Self::Accuracy { amount } => StatusModifier::Accuracy { amount },
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
    DealDamageToCounterpart {
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
            Self::DealDamageToCounterpart {
                amount,
                damage_type,
                penetration,
                multiply_by_stacks,
            } => StatusEffectPrimitive::DealDamageToCounterpart {
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
    mass_grams: Option<u32>,
    ammunition_capacity: Option<u16>,
    power_draw: Option<u16>,
    attack: RawWeaponAttack,
    #[serde(default)]
    effects: Vec<RawWeaponEffect>,
    #[serde(default)]
    capabilities: RawWeaponCapabilities,
}

impl RawWeaponDefinition {
    fn into_runtime(
        self,
        id: WeaponId,
        statuses: &StatusCatalog,
    ) -> Result<WeaponDefinition, WeaponDefinitionError> {
        let mass_grams = self.mass_grams;
        let ammunition_capacity = self.ammunition_capacity;
        let power_draw = self.power_draw;
        let attack = self.attack.into_runtime()?;
        let effects = self
            .effects
            .into_iter()
            .map(|effect| effect.into_runtime(statuses))
            .collect::<Result<Vec<_>, _>>()?;
        WeaponDefinition::new(id, self.name_key, self.description_key, attack).and_then(
            |definition| {
                let definition = definition
                    .with_effects(effects)
                    .with_capabilities(self.capabilities.into_runtime());
                let definition = match mass_grams {
                    Some(mass_grams) => definition.with_mass_grams(mass_grams),
                    None => Ok(definition),
                }?;
                let definition = match ammunition_capacity {
                    Some(capacity) => definition.with_ammunition_capacity(capacity),
                    None => Ok(definition),
                }?;
                match power_draw {
                    Some(power_draw) => definition.with_power_draw(power_draw),
                    None => Ok(definition),
                }
            },
        )
    }
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWeaponCapabilities {
    #[serde(default)]
    melee_parry: bool,
    #[serde(default)]
    automatic_fire: bool,
}

impl RawWeaponCapabilities {
    const fn into_runtime(self) -> crate::weapon::WeaponCapabilities {
        let mut capabilities = crate::weapon::WeaponCapabilities::new();
        if self.melee_parry {
            capabilities = capabilities.with_melee_parry();
        }
        if self.automatic_fire {
            capabilities = capabilities.with_automatic_fire();
        }
        capabilities
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWeaponAttack {
    range: u16,
    distance_metric: RawDistanceMetric,
    requires_line_of_sight: bool,
    damage: RawAttackDamage,
    #[serde(default)]
    delivery: Option<RawAttackDelivery>,
    #[serde(default)]
    accuracy_modifier: i16,
    #[serde(default)]
    area: RawAttackArea,
    impact: Option<RawMeleeImpactProfile>,
    recovery_time_units: Option<u16>,
    preparation_disruption: Option<RawPreparationDisruption>,
}

impl RawWeaponAttack {
    fn into_runtime(self) -> Result<crate::combat::AttackProfile, WeaponDefinitionError> {
        let damage = self
            .damage
            .into_runtime()
            .map_err(WeaponDefinitionError::InvalidDamage)?;
        let primary = damage.primary_component();
        let mut attack = crate::combat::AttackProfile::new(
            self.range,
            self.distance_metric.into_runtime(),
            self.requires_line_of_sight,
            primary.damage_type,
            primary.amount,
            primary.penetration,
        )
        .with_damage(damage)
        .with_accuracy_modifier(self.accuracy_modifier);
        if let Some(delivery) = self.delivery {
            attack = attack.with_delivery(delivery.into_runtime());
        }
        if let Some(impact) = self.impact {
            attack = attack
                .with_melee_impact(crate::combat::MeleeImpactProfile::new(
                    impact.material_cap,
                    impact.modifier,
                ))
                .map_err(WeaponDefinitionError::InvalidImpact)?;
        }
        if let Some(recovery) = self.recovery_time_units {
            attack = attack.with_recovery_after_attack(
                TimeUnits::new(recovery).map_err(WeaponDefinitionError::InvalidRecovery)?,
            );
        }
        if let Some(disruption) = self.preparation_disruption {
            attack = attack.with_preparation_disruption(
                crate::combat::PreparationDisruption::new(
                    disruption.family.into_runtime(),
                    disruption.intensity,
                )
                .map_err(WeaponDefinitionError::InvalidImpact)?,
            );
        }
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

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawPreparationDisruption {
    family: RawPreparationDisruptionFamily,
    intensity: u16,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawPreparationDisruptionFamily {
    SystemShock,
}

impl RawPreparationDisruptionFamily {
    const fn into_runtime(self) -> crate::combat::PreparationDisruptionFamily {
        match self {
            Self::SystemShock => crate::combat::PreparationDisruptionFamily::SystemShock,
        }
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
enum RawAttackDamage {
    Single(RawWeaponDamage),
    Mixed(RawMixedDamage),
}

impl RawAttackDamage {
    fn into_runtime(self) -> Result<DamageImpact, crate::combat::DamageImpactError> {
        match self {
            Self::Single(damage) => Ok(DamageImpact::single(damage.into_runtime())),
            Self::Mixed(damage) => damage.into_runtime(),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMixedDamage {
    components: Vec<RawDamageComponent>,
    #[serde(default)]
    armor_penetration: u16,
    #[serde(default)]
    resistance_penetrations: Vec<RawResistancePenetration>,
}

impl RawMixedDamage {
    fn into_runtime(self) -> Result<DamageImpact, crate::combat::DamageImpactError> {
        DamageImpact::mixed(
            self.components.into_iter().map(|component| {
                DamageComponent::new(component.amount, component.damage_type.into_runtime())
            }),
            self.armor_penetration,
            self.resistance_penetrations.into_iter().map(|penetration| {
                (
                    penetration.damage_type.into_runtime(),
                    penetration.percentage_points,
                )
            }),
        )
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawDamageComponent {
    amount: u16,
    damage_type: RawDamageType,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawResistancePenetration {
    damage_type: RawDamageType,
    percentage_points: u16,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMeleeImpactProfile {
    material_cap: u16,
    #[serde(default)]
    modifier: i16,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawAttackDelivery {
    Melee,
    Ranged,
}

impl RawAttackDelivery {
    const fn into_runtime(self) -> AttackDelivery {
        match self {
            Self::Melee => AttackDelivery::Melee,
            Self::Ranged => AttackDelivery::Ranged,
        }
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
        #[serde(default)]
        trigger: Option<RawWeaponEffectTrigger>,
    },
    CreateGroundEffect {
        id: String,
        duration_turns: u16,
        damage_each_turn: RawWeaponDamage,
        #[serde(default)]
        trigger: Option<RawWeaponEffectTrigger>,
    },
}

#[derive(Clone, Copy, Deserialize)]
enum RawWeaponEffectTrigger {
    #[serde(rename = "on_attack")]
    Attack,
    #[serde(rename = "on_hit")]
    Hit,
    #[serde(rename = "on_damage")]
    Damage,
    #[serde(rename = "on_target_destroyed")]
    TargetDestroyed,
}

impl RawWeaponEffectTrigger {
    const fn into_runtime(self) -> WeaponEffectTrigger {
        match self {
            Self::Attack => WeaponEffectTrigger::OnAttack,
            Self::Hit => WeaponEffectTrigger::OnHit,
            Self::Damage => WeaponEffectTrigger::OnDamage,
            Self::TargetDestroyed => WeaponEffectTrigger::OnTargetDestroyed,
        }
    }
}

impl RawWeaponEffect {
    fn into_runtime(self, statuses: &StatusCatalog) -> Result<WeaponEffect, WeaponDefinitionError> {
        match self {
            Self::ApplyStatus {
                status,
                stacks,
                trigger,
            } => {
                let status: StatusId = status
                    .parse()
                    .map_err(WeaponDefinitionError::InvalidEffectId)?;
                if !statuses.contains(&status) {
                    return Err(WeaponDefinitionError::UnknownStatus(status));
                }
                let effect = ApplyStatusEffect::new(status, stacks)
                    .map_err(WeaponDefinitionError::InvalidStatusEffect)?;
                match trigger {
                    Some(trigger) => WeaponEffect::apply_status(effect, trigger.into_runtime())
                        .map_err(WeaponDefinitionError::InvalidEffectTrigger),
                    None => Ok(WeaponEffect::legacy_apply_status(effect)),
                }
            }
            Self::CreateGroundEffect {
                id,
                duration_turns,
                damage_each_turn,
                trigger,
            } => {
                let id = id.parse().map_err(WeaponDefinitionError::InvalidEffectId)?;
                let effect = GroundEffectSpec::new(
                    id,
                    duration_turns,
                    DamagePacket::new(
                        damage_each_turn.amount,
                        damage_each_turn.damage_type.into_runtime(),
                        damage_each_turn.penetration,
                    ),
                )
                .map_err(WeaponDefinitionError::InvalidGroundEffect)?;
                Ok(match trigger {
                    Some(trigger) => {
                        WeaponEffect::create_ground_effect(effect, trigger.into_runtime())
                    }
                    None => WeaponEffect::legacy_create_ground_effect(effect),
                })
            }
        }
    }
}

const fn one_u16() -> u16 {
    1
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawWeaponDamage {
    amount: u16,
    damage_type: RawDamageType,
    #[serde(default)]
    penetration: u16,
}

impl RawWeaponDamage {
    const fn into_runtime(self) -> DamagePacket {
        DamagePacket::new(
            self.amount,
            self.damage_type.into_runtime(),
            self.penetration,
        )
    }
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
    mass_grams: Option<u32>,
    equipment: Option<RawEquipmentProfile>,
    #[serde(default)]
    effects: Vec<RawItemEffect>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawEquipmentProfile {
    slot: String,
    armor: u16,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCharacterClassDefinition {
    id: String,
    name_key: String,
    role_key: String,
    description_key: String,
    recommended_attributes: RawPrimaryAttributes,
    starting_weapons: Vec<String>,
    #[serde(default)]
    starting_items: Vec<RawClassStartingItem>,
    #[serde(default)]
    starting_equipment: Vec<Option<String>>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawClassStartingItem {
    item: String,
    quantity: u16,
}

impl RawItemDefinition {
    fn into_runtime(
        self,
        id: ItemId,
        equipment: Option<EquipmentProfile>,
    ) -> Result<ItemDefinition, ItemDefinitionError> {
        let mass_grams = self.mass_grams;
        let definition = ItemDefinition::new(
            id,
            self.name_key,
            self.description_key,
            self.maximum_stack,
            self.kind.into_runtime(),
            equipment,
            self.effects
                .into_iter()
                .map(RawItemEffect::into_runtime)
                .collect(),
        )?;
        match mass_grams {
            Some(mass_grams) => definition.with_mass_grams(mass_grams),
            None => Ok(definition),
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawItemKind {
    Armor,
    Consumable,
    Material,
}

impl RawItemKind {
    const fn into_runtime(self) -> ItemKind {
        match self {
            Self::Armor => ItemKind::Armor,
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
    #[serde(alias = "minimum_rank")]
    minimum_level: u16,
    #[serde(default)]
    minimum_attributes: RawTechniqueMinimumAttributes,
    kind: RawTechniqueKind,
    prerequisite: Option<String>,
    #[serde(default)]
    required_features: Vec<String>,
    action: Option<RawTechniqueAction>,
    activation_cost: Option<RawTechniqueActivationCost>,
    manifestation_item: Option<String>,
    manifestation_profile: Option<String>,
    material_cost: Option<RawTechniqueMaterialCost>,
    #[serde(default)]
    additional_material_costs: Vec<RawTechniqueMaterialCost>,
    required_tool: Option<String>,
    produced_item: Option<String>,
    improvement: Option<RawTechniqueImprovement>,
    on_hit_effect: Option<RawTechniqueOnHitEffect>,
    engagement_requirement: Option<RawTechniqueEngagementRequirement>,
    action_kind: Option<RawActionKind>,
    preparation_time_units: Option<u16>,
    cooldown_turns: Option<u16>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTechniqueActivationCost {
    #[serde(default)]
    energy: u16,
    #[serde(default)]
    heat: u16,
    #[serde(default)]
    persistent_bandwidth: u16,
    active_limit: Option<u8>,
}

#[derive(Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTechniqueMinimumAttributes {
    power: Option<u8>,
    coordination: Option<u8>,
    resilience: Option<u8>,
    perception: Option<u8>,
    processing: Option<u8>,
}

impl RawTechniqueMinimumAttributes {
    fn into_runtime(self) -> Result<Vec<TechniqueAttributeRequirement>, SkillDefinitionError> {
        [
            (PrimaryAttribute::Power, self.power),
            (PrimaryAttribute::Coordination, self.coordination),
            (PrimaryAttribute::Resilience, self.resilience),
            (PrimaryAttribute::Perception, self.perception),
            (PrimaryAttribute::Processing, self.processing),
        ]
        .into_iter()
        .filter_map(|(attribute, minimum)| minimum.map(|minimum| (attribute, minimum)))
        .map(|(attribute, minimum)| TechniqueAttributeRequirement::new(attribute, minimum))
        .collect()
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTechniqueMaterialCost {
    item: String,
    quantity: u16,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawTechniqueEngagementRequirement {
    TargetHasAnyStatusFamily { families: Vec<String> },
    TargetHasKnownPhysicalWeakness,
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawTechniqueOnHitEffect {
    ApplyStatus {
        status: String,
        #[serde(default = "one_u16")]
        stacks: u16,
        target_requirement: RawTechniqueTargetRequirement,
        resistance: Option<RawTechniqueEffectResistance>,
    },
}

impl RawTechniqueOnHitEffect {
    fn status(&self) -> &str {
        match self {
            Self::ApplyStatus { status, .. } => status,
        }
    }

    const fn stacks(&self) -> u16 {
        match self {
            Self::ApplyStatus { stacks, .. } => *stacks,
        }
    }

    fn into_runtime(self, status: StatusId) -> Result<TechniqueOnHitEffect, SkillDefinitionError> {
        match self {
            Self::ApplyStatus {
                stacks,
                target_requirement,
                resistance,
                ..
            } => {
                let mut effect = TechniqueOnHitEffect::new(
                    ApplyStatusEffect::new(status, stacks)
                        .expect("positive on-hit status stacks were validated"),
                    target_requirement.into_runtime(),
                );
                if let Some(RawTechniqueEffectResistance::Stability { intensity }) = resistance {
                    effect = effect.with_stability_resistance(intensity)?;
                }
                Ok(effect)
            }
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawTechniqueEffectResistance {
    Stability { intensity: u16 },
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawTechniqueTargetRequirement {
    #[serde(rename = "has_armor")]
    Armor,
    #[serde(rename = "has_compatible_locomotion")]
    CompatibleLocomotion,
    #[serde(rename = "has_compatible_suppression_response")]
    CompatibleSuppressionResponse,
}

impl RawTechniqueTargetRequirement {
    const fn into_runtime(self) -> TechniqueTargetRequirement {
        match self {
            Self::Armor => TechniqueTargetRequirement::HasArmor,
            Self::CompatibleLocomotion => TechniqueTargetRequirement::HasCompatibleLocomotion,
            Self::CompatibleSuppressionResponse => {
                TechniqueTargetRequirement::HasCompatibleSuppressionResponse
            }
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawTechniqueImprovement {
    NpcVisionOverlay,
    MeleeCounterattack,
    ExtendedRangedOverwatch,
    PersistentRangedAim {
        retained_accuracy_modifier: i16,
    },
    ControlledChargeInertia,
    CoveredApproach {
        optical_difficulty_bonus: i16,
    },
    SilentNeutralization {
        physical_damage_percentage: u16,
        #[serde(default)]
        extra_energy_cost: u16,
        #[serde(default)]
        noise_reduction: u16,
    },
    DroneAutonomousScout {
        maximum_unknown_steps: u8,
        energy_cost_override: u16,
        additional_bandwidth: u16,
    },
}

impl RawTechniqueImprovement {
    const fn into_runtime(self) -> TechniqueImprovement {
        match self {
            Self::NpcVisionOverlay => TechniqueImprovement::NpcVisionOverlay,
            Self::MeleeCounterattack => TechniqueImprovement::MeleeCounterattack,
            Self::ExtendedRangedOverwatch => TechniqueImprovement::ExtendedRangedOverwatch,
            Self::PersistentRangedAim {
                retained_accuracy_modifier,
            } => TechniqueImprovement::PersistentRangedAim {
                retained_accuracy_modifier,
            },
            Self::ControlledChargeInertia => TechniqueImprovement::ControlledChargeInertia,
            Self::CoveredApproach {
                optical_difficulty_bonus,
            } => TechniqueImprovement::CoveredApproach {
                optical_difficulty_bonus,
            },
            Self::SilentNeutralization {
                physical_damage_percentage,
                extra_energy_cost,
                noise_reduction,
            } => TechniqueImprovement::SilentNeutralization {
                physical_damage_percentage,
                extra_energy_cost,
                noise_reduction,
            },
            Self::DroneAutonomousScout {
                maximum_unknown_steps,
                energy_cost_override,
                additional_bandwidth,
            } => TechniqueImprovement::DroneAutonomousScout {
                maximum_unknown_steps,
                energy_cost_override,
                additional_bandwidth,
            },
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawActionKind {
    Offensive,
    Support,
}

impl RawActionKind {
    const fn into_runtime(self) -> ActionKind {
        match self {
            Self::Offensive => ActionKind::Offensive,
            Self::Support => ActionKind::Support,
        }
    }
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
    InspectNearbySecrets {
        radius: u16,
        detection_bonus: i16,
    },
    AnalyzeNearbyWalls {
        radius: u16,
        maximum_tiles: u8,
    },
    AnalyzeThreat {
        range: u16,
    },
    DiagnoseEnergy {
        range: u16,
        analysis_bonus: i16,
        energy_cost: u16,
    },
    RepairComponent {
        durability_restored: u16,
        #[serde(default)]
        energy_cost: u16,
    },
    SalvageComponent,
    DiagnoseComponent {
        analysis_bonus: i16,
        #[serde(default)]
        energy_cost: u16,
    },
    TuneModule {
        economy_output_percentage: u16,
        economy_energy_percentage: u16,
        power_output_percentage: u16,
        power_energy_percentage: u16,
    },
    EmergencyRepairComponent {
        durability_restored: u16,
    },
    OverclockModule {
        output_percentage: u16,
        usage_energy_percentage: u16,
        heat_per_use: u16,
        safe_heat_threshold: u16,
        maximum_heat_threshold: u16,
        duration_time_units: u16,
        activation_energy: u16,
        durability_damage_when_hot: u16,
    },
    BypassComponent {
        restored_output_percentage: u16,
        #[serde(default)]
        energy_cost: u16,
    },
    ReconditionModule {
        durability_restored: u16,
    },
    AssembleFieldBeacon {
        integrity: u16,
        battery_energy: u16,
        energy_per_phase: u16,
        noise_intensity: u16,
    },
    WeaponAttack {
        required_delivery: RawAttackDelivery,
        #[serde(default)]
        physical_damage_percentage: Option<u16>,
        #[serde(default)]
        armor_penetration_bonus: u16,
        #[serde(default)]
        accuracy_modifier: i16,
        #[serde(default)]
        energy_cost: u16,
        recovery_time_units: Option<u16>,
        forced_movement: Option<RawForcedMovement>,
        melee_arc: Option<RawMeleeArc>,
    },
    WeaponVolley {
        projectiles: u8,
        maximum_targets: u8,
        #[serde(default)]
        maximum_target_separation: Option<u16>,
        #[serde(default)]
        accuracy_modifier: i16,
        #[serde(default)]
        energy_cost: u16,
        #[serde(default)]
        requires_automatic_fire: bool,
    },
    WeaponComponentAttack {
        required_delivery: RawAttackDelivery,
        #[serde(default)]
        accuracy_modifier: i16,
        #[serde(default)]
        energy_cost: u16,
    },
    WeaponBarrage {
        stages: u8,
        cells: u8,
        #[serde(default)]
        accuracy_modifier: i16,
        #[serde(default)]
        energy_cost_per_stage: u16,
        #[serde(default)]
        requires_automatic_fire: bool,
    },
    PrepareRangedOverwatch {
        maximum_line_cells: u8,
    },
    PrepareMeleeParry {
        physical_reduction_percentage: u8,
        #[serde(default)]
        trigger_energy_cost: u16,
    },
    PrepareMeleeInterception,
    DeployExplosive {
        deployment: RawExplosiveDeployment,
        primary_payload: RawExplosivePayload,
        secondary_payload: Option<RawSecondaryExplosivePayload>,
    },
    NeutralizeExplosive {
        range: u16,
        #[serde(default)]
        analysis_bonus: i16,
        #[serde(default)]
        energy_cost: u16,
    },
    RecoverNeutralizedExplosive {
        range: u16,
    },
    TriggerRemoteExplosive {
        range: u16,
        #[serde(default)]
        energy_cost: u16,
        #[serde(default)]
        bandwidth_required: u16,
    },
    ProgramExplosives {
        range: u16,
        maximum_devices: u8,
        minimum_delay: u16,
        maximum_delay: u16,
        #[serde(default)]
        energy_cost: u16,
        #[serde(default)]
        bandwidth_required: u16,
    },
    CautiousMove {
        interception_evasion_modifier: i16,
    },
    PrepareAnchor {
        displacement_resistance_bonus: u16,
    },
    TraverseSingleObstacle {
        maximum_distance: u16,
        #[serde(default)]
        energy_cost: u16,
    },
    ChargeAttack {
        minimum_advance: u8,
        maximum_advance: u8,
        physical_damage_percentage: u16,
        energy_per_step: u16,
        recovery_time_units: u16,
    },
    PrepareEvasiveStep {
        #[serde(default)]
        trigger_energy_cost: u16,
    },
    PropelledMove {
        distance: u8,
        #[serde(default)]
        energy_cost: u16,
        #[serde(default)]
        heat_generated: u16,
    },
    Breakthrough {
        #[serde(default)]
        impact_modifier: i16,
        #[serde(default)]
        energy_cost: u16,
        recovery_time_units: u16,
    },
    ExtractAlly {
        #[serde(default)]
        energy_cost: u16,
    },
    SilentMove {
        #[serde(default)]
        noise_reduction: u16,
        minimum_time_units: u16,
    },
    ToggleEmissionSilence {
        channel: RawSignatureChannel,
    },
    ToggleLowProfile {
        optical_difficulty_bonus: i16,
        minimum_movement_time_units: u16,
    },
    AmbushAttack {
        accuracy_modifier: i16,
        physical_damage_percentage: u16,
    },
    DeploySoundDecoy {
        range: u16,
        intensity: u16,
        duration_phases: u16,
        integrity: u16,
    },
    BreakTrail {
        #[serde(default)]
        energy_cost: u16,
        maximum_steps: u8,
        maximum_duration: u16,
    },
    CamouflageExplosive {
        range: u16,
        optical_difficulty_bonus: i16,
    },
    ToggleActiveCamouflage {
        channel: RawSignatureChannel,
        optical_difficulty_bonus: i16,
        maximum_duration: u16,
        activation_energy: u16,
        upkeep_energy: u16,
        heat_per_phase: u16,
    },
    ManifestDrone {
        integrity: u16,
        energy_capacity: u16,
        starting_energy: u16,
        link_range: u16,
        link_power: u16,
        link_difficulty: u16,
        link_attenuation_per_cell: u16,
        link_wall_attenuation_multiplier: u16,
        sensor_radius: u16,
        bandwidth_required: u16,
        movement_energy_cost: u16,
        manipulator_capacity_grams: u32,
        decoy_intensity: u16,
        attack_range: u16,
        attack_damage: u16,
    },
    DroneEscort {
        link_range: u16,
        minimum_distance: u8,
        maximum_distance: u8,
        energy_cost: u16,
    },
    DronePatrol {
        link_range: u16,
        maximum_waypoints: u8,
        energy_cost: u16,
    },
    DroneMobileDecoy {
        link_range: u16,
        controller_energy_cost: u16,
        drone_energy_per_phase: u16,
        intensity: u16,
        maximum_duration: u16,
    },
    DroneCollect {
        link_range: u16,
        energy_cost: u16,
    },
    DroneCoordinateFire {
        link_range: u16,
        maximum_drones: u8,
        energy_cost: u16,
        transmission_bandwidth: u16,
    },
    DroneInterpose {
        link_range: u16,
        controller_energy_cost: u16,
        drone_trigger_energy_cost: u16,
    },
    DroneConditionalRoutine {
        link_range: u16,
        energy_cost: u16,
        additional_bandwidth: u16,
    },
    DroneCoordinatedDeployment {
        link_range: u16,
        maximum_drones: u8,
        energy_cost: u16,
        transmission_bandwidth: u16,
    },
    DroneEmergencyReturn {
        link_range: u16,
        maximum_drones: u8,
        energy_cost: u16,
        transmission_bandwidth: u16,
        duration_phases: u16,
    },
    ProbeInterface {
        range: u16,
        analysis_bonus: i16,
        energy_cost: u16,
        audit_delay: u16,
    },
    ForceElectronicLock {
        range: u16,
        energy_cost: u16,
        bandwidth_required: u16,
        failure_hardening_duration: u16,
        audit_delay: u16,
    },
    ExtractData {
        range: u16,
        energy_cost: u16,
    },
    SpoofAuthorization {
        range: u16,
        energy_cost: u16,
        duration_time_units: u16,
    },
    DivertDevice {
        range: u16,
        energy_cost: u16,
        additional_bandwidth: u16,
        duration_time_units: u16,
    },
    SuspendDigitalRoutine {
        range: u16,
        energy_cost: u16,
        duration_time_units: u16,
        repeat_protection_time_units: u16,
    },
    MaintainBackdoor {
        range: u16,
        installation_energy_cost: u16,
        reconnection_energy_cost: u16,
        maximum_backdoors: u8,
        session_duration_time_units: u16,
    },
    FalsifySecurityTrace {
        range: u16,
        energy_cost: u16,
    },
    DivertSubnet {
        range: u16,
        maximum_devices: u8,
        energy_cost: u16,
        bandwidth_per_device: u16,
        duration_time_units: u16,
    },
    LockDeviceControl {
        range: u16,
        energy_cost: u16,
        additional_bandwidth: u16,
        duration_time_units: u16,
    },
    ElectronicPulse {
        radius: u16,
        damage: u16,
        energy_cost: u16,
        heat_generated: u16,
        #[serde(default)]
        disruption_intensity: u16,
        #[serde(default)]
        directional: bool,
        #[serde(default)]
        filter_identified_allies: bool,
        #[serde(default)]
        bandwidth_required: u16,
    },
    ImplantOverheat {
        range: u16,
        energy_cost: u16,
        heat_generated: u16,
        bandwidth_required: u16,
        heat_per_tick: u16,
        dissipation_penalty: u16,
        duration_time_units: u16,
        audit_delay: u16,
    },
    MaintainJamming {
        radius: u16,
        penalty: u16,
        activation_energy: u16,
        energy_per_phase: u16,
        heat_per_phase: u16,
        bandwidth_required: u16,
        maximum_duration: u16,
    },
    PurgeHostileProgram {
        range: u16,
        energy_cost: u16,
        intrusion_bonus: i16,
    },
    ElectronicCascade {
        range: u16,
        jump_range: u16,
        maximum_targets: u8,
        damage_by_target: [u16; 4],
        energy_cost: u16,
        heat_generated: u16,
    },
    ImplantInfection {
        range: u16,
        propagation_range: u16,
        energy_cost: u16,
        heat_generated: u16,
        bandwidth_required: u16,
        thermal_damage_per_tick: u16,
        ticks_per_host: u16,
        maximum_hosts: u8,
        transmissions_per_host: u8,
        campaign_duration: u16,
        audit_delay: u16,
    },
    DeploySaturationBeacon {
        radius: u16,
        damage: u16,
        duration_time_units: u16,
        integrity: u16,
        battery_energy: u16,
        energy_per_phase: u16,
        #[serde(default)]
        manual_activation: bool,
        #[serde(default)]
        activation_energy: u16,
        #[serde(default)]
        activation_bandwidth: u16,
        #[serde(default)]
        activation_link_range: u16,
    },
    ImplantImplosion {
        range: u16,
        energy_cost: u16,
        heat_generated: u16,
        bandwidth_required: u16,
        minimum_stored_energy: u16,
        reserved_energy: u16,
        delay_time_units: u16,
        radius: u16,
        physical_damage: u16,
        thermal_damage: u16,
        audit_delay: u16,
    },
}

impl RawTechniqueAction {
    fn into_runtime(self) -> Result<TechniqueAction, SkillDefinitionError> {
        Ok(match self {
            Self::AnalyzeTarget { range } => TechniqueAction::AnalyzeTarget { range },
            Self::AnalyzeMultipleTargets {
                maximum_targets,
                energy_cost,
            } => TechniqueAction::AnalyzeMultipleTargets {
                maximum_targets,
                energy_cost,
            },
            Self::ReadMovementTraces { radius } => TechniqueAction::ReadMovementTraces { radius },
            Self::InspectNearbySecrets {
                radius,
                detection_bonus,
            } => TechniqueAction::InspectNearbySecrets {
                radius,
                detection_bonus,
            },
            Self::AnalyzeNearbyWalls {
                radius,
                maximum_tiles,
            } => TechniqueAction::AnalyzeNearbyWalls {
                radius,
                maximum_tiles,
            },
            Self::AnalyzeThreat { range } => TechniqueAction::AnalyzeThreat { range },
            Self::DiagnoseEnergy {
                range,
                analysis_bonus,
                energy_cost,
            } => TechniqueAction::DiagnoseEnergy {
                range,
                analysis_bonus,
                energy_cost,
            },
            Self::RepairComponent {
                durability_restored,
                energy_cost,
            } => TechniqueAction::RepairComponent {
                durability_restored,
                energy_cost,
            },
            Self::SalvageComponent => TechniqueAction::SalvageComponent,
            Self::DiagnoseComponent {
                analysis_bonus,
                energy_cost,
            } => TechniqueAction::DiagnoseComponent {
                analysis_bonus,
                energy_cost,
            },
            Self::TuneModule {
                economy_output_percentage,
                economy_energy_percentage,
                power_output_percentage,
                power_energy_percentage,
            } => TechniqueAction::TuneModule {
                economy_output_percentage,
                economy_energy_percentage,
                power_output_percentage,
                power_energy_percentage,
            },
            Self::EmergencyRepairComponent {
                durability_restored,
            } => TechniqueAction::EmergencyRepairComponent {
                durability_restored,
            },
            Self::OverclockModule {
                output_percentage,
                usage_energy_percentage,
                heat_per_use,
                safe_heat_threshold,
                maximum_heat_threshold,
                duration_time_units,
                activation_energy,
                durability_damage_when_hot,
            } => TechniqueAction::OverclockModule {
                output_percentage,
                usage_energy_percentage,
                heat_per_use,
                safe_heat_threshold,
                maximum_heat_threshold,
                duration_time_units,
                activation_energy,
                durability_damage_when_hot,
            },
            Self::BypassComponent {
                restored_output_percentage,
                energy_cost,
            } => TechniqueAction::BypassComponent {
                restored_output_percentage,
                energy_cost,
            },
            Self::ReconditionModule {
                durability_restored,
            } => TechniqueAction::ReconditionModule {
                durability_restored,
            },
            Self::AssembleFieldBeacon {
                integrity,
                battery_energy,
                energy_per_phase,
                noise_intensity,
            } => TechniqueAction::AssembleFieldBeacon {
                integrity,
                battery_energy,
                energy_per_phase,
                noise_intensity,
            },
            Self::WeaponAttack {
                required_delivery,
                physical_damage_percentage,
                armor_penetration_bonus,
                accuracy_modifier,
                energy_cost,
                recovery_time_units,
                forced_movement,
                melee_arc,
            } => TechniqueAction::WeaponAttack {
                required_delivery: required_delivery.into_runtime(),
                physical_damage_percentage,
                armor_penetration_bonus,
                accuracy_modifier,
                energy_cost,
                recovery_time_units,
                forced_movement: forced_movement.map(RawForcedMovement::into_runtime),
                melee_arc: match melee_arc {
                    Some(arc) => Some(MeleeArc::new(arc.maximum_cells).map_err(
                        |error| match error {
                            MeleeArcError::ZeroMaximumCells => {
                                SkillDefinitionError::ZeroMeleeArcCells
                            }
                            MeleeArcError::TooManyCells(cells) => {
                                SkillDefinitionError::TooManyMeleeArcCells(cells)
                            }
                        },
                    )?),
                    None => None,
                },
            },
            Self::WeaponVolley {
                projectiles,
                maximum_targets,
                maximum_target_separation,
                accuracy_modifier,
                energy_cost,
                requires_automatic_fire,
            } => TechniqueAction::WeaponVolley {
                projectiles,
                maximum_targets,
                maximum_target_separation,
                accuracy_modifier,
                energy_cost,
                requires_automatic_fire,
            },
            Self::WeaponComponentAttack {
                required_delivery,
                accuracy_modifier,
                energy_cost,
            } => TechniqueAction::WeaponComponentAttack {
                required_delivery: required_delivery.into_runtime(),
                accuracy_modifier,
                energy_cost,
            },
            Self::WeaponBarrage {
                stages,
                cells,
                accuracy_modifier,
                energy_cost_per_stage,
                requires_automatic_fire,
            } => TechniqueAction::WeaponBarrage {
                stages,
                cells,
                accuracy_modifier,
                energy_cost_per_stage,
                requires_automatic_fire,
            },
            Self::PrepareRangedOverwatch { maximum_line_cells } => {
                TechniqueAction::PrepareRangedOverwatch { maximum_line_cells }
            }
            Self::PrepareMeleeParry {
                physical_reduction_percentage,
                trigger_energy_cost,
            } => TechniqueAction::PrepareMeleeParry {
                physical_reduction_percentage,
                trigger_energy_cost,
            },
            Self::PrepareMeleeInterception => TechniqueAction::PrepareMeleeInterception,
            Self::DeployExplosive {
                deployment,
                primary_payload,
                secondary_payload,
            } => TechniqueAction::DeployExplosive {
                deployment: deployment.into_runtime()?,
                primary_payload: primary_payload.into_runtime()?,
                secondary_payload: secondary_payload
                    .map(RawSecondaryExplosivePayload::into_runtime)
                    .transpose()?,
            },
            Self::NeutralizeExplosive {
                range,
                analysis_bonus,
                energy_cost,
            } => TechniqueAction::NeutralizeExplosive {
                range,
                analysis_bonus,
                energy_cost,
            },
            Self::RecoverNeutralizedExplosive { range } => {
                TechniqueAction::RecoverNeutralizedExplosive { range }
            }
            Self::TriggerRemoteExplosive {
                range,
                energy_cost,
                bandwidth_required,
            } => TechniqueAction::TriggerRemoteExplosive {
                range,
                energy_cost,
                bandwidth_required,
            },
            Self::ProgramExplosives {
                range,
                maximum_devices,
                minimum_delay,
                maximum_delay,
                energy_cost,
                bandwidth_required,
            } => TechniqueAction::ProgramExplosives {
                range,
                maximum_devices,
                minimum_delay,
                maximum_delay,
                energy_cost,
                bandwidth_required,
            },
            Self::CautiousMove {
                interception_evasion_modifier,
            } => TechniqueAction::CautiousMove {
                interception_evasion_modifier,
            },
            Self::PrepareAnchor {
                displacement_resistance_bonus,
            } => TechniqueAction::PrepareAnchor {
                displacement_resistance_bonus,
            },
            Self::TraverseSingleObstacle {
                maximum_distance,
                energy_cost,
            } => TechniqueAction::TraverseSingleObstacle {
                maximum_distance,
                energy_cost,
            },
            Self::ChargeAttack {
                minimum_advance,
                maximum_advance,
                physical_damage_percentage,
                energy_per_step,
                recovery_time_units,
            } => TechniqueAction::ChargeAttack {
                minimum_advance,
                maximum_advance,
                physical_damage_percentage,
                energy_per_step,
                recovery_time_units,
            },
            Self::PrepareEvasiveStep {
                trigger_energy_cost,
            } => TechniqueAction::PrepareEvasiveStep {
                trigger_energy_cost,
            },
            Self::PropelledMove {
                distance,
                energy_cost,
                heat_generated,
            } => TechniqueAction::PropelledMove {
                distance,
                energy_cost,
                heat_generated,
            },
            Self::Breakthrough {
                impact_modifier,
                energy_cost,
                recovery_time_units,
            } => TechniqueAction::Breakthrough {
                impact_modifier,
                energy_cost,
                recovery_time_units,
            },
            Self::ExtractAlly { energy_cost } => TechniqueAction::ExtractAlly { energy_cost },
            Self::SilentMove {
                noise_reduction,
                minimum_time_units,
            } => TechniqueAction::SilentMove {
                noise_reduction,
                minimum_time_units,
            },
            Self::ToggleEmissionSilence { channel } => TechniqueAction::ToggleEmissionSilence {
                channel: channel.into_runtime(),
            },
            Self::ToggleLowProfile {
                optical_difficulty_bonus,
                minimum_movement_time_units,
            } => TechniqueAction::ToggleLowProfile {
                optical_difficulty_bonus,
                minimum_movement_time_units,
            },
            Self::AmbushAttack {
                accuracy_modifier,
                physical_damage_percentage,
            } => TechniqueAction::AmbushAttack {
                accuracy_modifier,
                physical_damage_percentage,
            },
            Self::DeploySoundDecoy {
                range,
                intensity,
                duration_phases,
                integrity,
            } => TechniqueAction::DeploySoundDecoy {
                range,
                intensity,
                duration_phases,
                integrity,
            },
            Self::BreakTrail {
                energy_cost,
                maximum_steps,
                maximum_duration,
            } => TechniqueAction::BreakTrail {
                energy_cost,
                maximum_steps,
                maximum_duration,
            },
            Self::CamouflageExplosive {
                range,
                optical_difficulty_bonus,
            } => TechniqueAction::CamouflageExplosive {
                range,
                optical_difficulty_bonus,
            },
            Self::ToggleActiveCamouflage {
                channel,
                optical_difficulty_bonus,
                maximum_duration,
                activation_energy,
                upkeep_energy,
                heat_per_phase,
            } => TechniqueAction::ToggleActiveCamouflage {
                channel: channel.into_runtime(),
                optical_difficulty_bonus,
                maximum_duration,
                activation_energy,
                upkeep_energy,
                heat_per_phase,
            },
            Self::ManifestDrone {
                integrity,
                energy_capacity,
                starting_energy,
                link_range,
                link_power,
                link_difficulty,
                link_attenuation_per_cell,
                link_wall_attenuation_multiplier,
                sensor_radius,
                bandwidth_required,
                movement_energy_cost,
                manipulator_capacity_grams,
                decoy_intensity,
                attack_range,
                attack_damage,
            } => TechniqueAction::ManifestDrone {
                integrity,
                energy_capacity,
                starting_energy,
                link_range,
                link_power,
                link_difficulty,
                link_attenuation_per_cell,
                link_wall_attenuation_multiplier,
                sensor_radius,
                bandwidth_required,
                movement_energy_cost,
                manipulator_capacity_grams,
                decoy_intensity,
                attack_range,
                attack_damage,
            },
            Self::DroneEscort {
                link_range,
                minimum_distance,
                maximum_distance,
                energy_cost,
            } => TechniqueAction::DroneEscort {
                link_range,
                minimum_distance,
                maximum_distance,
                energy_cost,
            },
            Self::DronePatrol {
                link_range,
                maximum_waypoints,
                energy_cost,
            } => TechniqueAction::DronePatrol {
                link_range,
                maximum_waypoints,
                energy_cost,
            },
            Self::DroneMobileDecoy {
                link_range,
                controller_energy_cost,
                drone_energy_per_phase,
                intensity,
                maximum_duration,
            } => TechniqueAction::DroneMobileDecoy {
                link_range,
                controller_energy_cost,
                drone_energy_per_phase,
                intensity,
                maximum_duration,
            },
            Self::DroneCollect {
                link_range,
                energy_cost,
            } => TechniqueAction::DroneCollect {
                link_range,
                energy_cost,
            },
            Self::DroneCoordinateFire {
                link_range,
                maximum_drones,
                energy_cost,
                transmission_bandwidth,
            } => TechniqueAction::DroneCoordinateFire {
                link_range,
                maximum_drones,
                energy_cost,
                transmission_bandwidth,
            },
            Self::DroneInterpose {
                link_range,
                controller_energy_cost,
                drone_trigger_energy_cost,
            } => TechniqueAction::DroneInterpose {
                link_range,
                controller_energy_cost,
                drone_trigger_energy_cost,
            },
            Self::DroneConditionalRoutine {
                link_range,
                energy_cost,
                additional_bandwidth,
            } => TechniqueAction::DroneConditionalRoutine {
                link_range,
                energy_cost,
                additional_bandwidth,
            },
            Self::DroneCoordinatedDeployment {
                link_range,
                maximum_drones,
                energy_cost,
                transmission_bandwidth,
            } => TechniqueAction::DroneCoordinatedDeployment {
                link_range,
                maximum_drones,
                energy_cost,
                transmission_bandwidth,
            },
            Self::DroneEmergencyReturn {
                link_range,
                maximum_drones,
                energy_cost,
                transmission_bandwidth,
                duration_phases,
            } => TechniqueAction::DroneEmergencyReturn {
                link_range,
                maximum_drones,
                energy_cost,
                transmission_bandwidth,
                duration_phases,
            },
            Self::ProbeInterface {
                range,
                analysis_bonus,
                energy_cost,
                audit_delay,
            } => TechniqueAction::ProbeInterface {
                range,
                analysis_bonus,
                energy_cost,
                audit_delay,
            },
            Self::ForceElectronicLock {
                range,
                energy_cost,
                bandwidth_required,
                failure_hardening_duration,
                audit_delay,
            } => TechniqueAction::ForceElectronicLock {
                range,
                energy_cost,
                bandwidth_required,
                failure_hardening_duration,
                audit_delay,
            },
            Self::ExtractData { range, energy_cost } => {
                TechniqueAction::ExtractData { range, energy_cost }
            }
            Self::SpoofAuthorization {
                range,
                energy_cost,
                duration_time_units,
            } => TechniqueAction::SpoofAuthorization {
                range,
                energy_cost,
                duration_time_units,
            },
            Self::DivertDevice {
                range,
                energy_cost,
                additional_bandwidth,
                duration_time_units,
            } => TechniqueAction::DivertDevice {
                range,
                energy_cost,
                additional_bandwidth,
                duration_time_units,
            },
            Self::SuspendDigitalRoutine {
                range,
                energy_cost,
                duration_time_units,
                repeat_protection_time_units,
            } => TechniqueAction::SuspendDigitalRoutine {
                range,
                energy_cost,
                duration_time_units,
                repeat_protection_time_units,
            },
            Self::MaintainBackdoor {
                range,
                installation_energy_cost,
                reconnection_energy_cost,
                maximum_backdoors,
                session_duration_time_units,
            } => TechniqueAction::MaintainBackdoor {
                range,
                installation_energy_cost,
                reconnection_energy_cost,
                maximum_backdoors,
                session_duration_time_units,
            },
            Self::FalsifySecurityTrace { range, energy_cost } => {
                TechniqueAction::FalsifySecurityTrace { range, energy_cost }
            }
            Self::DivertSubnet {
                range,
                maximum_devices,
                energy_cost,
                bandwidth_per_device,
                duration_time_units,
            } => TechniqueAction::DivertSubnet {
                range,
                maximum_devices,
                energy_cost,
                bandwidth_per_device,
                duration_time_units,
            },
            Self::LockDeviceControl {
                range,
                energy_cost,
                additional_bandwidth,
                duration_time_units,
            } => TechniqueAction::LockDeviceControl {
                range,
                energy_cost,
                additional_bandwidth,
                duration_time_units,
            },
            Self::ElectronicPulse {
                radius,
                damage,
                energy_cost,
                heat_generated,
                disruption_intensity,
                directional,
                filter_identified_allies,
                bandwidth_required,
            } => TechniqueAction::ElectronicPulse {
                radius,
                damage,
                energy_cost,
                heat_generated,
                disruption_intensity,
                directional,
                filter_identified_allies,
                bandwidth_required,
            },
            Self::ImplantOverheat {
                range,
                energy_cost,
                heat_generated,
                bandwidth_required,
                heat_per_tick,
                dissipation_penalty,
                duration_time_units,
                audit_delay,
            } => TechniqueAction::ImplantOverheat {
                range,
                energy_cost,
                heat_generated,
                bandwidth_required,
                heat_per_tick,
                dissipation_penalty,
                duration_time_units,
                audit_delay,
            },
            Self::MaintainJamming {
                radius,
                penalty,
                activation_energy,
                energy_per_phase,
                heat_per_phase,
                bandwidth_required,
                maximum_duration,
            } => TechniqueAction::MaintainJamming {
                radius,
                penalty,
                activation_energy,
                energy_per_phase,
                heat_per_phase,
                bandwidth_required,
                maximum_duration,
            },
            Self::PurgeHostileProgram {
                range,
                energy_cost,
                intrusion_bonus,
            } => TechniqueAction::PurgeHostileProgram {
                range,
                energy_cost,
                intrusion_bonus,
            },
            Self::ElectronicCascade {
                range,
                jump_range,
                maximum_targets,
                damage_by_target,
                energy_cost,
                heat_generated,
            } => TechniqueAction::ElectronicCascade {
                range,
                jump_range,
                maximum_targets,
                damage_by_target,
                energy_cost,
                heat_generated,
            },
            Self::ImplantInfection {
                range,
                propagation_range,
                energy_cost,
                heat_generated,
                bandwidth_required,
                thermal_damage_per_tick,
                ticks_per_host,
                maximum_hosts,
                transmissions_per_host,
                campaign_duration,
                audit_delay,
            } => TechniqueAction::ImplantInfection {
                range,
                propagation_range,
                energy_cost,
                heat_generated,
                bandwidth_required,
                thermal_damage_per_tick,
                ticks_per_host,
                maximum_hosts,
                transmissions_per_host,
                campaign_duration,
                audit_delay,
            },
            Self::DeploySaturationBeacon {
                radius,
                damage,
                duration_time_units,
                integrity,
                battery_energy,
                energy_per_phase,
                manual_activation,
                activation_energy,
                activation_bandwidth,
                activation_link_range,
            } => TechniqueAction::DeploySaturationBeacon {
                radius,
                damage,
                duration_time_units,
                integrity,
                battery_energy,
                energy_per_phase,
                manual_activation,
                activation_energy,
                activation_bandwidth,
                activation_link_range,
            },
            Self::ImplantImplosion {
                range,
                energy_cost,
                heat_generated,
                bandwidth_required,
                minimum_stored_energy,
                reserved_energy,
                delay_time_units,
                radius,
                physical_damage,
                thermal_damage,
                audit_delay,
            } => TechniqueAction::ImplantImplosion {
                range,
                energy_cost,
                heat_generated,
                bandwidth_required,
                minimum_stored_energy,
                reserved_energy,
                delay_time_units,
                radius,
                physical_damage,
                thermal_damage,
                audit_delay,
            },
        })
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum RawSignatureChannel {
    Optical,
    Acoustic,
    ActiveEmission,
}

impl RawSignatureChannel {
    const fn into_runtime(self) -> SignatureChannel {
        match self {
            Self::Optical => SignatureChannel::Optical,
            Self::Acoustic => SignatureChannel::Acoustic,
            Self::ActiveEmission => SignatureChannel::ActiveEmission,
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawExplosiveDeployment {
    ThrownImpact {
        range: u16,
        #[serde(default)]
        exact_placement_modifier: i16,
    },
    AdjacentTimed {
        delay_turns: u16,
        target: RawExplosivePlacementTarget,
    },
    AdjacentProximity {
        arming_delay_turns: u16,
        trigger_radius: u16,
    },
    AdjacentRemote {
        maximum_link_range: u16,
        target: RawExplosivePlacementTarget,
    },
}

impl RawExplosiveDeployment {
    fn into_runtime(self) -> Result<ExplosiveDeployment, SkillDefinitionError> {
        Ok(match self {
            Self::ThrownImpact {
                range,
                exact_placement_modifier,
            } => ExplosiveDeployment::ThrownImpact {
                range,
                exact_placement_modifier,
            },
            Self::AdjacentTimed {
                delay_turns,
                target,
            } => ExplosiveDeployment::AdjacentTimed {
                delay_turns,
                target: target.into_runtime(),
            },
            Self::AdjacentProximity {
                arming_delay_turns,
                trigger_radius,
            } => ExplosiveDeployment::AdjacentProximity {
                arming_delay_turns,
                trigger_radius,
            },
            Self::AdjacentRemote {
                maximum_link_range,
                target,
            } => ExplosiveDeployment::AdjacentRemote {
                maximum_link_range,
                target: target.into_runtime(),
            },
        })
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawExplosivePlacementTarget {
    KnownCell,
    FreeCell,
    DestructibleOccupant,
    StructuralSupport { maximum_cells: u8 },
}

impl RawExplosivePlacementTarget {
    const fn into_runtime(self) -> ExplosivePlacementTarget {
        match self {
            Self::KnownCell => ExplosivePlacementTarget::KnownCell,
            Self::FreeCell => ExplosivePlacementTarget::FreeCell,
            Self::DestructibleOccupant => ExplosivePlacementTarget::DestructibleOccupant,
            Self::StructuralSupport { maximum_cells } => {
                ExplosivePlacementTarget::StructuralSupport { maximum_cells }
            }
        }
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawSecondaryExplosivePayload {
    delay_after_first: u16,
    payload: RawExplosivePayload,
}

impl RawSecondaryExplosivePayload {
    fn into_runtime(self) -> Result<SecondaryExplosivePayload, SkillDefinitionError> {
        Ok(SecondaryExplosivePayload::new(
            self.delay_after_first,
            self.payload.into_runtime()?,
        ))
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawExplosivePayload {
    area: RawExplosiveArea,
    damage: RawWeaponDamage,
    center_damage: Option<RawWeaponDamage>,
    #[serde(default)]
    terrain_breach_cells: u8,
}

impl RawExplosivePayload {
    fn into_runtime(self) -> Result<ExplosivePayloadProfile, SkillDefinitionError> {
        let area = match self.area {
            RawExplosiveArea::Radial {
                radius,
                falloff_per_step,
            } => ExplosiveAreaProfile::Radial {
                radius,
                falloff_per_step,
            },
            RawExplosiveArea::Directional {
                range,
                narrow_length,
                maximum_half_width,
                widen_every,
            } => ExplosiveAreaProfile::Directional {
                range,
                cone: ConeAttack::new(narrow_length, maximum_half_width, widen_every).map_err(
                    |error| match error {
                        crate::combat::ConeAttackError::ZeroMaximumHalfWidth => {
                            SkillDefinitionError::ZeroMaximumTargets
                        }
                        crate::combat::ConeAttackError::ZeroWidenEvery => {
                            SkillDefinitionError::ZeroActionRange
                        }
                    },
                )?,
            },
        };
        let mut profile = ExplosivePayloadProfile::new(area, self.damage.into_runtime())
            .with_terrain_breach_cells(self.terrain_breach_cells);
        if let Some(damage) = self.center_damage {
            profile = profile.with_center_damage(damage.into_runtime());
        }
        Ok(profile)
    }
}

#[derive(Clone, Copy, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
enum RawExplosiveArea {
    Radial {
        radius: u16,
        #[serde(default)]
        falloff_per_step: u16,
    },
    Directional {
        range: u16,
        narrow_length: u16,
        maximum_half_width: u16,
        widen_every: u16,
    },
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawMeleeArc {
    maximum_cells: u8,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawForcedMovement {
    distance: u8,
    #[serde(default)]
    impact_modifier: i16,
}

impl RawForcedMovement {
    const fn into_runtime(self) -> ForcedMovement {
        ForcedMovement::new(self.distance, self.impact_modifier)
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
    CharacterClassDefinition {
        package: PackageId,
        path: PathBuf,
        content: Box<ContentId>,
        error: Box<CharacterClassDefinitionError>,
    },
    CharacterClassCatalog {
        package: PackageId,
        path: PathBuf,
        error: Box<CharacterClassCatalogError>,
    },
    RegionalWorldDefinition {
        package: PackageId,
        path: PathBuf,
        error: Box<RegionalWorldError>,
    },
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
    StatusCatalogValidation {
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
            Self::CharacterClassDefinition {
                package,
                path,
                content,
                error,
            } => write!(
                formatter,
                "[{package}]\n{}\n{content}\ninvalid character class definition: {error}",
                path.display()
            ),
            Self::CharacterClassCatalog {
                package,
                path,
                error,
            } => write!(formatter, "[{package}]\n{}\n{error}", path.display()),
            Self::RegionalWorldDefinition {
                package,
                path,
                error,
            } => write!(formatter, "[{package}]\n{}\n{error}", path.display()),
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
            Self::StatusCatalogValidation { error } => {
                write!(formatter, "invalid status catalog: {error}")
            }
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
    fn weapon_content_can_override_delivery_and_accuracy_without_a_code_branch() {
        let raw: RawWeaponAttack = json5::from_str(
            r#"{
                range: 1,
                distance_metric: "chebyshev",
                requires_line_of_sight: true,
                delivery: "ranged",
                accuracy_modifier: -7,
                damage: { amount: 2, damage_type: "piercing" },
            }"#,
        )
        .unwrap();

        let attack = raw.into_runtime().unwrap();

        assert_eq!(attack.delivery(), AttackDelivery::Ranged);
        assert_eq!(attack.accuracy_modifier(), -7);
    }

    #[test]
    fn item_and_weapon_content_can_declare_positive_unit_mass() {
        let raw_weapon: RawWeaponDefinition = json5::from_str(
            r#"{
                id: "core:weighted_blade",
                name_key: "weapon.weighted_blade.name",
                description_key: "weapon.weighted_blade.description",
                mass_grams: 3500,
                attack: {
                    range: 1,
                    distance_metric: "chebyshev",
                    requires_line_of_sight: false,
                    damage: { amount: 3, damage_type: "kinetic" },
                },
            }"#,
        )
        .unwrap();
        let weapon = raw_weapon
            .into_runtime(
                "core:weighted_blade".parse().unwrap(),
                &StatusCatalog::default(),
            )
            .unwrap();
        assert_eq!(weapon.mass_grams(), Some(3_500));

        let raw_item: RawItemDefinition = json5::from_str(
            r#"{
                id: "core:weighted_part",
                name_key: "item.weighted_part.name",
                description_key: "item.weighted_part.description",
                maximum_stack: 4,
                kind: "material",
                mass_grams: 1250,
            }"#,
        )
        .unwrap();
        let item = raw_item
            .into_runtime("core:weighted_part".parse().unwrap(), None)
            .unwrap();
        assert_eq!(item.mass_grams(), Some(1_250));

        let invalid: RawItemDefinition = json5::from_str(
            r#"{
                id: "core:massless_part",
                name_key: "item.massless_part.name",
                description_key: "item.massless_part.description",
                maximum_stack: 1,
                kind: "material",
                mass_grams: 0,
            }"#,
        )
        .unwrap();
        assert_eq!(
            invalid.into_runtime("core:massless_part".parse().unwrap(), None),
            Err(ItemDefinitionError::ZeroMass)
        );
    }

    #[test]
    fn weapon_content_supports_typed_mixed_damage_without_legacy_ambiguity() {
        let raw: RawWeaponAttack = json5::from_str(
            r#"{
                range: 5,
                distance_metric: "euclidean",
                requires_line_of_sight: true,
                damage: {
                    components: [
                        { amount: 6, damage_type: "kinetic" },
                        { amount: 4, damage_type: "piercing" },
                        { amount: 8, damage_type: "electrical" },
                    ],
                    armor_penetration: 2,
                    resistance_penetrations: [
                        { damage_type: "electrical", percentage_points: 10 },
                    ],
                },
            }"#,
        )
        .unwrap();

        let damage = raw.into_runtime().unwrap().damage();

        assert_eq!(damage.raw_total(), 18);
        assert_eq!(damage.raw_amount(DamageType::Kinetic), 6);
        assert_eq!(damage.raw_amount(DamageType::Piercing), 4);
        assert_eq!(damage.raw_amount(DamageType::Electrical), 8);
        assert_eq!(damage.armor_penetration(), 2);
        assert_eq!(damage.resistance_penetration(DamageType::Electrical), 10);
        assert!(!damage.is_legacy_single());
    }

    #[test]
    fn weapon_content_rejects_ungrouped_mixed_components() {
        let raw: RawWeaponAttack = json5::from_str(
            r#"{
                range: 5,
                distance_metric: "euclidean",
                requires_line_of_sight: true,
                damage: {
                    components: [
                        { amount: 3, damage_type: "kinetic" },
                        { amount: 2, damage_type: "kinetic" },
                    ],
                },
            }"#,
        )
        .unwrap();

        assert!(matches!(
            raw.into_runtime(),
            Err(WeaponDefinitionError::InvalidDamage(
                crate::combat::DamageImpactError::DuplicateComponent(DamageType::Kinetic)
            ))
        ));
    }

    #[test]
    fn weapon_content_declares_positive_recovery_without_code_ids() {
        let raw: RawWeaponAttack = json5::from_str(
            r#"{
                range: 1,
                distance_metric: "chebyshev",
                requires_line_of_sight: false,
                damage: { amount: 3, damage_type: "kinetic" },
                recovery_time_units: 1,
            }"#,
        )
        .unwrap();
        assert_eq!(
            raw.into_runtime().unwrap().recovery_after_attack(),
            Some(TimeUnits::ONE)
        );

        let invalid: RawWeaponAttack = json5::from_str(
            r#"{
                range: 1,
                distance_metric: "chebyshev",
                requires_line_of_sight: false,
                damage: { amount: 3, damage_type: "kinetic" },
                recovery_time_units: 0,
            }"#,
        )
        .unwrap();
        assert_eq!(
            invalid.into_runtime(),
            Err(WeaponDefinitionError::InvalidRecovery(
                crate::time::TimeUnitsError::Zero
            ))
        );
    }

    #[test]
    fn weapon_effect_content_supports_explicit_triggers_and_legacy_defaults() {
        let explicit: RawWeaponEffect = json5::from_str(
            r#"{
                type: "create_ground_effect",
                id: "core:test_ground",
                duration_turns: 2,
                damage_each_turn: { amount: 1, damage_type: "thermal" },
                trigger: "on_attack",
            }"#,
        )
        .unwrap();
        let legacy: RawWeaponEffect = json5::from_str(
            r#"{
                type: "create_ground_effect",
                id: "core:test_ground",
                duration_turns: 2,
                damage_each_turn: { amount: 1, damage_type: "thermal" },
            }"#,
        )
        .unwrap();

        let explicit = explicit.into_runtime(&StatusCatalog::default()).unwrap();
        let legacy = legacy.into_runtime(&StatusCatalog::default()).unwrap();

        assert_eq!(explicit.trigger(), Some(WeaponEffectTrigger::OnAttack));
        assert_eq!(legacy.trigger(), None);
    }

    #[test]
    fn body_content_can_declare_armor_mass_and_anchoring_without_a_code_branch() {
        let raw: RawBodyProfile = json5::from_str(
            r#"{
                base_hit_points: 12,
                material_bonus: 3,
                base_armor: 4,
                mass_grams: 80000,
                anchoring: 10,
            }"#,
        )
        .unwrap();

        let body = raw.into_runtime().unwrap();

        assert_eq!(body.base_hit_points, 12);
        assert_eq!(body.material_bonus, 3);
        assert_eq!(body.base_armor, 4);
        let displacement = body.displacement_profile().unwrap();
        assert_eq!(displacement.mass_grams(), 80_000);
        assert_eq!(displacement.anchoring(), 10);
        assert_eq!(displacement.resistance(0), 18);

        let invalid: RawBodyProfile =
            json5::from_str(r#"{ base_hit_points: 12, anchoring: 2 }"#).unwrap();
        assert_eq!(
            invalid.into_runtime(),
            Err(crate::stats::PhysicalRulesError::DisplacementPropertiesWithoutMass)
        );
    }

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
        let melee: crate::skills::DisciplineId = "core:combat_rapproche"
            .parse()
            .unwrap_or_else(|error| panic!("valid discipline ID rejected: {error}"));
        let drone_control: crate::skills::DisciplineId = "core:controle_drones"
            .parse()
            .unwrap_or_else(|error| panic!("valid discipline ID rejected: {error}"));
        let engineering: crate::skills::DisciplineId = "core:ingenierie"
            .parse()
            .unwrap_or_else(|error| panic!("valid discipline ID rejected: {error}"));
        let intrusion: crate::skills::DisciplineId = "core:intrusion"
            .parse()
            .unwrap_or_else(|error| panic!("valid discipline ID rejected: {error}"));
        let electronic_warfare: crate::skills::DisciplineId = "core:guerre_electronique"
            .parse()
            .unwrap_or_else(|error| panic!("valid discipline ID rejected: {error}"));
        let parry: crate::skills::TechniqueId = "core:mel_04"
            .parse()
            .unwrap_or_else(|error| panic!("valid technique ID rejected: {error}"));
        let crushing: crate::skills::TechniqueId = "core:mel_09"
            .parse()
            .unwrap_or_else(|error| panic!("valid technique ID rejected: {error}"));
        let interception: crate::skills::TechniqueId = "core:mel_10"
            .parse()
            .unwrap_or_else(|error| panic!("valid technique ID rejected: {error}"));
        let multiple_analysis: crate::skills::TechniqueId = "core:rec_09"
            .parse()
            .unwrap_or_else(|error| panic!("valid technique ID rejected: {error}"));
        let target_analysis: crate::skills::TechniqueId = "core:rec_01"
            .parse()
            .unwrap_or_else(|error| panic!("valid technique ID rejected: {error}"));
        let tactical_reading: crate::skills::TechniqueId = "core:rec_06"
            .parse()
            .unwrap_or_else(|error| panic!("valid technique ID rejected: {error}"));
        let starter_expedition: ExpeditionId = "core:starter_expedition"
            .parse()
            .unwrap_or_else(|error| panic!("valid expedition ID rejected: {error}"));
        let regional_world: ContentId = "core:simulation_overworld"
            .parse()
            .unwrap_or_else(|error| panic!("valid regional world ID rejected: {error}"));
        let breach: ContentId = "core:breche"
            .parse()
            .unwrap_or_else(|error| panic!("valid class ID rejected: {error}"));

        assert_eq!(
            loaded
                .package_order()
                .iter()
                .map(PackageId::as_str)
                .collect::<Vec<_>>(),
            vec!["core"]
        );
        assert!(loaded.statuses().contains(&corrosion));
        assert_eq!(loaded.character_classes().iter().count(), 3);
        assert_eq!(
            loaded
                .character_classes()
                .get(&breach)
                .map(|definition| definition.recommended_attributes()),
            Some(PrimaryAttributes::new(8, 5, 7, 4, 4))
        );
        assert_eq!(
            loaded
                .weapons()
                .get(&blade)
                .and_then(|weapon| weapon.attack().melee_impact())
                .map(|impact| impact.material_cap),
            Some(14)
        );
        assert!(
            loaded
                .weapons()
                .get(&blade)
                .is_some_and(|weapon| weapon.capabilities().can_melee_parry())
        );
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
            effect.kind(),
            crate::weapon::WeaponEffectKind::ApplyStatus(status)
                if status.status().as_str() == "core:burning"
                    && effect.trigger() == Some(WeaponEffectTrigger::OnHit)
        )));
        assert!(flame.effects().iter().any(|effect| matches!(
            effect.kind(),
            crate::weapon::WeaponEffectKind::CreateGroundEffect(ground)
                if ground.id().as_str() == "core:burning_ground"
                    && ground.duration_turns() == 3
                    && effect.trigger() == Some(WeaponEffectTrigger::OnAttack)
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
        assert!(loaded.skills().discipline(&melee).is_some());
        assert!(loaded.skills().discipline(&drone_control).is_some());
        assert!(loaded.skills().discipline(&engineering).is_some());
        assert!(loaded.skills().discipline(&intrusion).is_some());
        assert!(loaded.skills().discipline(&electronic_warfare).is_some());
        assert_eq!(
            loaded
                .skills()
                .techniques()
                .filter(|(_, definition)| definition.discipline() == &melee)
                .count(),
            10
        );
        assert_eq!(
            loaded
                .skills()
                .techniques()
                .filter(|(_, definition)| definition.discipline() == &drone_control)
                .count(),
            10
        );
        assert_eq!(
            loaded
                .skills()
                .techniques()
                .filter(|(_, definition)| definition.discipline() == &engineering)
                .count(),
            10
        );
        assert_eq!(
            loaded
                .skills()
                .techniques()
                .filter(|(_, definition)| definition.discipline() == &intrusion)
                .count(),
            10
        );
        assert_eq!(
            loaded
                .skills()
                .techniques()
                .filter(|(_, definition)| definition.discipline() == &electronic_warfare)
                .count(),
            18
        );
        assert_eq!(
            loaded
                .skills()
                .technique(&parry)
                .and_then(TechniqueDefinition::action),
            Some(TechniqueAction::PrepareMeleeParry {
                physical_reduction_percentage: 50,
                trigger_energy_cost: 2,
            })
        );
        assert!(matches!(
            loaded
                .skills()
                .technique(&crushing)
                .and_then(TechniqueDefinition::engagement_requirement),
            Some(TechniqueEngagementRequirement::TargetHasAnyStatusFamily(families))
                if families.iter().map(ContentId::as_str).collect::<Vec<_>>()
                    == ["core:immobilization", "core:locomotion_hindrance"]
        ));
        assert_eq!(
            loaded
                .skills()
                .technique(&interception)
                .and_then(TechniqueDefinition::action),
            Some(TechniqueAction::PrepareMeleeInterception)
        );
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
        let tactical_reading_definition = loaded
            .skills()
            .technique(&tactical_reading)
            .expect("tactical reading technique was not loaded");
        assert_eq!(tactical_reading_definition.minimum_level(), 2);
        assert_eq!(
            tactical_reading_definition.prerequisite(),
            Some(&target_analysis)
        );
        assert_eq!(
            tactical_reading_definition.improvement(),
            Some(TechniqueImprovement::NpcVisionOverlay)
        );
        assert!(tactical_reading_definition.material_cost().is_none());
        assert!(tactical_reading_definition.required_tool().is_none());
        assert!(tactical_reading_definition.action().is_none());
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
        assert!(prototype_availability.is_open());
        assert_eq!(prototype_availability.available.len(), 5);
        assert!(prototype_availability.complete_paths > 0);
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
        assert_eq!(trace_enabled_availability.available.len(), 6);
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
        assert_eq!(expedition.destination.population.len(), 3);
        assert_eq!(
            expedition.destination.population[0].primary_attributes(),
            Some(PrimaryAttributes::new(6, 6, 5, 6, 4))
        );
        assert_eq!(
            expedition.destination.population[0]
                .body_profile()
                .map(|body| body.base_hit_points),
            Some(9)
        );
        assert_eq!(
            expedition.destination.population[0]
                .body_profile()
                .and_then(|body| body.displacement_profile())
                .map(|profile| profile.mass_grams()),
            Some(75_000)
        );
        assert_eq!(
            expedition.destination.population[0]
                .body_profile()
                .and_then(|body| body.locomotion_profile())
                .map(|profile| profile.hindrance_compatible()),
            Some(true)
        );
        assert_eq!(
            expedition.destination.population[0]
                .attack()
                .melee_impact()
                .map(|impact| impact.material_cap),
            Some(12)
        );
        assert_eq!(
            expedition.destination.population[0]
                .attack()
                .preparation_disruption()
                .map(|disruption| (disruption.family(), disruption.intensity())),
            Some((crate::combat::PreparationDisruptionFamily::SystemShock, 55))
        );
        let expanded = expedition
            .expanded_world
            .as_ref()
            .expect("starter expedition should expose its expanded layout");
        assert_eq!(expanded.hub_passage, crate::world::GridPos::new(176, 108));
        assert_eq!(expanded.destination_generator.width, 128);
        assert_eq!(expanded.destination_generator.height, 88);
        assert_eq!(expanded.destination_generator.room_count, 24);
        assert_eq!(
            expedition.destination.population[0].ai().behavior,
            AiBehavior::Hunter
        );
        assert_eq!(
            expedition.destination.population[0]
                .ai()
                .maximum_pursuit_distance(),
            Some(14)
        );
        assert_eq!(
            expedition.destination.population[1].ai().behavior,
            AiBehavior::Sentry
        );
        assert_eq!(
            expedition.destination.population[1]
                .ai()
                .maximum_pursuit_distance(),
            None
        );
        assert_eq!(expedition.destination.population[0].count(), 1);
        assert_eq!(
            expedition.destination.population[0]
                .defeat_reward()
                .map(|reward| reward.base_experience),
            Some(8)
        );
        let merchant = expedition
            .hub_merchant
            .as_ref()
            .expect("core hub merchant was not loaded");
        assert_eq!(expedition.player_starting_credits, 120);
        assert_eq!(merchant.initial_credits, 300);
        assert_eq!(merchant.offers.len(), 3);
        assert_eq!(
            merchant
                .offers
                .iter()
                .filter(|offer| offer.available_at_depth(0))
                .count(),
            2
        );
        assert_eq!(
            merchant
                .offers
                .iter()
                .filter(|offer| offer.available_at_depth(3))
                .count(),
            2
        );
        assert_eq!(merchant.gambles.len(), 2);
        assert_eq!(merchant.gamble_scaling.player_levels_per_rank, 3);
        assert_eq!(merchant.gamble_scaling.zone_depths_per_rank, 1);
        assert_eq!(merchant.gamble_scaling.maximum_rank, 12);
        let clinic = expedition
            .hub_clinic
            .as_ref()
            .expect("core hub clinic was not loaded");
        assert_eq!(clinic.work_position, GridPos::new(15, 11));
        assert_eq!(clinic.break_position, GridPos::new(17, 13));
        assert_eq!(clinic.initial_credits, 80);
        assert_eq!(clinic.maximum_restoration, 8);
        assert_eq!(clinic.price_per_point, 3);
        assert_eq!((clinic.work_turns, clinic.break_turns), (12, 4));
        assert_eq!(expedition.hub_residents.len(), 1);
        let resident = &expedition.hub_residents[0];
        assert_eq!(resident.residence_position, GridPos::new(12, 27));
        assert_eq!(resident.gathering_position, GridPos::new(40, 23));
        assert_eq!((resident.residence_turns, resident.gathering_turns), (7, 5));
        let archive_follow_up = expedition
            .hub_quests
            .iter()
            .find(|quest| quest.quest.id().as_str() == "core:verify_archive_context")
            .expect("core archive follow-up quest was not loaded");
        assert!(matches!(
            archive_follow_up.completion_world_effects.as_slice(),
            [crate::content::QuestWorldEffectDefinition::UpdateDataTerminal {
                installation,
                record,
                summary_key,
            }] if installation.as_str() == "core:starter_city_archive_terminal"
                && record.as_str() == "core:starter_city_archive_verified_record"
                && summary_key == "world_effect.archive_terminal_updated.summary"
        ));
        let facility = expedition
            .hub_facility
            .as_ref()
            .expect("core hub facility was not loaded");
        assert_eq!(facility.blueprint.installations.len(), 6);
        assert!(facility.blueprint.installations.iter().any(|installation| {
            installation.id.as_str() == "core:orme_direction_board"
                && installation.position == GridPos::new(10, 28)
                && installation.capabilities.iter().any(|capability| {
                    matches!(
                        capability,
                        InstallationCapability::DataTerminal { record }
                            if record.as_str() == "core:orme_direction_board_old"
                    )
                })
        }));
        assert!(facility.blueprint.installations.iter().any(|installation| {
            installation.id.as_str() == "core:starter_city_archive_terminal"
                && installation.capabilities.iter().any(|capability| {
                    matches!(
                        capability,
                        InstallationCapability::DataTerminal { record }
                            if record.as_str() == "core:starter_city_archive_record"
                    )
                })
        }));
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
            facility.blueprint.workers[0]
                .property_report
                .map(|report| report.recipient_position),
            Some(GridPos::new(44, 32))
        );
        assert_eq!(
            facility.blueprint.workers[0]
                .property_report
                .map(|report| report.channel.field_of_view().radius),
            Some(32)
        );
        assert_eq!(
            facility.materials[0].owner.as_ref().map(ContentId::as_str),
            Some("core:maintenance_collective")
        );
        let regional_world = loaded
            .regional_worlds()
            .get(&regional_world)
            .expect("core regional world was not loaded");
        assert_eq!(
            regional_world.bounds().addressable_region_count(),
            8_388_608
        );
        assert_eq!(regional_world.province_size(), 4);
        assert_eq!(regional_world.biomes().len(), 8);
        assert_eq!(regional_world.vertical_links().len(), 5);
        assert_eq!(regional_world.cities().len(), 5);
        let city = regional_world
            .city_at(crate::content::RegionCoord::new(-1, 0, 1))
            .expect("the first deep layer must expose its authored city");
        assert_eq!(city.id().as_str(), "core:maintenance_exchange");
        assert_eq!(city.name(), "Nœud de maintenance");
        assert_eq!(city.kind().as_str(), "core:maintenance_city");
        assert_eq!(
            city.map_size(),
            crate::content::RegionMapSize::new(96, 64).unwrap()
        );
        assert_eq!(city.residents().len(), 2);
        assert_eq!(city.merchant().position, GridPos::new(36, 32));
        assert_eq!(city.clinic().work_position, GridPos::new(58, 32));
        let second_city = regional_world
            .city_at(crate::content::RegionCoord::new(-1, 0, 2))
            .expect("the second deep layer must expose a distinct authored city");
        assert_eq!(second_city.id().as_str(), "core:coolant_crown");
        assert_eq!(second_city.name(), "Couronne de refroidissement");
        assert_eq!(second_city.kind().as_str(), "core:coolant_city");
        assert_eq!(
            second_city.map_size(),
            crate::content::RegionMapSize::new(80, 72).unwrap()
        );
        assert_eq!(second_city.residents().len(), 3);
        let third_city = regional_world
            .city_at(crate::content::RegionCoord::new(-1, 0, 3))
            .expect("the third deep layer must expose a distinct authored city");
        assert_eq!(third_city.id().as_str(), "core:control_bastion");
        assert_eq!(third_city.name(), "Bastion dissonant");
        assert_eq!(third_city.kind().as_str(), "core:security_city");
        assert_eq!(
            third_city.map_size(),
            crate::content::RegionMapSize::new(112, 56).unwrap()
        );
        assert_eq!(third_city.residents().len(), 4);
        let fourth_city = regional_world
            .city_at(crate::content::RegionCoord::new(-1, 0, 4))
            .expect("the fourth deep layer must expose a distinct authored city");
        assert_eq!(fourth_city.id().as_str(), "core:shedding_rosette");
        assert_eq!(fourth_city.name(), "Rosace des mues");
        assert_eq!(fourth_city.kind().as_str(), "core:recursive_city");
        assert_eq!(
            fourth_city.map_size(),
            crate::content::RegionMapSize::new(88, 88).unwrap()
        );
        assert_eq!(fourth_city.residents().len(), 5);
        let fifth_city = regional_world
            .city_at(crate::content::RegionCoord::new(-1, 0, 5))
            .expect("the fifth deep layer must expose a distinct authored city");
        assert_eq!(fifth_city.id().as_str(), "core:error_core");
        assert_eq!(fifth_city.name(), "Noyau des erreurs");
        assert_eq!(fifth_city.kind().as_str(), "core:dead_system_city");
        assert_eq!(
            fifth_city.map_size(),
            crate::content::RegionMapSize::new(104, 80).unwrap()
        );
        assert_eq!(fifth_city.residents().len(), 6);
        assert_eq!(
            regional_world.vertical_neighbor(
                crate::content::RegionCoord::new(-1, 0, 1),
                crate::content::RegionVerticalDirection::Down,
            ),
            Some(crate::content::RegionCoord::new(-1, 0, 2))
        );
        let legacy_regions = loaded
            .regional_worlds()
            .without_second_layer_route_metadata();
        let legacy_world = legacy_regions.get(&regional_world.id().clone()).unwrap();
        assert_eq!(legacy_world.vertical_links().len(), 1);
        assert_eq!(
            legacy_world.vertical_neighbor(
                crate::content::RegionCoord::new(-1, 0, 0),
                crate::content::RegionVerticalDirection::Down,
            ),
            Some(crate::content::RegionCoord::new(-1, 0, 1))
        );
        assert!(
            legacy_world
                .vertical_neighbor(
                    crate::content::RegionCoord::new(-1, 0, 1),
                    crate::content::RegionVerticalDirection::Down,
                )
                .is_none()
        );
        let pre_city_regions = loaded.regional_worlds().without_city_metadata();
        assert!(
            pre_city_regions
                .get(&regional_world.id().clone())
                .unwrap()
                .cities()
                .is_empty()
        );
        assert!(
            regional_world
                .biomes()
                .iter()
                .filter(|biome| biome.maximum_depth() == Some(0))
                .all(|biome| {
                    !biome.population().is_empty()
                        && !biome.encounters().is_empty()
                        && biome.encounters().minimum_group_rolls() >= 3
                })
        );
        assert!(
            regional_world
                .biomes()
                .iter()
                .filter(|biome| biome.maximum_depth() == Some(0))
                .all(|biome| {
                    biome.loot().is_some()
                        && !biome.landmarks().is_empty()
                        && !biome.sites().is_empty()
                        && biome.site_terminals().is_some()
                        && biome.threats().is_some()
                        && biome.population().rules().iter().all(|rule| {
                            rule.ai().pursuit_lifecycle().is_some()
                                && rule.primary_attributes().is_some()
                        })
                        && biome
                            .threats()
                            .is_some_and(|threats| threats.primary_attributes().is_some())
                })
        );
        assert!(matches!(
            regional_world
                .region(42, crate::content::RegionCoord::new(0, 0, 0))
                .unwrap()
                .biome
                .as_str(),
            "core:human_habitat" | "core:surface_wilds"
        ));
        assert!(
            regional_world
                .biomes()
                .iter()
                .filter(|biome| biome.minimum_depth() == 1)
                .all(|biome| biome.destructibles().is_some_and(|profile| {
                    profile
                        .destruction_effect()
                        .ground_effect()
                        .is_some_and(|effect| {
                            effect.id().as_str() == "core:burning_ground"
                                && effect.duration_turns() == 3
                        })
                }))
        );
        let research = regional_world
            .biomes()
            .iter()
            .find(|biome| biome.biome().as_str() == "core:research")
            .expect("research biome must expose the conductive prototype");
        let relay = research
            .destructibles()
            .expect("research biome must contain conductive relays");
        let discharge = relay.destruction_effect().explosion();
        assert_eq!(discharge.damage.damage_type, DamageType::Electrical);
        assert_eq!(discharge.propagation_policy.floor_cost, Some(3));
        assert_eq!(discharge.propagation_policy.shallow_water_cost, Some(1));
        assert_eq!(discharge.propagation_policy.deep_water_cost, Some(1));
        assert!(relay.uses_distinct_water_propagation());
        assert!(
            relay
                .destruction_effect()
                .ground_effect()
                .is_some_and(|effect| {
                    effect.id().as_str() == "core:electrified_ground"
                        && effect.damage_each_turn().damage_type == DamageType::Electrical
                })
        );
    }

    #[test]
    fn technique_content_can_prepare_a_bounded_melee_parry() {
        let raw: RawTechniqueAction = json5::from_str(
            r#"{
                type: "prepare_melee_parry",
                physical_reduction_percentage: 50,
            }"#,
        )
        .unwrap();

        assert_eq!(
            raw.into_runtime(),
            Ok(TechniqueAction::PrepareMeleeParry {
                physical_reduction_percentage: 50,
                trigger_energy_cost: 0,
            })
        );
    }

    #[test]
    fn status_content_can_declare_non_refreshing_armor_fragilization() {
        let raw: RawStatusDefinition = json5::from_str(
            r#"{
                id: "test.mod:armor_fracture",
                duration_turns: 3,
                stacking: { mode: "keep_existing" },
                modifiers: [
                    { type: "armor_fragilization", amount: 4 },
                ],
            }"#,
        )
        .unwrap();
        let status = raw
            .into_runtime(
                "test.mod:armor_fracture".parse().unwrap(),
                None,
                Vec::new(),
                None,
            )
            .unwrap();

        assert_eq!(status.duration_turns(), Some(3));
        assert_eq!(status.stacking(), StatusStacking::KeepExisting);
        assert_eq!(
            status.modifiers(),
            [StatusModifier::ArmorFragilization { amount: 4 }]
        );
    }

    #[test]
    fn technique_content_can_apply_a_status_only_to_an_armored_hit_target() {
        let raw: RawTechniqueDefinition = json5::from_str(
            r#"{
                id: "test.mod:armor_break",
                discipline: "test.mod:melee",
                name_key: "technique.armor_break.name",
                description_key: "technique.armor_break.description",
                minimum_level: 1,
                kind: "action",
                prerequisite: null,
                action: {
                    type: "weapon_attack",
                    required_delivery: "melee",
                    physical_damage_percentage: 60,
                    energy_cost: 3,
                },
                on_hit_effect: {
                    type: "apply_status",
                    status: "test.mod:armor_fracture",
                    target_requirement: "has_armor",
                },
            }"#,
        )
        .unwrap();
        let effect = raw
            .on_hit_effect
            .unwrap()
            .into_runtime("test.mod:armor_fracture".parse().unwrap())
            .unwrap();

        assert_eq!(
            effect.target_requirement(),
            TechniqueTargetRequirement::HasArmor
        );
        assert_eq!(
            effect.application().status().as_str(),
            "test.mod:armor_fracture"
        );
        assert_eq!(effect.application().stacks(), 1);
    }

    #[test]
    fn status_content_can_damage_the_trigger_counterpart() {
        let raw: RawStatusEffect = json5::from_str(
            r#"{
                type: "deal_damage_to_counterpart",
                amount: 4,
                damage_type: "electrical",
                penetration: 2,
                multiply_by_stacks: true,
            }"#,
        )
        .unwrap();

        assert_eq!(
            raw.into_runtime(),
            StatusEffectPrimitive::DealDamageToCounterpart {
                packet: DamagePacket::new(4, DamageType::Electrical, 2),
                multiply_by_stacks: true,
            }
        );
    }

    #[test]
    fn content_can_declare_stability_resisted_locomotion_hindrance_and_cooldown() {
        let raw_status: RawStatusDefinition = json5::from_str(
            r#"{
                id: "test.mod:hindered",
                duration_turns: 2,
                stacking: { mode: "keep_existing" },
                family: "test.mod:locomotion_hindrance",
                expiration_transition: {
                    status: "test.mod:hindrance_protection",
                    stacks: 1,
                },
                modifiers: [
                    { type: "movement_time_minimum", time_units: 2 },
                ],
            }"#,
        )
        .unwrap();
        let transition = StatusTransition::new(
            raw_status
                .expiration_transition
                .as_ref()
                .unwrap()
                .status
                .parse()
                .unwrap(),
            1,
        )
        .unwrap();
        let status = raw_status
            .into_runtime(
                "test.mod:hindered".parse().unwrap(),
                Some("test.mod:locomotion_hindrance".parse().unwrap()),
                Vec::new(),
                Some(transition),
            )
            .unwrap();
        assert_eq!(
            status.modifiers(),
            [StatusModifier::MovementTimeMinimum { time_units: 2 }]
        );

        let raw: RawTechniqueDefinition = json5::from_str(
            r#"{
                id: "test.mod:hindrance",
                discipline: "test.mod:melee",
                name_key: "technique.hindrance.name",
                description_key: "technique.hindrance.description",
                minimum_level: 4,
                kind: "action",
                prerequisite: null,
                cooldown_turns: 2,
                action: {
                    type: "weapon_attack",
                    required_delivery: "melee",
                    physical_damage_percentage: 50,
                    energy_cost: 3,
                },
                on_hit_effect: {
                    type: "apply_status",
                    status: "test.mod:hindered",
                    target_requirement: "has_compatible_locomotion",
                    resistance: { type: "stability", intensity: 60 },
                },
            }"#,
        )
        .unwrap();
        assert_eq!(raw.cooldown_turns, Some(2));
        let effect = raw
            .on_hit_effect
            .unwrap()
            .into_runtime("test.mod:hindered".parse().unwrap())
            .unwrap();
        assert_eq!(
            effect.target_requirement(),
            TechniqueTargetRequirement::HasCompatibleLocomotion
        );
        assert_eq!(
            effect.resistance(),
            Some(crate::skills::TechniqueEffectResistance::Stability { intensity: 60 })
        );
    }

    #[test]
    fn technique_content_can_declare_a_passive_melee_counterattack() {
        let raw: RawTechniqueDefinition = json5::from_str(
            r#"{
                id: "test.mod:riposte",
                discipline: "test.mod:melee",
                name_key: "technique.riposte.name",
                description_key: "technique.riposte.description",
                minimum_level: 2,
                kind: "improvement",
                prerequisite: "test.mod:parry",
                improvement: { type: "melee_counterattack" },
            }"#,
        )
        .unwrap();

        assert_eq!(
            raw.improvement.map(RawTechniqueImprovement::into_runtime),
            Some(TechniqueImprovement::MeleeCounterattack)
        );
        assert!(raw.action.is_none());
    }

    #[test]
    fn technique_content_can_modify_a_selected_weapon_attack() {
        let raw: RawTechniqueAction = json5::from_str(
            r#"{
                type: "weapon_attack",
                required_delivery: "melee",
                physical_damage_percentage: 150,
                accuracy_modifier: 20,
                recovery_time_units: 1,
            }"#,
        )
        .unwrap();

        assert_eq!(
            raw.into_runtime(),
            Ok(TechniqueAction::WeaponAttack {
                required_delivery: AttackDelivery::Melee,
                physical_damage_percentage: Some(150),
                armor_penetration_bonus: 0,
                accuracy_modifier: 20,
                energy_cost: 0,
                recovery_time_units: Some(1),
                forced_movement: None,
                melee_arc: None,
            })
        );

        let accuracy_only: RawTechniqueAction = json5::from_str(
            r#"{
                type: "weapon_attack",
                required_delivery: "melee",
                accuracy_modifier: 20,
            }"#,
        )
        .unwrap();
        assert_eq!(
            accuracy_only.into_runtime(),
            Ok(TechniqueAction::WeaponAttack {
                required_delivery: AttackDelivery::Melee,
                physical_damage_percentage: None,
                armor_penetration_bonus: 0,
                accuracy_modifier: 20,
                energy_cost: 0,
                recovery_time_units: None,
                forced_movement: None,
                melee_arc: None,
            })
        );

        let push: RawTechniqueAction = json5::from_str(
            r#"{
                type: "weapon_attack",
                required_delivery: "melee",
                physical_damage_percentage: 50,
                forced_movement: { distance: 1, impact_modifier: 0 },
            }"#,
        )
        .unwrap();
        assert_eq!(
            push.into_runtime(),
            Ok(TechniqueAction::WeaponAttack {
                required_delivery: AttackDelivery::Melee,
                physical_damage_percentage: Some(50),
                armor_penetration_bonus: 0,
                accuracy_modifier: 0,
                energy_cost: 0,
                recovery_time_units: None,
                forced_movement: Some(ForcedMovement::new(1, 0)),
                melee_arc: None,
            })
        );

        let sweep: RawTechniqueAction = json5::from_str(
            r#"{
                type: "weapon_attack",
                required_delivery: "melee",
                physical_damage_percentage: 70,
                energy_cost: 4,
                recovery_time_units: 1,
                melee_arc: { maximum_cells: 3 },
            }"#,
        )
        .unwrap();
        assert_eq!(
            sweep.into_runtime(),
            Ok(TechniqueAction::WeaponAttack {
                required_delivery: AttackDelivery::Melee,
                physical_damage_percentage: Some(70),
                armor_penetration_bonus: 0,
                accuracy_modifier: 0,
                energy_cost: 4,
                recovery_time_units: Some(1),
                forced_movement: None,
                melee_arc: Some(MeleeArc::new(3).unwrap()),
            })
        );
    }

    #[test]
    fn technique_content_can_declare_multi_ut_preparation() {
        let raw: RawTechniqueDefinition = json5::from_str(
            r#"{
                id: "test.mod:slow_scan",
                discipline: "test.mod:scanning",
                name_key: "technique.slow_scan.name",
                description_key: "technique.slow_scan.description",
                minimum_level: 1,
                kind: "action",
                prerequisite: null,
                action: { type: "read_movement_traces", radius: 3 },
                action_kind: "offensive",
                preparation_time_units: 2,
            }"#,
        )
        .unwrap();

        assert_eq!(raw.preparation_time_units, Some(2));
        assert!(matches!(raw.action_kind, Some(RawActionKind::Offensive)));
        assert!(matches!(
            raw.action,
            Some(RawTechniqueAction::ReadMovementTraces { radius: 3 })
        ));
    }

    #[test]
    fn technique_content_can_declare_level_and_attribute_requirements() {
        let raw: RawTechniqueDefinition = json5::from_str(
            r#"{
                id: "test.mod:advanced_scan",
                discipline: "test.mod:scanning",
                name_key: "technique.advanced_scan.name",
                description_key: "technique.advanced_scan.description",
                minimum_level: 4,
                minimum_attributes: { perception: 7, processing: 6 },
                kind: "action",
                prerequisite: null,
                action: { type: "read_movement_traces", radius: 5 },
            }"#,
        )
        .unwrap();

        assert_eq!(raw.minimum_level, 4);
        assert_eq!(
            raw.minimum_attributes.into_runtime().unwrap(),
            vec![
                TechniqueAttributeRequirement::new(PrimaryAttribute::Perception, 7).unwrap(),
                TechniqueAttributeRequirement::new(PrimaryAttribute::Processing, 6).unwrap(),
            ]
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
                .package_manifests()
                .iter()
                .map(|manifest| (manifest.id.as_str(), manifest.version.to_string()))
                .collect::<Vec<_>>(),
            vec![
                ("core", "0.1.0".to_owned()),
                ("example.arc_arsenal", "0.1.0".to_owned()),
            ]
        );
        assert_eq!(
            loaded
                .weapons()
                .get(&arc_lance)
                .map(|weapon| weapon.attack().damage().primary_damage_type()),
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
        assert_eq!(expedition.destination.population.len(), 1);
        assert_eq!(expedition.destination.population[0].count(), 2);
        assert_eq!(
            expedition.destination.population[0].ai().behavior,
            AiBehavior::Skirmisher
        );
        let arc_world = loaded
            .regional_worlds()
            .get(&"example.arc_arsenal:arc_frontier".parse().unwrap())
            .expect("mod regional world was not loaded");
        assert_eq!(arc_world.bounds().addressable_region_count(), 3_267);
        assert_eq!(arc_world.province_size(), 3);
        assert_eq!(
            arc_world
                .biome(&"example.arc_arsenal:charged_wastes".parse().unwrap())
                .unwrap()
                .population()
                .maximum_group_rolls(),
            2
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
    fn regional_world_loader_rejects_invalid_or_incomplete_atlases() {
        let root = std::env::temp_dir().join(format!(
            "project-rl-regional-world-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let package_root = root.join("test.regions");
        fs::create_dir_all(package_root.join("regional_worlds")).unwrap();
        fs::write(
            package_root.join("manifest.toml"),
            r#"id = "test.regions"
name = "Regional world test"
version = "0.1.0"
author = "test"
game_version = ">=0.1.0, <0.2.0"
dependencies = ["core >=0.1.0, <0.2.0"]
optional_dependencies = []
incompatible = []
"#,
        )
        .unwrap();
        let path = package_root.join("regional_worlds/test.json5");
        let roots = [
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content"),
            root.clone(),
        ];
        let valid = r#"{
            id:'test.regions:world',
            bounds:{minimum_x:-4,maximum_x:4,minimum_y:-4,maximum_y:4,maximum_depth:2},
            province_size:2,
            local_map:{width:48,height:32},
            vertical_links:[{upper:[-1,0,0],lower:[-1,0,1]}],
            biomes:[
                {
                    id:'test.regions:surface',weight:2,maximum_depth:0,
                    terrain:{ground:'grass',patch_count:2,minimum_patch_radius:1,
                        maximum_patch_radius:3,features:[{kind:'tree',weight:1}]}
                },
                {
                    id:'test.regions:depths',weight:1,minimum_depth:1,maximum_depth:2,
                    terrain:{ground:'gravel',patch_count:2,minimum_patch_radius:1,
                        maximum_patch_radius:3,features:[{kind:'boulder',weight:1}]}
                }
            ]
        }"#;
        for (source, expected) in [
            (
                valid.replace("province_size:2", "province_size:0"),
                "positive",
            ),
            (
                valid.replace(
                    "{\n                    id:'test.regions:depths',weight:1,minimum_depth:1,maximum_depth:2,\n                    terrain:{ground:'gravel',patch_count:2,minimum_patch_radius:1,\n                        maximum_patch_radius:3,features:[{kind:'boulder',weight:1}]}\n                }",
                    "",
                ),
                "depth 1 has no eligible biome",
            ),
            (
                valid.replace("test.regions:world", "core:foreign"),
                "namespace",
            ),
            (
                valid.replace("province_size:2", "province_siz:2"),
                "unknown field",
            ),
            (
                valid.replace("lower:[-1,0,1]", "lower:[0,0,1]"),
                "must keep x/y",
            ),
        ] {
            fs::write(&path, source).unwrap();
            let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
                .unwrap_err()
                .to_string();
            assert!(error.contains("test.regions") && error.contains("test.json5"));
            assert!(error.contains(expected), "Expected {expected}: {error}");
        }

        fs::write(&path, valid).unwrap();
        let loaded = ContentLoader::load(&roots, &Version::new(0, 1, 0)).unwrap();
        let world = loaded
            .regional_worlds()
            .get(&"test.regions:world".parse().unwrap())
            .unwrap();
        assert_eq!(
            world.vertical_neighbor(
                RegionCoord::new(-1, 0, 0),
                crate::content::RegionVerticalDirection::Down,
            ),
            Some(RegionCoord::new(-1, 0, 1))
        );
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
            (
                valid.replace(
                    "player_property_take_authorizations:",
                    "population:[{count:0,minimum_entrance_distance:3,maximum_integrity:5,attack:{range:1,distance_metric:'chebyshev',requires_line_of_sight:false,damage:{amount:2,damage_type:'kinetic'}},ai:{behavior:'hunter',perception_radius:6}}],player_property_take_authorizations:",
                ),
                "population group count must be positive",
            ),
            (
                valid.replace(
                    "player_property_take_authorizations:",
                    "population:[{count:1,minimum_entrance_distance:3,maximum_integrity:5,primary_attributes:{power:11,coordination:5,resilience:5,perception:5,processing:5},attack:{range:1,distance_metric:'chebyshev',requires_line_of_sight:false,damage:{amount:2,damage_type:'kinetic'}},ai:{behavior:'hunter',perception_radius:6}}],player_property_take_authorizations:",
                ),
                "invalid population primary attributes",
            ),
            (
                valid.replace(
                    "player_property_take_authorizations:",
                    "population:[{count:1,minimum_entrance_distance:3,maximum_integrity:5,attack:{range:1,distance_metric:'chebyshev',requires_line_of_sight:false,damage:{amount:0,damage_type:'kinetic'}},ai:{behavior:'hunter',perception_radius:6}}],player_property_take_authorizations:",
                ),
                "population attacks require positive range and damage",
            ),
            (
                valid.replace(
                    "player_property_take_authorizations:",
                    "population:[{count:257,minimum_entrance_distance:3,maximum_integrity:5,attack:{range:1,distance_metric:'chebyshev',requires_line_of_sight:false,damage:{amount:2,damage_type:'kinetic'}},ai:{behavior:'hunter',perception_radius:6}}],player_property_take_authorizations:",
                ),
                "generated zone population exceeds",
            ),
            (
                valid.replace(
                    "player_property_take_authorizations:",
                    "population:[{count:1,minimum_entrance_distance:3,maximum_integrity:5,attack:{range:257,distance_metric:'chebyshev',requires_line_of_sight:false,damage:{amount:2,damage_type:'kinetic'}},ai:{behavior:'hunter',perception_radius:6}}],player_property_take_authorizations:",
                ),
                "attack range or cone width exceeds",
            ),
            (
                valid.replace(
                    "player_property_take_authorizations:",
                    "population:[{count:1,minimum_entrance_distance:3,maximum_integrity:5,attack:{range:1,distance_metric:'chebyshev',requires_line_of_sight:false,damage:{amount:2,damage_type:'kinetic'}},ai:{behavior:'hunter',perception_radius:257}}],player_property_take_authorizations:",
                ),
                "perception radius exceeds",
            ),
            (
                valid.replace(
                    "player_property_take_authorizations:",
                    "population:[{count:1,minimum_entrance_distance:3,maximum_integrity:5,attack:{range:1,distance_metric:'chebyshev',requires_line_of_sight:false,damage:{amount:2,damage_type:'kinetic'}},ai:{behavior:'hunter',perception_radius:6,maximum_pursuit_distance:0}}],player_property_take_authorizations:",
                ),
                "pursuit distance must be positive",
            ),
            (
                valid.replace(
                    "player_property_take_authorizations:",
                    "population:[{count:1,minimum_entrance_distance:3,maximum_integrity:5,attack:{range:1,distance_metric:'chebyshev',requires_line_of_sight:false,damage:{amount:2,damage_type:'kinetic'}},ai:{behavior:'hunter',perception_radius:6,maximum_pursuit_distance:257}}],player_property_take_authorizations:",
                ),
                "pursuit distance exceeds",
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
        assert!(expedition.hub_quests.is_empty());

        let quest_content = valid.replace(
            "player_property_take_authorizations:",
            "hub_quests:[{id:'test.world:local_delivery',title_key:'quest.local_delivery.title',summary_key:'quest.local_delivery.summary',objective:{type:'delivery',required_item:'core:power_regulator',required_quantity:2},reward_credits:45,provider:{type:'contact',position:[3,3],maximum_integrity:10}}],player_property_take_authorizations:",
        );
        fs::write(&path, &quest_content).unwrap();
        let loaded = ContentLoader::load(&roots, &Version::new(0, 1, 0)).unwrap();
        let quest = &loaded
            .expeditions()
            .get(&"test.world:expedition".parse().unwrap())
            .unwrap()
            .hub_quests[0];
        let crate::content::QuestDefinition::Delivery(delivery) = &quest.quest else {
            panic!("delivery objective expected");
        };
        assert_eq!(delivery.id.as_str(), "test.world:local_delivery");
        assert_eq!(delivery.required_item.as_str(), "core:power_regulator");
        assert_eq!(delivery.required_quantity, 2);
        assert_eq!(delivery.reward_credits, 45);
        assert!(matches!(
            quest.provider,
            crate::content::HubQuestProviderDefinition::Contact {
                position: GridPos { x: 3, y: 3 },
                maximum_integrity: 10
            }
        ));

        let chain_content = valid.replace(
            "player_property_take_authorizations:",
            "hub_quests:[{id:'test.world:chain_start',title_key:'quest.chain_start.title',summary_key:'quest.chain_start.summary',objective:{type:'explore_zones',required_zones:1},reward_credits:10,reward_experience:7,reward_items:[{item:'core:power_regulator',quantity:2}],completion_world_states:[{id:'test.world:reported',summary_key:'world.reported.summary',provider_dialogue_key:'world.reported.dialogue'}],completion_world_effects:[{type:'unlock_door',position:[4,4],summary_key:'world_effect.access.summary'},{type:'grant_property_take_authorization',owner:'test.world:salvage_collective',summary_key:'world_effect.salvage_authorized.summary'}],provider:{type:'contact',position:[3,3],maximum_integrity:10}},{id:'test.world:chain_follow_up',title_key:'quest.chain_follow_up.title',summary_key:'quest.chain_follow_up.summary',objective:{type:'explore_zones',required_zones:2},prerequisites:['test.world:chain_start'],required_world_states:['test.world:reported'],reward_credits:20,provider:{type:'contact',position:[3,3],maximum_integrity:10}}],player_property_take_authorizations:",
        );
        fs::write(&path, &chain_content).unwrap();
        let loaded = ContentLoader::load(&roots, &Version::new(0, 1, 0)).unwrap();
        let quests = &loaded
            .expeditions()
            .get(&"test.world:expedition".parse().unwrap())
            .unwrap()
            .hub_quests;
        assert_eq!(quests.len(), 2);
        assert_eq!(quests[0].reward_experience, 7);
        assert_eq!(quests[0].reward_items[0].quantity, 2);
        assert_eq!(
            quests[0].completion_world_states[0].id.as_str(),
            "test.world:reported"
        );
        assert_eq!(
            quests[0].completion_world_effects[0].door_position(),
            Some(GridPos::new(4, 4))
        );
        assert_eq!(
            quests[0].completion_world_effects[0].summary_key(),
            "world_effect.access.summary"
        );
        assert!(matches!(
            &quests[0].completion_world_effects[1],
            crate::content::QuestWorldEffectDefinition::GrantPropertyTakeAuthorization {
                owner,
                summary_key,
            } if owner.as_str() == "test.world:salvage_collective"
                && summary_key == "world_effect.salvage_authorized.summary"
        ));
        assert_eq!(
            quests[1].required_world_states[0].as_str(),
            "test.world:reported"
        );
        assert_eq!(
            quests[1].prerequisites[0].as_str(),
            "test.world:chain_start"
        );
        fs::write(
            &path,
            chain_content.replace("position:[4,4]", "position:[-1,4]"),
        )
        .unwrap();
        let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
            .unwrap_err()
            .to_string();
        assert!(error.contains("quest world effects"), "{error}");
        fs::write(
            &path,
            chain_content.replace("['test.world:chain_start']", "['test.world:unknown']"),
        )
        .unwrap();
        let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
            .unwrap_err()
            .to_string();
        assert!(error.contains("prerequisites"), "{error}");
        fs::write(
            &path,
            chain_content.replace(
                "required_world_states:['test.world:reported']",
                "required_world_states:['test.world:unknown_state']",
            ),
        )
        .unwrap();
        let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
            .unwrap_err()
            .to_string();
        assert!(error.contains("world states"), "{error}");

        for (source, expected) in [
            (
                quest_content.replace("core:power_regulator", "core:unknown_quest_item"),
                "unknown quest item",
            ),
            (
                quest_content.replace("test.world:local_delivery", "core:foreign_quest"),
                "namespace",
            ),
            (
                quest_content.replace("required_quantity:2", "required_quantity:0"),
                "positive item quantity",
            ),
            (
                quest_content.replace(
                    "provider:{type:'contact',position:[3,3],maximum_integrity:10}",
                    "provider:{type:'existing',position:[3,3]}",
                ),
                "no authored hub NPC",
            ),
            (
                quest_content.replace(
                    "}],player_property_take_authorizations:",
                    "},{id:'test.world:other_delivery',title_key:'quest.other.title',summary_key:'quest.other.summary',objective:{type:'delivery',required_item:'core:power_regulator',required_quantity:1},reward_credits:1,provider:{type:'existing',position:[3,3]}}],player_property_take_authorizations:",
                ),
                "more than one quest targets",
            ),
        ] {
            fs::write(&path, source).unwrap();
            let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
                .unwrap_err()
                .to_string();
            assert!(error.contains(expected), "Expected {expected}: {error}");
        }

        let exploration_content = valid.replace(
            "player_property_take_authorizations:",
            "hub_quests:[{id:'test.world:local_exploration',title_key:'quest.local_exploration.title',summary_key:'quest.local_exploration.summary',objective:{type:'explore_zones',required_zones:3},reward_credits:70,provider:{type:'contact',position:[3,3],maximum_integrity:10}}],player_property_take_authorizations:",
        );
        fs::write(&path, &exploration_content).unwrap();
        let loaded = ContentLoader::load(&roots, &Version::new(0, 1, 0)).unwrap();
        let quest = &loaded
            .expeditions()
            .get(&"test.world:expedition".parse().unwrap())
            .unwrap()
            .hub_quests[0];
        let crate::content::QuestDefinition::ExploreZones(exploration) = &quest.quest else {
            panic!("exploration objective expected");
        };
        assert_eq!(exploration.id.as_str(), "test.world:local_exploration");
        assert_eq!(exploration.required_zones, 3);
        assert_eq!(exploration.reward_credits, 70);
        fs::write(
            &path,
            exploration_content.replace("required_zones:3", "required_zones:0"),
        )
        .unwrap();
        let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
            .unwrap_err()
            .to_string();
        assert!(error.contains("positive zone count"), "{error}");

        let data_record_content = valid.replace(
            "player_property_take_authorizations:",
            "hub_quests:[{id:'test.world:local_archive',title_key:'quest.local_archive.title',summary_key:'quest.local_archive.summary',objective:{type:'access_data_record',record:'test.world:sealed_archive'},reward_credits:55,provider:{type:'contact',position:[3,3],maximum_integrity:10}}],player_property_take_authorizations:",
        );
        fs::write(&path, &data_record_content).unwrap();
        let loaded = ContentLoader::load(&roots, &Version::new(0, 1, 0)).unwrap();
        let quest = &loaded
            .expeditions()
            .get(&"test.world:expedition".parse().unwrap())
            .unwrap()
            .hub_quests[0];
        let crate::content::QuestDefinition::AccessDataRecord(data_record) = &quest.quest else {
            panic!("data record objective expected");
        };
        assert_eq!(data_record.id.as_str(), "test.world:local_archive");
        assert_eq!(data_record.record.as_str(), "test.world:sealed_archive");
        assert_eq!(data_record.reward_credits, 55);

        let tagged_population_content = valid.replace(
            "player_property_take_authorizations:",
            "population:[{count:1,minimum_entrance_distance:3,maximum_integrity:5,tags:['test.world:rust_hound'],attack:{range:1,distance_metric:'chebyshev',requires_line_of_sight:false,damage:{amount:2,damage_type:'kinetic'}},ai:{behavior:'hunter',perception_radius:6}}],player_property_take_authorizations:",
        );
        fs::write(&path, &tagged_population_content).unwrap();
        let loaded = ContentLoader::load(&roots, &Version::new(0, 1, 0)).unwrap();
        assert_eq!(
            loaded
                .expeditions()
                .get(&"test.world:expedition".parse().unwrap())
                .unwrap()
                .destination
                .population[0]
                .tags()[0]
                .as_str(),
            "test.world:rust_hound"
        );
        fs::write(
            &path,
            tagged_population_content.replace(
                "['test.world:rust_hound']",
                "['test.world:rust_hound','test.world:rust_hound']",
            ),
        )
        .unwrap();
        let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
            .unwrap_err()
            .to_string();
        assert!(error.contains("actor tags must be unique"), "{error}");

        let defeat_content = valid.replace(
            "player_property_take_authorizations:",
            "hub_quests:[{id:'test.world:local_hunt',title_key:'quest.local_hunt.title',summary_key:'quest.local_hunt.summary',objective:{type:'defeat_targets',target_tag:'test.world:rust_hound',required_quantity:2},reward_credits:80,provider:{type:'contact',position:[3,3],maximum_integrity:10}}],player_property_take_authorizations:",
        );
        fs::write(&path, &defeat_content).unwrap();
        let loaded = ContentLoader::load(&roots, &Version::new(0, 1, 0)).unwrap();
        let quest = &loaded
            .expeditions()
            .get(&"test.world:expedition".parse().unwrap())
            .unwrap()
            .hub_quests[0];
        let crate::content::QuestDefinition::DefeatTargets(defeat) = &quest.quest else {
            panic!("defeat targets objective expected");
        };
        assert_eq!(defeat.target_tag.as_str(), "test.world:rust_hound");
        assert_eq!(defeat.required_quantity, 2);
        assert_eq!(defeat.reward_credits, 80);
        fs::write(
            &path,
            defeat_content.replace("required_quantity:2", "required_quantity:0"),
        )
        .unwrap();
        let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
            .unwrap_err()
            .to_string();
        assert!(error.contains("positive target quantity"), "{error}");

        fs::write(&path, valid).unwrap();
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
        let invalid_property_report = facility
            .replace("core:unknown_material", "core:power_regulator")
            .replace(
                "workers:[]",
                "workers:[{position:[4,2],role:'retriever',maximum_integrity:5,property_report:{recipient_position:[5,2],radius:0}}]",
            );
        fs::write(&path, invalid_property_report).unwrap();
        let error = ContentLoader::load(&roots, &Version::new(0, 1, 0))
            .unwrap_err()
            .to_string();
        assert!(error.contains("property report range"), "{error}");
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
