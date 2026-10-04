//! Probes use current authored actors and rules; no production balance changes.
use super::*;
use project_rl::world::{Map, generation::*};
use std::collections::BTreeSet;

#[test]
fn gameplay_review_authored_surface_melee_and_ranged_enemies_really_attack() {
    let (rules, _, _, _) = ascii_game_content().unwrap();
    let worlds = ascii_regional_world_catalog().unwrap();
    let world = worlds
        .get(&"core:simulation_overworld".parse().unwrap())
        .unwrap();
    let biome = world.biome(&"core:human_habitat".parse().unwrap()).unwrap();
    let map = Map::filled(64, 64, Terrain::Floor).unwrap();
    let mut specimens = BTreeMap::new();
    for seed in 0..32 {
        for actor in generate_regional_population(
            &map,
            &[],
            biome.population(),
            seed,
            RegionalPopulationFeatures {
                pursuit_lifecycle: true,
                primary_attributes: true,
                physical_profiles: true,
                electronic_systems: true,
                player_relations: true,
            },
        )
        .unwrap()
        {
            if let Some(attack) = actor.attack(0) {
                specimens
                    .entry(matches!(attack.delivery(), AttackDelivery::Ranged))
                    .or_insert(actor);
            }
        }
        if specimens.len() == 2 {
            break;
        }
    }
    assert_eq!(specimens.len(), 2, "both authored roles must be sampled");
    for (ranged, actor) in specimens {
        let at = actor.position();
        let distance = if ranged { 3 } else { 1 };
        let player_at = [
            Direction::West,
            Direction::East,
            Direction::North,
            Direction::South,
        ]
        .into_iter()
        .map(|direction| {
            let (dx, dy) = direction.delta();
            GridPos::new(at.x + dx * distance, at.y + dy * distance)
        })
        .find(|position| map.is_walkable(*position))
        .unwrap();
        let mut game =
            GameState::new_with_rules(map.clone(), player_at, 42, rules.clone()).unwrap();
        let attack = actor.attack(0).unwrap();
        let id = game.spawn_actor(actor).unwrap();
        let player = game.player_id();
        let hp = game.actors().get(player).unwrap().integrity();
        let mut attacks = 0;
        let mut hits = 0;
        let mut damage = 0;
        let mut absorbed = 0;
        for _ in 0..8 {
            if game.status() != RunStatus::Active {
                break;
            }
            assert_eq!(
                game.process_player_command(GameCommand::Wait),
                CommandOutcome::Applied
            );
            for event in game.drain_events() {
                match event {
                    GameEvent::AttackPerformed { attacker, .. } if attacker == id => attacks += 1,
                    GameEvent::AttackHitResolved {
                        attacker,
                        hit: true,
                        ..
                    } if attacker == id => hits += 1,
                    GameEvent::DamageApplied {
                        target,
                        amount,
                        absorbed_by_armor,
                        ..
                    } if target == player => {
                        damage += u32::from(amount);
                        absorbed += u32::from(absorbed_by_armor);
                    }
                    _ => {}
                }
            }
        }
        assert!(attacks > 0, "authored role must attack in its valid range");
        println!(
            "GAMEPLAY_REVIEW {}",
            serde_json::json!({
                "role": if ranged { "ranged" } else { "melee" },
                "scenario": "open-floor-starting-player-eight-waits",
                "range": attack.range(), "attack": format!("{attack:?}"),
                "attacks": attacks, "hits": hits, "applied_damage": damage,
                "absorbed_by_armor": absorbed,
                "starting_hp": hp,
                "remaining_hp": game.actors().get(player).map_or(0, Actor::integrity),
            })
        );
    }
}

#[test]
fn gameplay_review_ranged_damage_proposal_preserves_armor_as_a_counter() {
    use project_rl::combat::{
        ArmorProfile, DamageImpact, DamagePacket, ResistanceProfile,
        resolve_damage_impact_with_armor,
    };
    let (rules, _, _, _) = ascii_game_content().unwrap();
    for armor in [1, 2, 3] {
        let impact = |amount| {
            resolve_damage_impact_with_armor(
                DamageImpact::single(DamagePacket::new(amount, DamageType::Piercing, 0)),
                ResistanceProfile::default(),
                ArmorProfile::new(armor, 0, 0, 0),
                rules.damage,
                rules.armor_rules.unwrap(),
            )
            .amount()
        };
        let proposed = impact(3);
        assert_eq!(impact(1), 0);
        assert_eq!(proposed, 3 - armor);
        println!(
            "GAMEPLAY_REVIEW {}",
            serde_json::json!({"scenario":"proposal-only-ranged-base-damage-three","armor":armor,"current_damage_per_hit":impact(1),"proposed_damage_per_hit":proposed,"penetration":0})
        );
    }
}

#[test]
fn gameplay_review_relay_locations_and_alternative_separation() {
    let (rules, _, loot, expeditions) = ascii_game_content().unwrap();
    let definition = expeditions
        .get(&"core:starter_expedition".parse().unwrap())
        .unwrap();
    let narrative = definition.narrative.as_ref().unwrap();
    let mut placements = BTreeSet::new();
    for seed in INITIAL_SEED..INITIAL_SEED + 8 {
        let mut generated = crate::test_expedition::generate_destination(
            &rules,
            seed,
            Some(&loot),
            definition,
            crate::test_expedition::ExpeditionGenerationFeatures {
                defined_population: true,
                expanded_world: true,
                pursuit_limits: true,
                pursuit_lifecycle: true,
                primary_attributes: true,
                physical_profiles: true,
                electronic_systems: true,
                preparation_disruption: true,
                player_relations: true,
            },
        )
        .unwrap();
        let facility = crate::test_expedition::install_narrative_relay_with_service_route(
            &mut generated,
            narrative,
        )
        .unwrap();
        let terminal = facility
            .installations
            .iter()
            .find(|installation| installation.id.as_str() == "core:relay_register")
            .unwrap()
            .position;
        let contact = generated
            .blueprint
            .actors
            .iter()
            .find(|actor| actor.tags().contains(&narrative.relay_character))
            .unwrap()
            .position();
        placements.insert(terminal);
        println!(
            "GAMEPLAY_REVIEW {}",
            serde_json::json!({"seed":seed,"terminal":[terminal.x,terminal.y],"contact":[contact.x,contact.y],"separation":terminal.x.abs_diff(contact.x)+terminal.y.abs_diff(contact.y)})
        );
    }
    assert!(
        placements.len() > 1,
        "changing the seed must already change the relay position"
    );
}
