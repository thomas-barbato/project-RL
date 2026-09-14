use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use super::{
    DisciplineId, InitialChoicesError, SkillCatalog, SkillCatalogError, SkillProgressionRules,
    SystemFeatureSet, TechniqueId,
};
use crate::stats::{PrimaryAttribute, PrimaryAttributes};

/// Ordered skill choices learned during one run.
///
/// The order is retained so costs can follow the configured purchase curve and
/// prerequisites can be validated without storing a second derived counter.
/// Learning additional techniques is never capped.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SkillProgressionState {
    choices: BTreeMap<DisciplineId, Vec<TechniqueId>>,
}

impl SkillProgressionState {
    pub fn from_ordered_choices(
        entries: impl IntoIterator<Item = (DisciplineId, Vec<TechniqueId>)>,
        catalog: &SkillCatalog,
        features: &SystemFeatureSet,
        rules: &SkillProgressionRules,
    ) -> Result<Self, SkillProgressionStateError> {
        let mut choices = BTreeMap::new();
        for (discipline, techniques) in entries {
            if techniques.is_empty() {
                return Err(SkillProgressionStateError::EmptyDiscipline(Box::new(
                    discipline,
                )));
            }
            if choices.insert(discipline.clone(), techniques).is_some() {
                return Err(SkillProgressionStateError::DuplicateDiscipline(Box::new(
                    discipline,
                )));
            }
        }
        let state = Self { choices };
        state.validate(catalog, features, rules)?;
        Ok(state)
    }

    pub fn validate(
        &self,
        catalog: &SkillCatalog,
        features: &SystemFeatureSet,
        rules: &SkillProgressionRules,
    ) -> Result<(), SkillProgressionStateError> {
        catalog
            .validate(rules)
            .map_err(|error| SkillProgressionStateError::Catalog(Box::new(error)))?;

        for (discipline, techniques) in &self.choices {
            if techniques.is_empty() {
                return Err(SkillProgressionStateError::EmptyDiscipline(Box::new(
                    discipline.clone(),
                )));
            }
            if catalog.discipline(discipline).is_none() {
                return Err(SkillProgressionStateError::UnknownDiscipline(Box::new(
                    discipline.clone(),
                )));
            }

            let complete_version = catalog
                .analyze_discipline(discipline, features, &[], rules)
                .map_err(|error| SkillProgressionStateError::InvalidChoices {
                    discipline: Box::new(discipline.clone()),
                    error: Box::new(error),
                })?;
            if !complete_version.is_open() {
                return Err(SkillProgressionStateError::DeferredDiscipline(Box::new(
                    discipline.clone(),
                )));
            }

            let continuation = catalog
                .analyze_discipline(discipline, features, techniques, rules)
                .map_err(|error| SkillProgressionStateError::InvalidChoices {
                    discipline: Box::new(discipline.clone()),
                    error: Box::new(error),
                })?;
            if !continuation.is_open() {
                return Err(SkillProgressionStateError::BlockedProgression(Box::new(
                    discipline.clone(),
                )));
            }
        }
        Ok(())
    }

    pub fn learned_count(&self, discipline: &DisciplineId) -> usize {
        self.choices(discipline).len()
    }

    pub fn has_learned(&self, technique: &TechniqueId) -> bool {
        self.choices
            .values()
            .any(|choices| choices.contains(technique))
    }

    pub fn choices(&self, discipline: &DisciplineId) -> &[TechniqueId] {
        self.choices
            .get(discipline)
            .map(Vec::as_slice)
            .unwrap_or_default()
    }

    pub fn disciplines(&self) -> impl ExactSizeIterator<Item = (&DisciplineId, &[TechniqueId])> {
        self.choices
            .iter()
            .map(|(discipline, choices)| (discipline, choices.as_slice()))
    }

    pub fn learn(
        &mut self,
        technique: &TechniqueId,
        catalog: &SkillCatalog,
        features: &SystemFeatureSet,
        rules: &SkillProgressionRules,
        player_level: u16,
        player_attributes: Option<PrimaryAttributes>,
    ) -> Result<TechniqueLearned, TechniqueLearningError> {
        self.validate(catalog, features, rules)
            .map_err(|error| TechniqueLearningError::InvalidCurrentState(Box::new(error)))?;
        let definition = catalog
            .technique(technique)
            .ok_or_else(|| TechniqueLearningError::UnknownTechnique(Box::new(technique.clone())))?;
        if self.has_learned(technique) {
            return Err(TechniqueLearningError::AlreadyLearned(Box::new(
                technique.clone(),
            )));
        }

        let discipline = definition.discipline().clone();
        let version = catalog
            .analyze_discipline(&discipline, features, &[], rules)
            .map_err(|error| TechniqueLearningError::InvalidChoices(Box::new(error)))?;
        if !version.is_open() {
            return Err(TechniqueLearningError::DeferredDiscipline(Box::new(
                discipline,
            )));
        }
        if !version.available.contains(technique) {
            return Err(TechniqueLearningError::UnavailableTechnique(Box::new(
                technique.clone(),
            )));
        }

        let current_choices = self.choices(&discipline);
        let choice_number = current_choices.len().saturating_add(1);
        let cost = rules
            .cost_for_choice_number(choice_number)
            .expect("validated skill progression always has a final choice cost");
        if rules.enforces_authored_requirements() {
            if definition.minimum_level() > player_level {
                return Err(TechniqueLearningError::LevelTooLow {
                    technique: Box::new(technique.clone()),
                    required: definition.minimum_level(),
                    current: player_level,
                });
            }
            if let Some(requirement) = definition.unmet_attribute_requirement(player_attributes) {
                return Err(TechniqueLearningError::AttributeTooLow {
                    technique: Box::new(technique.clone()),
                    attribute: requirement.attribute(),
                    required: requirement.minimum(),
                    current: player_attributes
                        .map_or(0, |attributes| attributes.value(requirement.attribute())),
                });
            }
        }
        if let Some(prerequisite) = definition.prerequisite()
            && !current_choices.contains(prerequisite)
        {
            return Err(TechniqueLearningError::MissingPrerequisite {
                technique: Box::new(technique.clone()),
                prerequisite: Box::new(prerequisite.clone()),
            });
        }

        let mut candidate = current_choices.to_vec();
        candidate.push(technique.clone());
        let continuation = catalog
            .analyze_discipline(&discipline, features, &candidate, rules)
            .map_err(|error| TechniqueLearningError::InvalidChoices(Box::new(error)))?;
        if !continuation.is_open() {
            return Err(TechniqueLearningError::WouldBlockProgression(Box::new(
                technique.clone(),
            )));
        }

        self.choices
            .entry(discipline.clone())
            .or_default()
            .push(technique.clone());
        Ok(TechniqueLearned {
            technique: technique.clone(),
            discipline,
            choice_number,
            cost,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TechniqueLearned {
    pub technique: TechniqueId,
    pub discipline: DisciplineId,
    pub choice_number: usize,
    pub cost: u16,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SkillProgressionStateError {
    Catalog(Box<SkillCatalogError>),
    DuplicateDiscipline(Box<DisciplineId>),
    EmptyDiscipline(Box<DisciplineId>),
    UnknownDiscipline(Box<DisciplineId>),
    DeferredDiscipline(Box<DisciplineId>),
    InvalidChoices {
        discipline: Box<DisciplineId>,
        error: Box<InitialChoicesError>,
    },
    BlockedProgression(Box<DisciplineId>),
}

impl Display for SkillProgressionStateError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Catalog(error) => write!(formatter, "invalid skill catalog: {error}"),
            Self::DuplicateDiscipline(discipline) => {
                write!(formatter, "duplicate saved discipline '{discipline}'")
            }
            Self::EmptyDiscipline(discipline) => {
                write!(formatter, "saved discipline '{discipline}' has no choices")
            }
            Self::UnknownDiscipline(discipline) => {
                write!(
                    formatter,
                    "saved discipline '{discipline}' no longer exists"
                )
            }
            Self::DeferredDiscipline(discipline) => write!(
                formatter,
                "saved discipline '{discipline}' is unavailable in this game version"
            ),
            Self::InvalidChoices { discipline, error } => {
                write!(
                    formatter,
                    "invalid choices for discipline '{discipline}': {error}"
                )
            }
            Self::BlockedProgression(discipline) => write!(
                formatter,
                "saved choices for discipline '{discipline}' cannot reach mastery"
            ),
        }
    }
}

impl Error for SkillProgressionStateError {}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TechniqueLearningError {
    InvalidCurrentState(Box<SkillProgressionStateError>),
    InvalidChoices(Box<InitialChoicesError>),
    UnknownTechnique(Box<TechniqueId>),
    DeferredDiscipline(Box<DisciplineId>),
    UnavailableTechnique(Box<TechniqueId>),
    AlreadyLearned(Box<TechniqueId>),
    LevelTooLow {
        technique: Box<TechniqueId>,
        required: u16,
        current: u16,
    },
    AttributeTooLow {
        technique: Box<TechniqueId>,
        attribute: PrimaryAttribute,
        required: u8,
        current: u8,
    },
    MissingPrerequisite {
        technique: Box<TechniqueId>,
        prerequisite: Box<TechniqueId>,
    },
    WouldBlockProgression(Box<TechniqueId>),
}

impl Display for TechniqueLearningError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCurrentState(error) => {
                write!(formatter, "invalid current skill progression: {error}")
            }
            Self::InvalidChoices(error) => write!(formatter, "invalid skill choices: {error}"),
            Self::UnknownTechnique(technique) => {
                write!(formatter, "unknown technique '{technique}'")
            }
            Self::DeferredDiscipline(discipline) => {
                write!(
                    formatter,
                    "discipline '{discipline}' is deferred in this version"
                )
            }
            Self::UnavailableTechnique(technique) => {
                write!(
                    formatter,
                    "technique '{technique}' is unavailable in this version"
                )
            }
            Self::AlreadyLearned(technique) => {
                write!(formatter, "technique '{technique}' is already learned")
            }
            Self::LevelTooLow {
                technique,
                required,
                current,
            } => write!(
                formatter,
                "technique '{technique}' requires level {required}, current level is {current}"
            ),
            Self::AttributeTooLow {
                technique,
                attribute,
                required,
                current,
            } => write!(
                formatter,
                "technique '{technique}' requires {attribute} {required}, current value is {current}"
            ),
            Self::MissingPrerequisite {
                technique,
                prerequisite,
            } => write!(
                formatter,
                "technique '{technique}' requires unlearned technique '{prerequisite}'"
            ),
            Self::WouldBlockProgression(technique) => write!(
                formatter,
                "learning technique '{technique}' would block later progression"
            ),
        }
    }
}

impl Error for TechniqueLearningError {}
