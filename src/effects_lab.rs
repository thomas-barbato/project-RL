//! Disposable equipment/animation fixture. Never registered in campaign content
//! or serialized as a normal run; the main-menu app is retained unchanged.
use super::*;
use project_rl::combat::DamagePacket;
use project_rl::effects::{DamageFalloff, GroundEffectSpec, RadialDamageEffect};
use project_rl::entity::MagicItemModifiers;
use project_rl::item::{
    EquipmentAffixId, EquipmentNameGrammar, NamedEquipmentAffixes, RolledEquipmentAffix,
};
use project_rl::presentation::VisualCueDefinition;
use project_rl::weapon::{
    WeaponDefinition, WeaponEffect, WeaponEffectAffixDefinition, WeaponEffectOrigin,
    WeaponEffectTrigger,
};
use project_rl::world::{FieldOfViewRules, Map, NeighborMode, TerrainPropagationPolicy};

const LAB_SEED: u64 = 0x1ab;
const LAB_AFFIX_TIER: u8 = 1;

fn roll_lab_bonuses(
    rng: &mut project_rl::game::GameRng,
    grammar: EquipmentNameGrammar,
) -> MagicItemModifiers {
    let mut pool = EquipmentAffixId::ALL;
    let count = rng.usize_inclusive(1, 3).unwrap();
    let mut rolls = Vec::new();
    for index in 0..count {
        let selected = rng.usize_inclusive(index, pool.len() - 1).unwrap();
        pool.swap(index, selected);
        let (minimum, maximum) = pool[index].range(LAB_AFFIX_TIER).unwrap();
        let value = rng
            .usize_inclusive(usize::from(minimum), usize::from(maximum))
            .unwrap() as u16;
        rolls.push(RolledEquipmentAffix::new(pool[index], LAB_AFFIX_TIER, value).unwrap());
    }
    MagicItemModifiers::from_affixes(NamedEquipmentAffixes::new(grammar, &rolls).unwrap())
}
const LAB_START: GridPos = GridPos::new(7, 12);
const LAB_PROBE: GridPos = GridPos::new(7, 20);
const LAB_KINDS: [&str; 14] = [
    "reference",
    "impact",
    "bearer",
    "conduction",
    "burning",
    "caustic",
    "healing",
    "piercing",
    "aegis",
    "percussion",
    "catalysis",
    "fracture",
    "ricochet",
    "echo",
];

fn lab_id(value: &str) -> ContentId {
    format!("lab:{value}")
        .parse()
        .expect("authored laboratory identifier")
}

fn lab_effect(kind: &str) -> Result<Vec<WeaponEffect>, String> {
    let hit = WeaponEffectTrigger::OnHit;
    let effect = match kind {
        "ricochet" => WeaponEffect::ricochet(4, DamagePacket::new(3, DamageType::Piercing, 0))
            .map_err(|e| e.to_string())?,
        "echo" => WeaponEffect::delayed_echo(2, DamagePacket::new(2, DamageType::Kinetic, 0))
            .map_err(|e| e.to_string())?,
        "fracture" => WeaponEffect::accumulated_fracture(
            lab_id("fracture_mark"),
            3,
            RadialDamageEffect {
                maximum_cost: 1,
                neighbor_mode: NeighborMode::CardinalAndDiagonal,
                propagation_policy: TerrainPropagationPolicy::blocked_by_walls(1),
                damage: DamagePacket::new(6, DamageType::Kinetic, 0),
                falloff: DamageFalloff::None,
            },
            false,
        )
        .map_err(|e| e.to_string())?,
        "reference" => return Ok(vec![]),
        "catalysis" => WeaponEffect::catalytic_cone(
            7,
            DamagePacket::new(2, DamageType::Thermal, 0),
            "core:burning".parse().unwrap(),
            RadialDamageEffect {
                maximum_cost: 1,
                neighbor_mode: NeighborMode::CardinalAndDiagonal,
                propagation_policy: TerrainPropagationPolicy::blocked_by_walls(1),
                damage: DamagePacket::new(6, DamageType::Thermal, 0),
                falloff: DamageFalloff::None,
            },
        )
        .map_err(|e| e.to_string())?,
        "percussion" => {
            WeaponEffect::percussion(4, lab_id("percussion_recovery")).map_err(|e| e.to_string())?
        }
        "aegis" => WeaponEffect::apply_bearer_status(
            ApplyStatusEffect::new(lab_id("impact_aegis"), 1).map_err(|e| e.to_string())?,
            WeaponEffectTrigger::OnDamage,
        )
        .map_err(|e| e.to_string())?,
        "piercing" => {
            WeaponEffect::piercing_line(3, DamagePacket::new(3, DamageType::Piercing, 0), hit)
                .map_err(|e| e.to_string())?
        }
        "healing" => WeaponEffect::life_steal(
            50,
            3,
            lab_id("healing_target"),
            WeaponEffectTrigger::OnDamage,
        )
        .map_err(|e| e.to_string())?,
        "burning" => WeaponEffect::apply_status(
            ApplyStatusEffect::new("core:burning".parse().unwrap(), 1)
                .map_err(|e| e.to_string())?,
            hit,
        )
        .map_err(|e| e.to_string())?,
        "caustic" => WeaponEffect::create_ground_effect(
            GroundEffectSpec::new(
                lab_id("caustic_ground"),
                3,
                DamagePacket::new(2, DamageType::Chemical, 0),
            )
            .map_err(|e| e.to_string())?,
            hit,
        ),
        "impact" | "bearer" | "conduction" => WeaponEffect::radial_damage(
            RadialDamageEffect {
                maximum_cost: if kind == "conduction" { 4 } else { 2 },
                neighbor_mode: NeighborMode::CardinalAndDiagonal,
                propagation_policy: if kind == "conduction" {
                    TerrainPropagationPolicy::conductive(3, 1)
                } else {
                    TerrainPropagationPolicy::blocked_by_walls(1)
                },
                damage: DamagePacket::new(4, DamageType::Electrical, 0),
                falloff: DamageFalloff::None,
            },
            hit,
            if kind == "bearer" {
                WeaponEffectOrigin::Bearer
            } else {
                WeaponEffectOrigin::Impact
            },
            false,
        )
        .map_err(|e| e.to_string())?,
        _ => return Err(format!("Unknown laboratory effect '{kind}'")),
    };
    Ok(vec![effect])
}

impl AsciiApp {
    /// Real persistent fields on empty cells, using ordinary attack commands.
    /// Diagnostic-only loadout; it never adds weapons to the playable lab.
    #[cfg(any(test, debug_assertions))]
    fn prepare_lab_ground_fixture(&mut self, element: &str) -> Result<(), String> {
        let mut rules = self.rules.clone();
        let base = rules
            .weapons
            .get(&"core:flamethrower".parse().unwrap())
            .ok_or("Lance-flammes de diagnostic absent")?;
        let mut effects = Vec::new();
        for (kind, id, damage) in [
            ("fire", "core:burning_ground", DamageType::Thermal),
            ("acid", "core:caustic_ground", DamageType::Chemical),
            (
                "electric",
                "core:electrified_ground",
                DamageType::Electrical,
            ),
        ] {
            if element == kind || element == "mixed" {
                effects.push(WeaponEffect::create_ground_effect(
                    GroundEffectSpec::new(id.parse().unwrap(), 3, DamagePacket::new(1, damage, 0))
                        .map_err(|e| e.to_string())?,
                    WeaponEffectTrigger::OnAttack,
                ));
            }
        }
        if effects.is_empty() {
            return Err("Élément de diagnostic inconnu".into());
        }
        let id = lab_id("ground_probe");
        let weapon = WeaponDefinition::new(
            id.clone(),
            "diagnostic.name".into(),
            "diagnostic.description".into(),
            base.attack(),
        )
        .map_err(|e| e.to_string())?
        .with_effects(effects);
        rules.weapons.register(weapon).map_err(|e| e.to_string())?;
        rules.player_starting_weapons = vec![id.clone()];
        rules.player_starting_equipment = vec![Some(id)];
        let game = GameState::new_with_rules(self.game.map().clone(), LAB_START, LAB_SEED, rules)
            .map_err(|e| e.to_string())?;
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.selected_target = None;
        self.game.drain_events();
        self.visual_cues.clear_world();
        self.floating_messages.clear();
        if self.execute_command(GameCommand::AttackAt {
            slot: 0,
            target: GridPos::new(13, 12),
        }) != CommandOutcome::Applied
        {
            return Err("Création des champs de diagnostic refusée".into());
        }
        Ok(())
    }

    #[cfg(any(test, debug_assertions))]
    fn prime_lab_catalysis(&mut self, target: EntityId) -> Result<(), String> {
        let brand = self
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &lab_id("rifle_burning"))
            .ok_or("Brandon de diagnostic absent")?
            .instance();
        self.execute_command(GameCommand::EquipWeapon {
            slot: 1,
            item: brand,
        });
        if self.execute_command(GameCommand::Attack { slot: 1, target }) != CommandOutcome::Applied
        {
            return Err("Préparation de la brûlure refusée".into());
        }
        self.game.drain_events();
        self.visual_cues.clear_world();
        self.floating_messages.clear();
        Ok(())
    }

    #[cfg(any(test, debug_assertions))]
    pub(super) fn prepare_lab_status_fixture(&mut self) -> Result<(), String> {
        let mut rules = self.rules.clone();
        rules.player_base_abilities = vec![AbilityProfile::new(
            7,
            DistanceMetric::Chebyshev,
            true,
            true,
            ["core:burning", "core:corroded", "core:locomotion_hindered"]
                .into_iter()
                .map(|id| {
                    EffectPrimitive::ApplyStatus(
                        ApplyStatusEffect::new(id.parse().unwrap(), 1).unwrap(),
                    )
                })
                .collect(),
        )];
        let mut game =
            GameState::new_with_rules(self.game.map().clone(), LAB_START, LAB_SEED, rules)
                .map_err(|e| e.to_string())?;
        let target = game
            .spawn_actor(
                Actor::new(GridPos::new(8, 12), 120)
                    .unwrap()
                    .with_evasion_disabled(),
            )
            .map_err(|e| e.to_string())?;
        self.game = WorldState::single(game);
        self.actor_glyphs.clear();
        self.actor_glyphs.insert(target, 'X');
        self.selected_target = Some(target);
        self.game.drain_events();
        if self.execute_command(GameCommand::UseAbility {
            slot: 0,
            target: GridPos::new(8, 12),
        }) != CommandOutcome::Applied
        {
            return Err("Application des états de diagnostic refusée".into());
        }
        Ok(())
    }

    /// Native camera/input regression: repeated clicks must remain selection
    /// even when the player has assigned their attack action to MouseLeft.
    #[cfg(debug_assertions)]
    pub(super) fn verify_lab_pointer_selection(
        &mut self,
        leave_selected: bool,
    ) -> Result<(), String> {
        use crate::controls::Binding;
        let controls = self.controls.clone();
        self.controls.rebind(Action::Attack, Binding::MouseLeft)?;
        self.selected_target = None;
        let before = suspension::fingerprint(&self.game);
        let history = self.history.len();
        let first = GridPos::new(8, 12);
        let second = GridPos::new(10, 12);
        let sequence = [
            (first, true),
            (first, false),
            (second, true),
            (first, true),
            (first, false),
        ];
        for (at, selected) in sequence
            .into_iter()
            .chain(leave_selected.then_some((first, true)))
        {
            let cell = self
                .terminal
                .world_cell_rect(
                    &self.game,
                    self.terminal_bounds(),
                    self.ui_scale(),
                    self.graphics.active.world_cell_px,
                    self.navigation_signal_summary().is_some(),
                    at,
                )
                .ok_or("Cible de diagnostic hors caméra")?;
            let mut frame = self.graphics.active.transform_input(InputFrame {
                pressed: [Binding::MouseLeft].into(),
                pointer: Some((cell.x + cell.w * 0.5, cell.y + cell.h * 0.5)),
                viewport: Some((screen_width(), screen_height())),
                ..Default::default()
            });
            self.update_input_at(&frame, None);
            frame.pressed.clear();
            self.update_input_at(&frame, None);
            let expected = if selected {
                self.game.actors().entity_at(at)
            } else {
                None
            };
            if self.selected_target != expected
                || self.attack_aim.is_some()
                || suspension::fingerprint(&self.game) != before
                || self.history.len() != history
            {
                return Err(
                    "Le clic de sélection a modifié la simulation ou choisi la mauvaise cible"
                        .into(),
                );
            }
        }
        self.controls = controls;
        Ok(())
    }

    #[cfg(debug_assertions)]
    pub(super) fn prepare_lab_effect_visual(&mut self, scene: &str) -> Result<(), String> {
        self.open_menu(MenuScreen::Main);
        let lab = self.build_test_lab_with_bonus_seed(LAB_SEED)?;
        let previous = std::mem::replace(self, lab);
        self.lab_return = Some(Box::new(previous));
        if scene == "inventory-held" {
            self.inventory_filter = InventoryFilter::All;
            self.inventory_selection = 0;
            self.inventory_open = true;
            let before = (suspension::fingerprint(&self.game), self.history.len());
            let binding = self.controls.binding(Action::MenuDown).clone();
            let mut input = InputFrame {
                pressed: [binding.clone()].into(),
                held: [binding].into(),
                ..Default::default()
            };
            self.update_input_at(&input, Some(10.0));
            input.pressed.clear();
            for index in 0..11 {
                self.update_input_at(&input, Some(10.36 + index as f64 * 0.1));
            }
            self.update_input_at(&InputFrame::default(), Some(12.0));
            self.update_input_at(&input, Some(13.0));
            if self.inventory_selection != 12
                || (suspension::fingerprint(&self.game), self.history.len()) != before
            {
                return Err(
                    "Le maintien de navigation ne conserve pas la sélection ou a modifié la partie"
                        .into(),
                );
            }
            return Ok(());
        }
        if scene == "inventory-dense" {
            let item = self
                .game
                .player_inventory()
                .iter()
                .filter(|entry| {
                    entry.magic_modifiers().is_some_and(|bonus| {
                        bonus
                            .named_affixes()
                            .is_some_and(|affixes| affixes.iter().count() == 3)
                    })
                })
                .filter_map(|entry| {
                    self.game
                        .player_item_weapon(entry.instance())
                        .filter(|weapon| !weapon.effects().is_empty())
                        .map(|weapon| (entry.instance(), weapon))
                })
                .max_by_key(|(_, weapon)| {
                    self.texts
                        .resolve(DISPLAY_LOCALE, weapon.description_key())
                        .unwrap_or("")
                        .len()
                        + self.weapon_effect_limits(weapon).len()
                })
                .map(|(item, _)| item)
                .ok_or("Arme de diagnostic avec trois bonus et effet absente")?;
            self.execute_command(GameCommand::EquipWeapon { slot: 0, item });
            self.inventory_filter = InventoryFilter::All;
            self.select_inventory_instance(item);
            self.update_input(&InputFrame {
                pressed: [self.controls.binding(Action::Inventory).clone()].into(),
                ..Default::default()
            });
            return Ok(());
        }
        if scene == "inventory-bag" {
            self.inventory_filter = InventoryFilter::All;
            self.inventory_selection = self.inventory_entries().len().saturating_sub(1);
            self.inventory_open = true;
            return Ok(());
        }
        if scene.starts_with("bonuses-") {
            let armor = scene == "bonuses-armor-inventory";
            let item = self
                .game
                .player_inventory()
                .iter()
                .find(|entry| {
                    entry.magic_modifiers().is_some_and(|bonus| {
                        if armor {
                            entry.item() == &lab_id("armor_reference")
                        } else if scene == "bonuses-three-inventory" {
                            bonus
                                .named_affixes()
                                .is_some_and(|affixes| affixes.iter().count() == 3)
                                && entry.item() != &lab_id("armor_reference")
                        } else {
                            bonus.maximum_hit_points_bonus() > 0
                                && entry.item() != &lab_id("armor_reference")
                        }
                    })
                })
                .ok_or("Variante avec bonus absente")?
                .instance();
            if armor {
                self.execute_command(GameCommand::EquipItem {
                    slot: lab_id("body_armor"),
                    item,
                });
                self.inventory_filter = InventoryFilter::Armor;
            } else {
                self.execute_command(GameCommand::EquipWeapon { slot: 0, item });
            }
            self.inventory_selection = self
                .inventory_entries()
                .iter()
                .position(|entry| entry.instance() == item)
                .ok_or("Variante filtrée")?;
            self.inventory_open = true;
            return Ok(());
        }
        if scene.starts_with("echo-empty") {
            let mut rules = self.game.rules().clone();
            rules.player_starting_equipment[0] = Some(lab_id("blade_echo"));
            let mut game =
                GameState::new_with_rules(self.game.map().clone(), LAB_START, LAB_SEED, rules)
                    .map_err(|e| e.to_string())?
                    .with_starting_weapon_effects([(
                        lab_id("blade_echo"),
                        lab_id("effect_echo"),
                    )])?;
            let target = game
                .spawn_actor(
                    Actor::new(GridPos::new(8, 12), 1)
                        .unwrap()
                        .with_evasion_disabled(),
                )
                .map_err(|e| e.to_string())?;
            self.game = WorldState::single(game);
            self.actor_glyphs.clear();
            self.actor_glyphs.insert(target, 'X');
            self.selected_target = Some(target);
            self.game.drain_events();
            self.visual_cues.clear_world();
            self.floating_messages.clear();
            self.execute_command(GameCommand::Attack { slot: 0, target });
            if self.game.actors().get(target).is_some()
                || self.game.pending_weapon_echoes().count() != 1
            {
                return Err("La marque d'écho n'a pas survécu à la cible".into());
            }
            self.graphics.active.reduced_motion = scene.ends_with("-reduced");
            self.capture_events_at(Some(get_time() - 5.0));
            return Ok(());
        }
        if let Some(scene) = scene.strip_prefix("ground-") {
            let (element, phase) = scene.split_once('-').unwrap_or((scene, "live"));
            self.prepare_lab_ground_fixture(element)?;
            self.graphics.active.reduced_motion = phase == "reduced";
            let waits = match phase {
                "last" => 2,
                "expired" => 3,
                _ => 0,
            };
            for _ in 0..waits {
                if self.execute_command(GameCommand::Wait) != CommandOutcome::Applied {
                    return Err("Tour de diagnostic refusé".into());
                }
            }
            self.capture_events_at(Some(get_time() - 30.0));
            self.floating_messages.clear();
            let at = GridPos::new(10, 12);
            let before = suspension::fingerprint(&self.game);
            let samples =
                self.sustained_effects_at(at, get_time(), self.graphics.active.reduced_motion);
            if samples.is_empty() != (phase == "expired")
                || self
                    .visual_cues
                    .sample_world(at, true, get_time())
                    .is_some()
                || self.glyph_at(at).is_some()
                || suspension::fingerprint(&self.game) != before
            {
                return Err("Durée des animations persistantes incorrecte".into());
            }
            self.push_log(format!(
                "CHAMPS AU SOL · {element} · {} tour(s) restant(s) · aucun flash transitoire.",
                3 - waits
            ));
            return Ok(());
        }
        let (kind, elapsed) = scene
            .split_once('-')
            .map_or((scene, 0.14), |(kind, phase)| {
                (
                    kind,
                    if phase == "embers" {
                        0.60
                    } else if phase == "late" || phase == "oblique" || phase == "burst" {
                        0.32
                    } else {
                        0.04
                    },
                )
            });
        self.graphics.active.reduced_motion = scene.ends_with("-reduced");
        if kind == "statuses" {
            self.prepare_lab_status_fixture()?;
            self.capture_events_at(Some(get_time() - 5.0));
            return Ok(());
        }
        if kind == "frost" {
            self.walk_fixture_to(GridPos::new(13, 8))?;
            self.game.drain_events();
            let before = suspension::fingerprint(&self.game);
            let center = GridPos::new(14, 8);
            let cells = (-1_i32..=1).flat_map(|dy| {
                (-1_i32..=1).map(move |dx| {
                    VisualCueCell::new(
                        GridPos::new(center.x + dx, center.y + dy),
                        dx.abs().max(dy.abs()) as u16,
                    )
                })
            });
            self.game.drain_events();
            self.visual_cues.clear_world();
            self.floating_messages.clear();
            self.visual_cues.play(
                VisualCue::world(visual_cue_id("core:frost_preview"), center, cells)
                    .map_err(|e| e.to_string())?,
                get_time() - elapsed,
            );
            self.push_log("GLACE · aperçu visuel uniquement, aucun dégât ni gel appliqué.".into());
            if suspension::fingerprint(&self.game) != before {
                return Err("L'aperçu de glace a modifié la simulation".into());
            }
            return Ok(());
        }
        let prefix = if kind == "bearer"
            || kind == "ricochet"
            || kind == "conduction"
            || scene == "fracture-ranged"
            || scene.ends_with("-oblique")
            || scene == "aegis-block"
        {
            "rifle"
        } else {
            "blade"
        };
        let id = lab_id(&format!("{prefix}_{kind}"));
        let item = self
            .game
            .player_inventory()
            .iter()
            .find(|entry| {
                entry.item() == &id
                    && (!scene.contains("-bonus")
                        || entry
                            .magic_modifiers()
                            .and_then(|bonus| bonus.named_affixes())
                            .is_some())
            })
            .ok_or("Arme de diagnostic inconnue")?
            .instance();
        if self.game.equipped_player_weapon_item(0) != Some(item) {
            self.execute_command(GameCommand::EquipWeapon { slot: 0, item });
        }
        if scene.ends_with("-inventory") {
            self.inventory_selection = self
                .inventory_entries()
                .iter()
                .position(|entry| entry.instance() == item)
                .ok_or("Variante absente")?;
            self.inventory_open = true;
            return Ok(());
        }
        if scene == "percussion-ready" {
            return Ok(());
        }
        let at = if kind == "catalysis" || kind == "fracture" {
            self.walk_fixture_to(GridPos::new(
                if scene == "fracture-ranged" { 11 } else { 13 },
                8,
            ))?;
            GridPos::new(14, 8)
        } else if scene == "percussion-blocked" {
            self.walk_fixture_to(GridPos::new(16, 17))?;
            GridPos::new(17, 17)
        } else if scene == "percussion-heavy" {
            self.walk_fixture_to(GridPos::new(11, 18))?;
            GridPos::new(12, 18)
        } else if scene == "percussion-fixed" {
            // Approach from the east, outside the damage probe's range.
            self.walk_fixture_to(GridPos::new(11, 18))?;
            self.walk_fixture_to(GridPos::new(11, 21))?;
            GridPos::new(12, 21)
        } else if scene == "percussion-oblique" {
            self.walk_fixture_to(GridPos::new(3, 3))?;
            GridPos::new(8, 5)
        } else if scene == "aegis-block" {
            self.walk_fixture_to(GridPos::new(7, 18))?;
            LAB_PROBE
        } else if scene == "piercing-oblique" {
            self.walk_fixture_to(GridPos::new(11, 7))?;
            GridPos::new(14, 8)
        } else if kind == "conduction" {
            self.walk_fixture_to(GridPos::new(20, 8))?;
            GridPos::new(22, 8)
        } else if kind == "bearer" {
            GridPos::new(10, 12)
        } else {
            GridPos::new(8, 12)
        };
        self.selected_target = self.game.actors().entity_at(at);
        if scene.starts_with("catalysis-preview") {
            if scene.ends_with("-burning") {
                self.prime_lab_catalysis(self.selected_target.ok_or("Cible absente")?)?;
            }
            let before = suspension::fingerprint(&self.game);
            if self.weapon_command(None).is_some() || self.attack_aim.is_none() {
                return Err("L'attaque n'a pas ouvert l'aperçu".into());
            }
            if scene.ends_with("-empty") {
                self.attack_aim.as_mut().unwrap().cursor = GridPos::new(14, 7);
            }
            if self
                .aimed_attack_footprint(self.attack_aim.unwrap())
                .map_err(|e| format!("{e:?}"))?
                .cells()
                .len()
                < 2
                || suspension::fingerprint(&self.game) != before
            {
                return Err("Aperçu du cône absent ou non neutre".into());
            }
            return Ok(());
        }
        if scene == "catalysis-multi" {
            self.execute_command(GameCommand::Attack {
                slot: 0,
                target: self.selected_target.ok_or("Cible absente")?,
            });
        } else if kind == "catalysis" && scene != "catalysis-empty" {
            self.prime_lab_catalysis(self.selected_target.ok_or("Cible absente")?)?;
            if scene == "catalysis-ready" {
                return Ok(());
            }
        }
        self.game.drain_events();
        self.visual_cues.clear_world();
        self.floating_messages.clear();
        self.trace_cells.clear();
        if self.execute_command(GameCommand::Attack {
            slot: 0,
            target: self.selected_target.ok_or("Cible absente")?,
        }) != CommandOutcome::Applied
        {
            return Err("Attaque de diagnostic refusée".into());
        }
        if scene == "aegis-block" {
            self.game.drain_events();
            if self.execute_command(GameCommand::Move(Direction::South)) != CommandOutcome::Applied
            {
                return Err("Approche du dispositif refusée".into());
            }
        }
        if kind == "fracture" {
            let extra_hits = if scene == "fracture-one" {
                0
            } else if scene == "fracture-two" || scene == "fracture-expired" {
                1
            } else {
                2
            };
            for _ in 0..extra_hits {
                self.game.drain_events();
                self.visual_cues.clear_world();
                self.floating_messages.clear();
                if self.execute_command(GameCommand::Attack {
                    slot: 0,
                    target: self.selected_target.ok_or("Cible absente")?,
                }) != CommandOutcome::Applied
                {
                    return Err("Charge de Fracture refusée".into());
                }
            }
            if scene == "fracture-expired" {
                for _ in 0..4 {
                    self.execute_command(GameCommand::Wait);
                }
            }
        }
        if kind == "echo" {
            let waits = if scene == "echo-last" {
                1
            } else if scene == "echo-burst" || scene == "echo-expired" {
                2
            } else {
                0
            };
            for _ in 0..waits {
                self.game.drain_events();
                self.visual_cues.clear_world();
                self.floating_messages.clear();
                self.execute_command(GameCommand::Wait);
            }
        }
        self.capture_events_at(Some(
            get_time()
                - if scene == "aegis-status"
                    || matches!(scene, "fracture-one" | "fracture-two" | "fracture-expired")
                    || matches!(scene, "echo-mark" | "echo-last" | "echo-expired")
                {
                    5.0
                } else {
                    elapsed
                },
        ));
        Ok(())
    }

    fn build_test_lab(&self) -> Result<Self, String> {
        let bonus_seed = if self.test_lab {
            self.lab_bonus_seed.wrapping_add(1)
        } else {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(LAB_SEED, |d| d.as_nanos() as u64)
        };
        self.build_test_lab_with_bonus_seed(bonus_seed)
    }

    fn build_test_lab_with_bonus_seed(&self, bonus_seed: u64) -> Result<Self, String> {
        let mut bonus_rng = project_rl::game::GameRng::from_seed(bonus_seed);
        let base = self.lab_return.as_deref().unwrap_or(self);
        let mut rules = base.rules.clone();
        // The disposable lab always exercises the latest contract, even when
        // the menu retains a historical campaign for deterministic resume.
        rules.statuses = rules.statuses.with_compatibility_stacking(
            &"core:corroded".parse().unwrap(),
            project_rl::status::StatusStacking::RefreshDuration,
        );
        rules
            .statuses
            .register(
                project_rl::status::StatusDefinition::new(
                    lab_id("fracture_mark"),
                    Some(5),
                    project_rl::status::StatusStacking::AddStacks {
                        maximum_stacks: 3,
                        refresh_duration: true,
                    },
                    vec![],
                )
                .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        rules
            .statuses
            .register(
                project_rl::status::StatusDefinition::new(
                    lab_id("impact_aegis"),
                    Some(3),
                    project_rl::status::StatusStacking::KeepExisting,
                    vec![],
                )
                .map_err(|e| e.to_string())?
                .with_modifiers([project_rl::status::StatusModifier::DamageGuard { amount: 3 }])
                .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        rules
            .statuses
            .register(
                project_rl::status::StatusDefinition::new(
                    lab_id("percussion_recovery"),
                    Some(3),
                    project_rl::status::StatusStacking::KeepExisting,
                    vec![],
                )
                .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        let mut texts = base.texts.clone();
        let mut visuals = ascii_visual_cue_catalog()?;
        rules.player_starting_weapons.clear();
        rules.player_starting_items.clear();
        rules.player_inventory_capacity = 64;
        rules.player_field_of_view = FieldOfViewRules {
            radius: 12,
            distance_metric: DistanceMetric::Chebyshev,
            block_closed_corners: true,
        };
        // Ordinary mannequins cannot evade; E explicitly exercises Accuracy.
        rules.hit_rules = Some(rules.hit_rules.unwrap_or_default());
        rules.player_system_resources = Some(rules.player_system_resources.unwrap_or_default());
        let mut magic = Vec::new();
        let mut instance_effects = Vec::new();
        for (prefix, base_id, label) in [
            ("blade", "core:integrity_blade", "Lame"),
            ("rifle", "core:needle_launcher", "Fusil"),
        ] {
            let base_weapon = rules
                .weapons
                .get(&base_id.parse().unwrap())
                .ok_or("Arme témoin absente")?
                .clone();
            for kind in LAB_KINDS {
                if prefix == "blade" && kind == "ricochet" {
                    continue;
                }
                let (name, description) = match kind {
                    "ricochet" => (
                        "ricochet",
                        "Le projectile rebondit vers une autre cible proche après l'impact.",
                    ),
                    "echo" => (
                        "écho différé",
                        "Frappe à nouveau la case touchée après 2 tours.",
                    ),
                    "fracture" => (
                        "marquage",
                        "Attaque qui inflige des dégâts et ajoute une marque sur la cible. À 3 marques, elle provoque une explosion cinétique autour de la cible.",
                    ),
                    "catalysis" => (
                        "catalyse",
                        "Cône de flammes qui brûle les cibles sur son passage et provoque une explosion sur les cibles déjà enflammées.",
                    ),
                    "reference" => ("témoin", "Aucun effet spécial. Profil de base inchangé."),
                    "percussion" => ("percussion", "Repousse la cible de l'attaque."),
                    "aegis" => (
                        "égide d'impact",
                        "Inflige des dégâts et génère un bouclier qui absorbe jusqu'à 3 dégâts du prochain coup reçu.",
                    ),
                    "impact" => (
                        "onde d'impact",
                        "Zone électrique autour de la cible touchée, en mêlée comme à distance.",
                    ),
                    "bearer" => (
                        "décharge circulaire",
                        "Crée une zone électrique autour du lanceur.",
                    ),
                    "conduction" => (
                        "conduction",
                        "Décharge électrique, plus efficace dans l'eau.",
                    ),
                    "burning" => (
                        "brandon",
                        "Une touche applique une brûlure temporaire à la cible.",
                    ),
                    "caustic" => (
                        "suintement caustique",
                        "Laisse temporairement une flaque d'acide sur la case d'impact.",
                    ),
                    "healing" => (
                        "vol de vie",
                        "Vole 50 % des dégâts infligés par l'attaque pour vous soigner.",
                    ),
                    "piercing" => (
                        "perforation",
                        "Attaque qui transperce la cible jusqu'à 3 cases derrière elle.",
                    ),
                    _ => unreachable!(),
                };
                let id = lab_id(&format!("{prefix}_{kind}"));
                let name_key = format!("{id}.name");
                let description_key = format!("{id}.description");
                texts
                    .register("fr".into(), name_key.clone(), format!("{label} · {name}"))
                    .map_err(|e| e.to_string())?;
                texts
                    .register("fr".into(), description_key.clone(), description.into())
                    .map_err(|e| e.to_string())?;
                if kind != "reference" {
                    let effect_id = lab_id(&format!("effect_{kind}"));
                    if rules.weapons.effect_affix(&effect_id).is_none() {
                        let suffix_key = format!("{effect_id}.suffix");
                        texts
                            .register(
                                "fr".into(),
                                suffix_key.clone(),
                                equipment_affix_names::lab_effect_suffix(kind)
                                    .unwrap_or("d'écho")
                                    .into(),
                            )
                            .map_err(|e| e.to_string())?;
                        rules
                            .weapons
                            .register_effect_affix(WeaponEffectAffixDefinition::new(
                                effect_id.clone(),
                                suffix_key,
                                description_key.clone(),
                                lab_effect(kind)?,
                            )?)
                            .map_err(|e| e.to_string())?;
                    }
                    instance_effects.push((id.clone(), effect_id));
                }
                let definition = WeaponDefinition::new(
                    id.clone(),
                    name_key,
                    description_key,
                    base_weapon.attack(),
                )
                .map_err(|e| e.to_string())?
                .with_capabilities(base_weapon.capabilities())
                .with_mass_grams(base_weapon.mass_grams().unwrap_or(2_000))
                .map_err(|e| e.to_string())?;
                rules
                    .weapons
                    .register(definition)
                    .map_err(|e| e.to_string())?;
                if let Some(style) = visuals
                    .get(base_weapon.id())
                    .map(|cue| cue.terminal().clone())
                {
                    visuals
                        .register(VisualCueDefinition::new(id.clone(), style))
                        .map_err(|e| e.to_string())?;
                }
                rules.player_starting_weapons.push(id.clone());
                magic.push((
                    id,
                    roll_lab_bonuses(
                        &mut bonus_rng,
                        if prefix == "blade" {
                            EquipmentNameGrammar::FeminineSingular
                        } else {
                            EquipmentNameGrammar::MasculineSingular
                        },
                    ),
                ));
            }
        }
        // One white/magical armor pair exercises the same instance bonuses.
        let armor = lab_id("armor_reference");
        let armor_slot = lab_id("body_armor");
        rules.player_armor_slots.push(armor_slot.clone());
        rules
            .items
            .register(
                project_rl::item::ItemDefinition::new(
                    armor.clone(),
                    "lab.armor.name".into(),
                    "lab.armor.description".into(),
                    1,
                    project_rl::item::ItemKind::Armor,
                    Some(
                        project_rl::item::EquipmentProfile::new(armor_slot, 2)
                            .map_err(|e| e.to_string())?,
                    ),
                    vec![],
                )
                .map_err(|e| e.to_string())?
                .with_mass_grams(2_000)
                .map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        texts
            .register("fr".into(), "lab.armor.name".into(), "Armure témoin".into())
            .map_err(|e| e.to_string())?;
        texts
            .register(
                "fr".into(),
                "equipment_slot.lab:body_armor".into(),
                "Torse".into(),
            )
            .map_err(|e| e.to_string())?;
        texts
            .register(
                "fr".into(),
                "lab.armor.description".into(),
                "Une armure pour comparer les bonus une fois portée.".into(),
            )
            .map_err(|e| e.to_string())?;
        rules
            .player_starting_items
            .push(project_rl::game::StartingItemStack::new(armor.clone(), 1));
        magic.push((
            armor,
            roll_lab_bonuses(&mut bonus_rng, EquipmentNameGrammar::FeminineSingular),
        ));
        rules.player_starting_equipment = vec![
            Some(lab_id("blade_impact")),
            Some(lab_id("rifle_impact")),
            Some(lab_id("blade_bearer")),
        ];
        let mut app = Self::from_seed_base(
            LAB_SEED,
            rules,
            texts,
            base.loot.clone(),
            base.expeditions.clone(),
            base.regional_worlds.clone(),
            CURRENT_GENERATION_VERSION,
        )?;
        let mut map = Map::filled(36, 25, Terrain::Floor).map_err(|e| e.to_string())?;
        let mut decor = SectorDecor {
            fallback_name: Some("Laboratoire de test".into()),
            ..Default::default()
        };
        for y in 0..25 {
            for x in 0..36 {
                let position = GridPos::new(x, y);
                let (terrain, tile) = if x == 0
                    || x == 35
                    || y == 0
                    || y == 24
                    || (x == 18 && (14..=21).contains(&y))
                {
                    (Terrain::Wall, crate::test_sector::Decor::Wall)
                } else if (21..=29).contains(&x) && (5..=11).contains(&y) {
                    (
                        Terrain::ShallowWater,
                        crate::test_sector::Decor::ShallowWater,
                    )
                } else {
                    (
                        Terrain::Floor,
                        if y == 12 || x == 7 {
                            crate::test_sector::Decor::Lane
                        } else {
                            crate::test_sector::Decor::Grate
                        },
                    )
                };
                map.set_terrain(position, terrain)
                    .map_err(|e| e.to_string())?;
                decor.cells.insert(position, tile);
            }
        }
        for (name, bounds) in [
            ("Essais de conduction", [20, 4, 11, 9]),
            ("Essais d'obstacles", [14, 14, 10, 9]),
            ("Essais de groupe", [12, 5, 6, 6]),
            ("Essais d'impact", [5, 10, 9, 4]),
            ("Essais de protection", [4, 17, 7, 6]),
        ] {
            decor.zones.push(crate::test_sector::Zone {
                name: name.into(),
                bounds,
            });
        }
        let mut game = GameState::new_with_rules(map, LAB_START, LAB_SEED, app.rules.clone())
            .map_err(|e| e.to_string())?
            .with_starting_magic_equipment(magic)?
            .with_starting_weapon_effects(instance_effects)?;
        let maximum = game
            .actors()
            .get(game.player_id())
            .ok_or("Personnage absent")?
            .maximum_integrity();
        game = game.with_starting_player_integrity(maximum.saturating_sub(12).max(1))?;
        let mut glyphs = BTreeMap::new();
        for (x, y) in [
            (8, 12),
            (10, 12),
            (11, 12),
            (8, 5),
            (14, 8),
            (15, 8),
            (14, 9),
            (22, 8),
            (24, 8),
            (26, 8),
            (17, 17),
            (19, 17),
        ] {
            let target = Actor::new(GridPos::new(x, y), 120)
                .map_err(|e| e.to_string())?
                .with_evasion_disabled()
                .with_tags([lab_id("healing_target")])
                .with_body_profile(
                    project_rl::stats::BodyProfile::new(120, 0)
                        .map_err(|e| e.to_string())?
                        .with_displacement_profile(
                            project_rl::stats::DisplacementProfile::new(10_000, 0)
                                .map_err(|e| e.to_string())?,
                        ),
                )
                .with_player_relation(PlayerRelation::Hostile);
            let entity = game.spawn_actor(target).map_err(|e| e.to_string())?;
            glyphs.insert(entity, 'X');
        }
        for (at, mass, fixed, symbol) in [
            (GridPos::new(12, 18), 80_000, false, 'L'),
            (GridPos::new(12, 21), 10_000, true, 'A'),
        ] {
            let displacement =
                project_rl::stats::DisplacementProfile::new(mass, 0).map_err(|e| e.to_string())?;
            let body = project_rl::stats::BodyProfile::new(120, 0)
                .map_err(|e| e.to_string())?
                .with_displacement_profile(if fixed {
                    displacement.fixed()
                } else {
                    displacement
                });
            let actor = Actor::new(at, 120)
                .map_err(|e| e.to_string())?
                .with_body_profile(body)
                .with_evasion_disabled()
                .with_tags([lab_id("healing_target")])
                .with_player_relation(PlayerRelation::Hostile);
            let entity = game.spawn_actor(actor).map_err(|e| e.to_string())?;
            glyphs.insert(entity, symbol);
        }
        for (position, armored, glyph) in [
            (GridPos::new(25, 17), true, 'B'),
            (GridPos::new(29, 17), false, 'E'),
        ] {
            let body = project_rl::stats::BodyProfile::new(120, 0)
                .map_err(|e| e.to_string())?
                .with_base_armor(if armored { 4 } else { 0 });
            let target = Actor::new(position, 120)
                .map_err(|e| e.to_string())?
                .with_body_profile(body)
                .with_tags([lab_id("healing_target")])
                .with_player_relation(PlayerRelation::Hostile);
            let target = if armored {
                target.with_evasion_disabled()
            } else {
                target
                    .with_primary_attributes(PrimaryAttributes::new(5, 8, 5, 5, 5))
                    .with_evasion_modifier(15)
            };
            let id = game.spawn_actor(target).map_err(|e| e.to_string())?;
            glyphs.insert(id, glyph);
        }
        let probe = Actor::new(LAB_PROBE, 120)
            .map_err(|e| e.to_string())?
            .with_evasion_disabled()
            .with_tags([lab_id("damage_probe")])
            .with_player_relation(PlayerRelation::Hostile)
            .with_ai(project_rl::ai::AiProfile::sentry(1, 0))
            .with_attacks([AttackProfile::new(
                1,
                DistanceMetric::Chebyshev,
                true,
                DamageType::Electrical,
                4,
                0,
            )]);
        let probe = game.spawn_actor(probe).map_err(|e| e.to_string())?;
        // Internal actor identity; terminal presentation is "!", not the
        // consumable symbol whose sensor category must remain an item.
        glyphs.insert(probe, 'Y');
        game.drain_events();
        app.terminal = TerminalView::new(decor, game.map(), game.player_visibility());
        app.game = WorldState::single(game);
        app.game
            .enable(project_rl::game::ZoneInfo {
                id: lab_id("test_map"),
                name: "Laboratoire de test".into(),
                kind: lab_id("laboratory"),
                depth: 0,
            })
            .map_err(|e| e.to_string())?;
        app.game.drain_events();
        app.refresh_zone_title();
        app.actor_glyphs = glyphs;
        app.zone_views.clear();
        app.zone_decor.clear();
        app.regional_zones.clear();
        app.test_lab = true;
        app.lab_bonus_seed = bonus_seed;
        visuals
            .register(VisualCueDefinition::new(
                lab_id("caustic_ground"),
                visuals
                    .get(&"core:caustic_ground".parse().unwrap())
                    .ok_or("Profil visuel caustique absent")?
                    .terminal()
                    .clone(),
            ))
            .map_err(|e| e.to_string())?;
        visuals
            .register(VisualCueDefinition::new(
                lab_id("impact_aegis"),
                visuals
                    .get(&"core:impact_aegis".parse().unwrap())
                    .ok_or("Profil visuel d'égide absent")?
                    .terminal()
                    .clone(),
            ))
            .map_err(|e| e.to_string())?;
        app.visual_cues = VisualCuePlayer::with_catalog(visuals);
        app.intro_city_reached = true;
        app.controls = self.controls.clone();
        app.controls_path = self.controls_path.clone();
        app.graphics = self.graphics.clone();
        app.options_message = self.options_message.clone();
        app.suspension_path = self.suspension_path.clone();
        app.crash_recovery_enabled = false;
        app.selected_target = app.game.actors().entity_at(GridPos::new(8, 12));
        app.log = vec![
            "LABORATOIRE DE TEST · session temporaire, sans sauvegarde.".into(),
            "X : cibles immobiles · eau : conduction · mur : arrêt de propagation.".into(),
            "54 armes et 2 armures d'essai, avec/sans bonus aléatoires. Ricochet : fusils uniquement.".into(),
            "Vol de vie : départ blessé ; 50 % des dégâts, au plus 3 PV par attaque. Mannequins autorisés ici seulement.".into(),
            "Inventaire : choisissez une variante. Réinitialiser le laboratoire tire de nouveaux bonus."
                .into(),
            "! au sud : dispositif d'essai immobile, frappe à 1 case (4 dégâts électriques). Égide : absorbe jusqu'à 3 dégâts puis se dissipe.".into(),
            "Percussion : X légers · L lourd · A ancré au sud-est. Recharge visible sous l'arme active.".into(),
            "Catalyse : cône de flammes depuis le porteur ; chaque cible déjà brûlante explose.".into(),
            "Au sud-est : B, mannequin blindé ; E, mannequin d'esquive. Aucun ne riposte.".into(),
        ];
        Ok(app)
    }

    pub(super) fn enter_test_lab(&mut self) -> Result<(), String> {
        if self.test_lab || self.menu != MenuScreen::Main {
            return Err("Le laboratoire s'ouvre depuis le menu principal.".into());
        }
        let lab = self.build_test_lab()?;
        let previous = std::mem::replace(self, lab);
        self.lab_return = Some(Box::new(previous));
        Ok(())
    }

    pub(super) fn reset_test_lab(&mut self) -> Result<(), String> {
        if !self.test_lab {
            return Err("Aucun laboratoire à réinitialiser.".into());
        }
        let mut fresh = self.build_test_lab()?;
        fresh.lab_return = self.lab_return.take();
        *self = fresh;
        Ok(())
    }

    pub(super) fn leave_test_lab(&mut self) {
        let Some(mut previous) = self.lab_return.take() else {
            return;
        };
        previous.controls = self.controls.clone();
        previous.graphics = self.graphics.clone();
        previous.options_message = self.options_message.clone();
        previous.open_menu(MenuScreen::Main);
        *self = *previous;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controls::{Binding, Layout};

    fn main_menu() -> AsciiApp {
        let (rules, texts, loot, expeditions) = ascii_game_content().unwrap();
        let mut app = AsciiApp::from_seed(INITIAL_SEED, rules, texts, loot, expeditions).unwrap();
        app.controls = Controls::preset(Layout::Azerty, KeySemantics::Physical);
        app.suspension_path = std::env::temp_dir().join(format!(
            "rl-lab-unused-{}-{}.json",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        app.open_menu(MenuScreen::Main);
        app
    }

    fn key(value: &str) -> InputFrame {
        InputFrame {
            pressed: [Binding::key(value)].into(),
            pause: value == "Escape",
            ..Default::default()
        }
    }

    #[test]
    fn gamble_names_never_reveal_instance_prefixes_or_effect_suffixes() {
        let app = main_menu()
            .build_test_lab_with_bonus_seed(LAB_SEED)
            .unwrap();
        let base = lab_id("rifle_reference");
        let bonus = app
            .game
            .player_inventory()
            .iter()
            .filter_map(|entry| entry.magic_modifiers())
            .find(|bonus| bonus.named_affixes().is_some() && bonus.effect_affix().is_some())
            .unwrap();
        let identified = app.equipment_name(&base, Some(bonus.clone()));
        assert_ne!(identified, app.item_name(&base));
        assert_eq!(
            app.trade_item_name(NpcTradeMode::Gamble, &base, Some(bonus.clone())),
            app.item_name(&base)
        );
        assert_eq!(
            app.trade_item_name(NpcTradeMode::Buy, &base, Some(bonus)),
            identified
        );
    }

    #[test]
    fn cone_preview_follows_the_instance_not_the_base_model() {
        use project_rl::entity::MagicItemModifiers;
        let mut app = main_menu()
            .build_test_lab_with_bonus_seed(LAB_SEED)
            .unwrap();
        let mut game = GameState::new_with_rules(
            app.game.map().clone(),
            LAB_START,
            LAB_SEED,
            app.game.rules().clone(),
        )
        .unwrap()
        .with_starting_magic_equipment([(
            lab_id("rifle_reference"),
            MagicItemModifiers::effect_only(lab_id("effect_catalysis")),
        )])
        .unwrap();
        let item = game
            .player_inventory()
            .iter()
            .find(|entry| entry.magic_modifiers().is_some())
            .unwrap()
            .instance();
        let target = game
            .spawn_actor(
                Actor::new(GridPos::new(8, 12), 120)
                    .unwrap()
                    .with_evasion_disabled(),
            )
            .unwrap();
        assert!(
            !game
                .rules()
                .weapons
                .get(&lab_id("rifle_reference"))
                .unwrap()
                .requires_area_aim()
        );
        assert_eq!(
            game.process_player_command(GameCommand::EquipWeapon { slot: 0, item }),
            CommandOutcome::Applied
        );
        app.game = WorldState::single(game);
        app.selected_target = Some(target);
        app.active_weapon_slot = 0;
        let before = suspension::fingerprint(&app.game);
        assert!(app.weapon_command(None).is_none());
        let aim = app
            .attack_aim
            .expect("instance effect must open the cone preview");
        assert!(app.aimed_attack_footprint(aim).unwrap().cells().len() > 2);
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn inventory_groups_preserve_selected_instances_when_equipping_with_keyboard_or_mouse() {
        let mut app = main_menu()
            .build_test_lab_with_bonus_seed(LAB_SEED)
            .unwrap();
        app.inventory_open = true;
        app.inventory_filter = InventoryFilter::All;
        let armor = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| {
                entry.item() == &lab_id("armor_reference") && entry.magic_modifiers().is_some()
            })
            .unwrap()
            .instance();
        let weapon = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| {
                app.game.rules().weapons.get(entry.item()).is_some()
                    && app
                        .game
                        .player_equipment()
                        .slot_of(entry.instance())
                        .is_none()
            })
            .unwrap()
            .instance();
        app.select_inventory_instance(armor);
        app.update_input(&key("U"));
        assert_eq!(
            app.inventory_entries()[app.inventory_selection].instance(),
            armor
        );
        assert_eq!(
            app.game.player_equipment().equipped(&lab_id("body_armor")),
            Some(armor)
        );
        assert!(
            app.inventory_equipped_label(armor)
                .unwrap()
                .starts_with("ÉQUIPÉ · ")
        );

        app.select_inventory_instance(weapon);
        let layout = InventoryLayout::with_equipment(
            1280.0,
            800.0,
            app.inventory_selection,
            app.inventory_entries().len(),
            app.inventory_equipped_count(),
        );
        let equip = layout.actions[1];
        app.update_input(&InputFrame {
            pressed: [Binding::MouseLeft].into(),
            pointer: Some((equip.x + 10.0, equip.y + 10.0)),
            viewport: Some((1280.0, 800.0)),
            ..Default::default()
        });
        assert_eq!(
            app.inventory_entries()[app.inventory_selection].instance(),
            weapon
        );
        assert_eq!(
            app.inventory_equipped_label(weapon).as_deref(),
            Some("EN MAIN · 2")
        );
        assert_eq!(app.inventory_selection, 1);
        let ordered = app
            .inventory_entries()
            .iter()
            .map(|entry| entry.instance())
            .collect::<Vec<_>>();
        app.active_weapon_slot = 0;
        assert_eq!(
            app.inventory_equipped_label(weapon).as_deref(),
            Some("ÉQUIPÉ · 2")
        );
        assert_eq!(
            app.inventory_entries()
                .iter()
                .map(|entry| entry.instance())
                .collect::<Vec<_>>(),
            ordered
        );
        let equipped = app.inventory_equipped_count();
        assert_eq!(equipped, 4);
        for (index, entry) in app.inventory_entries().iter().enumerate() {
            assert_eq!(
                app.game
                    .player_equipment()
                    .slot_of(entry.instance())
                    .is_some(),
                index < equipped
            );
        }
    }

    #[test]
    fn inventory_navigation_crosses_groups_and_keeps_sort_and_filter_selection_without_a_turn() {
        let mut app = main_menu()
            .build_test_lab_with_bonus_seed(LAB_SEED)
            .unwrap();
        app.inventory_open = true;
        app.inventory_filter = InventoryFilter::All;
        app.inventory_selection = 0;
        let before = (app.game.turn(), app.game.rng_state(), app.history.len());
        let count = app.inventory_entries().len();
        for index in 1..count {
            app.update_input(&key("Down"));
            assert_eq!(app.inventory_selection, index);
        }
        for index in (0..count - 1).rev() {
            app.update_input(&key("Up"));
            assert_eq!(app.inventory_selection, index);
        }
        let item = app.inventory_entries()[0].instance();
        app.update_input(&key("T"));
        assert_eq!(app.inventory_sort, InventorySort::Name);
        assert_eq!(
            app.inventory_entries()[app.inventory_selection].instance(),
            item
        );
        app.update_input(&key("Tab"));
        assert_eq!(app.inventory_filter, InventoryFilter::Weapons);
        assert_eq!(
            app.inventory_entries()[app.inventory_selection].instance(),
            item
        );
        for expected in [
            InventoryFilter::Armor,
            InventoryFilter::Consumables,
            InventoryFilter::Materials,
            InventoryFilter::All,
        ] {
            app.update_input(&key("Tab"));
            assert_eq!(app.inventory_filter, expected);
        }
        assert_eq!(
            (app.game.turn(), app.game.rng_state(), app.history.len()),
            before
        );
    }

    #[test]
    fn inventory_mouse_rows_headings_and_wheel_use_the_rendered_groups() {
        let mut app = main_menu()
            .build_test_lab_with_bonus_seed(LAB_SEED)
            .unwrap();
        app.inventory_open = true;
        app.inventory_filter = InventoryFilter::All;
        let before = (app.game.turn(), app.game.rng_state(), app.history.len());
        for (width, height) in [(960.0, 540.0), (1280.0, 800.0), (1536.0, 864.0)] {
            app.inventory_selection = 0;
            let layout = InventoryLayout::with_equipment(
                width,
                height,
                0,
                app.inventory_entries().len(),
                app.inventory_equipped_count(),
            );
            let click = |rect: Rect| InputFrame {
                pressed: [Binding::MouseLeft].into(),
                pointer: Some((rect.x + 8.0, rect.y + 8.0)),
                viewport: Some((width, height)),
                ..Default::default()
            };
            for (_, heading) in &layout.sections {
                app.update_input(&click(*heading));
                assert_eq!(app.inventory_selection, 0);
            }
            let (index, row) = *layout.rows.last().unwrap();
            app.update_input(&click(row));
            assert_eq!(app.inventory_selection, index);
            let mut wheel = click(layout.list);
            wheel.pressed.clear();
            wheel.wheel_y = -1.0;
            app.update_input(&wheel);
            assert!(app.inventory_selection > index);
            let selection = app.inventory_selection;
            wheel.pointer = Some((layout.list.right() + 25.0, layout.list.y + 40.0));
            app.update_input(&wheel);
            assert_eq!(app.inventory_selection, selection);
            let selected_item = app.inventory_entries()[selection].instance();
            app.update_input(&click(layout.sort));
            assert_eq!(
                app.inventory_entries()[app.inventory_selection].instance(),
                selected_item
            );
        }
        assert_eq!(
            (app.game.turn(), app.game.rng_state(), app.history.len()),
            before
        );
    }

    #[test]
    fn sustained_fields_follow_real_turns_not_event_timestamps_and_leave_no_final_flash() {
        use crate::visual_effects::TerminalEffectFamily;
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        for (element, families) in [
            ("fire", vec![TerminalEffectFamily::Flame]),
            ("acid", vec![TerminalEffectFamily::Caustic]),
            ("electric", vec![TerminalEffectFamily::Conduction]),
            (
                "mixed",
                vec![
                    TerminalEffectFamily::Flame,
                    TerminalEffectFamily::Caustic,
                    TerminalEffectFamily::Conduction,
                ],
            ),
        ] {
            app.prepare_lab_ground_fixture(element).unwrap();
            let at = GridPos::new(10, 12);
            assert!(app.game.actors().entity_at(at).is_none());
            for remaining in (1..=3).rev() {
                app.capture_events_at(Some(10.0));
                assert!(
                    app.game
                        .ground_effects()
                        .at(at)
                        .all(|field| field.remaining_turns() == remaining)
                );
                let before = suspension::fingerprint(&app.game);
                for time in [10.0, 10.23, 100.0, 86_400.0] {
                    for reduced in [false, true] {
                        let samples = app.sustained_effects_at(at, time, reduced);
                        assert_eq!(samples.len(), families.len());
                        for family in &families {
                            assert!(samples.iter().any(|sample| sample.family == *family
                                && sample.sustained
                                && sample.color.a > 0.0));
                        }
                        assert!(
                            app.sustained_effects_at(GridPos::new(34, 23), time, reduced)
                                .is_empty()
                        );
                    }
                }
                assert_eq!(suspension::fingerprint(&app.game), before);
                app.visual_cues.clear_world();
                assert_eq!(
                    app.sustained_effects_at(at, 500.0, false).len(),
                    families.len()
                );
                assert_eq!(
                    app.execute_command(GameCommand::Wait),
                    CommandOutcome::Applied
                );
            }
            app.capture_events_at(Some(10.0));
            assert!(app.game.ground_effects().is_empty());
            assert!(app.sustained_effects_at(at, 10.01, false).is_empty());
            assert!(app.visual_cues.sample_world(at, true, 10.01).is_none());
        }
    }

    #[test]
    fn sustained_statuses_coexist_and_stop_with_their_actual_status() {
        use crate::visual_effects::TerminalEffectFamily;
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        app.prepare_lab_status_fixture().unwrap();
        app.capture_events_at(Some(10.0));
        let at = GridPos::new(8, 12);
        let samples = app.sustained_effects_at(at, 30_000.0, false);
        assert_eq!(samples.len(), 2); // Flame + corrosion, not the movement debuff.
        assert!(
            samples
                .iter()
                .any(|sample| sample.family == TerminalEffectFamily::Flame)
        );
        assert!(
            samples
                .iter()
                .any(|sample| sample.family == TerminalEffectFamily::Corrosion)
        );
        app.visual_cues.clear_world();
        for _ in 0..6 {
            app.execute_command(GameCommand::Wait);
        }
        app.capture_events_at(Some(10.0));
        assert!(app.sustained_effects_at(at, 10.01, false).is_empty());
        assert!(
            app.visual_cues
                .sample_world(at, true, 10.01)
                .is_none_or(|sample| !matches!(
                    sample.family,
                    TerminalEffectFamily::Flame | TerminalEffectFamily::Corrosion
                ))
        );
    }

    #[test]
    fn sustained_burning_follows_its_bearer_without_leaving_false_ground_fire() {
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        // Use burning alone: the other fixture's locomotion debuff makes a
        // move take several turns and would legitimately expire the statuses.
        let mut rules = app.rules.clone();
        rules.player_base_abilities = vec![AbilityProfile::new(
            7,
            DistanceMetric::Chebyshev,
            true,
            true,
            vec![EffectPrimitive::ApplyStatus(
                ApplyStatusEffect::new("core:burning".parse().unwrap(), 1).unwrap(),
            )],
        )];
        app.game = WorldState::single(
            GameState::new_with_rules(app.game.map().clone(), LAB_START, LAB_SEED, rules).unwrap(),
        );
        assert_eq!(
            app.execute_command(GameCommand::UseAbility {
                slot: 0,
                target: LAB_START
            }),
            CommandOutcome::Applied
        );
        assert_eq!(app.sustained_effects_at(LAB_START, 50.0, false).len(), 1);
        assert_eq!(
            app.execute_command(GameCommand::Move(Direction::West)),
            CommandOutcome::Applied
        );
        let moved = app.game.player_position().unwrap();
        assert_ne!(moved, LAB_START);
        assert!(app.sustained_effects_at(LAB_START, 50.0, false).is_empty());
        assert_eq!(app.sustained_effects_at(moved, 50.0, false).len(), 1);
        assert!(app.game.ground_effects().is_empty());
    }

    fn click(menu: MenuScreen, row: usize) -> InputFrame {
        let layout = MenuLayout::for_screen(menu, 1280.0, 800.0, menu.buttons().len());
        let button = layout.buttons[row];
        InputFrame {
            pressed: [Binding::MouseLeft].into(),
            pointer: Some((button.x + 10.0, button.y + 10.0)),
            viewport: Some((1280.0, 800.0)),
            ..Default::default()
        }
    }

    #[test]
    fn main_menu_keyboard_and_mouse_open_the_same_disposable_map() {
        for mouse in [false, true] {
            let mut app = main_menu();
            assert_eq!(app.menu_labels()[2], "Laboratoire de test");
            if mouse {
                app.update_input(&click(MenuScreen::Main, 2));
            } else {
                assert_eq!(app.menu_selection, 1);
                app.update_input(&key("Down"));
                app.update_input(&key("Enter"));
            }
            assert!(app.test_lab, "{}", app.menu_message);
            assert_eq!(app.menu, MenuScreen::Hidden);
            assert!(app.character_creation.is_none());
            assert!(!app.crash_recovery_enabled);
            assert_eq!(
                app.game
                    .actors()
                    .get(app.game.player_id())
                    .unwrap()
                    .position(),
                LAB_START
            );
            assert_eq!(app.actor_glyphs.len(), 17);
            assert_eq!(app.terminal.title, "Laboratoire de test");
            let target = app.terminal_target_summary().unwrap();
            assert_eq!(target.name, "Mannequin d'essai");
            assert!(target.analysis.is_some());
            assert!(app.enter_test_lab().is_err());
        }
    }

    #[test]
    fn lab_bonus_rolls_are_varied_bounded_and_reproducible_without_weight_reduction() {
        let mut first = project_rl::game::GameRng::from_seed(91);
        let mut second = first;
        let rolls: Vec<_> = (0..128)
            .map(|_| roll_lab_bonuses(&mut first, EquipmentNameGrammar::FeminineSingular))
            .collect();
        assert_eq!(
            rolls,
            (0..128)
                .map(|_| roll_lab_bonuses(&mut second, EquipmentNameGrammar::FeminineSingular))
                .collect::<Vec<_>>()
        );
        let unique: std::collections::BTreeSet<_> =
            rolls.iter().map(|r| format!("{r:?}")).collect();
        assert!(unique.len() > 30);
        for bonus in rolls {
            assert_eq!(bonus.mass_reduction_percent(), 0);
            assert_eq!(bonus.armor_bonus(), 0);
            let attributes: Vec<_> = PrimaryAttribute::ALL
                .into_iter()
                .map(|a| bonus.attribute_bonus(a))
                .collect();
            assert!(attributes.iter().all(|v| *v <= 2));
            let count = attributes.iter().filter(|v| **v > 0).count()
                + usize::from(bonus.accuracy_bonus() > 0)
                + usize::from(bonus.armor_penetration_bonus() > 0)
                + usize::from(bonus.maximum_hit_points_bonus() > 0)
                + usize::from(bonus.energy_capacity_bonus() > 0)
                + usize::from(bonus.heat_dissipation_bonus() > 0);
            assert!((1..=3).contains(&count));
            assert!(bonus.accuracy_bonus() == 0 || (5..=10).contains(&bonus.accuracy_bonus()));
            assert!(bonus.armor_penetration_bonus() <= 1);
            assert!(
                bonus.maximum_hit_points_bonus() == 0
                    || (5..=15).contains(&bonus.maximum_hit_points_bonus())
            );
            assert!(
                bonus.energy_capacity_bonus() == 0
                    || (5..=15).contains(&bonus.energy_capacity_bonus())
            );
            assert!(bonus.heat_dissipation_bonus() <= 1);
            let affixes = bonus.named_affixes().unwrap();
            assert_eq!(affixes.iter().count(), count);
            assert!(affixes.iter().all(|roll| roll.tier() == LAB_AFFIX_TIER));
        }
    }

    #[test]
    fn equipped_stats_remain_stable_when_switching_but_life_steal_does_not_transfer() {
        let mut app = main_menu()
            .build_test_lab_with_bonus_seed(LAB_SEED)
            .unwrap();
        let find = |app: &AsciiApp, name: &str, magical: bool| {
            app.game
                .player_inventory()
                .iter()
                .find(|entry| {
                    entry.item() == &lab_id(name)
                        && entry
                            .magic_modifiers()
                            .and_then(|bonus| bonus.named_affixes())
                            .is_some()
                            == magical
                })
                .unwrap()
                .instance()
        };
        let rifle = find(&app, "rifle_reference", false);
        let blade = find(&app, "blade_healing", true);
        app.execute_command(GameCommand::EquipWeapon {
            slot: 0,
            item: rifle,
        });
        app.execute_command(GameCommand::EquipWeapon {
            slot: 1,
            item: blade,
        });
        let before = suspension::fingerprint(&app.game);
        for slot in [1, 0, 1, 0] {
            app.select_weapon_slot(slot);
            assert_eq!(suspension::fingerprint(&app.game), before);
        }
        let player = app.game.player_id();
        let hp = app.game.actors().get(player).unwrap().integrity();
        let target = app.selected_target.unwrap();
        app.execute_command(GameCommand::Attack { slot: 0, target });
        assert_eq!(app.game.actors().get(player).unwrap().integrity(), hp);
        app.select_weapon_slot(1);
        app.execute_command(GameCommand::Attack { slot: 1, target });
        assert!(app.game.actors().get(player).unwrap().integrity() > hp);
    }

    #[test]
    fn affix_names_and_values_survive_drop_recovery_and_pickup_without_rerolling() {
        let mut app = main_menu()
            .build_test_lab_with_bonus_seed(LAB_SEED)
            .unwrap();
        let entries: Vec<_> = app
            .game
            .player_inventory()
            .iter()
            .filter(|entry| entry.magic_modifiers().is_some())
            .map(|entry| {
                (
                    entry.instance(),
                    entry.item().clone(),
                    entry.magic_modifiers(),
                    app.inventory_entry_name(entry),
                )
            })
            .collect();
        let before = suspension::fingerprint(&app.game);
        for _ in 0..3 {
            for entry in app.inventory_entries() {
                let _ = app.inventory_entry_name(entry);
            }
        }
        assert_eq!(suspension::fingerprint(&app.game), before);
        for (instance, id, bonus, name) in entries {
            assert_eq!(
                app.execute_command(GameCommand::DropItem { item: instance }),
                CommandOutcome::Applied
            );
            let ground = app
                .game
                .ground_items()
                .get(
                    app.game
                        .ground_items()
                        .item_at(app.game.player_position().unwrap())
                        .unwrap(),
                )
                .unwrap();
            assert_eq!(ground.magic_modifiers(), bonus);
            assert_eq!(
                app.equipment_name(ground.item(), ground.magic_modifiers()),
                name
            );
            app.game.drain_events();
            let snapshot = app.game.recovery_snapshot_bytes().unwrap();
            app.game =
                WorldState::from_recovery_snapshot_bytes(&snapshot, app.game.rules().clone())
                    .unwrap();
            assert_eq!(
                app.execute_command(GameCommand::PickUp),
                CommandOutcome::Applied
            );
            let recovered = app
                .game
                .player_inventory()
                .iter()
                .find(|entry| entry.item() == &id && entry.magic_modifiers() == bonus)
                .unwrap();
            assert_eq!(app.inventory_entry_name(recovered), name);
            assert_ne!(recovered.instance(), instance);
            // The counterpart remains a distinct instance with its own rolls.
            assert!(
                app.game
                    .player_inventory()
                    .iter()
                    .any(|entry| entry.item() == &id && entry.magic_modifiers() != bonus)
            );
        }
    }

    #[test]
    fn armor_bonuses_apply_only_when_worn_and_combine_with_secondary_weapons() {
        let mut app = main_menu()
            .build_test_lab_with_bonus_seed(LAB_SEED)
            .unwrap();
        let pair: Vec<_> = app
            .game
            .player_inventory()
            .iter()
            .filter(|entry| entry.item() == &lab_id("armor_reference"))
            .map(|entry| (entry.instance(), entry.magic_modifiers()))
            .collect();
        assert_eq!(pair.len(), 2);
        let (plain, _) = pair.iter().find(|(_, b)| b.is_none()).cloned().unwrap();
        let (magic, bonus) = pair.iter().find(|(_, b)| b.is_some()).cloned().unwrap();
        let bonus = bonus.unwrap();
        let weapon = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| {
                entry.item() == &lab_id("rifle_reference") && entry.magic_modifiers().is_some()
            })
            .unwrap()
            .instance();
        app.execute_command(GameCommand::EquipWeapon {
            slot: 1,
            item: weapon,
        });
        let base = app.game.player_effective_primary_attributes().unwrap();
        let max_hp = app
            .game
            .actors()
            .get(app.game.player_id())
            .unwrap()
            .maximum_integrity();
        let energy = app.game.player_energy();
        let heat = app.game.player_heat().unwrap();
        assert_eq!(
            app.execute_command(GameCommand::EquipItem {
                slot: lab_id("body_armor"),
                item: magic
            }),
            CommandOutcome::Applied
        );
        for attribute in PrimaryAttribute::ALL {
            assert_eq!(
                app.game
                    .player_effective_primary_attributes()
                    .unwrap()
                    .value(attribute),
                base.value(attribute) + bonus.attribute_bonus(attribute)
            );
        }
        assert_eq!(
            app.game.player_energy().capacity(),
            energy.capacity() + bonus.energy_capacity_bonus()
        );
        assert_eq!(app.game.player_energy().available(), energy.available());
        assert_eq!(
            app.game.player_heat().unwrap().dissipation_per_phase(),
            heat.dissipation_per_phase() + bonus.heat_dissipation_bonus()
        );
        app.game.drain_events();
        let snapshot = app.game.recovery_snapshot_bytes().unwrap();
        let restored =
            WorldState::from_recovery_snapshot_bytes(&snapshot, app.game.rules().clone()).unwrap();
        assert_eq!(
            suspension::fingerprint(&restored),
            suspension::fingerprint(&app.game)
        );
        // Reject the obsolete binary envelope cleanly; regular suspension has
        // an authoritative command journal for its historical replay fallback.
        assert!(
            WorldState::from_recovery_snapshot_bytes(&snapshot[5..], app.game.rules().clone())
                .is_err()
        );
        app.execute_command(GameCommand::EquipItem {
            slot: lab_id("body_armor"),
            item: plain,
        });
        assert_eq!(app.game.player_effective_primary_attributes(), Some(base));
        assert_eq!(
            app.game
                .actors()
                .get(app.game.player_id())
                .unwrap()
                .maximum_integrity(),
            max_hp
        );
        assert_eq!(app.game.player_energy(), energy);
        assert_eq!(
            app.game.player_heat().unwrap().dissipation_per_phase(),
            heat.dissipation_per_phase()
        );
    }

    #[test]
    fn all_profiles_have_melee_ranged_and_real_stat_bonus_variants() {
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        assert_eq!(app.game.player_inventory().len(), 56);
        let mut expected_mass = 4_000_u64; // White/magical armor pair.
        for (prefix, base) in [
            ("blade", "core:integrity_blade"),
            ("rifle", "core:needle_launcher"),
        ] {
            let original = app.rules.weapons.get(&base.parse().unwrap()).unwrap();
            for kind in LAB_KINDS {
                if prefix == "blade" && kind == "ricochet" {
                    continue;
                }
                let id = lab_id(&format!("{prefix}_{kind}"));
                let definition = app.rules.weapons.get(&id).unwrap();
                assert_eq!(definition.attack(), original.attack());
                // Campaign bases do not all declare a mass yet; the authored
                // laboratory base supplies one, shared by both instances.
                assert_eq!(
                    definition.mass_grams(),
                    Some(original.mass_grams().unwrap_or(2_000))
                );
                assert!(definition.effects().is_empty());
                let pair = app
                    .game
                    .player_inventory()
                    .iter()
                    .filter(|entry| entry.item() == &id)
                    .collect::<Vec<_>>();
                assert_eq!(pair.len(), 2);
                assert_ne!(pair[0].instance(), pair[1].instance());
                assert!(
                    pair[0]
                        .magic_modifiers()
                        .and_then(|bonus| bonus.named_affixes())
                        .is_none()
                );
                for entry in &pair {
                    assert_eq!(
                        app.game
                            .player_item_weapon(entry.instance())
                            .unwrap()
                            .effects()
                            .is_empty(),
                        kind == "reference"
                    );
                }
                let bonus = pair[1].magic_modifiers().unwrap();
                assert_eq!(bonus.mass_reduction_percent(), 0);
                assert_eq!(bonus.armor_bonus(), 0);
                let count = PrimaryAttribute::ALL
                    .iter()
                    .filter(|a| bonus.attribute_bonus(**a) > 0)
                    .count()
                    + usize::from(bonus.accuracy_bonus() > 0)
                    + usize::from(bonus.armor_penetration_bonus() > 0)
                    + usize::from(bonus.maximum_hit_points_bonus() > 0)
                    + usize::from(bonus.energy_capacity_bonus() > 0)
                    + usize::from(bonus.heat_dissipation_bonus() > 0);
                assert!((1..=3).contains(&count));
                let mass = u64::from(definition.mass_grams().unwrap());
                expected_mass += mass * 2;
            }
        }
        assert_eq!(
            app.game.actor_carried_mass_grams(app.game.player_id()),
            Some(expected_mass)
        );
    }

    #[test]
    fn two_zone_recipients_produce_two_totals_not_three_packets() {
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        let target = app.selected_target.unwrap();
        let before = app.game.actors().get(target).unwrap().integrity();
        app.execute_command(GameCommand::Attack { slot: 0, target });
        let applied = before - app.game.actors().get(target).unwrap().integrity();
        let fingerprint = suspension::fingerprint(app.game.actors());
        app.capture_events_at(Some(2.0));
        assert_eq!(suspension::fingerprint(app.game.actors()), fingerprint);
        let values = app
            .floating_messages
            .iter()
            .filter(|message| message.tone == FloatingMessageTone::Damage)
            .collect::<Vec<_>>();
        assert_eq!(values.len(), 2);
        assert_eq!(
            values
                .iter()
                .find(|value| value.at == GridPos::new(8, 12))
                .unwrap()
                .text,
            format!("−{applied}")
        );
        assert_eq!(
            values
                .iter()
                .find(|value| value.at == GridPos::new(10, 12))
                .unwrap()
                .text,
            "−4"
        );
        assert_eq!(app.glyph_at(GridPos::new(8, 12)).unwrap().0, 'X');
        assert_eq!(
            app.radial_visual_id(Some(&(lab_id("effect_impact"), 0)))
                .as_str(),
            "core:weapon_impact_wave"
        );
        assert_eq!(
            app.radial_visual_id(Some(&(lab_id("effect_bearer"), 0)))
                .as_str(),
            "core:weapon_bearer_wave"
        );
        assert_eq!(
            app.radial_visual_id(Some(&(lab_id("effect_conduction"), 0)))
                .as_str(),
            "core:weapon_conduction"
        );
    }

    #[test]
    fn corrosion_is_nonstacking_now_but_historical_rules_stay_replayable() {
        use project_rl::status::StatusStacking;
        let (rules, _, _, _) = ascii_game_content().unwrap();
        let id = "core:corroded".parse().unwrap();
        assert_eq!(
            rules.statuses.get(&id).unwrap().stacking(),
            StatusStacking::RefreshDuration
        );
        let current = rules_for_generation_version(rules.clone(), CURRENT_GENERATION_VERSION);
        let old = rules_for_generation_version(rules.clone(), 103);
        assert_eq!(
            old.statuses.get(&id).unwrap().stacking(),
            StatusStacking::AddStacks {
                maximum_stacks: 3,
                refresh_duration: true
            }
        );
        assert_eq!(
            current.statuses.get(&id).unwrap().stacking(),
            StatusStacking::RefreshDuration
        );
        assert_eq!(
            rules_fingerprint_for_version(&rules, 103),
            rules_fingerprint_for_version(&old, 103)
        );
    }

    #[test]
    fn life_steal_feedback_stays_on_the_bearer_and_stops_when_no_hp_is_restored() {
        use crate::visual_effects::TerminalEffectFamily;
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        let id = lab_id("blade_healing");
        let item = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &id)
            .unwrap()
            .instance();
        app.execute_command(GameCommand::EquipWeapon { slot: 0, item });
        let target = app.selected_target.unwrap();
        let player = app.game.player_id();
        for index in 0..10 {
            app.game.drain_events();
            app.visual_cues.clear_world();
            app.floating_messages.clear();
            let before = app.game.actors().get(player).unwrap().integrity();
            app.execute_command(GameCommand::Attack { slot: 0, target });
            let after = app.game.actors().get(player).unwrap().integrity();
            app.capture_events_at(Some(2.0));
            let cue = app.visual_cues.sample_world(LAB_START, true, 2.14);
            let messages: Vec<_> = app
                .floating_messages
                .iter()
                .filter(|value| value.tone == FloatingMessageTone::Recovery)
                .collect();
            if after > before {
                assert_eq!(cue.unwrap().family, TerminalEffectFamily::Restoration);
                assert_eq!(messages.len(), 1);
                assert_eq!(messages[0].at, LAB_START);
                assert_eq!(messages[0].lane, 1);
                assert_eq!(messages[0].text, format!("+{} PV", after - before));
                app.push_floating_message("−1", LAB_START, FloatingMessageTone::Damage, 2.0);
                assert_eq!(app.floating_messages.last().unwrap().lane, 0);
            } else {
                assert!(cue.is_none());
                assert!(messages.is_empty());
            }
            if index == 9 {
                assert_eq!(
                    after,
                    app.game.actors().get(player).unwrap().maximum_integrity()
                );
            }
        }
    }

    #[test]
    fn badges_show_all_three_timed_statuses_and_disappear_on_expiration_or_hidden_cells() {
        use crate::terminal_view::TerminalEffectBadge;
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        app.prepare_lab_status_fixture().unwrap();
        let at = GridPos::new(8, 12);
        let badges = app.effect_badges_at(at);
        assert_eq!(badges.len(), 3);
        for badge in [
            TerminalEffectBadge::Burning,
            TerminalEffectBadge::Corrosion,
            TerminalEffectBadge::Slowed,
        ] {
            assert!(badges.contains(&badge));
        }
        assert!(
            app.terminal_target_summary()
                .unwrap()
                .visible_state
                .contains("CORROSION")
        );
        for _ in 0..6 {
            app.execute_command(GameCommand::Wait);
        }
        assert!(app.effect_badges_at(at).is_empty());
        assert!(app.effect_badges_at(GridPos::new(34, 23)).is_empty());
    }

    #[test]
    fn caustic_exposure_badge_tracks_the_field_not_a_permanent_actor_status() {
        use crate::terminal_view::TerminalEffectBadge;
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        let item = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &lab_id("blade_caustic"))
            .unwrap()
            .instance();
        app.execute_command(GameCommand::EquipWeapon { slot: 0, item });
        let target = app.selected_target.unwrap();
        for _ in 0..2 {
            app.execute_command(GameCommand::Attack { slot: 0, target });
        }
        assert_eq!(app.game.ground_effects().at(GridPos::new(8, 12)).count(), 1);
        assert_eq!(
            app.effect_badges_at(GridPos::new(8, 12)),
            vec![TerminalEffectBadge::Caustic]
        );
        for _ in 0..4 {
            app.execute_command(GameCommand::Wait);
        }
        assert!(app.effect_badges_at(GridPos::new(8, 12)).is_empty());
    }

    #[test]
    fn aegis_badge_expires_without_refresh_and_probe_consumes_it() {
        use crate::terminal_view::TerminalEffectBadge;
        use crate::visual_effects::TerminalEffectFamily;
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        let item = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &lab_id("rifle_aegis"))
            .unwrap()
            .instance();
        app.execute_command(GameCommand::EquipWeapon { slot: 0, item });
        let target = app.selected_target.unwrap();
        for expected_turns in [2, 1, 0] {
            app.execute_command(GameCommand::Attack { slot: 0, target });
            let status = app
                .game
                .actors()
                .get(app.game.player_id())
                .unwrap()
                .status(&lab_id("impact_aegis"));
            assert_eq!(
                status.and_then(|s| s.remaining_turns),
                (expected_turns > 0).then_some(expected_turns)
            );
        }
        assert!(app.effect_badges_at(LAB_START).is_empty());
        let hp = app
            .game
            .actors()
            .get(app.game.player_id())
            .unwrap()
            .integrity();
        app.walk_fixture_to(GridPos::new(7, 18)).unwrap();
        assert_eq!(
            app.game
                .actors()
                .get(app.game.player_id())
                .unwrap()
                .integrity(),
            hp,
            "probe must stay harmless outside its immediate reach"
        );
        let probe = app.game.actors().entity_at(LAB_PROBE).unwrap();
        app.game.drain_events();
        app.visual_cues.clear_world();
        app.floating_messages.clear();
        app.execute_command(GameCommand::Attack {
            slot: 0,
            target: probe,
        });
        let at = GridPos::new(7, 18);
        assert_eq!(app.effect_badges_at(at), [TerminalEffectBadge::Guard]);
        app.capture_events_at(Some(10.0));
        assert_eq!(
            app.visual_cues
                .sample_world(at, true, 10.14)
                .unwrap()
                .family,
            TerminalEffectFamily::Guard
        );
        app.visual_cues.clear_world();
        app.floating_messages.clear();
        assert_eq!(
            app.execute_command(GameCommand::Move(Direction::South)),
            CommandOutcome::Applied
        );
        let at = GridPos::new(7, 19);
        assert!(
            app.game
                .events()
                .iter()
                .any(|event| matches!(event, GameEvent::DamageGuardAbsorbed { amount: 3, .. }))
        );
        assert_eq!(
            app.game
                .actors()
                .get(app.game.player_id())
                .unwrap()
                .integrity(),
            hp - 1
        );
        assert!(app.effect_badges_at(at).is_empty());
        app.capture_events_at(Some(11.0));
        assert_eq!(
            app.visual_cues
                .sample_world(at, true, 11.14)
                .unwrap()
                .family,
            TerminalEffectFamily::GuardBreak
        );
        assert_eq!(
            app.floating_messages
                .iter()
                .filter(|m| m.text == "BLOQUÉ 3")
                .count(),
            1
        );
        assert!(
            !app.floating_messages
                .iter()
                .any(|m| m.text.contains("TERMINÉE"))
        );
    }

    #[test]
    fn catalytic_cone_burns_new_targets_then_explodes_on_all_previously_burning_targets() {
        use crate::terminal_view::TerminalEffectBadge;
        use crate::visual_effects::TerminalEffectFamily;
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        let item = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &lab_id("blade_catalysis"))
            .unwrap()
            .instance();
        app.execute_command(GameCommand::EquipWeapon { slot: 0, item });
        let at = GridPos::new(8, 12);
        let target = app.game.actors().entity_at(at).unwrap();
        app.execute_command(GameCommand::Attack { slot: 0, target });
        assert!(
            !app.game
                .events()
                .iter()
                .any(|e| matches!(e, GameEvent::CatalyticExplosion { .. }))
        );
        assert!(
            app.effect_badges_at(at)
                .contains(&TerminalEffectBadge::Burning)
        );
        app.capture_events_at(Some(10.0));
        let jet = app
            .visual_cues
            .sample_world(GridPos::new(9, 12), true, 10.2)
            .unwrap();
        assert_eq!(jet.family, TerminalEffectFamily::FlameJet);
        assert!(app.visual_cues.sample_world(at, false, 10.2).is_none());
        app.floating_messages.clear();
        app.visual_cues.clear_world();
        app.execute_command(GameCommand::Attack { slot: 0, target });
        let bursts = app
            .game
            .events()
            .iter()
            .filter(|e| matches!(e, GameEvent::CatalyticExplosion { .. }))
            .count();
        assert!(bursts >= 2);
        assert!(
            app.effect_badges_at(at)
                .contains(&TerminalEffectBadge::Burning)
        );
        let mut totals: BTreeMap<GridPos, u32> = BTreeMap::new();
        for event in app.game.events() {
            if let GameEvent::DamageApplied { at, amount, .. }
            | GameEvent::DamageImpactApplied { at, amount, .. } = event
            {
                *totals.entry(*at).or_default() += u32::from(*amount);
            }
        }
        app.capture_events_at(Some(11.0));
        for (at, total) in totals {
            assert_eq!(
                app.floating_messages
                    .iter()
                    .filter(|m| m.at == at && m.text == format!("−{total}"))
                    .count(),
                1
            );
        }
    }

    #[test]
    fn percussion_fixed_fixture_route_avoids_the_damage_probe() {
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        let player = app.game.player_id();
        let before = app.game.actors().get(player).unwrap().integrity();
        app.walk_fixture_to(GridPos::new(11, 18)).unwrap();
        app.walk_fixture_to(GridPos::new(11, 21)).unwrap();
        assert_eq!(app.game.player_position(), Some(GridPos::new(11, 21)));
        assert_eq!(app.game.actors().get(player).unwrap().integrity(), before);
    }

    #[test]
    fn percussion_feedback_follows_the_same_target_and_reports_readiness() {
        use crate::visual_effects::TerminalEffectFamily;
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        let item = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &lab_id("rifle_percussion"))
            .unwrap()
            .instance();
        app.execute_command(GameCommand::EquipWeapon { slot: 0, item });
        assert_eq!(
            app.percussion_readiness().as_deref(),
            Some("PERCUSSION PRÊTE")
        );
        let target = app.selected_target.unwrap();
        app.game.drain_events();
        app.execute_command(GameCommand::Attack { slot: 0, target });
        app.capture_events_at(Some(10.0));
        let at = GridPos::new(9, 12);
        assert_eq!(app.game.actors().get(target).unwrap().position(), at);
        assert_eq!(app.selected_target, Some(target));
        assert_eq!(
            app.percussion_readiness().as_deref(),
            Some("RECHARGE : 2 TOUR(S)")
        );
        assert_eq!(
            app.visual_cues
                .sample_world(at, true, 10.16)
                .unwrap()
                .family,
            TerminalEffectFamily::Impulse
        );
        assert!(app.visual_cues.sample_world(at, false, 10.16).is_none());
        let damage: Vec<_> = app
            .floating_messages
            .iter()
            .filter(|m| m.tone == FloatingMessageTone::Damage)
            .collect();
        assert_eq!(damage.len(), 1);
        assert_eq!(damage[0].at, at);
        for _ in 0..2 {
            app.execute_command(GameCommand::Attack { slot: 0, target });
        }
        assert_eq!(app.game.actors().get(target).unwrap().position(), at);
        assert_eq!(
            app.percussion_readiness().as_deref(),
            Some("PERCUSSION PRÊTE")
        );
        app.game.drain_events();
        app.visual_cues.clear_world();
        app.floating_messages.clear();
        // Next cell is occupied by another mannequin: the push is blocked.
        app.execute_command(GameCommand::Attack { slot: 0, target });
        app.capture_events_at(Some(11.0));
        assert_eq!(
            app.visual_cues
                .sample_world(at, true, 11.16)
                .unwrap()
                .family,
            TerminalEffectFamily::ImpulseBlocked
        );
        assert!(app.floating_messages.iter().any(|m| m.text == "BLOQUÉ"));
        assert_eq!(
            app.percussion_readiness().as_deref(),
            Some("RECHARGE : 2 TOUR(S)")
        );
    }

    #[test]
    fn laboratory_weapons_execute_their_announced_effects() {
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        for prefix in ["blade", "rifle"] {
            for kind in LAB_KINDS {
                if prefix == "blade" && kind == "ricochet" {
                    continue;
                }
                for bonus in [false, true] {
                    app.reset_test_lab().unwrap();
                    let id = lab_id(&format!("{prefix}_{kind}"));
                    let item = app
                        .game
                        .player_inventory()
                        .iter()
                        .find(|entry| {
                            entry.item() == &id
                                && entry
                                    .magic_modifiers()
                                    .and_then(|bonus| bonus.named_affixes())
                                    .is_some()
                                    == bonus
                        })
                        .unwrap()
                        .instance();
                    if app.game.equipped_player_weapon_item(0) != Some(item) {
                        let equip = app.execute_command(GameCommand::EquipWeapon { slot: 0, item });
                        assert!(
                            matches!(
                                equip,
                                CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime
                            ),
                            "{equip:?}"
                        );
                    }
                    let target = app.selected_target.unwrap();
                    if kind == "catalysis" {
                        app.prime_lab_catalysis(target).unwrap();
                    }
                    app.game.drain_events();
                    assert_eq!(
                        app.execute_command(GameCommand::Attack { slot: 0, target }),
                        CommandOutcome::Applied,
                        "{prefix}/{kind}/{bonus}"
                    );
                    let events = app.game.events();
                    let centers = events
                        .iter()
                        .filter_map(|event| match event {
                            GameEvent::PropagationResolved { origin, .. } => Some(*origin),
                            _ => None,
                        })
                        .collect::<Vec<_>>();
                    match kind {
                        "ricochet" => assert_eq!(centers, vec![GridPos::new(8, 12)]),
                        "echo" => assert_eq!(app.game.pending_weapon_echoes().collect::<Vec<_>>(), vec![(GridPos::new(8, 12), 2)]),
                        "fracture" => {
                            assert!(centers.is_empty());
                            assert!(events.iter().any(|event| matches!(event, GameEvent::WeaponFractureResolved { target: affected, charges: 1, threshold: 3, .. } if *affected == target)));
                        },
                        "catalysis" => {
                            assert_eq!(centers, vec![GridPos::new(8, 12)]);
                            assert!(events.iter().any(|event| matches!(event, GameEvent::CatalyticExplosion { at, .. } if *at == GridPos::new(8, 12))));
                            assert!(app.game.actors().get(target).unwrap().status(&"core:burning".parse().unwrap()).is_some());
                        },
                        "percussion" => {
                            assert_eq!(app.game.actors().get(target).unwrap().position(), GridPos::new(9, 12));
                            assert!(events.iter().any(|event| matches!(event, GameEvent::WeaponImpulseResolved { target: affected, .. } if *affected == target)));
                            assert_eq!(app.percussion_readiness().as_deref(), Some("RECHARGE : 2 TOUR(S)"));
                        },
                        "aegis" => {
                            assert!(events.iter().any(|event| matches!(event, GameEvent::StatusApplied { target: affected, status, .. } if *affected == app.game.player_id() && status == &lab_id("impact_aegis"))));
                            assert_eq!(app.game.actors().get(app.game.player_id()).unwrap().status(&lab_id("impact_aegis")).unwrap().remaining_turns, Some(2));
                        },
                        "impact" | "conduction" => assert_eq!(centers, vec![GridPos::new(8, 12)]),
                        "piercing" => {
                            assert_eq!(centers, vec![GridPos::new(8, 12)]);
                            for x in [10, 11] {
                                let recipient = app.game.actors().entity_at(GridPos::new(x, 12)).unwrap();
                                assert_eq!(app.game.actors().get(recipient).unwrap().integrity(), 117);
                            }
                        },
                        "bearer" => assert_eq!(centers, vec![LAB_START]),
                        "burning" => assert!(events.iter().any(|event| matches!(event, GameEvent::StatusApplied { target: affected, .. } if *affected == target))),
                        "caustic" => assert!(events.iter().any(|event| matches!(event, GameEvent::GroundEffectCreated { at, .. } if *at == GridPos::new(8, 12)))),
                        "healing" => {
                            let damage: u16 = events.iter().filter_map(|event| match event {
                                GameEvent::DamageApplied { source, target: victim, amount, .. }
                                | GameEvent::DamageImpactApplied { source, target: victim, amount, .. }
                                    if *source == Some(app.game.player_id()) && *victim == target => Some(*amount),
                                _ => None,
                            }).sum();
                            let expected = (damage / 2).min(3);
                            assert!(expected > 0);
                            assert!(events.iter().any(|event| matches!(event, GameEvent::IntegrityRestored { entity, amount } if *entity == app.game.player_id() && *amount == expected)));
                        },
                        _ => assert!(centers.is_empty()),
                    }
                }
            }
        }
    }

    #[test]
    fn fracture_lab_counter_survives_weapon_swap_and_selection_then_bursts_once() {
        use crate::terminal_view::TerminalEffectBadge;
        use crate::visual_effects::TerminalEffectFamily;
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        let target = app.selected_target.unwrap();
        let at = app.game.actors().get(target).unwrap().position();
        for (charges, profile, bonus) in
            [(1, "blade", false), (2, "rifle", true), (3, "blade", true)]
        {
            let item = app
                .game
                .player_inventory()
                .iter()
                .find(|entry| {
                    entry.item() == &lab_id(&format!("{profile}_fracture"))
                        && entry
                            .magic_modifiers()
                            .and_then(|bonus| bonus.named_affixes())
                            .is_some()
                            == bonus
                })
                .unwrap()
                .instance();
            app.execute_command(GameCommand::EquipWeapon { slot: 0, item });
            app.game.drain_events();
            app.visual_cues.clear_world();
            app.floating_messages.clear();
            let hp = app.game.actors().get(target).unwrap().integrity();
            assert_eq!(
                app.execute_command(GameCommand::Attack { slot: 0, target }),
                CommandOutcome::Applied
            );
            let loss = hp - app.game.actors().get(target).unwrap().integrity();
            app.capture_events_at(Some(10.0));
            if charges < 3 {
                assert_eq!(
                    app.fracture_readiness(),
                    Some(format!("MARQUAGE : {charges}/3"))
                );
                assert_eq!(
                    app.effect_badges_at(at),
                    [TerminalEffectBadge::Fracture {
                        charges,
                        threshold: 3
                    }]
                );
                assert!(
                    app.terminal_target_summary()
                        .unwrap()
                        .visible_state
                        .contains(&format!("MARQUAGE {charges}/3 4t"))
                );
                let before = suspension::fingerprint(&app.game);
                app.update_input(&key("Tab"));
                assert_eq!(suspension::fingerprint(&app.game), before);
                app.selected_target = Some(target);
                assert_eq!(
                    app.effect_badges_at(at),
                    [TerminalEffectBadge::Fracture {
                        charges,
                        threshold: 3
                    }]
                );
            } else {
                assert_eq!(app.fracture_readiness().as_deref(), Some("MARQUAGE : 0/3"));
                assert!(app.effect_badges_at(at).is_empty());
                assert_eq!(
                    app.visual_cues
                        .sample_world(at, true, 10.14)
                        .unwrap()
                        .family,
                    TerminalEffectFamily::Fracture
                );
                assert!(app.visual_cues.sample_world(at, false, 10.14).is_none());
                assert!(app.visual_cues.sample_world(at, true, 30.0).is_none());
                let still = app
                    .visual_cues
                    .sample_world_with_motion(at, true, 10.14, true)
                    .unwrap();
                assert_eq!(still.family, TerminalEffectFamily::Fracture);
                assert_eq!(
                    still,
                    app.visual_cues
                        .sample_world_with_motion(at, true, 10.20, true)
                        .unwrap()
                );
            }
            let totals: Vec<_> = app
                .floating_messages
                .iter()
                .filter(|msg| msg.at == at && msg.tone == FloatingMessageTone::Damage)
                .collect();
            assert_eq!(totals.len(), 1);
            assert!(totals[0].text.contains(&loss.to_string()));
        }
        // Reapply one charge, then expire it without any burst.
        app.execute_command(GameCommand::Attack { slot: 0, target });
        for _ in 0..4 {
            app.execute_command(GameCommand::Wait);
        }
        assert!(app.effect_badges_at(at).is_empty());
        assert!(app.effect_badges_at(GridPos::new(34, 23)).is_empty());
        app.selected_target = None;
        assert_eq!(
            app.fracture_readiness().as_deref(),
            Some("MARQUAGE : CHOISIR UNE CIBLE")
        );
    }

    #[test]
    fn ricochet_lab_hits_a_second_target_without_memory_or_extra_damage_labels() {
        use crate::visual_effects::TerminalEffectFamily;
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        let item = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &lab_id("rifle_ricochet"))
            .unwrap()
            .instance();
        app.execute_command(GameCommand::EquipWeapon { slot: 0, item });
        let a = app.game.actors().entity_at(GridPos::new(8, 12)).unwrap();
        let b = app.game.actors().entity_at(GridPos::new(10, 12)).unwrap();
        app.game.drain_events();
        app.floating_messages.clear();
        app.visual_cues.clear_world();
        let before = app.game.actors().get(b).unwrap().integrity();
        app.execute_command(GameCommand::Attack { slot: 0, target: a });
        assert_eq!(app.game.actors().get(b).unwrap().integrity(), before - 3);
        assert!(
            app.game
                .alternation_previous(app.game.player_id())
                .is_none()
        );
        assert!(app.effect_badges_at(GridPos::new(8, 12)).is_empty());
        app.capture_events_at(Some(10.0));
        assert_eq!(
            app.floating_messages
                .iter()
                .filter(|m| matches!(m.tone, FloatingMessageTone::Damage))
                .count(),
            2
        );
        let at = GridPos::new(9, 12);
        assert_eq!(
            app.visual_cues
                .sample_world(at, true, 10.15)
                .unwrap()
                .family,
            TerminalEffectFamily::Ricochet
        );
        assert!(app.visual_cues.sample_world(at, false, 10.15).is_none());
    }

    #[test]
    fn echo_lab_telegraph_and_badge_last_real_turns_then_resolve_without_ghost_marker() {
        use crate::terminal_view::TerminalEffectBadge;
        use crate::visual_effects::TerminalEffectFamily;
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        let item = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &lab_id("blade_echo"))
            .unwrap()
            .instance();
        app.execute_command(GameCommand::EquipWeapon { slot: 0, item });
        let at = GridPos::new(8, 12);
        let target = app.game.actors().entity_at(at).unwrap();
        app.execute_command(GameCommand::Attack { slot: 0, target });
        app.capture_events_at(Some(1.0));
        assert_eq!(
            app.effect_badges_at(at),
            [TerminalEffectBadge::Echo { turns: 2 }]
        );
        assert!(
            app.terminal_target_summary()
                .unwrap()
                .visible_state
                .contains("ÉCHO DANS 2t")
        );
        let before = suspension::fingerprint(&app.game);
        for now in [1.0, 20.0, 800.0] {
            let sample = app.sustained_effects_at(at, now, false);
            assert_eq!(sample.len(), 1);
            assert_eq!(sample[0].family, TerminalEffectFamily::Echo);
            assert!(sample[0].sustained);
        }
        assert_eq!(
            app.sustained_effects_at(at, 1.0, true),
            app.sustained_effects_at(at, 500.0, true)
        );
        assert!(
            app.sustained_effects_at(GridPos::new(31, 21), 50.0, false)
                .is_empty()
        );
        app.update_input(&key("Tab"));
        assert_eq!(suspension::fingerprint(&app.game), before);
        app.execute_command(GameCommand::Wait);
        assert_eq!(
            app.effect_badges_at(at),
            [TerminalEffectBadge::Echo { turns: 1 }]
        );
        app.capture_events_at(Some(2.0));
        app.visual_cues.clear_world();
        app.floating_messages.clear();
        let hp = app.game.actors().get(target).unwrap().integrity();
        app.execute_command(GameCommand::Wait);
        assert_eq!(app.game.actors().get(target).unwrap().integrity(), hp - 2);
        app.capture_events_at(Some(10.0));
        assert!(app.sustained_effects_at(at, 10.0, false).is_empty());
        assert!(app.effect_badges_at(at).is_empty());
        assert_eq!(
            app.visual_cues.sample_world(at, true, 10.2).unwrap().family,
            TerminalEffectFamily::Echo
        );
        assert!(app.visual_cues.sample_world(at, true, 20.0).is_none());
        assert_eq!(
            app.floating_messages
                .iter()
                .filter(|message| matches!(message.tone, FloatingMessageTone::Damage))
                .count(),
            1
        );
    }

    #[test]
    fn reset_and_return_restore_their_own_state_without_nesting_labs() {
        let mut app = main_menu();
        let normal = suspension::fingerprint(&app.game);
        let rules = suspension::fingerprint(&app.rules);
        app.enter_test_lab().unwrap();
        let initial = suspension::fingerprint(&app.game);
        let first_seed = app.lab_bonus_seed;
        let initial_actors = suspension::fingerprint(app.game.actors());
        let target = app.selected_target.unwrap();
        app.execute_command(GameCommand::Attack { slot: 0, target });
        assert_ne!(suspension::fingerprint(&app.game), initial);
        app.open_menu(MenuScreen::Pause);
        assert_eq!(app.menu_labels()[2], "Réinitialiser le laboratoire");
        app.update_input(&click(MenuScreen::Pause, 2));
        assert_ne!(suspension::fingerprint(&app.game), initial);
        assert_eq!(app.lab_bonus_seed, first_seed.wrapping_add(1));
        assert_eq!(suspension::fingerprint(app.game.actors()), initial_actors);
        assert!(app.lab_return.as_ref().unwrap().lab_return.is_none());
        app.execute_command(GameCommand::Wait);
        app.update_input(&key("R"));
        assert!(app.test_lab);
        assert_eq!(app.lab_bonus_seed, first_seed.wrapping_add(2));
        assert_eq!(suspension::fingerprint(app.game.actors()), initial_actors);
        app.open_menu(MenuScreen::Pause);
        app.menu_selection = 3;
        app.update_input(&key("Enter"));
        assert!(!app.test_lab);
        assert_eq!(app.menu, MenuScreen::Main);
        assert_eq!(suspension::fingerprint(&app.game), normal);
        assert_eq!(suspension::fingerprint(&app.rules), rules);
        app.enter_test_lab().unwrap();
        app.leave_test_lab();
        assert_eq!(suspension::fingerprint(&app.game), normal);
    }

    #[test]
    fn lab_cannot_write_or_clean_normal_suspension_or_recovery_files() {
        let mut app = main_menu();
        let paths = [
            app.suspension_path.clone(),
            app.crash_recovery_paths()[0].clone(),
            app.crash_recovery_paths()[1].clone(),
        ];
        for path in &paths {
            std::fs::write(path, b"normal run sentinel").unwrap();
        }
        app.crash_recovery_enabled = true;
        app.enter_test_lab().unwrap();
        assert!(app.suspension().is_err());
        assert!(app.suspend_run().is_err());
        assert!(app.write_crash_recovery_checkpoint().is_err());
        app.remove_crash_recovery_files_except(None).unwrap();
        app.reset_test_lab().unwrap();
        app.request_quit();
        assert!(app.should_quit());
        app.leave_test_lab();
        assert!(app.crash_recovery_enabled);
        assert!(!app.should_quit());
        for path in &paths {
            assert_eq!(std::fs::read(path).unwrap(), b"normal run sentinel");
            std::fs::remove_file(path).unwrap();
        }
    }

    #[test]
    fn pointer_target_toggle_and_switch_never_attack_or_advance_time() {
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        app.selected_target = None;
        let first = GridPos::new(8, 12);
        let second = GridPos::new(10, 12);
        let before = suspension::fingerprint(&app.game);
        let history = app.history.len();
        for (at, selected) in [
            (first, true),
            (first, false),
            (second, true),
            (first, true),
            (first, false),
        ] {
            assert!(app.toggle_pointer_target_at(at));
            assert_eq!(
                app.selected_target,
                if selected {
                    app.game.actors().entity_at(at)
                } else {
                    None
                }
            );
            assert_eq!(app.is_selected_target_at(at), selected);
            assert_eq!(app.terminal_target_summary().is_some(), selected);
            app.update_input(&InputFrame::default());
            assert_eq!(app.selected_target.is_some(), selected);
            assert!(app.attack_aim.is_none());
            assert_eq!(suspension::fingerprint(&app.game), before);
            assert_eq!(app.history.len(), history);
        }
    }

    #[test]
    fn pointer_target_ignores_empty_hidden_and_player_cells() {
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        let selected = app.selected_target;
        let before = suspension::fingerprint(&app.game);
        let hidden = app
            .game
            .actors()
            .iter()
            .map(|(_, actor)| actor.position())
            .find(|at| !app.game.player_visibility().is_visible(*at))
            .unwrap();
        for at in [GridPos::new(8, 11), LAB_START, hidden, GridPos::new(-1, -1)] {
            assert!(!app.toggle_pointer_target_at(at));
            assert_eq!(app.selected_target, selected);
        }
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn pointer_target_selection_drives_the_next_direct_ranged_attack() {
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        let item = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &lab_id("rifle_reference"))
            .unwrap()
            .instance();
        assert!(matches!(
            app.execute_command(GameCommand::EquipWeapon { slot: 0, item }),
            CommandOutcome::Applied | CommandOutcome::AppliedWithoutTime
        ));
        app.controls
            .rebind(Action::Attack, Binding::key("V"))
            .unwrap();
        let at = GridPos::new(10, 12);
        let target = app.game.actors().entity_at(at).unwrap();
        let original = app.game.actors().entity_at(GridPos::new(8, 12)).unwrap();
        let original_health = app.game.actors().get(original).unwrap().integrity();
        let health = app.game.actors().get(target).unwrap().integrity();
        let turn = app.game.turn();
        assert!(app.toggle_pointer_target_at(at));
        app.update_input(&key("V"));
        assert!(app.attack_aim.is_none());
        assert_eq!(app.game.turn(), turn + 1);
        assert!(app.game.actors().get(target).unwrap().integrity() < health);
        assert_eq!(
            app.game.actors().get(original).unwrap().integrity(),
            original_health
        );
        assert_eq!(
            app.history.last(),
            Some(&RecordedCommand::record(&GameCommand::Attack {
                slot: 0,
                target
            }))
        );
    }

    #[test]
    fn single_target_attack_is_direct_and_tab_still_changes_selection() {
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        let first = app.selected_target.unwrap();
        let before = suspension::fingerprint(&app.game);
        app.update_input(&key("Tab"));
        assert_ne!(app.selected_target, Some(first));
        assert!(app.attack_aim.is_none());
        assert_eq!(suspension::fingerprint(&app.game), before);
        app.selected_target = Some(first);
        app.update_input(&key("F"));
        assert!(app.attack_aim.is_none());
        assert_eq!(app.game.turn(), 1);
        assert_eq!(
            app.history.last(),
            Some(&RecordedCommand::record(&GameCommand::Attack {
                slot: 0,
                target: first
            }))
        );
    }

    #[test]
    fn catalytic_weapon_opens_preview_before_keyboard_confirmation_and_allows_cancel() {
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        let item = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &lab_id("blade_catalysis"))
            .unwrap()
            .instance();
        app.execute_command(GameCommand::EquipWeapon { slot: 0, item });
        let before = suspension::fingerprint(&app.game);
        let turn = app.game.turn();
        let history = app.history.len();
        app.update_input(&key("F"));
        let aim = app.attack_aim.expect("cone must preview before firing");
        assert!(app.aimed_attack_preview(aim).unwrap().cells().len() > 1);
        assert_eq!(suspension::fingerprint(&app.game), before);
        assert_eq!(app.history.len(), history);
        app.update_input(&key("Escape"));
        assert!(app.attack_aim.is_none());
        assert_eq!(suspension::fingerprint(&app.game), before);
        app.update_input(&key("F"));
        app.update_input(&key("F"));
        assert!(app.attack_aim.is_none());
        assert_eq!(app.game.turn(), turn + 1);
        assert_eq!(
            app.history.last(),
            Some(&RecordedCommand::record(&GameCommand::AttackAt {
                slot: 0,
                target: aim.cursor
            }))
        );
    }

    #[test]
    fn catalytic_pointer_aim_can_preview_empty_ground_but_cannot_attack_it() {
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        let item = app
            .game
            .player_inventory()
            .iter()
            .find(|entry| entry.item() == &lab_id("blade_catalysis"))
            .unwrap()
            .instance();
        app.execute_command(GameCommand::EquipWeapon { slot: 0, item });
        let before = suspension::fingerprint(&app.game);
        let at = GridPos::new(8, 11);
        assert!(app.begin_pointer_attack_aim(at, None));
        let aim = app.attack_aim.unwrap();
        assert!(app.aimed_attack_footprint(aim).unwrap().cells().len() > 1);
        assert_eq!(
            app.aimed_attack_preview(aim),
            Err(CommandRejection::AttackTargetHasNoActor(at))
        );
        app.update_input(&key("F"));
        assert!(app.attack_aim.is_some());
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn single_target_weapon_does_not_open_free_tile_aim() {
        let mut app = main_menu();
        app.enter_test_lab().unwrap();
        let before = suspension::fingerprint(&app.game);
        for at in [GridPos::new(8, 11), GridPos::new(8, 12)] {
            assert!(!app.begin_pointer_attack_aim(at, Some((400.0, 300.0))));
            assert!(app.attack_aim.is_none());
        }
        assert_eq!(suspension::fingerprint(&app.game), before);
    }

    #[test]
    fn starting_bonus_builder_rejects_unknown_equipment_and_late_injection() {
        let (rules, _, _, _) = ascii_game_content().unwrap();
        let build = || {
            GameState::new_with_rules(
                Map::filled(5, 5, Terrain::Floor).unwrap(),
                GridPos::new(2, 2),
                3,
                rules.clone(),
            )
            .unwrap()
        };
        let modifier = MagicItemModifiers::mass_reduction_only(25).unwrap();
        assert!(
            build()
                .with_starting_magic_equipment([(lab_id("unknown"), modifier.clone())])
                .is_err()
        );
        let mut game = build();
        assert_eq!(
            game.process_player_command(GameCommand::Wait),
            CommandOutcome::Applied
        );
        assert!(
            game.with_starting_magic_equipment([(
                "core:integrity_blade".parse().unwrap(),
                modifier
            )])
            .is_err()
        );
    }
}
