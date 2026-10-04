//! Finite, single-life round trips. No retry, free healing or ideal loot grant.
//! This is a controlled surface course, not an estimate of campaign win rate.
use super::*;
use project_rl::game::{GameRng, GroundLootBlueprint, StartingItemStack, ZoneBlueprint, ZoneInfo};
use project_rl::loot::{EquipmentLootCatalog, EquipmentQuality, EquipmentSource};
use project_rl::world::{Map, find_path_with, has_line_of_sight};
use std::collections::BTreeSet;

const RIFLE: &str = "core:fusil_de_patrouille";
const PATCH: &str = "core:repair_patch";
const ENTRY: GridPos = GridPos::new(1, 4);
const EXIT: GridPos = GridPos::new(11, 4);
const CACHE: GridPos = GridPos::new(9, 4);

fn map() -> Map {
    Map::from_ascii("#############\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#...........#\n#############").unwrap()
}

fn info(index: u8) -> ZoneInfo {
    ZoneInfo {
        id: format!("test:survival_{index}").parse().unwrap(),
        name: format!("Survival probe {index}"),
        kind: "core:surface_wilds".parse().unwrap(),
        depth: 0,
    }
}

pub(super) fn finite_rules(rules: &GameRules) -> GameRules {
    let mut rules = rules.clone();
    // Keep the historical magazine-only balance sample separate from the new
    // shared supplies fixtures.
    rules.weapon_matter_item = None;
    rules.player_energy_regeneration = 0;
    // Explicit finite fixture kit, not the generous all-systems diagnostic kit.
    // Keep the actual hit rolls, armor, body, recovery and item effects.
    rules.player_starting_weapons = vec![
        "core:couteau_de_camp".parse().unwrap(),
        RIFLE.parse().unwrap(),
    ];
    rules.player_starting_equipment = rules
        .player_starting_weapons
        .iter()
        .cloned()
        .map(Some)
        .collect();
    rules.player_base_attacks = rules
        .player_starting_weapons
        .iter()
        .map(|w| rules.weapons.get(w).unwrap().attack())
        .collect();
    rules.player_starting_items = vec![StartingItemStack::new(PATCH.parse().unwrap(), 2)];
    rules
}

fn expedition(
    rules: &GameRules,
    loot: &LootCatalog,
    catalog: &RegionalWorldCatalog,
    seed: u64,
) -> WorldState {
    let rules = finite_rules(rules);
    let game = GameState::new_with_rules(map(), ENTRY, seed, rules.clone()).unwrap();
    let mut world = WorldState::single(game);
    world.enable(info(0)).unwrap();
    // A human-site cache, never human armor dropped by a bat. Restrict only the
    // family for this armor-equip probe, retaining depth weights and affixes.
    let mut pool = EquipmentLootCatalog::default();
    for (_, base) in loot
        .equipment()
        .iter()
        .filter(|(_, b)| b.family.as_str() == "core:protective_jackets")
    {
        pool.register(base.clone(), &rules.items, &rules.weapons)
            .unwrap();
    }
    let mut rng = GameRng::from_seed(seed ^ 0x5355_5256_4956_414c);
    let rolled = pool
        .draw(
            EquipmentSource::HumanoidSite,
            0,
            EquipmentQuality::Random {
                enchanted_percent: 35,
            },
            &mut rng,
        )
        .unwrap()
        .unwrap();
    let mut cache = GroundLootBlueprint::new(CACHE, rolled.item, 1);
    if let Some(modifiers) = rolled.modifiers {
        cache = cache.with_magic_modifiers(modifiers).unwrap();
    }
    for index in 1..=2 {
        let (specimens, ids, _) =
            mixed_encounters::encounter(&rules, catalog, "mixed-surface", seed + u64::from(index))
                .unwrap();
        let actors = ids
            .into_iter()
            .map(|id| specimens.actors().get(id).unwrap().clone())
            .collect();
        world
            .add_zone(ZoneBlueprint {
                info: info(index),
                map: map(),
                entrance: ENTRY,
                seed: seed + u64::from(index),
                actors,
                loot: if index == 1 {
                    vec![cache.clone()]
                } else {
                    vec![]
                },
                threat_sources: vec![],
            })
            .unwrap();
        world
            .connect(info(index - 1).id, EXIT, info(index).id, ENTRY)
            .unwrap();
    }
    world.drain_events();
    world
}

pub(super) fn hp(world: &WorldState) -> u16 {
    world
        .actors()
        .get(world.player_id())
        .map_or(0, Actor::integrity)
}
pub(super) fn patches(world: &WorldState) -> u16 {
    world
        .player_inventory()
        .iter()
        .filter(|e| e.item().as_str() == PATCH)
        .map(|e| e.quantity())
        .sum()
}
pub(super) fn ammo(world: &WorldState) -> u16 {
    world
        .player_weapon_ammunition(&RIFLE.parse().unwrap())
        .unwrap()
        .0
}
fn zone(world: &WorldState) -> u8 {
    (0..=2)
        .find(|&i| world.current_zone().unwrap().id == info(i).id)
        .unwrap()
}
fn distance(a: GridPos, b: GridPos) -> u32 {
    a.x.abs_diff(b.x).max(a.y.abs_diff(b.y))
}
pub(super) fn danger(world: &WorldState) -> BTreeSet<GridPos> {
    world
        .actors()
        .iter()
        .flat_map(|(id, _)| {
            crate::terminal_view::visible_telegraphed_attack_cells(world, id)
                .into_iter()
                .map(|c| c.position)
        })
        .collect()
}
fn step(world: &WorldState, goal: GridPos, avoid: &BTreeSet<GridPos>) -> Option<GameCommand> {
    let from = world.player_position()?;
    let path = find_path_with(
        world.map(),
        from,
        goal,
        512,
        |map, at| map.is_walkable(at),
        |at| world.actors().entity_at(at).is_none() && !avoid.contains(&at),
    )?;
    let next = *path.get(1)?;
    if avoid.contains(&next) || world.actors().entity_at(next).is_some() {
        return None;
    }
    Some(GameCommand::Move(Direction::from_delta(
        next.x - from.x,
        next.y - from.y,
    )?))
}

#[derive(Debug, PartialEq, Eq)]
struct Report {
    dead: bool,
    returned: bool,
    reached_goal: bool,
    acquired: bool,
    equipped: bool,
    turns: u64,
    hp: u16,
    ammunition: u16,
    patches: u16,
    transitions: u16,
}

fn run(mut world: WorldState, cautious: bool) -> Report {
    let empty_danger = BTreeSet::new();
    let mut returning = false;
    let mut reached_goal = false;
    let mut acquired = false;
    let mut equipped = false;
    let mut departed = false;
    let mut transitions = 0;
    let mut stalled_actions = 0;
    let mut last_progress = None;
    for _ in 0..400 {
        if world.status() != RunStatus::Active {
            break;
        }
        let current = zone(&world);
        let progress = (current, world.actors().iter().count(), acquired);
        if last_progress == Some(progress) {
            stalled_actions += 1;
        } else {
            stalled_actions = 0;
            last_progress = Some(progress);
        }
        // The bounded test pilot abandons an unproductive fight; do not keep
        // dodging indefinitely, resurrect it or silently select another seed.
        if cautious && stalled_actions >= 40 {
            returning = true;
        }
        if departed && current == 0 {
            break;
        }
        let at = world.player_position().unwrap();
        if current == 2 && at == CACHE {
            reached_goal = true;
            returning = true;
        }
        let threats = danger(&world);
        let visible: Vec<_> = world
            .actors()
            .iter()
            .filter(|(id, a)| {
                *id != world.player_id()
                    && a.player_relation() == project_rl::social::PlayerRelation::Hostile
                    && world.player_visibility().is_visible(a.position())
            })
            .map(|(id, a)| (id, a.position()))
            .collect();
        if cautious && hp(&world) <= 8 && patches(&world) == 0 {
            returning = true;
        }
        let goal = if returning {
            ENTRY
        } else if current == 2 || (current == 1 && !acquired) {
            CACHE
        } else {
            EXIT
        };
        let escape = if cautious && threats.contains(&at) {
            [
                Direction::West,
                Direction::North,
                Direction::South,
                Direction::East,
            ]
            .into_iter()
            .filter(|&d| {
                let p = at.step(d);
                world.map().is_walkable(p)
                    && world.actors().entity_at(p).is_none()
                    && !threats.contains(&p)
            })
            .max_by_key(|&d| {
                visible
                    .iter()
                    .map(|(_, p)| distance(at.step(d), *p))
                    .min()
                    .unwrap_or(0)
            })
            .map(GameCommand::Move)
        } else {
            None
        };
        let heal = (cautious && hp(&world) <= 12)
            .then(|| {
                world
                    .player_inventory()
                    .iter()
                    .find(|e| e.item().as_str() == PATCH)
                    .map(|e| GameCommand::UseItem { item: e.instance() })
            })
            .flatten();
        let equip = if acquired && !equipped {
            world
                .player_inventory()
                .iter()
                .find(|e| {
                    e.item().as_str() != PATCH
                        && e.item().as_str() != RIFLE
                        && e.item().as_str() != "core:couteau_de_camp"
                })
                .map(|e| GameCommand::EquipItem {
                    slot: "core:body_armor".parse().unwrap(),
                    item: e.instance(),
                })
        } else {
            None
        };
        let attack = visible
            .iter()
            .filter(|(_, p)| has_line_of_sight(world.map(), at, *p, true))
            .min_by_key(|(_, p)| distance(at, *p))
            .and_then(|&(id, p)| {
                if distance(at, p) <= 1 {
                    Some(GameCommand::Attack {
                        slot: 0,
                        target: id,
                    })
                } else if ammo(&world) > 0 && (at.x - p.x).pow(2) + (at.y - p.y).pow(2) <= 49 {
                    Some(GameCommand::Attack {
                        slot: 1,
                        target: id,
                    })
                } else {
                    None
                }
            });
        let interact = ((returning || current == 0 || (current == 1 && acquired))
            && distance(at, goal) <= 1)
            .then_some(GameCommand::Interact { target: goal });
        let pickup = (!acquired && current == 1 && at == CACHE).then_some(GameCommand::PickUp);
        let command = escape
            .or(heal)
            .or(equip)
            .or(interact)
            .or(pickup)
            .or(if returning { None } else { attack })
            .or_else(|| {
                // With an empty rifle, close on a visible enemy instead of
                // oscillating forever between its warning and the route goal.
                if !returning && ammo(&world) == 0 {
                    visible
                        .iter()
                        .min_by_key(|(_, p)| distance(at, *p))
                        .and_then(|(_, p)| {
                            step(&world, *p, if cautious { &threats } else { &empty_danger })
                        })
                } else {
                    None
                }
            })
            .or_else(|| {
                step(
                    &world,
                    goal,
                    if cautious { &threats } else { &empty_danger },
                )
            })
            .unwrap_or(GameCommand::Wait);
        let before = (hp(&world), ammo(&world), patches(&world), current);
        let is_equip = matches!(command, GameCommand::EquipItem { .. });
        let is_pickup = matches!(command, GameCommand::PickUp);
        let expected_loot = is_pickup.then(|| {
            let (_, item) = world
                .ground_items()
                .iter()
                .find(|(_, item)| item.position() == CACHE)
                .unwrap();
            (item.item().clone(), item.magic_modifiers())
        });
        let result = world.process_player_command(command.clone());
        assert_eq!(
            result,
            CommandOutcome::Applied,
            "zone {current}, {at:?}, {command:?}"
        );
        if is_equip {
            equipped = true;
        }
        if is_pickup {
            acquired = true;
            let (definition, modifiers) = expected_loot.unwrap();
            let carried = world
                .player_inventory()
                .iter()
                .find(|item| item.item() == &definition)
                .unwrap();
            assert_eq!(
                carried.magic_modifiers(),
                modifiers,
                "picking up the cache must preserve its actual roll, not reroll or grant ideal bonuses"
            );
            assert_eq!(world.ground_items().count_at(CACHE), 0);
        }
        if zone(&world) != current {
            transitions += 1;
            departed = true;
            assert_eq!(
                (hp(&world), ammo(&world), patches(&world)),
                (before.0, before.1, before.2),
                "passage must not refill resources"
            );
        }
        assert!(
            ammo(&world) <= before.1,
            "no ammunition refill during this course"
        );
        world.drain_events();
    }
    Report {
        dead: world.status() == RunStatus::PlayerDestroyed,
        returned: departed && zone(&world) == 0,
        reached_goal,
        acquired,
        equipped,
        turns: world.turn(),
        hp: hp(&world),
        ammunition: ammo(&world),
        patches: patches(&world),
        transitions,
    }
}

#[test]
fn survival_expedition_measures_every_seed_once_without_resurrection() {
    let (rules, _, loot, _) = ascii_game_content().unwrap();
    let catalog = ascii_regional_world_catalog().unwrap();
    for cautious in [true, false] {
        for seed in 0..16 {
            let report = run(expedition(&rules, &loot, &catalog, seed), cautious);
            println!("seed {seed}, cautious {cautious}: {report:?}");
            assert!(
                report.dead || report.returned,
                "bounded course must terminate, never silently discard a stalled seed"
            );
            if cautious {
                assert!(
                    report.returned && !report.dead,
                    "this open fixture must retain a cautious return route, including abandonment of the cache"
                );
            }
            if report.acquired && report.returned {
                assert!(report.equipped);
            }
        }
    }
}

#[test]
fn survival_expedition_death_ends_commands_and_new_run_does_not_inherit_resources() {
    let (rules, _, loot, _) = ascii_game_content().unwrap();
    let catalog = ascii_regional_world_catalog().unwrap();
    let mut world = expedition(&rules, &loot, &catalog, 6);
    for _ in 0..9 {
        assert_eq!(
            world.process_player_command(GameCommand::Move(Direction::East)),
            CommandOutcome::Applied
        );
    }
    assert_eq!(
        world.process_player_command(GameCommand::Interact { target: EXIT }),
        CommandOutcome::Applied
    );
    for _ in 0..2 {
        world.process_player_command(GameCommand::Move(Direction::East));
    }
    for _ in 0..120 {
        if world.status() != RunStatus::Active {
            break;
        }
        world.process_player_command(GameCommand::Wait);
    }
    assert_eq!(world.status(), RunStatus::PlayerDestroyed);
    let turn = world.turn();
    let patch = world
        .player_inventory()
        .iter()
        .find(|e| e.item().as_str() == PATCH)
        .unwrap()
        .instance();
    for command in [
        GameCommand::Wait,
        GameCommand::Move(Direction::West),
        GameCommand::Interact { target: ENTRY },
        GameCommand::PickUp,
        GameCommand::UseItem { item: patch },
    ] {
        assert_eq!(
            world.process_player_command(command),
            CommandOutcome::Rejected(CommandRejection::RunEnded)
        );
        assert_eq!(world.turn(), turn);
    }
    let fresh = expedition(&rules, &loot, &catalog, 7);
    assert_eq!(fresh.status(), RunStatus::Active);
    assert_eq!(
        (
            zone(&fresh),
            ammo(&fresh),
            patches(&fresh),
            fresh.visited_zone_count()
        ),
        (0, 12, 2, 1)
    );
    assert_eq!(fresh.player_inventory().iter().count(), 3);
}

#[test]
fn survival_expedition_real_wound_care_and_retreat_preserve_spent_resources() {
    let (rules, _, loot, _) = ascii_game_content().unwrap();
    let catalog = ascii_regional_world_catalog().unwrap();
    let mut world = expedition(&rules, &loot, &catalog, 6);
    for _ in 0..9 {
        assert_eq!(
            world.process_player_command(GameCommand::Move(Direction::East)),
            CommandOutcome::Applied
        );
    }
    assert_eq!(
        world.process_player_command(GameCommand::Interact { target: EXIT }),
        CommandOutcome::Applied
    );
    for _ in 0..2 {
        assert_eq!(
            world.process_player_command(GameCommand::Move(Direction::East)),
            CommandOutcome::Applied
        );
    }
    // Real enemy damage, not a free health adjustment or a synthetic potion.
    for _ in 0..40 {
        if hp(&world) <= 14 {
            break;
        }
        assert_eq!(
            world.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
    }
    assert!(hp(&world) > 0 && hp(&world) <= 14);
    let item = world
        .player_inventory()
        .iter()
        .find(|e| e.item().as_str() == PATCH)
        .unwrap()
        .instance();
    world.drain_events();
    assert_eq!(
        world.process_player_command(GameCommand::UseItem { item }),
        CommandOutcome::Applied
    );
    assert_eq!(patches(&world), 1);
    assert!(world.drain_events().iter().any(|e| matches!(e, GameEvent::IntegrityRestored { entity, amount: 6 } if *entity == world.player_id())));
    assert_eq!(
        world.process_player_command(GameCommand::Move(Direction::West)),
        CommandOutcome::Applied
    );
    let before = (hp(&world), patches(&world), ammo(&world));
    assert!(before.0 > 0);
    assert_eq!(
        world.process_player_command(GameCommand::Interact { target: ENTRY }),
        CommandOutcome::Applied
    );
    assert_eq!(zone(&world), 0);
    assert_eq!((hp(&world), patches(&world), ammo(&world)), before);
    world.drain_events();
    let bytes = world.recovery_snapshot_bytes().unwrap();
    let restored = WorldState::from_recovery_snapshot_bytes(&bytes, world.rules().clone()).unwrap();
    assert_eq!(restored.recovery_snapshot_bytes().unwrap(), bytes);
    assert_eq!((hp(&restored), patches(&restored), ammo(&restored)), before);
}
