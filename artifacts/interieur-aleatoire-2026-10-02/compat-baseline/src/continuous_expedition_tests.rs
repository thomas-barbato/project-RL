//! Fixed, single-life pilots through the real client's lazily generated atlas.
//! Known route/terrain and a finite fixture kit: not blind campaign win rates.
use super::expedition_survival_tests::{ammo, danger, finite_rules, hp, patches};
use super::first_layer_expedition_tests::{Style, anchor, attack_command};
use super::generated_survival_tests::next_step;
use super::*;
use std::collections::BTreeSet;

struct Checkpoint {
    coordinate: RegionCoord,
    turn: u64,
    level: u16,
    xp: u64,
    hp: u16,
    maximum_hp: u16,
    ammo: u16,
    patches: u16,
    inventory: String,
}

impl std::fmt::Debug for Checkpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} t={} level={} xp={} hp={}/{} ammo={} patches={} inventory={}",
            self.coordinate,
            self.turn,
            self.level,
            self.xp,
            self.hp,
            self.maximum_hp,
            self.ammo,
            self.patches,
            self.inventory
        )
    }
}

fn checkpoint(app: &AsciiApp) -> Checkpoint {
    Checkpoint {
        coordinate: app.regional_zones[&app.game.current_zone().unwrap().id],
        turn: app.game.turn(),
        level: app.game.player_progression().level(),
        xp: app.game.player_progression().experience(),
        hp: hp(&app.game),
        maximum_hp: app
            .game
            .actors()
            .get(app.game.player_id())
            .map_or(0, Actor::maximum_integrity),
        ammo: ammo(&app.game),
        patches: patches(&app.game),
        inventory: app
            .game
            .player_inventory()
            .iter()
            .map(|e| {
                format!(
                    "{}x{}{}",
                    e.item(),
                    e.quantity(),
                    if e.magic_modifiers().is_some() {
                        "[bonus]"
                    } else {
                        ""
                    }
                )
            })
            .collect::<Vec<_>>()
            .join(","),
    }
}

#[derive(Debug, Default)]
struct Report {
    arrivals: Vec<Checkpoint>,
    destination_reached: bool,
    returned: bool,
    dead: bool,
    stopped: Option<String>,
    kills: usize,
    pickups: usize,
    equips: usize,
    shots: usize,
    heals: usize,
    level_healing: u32,
    item_healing: u32,
    max_turn_damage: u16,
    last_damage: Option<String>,
    final_state: Option<Checkpoint>,
    cover_moves: usize,
    weapon_swaps: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Policy {
    Baseline,
    Cover,
    CoverAndGear,
}

fn route(app: &AsciiApp, detour: bool, underground: usize) -> Vec<RegionCoord> {
    let source = app
        .regional_worlds
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap();
    let plan = source.first_layer_plan(app.seed).unwrap();
    let mut route = vec![RegionCoord::new(0, 0, 0)];
    if detour {
        route.extend([RegionCoord::new(0, 1, 0), RegionCoord::new(-1, 1, 0)]);
    }
    route.push(RegionCoord::new(-1, 0, 0));
    route.extend(plan.routes[underground].iter().copied());
    route.extend(plan.arrival_route);
    route
}

// A deliberately simple equipment policy: put on an acquired armor only if
// its base protection improves. Keep all affixes, never reroll or grant items.
fn equip_upgrade(app: &AsciiApp) -> Option<GameCommand> {
    let slot: ContentId = "core:body_armor".parse().unwrap();
    let armor = |entry: &project_rl::entity::InventoryEntry| {
        app.rules
            .items
            .get(entry.item())
            .and_then(|d| d.equipment())
            .filter(|p| p.slot() == &slot)
            .map(|p| p.armor())
    };
    let current = app
        .game
        .player_equipment()
        .equipped(&slot)
        .and_then(|id| app.game.player_inventory().get(id))
        .and_then(armor)
        .unwrap_or(0);
    app.game
        .player_inventory()
        .iter()
        .filter_map(|e| armor(e).filter(|&a| a > current).map(|a| (a, e.instance())))
        .max_by_key(|&(a, id)| (a, std::cmp::Reverse(id)))
        .map(|(_, item)| GameCommand::EquipItem { slot, item })
}

#[test]
fn continuous_expedition_routes_use_real_open_atlas_links_and_finite_start() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let app = AsciiApp::from_seed_version(0, finite_rules(&rules), texts, loot, expeditions, 122)
        .unwrap();
    assert_eq!(
        (hp(&app.game), ammo(&app.game), patches(&app.game)),
        (20, 12, 2)
    );
    assert_eq!(app.game.player_progression().level(), 1);
    assert_eq!(app.game.player_progression().experience(), 0);
    assert_eq!(app.game.player_inventory().len(), 3);
    assert!(
        app.game
            .player_inventory()
            .iter()
            .all(|e| e.magic_modifiers().is_none())
    );
    assert_eq!(equip_upgrade(&app), None);
    let world = app
        .regional_worlds
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap()
        .resolved_for_seed(app.seed);
    for detour in [false, true] {
        for underground in [0, 1] {
            let itinerary = route(&app, detour, underground);
            assert_eq!(
                itinerary.iter().collect::<BTreeSet<_>>().len(),
                itinerary.len()
            );
            assert_eq!(itinerary.first(), Some(&RegionCoord::new(0, 0, 0)));
            assert_eq!(itinerary.last(), Some(&RegionCoord::new(-1, 0, 2)));
            for pair in itinerary.windows(2) {
                if pair[0].depth == pair[1].depth {
                    assert!(world.cardinal_neighbors(pair[0]).contains(&pair[1]));
                } else {
                    assert_eq!(
                        world.vertical_neighbor(pair[0], RegionVerticalDirection::Down),
                        Some(pair[1])
                    );
                }
            }
        }
    }
}

#[test]
fn continuous_expedition_equips_a_found_armor_without_replacing_the_inventory() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut app =
        AsciiApp::from_seed_version(0, finite_rules(&rules), texts, loot, expeditions, 122)
            .unwrap();
    let at = app.game.player_position().unwrap();
    app.game
        .spawn_ground_item(at, "core:veste_matelassee".parse().unwrap(), 1)
        .unwrap();
    assert_eq!(
        app.execute_command(GameCommand::PickUp),
        CommandOutcome::Applied
    );
    let command = equip_upgrade(&app).expect("a genuinely acquired armor is usable");
    let inventory = app.game.player_inventory().clone();
    assert_eq!(app.execute_command(command), CommandOutcome::Applied);
    assert_eq!(app.game.player_inventory(), &inventory);
    assert!(
        app.game
            .player_equipment()
            .equipped(&"core:body_armor".parse().unwrap())
            .is_some()
    );
    assert_eq!(
        equip_upgrade(&app),
        None,
        "no repeated equip or artificial upgrade"
    );
}

fn run(app: AsciiApp, detour: bool, underground: usize, style: Style) -> Report {
    run_policy(app, detour, underground, style, Policy::Baseline)
}

fn run_policy(
    mut app: AsciiApp,
    detour: bool,
    underground: usize,
    style: Style,
    policy: Policy,
) -> Report {
    let home = app.game.player_position().unwrap();
    let route = route(&app, detour, underground);
    let world = app
        .regional_worlds
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap()
        .resolved_for_seed(app.seed);
    let mut report = Report::default();
    let mut returning = false;
    let mut idle = 0;
    let mut skipped_loot = BTreeSet::new();
    let mut cover_budget = super::expedition_decisions_tests::CoverBudget::default();
    app.game.drain_events();
    report.arrivals.push(checkpoint(&app));
    for _ in 0..2000 {
        if app.game.status() != RunStatus::Active {
            break;
        }
        let zone = app.game.current_zone().unwrap().id.clone();
        let coordinate = app.regional_zones[&zone];
        let index = route.iter().position(|&c| c == coordinate).unwrap();
        if index == route.len() - 1 {
            report.destination_reached = true;
            returning = true;
        }
        if returning && index == 0 && app.game.player_position() == Some(home) {
            report.returned = true;
            break;
        }
        returning |= hp(&app.game) <= 6 && patches(&app.game) == 0;
        if returning && index == 0 && app.game.player_position() == Some(home) {
            report.returned = true;
            break;
        }
        let goal = if index == 0 {
            if returning {
                home
            } else {
                let direction = if detour {
                    project_rl::content::RegionDirection::South
                } else {
                    project_rl::content::RegionDirection::West
                };
                AsciiApp::hub_regional_passage(direction).unwrap()
            }
        } else {
            let next = route[if returning { index - 1 } else { index + 1 }];
            anchor(&world, coordinate, next)
        };
        let at = app.game.player_position().unwrap();
        let visible: Vec<_> = app
            .game
            .actors()
            .iter()
            .filter(|(id, a)| {
                *id != app.game.player_id()
                    && a.player_relation() == project_rl::social::PlayerRelation::Hostile
                    && app.game.player_visibility().is_visible(a.position())
            })
            .map(|(id, a)| (id, a.position()))
            .collect();
        let warnings = danger(&app.game);
        let heal = (hp(&app.game) <= 12)
            .then(|| {
                app.game
                    .player_inventory()
                    .iter()
                    .find(|e| e.item().as_str() == "core:repair_patch")
                    .map(|e| GameCommand::UseItem { item: e.instance() })
            })
            .flatten();
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
                    app.game.map().is_walkable(at.step(d))
                        && app.game.actors().entity_at(at.step(d)).is_none()
                        && !warnings.contains(&at.step(d))
                })
                .min_by_key(|&d| at.step(d).x.abs_diff(goal.x) + at.step(d).y.abs_diff(goal.y))
                .map(GameCommand::Move)
            })
            .flatten();
        let travel = (!(index == 0 && returning)
            && at.x.abs_diff(goal.x) + at.y.abs_diff(goal.y) <= 1)
            .then_some(GameCommand::Interact { target: goal });
        // Only visible nearby loot in a quiet moment, never through a threat.
        let loot_step = if visible.is_empty() && !returning {
            app.game
                .ground_items()
                .iter()
                .filter(|(_, stack)| stack.owner().is_none())
                .map(|(_, stack)| stack.position())
                .filter(|p| {
                    app.game.player_visibility().is_visible(*p)
                        && !skipped_loot.contains(&(zone.clone(), *p))
                        && at.x.abs_diff(p.x) + at.y.abs_diff(p.y) <= 5
                })
                .min_by_key(|p| (at.x.abs_diff(p.x) + at.y.abs_diff(p.y), *p))
                .and_then(|p| {
                    if p == at {
                        Some(GameCommand::PickUp)
                    } else {
                        next_step(&app.game, p, &warnings)
                    }
                })
        } else {
            None
        };
        let equip = visible.is_empty().then(|| equip_upgrade(&app)).flatten();
        let weapon = (policy == Policy::CoverAndGear && visible.is_empty())
            .then(|| super::expedition_decisions_tests::weapon_upgrade(&app.game))
            .flatten();
        let cover = (policy != Policy::Baseline
            && cover_budget.available()
            && (returning || heal.is_some())
            && travel.is_none())
        .then(|| {
            super::expedition_decisions_tests::cover_step(&app.game, goal, &warnings, &visible)
        })
        .flatten();
        let used_cover = escape.is_none() && cover.is_some();
        let step = next_step(&app.game, goal, &warnings);
        let attack = attack_command(&app.game, style, &visible);
        let command = escape
            .or(cover)
            .or(heal)
            .or(travel)
            .or(weapon)
            .or(equip)
            .or(loot_step)
            .or_else(|| {
                if returning || matches!(style, Style::Avoid) {
                    step.or(attack)
                } else {
                    attack.or(step)
                }
            })
            .unwrap_or(GameCommand::Wait);
        idle = if command == GameCommand::Wait {
            idle + 1
        } else {
            0
        };
        if idle >= 12 {
            report.stopped = Some(format!("12 waits at {coordinate:?} {at:?}"));
            break;
        }
        let before = (
            hp(&app.game),
            ammo(&app.game),
            patches(&app.game),
            app.game.player_progression().clone(),
            app.game.player_inventory().clone(),
            app.game.player_equipment().clone(),
        );
        let outcome = app.execute_command(command.clone());
        let events = app.game.drain_events();
        if outcome == CommandOutcome::Rejected(CommandRejection::InventoryCannotFitItem)
            && command == GameCommand::PickUp
        {
            skipped_loot.insert((zone.clone(), at));
            continue;
        }
        if outcome != CommandOutcome::Applied {
            report.stopped = Some(format!("{coordinate:?} {at:?}: {command:?}: {outcome:?}"));
            break;
        }
        report.pickups += usize::from(command == GameCommand::PickUp);
        report.cover_moves += usize::from(used_cover);
        if used_cover {
            cover_budget.record_move();
        }
        report.weapon_swaps += usize::from(matches!(command, GameCommand::EquipWeapon { .. }));
        report.equips += usize::from(matches!(command, GameCommand::EquipItem { .. }));
        report.shots += usize::from(matches!(command, GameCommand::Attack { slot: 1, .. }));
        report.heals += usize::from(matches!(command, GameCommand::UseItem { .. }));
        let level = events
            .iter()
            .any(|e| matches!(e, GameEvent::LevelGained { .. }));
        let mut damage = 0;
        for event in &events {
            match event {
                GameEvent::EntityDefeatedByPlayer { .. } => report.kills += 1,
                GameEvent::IntegrityRestored { entity, amount }
                    if *entity == app.game.player_id() =>
                {
                    if level {
                        report.level_healing += u32::from(*amount);
                    } else {
                        report.item_healing += u32::from(*amount);
                    }
                }
                GameEvent::DamageApplied { target, amount, .. }
                | GameEvent::DamageImpactApplied { target, amount, .. }
                    if *target == app.game.player_id() =>
                {
                    damage += amount;
                    report.last_damage = Some(format!(
                        "{coordinate:?} t={} {command:?} {event:?}",
                        app.game.turn()
                    ));
                }
                _ => {}
            }
        }
        report.max_turn_damage = report.max_turn_damage.max(damage);
        if app.game.current_zone().unwrap().id != zone {
            cover_budget.enter_region();
            assert_eq!(
                before,
                (
                    hp(&app.game),
                    ammo(&app.game),
                    patches(&app.game),
                    app.game.player_progression().clone(),
                    app.game.player_inventory().clone(),
                    app.game.player_equipment().clone()
                ),
                "travel cannot reset the character"
            );
            let bytes = app.game.recovery_snapshot_bytes().unwrap();
            let restored =
                WorldState::from_recovery_snapshot_bytes(&bytes, app.rules.clone()).unwrap();
            assert_eq!(bytes, restored.recovery_snapshot_bytes().unwrap());
            app.game = restored;
            report.arrivals.push(checkpoint(&app));
        }
    }
    report.dead = app.game.status() == RunStatus::PlayerDestroyed;
    report.final_state = Some(checkpoint(&app));
    if !report.dead && !report.returned && report.stopped.is_none() {
        report.stopped = Some("2000 action limit".into());
    }
    report
}

fn sample(seed: u64, detour: bool) {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    for style in [Style::Melee, Style::Ranged, Style::Avoid] {
        let app = AsciiApp::from_seed_version(
            seed,
            finite_rules(&rules),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
            122,
        )
        .unwrap();
        let report = run(app, detour, seed as usize, style);
        println!(
            "CONTINUOUS seed={seed} detour={detour} underground={seed} style={style:?} {report:?}"
        );
        assert!(report.stopped.is_none(), "pilot failure: {report:?}");
        assert!(report.dead || report.returned);
    }
}

macro_rules! sample_case {
    ($name:ident, $seed:literal, $detour:literal) => {
        #[test]
        #[ignore = "continuous surface-to-depth fixed sample; run with --ignored --nocapture"]
        fn $name() {
            sample($seed, $detour);
        }
    };
}
sample_case!(continuous_expedition_sample_zero_direct, 0, false);
sample_case!(continuous_expedition_sample_zero_detour, 0, true);
sample_case!(continuous_expedition_sample_one_direct, 1, false);
sample_case!(continuous_expedition_sample_one_detour, 1, true);

fn cover_sample(seed: u64, policy: Policy) {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let mut failures = Vec::new();
    for style in [Style::Melee, Style::Ranged, Style::Avoid] {
        let app = AsciiApp::from_seed_version(
            seed,
            finite_rules(&rules),
            texts.clone(),
            loot.clone(),
            expeditions.clone(),
            122,
        )
        .unwrap();
        let report = run_policy(app, true, seed as usize, style, policy);
        println!("COVER seed={seed} policy={policy:?} style={style:?} {report:?}");
        if report.stopped.is_some() || !(report.dead || report.returned) {
            failures.push((style, report));
        }
    }
    assert!(failures.is_empty(), "pilot failures: {failures:?}");
}
macro_rules! cover_case {
    ($name:ident, $seed:literal, $policy:ident) => {
        #[test]
        #[ignore = "fixed surface retreats: cover versus cover and acquired gear"]
        fn $name() {
            cover_sample($seed, Policy::$policy);
        }
    };
}
cover_case!(continuous_cover_sample_zero, 0, Cover);
cover_case!(continuous_cover_sample_one, 1, Cover);
cover_case!(continuous_cover_sample_zero_gear, 0, CoverAndGear);
cover_case!(continuous_cover_sample_one_gear, 1, CoverAndGear);
