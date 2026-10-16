"""Record displayed implant approval and prepare the next ammunition review."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json
import runpy

BASE = Path(__file__).resolve().parent
ROOT = BASE.parents[1]
HELPERS = runpy.run_path(str(BASE / "continue_persistent_power_review_01.py"), run_name="helpers")
load, ref, pair, save, preserve = (HELPERS[k] for k in ["load", "ref", "pair", "save", "preserve"])
RECEIPT = "initial-implants-acceptance-decisions-01.json"
ACCEPTED = "review-initial-implants-START-IMPLANT01-02.json"
AMMO = "review-universal-ammunition-AMMO02-01.json"
BATCH = "review-class-starts-START-ALL01-10.json"
PAIRS = [
    ("review-implementation-readiness-41.json", "review-implementation-readiness-42.json"),
    ("review-attribute-model-99.json", "review-attribute-model-100.json"),
    ("review-primary-catalogue-271.json", "review-primary-catalogue-272.json"),
    ("review-complete-skill-design-246.json", "review-complete-skill-design-247.json"),
    ("remaining-individual-skill-review-counts-140.json", "remaining-individual-skill-review-counts-141.json"),
]


def main():
    displayed = {
        "Bastion": "agent_armor_plus1;maximum_energy_relative_minus10_percent_after_other_reserve_bonuses",
        "Technomancien": "all_own_drone_models_maximum_HP_plus15_percent;Interference_Faille_armure_Silence_Rupture_regeneration_Desorientation_plus1_turn_at_cast;Contagion_transmits_remaining_duration_without_extra_turn;own_melee_ranged_heavy_weapon_and_weapon_skill_damage_relative_minus10_percent",
        "Traqueur": "own_standard_ranged_and_heavy_weapon_and_weapon_skill_damage_plus10_percent;flat_evasion_plus2;own_melee_weapon_and_weapon_skill_damage_relative_minus10_percent",
        "weapon_scope": "own_weapon_attacks;spells_secondary_mine_and_grenade_skills_intrinsic_transformation_attacks_and_familiars_use_their_respective_bonuses",
        "Arcaniste": "spell_damage_plus15_percent_including_Decharge_mutagene_and_spell_periodic_damage;already_boosted_damage_derived_effects_do_not_receive_bonus_twice;ordinary_maximum_HP_relative_minus10_percent_after_other_bonuses_then_Colosse_uses_that_maximum",
        "Metamorphe": "form_duration_plus2_turns_at_activation;human_skill_activation_energy_cost_relative_plus10_percent_including_form_entry;ceil_final_cost;12_becomes14;stealth_rank10_step_cost1_retained;transformed_actions_and_upkeep_excluded",
        "common": "relative_minus10_percent_applies_to_boosted_value;150_damage_becomes135;implant_replacement_preserves_learned_skills_and_does_not_restore_HP_or_energy",
    }
    save(RECEIPT, {
        "date": "2026-10-09", "review_id": "START-IMPLANT01-ACCEPT01",
        "user_acceptance": "parfait :)", "question_answered": "Tu valides ces précisions ?",
        "presented_proposal": ref("review-initial-implants-START-IMPLANT01-01.json"),
        "accepted_displayed_rules": displayed,
        "scope": "displayed_eligibility_and_calculation_precisions_only;existing_names_class_RP_numbers_skills_and_visuals_retained",
        "not_implicitly_accepted": "new_implant_item_names_RP_icons_loot_catalogue_or_other_unshown_observable_effects",
        "runtime_integration": "not_integrated",
    })
    proposal = load("review-initial-implants-START-IMPLANT01-01.json")
    accepted = deepcopy(proposal)
    accepted.update(revision=2, status="displayed_implant_precisions_accepted;runtime_pending")
    accepted["previous_proposal"] = ref("review-initial-implants-START-IMPLANT01-01.json")
    accepted["acceptance"] = ref(RECEIPT)
    accepted["accepted_displayed_rules"] = displayed
    accepted["text_approval"] = "displayed_explanatory_copy_accepted;no_new_item_name_RP_or_icon"
    accepted["mechanic_approval"] = "accepted_displayed_rules_authoritative;original_technical_notes_only_implement_that_scope_and_existing_accepted_global_rules"
    save(ACCEPTED, accepted)

    ammo_copy = """**AMMO02 · Réserve et consommation des munitions — proposition**

**Munitions**
*Des charges standardisées pour alimenter vos armes à distance.*

- Toutes vos armes à distance puisent dans la même réserve d’inventaire, armes lourdes comprises.
- Le tir utilise directement cette réserve, sans chargeur ni action de rechargement.
- Une rafale de 3 projectiles à 1 munition chacun coûte 3 munitions, même si les tirs ratent leur cible.
- Une explosion consomme les munitions du projectile lancé, quel que soit le nombre d’ennemis touchés. Les rebonds et effets secondaires d’un projectile ne consomment pas de munition supplémentaire.
- Chaque arme affiche son coût par projectile ; les coûts des armes lourdes seront fixés avec leurs modèles.
- Une compétence qui fait tirer votre arme paie son coût en énergie et les munitions des projectiles tirés. Les sorts, les compétences de mines et de grenades conservent leurs coûts en énergie.

Si la réserve ne suffit pas pour l’attaque immédiate complète, elle est refusée : aucun projectile ne part et aucun tour, aucune énergie ou munition n’est consommé.

**Affichage proposé**

`Munitions : 40`

`Coût en munitions : 3`

`Munitions insuffisantes : 3 requises, 2 disponibles.`

Les départs validés sont conservés : Technomancien et Traqueur, 40 munitions chacun ; leurs armes de départ utilisent 1 munition par projectile.
"""
    save(AMMO, {
        "date": "2026-10-09", "review_id": "AMMO02", "revision": 1,
        "status": "proposal_pending_explicit_review",
        "accepted_direction": ref("universal-ammunition-and-matter-removal-decisions.json"),
        "accepted_start_stock_and_costs": ref("review-starting-equipment-START-GEAR01-02.json"),
        "scope": "agent_shared_reserve_and_immediate_weapon_attack_consumption_only",
        "player_copy_markdown": ammo_copy,
        "player_copy_sha256": hashlib.sha256(ammo_copy.encode("utf-8")).hexdigest(),
        "proposed_rules": {
            "item_name": "Munitions",
            "item_RP": "Des charges standardisées pour alimenter vos armes à distance.",
            "shared_reserve": "one_real_inventory_item_type_shared_by_all_agent_ranged_weapons_including_heavy;no_model_specific_hidden_counter",
            "magazines": "no_magazine_or_reload_action;current_ammunition_capacity_does_not_create_separate_refonte_reserve",
            "cost": "sum_actual_primary_projectiles_of_immediate_attack_times_each_applicable_weapon_model_cost;consume_for_misses;no_victim_multiplier_for_AoE",
            "secondary_effects": "no_extra_charge_for_existing_primary_projectile_ricochets_or_secondary_impacts",
            "weapon_skills": "pay_defined_skill_energy_plus_primary_weapon_projectile_ammunition;no_new_skill_energy_cost_created",
            "atomic_refusal": "preflight_whole_immediate_attack;insufficient_ammo_means_no_projectile_no_turn_no_energy_ammo_or_rng_consumption",
            "heavy_costs": "same_item_type;actual_per_model_costs_still_to_be_presented",
        },
        "illustrative_examples": [
            {"id": "burst", "projectiles": 3, "cost_per_projectile": 1, "required": 3},
            {"id": "AoE", "projectiles": 1, "illustrative_model_cost": 4, "victims": 8, "required": 4, "real_heavy_cost_approved": False},
            {"id": "refusal", "required": 3, "available": 2, "remaining": 2, "shots_fired": 0},
        ],
        "preserved": "START_GEAR01_40_each_for_Technomancien_Traqueur_and1_per_projectile_for_start_pistol_rifle;energy_only_spells_secondary_mine_and_grenade_skills",
        "still_to_define": ["weight_and_stack_presentation", "loot_and_shop_supply_amounts", "all_remaining_weapon_model_costs", "familiars_and_monsters_supply", "ammunition_reservation_for_delayed_or_multi_turn_attacks", "native_icon"],
        "approval_boundary": "only_copy_and_rules_explicitly_presented_in_chat_can_be_accepted;unshown_details_not_approved_by_link",
        "text_approval": "pending", "mechanic_approval": "pending", "visual_approval": "no_new_icon_or_art_proposed",
        "runtime_integration": "not_integrated",
    })

    old_batch = load("review-class-starts-START-ALL01-09.json")
    batch = deepcopy(old_batch)
    batch.update(revision=10, status="displayed_initial_implant_precisions_accepted;ammunition_details_proposed;gameplay_pending")
    batch["previous_batch"] = ref("review-class-starts-START-ALL01-09.json")
    batch["accepted_initial_implant_precisions"] = ref(ACCEPTED)
    batch["latest_implant_acceptance"] = ref(RECEIPT)
    batch["implant_condition_precedence"] = "START_IMPLANT01_rev2_accepted_displayed_rules_override_older_unshown_appendix_only_where_specified;historical_appendix_retained"
    batch["pending_universal_ammunition_review"] = ref(AMMO)
    batch["remaining"] = [x for x in batch["remaining"] if x != "unshown_initial_implant_scope_and_penalty_pipeline"]
    preserve(old_batch, batch)
    assert batch["profiles"] == old_batch["profiles"]
    save(BATCH, batch)

    for old_name, new_name in PAIRS:
        old, data = load(old_name), deepcopy(load(old_name))
        data["date"] = "2026-10-09"
        data["accepted_initial_implant_precisions"] = ref(ACCEPTED)
        data["latest_implant_acceptance"] = ref(RECEIPT)
        data["pending_universal_ammunition_review"] = ref(AMMO)
        data["prepared_five_class_starts"] = ref(BATCH)
        data["next_review_kind"] = "AMMO02_agent_shared_reserve_and_immediate_consumption;then_remaining_actor_supply_heavy_models_loot_and_UI"
        overrides = data.setdefault("active_rule_overrides", {})
        overrides["initial_implant_displayed_precisions"] = {
            **ref(ACCEPTED), "authority": ref(RECEIPT),
            "scope": "only_accepted_displayed_rules;unchanged_class_numbers_and_other_approved_rules_preserved",
            "runtime_integration": "not_integrated",
        }
        if old_name.startswith("review-implementation"):
            data.update(revision=42, status="displayed_initial_implant_rules_accepted;ammo_reserve_review_prepared;runtime_systems_pending")
            data["previous_inventory"] = ref(old_name)
            data["latest_user_decisions"] = ref(RECEIPT)
            data["recommendation"] = "review_AMMO02_without_reasking_accepted_start_stocks_then_actor_supply_and_heavy_models;approved_foundations_can_be_implemented_incrementally"
            for group in data["before_affected_complete_system_integration"]:
                if group["id"] == "READY04":
                    group["remaining"] = [x for x in group["remaining"] if x != "unshown_initial_implant_scope_and_penalty_pipeline"]
                    group["prepared_five_profiles"] = ref(BATCH)
                    group["accepted_implant_precisions"] = ref(ACCEPTED)
        else:
            pair(data, "implementation_readiness_audit", PAIRS[0][1])
            if old_name.startswith("review-attribute"):
                data.update(review_id="STAT-MODEL97", status="displayed_initial_implant_precisions_accepted;other_stats_preserved;ammunition_proposed")
                pair(data, "previous_model", old_name)
            elif old_name.startswith(("review-primary", "review-complete")):
                data["batch_revision"] += 1
                pair(data, "previous_manifest", old_name)
                pair(data, "latest_decisions", RECEIPT)
            else:
                pair(data, "previous_count", old_name)
                data["latest_feedback"] = "initial_implant_precisions_accepted;skill_review_counts_unchanged;AMMO02_next"
            if not old_name.startswith("review-attribute"):
                pair(data, "current_attribute_model", PAIRS[1][1])
        preserve(old, data)
        for key, value in old.get("active_rule_overrides", {}).items():
            assert data["active_rule_overrides"][key] == value, key
        save(new_name, data)

    hashes = runpy.run_path(str(BASE / "continue_pistol_arm_axis_review_01.py"), run_name="hash_helpers")["hashes"]
    names = [RECEIPT, ACCEPTED, AMMO, BATCH] + [new for _, new in PAIRS]
    checked = sum(hashes(load(name)) for name in names)
    print(json.dumps({"files": len(names), "references_checked": checked, "class_numbers_and_skill_rules_preserved": True, "runtime_modified": False}))


if __name__ == "__main__":
    main()
