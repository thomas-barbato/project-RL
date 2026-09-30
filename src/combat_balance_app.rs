//! Reproducible balance probes, not a replacement for progression playtests.
//! Uses real authored enemies, white equipment and the current damage pipeline.
use super::*;
use project_rl::combat::resolve_damage_impact_with_armor;
use project_rl::content::{RegionTerrain, RegionalWorldDefinition};
use project_rl::game::StartingItemStack;
use project_rl::world::{Map, generation::*};

const KNIVES: [&str; 6] = [
    "core:couteau_de_camp",
    "core:couteau_de_sapeur",
    "core:couteau_ceramique",
    "core:couteau_de_chitine",
    "core:couteau_a_dent_vivante",
    "core:couteau_du_dernier_seuil",
];
const RIFLES: [&str; 6] = [
    "core:fusil_de_patrouille",
    "core:fusil_de_guetteur",
    "core:fusil_a_induction",
    "core:fusil_de_parallaxe",
    "core:fusil_a_nerf_tendu",
    "core:fusil_de_l_horizon_fendu",
];
const JACKETS: [&str; 6] = [
    "core:veste_matelassee",
    "core:veste_de_veille",
    "core:veste_a_fibres_croisees",
    "core:veste_de_membranes",
    "core:veste_de_peau_seconde",
    "core:veste_de_la_seconde_ombre",
];
const TARGET_AT: GridPos = GridPos::new(4, 3);
const PLAYER_AT: GridPos = GridPos::new(3, 3);

fn arena() -> Map {
    Map::from_ascii("#########\n#.......#\n#.......#\n#.......#\n#.......#\n#.......#\n#########")
        .unwrap()
}

fn specimen(
    world: &RegionalWorldDefinition,
    biome: &str,
    habitat: RegionTerrain,
    tag: &str,
) -> Actor {
    let biome = world.biome(&biome.parse().unwrap()).unwrap();
    let mut map = arena();
    for y in 1..6 {
        for x in 1..8 {
            // A pack must still have enough cells for its authored minimum of
            // two members. Only its member at TARGET_AT is used in solo probes.
            if GridPos::new(x, y) != TARGET_AT
                && !(tag == "core:friche_biter" && GridPos::new(x, y) == GridPos::new(5, 3))
            {
                map.set_protected(GridPos::new(x, y), true).unwrap();
            }
        }
    }
    for seed in 0..128 {
        let actors = if tag == "core:sentinel_projector" {
            generate_regional_population(
                &map,
                &[],
                biome.population(),
                seed,
                RegionalPopulationFeatures {
                    pursuit_lifecycle: true,
                    primary_attributes: true,
                    physical_profiles: true,
                    electronic_systems: true,
                    player_relations: true,
                },
            )
            .unwrap()
        } else {
            generate_regional_fauna(
                &map,
                &[(TARGET_AT, habitat), (GridPos::new(5, 3), habitat)].into(),
                &[],
                &Default::default(),
                biome.fauna().unwrap(),
                seed,
            )
            .unwrap()
        };
        if let Some(actor) = actors.into_iter().find(|actor| {
            actor.position() == TARGET_AT && actor.tags().iter().any(|id| id.as_str() == tag)
        }) {
            return actor;
        }
    }
    panic!("authored specimen unavailable: {tag}");
}

#[test]
fn combat_balance_surface_roles_remain_distinct_with_starting_white_weapons() {
    use project_rl::social::PlayerRelation;
    let (rules, _, _, _) = ascii_game_content().unwrap();
    let catalog = ascii_regional_world_catalog().unwrap();
    let world = catalog
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap();
    println!("Surface | PV | touches couteau | touches fusil (avant réaction)");
    for (name, tag, neutral) in [
        ("Grignoteur", "core:rubble_nibbler", true),
        ("Mordeur des friches", "core:friche_biter", false),
        ("Fouisseur pâle", "core:vibration_burrower", false),
        ("Brise-os", "core:bone_breaker", false),
        ("Dos-rond", "core:moss_grazer", true),
    ] {
        let base = specimen(world, "core:surface_wilds", RegionTerrain::Scrub, tag);
        let mut hp = Range::default();
        let mut knife_hits = Range::default();
        let mut rifle_hits = Range::default();
        for seed in 0..8 {
            let mut actor = base.clone();
            balance_layer_encounters(
                std::slice::from_mut(&mut actor),
                world.encounter_balance().unwrap(),
                0,
                0,
                seed,
            );
            assert_eq!(actor.player_relation() == PlayerRelation::Neutral, neutral);
            if neutral {
                assert!(actor.defeat_reward().is_none());
            }
            assert_eq!(actor.ai(), base.ai());
            for (weapon, hits) in [(KNIVES[0], &mut knife_hits), (RIFLES[0], &mut rifle_hits)] {
                let mut game = equipped_probe(&rules, weapon, JACKETS[0], seed);
                let id = game.spawn_actor(actor.clone()).unwrap();
                let maximum = game.actors().get(id).unwrap().maximum_integrity();
                let attack = game.equipped_player_weapon(0).unwrap().attack();
                let damage = impact(&game, game.player_id(), id, attack);
                assert!(damage > 0);
                let count = maximum.div_ceil(damage);
                hp.add(maximum);
                hits.add(count);
                // These are role-specific regression budgets for current plain
                // gear, not a universal HP formula or a constraint on good loot.
                let allowed = match tag {
                    "core:rubble_nibbler" | "core:friche_biter" => 1..=3,
                    "core:vibration_burrower" => 2..=5,
                    "core:bone_breaker" => 4..=8,
                    // Neutral shelled fauna is not a routine combat objective.
                    "core:moss_grazer" => 1..=8,
                    _ => unreachable!(),
                };
                assert!(
                    allowed.contains(&count),
                    "{name}, {weapon}: {count} successful hits outside role budget"
                );
                assert_eq!(
                    game.process_player_command(GameCommand::Attack {
                        slot: 0,
                        target: id
                    }),
                    CommandOutcome::Applied
                );
                assert_eq!(
                    maximum - game.actors().get(id).map_or(0, Actor::integrity),
                    damage.min(maximum)
                );
                if tag == "core:moss_grazer" {
                    // Repeated attacks would trigger shell armor: do not report
                    // the first-hit quotient as a real full-fight duration.
                    assert!(
                        game.actor_armor_profile(id).unwrap().effective_against(0)
                            > base.body_profile().unwrap().base_armor
                    );
                    assert!(game.actors().get(id).unwrap().defeat_reward().is_none());
                }
            }
        }
        println!(
            "{name} | {} | {} | {}",
            hp.text(),
            knife_hits.text(),
            rifle_hits.text()
        );
    }
}

#[test]
fn combat_balance_better_weapon_shortens_surface_encounter_without_scaling_enemy() {
    let (rules, _, _, _) = ascii_game_content().unwrap();
    let catalog = ascii_regional_world_catalog().unwrap();
    let world = catalog
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap();
    let mut actor = specimen(
        world,
        "core:surface_wilds",
        RegionTerrain::Scrub,
        "core:bone_breaker",
    );
    balance_layer_encounters(
        std::slice::from_mut(&mut actor),
        world.encounter_balance().unwrap(),
        0,
        0,
        42,
    );
    let mut plain = equipped_probe(&rules, KNIVES[0], JACKETS[0], 42);
    let mut better = equipped_probe(&rules, KNIVES[1], JACKETS[0], 42);
    let id_plain = plain.spawn_actor(actor.clone()).unwrap();
    let id_better = better.spawn_actor(actor).unwrap();
    assert_eq!(plain.actors().get(id_plain), better.actors().get(id_better));
    let hp = plain.actors().get(id_plain).unwrap().maximum_integrity();
    let first = impact(
        &plain,
        plain.player_id(),
        id_plain,
        plain.equipped_player_weapon(0).unwrap().attack(),
    );
    let second = impact(
        &better,
        better.player_id(),
        id_better,
        better.equipped_player_weapon(0).unwrap().attack(),
    );
    assert!(hp.div_ceil(second) < hp.div_ceil(first));
    for (game, id, damage) in [
        (&mut plain, id_plain, first),
        (&mut better, id_better, second),
    ] {
        assert_eq!(
            game.process_player_command(GameCommand::Attack {
                slot: 0,
                target: id
            }),
            CommandOutcome::Applied
        );
        assert_eq!(hp - game.actors().get(id).unwrap().integrity(), damage);
    }
    println!(
        "Brise-os: {} touches avec couteau de camp, {} avec couteau de sapeur, ennemi inchangé",
        hp.div_ceil(first),
        hp.div_ceil(second)
    );
}

fn equipped_probe(rules: &GameRules, weapon: &str, jacket: &str, seed: u64) -> GameState {
    let mut rules = rules.clone();
    // A successful normal impact, not expected damage per action. Accuracy,
    // criticals and targeting are tested separately by the combat engine.
    rules.hit_rules = None;
    rules.player_starting_weapons = vec![weapon.parse().unwrap()];
    rules.player_starting_equipment = vec![Some(weapon.parse().unwrap())];
    rules.player_starting_items = vec![StartingItemStack::new(jacket.parse().unwrap(), 1)];
    let mut game = GameState::new_with_rules(arena(), PLAYER_AT, seed, rules).unwrap();
    let item = game
        .player_inventory()
        .iter()
        .find(|entry| entry.item().as_str() == jacket)
        .unwrap()
        .instance();
    assert_eq!(
        game.process_player_command(GameCommand::EquipItem {
            slot: "core:body_armor".parse().unwrap(),
            item,
        }),
        CommandOutcome::Applied
    );
    game.drain_events();
    game
}

fn impact(
    game: &GameState,
    attacker: EntityId,
    target: EntityId,
    attack: project_rl::combat::AttackProfile,
) -> u16 {
    resolve_damage_impact_with_armor(
        game.resolved_attack_damage(attacker, attack).unwrap(),
        game.actors().get(target).unwrap().resistances(),
        game.actor_armor_profile(target).unwrap(),
        game.rules().damage,
        game.rules().armor_rules.unwrap(),
    )
    .amount()
}

#[derive(Default)]
struct Range(Option<(u16, u16)>);
impl Range {
    fn add(&mut self, value: u16) {
        self.0 = Some(self.0.map_or((value, value), |(low, high)| {
            (low.min(value), high.max(value))
        }));
    }
    fn text(&self) -> String {
        let (low, high) = self.0.unwrap();
        if low == high {
            low.to_string()
        } else {
            format!("{low}–{high}")
        }
    }
}

#[test]
fn combat_balance_white_equipment_matrix_matches_real_impacts() {
    let (rules, _, _, _) = ascii_game_content().unwrap();
    let catalog = ascii_regional_world_catalog().unwrap();
    let world = catalog
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap();
    println!(
        "Creature | couche | palier | PV | coups couteau | coups fusil | degats recus | coups mortels (PV initiaux)"
    );
    for (name, biome, habitat, tag, first, last) in [
        (
            "Ver cuirassé",
            "core:maintenance",
            RegionTerrain::Mud,
            "core:armored_worm",
            1,
            2,
        ),
        (
            "Ver cuirassé (production)",
            "core:production",
            RegionTerrain::Mud,
            "core:armored_worm",
            1,
            3,
        ),
        (
            "Anémone des caves",
            "core:research",
            RegionTerrain::ShallowWater,
            "core:cave_anemone",
            2,
            4,
        ),
        (
            "Hurleur des failles",
            "core:research",
            RegionTerrain::Gravel,
            "core:fault_howler",
            2,
            4,
        ),
        (
            "Sentinelle",
            "core:security",
            RegionTerrain::RuinFloor,
            "core:sentinel_projector",
            3,
            6,
        ),
    ] {
        let base = specimen(world, biome, habitat, tag);
        for depth in first..=last {
            let tier = usize::from(depth.min(5));
            let mut hp = Range::default();
            let mut knife_hits = Range::default();
            let mut rifle_hits = Range::default();
            let mut incoming = Range::default();
            let mut lethal_hits = Range::default();
            for seed in 0..16 {
                let mut actor = base.clone();
                balance_layer_encounters(
                    std::slice::from_mut(&mut actor),
                    world.encounter_balance().unwrap(),
                    depth,
                    first,
                    seed,
                );
                for (weapon, hits) in [
                    (KNIVES[tier], &mut knife_hits),
                    (RIFLES[tier], &mut rifle_hits),
                ] {
                    let mut game = equipped_probe(&rules, weapon, JACKETS[tier], seed);
                    let id = game.spawn_actor(actor.clone()).unwrap();
                    let maximum = game.actors().get(id).unwrap().maximum_integrity();
                    hp.add(maximum);
                    let attack = game.equipped_player_weapon(0).unwrap().attack();
                    let damage = impact(&game, game.player_id(), id, attack);
                    assert!(
                        damage > 0,
                        "{name} depth {depth}: white weapon cannot damage it"
                    );
                    hits.add(maximum.div_ceil(damage));
                    // Broad smoke guard, not a promise of a seven-turn duel:
                    // misses, reloads, recovery and movement are excluded here.
                    assert!(
                        maximum.div_ceil(damage) <= 10,
                        "{name} depth {depth}: excessive successful-hit budget with {weapon}"
                    );
                    let received = impact(
                        &game,
                        id,
                        game.player_id(),
                        game.actors().get(id).unwrap().attack(0).unwrap(),
                    );
                    incoming.add(received);
                    assert!(
                        received
                            < game
                                .actors()
                                .get(game.player_id())
                                .unwrap()
                                .maximum_integrity(),
                        "{name} depth {depth}: ordinary hit kills the starting body outright"
                    );
                    if received > 0 {
                        lethal_hits.add(
                            game.actors()
                                .get(game.player_id())
                                .unwrap()
                                .maximum_integrity()
                                .div_ceil(received),
                        );
                    }
                    // Verify the read-only calculation against an actual attack,
                    // including the equipped weapon, physical impact and armor.
                    assert_eq!(
                        game.process_player_command(GameCommand::Attack {
                            slot: 0,
                            target: id
                        }),
                        CommandOutcome::Applied
                    );
                    let remaining = game.actors().get(id).map_or(0, Actor::integrity);
                    assert_eq!(
                        maximum - remaining,
                        damage.min(maximum),
                        "{name} depth {depth}, {weapon}"
                    );
                }
            }
            println!(
                "{name} | {depth} | {} | {} | {} | {} | {} | {}",
                tier + 1,
                hp.text(),
                knife_hits.text(),
                rifle_hits.text(),
                incoming.text(),
                if lethal_hits.0.is_some() {
                    lethal_hits.text()
                } else {
                    "aucun dégât".into()
                }
            );
        }
    }
}

#[test]
fn combat_balance_white_armor_preserves_grasp_and_electrical_threat() {
    let (rules, _, _, _) = ascii_game_content().unwrap();
    let catalog = ascii_regional_world_catalog().unwrap();
    let world = catalog
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap();
    for machine in [false, true] {
        let (biome, terrain, tag, depth, reference) = if machine {
            (
                "core:security",
                RegionTerrain::RuinFloor,
                "core:sentinel_projector",
                6,
                3,
            )
        } else {
            (
                "core:research",
                RegionTerrain::ShallowWater,
                "core:cave_anemone",
                4,
                2,
            )
        };
        let mut actor = specimen(world, biome, terrain, tag);
        balance_layer_encounters(
            std::slice::from_mut(&mut actor),
            world.encounter_balance().unwrap(),
            depth,
            reference,
            42,
        );
        let tier = usize::from(depth.min(5));
        let mut game = equipped_probe(&rules, KNIVES[tier], JACKETS[tier], 42);
        let id = game.spawn_actor(actor).unwrap();
        let player = game.player_id();
        let hp = game.actors().get(player).unwrap().integrity();
        let expected = impact(
            &game,
            id,
            player,
            game.actors().get(id).unwrap().attack(0).unwrap(),
        );
        if machine {
            assert!(expected >= 10);
        } else {
            assert_eq!(expected, 0);
        }
        let mut released = false;
        for _ in 0..6 {
            assert_eq!(
                game.process_player_command(GameCommand::Wait),
                CommandOutcome::Applied
            );
            if game.drain_events().iter().any(|event| matches!(event, GameEvent::AttackPerformed { attacker, .. } if *attacker == id)) {
                released = true;
                break;
            }
        }
        assert!(released, "the authored attack must actually be released");
        assert_eq!(
            hp - game.actors().get(player).unwrap().integrity(),
            expected
        );
        let hindered = game
            .actors()
            .get(player)
            .unwrap()
            .status(&"core:locomotion_hindered".parse().unwrap())
            .is_some();
        assert_eq!(
            hindered, !machine,
            "absorbing damage must not erase a successful grasp"
        );
    }
}

#[test]
fn combat_balance_scaled_telegraphs_allow_escape_with_white_equipment() {
    let (rules, _, _, _) = ascii_game_content().unwrap();
    let catalog = ascii_regional_world_catalog().unwrap();
    let world = catalog
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap();
    for (biome, terrain, tag, depth, reference, escape) in [
        (
            "core:production",
            RegionTerrain::Mud,
            "core:armored_worm",
            3,
            1,
            vec![Direction::North],
        ),
        (
            "core:research",
            RegionTerrain::ShallowWater,
            "core:cave_anemone",
            4,
            2,
            vec![Direction::West],
        ),
        (
            "core:research",
            RegionTerrain::Gravel,
            "core:fault_howler",
            4,
            2,
            vec![Direction::West, Direction::West, Direction::North],
        ),
        (
            "core:security",
            RegionTerrain::RuinFloor,
            "core:sentinel_projector",
            6,
            3,
            vec![Direction::North, Direction::North],
        ),
    ] {
        let base = specimen(world, biome, terrain, tag);
        for seed in 0..4 {
            let mut actor = base.clone();
            balance_layer_encounters(
                std::slice::from_mut(&mut actor),
                world.encounter_balance().unwrap(),
                depth,
                reference,
                seed,
            );
            let tier = usize::from(depth.min(5));
            let mut game = equipped_probe(&rules, KNIVES[tier], JACKETS[tier], seed);
            let id = game.spawn_actor(actor).unwrap();
            let hp = game.actors().get(game.player_id()).unwrap().integrity();
            assert_eq!(
                game.process_player_command(GameCommand::Wait),
                CommandOutcome::Applied
            );
            let cells = crate::terminal_view::visible_telegraphed_attack_cells(&game, id);
            assert!(
                cells.iter().any(|cell| cell.position == PLAYER_AT),
                "{tag}: actual danger must be previewed"
            );
            game.drain_events();
            for direction in &escape {
                assert_eq!(
                    game.process_player_command(GameCommand::Move(*direction)),
                    CommandOutcome::Applied
                );
            }
            assert_eq!(
                game.actors().get(game.player_id()).unwrap().integrity(),
                hp,
                "{tag}: advertised escape no longer works"
            );
            assert!(
                !cells
                    .iter()
                    .any(|cell| Some(cell.position) == game.player_position())
            );
            assert!(
                game.actors()
                    .get(id)
                    .unwrap()
                    .recovery_remaining()
                    .is_some(),
                "{tag}: verify the attack finished, not an idle enemy"
            );
            assert!(game.drain_events().iter().any(|event| matches!(event, GameEvent::AttackPerformed {attacker, ..} if *attacker == id)));
        }
    }
}
