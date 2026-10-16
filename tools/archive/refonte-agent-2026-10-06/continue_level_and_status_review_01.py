"""Record user review decisions and prepare the next design proposal only."""
from copy import deepcopy
from fractions import Fraction
from pathlib import Path
import hashlib
import json

ROOT = Path(__file__).resolve().parents[2]
BASE = Path(__file__).resolve().parent
PREFIX = "artifacts/refonte-agent-2026-10-06/"
NUM_ACCEPT = "current-level-skill-power-numeric-acceptance-decisions.json"
STATUS_ACCEPT = "last-status-cast-power-acceptance-decisions.json"
LEVEL = "review-current-level-skill-power-02.json"
CURVES = "skill-rank-curves-provisional-02.json"
CHECKS = "skill-rank-curves-validation-02.json"
PROPOSAL = "review-persistent-power-snapshot-INTERACT-POWER01-01.json"


def load(name):
    return json.loads((BASE / name).read_text(encoding="utf-8"))


def ref(name):
    source = (BASE / name).read_text(encoding="utf-8")
    return {"reference": PREFIX + name,
            "reference_sha256": hashlib.sha256(source.encode("utf-8")).hexdigest()}


def pair(data, key, name):
    source = ref(name)
    data[key] = source["reference"]
    data[key + "_sha256"] = source["reference_sha256"]


def save(name, data):
    path = BASE / name
    assert not path.exists(), name
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n",
                    encoding="utf-8", newline="\n")


def unchanged_accepted(old, new):
    for key, value in old.items():
        if key.startswith("accepted_"):
            assert new.get(key) == value, key
    for key in ["branches", "blocks", "removed_skills", "removed_branches", "purchase_rules", "candidate_count", "learned_rank_cap", "cost_per_rank"]:
        assert new.get(key) == old.get(key), key


def common(data):
    data["date"] = "2026-10-09"
    if data.get("pending_level_power_coefficient"):
        data["previous_pending_level_power_coefficient"] = deepcopy(data["pending_level_power_coefficient"])
    data["pending_level_power_coefficient"] = None
    data["level_power_numeric_acceptance"] = ref(NUM_ACCEPT)
    data["current_level_power_numeric_profile"] = ref(LEVEL)
    data["completed_skill_rank_curves"] = ref(CURVES)
    data["skill_rank_curve_validation"] = ref(CHECKS)
    data["latest_status_power_decision"] = ref(STATUS_ACCEPT)
    data["pending_persistent_power_snapshot"] = ref(PROPOSAL)
    data["runtime_integration"] = "not_integrated"


def override(data):
    values = data.setdefault("active_rule_overrides", {})
    assert values["current_level_nonweapon_power"]["numeric_coefficient_approval"].startswith("tentative_reference")
    values["current_level_nonweapon_power"].update(
        numeric_reference=ref(LEVEL),
        numeric_acceptance=ref(NUM_ACCEPT),
        numeric_coefficient_approval="initial_linear_coefficient10_percent_per_level_after1_explicitly_accepted;provisional_balance",
        pending_cases="existing_persistent_effect_power_update_timing_proposed_under_INTERACT_POWER01;native_review_and_final_balance_pending")
    values["delegated_provisional_skill_rank_curves"].update(ref(CURVES))
    values["last_status_cast_power"] = {
        **ref(STATUS_ACCEPT),
        "current_rule": "on_reapplication_of_same_status_the_latest_cast_sets_new_power_even_if_lower;duration_restarts_under_previous_accepted_rule",
        "preserved": "existing_application_conditions_resistances_immunities_Gel_Burn_replacement_and_cumulative_overlapping_zone_effects",
        "pending_cases": "simultaneous_zone_source_order_and_multi_source_provenance;no_strongest_value_or_additive_status_power_rule_inferred"
    }


def main():
    save(NUM_ACCEPT, {
        "date": "2026-10-09", "review_id": "SKILL-LEVEL01-NUM01-ACCEPT",
        "source_proposal": ref("review-current-level-skill-power-01.json"),
        "presented_coefficient": "+10 % des dégâts de base du premier niveau par niveau gagné, avec une croissance linéaire",
        "presented_level40_Braise": {"rank1_fire_damage": 39, "rank10_fire_damage": 171, "without_bonuses_before_defenses": True},
        "user_example_question": "d'accord, et s'il est niveau 11 à combien seront les dégats ?",
        "presented_level11_Braise": {"rank1_fire_damage": 16, "rank10_fire_damage": 70, "without_attribute_or_equipment_bonuses_noncritical_before_defenses": True},
        "user_acceptance": "d'accord, on continue",
        "accepted_numeric_rule": "1 + (current_caster_level - 1) / 10;linear_growth_from_level1_base;never_acquisition_level",
        "approval_scope": "initial_provisional_common_damage_growth_coefficient_and_shown_Braise_examples;existing_learning_rank_caps_attributes_equipment_and_damage_pipeline_preserved",
        "approval_boundary": "not_final_player_monster_balance_or_native_visual_approval;existing_persistent_effect_update_timing_not_resolved_by_this_acceptance",
        "runtime_integration": "not_integrated"
    })
    save(STATUS_ACCEPT, {
        "date": "2026-10-09", "review_id": "INTERACT-STATUS02",
        "user_message": "alors ici c'est simple, quand un même statu est réappliqué avec une puissance differente, c'est le dernier lancement qui compte.",
        "approved_rule": "when_same_status_is_reapplied_latest_cast_power_replaces_previous_power_even_if_lower",
        "strongest_power_selection": False,
        "status_power_addition_by_reapplication": False,
        "duration_rule": "restart_duration_from_beginning_under_existing_INTERACT_STATUS01",
        "prior_rules": ref("interaction-and-priority-user-decisions.json"),
        "preserved": [
            "existing_successful_status_application_conditions_and_resistance_or_importance_rules",
            "Gel_Burn_successful_mutual_replacement",
            "overlapping_zone_effects_apply_cumulatively;not_replaced_by_this_status_power_decision",
            "ordinary_learned_effect_persistence_and_skill_rank_decrease_cleanup_under_their_accepted_scopes"
        ],
        "interpretation_boundary": "reapplied_means_status_application_occurred;unsuccessful_attempt_does_not_supply_a_new_status_power",
        "remaining": "simultaneous_sources_zone_tick_order_and_source_provenance;existing_effect_power_updates_after_level_or_equipment_change",
        "runtime_integration": "not_integrated"
    })

    level = load("review-current-level-skill-power-01.json")
    level.update(revision=2, status="initial_linear_current_level_damage_coefficient_and_shown_Braise_examples_explicitly_accepted;provisional_balance;not_runtime_integrated",
                 coefficient_approval="accepted_as_initial_provisional_curve;not_finally_balanced")
    level["previous_profile"] = ref("review-current-level-skill-power-01.json")
    level["numeric_acceptance"] = ref(NUM_ACCEPT)
    level["initial_damage_factor"] = level.pop("tentative_damage_factor")
    level["history_invariance_example"]["numbers_status"] = "shown_level40_examples_and_initial_curve_now_accepted;provisional_balance"
    for rank, expected in [(1, 16), (10, 70)]:
        raw = Fraction(8 + 3 * (rank - 1)) * Fraction(11 + 9, 10)
        assert int(raw) == expected
        level["Braise_neutral_profile_examples"].append({
            "skill_id": "FE01", "character_current_level": 11,
            "effective_rank": rank, "Volonte": 5, "fire_damage_integer": expected,
            "noncritical_before_defenses": True,
            "profile": "neutral_attributes_no_equipment_or_other_modifiers;illustrative_not_real_save",
            "shown_example_approval": "accepted_by_daccord_on_continue"
        })
    save(LEVEL, level)

    old_curves = load("skill-rank-curves-provisional-01.json")
    curves = deepcopy(old_curves)
    curves.update(revision=2, status="rank_curves_completed_under_delegation;initial_current_level_coefficient_now_accepted;provisional_balance;not_runtime_integrated")
    curves["previous_curves"] = ref("skill-rank-curves-provisional-01.json")
    curves["current_level_numeric_profile"] = ref(LEVEL)
    curves["current_level_numeric_prototype"] = ref(LEVEL)
    curves["current_level_numeric_acceptance"] = ref(NUM_ACCEPT)
    curves["level_numeric_coefficient_approval"] = "explicitly_accepted_initial_linear_curve;not_final_balance_or_native_approval"
    curves["global_constraints"]["raw_nonweapon_damage_current_level_binding"] = "accepted_current_level_direction_and_initial_coefficient;never_learning_level"
    assert curves["skills"] == old_curves["skills"]
    save(CURVES, curves)
    checks = load("skill-rank-curves-validation-01.json")
    checks["revision"] = 2
    checks["previous_validation"] = ref("skill-rank-curves-validation-01.json")
    checks["curves"] = ref(CURVES)
    checks["level_coefficient_status"] = "initial_linear_coefficient_explicitly_accepted;not_final_balance_or_runtime"
    checks["shown_level11_Braise_checks"] = {"rank1": 16, "rank10": 70, "without_bonuses_noncritical_predefense": True}
    save(CHECKS, checks)

    save(PROPOSAL, {
        "date": "2026-10-09", "review_id": "INTERACT-POWER01", "revision": 1,
        "title": "Puissance des effets persistants après un changement de personnage",
        "status": "proposed_awaiting_user_review;not_runtime_integrated",
        "dependencies": [ref(NUM_ACCEPT), ref(STATUS_ACCEPT), ref("interaction-and-priority-user-decisions.json")],
        "scope": "caster_power_parameters_of_skill_generated_statuses_and_spell_ground_zones_already_applied;familiar_actor_statistics_not_changed",
        "proposed_rule": "calculate_caster_level_effective_rank_and_compatible_offensive_or_buff_power_at_cast_or_application;existing_effect_keeps_that_power_until_reapplication_or_expiry",
        "level_stat_equipment_change": "affects_future_casts;does_not_retroactively_raise_or_lower_power_of_an_existing_effect",
        "target_defenses": "current_target_armor_or_corresponding_resistance_is_still_used_at_each_new_eligible_impact_or_periodic_tick;do_not_snapshot_target_defenses",
        "reapplication": "uses_new_cast_values_and_restarts_duration_under_accepted_latest_cast_power_rule",
        "existing_cleanup": "accepted_effect_loss_or_invoked_element_rank_decrease_cleanup_still_removes_affected_elements_when_required;power_snapshot_does_not_protect_them_from_cleanup",
        "familiar_exception": "existing_drone_level_attributes_gear_and_rank_updates_and_raised_origin_statistics_follow_current_accepted_PETS_rules;no_new_freezing_of_actor_statistics",
        "illustrative_example": {
            "existing_Burn_source_power_per_turn": 12,
            "after_caster_gains_level_or_changes_weapon_or_gear": "existing_source_power_stays12;actual_suffered_damage_still_depends_on_current_target_resistance",
            "new_successful_Burn_application_source_power_per_turn": 18,
            "result": "new_Burn_uses18_and_restarts_duration",
            "example_status": "illustrative_values_only;not_Braise_balance_entries"
        },
        "not_resolved_here": [
            "simultaneous_multi_source_tick_order_and_last_cast_provenance_for_overlapping_zones",
            "mine_variant_or_capacity_passive_rank_loss_and_non_actor_invocation_classification",
            "predeath_marks_creating_future_raised_actors_and_birth_parameter_sampling",
            "unshown_attribute_control_duration_or_healing_mappings"
        ],
        "text_approval": "proposal_only;no_new_RP_or_skill_name",
        "mechanic_approval": "pending_user_review",
        "visual_approval": "no_new_visual_proposed",
        "runtime_integration": "not_integrated"
    })

    old_ready = load("review-implementation-readiness-03.json")
    ready = deepcopy(old_ready)
    common(ready)
    ready.update(revision=4, status="initial_level_coefficient_and_last_status_cast_power_accepted;persistent_power_snapshot_proposed;other_interactions_content_native_and_balance_remaining")
    ready["previous_inventory"] = ref("review-implementation-readiness-03.json")
    ready["latest_user_decisions"] = ref(STATUS_ACCEPT)
    for group in ready["before_affected_complete_system_integration"]:
        if group["id"] == "READY01":
            group["remaining"] = [s for s in group["remaining"] if not s.startswith("review_tentative_current_level_damage_coefficient")]
            group["resolved"] += ";initial_current_level_coefficient10_percent_now_accepted"
        if group["id"] == "READY02":
            group["remaining"] = [s.replace("power_source_merging_and_exact_tick_order_pending", "latest_cast_status_power_now_accepted;simultaneous_sources_and_exact_tick_order_pending") for s in group["remaining"]]
    ready["resolved_interactions"]["INTERACT-STATUS02"] = {
        "rule": "last_cast_power_replaces_previous_same_status_power_even_if_lower;duration_restarts",
        "source": ref(STATUS_ACCEPT),
        "unshown": "simultaneous_zone_source_order_and_provenance;overlapping_zone_effect_cumulation_preserved"
    }
    ready["remaining_after_user_directions"] = [
        "persistent_effect_power_snapshot_proposal_and_partial_non_numeric_receipt_consolidation",
        "simultaneous_status_source_provenance_zone_tick_and_exposure_order",
        "classification_of_invoked_creations_and_passive_cross_skill_capacity_changes",
        "command_action_budget_and_arrival_order",
        "actual_loot_affix_families_ranges_and_weights",
        "class_start_profiles_and_equipment",
        "joint_UI_and_native_visual_design"
    ]
    ready["current_progression_authority"]["level_numeric_coefficient"] = "initial_linear10_percent_coefficient_now_explicitly_accepted;later_balance_pending"
    ready["recommendation"] = "review_remaining_persistent_effect_source_and_turn_interactions_then_finalize_class_starts_and_first_playable_mission;joint_UI_design_and_later_balance_timing_preserved"
    unchanged_accepted(old_ready, ready)
    save("review-implementation-readiness-04.json", ready)

    old_model = load("review-attribute-model-61.json")
    model = deepcopy(old_model)
    common(model)
    override(model)
    model.update(review_id="STAT-MODEL59", status="initial_current_level_coefficient_and_last_status_cast_power_accepted;other_attributes_preserved;persistent_power_update_proposed")
    pair(model, "previous_model", "review-attribute-model-61.json")
    pair(model, "implementation_readiness_audit", "review-implementation-readiness-04.json")
    unchanged_accepted(old_model, model)
    save("review-attribute-model-62.json", model)
    for old_name, new_name in [
        ("review-primary-catalogue-233.json", "review-primary-catalogue-234.json"),
        ("review-complete-skill-design-208.json", "review-complete-skill-design-209.json")
    ]:
        old = load(old_name)
        data = deepcopy(old)
        common(data)
        override(data)
        data["batch_revision"] += 1
        pair(data, "previous_manifest", old_name)
        pair(data, "current_attribute_model", "review-attribute-model-62.json")
        pair(data, "implementation_readiness_audit", "review-implementation-readiness-04.json")
        pair(data, "latest_decisions", STATUS_ACCEPT)
        pair(data, "latest_proposal", PROPOSAL)
        data["latest_proposal_scope"] = "existing_persistent_effect_caster_power_update_rule_only;initial_level_coefficient_and_last_status_cast_power_now_accepted"
        data["next_review_kind"] = "persistent_effect_power_snapshot_INTERACT_POWER01"
        unchanged_accepted(old, data)
        save(new_name, data)
    old_counts = load("remaining-individual-skill-review-counts-102.json")
    counts = deepcopy(old_counts)
    common(counts)
    pair(counts, "previous_count", "remaining-individual-skill-review-counts-102.json")
    pair(counts, "current_attribute_model", "review-attribute-model-62.json")
    pair(counts, "implementation_readiness_audit", "review-implementation-readiness-04.json")
    counts["latest_feedback"] = "user_accepts_initial_current_level_curve_then_defines_latest_cast_same_status_power;existing_persistent_power_update_proposed"
    counts["next_review_kind"] = "persistent_effect_power_snapshot_INTERACT_POWER01"
    counts["skill_progression_review_scope"] = "rank_curves_completed_under_delegation_and_initial_level_coefficient_accepted;non_numeric_cases_and_later_balance_separate;first_pass_count0_unchanged"
    counts["additional_work_not_quantified"] = [
        "unshown_attribute_rules_and_derived_stats",
        "partial_non_numeric_receipt_reconciliation_and_remaining_persistent_effect_interactions",
        "DR01_catalogue_allocation_preserving_accepted_card",
        "native_visual_runtime_attributes_and_balance"
    ]
    unchanged_accepted(old_counts, counts)
    assert counts["remaining_first_pass_total"] == 0
    save("remaining-individual-skill-review-counts-103.json", counts)
    print(json.dumps({"level_curve": "accepted_initial_provisional", "status_power": "latest_cast_accepted_even_if_lower",
                      "next_proposal": "INTERACT-POWER01", "model": 62, "primary": 234, "complete": 209,
                      "counts": 103, "runtime": "not_integrated"}))


if __name__ == "__main__":
    main()
