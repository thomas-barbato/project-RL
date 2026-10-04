//! Resolve authored hub data into the seeded physical placement.
use super::*;
use crate::hub_layout::HomePlacement;
impl AsciiApp {
    pub(super) fn home_layout(&self) -> HomePlacement {
        self.home_placement
    }
    pub(super) fn home_position(&self, p: GridPos) -> GridPos {
        self.home_layout().position(p)
    }
    pub(super) fn home_regional_passages(&self) -> [(Direction, GridPos); 4] {
        self.home_layout().regional_passages()
    }
    pub(super) fn starter_hub_definition(
        &self,
    ) -> Result<project_rl::content::ExpeditionDefinition, String> {
        let mut definition = self
            .expeditions
            .get(&"core:starter_expedition".parse().unwrap())
            .ok_or("Définition d'expédition de départ absente.")?
            .clone();
        if self.generation_version < RANDOM_HOME_GENERATION_VERSION {
            return Ok(definition);
        }
        let layout = self.home_layout();
        if let Some(facility) = &mut definition.hub_facility {
            for installation in &mut facility.blueprint.installations {
                installation.position = layout.position(installation.position);
                for capability in &mut installation.capabilities {
                    if let InstallationCapability::DoorActuator { door } = capability {
                        *door = layout.position(*door);
                    }
                }
            }
            for worker in &mut facility.blueprint.workers {
                worker.actor_position = layout.position(worker.actor_position);
                if let Some(report) = &mut worker.property_report {
                    report.recipient_position = layout.position(report.recipient_position);
                }
            }
            for material in &mut facility.materials {
                // The introduction moves this quest item out of the plaza.
                let position = if material.position == GridPos::new(16, 23)
                    && material.item.as_str() == "core:power_regulator"
                {
                    TestSector::RECYCLING_REPAIR_PART
                } else {
                    material.position
                };
                material.position = layout.position(position);
            }
        }
        if let Some(merchant) = &mut definition.hub_merchant {
            merchant.position = layout.position(merchant.position);
        }
        if let Some(clinic) = &mut definition.hub_clinic {
            clinic.work_position = layout.position(clinic.work_position);
            clinic.break_position = layout.position(clinic.break_position);
        }
        for resident in &mut definition.hub_residents {
            resident.residence_position = layout.position(resident.residence_position);
            resident.gathering_position = layout.position(resident.gathering_position);
        }
        for quest in &mut definition.hub_quests {
            match &mut quest.provider {
                HubQuestProviderDefinition::Existing { position }
                | HubQuestProviderDefinition::Contact { position, .. } => {
                    *position = layout.position(*position)
                }
            }
            for effect in &mut quest.completion_world_effects {
                if let project_rl::content::QuestWorldEffectDefinition::UnlockDoor {
                    position,
                    ..
                } = effect
                {
                    *position = layout.position(*position);
                }
            }
        }
        if let Some(narrative) = &mut definition.narrative {
            for character in &mut narrative.characters {
                if let Some([x, y]) = character.hub_position {
                    let p = layout.position(GridPos::new(x, y));
                    character.hub_position = Some([p.x, p.y]);
                }
            }
        }
        Ok(definition)
    }
}
