use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::content::ContentId;

mod progression;

pub use progression::{
    SkillProgressionState, SkillProgressionStateError, TechniqueLearned, TechniqueLearningError,
};

pub type DisciplineId = ContentId;
pub type TechniqueId = ContentId;
pub type SystemFeatureId = ContentId;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SystemFeatureSet {
    enabled: BTreeSet<SystemFeatureId>,
}

impl SystemFeatureSet {
    pub fn new(features: impl IntoIterator<Item = SystemFeatureId>) -> Self {
        Self {
            enabled: features.into_iter().collect(),
        }
    }

    pub fn contains(&self, feature: &SystemFeatureId) -> bool {
        self.enabled.contains(feature)
    }

    pub fn iter(&self) -> impl Iterator<Item = &SystemFeatureId> {
        self.enabled.iter()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TechniqueKind {
    Action,
    Posture,
    Procedure,
    Behavior,
    Improvement,
}

/// Engine-supported behavior selected by data for an active technique.
///
/// The catalogue binds an ID to one of these reusable primitives; game logic
/// therefore never branches on a particular `core:*` technique identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TechniqueAction {
    AnalyzeTarget {
        range: u16,
    },
    /// Uses the analysis primitive and range of its learned prerequisite.
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisciplineDefinition {
    id: DisciplineId,
    name_key: String,
    description_key: String,
}

impl DisciplineDefinition {
    pub fn new(
        id: DisciplineId,
        name_key: String,
        description_key: String,
    ) -> Result<Self, SkillDefinitionError> {
        validate_text_keys(&name_key, &description_key)?;
        Ok(Self {
            id,
            name_key,
            description_key,
        })
    }

    pub const fn id(&self) -> &DisciplineId {
        &self.id
    }

    pub fn name_key(&self) -> &str {
        &self.name_key
    }

    pub fn description_key(&self) -> &str {
        &self.description_key
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TechniqueDefinition {
    id: TechniqueId,
    discipline: DisciplineId,
    name_key: String,
    description_key: String,
    minimum_rank: u8,
    kind: TechniqueKind,
    prerequisite: Option<TechniqueId>,
    required_features: BTreeSet<SystemFeatureId>,
    action: Option<TechniqueAction>,
}

impl TechniqueDefinition {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: TechniqueId,
        discipline: DisciplineId,
        name_key: String,
        description_key: String,
        minimum_rank: u8,
        kind: TechniqueKind,
        prerequisite: Option<TechniqueId>,
        required_features: impl IntoIterator<Item = SystemFeatureId>,
    ) -> Result<Self, SkillDefinitionError> {
        validate_text_keys(&name_key, &description_key)?;
        if minimum_rank == 0 {
            return Err(SkillDefinitionError::ZeroMinimumRank);
        }
        if prerequisite.as_ref() == Some(&id) {
            return Err(SkillDefinitionError::SelfPrerequisite);
        }
        Ok(Self {
            id,
            discipline,
            name_key,
            description_key,
            minimum_rank,
            kind,
            prerequisite,
            required_features: required_features.into_iter().collect(),
            action: None,
        })
    }

    pub fn with_action(mut self, action: TechniqueAction) -> Result<Self, SkillDefinitionError> {
        let multiple = matches!(action, TechniqueAction::AnalyzeMultipleTargets { .. });
        if multiple && (self.kind != TechniqueKind::Improvement || self.prerequisite.is_none()) {
            return Err(SkillDefinitionError::InvalidAnalysisImprovement);
        }
        if !multiple && self.kind != TechniqueKind::Action {
            return Err(SkillDefinitionError::ActionOnNonActionTechnique);
        }
        match action {
            TechniqueAction::AnalyzeMultipleTargets {
                maximum_targets: 0, ..
            } => {
                return Err(SkillDefinitionError::ZeroMaximumTargets);
            }
            TechniqueAction::AnalyzeTarget { range: 0 } => {
                return Err(SkillDefinitionError::ZeroActionRange);
            }
            TechniqueAction::ReadMovementTraces { radius: 0 } => {
                return Err(SkillDefinitionError::ZeroActionRange);
            }
            TechniqueAction::AnalyzeNearbyWalls { radius: 0, .. } => {
                return Err(SkillDefinitionError::ZeroActionRange);
            }
            TechniqueAction::AnalyzeNearbyWalls {
                maximum_tiles: 0, ..
            } => {
                return Err(SkillDefinitionError::ZeroMaximumTargets);
            }
            TechniqueAction::AnalyzeThreat { range: 0 } => {
                return Err(SkillDefinitionError::ZeroActionRange);
            }
            _ => {}
        }
        self.action = Some(action);
        Ok(self)
    }

    pub const fn id(&self) -> &TechniqueId {
        &self.id
    }

    pub const fn discipline(&self) -> &DisciplineId {
        &self.discipline
    }

    pub fn name_key(&self) -> &str {
        &self.name_key
    }

    pub fn description_key(&self) -> &str {
        &self.description_key
    }

    pub const fn minimum_rank(&self) -> u8 {
        self.minimum_rank
    }

    pub const fn kind(&self) -> TechniqueKind {
        self.kind
    }

    pub const fn prerequisite(&self) -> Option<&TechniqueId> {
        self.prerequisite.as_ref()
    }

    pub fn required_features(&self) -> impl Iterator<Item = &SystemFeatureId> {
        self.required_features.iter()
    }

    pub const fn action(&self) -> Option<TechniqueAction> {
        self.action
    }
}

fn validate_text_keys(name: &str, description: &str) -> Result<(), SkillDefinitionError> {
    if name.trim().is_empty() {
        return Err(SkillDefinitionError::EmptyNameKey);
    }
    if description.trim().is_empty() {
        return Err(SkillDefinitionError::EmptyDescriptionKey);
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkillDefinitionError {
    EmptyNameKey,
    EmptyDescriptionKey,
    ZeroMinimumRank,
    SelfPrerequisite,
    ActionOnNonActionTechnique,
    ZeroActionRange,
    ZeroMaximumTargets,
    InvalidAnalysisImprovement,
}

impl Display for SkillDefinitionError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyNameKey => write!(formatter, "skill name_key must not be empty"),
            Self::EmptyDescriptionKey => {
                write!(formatter, "skill description_key must not be empty")
            }
            Self::ZeroMinimumRank => write!(formatter, "technique minimum rank must be positive"),
            Self::SelfPrerequisite => write!(formatter, "a technique cannot require itself"),
            Self::ActionOnNonActionTechnique => {
                write!(
                    formatter,
                    "only an action technique can define an active action"
                )
            }
            Self::ZeroActionRange => write!(formatter, "technique action range must be positive"),
            Self::ZeroMaximumTargets => {
                write!(formatter, "technique action target limit must be positive")
            }
            Self::InvalidAnalysisImprovement => {
                write!(
                    formatter,
                    "multiple analysis must be an improvement of a target analysis"
                )
            }
        }
    }
}

impl Error for SkillDefinitionError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SkillProgressionRules {
    rank_costs: Vec<u16>,
}

impl SkillProgressionRules {
    pub fn new(rank_costs: Vec<u16>) -> Result<Self, SkillProgressionRulesError> {
        let rules = Self { rank_costs };
        rules.validate()?;
        Ok(rules)
    }

    pub fn validate(&self) -> Result<(), SkillProgressionRulesError> {
        if self.rank_costs.is_empty() {
            return Err(SkillProgressionRulesError::NoRanks);
        }
        if self.rank_costs.len() > usize::from(u8::MAX) {
            return Err(SkillProgressionRulesError::TooManyRanks);
        }
        if self.rank_costs.contains(&0) {
            return Err(SkillProgressionRulesError::ZeroRankCost);
        }
        Ok(())
    }

    pub fn maximum_rank(&self) -> u8 {
        u8::try_from(self.rank_costs.len()).unwrap_or(u8::MAX)
    }

    pub fn cost_for_rank(&self, rank: u8) -> Option<u16> {
        self.rank_costs
            .get(usize::from(rank.checked_sub(1)?))
            .copied()
    }

    pub fn rank_costs(&self) -> &[u16] {
        &self.rank_costs
    }
}

impl Default for SkillProgressionRules {
    fn default() -> Self {
        Self {
            rank_costs: vec![1, 1, 2, 2, 3],
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkillProgressionRulesError {
    NoRanks,
    TooManyRanks,
    ZeroRankCost,
}

impl Display for SkillProgressionRulesError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoRanks => write!(formatter, "skill progression must define at least one rank"),
            Self::TooManyRanks => write!(formatter, "skill progression defines too many ranks"),
            Self::ZeroRankCost => write!(formatter, "skill rank costs must be positive"),
        }
    }
}

impl Error for SkillProgressionRulesError {}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SkillCatalog {
    disciplines: BTreeMap<DisciplineId, DisciplineDefinition>,
    techniques: BTreeMap<TechniqueId, TechniqueDefinition>,
}

impl SkillCatalog {
    pub fn register_discipline(
        &mut self,
        definition: DisciplineDefinition,
    ) -> Result<(), SkillCatalogError> {
        let id = definition.id().clone();
        if self.disciplines.contains_key(&id) {
            return Err(SkillCatalogError::DuplicateDiscipline(id));
        }
        self.disciplines.insert(id, definition);
        Ok(())
    }

    pub fn register_technique(
        &mut self,
        definition: TechniqueDefinition,
    ) -> Result<(), SkillCatalogError> {
        let id = definition.id().clone();
        if self.techniques.contains_key(&id) {
            return Err(SkillCatalogError::DuplicateTechnique(id));
        }
        self.techniques.insert(id, definition);
        Ok(())
    }

    pub fn discipline(&self, id: &DisciplineId) -> Option<&DisciplineDefinition> {
        self.disciplines.get(id)
    }

    pub fn technique(&self, id: &TechniqueId) -> Option<&TechniqueDefinition> {
        self.techniques.get(id)
    }

    /// Targeted improvements inherit their prerequisite's sensor range.
    pub fn target_range(&self, id: &TechniqueId) -> Option<u16> {
        let definition = self.technique(id)?;
        match definition.action()? {
            TechniqueAction::AnalyzeTarget { range } | TechniqueAction::AnalyzeThreat { range } => {
                Some(range)
            }
            TechniqueAction::AnalyzeMultipleTargets { .. } => {
                match self.technique(definition.prerequisite()?)?.action()? {
                    TechniqueAction::AnalyzeTarget { range } => Some(range),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn disciplines(&self) -> impl Iterator<Item = (&DisciplineId, &DisciplineDefinition)> {
        self.disciplines.iter()
    }

    pub fn techniques(&self) -> impl Iterator<Item = (&TechniqueId, &TechniqueDefinition)> {
        self.techniques.iter()
    }

    pub fn validate(&self, rules: &SkillProgressionRules) -> Result<(), SkillCatalogError> {
        rules.validate().map_err(SkillCatalogError::Rules)?;
        self.validate_structure()?;
        for technique in self.techniques.values() {
            if technique.minimum_rank() > rules.maximum_rank() {
                return Err(SkillCatalogError::RankTooHigh {
                    technique: technique.id().clone(),
                    rank: technique.minimum_rank(),
                    maximum: rules.maximum_rank(),
                });
            }
        }

        let all_features = SystemFeatureSet::new(
            self.techniques
                .values()
                .flat_map(TechniqueDefinition::required_features)
                .cloned(),
        );
        for discipline in self.disciplines.keys() {
            let availability = self
                .analyze_discipline(discipline, &all_features, &[], rules)
                .map_err(SkillCatalogError::InvalidInitialChoices)?;
            if !availability.is_open() {
                return Err(SkillCatalogError::IncompleteDiscipline(discipline.clone()));
            }
        }
        Ok(())
    }

    pub fn validate_structure(&self) -> Result<(), SkillCatalogError> {
        for technique in self.techniques.values() {
            if !self.disciplines.contains_key(technique.discipline()) {
                return Err(SkillCatalogError::UnknownDiscipline {
                    technique: Box::new(technique.id().clone()),
                    discipline: Box::new(technique.discipline().clone()),
                });
            }
            if let Some(prerequisite_id) = technique.prerequisite() {
                let prerequisite = self.techniques.get(prerequisite_id).ok_or_else(|| {
                    SkillCatalogError::UnknownPrerequisite {
                        technique: Box::new(technique.id().clone()),
                        prerequisite: Box::new(prerequisite_id.clone()),
                    }
                })?;
                if prerequisite.discipline() != technique.discipline() {
                    return Err(SkillCatalogError::CrossDisciplinePrerequisite {
                        technique: Box::new(technique.id().clone()),
                        prerequisite: Box::new(prerequisite_id.clone()),
                    });
                }
                if matches!(
                    technique.action(),
                    Some(TechniqueAction::AnalyzeMultipleTargets { .. })
                ) && !matches!(
                    prerequisite.action(),
                    Some(TechniqueAction::AnalyzeTarget { .. })
                ) {
                    return Err(SkillCatalogError::InvalidAnalysisImprovement(
                        technique.id().clone(),
                    ));
                }
                if prerequisite.minimum_rank() >= technique.minimum_rank() {
                    return Err(SkillCatalogError::PrerequisiteRankNotEarlier {
                        technique: Box::new(technique.id().clone()),
                        prerequisite: Box::new(prerequisite_id.clone()),
                    });
                }
            }
        }
        Ok(())
    }

    /// Structural validation can load future content. Running an open
    /// discipline additionally requires executable behavior for every purchase.
    pub fn validate_runtime(
        &self,
        features: &SystemFeatureSet,
        rules: &SkillProgressionRules,
    ) -> Result<(), SkillCatalogError> {
        self.validate(rules)?;
        for discipline in self.disciplines.keys() {
            let availability = self
                .analyze_discipline(discipline, features, &[], rules)
                .map_err(SkillCatalogError::InvalidInitialChoices)?;
            if availability.is_open() {
                for id in availability.available {
                    if self
                        .technique(&id)
                        .and_then(TechniqueDefinition::action)
                        .is_none()
                    {
                        return Err(SkillCatalogError::MissingTechniqueBehavior(id));
                    }
                }
            }
        }
        Ok(())
    }

    pub fn analyze_discipline(
        &self,
        discipline: &DisciplineId,
        features: &SystemFeatureSet,
        initial_choices: &[TechniqueId],
        rules: &SkillProgressionRules,
    ) -> Result<DisciplineAvailability, InitialChoicesError> {
        let maximum_rank = rules.maximum_rank();
        let pool: Vec<&TechniqueDefinition> = self
            .techniques
            .values()
            .filter(|technique| technique.discipline() == discipline)
            .collect();
        let mut available: BTreeSet<TechniqueId> = pool
            .iter()
            .filter(|technique| {
                technique
                    .required_features()
                    .all(|feature| features.contains(feature))
            })
            .map(|technique| technique.id().clone())
            .collect();

        loop {
            let invalid: Vec<TechniqueId> = available
                .iter()
                .filter(|id| {
                    self.techniques
                        .get(*id)
                        .and_then(TechniqueDefinition::prerequisite)
                        .is_some_and(|prerequisite| !available.contains(prerequisite))
                })
                .cloned()
                .collect();
            if invalid.is_empty() {
                break;
            }
            for id in invalid {
                available.remove(&id);
            }
        }

        if initial_choices.len() > usize::from(maximum_rank) {
            return Err(InitialChoicesError::TooManyChoices);
        }
        let mut initial = BTreeSet::new();
        for (index, id) in initial_choices.iter().enumerate() {
            let technique = self
                .techniques
                .get(id)
                .ok_or_else(|| InitialChoicesError::UnknownTechnique(id.clone()))?;
            if technique.discipline() != discipline {
                return Err(InitialChoicesError::WrongDiscipline(id.clone()));
            }
            if !available.contains(id) {
                return Err(InitialChoicesError::UnavailableTechnique(id.clone()));
            }
            if !initial.insert(id.clone()) {
                return Err(InitialChoicesError::DuplicateTechnique(id.clone()));
            }
            let rank = u8::try_from(index + 1).unwrap_or(u8::MAX);
            if technique.minimum_rank() > rank {
                return Err(InitialChoicesError::RankTooLow(id.clone()));
            }
            if technique
                .prerequisite()
                .is_some_and(|prerequisite| !initial.contains(prerequisite))
            {
                return Err(InitialChoicesError::MissingPrerequisite(id.clone()));
            }
        }

        let mut states = vec![BTreeSet::new(); usize::from(maximum_rank) + 1];
        states[initial.len()].insert(initial.clone());
        let mut dead_ends = Vec::new();
        for choice_count in initial.len()..usize::from(maximum_rank) {
            let current_states: Vec<BTreeSet<TechniqueId>> =
                states[choice_count].iter().cloned().collect();
            for learned in current_states {
                let next_rank = u8::try_from(choice_count + 1).unwrap_or(u8::MAX);
                let successors: Vec<TechniqueId> = available
                    .iter()
                    .filter(|id| !learned.contains(*id))
                    .filter(|id| {
                        self.techniques
                            .get(*id)
                            .is_some_and(|technique| technique.minimum_rank() <= next_rank)
                    })
                    .filter(|id| {
                        self.techniques
                            .get(*id)
                            .and_then(TechniqueDefinition::prerequisite)
                            .is_none_or(|prerequisite| learned.contains(prerequisite))
                    })
                    .cloned()
                    .collect();
                if successors.is_empty() {
                    dead_ends.push(learned);
                } else {
                    for successor in successors {
                        let mut next = learned.clone();
                        next.insert(successor);
                        states[choice_count + 1].insert(next);
                    }
                }
            }
        }

        let unavailable = pool
            .iter()
            .map(|technique| technique.id().clone())
            .filter(|id| !available.contains(id))
            .collect();
        let complete_paths = states[usize::from(maximum_rank)].len();
        Ok(DisciplineAvailability {
            discipline: discipline.clone(),
            available,
            unavailable,
            dead_ends,
            complete_paths,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DisciplineAvailability {
    pub discipline: DisciplineId,
    pub available: BTreeSet<TechniqueId>,
    pub unavailable: BTreeSet<TechniqueId>,
    pub dead_ends: Vec<BTreeSet<TechniqueId>>,
    pub complete_paths: usize,
}

impl DisciplineAvailability {
    pub fn is_open(&self) -> bool {
        self.dead_ends.is_empty() && self.complete_paths > 0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InitialChoicesError {
    TooManyChoices,
    UnknownTechnique(TechniqueId),
    WrongDiscipline(TechniqueId),
    UnavailableTechnique(TechniqueId),
    DuplicateTechnique(TechniqueId),
    RankTooLow(TechniqueId),
    MissingPrerequisite(TechniqueId),
}

impl Display for InitialChoicesError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooManyChoices => write!(formatter, "too many initial skill choices"),
            Self::UnknownTechnique(id) => write!(formatter, "unknown initial technique '{id}'"),
            Self::WrongDiscipline(id) => {
                write!(
                    formatter,
                    "initial technique '{id}' belongs to another discipline"
                )
            }
            Self::UnavailableTechnique(id) => {
                write!(
                    formatter,
                    "initial technique '{id}' is unavailable in this version"
                )
            }
            Self::DuplicateTechnique(id) => {
                write!(formatter, "initial technique '{id}' is duplicated")
            }
            Self::RankTooLow(id) => write!(
                formatter,
                "initial technique '{id}' is granted before its minimum rank"
            ),
            Self::MissingPrerequisite(id) => {
                write!(
                    formatter,
                    "initial technique '{id}' is missing its prerequisite"
                )
            }
        }
    }
}

impl Error for InitialChoicesError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SkillCatalogError {
    MissingTechniqueBehavior(TechniqueId),
    InvalidAnalysisImprovement(TechniqueId),
    Rules(SkillProgressionRulesError),
    DuplicateDiscipline(DisciplineId),
    DuplicateTechnique(TechniqueId),
    UnknownDiscipline {
        technique: Box<TechniqueId>,
        discipline: Box<DisciplineId>,
    },
    UnknownPrerequisite {
        technique: Box<TechniqueId>,
        prerequisite: Box<TechniqueId>,
    },
    CrossDisciplinePrerequisite {
        technique: Box<TechniqueId>,
        prerequisite: Box<TechniqueId>,
    },
    PrerequisiteRankNotEarlier {
        technique: Box<TechniqueId>,
        prerequisite: Box<TechniqueId>,
    },
    RankTooHigh {
        technique: TechniqueId,
        rank: u8,
        maximum: u8,
    },
    InvalidInitialChoices(InitialChoicesError),
    IncompleteDiscipline(DisciplineId),
}

impl Display for SkillCatalogError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTechniqueBehavior(id) => write!(
                formatter,
                "purchasable technique '{id}' has no executable behavior"
            ),
            Self::InvalidAnalysisImprovement(id) => write!(
                formatter,
                "multiple analysis '{id}' must require a target analysis"
            ),
            Self::Rules(error) => write!(formatter, "invalid skill progression rules: {error}"),
            Self::DuplicateDiscipline(id) => write!(formatter, "duplicate discipline ID '{id}'"),
            Self::DuplicateTechnique(id) => write!(formatter, "duplicate technique ID '{id}'"),
            Self::UnknownDiscipline {
                technique,
                discipline,
            } => write!(
                formatter,
                "technique '{technique}' references unknown discipline '{discipline}'"
            ),
            Self::UnknownPrerequisite {
                technique,
                prerequisite,
            } => write!(
                formatter,
                "technique '{technique}' references unknown prerequisite '{prerequisite}'"
            ),
            Self::CrossDisciplinePrerequisite {
                technique,
                prerequisite,
            } => write!(
                formatter,
                "technique '{technique}' cannot require cross-discipline technique '{prerequisite}'"
            ),
            Self::PrerequisiteRankNotEarlier {
                technique,
                prerequisite,
            } => write!(
                formatter,
                "prerequisite '{prerequisite}' must have an earlier rank than '{technique}'"
            ),
            Self::RankTooHigh {
                technique,
                rank,
                maximum,
            } => write!(
                formatter,
                "technique '{technique}' requires rank {rank}, above maximum {maximum}"
            ),
            Self::InvalidInitialChoices(error) => {
                write!(formatter, "invalid initial skill choices: {error}")
            }
            Self::IncompleteDiscipline(id) => write!(
                formatter,
                "discipline '{id}' cannot complete every legal progression path"
            ),
        }
    }
}

impl Error for SkillCatalogError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(value: &str) -> ContentId {
        value
            .parse()
            .unwrap_or_else(|error| panic!("valid content ID rejected: {error}"))
    }

    fn reconnaissance_catalog() -> SkillCatalog {
        let discipline = id("core:reconnaissance");
        let mut catalog = SkillCatalog::default();
        catalog
            .register_discipline(
                DisciplineDefinition::new(
                    discipline.clone(),
                    "discipline.reconnaissance.name".to_owned(),
                    "discipline.reconnaissance.description".to_owned(),
                )
                .unwrap_or_else(|error| panic!("valid discipline rejected: {error}")),
            )
            .unwrap_or_else(|error| panic!("valid discipline registration rejected: {error}"));
        let definitions = [
            ("rec_01", 1, None, None),
            ("rec_02", 1, None, Some("traces")),
            ("rec_03", 2, None, Some("secrets")),
            ("rec_04", 2, None, None),
            ("rec_05", 3, None, None),
            ("rec_09", 3, Some("rec_01"), None),
            ("rec_08", 4, None, Some("energy_states")),
        ];
        for (name, rank, prerequisite, feature) in definitions {
            catalog
                .register_technique(
                    TechniqueDefinition::new(
                        id(&format!("core:{name}")),
                        discipline.clone(),
                        format!("technique.{name}.name"),
                        format!("technique.{name}.description"),
                        rank,
                        if prerequisite.is_some() {
                            TechniqueKind::Improvement
                        } else {
                            TechniqueKind::Action
                        },
                        prerequisite.map(|parent| id(&format!("core:{parent}"))),
                        feature.map(|required| id(&format!("core:{required}"))),
                    )
                    .unwrap_or_else(|error| panic!("valid technique rejected: {error}")),
                )
                .unwrap_or_else(|error| panic!("valid registration rejected: {error}"));
        }
        catalog
    }

    #[test]
    fn complete_documented_reconnaissance_catalog_is_valid() {
        let catalog = reconnaissance_catalog();
        assert_eq!(catalog.validate(&SkillProgressionRules::default()), Ok(()));
    }

    #[test]
    fn multiple_analysis_requires_a_compatible_parent_and_inherits_its_range() {
        let mut catalog = reconnaissance_catalog();
        let parent = id("core:rec_01");
        let child = id("core:rec_09");
        let improvement = TechniqueAction::AnalyzeMultipleTargets {
            maximum_targets: 3,
            energy_cost: 2,
        };
        assert_eq!(
            catalog
                .technique(&parent)
                .unwrap()
                .clone()
                .with_action(improvement),
            Err(SkillDefinitionError::InvalidAnalysisImprovement)
        );
        assert_eq!(
            catalog.technique(&child).unwrap().clone().with_action(
                TechniqueAction::AnalyzeMultipleTargets {
                    maximum_targets: 0,
                    energy_cost: 2
                }
            ),
            Err(SkillDefinitionError::ZeroMaximumTargets)
        );
        catalog.techniques.get_mut(&child).unwrap().action = Some(improvement);
        assert_eq!(catalog.target_range(&child), None);
        assert_eq!(
            catalog.validate_structure(),
            Err(SkillCatalogError::InvalidAnalysisImprovement(child.clone()))
        );
        catalog.techniques.get_mut(&parent).unwrap().action =
            Some(TechniqueAction::AnalyzeTarget { range: 2 });
        assert_eq!(catalog.validate_structure(), Ok(()));
        assert_eq!(catalog.target_range(&child), Some(2));
    }

    #[test]
    fn unknown_prerequisite_is_rejected_explicitly() {
        let mut catalog = SkillCatalog::default();
        catalog
            .register_discipline(
                DisciplineDefinition::new(
                    id("core:test"),
                    "discipline.test.name".to_owned(),
                    "discipline.test.description".to_owned(),
                )
                .unwrap_or_else(|error| panic!("valid discipline rejected: {error}")),
            )
            .unwrap_or_else(|error| panic!("valid registration rejected: {error}"));
        catalog
            .register_technique(
                TechniqueDefinition::new(
                    id("core:child"),
                    id("core:test"),
                    "technique.child.name".to_owned(),
                    "technique.child.description".to_owned(),
                    2,
                    TechniqueKind::Improvement,
                    Some(id("core:missing")),
                    [],
                )
                .unwrap_or_else(|error| panic!("valid technique shape rejected: {error}")),
            )
            .unwrap_or_else(|error| panic!("valid registration rejected: {error}"));

        assert!(matches!(
            catalog.validate_structure(),
            Err(SkillCatalogError::UnknownPrerequisite { .. })
        ));
    }

    #[test]
    fn cyclic_prerequisites_are_impossible_across_strictly_earlier_ranks() {
        let mut catalog = SkillCatalog::default();
        catalog
            .register_discipline(
                DisciplineDefinition::new(
                    id("core:test"),
                    "discipline.test.name".to_owned(),
                    "discipline.test.description".to_owned(),
                )
                .unwrap_or_else(|error| panic!("valid discipline rejected: {error}")),
            )
            .unwrap_or_else(|error| panic!("valid registration rejected: {error}"));
        for (technique, prerequisite, rank) in [("first", "second", 1), ("second", "first", 2)] {
            catalog
                .register_technique(
                    TechniqueDefinition::new(
                        id(&format!("core:{technique}")),
                        id("core:test"),
                        format!("technique.{technique}.name"),
                        format!("technique.{technique}.description"),
                        rank,
                        TechniqueKind::Improvement,
                        Some(id(&format!("core:{prerequisite}"))),
                        [],
                    )
                    .unwrap_or_else(|error| panic!("valid technique shape rejected: {error}")),
                )
                .unwrap_or_else(|error| panic!("valid registration rejected: {error}"));
        }

        assert!(matches!(
            catalog.validate_structure(),
            Err(SkillCatalogError::PrerequisiteRankNotEarlier { .. })
        ));
    }

    #[test]
    fn optional_diagnostic_can_be_absent_without_closing_reconnaissance() {
        let catalog = reconnaissance_catalog();
        let features = SystemFeatureSet::new([id("core:traces"), id("core:secrets")]);
        let availability = catalog
            .analyze_discipline(
                &id("core:reconnaissance"),
                &features,
                &[],
                &SkillProgressionRules::default(),
            )
            .unwrap_or_else(|error| panic!("valid version rejected: {error}"));

        assert!(availability.is_open());
        assert_eq!(availability.available.len(), 6);
        assert!(availability.unavailable.contains(&id("core:rec_08")));
    }

    #[test]
    fn incomplete_version_defers_the_whole_discipline() {
        let catalog = reconnaissance_catalog();
        let availability = catalog
            .analyze_discipline(
                &id("core:reconnaissance"),
                &SystemFeatureSet::default(),
                &[],
                &SkillProgressionRules::default(),
            )
            .unwrap_or_else(|error| panic!("version analysis failed: {error}"));

        assert!(!availability.is_open());
        assert_eq!(availability.available.len(), 4);
        assert_eq!(availability.complete_paths, 0);
    }

    #[test]
    fn disabling_a_parent_transitively_disables_its_improvement() {
        let mut catalog = reconnaissance_catalog();
        let parent = id("core:rec_01");
        catalog
            .techniques
            .get_mut(&parent)
            .unwrap_or_else(|| panic!("parent fixture missing"))
            .required_features
            .insert(id("core:target_analysis"));
        let availability = catalog
            .analyze_discipline(
                &id("core:reconnaissance"),
                &SystemFeatureSet::new([
                    id("core:traces"),
                    id("core:secrets"),
                    id("core:energy_states"),
                ]),
                &[],
                &SkillProgressionRules::default(),
            )
            .unwrap_or_else(|error| panic!("version analysis failed: {error}"));

        assert!(availability.unavailable.contains(&parent));
        assert!(availability.unavailable.contains(&id("core:rec_09")));
    }

    #[test]
    fn class_grants_must_follow_rank_and_prerequisite_order() {
        let catalog = reconnaissance_catalog();
        let features = SystemFeatureSet::new([
            id("core:traces"),
            id("core:secrets"),
            id("core:energy_states"),
        ]);

        assert_eq!(
            catalog.analyze_discipline(
                &id("core:reconnaissance"),
                &features,
                &[id("core:rec_09")],
                &SkillProgressionRules::default(),
            ),
            Err(InitialChoicesError::RankTooLow(id("core:rec_09")))
        );
        assert!(
            catalog
                .analyze_discipline(
                    &id("core:reconnaissance"),
                    &features,
                    &[id("core:rec_01"), id("core:rec_02"), id("core:rec_09"),],
                    &SkillProgressionRules::default(),
                )
                .is_ok()
        );
    }
}
