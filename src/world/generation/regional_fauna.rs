use super::RegionalPopulationFeatures;
use super::regional_population::{below, inclusive_u16, population_actor, regional_spawn_cells};
use crate::content::{RegionFaunaProfile, RegionTerrain};
use crate::entity::Actor;
use crate::game::GameRng;
use crate::world::{GridPos, Map, has_line_of_sight};
use std::collections::{BTreeMap, BTreeSet};

/// Family first, then species: adding species does not silently boost a family's
/// weight. Invalid habitats and unaffordable groups never enter either draw.
pub fn generate_regional_fauna(
    map: &Map,
    terrain: &BTreeMap<GridPos, RegionTerrain>,
    passages: &[GridPos],
    reserved: &BTreeSet<GridPos>,
    profile: &RegionFaunaProfile,
    region_seed: u64,
) -> Result<Vec<Actor>, String> {
    profile.validate()?;
    let mut rng = GameRng::from_seed(region_seed ^ 0x4641_554e_415f_5631);
    let count = inclusive_u16(&mut rng, profile.group_rolls[0], profile.group_rolls[1]);
    let mut budget = profile.danger_budget;
    let mut occupied = reserved.clone();
    let mut actors = Vec::new();
    let features = RegionalPopulationFeatures {
        pursuit_lifecycle: true,
        primary_attributes: true,
        physical_profiles: true,
        electronic_systems: true,
        player_relations: true,
    };
    for _ in 0..count {
        let eligible = profile
            .families
            .iter()
            .filter_map(|family| {
                let species = family
                    .species
                    .iter()
                    .filter_map(|kind| {
                        if !(profile.level_range[0]..=profile.level_range[1]).contains(&kind.level)
                            || kind.level.saturating_mul(kind.population.minimum_count()) > budget
                        {
                            return None;
                        }
                        let cells =
                            regional_spawn_cells(map, passages, &kind.population, &occupied)
                                .into_iter()
                                .filter(|cell| {
                                    terrain
                                        .get(cell)
                                        .is_some_and(|habitat| kind.habitats.contains(habitat))
                                })
                                .collect::<Vec<_>>();
                        (cells.len() >= usize::from(kind.population.minimum_count()))
                            .then_some((kind, cells))
                    })
                    .collect::<Vec<_>>();
                (!species.is_empty()).then_some((family, species))
            })
            .collect::<Vec<_>>();
        if eligible.is_empty() {
            break;
        }
        let mut ticket = below(
            &mut rng,
            eligible
                .iter()
                .map(|(family, _)| u64::from(family.weight))
                .sum(),
        );
        let (family, species) = eligible
            .iter()
            .find(|(family, _)| {
                if ticket < u64::from(family.weight) {
                    true
                } else {
                    ticket -= u64::from(family.weight);
                    false
                }
            })
            .unwrap();
        let mut ticket = below(
            &mut rng,
            species
                .iter()
                .map(|(kind, _)| u64::from(kind.population.weight()))
                .sum(),
        );
        let (kind, cells) = species
            .iter()
            .find(|(kind, _)| {
                let weight = u64::from(kind.population.weight());
                if ticket < weight {
                    true
                } else {
                    ticket -= weight;
                    false
                }
            })
            .unwrap();
        let anchor = cells[below(&mut rng, cells.len() as u64) as usize];
        let mut cluster = cells
            .iter()
            .copied()
            .filter(|at| {
                at.x.abs_diff(anchor.x) + at.y.abs_diff(anchor.y) <= 4
                    && has_line_of_sight(map, anchor, *at, true)
            })
            .collect::<Vec<_>>();
        cluster.sort_by_key(|at| (at.x.abs_diff(anchor.x) + at.y.abs_diff(anchor.y), *at));
        let maximum = kind
            .population
            .maximum_count()
            .min(budget / kind.level)
            .min(cluster.len() as u16);
        if maximum < kind.population.minimum_count() {
            continue;
        }
        let count = inclusive_u16(&mut rng, kind.population.minimum_count(), maximum);
        for at in cluster.into_iter().take(usize::from(count)) {
            let actor = population_actor(&kind.population, at, features).with_tags([
                kind.id.clone(),
                family.id.clone(),
                format!("core:fauna_level_{}", kind.level).parse().unwrap(),
            ]);
            occupied.insert(at);
            actors.push(actor);
        }
        budget -= count * kind.level;
    }
    Ok(actors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::AiProfile;
    use crate::combat::{AttackProfile, DamageType};
    use crate::content::{FaunaFamily, FaunaSpecies, RegionPopulationRule};

    fn fixture() -> (Map, BTreeMap<GridPos, RegionTerrain>, RegionFaunaProfile) {
        let row = format!("#{}#", ".".repeat(38));
        let source = std::iter::once("#".repeat(40))
            .chain(std::iter::repeat_n(row, 18))
            .chain(["#".repeat(40)])
            .collect::<Vec<_>>()
            .join("\n");
        let map = Map::from_ascii(&source).unwrap();
        let terrain = (1..19)
            .flat_map(|y| (1..39).map(move |x| (GridPos::new(x, y), RegionTerrain::Grass)))
            .collect();
        let family = |id: &str, level| FaunaFamily {
            id: format!("test:family_{id}").parse().unwrap(),
            weight: 1,
            species: vec![FaunaSpecies {
                id: format!("test:species_{id}").parse().unwrap(),
                level,
                habitats: vec![RegionTerrain::Grass],
                population: RegionPopulationRule::new(
                    1,
                    1,
                    2,
                    8,
                    10,
                    AttackProfile::melee(DamageType::Kinetic, 1),
                    AiProfile::idle(),
                    None,
                )
                .unwrap(),
            }],
        };
        (
            map,
            terrain,
            RegionFaunaProfile {
                group_rolls: [1, 2],
                level_range: [1, 2],
                danger_budget: 4,
                families: vec![family("a", 1), family("b", 2)],
            },
        )
    }

    #[test]
    fn fauna_draws_are_repeatable_varied_bounded_and_preserve_safe_and_reserved_cells() {
        let (mut map, terrain, profile) = fixture();
        let reserved = BTreeSet::from([GridPos::new(20, 10)]);
        map.set_protected(GridPos::new(21, 10), true).unwrap();
        let passages = [GridPos::new(1, 1)];
        let mut rosters = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for seed in 0..64 {
            let animals =
                generate_regional_fauna(&map, &terrain, &passages, &reserved, &profile, seed)
                    .unwrap();
            assert_eq!(
                animals,
                generate_regional_fauna(&map, &terrain, &passages, &reserved, &profile, seed)
                    .unwrap()
            );
            assert!(!animals.is_empty());
            let mut cost = 0;
            let mut positions = BTreeSet::new();
            for actor in &animals {
                assert!(positions.insert(actor.position()));
                assert!(
                    !reserved.contains(&actor.position()) && !map.is_protected(actor.position())
                );
                assert!(actor.position().x.abs_diff(1) + actor.position().y.abs_diff(1) >= 8);
                assert!(actor.electronic_system().is_none());
                for family in &profile.families {
                    if actor.tags().contains(&family.id) {
                        seen_families.insert(family.id.clone());
                        cost += family.species[0].level;
                    }
                }
            }
            assert!(cost <= profile.danger_budget);
            rosters.insert(
                animals
                    .iter()
                    .map(|actor| actor.tags().clone())
                    .collect::<Vec<_>>(),
            );
        }
        assert!(rosters.len() > 2);
        assert_eq!(seen_families.len(), 2);
    }

    #[test]
    fn fauna_adding_species_keeps_the_family_weight_and_draw_unchanged() {
        let (map, terrain, mut old) = fixture();
        old.group_rolls = [1, 1];
        let mut expanded = old.clone();
        let mut added = expanded.families[0].species[0].clone();
        added.id = "test:species_a2".parse().unwrap();
        let added_id = added.id.clone();
        expanded.families[0].species.push(added);
        let mut saw_added = false;
        let mut saw_original = false;
        for seed in 0..64 {
            let before =
                generate_regional_fauna(&map, &terrain, &[], &BTreeSet::new(), &old, seed).unwrap();
            let after =
                generate_regional_fauna(&map, &terrain, &[], &BTreeSet::new(), &expanded, seed)
                    .unwrap();
            for family in &old.families {
                assert_eq!(
                    before[0].tags().contains(&family.id),
                    after[0].tags().contains(&family.id)
                );
            }
            saw_added |= after[0].tags().contains(&added_id);
            saw_original |= after[0].tags().contains(&old.families[0].species[0].id);
        }
        assert!(saw_added && saw_original);
    }

    #[test]
    fn heavy_fauna_requires_a_single_clear_contact_bite_with_recovery() {
        let (_, _, mut profile) = fixture();
        let attack = AttackProfile::new(
            1,
            crate::world::DistanceMetric::Chebyshev,
            true,
            DamageType::Piercing,
            6,
            0,
        );
        for (attack, valid) in [
            (attack, false),
            (
                attack.with_recovery_after_attack(crate::time::TimeUnits::new(2).unwrap()),
                true,
            ),
            (
                attack.with_recovery_after_attack(crate::time::TimeUnits::new(9).unwrap()),
                false,
            ),
            (
                attack
                    .with_delivery(crate::combat::AttackDelivery::Ranged)
                    .with_recovery_after_attack(crate::time::TimeUnits::ONE),
                false,
            ),
        ] {
            profile.families[0].species[0].population = RegionPopulationRule::new(
                1,
                1,
                1,
                20,
                12,
                attack,
                AiProfile::new(crate::ai::AiBehavior::TelegraphedBiter, 7, 0, 128, 0),
                None,
            )
            .unwrap();
            assert_eq!(profile.validate().is_ok(), valid);
        }
    }

    #[test]
    fn fauna_filters_level_habitat_and_budget_before_choosing_a_family() {
        let (map, terrain, mut profile) = fixture();
        profile.level_range = [1, 1];
        profile.danger_budget = 1;
        for seed in 0..16 {
            let animals =
                generate_regional_fauna(&map, &terrain, &[], &BTreeSet::new(), &profile, seed)
                    .unwrap();
            assert_eq!(animals.len(), 1);
            assert!(animals[0].tags().contains(&profile.families[0].id));
        }
        profile.families[0].species[0].habitats = vec![RegionTerrain::Mud];
        assert!(
            generate_regional_fauna(&map, &terrain, &[], &BTreeSet::new(), &profile, 1)
                .unwrap()
                .is_empty()
        );
        profile.families[0].weight = 0;
        assert!(
            generate_regional_fauna(&map, &terrain, &[], &BTreeSet::new(), &profile, 1).is_err()
        );
    }
}
