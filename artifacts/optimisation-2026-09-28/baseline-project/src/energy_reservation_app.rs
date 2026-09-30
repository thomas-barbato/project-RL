//! Shared energy/party reader. Only explicit buttons commit game actions.
use super::ux::{draw_reader, reader_body, reader_close, reader_panel};
use super::*;

impl AsciiApp {
    #[cfg(debug_assertions)]
    pub(super) fn prepare_reserved_energy_diagnostic(&mut self) -> Result<(), String> {
        use project_rl::progression::{ExperienceAward, RunProgression, encode_player_progression};
        use project_rl::skills::SkillProgressionState;
        let mut rules = self.rules.clone();
        rules.progression.starting_skill_points = 100;
        rules.player_starting_energy = 60;
        rules.player_energy_capacity = 100;
        let skills = SkillProgressionState::from_ordered_choices(
            [
                (
                    "core:furtivite".parse().unwrap(),
                    ["fur_01", "fur_03", "fur_05", "fur_07", "fur_09"]
                        .into_iter()
                        .map(|s| format!("core:{s}").parse().unwrap())
                        .collect(),
                ),
                (
                    "core:guerre_electronique".parse().unwrap(),
                    ["gel_01", "gel_03"]
                        .into_iter()
                        .map(|s| format!("core:{s}").parse().unwrap())
                        .collect(),
                ),
            ],
            &rules.skills,
            &rules.enabled_system_features,
            &rules.skill_progression,
        )
        .map_err(|e| e.to_string())?;
        let mut run =
            RunProgression::with_starting_skill_points(rules.progression.starting_skill_points);
        run.award(&ExperienceAward::repeatable(100_000), &rules.progression);
        for (_, choices) in skills.disciplines() {
            for n in 1..=choices.len() {
                run.spend_skill_points(rules.skill_progression.cost_for_choice_number(n).unwrap())
                    .map_err(|e| e.to_string())?;
            }
        }
        let map =
            project_rl::world::Map::filled(24, 18, project_rl::world::Terrain::Floor).unwrap();
        let mut game = GameState::new_with_rules(map, GridPos::new(9, 9), 0, rules)
            .map_err(|e| e.to_string())?;
        game.restore_player_progression(
            &encode_player_progression(&run, &skills).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        self.terminal = TerminalView::new(
            crate::test_sector::SectorDecor::default(),
            game.map(),
            game.player_visibility(),
        );
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.intro_city_reached = true;
        for x in 7..10 {
            let profile = DroneProfile::new(
                "core:diagnostic_companion".parse().unwrap(),
                6,
                50,
                40,
                1,
                3,
                6,
                1,
                1,
                DroneCapabilities::default(),
            )
            .unwrap();
            self.game
                .spawn_manifested_player_drone(
                    Actor::new(GridPos::new(x, 10), 10).unwrap(),
                    profile,
                    20,
                    20,
                )
                .map_err(|e| e.to_string())?;
        }
        for command in [
            GameCommand::UseTechnique {
                technique: "core:fur_09".parse().unwrap(),
                targets: vec![],
                weapon_slot: None,
            },
            GameCommand::UseElectronicWarfareTechnique {
                technique: "core:gel_03".parse().unwrap(),
                directive: ElectronicDirective::Jam {
                    channel: ElectronicChannel::ControlLink,
                },
            },
        ] {
            let outcome = self.execute_command(command);
            if outcome != CommandOutcome::Applied {
                return Err(format!("Activation de diagnostic refusée : {outcome:?}"));
            }
        }
        self.capture_events_at(Some(0.0));
        Ok(())
    }

    pub(super) fn resource_technique_description(
        &self,
        definition: &project_rl::skills::TechniqueDefinition,
    ) -> &str {
        if self.game.rules().maintained_energy_reservations {
            match definition.action() {
                Some(TechniqueAction::ToggleActiveCamouflage { .. }) => {
                    return "Renforce temporairement votre dissimulation optique en réservant de l'énergie jusqu'à la fin du camouflage.";
                }
                Some(TechniqueAction::DivertSubnet { .. }) => {
                    return "Maintient une même consigne sur plusieurs dispositifs autorisés en réservant de l'énergie pour leurs contrôles.";
                }
                Some(TechniqueAction::ElectronicPulse {
                    filter_identified_allies: true,
                    ..
                }) => {
                    return "Émet une impulsion électronique qui épargne les alliés matériellement identifiés, au prix d'énergie.";
                }
                _ => {}
            }
        }
        self.texts
            .resolve(DISPLAY_LOCALE, definition.description_key())
            .unwrap_or("Description indisponible.")
    }

    pub(super) fn overclock_resource_lines(&self) -> Vec<String> {
        self.game.player_inventory().iter().filter_map(|item| {
            let overclock = self.game.equipment_engineering_state(item.instance())?.overclock()?;
            let heat = self.game.player_heat()?;
            Some(format!("{} surcadencé : chaleur {}, +{} par utilisation, usure au-delà de {}, limite {}, refroidissement {} par tour. Encore {} tours.", self.item_name(item.item()), heat.current(), overclock.heat_per_use(), overclock.safe_heat_threshold(), overclock.maximum_heat_threshold(), heat.dissipation_per_phase(), overclock.remaining_time_units()))
        }).collect()
    }
    pub(super) fn modern_resource_usage(
        &self,
        definition: &project_rl::skills::TechniqueDefinition,
    ) -> Option<String> {
        if let Some(TechniqueImprovement::DroneAutonomousScout {
            maximum_unknown_steps,
            energy_cost_override,
            ..
        }) = definition.improvement()
        {
            return Some(format!(
                "Passif de Patrouille bornée : autorise jusqu'à {maximum_unknown_steps} nouvelles cases avant retour, avec {energy_cost_override} E réservés tant que la routine est active. Le rapport reste daté et n'accorde aucune vision directe."
            ));
        }
        let timing = definition
            .preparation_steps()
            .map_or("Action 1 tour".to_owned(), |steps| {
                format!("Préparation {} tour(s), puis action", steps.get())
            });
        let reservation = " L'énergie réservée est libérée à l'arrêt ou à la fin de l'effet.";
        let text = match definition.action()? {
            TechniqueAction::ToggleActiveCamouflage {
                activation_energy,
                optical_difficulty_bonus,
                maximum_duration,
                ..
            } => format!(
                "{timing} · {activation_energy} E réservés. Augmente la dissimulation optique de {optical_difficulty_bonus} pendant au plus {maximum_duration} tours. Une attaque ou une désactivation met fin au camouflage.{reservation}"
            ),
            TechniqueAction::MaintainJamming {
                activation_energy,
                radius,
                penalty,
                maximum_duration,
                ..
            } => format!(
                "{timing} · {activation_energy} E réservés. Brouille le canal choisi dans un rayon de {radius} cases avec un malus de {penalty}, pendant au plus {maximum_duration} tours.{reservation}"
            ),
            TechniqueAction::ManifestDrone {
                integrity,
                energy_capacity,
                link_range,
                sensor_radius,
                attack_damage,
                attack_range,
                ..
            } => format!(
                "{timing} · {} E dépensés · 1 place de compagnon. Invoque un drone adjacent : {integrity} PV, batterie {energy_capacity} E, liaison {link_range} cases, capteurs {sensor_radius} cases. Attaque électrique : {attack_damage} dégâts, portée {attack_range}. Maximum de groupe : {} compagnons.",
                definition.activation_cost().map_or(0, |c| c.energy()),
                self.game.rules().player_companion_limit.unwrap_or(0)
            ),
            TechniqueAction::SpoofAuthorization {
                energy_cost,
                range,
                duration_time_units,
            } => format!(
                "{timing} · {energy_cost} E réservés. Utilise un identifiant déjà extrait à portée {range} pour obtenir ses droits locaux pendant {duration_time_units} tours.{reservation}"
            ),
            TechniqueAction::DivertDevice {
                energy_cost,
                range,
                duration_time_units,
                ..
            } => format!(
                "{timing} · {energy_cost} E réservés. Maintient une commande autorisée sur un dispositif à portée {range}, pendant {duration_time_units} tours. Une reprise adverse reste possible.{reservation}"
            ),
            TechniqueAction::DivertSubnet {
                energy_cost,
                range,
                maximum_devices,
                duration_time_units,
                ..
            } => format!(
                "{timing} · {energy_cost} E réservés au total, répartis entre les dispositifs. Commande jusqu'à {maximum_devices} interfaces autorisées à portée {range}, pendant {duration_time_units} tours.{reservation}"
            ),
            TechniqueAction::LockDeviceControl {
                energy_cost,
                range,
                duration_time_units,
                ..
            } => format!(
                "{timing} · {energy_cost} E réservés. Protège pendant {duration_time_units} tours votre contrôle d'un dispositif à portée {range} contre les reprises ordinaires.{reservation}"
            ),
            TechniqueAction::MaintainBackdoor {
                installation_energy_cost,
                reconnection_energy_cost,
                maximum_backdoors,
                range,
                session_duration_time_units,
            } => format!(
                "{timing}. Réserve {installation_energy_cost} E pour installer un accès, ou {reconnection_energy_cost} E pour rouvrir un accès existant. Maximum {maximum_backdoors} accès dormants, portée {range}, session de {session_duration_time_units} tours. L'énergie est libérée à la fin de la session ; l'accès dormant reste enregistré."
            ),
            TechniqueAction::ForceElectronicLock {
                energy_cost,
                range,
                failure_hardening_duration,
                ..
            } => format!(
                "{timing} · {energy_cost} E dépensés. Tente une ouverture électronique à portée {range}. Un échec renforce temporairement la défense de l'interface pendant {failure_hardening_duration} tours."
            ),
            TechniqueAction::TriggerRemoteExplosive {
                energy_cost, range, ..
            } => format!(
                "{timing} · {energy_cost} E dépensés. Déclenche un récepteur identifié et joignable à portée {range}."
            ),
            TechniqueAction::ProgramExplosives {
                energy_cost,
                range,
                maximum_devices,
                minimum_delay,
                maximum_delay,
                ..
            } => format!(
                "{timing} · {energy_cost} E dépensés. Programme jusqu'à {maximum_devices} récepteurs connus à portée {range}, avec un délai de {minimum_delay} à {maximum_delay} tours."
            ),
            TechniqueAction::PropelledMove {
                energy_cost,
                distance,
                ..
            } => format!(
                "{timing} · {energy_cost} E dépensés. Parcourt {distance} cases successives. Chaque case conserve ses dangers et peut déclencher une Surveillance."
            ),
            TechniqueAction::DroneCoordinateFire {
                energy_cost,
                maximum_drones,
                link_range,
                ..
            } => format!(
                "{timing} · {energy_cost} E dépensés. Jusqu'à {maximum_drones} drones en liaison à portée {link_range} viseront la cible lors de leur prochaine attaque ordinaire."
            ),
            TechniqueAction::DroneConditionalRoutine {
                energy_cost,
                link_range,
                ..
            } => format!(
                "{timing} · {energy_cost} E réservés. Confie à un drone à portée {link_range} une réaction à une condition locale. L'énergie reste réservée tant que cet ordre est actif.{reservation}"
            ),
            TechniqueAction::DroneCoordinatedDeployment {
                energy_cost,
                maximum_drones,
                link_range,
                ..
            } => format!(
                "{timing} · {energy_cost} E dépensés. Assigne des positions et des rôles à {maximum_drones} drones au plus, en liaison à portée {link_range}. Les drones doivent effectuer leurs déplacements."
            ),
            TechniqueAction::DroneEmergencyReturn {
                energy_cost,
                maximum_drones,
                link_range,
                duration_phases,
                ..
            } => format!(
                "{timing} · {energy_cost} E dépensés. Ordonne à {maximum_drones} drones au plus, en liaison à portée {link_range}, de revenir pendant {duration_phases} tours, puis d'attendre."
            ),
            TechniqueAction::ElectronicPulse {
                energy_cost,
                radius,
                damage,
                disruption_intensity,
                directional,
                filter_identified_allies,
                ..
            } => format!(
                "{timing} · {energy_cost} E dépensés. Impulsion électronique de rayon {radius} : {damage} dégâts électriques et perturbation d'intensité {disruption_intensity}.{}{}",
                if directional {
                    " Émission directionnelle."
                } else {
                    ""
                },
                if filter_identified_allies {
                    " Épargne les alliés identifiés."
                } else {
                    " Les alliés compatibles dans la zone peuvent être touchés."
                }
            ),
            TechniqueAction::ElectronicCascade {
                energy_cost,
                range,
                jump_range,
                maximum_targets,
                damage_by_target,
                ..
            } => format!(
                "{timing} · {energy_cost} E dépensés. Frappe jusqu'à {maximum_targets} machines : portée initiale {range}, saut {jump_range} cases. Dégâts électriques successifs : {}.",
                damage_by_target
                    .iter()
                    .take(usize::from(maximum_targets))
                    .map(u16::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            TechniqueAction::ImplantOverheat {
                energy_cost,
                range,
                heat_per_tick,
                dissipation_penalty,
                duration_time_units,
                ..
            } => format!(
                "{timing} · {energy_cost} E dépensés. Tente d'implanter une surchauffe dans une machine visible à portée {range}. Le système de la cible reçoit {heat_per_tick} H par tour et perd {dissipation_penalty} de dissipation pendant {duration_time_units} tours. Sa chaleur peut lui infliger des dégâts thermiques."
            ),
            TechniqueAction::ImplantInfection {
                energy_cost,
                range,
                propagation_range,
                thermal_damage_per_tick,
                ticks_per_host,
                maximum_hosts,
                campaign_duration,
                ..
            } => format!(
                "{timing} · {energy_cost} E dépensés. Tente d'infecter une machine à portée {range} : {thermal_damage_per_tick} dégâts thermiques par tour pendant {ticks_per_host} tours par hôte. Propagation à {propagation_range} cases, au plus {maximum_hosts} hôtes et {campaign_duration} tours de campagne."
            ),
            TechniqueAction::ImplantImplosion {
                energy_cost,
                range,
                reserved_energy,
                minimum_stored_energy,
                delay_time_units,
                radius,
                physical_damage,
                thermal_damage,
                ..
            } => format!(
                "{timing} · {energy_cost} E dépensés. Tente de bloquer {reserved_energy} E dans une machine à portée {range} contenant au moins {minimum_stored_energy} E. Cette énergie appartient à la cible. Détonation après {delay_time_units} tours : rayon {radius}, {physical_damage} dégâts physiques et {thermal_damage} thermiques."
            ),
            _ => return None,
        };
        Some(text)
    }
    fn resource_management_actions(&self) -> Vec<(String, GameCommand)> {
        let mut actions: Vec<_> = self
            .game
            .player_maintained_energy()
            .into_iter()
            .map(|r| {
                (
                    format!(
                        "Arrêter {} · libérer {} E",
                        self.technique_name(&r.technique),
                        r.amount
                    ),
                    GameCommand::EndMaintainedEffect { effect: r.effect },
                )
            })
            .collect();
        actions.extend(
            self.game
                .actors()
                .iter()
                .filter(|(_, a)| a.companion_origin().is_some_and(|o| o.uses_slot()))
                .map(|(entity, actor)| {
                    let origin = match actor.companion_origin().unwrap() {
                        project_rl::companion::CompanionOrigin::Summoned => "invoqué",
                        project_rl::companion::CompanionOrigin::Purchased => "acheté",
                        _ => "recruté",
                    };
                    (
                        format!(
                            "Renvoyer le compagnon {} ({origin}) · {} PV",
                            entity.get(),
                            actor.integrity()
                        ),
                        GameCommand::DismissCompanion { entity },
                    )
                }),
        );
        actions
    }

    fn resource_action_rect(body: Rect, offset: f32, index: usize) -> Rect {
        Rect::new(
            body.x,
            body.y + 192.0 + index as f32 * 44.0 - offset,
            body.w - 18.0,
            36.0,
        )
    }

    pub(super) fn update_energy_reservations(&mut self, input: &InputFrame) {
        let (w, h) = input.viewport.unwrap_or((1280.0, 800.0));
        let panel = reader_panel(w, h);
        let body = reader_body(panel);
        let list = Rect::new(body.x, body.y + 192.0, body.w, (body.h - 192.0).max(1.0));
        let clicked = input.pressed.contains(&controls::Binding::MouseLeft);
        if clicked
            && input
                .pointer
                .is_some_and(|p| reader_close(panel).contains(p.into()))
        {
            self.ux.resources_open = false;
            return;
        }
        let actions = self.resource_management_actions();
        self.ux.resources_selection = self
            .ux
            .resources_selection
            .min(actions.len().saturating_sub(1));
        let down = self.controls.pressed(Action::MenuDown, input);
        let up = self.controls.pressed(Action::MenuUp, input);
        if down {
            self.ux.resources_selection =
                (self.ux.resources_selection + 1).min(actions.len().saturating_sub(1));
        }
        if up {
            self.ux.resources_selection = self.ux.resources_selection.saturating_sub(1);
        }
        self.ux.resources_scroll.update(input, &self.controls);
        if (down || up) && !actions.is_empty() {
            let selected = Self::resource_action_rect(
                body,
                self.ux.resources_scroll.offset,
                self.ux.resources_selection,
            );
            if selected.bottom() > list.bottom() {
                self.ux.resources_scroll.offset += selected.bottom() - list.bottom();
            }
            if selected.y < list.y {
                self.ux.resources_scroll.offset =
                    (self.ux.resources_scroll.offset - (list.y - selected.y)).max(0.0);
            }
        }
        let mut chosen = self
            .controls
            .pressed(Action::Learn, input)
            .then_some(self.ux.resources_selection);
        for (index, _) in actions.iter().enumerate() {
            let rect = Self::resource_action_rect(body, self.ux.resources_scroll.offset, index);
            if rect.y >= list.y
                && rect.bottom() <= list.bottom()
                && input.pointer.is_some_and(|p| rect.contains(p.into()))
            {
                self.ux.resources_selection = index;
                if clicked {
                    chosen = Some(index);
                }
            }
        }
        if let Some((_, command)) = chosen.and_then(|i| actions.get(i)) {
            self.execute_command(command.clone());
            self.capture_events();
            self.ux.resources_selection = self
                .ux
                .resources_selection
                .min(self.resource_management_actions().len().saturating_sub(1));
        }
    }

    pub(super) fn draw_energy_reservations(&self) {
        let panel = reader_panel(self.ui_width(), self.ui_height());
        draw_rectangle(
            0.0,
            0.0,
            self.ui_width(),
            self.ui_height(),
            UiTheme.backdrop(),
        );
        draw_reader(
            "Énergie et compagnons",
            &format!(
                "Consultation gratuite · {} : confirmer la ligne choisie (1 tour)",
                self.controls.label(Action::Learn)
            ),
            panel,
            false,
            &self.controls,
        );
        let body = reader_body(panel);
        let energy = self.game.player_energy();
        let offset = self.ux.resources_scroll.offset;
        let y = body.y;
        if let Some(gauge) = self
            .resource_gauges()
            .into_iter()
            .find(|g| g.label == "Énergie")
        {
            Self::draw_resource_gauge(Rect::new(body.x, y + 4.0, body.w - 18.0, 48.0), &gauge);
        }
        let blue = Color::from_rgba(109, 186, 255, 255);
        let purple = Color::from_rgba(179, 132, 235, 255);
        draw_text(
            &format!(
                "{} E libres sur {} utilisables",
                energy.available(),
                energy.usable_capacity()
            ),
            body.x,
            y + 79.0,
            17.0,
            blue,
        );
        draw_text(
            &format!(
                "{} E à régénérer",
                energy.usable_capacity().saturating_sub(energy.available())
            ),
            body.x,
            y + 103.0,
            17.0,
            UiTheme.muted(),
        );
        draw_text(
            &format!(
                "{} E réservés · capacité totale {} E",
                energy.reserved(),
                energy.capacity()
            ),
            body.x,
            y + 127.0,
            17.0,
            purple,
        );
        draw_text(
            &format!(
                "Compagnons : {} / {} · les PNJ de quête ne comptent pas",
                self.game.player_companion_count(),
                self.game.rules().player_companion_limit.unwrap_or(0)
            ),
            body.x,
            y + 159.0,
            17.0,
            UiTheme.text(),
        );
        let actions = self.resource_management_actions();
        for (index, (label, _)) in actions.iter().enumerate() {
            let rect = Self::resource_action_rect(body, offset, index);
            if rect.y >= body.y + 192.0 && rect.bottom() <= body.bottom() {
                UiTheme.button(
                    rect,
                    label,
                    index == self.ux.resources_selection,
                    false,
                    true,
                    ButtonTone::Secondary,
                );
            }
        }
        if actions.is_empty() {
            draw_text(
                "Aucun effet maintenu ni compagnon à gérer.",
                body.x,
                y + 212.0,
                16.0,
                UiTheme.muted(),
            );
        }
        let list = Rect::new(body.x, body.y + 192.0, body.w, (body.h - 192.0).max(1.0));
        self.ux
            .resources_scroll
            .finish(list, list.y + actions.len() as f32 * 44.0);
    }
}
