//! Deterministic inspection comparisons using the actual game renderer.
use super::*;
use crate::game_art::Visual;
use project_rl::world::Map;

impl AsciiApp {
    pub async fn capture_campaign_monsters(
        output: &std::path::Path,
        extended_palettes: bool,
        blue_palettes: bool,
    ) -> Result<(), String> {
        use project_rl::monster_modifiers::{MonsterModifier, PHYSICAL_RESISTANCE_TAG};
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let mut app = Self::new_agent_drone_trial()?;
        app.suspension_path = output.join("isolated-run.json");
        let map=Map::from_ascii("####################\n#..................#\n#..................#\n#..................#\n#..................#\n#..................#\n#..................#\n#..................#\n#..................#\n#..................#\n#..................#\n#..................#\n####################").map_err(|e|e.to_string())?;
        let mut rules = app.rules.clone();
        rules.player_field_of_view.radius = 20;
        let mut game = GameState::new_with_rules(map, GridPos::new(9, 6), 123, rules)
            .map_err(|e| e.to_string())?;
        let models = if blue_palettes {
            [
                (101, 4),
                (138, 4),
                (146, 4),
                (147, 4),
                (149, 4),
                (152, 4),
                (141, 4),
                (142, 4),
                (136, 4),
                (155, 4),
                (28, 4),
                (20, 4),
            ]
        } else if extended_palettes {
            [
                (101, 2),
                (101, 3),
                (138, 2),
                (138, 3),
                (145, 2),
                (145, 3),
                (148, 2),
                (148, 3),
                (156, 2),
                (156, 3),
                (141, 2),
                (141, 3),
            ]
        } else {
            [
                (165, 0),
                (164, 0),
                (144, 0),
                (144, 1),
                (144, 2),
                (144, 3),
                (150, 2),
                (150, 3),
                (156, 0),
                (124, 0),
                (140, 0),
                (128, 0),
            ]
        };
        for (index, (number, variant)) in models.into_iter().enumerate() {
            let at = GridPos::new(5 + (index % 4) as i32 * 3, 2 + (index / 4) as i32 * 3);
            let mut actor = Actor::new(at, 100)
                .map_err(|e| e.to_string())?
                .with_player_relation(PlayerRelation::Hostile)
                .with_ai(AiProfile::idle())
                .with_body_profile(BodyProfile::new(100, 0).unwrap().with_base_armor(20))
                .with_attack(AttackProfile::melee(DamageType::Kinetic, 20))
                .with_tags([format!("core:monster_art_{number}_{variant}")
                    .parse()
                    .unwrap()]);
            if number == 124 {
                actor = actor.with_tags([PHYSICAL_RESISTANCE_TAG.parse().unwrap()]);
            }
            let id = game.spawn_actor(actor).map_err(|e| e.to_string())?;
            app.actor_glyphs.insert(id, 'd');
        }
        let corpse_at = GridPos::new(9, 10);
        let loot = game
            .rules()
            .items
            .iter()
            .find(|(id, _)| id.as_str() == "core:repair_patch")
            .map(|(id, _)| id.clone())
            .ok_or("Objet de capture absent")?;
        game.spawn_ground_item(corpse_at, loot, 1)
            .map_err(|e| e.to_string())?;
        game.drain_events();
        app.terminal =
            TerminalView::new(SectorDecor::default(), game.map(), game.player_visibility());
        app.terminal.title = "Bestiaire · intégration 64 px".into();
        app.terminal.combat_residue.cells.insert(
            corpse_at,
            crate::combat_visuals::ResidueCell {
                blood: 6,
                corpse_symbol: Some(if blue_palettes {
                    crate::game_art::campaign::corpse_code(101, 4)
                } else if extended_palettes {
                    crate::game_art::campaign::corpse_code(101, 3)
                } else {
                    crate::game_art::campaign::corpse_code(144, 2)
                }),
                ..Default::default()
            },
        );
        app.game = WorldState::single(game);
        app.test_lab = true;
        app.menu = MenuScreen::Hidden;
        let fingerprint = suspension::fingerprint(&app.game);
        let palettes: Vec<_> = if blue_palettes {
            project_rl::monster_palettes::all_palettes()
                .map(|p| (p.number, p.variant))
                .collect()
        } else if extended_palettes {
            project_rl::monster_palettes::MATERIAL_PALETTES
                .iter()
                .map(|p| (p.number, p.variant))
                .collect()
        } else {
            vec![(144, 2), (144, 3), (150, 2), (150, 3)]
        };
        for &(number, variant) in &palettes {
            crate::game_art::campaign::export_variant(
                number,
                variant,
                &output.join(format!("{number:03}-palette-{variant}.png")),
            );
        }
        let mut actual_sizes = Vec::new();
        for (width, height) in [(1920., 1080.), (2560., 1440.), (3840., 2160.)] {
            set_fullscreen(true);
            request_new_screen_size(width, height);
            for _ in 0..5 {
                app.draw();
                next_frame().await;
            }
            for style in crate::graphics::InterfaceStyle::ALL {
                app.graphics.active.interface_style = style;
                for _ in 0..3 {
                    app.draw();
                    next_frame().await;
                }
                app.draw();
                let summary = TerminalTargetSummary {
                    name: if extended_palettes {
                        "Démon"
                    } else {
                        "Esprit torturé"
                    }
                    .into(),
                    modifiers: if extended_palettes {
                        Vec::new()
                    } else {
                        vec![MonsterModifier::PhysicalResistance]
                    },
                    visual: Some(Visual::CampaignMonster {
                        number: if blue_palettes {
                            152
                        } else if extended_palettes {
                            156
                        } else {
                            124
                        },
                        variant: if blue_palettes {
                            4
                        } else if extended_palettes {
                            3
                        } else {
                            0
                        },
                    }),
                    distance: 4,
                    visible_state: "Aucun état visible".into(),
                    health: Some((70, 100)),
                    interaction: None,
                    analysis: Some(crate::terminal_view::TerminalTargetAnalysis {
                        integrity: 70,
                        maximum_integrity: 100,
                        armor: 20,
                        resistances: if extended_palettes {
                            "Aucune"
                        } else {
                            "PHY +30%"
                        }
                        .into(),
                    }),
                };
                crate::terminal_view::capture_inspection_in_layout(
                    app.terminal_bounds(),
                    app.ui_scale(),
                    &summary,
                );
                let image = crate::ui_capture::framebuffer()?;
                actual_sizes.push([image.width, image.height]);
                image.export_png(
                    output
                        .join(format!(
                            "bestiary-{}-{}x{}.png",
                            if style == crate::graphics::InterfaceStyle::Metal {
                                "metal"
                            } else {
                                "terminal"
                            },
                            image.width,
                            image.height
                        ))
                        .to_str()
                        .unwrap(),
                );
                next_frame().await;
            }
        }
        if suspension::fingerprint(&app.game) != fingerprint {
            return Err("La capture a modifié la simulation".into());
        }
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "native_models":129,"packaged_native_variants":240,"additional_material_palettes":palettes.len(),
            "extended_palettes":extended_palettes,
            "blue_palettes":blue_palettes,
            "simulation_unchanged":true,"real_game_renderer":true,"requested_resolutions":[[1920,1080],[2560,1440],[3840,2160]],
            "actual_framebuffer_sizes":actual_sizes,"window_manager_may_limit_requests":true,
            "interface_styles":["metal","terminal"]
        })).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }

    pub async fn capture_inspection(output: &std::path::Path) -> Result<(), String> {
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        let mut app = Self::new_agent_drone_trial()?;
        app.suspension_path = output.join("isolated-run.json");
        let fingerprint = suspension::fingerprint(&app.game);
        let targets = [
            ("Joueur", app.front_player_visual()),
            ("Drone traqueur", Visual::DroneTracker),
            ("Drone artilleur", Visual::DroneGunner),
            ("Mordeur des friches", Visual::Actor(4)),
            (
                "PNJ armé",
                Visual::ArmedActor {
                    body: 20,
                    weapon: 31,
                },
            ),
        ];
        set_fullscreen(true);
        for (width, height) in [(1920., 1080.), (2560., 1440.), (3840., 2160.)] {
            request_new_screen_size(width, height);
            for _ in 0..5 {
                app.draw();
                next_frame().await;
            }
            for style in crate::graphics::InterfaceStyle::ALL {
                app.graphics.active.interface_style = style;
                let scale = app.ui_scale();
                let draw = || {
                    // Set the active skin exactly as the game does, then draw
                    // the shared inspection panel in a controlled UI camera.
                    app.draw();
                    set_camera(&crate::graphics::ui_camera(width / scale, height / scale));
                    clear_background(Color::from_rgba(13, 19, 24, 255));
                    draw_text_bold(
                        "INSPECTION / SOURCES ACTUELLES",
                        24.,
                        30.,
                        18.,
                        UiTheme.text(),
                    );
                    for (column, (name, visual)) in targets.iter().enumerate() {
                        for row in 0..2 {
                            let summary = TerminalTargetSummary {
                                modifiers: Vec::new(),
                                name: (*name).into(),
                                visual: Some(*visual),
                                distance: 1,
                                visible_state: "Aucun état visible".into(),
                                health: Some((80, 100)),
                                interaction: None,
                                analysis: (row == 1).then(|| {
                                    crate::terminal_view::TerminalTargetAnalysis {
                                        integrity: 80,
                                        maximum_integrity: 100,
                                        armor: 4,
                                        resistances: "KIN +10% · FRD +20%".into(),
                                    }
                                }),
                            };
                            crate::terminal_view::draw_inspection_panel(
                                Rect::new(
                                    24. + column as f32 * 252.,
                                    55. + row as f32 * 255.,
                                    236.,
                                    240.,
                                ),
                                Some(&summary),
                            );
                        }
                        crate::game_art::draw_world(
                            Rect::new(60. + column as f32 * 252., 600., 64., 64.),
                            *visual,
                            1.,
                        );
                    }
                    set_default_camera();
                };
                for _ in 0..3 {
                    draw();
                    next_frame().await;
                }
                draw();
                crate::ui_capture::framebuffer()?.export_png(
                    output
                        .join(format!(
                            "inspection-{}-{}x{}.png",
                            if style == crate::graphics::InterfaceStyle::Metal {
                                "metal"
                            } else {
                                "terminal"
                            },
                            width as u32,
                            height as u32
                        ))
                        .to_str()
                        .ok_or("Chemin PNG")?,
                );
                next_frame().await;
                for (index, (name, visual)) in targets.iter().take(4).enumerate() {
                    let summary = TerminalTargetSummary {
                        modifiers: Vec::new(),
                        name: (*name).into(),
                        visual: Some(*visual),
                        distance: index as u32,
                        visible_state: "Aucun état visible".into(),
                        health: Some((80, 100)),
                        interaction: None,
                        analysis: Some(crate::terminal_view::TerminalTargetAnalysis {
                            integrity: 80,
                            maximum_integrity: 100,
                            armor: 4,
                            resistances: "KIN +10% · FRD +20%".into(),
                        }),
                    };
                    app.draw();
                    crate::terminal_view::capture_inspection_in_layout(
                        app.terminal_bounds(),
                        scale,
                        &summary,
                    )
                    .ok_or("Inspection absente dans la disposition native")?;
                    next_frame().await;
                    app.draw();
                    crate::terminal_view::capture_inspection_in_layout(
                        app.terminal_bounds(),
                        scale,
                        &summary,
                    );
                    crate::ui_capture::framebuffer()?.export_png(
                        output
                            .join(format!(
                                "jeu-{}-{}-{}x{}.png",
                                index,
                                if style == crate::graphics::InterfaceStyle::Metal {
                                    "metal"
                                } else {
                                    "terminal"
                                },
                                width as u32,
                                height as u32
                            ))
                            .to_str()
                            .ok_or("Chemin PNG")?,
                    );
                    next_frame().await;
                }
            }
        }
        if suspension::fingerprint(&app.game) != fingerprint {
            return Err("La capture a modifié la simulation".into());
        }
        Ok(())
    }
}
