//! CITY-NORTH01: real engine state, isolated from campaign creation and saves.
use super::scene::{Document, Scene};
use project_rl::{
    content::ContentId,
    game::{CommandOutcome, GameCommand, GameState, WorldState, ZoneBlueprint, ZoneInfo},
    world::{Direction, DoorState, GridPos, Map, Terrain},
};
use serde::Deserialize;
use std::collections::{BTreeMap, VecDeque};

pub const STREET: &str = "core:prison_street";
pub const CITY: &str = "review:north_city";

#[derive(Deserialize)]
pub struct Review {
    pub review: String,
    pub zones: Vec<Zone>,
    pub street_links: Vec<StreetLink>,
    pub links: Vec<Link>,
    pub future_exit: FutureExit,
    #[serde(default)]
    pub game_seed: u64,
}
#[derive(Deserialize)]
pub struct Zone {
    pub id: String,
    pub name: String,
    pub file: String,
    pub buildings: Vec<Building>,
    pub scenery: Vec<Scenery>,
    pub views: Vec<View>,
    pub sealed: Vec<[i32; 2]>,
    #[serde(default)]
    pub variants: Vec<Variant>,
    #[serde(default)]
    pub variant_salt: u64,
    #[serde(default)]
    pub selected_variant: Option<u8>,
    #[serde(default)]
    pub road_markings: Vec<RoadMarking>,
    #[serde(default)]
    pub rooms: Vec<Room>,
    #[serde(default)]
    pub fabric: Vec<Fabric>,
}
#[derive(Deserialize)]
pub struct Fabric {
    pub id: String,
    pub cells: Vec<[i32; 2]>,
}
#[derive(Deserialize)]
pub struct RoadMarking {
    pub at: [i32; 2],
    pub horizontal: bool,
}
#[derive(Deserialize)]
pub struct Room {
    pub id: String,
    pub building: String,
    pub rect: [i32; 4],
    pub material: usize,
}
#[derive(Deserialize)]
pub struct Variant {
    pub number: u8,
    pub file: String,
    pub scenery: Vec<Scenery>,
    pub views: Vec<View>,
    pub sealed: Vec<[i32; 2]>,
    #[serde(default)]
    pub rooms: Vec<Room>,
}
#[derive(Deserialize)]
pub struct Building {
    pub id: String,
    pub name: String,
    pub rect: [i32; 4],
    pub doors: Vec<[i32; 2]>,
    pub accessible: bool,
    pub roof_floor: usize,
}
impl Building {
    pub fn contains(&self, at: GridPos) -> bool {
        let [l, t, r, b] = self.rect;
        at.x > l && at.x < r && at.y > t && at.y < b
    }
    pub fn open(&self, world: &WorldState) -> bool {
        self.accessible
            && (world.player_position().is_some_and(|at| self.contains(at))
                || self.doors.iter().any(|&at| {
                    world
                        .map()
                        .tile(pos(at))
                        .is_some_and(|t| t.terrain == Terrain::Door(DoorState::Open))
                }))
    }
}
#[derive(Deserialize)]
pub struct Scenery {
    pub file: String,
    pub at_px: [i32; 2],
    pub building: Option<String>,
}
#[derive(Deserialize)]
pub struct View {
    pub id: String,
    pub at: [i32; 2],
}
#[derive(Deserialize)]
pub struct StreetLink {
    pub at: [i32; 2],
    pub arrival: [i32; 2],
}
#[derive(Deserialize)]
pub struct Link {
    pub source: String,
    pub at: [i32; 2],
    pub destination: String,
    pub arrival: [i32; 2],
}
#[derive(Deserialize)]
pub struct FutureExit {
    pub zone: String,
    pub at: [i32; 2],
    pub target: String,
}

pub fn pos(p: [i32; 2]) -> GridPos {
    GridPos::new(p[0], p[1])
}
pub fn id(value: &str) -> ContentId {
    value.parse().expect("review content ID")
}
fn info(value: &str, name: &str) -> ZoneInfo {
    ZoneInfo {
        id: id(value),
        name: name.into(),
        kind: id("core:street"),
        depth: 1,
    }
}
/// Stable local hash, independent from loot/combat RNG and visit ordering.
/// Variant choice happens once, before this game's zone blueprints exist.
fn variant_index(seed: u64, salt: u64, count: usize) -> usize {
    let mut value = seed ^ salt;
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    ((value ^ (value >> 31)) % count as u64) as usize
}
pub fn load(seed: u64) -> Result<(Review, Vec<Scene>), String> {
    let bytes = project_rl::data_paths::read("assets/maps/north-city-01/review.json")?;
    let mut review: Review = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
    review.game_seed = seed;
    let mut resolved = BTreeMap::new();
    for zone in &mut review.zones {
        if zone.variants.is_empty() {
            continue;
        }
        let selected =
            zone.variants
                .remove(variant_index(seed, zone.variant_salt, zone.variants.len()));
        let previous = zone.id.clone();
        zone.id = format!("{}_layout{:02}", previous, selected.number);
        zone.file = selected.file;
        zone.scenery = selected.scenery;
        zone.views = selected.views;
        zone.sealed = selected.sealed;
        zone.rooms = selected.rooms;
        zone.selected_variant = Some(selected.number);
        resolved.insert(previous, zone.id.clone());
    }
    for link in &mut review.links {
        if let Some(value) = resolved.get(&link.source) {
            link.source = value.clone();
        }
        if let Some(value) = resolved.get(&link.destination) {
            link.destination = value.clone();
        }
    }
    let scenes = review
        .zones
        .iter()
        .map(|z| {
            let bytes = project_rl::data_paths::read(&z.file)?;
            let doc: Document = serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
            Scene::from_document(doc)
        })
        .collect::<Result<_, _>>()?;
    Ok((review, scenes))
}

/// Mirrors the accepted street's immutable terrain repairs in this fixture.
/// Its population, campaign state, generation version and files are untouched.
fn street_map() -> Result<Map, String> {
    let source: serde_json::Value = serde_json::from_str(include_str!(
        "../../assets/maps/prison-street-expanded-01/street.json"
    ))
    .map_err(|e| e.to_string())?;
    let mut map = Map::filled(26, 66, Terrain::Wall).map_err(|e| e.to_string())?;
    for (y, row) in include_str!("../../assets/maps/prison-street-expanded-01/layout.txt")
        .lines()
        .enumerate()
    {
        for (x, ch) in row.bytes().enumerate() {
            if ch == b'.' {
                map.set_terrain(GridPos::new(x as i32, y as i32), Terrain::Floor)
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    for value in source["obstructions"]
        .as_array()
        .ok_or("street obstructions missing")?
    {
        let p: [i32; 2] = serde_json::from_value(value.clone()).map_err(|e| e.to_string())?;
        map.set_obstruction(pos(p), true, false)
            .map_err(|e| e.to_string())?;
    }
    let courtyard: Vec<[i32; 2]> = serde_json::from_value(source["courtyard_clearance"].clone())
        .map_err(|e| e.to_string())?;
    for cell in courtyard {
        map.set_obstruction(pos(cell), false, false)
            .map_err(|e| e.to_string())?;
    }
    for text in [
        include_str!("../../assets/maps/prison-street-expanded-01/collision-repairs.json"),
        include_str!("../../assets/maps/prison-street-expanded-01/collision-repairs-r09.json"),
        include_str!("../../assets/maps/prison-street-expanded-01/warehouse-clearance-r09.json"),
    ] {
        let repair: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
        for cell in repair["cells"].as_array().ok_or("repair cells missing")? {
            let p: [i32; 2] = serde_json::from_value(cell.clone()).map_err(|e| e.to_string())?;
            map.set_terrain(pos(p), Terrain::Floor)
                .map_err(|e| e.to_string())?;
            map.set_obstruction(pos(p), false, false)
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(map)
}

pub fn world(review: &Review, scenes: &[Scene]) -> Result<WorldState, String> {
    let mut world = WorldState::single(
        GameState::new(street_map()?, pos([18, 3]), review.game_seed).map_err(|e| e.to_string())?,
    );
    world.enable(info(STREET, "Abords de la prison — raccord nord"))?;
    for (zone, scene) in review.zones.iter().zip(scenes) {
        world.add_zone(ZoneBlueprint {
            info: info(&zone.id, &zone.name),
            map: scene.game.map().clone(),
            entrance: scene.document.spawn,
            seed: review.game_seed,
            actors: vec![],
            loot: vec![],
            threat_sources: vec![],
        })?;
    }
    for link in &review.street_links {
        world.connect_optional(id(STREET), pos(link.at), id(CITY), pos(link.arrival))?;
    }
    for link in &review.links {
        world.connect(
            id(&link.source),
            pos(link.at),
            id(&link.destination),
            pos(link.arrival),
        )?;
    }
    Ok(world)
}

pub fn zone_index(review: &Review, world: &WorldState) -> Option<usize> {
    review
        .zones
        .iter()
        .position(|z| Some(&id(&z.id)) == world.current_zone().map(|z| &z.id))
}

/// Returns cardinal engine commands; doors are opened through Interact.
pub fn route(world: &WorldState, target: GridPos) -> Result<Vec<Direction>, String> {
    let start = world.player_position().ok_or("player missing")?;
    let mut seen = BTreeMap::from([(start, (start, Direction::North))]);
    let mut queue = VecDeque::from([start]);
    while let Some(at) = queue.pop_front() {
        if at == target {
            break;
        }
        for (next, direction) in [
            (GridPos::new(at.x, at.y - 1), Direction::North),
            (GridPos::new(at.x + 1, at.y), Direction::East),
            (GridPos::new(at.x, at.y + 1), Direction::South),
            (GridPos::new(at.x - 1, at.y), Direction::West),
        ] {
            let traversable = world.map().is_walkable(next)
                || (world
                    .map()
                    .tile(next)
                    .is_some_and(|t| t.terrain == Terrain::Door(DoorState::Closed))
                    && !world.map().obstruction(next).0);
            if traversable && !seen.contains_key(&next) {
                seen.insert(next, (at, direction));
                queue.push_back(next);
            }
        }
    }
    let mut at = target;
    let mut result = vec![];
    while at != start {
        let &(previous, direction) = seen.get(&at).ok_or("Destination inaccessible.")?;
        result.push(direction);
        at = previous;
    }
    result.reverse();
    Ok(result)
}
pub fn step(world: &mut WorldState, direction: Direction) -> Result<(), String> {
    let at = world.player_position().ok_or("player missing")?;
    let (dx, dy) = match direction {
        Direction::North => (0, -1),
        Direction::East => (1, 0),
        Direction::South => (0, 1),
        Direction::West => (-1, 0),
    };
    let target = GridPos::new(at.x + dx, at.y + dy);
    if world
        .map()
        .tile(target)
        .is_some_and(|t| t.terrain == Terrain::Door(DoorState::Closed))
    {
        apply(world, GameCommand::Interact { target })?;
    }
    apply(world, GameCommand::Move(direction))
}
pub fn walk_to(world: &mut WorldState, at: GridPos) -> Result<(), String> {
    for direction in route(world, at)? {
        step(world, direction)?;
    }
    Ok(())
}
pub fn apply(world: &mut WorldState, command: GameCommand) -> Result<(), String> {
    match world.process_player_command(command) {
        CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime => {
            world.drain_events();
            Ok(())
        }
        other => Err(format!("Review command rejected: {other:?}")),
    }
}
pub fn enter_city(world: &mut WorldState) -> Result<(), String> {
    walk_to(world, pos([18, 2]))?;
    apply(
        world,
        GameCommand::Interact {
            target: pos([18, 2]),
        },
    )
}
pub fn travel_to(review: &Review, world: &mut WorldState, destination: &str) -> Result<(), String> {
    let source = world
        .current_zone()
        .ok_or("current zone missing")?
        .id
        .to_string();
    let mut seen = BTreeMap::from([(source.clone(), (String::new(), pos([0, 0])))]);
    let mut queue = VecDeque::from([source.clone()]);
    while let Some(at) = queue.pop_front() {
        if at == destination {
            break;
        }
        for link in &review.links {
            let next = if link.source == at {
                Some((&link.destination, pos(link.at)))
            } else if link.destination == at {
                Some((&link.source, pos(link.arrival)))
            } else {
                None
            };
            if let Some((next, anchor)) = next {
                if !seen.contains_key(next) {
                    seen.insert(next.clone(), (at.clone(), anchor));
                    queue.push_back(next.clone());
                }
            }
        }
    }
    let mut route = vec![];
    let mut at = destination.to_owned();
    while at != source {
        let (previous, anchor) = seen.get(&at).ok_or("No review zone route")?;
        route.push(*anchor);
        at = previous.clone();
    }
    for anchor in route.into_iter().rev() {
        walk_to(world, anchor)?;
        apply(world, GameCommand::Interact { target: anchor })?;
    }
    Ok(())
}
pub fn restore_obstructions(
    world: &mut WorldState,
    review: &Review,
    scenes: &[Scene],
) -> Result<(), String> {
    for (zone, scene) in review.zones.iter().zip(scenes) {
        let mut obstacles = vec![];
        for y in 0..scene.document.height {
            for x in 0..scene.document.width {
                let at = GridPos::new(x, y);
                let (movement, vision) = scene.game.map().obstruction(at);
                if movement || vision {
                    obstacles.push((at, movement, vision));
                }
            }
        }
        world.restore_static_zone_obstructions(&id(&zone.id), &obstacles)?;
    }
    let map = street_map()?;
    let mut obstacles = vec![];
    for y in 0..map.height() {
        for x in 0..map.width() {
            let at = GridPos::new(x as i32, y as i32);
            let (movement, vision) = map.obstruction(at);
            if movement || vision {
                obstacles.push((at, movement, vision));
            }
        }
    }
    world.restore_static_zone_obstructions(&id(STREET), &obstacles)
}

#[cfg(test)]
mod north_city_tests {
    use super::*;
    #[test]
    fn exact_north_links_and_every_authored_destination_are_reachable() {
        let (review, scenes) = load(20261105).unwrap();
        let mut w = world(&review, &scenes).unwrap();
        for link in &review.street_links {
            walk_to(&mut w, pos(link.at)).unwrap();
            apply(
                &mut w,
                GameCommand::Interact {
                    target: pos(link.at),
                },
            )
            .unwrap();
            assert_eq!(w.player_position(), Some(pos(link.arrival)));
            apply(
                &mut w,
                GameCommand::Interact {
                    target: pos(link.arrival),
                },
            )
            .unwrap();
            assert_eq!(w.player_position(), Some(pos(link.at)));
        }
        enter_city(&mut w).unwrap();
        assert_eq!((w.map().width(), w.map().height()), (112, 112));
        for view in &review.zones[0].views {
            walk_to(&mut w, pos(view.at)).unwrap();
        }
        walk_to(&mut w, pos(review.future_exit.at)).unwrap();
        assert!(w.passage(pos(review.future_exit.at)).is_none());
    }
    #[test]
    fn optional_police_floors_round_trip_and_keep_door_state_after_snapshot() {
        let (review, scenes) = load(20261105).unwrap();
        let mut w = world(&review, &scenes).unwrap();
        enter_city(&mut w).unwrap();
        for link in &review.links {
            travel_to(&review, &mut w, &link.source).unwrap();
            walk_to(&mut w, pos(link.at)).unwrap();
            apply(
                &mut w,
                GameCommand::Interact {
                    target: pos(link.at),
                },
            )
            .unwrap();
            assert_eq!(w.current_zone().unwrap().id, id(&link.destination));
            for view in &review
                .zones
                .iter()
                .find(|z| z.id == link.destination)
                .unwrap()
                .views
            {
                walk_to(&mut w, pos(view.at)).unwrap();
            }
            walk_to(&mut w, pos(link.arrival)).unwrap();
            apply(
                &mut w,
                GameCommand::Interact {
                    target: pos(link.arrival),
                },
            )
            .unwrap();
        }
        travel_to(&review, &mut w, CITY).unwrap();
        let door = pos(review.zones[0]
            .buildings
            .iter()
            .find(|b| b.id == "police")
            .unwrap()
            .doors[0]);
        assert_eq!(
            w.map().tile(door).unwrap().terrain,
            Terrain::Door(DoorState::Open)
        );
        let bytes = w.recovery_snapshot_bytes().unwrap();
        let mut restored = WorldState::from_recovery_snapshot_bytes(
            &bytes,
            project_rl::game::GameRules::default(),
        )
        .unwrap();
        restore_obstructions(&mut restored, &review, &scenes).unwrap();
        assert!(
            restored.map() == w.map(),
            "restored terrain and static obstacles must match"
        );
        assert_eq!(
            restored.map().tile(door).unwrap().terrain,
            Terrain::Door(DoorState::Open)
        );
        // The main route bypasses the optional dungeon completely.
        walk_to(&mut restored, pos(review.future_exit.at)).unwrap();
    }
    #[test]
    fn basement_choices_are_seeded_at_creation_and_persist_before_the_first_visit() {
        let (a, scenes) = load(20261105).unwrap();
        let (b, _) = load(20261105).unwrap();
        let choices = |r: &Review| {
            r.zones
                .iter()
                .filter_map(|z| z.selected_variant.map(|n| (z.id.clone(), n)))
                .collect::<Vec<_>>()
        };
        assert_eq!(choices(&a), choices(&b));
        assert_eq!(choices(&a).len(), 2);
        let w = world(&a, &scenes).unwrap();
        let bytes = w.recovery_snapshot_bytes().unwrap();
        let restored = WorldState::from_recovery_snapshot_bytes(
            &bytes,
            project_rl::game::GameRules::default(),
        )
        .unwrap();
        for (zone, _) in choices(&a) {
            assert!(
                restored.zone_info(&id(&zone)).is_some(),
                "choice must be saved before first visit"
            );
        }
        let mut selected = [
            std::collections::BTreeSet::new(),
            std::collections::BTreeSet::new(),
        ];
        for seed in 1..=48 {
            let (review, scenes) = load(seed).unwrap();
            // Engine validation checks every selected floor and all passages.
            world(&review, &scenes).unwrap();
            for (index, (_, choice)) in choices(&review).iter().enumerate() {
                selected[index].insert(*choice);
            }
        }
        assert!(
            selected
                .iter()
                .all(|s| s == &std::collections::BTreeSet::from([1, 2, 3]))
        );
    }
    #[test]
    fn furnished_rooms_have_distinct_materials_and_accessible_interiors() {
        let (review, scenes) = load(20261105).unwrap();
        let mut world = world(&review, &scenes).unwrap();
        enter_city(&mut world).unwrap();
        for building in review.zones[0].buildings.iter().filter(|b| b.accessible) {
            let rooms = review.zones[0]
                .rooms
                .iter()
                .filter(|r| r.building == building.id)
                .collect::<Vec<_>>();
            assert!(rooms.len() >= 3, "{} needs functional rooms", building.id);
            let materials = rooms
                .iter()
                .map(|r| r.material)
                .collect::<std::collections::BTreeSet<_>>();
            assert!(
                materials.len() >= 2,
                "{} needs appropriate distinct surfaces",
                building.id
            );
            for room in rooms {
                let [l, t, r, b] = room.rect;
                let at = (t..=b)
                    .flat_map(|y| (l..=r).map(move |x| GridPos::new(x, y)))
                    .find(|p| world.map().is_walkable(*p))
                    .expect("room with usable floor");
                walk_to(&mut world, at).unwrap_or_else(|e| panic!("{}: {e}", room.id));
            }
        }
    }
}
