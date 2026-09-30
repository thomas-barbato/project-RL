use crate::ai::AiBehavior;
use crate::content::EncounterBalance;
use crate::entity::Actor;
use crate::game::GameRng;
use crate::social::PlayerRelation;

/// Called on newly generated actors only. A persisted marker makes accidental
/// repeated application a no-op, including after snapshot restoration.
/// No player, time or shared placement RNG enters this calculation.
pub fn balance_layer_encounters(
    actors: &mut [Actor],
    profile: &EncounterBalance,
    depth: u16,
    reference_depth: u16,
    region_seed: u64,
) {
    if !profile.validate(
        profile
            .layers
            .len()
            .saturating_sub(1)
            .min(u16::MAX as usize) as u16,
    ) {
        return;
    }
    let Some(layer) = profile.layers.get(usize::from(depth)) else {
        return;
    };
    let Some(reference) = profile.layers.get(usize::from(reference_depth.min(depth))) else {
        return;
    };
    for actor in actors {
        if actor
            .tags()
            .iter()
            .any(|tag| tag.as_str().starts_with("core:encounter_level_"))
            || actor.drone().is_some()
            || actor.integrity() != actor.maximum_integrity()
            || actor.ai().is_none()
            || (actor.player_relation() != PlayerRelation::Hostile
                && !actor
                    .ai()
                    .is_some_and(|ai| ai.behavior == AiBehavior::Riveter)
                && !actor
                    .tags()
                    .iter()
                    .any(|tag| tag.as_str().starts_with("core:fauna_level_")))
        {
            continue;
        }
        let at = actor.position();
        let mut rng = GameRng::from_seed(
            region_seed
                ^ 0x4c41_5945_5250_4f57
                ^ (u64::from(at.x as u32) << 32)
                ^ u64::from(at.y as u32),
        );
        let heavy = matches!(
            actor.ai().unwrap().behavior,
            AiBehavior::TelegraphedBiter
                | AiBehavior::TelegraphedSweeper
                | AiBehavior::TelegraphedGrasper
                | AiBehavior::TelegraphedResonator
                | AiBehavior::TelegraphedProjector
        );
        let low = layer.levels[0]
            + if heavy {
                (layer.levels[1] - layer.levels[0]) / 2
            } else {
                0
            };
        let level = low + (rng.next_u64() % u64::from(layer.levels[1] - low + 1)) as u16;
        let variation =
            ((level - layer.levels[0]) * 10 / (layer.levels[1] - layer.levels[0]).max(1)) as u32;
        // Species are authored for their biome's first layer. Keep their relative
        // toughness rather than replacing every body by a universal HP total.
        let vitality = (u32::from(layer.vitality) * (95 + variation)
            / u32::from(reference.vitality))
        .clamp(90, 600) as u16;
        let carries_weapon = actor.equipped_weapon().is_some()
            || actor.tags().iter().any(|tag| {
                matches!(
                    tag.as_str(),
                    "core:humanoid_rifle_carrier" | "core:humanoid_knife_carrier"
                )
            });
        let damage = if carries_weapon {
            100
        } else {
            (u32::from(layer.innate_damage) * (100 + variation)
                / u32::from(reference.innate_damage))
            .clamp(100, 400) as u16
        };
        actor.scale_spawned_combat(vitality, damage);
        if let Some(mut reward) = actor.defeat_reward() {
            reward.threat_level = level;
            reward.base_experience =
                reward.base_experience.saturating_mul(u64::from(vitality)) / 100;
            *actor = actor.clone().with_defeat_reward(reward);
        }
        *actor = actor
            .clone()
            .with_tags([format!("core:encounter_level_{level}").parse().unwrap()]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::AiProfile;
    use crate::combat::{AttackProfile, DamageType};
    use crate::content::{ContentLoader, EncounterLayer};
    use crate::game::{GameRules, GameState};
    use crate::progression::DefeatReward;
    use crate::stats::BodyProfile;
    use crate::world::{GridPos, Map};

    fn profile() -> EncounterBalance {
        let loaded = ContentLoader::load(
            &[std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("content")],
            &semver::Version::new(0, 1, 0),
        )
        .unwrap();
        loaded
            .regional_worlds()
            .get(&"core:simulation_overworld".parse().unwrap())
            .unwrap()
            .encounter_balance()
            .unwrap()
            .clone()
    }

    fn specimen() -> Actor {
        Actor::new(GridPos::new(3, 2), 14)
            .unwrap()
            .with_body_profile(BodyProfile::new(14, 0).unwrap().with_base_armor(2))
            .with_ai(AiProfile::new(AiBehavior::TelegraphedSweeper, 7, 0, 512, 0))
            .with_attack(AttackProfile::melee(DamageType::Kinetic, 4))
            .with_player_relation(PlayerRelation::Hostile)
            .with_defeat_reward(DefeatReward::persistent(10, 5))
    }

    #[test]
    fn layer_balance_grows_real_bodies_damage_and_levels_without_changing_roles() {
        let profile = profile();
        for seed in 0..64 {
            let mut last = (0, 0, 0);
            for depth in 1..=7 {
                let original = specimen();
                let mut actors = [original.clone()];
                balance_layer_encounters(&mut actors, &profile, depth, 1, seed);
                let actor = &actors[0];
                let level = actor.defeat_reward().unwrap().threat_level;
                assert!(
                    (profile.layers[depth as usize].levels[0]
                        ..=profile.layers[depth as usize].levels[1])
                        .contains(&level)
                );
                assert!(actor.maximum_integrity() > last.0);
                assert!(actor.attack(0).unwrap().damage_output_percentage() >= last.1);
                assert!(level > last.2);
                assert_eq!(actor.body_profile().unwrap().base_armor, 2);
                assert_eq!(actor.ai(), original.ai());
                assert_eq!(actor.position(), original.position());
                assert_eq!(
                    actor.attack(0).unwrap().range(),
                    original.attack(0).unwrap().range()
                );
                assert_eq!(
                    actor.attack(0).unwrap().recovery_after_attack(),
                    original.attack(0).unwrap().recovery_after_attack()
                );
                last = (
                    actor.maximum_integrity(),
                    actor.attack(0).unwrap().damage_output_percentage(),
                    level,
                );
            }
        }
    }

    #[test]
    fn layer_balance_variation_is_bounded_persistent_and_independent_of_iteration_order() {
        let profile = profile();
        let mut levels = std::collections::BTreeSet::new();
        for seed in 0..64 {
            let mut actors = [specimen()];
            balance_layer_encounters(&mut actors, &profile, 3, 1, seed);
            levels.insert(actors[0].defeat_reward().unwrap().threat_level);
            let bytes = bincode::serialize(&actors[0]).unwrap();
            let mut restored = [bincode::deserialize(&bytes).unwrap()];
            balance_layer_encounters(&mut restored, &profile, 7, 0, seed + 1);
            assert_eq!(restored, actors);
        }
        assert!(levels.len() > 1);
        let other = Actor::new(GridPos::new(4, 2), 14)
            .unwrap()
            .with_ai(AiProfile::sentry(8, 0))
            .with_player_relation(PlayerRelation::Hostile);
        let mut first = [specimen(), other.clone()];
        let mut reverse = [other, specimen()];
        balance_layer_encounters(&mut first, &profile, 3, 1, 42);
        balance_layer_encounters(&mut reverse, &profile, 3, 1, 42);
        assert_eq!(first[0], reverse[1]);
        assert_eq!(first[1], reverse[0]);
    }

    #[test]
    fn layer_balance_spares_civilians_weapon_damage_and_renewable_rewards() {
        let profile = profile();
        let civilian = specimen().with_player_relation(PlayerRelation::Neutral);
        let armed = specimen().with_tags(["core:humanoid_rifle_carrier".parse().unwrap()]);
        let renewable = specimen().with_defeat_reward(DefeatReward::summoned(0, 0));
        let mobile = specimen()
            .with_body_profile(BodyProfile::new(10, 0).unwrap())
            .with_ai(AiProfile::new(AiBehavior::Skirmisher, 8, 0, 512, 2));
        let mut actors = [civilian.clone(), armed.clone(), renewable, mobile];
        balance_layer_encounters(&mut actors, &profile, 5, 1, 42);
        assert_eq!(actors[0], civilian);
        assert_eq!(actors[1].attack(0), armed.attack(0));
        assert!(actors[1].maximum_integrity() > armed.maximum_integrity());
        assert_eq!(actors[2].defeat_reward().unwrap().base_experience, 0);
        assert_eq!(actors[3].body_profile().unwrap().base_armor, 0);
    }

    #[test]
    fn layer_balance_survives_body_initialization_and_changes_resolved_damage() {
        let profile = profile();
        let mut actors = [specimen()];
        balance_layer_encounters(&mut actors, &profile, 4, 1, 42);
        let expected_hp = actors[0].maximum_integrity();
        let map = Map::from_ascii("#######\n#.....#\n#.....#\n#######").unwrap();
        let mut game = GameState::new_with_rules(
            map,
            GridPos::new(2, 2),
            1,
            GameRules {
                physical_rules: Some(Default::default()),
                ..GameRules::default()
            },
        )
        .unwrap();
        let id = game.spawn_actor(actors[0].clone()).unwrap();
        let actor = game.actors().get(id).unwrap();
        assert_eq!(actor.maximum_integrity(), expected_hp);
        assert_eq!(actor.integrity(), expected_hp);
        assert!(
            game.resolved_attack_damage(id, actor.attack(0).unwrap())
                .unwrap()
                .raw_total()
                > 4
        );
    }

    #[test]
    fn layer_balance_content_rejects_missing_bands_invalid_bounds_and_regressions() {
        let mut profile = profile();
        assert!(profile.validate(7));
        assert!(!profile.validate(6));
        let original = profile.layers[2].clone();
        for bad in [
            EncounterLayer {
                levels: [0, 11],
                ..original.clone()
            },
            EncounterLayer {
                levels: [12, 8],
                ..original.clone()
            },
            EncounterLayer {
                vitality: 0,
                ..original.clone()
            },
            EncounterLayer {
                innate_damage: 401,
                ..original.clone()
            },
            EncounterLayer {
                levels: [4, 7],
                ..original.clone()
            },
        ] {
            profile.layers[2] = bad;
            assert!(!profile.validate(7));
        }
        assert!(json5::from_str::<EncounterBalance>("{ layers: [], player_level: 8 }").is_err());
    }
}
