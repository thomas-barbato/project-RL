//! Isolated combat measurements and the accepted FIRE-NATIVE-01 comparison.
use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    use project_rl::world::{Map, Terrain};

    #[test]
    fn cosmetic_starting_outfits_keep_identity_class_bonus_and_legacy_rules() {
        let (base, _, _, _) = ascii_game_content().unwrap();
        let classes = ascii_character_class_catalog().unwrap();
        for (_, class) in classes.iter() {
            let mut rules = base.clone();
            class
                .apply_to_rules(&mut rules, class.recommended_attributes())
                .unwrap();
            let class_armor = class.agent_start().unwrap().armor;
            let mut identities = Vec::new();
            for version in [
                CITY_POPULATION_GENERATION_VERSION,
                COSMETIC_STARTING_OUTFITS_GENERATION_VERSION,
            ] {
                let rules = rules_for_generation_version(rules.clone(), version);
                let game = GameState::new_with_rules(
                    Map::filled(7, 7, Terrain::Floor).unwrap(),
                    GridPos::new(3, 3),
                    1872,
                    rules.clone(),
                )
                .unwrap();
                let torso = game
                    .player_equipment()
                    .equipped(&"core:body_armor".parse().unwrap())
                    .unwrap();
                let id = game.player_inventory().get(torso).unwrap().item();
                let profile = rules.items.get(id).unwrap().equipment().unwrap();
                identities.push(id.clone());
                if version == COSMETIC_STARTING_OUTFITS_GENERATION_VERSION {
                    assert_eq!((profile.armor(), profile.evasion_penalty()), (0, 0), "{id}");
                    assert_eq!(
                        game.actor_armor_profile(game.player_id())
                            .unwrap()
                            .effective_against(0),
                        class_armor
                    );
                } else {
                    assert!(profile.armor() > 0, "historical {id}");
                    assert_eq!(
                        profile.evasion_penalty(),
                        if id.as_str() == "core:tenue_assaut" {
                            5
                        } else {
                            0
                        }
                    );
                }
                let world = project_rl::game::WorldState::single(game);
                let bytes = world.recovery_snapshot_bytes().unwrap();
                let restored =
                    project_rl::game::WorldState::from_recovery_snapshot_bytes(&bytes, rules)
                        .unwrap();
                assert_eq!(format!("{world:?}"), format!("{restored:?}"));
            }
            assert_eq!(identities[0], identities[1]);
        }
    }

    /// Observe real command damage with starter weapons and attributes. A tough,
    /// unarmored inert target prevents overkill from hiding the amount. No demo
    /// equipment, resource boosts, class bonuses or campaign state are changed.
    #[test]
    fn starter_offense_audit_measures_real_commands_without_rebalancing() {
        use project_rl::{skills::agent, social::PlayerRelation};
        let (base, _, _, _) = ascii_game_content().unwrap();
        let classes = ascii_character_class_catalog().unwrap();
        let mut rows = Vec::new();
        for (class_id, class) in classes.iter() {
            let mut class_rules = base.clone();
            class
                .apply_to_rules(&mut class_rules, class.recommended_attributes())
                .unwrap();
            weapon_supplies_app::ensure_class_weapon_supplies(
                &mut class_rules,
                CURRENT_GENERATION_VERSION,
            );
            let skills: &[&str] = match class_id.as_str() {
                "core:bastion" => &["ordinary", "B01"],
                "core:traqueur" => &["ordinary", "T01"],
                "core:arcaniste" => &["ordinary", "FE01", "A01", "FE02", "FR01"],
                _ => &["ordinary"],
            };
            for &id in skills {
                let starting =
                    id == "ordinary" || class.agent_start().unwrap().skills.iter().any(|s| s == id);
                let mut observed = std::collections::BTreeSet::new();
                let mut hit_count = 0;
                let mut energy_spent = 0;
                let mut weapon_id = String::new();
                for seed in 1..=32 {
                    let mut rules = class_rules.clone();
                    // Unlearned examples are explicitly labelled as such in the audit.
                    if !starting {
                        rules
                            .agent_class_start
                            .as_mut()
                            .unwrap()
                            .skills
                            .push(id.into());
                    }
                    let mut game = GameState::new_with_rules(
                        Map::filled(11, 9, Terrain::Floor).unwrap(),
                        GridPos::new(3, 4),
                        seed,
                        rules,
                    )
                    .unwrap();
                    weapon_id = game.equipped_player_weapon(0).unwrap().id().to_string();
                    let at = GridPos::new(4, 4);
                    let target = game
                        .spawn_actor(
                            Actor::new(at, 1000)
                                .unwrap()
                                .with_evasion_disabled()
                                .with_player_relation(PlayerRelation::Hostile),
                        )
                        .unwrap();
                    game.drain_events();
                    let before = game.player_energy().available();
                    let command = if id == "ordinary" {
                        GameCommand::Attack { slot: 0, target }
                    } else {
                        GameCommand::UseTechniqueAt {
                            technique: agent::technique(id),
                            target: at,
                            weapon_slot: 0,
                        }
                    };
                    assert_eq!(
                        game.process_player_command(command),
                        CommandOutcome::Applied,
                        "{class_id} {id}"
                    );
                    energy_spent = before.saturating_sub(game.player_energy().available());
                    let damage: u32 = game
                        .drain_events()
                        .iter()
                        .filter_map(|event| match event {
                            GameEvent::DamageApplied {
                                target: entity,
                                amount,
                                ..
                            } if *entity == target => Some(u32::from(*amount)),
                            _ => None,
                        })
                        .sum();
                    if damage > 0 {
                        observed.insert(damage);
                        hit_count += 1;
                    }
                }
                rows.push(serde_json::json!({
                    "class":class_id.as_str(),"skill":id,"name":agent::definition(id).map_or("Attaque ordinaire",|d|d.title.as_str()),
                    "learned_at_start":starting,"rank":1,"player_level":1,"weapon":weapon_id,
                    "sample_seeds":32,"damaging_commands":hit_count,"observed_damage":observed,"energy_spent":energy_spent,
                }));
            }
        }
        let out =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("artifacts/combat-fixes-01");
        std::fs::create_dir_all(&out).unwrap();
        std::fs::write(
            out.join("offense-audit-02.json"),
            serde_json::to_vec_pretty(&rows).unwrap(),
        )
        .unwrap();
        eprintln!("{rows:?}");
    }
}

#[cfg(debug_assertions)]
impl AsciiApp {
    pub async fn capture_combat_fixes_review(output: &std::path::Path) -> Result<(), String> {
        use crate::visual_effects::{TerminalEffectFamily as Family, TerminalEffectSample};
        std::fs::create_dir_all(output).map_err(|e| e.to_string())?;
        macroquad::miniquad::window::set_fullscreen(false);
        request_new_screen_size(1240., 760.);
        for _ in 0..3 {
            next_frame().await;
        }
        let (rules, _, _, _) = ascii_game_content()?;
        let weapon = rules
            .weapons
            .get(&"core:flamethrower".parse().unwrap())
            .ok_or("flamethrower absent")?;
        let map = project_rl::world::Map::filled(9, 9, project_rl::world::Terrain::Floor)
            .map_err(|e| e.to_string())?;
        for (name, origin, target) in [
            ("droite", GridPos::new(1, 4), GridPos::new(8, 4)),
            ("gauche", GridPos::new(7, 4), GridPos::new(0, 4)),
            ("haut", GridPos::new(4, 7), GridPos::new(4, 0)),
            ("bas", GridPos::new(4, 1), GridPos::new(4, 8)),
        ] {
            let cells = weapon.attack().affected_cells(&map, origin, target);
            for frame in 0..16 {
                set_camera(&crate::graphics::ui_camera(screen_width(), screen_height()));
                clear_background(Color::from_rgba(14, 19, 21, 255));
                draw_text(
                    &format!("FIRE-NATIVE-01 · {name} · cases de 64 px"),
                    20.,
                    25.,
                    22.,
                    WHITE,
                );
                for panel in 0..2 {
                    let x = 20. + panel as f32 * 596.;
                    draw_text(
                        if panel == 0 {
                            "Ancien rendu"
                        } else {
                            "Flammes détaillées acceptées"
                        },
                        x,
                        53.,
                        20.,
                        WHITE,
                    );
                    for y in 0..9 {
                        for col in 0..9 {
                            let at = GridPos::new(col, y);
                            let rect =
                                Rect::new(x + col as f32 * 64., 66. + y as f32 * 64., 64., 64.);
                            crate::game_art::draw_world(
                                rect,
                                crate::game_art::Visual::Terrain {
                                    kind: crate::test_sector::Decor::Deck,
                                    position: at,
                                },
                                1.,
                            );
                            if let Some(cell) = cells.iter().find(|cell| cell.position == at) {
                                let phase = frame as f32 / 16.;
                                let delta = ((at.x - origin.x) as f32, (at.y - origin.y) as f32);
                                let sample = TerminalEffectSample {
                                    symbol: '^',
                                    color: Color::from_rgba(202, 53, 19, 220),
                                    accent_color: Some(Color::from_rgba(255, 118, 24, 235)),
                                    highlight_color: Some(Color::from_rgba(255, 231, 92, 255)),
                                    outline: None,
                                    family: Family::Flame,
                                    progress: phase,
                                    motion_progress: phase,
                                    sustained: false,
                                    variant: (col * 31 + y * 17) as u8,
                                    links: 0,
                                    direction: delta,
                                    anchor: None,
                                    radius: cell.step as f32,
                                };
                                if panel == 0 {
                                    crate::terminal_view::legacy_fire_review(rect, sample);
                                } else {
                                    crate::effect_geometry::clipped(
                                        rect,
                                        Rect::new(0., 0., screen_width(), screen_height()),
                                        || crate::terminal_fire::draw(rect, sample),
                                    );
                                }
                            }
                            if at == origin {
                                crate::game_art::draw_world(
                                    rect,
                                    crate::game_art::player(
                                        Some("core:bastion"),
                                        crate::terminal_view::PlayerWeaponPose::Ranged,
                                        Some("core:flamethrower"),
                                    ),
                                    1.,
                                );
                            }
                        }
                    }
                }
                draw_text(
                    "Même arme, même portée et mêmes cases affectées. Seul le dessin change.",
                    20.,
                    674.,
                    19.,
                    GRAY,
                );
                draw_text(
                    "À droite : atlas détaillé de 128 px réduit à 64 px, sans ancien raster agrandi.",
                    20.,
                    704.,
                    19.,
                    GRAY,
                );
                let image = crate::ui_capture::framebuffer()?;
                image.export_png(
                    output
                        .join(format!("{name}-{frame:02}.png"))
                        .to_str()
                        .ok_or("capture path")?,
                );
                next_frame().await;
            }
        }
        std::fs::write(output.join("checks.json"),serde_json::to_vec_pretty(&serde_json::json!({
            "review":"FIRE-NATIVE-01","native_cell":64,"atlas_frame":128,"directions":4,"frames_per_direction":16,
            "normal_activation":crate::terminal_fire::enabled(),"gameplay_unchanged":true
        })).unwrap()).map_err(|e|e.to_string())?;
        Ok(())
    }
}
