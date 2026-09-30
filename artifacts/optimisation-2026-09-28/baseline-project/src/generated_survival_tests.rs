//! Full regional generation, bounded retreat probes, and explicit failures.
use super::expedition_survival_tests::{ammo, danger, finite_rules, hp, patches};
use super::*;
use project_rl::ai::AiBehavior;
use project_rl::content::{RegionCoord, RegionDescriptor, RegionDirection};
use project_rl::game::ZoneInfo;
use project_rl::world::{DoorState, Map, Terrain, find_path_with};
use std::collections::BTreeSet;

const HOME: GridPos = GridPos::new(2, 1);

fn build(app: &AsciiApp, seed: u64) -> (WorldState, GridPos, GridPos) {
    let definition = app
        .regional_worlds
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap();
    let descriptor = RegionDescriptor {
        coordinate: RegionCoord::new(3, -2, 0),
        biome: "core:surface_wilds".parse().unwrap(),
        seed,
    };
    let entrance = project_rl::world::generation::cardinal_passage(
        definition.map_size_at(descriptor.coordinate),
        Direction::West,
    );
    let info = crate::test_regional::zone_info(definition, &descriptor).unwrap();
    let mut generated = crate::test_regional::generate(
        definition,
        &descriptor,
        info.clone(),
        entrance,
        Some(&app.loot),
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
        },
    )
    .unwrap();
    app.populate_regional_equipment(&mut generated.blueprint, &descriptor.biome, seed)
        .unwrap();
    app.populate_carried_weapons(&mut generated.blueprint, seed)
        .unwrap();
    let exit = generated
        .passages
        .iter()
        .find(|(d, _)| *d == RegionDirection::East)
        .unwrap()
        .1;
    let rules = finite_rules(&app.rules);
    let refuge = ZoneInfo {
        id: "test:retreat_refuge".parse().unwrap(),
        name: "Test refuge".into(),
        kind: "core:human_habitat".parse().unwrap(),
        depth: 0,
    };
    let mut world = WorldState::single(
        GameState::new_with_rules(
            Map::from_ascii("#####\n#...#\n#####").unwrap(),
            GridPos::new(1, 1),
            seed,
            rules,
        )
        .unwrap(),
    );
    world.enable(refuge.clone()).unwrap();
    world.add_zone(generated.blueprint).unwrap();
    if let Some(facility) = generated.facility {
        world.register_facility(info.id.clone(), facility).unwrap();
    }
    world.connect(refuge.id, HOME, info.id, entrance).unwrap();
    let before = (hp(&world), ammo(&world), patches(&world));
    assert_eq!(
        world.process_player_command(GameCommand::Interact { target: HOME }),
        CommandOutcome::Applied
    );
    assert_eq!(
        (hp(&world), ammo(&world), patches(&world)),
        before,
        "no ambush or resource reset on arrival"
    );
    world.drain_events();
    (world, entrance, exit)
}

pub(super) fn next_step(
    world: &WorldState,
    goal: GridPos,
    avoid: &BTreeSet<GridPos>,
) -> Option<GameCommand> {
    let from = world.player_position()?;
    let path = find_path_with(
        world.map(),
        from,
        goal,
        world.map().width() * world.map().height(),
        |map, at| {
            map.is_walkable(at)
                || map
                    .tile(at)
                    .is_some_and(|t| t.terrain == Terrain::Door(DoorState::Closed))
        },
        |at| !avoid.contains(&at) && world.actors().entity_at(at).is_none(),
    )?;
    let next = *path.get(1)?;
    if world.actors().entity_at(next).is_some() || avoid.contains(&next) {
        return None;
    }
    if world.map().tile(next)?.terrain == Terrain::Door(DoorState::Closed) {
        Some(GameCommand::Interact { target: next })
    } else {
        Some(GameCommand::Move(Direction::from_delta(
            next.x - from.x,
            next.y - from.y,
        )?))
    }
}

#[derive(Debug)]
struct Report {
    seed: u64,
    hesitation: u16,
    returned: bool,
    dead: bool,
    blocked: Option<String>,
    reached_exit: bool,
    contacts: usize,
    mobile_contacts: usize,
    turns: u64,
    minimum_hp: u16,
    ammunition: u16,
    soins: u16,
}

fn probe(app: &AsciiApp, seed: u64, hesitation: u16, verify_resume: bool) -> Report {
    let (mut world, entrance, exit) = build(app, seed);
    let mut contacts = BTreeSet::new();
    let mut mobile_contacts = BTreeSet::new();
    let mut returning = false;
    let mut first_contact = None;
    let mut reached_exit = false;
    let mut blocked = None;
    let mut minimum_hp = hp(&world);
    let mut idle = 0;
    let mut returned = false;
    let mut restored: Option<WorldState> = None;
    for _ in 0..600 {
        if world.status() != RunStatus::Active {
            break;
        }
        let at = world.player_position().unwrap();
        let visible: Vec<_> = world
            .actors()
            .iter()
            .filter(|(id, a)| {
                *id != world.player_id()
                    && a.player_relation() == project_rl::social::PlayerRelation::Hostile
                    && world.player_visibility().is_visible(a.position())
            })
            .map(|(id, a)| (id, a.position(), a.ai().map(|ai| ai.behavior)))
            .collect();
        for &(id, _, behavior) in &visible {
            contacts.insert(id);
            if matches!(
                behavior,
                Some(
                    AiBehavior::Hunter
                        | AiBehavior::PackHunter
                        | AiBehavior::Skirmisher
                        | AiBehavior::VibrationHunter { .. }
                )
            ) {
                mobile_contacts.insert(id);
            }
        }
        if !visible.is_empty() {
            first_contact.get_or_insert(world.turn());
        }
        reached_exit |= at == exit;
        returning |= reached_exit
            || first_contact.is_some_and(|t| world.turn() - t >= u64::from(hesitation));
        let warnings = danger(&world);
        let adjacent_exit = at.x.abs_diff(entrance.x) + at.y.abs_diff(entrance.y) <= 1;
        let heal = (hp(&world) <= 12)
            .then(|| {
                world
                    .player_inventory()
                    .iter()
                    .find(|e| e.item().as_str() == "core:repair_patch")
                    .map(|e| GameCommand::UseItem { item: e.instance() })
            })
            .flatten();
        let path = next_step(&world, if returning { entrance } else { exit }, &warnings);
        // A blocked route is not permission to attack a civilian. Only a
        // visible hostile already at melee range can be struck to disengage.
        let melee = visible
            .iter()
            .filter(|(_, p, _)| at.x.abs_diff(p.x).max(at.y.abs_diff(p.y)) <= 1)
            .min_by_key(|(id, _, _)| *id)
            .map(|&(target, _, _)| GameCommand::Attack { slot: 0, target });
        let command = if returning && adjacent_exit {
            GameCommand::Interact { target: entrance }
        } else if warnings.contains(&at) {
            path.clone().or(melee.clone()).unwrap_or(GameCommand::Wait)
        } else {
            heal.or(path).or(melee).unwrap_or(GameCommand::Wait)
        };
        if command == GameCommand::Wait {
            idle += 1;
        } else {
            idle = 0;
        }
        if idle >= 12 {
            blocked = Some("pilote sans chemin ni action pendant 12 tours".into());
            break;
        }
        let before = (hp(&world), ammo(&world), patches(&world));
        if verify_resume && returning && first_contact.is_some() && restored.is_none() {
            let bytes = world.recovery_snapshot_bytes().unwrap();
            restored = Some(
                WorldState::from_recovery_snapshot_bytes(&bytes, world.rules().clone()).unwrap(),
            );
        }
        let result = world.process_player_command(command.clone());
        let events = world.drain_events();
        if let Some(resumed) = &mut restored {
            assert_eq!(resumed.process_player_command(command.clone()), result);
            assert_eq!(
                resumed.drain_events(),
                events,
                "resume must not reroll combat or off-screen simulation"
            );
            assert_eq!(
                resumed.recovery_snapshot_bytes().unwrap(),
                world.recovery_snapshot_bytes().unwrap()
            );
        }
        if result != CommandOutcome::Applied {
            blocked = Some(format!("{command:?}: {result:?}"));
            break;
        }
        minimum_hp = minimum_hp.min(hp(&world));
        if world.current_zone().unwrap().id.as_str() == "test:retreat_refuge" {
            assert_eq!(
                (hp(&world), ammo(&world), patches(&world)),
                before,
                "return cannot refill resources"
            );
            returned = true;
            break;
        }
    }
    if verify_resume {
        assert!(
            restored.is_some(),
            "resume test must reach a real contact and retreat"
        );
    }
    if !returned && world.status() == RunStatus::Active && blocked.is_none() {
        blocked = Some("limite de 600 actions du pilote".into());
    }
    Report {
        seed,
        hesitation,
        returned,
        dead: world.status() == RunStatus::PlayerDestroyed,
        blocked,
        reached_exit,
        contacts: contacts.len(),
        mobile_contacts: mobile_contacts.len(),
        turns: world.turn(),
        minimum_hp,
        ammunition: ammo(&world),
        soins: patches(&world),
    }
}

#[test]
fn generated_survival_records_every_map_and_mobile_contact_without_retry() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
    let mut mobile = 0;
    for hesitation in [0, 3] {
        for seed in 0..8 {
            let report = probe(&app, seed, hesitation, false);
            println!(
                "seed={} delay={} returned={} dead={} blocked={:?} crossed={} contacts={} mobile={} turns={} min_hp={} ammo={} soins={}",
                report.seed,
                report.hesitation,
                report.returned,
                report.dead,
                report.blocked,
                report.reached_exit,
                report.contacts,
                report.mobile_contacts,
                report.turns,
                report.minimum_hp,
                report.ammunition,
                report.soins
            );
            mobile += report.mobile_contacts;
            assert!(report.returned || report.dead || report.blocked.is_some());
            assert!(report.ammunition <= 12 && report.soins <= 2);
            assert!(
                report.blocked.is_none(),
                "pilot failure on seed {seed} must be investigated, not counted as a safe return"
            );
            if hesitation == 0 {
                assert!(
                    report.returned && !report.dead,
                    "these eight initial-contact retreats must remain possible"
                );
            }
        }
    }
    assert!(
        mobile > 0,
        "the sample must actually encounter mobile enemies"
    );
}

#[test]
fn generated_survival_resume_during_pursuit_preserves_every_event_and_resource() {
    let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
    let app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
    // Includes a light wound and the more expensive two-care retreat. The
    // original and restored simulations receive identical commands, no retry.
    for seed in [1, 4] {
        let report = probe(&app, seed, 3, true);
        assert!(report.returned && report.blocked.is_none());
    }
}

#[test]
fn generated_survival_pilot_opens_doors_but_never_steps_into_a_warning_or_civilian() {
    let mut map = Map::from_ascii("#####\n#...#\n#####").unwrap();
    let door = GridPos::new(2, 1);
    let goal = GridPos::new(3, 1);
    map.set_terrain(door, Terrain::Door(DoorState::Closed))
        .unwrap();
    let mut world = WorldState::single(GameState::new(map, GridPos::new(1, 1), 0).unwrap());
    assert_eq!(
        next_step(&world, goal, &BTreeSet::new()),
        Some(GameCommand::Interact { target: door })
    );
    assert_eq!(
        world.process_player_command(GameCommand::Interact { target: door }),
        CommandOutcome::Applied
    );
    assert_eq!(
        next_step(&world, goal, &BTreeSet::new()),
        Some(GameCommand::Move(Direction::East))
    );
    assert_eq!(next_step(&world, goal, &BTreeSet::from([door])), None);
    world
        .spawn_actor(
            Actor::new(door, 10)
                .unwrap()
                .with_player_relation(project_rl::social::PlayerRelation::Neutral),
        )
        .unwrap();
    assert_eq!(next_step(&world, goal, &BTreeSet::new()), None);
}
