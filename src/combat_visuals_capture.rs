//! Review-only native screens. These do not replace the normal inventory or HUD.
use super::*;

const PHOSPHOR: Color = Color::new(0.49, 0.88, 0.70, 1.0);
const DIM: Color = Color::new(0.30, 0.51, 0.43, 1.0);
const INK: Color = Color::new(0.025, 0.050, 0.042, 1.0);

fn panel(rect: Rect, label: &str) {
    draw_rectangle(rect.x, rect.y, rect.w, rect.h, INK);
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, DIM);
    draw_text(label, rect.x + 12.0, rect.y + 25.0, 19.0, PHOSPHOR);
    draw_line(rect.x, rect.y + 36.0, rect.right(), rect.y + 36.0, 1.0, DIM);
}

impl AsciiApp {
    fn draw_equipment_palettes_review(&self) {
        use crate::game_art::Visual;
        use project_rl::entity::EquipmentPalette;
        draw_text_bold(
            "ÉQUIPEMENTS / PALETTES INDÉPENDANTES",
            24.0,
            35.0,
            25.0,
            PHOSPHOR,
        );
        draw_text(
            "Même objet, mêmes statistiques · corps de 30 px · aperçu agrandi et taille de jeu",
            24.0,
            62.0,
            18.0,
            DIM,
        );
        let colors = &EquipmentPalette::ALL[1..];
        for (col, color) in colors.iter().enumerate() {
            draw_text(
                color.name(),
                153.0 + col as f32 * 183.0,
                91.0,
                18.0,
                PHOSPHOR,
            );
        }
        for (row, (torso, id, label)) in [
            (101, "core:armure_cerbere", "Cerbère"),
            (109, "core:veste_de_la_seconde_ombre", "Seconde\nOmbre"),
            (113, "core:veste_matelassee", "Technicien"),
        ]
        .into_iter()
        .enumerate()
        {
            let y = 108.0 + row as f32 * 219.0;
            for (line, text) in label.lines().enumerate() {
                draw_text(text, 18.0, y + 96.0 + line as f32 * 19.0, 17.0, PHOSPHOR);
            }
            for (col, &color) in colors.iter().enumerate() {
                let x = 140.0 + col as f32 * 183.0;
                panel(Rect::new(x, y, 174.0, 205.0), "");
                let weapon_color = colors[(col + 3) % 6];
                let visual = Visual::Player {
                    body: 0,
                    equipment: [Some(torso), None, None, None],
                    weapon: crate::game_art::held_weapon("core:fusil_d_assaut"),
                    hands: if row == 2 {
                        Some([
                            crate::game_art::held_weapon("core:modele_cogneur_p01"),
                            crate::game_art::held_weapon("core:integrity_blade"),
                        ])
                    } else {
                        None
                    },
                    palettes: [color, weapon_color, colors[(col + 1) % 6]],
                };
                draw_text(
                    if row == 2 {
                        "Épée + pistolet"
                    } else {
                        "Fusil :"
                    },
                    x + 8.0,
                    y + 26.0,
                    15.0,
                    DIM,
                );
                if row != 2 {
                    draw_text(weapon_color.name(), x + 60.0, y + 26.0, 15.0, PHOSPHOR);
                }
                crate::game_art::draw_portrait(Rect::new(x + 8.0, y + 36.0, 110.0, 110.0), visual);
                crate::game_art::draw_world(
                    Rect::new(x + 130.0, y + 111.0, 32.0, 32.0),
                    visual,
                    1.0,
                );
                draw_text("32 px", x + 128.0, y + 169.0, 12.0, DIM);
                crate::game_art::draw_item_variant(
                    Rect::new(x + 12.0, y + 155.0, 40.0, 40.0),
                    "core:fusil_d_assaut",
                    weapon_color,
                );
                crate::game_art::draw_item_variant(
                    Rect::new(x + 66.0, y + 155.0, 40.0, 40.0),
                    id,
                    color,
                );
            }
        }
        draw_text(
            "Les prises de pistolet sont illustrées en référence ; les pistolets ne sont pas encore ajoutés au butin.",
            24.0,
            789.0,
            14.0,
            DIM,
        );
    }

    pub(super) fn draw_combat_proposal(&self, scene: &str, phase: f64) {
        if scene == "terminal-console-proposal" {
            self.draw_terminal_console_proposal();
            return;
        }
        if scene.starts_with("crawl-idle") {
            self.draw_idle_integration_review(phase, scene.ends_with("reduced"));
            return;
        }
        if scene.starts_with("crawl-") {
            self.draw_crawl_review(scene);
            return;
        }
        if scene == "terminal-hud-proposal" {
            self.draw();
            for rect in [
                Rect::new(28.0, 15.0, 260.0, 673.0),
                Rect::new(985.0, 15.0, 260.0, 673.0),
            ] {
                draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 1.0, PHOSPHOR);
            }
            // Light scanlines preserve the gauge values and existing contrast.
            for y in (0..800).step_by(4) {
                draw_line(
                    0.0,
                    y as f32,
                    1280.0,
                    y as f32,
                    1.0,
                    Color::new(0.02, 0.05, 0.035, 0.06),
                );
            }
            draw_rectangle(288.0, 16.0, 696.0, 28.0, INK);
            draw_text(
                "[ PROPOSITION ]  TERMINAL PERSONNEL / AFFICHAGE TACTIQUE",
                322.0,
                36.0,
                17.0,
                PHOSPHOR,
            );
            return;
        }
        clear_background(Color::from_rgba(4, 10, 9, 255));
        if scene == "equipment-palettes-review" {
            self.draw_equipment_palettes_review();
            return;
        }
        if scene == "terminal-inventory-proposal" {
            self.draw_terminal_inventory_proposal();
            return;
        }
        if matches!(scene, "player-cuffs-review" | "player-outfits-review") {
            draw_text_bold(
                "TENUES COMPLÈTES / 14 MODÈLES DE TORSE",
                34.0,
                40.0,
                25.0,
                PHOSPHOR,
            );
            draw_text(
                "Corps de 30 px · 1 épée courte / 2 géante / 3 fusil / 4 pistolet · trois poses ambidextres",
                34.0,
                68.0,
                18.0,
                DIM,
            );
            let weapon = match self.front_player_visual() {
                crate::game_art::Visual::Player { weapon, .. } => weapon,
                _ => None,
            };
            for index in 0..14 {
                let x = 32.0 + (index % 7) as f32 * 175.0;
                let y = 95.0 + (index / 7) as f32 * 340.0;
                let visual = crate::game_art::Visual::Player {
                    body: (index % 3) as u8,
                    palettes: Default::default(),
                    weapon,
                    equipment: [Some(101 + index), None, Some(121 + index % 6), None],
                    hands: None,
                };
                panel(Rect::new(x, y, 166.0, 318.0), "");
                let name = self
                    .game
                    .rules()
                    .items
                    .iter()
                    .find(|(id, _)| {
                        crate::game_art::item(id.as_str())
                            .is_some_and(|art| art.illustration == 101 + index)
                    })
                    .map(|(id, _)| self.item_name(id))
                    .unwrap_or_else(|| format!("CORPS {}", 101 + index));
                crate::ui_theme::draw_text_in_rect(
                    name,
                    Rect::new(x + 7.0, y + 4.0, 152.0, 28.0),
                    13,
                    PHOSPHOR,
                );
                let melee = crate::game_art::Visual::Player {
                    body: (index % 3) as u8,
                    palettes: Default::default(),
                    weapon: crate::game_art::held_weapon("core:melee_epee_longue"),
                    equipment: [Some(101 + index), None, None, None],
                    hands: None,
                };
                let make = |weapon, hands| crate::game_art::Visual::Player {
                    body: (index % 3) as u8,
                    palettes: Default::default(),
                    weapon,
                    hands,
                    equipment: [Some(101 + index), None, None, None],
                };
                let short_weapon = crate::game_art::held_weapon("core:integrity_blade");
                let short = make(short_weapon, None);
                let mixed = make(
                    short_weapon,
                    Some([
                        crate::game_art::held_weapon("core:modele_cogneur_p01"),
                        crate::game_art::held_weapon("core:integrity_blade"),
                    ]),
                );
                crate::game_art::draw_portrait(Rect::new(x + 3.0, y + 44.0, 80.0, 80.0), melee);
                crate::game_art::draw_portrait(Rect::new(x + 82.0, y + 44.0, 80.0, 80.0), mixed);
                draw_text("Grande épée / paire", x + 14.0, y + 139.0, 14.0, DIM);
                for (column, v) in [short, melee, visual].into_iter().enumerate() {
                    crate::game_art::draw_world(
                        Rect::new(x + 8.0 + column as f32 * 39.0, y + 178.0, 32.0, 32.0),
                        v,
                        1.0,
                    );
                    draw_text(
                        (column + 1).to_string(),
                        x + 18.0 + column as f32 * 39.0,
                        y + 226.0,
                        13.0,
                        DIM,
                    );
                }
                crate::game_art::draw_pistol_reference(
                    Rect::new(x + 125.0, y + 178.0, 32.0, 32.0),
                    short,
                    32,
                );
                draw_text("4", x + 135.0, y + 226.0, 13.0, DIM);
                for (column, hands) in [
                    [
                        crate::game_art::held_weapon("core:integrity_blade"),
                        crate::game_art::held_weapon("core:integrity_blade"),
                    ],
                    [
                        crate::game_art::held_weapon("core:modele_cogneur_p01"),
                        crate::game_art::held_weapon("core:modele_cogneur_p01"),
                    ],
                    [
                        crate::game_art::held_weapon("core:modele_cogneur_p01"),
                        crate::game_art::held_weapon("core:integrity_blade"),
                    ],
                ]
                .into_iter()
                .enumerate()
                {
                    crate::game_art::draw_world(
                        Rect::new(x + 15.0 + column as f32 * 51.0, y + 265.0, 32.0, 32.0),
                        make(short_weapon, Some(hands)),
                        1.0,
                    );
                }
            }
            return;
        }
        draw_text_bold("PERSONNAGE ET COMBATS", 34.0, 40.0, 27.0, PHOSPHOR);
        draw_text(
            "Rendu du client · équipement porté, fusil à deux mains, pistolet levé et réactions aux coups",
            34.0,
            69.0,
            18.0,
            DIM,
        );
        let mut naked =
            crate::game_art::player(Some("core:breche"), PlayerWeaponPose::Unarmed, None);
        let armed = self.front_player_visual();
        panel(
            Rect::new(32.0, 96.0, 388.0, 335.0),
            "CASQUE / PERSONNAGE DE BASE",
        );
        crate::game_art::draw_portrait(Rect::new(72.0, 143.0, 224.0, 224.0), naked);
        crate::game_art::draw_world(Rect::new(329.0, 318.0, 32.0, 32.0), naked, 1.0);
        draw_text("Portrait et taille réelle : 32 px", 50.0, 402.0, 17.0, DIM);
        panel(
            Rect::new(438.0, 96.0, 388.0, 335.0),
            "ARMURES / FUSIL À DEUX MAINS",
        );
        crate::game_art::draw_portrait(Rect::new(478.0, 143.0, 224.0, 224.0), armed);
        crate::game_art::draw_world(Rect::new(738.0, 318.0, 32.0, 32.0), armed, 1.0);
        draw_text(
            "Tenue complète déterminée par le torse",
            456.0,
            402.0,
            17.0,
            DIM,
        );
        panel(
            Rect::new(844.0, 96.0, 404.0, 335.0),
            "PISTOLET / BRAS PLIÉ ET LEVÉ",
        );
        crate::game_art::draw_pistol_reference(Rect::new(884.0, 143.0, 224.0, 224.0), armed, 128);
        crate::game_art::draw_pistol_reference(Rect::new(1160.0, 318.0, 32.0, 32.0), naked, 32);
        draw_text(
            "Arme de référence · aucun ajout au butin",
            862.0,
            402.0,
            16.0,
            DIM,
        );
        panel(
            Rect::new(32.0, 450.0, 1216.0, 308.0),
            "MÊLÉE · RECUL AUX DÉGÂTS · ÉCLABOUSSURES · CORPS AU SOL",
        );
        let mut motion = crate::combat_visuals::CombatMotion::default();
        let player = self.game.player_id();
        motion.play(player, GridPos::new(0, 0), GridPos::new(1, 0), false, 0.0);
        naked = crate::game_art::player(
            Some("core:breche"),
            PlayerWeaponPose::Melee,
            Some("core:melee_epee_longue"),
        );
        for (column, label) in [
            "Attaque vers la cible",
            "Recul après l'impact",
            "Traces et corps persistants",
        ]
        .into_iter()
        .enumerate()
        {
            let x = 80.0 + column as f32 * 390.0;
            draw_text(label, x, 532.0, 18.0, PHOSPHOR);
            for row in 0..3 {
                for col in 0..5 {
                    draw_rectangle(
                        x + col as f32 * 56.0,
                        551.0 + row as f32 * 56.0,
                        55.0,
                        55.0,
                        Color::from_rgba(30, 37, 35, 255),
                    );
                }
            }
            let rect = Rect::new(x + 56.0, 607.0, 56.0, 56.0);
            if column == 2 {
                let cell = crate::combat_visuals::ResidueCell {
                    blood: 6,
                    wall: false,
                    seed: 137,
                    corpse: Some(20),
                    corpse_symbol: None,
                };
                crate::combat_visuals::draw_residue(rect, &cell, true);
                let wall = crate::combat_visuals::ResidueCell {
                    blood: 3,
                    wall: true,
                    seed: 65,
                    corpse: None,
                    corpse_symbol: None,
                };
                draw_rectangle(
                    x + 56.0,
                    551.0,
                    55.0,
                    55.0,
                    Color::from_rgba(71, 73, 68, 255),
                );
                crate::combat_visuals::draw_residue(
                    Rect::new(x + 56.0, 551.0, 56.0, 56.0),
                    &wall,
                    true,
                );
                crate::game_art::draw_world(Rect::new(x + 168.0, 607.0, 56.0, 56.0), armed, 1.0);
            } else {
                if column == 1 {
                    motion.play(player, GridPos::new(0, 0), GridPos::new(1, 0), true, 0.0);
                }
                let offset = motion.offset(player, phase, false) * 56.0;
                crate::game_art::draw_world(
                    Rect::new(rect.x + offset.x, rect.y, rect.w, rect.h),
                    naked,
                    1.0,
                );
                crate::game_art::draw_world(
                    Rect::new(x + 112.0, 607.0, 56.0, 56.0),
                    crate::game_art::Visual::Actor(20),
                    1.0,
                );
                if column == 1 {
                    crate::combat_visuals::draw_residue(
                        rect,
                        &crate::combat_visuals::ResidueCell {
                            blood: 2,
                            seed: 23,
                            ..Default::default()
                        },
                        true,
                    );
                }
            }
        }
    }

    fn draw_terminal_inventory_proposal(&self) {
        draw_text_bold("[ INVENTAIRE / ÉQUIPEMENT ]", 36.0, 42.0, 25.0, PHOSPHOR);
        draw_text(
            "PROPOSITION VISUELLE · détails au survol des objets et des équipements",
            36.0,
            71.0,
            17.0,
            DIM,
        );
        panel(
            Rect::new(32.0, 98.0, 326.0, 619.0),
            "SAC  /  ARMES  /  ARMURES",
        );
        for (row, entry) in self.inventory_entries().iter().take(8).enumerate() {
            let y = 159.0 + row as f32 * 65.0;
            crate::game_art::draw_item(
                Rect::new(46.0, y - 22.0, 40.0, 40.0),
                entry.item().as_str(),
            );
            crate::ui_theme::draw_text_in_rect(
                &self.inventory_entry_name(entry),
                Rect::new(95.0, y - 20.0, 248.0, 28.0),
                18,
                PHOSPHOR,
            );
            if let Some(role) = self.inventory_equipped_label(entry.instance()) {
                draw_text(&role, 96.0, y + 24.0, 14.0, DIM);
            }
        }
        panel(
            Rect::new(376.0, 98.0, 550.0, 619.0),
            "PERSONNAGE / PROTOCOLE BRÈCHE",
        );
        let silhouette = Rect::new(500.0, 186.0, 280.0, 380.0);
        crate::game_art::draw_terminal_wireframe(silhouette, self.front_player_visual(), PHOSPHOR);
        for (x, y, id, label, body_point) in [
            (393.0, 162.0, "core:heaume_bastion", "TÊTE", vec2(16.0, 2.7)),
            (
                792.0,
                283.0,
                "core:plastron_bastion",
                "CORPS",
                vec2(20.0, 12.0),
            ),
            (
                393.0,
                404.0,
                "core:gantelets_bastion",
                "MAINS",
                vec2(8.0, 18.8),
            ),
            (
                792.0,
                525.0,
                "core:solerets_bastion",
                "PIEDS",
                vec2(19.2, 29.5),
            ),
        ] {
            draw_rectangle_lines(x, y, 108.0, 86.0, 1.0, DIM);
            crate::game_art::draw_item(Rect::new(x + 26.0, y + 7.0, 56.0, 56.0), id);
            draw_text(label, x + 18.0, y + 79.0, 15.0, PHOSPHOR);
            let start = vec2(if x < silhouette.x { x + 108.0 } else { x }, y + 40.0);
            let target = vec2(
                silhouette.x + body_point.x * silhouette.w / 32.0,
                silhouette.y + body_point.y * silhouette.h / 32.0,
            );
            let elbow_x = (start.x + target.x) * 0.5;
            draw_line(start.x, start.y, elbow_x, start.y, 1.0, DIM);
            draw_line(elbow_x, start.y, elbow_x, target.y, 1.0, DIM);
            draw_line(elbow_x, target.y, target.x, target.y, 1.0, DIM);
            draw_circle(target.x, target.y, 2.0, PHOSPHOR);
        }
        for (index, id) in [
            "core:modele_brenek_a80",
            "core:needle_launcher",
            "core:flamethrower",
        ]
        .into_iter()
        .enumerate()
        {
            let x = 393.0 + index as f32 * 169.0;
            draw_rectangle_lines(x, 625.0, 155.0, 74.0, 1.0, DIM);
            crate::game_art::draw_item(Rect::new(x + 52.0, 628.0, 50.0, 44.0), id);
            draw_text(&weapon_slot_label(index), x + 7.0, 690.0, 14.0, PHOSPHOR);
        }
        panel(Rect::new(944.0, 98.0, 304.0, 250.0), "ÉTAT DU PERSONNAGE");
        for (row, label, value, ratio) in [
            (0, "PV", "20 / 20", 1.0),
            (1, "ÉNERGIE", "100 / 100", 1.0),
            (2, "CHALEUR", "0 / 100", 0.0),
            (3, "EXPÉRIENCE", "NIVEAU 1", 0.12),
        ] {
            let y = 159.0 + row as f32 * 48.0;
            draw_text(label, 960.0, y, 15.0, DIM);
            draw_text(value, 1090.0, y, 16.0, PHOSPHOR);
            draw_rectangle(
                960.0,
                y + 7.0,
                267.0,
                6.0,
                Color::from_rgba(30, 49, 41, 255),
            );
            draw_rectangle(960.0, y + 7.0, 267.0 * ratio, 6.0, PHOSPHOR);
        }
        panel(
            Rect::new(944.0, 383.0, 304.0, 235.0),
            "SURVOL : PLASTRON BASTION",
        );
        crate::game_art::draw_item(
            Rect::new(1163.0, 463.0, 64.0, 64.0),
            "core:plastron_bastion",
        );
        draw_text("Équipé · Corps", 960.0, 450.0, 15.0, PHOSPHOR);
        let definition = self
            .game
            .rules()
            .items
            .get(&"core:plastron_bastion".parse().unwrap())
            .unwrap();
        draw_text(
            format!(
                "Armure                  {}",
                definition.equipment().unwrap().armor()
            ),
            960.0,
            487.0,
            17.0,
            PHOSPHOR,
        );
        draw_text(
            format!(
                "Masse              {:.1} kg",
                definition.mass_grams().unwrap_or(0) as f32 / 1000.0
            ),
            960.0,
            516.0,
            17.0,
            DIM,
        );
        draw_text("Attributs, effets et comparaison", 960.0, 559.0, 16.0, DIM);
        draw_text("dans le même tooltip.", 960.0, 588.0, 16.0, DIM);
        draw_text(
            "[1–3] Armes    [Entrée] Équiper    [X] Déposer    [Échap] Fermer",
            38.0,
            758.0,
            18.0,
            PHOSPHOR,
        );
    }
}
