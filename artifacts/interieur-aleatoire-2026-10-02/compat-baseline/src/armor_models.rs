//! Reinforced body protection with a fixed, explicitly displayed Evasion tradeoff.
use super::*;

pub(in crate::ascii_app) fn armor_model_ids() -> Vec<ContentId> {
    ARMOR_MODEL_IDS
        .iter()
        .map(|id| id.parse().unwrap())
        .collect()
}

pub(super) const ARMOR_MODEL_IDS: [&str; 6] = [
    "core:brigandine_cognefer",
    "core:plastron_bastion",
    "core:armure_cerbere",
    "core:carapace_chrysalide",
    "core:manteau_de_mue",
    "core:combinaison_fantome",
];

#[cfg(test)]
#[path = "armor_models_tests.rs"]
mod tests;
