//! Numbered captures using the active renderer and its exact native assets.
use super::*;
use crate::game_art::{Visual, WeaponHold};
use project_rl::entity::EquipmentPalette;

const CRAWL_NAMES: [&str; 17] = [
    "BRÈCHE",
    "VIGIE",
    "CREUSET",
    "Cerbère",
    "Cognefer",
    "Chrysalide",
    "Azurite",
    "Fantôme",
    "Robe de mue",
    "Bastion",
    "Ferrailleur",
    "Seconde Ombre",
    "Membranes",
    "Peau-Seconde",
    "Technicien",
    "Aviateur",
    "Nomade",
];

fn figure(
    outfit: usize,
    weapon: Option<u16>,
    hands: Option<[Option<u16>; 2]>,
    colors: [EquipmentPalette; 3],
) -> Visual {
    Visual::Player {
        body: outfit.min(2) as u8,
        equipment: [
            (outfit >= 3).then_some((98 + outfit) as u16),
            None,
            None,
            None,
        ],
        weapon,
        hands,
        palettes: colors,
    }
}
fn tile_background(x: f32, y: f32, side: f32) {
    crate::terminal_view::draw_scenery_preview(
        Rect::new(x, y, side, side),
        crate::test_sector::Decor::Deck,
        GridPos::new(3, 3),
    );
}
impl AsciiApp {
    fn draw_bestiary_bank_review(&self, scene: &str) {
        clear_background(Color::from_rgba(17, 25, 31, 255));
        let text = Color::from_rgba(185, 225, 220, 255);
        draw_text_bold("BESTIAIRE / RESSOURCES INTÉGRÉES", 24.0, 32.0, 25.0, text);
        draw_text(
            "Vivant + cadavre natifs / agrandissement / ombres séparées / loot au-dessus",
            24.0,
            59.0,
            18.0,
            text,
        );
        let manifest: serde_json::Value =
            serde_json::from_str(include_str!("../assets/sprites/bestiary/manifest.json")).unwrap();
        let (first, last) = match scene {
            "crawl-bank-1" => (1, 16),
            "crawl-bank-2" => (17, 32),
            "crawl-bank-3" => (33, 55),
            _ => (56, 66),
        };
        let columns = if first == 33 { 5 } else { 4 };
        let width = if columns == 5 { 250.0 } else { 312.0 };
        let height = if columns == 5 { 139.0 } else { 172.0 };
        for (position, entry) in manifest["entries"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|e| (first..=last).contains(&e["number"].as_u64().unwrap()))
            .enumerate()
        {
            let number = entry["number"].as_u64().unwrap() as u8;
            let side = entry["tile_size"].as_u64().unwrap() as f32;
            let x = 18.0 + (position % columns) as f32 * width;
            let y = 82.0 + (position / columns) as f32 * height;
            let name = entry["name"].as_str().unwrap();
            draw_rectangle(
                x,
                y,
                width - 8.0,
                height - 8.0,
                Color::from_rgba(27, 38, 45, 255),
            );
            draw_text(
                &format!("{number:02} {name}"),
                x + 8.0,
                y + 23.0,
                if columns == 5 { 14.0 } else { 16.0 },
                text,
            );
            for corpse_case in 0..if first == 33 { 3 } else { 2 } {
                let dead = corpse_case > 0;
                let left = x
                    + 12.0
                    + match corpse_case {
                        0 => 0.0,
                        1 if first == 33 => 38.0,
                        _ => 78.0,
                    };
                let top = y + 48.0;
                for row in 0..(side as u32 / 32) {
                    for column in 0..(side as u32 / 32) {
                        tile_background(left + column as f32 * 32.0, top + row as f32 * 32.0, 32.0);
                    }
                }
                let owning_cell =
                    Rect::new(left + (side - 32.0) * 0.5, top + side - 32.0, 32.0, 32.0);
                if dead && entry["legacy_actor_index"].is_u64() {
                    let actor = entry["legacy_actor_index"].as_u64().unwrap() as u8;
                    crate::combat_visuals::draw_residue(
                        owning_cell,
                        &crate::combat_visuals::ResidueCell {
                            corpse: Some(actor),
                            blood: 6,
                            ..Default::default()
                        },
                        true,
                    );
                    if corpse_case == 2 {
                        crate::game_art::draw_world(
                            owning_cell,
                            Visual::ColoredItem(
                                crate::game_art::item("core:repair_parts").unwrap(),
                                project_rl::entity::EquipmentPalette::Original,
                            ),
                            1.0,
                        );
                    }
                } else {
                    crate::game_art::draw_world(
                        owning_cell,
                        Visual::Bestiary {
                            number,
                            corpse: dead,
                        },
                        1.0,
                    );
                }
            }
            let zoom = if side == 64.0 {
                128.0
            } else if columns == 5 {
                64.0
            } else {
                96.0
            };
            crate::game_art::draw_portrait(
                Rect::new(
                    x + width - zoom - 16.0,
                    y + if side == 64.0 { 30.0 } else { 40.0 },
                    zoom,
                    zoom,
                ),
                Visual::Bestiary {
                    number,
                    corpse: false,
                },
            );
            draw_text(
                &format!(
                    "{} px / {}",
                    side as u32,
                    if entry["bleeds"].as_bool().unwrap() {
                        "sang"
                    } else {
                        "épave sèche"
                    }
                ),
                x + 8.0,
                y + height - 18.0,
                12.0,
                text,
            );
        }
    }

    pub(super) fn draw_crawl_review(&self, scene: &str) {
        if scene.starts_with("crawl-bank-") {
            self.draw_bestiary_bank_review(scene);
            return;
        }
        clear_background(Color::from_rgba(17, 25, 31, 255));
        let text = Color::from_rgba(185, 225, 220, 255);
        draw_text_bold("TILES INTÉGRÉES / RENDU ACTIF", 24.0, 32.0, 25.0, text);
        draw_text(
            if scene == "crawl-agents" {
                "Corps de 30 px · cases natives de 32 px · zoom ×2 nearest · arme hors collision"
            } else if scene.starts_with("crawl-grips") {
                "Prises corrigées · corps de 30 px · cases natives de 32 px · G / D = côté de l’écran"
            } else {
                "Corps de 30 px · cases de 32 px · zoom ×3 nearest · arme hors collision"
            },
            24.0,
            59.0,
            18.0,
            text,
        );
        let manifest: serde_json::Value =
            serde_json::from_str(include_str!("../assets/sprites/cyberpunk/manifest.json"))
                .unwrap();
        let held = manifest["held_weapons"].as_array().unwrap();
        let representative = |family: usize| {
            held.iter()
                .find(|m| m["family"].as_u64() == Some(family as u64))
                .map(|m| m["index"].as_u64().unwrap() as u16)
        };
        if scene == "crawl-agents" {
            let classes = [
                "Bastion",
                "Technomancien",
                "Traqueur",
                "Arcaniste",
                "Métamorphe",
            ];
            let bodies = [9, 2, 1, 11, 5];
            let starts = [3, 6, 17, 0, 0];
            let rows = [
                "Départ",
                "Main gauche",
                "Deux armes",
                "Arme lourde",
                "Gris neutre",
            ];
            for (row, label) in rows.into_iter().enumerate() {
                let y = 120.0 + row as f32 * 118.0;
                draw_text(label, 24.0, y + 48.0, 17.0, text);
                for class in 0..5 {
                    let x = 172.0 + class as f32 * 210.0;
                    if row == 0 {
                        draw_text(
                            &format!("{}. {}", class + 1, classes[class]),
                            x,
                            99.0,
                            17.0,
                            text,
                        );
                    }
                    let (weapon, hands) = match row {
                        0 => (representative(starts[class]), None),
                        1 => (None, Some([representative(6), None])),
                        2 | 4 => (None, Some([representative(6), representative(0)])),
                        _ => (representative(14), None),
                    };
                    let palettes = if row == 4 {
                        [EquipmentPalette::Gray; 3]
                    } else {
                        [EquipmentPalette::Original; 3]
                    };
                    let visual = figure(bodies[class], weapon, hands, palettes);
                    for (xx, yy, size) in [(x + 22.0, y + 32.0, 32.0), (x + 122.0, y + 16.0, 64.0)]
                    {
                        tile_background(xx, yy, size);
                        crate::game_art::draw_world(Rect::new(xx, yy, size, size), visual, 1.0);
                    }
                }
            }
            for (column, (id, palette)) in [
                ("core:modele_cogneur_p01", EquipmentPalette::Original),
                ("core:modele_cogneur_p01", EquipmentPalette::Gray),
                ("core:plastron_bastion", EquipmentPalette::Gray),
            ]
            .into_iter()
            .enumerate()
            {
                let x = 170.0 + column as f32 * 120.0;
                crate::game_art::draw_world(
                    Rect::new(x, 742.0, 32.0, 32.0),
                    Visual::ColoredItem(crate::game_art::item(id).unwrap(), palette),
                    1.0,
                );
            }
            draw_text("Objets au sol", 24.0, 764.0, 17.0, text);
        } else if scene.starts_with("crawl-grips-") {
            let one = [0, 1, 4, 5, 6, 7, 16];
            if scene == "crawl-grips-dual" {
                draw_text(
                    "Main à gauche : lignes / main à droite : colonnes · 49 associations",
                    24.0,
                    88.0,
                    18.0,
                    text,
                );
                for (row, left) in one.into_iter().enumerate() {
                    for (col, right) in one.into_iter().enumerate() {
                        let x = 110.0 + col as f32 * 156.0;
                        let y = 117.0 + row as f32 * 94.0;
                        if row == 0 {
                            draw_text(&format!("{:02}", right + 1), x, 109.0, 16.0, text);
                        }
                        if col == 0 {
                            draw_text(&format!("{:02}", left + 1), 72.0, y + 23.0, 16.0, text);
                        }
                        let hands = Some([representative(left), representative(right)]);
                        crate::game_art::draw_grip_review(
                            Rect::new(x, y, 32.0, 32.0),
                            5,
                            None,
                            hands,
                        );
                        crate::game_art::draw_grip_review(
                            Rect::new(x + 68.0, y, 64.0, 64.0),
                            5,
                            None,
                            hands,
                        );
                    }
                }
            } else {
                let single = scene.contains("-one-");
                let families: Vec<usize> = if single {
                    one.to_vec()
                } else {
                    vec![2, 3, 8, 9, 10, 11, 12, 13, 14, 15, 17]
                };
                let start = if scene.ends_with('2') { 12 } else { 0 };
                let end = (start + 12).min(23);
                for (column, family) in families.into_iter().enumerate() {
                    for side in 0..if single { 2 } else { 1 } {
                        let x = 155.0
                            + (column * if single { 2 } else { 1 } + side) as f32
                                * if single { 78.0 } else { 98.0 };
                        draw_text(
                            &format!(
                                "{:02}{}",
                                family + 1,
                                if single {
                                    if side == 0 { " G" } else { " D" }
                                } else {
                                    ""
                                }
                            ),
                            x,
                            93.0,
                            16.0,
                            text,
                        );
                        for body in start..end {
                            let y = 114.0 + (body - start) as f32 * 55.0;
                            if column == 0 && side == 0 {
                                let label = if body < 17 { CRAWL_NAMES[body] } else { "NPC" };
                                draw_text(
                                    &format!("{:02} {label}", body + 1),
                                    20.0,
                                    y + 23.0,
                                    14.0,
                                    text,
                                );
                            }
                            let weapon = representative(family);
                            let hands = if single {
                                Some(if side == 0 {
                                    [weapon, None]
                                } else {
                                    [None, weapon]
                                })
                            } else {
                                None
                            };
                            crate::game_art::draw_grip_review(
                                Rect::new(x, y, 32.0, 32.0),
                                body as u16,
                                weapon,
                                hands,
                            );
                        }
                    }
                }
            }
        } else if scene == "crawl-weapons" {
            for family in 0..18 {
                let x = 18.0 + (family % 6) as f32 * 210.0;
                let y = 82.0 + (family / 6) as f32 * 231.0;
                draw_rectangle(x, y, 202.0, 221.0, Color::from_rgba(27, 38, 45, 255));
                let name = manifest["weapon_families"][family]["name"]
                    .as_str()
                    .unwrap();
                draw_text(
                    &format!("{:02} {name}", family + 1),
                    x + 8.0,
                    y + 23.0,
                    16.0,
                    text,
                );
                let weapon = representative(family);
                let visual = figure(9, weapon, None, [EquipmentPalette::Original; 3]);
                tile_background(x + 13.0, y + 48.0, 96.0);
                crate::game_art::draw_world(Rect::new(x + 13.0, y + 48.0, 96.0, 96.0), visual, 1.0);
                tile_background(x + 140.0, y + 66.0, 32.0);
                crate::game_art::draw_world(
                    Rect::new(x + 140.0, y + 66.0, 32.0, 32.0),
                    visual,
                    1.0,
                );
                if let Some(index) = weapon {
                    let id = held
                        .iter()
                        .find(|m| m["index"].as_u64() == Some(u64::from(index)))
                        .unwrap()["id"]
                        .as_str()
                        .unwrap();
                    let item = crate::game_art::item(id).unwrap();
                    crate::game_art::draw_world(
                        Rect::new(x + 140.0, y + 132.0, 32.0, 32.0),
                        Visual::ColoredItem(item, project_rl::entity::EquipmentPalette::Original),
                        1.0,
                    );
                    let hold = crate::game_art::weapon_hold(index);
                    draw_text(
                        if matches!(hold, WeaponHold::TwoHanded | WeaponHold::MeleeTwoHanded) {
                            "Deux mains"
                        } else {
                            "Une main"
                        },
                        x + 8.0,
                        y + 191.0,
                        16.0,
                        text,
                    );
                }
                draw_text("×3           1:1 / sol", x + 14.0, y + 215.0, 14.0, text);
            }
        } else if scene.starts_with("crawl-palettes") {
            let start = if scene.ends_with('2') { 9 } else { 0 };
            let end = (start + 9).min(17);
            for outfit in start..end {
                let y = 96.0 + (outfit - start) as f32 * 75.0;
                draw_text(
                    &format!("{:02} {}", outfit + 1, CRAWL_NAMES[outfit]),
                    22.0,
                    y + 29.0,
                    17.0,
                    text,
                );
                for (col, color) in EquipmentPalette::ALL.into_iter().enumerate() {
                    let x = 210.0 + col as f32 * 150.0;
                    tile_background(x, y, 64.0);
                    crate::game_art::draw_world(
                        Rect::new(x, y, 64.0, 64.0),
                        figure(
                            outfit,
                            None,
                            None,
                            [
                                color,
                                EquipmentPalette::Original,
                                EquipmentPalette::Original,
                            ],
                        ),
                        1.0,
                    );
                    crate::game_art::draw_world(
                        Rect::new(x + 78.0, y + 20.0, 32.0, 32.0),
                        figure(
                            outfit,
                            None,
                            None,
                            [
                                color,
                                EquipmentPalette::Original,
                                EquipmentPalette::Original,
                            ],
                        ),
                        1.0,
                    );
                    if outfit == start {
                        draw_text(color.name(), x, 83.0, 16.0, text);
                    }
                }
            }
        } else if scene == "crawl-outfits" {
            for (outfit, name) in CRAWL_NAMES.iter().enumerate() {
                let x = 18.0 + (outfit % 6) as f32 * 210.0;
                let y = 82.0 + (outfit / 6) as f32 * 231.0;
                draw_rectangle(x, y, 202.0, 221.0, Color::from_rgba(27, 38, 45, 255));
                draw_text(
                    &format!("{:02} {name}", outfit + 1),
                    x + 8.0,
                    y + 23.0,
                    17.0,
                    text,
                );
                crate::game_art::draw_world(
                    Rect::new(x + 44.0, y + 45.0, 96.0, 96.0),
                    figure(outfit, None, None, [EquipmentPalette::Original; 3]),
                    1.0,
                );
                for (col, w) in [representative(1), representative(6), representative(8)]
                    .into_iter()
                    .enumerate()
                {
                    tile_background(x + 18.0 + col as f32 * 62.0, y + 170.0, 32.0);
                    crate::game_art::draw_world(
                        Rect::new(x + 18.0 + col as f32 * 62.0, y + 170.0, 32.0, 32.0),
                        figure(outfit, w, None, [EquipmentPalette::Original; 3]),
                        1.0,
                    );
                }
                draw_text(
                    "Épée / pistolet / fusil · 1:1",
                    x + 8.0,
                    y + 215.0,
                    13.0,
                    text,
                );
            }
        } else if scene == "crawl-bestiary" {
            for index in 3..27 {
                let n = index - 3;
                let x = 18.0 + (n % 6) as f32 * 210.0;
                let y = 83.0 + (n / 6) as f32 * 177.0;
                draw_rectangle(x, y, 202.0, 167.0, Color::from_rgba(27, 38, 45, 255));
                let visual = Visual::Actor(index as u8);
                draw_text(
                    &format!(
                        "{:02} {}",
                        n + 1,
                        if index == 26 {
                            "Zombie (réserve)"
                        } else {
                            "Bestiaire / NPC"
                        }
                    ),
                    x + 8.0,
                    y + 23.0,
                    16.0,
                    text,
                );
                crate::game_art::draw_world(Rect::new(x + 12.0, y + 51.0, 96.0, 96.0), visual, 1.0);
                crate::game_art::draw_world(
                    Rect::new(x + 145.0, y + 68.0, 32.0, 32.0),
                    visual,
                    1.0,
                );
                if index >= 20 && index <= 25 {
                    crate::game_art::draw_world(
                        Rect::new(x + 145.0, y + 124.0, 32.0, 32.0),
                        Visual::ArmedActor {
                            body: index as u8,
                            weapon: representative(8).unwrap(),
                        },
                        1.0,
                    );
                }
            }
        }
    }
}
