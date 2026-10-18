//! Six client shortcuts. Assignment never advances simulation or spends resources.
use super::*;

const ACTIONS: [Action; 6] = [
    Action::Skill1,
    Action::Skill2,
    Action::Skill3,
    Action::Skill4,
    Action::Skill5,
    Action::Skill6,
];

pub(super) struct Drag {
    id: String,
    from: Option<usize>,
    origin: Vec2,
    moved: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skill_window_close_is_below_the_slots_and_closes_without_a_game_action() {
        let mut app = AsciiApp::new_agent_blast_trial().unwrap();
        app.open_agent_skill_window();
        let before = suspension::fingerprint(&app.game);
        let bar = app.ux.skill_bar.clone();
        for (width, height) in [
            (1280., 800.),
            (1920., 1080.),
            (2560., 1440.),
            (3840., 2160.),
        ] {
            let panel = app.agent_skill_bar_panel(true, width, height).unwrap();
            let (slots, switch) = bar_layout(panel, true);
            let close = skill_window_close_rect(width, height);
            assert!(switch.is_none());
            assert_eq!(close.center().x, (slots[0].x + slots[5].right()) * 0.5);
            assert!(slots.iter().all(|slot| slot.bottom() < close.y));
            assert!(close.bottom() <= panel.bottom());
        }
        app.update_input(&InputFrame {
            pointer: Some(skill_window_close_rect(1280., 800.).center().into()),
            pressed: [controls::Binding::MouseLeft].into(),
            viewport: Some((1280., 800.)),
            ..Default::default()
        });
        assert!(!app.skills_open);
        assert_eq!(app.ux.skill_bar, bar);
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn right_click_over_the_skill_bar_cancels_preview_without_unassigning_a_skill() {
        let mut app = AsciiApp::new_agent_blast_trial().unwrap();
        app.skills_open = false;
        app.activate_agent_skill_bar(0, false);
        assert!(app.attack_aim.is_some());
        let bar = app.ux.skill_bar.clone();
        let before = suspension::fingerprint(&app.game);
        let panel = app.agent_skill_bar_panel(false, 1280., 800.).unwrap();
        let point = bar_layout(panel, false).0[0].center();
        app.update_input(&InputFrame {
            pointer: Some(point.into()),
            viewport: Some((1280., 800.)),
            pressed: [controls::Binding::MouseRight].into(),
            ..Default::default()
        });
        assert!(app.attack_aim.is_none());
        assert!(app.world_context.is_none());
        assert_eq!(app.ux.skill_bar, bar);
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn every_learned_active_accepts_a_bar_drop_but_contagion_and_other_passives_do_not() {
        let mut app = AsciiApp::new_agent_skill_laboratory().unwrap();
        app.open_agent_skill_window();
        let target = bar_layout(
            app.agent_skill_bar_panel(true, 1280.0, 800.0).unwrap(),
            true,
        )
        .0[5]
            .center();
        let before = suspension::fingerprint(&app.game);
        let history = app.history.len();
        let mut actives = 0;
        let mut passives = 0;
        for skill in project_rl::skills::agent::definitions() {
            assert!(
                app.game.player_agent_skill_rank(&skill.id) > 0,
                "{}",
                skill.id
            );
            let prior = app.ux.skill_bar.clone();
            let active = skill.kind == "active";
            assert_eq!(
                app.can_assign_agent_skill(&skill.id),
                active,
                "{}",
                skill.id
            );
            // Exercise the same in-progress drag and release used by the tree.
            // A stale drag for a passive must also be rejected at the drop site.
            app.begin_agent_skill_drag(&skill.id, None, (320.0, 150.0));
            assert!(app.route_agent_skill_bar(
                &InputFrame {
                    pointer: Some(target.into()),
                    mouse_left_down: true,
                    viewport: Some((1280.0, 800.0)),
                    ..Default::default()
                },
                true
            ));
            assert!(app.route_agent_skill_bar(
                &InputFrame {
                    pointer: Some(target.into()),
                    viewport: Some((1280.0, 800.0)),
                    ..Default::default()
                },
                true
            ));
            assert!(app.ux.skill_drag.is_none());
            if active {
                actives += 1;
                assert_eq!(
                    app.ux.skill_bar[5].as_deref(),
                    Some(skill.id.as_str()),
                    "{}",
                    skill.id
                );
            } else {
                passives += 1;
                assert_eq!(app.ux.skill_bar, prior, "{}", skill.id);
            }
        }
        assert_eq!((actives, passives), (68, 26));
        assert_eq!(
            project_rl::skills::agent::definition("AF06").unwrap().kind,
            "passive"
        );
        assert!(!app.can_assign_agent_skill("AF06"));
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert_eq!(app.history.len(), history);
    }

    #[test]
    fn transformed_bar_uses_form_actions_and_restores_saved_human_slots() {
        let mut app = AsciiApp::new_agent_melee_trial().unwrap();
        let human = app.ux.skill_bar.clone();
        assert_eq!(
            app.execute_command(GameCommand::LearnAgentSkill {
                skill: "M01".into()
            }),
            CommandOutcome::AppliedWithoutTime
        );
        let command = app
            .technique_command(project_rl::skills::agent::technique("M01"))
            .unwrap();
        assert_eq!(app.execute_command(command), CommandOutcome::Applied);
        assert_eq!(app.game.player_agent_form(), Some("M01"));
        assert_eq!(
            app.displayed_agent_bar_id(0, false).as_deref(),
            Some("agent:m01_claw")
        );
        assert_eq!(
            app.displayed_agent_bar_id(5, false).as_deref(),
            Some("agent:form_return")
        );
        assert_eq!(app.ux.skill_bar, human);
        let actions = app.active_learned_techniques();
        assert!(actions.iter().any(|a| a.as_str() == "agent:m01_claw"));
        assert!(!actions.contains(&project_rl::skills::agent::technique("B01")));
        assert!(
            app.quick_technique_status(&"agent:m01_claw".parse().unwrap())
                .1
                .is_none()
        );
        app.activate_agent_skill_bar(0, false);
        assert_eq!(
            app.attack_aim_technique.as_ref().map(|a| a.as_str()),
            Some("agent:m01_claw")
        );
        app.attack_aim = None;
        app.attack_aim_technique = None;
        let command = app
            .technique_command("agent:form_return".parse().unwrap())
            .unwrap();
        assert!(matches!(
            app.execute_command(command),
            CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime
        ));
        assert!(app.game.player_agent_form().is_none());
        assert_eq!(app.ux.skill_bar, human);
        assert_eq!(app.displayed_agent_bar_id(0, false).as_deref(), Some("B01"));
    }

    #[test]
    fn default_attack_in_form_uses_its_attack_without_consulting_the_weapon() {
        let mut app = AsciiApp::new_agent_melee_trial().unwrap();
        assert_eq!(
            app.execute_command(GameCommand::LearnAgentSkill {
                skill: "M01".into()
            }),
            CommandOutcome::AppliedWithoutTime
        );
        let command = app
            .technique_command(project_rl::skills::agent::technique("M01"))
            .unwrap();
        assert_eq!(app.execute_command(command), CommandOutcome::Applied);
        app.select_world_target(Some(GridPos::new(5, 4)));
        assert!(
            matches!(app.weapon_command(None),Some(GameCommand::UseTechniqueAt {technique,..}) if technique.as_str()=="agent:m01_claw")
        );
    }

    #[test]
    fn drone_bar_shortcuts_and_clicks_open_placement_then_invoke_without_a_weapon_requirement() {
        for index in 0..3 {
            for pointer in [true, false] {
                let mut app = AsciiApp::new_agent_drone_trial().unwrap();
                let before = suspension::fingerprint(&app.game);
                if pointer {
                    let at = bar_layout(
                        app.agent_skill_bar_panel(false, 1280., 800.).unwrap(),
                        false,
                    )
                    .0[index]
                        .center();
                    app.update_input(&InputFrame {
                        pointer: Some(at.into()),
                        pressed: [controls::Binding::MouseLeft].into(),
                        mouse_left_down: true,
                        viewport: Some((1280., 800.)),
                        ..Default::default()
                    });
                    app.update_input(&InputFrame {
                        pointer: Some(at.into()),
                        viewport: Some((1280., 800.)),
                        ..Default::default()
                    });
                } else {
                    app.update_input(&InputFrame {
                        pressed: [app.controls.binding(ACTIONS[index]).clone()].into(),
                        ..Default::default()
                    });
                }
                assert_eq!(suspension::fingerprint(&app.game), before);
                assert!(app.attack_aim.is_some());
                app.update_input(&InputFrame {
                    pressed: [app.controls.binding(Action::Attack).clone()].into(),
                    ..Default::default()
                });
                assert_eq!(app.game.turn(), 1);
                assert_eq!(app.game.player_agent_drone_count(), 1);
                assert!(app.attack_aim.is_none());
            }
        }
    }

    #[test]
    fn fireball_burn_uses_painted_body_effect_without_legacy_flame_overlay() {
        let mut app = AsciiApp::new_agent_fireball_trial().unwrap();
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Skill1).clone()].into(),
            ..Default::default()
        });
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Attack).clone()].into(),
            ..Default::default()
        });
        let (id, actor) = app
            .game
            .actors()
            .iter()
            .find(|(_, a)| {
                a.statuses()
                    .any(|s| s.definition.as_str() == "core:burning")
            })
            .unwrap();
        assert!(
            !app.sustained_effects_at(actor.position(), 0.0, false)
                .iter()
                .any(|e| e.family == crate::visual_effects::TerminalEffectFamily::Flame)
        );
        let now = 0.0;
        assert!(app.combat_motion.impact_time(id, now) > now);
        assert_eq!(app.combat_motion.offset(id, now, false), Vec2::ZERO);
    }
    #[test]
    fn fireball_bar_mouse_and_key_prepare_the_same_area_command() {
        for pointer in [false, true] {
            let mut app = AsciiApp::new_agent_fireball_trial().unwrap();
            let before = suspension::fingerprint(&app.game);
            if pointer {
                let at = bar_layout(
                    app.agent_skill_bar_panel(false, 1280., 800.).unwrap(),
                    false,
                )
                .0[0]
                    .center();
                app.update_input(&InputFrame {
                    pointer: Some(at.into()),
                    pressed: [controls::Binding::MouseLeft].into(),
                    mouse_left_down: true,
                    viewport: Some((1280., 800.)),
                    ..Default::default()
                });
                app.update_input(&InputFrame {
                    pointer: Some(at.into()),
                    viewport: Some((1280., 800.)),
                    ..Default::default()
                });
            } else {
                app.update_input(&InputFrame {
                    pressed: [app.controls.binding(Action::Skill1).clone()].into(),
                    ..Default::default()
                });
            }
            assert_eq!(suspension::fingerprint(&app.game), before);
            assert!(
                app.aimed_attack_preview(app.attack_aim.unwrap())
                    .unwrap()
                    .cells()
                    .len()
                    > 9
            );
            app.update_input(&InputFrame {
                pressed: [app.controls.binding(Action::Attack).clone()].into(),
                ..Default::default()
            });
            assert_eq!(app.game.turn(), 1);
            assert!(app.attack_aim.is_none());
        }
    }
    #[test]
    fn occult_blast_bar_mouse_and_key_prepare_the_same_area_command() {
        for pointer in [false, true] {
            let mut app = AsciiApp::new_agent_blast_trial().unwrap();
            let before = suspension::fingerprint(&app.game);
            if pointer {
                let at = bar_layout(
                    app.agent_skill_bar_panel(false, 1280., 800.).unwrap(),
                    false,
                )
                .0[0]
                    .center();
                app.update_input(&InputFrame {
                    pointer: Some(at.into()),
                    pressed: [controls::Binding::MouseLeft].into(),
                    mouse_left_down: true,
                    viewport: Some((1280., 800.)),
                    ..Default::default()
                });
                app.update_input(&InputFrame {
                    pointer: Some(at.into()),
                    viewport: Some((1280., 800.)),
                    ..Default::default()
                });
            } else {
                app.update_input(&InputFrame {
                    pressed: [app.controls.binding(Action::Skill1).clone()].into(),
                    ..Default::default()
                });
            }
            assert_eq!(suspension::fingerprint(&app.game), before);
            assert_eq!(
                app.aimed_attack_preview(app.attack_aim.unwrap())
                    .unwrap()
                    .cells()
                    .len(),
                9
            );
            app.update_input(&InputFrame {
                pressed: [app.controls.binding(Action::Attack).clone()].into(),
                ..Default::default()
            });
            assert_eq!(app.game.turn(), 1);
            assert!(app.attack_aim.is_none());
        }
    }
    #[test]
    fn rupture_bar_click_and_key_use_the_learned_skill_with_the_matching_hand() {
        for pointer in [true, false] {
            let mut app = AsciiApp::new_agent_melee_trial().unwrap();
            let before = suspension::fingerprint(&app.game);
            if pointer {
                let at = bar_layout(
                    app.agent_skill_bar_panel(false, 1280., 800.).unwrap(),
                    false,
                )
                .0[0]
                    .center();
                app.update_input(&InputFrame {
                    pointer: Some(at.into()),
                    pressed: [controls::Binding::MouseLeft].into(),
                    mouse_left_down: true,
                    viewport: Some((1280., 800.)),
                    ..Default::default()
                });
                app.update_input(&InputFrame {
                    pointer: Some(at.into()),
                    viewport: Some((1280., 800.)),
                    ..Default::default()
                });
            } else {
                app.update_input(&InputFrame {
                    pressed: [app.controls.binding(Action::Skill1).clone()].into(),
                    ..Default::default()
                });
            }
            assert_eq!(app.attack_aim.unwrap().slot, 0);
            assert_eq!(suspension::fingerprint(&app.game), before);
            let energy = app.game.player_energy().available();
            app.update_input(&InputFrame {
                pressed: [app.controls.binding(Action::Attack).clone()].into(),
                ..Default::default()
            });
            assert_eq!(app.game.turn(), 1);
            // The Arcanist fixture also restores energy at the end of its turn.
            assert_eq!(
                app.game.player_energy().available(),
                energy - 6 + app.game.rules().player_energy_regeneration
            );
            assert!(app.attack_aim.is_none());
        }
    }

    #[test]
    fn weapon_switch_cycles_real_slots_without_time_and_cancels_old_aim() {
        let mut app = AsciiApp::new_agent_weapon_switch_trial().unwrap();
        app.skills_open = false;
        let before = suspension::fingerprint(&app.game);
        let history = app.history.len();
        app.begin_technique_attack_aim(project_rl::skills::agent::technique("A01"));
        assert!(app.attack_aim.is_some());
        let button = bar_layout(
            app.agent_skill_bar_panel(false, 1280., 800.).unwrap(),
            false,
        )
        .1
        .unwrap();
        let click = InputFrame {
            pointer: Some(button.center().into()),
            pressed: [controls::Binding::MouseLeft].into(),
            mouse_left_down: true,
            viewport: Some((1280., 800.)),
            ..Default::default()
        };
        for expected in [1, 2, 0] {
            app.update_input(&click);
            assert_eq!(app.active_weapon_slot, expected);
            assert!(app.attack_aim.is_none());
            assert_eq!(suspension::fingerprint(&app.game), before);
            assert_eq!(app.history.len(), history);
            app.update_input(&InputFrame {
                mouse_left_down: true,
                ..Default::default()
            });
            assert_eq!(app.active_weapon_slot, expected); // Holding is not another click.
            app.update_input(&InputFrame::default());
        }
        app.skills_open = true;
        app.update_input(&click);
        assert_eq!(app.active_weapon_slot, 0);
    }

    #[test]
    fn weapon_switch_skips_empty_slots_and_does_nothing_with_only_one_weapon() {
        let mut app = AsciiApp::new_agent_weapon_switch_trial().unwrap();
        let item = app.game.equipped_player_weapon_item(1).unwrap();
        assert_eq!(
            app.execute_command(GameCommand::DropItem { item }),
            CommandOutcome::Applied
        );
        let before = suspension::fingerprint(&app.game);
        app.cycle_bar_weapon();
        assert_eq!(app.active_weapon_slot, 2);
        app.cycle_bar_weapon();
        assert_eq!(app.active_weapon_slot, 0);
        assert_eq!(suspension::fingerprint(&app.game), before);
        let item = app.game.equipped_player_weapon_item(2).unwrap();
        // The first dropped weapon already occupies the current ground cell.
        assert_eq!(
            app.execute_command(GameCommand::Move(Direction::South)),
            CommandOutcome::Applied
        );
        assert_eq!(
            app.execute_command(GameCommand::DropItem { item }),
            CommandOutcome::Applied
        );
        let before = suspension::fingerprint(&app.game);
        assert!(app.next_bar_weapon_slot().is_none());
        app.cycle_bar_weapon();
        assert_eq!(app.active_weapon_slot, 0);
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn weapon_switch_is_only_in_world_bar_and_stays_inside_small_panels() {
        for width in [300., 400., 674., 1100.] {
            let panel = Rect::new(20., 600., width, 52.);
            let (rects, switch) = bar_layout(panel, false);
            let switch = switch.unwrap();
            assert!(switch.x >= panel.x);
            assert!(rects[5].right() <= panel.right());
            assert!(rects[0].x >= switch.right() + 11.);
            assert!(bar_layout(panel, true).1.is_none());
        }
    }

    #[test]
    fn equipment_only_skill_is_assignable_and_disappears_from_bar_when_source_is_removed() {
        let mut app = AsciiApp::new_agent_spell_trial().unwrap();
        let mut rules = app.game.rules().clone();
        let source = app.game.equipped_player_weapon_item(0).unwrap();
        let model = app
            .game
            .player_inventory()
            .get(source)
            .unwrap()
            .item()
            .clone();
        rules
            .equipment_agent_skills
            .insert(model, [("FE02".into(), 1)].into());
        let game = GameState::new_with_rules(
            app.game.map().clone(),
            app.game.player_position().unwrap(),
            19,
            rules,
        )
        .unwrap();
        app.game = WorldState::single(game);
        let source = app.game.equipped_player_weapon_item(0).unwrap();
        assert_eq!(app.game.player_skills().agent_skill_rank("FE02"), 0);
        app.assign_agent_skill(5, "FE02");
        assert_eq!(app.ux.skill_bar[5].as_deref(), Some("FE02"));
        assert!(
            app.active_learned_techniques()
                .contains(&project_rl::skills::agent::technique("FE02"))
        );
        assert_eq!(
            app.execute_command(GameCommand::DropItem { item: source }),
            CommandOutcome::Applied
        );
        assert!(app.ux.skill_bar[5].is_none());
        assert!(
            !app.active_learned_techniques()
                .contains(&project_rl::skills::agent::technique("FE02"))
        );
    }

    #[test]
    fn skill_bar_drags_learned_actives_without_learning_or_spending_and_launches_with_its_key() {
        let mut app = AsciiApp::new_agent_spell_trial().unwrap();
        let before = suspension::fingerprint(&app.game);
        let history = app.history.len();
        app.update_input(&InputFrame {
            pointer: Some((320., 150.)),
            pressed: [controls::Binding::MouseLeft].into(),
            mouse_left_down: true,
            viewport: Some((1280., 800.)),
            ..Default::default()
        });
        assert!(app.ux.skill_drag.is_some());
        let panel = app.agent_skill_bar_panel(true, 1280., 800.).unwrap();
        let at = bar_layout(panel, true).0[2].center();
        app.update_input(&InputFrame {
            pointer: Some((at.x, at.y)),
            mouse_left_down: true,
            viewport: Some((1280., 800.)),
            ..Default::default()
        });
        app.update_input(&InputFrame {
            pointer: Some((at.x, at.y)),
            viewport: Some((1280., 800.)),
            ..Default::default()
        });
        assert_eq!(app.ux.skill_bar[2].as_deref(), Some("A01"));
        assert!(app.ux.skill_bar[0].is_none());
        app.assign_agent_skill(4, "A03"); // Passive, although learned.
        app.assign_agent_skill(4, "FR06"); // Active, but not learned.
        assert!(app.ux.skill_bar[4].is_none());
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert_eq!(app.history.len(), history);
        app.skills_open = false;
        app.update_input(&InputFrame {
            pressed: [controls::Binding::key("Key3")].into(),
            ..Default::default()
        });
        assert!(app.attack_aim.is_some());
        assert_eq!(suspension::fingerprint(&app.game), before);
        app.update_input(&InputFrame {
            pressed: [app.controls.binding(Action::Attack).clone()].into(),
            ..Default::default()
        });
        assert_eq!(app.game.turn(), 1);
        assert!(app.attack_aim.is_none());
    }

    #[test]
    fn skill_bar_mouse_click_uses_and_right_click_removes_without_world_actions() {
        let mut app = AsciiApp::new_agent_spell_trial().unwrap();
        app.skills_open = false;
        let before = suspension::fingerprint(&app.game);
        let at = bar_layout(
            app.agent_skill_bar_panel(false, 1280., 800.).unwrap(),
            false,
        )
        .0[0]
            .center();
        let frame = |down, pressed| InputFrame {
            pointer: Some((at.x, at.y)),
            mouse_left_down: down,
            pressed,
            viewport: Some((1280., 800.)),
            ..Default::default()
        };
        app.update_input(&frame(true, [controls::Binding::MouseLeft].into()));
        app.update_input(&frame(false, Default::default()));
        assert!(app.attack_aim.is_some());
        assert_eq!(suspension::fingerprint(&app.game), before);
        app.update_input(&InputFrame {
            pause: true,
            ..Default::default()
        });
        app.update_input(&frame(false, [controls::Binding::MouseRight].into()));
        assert!(app.ux.skill_bar[0].is_none());
        assert!(!app.technique_menu_open);
        assert_eq!(suspension::fingerprint(&app.game), before);
    }
}

fn slots(panel: Rect) -> [Rect; 6] {
    let side = 48.0_f32.min((panel.w - 30.) / 6.);
    let width = side * 6. + 30.;
    std::array::from_fn(|i| {
        Rect::new(
            panel.x + (panel.w - width) * 0.5 + i as f32 * (side + 6.),
            panel.y + (panel.h - side) * 0.5,
            side,
            side,
        )
    })
}

fn bar_layout(panel: Rect, editing: bool) -> ([Rect; 6], Option<Rect>) {
    if editing {
        return (slots(Rect::new(panel.x, panel.y, panel.w, 52.)), None);
    }
    let skills = slots(Rect::new(panel.x + 60., panel.y, panel.w - 60., panel.h));
    let first = skills[0];
    (
        skills,
        Some(Rect::new(first.x - 60., first.y, 48., first.h)),
    )
}

pub(super) fn skill_window_close_rect(width: f32, height: f32) -> Rect {
    Rect::new((width - 160.) * 0.5, height - 60., 160., 28.)
}

impl AsciiApp {
    fn next_bar_weapon_slot(&self) -> Option<u8> {
        let count = self.game.rules().player_weapon_slots.len().min(3);
        if count == 0 {
            return None;
        }
        (1..=count)
            .map(|step| ((usize::from(self.active_weapon_slot) + step) % count) as u8)
            .find(|&slot| {
                slot != self.active_weapon_slot && self.game.equipped_player_weapon(slot).is_some()
            })
    }

    pub(super) fn cycle_bar_weapon(&mut self) {
        if let Some(slot) = self.next_bar_weapon_slot() {
            self.select_weapon_slot(slot);
            self.attack_aim = None;
            self.attack_aim_technique = None;
            self.attack_aim_pointer = None;
            self.update_crash_recovery();
        }
    }
    fn displayed_agent_bar_id(&self, index: usize, editing: bool) -> Option<String> {
        if !editing && let Some(form) = self.game.player_agent_form() {
            return match index {
                0..=2 => self
                    .game
                    .player_agent_form_actions()
                    .get(index)
                    .map(|a| a.technique.to_string()),
                3 => Some(form.into()),
                4 => (self.game.player_agent_skill_rank("M02") > 0).then(|| "M02".into()),
                5 => Some("agent:form_return".into()),
                _ => None,
            };
        }
        self.ux.skill_bar.get(index).cloned().flatten()
    }

    fn bar_technique(id: &str) -> TechniqueId {
        if id.starts_with("agent:") {
            id.parse().expect("form action identifier")
        } else {
            project_rl::skills::agent::technique(id)
        }
    }

    pub(super) fn can_assign_agent_skill(&self, id: &str) -> bool {
        self.game.player_agent_skill_rank(id) > 0
            && project_rl::skills::agent::definition(id).is_some_and(|s| s.kind == "active")
    }
    fn agent_skill_bar_panel(&self, editing: bool, w: f32, h: f32) -> Option<Rect> {
        if editing {
            return Some(Rect::new(28., h - 124., w - 56., 96.));
        }
        crate::terminal_view::terminal_skill_bar_panel(
            &self.game,
            self.terminal_bounds_for(w, h, 1.),
            1.,
            self.navigation_signal_summary().is_some(),
        )
    }
    pub(super) fn sync_agent_skill_bar(&mut self) {
        for index in 0..self.ux.skill_bar.len() {
            if self.ux.skill_bar[index]
                .as_deref()
                .is_some_and(|id| !self.can_assign_agent_skill(id))
            {
                self.ux.skill_bar[index] = None;
            }
        }
        if self
            .ux
            .skill_drag
            .as_ref()
            .is_some_and(|drag| !self.can_assign_agent_skill(&drag.id))
        {
            self.ux.skill_drag = None;
        }
        if self
            .attack_aim_technique
            .as_ref()
            .and_then(project_rl::skills::agent::from_technique)
            .is_some_and(|skill| self.game.player_agent_skill_rank(&skill.id) == 0)
        {
            self.attack_aim = None;
            self.attack_aim_technique = None;
        }
    }
    pub(super) fn begin_agent_skill_drag(
        &mut self,
        id: &str,
        from: Option<usize>,
        point: (f32, f32),
    ) {
        self.ux.skill_drag = Some(Drag {
            id: id.into(),
            from,
            origin: point.into(),
            moved: false,
        });
    }
    fn assign_agent_skill(&mut self, index: usize, id: &str) {
        if !self.can_assign_agent_skill(id) {
            return;
        }
        if let Some(previous) = self
            .ux
            .skill_bar
            .iter()
            .position(|s| s.as_deref() == Some(id))
        {
            self.ux.skill_bar.swap(previous, index);
        } else {
            self.ux.skill_bar[index] = Some(id.into());
        }
        self.update_crash_recovery();
    }
    fn activate_agent_skill_bar(&mut self, index: usize, editing: bool) {
        let Some(id) = self.displayed_agent_bar_id(index, editing) else {
            return;
        };
        if (!self.game.player_agent_form().is_some() || editing)
            && !self.can_assign_agent_skill(&id)
        {
            return;
        }
        if editing {
            let _ = self.select_agent_skill_for_capture(&id);
            return;
        }
        if let Some(command) = self.technique_command(Self::bar_technique(&id)) {
            self.execute_command(command);
            self.capture_events();
        }
    }
    pub(super) fn route_agent_skill_bar(&mut self, input: &InputFrame, editing: bool) -> bool {
        if input.pause {
            self.ux.skill_drag = None;
            return false;
        }
        if self.game.rules().agent_class_start.is_none()
            || self.menu != MenuScreen::Hidden
            || self.game.status() != RunStatus::Active
            || self.inventory_open
            || self.character_open
            || self.legend_open
            || self.report_open
            || self.quest_journal_open
            || self.technique_menu_open
            || self.npc_interaction.is_some()
            || self.installation_menu.is_some()
            || self.context_menu.is_some()
            || self.ux.item_card.is_some()
            || self.ux.resources_open
            || self.ux.lab_open
            || self.ux.history_open
            || self.ux.inspected_target
            || self.character_creation.is_some()
        {
            self.ux.skill_drag = None;
            return false;
        }
        let (w, h) = input.viewport.unwrap_or((1280., 800.));
        let Some(panel) = self.agent_skill_bar_panel(editing, w, h) else {
            return false;
        };
        let (rects, weapon_switch) = bar_layout(panel, editing);
        self.ux.skill_bar_pointer = input.pointer;
        let hovered = input
            .pointer
            .and_then(|p| rects.iter().position(|r| r.contains(p.into())));
        if !editing && self.game.player_agent_form().is_some() {
            self.ux.skill_drag = None;
        }
        if let Some(mut drag) = self.ux.skill_drag.take() {
            drag.moved |= input
                .pointer
                .is_some_and(|p| drag.origin.distance(p.into()) > 6.);
            if input.mouse_left_down {
                self.ux.skill_drag = Some(drag);
                return true;
            }
            if !drag.moved && drag.from.is_some() && hovered == drag.from {
                self.activate_agent_skill_bar(drag.from.unwrap(), editing);
            } else if let Some(index) = hovered {
                self.assign_agent_skill(index, &drag.id);
            } else if drag.moved
                && let Some(index) = drag.from
            {
                self.ux.skill_bar[index] = None;
                self.update_crash_recovery();
            } else if !drag.moved {
                if let Some(index) = drag.from {
                    self.activate_agent_skill_bar(index, editing);
                }
            }
            return true;
        }
        if weapon_switch.is_some_and(|r| input.pointer.is_some_and(|p| r.contains(p.into()))) {
            if input.pressed.contains(&controls::Binding::MouseLeft) {
                self.cycle_bar_weapon();
                return true;
            }
            if input.pressed.contains(&controls::Binding::MouseRight) {
                return true;
            }
        }
        if !editing
            && let Some(index) = ACTIONS
                .iter()
                .position(|a| self.controls.pressed(*a, input))
        {
            self.activate_agent_skill_bar(index, false);
            return true;
        }
        if let Some(index) = hovered {
            if !editing && self.game.player_agent_form().is_some() {
                if input.pressed.contains(&controls::Binding::MouseLeft) {
                    self.activate_agent_skill_bar(index, false);
                    return true;
                }
                if input.pressed.contains(&controls::Binding::MouseRight) {
                    return true;
                }
            }
            if input.pressed.contains(&controls::Binding::MouseRight) {
                self.ux.skill_bar[index] = None;
                self.update_crash_recovery();
                return true;
            }
            if input.pressed.contains(&controls::Binding::MouseLeft) {
                if let Some(id) = self.ux.skill_bar[index].clone() {
                    if input.mouse_left_down {
                        self.begin_agent_skill_drag(&id, Some(index), input.pointer.unwrap());
                    } else {
                        self.activate_agent_skill_bar(index, editing);
                    }
                }
                return true;
            }
        }
        false
    }
    pub(super) fn draw_agent_skill_bar(&self, editing: bool) {
        if !editing
            && (self.game.rules().agent_class_start.is_none()
                || self.skills_open
                || self.menu != MenuScreen::Hidden
                || self.legend_open)
        {
            return;
        }
        let Some(panel) = self.agent_skill_bar_panel(editing, self.ui_width(), self.ui_height())
        else {
            return;
        };
        let (rects, weapon_switch) = bar_layout(panel, editing);
        if editing {
            let close = skill_window_close_rect(self.ui_width(), self.ui_height());
            UiTheme.button(
                close,
                "Fermer",
                self.ux
                    .skill_bar_pointer
                    .is_some_and(|p| close.contains(p.into())),
                false,
                true,
                ButtonTone::Secondary,
            );
        }
        for (index, r) in rects.iter().copied().enumerate() {
            let hover = self
                .ux
                .skill_bar_pointer
                .is_some_and(|p| r.contains(p.into()));
            draw_rectangle(r.x, r.y, r.w, r.h, Color::from_rgba(10, 12, 16, 245));
            if let Some(id) = self.displayed_agent_bar_id(index, editing) {
                let technique = Self::bar_technique(&id);
                let icon = if id.starts_with("agent:") {
                    self.game.player_agent_form().unwrap_or("M01")
                } else {
                    &id
                };
                agent_skill_window::art(
                    icon,
                    "compact",
                    Rect::new(r.x + 3., r.y + 3., r.w - 6., r.h - 6.),
                    None,
                );
                if !editing && self.quick_technique_status(&technique).1.is_some() {
                    draw_rectangle(
                        r.x + 2.,
                        r.y + 2.,
                        r.w - 4.,
                        r.h - 4.,
                        Color::new(0., 0., 0., 0.48),
                    );
                }
                if hover {
                    let title = self.technique_name(&technique);
                    let text = &title;
                    let width = measure_text_bold(text, 14).width + 16.;
                    let left = (r.center().x - width * 0.5).clamp(8., self.ui_width() - width - 8.);
                    draw_rectangle(
                        left,
                        panel.y - 28.,
                        width,
                        24.,
                        Color::from_rgba(10, 12, 16, 250),
                    );
                    draw_text(text, left + 8., panel.y - 11., 14.0, UiTheme.text());
                }
            }
            draw_rectangle_lines(
                r.x,
                r.y,
                r.w,
                r.h,
                if hover { 2. } else { 1. },
                if hover {
                    UiTheme.accent()
                } else {
                    UiTheme.muted()
                },
            );
            let binding = self.controls.binding(ACTIONS[index]);
            let key = match binding {
                controls::Binding::Key(name) => name.strip_prefix("Key").unwrap_or(name).to_owned(),
                controls::Binding::MouseLeft => "MG".into(),
                controls::Binding::MouseRight => "MD".into(),
                controls::Binding::MouseMiddle => "MM".into(),
            };
            let badge_width = (measure_text_bold(&key, 11).width + 8.).clamp(16., r.w - 4.);
            draw_rectangle(
                r.x + 2.,
                r.y + 2.,
                badge_width,
                14.,
                Color::from_rgba(3, 4, 6, 225),
            );
            draw_text(&key, r.x + 6., r.y + 13., 12.0, UiTheme.text());
        }
        if let Some(rect) = weapon_switch {
            let hover = self
                .ux
                .skill_bar_pointer
                .is_some_and(|p| rect.contains(p.into()));
            let enabled = self.next_bar_weapon_slot().is_some();
            UiTheme.button(rect, "", hover, false, enabled, ButtonTone::Secondary);
            let color = if enabled {
                UiTheme.accent()
            } else {
                UiTheme.muted()
            };
            let center = rect.center();
            let radius = 12.5;
            // A clean open loop with a tangent arrowhead, centered without a counter.
            let start = 0.12_f32;
            let end = std::f32::consts::TAU - 0.80;
            let point = |angle: f32| center + vec2(angle.cos(), angle.sin()) * radius;
            let mut previous = point(start);
            draw_circle(previous.x, previous.y, 1.4, color);
            for step in 1..=40 {
                let next = point(start + (end - start) * step as f32 / 40.);
                draw_line(previous.x, previous.y, next.x, next.y, 2.8, color);
                previous = next;
            }
            let tangent = vec2(-end.sin(), end.cos());
            let normal = vec2(end.cos(), end.sin());
            draw_triangle(
                previous + tangent * 2.5,
                previous - tangent * 4.5 + normal * 4.5,
                previous - tangent * 4.5 - normal * 4.5,
                color,
            );
            if hover {
                let name = self
                    .equipped_instance_name(self.active_weapon_slot)
                    .unwrap_or_else(|| self.hand_slot_label(usize::from(self.active_weapon_slot)));
                let text = format!("Changer d’arme · {name}");
                let width = (measure_text_bold(&text, 14).width + 16.).min(self.ui_width() - 16.);
                let left = (rect.right() - width).clamp(8., self.ui_width() - width - 8.);
                draw_rectangle(
                    left,
                    panel.y - 28.,
                    width,
                    24.,
                    Color::from_rgba(10, 12, 16, 250),
                );
                draw_text_bold_centered(
                    &text,
                    Rect::new(left + 6., panel.y - 28., width - 12., 24.),
                    14,
                    UiTheme.text(),
                );
            }
        }
        if let Some(drag) = &self.ux.skill_drag
            && drag.moved
            && let Some(p) = self.ux.skill_bar_pointer
        {
            agent_skill_window::art(
                &drag.id,
                "compact",
                Rect::new(p.0 - 22., p.1 - 22., 44., 44.),
                None,
            );
        }
    }
}
