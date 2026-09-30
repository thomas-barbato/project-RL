//! Authored layer budgets, independent of player progression or equipment.
#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncounterLayer {
    pub levels: [u16; 2],
    pub vitality: u16,
    pub innate_damage: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EncounterBalance {
    pub layers: Vec<EncounterLayer>,
}

impl EncounterBalance {
    pub fn validate(&self, maximum_depth: u16) -> bool {
        self.layers.len() == usize::from(maximum_depth) + 1
            && self.layers.len() <= usize::from(super::MAX_REGIONAL_WORLD_DEPTH) + 1
            && self.layers.iter().all(|layer| {
                (1..=100).contains(&layer.levels[0])
                    && layer.levels[0] <= layer.levels[1]
                    && layer.levels[1] <= 100
                    && layer.levels[1] - layer.levels[0] <= 5
                    && (100..=800).contains(&layer.vitality)
                    && (100..=400).contains(&layer.innate_damage)
            })
            && self.layers.windows(2).all(|pair| {
                pair[1].levels[0] > pair[0].levels[1]
                    && pair[1].vitality >= pair[0].vitality
                    && pair[1].innate_damage >= pair[0].innate_damage
            })
    }
}
