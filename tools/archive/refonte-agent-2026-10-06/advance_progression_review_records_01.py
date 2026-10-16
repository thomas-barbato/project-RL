"""Advance immutable design manifests after the delegated numeric pass."""
from copy import deepcopy
from pathlib import Path
import hashlib
import json

BASE = Path(__file__).resolve().parent
PREFIX = "artifacts/refonte-agent-2026-10-06/"


def load(name):
    return json.loads((BASE / name).read_text(encoding="utf-8"))


def ref(name):
    text = (BASE / name).read_text(encoding="utf-8")
    return {"reference": PREFIX + name,
            "reference_sha256": hashlib.sha256(text.encode("utf-8")).hexdigest()}


def pair(data, key, name):
    source = ref(name)
    data[key] = source["reference"]
    data[key + "_sha256"] = source["reference_sha256"]


def save(name, data):
    path = BASE / name
    assert not path.exists(), name
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n",
                    encoding="utf-8", newline="\n")


def preserve_accepted(old, new):
    for key, value in old.items():
        if key.startswith("accepted_"):
            assert new.get(key) == value, key


def add_overrides(data):
    overrides = data.setdefault("active_rule_overrides", {})
    assert "delegated_provisional_skill_rank_curves" not in overrides
    overrides["delegated_provisional_skill_rank_curves"] = {
        **ref("skill-rank-curves-provisional-01.json"),
        "authority": ref("skill-rank-balancing-delegation-decisions.json"),
        "current_rule": "use_completed_provisional_rank_parameters_for79_primary14_secondary_and1_accepted_DR01_complement",
        "scope": "numerical_rank_curves_under_user_delegation;not_new_skill_RP_effect_conditions_attribute_mapping_art_or_final_balance_approval",
        "precedence": "replaces_old_sample_numeric_curves_only;current_global_approved_effect_rules_and_modifier_pipeline_take_priority",
        "historical_receipts": "retained;earlier_B01_pending_rank_confirmation_gate_resolved_by_delegation",
        "runtime_integration": "not_integrated"
    }
    overrides["current_level_nonweapon_power"] = {
        **ref("current-level-skill-power-acceptance-decisions.json"),
        "current_rule": "nonweapon_damage_base_follows_current_caster_level_and_current_effective_rank;acquisition_level_has_no_effect",
        "accepted_scope": "current_level_direction_and_learning_history_invariance",
        "numeric_reference": ref("review-current-level-skill-power-01.json"),
        "numeric_coefficient_approval": "tentative_reference_not_individually_accepted;no_runtime_integration",
        "preserved": "weapon_base_drone_actor_base_raised_origin_stats_current_attributes_and_gear;no_duplicate_level_multiplier_on_actual_damage_derivatives",
        "pending_cases": "level_coefficient_review_and_existing_persistent_effect_update_timing"
    }


def common(data):
    data["date"] = "2026-10-09"
    if data.get("pending_skill_progression"):
        data["previous_pending_skill_progression"] = deepcopy(data["pending_skill_progression"])
    data["pending_skill_progression"] = None
    data["completed_skill_rank_curves"] = ref("skill-rank-curves-provisional-01.json")
    data["skill_rank_curve_validation"] = ref("skill-rank-curves-validation-01.json")
    data["skill_rank_numeric_authority"] = ref("skill-rank-balancing-delegation-decisions.json")
    data["current_level_power_acceptance"] = ref("current-level-skill-power-acceptance-decisions.json")
    data["pending_level_power_coefficient"] = ref("review-current-level-skill-power-01.json")
    data["runtime_integration"] = "not_integrated"


def main():
    curves = load("skill-rank-curves-provisional-01.json")
    assert curves["total_skill_count"] == 94
    old_ready = load("review-implementation-readiness-02.json")
    ready = deepcopy(old_ready)
    ready.update(date="2026-10-09", revision=3,
                 status="delegated_rank_curves_completed;current_level_direction_accepted;level_coefficient_interactions_content_and_native_review_remaining")
    ready["previous_inventory"] = ref("review-implementation-readiness-02.json")
    ready["latest_user_decisions"] = ref("current-level-skill-power-acceptance-decisions.json")
    common(ready)
    for group in ready["before_affected_complete_system_integration"]:
        if group["id"] == "READY01":
            group["remaining"] = [
                "review_tentative_current_level_damage_coefficient_separately_from_accepted_general_direction",
                "preserve_accepted_names_RP_and_effect_conditions;unshown_non_numeric_parts_still_need_review",
                "reconcile_eight_older_partial_receipts_against_exact_existing_approvals;rank_numbers_now_delegated_not_every_other_part",
                "allocate_already_accepted_DR01_Repair_card_in_final_catalogue_without_altering_its_identity"
            ]
            group["resolved"] = "94_provisional_rank_curves_completed_under_explicit_user_numeric_delegation;no_individual_rank_confirmation_gate"
        if group["id"] == "READY02":
            group["remaining"].append("existing_zone_or_periodic_effect_update_after_caster_level_change_where_unshown")
    ready["remaining_after_user_directions"] = [
        "tentative_level_damage_coefficient_review_and_partial_non_numeric_receipt_consolidation",
        "status_strength_refresh_and_multi_source_merging_details",
        "zone_tick_exposure_order_and_caster_level_change_update_policy",
        "classification_of_invoked_creations_and_passive_cross_skill_capacity_changes",
        "command_action_budget_and_arrival_order",
        "actual_loot_affix_families_ranges_and_weights",
        "class_start_profiles_and_equipment",
        "joint_UI_and_native_visual_design"
    ]
    ready["current_progression_authority"] = {
        "rank_numbers": "delegated_provisional_pass_completed;old_accepted_work_priorities_snapshot_retained_as_history",
        "level_growth_direction": "accepted_current_level_and_history_invariance",
        "level_numeric_coefficient": "separate_tentative_reference;review_before_game_integration"
    }
    ready["recommendation"] = "review_level_numeric_reference_and_remaining_effect_interactions_then_finalize_class_starts_and_first_playable_mission;parallel_joint_UI_design_and_later_balance_timing_preserved"
    preserve_accepted(old_ready, ready)
    save("review-implementation-readiness-03.json", ready)

    old_model = load("review-attribute-model-60.json")
    model = deepcopy(old_model)
    model.update(review_id="STAT-MODEL58", status="delegated_rank_curves_completed_and_current_level_power_direction_accepted;level_coefficient_proposed;other_attributes_and_effect_rules_preserved")
    pair(model, "previous_model", "review-attribute-model-60.json")
    pair(model, "implementation_readiness_audit", "review-implementation-readiness-03.json")
    common(model)
    add_overrides(model)
    preserve_accepted(old_model, model)
    save("review-attribute-model-61.json", model)

    for old_name, new_name in [
        ("review-primary-catalogue-232.json", "review-primary-catalogue-233.json"),
        ("review-complete-skill-design-207.json", "review-complete-skill-design-208.json")
    ]:
        old = load(old_name)
        data = deepcopy(old)
        common(data)
        add_overrides(data)
        data["batch_revision"] += 1
        pair(data, "previous_manifest", old_name)
        pair(data, "current_attribute_model", "review-attribute-model-61.json")
        pair(data, "implementation_readiness_audit", "review-implementation-readiness-03.json")
        pair(data, "latest_decisions", "current-level-skill-power-acceptance-decisions.json")
        pair(data, "latest_proposal", "review-current-level-skill-power-01.json")
        data["latest_proposal_scope"] = "current_level_numeric_coefficient_reference_only;94_rank_curves_completed_under_delegation"
        data["next_review_skill_id"] = None
        data["next_review_branch"] = None
        data["next_review_kind"] = "current_level_numeric_reference_and_remaining_effect_interactions"
        preserve_accepted(old, data)
        for key in ["branches", "blocks", "removed_skills", "removed_branches", "purchase_rules", "candidate_count"]:
            assert old.get(key) == data.get(key), key
        save(new_name, data)

    old_counts = load("remaining-individual-skill-review-counts-101.json")
    counts = deepcopy(old_counts)
    common(counts)
    pair(counts, "previous_count", "remaining-individual-skill-review-counts-101.json")
    pair(counts, "current_attribute_model", "review-attribute-model-61.json")
    pair(counts, "implementation_readiness_audit", "review-implementation-readiness-03.json")
    counts["latest_feedback"] = "user_delegates_rank_numbers_and_accepts_current_level_power_with_learning_history_invariance;94_rank_curves_completed_provisionally"
    counts["next_review_skill"] = None
    counts["next_review_kind"] = "current_level_numeric_reference_and_remaining_effect_interactions"
    counts["skill_progression_review_scope"] = "rank_curves_completed_under_delegation;level_direction_accepted;exact_level_coefficient_and_non_numeric_cases_separate;first_pass_count0_unchanged"
    counts["additional_work_not_quantified"] = [
        "unshown_attribute_rules_and_derived_stats",
        "partial_non_numeric_receipt_reconciliation_and_current_level_numeric_reference",
        "DR01_catalogue_allocation_preserving_accepted_card",
        "native_visual_runtime_attributes_and_balance"
    ]
    counts["completed_rank_curve_counts"] = {"primary": 79, "secondary": 14, "accepted_repair_complement": 1, "total": 94}
    preserve_accepted(old_counts, counts)
    assert counts["remaining_first_pass_total"] == 0
    save("remaining-individual-skill-review-counts-102.json", counts)
    print("Advanced model61, primary233, complete208, counts102 and readiness03; historical accepted fields preserved.")


if __name__ == "__main__":
    main()
