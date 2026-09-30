use crate::test_sector::{SectorDecor, Zone};
use project_rl::content::{ContentId, FirstLayerPlace, RegionLootProfile};
use project_rl::game::GameRng;
use project_rl::loot::{LootCatalog, LootContext, LootTable};
use project_rl::world::GridPos;
use project_rl::world::generation::FirstLayerLandscape;

#[cfg(any(test, debug_assertions))]
pub(super) fn diagnostic_features() -> crate::test_regional::RegionalGenerationFeatures {
    crate::test_regional::RegionalGenerationFeatures {
        vertical_travel: true,
        population: true,
        encounters: true,
        exploration_variety: true,
        exploration_salvage: true,
        exploration_salvage_breaches: true,
        pursuit_lifecycle: true,
        primary_attributes: true,
        physical_profiles: true,
        loot: true,
        landmarks: true,
        sites: true,
        site_interactions: true,
        site_security: true,
        site_terminals: true,
        reinforcement_investigation: true,
        site_navigation_signals: true,
        site_terminal_navigation_signals: true,
        threat_renewal: true,
        destructibles: true,
        environmental_conduction: true,
        electronic_systems: true,
        player_relations: true,
    }
}

#[cfg(any(test, debug_assertions))]
impl super::AsciiApp {
    pub(super) fn prepare_first_layer_landscape_diagnostic(
        &mut self,
        scene: &str,
    ) -> Result<(), String> {
        use super::*;
        use project_rl::game::ZoneInfo;
        let role = if scene.contains("pumps") {
            FirstLayerPlace::Pumps
        } else {
            FirstLayerPlace::Workshops
        };
        let source = self
            .regional_worlds
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap();
        let plan = source
            .first_layer_plan(self.seed)
            .ok_or("Layer plan missing")?;
        let coordinate = plan.places.iter().find(|(r, _)| *r == role).unwrap().1;
        let world = source.resolved_for_seed(self.seed);
        let descriptor = world.region(self.seed, coordinate).unwrap();
        let entrance = project_rl::world::generation::cardinal_passage(
            world.map_size_at(coordinate),
            Direction::West,
        );
        let mut generated = crate::test_regional::generate(
            &world,
            &descriptor,
            crate::test_regional::zone_info(&world, &descriptor)?,
            entrance,
            Some(&self.loot),
            diagnostic_features(),
        )?;
        self.populate_regional_equipment(
            &mut generated.blueprint,
            &descriptor.biome,
            descriptor.seed,
        )?;
        self.populate_carried_weapons(&mut generated.blueprint, descriptor.seed)?;
        let bounds = generated.decor.zones.first().ok_or("Annex absent")?.bounds;
        let position = if role == FirstLayerPlace::Pumps {
            GridPos::new(59, 26)
        } else {
            GridPos::new(bounds[0] + 3, bounds[1] + 4)
        };
        if generated
            .blueprint
            .actors
            .iter()
            .any(|a| a.position() == position)
        {
            return Err("Occupied diagnostic viewpoint".into());
        }
        let refuge = ZoneInfo {
            id: "test:landscape_refuge".parse().unwrap(),
            name: "Diagnostic".into(),
            kind: "core:human_habitat".parse().unwrap(),
            depth: 1,
        };
        let mut game = WorldState::single(
            GameState::new_with_rules(
                project_rl::world::Map::from_ascii("#####\n#...#\n#####").unwrap(),
                GridPos::new(1, 1),
                self.seed,
                self.rules.clone(),
            )
            .map_err(|e| e.to_string())?,
        );
        game.enable(refuge.clone())?;
        let info = generated.blueprint.info.clone();
        game.add_zone(generated.blueprint)?;
        if let Some(facility) = generated.facility {
            game.register_facility(info.id.clone(), facility)?;
        }
        game.connect(refuge.id, GridPos::new(2, 1), info.id, position)?;
        if game.process_player_command(GameCommand::Interact {
            target: GridPos::new(2, 1),
        }) != CommandOutcome::Applied
        {
            return Err("Diagnostic entry rejected".into());
        }
        self.terminal = TerminalView::new(generated.decor, game.map(), game.player_visibility());
        self.terminal.title = name(role).into();
        self.game = game;
        self.actor_glyphs.clear();
        self.selected_target = None;
        self.intro_city_reached = false;
        self.log.clear();
        self.capture_events_at(Some(0.0));
        Ok(())
    }
}

pub(crate) fn name(role: FirstLayerPlace) -> &'static str {
    match role {
        FirstLayerPlace::City => "Nœud de maintenance",
        FirstLayerPlace::Workshops => "Ateliers",
        FirstLayerPlace::Conduits => "Conduits",
        FirstLayerPlace::Pumps => "Station de pompage",
        FirstLayerPlace::Gallery => "Galerie de service",
        FirstLayerPlace::Descent => "Accès inférieur",
    }
}

pub(crate) fn decorate_landscape(decor: &mut SectorDecor, landscape: &FirstLayerLandscape) {
    decor.fallback_name = Some(name(landscape.role).into());
    if let Some(bounds) = landscape.annex {
        decor.zones.push(Zone {
            name: if landscape.role == FirstLayerPlace::Workshops {
                "Réserve des ateliers".into()
            } else {
                "Réserve de service".into()
            },
            bounds,
        });
    }
}

/// Narrow one budgeted cache draw, keeping authored depth, source, quantities
/// and relative weights. Equipment then goes through the ordinary adapter.
pub(crate) fn specialize_annex_reward(
    landscape: &FirstLayerLandscape,
    drops: &mut [(GridPos, ContentId, u16)],
    profile: Option<&RegionLootProfile>,
    catalog: Option<&LootCatalog>,
    biome: &ContentId,
    depth: u16,
    seed: u64,
) -> Result<(), String> {
    let Some(cache) = landscape.cache else {
        return Ok(());
    };
    let (Some(profile), Some(catalog)) = (profile, catalog) else {
        return Err("Annex reward profile missing".into());
    };
    let source = catalog
        .get(profile.table())
        .ok_or("Annex loot table missing")?;
    let entries = source
        .entries()
        .iter()
        .filter(|entry| {
            if landscape.role == FirstLayerPlace::Workshops {
                matches!(
                    entry.item.as_str(),
                    "core:integrity_blade"
                        | "core:needle_launcher"
                        | "core:patched_plating"
                        | "core:composite_carapace"
                )
            } else {
                matches!(
                    entry.item.as_str(),
                    "core:repair_patch" | "core:charged_battery"
                )
            }
        })
        .cloned()
        .collect();
    let table = LootTable::new(profile.table().clone(), entries).map_err(|e| e.to_string())?;
    let mut rng = GameRng::from_seed(seed ^ 0x414e_4e45_585f_4c54);
    let mut rolled = table
        .draw(
            &LootContext {
                depth,
                map_kind: biome.clone(),
                source: profile.source().clone(),
            },
            1,
            &mut rng,
        )
        .map_err(|e| e.to_string())?;
    let item = rolled.pop().ok_or("Annex reward draw empty")?;
    let drop = drops
        .iter_mut()
        .find(|d| d.0 == cache)
        .ok_or("Annex cache has no budgeted draw")?;
    drop.1 = item.item;
    drop.2 = item.quantity;
    Ok(())
}
