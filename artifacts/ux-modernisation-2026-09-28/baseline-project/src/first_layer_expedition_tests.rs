//! Bounded, single-life diagnostic pilots. Known terrain/route, visible threats
//! only: these are NOT blind-player win rates or the canonical starting kit.
use super::expedition_survival_tests::{ammo, danger, finite_rules, hp, patches};
use super::generated_survival_tests::next_step;
use super::*;
use project_rl::content::RegionDirection;
use project_rl::game::ZoneInfo;
use project_rl::world::Map;
use project_rl::world::generation::{cardinal_passage, vertical_passage};
use std::collections::BTreeSet;

pub(super) fn anchor(
    world: &project_rl::content::RegionalWorldDefinition,
    from: RegionCoord,
    to: RegionCoord,
) -> GridPos {
    let size = world.map_size_at(from);
    if from.depth != to.depth {
        return vertical_passage(
            size,
            if from.depth < to.depth {
                RegionVerticalDirection::Down
            } else {
                RegionVerticalDirection::Up
            },
        );
    }
    let direction = [
        RegionDirection::North,
        RegionDirection::East,
        RegionDirection::South,
        RegionDirection::West,
    ]
    .into_iter()
    .find(|d| from.step(*d) == Some(to))
    .unwrap();
    cardinal_passage(size, direction_from_region(direction))
}

fn build(
    app: &AsciiApp,
    seed: u64,
    route: usize,
    kit: PlainKit,
) -> (WorldState, Vec<ContentId>, Vec<GridPos>, Vec<GridPos>) {
    let source = app
        .regional_worlds
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap();
    let plan = source.first_layer_plan(seed).unwrap();
    let definition = source.resolved_for_seed(seed);
    let coordinates: Vec<_> = plan.routes[route]
        .iter()
        .copied()
        .chain(plan.arrival_route)
        .collect();
    let mut outgoing = Vec::new();
    let mut incoming = Vec::new();
    let mut infos = Vec::new();
    let refuge = ZoneInfo {
        id: "test:layer_start".parse().unwrap(),
        name: "Départ du diagnostic".into(),
        kind: "core:human_habitat".parse().unwrap(),
        depth: 0,
    };
    let mut game = WorldState::single(
        GameState::new_with_rules(
            Map::from_ascii("#####\n#...#\n#####").unwrap(),
            GridPos::new(1, 1),
            seed,
            kit.rules(&app.rules),
        )
        .unwrap(),
    );
    game.enable(refuge.clone()).unwrap();
    kit.equip_armor(&mut game);
    for (i, &coordinate) in coordinates.iter().enumerate() {
        let descriptor = definition.region(seed, coordinate).unwrap();
        let entry = if i == 0 {
            vertical_passage(
                definition.map_size_at(coordinate),
                RegionVerticalDirection::Up,
            )
        } else {
            anchor(&definition, coordinate, coordinates[i - 1])
        };
        let exit = coordinates
            .get(i + 1)
            .map_or(entry, |&next| anchor(&definition, coordinate, next));
        let info = crate::test_regional::zone_info(&definition, &descriptor).unwrap();
        let mut generated = crate::test_regional::generate(
            &definition,
            &descriptor,
            info.clone(),
            entry,
            Some(&app.loot),
            first_layer_landscapes::diagnostic_features(),
        )
        .unwrap();
        app.populate_regional_equipment(
            &mut generated.blueprint,
            &descriptor.biome,
            descriptor.seed,
        )
        .unwrap();
        app.populate_carried_weapons(&mut generated.blueprint, descriptor.seed)
            .unwrap();
        game.add_zone(generated.blueprint).unwrap();
        if let Some(facility) = generated.facility {
            game.register_facility(info.id.clone(), facility).unwrap();
        }
        incoming.push(entry);
        outgoing.push(exit);
        infos.push(info.id);
    }
    game.connect(refuge.id, GridPos::new(2, 1), infos[0].clone(), incoming[0])
        .unwrap();
    for i in 1..infos.len() {
        game.connect(
            infos[i - 1].clone(),
            outgoing[i - 1],
            infos[i].clone(),
            incoming[i],
        )
        .unwrap();
    }
    assert_eq!(
        game.process_player_command(GameCommand::Interact {
            target: GridPos::new(2, 1)
        }),
        CommandOutcome::Applied
    );
    game.drain_events();
    (game, infos, incoming, outgoing)
}

#[derive(Clone, Copy, Debug)]
pub(super) enum Style {
    Melee,
    Ranged,
    Avoid,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Objective {
    Expedition,
    Scout,
    WoundedRetreat,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PlainKit {
    Bare,
    Padded,
    SapperPadded,
}

impl PlainKit {
    fn rules(self, rules: &GameRules) -> GameRules {
        let mut rules = finite_rules(rules);
        if self == Self::SapperPadded {
            let knife = "core:couteau_de_sapeur".parse().unwrap();
            rules.player_starting_weapons[0] = knife;
            rules.player_starting_equipment[0] = Some(rules.player_starting_weapons[0].clone());
            rules.player_base_attacks[0] = rules
                .weapons
                .get(&rules.player_starting_weapons[0])
                .unwrap()
                .attack();
        }
        if self != Self::Bare {
            rules
                .player_starting_items
                .push(project_rl::game::StartingItemStack::new(
                    "core:veste_matelassee".parse().unwrap(),
                    1,
                ));
        }
        rules
    }

    fn equip_armor(self, game: &mut WorldState) {
        if self == Self::Bare {
            return;
        }
        let item = game
            .player_inventory()
            .iter()
            .find(|entry| entry.item().as_str() == "core:veste_matelassee")
            .unwrap()
            .instance();
        assert_eq!(
            game.process_player_command(GameCommand::EquipItem {
                slot: "core:body_armor".parse().unwrap(),
                item,
            }),
            CommandOutcome::Applied
        );
    }
}

#[derive(Debug)]
struct Report {
    version: u8,
    seed: u64,
    route: usize,
    style: Style,
    destination_reached: bool,
    returned: bool,
    dead: bool,
    interruption: Option<String>,
    turns: u64,
    minimum_hp: u16,
    final_hp: u16,
    shots: usize,
    heals: usize,
    contacts: usize,
    transitions: usize,
    last_hit: Option<String>,
    maximum_hit: u16,
    maximum_turn_damage: u16,
    maximum_turn_sources: usize,
    damage_trace: Vec<String>,
    experience_earned: u64,
    kills: usize,
    level: u16,
    maximum_hp: u16,
    level_healing: u32,
    item_healing: u32,
    level_turns: Vec<u64>,
}

pub(super) fn attack_command(
    game: &WorldState,
    style: Style,
    visible: &[(EntityId, GridPos)],
) -> Option<GameCommand> {
    let at = game.player_position()?;
    if game
        .actors()
        .get(game.player_id())?
        .recovery_remaining()
        .is_some()
    {
        return None;
    }
    visible
        .iter()
        .filter_map(|&(target, p)| {
            let distance = at.x.abs_diff(p.x).max(at.y.abs_diff(p.y));
            let slot = if matches!(style, Style::Ranged) && ammo(game) > 0 && distance > 1 {
                1
            } else {
                0
            };
            let attack = game.equipped_player_weapon(slot)?.attack();
            (attack.is_in_range(at, p)
                && !game.map().is_protected(at)
                && !game.map().is_protected(p)
                && (!attack.requires_line_of_sight()
                    || project_rl::world::has_line_of_sight(game.map(), at, p, true)))
            .then_some(((distance, target), GameCommand::Attack { slot, target }))
        })
        .min_by_key(|(key, _)| *key)
        .map(|(_, command)| command)
}

fn run(
    app: &AsciiApp,
    seed: u64,
    route: usize,
    style: Style,
    check_resume: bool,
    scout_only: bool,
) -> Report {
    run_with_kit(
        app,
        seed,
        route,
        style,
        check_resume,
        if scout_only {
            Objective::Scout
        } else {
            Objective::Expedition
        },
        PlainKit::Bare,
    )
}

fn run_with_kit(
    app: &AsciiApp,
    seed: u64,
    route: usize,
    style: Style,
    check_resume: bool,
    objective: Objective,
    kit: PlainKit,
) -> Report {
    let (mut game, ids, incoming, outgoing) = build(app, seed, route, kit);
    let initial_experience = game.player_progression().experience();
    let mut report = Report {
        version: app.generation_version,
        seed,
        route,
        style,
        destination_reached: false,
        returned: false,
        dead: false,
        interruption: None,
        turns: 0,
        minimum_hp: hp(&game),
        final_hp: hp(&game),
        shots: 0,
        heals: 0,
        contacts: 0,
        transitions: 0,
        last_hit: None,
        maximum_hit: 0,
        maximum_turn_damage: 0,
        maximum_turn_sources: 0,
        damage_trace: Vec::new(),
        experience_earned: 0,
        kills: 0,
        level: game.player_progression().level(),
        maximum_hp: game
            .actors()
            .get(game.player_id())
            .unwrap()
            .maximum_integrity(),
        level_healing: 0,
        item_healing: 0,
        level_turns: Vec::new(),
    };
    let mut contacts = BTreeSet::new();
    let mut returning = false;
    let mut departed = false;
    let mut idle = 0;
    let mut saved_once = false;
    for _ in 0..1200 {
        if game.status() != RunStatus::Active {
            break;
        }
        let index = ids
            .iter()
            .position(|id| id == &game.current_zone().unwrap().id)
            .unwrap();
        if index == ids.len() - 1 {
            report.destination_reached = true;
            returning = true;
        }
        if index > 0 {
            departed = true;
        }
        if returning && departed && index == 0 {
            report.returned = true;
            break;
        }
        returning |= hp(&game) <= 6 && patches(&game) == 0;
        let at = game.player_position().unwrap();
        let visible: Vec<_> = game
            .actors()
            .iter()
            .filter(|(id, a)| {
                *id != game.player_id()
                    && a.player_relation() == project_rl::social::PlayerRelation::Hostile
                    && game.player_visibility().is_visible(a.position())
            })
            .map(|(id, a)| (id, a.position()))
            .collect();
        contacts.extend(visible.iter().map(|(id, _)| *id));
        // Separate short scouting/reaction probes from completing the
        // expedition. Never substitute these results for the original sample.
        // Only the pure scouting probe stops at a quiet first region's exit.
        if index > 0
            && ((objective == Objective::Scout
                && (!visible.is_empty()
                    || at.x.abs_diff(outgoing[index].x) + at.y.abs_diff(outgoing[index].y) <= 1))
                || (objective == Objective::WoundedRetreat && report.maximum_hit > 0))
        {
            returning = true;
        }
        let goal = if returning {
            incoming[index]
        } else {
            outgoing[index]
        };
        let warnings = danger(&game);
        let heal = (hp(&game) <= 12)
            .then(|| {
                game.player_inventory()
                    .iter()
                    .find(|i| i.item().as_str() == "core:repair_patch")
                    .map(|i| GameCommand::UseItem { item: i.instance() })
            })
            .flatten();
        let attack = attack_command(&game, style, &visible);
        let step = next_step(&game, goal, &warnings);
        let escape = warnings
            .contains(&at)
            .then(|| {
                [
                    Direction::North,
                    Direction::East,
                    Direction::South,
                    Direction::West,
                ]
                .into_iter()
                .filter(|&d| {
                    game.map().is_walkable(at.step(d))
                        && game.actors().entity_at(at.step(d)).is_none()
                        && !warnings.contains(&at.step(d))
                })
                .min_by_key(|&d| at.step(d).x.abs_diff(goal.x) + at.step(d).y.abs_diff(goal.y))
                .map(GameCommand::Move)
            })
            .flatten();
        let travel = (at.x.abs_diff(goal.x) + at.y.abs_diff(goal.y) <= 1)
            .then_some(GameCommand::Interact { target: goal });
        let pickup = game
            .ground_items()
            .item_at(at)
            .is_some()
            .then_some(GameCommand::PickUp);
        let command = escape
            // A separate cautious probe: disengage before spending a heal under
            // an adjacent hostile. Keep the historical expedition pilot intact.
            .or_else(|| {
                (objective == Objective::WoundedRetreat
                    && returning
                    && visible
                        .iter()
                        .any(|(_, p)| at.x.abs_diff(p.x).max(at.y.abs_diff(p.y)) <= 1))
                .then(|| step.clone())
                .flatten()
            })
            .or(heal)
            .or(travel)
            .or(pickup)
            .or_else(|| {
                if returning || matches!(style, Style::Avoid) {
                    step.clone().or(attack.clone())
                } else {
                    attack.clone().or(step.clone())
                }
            })
            .unwrap_or(GameCommand::Wait);
        idle = if command == GameCommand::Wait {
            idle + 1
        } else {
            0
        };
        if idle >= 12 {
            report.interruption = Some("pilote sans action pendant 12 tours".into());
            break;
        }
        let restored = if check_resume && !saved_once && !visible.is_empty() {
            let bytes = game.recovery_snapshot_bytes().unwrap();
            saved_once = true;
            Some(WorldState::from_recovery_snapshot_bytes(&bytes, kit.rules(&app.rules)).unwrap())
        } else {
            None
        };
        let before = (hp(&game), ammo(&game), patches(&game));
        let previous = game.current_zone().unwrap().id.clone();
        let result = game.process_player_command(command.clone());
        let events = game.drain_events();
        // These ordinary kits have no life-steal or other healing effect.
        // Keep attribution explicit: a consumable action cannot also earn a
        // level in this bounded pilot. Fail if that assumption stops holding.
        let gained_level = events
            .iter()
            .any(|e| matches!(e, GameEvent::LevelGained { .. }));
        let used_item = matches!(command, GameCommand::UseItem { .. });
        assert!(
            !(gained_level && used_item),
            "ambiguous diagnostic healing source"
        );
        for event in &events {
            match event {
                GameEvent::ExperienceAwarded { amount, .. } => report.experience_earned += amount,
                GameEvent::EntityDefeatedByPlayer { .. } => report.kills += 1,
                GameEvent::LevelGained { level, .. } => {
                    report.level = *level;
                    report.level_turns.push(game.turn());
                }
                GameEvent::IntegrityRestored { entity, amount } if *entity == game.player_id() => {
                    if gained_level {
                        report.level_healing += u32::from(*amount);
                    } else {
                        assert!(used_item, "unexpected diagnostic healing source");
                        report.item_healing += u32::from(*amount);
                    }
                }
                _ => {}
            }
        }
        let mut turn_damage = 0_u16;
        let mut turn_sources = BTreeSet::new();
        for event in &events {
            let (source, target, amount) = match event {
                GameEvent::DamageApplied {
                    source,
                    target,
                    amount,
                    ..
                }
                | GameEvent::DamageImpactApplied {
                    source,
                    target,
                    amount,
                    ..
                } => (*source, *target, *amount),
                _ => continue,
            };
            if target == game.player_id() && amount > 0 {
                turn_damage = turn_damage.saturating_add(amount);
                if let Some(source) = source {
                    turn_sources.insert(source);
                }
                report.maximum_hit = report.maximum_hit.max(amount);
                let actor = source.and_then(|id| game.actors().get(id));
                report.damage_trace.push(format!(
                    "turn={} hp_before={} action={command:?} retreat={returning} visible={} source={source:?} ai={:?} state={:?} home={:?} tags={:?} attacks={:?} raw_slot0={:?} event={event:?} zone={}",
                    game.turn(), before.0, visible.len(),
                    actor.and_then(|a| a.ai()).map(|ai| ai.behavior),
                    actor.map(|a| a.ai_state()), actor.and_then(|a| a.ai_home()),
                    actor.map(|a| a.tags()), actor.map(|a| a.attacks()),
                    source.and_then(|id| actor.and_then(|a| a.attack(0)).and_then(|attack| game.resolved_attack_damage(id, attack))),
                    game.current_zone().unwrap().name,
                ));
                report.last_hit = Some(format!(
                    "{} degats, {:?}, {}",
                    amount,
                    source
                        .and_then(|id| game.actors().get(id))
                        .and_then(|a| a.ai())
                        .map(|ai| ai.behavior),
                    game.current_zone().unwrap().name
                ));
            }
        }
        report.maximum_turn_damage = report.maximum_turn_damage.max(turn_damage);
        report.maximum_turn_sources = report.maximum_turn_sources.max(turn_sources.len());
        if let Some(mut restored) = restored {
            assert_eq!(result, restored.process_player_command(command.clone()));
            assert_eq!(events, restored.drain_events());
            assert_eq!(
                game.recovery_snapshot_bytes().unwrap(),
                restored.recovery_snapshot_bytes().unwrap()
            );
            game = restored;
        }
        if result != CommandOutcome::Applied {
            report.interruption = Some(format!("{command:?}: {result:?}"));
            break;
        }
        if previous != game.current_zone().unwrap().id {
            assert_eq!(
                (hp(&game), ammo(&game), patches(&game)),
                before,
                "no refill on travel"
            );
            report.transitions += 1;
        }
        report.shots += usize::from(matches!(command, GameCommand::Attack { slot: 1, .. }));
        report.heals += usize::from(matches!(command, GameCommand::UseItem { .. }));
        report.minimum_hp = report.minimum_hp.min(hp(&game));
        if let Some(player) = game.actors().get(game.player_id()) {
            report.maximum_hp = player.maximum_integrity();
        }
    }
    report.dead = game.status() == RunStatus::PlayerDestroyed;
    report.turns = game.turn();
    report.final_hp = hp(&game);
    report.contacts = contacts.len();
    assert_eq!(report.level, game.player_progression().level());
    assert_eq!(
        report.experience_earned,
        game.player_progression().experience() - initial_experience,
        "the diagnostic must account for every XP reward"
    );
    if !report.dead && !report.returned && report.interruption.is_none() {
        report.interruption = Some("limite de 1200 actions".into());
    }
    report
}

#[test]
fn expedition_pilot_can_really_fire_single_target_weapons() {
    let (rules, _, _, _) = ascii_game_content().unwrap();
    let mut game = WorldState::single(
        GameState::new_with_rules(
            Map::from_ascii("#########\n#.......#\n#.......#\n#########").unwrap(),
            GridPos::new(1, 1),
            0,
            finite_rules(&rules),
        )
        .unwrap(),
    );
    let position = GridPos::new(5, 1);
    let target = game.spawn_actor(Actor::new(position, 40).unwrap()).unwrap();
    let visible = [(target, position)];
    assert_eq!(attack_command(&game, Style::Melee, &visible), None);
    let shot = attack_command(&game, Style::Ranged, &visible).unwrap();
    assert_eq!(shot, GameCommand::Attack { slot: 1, target });
    let before = ammo(&game);
    assert_eq!(game.process_player_command(shot), CommandOutcome::Applied);
    assert_eq!(ammo(&game), before - 1);
}

#[test]
fn first_layer_plain_kits_keep_ordinary_health_and_finite_reserves() {
    let (rules, _, _, _) = ascii_game_content().unwrap();
    for kit in [PlainKit::Bare, PlainKit::Padded, PlainKit::SapperPadded] {
        let kit_rules = kit.rules(&rules);
        let expected_attack = kit_rules
            .weapons
            .get(&kit_rules.player_starting_weapons[0])
            .unwrap()
            .attack();
        let mut game = WorldState::single(
            GameState::new_with_rules(
                Map::from_ascii("#####\n#...#\n#####").unwrap(),
                GridPos::new(1, 1),
                0,
                kit_rules.clone(),
            )
            .unwrap(),
        );
        let base_armor = game.actor_armor_profile(game.player_id()).unwrap();
        kit.equip_armor(&mut game);
        assert_eq!((hp(&game), ammo(&game), patches(&game)), (20, 12, 2));
        assert!(
            game.player_inventory()
                .iter()
                .all(|entry| entry.magic_modifiers().is_none())
        );
        assert_eq!(
            game.equipped_player_weapon(0).unwrap().attack(),
            expected_attack
        );
        assert_eq!(
            game.actor_armor_profile(game.player_id())
                .unwrap()
                .equipment(),
            base_armor.equipment() + u16::from(kit != PlainKit::Bare)
        );
        game.drain_events();
        let snapshot = game.recovery_snapshot_bytes().unwrap();
        let restored = WorldState::from_recovery_snapshot_bytes(&snapshot, kit_rules).unwrap();
        assert_eq!(snapshot, restored.recovery_snapshot_bytes().unwrap());
    }
}

#[test]
#[ignore = "long fixed-seed comparative expedition diagnostic; run with --ignored --nocapture"]
fn first_layer_expedition_fixed_sample_without_retries() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    for version in [119, 120] {
        let app = AsciiApp::from_seed_version(
            INITIAL_SEED,
            rules.clone(),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
            version,
        )
        .unwrap();
        for seed in [0, 1] {
            for route in [0, 1] {
                for style in [Style::Melee, Style::Ranged, Style::Avoid] {
                    let report = run(
                        &app,
                        seed,
                        route,
                        style,
                        version == 120 && seed == 0 && route == 0,
                        false,
                    );
                    println!(
                        "v={} seed={} route={} style={:?} reached={} returned={} dead={} stop={:?} turns={} min_hp={} hp={} shots={} heals={} contacts={} transitions={} last_hit={:?}",
                        report.version,
                        report.seed,
                        report.route,
                        report.style,
                        report.destination_reached,
                        report.returned,
                        report.dead,
                        report.interruption,
                        report.turns,
                        report.minimum_hp,
                        report.final_hp,
                        report.shots,
                        report.heals,
                        report.contacts,
                        report.transitions,
                        report.last_hit
                    );
                    assert!(report.returned || report.dead || report.interruption.is_some());
                }
            }
        }
    }
}

#[test]
#[ignore = "first-contact retreat diagnostic on generated maps; run with --ignored --nocapture"]
fn first_layer_plain_kit_scout_retreat_sample() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let app =
        AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 120).unwrap();
    let mut contacted = 0;
    for seed in [0, 1] {
        for route in [0, 1] {
            let report = run(&app, seed, route, Style::Avoid, true, true);
            println!(
                "SCOUT seed={} route={} returned={} dead={} stop={:?} turns={} min_hp={} hp={} shots={} heals={} contacts={} transitions={} last_hit={:?}",
                seed,
                route,
                report.returned,
                report.dead,
                report.interruption,
                report.turns,
                report.minimum_hp,
                report.final_hp,
                report.shots,
                report.heals,
                report.contacts,
                report.transitions,
                report.last_hit
            );
            contacted += report.contacts;
            assert!(
                !report.destination_reached,
                "scouting must not substitute a full expedition"
            );
            assert!(
                report.interruption.is_none(),
                "a pilot failure must be investigated"
            );
            assert!(report.returned || report.dead);
            if report.returned {
                assert_eq!(report.transitions, 2);
            }
        }
    }
    assert!(
        contacted > 0,
        "at least one probe must retreat from a real hostile"
    );
}

#[test]
#[ignore = "fixed plain equipment matrix and damage traces; run with --ignored --nocapture"]
fn first_layer_plain_equipment_pressure_sample() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let app =
        AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 120).unwrap();
    for seed in [0, 1] {
        for route in [0, 1] {
            for kit in [PlainKit::Bare, PlainKit::Padded, PlainKit::SapperPadded] {
                for style in [Style::Melee, Style::Ranged, Style::Avoid] {
                    let report =
                        run_with_kit(&app, seed, route, style, true, Objective::Expedition, kit);
                    println!(
                        "PRESSURE seed={seed} route={route} kit={kit:?} style={style:?} reached={} returned={} dead={} stop={:?} turns={} min_hp={} shots={} heals={} contacts={} max_hit={} max_turn_damage={} max_turn_sources={}",
                        report.destination_reached,
                        report.returned,
                        report.dead,
                        report.interruption,
                        report.turns,
                        report.minimum_hp,
                        report.shots,
                        report.heals,
                        report.contacts,
                        report.maximum_hit,
                        report.maximum_turn_damage,
                        report.maximum_turn_sources
                    );
                    if kit == PlainKit::Bare && matches!(style, Style::Melee) && report.dead {
                        for hit in &report.damage_trace {
                            println!("HIT {hit}");
                        }
                    }
                    assert!(report.interruption.is_none(), "pilot failure: {report:?}");
                    assert!(report.returned || report.dead);
                }
            }
        }
    }
}

#[test]
#[ignore = "fixed v122 expedition with earned XP; run with --ignored --nocapture"]
fn first_layer_level_progression_sample() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let app =
        AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 122).unwrap();
    assert_eq!(app.rules.player_hit_points_per_level, 3);
    assert!(app.rules.player_full_heal_on_level_up);
    let mut observed_level_healing = false;
    for seed in [0, 1] {
        for route in [0, 1] {
            for style in [Style::Melee, Style::Ranged, Style::Avoid] {
                let report = run_with_kit(
                    &app,
                    seed,
                    route,
                    style,
                    true,
                    Objective::Expedition,
                    PlainKit::Bare,
                );
                println!(
                    "PROGRESSION seed={seed} route={route} style={style:?} reached={} returned={} dead={} turns={} min_hp={} hp={} max_hp={} shots={} heals={} kills={} xp={} level={} level_healing={} item_healing={} level_turns={:?} max_hit={} max_turn_damage={}",
                    report.destination_reached,
                    report.returned,
                    report.dead,
                    report.turns,
                    report.minimum_hp,
                    report.final_hp,
                    report.maximum_hp,
                    report.shots,
                    report.heals,
                    report.kills,
                    report.experience_earned,
                    report.level,
                    report.level_healing,
                    report.item_healing,
                    report.level_turns,
                    report.maximum_hit,
                    report.maximum_turn_damage,
                );
                assert!(report.interruption.is_none(), "pilot failure: {report:?}");
                assert!(report.returned || report.dead);
                assert_eq!(report.maximum_hp, 20 + 3 * (report.level - 1));
                observed_level_healing |= report.level > 1 && report.level_healing > 0;
            }
        }
    }
    assert!(
        observed_level_healing,
        "sample must exercise a real earned level and its healing"
    );
}

#[test]
#[ignore = "retreat after first wound on fixed generated maps; run with --ignored --nocapture"]
fn first_layer_plain_kit_wounded_retreat_sample() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let app =
        AsciiApp::from_seed_version(INITIAL_SEED, rules, texts, loot, expeditions, 120).unwrap();
    let mut wounded = 0;
    for seed in [0, 1] {
        for route in [0, 1] {
            let report = run_with_kit(
                &app,
                seed,
                route,
                Style::Melee,
                true,
                Objective::WoundedRetreat,
                PlainKit::Bare,
            );
            println!(
                "WOUNDED seed={seed} route={route} reached={} returned={} dead={} stop={:?} turns={} min_hp={} hp={} heals={} max_hit={} max_turn_damage={} max_turn_sources={}",
                report.destination_reached,
                report.returned,
                report.dead,
                report.interruption,
                report.turns,
                report.minimum_hp,
                report.final_hp,
                report.heals,
                report.maximum_hit,
                report.maximum_turn_damage,
                report.maximum_turn_sources
            );
            for hit in &report.damage_trace {
                println!("WOUNDED_HIT {hit}");
            }
            wounded += usize::from(report.maximum_hit > 0);
            assert!(report.interruption.is_none(), "pilot failure: {report:?}");
            assert!(report.returned || report.dead);
            if report.returned {
                assert!(report.transitions >= 2);
                assert_eq!(report.transitions % 2, 0);
            }
        }
    }
    assert!(wounded > 0, "probe must include a real injury");
}
