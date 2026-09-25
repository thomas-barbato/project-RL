//! The tags opt specific authored humanoids in; behavior alone is not a species.
use super::*;
use project_rl::item::EquipmentAffixId;
use project_rl::loot::EquipmentLootCatalog;

impl AsciiApp {
    #[cfg(any(test, debug_assertions))]
    pub(in crate::ascii_app) fn prepare_enemy_loot_diagnostic(
        &mut self,
        pick_up: bool,
    ) -> Result<(), String> {
        self.open_menu(MenuScreen::Main);
        self.enter_test_lab()?;
        let mut rules = self.game.rules().clone();
        rules.player_weapon_slots.clear();
        rules.player_starting_equipment.clear();
        rules.player_starting_weapons.clear();
        rules.player_base_attacks = vec![project_rl::combat::AttackProfile::melee(
            project_rl::combat::DamageType::Kinetic,
            1000,
        )];
        rules.hit_rules = None;
        let map = project_rl::world::Map::from_ascii("###############\n#.............#\n#.............#\n#.............#\n#.............#\n#.............#\n#.............#\n###############").map_err(|e| e.to_string())?;
        let at = GridPos::new(6, 4);
        let mut blueprint = ZoneBlueprint {
            info: project_rl::game::ZoneInfo {
                id: "core:enemy_loot_probe".parse().unwrap(),
                name: "Essai du butin".into(),
                kind: "core:human_habitat".parse().unwrap(),
                depth: 0,
            },
            map: map.clone(),
            entrance: GridPos::new(5, 4),
            seed: 108,
            actors: vec![
                Actor::new(at, 8)
                    .map_err(|e| e.to_string())?
                    .with_tags(["core:humanoid_knife_carrier".parse().unwrap()]),
            ],
            loot: vec![],
            threat_sources: vec![],
        };
        let template = blueprint.actors[0].clone();
        for seed in 0..256 {
            blueprint.actors[0] = template.clone();
            self.populate_carried_weapons(&mut blueprint, seed)?;
            if blueprint.actors[0]
                .equipped_weapon()
                .and_then(|weapon| weapon.modifiers())
                .is_some()
            {
                break;
            }
        }
        let mut game = GameState::new_with_rules(map, blueprint.entrance, 108, rules)
            .map_err(|e| e.to_string())?;
        let target = game
            .spawn_actor(blueprint.actors.remove(0))
            .map_err(|e| e.to_string())?;
        if game.process_player_command(GameCommand::Attack { slot: 0, target })
            != project_rl::game::CommandOutcome::Applied
            || game.actors().get(target).is_some()
            || game.ground_items().count_at(at) != 1
        {
            return Err("The real diagnostic kill did not leave its weapon".into());
        }
        self.terminal = TerminalView::new(
            crate::test_sector::SectorDecor::default(),
            game.map(),
            game.player_visibility(),
        );
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.selected_target = None;
        self.intro_city_reached = true;
        self.log.clear();
        self.capture_events();
        if pick_up {
            self.execute_command(GameCommand::Move(Direction::East));
            self.execute_command(GameCommand::PickUp);
            self.inventory_filter = InventoryFilter::All;
            self.inventory_selection = 0;
            self.inventory_open = true;
        }
        Ok(())
    }

    pub(in crate::ascii_app) fn populate_carried_weapons(
        &self,
        blueprint: &mut ZoneBlueprint,
        seed: u64,
    ) -> Result<(), String> {
        self.equip_carried_weapons(&mut blueprint.actors, blueprint.info.depth, seed)
    }

    pub(in crate::ascii_app) fn equip_carried_weapons(
        &self,
        actors: &mut [Actor],
        depth: u16,
        seed: u64,
    ) -> Result<(), String> {
        for actor in actors {
            let rifle = actor
                .tags()
                .iter()
                .any(|id| id.as_str() == "core:humanoid_rifle_carrier");
            let knife = actor
                .tags()
                .iter()
                .any(|id| id.as_str() == "core:humanoid_knife_carrier");
            if !rifle && !knife {
                continue;
            }
            if rifle && knife
                || actor.electronic_system().is_some()
                || actor.equipped_weapon().is_some()
            {
                return Err("Ambiguous or already equipped humanoid carrier".into());
            }
            let family = if rifle { "core:rifles" } else { "core:knives" };
            let mut pool = EquipmentLootCatalog::default();
            for (_, base) in self.loot.equipment().iter().filter(|(_, base)| {
                base.family.as_str() == family
                    && base.tier <= depth.saturating_add(1).min(6) as u8
                    && base.sources.contains(&EquipmentSource::HumanoidEquipped)
            }) {
                let mut base = base.clone();
                // These two NPCs have no heat/energy gauges. Keep applicable
                // combat properties and their original relative rarity weights.
                base.stats.retain(|stat| {
                    !matches!(
                        stat.affix,
                        EquipmentAffixId::EnergyReserve | EquipmentAffixId::HeatDissipation
                    )
                });
                pool.register(base, &self.game.rules().items, &self.game.rules().weapons)?;
            }
            let at = actor.position();
            let mut rng = GameRng::from_seed(
                seed ^ 0x4e50_4357_4541_504e
                    ^ (u64::from(at.x as u32) << 32)
                    ^ u64::from(at.y as u32),
            );
            let rolled = pool
                .draw(
                    EquipmentSource::HumanoidEquipped,
                    depth,
                    EquipmentQuality::Random {
                        enchanted_percent: 35,
                    },
                    &mut rng,
                )?
                .ok_or("No compatible weapon for an authored humanoid carrier")?;
            *actor = actor.clone().with_equipped_weapon(
                rolled.item,
                rolled.modifiers,
                &self.game.rules().weapons,
            )?;
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "enemy_equipment_tests.rs"]
mod tests;
