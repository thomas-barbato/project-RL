//! Disposable base-rank examples. These constructors own a separate rules/state
//! instance; none of their setup conditions or resources alter a campaign.
use super::*;
use crate::skills::agent;

pub struct AgentSkillDemo {
    pub game: GameState,
    pub target: GridPos,
    pub weapon_slot: u8,
    pub actions: Vec<GameCommand>,
    pub bounds: (i32, i32, i32, i32),
}

impl AgentSkillDemo {
    pub fn new(mut rules: GameRules, id: &str) -> Result<Self, String> {
        let definition = agent::definition(id)
            .filter(|s| s.kind == "active")
            .ok_or_else(|| format!("Not an active skill: {id}"))?;
        rules
            .agent_class_start
            .as_mut()
            .ok_or("Missing agent rules")?
            .skills = agent::definitions()
            .iter()
            .filter(|s| s.kind == "active")
            .map(|s| s.id.clone())
            .collect();
        rules.equipment_agent_skills.clear();
        rules.player_energy_capacity = 1000;
        rules.player_starting_energy = 1000;
        rules.player_energy_regeneration = 0;
        rules.player_maximum_integrity = 100;
        rules.player_hit_points_per_level = 0;
        rules.player_companion_limit = Some(5);
        rules.player_companion_behaviors = true;
        rules.maintained_energy_reservations = true;
        rules.monster_equipment_loot = None;
        rules.player_weapon_slots = super::super::WEAPON_HAND_SLOTS
            .map(|s| s.parse().unwrap())
            .into();
        let heavy = id.starts_with("HL");
        let weapon: crate::weapon::WeaponId = if heavy {
            rules
                .weapons
                .iter()
                .find(|(_, w)| w.launcher().is_some())
                .map(|(id, _)| id.clone())
                .ok_or("Missing heavy demo weapon")?
        } else {
            "core:modele_cogneur_p01".parse().unwrap()
        };
        let blade: crate::weapon::WeaponId = "core:integrity_blade".parse().unwrap();
        rules.player_starting_weapons = vec![blade.clone(), weapon.clone()];
        rules.player_starting_equipment = vec![Some(blade), None, Some(weapon)];
        rules.player_inventory_capacity = 256;
        if let Some(item) = rules.weapon_matter_item.clone() {
            rules
                .player_starting_items
                .push(super::super::StartingItemStack::new(item, 1000));
        }
        let ranged = heavy || id.starts_with("DI") || id == "T01";
        let slot = if ranged { 2 } else { 0 };
        let origin = GridPos::new(4, 5);
        let map =
            Map::from_ascii(&vec![".".repeat(35); 15].join("\n")).map_err(|e| e.to_string())?;
        let mut game =
            GameState::new_with_rules(map, origin, 724, rules).map_err(|e| e.to_string())?;
        let damage = game.actors.get(game.player).unwrap().maximum_integrity() / 3;
        game.actors
            .get_mut(game.player)
            .unwrap()
            .apply_damage(damage);
        let melee = id.starts_with("ME") || matches!(id, "B01" | "BM01" | "BM02" | "OC07");
        let defence = matches!(id, "B02" | "SU03" | "BM05");
        let utility = matches!(id, "DR01" | "HY04" | "HY07");
        let summon = matches!(id, "D56" | "D57" | "D58" | "D59" | "D66");
        let form = matches!(id, "M01" | "TR61" | "TR63" | "TR64" | "TR65");
        let mut target = GridPos::new(if melee || defence || form { 5 } else { 6 }, 5);
        if matches!(id, "FE02" | "FE03" | "FR03" | "SM00") || summon {
            target = GridPos::new(5, 5);
        }
        if matches!(id, "ES03" | "T02" | "ES05") {
            target = GridPos::new(6, 5);
        }
        if matches!(
            id,
            "B02" | "SU03" | "SU04" | "SU05" | "BM01" | "BM02" | "BM05" | "SF00"
        ) || form
        {
            target = origin;
        }
        if id == "ME05" {
            target = GridPos::new(7, 5);
        }
        let no_enemies = matches!(
            id,
            "SU04" | "SU05" | "D56" | "D57" | "DR01" | "HY04" | "ES03" | "T02" | "SF00"
        );
        let positions = if no_enemies {
            vec![]
        } else if id == "ES05" {
            vec![(5, 5)]
        } else if id == "ME05" {
            vec![(7, 5)]
        } else if id == "ME02" {
            vec![(5, 5), (5, 4), (5, 6)]
        } else if id == "ME06" {
            vec![(5, 5), (4, 4), (3, 5)]
        } else if id == "DI01" {
            vec![(6, 5), (7, 5), (8, 5)]
        } else if id == "OC07" {
            vec![(5, 5), (6, 5)]
        } else if matches!(id, "HY06" | "AF05") {
            vec![(6, 5), (7, 5)]
        } else if defence || melee || form {
            vec![(5, 5)]
        } else if id == "SM00" {
            vec![(7, 5)]
        } else if utility || summon {
            vec![(7, 5)]
        } else if matches!(
            id,
            "FE02"
                | "FE04"
                | "FE05"
                | "A02"
                | "FR02"
                | "FR04"
                | "FR05"
                | "FR06"
                | "DI03"
                | "ME06"
                | "SG00"
        ) || heavy
        {
            vec![(6, 5), (6, 4), (6, 6)]
        } else {
            vec![(6, 5)]
        };
        let mut victims = Vec::new();
        for (i, (x, y)) in positions.into_iter().enumerate() {
            let mut actor = Actor::new(
                GridPos::new(x, y),
                if id == "OC07" && i == 0 { 1 } else { 200 },
            )
            .unwrap()
            .with_evasion_disabled()
            .with_player_relation(crate::social::PlayerRelation::Hostile)
            .with_tags([format!(
                "core:monster_art_{}_0",
                if i % 2 == 0 { 164 } else { 165 }
            )
            .parse()
            .unwrap()]);
            if defence || matches!(id, "SM00" | "HY06" | "AF05") || id == "OC07" && i == 0 {
                actor = actor
                    .with_attack(AttackProfile::melee(
                        if id == "BM05" {
                            crate::combat::DamageType::Corruption
                        } else {
                            crate::combat::DamageType::Kinetic
                        },
                        2,
                    ))
                    .with_ai(crate::ai::AiProfile::hunter(8, 0));
            }
            victims.push(game.spawn_actor(actor).map_err(|e| e.to_string())?);
        }
        if id == "FE06" || id == "SU05" {
            let victim = if id == "SU05" {
                game.player
            } else {
                victims[0]
            };
            let (power, duration, _) = game.agent_spell_burn_profile("FE01", 1).unwrap();
            game.apply_agent_burn(victim, power, duration);
        }
        if id == "FR06" {
            for &victim in &victims {
                let status = "agent:frozen".parse().unwrap();
                game.actors.get_mut(victim).unwrap().apply_status(
                    game.rules.statuses.get(&status).unwrap(),
                    1,
                    Some(game.player),
                );
                game.agent_runtime_mut().magic.frozen.insert(
                    victim,
                    super::agent_magic_runtime::AgentFrozen { expires_at: 10 },
                );
            }
        }
        if utility {
            let at = GridPos::new(5, 5);
            let outcome = game.process_player_command(cast("D58", at, 0));
            if outcome != CommandOutcome::Applied {
                return Err(format!("Prepare {id}: {outcome:?}"));
            }
            let drone = game
                .actors
                .iter()
                .find(|(_, a)| a.drone().is_some())
                .map(|(e, _)| e)
                .ok_or("Missing demonstration drone")?;
            target = game.actors.get(drone).unwrap().position();
            if id == "DR01" {
                game.actors.get_mut(drone).unwrap().apply_damage(10);
            }
        }
        let mut actions = vec![cast(id, target, slot), GameCommand::Wait, GameCommand::Wait];
        if matches!(id, "BM01" | "BM02") {
            actions[1] = cast("B01", GridPos::new(5, 5), 0);
        }
        if id == "T01" {
            actions[1] = cast("DI04", target, slot);
        }
        if id == "OC07" {
            actions[1] = GameCommand::Attack {
                slot: 0,
                target: victims[0],
            };
        }
        if id == "SF00" {
            actions[1] = GameCommand::Move(Direction::East);
        }
        if matches!(id, "D56" | "D57") {
            actions[1] = GameCommand::Move(Direction::West);
        }
        if form {
            let suffix = match id {
                "M01" => "m01_claw",
                "TR61" => "tr61_strike",
                "TR63" => "tr63_heurt",
                "TR64" => "tr64_bolt",
                _ => "tr65_reach",
            };
            actions[1] = GameCommand::UseTechniqueAt {
                technique: format!("agent:{suffix}").parse().unwrap(),
                target: GridPos::new(5, 5),
                weapon_slot: 0,
            };
        }
        let mut bounds = (3, 4, 7, 6);
        if let Ok(preview) =
            game.player_weapon_technique_footprint(&agent::technique(id), slot, target)
        {
            for cell in preview.cells() {
                bounds.0 = bounds.0.min(cell.position.x);
                bounds.1 = bounds.1.min(cell.position.y);
                bounds.2 = bounds.2.max(cell.position.x);
                bounds.3 = bounds.3.max(cell.position.y);
            }
        }
        for (_, actor) in game.actors.iter() {
            let at = actor.position();
            bounds.0 = bounds.0.min(at.x - 1);
            bounds.1 = bounds.1.min(at.y);
            bounds.2 = bounds.2.max(at.x + 1);
            bounds.3 = bounds.3.max(at.y);
        }
        let _ = definition;
        // A stable successful example, with the normal authored probabilities.
        // This generator belongs only to the disposable demonstration state.
        game.rng = GameRng::from_seed(2);
        game.drain_events();
        Ok(Self {
            game,
            target,
            weapon_slot: slot,
            actions,
            bounds,
        })
    }
}
fn cast(id: &str, target: GridPos, weapon_slot: u8) -> GameCommand {
    GameCommand::UseTechniqueAt {
        technique: agent::technique(id),
        target,
        weapon_slot,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn every_active_has_a_base_rank_example_with_valid_actions_and_no_passive_example() {
        let mut rules = super::super::agent_cast::tests::fixture().rules.clone();
        let content = crate::content::ContentLoader::load(
            &[std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content")],
            &semver::Version::new(0, 1, 0),
        )
        .unwrap();
        rules.skills = content.skills().clone();
        rules.stability_rules = Some(Default::default());
        let mut failures = Vec::new();
        let mut count = 0;
        for skill in agent::definitions() {
            if skill.kind != "active" {
                assert!(AgentSkillDemo::new(rules.clone(), &skill.id).is_err());
                continue;
            }
            count += 1;
            match AgentSkillDemo::new(rules.clone(), &skill.id) {
                Err(e) => failures.push(format!("{} setup: {e}", skill.id)),
                Ok(mut fixture) => {
                    assert_eq!(fixture.game.player_agent_skill_rank(&skill.id), 1);
                    for command in fixture.actions {
                        let result = fixture.game.process_player_command(command.clone());
                        if matches!(result, CommandOutcome::Rejected(_)) {
                            failures.push(format!("{} {command:?}: {result:?}", skill.id));
                            break;
                        }
                    }
                    if skill.id == "SM00" {
                        assert!(fixture.game.events.iter().any(|event| matches!(
                            event, GameEvent::AgentAreaSpellCast { skill, .. } if skill == "SM00"
                        )), "the example must detonate the mine");
                    }
                    if skill.id == "OC07" {
                        assert!(
                            fixture
                                .game
                                .actors
                                .iter()
                                .any(|(entity, actor)| entity != fixture.game.player
                                    && actor.player_relation()
                                        == crate::social::PlayerRelation::Allied),
                            "the example must raise an ally"
                        );
                    }
                }
            }
        }
        assert_eq!(count, 68);
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }
}
