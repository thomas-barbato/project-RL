//! Authored native64 street, independent of campaign and loot RNG.
use project_rl::game::{WorldState, ZoneBlueprint, ZoneInfo};
use project_rl::world::{DoorState, GridPos, Map, Terrain};
use serde::Deserialize;
use std::sync::OnceLock;

pub const ZONE_ID: &str = "core:prison_street";
pub const NAME: &str = "Abords de la prison";

#[derive(Deserialize)]
pub struct Street {
    pub width: usize,
    pub height: usize,
    pub image_origin: [i32; 2],
    pub image_offset_px: [i32; 2],
    pub image_size: [i32; 2],
    pub background: String,
    pub entrance: [i32; 2],
    pub home_entry: [i32; 2],
    pub home_inside: [i32; 2],
    pub car_at: [i32; 2],
    pub car_file: String,
    pub car_size: [i32; 2],
    pub obstructions: Vec<[i32; 2]>,
    #[serde(default)]
    pub atmosphere: Option<crate::camp_atmosphere::Config>,
    #[serde(default)]
    pub roofs: Vec<Roof>,
    #[serde(default)]
    pub doorway: Option<Doorway>,
    #[serde(default)]
    pub extra_doorways: Vec<ExtraDoorway>,
    #[serde(default)]
    pub ambience: Option<Ambience>,
    #[serde(default)]
    pub scenery_props: Vec<SceneryProp>,
}

#[derive(Deserialize)]
pub struct SceneryProp {
    pub file: String,
    pub at_px: [i32; 2],
    #[serde(default)]
    pub interior_roof: Option<String>,
}

#[derive(Deserialize)]
pub struct ExtraDoorway {
    pub door: [i32; 2],
    #[serde(flatten)]
    pub patch: Doorway,
}

#[derive(Deserialize)]
pub struct Ambience {
    pub steam: [f32; 2],
    pub embers: bool,
}

#[derive(Deserialize)]
pub struct Doorway {
    pub floor_source: [f32; 4],
    pub opening_at: [f32; 2],
    pub lamp_source: [f32; 4],
    pub lamp_at: [f32; 2],
    pub closed_source: [f32; 4],
    pub closed_at: [f32; 2],
}

#[derive(Deserialize)]
pub struct Roof {
    #[serde(default)]
    pub id: String,
    pub file: String,
    pub at_px: [i32; 2],
    pub door: Option<[i32; 2]>,
    pub interior: Vec<[i32; 2]>,
    pub exterior: Vec<[i32; 2]>,
}

impl Roof {
    pub fn hidden(&self, game: &WorldState) -> bool {
        self.hidden_at(
            game.player_position(),
            self.door
                .and_then(|at| game.map().tile(pos(at)))
                .map(|t| t.terrain),
        )
    }

    fn hidden_at(&self, player: Option<GridPos>, door: Option<Terrain>) -> bool {
        self.interior.iter().any(|&p| Some(pos(p)) == player)
            // Generation 155 saves retain their original permanently open
            // floor entrance. Never close or mutate it during restoration.
            || (self.door.is_some()
                && matches!(door, Some(Terrain::Floor | Terrain::Door(DoorState::Open))))
    }

    pub fn exposure(
        &self,
        visible: impl Fn(GridPos) -> bool,
        known: impl Fn(GridPos) -> bool,
    ) -> Option<f32> {
        // Observing the exterior reveals only the opaque roof surface, never
        // the interior tiles, actors or items beneath it. No FOV mutation.
        if self.exterior.iter().any(|&p| visible(pos(p))) {
            Some(1.)
        } else if self.exterior.iter().any(|&p| known(pos(p))) {
            Some(0.28)
        } else {
            None
        }
    }
}

pub fn scene() -> &'static Street {
    static SCENE: OnceLock<Street> = OnceLock::new();
    SCENE.get_or_init(|| {
        serde_json::from_str(include_str!("../assets/maps/prison-street-01/street.json"))
            .expect("accepted street manifest")
    })
}

pub fn expanded_scene() -> &'static Street {
    static SCENE: OnceLock<Street> = OnceLock::new();
    SCENE.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../assets/maps/prison-street-expanded-01/street.json"
        ))
        .expect("integrated expanded street")
    })
}

pub fn scene_for_game(game: &WorldState) -> &'static Street {
    if let Some(scene) = crate::sewer::scene(game) {
        return scene;
    }
    if game.map().height() == expanded_scene().height {
        expanded_scene()
    } else {
        scene()
    }
}

#[cfg(debug_assertions)]
pub fn expansion_review() -> bool {
    std::env::args().nth(1).as_deref() == Some("--ui-cold-street-expansion")
}
pub fn pos(at: [i32; 2]) -> GridPos {
    GridPos::new(at[0], at[1])
}
pub fn info(mut template: ZoneInfo) -> ZoneInfo {
    template.id = ZONE_ID.parse().unwrap();
    template.name = NAME.into();
    template
}
pub fn blueprint(
    info: ZoneInfo,
    seed: u64,
    doors: bool,
    expanded: bool,
    repaired: bool,
    contact_repairs: bool,
    warehouse_clearance: bool,
) -> Result<ZoneBlueprint, String> {
    #[cfg(debug_assertions)]
    let expanded = expanded || expansion_review();
    let s = if expanded { expanded_scene() } else { scene() };
    let mut map = Map::filled(s.width, s.height, Terrain::Wall).map_err(|e| e.to_string())?;
    let layout = if expanded {
        include_str!("../assets/maps/prison-street-expanded-01/layout.txt")
    } else {
        include_str!("../assets/maps/prison-street-01/layout.txt")
    };
    let rows: Vec<_> = layout.lines().collect();
    if rows.len() != s.height || rows.iter().any(|r| r.len() != s.width) {
        return Err("Invalid authored street dimensions".into());
    }
    for (y, row) in rows.iter().enumerate() {
        for (x, ch) in row.bytes().enumerate() {
            if ch == b'.' {
                map.set_terrain(GridPos::new(x as i32, y as i32), Terrain::Floor)
                    .map_err(|e| e.to_string())?;
            }
        }
    }
    for &at in &s.obstructions {
        map.set_obstruction(pos(at), true, false)
            .map_err(|e| e.to_string())?;
    }
    if expanded {
        apply_room_vision_review(&mut map)?;
    }
    #[cfg(debug_assertions)]
    let repaired =
        repaired || std::env::args().nth(1).as_deref() == Some("--ui-cold-street-defects");
    if expanded && repaired {
        apply_street_collision_proposal(&mut map)?;
    }
    if expanded && contact_repairs {
        for cell in contact_repair_cells()? {
            map.set_terrain(cell, Terrain::Floor)
                .map_err(|e| e.to_string())?;
            map.set_obstruction(cell, false, false)
                .map_err(|e| e.to_string())?;
        }
    }
    if expanded && warehouse_clearance {
        for cell in warehouse_clearance_cells()? {
            map.set_terrain(cell, Terrain::Floor)
                .map_err(|e| e.to_string())?;
            map.set_obstruction(cell, false, false)
                .map_err(|e| e.to_string())?;
        }
    }
    if doors {
        for at in s.roofs.iter().filter_map(|roof| roof.door) {
            map.set_terrain(pos(at), Terrain::Door(DoorState::Closed))
                .map_err(|e| e.to_string())?;
        }
    }
    #[cfg(debug_assertions)]
    if expanded && crate::street_population::enabled() {
        for &at in &crate::street_defense_review::data().proposed_solid_cells {
            map.set_obstruction(pos(at), true, false)
                .map_err(|e| e.to_string())?;
        }
    }
    map.set_protected(pos(s.entrance), true)
        .map_err(|e| e.to_string())?;
    project_rl::world::generation::validate_interactive_map(
        &map,
        pos(s.entrance),
        pos(s.home_inside),
        &[pos(s.home_entry)],
        project_rl::world::generation::MapValidationRules::default(),
    )
    .map_err(|e| e.to_string())?;
    Ok(ZoneBlueprint {
        info,
        map,
        entrance: pos(s.entrance),
        seed: seed ^ 0x5530_3153_5452_4545,
        actors: Vec::new(),
        loot: Vec::new(),
        threat_sources: Vec::new(),
    })
}

pub fn collision_repair_cells() -> Result<Vec<GridPos>, String> {
    let review: serde_json::Value = serde_json::from_str(include_str!(
        "../assets/maps/prison-street-expanded-01/collision-repairs.json"
    ))
    .map_err(|e| e.to_string())?;
    let cells: Vec<[i32; 2]> =
        serde_json::from_value(review["cells"].clone()).map_err(|e| e.to_string())?;
    Ok(cells.into_iter().map(pos).collect())
}

pub fn contact_repair_cells() -> Result<Vec<GridPos>, String> {
    let review: serde_json::Value = serde_json::from_str(include_str!(
        "../assets/maps/prison-street-expanded-01/collision-repairs-r09.json"
    ))
    .map_err(|e| e.to_string())?;
    let cells: Vec<[i32; 2]> =
        serde_json::from_value(review["cells"].clone()).map_err(|e| e.to_string())?;
    Ok(cells.into_iter().map(pos).collect())
}

pub fn warehouse_clearance_cells() -> Result<Vec<GridPos>, String> {
    let review: serde_json::Value = serde_json::from_str(include_str!(
        "../assets/maps/prison-street-expanded-01/warehouse-clearance-r09.json"
    ))
    .map_err(|e| e.to_string())?;
    let cells: Vec<[i32; 2]> =
        serde_json::from_value(review["cells"].clone()).map_err(|e| e.to_string())?;
    Ok(cells.into_iter().map(pos).collect())
}

fn apply_street_collision_proposal(map: &mut Map) -> Result<(), String> {
    for cell in collision_repair_cells()? {
        map.set_terrain(cell, Terrain::Floor)
            .map_err(|e| e.to_string())?;
        map.set_obstruction(cell, false, false)
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

fn apply_room_vision_review(map: &mut Map) -> Result<(), String> {
    let review: serde_json::Value = serde_json::from_str(include_str!(
        "../assets/maps/prison-street-expanded-01/room-vision.json"
    ))
    .map_err(|e| e.to_string())?;
    let cells: Vec<[i32; 2]> =
        serde_json::from_value(review["cells"].clone()).map_err(|e| e.to_string())?;
    for cell in cells {
        let at = pos(cell);
        let blocked = !map.is_walkable(at);
        map.set_terrain(at, Terrain::Floor)
            .map_err(|e| e.to_string())?;
        map.set_obstruction(at, blocked, false)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn removed_warehouse_props_leave_clear_ground_without_opening_the_facade() {
        let info = ZoneInfo {
            id: ZONE_ID.parse().unwrap(),
            name: NAME.into(),
            kind: "core:street".parse().unwrap(),
            depth: 1,
        };
        let old = blueprint(info.clone(), 1874, true, true, true, true, false)
            .unwrap()
            .map;
        let map = blueprint(info, 1874, true, true, true, true, true)
            .unwrap()
            .map;
        assert!(!old.is_walkable(pos([11, 7])));
        for y in 7..=8 {
            for x in 9..=12 {
                assert!(map.is_walkable(pos([x, y])), "former props {x},{y}");
            }
        }
        let cleared = warehouse_clearance_cells().unwrap();
        for y in 0..66 {
            for x in 0..26 {
                let at = GridPos::new(x, y);
                assert_eq!(
                    old.is_walkable(at) != map.is_walkable(at),
                    cleared.contains(&at)
                );
            }
        }
        for x in 8..=12 {
            assert!(!map.is_walkable(pos([x, 6])), "facade {x},6");
        }
    }
    #[test]
    fn contact_repairs_open_reported_ground_and_keep_real_solids_and_doors() {
        let info = ZoneInfo {
            id: ZONE_ID.parse().unwrap(),
            name: NAME.into(),
            kind: "core:street".parse().unwrap(),
            depth: 1,
        };
        let old = blueprint(info.clone(), 1874, true, true, true, false, false)
            .unwrap()
            .map;
        let map = blueprint(info, 1874, true, true, true, true, false)
            .unwrap()
            .map;
        let review: serde_json::Value = serde_json::from_str(include_str!(
            "../assets/maps/prison-street-expanded-01/collision-repairs-r09.json"
        ))
        .unwrap();
        let clear: Vec<[i32; 2]> =
            serde_json::from_value(review["required_clear"].clone()).unwrap();
        let solid: Vec<[i32; 2]> =
            serde_json::from_value(review["required_solid"].clone()).unwrap();
        for at in clear {
            assert!(map.is_walkable(pos(at)), "clear {at:?}");
        }
        for at in solid {
            assert!(!map.is_walkable(pos(at)), "solid {at:?}");
        }
        let repairs = contact_repair_cells().unwrap();
        for y in 0..66 {
            for x in 0..26 {
                let at = GridPos::new(x, y);
                assert_eq!(
                    old.is_walkable(at) != map.is_walkable(at),
                    repairs.contains(&at),
                    "{at:?}"
                );
            }
        }
        for roof in &expanded_scene().roofs {
            if let Some(door) = roof.door {
                assert_eq!(
                    map.tile(pos(door)).unwrap().terrain,
                    Terrain::Door(DoorState::Closed)
                );
            }
        }
        project_rl::world::generation::validate_interactive_map(
            &map,
            pos(expanded_scene().entrance),
            GridPos::new(18, 1),
            &repairs,
            project_rl::world::generation::MapValidationRules::default(),
        )
        .unwrap();
    }
    #[test]
    fn proposed_street_passages_are_connected_and_leave_real_objects_blocked() {
        let info = ZoneInfo {
            id: ZONE_ID.parse().unwrap(),
            name: NAME.into(),
            kind: "core:street".parse().unwrap(),
            depth: 1,
        };
        let mut map = blueprint(info, 1874, true, true, false, false, false)
            .unwrap()
            .map;
        let before = map.clone();
        apply_street_collision_proposal(&mut map).unwrap();
        let review:serde_json::Value=serde_json::from_str(include_str!("../assets/source-art/first-sector-review-01/u01-corrections-r01/collision-proposal.json")).unwrap();
        let cells: Vec<[i32; 2]> = serde_json::from_value(review["cells"].clone()).unwrap();
        assert_eq!(cells.len(), 61);
        let required: Vec<_> = cells.iter().copied().map(pos).collect();
        for y in 0..66 {
            for x in 0..26 {
                let at = GridPos::new(x, y);
                assert_eq!(
                    before.is_walkable(at) != map.is_walkable(at),
                    required.contains(&at)
                );
            }
        }
        for at in [[10, 7], [11, 7], [12, 7], [8, 6], [0, 33], [25, 33]] {
            assert!(!map.is_walkable(pos(at)));
        }
        project_rl::world::generation::validate_interactive_map(
            &map,
            pos(expanded_scene().entrance),
            GridPos::new(18, 1),
            &required,
            project_rl::world::generation::MapValidationRules::default(),
        )
        .unwrap();
    }

    #[test]
    fn room_vision_review_opens_the_false_interior_wall_without_changing_movement() {
        let layout = include_str!(
            "../assets/source-art/first-sector-review-01/u01-expansion-r06/layout.txt"
        );
        let mut map = Map::filled(26, 66, Terrain::Wall).unwrap();
        for (y, row) in layout.lines().enumerate() {
            for (x, cell) in row.bytes().enumerate() {
                if cell == b'.' {
                    map.set_terrain(GridPos::new(x as i32, y as i32), Terrain::Floor)
                        .unwrap();
                }
            }
        }
        let before = map.clone();
        let origin = GridPos::new(12, 20);
        let behind_false_wall = GridPos::new(9, 20);
        let rules = project_rl::world::FieldOfViewRules::default();
        assert!(!project_rl::world::is_tile_visible(
            &map,
            origin,
            behind_false_wall,
            rules
        ));
        apply_room_vision_review(&mut map).unwrap();
        assert!(project_rl::world::is_tile_visible(
            &map,
            origin,
            behind_false_wall,
            rules
        ));
        for y in 0..66 {
            for x in 0..26 {
                let at = GridPos::new(x, y);
                assert_eq!(before.is_walkable(at), map.is_walkable(at));
                if before.blocks_vision(at) != map.blocks_vision(at) {
                    assert!((8..=14).contains(&x) && matches!(y, 19..=21 | 27..=29));
                }
            }
        }
        assert!(map.blocks_vision(GridPos::new(11, 18)));
        assert!(map.blocks_vision(GridPos::new(12, 19)));
    }

    #[test]
    fn north_candidate_preserves_departure_and_connects_rooms_with_sealed_edges() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("assets/source-art/first-sector-review-01/u01-expansion-r06");
        let candidate: Street =
            serde_json::from_str(&std::fs::read_to_string(root.join("street.json")).unwrap())
                .unwrap();
        let old = scene();
        assert_eq!(candidate.width, old.width);
        assert_eq!(candidate.height - old.height, 48);
        assert_eq!(candidate.entrance, [old.entrance[0], old.entrance[1] + 48]);
        assert_eq!(
            candidate.home_entry,
            [old.home_entry[0], old.home_entry[1] + 48]
        );
        assert_eq!(candidate.car_at, [old.car_at[0], old.car_at[1] + 48]);
        let mut map = Map::filled(candidate.width, candidate.height, Terrain::Wall).unwrap();
        for (y, row) in std::fs::read_to_string(root.join("layout.txt"))
            .unwrap()
            .lines()
            .enumerate()
        {
            for (x, ch) in row.bytes().enumerate() {
                if ch == b'.' {
                    assert!(
                        y <= candidate.entrance[1] as usize,
                        "no exterior floor behind the prison departure"
                    );
                    map.set_terrain(GridPos::new(x as i32, y as i32), Terrain::Floor)
                        .unwrap();
                }
            }
        }
        for roof in &candidate.roofs {
            if let Some(door) = roof.door {
                map.set_terrain(pos(door), Terrain::Door(DoorState::Closed))
                    .unwrap();
            }
        }
        for &at in &candidate.obstructions {
            map.set_obstruction(pos(at), true, false).unwrap();
        }
        let mut required = vec![pos(candidate.home_entry), pos(candidate.home_inside)];
        for doorway in &candidate.extra_doorways {
            assert!(
                doorway.door[1] < 48,
                "new houses belong north of the original street"
            );
            required.push(pos(doorway.door));
        }
        project_rl::world::generation::validate_interactive_map(
            &map,
            pos(candidate.entrance),
            GridPos::new(18, 1),
            &required,
            project_rl::world::generation::MapValidationRules::default(),
        )
        .unwrap();
    }

    #[test]
    fn roof_follows_door_and_player_without_saved_state() {
        let roof = &scene().roofs[0];
        let outside = Some(GridPos::new(4, 6));
        let inside = Some(pos(scene().home_inside));
        assert!(!roof.hidden_at(outside, Some(Terrain::Door(DoorState::Closed))));
        assert!(roof.hidden_at(outside, Some(Terrain::Door(DoorState::Open))));
        assert!(roof.hidden_at(inside, Some(Terrain::Door(DoorState::Closed))));
        assert!(roof.hidden_at(outside, Some(Terrain::Floor)));
        assert!(!scene().roofs[1].hidden_at(outside, None));
    }

    #[test]
    fn exterior_roof_does_not_reveal_unknown_buildings_or_depend_on_interior_sight() {
        let roof = &scene().roofs[0];
        assert_eq!(roof.exposure(|_| false, |_| false), None);
        let wall = pos(roof.exterior[0]);
        assert_eq!(roof.exposure(|p| p == wall, |_| false), Some(1.));
        assert_eq!(roof.exposure(|_| false, |p| p == wall), Some(0.28));
        let inside = pos(scene().home_inside);
        assert_eq!(roof.exposure(|p| p == inside, |_| false), None);
    }

    #[test]
    fn doorway_light_is_off_the_opening_and_matches_its_real_emitter() {
        let s = scene();
        let doorway = s.doorway.as_ref().unwrap();
        let opening = macroquad::prelude::Rect::new(
            doorway.opening_at[0],
            doorway.opening_at[1],
            doorway.floor_source[2],
            doorway.floor_source[3],
        );
        let lamp = macroquad::prelude::Rect::new(
            doorway.lamp_at[0],
            doorway.lamp_at[1],
            doorway.lamp_source[2],
            doorway.lamp_source[3],
        );
        assert!(
            !opening.overlaps(&lamp),
            "Lamp must be fixed outside the walkable opening"
        );
        let config: serde_json::Value =
            serde_json::from_str(include_str!("../assets/maps/prison-street-01/street.json"))
                .unwrap();
        let lights: Vec<_> = config["atmosphere"]["lights"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|l| l["id"] == "home-door")
            .collect();
        assert_eq!(
            lights.len(),
            1,
            "No residual emitter at the former door position"
        );
        for (axis, lens) in [190., 307.].into_iter().enumerate() {
            let expected = s.image_origin[axis] as f32 * 64.
                + s.image_offset_px[axis] as f32
                + doorway.lamp_at[axis]
                + lens
                - doorway.lamp_source[axis];
            assert_eq!(lights[0]["at"][axis].as_f64().unwrap() as f32, expected);
        }
    }
}

/// Static low obstacles use the same recovery contract as the camp furniture.
pub fn restore_obstructions(world: &mut WorldState) -> Result<(), String> {
    let zone = ZONE_ID.parse().unwrap();
    // The reciprocal link is created atomically with the actual map. Metadata
    // alone also exists while the camp's street destination is still deferred.
    let s = if world
        .passage_in(&zone, pos(expanded_scene().entrance))
        .is_some()
    {
        expanded_scene()
    } else if world.passage_in(&zone, pos(scene().entrance)).is_some() {
        scene()
    } else {
        return Ok(());
    };
    let obstacles: Vec<_> = s
        .obstructions
        .iter()
        .map(|&at| (pos(at), true, false))
        .collect();
    #[cfg(debug_assertions)]
    let obstacles = if s.height == expanded_scene().height && crate::street_population::enabled() {
        // This fresh trial uses every accepted clearance repair. Rebuild the
        // derived layer from that exact blueprint, not the older raw manifest.
        // Door states and all other saved terrain remain in the snapshot.
        let template = world
            .zone_info(&zone)
            .ok_or("Trial street metadata absent")?
            .clone();
        let map = blueprint(template, 0, true, true, true, true, true)?.map;
        (0..map.height() as i32)
            .flat_map(|y| (0..map.width() as i32).map(move |x| GridPos::new(x, y)))
            .map(|at| {
                let (movement, vision) = map.obstruction(at);
                (at, movement, vision)
            })
            .collect()
    } else {
        obstacles
    };
    world.restore_static_zone_obstructions(&zone, &obstacles)
}
