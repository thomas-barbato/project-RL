//! Complete isolated review catalogue. Never creates or resumes a campaign.
use crate::game_art::{self, Visual, walking};
use macroquad::prelude::*;
use project_rl::entity::EquipmentPalette;

fn button(rect: Rect, label: &str, selected: bool) -> bool {
    let mouse = Vec2::from(mouse_position());
    let hover = rect.contains(mouse);
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if selected {
            Color::from_rgba(33, 92, 113, 255)
        } else if hover {
            Color::from_rgba(51, 65, 73, 255)
        } else {
            Color::from_rgba(33, 43, 50, 255)
        },
    );
    draw_text(label, rect.x + 10., rect.y + rect.h * 0.68, 18., WHITE);
    hover && is_mouse_button_pressed(MouseButton::Left)
}
fn choose(y: f32, label: &str, value: &str, index: &mut usize, count: usize) -> bool {
    draw_text(label, 16., y + 20., 17., LIGHTGRAY);
    let old = *index;
    if button(Rect::new(16., y + 28., 34., 30.), "<", false) {
        *index = (*index + count - 1) % count;
    }
    draw_text(value, 58., y + 49., 17., WHITE);
    if button(Rect::new(354., y + 28., 34., 30.), ">", false) {
        *index = (*index + 1) % count;
    }
    if Rect::new(16., y + 28., 372., 30.).contains(Vec2::from(mouse_position()))
        && mouse_wheel().1 != 0.
    {
        *index = if mouse_wheel().1 > 0. {
            (*index + count - 1) % count
        } else {
            (*index + 1) % count
        };
    }
    old != *index
}
fn variants() -> Vec<(u16, u8)> {
    project_rl::monster_art::MONSTER_ART
        .iter()
        .map(|a| (a.number, a.variant))
        .chain(project_rl::monster_palettes::all_palettes().map(|p| (p.number, p.variant)))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn arrival_pose(_visual: Visual, facing: i8, frame: u8) -> Option<walking::Pose> {
    if frame == 8 {
        None
    } else {
        Some(walking::Pose { facing, frame })
    }
}

fn draw_comparison(visual: Visual, frame: u8, progress: f32, light: bool) {
    let background = if light {
        Color::from_rgba(174, 172, 158, 255)
    } else {
        Color::from_rgba(30, 43, 46, 255)
    };
    for (row, facing) in [-1, 1].into_iter().enumerate() {
        let y = 205. + row as f32 * 175.;
        draw_text(
            if facing < 0 {
                "Gauche : memes phases pour les cinq"
            } else {
                "Droite : memes phases pour les cinq"
            },
            432.,
            y - 48.,
            18.,
            WHITE,
        );
        for (class, name) in CLASSES.iter().enumerate() {
            let x = 432. + class as f32 * 160.;
            draw_rectangle(x, y - 30., 154., 138., background);
            draw_text(name, x + 5., y - 10., 16., WHITE);
            for tile in 0..2 {
                draw_rectangle_lines(x + 12. + tile as f32 * 64., y, 64., 64., 1., DARKGRAY);
            }
            let mut person = visual;
            if let Visual::Player { body, .. } = &mut person {
                *body = 3 + class as u8;
            }
            let offset = if facing < 0 { 1. - progress } else { progress };
            walking::with_pose(arrival_pose(person, facing, frame), || {
                game_art::draw_world(Rect::new(x + 12. + offset * 64., y, 64., 64.), person, 1.)
            });
        }
    }
    draw_text(
        "Detail x2 - droite, toujours la meme phase",
        432.,
        568.,
        19.,
        WHITE,
    );
    for (class, name) in CLASSES.iter().enumerate() {
        let x = 432. + class as f32 * 160.;
        draw_text(name, x + 5., 596., 16., WHITE);
        let mut person = visual;
        if let Visual::Player { body, .. } = &mut person {
            *body = 3 + class as u8;
        }
        walking::with_pose(arrival_pose(person, 1, frame), || {
            game_art::draw_world(Rect::new(x + 12., 610., 128., 128.), person, 1.)
        });
    }
    draw_text(
        "Pour valider un personnage, quitter la vue cote a cote.",
        432.,
        790.,
        17.,
        LIGHTGRAY,
    );
}
const PALETTES: [EquipmentPalette; 7] = EquipmentPalette::ALL;
const CLASSES: [&str; 5] = [
    "Bastion",
    "Technomancien",
    "Traqueuse",
    "Arcaniste",
    "Metamorphe",
];
const EXTRAS: [(&str, Visual); 10] = [
    ("Predateur", Visual::AgentForm(0)),
    ("Colosse", Visual::AgentForm(1)),
    ("Cuirasse", Visual::AgentForm(2)),
    ("Possede", Visual::AgentForm(3)),
    ("Abysse", Visual::AgentForm(4)),
    ("Forme fermee", Visual::AgentForm(5)),
    ("Drone eclaireur", Visual::DroneScout),
    ("Drone traqueur", Visual::DroneTracker),
    ("Drone gardien", Visual::DroneGuardian),
    ("Drone canonnier", Visual::DroneGunner),
];

pub(crate) async fn run(capture: Option<&std::path::Path>) -> Result<(), String> {
    request_new_screen_size(1280., 900.);
    for _ in 0..3 {
        next_frame().await;
    }
    let labels: Vec<String> = serde_json::from_str(include_str!(
        "../assets/animations/walk-08/review/weapons.json"
    ))
    .map_err(|e| e.to_string())?;
    let monsters = variants();
    let left_models: Vec<u16> = (0..113)
        .filter(|n| !game_art::weapon_uses_two_hands(*n))
        .collect();
    let mut group = 0;
    let mut common = true;
    let mut proposed = true;
    let mut comparison = false;
    let mut class = 0;
    let mut outfit = 0;
    let mut weapon = 68;
    let mut left = 0;
    let mut head = 0;
    let mut color = 0;
    let mut left_color = 0;
    let mut monster = 0;
    let mut extra = 0;
    let mut pace = 2;
    let mut playing = true;
    let mut manual: Option<u8> = None;
    let mut elapsed = 0.;
    let mut previous = get_time();
    let mut floor = false;
    let mut capture_index = 0usize;
    let scenarios = [(0usize, 68usize), (1, 102), (2, 39), (3, 71), (4, 71)];
    let colored_sample = monsters
        .iter()
        .copied()
        .find(|(_, variant)| *variant > 0)
        .ok_or("aucune variante coloree dans le catalogue")?;
    let capture_monsters = [
        (14, 0),
        (15, 0),
        (22, 0),
        (24, 0),
        (115, 0),
        (124, 0),
        (22, 1),
        (24, 1),
        colored_sample,
    ];
    let mut report = Vec::new();
    let decisions_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("docs/reviews/player-walk-12/user-decisions.json");
    let mut decisions: std::collections::BTreeMap<String, serde_json::Value> =
        std::fs::read_to_string(&decisions_path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();
    let mut decision_message = String::new();
    loop {
        let now = get_time();
        if playing {
            elapsed += (now - previous).max(0.).min(0.1);
        }
        previous = now;
        if is_quit_requested() || is_key_pressed(KeyCode::Escape) {
            break;
        }
        if capture.is_some() {
            comparison = (25..30).contains(&capture_index);
            if capture_index < 30 {
                group = 0;
                class = if comparison { 0 } else { capture_index / 5 };
                outfit = if comparison {
                    capture_index - 25
                } else {
                    capture_index % 5
                };
                weapon = if comparison { 68 } else { scenarios[class].1 };
                head = 0;
                color = 0;
            } else if capture_index < 30 + capture_monsters.len() {
                group = 1;
                monster = monsters
                    .iter()
                    .position(|p| *p == capture_monsters[capture_index - 30])
                    .ok_or_else(|| {
                        format!(
                            "Modele de capture absent: {:?}",
                            capture_monsters[capture_index - 30]
                        )
                    })?;
            } else if capture_index < 30 + capture_monsters.len() + EXTRAS.len() {
                group = 2;
                extra = capture_index - 30 - capture_monsters.len();
            } else {
                group = 0;
                class = capture_index - 30 - capture_monsters.len() - EXTRAS.len();
                outfit = 4;
                weapon = 102;
                left = left_models
                    .iter()
                    .position(|n| *n == 68)
                    .ok_or("arme de capture absente")?
                    + 1;
                head = 0;
                color = 0;
                left_color = 0;
            }
        }
        clear_background(Color::from_rgba(19, 28, 34, 255));
        draw_text(
            "Animations - catalogue et comparaisons",
            16.,
            30.,
            25.,
            WHITE,
        );
        draw_text(
            "Revue isolee : aucun changement de partie ni de preferences",
            16.,
            54.,
            17.,
            LIGHTGRAY,
        );
        if button(Rect::new(16., 72., 112., 34.), "Joueurs", group == 0) {
            group = 0;
        }
        if button(Rect::new(136., 72., 112., 34.), "Monstres", group == 1) {
            group = 1;
        }
        if button(Rect::new(256., 72., 132., 34.), "Formes/drones", group == 2) {
            group = 2;
        }
        let mut changed = false;
        let version_label = if group == 2 {
            "Propositions"
        } else {
            "Version integree"
        };
        if button(
            Rect::new(432., 72., 160., 34.),
            version_label,
            common && (group != 0 || proposed),
        ) {
            common = true;
            proposed = true;
            changed = true;
        }
        if button(Rect::new(600., 72., 160., 34.), "Ancienne version", !common) {
            common = false;
            proposed = false;
            changed = true;
        }
        if group == 0 {
            if button(
                Rect::new(944., 72., 286., 34.),
                "Version precedente (WALK11)",
                common && !proposed,
            ) {
                proposed = false;
                common = true;
                changed = true;
            }
            if button(Rect::new(768., 72., 168., 34.), "Cote a cote", comparison) {
                comparison = !comparison;
                changed = true;
            }
            if comparison {
                draw_text("Les cinq personnages synchronises", 16., 153., 17., WHITE);
            } else {
                let value = CLASSES[class];
                changed |= choose(118., "Personnage", value, &mut class, 5);
            }
            let value = format!("Tenue {} / 5", outfit + 1);
            changed |= choose(183., "Armure", &value, &mut outfit, 5);
            let value = if weapon == 0 {
                "Sans arme".to_owned()
            } else {
                format!(
                    "{} - {}",
                    weapon - 1,
                    labels[weapon - 1].chars().take(25).collect::<String>()
                )
            };
            changed |= choose(248., "Main principale", &value, &mut weapon, 114);
            if weapon > 0 && game_art::weapon_uses_two_hands((weapon - 1) as u16) {
                left = 0;
                draw_text(
                    "Deuxieme main : arme a deux mains",
                    16.,
                    350.,
                    17.,
                    LIGHTGRAY,
                );
            } else {
                let value = if left == 0 {
                    "Libre".to_owned()
                } else {
                    format!("Modele {}", left_models[left - 1])
                };
                changed |= choose(
                    313.,
                    "Deuxieme main",
                    &value,
                    &mut left,
                    left_models.len() + 1,
                );
            }
            let value = format!("Casque {} / 6 (0 = tenue)", head);
            changed |= choose(378., "Casque", &value, &mut head, 7);
            let value = format!("{:?}", PALETTES[color]);
            changed |= choose(443., "Couleur des armes", &value, &mut color, 7);
            let value = format!("{:?}", PALETTES[left_color]);
            changed |= choose(508., "Couleur deuxieme main", &value, &mut left_color, 7);
        } else if group == 1 {
            let (n, v) = monsters[monster];
            let name = project_rl::monster_art::art(n, v).unwrap().name;
            let value = format!("MON-{n:03}-{v} - {name}");
            changed |= choose(
                118.,
                "Modele et couleur",
                &value,
                &mut monster,
                monsters.len(),
            );
            draw_text(
                &format!("{} / {}", monster + 1, monsters.len()),
                16.,
                207.,
                19.,
                WHITE,
            );
            if is_key_pressed(KeyCode::Right) {
                monster = (monster + 1) % monsters.len();
                changed = true;
            }
            if is_key_pressed(KeyCode::Left) {
                monster = (monster + monsters.len() - 1) % monsters.len();
                changed = true;
            }
            draw_text("Fleches : precedent / suivant", 16., 239., 16., LIGHTGRAY);
        } else {
            changed |= choose(
                118.,
                "Forme ou drone",
                EXTRAS[extra].0,
                &mut extra,
                EXTRAS.len(),
            );
        }
        let value = format!("{} ms par case", [120, 200, 400][pace]);
        changed |= choose(580., "Duree", &value, &mut pace, 3);
        if button(
            Rect::new(16., 659., 110., 32.),
            if playing { "Pause" } else { "Lire" },
            false,
        ) || is_key_pressed(KeyCode::Space)
        {
            playing = !playing;
            manual = None;
        }
        if button(Rect::new(136., 659., 120., 32.), "Image +", false) {
            manual = Some((manual.unwrap_or(0) + 1) % 7);
            playing = false;
        }
        if button(Rect::new(266., 659., 122., 32.), "Sol clair", floor) {
            floor = !floor;
        }
        if changed {
            manual = None;
            elapsed = 0.;
        }
        let _review = common.then(walking::review_corrections);
        let _common = walking::review_common(true);
        let _proportions = walking::review_proportions(common);
        let _gait = walking::review_gait12(proposed && group == 0);
        let visual = if group == 0 {
            let torsos = [None, Some(110), Some(102), Some(101), Some(103)];
            let mut equip = [torsos[outfit], None, None, None];
            if head > 0 {
                equip[1] = Some(114 + head as u16);
            }
            Visual::Player {
                body: 3 + class as u8,
                equipment: equip,
                weapon: weapon.checked_sub(1).map(|w| w as u16),
                hands: if left == 0 {
                    None
                } else {
                    Some([
                        Some(left_models[left - 1]),
                        weapon.checked_sub(1).map(|n| n as u16),
                    ])
                },
                palettes: [
                    EquipmentPalette::Original,
                    PALETTES[color],
                    PALETTES[left_color],
                ],
            }
        } else if group == 1 {
            let (number, variant) = monsters[monster];
            Visual::CampaignMonster { number, variant }
        } else {
            EXTRAS[extra].1
        };
        let mut case_id = if group == 0 {
            format!(
                "P{}-T{}-W{}-L{}-H{}-C{}:{}",
                class + 1,
                outfit + 1,
                weapon,
                left,
                head,
                color,
                left_color
            )
        } else if group == 1 {
            format!("MON-{:03}-{}", monsters[monster].0, monsters[monster].1)
        } else {
            format!("EXTRA-{extra:02}")
        };
        let revision = if group == 0 && weapon == 68 {
            // Preserve earlier choices as history; this changed drawing needs
            // its own decision without invalidating unrelated configurations.
            case_id.push_str("-AX67-r02");
            "WALK08-AX67-r02"
        } else {
            "WALK08-r01"
        };
        case_id.push_str("-STEP1");
        let revision = if proposed && group == 0 {
            case_id.push_str("-WALK12-COMMON-r01-FRONT");
            "WALK12-COMMON-r01"
        } else if common {
            if group == 0 {
                case_id.push_str("-PROPORTIONS11-GRIP-r04-FRONT");
                "WALK11-GRIP-r04-FRONT"
            } else {
                case_id.push_str("-PROPORTIONS11-r01");
                "WALK11-PROPORTIONS-r01"
            }
        } else if group == 0 {
            case_id.push_str("-COMMON09-r01");
            "WALK09-COMMON-r01"
        } else if group == 0 && class == 1 && outfit == 0 {
            case_id.push_str("-TECH01-r03");
            "WALK08-STEP1-TECH01-r03"
        } else if group == 0 && class == 1 && outfit == 4 {
            case_id.push_str("-TECH05-r02");
            "WALK08-STEP1-TECH05-r02"
        } else if group == 2 && extra == 0 {
            case_id.push_str("-PREDATOR-r02");
            "WALK08-STEP1-PREDATOR-r02"
        } else if revision == "WALK08-AX67-r02" {
            "WALK08-STEP1-AX67-r02"
        } else {
            "WALK08-STEP1-r02"
        };
        let (accept, reject) = if group == 0 && comparison {
            draw_text(
                "Validation dans la vue individuelle",
                16.,
                746.,
                16.,
                LIGHTGRAY,
            );
            (false, false)
        } else {
            (
                button(Rect::new(16., 724., 178., 34.), "Valider ce cas", false),
                button(Rect::new(204., 724., 184., 34.), "A revoir", false),
            )
        };
        if capture.is_none() && (accept || reject) {
            decisions.insert(case_id.clone(),serde_json::json!({"revision":revision,"status":if accept{"accepted"}else{"rejected"},"visual":format!("{visual:?}"),"scope":"both directions, six walking phases; player frontal at arrival"}));
            match std::fs::write(
                &decisions_path,
                serde_json::to_vec_pretty(&decisions).unwrap(),
            ) {
                Ok(()) => decision_message = "Decision enregistree pour ce cas".into(),
                Err(e) => decision_message = format!("Enregistrement impossible: {e}"),
            }
        }
        draw_text(&case_id, 16., 795., 16., WHITE);
        let decision = decisions
            .get(&case_id)
            .and_then(|v| v["status"].as_str())
            .unwrap_or(if group == 0 && proposed && common {
                "valide (lot WALK12)"
            } else {
                "a examiner"
            });
        draw_text(decision, 16., 819., 17., LIGHTGRAY);
        draw_text(&decision_message, 16., 843., 15., LIGHTGRAY);
        let duration = [0.12, 0.2, 0.4][pace];
        let trial = (elapsed / (duration + 0.52)).floor() as u64;
        let t = elapsed % (duration + 0.52);
        let progress = (t / duration).min(1.) as f32;
        let column = manual.unwrap_or_else(|| {
            if t >= duration {
                0
            } else {
                1 + (trial % 2) as u8 * 3 + (t / duration * 3.).floor() as u8
            }
        });
        draw_text("Un deplacement = une case de 64 px", 432., 129., 20., WHITE);
        if group == 0 && comparison {
            draw_comparison(
                visual,
                if column == 0 { 8 } else { column - 1 },
                progress,
                floor,
            );
        } else {
            for (row, facing) in [-1, 1].into_iter().enumerate() {
                let y = 190. + row as f32 * 175.;
                let bg = if floor {
                    Color::from_rgba(174, 172, 158, 255)
                } else {
                    Color::from_rgba(30, 43, 46, 255)
                };
                draw_rectangle(430., y - 30., 804., 145., bg);
                draw_text(
                    if facing < 0 { "< Gauche" } else { "Droite >" },
                    442.,
                    y - 42.,
                    18.,
                    WHITE,
                );
                for cell in 0..2 {
                    draw_rectangle_lines(
                        450. + cell as f32 * 64.,
                        y,
                        64.,
                        64.,
                        1.,
                        Color::new(0.5, 0.6, 0.6, 0.3),
                    );
                }
                let x = if facing < 0 {
                    514. - progress * 64.
                } else {
                    450. + progress * 64.
                };
                let pose = walking::Pose {
                    facing,
                    frame: if column == 0 { 8 } else { column - 1 },
                };
                walking::with_pose(arrival_pose(visual, pose.facing, pose.frame), || {
                    game_art::draw_world(Rect::new(x, y, 64., 64.), visual, 1.)
                });
                draw_text("Face de reference", 680., y - 8., 16., LIGHTGRAY);
                walking::with_pose(None, || {
                    game_art::draw_world(Rect::new(720., y, 64., 64.), visual, 1.)
                });
                let detail = if let Visual::CampaignMonster { number, variant } = visual {
                    if project_rl::monster_art::art(number, variant).is_some_and(|a| a.side > 64) {
                        64.
                    } else {
                        128.
                    }
                } else {
                    128.
                };
                walking::with_pose(arrival_pose(visual, pose.facing, pose.frame), || {
                    game_art::draw_world(Rect::new(960., y - 24., detail, detail), visual, 1.)
                });
            }
            draw_text(
                "2 sens : repos, pas A (3 phases), pas B (3 phases)",
                432.,
                535.,
                19.,
                WHITE,
            );
            for (row, facing) in [-1, 1].into_iter().enumerate() {
                for col in 0..7 {
                    let cell =
                        Rect::new(454. + col as f32 * 110., 572. + row as f32 * 138., 64., 64.);
                    draw_rectangle_lines(cell.x, cell.y, cell.w, cell.h, 1., DARKGRAY);
                    let frame = if col == 0 { 8 } else { col as u8 - 1 };
                    walking::with_pose(arrival_pose(visual, facing, frame), || {
                        game_art::draw_world(cell, visual, 1.)
                    });
                    draw_text(
                        if col == 0 { "Repos" } else { "Marche" },
                        cell.x,
                        cell.y + 87.,
                        15.,
                        LIGHTGRAY,
                    );
                }
            }
        }
        draw_text(
            "Espace : pause - Echap : fermer la revue",
            16.,
            870.,
            17.,
            LIGHTGRAY,
        );
        if let Some(output) = capture {
            if t > 0.19 {
                std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
                let filename = format!("catalogue-{capture_index:03}.png");
                crate::ui_capture::framebuffer()?
                    .export_png(output.join(&filename).to_str().ok_or("capture path")?);
                report.push(serde_json::json!({"case":capture_index,"visual":format!("{visual:?}"),"file":filename,"compared_characters":if comparison&&group==0{5}else{1},"common_gait":common&&group==0,"proposed_walk12":proposed&&group==0,"player_front_at_arrival":true}));
                capture_index += 1;
                elapsed = 0.;
                if capture_index == 30 + capture_monsters.len() + EXTRAS.len() + CLASSES.len() {
                    break;
                }
            }
        }
        next_frame().await;
    }
    if let Some(output) = capture {
        std::fs::write(
            output.join("checks.json"),
            serde_json::to_vec_pretty(&report).unwrap(),
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
