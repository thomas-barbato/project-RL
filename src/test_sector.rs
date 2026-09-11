//! Prototype world fixture: an invariant town plus a seeded exterior.
//! Layout and visual families are separate from generic interaction rules.
use project_rl::game::GameRng;
use project_rl::world::generation::{GeneratedMap, MapValidationRules};
use project_rl::world::{DoorState, GridPos, Map, Terrain};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Decor {
    #[default]
    Deck,
    Grate,
    Lane,
    Threshold,
    Gravel,
    Wall,
    Pillar,
    Crate,
    Server,
    Console,
    Coolant,
    DoorClosed,
    DoorOpen,
    DoorLocked,
    DoorUnpowered,
    ControlReady,
    ControlUsed,
    Depot,
    RelayOffline,
    RelayOnline,
    ActuatorOffline,
    ActuatorOnline,
    SensorOffline,
    SensorOnline,
    Passage,
}

impl Decor {
    pub fn label(self) -> &'static str {
        match self {
            Self::Deck => "Dallage · passage libre",
            Self::Grate => "Caillebotis · passage libre",
            Self::Lane => "Voie de circulation · passage libre",
            Self::Threshold => "Seuil ouvert · passage libre",
            Self::Gravel => "Terrain extérieur · passage libre",
            Self::Wall => "Cloison · passage et vue bloqués",
            Self::Pillar => "Pilier · passage et vue bloqués",
            Self::Crate => "Conteneur · passage et vue bloqués",
            Self::Server => "Baie serveur · passage et vue bloqués",
            Self::Console => "Pupitre inactif · obstacle",
            Self::Coolant => "Cuve · passage et vue bloqués",
            Self::DoorClosed => "Porte fermée · interagir pour ouvrir",
            Self::DoorOpen => "Porte ouverte · interagir pour fermer",
            Self::DoorLocked => "Porte verrouillée · trouver sa console",
            Self::DoorUnpowered => "Porte de service · alimentation absente",
            Self::ControlReady => "Console active · déverrouille un accès",
            Self::ControlUsed => "Console utilisée · accès déverrouillé",
            Self::Depot => "Dépôt de maintenance · interagir pour livrer une pièce demandée",
            Self::RelayOffline => "Relais de puissance · en panne",
            Self::RelayOnline => "Relais de puissance · opérationnel",
            Self::ActuatorOffline => "Commande de porte · hors ligne",
            Self::ActuatorOnline => "Commande de porte · alimentée",
            Self::SensorOffline => "Capteur de sécurité · hors ligne",
            Self::SensorOnline => "Capteur de sécurité · opérationnel",
            Self::Passage => "Passage vers une autre zone · interagir pour voyager",
        }
    }
    pub fn blocks(self) -> bool {
        !matches!(
            self,
            Self::Deck
                | Self::Grate
                | Self::Lane
                | Self::Threshold
                | Self::Gravel
                | Self::DoorOpen
                | Self::Passage
        )
    }
}

pub struct Zone {
    pub name: &'static str,
    pub bounds: [i32; 4],
}
#[derive(Default)]
pub struct SectorDecor {
    pub cells: BTreeMap<GridPos, Decor>,
    pub zones: Vec<Zone>,
}

impl SectorDecor {
    pub fn at(&self, position: GridPos, terrain: Terrain) -> Decor {
        match terrain {
            Terrain::Door(DoorState::Closed) => return Decor::DoorClosed,
            Terrain::Door(DoorState::Open) => return Decor::DoorOpen,
            Terrain::Door(DoorState::Locked) => return Decor::DoorLocked,
            Terrain::Door(DoorState::Unpowered) => return Decor::DoorUnpowered,
            Terrain::ControlPanel {
                activated: false, ..
            } => return Decor::ControlReady,
            Terrain::ControlPanel {
                activated: true, ..
            } => return Decor::ControlUsed,
            _ => {}
        }
        self.cells
            .get(&position)
            .copied()
            .filter(|d| d.blocks() == terrain.blocks_movement())
            .unwrap_or(if terrain.blocks_movement() {
                Decor::Wall
            } else {
                Decor::Deck
            })
    }
    pub fn zone_at(&self, position: GridPos) -> &str {
        self.zones
            .iter()
            .find(|zone| {
                let [x, y, w, h] = zone.bounds;
                position.x >= x && position.x < x + w && position.y >= y && position.y < y + h
            })
            .map_or("Friches extérieures", |zone| zone.name)
    }
}

pub struct TestSector {
    pub level: GeneratedMap,
    pub decor: SectorDecor,
    pub enemies: Vec<GridPos>,
    pub loot: Vec<GridPos>,
}

impl TestSector {
    pub const NAME: &str = "VILLE DE DÉPART / FRICHES";
    pub const GATE: GridPos = GridPos::new(62, 21);
    pub const LOCKED_DOOR: GridPos = GridPos::new(52, 15);
    pub const CONTROL: GridPos = GridPos::new(50, 17);

    pub fn build(seed: u64) -> Result<Self, String> {
        let mut map = Map::filled(110, 68, Terrain::Wall).map_err(|e| e.to_string())?;
        let mut decor = SectorDecor::default();
        paint(&mut map, &mut decor, [1, 1, 108, 66], Decor::Gravel)?;
        // The entire town, including its fixtures and spawn, is seed-independent.
        paint(&mut map, &mut decor, [1, 1, 62, 44], Decor::Wall)?;
        paint(&mut map, &mut decor, [2, 2, 60, 42], Decor::Deck)?;
        paint(&mut map, &mut decor, [2, 19, 60, 6], Decor::Lane)?;
        for bounds in [[22, 2, 3, 42], [42, 2, 3, 42]] {
            paint(&mut map, &mut decor, bounds, Decor::Lane)?;
        }
        for (name, bounds, surface, door) in [
            ("Accueil", [6, 5, 14, 11], Decor::Deck, GridPos::new(12, 15)),
            (
                "Atelier",
                [26, 5, 14, 11],
                Decor::Grate,
                GridPos::new(32, 15),
            ),
            (
                "Archives verrouillées",
                [46, 5, 12, 11],
                Decor::Deck,
                Self::LOCKED_DOOR,
            ),
            (
                "Habitat",
                [6, 28, 14, 11],
                Decor::Deck,
                GridPos::new(12, 28),
            ),
            ("Dépôt", [26, 28, 14, 11], Decor::Deck, GridPos::new(32, 28)),
            (
                "Poste de contrôle",
                [46, 28, 12, 11],
                Decor::Grate,
                GridPos::new(52, 28),
            ),
        ] {
            let [x, y, w, h] = bounds;
            paint(&mut map, &mut decor, bounds, Decor::Wall)?;
            paint(&mut map, &mut decor, [x + 1, y + 1, w - 2, h - 2], surface)?;
            map.set_terrain(
                door,
                Terrain::Door(if door == Self::LOCKED_DOOR {
                    DoorState::Locked
                } else {
                    DoorState::Closed
                }),
            )
            .map_err(|e| e.to_string())?;
            decor.zones.push(Zone { name, bounds });
        }
        decor.zones.push(Zone {
            name: "Place centrale",
            bounds: [21, 16, 25, 12],
        });
        decor.zones.push(Zone {
            name: "Rues de la ville",
            bounds: [1, 1, 62, 44],
        });
        for bounds in [[30, 20, 3, 3], [9, 31, 2, 2], [16, 34, 2, 2]] {
            paint(&mut map, &mut decor, bounds, Decor::Pillar)?;
        }
        for bounds in [[8, 7, 5, 1], [28, 7, 4, 2], [48, 35, 5, 1]] {
            paint(&mut map, &mut decor, bounds, Decor::Console)?;
        }
        for bounds in [[28, 31, 3, 2], [35, 35, 3, 2]] {
            paint(&mut map, &mut decor, bounds, Decor::Crate)?;
        }
        for bounds in [[48, 7, 2, 4], [53, 7, 2, 4]] {
            paint(&mut map, &mut decor, bounds, Decor::Server)?;
        }
        paint(&mut map, &mut decor, [35, 9, 2, 3], Decor::Coolant)?;
        map.set_terrain(
            Self::CONTROL,
            Terrain::ControlPanel {
                door: Self::LOCKED_DOOR,
                activated: false,
            },
        )
        .map_err(|e| e.to_string())?;
        paint(&mut map, &mut decor, [60, 20, 2, 3], Decor::Threshold)?;
        map.set_terrain(Self::GATE, Terrain::Door(DoorState::Closed))
            .map_err(|e| e.to_string())?;
        for y in 1..45 {
            for x in 1..63 {
                map.set_protected(GridPos::new(x, y), true)
                    .map_err(|e| e.to_string())?;
            }
        }
        // Fixed access road, seeded rubble fields. A gap around each obstacle
        // retains navigation; final independent validation includes all doors.
        paint(&mut map, &mut decor, [63, 20, 44, 3], Decor::Lane)?;
        let start = GridPos::new(27, 23);
        let exit = GridPos::new(103, 59);
        let enemies = vec![
            GridPos::new(74, 21),
            GridPos::new(91, 14),
            GridPos::new(85, 36),
            GridPos::new(99, 51),
            GridPos::new(71, 55),
        ];
        let loot = vec![GridPos::new(35, 24), GridPos::new(28, 25)];
        let required: Vec<_> = enemies.iter().chain(&loot).copied().chain([exit]).collect();
        let mut rng = GameRng::from_seed(seed);
        for _ in 0..180 {
            let x = rng.usize_inclusive(3, 102).unwrap() as i32;
            let y = rng.usize_inclusive(3, 60).unwrap() as i32;
            let w = rng.usize_inclusive(2, 5).unwrap() as i32;
            let h = rng.usize_inclusive(2, 4).unwrap() as i32;
            let clear = (y - 1..y + h + 1).all(|py| {
                (x - 1..x + w + 1).all(|px| {
                    let p = GridPos::new(px, py);
                    map.is_walkable(p)
                        && !map.is_protected(p)
                        && !required.contains(&p)
                        && decor.cells.get(&p) == Some(&Decor::Gravel)
                })
            });
            if clear {
                let kind = match rng.next_u64() % 3 {
                    0 => Decor::Crate,
                    1 => Decor::Wall,
                    _ => Decor::Coolant,
                };
                paint(&mut map, &mut decor, [x, y, w, h], kind)?;
            }
        }
        let level =
            GeneratedMap::from_layout(map, start, exit, &required, MapValidationRules::default())
                .map_err(|e| format!("Carte ville/extérieur invalide : {e}"))?;
        Ok(Self {
            level,
            decor,
            enemies,
            loot,
        })
    }
}

fn paint(
    map: &mut Map,
    decor: &mut SectorDecor,
    bounds: [i32; 4],
    kind: Decor,
) -> Result<(), String> {
    let [x, y, w, h] = bounds;
    for py in y..y + h {
        for px in x..x + w {
            let p = GridPos::new(px, py);
            map.set_terrain(
                p,
                if kind.blocks() {
                    Terrain::Wall
                } else {
                    Terrain::Floor
                },
            )
            .map_err(|e| e.to_string())?;
            decor.cells.insert(p, kind);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use project_rl::world::generation::validate_interactive_map;
    #[test]
    fn town_is_fixed_exterior_varies_and_all_interactive_routes_are_solvable() {
        let first = TestSector::build(1).unwrap();
        assert_eq!(first.level, TestSector::build(1).unwrap().level);
        for seed in 2..34 {
            let next = TestSector::build(seed).unwrap();
            assert_ne!(first.level.map(), next.level.map());
            for y in 1..45 {
                for x in 1..63 {
                    let p = GridPos::new(x, y);
                    assert_eq!(first.level.map().tile(p), next.level.map().tile(p));
                    assert_eq!(first.decor.cells.get(&p), next.decor.cells.get(&p));
                }
            }
            assert!(
                next.enemies
                    .iter()
                    .all(|p| !next.level.map().is_protected(*p))
            );
        }
    }
    #[test]
    fn validator_rejects_console_locked_behind_its_own_door() {
        let mut map = Map::from_ascii("#######\n#.....#\n#######").unwrap();
        let door = GridPos::new(3, 1);
        map.set_terrain(door, Terrain::Door(DoorState::Locked))
            .unwrap();
        map.set_terrain(
            GridPos::new(5, 1),
            Terrain::ControlPanel {
                door,
                activated: false,
            },
        )
        .unwrap();
        assert!(
            validate_interactive_map(
                &map,
                GridPos::new(1, 1),
                GridPos::new(4, 1),
                &[],
                MapValidationRules::default()
            )
            .is_err()
        );
        map.set_terrain(
            GridPos::new(2, 0),
            Terrain::ControlPanel {
                door,
                activated: false,
            },
        )
        .unwrap();
        validate_interactive_map(
            &map,
            GridPos::new(1, 1),
            GridPos::new(4, 1),
            &[],
            MapValidationRules::default(),
        )
        .unwrap();
    }
}
