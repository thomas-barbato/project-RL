//! Campaign adapter for PLACE-CAMP-01 r09b. Reuses existing service rules.
use super::*;
use crate::cryogenic_camp::{self as camp, pos};
#[cfg(debug_assertions)]
#[path = "art_finish_capture.rs"]
mod art_finish_capture;
#[cfg(debug_assertions)]
#[path = "art_finish_capture_02.rs"]
mod art_finish_capture_02;
#[cfg(debug_assertions)]
#[path = "art_finish_capture_03.rs"]
mod art_finish_capture_03;
#[cfg(debug_assertions)]
#[path = "art_finish_capture_04.rs"]
mod art_finish_capture_04;
#[cfg(debug_assertions)]
#[path = "art_finish_capture_05.rs"]
mod art_finish_capture_05;
#[cfg(debug_assertions)]
#[path = "art_finish_completion_capture.rs"]
mod art_finish_completion_capture;
#[cfg(debug_assertions)]
#[path = "cryogenic_camp_capture.rs"]
mod capture;
#[cfg(debug_assertions)]
#[path = "sewer_review_capture.rs"]
mod sewer_review_capture;
#[cfg(debug_assertions)]
#[path = "street_defense_capture.rs"]
mod street_defense_capture;
#[cfg(debug_assertions)]
#[path = "street_population_app.rs"]
mod street_population_app;
#[cfg(debug_assertions)]
#[path = "street_repairs_capture.rs"]
mod street_repairs_capture;
#[cfg(test)]
#[path = "cryogenic_camp_tests.rs"]
mod tests;
#[cfg(debug_assertions)]
#[path = "torso_icons_capture.rs"]
mod torso_icons_capture;
#[cfg(debug_assertions)]
#[path = "world_fixes_capture.rs"]
mod world_fixes_capture;

impl AsciiApp {
    /// City-specific regression fixtures must keep testing their original map.
    #[cfg(test)]
    pub(super) fn from_legacy_city_seed(
        seed: u64,
        rules: GameRules,
        texts: TextCatalog,
        loot: LootCatalog,
        expeditions: ExpeditionCatalog,
    ) -> Result<Self, String> {
        Self::from_seed_version(
            seed,
            rules,
            texts,
            loot,
            expeditions,
            MONSTER_BLUE_PALETTES_GENERATION_VERSION,
        )
    }

    pub(super) fn enable_cryogenic_camp(&mut self) -> Result<(), String> {
        let scene = camp::scene();
        let definition = self
            .expeditions
            .get(&"core:starter_expedition".parse().unwrap())
            .ok_or("Définition d’expédition absente")?
            .clone();
        let zone: ContentId = camp::ZONE_ID.parse().unwrap();
        self.game.enable(project_rl::game::ZoneInfo {
            id: zone.clone(),
            name: camp::NAME.into(),
            kind: definition.hub.kind.clone(),
            depth: 0,
        })?;
        let destination = project_rl::game::ZoneInfo {
            id: definition.destination.zone.id.clone(),
            name: definition.destination.zone.name.clone(),
            kind: definition.destination.zone.kind.clone(),
            depth: definition.destination.zone.depth,
        };
        // Generation 154 and older keep their exact deferred destination.
        let destination = if self.generation_version >= PRISON_STREET_GENERATION_VERSION {
            crate::prison_street::info(destination)
        } else {
            destination
        };
        self.game
            .declare_deferred_connection(zone.clone(), pos(scene.exits[0]), destination)?;
        let mut merchant = definition
            .hub_merchant
            .clone()
            .ok_or("Commerce de départ absent")?;
        let mut clinic = definition
            .hub_clinic
            .clone()
            .ok_or("Clinique de départ absente")?;
        for (index, sprite) in scene.actors.iter().enumerate().skip(1) {
            let index = index as u8;
            let mut tags = vec![format!("core:camp_npc_{index:02}").parse().unwrap()];
            if index == 1 {
                tags.push("core:conversation_contact".parse().unwrap());
            }
            if index == 4 {
                tags.push("core:equipment_artisan".parse().unwrap());
            }
            let provider = self
                .game
                .spawn_actor(
                    Actor::new(
                        pos(sprite.at),
                        if index == 3 {
                            clinic.maximum_integrity
                        } else {
                            merchant.maximum_integrity
                        },
                    )
                    .map_err(|e| e.to_string())?
                    .with_ai(AiProfile::idle())
                    .with_tags(tags),
                )
                .map_err(|e| e.to_string())?;
            if index == 2 {
                merchant.position = pos(sprite.at);
                self.register_local_equipment_merchant(
                    zone.clone(),
                    provider,
                    definition.player_starting_credits,
                    merchant.clone(),
                    self.seed ^ 0x434f_4d4d_4552_4345,
                )?;
            } else if index == 3 {
                clinic.work_position = pos(sprite.at);
                clinic.break_position = pos(sprite.at);
                self.game.register_clinic(
                    zone.clone(),
                    provider,
                    definition.player_starting_credits,
                    clinic.clone(),
                )?;
            }
        }
        self.refresh_zone_title();
        self.terminal
            .observe(self.game.map(), self.game.player_visibility());
        Ok(())
    }

    pub(super) fn camp_npc_name(&self, entity: EntityId) -> Option<&'static str> {
        self.game
            .actors()
            .get(entity)
            .and_then(camp::actor_index)
            .map(camp::npc_name)
    }
}
