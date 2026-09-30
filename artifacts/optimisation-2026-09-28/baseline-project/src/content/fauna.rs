//! Data-defined families, species and regional level filters. No faction semantics.
use super::{ContentId, RegionPopulationRule, RegionTerrain};
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaunaSpecies {
    pub id: ContentId,
    pub level: u16,
    pub habitats: Vec<RegionTerrain>,
    pub population: RegionPopulationRule,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FaunaFamily {
    pub id: ContentId,
    pub weight: u32,
    pub species: Vec<FaunaSpecies>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegionFaunaProfile {
    pub group_rolls: [u16; 2],
    pub level_range: [u16; 2],
    pub danger_budget: u16,
    pub families: Vec<FaunaFamily>,
}

impl RegionFaunaProfile {
    pub fn validate(&self) -> Result<(), String> {
        if self.group_rolls[0] > self.group_rolls[1]
            || self.group_rolls[1] > 8
            || self.group_rolls[1] == 0
            || self.level_range[0] == 0
            || self.level_range[0] > self.level_range[1]
            || self.level_range[1] > 100
            || self.danger_budget == 0
            || self.danger_budget > 100
            || self.families.is_empty()
            || self.families.len() > 16
        {
            return Err(
                "fauna requires bounded group rolls, positive levels and a danger budget".into(),
            );
        }
        let mut families = BTreeSet::new();
        let mut species = BTreeSet::new();
        for family in &self.families {
            if family.weight == 0
                || !families.insert(&family.id)
                || family.species.is_empty()
                || family.species.len() > 16
            {
                return Err(
                    "fauna families require unique IDs, positive weights and 1..16 species".into(),
                );
            }
            for kind in &family.species {
                if kind.population.ai().behavior == crate::ai::AiBehavior::TelegraphedBiter {
                    let attack = kind.population.attack();
                    if attack.range() != 1
                        || !attack.requires_line_of_sight()
                        || attack.delivery() != crate::combat::AttackDelivery::Melee
                        || attack.area() != crate::combat::AttackArea::Single
                        || !attack
                            .recovery_after_attack()
                            .is_some_and(|duration| duration.get() <= 8)
                    {
                        return Err("telegraphed fauna bites require contact, clear sight, a single cell and bounded recovery".into());
                    }
                }
                if !species.insert(&kind.id)
                    || kind.level == 0
                    || kind.level > 100
                    || kind.habitats.is_empty()
                    || kind.habitats.iter().any(|terrain| !terrain.is_walkable())
                    || kind.population.maximum_count() > 4
                    || kind.population.minimum_passage_distance() < 8
                    || kind.population.electronic_system().is_some()
                {
                    return Err("fauna species require unique IDs, positive levels, walkable habitats, small groups and safe arrivals; no electronic body".into());
                }
            }
        }
        Ok(())
    }

    pub fn maximum_actor_count(&self) -> u16 {
        self.group_rolls[1].saturating_mul(
            self.families
                .iter()
                .flat_map(|f| &f.species)
                .map(|kind| kind.population.maximum_count())
                .max()
                .unwrap_or(0),
        )
    }
}
