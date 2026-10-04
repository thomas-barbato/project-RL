//! Regional caches and explicitly authored humanoid weapon carriers.
use super::*;
use project_rl::game::{GameRng, GroundLootBlueprint, ZoneBlueprint};
use project_rl::loot::{EquipmentQuality, EquipmentSource};

pub(super) const BASE_IDS: [&str; 6] = [
    "core:couteau_de_camp",
    "core:couteau_de_sapeur",
    "core:fusil_de_patrouille",
    "core:fusil_de_guetteur",
    "core:veste_matelassee",
    "core:veste_de_veille",
];
pub(super) const EFFECT_IDS: [&str; 2] = ["core:affix_braise", "core:affix_decharge"];
pub(super) const TACTICAL_BASE_IDS: [&str; 2] = ["core:lance", "core:fusil_a_pompe"];
pub(super) const ARSENAL_BASE_IDS: [&str; 6] = [
    "core:fusil_d_assaut",
    "core:mitrailleuse_lourde",
    "core:lance_roquettes",
    "core:lance_grenades",
    "core:hache_de_combat",
    "core:marteau_de_guerre",
];
pub(super) const ARSENAL_EFFECT_IDS: [&str; 2] = ["core:affix_saignement", "core:affix_poison"];
pub(super) const ARSENAL_STATUS_IDS: [&str; 2] = ["core:bleeding", "core:poisoned"];
pub(super) const DEEP_BASE_IDS: [&str; 12] = [
    "core:couteau_ceramique",
    "core:couteau_de_chitine",
    "core:couteau_a_dent_vivante",
    "core:couteau_du_dernier_seuil",
    "core:fusil_a_induction",
    "core:fusil_de_parallaxe",
    "core:fusil_a_nerf_tendu",
    "core:fusil_de_l_horizon_fendu",
    "core:veste_a_fibres_croisees",
    "core:veste_de_membranes",
    "core:veste_de_peau_seconde",
    "core:veste_de_la_seconde_ombre",
];
const EQUIPMENT_SEED_SALT: u64 = 0x4551_5549_504c_4f54;

#[path = "firearm_models.rs"]
mod firearm_models;
pub(super) use firearm_models::{LEGACY_FIREARM_IDS, firearm_model_ids};

#[path = "melee_models.rs"]
mod melee_models;
pub(super) use melee_models::{LEGACY_MELEE_IDS, melee_model_ids, sword_model_ids};

#[path = "armor_models.rs"]
mod armor_models;
pub(super) use armor_models::armor_model_ids;

#[path = "accessory_models.rs"]
mod accessory_models;
pub(super) use accessory_models::{ACCESSORY_SLOTS, accessory_model_ids};

#[path = "enemy_equipment.rs"]
mod enemy_equipment;

impl AsciiApp {
    #[cfg(debug_assertions)]
    pub(super) fn prepare_rarity_equipment_diagnostic(&mut self) -> Result<(), String> {
        self.open_menu(MenuScreen::Main);
        self.enter_test_lab()?;
        let id: ItemId = "core:visiere_fantome".parse().unwrap();
        let mut examples = std::collections::BTreeMap::new();
        let mut rng = GameRng::from_seed(135);
        for _ in 0..4096 {
            let bonus = self.loot.equipment().enchant_base(&id, 1, &mut rng)?;
            examples.entry(bonus.bonus_count()).or_insert(bonus);
            if examples.len() == 6 {
                break;
            }
        }
        if examples.len() != 6 {
            return Err("Raretés de diagnostic incomplètes".into());
        }
        let mut rules = self.game.rules().clone();
        rules.player_starting_items = vec![project_rl::game::StartingItemStack::new(id.clone(), 1)];
        let game = GameState::new_with_rules(
            self.game.map().clone(),
            self.game.player_position().ok_or("Position absente")?,
            135,
            rules,
        )
        .map_err(|e| e.to_string())?
        .with_starting_magic_equipment(examples.into_values().map(|bonus| (id.clone(), bonus)))?;
        let item = game
            .player_inventory()
            .iter()
            .find(|entry| {
                entry
                    .magic_modifiers()
                    .is_some_and(|b| b.bonus_count() == 6)
            })
            .ok_or("Exemplaire absent")?
            .instance();
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.selected_target = None;
        self.inventory_filter = InventoryFilter::Armor;
        self.select_inventory_instance(item);
        self.inventory_open = true;
        Ok(())
    }

    pub(super) fn populate_regional_equipment(
        &self,
        blueprint: &mut ZoneBlueprint,
        biome: &ContentId,
        seed: u64,
    ) -> Result<(), String> {
        // Replace only the humanoid equipment already selected by cache tables.
        // A biome is not a creature's provenance: actors and machine parts are
        // never passed to this adapter, even in a machine-occupied region.
        let deep_cache = self.generation_version >= DEPTH_EQUIPMENT_GENERATION_VERSION
            && blueprint.info.depth > 0
            && matches!(
                biome.as_str(),
                "core:maintenance"
                    | "core:production"
                    | "core:research"
                    | "core:security"
                    | "core:network"
                    | "core:corrupted"
            );
        if biome.as_str() != "core:human_habitat" && !deep_cache {
            return Ok(());
        }
        let mut rng = GameRng::from_seed(seed ^ EQUIPMENT_SEED_SALT);
        for loot in &mut blueprint.loot {
            if loot.quantity() != 1
                || !matches!(
                    loot.item().as_str(),
                    "core:integrity_blade"
                        | "core:needle_launcher"
                        | "core:patched_plating"
                        | "core:composite_carapace"
                )
            {
                continue;
            }
            let Some(rolled) = self.loot.equipment().draw(
                EquipmentSource::HumanoidSite,
                blueprint.info.depth,
                EquipmentQuality::Random {
                    enchanted_percent: if self.generation_version >= BONUS_RARITY_GENERATION_VERSION
                    {
                        project_rl::loot::ENCHANTED_EQUIPMENT_PERCENT
                    } else {
                        35
                    },
                },
                &mut rng,
            )?
            else {
                continue;
            };
            let mut replacement = GroundLootBlueprint::new(loot.position(), rolled.item, 1);
            if let Some(owner) = loot.owner() {
                replacement = replacement.with_owner(owner.clone());
            }
            if let Some(modifiers) = rolled.modifiers {
                replacement = replacement.with_magic_modifiers(modifiers)?;
            }
            *loot = replacement;
        }
        Ok(())
    }

    #[cfg(debug_assertions)]
    pub(super) fn prepare_generated_equipment_diagnostic(
        &mut self,
        deep: bool,
    ) -> Result<(), String> {
        self.prepare_generated_equipment_model_diagnostic(deep, None)
    }

    #[cfg(debug_assertions)]
    pub(super) fn prepare_generated_equipment_model_diagnostic(
        &mut self,
        deep: bool,
        model: Option<&str>,
    ) -> Result<(), String> {
        self.open_menu(MenuScreen::Main);
        self.enter_test_lab()?;
        // Display a real base rolled by the common generator, not a lab model.
        let mut rng = GameRng::from_seed(0x4551_5549_5044_454d);
        let rolled = (0..4096)
            .find_map(|_| {
                let rolled = self
                    .loot
                    .equipment()
                    .draw(
                        EquipmentSource::HumanoidSite,
                        if deep { 5 } else { 1 },
                        EquipmentQuality::Enchanted,
                        &mut rng,
                    )
                    .ok()
                    .flatten()?;
                ((if let Some(model) = model {
                    rolled.item.as_str() == model
                } else {
                    !deep || (rolled.tier == 6 && rolled.item.as_str() == "core:modele_noryk_r72")
                }) && rolled.modifiers.as_ref().is_some_and(|m| {
                    m.named_affixes().is_some()
                        && (self.rules.items.get(&rolled.item).is_some()
                            || m.effect_affix().is_some())
                }))
                .then_some(rolled)
            })
            .ok_or("Aucun exemplaire de diagnostic")?;
        let game = GameState::new_with_rules(
            self.game.map().clone(),
            self.game.player_position().ok_or("Position absente")?,
            71,
            self.game.rules().clone(),
        )
        .map_err(|e| e.to_string())?
        .with_starting_magic_equipment([(rolled.item.clone(), rolled.modifiers.unwrap())])?;
        let item = game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &rolled.item)
            .ok_or("Exemplaire absent")?
            .instance();
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.selected_target = None;
        self.inventory_filter = InventoryFilter::default();
        self.inventory_selection = self
            .inventory_entries()
            .iter()
            .position(|entry| entry.instance() == item)
            .ok_or("Exemplaire filtré")?;
        self.inventory_open = true;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use project_rl::game::ZoneInfo;
    use project_rl::world::{Map, Terrain};
    #[test]
    fn improvement_preserves_version_135() {
        let old = app(135);
        let saved = old.suspension().unwrap();
        assert_eq!(saved.rules, 13250525110189897679);
        assert_eq!(saved.loot_rules, Some(14124179835268072052));
        assert_eq!(saved.state, 13575184762297074475);
    }

    #[test]
    fn weapon_families_preserve_version_127() {
        let app = app(127);
        assert_eq!(
            rules_fingerprint_for_version(&app.rules, 127),
            4887016499305801953
        );
        assert_eq!(suspension::fingerprint(&app.loot), 17749085145699605428);
        for id in TACTICAL_BASE_IDS.map(|id| id.parse().unwrap()) {
            assert!(app.rules.weapons.get(&id).is_none());
            assert!(app.loot.equipment().iter().all(|(item, _)| item != &id));
        }
    }

    fn app(version: u8) -> AsciiApp {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, version).unwrap()
    }
    #[test]
    fn bonus_rarity_preserves_version_134() {
        let old = app(134);
        let saved = old.suspension().unwrap();
        assert_eq!(saved.rules, 4694782463596524963);
        assert_eq!(saved.loot_rules, Some(13434178925032854592));
        assert_eq!(saved.state, 11827607781388651990);
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(
            suspension::fingerprint(&restored.game),
            suspension::fingerprint(&old.game)
        );
    }
    #[test]
    fn firearm_models_preserve_version_129() {
        let old = app(ARSENAL_GENERATION_VERSION);
        let saved = old.suspension().unwrap();
        assert_eq!(saved.rules, 10666186962048973248);
        assert_eq!(saved.loot_rules, Some(6175408984068260612));
        assert_eq!(saved.state, 5015229487898651663);
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn melee_models_preserve_version_130() {
        let old = app(FIREARM_MODELS_GENERATION_VERSION);
        let saved = old.suspension().unwrap();
        assert_eq!(saved.rules, 7889956807676227788);
        assert_eq!(saved.loot_rules, Some(6713396172964035548));
        assert_eq!(saved.state, 4364533401394836414);
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn sword_models_preserve_version_131() {
        let old = app(MELEE_MODELS_GENERATION_VERSION);
        let saved = old.suspension().unwrap();
        assert_eq!(saved.rules, 1185258536419116580);
        assert_eq!(saved.loot_rules, Some(7726250592950584704));
        assert_eq!(saved.state, 2731019285769788187);
        for id in sword_model_ids() {
            assert!(old.rules.weapons.get(&id).is_none());
            assert!(old.loot.equipment().iter().all(|(item, _)| item != &id));
        }
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn reinforced_armor_preserves_version_132() {
        let old = app(SWORD_MODELS_GENERATION_VERSION);
        let saved = old.suspension().unwrap();
        assert_eq!(saved.rules, 15135589561427297114);
        assert_eq!(saved.loot_rules, Some(799324603335339450));
        assert_eq!(saved.state, 7514443169048286002);
        for id in armor_model_ids() {
            assert!(old.rules.items.get(&id).is_none());
            assert!(old.loot.equipment().iter().all(|(item, _)| item != &id));
        }
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn accessory_models_preserve_version_133() {
        let old = app(REINFORCED_ARMOR_GENERATION_VERSION);
        let saved = old.suspension().unwrap();
        assert_eq!(saved.rules, 4249746003871922540);
        assert_eq!(saved.loot_rules, Some(13451380360041429538));
        assert_eq!(saved.state, 8547453497967341830);
        assert_eq!(old.rules.player_armor_slots.len(), 1);
        for id in accessory_model_ids() {
            assert!(old.rules.items.get(&id).is_none());
            assert!(old.loot.equipment().iter().all(|(item, _)| item != &id));
        }
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    fn blueprint() -> ZoneBlueprint {
        ZoneBlueprint {
            info: ZoneInfo {
                id: "core:equipment_probe".parse().unwrap(),
                name: "Dépôt".into(),
                kind: "core:human_habitat".parse().unwrap(),
                depth: 0,
            },
            map: Map::filled(12, 8, Terrain::Floor).unwrap(),
            entrance: GridPos::new(1, 1),
            seed: 91,
            actors: vec![],
            threat_sources: vec![],
            loot: (0..8)
                .map(|index| {
                    GroundLootBlueprint::new(
                        GridPos::new(index + 2, 2),
                        "core:needle_launcher".parse().unwrap(),
                        1,
                    )
                    .with_owner("core:test_owners".parse().unwrap())
                })
                .collect(),
        }
    }

    #[test]
    fn human_site_rolls_preserve_placement_count_owner_and_all_non_equipment_loot() {
        let app = app(CURRENT_GENERATION_VERSION);
        let mut before = blueprint();
        before.loot.push(GroundLootBlueprint::new(
            GridPos::new(2, 3),
            "core:repair_patch".parse().unwrap(),
            2,
        ));
        let mut after = before.clone();
        app.populate_regional_equipment(&mut after, &"core:human_habitat".parse().unwrap(), 91)
            .unwrap();
        assert_eq!(after.loot.len(), before.loot.len());
        for (old, new) in before.loot.iter().zip(&after.loot) {
            assert_eq!(
                (old.position(), old.quantity(), old.owner()),
                (new.position(), new.quantity(), new.owner())
            );
        }
        assert_eq!(after.loot.last(), before.loot.last());
        assert!(
            after.loot[..8]
                .iter()
                .all(|item| BASE_IDS.contains(&item.item().as_str())
                    || DEEP_BASE_IDS.contains(&item.item().as_str())
                    || firearm_model_ids().contains(item.item())
                    || melee_model_ids().contains(item.item())
                    || sword_model_ids().contains(item.item())
                    || armor_model_ids().contains(item.item())
                    || accessory_model_ids().contains(item.item())
                    || ARSENAL_BASE_IDS.contains(&item.item().as_str())
                    || TACTICAL_BASE_IDS.contains(&item.item().as_str()))
        );
        let mut again = before.clone();
        app.populate_regional_equipment(&mut again, &"core:human_habitat".parse().unwrap(), 91)
            .unwrap();
        assert_eq!(after.loot, again.loot);
        app.populate_regional_equipment(&mut before, &"core:surface_wilds".parse().unwrap(), 91)
            .unwrap();
        assert_eq!(before.loot[0].item().as_str(), "core:needle_launcher");
    }

    #[test]
    fn underground_caches_generate_equipment_on_every_playable_layer_but_not_in_cities() {
        use crate::test_regional::{self, RegionalGenerationFeatures};
        use project_rl::world::Direction;
        let app = app(CURRENT_GENERATION_VERSION);
        let previous = app.regional_worlds.without_deep_equipment_cache_metadata();
        let world_id = "core:simulation_overworld".parse().unwrap();
        let world = app.regional_worlds.get(&world_id).unwrap();
        let previous_world = previous.get(&world_id).unwrap();
        let features = RegionalGenerationFeatures {
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
        };
        for id in ["core:maintenance", "core:production"] {
            let id = id.parse().unwrap();
            assert_eq!(world.biome(&id), previous_world.biome(&id));
        }
        for id in [
            "core:research",
            "core:security",
            "core:network",
            "core:corrupted",
        ] {
            let id = id.parse().unwrap();
            assert!(world.biome(&id).unwrap().loot().is_some());
            assert!(previous_world.biome(&id).unwrap().loot().is_none());
        }
        let mut seen = std::collections::BTreeSet::new();
        for depth in 1..=5 {
            let coordinate = RegionCoord::new(1, 1, depth);
            let mut layer_equipment = 0;
            for seed in 0..12 {
                let descriptor = world.region(seed, coordinate).unwrap();
                seen.insert(descriptor.biome.clone());
                let mut generated = test_regional::generate(
                    world,
                    &descriptor,
                    test_regional::zone_info(world, &descriptor).unwrap(),
                    project_rl::world::generation::cardinal_passage(
                        world.map_size_at(coordinate),
                        Direction::East,
                    ),
                    Some(&app.loot),
                    features,
                )
                .unwrap();
                let before = generated.blueprint.clone();
                app.populate_regional_equipment(
                    &mut generated.blueprint,
                    &descriptor.biome,
                    descriptor.seed,
                )
                .unwrap();
                assert_eq!(generated.blueprint.actors, before.actors);
                assert_eq!(generated.blueprint.loot.len(), before.loot.len());
                for (old, new) in before.loot.iter().zip(&generated.blueprint.loot) {
                    assert_eq!(
                        (old.position(), old.quantity(), old.owner()),
                        (new.position(), new.quantity(), new.owner())
                    );
                    assert!(generated.blueprint.map.is_walkable(new.position()));
                    if app.loot.equipment().iter().any(|(id, _)| id == new.item()) {
                        layer_equipment += 1;
                    } else {
                        assert_eq!(old, new);
                    }
                }
            }
            assert!(
                layer_equipment > 0,
                "no generated equipment at depth {depth}"
            );
        }
        assert_eq!(
            seen.len(),
            6,
            "all underground biomes must be covered: {seen:?}"
        );
        let coordinate = RegionCoord::new(-1, 0, 5);
        let descriptor = world.region(17, coordinate).unwrap();
        let mut city = test_regional::generate(
            world,
            &descriptor,
            test_regional::zone_info(world, &descriptor).unwrap(),
            project_rl::world::generation::cardinal_passage(
                world.map_size_at(coordinate),
                Direction::East,
            ),
            Some(&app.loot),
            features,
        )
        .unwrap();
        app.populate_regional_equipment(&mut city.blueprint, &descriptor.biome, descriptor.seed)
            .unwrap();
        assert!(city.blueprint.loot.is_empty());
    }

    #[test]
    fn version_107_keeps_its_original_equipment_and_can_resume() {
        let old = app(EQUIPMENT_COMMERCE_GENERATION_VERSION);
        let saved = old.suspension().unwrap();
        assert_eq!(saved.rules, 6198306336801966463);
        assert_eq!(saved.loot_rules, Some(17749085145699605428));
        assert_eq!(saved.world_rules, Some(4147835719884813333));
        assert_eq!(saved.state, 7122100530405190243);
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_106_keeps_its_original_merchants_and_can_resume() {
        let old = app(DEPTH_EQUIPMENT_GENERATION_VERSION);
        let saved = old.suspension().unwrap();
        assert_eq!(saved.rules, 6198306336801966463);
        assert_eq!(saved.loot_rules, Some(17749085145699605428));
        assert_eq!(saved.world_rules, Some(4147835719884813333));
        assert_eq!(saved.state, 14919163715656792865);
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
    }

    #[test]
    fn version_105_keeps_its_exact_catalogues_draws_and_resume() {
        let previous = app(INSTANCE_LOOT_GENERATION_VERSION);
        let saved = previous.suspension().unwrap();
        // Captured before adding P3..P6 or underground cache metadata.
        assert_eq!(saved.rules, 3856290570910968433);
        assert_eq!(saved.loot_rules, Some(11835931522359807866));
        assert_eq!(saved.world_rules, Some(1879847749073525773));
        assert_eq!(previous.loot.equipment().iter().count(), 6);
        for id in DEEP_BASE_IDS {
            assert!(previous.rules.weapons.get(&id.parse().unwrap()).is_none());
            assert!(previous.rules.items.get(&id.parse().unwrap()).is_none());
        }
        let mut zone = blueprint();
        previous
            .populate_regional_equipment(&mut zone, &"core:human_habitat".parse().unwrap(), 91)
            .unwrap();
        assert_eq!(suspension::fingerprint(&zone.loot), 12501464613726254942);
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), saved.state);
        zone.info.depth = 5;
        let before = zone.loot.clone();
        previous
            .populate_regional_equipment(&mut zone, &"core:network".parse().unwrap(), 91)
            .unwrap();
        assert_eq!(zone.loot, before);
    }

    #[test]
    fn version_104_retains_its_original_catalogues_and_can_resume() {
        let old = app(NONSTACKING_DOT_GENERATION_VERSION);
        assert!(old.loot.equipment().is_empty());
        for id in BASE_IDS.into_iter().chain(DEEP_BASE_IDS) {
            assert!(old.rules.weapons.get(&id.parse().unwrap()).is_none());
            assert!(old.rules.items.get(&id.parse().unwrap()).is_none());
        }
        for id in EFFECT_IDS {
            assert!(
                old.rules
                    .weapons
                    .effect_affix(&id.parse().unwrap())
                    .is_none()
            );
        }
        let saved = old.suspension().unwrap();
        let expected = suspension::fingerprint(&old.game);
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let restored =
            AsciiApp::restore_suspension(&saved, rules, texts, loot, expeditions).unwrap();
        assert_eq!(suspension::fingerprint(&restored.game), expected);
        let mut zone = blueprint();
        let before = zone.loot.clone();
        old.populate_regional_equipment(&mut zone, &"core:human_habitat".parse().unwrap(), 91)
            .unwrap();
        assert_eq!(zone.loot, before);
    }
}
