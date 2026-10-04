use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::{CommandOutcome, CommandRejection, QuestStatus, WorldState};
use crate::content::{
    ContentId, DialogueAction, DialogueCondition, NarrativeCharacter, NarrativeDefinition,
};
use crate::entity::EntityId;
use crate::game::GameCommand;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(super) struct NarrativeState {
    pub definition: NarrativeDefinition,
    pub records: BTreeSet<ContentId>,
    pub cursors: BTreeMap<(ContentId, EntityId), String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DialogueView {
    pub name_key: String,
    pub role_key: String,
    pub node: String,
    pub text_key: String,
    pub choices: Vec<DialogueChoiceView>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DialogueChoiceView {
    pub index: u16,
    pub text_key: String,
}

impl WorldState {
    pub fn is_known_fact_objective(&self, record: &ContentId) -> bool {
        self.quests.values().flatten().any(|quest| {
            matches!(&quest.definition,
            super::QuestDefinition::KnownFact(definition) if &definition.record == record)
        })
    }

    /// Mark only directly perceived, still-relevant quest elements. This read
    /// does not add knowledge and never exposes an unseen source or destination.
    pub fn quest_marker_at(&self, position: crate::world::GridPos) -> Option<super::QuestMarker> {
        use super::{QuestMarker, QuestObjectiveView};
        if !self.active.player_visibility().is_visible(position) {
            return None;
        }
        let entity = self.active.actors().entity_at(position);
        if let Some(marker) = entity.and_then(|id| self.quest_marker(id)) {
            return Some(marker);
        }
        let actor = entity.and_then(|id| self.active.actors().get(id));
        let terminal_record = self
            .active_facility()
            .and_then(|facility| facility.data_terminal_record_at(position));
        let character = entity.and_then(|id| self.narrative_character(id));
        let ground = self
            .active
            .ground_items()
            .item_at(position)
            .and_then(|id| self.active.ground_items().get(id));
        for quest in self
            .quests
            .values()
            .flatten()
            .filter(|quest| quest.accepted && !quest.completed && !quest.excluded)
        {
            let view = self.quest_view(quest);
            if view.status != QuestStatus::Active {
                continue;
            }
            let relevant = match view.objective {
                QuestObjectiveView::AccessDataRecord { record, accessed: false } => {
                    terminal_record == Some(&record)
                        || character.is_some_and(|character| character.nodes.iter().flat_map(|node| &node.choices)
                            .any(|choice| matches!(&choice.action, Some(DialogueAction::LearnRecord(id)) if id == &record)))
                }
                QuestObjectiveView::Delivery { required_item, .. } => ground.is_some_and(|stack| stack.item() == &required_item),
                QuestObjectiveView::DefeatTargets { target_tag, .. } => actor.is_some_and(|actor| actor.tags().contains(&target_tag)),
                QuestObjectiveView::ExploreZones { .. } => {
                    if let (super::QuestDefinition::ExploreZones(definition), super::QuestProgress::ExploreZones { baseline, discovered }) = (&quest.definition, &quest.progress) {
                        terminal_record.is_some_and(|record| definition.qualifying_records.contains(record))
                            && self.current.as_ref().is_some_and(|zone| !baseline.contains(zone) && !discovered.contains(zone))
                    } else { false }
                }
                _ => false,
            };
            if relevant {
                return Some(QuestMarker::Objective);
            }
        }
        None
    }

    pub fn register_narrative(&mut self, definition: NarrativeDefinition) -> Result<(), String> {
        definition.validate()?;
        if self.narrative.is_some() {
            return Err("A narrative is already registered".into());
        }
        self.narrative = Some(NarrativeState {
            definition,
            records: BTreeSet::new(),
            cursors: BTreeMap::new(),
        });
        Ok(())
    }

    pub(super) fn narrative_character(&self, provider: EntityId) -> Option<&NarrativeCharacter> {
        let actor = self.active.actors().get(provider)?;
        self.narrative
            .as_ref()?
            .definition
            .characters
            .iter()
            .find(|character| actor.tags().contains(&character.tag))
    }

    pub fn bind_narrative_character(
        &mut self,
        provider: EntityId,
        tag: &ContentId,
    ) -> Result<(), String> {
        if !self.narrative.as_ref().is_some_and(|state| {
            state
                .definition
                .characters
                .iter()
                .any(|character| &character.tag == tag)
        }) {
            return Err("Unknown narrative character".into());
        }
        let actor = self
            .active
            .actors
            .get_mut(provider)
            .ok_or("Narrative actor missing")?;
        let mut tags = actor.tags().clone();
        tags.insert(tag.clone());
        *actor = actor.clone().with_tags(tags);
        Ok(())
    }

    pub fn narrative_name_key_in(&self, zone: &ContentId, provider: EntityId) -> Option<&str> {
        let actor = if self.current.as_ref() == Some(zone) {
            self.active.actors().get(provider)
        } else {
            self.inactive.get(zone)?.actors.get(provider)
        }?;
        self.narrative
            .as_ref()?
            .definition
            .characters
            .iter()
            .find(|character| actor.tags().contains(&character.tag))
            .map(|character| character.name_key.as_str())
    }

    pub fn dialogue_view(&self, provider: EntityId) -> Option<DialogueView> {
        // Use the same physical, perceptual and life-state boundary as services.
        self.npc_interaction(provider)?;
        let character = self.narrative_character(provider)?;
        let state = self.narrative.as_ref()?;
        let cursor = state
            .cursors
            .get(&(self.current.clone()?, provider))
            .unwrap_or(&character.entry);
        let node = character.nodes.iter().find(|node| &node.id == cursor)?;
        Some(DialogueView {
            name_key: character.name_key.clone(),
            role_key: character.role_key.clone(),
            node: node.id.clone(),
            text_key: node.text_key.clone(),
            choices: node
                .choices
                .iter()
                .enumerate()
                .filter_map(|(index, choice)| {
                    choice
                        .conditions
                        .iter()
                        .all(|condition| self.dialogue_condition(provider, condition))
                        .then(|| DialogueChoiceView {
                            index: index as u16,
                            text_key: choice.text_key.clone(),
                        })
                })
                .collect(),
        })
    }

    fn dialogue_condition(&self, provider: EntityId, condition: &DialogueCondition) -> bool {
        match condition {
            DialogueCondition::KnowsRecord(record) => {
                self.discovered_data_terminal_records().contains(record)
            }
            DialogueCondition::DoesNotKnowRecord(record) => {
                !self.discovered_data_terminal_records().contains(record)
            }
            _ => self
                .npc_quest_views(provider)
                .iter()
                .any(|quest| match condition {
                    DialogueCondition::QuestAvailable(id) => {
                        &quest.id == id && quest.status == QuestStatus::Available
                    }
                    DialogueCondition::QuestActive(id) => {
                        &quest.id == id && quest.status == QuestStatus::Active
                    }
                    DialogueCondition::QuestReady(id) => {
                        &quest.id == id && quest.status == QuestStatus::ReadyToComplete
                    }
                    DialogueCondition::QuestCompleted(id) => {
                        &quest.id == id && quest.status == QuestStatus::Completed
                    }
                    _ => false,
                }),
        }
    }

    pub(super) fn choose_dialogue(
        &mut self,
        provider: EntityId,
        node_id: &str,
        index: u16,
    ) -> CommandOutcome {
        let Some(view) = self.dialogue_view(provider) else {
            return CommandOutcome::Rejected(CommandRejection::InteractionOutOfReach);
        };
        if view.node != node_id || !view.choices.iter().any(|choice| choice.index == index) {
            return CommandOutcome::Rejected(CommandRejection::DialogueChoiceUnavailable);
        }
        let choice = self
            .narrative_character(provider)
            .unwrap()
            .nodes
            .iter()
            .find(|node| node.id == node_id)
            .unwrap()
            .choices[index as usize]
            .clone();
        let zone = self.current.clone().unwrap();
        let outcome = match &choice.action {
            Some(DialogueAction::AcceptQuest(quest)) => self.accept_quest(
                provider,
                quest,
                GameCommand::AcceptQuest {
                    giver: provider,
                    quest: quest.clone(),
                },
            ),
            Some(DialogueAction::CompleteQuest(quest)) => self.complete_quest(
                provider,
                quest,
                GameCommand::CompleteQuest {
                    giver: provider,
                    quest: quest.clone(),
                },
            ),
            _ => {
                let outcome = self
                    .active
                    .process_player_command(GameCommand::ChooseDialogue {
                        speaker: provider,
                        node: node_id.to_owned(),
                        choice: index,
                    });
                if outcome == CommandOutcome::AppliedWithoutTime
                    && let Some(DialogueAction::LearnRecord(record)) = &choice.action
                {
                    self.narrative
                        .as_mut()
                        .unwrap()
                        .records
                        .insert(record.clone());
                    self.record_data_record_quest_progress(record);
                }
                outcome
            }
        };
        if matches!(
            outcome,
            CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime
        ) {
            self.narrative
                .as_mut()
                .unwrap()
                .cursors
                .insert((zone, provider), choice.next);
        }
        outcome
    }
}
