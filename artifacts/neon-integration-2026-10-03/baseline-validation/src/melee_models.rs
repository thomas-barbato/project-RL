//! Approved melee models, separate from legacy suspension definitions.
use super::*;

pub(in crate::ascii_app) fn melee_model_ids() -> Vec<ContentId> {
    MELEE_MODEL_IDS
        .iter()
        .map(|id| id.parse().unwrap())
        .collect()
}

pub(in crate::ascii_app) fn sword_model_ids() -> Vec<ContentId> {
    SWORD_MODEL_IDS
        .iter()
        .map(|id| id.parse().unwrap())
        .collect()
}

pub(super) const SWORD_MODEL_IDS: [&str; 6] = [
    "core:melee_epee_courte",
    "core:melee_glaive",
    "core:melee_fauchon",
    "core:melee_epee_longue",
    "core:melee_epee_batarde",
    "core:melee_espadon",
];

pub(super) const MELEE_MODEL_IDS: [&str; 24] = [
    "core:melee_couteau_de_camp",
    "core:melee_couteau_de_chasse",
    "core:melee_poignard",
    "core:melee_dague",
    "core:melee_stylet",
    "core:melee_dague_a_rouelles",
    "core:melee_lance",
    "core:melee_epieu",
    "core:melee_pique",
    "core:melee_vouge",
    "core:melee_pertuisane",
    "core:melee_hallebarde",
    "core:melee_hachette",
    "core:melee_hache_de_guerre",
    "core:melee_hache_d_abordage",
    "core:melee_hache_d_armes",
    "core:melee_hache_danoise",
    "core:melee_bardiche",
    "core:melee_gourdin",
    "core:melee_gourdin_ferre",
    "core:melee_masse_d_armes",
    "core:melee_masse_a_ailettes",
    "core:melee_marteau_de_guerre",
    "core:melee_bec_de_corbin",
];

pub(in crate::ascii_app) const LEGACY_MELEE_IDS: [&str; 9] = [
    "core:couteau_de_camp",
    "core:couteau_de_sapeur",
    "core:couteau_ceramique",
    "core:couteau_de_chitine",
    "core:couteau_a_dent_vivante",
    "core:couteau_du_dernier_seuil",
    "core:lance",
    "core:hache_de_combat",
    "core:marteau_de_guerre",
];

#[cfg(test)]
#[path = "melee_models_tests.rs"]
mod tests;
