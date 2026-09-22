use std::collections::BTreeSet;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::content::{RegionPopulationProfile, RegionPopulationRule};
use crate::entity::Actor;
use crate::game::GameRng;
use crate::world::{GridPos, Map};

const POPULATION_SEED_SALT: u64 = 0x504f_5055_4c41_544e;
const ENCOUNTER_SEED_SALT: u64 = 0x454e_434f_554e_5452;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RegionalPopulationFeatures {
    pub pursuit_lifecycle: bool,
    pub primary_attributes: bool,
    pub physical_profiles: bool,
    pub electronic_systems: bool,
    pub player_relations: bool,
}

/// Creates persistent actors only when a regional zone is materialized. A
/// separate random stream prevents population tuning from changing terrain.
pub fn generate_regional_population(
    map: &Map,
    passages: &[GridPos],
    profile: &RegionPopulationProfile,
    region_seed: u64,
    features: RegionalPopulationFeatures,
) -> Result<Vec<Actor>, RegionalPopulationError> {
    generate_population_layer(
        map,
        passages,
        &[],
        &BTreeSet::new(),
        profile,
        region_seed ^ POPULATION_SEED_SALT,
        features,
        false,
    )
}

/// Adds v21 encounter groups after historical population and landmarks have
/// been placed. The first groups defend distinct landmarks; surplus groups use
/// independent roaming anchors. Reserved cells make both layers coexist.
pub fn generate_regional_encounters(
    map: &Map,
    passages: &[GridPos],
    landmarks: &[GridPos],
    reserved: &BTreeSet<GridPos>,
    profile: &RegionPopulationProfile,
    region_seed: u64,
    features: RegionalPopulationFeatures,
) -> Result<Vec<Actor>, RegionalPopulationError> {
    generate_regional_encounters_with_roles(
        map,
        passages,
        landmarks,
        reserved,
        profile,
        region_seed,
        features,
        false,
    )
}

pub fn generate_regional_encounters_with_roles(
    map: &Map,
    passages: &[GridPos],
    landmarks: &[GridPos],
    reserved: &BTreeSet<GridPos>,
    profile: &RegionPopulationProfile,
    region_seed: u64,
    features: RegionalPopulationFeatures,
    distinct_roles: bool,
) -> Result<Vec<Actor>, RegionalPopulationError> {
    generate_population_layer(
        map,
        passages,
        landmarks,
        reserved,
        profile,
        region_seed ^ ENCOUNTER_SEED_SALT,
        features,
        distinct_roles,
    )
}

fn generate_population_layer(
    map: &Map,
    passages: &[GridPos],
    landmarks: &[GridPos],
    reserved: &BTreeSet<GridPos>,
    profile: &RegionPopulationProfile,
    seed: u64,
    features: RegionalPopulationFeatures,
    distinct_roles: bool,
) -> Result<Vec<Actor>, RegionalPopulationError> {
    if profile.is_empty() {
        return Ok(Vec::new());
    }
    let mut rng = GameRng::from_seed(seed);
    let roll_count = inclusive_u16(
        &mut rng,
        profile.minimum_group_rolls(),
        profile.maximum_group_rolls(),
    );
    let total_weight: u64 = profile
        .rules()
        .iter()
        .map(|rule| u64::from(rule.weight()))
        .sum();
    let mut actors = Vec::new();
    let mut occupied = reserved.clone();

    for group_index in 0..roll_count {
        let rule = if distinct_roles && usize::from(group_index) < profile.rules().len() {
            &profile.rules()[usize::from(group_index)]
        } else {
            weighted_rule(profile.rules(), total_weight, &mut rng)
        };
        let count = inclusive_u16(&mut rng, rule.minimum_count(), rule.maximum_count());
        let mut candidates = regional_spawn_cells(map, passages, rule, &occupied);
        if candidates.len() < usize::from(count) {
            return Err(RegionalPopulationError::InsufficientSpawnSpace {
                requested: count,
                available: candidates.len(),
                minimum_passage_distance: rule.minimum_passage_distance(),
            });
        }
        let anchor = landmarks
            .get(usize::from(group_index))
            .and_then(|landmark| {
                candidates.iter().copied().min_by_key(|position| {
                    (
                        manhattan_distance(*position, *landmark),
                        position.y,
                        position.x,
                    )
                })
            })
            .unwrap_or_else(|| candidates[below(&mut rng, candidates.len() as u64) as usize]);
        candidates.sort_by_key(|position| {
            (
                manhattan_distance(*position, anchor),
                position.y,
                position.x,
            )
        });
        for position in candidates.into_iter().take(usize::from(count)) {
            let ai = if features.pursuit_lifecycle {
                rule.ai()
            } else {
                rule.ai().without_pursuit_lifecycle()
            };
            let mut actor = Actor::new(position, rule.maximum_integrity())
                .expect("regional population validation rejects zero integrity")
                .with_attack(if features.physical_profiles {
                    rule.attack()
                } else {
                    rule.attack().without_melee_impact()
                })
                .with_ai(ai)
                .with_tags(rule.tags().iter().cloned());
            if features.player_relations {
                actor = actor.with_player_relation(rule.player_relation());
            }
            if features.primary_attributes
                && let Some(attributes) = rule.primary_attributes()
            {
                actor = actor.with_primary_attributes(attributes);
            }
            if features.physical_profiles
                && let Some(body) = rule.body_profile()
            {
                actor = actor.with_body_profile(body);
            }
            if features.physical_profiles && !rule.body_components().is_empty() {
                actor = actor.with_body_components(rule.body_components().iter().cloned());
            }
            if features.electronic_systems
                && let Some(profile) = rule.electronic_system()
            {
                actor = actor.with_electronic_system(profile);
            }
            if let Some(reward) = rule.defeat_reward() {
                actor = actor.with_defeat_reward(reward);
            }
            occupied.insert(position);
            actors.push(actor);
        }
    }
    Ok(actors)
}

fn regional_spawn_cells(
    map: &Map,
    passages: &[GridPos],
    rule: &RegionPopulationRule,
    occupied: &BTreeSet<GridPos>,
) -> Vec<GridPos> {
    (1..map.height() as i32 - 1)
        .flat_map(|y| (1..map.width() as i32 - 1).map(move |x| GridPos::new(x, y)))
        .filter(|position| {
            map.is_walkable(*position)
                && !map.is_protected(*position)
                && !occupied.contains(position)
                && !passages.contains(position)
                && passages.iter().all(|passage| {
                    manhattan_distance(*position, *passage)
                        >= u32::from(rule.minimum_passage_distance())
                })
        })
        .collect()
}

fn weighted_rule<'a>(
    rules: &'a [RegionPopulationRule],
    total_weight: u64,
    rng: &mut GameRng,
) -> &'a RegionPopulationRule {
    let mut ticket = below(rng, total_weight);
    rules
        .iter()
        .find(|rule| {
            if ticket < u64::from(rule.weight()) {
                true
            } else {
                ticket -= u64::from(rule.weight());
                false
            }
        })
        .expect("validated population rules have positive total weight")
}

fn inclusive_u16(rng: &mut GameRng, minimum: u16, maximum: u16) -> u16 {
    let span = u64::from(maximum - minimum) + 1;
    minimum + below(rng, span) as u16
}

// Rejection sampling avoids modulo bias while preserving the historical
// GameRng helpers used by older suspension versions.
fn below(rng: &mut GameRng, bound: u64) -> u64 {
    debug_assert!(bound > 0);
    let threshold = bound.wrapping_neg() % bound;
    loop {
        let value = rng.next_u64();
        if value >= threshold {
            return value % bound;
        }
    }
}

fn manhattan_distance(left: GridPos, right: GridPos) -> u32 {
    left.x.abs_diff(right.x) + left.y.abs_diff(right.y)
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RegionalPopulationError {
    InsufficientSpawnSpace {
        requested: u16,
        available: usize,
        minimum_passage_distance: u16,
    },
}

impl Display for RegionalPopulationError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InsufficientSpawnSpace {
                requested,
                available,
                minimum_passage_distance,
            } => write!(
                formatter,
                "regional population requested {requested} actors but only {available} cells remain at least {minimum_passage_distance} tiles from every passage"
            ),
        }
    }
}

impl Error for RegionalPopulationError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::AiProfile;
    use crate::combat::{AttackProfile, DamageType};
    use crate::content::{RegionMapSize, RegionTerrain, RegionTerrainProfile, RegionTerrainRule};
    use crate::progression::DefeatReward;
    use crate::stats::PrimaryAttributes;
    use crate::world::{Direction, DistanceMetric};

    use super::super::{RegionalMapGenerator, cardinal_passage};

    const FEATURES: RegionalPopulationFeatures = RegionalPopulationFeatures {
        pursuit_lifecycle: true,
        primary_attributes: true,
        physical_profiles: true,
        electronic_systems: true,
        player_relations: true,
    };

    fn profile() -> RegionPopulationProfile {
        RegionPopulationProfile::new(
            3,
            4,
            vec![
                RegionPopulationRule::new(
                    3,
                    1,
                    2,
                    7,
                    8,
                    AttackProfile::melee(DamageType::Kinetic, 2),
                    AiProfile::hunter(9, 0)
                        .with_maximum_pursuit_distance(std::num::NonZeroU16::new(11).unwrap()),
                    Some(DefeatReward::persistent(5, 1)),
                )
                .unwrap()
                .with_primary_attributes(PrimaryAttributes::new(6, 6, 5, 6, 4))
                .unwrap()
                .with_tags(vec!["test:quest_target".parse().unwrap()])
                .unwrap(),
                RegionPopulationRule::new(
                    1,
                    1,
                    1,
                    9,
                    10,
                    AttackProfile::new(
                        5,
                        DistanceMetric::Euclidean,
                        true,
                        DamageType::Piercing,
                        2,
                        0,
                    ),
                    AiProfile::skirmisher(10, 0, 3)
                        .with_maximum_pursuit_distance(std::num::NonZeroU16::new(12).unwrap()),
                    Some(DefeatReward::persistent(7, 2)),
                )
                .unwrap()
                .with_primary_attributes(PrimaryAttributes::new(4, 7, 4, 7, 6))
                .unwrap()
                .with_tags(vec!["test:quest_target".parse().unwrap()])
                .unwrap(),
            ],
        )
        .unwrap()
    }

    fn generated_map() -> (Map, [GridPos; 4]) {
        let size = RegionMapSize::new(48, 36).unwrap();
        let terrain = RegionTerrainProfile::new(
            RegionTerrain::Grass,
            8,
            2,
            5,
            vec![RegionTerrainRule::new(RegionTerrain::Tree, 1).unwrap()],
        )
        .unwrap();
        let generated = RegionalMapGenerator::new(size, &terrain)
            .generate(17)
            .unwrap();
        let passages = [
            cardinal_passage(size, Direction::North),
            cardinal_passage(size, Direction::East),
            cardinal_passage(size, Direction::South),
            cardinal_passage(size, Direction::West),
        ];
        let (map, _, _) = generated.into_parts();
        (map, passages)
    }

    #[test]
    fn equal_region_seeds_create_the_same_bounded_packs() {
        let (map, passages) = generated_map();
        let profile = profile();
        let first = generate_regional_population(&map, &passages, &profile, 91, FEATURES).unwrap();
        let second = generate_regional_population(&map, &passages, &profile, 91, FEATURES).unwrap();

        assert_eq!(first, second);
        assert!((3..=8).contains(&first.len()));
        assert_eq!(
            first
                .iter()
                .map(Actor::position)
                .collect::<BTreeSet<_>>()
                .len(),
            first.len()
        );
        assert!(first.iter().all(|actor| {
            map.is_walkable(actor.position())
                && !passages.contains(&actor.position())
                && actor.ai_home() == Some(actor.position())
                && actor.ai().unwrap().maximum_pursuit_distance().is_some()
                && actor.primary_attributes().is_some()
                && actor.tags().contains(&"test:quest_target".parse().unwrap())
        }));
    }

    #[test]
    fn population_uses_a_stream_independent_from_terrain_generation() {
        let (map, passages) = generated_map();
        let profile = profile();
        let first = generate_regional_population(&map, &passages, &profile, 1, FEATURES).unwrap();
        let second = generate_regional_population(&map, &passages, &profile, 2, FEATURES).unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn encounter_groups_guard_landmarks_without_overlapping_existing_content() {
        let (map, passages) = generated_map();
        let profile = profile();
        let landmarks = [GridPos::new(12, 12), GridPos::new(34, 24)];
        let reserved = BTreeSet::from([landmarks[0], landmarks[1], GridPos::new(20, 18)]);
        let first = generate_regional_encounters(
            &map, &passages, &landmarks, &reserved, &profile, 91, FEATURES,
        )
        .unwrap();
        let second = generate_regional_encounters(
            &map, &passages, &landmarks, &reserved, &profile, 91, FEATURES,
        )
        .unwrap();

        assert_eq!(first, second);
        assert!((3..=8).contains(&first.len()));
        assert!(
            first
                .iter()
                .all(|actor| !reserved.contains(&actor.position()))
        );
        assert!(landmarks.iter().all(|landmark| {
            first
                .iter()
                .any(|actor| manhattan_distance(actor.position(), *landmark) == 1)
        }));
    }

    #[test]
    fn an_empty_profile_creates_no_actor() {
        let (map, passages) = generated_map();
        assert!(
            generate_regional_population(
                &map,
                &passages,
                &RegionPopulationProfile::default(),
                3,
                FEATURES,
            )
            .unwrap()
            .is_empty()
        );
    }
}
