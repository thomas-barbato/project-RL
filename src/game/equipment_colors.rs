use super::{GameRng, GameState};
use crate::entity::EquipmentPalette;

impl GameState {
    /// Assign only unassigned equipment, after a committed creation/action.
    /// `Some(Original)` is a completed draw and must never be drawn again.
    /// This stream belongs to the run, survives travel, and is persisted in
    /// the active snapshot. Zone setup uses the zone's own domain-separated seed.
    pub(super) fn assign_equipment_palettes(&mut self) {
        if !self.rules.equipment_color_variants {
            return;
        }
        let eligible = |item: &crate::item::ItemId| {
            self.rules.weapons.get(item).is_some()
                || self
                    .rules
                    .items
                    .get(item)
                    .and_then(|definition| definition.equipment())
                    .is_some()
        };
        let inventory: Vec<_> = self
            .player_inventory()
            .iter()
            .filter(|entry| entry.color_variant().is_none() && eligible(entry.item()))
            .map(|entry| entry.instance())
            .collect();
        let ground: Vec<_> = self
            .ground_items()
            .iter()
            .filter(|(_, entry)| entry.color_variant().is_none() && eligible(entry.item()))
            .map(|(id, _)| id)
            .collect();
        // A missing stream is possible only in a JSON snapshot predating this
        // feature; generation rules keep historical runs disabled.
        let mut rng = self
            .cosmetic_rng
            .unwrap_or_else(|| GameRng::from_seed(self.rng.state() ^ 0x5041_4c45_5454_4553));
        for instance in inventory {
            let definition = self.player_inventory().get(instance).unwrap().item();
            let palette = cosmetic_palette(definition.as_str(), rng.next_u64());
            self.player_inventory_mut()
                .set_palette(instance, palette)
                .expect("existing equipment");
        }
        for id in ground {
            let definition = self.ground_items.get(id).unwrap().item();
            let palette = cosmetic_palette(definition.as_str(), rng.next_u64());
            self.ground_items.set_palette(id, palette);
        }
        self.cosmetic_rng = Some(rng);
    }
}

fn cosmetic_palette(id: &str, roll: u64) -> EquipmentPalette {
    let authored = matches!(
        id,
        "core:tenue_assaut"
            | "core:tenue_technique"
            | "core:tenue_traque"
            | "core:tenue_rituelle"
            | "core:tenue_metamorphe"
            | "core:patched_plating"
            | "core:veste_de_membranes"
            | "core:veste_de_peau_seconde"
            | "core:veste_de_veille"
    );
    let authored_palettes = [
        EquipmentPalette::Original,
        EquipmentPalette::ReviewedVariant1,
        EquipmentPalette::ReviewedVariant2,
    ];
    let previous = if authored {
        &authored_palettes[..]
    } else {
        &EquipmentPalette::ALL[..]
    };
    if matches!(
        id,
        "core:armure_cerbere"
            | "core:brigandine_cognefer"
            | "core:carapace_chrysalide"
            | "core:composite_carapace"
            | "core:combinaison_fantome"
            | "core:manteau_de_mue"
            | "core:plastron_bastion"
            | "core:patched_plating"
            | "core:veste_de_la_seconde_ombre"
            | "core:veste_de_membranes"
            | "core:veste_de_peau_seconde"
            | "core:veste_de_veille"
            | "core:veste_matelassee"
            | "core:veste_a_fibres_croisees"
            | "core:tenue_assaut"
            | "core:tenue_technique"
            | "core:tenue_traque"
            | "core:tenue_rituelle"
            | "core:tenue_metamorphe"
    ) {
        // All previously available paints stay in the pool. The red robe
        // already has a red original; do not draw a duplicate DeepRed entry.
        let added = if matches!(id, "core:veste_de_la_seconde_ombre" | "core:tenue_rituelle") {
            3
        } else {
            4
        };
        let index = (roll % (previous.len() + added) as u64) as usize;
        if index < previous.len() {
            previous[index]
        } else {
            EquipmentPalette::TORSO_FINISH[index - previous.len()]
        }
    } else {
        previous[(roll % previous.len() as u64) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combat::{AttackProfile, DamageType};
    use crate::content::{ContentId, ContentLoader};
    use crate::entity::{Actor, MagicItemModifiers};
    use crate::game::{CommandOutcome, EquipmentUpgradeRules, GameCommand, GameRules, WorldState};
    use crate::world::{GridPos, Map, Terrain};
    use std::collections::BTreeSet;

    fn id(name: &str) -> ContentId {
        format!("core:{name}").parse().unwrap()
    }
    fn rules(enabled: bool) -> GameRules {
        static RULES: std::sync::OnceLock<GameRules> = std::sync::OnceLock::new();
        let mut rules = RULES
            .get_or_init(|| {
                let content = ContentLoader::load(
                    &[std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content")],
                    &semver::Version::new(0, 1, 0),
                )
                .unwrap();
                let catalog = content.loot().equipment().clone();
                GameRules {
                    weapons: content.weapons().clone(),
                    items: content.items().clone(),
                    statuses: content.statuses().clone(),
                    monster_equipment_loot: Some(catalog.clone()),
                    equipment_upgrade: Some(EquipmentUpgradeRules {
                        catalog,
                        material: id("unstable_fragment"),
                        costs: [(1, 0); 6],
                    }),
                    player_base_attacks: vec![AttackProfile::melee(DamageType::Kinetic, 5000)],
                    player_weapon_slots: vec![id("close_combat_channel")],
                    player_starting_equipment: vec![None],
                    player_maximum_integrity: 200,
                    ..GameRules::default()
                }
            })
            .clone();
        rules.equipment_color_variants = enabled;
        rules
    }
    fn game(enabled: bool) -> GameState {
        GameState::new_with_rules(
            Map::filled(12, 12, Terrain::Floor).unwrap(),
            GridPos::new(2, 2),
            145,
            rules(enabled),
        )
        .unwrap()
    }

    #[test]
    fn torso_palette_pool_adds_only_approved_colors_without_changing_other_equipment() {
        let ids = [
            "armure_cerbere",
            "brigandine_cognefer",
            "carapace_chrysalide",
            "composite_carapace",
            "combinaison_fantome",
            "manteau_de_mue",
            "plastron_bastion",
            "patched_plating",
            "veste_de_la_seconde_ombre",
            "veste_de_membranes",
            "veste_de_peau_seconde",
            "veste_de_veille",
            "veste_matelassee",
            "veste_a_fibres_croisees",
            "tenue_assaut",
            "tenue_technique",
            "tenue_traque",
            "tenue_rituelle",
            "tenue_metamorphe",
        ];
        for item in ids {
            let colors: BTreeSet<_> = (0..77)
                .map(|roll| cosmetic_palette(&format!("core:{item}"), roll))
                .collect();
            for palette in EquipmentPalette::TORSO_FINISH {
                assert_eq!(
                    colors.contains(&palette),
                    !matches!(item, "veste_de_la_seconde_ombre" | "tenue_rituelle")
                        || palette != EquipmentPalette::DeepRed,
                    "{item}/{palette:?}"
                );
            }
            if item.starts_with("tenue_") {
                for palette in [
                    EquipmentPalette::Original,
                    EquipmentPalette::ReviewedVariant1,
                    EquipmentPalette::ReviewedVariant2,
                ] {
                    assert!(colors.contains(&palette), "legacy {item}/{palette:?}");
                }
            }
        }
        for roll in 0..150 {
            for item in ["fusil_d_assaut", "bottines_fantome", "gantelets_bastion"] {
                assert_eq!(
                    cosmetic_palette(&format!("core:{item}"), roll),
                    EquipmentPalette::ALL[(roll % 7) as usize]
                );
            }
        }
    }

    #[test]
    fn new_torso_palettes_survive_pickup_equip_save_drop_and_restore() {
        for item in ["manteau_de_mue", "tenue_assaut"] {
            for palette in EquipmentPalette::TORSO_FINISH {
                let mut rules = rules(true);
                rules.player_armor_slots = vec![id("body_armor")];
                let mut world = WorldState::single(
                    GameState::new_with_rules(
                        Map::filled(8, 8, Terrain::Floor).unwrap(),
                        GridPos::new(2, 2),
                        145,
                        rules,
                    )
                    .unwrap(),
                );
                let stack = world
                    .spawn_ground_item(GridPos::new(2, 2), id(item), 1)
                    .unwrap();
                world.ground_items.set_palette(stack, palette);
                let stream = world.cosmetic_rng;
                assert_eq!(
                    world.process_player_command(GameCommand::PickUp),
                    CommandOutcome::Applied
                );
                let instance = world
                    .player_inventory()
                    .iter()
                    .find(|e| e.item() == &id(item))
                    .unwrap()
                    .instance();
                assert!(matches!(
                    world.process_player_command(GameCommand::EquipItem {
                        slot: id("body_armor"),
                        item: instance
                    }),
                    CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime
                ));
                assert_eq!(world.player_visible_body_item(), Some(instance));
                world.drain_events();
                let bytes = world.recovery_snapshot_bytes().unwrap();
                let mut restored =
                    WorldState::from_recovery_snapshot_bytes(&bytes, world.rules().clone())
                        .unwrap();
                assert_eq!(
                    restored.player_inventory().get(instance).unwrap().palette(),
                    palette
                );
                assert_eq!(restored.player_visible_body_item(), Some(instance));
                assert_eq!(bytes, restored.recovery_snapshot_bytes().unwrap());
                assert_eq!(
                    restored.process_player_command(GameCommand::DropItem { item: instance }),
                    CommandOutcome::Applied
                );
                assert_eq!(
                    restored
                        .ground_items()
                        .iter()
                        .find(|(_, e)| e.item() == &id(item))
                        .unwrap()
                        .1
                        .palette(),
                    palette
                );
                assert_eq!(restored.cosmetic_rng, stream);
            }
        }
    }

    #[test]
    fn cosmetic_draws_vary_instances_without_consuming_gameplay_randomness() {
        let mut game = game(true);
        let before = game.rng;
        let mut colors = BTreeSet::new();
        for n in 0..100 {
            let ground = game
                .spawn_ground_item(GridPos::new(n % 12, n / 12), id("fusil_d_assaut"), 1)
                .unwrap();
            let item = game.ground_items().get(ground).unwrap();
            assert_eq!(item.item(), &id("fusil_d_assaut"));
            assert!(item.magic_modifiers().is_none());
            assert!(item.color_variant().is_some());
            colors.insert(item.palette());
        }
        assert_eq!(colors, EquipmentPalette::ALL.into_iter().collect());
        assert_eq!(game.rng, before);
        let stream = game.cosmetic_rng;
        game.assign_equipment_palettes();
        assert_eq!(
            game.cosmetic_rng, stream,
            "including Original, every palette is drawn once"
        );
    }

    #[test]
    fn colored_monster_drops_leave_loot_models_bonuses_and_combat_rng_identical() {
        let fixture = |enabled| {
            let mut rules = rules(enabled);
            rules.player_weapon_slots.clear();
            rules.player_starting_equipment.clear();
            GameState::new_with_rules(
                Map::filled(12, 12, Terrain::Floor).unwrap(),
                GridPos::new(2, 2),
                145,
                rules,
            )
            .unwrap()
        };
        let mut colored = fixture(true);
        let mut legacy = fixture(false);
        for _ in 0..180 {
            for game in [&mut colored, &mut legacy] {
                let enemy = game
                    .spawn_actor(
                        Actor::new(GridPos::new(3, 2), 8)
                            .unwrap()
                            .with_player_relation(crate::social::PlayerRelation::Hostile)
                            .with_equipped_weapon(
                                id("modele_veyr_r12"),
                                None,
                                &game.rules().weapons,
                            )
                            .unwrap(),
                    )
                    .unwrap();
                assert_eq!(
                    game.process_player_command(GameCommand::Attack {
                        slot: 0,
                        target: enemy
                    }),
                    CommandOutcome::Applied
                );
                assert!(game.actors().get(enemy).is_none());
                game.drain_events();
            }
            assert_eq!(colored.rng, legacy.rng);
            assert_eq!(
                colored
                    .ground_items()
                    .iter()
                    .map(|(id, e)| (id, e.item().clone(), e.magic_modifiers()))
                    .collect::<Vec<_>>(),
                legacy
                    .ground_items()
                    .iter()
                    .map(|(id, e)| (id, e.item().clone(), e.magic_modifiers()))
                    .collect::<Vec<_>>()
            );
        }
        let colors: BTreeSet<_> = colored
            .ground_items()
            .iter()
            .map(|(_, e)| e.palette())
            .collect();
        let legacy: BTreeSet<_> = EquipmentPalette::ALL
            .into_iter()
            .chain([
                EquipmentPalette::ReviewedVariant1,
                EquipmentPalette::ReviewedVariant2,
            ])
            .collect();
        assert!(legacy.is_subset(&colors));
        let allowed: BTreeSet<_> = legacy
            .into_iter()
            .chain(EquipmentPalette::TORSO_FINISH)
            .collect();
        assert!(colors.is_subset(&allowed));
        // A finite random loot sample need not contain every new color. The
        // deterministic per-model pool test above verifies complete coverage.
    }

    #[test]
    fn pickup_equip_drop_and_improvement_preserve_palette_and_all_bonuses() {
        let mut game = game(true);
        let stack = game
            .spawn_ground_item(GridPos::new(2, 2), id("melee_couteau_de_camp"), 1)
            .unwrap();
        game.ground_items.set_palette(stack, EquipmentPalette::Gray);
        let bonus = MagicItemModifiers::new(2, 3).unwrap();
        game.ground_items
            .retain_magic_modifiers(stack, bonus.clone());
        assert_eq!(
            game.process_player_command(GameCommand::PickUp),
            CommandOutcome::Applied
        );
        let instance = game.player_inventory().iter().next().unwrap().instance();
        let stream = game.cosmetic_rng;
        assert!(matches!(
            game.process_player_command(GameCommand::EquipWeapon {
                slot: 0,
                item: instance
            }),
            CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime
        ));
        assert_eq!(
            game.player_inventory().get(instance).unwrap().palette(),
            EquipmentPalette::Gray
        );
        assert_eq!(
            game.process_player_command(GameCommand::DropItem { item: instance }),
            CommandOutcome::Applied
        );
        let dropped = game.ground_items().iter().next().unwrap().1;
        assert_eq!(dropped.palette(), EquipmentPalette::Gray);
        assert_eq!(dropped.magic_modifiers(), Some(bonus));
        assert_eq!(
            game.process_player_command(GameCommand::PickUp),
            CommandOutcome::Applied
        );
        let instance = game.player_inventory().iter().next().unwrap().instance();
        game.player_inventory_mut()
            .add(id("unstable_fragment"), 1, 100)
            .unwrap();
        game.improve_player_equipment(instance).unwrap();
        assert_eq!(
            game.player_inventory().get(instance).unwrap().palette(),
            EquipmentPalette::Gray
        );
        assert!(
            game.player_inventory()
                .get(instance)
                .unwrap()
                .magic_modifiers()
                .unwrap()
                .improved()
        );
        assert_eq!(game.cosmetic_rng, stream);
    }

    #[test]
    fn snapshot_restores_palettes_and_the_future_cosmetic_sequence() {
        let mut world = WorldState::single(game(true));
        for (n, item) in ["fusil_d_assaut", "armure_cerbere", "bottines_fantome"]
            .into_iter()
            .enumerate()
        {
            world
                .spawn_ground_item(GridPos::new(n as i32, 1), id(item), 1)
                .unwrap();
        }
        world.drain_events();
        let bytes = world.recovery_snapshot_bytes().unwrap();
        assert!(bytes.starts_with(b"RLWS\x0f"));
        let mut restored =
            WorldState::from_recovery_snapshot_bytes(&bytes, world.rules().clone()).unwrap();
        assert_eq!(bytes, restored.recovery_snapshot_bytes().unwrap());
        for n in 0..12 {
            let position = GridPos::new(n, 4);
            let a = world
                .spawn_ground_item(position, id("veste_matelassee"), 1)
                .unwrap();
            let b = restored
                .spawn_ground_item(position, id("veste_matelassee"), 1)
                .unwrap();
            assert_eq!(
                world.ground_items().get(a).unwrap().palette(),
                restored.ground_items().get(b).unwrap().palette()
            );
        }
        let mut old = bytes;
        old[4] = 11;
        assert!(WorldState::from_recovery_snapshot_bytes(&old, world.rules().clone()).is_err());
    }

    #[test]
    fn travel_preserves_carried_and_remote_palettes_and_the_run_stream() {
        use crate::game::{ZoneBlueprint, ZoneInfo};
        let map =
            Map::from_ascii("########\n#......#\n#......#\n#......#\n#......#\n########").unwrap();
        let game = GameState::new_with_rules(map, GridPos::new(2, 2), 145, rules(true)).unwrap();
        let mut world = WorldState::single(game);
        let ground = world
            .spawn_ground_item(GridPos::new(2, 2), id("melee_couteau_de_camp"), 1)
            .unwrap();
        world
            .ground_items
            .set_palette(ground, EquipmentPalette::Original);
        assert_eq!(
            world.process_player_command(GameCommand::PickUp),
            CommandOutcome::Applied
        );
        let carried = world.player_inventory().iter().next().unwrap().instance();
        let remote = world
            .spawn_ground_item(GridPos::new(4, 2), id("armure_cerbere"), 1)
            .unwrap();
        let remote_color = world.ground_items().get(remote).unwrap().palette();
        let info = |name, depth| ZoneInfo {
            id: id(name),
            name: name.into(),
            kind: id("city"),
            depth,
        };
        world.enable(info("palette_a", 0)).unwrap();
        world
            .add_zone(ZoneBlueprint {
                info: info("palette_b", 0),
                map: Map::from_ascii("########\n#......#\n#......#\n#......#\n#......#\n########")
                    .unwrap(),
                entrance: GridPos::new(2, 2),
                seed: 678,
                actors: vec![],
                loot: vec![(GridPos::new(4, 2), id("veste_matelassee"), 1).into()],
                threat_sources: vec![],
            })
            .unwrap();
        world
            .connect(
                id("palette_a"),
                GridPos::new(2, 2),
                id("palette_b"),
                GridPos::new(2, 2),
            )
            .unwrap();
        let stream = world.cosmetic_rng;
        let mut color_b = None;
        for trip in 0..4 {
            assert_eq!(
                world.process_player_command(GameCommand::Interact {
                    target: GridPos::new(2, 2)
                }),
                CommandOutcome::Applied
            );
            let item = world.ground_items().iter().next().unwrap().1;
            if trip % 2 == 0 {
                assert!(item.color_variant().is_some());
                if let Some(color) = color_b {
                    assert_eq!(item.palette(), color);
                } else {
                    color_b = Some(item.palette());
                }
            } else {
                assert_eq!(item.palette(), remote_color);
            }
            let entry = world.player_inventory().get(carried).unwrap();
            assert_eq!(entry.color_variant(), Some(EquipmentPalette::Original));
            assert_eq!(world.cosmetic_rng, stream);
            world.drain_events();
            world = WorldState::from_recovery_snapshot_bytes(
                &world.recovery_snapshot_bytes().unwrap(),
                world.rules().clone(),
            )
            .unwrap();
        }
    }

    #[test]
    fn rejected_pickup_and_failed_placement_are_atomic_for_both_random_streams() {
        let mut rules = rules(true);
        rules.player_inventory_capacity = 0;
        let mut game = GameState::new_with_rules(
            Map::filled(5, 5, Terrain::Floor).unwrap(),
            GridPos::new(2, 2),
            145,
            rules,
        )
        .unwrap();
        game.spawn_ground_item(GridPos::new(2, 2), id("fusil_d_assaut"), 1)
            .unwrap();
        game.drain_events();
        let before = format!("{game:?}");
        assert!(matches!(
            game.process_player_command(GameCommand::PickUp),
            CommandOutcome::Rejected(_)
        ));
        assert_eq!(format!("{game:?}"), before);
        assert!(
            game.spawn_ground_item(GridPos::new(2, 2), id("fusil_d_assaut"), 1)
                .is_err()
        );
        assert_eq!(format!("{game:?}"), before);
    }

    #[test]
    fn historical_json_and_debug_keep_the_original_appearance() {
        let mut inventory = crate::entity::Inventory::new(2);
        let instance = inventory.add(id("fusil_d_assaut"), 1, 1).unwrap()[0];
        let debug = format!("{inventory:?}");
        let mut json = serde_json::to_value(&inventory).unwrap();
        json["entries"][0]
            .as_object_mut()
            .unwrap()
            .remove("palette");
        let restored: crate::entity::Inventory = serde_json::from_value(json).unwrap();
        assert_eq!(
            restored.get(instance).unwrap().palette(),
            EquipmentPalette::Original
        );
        assert_eq!(format!("{restored:?}"), debug);
        let game = game(false);
        assert!(game.cosmetic_rng.is_none());
        assert!(!format!("{game:?}").contains("cosmetic_rng"));
        assert!(!format!("{:?}", game.rules()).contains("equipment_color_variants"));
    }
}
