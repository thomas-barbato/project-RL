//! Prototype world fixture: an invariant town plus a seeded exterior.
//! Layout and visual families are separate from generic interaction rules.
use project_rl::game::GameRng;
use project_rl::world::generation::{GeneratedMap, MapValidationRules};
use project_rl::world::{Direction, DoorState, GridPos, Map, Terrain};
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Decor {
    #[default]
    Deck,
    Grate,
    Lane,
    Threshold,
    ForeignFloor,
    VeinedFloor,
    ChitinFloor,
    PulseChannel,
    MemoryFloor,
    WindowFrame,
    FaultTrace,
    Gravel,
    Grass,
    Scrub,
    Mud,
    ShallowWater,
    DeepWater,
    Tree,
    Boulder,
    RuinFloor,
    RuinWall,
    Barricade,
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
    DataTerminalOffline,
    DataTerminalOnline,
    DataTerminalUpdated,
    DirectionBoard,
    DirectionBoardUpdated,
    ServicePlan,
    SupplyCache,
    ThreatCamp,
    ThreatCampDisabled,
    Passage,
    Ascent,
    Descent,
}

impl Decor {
    pub fn label(self) -> &'static str {
        match self {
            Self::Deck => "Dallage · passage libre",
            Self::Grate => "Caillebotis · passage libre",
            Self::Lane => "Voie de circulation · passage libre",
            Self::Threshold => "Seuil ouvert · passage libre",
            Self::ForeignFloor => "Substrat étranger · passage libre",
            Self::VeinedFloor => "Veine minérale · passage libre",
            Self::ChitinFloor => "Plaque chitineuse · passage libre",
            Self::PulseChannel => "Canal pulsatile · passage libre",
            Self::MemoryFloor => "Bloc mémoire stable · passage libre",
            Self::WindowFrame => "Cadre d'interface brisé · passage libre",
            Self::FaultTrace => "Erreur d'exécution · passage libre",
            Self::Gravel => "Terrain extérieur · passage libre",
            Self::Grass => "Prairie sauvage · passage libre",
            Self::Scrub => "Broussailles sèches · passage libre",
            Self::Mud => "Sol humide · passage libre",
            Self::ShallowWater => "Eau peu profonde · traversable",
            Self::DeepWater => "Eau profonde · passage bloqué, vue libre",
            Self::Tree => "Arbre · passage et vue bloqués",
            Self::Boulder => "Bloc rocheux · passage et vue bloqués",
            Self::RuinFloor => "Ruines habitées autrefois · passage libre",
            Self::RuinWall => "Mur en ruine · passage et vue bloqués",
            Self::Barricade => "Ancien accès barricadé · passage bloqué",
            Self::Wall => "Cloison · passage et vue bloqués",
            Self::MembraneWall => "Masse étrangère · passage et vue bloqués",
            Self::VoidWall => "Masse creuse · passage et vue bloqués",
            Self::DeadScreen => "Écran mort · passage et vue bloqués",
            Self::Pillar => "Pilier · passage et vue bloqués",
            Self::Crate => "Conteneur · passage et vue bloqués",
            Self::Server => "Baie serveur · passage et vue bloqués",
            Self::Console => "Pupitre inactif · obstacle",
            Self::Coolant => "Cuve · passage et vue bloqués",
            Self::Resonator => "Résonateur étranger · obstacle",
            Self::GrowthNode => "Nœud minéral · obstacle",
            Self::EyeNode => "Œil dormant · obstacle",
            Self::RootMass => "Racine calcifiée · obstacle",
            Self::KernelFault => "Faute noyau · obstacle",
            Self::OrphanProcess => "Processus orphelin · obstacle",
            Self::ClinicBed => "Lit de soin · clinique locale",
            Self::ClinicCounter => "Comptoir médical · clinique locale",
            Self::DoorClosed => "Porte fermée · interagir pour ouvrir",
            Self::DoorOpen => "Porte ouverte · interagir pour fermer",
            Self::DoorLocked => "Accès verrouillé · autorisation nécessaire",
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
            Self::DataTerminalOffline => "Terminal de données · hors ligne",
            Self::DataTerminalOnline => "Terminal de données · interagir pour consulter",
            Self::DataTerminalUpdated => {
                "Terminal de données · registre mis à jour, interagir pour consulter"
            }
            Self::DirectionBoard => "Panneau d'orientation · interagir pour lire",
            Self::DirectionBoardUpdated => "Panneau d'orientation · indication corrigée",
            Self::ServicePlan => "Plan du relais · interagir pour lire",
            Self::SupplyCache => "Cache de récupération · contenu ramassable sur place",
            Self::ThreatCamp => {
                "Camp hostile actif · interagir à côté pour neutraliser les renforts"
            }
            Self::ThreatCampDisabled => "Camp hostile neutralisé · aucun nouveau renfort",
            Self::Passage => "Passage vers une autre zone · interagir pour voyager",
            Self::Ascent => "Accès vers la couche supérieure · interagir pour monter",
            Self::Descent => "Accès vers la couche inférieure · interagir pour descendre",
        }
    }
    pub const fn blocks(self) -> bool {
        !matches!(
            self,
            Self::Deck
                | Self::Grate
                | Self::Lane
                | Self::Threshold
                | Self::ForeignFloor
                | Self::VeinedFloor
                | Self::ChitinFloor
                | Self::PulseChannel
                | Self::MemoryFloor
                | Self::WindowFrame
                | Self::FaultTrace
                | Self::Gravel
                | Self::Grass
                | Self::Scrub
                | Self::Mud
                | Self::ShallowWater
                | Self::RuinFloor
                | Self::SupplyCache
                | Self::ThreatCamp
                | Self::ThreatCampDisabled
                | Self::DoorOpen
                | Self::Passage
                | Self::Ascent
                | Self::Descent
        )
    }

    const fn terrain(self) -> Terrain {
        match self {
            Self::ShallowWater => Terrain::ShallowWater,
            Self::DeepWater => Terrain::DeepWater,
            _ if self.blocks() => Terrain::Wall,
            _ => Terrain::Floor,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Zone {
    pub name: String,
    pub bounds: [i32; 4],
}
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SectorDecor {
    pub cells: BTreeMap<GridPos, Decor>,
    pub zones: Vec<Zone>,
    pub biomes: BTreeMap<GridPos, String>,
    pub fallback_name: Option<String>,
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
            .map(|zone| zone.name.as_str())
            .or_else(|| self.biomes.get(&position).map(String::as_str))
            .or(self.fallback_name.as_deref())
            .unwrap_or("Friches extérieures")
    }
}

pub struct TestSector {
    pub level: GeneratedMap,
    pub decor: SectorDecor,
    pub enemies: Vec<GridPos>,
    pub loot: Vec<GridPos>,
}

impl TestSector {
    /// Authored, optional v100 contacts: one on the plaza, one in a protected
    /// roadside shelter. This never changes a pre-v100 generated map.
    pub fn with_surface_contacts(mut self) -> Result<Self, String> {
        let mut map = self.level.map().clone();
        paint(&mut map, &mut self.decor, [66, 24, 7, 6], Decor::Wall)?;
        paint(&mut map, &mut self.decor, [67, 25, 5, 4], Decor::Deck)?;
        paint(&mut map, &mut self.decor, [69, 21, 1, 4], Decor::Lane)?;
        for y in 25..29 {
            for x in 67..72 {
                map.set_protected(GridPos::new(x, y), true)
                    .map_err(|e| e.to_string())?;
            }
        }
        self.decor.zones.insert(
            0,
            Zone {
                name: "Abri de la lisière".to_owned(),
                bounds: [66, 24, 7, 6],
            },
        );
        let required: Vec<_> = self
            .enemies
            .iter()
            .chain(&self.loot)
            .copied()
            .chain([GridPos::new(68, 26), GridPos::new(36, 26)])
            .chain(Self::EXPANDED_REGIONAL_PASSAGES.iter().map(|(_, pos)| *pos))
            .collect();
        self.level = GeneratedMap::from_layout(
            map,
            self.level.player_start(),
            self.level.exit(),
            &required,
            MapValidationRules::default(),
        )
        .map_err(|e| format!("Surface contacts are unreachable: {e}"))?;
        Ok(self)
    }

    pub const NAME: &str = "VILLE DE DÉPART / FRICHES";
    pub const GATE: GridPos = GridPos::new(62, 21);
    pub const LOCKED_DOOR: GridPos = GridPos::new(52, 15);
    pub const QUEST_ACCESS_DOOR: GridPos = GridPos::new(32, 38);
    pub const CONTROL: GridPos = GridPos::new(50, 17);
    pub const RECYCLING_BOUNDS: [i32; 4] = [72, 4, 24, 15];
    pub const RECYCLING_START: GridPos = GridPos::new(82, 11);
    pub const RECYCLING_MAIN_DOOR: GridPos = GridPos::new(72, 11);
    pub const RECYCLING_CONTROL: GridPos = GridPos::new(76, 7);
    pub const RECYCLING_CONDUIT: GridPos = GridPos::new(88, 18);
    pub const RECYCLING_REPAIR_PART: GridPos = GridPos::new(86, 11);
    #[cfg(any(debug_assertions, test))]
    pub const ARCHIVE_TERMINAL: GridPos = GridPos::new(48, 7);
    #[cfg(any(debug_assertions, test))]
    pub const LEGACY_EXPEDITION_PASSAGE: GridPos = GridPos::new(67, 21);
    pub const EXPANDED_EXPEDITION_PASSAGE: GridPos = GridPos::new(176, 108);
    pub const EXPANDED_REGIONAL_PASSAGES: [(Direction, GridPos); 4] = [
        (Direction::North, GridPos::new(96, 2)),
        (Direction::East, GridPos::new(189, 64)),
        (Direction::South, GridPos::new(96, 125)),
        (Direction::West, GridPos::new(2, 64)),
    ];
    pub const EXPANDED_WIDTH: usize = 192;
    pub const EXPANDED_HEIGHT: usize = 128;

    #[cfg(test)]
    pub fn build(seed: u64) -> Result<Self, String> {
        Self::build_for_generation(seed, true, true, true, true, true)
    }

    pub fn build_for_generation(
        seed: u64,
        expanded: bool,
        include_regional_passages: bool,
        include_clinic: bool,
        include_quest_access: bool,
        include_recycling_intro: bool,
    ) -> Result<Self, String> {
        if expanded {
            Self::build_expanded(
                seed,
                include_regional_passages,
                include_clinic,
                include_quest_access,
                include_recycling_intro,
            )
        } else {
            Self::build_legacy(seed, include_clinic, include_quest_access)
        }
    }

    fn build_legacy(
        seed: u64,
        include_clinic: bool,
        include_quest_access: bool,
    ) -> Result<Self, String> {
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
            (
                if include_clinic {
                    "Clinique"
                } else {
                    "Accueil"
                },
                [6, 5, 14, 11],
                Decor::Deck,
                GridPos::new(12, 15),
            ),
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
            decor.zones.push(Zone {
                name: name.to_owned(),
                bounds,
            });
        }
        decor.zones.push(Zone {
            name: "Place centrale".to_owned(),
            bounds: [21, 16, 25, 12],
        });
        decor.zones.push(Zone {
            name: "Rues de la ville".to_owned(),
            bounds: [1, 1, 62, 44],
        });
        for bounds in [[30, 20, 3, 3], [9, 31, 2, 2], [16, 34, 2, 2]] {
            paint(&mut map, &mut decor, bounds, Decor::Pillar)?;
        }
        if include_clinic {
            paint(&mut map, &mut decor, [8, 7, 4, 2], Decor::ClinicBed)?;
            paint(&mut map, &mut decor, [15, 7, 3, 1], Decor::ClinicCounter)?;
        }
        let consoles = if include_clinic {
            &[[28, 7, 4, 2], [48, 35, 5, 1]][..]
        } else {
            &[[8, 7, 5, 1], [28, 7, 4, 2], [48, 35, 5, 1]][..]
        };
        for &bounds in consoles {
            paint(&mut map, &mut decor, bounds, Decor::Console)?;
        }
        for bounds in [[28, 31, 3, 2], [35, 35, 3, 2]] {
            paint(&mut map, &mut decor, bounds, Decor::Crate)?;
        }
        if include_quest_access {
            // Optional rear access to the depot: the ordinary front door keeps
            // the building reachable, while quest completion can unlock this
            // persistent shortcut without gating a merchant or clinic.
            map.set_terrain(Self::QUEST_ACCESS_DOOR, Terrain::Door(DoorState::Locked))
                .map_err(|error| error.to_string())?;
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

    fn build_expanded(
        seed: u64,
        include_regional_passages: bool,
        include_clinic: bool,
        include_quest_access: bool,
        include_recycling_intro: bool,
    ) -> Result<Self, String> {
        let legacy = Self::build_legacy(seed, include_clinic, include_quest_access)?;
        let mut map = Map::filled(Self::EXPANDED_WIDTH, Self::EXPANDED_HEIGHT, Terrain::Wall)
            .map_err(|error| error.to_string())?;
        let mut decor = SectorDecor::default();
        paint_biome(
            &mut map,
            &mut decor,
            [
                1,
                1,
                Self::EXPANDED_WIDTH as i32 - 2,
                Self::EXPANDED_HEIGHT as i32 - 2,
            ],
            Decor::Gravel,
            "Friches sèches",
        )?;

        // Preserve every authored town tile exactly while replacing only the
        // procedural land beyond its eastern and southern limits.
        for y in 0..=44 {
            for x in 0..=62 {
                let position = GridPos::new(x, y);
                let tile = legacy
                    .level
                    .map()
                    .tile(position)
                    .ok_or("Legacy town tile missing")?;
                map.set_terrain(position, tile.terrain)
                    .map_err(|error| error.to_string())?;
                map.set_protected(position, tile.protected)
                    .map_err(|error| error.to_string())?;
                if let Some(kind) = legacy.decor.cells.get(&position) {
                    decor.cells.insert(position, *kind);
                } else {
                    decor.cells.remove(&position);
                }
                decor.biomes.remove(&position);
            }
        }
        decor.zones = legacy.decor.zones;

        let mut rng = GameRng::from_seed(seed ^ 0x5355_5246_4143_4532);
        for (base_x, base_y, radius_x, radius_y, kind, name) in [
            (
                88,
                18,
                24,
                14,
                Decor::Grass,
                "Prairies de l'ancienne ceinture",
            ),
            (142, 29, 34, 20, Decor::Scrub, "Lande de broussailles"),
            (104, 70, 30, 22, Decor::Mud, "Marais des conduites"),
            (155, 91, 31, 25, Decor::Grass, "Bois de récupération"),
            (78, 108, 20, 14, Decor::Scrub, "Plateau rocheux"),
        ] {
            let offset_x = rng.usize_inclusive(0, 8).unwrap() as i32 - 4;
            let offset_y = rng.usize_inclusive(0, 8).unwrap() as i32 - 4;
            paint_biome_ellipse(
                &mut map,
                &mut decor,
                GridPos::new(base_x + offset_x, base_y + offset_y),
                radius_x,
                radius_y,
                kind,
                name,
            )?;
        }

        // Several real bodies of water, with traversable banks and deep cores.
        // Their positions vary by seed but remain in broad authored regions.
        for (base_x, base_y) in [(93, 66), (119, 92), (158, 57), (72, 105)] {
            let center = GridPos::new(
                base_x + rng.usize_inclusive(0, 10).unwrap() as i32 - 5,
                base_y + rng.usize_inclusive(0, 8).unwrap() as i32 - 4,
            );
            let radius_x = rng.usize_inclusive(7, 13).unwrap() as i32;
            let radius_y = rng.usize_inclusive(4, 8).unwrap() as i32;
            paint_biome_ellipse(
                &mut map,
                &mut decor,
                center,
                radius_x,
                radius_y,
                Decor::ShallowWater,
                "Point d'eau",
            )?;
            paint_biome_ellipse(
                &mut map,
                &mut decor,
                center,
                (radius_x - 3).max(2),
                (radius_y - 2).max(1),
                Decor::DeepWater,
                "Point d'eau",
            )?;
        }

        for (origin, size) in [
            (GridPos::new(104, 37), [15, 10]),
            (GridPos::new(148, 14), [18, 12]),
            (GridPos::new(74, 78), [13, 11]),
        ] {
            paint_ruin(&mut map, &mut decor, origin, size)?;
        }

        let mut enemies = vec![
            GridPos::new(74, 21),
            GridPos::new(91, 14),
            GridPos::new(85, 36),
            GridPos::new(99, 51),
            GridPos::new(71, 55),
            GridPos::new(127, 72),
            GridPos::new(153, 34),
            GridPos::new(168, 91),
            GridPos::new(138, 113),
            GridPos::new(181, 69),
        ];
        if include_recycling_intro {
            // The old second encounter occupied the new wake-up room. Moving
            // it outside preserves the enemy family while leaving both exits
            // available without a mandatory opening fight.
            enemies[1] = GridPos::new(99, 16);
        }
        let loot = vec![GridPos::new(35, 24), GridPos::new(28, 25)];
        let exit = Self::EXPANDED_EXPEDITION_PASSAGE;
        let regional_passages = include_regional_passages
            .then_some(Self::EXPANDED_REGIONAL_PASSAGES)
            .into_iter()
            .flatten()
            .map(|(_, position)| position);
        let required: Vec<_> = enemies
            .iter()
            .chain(&loot)
            .copied()
            .chain([exit])
            .chain(regional_passages)
            .collect();

        // Trees and boulders create tactical sight lines without becoming a
        // solid random wall. Their broad habitat comes from the biome map.
        for _ in 0..720 {
            let position = GridPos::new(
                rng.usize_inclusive(65, Self::EXPANDED_WIDTH - 3).unwrap() as i32,
                rng.usize_inclusive(3, Self::EXPANDED_HEIGHT - 3).unwrap() as i32,
            );
            if required
                .iter()
                .any(|required| grid_distance(*required, position) <= 2)
                || !map.is_walkable(position)
                || matches!(
                    decor.cells.get(&position),
                    Some(Decor::Lane | Decor::RuinFloor)
                )
            {
                continue;
            }
            let biome = decor
                .biomes
                .get(&position)
                .map(String::as_str)
                .unwrap_or_default();
            let kind =
                if biome == "Bois de récupération" || biome == "Prairies de l'ancienne ceinture" {
                    (!rng.next_u64().is_multiple_of(3)).then_some(Decor::Tree)
                } else if biome == "Plateau rocheux" || biome == "Lande de broussailles" {
                    rng.next_u64().is_multiple_of(4).then_some(Decor::Boulder)
                } else {
                    None
                };
            if let Some(kind) = kind {
                map.set_terrain(position, kind.terrain())
                    .map_err(|error| error.to_string())?;
                decor.cells.insert(position, kind);
            }
        }

        // A readable but non-linear old service road guarantees a route from
        // the city to the distant descent. Drawing it last lets it bridge water
        // and cut through vegetation without depending on a lucky seed.
        for bounds in [
            [63, 20, 60, 3],
            [120, 20, 3, 60],
            [120, 77, 57, 3],
            [174, 77, 3, 33],
        ] {
            paint(&mut map, &mut decor, bounds, Decor::Lane)?;
        }
        if include_recycling_intro {
            paint_recycling_intro(&mut map, &mut decor)?;
        }
        for position in &required {
            clear_required_position(&mut map, &mut decor, *position)?;
        }
        if include_regional_passages {
            for (_, position) in Self::EXPANDED_REGIONAL_PASSAGES {
                decor.cells.insert(position, Decor::Passage);
            }
        }

        let start = if include_recycling_intro {
            Self::RECYCLING_START
        } else {
            GridPos::new(27, 23)
        };
        let level =
            GeneratedMap::from_layout(map, start, exit, &required, MapValidationRules::default())
                .map_err(|error| format!("Grande carte ville/extérieur invalide : {error}"))?;
        Ok(Self {
            level,
            decor,
            enemies,
            loot,
        })
    }
}

fn paint_recycling_intro(map: &mut Map, decor: &mut SectorDecor) -> Result<(), String> {
    // A fixed, compact opening space inside the seeded exterior. It is
    // deliberately authored after the biomes so every new run teaches the
    // same first choices before opening onto the variable surface.
    paint(map, decor, TestSector::RECYCLING_BOUNDS, Decor::Wall)?;
    paint(map, decor, [73, 5, 22, 13], Decor::Grate)?;

    // Dismantling fixtures give the room a functional silhouette while
    // keeping a generous central lane around the player's chassis.
    for bounds in [[74, 5, 5, 1], [91, 6, 3, 3], [74, 14, 2, 3]] {
        paint(map, decor, bounds, Decor::Server)?;
    }
    for bounds in [[79, 6, 2, 2], [91, 14, 3, 2]] {
        paint(map, decor, bounds, Decor::Crate)?;
    }
    paint(map, decor, [84, 6, 4, 1], Decor::Console)?;

    map.set_terrain(
        TestSector::RECYCLING_MAIN_DOOR,
        Terrain::Door(DoorState::Locked),
    )
    .map_err(|error| error.to_string())?;
    map.set_terrain(
        TestSector::RECYCLING_CONTROL,
        Terrain::ControlPanel {
            door: TestSector::RECYCLING_MAIN_DOOR,
            activated: false,
        },
    )
    .map_err(|error| error.to_string())?;

    // Main route: rearm the western access, then follow the old service lane
    // down to the city road. This is the shortest and clearest path.
    paint(map, decor, [63, 10, 9, 3], Decor::Lane)?;
    paint(map, decor, [63, 10, 3, 13], Decor::Lane)?;

    // Alternate route: a narrow southern conduit is always open. It asks for
    // exploration rather than a skill, tool or inventory check.
    map.set_terrain(TestSector::RECYCLING_CONDUIT, Terrain::Floor)
        .map_err(|error| error.to_string())?;
    decor
        .cells
        .insert(TestSector::RECYCLING_CONDUIT, Decor::Threshold);
    paint(map, decor, [87, 19, 3, 4], Decor::Grate)?;

    decor.zones.push(Zone {
        name: "Zone de recyclage".to_owned(),
        bounds: TestSector::RECYCLING_BOUNDS,
    });
    decor.zones.push(Zone {
        name: "Conduit de maintenance".to_owned(),
        bounds: [87, 18, 3, 5],
    });
    Ok(())
}

fn paint_biome(
    map: &mut Map,
    decor: &mut SectorDecor,
    bounds: [i32; 4],
    kind: Decor,
    name: &'static str,
) -> Result<(), String> {
    paint(map, decor, bounds, kind)?;
    let [x, y, width, height] = bounds;
    for py in y..y + height {
        for px in x..x + width {
            decor.biomes.insert(GridPos::new(px, py), name.to_owned());
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn paint_biome_ellipse(
    map: &mut Map,
    decor: &mut SectorDecor,
    center: GridPos,
    radius_x: i32,
    radius_y: i32,
    kind: Decor,
    name: &'static str,
) -> Result<(), String> {
    let radius_x_squared = i64::from(radius_x) * i64::from(radius_x);
    let radius_y_squared = i64::from(radius_y) * i64::from(radius_y);
    let limit = radius_x_squared * radius_y_squared;
    for y in center.y - radius_y..=center.y + radius_y {
        for x in center.x - radius_x..=center.x + radius_x {
            let position = GridPos::new(x, y);
            if x <= 62
                || x <= 0
                || y <= 0
                || x >= map.width() as i32 - 1
                || y >= map.height() as i32 - 1
                || map.is_protected(position)
            {
                continue;
            }
            let dx = i64::from(x - center.x);
            let dy = i64::from(y - center.y);
            if dx * dx * radius_y_squared + dy * dy * radius_x_squared <= limit {
                map.set_terrain(position, kind.terrain())
                    .map_err(|error| error.to_string())?;
                decor.cells.insert(position, kind);
                decor.biomes.insert(position, name.to_owned());
            }
        }
    }
    Ok(())
}

fn paint_ruin(
    map: &mut Map,
    decor: &mut SectorDecor,
    origin: GridPos,
    size: [i32; 2],
) -> Result<(), String> {
    let [width, height] = size;
    paint_biome(
        map,
        decor,
        [origin.x, origin.y, width, height],
        Decor::RuinFloor,
        "Ruines de surface",
    )?;
    paint(map, decor, [origin.x, origin.y, width, 1], Decor::RuinWall)?;
    paint(
        map,
        decor,
        [origin.x, origin.y + height - 1, width, 1],
        Decor::RuinWall,
    )?;
    paint(map, decor, [origin.x, origin.y, 1, height], Decor::RuinWall)?;
    paint(
        map,
        decor,
        [origin.x + width - 1, origin.y, 1, height],
        Decor::RuinWall,
    )?;
    let entrance = GridPos::new(origin.x + width / 2, origin.y + height - 1);
    map.set_terrain(entrance, Terrain::Floor)
        .map_err(|error| error.to_string())?;
    decor.cells.insert(entrance, Decor::RuinFloor);
    Ok(())
}

fn clear_required_position(
    map: &mut Map,
    decor: &mut SectorDecor,
    center: GridPos,
) -> Result<(), String> {
    if map.is_protected(center) {
        return Ok(());
    }
    for y in center.y - 1..=center.y + 1 {
        for x in center.x - 1..=center.x + 1 {
            let position = GridPos::new(x, y);
            if x <= 0
                || y <= 0
                || x >= map.width() as i32 - 1
                || y >= map.height() as i32 - 1
                || map.is_protected(position)
            {
                continue;
            }
            map.set_terrain(position, Terrain::Floor)
                .map_err(|error| error.to_string())?;
            if !matches!(decor.cells.get(&position), Some(Decor::Lane)) {
                decor.cells.insert(position, Decor::Gravel);
            }
        }
    }
    Ok(())
}

fn grid_distance(left: GridPos, right: GridPos) -> u32 {
    left.x.abs_diff(right.x).max(left.y.abs_diff(right.y))
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
            map.set_terrain(p, kind.terrain())
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
    fn surface_contacts_keep_routes_solvable_and_enemies_outside_the_shelter() {
        for seed in 1..17 {
            let sector = TestSector::build(seed)
                .unwrap()
                .with_surface_contacts()
                .unwrap();
            for contact in [GridPos::new(36, 26), GridPos::new(68, 26)] {
                assert!(sector.level.map().is_protected(contact));
            }
            assert!(
                sector
                    .enemies
                    .iter()
                    .all(|at| !sector.level.map().is_protected(*at))
            );
        }
    }

    #[test]
    fn town_is_fixed_exterior_varies_and_all_interactive_routes_are_solvable() {
        let first = TestSector::build(1).unwrap();
        assert_eq!(first.level.map().width(), TestSector::EXPANDED_WIDTH);
        assert_eq!(first.level.map().height(), TestSector::EXPANDED_HEIGHT);
        assert_eq!(first.level.exit(), TestSector::EXPANDED_EXPEDITION_PASSAGE);
        assert_eq!(
            first
                .level
                .map()
                .tile(TestSector::QUEST_ACCESS_DOOR)
                .map(|tile| tile.terrain),
            Some(Terrain::Door(DoorState::Locked))
        );
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
    fn expanded_surface_contains_distinct_water_vegetation_and_ruin_regions() {
        let sector = TestSector::build(20_260_909).unwrap();
        for kind in [
            Decor::Grass,
            Decor::Scrub,
            Decor::Mud,
            Decor::ShallowWater,
            Decor::DeepWater,
            Decor::Tree,
            Decor::Boulder,
            Decor::RuinFloor,
            Decor::RuinWall,
        ] {
            assert!(
                sector
                    .decor
                    .cells
                    .values()
                    .any(|candidate| *candidate == kind),
                "expanded surface did not contain {kind:?}"
            );
        }
        let deep_water = sector
            .decor
            .cells
            .iter()
            .find_map(|(position, kind)| (*kind == Decor::DeepWater).then_some(*position))
            .unwrap();
        assert!(!sector.level.map().is_walkable(deep_water));
        assert!(!sector.level.map().blocks_vision(deep_water));
        assert!(
            sector
                .decor
                .biomes
                .values()
                .any(|name| *name == "Point d'eau")
        );
        assert!(
            sector
                .decor
                .biomes
                .values()
                .any(|name| *name == "Bois de récupération")
        );
    }

    #[test]
    fn recycling_intro_has_two_independent_routes_toward_the_city() {
        let sector = TestSector::build(20_260_920).unwrap();
        assert_eq!(sector.level.player_start(), TestSector::RECYCLING_START);
        assert_eq!(
            sector
                .level
                .map()
                .tile(TestSector::RECYCLING_MAIN_DOOR)
                .map(|tile| tile.terrain),
            Some(Terrain::Door(DoorState::Locked))
        );
        assert_eq!(
            sector
                .level
                .map()
                .tile(TestSector::RECYCLING_CONTROL)
                .map(|tile| tile.terrain),
            Some(Terrain::ControlPanel {
                door: TestSector::RECYCLING_MAIN_DOOR,
                activated: false,
            })
        );
        assert_eq!(
            sector.decor.zone_at(TestSector::RECYCLING_START),
            "Zone de recyclage"
        );

        let city_approach = GridPos::new(63, 21);
        let mut main_route_only = sector.level.map().clone();
        main_route_only
            .set_terrain(TestSector::RECYCLING_CONDUIT, Terrain::Wall)
            .unwrap();
        validate_interactive_map(
            &main_route_only,
            TestSector::RECYCLING_START,
            city_approach,
            &[],
            MapValidationRules::default(),
        )
        .expect("the interior control must make the main route independently solvable");

        let mut conduit_route_only = sector.level.map().clone();
        conduit_route_only
            .set_terrain(TestSector::RECYCLING_MAIN_DOOR, Terrain::Wall)
            .unwrap();
        conduit_route_only
            .set_terrain(TestSector::RECYCLING_CONTROL, Terrain::Wall)
            .unwrap();
        validate_interactive_map(
            &conduit_route_only,
            TestSector::RECYCLING_START,
            city_approach,
            &[],
            MapValidationRules::default(),
        )
        .expect("the conduit must remain usable without the main door");
    }

    #[test]
    fn expanded_pre_intro_generation_keeps_the_city_start() {
        let sector = TestSector::build_for_generation(1, true, true, true, true, false).unwrap();
        assert_eq!(sector.level.player_start(), GridPos::new(27, 23));
        assert_ne!(
            sector.decor.zone_at(TestSector::RECYCLING_START),
            "Zone de recyclage"
        );
    }

    #[test]
    fn legacy_surface_dimensions_and_passage_remain_available_for_replay() {
        let sector =
            TestSector::build_for_generation(1, false, false, false, false, false).unwrap();
        assert_eq!(sector.level.map().width(), 110);
        assert_eq!(sector.level.map().height(), 68);
        assert_eq!(sector.level.exit(), GridPos::new(103, 59));
        assert_eq!(TestSector::LEGACY_EXPEDITION_PASSAGE, GridPos::new(67, 21));
        assert_eq!(
            sector
                .level
                .map()
                .tile(TestSector::QUEST_ACCESS_DOOR)
                .map(|tile| tile.terrain),
            Some(Terrain::Wall)
        );
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
