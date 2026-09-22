//! Bounded, localized conversations. Dialogue only changes knowledge or uses
//! an existing quest action; physical work remains a simulation command.
use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use super::ContentId;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NarrativeDefinition {
    pub characters: Vec<NarrativeCharacter>,
    pub investigation: NarrativeInvestigation,
    #[serde(with = "readable_id")]
    pub relay_character: ContentId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NarrativeInvestigation {
    #[serde(with = "readable_id")]
    pub id: ContentId,
    #[serde(with = "readable_id")]
    pub provider_tag: ContentId,
    #[serde(with = "readable_id")]
    pub record: ContentId,
    pub title_key: String,
    pub summary_key: String,
    pub reward_credits: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NarrativeCharacter {
    #[serde(with = "readable_id")]
    pub tag: ContentId,
    pub name_key: String,
    pub role_key: String,
    pub entry: String,
    pub nodes: Vec<DialogueNode>,
    pub hub_position: Option<[i32; 2]>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DialogueNode {
    pub id: String,
    pub text_key: String,
    pub choices: Vec<DialogueChoice>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DialogueChoice {
    pub text_key: String,
    pub next: String,
    #[serde(default)]
    pub conditions: Vec<DialogueCondition>,
    pub action: Option<DialogueAction>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DialogueCondition {
    KnowsRecord(#[serde(with = "readable_id")] ContentId),
    DoesNotKnowRecord(#[serde(with = "readable_id")] ContentId),
    QuestAvailable(#[serde(with = "readable_id")] ContentId),
    QuestActive(#[serde(with = "readable_id")] ContentId),
    QuestReady(#[serde(with = "readable_id")] ContentId),
    QuestCompleted(#[serde(with = "readable_id")] ContentId),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DialogueAction {
    LearnRecord(#[serde(with = "readable_id")] ContentId),
    AcceptQuest(#[serde(with = "readable_id")] ContentId),
    CompleteQuest(#[serde(with = "readable_id")] ContentId),
}

mod readable_id {
    use super::*;
    pub fn serialize<S: serde::Serializer>(
        id: &ContentId,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        if serializer.is_human_readable() {
            serializer.serialize_str(id.as_str())
        } else {
            id.serialize(serializer)
        }
    }
    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<ContentId, D::Error> {
        if deserializer.is_human_readable() {
            String::deserialize(deserializer)?
                .parse()
                .map_err(serde::de::Error::custom)
        } else {
            ContentId::deserialize(deserializer)
        }
    }
}

impl NarrativeDefinition {
    pub fn validate(&self) -> Result<(), String> {
        if self.characters.is_empty() || self.characters.len() > 16 {
            return Err("A narrative needs between 1 and 16 characters".into());
        }
        let mut tags = BTreeSet::new();
        let mut hub_positions = BTreeSet::new();
        for character in &self.characters {
            if let Some(position) = character.hub_position
                && (position.iter().any(|coordinate| *coordinate < 0)
                    || !hub_positions.insert(position))
            {
                return Err("Invalid or duplicate narrative hub position".into());
            }
            if !tags.insert(&character.tag)
                || character.name_key.trim().is_empty()
                || character.role_key.trim().is_empty()
                || character.nodes.is_empty()
                || character.nodes.len() > 64
            {
                return Err("Invalid or duplicate narrative character".into());
            }
            let mut nodes = BTreeSet::new();
            for node in &character.nodes {
                if node.id.trim().is_empty()
                    || node.id.len() > 128
                    || !nodes.insert(&node.id)
                    || node.text_key.trim().is_empty()
                    || node.choices.is_empty()
                    || node.choices.len() > 12
                    || node.choices.iter().any(|choice| {
                        choice.text_key.trim().is_empty() || choice.conditions.len() > 8
                    })
                {
                    return Err("Invalid or duplicate dialogue node".into());
                }
            }
            if !nodes.contains(&character.entry)
                || character
                    .nodes
                    .iter()
                    .flat_map(|node| &node.choices)
                    .any(|choice| !nodes.contains(&choice.next))
            {
                return Err("Dialogue references an unknown node".into());
            }
            for choice in character.nodes.iter().flat_map(|node| &node.choices) {
                if let Some(
                    DialogueAction::AcceptQuest(quest) | DialogueAction::CompleteQuest(quest),
                ) = &choice.action
                    && (quest != &self.investigation.id
                        || character.tag != self.investigation.provider_tag)
                {
                    return Err("Dialogue references an unknown quest or the wrong provider".into());
                }
            }
        }
        if !tags.contains(&self.relay_character)
            || !tags.contains(&self.investigation.provider_tag)
            || self.investigation.title_key.trim().is_empty()
            || self.investigation.summary_key.trim().is_empty()
        {
            return Err("Invalid narrative investigation or relay character".into());
        }
        Ok(())
    }

    pub fn text_keys(&self) -> impl Iterator<Item = &str> {
        [
            self.investigation.title_key.as_str(),
            self.investigation.summary_key.as_str(),
        ]
        .into_iter()
        .chain(self.characters.iter().flat_map(|character| {
            [character.name_key.as_str(), character.role_key.as_str()]
                .into_iter()
                .chain(character.nodes.iter().flat_map(|node| {
                    std::iter::once(node.text_key.as_str())
                        .chain(node.choices.iter().map(|choice| choice.text_key.as_str()))
                }))
        }))
    }
}
