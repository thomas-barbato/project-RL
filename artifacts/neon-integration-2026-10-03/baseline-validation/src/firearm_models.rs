//! Approved firearm models. Legacy prototypes remain readable in old saves.
use super::*;

pub(in crate::ascii_app) fn firearm_model_ids() -> Vec<ContentId> {
    FIREARM_MODEL_IDS
        .iter()
        .map(|id| id.parse().unwrap())
        .collect()
}

pub(super) const FIREARM_MODEL_IDS: [&str; 48] = [
    "core:modele_veyr_r12",
    "core:modele_kardan_r24",
    "core:modele_orvek_r36",
    "core:modele_tervan_r48",
    "core:modele_serdak_r60",
    "core:modele_noryk_r72",
    "core:modele_darven_a10",
    "core:modele_korsan_a30",
    "core:modele_vektor_a60",
    "core:modele_brenek_a80",
    "core:modele_talvek_a90",
    "core:modele_vornek_a120",
    "core:modele_brask_p12",
    "core:modele_morn_p24",
    "core:modele_drek_p40",
    "core:modele_korven_p50",
    "core:modele_darsk_p70",
    "core:modele_valdran_p90",
    "core:modele_kordal_m20",
    "core:modele_varkan_m40",
    "core:modele_draven_m80",
    "core:modele_torgal_m100",
    "core:modele_kraven_m120",
    "core:modele_vornak_m160",
    "core:modele_torven_lr2",
    "core:modele_karvek_lr6",
    "core:modele_ordan_lr9",
    "core:modele_bravan_lr12",
    "core:modele_dorek_lr16",
    "core:modele_karsen_lr20",
    "core:modele_rovak_lg3",
    "core:modele_darsen_lg6",
    "core:modele_korven_lg9",
    "core:modele_merdan_lg12",
    "core:modele_torvik_lg16",
    "core:modele_valrek_lg20",
    "core:modele_varek_f10",
    "core:modele_torak_f30",
    "core:modele_karn_f60",
    "core:modele_derven_f80",
    "core:modele_korgan_f100",
    "core:modele_vardek_f120",
    "core:modele_neral_e12",
    "core:modele_seryn_e24",
    "core:modele_oryx_e48",
    "core:modele_telvar_e60",
    "core:modele_nerys_e80",
    "core:modele_odran_e100",
];

pub(in crate::ascii_app) const LEGACY_FIREARM_IDS: [&str; 11] = [
    "core:fusil_de_patrouille",
    "core:fusil_de_guetteur",
    "core:fusil_a_induction",
    "core:fusil_de_parallaxe",
    "core:fusil_a_nerf_tendu",
    "core:fusil_de_l_horizon_fendu",
    "core:fusil_d_assaut",
    "core:fusil_a_pompe",
    "core:mitrailleuse_lourde",
    "core:lance_roquettes",
    "core:lance_grenades",
];

#[cfg(test)]
#[path = "firearm_models_tests.rs"]
mod tests;
