use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::content::{RegionCityLayout, RegionMapSize};
use crate::world::{Direction, GridPos, Map, MapBuildError, Terrain};

use super::{MapValidationError, MapValidationRules, cardinal_passage, validate_playable_map};

/// Semantic fixtures used by the Terminal renderer. They never carry gameplay
/// meaning: collision and visibility remain encoded in the generated `Map`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionalCityFeature {
    Deck,
    CorrodedDeck,
    Grate,
    Lane,
    Threshold,
    CoolantChannel,
    ForeignFloor,
    VeinedFloor,
    ChitinFloor,
    PulseChannel,
    MemoryFloor,
    WindowFrame,
    FaultTrace,
    Wall,
    MembraneWall,
    VoidWall,
    DeadScreen,
    Pillar,
    Crate,
    Server,
    Console,
    Coolant,
    Resonator,
    GrowthNode,
    EyeNode,
    RootMass,
    KernelFault,
    OrphanProcess,
    ClinicBed,
    ClinicCounter,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedRegionalCityMap {
    map: Map,
    features: BTreeMap<GridPos, RegionalCityFeature>,
    passages: [GridPos; 4],
}

impl GeneratedRegionalCityMap {
    pub const fn map(&self) -> &Map {
        &self.map
    }

    pub fn features(&self) -> &BTreeMap<GridPos, RegionalCityFeature> {
        &self.features
    }

    pub const fn passages(&self) -> [GridPos; 4] {
        self.passages
    }

    pub fn into_parts(self) -> (Map, BTreeMap<GridPos, RegionalCityFeature>, [GridPos; 4]) {
        (self.map, self.features, self.passages)
    }
}

pub fn generate_regional_city(
    size: RegionMapSize,
    layout: RegionCityLayout,
) -> Result<GeneratedRegionalCityMap, RegionalCityGenerationError> {
    if !layout.supports(size) {
        return Err(RegionalCityGenerationError::LayoutTooSmall);
    }
    match layout {
        RegionCityLayout::MaintenanceSpine => generate_maintenance_spine(size),
        RegionCityLayout::CoolantRings => generate_coolant_rings(size),
        RegionCityLayout::DissonantLattice => generate_dissonant_lattice(size),
        RegionCityLayout::RecursiveBloom => generate_recursive_bloom(size),
        RegionCityLayout::ProcessRuin => generate_process_ruin(size),
    }
}

fn generate_maintenance_spine(
    size: RegionMapSize,
) -> Result<GeneratedRegionalCityMap, RegionalCityGenerationError> {
    let width = i32::from(size.width());
    let height = i32::from(size.height());
    let center_x = width / 2;
    let center_y = height / 2;
    let mut map = Map::filled(width as usize, height as usize, Terrain::Wall)?;
    let mut features = BTreeMap::new();
    for y in 0..height {
        for x in 0..width {
            features.insert(GridPos::new(x, y), RegionalCityFeature::Wall);
        }
    }

    // A narrow vertical load-bearing spine makes the settlement read as an
    // inhabited machine rather than another open procedural wilderness.
    carve_rect(
        &mut map,
        &mut features,
        center_x - 3,
        1,
        center_x + 3,
        height - 2,
        RegionalCityFeature::Grate,
    )?;
    for y in [height / 5, center_y, height - height / 5 - 1] {
        carve_rect(
            &mut map,
            &mut features,
            1,
            y - 1,
            width - 2,
            y + 1,
            RegionalCityFeature::Lane,
        )?;
    }

    // Distinct functional bays flank the spine. Their proportions derive from
    // the map size, while authored NPC anchors decide who inhabits them.
    carve_rect(
        &mut map,
        &mut features,
        center_x - 21,
        center_y - 8,
        center_x - 4,
        center_y + 7,
        RegionalCityFeature::Deck,
    )?;
    carve_rect(
        &mut map,
        &mut features,
        center_x + 4,
        center_y - 8,
        center_x + 21,
        center_y + 7,
        RegionalCityFeature::Deck,
    )?;
    carve_rect(
        &mut map,
        &mut features,
        center_x - 24,
        height / 5 - 5,
        center_x - 4,
        height / 5 + 5,
        RegionalCityFeature::Deck,
    )?;
    carve_rect(
        &mut map,
        &mut features,
        center_x + 4,
        height - height / 5 - 6,
        center_x + 27,
        height - height / 5 + 4,
        RegionalCityFeature::Deck,
    )?;
    carve_rect(
        &mut map,
        &mut features,
        6,
        center_y - 4,
        center_x - 25,
        center_y + 4,
        RegionalCityFeature::Deck,
    )?;
    carve_rect(
        &mut map,
        &mut features,
        center_x + 22,
        center_y - 4,
        width - 7,
        center_y + 4,
        RegionalCityFeature::Deck,
    )?;

    // Obstacles communicate function visually and break long firing lanes,
    // while the connectivity validator prevents decorative dead islands.
    for (position, feature) in [
        (
            GridPos::new(center_x - 18, center_y - 6),
            RegionalCityFeature::Crate,
        ),
        (
            GridPos::new(center_x - 18, center_y - 5),
            RegionalCityFeature::Crate,
        ),
        (
            GridPos::new(center_x - 7, center_y - 6),
            RegionalCityFeature::Server,
        ),
        (
            GridPos::new(center_x - 7, center_y + 5),
            RegionalCityFeature::Console,
        ),
        (
            GridPos::new(center_x + 7, center_y - 5),
            RegionalCityFeature::ClinicCounter,
        ),
        (
            GridPos::new(center_x + 18, center_y - 5),
            RegionalCityFeature::ClinicBed,
        ),
        (
            GridPos::new(center_x + 18, center_y - 2),
            RegionalCityFeature::ClinicBed,
        ),
        (
            GridPos::new(center_x + 18, center_y + 4),
            RegionalCityFeature::ClinicBed,
        ),
        (GridPos::new(9, center_y - 2), RegionalCityFeature::Coolant),
        (GridPos::new(12, center_y + 2), RegionalCityFeature::Coolant),
        (
            GridPos::new(width - 10, center_y - 2),
            RegionalCityFeature::Server,
        ),
        (
            GridPos::new(width - 13, center_y + 2),
            RegionalCityFeature::Server,
        ),
    ] {
        place_obstacle(&mut map, &mut features, position, feature)?;
    }
    for position in [
        GridPos::new(center_x - 22, height / 5 - 3),
        GridPos::new(center_x - 8, height / 5 + 3),
        GridPos::new(center_x + 8, height - height / 5 - 4),
        GridPos::new(center_x + 23, height - height / 5 + 2),
    ] {
        place_obstacle(
            &mut map,
            &mut features,
            position,
            RegionalCityFeature::Pillar,
        )?;
    }

    let passages = [
        cardinal_passage(size, Direction::North),
        cardinal_passage(size, Direction::East),
        cardinal_passage(size, Direction::South),
        cardinal_passage(size, Direction::West),
    ];
    validate_playable_map(
        &map,
        passages[0],
        passages[2],
        &[passages[1], passages[3]],
        MapValidationRules::default(),
    )?;
    for y in 0..height {
        for x in 0..width {
            let position = GridPos::new(x, y);
            if map.is_walkable(position) {
                map.set_protected(position, true)?;
            }
        }
    }
    Ok(GeneratedRegionalCityMap {
        map,
        features,
        passages,
    })
}

fn generate_coolant_rings(
    size: RegionMapSize,
) -> Result<GeneratedRegionalCityMap, RegionalCityGenerationError> {
    let width = i32::from(size.width());
    let height = i32::from(size.height());
    let center_x = width / 2;
    let center_y = height / 2;
    let mut map = Map::filled(width as usize, height as usize, Terrain::Wall)?;
    let mut features = BTreeMap::new();
    for y in 0..height {
        for x in 0..width {
            features.insert(GridPos::new(x, y), RegionalCityFeature::Wall);
        }
    }

    // Two inhabited rectangular rings and four radial galleries give this
    // layer a compact, recursive silhouette instead of reusing the long spine
    // of the maintenance settlement above it.
    carve_frame(
        &mut map,
        &mut features,
        [3, 4, width - 4, height - 5],
        5,
        RegionalCityFeature::CorrodedDeck,
    )?;
    carve_frame(
        &mut map,
        &mut features,
        [15, 17, width - 16, height - 18],
        4,
        RegionalCityFeature::Grate,
    )?;
    carve_rect(
        &mut map,
        &mut features,
        center_x - 13,
        center_y - 8,
        center_x + 13,
        center_y + 8,
        RegionalCityFeature::Deck,
    )?;
    carve_rect(
        &mut map,
        &mut features,
        center_x - 2,
        1,
        center_x + 2,
        center_y - 8,
        RegionalCityFeature::Lane,
    )?;
    carve_rect(
        &mut map,
        &mut features,
        center_x - 2,
        center_y + 8,
        center_x + 2,
        height - 2,
        RegionalCityFeature::Lane,
    )?;
    carve_rect(
        &mut map,
        &mut features,
        1,
        center_y - 2,
        center_x - 13,
        center_y + 2,
        RegionalCityFeature::Lane,
    )?;
    carve_rect(
        &mut map,
        &mut features,
        center_x + 13,
        center_y - 2,
        width - 2,
        center_y + 2,
        RegionalCityFeature::Lane,
    )?;

    // Coolant is real shallow-water terrain: it changes movement semantics
    // consistently with the rest of the engine while remaining traversable.
    for bounds in [
        [10, 5, center_x - 10, 7],
        [center_x + 10, 5, width - 11, 7],
        [10, height - 8, center_x - 10, height - 6],
        [center_x + 10, height - 8, width - 11, height - 6],
        [4, 18, 6, center_y - 8],
        [4, center_y + 8, 6, height - 19],
        [width - 7, 18, width - 5, center_y - 8],
        [width - 7, center_y + 8, width - 5, height - 19],
        [center_x - 12, center_y - 7, center_x - 5, center_y - 6],
        [center_x + 5, center_y - 7, center_x + 12, center_y - 6],
        [center_x - 12, center_y + 6, center_x - 5, center_y + 7],
        [center_x + 5, center_y + 6, center_x + 12, center_y + 7],
        [center_x - 12, center_y - 5, center_x - 11, center_y - 3],
        [center_x - 12, center_y + 3, center_x - 11, center_y + 5],
        [center_x + 11, center_y - 5, center_x + 12, center_y - 3],
        [center_x + 11, center_y + 3, center_x + 12, center_y + 5],
    ] {
        paint_rect(
            &mut map,
            &mut features,
            bounds,
            Terrain::ShallowWater,
            RegionalCityFeature::CoolantChannel,
        )?;
    }

    for (position, feature) in [
        (
            GridPos::new(center_x - 10, center_y - 5),
            RegionalCityFeature::Crate,
        ),
        (
            GridPos::new(center_x - 8, center_y + 5),
            RegionalCityFeature::Crate,
        ),
        (
            GridPos::new(center_x + 6, center_y - 2),
            RegionalCityFeature::ClinicCounter,
        ),
        (
            GridPos::new(center_x + 11, center_y - 5),
            RegionalCityFeature::ClinicBed,
        ),
        (
            GridPos::new(center_x + 11, center_y - 2),
            RegionalCityFeature::ClinicBed,
        ),
        (
            GridPos::new(center_x + 11, center_y + 4),
            RegionalCityFeature::ClinicBed,
        ),
        (
            GridPos::new(center_x, center_y - 7),
            RegionalCityFeature::Console,
        ),
        (GridPos::new(22, 19), RegionalCityFeature::Server),
        (GridPos::new(width - 23, 19), RegionalCityFeature::Server),
        (GridPos::new(22, height - 20), RegionalCityFeature::Server),
        (
            GridPos::new(width - 23, height - 20),
            RegionalCityFeature::Server,
        ),
        (
            GridPos::new(center_x - 9, center_y + 6),
            RegionalCityFeature::Coolant,
        ),
        (
            GridPos::new(center_x + 8, center_y + 6),
            RegionalCityFeature::Coolant,
        ),
    ] {
        place_obstacle(&mut map, &mut features, position, feature)?;
    }
    for position in [
        GridPos::new(5, 14),
        GridPos::new(width - 6, 14),
        GridPos::new(5, height - 15),
        GridPos::new(width - 6, height - 15),
    ] {
        place_obstacle(
            &mut map,
            &mut features,
            position,
            RegionalCityFeature::Pillar,
        )?;
    }

    let passages = [
        cardinal_passage(size, Direction::North),
        cardinal_passage(size, Direction::East),
        cardinal_passage(size, Direction::South),
        cardinal_passage(size, Direction::West),
    ];
    validate_playable_map(
        &map,
        passages[0],
        passages[2],
        &[passages[1], passages[3]],
        MapValidationRules::default(),
    )?;
    protect_walkable_cells(&mut map)?;
    Ok(GeneratedRegionalCityMap {
        map,
        features,
        passages,
    })
}

fn generate_dissonant_lattice(
    size: RegionMapSize,
) -> Result<GeneratedRegionalCityMap, RegionalCityGenerationError> {
    let width = i32::from(size.width());
    let height = i32::from(size.height());
    let center_x = width / 2;
    let center_y = height / 2;
    let mut map = Map::filled(width as usize, height as usize, Terrain::Wall)?;
    let mut features = BTreeMap::new();
    for y in 0..height {
        for x in 0..width {
            features.insert(GridPos::new(x, y), RegionalCityFeature::MembraneWall);
        }
    }

    // Layer three is the first visible break with human architecture. Uneven
    // overlapping lobes form inhabited cavities inside a foreign substrate;
    // oblique veins connect them without restoring a rectangular street grid.
    for (center, radius_x, radius_y) in [
        (GridPos::new(center_x - 40, center_y - 15), 13, 8),
        (GridPos::new(center_x - 33, center_y - 12), 11, 7),
        (GridPos::new(center_x + 40, center_y - 16), 12, 7),
        (GridPos::new(center_x + 33, center_y - 12), 10, 8),
        (GridPos::new(center_x - 42, center_y + 14), 13, 8),
        (GridPos::new(center_x - 34, center_y + 11), 11, 7),
        (GridPos::new(center_x + 39, center_y + 14), 16, 10),
        (GridPos::new(center_x - 8, center_y), 17, 9),
        (GridPos::new(center_x + 10, center_y + 3), 16, 10),
    ] {
        carve_ellipse(
            &mut map,
            &mut features,
            center,
            radius_x,
            radius_y,
            RegionalCityFeature::ForeignFloor,
        )?;
    }

    let nexus = GridPos::new(center_x - 2, center_y + 1);
    let passages = [
        cardinal_passage(size, Direction::North),
        cardinal_passage(size, Direction::East),
        cardinal_passage(size, Direction::South),
        cardinal_passage(size, Direction::West),
    ];
    for (start, end) in [
        (passages[3], nexus),
        (passages[1], GridPos::new(center_x + 39, center_y + 5)),
        (GridPos::new(center_x + 39, center_y + 5), nexus),
        (passages[0], GridPos::new(center_x + 8, center_y - 15)),
        (GridPos::new(center_x + 8, center_y - 15), nexus),
        (passages[2], GridPos::new(center_x - 8, center_y + 17)),
        (GridPos::new(center_x - 8, center_y + 17), nexus),
        (GridPos::new(center_x - 40, center_y - 15), nexus),
        (GridPos::new(center_x + 40, center_y - 16), nexus),
        (GridPos::new(center_x - 42, center_y + 14), nexus),
        (GridPos::new(center_x + 39, center_y + 14), nexus),
    ] {
        carve_tube(
            &mut map,
            &mut features,
            start,
            end,
            3,
            RegionalCityFeature::VeinedFloor,
        )?;
    }

    for (position, feature) in [
        (
            GridPos::new(center_x - 34, center_y - 15),
            RegionalCityFeature::GrowthNode,
        ),
        (
            GridPos::new(center_x + 35, center_y - 15),
            RegionalCityFeature::GrowthNode,
        ),
        (
            GridPos::new(center_x - 37, center_y + 15),
            RegionalCityFeature::GrowthNode,
        ),
        (
            GridPos::new(center_x + 34, center_y + 14),
            RegionalCityFeature::GrowthNode,
        ),
        (
            GridPos::new(center_x - 7, center_y - 5),
            RegionalCityFeature::Resonator,
        ),
        (
            GridPos::new(center_x + 5, center_y + 7),
            RegionalCityFeature::Resonator,
        ),
        (
            GridPos::new(center_x + 24, center_y + 10),
            RegionalCityFeature::ClinicCounter,
        ),
        (
            GridPos::new(center_x + 31, center_y + 9),
            RegionalCityFeature::ClinicBed,
        ),
        (
            GridPos::new(center_x + 32, center_y + 12),
            RegionalCityFeature::ClinicBed,
        ),
        (
            GridPos::new(center_x + 31, center_y + 15),
            RegionalCityFeature::ClinicBed,
        ),
        (
            GridPos::new(center_x - 26, center_y - 1),
            RegionalCityFeature::Server,
        ),
        (
            GridPos::new(center_x + 22, center_y - 5),
            RegionalCityFeature::Server,
        ),
        (
            GridPos::new(center_x - 2, center_y - 3),
            RegionalCityFeature::Console,
        ),
    ] {
        place_obstacle(&mut map, &mut features, position, feature)?;
    }
    validate_playable_map(
        &map,
        passages[0],
        passages[2],
        &[passages[1], passages[3]],
        MapValidationRules::default(),
    )?;
    protect_walkable_cells(&mut map)?;
    Ok(GeneratedRegionalCityMap {
        map,
        features,
        passages,
    })
}

fn generate_recursive_bloom(
    size: RegionMapSize,
) -> Result<GeneratedRegionalCityMap, RegionalCityGenerationError> {
    let width = i32::from(size.width());
    let height = i32::from(size.height());
    let center = GridPos::new(width / 2 - 2, height / 2 + 1);
    let northwest = GridPos::new(center.x - 24, center.y - 25);
    let northeast = GridPos::new(center.x + 26, center.y - 27);
    let east = GridPos::new(center.x + 28, center.y);
    let southeast = GridPos::new(center.x + 20, center.y + 25);
    let southwest = GridPos::new(center.x - 24, center.y + 23);
    let west = GridPos::new(center.x - 29, center.y - 2);
    let mut map = Map::filled(width as usize, height as usize, Terrain::Wall)?;
    let mut features = BTreeMap::new();
    for y in 0..height {
        for x in 0..width {
            features.insert(GridPos::new(x, y), RegionalCityFeature::VoidWall);
        }
    }

    // Layer four no longer resembles a building embedded in alien matter. A
    // recursive bloom of angular cells is the settlement itself: each scale
    // repeats the same diamond grammar, while asymmetric branches prevent it
    // from reading as a planned human plaza or corridor network.
    for (cell, radius) in [
        (center, 18),
        (northwest, 12),
        (northeast, 11),
        (east, 13),
        (southeast, 12),
        (southwest, 12),
        (west, 9),
    ] {
        carve_diamond(
            &mut map,
            &mut features,
            cell,
            radius,
            RegionalCityFeature::ChitinFloor,
        )?;
        paint_diamond_frame(
            &mut map,
            &mut features,
            cell,
            radius.saturating_sub(2),
            RegionalCityFeature::PulseChannel,
        )?;
    }
    for radius in [5, 11, 17] {
        paint_diamond_frame(
            &mut map,
            &mut features,
            center,
            radius,
            RegionalCityFeature::PulseChannel,
        )?;
    }

    let passages = [
        cardinal_passage(size, Direction::North),
        cardinal_passage(size, Direction::East),
        cardinal_passage(size, Direction::South),
        cardinal_passage(size, Direction::West),
    ];
    for (start, end) in [
        (passages[0], northeast),
        (northeast, center),
        (passages[1], east),
        (east, center),
        (passages[2], southeast),
        (southeast, center),
        (passages[3], west),
        (west, center),
        (northwest, center),
        (southwest, center),
    ] {
        carve_tube(
            &mut map,
            &mut features,
            start,
            end,
            1,
            RegionalCityFeature::PulseChannel,
        )?;
    }

    for (position, feature) in [
        (
            GridPos::new(center.x - 1, center.y - 9),
            RegionalCityFeature::EyeNode,
        ),
        (
            GridPos::new(center.x + 7, center.y + 3),
            RegionalCityFeature::EyeNode,
        ),
        (
            GridPos::new(northwest.x - 4, northwest.y + 1),
            RegionalCityFeature::RootMass,
        ),
        (
            GridPos::new(northeast.x + 3, northeast.y + 1),
            RegionalCityFeature::RootMass,
        ),
        (
            GridPos::new(southwest.x - 3, southwest.y - 1),
            RegionalCityFeature::RootMass,
        ),
        (
            GridPos::new(southeast.x + 3, southeast.y - 1),
            RegionalCityFeature::RootMass,
        ),
        (
            GridPos::new(center.x + 18, center.y - 3),
            RegionalCityFeature::Server,
        ),
        (
            GridPos::new(center.x - 17, center.y + 2),
            RegionalCityFeature::Crate,
        ),
        (
            GridPos::new(east.x - 4, east.y - 2),
            RegionalCityFeature::ClinicCounter,
        ),
        (
            GridPos::new(east.x + 5, east.y - 6),
            RegionalCityFeature::ClinicBed,
        ),
        (
            GridPos::new(east.x + 7, east.y - 3),
            RegionalCityFeature::ClinicBed,
        ),
        (
            GridPos::new(east.x + 7, east.y + 4),
            RegionalCityFeature::ClinicBed,
        ),
    ] {
        place_obstacle(&mut map, &mut features, position, feature)?;
    }

    validate_playable_map(
        &map,
        passages[0],
        passages[2],
        &[passages[1], passages[3]],
        MapValidationRules::default(),
    )?;
    protect_walkable_cells(&mut map)?;
    Ok(GeneratedRegionalCityMap {
        map,
        features,
        passages,
    })
}

fn generate_process_ruin(
    size: RegionMapSize,
) -> Result<GeneratedRegionalCityMap, RegionalCityGenerationError> {
    let width = i32::from(size.width());
    let height = i32::from(size.height());
    let mut map = Map::filled(width as usize, height as usize, Terrain::Wall)?;
    let mut features = BTreeMap::new();
    for y in 0..height {
        for x in 0..width {
            features.insert(GridPos::new(x, y), RegionalCityFeature::DeadScreen);
        }
    }

    // Layer five is a dead operating system that still executes. Rectangular
    // terminal windows have become rooms; their broken chrome remains visible
    // across the floor while the black screen outside consumes the map.
    for window in [
        [42, 2, 62, 19],
        [14, 10, 36, 27],
        [68, 11, 91, 28],
        [6, 32, 35, 52],
        [39, 25, 65, 54],
        [69, 31, 97, 53],
        [14, 57, 37, 73],
        [42, 58, 65, 78],
        [68, 57, 91, 72],
    ] {
        carve_rect(
            &mut map,
            &mut features,
            window[0],
            window[1],
            window[2],
            window[3],
            RegionalCityFeature::MemoryFloor,
        )?;
        paint_process_window(&map, &mut features, window);
    }

    let passages = [
        cardinal_passage(size, Direction::North),
        cardinal_passage(size, Direction::East),
        cardinal_passage(size, Direction::South),
        cardinal_passage(size, Direction::West),
    ];
    for (start, end) in [
        (passages[0], GridPos::new(52, 19)),
        (passages[1], GridPos::new(69, 40)),
        (passages[2], GridPos::new(52, 58)),
        (passages[3], GridPos::new(35, 40)),
        (GridPos::new(36, 17), GridPos::new(42, 17)),
        (GridPos::new(52, 19), GridPos::new(52, 25)),
        (GridPos::new(62, 18), GridPos::new(68, 18)),
        (GridPos::new(80, 28), GridPos::new(80, 31)),
        (GridPos::new(35, 40), GridPos::new(39, 40)),
        (GridPos::new(65, 40), GridPos::new(69, 40)),
        (GridPos::new(26, 52), GridPos::new(26, 57)),
        (GridPos::new(37, 64), GridPos::new(42, 64)),
        (GridPos::new(52, 54), GridPos::new(52, 58)),
        (GridPos::new(65, 64), GridPos::new(68, 64)),
        (GridPos::new(80, 53), GridPos::new(80, 57)),
    ] {
        carve_tube(
            &mut map,
            &mut features,
            start,
            end,
            1,
            RegionalCityFeature::MemoryFloor,
        )?;
    }

    // The arrival process contains a smaller window inside the larger one, so
    // the broken-interface idea is readable immediately rather than only at
    // the distant edges of the settlement.
    paint_process_window(&map, &mut features, [43, 31, 62, 49]);

    // Red appears only where execution has failed. These broken traces are
    // presentation on ordinary floor cells and never imply a hidden hazard.
    for (start, end) in [
        (GridPos::new(42, 34), GridPos::new(49, 34)),
        (GridPos::new(54, 34), GridPos::new(62, 36)),
        (GridPos::new(43, 47), GridPos::new(49, 42)),
        (GridPos::new(55, 41), GridPos::new(62, 38)),
        (GridPos::new(18, 15), GridPos::new(24, 15)),
        (GridPos::new(74, 23), GridPos::new(81, 21)),
        (GridPos::new(20, 66), GridPos::new(26, 66)),
        (GridPos::new(75, 63), GridPos::new(82, 65)),
    ] {
        carve_tube(
            &mut map,
            &mut features,
            start,
            end,
            0,
            RegionalCityFeature::FaultTrace,
        )?;
    }

    // Opaque dead windows float inside the central process. Their outlines are
    // real walls, so the software-nightmare silhouette follows normal LOS and
    // pathfinding instead of faking information through a screen effect.
    for [minimum_x, minimum_y, maximum_x, maximum_y] in
        [[41, 27, 47, 29], [57, 46, 63, 49], [48, 51, 55, 53]]
    {
        for y in minimum_y..=maximum_y {
            for x in minimum_x..=maximum_x {
                place_obstacle(
                    &mut map,
                    &mut features,
                    GridPos::new(x, y),
                    RegionalCityFeature::DeadScreen,
                )?;
            }
        }
    }

    for (position, feature) in [
        (GridPos::new(50, 34), RegionalCityFeature::KernelFault),
        (GridPos::new(60, 37), RegionalCityFeature::KernelFault),
        (GridPos::new(44, 46), RegionalCityFeature::OrphanProcess),
        (GridPos::new(61, 32), RegionalCityFeature::OrphanProcess),
        (GridPos::new(31, 16), RegionalCityFeature::OrphanProcess),
        (GridPos::new(75, 64), RegionalCityFeature::OrphanProcess),
        (GridPos::new(20, 45), RegionalCityFeature::Server),
        (GridPos::new(76, 35), RegionalCityFeature::ClinicCounter),
        (GridPos::new(84, 35), RegionalCityFeature::ClinicBed),
        (GridPos::new(87, 40), RegionalCityFeature::ClinicBed),
        (GridPos::new(84, 47), RegionalCityFeature::ClinicBed),
    ] {
        place_obstacle(&mut map, &mut features, position, feature)?;
    }

    validate_playable_map(
        &map,
        passages[0],
        passages[2],
        &[passages[1], passages[3]],
        MapValidationRules::default(),
    )?;
    protect_walkable_cells(&mut map)?;
    Ok(GeneratedRegionalCityMap {
        map,
        features,
        passages,
    })
}

fn paint_process_window(
    map: &Map,
    features: &mut BTreeMap<GridPos, RegionalCityFeature>,
    [minimum_x, minimum_y, maximum_x, maximum_y]: [i32; 4],
) {
    for y in minimum_y..=maximum_y {
        for x in minimum_x..=maximum_x {
            let on_edge = x == minimum_x || x == maximum_x || y == minimum_y || y == maximum_y;
            let on_title_bar = y == minimum_y + 2 && x >= minimum_x + 2 && x <= maximum_x - 2;
            if (on_edge || on_title_bar)
                && (x * 5 + y * 3).rem_euclid(11) != 0
                && map.is_walkable(GridPos::new(x, y))
            {
                features.insert(GridPos::new(x, y), RegionalCityFeature::WindowFrame);
            }
        }
    }
}

fn carve_diamond(
    map: &mut Map,
    features: &mut BTreeMap<GridPos, RegionalCityFeature>,
    center: GridPos,
    radius: i32,
    feature: RegionalCityFeature,
) -> Result<(), MapBuildError> {
    for y in (center.y - radius)..=(center.y + radius) {
        for x in (center.x - radius)..=(center.x + radius) {
            if x <= 0
                || y <= 0
                || x >= map.width() as i32 - 1
                || y >= map.height() as i32 - 1
                || (x - center.x).abs() + (y - center.y).abs() > radius
            {
                continue;
            }
            let position = GridPos::new(x, y);
            map.set_terrain(position, Terrain::Floor)?;
            features.insert(position, feature);
        }
    }
    Ok(())
}

fn paint_diamond_frame(
    map: &mut Map,
    features: &mut BTreeMap<GridPos, RegionalCityFeature>,
    center: GridPos,
    radius: i32,
    feature: RegionalCityFeature,
) -> Result<(), MapBuildError> {
    for y in (center.y - radius)..=(center.y + radius) {
        for x in (center.x - radius)..=(center.x + radius) {
            if (x - center.x).abs() + (y - center.y).abs() != radius {
                continue;
            }
            let position = GridPos::new(x, y);
            if map.is_walkable(position) {
                map.set_terrain(position, Terrain::Floor)?;
                features.insert(position, feature);
            }
        }
    }
    Ok(())
}

fn carve_ellipse(
    map: &mut Map,
    features: &mut BTreeMap<GridPos, RegionalCityFeature>,
    center: GridPos,
    radius_x: i32,
    radius_y: i32,
    feature: RegionalCityFeature,
) -> Result<(), MapBuildError> {
    let radius_x_squared = i64::from(radius_x * radius_x);
    let radius_y_squared = i64::from(radius_y * radius_y);
    let limit = radius_x_squared * radius_y_squared;
    for y in (center.y - radius_y)..=(center.y + radius_y) {
        for x in (center.x - radius_x)..=(center.x + radius_x) {
            if x <= 0 || y <= 0 || x >= map.width() as i32 - 1 || y >= map.height() as i32 - 1 {
                continue;
            }
            let dx = i64::from(x - center.x);
            let dy = i64::from(y - center.y);
            if dx * dx * radius_y_squared + dy * dy * radius_x_squared <= limit {
                let position = GridPos::new(x, y);
                map.set_terrain(position, Terrain::Floor)?;
                features.insert(position, feature);
            }
        }
    }
    Ok(())
}

fn carve_tube(
    map: &mut Map,
    features: &mut BTreeMap<GridPos, RegionalCityFeature>,
    start: GridPos,
    end: GridPos,
    radius: i32,
    feature: RegionalCityFeature,
) -> Result<(), MapBuildError> {
    let delta_x = end.x - start.x;
    let delta_y = end.y - start.y;
    let steps = delta_x.abs().max(delta_y.abs()).max(1);
    for step in 0..=steps {
        let center = GridPos::new(
            start.x + delta_x * step / steps,
            start.y + delta_y * step / steps,
        );
        for offset_y in -radius..=radius {
            for offset_x in -radius..=radius {
                if offset_x * offset_x + offset_y * offset_y > radius * radius {
                    continue;
                }
                let position = GridPos::new(center.x + offset_x, center.y + offset_y);
                if position.x <= 0
                    || position.y <= 0
                    || position.x >= map.width() as i32 - 1
                    || position.y >= map.height() as i32 - 1
                {
                    continue;
                }
                map.set_terrain(position, Terrain::Floor)?;
                features.insert(position, feature);
            }
        }
    }
    Ok(())
}

fn carve_frame(
    map: &mut Map,
    features: &mut BTreeMap<GridPos, RegionalCityFeature>,
    [minimum_x, minimum_y, maximum_x, maximum_y]: [i32; 4],
    thickness: i32,
    feature: RegionalCityFeature,
) -> Result<(), MapBuildError> {
    carve_rect(
        map,
        features,
        minimum_x,
        minimum_y,
        maximum_x,
        minimum_y + thickness - 1,
        feature,
    )?;
    carve_rect(
        map,
        features,
        minimum_x,
        maximum_y - thickness + 1,
        maximum_x,
        maximum_y,
        feature,
    )?;
    carve_rect(
        map,
        features,
        minimum_x,
        minimum_y + thickness,
        minimum_x + thickness - 1,
        maximum_y - thickness,
        feature,
    )?;
    carve_rect(
        map,
        features,
        maximum_x - thickness + 1,
        minimum_y + thickness,
        maximum_x,
        maximum_y - thickness,
        feature,
    )
}

fn paint_rect(
    map: &mut Map,
    features: &mut BTreeMap<GridPos, RegionalCityFeature>,
    [minimum_x, minimum_y, maximum_x, maximum_y]: [i32; 4],
    terrain: Terrain,
    feature: RegionalCityFeature,
) -> Result<(), MapBuildError> {
    for y in minimum_y..=maximum_y {
        for x in minimum_x..=maximum_x {
            let position = GridPos::new(x, y);
            map.set_terrain(position, terrain)?;
            features.insert(position, feature);
        }
    }
    Ok(())
}

fn protect_walkable_cells(map: &mut Map) -> Result<(), MapBuildError> {
    for y in 0..map.height() {
        for x in 0..map.width() {
            let position = GridPos::new(x as i32, y as i32);
            if map.is_walkable(position) {
                map.set_protected(position, true)?;
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn carve_rect(
    map: &mut Map,
    features: &mut BTreeMap<GridPos, RegionalCityFeature>,
    minimum_x: i32,
    minimum_y: i32,
    maximum_x: i32,
    maximum_y: i32,
    feature: RegionalCityFeature,
) -> Result<(), MapBuildError> {
    for y in minimum_y..=maximum_y {
        for x in minimum_x..=maximum_x {
            let position = GridPos::new(x, y);
            map.set_terrain(position, Terrain::Floor)?;
            features.insert(position, feature);
        }
    }
    Ok(())
}

fn place_obstacle(
    map: &mut Map,
    features: &mut BTreeMap<GridPos, RegionalCityFeature>,
    position: GridPos,
    feature: RegionalCityFeature,
) -> Result<(), MapBuildError> {
    map.set_terrain(position, Terrain::Wall)?;
    features.insert(position, feature);
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegionalCityGenerationError {
    LayoutTooSmall,
    Map(MapBuildError),
    Validation(MapValidationError),
}

impl Display for RegionalCityGenerationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LayoutTooSmall => write!(formatter, "regional city layout is too small"),
            Self::Map(error) => write!(formatter, "regional city map failed: {error}"),
            Self::Validation(error) => write!(formatter, "regional city is not playable: {error}"),
        }
    }
}

impl Error for RegionalCityGenerationError {}

impl From<MapBuildError> for RegionalCityGenerationError {
    fn from(value: MapBuildError) -> Self {
        Self::Map(value)
    }
}

impl From<MapValidationError> for RegionalCityGenerationError {
    fn from(value: MapValidationError) -> Self {
        Self::Validation(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maintenance_spine_is_connected_sealed_and_protected() {
        let generated = generate_regional_city(
            RegionMapSize::new(96, 64).unwrap(),
            RegionCityLayout::MaintenanceSpine,
        )
        .unwrap();
        let passages = generated.passages();
        validate_playable_map(
            generated.map(),
            passages[0],
            passages[2],
            &[passages[1], passages[3]],
            MapValidationRules::default(),
        )
        .unwrap();
        assert!(
            (0..generated.map().height()).all(|y| (0..generated.map().width()).all(|x| {
                let position = GridPos::new(x as i32, y as i32);
                !generated.map().is_walkable(position) || generated.map().is_protected(position)
            }))
        );
    }

    #[test]
    fn maintenance_spine_is_deterministic_and_visually_structured() {
        let size = RegionMapSize::new(96, 64).unwrap();
        let first = generate_regional_city(size, RegionCityLayout::MaintenanceSpine).unwrap();
        let second = generate_regional_city(size, RegionCityLayout::MaintenanceSpine).unwrap();
        assert_eq!(first, second);
        assert!(
            first
                .features()
                .values()
                .any(|feature| { matches!(feature, RegionalCityFeature::ClinicBed) })
        );
        assert!(
            first
                .features()
                .values()
                .any(|feature| { matches!(feature, RegionalCityFeature::Server) })
        );
    }

    #[test]
    fn coolant_rings_are_connected_protected_and_visually_distinct() {
        let size = RegionMapSize::new(80, 72).unwrap();
        let first = generate_regional_city(size, RegionCityLayout::CoolantRings).unwrap();
        let second = generate_regional_city(size, RegionCityLayout::CoolantRings).unwrap();
        assert_eq!(first, second);
        let passages = first.passages();
        validate_playable_map(
            first.map(),
            passages[0],
            passages[2],
            &[passages[1], passages[3]],
            MapValidationRules::default(),
        )
        .unwrap();
        assert!(
            first
                .features()
                .values()
                .any(|feature| *feature == RegionalCityFeature::CoolantChannel)
        );
        assert!(
            first
                .features()
                .values()
                .any(|feature| *feature == RegionalCityFeature::CorrodedDeck)
        );
        assert!((0..first.map().height()).all(|y| {
            (0..first.map().width()).all(|x| {
                let position = GridPos::new(x as i32, y as i32);
                !first.map().is_walkable(position) || first.map().is_protected(position)
            })
        }));
    }

    #[test]
    fn dissonant_lattice_is_wide_connected_and_visibly_foreign() {
        let size = RegionMapSize::new(112, 56).unwrap();
        let first = generate_regional_city(size, RegionCityLayout::DissonantLattice).unwrap();
        let second = generate_regional_city(size, RegionCityLayout::DissonantLattice).unwrap();
        assert_eq!(first, second);
        assert!(first.map().width() > first.map().height());
        let passages = first.passages();
        validate_playable_map(
            first.map(),
            passages[0],
            passages[2],
            &[passages[1], passages[3]],
            MapValidationRules::default(),
        )
        .unwrap();
        assert!(
            first
                .features()
                .values()
                .any(|feature| *feature == RegionalCityFeature::ForeignFloor)
        );
        assert!(
            first
                .features()
                .values()
                .any(|feature| *feature == RegionalCityFeature::VeinedFloor)
        );
        assert!(
            first
                .features()
                .values()
                .any(|feature| *feature == RegionalCityFeature::MembraneWall)
        );
        assert!(
            first
                .features()
                .values()
                .any(|feature| *feature == RegionalCityFeature::Resonator)
        );
        assert!((0..first.map().height()).all(|y| {
            (0..first.map().width()).all(|x| {
                let position = GridPos::new(x as i32, y as i32);
                !first.map().is_walkable(position) || first.map().is_protected(position)
            })
        }));
    }

    #[test]
    fn recursive_bloom_is_square_connected_and_uses_its_own_visual_family() {
        let size = RegionMapSize::new(88, 88).unwrap();
        let first = generate_regional_city(size, RegionCityLayout::RecursiveBloom).unwrap();
        let second = generate_regional_city(size, RegionCityLayout::RecursiveBloom).unwrap();
        assert_eq!(first, second);
        assert_eq!(first.map().width(), first.map().height());
        let passages = first.passages();
        validate_playable_map(
            first.map(),
            passages[0],
            passages[2],
            &[passages[1], passages[3]],
            MapValidationRules::default(),
        )
        .unwrap();
        for expected in [
            RegionalCityFeature::ChitinFloor,
            RegionalCityFeature::PulseChannel,
            RegionalCityFeature::VoidWall,
            RegionalCityFeature::EyeNode,
            RegionalCityFeature::RootMass,
        ] {
            assert!(
                first
                    .features()
                    .values()
                    .any(|feature| *feature == expected)
            );
        }
        assert!((0..first.map().height()).all(|y| {
            (0..first.map().width()).all(|x| {
                let position = GridPos::new(x as i32, y as i32);
                !first.map().is_walkable(position) || first.map().is_protected(position)
            })
        }));
    }

    #[test]
    fn process_ruin_is_connected_asymmetric_and_uses_broken_ui_geometry() {
        let size = RegionMapSize::new(104, 80).unwrap();
        let first = generate_regional_city(size, RegionCityLayout::ProcessRuin).unwrap();
        let second = generate_regional_city(size, RegionCityLayout::ProcessRuin).unwrap();
        assert_eq!(first, second);
        assert_ne!(first.map().width(), first.map().height());
        let passages = first.passages();
        validate_playable_map(
            first.map(),
            passages[0],
            passages[2],
            &[passages[1], passages[3]],
            MapValidationRules::default(),
        )
        .unwrap();
        for expected in [
            RegionalCityFeature::MemoryFloor,
            RegionalCityFeature::WindowFrame,
            RegionalCityFeature::FaultTrace,
            RegionalCityFeature::DeadScreen,
            RegionalCityFeature::KernelFault,
            RegionalCityFeature::OrphanProcess,
        ] {
            assert!(
                first
                    .features()
                    .values()
                    .any(|feature| *feature == expected)
            );
        }
        assert!((0..first.map().height()).all(|y| {
            (0..first.map().width()).all(|x| {
                let position = GridPos::new(x as i32, y as i32);
                !first.map().is_walkable(position) || first.map().is_protected(position)
            })
        }));
    }

    #[test]
    fn city_layouts_enforce_their_minimum_dimensions() {
        assert_eq!(
            generate_regional_city(
                RegionMapSize::new(48, 40).unwrap(),
                RegionCityLayout::CoolantRings,
            ),
            Err(RegionalCityGenerationError::LayoutTooSmall)
        );
        assert_eq!(
            generate_regional_city(
                RegionMapSize::new(87, 88).unwrap(),
                RegionCityLayout::RecursiveBloom,
            ),
            Err(RegionalCityGenerationError::LayoutTooSmall)
        );
        assert_eq!(
            generate_regional_city(
                RegionMapSize::new(103, 80).unwrap(),
                RegionCityLayout::ProcessRuin,
            ),
            Err(RegionalCityGenerationError::LayoutTooSmall)
        );
    }
}
